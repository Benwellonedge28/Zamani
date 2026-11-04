#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani AI Deployment Lifecycle
//!
//! Path: src/ai/deployment/lifecycle.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Provides the authoritative in-process lifecycle state and transition
//! journal for logical AI deployments.
//!
//! A deployment lifecycle records the progression of an AI deployment
//! through registration, validation, preparation, deployment, activation,
//! updating, degradation, rollback, stopping and terminal outcomes.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - AI deployment lifecycle states.
//! - AI deployment lifecycle transition validation.
//! - Per-deployment transition revisions.
//! - In-process lifecycle snapshots.
//! - In-process lifecycle event journaling.
//! - Optimistic revision checking.
//! - Thread-safe lifecycle state updates.
//!
//! ## Non-ownership
//!
//! This module does not own:
//!
//! - AI deployment manifest structure.
//! - Artifact payload storage.
//! - Artifact serialization.
//! - Artifact integrity verification.
//! - Model compatibility evaluation.
//! - Target capability discovery.
//! - Resource feasibility.
//! - Hardware selection.
//! - Physical placement.
//! - Scheduling.
//! - Process or container creation.
//! - Network provisioning.
//! - Model execution.
//! - Training or inference.
//! - Checkpoint creation.
//! - Recovery implementation.
//! - Durable event storage.
//! - Authentication or authorization.
//! - Deployment orchestration.
//!
//! These responsibilities remain with their existing repository owners.
//!
//! ## Canonical integration
//!
//! AI deployment manifest:
//!     crate::ai::deployment::manifests::AiDeploymentManifest
//!
//! Deployment manifest validation:
//!     AiDeploymentManifest::validate()
//!
//! AI deployment artifacts:
//!     crate::ai::deployment::artifacts
//!
//! AI deployment compatibility:
//!     crate::ai::deployment::compatibility
//!
//! Distributed execution lifecycle:
//!     crate::distributed::execution::lifecycle
//!
//! Runtime:
//!     crate::runtime
//!
//! Observability:
//!     Existing repository observability infrastructure.
//!
//! ## Lifecycle semantics
//!
//! A successful transition means that the in-process lifecycle state and
//! its event journal were updated atomically under the lifecycle mutex.
//!
//! It does not establish that a remote deployment operation succeeded.
//!
//! In particular:
//!
//! - Deploying is not Active.
//! - A deployment request is not deployment completion.
//! - A cancellation request is not confirmed cancellation.
//! - Entering RollingBack does not establish rollback success.
//! - Active must only be recorded after the deployment provider confirms
//!   successful activation.
//! - Stopped must only be recorded after the responsible provider confirms
//!   that the deployment has stopped.
//!
//! ## Persistence
//!
//! This implementation is an in-process lifecycle authority.
//!
//! Its event journal is not durable across process termination.
//!
//! A production runtime requiring crash recovery must persist lifecycle
//! events and snapshots through the repository's durable storage and
//! resilience infrastructure.
//!
//! ## Concurrency
//!
//! One mutex protects state changes and event journal updates.
//!
//! This provides atomicity between callers sharing this manager instance.
//! It does not provide distributed consensus, cross-process coordination,
//! or exactly-once remote deployment.
//!
//! ## Scalability and POCO-REAF
//!
//! No fixed limit is imposed on:
//!
//! - Deployments.
//! - Models.
//! - Artifacts.
//! - Deployment revisions.
//! - Lifecycle events.
//! - Execution providers.
//! - Machines.
//! - Workers.
//! - Accelerators.
//! - Nodes.
//! - Memory.
//! - Tensor dimensions.
//!
//! Actual growth is constrained by available memory, address space,
//! representation limits and caller-configured operational budgets.
//!
//! This module does not promise physically infinite resources.
//!
//! Operational limits must be enforced by the appropriate runtime,
//! storage, security or deployment infrastructure.
//!
//! They must not become language-level semantic ceilings.

use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::hash::Hash;
use std::sync::{Mutex, MutexGuard};

use crate::ai::deployment::manifests::AiDeploymentManifest;

// ============================================================================
// DEPLOYMENT STATE
// ============================================================================

