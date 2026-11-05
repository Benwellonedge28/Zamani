
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Meta-Compiler — IR Translation
//!
//! Path: `src/compiler/ir/translation.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97 or later.
//! Safety: unsafe Rust is forbidden.
//!
//! ## Purpose
//!
//! Provide the compiler-facing orchestration boundary for translating
//! canonical intermediate representations through domain-owned lowering
//! implementations.
//!
//! This module coordinates existing translation and lowering contracts.
//! It does not implement another IR, verifier, semantic model, optimizer,
//! backend registry, or domain-specific conversion algorithm.
//!
//! ## Canonical authorities
//!
//! - Representation adapters:
//!   `crate::compiler::ir::adapters`
//! - Verification:
//!   `crate::compiler::ir::verification`
//! - Verified compiler input:
//!   `crate::compiler::ir::input`
//! - Lowering context and identity mappings:
//!   `crate::zuir::lowering::context`
//! - Lowering legality:
//!   `crate::zuir::lowering::legality`
//! - Lowering orchestration:
//!   `crate::zuir::lowering::api`
//! - Canonical IR:
//!   `crate::zuir::ir`
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Compiler-facing translation orchestration.
//! - Translation result metadata.
//! - Translation-specific error classification.
//! - The policy that potentially lossy translations are not accepted
//!   through the normal translation entry point.
//!
//! This module does not own:
//!
//! - IR definitions or identifiers.
//! - Source-language semantics.
//! - Type inference or ownership analysis.
//! - Domain-specific conversion algorithms.
//! - Verification algorithms.
//! - Optimization passes.
//! - Hardware discovery or physical resource allocation.
//! - Scheduling, routing, or device placement.
//! - Backend code generation.
//! - Runtime execution.
//!
//! ## Translation guarantees
//!
//! A translation descriptor must declare its conversion guarantee.
//!
//! Identity and documented semantic-preservation conversions are
//! accepted by this API.
//!
//! Potentially lossy conversions are rejected. A caller must not
//! silently discard effects, ownership constraints, control flow,
//! numerical guarantees, resource requirements, provenance, or
//! domain-specific semantics.
//!
//! ## Verification
//!
//! This module does not independently verify IR.
//!
//! Callers must establish source verification using the canonical
//! verifier before asserting that the source is verified in the
//! lowering legality candidate.
//!
//! A successful translation does not automatically establish that
//! the destination representation is fully verified.
//!
//! The destination must pass its authoritative verifier before it
//! is trusted by subsequent compiler stages.
//!
//! ## Scalability
//!
//! This module introduces no fixed limits on:
//!
//! - Representations.
//! - Modules.
//! - Functions.
//! - Regions.
//! - Blocks.
//! - Operations.
//! - Values.
//! - Types.
//! - Qubits.
//! - Signals.
//! - Tensor dimensions.
//! - Tensor rank.
//! - Devices.
//! - Processing units.
//! - Distributed participants.
//! - Computing domains.
//!
//! The translation coordinator does not duplicate the IR graph.
//! Collection growth, conversion work, and resource consumption
//! belong to the canonical domain implementation and explicit
//! operational budgets.
//!
//! No finite implementation can promise literal infinite resources.
//! Resource exhaustion must be reported, not interpreted as success.
//!
//! ## Determinism
//!
//! This module introduces no random identifiers, global mutable state,
//! implicit hardware discovery, filesystem operations, network access,
//! or console output.
//!
//! Deterministic destination ordering and mapping behavior are delegated
//! to the canonical lowering API.
//!
//! ## Grammar boundary
//!
//! Source syntax and parsing remain owned by the grammar and frontend.
//! Source meaning remains owned by semantic analysis.
//!
//! This module accepts established representation and legality contracts.
//! It does not parse grammar productions, infer source semantics, or
//! introduce source-language keywords.
//!
//! ## Integration
//!
//! Parent:
//!   `src/compiler/ir/mod.rs`
//!
//! Compiler root:
//!   `src/compiler/mod.rs`
//!
//! Existing adapters:
//!   `src/compiler/ir/adapters.rs`
//!
//! Verification:
//!   `src/compiler/ir/verification.rs`
//!
//! Lowering authority:
//!   `src/zuir/lowering/`
//!
//! Domain-owned conversion implementations must be supplied by their
//! respective subsystems and invoked through this boundary.
//!
//! ## Freeze criteria
//!
//! - All conversions delegate to an authoritative domain implementation.
//! - Potentially lossy conversions are rejected by default.
//! - Non-legal lowering outcomes cannot be mistaken for completed translation.
//! - Backend and legality errors remain inspectable.
//! - No IR, verifier, or domain conversion algorithm is duplicated.
//! - No arbitrary hardware ceilings or unsafe code are introduced.
//! - The compiler module tree declares this file correctly.
//! - Repository formatting, compilation, tests, and Clippy pass.

