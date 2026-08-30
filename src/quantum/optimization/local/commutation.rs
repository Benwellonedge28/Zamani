//! Zamani Quantum Optimization — Commutation-Aware Local Reordering
//!
//! Production-grade, exact, deterministic commutation-aware gate movement over
//! Zamani's canonical Quantum IR.
//!
//! # Architectural position
//!
//! ```text
//!                    crate::quantum::ir
//!                           │
//!                           ▼
//!              optimization::analysis::commutation
//!                           │
//!                           ▼
//!               local::commutation
//!                           │
//!              ┌────────────┴────────────┐
//!              ▼                         ▼
//!       local cancellation        local peephole
//!              │                         │
//!              └────────────┬────────────┘
//!                           ▼
//!                    optimized Quantum IR
//! ```
//!
//! This module performs one narrowly defined responsibility:
//!
//! > Move an operation across operations that are proven to commute when doing
//! > so exposes an exact same-operation or fixed-inverse partner.
//!
//! Examples:
//!
//! ```text
//! X(q0); H(q1); X(q0)
//!     ↓
//! X(q0); X(q0); H(q1)
//! ```
//!
//! followed by the cancellation pass:
//!
//! ```text
//! X(q0); X(q0); H(q1)
//!     ↓
//! H(q1)
//! ```
//!
//! Likewise:
//!
//! ```text
//! S(q0); X(q1); Sdg(q0)
//!     ↓
//! S(q0); Sdg(q0); X(q1)
//! ```
//!
//! followed by inverse cancellation:
//!
//! ```text
//! X(q1)
//! ```
//!
//! # Important architectural rules
//!
//! This module MUST NOT:
//!
//! - define another `QuantumGate`;
//! - define another `QuantumCircuit`;
//! - define another parameter representation;
//! - implement a second commutation algebra;
//! - perform routing;
//! - perform scheduling;
//! - communicate with hardware;
//! - execute a QPU;
//! - perform approximate equivalence;
//! - cross measurement/reset/barrier boundaries;
//! - assume unknown operations commute;
//! - use floating-point tolerances to prove commutation;
//! - introduce `unsafe` code.
//!
//! The canonical representations remain:
//!
//! - `crate::quantum::ir::Gate`;
//! - `crate::quantum::ir::GateKind`;
//! - `crate::quantum::ir::QuantumCircuit`.
//!
//! Pairwise semantic commutation is owned by:
//!
//! `crate::quantum::optimization::analysis::commutation`.
//!
//! This file is therefore a transformation layer, not a second semantic
//! authority.
//!
//! # Why this pass exists separately from commutation analysis
//!
//! `analysis::commutation` answers:
//!
//! ```text
//! "Can A and B be exchanged?"
//! ```
//!
//! This pass answers:
//!
//! ```text
//! "Should I exchange this operation with these commuting operations in order
//!  to expose an exact optimization opportunity?"
//! ```
//!
//! Keeping these responsibilities separate prevents the semantic commutation
//! rules from becoming coupled to a particular optimization strategy.
//!
//! # Semantic safety
//!
//! The underlying commutation analysis is conservative, but generic circuit
//! reordering has one additional requirement:
//!
//! **semantic boundaries are never crossed by this pass.**
//!
//! Therefore the following always stop movement:
//!
//! - measurement;
//! - reset;
//! - barrier.
//!
//! This remains true even when the lower-level commutation analysis reports
//! that two operations have disjoint qubit support.
//!
//! This additional restriction is intentional because measurements can have
//! classical effects and future IR revisions may attach control-flow semantics
//! that cannot be inferred solely from qubit overlap.
//!
//! # Movement strategy
//!
//! The default strategy is `ExposePartners`.
//!
//! For each operation, the pass searches backwards for the nearest earlier
//! operation that is an exact optimization partner:
//!
//! - same gate kind and same ordered qubit support; or
//! - a fixed inverse pair such as `S/Sdg`, `T/Tdg`, `V/Vdg`.
//!
//! The candidate operation is moved immediately after that partner only when
//! it can cross every intervening operation using an exact commutation proof.
//!
//! This gives a monotonic movement direction:
//!
//! ```text
//! later operation → earlier position
//! ```
//!
//! The pass never blindly swaps arbitrary commuting pairs. That is important
//! because arbitrary swapping would allow repeated pipeline invocations to
//! oscillate between equivalent circuit layouts.
//!
//! # Complexity
//!
//! Let:
//!
//! - `N` = number of operations;
//! - `W` = configured maximum partner search distance.
//!
//! The default implementation performs at most `O(N * W)` pair/crossing
//! checks.
//!
//! With `W = usize::MAX`, the algorithm may become `O(N²)` in the worst case.
//! This is intentional and represents the cost of searching arbitrarily far
//! for an optimization partner.
//!
//! Production planners should choose an appropriate `W` for the workload.
//!
//! The implementation:
//!
//! - does not allocate an N×N matrix;
//! - does not construct unitary matrices;
//! - does not perform state-vector simulation;
//! - does not perform equality saturation;
//! - does not recurse with circuit depth;
//! - does not impose an artificial maximum circuit size.
//!
//! Memory is `O(N)` for the temporary gate/source-order workspace.
//!
//! # Atomic mutation
//!
//! The pass never mutates the canonical circuit while discovering a movement
//! plan.
//!
//! Instead:
//!
//! ```text
//! canonical circuit
//!       │
//!       ▼
//! local working sequence
//!       │
//!       ▼
//! source-operation permutation
//!       │
//!       ▼
//! cloned canonical circuit
//!       │
//!       ▼
//! validate
//!       │
//!       ▼
//! commit by replacing the original circuit
//! ```
//!
//! Because the canonical `QuantumCircuit` is `Clone`, transformation failures
//! cannot leave the caller's circuit partially modified.
//!
//! # Integration
//!
//! ## `analysis/commutation.rs`
//!
//! This is the semantic source of truth for exact commutation.
//!
//! This pass calls:
//!
//! `analysis::commutation::relation()`
//!
//! and accepts only:
//!
//! `CommutationKind::Commutes`.
//!
//! It never treats `Unknown`, `DoesNotCommute`, `AntiCommutes`, or
//! `CommutesUpToGlobalPhase` as swappable.
//!
//! ## `operation.rs`
//!
//! Operation semantics are intentionally not duplicated here. The pass only
//! uses canonical `GateKind` plus the central commutation analysis.
//!
//! ## `circuit.rs`
//!
//! The pass uses the canonical circuit's safe `replace()` mutation API after
//! constructing and validating a complete candidate circuit.
//!
//! The optimizer-local transactional editor remains available to future passes
//! that need insert/remove/decompose operations.
//!
//! ## `pass.rs`
//!
//! Implements `OptimizationPass` with:
//!
//! - deterministic execution;
//! - linear/local-window scope;
//! - structural rewrite classification;
//! - commutation capability;
//! - operation reordering capability;
//! - exact semantic preservation.
//!
//! ## `context.rs`
//!
//! The pass periodically checks the invocation context for cancellation and
//! charges analysis/rewrite candidate work. The context remains invocation
//! scoped and owns no circuit.
//!
//! ## `cancellation.rs`
//!
//! This pass intentionally does not delete gates.
//!
//! Its purpose is to expose cancellation opportunities for the cancellation
//! pass that follows it.
//!
//! ## `peephole.rs`
//!
//! The same relationship applies to peephole optimization. Commutation may
//! expose a bounded peephole pattern without owning the pattern rewrite.
//!
//! ## `pipeline.rs`
//!
//! A recommended ordering is:
//!
//! ```text
//! normalize
//!     ↓
//! commutation
//!     ↓
//! cancellation
//!     ↓
//! peephole
//!     ↓
//! commutation
//!     ↓
//! cancellation
//! ```
//!
//! The pipeline owns repetition/fixed-point decisions.
//!
//! This pass performs one deterministic movement phase and does not recursively
//! invoke itself.
//!
//! ## `routing`
//!
//! No routing dependency exists.
//!
//! This pass operates entirely on logical qubits before or between logical
//! optimization stages.
//!
//! ## `scheduling`
//!
//! No scheduling dependency exists.
//!
//! Commutation exposes ordering freedom; scheduling decides actual execution
//! timing later.
//!
//! ## `verification`
//!
//! The pass uses only exact commutation rules. Full semantic verification
//! remains a pipeline-level responsibility.
//!
//! # Rust compatibility
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no external dependencies;
//! - no `unsafe`.
//!
//! # Safety
//!
//! ```text
//! #![forbid(unsafe_code)]
//! ```
//!
//! No unsafe code is required or permitted.

