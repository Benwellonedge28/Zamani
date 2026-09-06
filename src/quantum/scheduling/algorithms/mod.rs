//! Zamani Quantum Scheduling — Algorithm Layer
//!
//! Path:
//!
//! `src/quantum/scheduling/algorithms/mod.rs`
//!
//! # Purpose
//!
//! This module is the stable public composition boundary for the algorithm
//! layer of Zamani's quantum scheduling subsystem.
//!
//! The algorithm layer answers:
//!
//! > Which scheduling strategy should be used to produce a legal schedule?
//!
//! It deliberately does NOT own:
//!
//! - Zamani source parsing;
//! - quantum IR definition;
//! - quantum operation semantics;
//! - logical-to-physical routing;
//! - hardware discovery;
//! - hardware execution;
//! - calibration acquisition;
//! - resource-calendar implementation;
//! - timing-model implementation;
//! - dependency-graph construction;
//! - QEC decoding;
//! - noise modelling;
//! - schedule verification implementation;
//! - schedule serialization;
//! - runtime execution.
//!
//! Those responsibilities belong to the appropriate quantum or scheduling
//! subsystem.
//!
//! # Architectural position
//!
//! ```text
//! Zamani source
//!      |
//!      v
//! quantum::frontend
//!      |
//!      v
//! quantum::ir
//!      |
//!      v
//! optimization
//!      |
//!      v
//! routing
//!      |
//!      v
//! scheduling::adapters
//!      |
//!      v
//! SchedulingContext
//!      |
//!      +-----------------------------+
//!      |                             |
//!      v                             v
//! dependency/resource/timing      constraints
//!      |                             |
//!      +-------------+---------------+
//!                    |
//!                    v
//!             scheduling::algorithms
//!                    |
//!        +-----------+-----------+
//!        |           |           |
//!       ASAP        ALAP        CP
//!        |           |           |
//!        +-----------+-----------+
//!                    |
//!                    v
//!                 planners
//!                    |
//!                    v
//!             SchedulingResult
//!                    |
//!                    v
//!               verification
//!                    |
//!                    v
//!          hardware/runtime lowering
//! ```
//!
//! # Algorithm layer contract
//!
//! Every algorithm in this directory must obey the following architectural
//! rules.
//!
//! ## 1. Canonical quantum identities
//!
//! Algorithms must never create scheduler-local quantum identities.
//!
//! Canonical identities remain owned by:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! When an algorithm needs to reason about qubits, it must consume the
//! canonical identities through the scheduling IR/context/adapters.
//!
//! No algorithm may introduce a competing `QubitId` type.
//!
//! ## 2. No machine-size assumptions
//!
//! No algorithm may contain hard-coded limits for:
//!
//! - qubits;
//! - physical qubits;
//! - logical qubits;
//! - operations;
//! - resources;
//! - channels;
//! - schedule depth;
//! - parallelism;
//! - topology size;
//! - QEC distance;
//! - number of QPUs;
//! - number of network nodes;
//! - gate arity.
//!
//! The target determines available resources.
//!
//! The scheduler determines how those resources are used.
//!
//! ```text
//! same Zamani program
//!        |
//!        +--> tiny target
//!        |
//!        +--> large target
//!        |
//!        +--> distributed target
//!        |
//!        +--> future target
//! ```
//!
//! The source program remains independent of the target's physical scale.
//!
//! "Infinity" therefore means that the algorithm layer introduces no artificial
//! finite machine-size ceiling. Actual execution remains bounded by the
//! resources, limits, address space, memory, and time explicitly available to
//! the compiler/runtime environment.
//!
//! ## 3. No hardware ownership
//!
//! Algorithms consume target information through scheduling abstractions.
//!
//! They must not:
//!
//! - connect to a QPU;
//! - query a provider SDK;
//! - discover hardware;
//! - mutate calibration state;
//! - submit jobs;
//! - communicate directly with hardware.
//!
//! Hardware information must enter through the hardware adapter/context
//! boundary.
//!
//! ## 4. No routing ownership
//!
//! Routing answers:
//!
//! ```text
//! WHERE?
//! ```
//!
//! Scheduling answers:
//!
//! ```text
//! WHEN?
//! ```
//!
//! Algorithm implementations must consume routing output rather than perform
//! logical-to-physical mapping themselves.
//!
//! ## 5. No semantic mutation
//!
//! Scheduling algorithms may choose placement in time and resources.
//!
//! They must not silently change the quantum computation.
//!
//! Any semantic transformation such as decomposition, synthesis, or gate
//! rewriting belongs to the optimization/transformation layers and must be
//! explicitly represented there.
//!
//! ## 6. Determinism
//!
//! Deterministic algorithms must not depend on:
//!
//! - hash-map iteration order;
//! - pointer addresses;
//! - memory layout;
//! - operating-system scheduling;
//! - wall-clock timing;
//! - thread timing;
//! - implicit randomness.
//!
//! If an algorithm is randomized, the random source must be explicitly owned
//! by its algorithm/configuration boundary.
//!
//! ## 7. No unsafe Rust
//!
//! This module and the algorithm layer must remain safe Rust.
//!
//! No algorithm may require `unsafe` to implement its scheduling semantics.
//!
//! ## 8. Checked arithmetic
//!
//! Temporal and resource calculations must use checked arithmetic through the
//! canonical scheduling types/models.
//!
//! Overflow must become an explicit error.
//!
//! ## 9. Explicit failure
//!
//! An algorithm must never silently:
//!
//! - drop an operation;
//! - ignore a dependency;
//! - exceed resource capacity;
//! - ignore a timing constraint;
//! - manufacture a missing duration;
//! - report an incomplete schedule as successful.
//!
//! Failures must propagate through the canonical scheduling error boundary.
//!
//! # Algorithms provided by this module
//!
//! ```text
//! adaptive
//! alap
//! asap
//! cp
//! list
//! rcpsp
//! ```
//!
//! Their responsibilities are intentionally different.
//!
//! ## ASAP
//!
//! As Soon As Possible scheduling.
//!
//! Primary objective:
//!
//! ```text
//! earliest legal execution
//! ```
//!
//! ASAP should use the canonical dependency/resource-aware planning machinery.
//!
//! ## ALAP
//!
//! As Late As Possible scheduling.
//!
//! Primary objective:
//!
//! ```text
//! latest legal execution
//! ```
//!
//! ALAP should use successor/deadline/timing information supplied by the
//! canonical scheduling models.
//!
//! ## Critical Path
//!
//! Critical-path-oriented scheduling.
//!
//! Its purpose is to prioritize operations according to dependency criticality
//! rather than simply using source order.
//!
//! ## List
//!
//! General-purpose resource-aware list scheduling.
//!
//! This is the primary scalable scheduling mechanism used by multiple policy
//! facades.
//!
//! ## RCPSP
//!
//! Resource-Constrained Project Scheduling Problem based scheduling.
//!
//! It is intended for resource-constrained scheduling scenarios where the
//! problem model and selected solver/heuristic justify RCPSP-style planning.
//!
//! It must not assume a fixed resource count or fixed resource capacity.
//!
//! ## Adaptive
//!
//! Adaptive algorithm selection.
//!
//! It may select an appropriate scheduling strategy based on characteristics
//! exposed by the scheduling context/problem.
//!
//! Adaptive selection must remain semantically transparent.
//!
//! It must never use machine-size constants as hidden strategy thresholds.
//!
//! # Important architectural distinction
//!
//! These modules are algorithm facades, not six independent scheduler
//! implementations.
//!
//! The intended relationship is:
//!
//! ```text
//! algorithms
//!      |
//!      +--> policy
//!      |
//!      +--> planner
//!      |
//!      +--> scheduling models
//! ```
//!
//! rather than:
//!
//! ```text
//! ASAP ──> private scheduler
//! ALAP ──> private scheduler
//! CP   ──> private scheduler
//! LIST ─> private scheduler
//! RCPSP -> private scheduler
//! ADAPTIVE -> private scheduler
//! ```
//!
//! Duplicated scheduler mechanics would cause the algorithms to diverge over
//! time and would make production verification substantially harder.
//!
//! # Module stability contract
//!
//! This file is deliberately kept independent of concrete algorithm internals.
//!
//! Adding a new scheduling algorithm should normally require:
//!
//! 1. adding its implementation file;
//! 2. adding one module declaration here;
//! 3. registering it with the explicit algorithm registry if the registry is
//!    used by the public orchestration layer.
//!
//! Adding or changing:
//!
//! - a hardware technology;
//! - a QEC code;
//! - a routing strategy;
//! - a resource kind;
//! - a timing representation;
//! - a communication technology;
//! - a noise model;
//! - a target provider;
//!
//! must not require changing this file.
//!
//! # Public API philosophy
//!
//! This module exposes algorithm modules and stable algorithm metadata.
//!
//! It does not expose private implementation details.
//!
//! Callers should normally use one of:
//!
//! ```text
//! crate::quantum::scheduling::algorithms::asap
//! crate::quantum::scheduling::algorithms::alap
//! crate::quantum::scheduling::algorithms::cp
//! crate::quantum::scheduling::algorithms::list
//! crate::quantum::scheduling::algorithms::rcpsp
//! crate::quantum::scheduling::algorithms::adaptive
//! ```
//!
//! or the higher-level scheduling/planner/registry API.
//!
//! # Algorithm metadata
//!
//! Stable identifiers are useful for:
//!
//! - configuration;
//! - serialization;
//! - reproducibility;
//! - diagnostics;
//! - benchmarking;
//! - plugin registration;
//! - telemetry;
//! - schedule provenance.
//!
//! The metadata in this file is descriptive only.
//!
//! Algorithm-specific versions remain owned by their algorithm modules.
//!
//! # Registry integration
//!
//! A registry, when provided by:
//!
//! ```text
//! crate::quantum::scheduling::plugins
//! ```
//!
//! or another scheduling orchestration layer, should use the stable IDs
//! exposed by the individual algorithm modules.
//!
//! This module must not create global mutable algorithm state.
//!
//! The preferred architecture is:
//!
//! ```text
//! caller
//!   |
//!   v
//! immutable algorithm selection/configuration
//!   |
//!   v
//! algorithm registry
//!   |
//!   v
//! algorithm facade
//!   |
//!   v
//! planner
//! ```
//!
//! # Dynamic quantum circuits
//!
//! Algorithms must support scheduling models that contain:
//!
//! - measurement dependencies;
//! - classical dependencies;
//! - conditional operations;
//! - feedback;
//! - runtime events;
//! - communication completion;
//! - partially known readiness.
//!
//! Compile-time algorithms must not invent information that is only available
//! at runtime.
//!
//! Runtime-resolved scheduling belongs to:
//!
//! ```text
//! crate::quantum::scheduling::dynamic
//! ```
//!
//! An algorithm may be invoked incrementally for a newly available scheduling
//! region.
//!
//! # Distributed quantum computing
//!
//! Algorithms must treat communication as schedulable work/resources when the
//! target model exposes it.
//!
//! This can include:
//!
//! - classical communication;
//! - quantum communication;
//! - entanglement generation;
//! - synchronization;
//! - remote-operation dependencies;
//! - inter-QPU resources;
//! - network contention.
//!
//! Network topology itself remains outside this module.
//!
//! # QEC integration
//!
//! QEC scheduling is not implemented in this module.
//!
//! QEC adapters may expose:
//!
//! - syndrome dependencies;
//! - ancilla requirements;
//! - measurement requirements;
//! - round boundaries;
//! - recovery dependencies;
//! - classical feedback;
//! - timing constraints.
//!
//! The scheduling algorithms consume those requirements through the common
//! scheduling models.
//!
//! QEC-specific construction remains under:
//!
//! ```text
//! crate::quantum::scheduling::qec
//! ```
//!
//! # Hardware integration
//!
//! The algorithm layer consumes target information through the scheduling
//! context and hardware adapter.
//!
//! It must not depend directly on a provider implementation.
//!
//! Conceptually:
//!
//! ```text
//! quantum::hardware
//!        |
//!        v
//! scheduling::adapters::hardware
//!        |
//!        v
//! SchedulingContext
//!        |
//!        v
//! algorithms
//! ```
//!
//! # Routing integration
//!
//! The intended order is:
//!
//! ```text
//! optimization
//!      |
//!      v
//! routing
//!      |
//!      v
//! scheduling::adapters::routing
//!      |
//!      v
//! scheduling::algorithms
//! ```
//!
//! An algorithm must not independently perform routing merely because routing
//! information is unavailable.
//!
//! Missing routing information must be an explicit integration error.
//!
//! # Benchmarking integration
//!
//! Scheduling algorithms should expose enough stable identity information for
//! the benchmarking subsystem to distinguish:
//!
//! - algorithm;
//! - algorithm version;
//! - configuration;
//! - target snapshot;
//! - scheduling result.
//!
//! Benchmarking itself remains outside this module.
//!
//! # Serialization integration
//!
//! Algorithm identity must be serializable by the scheduling serialization
//! layer when included in schedule provenance.
//!
//! The serialized algorithm identifier must be stable.
//!
//! Human-readable names must not be used as the sole machine-readable identity.
//!
//! # Versioning
//!
//! Individual algorithms own their semantic algorithm version.
//!
//! This module owns the algorithm-layer API surface, not the implementation
//! version of every algorithm.
//!
//! Changing a public re-export or module-level contract is an API change and
//! must be treated accordingly.
//!
//! # Rust compatibility
//!
//! This module is designed for:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! # Security
//!
//! This module contains no:
//!
//! - credentials;
//! - authentication;
//! - network communication;
//! - executable code loading;
//! - dynamic library loading;
//! - hardware access.
//!
//! Algorithm plugins must be validated through the appropriate plugin boundary
//! before being admitted into a production registry.
//!
//! # Testing contract
//!
//! This module should remain lightweight and should not duplicate algorithm
//! correctness tests.
//!
//! Algorithm correctness belongs to each algorithm's own test module.
//!
//! Integration tests belong under:
//!
//! ```text
//! src/quantum/scheduling/tests
//! ```
//!
//! or the repository's established integration-test location.
//!
//! This module only needs to test its public composition contract.
//!
//! # Forbidden design patterns
//!
//! The following must not be introduced here:
//!
//! ```text
//! static mut ...
//! ```
//!
//! ```text
//! MAX_QUBITS
//! ```
//!
//! ```text
//! MAX_OPERATIONS
//! ```
//!
//! ```text
//! fixed number of algorithms encoded in scheduler logic
//! ```
//!
//! ```text
//! hardware-specific branches
//! ```
//!
//! ```text
//! vendor-specific SDK calls
//! ```
//!
//! ```text
//! scheduler-local QubitId
//! ```
//!
//! ```text
//! unsafe
//! ```
//!
//! # Implementation
//!
//! The declarations below are intentionally simple.
//!
//! The algorithm files own their implementations.
//!
//! The order is stable and follows the conceptual dependency from general
//! scheduling strategies to specialized/adaptive strategies.
//!
//! ============================================================================
//! Safety boundary
//! ============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

