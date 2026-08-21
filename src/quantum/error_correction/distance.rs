//! Zamani Quantum Error Correction — resource-bounded code-distance verification.
//!
//! The distance of a stabilizer code is
//!
//!     d = min { wt(P) | P ∈ N(S) \ S }
//!
//! This module owns the verification/search of that quantity.
//!
//! It does NOT own:
//!
//! - surface-code topology;
//! - generic stabilizer algebra;
//! - decoding;
//! - MWPM;
//! - Union-Find;
//! - QPU execution;
//! - runtime resource accounting;
//! - checkpoint persistence;
//! - telemetry transport.
//!
//! Integration:
//!
//! ```text
//! limits.rs
//!      │
//!      ├── max_logical_operator_weight
//!      └── max_verification_operations
//!              │
//!              ▼
//! cancellation.rs ───────► distance.rs
//!                              │
//!                              ▼
//!                         stabilizer.rs
//!                              │
//!                 ┌────────────┴────────────┐
//!                 ▼                         ▼
//!          surface_code.rs           verification.rs
//! ```
//!
//! Mathematical guarantees:
//!
//! 1. A declared distance is never trusted.
//! 2. An exact result requires exhaustive rejection of every lower weight.
//! 3. A resource-limited search is never reported as exact.
//! 4. Cancellation is never reported as success.
//! 5. Enumeration is deterministic.
//! 6. Candidate counters use checked arithmetic.
//! 7. Candidate memory is checked before allocation.
//! 8. Stabilizer membership is delegated to `stabilizer.rs`.
//! 9. Normalizer membership is delegated to `stabilizer.rs`.
//! 10. Verification work is governed by `max_verification_operations`, not
//!     decoder-specific iteration policy.
//!
//! Rust compatibility: Rust 1.97.1.

use core::fmt;
use std::time::{Duration, Instant};

use super::cancellation::CancellationToken;
use super::limits::{LimitError, LimitKind, QecLimits};
use super::stabilizer::{Pauli, PauliString, StabilizerError, StabilizerGroup};

// ============================================================================
// Verification status
// ============================================================================

/// Mathematical status of a distance verification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DistanceStatus {
    /// The minimum distance was exhaustively established.
    VerifiedExact,

    /// A lower bound has been established, but the exact distance is unknown.
    VerifiedLowerBound,

    /// A valid logical witness establishes an upper bound.
    VerifiedUpperBound,

    /// No mathematical bound is available.
    Unverified,

    /// Verification stopped because a configured resource boundary was hit.
    ResourceLimited,

    /// Verification stopped because cancellation was requested.
    Cancelled,
}

impl DistanceStatus {
    #[must_use]
    pub const fn is_exact(self) -> bool {
        matches!(self, Self::VerifiedExact)
    }

    #[must_use]
    pub const fn is_verified(self) -> bool {
        matches!(
            self,
            Self::VerifiedExact
                | Self::VerifiedLowerBound
                | Self::VerifiedUpperBound
        )
    }
}

// ============================================================================
// Search configuration
// ============================================================================

/// Controls one distance-verification operation.
#[derive(Clone, Debug)]
pub struct DistanceOptions {
    /// Highest Pauli weight that may be exhaustively searched.
    pub max_weight: Option<usize>,

    /// Maximum number of candidate Pauli operators evaluated.
    pub max_operations: Option<u64>,

    /// Maximum memory budget for distance-search working state.
    pub max_memory_bytes: Option<u64>,

    /// Optional explicit wall-clock limit.
    ///
    /// `QecLimits` currently has no dedicated verification-time field, so this
    /// is intentionally caller-controlled rather than silently reusing the
    /// decoder-time policy.
    pub max_time: Option<Duration>,

    /// Optional cooperative cancellation token.
    pub cancellation: Option<CancellationToken>,

    /// Requests deterministic execution.
    ///
    /// The current implementation is always deterministic.
    pub deterministic: bool,
}

