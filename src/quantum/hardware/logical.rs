//! Zamani Quantum Hardware — Logical / Fault-Tolerant Hardware Model.
//!
//! Production-grade, provider-neutral representation of logical quantum
//! resources exposed by a quantum-computing backend.
//!
//! # Responsibility
//!
//! This module owns the *hardware-facing* description of logical quantum
//! computing. It describes what a physical backend can expose or execute at
//! the logical level.
//!
//! It owns:
//!
//! - logical-qubit resource descriptions;
//! - logical-qubit capacity;
//! - physical-to-logical resource ratios;
//! - logical-qubit identifiers;
//! - logical operation descriptors;
//! - logical measurement descriptors;
//! - logical error-rate metadata;
//! - code-family metadata;
//! - code distance;
//! - logical error-correction metadata;
//! - syndrome/decoder capability metadata;
//! - fault-tolerance capability metadata;
//! - logical execution requirements;
//! - logical hardware compatibility checks;
//! - logical resource estimates;
//! - deterministic serialization;
//! - validation and structured diagnostics;
//! - provider-neutral conversion/reference types;
//! - explicit separation between physical and logical resources.
//!
//! # It does NOT own
//!
//! This module deliberately does not own:
//!
//! - stabilizer algebra;
//! - Pauli multiplication;
//! - logical Pauli equivalence;
//! - decoder algorithms;
//! - surface-code mathematics;
//! - syndrome decoding;
//! - QEC circuit construction;
//! - QEC simulation;
//! - physical calibration acquisition;
//! - hardware topology algorithms;
//! - routing algorithms;
//! - scheduling algorithms;
//! - provider APIs;
//! - authentication;
//! - credentials;
//! - job submission;
//! - network communication.
//!
//! Those responsibilities belong to their respective modules.
//!
//! # Critical architectural distinction
//!
//! There are two different meanings of "logical":
//!
//! ```text
//! quantum::error_correction::logical
//!     = mathematical logical operators / outcomes
//!
//! quantum::hardware::logical
//!     = logical resources exposed by physical hardware
//! ```
//!
//! This module therefore MUST NOT duplicate the mathematical logical-Pauli
//! implementation in `quantum::error_correction::logical`.
//!
//! # Architecture
//!
//! ```text
//!                     Zamani Quantum IR
//!                            │
//!                            ▼
//!                     workload analysis
//!                            │
//!                            ▼
//!                  logical requirements
//!                            │
//!             ┌──────────────┴──────────────┐
//!             │                             │
//!             ▼                             ▼
//!       physical hardware             QEC subsystem
//!             │                             │
//!             │                             ▼
//!             │                   code / decoder / syndrome
//!             │                             │
//!             └──────────────┬──────────────┘
//!                            ▼
//!                  hardware::logical
//!                            │
//!              ┌─────────────┼─────────────┐
//!              ▼             ▼             ▼
//!       Logical resources  operations   compatibility
//!              │             │             │
//!              └─────────────┼─────────────┘
//!                            ▼
//!                       backend.rs
//! ```
//!
//! # Integration contract
//!
//! Future hardware modules consume this file as follows:
//!
//! - `backend.rs` consumes `LogicalHardwareCapabilities`;
//! - `capabilities.rs` may expose logical capability flags;
//! - `compatibility.rs` consumes `LogicalWorkloadRequirements`;
//! - `validation.rs` consumes `LogicalWorkloadRequirements` and
//!   `LogicalHardwareCapabilities`;
//! - `execution.rs` may carry `LogicalExecutionRequest`;
//! - `result.rs` may reference logical result metadata;
//! - `resource_estimator.rs` consumes logical resource estimates;
//! - `provider.rs` exposes provider-neutral logical resources;
//! - provider adapters translate native provider logical-resource metadata into
//!   these types;
//! - `quantum::error_correction` supplies mathematical/QEC semantics;
//! - benchmarking consumes these types but hardware never depends on
//!   benchmarking.
//!
//! No future file should modify this module merely because it needs to consume
//! logical-hardware information. Consumers must use the stable public API
//! defined here.
//!
//! # Dependency policy
//!
//! This module intentionally depends only on:
//!
//! - Rust standard library;
//! - `serde` for stable serialization.
//!
//! It does NOT depend on:
//!
//! - provider adapters;
//! - benchmarking;
//! - Danga;
//! - QEC implementation modules;
//! - routing;
//! - scheduling;
//! - network clients.
//!
//! This makes the file independently implementable and independently testable.
//!
//! # Security
//!
//! This module contains no credentials and never accepts secrets as part of
//! logical hardware metadata.
//!
//! Provider-specific authentication must remain outside this module.
//!
//! # Determinism
//!
//! All collections exposed by this module use deterministic ordering.
//!
//! Validation performs no:
//!
//! - network access;
//! - clock reads;
//! - random generation;
//! - provider calls;
//! - mutable global state access.
//!
//! # Numerical policy
//!
//! Floating-point values are validated before entering production data
//! structures. NaN and infinity are rejected.
//!
//! Probabilities and rates are required to be within `[0, 1]` unless a field's
//! documentation explicitly specifies another domain.
//!
//! # Versioning
//!
//! The schema identifier and schema version are stable machine-readable
//! contracts. Consumers should persist both when storing logical hardware
//! descriptions.
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
//! # Safety
//!
//! Unsafe Rust is forbidden.
//!
//! ```text
//! #![deny(unsafe_code)]
//! ```
//!
//! # Completion contract
//!
//! This file is considered complete when:
//!
//! 1. logical resources have stable identifiers;
//! 2. logical hardware capabilities are expressible;
//! 3. fault-tolerance metadata is expressible;
//! 4. logical operations are expressible;
//! 5. logical execution requirements are expressible;
//! 6. compatibility can be checked without provider knowledge;
//! 7. all invalid numerical/resource values are rejected;
//! 8. serialization is deterministic;
//! 9. errors are structured;
//! 10. multi-logical-qubit hardware is represented safely;
//! 11. physical and logical resource counts cannot be accidentally confused;
//! 12. provider-specific information does not leak into the core model;
//! 13. tests cover normal, boundary and failure cases.
//!
//! The file is therefore usable by later hardware modules without requiring
//! changes merely to accommodate their integration.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use core::fmt;
use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

// =============================================================================
// Schema
// =============================================================================

/// Stable logical-hardware schema identifier.
pub const LOGICAL_HARDWARE_SCHEMA_ID: &str =
    "zamani.quantum.hardware.logical";

/// Current logical-hardware schema version.
pub const LOGICAL_HARDWARE_SCHEMA_VERSION: u16 = 1;

/// Maximum logical resource identifier length.
pub const MAX_LOGICAL_ID_LENGTH: usize = 512;

/// Maximum provider/backend reference length.
pub const MAX_REFERENCE_LENGTH: usize = 512;

/// Maximum human-readable name length.
pub const MAX_NAME_LENGTH: usize = 512;

/// Maximum metadata entries.
pub const MAX_METADATA_ENTRIES: usize = 4096;

/// Maximum metadata key length.
pub const MAX_METADATA_KEY_LENGTH: usize = 256;

/// Maximum metadata value length.
pub const MAX_METADATA_VALUE_LENGTH: usize = 4096;

/// Maximum number of supported logical operations.
pub const MAX_LOGICAL_OPERATIONS: usize = 4096;

/// Maximum number of code parameters.
pub const MAX_CODE_PARAMETERS: usize = 1024;

/// Maximum logical qubit count accepted by one descriptor.
pub const MAX_LOGICAL_QUBITS: u64 = 1_000_000_000;

/// Maximum physical qubit count represented by one logical resource descriptor.
pub const MAX_PHYSICAL_QUBITS: u64 = 1_000_000_000_000;

/// Maximum code distance.
pub const MAX_CODE_DISTANCE: u32 = 1_000_000;

/// Maximum physical qubits per logical qubit.
pub const MAX_PHYSICAL_PER_LOGICAL: u64 = 1_000_000_000;

/// Maximum logical operation name length.
pub const MAX_OPERATION_NAME_LENGTH: usize = 256;

// =============================================================================
// Stable utility types
// =============================================================================

/// A stable logical-resource identifier.
///
/// The identifier is provider-neutral and must not contain credentials.
///
/// Examples:
///
/// ```text
/// logical://provider/backend/q0
/// logical://provider/backend/logical-0001
/// ```
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[serde(transparent)]
pub struct LogicalQubitId(String);

impl LogicalQubitId {
    /// Creates a validated logical-qubit identifier.
    pub fn new(value: impl Into<String>) -> Result<Self, LogicalHardwareError> {
        let value = value.into();

        validate_identifier(
            &value,
            MAX_LOGICAL_ID_LENGTH,
            "logical qubit identifier",
        )?;

        Ok(Self(value))
    }

    /// Returns the canonical identifier string.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the identifier and returns its string representation.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for LogicalQubitId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl TryFrom<String> for LogicalQubitId {
    type Error = LogicalHardwareError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<&str> for LogicalQubitId {
    type Error = LogicalHardwareError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

/// A stable backend/device reference.
///
/// This is deliberately not coupled to `backend.rs::BackendId` so this file
/// remains independently buildable. Conversion can be added at the integration
/// boundary without changing this type.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[serde(transparent)]
pub struct HardwareReference(String);

impl HardwareReference {
    /// Creates a validated hardware reference.
    pub fn new(value: impl Into<String>) -> Result<Self, LogicalHardwareError> {
        let value = value.into();

        validate_identifier(
            &value,
            MAX_REFERENCE_LENGTH,
            "hardware reference",
        )?;

        Ok(Self(value))
    }

    /// Returns the reference.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for HardwareReference {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// =============================================================================
// Numeric wrappers
// =============================================================================

/// A probability in the closed interval `[0, 1]`.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    PartialOrd,
    Serialize,
    Deserialize,
)]
#[serde(transparent)]
pub struct Probability(f64);

impl Probability {
    /// Creates a validated probability.
    pub fn new(value: f64) -> Result<Self, LogicalHardwareError> {
        validate_probability(value, "probability")?;
        Ok(Self(value))
    }

    /// Returns the raw probability.
    #[must_use]
    pub const fn get(self) -> f64 {
        self.0
    }
}

impl fmt::Display for Probability {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

/// A non-negative logical error rate.
///
/// This is represented separately from `Probability` to make the semantic
/// meaning explicit to callers.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    PartialOrd,
    Serialize,
    Deserialize,
)]
#[serde(transparent)]
pub struct LogicalErrorRate(f64);

impl LogicalErrorRate {
    /// Creates a validated logical error rate.
    pub fn new(value: f64) -> Result<Self, LogicalHardwareError> {
        validate_probability(value, "logical error rate")?;
        Ok(Self(value))
    }

    /// Returns the numerical rate.
    #[must_use]
    pub const fn get(self) -> f64 {
        self.0
    }

    /// Returns true if the error rate is exactly zero.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.0 == 0.0
    }
}

impl fmt::Display for LogicalErrorRate {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

/// A positive or zero physical-resource count.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
#[serde(transparent)]
pub struct PhysicalQubitCount(u64);

impl PhysicalQubitCount {
    /// Creates a validated physical-qubit count.
    pub fn new(value: u64) -> Result<Self, LogicalHardwareError> {
        if value > MAX_PHYSICAL_QUBITS {
            return Err(
                LogicalHardwareError::ResourceLimitExceeded {
                    resource: "physical qubits",
                    value,
                    maximum: MAX_PHYSICAL_QUBITS,
                },
            );
        }

        Ok(Self(value))
    }

    /// Returns the count.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }
}

/// A positive or zero logical-resource count.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
#[serde(transparent)]
pub struct LogicalQubitCount(u64);

impl LogicalQubitCount {
    /// Creates a validated logical-qubit count.
    pub fn new(value: u64) -> Result<Self, LogicalHardwareError> {
        if value > MAX_LOGICAL_QUBITS {
            return Err(
                LogicalHardwareError::ResourceLimitExceeded {
                    resource: "logical qubits",
                    value,
                    maximum: MAX_LOGICAL_QUBITS,
                },
            );
        }

        Ok(Self(value))
    }

    /// Returns the count.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }
}

/// Positive code distance.
///
/// A distance of zero is never a valid error-correcting code distance.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
#[serde(transparent)]
pub struct CodeDistance(u32);

impl CodeDistance {
    /// Creates a validated code distance.
    pub fn new(value: u32) -> Result<Self, LogicalHardwareError> {
        if value == 0 {
            return Err(
                LogicalHardwareError::InvalidCodeDistance {
                    value,
                },
            );
        }

        if value > MAX_CODE_DISTANCE {
            return Err(
                LogicalHardwareError::CodeDistanceLimitExceeded {
                    value,
                    maximum: MAX_CODE_DISTANCE,
                },
            );
        }

        Ok(Self(value))
    }

