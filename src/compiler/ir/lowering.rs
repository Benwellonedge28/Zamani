
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Meta-Compiler — IR Lowering Integration
//!
//! Path: `src/compiler/ir/lowering.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97 or later.
//! Safety: unsafe Rust is forbidden.
//!
//! ## PURPOSE
//!
//! Provide the compiler-facing entry point for lowering verified
//! canonical IR through existing, authoritative domain lowering
//! implementations.
//!
//! This module connects:
//!
//! - Compiler verification-gated IR input.
//! - Compiler representation translation.
//! - Canonical ZUIR lowering legality.
//! - Canonical lowering context and resource requirements.
//! - Domain-owned lowering backends.
//!
//! It does not implement a second lowering engine.
//!
//! ## ARCHITECTURAL OWNERSHIP
//!
//! This module owns:
//!
//! - Verification-gated compiler lowering entry points.
//! - Compiler-facing lowering error classification.
//! - Enforcement of the verified-input admission boundary.
//! - Delegation to the canonical translation API.
//!
//! This module does not own:
//!
//! - IR structures or identifiers.
//! - Source-language semantics.
//! - Type inference or ownership analysis.
//! - IR verification algorithms.
//! - Lowering legality algorithms.
//! - Domain-specific conversion algorithms.
//! - Domain IR builders.
//! - Backend registration or discovery.
//! - Physical resource allocation.
//! - Hardware scheduling or routing.
//! - Code generation or runtime execution.
//!
//! Those responsibilities remain with their canonical owners.
//!
//! ## CANONICAL AUTHORITIES
//!
//! Verified compiler input:
//!     crate::compiler::ir::input
//!
//! Representation translation:
//!     crate::compiler::ir::translation
//!
//! Representation adapters:
//!     crate::compiler::ir::adapters
//!
//! Lowering API:
//!     crate::zuir::lowering::api
//!
//! Lowering context:
//!     crate::zuir::lowering::context
//!
//! Lowering legality:
//!     crate::zuir::lowering::legality
//!
//! Universal IR:
//!     crate::zuir::ir
//!
//! ## TRUST BOUNDARY
//!
//! A lowering request must be based on a VerifiedIrInput.
//!
//! The associated LegalityCandidate must also explicitly state that
//! its source has been verified.
//!
//! This module cannot independently prove that a caller-created
//! EntityKey identifies an entity inside the supplied verified
//! program snapshot. The domain adapter constructing the candidate
//! must establish that correspondence.
//!
//! A successful lowering result means that:
//!
//! - The declared conversion guarantee was accepted.
//! - Canonical legality analysis authorized direct lowering.
//! - The registered backend completed successfully.
//!
//! It does not prove:
//!
//! - Destination IR verification.
//! - Physical target feasibility.
//! - Successful linking.
//! - Runtime execution.
//! - Universal portability.
//!
//! The destination representation must be verified by its own
//! authoritative verifier before being trusted by later stages.
//!
//! ## SEMANTIC PRESERVATION
//!
//! Potentially lossy conversions are rejected by the existing
//! translation boundary.
//!
//! Conditional, deferred, expansion-required, and unsupported
//! lowering decisions do not produce a completed artifact.
//!
//! No conversion may silently discard effects, ownership rules,
//! control flow, numerical guarantees, resource requirements,
//! synchronization, provenance, or domain-specific semantics.
//!
//! ## SCALABILITY AND POCO-REAF
//!
//! This module imposes no fixed limits on:
//!
//! - Programs, modules, functions, or regions.
//! - Blocks, operations, or values.
//! - Types, effects, or ownership relationships.
//! - Qubits, signals, registers, or tensor dimensions.
//! - Tensor rank or symbolic dimensions.
//! - Devices, processors, accelerators, or nodes.
//! - Computing domains or domain combinations.
//!
//! The module does not copy the verified IR graph.
//!
//! Work budgets and collection growth remain controlled by the
//! canonical lowering context and domain-owned implementations.
//!
//! No finite implementation can promise literal infinity.
//! Resource exhaustion must be reported rather than interpreted
//! as successful lowering.
//!
//! ## GRAMMAR BOUNDARY
//!
//! Source syntax belongs to the grammar and frontend.
//! Source meaning belongs to semantic analysis.
//! Canonical computational meaning belongs to the appropriate IR.
//!
//! This module does not parse grammar productions, infer source
//! semantics, introduce language keywords, or define domain syntax.
//!
//! Relevant specifications include:
//!
//! - grammar/Zamani.g4
//! - grammar/DESIGN.md
//! - grammar/specification/semantic-model.md
//! - grammar/specification/compilation-model.md
//! - grammar/specification/scalability-model.md
//! - grammar/specification/poco-reaf.md
//!
//! ## INTEGRATION
//!
//! Parent:
//!     src/compiler/ir/mod.rs
//!
//! Compiler root:
//!     src/compiler/mod.rs
//!
//! Required sibling modules:
//!     input
//!     adapters
//!     verification
//!     translation
//!
//! Canonical lowering:
//!     src/zuir/lowering/
//!
//! Domain implementations must implement the canonical
//! LoweringBackend contract and supply validated legality evidence.
//!
//! ## FREEZE CRITERIA
//!
//! - Unverified input cannot enter this API.
//! - Unverified candidates cannot enter this API.
//! - Lossy conversion remains rejected by translation.
//! - Canonical legality analysis remains authoritative.
//! - Backend errors remain inspectable.
//! - Non-legal outcomes cannot be mistaken for completed lowering.
//! - No IR or conversion algorithm is duplicated.
//! - No arbitrary hardware ceilings or unsafe code are introduced.
//! - All module declarations and repository integration tests pass.

