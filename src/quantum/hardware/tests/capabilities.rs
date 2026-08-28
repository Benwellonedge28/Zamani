//! Zamani Quantum Hardware — Capability Conformance Tests.
//!
//! Production conformance suite for:
//!
//! `crate::quantum::hardware::capabilities`
//!
//! # Responsibility
//!
//! This module verifies the public, provider-neutral capability contract:
//!
//! - capability identifiers;
//! - identifier parsing;
//! - identifier normalization;
//! - display/serialization stability;
//! - capability categories;
//! - explicit-opt-in classification;
//! - capability lifecycle states;
//! - capability policies;
//! - capability descriptors;
//! - deterministic capability sets;
//! - capability insertion/replacement/removal;
//! - stable/experimental/deprecated/unavailable handling;
//! - required/preferred capability requirements;
//! - capability negotiation;
//! - missing-capability diagnostics;
//! - rejected lifecycle-state diagnostics;
//! - merge semantics;
//! - JSON round trips;
//! - deterministic serialization;
//! - malformed serialized input;
//! - provider-neutrality invariants;
//! - broad quantum-technology coverage;
//! - regression protection for future hardware integrations.
//!
//! # Non-responsibilities
//!
//! This test module deliberately does NOT test:
//!
//! - provider APIs;
//! - provider authentication;
//! - credentials;
//! - network communication;
//! - QPU execution;
//! - routing algorithms;
//! - scheduling algorithms;
//! - calibration acquisition;
//! - topology algorithms;
//! - benchmarking algorithms;
//! - QEC algorithms;
//! - OpenQASM parsing;
//! - QIR generation;
//! - simulator implementations.
//!
//! Those concerns have their own ownership boundaries.
//!
//! # Integration contract
//!
//! This file is intentionally written against the stable public API of:
//!
//! `crate::quantum::hardware::capabilities`
//!
//! It does not import `backend.rs`, `provider.rs`, `execution.rs`,
//! `topology.rs`, `calibration.rs`, or any future hardware module.
//!
//! Consequently:
//!
//! 1. `capabilities.rs` can be completed and frozen independently.
//! 2. Future hardware modules can consume its API without requiring changes
//!    to this test module.
//! 3. Provider adapters can be added without modifying these tests.
//! 4. Benchmarking can consume the same capability contract without modifying
//!    these tests.
//!
//! # Production invariant
//!
//! A provider adapter MUST map its provider-specific capability information
//! into the authoritative `QuantumCapability` model rather than inventing a
//! second capability vocabulary.
//!
//! # Rust compatibility
//!
//! This test suite targets:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021.
//!
//! No nightly features are required.
//!
//! # Determinism
//!
//! Tests deliberately verify deterministic ordering and serialization.
//! Capability collections are expected to use ordered structures internally.
//!
//! # Security
//!
//! Capability identifiers are metadata only. No credential or secret is used
//! by this test suite.
//!
//! # External interoperability
//!
//! The capability model intentionally covers capabilities required by:
//!
//! - gate-model quantum processors;
//! - dynamic-circuit systems;
//! - pulse-level systems;
//! - analog systems;
//! - annealers;
//! - photonic/bosonic systems;
//! - logical/fault-tolerant systems;
//! - simulators/emulators;
//! - distributed quantum systems.
//!
//! OpenQASM/QIR/provider-specific interoperability belongs to adapter tests.
//! These tests only verify that the capability vocabulary can represent the
//! corresponding hardware-facing requirements.

use std::collections::{BTreeMap, BTreeSet};
use std::str::FromStr;

use serde_json::Value;

use crate::quantum::hardware::capabilities::{
    CapabilityCategory,
    CapabilityDescriptor,
    CapabilityParseError,
    CapabilityPolicy,
    CapabilityRequirements,
    CapabilitySet,
    CapabilityStatus,
    QuantumCapability,
};

// =============================================================================
// Test helpers
// =============================================================================

/// Returns a representative set covering all major capability domains.
///
/// The list is intentionally broad rather than exhaustive. The authoritative
/// enum remains owned by `capabilities.rs`; these tests verify representative
/// behavior from every architectural category.
fn representative_capabilities() -> &'static [QuantumCapability] {
    &[
        // Measurement
        QuantumCapability::Measurement,
        QuantumCapability::MidCircuitMeasurement,
        QuantumCapability::ObservableMeasurement,
        QuantumCapability::ExpectationValues,
        QuantumCapability::Sampling,
        QuantumCapability::ProbabilityDistributions,

        // State preparation / reset
        QuantumCapability::Reset,
        QuantumCapability::FastReset,
        QuantumCapability::MidCircuitReset,
        QuantumCapability::QubitReuse,
        QuantumCapability::StatePreparation,
        QuantumCapability::LeakageDetection,
        QuantumCapability::LeakageReduction,

        // Gate model
        QuantumCapability::SingleQubitGates,
        QuantumCapability::TwoQubitGates,
        QuantumCapability::ThreeQubitGates,
        QuantumCapability::MultiQubitGates,
        QuantumCapability::ArbitrarySingleQubitRotations,
        QuantumCapability::ParameterizedGates,
        QuantumCapability::NativeGateExecution,
        QuantumCapability::ControlledOperations,
        QuantumCapability::AdjointOperations,
        QuantumCapability::NonUnitaryOperations,

        // Dynamic/classical control
        QuantumCapability::ClassicalControl,
        QuantumCapability::ConditionalOperations,
        QuantumCapability::DynamicCircuits,
        QuantumCapability::ClassicalFeedForward,
        QuantumCapability::FastFeedForward,
        QuantumCapability::RuntimeBranching,
        QuantumCapability::RuntimeLoops,
        QuantumCapability::RuntimeClassicalExpressions,

        // Parameterized/variational
        QuantumCapability::ParameterizedExecution,
        QuantumCapability::RuntimeParameterBinding,
        QuantumCapability::ParameterBatchExecution,
        QuantumCapability::ParameterSweeps,
        QuantumCapability::VariationalExecution,

        // Timing
        QuantumCapability::TimingInformation,
        QuantumCapability::Delays,
        QuantumCapability::TimingAlignment,
        QuantumCapability::CycleTiming,
        QuantumCapability::Synchronization,
        QuantumCapability::InstructionDurations,
        QuantumCapability::HardwareClock,

        // Pulse
        QuantumCapability::PulseLevelControl,
        QuantumCapability::CustomWaveforms,
        QuantumCapability::DriveChannels,
        QuantumCapability::MeasureChannels,
        QuantumCapability::AcquireChannels,
        QuantumCapability::ControlChannels,
        QuantumCapability::PulseCalibrations,
        QuantumCapability::Frames,
        QuantumCapability::PulseSchedules,

        // Analog
        QuantumCapability::AnalogExecution,
        QuantumCapability::TimeDependentHamiltonians,
        QuantumCapability::SpatialHamiltonians,
        QuantumCapability::AnalogControlFields,
        QuantumCapability::AnalogObservables,
        QuantumCapability::AnalogProgramSubmission,

        // Annealing
        QuantumCapability::QuantumAnnealing,
        QuantumCapability::IsingModels,
        QuantumCapability::Qubo,
        QuantumCapability::AnnealingSchedules,
        QuantumCapability::ReverseAnnealing,
        QuantumCapability::AnnealingPauses,
        QuantumCapability::AnnealingGauges,

        // Photonic/bosonic
        QuantumCapability::PhotonicModes,
        QuantumCapability::BosonicOperations,
        QuantumCapability::ContinuousVariableOperations,
        QuantumCapability::FockStateOperations,
        QuantumCapability::GaussianOperations,
        QuantumCapability::NonGaussianOperations,
        QuantumCapability::PhotonNumberMeasurement,
        QuantumCapability::HomodyneMeasurement,
        QuantumCapability::HeterodyneMeasurement,

        // Logical/QEC
        QuantumCapability::LogicalQubits,
        QuantumCapability::LogicalGates,
        QuantumCapability::LogicalMeasurements,
        QuantumCapability::ErrorCorrectionCodes,
        QuantumCapability::SyndromeExtraction,
        QuantumCapability::DecoderExecution,
        QuantumCapability::FaultTolerantOperations,
        QuantumCapability::TransversalOperations,
        QuantumCapability::MagicStateSupport,
        QuantumCapability::LogicalReset,
        QuantumCapability::LogicalErrorRates,

        // Error mitigation / characterization
        QuantumCapability::NoiseModel,
        QuantumCapability::ReadoutErrorCharacterization,
        QuantumCapability::ReadoutErrorMitigation,
        QuantumCapability::GateErrorMitigation,
        QuantumCapability::ZeroNoiseExtrapolation,
        QuantumCapability::ProbabilisticErrorCancellation,
        QuantumCapability::RandomizedCompiling,
        QuantumCapability::ErrorRateEstimation,

        // Concurrency / resources
        QuantumCapability::ParallelOperations,
        QuantumCapability::ConcurrentExecution,
        QuantumCapability::BatchExecution,
        QuantumCapability::StreamingExecution,
        QuantumCapability::CircuitBatching,
        QuantumCapability::JobPriorities,
        QuantumCapability::QueueInformation,
        QuantumCapability::Reservations,

        // Execution lifecycle
        QuantumCapability::AsynchronousExecution,
        QuantumCapability::SynchronousExecution,
        QuantumCapability::JobStatus,
        QuantumCapability::JobCancellation,
        QuantumCapability::JobTimeout,
        QuantumCapability::RetryableExecution,
        QuantumCapability::ExecutionMetadata,
        QuantumCapability::CostEstimation,

        // Results
        QuantumCapability::Counts,
        QuantumCapability::RawSamples,
        QuantumCapability::StateVector,
        QuantumCapability::DensityMatrix,
        QuantumCapability::Amplitudes,
        QuantumCapability::Wavefunction,
        QuantumCapability::ClassicalRegisters,
        QuantumCapability::RawMeasurementRecords,
        QuantumCapability::AnalogResultAcquisition,
        QuantumCapability::AnnealingResults,
        QuantumCapability::LogicalResults,

        // Calibration
        QuantumCapability::CalibrationData,
        QuantumCapability::CalibrationSnapshots,
        QuantumCapability::CalibrationVersioning,
        QuantumCapability::CalibrationProvenance,
        QuantumCapability::HardwareCharacterization,

        // Topology
        QuantumCapability::TopologyInformation,
        QuantumCapability::ConnectivityInformation,
        QuantumCapability::DirectedCoupling,
        QuantumCapability::ModularTopology,
        QuantumCapability::InterModuleConnectivity,

        // Simulation / emulation
        QuantumCapability::StateVectorSimulation,
        QuantumCapability::StabilizerSimulation,
        QuantumCapability::TensorNetworkSimulation,
        QuantumCapability::DensityMatrixSimulation,
        QuantumCapability::TrajectorySimulation,
        QuantumCapability::NoisySimulation,
        QuantumCapability::DeterministicSimulation,
        QuantumCapability::HardwareEmulation,
        QuantumCapability::FaultInjection,

        // Distributed quantum
        QuantumCapability::DistributedExecution,
        QuantumCapability::RemoteQuantumResources,
        QuantumCapability::QuantumNetworkLinks,
        QuantumCapability::EntanglementResources,
        QuantumCapability::RemoteOperations,
        QuantumCapability::Teleportation,

        // Provider extension
        QuantumCapability::ProviderExtension,
    ]
}

