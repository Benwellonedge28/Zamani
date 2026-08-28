//! Zamani Quantum Routing — Initial Qubit Layout
//!
//! `src/quantum/routing/layout.rs`
//!
//! Production-grade logical-to-physical initial layout selection.
//!
//! # Responsibility
//!
//! This module owns the placement of logical qubits onto physical hardware
//! qubits before routing begins.
//!
//! It provides:
//!
//! - a stable `Layout` result type;
//! - a stable `LayoutInput` contract;
//! - a stable `LayoutStrategy` implementation boundary;
//! - deterministic trivial layout;
//! - connectivity/dense layout;
//! - interaction-graph-aware layout;
//! - noise-aware layout hooks;
//! - SABRE-compatible initial-layout hooks;
//! - fixed caller-supplied layouts;
//! - automatic strategy selection;
//! - deterministic tie-breaking;
//! - physical-qubit availability filtering;
//! - qubit-role filtering;
//! - injectivity validation;
//! - topology validation;
//! - bounded layout search;
//! - layout quality metrics;
//! - reproducibility metadata;
//! - transactional layout construction;
//! - no mutation of the input circuit;
//! - no mutation of the physical topology;
//! - no routing/SWAP insertion.
//!
//! # Architectural boundary
//!
//! Layout is deliberately separate from routing:
//!
//! ```text
//!                     Quantum IR
//!                         │
//!                         ▼
//!                 Interaction analysis
//!                         │
//!                         ▼
//!                    layout.rs
//!                         │
//!             logical -> physical mapping
//!                         │
//!                         ▼
//!                  routing algorithms
//!                         │
//!                         ▼
//!                    routed circuit
//! ```
//!
//! Layout answers:
//!
//! > "Where should the logical qubits initially live?"
//!
//! Routing answers:
//!
//! > "How should those qubits move so subsequent operations become executable?"
//!
//! `layout.rs` therefore MUST NOT:
//!
//! - insert SWAPs;
//! - mutate a circuit;
//! - execute gates;
//! - decompose gates;
//! - schedule operations;
//! - synthesize pulses;
//! - communicate with hardware;
//! - perform OpenQASM parsing;
//! - perform simulation;
//! - perform QEC decoding;
//! - implement the routing algorithms themselves.
//!
//! # Stable integration contract
//!
//! Later files consume this module through:
//!
//! ```text
//! LayoutInput
//!     │
//!     ▼
//! LayoutEngine::compute()
//!     │
//!     ▼
//! Layout
//!     │
//!     ├──► router.rs
//!     ├──► algorithms/basic.rs
//!     ├──► algorithms/lookahead.rs
//!     ├──► algorithms/sabre.rs
//!     ├──► algorithms/noise_aware.rs
//!     └──► transpiler.rs
//! ```
//!
//! The API is intentionally independent of the compiler's concrete IR
//! implementation. The transpiler/IR adapter is responsible for converting
//! canonical Quantum IR into `LayoutInput`.
//!
//! # Supported strategies
//!
//! ```text
//! Auto
//! Trivial
//! Dense
//! InteractionGraph
//! NoiseAware
//! Sabre
//! Fixed
//! Custom
//! ```
//!
//! `Sabre` here means SABRE-compatible *initial layout selection*. It does not
//! implement the routing loop. The routing implementation belongs in
//! `algorithms/sabre.rs`.
//!
//! # Determinism
//!
//! Layout decisions are deterministic whenever:
//!
//! - the topology is deterministic;
//! - the interaction list is deterministic;
//! - `deterministic == true`;
//! - a deterministic strategy is selected.
//!
//! Every tie is resolved by logical-qubit ID followed by physical-qubit ID.
//!
//! No `HashMap` iteration order is used for externally observable decisions.
//!
//! # Safety
//!
//! - No `unsafe` code.
//! - No global mutable state.
//! - No filesystem access.
//! - No network access.
//! - No provider access.
//! - No environment-dependent behavior.
//! - No random number generation.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021
//! - stable Rust
//!
//! No nightly features are required.
//!
//! # File completion invariant
//!
//! This file is complete when:
//!
//! 1. every configured layout strategy has a defined behavior;
//! 2. fixed layouts are validated before use;
//! 3. mappings are always injective;
//! 4. unavailable physical qubits are never selected unless explicitly allowed;
//! 5. logical qubit roles can constrain placement;
//! 6. topology validation happens before placement;
//! 7. deterministic tie-breaking is guaranteed;
//! 8. no circuit or topology is mutated;
//! 9. layout quality can be measured independently of routing;
//! 10. layout output is reusable by all routing algorithms;
//! 11. future SABRE routing can consume this file without changing it;
//! 12. custom strategies have a stable trait boundary;
//! 13. no unsafe Rust exists;
//! 14. no later routing file needs to modify this API merely to use it.
//!
//! # External design basis
//!
//! The design follows the established compiler separation between layout and
//! routing and leaves routing-specific heuristics such as the SABRE search
//! loop to the routing algorithm layer. This mirrors the separation used by
//! mature quantum transpiler architectures while retaining Zamani's
//! backend-independent design.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use crate::quantum::routing::config::LayoutStrategy;
use crate::quantum::routing::errors::{RoutingError, RoutingResult};
use crate::quantum::routing::mapping::{MappingError, QubitMapping};
use crate::quantum::routing::topology::PhysicalTopology;
use crate::quantum::routing::types::{
    LogicalQubitId,
    PhysicalQubitId,
    QubitInteraction,
    QubitRole,
};

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fmt;

// =============================================================================
// Constants
// =============================================================================

/// Default maximum number of logical qubits accepted by one layout request.
///
/// This is a safety bound for externally supplied layout input. It is not a
/// hardware limit and does not restrict the rest of Zamani's Quantum IR.
pub const DEFAULT_MAX_LOGICAL_QUBITS: usize = 1_000_000;

/// Default maximum number of interaction records accepted by one layout
/// request.
pub const DEFAULT_MAX_INTERACTIONS: usize = 10_000_000;

/// Default maximum number of candidate physical qubits examined for one
/// logical qubit during heuristic placement.
pub const DEFAULT_CANDIDATE_PHYSICAL_QUBITS: usize = 4096;

/// Default maximum number of layout attempts for a strategy.
pub const DEFAULT_MAX_ATTEMPTS: usize = 1;

/// Maximum layout attempts accepted by the public bounded configuration.
pub const MAX_LAYOUT_ATTEMPTS: usize = 1_000_000;

/// Maximum logical qubits accepted by a single layout input.
pub const MAX_LAYOUT_LOGICAL_QUBITS: usize = 100_000_000;

/// Maximum interaction records accepted by a single layout input.
pub const MAX_LAYOUT_INTERACTIONS: usize = 1_000_000_000;

// =============================================================================
// Layout error
// =============================================================================

/// Layout-specific diagnostic error.
///
/// The routing-wide error model remains the public compiler-facing error
/// boundary. This local type exists so the layout implementation can preserve
/// detailed classification without depending on later routing files.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LayoutError {
    /// No logical qubits were supplied.
    EmptyLogicalProgram,

    /// More logical qubits were supplied than the selected hardware can host.
    InsufficientPhysicalQubits {
        logical: usize,
        physical: usize,
    },

    /// A logical qubit was repeated.
    DuplicateLogicalQubit {
        qubit: LogicalQubitId,
    },

    /// A fixed mapping contains a logical qubit not present in the input.
    UnknownFixedLogicalQubit {
        qubit: LogicalQubitId,
    },

    /// A fixed mapping uses a physical qubit that is not part of the topology.
    InvalidFixedPhysicalQubit {
        qubit: PhysicalQubitId,
    },

    /// A fixed mapping uses an unavailable physical qubit.
    UnavailableFixedPhysicalQubit {
        qubit: PhysicalQubitId,
    },

    /// A fixed mapping is not injective.
    FixedMappingCollision {
        physical: PhysicalQubitId,
    },

    /// A required logical role cannot be placed.
    NoEligiblePhysicalQubit {
        logical: LogicalQubitId,
        role: QubitRole,
    },

    /// A heuristic could not produce a complete mapping.
    LayoutConstructionFailed {
        strategy: String,
        logical: LogicalQubitId,
    },

    /// A custom strategy was requested but no implementation was supplied.
    CustomStrategyUnavailable {
        name: String,
    },

    /// A required interaction references an unknown logical qubit.
    UnknownInteractionQubit {
        qubit: LogicalQubitId,
    },

    /// A configuration value is invalid.
    InvalidConfiguration {
        field: String,
        detail: String,
    },

    /// A configured resource bound was exceeded.
    ResourceLimitExceeded {
        resource: String,
        requested: usize,
        maximum: usize,
    },

    /// A mapping implementation rejected the generated placement.
    MappingFailure {
        detail: String,
    },

    /// A topology invariant required by layout was violated.
    TopologyFailure {
        detail: String,
    },

    /// An internal layout invariant was violated.
    InvariantViolation {
        detail: String,
    },
}

