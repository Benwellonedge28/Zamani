//! Zamani Quantum Optimization — Entanglement / Quantum-Interaction Analysis
//!
//! Production-grade, conservative, read-only analysis of potential quantum
//! interaction connectivity over the canonical Zamani Quantum IR.
//!
//! # Architectural position
//!
//! ```text
//! quantum::ir::QuantumCircuit
//!          │
//!          ▼
//! optimization::analysis::entanglement
//!          │
//!          ├── potential quantum-interaction components
//!          ├── conservative separability information
//!          ├── multi-qubit interaction discovery
//!          ├── bounded / streaming analysis
//!          └── deterministic results
//!          │
//!          ▼
//! optimization passes
//! ```
//!
//! # Important semantic distinction
//!
//! This module deliberately does NOT claim to prove that two qubits are
//! physically entangled.
//!
//! Entanglement is a property of a quantum state, not merely of a circuit's
//! syntactic gate connectivity. For example:
//!
//! ```text
//! CX(q0, q1)
//! ```
//!
//! does not by itself prove that q0 and q1 are entangled for every possible
//! input state.
//!
//! Instead, this module computes a conservative static property:
//!
//! > "These qubits may have become quantum-mechanically correlated through the
//! > operations represented by the circuit."
//!
//! This is called a `PotentialEntanglementComponent` in this API.
//!
//! The analysis is intentionally conservative. A false positive is preferable
//! to a false negative when the result is used by an optimizer to decide
//! whether a transformation may assume qubit independence.
//!
//! # Why this belongs in optimization::analysis
//!
//! The analysis is useful for:
//!
//! - block formation;
//! - region formation;
//! - commutation-aware optimization;
//! - tensor-network partitioning;
//! - width optimization;
//! - synthesis boundaries;
//! - circuit cutting decisions;
//! - separability-sensitive transformations;
//! - parallel optimization planning;
//! - avoiding unsafe transformations across interacting qubits;
//! - estimating the maximum interaction width of a circuit.
//!
//! It does NOT:
//!
//! - simulate a quantum state;
//! - calculate a density matrix;
//! - calculate von Neumann entropy;
//! - calculate concurrence;
//! - calculate negativity;
//! - execute a circuit;
//! - communicate with a QPU;
//! - perform routing;
//! - schedule hardware operations;
//! - modify the circuit;
//! - claim physical entanglement.
//!
//! State-based entanglement measures belong in simulation/analysis layers that
//! actually have access to a quantum state.
//!
//! # Canonical representation
//!
//! The canonical representation remains:
//!
//! ```text
//! crate::quantum::ir::QuantumCircuit
//! crate::quantum::ir::Gate
//! crate::quantum::ir::QubitId
//! ```
//!
//! This file deliberately defines no replacement circuit, gate, or qubit
//! representation.
//!
//! # Algorithm
//!
//! The core algorithm is a disjoint-set / union-find structure over logical
//! qubits.
//!
//! Every operation is inspected exactly once.
//!
//! For an operation touching:
//!
//! ```text
//! q0, q1, q2, ..., qn
//! ```
//!
//! the operation creates potential quantum interaction between those operands,
//! so their components are merged.
//!
//! Single-qubit operations do not merge components.
//!
//! Measurement and reset require special care:
//!
//! - measurement is a semantic boundary for quantum evolution of the measured
//!   qubit, but it does not justify assuming the remaining system is separable;
//! - reset definitely changes the state of its target, but the conservative
//!   optimizer-facing analysis does not use reset to claim that previously
//!   interacting qubits are independent;
//! - later multi-qubit operations can always reconnect components.
//!
//! Therefore the default analysis is intentionally monotonic with respect to
//! potential interaction connectivity.
//!
//! This avoids unsound conclusions such as:
//!
//! ```text
//! CX(q0,q1)
//! measure(q0)
//!
//! => "q0 and q1 are definitely independent"
//! ```
//!
//! which is not generally safe for compiler reasoning about the surrounding
//! program.
//!
//! # Complexity
//!
//! Let:
//!
//! - Q = number of logical qubits;
//! - N = number of operations;
//! - A = total number of qubit operands across all operations;
//! - C = number of resulting potential-interaction components.
//!
//! Core analysis:
//!
//! ```text
//! Time:   O(Q + A α(Q))
//! Memory: O(Q)
//! ```
//!
//! where α is the inverse Ackermann function.
//!
//! There is deliberately no `O(N²)` pair generation in the core analysis.
//!
//! For callers that need component members, the implementation can produce
//! deterministic component information without requiring a hash map.
//!
//! For extremely large circuits, callers should prefer the streaming summary
//! APIs rather than materializing every interaction pair.
//!
//! `usize::MAX` is treated as a valid capacity value where the underlying IR
//! itself permits it; this module never attempts to allocate an infinite
//! structure.
//!
//! # Determinism
//!
//! The analysis uses indexed vectors and canonical logical-qubit order.
//!
//! No `HashMap` or `HashSet` iteration order is exposed as compiler-visible
//! ordering.
//!
//! Component identifiers are deterministic for a given logical-qubit
//! namespace and circuit.
//!
//! # Safety
//!
//! This module forbids unsafe Rust.
//!
//! # Rust compatibility
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust only;
//! - no external dependencies.
//!
//! # Integration contract
//!
//! `analysis/mod.rs` should declare:
//!
//! ```text
//! pub mod entanglement;
//! ```
//!
//! and may re-export:
//!
//! ```text
//! pub use entanglement::{
//!     EntanglementAnalysis,
//!     EntanglementAnalysisConfig,
//!     EntanglementAnalysisError,
//!     EntanglementComponent,
//!     EntanglementSummary,
//!     QubitEntanglementInfo,
//!     PotentialEntanglement,
//! };
//! ```
//!
//! `context.rs` may cache the resulting immutable analysis object.
//!
//! Any circuit mutation that changes:
//!
//! - operations;
//! - operands;
//! - operation ordering;
//! - control dependencies;
//! - measurement/reset semantics;
//! - qubit namespace
//!
//! MUST invalidate the cached result.
//!
//! `dependency.rs`, `commutation.rs`, `liveness.rs`, `width.rs` and
//! `critical_path.rs` may consume this analysis, but this module must not
//! depend on those analyses.
//!
//! This keeps dependency direction acyclic:
//!
//! ```text
//! Quantum IR
//!     │
//!     ▼
//! entanglement analysis
//!     │
//!     ├── optimization passes
//!     ├── block analysis
//!     ├── width optimization
//!     └── circuit partitioning
//! ```
//!
//! No optimizer, router, scheduler, backend, or simulator is called from this
//! module.

