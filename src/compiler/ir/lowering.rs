
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Compiler — Verified IR Lowering Integration
//!
//! Path: `src/compiler/ir/lowering.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97 or later.
//! Safety: unsafe Rust is forbidden.
//!
//! ## Purpose
//!
//! Provide the compiler-facing admission and orchestration boundary
//! for lowering verified canonical IR through existing, authoritative
//! domain-owned lowering implementations.
//!
//! This module coordinates:
//!
//! - Verification-gated compiler input.
//! - Representation conversion contracts.
//! - Canonical ZUIR lowering legality.
//! - Canonical lowering context.
//! - Domain-owned lowering backends.
//! - Structured result and error propagation.
//!
//! It does not implement a second IR, verifier, lowering engine,
//! conversion algorithm, backend registry, or runtime.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - The compiler-facing verified lowering entry point.
//! - Compiler-specific lowering errors.
//! - Compiler-facing lowering outcomes.
//! - Enforcement of the verified-input admission boundary.
//! - Preservation of canonical translation evidence.
//!
//! It does not own:
//!
//! - Source grammar or parsing.
//! - Semantic analysis.
//! - Canonical IR definitions.
//! - IR verification algorithms.
//! - Lowering legality algorithms.
//! - Domain-specific conversion algorithms.
//! - Destination IR construction.
//! - Hardware discovery or allocation.
//! - Quantum routing or calibration.
//! - HDL synthesis.
//! - Physical scheduling.
//! - Backend code generation.
//! - Runtime execution.
//!
//! ## Canonical authorities
//!
//! Verified compiler input:
//!     crate::compiler::ir::input
//!
//! Representation adapters:
//!     crate::compiler::ir::adapters
//!
//! Representation translation:
//!     crate::compiler::ir::translation
//!
//! Canonical lowering API:
//!     crate::zuir::lowering::api
//!
//! Canonical lowering context:
//!     crate::zuir::lowering::context
//!
//! Canonical lowering legality:
//!     crate::zuir::lowering::legality
//!
//! ## Verification and trust boundary
//!
//! A request must contain a VerifiedIrInput produced through the
//! compiler's canonical verification admission boundary.
//!
//! The supplied LegalityCandidate must also explicitly indicate
//! that its source semantics have been verified.
//!
//! IMPORTANT:
//!
//! The current repository does not expose a canonical API that
//! proves an arbitrary EntityKey belongs to the exact verified
//! program snapshot held by VerifiedIrInput.
//!
//! Therefore, the domain adapter constructing the candidate
//! remains responsible for establishing that relationship.
//!
//! The source_verified flag is evidence supplied by that adapter;
//! it is not an independent cryptographic or structural proof.
//!
//! This module must not claim stronger verification guarantees
//! than the existing canonical APIs actually provide.
//!
//! ## Semantic preservation
//!
//! Potentially lossy conversions are rejected by the translation
//! boundary.
//!
//! Canonical legality analysis remains authoritative.
//!
//! Conditional, deferred, expansion-required, and unsupported
//! decisions do not produce a completed direct-lowering artifact.
//!
//! No lowering backend may silently discard:
//!
//! - Types or value semantics.
//! - Effects.
//! - Ownership and lifetime requirements.
//! - Control or data dependencies.
//! - Numerical guarantees.
//! - Quantum state or measurement semantics.
//! - HDL signal and structural semantics.
//! - Synchronization or communication semantics.
//! - Resource requirements.
//! - Provenance.
//!
//! A destination representation must pass its own authoritative
//! verification before downstream stages trust it.
//!
//! ## Scalability and POCO-REAF
//!
//! This module imposes no arbitrary universal limits on:
//!
//! - Programs, modules, functions, regions, blocks or operations.
//! - Values, types, effects or ownership relationships.
//! - Qubits, signals or registers.
//! - Tensor dimensions or tensor rank.
//! - Devices, processors, accelerators or nodes.
//! - Computing domains or domain combinations.
//!
//! It does not copy the verified IR graph.
//!
//! Work budgets and collection growth belong to the canonical
//! lowering context and the domain-owned implementation.
//!
//! Actual memory, address-space, time, representation and target
//! limits remain real constraints and must be reported.
//!
//! Literal infinite physical resources are not promised.
//!
//! ## Grammar boundary
//!
//! Source syntax belongs to the grammar and frontend.
//! Source meaning belongs to semantic analysis.
//! Canonical computational meaning belongs to the appropriate IR.
//!
//! This module does not parse grammar productions, introduce
//! source keywords, or define domain semantics.
//!
//! Relevant specifications:
//!
//! - grammar/Zamani.g4
//! - grammar/specification/semantic-model.md
//! - grammar/specification/compilation-model.md
//! - grammar/specification/scalability-model.md
//! - grammar/specification/poco-reaf.md
//! - grammar/validation/semantic-boundaries.md
//!
//! ## Integration
//!
//! Parent:
//!     src/compiler/ir/mod.rs
//!
//! Required siblings:
//!     input.rs
//!     adapters.rs
//!     translation.rs
//!     verification.rs
//!
//! Canonical implementation:
//!     src/zuir/lowering/
//!
//! Domain backends must implement the canonical LoweringBackend
//! contract and supply authoritative legality evidence.
//!
//! ## Completion criteria
//!
//! - Unverified compiler input is rejected.
//! - Candidates without source-verification evidence are rejected.
//! - Potentially lossy conversions remain rejected.
//! - Canonical legality analysis remains authoritative.
//! - Backend errors remain inspectable.
//! - Non-legal outcomes cannot be mistaken for completed lowering.
//! - No competing IR, verifier, or lowering engine is introduced.
//! - No unsafe code or arbitrary universal capacity limits exist.
//! - Repository integration tests and quality gates pass.

