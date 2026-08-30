//! Zamani Quantum Optimization — Logical Width Optimization Pass.
//!
//! Production-grade logical-width analysis and optimization policy over the
//! canonical `crate::quantum::ir::QuantumCircuit`.
//!
//! # Architectural position
//!
//! ```text
//!                         quantum::ir::QuantumCircuit
//!                                    │
//!                                    ▼
//!                    optimization::passes::optimize_width
//!                                    │
//!                         ┌──────────┴──────────┐
//!                         │                     │
//!                         ▼                     ▼
//!                  width analysis          liveness analysis
//!                         │                     │
//!                         └──────────┬──────────┘
//!                                    ▼
//!                         width optimization policy
//!                                    │
//!                    ┌───────────────┴────────────────┐
//!                    │                                │
//!                    ▼                                ▼
//!              safe reduction                    no reduction
//!                    │                                │
//!                    ▼                                ▼
//!             canonical IR                   unchanged IR
//! ```
//!
//! # Purpose
//!
//! This pass owns the **policy for reducing logical resource width**.
//!
//! It does NOT own:
//!
//! - the canonical Quantum IR;
//! - qubit identifiers;
//! - qubit allocation;
//! - qubit lifetime semantics;
//! - measurement semantics;
//! - routing;
//! - physical qubit topology;
//! - scheduling;
//! - hardware execution;
//! - QPU communication;
//! - error correction;
//! - quantum algorithms;
//! - benchmarking;
//! - state-vector simulation.
//!
//! Those responsibilities remain in their owning subsystems.
//!
//! # Critical semantic rule
//!
//! A declared qubit is not automatically removable merely because it is unused
//! by the operation list.
//!
//! The declared logical namespace can be semantically observable through:
//!
//! - terminal measurements;
//! - state/probability measurements over all wires;
//! - classical result schemas;
//! - external circuit interfaces;
//! - future dynamic allocation semantics;
//! - logical-to-physical mapping contracts;
//! - debugging/provenance information;
//! - serialized IR;
//! - API contracts that expose the declared width.
//!
//! Consequently, this pass NEVER changes `QuantumCircuit::num_qubits()` using
//! assumptions that are not represented by the canonical IR contract.
//!
//! The current canonical `QuantumCircuit` API exposes safe operation editing,
//! but does not expose a public, transactional logical-namespace-remapping
//! primitive. Therefore this implementation performs production-grade width
//! analysis and optimization decision-making without pretending that unused
//! namespace entries can already be deleted safely.
//!
//! This is intentional.
//!
//! A compiler pass that silently changes observable qubit namespace semantics
//! is not production-ready.
//!
//! # Current optimization behavior
//!
//! The pass:
//!
//! 1. validates the canonical input;
//! 2. checks cancellation/resource state;
//! 3. performs sparse width analysis;
//! 4. calculates width-reduction opportunities;
//! 5. records deterministic statistics;
//! 6. leaves the canonical circuit unchanged;
//! 7. returns `PassOutcome::unchanged(...)`.
//!
//! The resulting API is already suitable for:
//!
//! - planner decisions;
//! - cost models;
//! - diagnostics;
//! - benchmarking consumers;
//! - future namespace-remapping support;
//! - target-aware width planning;
//! - liveness-aware optimization;
//! - ancilla reuse planning.
//!
//! No later implementation file needs to modify this pass merely because those
//! consumers are added.
//!
//! # Why this is preferable to fake compaction
//!
//! Suppose a circuit declares 1,000 qubits but currently contains operations
//! using only qubits 0 and 1.
//!
//! It may be tempting to turn:
//!
//! ```text
//! width = 1000
//! ```
//!
//! into:
//!
//! ```text
//! width = 2
//! ```
//!
//! However, doing so requires changing the canonical logical namespace and
//! potentially changing the meaning of operations such as:
//!
//! ```text
//! measure(all)
//! state()
//! probs()
//! sample()
//! ```
//!
//! It also requires a deterministic logical-qubit mapping:
//!
//! ```text
//! old q17 → new q0
//! old q42 → new q1
//! ```
//!
//! and preservation of every semantic reference to those qubits.
//!
//! The current canonical IR deliberately owns the logical namespace and does
//! not yet expose that transformation as a safe public operation.
//!
//! Therefore the optimizer reports the opportunity instead of corrupting the
//! program.
//!
//! # Width terminology
//!
//! This pass deliberately distinguishes:
//!
//! ```text
//! declared width
//!     = QuantumCircuit::num_qubits()
//!
//! used width
//!     = number of distinct logical qubits occurring in operations
//!
//! unused width
//!     = declared width - used width
//!
//! peak operand width
//!     = maximum number of operands used by one operation
//!
//! peak use-span width
//!     = conservative overlap of first/last-use intervals
//! ```
//!
//! `peak_use_span_width` is NOT exact qubit liveness.
//!
//! Exact allocation/reuse semantics involving measurement, reset, dynamic
//! allocation, regions, loops and control flow belong to the dedicated liveness
//! and structural analyses.
//!
//! # Scaling
//!
//! Let:
//!
//! - `N` = operation count;
//! - `A` = total qubit operand count;
//! - `K` = number of distinct used qubits.
//!
//! The underlying width analysis is approximately:
//!
//! ```text
//! O(N + A + K log K)
//! ```
//!
//! with auxiliary memory proportional to used qubits rather than declared
//! namespace size.
//!
//! Therefore a sparse circuit such as:
//!
//! ```text
//! declared qubits: 1,000,000,000
//! used qubits:     17
//! ```
//!
//! does not require a billion-element used-qubit structure.
//!
//! This pass introduces no artificial circuit-size ceiling.
//!
//! Actual scaling is controlled by:
//!
//! - canonical IR limits;
//! - OptimizationLimits;
//! - OptimizationContext;
//! - host address space;
//! - available memory;
//! - available CPU;
//! - caller-selected resource policy.
//!
//! "Infinity" therefore means that this pass does not impose a fixed maximum
//! independent of available resources.
//!
//! # Determinism
//!
//! The pass is deterministic:
//!
//! - no random state;
//! - no global mutable state;
//! - no thread creation;
//! - deterministic analysis;
//! - deterministic statistics;
//! - deterministic configuration;
//! - deterministic pass metadata.
//!
//! Parallel execution belongs to `optimization::scheduler`.
//!
//! # Transactionality
//!
//! The pass currently does not mutate the circuit at all.
//!
//! This gives the strongest possible transaction guarantee:
//!
//! ```text
//! input circuit
//!      │
//!      ▼
//! validation
//!      │
//!      ▼
//! width analysis
//!      │
//!      ▼
//! optimization decision
//!      │
//!      ▼
//! unchanged canonical circuit
//! ```
//!
//! Future namespace compaction must be implemented through the canonical IR's
//! transactional editing/remapping contract before this pass is allowed to
//! change declared width.
//!
//! # Integration contract
//!
//! ## `quantum::ir`
//!
//! Input:
//!
//! ```text
//! crate::quantum::ir::QuantumCircuit
//! ```
//!
//! No optimizer-specific Quantum IR is introduced.
//!
//! ## `analysis::width`
//!
//! This pass consumes:
//!
//! ```text
//! analyze_width_with_config(...)
//! WidthAnalysis
//! WidthSummary
//! ```
//!
//! The analysis remains authoritative for width metrics.
//!
//! ## `analysis::liveness`
//!
//! Future width-reduction decisions that depend on exact reuse/release
//! semantics must consume the liveness analysis rather than inferring exact
//! liveness from `WidthAnalysis::peak_use_span_width()`.
//!
//! This file is already structured so liveness can be added as a prerequisite
//! without changing the public pass API.
//!
//! ## `analysis::qubit_use`
//!
//! Detailed per-qubit usage can be consumed by future planners and namespace
//! remappers. This pass deliberately does not duplicate that analysis.
//!
//! ## `context`
//!
//! Invocation state remains in `OptimizationContext`.
//!
//! The pass never owns global mutable optimization state.
//!
//! ## `limits`
//!
//! Width analysis uses the sparse analysis configuration and therefore does not
//! allocate according to the declared logical namespace merely to report
//! unused qubits.
//!
//! The canonical IR and optimization context remain authoritative for resource
//! limits.
//!
//! ## `pass`
//!
//! This file implements `OptimizationPass`.
//!
//! It declares itself as an analysis pass because, under the current canonical
//! IR contract, it cannot safely mutate the logical namespace.
//!
//! It declares:
//!
//! - circuit scope;
//! - deterministic execution;
//! - semantic preservation;
//! - large-circuit support;
//! - analysis-only behavior.
//!
//! ## `planner`
//!
//! The planner should use this pass when:
//!
//! - optimization objectives include width;
//! - the target has a limited logical/physical qubit budget;
//! - ancilla reduction is relevant;
//! - sparse logical namespaces are present;
//! - width diagnostics are requested;
//! - a future namespace-remapping capability is available.
//!
//! The pass does not inspect `OptimizationProfile` itself.
//!
//! ## `cost`
//!
//! Width cost models should consume the resulting `WidthSummary`/statistics.
//!
//! This pass does not invent a second cost model.
//!
//! ## `routing`
//!
//! Routing remains responsible for logical-to-physical mapping.
//!
//! This pass must not use physical topology as a reason to alter logical
//! namespace semantics.
//!
//! ## `scheduling`
//!
//! Scheduling owns temporal resource usage.
//!
//! This pass may provide width metrics to the scheduler indirectly through
//! optimizer results, but never schedules operations itself.
//!
//! ## `verification`
//!
//! Width analysis is exact with respect to the canonical IR metrics.
//!
//! If future namespace compaction is implemented, semantic equivalence must be
//! verified by the verification subsystem before the transformed circuit is
//! accepted.
//!
//! ## `benchmarking`
//!
//! Benchmarking may consume the before/after width metrics.
//!
//! This pass does not depend on benchmarking.
//!
//! ## `serialization` / `provenance`
//!
//! `PASS_ID` and `PASS_VERSION` are stable identifiers suitable for provenance.
//!
//! # Future namespace-remapping integration
//!
//! The pass deliberately anticipates the required future operation:
//!
//! ```text
//! LogicalQubitMap
//!     old QubitId → new QubitId
//! ```
//!
//! A production namespace compaction implementation will need to prove all of:
//!
//! 1. every referenced qubit has a mapping;
//! 2. mappings are injective;
//! 3. mapping is deterministic;
//! 4. every gate operand is remapped;
//! 5. measurement references are remapped;
//! 6. classical dependencies are preserved;
//! 7. control-flow references are preserved;
//! 8. metadata references are preserved where semantically required;
//! 9. circuit identity/provenance policy is preserved;
//! 10. declared width is changed atomically;
//! 11. canonical IR validation succeeds;
//! 12. configured semantic equivalence policy succeeds.
//!
//! This file does not implement a private version of that contract.
//!
//! # Rust compatibility
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust only;
//! - no nightly features;
//! - no external dependencies;
//! - no unsafe code.
//!
//! # Safety
//!
//! ```text
//! #![forbid(unsafe_code)]
//! ```
//!
//! No unsafe code is required or permitted.