#![forbid(unsafe_code)]

use std::fmt;

use crate::quantum::ir::{Gate, QuantumCircuit, QubitId};

// ============================================================================
// Public semantic classification
// ============================================================================

/// Conservative static relationship between two logical qubits.
///
/// This is deliberately weaker than a claim of physical entanglement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PotentialEntanglement {
    /// The qubits are currently in different potential-interaction components.
    ///
    /// This means this analysis has not observed a multi-qubit operation that
    /// connected them in the analyzed circuit.
    Separate,

    /// The qubits belong to the same potential-interaction component.
    ///
    /// This does NOT prove that their runtime quantum state is entangled.
    ///
    /// It means the optimizer must not assume that they are independent solely
    /// from the circuit connectivity represented by this analysis.
    PotentiallyCorrelated,
}

impl PotentialEntanglement {
    /// Returns true when the analysis permits a separability assumption.
    #[must_use]
    pub const fn permits_separability(self) -> bool {
        matches!(self, Self::Separate)
    }

    /// Returns true when the analysis requires conservative correlated-state
    /// handling.
    #[must_use]
    pub const fn may_be_correlated(self) -> bool {
        matches!(self, Self::PotentiallyCorrelated)
    }
}

// ============================================================================
// Errors
// ============================================================================

/// Errors produced by entanglement/interaction analysis.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntanglementAnalysisError {
    /// The supplied circuit could not be validated.
    InvalidCircuit {
        /// Canonical validation message.
        message: String,
    },

    /// A logical qubit cannot be represented by the circuit namespace.
    InvalidQubit {
        /// Logical qubit index.
        qubit: usize,

        /// Circuit logical-qubit count.
        qubit_count: usize,
    },

    /// A checked arithmetic operation overflowed.
    ArithmeticOverflow {
        /// Calculation that overflowed.
        calculation: &'static str,
    },

    /// The requested materialized result exceeds the configured result limit.
    ResultLimitExceeded {
        /// Requested number of retained entries.
        requested: usize,

        /// Maximum permitted entries.
        maximum: usize,
    },
}

impl fmt::Display for EntanglementAnalysisError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidCircuit { message } => {
                write!(
                    formatter,
                    "invalid quantum circuit: {message}"
                )
            }

            Self::InvalidQubit {
                qubit,
                qubit_count,
            } => {
                write!(
                    formatter,
                    "logical qubit {qubit} is outside circuit namespace of \
                     {qubit_count} qubits"
                )
            }

            Self::ArithmeticOverflow { calculation } => {
                write!(
                    formatter,
                    "arithmetic overflow while calculating {calculation}"
                )
            }

            Self::ResultLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "entanglement analysis result limit exceeded: \
                     requested {requested}, maximum {maximum}"
                )
            }
        }
    }
}

