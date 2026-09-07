//! # Zamani Quantum Resilience — Verified Learning Feedback
//!
//! Path:
//! `src/quantum/resilience/learning/feedback.rs`
//!
//! ## Purpose
//!
//! This module defines the production feedback boundary between verified
//! quantum-resilience execution outcomes and future learning/model consumers.
//!
//! The central rule is:
//!
//! > Only explicitly verified outcomes may become trusted learning feedback.
//!
//! Predictions are advisory evidence. Feedback is retrospective evidence.
//! Neither prediction nor feedback is itself authority to change quantum
//! program semantics, recovery policy, hardware selection, QEC configuration,
//! routing, scheduling, or result acceptance.
//!
//! This module therefore provides:
//!
//! - stable prediction references;
//! - verified outcome representation;
//! - feedback eligibility;
//! - feedback provenance;
//! - poisoning-resistance metadata;
//! - deterministic identity supplied by the caller;
//! - duplicate/replay protection;
//! - conflict detection;
//! - append-only feedback sinks;
//! - streaming-friendly feedback submission;
//! - batch submission without fixed batch limits;
//! - explicit validation;
//! - feedback statistics;
//! - model/task/schema association;
//! - quarantine of untrusted feedback;
//! - auditability;
//! - no hidden clock/randomness/global state.
//!
//! ## Architectural ownership
//!
//! This file owns:
//!
//! - verified learning feedback contracts;
//! - feedback identity;
//! - feedback provenance;
//! - feedback trust state;
//! - feedback validation;
//! - duplicate/conflict detection;
//! - feedback sink abstraction;
//! - feedback statistics;
//! - quarantine semantics.
//!
//! This file does NOT own:
//!
//! - feature extraction;
//! - model training;
//! - prediction;
//! - strategy selection;
//! - recovery;
//! - mitigation;
//! - semantic verification implementation;
//! - history persistence implementation;
//! - hardware discovery;
//! - routing;
//! - scheduling;
//! - QEC;
//! - canonical quantum IR.
//!
//! Those responsibilities remain in their respective subsystems.
//!
//! ## Integration
//!
//! ```text
//! learning/features.rs
//!        |
//!        v
//! learning/predictor.rs
//!        |
//!        v
//! PredictionRef
//!        |
//!        | execution
//!        v
//! quantum execution / recovery / mitigation / QEC
//!        |
//!        v
//! verification/verifier.rs
//!        |
//!        v
//! VerifiedOutcome
//!        |
//!        v
//! FeedbackRecord
//!        |
//!        v
//! FeedbackSink
//!        |
//!        +------------------> history/*
//!        |
//!        +------------------> learning/model training
//!        |
//!        +------------------> learning/strategy.rs
//! ```
//!
//! The learning subsystem MUST NOT consume a `FeedbackRecord` as trusted
//! training data until its [`FeedbackRecord::eligibility`] is
//! [`FeedbackEligibility::Eligible`].
//!
//! ## Security
//!
//! Feedback is an attractive model-poisoning target. Consequently:
//!
//! - unverified outcomes cannot become trusted feedback;
//! - rejected outcomes cannot become trusted feedback;
//! - conflicting duplicate feedback is detected;
//! - caller-supplied identities are retained;
//! - provenance is retained;
//! - model identity/version/schema are retained;
//! - trust status is explicit;
//! - quarantine is represented explicitly;
//! - this module never silently overwrites existing feedback;
//! - this module never silently merges conflicting observations.
//!
//! Cryptographic authentication/signatures belong to the surrounding security
//! and persistence layers. This module does not invent a cryptographic
//! primitive.
//!
//! ## Determinism
//!
//! This module does not:
//!
//! - read wall-clock time;
//! - access an operating-system random generator;
//! - generate hidden IDs;
//! - use global mutable state;
//! - depend on environment variables;
//! - reorder caller-provided feedback.
//!
//! Stable IDs/fingerprints MUST be supplied by the caller or by a deterministic
//! upstream component.
//!
//! ## Scalability
//!
//! There are no fixed limits on:
//!
//! - feedback records;
//! - executions;
//! - models;
//! - devices;
//! - qubits;
//! - logical qubits;
//! - physical qubits;
//! - batch size;
//! - provenance entries;
//! - feature references.
//!
//! Memory/storage limits are imposed by the selected sink, caller policy,
//! persistence backend, or available resources.
//!
//! A production deployment should normally use a streaming/persistent
//! [`FeedbackSink`] instead of retaining unbounded feedback in memory.
//!
//! ## Quantum identity
//!
//! This file does not create a quantum-resource identity.
//!
//! If feedback needs logical or physical quantum-resource identity, upstream
//! provenance MUST use the canonical types from:
//!
//! `crate::quantum::ir::qubit`
//!
//! In particular, this module MUST NOT introduce:
//!
//! - `FeedbackQubitId`;
//! - `LearningQubitId`;
//! - `ResilienceQubitId`;
//! - another logical/physical qubit identity type.
//!
//! ## Rust requirements
//!
//! - Rust 2021.
//! - Rust 1.97 / 1.97.1.
//! - Stable Rust.
//! - No nightly features.
//! - No `unsafe`.
//!
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler-enforced.

#![forbid(unsafe_code)]

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use super::model::{ModelId, ModelSchemaId, ModelVersion, PredictionTarget, PredictionTask};

// =============================================================================
// Result / error contract
// =============================================================================

