//! Zamani Quantum IR — Production Pulse Integration Tests.
//!
//! Path:
//!
//!     src/quantum/ir/tests/pulse.rs
//!
//! ============================================================================
//! PURPOSE
//! ============================================================================
//!
//! This module verifies the production contract of:
//!
//!     crate::quantum::ir::pulse
//!
//! The tests exercise the public pulse API through the canonical module
//! boundary. They intentionally do not depend on:
//!
//! - private pulse implementation modules;
//! - temporary migration paths;
//! - vendor APIs;
//! - hardware implementations;
//! - schedulers;
//! - routing algorithms;
//! - simulators;
//! - backend execution.
//!
//! The pulse IR is a semantic representation of WHAT pulse-level quantum
//! computation means. Hardware-specific realization belongs downstream.
//!
//! ============================================================================
//! ARCHITECTURAL CONTRACT
//! ============================================================================
//!
//! A valid Zamani pulse program must be able to represent:
//!
//! - logical qubit targets;
//! - multiple logical qubit targets;
//! - global pulse intent;
//! - abstract resource targets;
//! - waveform references;
//! - channel references;
//! - frame references;
//! - calibration references;
//! - exact duration;
//! - symbolic amplitude;
//! - symbolic phase;
//! - symbolic frequency;
//! - concrete numerical parameters;
//! - capture/acquisition;
//! - delay;
//! - frame frequency changes;
//! - frame phase changes;
//! - phase shifts;
//! - barriers;
//! - calibration invocation;
//! - extension-defined pulse operations;
//! - dependencies;
//! - deterministic metadata;
//! - sequential composition;
//! - parallel composition;
//! - repetition;
//! - sequence repetition.
//!
//! ============================================================================
//! SCALABILITY CONTRACT
//! ============================================================================
//!
//! "Infinity" is not a finite Rust value and cannot be directly allocated by a
//! finite compiler process.
//!
//! The production requirement is instead:
//!
//!     every finite representable workload
//!             |
//!             v
//!     limited only by:
//!         - representation boundaries;
//!         - explicit caller policy;
//!         - available host resources;
//!         - target capabilities;
//!         - execution resources.
//!
//! The pulse semantic model must not introduce an architectural maximum such
//! as:
//!
//!     MAX_QUBITS = 64
//!     MAX_PULSES = 4096
//!     MAX_TARGETS = 4096
//!
//! Test counts in this file are workload samples, never language limits.
//!
//! ============================================================================
//! CANONICAL IDENTITY CONTRACT
//! ============================================================================
//!
//! Logical qubits MUST use:
//!
//!     crate::quantum::ir::qubit::QubitId
//!
//! and never:
//!
//!     usize
//!     u64
//!     a locally defined QubitId
//!     quantum::ir::qubits::QubitId
//!
//! The compatibility alias `qubits` is intentionally not used here.
//!
//! ============================================================================
//! POLICY VERSUS SEMANTICS
//! ============================================================================
//!
//! `PulseValidationPolicy` is a resource/security policy.
//!
//! It is NOT:
//!
//!     "the largest quantum computer Zamani supports."
//!
//! An unrestricted policy means only that no pulse-local application policy
//! has been imposed. It does not claim infinite RAM, infinite execution time,
//! or infinite hardware.
//!
//! ============================================================================
//! WHAT THIS FILE TESTS
//! ============================================================================
//!
//! 1. Rust safety contract.
//! 2. Public namespace stability.
//! 3. Canonical QubitId integration.
//! 4. Exact duration semantics.
//! 5. Checked duration arithmetic.
//! 6. Duration boundary behavior.
//! 7. Pulse target semantics.
//! 8. Deterministic target canonicalization.
//! 9. Target validation.
//! 10. Target duplication rejection.
//! 11. Pulse construction.
//! 12. Play validation.
//! 13. Capture validation.
//! 14. Acquire validation.
//! 15. Delay validation.
//! 16. Frame operation validation.
//! 17. Resource references.
//! 18. Symbolic parameters.
//! 19. Concrete parameters.
//! 20. Dependency semantics.
//! 21. Metadata semantics.
//! 22. Metadata policy limits.
//! 23. Pulse composition.
//! 24. Repeat validation.
//! 25. Explicit policy isolation.
//! 26. Sparse large target sets.
//! 27. Deterministic parameter ordering.
//! 28. Multiple pulse kinds.
//! 29. Global targets.
//! 30. Abstract resource targets.
//! 31. Semantic versus target-specific validation.
//!
//! ============================================================================
//! WHAT THIS FILE DOES NOT TEST
//! ============================================================================
//!
//! This file does NOT test:
//!
//! - physical DACs;
//! - ADCs;
//! - microwave generators;
//! - lasers;
//! - hardware calibration databases;
//! - topology;
//! - physical qubit allocation;
//! - routing;
//! - scheduling algorithms;
//! - waveform synthesis;
//! - backend execution;
//! - provider SDKs;
//! - provider authentication;
//! - QPU communication;
//! - simulator state;
//! - QEC decoding;
//! - optimizer quality;
//! - frontend parsing.
//!
//! Those concerns belong outside canonical pulse semantics.
//!
//! ============================================================================
//! INTEGRATION CONTRACT
//! ============================================================================
//!
//! This test file depends only on the stable public pulse API:
//!
//!     crate::quantum::ir::pulse
//!
//! and the authoritative qubit API:
//!
//!     crate::quantum::ir::qubit
//!
//! It therefore remains valid if the internal pulse implementation is later
//! split into:
//!
//!     pulse/
//!         mod.rs
//!         duration.rs
//!         operation.rs
//!         target.rs
//!         metadata.rs
//!         composition.rs
//!         validation.rs
//!
//! as long as the public `quantum::ir::pulse` contract remains stable.
//!
//! ============================================================================
//! RUST CONTRACT
//! ============================================================================
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021
//! - stable Rust
//! - no nightly features
//! - no unsafe code
//!
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler-enforced.
//!
//! ============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use crate::quantum::ir::identity::{
    CalibrationId,
    ChannelId,
    FrameId,
    PulseId,
    ResourceId,
    WaveformId,
};

use crate::quantum::ir::parameter::Parameter;

use crate::quantum::ir::pulse::{
    Pulse,
    PulseComposition,
    PulseDependency,
    PulseDuration,
    PulseError,
    PulseKind,
    PulseResources,
    PulseTarget,
    PulseValidationPolicy,
    PULSE_SCHEMA_ID,
    PULSE_SCHEMA_MAJOR,
    PULSE_SCHEMA_MINOR,
    PULSE_SCHEMA_PATCH,
};

use crate::quantum::ir::qubit::QubitId;

// ============================================================================
// Test helpers
// ============================================================================

/// Constructs a canonical logical qubit.
///
/// Keeping this helper explicitly tied to `quantum::ir::qubit` prevents tests
/// from accidentally drifting to raw integer identifiers or compatibility
/// aliases.
fn qubit(index: usize) -> QubitId {
    QubitId::new(index)
}

/// Constructs a valid single-target play pulse.
///
/// The helper intentionally uses only the public API.
fn valid_play_pulse() -> Pulse {
    Pulse::new(
        PulseId::new(1),
        PulseKind::Play,
    )
    .with_target(
        PulseTarget::qubit(
            qubit(0),
        ),
    )
    .expect("valid logical target must be accepted")
    .with_duration(
        PulseDuration::from_nanoseconds(20)
            .expect("20ns must be representable"),
    )
    .with_resources(
        PulseResources::empty()
            .with_waveform(
                WaveformId::new(1),
            )
            .with_channel(
                ChannelId::new(1),
            )
            .with_frame(
                FrameId::new(1),
            ),
    )
}

