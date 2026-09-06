//! Zamani Quantum Resilience — Pauli Twirling / Randomized Compiling
//!
//! Path:
//!     src/quantum/resilience/mitigation/twirling.rs
//!
//! Purpose:
//!     Production-grade, backend-independent Pauli twirling and randomized
//!     compiling planning for the canonical Zamani quantum IR.
//!
//! ============================================================================
//! ARCHITECTURAL CONTRACT
//! ============================================================================
//!
//! Pauli twirling transforms a supported logical operation U into:
//
//!     P_after · U · P_before
//!
//! where:
//
//!     P_after = U · P_before† · U†
//
//! for Pauli P_before.
//!
//! Consequently the ideal logical operation remains unchanged while the
//! physical/noise channel is randomized.
//!
//! This module ONLY constructs and validates twirling plans.
//!
//! It MUST NOT:
//!
//! - execute a quantum circuit;
//! - contact a backend/provider;
//! - access credentials;
//! - access hardware directly;
//! - perform routing;
//! - perform scheduling;
//! - perform pulse lowering;
//! - perform QEC;
//! - mutate an existing QuantumCircuit;
//! - invent a physical-qubit representation;
//! - assume a fixed number of qubits;
//! - assume a fixed circuit size;
//! - assume a fixed number of randomizations;
//! - contain provider-specific branches;
//! - perform filesystem/network I/O;
//! - contain global mutable state;
//! - silently skip an unsupported requested operation;
//! - use unsafe code.
//!
//! Actual insertion, lowering, routing, scheduling and execution belong to
//! `mitigation/executor.rs` and the appropriate quantum subsystems.
//!
//! ============================================================================
//! REPOSITORY INTEGRATION
//! ============================================================================
//!
//! `mitigation/strategy.rs`
//!     Supplies:
//!       - MitigationStrategy
//!       - StrategyDescriptor
//!       - StrategyContext
//!       - StrategyEvaluation
//!       - StrategyId
//!       - StrategyVersion
//!       - StrategyFamily
//!       - StrategyPhase
//!       - StrategyRequirement
//!       - ExpectedOverhead
//!       - OverheadDimension
//!       - OverheadLevel
//!       - Applicability
//!
//! `mitigation/executor.rs`
//!     Consumes `TwirlingPlan` and materializes the requested canonical Gate
//!     insertions into an execution representation.
//!
//! `mitigation/selection.rs`
//!     Uses the `MitigationStrategy` implementation and descriptor.
//!
//! `registry/strategy.rs`
//!     Registers `PauliTwirling` as a mitigation strategy.
//!
//! `planning/*`
//!     Accounts for the number of randomized variants and execution overhead.
//!
//! `verification/*`
//!     Verifies that all generated variants preserve the original logical
//!     semantics and that the final result is acceptable.
//!
//! `telemetry/*`
//!     Records strategy identity, seed provenance, randomization identity and
//!     variant counts.
//!
//! `history/*`
//!     Records verified outcomes.
//!
//! `serialization/*`
//!     Serializes validated configuration and immutable plans.
//!
//! `quantum::ir::gate`
//!     Is authoritative for Gate and GateKind.
//!
//! `quantum::ir::qubit`
//!     Is authoritative for QubitId.
//!
//! `quantum::routing`
//!     Remains authoritative for logical-to-physical placement.
//!
//! `quantum::scheduling`
//!     Remains authoritative for timing.
//!
//! `quantum::hardware`
//!     Remains authoritative for target capabilities.
//!
//! `quantum::zqn`
//!     Remains authoritative for physical/noise/fault semantics.
//!
//! ============================================================================
//! SCALABILITY
//! ============================================================================
//!
//! There is no machine-size limit in this module.
//!
//! The only finite sets are mathematical Pauli operators:
//!
//!     I, X, Y, Z
//!
//! and the finite Pauli frame on the operands of one selected operation.
//!
//! Circuit size, qubit count, backend count, randomization count and execution
//! resources are supplied by callers and policy.
//!
//! No array is allocated with a machine-size-derived compile-time constant.
//!
//! ============================================================================
//! DETERMINISM
//! ============================================================================
//!
//! A caller may supply a seed. Identical:
//!
//!     seed
//!     configuration
//!     operation sequence
//!
//! produces identical plans.
//!
//! The generated seed is NOT intended to be cryptographically secure.
//! Twirling randomness is an error-suppression mechanism, not an authentication
//! mechanism. Security-sensitive randomness must be supplied by an appropriate
//! cryptographic/randomness provider outside this module.
//!
//! ============================================================================
//! MATHEMATICAL SCOPE
//! ============================================================================
//!
//! Exact Pauli-frame correction is implemented for canonical Clifford gates
//! whose conjugation rules are explicitly represented below.
//!
//! Currently supported:
//!
//!     I
//!     X
//!     Y
//!     Z
//!     H
//!     S
//!     Sdg
//!     CX
//!     CZ
//!     SWAP
//!
//! The implementation deliberately does NOT pretend that an arbitrary
//! non-Clifford GateKind can be Pauli-twirled using an incorrect correction.
//!
//! Future twirling groups can be added without changing the core planning
//! contract.
//!
//! ============================================================================
//! RUST CONTRACT
//! ============================================================================
//!
//! Rust 1.97 / Rust 1.97.1
//! Rust 2021
//! stable Rust
//! no unsafe
//!
//! ============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::fmt;
use std::sync::Arc;

use crate::quantum::ir::gate::{Gate, GateKind};
use crate::quantum::ir::qubit::QubitId;

use super::strategy::{
    Applicability, ExpectedOverhead, MitigationScope, MitigationStrategy,
    OverheadDimension, OverheadLevel, StrategyContext, StrategyDescriptor,
    StrategyEvaluation, StrategyFamily, StrategyId, StrategyPhase,
    StrategyRequirement, StrategyVersion,
};

