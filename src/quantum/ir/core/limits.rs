//! Zamani Quantum IR — Core Resource Limits
//!
//! Production-grade, target-independent resource policy for the canonical
//! Zamani Quantum Intermediate Representation.
//!
//! # Architectural role
//!
//! This module defines explicit resource policies used to protect IR
//! construction, validation, transformation, serialization, analysis,
//! compilation services, and other resource-consuming IR operations.
//!
//! `QuantumIrLimits` is a POLICY.
//!
//! It is NOT:
//!
//! - a hardware capacity description;
//! - a physical-qubit count;
//! - a simulator capacity;
//! - a backend limit;
//! - a routing limit;
//! - a scheduling implementation;
//! - a calibration constraint;
//! - a definition of the maximum number of qubits Zamani supports;
//! - a fixed architectural ceiling for quantum computation.
//!
//! The canonical Zamani Quantum IR is intentionally not bounded to a fixed
//! number such as 32, 63, 64, 128, 4096, or 1_000_000 qubits.
//!
//! A concrete `QuantumIrLimits` value describes how much resource a particular
//! compilation, validation, service, process, or security boundary is willing
//! to consume.
//!
//! Therefore the following all have the same semantic status:
//!
//! ```text
//! 1 qubit
//! 63 qubits
//! 64 qubits
//! 128 qubits
//! 4,096 qubits
//! 1,000,000 qubits
//! N finite qubits
//! ```
//!
//! The actual achievable value is determined by the selected policy and by
//! resources available to the complete execution environment.
//!
//! # No infinity claim
//!
//! A computer cannot physically allocate an infinite number of objects.
//!
//! `QuantumIrLimits::unbounded()` therefore does NOT mean that infinite
//! resources exist. It means that this particular policy does not impose a
//! finite application-level ceiling.
//!
//! The operating system, process address space, allocator, available memory,
//! compiler implementation, execution target, network, and physical quantum
//! machine remain independent constraints.
//!
//! # Qubit identity boundary
//!
//! The canonical logical-qubit identity is owned by:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! ```
//!
//! This module deliberately does NOT redefine or import `QubitId`.
//!
//! `QuantumIrLimits` deals with resource COUNTS, not qubit identity.
//!
//! For example:
//!
//! ```text
//! QubitId
//!     = identity
//!
//! usize count
//!     = number of resources permitted by a policy
//!
//! hardware capacity
//!     = target-specific capability
//! ```
//!
//! Keeping these concepts separate prevents a policy object from becoming
//! coupled to the quantum-object representation.
//!
//! # Dependency direction
//!
//! This file is deliberately foundational:
//!
//! ```text
//! core::limits
//!     ↑
//!     │ consumed by
//!     ├── qubit
//!     ├── gate
//!     ├── operation
//!     ├── program
//!     ├── pulse
//!     ├── timing
//!     ├── validation
//!     ├── analysis
//!     ├── serialization
//!     ├── scheduling
//!     └── downstream compiler infrastructure
//! ```
//!
//! This module must never depend on:
//!
//! - gate;
//! - circuit;
//! - operation;
//! - program;
//! - pulse;
//! - waveform;
//! - frame;
//! - channel;
//! - scheduling;
//! - routing;
//! - optimization;
//! - hardware;
//! - frontend;
//! - simulator;
//! - QEC implementation;
//! - backend.
//!
//! In particular, do not add a `QubitId` import here merely to perform count
//! validation. Doing so would move this foundational policy layer upward in
//! the dependency graph and can create circular dependencies.
//!
//! # Policy versus hardware
//!
//! A target may have:
//!
//! ```text
//! 127 physical qubits
//!
//! ```
//!
//! while a compilation policy may allow:
//!
//! ```text
//! 10_000 logical qubits
//! ```
//!
//! That is valid at the IR level. Target compatibility, logical-to-physical
//! mapping, routing, encoding, or compilation must later determine whether
//! and how the program can execute on the target.
//!
//! Conversely, a policy may restrict a compilation service to 512 qubits even
//! when a target has more resources.
//!
//! Neither value changes the semantic meaning of `QubitId`.
//!
//! # Security role
//!
//! Resource limits are an important defensive boundary for:
//!
//! - untrusted IR;
//! - deserialization;
//! - remote compilation;
//! - compiler-as-a-service deployments;
//! - language servers;
//! - generated programs;
//! - fuzzing;
//! - optimization services;
//! - benchmarking services;
//! - distributed compilation;
//! - maliciously large metadata;
//! - maliciously deep control flow;
//! - maliciously large symbolic expressions;
//! - waveform/resource exhaustion.
//!
//! All arithmetic that may overflow is checked.
//!
//! No arithmetic in this file intentionally wraps.
//!
//! # Rust compatibility
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust.
//!
//! No nightly features.
//! No external dependencies.
//! No `unsafe` code.
//!
//! The module explicitly forbids unsafe code.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::error::Error;
use std::fmt;

// =============================================================================
// Resource kinds
// =============================================================================

/// Resource categories governed by [`QuantumIrLimits`].
///
/// The variants describe semantic/resource accounting categories. They do not
/// describe hardware architectures.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ResourceKind {
    /// Number of logical quantum resources declared or referenced by an IR
    /// program.
    LogicalQubits,

    /// Number of classical bits.
    ClassicalBits,

    /// Number of quantum/classical registers.
    Registers,

    /// Number of IR operations.
    Operations,

    /// Number of operands attached to one operation.
    OperandsPerOperation,

    /// Number of parameters attached to one operation.
    ParametersPerOperation,

    /// Number of symbolic parameters in a program.
    Parameters,

    /// Number of symbolic-expression nodes.
    ExpressionNodes,

    /// Number of regions.
    Regions,

    /// Number of basic blocks.
    Blocks,

    /// Number of IR values.
    Values,

    /// Number of symbols.
    Symbols,

    /// Number of dependencies.
    Dependencies,

    /// Maximum control-flow nesting depth.
    ControlFlowDepth,

    /// Maximum general IR nesting depth.
    NestingDepth,

    /// Maximum logical circuit depth.
    CircuitDepth,

    /// Number of measurements.
    Measurements,

    /// Number of barriers.
    Barriers,

    /// Number of pulse operations.
    PulseOperations,

    /// Number of waveform samples.
    WaveformSamples,

    /// Waveform storage in bytes.
    WaveformBytes,

    /// Number of abstract channels.
    Channels,

    /// Number of abstract frames.
    Frames,

    /// Number of scheduled operations.
    ScheduledOperations,

    /// Number of mapping entries.
    MappingEntries,

    /// Number of resource requirements.
    ResourceRequirements,

    /// Number of extensions.
    Extensions,

    /// Number of diagnostics.
    Diagnostics,

    /// Metadata storage in bytes.
    MetadataBytes,

    /// Source information storage in bytes.
    SourceBytes,

    /// Serialized IR program size in bytes.
    ProgramBytes,

    /// General validation work units.
    ValidationSteps,

    /// General analysis work units.
    AnalysisSteps,

    /// General transformation work units.
    TransformationSteps,

    /// General compiler-service work units.
    CompilationSteps,

    /// Stack/recursion depth used by IR processing.
    ProcessingDepth,
}

impl ResourceKind {
    /// Returns a stable human-readable resource name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LogicalQubits => "logical qubits",
            Self::ClassicalBits => "classical bits",
            Self::Registers => "registers",
            Self::Operations => "operations",
            Self::OperandsPerOperation => "operands per operation",
            Self::ParametersPerOperation => "parameters per operation",
            Self::Parameters => "parameters",
            Self::ExpressionNodes => "expression nodes",
            Self::Regions => "regions",
            Self::Blocks => "blocks",
            Self::Values => "values",
            Self::Symbols => "symbols",
            Self::Dependencies => "dependencies",
            Self::ControlFlowDepth => "control-flow depth",
            Self::NestingDepth => "nesting depth",
            Self::CircuitDepth => "circuit depth",
            Self::Measurements => "measurements",
            Self::Barriers => "barriers",
            Self::PulseOperations => "pulse operations",
            Self::WaveformSamples => "waveform samples",
            Self::WaveformBytes => "waveform bytes",
            Self::Channels => "channels",
            Self::Frames => "frames",
            Self::ScheduledOperations => "scheduled operations",
            Self::MappingEntries => "mapping entries",
            Self::ResourceRequirements => "resource requirements",
            Self::Extensions => "extensions",
            Self::Diagnostics => "diagnostics",
            Self::MetadataBytes => "metadata bytes",
            Self::SourceBytes => "source bytes",
            Self::ProgramBytes => "program bytes",
            Self::ValidationSteps => "validation steps",
            Self::AnalysisSteps => "analysis steps",
            Self::TransformationSteps => "transformation steps",
            Self::CompilationSteps => "compilation steps",
            Self::ProcessingDepth => "processing depth",
        }
    }
}

impl fmt::Display for ResourceKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Limit errors
// =============================================================================