/// Assert that a capability identifier is canonical.
///
/// This protects persisted configuration, manifests, telemetry and benchmark
/// provenance from accidental identifier drift.
fn assert_canonical_identifier(capability: QuantumCapability) {
    let identifier = capability.as_str();

    assert!(
        !identifier.is_empty(),
        "capability {:?} has an empty identifier",
        capability
    );

    assert!(
        !identifier.starts_with('_'),
        "capability identifier must not start with '_'"
    );

    assert!(
        !identifier.ends_with('_'),
        "capability identifier must not end with '_'"
    );

    assert!(
        !identifier.contains(' '),
        "capability identifier must not contain spaces: {identifier}"
    );

    assert!(
        identifier
            .chars()
            .all(|character| character.is_ascii_lowercase()
                || character.is_ascii_digit()
                || character == '_'),
        "capability identifier is not canonical ASCII snake_case: {identifier}"
    );
}

// =============================================================================
// Identifier tests
// =============================================================================

#[test]
fn representative_capabilities_have_non_empty_stable_identifiers() {
    for &capability in representative_capabilities() {
        assert_canonical_identifier(capability);
    }
}

#[test]
fn representative_capability_identifiers_are_unique() {
    let mut identifiers = BTreeMap::<&'static str, QuantumCapability>::new();

    for &capability in representative_capabilities() {
        let identifier = capability.as_str();

        if let Some(previous) = identifiers.insert(identifier, capability) {
            panic!(
                "duplicate capability identifier `{identifier}` for {:?} and {:?}",
                previous, capability
            );
        }
    }
}

#[test]
fn capability_display_matches_stable_identifier() {
    for &capability in representative_capabilities() {
        assert_eq!(
            capability.to_string(),
            capability.as_str(),
            "Display must remain the stable machine-readable identifier"
        );
    }
}

#[test]
fn capability_parse_round_trip_is_lossless() {
    for &capability in representative_capabilities() {
        let parsed = QuantumCapability::from_str(capability.as_str())
            .expect("every canonical capability identifier must parse");

        assert_eq!(
            parsed, capability,
            "parse(as_str()) must recover the exact capability"
        );
    }
}

#[test]
fn capability_parse_accepts_normalized_aliases_when_defined() {
    // The capability parser normalizes identifiers. Verify representative
    // canonical forms through the public FromStr contract.
    let cases = [
        (
            "measurement",
            QuantumCapability::Measurement,
        ),
        (
            "mid_circuit_measurement",
            QuantumCapability::MidCircuitMeasurement,
        ),
        (
            "two_qubit_gates",
            QuantumCapability::TwoQubitGates,
        ),
        (
            "logical_qubits",
            QuantumCapability::LogicalQubits,
        ),
        (
            "quantum_annealing",
            QuantumCapability::QuantumAnnealing,
        ),
        (
            "provider_extension",
            QuantumCapability::ProviderExtension,
        ),
    ];

    for (identifier, expected) in cases {
        let parsed = identifier
            .parse::<QuantumCapability>()
            .expect("canonical capability identifier must parse");

        assert_eq!(parsed, expected);
    }
}

#[test]
fn unknown_capability_identifier_is_rejected() {
    let error = "definitely_not_a_zamani_capability"
        .parse::<QuantumCapability>()
        .expect_err("unknown capability must be rejected");

    match error {
        CapabilityParseError::UnknownCapability { value } => {
            assert_eq!(value, "definitely_not_a_zamani_capability");
        }
        other => panic!("unexpected parse error: {other:?}"),
    }
}

#[test]
fn empty_capability_identifier_is_rejected() {
    let error = ""
        .parse::<QuantumCapability>()
        .expect_err("empty capability identifier must be rejected");

    assert!(
        matches!(error, CapabilityParseError::Empty),
        "expected Empty, got {error:?}"
    );
}

// =============================================================================
// Category tests
// =============================================================================

