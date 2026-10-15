
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # ZUIR Lowering API
//!
//! Path: `src/zuir/lowering/api.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: `unsafe` Rust is prohibited.
//!
//! ## PURPOSE
//!
//! Provide the stable orchestration boundary for lowering verified ZUIR
//! entities into domain-specific or target-specific representations.
//!
//! ## AUTHORITY
//!
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/lowering/context.rs`
//! - `src/zuir/lowering/legality.rs`
//! - `src/zuir/lowering/diagnostics.rs`
//! - `src/zuir/lowering/provenance.rs`
//! - `src/zuir/source_map.rs`
//! - `src/zuir/verify/`
//! - `src/zuir/dialect/`
//! - `src/zuir/ir/`
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/compilation-model.md`
//! - `grammar/specification/execution-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/poco-reaf.md`
//!
//! ## OWNS
//!
//! - The public lowering invocation contract.
//! - Orchestration of legality analysis and backend invocation.
//! - The result contract distinguishing authorization from execution.
//! - Registration of successful source-to-destination entity mappings.
//! - Structured propagation of legality, context, and backend failures.
//!
//! ## DOES_NOT_OWN
//!
//! - Source parsing, AST construction, or semantic analysis.
//! - Canonical IR definitions or whole-program verification.
//! - Dialect registration or operation semantics.
//! - Source-map identity, source spans, or provenance conversion.
//! - Target discovery, physical resource allocation, routing, scheduling,
//!   synthesis, code generation, or runtime execution.
//! - The implementation of domain-specific conversion algorithms.
//!
//! ## INTEGRATION
//!
//! Callers must provide a `LegalityCandidate` built from authoritative
//! semantic, verification, capability, resource, and dialect evidence.
//! `analyze_legality` remains the authority for direct-lowering decisions.
//! A backend runs only when the resulting status is `Legal`.
//!
//! Backends receive the context immutably. They return destination entity
//! identities instead of mutating the shared mapping table themselves.
//! This API validates those identities and records mappings only after the
//! backend reports success. Source-map provenance remains the responsibility
//! of the provenance adapter.
//!
//! ## SEMANTIC SAFETY
//!
//! A `Legal` result authorizes the proposed conversion under the supplied
//! contracts; it does not prove target feasibility or execution success.
//! Conditional, deferred, expansion-required, and unsupported conversions
//! do not invoke the direct-lowering backend.
//!
//! This API does not independently run whole-program verification.
//! The caller must establish `source_verified` from the canonical verifier
//! when the lowering policy requires verified source input.
//!
//! ## SCALABILITY AND POCO-REAF
//!
//! Collections grow dynamically. No universal hardware, qubit, register,
//! tensor-rank, device, operation-count, or node-count limit is imposed.
//! The legality layer charges its own work according to its contract.
//! This API does not claim infinite memory, time, or physical resources.
//!
//! ## FREEZE CRITERIA
//!
//! - Only an explicit `Legal` result invokes the backend.
//! - Non-legal outcomes cannot be mistaken for successful lowering.
//! - Backend failures do not register destination mappings.
//! - Mapping categories are checked before mapping mutation.
//! - Duplicate mappings remain idempotent.
//! - Errors preserve their underlying failure category.
//! - Tests cover legal, deferred, mapping, and backend-failure paths.
//! - The parent module exports this API deliberately.
//! - Formatting, compilation, linting, and integration tests pass.

use core::fmt;
use std::error::Error;

use crate::zuir::lowering::context::{
    EntityKey,
    EntityKind,
    LoweringContext,
    LoweringContextError,
};
use crate::zuir::lowering::legality::{
    analyze_legality,
    LegalityCandidate,
    LegalityError,
    LegalityReport,
    LegalityStatus,
};

/// Result alias for the public lowering orchestration API.
pub type LoweringApiResult<T, E> = Result<T, LoweringApiError<E>>;

