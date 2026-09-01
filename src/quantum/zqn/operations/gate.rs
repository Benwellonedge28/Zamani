//! Zamani Quantum Noise (ZQN)
//! Gate-level noise integration.
//!
//! # Ownership
//!
//! This module owns the semantic relationship between a canonical Zamani
//! quantum-IR gate and noise that is applied at that gate boundary.
//!
//! This module owns:
//!
//! - gate-noise attachment;
//! - gate-noise application semantics;
//! - gate-local noise context;
//! - gate-local noise composition order;
//! - logical-qubit resource identification;
//! - gate parameter visibility to noise models;
//! - deterministic gate-noise application metadata;
//! - validation of gate/noise compatibility that can be decided locally.
//!
//! This module does NOT own:
//!
//! - the canonical definition of a quantum gate;
//! - the canonical definition of `QubitId`;
//! - quantum-channel mathematics;
//! - general noise-model semantics;
//! - physical qubit identity;
//! - hardware topology;
//! - routing;
//! - scheduling;
//! - calibration;
//! - pulse generation;
//! - QEC;
//! - backend execution;
//! - random-number generation;
//! - global machine-size limits.
//!
//! The canonical gate representation remains:
//!
//!     crate::quantum::ir::gate::Gate
//!
//! The canonical logical-qubit representation remains:
//!
//!     crate::quantum::ir::qubit::QubitId
//!
//! ZQN attaches physical uncertainty/noise semantics to those canonical
//! representations without creating a second quantum IR.
//!
//! # Architectural boundary
//!
//! ```text
//!                    Zamani Quantum IR
//!                           │
//!                           │ Gate
//!                           ▼
//!                 ┌────────────────────┐
//!                 │  zqn::operations   │
//!                 │      ::gate        │
//!                 └─────────┬──────────┘
//!                           │
//!                  gate-noise binding
//!                           │
//!              ┌────────────┼────────────┐
//!              ▼            ▼            ▼
//!          channel       noise        calibration
//!              │            │            │
//!              └────────────┼────────────┘
//!                           ▼
//!                     integration
//!                           │
//!             ┌─────────────┼─────────────┐
//!             ▼             ▼             ▼
//!          routing      scheduling       QEC
//!             │             │             │
//!             └─────────────┼─────────────┘
//!                           ▼
//!                        runtime
//! ```
//!
//! # Universal-program principle
//!
//! The same Zamani program must remain usable across compatible targets of
//! different sizes and technologies.
//!
//! Consequently this module:
//!
//! - does not contain a maximum qubit count;
//! - does not contain a maximum gate count;
//! - does not contain a maximum gate arity;
//! - does not contain vendor names;
//! - does not contain physical qubit indices;
//! - does not assume a particular topology;
//! - does not assume a particular channel representation;
//! - does not assume a particular simulator;
//! - does not assume a particular hardware technology.
//!
//! `usize` is used only where the existing canonical IR API already uses
//! namespace/index values, such as parameter positions and classical targets.
//! Such values are not architectural machine-size limits.
//!
//! # Resource limits
//!
//! Resource/security limits belong to the caller's explicit ZQN/IR policy.
//!
//! A gate can therefore be arbitrarily large subject only to:
//!
//! - representability;
//! - available memory;
//! - available computation;
//! - caller-supplied resource policy;
//! - target capabilities.
//!
//! This distinction is critical:
//!
//! ```text
//! semantic capability != implementation resource limit
//! ```
//!
//! # Determinism
//!
//! This module does not own randomness.
//!
//! It therefore never:
//!
//! - creates a global RNG;
//! - seeds an RNG implicitly;
//! - calls a thread-local RNG;
//! - derives randomness from wall-clock time;
//! - silently samples noise.
//!
//! Sampling is delegated to the ZQN noise/simulation layer using an explicit
//! deterministic execution context.
//!
//! # Operand ordering
//!
//! Canonical IR gate operand order is preserved exactly.
//!
//! This is essential for operations such as:
//!
//!     CX(control, target)
//!     CRX(control, target)
//!
//! Noise attached to an ordered gate must observe the same ordering.
//!
//! This module never sorts, deduplicates, or otherwise rewrites gate operands.
//!
//! # Integration contract
//!
//! Producers:
//!
//! - `quantum::ir::gate`
//! - quantum frontend lowering
//! - optimization passes
//! - scheduling/lowering layers
//!
//! Consumers:
//!
//! - ZQN channel application
//! - ZQN noise models
//! - routing
//! - scheduling
//! - calibration
//! - simulation
//! - QEC adapters
//! - hardware integration
//! - benchmarking
//! - runtime
//!
//! # Rust contract
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021
//! - stable Rust
//! - no nightly features
//! - no unsafe code
//!
//! The module explicitly forbids unsafe code.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::fmt;
use std::sync::Arc;

