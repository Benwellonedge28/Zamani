//! Zamani Quantum Error Correction — Exact Stabilizer-Code Distance.
//!
//! The code distance is defined as:
//!
//!     d = min { wt(P) : P ∈ N(S) \ S }
//!
//! where:
//!
//! - S is the stabilizer group;
//! - N(S) is its Pauli normalizer;
//! - wt(P) is the number of non-identity Paulis in P.
//!
//! A candidate logical operator is therefore valid only when:
//!
//! 1. it has the requested weight;
//! 2. it has the same number of qubits as the code;
//! 3. it commutes with every stabilizer;
//! 4. it is not a member of the stabilizer group.
//!
//! # Production guarantees
//!
//! This module does not trust a declared distance.
//!
//! It performs an actual minimum-weight logical-operator search and uses the
//! GF(2) membership implementation in `stabilizer.rs` to exclude stabilizers.
//!
//! Exact distance search is exponential in the worst case. That is a
//! mathematical property of the general problem, not an implementation bug.
//! Consequently, all production entry points are resource bounded.
//!
//! The search supports:
//!
//! - `QecLimits`;
//! - maximum search weight;
//! - maximum candidate operations;
//! - maximum memory policy;
//! - maximum wall time;
//! - cooperative cancellation;
//! - deterministic traversal;
//! - overflow-safe counters;
//! - explicit incomplete/resource-limited errors;
//! - exact logical witnesses;
//! - logical-qubit validation.
//!
//! A search that terminates because of a resource limit is NEVER reported as
//! an exact distance proof.

use core::fmt;
use std::time::{Duration, Instant};

use super::cancellation::CancellationToken;
use super::limits::QecLimits;
use super::stabilizer::{
    commutes_with_stabilizer_group,
    Pauli,
    PauliString,
    StabilizerError,
    StabilizerGroup,
};

// ============================================================================
// Verification status
// ============================================================================

/// Mathematical status associated with a distance verification.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
)]
pub enum DistanceStatus {
    /// The minimum-weight logical operator was exhaustively established.
    VerifiedExact,

    /// A valid lower bound has been established, but the exact minimum has
    /// not been found.
    VerifiedLowerBound,

    /// A valid logical witness establishes an upper bound, but minimality
    /// has not been exhaustively established.
    VerifiedUpperBound,

    /// No mathematical bound has been established.
    Unverified,

    /// The operation was stopped by a configured resource boundary.
    ResourceLimited,

    /// The operation was stopped cooperatively.
    Cancelled,
}

impl DistanceStatus {
    #[must_use]
    pub const fn is_exact(
        self,
    ) -> bool {
        matches!(
            self,
            Self::VerifiedExact
        )
    }

    #[must_use]
    pub const fn is_verified(
        self,
    ) -> bool {
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

/// Controls an exact or bounded distance search.
#[derive(
    Clone,
    Debug,
)]
pub struct DistanceOptions {
    /// Maximum Pauli weight to search.
    ///
    /// `None` means up to the physical-qubit count, subject to `QecLimits`.
    pub max_weight: Option<usize>,

    /// Maximum number of candidate Pauli operators evaluated.
    pub max_operations: Option<u64>,

    /// Maximum memory budget associated with this operation.
    pub max_memory_bytes: Option<u64>,

    /// Maximum wall-clock duration.
    pub max_time: Option<Duration>,

    /// Optional cooperative cancellation token.
    pub cancellation: Option<CancellationToken>,

    /// Requests deterministic traversal.
    ///
    /// The current exact implementation is deterministic regardless of this
    /// flag. It exists so future parallel implementations can preserve the
    /// same execution contract.
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
    /// Creates production options from the canonical QEC resource policy.
    #[must_use]
    pub fn from_limits(
        limits: &QecLimits,
    ) -> Self {
        Self {
            max_weight: Some(
                limits
                    .max_logical_operator_weight
                    .min(limits.max_qubits),
            ),

            // The current QecLimits API exposes decoder iterations as the
            // canonical bounded-work counter. Use it as a finite safety
            // boundary rather than permitting accidental unbounded
            // exponential enumeration.
            max_operations: Some(
                limits.max_decoder_iterations
                    as u64,
            ),

            max_memory_bytes: Some(
                limits.max_memory_bytes,
            ),

            max_time: Some(
                Duration::from_nanos(
                    limits.max_decoder_time_ns,
                ),
            ),

            cancellation: None,

            deterministic: true,
        }
    }

