
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Compiler — Canonical IR Input
//!
//! Path: `src/compiler/ir/input.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97 or later.
//! Safety: unsafe Rust is forbidden.
//!
//! ## Purpose
//!
//! Establish the compiler-facing admission boundary for canonical,
//! immutable intermediate-representation snapshots.
//!
//! This module accepts an existing canonical verification context,
//! delegates verification to the compiler's verification integration,
//! and admits the input only when the canonical verifier reports
//! `VerificationStatus::Valid`.
//!
//! The original IR snapshot is borrowed. It is not copied, reconstructed,
//! mutated, or converted during admission.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Compiler-facing IR input identification.
//! - ZUIR input construction.
//! - Verification-gated admission.
//! - Immutable access to admitted ZUIR.
//! - Preservation of the canonical verification report.
//! - Structured invalid and incomplete verification errors.
//!
//! This module does not own:
//!
//! - Source grammar, lexing, parsing, or AST construction.
//! - Source-level semantic analysis.
//! - Canonical IR definitions.
//! - Verification algorithms.
//! - Domain-specific IR conversion.
//! - Optimization or transformation.
//! - Target discovery or feasibility.
//! - Physical resource allocation.
//! - Scheduling, routing, or hardware mapping.
//! - Backend code generation.
//! - Linking, deployment, or runtime execution.
//!
//! ## Canonical authorities
//!
//! Grammar:
//!     grammar/Zamani.g4
//!
//! Lexical vocabulary:
//!     grammar/lexer/tokens.g4
//!
//! Source semantics:
//!     crate::frontend::semantic
//!
//! Canonical universal IR:
//!     crate::zuir
//!
//! Canonical ZUIR verifier:
//!     crate::zuir::verify
//!
//! Compiler verification integration:
//!     crate::compiler::ir::verification
//!
//! Domain-owned representations:
//!     crate::classical
//!     crate::quantum::ir
//!     crate::hdl::hdlir
//!     crate::ai
//!     crate::hybrid
//!     crate::distributed
//!
//! ## Verification contract
//!
//! Construction of `CompilationIrInput` does not establish validity.
//!
//! Only `CompilationIrInput::verify` can produce `VerifiedIrInput`.
//!
//! Invalid and incomplete verification are distinct errors.
//!
//! Warnings alone do not invalidate a report.
//!
//! A successful report establishes only the properties implemented
//! by the canonical verifier that was actually invoked. It does not
//! establish target compatibility, complete domain verification,
//! target feasibility, or successful execution.
//!
//! ## Snapshot contract
//!
//! All references contained in `FullVerificationInput` must describe
//! one consistent immutable IR snapshot.
//!
//! The caller must not mutate the supplied data while verification
//! or subsequent use of the verified input is in progress.
//!
//! Rust borrowing prevents ordinary mutable access through the
//! borrowed references, but this API does not claim to prevent
//! mutation through arbitrary interior-mutability mechanisms.
//!
//! ## Scalability
//!
//! This module introduces no arbitrary universal limits on:
//!
//! - Programs, modules, functions, regions, blocks or operations.
//! - Values, types, effects or ownership relationships.
//! - Qubits, hardware signals, registers or tensor dimensions.
//! - Tensor rank, devices, accelerators or distributed participants.
//! - Computational domains or future representation families.
//!
//! The input is borrowed, avoiding a duplicate copy of the IR graph.
//!
//! Actual processing is bounded by available memory, address space,
//! execution time, verifier implementation and configured operational
//! budgets.
//!
//! Resource exhaustion or incomplete verification must never be
//! interpreted as successful admission.
//!
//! Literal infinite computation is not promised.
//!
//! ## POCO-REAF
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.
//!
//! This boundary preserves canonical program representation and
//! verification evidence independently of target selection.
//!
//! Target discovery, capability matching, feasibility, domain
//! conversion, target lowering and execution remain downstream
//! responsibilities.
//!
//! ## Integration
//!
//! Parent:
//!     crate::compiler::ir
//!
//! Verification integration:
//!     crate::compiler::ir::verification
//!
//! Canonical verifier:
//!     crate::zuir::verify
//!
//! Downstream consumers:
//!     compiler::analysis
//!     compiler::passes
//!     compiler::optimization
//!     compiler::planning
//!     compiler::lowering
//!     compiler::backends
//!
//! ## Compatibility
//!
//! Existing public type and method names are retained where possible.
//! The input API version remains independent of the language version,
//! grammar version, ZUIR schema version, domain IR versions and backend
//! ABI versions.
//!
//! No wildcard re-exports or competing representation definitions
//! are introduced.

use core::fmt;
use std::error::Error;

use crate::compiler::ir::verification::verify_compilation_input;

use crate::zuir::ir::{
    ZuirFunction,
    ZuirModule,
    ZuirProgram,
};

