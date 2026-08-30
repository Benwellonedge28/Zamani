//! Zamani Quantum Optimization — Canonical Rewrite Rules
//!
//! Immutable, backend-independent rewrite-rule contracts and the built-in
//! exact optimization rule catalogue.
//!
//! # Architectural position
//!
//! ```text
//! quantum::ir::Gate / GateKind
//!             │
//!             ▼
//! optimization::operation
//!             │
//!             ▼
//! optimization::rules       ← this module
//!             │
//!             ├── pattern matching
//!             ├── rewrite engine
//!             ├── optimization passes
//!             ├── algebraic optimization
//!             ├── fault-tolerant optimization
//!             └── verification
//! ```
//!
//! This module defines RULES. It does not:
//!
//! - define another quantum IR;
//! - mutate circuits;
//! - match circuits;
//! - perform rewrites;
//! - perform routing;
//! - perform scheduling;
//! - perform synthesis;
//! - execute quantum programs;
//! - communicate with hardware;
//! - perform semantic equivalence checking itself.
//!
//! The canonical quantum representation remains `quantum::ir::Gate` and
//! `quantum::ir::GateKind`.
//!
//! # Integration contract
//!
//! Future optimization modules consume this file as follows:
//!
//! - `pattern.rs` consumes [`RulePattern`].
//! - `matcher.rs` evaluates [`RulePattern`] and [`RulePrecondition`].
//! - `rewrite.rs` evaluates [`RuleReplacement`] and constructs canonical IR.
//! - `registry.rs` indexes [`RuleMetadata`].
//! - `pass.rs` uses rule metadata when implementing passes.
//! - `pipeline.rs` selects rules according to profiles and objectives.
//! - `verification/*` uses [`RuleEquivalence`] and [`RuleSafety`].
//! - `cost.rs` consumes [`RuleCost`].
//!
//! None of those modules need to modify this contract merely because they are
//! implemented later.
//!
//! # Semantic safety
//!
//! Every rule declares:
//!
//! - stable identifier;
//! - category;
//! - pattern;
//! - replacement;
//! - parameter expressions;
//! - preconditions;
//! - postconditions;
//! - required analyses;
//! - semantic equivalence;
//! - safety classification;
//! - activation policy;
//! - resource/cost metadata.
//!
//! Rules involving global phase are not silently classified as exact-unitary
//! rewrites.
//!
//! # Scaling
//!
//! The catalogue is static and immutable. It does not contain circuit-sized
//! state and does not scan circuits. Matching complexity belongs to the
//! matcher/rewrite infrastructure. This keeps this module suitable for
//! circuits ranging from tiny examples to extremely large circuits, subject
//! only to the resource limits imposed by later optimization infrastructure.
//!
//! # Rust
//!
//! - Rust 1.97 / 1.97.1
//! - Rust 2021
//! - no nightly features
//! - no external dependencies
//! - no `unsafe`

use std::fmt;

use crate::quantum::ir::GateKind;
use crate::quantum::optimization::errors::RuleIdentifier;

// =============================================================================
// Rule categories
// =============================================================================

/// Semantic family of an optimization rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RuleCategory {
    /// Identity removal.
    Identity,

    /// Cancellation of an operation and its inverse.
    InverseCancellation,

    /// Combination of compatible parameterized operations.
    RotationComposition,

    /// Clifford algebra/template transformation.
    Clifford,

    /// Pauli algebra transformation.
    Pauli,

    /// Phase hierarchy transformation.
    Phase,

    /// Controlled-operation transformation.
    Controlled,

    /// Qubit permutation transformation.
    Permutation,

    /// Generic gate fusion.
    Fusion,

    /// Canonical representation normalization.
    Canonicalization,

    /// Fault-tolerant resource optimization.
    FaultTolerant,
}

impl fmt::Display for RuleCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Identity => "identity",
            Self::InverseCancellation => "inverse-cancellation",
            Self::RotationComposition => "rotation-composition",
            Self::Clifford => "clifford",
            Self::Pauli => "pauli",
            Self::Phase => "phase",
            Self::Controlled => "controlled",
            Self::Permutation => "permutation",
            Self::Fusion => "fusion",
            Self::Canonicalization => "canonicalization",
            Self::FaultTolerant => "fault-tolerant",
        };

        f.write_str(value)
    }
}

// =============================================================================
// Semantic equivalence
// =============================================================================

/// Semantic equivalence required for accepting a rewrite.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RuleEquivalence {
    /// Exact equality of the implemented unitary operator.
    ExactUnitary,

    /// Equality up to a global phase.
    UpToGlobalPhase,

    /// Equality of computational-basis measurement behavior.
    MeasurementEquivalent,

    /// Equality under Zamani's logical-circuit semantics.
    LogicalEquivalent,
}

impl RuleEquivalence {
    /// Returns whether this is an exact-unitary contract.
    #[must_use]
    pub const fn is_exact_unitary(self) -> bool {
        matches!(self, Self::ExactUnitary)
    }
}

// =============================================================================
// Qubit / parameter slots
// =============================================================================

/// Rule-local qubit binding slot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct QubitSlot(pub u16);

impl QubitSlot {
    /// Creates a rule-local qubit slot.
    #[must_use]
    pub const fn new(index: u16) -> Self {
        Self(index)
    }

    /// Returns the slot index.
    #[must_use]
    pub const fn index(self) -> u16 {
        self.0
    }
}

/// Rule-local parameter binding slot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ParameterSlot(pub u16);

impl ParameterSlot {
    /// Creates a rule-local parameter slot.
    #[must_use]
    pub const fn new(index: u16) -> Self {
        Self(index)
    }

    /// Returns the slot index.
    #[must_use]
    pub const fn index(self) -> u16 {
        self.0
    }
}

// =============================================================================
// Pattern parameters
// =============================================================================

/// Constraint on a matched parameter.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParameterConstraint {
    /// No additional constraint.
    Any,

    /// Parameter must be zero within the matcher's configured tolerance.
    Zero(ParameterSlot),

    /// Parameter must equal a finite constant within tolerance.
    Constant {
        slot: ParameterSlot,
        value: f64,
    },

    /// Two parameters must be equal.
    Equal {
        left: ParameterSlot,
        right: ParameterSlot,
    },

    /// Two parameters must sum to zero.
    NegationPair {
        left: ParameterSlot,
        right: ParameterSlot,
    },
}

impl ParameterConstraint {
    /// Returns whether the constraint contains only valid finite constants.
    #[must_use]
    pub fn is_valid(self) -> bool {
        match self {
            Self::Constant { value, .. } => value.is_finite(),

            Self::Any
            | Self::Zero(_)
            | Self::Equal { .. }
            | Self::NegationPair { .. } => true,
        }
    }
}

// =============================================================================
// Pattern operations
// =============================================================================

/// One canonical gate operation in a rule's left-hand-side pattern.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PatternOperation {
    /// Required canonical gate kind.
    pub gate: GateKind,

    /// Logical operands, expressed as local slots.
    pub qubits: &'static [QubitSlot],

    /// Parameters consumed by the operation.
    pub parameters: &'static [ParameterSlot],

    /// Optional constraint on the operation's parameters.
    pub parameter_constraint: Option<ParameterConstraint>,
}

impl PatternOperation {
    /// Creates a pattern operation.
    #[must_use]
    pub const fn new(
        gate: GateKind,
        qubits: &'static [QubitSlot],
        parameters: &'static [ParameterSlot],
        parameter_constraint: Option<ParameterConstraint>,
    ) -> Self {
        Self {
            gate,
            qubits,
            parameters,
            parameter_constraint,
        }
    }
}

/// Complete left-hand-side rule pattern.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RulePattern {
    /// Operations in program order.
    pub operations: &'static [PatternOperation],

    /// Number of distinct qubit slots.
    pub qubit_slots: usize,

    /// Number of distinct parameter slots.
    pub parameter_slots: usize,
}