/// Errors returned by the IR resource-policy system.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LimitsError {
    /// The policy itself is invalid.
    InvalidConfiguration {
        /// Name of the invalid configuration field.
        field: &'static str,

        /// Invalid value.
        value: usize,
    },

    /// A requested resource exceeds its configured limit.
    ResourceExceeded {
        /// Resource that exceeded its policy.
        resource: ResourceKind,

        /// Requested amount.
        requested: usize,

        /// Maximum permitted amount.
        maximum: usize,
    },

    /// A `usize` addition overflowed.
    ArithmeticOverflow {
        /// Resource associated with the calculation.
        resource: ResourceKind,
    },

    /// A `usize` multiplication overflowed.
    ArithmeticMultiplicationOverflow {
        /// Resource associated with the calculation.
        resource: ResourceKind,
    },

    /// A `u128` addition overflowed.
    TimeArithmeticOverflow,

    /// A `u128` multiplication overflowed.
    TimeArithmeticMultiplicationOverflow,

    /// A schedule-time quantity exceeded the configured policy.
    ScheduleTimeExceeded {
        /// Requested time quantity.
        requested: u128,

        /// Maximum permitted time quantity.
        maximum: u128,
    },
}

impl fmt::Display for LimitsError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidConfiguration { field, value } => {
                write!(
                    formatter,
                    "invalid quantum IR limit `{field}`: value {value}"
                )
            }

            Self::ResourceExceeded {
                resource,
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "quantum IR resource limit exceeded for {resource}: \
                     requested {requested}, maximum {maximum}"
                )
            }

            Self::ArithmeticOverflow { resource } => {
                write!(
                    formatter,
                    "arithmetic overflow while accounting for quantum IR \
                     resource `{resource}`"
                )
            }

            Self::ArithmeticMultiplicationOverflow { resource } => {
                write!(
                    formatter,
                    "arithmetic multiplication overflow while accounting \
                     for quantum IR resource `{resource}`"
                )
            }

            Self::TimeArithmeticOverflow => {
                formatter.write_str(
                    "arithmetic overflow while accounting for quantum IR \
                     schedule time",
                )
            }

            Self::TimeArithmeticMultiplicationOverflow => {
                formatter.write_str(
                    "arithmetic multiplication overflow while accounting \
                     for quantum IR schedule time",
                )
            }

            Self::ScheduleTimeExceeded {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "quantum IR schedule-time limit exceeded: \
                     requested {requested}, maximum {maximum}"
                )
            }
        }
    }
}

impl Error for LimitsError {}

// =============================================================================
// Quantum IR limits
// =============================================================================

/// Explicit resource-safety policy for the canonical Zamani Quantum IR.
///
/// # Important
///
/// These values are policy ceilings, not language or hardware ceilings.
///
/// In particular:
///
/// ```text
/// max_qubits
/// ```
///
/// does NOT define the largest quantum computer Zamani can represent.
///
/// It defines the largest logical-qubit resource count accepted under this
/// particular policy instance.
///
/// `QuantumIrLimits::unbounded()` removes the finite application-level policy
/// ceiling while retaining checked arithmetic.
///
/// # Policy lifecycle
///
/// A typical production flow is:
///
/// ```text
/// policy construction
///       ↓
/// policy validation
///       ↓
/// IR construction/deserialization
///       ↓
/// per-resource checks
///       ↓
/// validation
///       ↓
/// optimization/analysis/lowering
/// ```
///
/// The policy should be supplied explicitly at security boundaries rather than
/// being hidden inside individual IR objects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct QuantumIrLimits {
    max_qubits: usize,
    max_classical_bits: usize,
    max_registers: usize,

    max_operations: usize,
    max_operands: usize,
    max_parameters_per_operation: usize,
    max_parameters: usize,
    max_expression_nodes: usize,

    max_regions: usize,
    max_blocks: usize,
    max_values: usize,
    max_symbols: usize,
    max_dependencies: usize,

    max_control_flow_depth: usize,
    max_nesting_depth: usize,
    max_circuit_depth: usize,
    max_processing_depth: usize,

    max_measurements: usize,
    max_barriers: usize,

    max_pulse_operations: usize,
    max_waveform_samples: usize,
    max_waveform_bytes: usize,
    max_channels: usize,
    max_frames: usize,

    max_scheduled_operations: usize,
    max_mapping_entries: usize,
    max_resource_requirements: usize,

    max_extensions: usize,
    max_diagnostics: usize,

    max_metadata_bytes: usize,
    max_source_bytes: usize,
    max_program_bytes: usize,

    max_validation_steps: usize,
    max_analysis_steps: usize,
    max_transformation_steps: usize,
    max_compilation_steps: usize,

    max_schedule_time_units: u128,
}

// =============================================================================
// Defaults
// =============================================================================

impl QuantumIrLimits {
    /// Default production logical-qubit policy.
    ///
    /// This is a deployment default only.
    pub const DEFAULT_MAX_QUBITS: usize = 4_096;

    /// Default production classical-bit policy.
    pub const DEFAULT_MAX_CLASSICAL_BITS: usize = 4_096;

    /// Default production register policy.
    pub const DEFAULT_MAX_REGISTERS: usize = 4_096;

    /// Default total operation policy.
    pub const DEFAULT_MAX_OPERATIONS: usize = 1_000_000;

    /// Default operand count for one operation.
    ///
    /// This is an engineering policy value and is not a quantum-architecture
    /// restriction.
    pub const DEFAULT_MAX_OPERANDS: usize = 64;

    /// Default parameter count for one operation.
    pub const DEFAULT_MAX_PARAMETERS_PER_OPERATION: usize = 16;

    /// Default total parameter count.
    pub const DEFAULT_MAX_PARAMETERS: usize = 1_000_000;

    /// Default symbolic-expression node count.
    pub const DEFAULT_MAX_EXPRESSION_NODES: usize = 4_000_000;

    /// Default region count.
    pub const DEFAULT_MAX_REGIONS: usize = 65_536;

    /// Default basic-block count.
    pub const DEFAULT_MAX_BLOCKS: usize = 262_144;

    /// Default IR-value count.
    pub const DEFAULT_MAX_VALUES: usize = 4_000_000;

    /// Default symbol count.
    pub const DEFAULT_MAX_SYMBOLS: usize = 1_000_000;

    /// Default dependency count.
    pub const DEFAULT_MAX_DEPENDENCIES: usize = 8_000_000;

    /// Default control-flow nesting depth.
    pub const DEFAULT_MAX_CONTROL_FLOW_DEPTH: usize = 1_024;

    /// Default general IR nesting depth.
    pub const DEFAULT_MAX_NESTING_DEPTH: usize = 1_024;

    /// Default logical circuit depth.
    pub const DEFAULT_MAX_CIRCUIT_DEPTH: usize = 1_000_000;

    /// Default processing/recursion depth.
    pub const DEFAULT_MAX_PROCESSING_DEPTH: usize = 1_024;

    /// Default measurement count.
    pub const DEFAULT_MAX_MEASUREMENTS: usize = 4_096;

    /// Default barrier count.
    pub const DEFAULT_MAX_BARRIERS: usize = 4_096;

    /// Default pulse-operation count.
    pub const DEFAULT_MAX_PULSE_OPERATIONS: usize = 1_000_000;

    /// Default waveform sample count.
    pub const DEFAULT_MAX_WAVEFORM_SAMPLES: usize = 16_000_000;

    /// Default waveform storage.
    pub const DEFAULT_MAX_WAVEFORM_BYTES: usize = 256 * 1024 * 1024;

    /// Default abstract channel count.
    pub const DEFAULT_MAX_CHANNELS: usize = 65_536;

    /// Default abstract frame count.
    pub const DEFAULT_MAX_FRAMES: usize = 65_536;

    /// Default scheduled-operation count.
    pub const DEFAULT_MAX_SCHEDULED_OPERATIONS: usize = 2_000_000;

    /// Default mapping-entry count.
    pub const DEFAULT_MAX_MAPPING_ENTRIES: usize = 1_000_000;

    /// Default resource-requirement count.
    pub const DEFAULT_MAX_RESOURCE_REQUIREMENTS: usize = 1_000_000;

    /// Default extension count.
    pub const DEFAULT_MAX_EXTENSIONS: usize = 65_536;

    /// Default diagnostic count.
    pub const DEFAULT_MAX_DIAGNOSTICS: usize = 100_000;

    /// Default metadata size.
    pub const DEFAULT_MAX_METADATA_BYTES: usize = 64 * 1024;

    /// Default source-information size.
    pub const DEFAULT_MAX_SOURCE_BYTES: usize = 64 * 1024 * 1024;

    /// Default serialized program size.
    pub const DEFAULT_MAX_PROGRAM_BYTES: usize = 512 * 1024 * 1024;

    /// Default validation work budget.
    pub const DEFAULT_MAX_VALIDATION_STEPS: usize = 10_000_000;

    /// Default analysis work budget.
    pub const DEFAULT_MAX_ANALYSIS_STEPS: usize = 10_000_000;

    /// Default transformation work budget.
    pub const DEFAULT_MAX_TRANSFORMATION_STEPS: usize = 100_000_000;

    /// Default compilation work budget.
    pub const DEFAULT_MAX_COMPILATION_STEPS: usize = 100_000_000;

    /// Default schedule-time budget.
    ///
    /// The unit is intentionally abstract at this layer. The timing subsystem
    /// owns conversion between physical/semantic time units and this policy
    /// representation.
    pub const DEFAULT_MAX_SCHEDULE_TIME_UNITS: u128 =
        1_000_000_000_000_000_000u128;

