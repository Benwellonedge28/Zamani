
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! Zamani backend universal-IR interchange adapter.
//!
//! Path: `src/backend/interchange/universal_ir.rs`
//! Edition: Rust 2021.
//! Minimum supported Rust version: Rust 1.97.
//!
//! # Purpose
//!
//! Provide a format-neutral, verified boundary for exporting the repository's
//! existing legacy `IrModule` through a caller-selected interchange encoder.
//!
//! This module does not define a new IR or invent a universal serialization
//! format. A concrete exporter remains responsible for its format's syntax,
//! schema, version, capability declarations, and semantic-preservation rules.
//!
//! # Architectural ownership
//!
//! - `crate::ir_gen` owns the legacy `IrModule` representation.
//! - `crate::ir_verify` owns structural verification of that representation.
//! - Concrete interchange exporters own format-specific encoding.
//! - `crate::zuir` owns the intended canonical ZUIR architecture.
//! - `crate::quantum::ir` and the existing HDL/domain IRs retain their
//!   respective domain ownership.
//! - Compiler orchestration and file publication remain outside this module.
//!
//! This adapter must not treat legacy IR text as canonical ZUIR, silently
//! discard unsupported operations, or claim that an exported artifact is
//! executable merely because encoding succeeded.
//!
//! # Resource policy
//!
//! No universal program-size, operation-count, or hardware-capacity ceiling
//! is introduced. An optional output-byte budget is a caller-selected
//! operational safeguard, not a language or IR semantic limit.
//!
//! The streaming API checks writes before accepting bytes into its internal
//! buffer. An exporter can still allocate memory internally; exporters must
//! avoid unbounded intermediate allocations when operating on large inputs.
//!
//! # Integration
//!
//! 1. `src/backend.rs` remains the legacy backend root until its migration is
//!    deliberately completed.
//! 2. The backend root must declare `pub mod interchange;`.
//! 3. `src/backend/interchange/mod.rs` must declare and selectively re-export
//!    this module.
//! 4. The existing exporter implementation should be connected through
//!    `export_to` or `export_text`; its actual API must be verified before
//!    adding a concrete adapter.
//! 5. A future native-ZUIR exporter must consume the canonical ZUIR model
//!    through a separate typed adapter. It must not pretend `IrModule` is
//!    interchangeable with ZUIR.
//!
//! # Grammar boundary
//!
//! The Zamani grammar defines source syntax. This module consumes an already
//! constructed and structurally verified IR module. It must not parse source,
//! import ANTLR types, perform semantic analysis, or reconstruct meaning from
//! grammar productions.

use std::error::Error;
use std::fmt;
use std::io::{self, Write};

use crate::ir_gen::IrModule;

/// A completed interchange artifact.
///
/// `format_id` identifies the encoder's declared output format. It does not
/// independently establish schema validity, semantic equivalence, runtime
/// compatibility, or executability.
///
/// `bytes` stores the exact output produced by the selected encoder.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UniversalIrArtifact {
    /// Concrete format identifier selected by the caller.
    pub format_id: String,

    /// Name of the source IR module.
    pub module_name: String,

    /// Complete encoded artifact.
    pub bytes: Vec<u8>,

    /// Number of bytes in `bytes`.
    pub size_bytes: usize,
}

impl UniversalIrArtifact {
    /// Return the artifact bytes without copying them.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Consume the artifact and return its bytes.
    #[must_use]
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }

    /// Interpret the artifact as UTF-8 text.
    ///
    /// This conversion is fallible because interchange formats are not
    /// necessarily textual.
    pub fn into_utf8(self) -> Result<String, std::string::FromUtf8Error> {
        String::from_utf8(self.bytes)
    }
}

/// Optional operational limits for one export operation.
///
/// `max_output_bytes == None` means this adapter imposes no output-byte
/// budget. It does not promise unlimited memory, time, or system resources.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct UniversalIrExportOptions {
    /// Maximum accepted encoded output size, in bytes.
    pub max_output_bytes: Option<usize>,
}

impl UniversalIrExportOptions {
    /// Create options without an output-byte budget.
    #[must_use]
    pub const fn unbounded() -> Self {
        Self {
            max_output_bytes: None,
        }
    }

    /// Create options with an explicit output-byte budget.
    #[must_use]
    pub const fn with_max_output_bytes(max_output_bytes: usize) -> Self {
        Self {
            max_output_bytes: Some(max_output_bytes),
        }
    }
}

