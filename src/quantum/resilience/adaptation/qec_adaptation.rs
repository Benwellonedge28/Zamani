//! Zamani Quantum Resilience — QEC Adaptation
//!
//! Path:
//!     src/quantum/resilience/adaptation/qec_adaptation.rs
//!
//! Purpose:
//!     Production-grade resilience adapter for changing quantum error
//!     correction configuration while preserving the logical computation.
//!
//! ============================================================================
//! Architectural responsibility
//! ============================================================================
//!
//! This module translates the canonical resilience action:
//!
//!     ActionKind::AdaptQec
//!
//! into a request to the authoritative Quantum Error Correction subsystem.
//!
//! This module DOES:
//!
//! - implement `AdaptationAdapter`;
//! - accept only `ActionKind::AdaptQec`;
//! - enforce transactional adaptation semantics;
//! - enforce stale execution-generation protection;
//! - enforce stale semantic-revision protection;
//! - enforce deterministic-mode requirements;
//! - delegate QEC configuration selection to an injected QEC service;
//! - preserve provider independence;
//! - expose adapter capabilities;
//! - create opaque QEC adaptation candidates;
//! - validate candidates before commit;
//! - prevent accidental partial commits;
//! - support whole-computation and scoped adaptation;
//! - support arbitrarily large finite QEC configurations;
//! - avoid fixed qubit counts;
//! - avoid fixed code distances;
//! - avoid fixed decoder names;
//! - avoid provider-specific logic;
//! - avoid hidden global mutable state;
//! - avoid retry loops;
//! - avoid unsafe code.
//!
//! This module DOES NOT:
//!
//! - implement a QEC code;
//! - implement encoding;
//! - implement decoding;
//! - implement syndrome extraction;
//! - implement a decoder;
//! - choose a hard-coded code distance;
//! - allocate physical qubits;
//! - allocate ancillas;
//! - modify hardware topology;
//! - perform routing;
//! - perform scheduling;
//! - compile quantum circuits;
//! - optimize circuits;
//! - communicate directly with a QPU;
//! - authorize QEC privileges;
//! - replace `quantum::error_correction`;
//! - replace `quantum::ir`;
//! - decide whether QEC adaptation is policy-safe;
//! - perform final semantic verification.
//!
//! ============================================================================
//! Architectural boundary
//! ============================================================================
//!
//! ```text
//!                    Resilience Planner
//!                           |
//!                           v
//!                    RecoveryAction
//!                    ActionKind::AdaptQec
//!                           |
//!                           v
//!                  AdaptationRequest
//!                           |
//!                           v
//!                    QecAdapter
//!                           |
//!                           v
//!                QecAdaptationService
//!                           |
//!                           v
//!              quantum::error_correction
//!                           |
//!        +------------------+------------------+
//!        |                  |                  |
//!        v                  v                  v
//!   capabilities        configuration       resources
//!        |                  |                  |
//!        +------------------+------------------+
//!                           |
//!                           v
//!                    QEC candidate
//!                           |
//!                           v
//!                    Commit / Verify
//!                           |
//!                           v
//!              resilience::verification
//! ```
//!
//! The architectural boundary is:
//!
//!     resilience decides WHEN QEC adaptation is required;
//!     QEC decides HOW the new QEC configuration is constructed and executed.
//!
//! ============================================================================
//! Write once, scale everywhere
//! ============================================================================
//!
//! No finite machine-size assumption exists in this module.
//!
//! In particular, this file contains no:
//!
//!     MAX_QUBITS
//!     MAX_LOGICAL_QUBITS
//!     MAX_PHYSICAL_QUBITS
//!     MAX_ANCILLAS
//!     MAX_CODE_DISTANCE
//!     MAX_SYNDROME_ROUNDS
//!     DEFAULT_DECODER
//!     DEFAULT_BACKEND
//!     FIXED_RETRY_COUNT
//!
//! Actual limits are supplied by authoritative subsystems:
//!
//!     quantum::error_correction
//!     quantum::hardware
//!     quantum::routing
//!     quantum::scheduling
//!     resilience::policy
//!     resilience::limits
//!     runtime/resource management
//!
//! "Infinity" therefore means that this resilience adapter contributes no
//! artificial finite upper bound. Every concrete execution remains bounded by
//! the resources and capabilities actually available to it.
//!
//! ============================================================================
//! Canonical quantum identity
//! ============================================================================
//!
//! This adapter does not need to manufacture or store qubit identities.
//!
//! The canonical QEC request already carries its execution scope through
//! `AdaptationRequest` and the canonical action model.
//!
//! When a concrete QEC implementation needs qubit identities, it MUST use:
//!
//!     crate::quantum::ir::qubit::QubitId
//!
//! and MUST NOT introduce another QubitId type.
//!
//! This keeps:
//!
//!     quantum::ir::qubit
//!
//! as the canonical qubit identity boundary.
//!
//! ============================================================================
//! QEC ownership
//! ============================================================================
//!
//! The authoritative QEC subsystem is:
//!
//!     crate::quantum::error_correction
//!
//! It owns:
//!
//! - QEC codes;
//! - code parameters;
//! - encoding;
//! - decoding;
//! - syndrome extraction;
//! - QEC configuration;
//! - QEC capability authorization;
//! - QEC resource limits;
//! - QEC backend execution;
//! - hardware QEC adapters;
//! - QEC verification.
//!
//! Resilience owns the decision to request adaptation and the transactional
//! lifecycle around that request.
//!
//! ============================================================================
//! Safety invariant
//! ============================================================================
//!
//! A QEC adaptation candidate MUST NOT be accepted merely because it claims to
//! reduce the error rate.
//!
//! The complete resilience acceptance condition remains:
//!
//!     semantic validity
//!     + policy validity
//!     + capability validity
//!     + resource feasibility
//!     + security authorization
//!     + QEC compatibility
//!     + verification validity
//!
//! Only the surrounding resilience verification layer can finally accept the
//! adapted execution.
//!
//! ============================================================================
//! Determinism
//! ============================================================================
//!
//! This adapter itself is deterministic for equal explicit inputs.
//!
//! The injected QEC service is responsible for deterministic configuration
//! selection when deterministic execution is required.
//!
//! No random identifiers are generated here.
//! No system time is consulted here.
//! No global mutable state is consulted here.
//!
//! ============================================================================
//! Transactional lifecycle
//! ============================================================================
//!
//!     Preflight
//!         |
//!         v
//!     Prepare
//!         |
//!         v
//!     candidate
//!         |
//!         v
//!     Commit
//!         |
//!         v
//!     Verify
//!
//! Preparation MUST NOT mutate the committed QEC execution state.
//!
//! Commit MUST reject a candidate whose:
//!
//! - execution generation changed;
//! - semantic revision changed;
//! - QEC target became incompatible;
//! - resource requirements became unavailable;
//! - authorization became invalid;
//! - candidate identity is invalid.
//!
//! Verification performed here is QEC-adapter-local verification.
//!
//! Final program semantic verification remains owned by:
//!
//!     crate::quantum::resilience::verification
//!
//! ============================================================================
//! Rust compatibility
//! ============================================================================
//!
//! Required:
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021
//! - stable Rust
//! - no nightly features
//! - no unsafe code
//!
//! ============================================================================
//! Integration contract
//! ============================================================================
//!
//! planning/action.rs
//!     |
//!     +--> ActionKind::AdaptQec
//!     |
//!     v
//! adaptation/adapter.rs
//!     |
//!     v
//! qec_adaptation.rs
//!     |
//!     v
//! QecAdaptationService
//!     |
//!     v
//! quantum/error_correction/*
//!
//! The concrete service should normally be implemented at the integration
//! boundary around the existing `quantum::error_correction` subsystem.
//!
//! This file must not be modified merely because a new QEC code, decoder,
//! hardware QEC mechanism, or code family is added.
//!
//! New QEC implementations should be exposed through the injected service.
//!
//! ============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_debug_implementations)]