impl std::error::Error for EntanglementAnalysisError {}

// ============================================================================
// Configuration
// ============================================================================

/// Configuration controlling entanglement/interaction analysis.
///
/// The default configuration performs the complete connectivity analysis while
/// avoiding pairwise materialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EntanglementAnalysisConfig {
    /// Maximum number of qubits for which a fully materialized per-qubit result
    /// may be returned.
    ///
    /// This protects callers from accidentally requesting enormous vectors.
    ///
    /// The core analysis itself is not subject to this limit; callers can use
    /// the streaming/summary APIs for larger circuits.
    max_materialized_qubits: usize,

    /// Maximum number of components that may be materialized in a result.
    ///
    /// This is a caller-facing memory protection mechanism, not a restriction
    /// on the circuit itself.
    max_materialized_components: usize,
}

impl Default for EntanglementAnalysisConfig {
    fn default() -> Self {
        Self {
            max_materialized_qubits: 1_000_000,
            max_materialized_components: 1_000_000,
        }
    }
}

impl EntanglementAnalysisConfig {
    /// Creates the production default configuration.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            max_materialized_qubits: 1_000_000,
            max_materialized_components: 1_000_000,
        }
    }

    /// Creates a configuration without an artificial materialization ceiling.
    ///
    /// This does not cause unbounded allocation by itself. The caller still
    /// chooses whether to request a materialized result.
    #[must_use]
    pub const fn unlimited_materialization() -> Self {
        Self {
            max_materialized_qubits: usize::MAX,
            max_materialized_components: usize::MAX,
        }
    }

    /// Sets the maximum number of materialized qubits.
    #[must_use]
    pub const fn with_max_materialized_qubits(
        mut self,
        maximum: usize,
    ) -> Self {
        self.max_materialized_qubits = maximum;
        self
    }

    /// Sets the maximum number of materialized components.
    #[must_use]
    pub const fn with_max_materialized_components(
        mut self,
        maximum: usize,
    ) -> Self {
        self.max_materialized_components = maximum;
        self
    }

    /// Returns the maximum materialized qubit count.
    #[must_use]
    pub const fn max_materialized_qubits(self) -> usize {
        self.max_materialized_qubits
    }

    /// Returns the maximum materialized component count.
    #[must_use]
    pub const fn max_materialized_components(self) -> usize {
        self.max_materialized_components
    }
}

// ============================================================================
// Component
// ============================================================================

/// Deterministic potential-interaction component.
///
/// A component contains logical qubits that have been connected by at least
/// one multi-qubit operation in the analyzed circuit.
///
/// The component is a compiler-analysis concept, not a claim about the
/// physical quantum state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntanglementComponent {
    /// Stable component identifier.
    ///
    /// Component IDs are assigned deterministically from the smallest logical
    /// qubit index in the component.
    id: usize,

    /// Logical qubits belonging to the component.
    qubits: Vec<QubitId>,

    /// Number of multi-qubit operations that contributed connectivity to this
    /// component.
    interaction_count: usize,
}

impl EntanglementComponent {
    /// Creates an empty component.
    fn new(id: usize) -> Self {
        Self {
            id,
            qubits: Vec::new(),
            interaction_count: 0,
        }
    }

    /// Returns the stable component identifier.
    #[must_use]
    pub const fn id(&self) -> usize {
        self.id
    }

    /// Returns logical qubits in canonical ascending order.
    #[must_use]
    pub fn qubits(&self) -> &[QubitId] {
        &self.qubits
    }

    /// Returns the number of logical qubits in the component.
    #[must_use]
    pub fn qubit_count(&self) -> usize {
        self.qubits.len()
    }

    /// Returns the number of multi-qubit interaction operations attributed to
    /// this component.
    #[must_use]
    pub const fn interaction_count(&self) -> usize {
        self.interaction_count
    }

    /// Returns whether this component contains more than one qubit.
    #[must_use]
    pub fn is_multi_qubit(&self) -> bool {
        self.qubits.len() > 1
    }
}

// ============================================================================
// Per-qubit information
// ============================================================================

/// Conservative interaction information for one logical qubit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QubitEntanglementInfo {
    /// Logical qubit.
    qubit: QubitId,

    /// Deterministic component identifier.
    component_id: usize,

    /// Number of distinct logical qubits in the same component.
    component_size: usize,

    /// Number of multi-qubit operations touching the component.
    interaction_count: usize,
}