#![forbid(unsafe_code)]

use crate::quantum::ir::gate::{Gate, GateKind};
use crate::quantum::ir::QuantumCircuit;

use super::super::analysis::commutation::{
    relation,
    CommutationKind,
};
use super::super::context::OptimizationContext;
use super::super::errors::{
    OptimizationError,
    OptimizationLocation,
    OptimizationStage,
    PassIdentifier,
};
use super::super::pass::{
    OptimizationPass,
    PassCapability,
    PassChange,
    PassComplexity,
    PassDeterminism,
    PassEffects,
    PassExecutionPolicy,
    PassKind,
    PassMetadata,
    PassMetadataError,
    PassOutcome,
    PassScope,
};

// =============================================================================
// Stable identifiers
// =============================================================================

/// Stable pass identifier.
///
/// This identifier is part of optimizer provenance and should not be renamed
/// casually after publication.
pub const PASS_ID: &str = "quantum.local.commutation";

/// Stable human-readable pass name.
pub const PASS_NAME: &str = "Quantum Commutation Optimization";

/// Algorithm/provenance version.
///
/// This is independent from the Zamani compiler version.
pub const ALGORITHM_VERSION: &str = "1";

// =============================================================================
// Configuration
// =============================================================================

/// Strategy used by the commutation pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CommutationStrategy {
    /// Move later operations toward exact optimization partners.
    ///
    /// This is the production default because it is directly useful to
    /// cancellation and peephole optimization.
    ExposePartners,

    /// Do not perform a general canonical commuting sort.
    ///
    /// This mode exists as an explicit conservative configuration for callers
    /// that want the pass available but disabled without removing it from a
    /// registry/pipeline.
    Disabled,
}

