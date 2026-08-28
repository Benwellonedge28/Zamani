//! Zamani Quantum Routing — Routing Results and Metrics
//!
//! Production-grade immutable result model for the quantum routing subsystem.
//!
//! # Responsibility
//!
//! `result.rs` owns the successful output contract of a routing invocation.
//!
//! It records:
//!
//! - the routing disposition;
//! - the selected algorithm;
//! - the selected layout strategy;
//! - the optimization objective;
//! - the verification level and verification outcome;
//! - the initial logical-to-physical mapping;
//! - the final logical-to-physical mapping;
//! - the semantic routing operation stream;
//! - routing metrics;
//! - movement counts;
//! - depth information;
//! - duration information;
//! - candidate/search statistics;
//! - hardware-quality estimates when available;
//! - reproducibility metadata;
//! - routing events/diagnostics;
//! - final route quality information;
//! - stable summary information for compiler, hardware, benchmarking,
//!   diagnostics, and testing consumers.
//!
//! # Architectural boundary
//!
//! This file does NOT:
//!
//! - perform routing;
//! - select algorithms;
//! - calculate routing costs;
//! - validate topology;
//! - mutate mappings;
//! - execute quantum circuits;
//! - synthesize gates;
//! - schedule operations;
//! - communicate with hardware;
//! - parse OpenQASM;
//! - perform quantum simulation;
//! - perform QEC decoding.
//!
//! Those responsibilities belong to their respective subsystems.
//!
//! # Dependency direction
//!
//! ```text
//! types.rs --------┐
//!                  │
//! mapping.rs ------┤
//!                  │
//! config.rs -------┤
//!                  ▼
//!               result.rs
//!                  ▲
//!                  │
//!       ┌──────────┼──────────┐
//!       │          │          │
//!     router    verifier   transpiler
//!       │          │          │
//!       └──────────┼──────────┘
//!                  │
//!          benchmarking/hardware
//! ```
//!
//! `result.rs` therefore depends only on stable routing contracts and the
//! mapping/configuration value objects. It does not depend on later algorithm
//! implementations.
//!
//! # Important distinction
//!
//! `errors.rs` owns the canonical:
//!
//! ```text
//! Result<T, RoutingError>
//! ```
//!
//! This file owns the successful value:
//!
//! ```text
//! RoutingResult
//! ```
//!
//! The two intentionally have different responsibilities.
//!
//! # Reproducibility
//!
//! Routing results must be reproducible without requiring wall-clock timestamps
//! or other nondeterministic state. Consequently this module records:
//!
//! - routing ID;
//! - seed;
//! - algorithm;
//! - algorithm version;
//! - configuration hash;
//! - topology hash;
//! - circuit/input hash;
//! - result hash;
//! - deterministic flag;
//! - trial number;
//! - total trials.
//!
//! The hashes are supplied by the caller/router. This file does not prescribe a
//! hashing algorithm and therefore remains independent of cryptographic/hash
//! implementation choices.
//!
//! # Safety
//!
//! - No `unsafe` code.
//! - No global mutable state.
//! - No filesystem access.
//! - No network access.
//! - No hardware access.
//! - No environment-dependent behavior.
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
//! No external dependencies are required.

// =============================================================================
// Imports
// =============================================================================

use crate::quantum::routing::config::{
    LayoutStrategy,
    RoutingAlgorithm,
    RoutingObjective,
    RoutingMode,
    VerificationLevel,
};

use crate::quantum::routing::mapping::{
    QubitMapping,
    QubitMappingSnapshot,
};

use crate::quantum::routing::types::{
    GateIdentity,
    LogicalQubitId,
    PhysicalQubitId,
    RouteDisposition,
    RoutingEvent,
    RoutingId,
    RoutingMove,
    RoutingOperation,
    RoutingSeed,
};

use std::fmt;
use std::time::Duration;

// =============================================================================
// Result status
// =============================================================================

/// Verification status stored in a successful routing result.
///
/// A routing result can exist without a verification report when verification
/// was explicitly disabled. This enum records the state explicitly rather than
/// forcing consumers to infer it from an `Option`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VerificationStatus {
    /// Verification was not requested.
    NotRequested,

    /// Verification was requested and succeeded.
    Passed,

    /// Verification was requested but did not complete.
    ///
    /// A router should normally return an error instead of producing this state
    /// for a production route. It exists to represent externally assembled
    /// result records and future incremental verification pipelines.
    NotCompleted,
}

impl VerificationStatus {
    /// Returns whether verification passed.
    #[must_use]
    pub const fn passed(self) -> bool {
        matches!(self, Self::Passed)
    }

    /// Returns whether verification was explicitly skipped.
    #[must_use]
    pub const fn not_requested(self) -> bool {
        matches!(self, Self::NotRequested)
    }

    /// Returns a stable machine-readable name.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::NotRequested => "not_requested",
            Self::Passed => "passed",
            Self::NotCompleted => "not_completed",
        }
    }
}

impl fmt::Display for VerificationStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.name())
    }
}

// =============================================================================
// Verification summary
// =============================================================================

/// Compact verification summary attached to a routing result.
///
/// The complete verifier implementation belongs to `verification.rs`.
/// `result.rs` stores only the stable result-facing summary.
///
/// This separation prevents the result type from depending on the verifier's
/// implementation details.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerificationSummary {
    /// Verification level requested.
    pub level: VerificationLevel,

    /// Final verification state.
    pub status: VerificationStatus,

    /// Number of structural invariants checked.
    pub structural_checks: usize,

    /// Number of mapping invariants checked.
    pub mapping_checks: usize,

    /// Number of physical-executability checks performed.
    pub executability_checks: usize,

    /// Number of preservation checks performed.
    pub preservation_checks: usize,

    /// Number of successful checks.
    pub passed_checks: usize,

    /// Optional verifier implementation/version identifier.
    pub verifier_version: Option<String>,
}

impl VerificationSummary {
    /// Creates a summary for a verification pass.
    #[must_use]
    pub fn passed(level: VerificationLevel) -> Self {
        Self {
            level,
            status: VerificationStatus::Passed,
            structural_checks: 0,
            mapping_checks: 0,
            executability_checks: 0,
            preservation_checks: 0,
            passed_checks: 0,
            verifier_version: None,
        }
    }

    /// Creates a summary indicating that verification was not requested.
    #[must_use]
    pub fn not_requested() -> Self {
        Self {
            level: VerificationLevel::None,
            status: VerificationStatus::NotRequested,
            structural_checks: 0,
            mapping_checks: 0,
            executability_checks: 0,
            preservation_checks: 0,
            passed_checks: 0,
            verifier_version: None,
        }
    }

    /// Sets the verifier version.
    #[must_use]
    pub fn with_verifier_version(
        mut self,
        version: impl Into<String>,
    ) -> Self {
        self.verifier_version = Some(version.into());
        self
    }

    /// Records structural checks.
    #[must_use]
    pub const fn with_structural_checks(
        mut self,
        checks: usize,
    ) -> Self {
        self.structural_checks = checks;
        self
    }

    /// Records mapping checks.
    #[must_use]
    pub const fn with_mapping_checks(
        mut self,
        checks: usize,
    ) -> Self {
        self.mapping_checks = checks;
        self
    }

    /// Records executability checks.
    #[must_use]
    pub const fn with_executability_checks(
        mut self,
        checks: usize,
    ) -> Self {
        self.executability_checks = checks;
        self
    }