impl Default for DistanceOptions {
    fn default() -> Self {
        Self {
            max_weight: None,
            max_operations: None,
            max_memory_bytes: None,
            max_time: None,
            cancellation: None,
            deterministic: true,
        }
    }
}

impl DistanceOptions {
    /// Builds distance options from the canonical QEC policy.
    ///
    /// Mathematical verification uses `max_verification_operations`.
    #[must_use]
    pub fn from_limits(limits: &QecLimits) -> Self {
        Self {
            max_weight: Some(
                limits
                    .max_logical_operator_weight
                    .min(limits.max_qubits),
            ),
            max_operations: Some(limits.max_verification_operations),
            max_memory_bytes: Some(limits.max_memory_bytes),
            max_time: None,
            cancellation: None,
            deterministic: true,
        }
    }

    /// Intersects local options with the canonical QEC policy.
    ///
    /// This operation can only make the requested workload more restrictive.
    pub fn constrain_by_limits(
        &mut self,
        limits: &QecLimits,
    ) -> Result<(), DistanceError> {
        limits
            .validate()
            .map_err(DistanceError::LimitPolicy)?;

        let policy_weight = limits
            .max_logical_operator_weight
            .min(limits.max_qubits);

        self.max_weight = Some(
            self.max_weight
                .map_or(policy_weight, |value| value.min(policy_weight)),
        );

        self.max_operations = Some(
            self.max_operations
                .map_or(
                    limits.max_verification_operations,
                    |value| value.min(limits.max_verification_operations),
                ),
        );

        self.max_memory_bytes = Some(
            self.max_memory_bytes
                .map_or(
                    limits.max_memory_bytes,
                    |value| value.min(limits.max_memory_bytes),
                ),
        );

        self.validate()
    }

    fn validate(&self) -> Result<(), DistanceError> {
        if matches!(self.max_weight, Some(0)) {
            return Err(DistanceError::InvalidOption {
                field: "max_weight",
                value: 0,
            });
        }

        if matches!(self.max_operations, Some(0)) {
            return Err(DistanceError::InvalidOption {
                field: "max_operations",
                value: 0,
            });
        }

        if matches!(self.max_memory_bytes, Some(0)) {
            return Err(DistanceError::InvalidOption {
                field: "max_memory_bytes",
                value: 0,
            });
        }

        if self
            .max_time
            .is_some_and(|duration| duration.is_zero())
        {
            return Err(DistanceError::InvalidOption {
                field: "max_time",
                value: 0,
            });
        }

        Ok(())
    }
}

// ============================================================================
// Distance result
// ============================================================================

/// Verified code-distance result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodeDistance {
    distance: usize,
    logical_operator: PauliString,
    status: DistanceStatus,
    operations: u64,
    searched_through_weight: usize,
}

impl CodeDistance {
    /// Creates an exact result from a validated witness.
    pub fn new(
        distance: usize,
        logical_operator: PauliString,
    ) -> Result<Self, DistanceError> {
        if distance == 0 {
            return Err(DistanceError::InvalidDistance { distance });
        }

        if logical_operator.is_identity() {
            return Err(DistanceError::IdentityLogicalOperator);
        }

        let weight = logical_operator.weight();

        if weight != distance {
            return Err(DistanceError::DistanceWeightMismatch {
                distance,
                weight,
            });
        }

        Ok(Self {
            distance,
            logical_operator,
            status: DistanceStatus::VerifiedExact,
            operations: 0,
            searched_through_weight: distance,
        })
    }

    fn exact(
        distance: usize,
        logical_operator: PauliString,
        operations: u64,
        searched_through_weight: usize,
    ) -> Result<Self, DistanceError> {
        let mut result = Self::new(distance, logical_operator)?;

        result.operations = operations;
        result.searched_through_weight = searched_through_weight;

        Ok(result)
    }