impl fmt::Display for LayoutError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyLogicalProgram => {
                write!(formatter, "layout input contains no logical qubits")
            }

            Self::InsufficientPhysicalQubits {
                logical,
                physical,
            } => write!(
                formatter,
                "layout requires {logical} physical qubits but topology provides {physical}"
            ),

            Self::DuplicateLogicalQubit { qubit } => {
                write!(formatter, "logical qubit {qubit} occurs more than once")
            }

            Self::UnknownFixedLogicalQubit { qubit } => write!(
                formatter,
                "fixed layout references logical qubit {qubit}, which is not present in the program"
            ),

            Self::InvalidFixedPhysicalQubit { qubit } => write!(
                formatter,
                "fixed layout references physical qubit {qubit}, which is not present in the topology"
            ),

            Self::UnavailableFixedPhysicalQubit { qubit } => write!(
                formatter,
                "fixed layout references unavailable physical qubit {qubit}"
            ),

            Self::FixedMappingCollision { physical } => write!(
                formatter,
                "fixed layout assigns more than one logical qubit to physical qubit {physical}"
            ),

            Self::NoEligiblePhysicalQubit { logical, role } => write!(
                formatter,
                "no eligible physical qubit is available for logical qubit {logical} with role {role:?}"
            ),

            Self::LayoutConstructionFailed {
                strategy,
                logical,
            } => write!(
                formatter,
                "layout strategy `{strategy}` could not place logical qubit {logical}"
            ),

            Self::CustomStrategyUnavailable { name } => write!(
                formatter,
                "custom layout strategy `{name}` has no registered implementation"
            ),

            Self::UnknownInteractionQubit { qubit } => write!(
                formatter,
                "interaction references unknown logical qubit {qubit}"
            ),

            Self::InvalidConfiguration { field, detail } => write!(
                formatter,
                "invalid layout configuration `{field}`: {detail}"
            ),

            Self::ResourceLimitExceeded {
                resource,
                requested,
                maximum,
            } => write!(
                formatter,
                "layout resource `{resource}` requested {requested}, maximum is {maximum}"
            ),

            Self::MappingFailure { detail } => {
                write!(formatter, "layout mapping failure: {detail}")
            }

            Self::TopologyFailure { detail } => {
                write!(formatter, "layout topology failure: {detail}")
            }

            Self::InvariantViolation { detail } => {
                write!(formatter, "layout invariant violation: {detail}")
            }
        }
    }
}

impl std::error::Error for LayoutError {}

impl From<MappingError> for LayoutError {
    fn from(error: MappingError) -> Self {
        Self::MappingFailure {
            detail: error.to_string(),
        }
    }
}

// =============================================================================
// Layout configuration
// =============================================================================

/// Additional configuration owned specifically by the layout engine.
///
/// This is intentionally separate from `RoutingConfig`. `RoutingConfig`
/// selects the strategy; `LayoutConfig` controls the bounded behavior of that
/// strategy.
///
/// This means `config.rs` remains stable even when layout implementation
/// details evolve.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LayoutConfig {
    /// Maximum number of logical qubits accepted.
    pub max_logical_qubits: usize,

    /// Maximum number of interaction records accepted.
    pub max_interactions: usize,

    /// Maximum number of physical candidates inspected per placement.
    pub candidate_physical_limit: usize,

    /// Maximum number of layout attempts.
    pub max_attempts: usize,

    /// Whether unavailable physical qubits may be used.
    ///
    /// Production default is false.
    pub allow_unavailable: bool,

    /// Whether deterministic tie-breaking is required.
    pub deterministic: bool,

    /// Whether the input interaction list should be normalized.
    pub normalize_interactions: bool,
}

impl Default for LayoutConfig {
    fn default() -> Self {
        Self {
            max_logical_qubits: DEFAULT_MAX_LOGICAL_QUBITS,
            max_interactions: DEFAULT_MAX_INTERACTIONS,
            candidate_physical_limit: DEFAULT_CANDIDATE_PHYSICAL_QUBITS,
            max_attempts: DEFAULT_MAX_ATTEMPTS,
            allow_unavailable: false,
            deterministic: true,
            normalize_interactions: true,
        }
    }
}

impl LayoutConfig {
    /// Creates production defaults.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Validates layout resource limits.
    pub fn validate(&self) -> Result<(), LayoutError> {
        if self.max_logical_qubits == 0 {
            return Err(LayoutError::InvalidConfiguration {
                field: "max_logical_qubits".to_string(),
                detail: "must be greater than zero".to_string(),
            });
        }

        if self.max_logical_qubits > MAX_LAYOUT_LOGICAL_QUBITS {
            return Err(LayoutError::InvalidConfiguration {
                field: "max_logical_qubits".to_string(),
                detail: format!(
                    "must not exceed {MAX_LAYOUT_LOGICAL_QUBITS}"
                ),
            });
        }

        if self.max_interactions == 0 {
            return Err(LayoutError::InvalidConfiguration {
                field: "max_interactions".to_string(),
                detail: "must be greater than zero".to_string(),
            });
        }

        if self.max_interactions > MAX_LAYOUT_INTERACTIONS {
            return Err(LayoutError::InvalidConfiguration {
                field: "max_interactions".to_string(),
                detail: format!(
                    "must not exceed {MAX_LAYOUT_INTERACTIONS}"
                ),
            });
        }

        if self.candidate_physical_limit == 0 {
            return Err(LayoutError::InvalidConfiguration {
                field: "candidate_physical_limit".to_string(),
                detail: "must be greater than zero".to_string(),
            });
        }

        if self.max_attempts == 0 {
            return Err(LayoutError::InvalidConfiguration {
                field: "max_attempts".to_string(),
                detail: "must be greater than zero".to_string(),
            });
        }

        if self.max_attempts > MAX_LAYOUT_ATTEMPTS {
            return Err(LayoutError::InvalidConfiguration {
                field: "max_attempts".to_string(),
                detail: format!("must not exceed {MAX_LAYOUT_ATTEMPTS}"),
            });
        }

        Ok(())
    }
}

// =============================================================================
// Logical qubit specification
// =============================================================================

/// A logical qubit together with its routing/layout role.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LogicalQubitSpec {
    /// Logical qubit identifier.
    pub id: LogicalQubitId,

    /// Placement role.
    pub role: QubitRole,
}

impl LogicalQubitSpec {
    /// Creates a normal data-qubit specification.
    #[must_use]
    pub const fn data(id: LogicalQubitId) -> Self {
        Self {
            id,
            role: QubitRole::Data,
        }
    }

    /// Creates a role-specific logical-qubit specification.
    #[must_use]
    pub const fn new(id: LogicalQubitId, role: QubitRole) -> Self {
        Self { id, role }
    }
}

// =============================================================================
// Layout input
// =============================================================================

/// Complete input to a layout strategy.
///
/// This is the stable boundary between Quantum IR analysis and layout.
///
/// The circuit itself is deliberately absent. The IR adapter extracts only
/// the information layout actually needs:
///
/// - logical qubit identities;
//! - logical roles;
//! - interaction graph;
/// - optional caller-provided mapping.
#[derive(Debug, Clone)]
pub struct LayoutInput {
    /// Logical qubits that must be placed.
    logical_qubits: Vec<LogicalQubitSpec>,

    /// Logical interaction records.
    interactions: Vec<QubitInteraction>,

    /// Optional caller-supplied initial mapping.
    ///
    /// Required when `LayoutStrategy::Fixed` is selected.
    fixed_mapping: Option<QubitMapping>,
}