/// Result type used by the feedback subsystem.
pub type FeedbackResult<T> = Result<T, FeedbackError>;

/// Errors produced by the feedback subsystem.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FeedbackError {
    /// A required field is invalid.
    InvalidField {
        /// Field name.
        field: String,

        /// Explanation.
        reason: String,
    },

    /// A feedback record is not valid for submission.
    InvalidRecord {
        /// Explanation.
        reason: String,
    },

    /// Feedback is not trusted enough for submission to the trusted stream.
    NotEligible {
        /// Current eligibility.
        eligibility: FeedbackEligibility,
    },

    /// A feedback identity already exists with equivalent content.
    Duplicate {
        /// Stable feedback identifier.
        feedback_id: FeedbackId,
    },

    /// A feedback identity already exists with different content.
    Conflict {
        /// Stable feedback identifier.
        feedback_id: FeedbackId,

        /// Explanation.
        reason: String,
    },

    /// A referenced model is incompatible with the supplied feedback.
    ModelMismatch {
        /// Explanation.
        reason: String,
    },

    /// A provenance requirement was not satisfied.
    ProvenanceFailure {
        /// Explanation.
        reason: String,
    },

    /// A feedback sink rejected the record.
    SinkRejected {
        /// Explanation.
        reason: String,
    },
}

impl fmt::Display for FeedbackError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidField { field, reason } => {
                write!(f, "invalid feedback field `{field}`: {reason}")
            }
            Self::InvalidRecord { reason } => {
                write!(f, "invalid feedback record: {reason}")
            }
            Self::NotEligible { eligibility } => {
                write!(f, "feedback is not eligible: {eligibility:?}")
            }
            Self::Duplicate { feedback_id } => {
                write!(f, "duplicate feedback record `{feedback_id}`")
            }
            Self::Conflict {
                feedback_id,
                reason,
            } => {
                write!(
                    f,
                    "conflicting feedback for `{feedback_id}`: {reason}"
                )
            }
            Self::ModelMismatch { reason } => {
                write!(f, "feedback model mismatch: {reason}")
            }
            Self::ProvenanceFailure { reason } => {
                write!(f, "feedback provenance failure: {reason}")
            }
            Self::SinkRejected { reason } => {
                write!(f, "feedback sink rejected record: {reason}")
            }
        }
    }
}

impl Error for FeedbackError {}

// =============================================================================
// Stable identifiers
// =============================================================================

/// Stable identity for a feedback record.
///
/// The identifier MUST be supplied by a deterministic caller or generated by
/// a deterministic upstream identity system. This module deliberately does not
/// generate random IDs.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct FeedbackId(String);

impl FeedbackId {
    /// Creates a validated feedback identifier.
    pub fn new<S: Into<String>>(value: S) -> FeedbackResult<Self> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(FeedbackError::InvalidField {
                field: "feedback_id".to_owned(),
                reason: "identifier must not be empty".to_owned(),
            });
        }

        Ok(Self(value))
    }

    /// Returns the identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for FeedbackId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// Stable identity for the execution that produced an outcome.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExecutionId(String);

impl ExecutionId {
    /// Creates a validated execution identifier.
    pub fn new<S: Into<String>>(value: S) -> FeedbackResult<Self> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(FeedbackError::InvalidField {
                field: "execution_id".to_owned(),
                reason: "identifier must not be empty".to_owned(),
            });
        }

        Ok(Self(value))
    }

    /// Returns the identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ExecutionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// Stable identity for a verification decision.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct VerificationId(String);

impl VerificationId {
    /// Creates a validated verification identifier.
    pub fn new<S: Into<String>>(value: S) -> FeedbackResult<Self> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(FeedbackError::InvalidField {
                field: "verification_id".to_owned(),
                reason: "identifier must not be empty".to_owned(),
            });
        }

        Ok(Self(value))
    }

    /// Returns the identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for VerificationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// Stable identity for the model prediction associated with an outcome.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PredictionId(String);

impl PredictionId {
    /// Creates a validated prediction identifier.
    pub fn new<S: Into<String>>(value: S) -> FeedbackResult<Self> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(FeedbackError::InvalidField {
                field: "prediction_id".to_owned(),
                reason: "identifier must not be empty".to_owned(),
            });
        }

        Ok(Self(value))
    }

    /// Returns the identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for PredictionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// Stable fingerprint of the complete feedback payload.
///
/// This is intentionally caller-supplied. A security layer may use a
/// cryptographic digest here. The feedback layer does not dictate a hashing
/// algorithm.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct FeedbackFingerprint(String);

impl FeedbackFingerprint {
    /// Creates a validated fingerprint.
    pub fn new<S: Into<String>>(value: S) -> FeedbackResult<Self> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(FeedbackError::InvalidField {
                field: "fingerprint".to_owned(),
                reason: "fingerprint must not be empty".to_owned(),
            });
        }

        Ok(Self(value))
    }

    /// Returns the fingerprint.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for FeedbackFingerprint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

// =============================================================================
// Prediction reference
// =============================================================================

/// Immutable reference to the prediction that preceded an observed outcome.
///
/// This deliberately contains model identity/version/schema rather than a
/// copied prediction representation. The canonical prediction remains owned
/// by `learning/model.rs` / `learning/predictor.rs`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PredictionReference {
    /// Prediction identity.
    pub prediction_id: PredictionId,

    /// Model family identity.
    pub model_id: ModelId,

    /// Exact model version used.
    pub model_version: ModelVersion,

    /// Feature/input schema consumed by the model.
    pub schema_id: ModelSchemaId,

    /// Prediction task.
    pub task: PredictionTask,

    /// Expected interpretation of the principal prediction value.
    pub target: PredictionTarget,
}