#[test]
fn representative_capabilities_have_meaningful_categories() {
    let cases = [
        (
            QuantumCapability::Measurement,
            CapabilityCategory::Measurement,
        ),
        (
            QuantumCapability::Reset,
            CapabilityCategory::StatePreparation,
        ),
        (
            QuantumCapability::TwoQubitGates,
            CapabilityCategory::GateModel,
        ),
        (
            QuantumCapability::DynamicCircuits,
            CapabilityCategory::ClassicalControl,
        ),
        (
            QuantumCapability::ParameterizedGates,
            CapabilityCategory::Parameterization,
        ),
        (
            QuantumCapability::TimingInformation,
            CapabilityCategory::Timing,
        ),
        (
            QuantumCapability::PulseLevelControl,
            CapabilityCategory::PulseControl,
        ),
        (
            QuantumCapability::AnalogExecution,
            CapabilityCategory::Analog,
        ),
        (
            QuantumCapability::QuantumAnnealing,
            CapabilityCategory::Annealing,
        ),
        (
            QuantumCapability::PhotonicModes,
            CapabilityCategory::Photonic,
        ),
        (
            QuantumCapability::LogicalQubits,
            CapabilityCategory::FaultTolerance,
        ),
        (
            QuantumCapability::SyndromeExtraction,
            CapabilityCategory::ErrorCorrection,
        ),
        (
            QuantumCapability::ParallelOperations,
            CapabilityCategory::Concurrency,
        ),
        (
            QuantumCapability::JobStatus,
            CapabilityCategory::Execution,
        ),
        (
            QuantumCapability::Counts,
            CapabilityCategory::Results,
        ),
        (
            QuantumCapability::CalibrationData,
            CapabilityCategory::Calibration,
        ),
        (
            QuantumCapability::TopologyInformation,
            CapabilityCategory::Topology,
        ),
        (
            QuantumCapability::StateVectorSimulation,
            CapabilityCategory::Simulation,
        ),
        (
            QuantumCapability::DistributedExecution,
            CapabilityCategory::Distributed,
        ),
        (
            QuantumCapability::ProviderExtension,
            CapabilityCategory::Custom,
        ),
    ];

    for (capability, expected_category) in cases {
        assert_eq!(
            capability.category(),
            expected_category,
            "incorrect category for {}",
            capability.as_str()
        );
    }
}

#[test]
fn every_representative_capability_has_a_category_identifier() {
    for &capability in representative_capabilities() {
        let category = capability.category();

        assert!(
            !category.as_str().is_empty(),
            "category identifier must not be empty for {}",
            capability.as_str()
        );
    }
}

#[test]
fn category_identifiers_are_canonical() {
    let categories = [
        CapabilityCategory::Measurement,
        CapabilityCategory::StatePreparation,
        CapabilityCategory::GateModel,
        CapabilityCategory::ClassicalControl,
        CapabilityCategory::Parameterization,
        CapabilityCategory::Timing,
        CapabilityCategory::PulseControl,
        CapabilityCategory::Analog,
        CapabilityCategory::Annealing,
        CapabilityCategory::Photonic,
        CapabilityCategory::FaultTolerance,
        CapabilityCategory::ErrorCorrection,
        CapabilityCategory::Concurrency,
        CapabilityCategory::Execution,
        CapabilityCategory::Results,
        CapabilityCategory::Calibration,
        CapabilityCategory::Topology,
        CapabilityCategory::Simulation,
        CapabilityCategory::Distributed,
        CapabilityCategory::Custom,
    ];

    let mut identifiers = BTreeSet::new();

    for category in categories {
        let identifier = category.as_str();

        assert!(
            identifiers.insert(identifier),
            "duplicate category identifier: {identifier}"
        );

        assert!(
            !identifier.is_empty(),
            "category identifier cannot be empty"
        );

        assert!(
            identifier
                .chars()
                .all(|character| character.is_ascii_lowercase()
                    || character.is_ascii_digit()
                    || character == '_'),
            "category identifier is not canonical: {identifier}"
        );

        let parsed = identifier
            .parse::<CapabilityCategory>()
            .expect("category identifier must parse");

        assert_eq!(parsed, category);
    }
}

// =============================================================================
// Explicit-opt-in classification
// =============================================================================

#[test]
fn high_risk_capabilities_require_explicit_opt_in() {
    let explicitly_opted_in = [
        QuantumCapability::PulseLevelControl,
        QuantumCapability::CustomWaveforms,
        QuantumCapability::AnalogExecution,
        QuantumCapability::QuantumAnnealing,
        QuantumCapability::LogicalQubits,
        QuantumCapability::FaultTolerantOperations,
        QuantumCapability::ProviderExtension,
        QuantumCapability::DistributedExecution,
        QuantumCapability::RemoteQuantumResources,
        QuantumCapability::RuntimeBranching,
        QuantumCapability::RuntimeLoops,
        QuantumCapability::RawMeasurementRecords,
        QuantumCapability::StateVector,
        QuantumCapability::DensityMatrix,
        QuantumCapability::Wavefunction,
    ];

    for capability in explicitly_opted_in {
        assert!(
            capability.requires_explicit_opt_in(),
            "{} must require explicit opt-in",
            capability.as_str()
        );
    }
}

#[test]
fn ordinary_capabilities_are_not_marked_as_explicit_opt_in_without_policy_reason() {
    let ordinary = [
        QuantumCapability::Measurement,
        QuantumCapability::Reset,
        QuantumCapability::SingleQubitGates,
        QuantumCapability::TwoQubitGates,
        QuantumCapability::ParameterizedGates,
        QuantumCapability::Counts,
        QuantumCapability::JobStatus,
        QuantumCapability::CalibrationData,
        QuantumCapability::TopologyInformation,
    ];

    for capability in ordinary {
        assert!(
            !capability.requires_explicit_opt_in(),
            "{} unexpectedly requires explicit opt-in",
            capability.as_str()
        );
    }
}

// =============================================================================
// Capability status tests
// =============================================================================

#[test]
fn capability_status_has_stable_serialized_identifiers() {
    let cases = [
        (CapabilityStatus::Stable, "stable"),
        (CapabilityStatus::Experimental, "experimental"),
        (CapabilityStatus::Deprecated, "deprecated"),
        (CapabilityStatus::Unavailable, "unavailable"),
    ];

    for (status, expected) in cases {
        assert_eq!(status.as_str(), expected);
        assert_eq!(status.to_string(), expected);

        let parsed = expected
            .parse::<CapabilityStatus>()
            .expect("status identifier must parse");

        assert_eq!(parsed, status);
    }
}

#[test]
fn capability_status_support_semantics_are_correct() {
    assert!(CapabilityStatus::Stable.is_supported());
    assert!(CapabilityStatus::Experimental.is_supported());

    assert!(!CapabilityStatus::Deprecated.is_supported());
    assert!(!CapabilityStatus::Unavailable.is_supported());

    assert!(CapabilityStatus::Stable.is_stable());
    assert!(CapabilityStatus::Experimental.is_experimental());
    assert!(CapabilityStatus::Deprecated.is_deprecated());
    assert!(CapabilityStatus::Unavailable.is_unavailable());
}

#[test]
fn capability_status_default_is_stable() {
    assert_eq!(
        CapabilityStatus::default(),
        CapabilityStatus::Stable
    );
}

// =============================================================================
// Capability policy tests
// =============================================================================

#[test]
fn stable_only_accepts_only_stable() {
    assert!(CapabilityPolicy::StableOnly.accepts(CapabilityStatus::Stable));

    assert!(!CapabilityPolicy::StableOnly.accepts(
        CapabilityStatus::Experimental
    ));

    assert!(!CapabilityPolicy::StableOnly.accepts(
        CapabilityStatus::Deprecated
    ));

    assert!(!CapabilityPolicy::StableOnly.accepts(
        CapabilityStatus::Unavailable
    ));
}

#[test]
fn stable_and_experimental_accepts_only_current_supported_states() {
    assert!(CapabilityPolicy::StableAndExperimental.accepts(
        CapabilityStatus::Stable
    ));

    assert!(CapabilityPolicy::StableAndExperimental.accepts(
        CapabilityStatus::Experimental
    ));

    assert!(!CapabilityPolicy::StableAndExperimental.accepts(
        CapabilityStatus::Deprecated
    ));

    assert!(!CapabilityPolicy::StableAndExperimental.accepts(
        CapabilityStatus::Unavailable
    ));
}

#[test]
fn include_deprecated_is_migration_policy_not_normal_execution_policy() {
    assert!(CapabilityPolicy::IncludeDeprecated.accepts(
        CapabilityStatus::Stable
    ));

    assert!(CapabilityPolicy::IncludeDeprecated.accepts(
        CapabilityStatus::Experimental
    ));

    assert!(CapabilityPolicy::IncludeDeprecated.accepts(
        CapabilityStatus::Deprecated
    ));

    assert!(!CapabilityPolicy::IncludeDeprecated.accepts(
        CapabilityStatus::Unavailable
    ));
}

