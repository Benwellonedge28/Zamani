//! Zamani Quantum IR — Resource Validation
//!
//! Production-grade resource-policy validation for the canonical Zamani
//! Quantum Intermediate Representation.
//!
//! # Architectural role
//!
//! This module answers:
//!
//! > Does the supplied IR resource usage satisfy the explicit resource policy
//! > for this validation/compilation boundary?
//!
//! It does NOT answer:
//!
//! - whether a physical QPU exists;
//! - whether a target has enough physical qubits;
//! - whether logical qubits can be routed;
//! - whether qubits are connected;
//! - whether a backend supports an operation;
//! - how operations are scheduled;
//! - how pulses are calibrated;
//! - how a simulator represents quantum state;
//! - how QEC is decoded;
//! - how hardware is allocated.
//!
//! Those responsibilities belong to downstream subsystems.
//!
//! # Architectural principle
//!
//! ```text
//! Semantic IR
//!      |
//!      v
//! Resource usage
//!      |
//!      v
//! QuantumIrLimits
//!      |
//!      v
//! Resource-policy validation
//!      |
//!      +--------------------+
//!      |                    |
//!      v                    v
//! valid                  rejected
//!      |
//!      v
//! downstream target/resource compatibility
//! ```
//!
//! `QuantumIrLimits` is a policy boundary. It is not the maximum size of the
//! Zamani language and it is not a description of a particular quantum machine.
//!
//! # Scalability
//!
//! Zamani must support the same semantic program model from very small
//! workloads through arbitrarily large finite workloads permitted by:
//!
//! - the representable identifier domain;
//! - available host resources;
//! - explicit security/resource policy;
//! - compiler/runtime resources;
//! - target resources.
//!
//! This module therefore:
//!
//! - contains no fixed machine-size constant;
//! - contains no `MAX_QUBITS` constant;
//! - contains no fixed register size;
//! - contains no fixed topology;
//! - contains no vendor-specific resource assumptions;
//! - uses `QubitId` rather than a locally defined qubit identifier;
//! - uses sparse sets for touched qubits;
//! - uses checked arithmetic;
//! - never allocates one validation slot per declared qubit;
//! - treats policy limits as caller-supplied constraints;
//! - permits sparse/high logical identifiers when the enclosing namespace
//!   permits them;
//! - never interprets `usize::MAX` as semantic infinity.
//!
//! # Sparse validation
//!
//! A program may declare a very large logical namespace while touching only a
//! small subset of it.
//!
//! Validation must therefore prefer:
//!
//! ```text
//! touched qubits
//! ```
//!
//! over:
//!
//! ```text
//! every declared qubit
//! ```
//!
//! For example, validating `q_900_000_000` must not allocate a vector with
//! 900,000,001 entries.
//!
//! # Resource accounting
//!
//! This module distinguishes:
//!
//! ```text
//! ResourceUsage
//!     = observed/declared usage of an IR object
//!
//! QuantumIrLimits
//!     = explicit policy ceiling
//!
//! Resource validation
//!     = comparison between usage and policy
//! ```
//!
//! It deliberately does not duplicate `ResourceRequirement` semantics from
//! `quantum::ir::resource`.
//!
//! # Qubit identity
//!
//! The canonical logical qubit identifier is:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! ```
//!
//! This module imports it directly and never defines a replacement identifier.
//!
//! `QubitId` represents logical identity. It does not establish physical
//! hardware existence or allocation.
//!
//! # Trust boundary
//!
//! Resource validation must be performed even if an object was created through
//! a constructor that already performed local checks.
//!
//! IR may originate from:
//!
//! - frontend lowering;
//! - deserialization;
//! - generated programs;
//! - optimization;
//! - transformation passes;
//! - caches;
//! - replay;
//! - distributed compilation;
//! - external tools;
//! - future dialects/extensions.
//!
//! Therefore this module must treat its inputs as potentially untrusted.
//!
//! # Error handling
//!
//! Policy failures are returned through the canonical `IrResult`/`IrError`
//! boundary used by the validation subsystem.
//!
//! Arithmetic overflow is never silently wrapped.
//!
//! # Determinism
//!
//! Validation is:
//!
//! - read-only;
//! - deterministic;
//! - side-effect free;
//! - independent of hash-map iteration order;
//! - independent of global mutable state.
//!
//! `BTreeSet` is used where deterministic ordered resource identity tracking is
//! required.
//!
//! # Thread safety
//!
//! No global mutable state is used.
//!
//! Independent resource-validation calls can therefore be executed
//! concurrently by callers without synchronization inside this module.
//!
//! # Hardware independence
//!
//! This module MUST NOT depend on:
//!
//! - `quantum::hardware`;
//! - `quantum::routing`;
//! - `quantum::scheduling`;
//! - `quantum::optimization`;
//! - simulator implementations;
//! - QEC implementations;
//! - backend implementations;
//! - provider SDKs.
//!
//! # Rust compatibility
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust.
//!
//! Requirements:
//!
//! - no nightly features;
//! - no `unsafe`;
//! - no additional dependencies.
//!
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler-enforced.
//!
//! # Integration contract
//!
//! This module consumes:
//!
//! ```text
//! quantum::ir::limits::QuantumIrLimits
//! quantum::ir::qubit::QubitId
//! quantum::ir::errors::{IrError, IrResult}
//! ```
//!
//! The higher-level validation coordinator may consume this module through:
//!
//! ```text
//! validate_usage
//! validate_qubit_ids
//! ResourceUsage
//! ResourceUsageBuilder
//! ```
//!
//! This module does not own whole-program traversal. The caller is responsible
//! for extracting resource usage from the canonical IR.
//!
//! # Completion contract
//!
//! This file is complete when:
//!
//! - no fixed machine-size limit is encoded;
//! - no hardware dependency exists;
//! - `QubitId` comes from `quantum::ir::qubit`;
//! - all arithmetic is checked;
//! - all policy checks use `QuantumIrLimits`;
//! - sparse qubit validation does not allocate by namespace size;
//! - invalid policy configuration is rejected;
//! - resource overuse is reported through canonical IR errors;
//! - usage accumulation is deterministic;
//! - the API is read-only from the validator's perspective;
//! - Rust 1.97.1 compiles it;
//! - the module contains no unsafe code.
//!
//! -----------------------------------------------------------------------------
//! Public API
//! -----------------------------------------------------------------------------
//!
//! `ResourceUsage`
//!     Immutable-style snapshot of resource usage.
//!
//! `ResourceUsageBuilder`
//!     Checked incremental resource accumulator.
//!
//! `validate_usage`
//!     Validates a complete resource-usage snapshot against policy.
//!
//! `validate_qubit_ids`
//!     Validates a sparse set/iterator of logical qubit identities against a
//!     declared namespace and explicit policy.
//!
//! `validate_count`
//!     Validates one resource count through the canonical limits API.
//!
//! `ResourceUsageError`
//!     Local accounting errors that occur before policy validation.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use super::super::errors::{IrError, IrResult};
use super::super::limits::QuantumIrLimits;
use super::super::qubit::QubitId;