    /// Creates the normal production policy.
    #[must_use]
    pub const fn production() -> Self {
        Self {
            max_qubits: Self::DEFAULT_MAX_QUBITS,
            max_classical_bits: Self::DEFAULT_MAX_CLASSICAL_BITS,
            max_registers: Self::DEFAULT_MAX_REGISTERS,

            max_operations: Self::DEFAULT_MAX_OPERATIONS,
            max_operands: Self::DEFAULT_MAX_OPERANDS,
            max_parameters_per_operation:
                Self::DEFAULT_MAX_PARAMETERS_PER_OPERATION,
            max_parameters: Self::DEFAULT_MAX_PARAMETERS,
            max_expression_nodes:
                Self::DEFAULT_MAX_EXPRESSION_NODES,

            max_regions: Self::DEFAULT_MAX_REGIONS,
            max_blocks: Self::DEFAULT_MAX_BLOCKS,
            max_values: Self::DEFAULT_MAX_VALUES,
            max_symbols: Self::DEFAULT_MAX_SYMBOLS,
            max_dependencies: Self::DEFAULT_MAX_DEPENDENCIES,

            max_control_flow_depth:
                Self::DEFAULT_MAX_CONTROL_FLOW_DEPTH,
            max_nesting_depth:
                Self::DEFAULT_MAX_NESTING_DEPTH,
            max_circuit_depth:
                Self::DEFAULT_MAX_CIRCUIT_DEPTH,
            max_processing_depth:
                Self::DEFAULT_MAX_PROCESSING_DEPTH,

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
            max_mapping_entries:
                Self::DEFAULT_MAX_MAPPING_ENTRIES,
            max_resource_requirements:
                Self::DEFAULT_MAX_RESOURCE_REQUIREMENTS,

            max_extensions: Self::DEFAULT_MAX_EXTENSIONS,
            max_diagnostics: Self::DEFAULT_MAX_DIAGNOSTICS,

            max_metadata_bytes:
                Self::DEFAULT_MAX_METADATA_BYTES,
            max_source_bytes:
                Self::DEFAULT_MAX_SOURCE_BYTES,
            max_program_bytes:
                Self::DEFAULT_MAX_PROGRAM_BYTES,

            max_validation_steps:
                Self::DEFAULT_MAX_VALIDATION_STEPS,
            max_analysis_steps:
                Self::DEFAULT_MAX_ANALYSIS_STEPS,
            max_transformation_steps:
                Self::DEFAULT_MAX_TRANSFORMATION_STEPS,
            max_compilation_steps:
                Self::DEFAULT_MAX_COMPILATION_STEPS,

            max_schedule_time_units:
                Self::DEFAULT_MAX_SCHEDULE_TIME_UNITS,
        }
    }

    /// Creates an explicitly unbounded policy.
    ///
    /// This removes application-level finite ceilings.
    ///
    /// It does not make physical resources infinite and does not bypass
    /// arithmetic overflow protection.
    ///
    /// This mode is appropriate only when the caller deliberately wants the
    /// surrounding system to provide the effective resource boundary.
    #[must_use]
    pub const fn unbounded() -> Self {
        Self {
            max_qubits: usize::MAX,
            max_classical_bits: usize::MAX,
            max_registers: usize::MAX,

            max_operations: usize::MAX,
            max_operands: usize::MAX,
            max_parameters_per_operation: usize::MAX,
            max_parameters: usize::MAX,
            max_expression_nodes: usize::MAX,

            max_regions: usize::MAX,
            max_blocks: usize::MAX,
            max_values: usize::MAX,
            max_symbols: usize::MAX,
            max_dependencies: usize::MAX,

            max_control_flow_depth: usize::MAX,
            max_nesting_depth: usize::MAX,
            max_circuit_depth: usize::MAX,
            max_processing_depth: usize::MAX,

            max_measurements: usize::MAX,
            max_barriers: usize::MAX,

            max_pulse_operations: usize::MAX,
            max_waveform_samples: usize::MAX,
            max_waveform_bytes: usize::MAX,
            max_channels: usize::MAX,
            max_frames: usize::MAX,

            max_scheduled_operations: usize::MAX,
            max_mapping_entries: usize::MAX,
            max_resource_requirements: usize::MAX,

            max_extensions: usize::MAX,
            max_diagnostics: usize::MAX,

            max_metadata_bytes: usize::MAX,
            max_source_bytes: usize::MAX,
            max_program_bytes: usize::MAX,

            max_validation_steps: usize::MAX,
            max_analysis_steps: usize::MAX,
            max_transformation_steps: usize::MAX,
            max_compilation_steps: usize::MAX,

            max_schedule_time_units: u128::MAX,
        }
    }

    /// Creates a policy that permits no IR resources.
    ///
    /// The validation and analysis budgets remain one unit so the policy itself
    /// can still be validated and inspected.
    #[must_use]
    pub const fn deny_all() -> Self {
        Self {
            max_qubits: 0,
            max_classical_bits: 0,
            max_registers: 0,

            max_operations: 0,
            max_operands: 0,
            max_parameters_per_operation: 0,
            max_parameters: 0,
            max_expression_nodes: 0,

            max_regions: 0,
            max_blocks: 0,
            max_values: 0,
            max_symbols: 0,
            max_dependencies: 0,

            max_control_flow_depth: 0,
            max_nesting_depth: 0,
            max_circuit_depth: 0,
            max_processing_depth: 0,

            max_measurements: 0,
            max_barriers: 0,

            max_pulse_operations: 0,
            max_waveform_samples: 0,
            max_waveform_bytes: 0,
            max_channels: 0,
            max_frames: 0,

            max_scheduled_operations: 0,
            max_mapping_entries: 0,
            max_resource_requirements: 0,

            max_extensions: 0,
            max_diagnostics: 0,

            max_metadata_bytes: 0,
            max_source_bytes: 0,
            max_program_bytes: 0,

            max_validation_steps: 1,
            max_analysis_steps: 1,
            max_transformation_steps: 0,
            max_compilation_steps: 0,

            max_schedule_time_units: 0,
        }
    }
}

// =============================================================================
// Complete constructor
// =============================================================================

impl QuantumIrLimits {
    /// Constructs a complete explicit resource policy.
    ///
    /// This constructor performs no silent clamping.
    ///
    /// Call [`QuantumIrLimits::validate`] after constructing a policy from
    /// external configuration.
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub const fn new(
        max_qubits: usize,
        max_classical_bits: usize,
        max_registers: usize,
        max_operations: usize,
        max_operands: usize,
        max_parameters_per_operation: usize,
        max_parameters: usize,
        max_expression_nodes: usize,
        max_regions: usize,
        max_blocks: usize,
        max_values: usize,
        max_symbols: usize,
        max_dependencies: usize,
        max_control_flow_depth: usize,
        max_nesting_depth: usize,
        max_circuit_depth: usize,
        max_processing_depth: usize,
        max_measurements: usize,
        max_barriers: usize,
        max_pulse_operations: usize,
        max_waveform_samples: usize,
        max_waveform_bytes: usize,
        max_channels: usize,
        max_frames: usize,
        max_scheduled_operations: usize,
        max_mapping_entries: usize,
        max_resource_requirements: usize,
        max_extensions: usize,
        max_diagnostics: usize,
        max_metadata_bytes: usize,
        max_source_bytes: usize,
        max_program_bytes: usize,
        max_validation_steps: usize,
        max_analysis_steps: usize,
        max_transformation_steps: usize,
        max_compilation_steps: usize,
        max_schedule_time_units: u128,
    ) -> Self {
        Self {
            max_qubits,
            max_classical_bits,
            max_registers,

            max_operations,
            max_operands,
            max_parameters_per_operation,
            max_parameters,
            max_expression_nodes,

            max_regions,
            max_blocks,
            max_values,
            max_symbols,
            max_dependencies,

            max_control_flow_depth,
            max_nesting_depth,
            max_circuit_depth,
            max_processing_depth,

            max_measurements,
            max_barriers,

            max_pulse_operations,
            max_waveform_samples,
            max_waveform_bytes,
            max_channels,
            max_frames,

            max_scheduled_operations,
            max_mapping_entries,
            max_resource_requirements,

            max_extensions,
            max_diagnostics,

            max_metadata_bytes,
            max_source_bytes,
            max_program_bytes,

            max_validation_steps,
            max_analysis_steps,
            max_transformation_steps,
            max_compilation_steps,

            max_schedule_time_units,
        }
    }
}

// =============================================================================
// Builder methods
// =============================================================================

impl QuantumIrLimits {
    #[must_use]
    pub const fn with_max_qubits(mut self, value: usize) -> Self {
        self.max_qubits = value;
        self
    }

    #[must_use]
    pub const fn with_max_classical_bits(
        mut self,
        value: usize,
    ) -> Self {
        self.max_classical_bits = value;
        self
    }

    #[must_use]
    pub const fn with_max_registers(
        mut self,
        value: usize,
    ) -> Self {
        self.max_registers = value;
        self
    }

    #[must_use]
    pub const fn with_max_operations(
        mut self,
        value: usize,
    ) -> Self {
        self.max_operations = value;
        self
    }

    #[must_use]
    pub const fn with_max_operands(
        mut self,
        value: usize,
    ) -> Self {
        self.max_operands = value;
        self
    }

    #[must_use]
    pub const fn with_max_parameters_per_operation(
        mut self,
        value: usize,
    ) -> Self {
        self.max_parameters_per_operation = value;
        self
    }

    #[must_use]
    pub const fn with_max_parameters(
        mut self,
        value: usize,
    ) -> Self {
        self.max_parameters = value;
        self
    }