    /// Records preservation checks.
    #[must_use]
    pub const fn with_preservation_checks(
        mut self,
        checks: usize,
    ) -> Self {
        self.preservation_checks = checks;
        self
    }

    /// Records the total number of passed checks.
    #[must_use]
    pub const fn with_passed_checks(
        mut self,
        checks: usize,
    ) -> Self {
        self.passed_checks = checks;
        self
    }

    /// Returns the total number of checks represented by this summary.
    #[must_use]
    pub const fn total_checks(&self) -> usize {
        self.structural_checks
            + self.mapping_checks
            + self.executability_checks
            + self.preservation_checks
    }

    /// Returns whether every represented check passed.
    #[must_use]
    pub const fn all_checks_passed(&self) -> bool {
        self.status.passed()
            && self.passed_checks == self.total_checks()
    }
}

impl Default for VerificationSummary {
    fn default() -> Self {
        Self::not_requested()
    }
}

// =============================================================================
// Routing metrics
// =============================================================================

/// Complete quantitative metrics for one routing invocation.
///
/// The metrics intentionally distinguish:
///
/// - original circuit work;
/// - routing-added work;
/// - final circuit work;
/// - search effort;
/// - hardware-quality estimates.
///
/// This prevents consumers from treating `swap_count` as the complete measure
/// of routing quality.
///
/// All integer counters are monotonically non-negative.
///
/// Floating-point values are optional because hardware calibration data may not
/// be available. When present, they must be finite. Producers are responsible
/// for validating this before constructing the final result.
#[derive(Debug, Clone, PartialEq)]
pub struct RoutingMetrics {
    // -------------------------------------------------------------------------
    // Circuit size
    // -------------------------------------------------------------------------

    /// Number of logical qubits in the routed workload.
    pub logical_qubits: usize,

    /// Number of physical qubits available on the target topology.
    pub physical_qubits: usize,

    /// Number of operations in the original workload.
    pub original_operations: usize,

    /// Number of operations in the final routed operation stream.
    pub final_operations: usize,

    /// Number of original single-qubit operations.
    pub original_single_qubit_operations: usize,

    /// Number of original two-qubit operations.
    pub original_two_qubit_operations: usize,

    /// Number of original multi-qubit operations.
    pub original_multi_qubit_operations: usize,

    /// Number of final gate operations.
    pub final_gate_operations: usize,

    // -------------------------------------------------------------------------
    // Routing overhead
    // -------------------------------------------------------------------------

    /// Number of routed two-qubit gates from the original workload.
    pub routed_two_qubit_operations: usize,

    /// Number of inserted SWAP moves.
    pub inserted_swaps: usize,

    /// Number of inserted bridge moves.
    pub inserted_bridges: usize,

    /// Number of inserted arbitrary permutation moves.
    pub inserted_permutations: usize,

    /// Total number of inserted movement operations.
    pub inserted_moves: usize,

    /// Total routing overhead measured in operation count.
    ///
    /// This is normally equal to `inserted_moves`, but remains explicit because
    /// future routing implementations may represent semantic movement without
    /// a one-to-one physical operation count.
    pub routing_overhead_operations: usize,

    // -------------------------------------------------------------------------
    // Depth
    // -------------------------------------------------------------------------

    /// Estimated/original circuit depth.
    pub original_depth: usize,

    /// Estimated/final routed circuit depth.
    pub final_depth: usize,

    /// Additional depth caused by routing.
    pub routing_depth: usize,

    /// Number of two-qubit layers in the original circuit.
    pub original_two_qubit_depth: usize,

    /// Number of two-qubit layers in the routed circuit.
    pub final_two_qubit_depth: usize,

    /// Additional two-qubit depth caused by routing.
    pub routing_two_qubit_depth: usize,

    // -------------------------------------------------------------------------
    // Search effort
    // -------------------------------------------------------------------------

    /// Number of routing decisions/iterations performed.
    pub routing_iterations: usize,

    /// Number of candidate moves evaluated.
    pub candidate_evaluations: usize,

    /// Number of candidate moves rejected.
    pub candidate_rejections: usize,

    /// Number of algorithm trials performed.
    pub trials: usize,

    /// Number of layout trials performed.
    pub layout_trials: usize,

    /// Number of routing trials performed.
    pub routing_trials: usize,

    // -------------------------------------------------------------------------
    // Timing
    // -------------------------------------------------------------------------

    /// Total routing wall-clock duration.
    pub total_duration: Duration,

    /// Time spent selecting the initial layout.
    pub layout_duration: Duration,

    /// Time spent in routing search.
    pub routing_duration: Duration,

    /// Time spent verifying the result.
    pub verification_duration: Duration,

    // -------------------------------------------------------------------------
    // Hardware quality
    // -------------------------------------------------------------------------

    /// Estimated total physical execution duration.
    ///
    /// This is distinct from `total_duration`, which is compiler runtime.
    pub estimated_execution_duration: Option<Duration>,

    /// Estimated physical error probability/cost.
    pub estimated_error: Option<f64>,

    /// Estimated circuit fidelity.
    pub estimated_fidelity: Option<f64>,

    /// Objective value returned by the selected cost model.
    pub objective_value: Option<f64>,

    /// Number of physical two-qubit interactions used in the final route.
    pub physical_two_qubit_operations: usize,
}

impl RoutingMetrics {
    /// Creates zeroed metrics for a workload.
    #[must_use]
    pub const fn new(
        logical_qubits: usize,
        physical_qubits: usize,
    ) -> Self {
        Self {
            logical_qubits,
            physical_qubits,
            original_operations: 0,
            final_operations: 0,
            original_single_qubit_operations: 0,
            original_two_qubit_operations: 0,
            original_multi_qubit_operations: 0,
            final_gate_operations: 0,
            routed_two_qubit_operations: 0,
            inserted_swaps: 0,
            inserted_bridges: 0,
            inserted_permutations: 0,
            inserted_moves: 0,
            routing_overhead_operations: 0,
            original_depth: 0,
            final_depth: 0,
            routing_depth: 0,
            original_two_qubit_depth: 0,
            final_two_qubit_depth: 0,
            routing_two_qubit_depth: 0,
            routing_iterations: 0,
            candidate_evaluations: 0,
            candidate_rejections: 0,
            trials: 1,
            layout_trials: 1,
            routing_trials: 1,
            total_duration: Duration::ZERO,
            layout_duration: Duration::ZERO,
            routing_duration: Duration::ZERO,
            verification_duration: Duration::ZERO,
            estimated_execution_duration: None,
            estimated_error: None,
            estimated_fidelity: None,
            objective_value: None,
            physical_two_qubit_operations: 0,
        }
    }

    /// Creates metrics with all counters initialized to zero.
    #[must_use]
    pub const fn empty() -> Self {
        Self::new(0, 0)
    }

    /// Returns the number of operations added by routing.
    #[must_use]
    pub const fn operation_overhead(&self) -> usize {
        self.routing_overhead_operations
    }

    /// Returns the multiplicative operation overhead.
    ///
    /// Returns `None` when the original operation count is zero.
    #[must_use]
    pub fn operation_overhead_ratio(&self) -> Option<f64> {
        if self.original_operations == 0 {
            return None;
        }

        Some(
            self.final_operations as f64
                / self.original_operations as f64,
        )
    }