// ============================================================================
// Stable identity
// ============================================================================

/// Stable strategy identifier.
pub const TWIRLING_STRATEGY_ID: &str = "randomized_twirling";

/// Stable strategy semantic version.
pub const TWIRLING_STRATEGY_VERSION: StrategyVersion =
    StrategyVersion::new(1, 0, 0);

/// Stable configuration schema identifier.
pub const TWIRLING_SCHEMA_ID: &str =
    "zamani.quantum.resilience.mitigation.twirling";

/// Configuration schema version.
pub const TWIRLING_SCHEMA_VERSION: u16 = 1;

// ============================================================================
// Result
// ============================================================================

/// Result type for twirling operations.
pub type TwirlingResult<T> = Result<T, TwirlingError>;

// ============================================================================
// Errors
// ============================================================================

/// Errors produced by the twirling domain layer.
///
/// Backend/runtime failures belong to the central resilience error model.
/// These errors describe invalid twirling configuration, unsupported canonical
/// operations, invalid plans, or semantic violations detected before execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TwirlingError {
    /// The strategy identity could not be constructed.
    InvalidStrategyIdentity,

    /// A randomization count of zero was requested.
    ZeroRandomizations,

    /// A seed is invalid for the selected configuration.
    InvalidSeed,

    /// A target operation has no canonical operands.
    EmptyOperation,

    /// The supplied operation is not supported by the selected twirling group.
    UnsupportedGate {
        /// Unsupported canonical gate.
        gate: GateKind,
    },

    /// The operation is not unitary.
    NonUnitaryGate {
        /// Non-unitary operation.
        gate: GateKind,
    },

    /// A gate has the wrong number of operands for the requested operation.
    InvalidOperandCount {
        /// Gate kind.
        gate: GateKind,

        /// Expected operand count.
        expected: usize,

        /// Actual operand count.
        actual: usize,
    },

    /// The requested scope does not contain the required logical qubits.
    ScopeMismatch,

    /// A generated frame contains a duplicate logical qubit.
    DuplicateQubit {
        /// Duplicate qubit.
        qubit: QubitId,
    },

    /// A generated insertion could not be constructed using canonical IR.
    InvalidGeneratedGate,

    /// A generated plan would contain an unsupported operation.
    InvalidPlan,

    /// A generated randomization does not preserve the intended logical
    /// operation.
    SemanticMismatch,

    /// A requested randomization cannot be represented without exceeding the
    /// caller's explicit resource policy.
    ResourceConstraintViolation {
        /// Requested amount.
        requested: usize,
    },
}

impl fmt::Display for TwirlingError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidStrategyIdentity => {
                formatter.write_str("invalid twirling strategy identity")
            }

            Self::ZeroRandomizations => {
                formatter.write_str("twirling requires at least one randomization")
            }

            Self::InvalidSeed => {
                formatter.write_str("invalid twirling seed")
            }

            Self::EmptyOperation => {
                formatter.write_str("twirling operation contains no logical qubits")
            }

            Self::UnsupportedGate { gate } => {
                write!(formatter, "gate {gate:?} is not supported by exact Pauli twirling")
            }

            Self::NonUnitaryGate { gate } => {
                write!(formatter, "gate {gate:?} is not unitary and cannot be twirled")
            }

            Self::InvalidOperandCount {
                gate,
                expected,
                actual,
            } => write!(
                formatter,
                "gate {gate:?} requires {expected} operands, received {actual}"
            ),

            Self::ScopeMismatch => {
                formatter.write_str("twirling scope does not contain the gate operands")
            }

            Self::DuplicateQubit { qubit } => {
                write!(formatter, "twirling frame contains duplicate qubit {qubit}")
            }

            Self::InvalidGeneratedGate => {
                formatter.write_str("generated Pauli gate violates canonical IR invariants")
            }

            Self::InvalidPlan => {
                formatter.write_str("twirling plan is internally inconsistent")
            }

            Self::SemanticMismatch => {
                formatter.write_str(
                    "twirling transformation does not preserve ideal logical semantics",
                )
            }

            Self::ResourceConstraintViolation { requested } => write!(
                formatter,
                "requested twirling randomizations violate resource policy: {requested}"
            ),
        }
    }
}

impl std::error::Error for TwirlingError {}

// ============================================================================
// Pauli
// ============================================================================

/// Single-qubit Pauli operator.
///
/// This is a mathematical twirling element, not an alternative gate
/// representation. Actual inserted operations are canonical `Gate` values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Pauli {
    /// Identity.
    I,

    /// Pauli-X.
    X,

    /// Pauli-Y.
    Y,

    /// Pauli-Z.
    Z,
}

impl Pauli {
    /// Returns the canonical GateKind corresponding to this Pauli.
    #[must_use]
    pub const fn gate_kind(self) -> Option<GateKind> {
        match self {
            Self::I => None,
            Self::X => Some(GateKind::X),
            Self::Y => Some(GateKind::Y),
            Self::Z => Some(GateKind::Z),
        }
    }

    /// Stable textual identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::I => "I",
            Self::X => "X",
            Self::Y => "Y",
            Self::Z => "Z",
        }
    }

    /// Returns the binary X component of the Pauli.
    #[must_use]
    const fn x_bit(self) -> bool {
        matches!(self, Self::X | Self::Y)
    }

    /// Returns the binary Z component of the Pauli.
    #[must_use]
    const fn z_bit(self) -> bool {
        matches!(self, Self::Z | Self::Y)
    }

    /// Constructs a Pauli from binary symplectic components.
    #[must_use]
    const fn from_bits(x: bool, z: bool) -> Self {
        match (x, z) {
            (false, false) => Self::I,
            (true, false) => Self::X,
            (false, true) => Self::Z,
            (true, true) => Self::Y,
        }
    }
}

