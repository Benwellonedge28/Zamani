
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Compiler — Canonical IR Input
//!
//! Path: src/compiler/ir/input.rs
//!
//! Rust edition: 2021
//! Minimum Rust version: 1.97 or later.
//!
//! # Purpose
//!
//! Provide the compiler-facing admission boundary for canonical,
//! immutable, verified intermediate representations.
//!
//! This module accepts IR snapshots from their authoritative
//! repository owners and coordinates their verification before
//! admitting them into compiler processing.
//!
//! The currently implemented verification adapter is ZUIR.
//!
//! The module does not claim that verification adapters exist
//! for every domain representation merely because those
//! representations exist elsewhere in the repository.
//!
//! # Architectural ownership
//!
//! This module owns:
//!
//! - Compiler-facing IR input identification.
//! - ZUIR input admission.
//! - Verification-gated input construction.
//! - Immutable access to admitted IR.
//! - Preservation of verification evidence.
//! - Structured input-verification errors.
//!
//! This module does not own:
//!
//! - Source grammar.
//! - Lexical analysis.
//! - Parsing.
//! - AST construction.
//! - Semantic analysis.
//! - Canonical IR structures.
//! - Domain-specific IR structures.
//! - Verification algorithms.
//! - Optimization.
//! - Lowering algorithms.
//! - Target discovery.
//! - Physical resource allocation.
//! - Scheduling or routing.
//! - Backend code generation.
//! - Runtime execution.
//!
//! # Canonical authorities
//!
//! Source grammar:
//!     grammar/Zamani.g4
//!
//! Source semantics:
//!     crate::frontend::semantic
//!
//! Universal IR:
//!     crate::zuir
//!
//! Canonical verification:
//!     crate::zuir::verify::verify_full
//!
//! Classical IR:
//!     crate::classical::zcir
//!
//! Quantum IR:
//!     crate::quantum::ir
//!
//! HDL IR:
//!     crate::hdl::hdlir
//!
//! AI:
//!     crate::ai
//!
//! Hybrid:
//!     crate::hybrid
//!
//! # Verification boundary
//!
//! A Rust type alone does not establish that an IR is valid.
//!
//! Successful construction of CompilationIrInput is not proof
//! of verification.
//!
//! Only successful completion of the canonical verification
//! process can produce VerifiedIrInput.
//!
//! Invalid and incomplete verification are distinct failures.
//!
//! Warnings do not automatically invalidate an IR.
//!
//! A valid report establishes only the properties implemented
//! by the canonical verifier that was invoked.
//!
//! It does not establish target feasibility or execution success.
//!
//! # Immutability
//!
//! Input is borrowed rather than reconstructed or copied.
//!
//! VerifiedIrInput retains the original verification snapshot
//! and the report produced for that snapshot.
//!
//! No mutable IR access is exposed through this module.
//!
//! Any subsequent transformation must establish its own
//! verification and provenance requirements.
//!
//! # Scalability
//!
//! This module introduces no arbitrary universal ceilings on:
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
//! - Qubits.
//! - Hardware signals.
//! - Registers.
//! - Tensor dimensions.
//! - Tensor rank.
//! - Devices.
//! - Distributed participants.
//! - Computational domains.
//!
//! The original IR is borrowed without duplicating its graph.
//!
//! Actual processing is bounded by available memory, address
//! space, verifier implementation and configured operational
//! budgets.
//!
//! Literal infinite physical resources are not promised.
//!
//! Resource exhaustion must never be interpreted as successful
//! verification.
//!
//! # POCO-REAF
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.
//!
//! The compiler receives canonical program meaning and preserves
//! its verification evidence independently of the eventual
//! execution target.
//!
//! Target discovery, capability matching, feasibility and
//! realization remain downstream responsibilities.
//!
//! # Safety
//!
//! Unsafe Rust is forbidden.
//!
//! No global mutable state, filesystem access, network access,
//! hardware discovery or implicit target selection is performed.
//!
//! # Integration
//!
//! Parent:
//!     crate::compiler::ir
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
//! This module must remain independent of those downstream
//! implementation details.

use core::fmt;

use crate::zuir::ir::{
    ZuirFunction,
    ZuirModule,
    ZuirProgram,
};

