
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Compiler — IR Verification Integration
//!
//! Path: `src/compiler/ir/verification.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97 or later.
//! Safety: Unsafe Rust is forbidden.
//!
//! ## Purpose
//!
//! Provide the compiler-facing verification boundary for canonical
//! intermediate representations.
//!
//! This module coordinates authoritative verification implementations.
//! It does not define another IR or implement another verifier.
//!
//! The currently integrated full-program verification path is canonical
//! ZUIR through `crate::zuir::verify::verify_full`.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Compiler-facing ZUIR verification entry points.
//! - Compiler pipeline verification-point identification.
//! - Preservation of canonical verification reports.
//! - Strict acceptance and rejection of verification outcomes.
//! - Structured compiler-facing verification errors.
//!
//! This module does not own:
//!
//! - Source grammar, parsing, AST construction, or semantic analysis.
//! - Canonical IR structures, IDs, or types.
//! - Structural, type, control-flow, effect, ownership, or resource
//!   verification algorithms.
//! - Quantum, HDL, AI, hybrid, or distributed verification algorithms.
//! - Target discovery, physical allocation, scheduling, or routing.
//! - Backend code generation or runtime execution.
//!
//! ## Canonical authorities
//!
//! Universal IR:
//!     crate::zuir
//!
//! Full ZUIR verification:
//!     crate::zuir::verify::verify_full
//!
//! Classical IR:
//!     crate::classical::zcir::verify
//!
//! Quantum IR:
//!     crate::quantum::ir::validation
//!
//! HDLIR:
//!     crate::hdl::hdlir::verify
//!
//! Hybrid computation:
//!     crate::hybrid
//!
//! AI computation:
//!     crate::ai
//!
//! Distributed computation:
//!     crate::distributed
//!
//! The existence of a domain verifier does not imply that this module
//! already integrates it. Domain adapters must be added only after their
//! actual input, report, error, and completeness contracts are established.
//!
//! ## Trust boundary
//!
//! Verification applies to the exact IR snapshot described by the
//! supplied verification input.
//!
//! A previous report does not automatically verify:
//!
//! - A transformed IR.
//! - A deserialized IR.
//! - A cached IR.
//! - An externally supplied IR.
//! - A translated destination representation.
//!
//! A transformation that changes the representation requires fresh
//! verification before the changed representation crosses a boundary
//! that requires verified input.
//!
//! ## Verification status
//!
//! The canonical verifier distinguishes:
//!
//! Valid:
//!     All currently integrated checks passed.
//!
//! Invalid:
//!     At least one known invariant was violated.
//!
//! Incomplete:
//!     At least one required property could not be established.
//!
//! Only Valid is accepted by strict verification admission.
//!
//! Invalid and Incomplete must never be converted into success.
//!
//! ## Completeness boundary
//!
//! A Valid ZUIR report establishes only the properties checked by the
//! canonical full verifier.
//!
//! It does not automatically establish:
//!
//! - Complete SSA use-def legality.
//! - Target compatibility.
//! - Backend legality.
//! - Physical resource feasibility.
//! - Quantum topology feasibility.
//! - HDL synthesis correctness.
//! - Hybrid execution correctness.
//! - Distributed consistency.
//! - Successful linking or execution.
//!
//! Those properties require their respective authoritative checks.
//!
//! ## Scalability
//!
//! This module imposes no arbitrary universal ceilings on:
//!
//! - Programs, modules, functions, or regions.
//! - Blocks, operations, values, or types.
//! - Effects or ownership relationships.
//! - Qubits, signals, registers, or tensor dimensions.
//! - Tensor rank.
//! - Devices, processors, accelerators, or participants.
//! - Computing domains.
//!
//! The canonical verifier determines the actual work.
//!
//! Memory exhaustion, arithmetic failure, operational-budget exhaustion,
//! or unavailable verification context must never be interpreted as
//! successful verification.
//!
//! No finite implementation can promise literal infinite computation.
//!
//! ## Determinism
//!
//! This module introduces:
//!
//! - No global mutable state.
//! - No random identifiers.
//! - No hidden target selection.
//! - No hardware discovery.
//! - No filesystem or network access.
//! - No mutation of supplied IR.
//!
//! Canonical verification ordering and diagnostic ordering remain owned
//! by the authoritative verifier.
//!
//! ## Grammar boundary
//!
//! This module does not parse source text or define language semantics.
//!
//! The relevant authority chain is:
//!
//!     grammar/Zamani.g4
//!         |
//!         v
//!     frontend
//!         |
//!         v
//!     semantic model
//!         |
//!         v
//!     canonical IR
//!         |
//!         v
//!     compiler IR verification
//!
//! Relevant specifications:
//!
//! - grammar/Zamani.g4
//! - grammar/lexer/tokens.g4
//! - grammar/specification/semantic-model.md
//! - grammar/specification/compilation-model.md
//! - grammar/specification/portability-model.md
//! - grammar/specification/poco-reaf.md
//! - grammar/validation/semantic-boundaries.md
//!
//! ## Integration
//!
//! Parent:
//!     crate::compiler::ir
//!
//! Initial input:
//!     crate::compiler::ir::input
//!
//! Canonical verifier:
//!     crate::zuir::verify
//!
//! Downstream consumers:
//!     compiler analysis
//!     compiler pass management
//!     compiler optimization
//!     compiler lowering
//!     compiler planning
//!     compiler backend integration
//!
//! ## Freeze criteria
//!
//! - Canonical ZUIR verification is delegated, not duplicated.
//! - Invalid IR cannot pass strict admission.
//! - Incomplete verification cannot pass strict admission.
//! - Canonical reports and findings are preserved.
//! - Reverification after transformations is supported.
//! - Domain-specific verification is not falsely claimed.
//! - No arbitrary universal hardware ceiling is introduced.
//! - No unsafe code is introduced.
//! - Repository tests and quality gates pass.