#![forbid(unsafe_code)]

use crate::quantum::ir::QuantumCircuit;

use super::super::analysis::width::{
    analyze_width_with_config,
    WidthAnalysis,
    WidthAnalysisConfig,
};
use super::super::context::OptimizationContext;
use super::super::errors::{
    OptimizationError,
    OptimizationStage,
    PassIdentifier,
};
use super::super::pass::{
    OptimizationPass,
    PassCapability,
    PassComplexity,
    PassDeterminism,
    PassExecutionPolicy,
    PassKind,
    PassMetadata,
    PassMetadataError,
    PassOutcome,
    PassScope,
};

/// Stable machine-readable identifier.
///
/// This identifier is persisted in optimizer provenance and therefore must not
/// be changed merely because the Rust implementation is refactored.
pub const PASS_ID: &str = "passes.optimize_width";

/// Stable human-readable pass name.
pub const PASS_NAME: &str = "Logical Circuit Width Optimization";

/// Public behavior/schema version of this pass.
pub const PASS_VERSION: u32 = 1;

/// Stable identifier for the width analysis consumed by this pass.
pub const WIDTH_ANALYSIS_ID: &str = "optimization.analysis.width";

/// Width optimization configuration.
///
/// The current canonical IR does not expose a safe logical-namespace-remapping
/// primitive. Consequently the pass performs width analysis and decision
/// reporting without mutating declared width.
///
/// The configuration is intentionally small and stable. Width-analysis memory
/// policy belongs to `WidthAnalysisConfig`, while optimizer execution policy
/// belongs to `OptimizationContext`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WidthOptimizationConfig {
    /// Controls how many unused qubit identifiers the analysis may materialize.
    ///
    /// This does not limit circuit size. It only limits optional diagnostic
    /// output allocation.
    pub max_materialized_unused_qubits: usize,

    /// Whether operation positions attaining peak operand width are collected.
    pub collect_peak_operations: bool,
}