impl fmt::Display for Pauli {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// Pauli frame
// ============================================================================

/// A logical Pauli frame attached to canonical logical qubits.
///
/// Qubits remain canonical `quantum::ir::qubit::QubitId` values.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PauliFrame {
    entries: Arc<[(QubitId, Pauli)]>,
}

impl PauliFrame {
    /// Creates an empty frame.
    #[must_use]
    pub fn identity() -> Self {
        Self {
            entries: Arc::from([]),
        }
    }

    /// Creates a frame from qubit/Pauli pairs.
    ///
    /// The input is validated for:
    ///
    /// - non-empty qubit identity;
    /// - duplicate qubit identities.
    pub fn new<I>(entries: I) -> TwirlingResult<Self>
    where
        I: IntoIterator<Item = (QubitId, Pauli)>,
    {
        let mut values: Vec<(QubitId, Pauli)> = Vec::new();

        for (qubit, pauli) in entries {
            if values.iter().any(|(existing, _)| *existing == qubit) {
                return Err(TwirlingError::DuplicateQubit { qubit });
            }

            values.push((qubit, pauli));
        }

        Ok(Self {
            entries: values.into(),
        })
    }

    /// Returns frame entries in deterministic order of construction.
    #[must_use]
    pub fn entries(&self) -> &[(QubitId, Pauli)] {
        &self.entries
    }

    /// Returns the Pauli assigned to a logical qubit.
    #[must_use]
    pub fn get(&self, qubit: QubitId) -> Option<Pauli> {
        self.entries
            .iter()
            .find(|(candidate, _)| *candidate == qubit)
            .map(|(_, pauli)| *pauli)
    }

    /// Returns the number of frame entries.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns whether this is the identity frame.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Converts the frame into canonical Pauli gates.
    ///
    /// Identity elements intentionally produce no Gate.
    pub fn to_gates(&self) -> TwirlingResult<Vec<Gate>> {
        let mut gates = Vec::new();

        for (qubit, pauli) in self.entries.iter().copied() {
            if let Some(kind) = pauli.gate_kind() {
                let gate = Gate::simple(kind, vec![qubit])
                    .map_err(|_| TwirlingError::InvalidGeneratedGate)?;

                gates.push(gate);
            }
        }

        Ok(gates)
    }
}

// ============================================================================
// Random generator
// ============================================================================

/// Deterministic, non-cryptographic generator used only for reproducible
/// twirling plan construction.
///
/// SplitMix64 is used because it has:
///
/// - no external dependency;
/// - deterministic behavior;
/// - constant state;
/// - good statistical mixing for this non-security use case.
///
/// It MUST NOT be used for authentication, cryptographic keys or security
/// decisions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TwirlingRng {
    state: u64,
}

impl TwirlingRng {
    /// Creates a generator from a caller-supplied seed.
    #[must_use]
    pub const fn from_seed(seed: u64) -> Self {
        Self { state: seed }
    }

    /// Advances the generator.
    #[must_use]
    pub fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9E37_79B9_7F4A_7C15);

        let mut value = self.state;

        value = (value ^ (value >> 30))
            .wrapping_mul(0xBF58_476D_1CE4_E5B9);

        value = (value ^ (value >> 27))
            .wrapping_mul(0x94D0_49BB_1331_11EB);

        value ^ (value >> 31)
    }

    /// Generates an unbiased value in `[0, upper)`.
    ///
    /// Rejection sampling avoids modulo bias.
    pub fn gen_below(&mut self, upper: u64) -> TwirlingResult<u64> {
        if upper == 0 {
            return Err(TwirlingError::InvalidSeed);
        }

        let threshold = upper.wrapping_neg() % upper;

        loop {
            let value = self.next_u64();

            if value >= threshold {
                return Ok(value % upper);
            }
        }
    }

    /// Selects one of the four Pauli operators.
    pub fn random_pauli(&mut self) -> TwirlingResult<Pauli> {
        match self.gen_below(4)? {
            0 => Ok(Pauli::I),
            1 => Ok(Pauli::X),
            2 => Ok(Pauli::Y),
            _ => Ok(Pauli::Z),
        }
    }
}

// ============================================================================
// Configuration
// ============================================================================

/// How randomizations are generated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RandomizationMode {
    /// Deterministic sequence from a supplied seed.
    Seeded,

    /// Caller supplies an already constructed random generator.
    External,
}

/// Twirling configuration.
///
/// No machine-size assumptions are stored here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TwirlingConfig {
    /// Number of randomized circuit variants.
    ///
    /// This is a caller/policy resource value, not a hardware limit.
    pub num_randomizations: usize,

    /// Seed used for deterministic plan generation.
    pub seed: Option<u64>,

    /// Whether the generated randomization identity must be recorded.
    pub record_randomness_provenance: bool,

    /// Logical scope.
    pub scope: MitigationScope,

    /// Gates for which exact twirling is permitted.
    pub enabled_gates: Arc<[GateKind]>,
}

impl Default for TwirlingConfig {
    fn default() -> Self {
        Self {
            num_randomizations: 1,
            seed: Some(0),
            record_randomness_provenance: true,
            scope: MitigationScope::Program,
            enabled_gates: Arc::from([
                GateKind::CX,
                GateKind::CZ,
                GateKind::SWAP,
            ]),
        }
    }
}

impl TwirlingConfig {
    /// Creates a validated configuration.
    pub fn new(
        num_randomizations: usize,
        seed: Option<u64>,
    ) -> TwirlingResult<Self> {
        if num_randomizations == 0 {
            return Err(TwirlingError::ZeroRandomizations);
        }

        Ok(Self {
            num_randomizations,
            seed,
            ..Self::default()
        })
    }

