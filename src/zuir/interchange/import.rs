#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Interchange Import
//!
//! Path: src/zuir/interchange/import.rs
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only; unsafe code is forbidden.
//!
//! ## PURPOSE
//!
//! Provide the canonical import-adapter boundary for ZUIR interchange.
//!
//! This module selects a registered adapter using an exact format identity,
//! version, and required-capability contract. It then delegates decoding and
//! semantic conversion to that adapter while enforcing the caller's input
//! budget and preserving structured error information.
//!
//! The output type is generic. Once the canonical ZUIR program type is
//! finalized, callers can instantiate ImportRegistry with that type without
//! changing this module.
//!
//! ## AUTHORITY
//!
//! - src/zuir/ZUIR-SPEC.md
//! - src/zuir/interchange/format.rs
//! - src/zuir/version.rs
//! - grammar/specification/semantic-model.md
//! - grammar/specification/compilation-model.md
//! - grammar/specification/compatibility.md
//! - grammar/specification/scalability-model.md
//!
//! ## OWNS
//!
//! - The ImportAdapter contract.
//! - ImportContext and import-specific operational policy.
//! - The generic ImportRegistry.
//! - Budget-aware streaming input.
//! - Import-specific errors and error propagation.
//!
//! ## DOES_NOT_OWN
//!
//! - Source grammar, lexer, parser, or AST definitions.
//! - Canonical ZUIR node definitions or semantic analysis.
//! - Format identity, version, capability, or negotiation definitions.
//! - Serialization encoding or decoding algorithms.
//! - Version migration implementations.
//! - Target discovery, hardware allocation, routing, scheduling, or execution.
//! - Source-map storage or diagnostic rendering.
//!
//! ## DEPENDENCIES
//!
//! - crate::zuir::interchange::format
//! - Rust standard library
//!
//! No unsafe code, global mutable state, network access, filesystem access,
//! or runtime-specific assumptions are required.
//!
//! ## SCALABILITY
//!
//! Adapter storage grows dynamically. Input is exposed as a stream rather
//! than being unconditionally buffered into one allocation. Optional byte
//! budgets are operational safeguards, not universal format-size limits.
//! Actual address-space, memory, processing, and operating-system limits
//! remain applicable.
//!
//! ## DETERMINISM
//!
//! Adapters are selected by exact format ID and version. Registration order
//! never determines which adapter is selected. Duplicate registrations are
//! rejected.
//!
//! ## SEMANTIC PRESERVATION
//!
//! An adapter must preserve the semantics it claims to import. Unknown
//! operations or semantic features must not be silently discarded or
//! assigned invented meaning. Verification of the resulting representation
//! remains the responsibility of the canonical verifier.
//!
//! ## FREEZE CRITERIA
//!
//! - The format module exports the types used below.
//! - Duplicate registrations and unsupported formats fail explicitly.
//! - Required capabilities are checked before conversion.
//! - Input-budget exhaustion is reported explicitly.
//! - Adapter failures remain distinguishable from negotiation failures.
//! - Unit and integration tests pass against the actual repository.
//!
//! This file does not claim production readiness until its integration and
//! conformance gates have passed.

use core::fmt;
use std::collections::BTreeMap;
use std::error::Error;
use std::io::{self, Read};
use std::sync::Arc;

use crate::zuir::interchange::format::{
    negotiate_exact, FormatDescriptor, FormatError, FormatId, FormatRequirement,
    FormatVersion, InterchangeFormat,
};

/// Result type for interchange imports.
pub type ImportResult<T> = Result<T, ImportError>;

// =============================================================================
// Import context
// =============================================================================

/// Explicit policy and provenance context for one import operation.
///
/// `max_input_bytes = None` means that this context imposes no byte limit.
/// It does not imply unlimited physical memory or processing resources.
///
/// `source_uri` is opaque provenance metadata. This module does not open,
/// resolve, normalize, or interpret it as a filesystem or network path.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ImportContext {
    max_input_bytes: Option<u64>,
    source_uri: Option<String>,
}

impl ImportContext {
    /// Create an import context without an explicit byte budget.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the maximum number of input bytes the adapter may consume.
    ///
    /// A value of zero permits an empty input but rejects non-empty input
    /// when the adapter attempts to read it.
    #[must_use]
    pub fn with_max_input_bytes(mut self, limit: u64) -> Self {
        self.max_input_bytes = Some(limit);
        self
    }