use core::fmt;
use std::error::Error;

use crate::zuir::verify::{
    verify_full,
    FullVerificationInput,
    FullVerificationReport,
    VerificationSeverity,
    VerificationStage,
    VerificationStatus,
};

// =============================================================================
// API VERSION
// =============================================================================

/// Version of the compiler IR verification integration API.
///
/// This version is independent of the Zamani language version,
/// grammar version, semantic-model version, ZUIR version,
/// domain IR versions, and backend ABI versions.
pub const IR_VERIFICATION_API_VERSION: u16 = 2;

// =============================================================================
// VERIFICATION POINT
// =============================================================================

/// Identifies the compiler pipeline point requesting verification.
///
/// This value describes the verification request. It does not alter
/// canonical verification algorithms or their semantic meaning.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum VerificationPoint {
    /// Verify IR entering the compiler IR boundary.
    CompilationInput,

    /// Verify IR after an optimization or transformation.
    AfterTransformation,

    /// Verify IR before conversion or lowering.
    BeforeLowering,

    /// Verify IR before backend input construction.
    BeforeBackend,
}

// =============================================================================
// VERIFICATION ERROR CLASSIFICATION
// =============================================================================

/// Classification of a failed strict verification request.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum IrVerificationErrorKind {
    /// The canonical verifier established that the IR is invalid.
    InvalidIr,

    /// The canonical verifier could not establish all required
    /// verification properties.
    IncompleteVerification,
}

// =============================================================================
// VERIFICATION ERROR
// =============================================================================

/// Structured error produced when strict IR verification fails.
///
/// The original canonical report is retained without replacing,
/// flattening, or discarding its findings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IrVerificationError {
    point: VerificationPoint,
    kind: IrVerificationErrorKind,
    report: FullVerificationReport,
}

impl IrVerificationError {
    /// Construct a strict verification error from a non-valid report.
    ///
    /// This constructor is private to prevent inconsistent error states.
    fn from_report(
        point: VerificationPoint,
        report: FullVerificationReport,
    ) -> Result<(), Self> {
        let kind = match report.status() {
            VerificationStatus::Valid => return Ok(()),

            VerificationStatus::Invalid => {
                IrVerificationErrorKind::InvalidIr
            }

            VerificationStatus::Incomplete => {
                IrVerificationErrorKind::IncompleteVerification
            }
        };

        Err(Self {
            point,
            kind,
            report,
        })
    }

    /// Return the compiler pipeline point that requested verification.
    #[must_use]
    pub const fn point(&self) -> VerificationPoint {
        self.point
    }

    /// Return the failure classification.
    #[must_use]
    pub const fn kind(&self) -> IrVerificationErrorKind {
        self.kind
    }

    /// Borrow the complete canonical verification report.
    #[must_use]
    pub fn report(&self) -> &FullVerificationReport {
        &self.report
    }