    /// Returns the fraction of final operations introduced by routing.
    #[must_use]
    pub fn inserted_operation_fraction(&self) -> Option<f64> {
        if self.final_operations == 0 {
            return None;
        }

        Some(
            self.inserted_moves as f64
                / self.final_operations as f64,
        )
    }

    /// Returns the depth overhead.
    #[must_use]
    pub const fn depth_overhead(&self) -> usize {
        self.routing_depth
    }

    /// Returns the two-qubit-depth overhead.
    #[must_use]
    pub const fn two_qubit_depth_overhead(&self) -> usize {
        self.routing_two_qubit_depth
    }

    /// Returns whether hardware-quality information is available.
    #[must_use]
    pub const fn has_hardware_quality_metrics(&self) -> bool {
        self.estimated_error.is_some()
            || self.estimated_fidelity.is_some()
            || self.estimated_execution_duration.is_some()
    }

    /// Returns whether all present floating-point metrics are finite.
    #[must_use]
    pub fn floating_point_values_are_finite(&self) -> bool {
        self.estimated_error
            .map(|value| value.is_finite())
            .unwrap_or(true)
            && self
                .estimated_fidelity
                .map(|value| value.is_finite())
                .unwrap_or(true)
            && self
                .objective_value
                .map(|value| value.is_finite())
                .unwrap_or(true)
    }

    /// Returns the total search work represented by this result.
    #[must_use]
    pub const fn search_work(&self) -> usize {
        self.routing_iterations + self.candidate_evaluations
    }

    /// Returns the number of routing operations per original operation.
    #[must_use]
    pub fn inserted_moves_per_original_operation(
        &self,
    ) -> Option<f64> {
        if self.original_operations == 0 {
            return None;
        }

        Some(
            self.inserted_moves as f64
                / self.original_operations as f64,
        )
    }

    /// Returns whether the final route has no routing overhead.
    #[must_use]
    pub const fn has_zero_overhead(&self) -> bool {
        self.inserted_moves == 0
            && self.routing_overhead_operations == 0
            && self.routing_depth == 0
    }
}

impl Default for RoutingMetrics {
    fn default() -> Self {
        Self::empty()
    }
}

// =============================================================================
// Reproducibility metadata
// =============================================================================

/// Reproducibility metadata attached to a routing result.
///
/// The router supplies hashes rather than this type calculating them. This
/// keeps result construction deterministic and independent of a particular hash
/// implementation.
///
/// The hashes should represent canonicalized inputs/configuration rather than
/// arbitrary debug strings.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ReproducibilityMetadata {
    /// Unique routing invocation identifier.
    pub routing_id: Option<RoutingId>,

    /// Optional algorithm seed.
    pub seed: Option<RoutingSeed>,

    /// Whether the route was requested to be deterministic.
    pub deterministic: bool,

    /// Stable Zamani routing API/version identifier.
    pub routing_version: Option<String>,

    /// Algorithm implementation/version identifier.
    pub algorithm_version: Option<String>,

    /// Hash/fingerprint of the routing configuration.
    pub configuration_hash: Option<String>,

    /// Hash/fingerprint of the input workload/circuit.
    pub input_hash: Option<String>,

    /// Hash/fingerprint of the physical topology.
    pub topology_hash: Option<String>,

    /// Hash/fingerprint of the resulting routing operation stream.
    pub result_hash: Option<String>,

    /// Trial number when this result belongs to a multi-trial search.
    pub trial_index: Option<usize>,

    /// Total number of trials considered.
    pub total_trials: Option<usize>,
}

impl ReproducibilityMetadata {
    /// Creates metadata for a deterministic route.
    #[must_use]
    pub const fn deterministic() -> Self {
        Self {
            routing_id: None,
            seed: None,
            deterministic: true,
            routing_version: None,
            algorithm_version: None,
            configuration_hash: None,
            input_hash: None,
            topology_hash: None,
            result_hash: None,
            trial_index: None,
            total_trials: None,
        }
    }

    /// Creates metadata for a potentially stochastic route.
    #[must_use]
    pub const fn nondeterministic() -> Self {
        Self {
            deterministic: false,
            ..Self::deterministic()
        }
    }

    /// Sets the routing ID.
    #[must_use]
    pub const fn with_routing_id(
        mut self,
        id: RoutingId,
    ) -> Self {
        self.routing_id = Some(id);
        self
    }

    /// Sets the seed.
    #[must_use]
    pub const fn with_seed(
        mut self,
        seed: RoutingSeed,
    ) -> Self {
        self.seed = Some(seed);
        self
    }

    /// Sets the routing version.
    #[must_use]
    pub fn with_routing_version(
        mut self,
        version: impl Into<String>,
    ) -> Self {
        self.routing_version = Some(version.into());
        self
    }

    /// Sets the algorithm version.
    #[must_use]
    pub fn with_algorithm_version(
        mut self,
        version: impl Into<String>,
    ) -> Self {
        self.algorithm_version = Some(version.into());
        self
    }

    /// Sets the configuration hash.
    #[must_use]
    pub fn with_configuration_hash(
        mut self,
        hash: impl Into<String>,
    ) -> Self {
        self.configuration_hash = Some(hash.into());
        self
    }

    /// Sets the input hash.
    #[must_use]
    pub fn with_input_hash(
        mut self,
        hash: impl Into<String>,
    ) -> Self {
        self.input_hash = Some(hash.into());
        self
    }

    /// Sets the topology hash.
    #[must_use]
    pub fn with_topology_hash(
        mut self,
        hash: impl Into<String>,
    ) -> Self {
        self.topology_hash = Some(hash.into());
        self
    }

    /// Sets the result hash.
    #[must_use]
    pub fn with_result_hash(
        mut self,
        hash: impl Into<String>,
    ) -> Self {
        self.result_hash = Some(hash.into());
        self
    }

    /// Sets the trial information.
    #[must_use]
    pub const fn with_trial(
        mut self,
        index: usize,
        total: usize,
    ) -> Self {
        self.trial_index = Some(index);
        self.total_trials = Some(total);
        self
    }

    /// Returns whether enough metadata is present for deterministic replay
    /// identification.
    ///
    /// This does not mean replay is possible by itself; the actual circuit,
    /// topology, and configuration must also still be available.
    #[must_use]
    pub fn is_replay_identifiable(&self) -> bool {
        self.routing_version.is_some()
            && self.algorithm_version.is_some()
            && self.configuration_hash.is_some()
            && self.input_hash.is_some()
            && self.topology_hash.is_some()
    }
}

// =============================================================================
// Layout summary
// =============================================================================

/// Summary of the initial and final physical placement.
///
/// This deliberately stores immutable snapshots rather than mutable mapping
/// references so a routing result remains self-contained after the router's
/// working mapping is discarded.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LayoutSummary {
    /// Mapping before routing movements.
    pub initial_mapping: QubitMappingSnapshot,

    /// Mapping after routing movements.
    pub final_mapping: QubitMappingSnapshot,

    /// Whether the final mapping differs from the initial mapping.
    pub changed: bool,

    /// Number of logical qubits assigned in the initial mapping.
    pub initial_mapped_qubits: usize,

    /// Number of logical qubits assigned in the final mapping.
    pub final_mapped_qubits: usize,
}

