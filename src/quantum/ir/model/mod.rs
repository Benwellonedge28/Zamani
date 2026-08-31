//! Zamani Quantum IR — Universal Quantum Model Layer
//!
//! Path:
//!     src/quantum/ir/model/mod.rs
//!
//! # Purpose
//!
//! This module is the public module boundary for the model layer of the
//! canonical Zamani Quantum Intermediate Representation.
//!
//! The model layer provides semantic representations for different quantum
//! computational paradigms while keeping the canonical IR independent of:
//!
//! - a particular quantum processor;
//! - a particular qubit count;
//! - a particular topology;
//! - a particular vendor;
//! - a particular simulator;
//! - a particular compiler optimization;
//! - a particular routing implementation;
//! - a particular scheduler;
//! - a particular backend;
//! - a particular physical-control stack.
//!
//! The fundamental Zamani rule is:
//!
//! ```text
//! WRITE ONCE
//!     │
//!     ▼
//! CANONICAL ZAMANI SEMANTICS
//!     │
//!     ├── circuit
//!     ├── dynamic circuit
//!     ├── analog
//!     ├── Hamiltonian evolution
//!     ├── annealing
//!     ├── QUBO
//!     ├── fermionic
//!     ├── bosonic
//!     ├── continuous-variable
//!     ├── measurement-based
//!     ├── tensor-network
//!     ├── logical / fault-tolerant
//!     └── distributed quantum computation
//!     │
//!     ▼
//! TARGET CAPABILITIES
//!     │
//!     ▼
//! MAPPING / ROUTING / SCHEDULING
//!     │
//!     ▼
//! TARGET LOWERING
//!     │
//!     ▼
//! EXECUTION
//! ```
//!
//! The semantic model therefore answers:
//!
//! > What quantum computation does the programmer mean?
//!
//! It does not answer:
//!
//! > Which physical machine executes it?
//!
//! # Universal scaling principle
//!
//! No model exposed by this module establishes a fixed architectural quantum
//! size.
//!
//! The model layer must remain capable of describing programs involving:
//!
//! ```text
//! 1
//! 2
//! 10
//! 100
//! 1_000
//! 1_000_000
//! N
//! ```
//!
//! quantum resources, subject only to the representational capacity of the
//! selected data types and the explicit resource/security policy of the
//! compilation or execution environment.
//!
//! There is deliberately no:
//!
//! ```text
//! MAX_QUBITS
//! MAX_MODES
//! MAX_VARIABLES
//! MAX_TERMS
//! MAX_TENSORS
//! MAX_NODES
//! ```
//!
//! architectural constant in this module.
//!
//! A practical compiler may impose resource limits through
//! `quantum::ir::limits`, but those limits are execution/compilation policy,
//! not semantic limits of the Zamani language.
//!
//! # Platform independence
//!
//! Semantic identities and cardinalities must not use `usize` as their
//! externally meaningful representation.
//!
//! `usize` may be used internally by Rust collections because collection
//! indexing is necessarily host-addressable, but such implementation details
//! must never become part of the semantic model contract.
//!
//! # Logical and physical quantum resources
//!
//! Quantum models that operate on qubits must use the canonical types from:
//!
//! ```text
//! quantum::ir::qubit
//! ```
//!
//! In particular:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! are the authoritative identities.
//!
//! This module does not redefine qubit identities.
//!
//! A mathematical object such as a QUBO variable, fermionic mode or tensor
//! index is NOT automatically a `QubitId`. Conversion into quantum resources
//! belongs to the appropriate lowering/mapping layer.
//!
//! # Ownership
//!
//! This module owns:
//!
//! - model-module registration;
//! - model taxonomy;
//! - stable model-kind identification;
//! - model capability classification;
//! - model-layer documentation and invariants;
//! - public module exposure;
//! - compatibility-safe model discovery helpers.
//!
//! The individual model modules own their domain-specific data structures.
//!
//! This module does NOT own:
//!
//! - gate definitions;
//! - generic operations;
//! - qubit identity;
//! - physical topology;
//! - routing;
//! - scheduling;
//! - hardware descriptions;
//! - device calibration;
//! - backend execution;
//! - simulation state;
//! - QEC decoding algorithms;
//! - optimization algorithms;
//! - frontend parsing.
//!
//! # Model taxonomy
//!
//! The model layer deliberately separates computational paradigms.
//!
//! ```text
//! ModelKind
//! │
//! ├── Circuit
//! ├── Analog
//! ├── Hamiltonian
//! ├── Annealing
//! ├── Qubo
//! ├── Fermionic
//! ├── Bosonic
//! ├── ContinuousVariable
//! ├── MeasurementBased
//! ├── TensorNetwork
//! ├── Logical
//! └── Distributed
//! ```
//!
//! These are semantic categories, not hardware categories.
//!
//! For example:
//!
//! - a circuit may ultimately execute on superconducting hardware;
//! - an analog Hamiltonian model may execute on neutral atoms;
//! - a logical model may lower through a fault-tolerant stack;
//! - a QUBO may lower to annealing hardware or another compatible solver;
//! - a tensor-network model may be executed by a simulator rather than a QPU.
//!
//! The model kind therefore does not identify a backend.
//!
//! # Module dependency rule
//!
//! The dependency direction is strictly downstream:
//!
//! ```text
//! quantum::ir::core / primitive IR types
//!                 │
//!                 ▼
//!       quantum::ir::model::*
//!                 │
//!        ┌────────┼────────┐
//!        ▼        ▼        ▼
//! optimization  mapping  scheduling
//!        │        │        │
//!        └────────┼────────┘
//!                 ▼
//!              hardware
//!                 │
//!                 ▼
//!              backend
//! ```
//!
//! The model layer MUST NOT depend on those downstream layers.
//!
//! # Model modules
//!
//! Each model has an intentionally narrow ownership boundary.
//!
//! ## circuit
//!
//! `circuit.rs` owns the semantic gate-oriented circuit container.
//!
//! It represents ordered quantum operations over logical resources.
//!
//! It must use:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! ```
//!
//! for logical qubit identity.
//!
//! It must not perform physical allocation, routing, scheduling, calibration,
//! backend execution or simulation.
//!
//! ## analog
//!
//! `analog.rs` owns the semantic representation of analog quantum programs
//! and continuous quantum evolution.
//!
//! It must remain independent of the eventual physical control technology.
//!
//! ## hamiltonian
//!
//! `hamiltonian.rs` owns mathematical Hamiltonian representations and their
//! symbolic/time-dependent structure.
//!
//! Hamiltonians are semantic objects and must not contain backend-specific
//! control-channel decisions.
//!
//! ## annealing
//!
//! `annealing.rs` owns annealing-program semantics, schedules and problem
//! descriptions at the mathematical/semantic boundary.
//!
//! It must not encode a particular annealing provider.
//!
//! ## qubo
//!
//! `qubo.rs` owns Quadratic Unconstrained Binary Optimization semantics.
//!
//! A QUBO variable is a mathematical binary variable, not a quantum qubit.
//!
//! Embedding QUBO variables onto quantum resources is a downstream concern.
//!
//! ## fermionic
//!
//! `fermionic.rs` owns fermionic-mode and operator semantics.
//!
//! Fermionic modes must remain distinct from physical qubits until an explicit
//! transformation/lowering is performed.
//!
//! ## bosonic
//!
//! `bosonic.rs` owns bosonic-mode semantics.
//!
//! Bosonic modes are not implicitly qubits and must not be represented by
//! `QubitId` merely for convenience.
//!
//! ## continuous_variable
//!
//! `continuous_variable.rs` owns continuous-variable quantum computation
//! semantics, including mode/operator concepts appropriate to CV systems.
//!
//! ## measurement_based
//!
//! `measurement_based.rs` owns measurement-based quantum computation
//! semantics, including measurement patterns, adaptive semantics and related
//! logical information flow.
//!
//! ## tensor_network
//!
//! `tensor_network.rs` owns tensor-network computational semantics.
//!
//! Tensor indices, tensor nodes and contraction structure must remain distinct
//! from physical quantum resources.
//!
//! ## logical
//!
//! `logical.rs` owns logical/fault-tolerant quantum semantics.
//!
//! A logical qubit is not necessarily a single physical qubit.
//!
//! Physical encoding, code selection, syndrome extraction, routing and
//! decoding belong to downstream fault-tolerance/QEC infrastructure.
//!
//! ## distributed
//!
//! `distributed.rs` owns semantic distributed-quantum concepts.
//!
//! It must be possible to represent multiple quantum execution domains,
//! communication resources and non-local quantum operations without assuming
//! a particular network topology or provider.
//!
//! # Cross-model rule
//!
//! The model modules must not be artificially forced into one universal
//! representation when doing so would destroy semantic information.
//!
//! For example:
//!
//! ```text
//! QUBO
//!     !=
//! QuantumCircuit
//!
//! Hamiltonian evolution
//!     !=
//! Vec<Gate>
//!
//! Fermionic program
//!     !=
//! physical-qubit circuit
//!
//! Continuous-variable computation
//!     !=
//! qubit-only circuit
//! ```
//!
//! A lowering may transform one model into another when mathematically and
//! semantically valid, but that transformation belongs to an explicit
//! compiler/lowering pass.
//!
//! # ModelKind
//!
//! `ModelKind` is a stable semantic classification.
//!
//! It is intentionally small and closed at this layer because it describes
//! the top-level model categories implemented by Zamani itself.
//!
//! Vendor-specific or future model extensions must not require changing
//! existing model data structures merely to attach additional metadata.
//! Such extensions belong to the IR extension/dialect system.
//!
//! # Serialization
//!
//! The model modules own serialization of their concrete model data.
//!
//! This module does not introduce a second serialization format.
//!
//! `ModelKind::as_str` provides a stable semantic identifier suitable for:
//!
//! - schema discriminators;
//! - diagnostics;
//! - logs;
//! - metadata;
//! - format selection;
//! - model discovery.
//!
//! The canonical IR serialization layer remains authoritative for complete
//! program persistence.
//!
//! # Hashing
//!
//! This module does not implement cryptographic hashing.
//!
//! Canonical hashing belongs to:
//!
//! ```text
//! quantum::ir::hash
//! ```
//!
//! Model values must remain deterministically traversable so that canonical
//! hashing can operate without relying on hash-map iteration order.
//!
//! # Validation
//!
//! Individual model modules own model-local validation.
//!
//! Whole-program validation belongs to:
//!
//! ```text
//! quantum::ir::validation
//! ```
//!
//! A model may be structurally valid while still being incompatible with a
//! particular target. Target compatibility must therefore be checked by the
//! capability/resource/hardware layers.
//!
//! # Error policy
//!
//! This module does not introduce a second error hierarchy.
//!
//! Model-specific errors belong to their respective modules and should
//! integrate with the canonical IR error vocabulary where the surrounding
//! implementation provides that mechanism.
//!
//! No model module may silently discard unsupported information.
//!
//! Unsupported or unknown constructs must either:
//!
//! - be preserved as explicit extensions;
//! - be represented as an opaque semantic construct;
//! - or produce an explicit diagnostic/error.
//!
//! # Compatibility policy
//!
//! New model modules may be added without changing the meaning of existing
//! models.
//!
//! Existing model module paths are stable:
//!
//! ```text
//! quantum::ir::model::circuit
//! quantum::ir::model::analog
//! quantum::ir::model::hamiltonian
//! quantum::ir::model::annealing
//! quantum::ir::model::qubo
//! quantum::ir::model::fermionic
//! quantum::ir::model::bosonic
//! quantum::ir::model::continuous_variable
//! quantum::ir::model::measurement_based
//! quantum::ir::model::tensor_network
//! quantum::ir::model::logical
//! quantum::ir::model::distributed
//! ```
//!
//! Removing or changing the meaning of an existing module is a breaking IR
//! change and must go through the canonical IR version/migration mechanism.
//!
//! # Adding a future model
//!
//! A future quantum-computation paradigm should normally be introduced as a
//! new sibling module:
//!
//! ```text
//! src/quantum/ir/model/future_model.rs
//! ```
//!
//! and registered here:
//!
//! ```text
//! pub mod future_model;
//! ```
//!
//! The new module should not require modification of unrelated model files.
//!
//! If the new paradigm requires a new cross-cutting semantic primitive, that
//! primitive belongs in the appropriate lower-level IR module rather than
//! being hidden inside another model.
//!
//! # No hardware assumptions
//!
//! This module intentionally contains no imports from:
//!
//! ```text
//! quantum::hardware
//! quantum::backend
//! quantum::simulator
//! quantum::routing
//! quantum::scheduling
//! quantum::optimization
//! quantum::frontend
//! ```
//!
//! The model layer must remain usable independently of whether those
//! subsystems exist.
//!
//! # Rust contract
//!
//! Supported toolchains:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust.
//!
//! Unsafe Rust is forbidden.
//!
//! This file contains no unsafe code and explicitly forbids introducing it.
//!
//! # Thread safety
//!
//! The module itself contains no mutable global state, registries or caches.
//!
//! Model discovery is compile-time/module-based rather than dependent on a
//! mutable runtime registry.
//!
//! This makes the module deterministic and naturally compatible with
//! concurrent compilation.
//!
//! # Resource exhaustion
//!
//! The module performs no unbounded allocation itself.
//!
//! Concrete model constructors and mutators are responsible for maintaining
//! their own invariants. Compilation/service resource limits belong to
//! `quantum::ir::limits` and/or the consuming compiler.
//!
//! # Integration with quantum::ir
//!
//! The parent module must expose this module exactly once:
//!
//! ```rust
//! pub mod model;
//! ```
//!
//! No duplicate `model` module declaration should exist elsewhere.
//!
//! Consumers may then use:
//!
//! ```rust
//! use crate::quantum::ir::model;
//! ```
//!
//! or explicit model paths:
//!
//! ```rust
//! use crate::quantum::ir::model::circuit::QuantumCircuit;
//! ```
//!
//! Qubit-aware code should continue to use the authoritative path:
//!
//! ```rust
//! use crate::quantum::ir::qubit::QubitId;
//! ```
//!
//! and MUST NOT introduce another qubit identity type inside this module.
//!
//! # Integration with frontend
//!
//! Frontends parse source-language syntax into frontend-specific ASTs.
//!
//! They then lower into one or more canonical model representations.
//!
//! Example:
//!
//! ```text
//! Zamani source
//!      │
//!      ▼
//! frontend AST
//!      │
//!      ▼
//! semantic analysis
//!      │
//!      ▼
//! quantum::ir::model::*
//!      │
//!      ▼
//! canonical Quantum IR
//! ```
//!
//! A frontend must not make the model layer depend on its parser AST.
//!
//! # Integration with optimization
//!
//! Optimization consumes model/IR values and produces semantically equivalent
//! model/IR values.
//!
//! The model layer must not call optimization passes.
//!
//! # Integration with routing/mapping
//!
//! Routing consumes models containing logical quantum resources and produces
//! explicit mapping/lowering information.
//!
//! Routing must not modify the semantic meaning of the model merely because a
//! target has limited connectivity.
//!
//! # Integration with scheduling
//!
//! Scheduling consumes semantic operations and timing constraints and creates
//! target-aware schedules downstream.
//!
//! Model definitions must not embed a scheduler.
//!
//! # Integration with hardware
//!
//! Hardware descriptions answer:
//!
//! ```text
//! What resources/capabilities does this target provide?
//! ```
//!
//! Model definitions answer:
//!
//! ```text
//! What computation does the program mean?
//! ```
//!
//! These questions must remain separate.
//!
//! # Integration with simulators
//!
//! Simulators interpret model semantics.
//!
//! Simulator state, tensor representations, numerical precision and execution
//! engines do not belong in this module.
//!
//! # Integration with QEC
//!
//! The logical model may represent logical operations and encoded resources,
//! while QEC infrastructure provides code-specific transformations, syndrome
//! extraction, decoding and physical realization.
//!
//! This module must remain independent of a particular error-correcting code.
//!
//! # Integration with backends
//!
//! Backends consume lowered/target-compatible representations.
//!
//! They must not add backend-specific fields directly to canonical model
//! structures merely for convenience.
//!
//! Backend-specific information belongs in target descriptions, dialects,
//! attributes or explicit extensions.
//!
//! # Model selection
//!
//! `ModelKind` is useful for classification, diagnostics and dispatch, but it
//! must not become a second runtime type system.
//!
//! Concrete model values remain owned by their corresponding modules.
//!
//! The recommended pattern is:
//!
//! ```text
//! ModelKind
//!     │
//!     ├── identifies the semantic family
//!     │
//!     └── selects an appropriate interpretation/lowering path
//! ```
//!
//! It must not imply:
//!
//! ```text
//! ModelKind
//!     │
//!     └── hard-coded backend
//! ```
//!
//! # Extensibility
//!
//! The model layer is intentionally extensible through sibling modules and the
//! broader IR extension/dialect mechanisms.
//!
//! A future model should be able to coexist with existing models without
//! invalidating programs that do not use it.
//!
//! # Production-readiness invariants
//!
//! The following invariants apply to this module:
//!
//! 1. No unsafe code.
//! 2. No hardware dependency.
//! 3. No frontend dependency.
//! 4. No backend dependency.
//! 5. No fixed quantum-machine size.
//! 6. No fixed topology.
//! 7. No fixed vendor.
//! 8. No duplicate `QubitId`.
//! 9. No silent information loss.
//! 10. No runtime-global mutable registry.
//! 11. No second serialization system.
//! 12. No second hashing system.
//! 13. No optimization logic.
//! 14. No routing logic.
//! 15. No scheduling logic.
//! 16. No simulator state.
//! 17. No provider credentials.
//! 18. No network access.
//! 19. No filesystem access.
//! 20. New model families remain isolated from unrelated models.
//!
//! # Module tree
//!
//! ```text
//! quantum::ir::model
//! │
//! ├── circuit
//! ├── analog
//! ├── hamiltonian
//! ├── annealing
//! ├── qubo
//! ├── fermionic
//! ├── bosonic
//! ├── continuous_variable
//! ├── measurement_based
//! ├── tensor_network
//! ├── logical
//! └── distributed
//! ```
//!
//! -----------------------------------------------------------------------------
//! Implementation
//! -----------------------------------------------------------------------------
//
//! Keep this file intentionally small at runtime. Its principal responsibility
//! is module ownership and stable model-family classification.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