impl PredictionReference {
    /// Creates a validated prediction reference.
    #[must_use]
    pub fn new(
        prediction_id: PredictionId,
        model_id: ModelId,
        model_version: ModelVersion,
        schema_id: ModelSchemaId,
        task: PredictionTask,
        target: PredictionTarget,
    ) -> Self {
        Self {
            prediction_id,
            model_id,
            model_version,
            schema_id,
            task,
            target,
        }
    }
}

// =============================================================================
// Verification / eligibility
// =============================================================================

/// Trust state of an observed outcome.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VerificationStatus {
    /// Verification has not completed.
    Pending,

    /// Outcome passed semantic/acceptance verification.
    Verified,

    /// Outcome was explicitly rejected.
    Rejected,

    /// Verification failed or could not establish correctness.
    Inconclusive,
}

impl VerificationStatus {
    /// Returns whether the status is trusted for learning.
    #[must_use]
    pub const fn is_verified(self) -> bool {
        matches!(self, Self::Verified)
    }
}

/// Eligibility of a feedback record for trusted learning.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FeedbackEligibility {
    /// Feedback is not yet ready.
    PendingVerification,

    /// Feedback is eligible for trusted learning.
    Eligible,

    /// Feedback was rejected by verification.
    Rejected,

    /// Verification could not establish trust.
    Inconclusive,

    /// Feedback is quarantined for security/manual review.
    Quarantined,

    /// Feedback was invalidated after previously being accepted.
    Revoked,
}

impl FeedbackEligibility {
    /// Returns whether the feedback may enter the trusted learning stream.
    #[must_use]
    pub const fn is_trusted(self) -> bool {
        matches!(self, Self::Eligible)
    }
}

/// Reason why a feedback record is eligible or ineligible.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EligibilityReason {
    /// Verification succeeded and all required provenance is present.
    Verified,

    /// Verification has not completed.
    VerificationPending,

    /// Verification rejected the outcome.
    VerificationRejected,

    /// Verification could not establish correctness.
    VerificationInconclusive,

    /// Required provenance was absent.
    MissingProvenance,

    /// Security policy quarantined the record.
    SecurityQuarantine,

    /// Record was explicitly revoked.
    Revoked,

    /// Caller marked the record as ineligible.
    CallerRejected,

    /// Extension-defined reason.
    Custom(String),
}

// =============================================================================
// Outcome representation
// =============================================================================

/// Generic scalar outcome value.
///
/// Values are deliberately represented without assuming a particular quantum
/// result format. Quantum measurement distributions, logical error rates,
/// fidelities, latency, and other observations may be represented as named
/// scalar values or categories.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OutcomeValue {
    /// Finite floating-point value.
    Scalar(f64),

    /// Boolean observation.
    Boolean(bool),

    /// Categorical/string observation.
    Category(String),

    /// Integer observation.
    Integer(i64),

    /// Unsigned integer observation.
    Unsigned(u64),
}

impl OutcomeValue {
    /// Returns whether a scalar is finite.
    #[must_use]
    pub fn is_finite(&self) -> bool {
        match self {
            Self::Scalar(value) => value.is_finite(),
            Self::Boolean(_)
            | Self::Category(_)
            | Self::Integer(_)
            | Self::Unsigned(_) => true,
        }
    }
}

/// Named observed outcome.
///
/// Multiple outcome values are supported so one verified execution may provide
/// several labels without creating one feedback record per metric.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObservedOutcome {
    /// Stable outcome name.
    pub name: String,

    /// Observed value.
    pub value: OutcomeValue,

    /// Optional semantic unit.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

impl ObservedOutcome {
    /// Creates an observed outcome.
    pub fn new<S: Into<String>>(name: S, value: OutcomeValue) -> FeedbackResult<Self> {
        let name = name.into();

        if name.trim().is_empty() {
            return Err(FeedbackError::InvalidField {
                field: "outcome.name".to_owned(),
                reason: "outcome name must not be empty".to_owned(),
            });
        }

        if !value.is_finite() {
            return Err(FeedbackError::InvalidField {
                field: "outcome.value".to_owned(),
                reason: "floating-point outcome must be finite".to_owned(),
            });
        }

        Ok(Self {
            name,
            value,
            unit: None,
        })
    }

    /// Adds an optional unit.
    #[must_use]
    pub fn with_unit<S: Into<String>>(mut self, unit: S) -> Self {
        self.unit = Some(unit.into());
        self
    }
}

// =============================================================================
// Provenance
// =============================================================================

/// Provenance of a feedback record.
///
/// This is intentionally generic and does not depend on concrete history,
/// hardware, QEC, routing, scheduling, or runtime implementations.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FeedbackProvenance {
    /// Stable execution identity.
    pub execution_id: ExecutionId,

    /// Verification decision identity.
    pub verification_id: VerificationId,

    /// Original prediction identity.
    pub prediction_id: PredictionId,

    /// Optional canonical program/IR identity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub program_fingerprint: Option<String>,

    /// Optional canonical IR identity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ir_fingerprint: Option<String>,

    /// Optional hardware/device identity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_identity: Option<String>,

    /// Optional execution configuration identity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execution_configuration: Option<String>,

    /// Optional resilience-policy identity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy_identity: Option<String>,

    /// Optional calibration snapshot identity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub calibration_identity: Option<String>,

    /// Optional QEC configuration identity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub qec_configuration: Option<String>,

    /// Optional routing identity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub routing_identity: Option<String>,

    /// Optional scheduling identity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule_identity: Option<String>,

    /// Optional optimization identity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optimization_identity: Option<String>,

    /// Additional deterministic provenance attributes.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub attributes: BTreeMap<String, String>,
}