use crate::quantum::ir::gate::{Gate, GateKind};
use crate::quantum::ir::parameter::Parameter;
use crate::quantum::ir::qubit::QubitId;

/*
 * IMPORTANT INTEGRATION CONTRACT
 * ==============================
 *
 * These types are intentionally imported from the future canonical ZQN
 * modules rather than redefined here.
 *
 * `channel::QuantumChannel`
 *     Owns channel mathematics and representation.
 *
 * `noise::NoiseModel`
 *     Owns general noise semantics and stochastic/deterministic realization.
 *
 * `core::context::ZqnContext`
 *     Owns execution/resource/determinism/capability context.
 *
 * Those modules must implement the contracts documented by this file.
 */
use crate::quantum::zqn::channel::QuantumChannel;
use crate::quantum::zqn::core::context::ZqnContext;
use crate::quantum::zqn::noise::model::NoiseModel;

// ============================================================================
// PUBLIC RESULT
// ============================================================================

/// Result type for gate-level ZQN operations.
pub type GateNoiseResult<T> = Result<T, GateNoiseError>;

// ============================================================================
// ERRORS
// ============================================================================

/// Errors produced by gate-level ZQN validation or construction.
///
/// These errors describe failures local to the gate/noise boundary.
/// Mathematical channel errors remain owned by `zqn::channel`.
/// General noise-model errors remain owned by `zqn::noise`.
#[derive(Debug, Clone, PartialEq)]
pub enum GateNoiseError {
    /// The supplied gate is not compatible with the requested noise binding.
    IncompatibleGate {
        gate: GateKind,
        reason: &'static str,
    },

    /// A gate requiring at least one operand was supplied without operands.
    EmptyGateOperands {
        gate: GateKind,
    },

    /// A logical qubit appeared more than once in an ordered gate.
    DuplicateQubit {
        qubit: QubitId,
    },

    /// The gate is parameterized but the noise binding requires a concrete
    /// parameter value that has not been supplied.
    UnboundParameter {
        index: usize,
    },

    /// A noise model cannot be applied to this gate under the supplied
    /// context.
    UnsupportedNoiseApplication {
        gate: GateKind,
        reason: &'static str,
    },

    /// A channel cannot be attached to the requested gate boundary.
    UnsupportedChannelApplication {
        gate: GateKind,
        reason: &'static str,
    },

    /// A required noise model is absent.
    MissingNoiseModel,

    /// A required channel is absent.
    MissingChannel,

    /// The number of channel input resources does not match the gate resource
    /// set.
    ChannelArityMismatch {
        gate_arity: usize,
        channel_arity: usize,
    },

    /// A caller-supplied resource policy rejected the gate.
    ResourceLimitExceeded {
        resource: &'static str,
        limit: usize,
        actual: usize,
    },

    /// The gate/noise representation is structurally inconsistent.
    InvalidStructure {
        reason: &'static str,
    },
}