// =============================================================================
// Canonical model modules
// =============================================================================

/// Canonical analog quantum computation model.
pub mod analog;

/// Canonical quantum annealing model.
pub mod annealing;

/// Canonical bosonic quantum-computation model.
pub mod bosonic;

/// Canonical gate-oriented quantum circuit model.
pub mod circuit;

/// Canonical continuous-variable quantum-computation model.
pub mod continuous_variable;

/// Canonical distributed quantum-computation model.
pub mod distributed;

/// Canonical fermionic quantum-computation model.
pub mod fermionic;

/// Canonical Hamiltonian representation and evolution model.
pub mod hamiltonian;

/// Canonical logical/fault-tolerant quantum model.
pub mod logical;

/// Canonical measurement-based quantum-computation model.
pub mod measurement_based;

/// Canonical QUBO mathematical optimization model.
pub mod qubo;

/// Canonical tensor-network computation model.
pub mod tensor_network;

// =============================================================================
// Stable model classification
// =============================================================================

/// Stable semantic classification of a Zamani quantum computation model.
///
/// `ModelKind` identifies the *semantic family* of a model. It does not
/// identify a backend, vendor, device, topology, processor generation or
/// physical implementation.
///
/// This enum is intentionally independent of all concrete model structures.
///
/// # Stability
///
/// The string returned by [`ModelKind::as_str`] is the stable semantic name
/// intended for diagnostics, schema discriminators and metadata.
///
/// Adding a new variant is an additive API change. Existing variants must not
/// change meaning.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[non_exhaustive]
pub enum ModelKind {
    /// Ordered gate-oriented quantum computation.
    Circuit,

