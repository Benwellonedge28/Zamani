//! Zamani Quantum Memory — Measurement Collapse
//!
//! Production-grade, representation-independent post-measurement collapse
//! contract.
//!
//! # Responsibility
//!
//! This module owns the provider-neutral semantics for transforming quantum
//! state after a measurement outcome has been selected.
//!
//! It defines:
//!
//! - collapse requests;
//! - selected measurement outcomes;
//! - collapse execution metadata;
//! - collapse policies;
//! - collapse capabilities;
//! - representation-neutral collapse execution;
//! - deterministic validation of collapse requests;
//! - post-collapse invariant checking;
//! - provider/QPU capability boundaries;
//! - simulator-versus-QPU semantic separation;
//! - partial/multi-qubit collapse contracts;
//! - arbitrary projective-measurement contracts;
//! - classical-result association;
//! - provider-controlled collapse reporting;
//! - collapse audit metadata.
//!
//! # What this module does NOT implement
//!
//! This file does NOT contain representation-specific mathematics for:
//!
//! - state vectors;
//! - density matrices;
//! - stabilizer/tableau states;
//! - sparse states;
//! - tensor networks;
//! - GPU kernels;
//! - distributed state-vector communication;
//! - QPU network communication;
//! - hardware calibration;
//! - routing;
//! - scheduling;
//! - QEC decoding;
//! - readout mitigation;
//! - compiler parsing.
//!
//! Representation-specific implementations belong behind the
//! `CollapseExecutor` boundary.
//!
//! # Architectural position
//!
//! ```text
//!                    quantum::ir
//!                         |
//!                         v
//!                 execution layer
//!                         |
//!                         v
//!             memory::measurement
//!                         |
//!                 outcome selected
//!                         |
//!                         v
//!              memory::collapse
//!                         |
//!          +--------------+---------------+
//!          |              |               |
//!          v              v               v
//!      StateVector    DensityMatrix   Stabilizer
//!          |              |               |
//!          +--------------+---------------+
//!                         |
//!                         v
//!                  provider boundary
//!                         |
//!             +-----------+-----------+
//!             |                       |
//!             v                       v
//!            CPU/GPU                  QPU
//! ```
//!
//! # QPU rule
//!
//! A physical QPU is not assumed to expose its post-measurement quantum state.
//!
//! A QPU provider may:
//!
//! - perform measurement;
//! - physically collapse the measured system;
//! - continue a dynamic circuit;
//! - return classical results;
//! - report provider-specific post-measurement semantics;
//!
//! without exposing amplitudes or a mutable state object to Zamani.
//!
//! Therefore a caller MUST NOT require a local collapse operation merely
//! because a measurement occurred on a QPU.
//!
//! `CollapseExecutor` explicitly distinguishes:
//!
//! - locally executable collapse;
//! - provider-managed collapse;
//! - unsupported collapse;
//! - state-preserving measurement.
//!
//! # Canonical identity rule
//!
//! Logical qubits use:
//!
//! `crate::quantum::ir::QubitId`
//!
//! Physical qubits use:
//!
//! `crate::quantum::ir::PhysicalQubitId`
//!
//! Classical bits use:
//!
//! `crate::quantum::ir::ClassicalBitId`.
//!
//! No replacement identity types are defined here.
//!
//! # Error boundary
//!
//! All fallible operations return:
//!
//! `Result<T, MemoryError>`.
//!
//! # Safety
//!
//! Unsafe Rust is forbidden.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021.
//!
//! No nightly features are required.
//!
//! # Integration
//!
//! Earlier foundational modules:
//!
//! ```text
//! types.rs
//! errors.rs
//! numeric.rs
//! complex.rs
//! representation.rs
//! limits.rs
//! layout.rs
//! indexing.rs
//! state.rs
//! measurement.rs
//! ```
//!
//! Consumers:
//!
//! ```text
//! state_vector.rs
//! density_matrix.rs
//! stabilizer.rs
//! sparse.rs
//! tensor_network.rs
//! backend_state.rs
//! reset.rs
//! snapshot.rs
//! checkpoint.rs
//! diagnostics.rs
//! telemetry.rs
//! runtime/execution
//! quantum::error_correction
//! quantum::hardware
//! ```
//!
//! Higher-level modules must not depend on representation-specific details
//! exposed by this file.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::BTreeMap;
use std::fmt;

use crate::quantum::ir::{
    ClassicalBitId,
    PhysicalQubitId,
    QubitId,
};

use super::errors::MemoryError;
use super::measurement::{
    CollapsePolicy,
    MeasurementObservable,
};

use super::state::{
    StateConsistency,
    StateExecutionDomain,
    StateLifecycle,
    StateRepresentationName,
    StateStorageLocation,
};

// =============================================================================
// Schema
// =============================================================================

/// Stable schema identifier for the collapse contract.
pub const COLLAPSE_SCHEMA_ID: &str = "zamani.quantum.memory.collapse";

/// Semantic version of the collapse contract.
pub const COLLAPSE_SCHEMA_VERSION: u16 = 1;

