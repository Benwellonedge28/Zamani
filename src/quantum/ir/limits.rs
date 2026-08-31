//! Zamani Quantum IR — Resource Limits
//!
//! Production-grade resource policy for the hardware-independent Zamani
//! quantum intermediate representation.
//!
//! # Architectural purpose
//!
//! `QuantumIrLimits` defines *policy limits* for IR construction, validation,
//! transformation, serialization, and analysis.
//!
//! These limits are NOT:
//!
//! - hardware capacity;
//! - physical-qubit limits;
//! - routing limits;
//! - scheduling limits;
//! - calibration limits;
//! - backend limits;
//! - simulator limits;
//! - a definition of how many qubits Zamani supports.
//!
//! The canonical Zamani Quantum IR has no architectural fixed qubit-count
//! ceiling. Logical and physical identifiers are represented by scalable
//! integer identifiers elsewhere in the IR. A concrete `QuantumIrLimits`
//! instance supplies a resource policy for a particular compilation,
//! validation, service, process, security boundary, or deployment.
//!
//! Therefore:
//!
//!     1 qubit
//!     63 qubits
//!     64 qubits
//!     128 qubits
//!     4,096 qubits
//!     1,000,000 qubits
//!     N finite qubits
//!
//! are all architecturally representable, subject to the resource policy and
//! the resources actually available to the process/system.
//!
//! A production deployment should normally use explicit finite limits.
//! `unbounded()` exists only as an explicit policy for trusted environments
//! where the caller deliberately accepts the host/platform resource limits.
//!
//! # Important 63-qubit rule
//!
//! There is deliberately no `63`-qubit architectural boundary in this module.
//! A fixed value such as `63` may be valid elsewhere when it represents a
//! mathematical bit mask or encoding width, but it must never be used here as
//! a maximum logical-qubit or physical-machine size.
//!
//! # Hardware boundary
//!
//! This module intentionally does NOT depend on `hardware/`.
//!
//! Hardware-specific capacity and capabilities belong to the hardware layer.
//! For example:
//!
//! - actual physical-qubit count;
//! - topology;
//! - native instructions;
//! - DAC/ADC limits;
//! - real channel count;
//! - calibration;
//! - device timing;
//! - hardware pulse constraints;
//! - backend-specific memory;
//! - QPU execution limits.
//!
//! Those are supplied by downstream target/hardware compatibility stages.
//!
//! # IR boundary
//!
//! This module also intentionally does NOT depend on:
//!
//! - `gate`;
//! - `circuit`;
//! - `measurement`;
//! - `pulse`;
//! - `waveform`;
//! - `channel`;
//! - `frame`;
//! - `schedule`;
//! - `program`;
//! - `validation`;
//! - `analysis`;
//! - `routing`;
//! - `optimization`;
//! - `scheduling`;
//! - `hardware`;
//! - frontend modules;
//! - simulators.
//!
//! This makes the limits contract safe to freeze before the rest of the IR
//! is integrated.
//!
//! # Integration contract
//!
//! Downstream IR modules consume this module through:
//!
//! - `QuantumIrLimits::production()` for normal production policy;
//! - `QuantumIrLimits::unbounded()` only for explicitly trusted/unbounded
//!   policy;
//! - `QuantumIrLimits::new(...)` for complete explicit construction;
//! - builder methods such as `with_max_qubits(...)`;
//! - resource-specific `check_*` methods;
//! - `checked_add` and `checked_mul` for overflow-safe accounting;
//! - `validate()` before accepting a policy at a compiler/security boundary.
//!
//! Future modules MUST NOT add their own hidden resource ceilings when a
//! corresponding limit exists here.
//!
//! Future modules may impose stricter *semantic or hardware* constraints,
//! but those constraints must remain separate from the canonical IR policy.
//!
//! # Security
//!
//! Resource limits are an important security boundary for:
//!
//! - untrusted IR;
//! - deserialization;
//! - generated IR;
//! - compiler services;
//! - language servers;
//! - remote compilation;
//! - benchmarking services;
//! - optimizer services;
//! - distributed compilation.
//!
//! All accounting helpers use checked arithmetic. No arithmetic in this file
//! intentionally wraps.
//!
//! No `unsafe` code is used.
//!
//! # Rust compatibility
//!
//! Rust 1.97.1.
//!
//! No nightly features.
//! No external dependencies.

use std::fmt;

// ============================================================================
// Resource identifiers
// ============================================================================

/// Stable names for resources controlled by [`QuantumIrLimits`].
///
/// This enum provides a machine-readable resource vocabulary without requiring
/// heap allocation for error reporting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResourceKind {
    /// Logical quantum bits.
    LogicalQubits,

    /// Classical bits.
    ClassicalBits,

    /// Total IR operations.
    Operations,

    /// Quantum/classical operands attached to one operation.
    OperandsPerOperation,

    /// Parameters attached to one operation.
    ParametersPerOperation,

    /// Program regions.
    Regions,

    /// Basic blocks.
    Blocks,

    /// SSA/IR values or equivalent value records.
    Values,

    /// Control-flow nesting depth.
    ControlFlowDepth,

    /// Logical circuit depth.
    CircuitDepth,

    /// Measurement operations.
    Measurements,

    /// Barrier operations.
    Barriers,

    /// Pulse operations.
    PulseOperations,

    /// Waveform samples.
    WaveformSamples,

    /// Waveform storage in bytes.
    WaveformBytes,

    /// Abstract control channels.
    Channels,

    /// Abstract frames.
    Frames,

    /// Scheduled operations.
    ScheduledOperations,

    /// Abstract schedule duration/time budget.
    ScheduleTime,

    /// Metadata bytes.
    MetadataBytes,

    /// Serialized program bytes.
    ProgramBytes,

    /// Extension operations/records.
    Extensions,

    /// Validation work.
    ValidationSteps,

    /// Analysis work.
    AnalysisSteps,
}

impl ResourceKind {
    /// Returns the stable machine-readable resource name.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LogicalQubits => "logical qubits",
            Self::ClassicalBits => "classical bits",
            Self::Operations => "operations",
            Self::OperandsPerOperation => "operands per operation",
            Self::ParametersPerOperation => "parameters per operation",
            Self::Regions => "regions",
            Self::Blocks => "blocks",
            Self::Values => "values",
            Self::ControlFlowDepth => "control-flow depth",
            Self::CircuitDepth => "circuit depth",
            Self::Measurements => "measurements",
            Self::Barriers => "barriers",
            Self::PulseOperations => "pulse operations",
            Self::WaveformSamples => "waveform samples",
            Self::WaveformBytes => "waveform bytes",
            Self::Channels => "channels",
            Self::Frames => "frames",
            Self::ScheduledOperations => "scheduled operations",
            Self::ScheduleTime => "schedule time",
            Self::MetadataBytes => "metadata bytes",
            Self::ProgramBytes => "program bytes",
            Self::Extensions => "extensions",
            Self::ValidationSteps => "validation steps",
            Self::AnalysisSteps => "analysis steps",
        }
    }
}