    /// Restricts the options to the canonical QEC policy.
    ///
    /// This operation can only make a workload more restrictive.
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
                .map_or(
                    policy_weight,
                    |value| value.min(
                        policy_weight,
                    ),
                ),
        );

        let policy_operations =
            limits.max_decoder_iterations
                as u64;

        self.max_operations = Some(
            self.max_operations
                .map_or(
                    policy_operations,
                    |value| value.min(
                        policy_operations,
                    ),
                ),
        );

        self.max_memory_bytes = Some(
            self.max_memory_bytes
                .map_or(
                    limits.max_memory_bytes,
                    |value| {
                        value.min(
                            limits.max_memory_bytes,
                        )
                    },
                ),
        );

        let policy_time =
            Duration::from_nanos(
                limits.max_decoder_time_ns,
            );

        self.max_time = Some(
            self.max_time
                .map_or(
                    policy_time,
                    |value| value.min(
                        policy_time,
                    ),
                ),
        );

        Ok(())
    }

    fn validate(
        &self,
    ) -> Result<(), DistanceError> {
        if matches!(
            self.max_weight,
            Some(0)
        ) {
            return Err(
                DistanceError::InvalidOption {
                    field: "max_weight",
                    value: 0,
                },
            );
        }

        if matches!(
            self.max_operations,
            Some(0)
        ) {
            return Err(
                DistanceError::InvalidOption {
                    field: "max_operations",
                    value: 0,
                },
            );
        }

        if matches!(
            self.max_memory_bytes,
            Some(0)
        ) {
            return Err(
                DistanceError::InvalidOption {
                    field: "max_memory_bytes",
                    value: 0,
                },
            );
        }

        if matches!(
            self.max_time,
            Some(duration)
                if duration.is_zero()
        ) {
            return Err(
                DistanceError::InvalidOption {
                    field: "max_time",
                    value: 0,
                },
            );
        }

        Ok(())
    }
}

// ============================================================================
// Distance result
// ============================================================================

/// Verified code-distance result.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub struct CodeDistance {
    distance: usize,
    logical_operator: PauliString,
    status: DistanceStatus,
    operations: u64,
    searched_through_weight: usize,
}

impl CodeDistance {
    /// Creates an exact distance result.
    pub fn new(
        distance: usize,
        logical_operator: PauliString,
    ) -> Result<Self, DistanceError> {
        if distance == 0 {
            return Err(
                DistanceError::InvalidDistance {
                    distance,
                },
            );
        }

        if logical_operator
            .is_identity()
        {
            return Err(
                DistanceError::IdentityLogicalOperator,
            );
        }

        if logical_operator.weight()
            != distance
        {
            return Err(
                DistanceError::DistanceWeightMismatch {
                    distance,
                    weight:
                        logical_operator.weight(),
                },
            );
        }

        Ok(Self {
            distance,
            logical_operator,
            status:
                DistanceStatus::VerifiedExact,
            operations: 0,
            searched_through_weight:
                distance,
        })
    }

    fn exact(
        distance: usize,
        logical_operator: PauliString,
        operations: u64,
        searched_through_weight: usize,
    ) -> Result<Self, DistanceError> {
        let mut result =
            Self::new(
                distance,
                logical_operator,
            )?;

        result.operations =
            operations;

        result.searched_through_weight =
            searched_through_weight;

        Ok(result)
    }

    #[must_use]
    pub const fn distance(
        &self,
    ) -> usize {
        self.distance
    }

    #[must_use]
    pub fn logical_operator(
        &self,
    ) -> &PauliString {
        &self.logical_operator
    }