impl Default for CommutationStrategy {
    fn default() -> Self {
        Self::ExposePartners
    }
}

/// Configuration for one commutation optimization invocation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CommutationConfig {
    /// Transformation strategy.
    strategy: CommutationStrategy,

    /// Maximum number of positions an operation may search backwards for an
    /// optimization partner.
    ///
    /// `usize::MAX` means no artificial search-distance ceiling.
    ///
    /// A finite value is recommended by resource-constrained planners.
    max_search_distance: usize,

    /// Maximum number of successful movements in one pass invocation.
    ///
    /// `usize::MAX` means no pass-local movement ceiling.
    ///
    /// This is a deterministic work guard, not a circuit-size limit.
    max_movements: usize,
}

impl Default for CommutationConfig {
    fn default() -> Self {
        Self {
            strategy: CommutationStrategy::ExposePartners,
            max_search_distance: usize::MAX,
            max_movements: usize::MAX,
        }
    }
}

impl CommutationConfig {
    /// Creates the production default configuration.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            strategy: CommutationStrategy::ExposePartners,
            max_search_distance: usize::MAX,
            max_movements: usize::MAX,
        }
    }

    /// Returns a disabled configuration.
    #[must_use]
    pub const fn disabled() -> Self {
        Self {
            strategy: CommutationStrategy::Disabled,
            max_search_distance: 0,
            max_movements: 0,
        }
    }

    /// Sets the transformation strategy.
    #[must_use]
    pub const fn with_strategy(
        mut self,
        strategy: CommutationStrategy,
    ) -> Self {
        self.strategy = strategy;
        self
    }

    /// Sets the maximum backward partner search distance.
    #[must_use]
    pub const fn with_max_search_distance(
        mut self,
        distance: usize,
    ) -> Self {
        self.max_search_distance = distance;
        self
    }

    /// Sets the maximum number of successful movements.
    #[must_use]
    pub const fn with_max_movements(
        mut self,
        movements: usize,
    ) -> Self {
        self.max_movements = movements;
        self
    }

    /// Returns the selected strategy.
    #[must_use]
    pub const fn strategy(self) -> CommutationStrategy {
        self.strategy
    }

    /// Returns the maximum search distance.
    #[must_use]
    pub const fn max_search_distance(self) -> usize {
        self.max_search_distance
    }

    /// Returns the maximum successful movements.
    #[must_use]
    pub const fn max_movements(self) -> usize {
        self.max_movements
    }
}