/// Current lifecycle state of one logical AI deployment.
///
/// State transitions are validated by [`is_transition_allowed`].
///
/// Terminal states cannot transition again. A new deployment attempt after
/// a terminal outcome must follow the deployment system's explicit retry
/// and reconciliation policy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AiDeploymentState {
    /// Deployment has been registered after manifest validation.
    Registered,

    /// Manifest, artifacts and deployment requirements are being validated.
    Validating,

    /// Deployment preparation has completed.
    ///
    /// This state does not establish that physical resources have been
    /// allocated or that a target has been selected.
    Prepared,

    /// A deployment provider is performing the deployment operation.
    Deploying,

    /// Deployment has been confirmed active by the responsible provider.
    Active,

    /// An update to an existing deployment is being prepared.
    Updating,

    /// The deployment is operational but has a reported degradation.
    ///
    /// The reason is supplied by the responsible deployment provider.
    Degraded {
        /// Explanation of the degradation.
        reason: String,
    },

    /// Rollback is in progress.
    ///
    /// This state does not establish that the previous deployment has
    /// been restored.
    RollingBack {
        /// Explanation for the rollback.
        reason: String,
    },

    /// Shutdown has been requested and is being performed.
    Stopping,

    /// Deployment has been confirmed stopped.
    Stopped,

    /// Deployment failed.
    Failed {
        /// Stable deployment error code, when available.
        error_code: String,

        /// Human-readable failure explanation.
        message: String,
    },

    /// Deployment cancellation has been confirmed.
    Cancelled {
        /// Explanation for cancellation.
        reason: String,
    },
}

impl AiDeploymentState {
    /// Returns whether this state is terminal.
    ///
    /// Terminal states cannot transition to another state.
    #[must_use]
    pub const fn is_terminal(&self) -> bool {
        matches!(
            self,
            Self::Stopped | Self::Failed { .. } | Self::Cancelled { .. }
        )
    }

    /// Returns a stable lifecycle state identifier.
    ///
    /// These identifiers are suitable for diagnostics and event
    /// classification. If persisted or transmitted externally, their
    /// compatibility must be governed by the corresponding storage or
    /// transport protocol.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Registered => "registered",
            Self::Validating => "validating",
            Self::Prepared => "prepared",
            Self::Deploying => "deploying",
            Self::Active => "active",
            Self::Updating => "updating",
            Self::Degraded { .. } => "degraded",
            Self::RollingBack { .. } => "rolling_back",
            Self::Stopping => "stopping",
            Self::Stopped => "stopped",
            Self::Failed { .. } => "failed",
            Self::Cancelled { .. } => "cancelled",
        }
    }
}

// ============================================================================
// LIFECYCLE EVENTS AND SNAPSHOTS
// ============================================================================

/// A successful AI deployment lifecycle transition.
///
/// The revision is monotonically increasing for one deployment within
/// this lifecycle manager.
///
/// It is not a timestamp and does not establish ordering between
/// different deployments.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AiDeploymentLifecycleEvent<K> {
    /// Logical deployment identity supplied by the caller.
    pub deployment_id: K,

    /// State before the transition.
    ///
    /// The initial registration event has no previous state.
    pub previous: Option<AiDeploymentState>,

    /// State after the transition.
    pub current: AiDeploymentState,

    /// Per-deployment revision after this transition.
    pub revision: u64,
}

/// Immutable snapshot of one AI deployment lifecycle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AiDeploymentLifecycleSnapshot<K> {
    /// Logical deployment identity.
    pub deployment_id: K,

    /// Current deployment state.
    pub state: AiDeploymentState,

    /// Current per-deployment revision.
    pub revision: u64,
}

// ============================================================================
// INTERNAL RECORDS
// ============================================================================

/// Internal lifecycle state for one deployment.
#[derive(Debug, Clone)]
struct DeploymentRecord {
    state: AiDeploymentState,
    revision: u64,
}

/// Shared lifecycle manager state.
///
/// A single mutex protects deployment records and event journal updates.
#[derive(Debug)]
struct LifecycleStore<K> {
    deployments: HashMap<K, DeploymentRecord>,
    events: Vec<AiDeploymentLifecycleEvent<K>>,
}

// ============================================================================
// LIFECYCLE ERRORS
// ============================================================================

/// Errors produced by AI deployment lifecycle management.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AiDeploymentLifecycleError {
    /// The supplied deployment identifier is invalid.
    InvalidDeploymentIdentity {
        /// Explanation of the invalid identity.
        message: String,
    },

    /// The deployment manifest is invalid.
    InvalidManifest {
        /// Explanation from the canonical manifest validator.
        message: String,
    },

    /// The deployment manifest contains no artifacts.
    ///
    /// An empty manifest may be a valid intermediate data structure,
    /// but it is not sufficient to register a deployable AI artifact set.
    EmptyManifest,

    /// A deployment identity has already been registered.
    AlreadyRegistered,

    /// A deployment identity has not been registered.
    NotRegistered,

    /// The requested state transition is not permitted.
    InvalidTransition {
        /// Existing state.
        from: String,

        /// Requested state.
        to: String,
    },

    /// The caller supplied an outdated lifecycle revision.
    RevisionConflict {
        /// Expected revision supplied by the caller.
        expected: u64,

        /// Actual current revision.
        actual: u64,
    },

    /// The per-deployment revision counter is exhausted.
    RevisionExhausted,

    /// The lifecycle manager mutex was poisoned.
    LockPoisoned,

    /// An internal lifecycle invariant was violated.
    InternalInvariantViolation {
        /// Description of the violated invariant.
        message: String,
    },
}

