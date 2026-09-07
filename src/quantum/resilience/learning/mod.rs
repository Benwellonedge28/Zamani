//! # Zamani Quantum Resilience — Learning Subsystem
//!
//! Path:
//! `src/quantum/resilience/learning/mod.rs`
//!
//! ## Purpose
//!
//! This module is the public module boundary for the quantum-resilience
//! learning subsystem.
//!
//! The learning subsystem provides provider-independent contracts for:
//!
//! - deterministic feature construction;
//! - prediction-model metadata and contracts;
//! - prediction orchestration;
//! - learned resilience-strategy ranking.
//!
//! It deliberately does NOT make machine learning mandatory for quantum
//! execution. Learning is an optional advisory layer above the authoritative
//! resilience mechanisms.
//!
//! The architectural flow is:
//!
//! ```text
//! telemetry / history / execution observations
//!                  │
//!                  ▼
//!        learning::features
//!                  │
//!                  ▼
//!          learning::model
//!                  │
//!                  ▼
//!        learning::predictor
//!                  │
//!                  ▼
//!        learning::strategy
//!                  │
//!                  ▼
//!       resilience::planning
//!                  │
//!                  ▼
//!       resilience::recovery
//! ```
//!
//! ## Architectural ownership
//!
//! This module owns only the learning-subsystem module boundary and its public
//! re-export surface.
//!
//! The child modules own their respective contracts:
//!
//! - [`features`] owns feature schemas, feature values, feature vectors,
//!   extraction contracts, normalization, missing-value handling and feature
//!   provenance.
//! - [`model`] owns prediction-model identity, metadata, capabilities,
//!   prediction contracts, compatibility and model lifecycle.
//! - [`predictor`] owns prediction orchestration, model preflight,
//!   compatibility enforcement and prediction validation.
//! - [`strategy`] owns advisory learned strategy selection and ranking.
//!
//! The following responsibilities remain outside this module:
//!
//! - quantum program semantics;
//! - canonical quantum IR;
//! - hardware discovery;
//! - hardware execution;
//! - routing;
//! - scheduling;
//! - optimization;
//! - QEC;
//! - noise modelling;
//! - fault classification;
//! - resilience policy;
//! - recovery execution;
//! - semantic result acceptance;
//! - durable history persistence;
//! - model training infrastructure.
//!
//! Those responsibilities belong to their authoritative subsystems.
//!
//! ## Important integration boundary
//!
//! Learning is deliberately subordinate to the resilience safety architecture.
//!
//! A learned prediction or learned strategy ranking MUST NOT by itself:
//!
//! - authorize execution;
//! - authorize recovery;
//! - authorize migration;
//! - authorize backend selection;
//! - modify canonical quantum semantics;
//! - change QEC configuration;
//! - bypass capability validation;
//! - bypass security policy;
//! - bypass resource limits;
//! - bypass verification;
//! - accept an execution result.
//!
//! The authoritative decision flow remains:
//!
//! ```text
//! learned evidence
//!       │
//!       ▼
//! planning / policy / feasibility
//!       │
//!       ▼
//! adaptation / recovery
//!       │
//!       ▼
//! verification
//!       │
//!       ▼
//! accepted result
//! ```
//!
//! ## Current repository integration
//!
//! At the current repository revision, the learning directory contains:
//!
//! ```text
//! learning/
//! ├── features.rs
//! ├── model.rs
//! ├── predictor.rs
//! └── strategy.rs
//! ```
//!
//! This module therefore declares exactly those four modules.
//!
//! A `feedback.rs` module is intentionally NOT declared here yet because that
//! file is not currently present in the repository. Other learning contracts
//! already document a future `learning/feedback.rs` boundary. Declaring a
//! nonexistent child module would make the crate fail to compile.
//!
//! When `feedback.rs` is implemented, adding it should require only:
//!
//! ```rust
//! pub mod feedback;
//! ```
//!
//! plus any deliberately chosen public re-exports. No existing child module
//! should need architectural redesign merely because feedback is introduced.
//!
//! ## Dependency direction
//!
//! The dependency direction is intentionally one-way:
//!
//! ```text
//! features ───────► model
//!                     ▲
//!                     │
//! predictor ──────────┘
//!
//! predictor ───────► strategy
//!
//! strategy ────────► planning      (through its public contract)
//! ```
//!
//! The module boundary itself does not introduce dependencies on concrete
//! hardware providers, QEC implementations, routing algorithms, schedulers,
//! optimizers, or quantum backends.
//!
//! This prevents the learning subsystem from becoming a second quantum runtime.
//!
//! ## Canonical quantum identity
//!
//! The learning module does not define a competing quantum-resource identity.
//!
//! Whenever a child learning module needs logical or physical quantum-resource
//! identity, the canonical repository types remain authoritative:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! crate::quantum::ir::qubit::QubitRef
//! ```
//!
//! In particular, this module MUST NOT introduce:
//!
//! ```text
//! LearningQubitId
//! ResilienceQubitId
//! PredictionQubitId
//! ```
//!
//! or any other parallel identity system.
//!
//! ## Write once, scale everywhere
//!
//! No architectural resource limit is defined by this module.
//!
//! In particular, this module contains no fixed limits for:
//!
//! - qubits;
//! - logical qubits;
//! - physical qubits;
//! - devices;
//! - backends;
//! - features;
//! - models;
//! - predictions;
//! - strategies;
//! - executions;
//! - training samples;
//! - retry attempts.
//!
//! Runtime/resource bounds must come from explicit caller policy, model
//! requirements, execution budgets, available memory, backend capabilities, or
//! other authoritative resource constraints.
//!
//! The learning subsystem therefore remains applicable from a minimal quantum
//! system to arbitrarily larger systems subject to available computational and
//! storage resources.
//!
//! ## Determinism
//!
//! Determinism is an explicit contract rather than an accidental property.
//!
//! The learning subsystem MUST NOT introduce:
//!
//! - hidden global randomness;
//! - hidden wall-clock dependencies;
//! - hidden environment-variable dependencies;
//! - hidden filesystem dependencies;
//! - hidden network dependencies;
//! - global mutable state.
//!
//! Deterministic feature extraction and prediction must be driven by explicitly
//! supplied inputs and declared model capabilities.
//!
//! Strategy ranking must use deterministic ordering and deterministic
//! tie-breaking when deterministic operation is requested.
//!
//! If stochastic model behavior is used, the relevant randomness and
//! reproducibility information must be represented by the model/context
//! contract and provenance rather than being hidden from resilience.
//!
//! ## Learning safety
//!
//! Learning is an advisory mechanism.
//!
//! Verified outcomes are the authoritative source for durable learning
//! feedback. This is consistent with the resilience architecture's requirement
//! that unverified outcomes must not silently become trusted training evidence.
//!
//! Until a dedicated `feedback.rs` contract exists, this module does not invent
//! a persistence or training API.
//!
//! ## Serialization
//!
//! Child modules own serialization of their portable contracts.
//!
//! This module does not introduce a second serialization format or schema.
//!
//! Stable schema/version identifiers belong to the child contract that owns the
//! corresponding data model.
//!
//! ## Error ownership
//!
//! Child modules retain their own contract-level error types:
//!
//! - [`features::FeatureError`];
//! - [`model::ModelError`];
//! - [`predictor::PredictorError`];
//! - strategy-specific errors exposed by [`strategy`].
//!
//! This module does not create a second umbrella error type merely to combine
//! unrelated contracts. Doing so would unnecessarily couple independently
//! usable learning components.
//!
//! ## Public API policy
//!
//! The child modules are public because the resilience planner/runtime may need
//! to integrate them independently.
//!
//! Selected core types are re-exported below for ergonomic use through:
//!
//! ```text
//! quantum::resilience::learning::TypeName
//! ```
//!
//! Re-exports are limited to stable contract types. Implementation details
//! remain owned by their child modules.
//!
//! ## Rust compatibility
//!
//! - Rust 2021 edition.
//! - Rust 1.97.
//! - Rust 1.97.1.
//! - Stable Rust only.
//! - No nightly features.
//! - No `unsafe`.
//!
//! The module-level safety attribute makes the no-unsafe requirement explicit.
//!
//! ## Testing
//!
//! Child modules own their focused unit tests.
//!
//! Integration tests for the complete learning pipeline should exercise:
//!
//! ```text
//! features
//!     → model
//!     → predictor
//!     → strategy
//!     → planning
//! ```
//!
//! Tests must include:
//!
//! - empty inputs;
//! - large dynamically sized inputs;
//! - malformed feature values;
//! - NaN;
//! - positive infinity;
//! - negative infinity;
//! - schema mismatch;
//! - model incompatibility;
//! - unavailable models;
//! - deterministic prediction;
//! - nondeterministic-model rejection when deterministic operation is required;
//! - deterministic strategy ordering;
//! - equal-score tie breaking;
//! - rejected/unverified evidence;
//! - verified evidence;
//! - model lifecycle transitions;
//! - serialization round trips where supported;
//! - resource exhaustion through explicit caller limits;
//! - no fixed quantum-machine-size assumptions.
//!
//! ## Integration with future feedback
//!
//! The intended future contract is:
//!
//! ```text
//! verification
//!      │
//!      ▼
//! learning::feedback
//!      │
//!      ├── verified outcome
//!      ├── prediction provenance
//!      ├── feature provenance
//!      ├── strategy identity
//!      └── execution/decision identity
//!             │
//!             ▼
//!          history
//!             │
//!             ▼
//!        future learning
//! ```
//!
//! `feedback.rs` must remain subordinate to verification and must never turn
//! unverified execution outcomes into trusted learning data.
//!
//! ## Module extension rule
//!
//! New learning functionality should normally be introduced as a new child
//! module rather than by expanding this file into an implementation module.
//!
//! For example:
//!
//! ```text
//! learning/
//! ├── features.rs
//! ├── model.rs
//! ├── predictor.rs
//! ├── strategy.rs
//! ├── feedback.rs       # future
//! └── training/         # future, if required
//! ```
//!
//! Adding a new child must not require changing unrelated child modules.
//!
//! The parent should normally need only a `pub mod` declaration and, if
//! appropriate, an explicit stable re-export.
//!
//! ## No provider coupling
//!
//! Nothing in this module may branch on provider names such as:
//!
//! ```text
//! IBM
//! Google
//! AWS
//! Azure
//! IonQ
//! Rigetti
//! Quantinuum
//! ```
//!
//! Provider-specific behavior belongs behind the hardware/backend abstraction.
//!
//! The learning subsystem reasons over declared capabilities, observations,
//! schemas, resources and outcomes rather than vendor identity.
//!
//! ## No quantum-size assumptions
//!
//! This module must remain valid for:
//!
//! - a single logical qubit;
//! - a small physical QPU;
//! - a large QPU;
//! - fault-tolerant logical systems;
//! - heterogeneous quantum resources;
//! - distributed quantum execution;
//! - future quantum architectures not yet represented by the repository.
//!
//! The number and identity of resources must always come from authoritative
//! runtime/IR/hardware contracts rather than this module.
//!
//! =============================================================================
//! Module declarations
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(rust_2018_idioms)]