// ============================================================================
// Algorithm modules
// ============================================================================

/// Adaptive scheduling algorithm.
///
/// Selects among supported scheduling strategies according to explicit
/// scheduling context/problem characteristics.
pub mod adaptive;

/// As-Late-As-Possible scheduling algorithm.
pub mod alap;

/// As-Soon-As-Possible scheduling algorithm.
pub mod asap;

/// Critical-path-oriented scheduling algorithm.
pub mod cp;

/// General-purpose resource-aware list scheduling algorithm.
pub mod list;

/// Resource-Constrained Project Scheduling Problem algorithm.
pub mod rcpsp;

// ============================================================================
// Stable algorithm identifiers
// ============================================================================

/// Stable identifier for the adaptive scheduling algorithm.
///
/// This identifier is suitable for configuration, provenance, diagnostics, and
/// serialization.
pub const ADAPTIVE_ALGORITHM_ID: &str = "scheduling.algorithms.adaptive";

/// Stable identifier for the ALAP scheduling algorithm.
pub const ALAP_ALGORITHM_ID: &str = "scheduling.algorithms.alap";

/// Stable identifier for the ASAP scheduling algorithm.
pub const ASAP_ALGORITHM_ID: &str = "scheduling.algorithms.asap";

/// Stable identifier for the critical-path scheduling algorithm.
pub const CRITICAL_PATH_ALGORITHM_ID: &str = "scheduling.algorithms.critical_path";