#[test]
fn all_supported_never_accepts_unavailable() {
    assert!(CapabilityPolicy::AllSupported.accepts(
        CapabilityStatus::Stable
    ));

    assert!(CapabilityPolicy::AllSupported.accepts(
        CapabilityStatus::Experimental
    ));

    assert!(!CapabilityPolicy::AllSupported.accepts(
        CapabilityStatus::Deprecated
    ));

    assert!(!CapabilityPolicy::AllSupported.accepts(
        CapabilityStatus::Unavailable
    ));
}

#[test]
fn default_capability_policy_is_production_safe() {
    assert_eq!(
        CapabilityPolicy::default(),
        CapabilityPolicy::StableOnly
    );
}

// =============================================================================
// Descriptor tests
// =============================================================================

#[test]
fn stable_descriptor_has_expected_defaults() {
    let descriptor = CapabilityDescriptor::stable(
        QuantumCapability::Measurement,
    );

    assert_eq!(
        descriptor.capability,
        QuantumCapability::Measurement
    );

    assert_eq!(
        descriptor.status,
        CapabilityStatus::Stable
    );

    assert!(descriptor.description.is_none());
    assert!(descriptor.external_identifier.is_none());
    assert!(descriptor.is_supported());
}

#[test]
fn experimental_descriptor_is_not_stable_but_is_supported() {
    let descriptor = CapabilityDescriptor::experimental(
        QuantumCapability::DynamicCircuits,
    );

    assert_eq!(
        descriptor.status,
        CapabilityStatus::Experimental
    );

    assert!(descriptor.is_supported());
    assert!(descriptor.status.is_experimental());
    assert!(!descriptor.status.is_stable());
}

#[test]
fn descriptor_metadata_is_preserved() {
    let descriptor = CapabilityDescriptor::stable(
        QuantumCapability::Measurement,
    )
    .with_description("Terminal and mid-circuit measurement support")
    .with_external_identifier("provider.measurement.v1");

    assert_eq!(
        descriptor.description.as_deref(),
        Some("Terminal and mid-circuit measurement support")
    );

    assert_eq!(
        descriptor.external_identifier.as_deref(),
        Some("provider.measurement.v1")
    );
}

#[test]
fn descriptor_builder_does_not_change_capability_identity() {
    let descriptor = CapabilityDescriptor::experimental(
        QuantumCapability::PulseLevelControl,
    )
    .with_description("Provider experimental pulse interface")
    .with_external_identifier("pulse.v2");

    assert_eq!(
        descriptor.capability,
        QuantumCapability::PulseLevelControl
    );

    assert_eq!(
        descriptor.status,
        CapabilityStatus::Experimental
    );
}

// =============================================================================
// CapabilitySet tests
// =============================================================================

#[test]
fn empty_capability_set_is_empty() {
    let set = CapabilitySet::new();

    assert!(set.is_empty());
    assert_eq!(set.len(), 0);
    assert_eq!(set.capabilities().count(), 0);
}

#[test]
fn inserting_capability_creates_stable_descriptor() {
    let mut set = CapabilitySet::new();

    set.insert(QuantumCapability::Measurement);

    assert_eq!(set.len(), 1);
    assert!(set.contains(QuantumCapability::Measurement));

    let descriptor = set
        .descriptor(QuantumCapability::Measurement)
        .expect("inserted capability must have descriptor");

    assert_eq!(descriptor.status, CapabilityStatus::Stable);
    assert!(set.supports(
        QuantumCapability::Measurement,
        CapabilityPolicy::StableOnly
    ));
}

#[test]
fn inserting_same_capability_replaces_previous_descriptor() {
    let mut set = CapabilitySet::new();

    set.insert_experimental(QuantumCapability::DynamicCircuits);

    assert_eq!(
        set.descriptor(QuantumCapability::DynamicCircuits)
            .expect("descriptor must exist")
            .status,
        CapabilityStatus::Experimental
    );

    set.insert(QuantumCapability::DynamicCircuits);

    assert_eq!(
        set.descriptor(QuantumCapability::DynamicCircuits)
            .expect("descriptor must exist")
            .status,
        CapabilityStatus::Stable
    );
}

#[test]
fn insert_descriptor_preserves_explicit_lifecycle_state() {
    let mut set = CapabilitySet::new();

    let descriptor = CapabilityDescriptor {
        capability: QuantumCapability::DynamicCircuits,
        status: CapabilityStatus::Experimental,
        description: Some("Experimental dynamic execution".to_owned()),
        external_identifier: Some("provider.dynamic.v1".to_owned()),
    };

    set.insert_descriptor(descriptor.clone());

    assert_eq!(
        set.descriptor(QuantumCapability::DynamicCircuits),
        Some(&descriptor)
    );
}

#[test]
fn remove_returns_removed_descriptor() {
    let mut set = CapabilitySet::new();

    set.insert(QuantumCapability::Measurement);

    let removed = set
        .remove(QuantumCapability::Measurement)
        .expect("existing capability must be removable");

    assert_eq!(
        removed.capability,
        QuantumCapability::Measurement
    );

    assert!(!set.contains(QuantumCapability::Measurement));
    assert!(set.is_empty());
}

#[test]
fn removing_unknown_capability_returns_none() {
    let mut set = CapabilitySet::new();

    assert!(
        set.remove(QuantumCapability::Measurement).is_none()
    );
}

#[test]
fn from_capabilities_creates_stable_capabilities() {
    let set = CapabilitySet::from_capabilities([
        QuantumCapability::Measurement,
        QuantumCapability::Reset,
        QuantumCapability::TwoQubitGates,
    ]);

    assert_eq!(set.len(), 3);

    for capability in [
        QuantumCapability::Measurement,
        QuantumCapability::Reset,
        QuantumCapability::TwoQubitGates,
    ] {
        assert!(set.supports(
            capability,
            CapabilityPolicy::StableOnly
        ));
    }
}

#[test]
fn capabilities_are_returned_in_deterministic_enum_order() {
    let set = CapabilitySet::from_capabilities([
        QuantumCapability::TwoQubitGates,
        QuantumCapability::Measurement,
        QuantumCapability::Reset,
        QuantumCapability::DynamicCircuits,
    ]);

    let values = set.capabilities().collect::<Vec<_>>();

    let mut sorted = values.clone();
    sorted.sort();

    assert_eq!(
        values, sorted,
        "CapabilitySet must expose deterministic ordering"
    );
}

#[test]
fn to_set_matches_advertised_capabilities() {
    let set = CapabilitySet::from_capabilities([
        QuantumCapability::Measurement,
        QuantumCapability::Reset,
        QuantumCapability::DynamicCircuits,
    ]);

    let expected = BTreeSet::from([
        QuantumCapability::Measurement,
        QuantumCapability::Reset,
        QuantumCapability::DynamicCircuits,
    ]);

    assert_eq!(set.to_set(), expected);
}

#[test]
fn supported_filters_by_policy() {
    let mut set = CapabilitySet::new();

    set.insert(QuantumCapability::Measurement);
    set.insert_experimental(QuantumCapability::DynamicCircuits);

    set.insert_descriptor(CapabilityDescriptor {
        capability: QuantumCapability::Reset,
        status: CapabilityStatus::Deprecated,
        description: None,
        external_identifier: None,
    });

    assert_eq!(
        set.supported(CapabilityPolicy::StableOnly),
        vec![QuantumCapability::Measurement]
    );

    assert_eq!(
        set.supported(CapabilityPolicy::StableAndExperimental),
        vec![
            QuantumCapability::Measurement,
            QuantumCapability::DynamicCircuits
        ]
    );
}

#[test]
fn experimental_returns_only_experimental_capabilities() {
    let mut set = CapabilitySet::new();

    set.insert(QuantumCapability::Measurement);
    set.insert_experimental(QuantumCapability::DynamicCircuits);

    set.insert_descriptor(CapabilityDescriptor {
        capability: QuantumCapability::Reset,
        status: CapabilityStatus::Deprecated,
        description: None,
        external_identifier: None,
    });

    assert_eq!(
        set.experimental(),
        vec![QuantumCapability::DynamicCircuits]
    );
}