// =============================================================================
// Statistics
// =============================================================================

/// Statistics generated by one commutation optimization invocation.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CommutationStatistics {
    /// Number of input operations inspected.
    pub operations_inspected: u64,

    /// Number of candidate partner searches.
    pub partner_candidates_examined: u64,

    /// Number of intermediate crossing operations examined.
    pub crossing_candidates_examined: u64,

    /// Number of exact commutation proofs used.
    pub commutation_proofs: u64,

    /// Number of successful operation movements.
    pub operations_moved: u64,

    /// Number of movement opportunities discovered.
    ///
    /// This is currently equal to `operations_moved`, but remains separate so
    /// future implementations can distinguish discovery from application.
    pub opportunities: u64,

    /// Returns whether the pass changed the circuit.
    #[must_use]
    pub const fn changed(self) -> bool {
        self.operations_moved != 0
    }
}

// =============================================================================
// Internal movement representation
// =============================================================================

/// One source operation identity in the temporary permutation.
///
/// This is deliberately just an index into the original circuit. It is not a
/// persisted compiler operation identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SourceOperation {
    original_index: usize,
}

impl SourceOperation {
    #[must_use]
    const fn new(original_index: usize) -> Self {
        Self { original_index }
    }
}

/// Temporary working sequence used during movement discovery.
///
/// The gate and source vectors remain parallel:
///
/// `gates[position]` originated from `sources[position]`.
#[derive(Debug)]
struct WorkingSequence {
    gates: Vec<Gate>,
    sources: Vec<SourceOperation>,
}

impl WorkingSequence {
    fn new(
        operations: &[Gate],
    ) -> Result<Self, OptimizationError> {
        let mut gates = Vec::new();
        let mut sources = Vec::new();

        gates
            .try_reserve(operations.len())
            .map_err(|_| {
                OptimizationError::resource_limit(
                    OptimizationStage::Rewrite,
                    format!(
                        "unable to reserve working storage for {} \
                         quantum operations",
                        operations.len()
                    ),
                )
            })?;

        sources
            .try_reserve(operations.len())
            .map_err(|_| {
                OptimizationError::resource_limit(
                    OptimizationStage::Rewrite,
                    format!(
                        "unable to reserve source-operation storage for {} \
                         quantum operations",
                        operations.len()
                    ),
                )
            })?;

        for (index, gate) in operations.iter().enumerate() {
            gates.push(gate.clone());
            sources.push(SourceOperation::new(index));
        }

        Ok(Self { gates, sources })
    }

    #[must_use]
    fn len(&self) -> usize {
        self.gates.len()
    }

    fn move_operation(
        &mut self,
        from: usize,
        to: usize,
    ) {
        debug_assert!(from > to);
        debug_assert!(from < self.gates.len());
        debug_assert!(to < self.gates.len());

        let gate = self.gates.remove(from);
        let source = self.sources.remove(from);

        self.gates.insert(to, gate);
        self.sources.insert(to, source);
    }
}

// =============================================================================
// Pass
// =============================================================================

/// Production commutation-aware local optimization pass.
#[derive(Debug, Clone)]
pub struct CommutationPass {
    metadata: PassMetadata,
    config: CommutationConfig,
}

impl CommutationPass {
    /// Constructs a production commutation pass with default configuration.
    pub fn new() -> Result<Self, PassMetadataError> {
        Self::with_config(CommutationConfig::default())
    }

