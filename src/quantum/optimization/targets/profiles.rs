//! Zamani Quantum Optimization — Target Optimization Profiles
//!
//! Target-specific optimization policy for the Zamani quantum compiler.
//!
//! # Architectural boundary
//!
//! This module describes how an optimizer should value and transform quantum
//! operations for a target *class*. It does not own:
//!
//! - physical qubit topology;
//! - logical-to-physical routing;
//! - pulse scheduling;
//! - calibration;
//! - QPU communication;
//! - backend authentication;
//! - circuit execution;
//! - error-correction decoding;
//! - frontend parsing;
//! - the canonical Quantum IR;
//! - individual optimization pass implementations.
//!
//! The canonical dependency direction is:
//!
//! ```text
//! quantum::ir
//!      │
//!      ▼
//! optimization
//!      │
//!      ├── targets::profiles
//!      │       │
//!      │       ▼
//!      │   targets::target
//!      │       │
//!      ▼       ▼
//! optimization planner / pipeline
//!      │
//!      ▼
//! optimized logical IR
//!      │
//!      ▼
//! routing / scheduling / hardware
//! ```
//!
//! `profiles.rs` is deliberately independent of `target.rs`. A target
//! implementation may consume these immutable policies without requiring this
//! file to know how the target is represented internally.
//!
//! # Relationship with `optimization::config`
//!
//! `optimization::config` owns the compiler-facing `OptimizationProfile` and
//! `TargetSelection` vocabulary.
//!
//! This file owns the *target policy represented by a target profile*.
//!
//! In other words:
//!
//! ```text
//! OptimizationProfile
//!        │
//!        ▼
//! planner
//!        │
//!        ▼
//! TargetProfile
//!        │
//!        ├── native gate set
//!        ├── operation costs
//!        ├── optimization priorities
//!        ├── capabilities
//!        └── transformation policy
//! ```
//!
//! This separation is intentional. It prevents the public compiler
//! configuration from becoming coupled to hardware-specific implementation
//! details.
//!
//! # Design goals
//!
//! The implementation is designed for:
//!
//! - Rust 1.97 / 1.97.1;
//! - Rust 2021;
//! - safe Rust only;
//! - no `unsafe`;
//! - deterministic behavior;
//! - no global mutable state;
//! - no backend I/O;
//! - no hidden random state;
//! - arbitrary user-defined target profiles;
//! - very small circuits;
//! - very large circuits bounded only by available resources and the caller's
//!   explicit resource policy;
//! - future quantum modalities without redesigning the optimizer API;
//! - stable serialization;
//! - reproducible compiler behavior.
//!
//! # Important scalability rule
//!
//! This module intentionally does **not** impose a circuit-size limit.
//!
//! A target profile describes policy. Resource limits belong to
//! `optimization::limits` / `optimization::config` and ultimately to the
//! optimization invocation.
//!
//! Therefore a profile remains valid for:
//!
//! ```text
//! 1 qubit
//! 10 qubits
//! 1,000 qubits
//! 1,000,000 qubits
//! ...and larger,
//! ```
//!
//! subject only to the actual representation, memory, execution-time and
//! explicit compiler resource limits available to the caller.
//!
//! # Hardware boundary
//!
//! A target profile may say that a two-qubit operation is expensive or that a
//! particular native gate is preferred.
//!
//! It must NOT say that qubit 3 is physically connected to qubit 7.
//!
//! Physical topology belongs to `quantum::hardware` / `quantum::routing`.
//!
//! This distinction allows the same optimization profile to be reused across
//! different processor instances and calibrations.
//!
//! # External architectural references
//!
//! Modern quantum compiler stacks commonly separate target gate sets from
//! physical execution concerns. Device specifications may describe supported
//! gates, gate arity, duration and valid targets, while compilation transforms
//! circuits into a target gate set. Zamani follows that separation here.
//!
//! Target-aware decomposition and gate-set compilation are also important for
//! simulators and compiler toolchains that expose target-specific compilation
//! passes.
//!
//! # Integration contract
//!
//! `targets/mod.rs` should declare:
//!
//! ```text
//! pub mod constraints;
//! pub mod gate_set;
//! pub mod profiles;
//! pub mod target;
//! ```
//!
//! `targets/target.rs` should consume:
//!
//! ```text
//! TargetProfile
//! TargetProfileId
//! TargetFamily
//! TargetModality
//! GateCost
//! TargetOptimizationPolicy
//! TargetCapabilities
//! ```
//!
//! `optimization/planner.rs` should resolve an
//! `optimization::config::TargetSelection` to a target definition and then
//! consume the corresponding `TargetProfile`.
//!
//! `optimization/cost.rs` should consume `GateCost` and
//! `TargetOptimizationPolicy`.
//!
//! `optimization/synthesis/*` should consume the native gate-set information.
//!
//! `optimization/passes/*` should consult the target policy but must not access
//! hardware APIs.
//!
//! `routing` should consume the target's connectivity-sensitive metadata while
//! continuing to own actual topology and mapping.
//!
//! `scheduling` should consume duration/timing metadata only after routing and
//! target selection. Scheduling remains the owner of execution ordering.
//!
//! `benchmarking` may consume profile identifiers and optimization metadata for
//! reporting, but this module must not depend on benchmarking.
//!
//! No later target module should require this file to be edited merely because
//! a new target implementation is added. Custom targets should be represented
//! through `TargetProfile::custom`.
//!
//! # Safety
//!
//! This module contains no unsafe code and performs no external I/O.
//!
//! ```text
//! safe Rust only
//! ```
//!
//! # Rust compatibility
//!
//! The implementation intentionally avoids unstable language/library
//! facilities and targets Rust 1.97 / 1.97.1.

// =============================================================================
// Imports
// =============================================================================

use std::fmt;

use serde::{Deserialize, Serialize};

// =============================================================================
// Stable schema version
// =============================================================================

/// Version of the serialized target-profile policy contract.
///
/// Increment this when the meaning of an existing serialized field changes.
/// Adding a new optional capability or adding a new built-in profile does not
/// require changing this version when backwards-compatible semantics are
/// preserved.
pub const TARGET_PROFILE_SCHEMA_VERSION: u32 = 1;

// =============================================================================
// Target profile identifiers
// =============================================================================

/// Stable identifier for a target profile.
///
/// The identifier is intentionally represented as a string-backed value rather
/// than a Rust enum so external users can define targets without modifying this
/// source file.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TargetProfileId(String);

impl TargetProfileId {
    /// Creates a validated target profile identifier.
    pub fn new(value: impl Into<String>) -> Result<Self, TargetProfileError> {
        let value = value.into();

        validate_identifier("target_profile_id", &value)?;

        Ok(Self(value))
    }

    /// Creates an identifier from a known static string.
    ///
    /// Built-in profiles use this helper because their identifiers are compile-
    /// time constants and are already controlled by this module.
    pub fn from_static(value: &'static str) -> Self {
        Self(value.to_owned())
    }

    /// Returns the stable identifier.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the identifier and returns its owned string.
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for TargetProfileId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Target family
// =============================================================================

/// Broad architectural family represented by a target profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TargetFamily {
    /// Generic gate-model quantum computing.
    GateModel,

    /// Fault-tolerant logical quantum computing.
    FaultTolerant,

    /// Analog quantum computation.
    Analog,

    /// Continuous-variable quantum computation.
    ContinuousVariable,

    /// Measurement-based quantum computation.
    MeasurementBased,

    /// Quantum annealing / Ising/QUBO-oriented execution.
    Annealing,

    /// General-purpose quantum simulation.
    Simulation,

    /// A custom or future quantum computational model.
    Custom,
}

impl TargetFamily {
    /// Stable textual identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GateModel => "gate_model",
            Self::FaultTolerant => "fault_tolerant",
            Self::Analog => "analog",
            Self::ContinuousVariable => "continuous_variable",
            Self::MeasurementBased => "measurement_based",
            Self::Annealing => "annealing",
            Self::Simulation => "simulation",
            Self::Custom => "custom",
        }
    }

    /// Returns true when the family fundamentally represents a gate-model
    /// circuit target.
    pub const fn is_gate_model(self) -> bool {
        matches!(
            self,
            Self::GateModel | Self::FaultTolerant | Self::Simulation
        )
    }

    /// Returns true when a conventional logical gate circuit can be used as
    /// the primary optimization representation.
    pub const fn supports_logical_gate_circuits(self) -> bool {
        matches!(
            self,
            Self::GateModel
                | Self::FaultTolerant
                | Self::Simulation
                | Self::MeasurementBased
        )
    }
}

// =============================================================================
// Target modality
// =============================================================================

/// More precise computational modality.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TargetModality {
    /// Generic abstract gate model.
    GenericGateModel,

    /// Superconducting gate-model processors.
    Superconducting,

    /// Trapped-ion processors.
    TrappedIon,

    /// Neutral-atom processors.
    NeutralAtom,

    /// Semiconductor/spin-qubit processors.
    SpinQubit,

    /// Photonic discrete-variable processors.
    PhotonicDiscreteVariable,

    /// Photonic continuous-variable processors.
    PhotonicContinuousVariable,

    /// Bosonic modes / bosonic logical computing.
    Bosonic,

    /// Topological or topological-inspired logical targets.
    Topological,

    /// Generic fault-tolerant logical target.
    LogicalQubit,

    /// Generic stabilizer/Clifford simulation.
    StabilizerSimulator,

    /// State-vector simulation.
    StateVectorSimulator,

    /// Tensor-network simulation.
    TensorNetworkSimulator,

    /// Density-matrix simulation.
    DensityMatrixSimulator,

    /// Measurement-based quantum computation.
    MeasurementBased,

    /// Quantum annealing / Ising target.
    Annealing,

    /// Analog Hamiltonian simulation.
    AnalogHamiltonian,

    /// Continuous-variable target.
    ContinuousVariable,

    /// User-defined future modality.
    Custom,
}