impl RulePattern {
    /// Creates a pattern.
    #[must_use]
    pub const fn new(
        operations: &'static [PatternOperation],
        qubit_slots: usize,
        parameter_slots: usize,
    ) -> Self {
        Self {
            operations,
            qubit_slots,
            parameter_slots,
        }
    }

    /// Number of operations in the pattern.
    #[must_use]
    pub const fn len(self) -> usize {
        self.operations.len()
    }

    /// Whether the pattern is empty.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.operations.is_empty()
    }
}

// =============================================================================
// Replacement parameter expressions
// =============================================================================

/// Parameter expression constructed by a rewrite.
///
/// This is deliberately independent of the canonical IR's `Parameter` type.
/// It describes HOW the rewrite engine must construct the resulting
/// `Parameter`. The rewrite engine later lowers it into
/// `quantum::ir::Parameter`.
///
/// This prevents an unsound rule such as:
///
/// ```text
/// RX(a); RX(b) -> RX(a)
/// ```
///
/// which would lose `b`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ReplacementParameter {
    /// Reuse a matched parameter.
    Slot(ParameterSlot),

    /// Emit a finite constant.
    Constant(f64),

    /// Add two matched parameters.
    Add(ParameterSlot, ParameterSlot),

    /// Subtract the second matched parameter from the first.
    Subtract(ParameterSlot, ParameterSlot),

    /// Negate a matched parameter.
    Negate(ParameterSlot),
}

impl ReplacementParameter {
    /// Returns whether all embedded constants are finite.
    #[must_use]
    pub fn is_valid(self) -> bool {
        match self {
            Self::Constant(value) => value.is_finite(),

            Self::Slot(_)
            | Self::Add(_, _)
            | Self::Subtract(_, _)
            | Self::Negate(_) => true,
        }
    }
}

/// One operation emitted by a rule replacement.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ReplacementOperation {
    /// Canonical output gate kind.
    pub gate: GateKind,

    /// Output qubit operands.
    pub qubits: &'static [QubitSlot],

    /// Output parameter expressions.
    pub parameters: &'static [ReplacementParameter],
}

impl ReplacementOperation {
    /// Creates a replacement operation.
    #[must_use]
    pub const fn new(
        gate: GateKind,
        qubits: &'static [QubitSlot],
        parameters: &'static [ReplacementParameter],
    ) -> Self {
        Self {
            gate,
            qubits,
            parameters,
        }
    }
}

/// Complete right-hand-side rewrite.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RuleReplacement {
    /// Replacement operations in program order.
    ///
    /// An empty slice represents the identity operation.
    pub operations: &'static [ReplacementOperation],
}

impl RuleReplacement {
    /// Creates a replacement.
    #[must_use]
    pub const fn new(operations: &'static [ReplacementOperation]) -> Self {
        Self { operations }
    }

    /// Returns whether the replacement removes the complete matched region.
    #[must_use]
    pub const fn is_identity(self) -> bool {
        self.operations.is_empty()
    }
}

// =============================================================================
// Preconditions
// =============================================================================

/// Preconditions that must be established before a rule can be applied.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RulePrecondition {
    /// No special condition.
    None,

    /// Matched operations must be unitary.
    Unitary,

    /// Rewrite cannot cross a measurement/reset/barrier/control boundary.
    NoSemanticBoundary,

    /// Rewrite must not alter a classical side effect.
    NoClassicalEffect,

    /// Matched operations must use identical ordered operands.
    SameQubitOperands,

    /// Dependency analysis must prove the rewrite safe.
    DependencySafe,

    /// All relevant parameters must be concrete.
    ConstantParameters,

    /// Rewrite must preserve symbolic parameters without numerical evaluation.
    SymbolicSafe,
}

impl RulePrecondition {
    /// Returns whether dependency analysis is required.
    #[must_use]
    pub const fn requires_dependency_analysis(self) -> bool {
        matches!(self, Self::DependencySafe)
    }
}

// =============================================================================
// Postconditions
// =============================================================================

/// Conditions that the rewrite engine must establish after applying a rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RulePostcondition {
    /// Result is valid canonical Quantum IR.
    ValidIr,

    /// Semantic boundaries are preserved.
    PreservesBoundaries,

    /// Logical qubit footprint is preserved.
    PreservesQubitFootprint,

    /// Classical effects are preserved.
    PreservesClassicalEffects,

    /// Declared semantic equivalence has been established.
    SemanticallyEquivalent,
}

// =============================================================================
// Analysis requirements
// =============================================================================

/// Analysis required by a rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RuleAnalysis {
    /// No analysis beyond direct matching.
    None,

    /// Dependency analysis.
    Dependency,

    /// Commutation analysis.
    Commutation,

    /// Liveness analysis.
    Liveness,

    /// Parameter analysis.
    Parameters,

    /// Depth analysis.
    Depth,
}

impl RuleAnalysis {
    /// Returns whether this analysis can affect rewrite legality.
    #[must_use]
    pub const fn affects_legality(self) -> bool {
        matches!(
            self,
            Self::Dependency
                | Self::Commutation
                | Self::Liveness
        )
    }
}

// =============================================================================
// Safety / activation
// =============================================================================

/// Safety classification of a rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RuleSafety {
    /// Exact and local.
    ExactLocal,

    /// Exact but dependent on dependency/commutation analysis.
    ExactDependencyAware,

    /// Exact after parameter constraints are established.
    ExactParameterDependent,

    /// Valid only up to global phase.
    UpToGlobalPhase,

    /// Approximate/heuristic.
    Approximate,
}

impl RuleSafety {
    /// Returns whether this is an exact rule.
    #[must_use]
    pub const fn is_exact(self) -> bool {
        matches!(
            self,
            Self::ExactLocal
                | Self::ExactDependencyAware
                | Self::ExactParameterDependent
        )
    }
}

/// Default activation policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RuleActivation {
    /// Enabled by default.
    Default,

    /// Available only to explicitly selected profiles.
    ProfileOnly,

    /// Catalogue entry but disabled.
    Disabled,
}

impl RuleActivation {
    /// Returns whether the rule is enabled by default.
    #[must_use]
    pub const fn enabled_by_default(self) -> bool {
        matches!(self, Self::Default)
    }
}

// =============================================================================
// Cost metadata
// =============================================================================

/// Local resource delta produced by a rule.
///
/// Deltas are output minus input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RuleCost {
    /// Operation-count delta.
    pub operation_delta: isize,

    /// Single-qubit operation delta.
    pub single_qubit_delta: isize,

    /// Two-qubit operation delta.
    pub two_qubit_delta: isize,

    /// T-count delta.
    pub t_count_delta: isize,

    /// Local depth delta.
    pub local_depth_delta: isize,
}

impl RuleCost {
    /// Cost for complete identity elimination.
    #[must_use]
    pub const fn identity(input_operations: usize) -> Self {
        let amount = input_operations as isize;

        Self {
            operation_delta: -amount,
            single_qubit_delta: -amount,
            two_qubit_delta: 0,
            t_count_delta: 0,
            local_depth_delta: -amount,
        }
    }

    /// Creates explicit cost metadata.
    #[must_use]
    pub const fn new(
        operation_delta: isize,
        single_qubit_delta: isize,
        two_qubit_delta: isize,
        t_count_delta: isize,
        local_depth_delta: isize,
    ) -> Self {
        Self {
            operation_delta,
            single_qubit_delta,
            two_qubit_delta,
            t_count_delta,
            local_depth_delta,
        }
    }
}

// =============================================================================
// Rule metadata
// =============================================================================

/// Complete immutable metadata for one optimization rule.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RuleMetadata {
    /// Stable rule identifier.
    pub id: &'static str,

    /// Human-readable rule name.
    pub name: &'static str,

    /// Rule category.
    pub category: RuleCategory,

    /// Left-hand-side pattern.
    pub pattern: RulePattern,

    /// Right-hand-side replacement.
    pub replacement: RuleReplacement,

    /// Preconditions.
    pub preconditions: &'static [RulePrecondition],

    /// Postconditions.
    pub postconditions: &'static [RulePostcondition],

    /// Required analyses.
    pub analyses: &'static [RuleAnalysis],

    /// Semantic equivalence.
    pub equivalence: RuleEquivalence,

    /// Safety classification.
    pub safety: RuleSafety,

    /// Activation policy.
    pub activation: RuleActivation,

    /// Resource metadata.
    pub cost: RuleCost,
}