    /// Constructs a commutation pass with explicit configuration.
    pub fn with_config(
        config: CommutationConfig,
    ) -> Result<Self, PassMetadataError> {
        let pass_id = PassIdentifier::from_static(PASS_ID)
            .map_err(|error| {
                PassMetadataError::InvalidPassIdentifier {
                    message: error.to_string(),
                }
            })?;

        let metadata = PassMetadata::new(
            pass_id,
            PASS_NAME,
            PassKind::StructuralRewrite,
        )?
        .with_description(
            "Performs exact commutation-aware local gate movement \
             to expose same or inverse optimization partners.",
        )?
        .with_scope(PassScope::LocalWindow)
        .with_complexity(PassComplexity::Quadratic)
        .with_determinism(PassDeterminism::Deterministic)
        .with_capabilities([
            PassCapability::ReordersOperations,
            PassCapability::ChangesDepth,
            PassCapability::ChangesGateCount,
            PassCapability::ChangesTwoQubitCount,
            PassCapability::UsesCommutation,
        ])
        .with_semantic_preservation(true)
        .supports_empty_circuit(true)
        .supports_single_operation(true)
        .supports_large_circuits(true)
        .requires_target(false)
        .requires_verification(false)
        .fixed_point_safe(true);

        metadata.validate()?;

        Ok(Self {
            metadata,
            config,
        })
    }