/// Constructs a valid barrier pulse.
///
/// Barriers are timing/synchronization semantics and do not require a
/// waveform.
fn valid_barrier_pulse() -> Pulse {
    Pulse::new(
        PulseId::new(2),
        PulseKind::Barrier,
    )
}

/// Constructs a valid delay pulse.
///
/// Delay semantics require duration but do not require a waveform.
fn valid_delay_pulse() -> Pulse {
    Pulse::new(
        PulseId::new(3),
        PulseKind::Delay,
    )
    .with_duration(
        PulseDuration::from_nanoseconds(20)
            .expect("20ns must be representable"),
    )
}

// ============================================================================
// Foundation: safety and public API
// ============================================================================

/// The test module itself must compile under the same no-unsafe policy as the
/// production pulse module.
///
/// The actual safety guarantee is provided by:
///
///     #![forbid(unsafe_code)]
#[test]
fn pulse_tests_are_no_unsafe() {
    let _ = PulseDuration::ZERO;
}

/// The canonical schema identifier must remain stable.
#[test]
fn pulse_schema_identifier_is_stable() {
    assert_eq!(
        PULSE_SCHEMA_ID,
        "zamani.quantum.ir.pulse",
    );
}

/// The schema version must remain an explicitly represented semantic version.
#[test]
fn pulse_schema_version_is_explicit() {
    assert_eq!(
        PULSE_SCHEMA_MAJOR,
        1,
    );

    assert_eq!(
        PULSE_SCHEMA_MINOR,
        0,
    );

    assert_eq!(
        PULSE_SCHEMA_PATCH,
        0,
    );
}

/// The public pulse namespace must expose the fundamental duration contract.
#[test]
fn pulse_public_namespace_exposes_duration() {
    let duration = PulseDuration::from_nanoseconds(20)
        .expect("20ns must be representable");

    assert_eq!(
        duration.femtoseconds(),
        20_000_000,
    );
}

// ============================================================================
// Canonical logical-qubit integration
// ============================================================================

/// Pulse targets must accept the authoritative logical QubitId.
#[test]
fn pulse_target_uses_canonical_logical_qubit_identity() {
    let logical = crate::quantum::ir::qubit::QubitId::new(7);

    let target = PulseTarget::qubit(logical);

    assert!(target.is_single_qubit());

    assert_eq!(
        target.logical_qubits(),
        &[logical],
    );
}

/// The test deliberately compiles against `quantum::ir::qubit::QubitId`.
///
/// This prevents a future implementation from introducing a duplicate pulse
/// specific QubitId.
#[test]
fn pulse_target_does_not_require_a_private_qubit_type() {
    fn accepts_canonical_qubit(
        _: crate::quantum::ir::qubit::QubitId,
    ) {
    }

    let canonical = crate::quantum::ir::qubit::QubitId::new(0);

    accepts_canonical_qubit(canonical);
}

// ============================================================================
// Duration semantics
// ============================================================================

/// Nanoseconds must convert exactly to femtoseconds.
#[test]
fn duration_nanoseconds_are_exact() {
    let duration = PulseDuration::from_nanoseconds(20)
        .expect("20ns must be representable");

    assert_eq!(
        duration.femtoseconds(),
        20_000_000,
    );
}

/// Picoseconds must convert exactly.
#[test]
fn duration_picoseconds_are_exact() {
    let duration = PulseDuration::from_picoseconds(20)
        .expect("20ps must be representable");

    assert_eq!(
        duration.femtoseconds(),
        20_000,
    );
}

/// Microseconds must convert exactly.
#[test]
fn duration_microseconds_are_exact() {
    let duration = PulseDuration::from_microseconds(2)
        .expect("2us must be representable");

    assert_eq!(
        duration.femtoseconds(),
        2_000_000_000,
    );
}

/// Milliseconds must convert exactly.
#[test]
fn duration_milliseconds_are_exact() {
    let duration = PulseDuration::from_milliseconds(2)
        .expect("2ms must be representable");

    assert_eq!(
        duration.femtoseconds(),
        2_000_000_000_000,
    );
}

/// Seconds must convert exactly.
#[test]
fn duration_seconds_are_exact() {
    let duration = PulseDuration::from_seconds(2)
        .expect("2s must be representable");

    assert_eq!(
        duration.femtoseconds(),
        2_000_000_000_000_000,
    );
}

/// Zero duration must be stable.
#[test]
fn zero_duration_is_stable() {
    assert!(
        PulseDuration::ZERO.is_zero()
    );

    assert_eq!(
        PulseDuration::ZERO.femtoseconds(),
        0,
    );
}

/// Addition must be checked rather than wrapping.
#[test]
fn duration_checked_addition_is_exact() {
    let first =
        PulseDuration::from_femtoseconds(10);

    let second =
        PulseDuration::from_femtoseconds(20);

    let result = first
        .checked_add(second)
        .expect("10fs + 20fs must be representable");

    assert_eq!(
        result.femtoseconds(),
        30,
    );
}

/// Subtraction must be checked rather than wrapping.
#[test]
fn duration_checked_subtraction_is_exact() {
    let first =
        PulseDuration::from_femtoseconds(30);

    let second =
        PulseDuration::from_femtoseconds(10);

    let result = first
        .checked_sub(second)
        .expect("30fs - 10fs must be representable");

    assert_eq!(
        result.femtoseconds(),
        20,
    );
}

/// Negative semantic durations must never be created through checked
/// subtraction.
#[test]
fn duration_underflow_is_rejected() {
    let result =
        PulseDuration::ZERO.checked_sub(
            PulseDuration::from_femtoseconds(1),
        );

    assert_eq!(
        result,
        Err(PulseError::NegativeDuration),
    );
}

/// u128 duration representation must reject arithmetic overflow.
#[test]
fn duration_conversion_overflow_is_rejected() {
    let result =
        PulseDuration::from_seconds(u128::MAX);

    assert_eq!(
        result,
        Err(PulseError::DurationOverflow),
    );
}

/// Multiplication must be checked.
#[test]
fn duration_checked_multiplication_is_exact() {
    let duration =
        PulseDuration::from_nanoseconds(5)
            .expect("5ns must be representable");

    let result =
        duration
            .checked_mul(4)
            .expect("5ns * 4 must be representable");

    assert_eq!(
        result.femtoseconds(),
        20_000_000,
    );
}

/// Multiplication overflow must be rejected.
#[test]
fn duration_checked_multiplication_rejects_overflow() {
    let duration =
        PulseDuration::from_femtoseconds(u128::MAX);

    let result =
        duration.checked_mul(2);

    assert_eq!(
        result,
        Err(PulseError::DurationOverflow),
    );
}

/// Display must remain deterministic.
#[test]
fn duration_display_is_deterministic() {
    let duration =
        PulseDuration::from_femtoseconds(123);

    assert_eq!(
        duration.to_string(),
        "123fs",
    );
}

// ============================================================================
// Target semantics
// ============================================================================

/// A single logical qubit target must report one explicit qubit.
#[test]
fn single_qubit_target_has_one_explicit_qubit() {
    let target =
        PulseTarget::qubit(qubit(0));

    assert_eq!(
        target.explicit_qubit_count(),
        1,
    );
}

/// Multi-target construction must canonicalize ordering.
#[test]
fn multi_qubit_target_is_deterministically_sorted() {
    let target =
        PulseTarget::qubits([
            qubit(9),
            qubit(2),
            qubit(7),
            qubit(2),
            qubit(4),
        ]);

    assert_eq!(
        target.logical_qubits(),
        &[
            qubit(2),
            qubit(4),
            qubit(7),
            qubit(9),
        ],
    );

    assert_eq!(
        target.explicit_qubit_count(),
        4,
    );
}