    /// Consume the error and recover the canonical report.
    #[must_use]
    pub fn into_report(self) -> FullVerificationReport {
        self.report
    }

    /// Return the number of error findings.
    #[must_use]
    pub fn error_count(&self) -> usize {
        self.report.count(VerificationSeverity::Error)
    }

    /// Return the number of deferred findings.
    #[must_use]
    pub fn deferred_count(&self) -> usize {
        self.report.count(VerificationSeverity::Deferred)
    }

    /// Return whether the failure represents invalid IR.
    #[must_use]
    pub const fn is_invalid(&self) -> bool {
        matches!(
            self.kind,
            IrVerificationErrorKind::InvalidIr
        )
    }

    /// Return whether verification was incomplete.
    #[must_use]
    pub const fn is_incomplete(&self) -> bool {
        matches!(
            self.kind,
            IrVerificationErrorKind::IncompleteVerification
        )
    }
}

impl fmt::Display for IrVerificationError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self.kind {
            IrVerificationErrorKind::InvalidIr => {
                write!(
                    formatter,
                    "IR verification failed at {:?}: {} error finding(s)",
                    self.point,
                    self.error_count(),
                )
            }

            IrVerificationErrorKind::IncompleteVerification => {
                write!(
                    formatter,
                    "IR verification incomplete at {:?}: {} deferred finding(s)",
                    self.point,
                    self.deferred_count(),
                )
            }
        }
    }
}

impl Error for IrVerificationError {}

// =============================================================================
// VERIFICATION OUTCOME
// =============================================================================

/// Compiler-facing outcome of one canonical ZUIR verification run.
///
/// The outcome preserves the original canonical report.
///
/// An outcome is not automatically accepted. Call [`Self::into_result`]
/// when the caller requires successful verification before proceeding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IrVerificationOutcome {
    point: VerificationPoint,
    report: FullVerificationReport,
}

impl IrVerificationOutcome {
    /// Construct an outcome from a canonical verification report.
    ///
    /// This constructor is private. Production outcomes must originate
    /// from the canonical verification entry point.
    fn from_report(
        point: VerificationPoint,
        report: FullVerificationReport,
    ) -> Self {
        Self {
            point,
            report,
        }
    }

    /// Return the pipeline point associated with this verification.
    #[must_use]
    pub const fn point(&self) -> VerificationPoint {
        self.point
    }

    /// Borrow the original canonical verification report.
    #[must_use]
    pub fn report(&self) -> &FullVerificationReport {
        &self.report
    }

    /// Consume the outcome and return its canonical report.
    #[must_use]
    pub fn into_report(self) -> FullVerificationReport {
        self.report
    }

    /// Return the canonical verification status.
    #[must_use]
    pub fn status(&self) -> VerificationStatus {
        self.report.status()
    }

    /// Return true only when canonical verification succeeded.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.status() == VerificationStatus::Valid
    }

    /// Return true when canonical verification found invalid IR.
    #[must_use]
    pub fn is_invalid(&self) -> bool {
        self.status() == VerificationStatus::Invalid
    }

    /// Return true when canonical verification was incomplete.
    #[must_use]
    pub fn is_incomplete(&self) -> bool {
        self.status() == VerificationStatus::Incomplete
    }

    /// Return true when the report contains error findings.
    #[must_use]
    pub fn has_errors(&self) -> bool {
        self.report.has_errors()
    }

    /// Return true when the report contains deferred findings.
    #[must_use]
    pub fn has_deferred_findings(&self) -> bool {
        self.report.is_incomplete()
    }

    /// Return the total number of canonical findings.
    #[must_use]
    pub fn finding_count(&self) -> usize {
        self.report.issues().len()
    }

    /// Return the number of findings at the requested severity.
    #[must_use]
    pub fn count(
        &self,
        severity: VerificationSeverity,
    ) -> usize {
        self.report.count(severity)
    }

    /// Borrow the verification stages completed by the canonical verifier.
    #[must_use]
    pub fn completed_stages(&self) -> &[VerificationStage] {
        self.report.stages_completed()
    }

    /// Return the compiler IR verification API version.
    #[must_use]
    pub const fn api_version(&self) -> u16 {
        IR_VERIFICATION_API_VERSION
    }

    /// Require successful verification before proceeding.
    ///
    /// Invalid and incomplete reports are returned as structured errors.
    /// The complete canonical report is preserved in the error.
    ///
    /// A successful result proves only the properties covered by the
    /// canonical verifier. It does not prove target feasibility,
    /// backend compatibility, or execution success.
    pub fn into_result(
        self,
    ) -> Result<Self, IrVerificationError> {
        match IrVerificationError::from_report(
            self.point,
            self.report.clone(),
        ) {
            Ok(()) => Ok(self),
            Err(error) => Err(error),
        }
    }
}