#[test]
fn deprecated_returns_only_deprecated_capabilities() {
    let mut set = CapabilitySet::new();

    set.insert(QuantumCapability::Measurement);

    set.insert_descriptor(CapabilityDescriptor {
        capability: QuantumCapability::Reset,
        status: CapabilityStatus::Deprecated,
        description: None,
        external_identifier: None,
    });

    set.insert_experimental(QuantumCapability::DynamicCircuits);

    assert_eq!(
        set.deprecated(),
        vec![QuantumCapability::Reset]
    );
}

#[test]
fn unavailable_returns_only_unavailable_capabilities() {
    let mut set = CapabilitySet::new();

    set.insert(QuantumCapability::Measurement);

    set.insert_descriptor(CapabilityDescriptor {
        capability: QuantumCapability::Reset,
        status: CapabilityStatus::Unavailable,
        description: None,
        external_identifier: None,
    });

    assert_eq!(
        set.unavailable(),
        vec![QuantumCapability::Reset]
    );
}

#[test]
fn by_category_returns_only_matching_capabilities() {
    let set = CapabilitySet::from_capabilities([
        QuantumCapability::Measurement,
        QuantumCapability::MidCircuitMeasurement,
        QuantumCapability::Reset,
        QuantumCapability::TwoQubitGates,
    ]);

    assert_eq!(
        set.by_category(CapabilityCategory::Measurement),
        vec![
            QuantumCapability::Measurement,
            QuantumCapability::MidCircuitMeasurement
        ]
    );

    assert_eq!(
        set.by_category(CapabilityCategory::StatePreparation),
        vec![QuantumCapability::Reset]
    );

    assert_eq!(
        set.by_category(CapabilityCategory::GateModel),
        vec![QuantumCapability::TwoQubitGates]
    );
}

// =============================================================================
// CapabilityRequirements tests
// =============================================================================

#[test]
fn empty_requirements_are_empty() {
    let requirements = CapabilityRequirements::new();

    assert!(requirements.is_empty());
    assert!(requirements.required.is_empty());
    assert!(requirements.preferred.is_empty());
    assert_eq!(
        requirements.policy,
        CapabilityPolicy::StableOnly
    );
}

#[test]
fn requirement_builder_records_required_capabilities() {
    let requirements = CapabilityRequirements::new()
        .require(QuantumCapability::Measurement)
        .require(QuantumCapability::Reset)
        .require_all([
            QuantumCapability::TwoQubitGates,
            QuantumCapability::DynamicCircuits,
        ]);

    assert!(requirements.requires(QuantumCapability::Measurement));
    assert!(requirements.requires(QuantumCapability::Reset));
    assert!(requirements.requires(QuantumCapability::TwoQubitGates));
    assert!(
        requirements.requires(QuantumCapability::DynamicCircuits)
    );

    assert_eq!(requirements.required.len(), 4);
}

#[test]
fn preferred_capabilities_are_distinct_from_required_capabilities() {
    let requirements = CapabilityRequirements::new()
        .require(QuantumCapability::Measurement)
        .prefer(QuantumCapability::DynamicCircuits)
        .prefer_all([
            QuantumCapability::ParallelOperations,
            QuantumCapability::QueueInformation,
        ]);

    assert!(requirements.requires(QuantumCapability::Measurement));
    assert!(!requirements.requires(QuantumCapability::DynamicCircuits));

    assert!(requirements.prefers(QuantumCapability::DynamicCircuits));
    assert!(requirements.prefers(QuantumCapability::ParallelOperations));
    assert!(requirements.prefers(QuantumCapability::QueueInformation));
}

#[test]
fn requirement_policy_can_be_changed_explicitly() {
    let requirements = CapabilityRequirements::new()
        .with_policy(CapabilityPolicy::StableAndExperimental);

    assert_eq!(
        requirements.policy,
        CapabilityPolicy::StableAndExperimental
    );
}

// =============================================================================
// Capability negotiation tests
// =============================================================================

#[test]
fn compatible_requirements_produce_successful_check() {
    let hardware = CapabilitySet::from_capabilities([
        QuantumCapability::Measurement,
        QuantumCapability::Reset,
        QuantumCapability::TwoQubitGates,
    ]);

    let requirements = CapabilityRequirements::new()
        .require(QuantumCapability::Measurement)
        .require(QuantumCapability::Reset)
        .require(QuantumCapability::TwoQubitGates);

    let result = hardware.check(
        &requirements,
        CapabilityPolicy::StableOnly,
    );

    assert!(result.compatible);
    assert!(result.missing.is_empty());
    assert!(result.rejected_status.is_empty());
}

#[test]
fn missing_required_capability_makes_check_incompatible() {
    let hardware = CapabilitySet::from_capabilities([
        QuantumCapability::Measurement,
        QuantumCapability::Reset,
    ]);

    let requirements = CapabilityRequirements::new()
        .require(QuantumCapability::Measurement)
        .require(QuantumCapability::DynamicCircuits);

    let result = hardware.check(
        &requirements,
        CapabilityPolicy::StableOnly,
    );

    assert!(!result.compatible);

    assert!(
        result
            .missing
            .contains(&QuantumCapability::DynamicCircuits)
    );

    assert!(result.rejected_status.is_empty());
}

#[test]
fn experimental_capability_is_rejected_by_stable_only_policy() {
    let mut hardware = CapabilitySet::new();

    hardware.insert(QuantumCapability::Measurement);
    hardware.insert_experimental(QuantumCapability::DynamicCircuits);

    let requirements = CapabilityRequirements::new()
        .require(QuantumCapability::DynamicCircuits);

    let result = hardware.check(
        &requirements,
        CapabilityPolicy::StableOnly,
    );

    assert!(!result.compatible);
    assert!(result.missing.is_empty());

    assert_eq!(
        result.rejected_status.get(
            &QuantumCapability::DynamicCircuits
        ),
        Some(&CapabilityStatus::Experimental)
    );
}

#[test]
fn experimental_capability_can_be_accepted_explicitly() {
    let mut hardware = CapabilitySet::new();

    hardware.insert_experimental(
        QuantumCapability::DynamicCircuits,
    );

    let requirements = CapabilityRequirements::new()
        .require(QuantumCapability::DynamicCircuits);

    let result = hardware.check(
        &requirements,
        CapabilityPolicy::StableAndExperimental,
    );

    assert!(result.compatible);
    assert!(result.missing.is_empty());
    assert!(result.rejected_status.is_empty());
}

#[test]
fn deprecated_capability_is_not_accepted_by_normal_policy() {
    let mut hardware = CapabilitySet::new();

    hardware.insert_descriptor(CapabilityDescriptor {
        capability: QuantumCapability::Reset,
        status: CapabilityStatus::Deprecated,
        description: None,
        external_identifier: None,
    });

    let requirements = CapabilityRequirements::new()
        .require(QuantumCapability::Reset);

    let normal = hardware.check(
        &requirements,
        CapabilityPolicy::StableOnly,
    );

    assert!(!normal.compatible);

    let migration = hardware.check(
        &requirements,
        CapabilityPolicy::IncludeDeprecated,
    );

    assert!(migration.compatible);
}

#[test]
fn unavailable_capability_is_never_accepted() {
    let mut hardware = CapabilitySet::new();

    hardware.insert_descriptor(CapabilityDescriptor {
        capability: QuantumCapability::Reset,
        status: CapabilityStatus::Unavailable,
        description: None,
        external_identifier: None,
    });

    let requirements = CapabilityRequirements::new()
        .require(QuantumCapability::Reset);

    for policy in [
        CapabilityPolicy::StableOnly,
        CapabilityPolicy::StableAndExperimental,
        CapabilityPolicy::IncludeDeprecated,
        CapabilityPolicy::AllSupported,
    ] {
        let result = hardware.check(&requirements, policy);

        assert!(
            !result.compatible,
            "unavailable capability must not satisfy {:?}",
            policy
        );

        assert_eq!(
            result.rejected_status.get(
                &QuantumCapability::Reset
            ),
            Some(&CapabilityStatus::Unavailable)
        );
    }
}

