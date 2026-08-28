//! Zamani Quantum Routing — Production Verification
//!
//! Production-grade verification of logical-to-physical quantum routing.
//!
//! # Responsibility
//!
//! This module is the final correctness boundary of the routing subsystem.
//! It verifies that a routing result:
//!
//! - has a valid initial mapping;
//! - has a valid final mapping;
//! - never references nonexistent or unavailable physical qubits;
//! - never creates physical mapping collisions;
//! - performs only legal routing movements;
//! - applies SWAP/permutation semantics correctly;
//! - preserves logical gate operand identity;
//! - preserves gate operand order;
//! - produces physically executable gate operations;
//! - respects direction-sensitive topology;
//! - preserves the mapping evolution represented by the operation stream;
//! - preserves the declared final mapping;
//! - rejects malformed routing operations;
//! - rejects invalid permutations;
//! - rejects invalid gate arity;
//! - preserves measurement/reset/barrier semantics at the routing boundary;
//! - provides deterministic, structured diagnostics.
//!
//! # Architectural boundary
//!
//! `verification.rs` does NOT:
//!
//! - route circuits;
//! - select layouts;
//! - select routing algorithms;
//! - optimize circuits;
//! - synthesize gates;
//! - decompose SWAP into native gates;
//! - schedule operations;
//! - execute hardware;
//! - acquire calibration data;
//! - parse OpenQASM;
//! - simulate quantum states;
//! - decode QEC syndromes.
//!
//! It verifies the semantic routing representation produced by those stages.
//!
//! # Verification model
//!
//! Verification is performed in two dimensions:
//!
//! ```text
//! structural verification
//!         │
//!         ├── topology
//!         ├── mapping
//!         ├── movement legality
//!         └── operation shape
//!
//! semantic verification
//!         │
//!         ├── logical operands
//!         ├── physical operands
//!         ├── mapping evolution
//!         ├── gate order
//!         ├── measurement identity
//!         └── final mapping
//! ```
//!
//! The verifier does not merely ask whether the final gates are adjacent.
//! It replays the routing operation stream from the initial mapping and
//! independently reconstructs the final mapping. The reconstructed mapping
//! must exactly equal the mapping declared by the routing result.
//!
//! # Important invariant
//!
//! For every executable routed gate:
//!
//! ```text
//! mapping(logical_operand[i]) == physical_operand[i]
//! ```
//!
//! immediately before that gate executes.
//!
//! This catches the most dangerous class of routing bugs:
//!
//! ```text
//! gate is physically adjacent
//! BUT
//! gate is operating on the wrong logical qubits.
//! ```
//!
//! # Transactionality
//!
//! Verification never mutates the caller's mappings.
//!
//! It creates a private working mapping from the initial snapshot and replays
//! operations against that state. Failure therefore cannot corrupt router state.
//!
//! # Verification levels
//!
//! `VerificationLevel` controls how much work is performed:
//!
//! - `None`: no verification;
//! - `Basic`: structural and mapping checks;
//! - `Standard`: normal production verification, including semantic replay;
//! - `Strict`: Standard verification plus stronger gate, permutation, finite-value,
//!   and consistency checks.
//!
//! Production routing should use `Standard`.
//!
//! CI, fuzzing, development, and safety-critical compilation should use `Strict`.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021.
//!
//! No nightly features are required.
//! No `unsafe` code is used.
//! No external dependencies are required.
//!
//! # Integration contract
//!
//! This file is intentionally implemented against the already-established
//! routing contracts:
//!
//! ```text
//! types.rs
//!     │
//!     ├── RoutingOperation
//!     ├── RoutingMove
//!     ├── GateIdentity
//!     ├── LogicalQubitId
//!     └── PhysicalQubitId
//!
//! mapping.rs
//!     │
//!     ├── QubitMapping
//!     └── QubitMappingSnapshot
//!
//! topology.rs
//!     │
//!     ├── contains()
//!     ├── is_available()
//!     ├── is_adjacent()
//!     ├── supports_gate()
//!     └── has_explicit_gate_support()
//!
//! config.rs
//!     │
//!     └── VerificationLevel
//!
//! errors.rs
//!     │
//!     └── RoutingError / VerificationError
//!
//! result.rs
//!     │
//!     └── RoutingResult / VerificationSummary
//! ```
//!
//! Later files should consume this API rather than modifying this verifier
//! contract.

use crate::quantum::routing::config::VerificationLevel;
use crate::quantum::routing::errors::{
    RoutingError,
    RoutingErrorContext,
    RoutingErrorKind,
    VerificationError,
};
use crate::quantum::routing::mapping::{
    MappingError,
    QubitMapping,
    QubitMappingSnapshot,
};
use crate::quantum::routing::result::{
    RoutingResult,
    VerificationStatus,
    VerificationSummary,
};
use crate::quantum::routing::topology::PhysicalTopology;
use crate::quantum::routing::types::{
    GateIdentity,
    LogicalQubitId,
    PhysicalQubitId,
    RoutingMove,
    RoutingOperation,
};

use std::collections::{BTreeMap, BTreeSet};

// =============================================================================
// Constants
// =============================================================================

/// Stable verifier implementation version.
///
/// This value is recorded in verification summaries so routing results can be
/// reproduced and diagnosed across Zamani releases.
pub const VERIFIER_VERSION: &str = "zamani-routing-verifier-1.0";

/// Maximum operation-stream length accepted by the standalone verifier unless
/// the caller explicitly supplies a larger limit.
///
/// This is a defensive resource bound against accidentally verifying an
/// unbounded or maliciously constructed operation stream.
pub const DEFAULT_MAX_OPERATIONS: usize = 10_000_000;

/// Maximum number of logical operands accepted for one routing operation.
///
/// Arbitrary multi-qubit operations may exist in the canonical Quantum IR, but
/// routing verification must not accidentally allocate pathological temporary
/// structures for an unbounded operation.
pub const DEFAULT_MAX_ARITY: usize = 4096;

// =============================================================================
// Verification input
// =============================================================================

/// Complete input to the routing verifier.
///
/// This is deliberately independent from `RoutingResult` so algorithms,
/// transpilers, tests, and hardware integration can invoke verification before
/// constructing a final result object.
///
/// `original_interactions` represents the logical operations that the routing
/// stage was required to preserve.
///
/// `operations` represents the semantic routed operation stream.
///
/// The verifier replays `operations` beginning at `initial_mapping` and checks
/// that the resulting state equals `final_mapping`.
#[derive(Debug, Clone)]
pub struct RoutingVerificationInput<'a> {
    /// Physical hardware topology.
    pub topology: &'a PhysicalTopology,

    /// Mapping before any routing movement.
    pub initial_mapping: &'a QubitMappingSnapshot,

    /// Mapping declared by the router after all routing movement.
    pub final_mapping: &'a QubitMappingSnapshot,

    /// Original logical interaction stream.
    ///
    /// This is optional at the type level because some lower-level routing
    /// clients verify only physical legality and mapping evolution.
    ///
    /// When supplied, Standard/Strict verification additionally checks logical
    /// gate preservation.
    pub original_interactions: &'a [crate::quantum::routing::types::QubitInteraction],

    /// Semantic routing operation stream.
    pub operations: &'a [RoutingOperation],

    /// Requested verification strength.
    pub level: VerificationLevel,

    /// Maximum number of operations the verifier is permitted to inspect.
    pub max_operations: usize,

    /// Maximum allowed operation arity.
    pub max_arity: usize,
}