impl fmt::Display for ResourceKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// ============================================================================
// Errors
// ============================================================================

/// Errors produced while validating or applying IR resource limits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LimitsError {
    /// A configured limit is invalid.
    InvalidConfiguration {
        /// Stable name of the offending field.
        field: &'static str,

        /// Invalid configured value.
        value: usize,
    },

    /// A requested resource exceeds its configured maximum.
    ResourceExceeded {
        /// Resource that exceeded its policy.
        resource: ResourceKind,

        /// Requested amount.
        requested: usize,

        /// Maximum permitted amount.
        maximum: usize,
    },

    /// An addition overflowed `usize`.
    ArithmeticOverflow {
        /// Resource/calculation involved.
        resource: ResourceKind,
    },

    /// A multiplication overflowed `usize`.
    ArithmeticMultiplicationOverflow {
        /// Resource/calculation involved.
        resource: ResourceKind,
    },

    /// An addition overflowed `u128`.
    TimeArithmeticOverflow,

    /// A time value exceeds the configured maximum.
    ScheduleTimeExceeded {
        /// Requested abstract time units.
        requested: u128,

        /// Maximum permitted abstract time units.
        maximum: u128,
    },
}

impl fmt::Display for LimitsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidConfiguration { field, value } => {
                write!(
                    f,
                    "invalid quantum IR limit `{field}`: value {value}"
                )
            }

            Self::ResourceExceeded {
                resource,
                requested,
                maximum,
            } => {
                write!(
                    f,
                    "quantum IR resource limit exceeded for {resource}: \
                     requested {requested}, maximum {maximum}"
                )
            }

            Self::ArithmeticOverflow { resource } => {
                write!(
                    f,
                    "arithmetic overflow while checking quantum IR resource \
                     `{resource}`"
                )
            }

            Self::ArithmeticMultiplicationOverflow { resource } => {
                write!(
                    f,
                    "arithmetic multiplication overflow while checking \
                     quantum IR resource `{resource}`"
                )
            }

            Self::TimeArithmeticOverflow => {
                f.write_str(
                    "arithmetic overflow while checking quantum IR schedule time",
                )
            }

            Self::ScheduleTimeExceeded {
                requested,
                maximum,
            } => {
                write!(
                    f,
                    "quantum IR schedule-time limit exceeded: \
                     requested {requested}, maximum {maximum}"
                )
            }
        }
    }
}

impl std::error::Error for LimitsError {}

// ============================================================================
// Quantum IR limits
// ============================================================================

/// Complete resource-safety policy for the canonical Zamani Quantum IR.
///
/// Every field is an explicit upper bound.
///
/// Zero is valid for ordinary resources and means that the corresponding
/// resource is prohibited.
///
/// For example:
///
/// ```text
/// max_operations = 0
/// ```
///
/// means that only an empty program is permitted under that policy.
///
/// The validation and analysis work budgets must be non-zero because those
/// operations require at least one permitted unit of work.
///
/// # Architectural scalability
///
/// `max_qubits` is a policy value, not a language architecture limit.
///
/// Zamani therefore supports:
///
/// ```text
/// QuantumIrLimits::production()
/// QuantumIrLimits::new(...)
/// QuantumIrLimits::unbounded()
/// ```
///
/// according to the deployment's requirements.
///
/// A sufficiently large policy does not guarantee that a host can actually
/// allocate or execute that many resources. The operating system, process
/// address space, available memory, compiler implementation, backend, network,
/// and physical quantum target remain independent constraints.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct QuantumIrLimits {
    /// Maximum logical qubits in one IR program.
    ///
    /// This is deliberately `usize`, matching the canonical logical identifier
    /// representation used by `quantum::ir::qubit::QubitId`.
    pub(crate) max_qubits: usize,

    /// Maximum classical bits in one IR program.
    pub(crate) max_classical_bits: usize,

    /// Maximum total operations in one IR program.
    pub(crate) max_operations: usize,

    /// Maximum quantum/classical operands attached to one operation.
    pub(crate) max_operands: usize,

    /// Maximum parameters attached to one operation.
    pub(crate) max_parameters: usize,

    /// Maximum number of regions in one IR program.
    pub(crate) max_regions: usize,

    /// Maximum number of basic blocks in one IR program.
    pub(crate) max_blocks: usize,

    /// Maximum number of IR values in one IR program.
    pub(crate) max_values: usize,

    /// Maximum control-flow nesting depth.
    pub(crate) max_control_flow_depth: usize,

    /// Maximum logical circuit depth.
    pub(crate) max_depth: usize,

    /// Maximum measurement operations.
    pub(crate) max_measurements: usize,

    /// Maximum barrier operations.
    pub(crate) max_barriers: usize,

    /// Maximum pulse operations.
    pub(crate) max_pulse_operations: usize,

    /// Maximum waveform samples.
    pub(crate) max_waveform_samples: usize,

    /// Maximum waveform storage in bytes.
    pub(crate) max_waveform_bytes: usize,

    /// Maximum abstract control channels.
    pub(crate) max_channels: usize,

    /// Maximum abstract frames.
    pub(crate) max_frames: usize,

    /// Maximum scheduled operations.
    pub(crate) max_scheduled_operations: usize,

    /// Maximum abstract schedule duration.
    ///
    /// The unit is intentionally not defined here because this module must
    /// remain independent of the future timing representation.
    ///
    /// `timing.rs` will establish the canonical time unit and convert to this
    /// representation at the scheduling/validation boundary.
    pub(crate) max_schedule_time_units: u128,

    /// Maximum metadata size in bytes.
    pub(crate) max_metadata_bytes: usize,

    /// Maximum serialized program size in bytes.
    pub(crate) max_program_bytes: usize,

    /// Maximum extension records/operations.
    pub(crate) max_extensions: usize,

    /// Maximum deterministic validation work units.
    pub(crate) max_validation_steps: usize,

    /// Maximum deterministic analysis work units.
    pub(crate) max_analysis_steps: usize,
}

impl QuantumIrLimits {
    // ========================================================================
    // Production defaults
    // ========================================================================

    /// Default maximum logical qubits.
    ///
    /// This is a production *policy default*, not a Zamani architectural
    /// maximum.
    pub const DEFAULT_MAX_QUBITS: usize = 4_096;

    /// Default maximum classical bits.
    pub const DEFAULT_MAX_CLASSICAL_BITS: usize = 4_096;

    /// Default maximum operations.
    pub const DEFAULT_MAX_OPERATIONS: usize = 1_000_000;

    /// Default maximum operands per operation.
    ///
    /// This is deliberately not `63`; 64 is used because it is a power-of-two
    /// engineering default and is unrelated to quantum-machine size.
    pub const DEFAULT_MAX_OPERANDS: usize = 64;

    /// Default maximum parameters per operation.
    pub const DEFAULT_MAX_PARAMETERS: usize = 16;

    /// Default maximum regions.
    pub const DEFAULT_MAX_REGIONS: usize = 65_536;

    /// Default maximum basic blocks.
    pub const DEFAULT_MAX_BLOCKS: usize = 262_144;