impl RuleMetadata {
    /// Converts the stable rule ID into the optimization error-system
    /// identifier type.
    pub fn identifier(
        self,
    ) -> Result<
        RuleIdentifier,
        crate::quantum::optimization::errors::InvalidIdentifierError,
    > {
        RuleIdentifier::new(self.id)
    }

    /// Returns whether enabled by default.
    #[must_use]
    pub const fn enabled_by_default(self) -> bool {
        self.activation.enabled_by_default()
    }

    /// Returns whether safe for an exact-unitary optimizer.
    #[must_use]
    pub const fn exact_unitary_safe(self) -> bool {
        self.safety.is_exact()
            && self.equivalence.is_exact_unitary()
    }

    /// Returns whether dependency/commutation analysis is required.
    #[must_use]
    pub fn requires_dependency_analysis(self) -> bool {
        self.preconditions
            .iter()
            .any(|condition| {
                condition.requires_dependency_analysis()
            })
            || self.analyses.iter().any(|analysis| {
                matches!(
                    analysis,
                    RuleAnalysis::Dependency
                        | RuleAnalysis::Commutation
                )
            })
    }

    /// Validates the static rule descriptor.
    pub fn validate(self) -> Result<(), RuleValidationError> {
        if self.id.trim().is_empty() {
            return Err(RuleValidationError::EmptyId);
        }

        if self.pattern.is_empty() {
            return Err(RuleValidationError::EmptyPattern);
        }

        if self.pattern.qubit_slots == 0 {
            return Err(RuleValidationError::NoQubitSlots);
        }

        for operation in self.pattern.operations {
            if let Some(constraint) = operation.parameter_constraint {
                if !constraint.is_valid() {
                    return Err(RuleValidationError::NonFiniteConstant);
                }
            }
        }

        for operation in self.replacement.operations {
            for parameter in operation.parameters {
                if !parameter.is_valid() {
                    return Err(RuleValidationError::NonFiniteConstant);
                }
            }
        }

        if self.safety == RuleSafety::UpToGlobalPhase
            && self.equivalence != RuleEquivalence::UpToGlobalPhase
        {
            return Err(RuleValidationError::SafetyEquivalenceMismatch);
        }

        if self.safety.is_exact()
            && !self.equivalence.is_exact_unitary()
        {
            return Err(
                RuleValidationError::ExactRuleNeedsExactEquivalence,
            );
        }

        Ok(())
    }
}

/// Static rule-validation error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleValidationError {
    /// Empty identifier.
    EmptyId,

    /// Empty pattern.
    EmptyPattern,

    /// No qubit slots.
    NoQubitSlots,

    /// Invalid parameter metadata.
    ParameterSlotMismatch,

    /// Non-finite constant.
    NonFiniteConstant,

    /// Safety/equivalence mismatch.
    SafetyEquivalenceMismatch,

    /// Exact rule without exact equivalence.
    ExactRuleNeedsExactEquivalence,
}

impl fmt::Display for RuleValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::EmptyId => {
                "optimization rule has an empty identifier"
            }

            Self::EmptyPattern => {
                "optimization rule has an empty pattern"
            }

            Self::NoQubitSlots => {
                "optimization rule declares no qubit slots"
            }

            Self::ParameterSlotMismatch => {
                "optimization rule has inconsistent parameter slots"
            }

            Self::NonFiniteConstant => {
                "optimization rule contains a non-finite constant"
            }

            Self::SafetyEquivalenceMismatch => {
                "rule safety does not match its equivalence policy"
            }

            Self::ExactRuleNeedsExactEquivalence => {
                "an exact rule must declare exact-unitary equivalence"
            }
        };

        f.write_str(message)
    }
}

impl std::error::Error for RuleValidationError {}

// =============================================================================
// Static slots
// =============================================================================

const Q0: QubitSlot = QubitSlot(0);
const Q1: QubitSlot = QubitSlot(1);
const Q2: QubitSlot = QubitSlot(2);

const P0: ParameterSlot = ParameterSlot(0);
const P1: ParameterSlot = ParameterSlot(1);

const Q0_ONLY: &[QubitSlot] = &[Q0];
const Q01: &[QubitSlot] = &[Q0, Q1];
const Q012: &[QubitSlot] = &[Q0, Q1, Q2];

const P0_ONLY: &[ParameterSlot] = &[P0];
const P01: &[ParameterSlot] = &[P0, P1];
const NO_PARAMETERS: &[ParameterSlot] = &[];

const RP0: &[ReplacementParameter] =
    &[ReplacementParameter::Slot(P0)];

const RP_ADD_01: &[ReplacementParameter] =
    &[ReplacementParameter::Add(P0, P1)];

const EMPTY_PRECONDITIONS: &[RulePrecondition] =
    &[];

const UNITARY_PRECONDITIONS: &[RulePrecondition] =
    &[RulePrecondition::Unitary];

const LOCAL_PRECONDITIONS: &[RulePrecondition] = &[
    RulePrecondition::Unitary,
    RulePrecondition::NoSemanticBoundary,
    RulePrecondition::SameQubitOperands,
];

const ROTATION_PRECONDITIONS: &[RulePrecondition] = &[
    RulePrecondition::Unitary,
    RulePrecondition::NoSemanticBoundary,
    RulePrecondition::SameQubitOperands,
    RulePrecondition::SymbolicSafe,
];

const STANDARD_POSTCONDITIONS: &[RulePostcondition] = &[
    RulePostcondition::ValidIr,
    RulePostcondition::PreservesBoundaries,
    RulePostcondition::PreservesQubitFootprint,
    RulePostcondition::PreservesClassicalEffects,
    RulePostcondition::SemanticallyEquivalent,
];

const NO_ANALYSIS: &[RuleAnalysis] =
    &[RuleAnalysis::None];

const PARAMETER_ANALYSIS: &[RuleAnalysis] =
    &[RuleAnalysis::Parameters];

// =============================================================================
// Pattern-operation constants
// =============================================================================

const I_Q0: PatternOperation =
    PatternOperation::new(
        GateKind::I,
        Q0_ONLY,
        NO_PARAMETERS,
        None,
    );

const X_Q0: PatternOperation =
    PatternOperation::new(
        GateKind::X,
        Q0_ONLY,
        NO_PARAMETERS,
        None,
    );

const Y_Q0: PatternOperation =
    PatternOperation::new(
        GateKind::Y,
        Q0_ONLY,
        NO_PARAMETERS,
        None,
    );

const Z_Q0: PatternOperation =
    PatternOperation::new(
        GateKind::Z,
        Q0_ONLY,
        NO_PARAMETERS,
        None,
    );

const H_Q0: PatternOperation =
    PatternOperation::new(
        GateKind::H,
        Q0_ONLY,
        NO_PARAMETERS,
        None,
    );

const S_Q0: PatternOperation =
    PatternOperation::new(
        GateKind::S,
        Q0_ONLY,
        NO_PARAMETERS,
        None,
    );

const SDG_Q0: PatternOperation =
    PatternOperation::new(
        GateKind::Sdg,
        Q0_ONLY,
        NO_PARAMETERS,
        None,
    );

const T_Q0: PatternOperation =
    PatternOperation::new(
        GateKind::T,
        Q0_ONLY,
        NO_PARAMETERS,
        None,
    );

const TDG_Q0: PatternOperation =
    PatternOperation::new(
        GateKind::Tdg,
        Q0_ONLY,
        NO_PARAMETERS,
        None,
    );

const CX_Q01: PatternOperation =
    PatternOperation::new(
        GateKind::CX,
        Q01,
        NO_PARAMETERS,
        None,
    );

