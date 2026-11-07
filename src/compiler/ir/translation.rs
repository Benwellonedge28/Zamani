
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Compiler — IR Translation
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
//! - Lowering context:
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
//! - Admission of declared conversion guarantees.
//! - Rejection of unverified and malformed translation requests.
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
//! Identity and documented semantic-preservation conversions are
//! accepted by the normal translation API.
//!
//! Potentially lossy conversions are rejected.
//!
//! A descriptor is metadata. It does not prove that an adapter exists
//! or that its implementation actually preserves semantics.
//!
//! The authoritative adapter implementation and its conformance tests
//! remain responsible for establishing that guarantee.
//!
//! ## Verification
//!
//! Translation requires explicit source-verification evidence.
//!
//! The low-level translation API checks the verification assertion
//! supplied by the canonical legality candidate.
//!
//! The preferred public compiler entry point is:
//!
//! `crate::compiler::ir::lowering::lower_verified_with`
//!
//! That API additionally requires a `VerifiedIrInput`.
//!
//! A successful translation does not automatically establish that
//! the destination representation is verified.
//!
//! The destination must pass its authoritative verifier before it
//! is trusted by subsequent compiler stages.
//!
//! ## Scalability
//!
//! This module introduces no fixed limits on representations, modules,
//! functions, regions, blocks, operations, values, types, qubits,
//! signals, tensor dimensions, tensor rank, devices, processors,
//! distributed participants, or computing domains.
//!
//! The translation coordinator does not duplicate the IR graph.
//! Collection growth, conversion work, and resource consumption
//! belong to the canonical domain implementation and explicit
//! operational budgets.
//!
//! No finite implementation can promise literal infinite resources.
//! Resource exhaustion must be reported, not interpreted as success.
//!
//! ## Grammar boundary
//!
//! Source syntax and parsing remain owned by the grammar and frontend.
//! Source meaning remains owned by semantic analysis.
//!
//! This module accepts established representation and legality contracts.
//! It does not parse grammar productions, infer source semantics,
//! or introduce source-language keywords.
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
//! Verified input:
//!   `src/compiler/ir/input.rs`
//!
//! Compiler lowering:
//!   `src/compiler/ir/lowering.rs`
//!
//! Lowering authority:
//!   `src/zuir/lowering/`
//!
//! Domain-owned conversion implementations must be supplied by their
//! respective subsystems and invoked through this boundary.
//!
//! ## Freeze criteria
//!
//! - All conversions delegate to authoritative domain implementations.
//! - Potentially lossy conversions are rejected.
//! - Unverified source candidates are rejected.
//! - Malformed descriptors are rejected.
//! - Non-legal lowering outcomes cannot be mistaken for completed
//!   translation.
//! - Backend and legality errors remain inspectable.
//! - Translation evidence can be preserved when consuming results.
//! - No IR, verifier, or domain conversion algorithm is duplicated.
//! - No arbitrary hardware ceilings or unsafe code are introduced.
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
/// grammar version, semantic-model version, ZUIR version,
/// domain IR versions, and backend ABI version.
///
/// Version 2 adds explicit descriptor validation and source-verification
/// admission errors, and provides evidence-preserving outcome extraction.
pub const IR_TRANSLATION_API_VERSION: u16 = 2;

/// Return the compiler IR translation API version.
#[must_use]
pub const fn api_version() -> u16 {
    IR_TRANSLATION_API_VERSION
}

// =============================================================================
// TRANSLATION ERROR
// =============================================================================

/// Errors produced by compiler-facing IR translation orchestration.
#[derive(Debug)]
#[non_exhaustive]
pub enum TranslationError<E> {
    /// The supplied adapter descriptor is malformed.
    ///
    /// A malformed descriptor cannot identify a valid representation
    /// conversion request.
    InvalidDescriptor {
        /// The descriptor rejected by the translation boundary.
        descriptor: AdapterDescriptor,
    },