    /// Remove the explicit input-byte budget.
    #[must_use]
    pub fn without_input_byte_limit(mut self) -> Self {
        self.max_input_bytes = None;
        self
    }

    /// Attach an opaque source URI for diagnostics and provenance.
    ///
    /// An empty or whitespace-only URI is rejected.
    pub fn with_source_uri(
        mut self,
        uri: impl Into<String>,
    ) -> ImportResult<Self> {
        let uri = uri.into();

        if uri.trim().is_empty() {
            return Err(ImportError::InvalidContext(
                "source URI must not be empty".to_owned(),
            ));
        }

        self.source_uri = Some(uri);
        Ok(self)
    }

    /// Return the configured input-byte budget.
    #[must_use]
    pub const fn max_input_bytes(&self) -> Option<u64> {
        self.max_input_bytes
    }

    /// Return the optional opaque source URI.
    #[must_use]
    pub fn source_uri(&self) -> Option<&str> {
        self.source_uri.as_deref()
    }
}

// =============================================================================
// Adapter contract
// =============================================================================

/// Adapter for importing one declared interchange format into `T`.
///
/// Implementations must:
///
/// - Declare an accurate format descriptor.
/// - Consume the supplied reader rather than opening unrelated input sources.
/// - Respect the import context.
/// - Reject unsupported versions and malformed input.
/// - Preserve supported semantic information.
/// - Avoid silently dropping unknown semantic constructs.
/// - Return an error if the requested conversion cannot preserve meaning.
///
/// `T` is chosen by the integrating subsystem. The trait does not prescribe
/// a second IR, an AST, or a particular serialized representation.
pub trait ImportAdapter<T>: InterchangeFormat + Send + Sync {
    /// Import the input stream into the caller-selected representation.
    ///
    /// The reader is budget-aware. If its configured limit is exceeded,
    /// the registry reports `InputLimitExceeded`, even if the adapter
    /// catches and suppresses the resulting I/O error.
    fn import(
        &self,
        input: &mut dyn Read,
        context: &ImportContext,
    ) -> ImportResult<T>;
}

// =============================================================================
// Registry
// =============================================================================

/// Registry of import adapters producing one consistent output type.
///
/// The key is the exact `(format ID, format version)` pair. Different
/// versions can use different adapters. A duplicate pair is rejected,
/// eliminating registration-order-dependent selection.
///
/// The registry does not infer compatibility between versions. Migration
/// belongs to interchange/migration.rs and must be explicitly requested.
pub struct ImportRegistry<T> {
    adapters: BTreeMap<
        (FormatId, FormatVersion),
        Arc<dyn ImportAdapter<T>>,
    >,
}

impl<T> Default for ImportRegistry<T> {
    fn default() -> Self {
        Self {
            adapters: BTreeMap::new(),
        }
    }
}

impl<T> fmt::Debug for ImportRegistry<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ImportRegistry")
            .field("registered_adapter_count", &self.adapters.len())
            .finish()
    }
}

impl<T> ImportRegistry<T> {
    /// Create an empty registry.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Return the number of registered adapters.
    #[must_use]
    pub fn len(&self) -> usize {
        self.adapters.len()
    }

