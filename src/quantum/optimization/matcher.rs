//! Zamani Quantum Optimization — Production Pattern Matcher
//!
//! This module matches immutable optimization patterns from `pattern.rs`
//! against the canonical `quantum::ir::QuantumCircuit` / `Gate` sequence.
//!
//! The matcher deliberately does not define a second quantum IR, mutate a
//! circuit, perform rewrites, route qubits, schedule operations, or execute
//! hardware. It produces immutable match records for `rewrite.rs`, passes,
//! verification, provenance, and planning infrastructure.
//!
//! # Architectural contract
//!
//! ```text
//! quantum::ir::Gate / QuantumCircuit
//!                 │
//!                 ▼
//!        optimization::matcher
//!                 ▲
//!                 │
//!        optimization::pattern
//!                 │
//!        optimization::rules
//! ```
//!
//! The matcher consumes `CompiledPattern`, whose declarative source is the
//! canonical `RulePattern`. The canonical circuit remains owned by
//! `quantum::ir`.
//!
//! # Matching modes
//!
//! * `Exact` — contiguous operation sequence matching. This is the fastest and
//!   most conservative mode and uses the compiled pattern's rare-gate anchor.
//!
//! * `DependencyAware` — contiguous matching plus an explicit dependency
//!   predicate supplied by the caller.
//!
//! * `CommutationAware` — pattern operations may cross intervening operations
//!   only when a caller supplied `CommutationOracle` explicitly proves that
//!   the crossing is safe.
//!
//! The matcher itself does not invent quantum commutation identities. The
//! future `analysis::commutation` subsystem can implement `CommutationOracle`
//! without requiring this file to change.
//!
//! # Important semantic rule
//!
//! A match is not itself a rewrite.
//!
//! A successful `PatternMatch` records:
//!
//! * the pattern identity;
//! * the circuit span inspected;
//! * the exact circuit operation indices corresponding to pattern operations;
//! * concrete qubit bindings;
//! * concrete parameter bindings.
//!
//! `rewrite.rs` remains responsible for deciding whether and how that match
//! becomes a circuit transformation.
//!
//! # Scaling
//!
//! Exact matching uses the rarest pattern operation selected by
//! `CompiledPattern::anchor_operation()` to reduce candidate windows.
//!
//! The matcher does not impose a circuit-size ceiling. Resource limits are
//! explicit through `MatcherLimits`; zero means unlimited.
//!
//! Commutation-aware matching is potentially combinatorial, so it has explicit
//! candidate/comparison/scan/match budgets. This permits tiny circuits,
//! application-scale circuits, and extremely large circuits without encoding an
//! arbitrary global maximum.
//!
//! # Safety
//!
//! No unsafe code is used.
//!
//! # Rust compatibility
//!
//! * Rust 1.97 / 1.97.1
//! * Rust 2021
//! * Stable Rust only
//! * No nightly features
//! * No external dependencies

#![forbid(unsafe_code)]

use std::fmt;

use crate::quantum::ir::{
    Gate,
    GateKind,
    QubitId,
    QuantumCircuit,
};
use crate::quantum::ir::parameter::Parameter;

use super::pattern::{
    CompiledPattern,
    PatternFingerprint,
    PatternId,
};
use super::rules::{
    ParameterConstraint,
    ParameterSlot,
    PatternOperation,
    QubitSlot,
};

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by the pattern matcher.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MatcherError {
    /// The supplied pattern contains no operations.
    EmptyPattern,

    /// The compiled pattern contains inconsistent derived metadata.
    InvalidPatternMetadata {
        /// Pattern operation involved.
        operation: usize,

        /// Static explanation.
        message: &'static str,
    },

    /// A matcher resource limit was reached.
    LimitExceeded {
        /// Resource name.
        resource: &'static str,

        /// Configured limit.
        limit: usize,

        /// Observed amount.
        actual: usize,
    },

    /// A dependency oracle rejected a candidate.
    DependencyRejected {
        /// Circuit operation index.
        operation: usize,
    },

    /// A commutation oracle rejected an attempted crossing.
    CommutationRejected {
        /// Circuit operation that could not be crossed.
        operation: usize,

        /// Pattern operation being matched.
        pattern_operation: usize,
    },

    /// An oracle could not provide the requested proof.
    OracleFailure {
        /// Oracle category.
        oracle: &'static str,

        /// Static explanation.
        message: &'static str,
    },
}

impl fmt::Display for MatcherError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyPattern => {
                formatter.write_str(
                    "cannot match an empty optimization pattern",
                )
            }

            Self::InvalidPatternMetadata {
                operation,
                message,
            } => {
                write!(
                    formatter,
                    "invalid pattern metadata at operation \
                     {operation}: {message}"
                )
            }

            Self::LimitExceeded {
                resource,
                limit,
                actual,
            } => {
                write!(
                    formatter,
                    "matcher {resource} limit exceeded: \
                     limit {limit}, actual {actual}"
                )
            }

            Self::DependencyRejected { operation } => {
                write!(
                    formatter,
                    "dependency oracle rejected circuit \
                     operation {operation}"
                )
            }

            Self::CommutationRejected {
                operation,
                pattern_operation,
            } => {
                write!(
                    formatter,
                    "commutation oracle rejected crossing \
                     circuit operation {operation} with \
                     pattern operation {pattern_operation}"
                )
            }

            Self::OracleFailure {
                oracle,
                message,
            } => {
                write!(
                    formatter,
                    "{oracle} oracle failure: {message}"
                )
            }
        }
    }
}

impl std::error::Error for MatcherError {}

/// Result type for matcher operations.
pub type MatcherResult<T> = Result<T, MatcherError>;

// =============================================================================
// Resource limits
// =============================================================================

/// Resource policy for one matcher invocation.
///
/// A value of `0` means unlimited.
///
/// These limits are local matcher controls. The optimizer-wide
/// `OptimizationLimits` remains responsible for global compilation budgets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MatcherLimits {
    /// Maximum number of candidate anchors examined.
    pub max_candidates: usize,

    /// Maximum number of circuit/pattern comparisons.
    pub max_comparisons: usize,

    /// Maximum number of successful matches returned.
    pub max_matches: usize,

    /// Maximum number of circuit operations inspected during
    /// commutation-aware matching.
    pub max_scanned_operations: usize,
}