/// Output returned by a backend after a successful conversion.
///
/// `destinations` identifies the destination entities created or reused by
/// the conversion. The API sorts and deduplicates these identities before
/// recording them in the shared lowering context.
///
/// An empty destination list is valid for conversions that intentionally
/// produce no corresponding destination entity. The artifact itself remains
/// owned by the backend's output type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackendOutput<T> {
    /// The artifact produced by the backend.
    pub artifact: T,

    /// Destination entities associated with the source entity.
    pub destinations: Vec<EntityKey>,
}

impl<T> BackendOutput<T> {
    /// Construct a backend output.
    ///
    /// Destination categories are checked by [`lower_with`] before mappings
    /// are recorded.
    #[must_use]
    pub fn new(artifact: T, destinations: Vec<EntityKey>) -> Self {
        Self {
            artifact,
            destinations,
        }
    }
}

/// Contract implemented by a domain-specific lowering backend.
///
/// Implementations should perform conversion only; legality analysis and
/// source-to-destination mapping registration belong to this API.
///
/// The backend receives an immutable view of the lowering context. It may
/// inspect already-recorded decisions and requirements but cannot mutate
/// context bookkeeping through this reference.
///
/// Backend implementations must not assume that the current target is the
/// only target on which the source program may be realized. Target-specific
/// assumptions must be justified by explicit target contracts and must not
/// change source semantics silently.
pub trait LoweringBackend {
    /// Type of the artifact produced by this backend.
    type Artifact;

    /// Error type returned by this backend.
    type Error: Error + 'static;

    /// Lower one source entity.
    ///
    /// This method is called only after legality analysis returns `Legal`.
    /// The backend must return an error if it cannot complete the conversion.
    ///
    /// The method must not report success while silently dropping required
    /// effects, ownership rules, resource constraints, numerical guarantees,
    /// ordering, concurrency semantics, quantum semantics, or HDL semantics.
    fn lower(
        &mut self,
        source: &EntityKey,
        context: &LoweringContext,
    ) -> Result<BackendOutput<Self::Artifact>, Self::Error>;
}

/// Result of one lowering invocation.
///
/// `artifact` is `Some` only if legality analysis authorized direct lowering
/// and the backend completed successfully. A non-legal result is returned
/// as a report with no artifact; it is not represented as a successful
/// conversion.
///
/// The legality report and artifact are deliberately separate. A legal
/// conversion may still fail during backend execution, allocation, or
/// destination construction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoweringOutcome<T> {
    /// The authoritative legality report for the attempted conversion.
    pub legality: LegalityReport,

    /// The produced artifact, if conversion completed successfully.
    pub artifact: Option<T>,

    /// Destination entities registered for this source entity.
    ///
    /// This list is empty when direct lowering was not authorized or when
    /// the backend failed.
    pub destinations: Vec<EntityKey>,
}

impl<T> LoweringOutcome<T> {
    /// Return whether an artifact was successfully produced.
    #[must_use]
    pub fn is_lowered(&self) -> bool {
        self.artifact.is_some()
    }

    /// Consume the outcome and return the artifact, if present.
    #[must_use]
    pub fn into_artifact(self) -> Option<T> {
        self.artifact
    }
}

/// Failure while orchestrating a lowering invocation.
///
/// Backend errors retain their original type so callers can inspect or
/// report backend-specific failures without parsing an error message.
#[derive(Debug)]
pub enum LoweringApiError<E> {
    /// The legality analysis rejected malformed input or failed to complete.
    Legality(LegalityError),

    /// The backend failed after direct lowering was authorized.
    Backend(E),

    /// The shared lowering context rejected a mapping.
    Context(LoweringContextError),
}

impl<E: fmt::Display> fmt::Display for LoweringApiError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Legality(error) => {
                write!(f, "lowering legality analysis failed: {error}")
            }
            Self::Backend(error) => {
                write!(f, "lowering backend failed: {error}")
            }
            Self::Context(error) => {
                write!(f, "lowering context rejected an entity mapping: {error}")
            }
        }
    }
}