    /// Returns the distance.
    #[must_use]
    pub const fn get(self) -> u32 {
        self.0
    }
}

// =============================================================================
// Code family
// =============================================================================

/// Fault-tolerant/error-correction code family represented by a backend.
///
/// This enum describes the code architecture, not its decoder algorithm.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
pub enum LogicalCodeFamily {
    /// Surface code.
    SurfaceCode,

    /// Rotated surface code.
    RotatedSurfaceCode,

    /// Color code.
    ColorCode,

    /// Bacon-Shor code.
    BaconShor,

    /// Floquet code.
    Floquet,

    /// Bosonic error-correction code.
    Bosonic,

    /// Cat-code family.
    CatCode,

    /// GKP-style bosonic code.
    Gkp,

    /// Quantum low-density-parity-check code.
    Qldpc,

    /// Repetition-code family.
    Repetition,

    /// Custom/provider-specific code family.
    Custom,
}

impl LogicalCodeFamily {
    /// Stable machine-readable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SurfaceCode => "surface_code",
            Self::RotatedSurfaceCode => "rotated_surface_code",
            Self::ColorCode => "color_code",
            Self::BaconShor => "bacon_shor",
            Self::Floquet => "floquet",
            Self::Bosonic => "bosonic",
            Self::CatCode => "cat_code",
            Self::Gkp => "gkp",
            Self::Qldpc => "qldpc",
            Self::Repetition => "repetition",
            Self::Custom => "custom",
        }
    }
}

impl fmt::Display for LogicalCodeFamily {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Logical encoding
// =============================================================================

/// Physical representation of a logical quantum resource.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
pub enum LogicalEncoding {
    /// Encoded qubit constructed from physical qubits.
    QubitCode,

    /// Bosonic-mode encoding.
    BosonicMode,

    /// Continuous-variable logical encoding.
    ContinuousVariable,

    /// Photonic logical encoding.
    Photonic,

    /// Provider-defined/custom logical encoding.
    Custom,
}

impl LogicalEncoding {
    /// Returns whether the encoding is qubit-based.
    #[must_use]
    pub const fn is_qubit_based(self) -> bool {
        matches!(self, Self::QubitCode)
    }
}

// =============================================================================
// Logical operation kind
// =============================================================================

/// Semantic category of a logical operation.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
pub enum LogicalOperationKind {
    /// Logical identity.
    Identity,

    /// Single-logical-qubit X operation.
    X,

    /// Single-logical-qubit Y operation.
    Y,

    /// Single-logical-qubit Z operation.
    Z,

    /// Logical Hadamard.
    Hadamard,

    /// Logical phase/S operation.
    Phase,

    /// Logical T operation.
    T,

    /// Controlled-NOT.
    ControlledNot,

    /// Controlled-Z.
    ControlledZ,

    /// Swap.
    Swap,

    /// Toffoli/CCX.
    Toffoli,

    /// Generic Clifford operation.
    Clifford,

    /// Generic non-Clifford operation.
    NonClifford,

    /// Measurement.
    Measurement,

    /// Reset.
    Reset,

    /// State preparation.
    StatePreparation,

    /// Syndrome extraction.
    SyndromeExtraction,

    /// Decoder-assisted logical operation.
    DecoderAssisted,

    /// Provider-specific/custom logical operation.
    Custom,
}

impl LogicalOperationKind {
    /// Returns whether this operation consumes exactly one logical operand.
    #[must_use]
    pub const fn is_single_qubit(self) -> bool {
        matches!(
            self,
            Self::Identity
                | Self::X
                | Self::Y
                | Self::Z
                | Self::Hadamard
                | Self::Phase
                | Self::T
                | Self::Measurement
                | Self::Reset
                | Self::StatePreparation
                | Self::SyndromeExtraction
                | Self::DecoderAssisted
        )
    }

    /// Returns the minimum logical arity.
    #[must_use]
    pub const fn minimum_arity(self) -> u32 {
        match self {
            Self::Identity
            | Self::X
            | Self::Y
            | Self::Z
            | Self::Hadamard
            | Self::Phase
            | Self::T
            | Self::Measurement
            | Self::Reset
            | Self::StatePreparation
            | Self::SyndromeExtraction
            | Self::DecoderAssisted => 1,

            Self::ControlledNot
            | Self::ControlledZ
            | Self::Swap
            | Self::Clifford
            | Self::NonClifford => 2,

            Self::Toffoli => 3,

            Self::Custom => 1,
        }
    }
}

impl fmt::Display for LogicalOperationKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Identity => "identity",
            Self::X => "x",
            Self::Y => "y",
            Self::Z => "z",
            Self::Hadamard => "h",
            Self::Phase => "phase",
            Self::T => "t",
            Self::ControlledNot => "cx",
            Self::ControlledZ => "cz",
            Self::Swap => "swap",
            Self::Toffoli => "toffoli",
            Self::Clifford => "clifford",
            Self::NonClifford => "non_clifford",
            Self::Measurement => "measurement",
            Self::Reset => "reset",
            Self::StatePreparation => "state_preparation",
            Self::SyndromeExtraction => "syndrome_extraction",
            Self::DecoderAssisted => "decoder_assisted",
            Self::Custom => "custom",
        };

        formatter.write_str(value)
    }
}

// =============================================================================
// Logical operation descriptor
// =============================================================================

/// Provider-neutral description of one logical operation supported by hardware.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub struct LogicalOperation {
    /// Stable operation name.
    name: String,

    /// Operation semantic category.
    kind: LogicalOperationKind,

    /// Number of logical operands required.
    arity: u32,

    /// Whether execution is fault tolerant according to the provider's
    /// advertised semantics.
    fault_tolerant: bool,

    /// Whether the operation is available transversally.
    transversal: bool,

    /// Whether the operation requires magic-state resources.
    requires_magic_state: bool,

    /// Whether the operation requires decoder participation.
    decoder_assisted: bool,

    /// Estimated logical error rate, if known.
    logical_error_rate: Option<LogicalErrorRate>,
}

impl LogicalOperation {
    /// Creates a validated logical operation descriptor.
    pub fn new(
        name: impl Into<String>,
        kind: LogicalOperationKind,
        arity: u32,
    ) -> Result<Self, LogicalHardwareError> {
        let name = normalize_name(
            &name.into(),
            MAX_OPERATION_NAME_LENGTH,
            "logical operation name",
        )?;

        if arity == 0 {
            return Err(
                LogicalHardwareError::InvalidArity {
                    operation: name,
                    arity,
                },
            );
        }

        if arity < kind.minimum_arity() {
            return Err(
                LogicalHardwareError::ArityTooSmall {
                    operation: name,
                    kind,
                    minimum: kind.minimum_arity(),
                    actual: arity,
                },
            );
        }

        Ok(Self {
            name,
            kind,
            arity,
            fault_tolerant: false,
            transversal: false,
            requires_magic_state: false,
            decoder_assisted: false,
            logical_error_rate: None,
        })
    }

    /// Sets fault-tolerant execution metadata.
    #[must_use]
    pub const fn with_fault_tolerance(
        mut self,
        value: bool,
    ) -> Self {
        self.fault_tolerant = value;
        self
    }

    /// Sets transversal-operation metadata.
    #[must_use]
    pub const fn with_transversal(
        mut self,
        value: bool,
    ) -> Self {
        self.transversal = value;
        self
    }

    /// Sets magic-state dependency metadata.
    #[must_use]
    pub const fn with_magic_state(
        mut self,
        value: bool,
    ) -> Self {
        self.requires_magic_state = value;
        self
    }

    /// Sets decoder-assistance metadata.
    #[must_use]
    pub const fn with_decoder_assistance(
        mut self,
        value: bool,
    ) -> Self {
        self.decoder_assisted = value;
        self
    }

    /// Sets the estimated logical error rate.
    pub fn with_error_rate(
        mut self,
        rate: LogicalErrorRate,
    ) -> Self {
        self.logical_error_rate = Some(rate);
        self
    }

    /// Returns the stable operation name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the semantic operation kind.
    #[must_use]
    pub const fn kind(&self) -> LogicalOperationKind {
        self.kind
    }

    /// Returns the logical arity.
    #[must_use]
    pub const fn arity(&self) -> u32 {
        self.arity
    }

    /// Returns whether the operation is advertised as fault tolerant.
    #[must_use]
    pub const fn is_fault_tolerant(&self) -> bool {
        self.fault_tolerant
    }

    /// Returns whether the operation is transversal.
    #[must_use]
    pub const fn is_transversal(&self) -> bool {
        self.transversal
    }

    /// Returns whether a magic state is required.
    #[must_use]
    pub const fn requires_magic_state(&self) -> bool {
        self.requires_magic_state
    }

    /// Returns whether decoder assistance is required.
    #[must_use]
    pub const fn is_decoder_assisted(&self) -> bool {
        self.decoder_assisted
    }

    /// Returns the advertised logical error rate.
    #[must_use]
    pub const fn logical_error_rate(
        &self,
    ) -> Option<LogicalErrorRate> {
        self.logical_error_rate
    }
}

// =============================================================================
// Logical code descriptor
// =============================================================================

/// Provider-neutral description of an encoding/error-correction code.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub struct LogicalCodeDescriptor {
    /// Code family.
    family: LogicalCodeFamily,

    /// Physical encoding model.
    encoding: LogicalEncoding,

    /// Number of logical qubits encoded by one code block.
    logical_qubits_per_block: LogicalQubitCount,

    /// Number of physical qubits/modes required by one code block.
    physical_resources_per_block: PhysicalQubitCount,

    /// Code distance, when defined.
    distance: Option<CodeDistance>,

    /// Whether the code is advertised as fault tolerant.
    fault_tolerant: bool,

    /// Whether active error correction is required during execution.
    active_correction: bool,

    /// Whether syndrome extraction is required.
    syndrome_extraction: bool,

    /// Whether a classical decoder is required.
    decoder_required: bool,

    /// Whether logical operations are available natively at this encoding.
    native_logical_operations: bool,

    /// Optional logical error rate.
    logical_error_rate: Option<LogicalErrorRate>,

    /// Additional deterministic code parameters.
    parameters: BTreeMap<String, String>,
}

impl LogicalCodeDescriptor {
    /// Creates a code descriptor.
    pub fn new(
        family: LogicalCodeFamily,
        encoding: LogicalEncoding,
        logical_qubits_per_block: LogicalQubitCount,
        physical_resources_per_block: PhysicalQubitCount,
    ) -> Result<Self, LogicalHardwareError> {
        if logical_qubits_per_block.get() == 0 {
            return Err(
                LogicalHardwareError::ZeroLogicalCapacity,
            );
        }

        if physical_resources_per_block.get() == 0 {
            return Err(
                LogicalHardwareError::ZeroPhysicalCapacity,
            );
        }

        Ok(Self {
            family,
            encoding,
            logical_qubits_per_block,
            physical_resources_per_block,
            distance: None,
            fault_tolerant: false,
            active_correction: false,
            syndrome_extraction: false,
            decoder_required: false,
            native_logical_operations: false,
            logical_error_rate: None,
            parameters: BTreeMap::new(),
        })
    }

    /// Sets code distance.
    #[must_use]
    pub fn with_distance(
        mut self,
        distance: CodeDistance,
    ) -> Self {
        self.distance = Some(distance);
        self
    }

    /// Sets fault-tolerance metadata.
    #[must_use]
    pub const fn with_fault_tolerance(
        mut self,
        value: bool,
    ) -> Self {
        self.fault_tolerant = value;
        self
    }

    /// Sets active error-correction metadata.
    #[must_use]
    pub const fn with_active_correction(
        mut self,
        value: bool,
    ) -> Self {
        self.active_correction = value;
        self
    }

    /// Sets syndrome-extraction metadata.
    #[must_use]
    pub const fn with_syndrome_extraction(
        mut self,
        value: bool,
    ) -> Self {
        self.syndrome_extraction = value;
        self
    }

    /// Sets decoder requirement metadata.
    #[must_use]
    pub const fn with_decoder_required(
        mut self,
        value: bool,
    ) -> Self {
        self.decoder_required = value;
        self
    }

    /// Sets native logical-operation metadata.
    #[must_use]
    pub const fn with_native_logical_operations(
        mut self,
        value: bool,
    ) -> Self {
        self.native_logical_operations = value;
        self
    }

    /// Sets the logical error rate.
    #[must_use]
    pub fn with_logical_error_rate(
        mut self,
        rate: LogicalErrorRate,
    ) -> Self {
        self.logical_error_rate = Some(rate);
        self
    }