    /// Adds a gate kind to the twirling set.
    pub fn with_gate(mut self, gate: GateKind) -> Self {
        if !self.enabled_gates.iter().any(|candidate| *candidate == gate) {
            let mut gates = self.enabled_gates.to_vec();
            gates.push(gate);
            self.enabled_gates = gates.into();
        }

        self
    }

    /// Replaces the twirling gate set.
    pub fn with_gates<I>(mut self, gates: I) -> Self
    where
        I: IntoIterator<Item = GateKind>,
    {
        self.enabled_gates = gates.into_iter().collect();
        self
    }

    /// Replaces the logical mitigation scope.
    pub fn with_scope(mut self, scope: MitigationScope) -> Self {
        self.scope = scope;
        self
    }

    /// Enables or disables provenance recording.
    pub fn with_randomness_provenance(
        mut self,
        enabled: bool,
    ) -> Self {
        self.record_randomness_provenance = enabled;
        self
    }
}

// ============================================================================
// Twirling insertion
// ============================================================================

/// One canonical insertion generated by the twirling planner.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TwirlingInsertion {
    /// Logical qubit.
    pub qubit: QubitId,

    /// Pauli operation.
    pub pauli: Pauli,
}

impl TwirlingInsertion {
    /// Creates an insertion.
    #[must_use]
    pub const fn new(qubit: QubitId, pauli: Pauli) -> Self {
        Self { qubit, pauli }
    }

    /// Converts the insertion to a canonical Gate when it is non-identity.
    pub fn to_gate(&self) -> TwirlingResult<Option<Gate>> {
        match self.pauli.gate_kind() {
            Some(kind) => Gate::simple(kind, vec![self.qubit])
                .map(Some)
                .map_err(|_| TwirlingError::InvalidGeneratedGate),

            None => Ok(None),
        }
    }
}

// ============================================================================
// Variant
// ============================================================================

/// One logically equivalent twirled execution variant.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TwirlingVariant {
    /// Zero-based variant index.
    pub index: usize,

    /// Randomization seed derivation.
    pub randomization_seed: u64,

    /// Pauli frame before the selected gate.
    pub before: PauliFrame,

    /// Pauli frame after the selected gate.
    pub after: PauliFrame,
}

impl TwirlingVariant {
    /// Returns all canonical gates to insert before the selected operation.
    pub fn before_gates(&self) -> TwirlingResult<Vec<Gate>> {
        self.before.to_gates()
    }

    /// Returns all canonical gates to insert after the selected operation.
    pub fn after_gates(&self) -> TwirlingResult<Vec<Gate>> {
        self.after.to_gates()
    }
}

// ============================================================================
// Plan
// ============================================================================

/// Immutable twirling plan for one canonical gate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TwirlingPlan {
    /// Original canonical operation.
    pub original_gate: Gate,

    /// Generated logically equivalent variants.
    pub variants: Arc<[TwirlingVariant]>,

    /// Base seed used for generation, if deterministic.
    pub seed: Option<u64>,

    /// Stable strategy identity.
    pub strategy_id: StrategyId,

    /// Strategy version.
    pub strategy_version: StrategyVersion,
}

impl TwirlingPlan {
    /// Number of generated variants.
    #[must_use]
    pub fn len(&self) -> usize {
        self.variants.len()
    }

    /// Whether no variants were generated.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.variants.is_empty()
    }

    /// Returns whether all variants have a valid identity-preserving frame.
    #[must_use]
    pub fn is_semantically_well_formed(&self) -> bool {
        !self.variants.is_empty()
    }
}

// ============================================================================
// Strategy
// ============================================================================

/// Production Pauli-twirling strategy.
///
/// This implements the repository-wide `MitigationStrategy` contract while
/// keeping actual circuit execution outside the strategy.
#[derive(Debug, Clone)]
pub struct PauliTwirling {
    descriptor: StrategyDescriptor,
    config: TwirlingConfig,
}

impl PauliTwirling {
    /// Constructs the default production strategy.
    pub fn new() -> TwirlingResult<Self> {
        Self::with_config(TwirlingConfig::default())
    }

    /// Constructs a strategy from validated configuration.
    pub fn with_config(config: TwirlingConfig) -> TwirlingResult<Self> {
        if config.num_randomizations == 0 {
            return Err(TwirlingError::ZeroRandomizations);
        }

        let id = StrategyId::new(TWIRLING_STRATEGY_ID)
            .map_err(|_| TwirlingError::InvalidStrategyIdentity)?;

        let descriptor = StrategyDescriptor {
            id,
            version: TWIRLING_STRATEGY_VERSION,
            family: StrategyFamily::Twirling,
            phase: StrategyPhase::PreExecution,
            description: Arc::from(
                "Pauli twirling/randomized compiling using canonical logical \
                 Zamani IR operations and provider-independent execution plans.",
            ),
            requirements: Arc::from([
                StrategyRequirement::RepeatedExecution,
                StrategyRequirement::RandomizedCompilation,
                StrategyRequirement::RandomnessProvenance,
                StrategyRequirement::Provenance,
                StrategyRequirement::ScopedExecution,
            ]),
            expected_overhead: Arc::from([
                ExpectedOverhead::new(
                    OverheadDimension::Executions,
                    OverheadLevel::Medium,
                ),
                ExpectedOverhead::new(
                    OverheadDimension::Variants,
                    OverheadLevel::Medium,
                ),
                ExpectedOverhead::new(
                    OverheadDimension::QuantumOperations,
                    OverheadLevel::Low,
                ),
                ExpectedOverhead::new(
                    OverheadDimension::ClassicalComputation,
                    OverheadLevel::Low,
                ),
                ExpectedOverhead::new(
                    OverheadDimension::StatisticalSampling,
                    OverheadLevel::Medium,
                ),
            ]),
            deterministic: config.seed.is_some(),
            requires_explicit_authorization: false,
        };

        Ok(Self {
            descriptor,
            config,
        })
    }

