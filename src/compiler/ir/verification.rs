
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Meta-Compiler — IR Verification Integration
//!
//! Path: `src/compiler/ir/verification.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97 or later
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Provide the compiler-facing verification boundary for canonical
//! intermediate representations.
//!
//! This module coordinates existing authoritative verification
//! implementations. It does not implement another IR verifier.
//!
//! Its initial supported representation is canonical ZUIR.
//!
//! The compiler can verify:
//!
//! - Initial IR supplied to the compiler.
//! - IR after an optimization or transformation.
//! - IR before subsequent lowering or backend processing.
//!
//! All ZUIR verification is delegated to:
//!
//!     crate::zuir::verify::verify_full
//!
//! The canonical verifier owns the verification algorithms,
//! verification stages, findings, and completeness status.
//!
//! ## Architectural ownership
//!
//! This module owns:
//!
//! - Compiler-facing verification entry points.
//! - Verification-point identification.
//! - Preservation of canonical verification reports.
//! - Compiler-facing acceptance decisions.
//! - The boundary between IR verification and later compiler stages.
//!
//! This module does not own:
//!
//! - IR structures or identifiers.
//! - Semantic analysis.
//! - Type inference.
//! - Ownership semantics.
//! - Structural verification algorithms.
//! - Control-flow verification algorithms.
//! - Dominance analysis.
//! - SSA verification algorithms.
//! - Effect verification algorithms.
//! - Resource verification algorithms.
//! - Dialect verification algorithms.
//! - Quantum IR validation algorithms.
//! - HDLIR verification algorithms.
//! - AI semantic validation.
//! - Hardware capability discovery.
//! - Target feasibility.
//! - Physical resource allocation.
//! - Scheduling or routing.
//! - Backend code generation.
//! - Runtime execution.
//!
//! These responsibilities remain with their authoritative subsystems.
//!
//! ## Verification authority
//!
//! Canonical universal IR:
//!
//!     crate::zuir::ir
//!
//! Canonical full-program verifier:
//!
//!     crate::zuir::verify::verify_full
//!
//! Canonical verification input:
//!
//!     crate::zuir::verify::FullVerificationInput
//!
//! Canonical verification report:
//!
//!     crate::zuir::verify::FullVerificationReport
//!
//! Canonical verification status:
//!
//!     crate::zuir::verify::VerificationStatus
//!
//! Classical integration:
//!
//!     crate::classical::zcir::verify
//!
//! Quantum validation:
//!
//!     crate::quantum::ir::validation
//!
//! HDLIR verification:
//!
//!     crate::hdl::hdlir::verify
//!
//! AI semantic validation:
//!
//!     crate::ai::validation
//!
//! ## Verification trust boundary
//!
//! An IR is not trusted merely because it:
//!
//! - Was produced by the frontend.
//! - Was constructed through a builder.
//! - Was previously verified.
//! - Was loaded from a cache.
//! - Was deserialized.
//! - Was produced by an optimizer.
//! - Was received from another compilation process.
//!
//! Verification must be performed against the exact IR snapshot
//! that the downstream compiler stage will consume.
//!
//! Any transformation that changes the IR invalidates the assumption
//! that a previous verification report describes the changed snapshot.
//!
//! The caller must therefore supply a new, consistent verification
//! input after every transformation that requires renewed verification.
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
//! Only Valid is accepted by this integration boundary.
//!
//! Incomplete must never be silently converted into Valid.
//!
//! ## Completeness boundary
//!
//! A Valid result means that the currently integrated canonical
//! verification stages passed.
//!
//! It does not establish:
//!
//! - Complete SSA use-def legality unless an SSA verifier is integrated.
//! - Target compatibility.
//! - Backend legality.
//! - Hardware feasibility.
//! - Physical resource availability.
//! - Successful synthesis.
//! - Successful execution.
//! - Correctness of arbitrary external dialect implementations.
//!
//! These properties must be established by their respective
//! authoritative verification or realization stages.
//!
//! ## Scalability and POCO-REAF
//!
//! This module introduces no arbitrary limits on:
//!
//! - Programs.
//! - Modules.
//! - Functions.
//! - Regions.
//! - Blocks.
//! - Operations.
//! - Values.
//! - Types.
//! - Effects.
//! - Ownership relationships.
//! - Logical qubits.
//! - Hardware signals.
//! - Tensor dimensions.
//! - Tensor rank.
//! - Devices.
//! - Processing units.
//! - Distributed participants.
//! - Computing domains.
//!
//! It does not duplicate the IR or create fixed-size representations.
//!
//! Verification work is determined by the canonical verifier,
//! the supplied IR, available resources, and explicit policies.
//!
//! Resource exhaustion must not be interpreted as successful
//! verification.
//!
//! Literal infinity cannot be guaranteed by finite software.
//!
//! ## Determinism
//!
//! This module introduces:
//!
//! - No global mutable state.
//! - No random identifiers.
//! - No hidden target selection.
//! - No hardware discovery.
//! - No filesystem access.
//! - No network access.
//! - No console output.
//! - No mutation of the supplied IR.
//!
//! Verification ordering and finding ordering remain the
//! responsibility of the canonical verifier.
//!
//! ## Integration
//!
//! Parent:
//!
//!     src/compiler/ir/mod.rs
//!
//! Compiler root:
//!
//!     src/compiler/mod.rs
//!
//! Initial input:
//!
//!     src/compiler/ir/input.rs
//!
//! Representation adapters:
//!
//!     src/compiler/ir/adapters.rs
//!
//! Downstream consumers:
//!
//!     compiler analysis
//!     compiler pass manager
//!     optimization
//!     lowering
//!     planning
//!     backend integration
//!
//! Domain-specific verification must be integrated only through
//! the actual public APIs of the owning domain subsystems.
//!
//! ## Freeze criteria
//!
//! - Canonical ZUIR verification is invoked without reimplementation.
//! - Invalid IR is never accepted.
//! - Incomplete verification is never accepted.
//! - Reports are preserved without losing canonical findings.
//! - Verification can be repeated after transformations.
//! - No unsafe code is introduced.
//! - No arbitrary hardware ceiling is introduced.
//! - Tests and repository integration pass.