    /// Default maximum IR values.
    pub const DEFAULT_MAX_VALUES: usize = 4_000_000;

    /// Default maximum control-flow nesting depth.
    pub const DEFAULT_MAX_CONTROL_FLOW_DEPTH: usize = 1_024;

    /// Default maximum circuit depth.
    pub const DEFAULT_MAX_DEPTH: usize = 1_000_000;

    /// Default maximum measurements.
    pub const DEFAULT_MAX_MEASUREMENTS: usize = 4_096;

    /// Default maximum barriers.
    pub const DEFAULT_MAX_BARRIERS: usize = 4_096;

    /// Default maximum pulse operations.
    pub const DEFAULT_MAX_PULSE_OPERATIONS: usize = 1_000_000;

    /// Default maximum waveform samples.
    pub const DEFAULT_MAX_WAVEFORM_SAMPLES: usize = 16_000_000;

    /// Default maximum waveform storage.
    pub const DEFAULT_MAX_WAVEFORM_BYTES: usize = 256 * 1024 * 1024;

    /// Default maximum abstract channels.
    pub const DEFAULT_MAX_CHANNELS: usize = 65_536;

    /// Default maximum abstract frames.
    pub const DEFAULT_MAX_FRAMES: usize = 65_536;

    /// Default maximum scheduled operations.
    pub const DEFAULT_MAX_SCHEDULED_OPERATIONS: usize = 2_000_000;

    /// Default maximum schedule time.
    ///
    /// This is an intentionally large abstract value. `timing.rs` defines
    /// the canonical conversion into these units.
    pub const DEFAULT_MAX_SCHEDULE_TIME_UNITS: u128 =
        1_000_000_000_000_000_000u128;

    /// Default maximum metadata size.
    pub const DEFAULT_MAX_METADATA_BYTES: usize = 64 * 1024;

    /// Default maximum serialized program size.
    pub const DEFAULT_MAX_PROGRAM_BYTES: usize = 512 * 1024 * 1024;

    /// Default maximum extension records.
    pub const DEFAULT_MAX_EXTENSIONS: usize = 65_536;

    /// Default validation work budget.
    pub const DEFAULT_MAX_VALIDATION_STEPS: usize = 10_000_000;

    /// Default analysis work budget.
    pub const DEFAULT_MAX_ANALYSIS_STEPS: usize = 10_000_000;

    /// Creates the normal production resource policy.
    pub const fn production() -> Self {
        Self {
            max_qubits: Self::DEFAULT_MAX_QUBITS,
            max_classical_bits: Self::DEFAULT_MAX_CLASSICAL_BITS,
            max_operations: Self::DEFAULT_MAX_OPERATIONS,
            max_operands: Self::DEFAULT_MAX_OPERANDS,
            max_parameters: Self::DEFAULT_MAX_PARAMETERS,
            max_regions: Self::DEFAULT_MAX_REGIONS,
            max_blocks: Self::DEFAULT_MAX_BLOCKS,
            max_values: Self::DEFAULT_MAX_VALUES,
            max_control_flow_depth:
                Self::DEFAULT_MAX_CONTROL_FLOW_DEPTH,
            max_depth: Self::DEFAULT_MAX_DEPTH,
            max_measurements: Self::DEFAULT_MAX_MEASUREMENTS,
            max_barriers: Self::DEFAULT_MAX_BARRIERS,
            max_pulse_operations:
                Self::DEFAULT_MAX_PULSE_OPERATIONS,
            max_waveform_samples:
                Self::DEFAULT_MAX_WAVEFORM_SAMPLES,
            max_waveform_bytes:
                Self::DEFAULT_MAX_WAVEFORM_BYTES,
            max_channels: Self::DEFAULT_MAX_CHANNELS,
            max_frames: Self::DEFAULT_MAX_FRAMES,
            max_scheduled_operations:
                Self::DEFAULT_MAX_SCHEDULED_OPERATIONS,
            max_schedule_time_units:
                Self::DEFAULT_MAX_SCHEDULE_TIME_UNITS,
            max_metadata_bytes:
                Self::DEFAULT_MAX_METADATA_BYTES,
            max_program_bytes:
                Self::DEFAULT_MAX_PROGRAM_BYTES,
            max_extensions: Self::DEFAULT_MAX_EXTENSIONS,
            max_validation_steps:
                Self::DEFAULT_MAX_VALIDATION_STEPS,
            max_analysis_steps:
                Self::DEFAULT_MAX_ANALYSIS_STEPS,
        }
    }

    /// Creates an explicitly unbounded policy.
    ///
    /// This does NOT mean that the operating system, allocator, process,
    /// compiler, simulator, backend, or quantum hardware can actually support
    /// unlimited resources.
    ///
    /// It means this policy itself does not impose a finite application-level
    /// resource ceiling.
    ///
    /// This constructor must only be used deliberately for trusted workloads.
    pub const fn unbounded() -> Self {
        Self {
            max_qubits: usize::MAX,
            max_classical_bits: usize::MAX,
            max_operations: usize::MAX,
            max_operands: usize::MAX,
            max_parameters: usize::MAX,
            max_regions: usize::MAX,
            max_blocks: usize::MAX,
            max_values: usize::MAX,
            max_control_flow_depth: usize::MAX,
            max_depth: usize::MAX,
            max_measurements: usize::MAX,
            max_barriers: usize::MAX,
            max_pulse_operations: usize::MAX,
            max_waveform_samples: usize::MAX,
            max_waveform_bytes: usize::MAX,
            max_channels: usize::MAX,
            max_frames: usize::MAX,
            max_scheduled_operations: usize::MAX,
            max_schedule_time_units: u128::MAX,
            max_metadata_bytes: usize::MAX,
            max_program_bytes: usize::MAX,
            max_extensions: usize::MAX,
            max_validation_steps: usize::MAX,
            max_analysis_steps: usize::MAX,
        }
    }

    /// Creates a deny-all policy.
    ///
    /// This is useful for security tests and policy composition.
    pub const fn deny_all() -> Self {
        Self {
            max_qubits: 0,
            max_classical_bits: 0,
            max_operations: 0,
            max_operands: 0,
            max_parameters: 0,
            max_regions: 0,
            max_blocks: 0,
            max_values: 0,
            max_control_flow_depth: 0,
            max_depth: 0,
            max_measurements: 0,
            max_barriers: 0,
            max_pulse_operations: 0,
            max_waveform_samples: 0,
            max_waveform_bytes: 0,
            max_channels: 0,
            max_frames: 0,
            max_scheduled_operations: 0,
            max_schedule_time_units: 0,
            max_metadata_bytes: 0,
            max_program_bytes: 0,
            max_extensions: 0,
            max_validation_steps: 1,
            max_analysis_steps: 1,
        }
    }

    // ========================================================================
    // Complete constructor
    // ========================================================================