impl TargetModality {
    /// Stable textual identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GenericGateModel => "generic_gate_model",
            Self::Superconducting => "superconducting",
            Self::TrappedIon => "trapped_ion",
            Self::NeutralAtom => "neutral_atom",
            Self::SpinQubit => "spin_qubit",
            Self::PhotonicDiscreteVariable => "photonic_discrete_variable",
            Self::PhotonicContinuousVariable => "photonic_continuous_variable",
            Self::Bosonic => "bosonic",
            Self::Topological => "topological",
            Self::LogicalQubit => "logical_qubit",
            Self::StabilizerSimulator => "stabilizer_simulator",
            Self::StateVectorSimulator => "state_vector_simulator",
            Self::TensorNetworkSimulator => "tensor_network_simulator",
            Self::DensityMatrixSimulator => "density_matrix_simulator",
            Self::MeasurementBased => "measurement_based",
            Self::Annealing => "annealing",
            Self::AnalogHamiltonian => "analog_hamiltonian",
            Self::ContinuousVariable => "continuous_variable",
            Self::Custom => "custom",
        }
    }

    /// Returns true if the modality is normally constrained by a native gate
    /// set.
    pub const fn is_gate_set_oriented(self) -> bool {
        matches!(
            self,
            Self::GenericGateModel
                | Self::Superconducting
                | Self::TrappedIon
                | Self::NeutralAtom
                | Self::SpinQubit
                | Self::PhotonicDiscreteVariable
                | Self::LogicalQubit
                | Self::Topological
                | Self::StabilizerSimulator
                | Self::StateVectorSimulator
                | Self::TensorNetworkSimulator
                | Self::DensityMatrixSimulator
        )
    }
}

// =============================================================================
// Optimization strategy
// =============================================================================

/// Primary strategy used by the planner when selecting transformations for a
/// target profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TargetOptimizationStrategy {
    /// Prefer broad, semantics-preserving logical simplification.
    Logical,

    /// Prefer target-native decomposition and native gate reduction.
    NativeGate,

    /// Prefer minimizing expensive entangling operations.
    TwoQubitDominant,

    /// Prefer minimizing execution depth.
    DepthDominant,

    /// Prefer minimizing non-Clifford resources.
    FaultTolerant,

    /// Prefer exact simulator-friendly transformations.
    Simulation,

    /// Prefer transformations suitable for measurement-based execution.
    MeasurementBased,

    /// Prefer Hamiltonian/observable-preserving transformations.
    Hamiltonian,

    /// User-defined target policy.
    Custom,
}

impl TargetOptimizationStrategy {
    /// Stable textual identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Logical => "logical",
            Self::NativeGate => "native_gate",
            Self::TwoQubitDominant => "two_qubit_dominant",
            Self::DepthDominant => "depth_dominant",
            Self::FaultTolerant => "fault_tolerant",
            Self::Simulation => "simulation",
            Self::MeasurementBased => "measurement_based",
            Self::Hamiltonian => "hamiltonian",
            Self::Custom => "custom",
        }
    }
}

// =============================================================================
// Gate cost model
// =============================================================================

/// Relative cost of one operation category for a target profile.
///
/// Costs are dimensionless weights, not promises about physical hardware.
/// Concrete hardware calibration belongs elsewhere.
///
/// A value of `0.0` means the category contributes no cost to the profile's
/// objective. A value greater than `0.0` contributes proportionally to the
/// selected objective.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct GateCost {
    /// Cost of a generic one-qubit operation.
    pub single_qubit: f64,

    /// Cost of a two-qubit operation.
    pub two_qubit: f64,

    /// Cost of an operation acting on three or more qubits.
    pub multi_qubit: f64,

    /// Cost of a measurement.
    pub measurement: f64,

    /// Cost of a reset.
    pub reset: f64,

    /// Cost of a barrier/ordering operation.
    pub barrier: f64,

    /// Cost assigned to a Clifford operation.
    pub clifford: f64,

    /// Cost assigned to a non-Clifford operation.
    pub non_clifford: f64,

    /// Cost assigned to a T gate.
    pub t_gate: f64,

    /// Cost assigned to an ancilla qubit.
    pub ancilla: f64,

    /// Cost assigned to circuit depth.
    pub depth: f64,

    /// Cost assigned to two-qubit depth.
    pub two_qubit_depth: f64,

    /// Cost assigned to estimated execution duration.
    pub duration: f64,

    /// Cost assigned to estimated error.
    pub error: f64,
}

impl GateCost {
    /// Generic balanced logical cost model.
    pub const fn balanced() -> Self {
        Self {
            single_qubit: 1.0,
            two_qubit: 4.0,
            multi_qubit: 8.0,
            measurement: 1.0,
            reset: 1.0,
            barrier: 0.0,
            clifford: 1.0,
            non_clifford: 2.0,
            t_gate: 4.0,
            ancilla: 1.0,
            depth: 1.0,
            two_qubit_depth: 3.0,
            duration: 1.0,
            error: 1.0,
        }
    }

    /// Cost model emphasizing two-qubit operations.
    pub const fn two_qubit_dominant() -> Self {
        Self {
            single_qubit: 1.0,
            two_qubit: 12.0,
            multi_qubit: 24.0,
            measurement: 1.0,
            reset: 1.0,
            barrier: 0.0,
            clifford: 1.0,
            non_clifford: 2.0,
            t_gate: 4.0,
            ancilla: 1.0,
            depth: 2.0,
            two_qubit_depth: 8.0,
            duration: 1.0,
            error: 2.0,
        }
    }

    /// Cost model emphasizing circuit depth.
    pub const fn depth_dominant() -> Self {
        Self {
            single_qubit: 1.0,
            two_qubit: 4.0,
            multi_qubit: 8.0,
            measurement: 1.0,
            reset: 1.0,
            barrier: 0.0,
            clifford: 1.0,
            non_clifford: 2.0,
            t_gate: 4.0,
            ancilla: 1.0,
            depth: 12.0,
            two_qubit_depth: 16.0,
            duration: 4.0,
            error: 2.0,
        }
    }

    /// Cost model emphasizing fault-tolerant resources.
    pub const fn fault_tolerant() -> Self {
        Self {
            single_qubit: 1.0,
            two_qubit: 3.0,
            multi_qubit: 8.0,
            measurement: 1.0,
            reset: 1.0,
            barrier: 0.0,
            clifford: 1.0,
            non_clifford: 8.0,
            t_gate: 20.0,
            ancilla: 4.0,
            depth: 2.0,
            two_qubit_depth: 4.0,
            duration: 1.0,
            error: 8.0,
        }
    }

    /// Returns true if every cost component is finite and non-negative.
    pub fn is_valid(self) -> bool {
        [
            self.single_qubit,
            self.two_qubit,
            self.multi_qubit,
            self.measurement,
            self.reset,
            self.barrier,
            self.clifford,
            self.non_clifford,
            self.t_gate,
            self.ancilla,
            self.depth,
            self.two_qubit_depth,
            self.duration,
            self.error,
        ]
        .iter()
        .all(|value| value.is_finite() && *value >= 0.0)
    }

    /// Validates the cost model.
    pub fn validate(self) -> Result<(), TargetProfileError> {
        if !self.is_valid() {
            return Err(TargetProfileError::InvalidCostModel);
        }

        Ok(())
    }
}

impl Default for GateCost {
    fn default() -> Self {
        Self::balanced()
    }
}

// =============================================================================
// Target capabilities
// =============================================================================

/// Capabilities that influence which optimizer transformations are legal or
/// useful.
///
/// This is capability metadata, not a hardware implementation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TargetCapabilities {
    /// Whether arbitrary parameterized rotations are accepted natively.
    pub arbitrary_rotations: bool,

    /// Whether symbolic parameters can remain in the final circuit.
    pub symbolic_parameters: bool,

    /// Whether mid-circuit measurement is supported.
    pub mid_circuit_measurement: bool,

    /// Whether reset is supported.
    pub reset: bool,

    /// Whether dynamic classical control is supported.
    pub dynamic_classical_control: bool,

    /// Whether multi-qubit operations are supported natively.
    pub multi_qubit_gates: bool,

    /// Whether global phase may be represented explicitly.
    pub explicit_global_phase: bool,

    /// Whether approximate synthesis is supported.
    pub approximate_synthesis: bool,

    /// Whether ancilla introduction is supported by the target abstraction.
    pub ancillas: bool,

    /// Whether arbitrary-angle synthesis is supported.
    pub arbitrary_angle_synthesis: bool,

    /// Whether Clifford+T representation is a preferred logical basis.
    pub clifford_t: bool,

    /// Whether phase-polynomial optimization is applicable.
    pub phase_polynomials: bool,

    /// Whether physical connectivity affects operation validity.
    pub connectivity_sensitive: bool,

    /// Whether gate durations are meaningful for later scheduling.
    pub timing_information: bool,

    /// Whether the target can preserve measurements as first-class operations.
    pub first_class_measurements: bool,
}