/// Default numerical tolerance used when validating collapse probabilities.
pub const DEFAULT_COLLAPSE_TOLERANCE: f64 = 1.0e-12;

/// Maximum number of qubits in one collapse request.
pub const MAX_COLLAPSE_QUBITS: usize = 1_048_576;

/// Maximum number of classical destinations.
pub const MAX_COLLAPSE_DESTINATIONS: usize = 1_048_576;

/// Maximum provider metadata entries.
pub const MAX_PROVIDER_METADATA_ENTRIES: usize = 256;

/// Maximum provider metadata key length.
pub const MAX_PROVIDER_METADATA_KEY_LENGTH: usize = 128;

/// Maximum provider metadata value length.
pub const MAX_PROVIDER_METADATA_VALUE_LENGTH: usize = 4096;

// =============================================================================
// Result
// =============================================================================

/// Canonical collapse result.
pub type CollapseResult<T> = Result<T, MemoryError>;

// =============================================================================
// Outcome
// =============================================================================

/// A single binary measurement outcome.
///
/// `false` represents logical `0`.
///
/// `true` represents logical `1`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MeasurementOutcome(bool);

impl MeasurementOutcome {
    /// Creates an outcome from a classical bit.
    pub const fn new(value: bool) -> Self {
        Self(value)
    }

    /// Creates logical zero.
    pub const fn zero() -> Self {
        Self(false)
    }

    /// Creates logical one.
    pub const fn one() -> Self {
        Self(true)
    }

    /// Returns the logical value.
    pub const fn value(self) -> bool {
        self.0
    }

    /// Returns the outcome as an integer.
    pub const fn as_u8(self) -> u8 {
        if self.0 {
            1
        } else {
            0
        }
    }
}

impl From<bool> for MeasurementOutcome {
    fn from(value: bool) -> Self {
        Self(value)
    }
}

impl From<MeasurementOutcome> for bool {
    fn from(value: MeasurementOutcome) -> Self {
        value.0
    }
}

impl fmt::Display for MeasurementOutcome {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(if self.0 { "1" } else { "0" })
    }
}

// =============================================================================
// Per-qubit outcome
// =============================================================================

/// Outcome associated with one measured logical qubit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct QubitOutcome {
    /// Logical qubit.
    pub qubit: QubitId,

    /// Optional physical-qubit association.
    pub physical_qubit: Option<PhysicalQubitId>,

    /// Selected measurement outcome.
    pub outcome: MeasurementOutcome,

    /// Probability of the selected outcome before collapse, if known.
    ///
    /// QPU providers may not expose this value.
    pub probability: Option<f64>,

    /// Optional classical destination.
    pub classical_bit: Option<ClassicalBitId>,
}

impl QubitOutcome {
    /// Creates a qubit outcome without provider-specific information.
    pub const fn new(
        qubit: QubitId,
        outcome: MeasurementOutcome,
    ) -> Self {
        Self {
            qubit,
            physical_qubit: None,
            outcome,
            probability: None,
            classical_bit: None,
        }
    }

    /// Associates a physical qubit.
    pub const fn with_physical_qubit(
        mut self,
        physical_qubit: PhysicalQubitId,
    ) -> Self {
        self.physical_qubit = Some(physical_qubit);
        self
    }

    /// Associates a pre-collapse probability.
    pub fn with_probability(
        mut self,
        probability: f64,
    ) -> CollapseResult<Self> {
        validate_probability(probability, "collapse outcome probability")?;
        self.probability = Some(probability);
        Ok(self)
    }

    /// Associates a classical destination.
    pub const fn with_classical_bit(
        mut self,
        classical_bit: ClassicalBitId,
    ) -> Self {
        self.classical_bit = Some(classical_bit);
        self
    }

    /// Validates this outcome.
    pub fn validate(&self) -> CollapseResult<()> {
        if let Some(probability) = self.probability {
            validate_probability(probability, "collapse outcome probability")?;

            if probability <= DEFAULT_COLLAPSE_TOLERANCE {
                return Err(collapse_error(
                    "cannot collapse onto an outcome with zero probability",
                ));
            }
        }

        Ok(())
    }
}

// =============================================================================
// Collapse request
// =============================================================================

/// Complete post-measurement collapse request.
///
/// This object contains the selected outcome but deliberately does not own
/// quantum-state storage.
#[derive(Debug, Clone, PartialEq)]
pub struct CollapseRequest {
    /// Logical qubits whose measured state must be collapsed.
    pub outcomes: Vec<QubitOutcome>,

    /// Observable that produced the measurement.
    ///
    /// This is required because collapse depends on the measurement projector.
    pub observable: MeasurementObservable,

    /// Requested collapse policy.
    pub policy: CollapsePolicy,

    /// Execution domain from which the measurement originated.
    pub execution_domain: StateExecutionDomain,

    /// Expected representation, when known.
    ///
    /// A QPU may leave this as `None`.
    pub representation: Option<StateRepresentationName>,

    /// Storage location, when known.
    pub storage_location: Option<StateStorageLocation>,

    /// Expected state lifecycle before collapse.
    pub expected_lifecycle: Option<StateLifecycle>,