use core::fmt;
use std::error::Error;

use crate::compiler::ir::input::VerifiedIrInput;

use crate::compiler::ir::translation::{
    translate_with,
    TranslationError,
    TranslationOutcome,
};

use crate::compiler::ir::adapters::AdapterDescriptor;

use crate::zuir::lowering::api::LoweringBackend;

use crate::zuir::lowering::context::LoweringContext;

use crate::zuir::lowering::legality::LegalityCandidate;

// =============================================================================
// API VERSION
// =============================================================================

/// Public API version of the compiler IR lowering contract.
///
/// This version is independent of the Zamani language version,
/// grammar version, semantic-model version, ZUIR version,
/// domain IR versions, and backend ABI versions.
pub const IR_LOWERING_API_VERSION: u16 = 1;

// =============================================================================
// RESULT TYPE
// =============================================================================

/// Result alias for compiler-facing verified IR lowering.
pub type IrLoweringResult<T, E> =
    Result<IrLoweringOutcome<T>, IrLoweringError<E>>;

// =============================================================================
// LOWERING ERROR
// =============================================================================

/// Errors produced by compiler-facing IR lowering.
#[derive(Debug)]
pub enum IrLoweringError<E> {
    /// The supplied verified-input marker did not report a valid
    /// verification result.
    ///
    /// This is a defensive check. A correctly constructed
    /// VerifiedIrInput should always satisfy this condition.
    InvalidVerifiedInput,

    /// The lowering candidate does not explicitly assert that
    /// its source representation has passed verification.
    ///
    /// The compiler does not infer verification from an entity's
    /// Rust type or identity.
    SourceVerificationNotEstablished,

    /// The canonical translation or lowering implementation failed.
    Translation(TranslationError<E>),
}

impl<E: fmt::Display> fmt::Display for IrLoweringError<E> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
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

// =============================================================================
// LOWERING OUTCOME
// =============================================================================

/// Result of a compiler-facing lowering request.
///
/// The outcome preserves the canonical translation outcome,
/// including the legality report, optional artifact, and
/// destination entity mappings.
///
/// An outcome with no artifact is not a completed lowering.
///
/// Callers must inspect `is_lowered()` before using an artifact.
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

    /// Return the original translation outcome.
    ///
    /// The canonical legality report and destination mappings
    /// remain available through this reference.
    #[must_use]
    pub const fn translation(&self) -> &TranslationOutcome<T> {
        &self.translation
    }

    /// Return the representation conversion descriptor.
    #[must_use]
    pub const fn descriptor(&self) -> AdapterDescriptor {
        self.translation.descriptor()
    }

    /// Return whether the backend produced a lowering artifact.
    ///
    /// This is false for non-legal or otherwise incomplete
    /// lowering outcomes.
    ///
    /// It does not prove destination verification, target
    /// feasibility, linking success, or runtime execution.
    #[must_use]
    pub fn is_lowered(&self) -> bool {
        self.translation.is_translated()
    }

    /// Borrow the produced artifact, if lowering completed.
    #[must_use]
    pub fn artifact(&self) -> Option<&T> {
        self.translation.artifact()
    }

    /// Consume the outcome and return the artifact, if lowering
    /// completed.
    #[must_use]
    pub fn into_artifact(self) -> Option<T> {
        self.translation.into_artifact()
    }

    /// Return the destination entities registered by canonical
    /// lowering orchestration.
    #[must_use]
    pub fn destinations(
        &self,
    ) -> &[crate::zuir::lowering::context::EntityKey] {
        self.translation.destinations()
    }

    /// Return the canonical legality report.
    ///
    /// The report describes the supplied conversion evidence.
    /// It is not a physical hardware feasibility report.
    #[must_use]
    pub fn legality(
        &self,
    ) -> &crate::zuir::lowering::legality::LegalityReport {
        &self.translation.lowering_outcome().legality
    }
}