/// Errors returned by the universal-IR interchange boundary.
#[derive(Debug)]
pub enum UniversalIrExportError {
    /// The requested format identifier is malformed.
    InvalidFormatId {
        /// Supplied identifier.
        format_id: String,

        /// Explanation of the invalid identifier.
        reason: &'static str,
    },

    /// The authoritative legacy IR verifier rejected the input.
    Verification(Vec<crate::ir_verify::IrVerificationError>),

    /// The selected encoder failed.
    Encoder {
        /// Selected format identifier.
        format_id: String,

        /// Encoder-provided error message.
        message: String,
    },

    /// An encoder attempted to exceed the configured output budget.
    OutputLimitExceeded {
        /// Configured maximum.
        limit_bytes: usize,

        /// Number of bytes already accepted by the output buffer.
        written_bytes: usize,

        /// Size of the write that could not be accepted.
        attempted_write_bytes: usize,
    },

    /// The encoder or output adapter encountered an I/O error.
    OutputIo {
        /// Explanation from the writer.
        message: String,
    },

    /// An internal artifact invariant was violated.
    ArtifactInvariant {
        /// Expected byte count.
        expected_bytes: usize,

        /// Actual byte count recorded in metadata.
        actual_bytes: usize,
    },
}

impl fmt::Display for UniversalIrExportError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidFormatId { format_id, reason } => {
                write!(
                    formatter,
                    "invalid interchange format identifier `{format_id}`: {reason}"
                )
            }

            Self::Verification(errors) => {
                write!(
                    formatter,
                    "IR verification failed with {} error(s)",
                    errors.len()
                )?;

                for error in errors {
                    write!(formatter, "\n- {error}")?;
                }

                Ok(())
            }

            Self::Encoder { format_id, message } => {
                write!(
                    formatter,
                    "interchange encoder `{format_id}` failed: {message}"
                )
            }

            Self::OutputLimitExceeded {
                limit_bytes,
                written_bytes,
                attempted_write_bytes,
            } => write!(
                formatter,
                "interchange output budget exceeded: limit={limit_bytes} bytes, \
                 accepted={written_bytes} bytes, rejected write={attempted_write_bytes} bytes"
            ),

            Self::OutputIo { message } => {
                write!(formatter, "interchange output failed: {message}")
            }

            Self::ArtifactInvariant {
                expected_bytes,
                actual_bytes,
            } => write!(
                formatter,
                "interchange artifact size invariant failed: \
                 expected {expected_bytes} bytes, received {actual_bytes}"
            ),
        }
    }
}

impl Error for UniversalIrExportError {}

/// A checked output writer for one export operation.
///
/// The writer is private to the export call. If the budget is exceeded, the
/// current write is rejected and the operation fails; no partial artifact is
/// returned as a successful result.
struct BudgetedBuffer {
    bytes: Vec<u8>,
    max_output_bytes: Option<usize>,
    limit_error: Option<UniversalIrExportError>,
}

impl BudgetedBuffer {
    fn new(max_output_bytes: Option<usize>) -> Self {
        Self {
            bytes: Vec::new(),
            max_output_bytes,
            limit_error: None,
        }
    }

    fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }

    fn take_limit_error(&mut self) -> Option<UniversalIrExportError> {
        self.limit_error.take()
    }
}

impl Write for BudgetedBuffer {
    fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        if buffer.is_empty() {
            return Ok(0);
        }

        let attempted_size = self.bytes.len().checked_add(buffer.len());

        let exceeds_limit = match (self.max_output_bytes, attempted_size) {
            (Some(limit), Some(size)) => size > limit,
            (Some(_), None) => true,
            (None, _) => false,
        };

        if exceeds_limit {
            let limit = self.max_output_bytes.unwrap_or(usize::MAX);

            self.limit_error = Some(
                UniversalIrExportError::OutputLimitExceeded {
                    limit_bytes: limit,
                    written_bytes: self.bytes.len(),
                    attempted_write_bytes: buffer.len(),
                },
            );

            return Err(io::Error::new(
                io::ErrorKind::Other,
                "interchange output-byte budget exceeded",
            ));
        }

        self.bytes
            .try_reserve(buffer.len())
            .map_err(|error| {
                io::Error::new(
                    io::ErrorKind::Other,
                    format!("unable to reserve output memory: {error}"),
                )
            })?;

