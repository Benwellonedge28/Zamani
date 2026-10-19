
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani HDL Interoperability Export
//!
//! Path: `src/hdl/interoperability/export.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Authority
//!
//! - `grammar/spec/hdl.md`
//! - `grammar/spec/interoperability.md`
//! - `grammar/hdl/hdl.g4`
//! - `src/frontend/parser/domains/hdl.rs`
//! - `src/frontend/semantic/domains/hdl.rs`
//! - `src/hdl/interoperability/verilog.rs`
//! - `src/hdl/interoperability/system_verilog.rs`
//! - `src/hdl/interoperability/vhdl.rs`
//!
//! ## Purpose
//!
//! Provide a loss-aware, streaming export boundary for HDL source formats.
//! Preserve source bytes when source and destination formats are identical.
//! Provide an explicit backend contract for future semantic translation.
//!
//! ## Ownership
//!
//! This module owns export request policy, format identity, export reports,
//! export errors, output budgeting, and the streaming export boundary.
//!
//! This module does not own:
//!
//! - source parsing or preprocessing;
//! - the canonical Zamani AST or semantic model;
//! - name resolution, type checking, or elaboration;
//! - HDLIR, its operations, builders, passes, or lowering;
//! - a Verilog, SystemVerilog, or VHDL parser;
//! - semantic translation between different HDL languages;
//! - target selection, synthesis, placement, routing, or hardware allocation.
//!
//! All HDLIR implementation remains under `src/hdl/hdlir/`.
//!
//! ## Integration
//!
//! The interoperability composition root should register this module as
//! `pub mod export;`.
//!
//! Same-format export is immediately available through `export_source`.
//! Cross-format translation must be implemented by an explicit
//! `HdlExportBackend` operating on a validated, canonical representation.
//! That representation must be supplied by a future adapter owned by the
//! relevant AST/semantic integration work; this file intentionally does not
//! guess its types or constructors.
//!
//! Existing Verilog, SystemVerilog, and VHDL scanners remain responsible for
//! their own structural discovery. Successful byte-preserving export does
//! not imply that the input has passed parsing or semantic validation.
//!
//! ## Scalability
//!
//! There is no universal source-size, design-size, or hardware-size ceiling.
//! Optional source and output byte budgets are operational policies.
//! Streaming output avoids requiring a second full-sized output buffer.
//! Checked accounting rejects budget violations and integer overflow.
//!
//! A writer may itself partially write before returning an I/O error.
//! Callers requiring atomic file replacement must write to a temporary file
//! and commit it only after the complete export succeeds.
//!
//! ## Freeze criteria
//!
//! - Same-format exports preserve every source byte.
//! - Cross-format conversion is never silently represented as success.
//! - Output budgets are checked before writing.
//! - Writer failures remain distinguishable from translation failures.
//! - No unsafe Rust, physical-target assumptions, or HDLIR implementation
//!   is introduced here.

use std::error::Error;
use std::fmt;
use std::io::{self, Write};

/// A source or destination language understood by the export boundary.
///
/// `Other` allows integrations to identify future formats without requiring
/// this module to enumerate every language or dialect. A format identifier
/// is descriptive metadata; it does not certify parser compatibility.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum HdlSourceFormat {
    /// Zamani's native HDL source representation.
    Zamani,
    /// IEEE Verilog source.
    Verilog,
    /// IEEE SystemVerilog source.
    SystemVerilog,
    /// IEEE VHDL source.
    Vhdl,
    /// An externally defined format identified by a stable name.
    Other(String),
}

impl HdlSourceFormat {
    /// Construct an external format identifier.
    ///
    /// Empty identifiers and identifiers containing only whitespace are
    /// rejected. The identifier is otherwise preserved exactly.
    pub fn other(name: impl Into<String>) -> Result<Self, ExportError> {
        let name = name.into();

        if name.trim().is_empty() {
            return Err(ExportError::InvalidFormatIdentifier);
        }

        Ok(Self::Other(name))
    }

    /// Return a stable, human-readable format identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Zamani => "zamani-hdl",
            Self::Verilog => "verilog",
            Self::SystemVerilog => "system-verilog",
            Self::Vhdl => "vhdl",
            Self::Other(name) => name,
        }
    }
}