#[test]
fn preferred_missing_capability_is_warning_not_failure() {
    let hardware = CapabilitySet::from_capabilities([
        QuantumCapability::Measurement,
    ]);

    let requirements = CapabilityRequirements::new()
        .require(QuantumCapability::Measurement)
        .prefer(QuantumCapability::DynamicCircuits);

    let result = hardware.check(
        &requirements,
        CapabilityPolicy::StableOnly,
    );

    assert!(result.compatible);
    assert!(result.missing.is_empty());
    assert!(result.rejected_status.is_empty());

    assert!(
        !result.warnings.is_empty(),
        "missing preferred capability must produce warning"
    );
}

#[test]
fn required_explicit_opt_in_capability_produces_warning() {
    let hardware = CapabilitySet::from_capabilities([
        QuantumCapability::AnalogExecution,
    ]);

    let requirements = CapabilityRequirements::new()
        .require(QuantumCapability::AnalogExecution);

    let result = hardware.check(
        &requirements,
        CapabilityPolicy::StableOnly,
    );

    assert!(result.compatible);

    assert!(
        !result.warnings.is_empty(),
        "explicit-opt-in capabilities must produce an opt-in warning"
    );
}

#[test]
fn capability_check_is_deterministic() {
    let hardware = CapabilitySet::from_capabilities([
        QuantumCapability::Measurement,
        QuantumCapability::Reset,
        QuantumCapability::TwoQubitGates,
    ]);

    let requirements = CapabilityRequirements::new()
        .require(QuantumCapability::DynamicCircuits)
        .require(QuantumCapability::Measurement)
        .require(QuantumCapability::LogicalQubits)
        .prefer(QuantumCapability::PulseLevelControl)
        .prefer(QuantumCapability::Reset);

    let first = hardware.check(
        &requirements,
        CapabilityPolicy::StableOnly,
    );

    let second = hardware.check(
        &requirements,
        CapabilityPolicy::StableOnly,
    );

    assert_eq!(first, second);
}

// =============================================================================
// Merge tests
// =============================================================================

#[test]
fn merge_adds_new_capabilities() {
    let mut first = CapabilitySet::from_capabilities([
        QuantumCapability::Measurement,
    ]);

    let second = CapabilitySet::from_capabilities([
        QuantumCapability::Reset,
        QuantumCapability::TwoQubitGates,
    ]);

    first.merge(&second);

    assert_eq!(first.len(), 3);

    assert!(first.contains(QuantumCapability::Measurement));
    assert!(first.contains(QuantumCapability::Reset));
    assert!(first.contains(QuantumCapability::TwoQubitGates));
}

#[test]
fn merge_replaces_existing_descriptor_explicitly() {
    let mut first = CapabilitySet::new();

    first.insert(QuantumCapability::DynamicCircuits);

    let mut second = CapabilitySet::new();

    second.insert_descriptor(CapabilityDescriptor {
        capability: QuantumCapability::DynamicCircuits,
        status: CapabilityStatus::Experimental,
        description: Some("experimental".to_owned()),
        external_identifier: Some("provider.dynamic".to_owned()),
    });

    first.merge(&second);

    let descriptor = first
        .descriptor(QuantumCapability::DynamicCircuits)
        .expect("merged descriptor must exist");

    assert_eq!(
        descriptor.status,
        CapabilityStatus::Experimental
    );

    assert_eq!(
        descriptor.description.as_deref(),
        Some("experimental")
    );

    assert_eq!(
        descriptor.external_identifier.as_deref(),
        Some("provider.dynamic")
    );
}

#[test]
fn merging_a_set_with_itself_is_idempotent() {
    let mut set = CapabilitySet::new();

    set.insert(QuantumCapability::Measurement);

    set.insert_experimental(QuantumCapability::DynamicCircuits);

    let before = set.clone();

    set.merge(&before);

    assert_eq!(set, before);
}

// =============================================================================
// Serialization tests
// =============================================================================

#[test]
fn capability_status_serializes_as_stable_string() {
    let serialized = serde_json::to_string(
        &CapabilityStatus::Experimental,
    )
    .expect("status must serialize");

    assert_eq!(serialized, "\"experimental\"");
}

#[test]
fn capability_serializes_as_stable_identifier() {
    let serialized = serde_json::to_string(
        &QuantumCapability::MidCircuitMeasurement,
    )
    .expect("capability must serialize");

    assert_eq!(
        serialized,
        "\"mid_circuit_measurement\""
    );
}

#[test]
fn capability_deserializes_from_stable_identifier() {
    let capability: QuantumCapability =
        serde_json::from_str("\"two_qubit_gates\"")
            .expect("capability must deserialize");

    assert_eq!(
        capability,
        QuantumCapability::TwoQubitGates
    );
}

#[test]
fn capability_status_deserializes_from_stable_identifier() {
    let status: CapabilityStatus =
        serde_json::from_str("\"stable\"")
            .expect("status must deserialize");

    assert_eq!(status, CapabilityStatus::Stable);
}

#[test]
fn capability_set_json_round_trip_is_lossless() {
    let mut original = CapabilitySet::new();

    original.insert(QuantumCapability::Measurement);

    original.insert_experimental(
        QuantumCapability::DynamicCircuits,
    );

    original.insert_descriptor(
        CapabilityDescriptor {
            capability: QuantumCapability::Reset,
            status: CapabilityStatus::Deprecated,
            description: Some(
                "legacy reset contract".to_owned(),
            ),
            external_identifier: Some(
                "provider.reset.v1".to_owned(),
            ),
        },
    );

    let json = serde_json::to_string(&original)
        .expect("capability set must serialize");

    let restored: CapabilitySet =
        serde_json::from_str(&json)
            .expect("capability set must deserialize");

    assert_eq!(restored, original);
}

#[test]
fn capability_requirements_json_round_trip_is_lossless() {
    let original = CapabilityRequirements::new()
        .require(QuantumCapability::Measurement)
        .require(QuantumCapability::TwoQubitGates)
        .prefer(QuantumCapability::DynamicCircuits)
        .prefer(QuantumCapability::QueueInformation)
        .with_policy(CapabilityPolicy::StableAndExperimental);

    let json = serde_json::to_string(&original)
        .expect("requirements must serialize");

    let restored: CapabilityRequirements =
        serde_json::from_str(&json)
            .expect("requirements must deserialize");

    assert_eq!(restored, original);
}

#[test]
fn capability_set_serialization_is_deterministic() {
    let mut first = CapabilitySet::new();

    first.insert(QuantumCapability::TwoQubitGates);
    first.insert(QuantumCapability::Measurement);
    first.insert(QuantumCapability::Reset);

    let mut second = CapabilitySet::new();

    second.insert(QuantumCapability::Reset);
    second.insert(QuantumCapability::Measurement);
    second.insert(QuantumCapability::TwoQubitGates);

    let first_json = serde_json::to_string(&first)
        .expect("first set must serialize");

    let second_json = serde_json::to_string(&second)
        .expect("second set must serialize");

    assert_eq!(
        first_json, second_json,
        "capability serialization must be deterministic"
    );
}

#[test]
fn serialized_capability_set_is_object_with_capability_entries() {
    let set = CapabilitySet::from_capabilities([
        QuantumCapability::Measurement,
        QuantumCapability::Reset,
    ]);

    let json = serde_json::to_value(&set)
        .expect("capability set must serialize");

    let object = json
        .as_object()
        .expect("serialized capability set must be an object");

    assert!(
        object.contains_key("capabilities"),
        "serialized contract must contain capabilities field"
    );
}

#[test]
fn malformed_capability_json_is_rejected() {
    let result = serde_json::from_str::<CapabilitySet>(
        r#"{"capabilities":{"not_a_real_capability":{"capability":"not_a_real_capability","status":"stable"}}}"#,
    );

    assert!(
        result.is_err(),
        "unknown capability identifiers must not silently deserialize"
    );
}

#[test]
fn malformed_capability_status_json_is_rejected() {
    let result = serde_json::from_str::<CapabilityStatus>(
        "\"not_a_real_status\"",
    );

    assert!(
        result.is_err(),
        "unknown capability statuses must not silently deserialize"
    );
}

// =============================================================================
// Metadata safety tests
// =============================================================================

