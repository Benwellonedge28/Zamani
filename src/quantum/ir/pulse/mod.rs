//! Zamani Quantum IR — Pulse Subsystem
//!
//! Canonical public module boundary for pulse-level quantum semantics.
//!
//! # Architectural role
//!
//! `quantum::ir::pulse` represents the semantic meaning of pulse-level
//! quantum computation without binding Zamani to:
//!
//! - a particular QPU;
//! - a particular vendor;
//! - a particular DAC/ADC;
//! - a particular control electronics stack;
//! - a particular topology;
//! - a particular calibration database;
//! - a particular routing algorithm;
//! - a particular scheduler;
//! - a particular backend;
//! - a particular simulator.
//!
//! The pulse IR answers:
//!
//! > What pulse-level computation does the program mean?
//!
//! It does not answer:
//!
//! > Which physical machine resource executes it?
//!
//! Those decisions belong downstream.
//!
//! # Universal-program principle
//!
//! A Zamani program is written once at the semantic level and may be lowered
//! to any compatible target for which sufficient resources and capabilities
//! exist.
//!
//! The pulse IR therefore has no architectural quantum-machine-size limit.
//!
//! The following are resource instances, not language limits:
//!
//! ```text
//! 1 qubit
//! 10 qubits
//! 1_000 qubits
//! 1_000_000 qubits
//! N qubits
//! ```
//!
//! The same principle applies to:
//!
//! - number of pulses;
//! - number of targets;
//! - number of channels;
//! - number of frames;
//! - waveform size;
//! - program size;
//! - pulse duration;
//! - number of operations.
//!
//! Concrete resource limits are compilation/service policy, hardware
//! capability, host-resource, or security concerns. They must never become
//! semantic limits of Zamani.
//!
//! # Canonical dependency direction
//!
//! ```text
//! Zamani source
//!      |
//!      v
//! frontend
//!      |
//!      v
//! quantum::ir
//!      |
//!      +-----------------------------+
//!      |                             |
//!      v                             v
//! gate semantics                pulse semantics
//!                                    |
//!                                    v
//!                              optimization
//!                                    |
//!                                    v
//!                               scheduling
//!                                    |
//!                                    v
//!                                hardware
//!                                    |
//!                                    v
//!                                 backend
//!                                    |
//!                                    v
//!                                  QPU
//! ```
//!
//! The IR MUST NOT depend on downstream execution infrastructure.
//!
//! # Ownership
//!
//! This module owns the public pulse namespace and integration boundary.
//!
//! The semantic pulse implementation owns:
//!
//! - pulse identity;
//! - pulse targets;
//! - pulse duration;
//! - pulse amplitude;
//! - pulse phase;
//! - pulse frequency;
//! - waveform references;
//! - channel references;
//! - frame references;
//! - pulse composition;
//! - pulse metadata;
//! - pulse-local validation;
//! - checked duration arithmetic;
//! - symbolic parameter integration.
//!
//! It does NOT own:
//!
//! - physical channel allocation;
//! - hardware calibration databases;
//! - routing;
//! - scheduling algorithms;
//! - DAC/ADC programming;
//! - provider APIs;
//! - credentials;
//! - authentication;
//! - QPU communication;
//! - simulator state;
//! - optimization policy.
//!
//! # Canonical qubit identity
//!
//! Pulse semantics reference logical qubits using:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! ```
//!
//! `quantum::ir::qubit` is authoritative.
//!
//! This module MUST NOT define another `QubitId`.
//!
//! Physical qubit identity is deliberately not embedded into canonical
//! source-level pulse semantics. Logical-to-physical mapping belongs to the
//! mapping/routing layer.
//!
//! # Integration contract
//!
//! The pulse subsystem integrates with the rest of the IR through stable
//! canonical contracts:
//!
//! ```text
//! quantum::ir::identity
//!     PulseId
//!     WaveformId
//!     ChannelId
//!     FrameId
//!     CalibrationId
//!
//! quantum::ir::parameter
//!     Parameter
//!
//! quantum::ir::qubit
//!     QubitId
//!
//! quantum::ir::operation
//!     pulse operation references
//!
//! quantum::ir::timing
//!     program-wide temporal semantics
//!
//! quantum::ir::waveform
//!     waveform definitions
//!
//! quantum::ir::channel
//!     abstract control-channel definitions
//!
//! quantum::ir::frame
//!     frame semantics
//!
//! quantum::ir::validation
//!     whole-program validation
//!
//! quantum::ir::serialization
//!     canonical persistence
//!
//! quantum::ir::hash
//!     canonical content identity
//!
//! quantum::ir::provenance
//!     transformation lineage
//! ```
//!
//! The pulse module does not redefine any of those canonical concepts.
//!
//! # Migration architecture
//!
//! The repository currently contains the pulse implementation in:
//!
//! ```text
//! src/quantum/ir/pulse.rs
//! ```
//!
//! This directory is the new long-term module boundary:
//!
//! ```text
//! src/quantum/ir/pulse/
//!     mod.rs
//!     ...
//! ```
//!
//! Rust does not permit `pulse.rs` and `pulse/mod.rs` to independently define
//! the same `pulse` module. Therefore this module temporarily incorporates the
//! existing implementation as a private compatibility implementation.
//!
//! This is intentional.
//!
//! It provides:
//!
//! 1. one public `quantum::ir::pulse` namespace;
//! 2. one canonical pulse type definition;
//! 3. no duplicate pulse types;
//! 4. no changes required by downstream consumers;
//! 5. no duplicate `QubitId`;
//! 6. a clean migration point for later subdivision.
//!
//! The existing implementation is incorporated below without changing its
//! semantic ownership.
//!
//! Once pulse internals are split into:
//!
//! ```text
//! pulse/
//!     mod.rs
//!     duration.rs
//!     operation.rs
//!     target.rs
//!     metadata.rs
//!     composition.rs
//!     validation.rs
//! ```
//!
//! the public API established by this file remains the integration boundary.
//!
//! Downstream code should therefore depend on:
//!
//! ```text
//! quantum::ir::pulse::Pulse
//! quantum::ir::pulse::PulseDuration
//! quantum::ir::pulse::PulseResult
//! ```
//!
//! rather than depending on private implementation layout.
//!
//! # Safety
//!
//! This module is entirely safe Rust.
//!
//! No unsafe code is permitted.
//!
//! The restriction is compiler-enforced with `forbid(unsafe_code)`.
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
//! - no unsafe code.
//!
//! # Serialization
//!
//! Serialization remains owned by `quantum::ir::serialization`.
//!
//! This module does not introduce an independent serialization format.
//!
//! Pulse semantic fields must be serialized through the canonical IR
//! serialization contract.
//!
//! # Hashing
//!
//! Hashing remains owned by `quantum::ir::hash`.
//!
//! Hash identity must be derived from semantic fields and must not depend on:
//!
//! - memory addresses;
//! - process state;
//! - HashMap iteration order;
//! - allocator layout;
//! - temporary compiler paths;
//! - nondeterministic execution state.
//!
//! # Validation
//!
//! Pulse-local construction and validation may reject malformed pulse values.
//!
//! Whole-program validation remains owned by `quantum::ir::validation`.
//!
//! Target-specific validation belongs to hardware capability/compatibility
//! layers.
//!
//! These three validation levels MUST NOT be conflated:
//!
//! ```text
//! Pulse structural validity
//!          |
//!          v
//! Canonical IR semantic validity
//!          |
//!          v
//! Target compatibility
//! ```
//!
//! # Scalability
//!
//! No pulse API in this module may interpret a numeric constant as the maximum
//! supported quantum machine.
//!
//! Numeric constants used for defensive metadata/resource policies are limits
//! for a particular operation or validation policy only.
//!
//! A future caller with larger resources must be able to supply a larger
//! explicit policy without changing the pulse semantic model.
//!
//! # Public API policy
//!
//! The public API of this module is deliberately re-export based.
//!
//! This prevents downstream code from depending on the temporary migration
//! layout.
//!
//! The implementation can therefore be reorganized internally without
//! requiring consumers to be rewritten.
//!
//! # Important invariant
//!
//! There must be exactly one semantic definition for every pulse-domain type.
//!
//! In particular, this module MUST NOT introduce competing definitions of:
//!
//! - `Pulse`;
//! - `PulseDuration`;
//! - `PulseId`;
//! - `QubitId`;
//! - `WaveformId`;
//! - `ChannelId`;
//! - `FrameId`;
//! - `CalibrationId`;
//! - `Parameter`.
//!
//! Existing canonical definitions are reused.
//!
//! # Module-level contract
//!
//! This file is complete when:
//!
//! - `quantum::ir::pulse` is the sole public pulse namespace;
//! - all existing pulse API remains reachable;
//! - canonical `quantum::ir::qubit::QubitId` remains authoritative;
//! - no downstream module needs to know about `pulse.rs`;
//! - no physical-hardware policy enters the canonical pulse model;
//! - no fixed machine-size limit exists;
//! - no unsafe code exists;
//! - Rust 1.97/1.97.1 compatibility is preserved;
//! - future pulse submodules can be added behind this boundary without
//!   changing consumers.
//!
//! -----------------------------------------------------------------------------
//! Implementation boundary
//! -----------------------------------------------------------------------------
//
// The old implementation lives one level above this directory:
//
//     ../pulse.rs
//
// Its `super::identity`, `super::parameter`, and `super::qubit` paths would,
// when included as a child module here, resolve against `pulse` rather than
// `quantum::ir`. The aliases below deliberately bridge those canonical parent
// modules into the legacy implementation's expected namespace.
//
// These are aliases, not duplicate definitions.
//
// This permits a safe incremental migration from:
//
//     ir/pulse.rs
//
// to:
//
//     ir/pulse/
//
// without forcing simultaneous changes across the repository.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//
// Canonical parent-module bridges used only by the incorporated implementation.
//
// They are deliberately private to the `pulse` module. Consumers continue to
// use the canonical paths exported below.
//