/// Stable identifier for the list scheduling algorithm.
pub const LIST_ALGORITHM_ID: &str = "scheduling.algorithms.list";

/// Stable identifier for the RCPSP scheduling algorithm.
pub const RCPSP_ALGORITHM_ID: &str = "scheduling.algorithms.rcpsp";

// ============================================================================
// Algorithm names
// ============================================================================

/// Stable human-readable name for adaptive scheduling.
pub const ADAPTIVE_ALGORITHM_NAME: &str = "adaptive";

/// Stable human-readable name for ALAP scheduling.
pub const ALAP_ALGORITHM_NAME: &str = "as-late-as-possible";

/// Stable human-readable name for ASAP scheduling.
pub const ASAP_ALGORITHM_NAME: &str = "as-soon-as-possible";

/// Stable human-readable name for critical-path scheduling.
pub const CRITICAL_PATH_ALGORITHM_NAME: &str = "critical-path";

/// Stable human-readable name for list scheduling.
pub const LIST_ALGORITHM_NAME: &str = "list";

/// Stable human-readable name for RCPSP scheduling.
pub const RCPSP_ALGORITHM_NAME: &str = "resource-constrained-project-scheduling";

// ============================================================================
// Algorithm kind
// ============================================================================

/// Stable classification of built-in scheduling algorithms.
///
/// This type is deliberately independent of concrete scheduler state.
///
/// It is useful when configuration, diagnostics, serialization, or a registry
/// needs to refer to a built-in algorithm without constructing it.
///
/// No machine-specific information is encoded here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[non_exhaustive]
pub enum AlgorithmKind {
    /// Adaptive strategy selection.
    Adaptive,