impl LayoutSummary {
    /// Constructs a layout summary from two snapshots.
    #[must_use]
    pub fn new(
        initial_mapping: QubitMappingSnapshot,
        final_mapping: QubitMappingSnapshot,
    ) -> Self {
        let initial_mapped_qubits = initial_mapping.len();
        let final_mapped_qubits = final_mapping.len();
        let changed = initial_mapping != final_mapping;

        Self {
            initial_mapping,
            final_mapping,
            changed,
            initial_mapped_qubits,
            final_mapped_qubits,
        }
    }

    /// Returns whether the mapping was unchanged.
    #[must_use]
    pub const fn is_unchanged(&self) -> bool {
        !self.changed
    }
}

// =============================================================================
// Route quality
// =============================================================================

/// Summary of the quality of a completed route.
///
/// This is deliberately independent of a particular cost model implementation.
#[derive(Debug, Clone, PartialEq)]
pub struct RouteQuality {
    /// Primary objective used by the router.
    pub objective: RoutingObjective,

    /// Objective value, if the cost model produced one.
    pub objective_value: Option<f64>,

    /// Number of inserted SWAPs.
    pub swap_count: usize,

    /// Number of inserted movement operations.
    pub movement_count: usize,

    /// Final circuit depth.
    pub final_depth: usize,

    /// Estimated error, if available.
    pub estimated_error: Option<f64>,

    /// Estimated fidelity, if available.
    pub estimated_fidelity: Option<f64>,

    /// Whether this result is considered comparable by the objective model.
    pub comparable: bool,
}

impl RouteQuality {
    /// Constructs route quality from metrics.
    #[must_use]
    pub fn from_metrics(
        objective: RoutingObjective,
        metrics: &RoutingMetrics,
    ) -> Self {
        let comparable = metrics
            .objective_value
            .map(|value| value.is_finite())
            .unwrap_or(false);

        Self {
            objective,
            objective_value: metrics.objective_value,
            swap_count: metrics.inserted_swaps,
            movement_count: metrics.inserted_moves,
            final_depth: metrics.final_depth,
            estimated_error: metrics.estimated_error,
            estimated_fidelity: metrics.estimated_fidelity,
            comparable,
        }
    }

    /// Returns whether this route is strictly better by raw SWAP count than
    /// another route.
    ///
    /// This helper is intentionally narrow and does not pretend to compare
    /// arbitrary objectives.
    #[must_use]
    pub fn fewer_swaps_than(&self, other: &Self) -> bool {
        self.swap_count < other.swap_count
    }
}

// =============================================================================
// Routing result
// =============================================================================

/// Complete successful result of one quantum routing invocation.
///
/// This is the primary output contract consumed by:
///
/// - `router.rs`;
/// - `transpiler.rs`;
/// - `verification.rs`;
/// - quantum hardware integration;
/// - benchmarking;
/// - optimization;
/// - diagnostics;
/// - compiler tooling;
/// - tests;
/// - reproducibility tooling.
///
/// The result is self-contained. It owns the operation stream and immutable
/// mapping snapshots, so it remains valid after the mutable router state has
/// been discarded.
///
/// # Transactional guarantee
///
/// A production router should construct this value only after all required
/// routing work has completed successfully. If routing fails, `errors.rs` is
/// used instead and no successful `RoutingResult` should be returned.
#[derive(Debug, Clone, PartialEq)]
pub struct RoutingResult {
    // -------------------------------------------------------------------------
    // Disposition
    // -------------------------------------------------------------------------

    /// Final routing disposition.
    pub disposition: RouteDisposition,

    /// Algorithm selected for the invocation.
    pub algorithm: RoutingAlgorithm,

    /// Initial layout strategy.
    pub layout_strategy: LayoutStrategy,

    /// Optimization objective.
    pub objective: RoutingObjective,

    /// Overall routing mode.
    pub mode: RoutingMode,

    // -------------------------------------------------------------------------
    // Layout
    // -------------------------------------------------------------------------

    /// Complete initial/final mapping summary.
    pub layout: LayoutSummary,

    // -------------------------------------------------------------------------
    // Operations
    // -------------------------------------------------------------------------

    /// Complete semantic routing operation stream.
    ///
    /// This stream may contain:
    ///
    /// - movement operations;
    /// - executable gates;
    /// - barriers.
    ///
    /// It is not a hardware-native gate stream. Hardware lowering remains a
    /// later stage.
    pub operations: Vec<RoutingOperation>,

    // -------------------------------------------------------------------------
    // Metrics and quality
    // -------------------------------------------------------------------------

    /// Quantitative routing metrics.
    pub metrics: RoutingMetrics,

    /// Objective-independent quality summary.
    pub quality: RouteQuality,

    // -------------------------------------------------------------------------
    // Verification
    // -------------------------------------------------------------------------

    /// Verification summary.
    pub verification: VerificationSummary,

    // -------------------------------------------------------------------------
    // Reproducibility
    // -------------------------------------------------------------------------

    /// Reproducibility information.
    pub reproducibility: ReproducibilityMetadata,

    // -------------------------------------------------------------------------
    // Diagnostics/events
    // -------------------------------------------------------------------------

    /// Deterministic routing events.
    ///
    /// Events are optional because high-performance production paths may
    /// intentionally disable event collection.
    pub events: Vec<RoutingEvent>,
}

impl RoutingResult {
    // =========================================================================
    // Construction
    // =========================================================================

    /// Creates a routing result from its immutable components.
    ///
    /// This constructor intentionally performs no routing and no hardware
    /// validation. Those responsibilities belong to the router/verifier.
    #[must_use]
    pub fn new(
        disposition: RouteDisposition,
        algorithm: RoutingAlgorithm,
        layout_strategy: LayoutStrategy,
        objective: RoutingObjective,
        mode: RoutingMode,
        initial_mapping: QubitMappingSnapshot,
        final_mapping: QubitMappingSnapshot,
        operations: Vec<RoutingOperation>,
        metrics: RoutingMetrics,
        verification: VerificationSummary,
        reproducibility: ReproducibilityMetadata,
    ) -> Self {
        let layout =
            LayoutSummary::new(initial_mapping, final_mapping);

        let quality =
            RouteQuality::from_metrics(objective.clone(), &metrics);

        Self {
            disposition,
            algorithm,
            layout_strategy,
            objective,
            mode,
            layout,
            operations,
            metrics,
            quality,
            verification,
            reproducibility,
            events: Vec::new(),
        }
    }

    /// Creates a result for an already executable circuit.
    #[must_use]
    pub fn already_executable(
        algorithm: RoutingAlgorithm,
        layout_strategy: LayoutStrategy,
        objective: RoutingObjective,
        mode: RoutingMode,
        mapping: QubitMappingSnapshot,
        operations: Vec<RoutingOperation>,
        metrics: RoutingMetrics,
        verification: VerificationSummary,
        reproducibility: ReproducibilityMetadata,
    ) -> Self {
        Self::new(
            RouteDisposition::AlreadyExecutable,
            algorithm,
            layout_strategy,
            objective,
            mode,
            mapping.clone(),
            mapping,
            operations,
            metrics,
            verification,
            reproducibility,
        )
    }

    /// Creates a result for a route that inserted routing movement.
    #[must_use]
    pub fn routed(
        algorithm: RoutingAlgorithm,
        layout_strategy: LayoutStrategy,
        objective: RoutingObjective,
        mode: RoutingMode,
        initial_mapping: QubitMappingSnapshot,
        final_mapping: QubitMappingSnapshot,
        operations: Vec<RoutingOperation>,
        metrics: RoutingMetrics,
        verification: VerificationSummary,
        reproducibility: ReproducibilityMetadata,
    ) -> Self {
        Self::new(
            RouteDisposition::Routed,
            algorithm,
            layout_strategy,
            objective,
            mode,
            initial_mapping,
            final_mapping,
            operations,
            metrics,
            verification,
            reproducibility,
        )
    }