    /// Continuous/analog quantum computation.
    Analog,

    /// Hamiltonian-based quantum evolution.
    Hamiltonian,

    /// Quantum annealing computation.
    Annealing,

    /// Quadratic Unconstrained Binary Optimization.
    Qubo,

    /// Fermionic quantum computation.
    Fermionic,

    /// Bosonic quantum computation.
    Bosonic,

    /// Continuous-variable quantum computation.
    ContinuousVariable,

    /// Measurement-based quantum computation.
    MeasurementBased,

    /// Tensor-network computation.
    TensorNetwork,

    /// Logical/fault-tolerant quantum computation.
    Logical,

    /// Distributed quantum computation.
    Distributed,
}

impl ModelKind {
    /// Returns the stable semantic identifier for this model family.
    ///
    /// These identifiers are deliberately lowercase and namespace-friendly.
    ///
    /// They are suitable for:
    ///
    /// - diagnostics;
    /// - metadata;
    /// - schema discriminators;
    /// - model discovery;
    /// - logging;
    /// - stable textual interchange.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Circuit => "circuit",
            Self::Analog => "analog",
            Self::Hamiltonian => "hamiltonian",
            Self::Annealing => "annealing",
            Self::Qubo => "qubo",
            Self::Fermionic => "fermionic",
            Self::Bosonic => "bosonic",
            Self::ContinuousVariable => "continuous_variable",
            Self::MeasurementBased => "measurement_based",
            Self::TensorNetwork => "tensor_network",
            Self::Logical => "logical",
            Self::Distributed => "distributed",
        }
    }

    /// Returns all model families built into this version of the Zamani IR.
    ///
    /// The returned slice is deterministic and allocation-free.
    ///
    /// This is a compile-time taxonomy, not a statement that Zamani is
    /// incapable of representing future models.
    #[must_use]
    pub const fn built_in() -> &'static [Self] {
        &[
            Self::Circuit,
            Self::Analog,
            Self::Hamiltonian,
            Self::Annealing,
            Self::Qubo,
            Self::Fermionic,
            Self::Bosonic,
            Self::ContinuousVariable,
            Self::MeasurementBased,
            Self::TensorNetwork,
            Self::Logical,
            Self::Distributed,
        ]
    }

    /// Returns whether this model is primarily gate/circuit-oriented.
    ///
    /// This is a classification helper only. It must not be used as a
    /// capability check for a particular hardware target.
    #[must_use]
    pub const fn is_circuit_oriented(self) -> bool {
        matches!(self, Self::Circuit)
    }

    /// Returns whether this model is primarily continuous/evolution-oriented.
    ///
    /// This does not imply analog hardware support.
    #[must_use]
    pub const fn is_evolution_oriented(self) -> bool {
        matches!(self, Self::Analog | Self::Hamiltonian)
    }

    /// Returns whether this model represents an optimization/problem
    /// formulation rather than a conventional gate circuit.
    #[must_use]
    pub const fn is_problem_oriented(self) -> bool {
        matches!(self, Self::Annealing | Self::Qubo)
    }

    /// Returns whether this model is primarily mode/operator based.
    #[must_use]
    pub const fn is_mode_oriented(self) -> bool {
        matches!(
            self,
            Self::Fermionic
                | Self::Bosonic
                | Self::ContinuousVariable
        )
    }

    /// Returns whether this model explicitly represents a logical or
    /// distributed execution abstraction.
    #[must_use]
    pub const fn is_execution_architecture_oriented(self) -> bool {
        matches!(self, Self::Logical | Self::Distributed)
    }
}