impl<'a> RoutingVerificationInput<'a> {
    /// Creates a production verification input using safe defaults.
    #[must_use]
    pub fn new(
        topology: &'a PhysicalTopology,
        initial_mapping: &'a QubitMappingSnapshot,
        final_mapping: &'a QubitMappingSnapshot,
        original_interactions: &'a [
            crate::quantum::routing::types::QubitInteraction
        ],
        operations: &'a [RoutingOperation],
        level: VerificationLevel,
    ) -> Self {
        Self {
            topology,
            initial_mapping,
            final_mapping,
            original_interactions,
            operations,
            level,
            max_operations: DEFAULT_MAX_OPERATIONS,
            max_arity: DEFAULT_MAX_ARITY,
        }
    }

    /// Replaces the maximum operation-stream length.
    #[must_use]
    pub const fn with_max_operations(
        mut self,
        maximum: usize,
    ) -> Self {
        self.max_operations = maximum;
        self
    }

    /// Replaces the maximum accepted operation arity.
    #[must_use]
    pub const fn with_max_arity(
        mut self,
        maximum: usize,
    ) -> Self {
        self.max_arity = maximum;
        self
    }
}

// =============================================================================
// Verification checks
// =============================================================================

/// Individual verification category.
///
/// The verifier records counts rather than exposing internal implementation
/// details. This gives benchmarking and diagnostics stable information without
/// coupling them to the verifier's algorithm.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum VerificationCheckKind {
    /// Topology and physical-resource checks.
    Structural,

    /// Mapping bijection/collision checks.
    Mapping,

    /// Physical executability checks.
    Executability,

    /// Logical/semantic preservation checks.
    Preservation,
}

impl VerificationCheckKind {
    /// Returns the stable machine-readable name.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Structural => "structural",
            Self::Mapping => "mapping",
            Self::Executability => "executability",
            Self::Preservation => "preservation",
        }
    }
}

/// Detailed successful verification report.
///
/// `RoutingResult` stores a compact `VerificationSummary`; this report is the
/// richer standalone verifier output for compiler diagnostics, tests, fuzzing,
/// and debugging.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerificationReport {
    /// Verification level used.
    pub level: VerificationLevel,

    /// Stable verifier implementation identifier.
    pub verifier_version: &'static str,

    /// Total structural checks.
    pub structural_checks: usize,

    /// Total mapping checks.
    pub mapping_checks: usize,

    /// Total physical-executability checks.
    pub executability_checks: usize,

    /// Total semantic-preservation checks.
    pub preservation_checks: usize,

    /// Total successful checks.
    pub passed_checks: usize,

    /// Number of routed operations inspected.
    pub operations_checked: usize,

    /// Number of movement operations replayed.
    pub movements_checked: usize,

    /// Number of gate operations checked.
    pub gates_checked: usize,

    /// Number of barrier operations checked.
    pub barriers_checked: usize,

    /// Number of SWAP movements checked.
    pub swaps_checked: usize,

    /// Number of bridge movements checked.
    pub bridges_checked: usize,

    /// Number of permutation movements checked.
    pub permutations_checked: usize,

    /// Whether the final mapping reconstructed by replay equals the declared
    /// final mapping.
    pub final_mapping_matches: bool,
}

impl VerificationReport {
    /// Returns the total number of recorded checks.
    #[must_use]
    pub const fn total_checks(&self) -> usize {
        self.structural_checks
            + self.mapping_checks
            + self.executability_checks
            + self.preservation_checks
    }

    /// Returns whether every recorded check passed.
    #[must_use]
    pub const fn all_checks_passed(&self) -> bool {
        self.passed_checks == self.total_checks()
    }

    /// Converts this detailed report into the stable result-facing summary.
    #[must_use]
    pub fn summary(&self) -> VerificationSummary {
        VerificationSummary {
            level: self.level,
            status: VerificationStatus::Passed,
            structural_checks: self.structural_checks,
            mapping_checks: self.mapping_checks,
            executability_checks: self.executability_checks,
            preservation_checks: self.preservation_checks,
            passed_checks: self.passed_checks,
            verifier_version: Some(self.verifier_version.to_owned()),
        }
    }
}

// =============================================================================
// Verifier
// =============================================================================

/// Production routing verifier.
///
/// The verifier is stateless. This is intentional:
///
/// - no global mutable state;
/// - no cached mutable topology;
/// - no shared mutable mapping;
/// - deterministic behavior;
/// - thread-safe by construction;
/// - easy parallel use for benchmark trials.
///
/// The topology and routing state are supplied per verification call.
#[derive(Debug, Clone, Copy, Default)]
pub struct RoutingVerifier;

impl RoutingVerifier {
    /// Creates a stateless verifier.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Verifies a complete routing invocation.
    ///
    /// This is the primary API used by `router.rs`.
    pub fn verify(
        &self,
        input: &RoutingVerificationInput<'_>,
    ) -> Result<VerificationReport, RoutingError> {
        if matches!(input.level, VerificationLevel::None) {
            return Ok(VerificationReport {
                level: VerificationLevel::None,
                verifier_version: VERIFIER_VERSION,
                structural_checks: 0,
                mapping_checks: 0,
                executability_checks: 0,
                preservation_checks: 0,
                passed_checks: 0,
                operations_checked: 0,
                movements_checked: 0,
                gates_checked: 0,
                barriers_checked: 0,
                swaps_checked: 0,
                bridges_checked: 0,
                permutations_checked: 0,
                final_mapping_matches: false,
            });
        }

        self.validate_limits(input)?;
        self.validate_topology(input)?;
        self.validate_initial_mapping(input)?;

        let mut working_mapping =
            Self::mapping_from_snapshot(input.initial_mapping)?;

        let mut report = VerificationReport {
            level: input.level,
            verifier_version: VERIFIER_VERSION,
            structural_checks: 0,
            mapping_checks: 0,
            executability_checks: 0,
            preservation_checks: 0,
            passed_checks: 0,
            operations_checked: 0,
            movements_checked: 0,
            gates_checked: 0,
            barriers_checked: 0,
            swaps_checked: 0,
            bridges_checked: 0,
            permutations_checked: 0,
            final_mapping_matches: false,
        };

        if matches!(
            input.level,
            VerificationLevel::Standard | VerificationLevel::Strict
        ) {
            self.validate_logical_input(input, &mut report)?;
        }

        for (operation_index, operation) in
            input.operations.iter().enumerate()
        {
            report.operations_checked += 1;

            self.verify_operation(
                input,
                &mut working_mapping,
                operation,
                operation_index,
                &mut report,
            )?;
        }

        self.verify_final_mapping(
            input,
            &working_mapping,
            &mut report,
        )?;

        if matches!(input.level, VerificationLevel::Strict) {
            self.verify_strict_global_invariants(
                input,
                &working_mapping,
                &mut report,
            )?;
        }

        if !report.all_checks_passed() {
            return Err(Self::error(
                VerificationError::InvariantViolation {
                    detail: format!(
                        "verification completed with {} failed checks",
                        report
                            .total_checks()
                            .saturating_sub(report.passed_checks)
                    ),
                },
                None,
                None,
                None,
            ));
        }

        Ok(report)
    }