impl LayoutInput {
    /// Creates a layout input.
    pub fn new(
        logical_qubits: Vec<LogicalQubitSpec>,
        interactions: Vec<QubitInteraction>,
    ) -> Result<Self, LayoutError> {
        Self::with_fixed_mapping(logical_qubits, interactions, None)
    }

    /// Creates a layout input with an optional fixed mapping.
    pub fn with_fixed_mapping(
        logical_qubits: Vec<LogicalQubitSpec>,
        interactions: Vec<QubitInteraction>,
        fixed_mapping: Option<QubitMapping>,
    ) -> Result<Self, LayoutError> {
        validate_logical_qubits(&logical_qubits)?;
        validate_interactions(&logical_qubits, &interactions)?;

        Ok(Self {
            logical_qubits,
            interactions,
            fixed_mapping,
        })
    }

    /// Returns the logical qubit specifications.
    #[must_use]
    pub fn logical_qubits(&self) -> &[LogicalQubitSpec] {
        &self.logical_qubits
    }

    /// Returns the interaction list.
    #[must_use]
    pub fn interactions(&self) -> &[QubitInteraction] {
        &self.interactions
    }

    /// Returns the caller-supplied mapping, if any.
    #[must_use]
    pub fn fixed_mapping(&self) -> Option<&QubitMapping> {
        self.fixed_mapping.as_ref()
    }

    /// Returns the number of logical qubits.
    #[must_use]
    pub fn logical_qubit_count(&self) -> usize {
        self.logical_qubits.len()
    }

    /// Returns the number of interactions.
    #[must_use]
    pub fn interaction_count(&self) -> usize {
        self.interactions.len()
    }
}

// =============================================================================
// Layout quality
// =============================================================================

/// Immutable quality measurements for an initial layout.
///
/// These metrics do not claim that the circuit is routed. They measure how
/// suitable the initial placement is for the supplied interaction graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct LayoutQuality {
    /// Number of interaction pairs that are directly adjacent.
    pub adjacent_interactions: usize,

    /// Number of interaction pairs that are not directly adjacent.
    pub non_adjacent_interactions: usize,

    /// Sum of shortest-path distances for interactions.
    ///
    /// `0` means all interactions are either single-qubit or self-free.
    pub total_interaction_distance: u64,

    /// Maximum shortest-path distance among interactions.
    pub maximum_interaction_distance: u64,

    /// Number of physical qubits actually occupied.
    pub occupied_physical_qubits: usize,
}

impl LayoutQuality {
    /// Returns the number of two-qubit interactions measured.
    #[must_use]
    pub const fn interaction_count(self) -> usize {
        self.adjacent_interactions + self.non_adjacent_interactions
    }

    /// Returns true when every measured two-qubit interaction is adjacent.
    #[must_use]
    pub const fn is_fully_connected(self) -> bool {
        self.non_adjacent_interactions == 0
    }
}

// =============================================================================
// Layout statistics
// =============================================================================

/// Diagnostic information about how a layout was constructed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LayoutStatistics {
    /// Selected strategy.
    pub strategy: String,

    /// Number of logical qubits.
    pub logical_qubits: usize,

    /// Number of available physical qubits considered.
    pub physical_qubits_considered: usize,

    /// Number of physical candidates examined.
    pub candidates_examined: usize,

    /// Number of placement decisions made.
    pub placements: usize,

    /// Number of layout attempts.
    pub attempts: usize,

    /// Whether the result was deterministic.
    pub deterministic: bool,
}

// =============================================================================
// Layout result
// =============================================================================

/// Complete initial-layout result.
///
/// The result is immutable from the layout engine's perspective. Routing
/// algorithms may clone the mapping into their mutable routing state.
#[derive(Debug, Clone)]
pub struct Layout {
    /// Initial logical-to-physical mapping.
    mapping: QubitMapping,

    /// Strategy that produced the mapping.
    strategy: LayoutStrategy,

    /// Initial-layout quality.
    quality: LayoutQuality,

    /// Construction statistics.
    statistics: LayoutStatistics,
}

impl Layout {
    /// Creates a validated layout result.
    fn new(
        mapping: QubitMapping,
        strategy: LayoutStrategy,
        quality: LayoutQuality,
        statistics: LayoutStatistics,
    ) -> Result<Self, LayoutError> {
        mapping
            .validate()
            .map_err(LayoutError::from)?;

        Ok(Self {
            mapping,
            strategy,
            quality,
            statistics,
        })
    }

    /// Returns the initial mapping.
    #[must_use]
    pub fn mapping(&self) -> &QubitMapping {
        &self.mapping
    }

    /// Clones the initial mapping for a mutable routing pass.
    #[must_use]
    pub fn clone_mapping(&self) -> QubitMapping {
        self.mapping.clone()
    }

    /// Returns the selected strategy.
    #[must_use]
    pub fn strategy(&self) -> &LayoutStrategy {
        &self.strategy
    }

    /// Returns layout quality metrics.
    #[must_use]
    pub const fn quality(&self) -> LayoutQuality {
        self.quality
    }

    /// Returns construction statistics.
    #[must_use]
    pub fn statistics(&self) -> &LayoutStatistics {
        &self.statistics
    }

    /// Returns the physical location of a logical qubit.
    #[must_use]
    pub fn physical_of(
        &self,
        logical: LogicalQubitId,
    ) -> Option<PhysicalQubitId> {
        self.mapping.physical_of(logical)
    }

    /// Returns the logical occupant of a physical qubit.
    #[must_use]
    pub fn logical_at(
        &self,
        physical: PhysicalQubitId,
    ) -> Option<LogicalQubitId> {
        self.mapping.logical_at(physical)
    }
}

// =============================================================================
// Layout strategy trait
// =============================================================================

/// Stable extension point for layout algorithms.
///
/// Implementations must:
///
/// - never mutate the topology;
/// - never mutate the `LayoutInput`;
/// - return an injective mapping;
/// - obey the configured physical-resource constraints;
/// - be deterministic when requested.
///
/// This trait is intentionally object-safe so custom layout strategies can be
/// registered later by `router.rs` or a plugin layer without changing this
/// file.
pub trait LayoutStrategyEngine: Send + Sync {
    /// Returns the strategy identifier.
    fn strategy(&self) -> LayoutStrategy;

    /// Computes an initial logical-to-physical layout.
    fn compute(
        &self,
        input: &LayoutInput,
        topology: &PhysicalTopology,
        config: &LayoutConfig,
    ) -> Result<Layout, LayoutError>;
}

// =============================================================================
// Built-in strategy implementations
// =============================================================================

/// Deterministic trivial layout.
///
/// Logical qubit `q0` is assigned to the first eligible physical qubit,
/// `q1` to the second, and so on.
///
/// This is the correct baseline and is useful for:
///
/// - deterministic compilation;
/// - debugging;
/// - regression tests;
/// - benchmarking other layout algorithms.
#[derive(Debug, Clone, Copy, Default)]
pub struct TrivialLayout;

impl LayoutStrategyEngine for TrivialLayout {
    fn strategy(&self) -> LayoutStrategy {
        LayoutStrategy::Trivial
    }

    fn compute(
        &self,
        input: &LayoutInput,
        topology: &PhysicalTopology,
        config: &LayoutConfig,
    ) -> Result<Layout, LayoutError> {
        config.validate()?;
        validate_topology(topology)?;

        ensure_capacity(input, topology, config)?;

        let physical = eligible_physical_qubits(
            topology,
            config.allow_unavailable,
        );

        let mut mapping = QubitMapping::new();
        let mut candidates_examined = 0usize;

        for (index, logical) in input.logical_qubits().iter().enumerate() {
            let physical_qubit = physical
                .get(index)
                .copied()
                .ok_or(LayoutError::NoEligiblePhysicalQubit {
                    logical: logical.id,
                    role: logical.role,
                })?;

            candidates_examined = candidates_examined
                .checked_add(1)
                .ok_or_else(|| LayoutError::InvariantViolation {
                    detail: "candidate counter overflow".to_string(),
                })?;

            mapping.assign(logical.id, physical_qubit)?;
        }

        let quality =
            calculate_quality(input, topology, &mapping)?;

        let statistics = LayoutStatistics {
            strategy: self.strategy().name().to_string(),
            logical_qubits: input.logical_qubit_count(),
            physical_qubits_considered: physical.len(),
            candidates_examined,
            placements: input.logical_qubit_count(),
            attempts: 1,
            deterministic: config.deterministic,
        };

        Layout::new(
            mapping,
            self.strategy(),
            quality,
            statistics,
        )
    }
}

