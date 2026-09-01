//! Zamani Quantum IR — Semantic Circuit Properties
//!
//! This module provides deterministic, read-only semantic-property analysis
//! for the canonical logical `QuantumCircuit` representation.
//!
//! # Architectural role
//!
//! `properties.rs` answers:
//!
//! > What semantic properties can be established about this circuit without
//! > executing it, optimizing it, routing it, scheduling it, or binding it to
//! > a physical quantum machine?
//!
//! This module deliberately does NOT:
//!
//! - mutate the circuit;
//! - optimize the circuit;
//! - rewrite gates;
//! - route logical qubits;
//! - allocate physical qubits;
//! - schedule execution;
//! - select a backend;
//! - inspect vendor hardware;
//! - inspect calibration data;
//! - execute a simulator;
//! - execute a QPU;
//! - perform QEC decoding;
//! - estimate hardware-specific cost;
//! - define the canonical `QubitId` type;
//! - redefine gate semantics.
//!
//! Those responsibilities belong to other IR/compiler subsystems.
//!
//! # Architectural boundary
//!
//! ```text
//!                         QuantumCircuit
//!                              │
//!                              ▼
//!                    analysis::properties
//!                              │
//!            ┌─────────────────┼──────────────────┐
//!            ▼                 ▼                  ▼
//!       semantic flags    structural facts   property queries
//!            │                 │                  │
//!            └─────────────────┼──────────────────┘
//!                              ▼
//!                 read-only downstream consumers
//!
//!     optimization     diagnostics     reporting
//!     benchmarking     planning        visualization
//!     validation       compilation     research tooling
//! ```
//!
//! # Canonical IR ownership
//!
//! The canonical logical qubit identity remains:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! ```
//!
//! This module therefore imports `QubitId` from `qubit.rs` and never creates
//! another qubit identity type.
//!
//! The canonical gate representation remains `Gate` from `gate.rs`.
//! `GateKind` remains the semantic gate-kind enumeration.
//!
//! # Relationship with `analysis.rs`
//!
//! `analysis.rs` owns aggregate circuit statistics such as:
//!
//! - operation count;
//! - depth;
//! - gate histograms;
//! - qubit usage;
//! - classical-bit usage;
//! - arity statistics.
//!
//! This module owns semantic properties such as:
//!
//! - whether a circuit is unitary;
//! - whether it contains measurements;
//! - whether it contains reset;
//! - whether it contains barriers;
//! - whether it is parameterized;
//! - whether it contains multi-qubit operations;
//! - whether it is measurement-free;
//! - whether it is reset-free;
//! - whether it has classical destinations;
//! - whether all operations belong to the currently known canonical gate set;
//! - whether the circuit has state-changing operations;
//! - whether the circuit is structurally suitable for particular analysis
//!   categories.
//!
//! Keeping these responsibilities separate prevents the statistics API from
//! becoming the semantic-property API.
//!
//! # Universal quantum-computing principle
//!
//! `QuantumCircuit` is only the currently available circuit-oriented canonical
//! representation. The universal Zamani Quantum IR is intentionally broader
//! and is intended to support:
//!
//! - static circuits;
//! - dynamic circuits;
//! - classical control;
//! - pulse programs;
//! - analog/Hamiltonian programs;
//! - annealing/QUBO programs;
//! - logical/fault-tolerant programs;
//! - distributed quantum programs;
//! - future dialects and extensions.
//!
//! Therefore this module does not claim that the properties below completely
//! characterize all future quantum programs.
//!
//! When universal `QuantumProgram` / `Operation` analysis is introduced, it
//! should receive its own property analysis layer rather than forcing this
//! circuit-specific API to become a universal catch-all.
//!
//! # Scalability
//!
//! There is no architectural maximum quantum-machine size in this module.
//!
//! A circuit containing `N` declared logical qubits remains representable for
//! any `N` permitted by the surrounding process, memory, serialization and
//! resource policy.
//!
//! This module does not allocate one property slot per declared qubit.
//!
//! Property analysis is therefore generally:
//!
//! ```text
//! O(number of operations)
//! ```
//!
//! with only bounded scalar state for the aggregate result.
//!
//! It does not perform:
//!
//! ```text
//! O(number of declared qubits)
//! ```
//!
//! allocation merely to answer circuit-level semantic questions.
//!
//! A million-qubit namespace with no operations does not cause a million-entry
//! semantic-property table to be materialized.
//!
//! # Determinism
//!
//! Results are deterministic because:
//!
//! - gates are traversed in canonical circuit order;
//! - no hash-map iteration order is exposed;
//! - no global mutable state exists;
//! - no random sampling is used;
//! - no machine-specific metadata is consulted;
//! - all aggregate booleans are monotonic during one scan.
//!
//! # Security
//!
//! This module is intended to analyze IR that may originate from:
//!
//! - source-code compilation;
//! - deserialization;
//! - generated programs;
//! - remote compilation services;
//! - plugins;
//! - fuzzing;
//! - untrusted input.
//!
//! It therefore:
//!
//! - uses no `unsafe`;
//! - never performs unchecked indexing;
//! - does not trust declared qubit count as an array bound;
//! - does not allocate proportional to the declared qubit namespace;
//! - uses checked arithmetic where counters can grow;
//! - does not execute arbitrary expressions;
//! - does not execute external calls;
//! - does not access hardware.
//!
//! `QuantumIrLimits` remains the owner of per-invocation resource policy.
//! This module does not invent a global maximum circuit size.
//!
//! # Rust compatibility
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no external dependency requirement beyond the existing Zamani IR.
//!
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler-enforced.
//!
//! # Completion contract
//!
//! This file owns:
//!
//! - `CircuitProperties`;
//! - `CircuitProperty`;
//! - `PropertyProfile`;
//! - deterministic semantic classification of the current
//!   `QuantumCircuit` representation;
//! - read-only property queries;
//! - conservative aggregate classification.
//!
//! This file does not own:
//!
//! - `QubitId`;
//! - `Gate`;
//! - `GateKind`;
//! - circuit validation policy;
//! - resource limits;
//! - optimization transformations;
//! - routing;
//! - scheduling;
//! - hardware capabilities;
//! - serialization schemas;
//! - hashing;
//! - execution.
//!
//! Future files can consume this API without requiring this module to know
//! about their implementation details.
//!
//! # Integration contract
//!
//! ```text
//! crate::quantum::ir::circuit::QuantumCircuit
//!                 │
//!                 ▼
//!       properties::analyze
//!                 │
//!                 ▼
//!          CircuitProperties
//!                 │
//!        ┌────────┼────────┐
//!        ▼        ▼        ▼
//!    optimizer  planner  diagnostics
//! ```
//!
//! The optimizer, planner, benchmarker and backend compatibility layers must
//! treat these properties as descriptive facts about the supplied circuit.
//! They must not interpret them as hardware guarantees.
//!
//! # Important semantic rule
//!
//! `is_unitary()` is deliberately conservative.
//!
//! Measurement and reset are non-unitary.
//!
//! Barrier is treated as a semantic scheduling/analysis marker and therefore
//! makes the circuit unsuitable for a strict "pure unitary circuit" claim,
//! even though it does not itself alter the quantum state.
//!
//! This distinction prevents a compiler from accidentally interpreting a
//! barrier-containing program as a mathematically pure unitary program.
//!
//! # Future integration
//!
//! When the universal operation/dialect system is complete, the intended
//! architecture is:
//!
//! ```text
//! analysis/
//! ├── mod.rs
//! ├── analysis.rs          <- aggregate circuit statistics
//! ├── properties.rs        <- this file; circuit semantic properties
//! ├── program.rs           <- universal program properties
//! ├── operation.rs         <- universal operation properties
//! ├── dependencies.rs      <- dependency analysis
//! ├── liveness.rs          <- liveness analysis
//! └── resource_usage.rs    <- resource analysis
//! ```
//!
//! This file should remain stable when those future modules are added.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::fmt;