    #[must_use]
    pub const fn with_max_expression_nodes(
        mut self,
        value: usize,
    ) -> Self {
        self.max_expression_nodes = value;
        self
    }

    #[must_use]
    pub const fn with_max_regions(
        mut self,
        value: usize,
    ) -> Self {
        self.max_regions = value;
        self
    }

    #[must_use]
    pub const fn with_max_blocks(
        mut self,
        value: usize,
    ) -> Self {
        self.max_blocks = value;
        self
    }

    #[must_use]
    pub const fn with_max_values(
        mut self,
        value: usize,
    ) -> Self {
        self.max_values = value;
        self
    }

    #[must_use]
    pub const fn with_max_symbols(
        mut self,
        value: usize,
    ) -> Self {
        self.max_symbols = value;
        self
    }

    #[must_use]
    pub const fn with_max_dependencies(
        mut self,
        value: usize,
    ) -> Self {
        self.max_dependencies = value;
        self
    }

    #[must_use]
    pub const fn with_max_control_flow_depth(
        mut self,
        value: usize,
    ) -> Self {
        self.max_control_flow_depth = value;
        self
    }

    #[must_use]
    pub const fn with_max_nesting_depth(
        mut self,
        value: usize,
    ) -> Self {
        self.max_nesting_depth = value;
        self
    }

    #[must_use]
    pub const fn with_max_depth(
        mut self,
        value: usize,
    ) -> Self {
        self.max_circuit_depth = value;
        self
    }

    #[must_use]
    pub const fn with_max_circuit_depth(
        mut self,
        value: usize,
    ) -> Self {
        self.max_circuit_depth = value;
        self
    }

    #[must_use]
    pub const fn with_max_processing_depth(
        mut self,
        value: usize,
    ) -> Self {
        self.max_processing_depth = value;
        self
    }

    #[must_use]
    pub const fn with_max_measurements(
        mut self,
        value: usize,
    ) -> Self {
        self.max_measurements = value;
        self
    }

    #[must_use]
    pub const fn with_max_barriers(
        mut self,
        value: usize,
    ) -> Self {
        self.max_barriers = value;
        self
    }

    #[must_use]
    pub const fn with_max_pulse_operations(
        mut self,
        value: usize,
    ) -> Self {
        self.max_pulse_operations = value;
        self
    }

    #[must_use]
    pub const fn with_max_waveform_samples(
        mut self,
        value: usize,
    ) -> Self {
        self.max_waveform_samples = value;
        self
    }

    #[must_use]
    pub const fn with_max_waveform_bytes(
        mut self,
        value: usize,
    ) -> Self {
        self.max_waveform_bytes = value;
        self
    }

    #[must_use]
    pub const fn with_max_channels(
        mut self,
        value: usize,
    ) -> Self {
        self.max_channels = value;
        self
    }

    #[must_use]
    pub const fn with_max_frames(
        mut self,
        value: usize,
    ) -> Self {
        self.max_frames = value;
        self
    }

    #[must_use]
    pub const fn with_max_scheduled_operations(
        mut self,
        value: usize,
    ) -> Self {
        self.max_scheduled_operations = value;
        self
    }

    #[must_use]
    pub const fn with_max_mapping_entries(
        mut self,
        value: usize,
    ) -> Self {
        self.max_mapping_entries = value;
        self
    }

    #[must_use]
    pub const fn with_max_resource_requirements(
        mut self,
        value: usize,
    ) -> Self {
        self.max_resource_requirements = value;
        self
    }

    #[must_use]
    pub const fn with_max_extensions(
        mut self,
        value: usize,
    ) -> Self {
        self.max_extensions = value;
        self
    }

    #[must_use]
    pub const fn with_max_diagnostics(
        mut self,
        value: usize,
    ) -> Self {
        self.max_diagnostics = value;
        self
    }

    #[must_use]
    pub const fn with_max_metadata_bytes(
        mut self,
        value: usize,
    ) -> Self {
        self.max_metadata_bytes = value;
        self
    }

    #[must_use]
    pub const fn with_max_source_bytes(
        mut self,
        value: usize,
    ) -> Self {
        self.max_source_bytes = value;
        self
    }

    #[must_use]
    pub const fn with_max_program_bytes(
        mut self,
        value: usize,
    ) -> Self {
        self.max_program_bytes = value;
        self
    }

    #[must_use]
    pub const fn with_max_validation_steps(
        mut self,
        value: usize,
    ) -> Self {
        self.max_validation_steps = value;
        self
    }

    #[must_use]
    pub const fn with_max_analysis_steps(
        mut self,
        value: usize,
    ) -> Self {
        self.max_analysis_steps = value;
        self
    }

    #[must_use]
    pub const fn with_max_transformation_steps(
        mut self,
        value: usize,
    ) -> Self {
        self.max_transformation_steps = value;
        self
    }

    #[must_use]
    pub const fn with_max_compilation_steps(
        mut self,
        value: usize,
    ) -> Self {
        self.max_compilation_steps = value;
        self
    }

    #[must_use]
    pub const fn with_max_schedule_time_units(
        mut self,
        value: u128,
    ) -> Self {
        self.max_schedule_time_units = value;
        self
    }
}

// =============================================================================
// Accessors
// =============================================================================

impl QuantumIrLimits {
    #[must_use]
    pub const fn max_qubits(&self) -> usize {
        self.max_qubits
    }

    #[must_use]
    pub const fn max_classical_bits(&self) -> usize {
        self.max_classical_bits
    }

    #[must_use]
    pub const fn max_registers(&self) -> usize {
        self.max_registers
    }

    #[must_use]
    pub const fn max_operations(&self) -> usize {
        self.max_operations
    }

    #[must_use]
    pub const fn max_operands(&self) -> usize {
        self.max_operands
    }

    #[must_use]
    pub const fn max_parameters_per_operation(&self) -> usize {
        self.max_parameters_per_operation
    }

    #[must_use]
    pub const fn max_parameters(&self) -> usize {
        self.max_parameters
    }

    #[must_use]
    pub const fn max_expression_nodes(&self) -> usize {
        self.max_expression_nodes
    }

    #[must_use]
    pub const fn max_regions(&self) -> usize {
        self.max_regions
    }

    #[must_use]
    pub const fn max_blocks(&self) -> usize {
        self.max_blocks
    }

    #[must_use]
    pub const fn max_values(&self) -> usize {
        self.max_values
    }

    #[must_use]
    pub const fn max_symbols(&self) -> usize {
        self.max_symbols
    }

    #[must_use]
    pub const fn max_dependencies(&self) -> usize {
        self.max_dependencies
    }

    #[must_use]
    pub const fn max_control_flow_depth(&self) -> usize {
        self.max_control_flow_depth
    }

    #[must_use]
    pub const fn max_nesting_depth(&self) -> usize {
        self.max_nesting_depth
    }

    #[must_use]
    pub const fn max_depth(&self) -> usize {
        self.max_circuit_depth
    }

    #[must_use]
    pub const fn max_circuit_depth(&self) -> usize {
        self.max_circuit_depth
    }

    #[must_use]
    pub const fn max_processing_depth(&self) -> usize {
        self.max_processing_depth
    }

    #[must_use]
    pub const fn max_measurements(&self) -> usize {
        self.max_measurements
    }

    #[must_use]
    pub const fn max_barriers(&self) -> usize {
        self.max_barriers
    }

    #[must_use]
    pub const fn max_pulse_operations(&self) -> usize {
        self.max_pulse_operations
    }

    #[must_use]
    pub const fn max_waveform_samples(&self) -> usize {
        self.max_waveform_samples
    }

    #[must_use]
    pub const fn max_waveform_bytes(&self) -> usize {
        self.max_waveform_bytes
    }

    #[must_use]
    pub const fn max_channels(&self) -> usize {
        self.max_channels
    }

    #[must_use]
    pub const fn max_frames(&self) -> usize {
        self.max_frames
    }

    #[must_use]
    pub const fn max_scheduled_operations(&self) -> usize {
        self.max_scheduled_operations
    }

    #[must_use]
    pub const fn max_mapping_entries(&self) -> usize {
        self.max_mapping_entries
    }

    #[must_use]
    pub const fn max_resource_requirements(&self) -> usize {
        self.max_resource_requirements
    }

    #[must_use]
    pub const fn max_extensions(&self) -> usize {
        self.max_extensions
    }

    #[must_use]
    pub const fn max_diagnostics(&self) -> usize {
        self.max_diagnostics
    }

    #[must_use]
    pub const fn max_metadata_bytes(&self) -> usize {
        self.max_metadata_bytes
    }

    #[must_use]
    pub const fn max_source_bytes(&self) -> usize {
        self.max_source_bytes
    }

    #[must_use]
    pub const fn max_program_bytes(&self) -> usize {
        self.max_program_bytes
    }

    #[must_use]
    pub const fn max_validation_steps(&self) -> usize {
        self.max_validation_steps
    }

    #[must_use]
    pub const fn max_analysis_steps(&self) -> usize {
        self.max_analysis_steps
    }

    #[must_use]
    pub const fn max_transformation_steps(&self) -> usize {
        self.max_transformation_steps
    }

    #[must_use]
    pub const fn max_compilation_steps(&self) -> usize {
        self.max_compilation_steps
    }

    #[must_use]
    pub const fn max_schedule_time_units(&self) -> u128 {
        self.max_schedule_time_units
    }
}

