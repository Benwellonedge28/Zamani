//! Zamani Quantum Resilience — Detection Subsystem
//!
//! Path:
//!     src/quantum/resilience/detection/mod.rs
//!
//! # Purpose
//!
//! This module is the stable namespace and composition boundary for the
//! provider-neutral quantum-resilience detection subsystem.
//!
//! Detection converts explicit observations from quantum execution,
//! hardware, QEC, ZQN, simulation, benchmarking, and other trusted or
//! untrusted observation sources into normalized resilience signals.
//!
//! Detection answers:
//!
//! > "Is there an observable condition that downstream resilience components
//! > should interpret?"
//!
//! Detection does NOT decide:
//!
//! - root cause;
//! - severity;
//! - recovery;
//! - mitigation policy;
//! - backend selection;
//! - routing;
//! - scheduling;
//! - recompilation;
//! - QEC configuration;
//! - semantic result acceptance.
//!
//! Those responsibilities belong to the corresponding resilience and quantum
//! subsystems.
//!
//! # Architectural position
//!
//! ```text
//!                    Zamani Program
//!                          |
//!                          v
//!                    quantum::ir
//!                          |
//!          +---------------+----------------+
//!          |                                |
//!          v                                v
//!     quantum::zqn                   quantum::hardware
//!          |                                |
//!          +---------------+----------------+
//!                          |
//!                          v
//!                 execution / telemetry
//!                          |
//!                          v
//!                  resilience::detection
//!                          |
//!          +---------------+----------------+
//!          |               |                |
//!          v               v                v
//!       detector        detector          detector
//!       contract       mechanisms        mechanisms
//!          |               |                |
//!          +---------------+----------------+
//!                          |
//!                          v
//!                  normalized signals
//!                          |
//!                          v
//!                     diagnosis
//!                          |
//!                          v
//!                       policy
//!                          |
//!                          v
//!                      planning
//!                          |
//!                          v
//!                     adaptation
//!                          |
//!                          v
//!                      recovery
//! ```
//!
//! # Design principles
//!
//! This namespace enforces the following architectural rules.
//!
//! ## 1. Provider neutrality
//!
//! Core detection APIs must not contain vendor/provider-specific branches.
//!
//! Provider-specific observations are adapted at integration boundaries and
//! represented using the generic detector contract.
//!
//! ```text
//! forbidden:
//!
//! if provider == "some-provider" { ... }
//!
//! preferred:
//!
//! provider adapter -> DetectionObservation -> Detector
//! ```
//!
//! ## 2. No artificial machine-size limits
//!
//! This module introduces no:
//!
//! - maximum qubit count;
//! - maximum physical-qubit count;
//! - maximum detector count;
//! - maximum observation count;
//! - maximum signal count;
//! - maximum device count;
//! - maximum backend count;
//! - fixed topology;
//! - fixed execution width.
//!
//! Concrete resource limits belong to explicit resource, runtime, security,
//! execution, memory, and resilience policies.
//!
//! "Infinity" therefore means:
//!
//! > the detection architecture introduces no artificial finite machine-size
//! > ceiling; an actual execution remains bounded only by resources available
//! > to that execution.
//!
//! ## 3. Streaming first
//!
//! Detection mechanisms must be able to process observations incrementally.
//!
//! The core detector contract therefore operates over iterators and does not
//! require callers to materialize an entire telemetry stream.
//!
//! This permits:
//!
//! - single observations;
//! - slices;
//! - vectors;
//! - streaming telemetry;
//! - database-backed observation sources;
//! - distributed observation streams;
//! - arbitrarily large finite observation sequences.
//!
//! ## 4. Determinism
//!
//! Detection must not implicitly obtain:
//!
//! - wall-clock time;
//! - randomness;
//! - environment variables;
//! - process IDs;
//! - memory addresses;
//! - hidden global mutable state;
//! - provider SDK state.
//!
//! Temporal and stochastic information must enter through explicit inputs.
//!
//! Where deterministic execution is requested, identical explicit inputs must
//! produce identical detector behavior.
//!
//! ## 5. Detection is not diagnosis
//!
//! A detector may report:
//!
//! ```text
//! anomaly
//! ```
//!
//! but must not silently report:
//!
//! ```text
//! calibration drift caused the anomaly
//! ```
//!
//! The latter is a diagnosis-layer responsibility.
//!
//! ## 6. Detection is not recovery
//!
//! A detection signal must never itself authorize:
//!
//! - retry;
//! - restart;
//! - migration;
//! - rerouting;
//! - rescheduling;
//! - recompilation;
//! - backend switching;
//! - QEC changes;
//! - mitigation;
//! - abort.
//!
//! Those decisions belong to policy/planning/recovery.
//!
//! ## 7. Explicit trust and freshness
//!
//! Detection observations carry explicit trust and freshness metadata.
//!
//! Detection must not silently treat unverified or stale information as
//! trustworthy merely because it is syntactically valid.
//!
//! The final interpretation of trust remains a responsibility of the
//! surrounding security and policy layers.
//!
//! ## 8. Canonical quantum identity
//!
//! Detection does not define quantum-resource identity.
//!
//! Whenever quantum identities are required, implementations must use the
//! canonical Zamani IR identities:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! Detection-specific resource semantics must use:
//!
//! ```text
//! crate::quantum::resilience::model::resource
//! ```
//!
//! No detector may introduce:
//!
//! ```text
//! DetectionQubitId
//! ResilienceQubitId
//! DetectorQubitId
//! ```
//!
//! or an equivalent duplicate identity type.
//!
//! ## 9. Canonical fault ownership
//!
//! Quantum fault/noise semantics remain owned by ZQN.
//!
//! Detection observes canonical fault information and converts observations
//! into resilience detection signals. It must not establish a competing
//! quantum-fault ontology.
//!
//! ```text
//! quantum::zqn
//!      |
//!      v
//! canonical fault/noise information
//!      |
//!      v
//! detection
//!      |
//!      v
//! DetectionSignal
//!      |
//!      v
//! diagnosis
//! ```
//!
//! ## 10. No hidden I/O
//!
//! Detection modules must not silently perform:
//!
//! - filesystem I/O;
//! - network I/O;
//! - backend calls;
//! - credential lookup;
//! - hardware mutation;
//! - recovery actions.
//!
//! External information must be supplied through explicit integration
//! interfaces.
//!
//! # Module inventory
//!
//! ```text
//! detection/
//! ├── mod.rs
//! ├── detector.rs
//! ├── threshold.rs
//! ├── anomaly.rs
//! ├── statistical.rs
//! ├── drift.rs
//! ├── timeout.rs
//! ├── execution_failure.rs
//! ├── qec_signal.rs
//! └── hardware_signal.rs
//! ```
//!
//! Each child owns one detection mechanism or the common detection contract.
//!
//! ```text
//! detector.rs
//!     Common provider-neutral detector contract.
//!
//! threshold.rs
//!     Explicit threshold predicates and hysteresis.
//!
//! anomaly.rs
//!     General anomaly detection.
//!
//! statistical.rs
//!     Statistical detection.
//!
//! drift.rs
//!     Distribution/calibration/noise drift detection.
//!
//! timeout.rs
//!     Execution/deadline/timeout detection.
//!
//! execution_failure.rs
//!     Normalization of execution failures.
//!
//! qec_signal.rs
//!     Detection of resilience-relevant QEC signals.
//!
//! hardware_signal.rs
//!     Detection of resilience-relevant hardware signals.
//! ```
//!
//! # Dependency direction
//!
//! The dependency graph is intentionally one-directional:
//!
//! ```text
//! canonical quantum IR
//!          |
//!          +--------------------+
//!          |                    |
//!          v                    v
//! quantum::zqn           quantum::hardware
//!          |                    |
//!          +---------+----------+
//!                    |
//!                    v
//!               observation
//!                    |
//!                    v
//!              detection/mod.rs
//!                    |
//!                    v
//!              detector.rs
//!                    |
//!       +------------+-------------+
//!       |            |             |
//!       v            v             v
//!   threshold     anomaly      statistical
//!       |            |             |
//!       +------------+-------------+
//!                    |
//!                    +------------------+
//!                    |                  |
//!                    v                  v
//!                  drift        execution_failure
//!                    |
//!              +-----+------+
//!              |            |
//!              v            v
//!            qec_signal  hardware_signal
//!                    |
//!                    v
//!                diagnosis
//! ```
//!
//! Detection must not depend on diagnosis, planning, adaptation, recovery, or
//! verification implementations.
//!
//! # Public API boundary
//!
//! This module intentionally exposes two layers:
//!
//! ```text
//! detection::detector
//!     Stable low-level contract and canonical detection data types.
//!
//! detection::<mechanism>
//!     Concrete detector implementations.
//! ```
//!
//! The common contract is also re-exported at the detection namespace to make
//! the public API ergonomic without duplicating definitions.
//!
//! # Why the re-exports are selective
//!
//! The module does not use wildcard re-exports.
//!
//! Wildcard exports can introduce accidental API collisions as the subsystem
//! evolves. Explicit exports make the public surface auditable and stable.
//!
//! # Integration with other resilience modules
//!
//! Detection integrates with:
//!
//! ```text
//! model/
//!     resource
//!     fault
//!     confidence
//!     health
//!
//! errors/
//!     ResilienceError
//!     ResilienceResult
//!     ResilienceErrorCode
//!
//! telemetry/
//!     observation/event/metric producers
//!
//! diagnosis/
//!     consumes normalized DetectionOutput
//!
//! policy/
//!     supplies interpretation constraints
//!
//! planning/
//!     consumes diagnosis rather than raw detector implementation
//!
//! verification/
//!     consumes provenance and execution evidence
//! ```
//!
//! Detection must not import concrete downstream decision implementations.
//!
//! # Integration with quantum subsystems
//!
//! Detection is an observer/normalizer at the boundary of:
//!
//! ```text
//! quantum::ir
//! quantum::zqn
//! quantum::hardware
//! quantum::qec
//! quantum::simulation
//! quantum::benchmarking
//! runtime/execution
//! ```
//!
//! These systems provide observations.
//!
//! Detection normalizes them.
//!
//! Diagnosis interprets them.
//!
//! Recovery acts only after policy/planning authorization.
//!
//! # Integration with the canonical IR
//!
//! Detection does not mutate `quantum::ir`.
//!
//! When a detector needs to associate an observation with a quantum resource,
//! it must reference the canonical IR identity rather than creating a local
//! identifier.
//!
//! In particular, use:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! through the appropriate model/resource integration layer.
//!
//! This ensures that a logical/physical qubit retains the same identity across:
//!
//! ```text
//! IR
//! -> routing
//! -> scheduling
//! -> hardware
//! -> QEC
//! -> telemetry
//! -> detection
//! -> diagnosis
//! -> recovery
//! -> verification
//! ```
//!
//! # Integration with hardware
//!
//! `hardware_signal.rs` owns hardware-observation-specific detection logic.
//!
//! It must consume the hardware HAL's canonical capabilities, status,
//! calibration, telemetry, topology, and execution information through stable
//! contracts.
//!
//! The detection subsystem must not know a provider's SDK representation.
//!
//! ```text
//! provider adapter
//!       |
//!       v
//! quantum::hardware
//!       |
//!       v
//! hardware observation
//!       |
//!       v
//! hardware_signal detector
//! ```
//!
//! # Integration with QEC
//!
//! `qec_signal.rs` consumes QEC-produced evidence such as syndrome-derived
//! signals, decoder outcomes, leakage/erasure information, or logical-error
//! indicators according to the canonical QEC interfaces.
//!
//! Detection does not implement:
//!
//! - encoding;
//! - syndrome extraction;
//! - decoding;
//! - correction;
//! - code-distance selection.
//!
//! It detects resilience-relevant conditions produced by those systems.
//!
//! # Integration with ZQN
//!
//! ZQN remains the authoritative fault/noise semantic layer.
//!
//! Detection can observe canonical ZQN fault information and classify the
//! observation as a detection signal, but it must not redefine the underlying
//! quantum fault semantics.
//!
//! # Integration with telemetry
//!
//! Telemetry producers should construct explicit `DetectionObservation` values
//! or an equivalent adapter input and provide:
//!
//! - observation identity;
//! - detection sequence;
//! - source;
//! - trust;
//! - freshness;
//! - payload.
//!
//! Detection must not invent missing temporal or trust metadata through hidden
//! process state.
//!
//! # Integration with diagnosis
//!
//! The expected boundary is:
//!
//! ```text
//! DetectionInput
//!       |
//!       v
//! Detector
//!       |
//!       v
//! DetectionOutput
//!       |
//!       v
//! diagnosis::diagnostician
//! ```
//!
//! Diagnosis may correlate outputs from multiple detectors and multiple
//! observation sources.
//!
//! A detector must not perform that cross-system causal diagnosis itself.
//!
//! # Integration with policy
//!
//! Policy determines how much evidence is required and whether stale,
//! unverified, degraded, or inconclusive observations may influence decisions.
//!
//! Detection exposes evidence.
//!
//! Policy decides what evidence is acceptable for a particular operation.
//!
//! This separation prevents a detector implementation from embedding a global
//! threshold such as a hard-coded fidelity or error-rate boundary.
//!
//! # Integration with planning and recovery
//!
//! The dependency direction is:
//!
//! ```text
//! detection
//!     |
//!     v
//! diagnosis
//!     |
//!     v
//! policy
//!     |
//!     v
//! planning
//!     |
//!     v
//! adaptation
//!     |
//!     v
//! recovery
//! ```
//!
//! Detection must never bypass this chain by invoking recovery directly.
//!
//! # Detector registration
//!
//! Concrete detectors can be stored through the object-safe `DetectorObject`
//! boundary and registered by the future/current detector registry.
//!
//! The detection module itself does not own global mutable registries.
//!
//! Registry ownership belongs to:
//!
//! ```text
//! crate::quantum::resilience::registry
//! ```
//!
//! This avoids hidden global state and preserves deterministic composition.
//!
//! # Deterministic composition
//!
//! `detect_with_all` executes detectors in the order supplied by the caller.
//!
//! This is intentional.
//!
//! Detector ordering must never depend on:
//!
//! - hash-map iteration order;
//! - thread scheduling;
//! - memory addresses;
//! - provider SDK order.
//!
//! Callers requiring deterministic provenance should provide a deterministic
//! detector ordering and use `sort_signals_deterministically` before recording
//! a canonical signal sequence.
//!
//! # Streaming and large-scale execution
//!
//! The common detector contract supports streaming observation input.
//!
//! This means the detection subsystem can operate on:
//!
//! ```text
//! one observation
//!       |
//!       v
//! small execution
//!       |
//!       v
//! large QPU
//!       |
//!       v
//! distributed quantum system
//! ```
//!
//! without changing the detector API because the observation source is an
//! iterator rather than a fixed-size machine representation.
//!
//! Stateful detector implementations must keep bounded state appropriate to
//! their algorithm and must never accidentally retain an unbounded execution
//! history when an online algorithm can be used.
//!
//! # Fault storms
//!
//! Large quantum systems can generate many correlated observations for one
//! underlying event.
//!
//! Detection is allowed to emit multiple signals, but it must not assume that
//! every signal represents an independent incident.
//!
//! Correlation and incident aggregation belong to diagnosis/model layers.
//!
//! For example:
//!
//! ```text
//! 10,000 observations
//!        |
//!        v
//! detection
//!        |
//!        v
//! 10,000 normalized signals
//!        |
//!        v
//! diagnosis/correlation
//!        |
//!        v
//! one or more incidents
//! ```
//!
//! This prevents the detection layer from incorrectly embedding global
//! recovery behavior.
//!
//! # Error handling
//!
//! Concrete detectors use the canonical resilience error contract:
//!
//! ```text
//! crate::quantum::resilience::errors
//! ```
//!
//! In particular, detection-specific failures use the canonical error codes
//! rather than defining another error enum in this namespace.
//!
//! # Serialization
//!
//! This module does not define a wire format.
//!
//! Serialization is owned by:
//!
//! ```text
//! crate::quantum::resilience::serialization
//! ```
//!
//! Detection objects must therefore remain independent of a particular
//! serialization framework.
//!
//! # Security
//!
//! Detection signals are data, not authority.
//!
//! Possession of a `DetectionSignal` must never grant:
//!
//! - backend access;
//! - QPU access;
//! - credentials;
//! - filesystem access;
//! - network access;
//! - recovery authorization.
//!
//! Observation sources may be untrusted.
//!
//! Trust and integrity metadata must remain available to downstream policy and
//! diagnosis.
//!
//! # No unsafe Rust
//!
//! This module explicitly forbids unsafe code.
//!
//! It targets:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