// =============================================================================
// CANONICAL ZUIR VERIFICATION
// =============================================================================

/// Verify canonical ZUIR at a specified compiler pipeline point.
///
/// This function delegates the entire verification operation to
/// `crate::zuir::verify::verify_full`.
///
/// Invalid and incomplete reports are preserved in the returned outcome.
/// Use [`verify_zuir_required`] when successful verification is mandatory.
///
/// # Snapshot requirement
///
/// All references inside `FullVerificationInput` must describe one
/// consistent IR snapshot.
///
/// The caller must not mutate the supplied data while verification runs.
/// The borrowed input contract does not protect against interior mutability
/// in caller-provided types.
///
/// # Reverification
///
/// Call this function again after a transformation that changes the IR.
/// Supply a new verification input describing the transformed snapshot.
///
/// # Completeness
///
/// A valid result establishes only the properties covered by the canonical
/// full ZUIR verifier. It is not a target or execution guarantee.
#[must_use = "IR verification outcomes must be inspected"]
pub fn verify_zuir(
    input: &FullVerificationInput<'_>,
    point: VerificationPoint,
) -> IrVerificationOutcome {
    let report = verify_full(input);

    IrVerificationOutcome::from_report(point, report)
}

/// Verify canonical ZUIR and require successful verification.
///
/// Unlike [`verify_zuir`], this function rejects both Invalid and
/// Incomplete outcomes.
///
/// The complete canonical report is retained in the error.
///
/// This is the preferred entry point for compiler stages that must not
/// process unverified IR.
pub fn verify_zuir_required(
    input: &FullVerificationInput<'_>,
    point: VerificationPoint,
) -> Result<IrVerificationOutcome, IrVerificationError> {
    verify_zuir(input, point).into_result()
}

// =============================================================================
// PIPELINE ENTRY POINTS
// =============================================================================

/// Verify ZUIR entering the compiler IR pipeline.
///
/// Invalid and incomplete reports remain inspectable in the outcome.
/// Use `verify_zuir_required` with `CompilationInput` when admission
/// requires successful verification.
#[must_use = "IR verification outcomes must be inspected"]
pub fn verify_compilation_input(
    input: &FullVerificationInput<'_>,
) -> IrVerificationOutcome {
    verify_zuir(
        input,
        VerificationPoint::CompilationInput,
    )
}

/// Verify ZUIR after an optimization or transformation.
///
/// The supplied input must describe the transformed representation,
/// not a stale pre-transformation snapshot.
#[must_use = "IR verification outcomes must be inspected"]
pub fn verify_after_transformation(
    input: &FullVerificationInput<'_>,
) -> IrVerificationOutcome {
    verify_zuir(
        input,
        VerificationPoint::AfterTransformation,
    )
}

/// Verify ZUIR before representation lowering.
///
/// This is target-independent IR verification. It does not replace
/// conversion legality or target feasibility checks.
#[must_use = "IR verification outcomes must be inspected"]
pub fn verify_before_lowering(
    input: &FullVerificationInput<'_>,
) -> IrVerificationOutcome {
    verify_zuir(
        input,
        VerificationPoint::BeforeLowering,
    )
}

/// Verify ZUIR before backend input construction.
///
/// Backend-specific compatibility, ABI validation, and artifact
/// verification remain separate responsibilities.
#[must_use = "IR verification outcomes must be inspected"]
pub fn verify_before_backend(
    input: &FullVerificationInput<'_>,
) -> IrVerificationOutcome {
    verify_zuir(
        input,
        VerificationPoint::BeforeBackend,
    )
}

// =============================================================================
// STRICT PIPELINE ENTRY POINTS
// =============================================================================

/// Verify compiler input and reject invalid or incomplete IR.
pub fn require_valid_compilation_input(
    input: &FullVerificationInput<'_>,
) -> Result<IrVerificationOutcome, IrVerificationError> {
    verify_zuir_required(
        input,
        VerificationPoint::CompilationInput,
    )
}