impl fmt::Display for AiDeploymentLifecycleError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidDeploymentIdentity { message } => {
                write!(
                    formatter,
                    "invalid AI deployment identity: {message}"
                )
            }

            Self::InvalidManifest { message } => {
                write!(
                    formatter,
                    "invalid AI deployment manifest: {message}"
                )
            }

            Self::EmptyManifest => {
                formatter.write_str(
                    "an AI deployment cannot be registered without artifacts",
                )
            }

            Self::AlreadyRegistered => {
                formatter.write_str(
                    "AI deployment identity is already registered",
                )
            }

            Self::NotRegistered => {
                formatter.write_str(
                    "AI deployment identity has not been registered",
                )
            }

            Self::InvalidTransition { from, to } => {
                write!(
                    formatter,
                    "AI deployment transition from {from} to {to} is not permitted"
                )
            }

            Self::RevisionConflict { expected, actual } => {
                write!(
                    formatter,
                    "AI deployment revision conflict: expected {expected}, current revision is {actual}"
                )
            }

            Self::RevisionExhausted => {
                formatter.write_str(
                    "AI deployment lifecycle revision counter exhausted",
                )
            }

            Self::LockPoisoned => {
                formatter.write_str(
                    "AI deployment lifecycle state lock was poisoned",
                )
            }

            Self::InternalInvariantViolation { message } => {
                write!(
                    formatter,
                    "AI deployment lifecycle invariant violated: {message}"
                )
            }
        }
    }
}

impl Error for AiDeploymentLifecycleError {}

// ============================================================================
// LIFECYCLE MANAGER
// ============================================================================

/// Thread-safe lifecycle manager for logical AI deployments.
///
/// `K` is the deployment identity type supplied by the caller.
///
/// The lifecycle manager deliberately does not introduce another global
/// identity allocator or assume that deployment identities are hardware
/// identities, process identifiers, model identifiers, or network addresses.
///
/// The caller must use a stable identity scope appropriate to its
/// deployment orchestration system.
#[derive(Debug)]
pub struct AiDeploymentLifecycle<K> {
    store: Mutex<LifecycleStore<K>>,
}

impl<K> Default for AiDeploymentLifecycle<K>
where
    K: Eq + Hash,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<K> AiDeploymentLifecycle<K>