    /// Expected state consistency before collapse.
    pub expected_consistency: Option<StateConsistency>,

    /// Provider metadata.
    ///
    /// Metadata is informational and must never contain credentials or secrets.
    pub provider_metadata: BTreeMap<String, String>,
}

impl CollapseRequest {
    /// Creates a collapse request.
    pub fn new(
        outcomes: Vec<QubitOutcome>,
        observable: MeasurementObservable,
    ) -> CollapseResult<Self> {
        let request = Self {
            outcomes,
            observable,
            policy: CollapsePolicy::Collapse,
            execution_domain: StateExecutionDomain::LocalSimulator,
            representation: None,
            storage_location: None,
            expected_lifecycle: None,
            expected_consistency: None,
            provider_metadata: BTreeMap::new(),
        };

        request.validate()?;

        Ok(request)
    }

    /// Changes the collapse policy.
    pub const fn with_policy(
        mut self,
        policy: CollapsePolicy,
    ) -> Self {
        self.policy = policy;
        self
    }

    /// Changes the execution domain.
    pub const fn with_execution_domain(
        mut self,
        domain: StateExecutionDomain,
    ) -> Self {
        self.execution_domain = domain;
        self
    }

    /// Associates a representation.
    pub fn with_representation(
        mut self,
        representation: StateRepresentationName,
    ) -> Self {
        self.representation = Some(representation);
        self
    }

    /// Associates a storage location.
    pub const fn with_storage_location(
        mut self,
        location: StateStorageLocation,
    ) -> Self {
        self.storage_location = Some(location);
        self
    }

    /// Associates an expected lifecycle.
    pub const fn with_expected_lifecycle(
        mut self,
        lifecycle: StateLifecycle,
    ) -> Self {
        self.expected_lifecycle = Some(lifecycle);
        self
    }

    /// Associates an expected consistency state.
    pub const fn with_expected_consistency(
        mut self,
        consistency: StateConsistency,
    ) -> Self {
        self.expected_consistency = Some(consistency);
        self
    }

    /// Adds safe provider metadata.
    pub fn insert_provider_metadata(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> CollapseResult<()> {
        if self.provider_metadata.len() >= MAX_PROVIDER_METADATA_ENTRIES
            && !self.provider_metadata.contains_key(key.into().as_str())
        {
            return Err(collapse_error(
                "maximum provider metadata entry count exceeded",
            ));
        }

        // Avoid retaining arbitrary provider data.
        let key = key.into();
        let value = value.into();

        validate_metadata_text(
            &key,
            MAX_PROVIDER_METADATA_KEY_LENGTH,
            "provider metadata key",
        )?;

        validate_metadata_text(
            &value,
            MAX_PROVIDER_METADATA_VALUE_LENGTH,
            "provider metadata value",
        )?;

        self.provider_metadata.insert(key, value);

        Ok(())
    }

    /// Validates the complete request.
    pub fn validate(&self) -> CollapseResult<()> {
        if self.outcomes.is_empty() {
            return Err(collapse_error(
                "collapse request must contain at least one outcome",
            ));
        }

        if self.outcomes.len() > MAX_COLLAPSE_QUBITS {
            return Err(collapse_error(
                "collapse request exceeds maximum supported qubit count",
            ));
        }

        self.observable.validate()?;

        let mut logical_qubits = std::collections::BTreeSet::new();
        let mut classical_bits = std::collections::BTreeSet::new();
        let mut physical_qubits = std::collections::BTreeSet::new();

        for outcome in &self.outcomes {
            outcome.validate()?;

            if !logical_qubits.insert(outcome.qubit) {
                return Err(collapse_error(
                    "collapse request contains duplicate logical qubits",
                ));
            }

            if let Some(physical_qubit) = outcome.physical_qubit {
                if !physical_qubits.insert(physical_qubit) {
                    return Err(collapse_error(
                        "collapse request contains duplicate physical qubits",
                    ));
                }
            }

            if let Some(classical_bit) = outcome.classical_bit {
                if !classical_bits.insert(classical_bit) {
                    return Err(collapse_error(
                        "collapse request contains duplicate classical destinations",
                    ));
                }
            }
        }

        if classical_bits.len() > MAX_COLLAPSE_DESTINATIONS {
            return Err(collapse_error(
                "collapse request exceeds maximum classical destination count",
            ));
        }

        if self.policy == CollapsePolicy::Collapse
            && self.execution_domain.is_qpu()
        {
            // A QPU can physically collapse the system, but the local memory
            // layer cannot claim that it performed that collapse unless a
            // provider executor is present.
            //
            // This is intentionally NOT an error at request construction.
            // The executor decides whether the operation is provider-managed.
        }

        for (key, value) in &self.provider_metadata {
            validate_metadata_text(
                key,
                MAX_PROVIDER_METADATA_KEY_LENGTH,
                "provider metadata key",
            )?;

            validate_metadata_text(
                value,
                MAX_PROVIDER_METADATA_VALUE_LENGTH,
                "provider metadata value",
            )?;
        }

        Ok(())
    }

    /// Returns the number of collapsed qubits.
    pub fn qubit_count(&self) -> usize {
        self.outcomes.len()
    }

    /// Returns whether this request targets a QPU.
    pub const fn targets_qpu(&self) -> bool {
        self.execution_domain.is_qpu()
    }

    /// Returns whether this request requires a mathematical collapse.
    pub const fn requires_mathematical_collapse(&self) -> bool {
        matches!(self.policy, CollapsePolicy::Collapse)
    }
}

// =============================================================================
// Capability
// =============================================================================

/// Capability flags required by a collapse provider.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CollapseCapabilities(u32);

impl CollapseCapabilities {
    /// Provider can perform local state collapse.
    pub const LOCAL_COLLAPSE: Self = Self(1 << 0);