    /// Adds a deterministic code parameter.
    pub fn with_parameter(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self, LogicalHardwareError> {
        let key = normalize_name(
            &key.into(),
            MAX_METADATA_KEY_LENGTH,
            "code parameter key",
        )?;

        let value = normalize_name(
            &value.into(),
            MAX_METADATA_VALUE_LENGTH,
            "code parameter value",
        )?;

        if self.parameters.len() >= MAX_CODE_PARAMETERS
            && !self.parameters.contains_key(&key)
        {
            return Err(
                LogicalHardwareError::MetadataLimitExceeded {
                    maximum: MAX_CODE_PARAMETERS,
                },
            );
        }

        self.parameters.insert(key, value);
        Ok(self)
    }

    /// Returns the code family.
    #[must_use]
    pub const fn family(&self) -> LogicalCodeFamily {
        self.family
    }

    /// Returns the encoding.
    #[must_use]
    pub const fn encoding(&self) -> LogicalEncoding {
        self.encoding
    }

    /// Returns logical qubits per code block.
    #[must_use]
    pub const fn logical_qubits_per_block(
        &self,
    ) -> LogicalQubitCount {
        self.logical_qubits_per_block
    }

    /// Returns physical resources per code block.
    #[must_use]
    pub const fn physical_resources_per_block(
        &self,
    ) -> PhysicalQubitCount {
        self.physical_resources_per_block
    }

    /// Returns the code distance.
    #[must_use]
    pub const fn distance(
        &self,
    ) -> Option<CodeDistance> {
        self.distance
    }

    /// Returns whether the code is fault tolerant.
    #[must_use]
    pub const fn is_fault_tolerant(
        &self,
    ) -> bool {
        self.fault_tolerant
    }

    /// Returns whether active correction is required.
    #[must_use]
    pub const fn active_correction(
        &self,
    ) -> bool {
        self.active_correction
    }

    /// Returns whether syndrome extraction is required.
    #[must_use]
    pub const fn syndrome_extraction(
        &self,
    ) -> bool {
        self.syndrome_extraction
    }

    /// Returns whether a decoder is required.
    #[must_use]
    pub const fn decoder_required(
        &self,
    ) -> bool {
        self.decoder_required
    }

    /// Returns whether native logical operations are available.
    #[must_use]
    pub const fn native_logical_operations(
        &self,
    ) -> bool {
        self.native_logical_operations
    }

    /// Returns the logical error rate.
    #[must_use]
    pub const fn logical_error_rate(
        &self,
    ) -> Option<LogicalErrorRate> {
        self.logical_error_rate
    }

    /// Returns code parameters.
    #[must_use]
    pub fn parameters(
        &self,
    ) -> &BTreeMap<String, String> {
        &self.parameters
    }
}

// =============================================================================
// Logical hardware capabilities
// =============================================================================

/// Complete logical/fault-tolerant capability advertisement of one backend.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub struct LogicalHardwareCapabilities {
    /// Whether the backend exposes logical qubits.
    logical_qubits: bool,

    /// Number of logical qubits currently advertised.
    logical_qubit_capacity: LogicalQubitCount,

    /// Whether logical measurement is available.
    logical_measurement: bool,

    /// Whether logical reset is available.
    logical_reset: bool,

    /// Whether native logical operations are available.
    native_logical_operations: bool,

    /// Whether fault-tolerant execution is available.
    fault_tolerance: bool,

    /// Whether syndrome extraction is available.
    syndrome_measurement: bool,

    /// Whether decoder execution is available.
    decoder_execution: bool,

    /// Whether logical error rates are exposed.
    logical_error_rates: bool,

    /// Whether logical qubit mapping information is exposed.
    logical_mapping: bool,

    /// Whether logical operations can be performed concurrently.
    parallel_logical_operations: bool,

    /// Whether logical measurement can occur mid-program.
    mid_circuit_logical_measurement: bool,

    /// Whether logical feed-forward is supported.
    logical_classical_control: bool,

    /// Whether logical state preparation is supported.
    logical_state_preparation: bool,

    /// Whether logical operations are guaranteed fault tolerant.
    fault_tolerant_logical_operations: bool,

    /// Whether magic-state resources are supported.
    magic_state_support: bool,

    /// Whether logical T operations are supported.
    logical_t: bool,

    /// Whether logical Clifford operations are supported.
    logical_clifford: bool,

    /// Whether logical non-Clifford operations are supported.
    logical_non_clifford: bool,

    /// Whether logical resource metadata is versioned.
    versioned_logical_resources: bool,

    /// Code descriptors exposed by the backend.
    codes: Vec<LogicalCodeDescriptor>,

    /// Supported logical operations indexed by stable operation name.
    operations: BTreeMap<String, LogicalOperation>,
}

impl Default for LogicalHardwareCapabilities {
    fn default() -> Self {
        Self {
            logical_qubits: false,
            logical_qubit_capacity: LogicalQubitCount(0),
            logical_measurement: false,
            logical_reset: false,
            native_logical_operations: false,
            fault_tolerance: false,
            syndrome_measurement: false,
            decoder_execution: false,
            logical_error_rates: false,
            logical_mapping: false,
            parallel_logical_operations: false,
            mid_circuit_logical_measurement: false,
            logical_classical_control: false,
            logical_state_preparation: false,
            fault_tolerant_logical_operations: false,
            magic_state_support: false,
            logical_t: false,
            logical_clifford: false,
            logical_non_clifford: false,
            versioned_logical_resources: false,
            codes: Vec::new(),
            operations: BTreeMap::new(),
        }
    }
}

impl LogicalHardwareCapabilities {
    /// Creates an empty/conservative capability profile.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a capability profile for a logical backend.
    pub fn with_capacity(
        capacity: LogicalQubitCount,
    ) -> Result<Self, LogicalHardwareError> {
        if capacity.get() == 0 {
            return Err(
                LogicalHardwareError::ZeroLogicalCapacity,
            );
        }

        let mut result = Self::default();
        result.logical_qubits = true;
        result.logical_qubit_capacity = capacity;
        Ok(result)
    }

    /// Enables logical measurement.
    #[must_use]
    pub const fn with_logical_measurement(
        mut self,
        value: bool,
    ) -> Self {
        self.logical_measurement = value;
        self
    }

    /// Enables logical reset.
    #[must_use]
    pub const fn with_logical_reset(
        mut self,
        value: bool,
    ) -> Self {
        self.logical_reset = value;
        self
    }

    /// Enables native logical operations.
    #[must_use]
    pub const fn with_native_logical_operations(
        mut self,
        value: bool,
    ) -> Self {
        self.native_logical_operations = value;
        self
    }

    /// Enables fault tolerance.
    #[must_use]
    pub const fn with_fault_tolerance(
        mut self,
        value: bool,
    ) -> Self {
        self.fault_tolerance = value;
        self
    }

    /// Enables syndrome measurement.
    #[must_use]
    pub const fn with_syndrome_measurement(
        mut self,
        value: bool,
    ) -> Self {
        self.syndrome_measurement = value;
        self
    }

    /// Enables decoder execution.
    #[must_use]
    pub const fn with_decoder_execution(
        mut self,
        value: bool,
    ) -> Self {
        self.decoder_execution = value;
        self
    }

    /// Enables logical error-rate reporting.
    #[must_use]
    pub const fn with_logical_error_rates(
        mut self,
        value: bool,
    ) -> Self {
        self.logical_error_rates = value;
        self
    }

    /// Enables logical mapping information.
    #[must_use]
    pub const fn with_logical_mapping(
        mut self,
        value: bool,
    ) -> Self {
        self.logical_mapping = value;
        self
    }

    /// Enables parallel logical operations.
    #[must_use]
    pub const fn with_parallel_operations(
        mut self,
        value: bool,
    ) -> Self {
        self.parallel_logical_operations = value;
        self
    }

    /// Enables mid-circuit logical measurement.
    #[must_use]
    pub const fn with_mid_circuit_measurement(
        mut self,
        value: bool,
    ) -> Self {
        self.mid_circuit_logical_measurement = value;
        self
    }

    /// Enables logical classical control/feed-forward.
    #[must_use]
    pub const fn with_classical_control(
        mut self,
        value: bool,
    ) -> Self {
        self.logical_classical_control = value;
        self
    }

    /// Enables logical state preparation.
    #[must_use]
    pub const fn with_state_preparation(
        mut self,
        value: bool,
    ) -> Self {
        self.logical_state_preparation = value;
        self
    }

    /// Enables fault-tolerant logical operations.
    #[must_use]
    pub const fn with_fault_tolerant_operations(
        mut self,
        value: bool,
    ) -> Self {
        self.fault_tolerant_logical_operations = value;
        self
    }

    /// Enables magic-state support.
    #[must_use]
    pub const fn with_magic_state_support(
        mut self,
        value: bool,
    ) -> Self {
        self.magic_state_support = value;
        self
    }

    /// Enables logical T operations.
    #[must_use]
    pub const fn with_logical_t(
        mut self,
        value: bool,
    ) -> Self {
        self.logical_t = value;
        self
    }

    /// Enables logical Clifford operations.
    #[must_use]
    pub const fn with_logical_clifford(
        mut self,
        value: bool,
    ) -> Self {
        self.logical_clifford = value;
        self
    }

    /// Enables logical non-Clifford operations.
    #[must_use]
    pub const fn with_logical_non_clifford(
        mut self,
        value: bool,
    ) -> Self {
        self.logical_non_clifford = value;
        self
    }

    /// Marks logical-resource metadata as versioned.
    #[must_use]
    pub const fn with_versioned_resources(
        mut self,
        value: bool,
    ) -> Self {
        self.versioned_logical_resources = value;
        self
    }

    /// Adds a code descriptor.
    pub fn add_code(
        &mut self,
        code: LogicalCodeDescriptor,
    ) -> Result<(), LogicalHardwareError> {
        if self.codes.len() >= 1024 {
            return Err(
                LogicalHardwareError::CodeDescriptorLimitExceeded {
                    maximum: 1024,
                },
            );
        }

        self.codes.push(code);
        Ok(())
    }

    /// Adds a logical operation.
    pub fn add_operation(
        &mut self,
        operation: LogicalOperation,
    ) -> Result<(), LogicalHardwareError> {
        if self.operations.len() >= MAX_LOGICAL_OPERATIONS
            && !self.operations.contains_key(operation.name())
        {
            return Err(
                LogicalHardwareError::OperationLimitExceeded {
                    maximum: MAX_LOGICAL_OPERATIONS,
                },
            );
        }

        let name = operation.name().to_owned();

        if self.operations.contains_key(&name) {
            return Err(
                LogicalHardwareError::DuplicateOperation {
                    name,
                },
            );
        }

        self.operations.insert(name, operation);
        Ok(())
    }

    /// Returns whether logical qubits are supported.
    #[must_use]
    pub const fn logical_qubits(&self) -> bool {
        self.logical_qubits
    }

    /// Returns logical capacity.
    #[must_use]
    pub const fn logical_qubit_capacity(
        &self,
    ) -> LogicalQubitCount {
        self.logical_qubit_capacity
    }

    /// Returns whether logical measurement is supported.
    #[must_use]
    pub const fn logical_measurement(&self) -> bool {
        self.logical_measurement
    }

    /// Returns whether logical reset is supported.
    #[must_use]
    pub const fn logical_reset(&self) -> bool {
        self.logical_reset
    }

    /// Returns whether native logical operations are supported.
    #[must_use]
    pub const fn native_logical_operations(
        &self,
    ) -> bool {
        self.native_logical_operations
    }

    /// Returns whether fault tolerance is supported.
    #[must_use]
    pub const fn fault_tolerance(&self) -> bool {
        self.fault_tolerance
    }

    /// Returns whether syndrome measurement is supported.
    #[must_use]
    pub const fn syndrome_measurement(&self) -> bool {
        self.syndrome_measurement
    }

    /// Returns whether decoder execution is supported.
    #[must_use]
    pub const fn decoder_execution(&self) -> bool {
        self.decoder_execution
    }

    /// Returns whether logical error rates are exposed.
    #[must_use]
    pub const fn logical_error_rates(&self) -> bool {
        self.logical_error_rates
    }

    /// Returns whether logical mapping is exposed.
    #[must_use]
    pub const fn logical_mapping(&self) -> bool {
        self.logical_mapping
    }

    /// Returns whether parallel logical operations are supported.
    #[must_use]
    pub const fn parallel_logical_operations(
        &self,
    ) -> bool {
        self.parallel_logical_operations
    }

    /// Returns whether mid-circuit logical measurement is supported.
    #[must_use]
    pub const fn mid_circuit_logical_measurement(
        &self,
    ) -> bool {
        self.mid_circuit_logical_measurement
    }

    /// Returns whether logical classical control is supported.
    #[must_use]
    pub const fn logical_classical_control(
        &self,
    ) -> bool {
        self.logical_classical_control
    }

    /// Returns whether logical state preparation is supported.
    #[must_use]
    pub const fn logical_state_preparation(
        &self,
    ) -> bool {
        self.logical_state_preparation
    }

