
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani HDLIR Interchange
//!
//! Path: `src/hdl/hdlir/interchange/mod.rs`
//! Edition: Rust 2021.
//! Minimum supported Rust version: 1.97.
//! Safety: Safe Rust only.
//! Status: Interchange foundation; stabilize after adapter integration tests.
//!
//! ## Purpose
//!
//! Define the shared contracts for importing and exporting hardware designs
//! through explicitly supported interchange formats.
//!
//! This module is the interchange composition boundary. Concrete format
//! parsers, serializers, and conversion implementations belong in dedicated
//! child modules when their contracts and dependencies have been verified.
//!
//! ## Authority
//!
//! - `grammar/DESIGN.md`
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4`
//! - `grammar/Zamani.g4`
//! - `grammar/antlr/ZamaniLexer.g4`
//! - `grammar/lexer/tokens.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/validation/semantic-boundaries.md`
//! - `src/hdl/api.rs`
//! - `src/hdl/ids.rs`
//! - `src/frontend/semantic/domains/hdl.rs`
//! - `src/zuir/ZUIR-SPEC.md`
//!
//! The specifications define meaning and ownership. The frontend owns source
//! parsing and semantic validation. ZUIR owns universal computational
//! representation. HDLIR owns the hardware-domain representation.
//! Interchange adapters must respect those boundaries.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Interchange format identity and capability metadata.
//! - Explicit import/export contexts and operational budgets.
//! - Shared adapter interfaces.
//! - Structured interchange errors and conversion reports.
//! - Lossiness declarations and input-size preflight checks.
//!
//! It does not own:
//!
//! - Source grammar, lexing, parsing, AST construction, or name resolution.
//! - A competing HDL semantic model, source-span system, or universal IR.
//! - The HDLIR design graph or its verification rules.
//! - External-format implementations that have not been implemented.
//! - Synthesis, scheduling, allocation, placement, routing, or deployment.
//! - Hardware discovery, vendor runtime handles, or physical device identity.
//!
//! ## Scalability
//!
//! This module imposes no universal hardware, signal, port, width, module,
//! memory, or operation limits.
//!
//! Optional input/output budgets are operational safeguards, not language
//! semantics. A missing budget means that this layer imposes no additional
//! byte limit; it does not promise that the host can process arbitrary input.
//! Adapters must still defend against malformed input, excessive nesting,
//! expensive expansion, and resource exhaustion using their own explicit
//! operational controls.
//!
//! All size conversions and arithmetic must be checked. An adapter must
//! report incomplete or failed work explicitly and must never silently
//! truncate input, discard required semantics, or claim lossless conversion
//! when it cannot establish losslessness.
//!
//! ## Integration
//!
//! Implementations should follow this dependency direction:
//!
//! ```text
//! external HDL artifact
//!         |
//!         v
//! interchange adapter
//!         |
//!         v
//! canonical frontend / validated semantic input
//!         |
//!         v
//!       HDLIR
//!         |
//!         v
//! interchange adapter
//!         |
//!         v
//! external HDL artifact
//! ```
//!
//! The actual conversion path depends on the format and the implemented
//! frontend and HDLIR APIs. The diagram does not claim that these adapters
//! already exist.
//!
//! Importing source text must use the appropriate canonical frontend when
//! the format represents Zamani source. External HDL formats must use a
//! format-specific parser or a verified existing parser. They must not
//! bypass required semantic validation.
//!
//! Export must operate on verified HDLIR or another explicitly supported
//! input contract. It must report unsupported constructs and semantic loss.
//!
//! ## Freeze criteria
//!
//! - Public contracts compile with the repository's supported Rust toolchain.
//! - Every implemented adapter declares its actual format and supported
//!   capabilities.
//! - Malformed inputs and operational-budget exhaustion produce structured
//!   errors rather than false success.
//! - Lossy conversion is explicit and subject to caller policy.
//! - Round-trip, determinism, compatibility, and integration tests exist for
//!   every format that is advertised as supported.

use core::fmt;
use core::ops::Range;

/// Stable identifier for an interchange format.
///
/// Identifiers are lowercase, ASCII, and restricted to a portable alphabet.
/// Examples include `verilog`, `systemverilog`, and `vhdl`.
///
/// A syntactically valid identifier does not imply that its format is
/// implemented or registered.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct FormatId(String);