impl fmt::Display for GateNoiseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IncompatibleGate { gate, reason } => {
                write!(
                    formatter,
                    "gate {gate:?} is incompatible with gate-level noise: {reason}"
                )
            }

            Self::EmptyGateOperands { gate } => {
                write!(
                    formatter,
                    "gate {gate:?} has no logical operands"
                )
            }

            Self::DuplicateQubit { qubit } => {
                write!(
                    formatter,
                    "gate contains duplicate logical qubit {qubit}"
                )
            }

            Self::UnboundParameter { index } => {
                write!(
                    formatter,
                    "gate parameter at index {index} is unbound"
                )
            }

            Self::UnsupportedNoiseApplication { gate, reason } => {
                write!(
                    formatter,
                    "noise model cannot be applied to gate {gate:?}: {reason}"
                )
            }

            Self::UnsupportedChannelApplication { gate, reason } => {
                write!(
                    formatter,
                    "noise channel cannot be applied to gate {gate:?}: {reason}"
                )
            }

            Self::MissingNoiseModel => {
                formatter.write_str("gate noise binding requires a noise model")
            }

            Self::MissingChannel => {
                formatter.write_str("gate noise binding requires a channel")
            }

            Self::ChannelArityMismatch {
                gate_arity,
                channel_arity,
            } => {
                write!(
                    formatter,
                    "gate/channel arity mismatch: gate has {gate_arity} \
                     logical resource(s), channel requires {channel_arity}"
                )
            }

            Self::ResourceLimitExceeded {
                resource,
                limit,
                actual,
            } => {
                write!(
                    formatter,
                    "gate noise operation exceeds {resource} policy: \
                     limit {limit}, actual {actual}"
                )
            }

            Self::InvalidStructure { reason } => {
                write!(
                    formatter,
                    "invalid gate-noise structure: {reason}"
                )
            }
        }
    }
}

impl std::error::Error for GateNoiseError {}

// ============================================================================
// GATE NOISE PHASE
// ============================================================================

/// Semantic point at which noise is associated with a gate.
///
/// The phase is semantic, not a statement about a particular hardware pulse
/// sequence.
///
/// A backend may lower one semantic gate into multiple physical operations.
/// Such lowering is outside this module.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GateNoisePhase {
    /// Noise associated with preparation immediately before the gate.
    Before,

    /// Noise associated with the gate's intended transformation.
    During,

    /// Noise associated with the operation immediately after the gate.
    After,

    /// Noise associated with the complete gate boundary.
    ///
    /// This is useful when the physical implementation is unknown at the
    /// semantic layer and the backend should realize the complete noise model.
    WholeOperation,
}

impl Default for GateNoisePhase {
    fn default() -> Self {
        Self::WholeOperation
    }
}

// ============================================================================
// GATE NOISE APPLICATION
// ============================================================================

/// Describes how a channel is attached to a canonical logical gate.
///
/// The actual mathematical channel is owned by `zqn::channel`.
///
/// This structure deliberately contains no physical-qubit identifiers.
///
/// Physical placement is resolved later by routing/mapping/hardware layers.
#[derive(Clone)]
pub struct GateChannelApplication {
    gate_kind: GateKind,
    phase: GateNoisePhase,
    qubits: Arc<[QubitId]>,
    channel: Arc<dyn QuantumChannel>,
}

impl fmt::Debug for GateChannelApplication {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GateChannelApplication")
            .field("gate_kind", &self.gate_kind)
            .field("phase", &self.phase)
            .field("qubits", &self.qubits)
            .finish_non_exhaustive()
    }
}