impl MatcherLimits {
    /// Creates explicit matcher limits.
    #[must_use]
    pub const fn new(
        max_candidates: usize,
        max_comparisons: usize,
        max_matches: usize,
        max_scanned_operations: usize,
    ) -> Self {
        Self {
            max_candidates,
            max_comparisons,
            max_matches,
            max_scanned_operations,
        }
    }

    /// Creates unlimited matcher-local limits.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            max_candidates: 0,
            max_comparisons: 0,
            max_matches: 0,
            max_scanned_operations: 0,
        }
    }

    #[inline]
    fn check(
        limit: usize,
        actual: usize,
        resource: &'static str,
    ) -> MatcherResult<()> {
        if limit != 0 && actual > limit {
            return Err(MatcherError::LimitExceeded {
                resource,
                limit,
                actual,
            });
        }

        Ok(())
    }
}

impl Default for MatcherLimits {
    fn default() -> Self {
        Self::unlimited()
    }
}

// =============================================================================
// Numeric tolerance
// =============================================================================

/// Floating-point parameter matching policy.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MatchTolerance {
    /// Absolute tolerance.
    pub absolute: f64,

    /// Relative tolerance.
    pub relative: f64,
}

impl MatchTolerance {
    /// Creates a tolerance.
    ///
    /// Invalid values are normalized to zero rather than being accepted.
    #[must_use]
    pub fn new(
        absolute: f64,
        relative: f64,
    ) -> Self {
        Self {
            absolute: if absolute.is_finite() && absolute >= 0.0 {
                absolute
            } else {
                0.0
            },

            relative: if relative.is_finite() && relative >= 0.0 {
                relative
            } else {
                0.0
            },
        }
    }

    /// Exact floating-point equality.
    #[must_use]
    pub const fn exact() -> Self {
        Self {
            absolute: 0.0,
            relative: 0.0,
        }
    }

    /// Production compiler default.
    #[must_use]
    pub const fn compiler_default() -> Self {
        Self {
            absolute: 1.0e-12,
            relative: 1.0e-12,
        }
    }

    #[inline]
    fn equivalent(
        self,
        left: f64,
        right: f64,
    ) -> bool {
        if !left.is_finite() || !right.is_finite() {
            return false;
        }

        let delta = (left - right).abs();

        if delta <= self.absolute {
            return true;
        }

        let scale = left.abs().max(right.abs());

        delta <= self.absolute + self.relative * scale
    }
}

impl Default for MatchTolerance {
    fn default() -> Self {
        Self::compiler_default()
    }
}

// =============================================================================
// Matching modes
// =============================================================================

/// Pattern matching strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MatchMode {
    /// Exact contiguous matching.
    Exact,

    /// Exact contiguous matching plus dependency validation.
    DependencyAware,

    /// Dependency-safe matching across explicitly proven commuting
    /// operations.
    CommutationAware,
}

impl Default for MatchMode {
    fn default() -> Self {
        Self::Exact
    }
}

// =============================================================================
// Matcher options
// =============================================================================

/// Immutable matcher configuration.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MatcherOptions {
    /// Matching strategy.
    pub mode: MatchMode,

    /// Numeric parameter tolerance.
    pub tolerance: MatchTolerance,

    /// Resource limits.
    pub limits: MatcherLimits,

    /// Whether measurement/barrier/reset operations terminate ordinary
    /// matching regions.
    pub reject_semantic_boundaries: bool,
}

impl MatcherOptions {
    /// Creates conservative production defaults.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            mode: MatchMode::Exact,
            tolerance: MatchTolerance::compiler_default(),
            limits: MatcherLimits::unlimited(),
            reject_semantic_boundaries: true,
        }
    }

    /// Uses exact numerical parameter comparison.
    #[must_use]
    pub const fn with_exact_parameters(
        mut self,
    ) -> Self {
        self.tolerance = MatchTolerance::exact();
        self
    }

    /// Sets the matching mode.
    #[must_use]
    pub const fn with_mode(
        mut self,
        mode: MatchMode,
    ) -> Self {
        self.mode = mode;
        self
    }

    /// Sets matcher limits.
    #[must_use]
    pub const fn with_limits(
        mut self,
        limits: MatcherLimits,
    ) -> Self {
        self.limits = limits;
        self
    }

    /// Sets parameter matching tolerance.
    #[must_use]
    pub const fn with_tolerance(
        mut self,
        tolerance: MatchTolerance,
    ) -> Self {
        self.tolerance = tolerance;
        self
    }

    /// Enables or disables the ordinary semantic-boundary guard.
    ///
    /// This does not make a boundary crossable in commutation-aware mode.
    #[must_use]
    pub const fn with_boundary_rejection(
        mut self,
        enabled: bool,
    ) -> Self {
        self.reject_semantic_boundaries = enabled;
        self
    }
}

impl Default for MatcherOptions {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Concrete bindings
// =============================================================================

/// Concrete binding of a rule-local qubit slot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct QubitBinding {
    /// Rule-local slot.
    pub slot: QubitSlot,

    /// Concrete logical qubit.
    pub qubit: QubitId,
}

/// Concrete binding of a rule-local parameter slot.
#[derive(Debug, Clone, PartialEq)]
pub struct ParameterBinding {
    /// Rule-local parameter slot.
    pub slot: ParameterSlot,

    /// Concrete canonical IR parameter.
    pub parameter: Parameter,
}

/// Complete immutable binding environment.
#[derive(Debug, Clone, PartialEq)]
pub struct MatchBindings {
    qubits: Vec<QubitBinding>,
    parameters: Vec<ParameterBinding>,
}

impl MatchBindings {
    fn new(
        qubit_capacity: usize,
        parameter_capacity: usize,
    ) -> Self {
        Self {
            qubits: Vec::with_capacity(qubit_capacity),
            parameters: Vec::with_capacity(parameter_capacity),
        }
    }