    /// Creates a complete explicit resource policy.
    ///
    /// No value is silently clamped.
    ///
    /// Call [`QuantumIrLimits::validate`] after construction when the policy
    /// originates from configuration, deserialization, or an untrusted source.
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        max_qubits: usize,
        max_classical_bits: usize,
        max_operations: usize,
        max_operands: usize,
        max_parameters: usize,
        max_regions: usize,
        max_blocks: usize,
        max_values: usize,
        max_control_flow_depth: usize,
        max_depth: usize,
        max_measurements: usize,
        max_barriers: usize,
        max_pulse_operations: usize,
        max_waveform_samples: usize,
        max_waveform_bytes: usize,
        max_channels: usize,
        max_frames: usize,
        max_scheduled_operations: usize,
        max_schedule_time_units: u128,
        max_metadata_bytes: usize,
        max_program_bytes: usize,
        max_extensions: usize,
        max_validation_steps: usize,
        max_analysis_steps: usize,
    ) -> Self {
        Self {
            max_qubits,
            max_classical_bits,
            max_operations,
            max_operands,
            max_parameters,
            max_regions,
            max_blocks,
            max_values,
            max_control_flow_depth,
            max_depth,
            max_measurements,
            max_barriers,
            max_pulse_operations,
            max_waveform_samples,
            max_waveform_bytes,
            max_channels,
            max_frames,
            max_scheduled_operations,
            max_schedule_time_units,
            max_metadata_bytes,
            max_program_bytes,
            max_extensions,
            max_validation_steps,
            max_analysis_steps,
        }
    }

    // ========================================================================
    // Builder configuration
    // ========================================================================

    /// Sets the maximum logical-qubit count.
    pub const fn with_max_qubits(mut self, value: usize) -> Self {
        self.max_qubits = value;
        self
    }

    /// Sets the maximum classical-bit count.
    pub const fn with_max_classical_bits(
        mut self,
        value: usize,
    ) -> Self {
        self.max_classical_bits = value;
        self
    }

    /// Sets the maximum operation count.
    pub const fn with_max_operations(
        mut self,
        value: usize,
    ) -> Self {
        self.max_operations = value;
        self
    }

    /// Sets the maximum operands per operation.
    pub const fn with_max_operands(
        mut self,
        value: usize,
    ) -> Self {
        self.max_operands = value;
        self
    }

    /// Sets the maximum parameters per operation.
    pub const fn with_max_parameters(
        mut self,
        value: usize,
    ) -> Self {
        self.max_parameters = value;
        self
    }

    /// Sets the maximum region count.
    pub const fn with_max_regions(
        mut self,
        value: usize,
    ) -> Self {
        self.max_regions = value;
        self
    }

    /// Sets the maximum block count.
    pub const fn with_max_blocks(
        mut self,
        value: usize,
    ) -> Self {
        self.max_blocks = value;
        self
    }

    /// Sets the maximum IR value count.
    pub const fn with_max_values(
        mut self,
        value: usize,
    ) -> Self {
        self.max_values = value;
        self
    }

    /// Sets the maximum control-flow nesting depth.
    pub const fn with_max_control_flow_depth(
        mut self,
        value: usize,
    ) -> Self {
        self.max_control_flow_depth = value;
        self
    }

    /// Sets the maximum logical circuit depth.
    pub const fn with_max_depth(
        mut self,
        value: usize,
    ) -> Self {
        self.max_depth = value;
        self
    }

    /// Sets the maximum measurement count.
    pub const fn with_max_measurements(
        mut self,
        value: usize,
    ) -> Self {
        self.max_measurements = value;
        self
    }

    /// Sets the maximum barrier count.
    pub const fn with_max_barriers(
        mut self,
        value: usize,
    ) -> Self {
        self.max_barriers = value;
        self
    }

    /// Sets the maximum pulse-operation count.
    pub const fn with_max_pulse_operations(
        mut self,
        value: usize,
    ) -> Self {
        self.max_pulse_operations = value;
        self
    }

    /// Sets the maximum waveform sample count.
    pub const fn with_max_waveform_samples(
        mut self,
        value: usize,
    ) -> Self {
        self.max_waveform_samples = value;
        self
    }

    /// Sets the maximum waveform storage in bytes.
    pub const fn with_max_waveform_bytes(
        mut self,
        value: usize,
    ) -> Self {
        self.max_waveform_bytes = value;
        self
    }

    /// Sets the maximum abstract channel count.
    pub const fn with_max_channels(
        mut self,
        value: usize,
    ) -> Self {
        self.max_channels = value;
        self
    }

    /// Sets the maximum abstract frame count.
    pub const fn with_max_frames(
        mut self,
        value: usize,
    ) -> Self {
        self.max_frames = value;
        self
    }

    /// Sets the maximum scheduled-operation count.
    pub const fn with_max_scheduled_operations(
        mut self,
        value: usize,
    ) -> Self {
        self.max_scheduled_operations = value;
        self
    }

    /// Sets the maximum abstract schedule duration.
    pub const fn with_max_schedule_time_units(
        mut self,
        value: u128,
    ) -> Self {
        self.max_schedule_time_units = value;
        self
    }

    /// Sets the maximum metadata size in bytes.
    pub const fn with_max_metadata_bytes(
        mut self,
        value: usize,
    ) -> Self {
        self.max_metadata_bytes = value;
        self
    }

    /// Sets the maximum serialized program size.
    pub const fn with_max_program_bytes(
        mut self,
        value: usize,
    ) -> Self {
        self.max_program_bytes = value;
        self
    }

    /// Sets the maximum extension count.
    pub const fn with_max_extensions(
        mut self,
        value: usize,
    ) -> Self {
        self.max_extensions = value;
        self
    }

    /// Sets the validation work budget.
    pub const fn with_max_validation_steps(
        mut self,
        value: usize,
    ) -> Self {
        self.max_validation_steps = value;
        self
    }

    /// Sets the analysis work budget.
    pub const fn with_max_analysis_steps(
        mut self,
        value: usize,
    ) -> Self {
        self.max_analysis_steps = value;
        self
    }

    // ========================================================================
    // Accessors
    // ========================================================================

    /// Maximum logical qubits.
    pub const fn max_qubits(&self) -> usize {
        self.max_qubits
    }

    /// Maximum classical bits.
    pub const fn max_classical_bits(&self) -> usize {
        self.max_classical_bits
    }

    /// Maximum operations.
    pub const fn max_operations(&self) -> usize {
        self.max_operations
    }

    /// Maximum operands per operation.
    pub const fn max_operands(&self) -> usize {
        self.max_operands
    }

    /// Maximum parameters per operation.
    pub const fn max_parameters(&self) -> usize {
        self.max_parameters
    }

    /// Maximum regions.
    pub const fn max_regions(&self) -> usize {
        self.max_regions
    }

    /// Maximum blocks.
    pub const fn max_blocks(&self) -> usize {
        self.max_blocks
    }

    /// Maximum values.
    pub const fn max_values(&self) -> usize {
        self.max_values
    }

    /// Maximum control-flow depth.
    pub const fn max_control_flow_depth(&self) -> usize {
        self.max_control_flow_depth
    }

    /// Maximum circuit depth.
    pub const fn max_depth(&self) -> usize {
        self.max_depth
    }

    /// Maximum measurements.
    pub const fn max_measurements(&self) -> usize {
        self.max_measurements
    }

    /// Maximum barriers.
    pub const fn max_barriers(&self) -> usize {
        self.max_barriers
    }

    /// Maximum pulse operations.
    pub const fn max_pulse_operations(&self) -> usize {
        self.max_pulse_operations
    }

    /// Maximum waveform samples.
    pub const fn max_waveform_samples(&self) -> usize {
        self.max_waveform_samples
    }

    /// Maximum waveform bytes.
    pub const fn max_waveform_bytes(&self) -> usize {
        self.max_waveform_bytes
    }

    /// Maximum channels.
    pub const fn max_channels(&self) -> usize {
        self.max_channels
    }

    /// Maximum frames.
    pub const fn max_frames(&self) -> usize {
        self.max_frames
    }

    /// Maximum scheduled operations.
    pub const fn max_scheduled_operations(&self) -> usize {
        self.max_scheduled_operations
    }

    /// Maximum schedule time units.
    pub const fn max_schedule_time_units(&self) -> u128 {
        self.max_schedule_time_units
    }

    /// Maximum metadata bytes.
    pub const fn max_metadata_bytes(&self) -> usize {
        self.max_metadata_bytes
    }

    /// Maximum serialized program bytes.
    pub const fn max_program_bytes(&self) -> usize {
        self.max_program_bytes
    }

    /// Maximum extension records.
    pub const fn max_extensions(&self) -> usize {
        self.max_extensions
    }

    /// Maximum validation steps.
    pub const fn max_validation_steps(&self) -> usize {
        self.max_validation_steps
    }

    /// Maximum analysis steps.
    pub const fn max_analysis_steps(&self) -> usize {
        self.max_analysis_steps
    }

    // ========================================================================
    // Policy validation
    // ========================================================================

    /// Validates the consistency of this policy.
    ///
    /// Ordinary resource limits may be zero.
    ///
    /// Validation and analysis work budgets must be non-zero.
    pub const fn validate(&self) -> Result<(), LimitsError> {
        if self.max_validation_steps == 0 {
            return Err(
                LimitsError::InvalidConfiguration {
                    field: "max_validation_steps",
                    value: self.max_validation_steps,
                },
            );
        }

        if self.max_analysis_steps == 0 {
            return Err(
                LimitsError::InvalidConfiguration {
                    field: "max_analysis_steps",
                    value: self.max_analysis_steps,
                },
            );
        }

        Ok(())
    }

    /// Returns whether this is the explicit unbounded policy.
    ///
    /// This is intentionally an exact policy check rather than a hidden
    /// sentinel interpretation.
    pub const fn is_unbounded(&self) -> bool {
        self.max_qubits == usize::MAX
            && self.max_classical_bits == usize::MAX
            && self.max_operations == usize::MAX
            && self.max_operands == usize::MAX
            && self.max_parameters == usize::MAX
            && self.max_regions == usize::MAX
            && self.max_blocks == usize::MAX
            && self.max_values == usize::MAX
            && self.max_control_flow_depth == usize::MAX
            && self.max_depth == usize::MAX
            && self.max_measurements == usize::MAX
            && self.max_barriers == usize::MAX
            && self.max_pulse_operations == usize::MAX
            && self.max_waveform_samples == usize::MAX
            && self.max_waveform_bytes == usize::MAX
            && self.max_channels == usize::MAX
            && self.max_frames == usize::MAX
            && self.max_scheduled_operations == usize::MAX
            && self.max_schedule_time_units == u128::MAX
            && self.max_metadata_bytes == usize::MAX
            && self.max_program_bytes == usize::MAX
            && self.max_extensions == usize::MAX
            && self.max_validation_steps == usize::MAX
            && self.max_analysis_steps == usize::MAX
    }

    // ========================================================================
    // Resource checks
    // ========================================================================

    /// Checks logical-qubit count.
    pub const fn check_qubits(
        &self,
        requested: usize,
    ) -> Result<(), LimitsError> {
        Self::check(
            ResourceKind::LogicalQubits,
            requested,
            self.max_qubits,
        )
    }

    /// Checks classical-bit count.
    pub const fn check_classical_bits(
        &self,
        requested: usize,
    ) -> Result<(), LimitsError> {
        Self::check(
            ResourceKind::ClassicalBits,
            requested,
            self.max_classical_bits,
        )
    }

    /// Checks operation count.
    pub const fn check_operations(
        &self,
        requested: usize,
    ) -> Result<(), LimitsError> {
        Self::check(
            ResourceKind::Operations,
            requested,
            self.max_operations,
        )
    }

    /// Checks operand count for one operation.
    pub const fn check_operands(
        &self,
        requested: usize,
    ) -> Result<(), LimitsError> {
        Self::check(
            ResourceKind::OperandsPerOperation,
            requested,
            self.max_operands,
        )
    }

    /// Checks parameter count for one operation.
    pub const fn check_parameters(
        &self,
        requested: usize,
    ) -> Result<(), LimitsError> {
        Self::check(
            ResourceKind::ParametersPerOperation,
            requested,
            self.max_parameters,
        )
    }

    /// Checks region count.
    pub const fn check_regions(
        &self,
        requested: usize,
    ) -> Result<(), LimitsError> {
        Self::check(
            ResourceKind::Regions,
            requested,
            self.max_regions,
        )
    }

    /// Checks block count.
    pub const fn check_blocks(
        &self,
        requested: usize,
    ) -> Result<(), LimitsError> {
        Self::check(
            ResourceKind::Blocks,
            requested,
            self.max_blocks,
        )
    }

    /// Checks value count.
    pub const fn check_values(
        &self,
        requested: usize,
    ) -> Result<(), LimitsError> {
        Self::check(
            ResourceKind::Values,
            requested,
            self.max_values,
        )
    }

    /// Checks control-flow nesting depth.
    pub const fn check_control_flow_depth(
        &self,
        requested: usize,
    ) -> Result<(), LimitsError> {
        Self::check(
            ResourceKind::ControlFlowDepth,
            requested,
            self.max_control_flow_depth,
        )
    }

    /// Checks circuit depth.
    pub const fn check_depth(
        &self,
        requested: usize,
    ) -> Result<(), LimitsError> {
        Self::check(
            ResourceKind::CircuitDepth,
            requested,
            self.max_depth,
        )
    }

    /// Checks measurement count.
    pub const fn check_measurements(
        &self,
        requested: usize,
    ) -> Result<(), LimitsError> {
        Self::check(
            ResourceKind::Measurements,
            requested,
            self.max_measurements,
        )
    }

    /// Checks barrier count.
    pub const fn check_barriers(
        &self,
        requested: usize,
    ) -> Result<(), LimitsError> {
        Self::check(
            ResourceKind::Barriers,
            requested,
            self.max_barriers,
        )
    }

    /// Checks pulse-operation count.
    pub const fn check_pulse_operations(
        &self,
        requested: usize,
    ) -> Result<(), LimitsError> {
        Self::check(
            ResourceKind::PulseOperations,
            requested,
            self.max_pulse_operations,
        )
    }

    /// Checks waveform sample count.
    pub const fn check_waveform_samples(
        &self,
        requested: usize,
    ) -> Result<(), LimitsError> {
        Self::check(
            ResourceKind::WaveformSamples,
            requested,
            self.max_waveform_samples,
        )
    }

    /// Checks waveform storage size.
    pub const fn check_waveform_bytes(
        &self,
        requested: usize,
    ) -> Result<(), LimitsError> {
        Self::check(
            ResourceKind::WaveformBytes,
            requested,
            self.max_waveform_bytes,
        )
    }

    /// Checks channel count.
    pub const fn check_channels(
        &self,
        requested: usize,
    ) -> Result<(), LimitsError> {
        Self::check(
            ResourceKind::Channels,
            requested,
            self.max_channels,
        )
    }

    /// Checks frame count.
    pub const fn check_frames(
        &self,
        requested: usize,
    ) -> Result<(), LimitsError> {
        Self::check(
            ResourceKind::Frames,
            requested,
            self.max_frames,
        )
    }

    /// Checks scheduled-operation count.
    pub const fn check_scheduled_operations(
        &self,
        requested: usize,
    ) -> Result<(), LimitsError> {
        Self::check(
            ResourceKind::ScheduledOperations,
            requested,
            self.max_scheduled_operations,
        )
    }

    /// Checks abstract schedule time.
    pub const fn check_schedule_time_units(
        &self,
        requested: u128,
    ) -> Result<(), LimitsError> {
        if requested > self.max_schedule_time_units {
            return Err(
                LimitsError::ScheduleTimeExceeded {
                    requested,
                    maximum: self.max_schedule_time_units,
                },
            );
        }

        Ok(())
    }

    /// Checks metadata byte size.
    pub const fn check_metadata_bytes(
        &self,
        requested: usize,
    ) -> Result<(), LimitsError> {
        Self::check(
            ResourceKind::MetadataBytes,
            requested,
            self.max_metadata_bytes,
        )
    }

    /// Checks serialized program size.
    pub const fn check_program_bytes(
        &self,
        requested: usize,
    ) -> Result<(), LimitsError> {
        Self::check(
            ResourceKind::ProgramBytes,
            requested,
            self.max_program_bytes,
        )
    }

    /// Checks extension count.
    pub const fn check_extensions(
        &self,
        requested: usize,
    ) -> Result<(), LimitsError> {
        Self::check(
            ResourceKind::Extensions,
            requested,
            self.max_extensions,
        )
    }

    /// Checks validation work.
    pub const fn check_validation_steps(
        &self,
        requested: usize,
    ) -> Result<(), LimitsError> {
        Self::check(
            ResourceKind::ValidationSteps,
            requested,
            self.max_validation_steps,
        )
    }

    /// Checks analysis work.
    pub const fn check_analysis_steps(
        &self,
        requested: usize,
    ) -> Result<(), LimitsError> {
        Self::check(
            ResourceKind::AnalysisSteps,
            requested,
            self.max_analysis_steps,
        )
    }

    /// Generic resource check.
    pub const fn check_resource(
        &self,
        resource: ResourceKind,
        requested: usize,
    ) -> Result<(), LimitsError> {
        let maximum = match resource {
            ResourceKind::LogicalQubits => self.max_qubits,
            ResourceKind::ClassicalBits => self.max_classical_bits,
            ResourceKind::Operations => self.max_operations,
            ResourceKind::OperandsPerOperation => {
                self.max_operands
            }
            ResourceKind::ParametersPerOperation => {
                self.max_parameters
            }
            ResourceKind::Regions => self.max_regions,
            ResourceKind::Blocks => self.max_blocks,
            ResourceKind::Values => self.max_values,
            ResourceKind::ControlFlowDepth => {
                self.max_control_flow_depth
            }
            ResourceKind::CircuitDepth => self.max_depth,
            ResourceKind::Measurements => self.max_measurements,
            ResourceKind::Barriers => self.max_barriers,
            ResourceKind::PulseOperations => {
                self.max_pulse_operations
            }
            ResourceKind::WaveformSamples => {
                self.max_waveform_samples
            }
            ResourceKind::WaveformBytes => {
                self.max_waveform_bytes
            }
            ResourceKind::Channels => self.max_channels,
            ResourceKind::Frames => self.max_frames,
            ResourceKind::ScheduledOperations => {
                self.max_scheduled_operations
            }
            ResourceKind::ScheduleTime => {
                return if (requested as u128)
                    > self.max_schedule_time_units
                {
                    Err(
                        LimitsError::ScheduleTimeExceeded {
                            requested: requested as u128,
                            maximum: self.max_schedule_time_units,
                        },
                    )
                } else {
                    Ok(())
                };
            }
            ResourceKind::MetadataBytes => {
                self.max_metadata_bytes
            }
            ResourceKind::ProgramBytes => {
                self.max_program_bytes
            }
            ResourceKind::Extensions => {
                self.max_extensions
            }
            ResourceKind::ValidationSteps => {
                self.max_validation_steps
            }
            ResourceKind::AnalysisSteps => {
                self.max_analysis_steps
            }
        };

        Self::check(resource, requested, maximum)
    }

    // ========================================================================
    // Overflow-safe accounting
    // ========================================================================

    /// Adds two resource quantities and checks the result against a maximum.
    ///
    /// This must be used whenever a downstream module calculates:
    ///
    /// ```text
    /// current + incoming
    /// ```
    ///
    /// instead of using unchecked arithmetic.
    pub const fn checked_add(
        resource: ResourceKind,
        current: usize,
        additional: usize,
        maximum: usize,
    ) -> Result<usize, LimitsError> {
        match current.checked_add(additional) {
            Some(total) if total <= maximum => Ok(total),

            Some(total) => {
                Err(LimitsError::ResourceExceeded {
                    resource,
                    requested: total,
                    maximum,
                })
            }

            None => {
                Err(LimitsError::ArithmeticOverflow {
                    resource,
                })
            }
        }
    }

    /// Multiplies two resource quantities and checks the result.
    ///
    /// This is used for bounded work estimates such as:
    ///
    /// ```text
    /// operations × operands
    /// ```
    ///
    /// or:
    ///
    /// ```text
    /// channels × samples
    /// ```
    pub const fn checked_mul(
        resource: ResourceKind,
        left: usize,
        right: usize,
        maximum: usize,
    ) -> Result<usize, LimitsError> {
        match left.checked_mul(right) {
            Some(total) if total <= maximum => Ok(total),

            Some(total) => {
                Err(LimitsError::ResourceExceeded {
                    resource,
                    requested: total,
                    maximum,
                })
            }

            None => {
                Err(
                    LimitsError::ArithmeticMultiplicationOverflow {
                        resource,
                    },
                )
            }
        }
    }

    /// Adds two abstract schedule-time quantities without overflow.
    pub const fn checked_add_time(
        current: u128,
        additional: u128,
        maximum: u128,
    ) -> Result<u128, LimitsError> {
        match current.checked_add(additional) {
            Some(total) if total <= maximum => Ok(total),

            Some(total) => {
                Err(
                    LimitsError::ScheduleTimeExceeded {
                        requested: total,
                        maximum,
                    },
                )
            }

            None => Err(LimitsError::TimeArithmeticOverflow),
        }
    }

    /// Multiplies two abstract schedule-time quantities without overflow.
    pub const fn checked_mul_time(
        left: u128,
        right: u128,
        maximum: u128,
    ) -> Result<u128, LimitsError> {
        match left.checked_mul(right) {
            Some(total) if total <= maximum => Ok(total),

            Some(total) => {
                Err(
                    LimitsError::ScheduleTimeExceeded {
                        requested: total,
                        maximum,
                    },
                )
            }

            None => Err(LimitsError::TimeArithmeticOverflow),
        }
    }

    /// Internal allocation-free resource check.
    const fn check(
        resource: ResourceKind,
        requested: usize,
        maximum: usize,
    ) -> Result<(), LimitsError> {
        if requested > maximum {
            return Err(
                LimitsError::ResourceExceeded {
                    resource,
                    requested,
                    maximum,
                },
            );
        }

        Ok(())
    }
}

