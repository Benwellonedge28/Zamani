//! Zamani Quantum Intermediate Representation.
//!
//! Canonical, hardware-independent representation of quantum computation.
//!
//! # Architectural role
//!
//! `quantum::ir` is the semantic contract between the Zamani language/frontend
//! and all downstream quantum compilation and execution infrastructure.
//!
//! The IR answers:
//!
//! > What does this quantum program mean?
//!
//! It does NOT decide:
//!
//! - which physical machine executes the program;
//! - which physical qubits are selected;
//! - how logical qubits are routed;
//! - which hardware-native instruction is selected;
//! - which calibration is applied;
//! - how operations are scheduled;
//! - how pulses are synthesized for a particular device;
//! - how a QPU is contacted;
//! - how quantum state is simulated;
//! - how quantum error correction is decoded;
//! - which optimization algorithm is used;
//! - how source syntax is parsed.
//!
//! Those responsibilities belong to downstream subsystems.
//!
//! # Universal-program principle
//!
//! A Zamani quantum program is written once at the semantic level and may be
//! lowered to any compatible target for which sufficient resources and
//! capabilities exist.
//!
//! The IR therefore has no architectural fixed quantum-machine size.
//!
//! These are all semantically the same kind of resource declaration:
//!
//! ```text
//! 1 qubit
//! 2 qubits
//! 63 qubits
//! 64 qubits
//! 128 qubits
//! 4_096 qubits
//! 1_000_000 qubits
//! N qubits
//! ```
//!
//! The actual finite size of a compilation or execution is constrained by:
//!
//! 1. explicit IR resource/security policies;
//! 2. host memory and integer/address-space limits;
//! 3. compiler/runtime limits;
//! 4. target hardware capacity;
//! 5. target topology and capabilities;
//! 6. backend/execution constraints.
//!
//! None of those concrete constraints are the semantic maximum of Zamani.
//!
//! In particular, values such as `63` or `4096` must never silently become
//! architectural qubit-count limits.
//!
//! # Three layers of quantum truth
//!
//! ```text
//! SEMANTIC TRUTH
//!       │
//!       ▼
//! quantum::ir
//!       │
//!       │ What does the program mean?
//!       ▼
//! TARGET TRUTH
//!       │
//!       ▼
//! quantum::hardware
//!       │
//!       │ What physical resources exist?
//!       ▼
//! EXECUTION TRUTH
//!       │
//!       ▼
//! backend / runtime
//!
//!       How is this particular target executed?
//! ```
//!
//! # Dependency boundary
//!
//! The canonical dependency direction is:
//!
//! ```text
//!                    Zamani source
//!                         │
//!                         ▼
//!                    frontend
//!                         │
//!                         ▼
//!                 ┌───────────────┐
//!                 │ quantum::ir   │
//!                 │               │
//!                 │ semantic WHAT │
//!                 └───────┬───────┘
//!                         │
//!        ┌────────────────┼────────────────┐
//!        │                │                │
//!        ▼                ▼                ▼
//! optimization       routing         scheduling
//!        │                │                │
//!        └────────────────┼────────────────┘
//!                         ▼
//!                    hardware
//!                         │
//!                         ▼
//!                     backend
//!                         │
//!                         ▼
//!                    execution
//! ```
//!
//! The IR MUST NOT depend on:
//!
//! - `quantum::frontend`;
//! - `quantum::optimization`;
//! - `quantum::routing`;
//! - `quantum::scheduling`;
//! - `quantum::hardware`;
//! - `quantum::simulator`;
//! - `quantum::qec`;
//! - backend execution implementations.
//!
//! Those systems may depend on the IR.
//!
//! # Universal quantum-program model
//!
//! `QuantumCircuit` remains an important gate-oriented representation, but it
//! is not the complete definition of a quantum program.
//!
//! The broader IR supports:
//!
//! ```text
//! Quantum program
//! │
//! ├── declarations
//! ├── logical qubits
//! ├── classical resources
//! ├── parameters
//! ├── operations
//! ├── regions / blocks
//! ├── control flow
//! ├── measurements
//! ├── timing
//! ├── pulse semantics
//! ├── waveform semantics
//! ├── channel references
//! ├── frame semantics
//! ├── resource requirements
//! ├── capability requirements
//! ├── logical/physical mapping records
//! ├── provenance
//! ├── canonical serialization
//! ├── canonical hashing
//! └── extensions
//! ```
//!
//! This allows the same semantic representation to cover:
//!
//! - gate-based quantum computing;
//! - dynamic circuits;
//! - mid-circuit measurement;
//! - classical feedback;
//! - pulse-level control;
//! - analog quantum computation;
//! - annealing / Ising / QUBO workloads;
//! - logical and fault-tolerant quantum operations;
//! - hybrid quantum/classical programs;
//! - distributed quantum workloads;
//! - simulator targets;
//! - future quantum architectures.
//!
//! # Pulse-level control
//!
//! Zamani source such as:
//!
//! ```text
//! fn x_gate(q) {
//!     pulse(amp=0.3, dur=20ns)
//! }
//! ```
//!
//! is represented semantically through the pulse/operation/timing portions of
//! the IR.
//!
//! The IR can express:
//!
//! ```text
//! amplitude = 0.3
//! duration  = 20ns
//! target    = logical q
//! ```
//!
//! but it does not decide:
//!
//! - which DAC is used;
//! - which physical drive channel is selected;
//! - which carrier frequency is used;
//! - which calibration is applied;
//! - which native instruction is emitted;
//! - how the device-specific waveform is generated.
//!
//! Those decisions belong downstream.
//!
//! # Logical and physical qubits
//!
//! The canonical logical and physical identity types are owned by
//! `quantum::ir::qubit`:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! `qubit.rs` is therefore the authoritative module.
//!
//! `qubits` is retained below only as a compatibility alias for older
//! repository code. New code MUST use `quantum::ir::qubit`.
//!
//! # Resource policy versus architectural capability
//!
//! `QuantumIrLimits` is an explicit resource/security policy.
//!
//! It MUST NOT be interpreted as the maximum quantum computer Zamani supports.
//!
//! For example:
//!
//! ```text
//! QuantumIrLimits
//!     = "how much work this compiler/service invocation permits"
//!
//! quantum::hardware
//!     = "what the selected target physically provides"
//! ```
//!
//! This distinction is essential for scaling from tiny systems to very large
//! systems.
//!
//! # Stable API boundary
//!
//! This module is intentionally thin.
//!
//! It owns:
//!
//! 1. canonical module declarations;
//! 2. public API exposure;
//! 3. compatibility aliases;
//! 4. controlled prelude exports;
//! 5. integration-test registration.
//!
//! It does NOT contain quantum-domain algorithms.
//!
//! # Module ownership
//!
//! ```text
//! analysis.rs
//!     Read-only circuit/program analysis.
//!
//! attribute.rs
//!     Typed extensible metadata.
//!
//! capability.rs
//!     Hardware-independent capability requirements.
//!
//! channel.rs
//!     Abstract control-channel semantics.
//!
//! circuit.rs
//!     Ordered gate-oriented quantum circuit container.
//!
//! classical.rs
//!     Classical values, bits, expressions and predicates.
//!
//! control_flow.rs
//!     Dynamic quantum/classical control flow.
//!
//! errors.rs
//!     Canonical IR error vocabulary.
//!
//! extension.rs
//!     Forward-compatible IR extensions.
//!
//! frame.rs
//!     Abstract frame and phase/frequency semantics.
//!
//! gate.rs
//!     Mathematical/logical gate semantics.
//!
//! hash.rs
//!     Canonical content hashing.
//!
//! identity.rs
//!     Stable IR object identities and IR version.
//!
//! limits.rs
//!     Explicit resource/security limits.
//!
//! mapping.rs
//!     Logical-to-physical mapping records.
//!
//! measurement.rs
//!     Hardware-independent measurement semantics.
//!
//! operation.rs
//!     Universal operation model.
//!
//! parameter.rs
//!     Typed and symbolic parameters.
//!
//! program.rs
//!     Top-level quantum-program representation.
//!
//! provenance.rs
//!     Transformation and compilation lineage.
//!
//! pulse.rs
//!     Hardware-independent pulse semantics.
//!
//! qubit.rs
//!     Canonical logical/physical qubit identity.
//!
//! region.rs
//!     Structured program regions and blocks.
//!
//! resource.rs
//!     Abstract resource requirements.
//!
//! schedule.rs
//!     Semantic scheduled-operation representation.
//!
//! serialization.rs
//!     Canonical IR persistence/encoding.
//!
//! timing.rs
//!     Time and duration semantics.
//!
//! types.rs
//!     Canonical IR type vocabulary.
//!
//! validation.rs
//!     Structural and semantic IR validation.
//!
//! value.rs
//!     Canonical typed IR values.
//!
//! waveform.rs
//!     Hardware-independent waveform semantics.
//!
//! tests.rs
//!     Cross-module integration contracts.
//! ```
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! The module explicitly forbids unsafe code.
//!
//! # Integration policy
//!
//! Every downstream subsystem should consume the canonical module paths:
//!
//! ```text
//! quantum::ir::qubit
//! quantum::ir::gate
//! quantum::ir::operation
//! quantum::ir::program
//! quantum::ir::pulse
//! quantum::ir::measurement
//! quantum::ir::classical
//! ```
//!
//! Do not create duplicate quantum IR type definitions in downstream modules.
//!
//! If a downstream subsystem needs a specialized representation, it must
//! explicitly convert to/from the canonical IR boundary.
//!
//! # Versioning
//!
//! `identity::IrVersion` is the canonical schema/semantic version.
//!
//! `mod.rs` does not create a second versioning system.
//!
//! Future breaking semantic changes MUST be represented through the canonical
//! IR version contract rather than silently changing the meaning of existing
//! structures.
//!
//! # Serialization boundary
//!
//! `serialization.rs` owns persistence and canonical encoding.
//!
//! `mod.rs` deliberately does not define a second serialization format.
//!
//! # Hashing boundary
//!
//! `hash.rs` owns canonical content identity.
//!
//! `mod.rs` deliberately does not calculate hashes.
//!
//! # Testing boundary
//!
//! Module-local tests belong inside the corresponding implementation file.
//!
//! `tests.rs` is registered here only for cross-module integration contracts.
//!
//! # Important compatibility guarantee
//!
//! Existing users of:
//!
//! ```text
//! quantum::ir::QubitId
//! quantum::ir::Gate
//! quantum::ir::QuantumCircuit
//! ```
//!
//! remain supported through curated root re-exports.
//!
//! New code should prefer explicit canonical module paths where ambiguity
//! could exist.
//!
//! -----------------------------------------------------------------------------
//! No domain logic belongs below this point.
//! -----------------------------------------------------------------------------

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