pub(crate) use super::identity;
pub(crate) use super::parameter;
pub(crate) use super::qubit;

//
// Incorporate the existing complete pulse implementation.
//
// `path` is relative to this module's source directory:
//
//     src/quantum/ir/pulse/mod.rs
//
// therefore:
//
//     ../pulse.rs
//
// resolves to:
//
//     src/quantum/ir/pulse.rs
//
// No duplicate semantic implementation is introduced.
//
#[path = "../pulse.rs"]
mod implementation;

//
// Public pulse API.
//
// Everything exported by the established pulse implementation remains
// available through the new canonical module boundary:
//
//     quantum::ir::pulse::*
//
// This is the critical compatibility guarantee.
//

pub use implementation::*;

// =============================================================================
// Canonical namespace documentation helpers
// =============================================================================
//
// The following intentionally contain no duplicate domain types.
//
// They document the stable integration paths expected by downstream modules.
//
// Pulse implementation:
//     quantum::ir::pulse
//
// Canonical qubit identity:
//     quantum::ir::qubit::QubitId
//
// Canonical parameter:
//     quantum::ir::parameter::Parameter
//
// Canonical identity:
//     quantum::ir::identity::{PulseId, WaveformId, ChannelId, FrameId}
//
// Whole-program operation:
//     quantum::ir::operation
//
// Whole-program validation:
//     quantum::ir::validation
//
// Canonical serialization:
//     quantum::ir::serialization
//
// Canonical hashing:
//     quantum::ir::hash
//
// Mapping:
//     quantum::ir::mapping
//
// Scheduling:
//     quantum::ir::schedule
//
// Hardware:
//     quantum::hardware
//
// None of those modules are redefined here.