impl TargetCapabilities {
    /// Broad generic gate-model capabilities.
    pub const fn generic() -> Self {
        Self {
            arbitrary_rotations: true,
            symbolic_parameters: true,
            mid_circuit_measurement: true,
            reset: true,
            dynamic_classical_control: true,
            multi_qubit_gates: false,
            explicit_global_phase: true,
            approximate_synthesis: true,
            ancillas: true,
            arbitrary_angle_synthesis: true,
            clifford_t: false,
            phase_polynomials: true,
            connectivity_sensitive: false,
            timing_information: false,
            first_class_measurements: true,
        }
    }

    /// Conservative superconducting-style capabilities.
    pub const fn superconducting() -> Self {
        Self {
            arbitrary_rotations: true,
            symbolic_parameters: true,
            mid_circuit_measurement: true,
            reset: true,
            dynamic_classical_control: true,
            multi_qubit_gates: false,
            explicit_global_phase: true,
            approximate_synthesis: true,
            ancillas: true,
            arbitrary_angle_synthesis: true,
            clifford_t: false,
            phase_polynomials: true,
            connectivity_sensitive: true,
            timing_information: true,
            first_class_measurements: true,
        }
    }

    /// Trapped-ion style capabilities.
    pub const fn trapped_ion() -> Self {
        Self {
            arbitrary_rotations: true,
            symbolic_parameters: true,
            mid_circuit_measurement: true,
            reset: true,
            dynamic_classical_control: true,
            multi_qubit_gates: true,
            explicit_global_phase: true,
            approximate_synthesis: true,
            ancillas: true,
            arbitrary_angle_synthesis: true,
            clifford_t: false,
            phase_polynomials: true,
            connectivity_sensitive: false,
            timing_information: true,
            first_class_measurements: true,
        }
    }

    /// Fault-tolerant logical capabilities.
    pub const fn fault_tolerant() -> Self {
        Self {
            arbitrary_rotations: false,
            symbolic_parameters: true,
            mid_circuit_measurement: true,
            reset: true,
            dynamic_classical_control: true,
            multi_qubit_gates: false,
            explicit_global_phase: true,
            approximate_synthesis: true,
            ancillas: true,
            arbitrary_angle_synthesis: false,
            clifford_t: true,
            phase_polynomials: true,
            connectivity_sensitive: true,
            timing_information: true,
            first_class_measurements: true,
        }
    }

    /// Exact simulator capabilities.
    pub const fn simulator() -> Self {
        Self {
            arbitrary_rotations: true,
            symbolic_parameters: true,
            mid_circuit_measurement: true,
            reset: true,
            dynamic_classical_control: true,
            multi_qubit_gates: true,
            explicit_global_phase: true,
            approximate_synthesis: true,
            ancillas: true,
            arbitrary_angle_synthesis: true,
            clifford_t: true,
            phase_polynomials: true,
            connectivity_sensitive: false,
            timing_information: false,
            first_class_measurements: true,
        }
    }
}

// =============================================================================
// Transformation policy
// =============================================================================

/// Target-specific transformation policy.
///
/// The policy is intentionally descriptive. Individual passes remain owned by
/// the optimization pass framework.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TargetOptimizationPolicy {
    /// Primary strategy.
    pub strategy: TargetOptimizationStrategy,

    /// Prefer native operations even when operation count is not minimal.
    pub prefer_native_gates: bool,

    /// Permit decomposition of non-native operations.
    pub allow_decomposition: bool,

    /// Permit synthesis into a target gate set.
    pub allow_synthesis: bool,

    /// Prefer reducing two-qubit operations.
    pub prioritize_two_qubit_reduction: bool,

    /// Prefer reducing depth.
    pub prioritize_depth: bool,

    /// Prefer reducing total gate count.
    pub prioritize_gate_count: bool,

    /// Prefer reducing T count.
    pub prioritize_t_count: bool,

    /// Prefer reducing T depth.
    pub prioritize_t_depth: bool,

    /// Permit phase-polynomial optimization.
    pub allow_phase_polynomial_optimization: bool,

    /// Permit Clifford simplification.
    pub allow_clifford_optimization: bool,

    /// Permit approximate synthesis when explicitly supported.
    pub allow_approximation: bool,

    /// Permit ancilla introduction.
    pub allow_ancillas: bool,

    /// Permit target-aware gate fusion.
    pub allow_gate_fusion: bool,

    /// Permit aggressive equality-saturation/e-graph search.
    pub allow_egraph: bool,

    /// Whether semantic verification should be strongly preferred after
    /// target-aware transformations.
    pub prefer_semantic_verification: bool,
}

impl TargetOptimizationPolicy {
    /// Generic balanced policy.
    pub const fn balanced() -> Self {
        Self {
            strategy: TargetOptimizationStrategy::Logical,
            prefer_native_gates: false,
            allow_decomposition: true,
            allow_synthesis: true,
            prioritize_two_qubit_reduction: true,
            prioritize_depth: true,
            prioritize_gate_count: true,
            prioritize_t_count: false,
            prioritize_t_depth: false,
            allow_phase_polynomial_optimization: true,
            allow_clifford_optimization: true,
            allow_approximation: false,
            allow_ancillas: false,
            allow_gate_fusion: true,
            allow_egraph: false,
            prefer_semantic_verification: true,
        }
    }

    /// Superconducting-oriented policy.
    pub const fn superconducting() -> Self {
        Self {
            strategy: TargetOptimizationStrategy::TwoQubitDominant,
            prefer_native_gates: true,
            allow_decomposition: true,
            allow_synthesis: true,
            prioritize_two_qubit_reduction: true,
            prioritize_depth: true,
            prioritize_gate_count: false,
            prioritize_t_count: false,
            prioritize_t_depth: false,
            allow_phase_polynomial_optimization: true,
            allow_clifford_optimization: true,
            allow_approximation: true,
            allow_ancillas: false,
            allow_gate_fusion: true,
            allow_egraph: false,
            prefer_semantic_verification: true,
        }
    }

    /// Trapped-ion-oriented policy.
    pub const fn trapped_ion() -> Self {
        Self {
            strategy: TargetOptimizationStrategy::NativeGate,
            prefer_native_gates: true,
            allow_decomposition: true,
            allow_synthesis: true,
            prioritize_two_qubit_reduction: true,
            prioritize_depth: true,
            prioritize_gate_count: true,
            prioritize_t_count: false,
            prioritize_t_depth: false,
            allow_phase_polynomial_optimization: true,
            allow_clifford_optimization: true,
            allow_approximation: true,
            allow_ancillas: false,
            allow_gate_fusion: true,
            allow_egraph: false,
            prefer_semantic_verification: true,
        }
    }

    /// Fault-tolerant policy.
    pub const fn fault_tolerant() -> Self {
        Self {
            strategy: TargetOptimizationStrategy::FaultTolerant,
            prefer_native_gates: true,
            allow_decomposition: true,
            allow_synthesis: true,
            prioritize_two_qubit_reduction: true,
            prioritize_depth: true,
            prioritize_gate_count: true,
            prioritize_t_count: true,
            prioritize_t_depth: true,
            allow_phase_polynomial_optimization: true,
            allow_clifford_optimization: true,
            allow_approximation: false,
            allow_ancillas: true,
            allow_gate_fusion: true,
            allow_egraph: true,
            prefer_semantic_verification: true,
        }
    }

    /// Simulator-oriented policy.
    pub const fn simulator() -> Self {
        Self {
            strategy: TargetOptimizationStrategy::Simulation,
            prefer_native_gates: false,
            allow_decomposition: true,
            allow_synthesis: true,
            prioritize_two_qubit_reduction: false,
            prioritize_depth: false,
            prioritize_gate_count: true,
            prioritize_t_count: false,
            prioritize_t_depth: false,
            allow_phase_polynomial_optimization: true,
            allow_clifford_optimization: true,
            allow_approximation: false,
            allow_ancillas: false,
            allow_gate_fusion: true,
            allow_egraph: false,
            prefer_semantic_verification: true,
        }
    }
}

impl Default for TargetOptimizationPolicy {
    fn default() -> Self {
        Self::balanced()
    }
}

// =============================================================================
// Native gate-set description
// =============================================================================

/// Immutable description of a target's preferred native operations.
///
/// Gate names are semantic identifiers. They are not tied to a concrete
/// `quantum::ir::GateKind` so that this file remains independent of the IR's
/// internal gate taxonomy.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NativeGateSet {
    /// Stable gate-set identifier.
    pub id: String,

    /// Human-readable description.
    pub description: String,

    /// Operations that are accepted/preferred as native.
    pub native_gates: Vec<String>,

    /// Operations that are commonly preferred as entangling primitives.
    pub entangling_gates: Vec<String>,

    /// Operations that are commonly preferred as single-qubit primitives.
    pub single_qubit_gates: Vec<String>,

    /// Whether gate direction is semantically significant for target matching.
    pub directional_two_qubit_gates: bool,
}

impl NativeGateSet {
    /// Creates a custom native gate set.
    pub fn custom(
        id: impl Into<String>,
        description: impl Into<String>,
        native_gates: impl IntoIterator<Item = impl Into<String>>,
    ) -> Result<Self, TargetProfileError> {
        let id = id.into();

        validate_identifier("gate_set_id", &id)?;

        let native_gates: Vec<String> =
            native_gates.into_iter().map(Into::into).collect();

        if native_gates.is_empty() {
            return Err(TargetProfileError::EmptyGateSet);
        }

        validate_gate_names(&native_gates)?;

        Ok(Self {
            id,
            description: description.into(),
            native_gates,
            entangling_gates: Vec::new(),
            single_qubit_gates: Vec::new(),
            directional_two_qubit_gates: false,
        })
    }