    /// Returns the stable pass identifier.
    pub const fn pass_id() -> &'static str {
        PASS_ID
    }

    /// Returns the stable human-readable name.
    pub const fn pass_name() -> &'static str {
        PASS_NAME
    }

    /// Returns the algorithm version.
    pub const fn algorithm_version() -> &'static str {
        ALGORITHM_VERSION
    }

    /// Returns the active configuration.
    pub const fn config(&self) -> CommutationConfig {
        self.config
    }

    /// Creates a production pass using default settings.
    pub fn default_pass() -> Result<Self, PassMetadataError> {
        Self::new()
    }

    /// Performs one commutation optimization invocation using a standalone
    /// optimizer context.
    ///
    /// Production pipelines should normally call `OptimizationPass::run`.
    pub fn optimize(
        &self,
        circuit: &mut QuantumCircuit,
    ) -> Result<PassOutcome, OptimizationError> {
        let mut context = OptimizationContext::standalone();

        self.run(circuit, &mut context)
    }

    /// Discovers the final operation permutation without mutating the
    /// canonical circuit.
    fn discover(
        &self,
        circuit: &QuantumCircuit,
        context: &mut OptimizationContext,
    ) -> Result<
        (
            WorkingSequence,
            CommutationStatistics,
        ),
        OptimizationError,
    > {
        let operations = circuit.operations();

        let mut working = WorkingSequence::new(operations)?;

        let mut statistics = CommutationStatistics {
            operations_inspected: checked_u64(
                operations.len(),
                "commutation operation count",
            )?,
            ..CommutationStatistics::default()
        };

        if self.config.strategy == CommutationStrategy::Disabled {
            return Ok((working, statistics));
        }

        let length = working.len();

        for current_index in 1..length {
            context.check_cancelled().map_err(|error| {
                OptimizationError::resource_limit(
                    OptimizationStage::Rewrite,
                    format!(
                        "commutation optimization cancelled: {error}"
                    ),
                )
            })?;

            if statistics.operations_moved
                >= checked_u64(
                    self.config.max_movements,
                    "commutation movement limit",
                )?
            {
                break;
            }

            // A semantic boundary cannot be moved.
            if is_semantic_boundary(
                &working.gates[current_index],
            ) {
                continue;
            }

            let search_distance =
                self.config.max_search_distance.min(current_index);

            if search_distance == 0 {
                continue;
            }

            let search_start =
                current_index.saturating_sub(search_distance);

            let mut crossing_proven = true;

            // Search from the nearest preceding operation backwards.
            //
            // The first valid partner is preferred. This minimizes movement
            // and makes the transformation deterministic.
            for partner_index in
                (search_start..current_index).rev()
            {
                let candidate =
                    &working.gates[current_index];

                let crossing =
                    &working.gates[partner_index];

                // Boundaries stop movement irrespective of qubit overlap.
                if is_semantic_boundary(crossing) {
                    crossing_proven = false;
                    break;
                }

                statistics.partner_candidates_examined =
                    statistics
                        .partner_candidates_examined
                        .saturating_add(1);

                context
                    .charge_one(
                        super::super::limits::OptimizationResource::MatchCandidates,
                    )
                    .map_err(|error| {
                        OptimizationError::resource_limit(
                            OptimizationStage::Rewrite,
                            format!(
                                "commutation partner-candidate budget \
                                 accounting failed: {error}"
                            ),
                        )
                    })?;

                // If this is a valid optimization partner, every operation
                // between it and the candidate must commute with the
                // candidate.
                if is_optimization_partner(
                    crossing,
                    candidate,
                ) {
                    if crossing_proven {
                        working.move_operation(
                            current_index,
                            partner_index + 1,
                        );

                        statistics.operations_moved =
                            statistics.operations_moved.saturating_add(1);

                        statistics.opportunities =
                            statistics.opportunities.saturating_add(1);

                        context
                            .charge_one(
                                super::super::limits::OptimizationResource::Rewrites,
                            )
                            .map_err(|error| {
                                OptimizationError::resource_limit(
                                    OptimizationStage::Rewrite,
                                    format!(
                                        "commutation rewrite budget \
                                         accounting failed: {error}"
                                    ),
                                )
                            })?;

                        break;
                    }

                    // The candidate itself is a partner but the path was
                    // already blocked by a semantic boundary.
                    break;
                }

                // If there is another operation between candidate and this
                // potential partner, it must commute with the candidate.
                //
                // `partner_index` is the current crossing operation, so if it
                // is not the partner it must be traversable.
                let pair_relation =
                    relation(candidate, crossing);

                statistics.crossing_candidates_examined =
                    statistics
                        .crossing_candidates_examined
                        .saturating_add(1);

                context
                    .charge_one(
                        super::super::limits::OptimizationResource::AnalysisSteps,
                    )
                    .map_err(|error| {
                        OptimizationError::resource_limit(
                            OptimizationStage::Analysis,
                            format!(
                                "commutation analysis budget accounting \
                                 failed: {error}"
                            ),
                        )
                    })?;

                if pair_relation.kind()
                    != CommutationKind::Commutes
                {
                    crossing_proven = false;
                    break;
                }

                statistics.commutation_proofs =
                    statistics.commutation_proofs.saturating_add(1);
            }

            // `crossing_proven` is intentionally only used to document the
            // conservative traversal state. Actual movement occurs immediately
            // when a valid partner is discovered.
            let _ = crossing_proven;
        }

        Ok((working, statistics))
    }

    /// Applies a fully discovered permutation atomically.
    ///
    /// Every source operation is replaced exactly once. The operation count,
    /// qubit count, metadata, circuit identity, IR version, and resource policy
    /// therefore remain unchanged.
    fn apply(
        &self,
        circuit: &mut QuantumCircuit,
        working: WorkingSequence,
    ) -> Result<(), OptimizationError> {
        let original = circuit.operations();

        if original.len() != working.len() {
            return Err(
                invariant_error(
                    "commutation working sequence changed operation count",
                ),
            );
        }

        let mut replacements: Vec<Option<Gate>> = Vec::new();

        replacements
            .try_reserve(working.len())
            .map_err(|_| {
                OptimizationError::resource_limit(
                    OptimizationStage::Rewrite,
                    "unable to allocate commutation replacement table",
                )
            })?;

        for _ in 0..working.len() {
            replacements.push(None);
        }

        for position in 0..working.len() {
            let source = working.sources[position].original_index;

            if source >= replacements.len() {
                return Err(
                    invariant_error(
                        "commutation source operation index is invalid",
                    )
                    .with_location(
                        OptimizationLocation::new()
                            .operation(position),
                    ),
                );
            }

            if replacements[source].is_some() {
                return Err(
                    invariant_error(
                        "commutation generated duplicate source operation",
                    )
                    .with_location(
                        OptimizationLocation::new()
                            .operation(position),
                    ),
                );
            }

            replacements[source] =
                Some(working.gates[position].clone());
        }

        let mut candidate = circuit.clone();

        for (index, replacement) in
            replacements.into_iter().enumerate()
        {
            let replacement = replacement.ok_or_else(|| {
                invariant_error(
                    "commutation failed to generate a replacement for \
                     every source operation",
                )
                .with_location(
                    OptimizationLocation::new()
                        .operation(index),
                )
            })?;

            candidate
                .replace(index, replacement)
                .map_err(|error| {
                    OptimizationError::invalid_rewrite(
                        OptimizationStage::Rewrite,
                        format!(
                            "failed to apply commutation replacement at \
                             operation {index}: {error}"
                        ),
                    )
                    .with_location(
                        OptimizationLocation::new()
                            .operation(index),
                    )
                    .with_rule_identifier(
                        "commutation.exchange",
                    )
                })?;
        }

        candidate.validate().map_err(|error| {
            OptimizationError::rewrite_postcondition_failed(
                None,
                format!(
                    "commutation produced invalid canonical Quantum IR: \
                     {error}"
                ),
            )
        })?;

        *circuit = candidate;

        Ok(())
    }
}