// =============================================================================
// Policy validation
// =============================================================================

impl QuantumIrLimits {
    /// Validates the policy itself.
    ///
    /// Zero is valid for ordinary resource categories because zero can
    /// intentionally mean "this resource is prohibited".
    ///
    /// Validation and analysis are required security operations and therefore
    /// require at least one work unit.
    #[must_use]
    pub const fn validate(&self) -> Result<(), LimitsError> {
        if self.max_validation_steps == 0 {
            return Err(LimitsError::InvalidConfiguration {
                field: "max_validation_steps",
                value: 0,
            });
        }

        if self.max_analysis_steps == 0 {
            return Err(LimitsError::InvalidConfiguration {
                field: "max_analysis_steps",
                value: 0,
            });
        }

        Ok(())
    }

    /// Returns `true` when every finite resource limit is at its maximum
    /// representable value.
    #[must_use]
    pub const fn is_unbounded(&self) -> bool {
        self.max_qubits == usize::MAX
            && self.max_classical_bits == usize::MAX
            && self.max_registers == usize::MAX
            && self.max_operations == usize::MAX
            && self.max_operands == usize::MAX
            && self.max_parameters_per_operation == usize::MAX
            && self.max_parameters == usize::MAX
            && self.max_expression_nodes == usize::MAX
            && self.max_regions == usize::MAX
            && self.max_blocks == usize::MAX
            && self.max_values == usize::MAX
            && self.max_symbols == usize::MAX
            && self.max_dependencies == usize::MAX
            && self.max_control_flow_depth == usize::MAX
            && self.max_nesting_depth == usize::MAX
            && self.max_circuit_depth == usize::MAX
            && self.max_processing_depth == usize::MAX
            && self.max_measurements == usize::MAX
            && self.max_barriers == usize::MAX
            && self.max_pulse_operations == usize::MAX
            && self.max_waveform_samples == usize::MAX
            && self.max_waveform_bytes == usize::MAX
            && self.max_channels == usize::MAX
            && self.max_frames == usize::MAX
            && self.max_scheduled_operations == usize::MAX
            && self.max_mapping_entries == usize::MAX
            && self.max_resource_requirements == usize::MAX
            && self.max_extensions == usize::MAX
            && self.max_diagnostics == usize::MAX
            && self.max_metadata_bytes == usize::MAX
            && self.max_source_bytes == usize::MAX
            && self.max_program_bytes == usize::MAX
            && self.max_validation_steps == usize::MAX
            && self.max_analysis_steps == usize::MAX
            && self.max_transformation_steps == usize::MAX
            && self.max_compilation_steps == usize::MAX
            && self.max_schedule_time_units == u128::MAX
    }
}

// =============================================================================
// Individual resource checks
// =============================================================================

impl QuantumIrLimits {
    /// Checks a logical-qubit resource count.
    #[must_use]
    pub const fn check_qubits(
        &self,
        requested: usize,
    ) -> Result<(), LimitsError> {
        self.check_resource(ResourceKind::LogicalQubits, requested)
    }

    /// Checks a classical-bit resource count.
    #[must_use]
    pub const fn check_classical_bits(
        &self,
        requested: usize,
    ) -> Result<(), LimitsError> {
        self.check_resource(ResourceKind::ClassicalBits, requested)
    }

    /// Checks a register count.
    #[must_use]
    pub const fn check_registers(
        &self,
        requested: usize,
    ) -> Result<(), LimitsError> {
        self.check_resource(ResourceKind::Registers, requested)
    }

    /// Checks a total operation count.
    #[must_use]
    pub const fn check_operations(
        &self,
        requested: usize,
    ) -> Result<(), LimitsError> {
        self.check_resource(ResourceKind::Operations, requested)
    }

    /// Checks operands attached to one operation.
    #[must_use]
    pub const fn check_operands(
        &self,
        requested: usize,
    ) -> Result<(), LimitsError> {
        self.check_resource(
            ResourceKind::OperandsPerOperation,
            requested,
        )
    }

    /// Checks parameters attached to one operation.
    #[must_use]
    pub const fn check_parameters_per_operation(
        &self,
        requested: usize,
    ) -> Result<(), LimitsError> {
        self.check_resource(
            ResourceKind::ParametersPerOperation,
            requested,
        )
    }

    /// Checks total symbolic parameters.
    #[must_use]
    pub const fn check_parameters(
        &self,
        requested: usize,
    ) -> Result<(), LimitsError> {
        self.check_resource(ResourceKind::Parameters, requested)
    }

    /// Checks symbolic-expression node count.
    #[must_use]
    pub const fn check_expression_nodes(
        &self,
        requested: usize,
    ) -> Result<(), LimitsError> {
        self.check_resource(
            ResourceKind::ExpressionNodes,
            requested,
        )
    }

    /// Checks region count.
    #[must_use]
    pub const fn check_regions(
        &self,
        requested: usize,
    ) -> Result<(), LimitsError> {
        self.check_resource(ResourceKind::Regions, requested)
    }

    /// Checks block count.
    #[must_use]
    pub const fn check_blocks(
        &self,
        requested: usize,
    ) -> Result<(), LimitsError> {
        self.check_resource(ResourceKind::Blocks, requested)
    }

    /// Checks IR-value count.
    #[must_use]
    pub const fn check_values(
        &self,
        requested: usize,
    ) -> Result<(), LimitsError> {
        self.check_resource(ResourceKind::Values, requested)
    }

    /// Checks symbol count.
    #[must_use]
    pub const fn check_symbols(
        &self,
        requested: usize,
    ) -> Result<(), LimitsError> {
        self.check_resource(ResourceKind::Symbols, requested)
    }

    /// Checks dependency count.
    #[must_use]
    pub const fn check_dependencies(
        &self,
        requested: usize,
    ) -> Result<(), LimitsError> {
        self.check_resource(
            ResourceKind::Dependencies,
            requested,
        )
    }

    /// Checks control-flow nesting depth.
    #[must_use]
    pub const fn check_control_flow_depth(
        &self,
        requested: usize,
    ) -> Result<(), LimitsError> {
        self.check_resource(
            ResourceKind::ControlFlowDepth,
            requested,
        )
    }

    /// Checks general IR nesting depth.
    #[must_use]
    pub const fn check_nesting_depth(
        &self,
        requested: usize,
    ) -> Result<(), LimitsError> {
        self.check_resource(
            ResourceKind::NestingDepth,
            requested,
        )
    }

    /// Checks circuit depth.
    #[must_use]
    pub const fn check_depth(
        &self,
        requested: usize,
    ) -> Result<(), LimitsError> {
        self.check_circuit_depth(requested)
    }

    /// Checks circuit depth.
    #[must_use]
    pub const fn check_circuit_depth(
        &self,
        requested: usize,
    ) -> Result<(), LimitsError> {
        self.check_resource(
            ResourceKind::CircuitDepth,
            requested,
        )
    }

    /// Checks IR-processing depth.
    #[must_use]
    pub const fn check_processing_depth(
        &self,
        requested: usize,
    ) -> Result<(), LimitsError> {
        self.check_resource(
            ResourceKind::ProcessingDepth,
            requested,
        )
    }

    /// Checks measurement count.
    #[must_use]
    pub const fn check_measurements(
        &self,
        requested: usize,
    ) -> Result<(), LimitsError> {
        self.check_resource(
            ResourceKind::Measurements,
            requested,
        )
    }

    /// Checks barrier count.
    #[must_use]
    pub const fn check_barriers(
        &self,
        requested: usize,
    ) -> Result<(), LimitsError> {
        self.check_resource(
            ResourceKind::Barriers,
            requested,
        )
    }

    /// Checks pulse-operation count.
    #[must_use]
    pub const fn check_pulse_operations(
        &self,
        requested: usize,
    ) -> Result<(), LimitsError> {
        self.check_resource(
            ResourceKind::PulseOperations,
            requested,
        )
    }

    /// Checks waveform sample count.
    #[must_use]
    pub const fn check_waveform_samples(
        &self,
        requested: usize,
    ) -> Result<(), LimitsError> {
        self.check_resource(
            ResourceKind::WaveformSamples,
            requested,
        )
    }

    /// Checks waveform byte storage.
    #[must_use]
    pub const fn check_waveform_bytes(
        &self,
        requested: usize,
    ) -> Result<(), LimitsError> {
        self.check_resource(
            ResourceKind::WaveformBytes,
            requested,
        )
    }

    /// Checks channel count.
    #[must_use]
    pub const fn check_channels(
        &self,
        requested: usize,
    ) -> Result<(), LimitsError> {
        self.check_resource(ResourceKind::Channels, requested)
    }

    /// Checks frame count.
    #[must_use]
    pub const fn check_frames(
        &self,
        requested: usize,
    ) -> Result<(), LimitsError> {
        self.check_resource(ResourceKind::Frames, requested)
    }

    /// Checks scheduled-operation count.
    #[must_use]
    pub const fn check_scheduled_operations(
        &self,
        requested: usize,
    ) -> Result<(), LimitsError> {
        self.check_resource(
            ResourceKind::ScheduledOperations,
            requested,
        )
    }

    /// Checks mapping-entry count.
    #[must_use]
    pub const fn check_mapping_entries(
        &self,
        requested: usize,
    ) -> Result<(), LimitsError> {
        self.check_resource(
            ResourceKind::MappingEntries,
            requested,
        )
    }

