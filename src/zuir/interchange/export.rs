
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! ZUIR interchange export boundary.
//!
//! PATH
//!   src/zuir/interchange/export.rs
//!
//! PURPOSE
//!   Export an already-constructed representation through an explicitly
//!   registered interchange adapter. The adapter owns format-specific
//!   encoding; this module owns format selection, registry integrity,
//!   output budgeting, and error propagation.
//!
//! AUTHORITY
//!   - src/zuir/ZUIR-SPEC.md
//!   - src/zuir/interchange/format.rs
//!   - src/zuir/serialization/ (for ZUIR-native serialization adapters)
//!   - grammar/specification/semantic-model.md
//!   - grammar/specification/compilation-model.md
//!
//! DOES NOT OWN
//!   - Zamani parsing, lexing, AST construction, or semantic analysis.
//!   - The canonical ZUIR program or operation data model.
//!   - Encoding/decoding algorithms for individual formats.
//!   - Schema migration or cross-version compatibility inference.
//!   - Target discovery, physical allocation, routing, or execution.
//!
//! SAFETY
//!   Safe Rust only. No unsafe code.
//!
//! SCALABILITY
//!   The registry is dynamically sized and has no fixed format-count limit.
//!   Output limits are optional, caller-configured operational budgets;
//!   they do not define the semantic size of a valid program.
//!   Actual process memory, address space, writer behavior, and available
//!   resources remain finite.
//!
//! DETERMINISM
//!   Registry keys are ordered. Format selection is exact and explicit.
//!   This module does not select a format based on insertion order, file
//!   extension, or guessed content.
//!
//! OUTPUT CONTRACT
//!   Export writes to a caller-provided `Write` implementation. A failure
//!   can leave a partial prefix in that writer; arbitrary writers cannot
//!   generally be rolled back. Callers requiring atomic publication should
//!   encode to a temporary buffer/file and publish it only after success.
//!   This module does not flush or close caller-owned writers.
//!
//! INTEGRATION
//!   `format.rs` owns `FormatDescriptor`, `FormatRequirement`,
//!   `InterchangeFormat`, and format capability/version contracts.
//!   Concrete adapters implement `ExportAdapter<T>`.
//!   Native ZUIR serialization should be exposed as one adapter, not
//!   duplicated inside this module.
//!   `interchange/mod.rs` must declare `pub mod export;`.
//!
//! FREEZE CRITERIA
//!   - Exact format and version are required.
//!   - Required capabilities are checked before encoding.
//!   - Duplicate adapter registrations are rejected.
//!   - Output budgets cannot be silently bypassed by an adapter swallowing
//!     a writer error.
//!   - Adapter and writer errors propagate to the caller.
//!   - Tests cover successful export, unsupported format, duplicates,
//!     output limits, and writer failures.

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;
use std::io::{self, Write};
use std::sync::Arc;

use super::format::{
    FormatDescriptor, FormatError, FormatId, FormatRequirement, FormatVersion,
    InterchangeFormat,
};

/// Policy applied to one export operation.
///
/// `max_output_bytes == None` means this module imposes no byte budget.
/// It does not mean the operating system, destination, or process has
/// unlimited resources.
///
/// Budgets are operational safeguards, not IR semantic constraints.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ExportContext {
    max_output_bytes: Option<u64>,
}

impl ExportContext {
    /// Construct a context without an export byte budget.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            max_output_bytes: None,
        }
    }

    /// Construct a context with an explicit maximum number of output bytes.
    ///
    /// A zero-byte budget is valid and permits only empty output.
    #[must_use]
    pub const fn with_max_output_bytes(max_output_bytes: u64) -> Self {
        Self {
            max_output_bytes: Some(max_output_bytes),
        }
    }

    /// Return the configured byte budget, if any.
    #[must_use]
    pub const fn max_output_bytes(&self) -> Option<u64> {
        self.max_output_bytes
    }
}

/// Errors raised by the export boundary or an export adapter.
#[derive(Debug)]
#[non_exhaustive]
pub enum ExportError {
    /// A format descriptor or requested format was invalid or unsupported.
    Format(FormatError),

    /// Another adapter already owns this exact format ID and version.
    DuplicateAdapter {
        id: FormatId,
        version: FormatVersion,
    },