    #[must_use]
    pub const fn distance(&self) -> usize {
        self.distance
    }

    #[must_use]
    pub fn logical_operator(&self) -> &PauliString {
        &self.logical_operator
    }

    #[must_use]
    pub const fn status(&self) -> DistanceStatus {
        self.status
    }

    #[must_use]
    pub const fn operations(&self) -> u64 {
        self.operations
    }

    #[must_use]
    pub const fn searched_through_weight(&self) -> usize {
        self.searched_through_weight
    }
}

// ============================================================================
// Errors
// ============================================================================

/// Errors produced by distance verification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DistanceError {
    /// Invalid canonical QEC resource policy.
    LimitPolicy(LimitError),

    /// Invalid local search option.
    InvalidOption {
        field: &'static str,
        value: u64,
    },

    /// Invalid stabilizer representation or stabilizer operation failure.
    Stabilizer(StabilizerError),

    /// The stabilizer group encodes zero logical qubits.
    NoLogicalQubits {
        num_qubits: usize,
        rank: usize,
    },

    /// No non-trivial logical operator was found across the complete physical
    /// support. For a valid k > 0 stabilizer code this indicates an
    /// inconsistent representation.
    NoLogicalOperatorFound {
        num_qubits: usize,
    },

    /// The configured maximum weight ended before exact verification.
    SearchIncomplete {
        searched_through_weight: usize,
        next_weight: usize,
    },

    /// Candidate-operation budget was exhausted.
    OperationLimitExceeded {
        operations: u64,
        maximum: u64,
    },

    /// Explicit wall-clock budget was exhausted.
    TimeLimitExceeded {
        elapsed: Duration,
        maximum: Duration,
    },

    /// Working-state memory preflight failed.
    MemoryLimitExceeded {
        estimated: u64,
        maximum: u64,
    },

    /// Cooperative cancellation stopped verification.
    Cancelled {
        reason: String,
    },

    /// A checked search counter overflowed.
    ArithmeticOverflow {
        operation: &'static str,
    },

    /// Invalid distance value.
    InvalidDistance {
        distance: usize,
    },

    /// Identity cannot be a logical operator.
    IdentityLogicalOperator,

    /// Witness weight does not match the claimed distance.
    DistanceWeightMismatch {
        distance: usize,
        weight: usize,
    },

    /// Candidate dimension does not match the stabilizer code.
    CandidateDimensionMismatch {
        expected: usize,
        actual: usize,
    },
}

impl From<StabilizerError> for DistanceError {
    fn from(error: StabilizerError) -> Self {
        Self::Stabilizer(error)
    }
}

impl fmt::Display for DistanceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LimitPolicy(error) => {
                write!(f, "distance verification policy error: {error}")
            }

            Self::InvalidOption { field, value } => {
                write!(f, "invalid distance option {field}={value}")
            }

            Self::Stabilizer(error) => {
                write!(f, "stabilizer validation failed: {error}")
            }

            Self::NoLogicalQubits {
                num_qubits,
                rank,
            } => {
                write!(
                    f,
                    "stabilizer rank {rank} equals qubit count {num_qubits}; \
                     no logical qubits exist"
                )
            }

            Self::NoLogicalOperatorFound { num_qubits } => {
                write!(
                    f,
                    "no non-trivial logical operator found on \
                     {num_qubits} qubits"
                )
            }

            Self::SearchIncomplete {
                searched_through_weight,
                next_weight,
            } => {
                write!(
                    f,
                    "distance search incomplete after weight \
                     {searched_through_weight}; next weight is {next_weight}"
                )
            }

            Self::OperationLimitExceeded {
                operations,
                maximum,
            } => {
                write!(
                    f,
                    "distance operation limit exceeded: \
                     {operations}/{maximum}"
                )
            }

            Self::TimeLimitExceeded { elapsed, maximum } => {
                write!(
                    f,
                    "distance time limit exceeded: \
                     {elapsed:?}/{maximum:?}"
                )
            }

            Self::MemoryLimitExceeded {
                estimated,
                maximum,
            } => {
                write!(
                    f,
                    "distance memory preflight exceeded: \
                     {estimated} > {maximum} bytes"
                )
            }

            Self::Cancelled { reason } => {
                write!(
                    f,
                    "distance verification cancelled: {reason}"
                )
            }

            Self::ArithmeticOverflow { operation } => {
                write!(
                    f,
                    "distance arithmetic overflow in {operation}"
                )
            }

            Self::InvalidDistance { distance } => {
                write!(f, "invalid distance {distance}")
            }

            Self::IdentityLogicalOperator => {
                write!(
                    f,
                    "identity cannot be a logical operator"
                )
            }

            Self::DistanceWeightMismatch {
                distance,
                weight,
            } => {
                write!(
                    f,
                    "distance {distance} does not match \
                     witness weight {weight}"
                )
            }

            Self::CandidateDimensionMismatch {
                expected,
                actual,
            } => {
                write!(
                    f,
                    "candidate dimension mismatch: \
                     expected {expected}, got {actual}"
                )
            }
        }
    }
}

