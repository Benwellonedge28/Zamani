//! Zamani Quantum Optimization — Optimization Result
//!
//! Stable, backend-independent result contract for one logical quantum
//! optimization invocation.
//!
//! # Architectural role
//!
//! `result.rs` is the terminal data contract of the optimization subsystem.
//! It describes what an optimizer produced without owning optimization
//! algorithms, pass scheduling, cost-model implementation, verification
//! algorithms, routing, scheduling, hardware execution, benchmarking, or
//! frontend parsing.
//!
//! The canonical semantic representation remains
//! `crate::quantum::ir::QuantumCircuit`.
//!
//! ```text
//! Zamani source / external format
//!             │
//!             ▼
//!       quantum::ir
//!             │
//!             ▼
//!       optimization
//!             │
//!             ├── passes
//!             ├── analyses
//!             ├── rewrites
//!             ├── synthesis
//!             └── verification
//!             │
//!             ▼
//!     OptimizationResult
//!             │
//!       ┌─────┼───────────────┐
//!       ▼     ▼               ▼
//!    routing benchmarking diagnostics
//!       │
//!       ▼
//!    scheduling
//!       │
//!       ▼
//!    hardware/runtime
//! ```
//!
//! # Design goals
//!
//! This contract is designed for:
//!
//! - tiny circuits;
//! - very large circuits subject to available resources;
//! - deterministic and reproducible compilation;
//! - incremental/fixed-point optimization;
//! - target-aware optimization;
//! - multi-objective optimization;
//! - fault-tolerant optimization;
//! - synthesis and equality-saturation pipelines;
//! - partial optimization when configured limits are reached;
//! - post-optimization semantic verification;
//! - benchmarking and regression analysis;
//! - IDE/compiler diagnostics;
//! - future optimization passes without changing this file.
//!
//! The result is deliberately an owned value. The optimizer may therefore
//! release all invocation-local state after returning it.
//!
//! # Important invariants
//!
//! 1. The result contains the canonical Quantum IR, never an optimizer-owned
//!    replacement circuit representation.
//! 2. A result never performs backend I/O.
//! 3. A result never performs verification itself; it records verification
//!    performed by the verification subsystem.
//! 4. A result never decides whether a circuit is better; the cost/planner
//!    subsystem makes that decision before construction.
//! 5. Result metadata is observational. It must not affect circuit semantics.
//! 6. Counters use `u128` where practical so large workloads do not overflow at
//!    ordinary machine-scale limits.
//! 7. Optional information remains optional. A caller may construct a valid
//!    result without a cost model, verification engine, or provenance service.
//! 8. No `unsafe` code is permitted.
//!
//! # Integration contract
//!
//! ## `quantum::ir`
//!
//! `OptimizationResult::new` takes ownership of the canonical
//! `quantum::ir::QuantumCircuit`. No second Quantum IR is introduced.
//!
//! ## `optimization::pipeline`
//!
//! The pipeline is the primary producer. It should populate status, summary,
//! pass records, cost snapshots, verification status, diagnostics, and
//! provenance before returning the result.
//!
//! ## `optimization::statistics`
//!
//! The statistics subsystem remains the authoritative accounting engine.
//! `OptimizationMetrics` here is intentionally a small result-facing summary;
//! detailed statistics can be attached through pass records and future report
//! adapters without coupling this file to a particular statistics storage
//! implementation.
//!
//! ## `optimization::cost`
//!
//! `OptimizationCostSnapshot` is a stable, representation-only snapshot. The
//! cost subsystem can convert its richer internal cost model into this snapshot
//! without making `result.rs` depend on cost-model internals.
//!
//! ## `optimization::verification`
//!
//! Verification produces a `VerificationSummary` and the pipeline records it
//! here. This file deliberately does not import a verifier implementation.
//!
//! ## `optimization::provenance`
//!
//! `ProvenanceSnapshot` is intentionally storage-neutral. The provenance
//! subsystem can populate it directly or provide a conversion from its richer
//! provenance model later without changing this result contract.
//!
//! ## `routing` / `scheduling` / `hardware`
//!
//! These subsystems consume `result.circuit()` or `into_circuit()`. The result
//! does not depend on them and never calls them.
//!
//! ## `benchmarking`
//!
//! Benchmarking consumes the circuit and the observational fields. There is no
//! benchmarking dependency from optimization.
//!
//! # Rust compatibility
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021
//! - stable Rust only
//! - no nightly features
//! - no `unsafe`

