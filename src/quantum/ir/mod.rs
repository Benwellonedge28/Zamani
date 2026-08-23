//! Zamani Quantum Intermediate Representation.
//!
//! Canonical, hardware-independent representation of logical quantum
//! programs.
//!
//! # Architectural boundary
//!
//! The Quantum IR represents the logical program itself. It deliberately does
//! not contain:
//!
//! - physical hardware topology;
//! - logical-to-physical routing;
//! - pulse schedules;
//! - calibration;
//! - backend-specific gate decomposition;
//! - QPU communication;
//! - hardware execution;
//! - error-correction decoding;
//! - hardware-specific error-correction geometry;
//! - optimization algorithms;
//! - frontend parsing.
//!
//! Those responsibilities belong to downstream quantum compiler/backend
//! subsystems.
//!
//! # Public API
//!
//! This module is the stable public boundary for the Quantum IR. Individual
//! implementation modules own their internal algorithms and local types, while
//! this module controls what the rest of Zamani is expected to consume.
//!
//! The intended dependency direction is:
//!
//! ```text
//! limits
//!    │
//!    ├──────────────┐
//!    │              │
//! errors       identity / parameter / qubit
//!    │              │
//!    └──────┬───────┘
//!           │
//!      measurement
//!           │
//!         gate
//!           │
//!      validation
//!           │
//!        circuit
//!           │
//!        analysis
//!           │
//!       integration
//! ```
//!
//! Frontends consume this module's public API when lowering validated
//! source-language representations into canonical Quantum IR.
//!
//! ```text
//! external format
//!       │
//!       ▼
//! quantum::frontend
//!       │
//!       │ validated lowering
//!       ▼
//! quantum::ir
//!       │
//!       ▼
//! compiler / algorithms
//!       │
//!       ▼
//! backend / hardware
//! ```
//!
//! `mod.rs` itself contains no domain logic. Its responsibility is:
//!
//! 1. declare the canonical IR modules;
//! 2. expose the stable public types;
//! 3. expose canonical validation and analysis entry points;
//! 4. provide a controlled prelude;
//! 5. keep implementation-only details out of the public boundary.
//!
//! # Module/file naming
//!
//! The canonical qubit implementation is `qubit.rs`, therefore the Rust
//! module is `qubit`, not `qubits`.
//!
//! This distinction is intentional. Rust module declarations must correspond
//! to the actual source file layout:
//!
//! ```text
//! src/quantum/ir/qubit.rs
//!             │
//!             └── pub mod qubit;
//! ```
//!
//! # Rust compatibility
//!
//! This module targets Rust 1.97.1.
//!
//! No nightly features are required.
//! No external dependencies are required.

// =============================================================================
// Canonical IR modules
// =============================================================================

/// Deterministic, read-only circuit analysis.
pub mod analysis;

/// Canonical logical circuit container.
pub mod circuit;

/// Canonical Quantum IR error vocabulary.
pub mod errors;

/// Quantum gate definitions and gate-level semantics.
pub mod gate;

/// Strongly typed IR, circuit, and operation identities.
pub mod identity;

/// Resource limits and overflow-safe resource accounting.
pub mod limits;

/// Hardware-independent measurement semantics.
pub mod measurement;

/// Typed quantum gate parameters.
pub mod parameter;

/// Logical and physical qubit identity and registration types.
///
/// The implementation is located in `qubit.rs`.
pub mod qubit;

/// Canonical whole-IR validation.
pub mod validation;

// =============================================================================
// Integration tests
// =============================================================================

/// Cross-module Quantum IR integration tests.
///
/// Unit tests that belong exclusively to one implementation module should
/// remain next to that implementation. This test module is reserved for
/// contracts spanning multiple IR components.
#[cfg(test)]
mod tests;

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
//
// `errors.rs` is intentionally independent of the implementation modules.
// These are the errors that compiler-wide code should prefer when crossing
// the Quantum IR boundary.

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
//
// `Parameter` is the canonical parameter abstraction for the upgraded IR.
//
// `GateParameter` remains exported because gate.rs currently exposes it as
// part of its public construction API. New compiler code should prefer the
// canonical `Parameter` abstraction where applicable.

pub use parameter::Parameter;

// =============================================================================
// Qubit API
// =============================================================================

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

/// Stable collection of the primary Quantum IR types.
///
/// Downstream compiler stages should normally import from this prelude rather
/// than reaching into individual IR implementation modules.
///
/// The prelude intentionally excludes:
///
/// - specialized internal errors that are only useful for implementation
///   details;
/// - analysis helper implementation types that are not part of the common
///   compiler contract;
/// - limits internals;
/// - validation internals.
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