impl std::error::Error for DistanceError {}

// ============================================================================
// Search outcome
// ============================================================================

/// Result of searching one exact Pauli weight.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SearchOutcome {
    Found(PauliString),
    NotFound,
}

// ============================================================================
// Search budget
// ============================================================================

/// Shared budget for one complete distance verification.
#[derive(Debug, Clone)]
pub struct SearchBudget {
    operations: u64,
    started: Instant,
    max_operations: Option<u64>,
    max_time: Option<Duration>,
}

impl SearchBudget {
    #[must_use]
    pub fn new(
        options: &DistanceOptions,
        started: Instant,
    ) -> Self {
        Self {
            operations: 0,
            started,
            max_operations: options.max_operations,
            max_time: options.max_time,
        }
    }

    #[must_use]
    pub const fn operations(&self) -> u64 {
        self.operations
    }

    fn check(&self) -> Result<(), DistanceError> {
        if let Some(maximum) = self.max_operations {
            if self.operations >= maximum {
                return Err(
                    DistanceError::OperationLimitExceeded {
                        operations: self.operations,
                        maximum,
                    },
                );
            }
        }

        if let Some(maximum) = self.max_time {
            let elapsed = self.started.elapsed();

            if elapsed >= maximum {
                return Err(
                    DistanceError::TimeLimitExceeded {
                        elapsed,
                        maximum,
                    },
                );
            }
        }

        Ok(())
    }

    fn next(&mut self) -> Result<(), DistanceError> {
        self.check()?;

        self.operations = self
            .operations
            .checked_add(1)
            .ok_or(
                DistanceError::ArithmeticOverflow {
                    operation:
                        "candidate-operation counter",
                },
            )?;

        Ok(())
    }
}

// ============================================================================
// Public distance API
// ============================================================================

/// Computes exact distance using the canonical default QEC policy.
pub fn compute_distance(
    stabilizers: &StabilizerGroup,
) -> Result<CodeDistance, DistanceError> {
    let limits = QecLimits::default();

    compute_distance_with_limits(
        stabilizers,
        &limits,
    )
}

/// Computes exact distance under explicit QEC limits.
pub fn compute_distance_with_limits(
    stabilizers: &StabilizerGroup,
    limits: &QecLimits,
) -> Result<CodeDistance, DistanceError> {
    let mut options =
        DistanceOptions::from_limits(limits);

    compute_distance_with_options(
        stabilizers,
        limits,
        &mut options,
    )
}