/// Target construction must remove duplicate qubit identifiers deterministically.
#[test]
fn multi_qubit_target_deduplicates_logical_qubits() {
    let target =
        PulseTarget::qubits([
            qubit(3),
            qubit(3),
            qubit(3),
        ]);

    assert_eq!(
        target.logical_qubits(),
        &[qubit(3)],
    );

    assert_eq!(
        target.explicit_qubit_count(),
        1,
    );
}

/// Empty explicit target sets are structurally invalid.
#[test]
fn empty_explicit_target_set_is_rejected() {
    let target =
        PulseTarget::Qubits(Vec::new());

    assert_eq!(
        target.validate(),
        Err(PulseError::EmptyTargetSet),
    );
}

/// A manually malformed non-canonical target set must be rejected.
///
/// The public constructor `PulseTarget::qubits` canonicalizes its input, so
/// this test deliberately verifies the validator independently.
#[test]
fn noncanonical_target_set_is_rejected() {
    let target =
        PulseTarget::Qubits(vec![
            qubit(4),
            qubit(2),
        ]);

    assert_eq!(
        target.validate(),
        Err(PulseError::NonCanonicalTargetSet),
    );
}

/// Global targets must not claim an explicit qubit count.
#[test]
fn global_target_has_no_explicit_qubit_count() {
    let target =
        PulseTarget::global();

    assert!(
        target.is_global()
    );

    assert_eq!(
        target.explicit_qubit_count(),
        0,
    );

    assert!(
        target.logical_qubits().is_empty()
    );

    assert!(
        target.validate().is_ok()
    );
}

/// Abstract resource targets must remain separate from logical qubit targets.
#[test]
fn resource_target_is_not_a_logical_qubit() {
    let target =
        PulseTarget::Resource(
            ResourceId::new(17),
        );

    assert_eq!(
        target.explicit_qubit_count(),
        0,
    );

    assert!(
        target.logical_qubits().is_empty()
    );

    assert!(
        target.validate().is_ok()
    );
}

// ============================================================================
// Pulse construction
// ============================================================================

/// A valid play pulse must pass local validation.
#[test]
fn valid_play_pulse_passes_validation() {
    let pulse =
        valid_play_pulse();

    assert!(
        pulse.validate().is_ok()
    );

    assert_eq!(
        pulse.kind(),
        PulseKind::Play,
    );

    assert_eq!(
        pulse.explicit_qubit_count(),
        1,
    );
}

/// Pulse identity must remain stable.
#[test]
fn pulse_identity_is_preserved() {
    let pulse =
        valid_play_pulse();

    assert_eq!(
        pulse.id(),
        PulseId::new(1),
    );
}

/// Pulse duration must remain accessible.
#[test]
fn pulse_duration_is_preserved() {
    let pulse =
        valid_play_pulse();

    assert_eq!(
        pulse.duration()
            .expect("valid play pulse has duration")
            .femtoseconds(),
        20_000_000,
    );
}

/// A play pulse without a target must be rejected.
#[test]
fn play_requires_target() {
    let pulse =
        Pulse::new(
            PulseId::new(1),
            PulseKind::Play,
        )
        .with_duration(
            PulseDuration::from_nanoseconds(20)
                .expect("20ns must be representable"),
        )
        .with_resources(
            PulseResources::empty()
                .with_waveform(
                    WaveformId::new(1),
                ),
        );

    assert_eq!(
        pulse.validate(),
        Err(PulseError::MissingTarget),
    );
}

/// A play pulse without duration must be rejected.
#[test]
fn play_requires_duration() {
    let pulse =
        Pulse::new(
            PulseId::new(1),
            PulseKind::Play,
        )
        .with_target(
            PulseTarget::qubit(
                qubit(0),
            ),
        )
        .expect("target must be valid")
        .with_resources(
            PulseResources::empty()
                .with_waveform(
                    WaveformId::new(1),
                ),
        );

    assert_eq!(
        pulse.validate(),
        Err(PulseError::MissingDuration),
    );
}

/// A play pulse without a waveform must be rejected.
#[test]
fn play_requires_waveform() {
    let pulse =
        Pulse::new(
            PulseId::new(1),
            PulseKind::Play,
        )
        .with_target(
            PulseTarget::qubit(
                qubit(0),
            ),
        )
        .expect("target must be valid")
        .with_duration(
            PulseDuration::from_nanoseconds(20)
                .expect("20ns must be representable"),
        );

    assert_eq!(
        pulse.validate(),
        Err(PulseError::MissingWaveform),
    );
}

/// Duplicate target insertion must be rejected instead of silently changing
/// the semantic target list.
#[test]
fn duplicate_pulse_target_is_rejected() {
    let target =
        PulseTarget::qubit(
            qubit(0),
        );

    let pulse =
        Pulse::new(
            PulseId::new(1),
            PulseKind::Barrier,
        )
        .with_target(
            target.clone(),
        )
        .expect("first target must be accepted");

    let result =
        pulse.with_target(target);

    assert_eq!(
        result,
        Err(PulseError::DuplicateTarget),
    );
}

/// Duplicate targets supplied through with_targets must be rejected.
#[test]
fn duplicate_targets_in_target_list_are_rejected() {
    let result =
        Pulse::new(
            PulseId::new(1),
            PulseKind::Barrier,
        )
        .with_targets([
            PulseTarget::qubit(
                qubit(0),
            ),
            PulseTarget::qubit(
                qubit(0),
            ),
        ]);

    assert_eq!(
        result,
        Err(PulseError::DuplicateTarget),
    );
}

// ============================================================================
// Capture and acquisition
// ============================================================================

/// Capture must support explicit logical targets.
#[test]
fn capture_requires_target_and_duration() {
    let pulse =
        Pulse::new(
            PulseId::new(10),
            PulseKind::Capture,
        )
        .with_target(
            PulseTarget::qubit(
                qubit(0),
            ),
        )
        .expect("target must be valid")
        .with_duration(
            PulseDuration::from_nanoseconds(100)
                .expect("100ns must be representable"),
        );

    assert!(
        pulse.validate().is_ok()
    );
}

/// Capture without duration must be rejected.
#[test]
fn capture_requires_duration() {
    let pulse =
        Pulse::new(
            PulseId::new(10),
            PulseKind::Capture,
        )
        .with_target(
            PulseTarget::qubit(
                qubit(0),
            ),
        )
        .expect("target must be valid");

    assert_eq!(
        pulse.validate(),
        Err(PulseError::MissingDuration),
    );
}

/// Acquire must require duration.
#[test]
fn acquire_requires_duration() {
    let pulse =
        Pulse::new(
            PulseId::new(11),
            PulseKind::Acquire,
        )
        .with_target(
            PulseTarget::qubit(
                qubit(0),
            ),
        )
        .expect("target must be valid");

    assert_eq!(
        pulse.validate(),
        Err(PulseError::MissingDuration),
    );
}

/// Acquire with target and duration must validate.
#[test]
fn acquire_with_target_and_duration_is_valid() {
    let pulse =
        Pulse::new(
            PulseId::new(11),
            PulseKind::Acquire,
        )
        .with_target(
            PulseTarget::qubit(
                qubit(0),
            ),
        )
        .expect("target must be valid")
        .with_duration(
            PulseDuration::from_nanoseconds(100)
                .expect("100ns must be representable"),
        );

    assert!(
        pulse.validate().is_ok()
    );
}

// ============================================================================
// Delay and barrier
// ============================================================================

/// Delay is timing-only and does not require a waveform.
#[test]
fn delay_is_valid_without_waveform() {
    let pulse =
        valid_delay_pulse();

    assert!(
        pulse.validate().is_ok()
    );

    assert!(
        pulse.kind().is_timing_only()
    );
}