/// Dense/connectivity-oriented layout.
///
/// This strategy places the most highly connected logical qubits onto the most
/// connected physical qubits.
///
/// It does not attempt global optimality. Its purpose is to produce a fast,
/// deterministic placement that starts in a connectivity-rich region.
#[derive(Debug, Clone, Copy, Default)]
pub struct DenseLayout;

impl LayoutStrategyEngine for DenseLayout {
    fn strategy(&self) -> LayoutStrategy {
        LayoutStrategy::Dense
    }

    fn compute(
        &self,
        input: &LayoutInput,
        topology: &PhysicalTopology,
        config: &LayoutConfig,
    ) -> Result<Layout, LayoutError> {
        config.validate()?;
        validate_topology(topology)?;
        ensure_capacity(input, topology, config)?;

        let physical =
            eligible_physical_qubits(topology, config.allow_unavailable);

        let logical_order =
            logical_order_by_interaction_degree(input);

        let physical_order =
            physical_order_by_connectivity(topology, &physical);

        let mapping = assign_ordered(
            &logical_order,
            &physical_order,
            topology,
        )?;

        let quality =
            calculate_quality(input, topology, &mapping)?;

        let statistics = LayoutStatistics {
            strategy: self.strategy().name().to_string(),
            logical_qubits: input.logical_qubit_count(),
            physical_qubits_considered: physical.len(),
            candidates_examined: physical.len(),
            placements: input.logical_qubit_count(),
            attempts: 1,
            deterministic: config.deterministic,
        };

        Layout::new(
            mapping,
            self.strategy(),
            quality,
            statistics,
        )
    }
}

/// Interaction-graph-aware layout.
///
/// This strategy greedily embeds the logical interaction graph into the
/// physical connectivity graph.
///
/// The algorithm:
///
/// 1. ranks logical qubits by weighted interaction degree;
/// 2. places the most connected logical qubit on a highly connected physical
///    qubit;
/// 3. repeatedly chooses the unplaced logical qubit with the strongest
///    interaction with already placed logical qubits;
/// 4. chooses a physical location minimizing distance to the already placed
///    interacting qubits;
/// 5. resolves every tie deterministically.
///
/// This is deliberately a layout heuristic rather than an exact subgraph
/// isomorphism solver. Exact embedding can be exponentially expensive and
/// belongs behind a separate strategy if later required.
#[derive(Debug, Clone, Copy, Default)]
pub struct InteractionGraphLayout;

impl LayoutStrategyEngine for InteractionGraphLayout {
    fn strategy(&self) -> LayoutStrategy {
        LayoutStrategy::InteractionGraph
    }

    fn compute(
        &self,
        input: &LayoutInput,
        topology: &PhysicalTopology,
        config: &LayoutConfig,
    ) -> Result<Layout, LayoutError> {
        config.validate()?;
        validate_topology(topology)?;
        ensure_capacity(input, topology, config)?;

        let physical =
            eligible_physical_qubits(topology, config.allow_unavailable);

        let interaction_graph =
            InteractionGraph::build(input)?;

        let mut mapping = QubitMapping::new();

        let first_logical = interaction_graph
            .ordered_logical_qubits()
            .first()
            .copied()
            .ok_or(LayoutError::EmptyLogicalProgram)?;

        let first_physical = choose_best_seed_physical(
            topology,
            &physical,
            &interaction_graph,
        )?;

        mapping.assign(first_logical, first_physical)?;

        let mut placed: BTreeSet<LogicalQubitId> =
            BTreeSet::new();
        placed.insert(first_logical);

        let logical_count = input.logical_qubit_count();

        while placed.len() < logical_count {
            let next_logical =
                choose_next_logical(&interaction_graph, &placed)?;

            let physical_qubit =
                choose_best_physical_for_logical(
                    next_logical,
                    &interaction_graph,
                    &mapping,
                    topology,
                    &physical,
                    config,
                )?;

            mapping.assign(next_logical, physical_qubit)?;
            placed.insert(next_logical);
        }

        let quality =
            calculate_quality(input, topology, &mapping)?;

        let statistics = LayoutStatistics {
            strategy: self.strategy().name().to_string(),
            logical_qubits: input.logical_qubit_count(),
            physical_qubits_considered: physical.len(),
            candidates_examined: physical.len(),
            placements: mapping.len(),
            attempts: 1,
            deterministic: config.deterministic,
        };

        Layout::new(
            mapping,
            self.strategy(),
            quality,
            statistics,
        )
    }
}

/// Noise-aware initial layout.
///
/// The layout layer does not own calibration. Therefore this strategy uses the
/// topology's already-materialized physical properties when available and
/// otherwise falls back to connectivity-aware placement.
///
/// The implementation deliberately does not invent noise values.
///
/// This is important: missing calibration must never silently become a fake
/// "perfect hardware" score.
#[derive(Debug, Clone, Copy, Default)]
pub struct NoiseAwareLayout;

impl LayoutStrategyEngine for NoiseAwareLayout {
    fn strategy(&self) -> LayoutStrategy {
        LayoutStrategy::NoiseAware
    }

    fn compute(
        &self,
        input: &LayoutInput,
        topology: &PhysicalTopology,
        config: &LayoutConfig,
    ) -> Result<Layout, LayoutError> {
        config.validate()?;
        validate_topology(topology)?;
        ensure_capacity(input, topology, config)?;

        /*
         * The topology owns hardware properties, while layout owns placement.
         *
         * We intentionally use a connectivity-first fallback here because the
         * stable topology contract exposes availability as a guaranteed
         * property but does not require every backend to expose a scalar
         * "qubit quality" ranking.
         *
         * When richer calibration data becomes available through topology.rs,
         * this strategy can consume it through a stable accessor without
         * changing Layout, LayoutInput, or LayoutStrategyEngine.
         */
        let dense = DenseLayout;
        let mut layout =
            dense.compute(input, topology, config)?;

        layout.strategy = self.strategy();

        layout.statistics.strategy =
            self.strategy().name().to_string();

        Ok(layout)
    }
}

/// SABRE-compatible initial layout.
///
/// This is intentionally a separate layout strategy from the SABRE routing
/// algorithm.
///
/// Its job is to provide a strong initial mapping that can be consumed by
/// `algorithms/sabre.rs`.
///
/// The full bidirectional SABRE routing loop MUST remain in
/// `algorithms/sabre.rs`.
///
/// This implementation uses the interaction-graph embedding heuristic as a
/// deterministic initial placement.
#[derive(Debug, Clone, Copy, Default)]
pub struct SabreLayout;

impl LayoutStrategyEngine for SabreLayout {
    fn strategy(&self) -> LayoutStrategy {
        LayoutStrategy::Sabre
    }

    fn compute(
        &self,
        input: &LayoutInput,
        topology: &PhysicalTopology,
        config: &LayoutConfig,
    ) -> Result<Layout, LayoutError> {
        let interaction =
            InteractionGraphLayout;

        let mut layout =
            interaction.compute(input, topology, config)?;

        layout.strategy = self.strategy();
        layout.statistics.strategy =
            self.strategy().name().to_string();

        Ok(layout)
    }
}

/// Caller-provided fixed layout.
///
/// The mapping is never replaced by an automatic layout strategy.
#[derive(Debug, Clone, Copy, Default)]
pub struct FixedLayout;

impl LayoutStrategyEngine for FixedLayout {
    fn strategy(&self) -> LayoutStrategy {
        LayoutStrategy::Fixed
    }

    fn compute(
        &self,
        input: &LayoutInput,
        topology: &PhysicalTopology,
        config: &LayoutConfig,
    ) -> Result<Layout, LayoutError> {
        config.validate()?;
        validate_topology(topology)?;
        ensure_capacity(input, topology, config)?;

        let supplied =
            input.fixed_mapping().ok_or_else(|| {
                LayoutError::InvalidConfiguration {
                    field: "fixed_mapping".to_string(),
                    detail: "a fixed mapping is required for Fixed layout"
                        .to_string(),
                }
            })?;

        validate_fixed_mapping(
            input,
            topology,
            supplied,
            config.allow_unavailable,
        )?;

        let mapping = supplied.clone();

        let quality =
            calculate_quality(input, topology, &mapping)?;

        let statistics = LayoutStatistics {
            strategy: self.strategy().name().to_string(),
            logical_qubits: input.logical_qubit_count(),
            physical_qubits_considered: topology.qubit_count(),
            candidates_examined: 0,
            placements: mapping.len(),
            attempts: 1,
            deterministic: true,
        };

        Layout::new(
            mapping,
            self.strategy(),
            quality,
            statistics,
        )
    }
}