    /// Adds an entangling gate to the gate-set metadata.
    pub fn with_entangling_gate(mut self, gate: impl Into<String>) -> Self {
        let gate = gate.into();

        if !self.entangling_gates.iter().any(|item| item == &gate) {
            self.entangling_gates.push(gate);
        }

        self
    }

    /// Adds a single-qubit gate to the gate-set metadata.
    pub fn with_single_qubit_gate(mut self, gate: impl Into<String>) -> Self {
        let gate = gate.into();

        if !self.single_qubit_gates.iter().any(|item| item == &gate) {
            self.single_qubit_gates.push(gate);
        }

        self
    }

    /// Marks two-qubit operations as directional.
    pub fn with_directional_two_qubit_gates(
        mut self,
        directional: bool,
    ) -> Self {
        self.directional_two_qubit_gates = directional;
        self
    }

    /// Returns true if a gate is in the native set.
    pub fn contains(&self, gate: &str) -> bool {
        self.native_gates
            .iter()
            .any(|candidate| candidate == gate)
    }

    /// Validates this gate set.
    pub fn validate(&self) -> Result<(), TargetProfileError> {
        validate_identifier("gate_set_id", &self.id)?;

        if self.native_gates.is_empty() {
            return Err(TargetProfileError::EmptyGateSet);
        }

        validate_gate_names(&self.native_gates)?;
        validate_gate_names(&self.entangling_gates)?;
        validate_gate_names(&self.single_qubit_gates)?;

        Ok(())
    }
}

// =============================================================================
// Built-in gate sets
// =============================================================================

/// Creates a generic gate-model gate set.
pub fn generic_gate_set() -> NativeGateSet {
    NativeGateSet::custom(
        "generic",
        "Broad hardware-independent gate-model gate set.",
        [
            "i", "x", "y", "z", "h", "s", "sdg", "t", "tdg", "rx", "ry", "rz",
            "phase", "cx", "cnot", "cz", "swap", "measure", "reset",
        ],
    )
    .expect("built-in generic gate set must be valid")
    .with_entangling_gate("cx")
    .with_entangling_gate("cnot")
    .with_entangling_gate("cz")
    .with_entangling_gate("swap")
    .with_single_qubit_gate("x")
    .with_single_qubit_gate("y")
    .with_single_qubit_gate("z")
    .with_single_qubit_gate("h")
    .with_single_qubit_gate("rx")
    .with_single_qubit_gate("ry")
    .with_single_qubit_gate("rz")
}

/// Creates a superconducting-style gate set.
///
/// This is a policy profile, not a claim that every superconducting processor
/// uses exactly these operations.
pub fn superconducting_gate_set() -> NativeGateSet {
    NativeGateSet::custom(
        "superconducting",
        "Parameterized single-qubit operations with a native entangling primitive.",
        ["rz", "sx", "x", "ecr", "measure", "reset"],
    )
    .expect("built-in superconducting gate set must be valid")
    .with_entangling_gate("ecr")
    .with_single_qubit_gate("rz")
    .with_single_qubit_gate("sx")
    .with_single_qubit_gate("x")
}

/// Creates a trapped-ion-style gate set.
pub fn trapped_ion_gate_set() -> NativeGateSet {
    NativeGateSet::custom(
        "trapped_ion",
        "Single-qubit rotations with a native entangling interaction.",
        ["rx", "ry", "rz", "rxx", "rzz", "measure", "reset"],
    )
    .expect("built-in trapped-ion gate set must be valid")
    .with_entangling_gate("rxx")
    .with_entangling_gate("rzz")
    .with_single_qubit_gate("rx")
    .with_single_qubit_gate("ry")
    .with_single_qubit_gate("rz")
}

/// Creates a neutral-atom-style gate set.
pub fn neutral_atom_gate_set() -> NativeGateSet {
    NativeGateSet::custom(
        "neutral_atom",
        "Parameterized single-qubit operations and native entangling operations.",
        ["rx", "ry", "rz", "cz", "measure", "reset"],
    )
    .expect("built-in neutral-atom gate set must be valid")
    .with_entangling_gate("cz")
    .with_single_qubit_gate("rx")
    .with_single_qubit_gate("ry")
    .with_single_qubit_gate("rz")
}

/// Creates a fault-tolerant Clifford+T gate set.
pub fn clifford_t_gate_set() -> NativeGateSet {
    NativeGateSet::custom(
        "clifford_t",
        "Fault-tolerant logical Clifford+T basis.",
        [
            "i", "x", "y", "z", "h", "s", "sdg", "t", "tdg", "cx", "cz",
            "measure", "reset",
        ],
    )
    .expect("built-in Clifford+T gate set must be valid")
    .with_entangling_gate("cx")
    .with_entangling_gate("cz")
    .with_single_qubit_gate("h")
    .with_single_qubit_gate("s")
    .with_single_qubit_gate("sdg")
    .with_single_qubit_gate("t")
    .with_single_qubit_gate("tdg")
}

/// Creates a simulator-friendly unrestricted gate set.
pub fn simulator_gate_set() -> NativeGateSet {
    NativeGateSet::custom(
        "simulator",
        "Broad simulator gate set with arbitrary rotations.",
        [
            "i", "x", "y", "z", "h", "s", "sdg", "t", "tdg", "rx", "ry", "rz",
            "phase", "u", "cx", "cnot", "cz", "swap", "ccx", "measure", "reset",
        ],
    )
    .expect("built-in simulator gate set must be valid")
    .with_entangling_gate("cx")
    .with_entangling_gate("cnot")
    .with_entangling_gate("cz")
    .with_entangling_gate("swap")
    .with_entangling_gate("ccx")
    .with_single_qubit_gate("rx")
    .with_single_qubit_gate("ry")
    .with_single_qubit_gate("rz")
    .with_single_qubit_gate("u");

// =============================================================================
// Target profile
// =============================================================================

/// Complete target optimization profile.
///
/// This is the principal object consumed by `targets::target`,
/// `optimization::planner`, `optimization::cost`, and synthesis passes.
///
/// It contains policy and capability metadata only. It deliberately contains
/// no circuit, qubit mapping, topology object, backend connection, or runtime
/// state.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TargetProfile {
    /// Target-profile schema version.
    pub schema_version: u32,

    /// Stable profile identifier.
    pub id: TargetProfileId,

    /// Target family.
    pub family: TargetFamily,

    /// Target modality.
    pub modality: TargetModality,

    /// Human-readable name.
    pub name: String,

    /// Human-readable description.
    pub description: String,

    /// Native gate set.
    pub gate_set: NativeGateSet,

    /// Relative operation/resource costs.
    pub costs: GateCost,

    /// Target capabilities.
    pub capabilities: TargetCapabilities,

    /// Target-specific optimization policy.
    pub optimization: TargetOptimizationPolicy,

    /// Whether physical routing may affect the quality of the final circuit.
    ///
    /// This flag informs planning only. It does not contain topology.
    pub routing_sensitive: bool,

    /// Whether timing information can affect the quality of the final circuit.
    ///
    /// This flag informs planning only. Scheduling remains outside this module.
    pub scheduling_sensitive: bool,

    /// Whether the target is intended for exact transformations only.
    pub exact_semantics_required: bool,

    /// Whether the profile is intended to permit approximate transformations.
    pub approximation_allowed: bool,
}

impl TargetProfile {
    /// Creates a fully specified custom target profile.
    pub fn custom(
        id: TargetProfileId,
        family: TargetFamily,
        modality: TargetModality,
        name: impl Into<String>,
        description: impl Into<String>,
        gate_set: NativeGateSet,
        costs: GateCost,
        capabilities: TargetCapabilities,
        optimization: TargetOptimizationPolicy,
    ) -> Result<Self, TargetProfileError> {
        gate_set.validate()?;
        costs.validate()?;

        let approximation_allowed =
            optimization.allow_approximation && capabilities.approximate_synthesis;

        Ok(Self {
            schema_version: TARGET_PROFILE_SCHEMA_VERSION,
            id,
            family,
            modality,
            name: name.into(),
            description: description.into(),
            gate_set,
            costs,
            capabilities,
            optimization,
            routing_sensitive: capabilities.connectivity_sensitive,
            scheduling_sensitive: capabilities.timing_information,
            exact_semantics_required: !approximation_allowed,
            approximation_allowed,
        })
    }

    /// Returns the stable profile identifier.
    pub fn id(&self) -> &TargetProfileId {
        &self.id
    }

    /// Returns the native gate set.
    pub fn gate_set(&self) -> &NativeGateSet {
        &self.gate_set
    }

    /// Returns the target cost model.
    pub const fn costs(&self) -> GateCost {
        self.costs
    }

    /// Returns target capabilities.
    pub const fn capabilities(&self) -> TargetCapabilities {
        self.capabilities
    }

    /// Returns target optimization policy.
    pub const fn optimization_policy(&self) -> TargetOptimizationPolicy {
        self.optimization
    }

    /// Returns whether a gate is natively supported.
    pub fn supports_gate(&self, gate: &str) -> bool {
        self.gate_set.contains(gate)
    }

    /// Returns whether this target requires topology-aware routing.
    pub const fn requires_routing(&self) -> bool {
        self.routing_sensitive
    }

    /// Returns whether target timing should be considered by later scheduling.
    pub const fn requires_scheduling_information(&self) -> bool {
        self.scheduling_sensitive
    }

