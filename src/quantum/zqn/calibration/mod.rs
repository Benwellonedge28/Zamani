//! # Zamani Quantum Noise — Calibration
//!
//! Production calibration subsystem for ZQN.
//!
//! ## Ownership
//!
//! This module owns the **calibration namespace and public composition boundary**
//! of the ZQN calibration subsystem.
//!
//! The individual child modules own the actual calibration domains:
//!
//! - [`snapshot`] owns immutable calibration snapshots and their validity.
//! - [`parameter`] owns generic calibrated parameters, uncertainty and units.
//! - [`device`] owns device/resource-scoped calibration.
//! - [`gate`] owns gate-operation calibration.
//! - [`readout`] owns readout calibration.
//! - [`measurement`] owns measurement-specific calibration.
//! - [`drift`] owns deterministic analytic temporal evolution of calibration
//!   parameters.
//! - [`interpolation`] owns interpolation of discrete calibration observations.
//! - [`validation`] owns cross-object and cross-snapshot validation policies.
//!
//! This file deliberately contains no calibration algorithms.
//!
//! ## Does not own
//!
//! This module does **not** own:
//!
//! - the canonical quantum IR;
//! - source-language parsing;
//! - quantum operation semantics;
//! - quantum channels;
//! - noise-model semantics;
//! - routing;
//! - scheduling;
//! - QEC decoding or correction;
//! - simulator state evolution;
//! - QPU transport;
//! - vendor APIs;
//! - credentials;
//! - benchmark methodology;
//! - hardware-specific protocol implementations.
//!
//! The canonical quantum resource identifiers remain owned by
//! `crate::quantum::ir::qubit`.
//!
//! In particular, calibration code must use:
//!
//! - [`crate::quantum::ir::qubit::QubitId`] for logical qubit identity;
//! - [`crate::quantum::ir::qubit::PhysicalQubitId`] for physical qubit identity;
//!
//! wherever those identities are semantically applicable.
//!
//! ZQN must not introduce a competing `zqn::QubitId`.
//!
//! ## Architectural position
//!
//! ```text
//!                    Zamani source
//!                         |
//!                         v
//!                  quantum::frontend
//!                         |
//!                         v
//!                   quantum::ir
//!                         |
//!                         v
//!                        ZQN
//!                         |
//!              +----------+----------+
//!              |                     |
//!              v                     v
//!         calibration          characterization
//!              |
//!              v
//!       CalibrationSnapshot
//!              |
//!       +------+-------+
//!       |              |
//!       v              v
//!   noise model     execution
//!       |              |
//!       +------+-------+
//!              |
//!              v
//!        observations
//!              |
//!              v
//!       characterization
//!              |
//!              v
//!        new calibration
//! ```
//!
//! Calibration is therefore both:
//!
//! 1. an input to physical/noise-aware execution; and
//! 2. an output of characterization.
//!
//! ## Dependency direction
//!
//! The intended dependency direction is:
//!
//! ```text
//! core
//!   ^
//!   |
//! calibration
//!   |
//!   +--> noise
//!   +--> simulation
//!   +--> propagation
//!   +--> target
//!   +--> integration
//! ```
//!
//! Calibration must not depend on routing, scheduling, QEC implementations,
//! simulators or hardware-provider implementations.
//!
//! ## Stability contract
//!
//! This file is the stable namespace boundary for calibration.
//!
//! Child modules may evolve internally without changing this file unless a
//! genuinely new public calibration domain is introduced.
//!
//! New calibration implementations should therefore normally be added as a
//! new child module rather than by inserting implementation code into this
//! file.
//!
//! ## Scalability
//!
//! No machine-size limit is encoded here.
//!
//! The module does not assume:
//!
//! - a fixed number of qubits;
//! - a fixed number of physical resources;
//! - a fixed gate set;
//! - a fixed topology;
//! - a fixed calibration parameter count;
//! - a fixed device size;
//! - a fixed calibration history length.
//!
//! Resource limits belong to explicit validation/runtime policies and are not
//! semantic properties of the calibration model.
//!
//! This allows the same Zamani program and calibration semantics to operate
//! from a single resource through very large and distributed quantum systems,
//! subject only to available computational, memory, communication and hardware
//! resources.
//!
//! ## Determinism
//!
//! Calibration data itself is declarative state and must not depend on hidden
//! global mutable state or hidden randomness.
//!
//! Any stochastic characterization process that creates calibration data must
//! receive its deterministic execution context explicitly from the caller.
//!
//! Calibration lookup, validation and serialization must be deterministic for
//! identical inputs.
//!
//! ## Resource safety
//!
//! This module does not impose artificial machine-size limits.
//!
//! Potentially expensive operations in child modules must use explicit,
//! caller-configurable resource policies where appropriate.
//!
//! In particular, untrusted calibration data must not be allowed to trigger
//! uncontrolled allocation, pathological interpolation, enormous histories,
//! or unbounded validation work.
//!
//! ## Numerical safety
//!
//! Calibration values are required to preserve the numerical contracts defined
//! by [`parameter`] and [`validation`].
//!
//! NaN and infinite values must not be silently accepted.
//!
//! Invalid values must produce a [`crate::quantum::zqn::core::errors::ZqnError`]
//! rather than being silently coerced.
//!
//! ## Serialization
//!
//! Serialization contracts belong to `crate::quantum::zqn::io`.
//!
//! This module exposes stable semantic types but does not make Rust's internal
//! representation the external serialization format.
//!
//! Calibration snapshots must remain versioned, validated and provenance-aware
//! when serialized.
//!
//! ## Integration contracts
//!
//! ### Canonical quantum IR
//!
//! Calibration resources that refer to qubits must use the canonical types from
//! `crate::quantum::ir::qubit`.
//!
//! ```text
//! quantum::ir::qubit
//!        |
//!        v
//! CalibrationResource
//!        |
//!        v
//! CalibrationSnapshot
//! ```
//!
//! ### ZQN noise
//!
//! ```text
//! CalibrationSnapshot
//!        |
//!        v
//! ZQN NoiseModel
//!        |
//!        v
//! NoiseApplication
//! ```
//!
//! Calibration describes the physical state of a target; it does not itself
//! become the noise model.
//!
//! ### Characterization
//!
//! ```text
//! characterization::Observation
//!          |
//!          v
//!      estimator
//!          |
//!          v
//! CalibrationParameter
//!          |
//!          v
//! CalibrationSnapshot
//! ```
//!
//! ### Routing
//!
//! Routing consumes calibration-derived costs and quality information through
//! ZQN integration interfaces. Calibration must not directly depend on the
//! router.
//!
//! ### Scheduling
//!
//! Scheduling consumes calibration timing, validity and noise information.
//! Calibration remains independent of the scheduler.
//!
//! ### QEC
//!
//! QEC may consume calibration-derived physical error information through the
//! ZQN/QEC integration layer. Calibration must not depend on QEC decoding.
//!
//! ### Hardware
//!
//! Hardware adapters produce abstract calibration information. Vendor-specific
//! APIs belong outside this module.
//!
//! ```text
//! hardware adapter
//!       |
//!       v
//! abstract calibration data
//!       |
//!       v
//! ZQN calibration
//! ```
//!
//! ## Production invariants
//!
//! The calibration subsystem must preserve the following invariants:
//!
//! 1. Every calibration object has an explicit semantic identity.
//! 2. Calibration validity is explicit.
//! 3. Calibration provenance is preserved.
//! 4. Calibration uncertainty is not silently discarded.
//! 5. Resource scope is explicit.
//! 6. Calibration snapshots are immutable once published.
//! 7. Temporal validity is explicit.
//! 8. Analytic drift and sampled-data interpolation remain separate concerns.
//! 9. Validation is explicit and deterministic.
//! 10. No vendor identity is required by the semantic model.
//! 11. No fixed machine size is encoded in calibration.
//! 12. Canonical quantum resource identifiers come from `quantum::ir`.
//! 13. Approximation is explicit rather than silently introduced.
//! 14. Invalid numerical values are rejected.
//! 15. Serialization versions are explicit.
//!
//! ## Public API policy
//!
//! The child modules are publicly exposed because they represent independent
//! calibration domains. Consumers should nevertheless prefer the semantic
//! types re-exported by this module rather than reaching into implementation
//! details whenever possible.
//!
//! ## Adding a new calibration domain
//!
//! A future calibration domain should follow this process:
//!
//! 1. Create a new child module.
//! 2. Give it a complete ownership contract.
//! 3. Define its independent public API.
//! 4. Define validation and numerical invariants.
//! 5. Define its serialization contract.
//! 6. Define its scaling/resource contract.
//! 7. Define its deterministic behavior.
//! 8. Add its tests.
//! 9. Add `pub mod <domain>;` here.
//! 10. Re-export only stable semantic types.
//!
//! Existing modules should not need to be rewritten merely because another
//! calibration domain was added.
//!
//! ## Rust safety
//!
//! This module and the ZQN calibration subsystem are designed for safe Rust.
//!
//! No `unsafe` code is permitted.
//!
//! ```text
//! Rust 2021
//! Rust 1.97 / 1.97.1
//! no unsafe
//! ```
//!
//! ## Testing
//!
//! Calibration tests are organized below this namespace in the repository's
//! ZQN test hierarchy.
//!
//! Child modules should provide their own unit/property tests. Cross-module
//! behavior should be tested through the calibration integration tests rather
//! than by making this namespace module depend on implementation internals.

