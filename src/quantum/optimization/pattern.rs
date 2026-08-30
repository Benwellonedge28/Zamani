//! Zamani Quantum Optimization — Pattern Infrastructure
//!
//! Production pattern infrastructure for quantum-circuit optimization.
//!
//! # Architectural position
//!
//! ```text
//!                         quantum::ir
//!                             │
//!                             ▼
//!                 optimization::operation
//!                             │
//!                             ▼
//!                  optimization::rules
//!                             │
//!                             ▼
//!                 optimization::pattern
//!                             │
//!                    ┌────────┴────────┐
//!                    ▼                 ▼
//!                 matcher          rewrite
//!                    │                 │
//!                    └────────┬────────┘
//!                             ▼
//!                         pipeline
//! ```
//!
//! This module provides the production abstraction around the canonical rule
//! pattern contract defined by [`crate::quantum::optimization::rules`].
//!
//! # Critical architectural rule
//!
//! This file MUST NOT define another quantum operation representation.
//!
//! The canonical operation remains:
//!
//! ```text
//! crate::quantum::ir::Gate
//! ```
//!
//! Rule-level pattern descriptions remain:
//!
//! ```text
//! crate::quantum::optimization::rules::RulePattern
//! ```
//!
//! This module adds:
//!
//! - structural validation;
//! - deterministic pattern identity;
//! - deterministic fingerprints;
//! - compiled pattern metadata;
//! - gate-kind indexing;
//! - slot-use metadata;
//! - parameter-use metadata;
//! - pattern-size accounting;
//! - matcher-facing read-only accessors;
//! - bounded compilation;
//! - validation of rule-local invariants;
//! - stable diagnostics;
//! - resource-aware construction;
//! - zero-global-state operation.
//!
//! It does NOT:
//!
//! - mutate circuits;
//! - rewrite circuits;
//! - perform routing;
//! - perform scheduling;
//! - execute quantum programs;
//! - communicate with hardware;
//! - define another Quantum IR;
//! - perform semantic equivalence checking;
//! - perform circuit-wide pattern matching.
//!
//! Those responsibilities belong to other optimization subsystems.
//!
//! # Integration contract
//!
//! `rules.rs` remains the source of truth for the declarative rule pattern:
//!
//! ```text
//! RulePattern
//!   ├── PatternOperation
//!   ├── QubitSlot
//!   └── ParameterSlot
//! ```
//!
//! `pattern.rs` converts that declarative contract into a validated,
//! matcher-friendly representation.
//!
//! Future `matcher.rs` should consume [`CompiledPattern`] without needing to
//! modify this file.
//!
//! Future `rewrite.rs` should consume the same pattern identity and metadata
//! when recording provenance.
//!
//! Future `registry.rs` can validate and index patterns using the APIs here.
//!
//! Future `verification/*` can use the stable fingerprint and structural
//! metadata when reporting which pattern was applied.
//!
//! Future `pipeline.rs` can use pattern resource estimates before enabling
//! expensive matching strategies.
//!
//! # Scaling contract
//!
//! Pattern infrastructure must scale from:
//!
//! - one-operation patterns;
//! - tiny peephole rules;
//! - hundreds of operations;
//! - thousands of operations;
//! - large generated rule sets;
//! - application-scale circuits;
//! - extremely large circuits limited only by available resources.
//!
//! This module therefore does not impose artificial circuit-size limits.
//!
//! It DOES enforce structural safety limits supplied by the caller. This is
//! important because a compiler must never confuse "large" with "invalid" and
//! must never allocate unbounded memory from untrusted pattern descriptions.
//!
//! The caller controls:
//!
//! [`PatternLimits::max_operations`]
//! [`PatternLimits::max_qubit_slots`]
//! [`PatternLimits::max_parameter_slots`]
//!
//! A limit of `0` means "unlimited" for that resource.
//!
//! # Determinism
//!
//! Pattern fingerprints use an explicitly specified deterministic hash rather
//! than `DefaultHasher`. This ensures that fingerprints remain stable across
//! processes and Rust standard-library hasher implementation changes.
//!
//! Fingerprints are identifiers, not cryptographic hashes.
//!
//! # Safety
//!
//! This module contains no `unsafe` code.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//!
//! No nightly features are required.
//! No external dependencies are required.

// =============================================================================
// Imports
// =============================================================================

use std::fmt;

use crate::quantum::ir::GateKind;

use super::rules::{
    ParameterConstraint,
    ParameterSlot,
    PatternOperation,
    QubitSlot,
    RulePattern,
};

// =============================================================================
// Constants
// =============================================================================

/// FNV-1a 64-bit offset basis.
///
/// This is used only for deterministic pattern fingerprints. It is NOT a
/// cryptographic hash and must never be used for security purposes.
const FNV_OFFSET_BASIS: u64 = 0xcbf2_9ce4_8422_2325;

/// FNV-1a 64-bit prime.
const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;

/// Maximum representable rule-local slot value.
///
/// `QubitSlot` and `ParameterSlot` are `u16`, so this is the structural upper
/// bound before caller-provided resource limits are considered.
const MAX_SLOT_INDEX: usize = u16::MAX as usize;

// =============================================================================
// Pattern limits
// =============================================================================

/// Resource limits used while validating/compiling a pattern.
///
/// These limits protect the compiler from pathological or externally supplied
/// rule descriptions without imposing a fixed practical maximum on normal
/// Zamani optimization rules.
///
/// A value of `0` means unlimited.
///
/// The limits are intentionally local to pattern construction. Circuit-size
/// limits belong to the optimizer/pipeline/IR limits and are not duplicated
/// here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PatternLimits {
    /// Maximum number of operations in one pattern.
    ///
    /// `0` means unlimited.
    pub max_operations: usize,

    /// Maximum number of distinct qubit slots.
    ///
    /// `0` means unlimited.
    pub max_qubit_slots: usize,

    /// Maximum number of distinct parameter slots.
    ///
    /// `0` means unlimited.
    pub max_parameter_slots: usize,
}

impl PatternLimits {
    /// Creates explicit pattern limits.
    #[must_use]
    pub const fn new(
        max_operations: usize,
        max_qubit_slots: usize,
        max_parameter_slots: usize,
    ) -> Self {
        Self {
            max_operations,
            max_qubit_slots,
            max_parameter_slots,
        }
    }