use crate::zuir::verify::{
    verify_full,
    FullVerificationInput,
    FullVerificationReport,
    VerificationStatus,
};

// =============================================================================
// API VERSION
// =============================================================================

/// Version of the compiler IR verification integration API.
///
/// This version is independent of the Zamani language version,
/// grammar version, semantic-model version, ZUIR version,
/// and backend ABI version.
pub const IR_VERIFICATION_API_VERSION: u16 = 1;

// =============================================================================
// VERIFICATION POINT
// =============================================================================

/// Identifies the point in the compiler pipeline at which
/// verification is being performed.
///
/// This value describes the compiler stage requesting verification.
/// It does not change the verification algorithms or their meaning.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum VerificationPoint {
    /// Verification of IR entering the compiler IR boundary.
    ///
    /// This is normally used before analysis or optimization.
    CompilationInput,

    /// Verification of IR after an optimization or transformation.
    ///
    /// The caller must provide a new verification snapshot that
    /// describes the transformed representation.
    AfterTransformation,

    /// Verification before lowering to another representation.
    ///
    /// This check establishes the properties covered by the
    /// canonical verifier, not target-specific legality.
    BeforeLowering,

    /// Verification before backend input construction.
    ///
    /// Backend-specific compatibility and artifact validation
    /// remain separate responsibilities.
    BeforeBackend,
}

// =============================================================================
// VERIFICATION OUTCOME
// =============================================================================

/// Compiler-facing result of one canonical ZUIR verification run.
///
/// This structure retains the original canonical report.
///
/// It does not copy or reinterpret individual verification findings.
///
/// A caller must inspect `status()` or `is_valid()` before allowing
/// the IR to proceed to a stage that requires verified input.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IrVerificationOutcome {
    point: VerificationPoint,
    report: FullVerificationReport,
}

impl IrVerificationOutcome {
    /// Construct an outcome from a canonical verification report.
    ///
    /// This constructor is private because compiler callers must
    /// obtain outcomes through the canonical verification entry point.
    fn from_report(
        point: VerificationPoint,
        report: FullVerificationReport,
    ) -> Self {
        Self {
            point,
            report,
        }
    }

    /// Return the compiler pipeline point associated with this run.
    #[must_use]
    pub const fn point(&self) -> VerificationPoint {
        self.point
    }

    /// Borrow the original canonical verification report.
    ///
    /// The report contains the authoritative findings, stages
    /// completed, severity information, and verification status.
    #[must_use]
    pub fn report(&self) -> &FullVerificationReport {
        &self.report
    }

    /// Consume the outcome and return its canonical report.
    ///
    /// This is useful when an existing API stores
    /// `FullVerificationReport` directly.
    #[must_use]
    pub fn into_report(self) -> FullVerificationReport {
        self.report
    }

    /// Return the canonical verification status.
    #[must_use]
    pub fn status(&self) -> VerificationStatus {
        self.report.status()
    }