// =============================================================================
// Resource usage
// =============================================================================

/// Resource usage observed or declared for one validation boundary.
///
/// All values are counts rather than hardware objects.
///
/// This type deliberately does not contain hardware resources, physical
/// topology, routing state, or allocation handles.
///
/// `usize` is used for counts because the canonical `QuantumIrLimits` API uses
/// `usize` for policy accounting. Semantic identities remain separate and are
/// represented by canonical ID types such as [`QubitId`].
///
/// # Important
///
/// These fields are usage measurements. They are not architectural maxima.
///
/// A value such as `4_096` here means that the object being validated contains
/// or requires that amount of the corresponding resource. It does not mean
/// Zamani is limited to that amount.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ResourceUsage {
    /// Number of logical qubits declared/referenced by the validated object.
    pub logical_qubits: usize,

    /// Number of classical bits.
    pub classical_bits: usize,

    /// Number of registers.
    pub registers: usize,

    /// Number of IR operations.
    pub operations: usize,

    /// Maximum operands attached to one operation.
    pub operands_per_operation: usize,

    /// Maximum parameters attached to one operation.
    pub parameters_per_operation: usize,

    /// Total symbolic parameters.
    pub parameters: usize,

    /// Total symbolic-expression nodes.
    pub expression_nodes: usize,

    /// Number of regions.
    pub regions: usize,

    /// Number of basic blocks.
    pub blocks: usize,

    /// Number of IR values.
    pub values: usize,

    /// Number of symbols.
    pub symbols: usize,

    /// Number of dependency edges.
    pub dependencies: usize,

    /// Maximum control-flow nesting depth.
    pub control_flow_depth: usize,

    /// Maximum general IR nesting depth.
    pub nesting_depth: usize,

    /// Maximum circuit/program execution depth.
    pub circuit_depth: usize,

    /// Number of measurement operations.
    pub measurements: usize,

    /// Number of barriers.
    pub barriers: usize,

    /// Number of pulse operations.
    pub pulse_operations: usize,

    /// Number of waveform samples.
    pub waveform_samples: usize,

    /// Number of bytes occupied by waveform storage.
    pub waveform_bytes: usize,

    /// Number of abstract channels.
    pub channels: usize,

    /// Number of abstract frames.
    pub frames: usize,

    /// Number of scheduled operations.
    pub scheduled_operations: usize,

    /// Number of logical/physical mapping entries.
    pub mapping_entries: usize,

    /// Number of resource requirements.
    pub resource_requirements: usize,

    /// Number of extensions.
    pub extensions: usize,

    /// Number of diagnostics.
    pub diagnostics: usize,

    /// Metadata storage in bytes.
    pub metadata_bytes: usize,

    /// Source-location/source-information storage in bytes.
    pub source_bytes: usize,

    /// Serialized program size in bytes.
    pub program_bytes: usize,

    /// Estimated validation work.
    pub validation_steps: usize,

    /// Estimated analysis work.
    pub analysis_steps: usize,

    /// Estimated transformation work.
    pub transformation_steps: usize,

    /// Estimated compilation work.
    pub compilation_steps: usize,

    /// Maximum processing/recursion depth.
    pub processing_depth: usize,
}