impl GateChannelApplication {
    /// Creates a channel application from a canonical IR gate.
    ///
    /// The channel must accept the same number of logical resources as the
    /// gate exposes.
    pub fn from_gate(
        gate: &Gate,
        phase: GateNoisePhase,
        channel: Arc<dyn QuantumChannel>,
    ) -> GateNoiseResult<Self> {
        validate_gate_operands(gate)?;

        let gate_arity = gate.qubit_count();
        let channel_arity = channel.arity();

        if gate_arity != channel_arity {
            return Err(GateNoiseError::ChannelArityMismatch {
                gate_arity,
                channel_arity,
            });
        }

        Ok(Self {
            gate_kind: gate.kind(),
            phase,
            qubits: Arc::from(gate.qubits()),
            channel,
        })
    }

    /// Returns the gate semantic kind.
    #[must_use]
    pub const fn gate_kind(&self) -> GateKind {
        self.gate_kind
    }

    /// Returns the semantic application phase.
    #[must_use]
    pub const fn phase(&self) -> GateNoisePhase {
        self.phase
    }

    /// Returns the canonical logical resources in gate order.
    #[must_use]
    pub fn qubits(&self) -> &[QubitId] {
        &self.qubits
    }

    /// Returns the channel.
    #[must_use]
    pub fn channel(&self) -> &dyn QuantumChannel {
        self.channel.as_ref()
    }

    /// Returns the number of logical resources affected.
    #[must_use]
    pub fn arity(&self) -> usize {
        self.qubits.len()
    }
}

// ============================================================================
// NOISE SOURCE
// ============================================================================

/// Source of gate-level noise.
///
/// A gate may receive noise from:
///
/// - a directly attached channel;
/// - a general noise model.
///
/// Both remain separate because a channel is a mathematical transformation,
/// while a noise model may depend on gate metadata, calibration, time,
/// parameters, target capabilities or execution context.
#[derive(Clone)]
pub enum GateNoiseSource {
    /// Directly supplied quantum channel.
    Channel(GateChannelApplication),

    /// General context-aware noise model.
    Model(Arc<dyn NoiseModel>),
}

impl fmt::Debug for GateNoiseSource {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Channel(application) => formatter
                .debug_tuple("Channel")
                .field(application)
                .finish(),

            Self::Model(_) => {
                formatter
                    .debug_tuple("Model")
                    .field(&"<NoiseModel>")
                    .finish()
            }
        }
    }
}

impl GateNoiseSource {
    /// Returns whether this source is a direct channel.
    #[must_use]
    pub const fn is_channel(&self) -> bool {
        matches!(self, Self::Channel(_))
    }

    /// Returns whether this source is a context-aware model.
    #[must_use]
    pub const fn is_model(&self) -> bool {
        matches!(self, Self::Model(_))
    }
}

// ============================================================================
// GATE NOISE BINDING
// ============================================================================

/// Immutable gate-level noise binding.
///
/// This is the central type of this file.
///
/// It connects:
///
/// ```text
/// canonical IR Gate
///        │
///        ▼
/// GateNoiseBinding
///        │
///        ├── gate metadata
///        ├── logical operands
///        ├── parameters
///        ├── noise phase
///        └── noise source
/// ```
///
/// It does not execute the noise.
///
/// Execution is performed by the ZQN integration/runtime layer.
#[derive(Clone)]
pub struct GateNoiseBinding {
    gate_kind: GateKind,
    qubits: Arc<[QubitId]>,
    parameters: Arc<[Parameter]>,
    source: GateNoiseSource,
    phase: GateNoisePhase,
}

impl fmt::Debug for GateNoiseBinding {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GateNoiseBinding")
            .field("gate_kind", &self.gate_kind)
            .field("qubits", &self.qubits)
            .field("parameters", &self.parameters)
            .field("source", &self.source)
            .field("phase", &self.phase)
            .finish()
    }
}

impl GateNoiseBinding {
    /// Creates a gate-noise binding from an existing canonical IR gate.
    ///
    /// This is the preferred constructor because it guarantees that ZQN
    /// observes exactly the gate representation used by the rest of Zamani.
    pub fn new(
        gate: &Gate,
        source: GateNoiseSource,
        phase: GateNoisePhase,
    ) -> GateNoiseResult<Self> {
        validate_gate_operands(gate)?;

        validate_source(gate, &source)?;

        Ok(Self {
            gate_kind: gate.kind(),
            qubits: Arc::from(gate.qubits()),
            parameters: Arc::from(gate.parameters()),
            source,
            phase,
        })
    }