#![forbid(unsafe_code)]

use std::fmt;
use std::time::Duration;

use crate::quantum::ir::QuantumCircuit;

// =============================================================================
// Public result type
// =============================================================================

/// The complete outcome of one optimization invocation.
///
/// The optimized circuit is always owned by the result. Status and metadata
/// explain how that circuit was obtained.
pub struct OptimizationResult {
    circuit: QuantumCircuit,
    status: OptimizationStatus,
    summary: OptimizationSummary,
    metrics: OptimizationMetrics,
    cost: OptimizationCostSnapshot,
    verification: VerificationSummary,
    passes: Vec<PassResult>,
    diagnostics: Vec<OptimizationDiagnostic>,
    provenance: ProvenanceSnapshot,
}

impl OptimizationResult {
    /// Creates a result from the canonical optimized Quantum IR.
    ///
    /// Construction does not perform validation, cost evaluation, or
    /// verification. Those responsibilities remain with their owning
    /// subsystems.
    #[must_use]
    pub fn new(circuit: QuantumCircuit) -> Self {
        Self {
            circuit,
            status: OptimizationStatus::Unchanged,
            summary: OptimizationSummary::default(),
            metrics: OptimizationMetrics::default(),
            cost: OptimizationCostSnapshot::default(),
            verification: VerificationSummary::not_performed(),
            passes: Vec::new(),
            diagnostics: Vec::new(),
            provenance: ProvenanceSnapshot::default(),
        }
    }

    /// Returns the final optimized circuit by shared reference.
    #[must_use]
    pub fn circuit(&self) -> &QuantumCircuit {
        &self.circuit
    }

    /// Consumes the result and returns the final optimized circuit.
    #[must_use]
    pub fn into_circuit(self) -> QuantumCircuit {
        self.circuit
    }

    /// Returns the final optimization status.
    #[must_use]
    pub const fn status(&self) -> OptimizationStatus {
        self.status
    }

    /// Returns the high-level result summary.
    #[must_use]
    pub const fn summary(&self) -> &OptimizationSummary {
        &self.summary
    }

    /// Returns result-facing aggregate metrics.
    #[must_use]
    pub const fn metrics(&self) -> &OptimizationMetrics {
        &self.metrics
    }

    /// Returns the initial/final cost snapshot.
    #[must_use]
    pub const fn cost(&self) -> &OptimizationCostSnapshot {
        &self.cost
    }

    /// Returns the recorded verification outcome.
    #[must_use]
    pub const fn verification(&self) -> &VerificationSummary {
        &self.verification
    }

    /// Returns the pass records in execution order.
    #[must_use]
    pub fn passes(&self) -> &[PassResult] {
        &self.passes
    }

    /// Returns optimization diagnostics in emission order.
    #[must_use]
    pub fn diagnostics(&self) -> &[OptimizationDiagnostic] {
        &self.diagnostics
    }

    /// Returns reproducibility/provenance information.
    #[must_use]
    pub const fn provenance(&self) -> &ProvenanceSnapshot {
        &self.provenance
    }

    /// Returns true when optimization produced a circuit-changing result.
    #[must_use]
    pub const fn changed(&self) -> bool {
        self.status.changed()
    }

    /// Returns true when optimization completed normally.
    #[must_use]
    pub const fn completed(&self) -> bool {
        self.status.completed()
    }

    /// Returns true only when verification was performed and succeeded.
    #[must_use]
    pub const fn is_verified(&self) -> bool {
        self.verification.is_successful()
    }

    /// Sets the final status.
    pub fn set_status(&mut self, status: OptimizationStatus) {
        self.status = status;
    }

    /// Replaces the result summary.
    pub fn set_summary(&mut self, summary: OptimizationSummary) {
        self.summary = summary;
    }

    /// Replaces result-facing aggregate metrics.
    pub fn set_metrics(&mut self, metrics: OptimizationMetrics) {
        self.metrics = metrics;
    }

    /// Replaces the cost snapshot.
    pub fn set_cost(&mut self, cost: OptimizationCostSnapshot) {
        self.cost = cost;
    }

    /// Records verification performed by the verification subsystem.
    pub fn set_verification(&mut self, verification: VerificationSummary) {
        self.verification = verification;
    }

    /// Adds one pass result.
    pub fn record_pass(&mut self, pass: PassResult) {
        self.passes.push(pass);
    }