    /// Unlimited pattern limits.
    ///
    /// "Unlimited" means no pattern-specific artificial limit. Physical
    /// allocation and the surrounding compiler's resource limits still apply.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            max_operations: 0,
            max_qubit_slots: 0,
            max_parameter_slots: 0,
        }
    }

    /// Conservative default limits suitable for normal built-in rules.
    ///
    /// These are deliberately large enough for ordinary optimization patterns
    /// while preventing accidental pathological allocation.
    #[must_use]
    pub const fn default_compiler() -> Self {
        Self {
            max_operations: 4096,
            max_qubit_slots: 4096,
            max_parameter_slots: 4096,
        }
    }

    #[inline]
    fn accepts_operations(self, value: usize) -> bool {
        self.max_operations == 0 || value <= self.max_operations
    }

    #[inline]
    fn accepts_qubit_slots(self, value: usize) -> bool {
        self.max_qubit_slots == 0 || value <= self.max_qubit_slots
    }

    #[inline]
    fn accepts_parameter_slots(self, value: usize) -> bool {
        self.max_parameter_slots == 0 || value <= self.max_parameter_slots
    }
}

impl Default for PatternLimits {
    fn default() -> Self {
        Self::default_compiler()
    }
}

// =============================================================================
// Pattern errors
// =============================================================================

/// Errors produced while validating or compiling optimization patterns.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PatternError {
    /// The pattern contains no operations.
    EmptyPattern,

    /// The pattern exceeds the configured operation limit.
    OperationLimitExceeded {
        /// Configured maximum.
        limit: usize,

        /// Actual operation count.
        actual: usize,
    },

    /// The pattern exceeds the configured qubit-slot limit.
    QubitSlotLimitExceeded {
        /// Configured maximum.
        limit: usize,

        /// Actual number of declared slots.
        actual: usize,
    },

    /// The pattern exceeds the configured parameter-slot limit.
    ParameterSlotLimitExceeded {
        /// Configured maximum.
        limit: usize,

        /// Actual number of declared slots.
        actual: usize,
    },

    /// A qubit slot is outside the declared slot namespace.
    InvalidQubitSlot {
        /// Operation index containing the invalid slot.
        operation: usize,

        /// Invalid slot value.
        slot: u16,

        /// Number of declared slots.
        declared: usize,
    },

    /// A parameter slot is outside the declared slot namespace.
    InvalidParameterSlot {
        /// Operation index containing the invalid slot.
        operation: usize,

        /// Invalid slot value.
        slot: u16,

        /// Number of declared slots.
        declared: usize,
    },

    /// The pattern's declared qubit slot count does not agree with its actual
    /// slot usage.
    QubitSlotCountMismatch {
        /// Declared count.
        declared: usize,

        /// Highest referenced slot + 1, or zero when unused.
        required: usize,
    },

    /// The pattern's declared parameter slot count does not agree with its
    /// actual slot usage.
    ParameterSlotCountMismatch {
        /// Declared count.
        declared: usize,

        /// Highest referenced slot + 1, or zero when unused.
        required: usize,
    },

    /// The number of parameters attached to an operation does not agree with
    /// the canonical gate kind.
    ParameterArityMismatch {
        /// Operation index.
        operation: usize,

        /// Gate kind.
        gate: GateKind,

        /// Expected parameter count.
        expected: usize,

        /// Actual pattern parameter count.
        actual: usize,
    },

    /// The number of qubits attached to an operation does not satisfy the
    /// canonical gate kind's operand contract.
    OperandArityMismatch {
        /// Operation index.
        operation: usize,

        /// Gate kind.
        gate: GateKind,

        /// Expected textual operand contract.
        expected: String,

        /// Actual operand count.
        actual: usize,
    },

    /// An operation repeats a local qubit slot.
    DuplicateQubitSlot {
        /// Operation index.
        operation: usize,

        /// Repeated slot.
        slot: u16,
    },

    /// A parameter constraint references an undeclared parameter slot.
    InvalidParameterConstraint {
        /// Operation index.
        operation: usize,
    },

    /// A constant parameter constraint contains a non-finite value.
    NonFiniteConstraint {
        /// Operation index.
        operation: usize,

        /// Invalid value.
        value: f64,
    },

    /// An internal structural invariant was violated.
    InvalidStructure {
        /// Human-readable static explanation.
        message: &'static str,
    },
}

impl fmt::Display for PatternError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyPattern => {
                formatter.write_str("optimization pattern is empty")
            }

            Self::OperationLimitExceeded { limit, actual } => {
                write!(
                    formatter,
                    "optimization pattern contains {actual} operations, \
                     exceeding the configured limit of {limit}"
                )
            }

            Self::QubitSlotLimitExceeded { limit, actual } => {
                write!(
                    formatter,
                    "optimization pattern declares {actual} qubit slots, \
                     exceeding the configured limit of {limit}"
                )
            }

            Self::ParameterSlotLimitExceeded { limit, actual } => {
                write!(
                    formatter,
                    "optimization pattern declares {actual} parameter slots, \
                     exceeding the configured limit of {limit}"
                )
            }

            Self::InvalidQubitSlot {
                operation,
                slot,
                declared,
            } => {
                write!(
                    formatter,
                    "pattern operation {operation} references qubit slot {slot}, \
                     but only {declared} qubit slots are declared"
                )
            }

            Self::InvalidParameterSlot {
                operation,
                slot,
                declared,
            } => {
                write!(
                    formatter,
                    "pattern operation {operation} references parameter slot {slot}, \
                     but only {declared} parameter slots are declared"
                )
            }

            Self::QubitSlotCountMismatch {
                declared,
                required,
            } => {
                write!(
                    formatter,
                    "pattern declares {declared} qubit slots but requires {required}"
                )
            }

            Self::ParameterSlotCountMismatch {
                declared,
                required,
            } => {
                write!(
                    formatter,
                    "pattern declares {declared} parameter slots but requires {required}"
                )
            }

            Self::ParameterArityMismatch {
                operation,
                gate,
                expected,
                actual,
            } => {
                write!(
                    formatter,
                    "pattern operation {operation} using {gate:?} requires \
                     {expected} parameters but declares {actual}"
                )
            }

            Self::OperandArityMismatch {
                operation,
                gate,
                expected,
                actual,
            } => {
                write!(
                    formatter,
                    "pattern operation {operation} using {gate:?} requires \
                     {expected} operands but declares {actual}"
                )
            }

            Self::DuplicateQubitSlot { operation, slot } => {
                write!(
                    formatter,
                    "pattern operation {operation} references qubit slot {slot} \
                     more than once"
                )
            }

            Self::InvalidParameterConstraint { operation } => {
                write!(
                    formatter,
                    "pattern operation {operation} contains an invalid \
                     parameter constraint"
                )
            }

            Self::NonFiniteConstraint { operation, value } => {
                write!(
                    formatter,
                    "pattern operation {operation} contains non-finite \
                     parameter constraint value {value:?}"
                )
            }

            Self::InvalidStructure { message } => {
                write!(formatter, "invalid optimization pattern: {message}")
            }
        }
    }
}

impl std::error::Error for PatternError {}