    /// Returns whether fault-tolerant logical operations are supported.
    #[must_use]
    pub const fn fault_tolerant_logical_operations(
        &self,
    ) -> bool {
        self.fault_tolerant_logical_operations
    }

    /// Returns whether magic-state support is available.
    #[must_use]
    pub const fn magic_state_support(
        &self,
    ) -> bool {
        self.magic_state_support
    }

    /// Returns whether logical T is supported.
    #[must_use]
    pub const fn logical_t(&self) -> bool {
        self.logical_t
    }

    /// Returns whether logical Clifford operations are supported.
    #[must_use]
    pub const fn logical_clifford(&self) -> bool {
        self.logical_clifford
    }

    /// Returns whether logical non-Clifford operations are supported.
    #[must_use]
    pub const fn logical_non_clifford(
        &self,
    ) -> bool {
        self.logical_non_clifford
    }

    /// Returns whether resource metadata is versioned.
    #[must_use]
    pub const fn versioned_logical_resources(
        &self,
    ) -> bool {
        self.versioned_logical_resources
    }

    /// Returns advertised codes.
    #[must_use]
    pub fn codes(&self) -> &[LogicalCodeDescriptor] {
        &self.codes
    }

    /// Returns advertised logical operations.
    #[must_use]
    pub fn operations(
        &self,
    ) -> &BTreeMap<String, LogicalOperation> {
        &self.operations
    }

    /// Finds an operation by stable name.
    #[must_use]
    pub fn operation(
        &self,
        name: &str,
    ) -> Option<&LogicalOperation> {
        self.operations.get(name)
    }

    /// Validates the entire capability advertisement.
    pub fn validate(&self) -> Result<(), LogicalHardwareError> {
        if self.logical_qubits
            && self.logical_qubit_capacity.get() == 0
        {
            return Err(
                LogicalHardwareError::CapabilityContradiction {
                    field: "logical_qubit_capacity",
                    reason: "logical_qubits is true but capacity is zero",
                },
            );
        }

        if self.fault_tolerance
            && self.codes.is_empty()
        {
            return Err(
                LogicalHardwareError::CapabilityContradiction {
                    field: "codes",
                    reason: "fault tolerance is advertised without a code descriptor",
                },
            );
        }

        if self.decoder_execution
            && !self.syndrome_measurement
        {
            return Err(
                LogicalHardwareError::CapabilityContradiction {
                    field: "decoder_execution",
                    reason: "decoder execution requires syndrome measurement capability",
                },
            );
        }

        if self.logical_t
            && !self.logical_non_clifford
        {
            return Err(
                LogicalHardwareError::CapabilityContradiction {
                    field: "logical_t",
                    reason: "logical T requires logical non-Clifford capability",
                },
            );
        }

        if self.fault_tolerant_logical_operations
            && !self.fault_tolerance
        {
            return Err(
                LogicalHardwareError::CapabilityContradiction {
                    field: "fault_tolerant_logical_operations",
                    reason: "fault-tolerant logical operations require fault tolerance",
                },
            );
        }

        for code in &self.codes {
            if code.logical_qubits_per_block().get() == 0
                || code.physical_resources_per_block().get() == 0
            {
                return Err(
                    LogicalHardwareError::InvalidCodeDescriptor,
                );
            }
        }

        for operation in self.operations.values() {
            if operation.arity() == 0 {
                return Err(
                    LogicalHardwareError::InvalidArity {
                        operation: operation.name().to_owned(),
                        arity: 0,
                    },
                );
            }

            if operation.is_fault_tolerant()
                && !self.fault_tolerance
            {
                return Err(
                    LogicalHardwareError::CapabilityContradiction {
                        field: "operations",
                        reason:
                            "a fault-tolerant operation is advertised without backend fault tolerance",
                    },
                );
            }

            if operation.requires_magic_state()
                && !self.magic_state_support
            {
                return Err(
                    LogicalHardwareError::CapabilityContradiction {
                        field: "operations",
                        reason:
                            "an operation requires magic states but magic-state support is disabled",
                    },
                );
            }
        }

        Ok(())
    }
}

// =============================================================================
// Logical resource
// =============================================================================

/// A concrete logical-qubit resource exposed by a backend.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub struct LogicalQubitResource {
    /// Stable logical-qubit identity.
    id: LogicalQubitId,

    /// Hardware/device reference exposing this resource.
    hardware: HardwareReference,

    /// Code used by the resource.
    code: LogicalCodeDescriptor,

    /// Number of physical resources backing this logical resource.
    physical_resource_count: PhysicalQubitCount,

    /// Optional physical resource identifiers.
    ///
    /// This field is intentionally opaque: topology ownership remains in
    /// `topology.rs`.
    physical_resource_ids: Vec<String>,

    /// Current logical error rate, if available.
    logical_error_rate: Option<LogicalErrorRate>,

    /// Whether this logical resource is currently available for execution.
    available: bool,

    /// Whether the resource is reserved.
    reserved: bool,
}

impl LogicalQubitResource {
    /// Creates a validated logical resource.
    pub fn new(
        id: LogicalQubitId,
        hardware: HardwareReference,
        code: LogicalCodeDescriptor,
    ) -> Result<Self, LogicalHardwareError> {
        let physical_count =
            code.physical_resources_per_block();

        Ok(Self {
            id,
            hardware,
            code,
            physical_resource_count: physical_count,
            physical_resource_ids: Vec::new(),
            logical_error_rate: None,
            available: true,
            reserved: false,
        })
    }

    /// Sets the physical resource identifiers.
    ///
    /// This method does not validate topology semantics. It only validates
    /// identifiers and count consistency. Physical connectivity remains owned
    /// by `topology.rs`.
    pub fn with_physical_resource_ids(
        mut self,
        ids: Vec<String>,
    ) -> Result<Self, LogicalHardwareError> {
        if ids.len() as u64
            != self.physical_resource_count.get()
        {
            return Err(
                LogicalHardwareError::PhysicalResourceCountMismatch {
                    expected: self
                        .physical_resource_count
                        .get(),
                    actual: ids.len() as u64,
                },
            );
        }

        let mut normalized = Vec::with_capacity(ids.len());

        for id in ids {
            let id = normalize_name(
                &id,
                MAX_REFERENCE_LENGTH,
                "physical resource identifier",
            )?;

            normalized.push(id);
        }

        self.physical_resource_ids = normalized;
        Ok(self)
    }

    /// Sets the current logical error rate.
    #[must_use]
    pub fn with_error_rate(
        mut self,
        rate: LogicalErrorRate,
    ) -> Self {
        self.logical_error_rate = Some(rate);
        self
    }

    /// Sets availability.
    #[must_use]
    pub const fn with_available(
        mut self,
        value: bool,
    ) -> Self {
        self.available = value;
        self
    }

    /// Sets reservation state.
    #[must_use]
    pub const fn with_reserved(
        mut self,
        value: bool,
    ) -> Self {
        self.reserved = value;
        self
    }

    /// Returns the logical identifier.
    #[must_use]
    pub fn id(&self) -> &LogicalQubitId {
        &self.id
    }

    /// Returns the hardware reference.
    #[must_use]
    pub fn hardware(&self) -> &HardwareReference {
        &self.hardware
    }

    /// Returns the code descriptor.
    #[must_use]
    pub fn code(&self) -> &LogicalCodeDescriptor {
        &self.code
    }

    /// Returns the physical resource count.
    #[must_use]
    pub const fn physical_resource_count(
        &self,
    ) -> PhysicalQubitCount {
        self.physical_resource_count
    }

    /// Returns physical resource identifiers.
    #[must_use]
    pub fn physical_resource_ids(
        &self,
    ) -> &[String] {
        &self.physical_resource_ids
    }

    /// Returns the logical error rate.
    #[must_use]
    pub const fn logical_error_rate(
        &self,
    ) -> Option<LogicalErrorRate> {
        self.logical_error_rate
    }

    /// Returns whether the resource is available.
    #[must_use]
    pub const fn is_available(&self) -> bool {
        self.available
    }

    /// Returns whether the resource is reserved.
    #[must_use]
    pub const fn is_reserved(&self) -> bool {
        self.reserved
    }

    /// Returns whether this resource can currently be allocated.
    #[must_use]
    pub const fn is_allocatable(&self) -> bool {
        self.available && !self.reserved
    }

    /// Validates the complete resource.
    pub fn validate(&self) -> Result<(), LogicalHardwareError> {
        if self.code.logical_qubits_per_block().get() == 0 {
            return Err(
                LogicalHardwareError::ZeroLogicalCapacity,
            );
        }

        if self.physical_resource_count.get() == 0 {
            return Err(
                LogicalHardwareError::ZeroPhysicalCapacity,
            );
        }

        if !self.physical_resource_ids.is_empty()
            && self.physical_resource_ids.len() as u64
                != self.physical_resource_count.get()
        {
            return Err(
                LogicalHardwareError::PhysicalResourceCountMismatch {
                    expected: self
                        .physical_resource_count
                        .get(),
                    actual: self
                        .physical_resource_ids
                        .len() as u64,
                },
            );
        }

        if self.reserved && !self.available {
            // This state is meaningful: a resource can be unavailable while a
            // reservation is retained. It is therefore intentionally valid.
        }

        Ok(())
    }
}

// =============================================================================
// Logical resource estimate
// =============================================================================

/// Resource estimate for executing a logical workload.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub struct LogicalResourceEstimate {
    /// Required logical qubits.
    required_logical_qubits: LogicalQubitCount,

    /// Estimated physical resources required.
    estimated_physical_resources: PhysicalQubitCount,

    /// Optional code distance.
    code_distance: Option<CodeDistance>,

    /// Optional estimated logical error probability.
    estimated_logical_error_rate: Option<LogicalErrorRate>,

    /// Whether the estimate is exact or heuristic.
    exact: bool,

    /// Deterministic explanation.
    rationale: String,
}

impl LogicalResourceEstimate {
    /// Creates an estimate.
    pub fn new(
        required_logical_qubits: LogicalQubitCount,
        estimated_physical_resources: PhysicalQubitCount,
        exact: bool,
        rationale: impl Into<String>,
    ) -> Result<Self, LogicalHardwareError> {
        let rationale = normalize_name(
            &rationale.into(),
            MAX_METADATA_VALUE_LENGTH,
            "resource estimate rationale",
        )?;

        if required_logical_qubits.get() == 0 {
            return Err(
                LogicalHardwareError::ZeroLogicalCapacity,
            );
        }

        if estimated_physical_resources.get() == 0 {
            return Err(
                LogicalHardwareError::ZeroPhysicalCapacity,
            );
        }

        Ok(Self {
            required_logical_qubits,
            estimated_physical_resources,
            code_distance: None,
            estimated_logical_error_rate: None,
            exact,
            rationale,
        })
    }

    /// Adds code-distance information.
    #[must_use]
    pub fn with_code_distance(
        mut self,
        distance: CodeDistance,
    ) -> Self {
        self.code_distance = Some(distance);
        self
    }

    /// Adds an estimated logical error rate.
    #[must_use]
    pub fn with_error_rate(
        mut self,
        rate: LogicalErrorRate,
    ) -> Self {
        self.estimated_logical_error_rate = Some(rate);
        self
    }

    /// Returns required logical qubits.
    #[must_use]
    pub const fn required_logical_qubits(
        &self,
    ) -> LogicalQubitCount {
        self.required_logical_qubits
    }

    /// Returns estimated physical resources.
    #[must_use]
    pub const fn estimated_physical_resources(
        &self,
    ) -> PhysicalQubitCount {
        self.estimated_physical_resources
    }

    /// Returns code distance.
    #[must_use]
    pub const fn code_distance(
        &self,
    ) -> Option<CodeDistance> {
        self.code_distance
    }

    /// Returns estimated logical error rate.
    #[must_use]
    pub const fn estimated_logical_error_rate(
        &self,
    ) -> Option<LogicalErrorRate> {
        self.estimated_logical_error_rate
    }

    /// Returns whether the estimate is exact.
    #[must_use]
    pub const fn is_exact(&self) -> bool {
        self.exact
    }

    /// Returns the rationale.
    #[must_use]
    pub fn rationale(&self) -> &str {
        &self.rationale
    }
}

// =============================================================================
// Logical workload requirements
// =============================================================================