    /// Adds one diagnostic.
    pub fn record_diagnostic(&mut self, diagnostic: OptimizationDiagnostic) {
        self.diagnostics.push(diagnostic);
    }

    /// Replaces the provenance snapshot.
    pub fn set_provenance(&mut self, provenance: ProvenanceSnapshot) {
        self.provenance = provenance;
    }

    /// Returns mutable access to result-facing metrics.
    ///
    /// The circuit itself remains protected behind the canonical IR API.
    pub fn metrics_mut(&mut self) -> &mut OptimizationMetrics {
        &mut self.metrics
    }
}

impl fmt::Debug for OptimizationResult {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("OptimizationResult")
            .field("status", &self.status)
            .field("summary", &self.summary)
            .field("metrics", &self.metrics)
            .field("cost", &self.cost)
            .field("verification", &self.verification)
            .field("passes", &self.passes)
            .field("diagnostics", &self.diagnostics)
            .field("provenance", &self.provenance)
            .finish()
    }
}

// =============================================================================
// Result status
// =============================================================================

/// Final state of an optimization invocation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OptimizationStatus {
    /// Optimization changed the circuit and completed successfully.
    Optimized,

    /// Optimization completed successfully but made no change.
    Unchanged,

    /// Optimization produced a valid partial improvement before stopping.
    PartiallyOptimized,

    /// A configured resource, iteration, or deadline limit stopped the run.
    LimitReached,

    /// Verification found that the produced circuit was not acceptable.
    VerificationFailed,

    /// The optimizer failed before producing a successful result.
    Failed,
}

impl OptimizationStatus {
    /// Returns true when the result is known to contain a circuit change.
    #[must_use]
    pub const fn changed(self) -> bool {
        matches!(
            self,
            Self::Optimized | Self::PartiallyOptimized
        )
    }

    /// Returns true for normal successful completion.
    #[must_use]
    pub const fn completed(self) -> bool {
        matches!(self, Self::Optimized | Self::Unchanged)
    }

    /// Returns true for hard failure states.
    #[must_use]
    pub const fn is_failure(self) -> bool {
        matches!(
            self,
            Self::VerificationFailed | Self::Failed
        )
    }

    /// Returns true when optimization stopped because of a budget.
    #[must_use]
    pub const fn stopped_by_limit(self) -> bool {
        matches!(
            self,
            Self::LimitReached | Self::PartiallyOptimized
        )
    }
}

impl fmt::Display for OptimizationStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Optimized => "optimized",
            Self::Unchanged => "unchanged",
            Self::PartiallyOptimized => "partially_optimized",
            Self::LimitReached => "limit_reached",
            Self::VerificationFailed => "verification_failed",
            Self::Failed => "failed",
        };

        formatter.write_str(value)
    }
}

// =============================================================================
// High-level summary
// =============================================================================

/// Compact summary suitable for diagnostics, CLI output, and benchmarking.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct OptimizationSummary {
    /// Number of passes requested by the planner.
    pub passes_requested: u128,

    /// Number of passes actually executed.
    pub passes_executed: u128,

    /// Number of passes that changed the circuit.
    pub passes_changed: u128,

    /// Number of passes intentionally skipped.
    pub passes_skipped: u128,

    /// Number of rewrite applications.
    pub rewrites_applied: u128,

    /// Operations before optimization.
    pub operations_before: u128,

    /// Operations after optimization.
    pub operations_after: u128,

    /// Two-qubit operations before optimization.
    pub two_qubit_operations_before: u128,

    /// Two-qubit operations after optimization.
    pub two_qubit_operations_after: u128,

    /// Logical depth before optimization.
    pub depth_before: u128,

    /// Logical depth after optimization.
    pub depth_after: u128,
}

impl OptimizationSummary {
    /// Returns operations removed, saturating at zero.
    #[must_use]
    pub const fn operations_removed(&self) -> u128 {
        self.operations_before
            .saturating_sub(self.operations_after)
    }

    /// Returns two-qubit operations removed.
    #[must_use]
    pub const fn two_qubit_operations_removed(&self) -> u128 {
        self.two_qubit_operations_before
            .saturating_sub(self.two_qubit_operations_after)
    }

    /// Returns logical-depth reduction.
    #[must_use]
    pub const fn depth_reduction(&self) -> u128 {
        self.depth_before
            .saturating_sub(self.depth_after)
    }
}