const CZ_Q01: PatternOperation =
    PatternOperation::new(
        GateKind::CZ,
        Q01,
        NO_PARAMETERS,
        None,
    );

const SWAP_Q01: PatternOperation =
    PatternOperation::new(
        GateKind::SWAP,
        Q01,
        NO_PARAMETERS,
        None,
    );

const CCX_Q012: PatternOperation =
    PatternOperation::new(
        GateKind::CCX,
        Q012,
        NO_PARAMETERS,
        None,
    );

const RX_P0: PatternOperation =
    PatternOperation::new(
        GateKind::RX,
        Q0_ONLY,
        P0_ONLY,
        None,
    );

const RY_P0: PatternOperation =
    PatternOperation::new(
        GateKind::RY,
        Q0_ONLY,
        P0_ONLY,
        None,
    );

const RZ_P0: PatternOperation =
    PatternOperation::new(
        GateKind::RZ,
        Q0_ONLY,
        P0_ONLY,
        None,
    );

const PHASE_P0: PatternOperation =
    PatternOperation::new(
        GateKind::Phase,
        Q0_ONLY,
        P0_ONLY,
        None,
    );

const U1_P0: PatternOperation =
    PatternOperation::new(
        GateKind::U1,
        Q0_ONLY,
        P0_ONLY,
        None,
    );

const RX_P1: PatternOperation =
    PatternOperation::new(
        GateKind::RX,
        Q0_ONLY,
        P01,
        None,
    );

const RY_P1: PatternOperation =
    PatternOperation::new(
        GateKind::RY,
        Q0_ONLY,
        P01,
        None,
    );

const RZ_P1: PatternOperation =
    PatternOperation::new(
        GateKind::RZ,
        Q0_ONLY,
        P01,
        None,
    );

const PHASE_P1: PatternOperation =
    PatternOperation::new(
        GateKind::Phase,
        Q0_ONLY,
        P01,
        None,
    );

const U1_P1: PatternOperation =
    PatternOperation::new(
        GateKind::U1,
        Q0_ONLY,
        P01,
        None,
    );

// =============================================================================
// Replacement-operation constants
// =============================================================================

const Z_OUTPUT: ReplacementOperation =
    ReplacementOperation::new(
        GateKind::Z,
        Q0_ONLY,
        &[],
    );

const X_OUTPUT: ReplacementOperation =
    ReplacementOperation::new(
        GateKind::X,
        Q0_ONLY,
        &[],
    );

const Y_OUTPUT: ReplacementOperation =
    ReplacementOperation::new(
        GateKind::Y,
        Q0_ONLY,
        &[],
    );

const S_OUTPUT: ReplacementOperation =
    ReplacementOperation::new(
        GateKind::S,
        Q0_ONLY,
        &[],
    );

const SDG_OUTPUT: ReplacementOperation =
    ReplacementOperation::new(
        GateKind::Sdg,
        Q0_ONLY,
        &[],
    );

const RX_OUTPUT: ReplacementOperation =
    ReplacementOperation::new(
        GateKind::RX,
        Q0_ONLY,
        RP_ADD_01,
    );

const RY_OUTPUT: ReplacementOperation =
    ReplacementOperation::new(
        GateKind::RY,
        Q0_ONLY,
        RP_ADD_01,
    );

const RZ_OUTPUT: ReplacementOperation =
    ReplacementOperation::new(
        GateKind::RZ,
        Q0_ONLY,
        RP_ADD_01,
    );

const PHASE_OUTPUT: ReplacementOperation =
    ReplacementOperation::new(
        GateKind::Phase,
        Q0_ONLY,
        RP_ADD_01,
    );

const U1_OUTPUT: ReplacementOperation =
    ReplacementOperation::new(
        GateKind::U1,
        Q0_ONLY,
        RP_ADD_01,
    );

const EMPTY_REPLACEMENT: RuleReplacement =
    RuleReplacement::new(&[]);

const Z_REPLACEMENT: RuleReplacement =
    RuleReplacement::new(&[Z_OUTPUT]);

const X_REPLACEMENT: RuleReplacement =
    RuleReplacement::new(&[X_OUTPUT]);

const Y_REPLACEMENT: RuleReplacement =
    RuleReplacement::new(&[Y_OUTPUT]);

const S_REPLACEMENT: RuleReplacement =
    RuleReplacement::new(&[S_OUTPUT]);

const SDG_REPLACEMENT: RuleReplacement =
    RuleReplacement::new(&[SDG_OUTPUT]);

const RX_REPLACEMENT: RuleReplacement =
    RuleReplacement::new(&[RX_OUTPUT]);

const RY_REPLACEMENT: RuleReplacement =
    RuleReplacement::new(&[RY_OUTPUT]);

const RZ_REPLACEMENT: RuleReplacement =
    RuleReplacement::new(&[RZ_OUTPUT]);

const PHASE_REPLACEMENT: RuleReplacement =
    RuleReplacement::new(&[PHASE_OUTPUT]);

const U1_REPLACEMENT: RuleReplacement =
    RuleReplacement::new(&[U1_OUTPUT]);

// =============================================================================
// Identity / inverse patterns
// =============================================================================

const XX_PATTERN: &[PatternOperation] =
    &[X_Q0, X_Q0];

const YY_PATTERN: &[PatternOperation] =
    &[Y_Q0, Y_Q0];

const ZZ_PATTERN: &[PatternOperation] =
    &[Z_Q0, Z_Q0];

const HH_PATTERN: &[PatternOperation] =
    &[H_Q0, H_Q0];

const S_SDG_PATTERN: &[PatternOperation] =
    &[S_Q0, SDG_Q0];

const SDG_S_PATTERN: &[PatternOperation] =
    &[SDG_Q0, S_Q0];

const T_TDG_PATTERN: &[PatternOperation] =
    &[T_Q0, TDG_Q0];

const TDG_T_PATTERN: &[PatternOperation] =
    &[TDG_Q0, T_Q0];

const CX_CX_PATTERN: &[PatternOperation] =
    &[CX_Q01, CX_Q01];

const CZ_CZ_PATTERN: &[PatternOperation] =
    &[CZ_Q01, CZ_Q01];

const SWAP_SWAP_PATTERN: &[PatternOperation] =
    &[SWAP_Q01, SWAP_Q01];

const CCX_CCX_PATTERN: &[PatternOperation] =
    &[CCX_Q012, CCX_Q012];

// =============================================================================
// Identity rule
// =============================================================================

const RULE_IDENTITY_I: RuleMetadata = RuleMetadata {
    id: "quantum.identity.i",
    name: "remove identity",
    category: RuleCategory::Identity,
    pattern: RulePattern::new(&[I_Q0], 1, 0),
    replacement: EMPTY_REPLACEMENT,
    preconditions: UNITARY_PRECONDITIONS,
    postconditions: STANDARD_POSTCONDITIONS,
    analyses: NO_ANALYSIS,
    equivalence: RuleEquivalence::ExactUnitary,
    safety: RuleSafety::ExactLocal,
    activation: RuleActivation::Default,
    cost: RuleCost::new(-1, -1, 0, 0, -1),
};

// =============================================================================
// Inverse-cancellation rules
// =============================================================================

const RULE_CANCEL_X: RuleMetadata = RuleMetadata {
    id: "quantum.cancel.x.x",
    name: "cancel X X",
    category: RuleCategory::InverseCancellation,
    pattern: RulePattern::new(XX_PATTERN, 1, 0),
    replacement: EMPTY_REPLACEMENT,
    preconditions: LOCAL_PRECONDITIONS,
    postconditions: STANDARD_POSTCONDITIONS,
    analyses: NO_ANALYSIS,
    equivalence: RuleEquivalence::ExactUnitary,
    safety: RuleSafety::ExactLocal,
    activation: RuleActivation::Default,
    cost: RuleCost::new(-2, -2, 0, 0, -2),
};