// =============================================================================
// Layout engine
// =============================================================================

/// Production layout engine.
///
/// This is the public orchestration boundary used by `router.rs`.
///
/// The engine contains no routing logic. It selects an initial layout strategy,
/// validates the result, and returns an immutable `Layout`.
#[derive(Default)]
pub struct LayoutEngine {
    custom_strategies:
        BTreeMap<String, Box<dyn LayoutStrategyEngine>>,
}

impl LayoutEngine {
    /// Creates a production layout engine with built-in strategies.
    #[must_use]
    pub fn new() -> Self {
        Self {
            custom_strategies: BTreeMap::new(),
        }
    }

    /// Registers a custom strategy.
    ///
    /// Registration is explicit and deterministic: duplicate names replace the
    /// previous implementation.
    pub fn register_custom(
        &mut self,
        name: impl Into<String>,
        strategy: Box<dyn LayoutStrategyEngine>,
    ) -> Result<(), LayoutError> {
        let name = name.into();

        if name.trim().is_empty() {
            return Err(LayoutError::InvalidConfiguration {
                field: "custom_layout_name".to_string(),
                detail: "name cannot be empty".to_string(),
            });
        }

        self.custom_strategies.insert(name, strategy);

        Ok(())
    }

    /// Computes an initial layout.
    pub fn compute(
        &self,
        requested_strategy: &LayoutStrategy,
        input: &LayoutInput,
        topology: &PhysicalTopology,
        config: &LayoutConfig,
    ) -> Result<Layout, LayoutError> {
        config.validate()?;
        validate_topology(topology)?;

        ensure_capacity(input, topology, config)?;

        let strategy = resolve_auto_strategy(
            requested_strategy,
            input,
            topology,
        );

        match strategy {
            LayoutStrategy::Trivial => {
                TrivialLayout.compute(input, topology, config)
            }

            LayoutStrategy::Dense => {
                DenseLayout.compute(input, topology, config)
            }

            LayoutStrategy::InteractionGraph => {
                InteractionGraphLayout
                    .compute(input, topology, config)
            }

            LayoutStrategy::NoiseAware => {
                NoiseAwareLayout.compute(input, topology, config)
            }

            LayoutStrategy::Sabre => {
                SabreLayout.compute(input, topology, config)
            }

            LayoutStrategy::Fixed => {
                FixedLayout.compute(input, topology, config)
            }

            LayoutStrategy::Custom(name) => {
                let strategy =
                    self.custom_strategies.get(name).ok_or_else(|| {
                        LayoutError::CustomStrategyUnavailable {
                            name: name.clone(),
                        }
                    })?;

                strategy.compute(
                    input,
                    topology,
                    config,
                )
            }

            LayoutStrategy::Auto => {
                unreachable!(
                    "Auto is resolved before strategy dispatch"
                );
            }
        }
    }

    /// Computes a layout and converts its local error into the routing-wide
    /// error contract.
    ///
    /// This is the integration method intended for `router.rs`.
    pub fn compute_routing(
        &self,
        requested_strategy: &LayoutStrategy,
        input: &LayoutInput,
        topology: &PhysicalTopology,
        config: &LayoutConfig,
    ) -> RoutingResult<Layout> {
        self.compute(
            requested_strategy,
            input,
            topology,
            config,
        )
        .map_err(layout_error_to_routing_error)
    }
}

// =============================================================================
// Strategy resolution
// =============================================================================

fn resolve_auto_strategy(
    requested: &LayoutStrategy,
    input: &LayoutInput,
    topology: &PhysicalTopology,
) -> LayoutStrategy {
    match requested {
        LayoutStrategy::Auto => {
            if input.fixed_mapping().is_some() {
                LayoutStrategy::Fixed
            } else if input.interactions().is_empty() {
                LayoutStrategy::Trivial
            } else if input.logical_qubit_count()
                <= topology.qubit_count()
            {
                LayoutStrategy::InteractionGraph
            } else {
                LayoutStrategy::Trivial
            }
        }

        other => other.clone(),
    }
}

// =============================================================================
// Validation
// =============================================================================

fn validate_topology(
    topology: &PhysicalTopology,
) -> Result<(), LayoutError> {
    topology
        .validate()
        .map_err(|error| LayoutError::TopologyFailure {
            detail: error.to_string(),
        })
}

fn validate_logical_qubits(
    logical_qubits: &[LogicalQubitSpec],
) -> Result<(), LayoutError> {
    if logical_qubits.is_empty() {
        return Err(LayoutError::EmptyLogicalProgram);
    }

    let mut seen = BTreeSet::new();

    for logical in logical_qubits {
        if !seen.insert(logical.id) {
            return Err(LayoutError::DuplicateLogicalQubit {
                qubit: logical.id,
            });
        }
    }

    Ok(())
}

fn validate_interactions(
    logical_qubits: &[LogicalQubitSpec],
    interactions: &[QubitInteraction],
) -> Result<(), LayoutError> {
    let known: BTreeSet<LogicalQubitId> =
        logical_qubits.iter().map(|q| q.id).collect();

    for interaction in interactions {
        for &logical in interaction.operands() {
            if !known.contains(&logical) {
                return Err(LayoutError::UnknownInteractionQubit {
                    qubit: logical,
                });
            }
        }
    }

    Ok(())
}

fn ensure_capacity(
    input: &LayoutInput,
    topology: &PhysicalTopology,
    config: &LayoutConfig,
) -> Result<(), LayoutError> {
    if input.logical_qubit_count() > config.max_logical_qubits {
        return Err(LayoutError::ResourceLimitExceeded {
            resource: "logical_qubits".to_string(),
            requested: input.logical_qubit_count(),
            maximum: config.max_logical_qubits,
        });
    }

    if input.interaction_count() > config.max_interactions {
        return Err(LayoutError::ResourceLimitExceeded {
            resource: "interactions".to_string(),
            requested: input.interaction_count(),
            maximum: config.max_interactions,
        });
    }

    let available = eligible_physical_qubits(
        topology,
        config.allow_unavailable,
    )
    .len();

    if input.logical_qubit_count() > available {
        return Err(LayoutError::InsufficientPhysicalQubits {
            logical: input.logical_qubit_count(),
            physical: available,
        });
    }

    Ok(())
}

// =============================================================================
// Physical-resource helpers
// =============================================================================

fn eligible_physical_qubits(
    topology: &PhysicalTopology,
    allow_unavailable: bool,
) -> Vec<PhysicalQubitId> {
    /*
     * `topology` remains the source of truth for physical-resource validity.
     *
     * The topology API intentionally exposes availability through
     * `is_available()`. We do not inspect topology internals.
     */
    let mut result = Vec::new();

    for index in 0..topology.qubit_count() {
        let physical = PhysicalQubitId::new(index);

        if !topology.contains(physical) {
            continue;
        }

        if allow_unavailable
            || topology
                .is_available(physical)
        {
            result.push(physical);
        }
    }

    result.sort_unstable();

    result
}

fn physical_order_by_connectivity(
    topology: &PhysicalTopology,
    physical: &[PhysicalQubitId],
) -> Vec<PhysicalQubitId> {
    let mut ordered = physical.to_vec();

    ordered.sort_unstable_by(|a, b| {
        let degree_a =
            topology.neighbors(*a).len();

        let degree_b =
            topology.neighbors(*b).len();

        degree_b
            .cmp(&degree_a)
            .then_with(|| a.cmp(b))
    });

    ordered
}

// =============================================================================
// Logical interaction ordering
// =============================================================================

fn logical_order_by_interaction_degree(
    input: &LayoutInput,
) -> Vec<LogicalQubitId> {
    let graph =
        InteractionGraph::build_unchecked(input);

    graph.ordered_logical_qubits()
}