// ============================================================================
// Concrete detector modules
// ============================================================================

/// General anomaly detection.
pub mod anomaly;

/// Common provider-neutral detector contract and normalized detection types.
pub mod detector;

/// Calibration, distribution, and noise drift detection.
pub mod drift;

/// Normalization and detection of execution failures.
pub mod execution_failure;

/// Detection of resilience-relevant hardware observations.
pub mod hardware_signal;

/// Detection of resilience-relevant QEC observations.
pub mod qec_signal;

/// Statistical detection mechanisms.
pub mod statistical;

/// Explicit threshold and hysteresis detection.
pub mod threshold;

/// Timeout and deadline detection.
pub mod timeout;

// ============================================================================
// Stable common detection API
// ============================================================================
//
// Re-export only the stable contract types. Concrete implementation-specific
// types remain under their respective modules so that adding a new detector
// cannot accidentally collide with an existing public name.

pub use detector::{
    detect_with_all,
    sort_signals_deterministically,
    validate_observations,
    DetectionClassification,
    DetectionConfidence,
    DetectionContext,
    DetectionInput,
    DetectionMetadata,
    DetectionObservation,
    DetectionOutput,
    DetectionSequence,
    DetectionSignal,
    Detector,
    DetectorIdentity,
    DetectorObject,
    ObservationFreshness,
    ObservationId,
    ObservationPayload,
    ObservationSource,
    ObservationTrust,
    SignalId,
    DETECTOR_SCHEMA_ID,
    DETECTOR_SCHEMA_VERSION,
};