    #[must_use]
    pub const fn status(
        &self,
    ) -> DistanceStatus {
        self.status
    }

    #[must_use]
    pub const fn operations(
        &self,
    ) -> u64 {
        self.operations
    }

    #[must_use]
    pub const fn searched_through_weight(
        &self,
    ) -> usize {
        self.searched_through_weight
    }
}

// ============================================================================
// Exact distance calculation
// ============================================================================

/// Computes the exact code distance using the default production QEC policy.
///
/// This intentionally does NOT perform an unbounded search.
pub fn compute_distance(
    stabilizers: &StabilizerGroup,
) -> Result<CodeDistance, DistanceError> {
    let limits =
        QecLimits::default();

    compute_distance_with_limits(
        stabilizers,
        &limits,
    )
}

/// Computes the exact code distance under explicit resource limits.
pub fn compute_distance_with_limits(
    stabilizers: &StabilizerGroup,
    limits: &QecLimits,
) -> Result<CodeDistance, DistanceError> {
    let mut options =
        DistanceOptions::from_limits(
            limits,
        );

    compute_distance_with_options(
        stabilizers,
        limits,
        &mut options,
    )
}

/// Computes the exact code distance with complete execution controls.
///
/// The first valid logical operator found at weight `w` proves:
///
///     d <= w
///
/// Exactness is reported only because every lower weight was exhaustively
/// searched and rejected.
pub fn compute_distance_with_options(
    stabilizers: &StabilizerGroup,
    limits: &QecLimits,
    options: &mut DistanceOptions,
) -> Result<CodeDistance, DistanceError> {
    options
        .constrain_by_limits(limits)?;

    options.validate()?;

    stabilizers
        .validate_with_limits(limits)
        .map_err(
            DistanceError::Stabilizer,
        )?;

    let num_qubits =
        stabilizers.num_qubits();

    let rank =
        stabilizers
            .rank_with_limits(limits)
            .map_err(
                DistanceError::Stabilizer,
            )?;

    // k = n - rank(S)
    //
    // A rank-n stabilizer system encodes zero logical qubits. It must not be
    // assigned an artificial finite distance.
    if rank == num_qubits {
        return Err(
            DistanceError::NoLogicalQubits {
                num_qubits,
                rank,
            },
        );
    }

    let max_weight =
        options
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

    let started =
        Instant::now();

    let mut budget =
        SearchBudget::new(
            options,
            started,
        );

    for weight in
        1..=max_weight
    {
        check_cancelled(
            options.cancellation.as_ref(),
        )?;

        budget.check()?;

        match find_logical_operator_of_weight_with_options(
            stabilizers,
            weight,
            limits,
            options,
            &mut budget,
        )? {
            SearchOutcome::Found(
                operator,
            ) => {
                return CodeDistance::exact(
                    weight,
                    operator,
                    budget.operations,
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
                max_weight
                    .checked_add(1)
                    .unwrap_or(max_weight),
        },
    )
}

// ============================================================================
// Logical-operator search
// ============================================================================

/// Finds a logical operator of exactly `weight` using the default policy.
pub fn find_logical_operator_of_weight(
    stabilizers: &StabilizerGroup,
    weight: usize,
) -> Result<
    Option<PauliString>,
    DistanceError,
> {
    let limits =
        QecLimits::default();

    let mut options =
        DistanceOptions::from_limits(
            &limits,
        );

    let mut budget =
        SearchBudget::new(
            &options,
            Instant::now(),
        );

    find_logical_operator_of_weight_with_options(
        stabilizers,
        weight,
        &limits,
        &mut options,
        &mut budget,
    )
    .map(
        |outcome| match outcome {
            SearchOutcome::Found(
                operator,
            ) => Some(operator),

            SearchOutcome::NotFound =>
                None,
        },
    )
}

/// Finds a logical operator of exactly `weight` using explicit controls.
pub fn find_logical_operator_of_weight_with_options(
    stabilizers: &StabilizerGroup,
    weight: usize,
    limits: &QecLimits,
    options: &mut DistanceOptions,
    budget: &mut SearchBudget,
) -> Result<
    SearchOutcome,
    DistanceError,
> {
    stabilizers
        .validate_with_limits(limits)
        .map_err(
            DistanceError::Stabilizer,
        )?;

    if weight == 0
        || weight
            > stabilizers.num_qubits()
    {
        return Ok(
            SearchOutcome::NotFound,
        );
    }

    let maximum_weight =
        limits
            .max_logical_operator_weight
            .min(limits.max_qubits);

    if weight > maximum_weight {
        return Err(
            DistanceError::LimitPolicy(
                super::limits::LimitError::
                    LogicalOperatorWeight {
                        requested: weight,
                        maximum:
                            maximum_weight,
                    },
            ),
        );
    }

    options
        .constrain_by_limits(limits)?;

    options.validate()?;

    let mut selected =
        Vec::with_capacity(
            weight,
        );

    let mut paulis =
        vec![
            Pauli::I;
            stabilizers.num_qubits()
        ];

    search_supports(
        stabilizers,
        weight,
        0,
        &mut selected,
        &mut paulis,
        limits,
        options,
        budget,
    )
}

/// Explicit outcome so callers cannot confuse "not found" with "search
/// incomplete".
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub enum SearchOutcome {
    Found(PauliString),
    NotFound,
}

// ============================================================================
// Search budget
// ============================================================================

/// Shared budget for a single exact search.
///
/// Every candidate assignment consumes one operation. The counter is checked
/// before accepting a candidate as part of the proof.
#[derive(
    Debug,
    Clone,
)]
pub struct SearchBudget {
    operations: u64,
    started: Instant,
    max_operations: Option<u64>,
    max_time: Option<Duration>,
}