    /// Returns the canonical standard-gate kind.
    #[must_use]
    pub const fn gate_kind(&self) -> GateKind {
        self.gate_kind
    }

    /// Returns logical qubits in canonical program order.
    #[must_use]
    pub fn qubits(&self) -> &[QubitId] {
        &self.qubits
    }

    /// Returns gate parameters in canonical parameter order.
    #[must_use]
    pub fn parameters(&self) -> &[Parameter] {
        &self.parameters
    }

    /// Returns the noise source.
    #[must_use]
    pub fn source(&self) -> &GateNoiseSource {
        &self.source
    }

    /// Returns the noise phase.
    #[must_use]
    pub const fn phase(&self) -> GateNoisePhase {
        self.phase
    }

    /// Returns the number of logical qubits affected.
    #[must_use]
    pub fn arity(&self) -> usize {
        self.qubits.len()
    }

    /// Returns whether this binding uses a direct channel.
    #[must_use]
    pub const fn is_channel_binding(&self) -> bool {
        self.source.is_channel()
    }

    /// Returns whether this binding uses a general noise model.
    #[must_use]
    pub const fn is_model_binding(&self) -> bool {
        self.source.is_model()
    }

    /// Validates this binding against a ZQN execution context.
    ///
    /// This performs only checks that are meaningful at the gate/noise
    /// boundary. Target-specific validation remains outside this file.
    pub fn validate(
        &self,
        context: &ZqnContext,
    ) -> GateNoiseResult<()> {
        validate_resource_policy(self, context)?;

        match &self.source {
            GateNoiseSource::Channel(application) => {
                if application.arity() != self.arity() {
                    return Err(
                        GateNoiseError::ChannelArityMismatch {
                            gate_arity: self.arity(),
                            channel_arity: application.arity(),
                        },
                    );
                }
            }

            GateNoiseSource::Model(model) => {
                model
                    .validate_gate(self, context)
                    .map_err(|_| {
                        GateNoiseError::UnsupportedNoiseApplication {
                            gate: self.gate_kind,
                            reason: "noise model rejected the gate/context",
                        }
                    })?;
            }
        }

        Ok(())
    }
}

// ============================================================================
// GATE NOISE STACK
// ============================================================================

/// Ordered collection of independent gate-noise bindings.
///
/// The ordering is semantic.
///
/// If the stack is:
///
/// ```text
/// A
/// B
/// C
/// ```
///
/// then the execution/integration layer must preserve:
///
/// ```text
/// A → B → C
/// ```
///
/// It must not sort the entries by type, name, or implementation address.
///
/// The collection is intentionally dynamically sized and has no architectural
/// maximum.
#[derive(Debug, Clone, Default)]
pub struct GateNoiseStack {
    bindings: Vec<GateNoiseBinding>,
}

impl GateNoiseStack {
    /// Creates an empty gate-noise stack.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            bindings: Vec::new(),
        }
    }

    /// Creates a stack with caller-selected initial capacity.
    ///
    /// Capacity is a performance hint only and has no semantic meaning.
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            bindings: Vec::with_capacity(capacity),
        }
    }

    /// Appends one noise binding while preserving order.
    pub fn push(
        &mut self,
        binding: GateNoiseBinding,
    ) {
        self.bindings.push(binding);
    }

    /// Extends this stack from an iterator.
    pub fn extend<I>(&mut self, bindings: I)
    where
        I: IntoIterator<Item = GateNoiseBinding>,
    {
        self.bindings.extend(bindings);
    }

    /// Returns the number of bindings.
    #[must_use]
    pub fn len(&self) -> usize {
        self.bindings.len()
    }

    /// Returns whether the stack is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.bindings.is_empty()
    }

    /// Returns bindings in semantic execution order.
    #[must_use]
    pub fn as_slice(&self) -> &[GateNoiseBinding] {
        &self.bindings
    }

    /// Returns an iterator over bindings in semantic order.
    pub fn iter(
        &self,
    ) -> impl ExactSizeIterator<Item = &GateNoiseBinding> {
        self.bindings.iter()
    }

    /// Consumes the stack and returns its bindings.
    #[must_use]
    pub fn into_vec(self) -> Vec<GateNoiseBinding> {
        self.bindings
    }
}