impl QubitEntanglementInfo {
    /// Returns the logical qubit.
    #[must_use]
    pub const fn qubit(&self) -> QubitId {
        self.qubit
    }

    /// Returns the potential-interaction component.
    #[must_use]
    pub const fn component_id(&self) -> usize {
        self.component_id
    }

    /// Returns the number of qubits in the component.
    #[must_use]
    pub const fn component_size(&self) -> usize {
        self.component_size
    }

    /// Returns the number of multi-qubit interactions attributed to the
    /// component.
    #[must_use]
    pub const fn interaction_count(&self) -> usize {
        self.interaction_count
    }

    /// Returns whether this qubit belongs to a multi-qubit component.
    #[must_use]
    pub const fn may_be_entangled(&self) -> bool {
        self.component_size > 1
    }

    /// Returns the conservative relationship to another qubit.
    ///
    /// The caller must supply the other qubit's component identifier.
    #[must_use]
    pub const fn relation_to(
        self,
        other_component_id: usize,
    ) -> PotentialEntanglement {
        if self.component_id == other_component_id {
            PotentialEntanglement::PotentiallyCorrelated
        } else {
            PotentialEntanglement::Separate
        }
    }
}

// ============================================================================
// Summary
// ============================================================================

/// Compact summary suitable for large circuits.
///
/// Unlike [`EntanglementComponent`], this type does not retain component
/// membership lists.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EntanglementSummary {
    /// Logical qubit count.
    qubit_count: usize,

    /// Number of potential-interaction components.
    component_count: usize,

    /// Number of components containing multiple qubits.
    multi_qubit_component_count: usize,

    /// Size of the largest potential-interaction component.
    largest_component_size: usize,

    /// Number of operations that touched at least two logical qubits.
    multi_qubit_operation_count: usize,

    /// Maximum operand width observed for one operation.
    maximum_interaction_width: usize,
}

impl EntanglementSummary {
    /// Returns the logical qubit count.
    #[must_use]
    pub const fn qubit_count(&self) -> usize {
        self.qubit_count
    }

    /// Returns the number of potential-interaction components.
    #[must_use]
    pub const fn component_count(&self) -> usize {
        self.component_count
    }

    /// Returns the number of components containing multiple qubits.
    #[must_use]
    pub const fn multi_qubit_component_count(&self) -> usize {
        self.multi_qubit_component_count
    }

    /// Returns the largest component size.
    #[must_use]
    pub const fn largest_component_size(&self) -> usize {
        self.largest_component_size
    }

    /// Returns the number of multi-qubit operations.
    #[must_use]
    pub const fn multi_qubit_operation_count(&self) -> usize {
        self.multi_qubit_operation_count
    }

    /// Returns the maximum number of qubits touched by one operation.
    #[must_use]
    pub const fn maximum_interaction_width(&self) -> usize {
        self.maximum_interaction_width
    }

    /// Returns whether any potential multi-qubit interaction exists.
    #[must_use]
    pub const fn has_multi_qubit_interaction(&self) -> bool {
        self.multi_qubit_operation_count > 0
    }
}

// ============================================================================
// Internal union-find
// ============================================================================

/// Compact union-find structure.
///
/// This implementation is deliberately private. The optimizer should consume
/// semantic results rather than depend on the underlying algorithm.
#[derive(Debug, Clone)]
struct DisjointSet {
    parent: Vec<usize>,
    rank: Vec<u8>,
}

impl DisjointSet {
    fn new(size: usize) -> Self {
        let mut parent = Vec::with_capacity(size);

        for index in 0..size {
            parent.push(index);
        }

        Self {
            parent,
            rank: vec![0; size],
        }
    }

    fn len(&self) -> usize {
        self.parent.len()
    }

    fn find(&mut self, value: usize) -> usize {
        let mut root = value;

        while self.parent[root] != root {
            root = self.parent[root];
        }

        let mut current = value;

        while self.parent[current] != current {
            let next = self.parent[current];
            self.parent[current] = root;
            current = next;
        }

        root
    }

    fn find_read_only(&self, value: usize) -> usize {
        let mut root = value;

        while self.parent[root] != root {
            root = self.parent[root];
        }

        root
    }

    fn union(&mut self, first: usize, second: usize) {
        let first_root = self.find(first);
        let second_root = self.find(second);

        if first_root == second_root {
            return;
        }

        let first_rank = self.rank[first_root];
        let second_rank = self.rank[second_root];

        if first_rank < second_rank {
            self.parent[first_root] = second_root;
        } else if first_rank > second_rank {
            self.parent[second_root] = first_root;
        } else {
            self.parent[second_root] = first_root;

            self.rank[first_root] = self.rank[first_root]
                .saturating_add(1);
        }
    }
}