/// Result type for pattern construction and validation.
pub type PatternResult<T> = Result<T, PatternError>;

// =============================================================================
// Pattern identity
// =============================================================================

/// Stable deterministic identifier for a pattern.
///
/// This is deliberately not a cryptographic hash.
///
/// The identifier is derived from the complete structural content of the
/// pattern, including:
//!
//! - gate kinds;
//! - qubit slots;
//! - parameter slots;
//! - parameter constraints;
//! - declared slot counts;
//! - operation ordering.
///
/// A future registry can therefore use it as a deterministic lookup key.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PatternId(u64);

impl PatternId {
    /// Creates a pattern identifier from a deterministic fingerprint.
    #[must_use]
    pub const fn from_fingerprint(value: u64) -> Self {
        Self(value)
    }

    /// Returns the underlying deterministic identifier.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }
}

impl fmt::Display for PatternId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "pattern-{:016x}", self.0)
    }
}

// =============================================================================
// Pattern fingerprint
// =============================================================================

/// Deterministic fingerprint of a declarative optimization pattern.
///
/// This is a semantic-structure fingerprint, not a security primitive.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PatternFingerprint(u64);

impl PatternFingerprint {
    /// Returns the numeric fingerprint.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }

    /// Converts the fingerprint into the stable [`PatternId`].
    #[must_use]
    pub const fn pattern_id(self) -> PatternId {
        PatternId::from_fingerprint(self.0)
    }
}

impl fmt::Display for PatternFingerprint {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:016x}", self.0)
    }
}

// =============================================================================
// Pattern statistics
// =============================================================================

/// Structural statistics calculated once when a pattern is compiled.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PatternStatistics {
    /// Number of operations.
    pub operations: usize,

    /// Number of qubit slots.
    pub qubit_slots: usize,

    /// Number of parameter slots.
    pub parameter_slots: usize,

    /// Total number of qubit-slot references.
    pub qubit_references: usize,

    /// Total number of parameter-slot references.
    pub parameter_references: usize,

    /// Number of operations with parameter constraints.
    pub constrained_operations: usize,

    /// Number of operations requiring parameters.
    pub parameterized_operations: usize,

    /// Maximum operand arity.
    pub maximum_operand_arity: usize,

    /// Maximum parameter arity.
    pub maximum_parameter_arity: usize,

    /// Number of distinct gate kinds.
    pub distinct_gate_kinds: usize,
}

impl PatternStatistics {
    /// Returns whether the pattern contains parameters.
    #[must_use]
    pub const fn is_parameterized(self) -> bool {
        self.parameter_slots != 0
    }

    /// Returns whether the pattern has any constrained operations.
    #[must_use]
    pub const fn has_constraints(self) -> bool {
        self.constrained_operations != 0
    }
}

// =============================================================================
// Gate-kind index
// =============================================================================

/// A deterministic index entry describing where a gate kind occurs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GateKindOccurrence {
    /// Gate kind.
    pub gate: GateKind,

    /// First operation index containing the gate.
    pub first_operation: usize,

    /// Number of occurrences.
    pub count: usize,
}

/// A compact immutable gate-kind index.
///
/// The entries are sorted by `GateKind` discriminant order as defined by
/// `Ord`-free deterministic enum traversal performed during compilation.
///
/// This avoids a mandatory `HashMap` allocation for every pattern and keeps
/// small patterns cheap.
///
/// The underlying slice is owned by [`CompiledPattern`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateKindIndex {
    entries: Vec<GateKindOccurrence>,
}

impl GateKindIndex {
    /// Returns an empty index.
    #[must_use]
    pub fn empty() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    /// Returns all index entries.
    #[must_use]
    pub fn entries(&self) -> &[GateKindOccurrence] {
        &self.entries
    }

    /// Returns the number of distinct gate kinds.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns true when no gate kinds are indexed.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Finds an entry using deterministic linear lookup.
    ///
    /// Pattern gate-kind cardinality is normally tiny. A vector avoids the
    /// per-pattern overhead of a hash table and is cache-friendly for the
    /// matcher.
    #[must_use]
    pub fn get(&self, gate: GateKind) -> Option<GateKindOccurrence> {
        self.entries
            .iter()
            .copied()
            .find(|entry| entry.gate == gate)
    }
}

// =============================================================================
// Operation metadata
// =============================================================================

/// Matcher-facing immutable metadata for one pattern operation.
///
/// This avoids repeatedly traversing the rule's static slices during matching.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PatternOperationMetadata {
    /// Operation position.
    pub index: usize,

    /// Required gate kind.
    pub gate: GateKind,

    /// Number of qubit operands.
    pub qubit_arity: usize,

    /// Number of parameter operands.
    pub parameter_arity: usize,

    /// Whether the operation has a parameter constraint.
    pub has_parameter_constraint: bool,

    /// Whether the operation is parameterized according to its gate kind.
    pub is_parameterized: bool,

    /// First qubit slot, when one exists.
    pub first_qubit_slot: Option<QubitSlot>,

    /// First parameter slot, when one exists.
    pub first_parameter_slot: Option<ParameterSlot>,
}

// =============================================================================
// Compiled pattern
// =============================================================================

/// Validated, immutable, matcher-friendly optimization pattern.
///
/// `CompiledPattern` owns only derived metadata and cloned operation metadata;
/// it does not own or duplicate a Quantum IR circuit.
///
/// The original declarative [`RulePattern`] remains available through
/// [`Self::rule_pattern`].
///
/// The object is immutable after construction and can safely be shared through
/// ordinary Rust ownership (`Arc`, `&`, etc.) by higher-level compiler code.
/// No global cache is required.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompiledPattern {
    rule_pattern: RulePattern,

    fingerprint: PatternFingerprint,

    statistics: PatternStatistics,

    gate_index: GateKindIndex,

    operations: Vec<PatternOperationMetadata>,
}

impl CompiledPattern {
    /// Compiles and validates a rule pattern using explicit limits.
    ///
    /// Compilation performs all structural checks that can be established
    /// without inspecting a circuit.
    pub fn compile(
        pattern: RulePattern,
        limits: PatternLimits,
    ) -> PatternResult<Self> {
        validate_rule_pattern(pattern, limits)?;

        let fingerprint = fingerprint_rule_pattern(pattern);

        let statistics = calculate_statistics(pattern);

        let gate_index = build_gate_index(pattern);

        let operations = build_operation_metadata(pattern);

        Ok(Self {
            rule_pattern: pattern,
            fingerprint,
            statistics,
            gate_index,
            operations,
        })
    }

    /// Compiles a pattern using the standard compiler limits.
    pub fn compile_default(
        pattern: RulePattern,
    ) -> PatternResult<Self> {
        Self::compile(pattern, PatternLimits::default())
    }