// =============================================================================
// OptimizationPass implementation
// =============================================================================

impl OptimizationPass for CommutationPass {
    fn metadata(&self) -> &PassMetadata {
        &self.metadata
    }

    fn run(
        &self,
        circuit: &mut QuantumCircuit,
        context: &mut OptimizationContext,
    ) -> Result<PassOutcome, OptimizationError> {
        circuit.validate().map_err(|error| {
            OptimizationError::invalid_input(
                OptimizationStage::InputValidation,
                format!(
                    "canonical Quantum IR validation failed before \
                     commutation optimization: {error}"
                ),
            )
        })?;

        let operations_before = checked_u64(
            circuit.len(),
            "commutation operation count before optimization",
        )?;

        if circuit.is_empty()
            || self.config.strategy == CommutationStrategy::Disabled
        {
            return Ok(PassOutcome::unchanged(
                operations_before,
                operations_before,
            ));
        }

        let (working, statistics) =
            self.discover(circuit, context)?;

        if !statistics.changed() {
            return Ok(PassOutcome::no_improvement(
                operations_before,
                operations_before,
            ));
        }

        self.apply(circuit, working)?;

        let operations_after = checked_u64(
            circuit.len(),
            "commutation operation count after optimization",
        )?;

        debug_assert_eq!(
            operations_before,
            operations_after
        );

        Ok(
            PassOutcome::changed(
                operations_before,
                operations_after,
            )
            .with_change(PassChange::Changed)
            .with_rewrites(statistics.operations_moved)
            .with_iterations(1),
        )
    }

    fn execution_policy(&self) -> PassExecutionPolicy {
        PassExecutionPolicy::StopWhenStable
    }
}

// =============================================================================
// Semantic helpers
// =============================================================================

/// Returns true when an operation is a hard local movement boundary.
fn is_semantic_boundary(gate: &Gate) -> bool {
    let kind = gate.kind();

    matches!(
        kind,
        GateKind::Measure
            | GateKind::Reset
            | GateKind::Barrier
    )
}

/// Returns whether two operations are a useful exact optimization pair.
///
/// The pair relation is deliberately narrower than commutation itself.
///
/// Two arbitrary commuting operations are not automatically considered
/// partners because blindly sorting commuting gates can destroy locality,
/// increase compiler work, or cause repeated passes to rearrange the circuit
/// unnecessarily.
fn is_optimization_partner(
    first: &Gate,
    second: &Gate,
) -> bool {
    if is_semantic_boundary(first)
        || is_semantic_boundary(second)
    {
        return false;
    }

    // Same exact operation class and support.
///
/// For parameterized rotations, this deliberately means same gate kind and
/// same ordered qubit support. Parameter algebra/fusion remains the
/// responsibility of the parameter/rotation optimization passes.
    if first.kind() == second.kind()
        && first.qubits() == second.qubits()
    {
        return true;
    }

    fixed_inverse_pair(
        first.kind(),
        second.kind(),
    ) && first.qubits() == second.qubits()
}