    /// Creates a result representing a deliberately non-routed request.
    #[must_use]
    pub fn not_requested(
        algorithm: RoutingAlgorithm,
        layout_strategy: LayoutStrategy,
        objective: RoutingObjective,
        mode: RoutingMode,
        mapping: QubitMappingSnapshot,
        operations: Vec<RoutingOperation>,
        metrics: RoutingMetrics,
        verification: VerificationSummary,
        reproducibility: ReproducibilityMetadata,
    ) -> Self {
        Self::new(
            RouteDisposition::NotRequested,
            algorithm,
            layout_strategy,
            objective,
            mode,
            mapping.clone(),
            mapping,
            operations,
            metrics,
            verification,
            reproducibility,
        )
    }

    // =========================================================================
    // Event management
    // =========================================================================

    /// Adds a deterministic event to the result.
    ///
    /// This method is primarily useful while assembling a result in the router.
    pub fn push_event(&mut self, event: RoutingEvent) {
        self.events.push(event);
    }

    /// Adds multiple events while preserving their supplied order.
    pub fn extend_events<I>(&mut self, events: I)
    where
        I: IntoIterator<Item = RoutingEvent>,
    {
        self.events.extend(events);
    }

    /// Returns the number of recorded events.
    #[must_use]
    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    // =========================================================================
    // Mapping access
    // =========================================================================

    /// Returns the initial physical location of a logical qubit.
    #[must_use]
    pub fn initial_physical_of(
        &self,
        logical: LogicalQubitId,
    ) -> Option<PhysicalQubitId> {
        self.layout.initial_mapping.physical_of(logical)
    }

    /// Returns the final physical location of a logical qubit.
    #[must_use]
    pub fn final_physical_of(
        &self,
        logical: LogicalQubitId,
    ) -> Option<PhysicalQubitId> {
        self.layout.final_mapping.physical_of(logical)
    }

    /// Returns the logical qubit occupying a physical location at the end of
    /// routing.
    #[must_use]
    pub fn final_logical_at(
        &self,
        physical: PhysicalQubitId,
    ) -> Option<LogicalQubitId> {
        self.layout.final_mapping.logical_at(physical)
    }

    /// Returns the routing permutation as `(logical, initial, final)` tuples.
    ///
    /// Only logical qubits whose final location differs from their initial
    /// location are returned.
    #[must_use]
    pub fn routing_permutation(
        &self,
    ) -> Vec<(
        LogicalQubitId,
        PhysicalQubitId,
        PhysicalQubitId,
    )> {
        let mut permutation = Vec::new();

        for (logical, initial) in
            self.layout.initial_mapping.logical_to_physical()
        {
            if let Some(final_position) =
                self.layout.final_mapping.physical_of(logical)
            {
                if initial != final_position {
                    permutation.push((
                        logical,
                        initial,
                        final_position,
                    ));
                }
            }
        }

        permutation
    }

    /// Returns whether routing changed any logical qubit's physical position.
    #[must_use]
    pub const fn changed_layout(&self) -> bool {
        self.layout.changed
    }

    // =========================================================================
    // Operation access
    // =========================================================================

    /// Returns the number of operations in the final route.
    #[must_use]
    pub fn operation_count(&self) -> usize {
        self.operations.len()
    }

    /// Returns the number of movement operations.
    #[must_use]
    pub fn movement_count(&self) -> usize {
        self.operations
            .iter()
            .filter(|operation| operation.is_move())
            .count()
    }

    /// Returns the number of gate operations.
    #[must_use]
    pub fn gate_count(&self) -> usize {
        self.operations
            .iter()
            .filter(|operation| operation.is_gate())
            .count()
    }

    /// Returns the number of barrier operations.
    #[must_use]
    pub fn barrier_count(&self) -> usize {
        self.operations
            .iter()
            .filter(|operation| operation.is_barrier())
            .count()
    }

    /// Returns the number of inserted SWAP movement operations.
    #[must_use]
    pub fn swap_count(&self) -> usize {
        self.operations
            .iter()
            .filter(|operation| {
                matches!(
                    operation,
                    RoutingOperation::Move(
                        RoutingMove::Swap { .. }
                    )
                )
            })
            .count()
    }

    /// Returns the number of bridge movement operations.
    #[must_use]
    pub fn bridge_count(&self) -> usize {
        self.operations
            .iter()
            .filter(|operation| {
                matches!(
                    operation,
                    RoutingOperation::Move(
                        RoutingMove::Bridge { .. }
                    )
                )
            })
            .count()
    }

    /// Returns the number of permutation movement operations.
    #[must_use]
    pub fn permutation_count(&self) -> usize {
        self.operations
            .iter()
            .filter(|operation| {
                matches!(
                    operation,
                    RoutingOperation::Move(
                        RoutingMove::Permutation { .. }
                    )
                )
            })
            .count()
    }

    /// Returns all routed gate operations.
    #[must_use]
    pub fn routed_gates(
        &self,
    ) -> impl Iterator<Item = &RoutingOperation> {
        self.operations.iter().filter(|operation| operation.is_gate())
    }

    /// Returns all movement operations.
    #[must_use]
    pub fn movements(
        &self,
    ) -> impl Iterator<Item = &RoutingOperation> {
        self.operations.iter().filter(|operation| operation.is_move())
    }

    // =========================================================================
    // Verification
    // =========================================================================

    /// Returns whether final verification passed.
    #[must_use]
    pub const fn verification_passed(&self) -> bool {
        self.verification.status.passed()
    }

    /// Returns whether verification was skipped.
    #[must_use]
    pub const fn verification_skipped(&self) -> bool {
        self.verification.status.not_requested()
    }

    // =========================================================================
    // Reproducibility
    // =========================================================================

    /// Returns the routing invocation identifier, if one was assigned.
    #[must_use]
    pub const fn routing_id(&self) -> Option<RoutingId> {
        self.reproducibility.routing_id
    }

    /// Returns the routing seed, if one was assigned.
    #[must_use]
    pub const fn seed(&self) -> Option<RoutingSeed> {
        self.reproducibility.seed
    }

    /// Returns whether the result claims deterministic execution.
    #[must_use]
    pub const fn is_deterministic(&self) -> bool {
        self.reproducibility.deterministic
    }

    /// Returns whether this result contains sufficient identifying metadata for
    /// replay tooling.
    #[must_use]
    pub fn is_replay_identifiable(&self) -> bool {
        self.reproducibility.is_replay_identifiable()
    }

    // =========================================================================
    // Quality
    // =========================================================================

    /// Returns the number of inserted SWAPs.
    #[must_use]
    pub const fn inserted_swaps(&self) -> usize {
        self.metrics.inserted_swaps
    }

    /// Returns total routing movement count.
    #[must_use]
    pub const fn inserted_moves(&self) -> usize {
        self.metrics.inserted_moves
    }

    /// Returns final circuit depth.
    #[must_use]
    pub const fn final_depth(&self) -> usize {
        self.metrics.final_depth
    }