use core::fmt;
use std::error::Error;

use crate::compiler::ir::adapters::AdapterDescriptor;
use crate::compiler::ir::input::VerifiedIrInput;
use crate::compiler::ir::translation::{
    translate_with,
    TranslationError,
    TranslationOutcome,
};

use crate::zuir::lowering::api::LoweringBackend;
use crate::zuir::lowering::context::{
    EntityKey,
    LoweringContext,
};
use crate::zuir::lowering::legality::LegalityCandidate;

// =============================================================================
// API VERSION
// =============================================================================

/// Public API version of the compiler IR lowering contract.
///
/// This version is independent of the language, grammar, semantic
/// model, ZUIR, domain IR, and backend ABI versions.
pub const IR_LOWERING_API_VERSION: u16 = 1;

/// Return the compiler IR lowering API version.
#[must_use]
pub const fn api_version() -> u16 {
    IR_LOWERING_API_VERSION
}

// =============================================================================
// RESULT TYPE
// =============================================================================

/// Result of a compiler-facing verified IR lowering request.
///
/// A successful result means the lowering request was processed
/// by the canonical translation and legality subsystems.
///
/// It does not necessarily mean that an artifact was produced.
/// Inspect [`IrLoweringOutcome::is_lowered`] before consuming
/// the artifact.
pub type IrLoweringResult<T, E> =
    Result<IrLoweringOutcome<T>, IrLoweringError<E>>;

// =============================================================================
// ERRORS
// =============================================================================

/// Errors produced by compiler-facing IR lowering.
///
/// The underlying translation error is preserved so callers can
/// inspect legality, context, and backend failures without
/// parsing diagnostic strings.
#[derive(Debug)]
pub enum IrLoweringError<E> {
    /// The supplied verified-input marker did not establish
    /// successful canonical verification.
    InvalidVerifiedInput,

    /// The lowering candidate did not explicitly establish
    /// source-verification evidence.
    SourceVerificationNotEstablished,

    /// The canonical translation or lowering implementation failed.
    Translation(TranslationError<E>),
}

impl<E: fmt::Display> fmt::Display for IrLoweringError<E> {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidVerifiedInput => {
                write!(
                    formatter,
                    "IR lowering rejected an invalid verified-input marker"
                )
            }

            Self::SourceVerificationNotEstablished => {
                write!(
                    formatter,
                    "IR lowering requires explicitly verified source semantics"
                )
            }

            Self::Translation(error) => {
                write!(
                    formatter,
                    "compiler IR lowering failed: {error}"
                )
            }
        }
    }
}

impl<E> Error for IrLoweringError<E>
where
    E: Error + 'static,
{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidVerifiedInput => None,
            Self::SourceVerificationNotEstablished => None,
            Self::Translation(error) => Some(error),
        }
    }
}

impl<E> IrLoweringError<E> {
    /// Return the underlying translation error, if one exists.
    #[must_use]
    pub fn translation_error(
        &self,
    ) -> Option<&TranslationError<E>> {
        match self {
            Self::Translation(error) => Some(error),
            _ => None,
        }
    }