#[test]
fn capability_external_identifier_is_metadata_not_capability_identity() {
    let descriptor = CapabilityDescriptor::stable(
        QuantumCapability::Measurement,
    )
    .with_external_identifier(
        "provider-specific-measurement-v9",
    );

    assert_eq!(
        descriptor.capability,
        QuantumCapability::Measurement
    );

    assert_eq!(
        descriptor.external_identifier.as_deref(),
        Some("provider-specific-measurement-v9")
    );
}

#[test]
fn descriptor_round_trip_preserves_non_secret_metadata() {
    let descriptor = CapabilityDescriptor::experimental(
        QuantumCapability::PulseLevelControl,
    )
    .with_description("experimental pulse support")
    .with_external_identifier("provider.pulse.v2");

    let json = serde_json::to_string(&descriptor)
        .expect("descriptor must serialize");

    let restored: CapabilityDescriptor =
        serde_json::from_str(&json)
            .expect("descriptor must deserialize");

    assert_eq!(restored, descriptor);
}

#[test]
fn serialized_descriptor_does_not_create_an_implicit_secret_field() {
    let descriptor = CapabilityDescriptor::stable(
        QuantumCapability::Measurement,
    );

    let value = serde_json::to_value(&descriptor)
        .expect("descriptor must serialize");

    let object = value
        .as_object()
        .expect("descriptor must serialize as object");

    for forbidden in [
        "api_key",
        "access_token",
        "token",
        "password",
        "private_key",
        "secret",
        "authorization",
        "cookie",
    ] {
        assert!(
            !object.contains_key(forbidden),
            "capability descriptor must not expose secret field `{forbidden}`"
        );
    }
}

// =============================================================================
// Cross-domain capability coverage
// =============================================================================

#[test]
fn capability_vocabulary_covers_gate_model_hardware() {
    let required = [
        QuantumCapability::SingleQubitGates,
        QuantumCapability::TwoQubitGates,
        QuantumCapability::Measurement,
        QuantumCapability::Reset,
    ];

    for capability in required {
        assert!(
            representative_capabilities().contains(&capability),
            "gate-model capability missing from conformance vocabulary"
        );
    }
}

#[test]
fn capability_vocabulary_covers_dynamic_circuit_hardware() {
    let required = [
        QuantumCapability::MidCircuitMeasurement,
        QuantumCapability::ClassicalFeedForward,
        QuantumCapability::DynamicCircuits,
        QuantumCapability::MidCircuitReset,
    ];

    for capability in required {
        assert!(
            representative_capabilities().contains(&capability),
            "dynamic-circuit capability missing"
        );
    }
}

#[test]
fn capability_vocabulary_covers_pulse_hardware() {
    let required = [
        QuantumCapability::PulseLevelControl,
        QuantumCapability::CustomWaveforms,
        QuantumCapability::DriveChannels,
        QuantumCapability::MeasureChannels,
        QuantumCapability::AcquireChannels,
        QuantumCapability::PulseSchedules,
    ];

    for capability in required {
        assert!(
            representative_capabilities().contains(&capability),
            "pulse capability missing"
        );
    }
}

#[test]
fn capability_vocabulary_covers_analog_hardware() {
    let required = [
        QuantumCapability::AnalogExecution,
        QuantumCapability::TimeDependentHamiltonians,
        QuantumCapability::SpatialHamiltonians,
        QuantumCapability::AnalogControlFields,
        QuantumCapability::AnalogObservables,
    ];

    for capability in required {
        assert!(
            representative_capabilities().contains(&capability),
            "analog capability missing"
        );
    }
}

#[test]
fn capability_vocabulary_covers_annealing_hardware() {
    let required = [
        QuantumCapability::QuantumAnnealing,
        QuantumCapability::IsingModels,
        QuantumCapability::Qubo,
        QuantumCapability::AnnealingSchedules,
    ];

    for capability in required {
        assert!(
            representative_capabilities().contains(&capability),
            "annealing capability missing"
        );
    }
}

#[test]
fn capability_vocabulary_covers_photonic_and_bosonic_hardware() {
    let required = [
        QuantumCapability::PhotonicModes,
        QuantumCapability::BosonicOperations,
        QuantumCapability::ContinuousVariableOperations,
        QuantumCapability::PhotonNumberMeasurement,
        QuantumCapability::HomodyneMeasurement,
        QuantumCapability::HeterodyneMeasurement,
    ];

    for capability in required {
        assert!(
            representative_capabilities().contains(&capability),
            "photonic/bosonic capability missing"
        );
    }
}

#[test]
fn capability_vocabulary_covers_logical_and_fault_tolerant_hardware() {
    let required = [
        QuantumCapability::LogicalQubits,
        QuantumCapability::LogicalGates,
        QuantumCapability::LogicalMeasurements,
        QuantumCapability::ErrorCorrectionCodes,
        QuantumCapability::SyndromeExtraction,
        QuantumCapability::DecoderExecution,
        QuantumCapability::FaultTolerantOperations,
        QuantumCapability::TransversalOperations,
        QuantumCapability::MagicStateSupport,
    ];

    for capability in required {
        assert!(
            representative_capabilities().contains(&capability),
            "logical/FTQC capability missing"
        );
    }
}

#[test]
fn capability_vocabulary_covers_simulation_and_emulation() {
    let required = [
        QuantumCapability::StateVectorSimulation,
        QuantumCapability::StabilizerSimulation,
        QuantumCapability::TensorNetworkSimulation,
        QuantumCapability::DensityMatrixSimulation,
        QuantumCapability::TrajectorySimulation,
        QuantumCapability::NoisySimulation,
        QuantumCapability::DeterministicSimulation,
        QuantumCapability::HardwareEmulation,
        QuantumCapability::FaultInjection,
    ];

    for capability in required {
        assert!(
            representative_capabilities().contains(&capability),
            "simulation/emulation capability missing"
        );
    }
}

#[test]
fn capability_vocabulary_covers_distributed_quantum() {
    let required = [
        QuantumCapability::DistributedExecution,
        QuantumCapability::RemoteQuantumResources,
        QuantumCapability::QuantumNetworkLinks,
        QuantumCapability::EntanglementResources,
        QuantumCapability::RemoteOperations,
        QuantumCapability::Teleportation,
    ];

    for capability in required {
        assert!(
            representative_capabilities().contains(&capability),
            "distributed capability missing"
        );
    }
}

#[test]
fn capability_vocabulary_covers_production_execution_lifecycle() {
    let required = [
        QuantumCapability::AsynchronousExecution,
        QuantumCapability::SynchronousExecution,
        QuantumCapability::JobStatus,
        QuantumCapability::JobCancellation,
        QuantumCapability::JobTimeout,
        QuantumCapability::RetryableExecution,
        QuantumCapability::ExecutionMetadata,
        QuantumCapability::CostEstimation,
    ];

    for capability in required {
        assert!(
            representative_capabilities().contains(&capability),
            "execution lifecycle capability missing"
        );
    }
}

// =============================================================================
// Provider-independence invariants
// =============================================================================

#[test]
fn capability_identifiers_do_not_encode_known_provider_names() {
    let provider_names = [
        "ibm",
        "ionq",
        "aws",
        "braket",
        "rigetti",
        "iqm",
        "quantinuum",
        "quera",
    ];

    for &capability in representative_capabilities() {
        let identifier = capability.as_str();

        for provider in provider_names {
            assert!(
                !identifier.contains(provider),
                "core capability identifier `{identifier}` must not encode provider `{provider}`"
            );
        }
    }
}

#[test]
fn capability_model_does_not_require_provider_specific_types() {
    // This is intentionally a compile-time architectural test expressed
    // through the fact that all operations below use only provider-neutral
    // types from capabilities.rs.
    let mut hardware = CapabilitySet::new();

    hardware.insert(QuantumCapability::Measurement);
    hardware.insert_experimental(
        QuantumCapability::DynamicCircuits,
    );

    let requirements = CapabilityRequirements::new()
        .require(QuantumCapability::Measurement)
        .prefer(QuantumCapability::DynamicCircuits);

    let result = hardware.check(
        &requirements,
        CapabilityPolicy::StableAndExperimental,
    );

    assert!(result.compatible);
}

// =============================================================================
// Stable API contract tests
// =============================================================================