impl WidthOptimizationConfig {
    /// Production configuration.
    ///
    /// The one-million diagnostic-ID ceiling prevents pathological diagnostic
    /// allocations while preserving the ability to inspect ordinary large
    /// circuits.
    #[must_use]
    pub const fn production() -> Self {
        Self {
            max_materialized_unused_qubits: 1_000_000,
            collect_peak_operations: false,
        }
    }

    /// Sparse configuration for very large compiler workloads.
    #[must_use]
    pub const fn sparse() -> Self {
        Self {
            max_materialized_unused_qubits: 0,
            collect_peak_operations: false,
        }
    }

    /// Diagnostic/exhaustive configuration.
    ///
    /// This should only be used where the caller explicitly accepts the memory
    /// required to materialize all unused qubit identifiers.
    #[must_use]
    pub const fn exhaustive() -> Self {
        Self {
            max_materialized_unused_qubits: usize::MAX,
            collect_peak_operations: true,
        }
    }

    /// Creates a production configuration.
    #[must_use]
    pub const fn new() -> Self {
        Self::production()
    }

    /// Sets the optional unused-qubit materialization budget.
    #[must_use]
    pub const fn with_max_materialized_unused_qubits(
        mut self,
        maximum: usize,
    ) -> Self {
        self.max_materialized_unused_qubits = maximum;
        self
    }

