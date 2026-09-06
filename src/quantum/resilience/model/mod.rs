//! Zamani Quantum Resilience — Canonical Resilience Domain Model
//!
//! Path:
//!     src/quantum/resilience/model/mod.rs
//!
//! Purpose:
//!     Composes the foundational domain-model modules used by
//!     `quantum::resilience`.
//!
//! Architectural role:
//!
//!     This module is the namespace boundary for resilience-domain values.
//!     It owns module composition and public namespace organization.
//!     It does NOT implement resilience algorithms.
//!
//! The model layer describes:
//!
//!     - resources;
//!     - resource capabilities;
//!     - health;
//!     - degradation;
//!     - faults;
//!     - incidents;
//!     - severity;
//!     - evidentiary confidence.
//!
//! It deliberately does not perform:
//!
//!     - fault detection;
//!     - diagnosis;
//!     - planning;
//!     - routing;
//!     - scheduling;
//!     - compilation;
//!     - optimization;
//!     - QEC;
//!     - error mitigation;
//!     - execution;
//!     - recovery;
//!     - hardware discovery;
//!     - persistence;
//!     - telemetry transport;
//!     - authorization.
//!
//! Those responsibilities belong to sibling resilience subsystems and to
//! the corresponding quantum subsystems elsewhere in the repository.
//!
//! # Architectural position
//!
//! ```text
//!                         Zamani Program
//!                               │
//!                               ▼
//!                       quantum::ir
//!                               │
//!              canonical semantic representation
//!                               │
//!        ┌──────────────────────┴──────────────────────┐
//!        │                                             │
//!        ▼                                             ▼
//! quantum::zqn                                quantum::hardware
//! canonical fault/noise                       hardware capabilities,
//! semantics                                    topology and status
//!        │                                             │
//!        └──────────────────────┬──────────────────────┘
//!                               ▼
//!                    quantum::resilience::model
//!                               │
//!        ┌──────────┬───────────┼───────────┬──────────┐
//!        ▼          ▼           ▼           ▼          ▼
//!     detection  diagnosis   policy      planning   verification
//!        │          │           │           │          │
//!        └──────────┴───────────┴───────────┴──────────┘
//!                               │
//!                               ▼
//!                           recovery
//! ```
//!
//! The model layer is therefore intentionally below decision-making.
//!
//! # Canonical identity boundary
//!
//! The resilience model MUST reuse canonical identities from the quantum IR.
//!
//! In particular:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! remain authoritative for logical and physical qubit identity.
//!
//! Generic IR resources use the canonical IR resource identity exposed by:
//!
//! ```text
//! crate::quantum::ir::core::identity::ResourceId
//! ```
//!
//! No resilience module may introduce a competing:
//!
//! ```text
//! ResilienceQubitId
//! LogicalQubitId
//! PhysicalQubitId
//! ResilienceResourceId
//! ```
//!
//! or equivalent identity type.
//!
//! The concrete `resource` model already follows this rule. It imports the
//! canonical `QubitId`, `PhysicalQubitId`, and `ResourceId` instead of
//! defining replacements. This module therefore only exposes that existing
//! implementation through the `resource` namespace.
//!
//! # Write once, scale everywhere
//!
//! This module introduces no machine-size limits.
//!
//! It contains no:
//!
//! ```text
//! MAX_QUBITS
//! MAX_RESOURCES
//! MAX_DEVICES
//! MAX_BACKENDS
//! MAX_INCIDENTS
//! MAX_FAULTS
//! ```
//!
//! and no fixed hardware topology.
//!
//! The model layer must remain valid for:
//!
//! ```text
//! one qubit
//!        │
//!        ▼
//! small QPU
//!        │
//!        ▼
//! large QPU
//!        │
//!        ▼
//! fault-tolerant logical machine
//!        │
//!        ▼
//! heterogeneous quantum system
//!        │
//!        ▼
//! distributed quantum execution fabric
//! ```
//!
//! "Infinity" is interpreted architecturally as:
//!
//! > no artificial finite machine-size ceiling imposed by the resilience
//! > model.
//!
//! Actual execution remains bounded by the resources available to the
//! compiler, runtime, operating system, network and physical quantum system.
//!
//! # Dependency direction
//!
//! The intended dependency direction is:
//!
//! ```text
//! canonical quantum IR
//!        │
//!        ├──────────────► model::resource
//!        │
//!        └──────────────► model-domain semantics
//!                              │
//!                              ▼
//!                       resilience algorithms
//! ```
//!
//! This module must NOT import:
//!
//! ```text
//! quantum::frontend
//! quantum::optimization
//! quantum::routing
//! quantum::scheduling
//! quantum::hardware
//! quantum::qec
//! quantum::simulation
//! backend SDKs
//! provider SDKs
//! network clients
//! filesystem APIs
//! ```
//!
//! Keeping the model layer independent prevents dependency cycles and allows
//! the same model to be used by local execution, simulators, distributed
//! execution and future quantum technologies.
//!
//! # Model separation
//!
//! Each child module owns one semantic dimension.
//!
//! ```text
//! resource.rs
//!     Which resource is being referenced?
//!
//! capability.rs
//!     What can the resource currently do?
//!
//! health.rs
//!     What is the resource's current operational condition?
//!
//! degradation.rs
//!     How has useful resource capability changed?
//!
//! fault.rs
//!     What normalized quantum-resilience fault has been observed?
//!
//! incident.rs
//!     Which faults belong to the same operational incident?
//!
//! severity.rs
//!     What is the consequence/seriousness of an incident or condition?
//!
//! confidence.rs
//!     How strongly is an observation/claim supported by evidence?
//! ```
//!
//! These dimensions MUST NOT be collapsed into one universal state type.
//!
//! For example:
//!
//! ```text
//! health     != severity
//! severity   != confidence
//! confidence != probability
//! fault      != health
//! incident   != fault
//! resource   != capability
//! capability != allocation
//! ```
//!
//! This separation is essential for correct resilience decisions.
//!
//! # Resource and qubit identity
//!
//! `resource.rs` is the integration boundary for canonical quantum-resource
//! identity.
//!
//! It uses:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! crate::quantum::ir::core::identity::ResourceId
//! ```
//!
//! Therefore callers should prefer:
//!
//! ```text
//! crate::quantum::resilience::model::resource
//! ```
//!
//! when a resilience operation needs resource semantics, while obtaining
//! canonical quantum identities from:
//!
//! ```text
//! crate::quantum::ir::qubit
//! ```
//!
//! No conversion through an intermediate resilience-specific qubit ID is
//! required or permitted.
//!
//! # ZQN integration
//!
//! `fault.rs` is the resilience boundary over the repository's canonical
//! quantum fault/noise semantics.
//!
//! The intended direction is:
//!
//! ```text
//! quantum::zqn
//!       │
//!       │ canonical physical/noise/fault semantics
//!       ▼
//! resilience::model::fault
//!       │
//!       ├── detection
//!       ├── diagnosis
//!       ├── planning
//!       ├── recovery
//!       └── verification
//! ```
//!
//! Resilience must not create a competing quantum fault taxonomy merely for
//! convenience.
//!
//! # Hardware integration
//!
//! `capability.rs` represents the effective capability view required by
//! resilience. It consumes the canonical hardware/IR capability contracts;
//! it does not discover hardware itself.
//!
//! The intended direction is:
//!
//! ```text
//! quantum::hardware
//!        │
//!        │ canonical target information
//!        ▼
//! resilience::model::capability
//!        │
//!        ▼
//! planning / adaptation / verification
//! ```
//!
//! Provider-specific device names, vendor branches and physical-machine
//! assumptions must remain outside this model namespace.
//!
//! # Health integration
//!
//! `health.rs` owns the canonical resilience health vocabulary.
//!
//! Health describes condition, not cause or action:
//!
//! ```text
//! Unknown
//! Healthy
//! Degraded
//! Unstable
//! Unavailable
//! Recovering
//! Quarantined
//! Retired
//! ```
//!
//! A health state does not:
//!
//! - authorize recovery;
//! - establish trust;
//! - prove a fault;
//! - determine severity;
//! - prove result correctness.
//!
//! Those responsibilities belong to higher-level layers.
//!
//! # Degradation integration
//!
//! `degradation.rs` represents changing effective resource capability without
//! requiring resilience to hard-code a machine size.
//!
//! Degradation must therefore work with dynamically discovered/configured
//! quantities rather than assumptions such as:
//!
//! ```text
//! 127 qubits
//! 1000 qubits
//! 1_000_000 qubits
//! ```
//!
//! A degradation model must remain meaningful whether one resource or an
//! arbitrarily large finite resource set is being considered.
//!
//! # Incident integration
//!
//! `incident.rs` groups related faults into operational incidents.
//!
//! This is important for large systems because several observations may
//! represent one underlying event.
//!
//! The model layer only represents that grouping. It does not decide how
//! incidents are detected or diagnosed.
//!
//! # Severity integration
//!
//! `severity.rs` is deliberately independent from health and confidence.
//!
//! Severity answers:
//!
//! ```text
//! How consequential is this condition or incident?
//! ```
//!
//! It does not answer:
//!
//! ```text
//! How certain are we?
//! Is the resource healthy?
//! What caused the problem?
//! What action should be taken?
//! ```
//!
//! # Confidence integration
//!
//! `confidence.rs` represents evidentiary strength.
//!
//! It must not be interpreted as:
//!
//! ```text
//! probability of failure
//! fidelity
//! logical error rate
//! severity
//! priority
//! retry count
//! ```
//!
//! Higher layers may combine confidence with other evidence, but the model
//! itself remains epistemically neutral.
//!
//! # Determinism
//!
//! The model namespace must remain deterministic.
//!
//! It must not:
//!
//! - read the system clock;
//! - generate random values;
//! - inspect environment variables;
//! - access process-global mutable state;
//! - perform I/O;
//! - inspect memory addresses;
//! - depend on provider SDK state.
//!
//! Collection ordering that is semantically observable must be explicitly
//! deterministic in the child model implementations.
//!
//! Deterministic behavior is especially important because the resilience
//! planner may later use these model values as inputs to deterministic
//! recovery planning and replay.
//!
//! # Serialization
//!
//! The model namespace defines semantic domain values, not their wire format.
//!
//! Serialization belongs to:
//!
//! ```text
//! crate::quantum::resilience::serialization
//! ```
//!
//! The child models should therefore remain usable without requiring a
//! particular serialization framework.
//!
//! # Security
//!
//! A valid model value is not proof that the underlying observation is
//! trustworthy.
//!
//! For example:
//!
//! ```text
//! HealthState::Healthy
//! ```
//!
//! does not prove that a hardware provider actually reported a healthy device.
//!
//! Authentication, provenance, freshness, source trust and authorization
//! belong to telemetry/security/integration layers.
//!
//! Likewise, a model value must never grant:
//!
//! - hardware access;
//! - credentials;
//! - migration permission;
//! - recovery permission;
//! - verification bypass.
//!
//! # Compatibility
//!
//! The child model modules are public namespaces rather than an opaque
//! implementation detail because downstream resilience components need stable
//! access to their domain contracts.
//!
//! This parent module intentionally does not perform wildcard re-exports.
//!
//! Therefore:
//!
//! ```text
//! quantum::resilience::model::resource::...
//! quantum::resilience::model::capability::...
//! quantum::resilience::model::health::...
//! quantum::resilience::model::degradation::...
//! quantum::resilience::model::fault::...
//! quantum::resilience::model::incident::...
//! quantum::resilience::model::severity::...
//! quantum::resilience::model::confidence::...
//! ```
//!
//! remain stable and unambiguous.
//!
//! This avoids accidental API collisions when future model modules add types
//! with common names such as `State`, `Status`, `Context`, `Result`, `Event`,
//! or `Resource`.
//!
//! # Future extensibility
//!
//! Additional model modules may be added when a genuinely new semantic
//! dimension is required.
//!
//! Examples could include future domain concepts for:
//!
//! ```text
//! provenance
//! trust
//! freshness
//! execution-domain
//! logical-state
//! physical-state
//! resource-condition
//! capability-evidence
//! ```
//!
//! New modules must first establish a distinct ownership boundary. Existing
//! concepts must not be duplicated merely to make imports shorter.
//!
//! # Rust compatibility
//!
//! This module targets:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe` code.
//!
//! The module explicitly forbids unsafe code.
//!
//! # Integration contract
//!
//! Parent modules should depend on this namespace as follows:
//!
//! ```text
//! quantum::resilience::model
//!             │
//!             ├── resource
//!             ├── capability
//!             ├── health
//!             ├── degradation
//!             ├── fault
//!             ├── incident
//!             ├── severity
//!             └── confidence
//! ```
//!
//! Higher-level resilience modules may import exactly the model namespace
//! they require. The model layer must not import those higher-level modules.
//!
//! This establishes the dependency rule:
//!
//! ```text
//! model ───────────────► no resilience decision layer
//!
//! detection ──────────► model
//! diagnosis ───────────► model
//! policy ───────────────► model
//! planning ─────────────► model
//! adaptation ───────────► model
//! recovery ─────────────► model
//! verification ─────────► model
//! telemetry ────────────► model
//! ```
//!
//! and prevents circular dependencies.
//!
//! # Module inventory
//!
//! ```text
//! model/
//! ├── mod.rs
//! ├── capability.rs
//! ├── confidence.rs
//! ├── degradation.rs
//! ├── fault.rs
//! ├── health.rs
//! ├── incident.rs
//! ├── resource.rs
//! └── severity.rs
//! ```
//!
//! Every child module has a single semantic ownership boundary.
//!
//! The parent does not duplicate those definitions.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