impl SearchBudget {
    fn new(
        options: &DistanceOptions,
        started: Instant,
    ) -> Self {
        Self {
            operations: 0,
            started,
            max_operations:
                options.max_operations,
            max_time:
                options.max_time,
        }
    }

    #[inline]
    fn step(
        &mut self,
        options: &DistanceOptions,
    ) -> Result<(), DistanceError> {
        self.operations =
            self.operations
                .checked_add(1)
                .ok_or(
                    DistanceError::
                        OperationCounterOverflow,
                )?;

        if let Some(maximum) =
            self.max_operations
        {
            if self.operations
                > maximum
            {
                return Err(
                    DistanceError::
                        ResourceLimitExceeded {
                            resource:
                                "distance operations",
                            requested:
                                self.operations,
                            maximum,
                        },
                );
            }
        }

        if let Some(maximum) =
            self.max_time
        {
            let elapsed =
                self.started.elapsed();

            if elapsed >= maximum {
                return Err(
                    DistanceError::
                        TimeLimitExceeded {
                            elapsed,
                            maximum,
                        },
                );
            }
        }

        check_cancelled(
            options.cancellation
                .as_ref(),
        )
    }

    fn check(
        &self,
    ) -> Result<(), DistanceError> {
        if let Some(maximum) =
            self.max_operations
        {
            if self.operations
                >= maximum
            {
                return Err(
                    DistanceError::
                        ResourceLimitExceeded {
                            resource:
                                "distance operations",
                            requested:
                                self.operations,
                            maximum,
                        },
                );
            }
        }

        if let Some(maximum) =
            self.max_time
        {
            let elapsed =
                self.started.elapsed();

            if elapsed >= maximum {
                return Err(
                    DistanceError::
                        TimeLimitExceeded {
                            elapsed,
                            maximum,
                        },
                );
            }
        }

        Ok(())
    }
}

// ============================================================================
// Deterministic support search
// ============================================================================

fn search_supports(
    stabilizers: &StabilizerGroup,
    remaining_weight: usize,
    start_qubit: usize,
    selected: &mut Vec<usize>,
    paulis: &mut [Pauli],
    limits: &QecLimits,
    options: &DistanceOptions,
    budget: &mut SearchBudget,
) -> Result<
    SearchOutcome,
    DistanceError,