impl FormatId {
    /// Create a format identifier.
    ///
    /// Identifiers must start with an ASCII lowercase letter or digit.
    /// Remaining characters may additionally include `.`, `_`, `+`, and `-`.
    ///
    /// # Errors
    ///
    /// Returns [`InterchangeErrorKind::InvalidFormatId`] for an invalid ID.
    pub fn new(value: impl Into<String>) -> Result<Self, InterchangeError> {
        let value = value.into();
        let mut chars = value.chars();

        let valid_first = chars
            .next()
            .is_some_and(|c| c.is_ascii_lowercase() || c.is_ascii_digit());

        let valid_rest = chars.all(|c| {
            c.is_ascii_lowercase()
                || c.is_ascii_digit()
                || matches!(c, '.' | '_' | '+' | '-')
        });

        if !valid_first || !valid_rest {
            return Err(InterchangeError::new(
                InterchangeErrorKind::InvalidFormatId,
                format!("invalid interchange format identifier: {value:?}"),
            ));
        }

        Ok(Self(value))
    }

    /// Return the canonical identifier string.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for FormatId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Direction supported by an interchange adapter.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum InterchangeDirection {
    /// Convert an external artifact into the requested internal representation.
    Import,
    /// Convert an internal representation into an external artifact.
    Export,
}

/// The semantic fidelity of a completed conversion.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ConversionFidelity {
    /// The adapter establishes that all required semantics were preserved.
    Lossless,
    /// The conversion intentionally or unavoidably loses semantic information.
    Lossy,
}

/// Capability metadata declared by a concrete format adapter.
///
/// These flags describe implemented adapter behavior, not the complete
/// capabilities of the external standard.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FormatCapabilities {
    /// The adapter implements import for this format.
    pub import: bool,
    /// The adapter implements export for this format.
    pub export: bool,
    /// The adapter can preserve unknown extension records without interpreting
    /// their semantics, where the external format permits this.
    pub preserves_unknown_extensions: bool,
    /// The adapter has a tested lossless round-trip path for its documented
    /// supported subset.
    ///
    /// This does not promise lossless round trips for every construct in the
    /// external standard.
    pub lossless_round_trip_for_supported_subset: bool,
}

/// Description of a concrete interchange adapter.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FormatDescriptor {
    /// Canonical format identifier.
    pub id: FormatId,
    /// Human-readable format name.
    pub display_name: String,
    /// Version of the external format contract implemented by the adapter.
    ///
    /// This is not the HDLIR schema version or Zamani language version.
    pub format_version: String,
    /// Declared MIME types, when applicable.
    pub media_types: Vec<String>,
    /// Implemented adapter capabilities.
    pub capabilities: FormatCapabilities,
}

impl FormatDescriptor {
    /// Construct a descriptor after validating its required metadata.
    ///
    /// Format versions are opaque format-specific strings. This constructor
    /// does not claim conformance to an external standard.
    ///
    /// # Errors
    ///
    /// Returns an error if a required metadata field is empty or contains
    /// invalid format identity.
    pub fn new(
        id: FormatId,
        display_name: impl Into<String>,
        format_version: impl Into<String>,
        media_types: Vec<String>,
        capabilities: FormatCapabilities,
    ) -> Result<Self, InterchangeError> {
        let display_name = display_name.into();
        let format_version = format_version.into();

        if display_name.trim().is_empty() {
            return Err(InterchangeError::new(
                InterchangeErrorKind::InvalidFormatMetadata,
                "format display name must not be empty",
            ));
        }

        if format_version.trim().is_empty() {
            return Err(InterchangeError::new(
                InterchangeErrorKind::InvalidFormatMetadata,
                "format version must not be empty",
            ));
        }

        if media_types.iter().any(|value| value.trim().is_empty()) {
            return Err(InterchangeError::new(
                InterchangeErrorKind::InvalidFormatMetadata,
                "media types must not contain empty entries",
            ));
        }

        Ok(Self {
            id,
            display_name,
            format_version,
            media_types,
            capabilities,
        })
    }
}

/// Optional operational limits for interchange processing.
///
/// `None` means this contract imposes no limit for that dimension. Concrete
/// adapters must document and enforce any additional parser, nesting,
/// expansion, or work budgets they require.
///
/// These are operational limits, never universal HDL language limits.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct InterchangeBudget {
    /// Maximum accepted input bytes, if configured.
    pub max_input_bytes: Option<u64>,
    /// Maximum permitted output bytes, if configured.
    pub max_output_bytes: Option<u64>,
    /// Maximum number of adapter notices, if configured.
    ///
    /// If this budget is reached, adapters must report omitted notices or
    /// return a budget error. They must not silently claim a complete report.
    pub max_notices: Option<usize>,
}