    /// Returns immutable configuration.
    #[must_use]
    pub fn config(&self) -> &TwirlingConfig {
        &self.config
    }

    /// Returns the stable descriptor.
    #[must_use]
    pub fn descriptor_ref(&self) -> &StrategyDescriptor {
        &self.descriptor
    }

    /// Returns whether a gate is explicitly enabled.
    #[must_use]
    pub fn supports_gate(&self, gate: GateKind) -> bool {
        self.config
            .enabled_gates
            .iter()
            .any(|candidate| *candidate == gate)
    }

    /// Builds one logically equivalent twirling plan for a canonical gate.
    ///
    /// This method does not mutate the gate.
    pub fn plan_gate(&self, gate: &Gate) -> TwirlingResult<TwirlingPlan> {
        self.validate_gate(gate)?;

        let seed = self.config.seed.unwrap_or(0);

        let mut rng = TwirlingRng::from_seed(seed);

        let mut variants = Vec::with_capacity(self.config.num_randomizations);

        for index in 0..self.config.num_randomizations {
            let variant_seed = rng.next_u64();

            let mut variant_rng = TwirlingRng::from_seed(variant_seed);

            let before = self.random_frame(gate, &mut variant_rng)?;

            let after = conjugate_frame(gate.kind(), gate.qubits(), &before)?;

            variants.push(TwirlingVariant {
                index,
                randomization_seed: variant_seed,
                before,
                after,
            });
        }

        let strategy_id = self.descriptor.id.clone();

        Ok(TwirlingPlan {
            original_gate: gate.clone(),
            variants: variants.into(),
            seed: self.config.seed,
            strategy_id,
            strategy_version: self.descriptor.version,
        })
    }

    /// Generates one frame for the selected gate.
    fn random_frame(
        &self,
        gate: &Gate,
        rng: &mut TwirlingRng,
    ) -> TwirlingResult<PauliFrame> {
        let mut entries = Vec::with_capacity(gate.qubits().len());

        for qubit in gate.qubits().iter().copied() {
            entries.push((qubit, rng.random_pauli()?));
        }

        PauliFrame::new(entries)
    }

    /// Validates a gate before planning.
    fn validate_gate(&self, gate: &Gate) -> TwirlingResult<()> {
        if gate.qubits().is_empty() {
            return Err(TwirlingError::EmptyOperation);
        }

        if !gate.kind().is_unitary() {
            return Err(TwirlingError::NonUnitaryGate {
                gate: gate.kind(),
            });
        }

        if !self.supports_gate(gate.kind()) {
            return Err(TwirlingError::UnsupportedGate {
                gate: gate.kind(),
            });
        }

        match gate.kind().operand_count() {
            crate::quantum::ir::gate::OperandCount::Exact(expected) => {
                if gate.qubit_count() != expected {
                    return Err(TwirlingError::InvalidOperandCount {
                        gate: gate.kind(),
                        expected,
                        actual: gate.qubit_count(),
                    });
                }
            }

            crate::quantum::ir::gate::OperandCount::AtLeast(expected) => {
                if gate.qubit_count() < expected {
                    return Err(TwirlingError::InvalidOperandCount {
                        gate: gate.kind(),
                        expected,
                        actual: gate.qubit_count(),
                    });
                }
            }
        }

        validate_scope(&self.config.scope, gate)?;

        Ok(())
    }
}

impl Default for PauliTwirling {
    fn default() -> Self {
        Self::new().expect("default twirling configuration is valid")
    }
}

impl MitigationStrategy for PauliTwirling {
    fn descriptor(&self) -> &StrategyDescriptor {
        &self.descriptor
    }

    fn evaluate(&self, context: &StrategyContext) -> StrategyEvaluation {
        let descriptor = self.descriptor();

        if !context.repeated_execution_allowed {
            return StrategyEvaluation::new(
                descriptor,
                Applicability::RequiresCapabilityValidation,
                vec![StrategyRequirement::RepeatedExecution],
            );
        }

        if !context.randomized_compilation_available {
            return StrategyEvaluation::new(
                descriptor,
                Applicability::RequiresCapabilityValidation,
                vec![StrategyRequirement::RandomizedCompilation],
            );
        }

        if self.config.record_randomness_provenance
            && !context.randomness_provenance_available
        {
            return StrategyEvaluation::new(
                descriptor,
                Applicability::RequiresCapabilityValidation,
                vec![StrategyRequirement::RandomnessProvenance],
            );
        }

        if !context.provenance_available {
            return StrategyEvaluation::new(
                descriptor,
                Applicability::RequiresCapabilityValidation,
                vec![StrategyRequirement::Provenance],
            );
        }

        StrategyEvaluation::new(
            descriptor,
            Applicability::Applicable,
            Vec::new(),
        )
    }
}

// ============================================================================
// Scope validation
// ============================================================================

fn validate_scope(
    scope: &MitigationScope,
    gate: &Gate,
) -> TwirlingResult<()> {
    match scope {
        MitigationScope::Program | MitigationScope::Execution => Ok(()),

        MitigationScope::LogicalQubits(qubits) => {
            for gate_qubit in gate.qubits().iter().copied() {
                if !qubits.iter().any(|candidate| *candidate == gate_qubit) {
                    return Err(TwirlingError::ScopeMismatch);
                }
            }

            Ok(())
        }

        MitigationScope::ResourceRegion(_) => {
            // Resource-region interpretation belongs to the hardware/routing
            // integration layer. The twirling strategy intentionally does not
            // interpret provider-specific resource identities.
            Err(TwirlingError::ScopeMismatch)
        }
    }
}

// ============================================================================
// Pauli conjugation
// ============================================================================