impl FeedbackProvenance {
    /// Creates the minimum provenance required by the feedback contract.
    #[must_use]
    pub fn new(
        execution_id: ExecutionId,
        verification_id: VerificationId,
        prediction_id: PredictionId,
    ) -> Self {
        Self {
            execution_id,
            verification_id,
            prediction_id,
            program_fingerprint: None,
            ir_fingerprint: None,
            target_identity: None,
            execution_configuration: None,
            policy_identity: None,
            calibration_identity: None,
            qec_configuration: None,
            routing_identity: None,
            schedule_identity: None,
            optimization_identity: None,
            attributes: BTreeMap::new(),
        }
    }

    /// Returns whether minimum provenance is present.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        !self.execution_id.as_str().trim().is_empty()
            && !self.verification_id.as_str().trim().is_empty()
            && !self.prediction_id.as_str().trim().is_empty()
    }
}

// =============================================================================
// Feedback record
// =============================================================================

/// A complete verified learning-feedback record.
///
/// The record is immutable after construction from the perspective of the
/// feedback contract. If an outcome must later be revoked, a new revocation
/// event should be recorded by the persistence/audit layer rather than
/// mutating historical evidence invisibly.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeedbackRecord {
    /// Stable feedback identity.
    pub feedback_id: FeedbackId,

    /// Caller/security-layer fingerprint of the complete record.
    pub fingerprint: FeedbackFingerprint,

    /// Prediction that produced the hypothesis being evaluated.
    pub prediction: PredictionReference,

    /// Verified execution provenance.
    pub provenance: FeedbackProvenance,

    /// Verification result.
    pub verification_status: VerificationStatus,

    /// Eligibility for trusted learning.
    pub eligibility: FeedbackEligibility,

    /// Explanation for the eligibility state.
    pub eligibility_reason: EligibilityReason,

    /// Observed outcome values.
    pub outcomes: Vec<ObservedOutcome>,

    /// Optional error/deviation measurements.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<ObservedOutcome>,

    /// Optional strategy/action identity that was evaluated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub strategy_identity: Option<String>,

    /// Optional recovery-plan identity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recovery_identity: Option<String>,

    /// Optional mitigation identity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mitigation_identity: Option<String>,

    /// Optional deterministic sample/run identity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sample_identity: Option<String>,

    /// Additional non-sensitive metadata.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub attributes: BTreeMap<String, String>,
}

impl FeedbackRecord {
    /// Creates a feedback record.
    #[must_use]
    pub fn new(
        feedback_id: FeedbackId,
        fingerprint: FeedbackFingerprint,
        prediction: PredictionReference,
        provenance: FeedbackProvenance,
        verification_status: VerificationStatus,
        eligibility: FeedbackEligibility,
        eligibility_reason: EligibilityReason,
        outcomes: Vec<ObservedOutcome>,
    ) -> Self {
        Self {
            feedback_id,
            fingerprint,
            prediction,
            provenance,
            verification_status,
            eligibility,
            eligibility_reason,
            outcomes,
            errors: Vec::new(),
            strategy_identity: None,
            recovery_identity: None,
            mitigation_identity: None,
            sample_identity: None,
            attributes: BTreeMap::new(),
        }
    }

    /// Adds an observed error/deviation value.
    #[must_use]
    pub fn with_error(mut self, error: ObservedOutcome) -> Self {
        self.errors.push(error);
        self
    }

    /// Associates a strategy with the feedback.
    #[must_use]
    pub fn with_strategy<S: Into<String>>(mut self, strategy: S) -> Self {
        self.strategy_identity = Some(strategy.into());
        self
    }

    /// Associates a recovery operation with the feedback.
    #[must_use]
    pub fn with_recovery<S: Into<String>>(mut self, recovery: S) -> Self {
        self.recovery_identity = Some(recovery.into());
        self
    }

    /// Associates a mitigation strategy with the feedback.
    #[must_use]
    pub fn with_mitigation<S: Into<String>>(mut self, mitigation: S) -> Self {
        self.mitigation_identity = Some(mitigation.into());
        self
    }

    /// Associates a sample identity.
    #[must_use]
    pub fn with_sample<S: Into<String>>(mut self, sample: S) -> Self {
        self.sample_identity = Some(sample.into());
        self
    }

    /// Returns whether this record is trusted feedback.
    #[must_use]
    pub const fn is_trusted(&self) -> bool {
        self.eligibility.is_trusted() && self.verification_status.is_verified()
    }