const RULE_CANCEL_Y: RuleMetadata = RuleMetadata {
    id: "quantum.cancel.y.y",
    name: "cancel Y Y",
    category: RuleCategory::InverseCancellation,
    pattern: RulePattern::new(YY_PATTERN, 1, 0),
    replacement: EMPTY_REPLACEMENT,
    preconditions: LOCAL_PRECONDITIONS,
    postconditions: STANDARD_POSTCONDITIONS,
    analyses: NO_ANALYSIS,
    equivalence: RuleEquivalence::ExactUnitary,
    safety: RuleSafety::ExactLocal,
    activation: RuleActivation::Default,
    cost: RuleCost::new(-2, -2, 0, 0, -2),
};

const RULE_CANCEL_Z: RuleMetadata = RuleMetadata {
    id: "quantum.cancel.z.z",
    name: "cancel Z Z",
    category: RuleCategory::InverseCancellation,
    pattern: RulePattern::new(ZZ_PATTERN, 1, 0),
    replacement: EMPTY_REPLACEMENT,
    preconditions: LOCAL_PRECONDITIONS,
    postconditions: STANDARD_POSTCONDITIONS,
    analyses: NO_ANALYSIS,
    equivalence: RuleEquivalence::ExactUnitary,
    safety: RuleSafety::ExactLocal,
    activation: RuleActivation::Default,
    cost: RuleCost::new(-2, -2, 0, 0, -2),
};

const RULE_CANCEL_H: RuleMetadata = RuleMetadata {
    id: "quantum.cancel.h.h",
    name: "cancel H H",
    category: RuleCategory::InverseCancellation,
    pattern: RulePattern::new(HH_PATTERN, 1, 0),
    replacement: EMPTY_REPLACEMENT,
    preconditions: LOCAL_PRECONDITIONS,
    postconditions: STANDARD_POSTCONDITIONS,
    analyses: NO_ANALYSIS,
    equivalence: RuleEquivalence::ExactUnitary,
    safety: RuleSafety::ExactLocal,
    activation: RuleActivation::Default,
    cost: RuleCost::new(-2, -2, 0, 0, -2),
};

const RULE_CANCEL_S_SDG: RuleMetadata = RuleMetadata {
    id: "quantum.cancel.s.sdg",
    name: "cancel S Sdg",
    category: RuleCategory::InverseCancellation,
    pattern: RulePattern::new(S_SDG_PATTERN, 1, 0),
    replacement: EMPTY_REPLACEMENT,
    preconditions: LOCAL_PRECONDITIONS,
    postconditions: STANDARD_POSTCONDITIONS,
    analyses: NO_ANALYSIS,
    equivalence: RuleEquivalence::ExactUnitary,
    safety: RuleSafety::ExactLocal,
    activation: RuleActivation::Default,
    cost: RuleCost::new(-2, -2, 0, 0, -2),
};

const RULE_CANCEL_SDG_S: RuleMetadata = RuleMetadata {
    id: "quantum.cancel.sdg.s",
    name: "cancel Sdg S",
    category: RuleCategory::InverseCancellation,
    pattern: RulePattern::new(SDG_S_PATTERN, 1, 0),
    replacement: EMPTY_REPLACEMENT,
    preconditions: LOCAL_PRECONDITIONS,
    postconditions: STANDARD_POSTCONDITIONS,
    analyses: NO_ANALYSIS,
    equivalence: RuleEquivalence::ExactUnitary,
    safety: RuleSafety::ExactLocal,
    activation: RuleActivation::Default,
    cost: RuleCost::new(-2, -2, 0, 0, -2),
};

const RULE_CANCEL_T_TDG: RuleMetadata = RuleMetadata {
    id: "quantum.cancel.t.tdg",
    name: "cancel T Tdg",
    category: RuleCategory::InverseCancellation,
    pattern: RulePattern::new(T_TDG_PATTERN, 1, 0),
    replacement: EMPTY_REPLACEMENT,
    preconditions: LOCAL_PRECONDITIONS,
    postconditions: STANDARD_POSTCONDITIONS,
    analyses: NO_ANALYSIS,
    equivalence: RuleEquivalence::ExactUnitary,
    safety: RuleSafety::ExactLocal,
    activation: RuleActivation::Default,
    cost: RuleCost::new(-2, -2, 0, -2, -2),
};

const RULE_CANCEL_TDG_T: RuleMetadata = RuleMetadata {
    id: "quantum.cancel.tdg.t",
    name: "cancel Tdg T",
    category: RuleCategory::InverseCancellation,
    pattern: RulePattern::new(TDG_T_PATTERN, 1, 0),
    replacement: EMPTY_REPLACEMENT,
    preconditions: LOCAL_PRECONDITIONS,
    postconditions: STANDARD_POSTCONDITIONS,
    analyses: NO_ANALYSIS,
    equivalence: RuleEquivalence::ExactUnitary,
    safety: RuleSafety::ExactLocal,
    activation: RuleActivation::Default,
    cost: RuleCost::new(-2, -2, 0, -2, -2),
};

const RULE_CANCEL_CX: RuleMetadata = RuleMetadata {
    id: "quantum.cancel.cx.cx",
    name: "cancel CX CX",
    category: RuleCategory::InverseCancellation,
    pattern: RulePattern::new(CX_CX_PATTERN, 2, 0),
    replacement: EMPTY_REPLACEMENT,
    preconditions: LOCAL_PRECONDITIONS,
    postconditions: STANDARD_POSTCONDITIONS,
    analyses: NO_ANALYSIS,
    equivalence: RuleEquivalence::ExactUnitary,
    safety: RuleSafety::ExactLocal,
    activation: RuleActivation::Default,
    cost: RuleCost::new(-2, 0, -2, 0, -2),
};

const RULE_CANCEL_CZ: RuleMetadata = RuleMetadata {
    id: "quantum.cancel.cz.cz",
    name: "cancel CZ CZ",
    category: RuleCategory::InverseCancellation,
    pattern: RulePattern::new(CZ_CZ_PATTERN, 2, 0),
    replacement: EMPTY_REPLACEMENT,
    preconditions: LOCAL_PRECONDITIONS,
    postconditions: STANDARD_POSTCONDITIONS,
    analyses: NO_ANALYSIS,
    equivalence: RuleEquivalence::ExactUnitary,
    safety: RuleSafety::ExactLocal,
    activation: RuleActivation::Default,
    cost: RuleCost::new(-2, 0, -2, 0, -2),
};

const RULE_CANCEL_SWAP: RuleMetadata = RuleMetadata {
    id: "quantum.cancel.swap.swap",
    name: "cancel SWAP SWAP",
    category: RuleCategory::Permutation,
    pattern: RulePattern::new(SWAP_SWAP_PATTERN, 2, 0),
    replacement: EMPTY_REPLACEMENT,
    preconditions: LOCAL_PRECONDITIONS,
    postconditions: STANDARD_POSTCONDITIONS,
    analyses: NO_ANALYSIS,
    equivalence: RuleEquivalence::ExactUnitary,
    safety: RuleSafety::ExactLocal,
    activation: RuleActivation::Default,
    cost: RuleCost::new(-2, 0, -2, 0, -2),
};

const RULE_CANCEL_CCX: RuleMetadata = RuleMetadata {
    id: "quantum.cancel.ccx.ccx",
    name: "cancel CCX CCX",
    category: RuleCategory::InverseCancellation,
    pattern: RulePattern::new(CCX_CCX_PATTERN, 3, 0),
    replacement: EMPTY_REPLACEMENT,
    preconditions: LOCAL_PRECONDITIONS,
    postconditions: STANDARD_POSTCONDITIONS,
    analyses: NO_ANALYSIS,
    equivalence: RuleEquivalence::ExactUnitary,
    safety: RuleSafety::ExactLocal,
    activation: RuleActivation::Default,
    cost: RuleCost::new(-2, 0, -2, 0, -2),
};

// =============================================================================
// Phase hierarchy
// =============================================================================

const SS_PATTERN: &[PatternOperation] =
    &[S_Q0, S_Q0];