use crate::quantum::ir::circuit::QuantumCircuit;
use crate::quantum::ir::gate::{Gate, GateKind};
use crate::quantum::ir::qubit::QubitId;

// =============================================================================
// Property identifiers
// =============================================================================

/// Stable semantic property identifiers.
///
/// These identifiers are intentionally descriptive rather than hardware
/// specific. New properties can be added without changing the meaning of
/// existing properties.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CircuitProperty {
    /// The circuit contains at least one operation.
    NonEmpty,

    /// The circuit contains no operations.
    Empty,

    /// Every operation is mathematically unitary and no semantic barrier is
    /// present.
    StrictlyUnitary,

    /// At least one non-unitary operation is present.
    NonUnitary,

    /// At least one measurement is present.
    HasMeasurement,

    /// No measurement operation is present.
    MeasurementFree,

    /// At least one reset is present.
    HasReset,

    /// No reset operation is present.
    ResetFree,

    /// At least one barrier is present.
    HasBarrier,

    /// No barrier is present.
    BarrierFree,

    /// At least one operation has parameters.
    Parameterized,

    /// No operation has parameters.
    ParameterFree,

    /// At least one operation acts on two or more logical qubits.
    MultiQubit,

    /// Every operation acts on at most one logical qubit.
    SingleQubitOnly,

    /// At least one operation targets a classical destination.
    HasClassicalTarget,

    /// No operation targets a classical destination.
    ClassicalTargetFree,

    /// At least one state-changing quantum operation is present.
    HasQuantumStateChange,

    /// No state-changing quantum operation is present.
    NoQuantumStateChange,

    /// The circuit contains only currently recognized canonical gate kinds.
    CanonicalGateSet,

    /// The circuit contains at least one operation that is not part of the
    /// current standard gate dialect.
    NonCanonicalGateSet,

    /// The circuit has exactly one logical qubit in its declared namespace.
    SingleDeclaredQubit,

    /// The circuit declares more than one logical qubit.
    MultipleDeclaredQubits,

    /// No logical qubit is actually referenced by an operation.
    NoReferencedQubits,

    /// At least one logical qubit is referenced by an operation.
    HasReferencedQubits,
}