    /// Returns whether approximate transformations are allowed.
    pub const fn allows_approximation(&self) -> bool {
        self.approximation_allowed
    }

    /// Validates the complete profile.
    pub fn validate(&self) -> Result<(), TargetProfileError> {
        if self.schema_version != TARGET_PROFILE_SCHEMA_VERSION {
            return Err(TargetProfileError::UnsupportedSchemaVersion {
                version: self.schema_version,
            });
        }

        validate_identifier("target_profile_id", self.id.as_str())?;

        if self.name.trim().is_empty() {
            return Err(TargetProfileError::EmptyField {
                field: "name",
            });
        }

        if self.description.trim().is_empty() {
            return Err(TargetProfileError::EmptyField {
                field: "description",
            });
        }

        self.gate_set.validate()?;
        self.costs.validate()?;

        if self.approximation_allowed
            && !self.capabilities.approximate_synthesis
        {
            return Err(TargetProfileError::InconsistentCapabilities {
                message:
                    "approximation is enabled but the target does not support approximate synthesis"
                        .to_owned(),
            });
        }

        if self.family == TargetFamily::FaultTolerant
            && !self.capabilities.clifford_t
        {
            return Err(TargetProfileError::InconsistentCapabilities {
                message:
                    "fault-tolerant target profiles must advertise Clifford+T capability"
                        .to_owned(),
            });
        }

        if self.optimization.allow_phase_polynomial_optimization
            && !self.capabilities.phase_polynomials
        {
            return Err(TargetProfileError::InconsistentCapabilities {
                message:
                    "phase-polynomial optimization is enabled without phase-polynomial capability"
                        .to_owned(),
            });
        }

        Ok(())
    }

    /// Converts this target profile into the named target selector used by the
    /// compiler configuration layer.
    ///
    /// This keeps `config.rs` independent from target implementation details.
    pub fn target_selection(&self) -> crate::quantum::optimization::config::TargetSelection {
        crate::quantum::optimization::config::TargetSelection::Named(
            self.id.as_str().to_owned(),
        )
    }
}

// =============================================================================
// Built-in profiles
// =============================================================================

/// Generic hardware-independent target profile.
pub fn generic() -> TargetProfile {
    TargetProfile::custom(
        TargetProfileId::from_static("generic"),
        TargetFamily::GateModel,
        TargetModality::GenericGateModel,
        "Generic Quantum Target",
        "Hardware-independent logical quantum optimization target.",
        generic_gate_set(),
        GateCost::balanced(),
        TargetCapabilities::generic(),
        TargetOptimizationPolicy::balanced(),
    )
    .expect("built-in generic target profile must be valid")
}

/// Superconducting-oriented target profile.
///
/// This is a target *class*, not a vendor-specific hardware definition.
pub fn superconducting() -> TargetProfile {
    TargetProfile::custom(
        TargetProfileId::from_static("superconducting"),
        TargetFamily::GateModel,
        TargetModality::Superconducting,
        "Superconducting Target",
        "Target policy emphasizing native single-qubit operations, expensive entangling operations, connectivity and timing awareness.",
        superconducting_gate_set(),
        GateCost::two_qubit_dominant(),
        TargetCapabilities::superconducting(),
        TargetOptimizationPolicy::superconducting(),
    )
    .expect("built-in superconducting target profile must be valid")
}

/// Trapped-ion-oriented target profile.
pub fn trapped_ion() -> TargetProfile {
    TargetProfile::custom(
        TargetProfileId::from_static("trapped_ion"),
        TargetFamily::GateModel,
        TargetModality::TrappedIon,
        "Trapped-Ion Target",
        "Target policy for trapped-ion-style native entangling operations and broad connectivity.",
        trapped_ion_gate_set(),
        GateCost {
            single_qubit: 1.0,
            two_qubit: 5.0,
            multi_qubit: 7.0,
            measurement: 1.5,
            reset: 1.5,
            barrier: 0.0,
            clifford: 1.0,
            non_clifford: 2.0,
            t_gate: 4.0,
            ancilla: 1.0,
            depth: 3.0,
            two_qubit_depth: 5.0,
            duration: 3.0,
            error: 2.0,
        },
        TargetCapabilities::trapped_ion(),
        TargetOptimizationPolicy::trapped_ion(),
    )
    .expect("built-in trapped-ion target profile must be valid")
}

/// Neutral-atom-oriented target profile.
pub fn neutral_atom() -> TargetProfile {
    TargetProfile::custom(
        TargetProfileId::from_static("neutral_atom"),
        TargetFamily::GateModel,
        TargetModality::NeutralAtom,
        "Neutral-Atom Target",
        "Target policy for neutral-atom-style gate-model computation.",
        neutral_atom_gate_set(),
        GateCost {
            single_qubit: 1.0,
            two_qubit: 6.0,
            multi_qubit: 8.0,
            measurement: 1.5,
            reset: 1.5,
            barrier: 0.0,
            clifford: 1.0,
            non_clifford: 2.0,
            t_gate: 4.0,
            ancilla: 1.0,
            depth: 4.0,
            two_qubit_depth: 7.0,
            duration: 3.0,
            error: 3.0,
        },
        TargetCapabilities::generic(),
        TargetOptimizationPolicy::superconducting(),
    )
    .expect("built-in neutral-atom target profile must be valid")
}

/// Fault-tolerant logical target profile.
pub fn fault_tolerant() -> TargetProfile {
    TargetProfile::custom(
        TargetProfileId::from_static("fault_tolerant"),
        TargetFamily::FaultTolerant,
        TargetModality::LogicalQubit,
        "Fault-Tolerant Logical Target",
        "Logical-qubit target emphasizing Clifford+T resource optimization.",
        clifford_t_gate_set(),
        GateCost::fault_tolerant(),
        TargetCapabilities::fault_tolerant(),
        TargetOptimizationPolicy::fault_tolerant(),
    )
    .expect("built-in fault-tolerant target profile must be valid")
}

/// Topological logical target profile.
pub fn topological_logical() -> TargetProfile {
    TargetProfile::custom(
        TargetProfileId::from_static("topological_logical"),
        TargetFamily::FaultTolerant,
        TargetModality::Topological,
        "Topological Logical Target",
        "Logical target emphasizing Clifford+T resources, locality and logical depth.",
        clifford_t_gate_set(),
        GateCost {
            single_qubit: 1.0,
            two_qubit: 8.0,
            multi_qubit: 16.0,
            measurement: 2.0,
            reset: 2.0,
            barrier: 0.0,
            clifford: 1.0,
            non_clifford: 10.0,
            t_gate: 30.0,
            ancilla: 8.0,
            depth: 6.0,
            two_qubit_depth: 12.0,
            duration: 3.0,
            error: 10.0,
        },
        TargetCapabilities::fault_tolerant(),
        TargetOptimizationPolicy::fault_tolerant(),
    )
    .expect("built-in topological target profile must be valid")
}

/// Stabilizer simulator target profile.
pub fn stabilizer_simulator() -> TargetProfile {
    TargetProfile::custom(
        TargetProfileId::from_static("stabilizer_simulator"),
        TargetFamily::Simulation,
        TargetModality::StabilizerSimulator,
        "Stabilizer Simulator Target",
        "Simulator target optimized for Clifford/stabilizer circuits.",
        NativeGateSet::custom(
            "stabilizer",
            "Clifford-oriented simulator gate set.",
            ["i", "x", "y", "z", "h", "s", "sdg", "cx", "cnot", "cz", "swap", "measure", "reset"],
        )
        .expect("built-in stabilizer gate set must be valid")
        .with_entangling_gate("cx")
        .with_entangling_gate("cnot")
        .with_entangling_gate("cz")
        .with_entangling_gate("swap")
        .with_single_qubit_gate("x")
        .with_single_qubit_gate("y")
        .with_single_qubit_gate("z")
        .with_single_qubit_gate("h")
        .with_single_qubit_gate("s")
        .with_single_qubit_gate("sdg"),
        GateCost::balanced(),
        TargetCapabilities {
            arbitrary_rotations: false,
            symbolic_parameters: false,
            mid_circuit_measurement: true,
            reset: true,
            dynamic_classical_control: true,
            multi_qubit_gates: false,
            explicit_global_phase: true,
            approximate_synthesis: false,
            ancillas: true,
            arbitrary_angle_synthesis: false,
            clifford_t: false,
            phase_polynomials: true,
            connectivity_sensitive: false,
            timing_information: false,
            first_class_measurements: true,
        },
        TargetOptimizationPolicy {
            strategy: TargetOptimizationStrategy::Simulation,
            prefer_native_gates: true,
            allow_decomposition: false,
            allow_synthesis: false,
            prioritize_two_qubit_reduction: true,
            prioritize_depth: true,
            prioritize_gate_count: true,
            prioritize_t_count: false,
            prioritize_t_depth: false,
            allow_phase_polynomial_optimization: true,
            allow_clifford_optimization: true,
            allow_approximation: false,
            allow_ancillas: false,
            allow_gate_fusion: false,
            allow_egraph: false,
            prefer_semantic_verification: true,
        },
    )
    .expect("built-in stabilizer simulator profile must be valid")
}