impl Default for QuantumIrLimits {
    fn default() -> Self {
        Self::production()
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn production_policy_is_valid() {
        assert!(
            QuantumIrLimits::production()
                .validate()
                .is_ok()
        );
    }

    #[test]
    fn default_is_production_policy() {
        assert_eq!(
            QuantumIrLimits::default(),
            QuantumIrLimits::production()
        );
    }

    #[test]
    fn unbounded_policy_is_explicit() {
        let limits = QuantumIrLimits::unbounded();

        assert!(limits.is_unbounded());
        assert!(
            limits.validate().is_ok()
        );
    }

    #[test]
    fn production_policy_is_not_unbounded() {
        assert!(
            !QuantumIrLimits::production()
                .is_unbounded()
        );
    }

    #[test]
    fn deny_all_policy_is_valid() {
        let limits = QuantumIrLimits::deny_all();

        assert!(
            limits.validate().is_ok()
        );

        assert!(
            limits.check_qubits(0).is_ok()
        );

        assert!(
            limits.check_operations(0).is_ok()
        );

        assert!(
            limits.check_pulse_operations(0).is_ok()
        );
    }

    #[test]
    fn zero_validation_budget_is_invalid() {
        let limits =
            QuantumIrLimits::production()
                .with_max_validation_steps(0);

        assert_eq!(
            limits.validate(),
            Err(
                LimitsError::InvalidConfiguration {
                    field: "max_validation_steps",
                    value: 0,
                }
            )
        );
    }

    #[test]
    fn zero_analysis_budget_is_invalid() {
        let limits =
            QuantumIrLimits::production()
                .with_max_analysis_steps(0);

        assert_eq!(
            limits.validate(),
            Err(
                LimitsError::InvalidConfiguration {
                    field: "max_analysis_steps",
                    value: 0,
                }
            )
        );
    }

    #[test]
    fn exact_resource_limit_is_accepted() {
        let limits =
            QuantumIrLimits::production()
                .with_max_qubits(8);

        assert!(
            limits.check_qubits(8).is_ok()
        );
    }

    #[test]
    fn resource_above_limit_is_rejected() {
        let limits =
            QuantumIrLimits::production()
                .with_max_qubits(8);

        assert_eq!(
            limits.check_qubits(9),
            Err(
                LimitsError::ResourceExceeded {
                    resource:
                        ResourceKind::LogicalQubits,
                    requested: 9,
                    maximum: 8,
                }
            )
        );
    }

    #[test]
    fn zero_is_a_valid_resource_limit() {
        let limits =
            QuantumIrLimits::production()
                .with_max_qubits(0);

        assert!(
            limits.check_qubits(0).is_ok()
        );

        assert!(
            limits.check_qubits(1).is_err()
        );
    }

    #[test]
    fn sixty_three_qubits_is_not_special() {
        let limits =
            QuantumIrLimits::unbounded();

        assert!(
            limits.check_qubits(63).is_ok()
        );

        assert!(
            limits.check_qubits(64).is_ok()
        );

        assert!(
            limits.check_qubits(128).is_ok()
        );
    }

    #[test]
    fn large_qubit_counts_are_policy_driven() {
        let limits =
            QuantumIrLimits::production()
                .with_max_qubits(1_000_000);

        assert!(
            limits.check_qubits(1_000_000)
                .is_ok()
        );

        assert!(
            limits.check_qubits(1_000_001)
                .is_err()
        );
    }

    #[test]
    fn pulse_limits_are_independent_of_qubit_limits() {
        let limits =
            QuantumIrLimits::production()
                .with_max_qubits(1)
                .with_max_pulse_operations(100);

        assert!(
            limits.check_qubits(1).is_ok()
        );

        assert!(
            limits.check_pulse_operations(100)
                .is_ok()
        );
    }

    #[test]
    fn waveform_limits_are_enforced() {
        let limits =
            QuantumIrLimits::production()
                .with_max_waveform_samples(1_000)
                .with_max_waveform_bytes(8_000);

        assert!(
            limits.check_waveform_samples(1_000)
                .is_ok()
        );

        assert!(
            limits.check_waveform_samples(1_001)
                .is_err()
        );

        assert!(
            limits.check_waveform_bytes(8_000)
                .is_ok()
        );

        assert!(
            limits.check_waveform_bytes(8_001)
                .is_err()
        );
    }

    #[test]
    fn channel_and_frame_limits_are_enforced() {
        let limits =
            QuantumIrLimits::production()
                .with_max_channels(8)
                .with_max_frames(16);

        assert!(
            limits.check_channels(8).is_ok()
        );

        assert!(
            limits.check_channels(9).is_err()
        );

        assert!(
            limits.check_frames(16).is_ok()
        );

        assert!(
            limits.check_frames(17).is_err()
        );
    }

    #[test]
    fn control_flow_limit_is_enforced() {
        let limits =
            QuantumIrLimits::production()
                .with_max_control_flow_depth(32);

        assert!(
            limits
                .check_control_flow_depth(32)
                .is_ok()
        );

        assert!(
            limits
                .check_control_flow_depth(33)
                .is_err()
        );
    }

    #[test]
    fn schedule_time_limit_is_enforced() {
        let limits =
            QuantumIrLimits::production()
                .with_max_schedule_time_units(100);

        assert!(
            limits
                .check_schedule_time_units(100)
                .is_ok()
        );

        assert!(
            limits
                .check_schedule_time_units(101)
                .is_err()
        );
    }

    #[test]
    fn checked_add_accepts_exact_limit() {
        let result =
            QuantumIrLimits::checked_add(
                ResourceKind::Operations,
                5,
                5,
                10,
            );

        assert_eq!(result, Ok(10));
    }

    #[test]
    fn checked_add_rejects_limit_excess() {
        let result =
            QuantumIrLimits::checked_add(
                ResourceKind::Operations,
                6,
                5,
                10,
            );

        assert_eq!(
            result,
            Err(
                LimitsError::ResourceExceeded {
                    resource:
                        ResourceKind::Operations,
                    requested: 11,
                    maximum: 10,
                }
            )
        );
    }

    #[test]
    fn checked_add_rejects_overflow() {
        let result =
            QuantumIrLimits::checked_add(
                ResourceKind::Operations,
                usize::MAX,
                1,
                usize::MAX,
            );

        assert_eq!(
            result,
            Err(
                LimitsError::ArithmeticOverflow {
                    resource:
                        ResourceKind::Operations,
                }
            )
        );
    }

    #[test]
    fn checked_mul_accepts_exact_limit() {
        let result =
            QuantumIrLimits::checked_mul(
                ResourceKind::ValidationSteps,
                10,
                10,
                100,
            );

        assert_eq!(result, Ok(100));
    }

    #[test]
    fn checked_mul_rejects_limit_excess() {
        let result =
            QuantumIrLimits::checked_mul(
                ResourceKind::ValidationSteps,
                11,
                10,
                100,
            );

        assert_eq!(
            result,
            Err(
                LimitsError::ResourceExceeded {
                    resource:
                        ResourceKind::ValidationSteps,
                    requested: 110,
                    maximum: 100,
                }
            )
        );
    }

    #[test]
    fn checked_mul_rejects_overflow() {
        let result =
            QuantumIrLimits::checked_mul(
                ResourceKind::AnalysisSteps,
                usize::MAX,
                2,
                usize::MAX,
            );

        assert_eq!(
            result,
            Err(
                LimitsError::ArithmeticMultiplicationOverflow {
                    resource:
                        ResourceKind::AnalysisSteps,
                }
            )
        );
    }

    #[test]
    fn checked_time_add_rejects_overflow() {
        let result =
            QuantumIrLimits::checked_add_time(
                u128::MAX,
                1,
                u128::MAX,
            );

        assert_eq!(
            result,
            Err(
                LimitsError::TimeArithmeticOverflow
            )
        );
    }

    #[test]
    fn checked_time_mul_rejects_overflow() {
        let result =
            QuantumIrLimits::checked_mul_time(
                u128::MAX,
                2,
                u128::MAX,
            );

        assert_eq!(
            result,
            Err(
                LimitsError::TimeArithmeticOverflow
            )
        );
    }

    #[test]
    fn generic_resource_check_works() {
        let limits =
            QuantumIrLimits::production()
                .with_max_channels(4);

        assert!(
            limits
                .check_resource(
                    ResourceKind::Channels,
                    4,
                )
                .is_ok()
        );

        assert!(
            limits
                .check_resource(
                    ResourceKind::Channels,
                    5,
                )
                .is_err()
        );
    }

    #[test]
    fn builder_methods_are_composable() {
        let limits =
            QuantumIrLimits::production()
                .with_max_qubits(128)
                .with_max_classical_bits(128)
                .with_max_operations(10_000)
                .with_max_operands(8)
                .with_max_parameters(4)
                .with_max_regions(128)
                .with_max_blocks(256)
                .with_max_values(1_024)
                .with_max_control_flow_depth(16)
                .with_max_depth(512)
                .with_max_measurements(128)
                .with_max_barriers(32)
                .with_max_pulse_operations(10_000)
                .with_max_waveform_samples(100_000)
                .with_max_waveform_bytes(1_000_000)
                .with_max_channels(128)
                .with_max_frames(128)
                .with_max_scheduled_operations(20_000)
                .with_max_schedule_time_units(1_000_000)
                .with_max_metadata_bytes(4_096)
                .with_max_program_bytes(10_000_000)
                .with_max_extensions(128)
                .with_max_validation_steps(100_000)
                .with_max_analysis_steps(100_000);

        assert_eq!(
            limits.max_qubits(),
            128
        );

        assert_eq!(
            limits.max_classical_bits(),
            128
        );

        assert_eq!(
            limits.max_operations(),
            10_000
        );

        assert_eq!(
            limits.max_operands(),
            8
        );

        assert_eq!(
            limits.max_parameters(),
            4
        );

        assert_eq!(
            limits.max_regions(),
            128
        );

        assert_eq!(
            limits.max_blocks(),
            256
        );

        assert_eq!(
            limits.max_values(),
            1_024
        );

        assert_eq!(
            limits.max_control_flow_depth(),
            16
        );

        assert_eq!(
            limits.max_depth(),
            512
        );

        assert_eq!(
            limits.max_measurements(),
            128
        );

        assert_eq!(
            limits.max_barriers(),
            32
        );

        assert_eq!(
            limits.max_pulse_operations(),
            10_000
        );

        assert_eq!(
            limits.max_waveform_samples(),
            100_000
        );

        assert_eq!(
            limits.max_waveform_bytes(),
            1_000_000
        );

        assert_eq!(
            limits.max_channels(),
            128
        );

        assert_eq!(
            limits.max_frames(),
            128
        );

        assert_eq!(
            limits.max_scheduled_operations(),
            20_000
        );

        assert_eq!(
            limits.max_schedule_time_units(),
            1_000_000
        );

        assert_eq!(
            limits.max_metadata_bytes(),
            4_096
        );

        assert_eq!(
            limits.max_program_bytes(),
            10_000_000
        );

        assert_eq!(
            limits.max_extensions(),
            128
        );

        assert_eq!(
            limits.max_validation_steps(),
            100_000
        );

        assert_eq!(
            limits.max_analysis_steps(),
            100_000
        );

        assert!(
            limits.validate().is_ok()
        );
    }

    #[test]
    fn resource_kind_names_are_stable() {
        assert_eq!(
            ResourceKind::LogicalQubits.as_str(),
            "logical qubits"
        );

        assert_eq!(
            ResourceKind::PulseOperations.as_str(),
            "pulse operations"
        );

        assert_eq!(
            ResourceKind::WaveformSamples.as_str(),
            "waveform samples"
        );

        assert_eq!(
            ResourceKind::ScheduleTime.as_str(),
            "schedule time"
        );
    }

    #[test]
    fn default_policy_supports_pulse_programs() {
        let limits =
            QuantumIrLimits::production();

        assert!(
            limits
                .check_pulse_operations(1)
                .is_ok()
        );

        assert!(
            limits
                .check_waveform_samples(1)
                .is_ok()
        );

        assert!(
            limits
                .check_channels(1)
                .is_ok()
        );

        assert!(
            limits
                .check_frames(1)
                .is_ok()
        );
    }

    #[test]
    fn policy_does_not_confuse_qubits_with_hardware_capacity() {
        let limits =
            QuantumIrLimits::unbounded();

        assert!(
            limits.check_qubits(1).is_ok()
        );

        assert!(
            limits.check_qubits(63).is_ok()
        );

        assert!(
            limits.check_qubits(64).is_ok()
        );

        assert!(
            limits.check_qubits(4_096).is_ok()
        );

        assert!(
            limits.check_qubits(1_000_000).is_ok()
        );
    }
}