/// Requirements imposed by a logical quantum workload.
///
/// This type is deliberately independent of any backend implementation.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub struct LogicalWorkloadRequirements {
    /// Required logical qubit count.
    logical_qubits: LogicalQubitCount,

    /// Required code family, if any.
    required_code_family: Option<LogicalCodeFamily>,

    /// Required code distance, if any.
    minimum_code_distance: Option<CodeDistance>,

    /// Required fault tolerance.
    fault_tolerant: bool,

    /// Required logical measurement.
    logical_measurement: bool,

    /// Required logical reset.
    logical_reset: bool,

    /// Required mid-circuit logical measurement.
    mid_circuit_measurement: bool,

    /// Required logical classical control.
    classical_control: bool,

    /// Required decoder support.
    decoder_execution: bool,

    /// Required syndrome extraction.
    syndrome_measurement: bool,

    /// Required magic-state support.
    magic_state_support: bool,

    /// Required logical T operation.
    logical_t: bool,

    /// Required logical Clifford support.
    logical_clifford: bool,

    /// Required logical non-Clifford support.
    logical_non_clifford: bool,

    /// Required operation names.
    required_operations: Vec<String>,

    /// Optional maximum tolerated logical error rate.
    maximum_logical_error_rate: Option<LogicalErrorRate>,
}

impl LogicalWorkloadRequirements {
    /// Creates requirements for a logical workload.
    pub fn new(
        logical_qubits: LogicalQubitCount,
    ) -> Result<Self, LogicalHardwareError> {
        if logical_qubits.get() == 0 {
            return Err(
                LogicalHardwareError::ZeroLogicalCapacity,
            );
        }

        Ok(Self {
            logical_qubits,
            required_code_family: None,
            minimum_code_distance: None,
            fault_tolerant: false,
            logical_measurement: false,
            logical_reset: false,
            mid_circuit_measurement: false,
            classical_control: false,
            decoder_execution: false,
            syndrome_measurement: false,
            magic_state_support: false,
            logical_t: false,
            logical_clifford: false,
            logical_non_clifford: false,
            required_operations: Vec::new(),
            maximum_logical_error_rate: None,
        })
    }

    /// Requires a particular code family.
    #[must_use]
    pub const fn with_code_family(
        mut self,
        family: LogicalCodeFamily,
    ) -> Self {
        self.required_code_family = Some(family);
        self
    }

    /// Requires a minimum code distance.
    #[must_use]
    pub const fn with_minimum_distance(
        mut self,
        distance: CodeDistance,
    ) -> Self {
        self.minimum_code_distance = Some(distance);
        self
    }

    /// Requires fault tolerance.
    #[must_use]
    pub const fn with_fault_tolerance(
        mut self,
        value: bool,
    ) -> Self {
        self.fault_tolerant = value;
        self
    }

    /// Requires logical measurement.
    #[must_use]
    pub const fn with_measurement(
        mut self,
        value: bool,
    ) -> Self {
        self.logical_measurement = value;
        self
    }

    /// Requires logical reset.
    #[must_use]
    pub const fn with_reset(
        mut self,
        value: bool,
    ) -> Self {
        self.logical_reset = value;
        self
    }

    /// Requires mid-circuit measurement.
    #[must_use]
    pub const fn with_mid_circuit_measurement(
        mut self,
        value: bool,
    ) -> Self {
        self.mid_circuit_measurement = value;
        self
    }

    /// Requires classical feed-forward.
    #[must_use]
    pub const fn with_classical_control(
        mut self,
        value: bool,
    ) -> Self {
        self.classical_control = value;
        self
    }

    /// Requires decoder execution.
    #[must_use]
    pub const fn with_decoder_execution(
        mut self,
        value: bool,
    ) -> Self {
        self.decoder_execution = value;
        self
    }

    /// Requires syndrome measurement.
    #[must_use]
    pub const fn with_syndrome_measurement(
        mut self,
        value: bool,
    ) -> Self {
        self.syndrome_measurement = value;
        self
    }

    /// Requires magic-state support.
    #[must_use]
    pub const fn with_magic_state_support(
        mut self,
        value: bool,
    ) -> Self {
        self.magic_state_support = value;
        self
    }

    /// Requires logical T.
    #[must_use]
    pub const fn with_logical_t(
        mut self,
        value: bool,
    ) -> Self {
        self.logical_t = value;
        self
    }

    /// Requires logical Clifford operations.
    #[must_use]
    pub const fn with_logical_clifford(
        mut self,
        value: bool,
    ) -> Self {
        self.logical_clifford = value;
        self
    }

    /// Requires logical non-Clifford operations.
    #[must_use]
    pub const fn with_logical_non_clifford(
        mut self,
        value: bool,
    ) -> Self {
        self.logical_non_clifford = value;
        self
    }

    /// Requires a logical operation.
    pub fn require_operation(
        mut self,
        name: impl Into<String>,
    ) -> Result<Self, LogicalHardwareError> {
        let name = normalize_name(
            &name.into(),
            MAX_OPERATION_NAME_LENGTH,
            "required logical operation",
        )?;

        if self.required_operations.len()
            >= MAX_LOGICAL_OPERATIONS
            && !self.required_operations.contains(&name)
        {
            return Err(
                LogicalHardwareError::OperationLimitExceeded {
                    maximum: MAX_LOGICAL_OPERATIONS,
                },
            );
        }

        if !self.required_operations.contains(&name) {
            self.required_operations.push(name);
            self.required_operations.sort();
        }

        Ok(self)
    }

    /// Sets the maximum tolerated logical error rate.
    #[must_use]
    pub const fn with_maximum_error_rate(
        mut self,
        rate: LogicalErrorRate,
    ) -> Self {
        self.maximum_logical_error_rate = Some(rate);
        self
    }

    /// Returns required logical qubits.
    #[must_use]
    pub const fn logical_qubits(
        &self,
    ) -> LogicalQubitCount {
        self.logical_qubits
    }

    /// Returns required code family.
    #[must_use]
    pub const fn required_code_family(
        &self,
    ) -> Option<LogicalCodeFamily> {
        self.required_code_family
    }

    /// Returns minimum code distance.
    #[must_use]
    pub const fn minimum_code_distance(
        &self,
    ) -> Option<CodeDistance> {
        self.minimum_code_distance
    }

    /// Returns whether fault tolerance is required.
    #[must_use]
    pub const fn fault_tolerant(
        &self,
    ) -> bool {
        self.fault_tolerant
    }

    /// Returns whether logical measurement is required.
    #[must_use]
    pub const fn logical_measurement(
        &self,
    ) -> bool {
        self.logical_measurement
    }

    /// Returns whether logical reset is required.
    #[must_use]
    pub const fn logical_reset(
        &self,
    ) -> bool {
        self.logical_reset
    }

    /// Returns whether mid-circuit logical measurement is required.
    #[must_use]
    pub const fn mid_circuit_measurement(
        &self,
    ) -> bool {
        self.mid_circuit_measurement
    }

    /// Returns whether classical control is required.
    #[must_use]
    pub const fn classical_control(
        &self,
    ) -> bool {
        self.classical_control
    }

    /// Returns whether decoder execution is required.
    #[must_use]
    pub const fn decoder_execution(
        &self,
    ) -> bool {
        self.decoder_execution
    }

    /// Returns whether syndrome measurement is required.
    #[must_use]
    pub const fn syndrome_measurement(
        &self,
    ) -> bool {
        self.syndrome_measurement
    }

    /// Returns whether magic-state support is required.
    #[must_use]
    pub const fn magic_state_support(
        &self,
    ) -> bool {
        self.magic_state_support
    }

    /// Returns whether logical T is required.
    #[must_use]
    pub const fn logical_t(
        &self,
    ) -> bool {
        self.logical_t
    }

    /// Returns whether logical Clifford support is required.
    #[must_use]
    pub const fn logical_clifford(
        &self,
    ) -> bool {
        self.logical_clifford
    }

    /// Returns whether logical non-Clifford support is required.
    #[must_use]
    pub const fn logical_non_clifford(
        &self,
    ) -> bool {
        self.logical_non_clifford
    }

    /// Returns required operations in deterministic order.
    #[must_use]
    pub fn required_operations(
        &self,
    ) -> &[String] {
        &self.required_operations
    }

    /// Returns the maximum tolerated logical error rate.
    #[must_use]
    pub const fn maximum_logical_error_rate(
        &self,
    ) -> Option<LogicalErrorRate> {
        self.maximum_logical_error_rate
    }
}

// =============================================================================
// Compatibility
// =============================================================================

/// Result of comparing logical workload requirements with hardware.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub enum LogicalCompatibility {
    /// Hardware satisfies all requirements.
    Compatible,

    /// Hardware satisfies requirements but has warnings.
    CompatibleWithWarnings,

    /// Hardware requires a transformation before execution.
    RequiresTransformation,

    /// Hardware cannot satisfy the requirements.
    Incompatible,
}

impl LogicalCompatibility {
    /// Returns whether execution is directly possible.
    #[must_use]
    pub const fn is_compatible(
        self,
    ) -> bool {
        matches!(
            self,
            Self::Compatible
                | Self::CompatibleWithWarnings
        )
    }

    /// Returns whether transformation is required.
    #[must_use]
    pub const fn requires_transformation(
        self,
    ) -> bool {
        matches!(
            self,
            Self::RequiresTransformation
        )
    }

    /// Returns whether execution is impossible under the supplied contract.
    #[must_use]
    pub const fn is_incompatible(
        self,
    ) -> bool {
        matches!(self, Self::Incompatible)
    }
}

/// One compatibility diagnostic.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub struct LogicalCompatibilityDiagnostic {
    /// Stable diagnostic code.
    pub code: &'static str,

    /// Human-readable explanation.
    pub message: String,

    /// Whether the diagnostic prevents direct execution.
    pub blocking: bool,
}

impl LogicalCompatibilityDiagnostic {
    fn blocking(
        code: &'static str,
        message: impl Into<String>,
    ) -> Self {
        Self {
            code,
            message: message.into(),
            blocking: true,
        }
    }

    fn warning(
        code: &'static str,
        message: impl Into<String>,
    ) -> Self {
        Self {
            code,
            message: message.into(),
            blocking: false,
        }
    }
}

/// Complete compatibility report.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub struct LogicalCompatibilityReport {
    /// Final compatibility classification.
    pub status: LogicalCompatibility,

    /// Deterministically ordered diagnostics.
    pub diagnostics: Vec<LogicalCompatibilityDiagnostic>,
}

impl LogicalCompatibilityReport {
    /// Creates a report.
    #[must_use]
    pub fn new(
        status: LogicalCompatibility,
        mut diagnostics: Vec<
            LogicalCompatibilityDiagnostic,
        >,
    ) -> Self {
        diagnostics.sort_by(|left, right| {
            left.code
                .cmp(right.code)
                .then_with(|| {
                    left.message.cmp(&right.message)
                })
        });

        Self {
            status,
            diagnostics,
        }
    }

    /// Returns whether the workload is executable without transformation.
    #[must_use]
    pub const fn is_compatible(
        &self,
    ) -> bool {
        self.status.is_compatible()
    }
}