where
    K: Clone + Eq + Hash + fmt::Debug,
{
    /// Creates an empty lifecycle manager.
    #[must_use]
    pub fn new() -> Self {
        Self {
            store: Mutex::new(LifecycleStore {
                deployments: HashMap::new(),
                events: Vec::new(),
            }),
        }
    }

    /// Registers a new AI deployment.
    ///
    /// The canonical AI deployment manifest validator is invoked before
    /// registration.
    ///
    /// Empty manifests are rejected because they do not describe a
    /// deployable artifact set.
    ///
    /// Duplicate registration is rejected. It never silently resets
    /// an existing deployment lifecycle.
    ///
    /// The initial state is [`AiDeploymentState::Registered`].
    ///
    /// Registration does not deploy or execute anything.
    pub fn register(
        &self,
        deployment_id: K,
        manifest: &AiDeploymentManifest,
    ) -> Result<
        AiDeploymentLifecycleEvent<K>,
        AiDeploymentLifecycleError,
    > {
        validate_deployment_identity(&deployment_id)?;

        manifest.validate().map_err(|error| {
            AiDeploymentLifecycleError::InvalidManifest {
                message: error.to_string(),
            }
        })?;

        if manifest.is_empty() {
            return Err(AiDeploymentLifecycleError::EmptyManifest);
        }

        let mut store = self.lock_store()?;

        if store.deployments.contains_key(&deployment_id) {
            return Err(
                AiDeploymentLifecycleError::AlreadyRegistered,
            );
        }

        let event = AiDeploymentLifecycleEvent {
            deployment_id: deployment_id.clone(),
            previous: None,
            current: AiDeploymentState::Registered,
            revision: 0,
        };

        // The mutex protects both journal insertion and state registration.
        // No fallible operation follows the event append before insertion.
        store.events.push(event.clone());

        store.deployments.insert(
            deployment_id,
            DeploymentRecord {
                state: AiDeploymentState::Registered,
                revision: 0,
            },
        );

        Ok(event)
    }

    /// Begins manifest and deployment validation.
    ///
    /// Call this when the deployment validation subsystem begins its work.
    pub fn begin_validation(
        &self,
        deployment_id: &K,
    ) -> Result<
        AiDeploymentLifecycleEvent<K>,
        AiDeploymentLifecycleError,
    > {
        self.transition(
            deployment_id,
            AiDeploymentState::Validating,
        )
    }

    /// Records successful deployment preparation.
    ///
    /// Call this only after the responsible preparation subsystem has
    /// confirmed that all required preparation steps have completed.
    ///
    /// This does not claim that physical resources have been allocated.
    pub fn prepare(
        &self,
        deployment_id: &K,
    ) -> Result<
        AiDeploymentLifecycleEvent<K>,
        AiDeploymentLifecycleError,
    > {
        self.transition(
            deployment_id,
            AiDeploymentState::Prepared,
        )
    }

    /// Records the beginning of deployment.
    ///
    /// This is a state transition only. It does not perform deployment.
    pub fn begin_deployment(
        &self,
        deployment_id: &K,
    ) -> Result<
        AiDeploymentLifecycleEvent<K>,
        AiDeploymentLifecycleError,
    > {
        self.transition(
            deployment_id,
            AiDeploymentState::Deploying,
        )
    }

    /// Records confirmed activation.
    ///
    /// The caller must invoke this only after the responsible deployment
    /// provider confirms that the deployment is active and the required
    /// activation checks have succeeded.
    pub fn activate(
        &self,
        deployment_id: &K,
    ) -> Result<
        AiDeploymentLifecycleEvent<K>,
        AiDeploymentLifecycleError,
    > {
        self.transition(
            deployment_id,
            AiDeploymentState::Active,
        )
    }

    /// Begins an update to an existing deployment.
    ///
    /// This does not modify artifacts, models, manifests, or runtime state.
    pub fn begin_update(
        &self,
        deployment_id: &K,
    ) -> Result<
        AiDeploymentLifecycleEvent<K>,
        AiDeploymentLifecycleError,
    > {
        self.transition(
            deployment_id,
            AiDeploymentState::Updating,
        )
    }

    /// Records a deployment degradation.
    ///
    /// A degradation report is an observation supplied by the responsible
    /// provider. This method does not diagnose the underlying cause.
    pub fn report_degraded(
        &self,
        deployment_id: &K,
        reason: impl Into<String>,
    ) -> Result<
        AiDeploymentLifecycleEvent<K>,
        AiDeploymentLifecycleError,
    > {
        let reason = validate_reason(reason.into())?;

        self.transition(
            deployment_id,
            AiDeploymentState::Degraded { reason },
        )
    }

    /// Begins rollback.
    ///
    /// The caller must have independently decided that rollback is valid
    /// and has initiated the corresponding operation.
    ///
    /// This method does not restore artifacts, switch traffic, or perform
    /// provider-specific rollback.
    pub fn begin_rollback(
        &self,
        deployment_id: &K,
        reason: impl Into<String>,
    ) -> Result<
        AiDeploymentLifecycleEvent<K>,
        AiDeploymentLifecycleError,
    > {
        let reason = validate_reason(reason.into())?;

        self.transition(
            deployment_id,
            AiDeploymentState::RollingBack { reason },
        )
    }

    /// Records the successful completion of rollback.
    ///
    /// The caller must verify that the previous deployment has actually
    /// been restored before invoking this method.
    pub fn complete_rollback(
        &self,
        deployment_id: &K,
    ) -> Result<
        AiDeploymentLifecycleEvent<K>,
        AiDeploymentLifecycleError,
    > {
        self.transition(
            deployment_id,
            AiDeploymentState::Active,
        )
    }

    /// Begins deployment shutdown.
    ///
    /// This records shutdown progress. It does not stop processes or
    /// release physical resources.
    pub fn begin_stopping(
        &self,
        deployment_id: &K,
    ) -> Result<
        AiDeploymentLifecycleEvent<K>,
        AiDeploymentLifecycleError,
    > {
        self.transition(
            deployment_id,
            AiDeploymentState::Stopping,
        )
    }

    /// Records confirmed deployment shutdown.
    ///
    /// The caller must invoke this only after the responsible provider
    /// confirms that the deployment has stopped.
    pub fn confirm_stopped(
        &self,
        deployment_id: &K,
    ) -> Result<
        AiDeploymentLifecycleEvent<K>,
        AiDeploymentLifecycleError,
    > {
        self.transition(
            deployment_id,
            AiDeploymentState::Stopped,
        )
    }

    /// Records deployment failure.
    ///
    /// The original error should still be propagated by the deployment
    /// orchestrator. This lifecycle record retains a stable code and
    /// explanation, not the original error source chain.
    pub fn fail(
        &self,
        deployment_id: &K,
        error_code: impl Into<String>,
        message: impl Into<String>,
    ) -> Result<
        AiDeploymentLifecycleEvent<K>,
        AiDeploymentLifecycleError,
    > {
        let error_code = validate_reason(error_code.into())?;
        let message = validate_reason(message.into())?;

        self.transition(
            deployment_id,
            AiDeploymentState::Failed {
                error_code,
                message,
            },
        )
    }

    /// Records confirmed deployment cancellation.
    ///
    /// A cancellation request is not confirmation. The caller must first
    /// obtain an acknowledgement or definitive result from the responsible
    /// deployment provider.
    pub fn confirm_cancelled(
        &self,
        deployment_id: &K,
        reason: impl Into<String>,
    ) -> Result<
        AiDeploymentLifecycleEvent<K>,
        AiDeploymentLifecycleError,
    > {
        let reason = validate_reason(reason.into())?;

        self.transition(
            deployment_id,
            AiDeploymentState::Cancelled { reason },
        )
    }

    /// Returns the current snapshot for one deployment.
    pub fn get(
        &self,
        deployment_id: &K,
    ) -> Result<
        Option<AiDeploymentLifecycleSnapshot<K>>,
        AiDeploymentLifecycleError,
    > {
        let store = self.lock_store()?;

        Ok(store.deployments.get(deployment_id).map(|record| {
            AiDeploymentLifecycleSnapshot {
                deployment_id: deployment_id.clone(),
                state: record.state.clone(),
                revision: record.revision,
            }
        }))
    }

    /// Returns whether a deployment is currently registered.
    pub fn contains(
        &self,
        deployment_id: &K,
    ) -> Result<bool, AiDeploymentLifecycleError> {
        let store = self.lock_store()?;

        Ok(store.deployments.contains_key(deployment_id))
    }

    /// Returns the number of deployments currently tracked by this manager.
    ///
    /// This is an in-process observation, not a global deployment count.
    pub fn deployment_count(
        &self,
    ) -> Result<usize, AiDeploymentLifecycleError> {
        let store = self.lock_store()?;

        Ok(store.deployments.len())
    }

    /// Returns the number of lifecycle events waiting to be consumed.
    pub fn pending_event_count(
        &self,
    ) -> Result<usize, AiDeploymentLifecycleError> {
        let store = self.lock_store()?;

        Ok(store.events.len())
    }

    /// Drains all currently pending lifecycle events.
    ///
    /// Draining does not delete deployment state.
    ///
    /// The caller becomes responsible for retaining or persisting the
    /// returned events. If persistence fails, the caller must reconcile
    /// the events through its durable storage authority.
    ///
    /// This method does not provide durable event delivery.
    pub fn drain_events(
        &self,
    ) -> Result<
        Vec<AiDeploymentLifecycleEvent<K>>,
        AiDeploymentLifecycleError,
    > {
        let mut store = self.lock_store()?;

        Ok(std::mem::take(&mut store.events))
    }

    /// Returns snapshots of all tracked deployments.
    ///
    /// Hash-map iteration order is unspecified. Callers requiring
    /// deterministic presentation or serialization must sort snapshots
    /// using their own deployment identity contract.
    pub fn snapshots(
        &self,
    ) -> Result<
        Vec<AiDeploymentLifecycleSnapshot<K>>,
        AiDeploymentLifecycleError,
    > {
        let store = self.lock_store()?;

        Ok(store
            .deployments
            .iter()
            .map(|(deployment_id, record)| {
                AiDeploymentLifecycleSnapshot {
                    deployment_id: deployment_id.clone(),
                    state: record.state.clone(),
                    revision: record.revision,
                }
            })
            .collect())
    }

    /// Applies a validated lifecycle transition.
    ///
    /// This method serializes concurrent transitions under the lifecycle
    /// mutex and appends the event and updates the state atomically.
    ///
    /// It does not perform the external operation represented by the state.
    pub fn transition(
        &self,
        deployment_id: &K,
        next: AiDeploymentState,
    ) -> Result<
        AiDeploymentLifecycleEvent<K>,
        AiDeploymentLifecycleError,
    > {
        self.transition_inner(deployment_id, next, None)
    }

    /// Applies a transition only if the current revision matches the
    /// caller's expected revision.
    ///
    /// This prevents stale controllers from silently overwriting a newer
    /// lifecycle state.
    pub fn transition_if_revision(
        &self,
        deployment_id: &K,
        expected_revision: u64,
        next: AiDeploymentState,
    ) -> Result<
        AiDeploymentLifecycleEvent<K>,
        AiDeploymentLifecycleError,
    > {
        self.transition_inner(
            deployment_id,
            next,
            Some(expected_revision),
        )
    }

    fn transition_inner(
        &self,
        deployment_id: &K,
        next: AiDeploymentState,
        expected_revision: Option<u64>,
    ) -> Result<
        AiDeploymentLifecycleEvent<K>,
        AiDeploymentLifecycleError,
    > {
        validate_state_payload(&next)?;

        let mut store = self.lock_store()?;

        let record = store
            .deployments
            .get(deployment_id)
            .ok_or(AiDeploymentLifecycleError::NotRegistered)?;

        if let Some(expected) = expected_revision {
            if record.revision != expected {
                return Err(
                    AiDeploymentLifecycleError::RevisionConflict {
                        expected,
                        actual: record.revision,
                    },
                );
            }
        }

        if !is_transition_allowed(&record.state, &next) {
            return Err(
                AiDeploymentLifecycleError::InvalidTransition {
                    from: record.state.as_str().to_owned(),
                    to: next.as_str().to_owned(),
                },
            );
        }

        let revision = record
            .revision
            .checked_add(1)
            .ok_or(AiDeploymentLifecycleError::RevisionExhausted)?;

        let event = AiDeploymentLifecycleEvent {
            deployment_id: deployment_id.clone(),
            previous: Some(record.state.clone()),
            current: next.clone(),
            revision,
        };

        // The mutex protects the journal and state from concurrent
        // transitions. No fallible operation follows the event append
        // before the record update.
        store.events.push(event.clone());

        let record = store
            .deployments
            .get_mut(deployment_id)
            .ok_or_else(|| {
                AiDeploymentLifecycleError::InternalInvariantViolation {
                    message:
                        "deployment record disappeared while lifecycle lock was held"
                            .to_owned(),
                }
            })?;

        record.state = next;
        record.revision = revision;

        Ok(event)
    }

    fn lock_store(
        &self,
    ) -> Result<
        MutexGuard<'_, LifecycleStore<K>>,
        AiDeploymentLifecycleError,
    > {
        self.store
            .lock()
            .map_err(|_| AiDeploymentLifecycleError::LockPoisoned)
    }
}