    /// Provider manages physical collapse itself.
    pub const PROVIDER_MANAGED_COLLAPSE: Self = Self(1 << 1);

    /// Provider supports multi-qubit projective collapse.
    pub const MULTI_QUBIT: Self = Self(1 << 2);

    /// Provider supports arbitrary Bloch-axis collapse.
    pub const ARBITRARY_BLOCH_AXIS: Self = Self(1 << 3);

    /// Provider supports provider-defined observables.
    pub const PROVIDER_DEFINED_OBSERVABLE: Self = Self(1 << 4);

    /// Provider can preserve the state after measurement.
    pub const NON_DESTRUCTIVE: Self = Self(1 << 5);

    /// Provider can perform mid-circuit collapse.
    pub const MID_CIRCUIT: Self = Self(1 << 6);

    /// Provider can validate post-collapse state.
    pub const POST_COLLAPSE_VALIDATION: Self = Self(1 << 7);

    /// Provider supports logical-to-physical associations.
    pub const PHYSICAL_MAPPING: Self = Self(1 << 8);

    /// Provider supports classical destinations.
    pub const CLASSICAL_DESTINATION: Self = Self(1 << 9);

    /// Provider supports distributed collapse.
    pub const DISTRIBUTED: Self = Self(1 << 10);

    /// Provider supports device-side collapse.
    pub const DEVICE_SIDE: Self = Self(1 << 11);

    /// Empty capability set.
    pub const EMPTY: Self = Self(0);

    /// Returns the raw capability bits.
    pub const fn bits(self) -> u32 {
        self.0
    }

    /// Creates a capability set from raw bits.
    pub const fn from_bits(bits: u32) -> Self {
        Self(bits)
    }

    /// Adds another capability set.
    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    /// Tests whether all requested capabilities are present.
    pub const fn contains(self, required: Self) -> bool {
        self.0 & required.0 == required.0
    }

    /// Adds capabilities.
    pub const fn with(self, capability: Self) -> Self {
        self.union(capability)
    }

    /// Returns whether local collapse is available.
    pub const fn supports_local_collapse(self) -> bool {
        self.contains(Self::LOCAL_COLLAPSE)
    }

    /// Returns whether provider-managed collapse is available.
    pub const fn supports_provider_managed_collapse(self) -> bool {
        self.contains(Self::PROVIDER_MANAGED_COLLAPSE)
    }

    /// Returns whether non-destructive measurement is supported.
    pub const fn supports_non_destructive(self) -> bool {
        self.contains(Self::NON_DESTRUCTIVE)
    }
}

// =============================================================================
// Collapse execution mode
// =============================================================================

/// Actual collapse execution mode selected by the provider.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CollapseExecutionMode {
    /// Mathematical collapse was executed against locally accessible state.
    Local,

    /// Collapse was executed by an accelerator/device provider.
    Device,

    /// Physical hardware owns the collapse.
    ProviderManaged,

    /// The measurement did not modify state.
    Preserved,

    /// No state is exposed by the provider.
    Opaque,
}

impl CollapseExecutionMode {
    /// Returns whether Zamani locally changed the state.
    pub const fn locally_modified_state(self) -> bool {
        matches!(self, Self::Local | Self::Device)
    }

    /// Returns whether an external provider owns the transition.
    pub const fn provider_owned(self) -> bool {
        matches!(
            self,
            Self::ProviderManaged | Self::Opaque
        )
    }

    /// Returns whether the state was preserved.
    pub const fn preserved(self) -> bool {
        matches!(self, Self::Preserved)
    }
}

impl fmt::Display for CollapseExecutionMode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Local => "local",
            Self::Device => "device",
            Self::ProviderManaged => "provider_managed",
            Self::Preserved => "preserved",
            Self::Opaque => "opaque",
        };

        formatter.write_str(value)
    }
}

// =============================================================================
// Result
// =============================================================================

/// Result of applying or delegating a collapse.
#[derive(Debug, Clone, PartialEq)]
pub struct CollapseResultInfo {
    /// Schema identifier.
    pub schema_id: &'static str,

    /// Schema version.
    pub schema_version: u16,

    /// Actual execution mode.
    pub execution_mode: CollapseExecutionMode,

    /// Number of qubits whose measurement outcomes were applied.
    pub collapsed_qubits: usize,

    /// Whether the result has been mathematically verified.
    pub verified: bool,