        self.bytes.extend_from_slice(buffer);
        Ok(buffer.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        // The buffer is memory-backed. There is no external stream to flush.
        Ok(())
    }
}

/// Export an IR module using a streaming, caller-supplied encoder.
///
/// The module is structurally verified before the encoder is called.
/// `encode` receives the IR's existing textual representation and a bounded
/// writer. The encoder owns the output format's schema and semantics.
///
/// # Encoder contract
///
/// The encoder must:
///
/// - implement the explicitly requested format;
/// - preserve every semantic property it claims to support;
/// - reject unsupported constructs instead of silently dropping them;
/// - propagate writer errors instead of treating them as successful output;
/// - avoid assuming that the legacy IR text is a universal canonical format.
///
/// # Output budget
///
/// The output writer enforces the configured byte budget as bytes are
/// accepted. The encoder may still allocate its own intermediate buffers;
/// a strict whole-process memory budget requires cooperation from the
/// encoder and its toolchain.
///
/// # Errors
///
/// Verification failures, encoder failures, writer failures, and output
/// budget failures remain distinguishable.
pub fn export_to<E, F>(
    module: &IrModule,
    format_id: &str,
    options: UniversalIrExportOptions,
    encode: F,
) -> Result<UniversalIrArtifact, UniversalIrExportError>
where
    E: fmt::Display,
    F: FnOnce(&str, &mut dyn Write) -> Result<(), E>,
{
    validate_format_id(format_id)?;

    crate::ir_verify::verify_module(module)
        .map_err(UniversalIrExportError::Verification)?;

    let ir_text = module.to_ir_string();
    let mut output = BudgetedBuffer::new(options.max_output_bytes);

    if let Err(error) = encode(&ir_text, &mut output) {
        if let Some(limit_error) = output.take_limit_error() {
            return Err(limit_error);
        }

        return Err(UniversalIrExportError::Encoder {
            format_id: format_id.to_owned(),
            message: error.to_string(),
        });
    }

    let bytes = output.into_bytes();
    let size_bytes = bytes.len();

    // Keep the byte-count invariant explicit at the artifact boundary.
    if size_bytes != bytes.as_slice().len() {
        return Err(UniversalIrExportError::ArtifactInvariant {
            expected_bytes: bytes.as_slice().len(),
            actual_bytes: size_bytes,
        });
    }

    Ok(UniversalIrArtifact {
        format_id: format_id.to_owned(),
        module_name: module.name.clone(),
        bytes,
        size_bytes,
    })
}

/// Compatibility adapter for existing encoders that return a complete
/// `String` rather than writing incrementally.
///
/// This method is convenient for integrating legacy functions such as
/// exporters that accept `&str` and return `Result<String, E>`.
///
/// Unlike [`export_to`], the output limit is checked after the legacy encoder
/// has built its string. It therefore limits the accepted artifact size, not
/// the encoder's peak intermediate memory use. Prefer [`export_to`] for new
/// encoders that support incremental output.
pub fn export_text<E, F>(
    module: &IrModule,
    format_id: &str,
    options: UniversalIrExportOptions,
    encode: F,
) -> Result<UniversalIrArtifact, UniversalIrExportError>
where
    E: fmt::Display,
    F: FnOnce(&str) -> Result<String, E>,
{
    validate_format_id(format_id)?;

    crate::ir_verify::verify_module(module)
        .map_err(UniversalIrExportError::Verification)?;

    let ir_text = module.to_ir_string();

    let text = encode(&ir_text).map_err(|error| {
        UniversalIrExportError::Encoder {
            format_id: format_id.to_owned(),
            message: error.to_string(),
        }
    })?;

    let size_bytes = text.len();

    if let Some(limit_bytes) = options.max_output_bytes {
        if size_bytes > limit_bytes {
            return Err(UniversalIrExportError::OutputLimitExceeded {
                limit_bytes,
                written_bytes: 0,
                attempted_write_bytes: size_bytes,
            });
        }
    }

    let bytes = text.into_bytes();

    if size_bytes != bytes.len() {
        return Err(UniversalIrExportError::ArtifactInvariant {
            expected_bytes: size_bytes,
            actual_bytes: bytes.len(),
        });
    }

    Ok(UniversalIrArtifact {
        format_id: format_id.to_owned(),
        module_name: module.name.clone(),
        bytes,
        size_bytes,
    })
}

