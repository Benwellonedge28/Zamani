//! Zamani Quantum IR — Quantum Domain Facade
//!
//! This module is the quantum-domain namespace inside the canonical Zamani
//! Quantum Intermediate Representation.
//!
//! # Architectural role
//!
//! `quantum::ir::quantum` provides the quantum-semantic portion of the
//! canonical IR without creating a second IR implementation.
//!
//! The canonical dependency boundary is:
//!
//! ```text
//! Zamani source
//!      │
//!      ▼
//! quantum::frontend
//!      │
//!      ▼
//! quantum::ir
//!      │
//!      ├── quantum
//!      │    ├── qubit
//!      │    ├── gate
//!      │    ├── measurement
//!      │    ├── instruction
//!      │    └── channel
//!      │
//!      ├── classical
//!      ├── control
//!      ├── timing
//!      ├── pulse
//!      ├── program
//!      ├── resources
//!      └── models
//!      │
//!      ▼
//! target-independent compilation
//!      │
//!      ▼
//! routing / scheduling / hardware lowering
//!      │
//!      ▼
//! backend / runtime
//! ```
//!
//! The purpose of this module is therefore namespace composition and public
//! API organization. It does not contain hardware-specific implementation,
//! routing algorithms, scheduling algorithms, simulator state, QEC decoders,
//! frontend parsing, or backend execution.
//!
//! # Critical ownership rule
//!
//! This module MUST NOT redefine the canonical quantum IR types.
//!
//! In particular, there must be exactly one authoritative definition of:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! quantum::ir::qubit::PhysicalQubitId
//! quantum::ir::gate::Gate
//! quantum::ir::measurement::Measurement
//! ```
//!
//! Existing repository modules already own those definitions.
//!
//! This facade therefore re-exports the existing canonical modules instead of
//! copying their types.
//!
//! # Qubit identity
//!
//! The authoritative qubit module is:
//!
//! ```text
//! quantum::ir::qubit
//! ```
//!
//! New implementation code MUST import logical qubit identities from:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! ```
//!
//! or, from a sibling IR module:
//!
//! ```text
//! super::qubit::QubitId
//! ```
//!
//! depending on the module's location.
//!
//! This module MUST NOT introduce:
//!
//! ```text
//! quantum::ir::quantum::QubitId
//! ```
//!
//! as a second type.
//!
//! # Universal quantum-program principle
//!
//! The quantum domain is intentionally independent of machine size.
//!
//! The following are all valid semantic scales:
//!
//! ```text
//! 1 qubit
//! 2 qubits
//! 10 qubits
//! 100 qubits
//! 1_000 qubits
//! 1_000_000 qubits
//! N qubits
//! ```
//!
//! There is no architectural maximum encoded here.
//!
//! A concrete compilation may still be bounded by:
//!
//! - host memory;
//! - address-space limits;
//! - compiler resource policies;
//! - `QuantumIrLimits`;
//! - target capabilities;
//! - target capacity;
//! - target topology;
//! - backend limits;
//! - execution environment.
//!
//! Those are execution/resource constraints, not semantic limits of Zamani.
//!
//! # Program written once
//!
//! The semantic quantum representation must remain independent of:
//!
//! - IBM;
//! - IonQ;
//! - Quantinuum;
//! - Rigetti;
//! - neutral-atom hardware;
//! - photonic hardware;
//! - spin hardware;
//! - superconducting hardware;
//! - annealers;
//! - simulators;
//! - GPUs;
//! - CPUs;
//! - a particular number of physical qubits.
//!
//! A downstream compiler may lower the same canonical representation to any
//! compatible target.
//!
//! # What belongs here
//!
//! This namespace exposes quantum-semantic concepts such as:
//!
//! - logical qubits;
//! - physical-qubit identity vocabulary;
//! - registers;
//! - gate semantics;
//! - quantum instructions;
//! - measurements;
//! - reset;
//! - initialization;
//! - abstract channels.
//!
//! # What does NOT belong here
//!
//! This namespace must not own:
//!
//! - hardware topology;
//! - logical-to-physical routing algorithms;
//! - placement algorithms;
//! - scheduling algorithms;
//! - device calibration databases;
//! - DAC implementation;
//! - waveform synthesis algorithms;
//! - simulator state;
//! - state vectors;
//! - density matrices;
//! - tensor-network execution;
//! - QEC decoding;
//! - backend API calls;
//! - credentials;
//! - source-language parsing.
//!
//! Those responsibilities belong to downstream or sibling subsystems.
//!
//! # Module composition strategy
//!
//! At the current repository stage, the canonical implementations live
//! directly under `quantum::ir`:
//!
//! ```text
//! quantum::ir::qubit
//! quantum::ir::gate
//! quantum::ir::measurement
//! quantum::ir::channel
//! quantum::ir::pulse
//! quantum::ir::frame
//! quantum::ir::waveform
//! ```
//!
//! This module provides the nested quantum-domain namespace without moving or
//! duplicating those implementations.
//!
//! That is intentional.
//!
//! It permits the repository to evolve toward:
//!
//! ```text
//! quantum::ir::quantum::*
//! ```
//!
//! without requiring simultaneous rewrites of all existing IR consumers.
//!
//! # Migration rule
//!
//! The nested namespace is a compatibility/facade boundary.
//!
//! New files that are genuinely quantum-domain implementations should be
//! introduced deliberately under the future `quantum/` hierarchy only when
//! their ownership is stable.
//!
//! Existing canonical definitions must not be copied merely to satisfy the
//! directory layout.
//!
//! # Integration contract
//!
//! This module depends only on sibling canonical IR modules.
//!
//! It MUST NOT depend on:
//!
//! ```text
//! quantum::frontend
//! quantum::optimization
//! quantum::routing
//! quantum::scheduling
//! quantum::hardware
//! quantum::simulator
//! quantum::qec
//! quantum::benchmarking
//! ```
//!
//! Those systems may depend on this namespace.
//!
//! # Rust compatibility
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler
//! enforced for this module.
//!
//! # Scalability contract
//!
//! This file contains no:
//!
//! - fixed qubit count;
//! - fixed register size;
//! - fixed gate count;
//! - fixed operation count;
//! - fixed topology;
//! - fixed hardware architecture;
//! - fixed vendor;
//! - fixed pulse representation.
//!
//! Any concrete limit belongs to the appropriate resource-policy or target
//! subsystem.
//!
//! # API stability
//!
//! The re-exports in this file are intentionally explicit.
//!
//! Do not use wildcard re-exports such as:
//!
//! ```text
//! pub use super::*;
//! ```
//!
//! because that would make the public API depend implicitly on unrelated IR
//! modules and would create accidental API coupling.
//!
//! Explicit exports also make API review and semver management substantially
//! safer.
//!
//! # No duplicate ownership
//!
//! The following ownership rules are mandatory:
//!
//! ```text
//! qubit.rs
//!     QubitId
//!     PhysicalQubitId
//!     QubitRef
//!     Qubit
//!     QubitRegister
//!
//! gate.rs
//!     Gate
//!     GateKind
//!     GateParameter
//!     GateError
//!
//! measurement.rs
//!     Measurement
//!     MeasurementBasis
//!     MeasurementMode
//!     ClassicalBitId
//!
//! channel.rs
//!     Channel semantics
//!
//! pulse.rs
//!     Pulse semantics
//!
//! frame.rs
//!     Frame semantics
//!
//! waveform.rs
//!     Waveform semantics
//! ```
//!
//! This facade only exposes those authoritative definitions.
//!
//! # Pulse-level boundary
//!
//! Pulse-level control such as:
//!
//! ```text
//! fn x_gate(q) {
//!     pulse(amp=0.3, dur=20ns)
//! }
//! ```
//!
//! remains a semantic operation.
//!
//! This module does not decide:
//!
//! - which DAC is used;
//! - which physical channel is selected;
//! - sample rate;
//! - carrier frequency;
//! - calibration;
//! - hardware waveform encoding.
//!
//! Those decisions happen after target resolution.
//!
//! # Logical versus physical identity
//!
//! A logical qubit and physical qubit are intentionally different types.
//!
//! A gate in the canonical semantic layer must not silently turn:
//!
//! ```text
//! QubitId
//! ```
//!
//! into:
//!
//! ```text
//! PhysicalQubitId
//! ```
//!
//! Logical-to-physical conversion belongs to mapping/routing.
//!
//! # Future expansion
//!
//! Future quantum-domain modules may include:
//!
//! ```text
//! instruction
//! initialization
//! reset
//! register
//! observable
//! operator
//! state_preparation
//! channel
//! ```
//!
//! They should be added as independent modules only when their ownership and
//! public contracts are established.
//!
//! They must then depend on the canonical foundation rather than redefining
//! existing identities.
//!
//! # Testing contract
//!
//! This facade should have only lightweight namespace/integration tests.
//!
//! Domain behavior belongs to the implementation modules themselves.
//!
//! The critical integration invariant is:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! ==
//! quantum::ir::quantum::qubit::QubitId
//! ```
//!
//! semantically and by Rust type identity, because the latter is a re-export
//! of the former rather than a duplicate definition.
//!
//! -----------------------------------------------------------------------------
//! Implementation
//! -----------------------------------------------------------------------------
//
// Safety contract.
//
// This module must never contain unsafe code.
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