    /// Returns compiler routing duration.
    #[must_use]
    pub const fn routing_duration(&self) -> Duration {
        self.metrics.routing_duration
    }

    /// Returns whether routing introduced no movement.
    #[must_use]
    pub const fn introduced_no_movement(&self) -> bool {
        self.metrics.inserted_moves == 0
    }

    /// Returns whether the result is usable according to its disposition.
    #[must_use]
    pub const fn is_success(&self) -> bool {
        self.disposition.is_success()
    }

    /// Returns whether this result involved actual routing.
    #[must_use]
    pub const fn involved_routing(&self) -> bool {
        self.disposition.involved_routing()
    }

    // =========================================================================
    // Consistency checks
    // =========================================================================

    /// Performs cheap result-internal consistency checks.
    ///
    /// This does not replace `verification.rs`. It only checks that the result
    /// object itself is internally coherent.
    ///
    /// Returns `true` when:
    ///
    /// - operation count agrees with the stored operation stream;
    /// - movement counts agree with the operation stream;
    /// - the routing permutation is internally consistent;
    /// - floating-point metrics are finite when present;
    /// - verification status is coherent.
    #[must_use]
    pub fn is_internally_consistent(&self) -> bool {
        if self.metrics.final_operations != self.operations.len() {
            return false;
        }

        if self.metrics.inserted_swaps != self.swap_count() {
            return false;
        }

        if self.metrics.inserted_bridges != self.bridge_count() {
            return false;
        }

        if self.metrics.inserted_permutations != self.permutation_count() {
            return false;
        }

        if self.metrics.inserted_moves != self.movement_count() {
            return false;
        }

        if self.metrics.final_gate_operations != self.gate_count() {
            return false;
        }

        if self.metrics.routing_overhead_operations
            < self.metrics.inserted_moves
        {
            return false;
        }

        if !self.metrics.floating_point_values_are_finite() {
            return false;
        }

        if self.verification.status == VerificationStatus::Passed
            && self.verification.level == VerificationLevel::None
        {
            return false;
        }

        true
    }

    // =========================================================================
    // Conversion helpers
    // =========================================================================

    /// Returns an immutable snapshot of the final mapping.
    #[must_use]
    pub fn final_mapping_snapshot(
        &self,
    ) -> QubitMappingSnapshot {
        self.layout.final_mapping.clone()
    }

    /// Returns an immutable snapshot of the initial mapping.
    #[must_use]
    pub fn initial_mapping_snapshot(
        &self,
    ) -> QubitMappingSnapshot {
        self.layout.initial_mapping.clone()
    }

    /// Reconstructs a standalone `QubitMapping` from the final snapshot.
    ///
    /// This method intentionally uses the public mapping constructor rather
    /// than accessing mapping internals.
    pub fn final_mapping(
        &self,
    ) -> Result<QubitMapping, crate::quantum::routing::mapping::MappingError>
    {
        QubitMapping::from_assignments(
            self.layout
                .final_mapping
                .logical_to_physical()
                .into_iter(),
        )
    }

    /// Reconstructs a standalone `QubitMapping` from the initial snapshot.
    pub fn initial_mapping(
        &self,
    ) -> Result<QubitMapping, crate::quantum::routing::mapping::MappingError>
    {
        QubitMapping::from_assignments(
            self.layout
                .initial_mapping
                .logical_to_physical()
                .into_iter(),
        )
    }

    // =========================================================================
    // Human-readable summary
    // =========================================================================

    /// Returns a concise stable summary suitable for diagnostics.
    #[must_use]
    pub fn summary(&self) -> RoutingResultSummary {
        RoutingResultSummary {
            disposition: self.disposition,
            algorithm: self.algorithm.clone(),
            layout_strategy: self.layout_strategy.clone(),
            objective: self.objective.clone(),
            logical_qubits: self.metrics.logical_qubits,
            physical_qubits: self.metrics.physical_qubits,
            original_operations: self.metrics.original_operations,
            final_operations: self.metrics.final_operations,
            inserted_swaps: self.metrics.inserted_swaps,
            inserted_moves: self.metrics.inserted_moves,
            original_depth: self.metrics.original_depth,
            final_depth: self.metrics.final_depth,
            routing_depth: self.metrics.routing_depth,
            verification: self.verification.status,
            deterministic: self.reproducibility.deterministic,
        }
    }
}

// =============================================================================
// Routing result summary
// =============================================================================

/// Lightweight result summary for diagnostics, CLI output, telemetry, and
/// benchmarking.
///
/// This deliberately excludes the potentially large operation stream and
/// mapping snapshots.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoutingResultSummary {
    /// Routing disposition.
    pub disposition: RouteDisposition,

    /// Selected algorithm.
    pub algorithm: RoutingAlgorithm,

    /// Initial layout strategy.
    pub layout_strategy: LayoutStrategy,

    /// Optimization objective.
    pub objective: RoutingObjective,

    /// Logical qubit count.
    pub logical_qubits: usize,

    /// Physical qubit count.
    pub physical_qubits: usize,

    /// Original operation count.
    pub original_operations: usize,

    /// Final operation count.
    pub final_operations: usize,

    /// Inserted SWAP count.
    pub inserted_swaps: usize,

    /// Inserted movement count.
    pub inserted_moves: usize,

    /// Original circuit depth.
    pub original_depth: usize,

    /// Final routed circuit depth.
    pub final_depth: usize,

    /// Routing depth overhead.
    pub routing_depth: usize,

    /// Verification state.
    pub verification: VerificationStatus,

    /// Deterministic execution flag.
    pub deterministic: bool,
}

impl RoutingResultSummary {
    /// Returns whether routing inserted no movement.
    #[must_use]
    pub const fn is_zero_overhead(&self) -> bool {
        self.inserted_moves == 0
            && self.routing_depth == 0
    }
}

impl fmt::Display for RoutingResultSummary {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            formatter,
            "routing disposition={}, algorithm={}, layout={}, \
             objective={}, logical_qubits={}, physical_qubits={}, \
             original_ops={}, final_ops={}, swaps={}, moves={}, \
             original_depth={}, final_depth={}, routing_depth={}, \
             verification={}, deterministic={}",
            self.disposition_name(),
            self.algorithm_name(),
            self.layout_name(),
            self.objective_name(),
            self.logical_qubits,
            self.physical_qubits,
            self.original_operations,
            self.final_operations,
            self.inserted_swaps,
            self.inserted_moves,
            self.original_depth,
            self.final_depth,
            self.routing_depth,
            self.verification,
            self.deterministic,
        )
    }
}

impl RoutingResultSummary {
    fn disposition_name(&self) -> &'static str {
        match self.disposition {
            RouteDisposition::Routed => "routed",
            RouteDisposition::AlreadyExecutable => "already_executable",
            RouteDisposition::NotRequested => "not_requested",
            RouteDisposition::Fallback => "fallback",
            RouteDisposition::Approximate => "approximate",
        }
    }

    fn algorithm_name(&self) -> &str {
        self.algorithm.name()
    }

    fn layout_name(&self) -> &str {
        self.layout_strategy.name()
    }

    fn objective_name(&self) -> &str {
        self.objective.name()
    }
}

// =============================================================================
// Builder
// =============================================================================