    /// Return true only when canonical verification completed
    /// without errors or deferred findings.
    ///
    /// This is the acceptance decision for this verification boundary.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.status() == VerificationStatus::Valid
    }

    /// Return true if the canonical verifier identified invalid IR.
    #[must_use]
    pub fn is_invalid(&self) -> bool {
        self.status() == VerificationStatus::Invalid
    }

    /// Return true if canonical verification was incomplete.
    #[must_use]
    pub fn is_incomplete(&self) -> bool {
        self.status() == VerificationStatus::Incomplete
    }

    /// Return the number of canonical verification findings.
    #[must_use]
    pub fn finding_count(&self) -> usize {
        self.report.issues().len()
    }

    /// Return the verification stages completed by the canonical verifier.
    #[must_use]
    pub fn completed_stages(
        &self,
    ) -> &[crate::zuir::verify::VerificationStage] {
        self.report.stages_completed()
    }

    /// Return the verification integration API version.
    #[must_use]
    pub const fn api_version(&self) -> u16 {
        IR_VERIFICATION_API_VERSION
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
/// It does not implement or duplicate any verification algorithm.
///
/// The supplied `FullVerificationInput` must describe one consistent,
/// immutable snapshot of the program and all related IR entities.
///
/// # Acceptance
///
/// A result is accepted only when:
///
///     outcome.status() == VerificationStatus::Valid
///
/// Invalid and incomplete reports are preserved and returned as
/// outcomes so callers can inspect the canonical findings.
///
/// They are not converted into successful verification.
///
/// # Reverification
///
/// Call this function again after any transformation that changes
/// the IR and requires renewed verification.
///
/// The caller must construct a new `FullVerificationInput` referring
/// to the transformed IR snapshot.
///
/// # Important
///
/// A valid result proves only the properties currently checked by
/// the canonical ZUIR verifier. It does not prove target feasibility,
/// complete SSA legality, backend compatibility, or execution success.
///
/// # Arguments
///
/// * `input` - Complete canonical verification input.
/// * `point` - Compiler pipeline point requesting verification.
///
/// # Returns
///
/// An `IrVerificationOutcome` preserving the canonical report.
#[must_use = "IR verification outcomes must be inspected"]
pub fn verify_zuir(
    input: &FullVerificationInput<'_>,
    point: VerificationPoint,
) -> IrVerificationOutcome {
    let report = verify_full(input);

    IrVerificationOutcome::from_report(point, report)
}

// =============================================================================
// PIPELINE-SPECIFIC ENTRY POINTS
// =============================================================================

/// Verify ZUIR entering the compiler IR pipeline.
///
/// This is the normal entry point for `compiler::ir::input`.
#[must_use = "IR verification outcomes must be inspected"]
pub fn verify_compilation_input(
    input: &FullVerificationInput<'_>,
) -> IrVerificationOutcome {
    verify_zuir(input, VerificationPoint::CompilationInput)
}

/// Verify ZUIR after an optimization or transformation.
///
/// The supplied input must describe the transformed IR snapshot,
/// not the original pre-transformation snapshot.
///
/// A transformation must not reuse a stale verification input if
/// the transformation changed any referenced IR entity.
#[must_use = "IR verification outcomes must be inspected"]
pub fn verify_after_transformation(
    input: &FullVerificationInput<'_>,
) -> IrVerificationOutcome {
    verify_zuir(input, VerificationPoint::AfterTransformation)
}

/// Verify ZUIR immediately before representation lowering.
///
/// This is a target-independent IR check.
///
/// Lowering legality, conversion correctness, and target feasibility
/// must still be established by their respective subsystems.
#[must_use = "IR verification outcomes must be inspected"]
pub fn verify_before_lowering(
    input: &FullVerificationInput<'_>,
) -> IrVerificationOutcome {
    verify_zuir(input, VerificationPoint::BeforeLowering)
}

/// Verify ZUIR immediately before backend input construction.
///
/// This is not a replacement for backend-specific input validation.
///
/// The backend remains responsible for checking its accepted
/// representation, ABI, target compatibility, and artifact contract.
#[must_use = "IR verification outcomes must be inspected"]
pub fn verify_before_backend(
    input: &FullVerificationInput<'_>,
) -> IrVerificationOutcome {
    verify_zuir(input, VerificationPoint::BeforeBackend)
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verification_api_version_is_available() {
        assert_eq!(IR_VERIFICATION_API_VERSION, 1);
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
    fn canonical_report_status_is_preserved() {
        let report = FullVerificationReport::default();

        let outcome = IrVerificationOutcome::from_report(
            VerificationPoint::CompilationInput,
            report.clone(),
        );

        assert_eq!(outcome.report(), &report);
        assert_eq!(outcome.status(), report.status());
        assert_eq!(outcome.is_valid(), report.is_valid());
        assert_eq!(outcome.finding_count(), report.issues().len());
    }

    #[test]
    fn outcome_preserves_verification_point() {
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
    fn outcome_can_return_the_original_report() {
        let report = FullVerificationReport::default();

        let outcome = IrVerificationOutcome::from_report(
            VerificationPoint::BeforeLowering,
            report.clone(),
        );

        assert_eq!(outcome.into_report(), report);
    }

    #[test]
    fn completed_stages_are_exposed_from_canonical_report() {
        let report = FullVerificationReport::default();

        let outcome = IrVerificationOutcome::from_report(
            VerificationPoint::BeforeBackend,
            report.clone(),
        );

        assert_eq!(
            outcome.completed_stages(),
            report.stages_completed()
        );
    }
}