    /// Whether state normalization was verified where applicable.
    pub normalization_verified: bool,

    /// Whether probability conservation was verified where applicable.
    pub probability_verified: bool,

    /// Optional post-collapse representation.
    pub representation: Option<StateRepresentationName>,

    /// Optional storage location.
    pub storage_location: Option<StateStorageLocation>,

    /// Provider metadata returned by the execution boundary.
    pub provider_metadata: BTreeMap<String, String>,
}

impl CollapseResultInfo {
    /// Creates an execution result.
    pub fn new(
        mode: CollapseExecutionMode,
        collapsed_qubits: usize,
    ) -> CollapseResult<Self> {
        if collapsed_qubits == 0 {
            return Err(collapse_error(
                "collapse result cannot report zero collapsed qubits",
            ));
        }

        Ok(Self {
            schema_id: COLLAPSE_SCHEMA_ID,
            schema_version: COLLAPSE_SCHEMA_VERSION,
            execution_mode: mode,
            collapsed_qubits,
            verified: false,
            normalization_verified: false,
            probability_verified: false,
            representation: None,
            storage_location: None,
            provider_metadata: BTreeMap::new(),
        })
    }

    /// Marks mathematical validation as successful.
    pub const fn verified(mut self) -> Self {
        self.verified = true;
        self.normalization_verified = true;
        self.probability_verified = true;
        self
    }

    /// Associates the representation.
    pub fn with_representation(
        mut self,
        representation: StateRepresentationName,
    ) -> Self {
        self.representation = Some(representation);
        self
    }

    /// Associates storage location.
    pub const fn with_storage_location(
        mut self,
        location: StateStorageLocation,
    ) -> Self {
        self.storage_location = Some(location);
        self
    }

    /// Adds provider metadata.
    pub fn insert_provider_metadata(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> CollapseResult<()> {
        let key = key.into();
        let value = value.into();

        validate_metadata_text(
            &key,
            MAX_PROVIDER_METADATA_KEY_LENGTH,
            "provider metadata key",
        )?;

        validate_metadata_text(
            &value,
            MAX_PROVIDER_METADATA_VALUE_LENGTH,
            "provider metadata value",
        )?;

        if self.provider_metadata.len() >= MAX_PROVIDER_METADATA_ENTRIES
            && !self.provider_metadata.contains_key(&key)
        {
            return Err(collapse_error(
                "maximum provider metadata entry count exceeded",
            ));
        }

        self.provider_metadata.insert(key, value);

        Ok(())
    }
}

// =============================================================================
// Collapse executor
// =============================================================================

/// Representation/provider-neutral collapse executor.
///
/// Implementations live in representation-specific modules or provider
/// adapters. The trait itself contains no state-vector, density-matrix, GPU,
/// or QPU implementation details.
pub trait CollapseExecutor {
    /// Returns the capabilities of this executor.
    fn capabilities(&self) -> CollapseCapabilities;

    /// Returns the execution domain.
    fn execution_domain(&self) -> StateExecutionDomain;

    /// Applies or delegates collapse.
    fn collapse(
        &mut self,
        request: &CollapseRequest,
    ) -> CollapseResult<CollapseResultInfo>;

    /// Returns a human-readable provider/representation identifier.
    fn provider_name(&self) -> &str;

    /// Returns the representation name when one is exposed.
    fn representation_name(&self) -> Option<StateRepresentationName> {
        None
    }