    /// Return whether no adapters are registered.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.adapters.is_empty()
    }

    /// Register an adapter after validating its descriptor.
    ///
    /// Ownership is shared through `Arc`, permitting safe concurrent reads
    /// when adapters themselves satisfy their declared thread-safety
    /// contracts. This registry performs no internal mutation during import.
    pub fn register<A>(&mut self, adapter: A) -> ImportResult<()>
    where
        A: ImportAdapter<T> + 'static,
    {
        self.register_shared(Arc::new(adapter))
    }

    /// Register a shared adapter.
    ///
    /// This is useful when an application owns adapter instances or shares
    /// one adapter with its export subsystem.
    pub fn register_shared(
        &mut self,
        adapter: Arc<dyn ImportAdapter<T>>,
    ) -> ImportResult<()> {
        let descriptor = adapter.descriptor().clone();

        descriptor
            .validate()
            .map_err(ImportError::Format)?;

        let key = (
            descriptor.id().clone(),
            descriptor.version(),
        );

        if self.adapters.contains_key(&key) {
            return Err(ImportError::DuplicateAdapter {
                format_id: key.0,
                version: key.1,
            });
        }

        self.adapters.insert(key, adapter);
        Ok(())
    }

    /// Return descriptors in deterministic format-ID/version order.
    ///
    /// The returned descriptors are owned snapshots. Modifying a snapshot
    /// cannot alter the registry or change a registered adapter's claims.
    #[must_use]
    pub fn descriptors(&self) -> Vec<FormatDescriptor> {
        self.adapters
            .values()
            .map(|adapter| adapter.descriptor().clone())
            .collect()
    }

    /// Find an adapter for an exact format requirement and import its input.
    ///
    /// Capability requirements are checked before the adapter is invoked.
    /// The function does not try other versions, guess formats from file
    /// extensions, or silently perform migrations.
    pub fn import_exact(
        &self,
        requirement: &FormatRequirement,
        input: &mut dyn Read,
        context: &ImportContext,
    ) -> ImportResult<T> {
        let descriptors = self.descriptors();

        let selected = negotiate_exact(requirement, &descriptors)
            .map_err(ImportError::Format)?;

        let key = (
            selected.id().clone(),
            selected.version(),
        );

        let adapter = self
            .adapters
            .get(&key)
            .ok_or_else(|| ImportError::InternalInvariant {
                message: format!(
                    "negotiated adapter {} version {} is not registered",
                    key.0, key.1
                ),
            })?;

        // Defend against a descriptor changing after registration. Adapters
        // must expose stable descriptors for their entire registered life.
        let current = adapter.descriptor();

        if current.id() != &key.0 || current.version() != key.1 {
            return Err(ImportError::InternalInvariant {
                message: "registered adapter descriptor changed after registration"
                    .to_owned(),
            });
        }

        let mut budgeted = BudgetedReader::new(
            input,
            context.max_input_bytes(),
        );

        let result = adapter.import(&mut budgeted, context);

        // Prefer the explicit budget error if the adapter attempted to read
        // beyond the limit, even if it converted or suppressed the I/O error.
        if budgeted.limit_exceeded() {
            return Err(ImportError::InputLimitExceeded {
                limit: context.max_input_bytes().unwrap_or(0),
                source_uri: context.source_uri().map(str::to_owned),
            });
        }

        result
    }
}

// =============================================================================
// Budget-aware reader
// =============================================================================

/// Reader wrapper that enforces an optional input-consumption budget.
///
/// The wrapper does not buffer the entire source. It counts bytes actually
/// consumed by the adapter. When the budget is exhausted, it probes for one
/// additional byte to distinguish an exactly-sized input from an oversized
/// input. The probe byte is not delivered to the adapter.
struct BudgetedReader<'a> {
    inner: &'a mut dyn Read,
    limit: Option<u64>,
    consumed: u64,
    exceeded: bool,
}

impl<'a> BudgetedReader<'a> {
    fn new(inner: &'a mut dyn Read, limit: Option<u64>) -> Self {
        Self {
            inner,
            limit,
            consumed: 0,
            exceeded: false,
        }
    }

    fn limit_exceeded(&self) -> bool {
        self.exceeded
    }
}

impl Read for BudgetedReader<'_> {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        if buffer.is_empty() {
            return Ok(0);
        }

        let Some(limit) = self.limit else {
            let count = self.inner.read(buffer)?;
            self.consumed = self
                .consumed
                .checked_add(count as u64)
                .ok_or_else(|| {
                    io::Error::other("import byte counter overflow")
                })?;
            return Ok(count);
        };

        if self.consumed >= limit {
            let mut probe = [0_u8; 1];

            return match self.inner.read(&mut probe) {
                Ok(0) => Ok(0),
                Ok(_) => {
                    self.exceeded = true;

                    Err(io::Error::other(
                        "configured import input-byte budget exceeded",
                    ))
                }
                Err(error) => Err(error),
            };
        }

        let remaining = limit - self.consumed;
        let allowed = usize::try_from(remaining)
            .unwrap_or(usize::MAX)
            .min(buffer.len());

        let count = self.inner.read(&mut buffer[..allowed])?;

        self.consumed = self
            .consumed
            .checked_add(count as u64)
            .ok_or_else(|| {
                io::Error::other("import byte counter overflow")
            })?;

        Ok(count)
    }
}

// =============================================================================
// Errors
// =============================================================================

/// Structured import failure.
///
/// These errors deliberately remain independent of the canonical ZUIR
/// diagnostic renderer. The integration layer may map them to `ZuirError`
/// and `Diagnostic` without making this module depend on those systems.
#[derive(Debug)]
pub enum ImportError {
    /// Format identity, version, descriptor, or capability negotiation failed.
    Format(FormatError),