    /// Consume the error and return its underlying translation error,
    /// if one exists.
    #[must_use]
    pub fn into_translation_error(
        self,
    ) -> Option<TranslationError<E>> {
        match self {
            Self::Translation(error) => Some(error),
            _ => None,
        }
    }

    /// Return whether the error represents rejected verified input.
    #[must_use]
    pub const fn is_invalid_verified_input(&self) -> bool {
        matches!(self, Self::InvalidVerifiedInput)
    }

    /// Return whether the error represents missing source-verification
    /// evidence.
    #[must_use]
    pub const fn is_source_unverified(&self) -> bool {
        matches!(
            self,
            Self::SourceVerificationNotEstablished
        )
    }
}

// =============================================================================
// OUTCOME
// =============================================================================

/// Result of one compiler-facing lowering request.
///
/// The outcome preserves the canonical translation result,
/// including its legality report, optional artifact, and
/// destination entity mappings.
///
/// An outcome without an artifact is not a completed lowering.
///
/// A completed lowering is not automatically a verified
/// destination representation, feasible physical realization,
/// linked executable, or successful runtime execution.
#[derive(Debug)]
pub struct IrLoweringOutcome<T> {
    translation: TranslationOutcome<T>,
}

impl<T> IrLoweringOutcome<T> {
    /// Construct an outcome from the canonical translation result.
    ///
    /// This constructor is private. Outcomes must originate from
    /// the verified compiler lowering entry point.
    fn new(translation: TranslationOutcome<T>) -> Self {
        Self { translation }
    }

    /// Borrow the original canonical translation outcome.
    ///
    /// The canonical legality report and destination mappings
    /// remain available without reconstruction.
    #[must_use]
    pub const fn translation(&self) -> &TranslationOutcome<T> {
        &self.translation
    }

    /// Consume the outcome and return the canonical translation result.
    ///
    /// This preserves the descriptor, legality report, artifact,
    /// and destination mappings.
    #[must_use]
    pub fn into_translation(self) -> TranslationOutcome<T> {
        self.translation
    }

    /// Return the representation conversion descriptor.
    #[must_use]
    pub const fn descriptor(&self) -> AdapterDescriptor {
        self.translation.descriptor()
    }

    /// Return whether the canonical backend produced an artifact.
    ///
    /// This is false for non-legal decisions or when the canonical
    /// lowering API did not invoke the backend.
    #[must_use]
    pub fn is_lowered(&self) -> bool {
        self.translation.is_translated()
    }

    /// Return whether no lowering artifact was produced.
    ///
    /// A false result does not imply an error: the canonical
    /// legality subsystem may have returned a valid deferred,
    /// conditional, expansion-required, or unsupported outcome.
    #[must_use]
    pub fn is_not_lowered(&self) -> bool {
        !self.is_lowered()
    }

    /// Borrow the produced artifact, if lowering completed.
    #[must_use]
    pub fn artifact(&self) -> Option<&T> {
        self.translation.artifact()
    }

    /// Consume the outcome and return the artifact, if lowering
    /// completed.
    ///
    /// This convenience method discards the associated report,
    /// descriptor, and destination mappings. Use
    /// [`Self::into_translation`] when that evidence is required.
    #[must_use]
    pub fn into_artifact(self) -> Option<T> {
        self.translation.into_artifact()
    }

    /// Borrow the destination entities registered by canonical
    /// lowering orchestration.
    #[must_use]
    pub fn destinations(&self) -> &[EntityKey] {
        self.translation.destinations()
    }

    /// Borrow the canonical lowering-legality report.
    ///
    /// This report describes the supplied conversion evidence.
    /// It is not a physical hardware feasibility report.
    #[must_use]
    pub fn legality(
        &self,
    ) -> &crate::zuir::lowering::legality::LegalityReport {
        self.translation.legality()
    }
}

// =============================================================================
// VERIFIED LOWERING
// =============================================================================