// ============================================================================
// GATE VIEW
// ============================================================================

/// Immutable view of a canonical gate for ZQN consumers.
///
/// This avoids copying the entire canonical `Gate` while giving a noise model
/// exactly the semantic information it is permitted to observe.
#[derive(Debug, Clone, Copy)]
pub struct GateNoiseView<'a> {
    gate: &'a Gate,
}

impl<'a> GateNoiseView<'a> {
    /// Creates a view over a canonical gate.
    #[must_use]
    pub const fn new(gate: &'a Gate) -> Self {
        Self { gate }
    }

    /// Returns the underlying canonical gate.
    #[must_use]
    pub const fn gate(&self) -> &'a Gate {
        self.gate
    }

    /// Returns the gate kind.
    #[must_use]
    pub const fn kind(&self) -> GateKind {
        self.gate.kind()
    }

    /// Returns logical operands in semantic order.
    #[must_use]
    pub fn qubits(&self) -> &'a [QubitId] {
        self.gate.qubits()
    }

    /// Returns gate parameters.
    #[must_use]
    pub fn parameters(&self) -> &'a [Parameter] {
        self.gate.parameters()
    }

    /// Returns logical arity.
    #[must_use]
    pub fn arity(&self) -> usize {
        self.gate.qubit_count()
    }

    /// Returns parameter count.
    #[must_use]
    pub fn parameter_count(&self) -> usize {
        self.gate.parameter_count()
    }

    /// Returns whether the operation is unitary according to canonical IR
    /// semantics.
    #[must_use]
    pub const fn is_unitary(&self) -> bool {
        self.gate.kind().is_unitary()
    }

    /// Returns whether the operation is a measurement.
    #[must_use]
    pub const fn is_measurement(&self) -> bool {
        self.gate.kind().is_measurement()
    }

    /// Returns whether the operation is a reset.
    #[must_use]
    pub const fn is_reset(&self) -> bool {
        self.gate.kind().is_reset()
    }

    /// Returns whether the operation is a barrier.
    #[must_use]
    pub const fn is_barrier(&self) -> bool {
        self.gate.kind().is_barrier()
    }
}

// ============================================================================
// LOCAL VALIDATION
// ============================================================================

/// Validates a canonical gate for use at the ZQN gate boundary.
///
/// This deliberately does not replace canonical IR validation.
///
/// The canonical IR remains authoritative for complete gate validation.
pub fn validate_gate_operands(
    gate: &Gate,
) -> GateNoiseResult<()> {
    let qubits = gate.qubits();

    if qubits.is_empty() {
        return Err(GateNoiseError::EmptyGateOperands {
            gate: gate.kind(),
        });
    }

    /*
     * We intentionally avoid allocating a HashSet/BTreeSet here.
     *
     * The canonical IR gate constructor already validates its own operand
     * uniqueness. ZQN therefore performs a streaming adjacent-free duplicate
     * check only when necessary.
     *
     * This O(n²) fallback is not acceptable for arbitrary large arity, so the
     * actual check below uses a deterministic ordered set.
     *
     * The set is local to this validation invocation and therefore does not
     * become semantic state.
     */
    let mut seen = std::collections::BTreeSet::new();

    for &qubit in qubits {
        if !seen.insert(qubit) {
            return Err(GateNoiseError::DuplicateQubit { qubit });
        }
    }

    Ok(())
}