    /// Compiles a pattern without artificial pattern-size limits.
    pub fn compile_unlimited(
        pattern: RulePattern,
    ) -> PatternResult<Self> {
        Self::compile(pattern, PatternLimits::unlimited())
    }

    /// Returns the original declarative rule pattern.
    #[must_use]
    pub const fn rule_pattern(&self) -> RulePattern {
        self.rule_pattern
    }

    /// Returns the deterministic pattern fingerprint.
    #[must_use]
    pub const fn fingerprint(&self) -> PatternFingerprint {
        self.fingerprint
    }

    /// Returns the stable pattern identifier.
    #[must_use]
    pub const fn id(&self) -> PatternId {
        self.fingerprint.pattern_id()
    }

    /// Returns structural statistics.
    #[must_use]
    pub const fn statistics(&self) -> PatternStatistics {
        self.statistics
    }

    /// Returns the gate-kind index.
    #[must_use]
    pub fn gate_index(&self) -> &GateKindIndex {
        &self.gate_index
    }

    /// Returns operation metadata in pattern order.
    #[must_use]
    pub fn operations(&self) -> &[PatternOperationMetadata] {
        &self.operations
    }

    /// Returns one operation's metadata.
    #[must_use]
    pub fn operation(
        &self,
        index: usize,
    ) -> Option<PatternOperationMetadata> {
        self.operations.get(index).copied()
    }

    /// Returns the number of operations.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.statistics.operations
    }

    /// Returns whether the pattern contains no operations.
    ///
    /// Valid compiled patterns are never empty. This method exists for
    /// generic collection-like callers.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.statistics.operations == 0
    }

    /// Returns the operation most useful as a candidate-search anchor.
    ///
    /// The chosen anchor is the rarest gate kind within the pattern. This is a
    /// deterministic heuristic and does not perform circuit matching.
    ///
    /// A future matcher can use the returned operation index to reduce the
    /// number of candidate windows it examines.
    #[must_use]
    pub fn anchor_operation(&self) -> Option<usize> {
        let mut best: Option<(usize, usize)> = None;

        for operation in &self.operations {
            let count = self
                .gate_index
                .get(operation.gate)
                .map(|entry| entry.count)
                .unwrap_or(usize::MAX);

            match best {
                None => {
                    best = Some((operation.index, count));
                }

                Some((best_index, best_count)) => {
                    if count < best_count
                        || (count == best_count
                            && operation.index < best_index)
                    {
                        best = Some((operation.index, count));
                    }
                }
            }
        }

        best.map(|(index, _)| index)
    }

    /// Returns the anchor gate kind.
    #[must_use]
    pub fn anchor_gate(&self) -> Option<GateKind> {
        self.anchor_operation()
            .and_then(|index| self.operation(index))
            .map(|operation| operation.gate)
    }
}

// =============================================================================
// Pattern view
// =============================================================================

/// Lightweight borrowed pattern view.
///
/// This type is useful when a caller needs validation/fingerprinting without
/// retaining compiled metadata.
///
/// It deliberately contains no mutable state.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PatternView {
    pattern: RulePattern,
}

impl PatternView {
    /// Creates a view over a declarative rule pattern.
    #[must_use]
    pub const fn new(pattern: RulePattern) -> Self {
        Self { pattern }
    }

    /// Returns the underlying rule pattern.
    #[must_use]
    pub const fn as_rule_pattern(self) -> RulePattern {
        self.pattern
    }

    /// Returns the operation count.
    #[must_use]
    pub const fn len(self) -> usize {
        self.pattern.operations.len()
    }

    /// Returns whether the pattern is empty.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.pattern.operations.is_empty()
    }

    /// Validates the pattern with explicit limits.
    pub fn validate(
        self,
        limits: PatternLimits,
    ) -> PatternResult<()> {
        validate_rule_pattern(self.pattern, limits)
    }

    /// Calculates the deterministic fingerprint.
    ///
    /// The pattern is validated first so callers never accidentally assign a
    /// stable identity to structurally invalid data.
    pub fn fingerprint(
        self,
        limits: PatternLimits,
    ) -> PatternResult<PatternFingerprint> {
        self.validate(limits)?;
        Ok(fingerprint_rule_pattern(self.pattern))
    }
}

// =============================================================================
// Public validation API
// =============================================================================

/// Validates a declarative rule pattern using default compiler limits.
pub fn validate(
    pattern: RulePattern,
) -> PatternResult<()> {
    validate_with_limits(pattern, PatternLimits::default())
}

/// Validates a declarative rule pattern with explicit limits.
pub fn validate_with_limits(
    pattern: RulePattern,
    limits: PatternLimits,
) -> PatternResult<()> {
    validate_rule_pattern(pattern, limits)
}

/// Compiles a declarative rule pattern using default compiler limits.
pub fn compile(
    pattern: RulePattern,
) -> PatternResult<CompiledPattern> {
    CompiledPattern::compile_default(pattern)
}

/// Compiles a declarative rule pattern with explicit limits.
pub fn compile_with_limits(
    pattern: RulePattern,
    limits: PatternLimits,
) -> PatternResult<CompiledPattern> {
    CompiledPattern::compile(pattern, limits)
}

/// Compiles a declarative rule pattern without artificial pattern-size
/// limits.
///
/// This is appropriate for trusted generated rule sets when the surrounding
/// compiler already owns resource limits.
pub fn compile_unlimited(
    pattern: RulePattern,
) -> PatternResult<CompiledPattern> {
    CompiledPattern::compile_unlimited(pattern)
}

/// Calculates a deterministic fingerprint after validation.
pub fn fingerprint(
    pattern: RulePattern,
) -> PatternResult<PatternFingerprint> {
    fingerprint_with_limits(pattern, PatternLimits::default())
}

/// Calculates a deterministic fingerprint using explicit limits.
pub fn fingerprint_with_limits(
    pattern: RulePattern,
    limits: PatternLimits,
) -> PatternResult<PatternFingerprint> {
    validate_rule_pattern(pattern, limits)?;
    Ok(fingerprint_rule_pattern(pattern))
}

// =============================================================================
// Internal validation
// =============================================================================