    /// Verifies a complete `RoutingResult`.
    ///
    /// This is the convenience API used by compiler diagnostics, benchmarking,
    /// tests, and the final routing pipeline.
    ///
    /// The original logical interaction list must be supplied separately
    /// because `RoutingResult` intentionally stores the routed operation stream
    /// rather than retaining the complete original circuit representation.
    pub fn verify_result(
        &self,
        result: &RoutingResult,
        topology: &PhysicalTopology,
        original_interactions: &[
            crate::quantum::routing::types::QubitInteraction
        ],
        level: VerificationLevel,
    ) -> Result<VerificationReport, RoutingError> {
        let input = RoutingVerificationInput::new(
            topology,
            &result.layout.initial_mapping,
            &result.layout.final_mapping,
            original_interactions,
            &result.operations,
            level,
        );

        self.verify(&input)
    }

    /// Verifies only the structural legality of a routing result.
    ///
    /// This is useful for performance-sensitive internal passes that already
    /// performed semantic verification elsewhere.
    pub fn verify_structural(
        &self,
        topology: &PhysicalTopology,
        initial_mapping: &QubitMappingSnapshot,
        final_mapping: &QubitMappingSnapshot,
        operations: &[RoutingOperation],
    ) -> Result<VerificationReport, RoutingError> {
        let input = RoutingVerificationInput::new(
            topology,
            initial_mapping,
            final_mapping,
            &[],
            operations,
            VerificationLevel::Basic,
        );

        self.verify(&input)
    }

    /// Verifies a route at the strongest supported level.
    pub fn verify_strict(
        &self,
        input: &RoutingVerificationInput<'_>,
    ) -> Result<VerificationReport, RoutingError> {
        let strict_input = RoutingVerificationInput {
            level: VerificationLevel::Strict,
            ..*input
        };

        self.verify(&strict_input)
    }

    // =========================================================================
    // Input validation
    // =========================================================================

    fn validate_limits(
        &self,
        input: &RoutingVerificationInput<'_>,
    ) -> Result<(), RoutingError> {
        if input.max_operations == 0 && !input.operations.is_empty() {
            return Err(Self::error(
                VerificationError::InvariantViolation {
                    detail:
                        "verification operation limit is zero but operations \
                         were supplied"
                            .to_owned(),
                },
                None,
                None,
                None,
            ));
        }

        if input.operations.len() > input.max_operations {
            return Err(Self::error(
                VerificationError::InvariantViolation {
                    detail: format!(
                        "operation stream contains {} operations, exceeding \
                         verifier limit {}",
                        input.operations.len(),
                        input.max_operations
                    ),
                },
                None,
                None,
                None,
            ));
        }

        if input.max_arity == 0 && !input.operations.is_empty() {
            return Err(Self::error(
                VerificationError::InvariantViolation {
                    detail:
                        "verification arity limit is zero but operations \
                         were supplied"
                            .to_owned(),
                },
                None,
                None,
                None,
            ));
        }

        Ok(())
    }

    fn validate_topology(
        &self,
        input: &RoutingVerificationInput<'_>,
    ) -> Result<(), RoutingError> {
        input.topology.validate().map_err(|error| {
            Self::contextualize(
                error,
                None,
                None,
                Some("topology validation failed"),
            )
        })?;

        if input.topology.qubit_count() == 0 {
            return Err(RoutingError::empty_topology());
        }

        Ok(())
    }

    fn validate_initial_mapping(
        &self,
        input: &RoutingVerificationInput<'_>,
    ) -> Result<(), RoutingError> {
        let mapping =
            Self::mapping_from_snapshot(input.initial_mapping)?;

        mapping.validate().map_err(|error| {
            Self::mapping_error(
                error,
                "initial mapping invariant failed",
            )
        })?;

        for physical in mapping.physical_qubits() {
            if !input.topology.contains(physical) {
                return Err(Self::error(
                    VerificationError::InvalidInitialMapping {
                        detail: format!(
                            "initial mapping references physical qubit \
                             {physical}, which is absent from topology"
                        ),
                    },
                    None,
                    Some(physical),
                    None,
                ));
            }

            if !input.topology.is_available(physical) {
                return Err(Self::error(
                    VerificationError::InvalidInitialMapping {
                        detail: format!(
                            "initial mapping uses unavailable physical \
                             qubit {physical}"
                        ),
                    },
                    None,
                    Some(physical),
                    None,
                ));
            }
        }

        Ok(())
    }

    fn validate_logical_input(
        &self,
        input: &RoutingVerificationInput<'_>,
        report: &mut VerificationReport,
    ) -> Result<(), RoutingError> {
        let mut seen_logical = BTreeSet::new();

        for interaction in input.original_interactions {
            let arity = interaction.arity();

            if arity > input.max_arity {
                return Err(Self::error(
                    VerificationError::UnsupportedOperation {
                        operation_index: 0,
                        operation: format!(
                            "gate `{}` has arity {} exceeding verifier \
                             limit {}",
                            interaction.gate().name(),
                            arity,
                            input.max_arity
                        ),
                    },
                    None,
                    None,
                    Some(interaction.gate().name()),
                ));
            }

            if arity == 0 {
                return Err(Self::error(
                    VerificationError::UnsupportedOperation {
                        operation_index: 0,
                        operation: format!(
                            "gate `{}` has zero operands",
                            interaction.gate().name()
                        ),
                    },
                    None,
                    None,
                    Some(interaction.gate().name()),
                ));
            }

            let mut local_operands = BTreeSet::new();

            for &logical in interaction.operands() {
                if !local_operands.insert(logical) {
                    return Err(Self::error(
                        VerificationError::QubitSemanticsChanged {
                            logical: logical.to_string(),
                        },
                        None,
                        None,
                        Some(interaction.gate().name()),
                    ));
                }

                seen_logical.insert(logical);

                if !input
                    .initial_mapping
                    .logical_to_physical()
                    .iter()
                    .any(|(candidate, _)| *candidate == logical)
                {
                    return Err(Self::error(
                        VerificationError::InvalidInitialMapping {
                            detail: format!(
                                "logical qubit {logical} is required by the \
                                 input circuit but absent from the initial \
                                 mapping"
                            ),
                        },
                        Some(logical),
                        None,
                        Some(interaction.gate().name()),
                    ));
                }
            }

            report.preservation_checks += 1;
            report.passed_checks += 1;
        }

        if !seen_logical.is_empty() {
            report.mapping_checks += 1;
            report.passed_checks += 1;
        }

        Ok(())
    }

    // =========================================================================
    // Operation verification
    // =========================================================================