impl InterchangeBudget {
    /// Check the size of an already available input buffer.
    ///
    /// This check cannot undo an allocation already performed by the caller.
    /// Streaming adapters should enforce the same budget before accepting
    /// additional bytes.
    ///
    /// # Errors
    ///
    /// Returns [`InterchangeErrorKind::InputBudgetExceeded`] when the
    /// configured input budget is exceeded.
    pub fn check_input_size(self, actual: usize) -> Result<(), InterchangeError> {
        let actual = u64::try_from(actual).map_err(|_| {
            InterchangeError::new(
                InterchangeErrorKind::SizeNotRepresentable,
                "input length cannot be represented by the interchange size type",
            )
        })?;

        if self.max_input_bytes.is_some_and(|limit| actual > limit) {
            return Err(InterchangeError::new(
                InterchangeErrorKind::InputBudgetExceeded,
                format!(
                    "input contains {actual} bytes, exceeding the configured budget of {} bytes",
                    self.max_input_bytes.unwrap_or_default()
                ),
            ));
        }

        Ok(())
    }

    /// Check the size of a proposed output buffer.
    ///
    /// An adapter should use this before committing the completed artifact
    /// to a caller or persistent destination.
    ///
    /// # Errors
    ///
    /// Returns [`InterchangeErrorKind::OutputBudgetExceeded`] when the
    /// configured output budget is exceeded.
    pub fn check_output_size(self, actual: usize) -> Result<(), InterchangeError> {
        let actual = u64::try_from(actual).map_err(|_| {
            InterchangeError::new(
                InterchangeErrorKind::SizeNotRepresentable,
                "output length cannot be represented by the interchange size type",
            )
        })?;

        if self.max_output_bytes.is_some_and(|limit| actual > limit) {
            return Err(InterchangeError::new(
                InterchangeErrorKind::OutputBudgetExceeded,
                format!(
                    "output contains {actual} bytes, exceeding the configured budget of {} bytes",
                    self.max_output_bytes.unwrap_or_default()
                ),
            ));
        }

        Ok(())
    }
}

/// Policy for handling conversions that cannot preserve every semantic detail.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum LossyConversionPolicy {
    /// Reject conversions that cannot establish losslessness.
    #[default]
    Reject,
    /// Permit lossy conversion, but require it to be reported.
    AllowWithReport,
}

/// Context shared by import adapters.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ImportContext {
    /// Operational limits for this import operation.
    pub budget: InterchangeBudget,
    /// Whether the caller permits a conversion that loses semantics.
    pub lossy_policy: LossyConversionPolicy,
}

/// Context shared by export adapters.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ExportContext {
    /// Operational limits for this export operation.
    pub budget: InterchangeBudget,
    /// Whether the caller permits a conversion that loses semantics.
    pub lossy_policy: LossyConversionPolicy,
}

/// Classification of a failed interchange operation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum InterchangeErrorKind {
    /// A format identifier is syntactically invalid.
    InvalidFormatId,
    /// Required descriptor metadata is invalid.
    InvalidFormatMetadata,
    /// The requested direction is not implemented by the adapter.
    UnsupportedDirection,
    /// Input is malformed according to the implemented format contract.
    MalformedInput,
    /// The artifact requests a feature that the adapter cannot interpret.
    UnsupportedFeature,
    /// A conversion would lose required semantics.
    LossyConversionRejected,
    /// The input exceeds its configured operational budget.
    InputBudgetExceeded,
    /// The output exceeds its configured operational budget.
    OutputBudgetExceeded,
    /// A size cannot be represented by the interchange size type.
    SizeNotRepresentable,
    /// The internal representation is invalid for this adapter.
    InvalidInternalRepresentation,
    /// An external format version is unsupported.
    UnsupportedFormatVersion,
    /// An external extension or schema conflicts with the supported contract.
    IncompatibleExtension,
    /// An adapter could not complete the requested operation.
    ConversionFailed,
}

/// Structured failure from an interchange operation.
///
/// The message is intended for diagnostics and is not a stable machine
/// interface. Consumers should branch on `kind`, not parse this message.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InterchangeError {
    /// Stable error classification.
    pub kind: InterchangeErrorKind,
    /// Optional format associated with the failure.
    pub format: Option<FormatId>,
    /// Human-readable explanation.
    pub message: String,
    /// Optional byte range in the external artifact.
    ///
    /// The range is half-open. Its offsets refer to the artifact byte stream,
    /// not necessarily to source-language character positions.
    pub artifact_range: Option<Range<u64>>,
}