#[test]
fn default_capability_set_is_equivalent_to_new() {
    assert_eq!(
        CapabilitySet::default(),
        CapabilitySet::new()
    );
}

#[test]
fn default_requirements_are_equivalent_to_new() {
    assert_eq!(
        CapabilityRequirements::default(),
        CapabilityRequirements::new()
    );
}

#[test]
fn stable_capability_descriptor_is_supported() {
    for &capability in representative_capabilities() {
        let descriptor = CapabilityDescriptor::stable(capability);

        assert!(
            descriptor.is_supported(),
            "stable capability must be supported: {}",
            capability.as_str()
        );

        assert_eq!(
            descriptor.status,
            CapabilityStatus::Stable
        );
    }
}

#[test]
fn capability_set_contains_exactly_inserted_capabilities() {
    let capabilities = [
        QuantumCapability::Measurement,
        QuantumCapability::Reset,
        QuantumCapability::TwoQubitGates,
        QuantumCapability::Counts,
    ];

    let set = CapabilitySet::from_capabilities(capabilities);

    assert_eq!(
        set.to_set(),
        capabilities.into_iter().collect()
    );
}

#[test]
fn capability_requirements_are_deterministically_ordered() {
    let requirements = CapabilityRequirements::new()
        .require(QuantumCapability::TwoQubitGates)
        .require(QuantumCapability::Measurement)
        .require(QuantumCapability::Reset)
        .prefer(QuantumCapability::QueueInformation)
        .prefer(QuantumCapability::DynamicCircuits);

    let required: Vec<_> =
        requirements.required.iter().copied().collect();

    let preferred: Vec<_> =
        requirements.preferred.iter().copied().collect();

    let mut sorted_required = required.clone();
    let mut sorted_preferred = preferred.clone();

    sorted_required.sort();
    sorted_preferred.sort();

    assert_eq!(required, sorted_required);
    assert_eq!(preferred, sorted_preferred);
}

// =============================================================================
// Regression tests for important semantic distinctions
// =============================================================================

#[test]
fn capability_is_not_instruction() {
    // The core capability model deliberately identifies a capability rather
    // than a provider/native instruction.
    //
    // `Measurement` describes an ability. A future instruction_set.rs module
    // owns concrete instructions such as `measure`, `mz`, or provider-native
    // equivalents.
    assert_eq!(
        QuantumCapability::Measurement.as_str(),
        "measurement"
    );

    assert_ne!(
        QuantumCapability::Measurement.as_str(),
        "measure"
    );
}

#[test]
fn capability_is_not_technology() {
    // Technology belongs to technology.rs. The capability vocabulary must
    // remain independent of physical implementation technology.
    assert_eq!(
        QuantumCapability::TwoQubitGates.category(),
        CapabilityCategory::GateModel
    );
}

#[test]
fn experimental_capability_never_satisfies_stable_policy_implicitly() {
    let mut hardware = CapabilitySet::new();

    hardware.insert_experimental(
        QuantumCapability::DynamicCircuits,
    );

    let requirements = CapabilityRequirements::new()
        .require(QuantumCapability::DynamicCircuits);

    let result = hardware.check(
        &requirements,
        requirements.policy,
    );

    assert!(
        !result.compatible,
        "production-default StableOnly policy must reject experimental capability"
    );
}

#[test]
fn preferred_capability_never_becomes_required_implicitly() {
    let hardware = CapabilitySet::from_capabilities([
        QuantumCapability::Measurement,
    ]);

    let requirements = CapabilityRequirements::new()
        .require(QuantumCapability::Measurement)
        .prefer(QuantumCapability::DynamicCircuits);

    let result = hardware.check(
        &requirements,
        CapabilityPolicy::StableOnly,
    );

    assert!(
        result.compatible,
        "preferred capability absence must not make execution incompatible"
    );
}

#[test]
fn capability_requirement_check_has_no_hidden_state() {
    let hardware = CapabilitySet::from_capabilities([
        QuantumCapability::Measurement,
        QuantumCapability::Reset,
    ]);

    let requirements = CapabilityRequirements::new()
        .require(QuantumCapability::Measurement)
        .prefer(QuantumCapability::DynamicCircuits);

    let first = hardware.check(
        &requirements,
        CapabilityPolicy::StableOnly,
    );

    let second = hardware.check(
        &requirements,
        CapabilityPolicy::StableOnly,
    );

    assert_eq!(first, second);
}

// =============================================================================
// Serialization schema sanity
// =============================================================================

#[test]
fn capability_serialization_uses_string_not_numeric_enum_encoding() {
    let value = serde_json::to_value(
        QuantumCapability::Measurement,
    )
    .expect("capability must serialize");

    assert_eq!(
        value,
        Value::String("measurement".to_owned())
    );
}

#[test]
fn capability_category_serialization_uses_string_identifier() {
    let value = serde_json::to_value(
        CapabilityCategory::GateModel,
    )
    .expect("category must serialize");

    assert_eq!(
        value,
        Value::String("gate_model".to_owned())
    );
}

#[test]
fn capability_policy_serialization_is_stable() {
    let cases = [
        (
            CapabilityPolicy::StableOnly,
            "\"stable_only\"",
        ),
        (
            CapabilityPolicy::StableAndExperimental,
            "\"stable_and_experimental\"",
        ),
        (
            CapabilityPolicy::IncludeDeprecated,
            "\"include_deprecated\"",
        ),
        (
            CapabilityPolicy::AllSupported,
            "\"all_supported\"",
        ),
    ];

    for (policy, expected_json) in cases {
        let actual = serde_json::to_string(&policy)
            .expect("policy must serialize");

        assert_eq!(actual, expected_json);
    }
}

// =============================================================================
// Final architectural invariants
// =============================================================================

#[test]
fn capability_model_supports_all_required_zamani_hardware_domains() {
    let required_domains = [
        CapabilityCategory::Measurement,
        CapabilityCategory::StatePreparation,
        CapabilityCategory::GateModel,
        CapabilityCategory::ClassicalControl,
        CapabilityCategory::Parameterization,
        CapabilityCategory::Timing,
        CapabilityCategory::PulseControl,
        CapabilityCategory::Analog,
        CapabilityCategory::Annealing,
        CapabilityCategory::Photonic,
        CapabilityCategory::FaultTolerance,
        CapabilityCategory::ErrorCorrection,
        CapabilityCategory::Concurrency,
        CapabilityCategory::Execution,
        CapabilityCategory::Results,
        CapabilityCategory::Calibration,
        CapabilityCategory::Topology,
        CapabilityCategory::Simulation,
        CapabilityCategory::Distributed,
        CapabilityCategory::Custom,
    ];

    let represented = representative_capabilities()
        .iter()
        .map(|capability| capability.category())
        .collect::<BTreeSet<_>>();

    for category in required_domains {
        assert!(
            represented.contains(&category),
            "capability vocabulary lacks coverage for category {}",
            category.as_str()
        );
    }
}

#[test]
fn capability_set_is_suitable_as_backend_authoritative_metadata() {
    let mut backend_capabilities = CapabilitySet::new();

    backend_capabilities.insert(
        QuantumCapability::Measurement,
    );

    backend_capabilities.insert(
        QuantumCapability::Reset,
    );

    backend_capabilities.insert(
        QuantumCapability::TwoQubitGates,
    );

    backend_capabilities.insert_experimental(
        QuantumCapability::DynamicCircuits,
    );

    let workload = CapabilityRequirements::new()
        .require(QuantumCapability::Measurement)
        .require(QuantumCapability::Reset)
        .require(QuantumCapability::TwoQubitGates)
        .prefer(QuantumCapability::DynamicCircuits);

    let compatibility = backend_capabilities.check(
        &workload,
        CapabilityPolicy::StableOnly,
    );

    assert!(compatibility.compatible);

    assert!(
        compatibility.missing.is_empty()
    );

    assert!(
        compatibility.rejected_status.is_empty()
    );

    // Experimental functionality is visible but not silently promoted to
    // stable support.
    assert_eq!(
        backend_capabilities.experimental(),
        vec![QuantumCapability::DynamicCircuits]
    );
}