// ============================================================================
// TRANSITION VALIDATION
// ============================================================================

/// Returns whether a transition is valid under the AI deployment lifecycle
/// contract.
///
/// Permitted transitions:
///
/// - Registered -> Validating, Failed, Cancelled
/// - Validating -> Prepared, Failed, Cancelled
/// - Prepared -> Deploying, Failed, Cancelled
/// - Deploying -> Active, Degraded, RollingBack, Failed, Cancelled
/// - Active -> Updating, Degraded, Stopping
/// - Updating -> Validating, Prepared, Deploying, RollingBack, Failed, Cancelled
/// - Degraded -> Updating, RollingBack, Stopping, Failed
/// - RollingBack -> Active, Failed, Cancelled
/// - Stopping -> Stopped, Failed
///
/// Terminal states cannot transition again.
///
/// A deployment must not become Active before the provider confirms
/// successful activation. A deployment must not become Stopped before
/// the provider confirms successful shutdown.
#[must_use]
pub fn is_transition_allowed(
    current: &AiDeploymentState,
    next: &AiDeploymentState,
) -> bool {
    match current {
        AiDeploymentState::Registered => matches!(
            next,
            AiDeploymentState::Validating
                | AiDeploymentState::Failed { .. }
                | AiDeploymentState::Cancelled { .. }
        ),

        AiDeploymentState::Validating => matches!(
            next,
            AiDeploymentState::Prepared
                | AiDeploymentState::Failed { .. }
                | AiDeploymentState::Cancelled { .. }
        ),

        AiDeploymentState::Prepared => matches!(
            next,
            AiDeploymentState::Deploying
                | AiDeploymentState::Failed { .. }
                | AiDeploymentState::Cancelled { .. }
        ),

        AiDeploymentState::Deploying => matches!(
            next,
            AiDeploymentState::Active
                | AiDeploymentState::Degraded { .. }
                | AiDeploymentState::RollingBack { .. }
                | AiDeploymentState::Failed { .. }
                | AiDeploymentState::Cancelled { .. }
        ),

        AiDeploymentState::Active => matches!(
            next,
            AiDeploymentState::Updating
                | AiDeploymentState::Degraded { .. }
                | AiDeploymentState::Stopping
        ),

        AiDeploymentState::Updating => matches!(
            next,
            AiDeploymentState::Validating
                | AiDeploymentState::Prepared
                | AiDeploymentState::Deploying
                | AiDeploymentState::RollingBack { .. }
                | AiDeploymentState::Failed { .. }
                | AiDeploymentState::Cancelled { .. }
        ),

        AiDeploymentState::Degraded { .. } => matches!(
            next,
            AiDeploymentState::Updating
                | AiDeploymentState::RollingBack { .. }
                | AiDeploymentState::Stopping
                | AiDeploymentState::Failed { .. }
        ),

        AiDeploymentState::RollingBack { .. } => matches!(
            next,
            AiDeploymentState::Active
                | AiDeploymentState::Failed { .. }
                | AiDeploymentState::Cancelled { .. }
        ),

        AiDeploymentState::Stopping => matches!(
            next,
            AiDeploymentState::Stopped
                | AiDeploymentState::Failed { .. }
        ),

        AiDeploymentState::Stopped
        | AiDeploymentState::Failed { .. }
        | AiDeploymentState::Cancelled { .. } => false,
    }
}