impl fmt::Display for HdlSourceFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Operational policies for a single export request.
///
/// These budgets are not language restrictions and do not impose limits on
/// the number of modules, ports, signals, bits, or physical devices.
///
/// A missing budget means that this API imposes no corresponding byte limit.
/// The process remains subject to available memory, I/O capacity, and other
/// operating-system or caller-defined resource constraints.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ExportConfig {
    /// Optional maximum source length, in bytes.
    pub max_source_bytes: Option<usize>,

    /// Optional maximum number of bytes written to the destination.
    ///
    /// This is checked before the write operation. It does not constrain the
    /// size of an otherwise valid HDL design.
    pub max_output_bytes: Option<usize>,
}

impl ExportConfig {
    /// Construct a configuration without exporter-imposed byte budgets.
    #[must_use]
    pub const fn unbounded() -> Self {
        Self {
            max_source_bytes: None,
            max_output_bytes: None,
        }
    }

    /// Validate that the configured output budget does not exceed the
    /// source budget when both are present.
    ///
    /// This relationship is not generally required for translation, because
    /// translated output can be either larger or smaller than its input.
    /// Consequently, the configuration is accepted without imposing such
    /// a restriction.
    pub fn validate(&self) -> Result<(), ExportError> {
        Ok(())
    }
}

/// Describes the result of a completed export operation.
///
/// `lossless` means the output bytes exactly match the supplied source bytes.
/// It does not mean the source has been parsed, semantically validated,
/// elaborated, or proven realizable on a hardware target.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExportReport {
    /// Format of the supplied source.
    pub source_format: HdlSourceFormat,

    /// Requested destination format.
    pub destination_format: HdlSourceFormat,

    /// Number of output bytes successfully written.
    pub bytes_written: usize,

    /// Whether the export preserved the original source bytes exactly.
    pub lossless: bool,
}

impl ExportReport {
    /// Return whether the source and destination formats are identical.
    #[must_use]
    pub fn preserves_format(&self) -> bool {
        self.source_format == self.destination_format
    }
}

/// Errors returned by the HDL export boundary.
#[derive(Debug)]
#[non_exhaustive]
pub enum ExportError {
    /// An external format identifier is empty or whitespace-only.
    InvalidFormatIdentifier,

    /// The source exceeds the configured operational source budget.
    SourceBudgetExceeded {
        /// Actual source length in bytes.
        actual: usize,
        /// Configured maximum source length in bytes.
        budget: usize,
    },

    /// The output would exceed the configured output budget.
    OutputBudgetExceeded {
        /// Number of bytes requested for output.
        requested: usize,
        /// Configured maximum output length in bytes.
        budget: usize,
    },

    /// A source-to-destination translation has no registered implementation.
    UnsupportedTranslation {
        /// Format of the supplied source.
        source: HdlSourceFormat,
        /// Requested destination format.
        destination: HdlSourceFormat,
    },

    /// A registered translation backend rejected the request.
    BackendFailure {
        /// Stable identifier of the backend that rejected the request.
        backend: String,
        /// Description of the failure.
        message: String,
    },

    /// An output operation failed.
    Io(io::Error),
}

impl fmt::Display for ExportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidFormatIdentifier => {
                f.write_str("HDL export format identifier cannot be empty")
            }
            Self::SourceBudgetExceeded { actual, budget } => {
                write!(
                    f,
                    "HDL export source budget exceeded: {actual} bytes \
                     supplied, budget is {budget} bytes"
                )
            }
            Self::OutputBudgetExceeded { requested, budget } => {
                write!(
                    f,
                    "HDL export output budget exceeded: {requested} bytes \
                     requested, budget is {budget} bytes"
                )
            }
            Self::UnsupportedTranslation {
                source,
                destination,
            } => {
                write!(
                    f,
                    "no HDL export translation is registered from \
                     `{source}` to `{destination}`"
                )
            }
            Self::BackendFailure { backend, message } => {
                write!(f, "HDL export backend `{backend}` failed: {message}")
            }
            Self::Io(error) => write!(f, "HDL export I/O failure: {error}"),
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