    /// Returns all qubit bindings.
    #[must_use]
    pub fn qubits(&self) -> &[QubitBinding] {
        &self.qubits
    }

    /// Returns all parameter bindings.
    #[must_use]
    pub fn parameters(&self) -> &[ParameterBinding] {
        &self.parameters
    }

    /// Looks up a qubit binding.
    #[must_use]
    pub fn qubit(
        &self,
        slot: QubitSlot,
    ) -> Option<QubitId> {
        self.qubits
            .iter()
            .find(|binding| binding.slot == slot)
            .map(|binding| binding.qubit)
    }

    /// Looks up a parameter binding.
    #[must_use]
    pub fn parameter(
        &self,
        slot: ParameterSlot,
    ) -> Option<&Parameter> {
        self.parameters
            .iter()
            .find(|binding| binding.slot == slot)
            .map(|binding| &binding.parameter)
    }
}

// =============================================================================
// Match span
// =============================================================================

/// Circuit operation span associated with a successful match.
///
/// `start` is inclusive and `end` is exclusive.
///
/// For commutation-aware matching, the span includes intervening operations
/// that were inspected and proven movable. This is intentional: a rewrite
/// layer must never interpret a broad span as permission to delete those
/// operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MatchSpan {
    /// Inclusive start.
    pub start: usize,

    /// Exclusive end.
    pub end: usize,
}

impl MatchSpan {
    /// Creates a span.
    #[must_use]
    pub const fn new(
        start: usize,
        end: usize,
    ) -> Self {
        Self { start, end }
    }

    /// Number of operations in the span.
    #[must_use]
    pub const fn len(self) -> usize {
        self.end - self.start
    }

    /// Returns true when the span is empty.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.start == self.end
    }
}

// =============================================================================
// Successful match
// =============================================================================

/// Immutable result of matching one compiled pattern.
#[derive(Debug, Clone, PartialEq)]
pub struct PatternMatch {
    pattern_id: PatternId,
    fingerprint: PatternFingerprint,
    span: MatchSpan,
    matched_operations: Vec<usize>,
    bindings: MatchBindings,
}

impl PatternMatch {
    fn new(
        pattern: &CompiledPattern,
        span: MatchSpan,
        matched_operations: Vec<usize>,
        bindings: MatchBindings,
    ) -> Self {
        Self {
            pattern_id: pattern.id(),
            fingerprint: pattern.fingerprint(),
            span,
            matched_operations,
            bindings,
        }
    }

    /// Stable pattern identifier.
    #[must_use]
    pub const fn pattern_id(
        &self,
    ) -> PatternId {
        self.pattern_id
    }

    /// Deterministic pattern fingerprint.
    #[must_use]
    pub const fn fingerprint(
        &self,
    ) -> PatternFingerprint {
        self.fingerprint
    }

    /// Circuit span inspected by this match.
    #[must_use]
    pub const fn span(
        &self,
    ) -> MatchSpan {
        self.span
    }

    /// Circuit operation positions corresponding to pattern operations.
    #[must_use]
    pub fn matched_operations(
        &self,
    ) -> &[usize] {
        &self.matched_operations
    }

    /// Concrete bindings.
    #[must_use]
    pub fn bindings(
        &self,
    ) -> &MatchBindings {
        &self.bindings
    }
}

// =============================================================================
// Dependency oracle
// =============================================================================

/// External dependency-safety proof used by dependency-aware matching.
///
/// The matcher does not implement whole-circuit dependency analysis. This
/// keeps dependency semantics owned by `analysis::dependency`.
pub trait DependencyOracle {
    /// Returns whether a candidate operation may satisfy the pattern position.
    fn permits(
        &self,
        circuit: &QuantumCircuit,
        circuit_index: usize,
        pattern_index: usize,
    ) -> bool;
}

/// Dependency oracle for callers that already established safety.
#[derive(Debug, Clone, Copy, Default)]
pub struct AllowAllDependencies;

impl DependencyOracle for AllowAllDependencies {
    fn permits(
        &self,
        _circuit: &QuantumCircuit,
        _circuit_index: usize,
        _pattern_index: usize,
    ) -> bool {
        true
    }
}

// =============================================================================
// Commutation oracle
// =============================================================================

/// External commutation-safety proof.
///
/// The matcher deliberately does not guess whether two quantum operations
/// commute.
///
/// `pattern_gate` and `pattern_qubits` describe the operation currently being
/// matched. `intervening` is the circuit operation that would have to cross it.
///
/// Implementations must return `false` whenever the movement is not proven
/// semantics-preserving under the active optimization equivalence policy.
pub trait CommutationOracle {
    /// Returns whether `intervening` may cross the pattern operation.
    fn can_cross(
        &self,
        intervening: &Gate,
        pattern_gate: GateKind,
        pattern_qubits: &[QubitId],
    ) -> bool;
}

/// Oracle that never permits commutation.
#[derive(Debug, Clone, Copy, Default)]
pub struct NoCommutation;

impl CommutationOracle for NoCommutation {
    fn can_cross(
        &self,
        _intervening: &Gate,
        _pattern_gate: GateKind,
        _pattern_qubits: &[QubitId],
    ) -> bool {
        false
    }
}

/// Conservative commutation oracle for disjoint logical qubits.
///
/// Operations on disjoint qubits commute. Operations sharing a qubit are not
/// assumed to commute here and must be handled by the dedicated commutation
/// analysis subsystem.
#[derive(Debug, Clone, Copy, Default)]
pub struct DisjointQubitCommutation;

impl CommutationOracle for DisjointQubitCommutation {
    fn can_cross(
        &self,
        intervening: &Gate,
        pattern_gate: GateKind,
        pattern_qubits: &[QubitId],
    ) -> bool {
        if is_semantic_boundary(intervening)
            || !pattern_gate.is_unitary()
        {
            return false;
        }

        intervening
            .qubits()
            .iter()
            .all(|qubit| !pattern_qubits.contains(qubit))
    }
}

// =============================================================================
// Matcher
// =============================================================================