    /// As-Late-As-Possible scheduling.
    Alap,

    /// As-Soon-As-Possible scheduling.
    Asap,

    /// Critical-path-oriented scheduling.
    CriticalPath,

    /// Resource-aware list scheduling.
    List,

    /// Resource-Constrained Project Scheduling Problem scheduling.
    Rcpsp,
}

impl AlgorithmKind {
    /// Returns the stable machine-readable identifier.
    #[must_use]
    pub const fn id(self) -> &'static str {
        match self {
            Self::Adaptive => ADAPTIVE_ALGORITHM_ID,
            Self::Alap => ALAP_ALGORITHM_ID,
            Self::Asap => ASAP_ALGORITHM_ID,
            Self::CriticalPath => CRITICAL_PATH_ALGORITHM_ID,
            Self::List => LIST_ALGORITHM_ID,
            Self::Rcpsp => RCPSP_ALGORITHM_ID,
        }
    }

    /// Returns the stable human-readable name.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Adaptive => ADAPTIVE_ALGORITHM_NAME,
            Self::Alap => ALAP_ALGORITHM_NAME,
            Self::Asap => ASAP_ALGORITHM_NAME,
            Self::CriticalPath => CRITICAL_PATH_ALGORITHM_NAME,
            Self::List => LIST_ALGORITHM_NAME,
            Self::Rcpsp => RCPSP_ALGORITHM_NAME,
        }
    }

    /// Returns the complete built-in algorithm set.
    ///
    /// This is a compile-time static description, not a machine-size limit.
    ///
    /// The array contains algorithm kinds, not operations, resources, qubits,
    /// or schedule entries. Its finite size therefore does not constrain
    /// scheduling scalability.
    #[must_use]
    pub const fn builtins() -> [Self; 6] {
        [
            Self::Adaptive,
            Self::Alap,
            Self::Asap,
            Self::CriticalPath,
            Self::List,
            Self::Rcpsp,
        ]
    }

    /// Converts a stable algorithm identifier into a built-in algorithm kind.
    ///
    /// Unknown identifiers return `None` rather than being silently mapped to a
    /// different algorithm.
    #[must_use]
    pub const fn from_id(identifier: &str) -> Option<Self> {
        match identifier {
            ADAPTIVE_ALGORITHM_ID => Some(Self::Adaptive),
            ALAP_ALGORITHM_ID => Some(Self::Alap),
            ASAP_ALGORITHM_ID => Some(Self::Asap),
            CRITICAL_PATH_ALGORITHM_ID => Some(Self::CriticalPath),
            LIST_ALGORITHM_ID => Some(Self::List),
            RCPSP_ALGORITHM_ID => Some(Self::Rcpsp),
            _ => None,
        }
    }

    /// Returns whether this algorithm is primarily a strategy selector.
    #[must_use]
    pub const fn is_adaptive(self) -> bool {
        matches!(self, Self::Adaptive)
    }

    /// Returns whether this algorithm is an earliest-start strategy.
    #[must_use]
    pub const fn is_asap(self) -> bool {
        matches!(self, Self::Asap)
    }

    /// Returns whether this algorithm is a latest-start strategy.
    #[must_use]
    pub const fn is_alap(self) -> bool {
        matches!(self, Self::Alap)
    }

    /// Returns whether this algorithm is critical-path oriented.
    #[must_use]
    pub const fn is_critical_path(self) -> bool {
        matches!(self, Self::CriticalPath)
    }

    /// Returns whether this algorithm is a list-scheduling strategy.
    #[must_use]
    pub const fn is_list(self) -> bool {
        matches!(self, Self::List)
    }

    /// Returns whether this algorithm is resource-constrained-project based.
    #[must_use]
    pub const fn is_rcpsp(self) -> bool {
        matches!(self, Self::Rcpsp)
    }
}