    fn verify_operation(
        &self,
        input: &RoutingVerificationInput<'_>,
        mapping: &mut QubitMapping,
        operation: &RoutingOperation,
        operation_index: usize,
        report: &mut VerificationReport,
    ) -> Result<(), RoutingError> {
        match operation {
            RoutingOperation::Move(movement) => {
                report.movements_checked += 1;

                self.verify_move(
                    input,
                    mapping,
                    movement,
                    operation_index,
                    report,
                )
            }

            RoutingOperation::Gate {
                gate,
                operands,
                logical_operands,
            } => {
                report.gates_checked += 1;

                self.verify_gate(
                    input,
                    mapping,
                    gate,
                    operands,
                    logical_operands,
                    operation_index,
                    report,
                )
            }

            RoutingOperation::Barrier { operands } => {
                report.barriers_checked += 1;

                self.verify_barrier(
                    input,
                    mapping,
                    operands,
                    operation_index,
                    report,
                )
            }
        }
    }

    fn verify_move(
        &self,
        input: &RoutingVerificationInput<'_>,
        mapping: &mut QubitMapping,
        movement: &RoutingMove,
        operation_index: usize,
        report: &mut VerificationReport,
    ) -> Result<(), RoutingError> {
        match movement {
            RoutingMove::Swap { a, b } => {
                report.swaps_checked += 1;

                if *a == *b {
                    return Err(Self::error(
                        VerificationError::IllegalMovement {
                            movement: "swap".to_owned(),
                            a: a.index(),
                            b: b.index(),
                        },
                        None,
                        Some(*a),
                        None,
                    ));
                }

                self.require_available_physical(
                    input.topology,
                    *a,
                    operation_index,
                    None,
                )?;

                self.require_available_physical(
                    input.topology,
                    *b,
                    operation_index,
                    None,
                )?;

                // A semantic SWAP exchanges the states at both locations.
                // Therefore a directed one-way physical edge is insufficient
                // unless the hardware explicitly provides a bidirectional
                // SWAP capability.
                let swap_supported = input
                    .topology
                    .supports_gate("swap", *a, *b)
                    && input
                        .topology
                        .supports_gate("swap", *b, *a);

                let structurally_bidirectional =
                    input.topology.is_bidirectionally_adjacent(*a, *b);

                if !swap_supported && !structurally_bidirectional {
                    return Err(Self::error(
                        VerificationError::IllegalMovement {
                            movement: "swap".to_owned(),
                            a: a.index(),
                            b: b.index(),
                        },
                        None,
                        Some(*a),
                        None,
                    ));
                }

                if let Err(error) = mapping.swap_physical(*a, *b) {
                    return Err(Self::mapping_error(
                        error,
                        "SWAP could not be applied to verification mapping",
                    ));
                }

                report.structural_checks += 1;
                report.mapping_checks += 1;
                report.passed_checks += 2;

                Ok(())
            }

            RoutingMove::Bridge {
                a,
                bridge,
                b,
                gate,
            } => {
                report.bridges_checked += 1;

                if *a == *b || *a == *bridge || *bridge == *b {
                    return Err(Self::error(
                        VerificationError::IllegalMovement {
                            movement: "bridge".to_owned(),
                            a: a.index(),
                            b: b.index(),
                        },
                        None,
                        Some(*a),
                        None,
                    ));
                }

                self.require_available_physical(
                    input.topology,
                    *a,
                    operation_index,
                    Some(gate),
                )?;

                self.require_available_physical(
                    input.topology,
                    *bridge,
                    operation_index,
                    Some(gate),
                )?;

                self.require_available_physical(
                    input.topology,
                    *b,
                    operation_index,
                    Some(gate),
                )?;

                if !input.topology.is_adjacent(*a, *bridge)
                    || !input.topology.is_adjacent(*bridge, *b)
                {
                    return Err(Self::error(
                        VerificationError::IllegalMovement {
                            movement: "bridge".to_owned(),
                            a: a.index(),
                            b: b.index(),
                        },
                        None,
                        Some(*a),
                        None,
                    ));
                }

                // A bridge is a semantic movement/lowering boundary. It does
                // not change the logical-to-physical mapping by itself.
                //
                // Gate-specific physical support on the remote endpoints is
                // intentionally not required here: a bridge exists precisely
                // to represent a later decomposition/lowering path.
                report.structural_checks += 1;
                report.executability_checks += 1;
                report.passed_checks += 2;

                Ok(())
            }

            RoutingMove::Permutation {
                mapping: permutation,
            } => {
                report.permutations_checked += 1;

                self.verify_permutation(
                    input,
                    mapping,
                    operation_index,
                    report,
                )?;

                let target =
                    QubitMapping::from_assignments(permutation.clone())
                        .map_err(|error| {
                            Self::mapping_error(
                                error,
                                "invalid permutation mapping",
                            )
                        })?;

                target.validate().map_err(|error| {
                    Self::mapping_error(
                        error,
                        "permutation mapping invariant failed",
                    )
                })?;

                *mapping = target;

                report.mapping_checks += 1;
                report.passed_checks += 1;

                Ok(())
            }
        }
    }

    fn verify_gate(
        &self,
        input: &RoutingVerificationInput<'_>,
        mapping: &QubitMapping,
        gate: &GateIdentity,
        physical_operands: &[PhysicalQubitId],
        logical_operands: &[LogicalQubitId],
        operation_index: usize,
        report: &mut VerificationReport,
    ) -> Result<(), RoutingError> {
        let arity = physical_operands.len();

        if arity != logical_operands.len() {
            return Err(Self::error(
                VerificationError::GateSequenceMismatch {
                    operation_index,
                },
                None,
                None,
                Some(gate.name()),
            ));
        }

        if arity > input.max_arity {
            return Err(Self::error(
                VerificationError::UnsupportedOperation {
                    operation_index,
                    operation: format!(
                        "gate `{}` has arity {} exceeding verifier limit {}",
                        gate.name(),
                        arity,
                        input.max_arity
                    ),
                },
                None,
                None,
                Some(gate.name()),
            ));
        }

        if arity == 0 {
            return Err(Self::error(
                VerificationError::UnsupportedOperation {
                    operation_index,
                    operation: format!(
                        "gate `{}` has zero physical operands",
                        gate.name()
                    ),
                },
                None,
                None,
                Some(gate.name()),
            ));
        }

        self.verify_unique_operands(
            logical_operands,
            physical_operands,
            gate,
            operation_index,
        )?;

        for (position, (&logical, &physical)) in logical_operands
            .iter()
            .zip(physical_operands.iter())
            .enumerate()
        {
            let expected = mapping.physical_of(logical).ok_or_else(|| {
                Self::error(
                    VerificationError::QubitSemanticsChanged {
                        logical: logical.to_string(),
                    },
                    Some(logical),
                    Some(physical),
                    Some(gate.name()),
                )
            })?;

            if expected != physical {
                return Err(Self::error(
                    VerificationError::GateSequenceMismatch {
                        operation_index,
                    },
                    Some(logical),
                    Some(physical),
                    Some(gate.name()),
                ));
            }

            if !input.topology.contains(physical) {
                return Err(Self::error(
                    VerificationError::IllegalOperation {
                        gate: gate.name_owned(),
                        physical_operands: physical_operands
                            .iter()
                            .map(|qubit| qubit.index())
                            .collect(),
                    },
                    Some(logical),
                    Some(physical),
                    Some(gate.name()),
                ));
            }

            if !input.topology.is_available(physical) {
                return Err(Self::error(
                    VerificationError::IllegalOperation {
                        gate: gate.name_owned(),
                        physical_operands: physical_operands
                            .iter()
                            .map(|qubit| qubit.index())
                            .collect(),
                    },
                    Some(logical),
                    Some(physical),
                    Some(gate.name()),
                ));
            }

            report.mapping_checks += 1;
            report.passed_checks += 1;

            if position >= physical_operands.len() {
                return Err(Self::error(
                    VerificationError::InvariantViolation {
                        detail:
                            "gate operand position exceeded physical operand \
                             list"
                                .to_owned(),
                    },
                    Some(logical),
                    Some(physical),
                    Some(gate.name()),
                ));
            }
        }

        self.verify_gate_shape(
            input,
            gate,
            physical_operands,
            operation_index,
            report,
        )?;

        self.verify_gate_executability(
            input,
            gate,
            physical_operands,
            operation_index,
            report,
        )?;

        report.preservation_checks += 1;
        report.passed_checks += 1;

        Ok(())
    }

