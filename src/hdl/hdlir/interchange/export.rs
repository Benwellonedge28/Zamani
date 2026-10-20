
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # HDLIR Export Boundary
//!
//! Path: `src/hdl/hdlir/interchange/export.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Define the backend-independent contract for exporting a verified HDLIR
//! design to an explicitly selected external hardware-description format.
//!
//! This module provides export policy, checked streaming output, structured
//! failures, and an extensible backend trait. It does not implement a
//! Verilog, SystemVerilog, VHDL, or other format encoder.
//!
//! ## Authority
//!
//! - `grammar/spec/hdl.md`
//! - `grammar/spec/interoperability.md`
//! - `src/hdl/api.rs`
//! - `src/hdl/ids.rs`
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/hdl/hdlir/HDLIR-SPEC.md` once established
//! - The actual HDLIR graph and verifier APIs once implemented
//!
//! The grammar and frontend own source syntax and source-level semantics.
//! ZUIR owns universal computational representation. HDLIR owns the
//! hardware-domain representation. This module owns only export orchestration.
//!
//! ## Ownership
//!
//! This module owns:
//! - destination-format identity;
//! - export options and operational output budgets;
//! - the input contract required by an export backend;
//! - explicit export-fidelity reporting;
//! - streaming output-budget enforcement;
//! - export errors and completion reports.
//!
//! It does not own:
//! - the HDLIR graph, its types, operations, or verifier;
//! - source parsing, source-level semantic analysis, or elaboration;
//! - serialization of the canonical HDLIR artifact;
//! - external HDL grammars or standards compliance;
//! - semantic translation algorithms for individual output formats;
//! - synthesis, scheduling, placement, routing, or physical realization.
//!
//! ## Integration contract
//!
//! The eventual HDLIR design type must implement `HdlirExportInput`.
//! A format-specific backend must implement `HdlirExportBackend<D>` for
//! that design type and must emit only syntax and semantics supported by
//! the destination format and its declared dialect/version.
//!
//! Call `export_design` only after the authoritative HDLIR verifier has
//! completed successfully. The input's `is_verified` method is a guard
//! against accidental misuse; it is not a replacement for the verifier.
//!
//! The generic backend trait intentionally avoids inventing a concrete
//! `HdlirDesign` API before that type exists. A backend can be implemented
//! against the real design type when the core IR contract is frozen.
//!
//! ## Scalability
//!
//! This module defines no universal limit on signals, ports, modules, widths,
//! memories, operations, clock domains, or hardware resources.
//!
//! `max_output_bytes` is an optional operational budget, not a language
//! restriction. `None` means this module imposes no output-byte ceiling.
//! The writer and operating system can still impose their own limits.
//!
//! Every output-length increment is checked. Budget exhaustion and arithmetic
//! overflow are explicit failures, never successful truncated exports.
//!
//! ## Failure and atomicity
//!
//! A backend or writer can fail after some output has already been written.
//! The exporter reports the failure but cannot roll back an arbitrary writer.
//! Callers requiring atomic publication should export to a temporary
//! destination and commit it only after the export succeeds.
//!
//! ## Semantic preservation
//!
//! `SemanticsPreserved` may be reported only when the backend's implementation
//! contract justifies that claim. `SemanticsNotGuaranteed` is the conservative
//! option when equivalence has not been established. Neither result implies
//! synthesis success, target feasibility, timing closure, or physical
//! correctness.
//!
//! A backend must return an error for unsupported constructs unless an
//! explicitly specified conversion policy allows their handling. It must
//! never silently discard required semantics.

use std::error::Error;
use std::fmt;
use std::io::{self, Write};

/// A stable identifier for an external HDL output format.
///
/// This identifier describes the requested format. It does not prove that
/// an exporter for that format is implemented or standards-compliant.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct HdlFormatId(String);

impl HdlFormatId {
    /// Create a format identifier.
    ///
    /// The identifier must not be empty or consist entirely of whitespace.
    /// The original value is preserved rather than silently normalized.
    ///
    /// # Errors
    ///
    /// Returns [`ExportError::InvalidFormatId`] if the identifier is empty
    /// or whitespace-only.
    pub fn new(value: impl Into<String>) -> Result<Self, ExportError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(ExportError::InvalidFormatId);
        }

        Ok(Self(value))
    }

    /// Return the exact registered format identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for HdlFormatId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Operational policy for one HDLIR export.
///
/// These options govern exporter work; they do not limit the HDL language or
/// the design sizes that HDLIR may represent.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ExportOptions {
    /// Optional maximum number of output bytes.
    ///
    /// The budget is enforced while writing, before bytes beyond the budget
    /// can be forwarded to the underlying writer. `None` means this module
    /// imposes no output-byte budget.
    pub max_output_bytes: Option<u64>,

    /// Whether the exporter must flush the supplied writer before reporting
    /// success.
    ///
    /// When false, flushing remains the caller's responsibility. When true,
    /// a flush failure is returned as an I/O error.
    pub flush_on_success: bool,
}

impl ExportOptions {
    /// Construct options without an exporter-defined output-byte budget.
    #[must_use]
    pub const fn unbounded() -> Self {
        Self {
            max_output_bytes: None,
            flush_on_success: false,
        }
    }
}

/// Declares the backend's semantic-preservation claim.
///
/// This is a report about the export mapping, not a claim that the design
/// can be synthesized or implemented on any particular physical target.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum SemanticFidelity {
    /// The backend's documented mapping preserves the relevant HDLIR
    /// semantics for the constructs it successfully exports.
    ///
    /// A backend must not use this value merely because output was generated.
    SemanticsPreserved,

    /// The backend cannot establish that all relevant HDLIR semantics are
    /// preserved by the emitted representation.
    SemanticsNotGuaranteed,
}

/// Minimal contract required from a design passed to an exporter.
///
/// Implement this trait for the actual HDLIR design type after its API and
/// verifier are established. The methods must report the state of the same
/// design instance supplied to the backend; they must not manufacture
/// verification success.
///
/// The trait is generic-backend infrastructure, not a competing IR model.
pub trait HdlirExportInput {
    /// Return whether the authoritative HDLIR verifier completed successfully
    /// for this design in its current, unchanged state.
    ///
    /// A design must not continue to report verified after mutation that
    /// invalidates verification. Prefer immutable verified-design wrappers
    /// or verifier-issued proof/state tokens when the core API supports them.
    fn is_verified(&self) -> bool;

    /// Return the schema version under which this design is represented.
    ///
    /// The version must come from the authoritative HDLIR schema/version
    /// contract, not from a backend guess.
    fn schema_version(&self) -> u32;
}

/// Backend contract for exporting one concrete HDLIR design type.
///
/// Implementations should be registered by the future
/// `interchange` composition layer only after the corresponding encoder,
/// dialect mapping, compatibility policy, and conformance tests exist.
///
/// The trait is generic over `D` intentionally: it does not require a
/// speculative universal `HdlirDesign` type or a type-erased parallel IR.
pub trait HdlirExportBackend<D: HdlirExportInput> {
    /// Return a stable identifier for this backend implementation.
    fn backend_id(&self) -> &str;

    /// Return the destination format supported by this backend.
    fn destination_format(&self) -> HdlFormatId;

    /// Return the HDLIR schema versions this backend can consume.
    ///
    /// The list may be empty, in which case the backend supports no schema
    /// versions and export is rejected. Backends must not claim support for
    /// a version they cannot correctly interpret.
    fn supported_schema_versions(&self) -> &[u32];

    /// Encode the design into the supplied writer.
    ///
    /// The writer passed here enforces the configured output budget. The
    /// backend must propagate write failures and must not convert them into
    /// successful reports.
    ///
    /// The backend must reject unsupported constructs or explicitly defined
    /// incompatible semantics. It must not silently omit required behavior.
    ///
    /// The returned fidelity describes the backend's semantic mapping. It
    /// does not establish physical realizability or target feasibility.
    ///
    /// # Errors
    ///
    /// Return an explanatory message when the design cannot be represented
    /// faithfully under the backend's declared mapping contract. Writer
    /// failures are reported separately by the output boundary.
    fn encode(
        &self,
        design: &D,
        writer: &mut dyn Write,
    ) -> Result<SemanticFidelity, String>;
}

/// Result of a completed HDLIR export.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExportReport {
    /// Stable identifier of the selected backend.
    pub backend_id: String,

    /// Destination format emitted by the backend.
    pub destination_format: HdlFormatId,

    /// HDLIR schema version consumed by the backend.
    pub input_schema_version: u32,

    /// Number of bytes successfully accepted by the underlying writer.
    pub bytes_written: u64,

    /// Semantic-preservation claim returned by the backend.
    pub semantic_fidelity: SemanticFidelity,
}

impl ExportReport {
    /// Return whether the backend reported semantic preservation.
    ///
    /// This does not mean that a target has been synthesized, validated,
    /// physically implemented, or timing-closed.
    #[must_use]
    pub const fn claims_semantic_preservation(&self) -> bool {
        matches!(
            self.semantic_fidelity,
            SemanticFidelity::SemanticsPreserved
        )
    }
}

/// Errors from the HDLIR export boundary.
#[derive(Debug)]
#[non_exhaustive]
pub enum ExportError {
    /// A destination format identifier was empty or whitespace-only.
    InvalidFormatId,

    /// The supplied design has not passed the required HDLIR verification.
    UnverifiedDesign,

    /// The backend does not support the design's HDLIR schema version.
    UnsupportedSchemaVersion {
        /// Schema version of the supplied design.
        actual: u32,
        /// Schema versions supported by the selected backend.
        supported: Vec<u32>,
    },

    /// The backend identifier was empty or whitespace-only.
    InvalidBackendId,

    /// The backend does not support the requested output format.
    UnsupportedDestination {
        /// Requested destination format.
        requested: HdlFormatId,
        /// Format declared by the backend.
        backend_format: HdlFormatId,
    },

    /// The output budget would be exceeded.
    OutputBudgetExceeded {
        /// Bytes already accepted by the underlying writer.
        written: u64,
        /// Number of additional bytes requested by the backend.
        requested: u64,
        /// Configured maximum output size.
        budget: u64,
    },

    /// Output byte accounting overflowed its representable range.
    OutputLengthOverflow,

    /// The selected backend could not encode the design.
    BackendFailure {
        /// Stable identifier of the backend.
        backend_id: String,
        /// Explanation supplied by the backend.
        message: String,
    },

    /// The underlying writer failed.
    Io(io::Error),
}

impl fmt::Display for ExportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidFormatId => {
                f.write_str("HDLIR export format identifier cannot be empty")
            }
            Self::UnverifiedDesign => {
                f.write_str("HDLIR export requires a successfully verified design")
            }
            Self::UnsupportedSchemaVersion { actual, supported } => {
                write!(
                    f,
                    "HDLIR schema version {actual} is not supported by the \
                     selected backend; supported versions: {supported:?}"
                )
            }
            Self::InvalidBackendId => {
                f.write_str("HDLIR export backend identifier cannot be empty")
            }
            Self::UnsupportedDestination {
                requested,
                backend_format,
            } => {
                write!(
                    f,
                    "requested destination `{requested}` does not match \
                     backend destination `{backend_format}`"
                )
            }
            Self::OutputBudgetExceeded {
                written,
                requested,
                budget,
            } => {
                write!(
                    f,
                    "HDLIR export output budget exceeded: {written} bytes \
                     already written, {requested} more requested, budget \
                     is {budget} bytes"
                )
            }
            Self::OutputLengthOverflow => {
                f.write_str("HDLIR export output byte count overflowed")
            }
            Self::BackendFailure {
                backend_id,
                message,
            } => {
                write!(f, "HDLIR export backend `{backend_id}` failed: {message}")
            }
            Self::Io(error) => write!(f, "HDLIR export I/O failure: {error}"),
        }
    }
}

impl Error for ExportError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            _ => None,
        }
    }
}

impl From<io::Error> for ExportError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

/// Writer that enforces a byte budget and counts successfully written bytes.
///
/// This wrapper is intentionally private. All backends invoked through
/// `export_design` write through it, so the limit is enforced incrementally
/// rather than only after a potentially enormous output has been generated.
struct BudgetWriter<'a> {
    inner: &'a mut dyn Write,
    budget: Option<u64>,
    written: u64,
}

impl<'a> BudgetWriter<'a> {
    fn new(inner: &'a mut dyn Write, budget: Option<u64>) -> Self {
        Self {
            inner,
            budget,
            written: 0,
        }
    }

    fn bytes_written(&self) -> u64 {
        self.written
    }
}

impl Write for BudgetWriter<'_> {
    fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        if buffer.is_empty() {
            return Ok(0);
        }

        let requested = u64::try_from(buffer.len()).map_err(|_| {
            io::Error::other(ExportError::OutputLengthOverflow)
        })?;

        let next = self.written.checked_add(requested).ok_or_else(|| {
            io::Error::other(ExportError::OutputLengthOverflow)
        })?;

        if let Some(budget) = self.budget {
            if next > budget {
                let error = ExportError::OutputBudgetExceeded {
                    written: self.written,
                    requested,
                    budget,
                };
                return Err(io::Error::other(error));
            }
        }

        let count = self.inner.write(buffer)?;
        let count = u64::try_from(count).map_err(|_| {
            io::Error::other(ExportError::OutputLengthOverflow)
        })?;

        self.written = self.written.checked_add(count).ok_or_else(|| {
            io::Error::other(ExportError::OutputLengthOverflow)
        })?;

        Ok(usize::try_from(count).unwrap_or(usize::MAX))
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}

/// Export a verified HDLIR design through an explicitly selected backend.
///
/// The design type `D` is the actual HDLIR design type and must implement
/// [`HdlirExportInput`]. The backend must implement
/// [`HdlirExportBackend<D>`].
///
/// Output is streamed directly to the caller's writer. The exporter does not
/// allocate a second complete output buffer.
///
/// # Errors
///
/// Returns an error if the design is unverified, the backend identifier is
/// invalid, the backend does not support the design schema or requested
/// destination, the backend rejects the design, the output budget is
/// exceeded, output accounting overflows, or the writer fails.
///
/// A failed export can leave partial output in the writer. Use a temporary
/// file or another transactional sink if partial output must not be published.
pub fn export_design<D, B, W>(
    design: &D,
    backend: &B,
    destination: &HdlFormatId,
    options: &ExportOptions,
    writer: &mut W,
) -> Result<ExportReport, ExportError>
where
    D: HdlirExportInput,
    B: HdlirExportBackend<D>,
    W: Write,
{
    if !design.is_verified() {
        return Err(ExportError::UnverifiedDesign);
    }

    let backend_id = backend.backend_id().to_owned();
    if backend_id.trim().is_empty() {
        return Err(ExportError::InvalidBackendId);
    }

    let backend_destination = backend.destination_format();
    if &backend_destination != destination {
        return Err(ExportError::UnsupportedDestination {
            requested: destination.clone(),
            backend_format: backend_destination,
        });
    }

    let schema_version = design.schema_version();
    let supported_versions = backend.supported_schema_versions();

    if !supported_versions.contains(&schema_version) {
        return Err(ExportError::UnsupportedSchemaVersion {
            actual: schema_version,
            supported: supported_versions.to_vec(),
        });
    }

    let mut budget_writer =
        BudgetWriter::new(writer, options.max_output_bytes);

    let semantic_fidelity = backend
        .encode(design, &mut budget_writer)
        .map_err(|message| ExportError::BackendFailure {
            backend_id: backend_id.clone(),
            message,
        })?;

    if options.flush_on_success {
        budget_writer.flush()?;
    }

    Ok(ExportReport {
        backend_id,
        destination_format: destination.clone(),
        input_schema_version: schema_version,
        bytes_written: budget_writer.bytes_written(),
        semantic_fidelity,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestDesign {
        verified: bool,
        schema_version: u32,
        payload: &'static [u8],
    }

    impl HdlirExportInput for TestDesign {
        fn is_verified(&self) -> bool {
            self.verified
        }

        fn schema_version(&self) -> u32 {
            self.schema_version
        }
    }

    struct TestBackend {
        id: &'static str,
        format: HdlFormatId,
        versions: Vec<u32>,
    }

    impl HdlirExportBackend<TestDesign> for TestBackend {
        fn backend_id(&self) -> &str {
            self.id
        }

        fn destination_format(&self) -> HdlFormatId {
            self.format.clone()
        }

        fn supported_schema_versions(&self) -> &[u32] {
            &self.versions
        }

        fn encode(
            &self,
            design: &TestDesign,
            writer: &mut dyn Write,
        ) -> Result<SemanticFidelity, String> {
            writer
                .write_all(design.payload)
                .map_err(|error| error.to_string())?;

            Ok(SemanticFidelity::SemanticsPreserved)
        }
    }

    fn format(name: &str) -> HdlFormatId {
        HdlFormatId::new(name).expect("test format should be valid")
    }

    fn design(payload: &'static [u8]) -> TestDesign {
        TestDesign {
            verified: true,
            schema_version: 1,
            payload,
        }
    }

    fn backend() -> TestBackend {
        TestBackend {
            id: "test.backend",
            format: format("test-hdl"),
            versions: vec![1],
        }
    }

    #[test]
    fn format_id_rejects_empty_values() {
        assert!(matches!(
            HdlFormatId::new(" \t"),
            Err(ExportError::InvalidFormatId)
        ));
    }

    #[test]
    fn verified_design_is_streamed_and_reported() {
        let input = design(b"exported design");
        let backend = backend();
        let destination = format("test-hdl");
        let mut output = Vec::new();

        let report = export_design(
            &input,
            &backend,
            &destination,
            &ExportOptions::unbounded(),
            &mut output,
        )
        .expect("verified design should export");

        assert_eq!(output, b"exported design");
        assert_eq!(report.bytes_written, 15);
        assert_eq!(report.input_schema_version, 1);
        assert!(report.claims_semantic_preservation());
    }

    #[test]
    fn unverified_design_is_rejected_before_writing() {
        let input = TestDesign {
            verified: false,
            schema_version: 1,
            payload: b"must not be written",
        };
        let backend = backend();
        let destination = format("test-hdl");
        let mut output = Vec::new();

        let result = export_design(
            &input,
            &backend,
            &destination,
            &ExportOptions::unbounded(),
            &mut output,
        );

        assert!(matches!(result, Err(ExportError::UnverifiedDesign)));
        assert!(output.is_empty());
    }

    #[test]
    fn unsupported_schema_is_rejected_before_writing() {
        let input = TestDesign {
            verified: true,
            schema_version: 9,
            payload: b"must not be written",
        };
        let backend = backend();
        let destination = format("test-hdl");
        let mut output = Vec::new();

        let result = export_design(
            &input,
            &backend,
            &destination,
            &ExportOptions::unbounded(),
            &mut output,
        );

        assert!(matches!(
            result,
            Err(ExportError::UnsupportedSchemaVersion { actual: 9, .. })
        ));
        assert!(output.is_empty());
    }

    #[test]
    fn output_budget_is_enforced_during_writing() {
        let input = design(b"12345");
        let backend = backend();
        let destination = format("test-hdl");
        let options = ExportOptions {
            max_output_bytes: Some(4),
            flush_on_success: false,
        };
        let mut output = Vec::new();

        let result =
            export_design(&input, &backend, &destination, &options, &mut output);

        assert!(matches!(
            result,
            Err(ExportError::BackendFailure { .. })
        ));
        assert!(output.len() <= 4);
    }

    #[test]
    fn destination_mismatch_is_rejected_before_writing() {
        let input = design(b"payload");
        let backend = backend();
        let destination = format("different-format");
        let mut output = Vec::new();

        let result = export_design(
            &input,
            &backend,
            &destination,
            &ExportOptions::unbounded(),
            &mut output,
        );

        assert!(matches!(
            result,
            Err(ExportError::UnsupportedDestination { .. })
        ));
        assert!(output.is_empty());
    }

    #[test]
    fn empty_design_output_is_supported() {
        let input = design(b"");
        let backend = backend();
        let destination = format("test-hdl");
        let mut output = Vec::new();

        let report = export_design(
            &input,
            &backend,
            &destination,
            &ExportOptions::unbounded(),
            &mut output,
        )
        .expect("empty output is valid when the backend permits it");

        assert!(output.is_empty());
        assert_eq!(report.bytes_written, 0);
    }

    #[test]
    fn writer_errors_are_reported() {
        struct FailingWriter;

        impl Write for FailingWriter {
            fn write(&mut self, _buffer: &[u8]) -> io::Result<usize> {
                Err(io::Error::other("intentional test failure"))
            }

            fn flush(&mut self) -> io::Result<()> {
                Ok(())
            }
        }

        let input = design(b"payload");
        let backend = backend();
        let destination = format("test-hdl");
        let mut writer = FailingWriter;

        let result = export_design(
            &input,
            &backend,
            &destination,
            &ExportOptions::unbounded(),
            &mut writer,
        );

        assert!(matches!(result, Err(ExportError::BackendFailure { .. })));
    }
}