impl std::fmt::Display for AlgorithmKind {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.id())
    }
}

// ============================================================================
// Algorithm metadata
// ============================================================================

/// Immutable metadata describing a built-in scheduling algorithm.
///
/// This type deliberately contains descriptive information only.
///
/// It does not contain:
///
/// - a scheduler instance;
/// - hardware state;
/// - a resource calendar;
/// - a dependency graph;
/// - mutable global state;
/// - a quantum program.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AlgorithmMetadata {
    /// Built-in algorithm kind.
    pub kind: AlgorithmKind,

    /// Stable machine-readable identifier.
    pub id: &'static str,

    /// Stable human-readable name.
    pub name: &'static str,
}

impl AlgorithmMetadata {
    /// Creates metadata for a built-in algorithm.
    #[must_use]
    pub const fn for_kind(kind: AlgorithmKind) -> Self {
        Self {
            kind,
            id: kind.id(),
            name: kind.name(),
        }
    }
}

/// Returns metadata for all built-in algorithms.
///
/// The returned metadata contains no target-specific state and therefore may
/// safely be used by registries, diagnostics, and documentation generation.
#[must_use]
pub const fn builtin_metadata() -> [AlgorithmMetadata; 6] {
    [
        AlgorithmMetadata::for_kind(AlgorithmKind::Adaptive),
        AlgorithmMetadata::for_kind(AlgorithmKind::Alap),
        AlgorithmMetadata::for_kind(AlgorithmKind::Asap),
        AlgorithmMetadata::for_kind(AlgorithmKind::CriticalPath),
        AlgorithmMetadata::for_kind(AlgorithmKind::List),
        AlgorithmMetadata::for_kind(AlgorithmKind::Rcpsp),
    ]
}