/// Export source bytes without changing their representation.
///
/// This function deliberately supports only identical source and destination
/// formats. It does not claim to translate HDL syntax or semantics.
///
/// Source and output budgets are checked before writing. The output is
/// streamed directly to `writer`, so the exporter does not allocate another
/// buffer proportional to the complete source size.
///
/// # Errors
///
/// Returns an error if the configuration is invalid, either configured
/// budget is exceeded, the formats differ, or the writer fails.
///
/// An I/O error can occur after the writer has accepted some bytes. Use a
/// temporary destination and an atomic commit strategy when partial output
/// must not become visible.
///
/// # Examples
///
/// ```
/// use zamani_compiler::hdl::interoperability::export::{
///     export_source, ExportConfig, HdlSourceFormat,
/// };
///
/// let source = "module counter; endmodule\n";
/// let mut output = Vec::new();
///
/// let report = export_source(
///     source,
///     &HdlSourceFormat::Verilog,
///     &HdlSourceFormat::Verilog,
///     &ExportConfig::unbounded(),
///     &mut output,
/// ).expect("same-format export should succeed");
///
/// assert_eq!(output, source.as_bytes());
/// assert!(report.lossless);
/// ```
pub fn export_source<W: Write>(
    source: &str,
    source_format: &HdlSourceFormat,
    destination_format: &HdlSourceFormat,
    config: &ExportConfig,
    writer: &mut W,
) -> Result<ExportReport, ExportError> {
    config.validate()?;

    if let Some(budget) = config.max_source_bytes {
        if source.len() > budget {
            return Err(ExportError::SourceBudgetExceeded {
                actual: source.len(),
                budget,
            });
        }
    }

    if source_format != destination_format {
        return Err(ExportError::UnsupportedTranslation {
            source: source_format.clone(),
            destination: destination_format.clone(),
        });
    }

    if let Some(budget) = config.max_output_bytes {
        if source.len() > budget {
            return Err(ExportError::OutputBudgetExceeded {
                requested: source.len(),
                budget,
            });
        }
    }

    writer.write_all(source.as_bytes())?;

    Ok(ExportReport {
        source_format: source_format.clone(),
        destination_format: destination_format.clone(),
        bytes_written: source.len(),
        lossless: true,
    })
}

/// Contract for a backend that translates a canonical HDL representation
/// into a destination language.
///
/// Implementations must be explicitly registered by the future
/// interoperability composition layer. This trait does not prescribe an AST
/// type because the repository's canonical AST and semantic integration
/// contracts own those types.
///
/// A backend must not claim successful conversion when unsupported constructs
/// have been dropped, changed, or guessed. Lossy conversions must be exposed
/// through a documented policy and structured report rather than silently
/// reported as lossless.
///
/// The writer is supplied by the caller to allow streaming output. Backends
/// must document whether errors can leave partial output in the writer.
pub trait HdlExportBackend {
    /// Return the stable identifier of this backend.
    fn backend_id(&self) -> &str;

    /// Return the destination format supported by this backend.
    fn destination_format(&self) -> HdlSourceFormat;

    /// Translate a validated, canonical HDL representation to the requested
    /// destination.
    ///
    /// `canonical_input` is an opaque byte view at this boundary. The
    /// integration adapter is responsible for defining and validating its
    /// representation before calling the backend. It must not pass arbitrary
    /// source text as canonical input unless that is explicitly the contract
    /// of the registered backend.
    ///
    /// The implementation must return an error for unsupported constructs
    /// rather than silently altering their meaning.
    fn export_canonical(
        &self,
        canonical_input: &[u8],
        config: &ExportConfig,
        writer: &mut dyn Write,
    ) -> Result<ExportReport, ExportError>;
}