// =============================================================================
// Canonical foundation modules
// =============================================================================

/// Deterministic read-only IR and circuit analysis.
pub mod analysis;

/// Typed extensible IR metadata and attributes.
pub mod attribute;

/// Hardware-independent capability requirements.
pub mod capability;

/// Abstract hardware-independent control-channel semantics.
pub mod channel;

/// Ordered gate-oriented quantum circuit representation.
pub mod circuit;

/// Classical bits, values, expressions, assignments and predicates.
pub mod classical;

/// Dynamic quantum/classical control flow.
pub mod control_flow;

/// Canonical Quantum IR error vocabulary.
pub mod errors;

/// Forward-compatible extensibility mechanisms.
pub mod extension;

/// Hardware-independent frame semantics.
pub mod frame;

/// Mathematical and logical gate semantics.
pub mod gate;

/// Canonical IR content hashing.
pub mod hash;

/// Stable IR object identities and IR schema versioning.
pub mod identity;

/// Explicit resource and security policy.
pub mod limits;

/// Logical-to-physical mapping records.
pub mod mapping;

/// Hardware-independent measurement semantics.
pub mod measurement;

/// Universal canonical operation model.
pub mod operation;

/// Typed and symbolic parameter semantics.
pub mod parameter;

/// Top-level universal quantum-program representation.
pub mod program;