/// Verify transformed IR and reject invalid or incomplete IR.
pub fn require_valid_transformation(
    input: &FullVerificationInput<'_>,
) -> Result<IrVerificationOutcome, IrVerificationError> {
    verify_zuir_required(
        input,
        VerificationPoint::AfterTransformation,
    )
}

/// Verify IR before lowering and reject invalid or incomplete IR.
pub fn require_valid_before_lowering(
    input: &FullVerificationInput<'_>,
) -> Result<IrVerificationOutcome, IrVerificationError> {
    verify_zuir_required(
        input,
        VerificationPoint::BeforeLowering,
    )
}

/// Verify IR before backend construction and reject invalid or incomplete IR.
pub fn require_valid_before_backend(
    input: &FullVerificationInput<'_>,
) -> Result<IrVerificationOutcome, IrVerificationError> {
    verify_zuir_required(
        input,
        VerificationPoint::BeforeBackend,
    )
}

// =============================================================================
// API INFORMATION
// =============================================================================

/// Return the compiler IR verification integration API version.
#[must_use]
pub const fn api_version() -> u16 {
    IR_VERIFICATION_API_VERSION
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verification_api_version_is_available() {
        assert_eq!(
            api_version(),
            IR_VERIFICATION_API_VERSION
        );
        assert_ne!(IR_VERIFICATION_API_VERSION, 0);
    }

    #[test]
    fn verification_points_are_distinct() {
        assert_ne!(
            VerificationPoint::CompilationInput,
            VerificationPoint::AfterTransformation
        );

        assert_ne!(
            VerificationPoint::BeforeLowering,
            VerificationPoint::BeforeBackend
        );
    }

    #[test]
    fn strict_conversion_accepts_only_valid_reports() {
        let report = FullVerificationReport::default();

        let outcome = IrVerificationOutcome::from_report(
            VerificationPoint::CompilationInput,
            report,
        );

        assert!(outcome.is_valid());
        assert!(outcome.into_result().is_ok());
    }

    #[test]
    fn outcome_preserves_its_verification_point() {
        let outcome = IrVerificationOutcome::from_report(
            VerificationPoint::AfterTransformation,
            FullVerificationReport::default(),
        );

        assert_eq!(
            outcome.point(),
            VerificationPoint::AfterTransformation
        );
    }

    #[test]
    fn canonical_report_is_preserved() {
        let report = FullVerificationReport::default();

        let outcome = IrVerificationOutcome::from_report(
            VerificationPoint::BeforeLowering,
            report.clone(),
        );

        assert_eq!(outcome.report(), &report);
        assert_eq!(outcome.status(), report.status());
        assert_eq!(
            outcome.finding_count(),
            report.issues().len()
        );
        assert_eq!(
            outcome.completed_stages(),
            report.stages_completed()
        );
    }

    #[test]
    fn outcome_can_return_the_original_report() {
        let report = FullVerificationReport::default();

        let outcome = IrVerificationOutcome::from_report(
            VerificationPoint::BeforeBackend,
            report.clone(),
        );

        assert_eq!(outcome.into_report(), report);
    }

    #[test]
    fn verification_error_classifications_are_distinct() {
        assert_ne!(
            IrVerificationErrorKind::InvalidIr,
            IrVerificationErrorKind::IncompleteVerification
        );
    }

    #[test]
    fn error_display_is_structured() {
        let report = FullVerificationReport::default();

        let error = IrVerificationError {
            point: VerificationPoint::BeforeLowering,
            kind: IrVerificationErrorKind::InvalidIr,
            report,
        };

        assert!(error.to_string().contains("IR verification failed"));
        assert!(error.to_string().contains("BeforeLowering"));
        assert!(error.is_invalid());
        assert!(!error.is_incomplete());
    }

    #[test]
    fn error_preserves_the_canonical_report() {
        let report = FullVerificationReport::default();

        let error = IrVerificationError {
            point: VerificationPoint::BeforeBackend,
            kind: IrVerificationErrorKind::IncompleteVerification,
            report: report.clone(),
        };

        assert_eq!(error.report(), &report);
        assert_eq!(error.into_report(), report);
    }

    #[test]
    fn error_implements_standard_error_trait() {
        fn accepts_error<E: Error>() {}

        accepts_error::<IrVerificationError>();
    }
}