impl<E> Error for LoweringApiError<E>
where
    E: Error + 'static,
{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Legality(error) => Some(error),
            Self::Backend(error) => Some(error),
            Self::Context(error) => Some(error),
        }
    }
}

/// Orchestrate legality analysis and, when authorized, one backend conversion.
///
/// # Behavior
///
/// 1. Analyze the supplied candidate using the canonical legality layer.
/// 2. Return without invoking the backend unless the status is `Legal`.
/// 3. Invoke the backend using the source identity and read-only context.
/// 4. Sort and deduplicate the returned destination identities.
/// 5. Validate every destination category before changing the mapping table.
/// 6. Record the source-to-destination mappings.
/// 7. Return the legality report, artifact, and registered destinations.
///
/// The backend is not invoked for conditional, deferred, expansion-required,
/// or unsupported outcomes. Those cases return `Ok` with an outcome whose
/// `artifact` is `None`; the caller must inspect `outcome.legality.status`.
///
/// A backend failure returns `Err` and does not register destination mappings.
/// The backend itself is responsible for not publishing partial artifacts
/// outside its returned output before it reports success.
///
/// # Verification
///
/// This function does not run the full ZUIR verifier because full verification
/// requires a consistent program snapshot and all associated IR, type,
/// ownership, resource, and dialect inputs. Callers must verify the relevant
/// source snapshot before setting `candidate.source_verified = true`.
///
/// # Errors
///
/// Returns an error if legality analysis fails, the backend fails, or the
/// lowering context rejects a mapping.
pub fn lower_with<B>(
    context: &mut LoweringContext,
    candidate: LegalityCandidate,
    backend: &mut B,
) -> LoweringApiResult<LoweringOutcome<B::Artifact>, B::Error>
where
    B: LoweringBackend,
{
    let source = candidate.source.clone();

    let legality = analyze_legality(context, candidate)
        .map_err(LoweringApiError::Legality)?;

    if !matches!(&legality.status, LegalityStatus::Legal) {
        return Ok(LoweringOutcome {
            legality,
            artifact: None,
            destinations: Vec::new(),
        });
    }

    let mut backend_output = backend
        .lower(&source, context)
        .map_err(LoweringApiError::Backend)?;

    // Normalize the output before validating or recording any mappings.
    // EntityKey has a deterministic total ordering.
    backend_output.destinations.sort();
    backend_output.destinations.dedup();

    // Validate the complete mapping set first. This prevents a category
    // mismatch in a later destination from leaving earlier mappings behind.
    for destination in &backend_output.destinations {
        if source.kind() != destination.kind() {
            return Err(LoweringApiError::Context(
                LoweringContextError::EntityKindMismatch {
                    source: source.kind().clone(),
                    destination: destination.kind().clone(),
                },
            ));
        }
    }

    // After the validation pass, all category checks performed by
    // record_mapping are known to succeed. Duplicate mappings are idempotent.
    for destination in &backend_output.destinations {
        context
            .record_mapping(source.clone(), destination.clone())
            .map_err(LoweringApiError::Context)?;
    }

    Ok(LoweringOutcome {
        legality,
        artifact: Some(backend_output.artifact),
        destinations: backend_output.destinations,
    })
}