/// State-vector simulator target profile.
pub fn state_vector_simulator() -> TargetProfile {
    TargetProfile::custom(
        TargetProfileId::from_static("state_vector_simulator"),
        TargetFamily::Simulation,
        TargetModality::StateVectorSimulator,
        "State-Vector Simulator Target",
        "Simulator target favoring compact exact circuit representations.",
        simulator_gate_set(),
        GateCost {
            single_qubit: 1.0,
            two_qubit: 2.0,
            multi_qubit: 4.0,
            measurement: 1.0,
            reset: 1.0,
            barrier: 0.0,
            clifford: 1.0,
            non_clifford: 1.0,
            t_gate: 1.0,
            ancilla: 4.0,
            depth: 1.0,
            two_qubit_depth: 2.0,
            duration: 0.0,
            error: 0.0,
        },
        TargetCapabilities::simulator(),
        TargetOptimizationPolicy::simulator(),
    )
    .expect("built-in state-vector simulator profile must be valid")
}

/// Tensor-network simulator target profile.
pub fn tensor_network_simulator() -> TargetProfile {
    TargetProfile::custom(
        TargetProfileId::from_static("tensor_network_simulator"),
        TargetFamily::Simulation,
        TargetModality::TensorNetworkSimulator,
        "Tensor-Network Simulator Target",
        "Simulator target emphasizing low entangling complexity and circuit structure.",
        simulator_gate_set(),
        GateCost {
            single_qubit: 1.0,
            two_qubit: 12.0,
            multi_qubit: 24.0,
            measurement: 1.0,
            reset: 1.0,
            barrier: 0.0,
            clifford: 1.0,
            non_clifford: 2.0,
            t_gate: 3.0,
            ancilla: 2.0,
            depth: 6.0,
            two_qubit_depth: 12.0,
            duration: 0.0,
            error: 0.0,
        },
        TargetCapabilities::simulator(),
        TargetOptimizationPolicy {
            strategy: TargetOptimizationStrategy::Simulation,
            prefer_native_gates: false,
            allow_decomposition: true,
            allow_synthesis: true,
            prioritize_two_qubit_reduction: true,
            prioritize_depth: true,
            prioritize_gate_count: false,
            prioritize_t_count: false,
            prioritize_t_depth: false,
            allow_phase_polynomial_optimization: true,
            allow_clifford_optimization: true,
            allow_approximation: false,
            allow_ancillas: false,
            allow_gate_fusion: true,
            allow_egraph: false,
            prefer_semantic_verification: true,
        },
    )
    .expect("built-in tensor-network simulator profile must be valid")
}

/// Continuous-variable target profile.
pub fn continuous_variable() -> TargetProfile {
    TargetProfile::custom(
        TargetProfileId::from_static("continuous_variable"),
        TargetFamily::ContinuousVariable,
        TargetModality::ContinuousVariable,
        "Continuous-Variable Target",
        "Policy descriptor for continuous-variable quantum compilation.",
        NativeGateSet::custom(
            "continuous_variable",
            "Continuous-variable operation identifiers.",
            [
                "displacement",
                "rotation",
                "squeeze",
                "beamsplitter",
                "controlled_phase",
                "measure",
            ],
        )
        .expect("built-in CV gate set must be valid")
        .with_entangling_gate("beamsplitter")
        .with_entangling_gate("controlled_phase")
        .with_single_qubit_gate("displacement")
        .with_single_qubit_gate("rotation")
        .with_single_qubit_gate("squeeze"),
        GateCost::balanced(),
        TargetCapabilities {
            arbitrary_rotations: true,
            symbolic_parameters: true,
            mid_circuit_measurement: true,
            reset: true,
            dynamic_classical_control: true,
            multi_qubit_gates: false,
            explicit_global_phase: true,
            approximate_synthesis: true,
            ancillas: true,
            arbitrary_angle_synthesis: true,
            clifford_t: false,
            phase_polynomials: false,
            connectivity_sensitive: true,
            timing_information: true,
            first_class_measurements: true,
        },
        TargetOptimizationPolicy {
            strategy: TargetOptimizationStrategy::NativeGate,
            prefer_native_gates: true,
            allow_decomposition: true,
            allow_synthesis: true,
            prioritize_two_qubit_reduction: true,
            prioritize_depth: true,
            prioritize_gate_count: true,
            prioritize_t_count: false,
            prioritize_t_depth: false,
            allow_phase_polynomial_optimization: false,
            allow_clifford_optimization: false,
            allow_approximation: true,
            allow_ancillas: true,
            allow_gate_fusion: true,
            allow_egraph: false,
            prefer_semantic_verification: true,
        },
    )
    .expect("built-in continuous-variable profile must be valid")
}

/// Measurement-based quantum-computing target profile.
pub fn measurement_based() -> TargetProfile {
    TargetProfile::custom(
        TargetProfileId::from_static("measurement_based"),
        TargetFamily::MeasurementBased,
        TargetModality::MeasurementBased,
        "Measurement-Based Quantum Target",
        "Target policy for measurement-driven quantum computation.",
        NativeGateSet::custom(
            "measurement_based",
            "Measurement and resource-state oriented operations.",
            ["h", "s", "cz", "measure", "reset"],
        )
        .expect("built-in MBQC gate set must be valid")
        .with_entangling_gate("cz")
        .with_single_qubit_gate("h")
        .with_single_qubit_gate("s"),
        GateCost {
            single_qubit: 1.0,
            two_qubit: 4.0,
            multi_qubit: 8.0,
            measurement: 5.0,
            reset: 2.0,
            barrier: 0.0,
            clifford: 1.0,
            non_clifford: 4.0,
            t_gate: 8.0,
            ancilla: 3.0,
            depth: 4.0,
            two_qubit_depth: 4.0,
            duration: 3.0,
            error: 4.0,
        },
        TargetCapabilities {
            arbitrary_rotations: false,
            symbolic_parameters: false,
            mid_circuit_measurement: true,
            reset: true,
            dynamic_classical_control: true,
            multi_qubit_gates: false,
            explicit_global_phase: true,
            approximate_synthesis: false,
            ancillas: true,
            arbitrary_angle_synthesis: false,
            clifford_t: true,
            phase_polynomials: true,
            connectivity_sensitive: true,
            timing_information: true,
            first_class_measurements: true,
        },
        TargetOptimizationPolicy {
            strategy: TargetOptimizationStrategy::MeasurementBased,
            prefer_native_gates: true,
            allow_decomposition: true,
            allow_synthesis: true,
            prioritize_two_qubit_reduction: true,
            prioritize_depth: true,
            prioritize_gate_count: true,
            prioritize_t_count: true,
            prioritize_t_depth: true,
            allow_phase_polynomial_optimization: true,
            allow_clifford_optimization: true,
            allow_approximation: false,
            allow_ancillas: true,
            allow_gate_fusion: false,
            allow_egraph: false,
            prefer_semantic_verification: true,
        },
    )
    .expect("built-in measurement-based profile must be valid")
}

/// Analog Hamiltonian target profile.
///
/// This profile is deliberately descriptive. It does not attempt to convert
/// an analog Hamiltonian into pulses; that belongs to the appropriate
/// frontend/backend/compiler subsystem.
pub fn analog_hamiltonian() -> TargetProfile {
    TargetProfile::custom(
        TargetProfileId::from_static("analog_hamiltonian"),
        TargetFamily::Analog,
        TargetModality::AnalogHamiltonian,
        "Analog Hamiltonian Target",
        "Policy descriptor for analog/Hamiltonian quantum computation.",
        NativeGateSet::custom(
            "analog_hamiltonian",
            "Hamiltonian evolution and observable-oriented operations.",
            ["hamiltonian_evolution", "measure"],
        )
        .expect("built-in analog gate set must be valid")
        .with_entangling_gate("hamiltonian_evolution"),
        GateCost {
            single_qubit: 1.0,
            two_qubit: 4.0,
            multi_qubit: 8.0,
            measurement: 4.0,
            reset: 2.0,
            barrier: 0.0,
            clifford: 0.0,
            non_clifford: 0.0,
            t_gate: 0.0,
            ancilla: 1.0,
            depth: 4.0,
            two_qubit_depth: 4.0,
            duration: 8.0,
            error: 4.0,
        },
        TargetCapabilities {
            arbitrary_rotations: true,
            symbolic_parameters: true,
            mid_circuit_measurement: false,
            reset: true,
            dynamic_classical_control: false,
            multi_qubit_gates: true,
            explicit_global_phase: true,
            approximate_synthesis: true,
            ancillas: true,
            arbitrary_angle_synthesis: true,
            clifford_t: false,
            phase_polynomials: false,
            connectivity_sensitive: true,
            timing_information: true,
            first_class_measurements: true,
        },
        TargetOptimizationPolicy {
            strategy: TargetOptimizationStrategy::Hamiltonian,
            prefer_native_gates: true,
            allow_decomposition: false,
            allow_synthesis: false,
            prioritize_two_qubit_reduction: true,
            prioritize_depth: true,
            prioritize_gate_count: false,
            prioritize_t_count: false,
            prioritize_t_depth: false,
            allow_phase_polynomial_optimization: false,
            allow_clifford_optimization: false,
            allow_approximation: true,
            allow_ancillas: true,
            allow_gate_fusion: true,
            allow_egraph: false,
            prefer_semantic_verification: true,
        },
    )
    .expect("built-in analog profile must be valid")
}