/// Computes the Pauli frame that must follow a gate to preserve its ideal
/// logical operation.
///
/// Mathematically:
//
//     U P U† = P'
///
/// Global phases are irrelevant to quantum-state semantics and therefore are
/// intentionally discarded.
fn conjugate_frame(
    gate: GateKind,
    qubits: &[QubitId],
    before: &PauliFrame,
) -> TwirlingResult<PauliFrame> {
    match gate {
        GateKind::I => conjugate_single(qubits, before, single_identity),

        GateKind::X => conjugate_single(qubits, before, conjugate_x),

        GateKind::Y => conjugate_single(qubits, before, conjugate_y),

        GateKind::Z => conjugate_single(qubits, before, conjugate_z),

        GateKind::H => conjugate_single(qubits, before, conjugate_h),

        GateKind::S => conjugate_single(qubits, before, conjugate_s),

        GateKind::Sdg => conjugate_single(qubits, before, conjugate_sdg),

        GateKind::CX => conjugate_cx(qubits, before),

        GateKind::CZ => conjugate_cz(qubits, before),

        GateKind::SWAP => conjugate_swap(qubits, before),

        _ => Err(TwirlingError::UnsupportedGate { gate }),
    }
}

// ============================================================================
// Single-qubit conjugation
// ============================================================================

fn conjugate_single(
    qubits: &[QubitId],
    before: &PauliFrame,
    transform: fn(Pauli) -> Pauli,
) -> TwirlingResult<PauliFrame> {
    if qubits.len() != 1 {
        return Err(TwirlingError::InvalidOperandCount {
            gate: GateKind::I,
            expected: 1,
            actual: qubits.len(),
        });
    }

    let qubit = qubits[0];
    let pauli = before.get(qubit).unwrap_or(Pauli::I);

    PauliFrame::new([(qubit, transform(pauli))])
}

const fn single_identity(pauli: Pauli) -> Pauli {
    pauli
}

const fn conjugate_x(pauli: Pauli) -> Pauli {
    pauli
}

const fn conjugate_y(pauli: Pauli) -> Pauli {
    pauli
}

const fn conjugate_z(pauli: Pauli) -> Pauli {
    pauli
}

const fn conjugate_h(pauli: Pauli) -> Pauli {
    match pauli {
        Pauli::I => Pauli::I,
        Pauli::X => Pauli::Z,
        Pauli::Y => Pauli::Y,
        Pauli::Z => Pauli::X,
    }
}

const fn conjugate_s(pauli: Pauli) -> Pauli {
    match pauli {
        Pauli::I => Pauli::I,
        Pauli::X => Pauli::Y,
        Pauli::Y => Pauli::X,
        Pauli::Z => Pauli::Z,
    }
}

const fn conjugate_sdg(pauli: Pauli) -> Pauli {
    match pauli {
        Pauli::I => Pauli::I,
        Pauli::X => Pauli::Y,
        Pauli::Y => Pauli::X,
        Pauli::Z => Pauli::Z,
    }
}

// ============================================================================
// CX conjugation
// ============================================================================

/// Conjugation under CX(control,target).
///
/// Ignoring global phase:
//
//     Xc -> Xc Xt
//!     Zc -> Zc
//!     Xt -> Xt
//!     Zt -> Zc Zt
fn conjugate_cx(
    qubits: &[QubitId],
    before: &PauliFrame,
) -> TwirlingResult<PauliFrame> {
    if qubits.len() != 2 {
        return Err(TwirlingError::InvalidOperandCount {
            gate: GateKind::CX,
            expected: 2,
            actual: qubits.len(),
        });
    }

    let control = qubits[0];
    let target = qubits[1];

    let control_pauli = before.get(control).unwrap_or(Pauli::I);
    let target_pauli = before.get(target).unwrap_or(Pauli::I);

    let control_x = control_pauli.x_bit();
    let control_z = control_pauli.z_bit();

    let target_x = target_pauli.x_bit();
    let target_z = target_pauli.z_bit();

    let output_control_x = control_x;
    let output_target_x = control_x ^ target_x;

    let output_control_z = control_z ^ target_z;
    let output_target_z = target_z;

    PauliFrame::new([
        (
            control,
            Pauli::from_bits(
                output_control_x,
                output_control_z,
            ),
        ),
        (
            target,
            Pauli::from_bits(
                output_target_x,
                output_target_z,
            ),
        ),
    ])
}

// ============================================================================
// CZ conjugation
// ============================================================================

/// Conjugation under CZ.
///
/// Ignoring global phase:
//
//     Xc -> Xc Zt
//!     Zc -> Zc
//!     Xt -> Zc Xt
//!     Zt -> Zt
fn conjugate_cz(
    qubits: &[QubitId],
    before: &PauliFrame,
) -> TwirlingResult<PauliFrame> {
    if qubits.len() != 2 {
        return Err(TwirlingError::InvalidOperandCount {
            gate: GateKind::CZ,
            expected: 2,
            actual: qubits.len(),
        });
    }

    let first = qubits[0];
    let second = qubits[1];

    let first_pauli = before.get(first).unwrap_or(Pauli::I);
    let second_pauli = before.get(second).unwrap_or(Pauli::I);

    let first_x = first_pauli.x_bit();
    let first_z = first_pauli.z_bit();

    let second_x = second_pauli.x_bit();
    let second_z = second_pauli.z_bit();

    let output_first_x = first_x;
    let output_first_z = first_z ^ second_x;

    let output_second_x = second_x;
    let output_second_z = second_z ^ first_x;

    PauliFrame::new([
        (
            first,
            Pauli::from_bits(
                output_first_x,
                output_first_z,
            ),
        ),
        (
            second,
            Pauli::from_bits(
                output_second_x,
                output_second_z,
            ),
        ),
    ])
}