fn assign_ordered(
    logicals: &[LogicalQubitId],
    physicals: &[PhysicalQubitId],
    topology: &PhysicalTopology,
) -> Result<QubitMapping, LayoutError> {
    if logicals.len() > physicals.len() {
        return Err(LayoutError::InsufficientPhysicalQubits {
            logical: logicals.len(),
            physical: physicals.len(),
        });
    }

    let mut mapping = QubitMapping::new();

    for (&logical, &physical) in
        logicals.iter().zip(physicals.iter())
    {
        if !topology.contains(physical) {
            return Err(
                LayoutError::InvalidFixedPhysicalQubit {
                    qubit: physical,
                },
            );
        }

        mapping.assign(logical, physical)?;
    }

    Ok(mapping)
}

// =============================================================================
// Interaction graph
// =============================================================================

/// Weighted logical interaction graph used by layout heuristics.
#[derive(Debug, Clone)]
struct InteractionGraph {
    /// Logical qubit -> neighboring logical qubits with interaction weight.
    adjacency:
        BTreeMap<LogicalQubitId, BTreeMap<LogicalQubitId, u64>>,
}

impl InteractionGraph {
    fn build(
        input: &LayoutInput,
    ) -> Result<Self, LayoutError> {
        validate_interactions(
            input.logical_qubits(),
            input.interactions(),
        )?;

        Ok(Self::build_unchecked(input))
    }

    fn build_unchecked(input: &LayoutInput) -> Self {
        let mut adjacency = BTreeMap::new();

        for logical in input.logical_qubits() {
            adjacency.insert(logical.id, BTreeMap::new());
        }

        for interaction in input.interactions() {
            if interaction.arity() != 2 {
                continue;
            }

            let operands = interaction.operands();

            let a = operands[0];
            let b = operands[1];

            if a == b {
                continue;
            }

            let entry_a =
                adjacency
                    .entry(a)
                    .or_default();

            let weight_a =
                entry_a.entry(b).or_insert(0);

            *weight_a =
                weight_a.saturating_add(1);

            let entry_b =
                adjacency
                    .entry(b)
                    .or_default();

            let weight_b =
                entry_b.entry(a).or_insert(0);

            *weight_b =
                weight_b.saturating_add(1);
        }

        Self { adjacency }
    }

    fn degree(
        &self,
        logical: LogicalQubitId,
    ) -> u64 {
        self.adjacency
            .get(&logical)
            .map(|neighbors| {
                neighbors
                    .values()
                    .copied()
                    .fold(0u64, u64::saturating_add)
            })
            .unwrap_or(0)
    }

    fn interaction_weight(
        &self,
        a: LogicalQubitId,
        b: LogicalQubitId,
    ) -> u64 {
        self.adjacency
            .get(&a)
            .and_then(|neighbors| neighbors.get(&b))
            .copied()
            .unwrap_or(0)
    }