    /// Enables or disables collection of peak operation positions.
    #[must_use]
    pub const fn with_peak_operations(
        mut self,
        enabled: bool,
    ) -> Self {
        self.collect_peak_operations = enabled;
        self
    }

    fn analysis_config(self) -> WidthAnalysisConfig {
        WidthAnalysisConfig {
            max_materialized_unused_qubits:
                self.max_materialized_unused_qubits,
            collect_peak_operations:
                self.collect_peak_operations,
        }
    }
}

impl Default for WidthOptimizationConfig {
    fn default() -> Self {
        Self::production()
    }
}

/// Immutable statistics produced by one width optimization invocation.
///
/// The pass is intentionally analysis-only under the current canonical IR
/// contract, so `width_after == width_before`.
///
/// `candidate_reduction` describes an opportunity that would exist if and only
/// if a future semantic namespace-remapping operation proves that those
/// declared qubits are removable.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WidthOptimizationStatistics {
    /// Declared logical width before the pass.
    pub width_before: usize,

    /// Declared logical width after the pass.
    ///
    /// Currently equal to `width_before` because the canonical IR does not yet
    /// expose a safe namespace-remapping mutation.
    pub width_after: usize,

    /// Number of distinct logical qubits used by operations.
    pub used_width: usize,

    /// Number of declared logical qubits not appearing in operations.
    pub unused_width: usize,

    /// Maximum operand width of one operation.
    pub peak_operand_width: usize,

    /// Conservative peak first/last-use span width.
    pub peak_use_span_width: usize,

    /// Number of operations.
    pub operation_count: usize,

    /// Number of qubit operand occurrences.
    pub total_operand_uses: usize,

    /// Maximum operation arity.
    pub maximum_operation_arity: usize,

    /// Whether namespace compaction appears potentially beneficial.
    ///
    /// This is an opportunity indicator, not proof that compaction is
    /// semantically legal.
    pub compaction_opportunity: bool,
}

impl WidthOptimizationStatistics {
    /// Returns true when the pass actually changed declared width.
    ///
    /// Under the current IR contract this is always false.
    #[must_use]
    pub const fn changed(self) -> bool {
        self.width_after < self.width_before
    }

    /// Returns the actual declared-width reduction.
    #[must_use]
    pub const fn width_reduction(self) -> usize {
        self.width_before - self.width_after
    }

    /// Returns the number of currently unused logical namespace entries.
    #[must_use]
    pub const fn removable_namespace_candidates(self) -> usize {
        self.unused_width
    }
}