/// Computes exact distance with explicit execution controls.
///
/// A result is exact only when every weight below the returned distance has
/// been exhaustively searched and rejected.
pub fn compute_distance_with_options(
    stabilizers: &StabilizerGroup,
    limits: &QecLimits,
    options: &mut DistanceOptions,
) -> Result<CodeDistance, DistanceError> {
    limits
        .validate()
        .map_err(DistanceError::LimitPolicy)?;

    options.constrain_by_limits(limits)?;
    options.validate()?;

    stabilizers
        .validate_with_limits(limits)?;

    let num_qubits =
        stabilizers.num_qubits();

    let rank =
        stabilizers
            .rank_with_limits(limits)?;

    if rank >= num_qubits {
        return Err(
            DistanceError::NoLogicalQubits {
                num_qubits,
                rank,
            },
        );
    }

    let max_weight = options
        .max_weight
        .unwrap_or(num_qubits)
        .min(num_qubits);

    if max_weight == 0 {
        return Err(
            DistanceError::InvalidDistance {
                distance: 0,
            },
        );
    }

    let mut budget =
        SearchBudget::new(
            options,
            Instant::now(),
        );

    for weight in 1..=max_weight {
        check_cancellation(
            options.cancellation.as_ref(),
        )?;

        budget.check()?;

        match find_logical_operator_of_weight_with_budget(
            stabilizers,
            weight,
            limits,
            options,
            &mut budget,
        )? {
            SearchOutcome::Found(operator) => {
                return CodeDistance::exact(
                    weight,
                    operator,
                    budget.operations(),
                    weight,
                );
            }

            SearchOutcome::NotFound => {}
        }
    }

    if max_weight == num_qubits {
        return Err(
            DistanceError::NoLogicalOperatorFound {
                num_qubits,
            },
        );
    }

    Err(
        DistanceError::SearchIncomplete {
            searched_through_weight:
                max_weight,
            next_weight:
                max_weight.checked_add(1).ok_or(
                    DistanceError::ArithmeticOverflow {
                        operation:
                            "next search weight",
                    },
                )?,
        },
    )
}

// ============================================================================
// Logical-operator search
// ============================================================================

/// Finds a logical operator of exactly `weight` under the default policy.
pub fn find_logical_operator_of_weight(
    stabilizers: &StabilizerGroup,
    weight: usize,
) -> Result<Option<PauliString>, DistanceError> {
    let limits = QecLimits::default();

    let mut options =
        DistanceOptions::from_limits(&limits);

    let mut budget =
        SearchBudget::new(
            &options,
            Instant::now(),
        );

    find_logical_operator_of_weight_with_budget(
        stabilizers,
        weight,
        &limits,
        &mut options,
        &mut budget,
    )
    .map(|outcome| match outcome {
        SearchOutcome::Found(operator) =>
            Some(operator),

        SearchOutcome::NotFound =>
            None,
    })
}

/// Compatibility entry point for callers that own a search budget.
pub fn find_logical_operator_of_weight_with_options(
    stabilizers: &StabilizerGroup,
    weight: usize,
    limits: &QecLimits,
    options: &mut DistanceOptions,
    budget: &mut SearchBudget,
) -> Result<SearchOutcome, DistanceError> {
    find_logical_operator_of_weight_with_budget(
        stabilizers,
        weight,
        limits,
        options,
        budget,
    )
}