impl ResourceUsage {
    /// Creates an empty resource-usage snapshot.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            logical_qubits: 0,
            classical_bits: 0,
            registers: 0,
            operations: 0,
            operands_per_operation: 0,
            parameters_per_operation: 0,
            parameters: 0,
            expression_nodes: 0,
            regions: 0,
            blocks: 0,
            values: 0,
            symbols: 0,
            dependencies: 0,
            control_flow_depth: 0,
            nesting_depth: 0,
            circuit_depth: 0,
            measurements: 0,
            barriers: 0,
            pulse_operations: 0,
            waveform_samples: 0,
            waveform_bytes: 0,
            channels: 0,
            frames: 0,
            scheduled_operations: 0,
            mapping_entries: 0,
            resource_requirements: 0,
            extensions: 0,
            diagnostics: 0,
            metadata_bytes: 0,
            source_bytes: 0,
            program_bytes: 0,
            validation_steps: 0,
            analysis_steps: 0,
            transformation_steps: 0,
            compilation_steps: 0,
            processing_depth: 0,
        }
    }

    /// Creates an empty usage builder.
    #[must_use]
    pub const fn builder() -> ResourceUsageBuilder {
        ResourceUsageBuilder::new()
    }

    /// Returns the number of logical qubits.
    #[must_use]
    pub const fn logical_qubits(&self) -> usize {
        self.logical_qubits
    }

    /// Returns the number of classical bits.
    #[must_use]
    pub const fn classical_bits(&self) -> usize {
        self.classical_bits
    }

    /// Returns the number of operations.
    #[must_use]
    pub const fn operations(&self) -> usize {
        self.operations
    }

    /// Returns the number of measurements.
    #[must_use]
    pub const fn measurements(&self) -> usize {
        self.measurements
    }

    /// Returns the number of pulse operations.
    #[must_use]
    pub const fn pulse_operations(&self) -> usize {
        self.pulse_operations
    }

    /// Returns the number of waveform samples.
    #[must_use]
    pub const fn waveform_samples(&self) -> usize {
        self.waveform_samples
    }

    /// Returns the number of waveform bytes.
    #[must_use]
    pub const fn waveform_bytes(&self) -> usize {
        self.waveform_bytes
    }

    /// Returns the number of channels.
    #[must_use]
    pub const fn channels(&self) -> usize {
        self.channels
    }

    /// Returns the number of frames.
    #[must_use]
    pub const fn frames(&self) -> usize {
        self.frames
    }

    /// Returns the maximum operation operand count.
    #[must_use]
    pub const fn operands_per_operation(&self) -> usize {
        self.operands_per_operation
    }

    /// Returns the maximum operation parameter count.
    #[must_use]
    pub const fn parameters_per_operation(&self) -> usize {
        self.parameters_per_operation
    }

    /// Returns whether this snapshot contains no resource usage.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.logical_qubits == 0
            && self.classical_bits == 0
            && self.registers == 0
            && self.operations == 0
            && self.operands_per_operation == 0
            && self.parameters_per_operation == 0
            && self.parameters == 0
            && self.expression_nodes == 0
            && self.regions == 0
            && self.blocks == 0
            && self.values == 0
            && self.symbols == 0
            && self.dependencies == 0
            && self.control_flow_depth == 0
            && self.nesting_depth == 0
            && self.circuit_depth == 0
            && self.measurements == 0
            && self.barriers == 0
            && self.pulse_operations == 0
            && self.waveform_samples == 0
            && self.waveform_bytes == 0
            && self.channels == 0
            && self.frames == 0
            && self.scheduled_operations == 0
            && self.mapping_entries == 0
            && self.resource_requirements == 0
            && self.extensions == 0
            && self.diagnostics == 0
            && self.metadata_bytes == 0
            && self.source_bytes == 0
            && self.program_bytes == 0
            && self.validation_steps == 0
            && self.analysis_steps == 0
            && self.transformation_steps == 0
            && self.compilation_steps == 0
            && self.processing_depth == 0
    }

    /// Checked addition of two resource-usage snapshots.
    ///
    /// This is useful when independent regions are validated or analysed and
    /// their resource usage must be combined.
    ///
    /// Maximum/depth fields are combined using `max`, while cumulative fields
    /// are added.
    pub fn checked_add(self, other: Self) -> Result<Self, ResourceUsageError> {
        Ok(Self {
            logical_qubits: checked_add(
                self.logical_qubits,
                other.logical_qubits,
                ResourceField::LogicalQubits,
            )?,
            classical_bits: checked_add(
                self.classical_bits,
                other.classical_bits,
                ResourceField::ClassicalBits,
            )?,
            registers: checked_add(
                self.registers,
                other.registers,
                ResourceField::Registers,
            )?,
            operations: checked_add(
                self.operations,
                other.operations,
                ResourceField::Operations,
            )?,
            operands_per_operation: self
                .operands_per_operation
                .max(other.operands_per_operation),
            parameters_per_operation: self
                .parameters_per_operation
                .max(other.parameters_per_operation),
            parameters: checked_add(
                self.parameters,
                other.parameters,
                ResourceField::Parameters,
            )?,
            expression_nodes: checked_add(
                self.expression_nodes,
                other.expression_nodes,
                ResourceField::ExpressionNodes,
            )?,
            regions: checked_add(
                self.regions,
                other.regions,
                ResourceField::Regions,
            )?,
            blocks: checked_add(
                self.blocks,
                other.blocks,
                ResourceField::Blocks,
            )?,
            values: checked_add(
                self.values,
                other.values,
                ResourceField::Values,
            )?,
            symbols: checked_add(
                self.symbols,
                other.symbols,
                ResourceField::Symbols,
            )?,
            dependencies: checked_add(
                self.dependencies,
                other.dependencies,
                ResourceField::Dependencies,
            )?,
            control_flow_depth: self
                .control_flow_depth
                .max(other.control_flow_depth),
            nesting_depth: self.nesting_depth.max(other.nesting_depth),
            circuit_depth: self.circuit_depth.max(other.circuit_depth),
            measurements: checked_add(
                self.measurements,
                other.measurements,
                ResourceField::Measurements,
            )?,
            barriers: checked_add(
                self.barriers,
                other.barriers,
                ResourceField::Barriers,
            )?,
            pulse_operations: checked_add(
                self.pulse_operations,
                other.pulse_operations,
                ResourceField::PulseOperations,
            )?,
            waveform_samples: checked_add(
                self.waveform_samples,
                other.waveform_samples,
                ResourceField::WaveformSamples,
            )?,
            waveform_bytes: checked_add(
                self.waveform_bytes,
                other.waveform_bytes,
                ResourceField::WaveformBytes,
            )?,
            channels: checked_add(
                self.channels,
                other.channels,
                ResourceField::Channels,
            )?,
            frames: checked_add(
                self.frames,
                other.frames,
                ResourceField::Frames,
            )?,
            scheduled_operations: checked_add(
                self.scheduled_operations,
                other.scheduled_operations,
                ResourceField::ScheduledOperations,
            )?,
            mapping_entries: checked_add(
                self.mapping_entries,
                other.mapping_entries,
                ResourceField::MappingEntries,
            )?,
            resource_requirements: checked_add(
                self.resource_requirements,
                other.resource_requirements,
                ResourceField::ResourceRequirements,
            )?,
            extensions: checked_add(
                self.extensions,
                other.extensions,
                ResourceField::Extensions,
            )?,
            diagnostics: checked_add(
                self.diagnostics,
                other.diagnostics,
                ResourceField::Diagnostics,
            )?,
            metadata_bytes: checked_add(
                self.metadata_bytes,
                other.metadata_bytes,
                ResourceField::MetadataBytes,
            )?,
            source_bytes: checked_add(
                self.source_bytes,
                other.source_bytes,
                ResourceField::SourceBytes,
            )?,
            program_bytes: checked_add(
                self.program_bytes,
                other.program_bytes,
                ResourceField::ProgramBytes,
            )?,
            validation_steps: checked_add(
                self.validation_steps,
                other.validation_steps,
                ResourceField::ValidationSteps,
            )?,
            analysis_steps: checked_add(
                self.analysis_steps,
                other.analysis_steps,
                ResourceField::AnalysisSteps,
            )?,
            transformation_steps: checked_add(
                self.transformation_steps,
                other.transformation_steps,
                ResourceField::TransformationSteps,
            )?,
            compilation_steps: checked_add(
                self.compilation_steps,
                other.compilation_steps,
                ResourceField::CompilationSteps,
            )?,
            processing_depth: self.processing_depth.max(other.processing_depth),
        })
    }
}

// =============================================================================
// Resource usage builder
// =============================================================================

/// Checked incremental resource-usage accumulator.
///
/// The builder exists so resource extraction code can accumulate usage without
/// ever allowing integer overflow to wrap into a smaller value.
///
/// It does not apply `QuantumIrLimits`; use [`validate_usage`] after building
/// the snapshot.
///
/// This separation is intentional:
///
/// ```text
/// accumulation correctness
///         !=
/// policy validation
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ResourceUsageBuilder {
    usage: ResourceUsage,
}