// =============================================================================
// Canonical quantum-domain modules
// =============================================================================
//
// These are aliases to the authoritative implementations in `quantum::ir`.
//
// We intentionally use explicit `pub use` rather than creating:
// 
//     pub mod qubit { ... }
// 
// with duplicated implementations.
//
// This guarantees that:
//
//     quantum::ir::qubit::QubitId
//
// and:
//
//     quantum::ir::quantum::qubit::QubitId
//
// refer to exactly the same Rust type.

/// Canonical logical and physical qubit identity.
///
/// The implementation remains owned by `quantum::ir::qubit`.
pub use super::qubit;

/// Canonical logical/physical qubit types.
///
/// This explicit re-export makes the most important quantum identity types
/// directly available from `quantum::ir::quantum`.
pub use super::qubit::{
    validate_qubits,
    validate_unique_qubits,
    PhysicalQubitId,
    Qubit,
    QubitError,
    QubitId,
    QubitRange,
    QubitRangeError,
    QubitRef,
    QubitRegister,
    QubitState,
};

/// Canonical mathematical/logical gate semantics.
pub use super::gate;

/// Canonical gate API.
pub use super::gate::{
    Gate,
    GateError,
    GateKind,
    GateParameter,
    GateResult,
    OperandCount,
};

/// Canonical measurement semantics.
pub use super::measurement;