/// Quantum annealing / Ising-oriented target profile.
pub fn annealing() -> TargetProfile {
    TargetProfile::custom(
        TargetProfileId::from_static("annealing"),
        TargetFamily::Annealing,
        TargetModality::Annealing,
        "Quantum Annealing Target",
        "Policy descriptor for QUBO/Ising-oriented quantum optimization targets.",
        NativeGateSet::custom(
            "annealing",
            "Problem-Hamiltonian and annealing operations.",
            ["qubo", "ising", "anneal", "measure"],
        )
        .expect("built-in annealing gate set must be valid")
        .with_entangling_gate("ising"),
        GateCost {
            single_qubit: 1.0,
            two_qubit: 3.0,
            multi_qubit: 6.0,
            measurement: 4.0,
            reset: 2.0,
            barrier: 0.0,
            clifford: 0.0,
            non_clifford: 0.0,
            t_gate: 0.0,
            ancilla: 1.0,
            depth: 8.0,
            two_qubit_depth: 8.0,
            duration: 10.0,
            error: 5.0,
        },
        TargetCapabilities {
            arbitrary_rotations: false,
            symbolic_parameters: true,
            mid_circuit_measurement: false,
            reset: true,
            dynamic_classical_control: false,
            multi_qubit_gates: true,
            explicit_global_phase: false,
            approximate_synthesis: false,
            ancillas: true,
            arbitrary_angle_synthesis: false,
            clifford_t: false,
            phase_polynomials: false,
            connectivity_sensitive: true,
            timing_information: true,
            first_class_measurements: true,
        },
        TargetOptimizationPolicy {
            strategy: TargetOptimizationStrategy::Hamiltonian,
            prefer_native_gates: true,
            allow_decomposition: false,
            allow_synthesis: false,
            prioritize_two_qubit_reduction: true,
            prioritize_depth: false,
            prioritize_gate_count: true,
            prioritize_t_count: false,
            prioritize_t_depth: false,
            allow_phase_polynomial_optimization: false,
            allow_clifford_optimization: false,
            allow_approximation: false,
            allow_ancillas: true,
            allow_gate_fusion: false,
            allow_egraph: false,
            prefer_semantic_verification: true,
        },
    )
    .expect("built-in annealing profile must be valid")
}

// =============================================================================
// Built-in profile registry
// =============================================================================

/// Stable built-in target profile identifiers.
///
/// This function returns identifiers rather than references to mutable global
/// state. Every call is independent and deterministic.
pub fn built_in_profile_ids() -> Vec<TargetProfileId> {
    vec![
        TargetProfileId::from_static("generic"),
        TargetProfileId::from_static("superconducting"),
        TargetProfileId::from_static("trapped_ion"),
        TargetProfileId::from_static("neutral_atom"),
        TargetProfileId::from_static("fault_tolerant"),
        TargetProfileId::from_static("topological_logical"),
        TargetProfileId::from_static("stabilizer_simulator"),
        TargetProfileId::from_static("state_vector_simulator"),
        TargetProfileId::from_static("tensor_network_simulator"),
        TargetProfileId::from_static("continuous_variable"),
        TargetProfileId::from_static("measurement_based"),
        TargetProfileId::from_static("analog_hamiltonian"),
        TargetProfileId::from_static("annealing"),
    ]
}

/// Returns a built-in target profile by stable identifier.
///
/// No global registry is used. This function is deterministic and thread-safe.
pub fn built_in(id: &str) -> Option<TargetProfile> {
    match id {
        "generic" => Some(generic()),
        "superconducting" => Some(superconducting()),
        "trapped_ion" => Some(trapped_ion()),
        "neutral_atom" => Some(neutral_atom()),
        "fault_tolerant" => Some(fault_tolerant()),
        "topological_logical" => Some(topological_logical()),
        "stabilizer_simulator" => Some(stabilizer_simulator()),
        "state_vector_simulator" => Some(state_vector_simulator()),
        "tensor_network_simulator" => Some(tensor_network_simulator()),
        "continuous_variable" => Some(continuous_variable()),
        "measurement_based" => Some(measurement_based()),
        "analog_hamiltonian" => Some(analog_hamiltonian()),
        "annealing" => Some(annealing()),
        _ => None,
    }
}

// =============================================================================
// Compatibility aliases
// =============================================================================

/// Target profile suitable for general-purpose logical compilation.
pub fn balanced() -> TargetProfile {
    generic()
}

/// Target profile emphasizing two-qubit reduction.
pub fn minimum_two_qubit() -> TargetProfile {
    superconducting()
}

/// Target profile emphasizing circuit depth.
pub fn minimum_depth() -> TargetProfile {
    let mut profile = generic();

    profile.optimization =
        TargetOptimizationPolicy::balanced();

    profile.optimization = TargetOptimizationPolicy {
        strategy: TargetOptimizationStrategy::DepthDominant,
        prefer_native_gates: false,
        allow_decomposition: true,
        allow_synthesis: true,
        prioritize_two_qubit_reduction: true,
        prioritize_depth: true,
        prioritize_gate_count: false,
        prioritize_t_count: false,
        prioritize_t_depth: false,
        allow_phase_polynomial_optimization: true,
        allow_clifford_optimization: true,
        allow_approximation: false,
        allow_ancillas: false,
        allow_gate_fusion: true,
        allow_egraph: false,
        prefer_semantic_verification: true,
    };

    profile.costs = GateCost::depth_dominant();

    profile
}

/// Target profile emphasizing total operation count.
pub fn minimum_gate_count() -> TargetProfile {
    let mut profile = generic();

    profile.optimization = TargetOptimizationPolicy {
        strategy: TargetOptimizationStrategy::Logical,
        prefer_native_gates: false,
        allow_decomposition: true,
        allow_synthesis: true,
        prioritize_two_qubit_reduction: true,
        prioritize_depth: false,
        prioritize_gate_count: true,
        prioritize_t_count: false,
        prioritize_t_depth: false,
        allow_phase_polynomial_optimization: true,
        allow_clifford_optimization: true,
        allow_approximation: false,
        allow_ancillas: false,
        allow_gate_fusion: true,
        allow_egraph: false,
        prefer_semantic_verification: true,
    };

    profile
}

// =============================================================================
// Errors
// =============================================================================

/// Errors produced while constructing or validating target profiles.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TargetProfileError {
    /// A required identifier is empty.
    EmptyIdentifier {
        /// Field containing the invalid identifier.
        field: &'static str,
    },

    /// An identifier contains characters outside the stable identifier
    /// vocabulary.
    InvalidIdentifier {
        /// Field containing the invalid identifier.
        field: &'static str,

        /// Supplied value.
        value: String,
    },

    /// A required textual field is empty.
    EmptyField {
        /// Name of the empty field.
        field: &'static str,
    },

    /// A native gate set contains no gates.
    EmptyGateSet,

    /// A gate name is invalid.
    InvalidGateName {
        /// Supplied gate name.
        gate: String,
    },

    /// A cost model contains an invalid value.
    InvalidCostModel,

    /// A target profile schema is not supported.
    UnsupportedSchemaVersion {
        /// Supplied schema version.
        version: u32,
    },

    /// Capability flags contradict one another.
    InconsistentCapabilities {
        /// Explanation of the inconsistency.
        message: String,
    },
}

impl fmt::Display for TargetProfileError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyIdentifier { field } => {
                write!(
                    formatter,
                    "target profile field `{field}` must not be empty"
                )
            }

            Self::InvalidIdentifier { field, value } => {
                write!(
                    formatter,
                    "target profile field `{field}` contains invalid identifier `{value}`"
                )
            }

            Self::EmptyField { field } => {
                write!(
                    formatter,
                    "target profile field `{field}` must not be empty"
                )
            }

            Self::EmptyGateSet => {
                formatter.write_str("target profile gate set must not be empty")
            }

            Self::InvalidGateName { gate } => {
                write!(
                    formatter,
                    "target profile contains invalid gate name `{gate}`"
                )
            }

            Self::InvalidCostModel => {
                formatter.write_str(
                    "target profile contains an invalid cost model",
                )
            }

            Self::UnsupportedSchemaVersion { version } => {
                write!(
                    formatter,
                    "unsupported target profile schema version {version}"
                )
            }

            Self::InconsistentCapabilities { message } => {
                write!(
                    formatter,
                    "inconsistent target profile capabilities: {message}"
                )
            }
        }
    }
}

impl std::error::Error for TargetProfileError {}

// =============================================================================
// Validation helpers
// =============================================================================

fn validate_identifier(
    field: &'static str,
    value: &str,
) -> Result<(), TargetProfileError> {
    if value.is_empty() {
        return Err(TargetProfileError::EmptyIdentifier { field });
    }

    let valid = value.chars().enumerate().all(|(index, character)| {
        if index == 0 {
            character.is_ascii_alphanumeric() || character == '_'
        } else {
            character.is_ascii_alphanumeric()
                || matches!(character, '_' | '-' | '.')
        }
    });

    if !valid {
        return Err(TargetProfileError::InvalidIdentifier {
            field,
            value: value.to_owned(),
        });
    }

    Ok(())
}