// ============================================================================
// Analysis engine
// ============================================================================

/// Production entanglement / potential-interaction analysis engine.
///
/// The engine is stateless apart from its immutable configuration. It can be
/// reused for multiple circuits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EntanglementAnalysis {
    config: EntanglementAnalysisConfig,
}

impl Default for EntanglementAnalysis {
    fn default() -> Self {
        Self::new()
    }
}

impl EntanglementAnalysis {
    /// Creates an analyzer using production defaults.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            config: EntanglementAnalysisConfig::new(),
        }
    }

    /// Creates an analyzer with explicit configuration.
    #[must_use]
    pub const fn with_config(
        config: EntanglementAnalysisConfig,
    ) -> Self {
        Self { config }
    }

    /// Returns the analyzer configuration.
    #[must_use]
    pub const fn config(self) -> EntanglementAnalysisConfig {
        self.config
    }

    /// Computes a compact summary.
    ///
    /// This is the preferred API when optimizing extremely large circuits
    /// because it does not retain component membership lists.
    pub fn summarize(
        &self,
        circuit: &QuantumCircuit,
    ) -> Result<EntanglementSummary, EntanglementAnalysisError> {
        circuit
            .validate()
            .map_err(|error| EntanglementAnalysisError::InvalidCircuit {
                message: error.to_string(),
            })?;

        let qubit_count = circuit.num_qubits();

        let mut dsu = DisjointSet::new(qubit_count);

        let mut multi_qubit_operation_count = 0usize;
        let mut maximum_interaction_width = 0usize;

        for gate in circuit.operations() {
            let qubits = gate.qubits();

            if qubits.len() < 2 {
                continue;
            }

            multi_qubit_operation_count = multi_qubit_operation_count
                .checked_add(1)
                .ok_or(
                    EntanglementAnalysisError::ArithmeticOverflow {
                        calculation:
                            "multi-qubit operation count",
                    },
                )?;

            if qubits.len() > maximum_interaction_width {
                maximum_interaction_width = qubits.len();
            }

            let first = qubit_index(qubits[0], qubit_count)?;

            for qubit in &qubits[1..] {
                let index = qubit_index(*qubit, qubit_count)?;

                dsu.union(first, index);
            }
        }

        let mut component_sizes = vec![0usize; qubit_count];

        for qubit in 0..qubit_count {
            let root = dsu.find_read_only(qubit);

            component_sizes[root] = component_sizes[root]
                .checked_add(1)
                .ok_or(
                    EntanglementAnalysisError::ArithmeticOverflow {
                        calculation: "component size",
                    },
                )?;
        }

        let mut component_count = 0usize;
        let mut multi_qubit_component_count = 0usize;
        let mut largest_component_size = 0usize;

        for size in component_sizes {
            if size == 0 {
                continue;
            }

            component_count = component_count
                .checked_add(1)
                .ok_or(
                    EntanglementAnalysisError::ArithmeticOverflow {
                        calculation: "component count",
                    },
                )?;

            if size > 1 {
                multi_qubit_component_count =
                    multi_qubit_component_count
                        .checked_add(1)
                        .ok_or(
                            EntanglementAnalysisError::ArithmeticOverflow {
                                calculation:
                                    "multi-qubit component count",
                            },
                        )?;
            }

            if size > largest_component_size {
                largest_component_size = size;
            }
        }

        Ok(EntanglementSummary {
            qubit_count,
            component_count,
            multi_qubit_component_count,
            largest_component_size,
            multi_qubit_operation_count,
            maximum_interaction_width,
        })
    }

    /// Computes the complete component representation.
    ///
    /// This API materializes component membership and is therefore protected by
    /// `max_materialized_qubits` and `max_materialized_components`.
    pub fn analyze(
        &self,
        circuit: &QuantumCircuit,
    ) -> Result<Vec<EntanglementComponent>, EntanglementAnalysisError> {
        circuit
            .validate()
            .map_err(|error| EntanglementAnalysisError::InvalidCircuit {
                message: error.to_string(),
            })?;

        let qubit_count = circuit.num_qubits();

        if qubit_count > self.config.max_materialized_qubits {
            return Err(
                EntanglementAnalysisError::ResultLimitExceeded {
                    requested: qubit_count,
                    maximum: self.config.max_materialized_qubits,
                },
            );
        }

        let mut dsu = DisjointSet::new(qubit_count);
        let mut interaction_counts = vec![0usize; qubit_count];

        for gate in circuit.operations() {
            let qubits = gate.qubits();

            if qubits.len() < 2 {
                continue;
            }

            let first = qubit_index(qubits[0], qubit_count)?;

            for qubit in &qubits[1..] {
                let index = qubit_index(*qubit, qubit_count)?;

                dsu.union(first, index);
            }
        }

        /*
         * Attribute interaction counts after connectivity has been built.
         *
         * Doing this in a second pass means an operation that connects two
         * previously independent components contributes to the final merged
         * component rather than to a transient implementation-specific root.
         */
        for gate in circuit.operations() {
            let qubits = gate.qubits();

            if qubits.len() < 2 {
                continue;
            }

            let first = qubit_index(qubits[0], qubit_count)?;
            let root = dsu.find_read_only(first);

            interaction_counts[root] =
                interaction_counts[root]
                    .checked_add(1)
                    .ok_or(
                        EntanglementAnalysisError::ArithmeticOverflow {
                            calculation: "component interaction count",
                        },
                    )?;
        }

        let mut component_sizes = vec![0usize; qubit_count];

        for qubit in 0..qubit_count {
            let root = dsu.find_read_only(qubit);

            component_sizes[root] =
                component_sizes[root]
                    .checked_add(1)
                    .ok_or(
                        EntanglementAnalysisError::ArithmeticOverflow {
                            calculation: "component size",
                        },
                    )?;
        }

        let mut components = Vec::<EntanglementComponent>::new();

        for qubit in 0..qubit_count {
            let root = dsu.find_read_only(qubit);

            if root != qubit {
                continue;
            }

            if component_sizes[root] == 0 {
                continue;
            }

            if components.len()
                >= self.config.max_materialized_components
            {
                return Err(
                    EntanglementAnalysisError::ResultLimitExceeded {
                        requested: components
                            .len()
                            .checked_add(1)
                            .ok_or(
                                EntanglementAnalysisError::ArithmeticOverflow {
                                    calculation:
                                        "materialized component count",
                                },
                            )?,
                        maximum: self
                            .config
                            .max_materialized_components,
                    },
                );
            }

            let component_id = smallest_qubit_in_component(
                &dsu,
                root,
                qubit_count,
            )?;

            components.push(EntanglementComponent {
                id: component_id,
                qubits: Vec::new(),
                interaction_count: interaction_counts[root],
            });
        }

        /*
         * Fill component membership deterministically.
         *
         * Components are already ordered by their root discovery order. The
         * final sort below converts implementation roots into semantic IDs.
         */
        for qubit in 0..qubit_count {
            let root = dsu.find_read_only(qubit);

            let component_id =
                smallest_qubit_in_component(
                    &dsu,
                    root,
                    qubit_count,
                )?;

            if let Some(component) = components
                .iter_mut()
                .find(|component| component.id == component_id)
            {
                component.qubits.push(
                    QubitId::new(qubit),
                );
            }
        }

        components.sort_by_key(|component| component.id);

        Ok(components)
    }

    /// Streams complete component membership without requiring the caller to
    /// retain the entire result.
    ///
    /// The callback receives one fully constructed component at a time.
    ///
    /// Returning `false` stops analysis cleanly.
    ///
    /// This API is intended for large circuits where retaining all component
    /// vectors would be unnecessary.
    pub fn analyze_into<F>(
        &self,
        circuit: &QuantumCircuit,
        mut sink: F,
    ) -> Result<(), EntanglementAnalysisError>
    where
        F: FnMut(EntanglementComponent) -> bool,
    {
        circuit
            .validate()
            .map_err(|error| EntanglementAnalysisError::InvalidCircuit {
                message: error.to_string(),
            })?;

        let qubit_count = circuit.num_qubits();

        /*
         * The core union-find state is still O(Q). The streamed API avoids
         * retaining O(Q) component vectors simultaneously.
         */
        let mut dsu = DisjointSet::new(qubit_count);

        for gate in circuit.operations() {
            let qubits = gate.qubits();

            if qubits.len() < 2 {
                continue;
            }

            let first = qubit_index(qubits[0], qubit_count)?;

            for qubit in &qubits[1..] {
                let index = qubit_index(*qubit, qubit_count)?;

                dsu.union(first, index);
            }
        }

        let mut component_sizes = vec![0usize; qubit_count];
        let mut interaction_counts = vec![0usize; qubit_count];

        for qubit in 0..qubit_count {
            let root = dsu.find_read_only(qubit);

            component_sizes[root] =
                component_sizes[root]
                    .checked_add(1)
                    .ok_or(
                        EntanglementAnalysisError::ArithmeticOverflow {
                            calculation: "component size",
                        },
                    )?;
        }

        for gate in circuit.operations() {
            let qubits = gate.qubits();

            if qubits.len() < 2 {
                continue;
            }

            let first = qubit_index(qubits[0], qubit_count)?;
            let root = dsu.find_read_only(first);

            interaction_counts[root] =
                interaction_counts[root]
                    .checked_add(1)
                    .ok_or(
                        EntanglementAnalysisError::ArithmeticOverflow {
                            calculation: "interaction count",
                        },
                    )?;
        }

        for qubit in 0..qubit_count {
            if dsu.find_read_only(qubit) != qubit {
                continue;
            }

            if component_sizes[qubit] == 0 {
                continue;
            }

            let component_id =
                smallest_qubit_in_component(
                    &dsu,
                    qubit,
                    qubit_count,
                )?;

            let mut members = Vec::with_capacity(
                component_sizes[qubit],
            );

            for candidate in 0..qubit_count {
                if dsu.find_read_only(candidate) == qubit {
                    members.push(QubitId::new(candidate));
                }
            }

            let component = EntanglementComponent {
                id: component_id,
                qubits: members,
                interaction_count: interaction_counts[qubit],
            };

            if !sink(component) {
                return Ok(());
            }
        }

        Ok(())
    }

    /// Returns the conservative relationship between two logical qubits.
    ///
    /// This uses the same potential-interaction semantics as the complete
    /// analysis.
    #[must_use]
    pub fn relationship(
        &self,
        circuit: &QuantumCircuit,
        first: QubitId,
        second: QubitId,
    ) -> Result<PotentialEntanglement, EntanglementAnalysisError> {
        circuit
            .validate()
            .map_err(|error| EntanglementAnalysisError::InvalidCircuit {
                message: error.to_string(),
            })?;

        let qubit_count = circuit.num_qubits();

        let first_index = qubit_index(first, qubit_count)?;
        let second_index = qubit_index(second, qubit_count)?;

        if first_index == second_index {
            return Ok(PotentialEntanglement::PotentiallyCorrelated);
        }

        let mut dsu = DisjointSet::new(qubit_count);

        for gate in circuit.operations() {
            let qubits = gate.qubits();

            if qubits.len() < 2 {
                continue;
            }

            let first_operand =
                qubit_index(qubits[0], qubit_count)?;

            for qubit in &qubits[1..] {
                let operand = qubit_index(*qubit, qubit_count)?;

                dsu.union(first_operand, operand);
            }
        }

        let first_root = dsu.find_read_only(first_index);
        let second_root = dsu.find_read_only(second_index);

        if first_root == second_root {
            Ok(PotentialEntanglement::PotentiallyCorrelated)
        } else {
            Ok(PotentialEntanglement::Separate)
        }
    }

    /// Returns per-qubit information for one logical qubit.
    ///
    /// This performs the same connectivity analysis but returns only the
    /// requested qubit's result.
    pub fn qubit_info(
        &self,
        circuit: &QuantumCircuit,
        qubit: QubitId,
    ) -> Result<QubitEntanglementInfo, EntanglementAnalysisError> {
        circuit
            .validate()
            .map_err(|error| EntanglementAnalysisError::InvalidCircuit {
                message: error.to_string(),
            })?;

        let qubit_count = circuit.num_qubits();
        let target = qubit_index(qubit, qubit_count)?;

        let mut dsu = DisjointSet::new(qubit_count);

        for gate in circuit.operations() {
            let qubits = gate.qubits();

            if qubits.len() < 2 {
                continue;
            }

            let first =
                qubit_index(qubits[0], qubit_count)?;

            for operand in &qubits[1..] {
                let index =
                    qubit_index(*operand, qubit_count)?;

                dsu.union(first, index);
            }
        }

        let root = dsu.find_read_only(target);

        let mut component_size = 0usize;

        for candidate in 0..qubit_count {
            if dsu.find_read_only(candidate) == root {
                component_size =
                    component_size
                        .checked_add(1)
                        .ok_or(
                            EntanglementAnalysisError::ArithmeticOverflow {
                                calculation:
                                    "qubit component size",
                            },
                        )?;
            }
        }

        let mut interaction_count = 0usize;

        for gate in circuit.operations() {
            let qubits = gate.qubits();

            if qubits.len() < 2 {
                continue;
            }

            let first =
                qubit_index(qubits[0], qubit_count)?;

            if dsu.find_read_only(first) == root {
                interaction_count =
                    interaction_count
                        .checked_add(1)
                        .ok_or(
                            EntanglementAnalysisError::ArithmeticOverflow {
                                calculation:
                                    "qubit interaction count",
                            },
                        )?;
            }
        }

        Ok(QubitEntanglementInfo {
            qubit,
            component_id: smallest_qubit_in_component(
                &dsu,
                root,
                qubit_count,
            )?,
            component_size,
            interaction_count,
        })
    }
}