    /// Checks resource-requirement count.
    #[must_use]
    pub const fn check_resource_requirements(
        &self,
        requested: usize,
    ) -> Result<(), LimitsError> {
        self.check_resource(
            ResourceKind::ResourceRequirements,
            requested,
        )
    }

    /// Checks extension count.
    #[must_use]
    pub const fn check_extensions(
        &self,
        requested: usize,
    ) -> Result<(), LimitsError> {
        self.check_resource(
            ResourceKind::Extensions,
            requested,
        )
    }

    /// Checks diagnostic count.
    #[must_use]
    pub const fn check_diagnostics(
        &self,
        requested: usize,
    ) -> Result<(), LimitsError> {
        self.check_resource(
            ResourceKind::Diagnostics,
            requested,
        )
    }

    /// Checks metadata byte size.
    #[must_use]
    pub const fn check_metadata_bytes(
        &self,
        requested: usize,
    ) -> Result<(), LimitsError> {
        self.check_resource(
            ResourceKind::MetadataBytes,
            requested,
        )
    }

    /// Checks source-information byte size.
    #[must_use]
    pub const fn check_source_bytes(
        &self,
        requested: usize,
    ) -> Result<(), LimitsError> {
        self.check_resource(
            ResourceKind::SourceBytes,
            requested,
        )
    }

    /// Checks serialized program byte size.
    #[must_use]
    pub const fn check_program_bytes(
        &self,
        requested: usize,
    ) -> Result<(), LimitsError> {
        self.check_resource(
            ResourceKind::ProgramBytes,
            requested,
        )
    }

    /// Checks validation work.
    #[must_use]
    pub const fn check_validation_steps(
        &self,
        requested: usize,
    ) -> Result<(), LimitsError> {
        self.check_resource(
            ResourceKind::ValidationSteps,
            requested,
        )
    }

    /// Checks analysis work.
    #[must_use]
    pub const fn check_analysis_steps(
        &self,
        requested: usize,
    ) -> Result<(), LimitsError> {
        self.check_resource(
            ResourceKind::AnalysisSteps,
            requested,
        )
    }

    /// Checks transformation work.
    #[must_use]
    pub const fn check_transformation_steps(
        &self,
        requested: usize,
    ) -> Result<(), LimitsError> {
        self.check_resource(
            ResourceKind::TransformationSteps,
            requested,
        )
    }

    /// Checks compilation work.
    #[must_use]
    pub const fn check_compilation_steps(
        &self,
        requested: usize,
    ) -> Result<(), LimitsError> {
        self.check_resource(
            ResourceKind::CompilationSteps,
            requested,
        )
    }

    /// Checks abstract schedule time.
    #[must_use]
    pub const fn check_schedule_time_units(
        &self,
        requested: u128,
    ) -> Result<(), LimitsError> {
        if requested > self.max_schedule_time_units {
            return Err(LimitsError::ScheduleTimeExceeded {
                requested,
                maximum: self.max_schedule_time_units,
            });
        }

        Ok(())
    }
}

// =============================================================================
// Generic resource checking
// =============================================================================

impl QuantumIrLimits {
    /// Checks an arbitrary finite resource category.
    ///
    /// `ScheduleTime` is intentionally handled by the dedicated
    /// `check_schedule_time_units` API because schedule time uses `u128`.
    #[must_use]
    pub const fn check_resource(
        &self,
        resource: ResourceKind,
        requested: usize,
    ) -> Result<(), LimitsError> {
        let maximum = match resource {
            ResourceKind::LogicalQubits => self.max_qubits,
            ResourceKind::ClassicalBits => {
                self.max_classical_bits
            }
            ResourceKind::Registers => self.max_registers,
            ResourceKind::Operations => self.max_operations,
            ResourceKind::OperandsPerOperation => {
                self.max_operands
            }
            ResourceKind::ParametersPerOperation => {
                self.max_parameters_per_operation
            }
            ResourceKind::Parameters => {
                self.max_parameters
            }
            ResourceKind::ExpressionNodes => {
                self.max_expression_nodes
            }
            ResourceKind::Regions => self.max_regions,
            ResourceKind::Blocks => self.max_blocks,
            ResourceKind::Values => self.max_values,
            ResourceKind::Symbols => self.max_symbols,
            ResourceKind::Dependencies => {
                self.max_dependencies
            }
            ResourceKind::ControlFlowDepth => {
                self.max_control_flow_depth
            }
            ResourceKind::NestingDepth => {
                self.max_nesting_depth
            }
            ResourceKind::CircuitDepth => {
                self.max_circuit_depth
            }
            ResourceKind::Measurements => {
                self.max_measurements
            }
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
            ResourceKind::MappingEntries => {
                self.max_mapping_entries
            }
            ResourceKind::ResourceRequirements => {
                self.max_resource_requirements
            }
            ResourceKind::Extensions => {
                self.max_extensions
            }
            ResourceKind::Diagnostics => {
                self.max_diagnostics
            }
            ResourceKind::MetadataBytes => {
                self.max_metadata_bytes
            }
            ResourceKind::SourceBytes => {
                self.max_source_bytes
            }
            ResourceKind::ProgramBytes => {
                self.max_program_bytes
            }
            ResourceKind::ValidationSteps => {
                self.max_validation_steps
            }
            ResourceKind::AnalysisSteps => {
                self.max_analysis_steps
            }
            ResourceKind::TransformationSteps => {
                self.max_transformation_steps
            }
            ResourceKind::CompilationSteps => {
                self.max_compilation_steps
            }
            ResourceKind::ProcessingDepth => {
                self.max_processing_depth
            }
        };

        Self::check(resource, requested, maximum)
    }

    /// Internal allocation-free finite-resource check.
    #[must_use]
    pub const fn check(
        resource: ResourceKind,
        requested: usize,
        maximum: usize,
    ) -> Result<(), LimitsError> {
        if requested > maximum {
            return Err(LimitsError::ResourceExceeded {
                resource,
                requested,
                maximum,
            });
        }

        Ok(())
    }
}

// =============================================================================
// Overflow-safe accounting
// =============================================================================

impl QuantumIrLimits {
    /// Adds two finite resource quantities without overflow.
    ///
    /// The resulting quantity must not exceed `maximum`.
    #[must_use]
    pub const fn checked_add(
        resource: ResourceKind,
        current: usize,
        additional: usize,
        maximum: usize,
    ) -> Result<usize, LimitsError> {
        match current.checked_add(additional) {
            Some(total) => {
                if total > maximum {
                    Err(LimitsError::ResourceExceeded {
                        resource,
                        requested: total,
                        maximum,
                    })
                } else {
                    Ok(total)
                }
            }

            None => Err(LimitsError::ArithmeticOverflow {
                resource,
            }),
        }
    }

    /// Multiplies two finite resource quantities without overflow.
    ///
    /// The resulting quantity must not exceed `maximum`.
    #[must_use]
    pub const fn checked_mul(
        resource: ResourceKind,
        left: usize,
        right: usize,
        maximum: usize,
    ) -> Result<usize, LimitsError> {
        match left.checked_mul(right) {
            Some(total) => {
                if total > maximum {
                    Err(LimitsError::ResourceExceeded {
                        resource,
                        requested: total,
                        maximum,
                    })
                } else {
                    Ok(total)
                }
            }

            None => Err(
                LimitsError::ArithmeticMultiplicationOverflow {
                    resource,
                },
            ),
        }
    }

    /// Adds schedule-time quantities without overflow.
    #[must_use]
    pub const fn checked_add_time(
        current: u128,
        additional: u128,
        maximum: u128,
    ) -> Result<u128, LimitsError> {
        match current.checked_add(additional) {
            Some(total) => {
                if total > maximum {
                    Err(LimitsError::ScheduleTimeExceeded {
                        requested: total,
                        maximum,
                    })
                } else {
                    Ok(total)
                }
            }

            None => Err(LimitsError::TimeArithmeticOverflow),
        }
    }

    /// Multiplies schedule-time quantities without overflow.
    #[must_use]
    pub const fn checked_mul_time(
        left: u128,
        right: u128,
        maximum: u128,
    ) -> Result<u128, LimitsError> {
        match left.checked_mul(right) {
            Some(total) => {
                if total > maximum {
                    Err(LimitsError::ScheduleTimeExceeded {
                        requested: total,
                        maximum,
                    })
                } else {
                    Ok(total)
                }
            }

            None => Err(
                LimitsError::TimeArithmeticMultiplicationOverflow,
            ),
        }
    }