// -----------------------------------------------------------------------------
// Public child modules
// -----------------------------------------------------------------------------

/// Immutable calibration snapshots and their validity/provenance state.
pub mod snapshot;

/// Generic calibrated parameters, units, uncertainty and validity.
pub mod parameter;

/// Device/resource-scoped calibration.
pub mod device;

/// Gate-operation calibration.
pub mod gate;

/// Readout-specific calibration.
pub mod readout;

/// Measurement-specific calibration.
pub mod measurement;

/// Deterministic analytic calibration drift models.
pub mod drift;

/// Interpolation of discrete calibration observations.
pub mod interpolation;

/// Cross-object and cross-snapshot calibration validation.
pub mod validation;

// -----------------------------------------------------------------------------
// Stable public re-exports
// -----------------------------------------------------------------------------

// Snapshot API

pub use self::snapshot::{
    CalibrationResource,
    CalibrationSnapshot,
    CalibrationSnapshotStatus,
    CalibrationTime,
    CalibrationValidity,
    SnapshotValidationLimits,
};

// Parameter API

pub use self::parameter::CalibrationParameter;

// Validation API

pub use self::validation::{
    CalibrationValidationLimits,
    CalibrationValidationPolicy,
    CalibrationValidationReport,
    CalibrationValidator,
    LineagePolicy,
    OverlapPolicy,
    SnapshotStatusPolicy,
};

// -----------------------------------------------------------------------------
// Module-level API contract
// -----------------------------------------------------------------------------

/// Marker type identifying the ZQN calibration subsystem.
///
/// This is intentionally a zero-sized semantic marker rather than a runtime
/// singleton. It provides a stable type-level anchor for generic APIs without
/// introducing global mutable state.
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct CalibrationDomain;

/// Returns the stable semantic name of the calibration subsystem.
///
/// This function is deliberately independent of runtime state and therefore
/// safe to use in diagnostics, logging and schema metadata.
#[inline]
pub const fn domain_name() -> &'static str {
    "zqn.calibration"
}

/// Returns the semantic role of the calibration subsystem.
///
/// This is descriptive metadata, not a version number and must not be used as
/// a compatibility check.
#[inline]
pub const fn domain_description() -> &'static str {
    "Zamani Quantum Noise calibration and physical-state characterization"
}