fn find_logical_operator_of_weight_with_budget(
    stabilizers: &StabilizerGroup,
    weight: usize,
    limits: &QecLimits,
    options: &mut DistanceOptions,
    budget: &mut SearchBudget,
) -> Result<SearchOutcome, DistanceError> {
    limits
        .validate()
        .map_err(DistanceError::LimitPolicy)?;

    options.constrain_by_limits(limits)?;
    options.validate()?;

    stabilizers
        .validate_with_limits(limits)?;

    let num_qubits =
        stabilizers.num_qubits();

    if weight == 0 || weight > num_qubits {
        return Ok(SearchOutcome::NotFound);
    }

    let maximum_weight = limits
        .max_logical_operator_weight
        .min(limits.max_qubits);

    if weight > maximum_weight {
        return Err(
            DistanceError::LimitPolicy(
                LimitError::Exceeded {
                    resource:
                        LimitKind::LogicalOperatorWeight,
                    requested:
                        weight as u128,
                    maximum:
                        maximum_weight as u128,
                },
            ),
        );
    }

    let estimated_working_memory =
        estimate_working_memory(
            num_qubits,
            weight,
        )?;

    if let Some(maximum) =
        options.max_memory_bytes
    {
        if estimated_working_memory > maximum {
            return Err(
                DistanceError::MemoryLimitExceeded {
                    estimated:
                        estimated_working_memory,
                    maximum,
                },
            );
        }
    }

    /*
     * Iterative enumeration is deliberate.
     *
     * A recursive search over `num_qubits` could overflow the Rust call stack
     * for large codes before the mathematical resource budget has a chance to
     * reject the workload.
     *
     * The state is therefore:
     *
     *   support combination
     *       +
     *   ternary Pauli assignment
     *
     * and requires O(n + weight) working memory.
     */

    let mut candidate =
        vec![Pauli::I; num_qubits];

    let mut support: Vec<usize> =
        (0..weight).collect();

    let mut pauli_digits =
        vec![0_u8; weight];

    loop {
        check_cancellation(
            options.cancellation.as_ref(),
        )?;

        budget.check()?;

        for value in
            candidate.iter_mut()
        {
            *value = Pauli::I;
        }

        for (slot, &qubit) in
            support.iter().enumerate()
        {
            candidate[qubit] =
                match pauli_digits[slot] {
                    0 => Pauli::X,
                    1 => Pauli::Y,
                    2 => Pauli::Z,

                    _ => {
                        return Err(
                            DistanceError::ArithmeticOverflow {
                                operation:
                                    "Pauli digit state",
                            },
                        );
                    }
                };
        }

        budget.next()?;

        let operator =
            PauliString::from_paulis(
                &candidate,
            );

        if operator.weight()
            != weight
        {
            return Err(
                DistanceError::DistanceWeightMismatch {
                    distance: weight,
                    weight: operator.weight(),
                },
            );
        }

        /*
         * These two checks deliberately use the canonical stabilizer layer.
         *
         * First:
         *     P ∈ N(S)
         *
         * Second:
         *     P ∉ S
         *
         * Therefore:
         *     P ∈ N(S) \ S
         *
         * which is precisely the mathematical definition of a non-trivial
         * logical Pauli.
         */
        if stabilizers
            .is_in_normalizer(&operator)?
            && !stabilizers
                .contains_with_limits(
                    &operator,
                    limits,
                )?
        {
            return Ok(
                SearchOutcome::Found(
                    operator,
                ),
            );
        }

        // Advance ternary Pauli assignment.
        let mut digit = 0_usize;

        while digit < pauli_digits.len() {
            if pauli_digits[digit] < 2 {
                pauli_digits[digit] += 1;
                break;
            }

            pauli_digits[digit] = 0;
            digit += 1;
        }

        if digit < pauli_digits.len() {
            continue;
        }

        /*
         * All 3^weight assignments for the current support have been
         * exhausted. Advance the support combination.
         */
        let mut position = weight;
        let mut advanced = false;

        while position > 0 {
            position -= 1;

            let maximum =
                position
                    .checked_add(
                        num_qubits - weight,
                    )
                    .ok_or(
                        DistanceError::ArithmeticOverflow {
                            operation:
                                "support-combination bound",
                        },
                    )?;

            if support[position]
                < maximum
            {
                support[position] += 1;

                for index in
                    (position + 1)..weight
                {
                    support[index] =
                        support[index - 1]
                            .checked_add(1)
                            .ok_or(
                                DistanceError::ArithmeticOverflow {
                                    operation:
                                        "support-combination index",
                                },
                            )?;
                }

                pauli_digits.fill(0);

                advanced = true;
                break;
            }
        }

        if !advanced {
            return Ok(
                SearchOutcome::NotFound,
            );
        }
    }
}