/// Lower one verified source entity through an authoritative
/// domain-owned lowering backend.
///
/// # Arguments
///
/// `verified_input`:
///
/// A verification-gated compiler input created by
/// `CompilationIrInput::verify()`.
///
/// `descriptor`:
///
/// The source and destination representation contract supplied
/// by the existing compiler adapter subsystem.
///
/// `context`:
///
/// The canonical lowering context. It owns lowering-local mappings,
/// symbolic requirements, provenance, and optional operational
/// work accounting.
///
/// `candidate`:
///
/// Semantic, capability, and legality evidence for the conversion.
/// Its `source_verified` field must be true.
///
/// The domain adapter must establish that the candidate's source
/// entity belongs to the exact verified input snapshot.
///
/// `backend`:
///
/// An implementation of the canonical ZUIR LoweringBackend contract.
/// It owns the actual conversion algorithm.
///
/// # Behavior
///
/// 1. Check the compiler verified-input marker.
/// 2. Require explicit source-verification evidence.
/// 3. Validate the representation conversion descriptor.
/// 4. Reject potentially lossy conversions.
/// 5. Delegate legality analysis to canonical ZUIR lowering.
/// 6. Invoke the backend only when legality permits direct lowering.
/// 7. Preserve the canonical report and destination mappings.
///
/// # Errors
///
/// Returns `InvalidVerifiedInput` when the verified-input marker
/// does not report successful verification.
///
/// Returns `SourceVerificationNotEstablished` when the candidate
/// does not carry source-verification evidence.
///
/// Returns `Translation` for malformed descriptors, rejected
/// conversion guarantees, legality-analysis failures, context
/// failures, or backend failures.
///
/// # Important
///
/// A successful function return does not necessarily mean that
/// an artifact was produced.
///
/// Always inspect [`IrLoweringOutcome::is_lowered`] and
/// [`IrLoweringOutcome::legality`].
///
/// This function does not establish destination verification,
/// target feasibility, physical allocation, linking, or execution.
pub fn lower_verified_with<B>(
    verified_input: &VerifiedIrInput<'_>,
    descriptor: AdapterDescriptor,
    context: &mut LoweringContext,
    candidate: LegalityCandidate,
    backend: &mut B,
) -> IrLoweringResult<B::Artifact, B::Error>
where
    B: LoweringBackend,
{
    // Defensive admission check. A correctly constructed
    // VerifiedIrInput should always satisfy this condition.
    if !verified_input.is_verified() {
        return Err(IrLoweringError::InvalidVerifiedInput);
    }

    // The canonical lowering subsystem must not be asked to infer
    // source verification from an entity identity or descriptor.
    if !candidate.source_verified {
        return Err(
            IrLoweringError::SourceVerificationNotEstablished,
        );
    }

    // Do not reconstruct a converter here. The existing translation
    // boundary validates the descriptor and conversion guarantee.
    //
    // The canonical lowering API remains responsible for legality,
    // backend invocation, and destination mapping registration.
    let translation = translate_with(
        descriptor,
        context,
        candidate,
        backend,
    )
    .map_err(IrLoweringError::Translation)?;

    Ok(IrLoweringOutcome::new(translation))
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::io;

    #[test]
    fn lowering_api_version_is_available() {
        assert_eq!(
            api_version(),
            IR_LOWERING_API_VERSION
        );

        assert_eq!(
            IR_LOWERING_API_VERSION,
            1
        );
    }

    #[test]
    fn source_verification_error_is_distinct() {
        let error: IrLoweringError<io::Error> =
            IrLoweringError::SourceVerificationNotEstablished;

        assert!(error.is_source_unverified());

        assert!(!error.is_invalid_verified_input());

        assert!(error.translation_error().is_none());
    }

    #[test]
    fn invalid_verified_input_error_is_distinct() {
        let error: IrLoweringError<io::Error> =
            IrLoweringError::InvalidVerifiedInput;

        assert!(error.is_invalid_verified_input());

        assert!(!error.is_source_unverified());

        assert!(error.translation_error().is_none());
    }

    #[test]
    fn source_verification_error_display_is_stable() {
        let error: IrLoweringError<io::Error> =
            IrLoweringError::SourceVerificationNotEstablished;

        assert_eq!(
            error.to_string(),
            "IR lowering requires explicitly verified source semantics"
        );
    }

    #[test]
    fn invalid_verified_input_error_display_is_stable() {
        let error: IrLoweringError<io::Error> =
            IrLoweringError::InvalidVerifiedInput;

        assert_eq!(
            error.to_string(),
            "IR lowering rejected an invalid verified-input marker"
        );
    }

    #[test]
    fn lowering_error_has_no_translation_error_when_admission_fails() {
        let error: IrLoweringError<io::Error> =
            IrLoweringError::InvalidVerifiedInput;

        assert!(error.into_translation_error().is_none());
    }
}