impl InterchangeError {
    /// Construct an interchange error without format-specific context.
    #[must_use]
    pub fn new(kind: InterchangeErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            format: None,
            message: message.into(),
            artifact_range: None,
        }
    }

    /// Attach the external format associated with this error.
    #[must_use]
    pub fn with_format(mut self, format: FormatId) -> Self {
        self.format = Some(format);
        self
    }

    /// Attach a validated half-open artifact byte range.
    ///
    /// # Errors
    ///
    /// Returns an error if the range end precedes its start.
    pub fn with_artifact_range(
        mut self,
        range: Range<u64>,
    ) -> Result<Self, InterchangeError> {
        if range.end < range.start {
            return Err(Self::new(
                InterchangeErrorKind::ConversionFailed,
                "artifact range end precedes its start",
            ));
        }

        self.artifact_range = Some(range);
        Ok(self)
    }
}

impl fmt::Display for InterchangeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(format) = &self.format {
            write!(f, "{format}: ")?;
        }

        write!(f, "{:?}: {}", self.kind, self.message)?;

        if let Some(range) = &self.artifact_range {
            write!(f, " (artifact bytes {}..{})", range.start, range.end)?;
        }

        Ok(())
    }
}

impl std::error::Error for InterchangeError {}

/// A non-fatal notice emitted by a concrete adapter.
///
/// This is limited to interchange-specific reporting. Source-language
/// diagnostics and canonical semantic diagnostics remain owned by the
/// frontend and semantic subsystem.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InterchangeNotice {
    /// Stable adapter-defined notice code.
    pub code: String,
    /// Human-readable notice.
    pub message: String,
    /// Optional byte range in the external artifact.
    pub artifact_range: Option<Range<u64>>,
}

/// Result of importing an external artifact.
///
/// An adapter must not return a design as successfully imported if required
/// parsing, semantic validation, or conversion work remains incomplete.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImportReport<T> {
    /// Imported internal representation.
    pub value: T,
    /// Fidelity established by the adapter.
    pub fidelity: ConversionFidelity,
    /// Interchange-specific notices.
    pub notices: Vec<InterchangeNotice>,
}

/// Result of exporting an internal representation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExportReport {
    /// Complete external artifact bytes.
    pub bytes: Vec<u8>,
    /// Fidelity established by the adapter.
    pub fidelity: ConversionFidelity,
    /// Interchange-specific notices.
    pub notices: Vec<InterchangeNotice>,
}

/// Contract implemented by a concrete external-format importer.
///
/// `Design` must be the actual verified destination representation selected
/// by the integration layer. This trait deliberately does not invent an
/// HDLIR design type before the core IR API has been implemented.
pub trait HdlImporter<Design> {
    /// Return metadata for the external format implemented by this adapter.
    fn descriptor(&self) -> &FormatDescriptor;

    /// Import one complete artifact into the selected destination type.
    ///
    /// Implementations must enforce their parser-specific budgets, reject
    /// malformed input, preserve provenance where supported, and report
    /// semantic loss explicitly.
    ///
    /// # Errors
    ///
    /// Returns a structured error for unsupported directions, malformed
    /// input, unsupported features, budget exhaustion, invalid conversion,
    /// or rejected lossy conversion.
    fn import(
        &self,
        input: &[u8],
        context: &ImportContext,
    ) -> Result<ImportReport<Design>, InterchangeError>;
}

/// Contract implemented by a concrete external-format exporter.
///
/// `Design` must be the actual verified source representation selected by
/// the integration layer. An exporter must not claim that emitted output is
/// valid for every construct in an external standard unless it implements
/// and tests that complete scope.
pub trait HdlExporter<Design> {
    /// Return metadata for the external format implemented by this adapter.
    fn descriptor(&self) -> &FormatDescriptor;

    /// Export one internal design to a complete external artifact.
    ///
    /// Implementations must reject unsupported semantics unless the selected
    /// lossy-conversion policy permits them, and must enforce output budgets.
    ///
    /// # Errors
    ///
    /// Returns a structured error for unsupported directions, invalid
    /// internal representations, unsupported features, budget exhaustion,
    /// conversion failures, or rejected lossy conversion.
    fn export(
        &self,
        design: &Design,
        context: &ExportContext,
    ) -> Result<ExportReport, InterchangeError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn format_id(value: &str) -> FormatId {
        FormatId::new(value).expect("test format ID must be valid")
    }