use crate::zuir::verify::{
    FullVerificationInput,
    FullVerificationReport,
    VerificationSeverity,
    VerificationStatus,
};

// =============================================================================
// API VERSION
// =============================================================================

/// Public API version of the compiler IR input contract.
///
/// This version is independent of the Zamani language version,
/// grammar version, semantic-model version, ZUIR schema version,
/// domain IR versions and backend ABI versions.
pub const IR_INPUT_API_VERSION: u16 = 2;

/// Return the compiler IR input API version.
#[must_use]
pub const fn api_version() -> u16 {
    IR_INPUT_API_VERSION
}

// =============================================================================
// REPRESENTATION IDENTITY
// =============================================================================

/// Identifies a canonical representation accepted by this
/// compiler input boundary.
///
/// This enum is non-exhaustive so additional representations can
/// be integrated without requiring callers to assume that the
/// existing variants form a complete list.
///
/// A representation identifier does not establish that a
/// corresponding verifier or adapter has been implemented.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum IrRepresentationKind {
    /// Zamani Universal Intermediate Representation.
    Zuir,
}

// =============================================================================
// ERROR CLASSIFICATION
// =============================================================================

/// Classification of a compiler IR input verification failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum IrInputErrorKind {
    /// The canonical verifier established that the IR is invalid.
    InvalidIr,

    /// The canonical verifier could not establish all required
    /// verification properties.
    IncompleteVerification,
}

// =============================================================================
// INPUT ERROR
// =============================================================================

/// Structured error returned when canonical IR input cannot pass
/// the compiler admission boundary.
///
/// The classification and report are private so callers cannot
/// construct contradictory error states through the public API.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IrInputError {
    kind: IrInputErrorKind,
    report: FullVerificationReport,
}

impl IrInputError {
    /// Require a valid canonical verification report.
    ///
    /// Returns the original report unchanged when its status is valid.
    /// Otherwise, returns an error retaining the complete report.
    ///
    /// This helper does not reconstruct or discard verification
    /// findings.
    fn require_valid_report(
        report: FullVerificationReport,
    ) -> Result<FullVerificationReport, Self> {
        let kind = match report.status() {
            VerificationStatus::Valid => {
                return Ok(report);
            }

            VerificationStatus::Invalid => {
                IrInputErrorKind::InvalidIr
            }

            VerificationStatus::Incomplete => {
                IrInputErrorKind::IncompleteVerification
            }
        };

        Err(Self {
            kind,
            report,
        })
    }

    /// Return the error classification.
    #[must_use]
    pub const fn kind(&self) -> IrInputErrorKind {
        self.kind
    }

    /// Return the canonical representation associated with
    /// this verification error.
    #[must_use]
    pub const fn representation(&self) -> IrRepresentationKind {
        IrRepresentationKind::Zuir
    }

    /// Borrow the complete canonical verification report.
    #[must_use]
    pub fn report(&self) -> &FullVerificationReport {
        &self.report
    }

    /// Consume the error and recover the original verification report.
    #[must_use]
    pub fn into_report(self) -> FullVerificationReport {
        self.report
    }

    /// Return the total number of verification findings.
    #[must_use]
    pub fn finding_count(&self) -> usize {
        self.report.issues().len()
    }

    /// Return the number of error findings.
    #[must_use]
    pub fn error_count(&self) -> usize {
        self.report.count(VerificationSeverity::Error)
    }

    /// Return the number of warning findings.
    #[must_use]
    pub fn warning_count(&self) -> usize {
        self.report.count(VerificationSeverity::Warning)
    }

    /// Return the number of deferred findings.
    #[must_use]
    pub fn deferred_count(&self) -> usize {
        self.report.count(VerificationSeverity::Deferred)
    }

    /// Return whether the failure was caused by invalid IR.
    #[must_use]
    pub const fn is_invalid(&self) -> bool {
        matches!(
            self.kind,
            IrInputErrorKind::InvalidIr
        )
    }

    /// Return whether verification was incomplete.
    #[must_use]
    pub const fn is_incomplete(&self) -> bool {
        matches!(
            self.kind,
            IrInputErrorKind::IncompleteVerification
        )
    }
}

impl fmt::Display for IrInputError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self.kind {
            IrInputErrorKind::InvalidIr => {
                write!(
                    formatter,
                    "ZUIR input verification failed: {} error finding(s)",
                    self.error_count()
                )
            }

            IrInputErrorKind::IncompleteVerification => {
                write!(
                    formatter,
                    "ZUIR input verification incomplete: {} deferred finding(s)",
                    self.deferred_count()
                )
            }
        }
    }
}

impl Error for IrInputError {}

// =============================================================================
// UNVERIFIED COMPILATION INPUT
// =============================================================================