    /// Validates the complete record.
    pub fn validate(&self) -> FeedbackResult<()> {
        if self.feedback_id.as_str().trim().is_empty() {
            return Err(FeedbackError::InvalidRecord {
                reason: "feedback ID must not be empty".to_owned(),
            });
        }

        if self.fingerprint.as_str().trim().is_empty() {
            return Err(FeedbackError::InvalidRecord {
                reason: "feedback fingerprint must not be empty".to_owned(),
            });
        }

        if !self.provenance.is_complete() {
            return Err(FeedbackError::ProvenanceFailure {
                reason: "execution, verification, and prediction provenance are required"
                    .to_owned(),
            });
        }

        if self.provenance.prediction_id != self.prediction.prediction_id {
            return Err(FeedbackError::ProvenanceFailure {
                reason: "provenance prediction ID does not match prediction reference".to_owned(),
            });
        }

        if self.verification_status.is_verified() != self.eligibility.is_trusted() {
            return Err(FeedbackError::InvalidRecord {
                reason: "verified/trusted state is inconsistent".to_owned(),
            });
        }

        if self.outcomes.is_empty() {
            return Err(FeedbackError::InvalidRecord {
                reason: "at least one observed outcome is required".to_owned(),
            });
        }

        validate_unique_outcome_names(&self.outcomes)?;
        validate_unique_outcome_names(&self.errors)?;

        for outcome in self.outcomes.iter().chain(self.errors.iter()) {
            if !outcome.value.is_finite() {
                return Err(FeedbackError::InvalidRecord {
                    reason: format!("outcome `{}` is not finite", outcome.name),
                });
            }
        }

        Ok(())
    }
}

// =============================================================================
// Submission
// =============================================================================

/// Result of submitting one feedback record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FeedbackSubmission {
    /// New record was accepted.
    Accepted,

    /// Record was already present with identical fingerprint.
    AlreadyPresent,

    /// Record was quarantined rather than trusted.
    Quarantined,
}

/// Result of submitting a batch.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FeedbackBatchSubmission {
    /// Number of records accepted.
    pub accepted: usize,

    /// Number of records already present.
    pub already_present: usize,

    /// Number quarantined.
    pub quarantined: usize,

    /// Number rejected before reaching the sink.
    pub rejected: usize,
}

impl FeedbackBatchSubmission {
    /// Creates an empty submission result.
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            accepted: 0,
            already_present: 0,
            quarantined: 0,
            rejected: 0,
        }
    }

    /// Total records processed.
    #[must_use]
    pub const fn total(&self) -> usize {
        self.accepted + self.already_present + self.quarantined + self.rejected
    }
}

// =============================================================================
// Feedback sink
// =============================================================================

/// Provider-independent persistence/streaming boundary for feedback.
///
/// Implementations may write to:
///
/// - in-memory storage;
/// - a local database;
/// - an append-only journal;
/// - a distributed store;
/// - a training-data pipeline;
/// - an encrypted persistence layer.
///
/// The feedback core does not choose the persistence technology.
pub trait FeedbackSink {
    /// Submit one feedback record.
    fn submit(&mut self, record: FeedbackRecord) -> FeedbackResult<FeedbackSubmission>;

    /// Returns whether a record identity is already known.
    fn contains(&self, feedback_id: &FeedbackId) -> FeedbackResult<bool>;
}

/// Sink wrapper that enforces validation and trusted-feedback rules before
/// forwarding records to another sink.
pub struct ValidatingFeedbackSink<S> {
    inner: S,
    allow_quarantine: bool,
}

impl<S> ValidatingFeedbackSink<S>
where
    S: FeedbackSink,
{
    /// Creates a validating sink.
    #[must_use]
    pub const fn new(inner: S) -> Self {
        Self {
            inner,
            allow_quarantine: true,
        }
    }

    /// Configures whether untrusted records may be forwarded as quarantined
    /// records.
    #[must_use]
    pub const fn with_quarantine(mut self, enabled: bool) -> Self {
        self.allow_quarantine = enabled;
        self
    }

    /// Returns a reference to the wrapped sink.
    #[must_use]
    pub const fn inner(&self) -> &S {
        &self.inner
    }

    /// Returns a mutable reference to the wrapped sink.
    #[must_use]
    pub const fn inner_mut(&mut self) -> &mut S {
        &mut self.inner
    }

    /// Consumes the wrapper and returns the wrapped sink.
    #[must_use]
    pub fn into_inner(self) -> S {
        self.inner
    }
}

impl<S> FeedbackSink for ValidatingFeedbackSink<S>
where
    S: FeedbackSink,
{
    fn submit(&mut self, record: FeedbackRecord) -> FeedbackResult<FeedbackSubmission> {
        record.validate()?;

        if !record.is_trusted() && !self.allow_quarantine {
            return Err(FeedbackError::NotEligible {
                eligibility: record.eligibility,
            });
        }

        if !record.is_trusted() {
            let mut quarantined = record;
            quarantined.eligibility = FeedbackEligibility::Quarantined;
            quarantined.eligibility_reason = EligibilityReason::SecurityQuarantine;
            return self.inner.submit(quarantined);
        }

        self.inner.submit(record)
    }

    fn contains(&self, feedback_id: &FeedbackId) -> FeedbackResult<bool> {
        self.inner.contains(feedback_id)
    }
}

// =============================================================================
// In-memory sink
// =============================================================================

/// Deterministic in-memory feedback sink.
///
/// This implementation is intended for:
///
/// - unit tests;
/// - simulations;
/// - deterministic replay;
/// - small deployments.
///
/// It imposes no artificial record count. Its actual capacity is therefore
/// bounded only by available process memory.
///
/// Production deployments with large or unbounded histories should use a
/// persistent/streaming implementation of [`FeedbackSink`].
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct InMemoryFeedbackSink {
    records: BTreeMap<FeedbackId, FeedbackRecord>,
    quarantined: BTreeMap<FeedbackId, FeedbackRecord>,
}