impl ResourceUsageBuilder {
    /// Creates an empty builder.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            usage: ResourceUsage::new(),
        }
    }

    /// Returns the current snapshot.
    #[must_use]
    pub const fn snapshot(&self) -> ResourceUsage {
        self.usage
    }

    /// Adds logical-qubit usage.
    pub fn add_logical_qubits(&mut self, amount: usize) -> Result<(), ResourceUsageError> {
        self.usage.logical_qubits = checked_add(
            self.usage.logical_qubits,
            amount,
            ResourceField::LogicalQubits,
        )?;
        Ok(())
    }

    /// Adds classical-bit usage.
    pub fn add_classical_bits(&mut self, amount: usize) -> Result<(), ResourceUsageError> {
        self.usage.classical_bits = checked_add(
            self.usage.classical_bits,
            amount,
            ResourceField::ClassicalBits,
        )?;
        Ok(())
    }

    /// Adds register usage.
    pub fn add_registers(&mut self, amount: usize) -> Result<(), ResourceUsageError> {
        self.usage.registers =
            checked_add(self.usage.registers, amount, ResourceField::Registers)?;
        Ok(())
    }

    /// Adds operation usage.
    pub fn add_operations(&mut self, amount: usize) -> Result<(), ResourceUsageError> {
        self.usage.operations =
            checked_add(self.usage.operations, amount, ResourceField::Operations)?;
        Ok(())
    }

    /// Records an operation operand count.
    ///
    /// This is a maximum, not a cumulative total.
    pub fn observe_operands_per_operation(
        &mut self,
        amount: usize,
    ) {
        self.usage.operands_per_operation =
            self.usage.operands_per_operation.max(amount);
    }

    /// Records an operation parameter count.
    ///
    /// This is a maximum, not a cumulative total.
    pub fn observe_parameters_per_operation(
        &mut self,
        amount: usize,
    ) {
        self.usage.parameters_per_operation =
            self.usage.parameters_per_operation.max(amount);
    }

    /// Adds parameter usage.
    pub fn add_parameters(&mut self, amount: usize) -> Result<(), ResourceUsageError> {
        self.usage.parameters =
            checked_add(self.usage.parameters, amount, ResourceField::Parameters)?;
        Ok(())
    }

    /// Adds symbolic-expression nodes.
    pub fn add_expression_nodes(
        &mut self,
        amount: usize,
    ) -> Result<(), ResourceUsageError> {
        self.usage.expression_nodes = checked_add(
            self.usage.expression_nodes,
            amount,
            ResourceField::ExpressionNodes,
        )?;
        Ok(())
    }

    /// Adds region usage.
    pub fn add_regions(&mut self, amount: usize) -> Result<(), ResourceUsageError> {
        self.usage.regions =
            checked_add(self.usage.regions, amount, ResourceField::Regions)?;
        Ok(())
    }

    /// Adds block usage.
    pub fn add_blocks(&mut self, amount: usize) -> Result<(), ResourceUsageError> {
        self.usage.blocks =
            checked_add(self.usage.blocks, amount, ResourceField::Blocks)?;
        Ok(())
    }

    /// Adds value usage.
    pub fn add_values(&mut self, amount: usize) -> Result<(), ResourceUsageError> {
        self.usage.values =
            checked_add(self.usage.values, amount, ResourceField::Values)?;
        Ok(())
    }

    /// Adds symbol usage.
    pub fn add_symbols(&mut self, amount: usize) -> Result<(), ResourceUsageError> {
        self.usage.symbols =
            checked_add(self.usage.symbols, amount, ResourceField::Symbols)?;
        Ok(())
    }

    /// Adds dependency usage.
    pub fn add_dependencies(
        &mut self,
        amount: usize,
    ) -> Result<(), ResourceUsageError> {
        self.usage.dependencies = checked_add(
            self.usage.dependencies,
            amount,
            ResourceField::Dependencies,
        )?;
        Ok(())
    }

    /// Records control-flow depth.
    pub fn observe_control_flow_depth(&mut self, depth: usize) {
        self.usage.control_flow_depth =
            self.usage.control_flow_depth.max(depth);
    }

    /// Records general IR nesting depth.
    pub fn observe_nesting_depth(&mut self, depth: usize) {
        self.usage.nesting_depth =
            self.usage.nesting_depth.max(depth);
    }

    /// Records circuit depth.
    pub fn observe_circuit_depth(&mut self, depth: usize) {
        self.usage.circuit_depth =
            self.usage.circuit_depth.max(depth);
    }

    /// Adds measurement usage.
    pub fn add_measurements(
        &mut self,
        amount: usize,
    ) -> Result<(), ResourceUsageError> {
        self.usage.measurements = checked_add(
            self.usage.measurements,
            amount,
            ResourceField::Measurements,
        )?;
        Ok(())
    }

    /// Adds barrier usage.
    pub fn add_barriers(
        &mut self,
        amount: usize,
    ) -> Result<(), ResourceUsageError> {
        self.usage.barriers =
            checked_add(self.usage.barriers, amount, ResourceField::Barriers)?;
        Ok(())
    }

    /// Adds pulse-operation usage.
    pub fn add_pulse_operations(
        &mut self,
        amount: usize,
    ) -> Result<(), ResourceUsageError> {
        self.usage.pulse_operations = checked_add(
            self.usage.pulse_operations,
            amount,
            ResourceField::PulseOperations,
        )?;
        Ok(())
    }

    /// Adds waveform samples.
    pub fn add_waveform_samples(
        &mut self,
        amount: usize,
    ) -> Result<(), ResourceUsageError> {
        self.usage.waveform_samples = checked_add(
            self.usage.waveform_samples,
            amount,
            ResourceField::WaveformSamples,
        )?;
        Ok(())
    }

    /// Adds waveform bytes.
    pub fn add_waveform_bytes(
        &mut self,
        amount: usize,
    ) -> Result<(), ResourceUsageError> {
        self.usage.waveform_bytes = checked_add(
            self.usage.waveform_bytes,
            amount,
            ResourceField::WaveformBytes,
        )?;
        Ok(())
    }

    /// Adds channel usage.
    pub fn add_channels(&mut self, amount: usize) -> Result<(), ResourceUsageError> {
        self.usage.channels =
            checked_add(self.usage.channels, amount, ResourceField::Channels)?;
        Ok(())
    }

    /// Adds frame usage.
    pub fn add_frames(&mut self, amount: usize) -> Result<(), ResourceUsageError> {
        self.usage.frames =
            checked_add(self.usage.frames, amount, ResourceField::Frames)?;
        Ok(())
    }

    /// Adds scheduled-operation usage.
    pub fn add_scheduled_operations(
        &mut self,
        amount: usize,
    ) -> Result<(), ResourceUsageError> {
        self.usage.scheduled_operations = checked_add(
            self.usage.scheduled_operations,
            amount,
            ResourceField::ScheduledOperations,
        )?;
        Ok(())
    }

    /// Adds mapping-entry usage.
    pub fn add_mapping_entries(
        &mut self,
        amount: usize,
    ) -> Result<(), ResourceUsageError> {
        self.usage.mapping_entries = checked_add(
            self.usage.mapping_entries,
            amount,
            ResourceField::MappingEntries,
        )?;
        Ok(())
    }

    /// Adds resource-requirement usage.
    pub fn add_resource_requirements(
        &mut self,
        amount: usize,
    ) -> Result<(), ResourceUsageError> {
        self.usage.resource_requirements = checked_add(
            self.usage.resource_requirements,
            amount,
            ResourceField::ResourceRequirements,
        )?;
        Ok(())
    }

    /// Adds extension usage.
    pub fn add_extensions(
        &mut self,
        amount: usize,
    ) -> Result<(), ResourceUsageError> {
        self.usage.extensions =
            checked_add(self.usage.extensions, amount, ResourceField::Extensions)?;
        Ok(())
    }

    /// Adds diagnostic usage.
    pub fn add_diagnostics(
        &mut self,
        amount: usize,
    ) -> Result<(), ResourceUsageError> {
        self.usage.diagnostics =
            checked_add(self.usage.diagnostics, amount, ResourceField::Diagnostics)?;
        Ok(())
    }

    /// Adds metadata bytes.
    pub fn add_metadata_bytes(
        &mut self,
        amount: usize,
    ) -> Result<(), ResourceUsageError> {
        self.usage.metadata_bytes = checked_add(
            self.usage.metadata_bytes,
            amount,
            ResourceField::MetadataBytes,
        )?;
        Ok(())
    }

    /// Adds source bytes.
    pub fn add_source_bytes(
        &mut self,
        amount: usize,
    ) -> Result<(), ResourceUsageError> {
        self.usage.source_bytes = checked_add(
            self.usage.source_bytes,
            amount,
            ResourceField::SourceBytes,
        )?;
        Ok(())
    }

    /// Adds serialized-program bytes.
    pub fn add_program_bytes(
        &mut self,
        amount: usize,
    ) -> Result<(), ResourceUsageError> {
        self.usage.program_bytes = checked_add(
            self.usage.program_bytes,
            amount,
            ResourceField::ProgramBytes,
        )?;
        Ok(())
    }

    /// Adds validation work.
    pub fn add_validation_steps(
        &mut self,
        amount: usize,
    ) -> Result<(), ResourceUsageError> {
        self.usage.validation_steps = checked_add(
            self.usage.validation_steps,
            amount,
            ResourceField::ValidationSteps,
        )?;
        Ok(())
    }

    /// Adds analysis work.
    pub fn add_analysis_steps(
        &mut self,
        amount: usize,
    ) -> Result<(), ResourceUsageError> {
        self.usage.analysis_steps = checked_add(
            self.usage.analysis_steps,
            amount,
            ResourceField::AnalysisSteps,
        )?;
        Ok(())
    }

    /// Adds transformation work.
    pub fn add_transformation_steps(
        &mut self,
        amount: usize,
    ) -> Result<(), ResourceUsageError> {
        self.usage.transformation_steps = checked_add(
            self.usage.transformation_steps,
            amount,
            ResourceField::TransformationSteps,
        )?;
        Ok(())
    }

    /// Adds compilation work.
    pub fn add_compilation_steps(
        &mut self,
        amount: usize,
    ) -> Result<(), ResourceUsageError> {
        self.usage.compilation_steps = checked_add(
            self.usage.compilation_steps,
            amount,
            ResourceField::CompilationSteps,
        )?;
        Ok(())
    }

    /// Records processing depth.
    pub fn observe_processing_depth(&mut self, depth: usize) {
        self.usage.processing_depth =
            self.usage.processing_depth.max(depth);
    }

    /// Finishes the builder and returns its immutable snapshot.
    #[must_use]
    pub const fn finish(self) -> ResourceUsage {
        self.usage
    }
}