/// Canonical feature definitions, schemas, values, vectors, extraction
/// contracts and feature provenance.
///
/// This module is the boundary between raw resilience observations and
/// numerical/model-ready representations.
pub mod features;

/// Prediction-model identity, metadata, capabilities, compatibility, input
/// and output contracts, and prediction provenance.
///
/// This module does not implement a particular machine-learning algorithm.
pub mod model;

/// Prediction orchestration over validated model contracts.
///
/// This module performs preflight checks and invokes models but does not own
/// training, recovery, planning, QEC, routing, scheduling or hardware access.
pub mod predictor;

/// Advisory learned strategy selection and deterministic strategy ranking.
///
/// This module does not execute the selected strategy. Execution remains the
/// responsibility of planning/adaptation/recovery and the corresponding
/// authoritative subsystems.
pub mod strategy;

// =============================================================================
// Stable public re-exports
// =============================================================================
//
// Keep these re-exports deliberately explicit. They form the ergonomic public
// learning API while preserving ownership of each type in its child module.
//
// Do not use wildcard re-exports. Wildcards make API growth harder to audit,
// can introduce accidental name collisions, and make semver/API review less
// predictable.

pub use features::{
    FeatureContext,
    FeatureDefinition,
    FeatureError,
    FeatureExtractor,
    FeatureId,
    FeatureResult,
    FeatureSchema,
    FeatureSchemaId,
    FeatureSchemaVersion,
    FeatureType,
    FeatureValue,
    FeatureVector,
};