impl InMemoryFeedbackSink {
    /// Creates an empty sink.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the number of trusted records.
    #[must_use]
    pub fn len(&self) -> usize {
        self.records.len()
    }

    /// Returns whether no trusted records exist.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    /// Returns the number of quarantined records.
    #[must_use]
    pub fn quarantined_len(&self) -> usize {
        self.quarantined.len()
    }

    /// Returns a trusted record by identity.
    #[must_use]
    pub fn get(&self, id: &FeedbackId) -> Option<&FeedbackRecord> {
        self.records.get(id)
    }

    /// Returns a quarantined record by identity.
    #[must_use]
    pub fn get_quarantined(&self, id: &FeedbackId) -> Option<&FeedbackRecord> {
        self.quarantined.get(id)
    }

    /// Iterates over trusted records in deterministic ID order.
    pub fn iter(&self) -> impl Iterator<Item = &FeedbackRecord> {
        self.records.values()
    }

    /// Iterates over quarantined records in deterministic ID order.
    pub fn iter_quarantined(&self) -> impl Iterator<Item = &FeedbackRecord> {
        self.quarantined.values()
    }
}

impl FeedbackSink for InMemoryFeedbackSink {
    fn submit(&mut self, record: FeedbackRecord) -> FeedbackResult<FeedbackSubmission> {
        record.validate()?;

        let id = record.feedback_id.clone();

        if record.is_trusted() {
            if let Some(existing) = self.records.get(&id) {
                if existing.fingerprint == record.fingerprint {
                    return Ok(FeedbackSubmission::AlreadyPresent);
                }

                return Err(FeedbackError::Conflict {
                    feedback_id: id,
                    reason: "same feedback ID has a different fingerprint".to_owned(),
                });
            }

            self.records.insert(id, record);
            Ok(FeedbackSubmission::Accepted)
        } else {
            if let Some(existing) = self.quarantined.get(&id) {
                if existing.fingerprint == record.fingerprint {
                    return Ok(FeedbackSubmission::AlreadyPresent);
                }

                return Err(FeedbackError::Conflict {
                    feedback_id: id,
                    reason: "same quarantined feedback ID has a different fingerprint"
                        .to_owned(),
                });
            }

            self.quarantined.insert(id, record);
            Ok(FeedbackSubmission::Quarantined)
        }
    }

    fn contains(&self, feedback_id: &FeedbackId) -> FeedbackResult<bool> {
        Ok(self.records.contains_key(feedback_id)
            || self.quarantined.contains_key(feedback_id))
    }
}

// =============================================================================
// Feedback collector
// =============================================================================

/// Orchestrates validation and submission without owning persistence.
///
/// The collector is intentionally generic over [`FeedbackSink`], allowing
/// streaming/persistent implementations without modifying this module.
pub struct FeedbackCollector<S> {
    sink: S,
}

impl<S> FeedbackCollector<S>
where
    S: FeedbackSink,
{
    /// Creates a collector.
    #[must_use]
    pub const fn new(sink: S) -> Self {
        Self { sink }
    }

    /// Returns a reference to the sink.
    #[must_use]
    pub const fn sink(&self) -> &S {
        &self.sink
    }

    /// Returns a mutable reference to the sink.
    #[must_use]
    pub const fn sink_mut(&mut self) -> &mut S {
        &mut self.sink
    }

    /// Consumes the collector and returns its sink.
    #[must_use]
    pub fn into_sink(self) -> S {
        self.sink
    }

    /// Submits one record.
    pub fn submit(&mut self, record: FeedbackRecord) -> FeedbackResult<FeedbackSubmission> {
        record.validate()?;
        self.sink.submit(record)
    }

    /// Submits records sequentially, preserving caller order.
    ///
    /// No fixed batch size is imposed. A production caller should still choose
    /// a batch size appropriate to its available resources and persistence
    /// backend.
    pub fn submit_batch<I>(&mut self, records: I) -> FeedbackResult<FeedbackBatchSubmission>
    where
        I: IntoIterator<Item = FeedbackRecord>,
    {
        let mut result = FeedbackBatchSubmission::empty();

        for record in records {
            match self.submit(record) {
                Ok(FeedbackSubmission::Accepted) => {
                    result.accepted += 1;
                }
                Ok(FeedbackSubmission::AlreadyPresent) => {
                    result.already_present += 1;
                }
                Ok(FeedbackSubmission::Quarantined) => {
                    result.quarantined += 1;
                }
                Err(_) => {
                    result.rejected += 1;
                }
            }
        }

        Ok(result)
    }
}

// =============================================================================
// Statistics
// =============================================================================

/// Deterministic aggregate statistics over feedback.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FeedbackStatistics {
    /// Total trusted records.
    pub trusted_records: usize,

    /// Total quarantined records.
    pub quarantined_records: usize,

    /// Distinct models represented.
    pub distinct_models: usize,

    /// Distinct prediction tasks represented.
    pub distinct_tasks: usize,

    /// Distinct strategies represented.
    pub distinct_strategies: usize,

    /// Number of verified records.
    pub verified_records: usize,

    /// Number of records with incomplete/inconclusive verification.
    pub inconclusive_records: usize,
}