    /// The requested conversion is potentially lossy.
    ///
    /// Potentially lossy conversions are not accepted by the normal
    /// translation API because they could change program meaning.
    LossyConversionNotPermitted {
        /// The conversion descriptor that was rejected.
        descriptor: AdapterDescriptor,
    },

    /// The candidate does not explicitly establish that its source
    /// representation has passed the required verification process.
    ///
    /// Translation must not infer verification from an entity identity,
    /// a descriptor, or the existence of a lowering backend.
    SourceVerificationNotEstablished,

    /// The canonical lowering subsystem rejected or could not
    /// complete the translation.
    Lowering(LoweringApiError<E>),
}

impl<E: fmt::Display> fmt::Display for TranslationError<E> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidDescriptor { descriptor } => {
                write!(
                    formatter,
                    "invalid IR translation descriptor: {} -> {}",
                    descriptor.source().as_str(),
                    descriptor.destination().as_str(),
                )
            }

            Self::LossyConversionNotPermitted { descriptor } => {
                write!(
                    formatter,
                    "potentially lossy IR translation is not permitted: {} -> {}",
                    descriptor.source().as_str(),
                    descriptor.destination().as_str(),
                )
            }

            Self::SourceVerificationNotEstablished => {
                write!(
                    formatter,
                    "IR translation requires explicitly verified source semantics"
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
            Self::InvalidDescriptor { .. } => None,
            Self::LossyConversionNotPermitted { .. } => None,
            Self::SourceVerificationNotEstablished => None,
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
/// Callers must inspect [`Self::is_translated`] before consuming
/// an artifact.
///
/// A translated artifact is not automatically a verified destination
/// representation or a deployable target artifact.
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

    /// Return the canonical legality report.
    ///
    /// This report describes the supplied conversion evidence.
    /// It is not a target-feasibility or execution report.
    #[must_use]
    pub const fn legality(
        &self,
    ) -> &crate::zuir::lowering::legality::LegalityReport {
        &self.lowering.legality
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
    ///
    /// This convenience method discards the descriptor, legality report,
    /// and destination mappings. Prefer [`Self::into_parts`] when those
    /// records are needed for diagnostics, provenance, or audit.
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

    /// Consume the outcome while preserving all translation evidence.
    ///
    /// Returns the original descriptor and canonical lowering outcome.
    /// No report, artifact, or destination mapping is reconstructed.
    #[must_use]
    pub fn into_parts(
        self,
    ) -> (
        AdapterDescriptor,
        LoweringOutcome<T>,
    ) {
        (self.descriptor, self.lowering)
    }
}

// =============================================================================
// TRANSLATION POLICY
// =============================================================================

/// Return whether a conversion guarantee is accepted by the normal
/// compiler translation API.
///
/// Identity and documented semantic-preservation conversions are
/// accepted.
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

/// Validate the structural form of a translation descriptor.
///
/// This checks that the descriptor has non-empty representation
/// identifiers.
///
/// It does not establish that an adapter implementation exists,
/// that the conversion is registered, or that the declared
/// semantic-preservation guarantee is true.
///
/// Adapter existence and implementation correctness remain the
/// responsibility of the authoritative adapter and domain owners.
#[must_use]
pub const fn is_valid_descriptor(
    descriptor: AdapterDescriptor,
) -> bool {
    descriptor.is_well_formed()
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
/// 1. Validates the adapter descriptor.
/// 2. Checks the adapter's declared conversion guarantee.
/// 3. Rejects potentially lossy conversion.
/// 4. Requires explicit source-verification evidence.
/// 5. Delegates legality analysis to the canonical ZUIR lowering API.
/// 6. Invokes the backend only when legality permits direct lowering.
/// 7. Preserves the canonical legality report and destination mappings.
///
/// The backend must implement the existing `LoweringBackend` contract.
/// It owns the actual representation conversion algorithm.
///
/// # Important
///
/// This function does not create a converter for a representation pair.
/// It coordinates a converter supplied by the owning domain subsystem.
///
/// The descriptor is metadata and does not independently prove that
/// the backend implements the named representation conversion.
///
/// This function does not independently run whole-program verification.
/// The candidate's verification assertion must originate from a trusted
/// compiler integration path.
///
/// For the stronger public compiler admission boundary, use
/// `crate::compiler::ir::lowering::lower_verified_with`.
///
/// # Errors
///
/// Returns `TranslationError::InvalidDescriptor` when the descriptor
/// contains malformed representation identifiers.
///
/// Returns `TranslationError::LossyConversionNotPermitted` when the
/// descriptor declares a potentially lossy conversion.
///
/// Returns `TranslationError::SourceVerificationNotEstablished` when
/// the candidate does not explicitly establish source verification.
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
    if !is_valid_descriptor(descriptor) {
        return Err(
            TranslationError::InvalidDescriptor {
                descriptor,
            },
        );
    }

    if !permits_translation(descriptor.guarantee()) {
        return Err(
            TranslationError::LossyConversionNotPermitted {
                descriptor,
            },
        );
    }

    if !candidate.source_verified {
        return Err(
            TranslationError::SourceVerificationNotEstablished,
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
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::compiler::ir::adapters::Representation;

    use crate::zuir::lowering::context::{
        EntityKey,
        EntityKind,
        LoweringPolicy,
    };

    use crate::zuir::lowering::api::{
        BackendOutput,
        LoweringBackend,
    };

    use crate::zuir::lowering::legality::{
        DestinationSemantics,
        LegalityCandidate,
        OperationSemantics,
    };

    use std::collections::BTreeSet;
    use std::io;

    struct TestBackend {
        called: bool,
    }

    impl LoweringBackend for TestBackend {
        type Artifact = String;
        type Error = io::Error;

        fn lower(
            &mut self,
            _source: &EntityKey,
            _context: &LoweringContext,
        ) -> Result<BackendOutput<Self::Artifact>, Self::Error> {
            self.called = true;

            Ok(BackendOutput::new(
                "translated".to_owned(),
                Vec::new(),
            ))
        }
    }

    fn descriptor(
        guarantee: ConversionGuarantee,
    ) -> AdapterDescriptor {
        AdapterDescriptor::new(
            Representation::Zcir,
            Representation::Zuir,
            guarantee,
        )
    }

    fn source() -> EntityKey {
        EntityKey::new(
            EntityKind::Operation,
            "test",
            "source-operation",
        )
        .expect("valid source identity")
    }

    fn candidate(
        source_verified: bool,
    ) -> LegalityCandidate {
        let source_semantics = OperationSemantics::new(
            "test.source",
            BTreeSet::new(),
            BTreeSet::new(),
            BTreeSet::new(),
            false,
        )
        .expect("valid source semantics");

        let destination_semantics = DestinationSemantics::new(
            "test.destination",
            BTreeSet::new(),
            BTreeSet::new(),
            false,
            true,
        )
        .expect("valid destination semantics");

        LegalityCandidate::new(
            source(),
            Some(source_semantics),
            Some(destination_semantics),
            BTreeSet::new(),
            Vec::new(),
            Vec::new(),
            None,
            source_verified,
        )
        .expect("valid legality candidate")
    }

    #[test]
    fn translation_api_version_is_available() {
        assert_eq!(api_version(), IR_TRANSLATION_API_VERSION);
        assert_eq!(IR_TRANSLATION_API_VERSION, 2);
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
    fn valid_descriptor_preserves_representation_direction() {
        let descriptor = descriptor(
            ConversionGuarantee::Identity
        );

        assert!(is_valid_descriptor(descriptor));

        assert_eq!(
            descriptor.source(),
            Representation::Zcir
        );

        assert_eq!(
            descriptor.destination(),
            Representation::Zuir
        );
    }

    #[test]
    fn empty_extension_identifier_is_rejected() {
        let descriptor = AdapterDescriptor::new(
            Representation::Extension(""),
            Representation::Zuir,
            ConversionGuarantee::VerifiedSemanticPreservation,
        );

        assert!(!is_valid_descriptor(descriptor));
    }

    #[test]
    fn lossy_translation_error_is_structured() {
        let descriptor = descriptor(
            ConversionGuarantee::PotentiallyLossy
        );

        let error: TranslationError<io::Error> =
            TranslationError::LossyConversionNotPermitted {
                descriptor,
            };

        assert!(error.to_string().contains("potentially lossy"));
    }

    #[test]
    fn unverified_source_is_rejected_before_backend_invocation() {
        let mut context = LoweringContext::new(
            LoweringPolicy::default(),
        );

        let mut backend = TestBackend {
            called: false,
        };

        let result = translate_with(
            descriptor(ConversionGuarantee::Identity),
            &mut context,
            candidate(false),
            &mut backend,
        );

        assert!(matches!(
            result,
            Err(
                TranslationError::SourceVerificationNotEstablished
            )
        ));

        assert!(!backend.called);
    }

    #[test]
    fn malformed_descriptor_is_rejected_before_backend_invocation() {
        let mut context = LoweringContext::new(
            LoweringPolicy::default(),
        );

        let mut backend = TestBackend {
            called: false,
        };

        let malformed = AdapterDescriptor::new(
            Representation::Extension(""),
            Representation::Zuir,
            ConversionGuarantee::Identity,
        );

        let result = translate_with(
            malformed,
            &mut context,
            candidate(true),
            &mut backend,
        );

        assert!(matches!(
            result,
            Err(TranslationError::InvalidDescriptor { .. })
        ));

        assert!(!backend.called);
    }

    #[test]
    fn lossy_conversion_is_rejected_before_backend_invocation() {
        let mut context = LoweringContext::new(
            LoweringPolicy::default(),
        );

        let mut backend = TestBackend {
            called: false,
        };

        let result = translate_with(
            descriptor(ConversionGuarantee::PotentiallyLossy),
            &mut context,
            candidate(true),
            &mut backend,
        );

        assert!(matches!(
            result,
            Err(
                TranslationError::LossyConversionNotPermitted { .. }
            )
        ));

        assert!(!backend.called);
    }

    #[test]
    fn translation_outcome_preserves_descriptor_and_lowering_report() {
        let mut context = LoweringContext::new(
            LoweringPolicy::default(),
        );

        let mut backend = TestBackend {
            called: false,
        };

        let result = translate_with(
            descriptor(ConversionGuarantee::Identity),
            &mut context,
            candidate(true),
            &mut backend,
        )
        .expect("translation request is admitted");

        assert!(backend.called);

        // A backend artifact is produced only when the canonical
        // legality subsystem authorizes direct lowering.
        assert!(result.is_translated());

        assert_eq!(
            result.artifact().map(String::as_str),
            Some("translated")
        );

        assert_eq!(
            result.descriptor().source(),
            Representation::Zcir
        );

        let (descriptor, lowering) = result.into_parts();

        assert_eq!(
            descriptor.destination(),
            Representation::Zuir
        );

        assert!(lowering.is_lowered());
        assert!(lowering.artifact.is_some());
    }

    #[test]
    fn deferred_legality_does_not_produce_an_artifact() {
        let mut context = LoweringContext::new(
            LoweringPolicy::default(),
        );

        let mut backend = TestBackend {
            called: false,
        };

        let deferred = LegalityCandidate::new(
            source(),
            None,
            None,
            BTreeSet::new(),
            Vec::new(),
            Vec::new(),
            None,
            true,
        )
        .expect("valid deferred candidate");

        let result = translate_with(
            descriptor(ConversionGuarantee::Identity),
            &mut context,
            deferred,
            &mut backend,
        )
        .expect("deferred legality is a report");

        assert!(!backend.called);
        assert!(!result.is_translated());
        assert!(result.artifact().is_none());
    }
}