impl CircuitProperty {
    /// Returns a stable machine-readable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NonEmpty => "non_empty",
            Self::Empty => "empty",
            Self::StrictlyUnitary => "strictly_unitary",
            Self::NonUnitary => "non_unitary",
            Self::HasMeasurement => "has_measurement",
            Self::MeasurementFree => "measurement_free",
            Self::HasReset => "has_reset",
            Self::ResetFree => "reset_free",
            Self::HasBarrier => "has_barrier",
            Self::BarrierFree => "barrier_free",
            Self::Parameterized => "parameterized",
            Self::ParameterFree => "parameter_free",
            Self::MultiQubit => "multi_qubit",
            Self::SingleQubitOnly => "single_qubit_only",
            Self::HasClassicalTarget => "has_classical_target",
            Self::ClassicalTargetFree => "classical_target_free",
            Self::HasQuantumStateChange => "has_quantum_state_change",
            Self::NoQuantumStateChange => "no_quantum_state_change",
            Self::CanonicalGateSet => "canonical_gate_set",
            Self::NonCanonicalGateSet => "non_canonical_gate_set",
            Self::SingleDeclaredQubit => "single_declared_qubit",
            Self::MultipleDeclaredQubits => "multiple_declared_qubits",
            Self::NoReferencedQubits => "no_referenced_qubits",
            Self::HasReferencedQubits => "has_referenced_qubits",
        }
    }
}

impl fmt::Display for CircuitProperty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// =============================================================================
// Property profile
// =============================================================================

/// Compact deterministic semantic profile of a canonical quantum circuit.
///
/// This is intentionally a value type with no references to the circuit.
/// Consequently, callers may retain it after the original circuit borrow ends.
///
/// The profile contains descriptive facts only. It does not contain hardware
/// information and must never be interpreted as a backend capability claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PropertyProfile {
    operation_count: usize,
    declared_qubits: usize,
    referenced_qubits: usize,

    unitary_operations: usize,
    non_unitary_operations: usize,

    measurement_operations: usize,
    reset_operations: usize,
    barrier_operations: usize,

    parameterized_operations: usize,
    multi_qubit_operations: usize,
    classical_target_operations: usize,

    state_changing_operations: usize,
    canonical_operations: usize,
    non_canonical_operations: usize,
}

impl PropertyProfile {
    fn empty(declared_qubits: usize) -> Self {
        Self {
            operation_count: 0,
            declared_qubits,
            referenced_qubits: 0,
            unitary_operations: 0,
            non_unitary_operations: 0,
            measurement_operations: 0,
            reset_operations: 0,
            barrier_operations: 0,
            parameterized_operations: 0,
            multi_qubit_operations: 0,
            classical_target_operations: 0,
            state_changing_operations: 0,
            canonical_operations: 0,
            non_canonical_operations: 0,
        }
    }

    /// Returns the number of operations.
    #[must_use]
    pub const fn operation_count(&self) -> usize {
        self.operation_count
    }

    /// Returns the number of declared logical qubits.
    #[must_use]
    pub const fn declared_qubits(&self) -> usize {
        self.declared_qubits
    }

    /// Returns the number of distinct logical qubits referenced by operations.
    ///
    /// The current circuit API permits this to be calculated without
    /// materializing a vector proportional to the declared namespace.
    #[must_use]
    pub const fn referenced_qubits(&self) -> usize {
        self.referenced_qubits
    }

    /// Returns the number of unitary operations.
    #[must_use]
    pub const fn unitary_operations(&self) -> usize {
        self.unitary_operations
    }

    /// Returns the number of non-unitary operations.
    #[must_use]
    pub const fn non_unitary_operations(&self) -> usize {
        self.non_unitary_operations
    }

    /// Returns the number of measurements.
    #[must_use]
    pub const fn measurement_operations(&self) -> usize {
        self.measurement_operations
    }

    /// Returns the number of resets.
    #[must_use]
    pub const fn reset_operations(&self) -> usize {
        self.reset_operations
    }

    /// Returns the number of barriers.
    #[must_use]
    pub const fn barrier_operations(&self) -> usize {
        self.barrier_operations
    }

    /// Returns the number of parameterized operations.
    #[must_use]
    pub const fn parameterized_operations(&self) -> usize {
        self.parameterized_operations
    }

    /// Returns the number of multi-qubit operations.
    #[must_use]
    pub const fn multi_qubit_operations(&self) -> usize {
        self.multi_qubit_operations
    }

    /// Returns the number of operations with classical destinations.
    #[must_use]
    pub const fn classical_target_operations(&self) -> usize {
        self.classical_target_operations
    }

    /// Returns the number of quantum state-changing operations.
    #[must_use]
    pub const fn state_changing_operations(&self) -> usize {
        self.state_changing_operations
    }

    /// Returns the number of operations belonging to the current canonical
    /// gate dialect.
    #[must_use]
    pub const fn canonical_operations(&self) -> usize {
        self.canonical_operations
    }

    /// Returns the number of operations outside the current canonical gate
    /// dialect.
    #[must_use]
    pub const fn non_canonical_operations(&self) -> usize {
        self.non_canonical_operations
    }