const SDGSDG_PATTERN: &[PatternOperation] =
    &[SDG_Q0, SDG_Q0];

const TT_PATTERN: &[PatternOperation] =
    &[T_Q0, T_Q0];

const TDGTDG_PATTERN: &[PatternOperation] =
    &[TDG_Q0, TDG_Q0];

const T4_PATTERN: &[PatternOperation] =
    &[T_Q0, T_Q0, T_Q0, T_Q0];

const TDG4_PATTERN: &[PatternOperation] =
    &[TDG_Q0, TDG_Q0, TDG_Q0, TDG_Q0];

const T8_PATTERN: &[PatternOperation] = &[
    T_Q0,
    T_Q0,
    T_Q0,
    T_Q0,
    T_Q0,
    T_Q0,
    T_Q0,
    T_Q0,
];

const TDG8_PATTERN: &[PatternOperation] = &[
    TDG_Q0,
    TDG_Q0,
    TDG_Q0,
    TDG_Q0,
    TDG_Q0,
    TDG_Q0,
    TDG_Q0,
    TDG_Q0,
];

const RULE_S_S_TO_Z: RuleMetadata = RuleMetadata {
    id: "quantum.phase.s.s-to-z",
    name: "S S to Z",
    category: RuleCategory::Phase,
    pattern: RulePattern::new(SS_PATTERN, 1, 0),
    replacement: Z_REPLACEMENT,
    preconditions: LOCAL_PRECONDITIONS,
    postconditions: STANDARD_POSTCONDITIONS,
    analyses: NO_ANALYSIS,
    equivalence: RuleEquivalence::ExactUnitary,
    safety: RuleSafety::ExactLocal,
    activation: RuleActivation::Default,
    cost: RuleCost::new(-1, -1, 0, 0, -1),
};

const RULE_SDG_SDG_TO_Z: RuleMetadata = RuleMetadata {
    id: "quantum.phase.sdg.sdg-to-z",
    name: "Sdg Sdg to Z",
    category: RuleCategory::Phase,
    pattern: RulePattern::new(SDGSDG_PATTERN, 1, 0),
    replacement: Z_REPLACEMENT,
    preconditions: LOCAL_PRECONDITIONS,
    postconditions: STANDARD_POSTCONDITIONS,
    analyses: NO_ANALYSIS,
    equivalence: RuleEquivalence::ExactUnitary,
    safety: RuleSafety::ExactLocal,
    activation: RuleActivation::Default,
    cost: RuleCost::new(-1, -1, 0, 0, -1),
};

const RULE_T_T_TO_S: RuleMetadata = RuleMetadata {
    id: "quantum.phase.t.t-to-s",
    name: "T T to S",
    category: RuleCategory::FaultTolerant,
    pattern: RulePattern::new(TT_PATTERN, 1, 0),
    replacement: S_REPLACEMENT,
    preconditions: LOCAL_PRECONDITIONS,
    postconditions: STANDARD_POSTCONDITIONS,
    analyses: NO_ANALYSIS,
    equivalence: RuleEquivalence::ExactUnitary,
    safety: RuleSafety::ExactLocal,
    activation: RuleActivation::Default,
    cost: RuleCost::new(-1, -1, 0, -2, -1),
};

const RULE_TDG_TDG_TO_SDG: RuleMetadata = RuleMetadata {
    id: "quantum.phase.tdg.tdg-to-sdg",
    name: "Tdg Tdg to Sdg",
    category: RuleCategory::FaultTolerant,
    pattern: RulePattern::new(TDGTDG_PATTERN, 1, 0),
    replacement: SDG_REPLACEMENT,
    preconditions: LOCAL_PRECONDITIONS,
    postconditions: STANDARD_POSTCONDITIONS,
    analyses: NO_ANALYSIS,
    equivalence: RuleEquivalence::ExactUnitary,
    safety: RuleSafety::ExactLocal,
    activation: RuleActivation::Default,
    cost: RuleCost::new(-1, -1, 0, -2, -1),
};

const RULE_T4_TO_Z: RuleMetadata = RuleMetadata {
    id: "quantum.phase.t4-to-z",
    name: "T four times to Z",
    category: RuleCategory::FaultTolerant,
    pattern: RulePattern::new(T4_PATTERN, 1, 0),
    replacement: Z_REPLACEMENT,
    preconditions: LOCAL_PRECONDITIONS,
    postconditions: STANDARD_POSTCONDITIONS,
    analyses: NO_ANALYSIS,
    equivalence: RuleEquivalence::ExactUnitary,
    safety: RuleSafety::ExactLocal,
    activation: RuleActivation::Default,
    cost: RuleCost::new(-3, -3, 0, -4, -3),
};

const RULE_TDG4_TO_Z: RuleMetadata = RuleMetadata {
    id: "quantum.phase.tdg4-to-z",
    name: "Tdg four times to Z",
    category: RuleCategory::FaultTolerant,
    pattern: RulePattern::new(TDG4_PATTERN, 1, 0),
    replacement: Z_REPLACEMENT,
    preconditions: LOCAL_PRECONDITIONS,
    postconditions: STANDARD_POSTCONDITIONS,
    analyses: NO_ANALYSIS,
    equivalence: RuleEquivalence::ExactUnitary,
    safety: RuleSafety::ExactLocal,
    activation: RuleActivation::Default,
    cost: RuleCost::new(-3, -3, 0, -4, -3),
};

const RULE_T8_TO_IDENTITY: RuleMetadata = RuleMetadata {
    id: "quantum.phase.t8-to-identity",
    name: "T eight times to identity",
    category: RuleCategory::FaultTolerant,
    pattern: RulePattern::new(T8_PATTERN, 1, 0),
    replacement: EMPTY_REPLACEMENT,
    preconditions: LOCAL_PRECONDITIONS,
    postconditions: STANDARD_POSTCONDITIONS,
    analyses: NO_ANALYSIS,
    equivalence: RuleEquivalence::ExactUnitary,
    safety: RuleSafety::ExactLocal,
    activation: RuleActivation::Default,
    cost: RuleCost::new(-8, -8, 0, -8, -8),
};

const RULE_TDG8_TO_IDENTITY: RuleMetadata = RuleMetadata {
    id: "quantum.phase.tdg8-to-identity",
    name: "Tdg eight times to identity",
    category: RuleCategory::FaultTolerant,
    pattern: RulePattern::new(TDG8_PATTERN, 1, 0),
    replacement: EMPTY_REPLACEMENT,
    preconditions: LOCAL_PRECONDITIONS,
    postconditions: STANDARD_POSTCONDITIONS,
    analyses: NO_ANALYSIS,
    equivalence: RuleEquivalence::ExactUnitary,
    safety: RuleSafety::ExactLocal,
    activation: RuleActivation::Default,
    cost: RuleCost::new(-8, -8, 0, -8, -8),
};

// =============================================================================
// Clifford templates
// =============================================================================

const HXH_PATTERN: &[PatternOperation] =
    &[H_Q0, X_Q0, H_Q0];

const HZH_PATTERN: &[PatternOperation] =
    &[H_Q0, Z_Q0, H_Q0];

const S_X_SDG_PATTERN: &[PatternOperation] =
    &[S_Q0, X_Q0, SDG_Q0];

const RULE_HXH_TO_Z: RuleMetadata = RuleMetadata {
    id: "quantum.clifford.h-x-h-to-z",
    name: "H X H to Z",
    category: RuleCategory::Clifford,
    pattern: RulePattern::new(HXH_PATTERN, 1, 0),
    replacement: Z_REPLACEMENT,
    preconditions: LOCAL_PRECONDITIONS,
    postconditions: STANDARD_POSTCONDITIONS,
    analyses: NO_ANALYSIS,
    equivalence: RuleEquivalence::ExactUnitary,
    safety: RuleSafety::ExactLocal,
    activation: RuleActivation::Default,
    cost: RuleCost::new(-2, -2, 0, 0, -2),
};