impl FeedbackStatistics {
    /// Creates statistics from an iterator of feedback records.
    pub fn from_records<'a, I>(records: I) -> Self
    where
        I: IntoIterator<Item = &'a FeedbackRecord>,
    {
        let mut models = BTreeSet::new();
        let mut tasks = BTreeSet::new();
        let mut strategies = BTreeSet::new();

        let mut trusted_records = 0usize;
        let mut verified_records = 0usize;
        let mut inconclusive_records = 0usize;

        for record in records {
            if record.is_trusted() {
                trusted_records += 1;
            }

            if record.verification_status.is_verified() {
                verified_records += 1;
            }

            if matches!(
                record.verification_status,
                VerificationStatus::Pending | VerificationStatus::Inconclusive
            ) {
                inconclusive_records += 1;
            }

            models.insert(record.prediction.model_id.clone());
            tasks.insert(record.prediction.task.clone());

            if let Some(strategy) = &record.strategy_identity {
                strategies.insert(strategy.clone());
            }
        }

        Self {
            trusted_records,
            quarantined_records: 0,
            distinct_models: models.len(),
            distinct_tasks: tasks.len(),
            distinct_strategies: strategies.len(),
            verified_records,
            inconclusive_records,
        }
    }

    /// Adds the quarantined count from a sink.
    #[must_use]
    pub const fn with_quarantined(mut self, count: usize) -> Self {
        self.quarantined_records = count;
        self
    }
}

// =============================================================================
// Validation helpers
// =============================================================================

fn validate_unique_outcome_names(outcomes: &[ObservedOutcome]) -> FeedbackResult<()> {
    let mut names = BTreeSet::new();

    for outcome in outcomes {
        if !names.insert(outcome.name.as_str()) {
            return Err(FeedbackError::InvalidRecord {
                reason: format!("duplicate outcome name `{}`", outcome.name),
            });
        }
    }

    Ok(())
}

// =============================================================================
// Convenience constructors for verified feedback
// =============================================================================

/// Builds trusted feedback after verification.
///
/// This constructor intentionally requires [`VerificationStatus::Verified`]
/// and [`FeedbackEligibility::Eligible`] explicitly. It prevents callers from
/// accidentally converting an unverified execution into trusted training data.
pub fn verified_feedback(
    feedback_id: FeedbackId,
    fingerprint: FeedbackFingerprint,
    prediction: PredictionReference,
    provenance: FeedbackProvenance,
    outcomes: Vec<ObservedOutcome>,
) -> FeedbackResult<FeedbackRecord> {
    let record = FeedbackRecord::new(
        feedback_id,
        fingerprint,
        prediction,
        provenance,
        VerificationStatus::Verified,
        FeedbackEligibility::Eligible,
        EligibilityReason::Verified,
        outcomes,
    );

    record.validate()?;

    Ok(record)
}