    /// Adds two finite quantities and checks them against this policy.
    #[must_use]
    pub const fn checked_add_with_policy(
        &self,
        resource: ResourceKind,
        current: usize,
        additional: usize,
    ) -> Result<usize, LimitsError> {
        let maximum = match resource {
            ResourceKind::LogicalQubits => self.max_qubits,
            ResourceKind::ClassicalBits => {
                self.max_classical_bits
            }
            ResourceKind::Registers => self.max_registers,
            ResourceKind::Operations => self.max_operations,
            ResourceKind::OperandsPerOperation => {
                self.max_operands
            }
            ResourceKind::ParametersPerOperation => {
                self.max_parameters_per_operation
            }
            ResourceKind::Parameters => {
                self.max_parameters
            }
            ResourceKind::ExpressionNodes => {
                self.max_expression_nodes
            }
            ResourceKind::Regions => self.max_regions,
            ResourceKind::Blocks => self.max_blocks,
            ResourceKind::Values => self.max_values,
            ResourceKind::Symbols => self.max_symbols,
            ResourceKind::Dependencies => {
                self.max_dependencies
            }
            ResourceKind::ControlFlowDepth => {
                self.max_control_flow_depth
            }
            ResourceKind::NestingDepth => {
                self.max_nesting_depth
            }
            ResourceKind::CircuitDepth => {
                self.max_circuit_depth
            }
            ResourceKind::Measurements => {
                self.max_measurements
            }
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
            ResourceKind::MappingEntries => {
                self.max_mapping_entries
            }
            ResourceKind::ResourceRequirements => {
                self.max_resource_requirements
            }
            ResourceKind::Extensions => {
                self.max_extensions
            }
            ResourceKind::Diagnostics => {
                self.max_diagnostics
            }
            ResourceKind::MetadataBytes => {
                self.max_metadata_bytes
            }
            ResourceKind::SourceBytes => {
                self.max_source_bytes
            }
            ResourceKind::ProgramBytes => {
                self.max_program_bytes
            }
            ResourceKind::ValidationSteps => {
                self.max_validation_steps
            }
            ResourceKind::AnalysisSteps => {
                self.max_analysis_steps
            }
            ResourceKind::TransformationSteps => {
                self.max_transformation_steps
            }
            ResourceKind::CompilationSteps => {
                self.max_compilation_steps
            }
            ResourceKind::ProcessingDepth => {
                self.max_processing_depth
            }
        };

        Self::checked_add(
            resource,
            current,
            additional,
            maximum,
        )
    }

    /// Multiplies two finite quantities and checks the result against this
    /// policy.
    #[must_use]
    pub const fn checked_mul_with_policy(
        &self,
        resource: ResourceKind,
        left: usize,
        right: usize,
    ) -> Result<usize, LimitsError> {
        let maximum = match resource {
            ResourceKind::LogicalQubits => self.max_qubits,
            ResourceKind::ClassicalBits => {
                self.max_classical_bits
            }
            ResourceKind::Registers => self.max_registers,
            ResourceKind::Operations => self.max_operations,
            ResourceKind::OperandsPerOperation => {
                self.max_operands
            }
            ResourceKind::ParametersPerOperation => {
                self.max_parameters_per_operation
            }
            ResourceKind::Parameters => {
                self.max_parameters
            }
            ResourceKind::ExpressionNodes => {
                self.max_expression_nodes
            }
            ResourceKind::Regions => self.max_regions,
            ResourceKind::Blocks => self.max_blocks,
            ResourceKind::Values => self.max_values,
            ResourceKind::Symbols => self.max_symbols,
            ResourceKind::Dependencies => {
                self.max_dependencies
            }
            ResourceKind::ControlFlowDepth => {
                self.max_control_flow_depth
            }
            ResourceKind::NestingDepth => {
                self.max_nesting_depth
            }
            ResourceKind::CircuitDepth => {
                self.max_circuit_depth
            }
            ResourceKind::Measurements => {
                self.max_measurements
            }
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
            ResourceKind::MappingEntries => {
                self.max_mapping_entries
            }
            ResourceKind::ResourceRequirements => {
                self.max_resource_requirements
            }
            ResourceKind::Extensions => {
                self.max_extensions
            }
            ResourceKind::Diagnostics => {
                self.max_diagnostics
            }
            ResourceKind::MetadataBytes => {
                self.max_metadata_bytes
            }
            ResourceKind::SourceBytes => {
                self.max_source_bytes
            }
            ResourceKind::ProgramBytes => {
                self.max_program_bytes
            }
            ResourceKind::ValidationSteps => {
                self.max_validation_steps
            }
            ResourceKind::AnalysisSteps => {
                self.max_analysis_steps
            }
            ResourceKind::TransformationSteps => {
                self.max_transformation_steps
            }
            ResourceKind::CompilationSteps => {
                self.max_compilation_steps
            }
            ResourceKind::ProcessingDepth => {
                self.max_processing_depth
            }
        };

        Self::checked_mul(
            resource,
            left,
            right,
            maximum,
        )
    }

    /// Adds schedule time using this policy's schedule-time maximum.
    #[must_use]
    pub const fn checked_add_time_with_policy(
        &self,
        current: u128,
        additional: u128,
    ) -> Result<u128, LimitsError> {
        Self::checked_add_time(
            current,
            additional,
            self.max_schedule_time_units,
        )
    }

    /// Multiplies schedule time using this policy's schedule-time maximum.
    #[must_use]
    pub const fn checked_mul_time_with_policy(
        &self,
        left: u128,
        right: u128,
    ) -> Result<u128, LimitsError> {
        Self::checked_mul_time(
            left,
            right,
            self.max_schedule_time_units,
        )
    }
}

// =============================================================================
// Default
// =============================================================================

impl Default for QuantumIrLimits {
    fn default() -> Self {
        Self::production()
    }
}