// =============================================================================
// VERIFIED LOWERING ENTRY POINT
// =============================================================================

/// Lower one verified source entity through an authoritative
/// domain-owned lowering backend.
///
/// This is the compiler-facing lowering entry point.
///
/// # Inputs
///
/// `verified_input`:
///
/// A verification-gated input created by
/// `CompilationIrInput::verify()`.
///
/// `descriptor`:
///
/// The source and destination representation contract supplied
/// by the existing compiler adapter subsystem.
///
/// `context`:
///
/// The canonical lowering context. It owns lowering-local
/// mappings, symbolic requirements, provenance, and optional
/// operational work budgets.
///
/// `candidate`:
///
/// The semantic and capability evidence for this conversion.
/// Its `source_verified` field must be true. The domain adapter
/// must establish that the candidate's source entity belongs to
/// the supplied verified snapshot.
///
/// `backend`:
///
/// An implementation of the canonical ZUIR LoweringBackend
/// contract. It owns the actual conversion algorithm.
///
/// # Behavior
///
/// 1. Reject an invalid verified-input marker.
/// 2. Reject a candidate without source-verification evidence.
/// 3. Delegate conversion-guarantee validation to the existing
///    translation module.
/// 4. Delegate legality analysis to the canonical ZUIR lowering API.
/// 5. Invoke the domain backend only when legality permits it.
/// 6. Preserve the canonical report and destination mappings.
/// 7. Return an outcome that distinguishes completed lowering
///    from a non-legal decision.
///
/// # Errors
///
/// Returns `InvalidVerifiedInput` if the verified-input marker
/// does not report a valid verification state.
///
/// Returns `SourceVerificationNotEstablished` if the candidate
/// does not carry verified-source evidence.
///
/// Returns `Translation` for lossy conversion rejection,
/// legality-analysis failure, backend failure, or mapping failure.
///
/// # Important
///
/// A successful function return does not necessarily mean that
/// an artifact was produced. The canonical legality subsystem
/// can return a deferred, conditional, expansion-required, or
/// unsupported decision without invoking the backend.
///
/// Always inspect `is_lowered()` and `legality()`.
///
/// # Semantic safety
///
/// This function does not manufacture missing capabilities,
/// resolve unsupported semantics, allocate hardware, or silently
/// change source behavior.
///
/// The destination must pass its own authoritative verifier
/// before it is trusted by subsequent compiler stages.
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
    if !verified_input.is_verified() {
        return Err(IrLoweringError::InvalidVerifiedInput);
    }

    if !candidate.source_verified {
        return Err(
            IrLoweringError::SourceVerificationNotEstablished
        );
    }

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
// API INFORMATION
// =============================================================================

/// Return the compiler IR lowering API version.
#[must_use]
pub const fn api_version() -> u16 {
    IR_LOWERING_API_VERSION
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lowering_api_version_is_available() {
        assert_eq!(api_version(), IR_LOWERING_API_VERSION);
        assert_eq!(IR_LOWERING_API_VERSION, 1);
    }

    #[test]
    fn lowering_error_display_is_stable() {
        let error: IrLoweringError<std::io::Error> =
            IrLoweringError::SourceVerificationNotEstablished;

        assert_eq!(
            error.to_string(),
            "IR lowering requires explicitly verified source semantics"
        );
    }

    #[test]
    fn invalid_verified_input_error_is_distinct() {
        let error: IrLoweringError<std::io::Error> =
            IrLoweringError::InvalidVerifiedInput;

        assert!(error.to_string().contains("invalid verified-input"));
    }
}