use core::fmt;
use std::error::Error;

use crate::compiler::ir::adapters::{
    AdapterDescriptor,
    ConversionGuarantee,
};

use crate::zuir::lowering::api::{
    lower_with,
    LoweringApiError,
    LoweringBackend,
    LoweringOutcome,
};

use crate::zuir::lowering::context::LoweringContext;

use crate::zuir::lowering::legality::LegalityCandidate;

// =============================================================================
// API VERSION
// =============================================================================

/// Public API version of the compiler IR translation contract.
///
/// This version is independent of the Zamani language version,
/// grammar version, semantic-model version, ZUIR version, and
/// backend ABI version.
pub const IR_TRANSLATION_API_VERSION: u16 = 1;

// =============================================================================
// TRANSLATION ERROR
// =============================================================================

/// Errors produced by compiler-facing IR translation orchestration.
#[derive(Debug)]
pub enum TranslationError<E> {
    /// The requested conversion is potentially lossy.
    ///
    /// Potentially lossy conversions are not accepted by the normal
    /// translation API because they could change program meaning.
    LossyConversionNotPermitted {
        /// The conversion descriptor that was rejected.
        descriptor: AdapterDescriptor,
    },

    /// The canonical lowering subsystem rejected or could not
    /// complete the translation.
    Lowering(LoweringApiError<E>),
}

impl<E: fmt::Display> fmt::Display for TranslationError<E> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LossyConversionNotPermitted { descriptor } => {
                write!(
                    formatter,
                    "potentially lossy IR translation is not permitted: {} -> {}",
                    descriptor.source().as_str(),
                    descriptor.destination().as_str(),
                )
            }

            Self::Lowering(error) => {
                write!(formatter, "IR translation failed: {error}")
            }
        }
    }
}

impl<E> Error for TranslationError<E>
where
    E: Error + 'static,
{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::LossyConversionNotPermitted { .. } => None,
            Self::Lowering(error) => Some(error),
        }
    }
}

// =============================================================================
// TRANSLATION OUTCOME
// =============================================================================

/// Result of one compiler-facing IR translation request.
///
/// The descriptor identifies the requested representation direction
/// and its declared conversion guarantee.
///
/// The lowering outcome preserves the canonical legality report,
/// optional produced artifact, and destination entity mappings.
///
/// An outcome with no artifact is not a completed translation.
/// Callers must inspect `is_translated()` before consuming an artifact.
#[derive(Debug)]
pub struct TranslationOutcome<T> {
    descriptor: AdapterDescriptor,
    lowering: LoweringOutcome<T>,
}

impl<T> TranslationOutcome<T> {
    /// Construct an outcome from a descriptor and canonical lowering result.
    ///
    /// This constructor is private because translation outcomes must
    /// originate from the translation orchestration boundary.
    fn new(
        descriptor: AdapterDescriptor,
        lowering: LoweringOutcome<T>,
    ) -> Self {
        Self {
            descriptor,
            lowering,
        }
    }

    /// Return the translation descriptor.
    #[must_use]
    pub const fn descriptor(&self) -> AdapterDescriptor {
        self.descriptor
    }

    /// Return the canonical lowering outcome.
    ///
    /// This preserves the original legality report and destination
    /// mapping information without reconstructing either.
    #[must_use]
    pub const fn lowering_outcome(&self) -> &LoweringOutcome<T> {
        &self.lowering
    }

    /// Return whether the translation produced an artifact.
    ///
    /// This returns false when legality was not established or when
    /// the canonical lowering API did not invoke the backend.
    ///
    /// It does not establish destination verification, target
    /// feasibility, artifact validity, or runtime success.
    #[must_use]
    pub fn is_translated(&self) -> bool {
        self.lowering.is_lowered()
    }

    /// Borrow the translated artifact, if one was produced.
    #[must_use]
    pub fn artifact(&self) -> Option<&T> {
        self.lowering.artifact.as_ref()
    }

    /// Consume the outcome and return the artifact, if one was produced.
    #[must_use]
    pub fn into_artifact(self) -> Option<T> {
        self.lowering.into_artifact()
    }

    /// Return the destination entity mappings registered by the
    /// canonical lowering API.
    #[must_use]
    pub fn destinations(
        &self,
    ) -> &[crate::zuir::lowering::context::EntityKey] {
        &self.lowering.destinations
    }
}