// =============================================================================
// Tests
// =============================================================================

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
    fn default_is_production() {
        assert_eq!(
            QuantumIrLimits::default(),
            QuantumIrLimits::production()
        );
    }

    #[test]
    fn unbounded_policy_is_valid() {
        let limits = QuantumIrLimits::unbounded();

        assert!(limits.validate().is_ok());
        assert!(limits.is_unbounded());
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

        assert!(limits.validate().is_ok());
        assert!(limits.check_qubits(0).is_ok());
        assert!(limits.check_operations(0).is_ok());
        assert!(limits.check_pulse_operations(0).is_ok());
    }

    #[test]
    fn validation_budget_must_not_be_zero() {
        let limits = QuantumIrLimits::production()
            .with_max_validation_steps(0);

        assert_eq!(
            limits.validate(),
            Err(LimitsError::InvalidConfiguration {
                field: "max_validation_steps",
                value: 0,
            })
        );
    }

    #[test]
    fn analysis_budget_must_not_be_zero() {
        let limits = QuantumIrLimits::production()
            .with_max_analysis_steps(0);

        assert_eq!(
            limits.validate(),
            Err(LimitsError::InvalidConfiguration {
                field: "max_analysis_steps",
                value: 0,
            })
        );
    }

    #[test]
    fn exact_limit_is_allowed() {
        let limits = QuantumIrLimits::production()
            .with_max_qubits(128);

        assert!(limits.check_qubits(128).is_ok());
    }

    #[test]
    fn exceeding_limit_is_rejected() {
        let limits = QuantumIrLimits::production()
            .with_max_qubits(128);

        assert_eq!(
            limits.check_qubits(129),
            Err(LimitsError::ResourceExceeded {
                resource: ResourceKind::LogicalQubits,
                requested: 129,
                maximum: 128,
            })
        );
    }

    #[test]
    fn zero_resource_limit_is_valid() {
        let limits = QuantumIrLimits::production()
            .with_max_qubits(0);

        assert!(limits.check_qubits(0).is_ok());
        assert!(limits.check_qubits(1).is_err());
    }

    #[test]
    fn no_special_sixty_three_qubit_boundary_exists() {
        let limits = QuantumIrLimits::unbounded();

        assert!(limits.check_qubits(63).is_ok());
        assert!(limits.check_qubits(64).is_ok());
        assert!(limits.check_qubits(128).is_ok());
        assert!(limits.check_qubits(4_096).is_ok());
        assert!(limits.check_qubits(1_000_000).is_ok());
    }

    #[test]
    fn large_qubit_policy_is_explicit() {
        let limits = QuantumIrLimits::production()
            .with_max_qubits(1_000_000);

        assert!(
            limits
                .check_qubits(1_000_000)
                .is_ok()
        );

        assert!(
            limits
                .check_qubits(1_000_001)
                .is_err()
        );
    }

    #[test]
    fn qubit_policy_is_independent_from_pulse_policy() {
        let limits = QuantumIrLimits::production()
            .with_max_qubits(1)
            .with_max_pulse_operations(100);

        assert!(limits.check_qubits(1).is_ok());
        assert!(
            limits
                .check_pulse_operations(100)
                .is_ok()
        );
    }

    #[test]
    fn waveform_limits_are_independent() {
        let limits = QuantumIrLimits::production()
            .with_max_waveform_samples(1_000)
            .with_max_waveform_bytes(8_000);

        assert!(
            limits
                .check_waveform_samples(1_000)
                .is_ok()
        );

        assert!(
            limits
                .check_waveform_samples(1_001)
                .is_err()
        );

        assert!(
            limits
                .check_waveform_bytes(8_000)
                .is_ok()
        );

        assert!(
            limits
                .check_waveform_bytes(8_001)
                .is_err()
        );
    }

    #[test]
    fn expression_limits_are_enforced() {
        let limits = QuantumIrLimits::production()
            .with_max_expression_nodes(100);

        assert!(
            limits
                .check_expression_nodes(100)
                .is_ok()
        );

        assert!(
            limits
                .check_expression_nodes(101)
                .is_err()
        );
    }

    #[test]
    fn dependency_limits_are_enforced() {
        let limits = QuantumIrLimits::production()
            .with_max_dependencies(10);

        assert!(
            limits
                .check_dependencies(10)
                .is_ok()
        );

        assert!(
            limits
                .check_dependencies(11)
                .is_err()
        );
    }

    #[test]
    fn processing_depth_is_independent_from_circuit_depth() {
        let limits = QuantumIrLimits::production()
            .with_max_processing_depth(8)
            .with_max_circuit_depth(100_000);

        assert!(
            limits
                .check_processing_depth(8)
                .is_ok()
        );

        assert!(
            limits
                .check_circuit_depth(100_000)
                .is_ok()
        );
    }

    #[test]
    fn schedule_time_limit_is_enforced() {
        let limits = QuantumIrLimits::production()
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
        assert_eq!(
            QuantumIrLimits::checked_add(
                ResourceKind::Operations,
                5,
                5,
                10,
            ),
            Ok(10)
        );
    }

    #[test]
    fn checked_add_rejects_limit_excess() {
        assert_eq!(
            QuantumIrLimits::checked_add(
                ResourceKind::Operations,
                6,
                5,
                10,
            ),
            Err(LimitsError::ResourceExceeded {
                resource: ResourceKind::Operations,
                requested: 11,
                maximum: 10,
            })
        );
    }

    #[test]
    fn checked_add_rejects_overflow() {
        assert_eq!(
            QuantumIrLimits::checked_add(
                ResourceKind::Operations,
                usize::MAX,
                1,
                usize::MAX,
            ),
            Err(LimitsError::ArithmeticOverflow {
                resource: ResourceKind::Operations,
            })
        );
    }

    #[test]
    fn checked_mul_accepts_exact_limit() {
        assert_eq!(
            QuantumIrLimits::checked_mul(
                ResourceKind::ValidationSteps,
                10,
                10,
                100,
            ),
            Ok(100)
        );
    }

    #[test]
    fn checked_mul_rejects_limit_excess() {
        assert_eq!(
            QuantumIrLimits::checked_mul(
                ResourceKind::ValidationSteps,
                11,
                10,
                100,
            ),
            Err(LimitsError::ResourceExceeded {
                resource: ResourceKind::ValidationSteps,
                requested: 110,
                maximum: 100,
            })
        );
    }

    #[test]
    fn checked_mul_rejects_overflow() {
        assert_eq!(
            QuantumIrLimits::checked_mul(
                ResourceKind::AnalysisSteps,
                usize::MAX,
                2,
                usize::MAX,
            ),
            Err(
                LimitsError::ArithmeticMultiplicationOverflow {
                    resource: ResourceKind::AnalysisSteps,
                }
            )
        );
    }

    #[test]
    fn checked_time_add_rejects_overflow() {
        assert_eq!(
            QuantumIrLimits::checked_add_time(
                u128::MAX,
                1,
                u128::MAX,
            ),
            Err(LimitsError::TimeArithmeticOverflow)
        );
    }

    #[test]
    fn checked_time_mul_rejects_overflow() {
        assert_eq!(
            QuantumIrLimits::checked_mul_time(
                u128::MAX,
                2,
                u128::MAX,
            ),
            Err(
                LimitsError::TimeArithmeticMultiplicationOverflow
            )
        );
    }

    #[test]
    fn generic_resource_check_works() {
        let limits = QuantumIrLimits::production()
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
    fn policy_checked_add_uses_policy_maximum() {
        let limits = QuantumIrLimits::production()
            .with_max_operations(10);

        assert_eq!(
            limits.checked_add_with_policy(
                ResourceKind::Operations,
                6,
                4,
            ),
            Ok(10)
        );

        assert!(
            limits
                .checked_add_with_policy(
                    ResourceKind::Operations,
                    6,
                    5,
                )
                .is_err()
        );
    }

    #[test]
    fn policy_checked_mul_uses_policy_maximum() {
        let limits = QuantumIrLimits::production()
            .with_max_waveform_samples(100);

        assert_eq!(
            limits.checked_mul_with_policy(
                ResourceKind::WaveformSamples,
                10,
                10,
            ),
            Ok(100)
        );

        assert!(
            limits
                .checked_mul_with_policy(
                    ResourceKind::WaveformSamples,
                    11,
                    10,
                )
                .is_err()
        );
    }

    #[test]
    fn policy_checked_time_add_uses_policy_maximum() {
        let limits = QuantumIrLimits::production()
            .with_max_schedule_time_units(100);

        assert_eq!(
            limits.checked_add_time_with_policy(40, 60),
            Ok(100)
        );

        assert!(
            limits
                .checked_add_time_with_policy(40, 61)
                .is_err()
        );
    }

    #[test]
    fn policy_checked_time_mul_uses_policy_maximum() {
        let limits = QuantumIrLimits::production()
            .with_max_schedule_time_units(100);

        assert_eq!(
            limits.checked_mul_time_with_policy(10, 10),
            Ok(100)
        );

        assert!(
            limits
                .checked_mul_time_with_policy(11, 10)
                .is_err()
        );
    }

    #[test]
    fn builder_methods_are_composable() {
        let limits = QuantumIrLimits::production()
            .with_max_qubits(128)
            .with_max_classical_bits(128)
            .with_max_registers(16)
            .with_max_operations(10_000)
            .with_max_operands(8)
            .with_max_parameters_per_operation(4)
            .with_max_parameters(1_000)
            .with_max_expression_nodes(10_000)
            .with_max_regions(128)
            .with_max_blocks(256)
            .with_max_values(1_024)
            .with_max_symbols(1_024)
            .with_max_dependencies(4_096)
            .with_max_control_flow_depth(16)
            .with_max_nesting_depth(16)
            .with_max_circuit_depth(512)
            .with_max_processing_depth(32)
            .with_max_measurements(128)
            .with_max_barriers(32)
            .with_max_pulse_operations(10_000)
            .with_max_waveform_samples(100_000)
            .with_max_waveform_bytes(1_000_000)
            .with_max_channels(128)
            .with_max_frames(128)
            .with_max_scheduled_operations(20_000)
            .with_max_mapping_entries(128)
            .with_max_resource_requirements(128)
            .with_max_extensions(128)
            .with_max_diagnostics(1_000)
            .with_max_metadata_bytes(4_096)
            .with_max_source_bytes(1_000_000)
            .with_max_program_bytes(10_000_000)
            .with_max_validation_steps(100_000)
            .with_max_analysis_steps(100_000)
            .with_max_transformation_steps(1_000_000)
            .with_max_compilation_steps(1_000_000)
            .with_max_schedule_time_units(1_000_000);

        assert_eq!(limits.max_qubits(), 128);
        assert_eq!(limits.max_classical_bits(), 128);
        assert_eq!(limits.max_registers(), 16);
        assert_eq!(limits.max_operations(), 10_000);
        assert_eq!(limits.max_operands(), 8);
        assert_eq!(
            limits.max_parameters_per_operation(),
            4
        );
        assert_eq!(limits.max_parameters(), 1_000);
        assert_eq!(
            limits.max_expression_nodes(),
            10_000
        );
        assert_eq!(limits.max_regions(), 128);
        assert_eq!(limits.max_blocks(), 256);
        assert_eq!(limits.max_values(), 1_024);
        assert_eq!(limits.max_symbols(), 1_024);
        assert_eq!(limits.max_dependencies(), 4_096);
        assert_eq!(
            limits.max_control_flow_depth(),
            16
        );
        assert_eq!(
            limits.max_nesting_depth(),
            16
        );
        assert_eq!(
            limits.max_circuit_depth(),
            512
        );
        assert_eq!(
            limits.max_processing_depth(),
            32
        );
        assert_eq!(limits.max_measurements(), 128);
        assert_eq!(limits.max_barriers(), 32);
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
        assert_eq!(limits.max_channels(), 128);
        assert_eq!(limits.max_frames(), 128);
        assert_eq!(
            limits.max_scheduled_operations(),
            20_000
        );
        assert_eq!(
            limits.max_mapping_entries(),
            128
        );
        assert_eq!(
            limits.max_resource_requirements(),
            128
        );
        assert_eq!(limits.max_extensions(), 128);
        assert_eq!(limits.max_diagnostics(), 1_000);
        assert_eq!(
            limits.max_metadata_bytes(),
            4_096
        );
        assert_eq!(
            limits.max_source_bytes(),
            1_000_000
        );
        assert_eq!(
            limits.max_program_bytes(),
            10_000_000
        );
        assert_eq!(
            limits.max_validation_steps(),
            100_000
        );
        assert_eq!(
            limits.max_analysis_steps(),
            100_000
        );
        assert_eq!(
            limits.max_transformation_steps(),
            1_000_000
        );
        assert_eq!(
            limits.max_compilation_steps(),
            1_000_000
        );
        assert_eq!(
            limits.max_schedule_time_units(),
            1_000_000
        );

        assert!(limits.validate().is_ok());
    }

    #[test]
    fn resource_names_are_stable() {
        assert_eq!(
            ResourceKind::LogicalQubits.as_str(),
            "logical qubits"
        );

        assert_eq!(
            ResourceKind::ExpressionNodes.as_str(),
            "expression nodes"
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
    fn default_policy_supports_pulse_ir() {
        let limits = QuantumIrLimits::production();

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
                .check_waveform_bytes(1)
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
    fn no_fixed_qubit_architecture_is_encoded() {
        let limits = QuantumIrLimits::unbounded();

        assert!(limits.check_qubits(1).is_ok());
        assert!(limits.check_qubits(63).is_ok());
        assert!(limits.check_qubits(64).is_ok());
        assert!(limits.check_qubits(128).is_ok());
        assert!(limits.check_qubits(4_096).is_ok());
        assert!(limits.check_qubits(1_000_000).is_ok());
    }
}