/// Checks logical workload requirements against hardware capabilities.
#[must_use]
pub fn check_compatibility(
    requirements: &LogicalWorkloadRequirements,
    capabilities: &LogicalHardwareCapabilities,
) -> LogicalCompatibilityReport {
    let mut diagnostics = Vec::new();

    if let Err(error) = capabilities.validate() {
        diagnostics.push(
            LogicalCompatibilityDiagnostic::blocking(
                "LOGICAL-HARDWARE-INVALID",
                error.to_string(),
            ),
        );

        return LogicalCompatibilityReport::new(
            LogicalCompatibility::Incompatible,
            diagnostics,
        );
    }

    if capabilities.logical_qubit_capacity().get()
        < requirements.logical_qubits().get()
    {
        diagnostics.push(
            LogicalCompatibilityDiagnostic::blocking(
                "LOGICAL-QUBIT-CAPACITY",
                format!(
                    "workload requires {} logical qubits but backend exposes {}",
                    requirements.logical_qubits().get(),
                    capabilities
                        .logical_qubit_capacity()
                        .get()
                ),
            ),
        );
    }

    if requirements.logical_measurement()
        && !capabilities.logical_measurement()
    {
        diagnostics.push(
            LogicalCompatibilityDiagnostic::blocking(
                "LOGICAL-MEASUREMENT",
                "logical measurement is required but unsupported",
            ),
        );
    }

    if requirements.logical_reset()
        && !capabilities.logical_reset()
    {
        diagnostics.push(
            LogicalCompatibilityDiagnostic::blocking(
                "LOGICAL-RESET",
                "logical reset is required but unsupported",
            ),
        );
    }

    if requirements.mid_circuit_measurement()
        && !capabilities
            .mid_circuit_logical_measurement()
    {
        diagnostics.push(
            LogicalCompatibilityDiagnostic::blocking(
                "LOGICAL-MID-CIRCUIT-MEASUREMENT",
                "mid-circuit logical measurement is required but unsupported",
            ),
        );
    }

    if requirements.classical_control()
        && !capabilities.logical_classical_control()
    {
        diagnostics.push(
            LogicalCompatibilityDiagnostic::blocking(
                "LOGICAL-CLASSICAL-CONTROL",
                "logical classical feed-forward is required but unsupported",
            ),
        );
    }

    if requirements.decoder_execution()
        && !capabilities.decoder_execution()
    {
        diagnostics.push(
            LogicalCompatibilityDiagnostic::blocking(
                "LOGICAL-DECODER",
                "decoder execution is required but unsupported",
            ),
        );
    }

    if requirements.syndrome_measurement()
        && !capabilities.syndrome_measurement()
    {
        diagnostics.push(
            LogicalCompatibilityDiagnostic::blocking(
                "LOGICAL-SYNDROME",
                "syndrome measurement is required but unsupported",
            ),
        );
    }

    if requirements.fault_tolerant()
        && !capabilities.fault_tolerance()
    {
        diagnostics.push(
            LogicalCompatibilityDiagnostic::blocking(
                "FAULT-TOLERANCE",
                "fault-tolerant execution is required but unsupported",
            ),
        );
    }

    if requirements.magic_state_support()
        && !capabilities.magic_state_support()
    {
        diagnostics.push(
            LogicalCompatibilityDiagnostic::blocking(
                "MAGIC-STATE",
                "magic-state support is required but unsupported",
            ),
        );
    }

    if requirements.logical_t()
        && !capabilities.logical_t()
    {
        diagnostics.push(
            LogicalCompatibilityDiagnostic::blocking(
                "LOGICAL-T",
                "logical T operation is required but unsupported",
            ),
        );
    }

    if requirements.logical_clifford()
        && !capabilities.logical_clifford()
    {
        diagnostics.push(
            LogicalCompatibilityDiagnostic::blocking(
                "LOGICAL-CLIFFORD",
                "logical Clifford operations are required but unsupported",
            ),
        );
    }

    if requirements.logical_non_clifford()
        && !capabilities.logical_non_clifford()
    {
        diagnostics.push(
            LogicalCompatibilityDiagnostic::blocking(
                "LOGICAL-NON-CLIFFORD",
                "logical non-Clifford operations are required but unsupported",
            ),
        );
    }

    if let Some(required_family) =
        requirements.required_code_family()
    {
        let found = capabilities
            .codes()
            .iter()
            .any(|code| {
                code.family() == required_family
            });

        if !found {
            diagnostics.push(
                LogicalCompatibilityDiagnostic::blocking(
                    "LOGICAL-CODE-FAMILY",
                    format!(
                        "required code family '{}' is not advertised",
                        required_family
                    ),
                ),
            );
        }
    }

    if let Some(required_distance) =
        requirements.minimum_code_distance()
    {
        let found = capabilities
            .codes()
            .iter()
            .any(|code| {
                code.distance()
                    .map(|distance| {
                        distance.get()
                            >= required_distance.get()
                    })
                    .unwrap_or(false)
            });

        if !found {
            diagnostics.push(
                LogicalCompatibilityDiagnostic::blocking(
                    "LOGICAL-CODE-DISTANCE",
                    format!(
                        "no advertised code satisfies minimum distance {}",
                        required_distance.get()
                    ),
                ),
            );
        }
    }

    for operation in
        requirements.required_operations()
    {
        match capabilities.operation(operation) {
            Some(_) => {}
            None => diagnostics.push(
                LogicalCompatibilityDiagnostic::blocking(
                    "LOGICAL-OPERATION",
                    format!(
                        "required logical operation '{}' is unsupported",
                        operation
                    ),
                ),
            ),
        }
    }

    if let Some(maximum) =
        requirements.maximum_logical_error_rate()
    {
        let advertised_rates: Vec<
            LogicalErrorRate,
        > = capabilities
            .codes()
            .iter()
            .filter_map(|code| {
                code.logical_error_rate()
            })
            .collect();

        if advertised_rates.is_empty() {
            diagnostics.push(
                LogicalCompatibilityDiagnostic::warning(
                    "LOGICAL-ERROR-RATE-UNKNOWN",
                    "workload constrains logical error rate but backend exposes no logical error-rate estimate",
                ),
            );
        } else if !advertised_rates
            .iter()
            .any(|rate| rate.get() <= maximum.get())
        {
            diagnostics.push(
                LogicalCompatibilityDiagnostic::blocking(
                    "LOGICAL-ERROR-RATE",
                    format!(
                        "no advertised logical resource satisfies maximum logical error rate {}",
                        maximum.get()
                    ),
                ),
            );
        }
    }

    let has_blocking = diagnostics
        .iter()
        .any(|diagnostic| diagnostic.blocking);

    let has_warning = diagnostics
        .iter()
        .any(|diagnostic| !diagnostic.blocking);

    let status = if has_blocking {
        LogicalCompatibility::Incompatible
    } else if has_warning {
        LogicalCompatibility::CompatibleWithWarnings
    } else {
        LogicalCompatibility::Compatible
    };

    LogicalCompatibilityReport::new(
        status,
        diagnostics,
    )
}

// =============================================================================
// Logical execution request
// =============================================================================

/// Provider-neutral request to execute a logical workload.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub struct LogicalExecutionRequest {
    /// Target hardware.
    hardware: HardwareReference,

    /// Workload requirements.
    requirements: LogicalWorkloadRequirements,

    /// Optional explicit logical-qubit allocation.
    logical_qubits: Vec<LogicalQubitId>,

    /// Whether automatic allocation is allowed.
    automatic_allocation: bool,

    /// Whether the caller requires pre-execution compatibility validation.
    require_validation: bool,
}

impl LogicalExecutionRequest {
    /// Creates a request.
    pub fn new(
        hardware: HardwareReference,
        requirements: LogicalWorkloadRequirements,
    ) -> Self {
        Self {
            hardware,
            requirements,
            logical_qubits: Vec::new(),
            automatic_allocation: true,
            require_validation: true,
        }
    }

    /// Adds an explicit logical-qubit allocation.
    pub fn with_logical_qubits(
        mut self,
        ids: Vec<LogicalQubitId>,
    ) -> Result<Self, LogicalHardwareError> {
        let required =
            self.requirements.logical_qubits().get();

        if ids.len() as u64 != required {
            return Err(
                LogicalHardwareError::LogicalAllocationMismatch {
                    expected: required,
                    actual: ids.len() as u64,
                },
            );
        }

        self.logical_qubits = ids;
        self.automatic_allocation = false;

        Ok(self)
    }

    /// Controls automatic allocation.
    #[must_use]
    pub const fn with_automatic_allocation(
        mut self,
        value: bool,
    ) -> Self {
        self.automatic_allocation = value;
        self
    }

    /// Controls mandatory pre-execution validation.
    #[must_use]
    pub const fn with_validation(
        mut self,
        value: bool,
    ) -> Self {
        self.require_validation = value;
        self
    }

    /// Returns hardware.
    #[must_use]
    pub fn hardware(&self) -> &HardwareReference {
        &self.hardware
    }

    /// Returns requirements.
    #[must_use]
    pub fn requirements(
        &self,
    ) -> &LogicalWorkloadRequirements {
        &self.requirements
    }

    /// Returns explicit logical-qubit allocation.
    #[must_use]
    pub fn logical_qubits(
        &self,
    ) -> &[LogicalQubitId] {
        &self.logical_qubits
    }

    /// Returns whether automatic allocation is allowed.
    #[must_use]
    pub const fn automatic_allocation(
        &self,
    ) -> bool {
        self.automatic_allocation
    }

    /// Returns whether validation is required.
    #[must_use]
    pub const fn require_validation(
        &self,
    ) -> bool {
        self.require_validation
    }

    /// Validates request-level invariants.
    pub fn validate(&self) -> Result<(), LogicalHardwareError> {
        if self.hardware.as_str().is_empty() {
            return Err(
                LogicalHardwareError::EmptyIdentifier {
                    field: "hardware reference",
                },
            );
        }

        if !self.automatic_allocation
            && self.logical_qubits.len() as u64
                != self
                    .requirements
                    .logical_qubits()
                    .get()
        {
            return Err(
                LogicalHardwareError::LogicalAllocationMismatch {
                    expected: self
                        .requirements
                        .logical_qubits()
                        .get(),
                    actual: self
                        .logical_qubits
                        .len() as u64,
                },
            );
        }

        Ok(())
    }
}

// =============================================================================
// Logical hardware inventory
// =============================================================================

/// Complete logical-resource inventory exposed by one hardware target.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub struct LogicalHardwareInventory {
    /// Schema identifier.
    schema_id: String,

    /// Schema version.
    schema_version: u16,

    /// Hardware reference.
    hardware: HardwareReference,

    /// Logical capability advertisement.
    capabilities: LogicalHardwareCapabilities,

    /// Concrete logical resources.
    resources: BTreeMap<LogicalQubitId, LogicalQubitResource>,
}

impl LogicalHardwareInventory {
    /// Creates an inventory.
    pub fn new(
        hardware: HardwareReference,
        capabilities: LogicalHardwareCapabilities,
    ) -> Result<Self, LogicalHardwareError> {
        capabilities.validate()?;

        Ok(Self {
            schema_id:
                LOGICAL_HARDWARE_SCHEMA_ID.to_owned(),
            schema_version:
                LOGICAL_HARDWARE_SCHEMA_VERSION,
            hardware,
            capabilities,
            resources: BTreeMap::new(),
        })
    }

    /// Adds a logical resource.
    pub fn add_resource(
        &mut self,
        resource: LogicalQubitResource,
    ) -> Result<(), LogicalHardwareError> {
        resource.validate()?;

        if resource.hardware() != &self.hardware {
            return Err(
                LogicalHardwareError::HardwareReferenceMismatch,
            );
        }

        if self.resources.contains_key(resource.id()) {
            return Err(
                LogicalHardwareError::DuplicateLogicalResource {
                    id: resource.id().to_string(),
                },
            );
        }

        if self.resources.len() as u64
            >= self
                .capabilities
                .logical_qubit_capacity()
                .get()
        {
            return Err(
                LogicalHardwareError::LogicalCapacityExceeded {
                    capacity: self
                        .capabilities
                        .logical_qubit_capacity()
                        .get(),
                },
            );
        }

        self.resources
            .insert(resource.id().clone(), resource);

        Ok(())
    }

    /// Returns the schema identifier.
    #[must_use]
    pub fn schema_id(&self) -> &str {
        &self.schema_id
    }

    /// Returns the schema version.
    #[must_use]
    pub const fn schema_version(&self) -> u16 {
        self.schema_version
    }

    /// Returns the hardware reference.
    #[must_use]
    pub fn hardware(&self) -> &HardwareReference {
        &self.hardware
    }

    /// Returns logical capabilities.
    #[must_use]
    pub fn capabilities(
        &self,
    ) -> &LogicalHardwareCapabilities {
        &self.capabilities
    }

    /// Returns all resources in deterministic ID order.
    #[must_use]
    pub fn resources(
        &self,
    ) -> &BTreeMap<
        LogicalQubitId,
        LogicalQubitResource,
    > {
        &self.resources
    }

    /// Returns one logical resource.
    #[must_use]
    pub fn resource(
        &self,
        id: &LogicalQubitId,
    ) -> Option<&LogicalQubitResource> {
        self.resources.get(id)
    }

    /// Counts allocatable logical resources.
    #[must_use]
    pub fn allocatable_count(&self) -> u64 {
        self.resources
            .values()
            .filter(|resource| {
                resource.is_allocatable()
            })
            .count() as u64
    }

    /// Validates the inventory.
    pub fn validate(&self) -> Result<(), LogicalHardwareError> {
        if self.schema_id
            != LOGICAL_HARDWARE_SCHEMA_ID
        {
            return Err(
                LogicalHardwareError::SchemaMismatch {
                    expected:
                        LOGICAL_HARDWARE_SCHEMA_ID,
                    actual: self.schema_id.clone(),
                },
            );
        }

        if self.schema_version
            != LOGICAL_HARDWARE_SCHEMA_VERSION
        {
            return Err(
                LogicalHardwareError::UnsupportedSchemaVersion {
                    version: self.schema_version,
                },
            );
        }

        self.capabilities.validate()?;

        if self.resources.len() as u64
            > self
                .capabilities
                .logical_qubit_capacity()
                .get()
        {
            return Err(
                LogicalHardwareError::LogicalCapacityExceeded {
                    capacity: self
                        .capabilities
                        .logical_qubit_capacity()
                        .get(),
                },
            );
        }

        for resource in self.resources.values() {
            resource.validate()?;
        }

        Ok(())
    }

    /// Checks workload compatibility against this inventory.
    #[must_use]
    pub fn check_compatibility(
        &self,
        requirements: &LogicalWorkloadRequirements,
    ) -> LogicalCompatibilityReport {
        check_compatibility(
            requirements,
            &self.capabilities,
        )
    }
}

// =============================================================================
// Resource estimation
// =============================================================================