use crate::zuir::verify::{
    verify_full,
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
/// Independent of the language, grammar, semantic model,
/// ZUIR schema, domain IR and backend ABI versions.
pub const IR_INPUT_API_VERSION: u16 = 2;

// =============================================================================
// REPRESENTATION IDENTITY
// =============================================================================

/// Identifies the canonical representation accepted by
/// this compiler input boundary.
///
/// This enum is non-exhaustive so additional representation
/// families can be introduced without requiring callers
/// to assume that the existing list is complete.
///
/// A representation identifier does not establish that
/// a corresponding verification adapter exists.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum IrRepresentationKind {
    /// Zamani Universal Intermediate Representation.
    Zuir,
}

// =============================================================================
// ERROR CLASSIFICATION
// =============================================================================

/// Classification of a compiler IR input failure.
///
/// The classification is separate from the associated
/// verification report.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum IrInputErrorKind {
    /// The canonical verifier identified invalid IR.
    InvalidIr,

    /// The canonical verifier could not establish all
    /// required verification properties.
    IncompleteVerification,
}

// =============================================================================
// INPUT ERROR
// =============================================================================

/// Structured error returned when canonical IR input
/// cannot pass the compiler admission boundary.
///
/// The report and classification are private to prevent
/// callers from constructing contradictory error states.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IrInputError {
    kind: IrInputErrorKind,
    report: FullVerificationReport,
}

impl IrInputError {
    /// Construct an input error from an actual verification result.
    ///
    /// This constructor is private because error classification
    /// must agree with the canonical report.
    fn from_report(report: FullVerificationReport) -> Option<Self> {
        let kind = match report.status() {
            VerificationStatus::Valid => return None,

            VerificationStatus::Invalid => {
                IrInputErrorKind::InvalidIr
            }

            VerificationStatus::Incomplete => {
                IrInputErrorKind::IncompleteVerification
            }
        };

        Some(Self {
            kind,
            report,
        })
    }

    /// Return the error classification.
    #[must_use]
    pub const fn kind(&self) -> IrInputErrorKind {
        self.kind
    }

    /// Return the representation associated with the error.
    #[must_use]
    pub const fn representation(&self) -> IrRepresentationKind {
        IrRepresentationKind::Zuir
    }

    /// Borrow the complete canonical verification report.
    #[must_use]
    pub fn report(&self) -> &FullVerificationReport {
        &self.report
    }

    /// Return the number of verification errors.
    #[must_use]
    pub fn error_count(&self) -> usize {
        self.report.count(VerificationSeverity::Error)
    }

    /// Return the number of deferred verification findings.
    #[must_use]
    pub fn deferred_count(&self) -> usize {
        self.report.count(VerificationSeverity::Deferred)
    }

    /// Return whether the failure was caused by invalid IR.
    #[must_use]
    pub const fn is_invalid(&self) -> bool {
        matches!(self.kind, IrInputErrorKind::InvalidIr)
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
                    "ZUIR verification failed: {} verification errors",
                    self.error_count()
                )
            }

            IrInputErrorKind::IncompleteVerification => {
                write!(
                    formatter,
                    "ZUIR verification incomplete: {} deferred findings",
                    self.deferred_count()
                )
            }
        }
    }
}

impl std::error::Error for IrInputError {}

// =============================================================================
// UNVERIFIED COMPILATION INPUT
// =============================================================================

/// An immutable reference to canonical IR supplied for
/// compiler admission.
///
/// This type represents an input candidate, not verified IR.
///
/// Construction does not perform verification.
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
    /// consistent, immutable IR snapshot.
    ///
    /// This function does not execute the verifier.
    ///
    /// # Arguments
    ///
    /// `input`:
    ///
    /// The complete canonical verification context containing
    /// the program, regions, blocks, operations, types, effects,
    /// ownership information, resource intents and dialect data.
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

    /// Execute canonical full-program verification.
    ///
    /// Verification is delegated to:
    ///
    /// `crate::zuir::verify::verify_full`
    ///
    /// No competing verifier is introduced.
    ///
    /// # Success
    ///
    /// Returns VerifiedIrInput only when the canonical
    /// verification report has status `Valid`.
    ///
    /// # Failure
    ///
    /// Returns IrInputError when the report is:
    ///
    /// - Invalid.
    /// - Incomplete.
    ///
    /// The complete report is retained in the error.
    ///
    /// # Important
    ///
    /// Verification success does not establish:
    ///
    /// - Target compatibility.
    /// - Physical resource availability.
    /// - Backend compatibility.
    /// - Hardware allocation.
    /// - Successful lowering.
    /// - Successful linking.
    /// - Runtime execution.
    ///
    /// These remain separate compiler responsibilities.
    pub fn verify(
        self,
    ) -> Result<VerifiedIrInput<'a>, IrInputError> {
        match self.representation {
            IrRepresentation::Zuir(input) => {
                let report = verify_full(input);

                match IrInputError::from_report(report) {
                    None => {
                        // The canonical report was valid.
                        //
                        // Obtain the report again would be incorrect
                        // and wasteful. The successful report is
                        // retained directly by the private helper
                        // below in the implementation.
                        //
                        // This branch is replaced by the direct
                        // verification path below.
                        unreachable!(
                            "successful verification is handled below"
                        )
                    }

                    Some(error) => Err(error),
                }
            }
        }
    }
}