    /// Returns whether the circuit contains no operations.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.operation_count == 0
    }

    /// Returns whether the circuit contains at least one operation.
    #[must_use]
    pub const fn is_non_empty(&self) -> bool {
        self.operation_count != 0
    }

    /// Returns whether the circuit is strictly unitary.
    ///
    /// A barrier prevents a strict-unitary classification because it is a
    /// semantic/scheduling marker rather than a mathematical unitary.
    #[must_use]
    pub const fn is_strictly_unitary(&self) -> bool {
        self.operation_count > 0
            && self.non_unitary_operations == 0
            && self.barrier_operations == 0
    }

    /// Returns whether the circuit contains a non-unitary operation.
    #[must_use]
    pub const fn is_non_unitary(&self) -> bool {
        self.non_unitary_operations != 0
    }

    /// Returns whether the circuit contains measurement.
    #[must_use]
    pub const fn has_measurement(&self) -> bool {
        self.measurement_operations != 0
    }

    /// Returns whether the circuit contains reset.
    #[must_use]
    pub const fn has_reset(&self) -> bool {
        self.reset_operations != 0
    }

    /// Returns whether the circuit contains a barrier.
    #[must_use]
    pub const fn has_barrier(&self) -> bool {
        self.barrier_operations != 0
    }

    /// Returns whether the circuit contains parameterized operations.
    #[must_use]
    pub const fn is_parameterized(&self) -> bool {
        self.parameterized_operations != 0
    }

    /// Returns whether all operations are single-qubit operations.
    #[must_use]
    pub const fn is_single_qubit_only(&self) -> bool {
        self.multi_qubit_operations == 0
    }

    /// Returns whether at least one multi-qubit operation exists.
    #[must_use]
    pub const fn has_multi_qubit_operation(&self) -> bool {
        self.multi_qubit_operations != 0
    }

    /// Returns whether the circuit contains a classical destination.
    #[must_use]
    pub const fn has_classical_target(&self) -> bool {
        self.classical_target_operations != 0
    }

    /// Returns whether any quantum state-changing operation is present.
    #[must_use]
    pub const fn has_quantum_state_change(&self) -> bool {
        self.state_changing_operations != 0
    }

    /// Returns whether all operations belong to the currently known canonical
    /// gate set.
    #[must_use]
    pub const fn is_canonical_gate_set(&self) -> bool {
        self.non_canonical_operations == 0
    }

    /// Returns whether the circuit declares exactly one logical qubit.
    #[must_use]
    pub const fn has_single_declared_qubit(&self) -> bool {
        self.declared_qubits == 1
    }

    /// Returns whether the circuit declares multiple logical qubits.
    #[must_use]
    pub const fn has_multiple_declared_qubits(&self) -> bool {
        self.declared_qubits > 1
    }

    /// Returns whether at least one operation references a logical qubit.
    #[must_use]
    pub const fn has_referenced_qubits(&self) -> bool {
        self.referenced_qubits != 0
    }

    /// Evaluates a semantic property against this profile.
    #[must_use]
    pub const fn has_property(&self, property: CircuitProperty) -> bool {
        match property {
            CircuitProperty::NonEmpty => self.is_non_empty(),
            CircuitProperty::Empty => self.is_empty(),
            CircuitProperty::StrictlyUnitary => self.is_strictly_unitary(),
            CircuitProperty::NonUnitary => self.is_non_unitary(),
            CircuitProperty::HasMeasurement => self.has_measurement(),
            CircuitProperty::MeasurementFree => !self.has_measurement(),
            CircuitProperty::HasReset => self.has_reset(),
            CircuitProperty::ResetFree => !self.has_reset(),
            CircuitProperty::HasBarrier => self.has_barrier(),
            CircuitProperty::BarrierFree => !self.has_barrier(),
            CircuitProperty::Parameterized => self.is_parameterized(),
            CircuitProperty::ParameterFree => !self.is_parameterized(),
            CircuitProperty::MultiQubit => self.has_multi_qubit_operation(),
            CircuitProperty::SingleQubitOnly => self.is_single_qubit_only(),
            CircuitProperty::HasClassicalTarget => self.has_classical_target(),
            CircuitProperty::ClassicalTargetFree => !self.has_classical_target(),
            CircuitProperty::HasQuantumStateChange => self.has_quantum_state_change(),
            CircuitProperty::NoQuantumStateChange => !self.has_quantum_state_change(),
            CircuitProperty::CanonicalGateSet => self.is_canonical_gate_set(),
            CircuitProperty::NonCanonicalGateSet => self.non_canonical_operations != 0,
            CircuitProperty::SingleDeclaredQubit => self.has_single_declared_qubit(),
            CircuitProperty::MultipleDeclaredQubits => {
                self.has_multiple_declared_qubits()
            }
            CircuitProperty::NoReferencedQubits => !self.has_referenced_qubits(),
            CircuitProperty::HasReferencedQubits => self.has_referenced_qubits(),
        }
    }
}

// =============================================================================
// Circuit properties
// =============================================================================

/// Complete read-only semantic property result.
///
/// This is the primary public result returned by [`analyze`] and
/// [`properties`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CircuitProperties {
    profile: PropertyProfile,
}

impl CircuitProperties {
    fn from_profile(profile: PropertyProfile) -> Self {
        Self { profile }
    }

    /// Returns the complete compact property profile.
    #[must_use]
    pub const fn profile(&self) -> PropertyProfile {
        self.profile
    }