// ============================================================================
// VALIDATION HELPERS
// ============================================================================

fn validate_deployment_identity<K>(
    deployment_id: &K,
) -> Result<(), AiDeploymentLifecycleError>
where
    K: fmt::Debug,
{
    let identity = format!("{deployment_id:?}");

    if identity.trim().is_empty() {
        return Err(
            AiDeploymentLifecycleError::InvalidDeploymentIdentity {
                message: "deployment identity must not be empty".to_owned(),
            },
        );
    }

    if identity.chars().any(char::is_control) {
        return Err(
            AiDeploymentLifecycleError::InvalidDeploymentIdentity {
                message:
                    "deployment identity debug representation contains control characters"
                        .to_owned(),
            },
        );
    }

    Ok(())
}

fn validate_reason(
    value: String,
) -> Result<String, AiDeploymentLifecycleError> {
    if value.trim().is_empty() {
        return Err(
            AiDeploymentLifecycleError::InternalInvariantViolation {
                message: "lifecycle reason or error description must not be empty"
                    .to_owned(),
            },
        );
    }

    if value.chars().any(char::is_control) {
        return Err(
            AiDeploymentLifecycleError::InternalInvariantViolation {
                message:
                    "lifecycle reason or error description contains control characters"
                        .to_owned(),
            },
        );
    }

    Ok(value)
}