/// Estimates physical resources from a logical code and workload.
pub fn estimate_resources(
    requirements: &LogicalWorkloadRequirements,
    code: &LogicalCodeDescriptor,
) -> Result<LogicalResourceEstimate, LogicalHardwareError> {
    if requirements.logical_qubits().get() == 0 {
        return Err(
            LogicalHardwareError::ZeroLogicalCapacity,
        );
    }

    if code.logical_qubits_per_block().get() == 0 {
        return Err(
            LogicalHardwareError::ZeroLogicalCapacity,
        );
    }

    if code.physical_resources_per_block().get()
        == 0
    {
        return Err(
            LogicalHardwareError::ZeroPhysicalCapacity,
        );
    }

    let logical_per_block =
        code.logical_qubits_per_block().get();

    let blocks = ceil_div(
        requirements.logical_qubits().get(),
        logical_per_block,
    )?;

    let physical_per_block =
        code.physical_resources_per_block().get();

    let physical = checked_mul(
        blocks,
        physical_per_block,
        "estimated physical resources",
    )?;

    let physical_resources =
        PhysicalQubitCount::new(physical)?;

    let mut estimate = LogicalResourceEstimate::new(
        requirements.logical_qubits(),
        physical_resources,
        true,
        "computed from logical workload capacity and advertised code-block resource ratio",
    )?;

    if let Some(distance) =
        code.distance()
    {
        estimate =
            estimate.with_code_distance(distance);
    }

    if let Some(rate) =
        code.logical_error_rate()
    {
        estimate =
            estimate.with_error_rate(rate);
    }

    Ok(estimate)
}

// =============================================================================
// Errors
// =============================================================================

/// Structured error boundary for logical hardware operations.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub enum LogicalHardwareError {
    /// An identifier is empty.
    EmptyIdentifier {
        /// Field name.
        field: &'static str,
    },

    /// An identifier exceeds the allowed length.
    IdentifierTooLong {
        /// Field name.
        field: &'static str,

        /// Actual length.
        length: usize,

        /// Maximum allowed length.
        maximum: usize,
    },

    /// An identifier contains forbidden whitespace.
    InvalidIdentifierWhitespace {
        /// Field name.
        field: &'static str,
    },

    /// A field exceeds a generic maximum.
    ValueTooLong {
        /// Field name.
        field: &'static str,

        /// Actual length.
        length: usize,

        /// Maximum.
        maximum: usize,
    },

    /// Floating-point input is not finite.
    NonFiniteValue {
        /// Field name.
        field: &'static str,
    },

    /// A probability/rate lies outside `[0, 1]`.
    InvalidProbability {
        /// Field name.
        field: &'static str,

        /// Supplied value.
        value: f64,
    },

    /// Resource exceeds the production safety limit.
    ResourceLimitExceeded {
        /// Resource name.
        resource: &'static str,

        /// Requested value.
        value: u64,

        /// Maximum value.
        maximum: u64,
    },

    /// Code distance is zero.
    InvalidCodeDistance {
        /// Supplied distance.
        value: u32,
    },

    /// Code distance exceeds the safety limit.
    CodeDistanceLimitExceeded {
        /// Supplied distance.
        value: u32,

        /// Maximum distance.
        maximum: u32,
    },

    /// Logical capacity is zero.
    ZeroLogicalCapacity,

    /// Physical capacity is zero.
    ZeroPhysicalCapacity,

    /// Operation arity is zero.
    InvalidArity {
        /// Operation name.
        operation: String,

        /// Supplied arity.
        arity: u32,
    },

    /// Operation arity is smaller than its semantic minimum.
    ArityTooSmall {
        /// Operation name.
        operation: String,

        /// Operation kind.
        kind: LogicalOperationKind,

        /// Minimum.
        minimum: u32,

        /// Actual.
        actual: u32,
    },

    /// Too many metadata entries.
    MetadataLimitExceeded {
        /// Maximum number of entries.
        maximum: usize,
    },

    /// Too many logical operations.
    OperationLimitExceeded {
        /// Maximum number.
        maximum: usize,
    },

    /// Duplicate operation.
    DuplicateOperation {
        /// Operation name.
        name: String,
    },

    /// Duplicate logical resource.
    DuplicateLogicalResource {
        /// Resource ID.
        id: String,
    },

    /// Logical resource capacity exceeded.
    LogicalCapacityExceeded {
        /// Advertised capacity.
        capacity: u64,
    },

    /// Physical resource list count does not match code metadata.
    PhysicalResourceCountMismatch {
        /// Expected number.
        expected: u64,

        /// Actual number.
        actual: u64,
    },

    /// Logical allocation count mismatch.
    LogicalAllocationMismatch {
        /// Expected number.
        expected: u64,

        /// Actual number.
        actual: u64,
    },

    /// Backend reference mismatch.
    HardwareReferenceMismatch,

    /// Capability fields contradict one another.
    CapabilityContradiction {
        /// Contradictory field.
        field: &'static str,

        /// Explanation.
        reason: &'static str,
    },

    /// Code descriptor is invalid.
    InvalidCodeDescriptor,

    /// Code descriptor count limit exceeded.
    CodeDescriptorLimitExceeded {
        /// Maximum count.
        maximum: usize,
    },

    /// Schema identifier differs.
    SchemaMismatch {
        /// Expected schema.
        expected: &'static str,

        /// Actual schema.
        actual: String,
    },

    /// Schema version is unsupported.
    UnsupportedSchemaVersion {
        /// Supplied version.
        version: u16,
    },

    /// Arithmetic overflow.
    ArithmeticOverflow {
        /// Operation.
        operation: &'static str,
    },
}

impl fmt::Display for LogicalHardwareError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::EmptyIdentifier { field } => {
                write!(
                    formatter,
                    "{field} must not be empty"
                )
            }

            Self::IdentifierTooLong {
                field,
                length,
                maximum,
            } => {
                write!(
                    formatter,
                    "{field} length {length} exceeds maximum {maximum}"
                )
            }

            Self::InvalidIdentifierWhitespace {
                field,
            } => {
                write!(
                    formatter,
                    "{field} contains forbidden whitespace"
                )
            }

            Self::ValueTooLong {
                field,
                length,
                maximum,
            } => {
                write!(
                    formatter,
                    "{field} length {length} exceeds maximum {maximum}"
                )
            }

            Self::NonFiniteValue { field } => {
                write!(
                    formatter,
                    "{field} must be finite"
                )
            }

            Self::InvalidProbability {
                field,
                value,
            } => {
                write!(
                    formatter,
                    "{field}={value} is outside [0, 1]"
                )
            }

            Self::ResourceLimitExceeded {
                resource,
                value,
                maximum,
            } => {
                write!(
                    formatter,
                    "{resource} value {value} exceeds maximum {maximum}"
                )
            }

            Self::InvalidCodeDistance { value } => {
                write!(
                    formatter,
                    "code distance {value} is invalid; distance must be greater than zero"
                )
            }

            Self::CodeDistanceLimitExceeded {
                value,
                maximum,
            } => {
                write!(
                    formatter,
                    "code distance {value} exceeds maximum {maximum}"
                )
            }

            Self::ZeroLogicalCapacity => {
                formatter.write_str(
                    "logical capacity must be greater than zero",
                )
            }

            Self::ZeroPhysicalCapacity => {
                formatter.write_str(
                    "physical resource capacity must be greater than zero",
                )
            }

            Self::InvalidArity {
                operation,
                arity,
            } => {
                write!(
                    formatter,
                    "logical operation '{operation}' has invalid arity {arity}"
                )
            }

            Self::ArityTooSmall {
                operation,
                kind,
                minimum,
                actual,
            } => {
                write!(
                    formatter,
                    "logical operation '{operation}' of kind {kind} requires minimum arity {minimum}, got {actual}"
                )
            }

            Self::MetadataLimitExceeded {
                maximum,
            } => {
                write!(
                    formatter,
                    "metadata limit of {maximum} entries exceeded"
                )
            }

            Self::OperationLimitExceeded {
                maximum,
            } => {
                write!(
                    formatter,
                    "logical operation limit of {maximum} exceeded"
                )
            }

            Self::DuplicateOperation { name } => {
                write!(
                    formatter,
                    "logical operation '{name}' is already registered"
                )
            }

            Self::DuplicateLogicalResource { id } => {
                write!(
                    formatter,
                    "logical resource '{id}' is already registered"
                )
            }

            Self::LogicalCapacityExceeded {
                capacity,
            } => {
                write!(
                    formatter,
                    "logical resource capacity {capacity} exceeded"
                )
            }

            Self::PhysicalResourceCountMismatch {
                expected,
                actual,
            } => {
                write!(
                    formatter,
                    "physical resource count mismatch: expected {expected}, got {actual}"
                )
            }

            Self::LogicalAllocationMismatch {
                expected,
                actual,
            } => {
                write!(
                    formatter,
                    "logical allocation mismatch: expected {expected}, got {actual}"
                )
            }

            Self::HardwareReferenceMismatch => {
                formatter.write_str(
                    "logical resource belongs to a different hardware reference",
                )
            }

            Self::CapabilityContradiction {
                field,
                reason,
            } => {
                write!(
                    formatter,
                    "capability contradiction in {field}: {reason}"
                )
            }

            Self::InvalidCodeDescriptor => {
                formatter.write_str(
                    "logical code descriptor is invalid",
                )
            }

            Self::CodeDescriptorLimitExceeded {
                maximum,
            } => {
                write!(
                    formatter,
                    "code descriptor limit of {maximum} exceeded"
                )
            }

            Self::SchemaMismatch {
                expected,
                actual,
            } => {
                write!(
                    formatter,
                    "logical hardware schema mismatch: expected {expected}, got {actual}"
                )
            }

            Self::UnsupportedSchemaVersion {
                version,
            } => {
                write!(
                    formatter,
                    "unsupported logical hardware schema version {version}"
                )
            }

            Self::ArithmeticOverflow {
                operation,
            } => {
                write!(
                    formatter,
                    "arithmetic overflow while performing {operation}"
                )
            }
        }
    }
}

impl std::error::Error for LogicalHardwareError {}

// =============================================================================
// Helpers
// =============================================================================

fn validate_identifier(
    value: &str,
    maximum: usize,
    field: &'static str,
) -> Result<(), LogicalHardwareError> {
    if value.is_empty() {
        return Err(
            LogicalHardwareError::EmptyIdentifier {
                field,
            },
        );
    }

    if value.len() > maximum {
        return Err(
            LogicalHardwareError::IdentifierTooLong {
                field,
                length: value.len(),
                maximum,
            },
        );
    }

    if value.chars().any(char::is_whitespace) {
        return Err(
            LogicalHardwareError::InvalidIdentifierWhitespace {
                field,
            },
        );
    }

    Ok(())
}

fn normalize_name(
    value: &str,
    maximum: usize,
    field: &'static str,
) -> Result<String, LogicalHardwareError> {
    if value.is_empty() {
        return Err(
            LogicalHardwareError::EmptyIdentifier {
                field,
            },
        );
    }

    if value.len() > maximum {
        return Err(
            LogicalHardwareError::ValueTooLong {
                field,
                length: value.len(),
                maximum,
            },
        );
    }

    let normalized = value.trim();

    if normalized.is_empty() {
        return Err(
            LogicalHardwareError::EmptyIdentifier {
                field,
            },
        );
    }

    if normalized.chars().any(char::is_control) {
        return Err(
            LogicalHardwareError::InvalidIdentifierWhitespace {
                field,
            },
        );
    }

    Ok(normalized.to_owned())
}

fn normalize_instruction_name(
    value: &str,
) -> String {
    value
        .trim()
        .to_ascii_lowercase()
}

fn validate_probability(
    value: f64,
    field: &'static str,
) -> Result<(), LogicalHardwareError> {
    if !value.is_finite() {
        return Err(
            LogicalHardwareError::NonFiniteValue {
                field,
            },
        );
    }

    if !(0.0..=1.0).contains(&value) {
        return Err(
            LogicalHardwareError::InvalidProbability {
                field,
                value,
            },
        );
    }

    Ok(())
}

fn ceil_div(
    numerator: u64,
    denominator: u64,
) -> Result<u64, LogicalHardwareError> {
    if denominator == 0 {
        return Err(
            LogicalHardwareError::ArithmeticOverflow {
                operation: "division by zero",
            },
        );
    }

    let adjusted = numerator
        .checked_add(denominator - 1)
        .ok_or(
            LogicalHardwareError::ArithmeticOverflow {
                operation: "ceil division",
            },
        )?;

    Ok(adjusted / denominator)
}