/// Production immutable pattern matcher.
#[derive(Debug, Clone)]
pub struct PatternMatcher {
    options: MatcherOptions,
}

impl PatternMatcher {
    /// Creates a matcher with production defaults.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            options: MatcherOptions::new(),
        }
    }

    /// Creates a matcher with explicit options.
    #[must_use]
    pub const fn with_options(
        options: MatcherOptions,
    ) -> Self {
        Self { options }
    }

    /// Returns matcher configuration.
    #[must_use]
    pub const fn options(
        &self,
    ) -> &MatcherOptions {
        &self.options
    }

    /// Matches using the configured mode.
    ///
    /// `CommutationAware` requires an explicit commutation oracle and therefore
    /// returns an error through this convenience method rather than silently
    /// using an unsafe commutation assumption.
    pub fn find(
        &self,
        circuit: &QuantumCircuit,
        pattern: &CompiledPattern,
    ) -> MatcherResult<Vec<PatternMatch>> {
        match self.options.mode {
            MatchMode::Exact => {
                self.find_exact(circuit, pattern)
            }

            MatchMode::DependencyAware => {
                self.find_with_dependency_oracle(
                    circuit,
                    pattern,
                    &AllowAllDependencies,
                )
            }

            MatchMode::CommutationAware => {
                Err(MatcherError::OracleFailure {
                    oracle: "commutation",
                    message:
                        "CommutationAware mode requires \
                         find_with_commutation_oracle",
                })
            }
        }
    }

    /// Finds every exact contiguous match.
    pub fn find_exact(
        &self,
        circuit: &QuantumCircuit,
        pattern: &CompiledPattern,
    ) -> MatcherResult<Vec<PatternMatch>> {
        self.find_exact_with_dependency_oracle(
            circuit,
            pattern,
            &AllowAllDependencies,
        )
    }

    /// Finds exact matches while consulting an external dependency oracle.
    pub fn find_with_dependency_oracle<O>(
        &self,
        circuit: &QuantumCircuit,
        pattern: &CompiledPattern,
        oracle: &O,
    ) -> MatcherResult<Vec<PatternMatch>>
    where
        O: DependencyOracle + ?Sized,
    {
        self.find_exact_with_dependency_oracle(
            circuit,
            pattern,
            oracle,
        )
    }

    /// Finds commutation-aware matches using an explicit oracle.
    pub fn find_with_commutation_oracle<O>(
        &self,
        circuit: &QuantumCircuit,
        pattern: &CompiledPattern,
        oracle: &O,
    ) -> MatcherResult<Vec<PatternMatch>>
    where
        O: CommutationOracle + ?Sized,
    {
        validate_pattern_for_matching(pattern)?;

        if pattern.is_empty() {
            return Err(MatcherError::EmptyPattern);
        }

        let first_operation =
            pattern
                .operation(0)
                .ok_or(
                    MatcherError::InvalidPatternMetadata {
                        operation: 0,
                        message:
                            "first pattern operation \
                             is unavailable",
                    },
                )?;

        let first_gate = first_operation.gate;

        let mut matches = Vec::new();

        let mut candidates = 0usize;
        let mut comparisons = 0usize;
        let mut scanned = 0usize;

        for start in 0..circuit.len() {
            if self.options.limits.max_scanned_operations != 0
                && scanned
                    >= self
                        .options
                        .limits
                        .max_scanned_operations
            {
                return Err(
                    MatcherError::LimitExceeded {
                        resource:
                            "scanned_operations",
                        limit: self
                            .options
                            .limits
                            .max_scanned_operations,
                        actual: scanned,
                    },
                );
            }

            scanned += 1;

            let Some(gate) = circuit.get(start) else {
                break;
            };

            if gate.kind() != first_gate {
                continue;
            }

            candidates = candidates
                .checked_add(1)
                .ok_or(
                    MatcherError::LimitExceeded {
                        resource:
                            "candidate_overflow",
                        limit: usize::MAX,
                        actual: usize::MAX,
                    },
                )?;

            MatcherLimits::check(
                self.options
                    .limits
                    .max_candidates,
                candidates,
                "candidates",
            )?;

            if let Some(found) =
                self.match_commuting_from(
                    circuit,
                    pattern,
                    start,
                    oracle,
                    &mut comparisons,
                )?
            {
                matches.push(found);

                MatcherLimits::check(
                    self.options
                        .limits
                        .max_matches,
                    matches.len(),
                    "matches",
                )?;
            }
        }

        Ok(matches)
    }

    /// Returns the first exact match.
    pub fn first_exact(
        &self,
        circuit: &QuantumCircuit,
        pattern: &CompiledPattern,
    ) -> MatcherResult<Option<PatternMatch>> {
        self.first_exact_with_dependency_oracle(
            circuit,
            pattern,
            &AllowAllDependencies,
        )
    }

    /// Returns the first exact match satisfying a dependency oracle.
    pub fn first_exact_with_dependency_oracle<O>(
        &self,
        circuit: &QuantumCircuit,
        pattern: &CompiledPattern,
        oracle: &O,
    ) -> MatcherResult<Option<PatternMatch>>
    where
        O: DependencyOracle + ?Sized,
    {
        validate_pattern_for_matching(pattern)?;

        let pattern_len = pattern.len();

        if pattern_len == 0 {
            return Err(MatcherError::EmptyPattern);
        }

        if pattern_len > circuit.len() {
            return Ok(None);
        }

        let anchor = pattern
            .anchor_operation()
            .unwrap_or(0);

        let anchor_metadata =
            pattern
                .operation(anchor)
                .ok_or(
                    MatcherError::InvalidPatternMetadata {
                        operation: anchor,
                        message:
                            "anchor operation is missing",
                    },
                )?;

        let anchor_gate = anchor_metadata.gate;

        let max_start =
            circuit.len() - pattern_len;

        let mut candidates = 0usize;
        let mut comparisons = 0usize;

        for anchor_index in
            anchor..=max_start.saturating_add(anchor)
        {
            let start =
                anchor_index.saturating_sub(anchor);

            if start > max_start {
                break;
            }

            let Some(gate) =
                circuit.get(anchor_index)
            else {
                break;
            };

            if gate.kind() != anchor_gate {
                continue;
            }

            candidates = candidates
                .checked_add(1)
                .ok_or(
                    MatcherError::LimitExceeded {
                        resource:
                            "candidate_overflow",
                        limit: usize::MAX,
                        actual: usize::MAX,
                    },
                )?;

            MatcherLimits::check(
                self.options
                    .limits
                    .max_candidates,
                candidates,
                "candidates",
            )?;

            if let Some(found) =
                self.match_exact_at(
                    circuit,
                    pattern,
                    start,
                    oracle,
                    &mut comparisons,
                )?
            {
                return Ok(Some(found));
            }
        }

        Ok(None)
    }

    fn find_exact_with_dependency_oracle<O>(
        &self,
        circuit: &QuantumCircuit,
        pattern: &CompiledPattern,
        oracle: &O,
    ) -> MatcherResult<Vec<PatternMatch>>
    where
        O: DependencyOracle + ?Sized,
    {
        validate_pattern_for_matching(pattern)?;

        if pattern.is_empty() {
            return Err(MatcherError::EmptyPattern);
        }

        if pattern.len() > circuit.len() {
            return Ok(Vec::new());
        }

        let anchor =
            pattern
                .anchor_operation()
                .unwrap_or(0);

        let anchor_gate =
            pattern
                .operation(anchor)
                .ok_or(
                    MatcherError::InvalidPatternMetadata {
                        operation: anchor,
                        message:
                            "anchor operation is missing",
                    },
                )?
                .gate;

        let max_start =
            circuit.len() - pattern.len();

        let mut matches = Vec::new();
        let mut candidates = 0usize;
        let mut comparisons = 0usize;

        for anchor_index in
            anchor..=max_start.saturating_add(anchor)
        {
            let start =
                anchor_index.saturating_sub(anchor);

            if start > max_start {
                break;
            }

            let Some(gate) =
                circuit.get(anchor_index)
            else {
                break;
            };

            if gate.kind() != anchor_gate {
                continue;
            }

            candidates = candidates
                .checked_add(1)
                .ok_or(
                    MatcherError::LimitExceeded {
                        resource:
                            "candidate_overflow",
                        limit: usize::MAX,
                        actual: usize::MAX,
                    },
                )?;

            MatcherLimits::check(
                self.options
                    .limits
                    .max_candidates,
                candidates,
                "candidates",
            )?;

            if let Some(found) =
                self.match_exact_at(
                    circuit,
                    pattern,
                    start,
                    oracle,
                    &mut comparisons,
                )?
            {
                matches.push(found);

                MatcherLimits::check(
                    self.options
                        .limits
                        .max_matches,
                    matches.len(),
                    "matches",
                )?;
            }
        }

        Ok(matches)
    }

    fn match_exact_at<O>(
        &self,
        circuit: &QuantumCircuit,
        pattern: &CompiledPattern,
        start: usize,
        oracle: &O,
        comparisons: &mut usize,
    ) -> MatcherResult<Option<PatternMatch>>
    where
        O: DependencyOracle + ?Sized,
    {
        let pattern_len = pattern.len();

        let end =
            start
                .checked_add(pattern_len)
                .ok_or(
                    MatcherError::LimitExceeded {
                        resource:
                            "index_overflow",
                        limit: usize::MAX,
                        actual: usize::MAX,
                    },
                )?;

        if end > circuit.len() {
            return Ok(None);
        }

        let mut bindings =
            MatchBindings::new(
                pattern.statistics().qubit_slots,
                pattern.statistics().parameter_slots,
            );

        let mut matched_operations =
            Vec::with_capacity(pattern_len);

        for pattern_index in 0..pattern_len {
            let circuit_index =
                start + pattern_index;

            let gate =
                circuit
                    .get(circuit_index)
                    .ok_or(
                        MatcherError::InvalidPatternMetadata {
                            operation: pattern_index,
                            message:
                                "circuit operation is unavailable",
                        },
                    )?;

            let metadata =
                pattern
                    .operation(pattern_index)
                    .ok_or(
                        MatcherError::InvalidPatternMetadata {
                            operation: pattern_index,
                            message:
                                "pattern operation metadata is unavailable",
                        },
                    )?;

            if self.options.reject_semantic_boundaries
                && is_semantic_boundary(gate)
            {
                return Ok(None);
            }

            if gate.kind() != metadata.gate {
                return Ok(None);
            }

            *comparisons =
                comparisons
                    .checked_add(1)
                    .ok_or(
                        MatcherError::LimitExceeded {
                            resource:
                                "comparison_overflow",
                            limit: usize::MAX,
                            actual: usize::MAX,
                        },
                    )?;

            MatcherLimits::check(
                self.options
                    .limits
                    .max_comparisons,
                *comparisons,
                "comparisons",
            )?;

            if !oracle.permits(
                circuit,
                circuit_index,
                pattern_index,
            ) {
                return Ok(None);
            }

            let pattern_operation =
                pattern
                    .rule_pattern()
                    .operations
                    .get(pattern_index)
                    .ok_or(
                        MatcherError::InvalidPatternMetadata {
                            operation: pattern_index,
                            message:
                                "source pattern operation \
                                 is unavailable",
                        },
                    )?;

            if !match_operation(
                gate,
                pattern_operation,
                &mut bindings,
                self.options.tolerance,
            ) {
                return Ok(None);
            }

            matched_operations.push(
                circuit_index,
            );
        }

        Ok(Some(
            PatternMatch::new(
                pattern,
                MatchSpan::new(start, end),
                matched_operations,
                bindings,
            ),
        ))
    }

    fn match_commuting_from<O>(
        &self,
        circuit: &QuantumCircuit,
        pattern: &CompiledPattern,
        start: usize,
        oracle: &O,
        comparisons: &mut usize,
    ) -> MatcherResult<Option<PatternMatch>>
    where
        O: CommutationOracle + ?Sized,
    {
        let mut bindings =
            MatchBindings::new(
                pattern.statistics().qubit_slots,
                pattern.statistics().parameter_slots,
            );

        let mut matched_operations =
            Vec::with_capacity(pattern.len());

        let mut cursor = start;
        let mut scanned = 0usize;

        for pattern_index in 0..pattern.len() {
            let pattern_operation =
                pattern
                    .rule_pattern()
                    .operations
                    .get(pattern_index)
                    .ok_or(
                        MatcherError::InvalidPatternMetadata {
                            operation: pattern_index,
                            message:
                                "source pattern operation \
                                 is unavailable",
                        },
                    )?;

            let mut found = false;

            while cursor < circuit.len() {
                if self.options.limits.max_scanned_operations != 0
                    && scanned
                        >= self
                            .options
                            .limits
                            .max_scanned_operations
                {
                    return Err(
                        MatcherError::LimitExceeded {
                            resource:
                                "scanned_operations",
                            limit: self
                                .options
                                .limits
                                .max_scanned_operations,
                            actual: scanned,
                        },
                    );
                }

                scanned += 1;

                let gate =
                    circuit
                        .get(cursor)
                        .ok_or(
                            MatcherError::InvalidPatternMetadata {
                                operation:
                                    pattern_index,
                                message:
                                    "circuit operation is unavailable",
                            },
                        )?;

                if is_semantic_boundary(gate) {
                    return Ok(None);
                }

                *comparisons =
                    comparisons
                        .checked_add(1)
                        .ok_or(
                            MatcherError::LimitExceeded {
                                resource:
                                    "comparison_overflow",
                                limit: usize::MAX,
                                actual: usize::MAX,
                            },
                        )?;

                MatcherLimits::check(
                    self.options
                        .limits
                        .max_comparisons,
                    *comparisons,
                    "comparisons",
                )?;

                if gate.kind()
                    == pattern_operation.gate
                    && match_operation(
                        gate,
                        pattern_operation,
                        &mut bindings,
                        self.options.tolerance,
                    )
                {
                    matched_operations.push(cursor);

                    cursor += 1;
                    found = true;
                    break;
                }

                let Some(pattern_qubits) =
                    bound_pattern_qubits(
                        pattern_operation,
                        &bindings,
                    )
                else {
                    return Ok(None);
                };

                if !oracle.can_cross(
                    gate,
                    pattern_operation.gate,
                    &pattern_qubits,
                ) {
                    return Ok(None);
                }

                cursor += 1;
            }

            if !found {
                return Ok(None);
            }
        }

        Ok(Some(
            PatternMatch::new(
                pattern,
                MatchSpan::new(start, cursor),
                matched_operations,
                bindings,
            ),
        ))
    }
}