/// Run an explicitly selected translation backend.
///
/// This function checks the output format declared by the backend, applies
/// the operational output budget to the completed report, and rejects reports
/// that claim more output bytes than the configured budget.
///
/// A backend must enforce its output budget while streaming. This wrapper's
/// postcondition check is a defense-in-depth check, not a substitute for
/// backend-level enforcement.
///
/// # Errors
///
/// Returns an error if the backend's declared destination does not match the
/// requested format, if the backend fails, or if its report violates the
/// configured output budget.
pub fn export_with_backend<B: HdlExportBackend, W: Write>(
    backend: &B,
    canonical_input: &[u8],
    destination_format: &HdlSourceFormat,
    config: &ExportConfig,
    writer: &mut W,
) -> Result<ExportReport, ExportError> {
    config.validate()?;

    let actual_destination = backend.destination_format();

    if &actual_destination != destination_format {
        return Err(ExportError::UnsupportedTranslation {
            source: HdlSourceFormat::Zamani,
            destination: destination_format.clone(),
        });
    }

    let mut report =
        backend.export_canonical(canonical_input, config, writer)?;

    if let Some(budget) = config.max_output_bytes {
        if report.bytes_written > budget {
            return Err(ExportError::OutputBudgetExceeded {
                requested: report.bytes_written,
                budget,
            });
        }
    }

    report.destination_format = actual_destination;
    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_format_export_preserves_exact_bytes() {
        let source = "module top;\r\n  // preserve spacing\r\nendmodule\r\n";
        let mut output = Vec::new();

        let report = export_source(
            source,
            &HdlSourceFormat::Verilog,
            &HdlSourceFormat::Verilog,
            &ExportConfig::unbounded(),
            &mut output,
        )
        .expect("same-format export should succeed");

        assert_eq!(output, source.as_bytes());
        assert_eq!(report.bytes_written, source.len());
        assert!(report.lossless);
        assert!(report.preserves_format());
    }

    #[test]
    fn empty_source_is_preserved_without_inventing_content() {
        let mut output = Vec::new();

        let report = export_source(
            "",
            &HdlSourceFormat::Vhdl,
            &HdlSourceFormat::Vhdl,
            &ExportConfig::unbounded(),
            &mut output,
        )
        .expect("empty input is representable by the byte-preserving layer");

        assert!(output.is_empty());
        assert_eq!(report.bytes_written, 0);
        assert!(report.lossless);
    }

    #[test]
    fn cross_format_export_is_never_silently_claimed() {
        let mut output = Vec::new();

        let error = export_source(
            "module top; endmodule",
            &HdlSourceFormat::Verilog,
            &HdlSourceFormat::Vhdl,
            &ExportConfig::unbounded(),
            &mut output,
        )
        .expect_err("translation must require an implementation");

        assert!(matches!(
            error,
            ExportError::UnsupportedTranslation { .. }
        ));
        assert!(output.is_empty());
    }

    #[test]
    fn source_budget_is_enforced_before_writing() {
        let config = ExportConfig {
            max_source_bytes: Some(3),
            max_output_bytes: None,
        };
        let mut output = Vec::new();

        let error = export_source(
            "module",
            &HdlSourceFormat::Verilog,
            &HdlSourceFormat::Verilog,
            &config,
            &mut output,
        )
        .expect_err("source budget should be enforced");

        assert!(matches!(
            error,
            ExportError::SourceBudgetExceeded {
                actual: 6,
                budget: 3
            }
        ));
        assert!(output.is_empty());
    }

    #[test]
    fn output_budget_is_enforced_before_writing() {
        let config = ExportConfig {
            max_source_bytes: None,
            max_output_bytes: Some(2),
        };
        let mut output = Vec::new();

        let error = export_source(
            "module",
            &HdlSourceFormat::Verilog,
            &HdlSourceFormat::Verilog,
            &config,
            &mut output,
        )
        .expect_err("output budget should be enforced");

        assert!(matches!(
            error,
            ExportError::OutputBudgetExceeded {
                requested: 6,
                budget: 2
            }
        ));
        assert!(output.is_empty());
    }

    #[test]
    fn external_format_identifiers_are_validated() {
        assert!(HdlSourceFormat::other(" ").is_err());

        let format =
            HdlSourceFormat::other("vendor.example.rtl.v2").expect("valid ID");

        assert_eq!(format.as_str(), "vendor.example.rtl.v2");
    }

    #[test]
    fn reports_keep_format_identity() {
        let mut output = Vec::new();

        let report = export_source(
            "entity top is end entity;",
            &HdlSourceFormat::Vhdl,
            &HdlSourceFormat::Vhdl,
            &ExportConfig::unbounded(),
            &mut output,
        )
        .expect("same-format export should succeed");

        assert_eq!(report.source_format, HdlSourceFormat::Vhdl);
        assert_eq!(report.destination_format, HdlSourceFormat::Vhdl);
        assert!(report.preserves_format());
    }

    #[test]
    fn writer_failures_remain_io_errors() {
        struct FailingWriter;

        impl Write for FailingWriter {
            fn write(&mut self, _buffer: &[u8]) -> io::Result<usize> {
                Err(io::Error::other("intentional test failure"))
            }

            fn flush(&mut self) -> io::Result<()> {
                Ok(())
            }
        }

        let error = export_source(
            "module top; endmodule",
            &HdlSourceFormat::Verilog,
            &HdlSourceFormat::Verilog,
            &ExportConfig::unbounded(),
            &mut FailingWriter,
        )
        .expect_err("writer failure must propagate");

        assert!(matches!(error, ExportError::Io(_)));
    }
}