// =============================================================================
// TRANSLATION POLICY
// =============================================================================

/// Return whether a conversion guarantee is accepted by the normal
/// compiler translation API.
///
/// Identity conversions preserve the existing canonical representation.
///
/// Verified semantic-preservation conversions are accepted only when
/// the authoritative adapter declares that guarantee.
///
/// Potentially lossy conversions are rejected.
///
/// This function evaluates the declared guarantee. It does not
/// independently prove the adapter's semantic-preservation claim.
#[must_use]
pub const fn permits_translation(
    guarantee: ConversionGuarantee,
) -> bool {
    matches!(
        guarantee,
        ConversionGuarantee::Identity
            | ConversionGuarantee::VerifiedSemanticPreservation
    )
}

// =============================================================================
// TRANSLATION ORCHESTRATION
// =============================================================================

/// Translate one source entity through an authoritative lowering backend.
///
/// This is the compiler-facing translation entry point.
///
/// The function:
///
/// 1. Checks the adapter's declared conversion guarantee.
/// 2. Rejects potentially lossy conversion.
/// 3. Delegates legality analysis to the canonical ZUIR lowering API.
/// 4. Invokes the backend only when legality permits direct lowering.
/// 5. Preserves the canonical legality report and destination mappings.
///
/// The backend must implement the existing `LoweringBackend` contract.
/// It owns the actual representation conversion algorithm.
///
/// # Important
///
/// This function does not create a converter for a representation pair.
/// It coordinates a converter supplied by the owning domain subsystem.
///
/// It does not independently verify the source or destination IR.
///
/// # Errors
///
/// Returns `TranslationError::LossyConversionNotPermitted` when the
/// descriptor declares a potentially lossy conversion.
///
/// Returns `TranslationError::Lowering` when canonical legality analysis,
/// backend execution, or lowering-context mapping fails.
///
/// A non-legal but structurally valid lowering decision is represented
/// by a successful function return containing an outcome with no artifact.
/// The caller must inspect `is_translated()` and the legality report.
pub fn translate_with<B>(
    descriptor: AdapterDescriptor,
    context: &mut LoweringContext,
    candidate: LegalityCandidate,
    backend: &mut B,
) -> Result<
    TranslationOutcome<B::Artifact>,
    TranslationError<B::Error>,
>
where
    B: LoweringBackend,
{
    if !permits_translation(descriptor.guarantee()) {
        return Err(
            TranslationError::LossyConversionNotPermitted {
                descriptor,
            },
        );
    }

    let lowering = lower_with(context, candidate, backend)
        .map_err(TranslationError::Lowering)?;

    Ok(TranslationOutcome::new(
        descriptor,
        lowering,
    ))
}

// =============================================================================
// API VERSION
// =============================================================================

/// Return the compiler IR translation API version.
#[must_use]
pub const fn api_version() -> u16 {
    IR_TRANSLATION_API_VERSION
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::compiler::ir::adapters::Representation;

    #[test]
    fn translation_api_version_is_available() {
        assert_eq!(api_version(), IR_TRANSLATION_API_VERSION);
        assert_eq!(IR_TRANSLATION_API_VERSION, 1);
    }

    #[test]
    fn identity_conversion_is_permitted() {
        assert!(permits_translation(
            ConversionGuarantee::Identity
        ));
    }

    #[test]
    fn verified_semantic_preservation_is_permitted() {
        assert!(permits_translation(
            ConversionGuarantee::VerifiedSemanticPreservation
        ));
    }

    #[test]
    fn potentially_lossy_conversion_is_rejected() {
        assert!(!permits_translation(
            ConversionGuarantee::PotentiallyLossy
        ));
    }

    #[test]
    fn translation_descriptor_preserves_representation_direction() {
        let descriptor = AdapterDescriptor::new(
            Representation::Zcir,
            Representation::Zuir,
            ConversionGuarantee::Identity,
        );

        assert_eq!(
            descriptor.source(),
            Representation::Zcir
        );

        assert_eq!(
            descriptor.destination(),
            Representation::Zuir
        );

        assert!(permits_translation(descriptor.guarantee()));
    }

    #[test]
    fn lossy_translation_error_is_structured() {
        let descriptor = AdapterDescriptor::new(
            Representation::Quantum,
            Representation::Zuir,
            ConversionGuarantee::PotentiallyLossy,
        );

        let error: TranslationError<std::io::Error> =
            TranslationError::LossyConversionNotPermitted {
                descriptor,
            };

        assert!(error.to_string().contains("potentially lossy"));
    }
}