const RULE_HZH_TO_X: RuleMetadata = RuleMetadata {
    id: "quantum.clifford.h-z-h-to-x",
    name: "H Z H to X",
    category: RuleCategory::Clifford,
    pattern: RulePattern::new(HZH_PATTERN, 1, 0),
    replacement: X_REPLACEMENT,
    preconditions: LOCAL_PRECONDITIONS,
    postconditions: STANDARD_POSTCONDITIONS,
    analyses: NO_ANALYSIS,
    equivalence: RuleEquivalence::ExactUnitary,
    safety: RuleSafety::ExactLocal,
    activation: RuleActivation::Default,
    cost: RuleCost::new(-2, -2, 0, 0, -2),
};

const RULE_S_X_SDG_TO_Y: RuleMetadata = RuleMetadata {
    id: "quantum.clifford.s-x-sdg-to-y",
    name: "S X Sdg to Y",
    category: RuleCategory::Clifford,
    pattern: RulePattern::new(S_X_SDG_PATTERN, 1, 0),
    replacement: Y_REPLACEMENT,
    preconditions: LOCAL_PRECONDITIONS,
    postconditions: STANDARD_POSTCONDITIONS,
    analyses: NO_ANALYSIS,
    equivalence: RuleEquivalence::ExactUnitary,
    safety: RuleSafety::ExactLocal,
    activation: RuleActivation::Default,
    cost: RuleCost::new(-2, -2, 0, 0, -2),
};

// =============================================================================
// Parameter-composition patterns
// =============================================================================

const RX_COMBINE_PATTERN: &[PatternOperation] = &[
    RX_P0,
    RX_P1,
];

const RY_COMBINE_PATTERN: &[PatternOperation] = &[
    RY_P0,
    RY_P1,
];

const RZ_COMBINE_PATTERN: &[PatternOperation] = &[
    RZ_P0,
    RZ_P1,
];

const PHASE_COMBINE_PATTERN: &[PatternOperation] = &[
    PHASE_P0,
    PHASE_P1,
];

const U1_COMBINE_PATTERN: &[PatternOperation] = &[
    U1_P0,
    U1_P1,
];

const RX_NEG_PATTERN: &[PatternOperation] = &[
    PatternOperation::new(
        GateKind::RX,
        Q0_ONLY,
        P0_ONLY,
        None,
    ),
    PatternOperation::new(
        GateKind::RX,
        Q0_ONLY,
        P01,
        Some(
            ParameterConstraint::NegationPair {
                left: P0,
                right: P1,
            },
        ),
    ),
];

const RY_NEG_PATTERN: &[PatternOperation] = &[
    PatternOperation::new(
        GateKind::RY,
        Q0_ONLY,
        P0_ONLY,
        None,
    ),
    PatternOperation::new(
        GateKind::RY,
        Q0_ONLY,
        P01,
        Some(
            ParameterConstraint::NegationPair {
                left: P0,
                right: P1,
            },
        ),
    ),
];

const RZ_NEG_PATTERN: &[PatternOperation] = &[
    PatternOperation::new(
        GateKind::RZ,
        Q0_ONLY,
        P0_ONLY,
        None,
    ),
    PatternOperation::new(
        GateKind::RZ,
        Q0_ONLY,
        P01,
        Some(
            ParameterConstraint::NegationPair {
                left: P0,
                right: P1,
            },
        ),
    ),
];

const RULE_RX_COMBINE: RuleMetadata = RuleMetadata {
    id: "quantum.rotation.rx-plus-rx",
    name: "combine RX rotations",
    category: RuleCategory::RotationComposition,
    pattern: RulePattern::new(
        RX_COMBINE_PATTERN,
        1,
        2,
    ),
    replacement: RX_REPLACEMENT,
    preconditions: ROTATION_PRECONDITIONS,
    postconditions: STANDARD_POSTCONDITIONS,
    analyses: PARAMETER_ANALYSIS,
    equivalence: RuleEquivalence::ExactUnitary,
    safety: RuleSafety::ExactParameterDependent,
    activation: RuleActivation::Default,
    cost: RuleCost::new(-1, -1, 0, 0, -1),
};

const RULE_RY_COMBINE: RuleMetadata = RuleMetadata {
    id: "quantum.rotation.ry-plus-ry",
    name: "combine RY rotations",
    category: RuleCategory::RotationComposition,
    pattern: RulePattern::new(
        RY_COMBINE_PATTERN,
        1,
        2,
    ),
    replacement: RY_REPLACEMENT,
    preconditions: ROTATION_PRECONDITIONS,
    postconditions: STANDARD_POSTCONDITIONS,
    analyses: PARAMETER_ANALYSIS,
    equivalence: RuleEquivalence::ExactUnitary,
    safety: RuleSafety::ExactParameterDependent,
    activation: RuleActivation::Default,
    cost: RuleCost::new(-1, -1, 0, 0, -1),
};

const RULE_RZ_COMBINE: RuleMetadata = RuleMetadata {
    id: "quantum.rotation.rz-plus-rz",
    name: "combine RZ rotations",
    category: RuleCategory::RotationComposition,
    pattern: RulePattern::new(
        RZ_COMBINE_PATTERN,
        1,
        2,
    ),
    replacement: RZ_REPLACEMENT,
    preconditions: ROTATION_PRECONDITIONS,
    postconditions: STANDARD_POSTCONDITIONS,
    analyses: PARAMETER_ANALYSIS,
    equivalence: RuleEquivalence::ExactUnitary,
    safety: RuleSafety::ExactParameterDependent,
    activation: RuleActivation::Default,
    cost: RuleCost::new(-1, -1, 0, 0, -1),
};

const RULE_PHASE_COMBINE: RuleMetadata = RuleMetadata {
    id: "quantum.rotation.phase-plus-phase",
    name: "combine phase rotations",
    category: RuleCategory::RotationComposition,
    pattern: RulePattern::new(
        PHASE_COMBINE_PATTERN,
        1,
        2,
    ),
    replacement: PHASE_REPLACEMENT,
    preconditions: ROTATION_PRECONDITIONS,
    postconditions: STANDARD_POSTCONDITIONS,
    analyses: PARAMETER_ANALYSIS,
    equivalence: RuleEquivalence::ExactUnitary,
    safety: RuleSafety::ExactParameterDependent,
    activation: RuleActivation::Default,
    cost: RuleCost::new(-1, -1, 0, 0, -1),
};

const RULE_U1_COMBINE: RuleMetadata = RuleMetadata {
    id: "quantum.rotation.u1-plus-u1",
    name: "combine U1 rotations",
    category: RuleCategory::RotationComposition,
    pattern: RulePattern::new(
        U1_COMBINE_PATTERN,
        1,
        2,
    ),
    replacement: U1_REPLACEMENT,
    preconditions: ROTATION_PRECONDITIONS,
    postconditions: STANDARD_POSTCONDITIONS,
    analyses: PARAMETER_ANALYSIS,
    equivalence: RuleEquivalence::ExactUnitary,
    safety: RuleSafety::ExactParameterDependent,
    activation: RuleActivation::Default,
    cost: RuleCost::new(-1, -1, 0, 0, -1),
};

const RULE_RX_CANCEL: RuleMetadata = RuleMetadata {
    id: "quantum.rotation.rx-theta-minus-theta",
    name: "cancel opposite RX rotations",
    category: RuleCategory::RotationComposition,
    pattern: RulePattern::new(
        RX_NEG_PATTERN,
        1,
        2,
    ),
    replacement: EMPTY_REPLACEMENT,
    preconditions: ROTATION_PRECONDITIONS,
    postconditions: STANDARD_POSTCONDITIONS,
    analyses: PARAMETER_ANALYSIS,
    equivalence: RuleEquivalence::ExactUnitary,
    safety: RuleSafety::ExactParameterDependent,
    activation: RuleActivation::Default,
    cost: RuleCost::new(-2, -2, 0, 0, -2),
};