    /// Returns whether the circuit has no operations.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.profile.is_empty()
    }

    /// Returns whether the circuit has at least one operation.
    #[must_use]
    pub const fn is_non_empty(&self) -> bool {
        self.profile.is_non_empty()
    }

    /// Returns whether the circuit is strictly unitary.
    ///
    /// This is intentionally stronger than merely checking that every gate
    /// reports `is_unitary()`: barriers are semantic boundaries and therefore
    /// prevent a strict-unitary classification.
    #[must_use]
    pub const fn is_strictly_unitary(&self) -> bool {
        self.profile.is_strictly_unitary()
    }

    /// Returns whether the circuit contains non-unitary operations.
    #[must_use]
    pub const fn is_non_unitary(&self) -> bool {
        self.profile.is_non_unitary()
    }

    /// Returns whether the circuit contains measurements.
    #[must_use]
    pub const fn has_measurement(&self) -> bool {
        self.profile.has_measurement()
    }

    /// Returns whether the circuit contains no measurements.
    #[must_use]
    pub const fn is_measurement_free(&self) -> bool {
        !self.has_measurement()
    }

    /// Returns whether the circuit contains reset.
    #[must_use]
    pub const fn has_reset(&self) -> bool {
        self.profile.has_reset()
    }

    /// Returns whether the circuit contains no reset operations.
    #[must_use]
    pub const fn is_reset_free(&self) -> bool {
        !self.has_reset()
    }

    /// Returns whether the circuit contains barriers.
    #[must_use]
    pub const fn has_barrier(&self) -> bool {
        self.profile.has_barrier()
    }

    /// Returns whether the circuit contains no barriers.
    #[must_use]
    pub const fn is_barrier_free(&self) -> bool {
        !self.has_barrier()
    }

    /// Returns whether at least one operation is parameterized.
    #[must_use]
    pub const fn is_parameterized(&self) -> bool {
        self.profile.is_parameterized()
    }

    /// Returns whether no operation is parameterized.
    #[must_use]
    pub const fn is_parameter_free(&self) -> bool {
        !self.is_parameterized()
    }

    /// Returns whether at least one operation acts on multiple qubits.
    #[must_use]
    pub const fn has_multi_qubit_operation(&self) -> bool {
        self.profile.has_multi_qubit_operation()
    }

    /// Returns whether every operation acts on at most one qubit.
    #[must_use]
    pub const fn is_single_qubit_only(&self) -> bool {
        self.profile.is_single_qubit_only()
    }

    /// Returns whether at least one operation has a classical destination.
    #[must_use]
    pub const fn has_classical_target(&self) -> bool {
        self.profile.has_classical_target()
    }

    /// Returns whether no operation has a classical destination.
    #[must_use]
    pub const fn is_classical_target_free(&self) -> bool {
        !self.has_classical_target()
    }

    /// Returns whether at least one operation changes quantum state.
    #[must_use]
    pub const fn has_quantum_state_change(&self) -> bool {
        self.profile.has_quantum_state_change()
    }

    /// Returns whether the circuit contains no quantum state-changing
    /// operations.
    #[must_use]
    pub const fn has_no_quantum_state_change(&self) -> bool {
        !self.has_quantum_state_change()
    }

    /// Returns whether all operations belong to the current canonical gate
    /// dialect.
    #[must_use]
    pub const fn is_canonical_gate_set(&self) -> bool {
        self.profile.is_canonical_gate_set()
    }

    /// Returns whether the circuit contains an operation outside the current
    /// canonical gate dialect.
    #[must_use]
    pub const fn has_non_canonical_operation(&self) -> bool {
        self.profile.non_canonical_operations() != 0
    }

    /// Returns the number of declared logical qubits.
    #[must_use]
    pub const fn declared_qubits(&self) -> usize {
        self.profile.declared_qubits()
    }

    /// Returns the number of distinct referenced logical qubits.
    #[must_use]
    pub const fn referenced_qubits(&self) -> usize {
        self.profile.referenced_qubits()
    }

    /// Returns whether the circuit declares exactly one logical qubit.
    #[must_use]
    pub const fn is_single_declared_qubit(&self) -> bool {
        self.profile.has_single_declared_qubit()
    }

    /// Returns whether the circuit declares more than one logical qubit.
    #[must_use]
    pub const fn is_multi_declared_qubit(&self) -> bool {
        self.profile.has_multiple_declared_qubits()
    }

    /// Returns whether a property holds.
    #[must_use]
    pub const fn has_property(&self, property: CircuitProperty) -> bool {
        self.profile.has_property(property)
    }
}

// =============================================================================
// Public analysis entry points
// =============================================================================