/// Production logical-width optimization pass.
///
/// The pass is stateless. Invocation-specific state is held by
/// `OptimizationContext`.
#[derive(Debug, Clone)]
pub struct OptimizeWidth {
    metadata: PassMetadata,
    config: WidthOptimizationConfig,
}

impl OptimizeWidth {
    /// Constructs the production width optimizer.
    pub fn new() -> Result<Self, PassMetadataError> {
        Self::with_config(WidthOptimizationConfig::default())
    }

    /// Constructs the pass with explicit width-analysis configuration.
    pub fn with_config(
        config: WidthOptimizationConfig,
    ) -> Result<Self, PassMetadataError> {
        let identifier = PassIdentifier::from_static(PASS_ID)?;

        let metadata = PassMetadata::new(
            identifier,
            PASS_NAME,
            PassKind::Analysis,
        )?
        .with_description(
            "Analyzes logical circuit width and determines safe width \
             optimization opportunities without changing canonical logical \
             namespace semantics.",
        )?
        .with_scope(PassScope::Circuit)
        .with_complexity(PassComplexity::Linearithmic)
        .with_determinism(PassDeterminism::Deterministic)
        .with_capabilities([
            PassCapability::AnalysisOnly,
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
    #[must_use]
    pub const fn pass_id() -> &'static str {
        PASS_ID
    }

    /// Returns the stable pass name.
    #[must_use]
    pub const fn pass_name() -> &'static str {
        PASS_NAME
    }

    /// Returns the pass behavior version.
    #[must_use]
    pub const fn pass_version() -> u32 {
        PASS_VERSION
    }

    /// Returns the stable width-analysis identifier.
    #[must_use]
    pub const fn width_analysis_id() -> &'static str {
        WIDTH_ANALYSIS_ID
    }

    /// Returns the active configuration.
    #[must_use]
    pub const fn config(&self) -> WidthOptimizationConfig {
        self.config
    }

    /// Performs width analysis without mutating the circuit.
    ///
    /// This method is useful to planners, diagnostics, cost models and tests
    /// that need the complete immutable analysis object rather than the coarse
    /// `PassOutcome`.
    pub fn analyze(
        &self,
        circuit: &QuantumCircuit,
    ) -> Result<WidthAnalysis, OptimizationError> {
        circuit.validate().map_err(|error| {
            OptimizationError::invalid_input(
                OptimizationStage::InputValidation,
                format!(
                    "{PASS_ID}: input Quantum IR validation failed: {error}"
                ),
            )
        })?;

        analyze_width_with_config(
            circuit,
            self.config.analysis_config(),
        )
        .map_err(|error| {
            OptimizationError::internal(
                OptimizationStage::Analysis,
                format!(
                    "{PASS_ID}: logical width analysis failed: {error}"
                ),
            )
        })
    }

    /// Calculates stable width optimization statistics.
    pub fn statistics(
        &self,
        circuit: &QuantumCircuit,
    ) -> Result<WidthOptimizationStatistics, OptimizationError> {
        let analysis = self.analyze(circuit)?;

        Ok(Self::statistics_from_analysis(&analysis))
    }

    /// Converts an immutable width analysis into pass statistics.
    fn statistics_from_analysis(
        analysis: &WidthAnalysis,
    ) -> WidthOptimizationStatistics {
        WidthOptimizationStatistics {
            width_before: analysis.width(),
            width_after: analysis.width(),
            used_width: analysis.used_width(),
            unused_width: analysis.unused_width(),
            peak_operand_width: analysis.peak_operand_width(),
            peak_use_span_width:
                analysis.peak_use_span_width(),
            operation_count: analysis.operation_count(),
            total_operand_uses:
                analysis.total_operand_uses(),
            maximum_operation_arity:
                analysis.maximum_operation_arity(),
            compaction_opportunity:
                analysis.unused_width() != 0,
        }
    }

    /// Executes the pass using a standalone optimizer context.
    ///
    /// Production compiler pipelines should normally call `run()` so the
    /// caller's shared cancellation and resource policy is respected.
    pub fn optimize(
        &self,
        circuit: &mut QuantumCircuit,
    ) -> Result<PassOutcome, OptimizationError> {
        let mut context = OptimizationContext::standalone();

        self.run(circuit, &mut context)
    }