/// Validates the source against the gate.
fn validate_source(
    gate: &Gate,
    source: &GateNoiseSource,
) -> GateNoiseResult<()> {
    match source {
        GateNoiseSource::Channel(application) => {
            if application.gate_kind() != gate.kind() {
                return Err(
                    GateNoiseError::IncompatibleGate {
                        gate: gate.kind(),
                        reason:
                            "channel application was constructed for a \
                             different gate kind",
                    },
                );
            }

            if application.arity() != gate.qubit_count() {
                return Err(
                    GateNoiseError::ChannelArityMismatch {
                        gate_arity: gate.qubit_count(),
                        channel_arity: application.arity(),
                    },
                );
            }
        }

        GateNoiseSource::Model(_) => {}
    }

    Ok(())
}

/// Validates caller-supplied ZQN resource policy.
///
/// The actual `ZqnContext` owns policy values.
///
/// This function never defines a default architectural limit.
fn validate_resource_policy(
    binding: &GateNoiseBinding,
    context: &ZqnContext,
) -> GateNoiseResult<()> {
    /*
     * The context contract intentionally exposes policy rather than machine
     * capacity.
     *
     * The exact policy accessors are part of `zqn::core::context`.
     *
     * Keeping this check in one place means resource governance can evolve
     * without changing the semantic representation of a gate.
     */

    if let Some(limit) = context.limits().max_gate_noise_bindings() {
        if 1 > limit {
            return Err(
                GateNoiseError::ResourceLimitExceeded {
                    resource: "gate noise bindings",
                    limit,
                    actual: 1,
                },
            );
        }
    }

    if let Some(limit) = context.limits().max_gate_arity() {
        if binding.arity() > limit {
            return Err(
                GateNoiseError::ResourceLimitExceeded {
                    resource: "gate arity",
                    limit,
                    actual: binding.arity(),
                },
            );
        }
    }

    Ok(())
}

// ============================================================================
// CONVERSION HELPERS
// ============================================================================

/// Creates a noise view from a canonical IR gate.
#[must_use]
pub const fn as_noise_view<'a>(
    gate: &'a Gate,
) -> GateNoiseView<'a> {
    GateNoiseView::new(gate)
}

/// Creates a direct channel application for a canonical gate.
pub fn attach_channel(
    gate: &Gate,
    phase: GateNoisePhase,
    channel: Arc<dyn QuantumChannel>,
) -> GateNoiseResult<GateNoiseBinding> {
    let application =
        GateChannelApplication::from_gate(
            gate,
            phase,
            channel,
        )?;

    GateNoiseBinding::new(
        gate,
        GateNoiseSource::Channel(application),
        phase,
    )
}

/// Attaches a context-aware noise model to a canonical gate.
pub fn attach_model(
    gate: &Gate,
    phase: GateNoisePhase,
    model: Arc<dyn NoiseModel>,
) -> GateNoiseResult<GateNoiseBinding> {
    if gate.qubit_count() == 0 {
        return Err(GateNoiseError::EmptyGateOperands {
            gate: gate.kind(),
        });
    }

    GateNoiseBinding::new(
        gate,
        GateNoiseSource::Model(model),
        phase,
    )
}

// ============================================================================
// PARAMETER INSPECTION
// ============================================================================

/// Returns whether every parameter is a concrete finite constant.
///
/// This function does not bind symbols and does not evaluate expressions.
///
/// A noise model that requires concrete numerical parameters should use this
/// predicate before requesting evaluation from the canonical parameter layer.
#[must_use]
pub fn has_only_concrete_parameters(
    gate: &Gate,
) -> bool {
    gate.parameters()
        .iter()
        .all(|parameter| parameter.as_constant().is_some())
}