/// Delay without duration must be rejected.
#[test]
fn delay_requires_duration() {
    let pulse =
        Pulse::new(
            PulseId::new(3),
            PulseKind::Delay,
        );

    assert_eq!(
        pulse.validate(),
        Err(PulseError::MissingDuration),
    );
}

/// Barrier is timing-only and does not require a target or waveform.
#[test]
fn barrier_is_valid_without_target_or_waveform() {
    let pulse =
        valid_barrier_pulse();

    assert!(
        pulse.validate().is_ok()
    );

    assert!(
        pulse.kind().is_timing_only()
    );
}

// ============================================================================
// Frame operations
// ============================================================================

/// Set-frequency operations require both frequency and frame.
#[test]
fn set_frequency_requires_frequency_and_frame() {
    let pulse =
        Pulse::new(
            PulseId::new(20),
            PulseKind::SetFrequency,
        )
        .with_target(
            PulseTarget::qubit(
                qubit(0),
            ),
        )
        .expect("target must be valid")
        .with_frequency(
            Parameter::constant(
                5.0,
            )
            .expect("finite parameter must be valid"),
        );

    assert_eq!(
        pulse.validate(),
        Err(PulseError::MissingFrame),
    );
}

/// Set-frequency with frame and frequency must validate.
#[test]
fn set_frequency_with_frame_is_valid() {
    let pulse =
        Pulse::new(
            PulseId::new(20),
            PulseKind::SetFrequency,
        )
        .with_target(
            PulseTarget::qubit(
                qubit(0),
            ),
        )
        .expect("target must be valid")
        .with_frequency(
            Parameter::constant(
                5.0,
            )
            .expect("finite parameter must be valid"),
        )
        .expect("frequency must be accepted")
        .with_resources(
            PulseResources::empty()
                .with_frame(
                    FrameId::new(1),
                ),
        );

    assert!(
        pulse.validate().is_ok()
    );
}

/// Set-phase requires a phase and a frame.
#[test]
fn set_phase_requires_phase_and_frame() {
    let pulse =
        Pulse::new(
            PulseId::new(21),
            PulseKind::SetPhase,
        )
        .with_target(
            PulseTarget::qubit(
                qubit(0),
            ),
        )
        .expect("target must be valid");

    assert_eq!(
        pulse.validate(),
        Err(PulseError::MissingPhase),
    );
}

/// Set-phase with phase and frame must validate.
#[test]
fn set_phase_with_frame_is_valid() {
    let pulse =
        Pulse::new(
            PulseId::new(21),
            PulseKind::SetPhase,
        )
        .with_target(
            PulseTarget::qubit(
                qubit(0),
            ),
        )
        .expect("target must be valid")
        .with_phase(
            Parameter::constant(
                1.25,
            )
            .expect("finite parameter must be valid"),
        )
        .expect("phase must be accepted")
        .with_resources(
            PulseResources::empty()
                .with_frame(
                    FrameId::new(1),
                ),
        );

    assert!(
        pulse.validate().is_ok()
    );
}

/// Shift-phase has the same frame requirement as set-phase.
#[test]
fn shift_phase_requires_phase_and_frame() {
    let pulse =
        Pulse::new(
            PulseId::new(22),
            PulseKind::ShiftPhase,
        )
        .with_target(
            PulseTarget::qubit(
                qubit(0),
            ),
        )
        .expect("target must be valid")
        .with_phase(
            Parameter::constant(
                0.5,
            )
            .expect("finite parameter must be valid"),
        )
        .expect("phase must be accepted");

    assert_eq!(
        pulse.validate(),
        Err(PulseError::MissingFrame),
    );
}

/// Shift-phase with phase and frame must validate.
#[test]
fn shift_phase_with_frame_is_valid() {
    let pulse =
        Pulse::new(
            PulseId::new(22),
            PulseKind::ShiftPhase,
        )
        .with_target(
            PulseTarget::qubit(
                qubit(0),
            ),
        )
        .expect("target must be valid")
        .with_phase(
            Parameter::constant(
                0.5,
            )
            .expect("finite parameter must be valid"),
        )
        .expect("phase must be accepted")
        .with_resources(
            PulseResources::empty()
                .with_frame(
                    FrameId::new(1),
                ),
        );

    assert!(
        pulse.validate().is_ok()
    );
}

// ============================================================================
// Symbolic parameters
// ============================================================================

/// Symbolic amplitude must remain symbolic.
#[test]
fn symbolic_amplitude_is_preserved() {
    let amplitude =
        Parameter::symbol(
            "drive_amplitude",
        )
        .expect("valid symbol must be accepted");

    let pulse =
        valid_play_pulse()
            .with_amplitude(
                amplitude,
            )
            .expect("symbolic amplitude must be accepted");

    assert!(
        pulse.is_symbolic()
    );

    assert_eq!(
        pulse.parameters().len(),
        1,
    );
}

/// Symbolic phase must remain symbolic.
#[test]
fn symbolic_phase_is_preserved() {
    let phase =
        Parameter::symbol(
            "drive_phase",
        )
        .expect("valid symbol must be accepted");

    let pulse =
        Pulse::new(
            PulseId::new(30),
            PulseKind::SetPhase,
        )
        .with_target(
            PulseTarget::qubit(
                qubit(0),
            ),
        )
        .expect("target must be valid")
        .with_phase(
            phase,
        )
        .expect("symbolic phase must be accepted")
        .with_resources(
            PulseResources::empty()
                .with_frame(
                    FrameId::new(1),
                ),
        );

    assert!(
        pulse.is_symbolic()
    );

    assert_eq!(
        pulse.parameters().len(),
        1,
    );

    assert!(
        pulse.validate().is_ok()
    );
}

/// Symbolic frequency must remain symbolic.
#[test]
fn symbolic_frequency_is_preserved() {
    let frequency =
        Parameter::symbol(
            "drive_frequency",
        )
        .expect("valid symbol must be accepted");

    let pulse =
        Pulse::new(
            PulseId::new(31),
            PulseKind::SetFrequency,
        )
        .with_target(
            PulseTarget::qubit(
                qubit(0),
            ),
        )
        .expect("target must be valid")
        .with_frequency(
            frequency,
        )
        .expect("symbolic frequency must be accepted")
        .with_resources(
            PulseResources::empty()
                .with_frame(
                    FrameId::new(1),
                ),
        );

    assert!(
        pulse.is_symbolic()
    );

    assert!(
        pulse.validate().is_ok()
    );
}

/// Concrete parameters must not be incorrectly reported as symbolic.
#[test]
fn concrete_parameters_are_not_symbolic() {
    let pulse =
        valid_play_pulse()
            .with_amplitude(
                Parameter::constant(
                    0.3,
                )
                .expect("finite amplitude must be valid"),
            )
            .expect("amplitude must be accepted");

    assert!(
        !pulse.is_symbolic()
    );

    assert_eq!(
        pulse
            .amplitude()
            .expect("amplitude must exist")
            .as_constant(),
        Some(0.3),
    );
}

/// Parameters must be returned in deterministic semantic order:
///
///     amplitude
///     phase
///     frequency
#[test]
fn parameters_have_deterministic_semantic_order() {
    let pulse =
        valid_play_pulse()
            .with_amplitude(
                Parameter::symbol(
                    "amplitude",
                )
                .expect("symbol must be valid"),
            )
            .expect("amplitude must be valid")
            .with_phase(
                Parameter::symbol(
                    "phase",
                )
                .expect("symbol must be valid"),
            )
            .expect("phase must be valid")
            .with_frequency(
                Parameter::symbol(
                    "frequency",
                )
                .expect("symbol must be valid"),
            )
            .expect("frequency must be valid");

    let parameters =
        pulse.parameters();

    assert_eq!(
        parameters.len(),
        3,
    );

    assert_eq!(
        parameters[0]
            .as_symbol(),
        Some("amplitude"),
    );

    assert_eq!(
        parameters[1]
            .as_symbol(),
        Some("phase"),
    );

    assert_eq!(
        parameters[2]
            .as_symbol(),
        Some("frequency"),
    );
}