    /// The adapter failed for a format-specific reason.
    Adapter {
        id: FormatId,
        message: String,
    },

    /// The destination writer failed.
    Io(io::Error),

    /// An adapter attempted to exceed the configured output budget.
    OutputLimitExceeded {
        limit: u64,
    },

    /// The byte counter could not represent the output size.
    OutputSizeOverflow,
}

impl ExportError {
    /// Construct a format-specific adapter error.
    #[must_use]
    pub fn adapter(
        id: FormatId,
        message: impl Into<String>,
    ) -> Self {
        Self::Adapter {
            id,
            message: message.into(),
        }
    }
}

impl fmt::Display for ExportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Format(error) => write!(f, "export format error: {error}"),
            Self::DuplicateAdapter { id, version } => {
                write!(
                    f,
                    "an export adapter is already registered for {id} version {version}"
                )
            }
            Self::Adapter { id, message } => {
                write!(f, "export adapter {id} failed: {message}")
            }
            Self::Io(error) => write!(f, "export destination failed: {error}"),
            Self::OutputLimitExceeded { limit } => {
                write!(f, "export exceeded the configured output limit of {limit} bytes")
            }
            Self::OutputSizeOverflow => {
                f.write_str("export output byte count overflowed")
            }
        }
    }
}

impl Error for ExportError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Format(error) => Some(error),
            Self::Io(error) => Some(error),
            Self::DuplicateAdapter { .. }
            | Self::Adapter { .. }
            | Self::OutputLimitExceeded { .. }
            | Self::OutputSizeOverflow => None,
        }
    }
}

impl From<FormatError> for ExportError {
    fn from(error: FormatError) -> Self {
        Self::Format(error)
    }
}

impl From<io::Error> for ExportError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

/// Contract implemented by a format-specific exporter.
///
/// `T` is the representation being exported. It is intentionally generic:
/// the interchange layer does not require a particular concrete Program
/// layout, and can be reused for canonical ZUIR programs or another
/// explicitly supported representation.
///
/// Implementations must:
/// - report an accurate, stable descriptor;
/// - preserve all semantics promised by their declared capabilities;
/// - return an error rather than silently discard unsupported semantics;
/// - honor the supplied context;
/// - avoid executing the representation being exported.
///
/// The registry additionally wraps the writer to enforce its byte budget.
pub trait ExportAdapter<T>: InterchangeFormat {
    /// Encode `value` into `output` using this adapter's declared format.
    ///
    /// This method must not close or flush the caller-owned writer.
    fn export(
        &self,
        value: &T,
        output: &mut dyn Write,
        context: &ExportContext,
    ) -> Result<(), ExportError>;
}

type AdapterKey = (FormatId, FormatVersion);

/// Registry of exporters for one representation type.
///
/// Registration is explicit. The registry never guesses a format and never
/// silently replaces an adapter. `BTreeMap` provides deterministic key order;
/// actual export still requires an explicit `FormatRequirement`.
pub struct ExportRegistry<T> {
    adapters: BTreeMap<AdapterKey, Arc<dyn ExportAdapter<T>>>,
}

impl<T> fmt::Debug for ExportRegistry<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ExportRegistry")
            .field("registered_adapter_count", &self.adapters.len())
            .finish()
    }
}

impl<T> Default for ExportRegistry<T> {
    fn default() -> Self {
        Self {
            adapters: BTreeMap::new(),
        }
    }
}

impl<T> ExportRegistry<T> {
    /// Create an empty exporter registry.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Number of registered format/version adapters.
    #[must_use]
    pub fn len(&self) -> usize {
        self.adapters.len()
    }