/// Returns true for fixed inverse gate-kind pairs whose inverse relationship
/// is independent of parameter algebra.
fn fixed_inverse_pair(
    first: GateKind,
    second: GateKind,
) -> bool {
    matches!(
        (first, second),
        (GateKind::S, GateKind::Sdg)
            | (GateKind::Sdg, GateKind::S)
            | (GateKind::T, GateKind::Tdg)
            | (GateKind::Tdg, GateKind::T)
            | (GateKind::V, GateKind::Vdg)
            | (GateKind::Vdg, GateKind::V)
    )
}

// =============================================================================
// Error helpers
// =============================================================================

fn invariant_error(
    message: &'static str,
) -> OptimizationError {
    OptimizationError::invalid_rewrite(
        OptimizationStage::Rewrite,
        message,
    )
    .with_rule_identifier("commutation.invariant")
}

fn checked_u64(
    value: usize,
    what: &'static str,
) -> Result<u64, OptimizationError> {
    u64::try_from(value).map_err(|_| {
        OptimizationError::resource_limit(
            OptimizationStage::Analysis,
            format!(
                "{what} cannot be represented as u64"
            ),
        )
    })
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::quantum::ir::qubit::QubitId;

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

    #[test]
    fn semantic_boundaries_are_never_movement_candidates() {
        let measurement =
            gate(GateKind::Measure, &[0]);

        let x =
            gate(GateKind::X, &[0]);

        assert!(!is_optimization_partner(
            &measurement,
            &x,
        ));
        assert!(!is_optimization_partner(
            &x,
            &measurement,
        ));
    }

    #[test]
    fn same_gate_and_support_are_partners() {
        let first =
            gate(GateKind::X, &[0]);

        let second =
            gate(GateKind::X, &[0]);

        assert!(is_optimization_partner(
            &first,
            &second,
        ));
    }

    #[test]
    fn different_support_is_not_a_partner() {
        let first =
            gate(GateKind::X, &[0]);

        let second =
            gate(GateKind::X, &[1]);

        assert!(!is_optimization_partner(
            &first,
            &second,
        ));
    }

    #[test]
    fn fixed_inverse_pairs_are_partners() {
        let first =
            gate(GateKind::T, &[0]);

        let second =
            gate(GateKind::Tdg, &[0]);

        assert!(is_optimization_partner(
            &first,
            &second,
        ));
    }

    #[test]
    fn unrelated_gate_kinds_are_not_partners() {
        let first =
            gate(GateKind::H, &[0]);

        let second =
            gate(GateKind::X, &[0]);

        assert!(!is_optimization_partner(
            &first,
            &second,
        ));
    }

    #[test]
    fn disjoint_gates_are_proven_to_commute() {
        let first =
            gate(GateKind::X, &[0]);

        let second =
            gate(GateKind::H, &[1]);

        assert_eq!(
            relation(&first, &second).kind(),
            CommutationKind::Commutes
        );
    }

    #[test]
    fn distinct_paulis_do_not_authorize_a_swap() {
        let first =
            gate(GateKind::X, &[0]);

        let second =
            gate(GateKind::Z, &[0]);

        assert_ne!(
            relation(&first, &second).kind(),
            CommutationKind::Commutes
        );
    }

    #[test]
    fn fixed_inverse_pair_requires_same_support() {
        let first =
            gate(GateKind::S, &[0]);

        let second =
            gate(GateKind::Sdg, &[1]);

        assert!(!is_optimization_partner(
            &first,
            &second,
        ));
    }

    #[test]
    fn config_defaults_to_partner_exposure() {
        let config =
            CommutationConfig::default();

        assert_eq!(
            config.strategy(),
            CommutationStrategy::ExposePartners
        );

        assert_eq!(
            config.max_search_distance(),
            usize::MAX
        );

        assert_eq!(
            config.max_movements(),
            usize::MAX
        );
    }

    #[test]
    fn disabled_config_has_no_work_budget() {
        let config =
            CommutationConfig::disabled();

        assert_eq!(
            config.strategy(),
            CommutationStrategy::Disabled
        );

        assert_eq!(
            config.max_search_distance(),
            0
        );

        assert_eq!(
            config.max_movements(),
            0
        );
    }
}