// =============================================================================
// Result-facing aggregate metrics
// =============================================================================

/// Resource metrics captured specifically for the returned result.
///
/// The detailed `statistics.rs` subsystem remains authoritative. This is the
/// compact terminal representation needed by callers of the optimizer.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct OptimizationMetrics {
    /// Number of operations removed.
    pub operations_removed: u128,

    /// Number of operations added.
    pub operations_added: u128,

    /// Number of operations replaced.
    pub operations_replaced: u128,

    /// Number of two-qubit operations removed.
    pub two_qubit_operations_removed: u128,

    /// Number of T gates removed.
    pub t_gates_removed: u128,

    /// Number of T gates added.
    pub t_gates_added: u128,

    /// Initial logical depth.
    pub depth_before: u128,

    /// Final logical depth.
    pub depth_after: u128,

    /// Initial T-depth.
    pub t_depth_before: u128,

    /// Final T-depth.
    pub t_depth_after: u128,

    /// Initial logical qubit count.
    pub qubits_before: u128,

    /// Final logical qubit count.
    pub qubits_after: u128,

    /// Total optimization wall-clock time.
    pub elapsed: Duration,
}

impl OptimizationMetrics {
    /// Returns logical-depth reduction.
    #[must_use]
    pub const fn depth_reduction(&self) -> u128 {
        self.depth_before
            .saturating_sub(self.depth_after)
    }

    /// Returns T-depth reduction.
    #[must_use]
    pub const fn t_depth_reduction(&self) -> u128 {
        self.t_depth_before
            .saturating_sub(self.t_depth_after)
    }

    /// Returns whether at least one resource dimension improved.
    #[must_use]
    pub const fn improved(&self) -> bool {
        self.operations_removed > 0
            || self.two_qubit_operations_removed > 0
            || self.t_gates_removed > 0
            || self.depth_after < self.depth_before
            || self.t_depth_after < self.t_depth_before
    }
}

// =============================================================================
// Cost snapshot
// =============================================================================

/// Backend-independent snapshot of optimization cost.
///
/// The full cost subsystem can contain richer weighted, lexicographic, or
/// Pareto objectives. This terminal snapshot stores only stable scalar
/// observations.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct OptimizationCostSnapshot {
    /// Whether an initial cost was supplied.
    pub has_initial: bool,

    /// Whether a final cost was supplied.
    pub has_final: bool,

    /// Initial total cost.
    pub initial_total: Option<f64>,

    /// Final total cost.
    pub final_total: Option<f64>,

    /// Initial two-qubit cost.
    pub initial_two_qubit: Option<f64>,

    /// Final two-qubit cost.
    pub final_two_qubit: Option<f64>,

    /// Initial depth cost.
    pub initial_depth: Option<f64>,

    /// Final depth cost.
    pub final_depth: Option<f64>,

    /// Initial fault-tolerant T cost.
    pub initial_t_cost: Option<f64>,

    /// Final fault-tolerant T cost.
    pub final_t_cost: Option<f64>,
}

impl OptimizationCostSnapshot {
    /// Creates a snapshot containing total costs.
    #[must_use]
    pub fn totals(initial: f64, final_cost: f64) -> Self {
        let initial_valid = initial.is_finite();
        let final_valid = final_cost.is_finite();

        Self {
            has_initial: initial_valid,
            has_final: final_valid,
            initial_total: initial_valid.then_some(initial),
            final_total: final_valid.then_some(final_cost),
            ..Self::empty()
        }
    }

    /// Returns an empty snapshot.
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            has_initial: false,
            has_final: false,
            initial_total: None,
            final_total: None,
            initial_two_qubit: None,
            final_two_qubit: None,
            initial_depth: None,
            final_depth: None,
            initial_t_cost: None,
            final_t_cost: None,
        }
    }

    /// Returns true when initial and final total costs are comparable.
    #[must_use]
    pub fn has_comparable_totals(&self) -> bool {
        self.initial_total
            .zip(self.final_total)
            .is_some_and(|(initial, final_cost)| {
                initial.is_finite() && final_cost.is_finite()
            })
    }

    /// Returns `final - initial` when both totals exist.
    #[must_use]
    pub fn total_delta(&self) -> Option<f64> {
        self.initial_total
            .zip(self.final_total)
            .map(|(initial, final_cost)| final_cost - initial)
    }
}