fn checked_mul(
    left: u64,
    right: u64,
    operation: &'static str,
) -> Result<u64, LogicalHardwareError> {
    left.checked_mul(right).ok_or(
        LogicalHardwareError::ArithmeticOverflow {
            operation,
        },
    )
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn hardware() -> HardwareReference {
        HardwareReference::new(
            "provider://example/backend/logical",
        )
        .expect("valid hardware reference")
    }

    fn code() -> LogicalCodeDescriptor {
        LogicalCodeDescriptor::new(
            LogicalCodeFamily::SurfaceCode,
            LogicalEncoding::QubitCode,
            LogicalQubitCount::new(1)
                .expect("valid logical count"),
            PhysicalQubitCount::new(49)
                .expect("valid physical count"),
        )
        .expect("valid code")
        .with_distance(
            CodeDistance::new(7)
                .expect("valid distance"),
        )
        .with_fault_tolerance(true)
        .with_active_correction(true)
        .with_syndrome_extraction(true)
        .with_decoder_required(true)
    }

    #[test]
    fn logical_id_rejects_empty_value() {
        assert!(LogicalQubitId::new("").is_err());
    }

    #[test]
    fn logical_id_rejects_whitespace() {
        assert!(
            LogicalQubitId::new("logical qubit").is_err()
        );
    }

    #[test]
    fn logical_id_accepts_canonical_identifier() {
        let id = LogicalQubitId::new(
            "logical://example/q0",
        )
        .expect("valid identifier");

        assert_eq!(
            id.as_str(),
            "logical://example/q0"
        );
    }

    #[test]
    fn probability_rejects_nan() {
        assert!(
            Probability::new(f64::NAN).is_err()
        );
    }

    #[test]
    fn probability_rejects_infinity() {
        assert!(
            Probability::new(f64::INFINITY).is_err()
        );
    }

    #[test]
    fn probability_rejects_out_of_range() {
        assert!(
            Probability::new(1.1).is_err()
        );
    }

    #[test]
    fn probability_accepts_boundaries() {
        assert!(
            Probability::new(0.0).is_ok()
        );
        assert!(
            Probability::new(1.0).is_ok()
        );
    }

    #[test]
    fn zero_code_distance_is_rejected() {
        assert!(
            CodeDistance::new(0).is_err()
        );
    }

    #[test]
    fn logical_operation_enforces_minimum_arity() {
        assert!(
            LogicalOperation::new(
                "cx",
                LogicalOperationKind::ControlledNot,
                1,
            )
            .is_err()
        );

        assert!(
            LogicalOperation::new(
                "cx",
                LogicalOperationKind::ControlledNot,
                2,
            )
            .is_ok()
        );
    }

    #[test]
    fn code_descriptor_is_constructible() {
        let descriptor = code();

        assert_eq!(
            descriptor.family(),
            LogicalCodeFamily::SurfaceCode
        );

        assert_eq!(
            descriptor.distance()
                .expect("distance")
                .get(),
            7
        );

        assert!(
            descriptor.is_fault_tolerant()
        );
    }

    #[test]
    fn capabilities_validate() {
        let mut capabilities =
            LogicalHardwareCapabilities::with_capacity(
                LogicalQubitCount::new(4)
                    .expect("capacity"),
            )
            .expect("capabilities");

        capabilities =
            capabilities
                .with_fault_tolerance(true)
                .with_syndrome_measurement(true)
                .with_decoder_execution(true)
                .with_logical_measurement(true)
                .with_logical_reset(true)
                .with_native_logical_operations(true);

        capabilities
            .add_code(code())
            .expect("add code");

        capabilities
            .add_operation(
                LogicalOperation::new(
                    "h",
                    LogicalOperationKind::Hadamard,
                    1,
                )
                .expect("operation"),
            )
            .expect("add operation");

        assert!(
            capabilities.validate().is_ok()
        );
    }

    #[test]
    fn decoder_requires_syndrome_measurement() {
        let capabilities =
            LogicalHardwareCapabilities::with_capacity(
                LogicalQubitCount::new(1)
                    .expect("capacity"),
            )
            .expect("capabilities")
            .with_decoder_execution(true);

        assert!(
            capabilities.validate().is_err()
        );
    }

    #[test]
    fn logical_t_requires_non_clifford_capability() {
        let capabilities =
            LogicalHardwareCapabilities::with_capacity(
                LogicalQubitCount::new(1)
                    .expect("capacity"),
            )
            .expect("capabilities")
            .with_logical_t(true);

        assert!(
            capabilities.validate().is_err()
        );
    }

    #[test]
    fn capability_contradiction_is_rejected() {
        let capabilities =
            LogicalHardwareCapabilities::with_capacity(
                LogicalQubitCount::new(1)
                    .expect("capacity"),
            )
            .expect("capabilities")
            .with_fault_tolerant_operations(true);

        assert!(
            capabilities.validate().is_err()
        );
    }

    #[test]
    fn resource_requires_matching_physical_count() {
        let resource =
            LogicalQubitResource::new(
                LogicalQubitId::new(
                    "logical://example/q0",
                )
                .expect("id"),
                hardware(),
                code(),
            )
            .expect("resource");

        let result =
            resource.with_physical_resource_ids(
                vec!["p0".to_owned()],
            );

        assert!(result.is_err());
    }

    #[test]
    fn resource_accepts_matching_physical_count() {
        let resource =
            LogicalQubitResource::new(
                LogicalQubitId::new(
                    "logical://example/q0",
                )
                .expect("id"),
                hardware(),
                code(),
            )
            .expect("resource");

        let physical_ids =
            (0..49)
                .map(|index| {
                    format!("physical://q{index}")
                })
                .collect();

        let resource =
            resource
                .with_physical_resource_ids(
                    physical_ids,
                )
                .expect("matching resources");

        assert_eq!(
            resource.physical_resource_ids()
                .len(),
            49
        );
    }

    #[test]
    fn inventory_rejects_wrong_hardware_reference() {
        let capabilities =
            LogicalHardwareCapabilities::with_capacity(
                LogicalQubitCount::new(1)
                    .expect("capacity"),
            )
            .expect("capabilities");

        let mut inventory =
            LogicalHardwareInventory::new(
                hardware(),
                capabilities,
            )
            .expect("inventory");

        let other_hardware =
            HardwareReference::new(
                "provider://other/backend",
            )
            .expect("hardware");

        let resource =
            LogicalQubitResource::new(
                LogicalQubitId::new(
                    "logical://other/q0",
                )
                .expect("id"),
                other_hardware,
                code(),
            )
            .expect("resource");

        assert!(
            inventory
                .add_resource(resource)
                .is_err()
        );
    }

    #[test]
    fn compatibility_accepts_matching_workload() {
        let mut capabilities =
            LogicalHardwareCapabilities::with_capacity(
                LogicalQubitCount::new(4)
                    .expect("capacity"),
            )
            .expect("capabilities")
            .with_fault_tolerance(true)
            .with_syndrome_measurement(true)
            .with_decoder_execution(true)
            .with_logical_measurement(true)
            .with_logical_reset(true)
            .with_native_logical_operations(true);

        capabilities
            .add_code(code())
            .expect("code");

        capabilities
            .add_operation(
                LogicalOperation::new(
                    "h",
                    LogicalOperationKind::Hadamard,
                    1,
                )
                .expect("h"),
            )
            .expect("operation");

        let requirements =
            LogicalWorkloadRequirements::new(
                LogicalQubitCount::new(2)
                    .expect("logical qubits"),
            )
            .expect("requirements")
            .with_fault_tolerance(true)
            .with_measurement(true)
            .with_syndrome_measurement(true)
            .with_decoder_execution(true)
            .with_code_family(
                LogicalCodeFamily::SurfaceCode,
            )
            .with_minimum_distance(
                CodeDistance::new(5)
                    .expect("distance"),
            );

        let report =
            check_compatibility(
                &requirements,
                &capabilities,
            );

        assert!(
            report.is_compatible()
        );
    }

    #[test]
    fn compatibility_rejects_insufficient_capacity() {
        let capabilities =
            LogicalHardwareCapabilities::with_capacity(
                LogicalQubitCount::new(1)
                    .expect("capacity"),
            )
            .expect("capabilities");

        let requirements =
            LogicalWorkloadRequirements::new(
                LogicalQubitCount::new(2)
                    .expect("logical qubits"),
            )
            .expect("requirements");

        let report =
            check_compatibility(
                &requirements,
                &capabilities,
            );

        assert!(
            report.status.is_incompatible()
        );
    }

    #[test]
    fn explicit_logical_allocation_must_match_requirement() {
        let requirements =
            LogicalWorkloadRequirements::new(
                LogicalQubitCount::new(2)
                    .expect("count"),
            )
            .expect("requirements");

        let request =
            LogicalExecutionRequest::new(
                hardware(),
                requirements,
            );

        let ids = vec![
            LogicalQubitId::new(
                "logical://example/q0",
            )
            .expect("id"),
        ];

        assert!(
            request
                .with_logical_qubits(ids)
                .is_err()
        );
    }

    #[test]
    fn resource_estimation_uses_code_block_ratio() {
        let requirements =
            LogicalWorkloadRequirements::new(
                LogicalQubitCount::new(3)
                    .expect("logical count"),
            )
            .expect("requirements");

        let estimate =
            estimate_resources(
                &requirements,
                &code(),
            )
            .expect("estimate");

        assert_eq!(
            estimate
                .required_logical_qubits()
                .get(),
            3
        );

        assert_eq!(
            estimate
                .estimated_physical_resources()
                .get(),
            147
        );
    }

    #[test]
    fn resource_estimation_ceil_division_is_correct() {
        let requirements =
            LogicalWorkloadRequirements::new(
                LogicalQubitCount::new(2)
                    .expect("logical count"),
            )
            .expect("requirements");

        let code = LogicalCodeDescriptor::new(
            LogicalCodeFamily::Repetition,
            LogicalEncoding::QubitCode,
            LogicalQubitCount::new(1)
                .expect("logical count"),
            PhysicalQubitCount::new(3)
                .expect("physical count"),
        )
        .expect("code");

        let estimate =
            estimate_resources(
                &requirements,
                &code,
            )
            .expect("estimate");

        assert_eq!(
            estimate
                .estimated_physical_resources()
                .get(),
            6
        );
    }

    #[test]
    fn inventory_is_deterministically_ordered() {
        let capabilities =
            LogicalHardwareCapabilities::with_capacity(
                LogicalQubitCount::new(2)
                    .expect("capacity"),
            )
            .expect("capabilities");

        let mut inventory =
            LogicalHardwareInventory::new(
                hardware(),
                capabilities,
            )
            .expect("inventory");

        let resource_a =
            LogicalQubitResource::new(
                LogicalQubitId::new(
                    "logical://example/q1",
                )
                .expect("id"),
                hardware(),
                code(),
            )
            .expect("resource");

        let resource_b =
            LogicalQubitResource::new(
                LogicalQubitId::new(
                    "logical://example/q0",
                )
                .expect("id"),
                hardware(),
                code(),
            )
            .expect("resource");

        inventory
            .add_resource(resource_a)
            .expect("add");

        inventory
            .add_resource(resource_b)
            .expect("add");

        let ids: Vec<String> =
            inventory
                .resources()
                .keys()
                .map(ToString::to_string)
                .collect();

        assert_eq!(
            ids,
            vec![
                "logical://example/q0"
                    .to_owned(),
                "logical://example/q1"
                    .to_owned(),
            ]
        );
    }

    #[test]
    fn serde_round_trip_is_supported() {
        let capabilities =
            LogicalHardwareCapabilities::with_capacity(
                LogicalQubitCount::new(2)
                    .expect("capacity"),
            )
            .expect("capabilities");

        let inventory =
            LogicalHardwareInventory::new(
                hardware(),
                capabilities,
            )
            .expect("inventory");

        let encoded =
            serde_json::to_string(&inventory)
                .expect("serialize");

        let decoded:
            LogicalHardwareInventory =
            serde_json::from_str(&encoded)
                .expect("deserialize");

        assert_eq!(
            inventory,
            decoded
        );
    }

    #[test]
    fn schema_identity_is_stable() {
        assert_eq!(
            LOGICAL_HARDWARE_SCHEMA_ID,
            "zamani.quantum.hardware.logical"
        );

        assert_eq!(
            LOGICAL_HARDWARE_SCHEMA_VERSION,
            1
        );
    }

    #[test]
    fn no_provider_specific_types_are_required() {
        let reference =
            HardwareReference::new(
                "provider://example/backend",
            )
            .expect("reference");

        assert_eq!(
            reference.as_str(),
            "provider://example/backend"
        );
    }

    #[test]
    fn logical_operation_names_are_normalized_for_lookup() {
        let operation =
            LogicalOperation::new(
                "h",
                LogicalOperationKind::Hadamard,
                1,
            )
            .expect("operation");

        let mut capabilities =
            LogicalHardwareCapabilities::with_capacity(
                LogicalQubitCount::new(1)
                    .expect("capacity"),
            )
            .expect("capabilities");

        capabilities
            .add_operation(operation)
            .expect("operation");

        assert!(
            capabilities.operation("h")
                .is_some()
        );
    }
}