impl core::fmt::Display for ModelKind {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Model-family constants
// =============================================================================

/// Stable namespace for built-in Zamani quantum model identifiers.
///
/// Concrete serialization formats remain owned by the canonical serialization
/// layer; these identifiers are semantic names only.
pub const MODEL_NAMESPACE: &str = "zamani.quantum.ir.model";

/// Stable model-layer API version.
///
/// This is deliberately NOT the canonical IR version.
///
/// `quantum::ir::identity::IrVersion` remains authoritative for IR semantic
/// compatibility. This constant only identifies the public model-layer API
/// contract represented by this module.
pub const MODEL_API_VERSION: u16 = 1;

// =============================================================================
// Model classification helpers
// =============================================================================

/// Returns the stable identifier of a model family.
///
/// This is a convenience wrapper around [`ModelKind::as_str`].
#[must_use]
pub const fn model_name(kind: ModelKind) -> &'static str {
    kind.as_str()
}

/// Returns the namespace-qualified semantic identifier of a model family.
///
/// Example:
///
/// ```text
/// zamani.quantum.ir.model.circuit
/// ```
#[must_use]
pub fn qualified_model_name(kind: ModelKind) -> String {
    let mut name = String::with_capacity(
        MODEL_NAMESPACE.len() + 1 + kind.as_str().len(),
    );

    name.push_str(MODEL_NAMESPACE);
    name.push('.');
    name.push_str(kind.as_str());

    name
}