/// Compilation and transformation provenance.
pub mod provenance;

/// Hardware-independent pulse semantics.
pub mod pulse;

/// Canonical logical and physical qubit identity.
///
/// The authoritative module path is:
///
/// `quantum::ir::qubit`
pub mod qubit;

/// Structured program regions and blocks.
pub mod region;

/// Abstract quantum/classical resource requirements.
pub mod resource;

/// Semantic scheduled-operation representation.
pub mod schedule;

/// Canonical IR serialization and persistence.
pub mod serialization;

/// Time, duration and temporal semantics.
pub mod timing;

/// Canonical IR type vocabulary.
pub mod types;

/// Whole-IR structural and semantic validation.
pub mod validation;

/// Canonical typed IR values.
pub mod value;

/// Hardware-independent waveform semantics.
pub mod waveform;

// =============================================================================
// Compatibility aliases
// =============================================================================

/// Compatibility alias for legacy code that still refers to the old
/// `qubits` module name.
///
/// # Canonical path
///
/// New code MUST use:
///
/// `quantum::ir::qubit`
///
/// This alias exists so that older repository code can transition without
/// duplicating or redefining `QubitId` and `PhysicalQubitId`.
pub use qubit as qubits;

// =============================================================================
// Canonical circuit API
// =============================================================================