    /// Whether no adapters are registered.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.adapters.is_empty()
    }

    /// Register an adapter.
    ///
    /// The descriptor is validated before registration. Registering another
    /// adapter for the same exact ID and version is an error; callers must
    /// resolve the conflict explicitly rather than relying on insertion order.
    pub fn register<A>(&mut self, adapter: A) -> Result<(), ExportError>
    where
        A: ExportAdapter<T> + 'static,
    {
        let descriptor = adapter.descriptor();
        descriptor.validate()?;

        let key = (
            descriptor.id().clone(),
            descriptor.version(),
        );

        if self.adapters.contains_key(&key) {
            return Err(ExportError::DuplicateAdapter {
                id: key.0,
                version: key.1,
            });
        }

        self.adapters.insert(key, Arc::new(adapter));
        Ok(())
    }

    /// Export using an exact format/version requirement.
    ///
    /// Returns the number of bytes successfully accepted by the destination
    /// writer. This is not a durability guarantee; the caller owns flushing,
    /// syncing, closing, and publishing the destination.
    ///
    /// No fallback to another format or version is attempted.
    pub fn export_to(
        &self,
        value: &T,
        requirement: &FormatRequirement,
        output: &mut dyn Write,
        context: &ExportContext,
    ) -> Result<u64, ExportError> {
        let key = (
            requirement.id().clone(),
            requirement.version(),
        );

        let adapter = self.adapters.get(&key).ok_or_else(|| {
            ExportError::Format(FormatError::UnsupportedFormat {
                id: key.0.clone(),
                version: key.1,
            })
        })?;

        // Revalidate at the trust boundary. Registration validates once,
        // but this check also protects against future registry refactors.
        let descriptor = adapter.descriptor();
        descriptor.validate()?;
        requirement.matches_exact(descriptor)?;

        let mut budgeted = BudgetedWriter::new(
            output,
            context.max_output_bytes(),
        );

        let adapter_result = adapter.export(value, &mut budgeted, context);

        // Check the budget before propagating the adapter error. An adapter
        // must not turn a swallowed writer-limit error into apparent success.
        if budgeted.exceeded {
            return Err(ExportError::OutputLimitExceeded {
                limit: context
                    .max_output_bytes()
                    .expect("exceeded is set only when a limit exists"),
            });
        }

        adapter_result?;
        Ok(budgeted.bytes_written)
    }

    /// Return descriptors in deterministic ID/version order.
    ///
    /// This is useful for tooling and diagnostics. The returned descriptors
    /// are snapshots; export requests still require an explicit requirement.
    pub fn descriptors(&self) -> Vec<FormatDescriptor> {
        self.adapters
            .values()
            .map(|adapter| adapter.descriptor().clone())
            .collect()
    }
}

/// A writer wrapper that counts accepted bytes and enforces an optional limit.
///
/// A write request that would exceed the limit is rejected as a whole.
/// Previously written bytes cannot be rolled back on a general `Write`.
struct BudgetedWriter<'a> {
    inner: &'a mut dyn Write,
    limit: Option<u64>,
    bytes_written: u64,
    exceeded: bool,
}

impl<'a> BudgetedWriter<'a> {
    fn new(inner: &'a mut dyn Write, limit: Option<u64>) -> Self {
        Self {
            inner,
            limit,
            bytes_written: 0,
            exceeded: false,
        }
    }
}