    /// An adapter for the exact format/version is already registered.
    DuplicateAdapter {
        format_id: FormatId,
        version: FormatVersion,
    },

    /// The adapter consumed more input than the configured budget permits.
    InputLimitExceeded {
        limit: u64,
        source_uri: Option<String>,
    },

    /// An import context violates its own invariants.
    InvalidContext(String),

    /// The adapter rejected the input or could not preserve its semantics.
    Adapter {
        format_id: FormatId,
        version: FormatVersion,
        message: String,
    },

    /// An input stream failed while being consumed.
    Io {
        source_uri: Option<String>,
        source: io::Error,
    },

    /// A registry invariant failed. This indicates an implementation or
    /// integration defect rather than an ordinary malformed source file.
    InternalInvariant {
        message: String,
    },
}

impl ImportError {
    /// Wrap an adapter error with the identity of the responsible format.
    #[must_use]
    pub fn adapter(
        format_id: FormatId,
        version: FormatVersion,
        message: impl Into<String>,
    ) -> Self {
        Self::Adapter {
            format_id,
            version,
            message: message.into(),
        }
    }

    /// Convert a reader failure into an import error with optional provenance.
    #[must_use]
    pub fn io(source_uri: Option<String>, source: io::Error) -> Self {
        Self::Io { source_uri, source }
    }
}

impl fmt::Display for ImportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Format(error) => {
                write!(f, "import format negotiation failed: {error}")
            }
            Self::DuplicateAdapter { format_id, version } => {
                write!(
                    f,
                    "import adapter already registered for {format_id} version {version}"
                )
            }
            Self::InputLimitExceeded { limit, source_uri } => {
                write!(
                    f,
                    "import input exceeded the configured byte budget of {limit} bytes"
                )?;

                if let Some(uri) = source_uri {
                    write!(f, " (source: {uri})")?;
                }

                Ok(())
            }
            Self::InvalidContext(message) => {
                write!(f, "invalid import context: {message}")
            }
            Self::Adapter {
                format_id,
                version,
                message,
            } => {
                write!(
                    f,
                    "import adapter {format_id} version {version} failed: {message}"
                )
            }
            Self::Io { source_uri, source } => {
                write!(f, "import input I/O failure: {source}")?;

                if let Some(uri) = source_uri {
                    write!(f, " (source: {uri})")?;
                }

                Ok(())
            }
            Self::InternalInvariant { message } => {
                write!(f, "ZUIR import registry invariant failed: {message}")
            }
        }
    }
}