    /// Returns the storage location when one is exposed.
    fn storage_location(&self) -> Option<StateStorageLocation> {
        None
    }
}

// =============================================================================
// Capability negotiation
// =============================================================================

/// Validates that an executor can satisfy a collapse request.
pub fn validate_capabilities<E: CollapseExecutor>(
    executor: &E,
    request: &CollapseRequest,
) -> CollapseResult<()> {
    request.validate()?;

    let capabilities = executor.capabilities();

    if request.qubit_count() > 1
        && !capabilities.contains(CollapseCapabilities::MULTI_QUBIT)
    {
        return Err(capability_error(
            "collapse executor does not support multi-qubit collapse",
        ));
    }

    match &request.observable {
        MeasurementObservable::Standard(_) => {}

        MeasurementObservable::BlochAxis(_) => {
            if !capabilities.contains(
                CollapseCapabilities::ARBITRARY_BLOCH_AXIS,
            ) {
                return Err(capability_error(
                    "collapse executor does not support arbitrary Bloch-axis collapse",
                ));
            }
        }

        MeasurementObservable::ProviderDefined { .. } => {
            if !capabilities.contains(
                CollapseCapabilities::PROVIDER_DEFINED_OBSERVABLE,
            ) {
                return Err(capability_error(
                    "collapse executor does not support provider-defined observables",
                ));
            }
        }
    }

    if request.execution_domain.is_distributed()
        && !capabilities.contains(CollapseCapabilities::DISTRIBUTED)
    {
        return Err(capability_error(
            "collapse executor does not support distributed collapse",
        ));
    }

    if request.policy == CollapsePolicy::PreserveIfSupported
        && !capabilities.contains(CollapseCapabilities::NON_DESTRUCTIVE)
    {
        return Err(capability_error(
            "collapse request requires non-destructive support",
        ));
    }

    if request.execution_domain.is_qpu()
        && !capabilities.contains(
            CollapseCapabilities::PROVIDER_MANAGED_COLLAPSE,
        )
        && !capabilities.contains(CollapseCapabilities::LOCAL_COLLAPSE)
    {
        return Err(capability_error(
            "QPU collapse requires provider-managed or explicit local collapse capability",
        ));
    }

    Ok(())
}

// =============================================================================
// High-level execution
// =============================================================================

/// Executes a validated collapse through the supplied executor.
///
/// This is the principal integration point for state representations and
/// hardware adapters.
pub fn execute_collapse<E: CollapseExecutor>(
    executor: &mut E,
    request: &CollapseRequest,
) -> CollapseResult<CollapseResultInfo> {
    validate_capabilities(executor, request)?;

    let mut result = executor.collapse(request)?;

    if result.collapsed_qubits != request.qubit_count() {
        return Err(invariant_error(
            "collapse executor returned an inconsistent collapsed-qubit count",
        ));
    }

    if result.schema_id != COLLAPSE_SCHEMA_ID {
        return Err(invariant_error(
            "collapse executor returned an incompatible schema identifier",
        ));
    }

    if result.schema_version != COLLAPSE_SCHEMA_VERSION {
        return Err(invariant_error(
            "collapse executor returned an incompatible schema version",
        ));
    }

    if result.representation.is_none() {
        result.representation = executor.representation_name();
    }

    if result.storage_location.is_none() {
        result.storage_location = executor.storage_location();
    }

    Ok(result)
}

// =============================================================================
// Provider-managed result
// =============================================================================

/// Constructs a result for a physical QPU whose hardware owns the collapse.
///
/// This function deliberately does not fabricate amplitudes, probabilities,
/// state vectors, or local state transitions.
pub fn provider_managed_result(
    request: &CollapseRequest,
    provider_metadata: BTreeMap<String, String>,
) -> CollapseResult<CollapseResultInfo> {
    request.validate()?;

    if !request.execution_domain.is_qpu() {
        return Err(collapse_error(
            "provider-managed collapse result requires QPU execution domain",
        ));
    }

    let mut result = CollapseResultInfo::new(
        CollapseExecutionMode::ProviderManaged,
        request.qubit_count(),
    )?;

    for (key, value) in provider_metadata {
        result.insert_provider_metadata(key, value)?;
    }

    Ok(result)
}

/// Constructs a result for an opaque backend state.
///
/// This is appropriate when the backend has physically executed measurement
/// and collapse but exposes no mutable quantum state to Zamani.
pub fn opaque_provider_result(
    request: &CollapseRequest,
    provider_metadata: BTreeMap<String, String>,
) -> CollapseResult<CollapseResultInfo> {
    request.validate()?;

    let mut result = CollapseResultInfo::new(
        CollapseExecutionMode::Opaque,
        request.qubit_count(),
    )?;

    for (key, value) in provider_metadata {
        result.insert_provider_metadata(key, value)?;
    }

    Ok(result)
}

// =============================================================================
// Mathematical validation helpers
// =============================================================================

/// Validates a post-collapse probability.
pub fn validate_post_collapse_probability(
    probability: f64,
) -> CollapseResult<()> {
    validate_probability(
        probability,
        "post-collapse probability",
    )
}

/// Validates that a set of probabilities is normalized.
pub fn validate_probability_distribution(
    probabilities: &[f64],
    tolerance: f64,
) -> CollapseResult<()> {
    if probabilities.is_empty() {
        return Err(collapse_error(
            "probability distribution cannot be empty",
        ));
    }

    validate_tolerance(tolerance)?;

    let mut sum = 0.0_f64;

    for probability in probabilities {
        validate_probability(
            *probability,
            "probability distribution element",
        )?;

        sum += *probability;

        if !sum.is_finite() {
            return Err(collapse_error(
                "probability distribution sum became non-finite",
            ));
        }
    }

    if (sum - 1.0).abs() > tolerance {
        return Err(collapse_error(
            "probability distribution is not normalized",
        ));
    }

    Ok(())
}

/// Validates that a selected outcome has non-zero probability.
pub fn validate_selected_probability(
    probability: f64,
) -> CollapseResult<()> {
    validate_probability(
        probability,
        "selected collapse probability",
    )?;

    if probability <= DEFAULT_COLLAPSE_TOLERANCE {
        return Err(collapse_error(
            "selected outcome has zero probability",
        ));
    }

    Ok(())
}

/// Calculates the renormalization factor after projection.
///
/// For a valid non-zero selected probability `p`, the normalized projected
/// state is scaled by `1 / sqrt(p)`.
pub fn projection_normalization_factor(
    selected_probability: f64,
) -> CollapseResult<f64> {
    validate_selected_probability(selected_probability)?;

    let factor = 1.0 / selected_probability.sqrt();

    if !factor.is_finite() {
        return Err(collapse_error(
            "projection normalization factor is non-finite",
        ));
    }

    Ok(factor)
}

// =============================================================================
// Outcome encoding
// =============================================================================

/// Encodes ordered binary outcomes into a little-endian integer.
///
/// The first supplied outcome occupies bit 0.
pub fn encode_outcomes(
    outcomes: &[QubitOutcome],
) -> CollapseResult<u64> {
    if outcomes.len() > 64 {
        return Err(collapse_error(
            "cannot encode more than 64 collapse outcomes into u64",
        ));
    }

    let mut value = 0_u64;

    for (index, outcome) in outcomes.iter().enumerate() {
        outcome.validate()?;

        if outcome.outcome.value() {
            value |= 1_u64
                .checked_shl(index as u32)
                .ok_or_else(|| {
                    arithmetic_error(
                        "collapse outcome bit shift overflowed",
                    )
                })?;
        }
    }

    Ok(value)
}

/// Encodes ordered outcomes as a binary string.
///
/// The returned string follows the supplied qubit order.
pub fn encode_outcomes_binary(
    outcomes: &[QubitOutcome],
) -> CollapseResult<String> {
    for outcome in outcomes {
        outcome.validate()?;
    }

    let mut encoded = String::with_capacity(outcomes.len());

    for outcome in outcomes {
        encoded.push(if outcome.outcome.value() {
            '1'
        } else {
            '0'
        });
    }

    Ok(encoded)
}

// =============================================================================
// Outcome lookup
// =============================================================================

/// Looks up the selected outcome for a logical qubit.
pub fn outcome_for_qubit(
    request: &CollapseRequest,
    qubit: QubitId,
) -> Option<QubitOutcome> {
    request
        .outcomes
        .iter()
        .find(|outcome| outcome.qubit == qubit)
        .copied()
}

// =============================================================================
// Post-collapse contract
// =============================================================================

/// Contract describing invariants that an executor should preserve.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PostCollapseInvariantPolicy {
    /// Maximum allowed normalization error.
    pub normalization_tolerance: f64,