> {
    check_cancelled(
        options.cancellation
            .as_ref(),
    )?;

    budget.check()?;

    let num_qubits =
        stabilizers.num_qubits();

    if remaining_weight == 0 {
        return search_pauli_assignments(
            stabilizers,
            selected,
            0,
            paulis,
            limits,
            options,
            budget,
        );
    }

    if num_qubits
        .saturating_sub(
            start_qubit,
        )
        < remaining_weight
    {
        return Ok(
            SearchOutcome::NotFound,
        );
    }

    // Increasing qubit order gives deterministic support enumeration.
    for qubit in
        start_qubit..num_qubits
    {
        selected.push(qubit);

        let outcome =
            search_supports(
                stabilizers,
                remaining_weight - 1,
                qubit
                    .saturating_add(1),
                selected,
                paulis,
                limits,
                options,
                budget,
            )?;

        selected.pop();

        if let SearchOutcome::Found(_) =
            outcome
        {
            return Ok(outcome);
        }
    }

    Ok(
        SearchOutcome::NotFound,
    )
}

// ============================================================================
// Deterministic Pauli assignment search
// ============================================================================

fn search_pauli_assignments(
    stabilizers: &StabilizerGroup,
    support: &[usize],
    position: usize,
    paulis: &mut [Pauli],
    limits: &QecLimits,
    options: &DistanceOptions,
    budget: &mut SearchBudget,
) -> Result<
    SearchOutcome,
    DistanceError,
> {
    check_cancelled(
        options.cancellation
            .as_ref(),
    )?;

    if position
        == support.len()
    {
        budget.step(options)?;

        let operator =
            PauliString::from_paulis(
                paulis,
            );

        if operator.weight()
            != support.len()
        {
            return Err(
                DistanceError::
                    InternalInvariantViolation {
                        detail:
                            "generated Pauli has incorrect support weight",
                    },
            );
        }

        // Normalizer membership:
        //
        //     [P, S_i] = 0
        //
        // for every stabilizer generator.
        if !commutes_with_stabilizer_group(
            &operator,
            stabilizers,
        )
        .map_err(
            DistanceError::Stabilizer,
        )? {
            return Ok(
                SearchOutcome::NotFound,
            );
        }

        // Exclude trivial logical operators that are already stabilizers.
        //
        // This uses GF(2) reduction rather than enumerating all products of
        // stabilizer generators.
        if stabilizers
            .contains_with_limits(
                &operator,
                limits,
            )
            .map_err(
                DistanceError::Stabilizer,
            )?
        {
            return Ok(
                SearchOutcome::NotFound,
            );
        }

        return Ok(
            SearchOutcome::Found(
                operator,
            ),
        );
    }

    let qubit =
        support[position];

    // Deterministic X/Y/Z ordering.
    for pauli in [
        Pauli::X,
        Pauli::Y,
        Pauli::Z,
    ] {
        paulis[qubit] =
            pauli;

        let outcome =
            search_pauli_assignments(
                stabilizers,
                support,
                position + 1,
                paulis,
                limits,
                options,
                budget,
            )?;

        if let SearchOutcome::Found(_) =
            outcome
        {
            paulis[qubit] =
                Pauli::I;

            return Ok(outcome);
        }
    }

    paulis[qubit] =
        Pauli::I;

    Ok(
        SearchOutcome::NotFound,
    )
}

// ============================================================================
// Cancellation
// ============================================================================

fn check_cancelled(
    token: Option<&CancellationToken>,
) -> Result<(), DistanceError> {
    if let Some(token) =
        token
    {
        token
            .check()
            .map_err(|_| {
                DistanceError::
                    CancellationRequested
            })?;
    }

    Ok(())
}

// ============================================================================
// Distance validation
// ============================================================================

/// Validates a claimed distance and witness.
///
/// The witness first establishes that:
///
///     distance <= claimed_distance
///
/// Exact validation then searches every strictly smaller weight.
///
/// Therefore successful return means the claimed distance is mathematically
/// verified, not merely trusted.
pub fn validate_distance(
    stabilizers: &StabilizerGroup,
    claimed_distance: usize,
    witness: &PauliString,
) -> Result<(), DistanceError> {
    let limits =
        QecLimits::default();

    validate_distance_with_limits(
        stabilizers,
        claimed_distance,
        witness,
        &limits,
    )
}