pub use circuit::{
    CircuitError,
    CircuitMetadata,
    QuantumCircuit,
};

// =============================================================================
// Canonical error API
// =============================================================================

pub use errors::{
    IrError,
    IrErrorKind,
    IrGateError,
    IrIdentifierError,
    IrLimitError,
    IrMeasurementError,
    IrParameterError,
    IrQubitError,
    IrResult,
};

// =============================================================================
// Gate API
// =============================================================================

pub use gate::{
    Gate,
    GateError,
    GateKind,
    GateParameter,
};

// =============================================================================
// Identity API
// =============================================================================

pub use identity::{
    CircuitId,
    IrVersion,
    OperationId,
};

// =============================================================================
// Limits API
// =============================================================================

pub use limits::{
    LimitsError,
    QuantumIrLimits,
};

// =============================================================================
// Measurement API
// =============================================================================

pub use measurement::{
    measure,
    measure_x,
    measure_y,
    ClassicalBitId,
    ClassicalRegister,
    Measurement,
    MeasurementBasis,
    MeasurementError,
    MeasurementGroup,
    MeasurementMode,
};

// =============================================================================
// Parameter API
// =============================================================================

pub use parameter::Parameter;

// =============================================================================
// Qubit API
// =============================================================================
//
// IMPORTANT:
// `QubitId` and `PhysicalQubitId` come from `qubit.rs`, never `qubits.rs`.

pub use qubit::{
    validate_qubits,
    validate_unique_qubits,
    PhysicalQubitId,
    Qubit,
    QubitError,
    QubitId,
    QubitRegister,
    QubitState,
};

// =============================================================================
// Validation API
// =============================================================================

pub use validation::{
    validate_circuit,
    validate_circuit_with_config,
    validate_circuit_with_limits,
    validate_gate,
    validate_measurement,
    validate_operation,
    ValidationConfig,
};

// =============================================================================
// Analysis API
// =============================================================================

pub use analysis::{
    analyze,
    analyze_with_limits,
    basic_statistics,
    basic_statistics_with_limits,
    BasicCircuitStatistics,
    CircuitStatistics,
    GateKindCount,
    QubitUsage,
};

// =============================================================================
// Controlled prelude
// =============================================================================

/// Stable common-import surface for downstream quantum compiler stages.
///
/// The prelude intentionally contains the most commonly consumed semantic
/// types and entry points. Specialized modules remain available through their
/// canonical paths.
///
/// Example:
///
/// ```rust
/// use crate::quantum::ir::prelude::{
///     Gate,
///     GateKind,
///     Parameter,
///     QubitId,
///     QuantumCircuit,
/// };
/// ```
pub mod prelude {
    pub use super::{
        analyze,
        analyze_with_limits,
        basic_statistics,
        basic_statistics_with_limits,
        measure,
        measure_x,
        measure_y,
        validate_circuit,
        validate_circuit_with_config,
        validate_circuit_with_limits,
        validate_gate,
        validate_measurement,
        validate_operation,
        BasicCircuitStatistics,
        CircuitError,
        CircuitId,
        CircuitMetadata,
        CircuitStatistics,
        ClassicalBitId,
        ClassicalRegister,
        Gate,
        GateKind,
        GateKindCount,
        GateParameter,
        IrError,
        IrErrorKind,
        IrResult,
        IrVersion,
        Measurement,
        MeasurementBasis,
        MeasurementGroup,
        MeasurementMode,
        OperationId,
        Parameter,
        PhysicalQubitId,
        Qubit,
        QubitId,
        QubitRegister,
        QubitState,
        QuantumCircuit,
        QuantumIrLimits,
        ValidationConfig,
    };
}

// =============================================================================
// Integration-test registration
// =============================================================================
//
// Tests spanning multiple IR modules belong here. Individual modules retain
// their own unit tests.
//
// This is intentionally the final declaration so that the complete public
// module graph is available to the integration suite.

#[cfg(test)]
mod tests;