    /// Maximum allowed probability error.
    pub probability_tolerance: f64,

    /// Whether finite-value checking is required.
    pub require_finite_values: bool,

    /// Whether the executor must report successful validation.
    pub require_verification: bool,
}

impl Default for PostCollapseInvariantPolicy {
    fn default() -> Self {
        Self {
            normalization_tolerance: DEFAULT_COLLAPSE_TOLERANCE,
            probability_tolerance: DEFAULT_COLLAPSE_TOLERANCE,
            require_finite_values: true,
            require_verification: false,
        }
    }
}

impl PostCollapseInvariantPolicy {
    /// Validates the policy.
    pub fn validate(&self) -> CollapseResult<()> {
        validate_tolerance(self.normalization_tolerance)?;
        validate_tolerance(self.probability_tolerance)?;

        Ok(())
    }

    /// Validates a normalized value.
    pub fn validate_normalization(
        &self,
        norm: f64,
    ) -> CollapseResult<()> {
        if !norm.is_finite() {
            return Err(collapse_error(
                "post-collapse norm is non-finite",
            ));
        }

        if (norm - 1.0).abs() > self.normalization_tolerance {
            return Err(collapse_error(
                "post-collapse state is not normalized",
            ));
        }

        Ok(())
    }
}

// =============================================================================
// Audit information
// =============================================================================

/// Safe audit metadata for a collapse transition.
///
/// This deliberately contains no quantum amplitudes, raw memory addresses,
/// secrets, or credentials.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CollapseAudit {
    /// Number of measured/collapsed qubits.
    pub qubit_count: usize,

    /// Execution mode.
    pub execution_mode: CollapseExecutionMode,

    /// Whether the transition was provider-managed.
    pub provider_managed: bool,

    /// Whether the state was locally modified.
    pub locally_modified: bool,

    /// Whether the state was preserved.
    pub preserved: bool,
}

impl CollapseAudit {
    /// Creates audit information from a result.
    pub const fn from_result(
        result: &CollapseResultInfo,
    ) -> Self {
        Self {
            qubit_count: result.collapsed_qubits,
            execution_mode: result.execution_mode,
            provider_managed: result.execution_mode.provider_owned(),
            locally_modified: result.execution_mode.locally_modified_state(),
            preserved: result.execution_mode.preserved(),
        }
    }
}

// =============================================================================
// Error helpers
// =============================================================================

fn collapse_error(message: &'static str) -> MemoryError {
    MemoryError::collapse_error(message)
}

fn capability_error(message: &'static str) -> MemoryError {
    MemoryError::backend_capability_unavailable(message)
}

fn invariant_error(message: &'static str) -> MemoryError {
    MemoryError::invariant_violation(message)
}

fn arithmetic_error(message: &'static str) -> MemoryError {
    MemoryError::arithmetic_overflow(message)
}

fn validate_probability(
    probability: f64,
    field: &'static str,
) -> CollapseResult<()> {
    if !probability.is_finite() {
        return Err(collapse_error(field));
    }

    if probability < 0.0 || probability > 1.0 {
        return Err(collapse_error(
            "probability must be within the closed interval [0, 1]",
        ));
    }

    Ok(())
}