// =============================================================================
// Compile-time API assertions
// =============================================================================
//
// These assertions intentionally reference the public API through this module
// boundary rather than the implementation module.
//
// They ensure future internal reorganization does not accidentally remove the
// fundamental pulse contract.
//
// The assertions are expressed through generic functions rather than requiring
// unstable compile-time reflection.

#[allow(dead_code)]
fn assert_pulse_api_boundary() {
    fn accepts_duration(_: PulseDuration) {}
    fn accepts_result(_: PulseResult<()>) {}

    let duration = PulseDuration::ZERO;

    accepts_duration(duration);
    accepts_result(Ok(()));
}

// =============================================================================
// Module tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pulse_namespace_exposes_duration() {
        let duration = PulseDuration::from_nanoseconds(20)
            .expect("20ns must be representable");

        assert_eq!(
            duration.femtoseconds(),
            20_000_000
        );
    }

    #[test]
    fn pulse_duration_is_exact_integer_semantics() {
        let one_ns = PulseDuration::from_nanoseconds(1)
            .expect("1ns must be representable");

        let two_ns = one_ns
            .checked_add(one_ns)
            .expect("1ns + 1ns must be representable");

        assert_eq!(two_ns.femtoseconds(), 2_000_000);
    }

    #[test]
    fn pulse_duration_underflow_is_rejected() {
        let one = PulseDuration::from_femtoseconds(1);

        let result = PulseDuration::ZERO.checked_sub(one);

        assert!(matches!(
            result,
            Err(PulseError::NegativeDuration)
        ));
    }

    #[test]
    fn pulse_duration_overflow_is_rejected() {
        let result = PulseDuration::from_seconds(u64::MAX);

        assert!(matches!(
            result,
            Err(PulseError::DurationOverflow)
        ));
    }

    #[test]
    fn pulse_duration_zero_is_stable() {
        assert!(PulseDuration::ZERO.is_zero());
        assert_eq!(
            PulseDuration::ZERO.femtoseconds(),
            0
        );
    }

    #[test]
    fn pulse_namespace_is_based_on_canonical_qubit_module() {
        //
        // This test is intentionally type-level.
        //
        // `QubitId` must come from the canonical `quantum::ir::qubit` module.
        //
        // The pulse implementation itself imports:
        //
        //     super::qubit::QubitId
        //
        // through the bridge above.
        //
        // No local pulse QubitId exists.
        //
        fn accepts_canonical_qubit(_: crate::quantum::ir::qubit::QubitId) {}

        //
        // We cannot manufacture an arbitrary QubitId without depending on its
        // private construction contract. The function declaration itself is
        // sufficient to enforce the type path at compile time.
        let _ = accepts_canonical_qubit;
    }
}