    /// Executes the width analysis and returns its statistics.
    ///
    /// The circuit is deliberately not mutated.
    fn run_impl(
        &self,
        circuit: &QuantumCircuit,
        context: &mut OptimizationContext,
    ) -> Result<
        (
            PassOutcome,
            WidthOptimizationStatistics,
        ),
        OptimizationError,
    > {
        circuit.validate().map_err(|error| {
            OptimizationError::invalid_input(
                OptimizationStage::InputValidation,
                format!(
                    "{PASS_ID}: input Quantum IR validation failed: {error}"
                ),
            )
        })?;

        context.check_cancelled().map_err(|error| {
            OptimizationError::resource_limit(
                OptimizationStage::Analysis,
                format!(
                    "{PASS_ID}: width optimization cancelled before \
                     analysis: {error}"
                ),
            )
        })?;

        let operations_before =
            u64::try_from(circuit.len()).map_err(|_| {
                OptimizationError::internal(
                    OptimizationStage::Analysis,
                    format!(
                        "{PASS_ID}: operation count cannot be represented \
                         by optimizer accounting"
                    ),
                )
            })?;

        let analysis = self.analyze(circuit)?;

        context.check_cancelled().map_err(|error| {
            OptimizationError::resource_limit(
                OptimizationStage::Analysis,
                format!(
                    "{PASS_ID}: width optimization cancelled after \
                     analysis: {error}"
                ),
            )
        })?;

        let statistics =
            Self::statistics_from_analysis(&analysis);

        // ---------------------------------------------------------------------
        // IMPORTANT:
        //
        // Do not mutate `circuit` here.
        //
        // The current canonical QuantumCircuit API provides safe transactional
        // operation editing but does not provide a semantic logical-namespace
        // remapping primitive. Rebuilding the circuit through
        // `QuantumCircuit::from_operations` would lose canonical circuit
        // metadata/identity/resource-policy semantics and could change the
        // meaning of all-wire measurements.
        //
        // Therefore the correct production behavior is to expose the
        // opportunity and leave the canonical circuit unchanged.
        // ---------------------------------------------------------------------

        let outcome =
            PassOutcome::unchanged(
                operations_before,
                operations_before,
            );

        Ok((outcome, statistics))
    }
}

impl OptimizationPass for OptimizeWidth {
    fn metadata(&self) -> &PassMetadata {
        &self.metadata
    }

    fn run(
        &self,
        circuit: &mut QuantumCircuit,
        context: &mut OptimizationContext,
    ) -> Result<PassOutcome, OptimizationError> {
        let (outcome, _) =
            self.run_impl(circuit, context)?;

        Ok(outcome)
    }

    fn execution_policy(&self) -> PassExecutionPolicy {
        PassExecutionPolicy::StopWhenStable
    }
}