fn validate_gate_names(
    gates: &[String],
) -> Result<(), TargetProfileError> {
    for gate in gates {
        if gate.trim().is_empty() {
            return Err(TargetProfileError::InvalidGateName {
                gate: gate.clone(),
            });
        }

        if gate
            .chars()
            .any(|character| character.is_control())
        {
            return Err(TargetProfileError::InvalidGateName {
                gate: gate.clone(),
            });
        }
    }

    Ok(())
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schema_version_is_stable() {
        assert_eq!(TARGET_PROFILE_SCHEMA_VERSION, 1);
    }

    #[test]
    fn generic_profile_is_valid() {
        let profile = generic();

        profile
            .validate()
            .expect("generic target profile must validate");

        assert_eq!(profile.id().as_str(), "generic");
        assert!(profile.supports_gate("cx"));
        assert!(profile.supports_gate("rz"));
    }

    #[test]
    fn superconducting_profile_is_routing_sensitive() {
        let profile = superconducting();

        assert!(profile.requires_routing());
        assert!(profile.requires_scheduling_information());
        assert!(profile.supports_gate("ecr"));
    }

    #[test]
    fn trapped_ion_profile_is_valid() {
        let profile = trapped_ion();

        profile
            .validate()
            .expect("trapped-ion profile must validate");

        assert!(profile.supports_gate("rxx"));
        assert!(profile.supports_gate("rzz"));
    }

    #[test]
    fn neutral_atom_profile_is_valid() {
        let profile = neutral_atom();

        profile
            .validate()
            .expect("neutral-atom profile must validate");

        assert!(profile.supports_gate("cz"));
    }

    #[test]
    fn fault_tolerant_profile_is_clifford_t() {
        let profile = fault_tolerant();

        profile
            .validate()
            .expect("fault-tolerant profile must validate");

        assert!(profile.capabilities().clifford_t);
        assert!(profile.supports_gate("t"));
        assert!(profile.supports_gate("tdg"));
        assert!(profile.optimization_policy().prioritize_t_count);
        assert!(profile.optimization_policy().prioritize_t_depth);
    }

    #[test]
    fn topological_profile_is_valid() {
        let profile = topological_logical();

        profile
            .validate()
            .expect("topological profile must validate");

        assert_eq!(
            profile.modality,
            TargetModality::Topological
        );
    }

    #[test]
    fn stabilizer_simulator_rejects_arbitrary_rotations() {
        let profile = stabilizer_simulator();

        assert!(!profile.capabilities().arbitrary_rotations);
        assert!(!profile.supports_gate("rx"));
        assert!(profile.supports_gate("h"));
    }

    #[test]
    fn state_vector_profile_supports_arbitrary_rotations() {
        let profile = state_vector_simulator();

        assert!(profile.capabilities().arbitrary_rotations);
        assert!(profile.supports_gate("rx"));
        assert!(profile.supports_gate("rz"));
    }

    #[test]
    fn tensor_network_profile_prioritizes_two_qubit_operations() {
        let profile = tensor_network_simulator();

        assert!(
            profile
                .optimization_policy()
                .prioritize_two_qubit_reduction
        );
    }

    #[test]
    fn continuous_variable_profile_is_valid() {
        let profile = continuous_variable();

        profile
            .validate()
            .expect("continuous-variable profile must validate");

        assert!(profile.supports_gate("beamsplitter"));
    }

    #[test]
    fn measurement_based_profile_preserves_measurements() {
        let profile = measurement_based();

        profile
            .validate()
            .expect("measurement-based profile must validate");

        assert!(profile.capabilities().mid_circuit_measurement);
        assert!(profile.capabilities().dynamic_classical_control);
        assert!(
            profile
                .optimization_policy()
                .prefer_semantic_verification
        );
    }

    #[test]
    fn analog_profile_is_valid() {
        let profile = analog_hamiltonian();

        profile
            .validate()
            .expect("analog profile must validate");

        assert_eq!(
            profile.family,
            TargetFamily::Analog
        );
    }

    #[test]
    fn annealing_profile_is_valid() {
        let profile = annealing();

        profile
            .validate()
            .expect("annealing profile must validate");

        assert_eq!(
            profile.family,
            TargetFamily::Annealing
        );
    }

    #[test]
    fn all_builtin_profiles_validate() {
        for id in built_in_profile_ids() {
            let profile = built_in(id.as_str())
                .expect("built-in profile identifier must resolve");

            profile
                .validate()
                .expect("built-in target profile must validate");
        }
    }

    #[test]
    fn unknown_builtin_profile_returns_none() {
        assert!(built_in("does_not_exist").is_none());
    }

    #[test]
    fn target_selection_is_compatible_with_configuration() {
        let profile = generic();

        let selection = profile.target_selection();

        match selection {
            crate::quantum::optimization::config::TargetSelection::Named(
                name,
            ) => {
                assert_eq!(name, "generic");
            }
            _ => panic!("target profile must resolve to a named selection"),
        }
    }

    #[test]
    fn custom_target_profile_supports_extension_without_source_changes() {
        let gate_set = NativeGateSet::custom(
            "custom_target",
            "User-defined target.",
            ["my_1q", "my_2q", "measure"],
        )
        .expect("custom gate set must be valid")
        .with_single_qubit_gate("my_1q")
        .with_entangling_gate("my_2q");

        let profile = TargetProfile::custom(
            TargetProfileId::new("my_target")
                .expect("custom identifier must be valid"),
            TargetFamily::Custom,
            TargetModality::Custom,
            "My Target",
            "User-defined Zamani target.",
            gate_set,
            GateCost::balanced(),
            TargetCapabilities::generic(),
            TargetOptimizationPolicy::balanced(),
        )
        .expect("custom target profile must be valid");

        profile
            .validate()
            .expect("custom profile must validate");

        assert!(profile.supports_gate("my_1q"));
        assert!(profile.supports_gate("my_2q"));
    }

    #[test]
    fn invalid_identifier_is_rejected() {
        let result = TargetProfileId::new("invalid target");

        assert!(matches!(
            result,
            Err(TargetProfileError::InvalidIdentifier { .. })
        ));
    }

    #[test]
    fn empty_gate_set_is_rejected() {
        let result = NativeGateSet::custom(
            "empty",
            "Invalid gate set.",
            std::iter::empty::<String>(),
        );

        assert!(matches!(
            result,
            Err(TargetProfileError::EmptyGateSet)
        ));
    }

    #[test]
    fn negative_cost_is_rejected() {
        let costs = GateCost {
            single_qubit: -1.0,
            ..GateCost::balanced()
        };

        assert!(!costs.is_valid());
        assert!(costs.validate().is_err());
    }

    #[test]
    fn approximate_profile_requires_capability() {
        let result = TargetProfile::custom(
            TargetProfileId::new("invalid_approx")
                .expect("identifier must be valid"),
            TargetFamily::GateModel,
            TargetModality::GenericGateModel,
            "Invalid Approximation Target",
            "Target with inconsistent approximation settings.",
            generic_gate_set(),
            GateCost::balanced(),
            TargetCapabilities {
                approximate_synthesis: false,
                ..TargetCapabilities::generic()
            },
            TargetOptimizationPolicy {
                allow_approximation: true,
                ..TargetOptimizationPolicy::balanced()
            },
        )
        .expect("construction itself remains possible");

        assert!(result.validate().is_err());
    }

    #[test]
    fn fault_tolerant_profile_requires_clifford_t_capability() {
        let result = TargetProfile::custom(
            TargetProfileId::new("invalid_ft")
                .expect("identifier must be valid"),
            TargetFamily::FaultTolerant,
            TargetModality::LogicalQubit,
            "Invalid Fault-Tolerant Target",
            "Target with inconsistent fault-tolerant capability.",
            clifford_t_gate_set(),
            GateCost::fault_tolerant(),
            TargetCapabilities {
                clifford_t: false,
                ..TargetCapabilities::fault_tolerant()
            },
            TargetOptimizationPolicy::fault_tolerant(),
        )
        .expect("construction should succeed before validation");

        assert!(result.validate().is_err());
    }

    #[test]
    fn phase_polynomial_policy_requires_capability() {
        let result = TargetProfile::custom(
            TargetProfileId::new("invalid_phase")
                .expect("identifier must be valid"),
            TargetFamily::GateModel,
            TargetModality::GenericGateModel,
            "Invalid Phase Target",
            "Target with inconsistent phase-polynomial capability.",
            generic_gate_set(),
            GateCost::balanced(),
            TargetCapabilities {
                phase_polynomials: false,
                ..TargetCapabilities::generic()
            },
            TargetOptimizationPolicy::balanced(),
        )
        .expect("construction should succeed before validation");

        assert!(result.validate().is_err());
    }

    #[test]
    fn cost_models_are_finite() {
        let costs = [
            GateCost::balanced(),
            GateCost::two_qubit_dominant(),
            GateCost::depth_dominant(),
            GateCost::fault_tolerant(),
        ];

        for cost in costs {
            assert!(cost.is_valid());
        }
    }

    #[test]
    fn target_profile_does_not_impose_circuit_size_limits() {
        // The profile API deliberately contains no circuit-size field.
        // Resource limits belong to the optimizer invocation.
        let profile = generic();

        assert!(profile.validate().is_ok());
    }

    #[test]
    fn built_in_profile_ids_are_deterministic() {
        let first = built_in_profile_ids();
        let second = built_in_profile_ids();

        assert_eq!(first, second);
    }

    #[test]
    fn minimum_depth_profile_changes_policy_without_global_state() {
        let generic_profile = generic();
        let depth_profile = minimum_depth();

        assert_ne!(
            generic_profile.optimization.strategy,
            depth_profile.optimization.strategy
        );

        assert!(
            depth_profile
                .optimization
                .prioritize_depth
        );
    }

    #[test]
    fn minimum_gate_count_profile_is_gate_count_oriented() {
        let profile = minimum_gate_count();

        assert!(
            profile
                .optimization
                .prioritize_gate_count
        );
    }

    #[test]
    fn minimum_two_qubit_profile_is_two_qubit_oriented() {
        let profile = minimum_two_qubit();

        assert!(
            profile
                .optimization
                .prioritize_two_qubit_reduction
        );
    }
}