/// Canonical measurement API.
pub use super::measurement::{
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

/// Canonical abstract control-channel semantics.
pub use super::channel;

/// Canonical pulse semantics.
pub use super::pulse;

/// Canonical frame semantics.
pub use super::frame;

/// Canonical waveform semantics.
pub use super::waveform;

// =============================================================================
// Integration tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_qubit_type_is_reused() {
        let logical = QubitId::new(7);

        let canonical: super::super::qubit::QubitId = logical;

        assert_eq!(canonical, logical);
        assert_eq!(canonical.index(), 7);
    }

    #[test]
    fn canonical_physical_qubit_type_is_reused() {
        let physical = PhysicalQubitId::new(11);

        let canonical: super::super::qubit::PhysicalQubitId = physical;

        assert_eq!(canonical, physical);
        assert_eq!(canonical.index(), 11);
    }

    #[test]
    fn logical_and_physical_qubits_remain_distinct() {
        let logical = QubitId::new(3);
        let physical = PhysicalQubitId::new(3);

        let logical_ref = QubitRef::Logical(logical);
        let physical_ref = QubitRef::Physical(physical);

        assert!(logical_ref.is_logical());
        assert!(!logical_ref.is_physical());

        assert!(physical_ref.is_physical());
        assert!(!physical_ref.is_logical());
    }

    #[test]
    fn qubit_range_is_lazy_and_non_materializing() {
        let range = QubitRange::new(10, 1_000_000)
            .expect("valid half-open qubit range");

        assert_eq!(range.start(), 10);
        assert_eq!(range.end(), 1_000_000);
        assert_eq!(range.len(), 999_990);
    }

    #[test]
    fn qubit_identity_does_not_encode_machine_capacity() {
        let small = QubitId::new(0);
        let large = QubitId::new(usize::MAX);

        assert_ne!(small, large);
        assert_eq!(large.index(), usize::MAX);
    }

    #[test]
    fn qubit_state_is_ir_bookkeeping_only() {
        assert!(QubitState::Available.is_usable());
        assert!(QubitState::Reset.is_usable());
        assert!(QubitState::Measured.is_usable());
        assert!(!QubitState::Disabled.is_usable());
    }

    #[test]
    fn canonical_gate_type_is_reused() {
        fn accepts_canonical_gate(_: &super::super::gate::Gate) {}

        let _ = accepts_canonical_gate;
    }

    #[test]
    fn canonical_measurement_type_is_reused() {
        fn accepts_canonical_measurement(
            _: &super::super::measurement::Measurement,
        ) {
        }

        let _ = accepts_canonical_measurement;
    }
}