fn validate_rule_pattern(
    pattern: RulePattern,
    limits: PatternLimits,
) -> PatternResult<()> {
    let operation_count = pattern.operations.len();

    if operation_count == 0 {
        return Err(PatternError::EmptyPattern);
    }

    if !limits.accepts_operations(operation_count) {
        return Err(PatternError::OperationLimitExceeded {
            limit: limits.max_operations,
            actual: operation_count,
        });
    }

    if pattern.qubit_slots > MAX_SLOT_INDEX + 1 {
        return Err(PatternError::InvalidStructure {
            message: "qubit slot count exceeds u16 slot namespace",
        });
    }

    if pattern.parameter_slots > MAX_SLOT_INDEX + 1 {
        return Err(PatternError::InvalidStructure {
            message: "parameter slot count exceeds u16 slot namespace",
        });
    }

    if !limits.accepts_qubit_slots(pattern.qubit_slots) {
        return Err(PatternError::QubitSlotLimitExceeded {
            limit: limits.max_qubit_slots,
            actual: pattern.qubit_slots,
        });
    }

    if !limits.accepts_parameter_slots(pattern.parameter_slots) {
        return Err(PatternError::ParameterSlotLimitExceeded {
            limit: limits.max_parameter_slots,
            actual: pattern.parameter_slots,
        });
    }

    let mut required_qubit_slots = 0usize;
    let mut required_parameter_slots = 0usize;

    for (operation_index, operation) in
        pattern.operations.iter().enumerate()
    {
        validate_operation(
            operation_index,
            operation,
            pattern.qubit_slots,
            pattern.parameter_slots,
        )?;

        for slot in operation.qubits {
            let required = usize::from(slot.index()) + 1;

            if required > required_qubit_slots {
                required_qubit_slots = required;
            }
        }

        for slot in operation.parameters {
            let required = usize::from(slot.index()) + 1;

            if required > required_parameter_slots {
                required_parameter_slots = required;
            }
        }

        if let Some(constraint) =
            operation.parameter_constraint
        {
            validate_parameter_constraint(
                operation_index,
                constraint,
                pattern.parameter_slots,
            )?;
        }
    }

    if required_qubit_slots > pattern.qubit_slots {
        return Err(PatternError::QubitSlotCountMismatch {
            declared: pattern.qubit_slots,
            required: required_qubit_slots,
        });
    }

    if required_parameter_slots > pattern.parameter_slots {
        return Err(PatternError::ParameterSlotCountMismatch {
            declared: pattern.parameter_slots,
            required: required_parameter_slots,
        });
    }

    Ok(())
}

fn validate_operation(
    operation_index: usize,
    operation: &PatternOperation,
    declared_qubit_slots: usize,
    declared_parameter_slots: usize,
) -> PatternResult<()> {
    let expected_operands = operation.gate.operand_count();

    if !expected_operands.accepts(operation.qubits.len()) {
        return Err(PatternError::OperandArityMismatch {
            operation: operation_index,
            gate: operation.gate,
            expected: expected_operands.to_string(),
            actual: operation.qubits.len(),
        });
    }

    let expected_parameters =
        operation.gate.parameter_count();

    if expected_parameters != operation.parameters.len() {
        return Err(PatternError::ParameterArityMismatch {
            operation: operation_index,
            gate: operation.gate,
            expected: expected_parameters,
            actual: operation.parameters.len(),
        });
    }

    for (left_index, left) in
        operation.qubits.iter().enumerate()
    {
        let left_value = left.index();

        if usize::from(left_value) >= declared_qubit_slots {
            return Err(PatternError::InvalidQubitSlot {
                operation: operation_index,
                slot: left_value,
                declared: declared_qubit_slots,
            });
        }

        for right in
            operation.qubits.iter().skip(left_index + 1)
        {
            if left == right {
                return Err(PatternError::DuplicateQubitSlot {
                    operation: operation_index,
                    slot: left_value,
                });
            }
        }
    }

    for slot in operation.parameters {
        if usize::from(slot.index())
            >= declared_parameter_slots
        {
            return Err(PatternError::InvalidParameterSlot {
                operation: operation_index,
                slot: slot.index(),
                declared: declared_parameter_slots,
            });
        }
    }

    if let Some(constraint) =
        operation.parameter_constraint
    {
        validate_parameter_constraint(
            operation_index,
            constraint,
            declared_parameter_slots,
        )?;
    }

    Ok(())
}

fn validate_parameter_constraint(
    operation_index: usize,
    constraint: ParameterConstraint,
    declared_parameter_slots: usize,
) -> PatternResult<()> {
    match constraint {
        ParameterConstraint::Any => {}

        ParameterConstraint::Zero(slot) => {
            validate_constraint_slot(
                operation_index,
                slot,
                declared_parameter_slots,
            )?;
        }

        ParameterConstraint::Constant {
            slot,
            value,
        } => {
            validate_constraint_slot(
                operation_index,
                slot,
                declared_parameter_slots,
            )?;

            if !value.is_finite() {
                return Err(
                    PatternError::NonFiniteConstraint {
                        operation: operation_index,
                        value,
                    },
                );
            }
        }

        ParameterConstraint::Equal {
            left,
            right,
        } => {
            validate_constraint_slot(
                operation_index,
                left,
                declared_parameter_slots,
            )?;

            validate_constraint_slot(
                operation_index,
                right,
                declared_parameter_slots,
            )?;
        }

        ParameterConstraint::NegationPair {
            left,
            right,
        } => {
            validate_constraint_slot(
                operation_index,
                left,
                declared_parameter_slots,
            )?;

            validate_constraint_slot(
                operation_index,
                right,
                declared_parameter_slots,
            )?;
        }
    }

    Ok(())
}

fn validate_constraint_slot(
    operation_index: usize,
    slot: ParameterSlot,
    declared_parameter_slots: usize,
) -> PatternResult<()> {
    if usize::from(slot.index())
        >= declared_parameter_slots
    {
        return Err(PatternError::InvalidParameterConstraint {
            operation: operation_index,
        });
    }

    Ok(())
}

// =============================================================================
// Statistics
// =============================================================================

fn calculate_statistics(
    pattern: RulePattern,
) -> PatternStatistics {
    let mut qubit_references = 0usize;
    let mut parameter_references = 0usize;
    let mut constrained_operations = 0usize;
    let mut parameterized_operations = 0usize;
    let mut maximum_operand_arity = 0usize;
    let mut maximum_parameter_arity = 0usize;

    let mut gate_kinds = Vec::<GateKind>::new();

    for operation in pattern.operations {
        qubit_references = qubit_references
            .saturating_add(operation.qubits.len());

        parameter_references = parameter_references
            .saturating_add(operation.parameters.len());

        if operation.parameter_constraint.is_some() {
            constrained_operations =
                constrained_operations.saturating_add(1);
        }

        if operation.gate.is_parameterized() {
            parameterized_operations =
                parameterized_operations.saturating_add(1);
        }

        maximum_operand_arity =
            maximum_operand_arity.max(operation.qubits.len());

        maximum_parameter_arity =
            maximum_parameter_arity
                .max(operation.parameters.len());

        if !gate_kinds.contains(&operation.gate) {
            gate_kinds.push(operation.gate);
        }
    }

    PatternStatistics {
        operations: pattern.operations.len(),
        qubit_slots: pattern.qubit_slots,
        parameter_slots: pattern.parameter_slots,
        qubit_references,
        parameter_references,
        constrained_operations,
        parameterized_operations,
        maximum_operand_arity,
        maximum_parameter_arity,
        distinct_gate_kinds: gate_kinds.len(),
    }
}