use std::fmt;
use std::sync::Arc;

use crate::quantum::resilience::adaptation::adapter::{
    AdaptationAdapter,
    AdaptationCandidate,
    AdaptationCapabilities,
    AdaptationOperation,
    AdaptationRequest,
    AdaptationResult,
    AdaptationStatus,
    AdapterId,
    AdapterVersion,
};

use crate::quantum::resilience::errors::{
    ResilienceError,
    ResilienceErrorCode,
    ResilienceResult,
};

use crate::quantum::resilience::planning::action::ActionKind;

// ============================================================================
// Stable identity
// ============================================================================

/// Stable semantic identifier for the QEC adaptation adapter.
pub const QEC_ADAPTATION_ADAPTER_ID: &str =
    "zamani.quantum.resilience.adaptation.qec";

/// Implementation version of this adapter.
pub const QEC_ADAPTATION_ADAPTER_VERSION: AdapterVersion =
    AdapterVersion::new(1, 0, 0);

/// Stable schema identifier for the QEC adaptation boundary.
pub const QEC_ADAPTATION_SCHEMA_ID: &str =
    "zamani.quantum.resilience.adaptation.qec";

/// Semantic schema version.
pub const QEC_ADAPTATION_SCHEMA_VERSION: u16 = 1;

// ============================================================================
// Supported action
// ============================================================================