// =============================================================================
// Resource-field accounting errors
// =============================================================================

/// Resource-accounting failure occurring before policy validation.
///
/// This is intentionally separate from `QuantumIrLimits` policy errors:
///
/// ```text
/// ResourceUsageError
///     = accounting could not be represented safely
///
/// LimitsError
///     = accounting was representable but policy rejected it
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceUsageError {
    /// An accumulation would overflow `usize`.
    ArithmeticOverflow {
        /// Resource field whose accumulation overflowed.
        field: ResourceField,
    },
}

impl fmt::Display for ResourceUsageError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ArithmeticOverflow { field } => write!(
                formatter,
                "resource usage arithmetic overflow for `{field}`"
            ),
        }
    }
}

impl Error for ResourceUsageError {}

/// Resource field used by checked usage accounting.
///
/// This type is deliberately local to accounting. It is not a replacement for
/// `quantum::ir::limits::ResourceKind`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResourceField {
    /// Logical qubits.
    LogicalQubits,

    /// Classical bits.
    ClassicalBits,

    /// Registers.
    Registers,

    /// Operations.
    Operations,

    /// Parameters.
    Parameters,

    /// Expression nodes.
    ExpressionNodes,

    /// Regions.
    Regions,

    /// Blocks.
    Blocks,

    /// Values.
    Values,

    /// Symbols.
    Symbols,

    /// Dependencies.
    Dependencies,

    /// Measurements.
    Measurements,

    /// Barriers.
    Barriers,

    /// Pulse operations.
    PulseOperations,

    /// Waveform samples.
    WaveformSamples,

    /// Waveform bytes.
    WaveformBytes,

    /// Channels.
    Channels,

    /// Frames.
    Frames,

    /// Scheduled operations.
    ScheduledOperations,

    /// Mapping entries.
    MappingEntries,

    /// Resource requirements.
    ResourceRequirements,

    /// Extensions.
    Extensions,

    /// Diagnostics.
    Diagnostics,

    /// Metadata bytes.
    MetadataBytes,

    /// Source bytes.
    SourceBytes,

    /// Program bytes.
    ProgramBytes,

    /// Validation steps.
    ValidationSteps,

    /// Analysis steps.
    AnalysisSteps,

    /// Transformation steps.
    TransformationSteps,

    /// Compilation steps.
    CompilationSteps,
}

impl fmt::Display for ResourceField {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::LogicalQubits => "logical_qubits",
            Self::ClassicalBits => "classical_bits",
            Self::Registers => "registers",
            Self::Operations => "operations",
            Self::Parameters => "parameters",
            Self::ExpressionNodes => "expression_nodes",
            Self::Regions => "regions",
            Self::Blocks => "blocks",
            Self::Values => "values",
            Self::Symbols => "symbols",
            Self::Dependencies => "dependencies",
            Self::Measurements => "measurements",
            Self::Barriers => "barriers",
            Self::PulseOperations => "pulse_operations",
            Self::WaveformSamples => "waveform_samples",
            Self::WaveformBytes => "waveform_bytes",
            Self::Channels => "channels",
            Self::Frames => "frames",
            Self::ScheduledOperations => "scheduled_operations",
            Self::MappingEntries => "mapping_entries",
            Self::ResourceRequirements => "resource_requirements",
            Self::Extensions => "extensions",
            Self::Diagnostics => "diagnostics",
            Self::MetadataBytes => "metadata_bytes",
            Self::SourceBytes => "source_bytes",
            Self::ProgramBytes => "program_bytes",
            Self::ValidationSteps => "validation_steps",
            Self::AnalysisSteps => "analysis_steps",
            Self::TransformationSteps => "transformation_steps",
            Self::CompilationSteps => "compilation_steps",
        };

        formatter.write_str(name)
    }
}