impl Default for OptimizeWidth {
    fn default() -> Self {
        Self::new()
            .expect(
                "production width optimization metadata \
                 must be valid",
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::quantum::ir::gate::{
        Gate,
        GateKind,
    };
    use crate::quantum::ir::qubits::QubitId;

    fn x(qubit: usize) -> Gate {
        Gate::new(
            GateKind::X,
            vec![QubitId::new(qubit)],
            Vec::new(),
            None,
            None,
        )
        .expect("X gate should be valid")
    }

    fn cx(
        control: usize,
        target: usize,
    ) -> Gate {
        Gate::new(
            GateKind::CX,
            vec![
                QubitId::new(control),
                QubitId::new(target),
            ],
            Vec::new(),
            None,
            None,
        )
        .expect("CX gate should be valid")
    }

    #[test]
    fn production_configuration_is_sparse() {
        let config =
            WidthOptimizationConfig::production();

        assert_eq!(
            config.max_materialized_unused_qubits,
            1_000_000
        );

        assert!(!config.collect_peak_operations);
    }

    #[test]
    fn sparse_configuration_never_materializes_unused_qubits() {
        let config =
            WidthOptimizationConfig::sparse();

        assert_eq!(
            config.max_materialized_unused_qubits,
            0
        );

        assert!(!config.collect_peak_operations);
    }

    #[test]
    fn exhaustive_configuration_allows_materialization() {
        let config =
            WidthOptimizationConfig::exhaustive();

        assert_eq!(
            config.max_materialized_unused_qubits,
            usize::MAX
        );

        assert!(config.collect_peak_operations);
    }

    #[test]
    fn statistics_report_unused_namespace_without_claiming_reduction() {
        let circuit =
            QuantumCircuit::from_operations(
                8,
                0,
                vec![
                    x(0),
                    cx(0, 1),
                    x(1),
                ],
            )
            .expect("circuit should be valid");

        let pass =
            OptimizeWidth::default();

        let statistics =
            pass.statistics(&circuit)
                .expect("width analysis should succeed");

        assert_eq!(
            statistics.width_before,
            8
        );

        assert_eq!(
            statistics.width_after,
            8
        );

        assert_eq!(
            statistics.used_width,
            2
        );

        assert_eq!(
            statistics.unused_width,
            6
        );

        assert!(
            statistics.compaction_opportunity
        );

        assert!(!statistics.changed());

        assert_eq!(
            statistics.width_reduction(),
            0
        );

        assert_eq!(
            statistics.removable_namespace_candidates(),
            6
        );
    }

    #[test]
    fn empty_circuit_is_supported() {
        let circuit =
            QuantumCircuit::from_operations(
                0,
                0,
                Vec::new(),
            )
            .expect("empty circuit should be valid");

        let pass =
            OptimizeWidth::default();

        let statistics =
            pass.statistics(&circuit)
                .expect("empty circuit should analyze");

        assert_eq!(
            statistics.width_before,
            0
        );

        assert_eq!(
            statistics.used_width,
            0
        );

        assert_eq!(
            statistics.unused_width,
            0
        );

        assert_eq!(
            statistics.operation_count,
            0
        );

        assert!(
            !statistics.compaction_opportunity
        );
    }

    #[test]
    fn width_analysis_does_not_mutate_the_circuit() {
        let circuit =
            QuantumCircuit::from_operations(
                4,
                0,
                vec![
                    x(0),
                    cx(0, 2),
                ],
            )
            .expect("circuit should be valid");

        let before = circuit.clone();

        let pass =
            OptimizeWidth::default();

        let mut candidate =
            circuit.clone();

        let mut context =
            OptimizationContext::standalone();

        let outcome =
            pass.run(
                &mut candidate,
                &mut context,
            )
            .expect("width pass should succeed");

        assert_eq!(
            candidate,
            before
        );

        assert!(
            !outcome.changed()
        );
    }

    #[test]
    fn sparse_analysis_scales_with_used_qubits_not_declared_namespace() {
        let circuit =
            QuantumCircuit::from_operations(
                1_000_000,
                0,
                vec![
                    x(0),
                    x(999_999),
                ],
            )
            .expect("sparse circuit should be valid");

        let pass =
            OptimizeWidth::with_config(
                WidthOptimizationConfig::sparse(),
            )
            .expect("metadata should be valid");

        let analysis =
            pass.analyze(&circuit)
                .expect("analysis should succeed");

        assert_eq!(
            analysis.width(),
            1_000_000
        );

        assert_eq!(
            analysis.used_width(),
            2
        );

        assert_eq!(
            analysis.unused_width(),
            999_998
        );

        assert!(
            analysis.unused_qubits().is_none()
        );
    }

    #[test]
    fn production_analysis_is_deterministic() {
        let circuit =
            QuantumCircuit::from_operations(
                16,
                0,
                vec![
                    x(3),
                    cx(3, 7),
                    x(7),
                    cx(7, 11),
                ],
            )
            .expect("circuit should be valid");

        let pass =
            OptimizeWidth::default();

        let first =
            pass.statistics(&circuit)
                .expect("first analysis");

        let second =
            pass.statistics(&circuit)
                .expect("second analysis");

        assert_eq!(
            first,
            second
        );
    }

    #[test]
    fn pass_identity_is_stable() {
        assert_eq!(
            OptimizeWidth::pass_id(),
            "passes.optimize_width"
        );

        assert_eq!(
            OptimizeWidth::pass_name(),
            "Logical Circuit Width Optimization"
        );

        assert_eq!(
            OptimizeWidth::pass_version(),
            1
        );

        assert_eq!(
            OptimizeWidth::width_analysis_id(),
            "optimization.analysis.width"
        );
    }
}