impl Default for PatternMatcher {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Free-function API
// =============================================================================

/// Finds exact matches using production defaults.
pub fn find_matches(
    circuit: &QuantumCircuit,
    pattern: &CompiledPattern,
) -> MatcherResult<Vec<PatternMatch>> {
    PatternMatcher::new()
        .find_exact(circuit, pattern)
}

/// Finds matches using explicit matcher options.
pub fn find_matches_with_options(
    circuit: &QuantumCircuit,
    pattern: &CompiledPattern,
    options: MatcherOptions,
) -> MatcherResult<Vec<PatternMatch>> {
    PatternMatcher::with_options(options)
        .find(circuit, pattern)
}

/// Returns the first exact match.
pub fn find_first_match(
    circuit: &QuantumCircuit,
    pattern: &CompiledPattern,
) -> MatcherResult<Option<PatternMatch>> {
    PatternMatcher::new()
        .first_exact(circuit, pattern)
}

// =============================================================================
// Structural validation
// =============================================================================

fn validate_pattern_for_matching(
    pattern: &CompiledPattern,
) -> MatcherResult<()> {
    if pattern.is_empty() {
        return Err(MatcherError::EmptyPattern);
    }

    if pattern.operations().len()
        != pattern.len()
    {
        return Err(
            MatcherError::InvalidPatternMetadata {
                operation:
                    pattern.operations().len(),
                message:
                    "compiled operation metadata \
                     length differs from pattern length",
            },
        );
    }

    let source =
        pattern.rule_pattern();

    if source.operations.len()
        != pattern.len()
    {
        return Err(
            MatcherError::InvalidPatternMetadata {
                operation:
                    source.operations.len(),
                message:
                    "source pattern length differs \
                     from compiled length",
            },
        );
    }

    for (index, metadata) in
        pattern.operations().iter().enumerate()
    {
        let source_operation =
            &source.operations[index];

        if metadata.index != index {
            return Err(
                MatcherError::InvalidPatternMetadata {
                    operation: index,
                    message:
                        "operation metadata index \
                         is not canonical",
                },
            );
        }

        if metadata.gate
            != source_operation.gate
        {
            return Err(
                MatcherError::InvalidPatternMetadata {
                    operation: index,
                    message:
                        "compiled gate kind differs \
                         from source pattern",
                },
            );
        }

        if metadata.qubit_arity
            != source_operation.qubits.len()
            || metadata.parameter_arity
                != source_operation.parameters.len()
        {
            return Err(
                MatcherError::InvalidPatternMetadata {
                    operation: index,
                    message:
                        "compiled operand metadata \
                         differs from source pattern",
                },
            );
        }
    }

    Ok(())
}

// =============================================================================
// Operation matching
// =============================================================================

fn match_operation(
    gate: &Gate,
    pattern: &PatternOperation,
    bindings: &mut MatchBindings,
    tolerance: MatchTolerance,
) -> bool {
    if gate.kind() != pattern.gate {
        return false;
    }

    if gate.qubits().len()
        != pattern.qubits.len()
    {
        return false;
    }

    if gate.parameters().len()
        != pattern.parameters.len()
    {
        return false;
    }

    // -------------------------------------------------------------------------
    // Qubit bindings
    // -------------------------------------------------------------------------

    for (
        actual,
        slot,
    ) in gate
        .qubits()
        .iter()
        .copied()
        .zip(pattern.qubits.iter().copied())
    {
        if let Some(existing) =
            bindings.qubit(slot)
        {
            if existing != actual {
                return false;
            }
        } else {
            bindings.qubits.push(
                QubitBinding {
                    slot,
                    qubit: actual,
                },
            );
        }
    }

    // -------------------------------------------------------------------------
    // Parameter bindings
    // -------------------------------------------------------------------------

    for (
        actual,
        slot,
    ) in gate
        .parameters()
        .iter()
        .cloned()
        .zip(pattern.parameters.iter().copied())
    {
        if let Some(existing) =
            bindings.parameter(slot)
        {
            if !parameters_equivalent(
                existing,
                &actual,
                tolerance,
            ) {
                return false;
            }
        } else {
            bindings.parameters.push(
                ParameterBinding {
                    slot,
                    parameter: actual,
                },
            );
        }
    }

    // -------------------------------------------------------------------------
    // Parameter constraints
    // -------------------------------------------------------------------------

    if let Some(constraint) =
        pattern.parameter_constraint
    {
        if !constraint_holds(
            constraint,
            bindings,
            tolerance,
        ) {
            return false;
        }
    }

    true
}

// =============================================================================
// Parameter matching
// =============================================================================

fn parameters_equivalent(
    left: &Parameter,
    right: &Parameter,
    tolerance: MatchTolerance,
) -> bool {
    match (left, right) {
        (
            Parameter::Constant(left),
            Parameter::Constant(right),
        ) => {
            tolerance.equivalent(
                *left,
                *right,
            )
        }

        // Symbolic and expression parameters are compared structurally.
        //
        // The parameter IR is already responsible for canonical expression
        // structure. The matcher therefore does not invent an independent
        // symbolic algebra.
        _ => left == right,
    }
}

fn constraint_holds(
    constraint: ParameterConstraint,
    bindings: &MatchBindings,
    tolerance: MatchTolerance,
) -> bool {
    match constraint {
        ParameterConstraint::Any => true,

        ParameterConstraint::Zero(slot) => {
            bindings
                .parameter(slot)
                .and_then(Parameter::as_constant)
                .map(|value| {
                    tolerance.equivalent(
                        value,
                        0.0,
                    )
                })
                .unwrap_or(false)
        }

        ParameterConstraint::Constant {
            slot,
            value,
        } => {
            if !value.is_finite() {
                return false;
            }

            bindings
                .parameter(slot)
                .and_then(Parameter::as_constant)
                .map(|actual| {
                    tolerance.equivalent(
                        actual,
                        value,
                    )
                })
                .unwrap_or(false)
        }

        ParameterConstraint::Equal {
            left,
            right,
        } => {
            match (
                bindings.parameter(left),
                bindings.parameter(right),
            ) {
                (Some(left), Some(right)) => {
                    parameters_equivalent(
                        left,
                        right,
                        tolerance,
                    )
                }

                _ => false,
            }
        }

        ParameterConstraint::NegationPair {
            left,
            right,
        } => {
            match (
                bindings.parameter(left),
                bindings.parameter(right),
            ) {
                (
                    Some(Parameter::Constant(left)),
                    Some(Parameter::Constant(right)),
                ) => {
                    tolerance.equivalent(
                        *left + *right,
                        0.0,
                    )
                }

                // Do not guess symbolic negation here. A later parameter
                // algebra layer can normalize symbolic expressions before
                // matching.
                _ => false,
            }
        }
    }
}

// =============================================================================
// Commutation helpers
// =============================================================================

fn bound_pattern_qubits(
    operation: &PatternOperation,
    bindings: &MatchBindings,
) -> Option<Vec<QubitId>> {
    let mut result =
        Vec::with_capacity(
            operation.qubits.len(),
        );

    for slot in operation.qubits {
        let qubit =
            bindings.qubit(*slot)?;

        result.push(qubit);
    }

    Some(result)
}

fn is_semantic_boundary(
    gate: &Gate,
) -> bool {
    gate.is_measurement()
        || gate.is_barrier()
        || gate.is_reset()
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::quantum::ir::{
        Gate,
        GateKind,
        QubitId,
        QuantumCircuit,
    };
    use crate::quantum::ir::parameter::Parameter;

    use crate::quantum::optimization::pattern::{
        CompiledPattern,
        PatternLimits,
    };

    use crate::quantum::optimization::rules::{
        ParameterSlot,
        PatternOperation,
        QubitSlot,
        RulePattern,
    };

    const Q0: [QubitSlot; 1] = [
        QubitSlot::new(0),
    ];

    const P0: [ParameterSlot; 1] = [
        ParameterSlot::new(0),
    ];

    const H: PatternOperation =
        PatternOperation::new(
            GateKind::H,
            &Q0,
            &[],
            None,
        );

    const X: PatternOperation =
        PatternOperation::new(
            GateKind::X,
            &Q0,
            &[],
            None,
        );

    const RX: PatternOperation =
        PatternOperation::new(
            GateKind::RX,
            &Q0,
            &P0,
            None,
        );

    const HX: [PatternOperation; 2] = [
        H,
        X,
    ];

    const RX_PATTERN: [PatternOperation; 1] = [
        RX,
    ];

    fn compile_pattern(
        operations: &'static [PatternOperation],
        qubit_slots: usize,
        parameter_slots: usize,
    ) -> CompiledPattern {
        CompiledPattern::compile(
            RulePattern::new(
                operations,
                qubit_slots,
                parameter_slots,
            ),
            PatternLimits::unlimited(),
        )
        .expect(
            "test optimization pattern \
             must compile",
        )
    }

    fn gate(
        kind: GateKind,
        qubits: &[usize],
    ) -> Gate {
        Gate::new(
            kind,
            qubits
                .iter()
                .copied()
                .map(QubitId::new)
                .collect(),
            Vec::new(),
            None,
            None,
        )
        .expect("test gate must be valid")
    }

    fn rotation(
        angle: f64,
    ) -> Gate {
        Gate::new(
            GateKind::RX,
            vec![QubitId::new(0)],
            vec![
                Parameter::constant(angle)
                    .expect("finite"),
            ],
            None,
            None,
        )
        .expect("test rotation must be valid")
    }

    #[test]
    fn exact_match_returns_span_and_bindings() {
        let circuit =
            QuantumCircuit::from_operations(
                1,
                0,
                vec![
                    gate(
                        GateKind::H,
                        &[0],
                    ),
                    gate(
                        GateKind::X,
                        &[0],
                    ),
                ],
            )
            .expect(
                "circuit must be valid",
            );

        let pattern =
            compile_pattern(
                &HX,
                1,
                0,
            );

        let matches =
            find_matches(
                &circuit,
                &pattern,
            )
            .expect(
                "matching must succeed",
            );

        assert_eq!(
            matches.len(),
            1
        );

        assert_eq!(
            matches[0].span(),
            MatchSpan::new(0, 2)
        );

        assert_eq!(
            matches[0]
                .matched_operations(),
            &[0, 1]
        );

        assert_eq!(
            matches[0]
                .bindings()
                .qubit(
                    QubitSlot::new(0)
                ),
            Some(QubitId::new(0))
        );
    }

    #[test]
    fn repeated_pattern_qubit_binding_is_consistent() {
        const CX: [QubitSlot; 2] = [
            QubitSlot::new(0),
            QubitSlot::new(1),
        ];

        const CX_PATTERN: [PatternOperation; 1] = [
            PatternOperation::new(
                GateKind::CX,
                &CX,
                &[],
                None,
            ),
        ];

        let circuit =
            QuantumCircuit::from_operations(
                2,
                0,
                vec![
                    gate(
                        GateKind::CX,
                        &[0, 1],
                    ),
                ],
            )
            .expect(
                "circuit must be valid",
            );

        let pattern =
            compile_pattern(
                &CX_PATTERN,
                2,
                0,
            );

        let result =
            find_first_match(
                &circuit,
                &pattern,
            )
            .expect(
                "matching must succeed",
            )
            .expect(
                "CX must match",
            );

        assert_eq!(
            result
                .bindings()
                .qubit(
                    QubitSlot::new(0)
                ),
            Some(QubitId::new(0))
        );

        assert_eq!(
            result
                .bindings()
                .qubit(
                    QubitSlot::new(1)
                ),
            Some(QubitId::new(1))
        );
    }

    #[test]
    fn parameter_binding_is_preserved() {
        let circuit =
            QuantumCircuit::from_operations(
                1,
                0,
                vec![
                    rotation(0.25),
                ],
            )
            .expect(
                "circuit must be valid",
            );

        let pattern =
            compile_pattern(
                &RX_PATTERN,
                1,
                1,
            );

        let result =
            find_first_match(
                &circuit,
                &pattern,
            )
            .expect(
                "matching must succeed",
            )
            .expect(
                "RX must match",
            );

        assert_eq!(
            result
                .bindings()
                .parameter(
                    ParameterSlot::new(0)
                ),
            Some(
                &Parameter::Constant(
                    0.25
                )
            )
        );
    }

    #[test]
    fn measurement_is_a_boundary() {
        let measurement =
            Gate::new(
                GateKind::Measure,
                vec![
                    QubitId::new(0),
                ],
                Vec::new(),
                Some(0),
                None,
            )
            .expect(
                "measurement must be valid",
            );

        let circuit =
            QuantumCircuit::from_operations(
                1,
                1,
                vec![
                    gate(
                        GateKind::H,
                        &[0],
                    ),
                    measurement,
                    gate(
                        GateKind::X,
                        &[0],
                    ),
                ],
            )
            .expect(
                "circuit must be valid",
            );

        let pattern =
            compile_pattern(
                &HX,
                1,
                0,
            );

        let matches =
            find_matches(
                &circuit,
                &pattern,
            )
            .expect(
                "matching must succeed",
            );

        assert!(
            matches.is_empty()
        );
    }

    #[test]
    fn disjoint_operations_can_be_crossed() {
        const HX0: [PatternOperation; 2] = [
            PatternOperation::new(
                GateKind::H,
                &Q0,
                &[],
                None,
            ),
            PatternOperation::new(
                GateKind::X,
                &Q0,
                &[],
                None,
            ),
        ];

        let circuit =
            QuantumCircuit::from_operations(
                2,
                0,
                vec![
                    gate(
                        GateKind::H,
                        &[0],
                    ),
                    gate(
                        GateKind::Z,
                        &[1],
                    ),
                    gate(
                        GateKind::X,
                        &[0],
                    ),
                ],
            )
            .expect(
                "circuit must be valid",
            );

        let pattern =
            compile_pattern(
                &HX0,
                1,
                0,
            );

        let matcher =
            PatternMatcher::new();

        let matches =
            matcher
                .find_with_commutation_oracle(
                    &circuit,
                    &pattern,
                    &DisjointQubitCommutation,
                )
                .expect(
                    "commutation matching \
                     must succeed",
                );

        assert_eq!(
            matches.len(),
            1
        );

        assert_eq!(
            matches[0]
                .matched_operations(),
            &[0, 2]
        );

        assert_eq!(
            matches[0].span(),
            MatchSpan::new(0, 3)
        );
    }

    #[test]
    fn matcher_limits_are_enforced() {
        const H_PATTERN: [PatternOperation; 1] = [
            H,
        ];

        let circuit =
            QuantumCircuit::from_operations(
                1,
                0,
                vec![
                    gate(
                        GateKind::H,
                        &[0],
                    ),
                    gate(
                        GateKind::H,
                        &[0],
                    ),
                ],
            )
            .expect(
                "circuit must be valid",
            );

        let pattern =
            compile_pattern(
                &H_PATTERN,
                1,
                0,
            );

        let options =
            MatcherOptions::new()
                .with_limits(
                    MatcherLimits::new(
                        1,
                        0,
                        0,
                        0,
                    ),
                );

        let result =
            PatternMatcher::with_options(
                options,
            )
            .find_exact(
                &circuit,
                &pattern,
            );

        assert!(
            result.is_err()
        );
    }
}