pub use model::{
    ModelCapabilities,
    ModelCompatibility,
    ModelError,
    ModelHandle,
    ModelId,
    ModelKind,
    ModelMetadata,
    ModelRequirements,
    ModelResult,
    ModelSchemaId,
    ModelState,
    ModelVersion,
    Prediction,
    PredictionContext,
    PredictionInput,
    PredictionModel,
    PredictionOutput,
    PredictionTarget,
    PredictionTask,
};

pub use predictor::{
    PredictionBatchResult,
    PredictionRequest,
    PredictionResult,
    Predictor,
    PredictorError,
    PredictorResult,
};

pub use strategy::{
    EvidenceTrust,
    Probability,
    ProbabilityError,
    StrategyDescriptor,
    StrategyId,
    StrategyIdError,
    StrategyKind,
    Utility,
    UtilityError,
};

// =============================================================================
// Compile-time architectural contract tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn module_contract_exposes_core_learning_boundaries() {
        let _ = core::any::TypeId::of::<FeatureError>();
        let _ = core::any::TypeId::of::<ModelError>();
        let _ = core::any::TypeId::of::<PredictorError>();
        let _ = core::any::TypeId::of::<StrategyId>();
    }

    #[test]
    fn strategy_id_is_provider_independent() {
        let id = StrategyId::new("adaptive-remap")
            .expect("non-empty strategy identifiers must be accepted");

        assert_eq!(id.as_str(), "adaptive-remap");
    }

    #[test]
    fn strategy_probability_rejects_non_finite_values() {
        assert!(Probability::new(f64::NAN).is_err());
        assert!(Probability::new(f64::INFINITY).is_err());
        assert!(Probability::new(f64::NEG_INFINITY).is_err());
    }

    #[test]
    fn strategy_probability_rejects_out_of_range_values() {
        assert!(Probability::new(-f64::EPSILON).is_err());
        assert!(Probability::new(1.0 + f64::EPSILON).is_err());
    }

    #[test]
    fn strategy_probability_accepts_boundaries() {
        assert_eq!(Probability::new(0.0).expect("0 is valid").value(), 0.0);
        assert_eq!(Probability::new(1.0).expect("1 is valid").value(), 1.0);
    }

    #[test]
    fn strategy_utility_rejects_non_finite_values() {
        assert!(Utility::new(f64::NAN).is_err());
        assert!(Utility::new(f64::INFINITY).is_err());
        assert!(Utility::new(f64::NEG_INFINITY).is_err());
    }

    #[test]
    fn evidence_trust_requires_verification_for_learning() {
        assert!(EvidenceTrust::Verified.is_learning_eligible());
        assert!(!EvidenceTrust::Unverified.is_learning_eligible());
        assert!(!EvidenceTrust::Rejected.is_learning_eligible());
    }

    #[test]
    fn terminal_strategy_is_explicit() {
        assert!(StrategyKind::Abort.is_terminal());
        assert!(!StrategyKind::Retry.is_terminal());
    }

    #[test]
    fn adaptive_strategy_classification_is_semantic() {
        assert!(StrategyKind::Remap.is_adaptive());
        assert!(StrategyKind::Reroute.is_adaptive());
        assert!(StrategyKind::Reschedule.is_adaptive());
        assert!(StrategyKind::Recompile.is_adaptive());
        assert!(!StrategyKind::Retry.is_adaptive());
        assert!(!StrategyKind::Abort.is_adaptive());
    }
}