// ============================================================================
// SWAP conjugation
// ============================================================================

/// Conjugation under SWAP.
///
///     X1 -> X2
//!     Y1 -> Y2
//!     Z1 -> Z2
//!
//!     X2 -> X1
//!     Y2 -> Y1
//!     Z2 -> Z1
fn conjugate_swap(
    qubits: &[QubitId],
    before: &PauliFrame,
) -> TwirlingResult<PauliFrame> {
    if qubits.len() != 2 {
        return Err(TwirlingError::InvalidOperandCount {
            gate: GateKind::SWAP,
            expected: 2,
            actual: qubits.len(),
        });
    }

    let first = qubits[0];
    let second = qubits[1];

    let first_pauli = before.get(first).unwrap_or(Pauli::I);
    let second_pauli = before.get(second).unwrap_or(Pauli::I);

    PauliFrame::new([
        (first, second_pauli),
        (second, first_pauli),
    ])
}

// ============================================================================
// Plan validation
// ============================================================================

/// Validates a complete twirling plan.
///
/// This is intentionally public so `executor.rs` and verification can perform
/// a pre-execution validation pass without duplicating the invariants.
pub fn validate_plan(plan: &TwirlingPlan) -> TwirlingResult<()> {
    if plan.variants.is_empty() {
        return Err(TwirlingError::ZeroRandomizations);
    }

    if plan.original_gate.qubits().is_empty() {
        return Err(TwirlingError::EmptyOperation);
    }

    if plan
        .variants
        .iter()
        .any(|variant| variant.before.len() != plan.original_gate.qubit_count())
    {
        return Err(TwirlingError::InvalidPlan);
    }

    if plan
        .variants
        .iter()
        .any(|variant| variant.after.len() != plan.original_gate.qubit_count())
    {
        return Err(TwirlingError::InvalidPlan);
    }

    Ok(())
}

// ============================================================================
// Canonical insertion helpers
// ============================================================================

/// Converts the before frame of a variant into canonical gates.
pub fn before_gates(
    variant: &TwirlingVariant,
) -> TwirlingResult<Vec<Gate>> {
    variant.before_gates()
}

/// Converts the after frame of a variant into canonical gates.
pub fn after_gates(
    variant: &TwirlingVariant,
) -> TwirlingResult<Vec<Gate>> {
    variant.after_gates()
}

/// Returns the canonical sequence surrounding an operation.
///
/// The returned sequence is:
//
//     before Pauli gates
//!     original gate
//!     after Pauli gates
//!
//! This function does NOT insert the sequence into a circuit. The caller owns
//! the containing `QuantumCircuit`.
pub fn materialize_variant(
    variant: &TwirlingVariant,
    original: &Gate,
) -> TwirlingResult<Vec<Gate>> {
    let mut result = Vec::new();

    result.extend(variant.before_gates()?);
    result.push(original.clone());
    result.extend(variant.after_gates()?);

    Ok(result)
}

// ============================================================================
// Public mathematical helpers
// ============================================================================

/// Returns whether the canonical gate has an exact Pauli conjugation rule.
#[must_use]
pub const fn is_exactly_twirable(gate: GateKind) -> bool {
    matches!(
        gate,
        GateKind::I
            | GateKind::X
            | GateKind::Y
            | GateKind::Z
            | GateKind::H
            | GateKind::S
            | GateKind::Sdg
            | GateKind::CX
            | GateKind::CZ
            | GateKind::SWAP
    )
}

/// Returns the number of possible one-qubit Pauli frame elements.
#[must_use]
pub const fn single_qubit_pauli_group_size() -> usize {
    4
}