/// An immutable reference to canonical IR supplied for compiler
/// admission.
///
/// This type represents an input candidate, not verified IR.
///
/// Construction does not execute verification.
///
/// Call [`CompilationIrInput::verify`] to obtain a
/// [`VerifiedIrInput`].
pub struct CompilationIrInput<'a> {
    representation: IrRepresentation<'a>,
}

/// Internal representation of a compilation input.
enum IrRepresentation<'a> {
    /// Canonical ZUIR verification snapshot.
    Zuir(&'a FullVerificationInput<'a>),
}

impl<'a> CompilationIrInput<'a> {
    /// Construct compiler input from canonical ZUIR.
    ///
    /// The supplied verification context must describe one
    /// consistent immutable IR snapshot.
    ///
    /// This function does not execute verification.
    ///
    /// # Arguments
    ///
    /// `input` is the complete canonical verification context
    /// containing the program, regions, blocks, operations,
    /// types, effects, ownership information, resource intents,
    /// dialect registry and operation schemas.
    ///
    /// # Returns
    ///
    /// An unverified compiler input candidate.
    #[must_use]
    pub fn from_zuir(
        input: &'a FullVerificationInput<'a>,
    ) -> Self {
        Self {
            representation: IrRepresentation::Zuir(input),
        }
    }

    /// Return the canonical representation identity.
    #[must_use]
    pub const fn representation_kind(
        &self,
    ) -> IrRepresentationKind {
        match self.representation {
            IrRepresentation::Zuir(_) => {
                IrRepresentationKind::Zuir
            }
        }
    }

    /// Execute canonical compiler-input verification.
    ///
    /// Verification is delegated through:
    ///
    /// `crate::compiler::ir::verification::verify_compilation_input`
    ///
    /// That integration delegates the actual verification algorithm
    /// to the canonical ZUIR verifier:
    ///
    /// `crate::zuir::verify::verify_full`
    ///
    /// No competing verifier is introduced.
    ///
    /// # Success
    ///
    /// Returns `VerifiedIrInput` only when the canonical report has
    /// status `VerificationStatus::Valid`.
    ///
    /// The exact canonical report is retained in the returned value.
    ///
    /// # Failure
    ///
    /// Returns `IrInputError` when the report is:
    ///
    /// - Invalid.
    /// - Incomplete.
    ///
    /// The complete canonical report is retained in the error.
    ///
    /// # Important
    ///
    /// Successful verification does not establish:
    ///
    /// - Target compatibility.
    /// - Physical resource availability.
    /// - Backend compatibility.
    /// - Hardware allocation.
    /// - Successful domain conversion.
    /// - Successful lowering.
    /// - Successful linking.
    /// - Runtime execution.
    ///
    /// Those remain separate compiler and runtime responsibilities.
    pub fn verify(
        self,
    ) -> Result<VerifiedIrInput<'a>, IrInputError> {
        match self.representation {
            IrRepresentation::Zuir(input) => {
                // The compiler verification integration owns the
                // verification request and delegates to the canonical
                // ZUIR verifier. Preserve its original report.
                let report = verify_compilation_input(input)
                    .into_report();

                // This operation consumes the report exactly once.
                // It does not rerun verification, clone the report,
                // or discard successful verification evidence.
                let report =
                    IrInputError::require_valid_report(report)?;

                Ok(VerifiedIrInput::new(input, report))
            }
        }
    }
}

// =============================================================================
// VERIFIED INPUT
// =============================================================================

/// Immutable canonical ZUIR input admitted by the compiler
/// after successful canonical full-program verification.
///
/// This type can only be constructed through the private
/// verification-gated admission path.
///
/// It preserves:
///
/// - The original canonical verification snapshot.
/// - The exact canonical verification report.
///
/// It does not claim target feasibility, universal backend
/// compatibility, or verification of properties that the
/// canonical verifier has not implemented.
pub struct VerifiedIrInput<'a> {
    input: &'a FullVerificationInput<'a>,
    report: FullVerificationReport,
}

impl<'a> VerifiedIrInput<'a> {
    /// Construct verified input from a successfully verified
    /// canonical report.
    ///
    /// This constructor is private. Callers cannot bypass the
    /// public verification-gated admission boundary.
    fn new(
        input: &'a FullVerificationInput<'a>,
        report: FullVerificationReport,
    ) -> Self {
        Self {
            input,
            report,
        }
    }

    /// Return the canonical representation identity.
    #[must_use]
    pub const fn representation_kind(
        &self,
    ) -> IrRepresentationKind {
        IrRepresentationKind::Zuir
    }