// =============================================================================
// Verification
// =============================================================================

/// Semantic verification state recorded in an optimization result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VerificationStatus {
    /// Verification was not requested or not performed.
    NotPerformed,

    /// Verification was requested but unavailable for this circuit.
    Unavailable,

    /// Exact semantic equivalence was established.
    Exact,

    /// Equivalence was established up to global phase.
    UpToGlobalPhase,

    /// Measurement-level equivalence was established.
    MeasurementEquivalent,

    /// Statistical/randomized verification succeeded.
    Statistical,

    /// Approximate equivalence succeeded within tolerance.
    Approximate,

    /// Verification detected a mismatch.
    Failed,
}

impl VerificationStatus {
    /// Returns true only for successful verification states.
    #[must_use]
    pub const fn successful(self) -> bool {
        matches!(
            self,
            Self::Exact
                | Self::UpToGlobalPhase
                | Self::MeasurementEquivalent
                | Self::Statistical
                | Self::Approximate
        )
    }
}

/// Verification information attached to an optimization result.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VerificationSummary {
    /// Verification state.
    pub status: VerificationStatus,

    /// Number of checks or samples performed.
    pub checks: u128,

    /// Statistical confidence, if available.
    pub confidence: Option<f64>,

    /// Approximation tolerance, if applicable.
    pub tolerance: Option<f64>,

    /// Optional fidelity/equivalence score.
    pub fidelity: Option<f64>,
}

impl VerificationSummary {
    /// Returns the canonical "not performed" state.
    #[must_use]
    pub const fn not_performed() -> Self {
        Self {
            status: VerificationStatus::NotPerformed,
            checks: 0,
            confidence: None,
            tolerance: None,
            fidelity: None,
        }
    }

    /// Returns true only for successful verification.
    #[must_use]
    pub const fn is_successful(&self) -> bool {
        self.status.successful()
    }

    /// Returns true if verification explicitly failed.
    #[must_use]
    pub const fn failed(&self) -> bool {
        matches!(self.status, VerificationStatus::Failed)
    }
}

// =============================================================================
// Pass results
// =============================================================================

/// Result of one optimization pass invocation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PassResult {
    /// Stable pass identifier.
    pub pass_id: String,

    /// Broad optimization phase.
    pub phase: PassPhase,

    /// Pass outcome.
    pub outcome: PassOutcome,

    /// Operations before this pass.
    pub operations_before: Option<u128>,

    /// Operations after this pass.
    pub operations_after: Option<u128>,

    /// Rewrite applications.
    pub rewrites: u128,

    /// Operations removed.
    pub operations_removed: u128,

    /// Operations added.
    pub operations_added: u128,

    /// Verification checks associated with this pass.
    pub verification_checks: u128,

    /// Pass execution time.
    pub elapsed: Duration,
}

impl PassResult {
    /// Creates a pass record.
    #[must_use]
    pub fn new(
        pass_id: impl Into<String>,
        phase: PassPhase,
        outcome: PassOutcome,
    ) -> Self {
        Self {
            pass_id: pass_id.into(),
            phase,
            outcome,
            operations_before: None,
            operations_after: None,
            rewrites: 0,
            operations_removed: 0,
            operations_added: 0,
            verification_checks: 0,
            elapsed: Duration::ZERO,
        }
    }
}

/// Broad pass category.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PassPhase {
    Validation,
    Normalization,
    Local,
    Algebraic,
    Parameter,
    Clifford,
    PhasePolynomial,
    FaultTolerant,
    Synthesis,
    Structural,
    TargetAware,
    Search,
    Verification,
    Other,
}

/// Outcome of one optimization pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PassOutcome {
    Changed,
    Unchanged,
    Skipped,
    LimitReached,
    PartiallyCompleted,
    Failed,
}

impl PassOutcome {
    /// Returns true if this pass changed the circuit.
    #[must_use]
    pub const fn changed(self) -> bool {
        matches!(
            self,
            Self::Changed | Self::PartiallyCompleted
        )
    }
}

// =============================================================================
// Diagnostics
// =============================================================================

/// Severity of an optimization diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DiagnosticSeverity {
    Info,
    Warning,
    Error,
}