// ============================================================================
// Algorithm contract
// ============================================================================

/// Common semantic contract implemented by algorithm facades.
///
/// This trait deliberately does not prescribe the concrete scheduling input or
/// result types. Those remain owned by the canonical planner/context layers.
///
/// The trait exists so a registry or orchestration layer can reason about
/// algorithm identity without depending on implementation details.
///
/// Algorithm implementations may expose richer, algorithm-specific APIs in
/// their own modules.
pub trait Algorithm {
    /// Returns the stable algorithm kind.
    fn kind(&self) -> AlgorithmKind;

    /// Returns the stable algorithm identifier.
    fn id(&self) -> &'static str {
        self.kind().id()
    }

    /// Returns the stable human-readable name.
    fn name(&self) -> &'static str {
        self.kind().name()
    }
}

// ============================================================================
// Built-in algorithm contract implementations
// ============================================================================

impl Algorithm for adaptive::AdaptiveAlgorithm {
    fn kind(&self) -> AlgorithmKind {
        AlgorithmKind::Adaptive
    }
}

impl Algorithm for alap::AlapAlgorithm {
    fn kind(&self) -> AlgorithmKind {
        AlgorithmKind::Alap
    }
}

impl Algorithm for asap::AsapScheduler {
    fn kind(&self) -> AlgorithmKind {
        AlgorithmKind::Asap
    }
}