impl Write for BudgetedWriter<'_> {
    fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        if buffer.is_empty() {
            return Ok(0);
        }

        let requested = u64::try_from(buffer.len()).map_err(|_| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "output write length cannot be represented",
            )
        })?;

        let next_total = self.bytes_written.checked_add(requested);

        if next_total.is_none()
            || self
                .limit
                .is_some_and(|limit| next_total.is_none_or(|total| total > limit))
        {
            self.exceeded = self.limit.is_some();
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "export output budget exceeded",
            ));
        }

        let written = self.inner.write(buffer)?;
        let written_u64 = u64::try_from(written).map_err(|_| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "writer result cannot be represented",
            )
        })?;

        self.bytes_written = self
            .bytes_written
            .checked_add(written_u64)
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "export output byte count overflowed",
                )
            })?;

        Ok(written)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zuir::interchange::format::{
        CapabilityId, FormatCapabilities, FormatDescriptor, FormatId,
    };

    const FORMAT_VERSION: FormatVersion = FormatVersion::new(1, 0, 0);

    struct TextAdapter {
        descriptor: FormatDescriptor,
        swallow_write_errors: bool,
    }

    impl TextAdapter {
        fn new() -> Self {
            let id = FormatId::new("example.text").expect("valid format ID");
            let capability =
                CapabilityId::new("semantics.text").expect("valid capability");

            let mut capabilities = FormatCapabilities::new();
            capabilities.insert(capability);

            let descriptor = FormatDescriptor::new(id, FORMAT_VERSION)
                .with_capability(
                    CapabilityId::new("semantics.text").expect("valid capability"),
                );

            let _ = capabilities;

            Self {
                descriptor,
                swallow_write_errors: false,
            }
        }

        fn swallowing_errors() -> Self {
            let mut adapter = Self::new();
            adapter.swallow_write_errors = true;
            adapter
        }
    }

    impl InterchangeFormat for TextAdapter {
        fn descriptor(&self) -> &FormatDescriptor {
            &self.descriptor
        }
    }

    impl ExportAdapter<String> for TextAdapter {
        fn export(
            &self,
            value: &String,
            output: &mut dyn Write,
            _context: &ExportContext,
        ) -> Result<(), ExportError> {
            if self.swallow_write_errors {
                let _ = output.write_all(value.as_bytes());
                return Ok(());
            }

            output.write_all(value.as_bytes())?;
            Ok(())
        }
    }

    fn requirement() -> FormatRequirement {
        FormatRequirement::new(
            FormatId::new("example.text").expect("valid format ID"),
            FORMAT_VERSION,
        )
        .requiring(
            CapabilityId::new("semantics.text").expect("valid capability"),
        )
    }

    #[test]
    fn exports_with_an_exact_registered_format() {
        let mut registry = ExportRegistry::<String>::new();
        registry.register(TextAdapter::new()).expect("register adapter");

        let mut output = Vec::new();
        let written = registry
            .export_to(
                &"hello".to_owned(),
                &requirement(),
                &mut output,
                &ExportContext::new(),
            )
            .expect("export succeeds");

        assert_eq!(output, b"hello");
        assert_eq!(written, 5);
    }

    #[test]
    fn rejects_duplicate_format_and_version() {
        let mut registry = ExportRegistry::<String>::new();
        registry.register(TextAdapter::new()).expect("first registration");

        assert!(matches!(
            registry.register(TextAdapter::new()),
            Err(ExportError::DuplicateAdapter { .. })
        ));
        assert_eq!(registry.len(), 1);
    }

    #[test]
    fn rejects_unknown_versions_without_fallback() {
        let mut registry = ExportRegistry::<String>::new();
        registry.register(TextAdapter::new()).expect("register adapter");

        let other_version = FormatRequirement::new(
            FormatId::new("example.text").expect("valid format ID"),
            FormatVersion::new(2, 0, 0),
        );

        let mut output = Vec::new();
        assert!(matches!(
            registry.export_to(
                &"hello".to_owned(),
                &other_version,
                &mut output,
                &ExportContext::new(),
            ),
            Err(ExportError::Format(FormatError::UnsupportedFormat { .. }))
        ));
        assert!(output.is_empty());
    }

    #[test]
    fn enforces_configured_output_budget() {
        let mut registry = ExportRegistry::<String>::new();
        registry.register(TextAdapter::new()).expect("register adapter");

        let mut output = Vec::new();
        let result = registry.export_to(
            &"hello".to_owned(),
            &requirement(),
            &mut output,
            &ExportContext::with_max_output_bytes(3),
        );

        assert!(matches!(
            result,
            Err(ExportError::OutputLimitExceeded { limit: 3 })
        ));
        assert!(output.len() <= 3);
    }

    #[test]
    fn detects_an_adapter_that_swallows_the_limit_error() {
        let mut registry = ExportRegistry::<String>::new();
        registry
            .register(TextAdapter::swallowing_errors())
            .expect("register adapter");

        let mut output = Vec::new();
        let result = registry.export_to(
            &"hello".to_owned(),
            &requirement(),
            &mut output,
            &ExportContext::with_max_output_bytes(3),
        );

        assert!(matches!(
            result,
            Err(ExportError::OutputLimitExceeded { limit: 3 })
        ));
    }

    #[test]
    fn reports_registered_descriptors_deterministically() {
        let mut registry = ExportRegistry::<String>::new();
        registry.register(TextAdapter::new()).expect("register adapter");

        let descriptors = registry.descriptors();
        assert_eq!(descriptors.len(), 1);
        assert_eq!(descriptors[0].id().as_str(), "example.text");
    }
}