/// The QEC adapter intentionally supports exactly one canonical action.
///
/// This prevents accidental dispatch of unrelated adaptation operations.
static SUPPORTED_ACTIONS: [ActionKind; 1] = [ActionKind::AdaptQec];

// ============================================================================
// Opaque QEC candidate identity
// ============================================================================

/// Opaque identity of a prepared QEC adaptation.
///
/// The resilience layer deliberately does not know whether the QEC subsystem
/// represents a candidate using:
///
/// - a code-family identifier;
/// - a code-distance configuration;
/// - a decoder configuration;
/// - a logical-layout artifact;
/// - a syndrome-extraction plan;
/// - a hardware-QEC program;
/// - a complete fault-tolerant execution configuration.
///
/// The authoritative QEC subsystem owns that representation.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct QecAdaptationCandidateId(String);

impl QecAdaptationCandidateId {
    /// Creates a validated candidate identity.
    ///
    /// Empty identities are forbidden because they cannot safely participate
    /// in transactional commit or provenance.
    pub fn new(value: impl Into<String>) -> ResilienceResult<Self> {
        let value = value.into();

        if value.is_empty() {
            return Err(ResilienceError::new(
                ResilienceErrorCode::InvalidIdentifier,
            ));
        }

        Ok(Self(value))
    }

    /// Returns the opaque candidate identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the candidate identity.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for QecAdaptationCandidateId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// ============================================================================
// QEC service contract
// ============================================================================

/// Authoritative QEC adaptation service.
///
/// This trait is the integration boundary between resilience and the existing
/// `quantum::error_correction` subsystem.
///
/// It is deliberately not a QEC algorithm.
///
/// A production implementation should delegate into the repository's existing
/// QEC configuration, capability, resource, backend and verification contracts.
///
/// # Required properties
///
/// Implementations MUST:
///
/// - accept only validated adaptation requests;
/// - preserve the logical computation;
/// - validate QEC capability requirements;
/// - validate resource requirements;
/// - honor QEC authorization;
/// - honor deterministic execution requirements;
/// - reject stale execution state;
/// - reject stale semantic state;
/// - reject incompatible QEC configurations;
/// - return an opaque immutable candidate identity;
/// - never report a partial successful preparation;
/// - never silently change the target execution environment;
/// - never invent physical resources;
/// - never assume a fixed number of qubits;
/// - never assume a fixed code distance;
/// - never assume a fixed decoder;
/// - never assume a fixed number of syndrome rounds.
///
/// # Canonical QEC integration
///
/// The implementation should consume the existing:
///
///     crate::quantum::error_correction
///
/// contracts rather than creating a second QEC abstraction.
///
/// Where physical or logical qubit identities are needed, implementations
/// should use:
///
///     crate::quantum::ir::qubit::QubitId
///
/// # Security
///
/// The service is not allowed to treat the possession of a candidate identity
/// as authorization. QEC capability authorization remains owned by the QEC
/// capability subsystem and the surrounding resilience security policy.
pub trait QecAdaptationService: Send + Sync + fmt::Debug {
    /// Validate the QEC adaptation request without changing committed state.
    fn preflight(
        &self,
        request: &AdaptationRequest,
    ) -> ResilienceResult<()>;