// =============================================================================
// Gate-kind indexing
// =============================================================================

fn build_gate_index(
    pattern: RulePattern,
) -> GateKindIndex {
    let mut entries =
        Vec::<GateKindOccurrence>::new();

    for (operation_index, operation) in
        pattern.operations.iter().enumerate()
    {
        if let Some(entry) = entries
            .iter_mut()
            .find(|entry| entry.gate == operation.gate)
        {
            entry.count =
                entry.count.saturating_add(1);
        } else {
            entries.push(GateKindOccurrence {
                gate: operation.gate,
                first_operation: operation_index,
                count: 1,
            });
        }
    }

    entries.sort_by_key(|entry| {
        gate_kind_rank(entry.gate)
    });

    entries.shrink_to_fit();

    GateKindIndex { entries }
}

/// Deterministic rank for canonical gate kinds.
///
/// This does not depend on Rust's internal enum discriminant layout.
const fn gate_kind_rank(gate: GateKind) -> u16 {
    match gate {
        GateKind::I => 0,
        GateKind::X => 1,
        GateKind::Y => 2,
        GateKind::Z => 3,
        GateKind::H => 4,
        GateKind::S => 5,
        GateKind::Sdg => 6,
        GateKind::T => 7,
        GateKind::Tdg => 8,
        GateKind::V => 9,
        GateKind::Vdg => 10,
        GateKind::RX => 11,
        GateKind::RY => 12,
        GateKind::RZ => 13,
        GateKind::Phase => 14,
        GateKind::U1 => 15,
        GateKind::U2 => 16,
        GateKind::U3 => 17,
        GateKind::CX => 18,
        GateKind::CY => 19,
        GateKind::CZ => 20,
        GateKind::CH => 21,
        GateKind::SWAP => 22,
        GateKind::ISWAP => 23,
        GateKind::ECR => 24,
        GateKind::CRX => 25,
        GateKind::CRY => 26,
        GateKind::CRZ => 27,
        GateKind::CCX => 28,
        GateKind::CSWAP => 29,
        GateKind::Measure => 30,
        GateKind::Barrier => 31,
        GateKind::Reset => 32,
    }
}

// =============================================================================
// Operation metadata
// =============================================================================

fn build_operation_metadata(
    pattern: RulePattern,
) -> Vec<PatternOperationMetadata> {
    let mut operations =
        Vec::with_capacity(pattern.operations.len());

    for (index, operation) in
        pattern.operations.iter().enumerate()
    {
        operations.push(PatternOperationMetadata {
            index,
            gate: operation.gate,
            qubit_arity: operation.qubits.len(),
            parameter_arity: operation.parameters.len(),
            has_parameter_constraint:
                operation.parameter_constraint.is_some(),
            is_parameterized:
                operation.gate.is_parameterized(),
            first_qubit_slot:
                operation.qubits.first().copied(),
            first_parameter_slot:
                operation.parameters.first().copied(),
        });
    }

    operations
}

// =============================================================================
// Deterministic fingerprint
// =============================================================================

fn fingerprint_rule_pattern(
    pattern: RulePattern,
) -> PatternFingerprint {
    let mut hasher =
        StablePatternHasher::new();

    hasher.write_u64(
        pattern.qubit_slots as u64
    );

    hasher.write_u64(
        pattern.parameter_slots as u64
    );

    hasher.write_u64(
        pattern.operations.len() as u64
    );

    for operation in pattern.operations {
        hasher.write_u16(
            gate_kind_rank(operation.gate)
        );

        hasher.write_u64(
            operation.qubits.len() as u64
        );

        for slot in operation.qubits {
            hasher.write_u16(slot.index());
        }

        hasher.write_u64(
            operation.parameters.len() as u64
        );

        for slot in operation.parameters {
            hasher.write_u16(slot.index());
        }

        write_parameter_constraint(
            &mut hasher,
            operation.parameter_constraint,
        );
    }

    PatternFingerprint(hasher.finish())
}

fn write_parameter_constraint(
    hasher: &mut StablePatternHasher,
    constraint: Option<ParameterConstraint>,
) {
    match constraint {
        None => {
            hasher.write_byte(0);
        }

        Some(ParameterConstraint::Any) => {
            hasher.write_byte(1);
        }

        Some(ParameterConstraint::Zero(slot)) => {
            hasher.write_byte(2);
            hasher.write_u16(slot.index());
        }

        Some(ParameterConstraint::Constant {
            slot,
            value,
        }) => {
            hasher.write_byte(3);
            hasher.write_u16(slot.index());
            hasher.write_u64(value.to_bits());
        }

        Some(ParameterConstraint::Equal {
            left,
            right,
        }) => {
            hasher.write_byte(4);
            hasher.write_u16(left.index());
            hasher.write_u16(right.index());
        }

        Some(ParameterConstraint::NegationPair {
            left,
            right,
        }) => {
            hasher.write_byte(5);
            hasher.write_u16(left.index());
            hasher.write_u16(right.index());
        }
    }
}

// =============================================================================
// Stable hasher
// =============================================================================

/// Small deterministic FNV-1a hasher used exclusively for pattern identity.
#[derive(Debug, Clone, Copy)]
struct StablePatternHasher {
    state: u64,
}

impl StablePatternHasher {
    #[must_use]
    const fn new() -> Self {
        Self {
            state: FNV_OFFSET_BASIS,
        }
    }

    #[inline]
    fn write_byte(
        &mut self,
        value: u8,
    ) {
        self.state ^= u64::from(value);
        self.state =
            self.state.wrapping_mul(FNV_PRIME);
    }

    #[inline]
    fn write_u16(
        &mut self,
        value: u16,
    ) {
        for byte in value.to_le_bytes() {
            self.write_byte(byte);
        }
    }

    #[inline]
    fn write_u64(
        &mut self,
        value: u64,
    ) {
        for byte in value.to_le_bytes() {
            self.write_byte(byte);
        }
    }

    #[must_use]
    const fn finish(self) -> u64 {
        self.state
    }
}

// =============================================================================
// Pattern comparison helpers
// =============================================================================

/// Returns whether two declarative patterns have identical structural
/// semantics.
///
/// This does not perform semantic circuit equivalence checking. It only checks
/// whether the pattern descriptions themselves are structurally identical.
#[must_use]
pub fn structurally_equal(
    left: RulePattern,
    right: RulePattern,
) -> bool {
    left == right
}