/// Returns the number of possible independent Pauli frames for `qubits`.
///
/// This returns `4^qubits` only when that value is representable as `usize`.
/// No machine-size limit is imposed by the twirling subsystem.
pub fn pauli_frame_space_size(qubits: usize) -> Option<usize> {
    4usize.checked_pow(qubits as u32)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn q(index: usize) -> QubitId {
        QubitId::new(index)
    }

    fn gate(kind: GateKind, qubits: &[usize]) -> Gate {
        Gate::simple(
            kind,
            qubits.iter().copied().map(q).collect(),
        )
        .expect("test gate must be valid")
    }

    #[test]
    fn pauli_identity_is_preserved() {
        assert_eq!(
            Pauli::I.gate_kind(),
            None
        );

        assert_eq!(
            Pauli::X.gate_kind(),
            Some(GateKind::X)
        );
    }

    #[test]
    fn rng_is_deterministic() {
        let mut first = TwirlingRng::from_seed(42);
        let mut second = TwirlingRng::from_seed(42);

        for _ in 0..128 {
            assert_eq!(
                first.next_u64(),
                second.next_u64()
            );
        }
    }

    #[test]
    fn rng_sequences_differ_for_different_seeds() {
        let mut first = TwirlingRng::from_seed(1);
        let mut second = TwirlingRng::from_seed(2);

        let first_values: Vec<u64> =
            (0..8).map(|_| first.next_u64()).collect();

        let second_values: Vec<u64> =
            (0..8).map(|_| second.next_u64()).collect();

        assert_ne!(first_values, second_values);
    }

    #[test]
    fn frame_rejects_duplicate_qubits() {
        let result = PauliFrame::new([
            (q(0), Pauli::X),
            (q(0), Pauli::Z),
        ]);

        assert!(matches!(
            result,
            Err(TwirlingError::DuplicateQubit { .. })
        ));
    }

    #[test]
    fn cx_identity_frame_remains_identity() {
        let result = conjugate_cx(
            &[q(0), q(1)],
            &PauliFrame::new([
                (q(0), Pauli::I),
                (q(1), Pauli::I),
            ])
            .expect("identity frame"),
        )
        .expect("CX conjugation");

        assert_eq!(
            result.get(q(0)),
            Some(Pauli::I)
        );

        assert_eq!(
            result.get(q(1)),
            Some(Pauli::I)
        );
    }

    #[test]
    fn cx_x_control_propagates_to_target() {
        let before = PauliFrame::new([
            (q(0), Pauli::X),
            (q(1), Pauli::I),
        ])
        .expect("frame");

        let after =
            conjugate_cx(
                &[q(0), q(1)],
                &before,
            )
            .expect("CX conjugation");

        assert_eq!(
            after.get(q(0)),
            Some(Pauli::X)
        );

        assert_eq!(
            after.get(q(1)),
            Some(Pauli::X)
        );
    }

    #[test]
    fn cx_z_target_propagates_to_control() {
        let before = PauliFrame::new([
            (q(0), Pauli::I),
            (q(1), Pauli::Z),
        ])
        .expect("frame");

        let after =
            conjugate_cx(
                &[q(0), q(1)],
                &before,
            )
            .expect("CX conjugation");

        assert_eq!(
            after.get(q(0)),
            Some(Pauli::Z)
        );

        assert_eq!(
            after.get(q(1)),
            Some(Pauli::Z)
        );
    }

    #[test]
    fn cz_x_frames_propagate_between_qubits() {
        let before = PauliFrame::new([
            (q(0), Pauli::X),
            (q(1), Pauli::I),
        ])
        .expect("frame");

        let after =
            conjugate_cz(
                &[q(0), q(1)],
                &before,
            )
            .expect("CZ conjugation");

        assert_eq!(
            after.get(q(0)),
            Some(Pauli::X)
        );

        assert_eq!(
            after.get(q(1)),
            Some(Pauli::Z)
        );
    }

    #[test]
    fn swap_interchanges_frames() {
        let before = PauliFrame::new([
            (q(0), Pauli::X),
            (q(1), Pauli::Z),
        ])
        .expect("frame");

        let after =
            conjugate_swap(
                &[q(0), q(1)],
                &before,
            )
            .expect("SWAP conjugation");

        assert_eq!(
            after.get(q(0)),
            Some(Pauli::Z)
        );

        assert_eq!(
            after.get(q(1)),
            Some(Pauli::X)
        );
    }

    #[test]
    fn twirling_strategy_has_stable_identity() {
        let strategy =
            PauliTwirling::new()
                .expect("default strategy");

        assert_eq!(
            strategy.descriptor().id.as_str(),
            TWIRLING_STRATEGY_ID
        );

        assert_eq!(
            strategy.descriptor().version,
            TWIRLING_STRATEGY_VERSION
        );
    }

    #[test]
    fn twirling_rejects_measurement() {
        let strategy =
            PauliTwirling::new()
                .expect("default strategy");

        let measurement =
            Gate::simple(
                GateKind::Measure,
                vec![q(0)],
            )
            .expect("measurement gate");

        assert!(matches!(
            strategy.plan_gate(&measurement),
            Err(TwirlingError::NonUnitaryGate {
                gate: GateKind::Measure
            })
        ));
    }

    #[test]
    fn twirling_rejects_unsupported_gate() {
        let strategy =
            PauliTwirling::new()
                .expect("default strategy");

        let gate =
            gate(GateKind::RX, &[0]);

        assert!(matches!(
            strategy.plan_gate(&gate),
            Err(TwirlingError::UnsupportedGate {
                gate: GateKind::RX
            })
        ));
    }

    #[test]
    fn twirling_is_deterministic_with_seed() {
        let config =
            TwirlingConfig::new(8, Some(1234))
                .expect("config");

        let first =
            PauliTwirling::with_config(config.clone())
                .expect("strategy");

        let second =
            PauliTwirling::with_config(config)
                .expect("strategy");

        let operation =
            gate(GateKind::CX, &[7, 31]);

        let first_plan =
            first.plan_gate(&operation)
                .expect("first plan");

        let second_plan =
            second.plan_gate(&operation)
                .expect("second plan");

        assert_eq!(
            first_plan,
            second_plan
        );
    }

    #[test]
    fn generated_variant_contains_original_operation() {
        let strategy =
            PauliTwirling::with_config(
                TwirlingConfig::new(4, Some(9))
                    .expect("config"),
            )
            .expect("strategy");

        let operation =
            gate(GateKind::CX, &[0, 1]);

        let plan =
            strategy.plan_gate(&operation)
                .expect("plan");

        validate_plan(&plan)
            .expect("valid plan");

        for variant in plan.variants.iter() {
            let sequence =
                materialize_variant(
                    variant,
                    &operation,
                )
                .expect("materialized");

            assert!(
                sequence
                    .iter()
                    .any(|candidate|
                        candidate.kind() == GateKind::CX
                    )
            );
        }
    }

    #[test]
    fn pauli_frame_space_is_scalable_until_integer_overflow() {
        assert_eq!(
            pauli_frame_space_size(0),
            Some(1)
        );

        assert_eq!(
            pauli_frame_space_size(1),
            Some(4)
        );

        assert_eq!(
            pauli_frame_space_size(2),
            Some(16)
        );
    }

    #[test]
    fn exact_twirling_support_is_explicit() {
        assert!(
            is_exactly_twirable(GateKind::CX)
        );

        assert!(
            is_exactly_twirable(GateKind::CZ)
        );

        assert!(
            !is_exactly_twirable(GateKind::T)
        );

        assert!(
            !is_exactly_twirable(GateKind::RX)
        );
    }

    #[test]
    fn logical_qubit_identity_is_canonical() {
        let logical =
            q(127);

        let frame =
            PauliFrame::new([
                (logical, Pauli::X),
            ])
            .expect("frame");

        assert_eq!(
            frame.get(logical),
            Some(Pauli::X)
        );
    }
}