/// Builder for assembling a `RoutingResult` without requiring callers to
/// construct every field manually.
///
/// The builder is intentionally lightweight. It does not perform routing or
/// full semantic validation.
///
/// The router should use this builder only after it has completed routing and
/// verification.
#[derive(Debug, Clone)]
pub struct RoutingResultBuilder {
    disposition: RouteDisposition,
    algorithm: RoutingAlgorithm,
    layout_strategy: LayoutStrategy,
    objective: RoutingObjective,
    mode: RoutingMode,
    initial_mapping: Option<QubitMappingSnapshot>,
    final_mapping: Option<QubitMappingSnapshot>,
    operations: Vec<RoutingOperation>,
    metrics: RoutingMetrics,
    verification: VerificationSummary,
    reproducibility: ReproducibilityMetadata,
    events: Vec<RoutingEvent>,
}

impl RoutingResultBuilder {
    /// Creates a builder with explicit routing policy.
    #[must_use]
    pub fn new(
        disposition: RouteDisposition,
        algorithm: RoutingAlgorithm,
        layout_strategy: LayoutStrategy,
        objective: RoutingObjective,
        mode: RoutingMode,
    ) -> Self {
        Self {
            disposition,
            algorithm,
            layout_strategy,
            objective,
            mode,
            initial_mapping: None,
            final_mapping: None,
            operations: Vec::new(),
            metrics: RoutingMetrics::empty(),
            verification: VerificationSummary::default(),
            reproducibility: ReproducibilityMetadata::default(),
            events: Vec::new(),
        }
    }

    /// Sets the initial mapping snapshot.
    #[must_use]
    pub fn with_initial_mapping(
        mut self,
        mapping: QubitMappingSnapshot,
    ) -> Self {
        self.initial_mapping = Some(mapping);
        self
    }

    /// Sets the final mapping snapshot.
    #[must_use]
    pub fn with_final_mapping(
        mut self,
        mapping: QubitMappingSnapshot,
    ) -> Self {
        self.final_mapping = Some(mapping);
        self
    }

    /// Sets the operation stream.
    #[must_use]
    pub fn with_operations(
        mut self,
        operations: Vec<RoutingOperation>,
    ) -> Self {
        self.operations = operations;
        self
    }

    /// Sets metrics.
    #[must_use]
    pub fn with_metrics(
        mut self,
        metrics: RoutingMetrics,
    ) -> Self {
        self.metrics = metrics;
        self
    }

    /// Sets verification summary.
    #[must_use]
    pub fn with_verification(
        mut self,
        verification: VerificationSummary,
    ) -> Self {
        self.verification = verification;
        self
    }

    /// Sets reproducibility metadata.
    #[must_use]
    pub fn with_reproducibility(
        mut self,
        reproducibility: ReproducibilityMetadata,
    ) -> Self {
        self.reproducibility = reproducibility;
        self
    }

    /// Sets routing events.
    #[must_use]
    pub fn with_events(
        mut self,
        events: Vec<RoutingEvent>,
    ) -> Self {
        self.events = events;
        self
    }

    /// Adds one routing event.
    pub fn push_event(
        &mut self,
        event: RoutingEvent,
    ) {
        self.events.push(event);
    }

    /// Builds the final routing result.
    ///
    /// Returns `None` if either required mapping snapshot was not supplied.
    ///
    /// This method does not return `RoutingError` because the missing mappings
    /// are a builder-programming error rather than a routing execution error.
    pub fn build(self) -> Option<RoutingResult> {
        let initial_mapping = self.initial_mapping?;
        let final_mapping = self.final_mapping?;

        let mut result = RoutingResult::new(
            self.disposition,
            self.algorithm,
            self.layout_strategy,
            self.objective,
            self.mode,
            initial_mapping,
            final_mapping,
            self.operations,
            self.metrics,
            self.verification,
            self.reproducibility,
        );

        result.events = self.events;

        Some(result)
    }
}