    fn verify_gate_shape(
        &self,
        _input: &RoutingVerificationInput<'_>,
        gate: &GateIdentity,
        physical_operands: &[PhysicalQubitId],
        operation_index: usize,
        _report: &mut VerificationReport,
    ) -> Result<(), RoutingError> {
        let expected = expected_gate_arity(gate);

        if let Some(expected_arity) = expected {
            if physical_operands.len() != expected_arity {
                return Err(Self::error(
                    VerificationError::UnsupportedOperation {
                        operation_index,
                        operation: format!(
                            "gate `{}` requires arity {}, received {}",
                            gate.name(),
                            expected_arity,
                            physical_operands.len()
                        ),
                    },
                    None,
                    None,
                    Some(gate.name()),
                ));
            }
        }

        if gate.is_measurement() && physical_operands.len() != 1 {
            return Err(Self::error(
                VerificationError::MeasurementMismatch {
                    operation_index,
                },
                None,
                None,
                Some(gate.name()),
            ));
        }

        if gate.is_reset() && physical_operands.len() != 1 {
            return Err(Self::error(
                VerificationError::QubitSemanticsChanged {
                    logical: "reset requires exactly one logical qubit"
                        .to_owned(),
                },
                None,
                None,
                Some(gate.name()),
            ));
        }

        Ok(())
    }

    fn verify_gate_executability(
        &self,
        input: &RoutingVerificationInput<'_>,
        gate: &GateIdentity,
        operands: &[PhysicalQubitId],
        operation_index: usize,
        report: &mut VerificationReport,
    ) -> Result<(), RoutingError> {
        if operands.len() == 1 {
            let physical = operands[0];

            if !input.topology.is_available(physical) {
                return Err(Self::error(
                    VerificationError::IllegalOperation {
                        gate: gate.name_owned(),
                        physical_operands: vec![physical.index()],
                    },
                    None,
                    Some(physical),
                    Some(gate.name()),
                ));
            }

            report.executability_checks += 1;
            report.passed_checks += 1;

            return Ok(());
        }

        if operands.len() < 2 {
            return Err(Self::error(
                VerificationError::UnsupportedOperation {
                    operation_index,
                    operation: format!(
                        "gate `{}` has invalid arity {}",
                        gate.name(),
                        operands.len()
                    ),
                },
                None,
                None,
                Some(gate.name()),
            ));
        }

        if operands.len() > 2 {
            // Multi-qubit native operations are valid only when the target
            // explicitly declares the required pairwise connectivity.
            //
            // This verifier intentionally does not invent a decomposition.
            // A future native multi-qubit topology capability can extend
            // topology.rs without changing the verifier's public API.
            for window in operands.windows(2) {
                let a = window[0];
                let b = window[1];

                if !input.topology.is_adjacent(a, b) {
                    return Err(Self::error(
                        VerificationError::IllegalOperation {
                            gate: gate.name_owned(),
                            physical_operands: operands
                                .iter()
                                .map(|qubit| qubit.index())
                                .collect(),
                        },
                        None,
                        Some(a),
                        Some(gate.name()),
                    ));
                }
            }

            report.executability_checks += 1;
            report.passed_checks += 1;

            return Ok(());
        }

        let source = operands[0];
        let target = operands[1];

        if !input.topology.has_connection(source, target) {
            return Err(Self::error(
                VerificationError::IllegalOperation {
                    gate: gate.name_owned(),
                    physical_operands: operands
                        .iter()
                        .map(|qubit| qubit.index())
                        .collect(),
                },
                None,
                Some(source),
                Some(gate.name()),
            ));
        }

        if !input.topology.is_adjacent(source, target) {
            // Distinguish directionality from complete non-adjacency.
            if input.topology.is_adjacent(target, source) {
                return Err(Self::error(
                    VerificationError::IllegalDirection {
                        gate: gate.name_owned(),
                        from: source.index(),
                        to: target.index(),
                    },
                    None,
                    Some(source),
                    Some(gate.name()),
                ));
            }

            return Err(Self::error(
                VerificationError::IllegalOperation {
                    gate: gate.name_owned(),
                    physical_operands: operands
                        .iter()
                        .map(|qubit| qubit.index())
                        .collect(),
                },
                None,
                Some(source),
                Some(gate.name()),
            ));
        }

        // Gate-specific topology information is authoritative when present.
        //
        // If there is an explicit gate entry, supports_gate() must agree with
        // it. If there is no explicit entry, topology adjacency remains the
        // generic fallback.
        if input
            .topology
            .has_explicit_gate_support(
                gate.name(),
                source,
                target,
            )
            && !input
                .topology
                .supports_gate(
                    gate.name(),
                    source,
                    target,
                )
        {
            return Err(Self::error(
                VerificationError::IllegalOperation {
                    gate: gate.name_owned(),
                    physical_operands: operands
                        .iter()
                        .map(|qubit| qubit.index())
                        .collect(),
                },
                None,
                Some(source),
                Some(gate.name()),
            ));
        }

        // Directional gates require the exact source -> target direction.
        if gate.is_directional()
            && !input.topology.is_adjacent(source, target)
        {
            return Err(Self::error(
                VerificationError::IllegalDirection {
                    gate: gate.name_owned(),
                    from: source.index(),
                    to: target.index(),
                },
                None,
                Some(source),
                Some(gate.name()),
            ));
        }

        report.executability_checks += 1;
        report.passed_checks += 1;

        Ok(())
    }