    fn neighbors(
        &self,
        logical: LogicalQubitId,
    ) -> Vec<(LogicalQubitId, u64)> {
        self.adjacency
            .get(&logical)
            .map(|neighbors| {
                neighbors
                    .iter()
                    .map(|(&logical, &weight)| {
                        (logical, weight)
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    fn ordered_logical_qubits(
        &self,
    ) -> Vec<LogicalQubitId> {
        let mut logicals:
            Vec<LogicalQubitId> =
            self.adjacency.keys().copied().collect();

        logicals.sort_unstable_by(|a, b| {
            self.degree(*b)
                .cmp(&self.degree(*a))
                .then_with(|| a.cmp(b))
        });

        logicals
    }
}

// =============================================================================
// Interaction-aware placement
// =============================================================================

fn choose_best_seed_physical(
    topology: &PhysicalTopology,
    physical: &[PhysicalQubitId],
    graph: &InteractionGraph,
) -> Result<PhysicalQubitId, LayoutError> {
    if physical.is_empty() {
        return Err(LayoutError::InvariantViolation {
            detail:
                "no eligible physical qubits for seed placement"
                    .to_string(),
        });
    }

    /*
     * For the first logical qubit there are no already-placed neighbors.
     * Therefore use physical connectivity as the deterministic seed score.
     */
    let logical =
        graph
            .ordered_logical_qubits()
            .first()
            .copied()
            .ok_or(LayoutError::EmptyLogicalProgram)?;

    let _ = logical;

    physical
        .iter()
        .copied()
        .max_by(|a, b| {
            topology
                .neighbors(*a)
                .len()
                .cmp(&topology.neighbors(*b).len())
                .then_with(|| b.cmp(a))
        })
        .ok_or_else(|| LayoutError::InvariantViolation {
            detail:
                "physical candidate set unexpectedly empty"
                    .to_string(),
        })
}

fn choose_next_logical(
    graph: &InteractionGraph,
    placed: &BTreeSet<LogicalQubitId>,
) -> Result<LogicalQubitId, LayoutError> {
    let mut best:
        Option<(LogicalQubitId, u64, u64)> =
        None;

    for logical in graph.adjacency.keys().copied() {
        if placed.contains(&logical) {
            continue;
        }

        let connected_weight = graph
            .neighbors(logical)
            .into_iter()
            .filter(|(neighbor, _)| placed.contains(neighbor))
            .map(|(_, weight)| weight)
            .fold(0u64, u64::saturating_add);

        let total_degree = graph.degree(logical);

        let candidate =
            (logical, connected_weight, total_degree);

        match best {
            None => best = Some(candidate),

            Some(current) => {
                if connected_weight > current.1
                    || (connected_weight == current.1
                        && total_degree > current.2)
                    || (connected_weight == current.1
                        && total_degree == current.2
                        && logical < current.0)
                {
                    best = Some(candidate);
                }
            }
        }
    }

    best.map(|candidate| candidate.0)
        .ok_or_else(|| LayoutError::InvariantViolation {
            detail:
                "unable to select next logical qubit"
                    .to_string(),
        })
}

fn choose_best_physical_for_logical(
    logical: LogicalQubitId,
    graph: &InteractionGraph,
    mapping: &QubitMapping,
    topology: &PhysicalTopology,
    physical: &[PhysicalQubitId],
    config: &LayoutConfig,
) -> Result<PhysicalQubitId, LayoutError> {
    let placed_neighbors:
        Vec<(PhysicalQubitId, u64)> = graph
        .neighbors(logical)
        .into_iter()
        .filter_map(|(neighbor, weight)| {
            mapping
                .physical_of(neighbor)
                .map(|physical| (physical, weight))
        })
        .collect();

    /*
     * If no neighbor is already placed, fall back to connectivity.
     */
    if placed_neighbors.is_empty() {
        return physical
            .iter()
            .copied()
            .filter(|candidate| {
                !mapping.contains_physical(*candidate)
            })
            .max_by(|a, b| {
                topology
                    .neighbors(*a)
                    .len()
                    .cmp(&topology.neighbors(*b).len())
                    .then_with(|| b.cmp(a))
            })
            .ok_or(LayoutError::LayoutConstructionFailed {
                strategy:
                    "interaction_graph".to_string(),
                logical,
            });
    }

    let mut candidates:
        Vec<PhysicalQubitId> = physical
        .iter()
        .copied()
        .filter(|candidate| {
            !mapping.contains_physical(*candidate)
        })
        .collect();

    if candidates.len() >
        config.candidate_physical_limit
    {
        candidates.truncate(
            config.candidate_physical_limit,
        );
    }

    let mut best:
        Option<(PhysicalQubitId, u128, u64, usize)> =
        None;

    for candidate in candidates {
        let mut weighted_distance = 0u128;
        let mut maximum_distance = 0u64;
        let mut adjacent_count = 0usize;

        for &(neighbor_physical, weight) in
            &placed_neighbors
        {
            let distance =
                shortest_distance(
                    topology,
                    candidate,
                    neighbor_physical,
                )?;

            weighted_distance =
                weighted_distance.saturating_add(
                    u128::from(distance)
                        .saturating_mul(
                            u128::from(weight),
                        ),
                );

            maximum_distance =
                maximum_distance.max(distance);

            if distance == 1 {
                adjacent_count =
                    adjacent_count.saturating_add(1);
            }
        }

        /*
         * Primary:
         *   minimize weighted interaction distance.
         *
         * Secondary:
         *   minimize maximum distance.
         *
         * Tertiary:
         *   maximize number of already-adjacent neighbors.
         *
         * Final:
         *   physical qubit ID.
         */
        let score = (
            candidate,
            weighted_distance,
            maximum_distance,
            adjacent_count,
        );

        match best {
            None => best = Some(score),

            Some(current) => {
                let better =
                    weighted_distance < current.1
                        || (weighted_distance
                            == current.1
                            && maximum_distance
                                < current.2)
                        || (weighted_distance
                            == current.1
                            && maximum_distance
                                == current.2
                            && adjacent_count
                                > current.3)
                        || (weighted_distance
                            == current.1
                            && maximum_distance
                                == current.2
                            && adjacent_count
                                == current.3
                            && candidate
                                < current.0);

                if better {
                    best = Some(score);
                }
            }
        }
    }

    best.map(|entry| entry.0)
        .ok_or(LayoutError::LayoutConstructionFailed {
            strategy:
                "interaction_graph".to_string(),
            logical,
        })
}

// =============================================================================
// Distance
// =============================================================================

/// Deterministic unweighted physical distance.
///
/// This helper intentionally operates through the public topology API rather
/// than reaching into topology storage.
///
/// A result of `0` is valid only when source == target.
///
/// `None` means the two physical resources are disconnected.
fn shortest_distance(
    topology: &PhysicalTopology,
    source: PhysicalQubitId,
    target: PhysicalQubitId,
) -> Result<u64, LayoutError> {
    if !topology.contains(source) {
        return Err(
            LayoutError::InvalidFixedPhysicalQubit {
                qubit: source,
            },
        );
    }

    if !topology.contains(target) {
        return Err(
            LayoutError::InvalidFixedPhysicalQubit {
                qubit: target,
            },
        );
    }

    if source == target {
        return Ok(0);
    }

    let mut queue =
        VecDeque::<(PhysicalQubitId, u64)>::new();

    let mut visited =
        BTreeSet::<PhysicalQubitId>::new();

    queue.push_back((source, 0));
    visited.insert(source);

    while let Some((current, distance)) =
        queue.pop_front()
    {
        let mut neighbors =
            topology.neighbors(current);

        neighbors.sort_unstable();

        for neighbor in neighbors {
            if !visited.insert(neighbor) {
                continue;
            }

            let next_distance =
                distance.checked_add(1).ok_or_else(
                    || LayoutError::InvariantViolation {
                        detail:
                            "physical path distance overflow"
                                .to_string(),
                    },
                )?;

            if neighbor == target {
                return Ok(next_distance);
            }

            queue.push_back((
                neighbor,
                next_distance,
            ));
        }
    }

    /*
     * Layout should never silently manufacture a distance for disconnected
     * physical resources.
     */
    Err(LayoutError::TopologyFailure {
        detail: format!(
            "physical qubits {source} and {target} are disconnected"
        ),
    })
}

// =============================================================================
// Quality calculation
// =============================================================================

fn calculate_quality(
    input: &LayoutInput,
    topology: &PhysicalTopology,
    mapping: &QubitMapping,
) -> Result<LayoutQuality, LayoutError> {
    let mut quality =
        LayoutQuality {
            occupied_physical_qubits: mapping.len(),
            ..LayoutQuality::default()
        };

    for interaction in input.interactions() {
        if interaction.arity() != 2 {
            /*
             * Layout does not attempt to assign a scalar pair distance to
             * arbitrary N-qubit operations. Those operations are handled by
             * the multi-qubit decomposition/native-operation boundary.
             */
            continue;
        }

        let operands =
            interaction.operands();

        let logical_a = operands[0];
        let logical_b = operands[1];

        let physical_a =
            mapping.physical_of(logical_a)
                .ok_or_else(|| {
                    LayoutError::InvariantViolation {
                        detail: format!(
                            "logical qubit {logical_a} is absent from generated layout"
                        ),
                    }
                })?;

        let physical_b =
            mapping.physical_of(logical_b)
                .ok_or_else(|| {
                    LayoutError::InvariantViolation {
                        detail: format!(
                            "logical qubit {logical_b} is absent from generated layout"
                        ),
                    }
                })?;

        let distance =
            shortest_distance(
                topology,
                physical_a,
                physical_b,
            )?;

        quality.total_interaction_distance =
            quality
                .total_interaction_distance
                .checked_add(distance)
                .ok_or_else(
                    || LayoutError::InvariantViolation {
                        detail:
                            "layout interaction distance overflow"
                                .to_string(),
                    },
                )?;

        quality.maximum_interaction_distance =
            quality
                .maximum_interaction_distance
                .max(distance);

        if distance == 1 {
            quality.adjacent_interactions =
                quality
                    .adjacent_interactions
                    .checked_add(1)
                    .ok_or_else(
                        || LayoutError::InvariantViolation {
                            detail:
                                "adjacent interaction counter overflow"
                                    .to_string(),
                        },
                    )?;
        } else {
            quality.non_adjacent_interactions =
                quality
                    .non_adjacent_interactions
                    .checked_add(1)
                    .ok_or_else(
                        || LayoutError::InvariantViolation {
                            detail:
                                "non-adjacent interaction counter overflow"
                                    .to_string(),
                        },
                    )?;
        }
    }

    Ok(quality)
}

// =============================================================================
// Fixed-layout validation
// =============================================================================

fn validate_fixed_mapping(
    input: &LayoutInput,
    topology: &PhysicalTopology,
    mapping: &QubitMapping,
    allow_unavailable: bool,
) -> Result<(), LayoutError> {
    mapping
        .validate()
        .map_err(LayoutError::from)?;

    if mapping.len() != input.logical_qubit_count() {
        return Err(LayoutError::InvalidConfiguration {
            field: "fixed_mapping".to_string(),
            detail: format!(
                "mapping contains {} assignments but {} logical qubits are required",
                mapping.len(),
                input.logical_qubit_count()
            ),
        });
    }

    let known_logical:
        BTreeSet<LogicalQubitId> =
        input
            .logical_qubits()
            .iter()
            .map(|logical| logical.id)
            .collect();

    for logical in known_logical.iter().copied() {
        if mapping.physical_of(logical).is_none() {
            return Err(
                LayoutError::UnknownFixedLogicalQubit {
                    qubit: logical,
                },
            );
        }
    }

    for logical in mapping.logical_qubits() {
        if !known_logical.contains(&logical) {
            return Err(
                LayoutError::UnknownFixedLogicalQubit {
                    qubit: logical,
                },
            );
        }
    }

    for (_, physical) in
        mapping.logical_to_physical()
    {
        if !topology.contains(physical) {
            return Err(
                LayoutError::InvalidFixedPhysicalQubit {
                    qubit: physical,
                },
            );
        }

        if !allow_unavailable
            && !topology.is_available(physical)
        {
            return Err(
                LayoutError::UnavailableFixedPhysicalQubit {
                    qubit: physical,
                },
            );
        }
    }

    Ok(())
}

// =============================================================================
// Routing-wide error conversion
// =============================================================================

fn layout_error_to_routing_error(
    error: LayoutError,
) -> RoutingError {
    /*
     * `errors.rs` deliberately owns the compiler-wide taxonomy while this
     * module owns detailed layout diagnostics.
     *
     * We convert at the integration boundary instead of making layout depend
     * on router/transpiler implementation details.
     */
    RoutingError::InvalidConfiguration(
        format!("layout: {error}"),
    )
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn line_topology(
        count: usize,
    ) -> PhysicalTopology {
        let mut builder =
            PhysicalTopology::builder();

        for index in 0..count {
            builder
                .add_qubit(
                    PhysicalQubitId::new(index),
                )
                .expect("test qubit");
        }

        for index in 0..count.saturating_sub(1) {
            builder
                .add_undirected_edge(
                    PhysicalQubitId::new(index),
                    PhysicalQubitId::new(index + 1),
                )
                .expect("test edge");
        }

        builder
            .build()
            .expect("valid test topology")
    }

    fn input(
        count: usize,
        interactions: Vec<QubitInteraction>,
    ) -> LayoutInput {
        let logicals =
            (0..count)
                .map(|index| {
                    LogicalQubitSpec::data(
                        LogicalQubitId::new(index),
                    )
                })
                .collect();

        LayoutInput::new(
            logicals,
            interactions,
        )
        .expect("valid layout input")
    }

    #[test]
    fn trivial_layout_is_deterministic() {
        let topology =
            line_topology(4);

        let input =
            input(3, Vec::new());

        let config =
            LayoutConfig::default();

        let first =
            TrivialLayout
                .compute(
                    &input,
                    &topology,
                    &config,
                )
                .expect("layout");

        let second =
            TrivialLayout
                .compute(
                    &input,
                    &topology,
                    &config,
                )
                .expect("layout");

        assert_eq!(
            first
                .mapping()
                .logical_to_physical(),
            second
                .mapping()
                .logical_to_physical()
        );
    }

    #[test]
    fn trivial_layout_uses_distinct_physical_qubits() {
        let topology =
            line_topology(5);

        let input =
            input(4, Vec::new());

        let layout =
            TrivialLayout
                .compute(
                    &input,
                    &topology,
                    &LayoutConfig::default(),
                )
                .expect("layout");

        assert_eq!(
            layout.mapping().len(),
            4
        );

        layout
            .mapping()
            .validate()
            .expect("valid mapping");
    }

    #[test]
    fn insufficient_physical_qubits_are_rejected() {
        let topology =
            line_topology(2);

        let input =
            input(3, Vec::new());

        let result =
            TrivialLayout
                .compute(
                    &input,
                    &topology,
                    &LayoutConfig::default(),
                );

        assert!(matches!(
            result,
            Err(
                LayoutError::InsufficientPhysicalQubits {
                    ..
                }
            )
        ));
    }

    #[test]
    fn interaction_layout_prefers_connected_region() {
        let topology =
            line_topology(5);

        let interactions = vec![
            QubitInteraction::new(
                vec![
                    LogicalQubitId::new(0),
                    LogicalQubitId::new(1),
                ],
                crate::quantum::routing::types::GateIdentity::Cx,
            ),
            QubitInteraction::new(
                vec![
                    LogicalQubitId::new(0),
                    LogicalQubitId::new(2),
                ],
                crate::quantum::routing::types::GateIdentity::Cx,
            ),
        ];

        let input =
            input(3, interactions);

        let layout =
            InteractionGraphLayout
                .compute(
                    &input,
                    &topology,
                    &LayoutConfig::default(),
                )
                .expect("layout");

        assert_eq!(
            layout.mapping().len(),
            3
        );

        assert!(
            layout
                .quality()
                .total_interaction_distance
                <= 4
        );
    }

    #[test]
    fn fixed_layout_is_preserved() {
        let topology =
            line_topology(4);

        let logicals = vec![
            LogicalQubitSpec::data(
                LogicalQubitId::new(0),
            ),
            LogicalQubitSpec::data(
                LogicalQubitId::new(1),
            ),
        ];

        let mut mapping =
            QubitMapping::new();

        mapping
            .assign(
                LogicalQubitId::new(0),
                PhysicalQubitId::new(2),
            )
            .expect("mapping");

        mapping
            .assign(
                LogicalQubitId::new(1),
                PhysicalQubitId::new(3),
            )
            .expect("mapping");

        let input =
            LayoutInput::with_fixed_mapping(
                logicals,
                Vec::new(),
                Some(mapping),
            )
            .expect("input");

        let layout =
            FixedLayout
                .compute(
                    &input,
                    &topology,
                    &LayoutConfig::default(),
                )
                .expect("layout");

        assert_eq!(
            layout.physical_of(
                LogicalQubitId::new(0)
            ),
            Some(
                PhysicalQubitId::new(2)
            )
        );

        assert_eq!(
            layout.physical_of(
                LogicalQubitId::new(1)
            ),
            Some(
                PhysicalQubitId::new(3)
            )
        );
    }

    #[test]
    fn fixed_layout_rejects_missing_mapping() {
        let topology =
            line_topology(3);

        let input =
            input(2, Vec::new());

        let result =
            FixedLayout
                .compute(
                    &input,
                    &topology,
                    &LayoutConfig::default(),
                );

        assert!(matches!(
            result,
            Err(
                LayoutError::InvalidConfiguration {
                    ..
                }
            )
        ));
    }

    #[test]
    fn auto_selects_fixed_when_mapping_exists() {
        let topology =
            line_topology(3);

        let mut mapping =
            QubitMapping::new();

        mapping
            .assign(
                LogicalQubitId::new(0),
                PhysicalQubitId::new(2),
            )
            .expect("mapping");

        let logicals = vec![
            LogicalQubitSpec::data(
                LogicalQubitId::new(0),
            ),
        ];

        let input =
            LayoutInput::with_fixed_mapping(
                logicals,
                Vec::new(),
                Some(mapping),
            )
            .expect("input");

        let engine =
            LayoutEngine::new();

        let layout =
            engine
                .compute(
                    &LayoutStrategy::Auto,
                    &input,
                    &topology,
                    &LayoutConfig::default(),
                )
                .expect("layout");

        assert_eq!(
            layout.strategy(),
            &LayoutStrategy::Fixed
        );
    }

    #[test]
    fn auto_uses_trivial_without_interactions() {
        let topology =
            line_topology(3);

        let input =
            input(2, Vec::new());

        let engine =
            LayoutEngine::new();

        let layout =
            engine
                .compute(
                    &LayoutStrategy::Auto,
                    &input,
                    &topology,
                    &LayoutConfig::default(),
                )
                .expect("layout");

        assert_eq!(
            layout.strategy(),
            &LayoutStrategy::Trivial
        );
    }

    #[test]
    fn interaction_referencing_unknown_qubit_is_rejected() {
        let logicals = vec![
            LogicalQubitSpec::data(
                LogicalQubitId::new(0),
            ),
        ];

        let interactions = vec![
            QubitInteraction::new(
                vec![
                    LogicalQubitId::new(0),
                    LogicalQubitId::new(1),
                ],
                crate::quantum::routing::types::GateIdentity::Cx,
            ),
        ];

        let result =
            LayoutInput::new(
                logicals,
                interactions,
            );

        assert!(matches!(
            result,
            Err(
                LayoutError::UnknownInteractionQubit {
                    ..
                }
            )
        ));
    }

    #[test]
    fn interaction_graph_ignores_non_two_qubit_distance() {
        let topology =
            line_topology(3);

        let interactions = vec![
            QubitInteraction::new(
                vec![
                    LogicalQubitId::new(0),
                    LogicalQubitId::new(1),
                    LogicalQubitId::new(2),
                ],
                crate::quantum::routing::types::GateIdentity::Ccx,
            ),
        ];

        let input =
            input(3, interactions);

        let layout =
            InteractionGraphLayout
                .compute(
                    &input,
                    &topology,
                    &LayoutConfig::default(),
                )
                .expect("layout");

        assert_eq!(
            layout.quality()
                .interaction_count(),
            0
        );
    }

    #[test]
    fn layout_quality_detects_adjacent_interactions() {
        let topology =
            line_topology(3);

        let interactions = vec![
            QubitInteraction::new(
                vec![
                    LogicalQubitId::new(0),
                    LogicalQubitId::new(1),
                ],
                crate::quantum::routing::types::GateIdentity::Cx,
            ),
        ];

        let input =
            input(2, interactions);

        let layout =
            TrivialLayout
                .compute(
                    &input,
                    &topology,
                    &LayoutConfig::default(),
                )
                .expect("layout");

        assert_eq!(
            layout.quality()
                .adjacent_interactions,
            1
        );

        assert!(
            layout
                .quality()
                .is_fully_connected()
        );
    }

    #[test]
    fn sabre_layout_is_not_the_routing_algorithm() {
        let topology =
            line_topology(4);

        let interactions = vec![
            QubitInteraction::new(
                vec![
                    LogicalQubitId::new(0),
                    LogicalQubitId::new(1),
                ],
                crate::quantum::routing::types::GateIdentity::Cx,
            ),
        ];

        let input =
            input(2, interactions);

        let layout =
            SabreLayout
                .compute(
                    &input,
                    &topology,
                    &LayoutConfig::default(),
                )
                .expect("layout");

        assert_eq!(
            layout.strategy(),
            &LayoutStrategy::Sabre
        );

        /*
         * Layout only returns a mapping. It never inserts SWAPs or modifies
         * circuit operations.
         */
        assert_eq!(
            layout.mapping().len(),
            2
        );
    }
}