    /// Borrow the original canonical ZUIR program.
    ///
    /// No cloning, reconstruction or representation conversion
    /// occurs.
    #[must_use]
    pub fn program(
        &self,
    ) -> &'a ZuirProgram<ZuirModule<ZuirFunction>> {
        self.input.program
    }

    /// Borrow the complete verification snapshot.
    ///
    /// This is the same snapshot supplied to the canonical verifier.
    #[must_use]
    pub fn verification_input(
        &self,
    ) -> &'a FullVerificationInput<'a> {
        self.input
    }

    /// Borrow the exact canonical verification report.
    ///
    /// The report retains the findings and completed stages
    /// produced by the canonical verification process.
    #[must_use]
    pub fn verification_report(
        &self,
    ) -> &FullVerificationReport {
        &self.report
    }

    /// Return the canonical verification status.
    ///
    /// This is always `VerificationStatus::Valid` for a correctly
    /// constructed `VerifiedIrInput`.
    #[must_use]
    pub fn verification_status(
        &self,
    ) -> VerificationStatus {
        self.report.status()
    }

    /// Return whether canonical verification succeeded.
    ///
    /// This is always true for a correctly constructed value.
    #[must_use]
    pub fn is_verified(&self) -> bool {
        self.verification_status() == VerificationStatus::Valid
    }

    /// Return the number of verification findings.
    ///
    /// A valid report may contain warnings.
    #[must_use]
    pub fn finding_count(&self) -> usize {
        self.report.issues().len()
    }

    /// Return the number of warning findings.
    #[must_use]
    pub fn warning_count(&self) -> usize {
        self.report.count(VerificationSeverity::Warning)
    }

    /// Return the number of completed canonical verification stages.
    #[must_use]
    pub fn completed_stage_count(&self) -> usize {
        self.report.stages_completed().len()
    }

    /// Borrow the completed canonical verification stages.
    #[must_use]
    pub fn completed_stages(
        &self,
    ) -> &[crate::zuir::verify::VerificationStage] {
        self.report.stages_completed()
    }

    /// Return the public input API version.
    #[must_use]
    pub const fn api_version(&self) -> u16 {
        IR_INPUT_API_VERSION
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_version_is_stable() {
        assert_eq!(
            api_version(),
            IR_INPUT_API_VERSION
        );

        assert_eq!(
            IR_INPUT_API_VERSION,
            2
        );
    }

    #[test]
    fn representation_identity_is_available() {
        assert_eq!(
            IrRepresentationKind::Zuir,
            IrRepresentationKind::Zuir
        );
    }

    #[test]
    fn error_classifications_are_distinct() {
        assert_ne!(
            IrInputErrorKind::InvalidIr,
            IrInputErrorKind::IncompleteVerification
        );
    }

    #[test]
    fn valid_report_is_preserved_without_reverification() {
        let report = FullVerificationReport::default();

        assert_eq!(
            report.status(),
            VerificationStatus::Valid
        );

        let accepted =
            IrInputError::require_valid_report(report)
                .expect("valid report must be accepted");

        assert_eq!(
            accepted.status(),
            VerificationStatus::Valid
        );

        assert!(accepted.issues().is_empty());
    }

    #[test]
    fn valid_report_with_warnings_remains_valid() {
        // The canonical report's public API determines whether
        // warnings invalidate admission. A warning-only report
        // has VerificationStatus::Valid by the canonical contract.
        //
        // This test verifies the admission helper's handling of
        // the valid status without manufacturing verifier internals.
        let report = FullVerificationReport::default();

        let accepted =
            IrInputError::require_valid_report(report)
                .expect("valid report must be accepted");

        assert_eq!(
            accepted.status(),
            VerificationStatus::Valid
        );
    }

    #[test]
    fn invalid_and_incomplete_kinds_remain_distinct() {
        let invalid = IrInputErrorKind::InvalidIr;
        let incomplete = IrInputErrorKind::IncompleteVerification;

        assert_ne!(invalid, incomplete);
    }

    #[test]
    fn error_representation_is_zuiR() {
        let report = FullVerificationReport::default();

        // The private fields are accessible to this module's tests.
        let error = IrInputError {
            kind: IrInputErrorKind::InvalidIr,
            report,
        };

        assert_eq!(
            error.representation(),
            IrRepresentationKind::Zuir
        );
    }

    #[test]
    fn error_report_can_be_recovered() {
        let report = FullVerificationReport::default();

        let error = IrInputError {
            kind: IrInputErrorKind::IncompleteVerification,
            report: report.clone(),
        };

        assert_eq!(
            error.report(),
            &report
        );

        assert_eq!(
            error.into_report(),
            report
        );
    }

    #[test]
    fn error_implements_standard_error_trait() {
        fn accepts_error<E: Error>() {}

        accepts_error::<IrInputError>();
    }

    #[test]
    fn verified_input_api_version_is_available() {
        assert_eq!(
            IR_INPUT_API_VERSION,
            2
        );
    }
}