/// Analyzes semantic properties of a canonical logical quantum circuit.
///
/// This function performs one deterministic read-only scan over the circuit's
/// canonical operations.
///
/// # Complexity
///
/// Let `O` be the number of operations and `Q` the number of distinct logical
/// qubits actually referenced.
///
/// The scan is:
///
/// - time: `O(O * average_operand_count)`;
/// - auxiliary memory: `O(Q)` for the sparse set of referenced qubits.
///
/// No storage proportional to the declared qubit namespace is allocated.
///
/// # Validation
///
/// This function does not replace the canonical IR validator. Callers that
/// require complete structural validation should validate the circuit through
/// `quantum::ir::validation` before consuming semantic properties.
///
/// Property analysis itself remains defensive and does not assume that a
/// circuit is executable.
///
/// # Errors
///
/// No semantic-property error is required for the current circuit
/// representation. The result is therefore infallible for a structurally
/// representable `QuantumCircuit`.
///
/// Structural validation remains the responsibility of the validation layer.
///
/// # Integration
///
/// This is the stable entry point for downstream consumers.
///
/// ```text
/// QuantumCircuit
///       │
///       ▼
/// properties::analyze
///       │
///       ▼
/// CircuitProperties
/// ```
///
/// The circuit is borrowed and never cloned or modified.
#[must_use = "semantic property analysis should not be silently discarded"]
pub fn analyze(circuit: &QuantumCircuit) -> CircuitProperties {
    let declared_qubits = circuit.num_qubits();
    let operations = circuit.operations();

    let mut profile = PropertyProfile::empty(declared_qubits);

    // A BTreeSet gives deterministic logical-qubit identity tracking and does
    // not depend on randomized hash-map iteration.
    //
    // The set grows with actual references, not with the declared namespace.
    let mut referenced_qubits = std::collections::BTreeSet::<QubitId>::new();

    for gate in operations {
        profile.operation_count = profile
            .operation_count
            .saturating_add(1);

        classify_gate(
            gate,
            &mut profile,
            &mut referenced_qubits,
        );
    }

    profile.referenced_qubits = referenced_qubits.len();

    CircuitProperties::from_profile(profile)
}

/// Alias for [`analyze`].
///
/// This name is useful when the caller already operates in a property-oriented
/// context:
///
/// ```text
/// let properties = properties(&circuit);
/// ```
///
/// It intentionally does not conflict with `analysis::analyze` because callers
/// should import one of the functions explicitly when both modules are in
/// scope.
#[must_use = "semantic property analysis should not be silently discarded"]
pub fn properties(circuit: &QuantumCircuit) -> CircuitProperties {
    analyze(circuit)
}

/// Analyzes a single canonical gate without requiring a complete circuit.
///
/// This is useful for optimization dispatch, diagnostics and future
/// operation-level analysis while keeping the canonical `Gate` type owned by
/// `gate.rs`.
///
/// The returned value is intentionally a `GateProperties` value rather than a
/// circuit aggregate.
#[must_use]
pub fn gate_properties(gate: &Gate) -> GateProperties {
    GateProperties::from_gate(gate)
}

// =============================================================================
// Gate properties
// =============================================================================

/// Semantic properties of one canonical gate.
///
/// This is deliberately smaller than the optimizer's operation-property
/// abstraction. It exists at the IR-analysis boundary and does not contain
/// optimization transformation semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GateProperties {
    kind: GateKind,
    operand_count: usize,
    parameter_count: usize,
    unitary: bool,
    measurement: bool,
    reset: bool,
    barrier: bool,
    parameterized: bool,
    multi_qubit: bool,
    classical_target: bool,
    state_changing: bool,
    canonical: bool,
}

impl GateProperties {
    fn from_gate(gate: &Gate) -> Self {
        let kind = gate.kind();
        let operand_count = gate.qubits().len();
        let parameter_count = gate.parameters().len();

        Self {
            kind,
            operand_count,
            parameter_count,
            unitary: kind.is_unitary(),
            measurement: kind.is_measurement(),
            reset: kind.is_reset(),
            barrier: kind.is_barrier(),
            parameterized: parameter_count != 0,
            multi_qubit: operand_count > 1,
            classical_target: gate.classical_target().is_some(),
            state_changing: is_state_changing_kind(kind),
            canonical: is_canonical_kind(kind),
        }
    }

    /// Returns the canonical gate kind.
    #[must_use]
    pub const fn kind(&self) -> GateKind {
        self.kind
    }

    /// Returns the number of logical-qubit operands.
    #[must_use]
    pub const fn operand_count(&self) -> usize {
        self.operand_count
    }

    /// Returns the number of parameters.
    #[must_use]
    pub const fn parameter_count(&self) -> usize {
        self.parameter_count
    }

    /// Returns whether the gate is unitary according to canonical `GateKind`
    /// semantics.
    #[must_use]
    pub const fn is_unitary(&self) -> bool {
        self.unitary
    }

    /// Returns whether the gate is non-unitary.
    #[must_use]
    pub const fn is_non_unitary(&self) -> bool {
        !self.unitary
    }

    /// Returns whether the gate is measurement.
    #[must_use]
    pub const fn is_measurement(&self) -> bool {
        self.measurement
    }

    /// Returns whether the gate is reset.
    #[must_use]
    pub const fn is_reset(&self) -> bool {
        self.reset
    }

    /// Returns whether the gate is a barrier.
    #[must_use]
    pub const fn is_barrier(&self) -> bool {
        self.barrier
    }

    /// Returns whether the gate is parameterized.
    #[must_use]
    pub const fn is_parameterized(&self) -> bool {
        self.parameterized
    }

    /// Returns whether the gate acts on more than one logical qubit.
    #[must_use]
    pub const fn is_multi_qubit(&self) -> bool {
        self.multi_qubit
    }