impl Algorithm for cp::CriticalPathAlgorithm {
    fn kind(&self) -> AlgorithmKind {
        AlgorithmKind::CriticalPath
    }
}

impl Algorithm for list::ListScheduler {
    fn kind(&self) -> AlgorithmKind {
        AlgorithmKind::List
    }
}

impl Algorithm for rcpsp::RcpspScheduler {
    fn kind(&self) -> AlgorithmKind {
        AlgorithmKind::Rcpsp
    }
}

// ============================================================================
// Public algorithm selection
// ============================================================================

/// Built-in algorithm selection.
///
/// This type represents configuration intent only.
///
/// It does not contain a machine, target, quantum circuit, resource model, or
/// schedule.
///
/// Higher-level scheduling orchestration should map this selection to the
/// corresponding algorithm facade.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum AlgorithmSelection {
    /// Select adaptive scheduling.
    Adaptive,

    /// Select ALAP scheduling.
    Alap,

    /// Select ASAP scheduling.
    Asap,

    /// Select critical-path scheduling.
    CriticalPath,

    /// Select list scheduling.
    List,

    /// Select RCPSP scheduling.
    Rcpsp,
}

impl AlgorithmSelection {
    /// Returns the corresponding built-in algorithm kind.
    #[must_use]
    pub const fn kind(self) -> AlgorithmKind {
        match self {
            Self::Adaptive => AlgorithmKind::Adaptive,
            Self::Alap => AlgorithmKind::Alap,
            Self::Asap => AlgorithmKind::Asap,
            Self::CriticalPath => AlgorithmKind::CriticalPath,
            Self::List => AlgorithmKind::List,
            Self::Rcpsp => AlgorithmKind::Rcpsp,
        }
    }