    /// Prepare an immutable QEC adaptation candidate.
    ///
    /// Preparation MUST NOT commit the candidate.
    fn prepare(
        &self,
        request: &AdaptationRequest,
    ) -> ResilienceResult<QecAdaptationCandidateId>;

    /// Commit a previously prepared QEC candidate.
    ///
    /// Returns `true` only when the candidate was actually committed.
    ///
    /// Implementations MUST reject stale or incompatible candidates.
    fn commit(
        &self,
        request: &AdaptationRequest,
        candidate: &QecAdaptationCandidateId,
    ) -> ResilienceResult<bool>;

    /// Verify QEC-local invariants after commit.
    ///
    /// This does not replace final resilience semantic verification.
    fn verify(
        &self,
        request: &AdaptationRequest,
        candidate: &QecAdaptationCandidateId,
    ) -> ResilienceResult<bool>;

    /// Whether equal explicit inputs produce deterministic QEC adaptation.
    ///
    /// The default is conservative: deterministic.
    ///
    /// A nondeterministic implementation MUST override this method and return
    /// `false`.
    fn deterministic(&self) -> bool {
        true
    }
}

// ============================================================================
// Adapter
// ============================================================================

/// Production QEC adaptation adapter.
///
/// The concrete QEC implementation is injected through `Q`.
///
/// This prevents the resilience subsystem from becoming coupled to:
///
/// - one QEC code;
/// - one decoder;
/// - one hardware architecture;
/// - one QPU;
/// - one provider;
/// - one fault-tolerant strategy.
#[derive(Clone)]
pub struct QecAdaptationAdapter<Q>
where
    Q: QecAdaptationService,
{
    id: AdapterId,
    qec: Arc<Q>,
    capabilities: AdaptationCapabilities,
}

impl<Q> fmt::Debug for QecAdaptationAdapter<Q>
where
    Q: QecAdaptationService,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("QecAdaptationAdapter")
            .field("id", &self.id)
            .field("capabilities", &self.capabilities)
            .finish()
    }
}