// ============================================================================
// Free-function API
// ============================================================================

/// Performs production-default potential-entanglement analysis.
pub fn analyze(
    circuit: &QuantumCircuit,
) -> Result<Vec<EntanglementComponent>, EntanglementAnalysisError> {
    EntanglementAnalysis::new().analyze(circuit)
}

/// Produces a compact production-default summary.
pub fn summarize(
    circuit: &QuantumCircuit,
) -> Result<EntanglementSummary, EntanglementAnalysisError> {
    EntanglementAnalysis::new().summarize(circuit)
}

/// Determines the conservative potential-correlation relationship between
/// two logical qubits.
pub fn relationship(
    circuit: &QuantumCircuit,
    first: QubitId,
    second: QubitId,
) -> Result<PotentialEntanglement, EntanglementAnalysisError> {
    EntanglementAnalysis::new()
        .relationship(circuit, first, second)
}

// ============================================================================
// Helpers
// ============================================================================

/// Converts a canonical `QubitId` into a checked vector index.
///
/// The canonical IR exposes logical qubit identities separately from the
/// optimizer's internal indexing representation. This helper keeps that
/// conversion at one boundary.
fn qubit_index(
    qubit: QubitId,
    qubit_count: usize,
) -> Result<usize, EntanglementAnalysisError> {
    let index = qubit.index();

    if index >= qubit_count {
        return Err(
            EntanglementAnalysisError::InvalidQubit {
                qubit: index,
                qubit_count,
            },
        );
    }

    Ok(index)
}