fn validate_state_payload(
    state: &AiDeploymentState,
) -> Result<(), AiDeploymentLifecycleError> {
    match state {
        AiDeploymentState::Degraded { reason }
        | AiDeploymentState::RollingBack { reason }
        | AiDeploymentState::Cancelled { reason } => {
            validate_reason(reason.clone())?;
        }

        AiDeploymentState::Failed {
            error_code,
            message,
        } => {
            validate_reason(error_code.clone())?;
            validate_reason(message.clone())?;
        }

        AiDeploymentState::Registered
        | AiDeploymentState::Validating
        | AiDeploymentState::Prepared
        | AiDeploymentState::Deploying
        | AiDeploymentState::Active
        | AiDeploymentState::Updating
        | AiDeploymentState::Stopping
        | AiDeploymentState::Stopped => {}
    }

    Ok(())
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_state_is_not_terminal() {
        assert!(!AiDeploymentState::Registered.is_terminal());
        assert!(!AiDeploymentState::Validating.is_terminal());
        assert!(!AiDeploymentState::Prepared.is_terminal());
        assert!(!AiDeploymentState::Deploying.is_terminal());
        assert!(!AiDeploymentState::Active.is_terminal());
        assert!(!AiDeploymentState::Updating.is_terminal());
        assert!(!AiDeploymentState::Degraded {
            reason: "provider warning".to_owned(),
        }
        .is_terminal());
    }

    #[test]
    fn terminal_states_are_terminal() {
        assert!(AiDeploymentState::Stopped.is_terminal());

        assert!(AiDeploymentState::Failed {
            error_code: "deployment.failed".to_owned(),
            message: "provider failure".to_owned(),
        }
        .is_terminal());

        assert!(AiDeploymentState::Cancelled {
            reason: "operator requested cancellation".to_owned(),
        }
        .is_terminal());
    }

    #[test]
    fn normal_deployment_transition_path_is_valid() {
        let transitions = [
            (
                AiDeploymentState::Registered,
                AiDeploymentState::Validating,
            ),
            (
                AiDeploymentState::Validating,
                AiDeploymentState::Prepared,
            ),
            (
                AiDeploymentState::Prepared,
                AiDeploymentState::Deploying,
            ),
            (
                AiDeploymentState::Deploying,
                AiDeploymentState::Active,
            ),
            (
                AiDeploymentState::Active,
                AiDeploymentState::Stopping,
            ),
            (
                AiDeploymentState::Stopping,
                AiDeploymentState::Stopped,
            ),
        ];

        for (current, next) in transitions {
            assert!(
                is_transition_allowed(&current, &next),
                "{} -> {} should be permitted",
                current.as_str(),
                next.as_str()
            );
        }
    }

    #[test]
    fn update_and_rollback_paths_are_valid() {
        assert!(is_transition_allowed(
            &AiDeploymentState::Active,
            &AiDeploymentState::Updating,
        ));

        assert!(is_transition_allowed(
            &AiDeploymentState::Updating,
            &AiDeploymentState::Validating,
        ));

        assert!(is_transition_allowed(
            &AiDeploymentState::Updating,
            &AiDeploymentState::RollingBack {
                reason: "new release failed".to_owned(),
            },
        ));

        assert!(is_transition_allowed(
            &AiDeploymentState::RollingBack {
                reason: "restoring previous release".to_owned(),
            },
            &AiDeploymentState::Active,
        ));
    }

    #[test]
    fn degraded_deployment_has_explicit_recovery_paths() {
        let degraded = AiDeploymentState::Degraded {
            reason: "provider health check failed".to_owned(),
        };

        assert!(is_transition_allowed(
            &degraded,
            &AiDeploymentState::Updating,
        ));

        assert!(is_transition_allowed(
            &degraded,
            &AiDeploymentState::RollingBack {
                reason: "degradation requires rollback".to_owned(),
            },
        ));

        assert!(is_transition_allowed(
            &degraded,
            &AiDeploymentState::Stopping,
        ));
    }

    #[test]
    fn activation_before_deployment_is_rejected() {
        assert!(!is_transition_allowed(
            &AiDeploymentState::Registered,
            &AiDeploymentState::Active,
        ));

        assert!(!is_transition_allowed(
            &AiDeploymentState::Prepared,
            &AiDeploymentState::Active,
        ));
    }

    #[test]
    fn stopping_requires_confirmation_before_terminal_state() {
        assert!(!is_transition_allowed(
            &AiDeploymentState::Active,
            &AiDeploymentState::Stopped,
        ));

        assert!(is_transition_allowed(
            &AiDeploymentState::Active,
            &AiDeploymentState::Stopping,
        ));

        assert!(is_transition_allowed(
            &AiDeploymentState::Stopping,
            &AiDeploymentState::Stopped,
        ));
    }

    #[test]
    fn terminal_states_reject_all_transitions() {
        let terminal_states = [
            AiDeploymentState::Stopped,
            AiDeploymentState::Failed {
                error_code: "deployment.failed".to_owned(),
                message: "failure".to_owned(),
            },
            AiDeploymentState::Cancelled {
                reason: "cancelled".to_owned(),
            },
        ];

        let possible_next_states = [
            AiDeploymentState::Registered,
            AiDeploymentState::Validating,
            AiDeploymentState::Prepared,
            AiDeploymentState::Deploying,
            AiDeploymentState::Active,
            AiDeploymentState::Updating,
            AiDeploymentState::Stopping,
            AiDeploymentState::Stopped,
        ];

        for current in &terminal_states {
            for next in &possible_next_states {
                assert!(
                    !is_transition_allowed(current, next),
                    "{} must not transition to {}",
                    current.as_str(),
                    next.as_str()
                );
            }
        }
    }

    #[test]
    fn failure_is_permitted_from_nonterminal_states() {
        let states = [
            AiDeploymentState::Registered,
            AiDeploymentState::Validating,
            AiDeploymentState::Prepared,
            AiDeploymentState::Deploying,
            AiDeploymentState::Updating,
            AiDeploymentState::Degraded {
                reason: "provider issue".to_owned(),
            },
            AiDeploymentState::RollingBack {
                reason: "rollback issue".to_owned(),
            },
            AiDeploymentState::Stopping,
        ];

        let failure = AiDeploymentState::Failed {
            error_code: "deployment.failed".to_owned(),
            message: "operation failed".to_owned(),
        };

        for current in &states {
            assert!(
                is_transition_allowed(current, &failure),
                "{} should permit failure recording",
                current.as_str()
            );
        }
    }

    #[test]
    fn invalid_empty_reason_is_rejected() {
        assert!(validate_reason(String::new()).is_err());
        assert!(validate_reason("   ".to_owned()).is_err());
    }

    #[test]
    fn control_characters_in_reason_are_rejected() {
        assert!(
            validate_reason("invalid\nreason".to_owned()).is_err()
        );
    }

    #[test]
    fn state_identifiers_are_stable() {
        assert_eq!(
            AiDeploymentState::Registered.as_str(),
            "registered"
        );

        assert_eq!(
            AiDeploymentState::RollingBack {
                reason: "rollback".to_owned(),
            }
            .as_str(),
            "rolling_back"
        );

        assert_eq!(
            AiDeploymentState::Stopped.as_str(),
            "stopped"
        );
    }

    #[test]
    fn unknown_deployment_is_not_implicitly_created() {
        let lifecycle = AiDeploymentLifecycle::<String>::new();

        let result = lifecycle.begin_validation(
            &"unknown-deployment".to_owned(),
        );

        assert_eq!(
            result.unwrap_err(),
            AiDeploymentLifecycleError::NotRegistered
        );

        assert_eq!(
            lifecycle.deployment_count().unwrap(),
            0
        );
    }
}