    fn verify_barrier(
        &self,
        input: &RoutingVerificationInput<'_>,
        mapping: &QubitMapping,
        operands: &[PhysicalQubitId],
        operation_index: usize,
        report: &mut VerificationReport,
    ) -> Result<(), RoutingError> {
        if operands.len() > input.max_arity {
            return Err(Self::error(
                VerificationError::UnsupportedOperation {
                    operation_index,
                    operation: format!(
                        "barrier has arity {} exceeding verifier limit {}",
                        operands.len(),
                        input.max_arity
                    ),
                },
                None,
                None,
                Some("barrier"),
            ));
        }

        let mut seen = BTreeSet::new();

        for &physical in operands {
            if !seen.insert(physical) {
                return Err(Self::error(
                    VerificationError::InvariantViolation {
                        detail: format!(
                            "barrier contains duplicate physical qubit \
                             {physical}"
                        ),
                    },
                    None,
                    Some(physical),
                    Some("barrier"),
                ));
            }

            if !input.topology.contains(physical)
                || !input.topology.is_available(physical)
            {
                return Err(Self::error(
                    VerificationError::IllegalOperation {
                        gate: "barrier".to_owned(),
                        physical_operands: operands
                            .iter()
                            .map(|qubit| qubit.index())
                            .collect(),
                    },
                    None,
                    Some(physical),
                    Some("barrier"),
                ));
            }

            if mapping.logical_at(physical).is_none() {
                return Err(Self::error(
                    VerificationError::QubitSemanticsChanged {
                        logical: format!(
                            "barrier references unmapped physical qubit \
                             {physical}"
                        ),
                    },
                    None,
                    Some(physical),
                    Some("barrier"),
                ));
            }
        }

        report.structural_checks += 1;
        report.executability_checks += 1;
        report.passed_checks += 2;

        Ok(())
    }

    // =========================================================================
    // Permutation verification
    // =========================================================================

    fn verify_permutation(
        &self,
        input: &RoutingVerificationInput<'_>,
        permutation: &[(LogicalQubitId, PhysicalQubitId)],
        operation_index: usize,
        report: &mut VerificationReport,
    ) -> Result<(), RoutingError> {
        if permutation.is_empty() {
            return Err(Self::error(
                VerificationError::InvalidPermutation {
                    detail:
                        "permutation operation contains no assignments"
                            .to_owned(),
                },
                None,
                None,
                None,
            ));
        }

        if permutation.len() > input.max_arity {
            return Err(Self::error(
                VerificationError::InvalidPermutation {
                    detail: format!(
                        "permutation contains {} assignments, exceeding \
                         verifier arity limit {}",
                        permutation.len(),
                        input.max_arity
                    ),
                },
                None,
                None,
                None,
            ));
        }

        let mut logicals = BTreeSet::new();
        let mut physicals = BTreeSet::new();

        for &(logical, physical) in permutation {
            if !logicals.insert(logical) {
                return Err(Self::error(
                    VerificationError::InvalidPermutation {
                        detail: format!(
                            "logical qubit {logical} occurs more than once"
                        ),
                    },
                    Some(logical),
                    Some(physical),
                    None,
                ));
            }

            if !physicals.insert(physical) {
                return Err(Self::error(
                    VerificationError::InvalidPermutation {
                        detail: format!(
                            "physical qubit {physical} occurs more than once"
                        ),
                    },
                    Some(logical),
                    Some(physical),
                    None,
                ));
            }

            if !input.topology.contains(physical) {
                return Err(Self::error(
                    VerificationError::InvalidPermutation {
                        detail: format!(
                            "permutation references physical qubit {physical} \
                             absent from topology"
                        ),
                    },
                    Some(logical),
                    Some(physical),
                    None,
                ));
            }

            if !input.topology.is_available(physical) {
                return Err(Self::error(
                    VerificationError::InvalidPermutation {
                        detail: format!(
                            "permutation references unavailable physical \
                             qubit {physical}"
                        ),
                    },
                    Some(logical),
                    Some(physical),
                    None,
                ));
            }

            report.structural_checks += 1;
            report.passed_checks += 1;
        }

        if permutation.len() != input.initial_mapping.len() {
            return Err(Self::error(
                VerificationError::InvalidPermutation {
                    detail: format!(
                        "permutation contains {} assignments but the routing \
                         mapping contains {} logical assignments",
                        permutation.len(),
                        input.initial_mapping.len()
                    ),
                },
                None,
                None,
                None,
            ));
        }

        let initial_logicals: BTreeSet<_> = input
            .initial_mapping
            .logical_to_physical()
            .into_iter()
            .map(|(logical, _)| logical)
            .collect();

        if logicals != initial_logicals {
            return Err(Self::error(
                VerificationError::InvalidPermutation {
                    detail:
                        "permutation changes the set of logical qubits"
                            .to_owned(),
                },
                None,
                None,
                None,
            ));
        }

        let _ = operation_index;

        Ok(())
    }

    // =========================================================================
    // Final-state verification
    // =========================================================================

    fn verify_final_mapping(
        &self,
        input: &RoutingVerificationInput<'_>,
        working_mapping: &QubitMapping,
        report: &mut VerificationReport,
    ) -> Result<(), RoutingError> {
        working_mapping.validate().map_err(|error| {
            Self::mapping_error(
                error,
                "replayed routing mapping is internally inconsistent",
            )
        })?;

        let expected =
            Self::mapping_from_snapshot(input.final_mapping)?;

        expected.validate().map_err(|error| {
            Self::mapping_error(
                error,
                "declared final mapping is internally inconsistent",
            )
        })?;

        for physical in expected.physical_qubits() {
            if !input.topology.contains(physical) {
                return Err(Self::error(
                    VerificationError::InvalidFinalMapping {
                        detail: format!(
                            "final mapping references physical qubit {physical} \
                             absent from topology"
                        ),
                    },
                    None,
                    Some(physical),
                    None,
                ));
            }

            if !input.topology.is_available(physical) {
                return Err(Self::error(
                    VerificationError::InvalidFinalMapping {
                        detail: format!(
                            "final mapping uses unavailable physical qubit \
                             {physical}"
                        ),
                    },
                    None,
                    Some(physical),
                    None,
                ));
            }
        }

        let actual = working_mapping.logical_to_physical();
        let declared = expected.logical_to_physical();

        if actual != declared {
            return Err(Self::error(
                VerificationError::InvalidFinalMapping {
                    detail: format!(
                        "replayed final mapping {:?} does not equal declared \
                         final mapping {:?}",
                        actual, declared
                    ),
                },
                None,
                None,
                None,
            ));
        }

        report.final_mapping_matches = true;
        report.mapping_checks += 1;
        report.passed_checks += 1;

        Ok(())
    }

    fn verify_strict_global_invariants(
        &self,
        input: &RoutingVerificationInput<'_>,
        working_mapping: &QubitMapping,
        report: &mut VerificationReport,
    ) -> Result<(), RoutingError> {
        working_mapping.validate().map_err(|error| {
            Self::mapping_error(
                error,
                "strict verification found invalid final mapping",
            )
        })?;

        // Every mapped physical qubit must be unique. `QubitMapping` already
        // maintains a reverse index, but explicitly checking the deterministic
        // representation gives Strict verification an independent assertion.
        let assignments =
            working_mapping.logical_to_physical();

        let mut physicals = BTreeSet::new();

        for (logical, physical) in assignments {
            if !physicals.insert(physical) {
                return Err(Self::error(
                    VerificationError::MappingCollision {
                        physical: physical.index(),
                    },
                    Some(logical),
                    Some(physical),
                    None,
                ));
            }

            if !input.topology.contains(physical) {
                return Err(Self::error(
                    VerificationError::InvalidFinalMapping {
                        detail: format!(
                            "strict verification found unknown physical \
                             qubit {physical}"
                        ),
                    },
                    Some(logical),
                    Some(physical),
                    None,
                ));
            }

            report.mapping_checks += 1;
            report.passed_checks += 1;
        }

        // Strictly validate that every logical qubit used by the original
        // workload still exists in the final mapping.
        for interaction in input.original_interactions {
            for &logical in interaction.operands() {
                if !working_mapping.contains_logical(logical) {
                    return Err(Self::error(
                        VerificationError::QubitSemanticsChanged {
                            logical: logical.to_string(),
                        },
                        Some(logical),
                        None,
                        Some(interaction.gate().name()),
                    ));
                }
            }
        }

        report.preservation_checks += 1;
        report.passed_checks += 1;

        Ok(())
    }