/// Validate an open-world interchange format identifier.
///
/// This validates identifier hygiene only. It does not assert that a format
/// exists, is registered, or is compatible with the input module.
fn validate_format_id(
    format_id: &str,
) -> Result<(), UniversalIrExportError> {
    let mut characters = format_id.chars();

    let Some(first) = characters.next() else {
        return Err(UniversalIrExportError::InvalidFormatId {
            format_id: format_id.to_owned(),
            reason: "identifier must not be empty",
        });
    };

    let Some(last) = format_id.chars().last() else {
        return Err(UniversalIrExportError::InvalidFormatId {
            format_id: format_id.to_owned(),
            reason: "identifier must not be empty",
        });
    };

    let is_alphanumeric = |character: char| character.is_ascii_alphanumeric();
    let is_allowed = |character: char| {
        is_alphanumeric(character)
            || matches!(character, '.' | '_' | '-' | '/' | ':')
    };

    if !is_alphanumeric(first)
        || !is_alphanumeric(last)
        || !format_id.chars().all(is_allowed)
    {
        return Err(UniversalIrExportError::InvalidFormatId {
            format_id: format_id.to_owned(),
            reason:
                "use ASCII letters, digits, '.', '_', '-', '/', or ':'; \
                 the first and last characters must be alphanumeric",
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{
        export_text, export_to, UniversalIrExportError,
        UniversalIrExportOptions,
    };
    use crate::ir_gen::IrModule;

    fn empty_module() -> IrModule {
        IrModule::new("interchange_test")
    }

    #[test]
    fn rejects_empty_format_identifier_before_encoding() {
        let module = empty_module();
        let mut encoder_called = false;

        let result = export_text(
            &module,
            "",
            UniversalIrExportOptions::unbounded(),
            |_| {
                encoder_called = true;
                Ok::<String, &'static str>("encoded".to_owned())
            },
        );

        assert!(matches!(
            result,
            Err(UniversalIrExportError::InvalidFormatId { .. })
        ));
        assert!(!encoder_called);
    }

    #[test]
    fn exports_through_the_selected_text_encoder() {
        let module = empty_module();

        let artifact = export_text(
            &module,
            "example.text-ir",
            UniversalIrExportOptions::unbounded(),
            |ir| Ok::<String, &'static str>(format!("format-header\n{ir}")),
        )
        .expect("the test module should pass structural verification");

        assert_eq!(artifact.format_id, "example.text-ir");
        assert_eq!(artifact.module_name, "interchange_test");
        assert_eq!(artifact.size_bytes, artifact.bytes.len());
        assert!(artifact.bytes.starts_with(b"format-header\n"));
    }

    #[test]
    fn text_export_rejects_artifacts_over_the_configured_budget() {
        let module = empty_module();

        let result = export_text(
            &module,
            "example.text-ir",
            UniversalIrExportOptions::with_max_output_bytes(3),
            |_| Ok::<String, &'static str>("four".to_owned()),
        );

        assert!(matches!(
            result,
            Err(UniversalIrExportError::OutputLimitExceeded {
                limit_bytes: 3,
                ..
            })
        ));
    }

    #[test]
    fn streaming_export_enforces_the_budget_during_writes() {
        let module = empty_module();

        let result = export_to(
            &module,
            "example.stream-ir",
            UniversalIrExportOptions::with_max_output_bytes(3),
            |_ir, writer| {
                writer.write_all(b"four")?;
                Ok::<(), std::io::Error>(())
            },
        );

        assert!(matches!(
            result,
            Err(UniversalIrExportError::OutputLimitExceeded {
                limit_bytes: 3,
                ..
            })
        ));
    }

    #[test]
    fn streaming_export_returns_complete_artifact_on_success() {
        let module = empty_module();

        let artifact = export_to(
            &module,
            "example.stream-ir",
            UniversalIrExportOptions::with_max_output_bytes(64),
            |_ir, writer| {
                writer.write_all(b"complete")?;
                Ok::<(), std::io::Error>(())
            },
        )
        .expect("the output fits within the configured budget");

        assert_eq!(artifact.bytes, b"complete");
        assert_eq!(artifact.size_bytes, 8);
    }

    #[test]
    fn encoder_failures_are_not_reported_as_success() {
        let module = empty_module();

        let result = export_text(
            &module,
            "example.text-ir",
            UniversalIrExportOptions::unbounded(),
            |_| Err::<String, _>("unsupported operation"),
        );

        assert!(matches!(
            result,
            Err(UniversalIrExportError::Encoder { .. })
        ));
    }
}