fn validate_tolerance(tolerance: f64) -> CollapseResult<()> {
    if !tolerance.is_finite()
        || tolerance < 0.0
        || tolerance > 1.0
    {
        return Err(collapse_error(
            "collapse tolerance must be finite and within [0, 1]",
        ));
    }

    Ok(())
}

fn validate_metadata_text(
    value: &str,
    maximum_length: usize,
    field: &'static str,
) -> CollapseResult<()> {
    if value.is_empty() {
        return Err(collapse_error(field));
    }

    if value.len() > maximum_length {
        return Err(collapse_error(field));
    }

    if value.chars().any(char::is_control) {
        return Err(collapse_error(field));
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
    fn outcome_zero_is_zero() {
        assert_eq!(MeasurementOutcome::zero().as_u8(), 0);
        assert!(!MeasurementOutcome::zero().value());
    }

    #[test]
    fn outcome_one_is_one() {
        assert_eq!(MeasurementOutcome::one().as_u8(), 1);
        assert!(MeasurementOutcome::one().value());
    }

    #[test]
    fn projection_factor_is_correct() {
        let factor = projection_normalization_factor(0.25)
            .expect("valid probability");

        assert!((factor - 2.0).abs() < 1.0e-12);
    }

    #[test]
    fn zero_probability_projection_is_rejected() {
        assert!(
            projection_normalization_factor(0.0).is_err()
        );
    }

    #[test]
    fn invalid_probability_is_rejected() {
        assert!(
            validate_post_collapse_probability(-0.1).is_err()
        );

        assert!(
            validate_post_collapse_probability(1.1).is_err()
        );

        assert!(
            validate_post_collapse_probability(f64::NAN).is_err()
        );
    }

    #[test]
    fn probability_distribution_is_validated() {
        assert!(
            validate_probability_distribution(
                &[0.25, 0.25, 0.25, 0.25],
                1.0e-12,
            )
            .is_ok()
        );
    }

    #[test]
    fn invalid_probability_distribution_is_rejected() {
        assert!(
            validate_probability_distribution(
                &[0.25, 0.25, 0.25],
                1.0e-12,
            )
            .is_err()
        );
    }

    #[test]
    fn duplicate_qubits_are_rejected() {
        let qubit = QubitId::new(0);

        let request = CollapseRequest::new(
            vec![
                QubitOutcome::new(
                    qubit,
                    MeasurementOutcome::zero(),
                ),
                QubitOutcome::new(
                    qubit,
                    MeasurementOutcome::one(),
                ),
            ],
            MeasurementObservable::default(),
        );

        assert!(request.is_err());
    }

    #[test]
    fn outcomes_encode_little_endian() {
        let q0 = QubitId::new(0);
        let q1 = QubitId::new(1);

        let outcomes = vec![
            QubitOutcome::new(
                q0,
                MeasurementOutcome::one(),
            ),
            QubitOutcome::new(
                q1,
                MeasurementOutcome::zero(),
            ),
        ];

        assert_eq!(
            encode_outcomes(&outcomes)
                .expect("valid outcomes"),
            1
        );
    }

    #[test]
    fn binary_encoding_preserves_order() {
        let q0 = QubitId::new(0);
        let q1 = QubitId::new(1);

        let outcomes = vec![
            QubitOutcome::new(
                q0,
                MeasurementOutcome::one(),
            ),
            QubitOutcome::new(
                q1,
                MeasurementOutcome::zero(),
            ),
        ];

        assert_eq!(
            encode_outcomes_binary(&outcomes)
                .expect("valid outcomes"),
            "10"
        );
    }

    #[test]
    fn qpu_request_is_not_rejected_at_construction() {
        let request = CollapseRequest::new(
            vec![
                QubitOutcome::new(
                    QubitId::new(0),
                    MeasurementOutcome::zero(),
                ),
            ],
            MeasurementObservable::default(),
        )
        .expect("valid request")
        .with_execution_domain(
            StateExecutionDomain::Qpu,
        );

        assert!(request.targets_qpu());
    }

    #[test]
    fn capability_set_is_composable() {
        let capabilities = CollapseCapabilities::LOCAL_COLLAPSE
            .with(CollapseCapabilities::MULTI_QUBIT);

        assert!(
            capabilities.supports_local_collapse()
        );

        assert!(
            capabilities.contains(
                CollapseCapabilities::MULTI_QUBIT
            )
        );
    }

    #[test]
    fn audit_identifies_provider_managed_execution() {
        let result = CollapseResultInfo::new(
            CollapseExecutionMode::ProviderManaged,
            1,
        )
        .expect("valid result");

        let audit = CollapseAudit::from_result(&result);

        assert!(audit.provider_managed);
        assert!(!audit.locally_modified);
    }

    #[test]
    fn audit_identifies_local_execution() {
        let result = CollapseResultInfo::new(
            CollapseExecutionMode::Local,
            1,
        )
        .expect("valid result");

        let audit = CollapseAudit::from_result(&result);

        assert!(!audit.provider_managed);
        assert!(audit.locally_modified);
        assert!(!audit.preserved);
    }
}