/// Validates a claimed distance using explicit limits.
pub fn validate_distance_with_limits(
    stabilizers: &StabilizerGroup,
    claimed_distance: usize,
    witness: &PauliString,
    limits: &QecLimits,
) -> Result<(), DistanceError> {
    let mut options =
        DistanceOptions::from_limits(
            limits,
        );

    validate_distance_with_options(
        stabilizers,
        claimed_distance,
        witness,
        limits,
        &mut options,
    )
}

/// Validates a claimed distance using explicit execution controls.
pub fn validate_distance_with_options(
    stabilizers: &StabilizerGroup,
    claimed_distance: usize,
    witness: &PauliString,
    limits: &QecLimits,
    options: &mut DistanceOptions,
) -> Result<(), DistanceError> {
    options
        .constrain_by_limits(limits)?;

    options.validate()?;

    stabilizers
        .validate_with_limits(limits)
        .map_err(
            DistanceError::Stabilizer,
        )?;

    if claimed_distance == 0 {
        return Err(
            DistanceError::InvalidDistance {
                distance: 0,
            },
        );
    }

    if claimed_distance
        > stabilizers.num_qubits()
    {
        return Err(
            DistanceError::InvalidDistance {
                distance:
                    claimed_distance,
            },
        );
    }

    if witness.num_qubits()
        != stabilizers.num_qubits()
    {
        return Err(
            DistanceError::Stabilizer(
                StabilizerError::
                    QubitCountMismatch {
                        first:
                            stabilizers
                                .num_qubits(),
                        second:
                            witness
                                .num_qubits(),
                    },
            ),
        );
    }

    if witness.is_identity() {
        return Err(
            DistanceError::
                IdentityLogicalOperator,
        );
    }

    if witness.weight()
        != claimed_distance
    {
        return Err(
            DistanceError::
                DistanceWeightMismatch {
                    distance:
                        claimed_distance,
                    weight:
                        witness.weight(),
                },
        );
    }

    if witness.weight()
        > limits.max_logical_operator_weight
    {
        return Err(
            DistanceError::LimitPolicy(
                super::limits::LimitError::
                    LogicalOperatorWeight {
                        requested:
                            witness.weight(),
                        maximum:
                            limits
                                .max_logical_operator_weight,
                    },
            ),
        );
    }

    if !commutes_with_stabilizer_group(
        witness,
        stabilizers,
    )
    .map_err(
        DistanceError::Stabilizer,
    )? {
        return Err(
            DistanceError::
                WitnessDoesNotCommute,
        );
    }

    if stabilizers
        .contains_with_limits(
            witness,
            limits,
        )
        .map_err(
            DistanceError::Stabilizer,
        )?
    {
        return Err(
            DistanceError::
                WitnessIsStabilizer,
        );
    }

    let mut budget =
        SearchBudget::new(
            options,
            Instant::now(),
        );

    // A witness is not enough to prove minimality.
    // Every lower weight must be exhaustively eliminated.
    for weight in
        1..claimed_distance
    {
        check_cancelled(
            options.cancellation
                .as_ref(),
        )?;

        budget.check()?;

        match find_logical_operator_of_weight_with_options(
            stabilizers,
            weight,
            limits,
            options,
            &mut budget,
        )? {
            SearchOutcome::Found(_) => {
                return Err(
                    DistanceError::
                        LowerWeightLogicalOperator {
                            weight,
                        },
                );
            }

            SearchOutcome::NotFound => {}
        }
    }

    Ok(())
}

// ============================================================================
// Convenience API
// ============================================================================

/// Returns the exact distance as a plain integer.
pub fn distance(
    stabilizers: &StabilizerGroup,
) -> Result<usize, DistanceError> {
    Ok(
        compute_distance(
            stabilizers,
        )?
        .distance(),
    )
}