/// Finds the smallest logical qubit belonging to one component.
///
/// This provides a semantic component ID independent of the union-find root
/// selected by rank heuristics.
fn smallest_qubit_in_component(
    dsu: &DisjointSet,
    root: usize,
    qubit_count: usize,
) -> Result<usize, EntanglementAnalysisError> {
    for qubit in 0..qubit_count {
        if dsu.find_read_only(qubit) == root {
            return Ok(qubit);
        }
    }

    Err(
        EntanglementAnalysisError::ArithmeticOverflow {
            calculation: "component root discovery",
        },
    )
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn disjoint_set_starts_as_singletons() {
        let dsu = DisjointSet::new(4);

        assert_eq!(dsu.len(), 4);

        for index in 0..4 {
            assert_eq!(dsu.find_read_only(index), index);
        }
    }

    #[test]
    fn disjoint_set_merges_components() {
        let mut dsu = DisjointSet::new(4);

        dsu.union(0, 1);
        dsu.union(1, 2);

        assert_eq!(
            dsu.find_read_only(0),
            dsu.find_read_only(2)
        );

        assert_ne!(
            dsu.find_read_only(0),
            dsu.find_read_only(3)
        );
    }

    #[test]
    fn potential_entanglement_semantics_are_conservative() {
        assert!(
            PotentialEntanglement::Separate
                .permits_separability()
        );

        assert!(
            PotentialEntanglement::PotentiallyCorrelated
                .may_be_correlated()
        );
    }

    #[test]
    fn config_defaults_are_bounded_for_materialization() {
        let config = EntanglementAnalysisConfig::new();

        assert_eq!(
            config.max_materialized_qubits(),
            1_000_000
        );

        assert_eq!(
            config.max_materialized_components(),
            1_000_000
        );
    }

    #[test]
    fn unlimited_configuration_does_not_change_core_semantics() {
        let config =
            EntanglementAnalysisConfig::unlimited_materialization();

        assert_eq!(
            config.max_materialized_qubits(),
            usize::MAX
        );

        assert_eq!(
            config.max_materialized_components(),
            usize::MAX
        );
    }

    #[test]
    fn qubit_index_rejects_out_of_range_values() {
        let result = qubit_index(
            QubitId::new(3),
            3,
        );

        assert!(matches!(
            result,
            Err(
                EntanglementAnalysisError::InvalidQubit {
                    qubit: 3,
                    qubit_count: 3
                }
            )
        ));
    }

    #[test]
    fn qubit_index_accepts_valid_values() {
        assert_eq!(
            qubit_index(QubitId::new(2), 3)
                .expect("valid qubit"),
            2
        );
    }
}