impl<Q> QecAdaptationAdapter<Q>
where
    Q: QecAdaptationService,
{
    /// Creates a QEC adaptation adapter.
    ///
    /// The QEC service is shared through `Arc` so the adapter remains cheap to
    /// clone and safe to register in concurrent adapter registries.
    pub fn new(qec: Arc<Q>) -> ResilienceResult<Self> {
        let id = AdapterId::new(QEC_ADAPTATION_ADAPTER_ID)?;

        let capabilities = AdaptationCapabilities::new(
            true,  // prepare
            true,  // commit
            true,  // preflight
            qec.deterministic(),
            true,  // scoped
            true,  // partial
            false, // reversible is not assumed
        );

        Ok(Self {
            id,
            qec,
            capabilities,
        })
    }

    /// Returns the injected QEC service.
    #[must_use]
    pub fn qec_service(&self) -> &Arc<Q> {
        &self.qec
    }

    /// Returns the adapter identity.
    #[must_use]
    pub fn adapter_id(&self) -> &AdapterId {
        &self.id
    }

    /// Validates that a request targets QEC adaptation.
    fn validate_request(
        &self,
        request: &AdaptationRequest,
    ) -> ResilienceResult<()> {
        if request.action_kind() != ActionKind::AdaptQec {
            return Err(ResilienceError::new(
                ResilienceErrorCode::InvalidArgument,
            ));
        }

        if request.generation().is_empty()
            || request.semantic_revision().is_empty()
        {
            return Err(ResilienceError::new(
                ResilienceErrorCode::InvalidArgument,
            ));
        }

        if !self.capabilities.deterministic()
            && request.requires_determinism()
        {
            return Err(ResilienceError::new(
                ResilienceErrorCode::InvalidArgument,
            ));
        }

        Ok(())
    }

    /// Validates an adaptation candidate against the current request.
    fn validate_candidate(
        &self,
        request: &AdaptationRequest,
        candidate: &AdaptationCandidate,
    ) -> ResilienceResult<()> {
        if candidate.adapter() != &self.id {
            return Err(ResilienceError::new(
                ResilienceErrorCode::InvalidArgument,
            ));
        }

        if candidate.action_kind() != ActionKind::AdaptQec {
            return Err(ResilienceError::new(
                ResilienceErrorCode::InvalidArgument,
            ));
        }

        if candidate.generation() != request.generation() {
            return Err(ResilienceError::new(
                ResilienceErrorCode::StaleExecution,
            ));
        }

        if candidate.semantic_revision()
            != request.semantic_revision()
        {
            return Err(ResilienceError::new(
                ResilienceErrorCode::StaleSemanticRevision,
            ));
        }

        if candidate.identity().is_empty() {
            return Err(ResilienceError::new(
                ResilienceErrorCode::InvalidIdentifier,
            ));
        }

        Ok(())
    }

    /// Converts a QEC service candidate into the canonical resilience
    /// `AdaptationCandidate`.
    fn make_candidate(
        &self,
        request: &AdaptationRequest,
        candidate: QecAdaptationCandidateId,
    ) -> ResilienceResult<AdaptationCandidate> {
        AdaptationCandidate::new(
            self.id.clone(),
            ActionKind::AdaptQec,
            request.generation().clone(),
            request.semantic_revision().clone(),
            candidate.into_string(),
        )
    }

    /// Executes the QEC service operation represented by the canonical
    /// adaptation operation.
    fn execute_operation(
        &self,
        operation: &AdaptationOperation,
        request: &AdaptationRequest,
    ) -> ResilienceResult<AdaptationResult> {
        self.validate_request(request)?;

        match operation {
            AdaptationOperation::Preflight => {
                self.qec.preflight(request)?;

                Ok(AdaptationResult::new(
                    self.id.clone(),
                    ActionKind::AdaptQec,
                    AdaptationStatus::Prepared,
                    request.generation().clone(),
                    request.semantic_revision().clone(),
                ))
            }

            AdaptationOperation::Prepare => {
                self.qec.preflight(request)?;

                let candidate = self.qec.prepare(request)?;
                let candidate =
                    self.make_candidate(request, candidate)?;

                Ok(AdaptationResult::new(
                    self.id.clone(),
                    ActionKind::AdaptQec,
                    AdaptationStatus::Prepared,
                    candidate_generation(&candidate),
                    candidate_semantic_revision(&candidate),
                ))
            }

            AdaptationOperation::Commit { candidate } => {
                self.validate_candidate(request, candidate)?;

                let candidate_id =
                    QecAdaptationCandidateId::new(
                        candidate.identity().to_owned(),
                    )?;

                let committed =
                    self.qec.commit(request, &candidate_id)?;

                if !committed {
                    return Err(ResilienceError::new(
                        ResilienceErrorCode::CommitFailed,
                    ));
                }

                Ok(AdaptationResult::new(
                    self.id.clone(),
                    ActionKind::AdaptQec,
                    AdaptationStatus::CommittedPendingVerification,
                    request.generation().clone(),
                    request.semantic_revision().clone(),
                ))
            }

            AdaptationOperation::Verify { candidate } => {
                self.validate_candidate(request, candidate)?;

                let candidate_id =
                    QecAdaptationCandidateId::new(
                        candidate.identity().to_owned(),
                    )?;

                let verified =
                    self.qec.verify(request, &candidate_id)?;

                if !verified {
                    return Err(ResilienceError::new(
                        ResilienceErrorCode::VerificationFailed,
                    ));
                }

                Ok(AdaptationResult::new(
                    self.id.clone(),
                    ActionKind::AdaptQec,
                    AdaptationStatus::Verified,
                    request.generation().clone(),
                    request.semantic_revision().clone(),
                ))
            }
        }
    }
}