    #[test]
    fn accepts_portable_format_identifiers() {
        for value in [
            "verilog",
            "systemverilog",
            "vhdl",
            "zamani.hdl",
            "vendor+extension",
            "format_v2",
            "hdl-2026",
            "3gpp.hdl",
        ] {
            assert_eq!(format_id(value).as_str(), value);
        }
    }

    #[test]
    fn rejects_invalid_format_identifiers() {
        for value in [
            "",
            "Verilog",
            "VHDL",
            "has space",
            "../format",
            "format/name",
            "-format",
            "_format",
            "format:",
        ] {
            assert_eq!(
                FormatId::new(value).expect_err("identifier must be rejected").kind,
                InterchangeErrorKind::InvalidFormatId,
                "unexpected result for {value:?}",
            );
        }
    }

    #[test]
    fn descriptor_rejects_empty_required_metadata() {
        let id = format_id("example");

        assert_eq!(
            FormatDescriptor::new(
                id.clone(),
                "  ",
                "1",
                Vec::new(),
                FormatCapabilities::default(),
            )
            .expect_err("empty display name must fail")
            .kind,
            InterchangeErrorKind::InvalidFormatMetadata,
        );

        assert_eq!(
            FormatDescriptor::new(
                id.clone(),
                "Example",
                "",
                Vec::new(),
                FormatCapabilities::default(),
            )
            .expect_err("empty version must fail")
            .kind,
            InterchangeErrorKind::InvalidFormatMetadata,
        );

        assert_eq!(
            FormatDescriptor::new(
                id,
                "Example",
                "1",
                vec!["".to_owned()],
                FormatCapabilities::default(),
            )
            .expect_err("empty media type must fail")
            .kind,
            InterchangeErrorKind::InvalidFormatMetadata,
        );
    }

    #[test]
    fn input_budget_accepts_boundary_and_rejects_excess() {
        let budget = InterchangeBudget {
            max_input_bytes: Some(4),
            ..InterchangeBudget::default()
        };

        assert!(budget.check_input_size(4).is_ok());

        assert_eq!(
            budget
                .check_input_size(5)
                .expect_err("input above the limit must fail")
                .kind,
            InterchangeErrorKind::InputBudgetExceeded,
        );
    }

    #[test]
    fn output_budget_accepts_boundary_and_rejects_excess() {
        let budget = InterchangeBudget {
            max_output_bytes: Some(4),
            ..InterchangeBudget::default()
        };

        assert!(budget.check_output_size(4).is_ok());

        assert_eq!(
            budget
                .check_output_size(5)
                .expect_err("output above the limit must fail")
                .kind,
            InterchangeErrorKind::OutputBudgetExceeded,
        );
    }

    #[test]
    fn absent_byte_budgets_do_not_impose_a_fixed_limit() {
        let budget = InterchangeBudget::default();

        assert!(budget.check_input_size(usize::MAX).is_ok());
        assert!(budget.check_output_size(usize::MAX).is_ok());
    }

    #[test]
    fn error_context_preserves_format_and_artifact_range() {
        let error = InterchangeError::new(
            InterchangeErrorKind::MalformedInput,
            "invalid external construct",
        )
        .with_format(format_id("example"))
        .with_artifact_range(12..19)
        .expect("valid range");

        assert_eq!(error.format.as_ref().map(FormatId::as_str), Some("example"));
        assert_eq!(error.artifact_range, Some(12..19));
        assert!(error.to_string().contains("example"));
        assert!(error.to_string().contains("12..19"));
    }

    #[test]
    fn error_rejects_reversed_artifact_range() {
        let error = InterchangeError::new(
            InterchangeErrorKind::MalformedInput,
            "invalid range",
        )
        .with_artifact_range(10..5)
        .expect_err("reversed range must fail");

        assert_eq!(error.kind, InterchangeErrorKind::ConversionFailed);
    }

    #[test]
    fn default_policy_rejects_lossy_conversion() {
        assert_eq!(
            LossyConversionPolicy::default(),
            LossyConversionPolicy::Reject,
        );
    }

    #[test]
    fn capability_metadata_does_not_claim_unimplemented_formats() {
        let capabilities = FormatCapabilities::default();

        assert!(!capabilities.import);
        assert!(!capabilities.export);
        assert!(!capabilities.preserves_unknown_extensions);
        assert!(!capabilities.lossless_round_trip_for_supported_subset);
    }
}