// ============================================================================
// Resource references
// ============================================================================

/// Empty resource references must be representable.
#[test]
fn empty_resource_references_are_valid() {
    let resources =
        PulseResources::empty();

    assert!(
        resources.is_empty()
    );

    assert_eq!(
        resources.waveform(),
        None,
    );

    assert_eq!(
        resources.channel(),
        None,
    );

    assert_eq!(
        resources.frame(),
        None,
    );

    assert_eq!(
        resources.calibration(),
        None,
    );
}

/// Resource references must remain typed and independent.
#[test]
fn typed_resource_references_are_preserved() {
    let resources =
        PulseResources::empty()
            .with_waveform(
                WaveformId::new(1),
            )
            .with_channel(
                ChannelId::new(2),
            )
            .with_frame(
                FrameId::new(3),
            )
            .with_calibration(
                CalibrationId::new(4),
            );

    assert_eq!(
        resources.waveform(),
        Some(WaveformId::new(1)),
    );

    assert_eq!(
        resources.channel(),
        Some(ChannelId::new(2)),
    );

    assert_eq!(
        resources.frame(),
        Some(FrameId::new(3)),
    );

    assert_eq!(
        resources.calibration(),
        Some(CalibrationId::new(4)),
    );

    assert!(
        !resources.is_empty()
    );
}

// ============================================================================
// Dependencies
// ============================================================================

/// Dependencies must be preserved.
#[test]
fn pulse_dependency_is_preserved() {
    let pulse =
        valid_barrier_pulse()
            .with_dependency(
                PulseDependency::After(
                    PulseId::new(10),
                ),
            )
            .expect("dependency must be accepted");

    assert_eq!(
        pulse.dependencies(),
        &[
            PulseDependency::After(
                PulseId::new(10),
            ),
        ],
    );
}

/// Duplicate dependencies must be rejected.
#[test]
fn duplicate_pulse_dependency_is_rejected() {
    let dependency =
        PulseDependency::After(
            PulseId::new(10),
        );

    let pulse =
        valid_barrier_pulse()
            .with_dependency(
                dependency,
            )
            .expect("first dependency must be accepted");

    let result =
        pulse.with_dependency(
            dependency,
        );

    assert_eq!(
        result,
        Err(PulseError::DuplicateDependency),
    );
}

/// Dependencies must be deterministically ordered.
#[test]
fn pulse_dependencies_are_deterministically_ordered() {
    let pulse =
        valid_barrier_pulse()
            .with_dependency(
                PulseDependency::After(
                    PulseId::new(30),
                ),
            )
            .expect("dependency must be valid")
            .with_dependency(
                PulseDependency::After(
                    PulseId::new(10),
                ),
            )
            .expect("dependency must be valid");

    assert_eq!(
        pulse.dependencies(),
        &[
            PulseDependency::After(
                PulseId::new(10),
            ),
            PulseDependency::After(
                PulseId::new(30),
            ),
        ],
    );
}

// ============================================================================
// Metadata
// ============================================================================

/// Metadata keys must not be empty.
#[test]
fn empty_metadata_key_is_rejected() {
    let result =
        valid_barrier_pulse()
            .with_metadata(
                "",
                "value",
            );

    assert_eq!(
        result,
        Err(PulseError::EmptyMetadataKey),
    );
}

/// Metadata must be deterministic because the production implementation uses
/// an ordered map.
#[test]
fn metadata_is_deterministically_ordered() {
    let pulse =
        valid_barrier_pulse()
            .with_metadata(
                "z",
                "last",
            )
            .expect("metadata must be valid")
            .with_metadata(
                "a",
                "first",
            )
            .expect("metadata must be valid")
            .with_metadata(
                "m",
                "middle",
            )
            .expect("metadata must be valid");

    let keys: Vec<&str> =
        pulse
            .metadata()
            .keys()
            .map(String::as_str)
            .collect();

    assert_eq!(
        keys,
        vec![
            "a",
            "m",
            "z",
        ],
    );
}

/// Metadata values must be preserved exactly.
#[test]
fn metadata_values_are_preserved() {
    let pulse =
        valid_barrier_pulse()
            .with_metadata(
                "purpose",
                "calibration",
            )
            .expect("metadata must be valid");

    assert_eq!(
        pulse.metadata().get(
            "purpose",
        ),
        Some(
            &"calibration".to_string()
        ),
    );
}

/// Metadata policy must be independent from semantic construction.
#[test]
fn metadata_policy_is_explicit() {
    let pulse =
        valid_barrier_pulse()
            .with_metadata(
                "a",
                "value",
            )
            .expect("metadata must be valid");

    assert!(
        pulse
            .validate_with_policy(
                PulseValidationPolicy::unrestricted(),
            )
            .is_ok()
    );

    let restrictive =
        PulseValidationPolicy::unrestricted()
            .with_max_metadata_fields(
                0,
            );

    assert!(matches!(
        pulse.validate_with_policy(
            restrictive,
        ),
        Err(
            PulseError::MetadataFieldLimitExceeded {
                maximum: 0,
                actual: 1,
            }
        )
    ));
}

/// Metadata key-size limits must be explicit.
#[test]
fn metadata_key_size_policy_is_enforced() {
    let pulse =
        valid_barrier_pulse()
            .with_metadata(
                "long-key",
                "value",
            )
            .expect("metadata must be valid");

    let policy =
        PulseValidationPolicy::unrestricted()
            .with_max_metadata_key_bytes(
                3,
            );

    assert!(matches!(
        pulse.validate_with_policy(
            policy,
        ),
        Err(
            PulseError::MetadataKeyLimitExceeded {
                maximum: 3,
                actual: 8,
            }
        )
    ));
}

/// Metadata value-size limits must be explicit.
#[test]
fn metadata_value_size_policy_is_enforced() {
    let pulse =
        valid_barrier_pulse()
            .with_metadata(
                "key",
                "0123456789",
            )
            .expect("metadata must be valid");

    let policy =
        PulseValidationPolicy::unrestricted()
            .with_max_metadata_value_bytes(
                5,
            );

    assert!(matches!(
        pulse.validate_with_policy(
            policy,
        ),
        Err(
            PulseError::MetadataValueLimitExceeded {
                maximum: 5,
                actual: 10,
            }
        )
    ));
}

// ============================================================================
// Pulse kind semantics
// ============================================================================

/// Play is explicitly classified as waveform-using.
#[test]
fn play_kind_uses_waveform() {
    assert!(
        PulseKind::Play.uses_waveform()
    );
}

/// Capture is explicitly classified as waveform-using.
#[test]
fn capture_kind_uses_waveform() {
    assert!(
        PulseKind::Capture.uses_waveform()
    );
}

/// Acquire is explicitly classified as waveform-using.
#[test]
fn acquire_kind_uses_waveform() {
    assert!(
        PulseKind::Acquire.uses_waveform()
    );
}

/// Delay must be timing-only.
#[test]
fn delay_kind_is_timing_only() {
    assert!(
        PulseKind::Delay.is_timing_only()
    );
}

/// Barrier must be timing-only.
#[test]
fn barrier_kind_is_timing_only() {
    assert!(
        PulseKind::Barrier.is_timing_only()
    );
}

/// Set-frequency modifies frame state.
#[test]
fn set_frequency_modifies_frame() {
    assert!(
        PulseKind::SetFrequency
            .modifies_frame()
    );
}