// ============================================================================
// Candidate metadata helpers
// ============================================================================

/// Returns the candidate generation without exposing the internal candidate
/// representation to the QEC service.
fn candidate_generation(
    candidate: &AdaptationCandidate,
) -> crate::quantum::resilience::adaptation::adapter::ExecutionGeneration {
    candidate.generation().clone()
}

/// Returns the candidate semantic revision without exposing the internal
/// candidate representation to the QEC service.
fn candidate_semantic_revision(
    candidate: &AdaptationCandidate,
) -> crate::quantum::resilience::adaptation::adapter::SemanticRevision {
    candidate.semantic_revision().clone()
}

// ============================================================================
// AdaptationAdapter implementation
// ============================================================================

impl<Q> AdaptationAdapter for QecAdaptationAdapter<Q>
where
    Q: QecAdaptationService,
{
    /// Stable adapter identifier.
    fn id(&self) -> &AdapterId {
        &self.id
    }

    /// Adapter implementation version.
    fn version(&self) -> AdapterVersion {
        QEC_ADAPTATION_ADAPTER_VERSION
    }

    /// Canonical action set supported by this adapter.
    fn supported(&self) -> &[ActionKind] {
        &SUPPORTED_ACTIONS
    }

    /// Adapter execution capabilities.
    fn capabilities(&self) -> AdaptationCapabilities {
        self.capabilities
    }

    /// Executes one transactional adaptation operation.
    fn execute(
        &self,
        operation: &AdaptationOperation,
        request: &AdaptationRequest,
    ) -> ResilienceResult<AdaptationResult> {
        self.execute_operation(operation, request)
    }
}

// ============================================================================
// Factory
// ============================================================================

/// Creates a thread-safe QEC adaptation adapter handle.
///
/// The returned handle can be registered with the adaptation registry.
pub fn qec_adaptation_adapter<Q>(
    qec: Arc<Q>,
) -> ResilienceResult<Arc<dyn AdaptationAdapter>>
where
    Q: QecAdaptationService + 'static,
{
    Ok(Arc::new(QecAdaptationAdapter::new(qec)?))
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Default)]
    struct TestQecService;

    impl QecAdaptationService for TestQecService {
        fn preflight(
            &self,
            request: &AdaptationRequest,
        ) -> ResilienceResult<()> {
            if request.action_kind() != ActionKind::AdaptQec {
                return Err(ResilienceError::new(
                    ResilienceErrorCode::InvalidArgument,
                ));
            }

            Ok(())
        }

        fn prepare(
            &self,
            request: &AdaptationRequest,
        ) -> ResilienceResult<QecAdaptationCandidateId> {
            self.preflight(request)?;

            QecAdaptationCandidateId::new("test-qec-candidate")
        }

        fn commit(
            &self,
            request: &AdaptationRequest,
            candidate: &QecAdaptationCandidateId,
        ) -> ResilienceResult<bool> {
            self.preflight(request)?;

            if candidate.as_str().is_empty() {
                return Ok(false);
            }

            Ok(true)
        }

        fn verify(
            &self,
            request: &AdaptationRequest,
            candidate: &QecAdaptationCandidateId,
        ) -> ResilienceResult<bool> {
            self.preflight(request)?;

            Ok(!candidate.as_str().is_empty())
        }
    }

    #[test]
    fn candidate_identity_rejects_empty_values() {
        let result = QecAdaptationCandidateId::new("");
        assert!(result.is_err());
    }

    #[test]
    fn adapter_constructs_without_hardware_assumptions() {
        let service = Arc::new(TestQecService);
        let adapter = QecAdaptationAdapter::new(service);

        assert!(adapter.is_ok());

        let adapter = adapter.expect("adapter construction must succeed");

        assert_eq!(
            adapter.id().as_str(),
            QEC_ADAPTATION_ADAPTER_ID
        );

        assert_eq!(
            adapter.supported(),
            &[ActionKind::AdaptQec]
        );

        assert!(adapter.capabilities().supports_prepare());
        assert!(adapter.capabilities().supports_commit());
        assert!(adapter.capabilities().supports_preflight());
    }
}