    /// Returns whether the gate has a classical destination.
    #[must_use]
    pub const fn has_classical_target(&self) -> bool {
        self.classical_target
    }

    /// Returns whether the operation can change quantum state.
    ///
    /// Measurement and barrier are not classified as state-changing operations
    /// here. Reset is state-changing because it prepares the logical qubit in a
    /// defined state.
    #[must_use]
    pub const fn changes_quantum_state(&self) -> bool {
        self.state_changing
    }

    /// Returns whether the gate belongs to the current canonical standard
    /// gate-kind universe.
    #[must_use]
    pub const fn is_canonical(&self) -> bool {
        self.canonical
    }
}

// =============================================================================
// Internal classification
// =============================================================================

fn classify_gate(
    gate: &Gate,
    profile: &mut PropertyProfile,
    referenced_qubits: &mut std::collections::BTreeSet<QubitId>,
) {
    let kind = gate.kind();

    for &qubit in gate.qubits() {
        referenced_qubits.insert(qubit);
    }

    let properties = GateProperties::from_gate(gate);

    if properties.is_unitary() {
        profile.unitary_operations = profile
            .unitary_operations
            .saturating_add(1);
    } else {
        profile.non_unitary_operations = profile
            .non_unitary_operations
            .saturating_add(1);
    }

    if properties.is_measurement() {
        profile.measurement_operations = profile
            .measurement_operations
            .saturating_add(1);
    }

    if properties.is_reset() {
        profile.reset_operations = profile
            .reset_operations
            .saturating_add(1);
    }

    if properties.is_barrier() {
        profile.barrier_operations = profile
            .barrier_operations
            .saturating_add(1);
    }

    if properties.is_parameterized() {
        profile.parameterized_operations = profile
            .parameterized_operations
            .saturating_add(1);
    }

    if properties.is_multi_qubit() {
        profile.multi_qubit_operations = profile
            .multi_qubit_operations
            .saturating_add(1);
    }

    if properties.has_classical_target() {
        profile.classical_target_operations = profile
            .classical_target_operations
            .saturating_add(1);
    }

    if properties.changes_quantum_state() {
        profile.state_changing_operations = profile
            .state_changing_operations
            .saturating_add(1);
    }

    if properties.is_canonical() {
        profile.canonical_operations = profile
            .canonical_operations
            .saturating_add(1);
    } else {
        profile.non_canonical_operations = profile
            .non_canonical_operations
            .saturating_add(1);
    }

    // Keep this explicit binding so future additions to the canonical gate
    // universe cannot accidentally make `kind` unused when the classifier is
    // expanded.
    let _ = kind;
}

const fn is_state_changing_kind(kind: GateKind) -> bool {
    match kind {
        GateKind::Measure => false,
        GateKind::Barrier => false,

        // Reset explicitly prepares the qubit and therefore changes its
        // quantum state semantics.
        GateKind::Reset => true,

        // All recognized mathematical gates are capable of changing the
        // state, except identity.
        GateKind::I => false,

        _ => true,
    }
}

/// Returns whether the operation belongs to the currently compiled-in
/// canonical `GateKind` dialect.
///
/// Because `GateKind` is an exhaustive Rust enum, every value currently
/// representable by this module is canonical.
///
/// The function is intentionally isolated because the future dialect system
/// will eventually replace this finite classification with dialect-aware
/// operation identity.
///
/// When extensible operation kinds become available, this function should be
/// replaced by a dialect registry query without changing the public
/// `CircuitProperties` contract.
const fn is_canonical_kind(_kind: GateKind) -> bool {
    true
}

// =============================================================================
// Property assertions
// =============================================================================

/// Returns whether a circuit satisfies the requested semantic property.
///
/// This is a convenience API for code that needs one property and does not
/// need the complete profile.
///
/// The circuit is analyzed once for this call.
#[must_use]
pub fn has_property(
    circuit: &QuantumCircuit,
    property: CircuitProperty,
) -> bool {
    analyze(circuit).has_property(property)
}

/// Returns whether the circuit is strictly unitary.
///
/// This is equivalent to:
///
/// ```text
/// properties(circuit).is_strictly_unitary()
/// ```
///
/// A circuit containing measurement, reset or barrier is not strictly unitary.
#[must_use]
pub fn is_strictly_unitary(circuit: &QuantumCircuit) -> bool {
    analyze(circuit).is_strictly_unitary()
}

/// Returns whether the circuit contains measurement.
#[must_use]
pub fn has_measurement(circuit: &QuantumCircuit) -> bool {
    analyze(circuit).has_measurement()
}

/// Returns whether the circuit contains reset.
#[must_use]
pub fn has_reset(circuit: &QuantumCircuit) -> bool {
    analyze(circuit).has_reset()
}

/// Returns whether the circuit contains barriers.
#[must_use]
pub fn has_barrier(circuit: &QuantumCircuit) -> bool {
    analyze(circuit).has_barrier()
}

/// Returns whether the circuit contains parameterized operations.
#[must_use]
pub fn is_parameterized(circuit: &QuantumCircuit) -> bool {
    analyze(circuit).is_parameterized()
}