/// Return whether a legality report authorizes direct lowering.
///
/// This convenience function intentionally checks only the legality result.
/// It does not imply target feasibility, backend success, or runtime success.
#[must_use]
pub fn permits_direct_lowering(report: &LegalityReport) -> bool {
    matches!(&report.status, LegalityStatus::Legal)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zuir::lowering::context::LoweringPolicy;
    use crate::zuir::lowering::legality::{
        DestinationSemantics,
        OperationSemantics,
    };
    use std::collections::BTreeSet;
    use std::io;

    struct TestBackend {
        called: bool,
        destinations: Vec<EntityKey>,
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
                "lowered-artifact".to_owned(),
                self.destinations.clone(),
            ))
        }
    }

    fn key(kind: EntityKind, namespace: &str, value: &str) -> EntityKey {
        EntityKey::new(kind, namespace, value).expect("valid test key")
    }

    fn legal_candidate(source: EntityKey) -> LegalityCandidate {
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
            source,
            Some(source_semantics),
            Some(destination_semantics),
            BTreeSet::new(),
            Vec::new(),
            Vec::new(),
            None,
            true,
        )
        .expect("valid legality candidate")
    }

    #[test]
    fn legal_conversion_invokes_backend_and_records_mapping() {
        let source = key(EntityKind::Operation, "zui", "op-1");
        let destination = key(EntityKind::Operation, "classical", "op-2");

        let mut context = LoweringContext::new(LoweringPolicy::default());
        let mut backend = TestBackend {
            called: false,
            destinations: vec![destination.clone()],
        };

        let outcome = lower_with(
            &mut context,
            legal_candidate(source.clone()),
            &mut backend,
        )
        .expect("lowering succeeds");

        assert!(backend.called);
        assert!(outcome.is_lowered());
        assert_eq!(outcome.artifact.as_deref(), Some("lowered-artifact"));
        assert_eq!(
            context
                .mapped_entities(&source)
                .expect("mapping exists")
                .len(),
            1
        );
        assert_eq!(outcome.destinations, vec![destination]);
    }

    #[test]
    fn deferred_legality_does_not_invoke_backend() {
        let source = key(EntityKind::Operation, "zui", "op-1");

        let candidate = LegalityCandidate::new(
            source,
            None,
            None,
            BTreeSet::new(),
            Vec::new(),
            Vec::new(),
            None,
            true,
        )
        .expect("candidate is structurally valid");

        let mut context = LoweringContext::new(LoweringPolicy::default());
        let mut backend = TestBackend {
            called: false,
            destinations: Vec::new(),
        };

        let outcome = lower_with(&mut context, candidate, &mut backend)
            .expect("deferred legality is a report, not a backend error");

        assert!(!backend.called);
        assert!(!outcome.is_lowered());
        assert!(outcome.destinations.is_empty());
        assert!(!permits_direct_lowering(&outcome.legality));
    }

    #[test]
    fn mismatched_destination_kind_is_rejected() {
        let source = key(EntityKind::Operation, "zui", "op-1");
        let wrong_destination = key(EntityKind::Value, "target", "value-1");

        let mut context = LoweringContext::new(LoweringPolicy::default());
        let mut backend = TestBackend {
            called: false,
            destinations: vec![wrong_destination],
        };

        let result = lower_with(
            &mut context,
            legal_candidate(source.clone()),
            &mut backend,
        );

        assert!(matches!(
            result,
            Err(LoweringApiError::Context(
                LoweringContextError::EntityKindMismatch { .. }
            ))
        ));
        assert!(context.mapped_entities(&source).is_none());
    }

    #[test]
    fn destination_mappings_are_sorted_and_deduplicated() {
        let source = key(EntityKind::Operation, "zui", "op-1");
        let destination_b = key(EntityKind::Operation, "target", "b");
        let destination_a = key(EntityKind::Operation, "target", "a");

        let mut context = LoweringContext::new(LoweringPolicy::default());
        let mut backend = TestBackend {
            called: false,
            destinations: vec![
                destination_b.clone(),
                destination_a.clone(),
                destination_b,
            ],
        };

        let outcome = lower_with(
            &mut context,
            legal_candidate(source.clone()),
            &mut backend,
        )
        .expect("lowering succeeds");

        assert_eq!(
            outcome.destinations,
            vec![destination_a, destination_b]
        );
        assert_eq!(
            context
                .mapped_entities(&source)
                .expect("mappings exist")
                .len(),
            2
        );
    }
}