impl Error for ImportError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Format(error) => Some(error),
            Self::Io { source, .. } => Some(source),
            _ => None,
        }
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zuir::interchange::format::{
        CapabilityId, FormatDescriptor,
    };
    use std::io::Cursor;

    struct TextAdapter {
        descriptor: FormatDescriptor,
    }

    impl InterchangeFormat for TextAdapter {
        fn descriptor(&self) -> &FormatDescriptor {
            &self.descriptor
        }
    }

    impl ImportAdapter<String> for TextAdapter {
        fn import(
            &self,
            input: &mut dyn Read,
            _context: &ImportContext,
        ) -> ImportResult<String> {
            let mut bytes = Vec::new();

            input
                .read_to_end(&mut bytes)
                .map_err(|error| {
                    ImportError::io(None, error)
                })?;

            String::from_utf8(bytes).map_err(|error| {
                ImportError::adapter(
                    self.descriptor.id().clone(),
                    self.descriptor.version(),
                    format!("input is not valid UTF-8: {error}"),
                )
            })
        }
    }

    fn id(value: &str) -> FormatId {
        FormatId::new(value).expect("valid format identifier")
    }

    fn version() -> FormatVersion {
        FormatVersion::new(1, 0, 0)
    }

    fn capability(value: &str) -> CapabilityId {
        CapabilityId::new(value).expect("valid capability identifier")
    }

    fn adapter() -> TextAdapter {
        let descriptor =
            FormatDescriptor::new(id("example.text"), version())
                .with_capability(capability("encoding.text"));

        TextAdapter { descriptor }
    }

    fn requirement() -> FormatRequirement {
        FormatRequirement::new(id("example.text"), version())
            .requiring(capability("encoding.text"))
    }

    #[test]
    fn imports_using_an_exact_registered_adapter() {
        let mut registry = ImportRegistry::<String>::new();
        registry.register(adapter()).expect("register adapter");

        let mut input = Cursor::new(b"hello Zamani".to_vec());

        let result = registry
            .import_exact(
                &requirement(),
                &mut input,
                &ImportContext::new(),
            )
            .expect("import succeeds");

        assert_eq!(result, "hello Zamani");
    }

    #[test]
    fn rejects_duplicate_format_version_registration() {
        let mut registry = ImportRegistry::<String>::new();

        registry.register(adapter()).expect("first registration");

        assert!(matches!(
            registry.register(adapter()),
            Err(ImportError::DuplicateAdapter { .. })
        ));

        assert_eq!(registry.len(), 1);
    }

    #[test]
    fn rejects_an_unregistered_format() {
        let registry = ImportRegistry::<String>::new();
        let mut input = Cursor::new(b"data".to_vec());

        assert!(matches!(
            registry.import_exact(
                &requirement(),
                &mut input,
                &ImportContext::new(),
            ),
            Err(ImportError::Format(
                FormatError::UnsupportedFormat { .. }
            ))
        ));
    }

    #[test]
    fn rejects_missing_required_capabilities() {
        let mut registry = ImportRegistry::<String>::new();
        registry.register(adapter()).expect("register adapter");

        let requirement =
            FormatRequirement::new(id("example.text"), version())
                .requiring(capability("semantics.lossless-quantum"));

        let mut input = Cursor::new(b"data".to_vec());

        assert!(matches!(
            registry.import_exact(
                &requirement,
                &mut input,
                &ImportContext::new(),
            ),
            Err(ImportError::Format(
                FormatError::UnsupportedFormat { .. }
            ))
        ));
    }

    #[test]
    fn permits_input_equal_to_the_configured_budget() {
        let mut registry = ImportRegistry::<String>::new();
        registry.register(adapter()).expect("register adapter");

        let mut input = Cursor::new(b"12345".to_vec());

        let result = registry
            .import_exact(
                &requirement(),
                &mut input,
                &ImportContext::new().with_max_input_bytes(5),
            )
            .expect("exact budget is permitted");

        assert_eq!(result, "12345");
    }

    #[test]
    fn rejects_input_larger_than_the_configured_budget() {
        let mut registry = ImportRegistry::<String>::new();
        registry.register(adapter()).expect("register adapter");

        let mut input = Cursor::new(b"123456".to_vec());

        assert!(matches!(
            registry.import_exact(
                &requirement(),
                &mut input,
                &ImportContext::new().with_max_input_bytes(5),
            ),
            Err(ImportError::InputLimitExceeded { limit: 5, .. })
        ));
    }

    #[test]
    fn zero_budget_accepts_empty_input() {
        let mut registry = ImportRegistry::<String>::new();
        registry.register(adapter()).expect("register adapter");

        let mut input = Cursor::new(Vec::<u8>::new());

        let result = registry
            .import_exact(
                &requirement(),
                &mut input,
                &ImportContext::new().with_max_input_bytes(0),
            )
            .expect("empty input is permitted");

        assert!(result.is_empty());
    }

    #[test]
    fn zero_budget_rejects_nonempty_input() {
        let mut registry = ImportRegistry::<String>::new();
        registry.register(adapter()).expect("register adapter");

        let mut input = Cursor::new(b"x".to_vec());

        assert!(matches!(
            registry.import_exact(
                &requirement(),
                &mut input,
                &ImportContext::new().with_max_input_bytes(0),
            ),
            Err(ImportError::InputLimitExceeded { limit: 0, .. })
        ));
    }

    #[test]
    fn rejects_an_empty_source_uri() {
        assert!(matches!(
            ImportContext::new().with_source_uri("  "),
            Err(ImportError::InvalidContext(_))
        ));
    }

    #[test]
    fn exposes_registered_descriptors_deterministically() {
        let mut registry = ImportRegistry::<String>::new();

        registry.register(adapter()).expect("register adapter");

        let descriptors = registry.descriptors();

        assert_eq!(descriptors.len(), 1);
        assert_eq!(descriptors[0].id().as_str(), "example.text");
    }

    #[test]
    fn adapter_errors_retain_their_format_identity() {
        let error = ImportError::adapter(
            id("example.text"),
            version(),
            "unsupported semantic construct",
        );

        let rendered = error.to_string();

        assert!(rendered.contains("example.text"));
        assert!(rendered.contains("unsupported semantic construct"));
    }
}