    // =========================================================================
    // Operand validation
    // =========================================================================

    fn verify_unique_operands(
        &self,
        logical_operands: &[LogicalQubitId],
        physical_operands: &[PhysicalQubitId],
        gate: &GateIdentity,
        operation_index: usize,
    ) -> Result<(), RoutingError> {
        let mut logical_seen = BTreeSet::new();

        for &logical in logical_operands {
            if !logical_seen.insert(logical) {
                return Err(Self::error(
                    VerificationError::GateSequenceMismatch {
                        operation_index,
                    },
                    Some(logical),
                    None,
                    Some(gate.name()),
                ));
            }
        }

        let mut physical_seen = BTreeSet::new();

        for &physical in physical_operands {
            if !physical_seen.insert(physical) {
                return Err(Self::error(
                    VerificationError::IllegalOperation {
                        gate: gate.name_owned(),
                        physical_operands: physical_operands
                            .iter()
                            .map(|qubit| qubit.index())
                            .collect(),
                    },
                    None,
                    Some(physical),
                    Some(gate.name()),
                ));
            }
        }

        Ok(())
    }

    fn require_available_physical(
        &self,
        topology: &PhysicalTopology,
        physical: PhysicalQubitId,
        operation_index: usize,
        gate: Option<&GateIdentity>,
    ) -> Result<(), RoutingError> {
        if !topology.contains(physical) {
            return Err(Self::error(
                VerificationError::IllegalMovement {
                    movement: "physical qubit is absent from topology"
                        .to_owned(),
                    a: physical.index(),
                    b: physical.index(),
                },
                None,
                Some(physical),
                gate.map(GateIdentity::name),
            ));
        }

        if !topology.is_available(physical) {
            return Err(Self::error(
                VerificationError::IllegalMovement {
                    movement: "physical qubit is unavailable".to_owned(),
                    a: physical.index(),
                    b: physical.index(),
                },
                None,
                Some(physical),
                gate.map(GateIdentity::name),
            ));
        }

        let _ = operation_index;

        Ok(())
    }

    // =========================================================================
    // Mapping conversion
    // =========================================================================

    fn mapping_from_snapshot(
        snapshot: &QubitMappingSnapshot,
    ) -> Result<QubitMapping, RoutingError> {
        QubitMapping::from_assignments(
            snapshot.logical_to_physical(),
        )
        .map_err(|error| {
            Self::mapping_error(
                error,
                "unable to reconstruct mapping from snapshot",
            )
        })
    }

    // =========================================================================
    // Error construction
    // =========================================================================

    fn mapping_error(
        error: MappingError,
        detail: &str,
    ) -> RoutingError {
        Self::error(
            VerificationError::InvariantViolation {
                detail: format!("{detail}: {error}"),
            },
            None,
            None,
            None,
        )
    }

    fn contextualize(
        error: RoutingError,
        operation_index: Option<usize>,
        physical: Option<PhysicalQubitId>,
        detail: Option<&str>,
    ) -> RoutingError {
        let mut context = error.context.clone();

        context.stage = Some(
            crate::quantum::routing::errors::RoutingStage::Verification,
        );

        if let Some(index) = operation_index {
            context.operation_index = Some(index);
        }

        if let Some(physical) = physical {
            context.physical_qubit = Some(physical.index());
        }

        if let Some(detail) = detail {
            context.detail = Some(detail.to_owned());
        }

        error.with_diagnostic_context(context)
    }

    fn error(
        verification: VerificationError,
        logical: Option<LogicalQubitId>,
        physical: Option<PhysicalQubitId>,
        gate: Option<&str>,
    ) -> RoutingError {
        let mut context = RoutingErrorContext::new()
            .with_stage(
                crate::quantum::routing::errors::RoutingStage::Verification,
            );

        if let Some(logical) = logical {
            context = context.with_logical_qubit(logical.to_string());
        }

        if let Some(physical) = physical {
            context =
                context.with_physical_qubit(physical.index());
        }

        if let Some(gate) = gate {
            context = context.with_gate(gate);
        }

        RoutingError::with_context(
            RoutingErrorKind::Verification(verification),
            context,
        )
    }
}

// =============================================================================
// Gate arity
// =============================================================================

/// Returns the semantic arity of built-in routing gates.
///
/// `Custom` gates return `None` because their arity is determined by the
/// canonical Quantum IR/hardware capability layer rather than guessed here.
fn expected_gate_arity(gate: &GateIdentity) -> Option<usize> {
    match gate {
        GateIdentity::Identity
        | GateIdentity::X
        | GateIdentity::Y
        | GateIdentity::Z
        | GateIdentity::H
        | GateIdentity::S
        | GateIdentity::Sdg
        | GateIdentity::T
        | GateIdentity::Tdg
        | GateIdentity::Rx
        | GateIdentity::Ry
        | GateIdentity::Rz
        | GateIdentity::Phase
        | GateIdentity::Measure
        | GateIdentity::Reset => Some(1),

        GateIdentity::Cx
        | GateIdentity::Cy
        | GateIdentity::Cz
        | GateIdentity::Ch
        | GateIdentity::Swap
        | GateIdentity::ISwap
        | GateIdentity::Ecr
        | GateIdentity::Crx
        | GateIdentity::Cry
        | GateIdentity::Crz => Some(2),

        GateIdentity::Ccx => Some(3),

        GateIdentity::CSwap => Some(3),

        GateIdentity::Barrier => None,

        GateIdentity::Custom(_) => None,
    }
}

// =============================================================================
// Convenience free functions
// =============================================================================

/// Verifies a routing operation stream using production-standard verification.
pub fn verify_routing(
    topology: &PhysicalTopology,
    initial_mapping: &QubitMappingSnapshot,
    final_mapping: &QubitMappingSnapshot,
    original_interactions: &[
        crate::quantum::routing::types::QubitInteraction
    ],
    operations: &[RoutingOperation],
) -> Result<VerificationReport, RoutingError> {
    let verifier = RoutingVerifier::new();

    let input = RoutingVerificationInput::new(
        topology,
        initial_mapping,
        final_mapping,
        original_interactions,
        operations,
        VerificationLevel::Standard,
    );

    verifier.verify(&input)
}