/// Returns whether two compiled patterns have identical structural content.
#[must_use]
pub fn compiled_structurally_equal(
    left: &CompiledPattern,
    right: &CompiledPattern,
) -> bool {
    left.fingerprint == right.fingerprint
        && left.rule_pattern == right.rule_pattern
}

// =============================================================================
// Pattern-size utility
// =============================================================================

/// Returns the total number of primitive pattern references.
///
/// This is useful for estimating matcher work before matching begins.
///
/// The value is saturated at `usize::MAX` rather than overflowing.
#[must_use]
pub fn reference_count(
    pattern: RulePattern,
) -> usize {
    let mut total = 0usize;

    for operation in pattern.operations {
        total = total.saturating_add(
            operation.qubits.len(),
        );

        total = total.saturating_add(
            operation.parameters.len(),
        );
    }

    total
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    const Q0: QubitSlot = QubitSlot::new(0);
    const Q1: QubitSlot = QubitSlot::new(1);

    const P0: ParameterSlot = ParameterSlot::new(0);
    const P1: ParameterSlot = ParameterSlot::new(1);

    const H_PATTERN_OPERATION: PatternOperation =
        PatternOperation::new(
            GateKind::H,
            &[Q0],
            &[],
            None,
        );

    const XX_PATTERN_OPERATIONS: &[PatternOperation] =
        &[
            PatternOperation::new(
                GateKind::X,
                &[Q0],
                &[],
                None,
            ),
            PatternOperation::new(
                GateKind::X,
                &[Q0],
                &[],
                None,
            ),
        ];

    const ROTATION_PATTERN_OPERATIONS: &[PatternOperation] =
        &[
            PatternOperation::new(
                GateKind::RZ,
                &[Q0],
                &[P0],
                None,
            ),
            PatternOperation::new(
                GateKind::RZ,
                &[Q0],
                &[P1],
                None,
            ),
        ];

    const CX_PATTERN_OPERATIONS: &[PatternOperation] =
        &[
            PatternOperation::new(
                GateKind::CX,
                &[Q0, Q1],
                &[],
                None,
            ),
            PatternOperation::new(
                GateKind::CX,
                &[Q0, Q1],
                &[],
                None,
            ),
        ];

    #[test]
    fn validates_single_gate_pattern() {
        let pattern = RulePattern::new(
            &[H_PATTERN_OPERATION],
            1,
            0,
        );

        validate(pattern)
            .expect("single-gate pattern should validate");
    }

    #[test]
    fn compiles_single_gate_pattern() {
        let pattern = RulePattern::new(
            &[H_PATTERN_OPERATION],
            1,
            0,
        );

        let compiled =
            compile(pattern)
                .expect("pattern should compile");

        assert_eq!(compiled.len(), 1);
        assert_eq!(
            compiled.statistics().qubit_slots,
            1
        );
        assert_eq!(
            compiled.statistics().parameter_slots,
            0
        );
        assert_eq!(
            compiled.anchor_gate(),
            Some(GateKind::H)
        );
    }

    #[test]
    fn validates_two_qubit_pattern() {
        let pattern = RulePattern::new(
            CX_PATTERN_OPERATIONS,
            2,
            0,
        );

        validate(pattern)
            .expect("CX pattern should validate");

        let compiled =
            compile(pattern)
                .expect("CX pattern should compile");

        assert_eq!(
            compiled.statistics().operations,
            2
        );

        assert_eq!(
            compiled.statistics().maximum_operand_arity,
            2
        );

        assert_eq!(
            compiled
                .gate_index()
                .get(GateKind::CX)
                .map(|entry| entry.count),
            Some(2)
        );
    }

    #[test]
    fn validates_parameterized_pattern() {
        let pattern = RulePattern::new(
            ROTATION_PATTERN_OPERATIONS,
            1,
            2,
        );

        validate(pattern)
            .expect(
                "parameterized pattern should validate",
            );

        let compiled =
            compile(pattern)
                .expect(
                    "parameterized pattern should compile",
                );

        assert_eq!(
            compiled.statistics().parameter_slots,
            2
        );

        assert_eq!(
            compiled.statistics().parameter_references,
            2
        );

        assert!(
            compiled.statistics().is_parameterized()
        );
    }

    #[test]
    fn rejects_empty_pattern() {
        let pattern =
            RulePattern::new(&[], 0, 0);

        assert_eq!(
            validate(pattern),
            Err(PatternError::EmptyPattern)
        );
    }

    #[test]
    fn rejects_invalid_qubit_slot() {
        let operations: &[PatternOperation] =
            &[PatternOperation::new(
                GateKind::H,
                &[QubitSlot::new(1)],
                &[],
                None,
            )];

        let pattern =
            RulePattern::new(operations, 1, 0);

        assert!(matches!(
            validate(pattern),
            Err(PatternError::InvalidQubitSlot {
                operation: 0,
                slot: 1,
                declared: 1,
            })
        ));
    }

    #[test]
    fn rejects_invalid_parameter_slot() {
        let operations: &[PatternOperation] =
            &[PatternOperation::new(
                GateKind::RZ,
                &[Q0],
                &[P1],
                None,
            )];

        let pattern =
            RulePattern::new(operations, 1, 1);

        assert!(matches!(
            validate(pattern),
            Err(
                PatternError::InvalidParameterSlot {
                    operation: 0,
                    slot: 1,
                    declared: 1,
                }
            )
        ));
    }

    #[test]
    fn rejects_duplicate_qubit_slot() {
        let operations: &[PatternOperation] =
            &[PatternOperation::new(
                GateKind::CX,
                &[Q0, Q0],
                &[],
                None,
            )];

        let pattern =
            RulePattern::new(operations, 1, 0);

        assert!(matches!(
            validate(pattern),
            Err(PatternError::DuplicateQubitSlot {
                operation: 0,
                slot: 0,
            })
        ));
    }

    #[test]
    fn rejects_wrong_parameter_arity() {
        let operations: &[PatternOperation] =
            &[PatternOperation::new(
                GateKind::RZ,
                &[Q0],
                &[],
                None,
            )];

        let pattern =
            RulePattern::new(operations, 1, 0);

        assert!(matches!(
            validate(pattern),
            Err(PatternError::ParameterArityMismatch {
                operation: 0,
                gate: GateKind::RZ,
                expected: 1,
                actual: 0,
            })
        ));
    }

    #[test]
    fn rejects_wrong_operand_arity() {
        let operations: &[PatternOperation] =
            &[PatternOperation::new(
                GateKind::CX,
                &[Q0],
                &[],
                None,
            )];

        let pattern =
            RulePattern::new(operations, 1, 0);

        assert!(matches!(
            validate(pattern),
            Err(PatternError::OperandArityMismatch {
                operation: 0,
                gate: GateKind::CX,
                actual: 1,
                ..
            })
        ));
    }

    #[test]
    fn rejects_parameter_constraint_outside_namespace() {
        let operations: &[PatternOperation] =
            &[PatternOperation::new(
                GateKind::RZ,
                &[Q0],
                &[P0],
                Some(
                    ParameterConstraint::Equal {
                        left: P0,
                        right: P1,
                    },
                ),
            )];

        let pattern =
            RulePattern::new(operations, 1, 1);

        assert!(matches!(
            validate(pattern),
            Err(
                PatternError::InvalidParameterConstraint {
                    operation: 0
                }
            )
        ));
    }

    #[test]
    fn rejects_non_finite_constant_constraint() {
        let operations: &[PatternOperation] =
            &[PatternOperation::new(
                GateKind::RZ,
                &[Q0],
                &[P0],
                Some(
                    ParameterConstraint::Constant {
                        slot: P0,
                        value: f64::NAN,
                    },
                ),
            )];

        let pattern =
            RulePattern::new(operations, 1, 1);

        assert!(matches!(
            validate(pattern),
            Err(PatternError::NonFiniteConstraint {
                operation: 0,
                ..
            })
        ));
    }

    #[test]
    fn respects_operation_limit() {
        let pattern = RulePattern::new(
            XX_PATTERN_OPERATIONS,
            1,
            0,
        );

        let limits =
            PatternLimits::new(1, 0, 0);

        assert!(matches!(
            validate_with_limits(
                pattern,
                limits
            ),
            Err(
                PatternError::OperationLimitExceeded {
                    limit: 1,
                    actual: 2,
                }
            )
        ));
    }

    #[test]
    fn respects_qubit_slot_limit() {
        let pattern = RulePattern::new(
            CX_PATTERN_OPERATIONS,
            2,
            0,
        );

        let limits =
            PatternLimits::new(0, 1, 0);

        assert!(matches!(
            validate_with_limits(
                pattern,
                limits
            ),
            Err(
                PatternError::QubitSlotLimitExceeded {
                    limit: 1,
                    actual: 2,
                }
            )
        ));
    }

    #[test]
    fn respects_parameter_slot_limit() {
        let pattern = RulePattern::new(
            ROTATION_PATTERN_OPERATIONS,
            1,
            2,
        );

        let limits =
            PatternLimits::new(0, 0, 1);

        assert!(matches!(
            validate_with_limits(
                pattern,
                limits
            ),
            Err(
                PatternError::ParameterSlotLimitExceeded {
                    limit: 1,
                    actual: 2,
                }
            )
        ));
    }

    #[test]
    fn fingerprint_is_deterministic() {
        let pattern = RulePattern::new(
            XX_PATTERN_OPERATIONS,
            1,
            0,
        );

        let first =
            fingerprint(pattern)
                .expect("fingerprint should succeed");

        let second =
            fingerprint(pattern)
                .expect("fingerprint should succeed");

        assert_eq!(first, second);
        assert_eq!(
            first.pattern_id().value(),
            first.value()
        );
    }

    #[test]
    fn different_patterns_have_different_structural_fingerprints() {
        let h_pattern =
            RulePattern::new(
                &[H_PATTERN_OPERATION],
                1,
                0,
            );

        let x_pattern =
            RulePattern::new(
                &[PatternOperation::new(
                    GateKind::X,
                    &[Q0],
                    &[],
                    None,
                )],
                1,
                0,
            );

        let h =
            fingerprint(h_pattern)
                .expect("fingerprint should succeed");

        let x =
            fingerprint(x_pattern)
                .expect("fingerprint should succeed");

        assert_ne!(h, x);
    }

    #[test]
    fn structural_equality_is_not_semantic_equivalence() {
        let first = RulePattern::new(
            XX_PATTERN_OPERATIONS,
            1,
            0,
        );

        let second = RulePattern::new(
            XX_PATTERN_OPERATIONS,
            1,
            0,
        );

        assert!(structurally_equal(
            first,
            second
        ));
    }

    #[test]
    fn gate_kind_index_is_deterministic() {
        let operations: &[PatternOperation] =
            &[
                PatternOperation::new(
                    GateKind::Z,
                    &[Q0],
                    &[],
                    None,
                ),
                PatternOperation::new(
                    GateKind::H,
                    &[Q0],
                    &[],
                    None,
                ),
                PatternOperation::new(
                    GateKind::Z,
                    &[Q0],
                    &[],
                    None,
                ),
            ];

        let pattern =
            RulePattern::new(operations, 1, 0);

        let compiled =
            compile(pattern)
                .expect("pattern should compile");

        let entries =
            compiled.gate_index().entries();

        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].gate, GateKind::H);
        assert_eq!(entries[1].gate, GateKind::Z);
        assert_eq!(entries[1].count, 2);
    }

    #[test]
    fn rare_gate_is_selected_as_anchor() {
        let operations: &[PatternOperation] =
            &[
                PatternOperation::new(
                    GateKind::H,
                    &[Q0],
                    &[],
                    None,
                ),
                PatternOperation::new(
                    GateKind::H,
                    &[Q0],
                    &[],
                    None,
                ),
                PatternOperation::new(
                    GateKind::T,
                    &[Q0],
                    &[],
                    None,
                ),
            ];

        let pattern =
            RulePattern::new(operations, 1, 0);

        let compiled =
            compile(pattern)
                .expect("pattern should compile");

        assert_eq!(
            compiled.anchor_operation(),
            Some(2)
        );

        assert_eq!(
            compiled.anchor_gate(),
            Some(GateKind::T)
        );
    }

    #[test]
    fn reference_count_is_saturating() {
        let pattern = RulePattern::new(
            ROTATION_PATTERN_OPERATIONS,
            1,
            2,
        );

        assert_eq!(
            reference_count(pattern),
            4
        );
    }

    #[test]
    fn pattern_view_validates_and_fingerprints() {
        let pattern =
            RulePattern::new(
                XX_PATTERN_OPERATIONS,
                1,
                0,
            );

        let view =
            PatternView::new(pattern);

        assert_eq!(view.len(), 2);

        view.validate(
            PatternLimits::unlimited(),
        )
        .expect("view validation should succeed");

        let fingerprint = view
            .fingerprint(
                PatternLimits::unlimited(),
            )
            .expect("fingerprint should succeed");

        assert_ne!(fingerprint.value(), 0);
    }

    #[test]
    fn unlimited_compilation_accepts_large_declared_limits() {
        let pattern =
            RulePattern::new(
                &[H_PATTERN_OPERATION],
                1,
                0,
            );

        let compiled =
            compile_unlimited(pattern)
                .expect(
                    "unlimited compilation should succeed",
                );

        assert_eq!(compiled.len(), 1);
    }
}