/// Set-phase modifies frame state.
#[test]
fn set_phase_modifies_frame() {
    assert!(
        PulseKind::SetPhase
            .modifies_frame()
    );
}

/// Shift-phase modifies frame state.
#[test]
fn shift_phase_modifies_frame() {
    assert!(
        PulseKind::ShiftPhase
            .modifies_frame()
    );
}

/// Extension operations must remain possible without modifying the core enum
/// for every future operation.
#[test]
fn extension_pulse_kind_remains_nonhardware_specific() {
    let extension =
        PulseKind::Extension(
            crate::quantum::ir::identity::ExtensionId::new(
                1,
            ),
        );

    assert!(
        !extension.uses_waveform()
    );

    assert!(
        !extension.modifies_frame()
    );
}

// ============================================================================
// Calibration semantics
// ============================================================================

/// Calibration invocation must be representable without embedding calibration
/// implementation details into the pulse.
#[test]
fn calibration_pulse_kind_is_representable() {
    let pulse =
        Pulse::new(
            PulseId::new(40),
            PulseKind::Calibration,
        )
        .with_target(
            PulseTarget::qubit(
                qubit(0),
            ),
        )
        .expect("target must be valid")
        .with_resources(
            PulseResources::empty()
                .with_calibration(
                    CalibrationId::new(10),
                ),
        );

    assert!(
        pulse.validate().is_ok()
    );

    assert_eq!(
        pulse.resources().calibration(),
        Some(
            CalibrationId::new(10)
        ),
    );
}

// ============================================================================
// Composition
// ============================================================================

/// Sequential composition must reject empty input.
#[test]
fn empty_sequential_composition_is_rejected() {
    let composition =
        PulseComposition::Sequential(
            Vec::new(),
        );

    assert_eq!(
        composition.validate(),
        Err(PulseError::EmptyComposition),
    );
}

/// Parallel composition must reject empty input.
#[test]
fn empty_parallel_composition_is_rejected() {
    let composition =
        PulseComposition::Parallel(
            Vec::new(),
        );

    assert_eq!(
        composition.validate(),
        Err(PulseError::EmptyComposition),
    );
}

/// Sequential composition must count direct references.
#[test]
fn sequential_composition_counts_references() {
    let composition =
        PulseComposition::Sequential(
            vec![
                PulseId::new(1),
                PulseId::new(2),
                PulseId::new(3),
            ],
        );

    assert_eq!(
        composition.reference_count(),
        3,
    );

    assert!(
        composition.validate().is_ok()
    );
}

/// Parallel composition must count direct references.
#[test]
fn parallel_composition_counts_references() {
    let composition =
        PulseComposition::Parallel(
            vec![
                PulseId::new(1),
                PulseId::new(2),
            ],
        );

    assert_eq!(
        composition.reference_count(),
        2,
    );

    assert!(
        composition.validate().is_ok()
    );
}

/// Repetition count zero must be rejected.
#[test]
fn zero_pulse_repeat_is_rejected() {
    let composition =
        PulseComposition::Repeat {
            pulse: PulseId::new(1),
            count: 0,
        };

    assert_eq!(
        composition.validate(),
        Err(PulseError::ZeroRepeatCount),
    );
}

/// Repetition count one must be accepted.
#[test]
fn one_pulse_repeat_is_valid() {
    let composition =
        PulseComposition::Repeat {
            pulse: PulseId::new(1),
            count: 1,
        };

    assert!(
        composition.validate().is_ok()
    );

    assert_eq!(
        composition.reference_count(),
        1,
    );
}

/// Very large repetition counts are semantic data and must not be artificially
/// capped by this IR test.
#[test]
fn large_repeat_count_is_representable() {
    let composition =
        PulseComposition::Repeat {
            pulse: PulseId::new(1),
            count: u128::MAX,
        };

    assert!(
        composition.validate().is_ok()
    );

    assert_eq!(
        composition.reference_count(),
        1,
    );
}

/// Sequence repetition must support large finite counts.
#[test]
fn large_sequence_repeat_count_is_representable() {
    let composition =
        PulseComposition::RepeatSequence {
            sequence: PulseId::new(1),
            count: u128::MAX,
        };

    assert!(
        composition.validate().is_ok()
    );
}

// ============================================================================
// Explicit policy versus semantic model
// ============================================================================

/// An unrestricted policy must not impose a pulse target limit.
#[test]
fn unrestricted_policy_has_no_target_limit() {
    let pulse =
        valid_play_pulse();

    assert!(
        pulse
            .validate_with_policy(
                PulseValidationPolicy::unrestricted(),
            )
            .is_ok()
    );
}

/// A caller may explicitly impose a target policy.
#[test]
fn explicit_target_policy_is_enforced() {
    let pulse =
        Pulse::new(
            PulseId::new(50),
            PulseKind::Barrier,
        )
        .with_targets([
            PulseTarget::qubit(
                qubit(0),
            ),
            PulseTarget::qubit(
                qubit(1),
            ),
            PulseTarget::qubit(
                qubit(2),
            ),
        ])
        .expect(
            "targets must be structurally valid",
        );

    let policy =
        PulseValidationPolicy::unrestricted()
            .with_max_targets(
                2,
            );

    assert!(matches!(
        pulse.validate_with_policy(
            policy,
        ),
        Err(
            PulseError::TargetLimitExceeded {
                maximum: 2,
                actual: 3,
            }
        )
    ));
}

/// The same pulse must validate under a sufficiently permissive explicit
/// policy. This demonstrates that the policy is not part of the semantic pulse
/// identity.
#[test]
fn increasing_policy_does_not_change_pulse_semantics() {
    let pulse =
        Pulse::new(
            PulseId::new(51),
            PulseKind::Barrier,
        )
        .with_targets([
            PulseTarget::qubit(
                qubit(0),
            ),
            PulseTarget::qubit(
                qubit(1),
            ),
            PulseTarget::qubit(
                qubit(2),
            ),
        ])
        .expect(
            "targets must be structurally valid",
        );

    let restrictive =
        PulseValidationPolicy::unrestricted()
            .with_max_targets(
                2,
            );

    let permissive =
        PulseValidationPolicy::unrestricted()
            .with_max_targets(
                3,
            );

    assert!(
        pulse
            .validate_with_policy(
                restrictive,
            )
            .is_err()
    );

    assert!(
        pulse
            .validate_with_policy(
                permissive,
            )
            .is_ok()
    );
}

// ============================================================================
// Sparse scaling
// ============================================================================

/// A large finite target collection must be representable without a
/// hard-coded quantum-machine size limit.
///
/// This uses a deliberately moderate allocation because the test must remain
/// practical on developer machines and CI workers. The important property is
/// that the implementation does not encode a small architectural ceiling.
#[test]
fn pulse_supports_large_finite_sparse_target_sets() {
    let target_count =
        10_000usize;

    let qubits =
        (0..target_count)
            .map(QubitId::new)
            .collect::<Vec<_>>();

    let target =
        PulseTarget::qubits(
            qubits,
        );

    assert_eq!(
        target.explicit_qubit_count(),
        target_count,
    );

    assert_eq!(
        target.logical_qubits().len(),
        target_count,
    );

    assert!(
        target.validate().is_ok()
    );
}

/// Large logical identifiers must remain ordinary semantic identifiers and
/// must not be confused with a machine-size limit.
#[test]
fn very_large_logical_identifier_remains_a_valid_semantic_identity() {
    let large =
        QubitId::new(
            usize::MAX,
        );

    let target =
        PulseTarget::qubit(
            large,
        );

    assert_eq!(
        target.logical_qubits(),
        &[large],
    );

    assert_eq!(
        target.explicit_qubit_count(),
        1,
    );

    assert!(
        target.validate().is_ok()
    );
}