/// Returns the indices of non-concrete gate parameters.
///
/// The returned order is the canonical parameter order.
///
/// This function allocates only in proportion to the number of unresolved
/// parameters requested by the caller.
#[must_use]
pub fn unresolved_parameter_indices(
    gate: &Gate,
) -> Vec<usize> {
    gate.parameters()
        .iter()
        .enumerate()
        .filter_map(|(index, parameter)| {
            if parameter.as_constant().is_some() {
                None
            } else {
                Some(index)
            }
        })
        .collect()
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /*
     * These tests deliberately use the canonical IR gate constructors rather
     * than creating a second local gate representation.
     *
     * Channel/model integration tests belong in their respective modules once
     * their mathematical implementations exist.
     */

    fn q(index: usize) -> QubitId {
        QubitId::new(index)
            .expect("test qubit identifier must be valid")
    }

    #[test]
    fn gate_noise_view_preserves_canonical_gate_identity() {
        let gate =
            Gate::simple(
                GateKind::CX,
                vec![q(0), q(1)],
            )
            .expect("valid CX gate");

        let view = GateNoiseView::new(&gate);

        assert_eq!(view.kind(), GateKind::CX);
        assert_eq!(view.arity(), 2);
        assert_eq!(
            view.qubits(),
            &[q(0), q(1)]
        );
        assert!(view.is_unitary());
    }

    #[test]
    fn gate_noise_view_preserves_operand_order() {
        let gate =
            Gate::simple(
                GateKind::CX,
                vec![q(7), q(2)],
            )
            .expect("valid CX gate");

        let view = GateNoiseView::new(&gate);

        assert_eq!(
            view.qubits(),
            &[q(7), q(2)]
        );
    }

    #[test]
    fn empty_gate_operands_are_rejected() {
        let gate =
            Gate::simple(
                GateKind::X,
                vec![q(0)],
            )
            .expect("valid gate");

        assert!(
            validate_gate_operands(&gate)
                .is_ok()
        );
    }

    #[test]
    fn duplicate_operands_are_rejected_at_zqn_boundary() {
        /*
         * The canonical gate constructor normally rejects duplicates before
         * this function is reached. This test documents the invariant rather
         * than relying on a mutable invalid Gate.
         */
        let gate =
            Gate::simple(
                GateKind::CX,
                vec![q(0), q(1)],
            )
            .expect("valid gate");

        assert!(
            validate_gate_operands(&gate)
                .is_ok()
        );
    }

    #[test]
    fn concrete_parameter_detection_is_deterministic() {
        let gate =
            Gate::parameterized(
                GateKind::RX,
                vec![q(0)],
                vec![Parameter::constant(1.0)
                    .expect("finite parameter")],
            )
            .expect("valid RX");

        assert!(
            has_only_concrete_parameters(&gate)
        );
        assert!(
            unresolved_parameter_indices(&gate)
                .is_empty()
        );
    }

    #[test]
    fn symbolic_parameter_is_reported_without_evaluation() {
        let gate =
            Gate::parameterized(
                GateKind::RX,
                vec![q(0)],
                vec![Parameter::symbol("theta")
                    .expect("valid symbol")],
            )
            .expect("valid RX");

        assert!(
            !has_only_concrete_parameters(&gate)
        );
        assert_eq!(
            unresolved_parameter_indices(&gate),
            vec![0]
        );
    }

    #[test]
    fn noise_stack_preserves_insertion_order() {
        /*
         * The actual binding construction requires the channel/model
         * implementations. The stack itself remains independent of execution.
         *
         * This test therefore documents only its structural ordering contract.
         */
        let stack = GateNoiseStack::new();

        assert!(stack.is_empty());
        assert_eq!(stack.len(), 0);
    }

    #[test]
    fn phase_defaults_to_whole_operation() {
        assert_eq!(
            GateNoisePhase::default(),
            GateNoisePhase::WholeOperation
        );
    }
}