// ============================================================================
// Foundational resilience model modules
// ============================================================================

/// Effective capabilities available to resilience after considering the
/// current execution/resource condition.
pub mod capability;

/// Evidentiary confidence associated with observations and resilience claims.
pub mod confidence;

/// Representation of degradation in resources or effective capabilities.
pub mod degradation;

/// Normalized resilience-level fault semantics.
pub mod fault;

/// Operational health state of a resilience resource or execution capability.
pub mod health;

/// Grouping and lifecycle representation of related resilience faults.
pub mod incident;

/// Canonical resource identity and availability semantics for resilience.
pub mod resource;

/// Operational consequence/severity semantics for resilience incidents and
/// conditions.
pub mod severity;

// ============================================================================
// Architectural compile-time checks
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_model_namespaces_are_available() {
        // These references intentionally exercise the complete module
        // composition boundary without depending on concrete child types.
        //
        // The test is deliberately small because the child modules own their
        // own behavioral tests. This test verifies only that the parent model
        // namespace exposes the complete expected architecture.
        let _ = core::any::type_name::<capability::CapabilityError>;
        let _ = core::any::type_name::<confidence::Confidence>;
        let _ = core::any::type_name::<degradation::Degradation>;
        let _ = core::any::type_name::<fault::ResilienceFault>;
        let _ = core::any::type_name::<health::HealthState>;
        let _ = core::any::type_name::<incident::Incident>;
        let _ = core::any::type_name::<resource::ResourceIdentity>;
        let _ = core::any::type_name::<severity::Severity>;
    }
}