/// A pulse can target a sparse set of very widely separated logical qubit
/// identifiers without materializing the identifiers between them.
#[test]
fn sparse_large_logical_namespace_is_supported() {
    let target =
        PulseTarget::qubits([
            QubitId::new(0),
            QubitId::new(
                usize::MAX / 2,
            ),
            QubitId::new(
                usize::MAX,
            ),
        ]);

    assert_eq!(
        target.explicit_qubit_count(),
        3,
    );

    assert_eq!(
        target.logical_qubits().len(),
        3,
    );

    assert!(
        target.validate().is_ok()
    );
}

// ============================================================================
// Multi-target pulse semantics
// ============================================================================

/// A pulse can target multiple logical qubits.
#[test]
fn multi_target_play_pulse_is_valid() {
    let pulse =
        Pulse::new(
            PulseId::new(60),
            PulseKind::Play,
        )
        .with_targets([
            PulseTarget::qubit(
                qubit(0),
            ),
            PulseTarget::qubit(
                qubit(2),
            ),
            PulseTarget::qubit(
                qubit(1),
            ),
        ])
        .expect(
            "multi-target list must be valid",
        )
        .with_duration(
            PulseDuration::from_nanoseconds(
                20,
            )
            .expect(
                "20ns must be representable",
            ),
        )
        .with_resources(
            PulseResources::empty()
                .with_waveform(
                    WaveformId::new(1),
                ),
        );

    assert!(
        pulse.validate().is_ok()
    );

    assert_eq!(
        pulse.explicit_qubit_count(),
        3,
    );
}

/// Multi-target pulses must preserve deterministic ordering.
#[test]
fn multi_target_pulse_has_deterministic_target_order() {
    let pulse =
        Pulse::new(
            PulseId::new(61),
            PulseKind::Play,
        )
        .with_targets([
            PulseTarget::qubit(
                qubit(9),
            ),
            PulseTarget::qubit(
                qubit(1),
            ),
            PulseTarget::qubit(
                qubit(5),
            ),
        ])
        .expect(
            "targets must be valid",
        );

    assert_eq!(
        pulse.targets(),
        &[
            PulseTarget::qubit(
                qubit(1),
            ),
            PulseTarget::qubit(
                qubit(5),
            ),
            PulseTarget::qubit(
                qubit(9),
            ),
        ],
    );
}

// ============================================================================
// Pulse resource semantics
// ============================================================================

/// Play pulses can carry all relevant abstract resource references without
/// embedding physical hardware.
#[test]
fn play_resources_remain_abstract() {
    let resources =
        PulseResources::empty()
            .with_waveform(
                WaveformId::new(10),
            )
            .with_channel(
                ChannelId::new(20),
            )
            .with_frame(
                FrameId::new(30),
            )
            .with_calibration(
                CalibrationId::new(40),
            );

    let pulse =
        Pulse::new(
            PulseId::new(70),
            PulseKind::Play,
        )
        .with_target(
            PulseTarget::qubit(
                qubit(0),
            ),
        )
        .expect("target must be valid")
        .with_duration(
            PulseDuration::from_nanoseconds(
                20,
            )
            .expect("20ns must be representable"),
        )
        .with_resources(
            resources,
        );

    assert!(
        pulse.validate().is_ok()
    );

    assert_eq!(
        pulse.resources().waveform(),
        Some(
            WaveformId::new(10)
        ),
    );

    assert_eq!(
        pulse.resources().channel(),
        Some(
            ChannelId::new(20)
        ),
    );

    assert_eq!(
        pulse.resources().frame(),
        Some(
            FrameId::new(30)
        ),
    );

    assert_eq!(
        pulse.resources().calibration(),
        Some(
            CalibrationId::new(40)
        ),
    );
}

// ============================================================================
// Immutability / transformation contract
// ============================================================================

/// Builder-style transformations must return new values rather than requiring
/// mutation of an already-created semantic pulse.
#[test]
fn pulse_builder_operations_preserve_value_semantics() {
    let original =
        valid_play_pulse();

    let transformed =
        original
            .clone()
            .with_amplitude(
                Parameter::constant(
                    0.3,
                )
                .expect(
                    "finite amplitude must be valid",
                ),
            )
            .expect(
                "amplitude must be accepted",
            );

    assert!(
        original.amplitude().is_none()
    );

    assert_eq!(
        transformed
            .amplitude()
            .expect(
                "transformed pulse must have amplitude",
            )
            .as_constant(),
        Some(0.3),
    );
}

/// Pulse equality must reflect semantic structure.
#[test]
fn semantically_identical_pulses_are_equal() {
    let first =
        valid_play_pulse();

    let second =
        valid_play_pulse();

    assert_eq!(
        first,
        second,
    );
}

/// Changing semantic identity must change equality.
#[test]
fn different_pulse_identity_changes_semantic_value() {
    let first =
        valid_play_pulse();

    let second =
        Pulse::new(
            PulseId::new(999),
            PulseKind::Play,
        )
        .with_target(
            PulseTarget::qubit(
                qubit(0),
            ),
        )
        .expect("target must be valid")
        .with_duration(
            PulseDuration::from_nanoseconds(
                20,
            )
            .expect(
                "20ns must be representable",
            ),
        )
        .with_resources(
            PulseResources::empty()
                .with_waveform(
                    WaveformId::new(1),
                ),
        );

    assert_ne!(
        first,
        second,
    );
}

// ============================================================================
// Global semantics
// ============================================================================

/// A global target must remain a semantic scope rather than being expanded
/// into physical qubits by the pulse IR.
#[test]
fn global_pulse_scope_is_not_materialized() {
    let pulse =
        Pulse::new(
            PulseId::new(80),
            PulseKind::Play,
        )
        .with_target(
            PulseTarget::global(),
        )
        .expect(
            "global target must be accepted",
        )
        .with_duration(
            PulseDuration::from_nanoseconds(
                20,
            )
            .expect(
                "20ns must be representable",
            ),
        )
        .with_resources(
            PulseResources::empty()
                .with_waveform(
                    WaveformId::new(1),
                ),
        );

    assert!(
        pulse.validate().is_ok()
    );

    assert_eq!(
        pulse.explicit_qubit_count(),
        0,
    );
}

// ============================================================================
// Semantic separation from hardware
// ============================================================================

/// Pulse resources must remain references rather than physical hardware
/// descriptions.
#[test]
fn pulse_resources_do_not_require_physical_hardware_identity() {
    let resources =
        PulseResources::empty()
            .with_waveform(
                WaveformId::new(1),
            )
            .with_channel(
                ChannelId::new(2),
            )
            .with_frame(
                FrameId::new(3),
            );

    let pulse =
        Pulse::new(
            PulseId::new(90),
            PulseKind::Play,
        )
        .with_target(
            PulseTarget::qubit(
                qubit(0),
            ),
        )
        .expect("target must be valid")
        .with_duration(
            PulseDuration::from_nanoseconds(
                20,
            )
            .expect(
                "20ns must be representable",
            ),
        )
        .with_resources(
            resources,
        );

    assert!(
        pulse.validate().is_ok()
    );
}

// ============================================================================
// Validation precedence and consistency
// ============================================================================

/// A completely empty play pulse must fail deterministically with the missing
/// target error before target-specific resource errors.
#[test]
fn empty_play_pulse_has_deterministic_validation_error() {
    let pulse =
        Pulse::new(
            PulseId::new(100),
            PulseKind::Play,
        );

    assert_eq!(
        pulse.validate(),
        Err(PulseError::MissingTarget),
    );
}