// =============================================================================
// Compile-time module contract tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn model_taxonomy_is_deterministic() {
        let names: Vec<&'static str> =
            ModelKind::built_in().iter().map(|kind| kind.as_str()).collect();

        assert_eq!(
            names,
            vec![
                "circuit",
                "analog",
                "hamiltonian",
                "annealing",
                "qubo",
                "fermionic",
                "bosonic",
                "continuous_variable",
                "measurement_based",
                "tensor_network",
                "logical",
                "distributed",
            ]
        );
    }

    #[test]
    fn model_names_are_unique() {
        let models = ModelKind::built_in();

        for (index, left) in models.iter().enumerate() {
            for right in models.iter().skip(index + 1) {
                assert_ne!(left.as_str(), right.as_str());
            }
        }
    }

    #[test]
    fn model_names_are_stable() {
        assert_eq!(ModelKind::Circuit.as_str(), "circuit");
        assert_eq!(ModelKind::Analog.as_str(), "analog");
        assert_eq!(ModelKind::Hamiltonian.as_str(), "hamiltonian");
        assert_eq!(ModelKind::Annealing.as_str(), "annealing");
        assert_eq!(ModelKind::Qubo.as_str(), "qubo");
        assert_eq!(ModelKind::Fermionic.as_str(), "fermionic");
        assert_eq!(ModelKind::Bosonic.as_str(), "bosonic");
        assert_eq!(
            ModelKind::ContinuousVariable.as_str(),
            "continuous_variable"
        );
        assert_eq!(
            ModelKind::MeasurementBased.as_str(),
            "measurement_based"
        );
        assert_eq!(ModelKind::TensorNetwork.as_str(), "tensor_network");
        assert_eq!(ModelKind::Logical.as_str(), "logical");
        assert_eq!(ModelKind::Distributed.as_str(), "distributed");
    }

    #[test]
    fn qualified_names_use_one_stable_namespace() {
        assert_eq!(
            qualified_model_name(ModelKind::Circuit),
            "zamani.quantum.ir.model.circuit"
        );

        assert_eq!(
            qualified_model_name(ModelKind::ContinuousVariable),
            "zamani.quantum.ir.model.continuous_variable"
        );
    }

    #[test]
    fn classification_is_semantic_only() {
        assert!(ModelKind::Circuit.is_circuit_oriented());
        assert!(ModelKind::Analog.is_evolution_oriented());
        assert!(ModelKind::Hamiltonian.is_evolution_oriented());
        assert!(ModelKind::Qubo.is_problem_oriented());
        assert!(ModelKind::Fermionic.is_mode_oriented());
        assert!(ModelKind::Bosonic.is_mode_oriented());
        assert!(ModelKind::ContinuousVariable.is_mode_oriented());
        assert!(ModelKind::Logical.is_execution_architecture_oriented());
        assert!(ModelKind::Distributed.is_execution_architecture_oriented());

        assert!(!ModelKind::Circuit.is_problem_oriented());
        assert!(!ModelKind::Qubo.is_circuit_oriented());
    }
}