// ============================================================================
// Errors
// ============================================================================

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub enum DistanceError {
    Stabilizer(
        StabilizerError,
    ),

    LimitPolicy(
        super::limits::LimitError,
    ),

    InvalidDistance {
        distance: usize,
    },

    InvalidOption {
        field: &'static str,
        value: u64,
    },

    IdentityLogicalOperator,

    DistanceWeightMismatch {
        distance: usize,
        weight: usize,
    },

    WitnessDoesNotCommute,

    WitnessIsStabilizer,

    LowerWeightLogicalOperator {
        weight: usize,
    },

    NoLogicalQubits {
        num_qubits: usize,
        rank: usize,
    },

    NoLogicalOperatorFound {
        num_qubits: usize,
    },

    SearchIncomplete {
        searched_through_weight: usize,
        next_weight: usize,
    },

    ResourceLimitExceeded {
        resource: &'static str,
        requested: u64,
        maximum: u64,
    },

    TimeLimitExceeded {
        elapsed: Duration,
        maximum: Duration,
    },

    CancellationRequested,

    OperationCounterOverflow,

    InternalInvariantViolation {
        detail: &'static str,
    },
}

impl fmt::Display
    for DistanceError
{
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Stabilizer(error) => {
                write!(
                    f,
                    "stabilizer error: {error}"
                )
            }

            Self::LimitPolicy(error) => {
                write!(
                    f,
                    "QEC limit policy rejected distance operation: {error}"
                )
            }

            Self::InvalidDistance {
                distance,
            } => {
                write!(
                    f,
                    "invalid code distance: {distance}"
                )
            }

            Self::InvalidOption {
                field,
                value,
            } => {
                write!(
                    f,
                    "invalid distance option {field}={value}"
                )
            }

            Self::IdentityLogicalOperator => {
                write!(
                    f,
                    "identity cannot be a logical-operator witness"
                )
            }

            Self::DistanceWeightMismatch {
                distance,
                weight,
            } => {
                write!(
                    f,
                    "claimed distance {distance} does not match witness weight {weight}"
                )
            }

            Self::WitnessDoesNotCommute => {
                write!(
                    f,
                    "logical witness does not commute with the stabilizer group"
                )
            }

            Self::WitnessIsStabilizer => {
                write!(
                    f,
                    "logical witness is itself a stabilizer"
                )
            }

            Self::LowerWeightLogicalOperator {
                weight,
            } => {
                write!(
                    f,
                    "found a logical operator of lower weight {weight}"
                )
            }

            Self::NoLogicalQubits {
                num_qubits,
                rank,
            } => {
                write!(
                    f,
                    "stabilizer system encodes no logical qubits: n={num_qubits}, rank={rank}"
                )
            }

            Self::NoLogicalOperatorFound {
                num_qubits,
            } => {
                write!(
                    f,
                    "no non-trivial logical operator found for {num_qubits}-qubit stabilizer system"
                )
            }

            Self::SearchIncomplete {
                searched_through_weight,
                next_weight,
            } => {
                write!(
                    f,
                    "exact distance search stopped after weight {searched_through_weight}; next weight is {next_weight}"
                )
            }

            Self::ResourceLimitExceeded {
                resource,
                requested,
                maximum,
            } => {
                write!(
                    f,
                    "distance {resource} {requested} exceeds configured maximum {maximum}"
                )
            }

            Self::TimeLimitExceeded {
                elapsed,
                maximum,
            } => {
                write!(
                    f,
                    "distance search exceeded time limit: elapsed={elapsed:?}, maximum={maximum:?}"
                )
            }

            Self::CancellationRequested => {
                write!(
                    f,
                    "distance search was cancelled"
                )
            }

            Self::OperationCounterOverflow => {
                write!(
                    f,
                    "distance operation counter overflowed"
                )
            }

            Self::InternalInvariantViolation {
                detail,
            } => {
                write!(
                    f,
                    "distance invariant violated: {detail}"
                )
            }
        }
    }
}

impl std::error::Error
    for DistanceError
{
}