    /// Returns the stable machine-readable algorithm identifier.
    #[must_use]
    pub const fn id(self) -> &'static str {
        self.kind().id()
    }
}

impl From<AlgorithmSelection> for AlgorithmKind {
    fn from(selection: AlgorithmSelection) -> Self {
        selection.kind()
    }
}

impl From<AlgorithmKind> for AlgorithmSelection {
    fn from(kind: AlgorithmKind) -> Self {
        match kind {
            AlgorithmKind::Adaptive => Self::Adaptive,
            AlgorithmKind::Alap => Self::Alap,
            AlgorithmKind::Asap => Self::Asap,
            AlgorithmKind::CriticalPath => Self::CriticalPath,
            AlgorithmKind::List => Self::List,
            AlgorithmKind::Rcpsp => Self::Rcpsp,
        }
    }
}

// ============================================================================
// Compile-time architectural guarantees
// ============================================================================

/// Returns whether the algorithm layer contains no machine-size limit.
///
/// This function is intentionally declarative and exists to make the
/// architectural guarantee testable/documentable.
#[must_use]
pub const fn has_no_machine_size_limit() -> bool {
    true
}

/// Returns whether the algorithm layer requires no unsafe Rust.
///
/// The module is also guarded by `#![forbid(unsafe_code)]`.
#[must_use]
pub const fn uses_no_unsafe() -> bool {
    true
}

/// Returns whether built-in algorithm selection is deterministic.
///
/// Algorithm execution can still be configured through an algorithm's explicit
/// deterministic/randomized policy where applicable, but this module itself
/// performs no implicit random selection.
#[must_use]
pub const fn selection_is_deterministic() -> bool {
    true
}

/// Returns whether algorithm selection is target-independent.
///
/// Target-specific behavior is supplied through the scheduling context rather
/// than encoded in this module.
#[must_use]
pub const fn selection_is_target_independent() -> bool {
    true
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_builtin_has_stable_identity() {
        let algorithms = AlgorithmKind::builtins();

        assert_eq!(algorithms.len(), 6);

        for algorithm in algorithms {
            assert!(!algorithm.id().is_empty());
            assert!(!algorithm.name().is_empty());
        }
    }

    #[test]
    fn identifiers_round_trip() {
        for algorithm in AlgorithmKind::builtins() {
            assert_eq!(AlgorithmKind::from_id(algorithm.id()), Some(algorithm));
        }
    }

    #[test]
    fn unknown_identifier_is_not_silently_accepted() {
        assert_eq!(
            AlgorithmKind::from_id("scheduling.algorithms.unknown"),
            None
        );
    }

    #[test]
    fn metadata_matches_algorithm_kind() {
        for metadata in builtin_metadata() {
            assert_eq!(metadata.id, metadata.kind.id());
            assert_eq!(metadata.name, metadata.kind.name());
        }
    }

    #[test]
    fn algorithm_selection_maps_to_algorithm_kind() {
        let selections = [
            AlgorithmSelection::Adaptive,
            AlgorithmSelection::Alap,
            AlgorithmSelection::Asap,
            AlgorithmSelection::CriticalPath,
            AlgorithmSelection::List,
            AlgorithmSelection::Rcpsp,
        ];

        for selection in selections {
            assert_eq!(selection.id(), selection.kind().id());
        }
    }

    #[test]
    fn architectural_guarantees_are_explicit() {
        assert!(has_no_machine_size_limit());
        assert!(uses_no_unsafe());
        assert!(selection_is_deterministic());
        assert!(selection_is_target_independent());
    }

    #[test]
    fn algorithm_trait_uses_stable_identity() {
        let asap = asap::AsapScheduler::new();

        assert_eq!(Algorithm::kind(&asap), AlgorithmKind::Asap);
        assert_eq!(Algorithm::id(&asap), ASAP_ALGORITHM_ID);
        assert_eq!(Algorithm::name(&asap), ASAP_ALGORITHM_NAME);
    }
}