// =============================================================================
// Unit tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::quantum::routing::mapping::QubitMapping;
    use crate::quantum::routing::types::{
        GateIdentity,
        LogicalQubitId,
        PhysicalQubitId,
        RoutingMove,
        RoutingOperation,
    };

    fn mapping_snapshot() -> QubitMappingSnapshot {
        let mapping = QubitMapping::from_assignments([
            (
                LogicalQubitId::new(0),
                PhysicalQubitId::new(0),
            ),
            (
                LogicalQubitId::new(1),
                PhysicalQubitId::new(1),
            ),
        ])
        .expect("test mapping must be valid");

        mapping.snapshot()
    }

    fn routed_operations() -> Vec<RoutingOperation> {
        vec![
            RoutingOperation::Move(RoutingMove::Swap {
                a: PhysicalQubitId::new(0),
                b: PhysicalQubitId::new(1),
            }),
            RoutingOperation::Gate {
                gate: GateIdentity::Cx,
                operands: vec![
                    PhysicalQubitId::new(1),
                    PhysicalQubitId::new(0),
                ],
                logical_operands: vec![
                    LogicalQubitId::new(0),
                    LogicalQubitId::new(1),
                ],
            },
        ]
    }

    #[test]
    fn metrics_default_is_zeroed() {
        let metrics = RoutingMetrics::default();

        assert_eq!(metrics.logical_qubits, 0);
        assert_eq!(metrics.physical_qubits, 0);
        assert_eq!(metrics.original_operations, 0);
        assert_eq!(metrics.final_operations, 0);
        assert_eq!(metrics.inserted_swaps, 0);
        assert_eq!(metrics.inserted_moves, 0);
        assert_eq!(metrics.total_duration, Duration::ZERO);
    }

    #[test]
    fn reproducibility_metadata_detects_replay_identity() {
        let metadata = ReproducibilityMetadata::deterministic()
            .with_routing_version("routing-1")
            .with_algorithm_version("sabre-1")
            .with_configuration_hash("config")
            .with_input_hash("input")
            .with_topology_hash("topology");

        assert!(metadata.is_replay_identifiable());
        assert!(metadata.deterministic);
    }

    #[test]
    fn verification_summary_counts_checks() {
        let summary = VerificationSummary::passed(
            VerificationLevel::Standard,
        )
        .with_structural_checks(2)
        .with_mapping_checks(3)
        .with_executability_checks(4)
        .with_preservation_checks(5)
        .with_passed_checks(14);

        assert_eq!(summary.total_checks(), 14);
        assert!(summary.all_checks_passed());
    }

    #[test]
    fn layout_summary_detects_changed_mapping() {
        let initial = mapping_snapshot();

        let changed_mapping =
            QubitMapping::from_assignments([
                (
                    LogicalQubitId::new(0),
                    PhysicalQubitId::new(1),
                ),
                (
                    LogicalQubitId::new(1),
                    PhysicalQubitId::new(0),
                ),
            ])
            .expect("test mapping must be valid");

        let final_mapping = changed_mapping.snapshot();

        let summary =
            LayoutSummary::new(initial, final_mapping);

        assert!(summary.changed);
        assert!(!summary.is_unchanged());
        assert_eq!(summary.initial_mapped_qubits, 2);
        assert_eq!(summary.final_mapped_qubits, 2);
    }

    #[test]
    fn result_counts_operations_from_operation_stream() {
        let operations = routed_operations();

        let mut metrics = RoutingMetrics::new(2, 2);
        metrics.original_operations = 1;
        metrics.final_operations = operations.len();
        metrics.final_gate_operations = 1;
        metrics.inserted_swaps = 1;
        metrics.inserted_moves = 1;
        metrics.routing_overhead_operations = 1;

        let mapping = mapping_snapshot();

        let result = RoutingResult::routed(
            RoutingAlgorithm::ShortestPath,
            LayoutStrategy::Trivial,
            RoutingObjective::SwapCount,
            RoutingMode::Strict,
            mapping.clone(),
            mapping,
            operations,
            metrics,
            VerificationSummary::passed(
                VerificationLevel::Standard,
            ),
            ReproducibilityMetadata::deterministic(),
        );

        assert_eq!(result.swap_count(), 1);
        assert_eq!(result.movement_count(), 1);
        assert_eq!(result.gate_count(), 1);
        assert_eq!(result.operation_count(), 2);
    }

    #[test]
    fn result_internal_consistency_passes_for_valid_result() {
        let operations = routed_operations();

        let mut metrics = RoutingMetrics::new(2, 2);
        metrics.original_operations = 1;
        metrics.final_operations = 2;
        metrics.final_gate_operations = 1;
        metrics.inserted_swaps = 1;
        metrics.inserted_moves = 1;
        metrics.routing_overhead_operations = 1;

        let mapping = mapping_snapshot();

        let result = RoutingResult::routed(
            RoutingAlgorithm::ShortestPath,
            LayoutStrategy::Trivial,
            RoutingObjective::SwapCount,
            RoutingMode::Strict,
            mapping.clone(),
            mapping,
            operations,
            metrics,
            VerificationSummary::passed(
                VerificationLevel::Standard,
            ),
            ReproducibilityMetadata::deterministic(),
        );

        assert!(result.is_internally_consistent());
    }

    #[test]
    fn result_internal_consistency_detects_count_mismatch() {
        let operations = routed_operations();

        let mut metrics = RoutingMetrics::new(2, 2);
        metrics.original_operations = 1;
        metrics.final_operations = 99;
        metrics.final_gate_operations = 1;
        metrics.inserted_swaps = 1;
        metrics.inserted_moves = 1;
        metrics.routing_overhead_operations = 1;

        let mapping = mapping_snapshot();

        let result = RoutingResult::routed(
            RoutingAlgorithm::ShortestPath,
            LayoutStrategy::Trivial,
            RoutingObjective::SwapCount,
            RoutingMode::Strict,
            mapping.clone(),
            mapping,
            operations,
            metrics,
            VerificationSummary::passed(
                VerificationLevel::Standard,
            ),
            ReproducibilityMetadata::deterministic(),
        );

        assert!(!result.is_internally_consistent());
    }

    #[test]
    fn routing_permutation_reports_changed_locations() {
        let initial = mapping_snapshot();

        let final_mapping =
            QubitMapping::from_assignments([
                (
                    LogicalQubitId::new(0),
                    PhysicalQubitId::new(1),
                ),
                (
                    LogicalQubitId::new(1),
                    PhysicalQubitId::new(0),
                ),
            ])
            .expect("test mapping must be valid")
            .snapshot();

        let mut metrics = RoutingMetrics::new(2, 2);
        metrics.final_operations = 0;

        let result = RoutingResult::routed(
            RoutingAlgorithm::Basic,
            LayoutStrategy::Trivial,
            RoutingObjective::SwapCount,
            RoutingMode::Strict,
            initial,
            final_mapping,
            Vec::new(),
            metrics,
            VerificationSummary::not_requested(),
            ReproducibilityMetadata::deterministic(),
        );

        let permutation = result.routing_permutation();

        assert_eq!(permutation.len(), 2);

        assert_eq!(
            permutation[0],
            (
                LogicalQubitId::new(0),
                PhysicalQubitId::new(0),
                PhysicalQubitId::new(1),
            )
        );

        assert_eq!(
            permutation[1],
            (
                LogicalQubitId::new(1),
                PhysicalQubitId::new(1),
                PhysicalQubitId::new(0),
            )
        );
    }

    #[test]
    fn summary_is_lightweight_and_accurate() {
        let mapping = mapping_snapshot();

        let metrics = RoutingMetrics::new(2, 2);

        let result = RoutingResult::already_executable(
            RoutingAlgorithm::None,
            LayoutStrategy::Fixed,
            RoutingObjective::SwapCount,
            RoutingMode::Strict,
            mapping,
            Vec::new(),
            metrics,
            VerificationSummary::passed(
                VerificationLevel::Standard,
            ),
            ReproducibilityMetadata::deterministic(),
        );

        let summary = result.summary();

        assert_eq!(
            summary.disposition,
            RouteDisposition::AlreadyExecutable
        );
        assert_eq!(summary.logical_qubits, 2);
        assert_eq!(summary.physical_qubits, 2);
        assert_eq!(summary.inserted_swaps, 0);
        assert!(summary.is_zero_overhead());
    }

    #[test]
    fn builder_requires_both_mapping_snapshots() {
        let builder = RoutingResultBuilder::new(
            RouteDisposition::Routed,
            RoutingAlgorithm::Basic,
            LayoutStrategy::Trivial,
            RoutingObjective::SwapCount,
            RoutingMode::Strict,
        );

        assert!(builder.build().is_none());
    }

    #[test]
    fn builder_constructs_complete_result() {
        let mapping = mapping_snapshot();

        let builder = RoutingResultBuilder::new(
            RouteDisposition::AlreadyExecutable,
            RoutingAlgorithm::None,
            LayoutStrategy::Fixed,
            RoutingObjective::SwapCount,
            RoutingMode::Strict,
        )
        .with_initial_mapping(mapping.clone())
        .with_final_mapping(mapping)
        .with_metrics(RoutingMetrics::new(2, 2))
        .with_verification(
            VerificationSummary::passed(
                VerificationLevel::Standard,
            ),
        );

        let result =
            builder.build().expect("builder should succeed");

        assert_eq!(
            result.disposition,
            RouteDisposition::AlreadyExecutable
        );
        assert!(result.is_internally_consistent());
    }

    #[test]
    fn zero_overhead_is_detected() {
        let metrics = RoutingMetrics::new(2, 2);

        assert!(metrics.has_zero_overhead());
    }

    #[test]
    fn floating_point_validation_rejects_nan() {
        let mut metrics = RoutingMetrics::new(2, 2);
        metrics.estimated_error = Some(f64::NAN);

        assert!(!metrics.floating_point_values_are_finite());
    }

    #[test]
    fn floating_point_validation_accepts_finite_values() {
        let mut metrics = RoutingMetrics::new(2, 2);
        metrics.estimated_error = Some(0.01);
        metrics.estimated_fidelity = Some(0.99);
        metrics.objective_value = Some(4.0);

        assert!(metrics.floating_point_values_are_finite());
    }

    #[test]
    fn verification_pass_requires_non_none_level() {
        let mapping = mapping_snapshot();

        let metrics = RoutingMetrics::new(2, 2);

        let result = RoutingResult::already_executable(
            RoutingAlgorithm::None,
            LayoutStrategy::Fixed,
            RoutingObjective::SwapCount,
            RoutingMode::Strict,
            mapping,
            Vec::new(),
            metrics,
            VerificationSummary {
                level: VerificationLevel::None,
                status: VerificationStatus::Passed,
                structural_checks: 0,
                mapping_checks: 0,
                executability_checks: 0,
                preservation_checks: 0,
                passed_checks: 0,
                verifier_version: None,
            },
            ReproducibilityMetadata::deterministic(),
        );

        assert!(!result.is_internally_consistent());
    }
}