fn checked_add(
    lhs: usize,
    rhs: usize,
    field: ResourceField,
) -> Result<usize, ResourceUsageError> {
    lhs.checked_add(rhs)
        .ok_or(ResourceUsageError::ArithmeticOverflow { field })
}

// =============================================================================
// Policy validation
// =============================================================================

/// Validates a complete resource-usage snapshot against the supplied policy.
///
/// This is the canonical resource-policy validation entry point.
///
/// The function:
///
/// 1. validates the policy itself;
/// 2. validates cumulative resource counts;
/// 3. validates maximum-per-operation resources;
/// 4. validates depth resources.
///
/// No IR is modified.
///
/// No hardware is inspected.
///
/// No target is selected.
pub fn validate_usage(
    usage: &ResourceUsage,
    limits: &QuantumIrLimits,
) -> IrResult<()> {
    limits.validate().map_err(IrError::from)?;

    validate_count(
        limits,
        usage.logical_qubits,
        ResourceCheck::LogicalQubits,
    )?;

    validate_count(
        limits,
        usage.classical_bits,
        ResourceCheck::ClassicalBits,
    )?;

    validate_count(
        limits,
        usage.registers,
        ResourceCheck::Registers,
    )?;

    validate_count(
        limits,
        usage.operations,
        ResourceCheck::Operations,
    )?;

    validate_count(
        limits,
        usage.operands_per_operation,
        ResourceCheck::OperandsPerOperation,
    )?;

    validate_count(
        limits,
        usage.parameters_per_operation,
        ResourceCheck::ParametersPerOperation,
    )?;

    validate_count(
        limits,
        usage.parameters,
        ResourceCheck::Parameters,
    )?;

    validate_count(
        limits,
        usage.expression_nodes,
        ResourceCheck::ExpressionNodes,
    )?;

    validate_count(
        limits,
        usage.regions,
        ResourceCheck::Regions,
    )?;

    validate_count(
        limits,
        usage.blocks,
        ResourceCheck::Blocks,
    )?;

    validate_count(
        limits,
        usage.values,
        ResourceCheck::Values,
    )?;

    validate_count(
        limits,
        usage.symbols,
        ResourceCheck::Symbols,
    )?;

    validate_count(
        limits,
        usage.dependencies,
        ResourceCheck::Dependencies,
    )?;

    validate_count(
        limits,
        usage.control_flow_depth,
        ResourceCheck::ControlFlowDepth,
    )?;

    validate_count(
        limits,
        usage.nesting_depth,
        ResourceCheck::NestingDepth,
    )?;

    validate_count(
        limits,
        usage.circuit_depth,
        ResourceCheck::CircuitDepth,
    )?;

    validate_count(
        limits,
        usage.measurements,
        ResourceCheck::Measurements,
    )?;

    validate_count(
        limits,
        usage.barriers,
        ResourceCheck::Barriers,
    )?;

    validate_count(
        limits,
        usage.pulse_operations,
        ResourceCheck::PulseOperations,
    )?;

    validate_count(
        limits,
        usage.waveform_samples,
        ResourceCheck::WaveformSamples,
    )?;

    validate_count(
        limits,
        usage.waveform_bytes,
        ResourceCheck::WaveformBytes,
    )?;

    validate_count(
        limits,
        usage.channels,
        ResourceCheck::Channels,
    )?;

    validate_count(
        limits,
        usage.frames,
        ResourceCheck::Frames,
    )?;

    validate_count(
        limits,
        usage.scheduled_operations,
        ResourceCheck::ScheduledOperations,
    )?;

    validate_count(
        limits,
        usage.mapping_entries,
        ResourceCheck::MappingEntries,
    )?;

    validate_count(
        limits,
        usage.resource_requirements,
        ResourceCheck::ResourceRequirements,
    )?;

    validate_count(
        limits,
        usage.extensions,
        ResourceCheck::Extensions,
    )?;

    validate_count(
        limits,
        usage.diagnostics,
        ResourceCheck::Diagnostics,
    )?;

    validate_count(
        limits,
        usage.metadata_bytes,
        ResourceCheck::MetadataBytes,
    )?;

    validate_count(
        limits,
        usage.source_bytes,
        ResourceCheck::SourceBytes,
    )?;

    validate_count(
        limits,
        usage.program_bytes,
        ResourceCheck::ProgramBytes,
    )?;

    validate_count(
        limits,
        usage.validation_steps,
        ResourceCheck::ValidationSteps,
    )?;

    validate_count(
        limits,
        usage.analysis_steps,
        ResourceCheck::AnalysisSteps,
    )?;

    validate_count(
        limits,
        usage.transformation_steps,
        ResourceCheck::TransformationSteps,
    )?;

    validate_count(
        limits,
        usage.compilation_steps,
        ResourceCheck::CompilationSteps,
    )?;

    validate_count(
        limits,
        usage.processing_depth,
        ResourceCheck::ProcessingDepth,
    )?;

    Ok(())
}

// =============================================================================
// Resource check dispatch
// =============================================================================

/// Resource category understood by the resource-validation adapter.
///
/// This is deliberately private: callers should use `validate_usage` rather
/// than depending on the limits implementation's internal resource taxonomy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ResourceCheck {
    LogicalQubits,
    ClassicalBits,
    Registers,
    Operations,
    OperandsPerOperation,
    ParametersPerOperation,
    Parameters,
    ExpressionNodes,
    Regions,
    Blocks,
    Values,
    Symbols,
    Dependencies,
    ControlFlowDepth,
    NestingDepth,
    CircuitDepth,
    Measurements,
    Barriers,
    PulseOperations,
    WaveformSamples,
    WaveformBytes,
    Channels,
    Frames,
    ScheduledOperations,
    MappingEntries,
    ResourceRequirements,
    Extensions,
    Diagnostics,
    MetadataBytes,
    SourceBytes,
    ProgramBytes,
    ValidationSteps,
    AnalysisSteps,
    TransformationSteps,
    CompilationSteps,
    ProcessingDepth,
}