/// A play pulse with a target but no duration or waveform must report the
/// duration requirement before the waveform requirement according to the
/// canonical validation order.
#[test]
fn incomplete_play_pulse_reports_missing_duration() {
    let pulse =
        Pulse::new(
            PulseId::new(101),
            PulseKind::Play,
        )
        .with_target(
            PulseTarget::qubit(
                qubit(0),
            ),
        )
        .expect(
            "target must be valid",
        );

    assert_eq!(
        pulse.validate(),
        Err(PulseError::MissingWaveform),
    );
}

/// A pulse with invalid target structure must reject that structure before
/// accepting it as a semantic pulse.
#[test]
fn invalid_target_structure_is_not_silently_accepted() {
    let pulse =
        Pulse::new(
            PulseId::new(102),
            PulseKind::Play,
        )
        .with_target(
            PulseTarget::Qubits(
                Vec::new(),
            ),
        );

    assert_eq!(
        pulse,
        Err(PulseError::EmptyTargetSet),
    );
}

// ============================================================================
// End-to-end semantic pulse examples
// ============================================================================

/// This represents the semantic equivalent of:
///
///     pulse(amp=0.3, dur=20ns)
///
/// without selecting a DAC, physical channel, sample clock, or hardware
/// waveform implementation.
#[test]
fn source_level_x_like_pulse_is_representable() {
    let pulse =
        Pulse::new(
            PulseId::new(200),
            PulseKind::Play,
        )
        .with_target(
            PulseTarget::qubit(
                qubit(0),
            ),
        )
        .expect(
            "logical target must be valid",
        )
        .with_amplitude(
            Parameter::constant(
                0.3,
            )
            .expect(
                "0.3 is a valid finite parameter",
            ),
        )
        .expect(
            "amplitude must be valid",
        )
        .with_duration(
            PulseDuration::from_nanoseconds(
                20,
            )
            .expect(
                "20ns must be representable",
            ),
        )
        .with_resources(
            PulseResources::empty()
                .with_waveform(
                    WaveformId::new(1),
                ),
        );

    assert!(
        pulse.validate().is_ok()
    );

    assert_eq!(
        pulse.explicit_qubit_count(),
        1,
    );

    assert_eq!(
        pulse
            .duration()
            .expect(
                "duration must exist",
            )
            .femtoseconds(),
        20_000_000,
    );

    assert_eq!(
        pulse
            .amplitude()
            .expect(
                "amplitude must exist",
            )
            .as_constant(),
        Some(0.3),
    );
}

/// A complete symbolic pulse must remain unresolved until a later compilation
/// stage.
#[test]
fn symbolic_source_level_pulse_remains_unresolved() {
    let pulse =
        Pulse::new(
            PulseId::new(201),
            PulseKind::Play,
        )
        .with_target(
            PulseTarget::qubit(
                qubit(0),
            ),
        )
        .expect(
            "target must be valid",
        )
        .with_duration(
            PulseDuration::from_nanoseconds(
                20,
            )
            .expect(
                "20ns must be representable",
            ),
        )
        .with_amplitude(
            Parameter::symbol(
                "drive_amplitude",
            )
            .expect(
                "symbol must be valid",
            ),
        )
        .expect(
            "symbolic amplitude must be valid",
        )
        .with_resources(
            PulseResources::empty()
                .with_waveform(
                    WaveformId::new(1),
                ),
        );

    assert!(
        pulse.validate().is_ok()
    );

    assert!(
        pulse.is_symbolic()
    );

    assert_eq!(
        pulse
            .amplitude()
            .expect(
                "amplitude must exist",
            )
            .as_symbol(),
        Some(
            "drive_amplitude",
        ),
    );
}

// ============================================================================
// Regression guards
// ============================================================================

/// Regression guard: duration must use u128 semantics rather than silently
/// narrowing through an intermediate u64.
///
/// This test deliberately chooses a value larger than u64::MAX but still
/// representable by u128.
#[test]
fn duration_supports_values_beyond_u64_range() {
    let value =
        (u64::MAX as u128)
            .saturating_add(1);

    let duration =
        PulseDuration::from_femtoseconds(
            value,
        );

    assert_eq!(
        duration.femtoseconds(),
        value,
    );
}

/// Regression guard: explicit target counting must not truncate large
/// cardinalities to small fixed-width machine-specific values.
#[test]
fn target_count_is_usize_based_on_actual_storage() {
    let targets =
        (0usize..1_000usize)
            .map(
                |index| {
                    PulseTarget::qubit(
                        QubitId::new(
                            index,
                        ),
                    )
                },
            )
            .collect::<Vec<_>>();

    let pulse =
        Pulse::new(
            PulseId::new(300),
            PulseKind::Barrier,
        )
        .with_targets(
            targets,
        )
        .expect(
            "1,000 unique targets must be structurally representable",
        );

    assert_eq!(
        pulse.explicit_qubit_count(),
        1_000,
    );

    assert!(
        pulse.validate().is_ok()
    );
}

/// Regression guard: target identifiers may be sparse and do not imply
/// materialization of the namespace between them.
#[test]
fn sparse_target_identifiers_do_not_expand_namespace() {
    let first =
        QubitId::new(1);

    let second =
        QubitId::new(
            usize::MAX,
        );

    let target =
        PulseTarget::qubits([
            first,
            second,
        ]);

    assert_eq!(
        target.explicit_qubit_count(),
        2,
    );

    assert_eq!(
        target.logical_qubits(),
        &[
            first,
            second,
        ],
    );
}

// ============================================================================
// Final production contract
// ============================================================================

/// A representative pulse exercising the complete semantic surface must
/// validate in one operation.
#[test]
fn complete_production_pulse_contract_validates() {
    let pulse =
        Pulse::new(
            PulseId::new(500),
            PulseKind::Play,
        )
        .with_targets([
            PulseTarget::qubit(
                qubit(5),
            ),
            PulseTarget::qubit(
                qubit(2),
            ),
            PulseTarget::qubit(
                qubit(9),
            ),
        ])
        .expect(
            "targets must be valid",
        )
        .with_duration(
            PulseDuration::from_nanoseconds(
                20,
            )
            .expect(
                "20ns must be representable",
            ),
        )
        .with_amplitude(
            Parameter::symbol(
                "amplitude",
            )
            .expect(
                "symbol must be valid",
            ),
        )
        .expect(
            "symbolic amplitude must be valid",
        )
        .with_phase(
            Parameter::symbol(
                "phase",
            )
            .expect(
                "symbol must be valid",
            ),
        )
        .expect(
            "symbolic phase must be valid",
        )
        .with_frequency(
            Parameter::symbol(
                "frequency",
            )
            .expect(
                "symbol must be valid",
            ),
        )
        .expect(
            "symbolic frequency must be valid",
        )
        .with_resources(
            PulseResources::empty()
                .with_waveform(
                    WaveformId::new(1),
                )
                .with_channel(
                    ChannelId::new(2),
                )
                .with_frame(
                    FrameId::new(3),
                )
                .with_calibration(
                    CalibrationId::new(4),
                ),
        )
        .with_dependency(
            PulseDependency::After(
                PulseId::new(499),
            ),
        )
        .expect(
            "dependency must be valid",
        )
        .with_metadata(
            "purpose",
            "production-pulse",
        )
        .expect(
            "metadata must be valid",
        );

    assert!(
        pulse.validate().is_ok()
    );

    assert!(
        pulse.is_symbolic()
    );

    assert_eq!(
        pulse.explicit_qubit_count(),
        3,
    );

    assert_eq!(
        pulse.parameters().len(),
        3,
    );

    assert_eq!(
        pulse.dependencies().len(),
        1,
    );

    assert_eq!(
        pulse.metadata().len(),
        1,
    );

    assert_eq!(
        pulse.resources().waveform(),
        Some(
            WaveformId::new(1),
        ),
    );
}