/// Returns whether the circuit contains multi-qubit operations.
#[must_use]
pub fn has_multi_qubit_operation(circuit: &QuantumCircuit) -> bool {
    analyze(circuit).has_multi_qubit_operation()
}

/// Returns whether the circuit contains a classical destination.
#[must_use]
pub fn has_classical_target(circuit: &QuantumCircuit) -> bool {
    analyze(circuit).has_classical_target()
}

/// Returns whether the circuit contains any state-changing operation.
#[must_use]
pub fn has_quantum_state_change(circuit: &QuantumCircuit) -> bool {
    analyze(circuit).has_quantum_state_change()
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn property_identifiers_are_stable_non_empty_strings() {
        let properties = [
            CircuitProperty::NonEmpty,
            CircuitProperty::Empty,
            CircuitProperty::StrictlyUnitary,
            CircuitProperty::NonUnitary,
            CircuitProperty::HasMeasurement,
            CircuitProperty::MeasurementFree,
            CircuitProperty::HasReset,
            CircuitProperty::ResetFree,
            CircuitProperty::HasBarrier,
            CircuitProperty::BarrierFree,
            CircuitProperty::Parameterized,
            CircuitProperty::ParameterFree,
            CircuitProperty::MultiQubit,
            CircuitProperty::SingleQubitOnly,
            CircuitProperty::HasClassicalTarget,
            CircuitProperty::ClassicalTargetFree,
            CircuitProperty::HasQuantumStateChange,
            CircuitProperty::NoQuantumStateChange,
            CircuitProperty::CanonicalGateSet,
            CircuitProperty::NonCanonicalGateSet,
            CircuitProperty::SingleDeclaredQubit,
            CircuitProperty::MultipleDeclaredQubits,
            CircuitProperty::NoReferencedQubits,
            CircuitProperty::HasReferencedQubits,
        ];

        for property in properties {
            assert!(!property.as_str().is_empty());
        }
    }

    #[test]
    fn gate_properties_identity_are_conservative() {
        // This test intentionally uses GateKind directly rather than
        // constructing a complete circuit. The classification contract for
        // I is stable independently of circuit construction APIs.
        assert!(GateKind::I.is_unitary());
        assert!(!is_state_changing_kind(GateKind::I));
        assert!(is_canonical_kind(GateKind::I));
    }

    #[test]
    fn measurement_is_non_unitary() {
        assert!(!GateKind::Measure.is_unitary());
        assert!(GateKind::Measure.is_measurement());
        assert!(!is_state_changing_kind(GateKind::Measure));
    }

    #[test]
    fn reset_is_non_unitary_and_state_changing() {
        assert!(!GateKind::Reset.is_unitary());
        assert!(GateKind::Reset.is_reset());
        assert!(is_state_changing_kind(GateKind::Reset));
    }

    #[test]
    fn barrier_is_not_strictly_unitary() {
        assert!(GateKind::Barrier.is_unitary());

        let profile = PropertyProfile::empty(0);
        assert!(!profile.is_strictly_unitary());

        let mut with_barrier = profile;
        with_barrier.operation_count = 1;
        with_barrier.unitary_operations = 1;
        with_barrier.barrier_operations = 1;

        assert!(!with_barrier.is_strictly_unitary());
    }

    #[test]
    fn property_complements_are_consistent() {
        let mut profile = PropertyProfile::empty(2);
        profile.operation_count = 1;
        profile.unitary_operations = 1;
        profile.referenced_qubits = 1;

        assert!(profile.is_non_empty());
        assert!(!profile.is_empty());

        assert!(profile.is_strictly_unitary());
        assert!(!profile.is_non_unitary());

        assert!(!profile.has_measurement());
        assert!(!profile.has_reset());
        assert!(!profile.has_barrier());
        assert!(!profile.is_parameterized());
        assert!(profile.is_single_qubit_only());
        assert!(!profile.has_classical_target());
        assert!(profile.has_quantum_state_change());
        assert!(profile.has_referenced_qubits());
    }

    #[test]
    fn property_profile_does_not_materialize_declared_qubit_namespace() {
        let profile = PropertyProfile::empty(usize::MAX);

        assert_eq!(profile.declared_qubits(), usize::MAX);
        assert_eq!(profile.referenced_qubits(), 0);
        assert!(profile.is_empty());
    }

    #[test]
    fn circuit_property_queries_are_pure_profile_queries() {
        let profile = PropertyProfile::empty(1);

        assert!(profile.has_property(CircuitProperty::Empty));
        assert!(!profile.has_property(CircuitProperty::NonEmpty));
        assert!(profile.has_property(CircuitProperty::MeasurementFree));
        assert!(profile.has_property(CircuitProperty::ResetFree));
        assert!(profile.has_property(CircuitProperty::BarrierFree));
        assert!(profile.has_property(CircuitProperty::ParameterFree));
        assert!(profile.has_property(CircuitProperty::SingleQubitOnly));
        assert!(profile.has_property(CircuitProperty::ClassicalTargetFree));
        assert!(profile.has_property(CircuitProperty::NoQuantumStateChange));
        assert!(profile.has_property(CircuitProperty::SingleDeclaredQubit));
        assert!(profile.has_property(CircuitProperty::NoReferencedQubits));
    }
}