/// Validates one count against the corresponding canonical IR limit.
///
/// This adapter is the only place where this module couples its usage fields
/// to the `QuantumIrLimits` API.
///
/// Keeping that mapping centralized means future changes to the limits API
/// require one integration point rather than changes throughout the resource
/// accumulator.
fn validate_count(
    limits: &QuantumIrLimits,
    value: usize,
    resource: ResourceCheck,
) -> IrResult<()> {
    let result = match resource {
        ResourceCheck::LogicalQubits => limits.check_qubits(value),
        ResourceCheck::ClassicalBits => limits.check_classical_bits(value),
        ResourceCheck::Registers => limits.check_registers(value),
        ResourceCheck::Operations => limits.check_operations(value),
        ResourceCheck::OperandsPerOperation => limits.check_operands(value),
        ResourceCheck::ParametersPerOperation => {
            limits.check_parameters_per_operation(value)
        }
        ResourceCheck::Parameters => limits.check_parameters(value),
        ResourceCheck::ExpressionNodes => {
            limits.check_expression_nodes(value)
        }
        ResourceCheck::Regions => limits.check_regions(value),
        ResourceCheck::Blocks => limits.check_blocks(value),
        ResourceCheck::Values => limits.check_values(value),
        ResourceCheck::Symbols => limits.check_symbols(value),
        ResourceCheck::Dependencies => limits.check_dependencies(value),
        ResourceCheck::ControlFlowDepth => {
            limits.check_control_flow_depth(value)
        }
        ResourceCheck::NestingDepth => limits.check_nesting_depth(value),
        ResourceCheck::CircuitDepth => limits.check_circuit_depth(value),
        ResourceCheck::Measurements => limits.check_measurements(value),
        ResourceCheck::Barriers => limits.check_barriers(value),
        ResourceCheck::PulseOperations => {
            limits.check_pulse_operations(value)
        }
        ResourceCheck::WaveformSamples => {
            limits.check_waveform_samples(value)
        }
        ResourceCheck::WaveformBytes => {
            limits.check_waveform_bytes(value)
        }
        ResourceCheck::Channels => limits.check_channels(value),
        ResourceCheck::Frames => limits.check_frames(value),
        ResourceCheck::ScheduledOperations => {
            limits.check_scheduled_operations(value)
        }
        ResourceCheck::MappingEntries => {
            limits.check_mapping_entries(value)
        }
        ResourceCheck::ResourceRequirements => {
            limits.check_resource_requirements(value)
        }
        ResourceCheck::Extensions => limits.check_extensions(value),
        ResourceCheck::Diagnostics => limits.check_diagnostics(value),
        ResourceCheck::MetadataBytes => limits.check_metadata_bytes(value),
        ResourceCheck::SourceBytes => limits.check_source_bytes(value),
        ResourceCheck::ProgramBytes => limits.check_program_bytes(value),
        ResourceCheck::ValidationSteps => {
            limits.check_validation_steps(value)
        }
        ResourceCheck::AnalysisSteps => limits.check_analysis_steps(value),
        ResourceCheck::TransformationSteps => {
            limits.check_transformation_steps(value)
        }
        ResourceCheck::CompilationSteps => {
            limits.check_compilation_steps(value)
        }
        ResourceCheck::ProcessingDepth => limits.check_processing_depth(value),
    };

    result.map_err(IrError::from)
}

// =============================================================================
// Sparse logical-qubit validation
// =============================================================================

/// Validates logical qubit identities against an enclosing logical namespace
/// and resource policy.
///
/// The iterator is consumed exactly once.
///
/// No collection proportional to the largest `QubitId` is allocated.
///
/// # Arguments
///
/// `qubits`
///     Logical qubit identities actually referenced by the object.
///
/// `declared_qubits`
///     Number of logical qubits declared by the enclosing namespace.
///
/// `limits`
///     Explicit resource policy.
///
/// # Important
///
/// The function intentionally does not assume that logical identifiers are
/// dense.
///
/// For example, if the enclosing namespace contains a valid logical identity
/// `q900_000_000`, this function does not allocate a vector of 900,000,001
/// elements.
///
/// The identity is checked directly against the declared namespace.
///
/// # Duplicate identities
///
/// Duplicate logical identities are rejected because this function is intended
/// for validating a logical-resource identity collection, not an operation
/// operand list where duplicate semantics may have a separate rule.
pub fn validate_qubit_ids<I>(
    qubits: I,
    declared_qubits: usize,
    limits: &QuantumIrLimits,
) -> IrResult<()>
where
    I: IntoIterator<Item = QubitId>,
{
    limits.validate().map_err(IrError::from)?;

    limits
        .check_qubits(declared_qubits)
        .map_err(IrError::from)?;

    let mut seen = BTreeSet::<QubitId>::new();

    for qubit in qubits {
        let index = qubit.index();

        if index >= declared_qubits {
            return Err(IrError::Identifier(
                super::super::errors::IrIdentifierError::QubitOutOfRange {
                    index,
                    count: declared_qubits,
                },
            ));
        }

        if !seen.insert(qubit) {
            return Err(IrError::Qubit(
                super::super::errors::IrQubitError::Duplicate {
                    qubit: index,
                },
            ));
        }
    }

    Ok(())
}

/// Validates a sparse logical-qubit set.
///
/// This convenience API avoids forcing callers to create an intermediate
/// `BTreeSet`.
pub fn validate_qubit_set(
    qubits: &BTreeSet<QubitId>,
    declared_qubits: usize,
    limits: &QuantumIrLimits,
) -> IrResult<()> {
    validate_qubit_ids(
        qubits.iter().copied(),
        declared_qubits,
        limits,
    )
}

// =============================================================================
// Combined resource validation
// =============================================================================

/// Validates both a resource-usage snapshot and the sparse logical-qubit
/// identity collection.
///
/// This is useful at a trust boundary where both:
///
/// - resource counts; and
/// - actual logical identifiers
///
/// must be checked.
///
/// The resource count and declared namespace must agree at the caller's
/// semantic boundary.
///
/// This function does not infer declarations from the iterator because doing
/// so would make the meaning of the namespace ambiguous.
pub fn validate_usage_with_qubits<I>(
    usage: &ResourceUsage,
    qubits: I,
    limits: &QuantumIrLimits,
) -> IrResult<()>
where
    I: IntoIterator<Item = QubitId>,
{
    validate_usage(usage, limits)?;

    validate_qubit_ids(
        qubits,
        usage.logical_qubits,
        limits,
    )
}

// =============================================================================
// Explicit namespace/count consistency
// =============================================================================

/// Validates that an observed logical-qubit count agrees with a declared
/// namespace count.
///
/// This function exists to make namespace/resource consistency explicit rather
/// than relying on implicit assumptions in callers.
///
/// The two values must be equal.
///
/// A resource policy may permit a larger namespace, but that does not make a
/// declaration of one size semantically identical to an object reporting
/// another size.
pub fn validate_logical_qubit_count(
    declared_qubits: usize,
    observed_qubits: usize,
    limits: &QuantumIrLimits,
) -> IrResult<()> {
    limits
        .check_qubits(declared_qubits)
        .map_err(IrError::from)?;

    limits
        .check_qubits(observed_qubits)
        .map_err(IrError::from)?;

    if declared_qubits != observed_qubits {
        return Err(IrError::InvalidStructure {
            message: "logical-qubit declaration and resource usage disagree",
        });
    }

    Ok(())
}

// =============================================================================
// Incremental policy-safe accumulation
// =============================================================================