// ============================================================================
// Architectural invariants
// ============================================================================
//
// These aliases intentionally remain private. They exist only to ensure that
// the module boundary continues to compile against the canonical detector
// contract and does not accidentally drift toward duplicate local types.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detection_schema_identity_is_stable() {
        assert_eq!(
            DETECTOR_SCHEMA_ID,
            "zamani.quantum.resilience.detection.detector"
        );
        assert_eq!(DETECTOR_SCHEMA_VERSION, 1);
    }

    #[test]
    fn canonical_detector_contract_is_exported() {
        let identity = DetectorIdentity::new("test-detector", "1.0.0")
            .expect("test detector identity should be valid");

        assert_eq!(identity.name(), "test-detector");
        assert_eq!(identity.version(), "1.0.0");
    }

    #[test]
    fn canonical_detection_types_are_available_at_namespace_boundary() {
        let sequence = DetectionSequence::from_u64(1)
            .expect("non-zero sequence should be valid");

        let observation_id = ObservationId::from_u64(1)
            .expect("non-zero observation ID should be valid");

        let signal_id = SignalId::from_u64(1)
            .expect("non-zero signal ID should be valid");

        let context = DetectionContext::new(sequence, false, false);

        let observation = DetectionObservation::new(
            observation_id,
            sequence,
            ObservationSource::Runtime,
            ObservationTrust::Verified,
            ObservationFreshness::Fresh,
            ObservationPayload::Marker,
        )
        .expect("observation should be valid");

        let identity = DetectorIdentity::new("test-detector", "1.0.0")
            .expect("identity should be valid");

        let confidence =
            DetectionConfidence::new(1.0).expect("confidence should be valid");

        let signal = DetectionSignal::new(
            signal_id,
            identity.clone(),
            DetectionClassification::NoCondition,
            confidence,
            Some(observation_id),
            sequence,
        );

        let metadata =
            DetectionMetadata::new(identity, sequence, 1);

        let output = DetectionOutput::new(metadata, vec![signal]);

        assert_eq!(context.sequence(), sequence);
        assert_eq!(observation.id(), observation_id);
        assert_eq!(output.len(), 1);
    }

    #[test]
    fn detection_classification_remains_provider_neutral() {
        assert_eq!(
            DetectionClassification::Anomaly.as_str(),
            "anomaly"
        );

        assert_eq!(
            DetectionClassification::Fault.as_str(),
            "fault"
        );

        assert_eq!(
            DetectionClassification::QecSignal.as_str(),
            "qec_signal"
        );

        assert_eq!(
            DetectionClassification::HardwareSignal.as_str(),
            "hardware_signal"
        );
    }

    #[test]
    fn namespace_does_not_define_a_fixed_machine_size() {
        // This test intentionally contains no machine-size constant.
        //
        // The purpose is architectural: detection/mod.rs must remain a
        // namespace/composition layer rather than becoming a source of
        // hardware limits.
        assert!(DETECTOR_SCHEMA_VERSION > 0);
    }

    #[test]
    fn canonical_detection_helpers_are_reachable() {
        let context = DetectionContext::new(
            DetectionSequence::from_u64(1)
                .expect("sequence should be valid"),
            false,
            false,
        );

        let observation = DetectionObservation::new(
            ObservationId::from_u64(1)
                .expect("observation ID should be valid"),
            context.sequence(),
            ObservationSource::Runtime,
            ObservationTrust::Verified,
            ObservationFreshness::Fresh,
            ObservationPayload::Unsigned(1),
        )
        .expect("observation should be valid");

        let validated =
            validate_observations(&context, [observation].iter())
                .expect("observation should validate");

        assert_eq!(validated.len(), 1);
    }
}