// ============================================================================
// Resource estimation
// ============================================================================

/// Estimates only the working memory owned by this search.
///
/// The stabilizer group's already-existing memory is not double-counted here;
/// its own construction/validation is governed by `stabilizer.rs` and
/// `QecLimits`.
fn estimate_working_memory(
    num_qubits: usize,
    weight: usize,
) -> Result<u64, DistanceError> {
    let pauli_bytes =
        (num_qubits as u64)
            .checked_mul(
                std::mem::size_of::<Pauli>()
                    as u64,
            )
            .ok_or(
                DistanceError::ArithmeticOverflow {
                    operation:
                        "candidate Pauli buffer size",
                },
            )?;

    let symplectic_bytes =
        (num_qubits as u64)
            .checked_mul(2)
            .ok_or(
                DistanceError::ArithmeticOverflow {
                    operation:
                        "candidate symplectic size",
                },
            )?;

    let support_bytes =
        (weight as u64)
            .checked_mul(
                std::mem::size_of::<usize>()
                    as u64,
            )
            .ok_or(
                DistanceError::ArithmeticOverflow {
                    operation:
                        "candidate support size",
                },
            )?;

    pauli_bytes
        .checked_add(symplectic_bytes)
        .and_then(|value| {
            value.checked_add(support_bytes)
        })
        .ok_or(
            DistanceError::ArithmeticOverflow {
                operation:
                    "distance working-set size",
            },
        )
}

// ============================================================================
// Cancellation
// ============================================================================

fn check_cancellation(
    token: Option<&CancellationToken>,
) -> Result<(), DistanceError> {
    if let Some(token) = token {
        if token.is_cancelled() {
            let reason = token
                .reason()
                .map(|reason| reason.to_string())
                .unwrap_or_else(
                    || "requested".to_string(),
                );

            return Err(
                DistanceError::Cancelled {
                    reason,
                },
            );
        }
    }

    Ok(())
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::quantum::error_correction::stabilizer::{
        StabilizerGenerator,
    };

    fn repetition_group()
        -> StabilizerGroup
    {
        let mut group =
            StabilizerGroup::new(3)
                .expect("group");

        group
            .add_generator(
                StabilizerGenerator::new(
                    0,
                    PauliString::from_paulis(
                        &[
                            Pauli::Z,
                            Pauli::Z,
                            Pauli::I,
                        ],
                    ),
                )
                .expect("generator"),
            )
            .expect("add");

        group
            .add_generator(
                StabilizerGenerator::new(
                    1,
                    PauliString::from_paulis(
                        &[
                            Pauli::I,
                            Pauli::Z,
                            Pauli::Z,
                        ],
                    ),
                )
                .expect("generator"),
            )
            .expect("add");

        group
    }

    #[test]
    fn finds_exact_repetition_code_distance()
    {
        let group =
            repetition_group();

        let mut limits =
            QecLimits::default();

        limits.max_verification_operations =
            1_000;

        let result =
            compute_distance_with_limits(
                &group,
                &limits,
            )
            .expect("distance");

        assert_eq!(
            result.distance(),
            1
        );

        assert!(
            result.status().is_exact()
        );

        assert_eq!(
            result
                .logical_operator()
                .weight(),
            1
        );
    }

    #[test]
    fn rejects_zero_weight_option()
    {
        let group =
            repetition_group();

        let limits =
            QecLimits::default();

        let mut options =
            DistanceOptions::from_limits(
                &limits,
            );

        options.max_weight =
            Some(0);

        assert!(matches!(
            compute_distance_with_options(
                &group,
                &limits,
                &mut options,
            ),
            Err(
                DistanceError::InvalidOption {
                    field: "max_weight",
                    ..
                }
            )
        ));
    }
}