const RULE_RY_CANCEL: RuleMetadata = RuleMetadata {
    id: "quantum.rotation.ry-theta-minus-theta",
    name: "cancel opposite RY rotations",
    category: RuleCategory::RotationComposition,
    pattern: RulePattern::new(
        RY_NEG_PATTERN,
        1,
        2,
    ),
    replacement: EMPTY_REPLACEMENT,
    preconditions: ROTATION_PRECONDITIONS,
    postconditions: STANDARD_POSTCONDITIONS,
    analyses: PARAMETER_ANALYSIS,
    equivalence: RuleEquivalence::ExactUnitary,
    safety: RuleSafety::ExactParameterDependent,
    activation: RuleActivation::Default,
    cost: RuleCost::new(-2, -2, 0, 0, -2),
};

const RULE_RZ_CANCEL: RuleMetadata = RuleMetadata {
    id: "quantum.rotation.rz-theta-minus-theta",
    name: "cancel opposite RZ rotations",
    category: RuleCategory::RotationComposition,
    pattern: RulePattern::new(
        RZ_NEG_PATTERN,
        1,
        2,
    ),
    replacement: EMPTY_REPLACEMENT,
    preconditions: ROTATION_PRECONDITIONS,
    postconditions: STANDARD_POSTCONDITIONS,
    analyses: PARAMETER_ANALYSIS,
    equivalence: RuleEquivalence::ExactUnitary,
    safety: RuleSafety::ExactParameterDependent,
    activation: RuleActivation::Default,
    cost: RuleCost::new(-2, -2, 0, 0, -2),
};

// =============================================================================
// Built-in catalogue
// =============================================================================

/// Immutable built-in rule catalogue.
///
/// Ordering is deterministic and stable. More specific reductions appear
/// before generic parameter-composition rules.
const BUILTIN_RULES: &[RuleMetadata] = &[
    // Identity.
    RULE_IDENTITY_I,

    // Inverse cancellation.
    RULE_CANCEL_X,
    RULE_CANCEL_Y,
    RULE_CANCEL_Z,
    RULE_CANCEL_H,
    RULE_CANCEL_S_SDG,
    RULE_CANCEL_SDG_S,
    RULE_CANCEL_T_TDG,
    RULE_CANCEL_TDG_T,
    RULE_CANCEL_CX,
    RULE_CANCEL_CZ,
    RULE_CANCEL_SWAP,
    RULE_CANCEL_CCX,

    // Phase hierarchy.
    RULE_S_S_TO_Z,
    RULE_SDG_SDG_TO_Z,
    RULE_T_T_TO_S,
    RULE_TDG_TDG_TO_SDG,
    RULE_T4_TO_Z,
    RULE_TDG4_TO_Z,
    RULE_T8_TO_IDENTITY,
    RULE_TDG8_TO_IDENTITY,

    // Clifford.
    RULE_HXH_TO_Z,
    RULE_HZH_TO_X,
    RULE_S_X_SDG_TO_Y,

    // Parameterized rotations.
    RULE_RX_CANCEL,
    RULE_RY_CANCEL,
    RULE_RZ_CANCEL,
    RULE_RX_COMBINE,
    RULE_RY_COMBINE,
    RULE_RZ_COMBINE,
    RULE_PHASE_COMBINE,
    RULE_U1_COMBINE,
];

/// Returns all built-in rules.
#[must_use]
pub const fn builtin_rules() -> &'static [RuleMetadata] {
    BUILTIN_RULES
}

/// Returns the number of built-in rules.
#[must_use]
pub const fn builtin_rule_count() -> usize {
    BUILTIN_RULES.len()
}

/// Finds a rule by its stable identifier.
#[must_use]
pub fn find_rule(id: &str) -> Option<&'static RuleMetadata> {
    BUILTIN_RULES
        .iter()
        .find(|rule| rule.id == id)
}

/// Returns rules whose patterns contain a given canonical gate kind.
///
/// This is intentionally an iterator over immutable static data and performs
/// no allocation.
pub fn rules_for_gate(
    gate: GateKind,
) -> impl Iterator<Item = &'static RuleMetadata> {
    BUILTIN_RULES.iter().filter(move |rule| {
        rule.pattern
            .operations
            .iter()
            .any(|operation| operation.gate == gate)
    })
}

/// Returns the default-enabled exact rule set.
///
/// The catalogue is static; the returned vector is merely a convenience for
/// callers that need ownership. High-performance code should iterate over
/// [`builtin_rules`] directly to avoid allocation.
#[must_use]
pub fn default_rules() -> Vec<RuleMetadata> {
    BUILTIN_RULES
        .iter()
        .copied()
        .filter(|rule| rule.enabled_by_default())
        .collect()
}

/// Validates every built-in rule.
///
/// This should be used by tests and CI and may also be called by an optimizer
/// initialization/self-check path.
pub fn validate_builtin_rules() -> Result<(), RuleValidationError> {
    for rule in BUILTIN_RULES {
        rule.validate()?;
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
    fn catalogue_is_non_empty() {
        assert!(!builtin_rules().is_empty());
    }

    #[test]
    fn catalogue_is_valid() {
        validate_builtin_rules()
            .expect("all built-in rules must be valid");
    }

    #[test]
    fn identifiers_are_unique() {
        for (index, left) in builtin_rules().iter().enumerate() {
            for right in builtin_rules().iter().skip(index + 1) {
                assert_ne!(left.id, right.id);
            }
        }
    }

    #[test]
    fn exact_rules_use_exact_unitary_equivalence() {
        for rule in builtin_rules() {
            if rule.safety.is_exact() {
                assert_eq!(
                    rule.equivalence,
                    RuleEquivalence::ExactUnitary
                );
            }
        }
    }

    #[test]
    fn default_rules_are_exact() {
        for rule in default_rules() {
            assert!(rule.exact_unitary_safe());
        }
    }

    #[test]
    fn cx_cancellation_exists() {
        let rule = find_rule("quantum.cancel.cx.cx")
            .expect("CX cancellation rule must exist");

        assert_eq!(
            rule.category,
            RuleCategory::InverseCancellation
        );

        assert_eq!(rule.pattern.len(), 2);
        assert!(rule.replacement.is_identity());
    }

    #[test]
    fn t8_rule_removes_eight_t_gates() {
        let rule = find_rule(
            "quantum.phase.t8-to-identity",
        )
        .expect("T8 rule must exist");

        assert_eq!(rule.cost.t_count_delta, -8);
        assert!(rule.replacement.is_identity());
    }

    #[test]
    fn t4_rule_reduces_to_z() {
        let rule = find_rule(
            "quantum.phase.t4-to-z",
        )
        .expect("T4 rule must exist");

        assert_eq!(
            rule.replacement.operations.len(),
            1
        );

        assert_eq!(
            rule.replacement.operations[0].gate,
            GateKind::Z
        );
    }

    #[test]
    fn rotation_rule_preserves_both_parameters() {
        let rule = find_rule(
            "quantum.rotation.rx-plus-rx",
        )
        .expect("RX composition rule must exist");

        let output = &rule.replacement.operations[0];

        assert_eq!(
            output.parameters,
            &[
                ReplacementParameter::Add(P0, P1)
            ]
        );
    }

    #[test]
    fn opposite_rotation_rule_is_symbolic() {
        let rule = find_rule(
            "quantum.rotation.rz-theta-minus-theta",
        )
        .expect("RZ cancellation rule must exist");

        assert!(
            rule.preconditions
                .contains(
                    &RulePrecondition::SymbolicSafe
                )
        );
    }

    #[test]
    fn gate_index_finds_rotation_rules() {
        let rules: Vec<_> =
            rules_for_gate(GateKind::RZ).collect();

        assert!(
            rules.iter().any(|rule| {
                rule.id == "quantum.rotation.rz-plus-rz"
            })
        );

        assert!(
            rules.iter().any(|rule| {
                rule.id
                    == "quantum.rotation.rz-theta-minus-theta"
            })
        );
    }

    #[test]
    fn identifiers_match_error_contract() {
        for rule in builtin_rules() {
            rule.identifier()
                .expect("built-in identifiers must be valid");
        }
    }
}