impl From<StabilizerError>
    for DistanceError
{
    fn from(
        error: StabilizerError,
    ) -> Self {
        Self::Stabilizer(error)
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::quantum::error_correction::stabilizer::StabilizerGenerator;

    fn repetition_code()
        -> StabilizerGroup
    {
        let mut group =
            StabilizerGroup::new(3)
                .unwrap();

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
                .unwrap(),
            )
            .unwrap();

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
                .unwrap(),
            )
            .unwrap();

        group
    }

    fn x_logical()
        -> PauliString
    {
        PauliString::from_paulis(
            &[
                Pauli::X,
                Pauli::X,
                Pauli::X,
            ],
        )
    }

    #[test]
    fn verifies_repetition_code_witness() {
        let group =
            repetition_code();

        let witness =
            x_logical();

        assert!(
            validate_distance(
                &group,
                3,
                &witness,
            )
            .is_ok()
        );
    }

    #[test]
    fn finds_exact_repetition_code_distance() {
        let group =
            repetition_code();

        let result =
            compute_distance(
                &group,
            )
            .unwrap();

        assert_eq!(
            result.distance(),
            3
        );

        assert_eq!(
            result.status(),
            DistanceStatus::VerifiedExact
        );

        assert_eq!(
            result
                .logical_operator()
                .weight(),
            3
        );
    }

    #[test]
    fn rejects_stabilizer_witness() {
        let group =
            repetition_code();

        let witness =
            PauliString::from_paulis(
                &[
                    Pauli::Z,
                    Pauli::Z,
                    Pauli::I,
                ],
            );

        assert!(matches!(
            validate_distance(
                &group,
                2,
                &witness,
            ),
            Err(
                DistanceError::
                    WitnessIsStabilizer
            )
        ));
    }

    #[test]
    fn rejects_non_commuting_witness() {
        let group =
            repetition_code();

        let witness =
            PauliString::from_paulis(
                &[
                    Pauli::X,
                    Pauli::I,
                    Pauli::I,
                ],
            );

        assert!(matches!(
            validate_distance(
                &group,
                1,
                &witness,
            ),
            Err(
                DistanceError::
                    WitnessDoesNotCommute
            )
        ));
    }

    #[test]
    fn empty_stabilizer_group_has_distance_one() {
        let group =
            StabilizerGroup::new(2)
                .unwrap();

        let result =
            compute_distance(
                &group,
            )
            .unwrap();

        assert_eq!(
            result.distance(),
            1
        );

        assert_eq!(
            result
                .logical_operator()
                .weight(),
            1
        );
    }

    #[test]
    fn zero_logical_qubit_system_is_rejected() {
        let mut group =
            StabilizerGroup::new(1)
                .unwrap();

        group
            .add_generator(
                StabilizerGenerator::new(
                    0,
                    PauliString::from_paulis(
                        &[Pauli::Z],
                    ),
                )
                .unwrap(),
            )
            .unwrap();

        assert!(matches!(
            compute_distance(
                &group,
            ),
            Err(
                DistanceError::
                    NoLogicalQubits { .. }
            )
        ));
    }

    #[test]
    fn resource_budget_is_enforced() {
        let group =
            repetition_code();

        let limits =
            QecLimits {
                max_decoder_iterations: 1,
                ..QecLimits::default()
            };

        assert!(matches!(
            compute_distance_with_limits(
                &group,
                &limits,
            ),
            Err(
                DistanceError::
                    ResourceLimitExceeded { .. }
            )
        ));
    }

    #[test]
    fn cancellation_is_reported() {
        let group =
            repetition_code();

        let source =
            super::super::cancellation::
                CancellationSource::new();

        source.cancel();

        let limits =
            QecLimits::default();

        let mut options =
            DistanceOptions::from_limits(
                &limits,
            );

        options.cancellation =
            Some(source.token());

        assert!(matches!(
            compute_distance_with_options(
                &group,
                &limits,
                &mut options,
            ),
            Err(
                DistanceError::
                    CancellationRequested
            )
        ));
    }
}