/// Machine-readable optimization diagnostic.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OptimizationDiagnostic {
    /// Stable diagnostic code.
    pub code: String,

    /// Diagnostic severity.
    pub severity: DiagnosticSeverity,

    /// Human-readable message.
    pub message: String,

    /// Pass that emitted the diagnostic.
    pub pass_id: Option<String>,

    /// Rewrite rule that emitted the diagnostic.
    pub rule_id: Option<String>,

    /// Operation identifier represented as a stable string.
    pub operation_id: Option<String>,
}

impl OptimizationDiagnostic {
    /// Creates a diagnostic with no optional context.
    #[must_use]
    pub fn new(
        code: impl Into<String>,
        severity: DiagnosticSeverity,
        message: impl Into<String>,
    ) -> Self {
        Self {
            code: code.into(),
            severity,
            message: message.into(),
            pass_id: None,
            rule_id: None,
            operation_id: None,
        }
    }
}

// =============================================================================
// Provenance
// =============================================================================

/// Storage-neutral reproducibility metadata.
///
/// The dedicated provenance subsystem can populate this structure directly.
/// Unstable process-local information is deliberately not required.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ProvenanceSnapshot {
    /// Optimizer/compiler version.
    pub optimizer_version: Option<String>,

    /// Optimization profile.
    pub profile: Option<String>,

    /// Optimization target.
    pub target: Option<String>,

    /// Random seed, if applicable.
    pub random_seed: Option<u64>,

    /// Input circuit hash.
    pub input_hash: Option<String>,

    /// Output circuit hash.
    pub output_hash: Option<String>,

    /// Pipeline identifier/version.
    pub pipeline: Option<String>,

    /// Whether deterministic execution was requested.
    pub deterministic: Option<bool>,
}

// =============================================================================
// Construction helper
// =============================================================================

/// Creates an optimization result from a canonical circuit and final status.
#[must_use]
pub fn from_circuit(
    circuit: QuantumCircuit,
    status: OptimizationStatus,
) -> OptimizationResult {
    let mut result = OptimizationResult::new(circuit);
    result.set_status(status);
    result
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_semantics_are_stable() {
        assert!(OptimizationStatus::Optimized.changed());
        assert!(!OptimizationStatus::Unchanged.changed());
        assert!(OptimizationStatus::PartiallyOptimized.changed());
        assert!(!OptimizationStatus::LimitReached.changed());

        assert!(!OptimizationStatus::VerificationFailed.completed());
        assert!(OptimizationStatus::Optimized.completed());
    }

    #[test]
    fn summary_reductions_are_saturating() {
        let summary = OptimizationSummary {
            operations_before: 4,
            operations_after: 1,
            two_qubit_operations_before: 3,
            two_qubit_operations_after: 1,
            depth_before: 10,
            depth_after: 4,
            ..OptimizationSummary::default()
        };

        assert_eq!(summary.operations_removed(), 3);
        assert_eq!(summary.two_qubit_operations_removed(), 2);
        assert_eq!(summary.depth_reduction(), 6);
    }

    #[test]
    fn metrics_never_report_negative_reduction() {
        let metrics = OptimizationMetrics {
            depth_before: 2,
            depth_after: 5,
            t_depth_before: 1,
            t_depth_after: 3,
            ..OptimizationMetrics::default()
        };

        assert_eq!(metrics.depth_reduction(), 0);
        assert_eq!(metrics.t_depth_reduction(), 0);
        assert!(!metrics.improved());
    }

    #[test]
    fn empty_cost_snapshot_is_safe() {
        let cost = OptimizationCostSnapshot::empty();

        assert!(!cost.has_comparable_totals());
        assert_eq!(cost.total_delta(), None);
    }

    #[test]
    fn cost_snapshot_rejects_non_finite_totals() {
        let cost = OptimizationCostSnapshot::totals(f64::NAN, 1.0);

        assert!(!cost.has_initial);
        assert!(cost.has_final);
        assert!(!cost.has_comparable_totals());
    }

    #[test]
    fn verification_defaults_to_not_performed() {
        let verification = VerificationSummary::not_performed();

        assert!(!verification.is_successful());
        assert!(!verification.failed());
        assert_eq!(
            verification.status,
            VerificationStatus::NotPerformed
        );
    }

    #[test]
    fn pass_result_is_explicit() {
        let pass = PassResult::new(
            "local.cancellation",
            PassPhase::Local,
            PassOutcome::Changed,
        );

        assert_eq!(pass.pass_id, "local.cancellation");
        assert!(pass.outcome.changed());
        assert_eq!(pass.rewrites, 0);
    }
}