/// Builds quarantined feedback.
///
/// Quarantined feedback is retained for audit/review but is not trusted
/// training data.
pub fn quarantined_feedback(
    feedback_id: FeedbackId,
    fingerprint: FeedbackFingerprint,
    prediction: PredictionReference,
    provenance: FeedbackProvenance,
    verification_status: VerificationStatus,
    reason: EligibilityReason,
    outcomes: Vec<ObservedOutcome>,
) -> FeedbackResult<FeedbackRecord> {
    if verification_status.is_verified() {
        return Err(FeedbackError::InvalidRecord {
            reason: "verified outcomes must use trusted feedback construction".to_owned(),
        });
    }

    let eligibility = match verification_status {
        VerificationStatus::Pending => FeedbackEligibility::PendingVerification,
        VerificationStatus::Rejected => FeedbackEligibility::Rejected,
        VerificationStatus::Inconclusive => FeedbackEligibility::Inconclusive,
        VerificationStatus::Verified => FeedbackEligibility::Eligible,
    };

    let record = FeedbackRecord::new(
        feedback_id,
        fingerprint,
        prediction,
        provenance,
        verification_status,
        eligibility,
        reason,
        outcomes,
    );

    record.validate()?;

    Ok(record)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn prediction_reference() -> PredictionReference {
        PredictionReference::new(
            PredictionId::new("prediction-1").expect("valid prediction ID"),
            ModelId::new("failure-model").expect("valid model ID"),
            ModelVersion::new(1, 0, 0),
            ModelSchemaId::new("resilience.features.v1").expect("valid schema"),
            PredictionTask::FailureProbability,
            PredictionTarget::Probability,
        )
    }

    fn provenance() -> FeedbackProvenance {
        FeedbackProvenance::new(
            ExecutionId::new("execution-1").expect("valid execution ID"),
            VerificationId::new("verification-1").expect("valid verification ID"),
            PredictionId::new("prediction-1").expect("valid prediction ID"),
        )
    }

    fn outcome() -> ObservedOutcome {
        ObservedOutcome::new(
            "failure",
            OutcomeValue::Boolean(false),
        )
        .expect("valid outcome")
    }

    fn trusted_record() -> FeedbackRecord {
        verified_feedback(
            FeedbackId::new("feedback-1").expect("valid feedback ID"),
            FeedbackFingerprint::new("sha256:test").expect("valid fingerprint"),
            prediction_reference(),
            provenance(),
            vec![outcome()],
        )
        .expect("valid feedback")
    }

    #[test]
    fn verified_feedback_is_trusted() {
        let record = trusted_record();

        assert!(record.is_trusted());
        assert_eq!(
            record.verification_status,
            VerificationStatus::Verified
        );
        assert_eq!(
            record.eligibility,
            FeedbackEligibility::Eligible
        );
    }

    #[test]
    fn incomplete_provenance_is_rejected() {
        let mut record = trusted_record();
        record.provenance.prediction_id =
            PredictionId::new("different-prediction").expect("valid ID");

        let result = record.validate();

        assert!(matches!(
            result,
            Err(FeedbackError::ProvenanceFailure { .. })
        ));
    }

    #[test]
    fn duplicate_identical_feedback_is_idempotent() {
        let mut sink = InMemoryFeedbackSink::new();

        let first = sink
            .submit(trusted_record())
            .expect("first submission succeeds");

        let second = sink
            .submit(trusted_record())
            .expect("duplicate submission succeeds");

        assert_eq!(first, FeedbackSubmission::Accepted);
        assert_eq!(second, FeedbackSubmission::AlreadyPresent);
        assert_eq!(sink.len(), 1);
    }

    #[test]
    fn conflicting_feedback_is_rejected() {
        let mut sink = InMemoryFeedbackSink::new();

        let first = trusted_record();
        sink.submit(first).expect("first submission succeeds");

        let mut conflicting = trusted_record();
        conflicting.fingerprint =
            FeedbackFingerprint::new("different-fingerprint").expect("valid fingerprint");

        let result = sink.submit(conflicting);

        assert!(matches!(result, Err(FeedbackError::Conflict { .. })));
        assert_eq!(sink.len(), 1);
    }

    #[test]
    fn unverified_feedback_is_not_trusted() {
        let record = quarantined_feedback(
            FeedbackId::new("feedback-pending").expect("valid feedback ID"),
            FeedbackFingerprint::new("sha256:pending").expect("valid fingerprint"),
            prediction_reference(),
            provenance(),
            VerificationStatus::Pending,
            EligibilityReason::VerificationPending,
            vec![outcome()],
        )
        .expect("valid pending feedback");

        assert!(!record.is_trusted());
        assert_eq!(
            record.eligibility,
            FeedbackEligibility::PendingVerification
        );
    }

    #[test]
    fn quarantined_feedback_is_not_added_to_trusted_store() {
        let mut sink = InMemoryFeedbackSink::new();

        let record = quarantined_feedback(
            FeedbackId::new("feedback-pending").expect("valid feedback ID"),
            FeedbackFingerprint::new("sha256:pending").expect("valid fingerprint"),
            prediction_reference(),
            provenance(),
            VerificationStatus::Pending,
            EligibilityReason::VerificationPending,
            vec![outcome()],
        )
        .expect("valid pending feedback");

        let result = sink.submit(record).expect("submission succeeds");

        assert_eq!(result, FeedbackSubmission::Quarantined);
        assert_eq!(sink.len(), 0);
        assert_eq!(sink.quarantined_len(), 1);
    }

    #[test]
    fn duplicate_outcome_names_are_rejected() {
        let mut record = trusted_record();

        record.outcomes.push(
            ObservedOutcome::new(
                "failure",
                OutcomeValue::Boolean(true),
            )
            .expect("valid outcome"),
        );

        assert!(matches!(
            record.validate(),
            Err(FeedbackError::InvalidRecord { .. })
        ));
    }

    #[test]
    fn non_finite_scalar_is_rejected() {
        let result = ObservedOutcome::new(
            "fidelity",
            OutcomeValue::Scalar(f64::NAN),
        );

        assert!(matches!(
            result,
            Err(FeedbackError::InvalidField { .. })
        ));
    }

    #[test]
    fn batch_submission_preserves_all_records() {
        let mut collector = FeedbackCollector::new(InMemoryFeedbackSink::new());

        let first = trusted_record();

        let mut second = trusted_record();
        second.feedback_id =
            FeedbackId::new("feedback-2").expect("valid feedback ID");
        second.fingerprint =
            FeedbackFingerprint::new("sha256:second").expect("valid fingerprint");

        let result = collector
            .submit_batch(vec![first, second])
            .expect("batch succeeds");

        assert_eq!(result.accepted, 2);
        assert_eq!(result.total(), 2);
        assert_eq!(collector.sink().len(), 2);
    }

    #[test]
    fn statistics_are_deterministic() {
        let first = trusted_record();

        let mut second = trusted_record();
        second.feedback_id =
            FeedbackId::new("feedback-2").expect("valid feedback ID");
        second.fingerprint =
            FeedbackFingerprint::new("sha256:second").expect("valid fingerprint");
        second.strategy_identity = Some("reroute".to_owned());

        let statistics =
            FeedbackStatistics::from_records([&first, &second]);

        assert_eq!(statistics.trusted_records, 2);
        assert_eq!(statistics.verified_records, 2);
        assert_eq!(statistics.distinct_models, 1);
        assert_eq!(statistics.distinct_tasks, 1);
        assert_eq!(statistics.distinct_strategies, 1);
    }

    #[test]
    fn prediction_and_provenance_must_reference_same_prediction() {
        let mut provenance = provenance();
        provenance.prediction_id =
            PredictionId::new("different").expect("valid ID");

        let result = verified_feedback(
            FeedbackId::new("feedback-1").expect("valid ID"),
            FeedbackFingerprint::new("fingerprint").expect("valid fingerprint"),
            prediction_reference(),
            provenance,
            vec![outcome()],
        );

        assert!(matches!(
            result,
            Err(FeedbackError::ProvenanceFailure { .. })
        ));
    }
}