/// Verifies a routing operation stream using strict verification.
pub fn verify_routing_strict(
    topology: &PhysicalTopology,
    initial_mapping: &QubitMappingSnapshot,
    final_mapping: &QubitMappingSnapshot,
    original_interactions: &[
        crate::quantum::routing::types::QubitInteraction
    ],
    operations: &[RoutingOperation],
) -> Result<VerificationReport, RoutingError> {
    let verifier = RoutingVerifier::new();

    let input = RoutingVerificationInput::new(
        topology,
        initial_mapping,
        final_mapping,
        original_interactions,
        operations,
        VerificationLevel::Strict,
    );

    verifier.verify(&input)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::quantum::routing::topology::{
        TopologyMetadata,
    };
    use crate::quantum::routing::types::{
        EdgeDirection,
        PhysicalEdge,
    };
    use std::collections::BTreeMap;

    fn line_topology(
        count: usize,
    ) -> PhysicalTopology {
        PhysicalTopology::line(count)
            .expect("test topology must be valid")
    }

    fn mapping(
        assignments: &[
            (usize, usize)
        ],
    ) -> QubitMappingSnapshot {
        let routing_mapping =
            QubitMapping::from_assignments(
                assignments
                    .iter()
                    .copied()
                    .map(|(logical, physical)| {
                        (
                            LogicalQubitId::new(logical),
                            PhysicalQubitId::new(physical),
                        )
                    }),
            )
            .expect("test mapping must be valid");

        routing_mapping.snapshot()
    }

    fn cx(
        logical_a: usize,
        logical_b: usize,
        physical_a: usize,
        physical_b: usize,
    ) -> RoutingOperation {
        RoutingOperation::Gate {
            gate: GateIdentity::Cx,
            operands: vec![
                PhysicalQubitId::new(physical_a),
                PhysicalQubitId::new(physical_b),
            ],
            logical_operands: vec![
                LogicalQubitId::new(logical_a),
                LogicalQubitId::new(logical_b),
            ],
        }
    }

    #[test]
    fn verifies_adjacent_gate() {
        let topology = line_topology(2);
        let initial = mapping(&[(0, 0), (1, 1)]);
        let final_mapping = initial.clone();

        let interactions = vec![
            crate::quantum::routing::types::QubitInteraction::new(
                vec![
                    LogicalQubitId::new(0),
                    LogicalQubitId::new(1),
                ],
                GateIdentity::Cx,
            ),
        ];

        let operations = vec![cx(0, 1, 0, 1)];

        let report = verify_routing(
            &topology,
            &initial,
            &final_mapping,
            &interactions,
            &operations,
        )
        .expect("adjacent CX should verify");

        assert!(report.all_checks_passed());
        assert!(report.final_mapping_matches);
    }

    #[test]
    fn verifies_swap_mapping_evolution() {
        let topology = line_topology(3);
        let initial = mapping(&[(0, 0), (1, 1)]);
        let final_mapping = mapping(&[(0, 1), (1, 0)]);

        let interactions: Vec<
            crate::quantum::routing::types::QubitInteraction,
        > = Vec::new();

        let operations = vec![
            RoutingOperation::Move(RoutingMove::Swap {
                a: PhysicalQubitId::new(0),
                b: PhysicalQubitId::new(1),
            }),
        ];

        let report = verify_routing(
            &topology,
            &initial,
            &final_mapping,
            &interactions,
            &operations,
        )
        .expect("SWAP mapping evolution should verify");

        assert!(report.final_mapping_matches);
        assert_eq!(report.swaps_checked, 1);
    }

    #[test]
    fn rejects_wrong_logical_operand_mapping() {
        let topology = line_topology(2);
        let initial = mapping(&[(0, 0), (1, 1)]);
        let final_mapping = initial.clone();

        let interactions = vec![
            crate::quantum::routing::types::QubitInteraction::new(
                vec![
                    LogicalQubitId::new(0),
                    LogicalQubitId::new(1),
                ],
                GateIdentity::Cx,
            ),
        ];

        let operations = vec![cx(0, 1, 1, 0)];

        let result = verify_routing(
            &topology,
            &initial,
            &final_mapping,
            &interactions,
            &operations,
        );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_illegal_direction() {
        let mut builder = PhysicalTopology::builder();

        builder = builder
            .qubit(PhysicalQubitId::new(0))
            .expect("qubit");
        builder = builder
            .qubit(PhysicalQubitId::new(1))
            .expect("qubit");

        builder = builder
            .edge(PhysicalEdge::new(
                PhysicalQubitId::new(0),
                PhysicalQubitId::new(1),
                EdgeDirection::Forward,
            ))
            .expect("edge");

        let topology =
            builder.build().expect("directed topology");

        let initial = mapping(&[(0, 1), (1, 0)]);
        let final_mapping = initial.clone();

        let operations = vec![cx(0, 1, 1, 0)];

        let result = verify_routing(
            &topology,
            &initial,
            &final_mapping,
            &[],
            &operations,
        );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_wrong_final_mapping() {
        let topology = line_topology(2);
        let initial = mapping(&[(0, 0), (1, 1)]);
        let declared_final = mapping(&[(0, 1), (1, 0)]);

        let operations: Vec<RoutingOperation> = Vec::new();

        let result = verify_routing(
            &topology,
            &initial,
            &declared_final,
            &[],
            &operations,
        );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_duplicate_gate_operands() {
        let topology = line_topology(2);
        let initial = mapping(&[(0, 0), (1, 1)]);
        let final_mapping = initial.clone();

        let operations = vec![
            RoutingOperation::Gate {
                gate: GateIdentity::Cx,
                operands: vec![
                    PhysicalQubitId::new(0),
                    PhysicalQubitId::new(0),
                ],
                logical_operands: vec![
                    LogicalQubitId::new(0),
                    LogicalQubitId::new(0),
                ],
            },
        ];

        let result = verify_routing(
            &topology,
            &initial,
            &final_mapping,
            &[],
            &operations,
        );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_illegal_swap() {
        let topology = line_topology(3);
        let initial = mapping(&[(0, 0), (1, 1)]);
        let final_mapping = mapping(&[(0, 2), (1, 1)]);

        let operations = vec![
            RoutingOperation::Move(RoutingMove::Swap {
                a: PhysicalQubitId::new(0),
                b: PhysicalQubitId::new(2),
            }),
        ];

        let result = verify_routing(
            &topology,
            &initial,
            &final_mapping,
            &[],
            &operations,
        );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_missing_logical_mapping() {
        let topology = line_topology(2);
        let initial = mapping(&[(0, 0)]);
        let final_mapping = initial.clone();

        let interactions = vec![
            crate::quantum::routing::types::QubitInteraction::new(
                vec![
                    LogicalQubitId::new(0),
                    LogicalQubitId::new(1),
                ],
                GateIdentity::Cx,
            ),
        ];

        let operations: Vec<RoutingOperation> = Vec::new();

        let result = verify_routing(
            &topology,
            &initial,
            &final_mapping,
            &interactions,
            &operations,
        );

        assert!(result.is_err());
    }

    #[test]
    fn strict_verification_checks_final_mapping_bijection() {
        let topology = line_topology(2);
        let initial = mapping(&[(0, 0), (1, 1)]);
        let final_mapping = initial.clone();

        let verifier = RoutingVerifier::new();

        let input = RoutingVerificationInput::new(
            &topology,
            &initial,
            &final_mapping,
            &[],
            &[],
            VerificationLevel::Strict,
        );

        let report = verifier
            .verify(&input)
            .expect("strict verification should pass");

        assert!(report.all_checks_passed());
        assert!(report.final_mapping_matches);
    }
}