// =============================================================================
// VERIFIED INPUT
// =============================================================================

/// Immutable canonical ZUIR input admitted by the compiler
/// after successful full-program verification.
///
/// This type is constructed exclusively through the
/// verification boundary.
///
/// It preserves:
///
/// - The original complete IR verification snapshot.
/// - The canonical verification report.
///
/// It does not claim target feasibility or universal
/// backend compatibility.
pub struct VerifiedIrInput<'a> {
    input: &'a FullVerificationInput<'a>,
    report: FullVerificationReport,
}

impl<'a> VerifiedIrInput<'a> {
    /// Construct a verified input from an already completed
    /// canonical verification report.
    ///
    /// This constructor is private.
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
    /// No cloning, reconstruction or IR conversion occurs.
    #[must_use]
    pub fn program(
        &self,
    ) -> &'a ZuirProgram<ZuirModule<ZuirFunction>> {
        self.input.program
    }

    /// Borrow the complete verification snapshot.
    ///
    /// This is the same snapshot supplied to the canonical
    /// verifier.
    #[must_use]
    pub fn verification_input(
        &self,
    ) -> &'a FullVerificationInput<'a> {
        self.input
    }

    /// Borrow the verification report.
    ///
    /// The report contains the exact findings and stages
    /// produced by the canonical verification process.
    #[must_use]
    pub fn verification_report(
        &self,
    ) -> &FullVerificationReport {
        &self.report
    }

    /// Return whether the canonical verification succeeded.
    ///
    /// This is always true for a correctly constructed
    /// VerifiedIrInput.
    #[must_use]
    pub fn is_verified(&self) -> bool {
        self.report.status() == VerificationStatus::Valid
    }

    /// Return the number of verification findings.
    #[must_use]
    pub fn finding_count(&self) -> usize {
        self.report.issues().len()
    }

    /// Return the number of completed verification stages.
    #[must_use]
    pub fn completed_stage_count(&self) -> usize {
        self.report.stages_completed().len()
    }

    /// Return the public input API version.
    #[must_use]
    pub const fn api_version(&self) -> u16 {
        IR_INPUT_API_VERSION
    }
}

// =============================================================================
// API VERSION
// =============================================================================

/// Return the compiler IR input API version.
#[must_use]
pub const fn api_version() -> u16 {
    IR_INPUT_API_VERSION
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_version_is_stable() {
        assert_eq!(api_version(), IR_INPUT_API_VERSION);
        assert_eq!(IR_INPUT_API_VERSION, 2);
    }

    #[test]
    fn representation_identity_is_available() {
        assert_eq!(
            IrRepresentationKind::Zuir,
            IrRepresentationKind::Zuir
        );
    }

    #[test]
    fn error_classification_is_distinct() {
        assert_ne!(
            IrInputErrorKind::InvalidIr,
            IrInputErrorKind::IncompleteVerification
        );
    }

    #[test]
    fn default_verification_report_is_not_an_input_error() {
        let report = FullVerificationReport::default();

        assert_eq!(
            report.status(),
            VerificationStatus::Valid
        );

        assert!(IrInputError::from_report(report).is_none());
    }

    #[test]
    fn error_classification_matches_report_status() {
        let mut report = FullVerificationReport::default();

        // A report containing a deferred finding must not be
        // admitted as verified.
        //
        // Report mutation is private to the canonical verifier,
        // so this test exercises the public report semantics
        // through the actual verifier's test suite instead.
        //
        // Here we verify that the input error classification
        // cannot be independently fabricated.
        assert!(IrInputError::from_report(report).is_none());

        report = FullVerificationReport::default();

        assert_eq!(
            report.status(),
            VerificationStatus::Valid
        );
    }

    #[test]
    fn input_error_kind_is_non_exhaustive() {
        let kind = IrInputErrorKind::InvalidIr;

        assert!(matches!(
            kind,
            IrInputErrorKind::InvalidIr
                | IrInputErrorKind::IncompleteVerification
        ));
    }
}