/// Adds two usage snapshots and immediately validates the combined result.
///
/// This is useful when processing independent IR regions incrementally.
///
/// It prevents a caller from accidentally accumulating an overflowing
/// `usize` value before validation occurs.
pub fn checked_add_and_validate(
    lhs: ResourceUsage,
    rhs: ResourceUsage,
    limits: &QuantumIrLimits,
) -> IrResult<ResourceUsage> {
    let combined = lhs.checked_add(rhs).map_err(|error| {
        IrError::InvalidStructure {
            message: match error {
                ResourceUsageError::ArithmeticOverflow { .. } => {
                    "resource usage arithmetic overflow"
                }
            },
        }
    })?;

    validate_usage(&combined, limits)?;

    Ok(combined)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn permissive_limits() -> QuantumIrLimits {
        QuantumIrLimits::unbounded()
    }

    #[test]
    fn empty_usage_is_empty() {
        let usage = ResourceUsage::new();

        assert!(usage.is_empty());
    }

    #[test]
    fn builder_accumulates_counts() {
        let mut builder = ResourceUsage::builder();

        builder
            .add_logical_qubits(3)
            .expect("logical qubit accounting should succeed");

        builder
            .add_operations(10)
            .expect("operation accounting should succeed");

        builder
            .add_measurements(2)
            .expect("measurement accounting should succeed");

        let usage = builder.finish();

        assert_eq!(usage.logical_qubits, 3);
        assert_eq!(usage.operations, 10);
        assert_eq!(usage.measurements, 2);
    }

    #[test]
    fn builder_tracks_operation_maxima() {
        let mut builder = ResourceUsage::builder();

        builder.observe_operands_per_operation(2);
        builder.observe_operands_per_operation(9);
        builder.observe_operands_per_operation(4);

        builder.observe_parameters_per_operation(1);
        builder.observe_parameters_per_operation(7);
        builder.observe_parameters_per_operation(3);

        let usage = builder.finish();

        assert_eq!(usage.operands_per_operation, 9);
        assert_eq!(usage.parameters_per_operation, 7);
    }

    #[test]
    fn builder_tracks_depth_as_maximum() {
        let mut builder = ResourceUsage::builder();

        builder.observe_control_flow_depth(2);
        builder.observe_control_flow_depth(11);
        builder.observe_control_flow_depth(4);

        builder.observe_nesting_depth(5);
        builder.observe_nesting_depth(3);

        let usage = builder.finish();

        assert_eq!(usage.control_flow_depth, 11);
        assert_eq!(usage.nesting_depth, 5);
    }

    #[test]
    fn usage_checked_add_is_overflow_safe() {
        let lhs = ResourceUsage {
            operations: usize::MAX,
            ..ResourceUsage::new()
        };

        let rhs = ResourceUsage {
            operations: 1,
            ..ResourceUsage::new()
        };

        let result = lhs.checked_add(rhs);

        assert!(matches!(
            result,
            Err(ResourceUsageError::ArithmeticOverflow {
                field: ResourceField::Operations
            })
        ));
    }

    #[test]
    fn usage_checked_add_combines_cumulative_values() {
        let lhs = ResourceUsage {
            operations: 10,
            measurements: 2,
            ..ResourceUsage::new()
        };

        let rhs = ResourceUsage {
            operations: 7,
            measurements: 5,
            ..ResourceUsage::new()
        };

        let result = lhs
            .checked_add(rhs)
            .expect("checked addition should succeed");

        assert_eq!(result.operations, 17);
        assert_eq!(result.measurements, 7);
    }

    #[test]
    fn usage_checked_add_uses_max_for_depth() {
        let lhs = ResourceUsage {
            circuit_depth: 20,
            processing_depth: 8,
            ..ResourceUsage::new()
        };

        let rhs = ResourceUsage {
            circuit_depth: 12,
            processing_depth: 19,
            ..ResourceUsage::new()
        };

        let result = lhs
            .checked_add(rhs)
            .expect("checked addition should succeed");

        assert_eq!(result.circuit_depth, 20);
        assert_eq!(result.processing_depth, 19);
    }

    #[test]
    fn empty_usage_passes_unbounded_policy() {
        let usage = ResourceUsage::new();

        validate_usage(&usage, &permissive_limits())
            .expect("empty usage should satisfy unbounded policy");
    }

    #[test]
    fn sparse_qubit_validation_does_not_require_dense_storage() {
        let limits = permissive_limits();

        let qubits = [
            QubitId::new(0),
            QubitId::new(1_000_000),
            QubitId::new(2_000_000),
        ];

        validate_qubit_ids(
            qubits,
            3_000_000,
            &limits,
        )
        .expect("sparse logical identifiers should be valid");
    }

    #[test]
    fn out_of_range_qubit_is_rejected() {
        let limits = permissive_limits();

        let qubits = [QubitId::new(4)];

        let result = validate_qubit_ids(
            qubits,
            4,
            &limits,
        );

        assert!(result.is_err());
    }

    #[test]
    fn duplicate_qubit_is_rejected() {
        let limits = permissive_limits();

        let qubits = [
            QubitId::new(2),
            QubitId::new(2),
        ];

        let result = validate_qubit_ids(
            qubits,
            4,
            &limits,
        );

        assert!(result.is_err());
    }

    #[test]
    fn logical_qubit_count_consistency_is_checked() {
        let limits = permissive_limits();

        validate_logical_qubit_count(
            10,
            10,
            &limits,
        )
        .expect("equal counts should pass");

        let result = validate_logical_qubit_count(
            10,
            9,
            &limits,
        );

        assert!(result.is_err());
    }

    #[test]
    fn combined_usage_and_qubit_validation_works() {
        let limits = permissive_limits();

        let usage = ResourceUsage {
            logical_qubits: 4,
            operations: 2,
            ..ResourceUsage::new()
        };

        let qubits = [
            QubitId::new(0),
            QubitId::new(1),
            QubitId::new(2),
            QubitId::new(3),
        ];

        validate_usage_with_qubits(
            &usage,
            qubits,
            &limits,
        )
        .expect("valid usage and qubits should pass");
    }

    #[test]
    fn combined_usage_and_qubit_validation_rejects_namespace_mismatch() {
        let limits = permissive_limits();

        let usage = ResourceUsage {
            logical_qubits: 2,
            ..ResourceUsage::new()
        };

        let qubits = [
            QubitId::new(0),
            QubitId::new(1),
            QubitId::new(2),
        ];

        let result = validate_usage_with_qubits(
            &usage,
            qubits,
            &limits,
        );

        assert!(result.is_err());
    }

    #[test]
    fn checked_add_and_validate_returns_combined_usage() {
        let limits = permissive_limits();

        let lhs = ResourceUsage {
            operations: 3,
            ..ResourceUsage::new()
        };

        let rhs = ResourceUsage {
            operations: 5,
            ..ResourceUsage::new()
        };

        let combined = checked_add_and_validate(
            lhs,
            rhs,
            &limits,
        )
        .expect("combined usage should validate");

        assert_eq!(combined.operations, 8);
    }
}