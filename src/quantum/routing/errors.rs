//! Zamani Quantum Routing — Canonical Error Model
//!
//! This module defines the stable error vocabulary for the quantum routing
//! subsystem.
//!
//! # Architectural responsibility
//!
//! `errors.rs` owns failures that occur while transforming a logical quantum
//! program into a topology-compatible physical program.
//!
//! It covers:
//!
//! - topology construction and validation;
//! - physical connectivity;
//! - logical/physical qubit identity;
//! - logical-to-physical mapping;
//! - layout;
//! - routing;
//! - routing candidates;
//! - movement operations such as SWAP and bridge operations;
//! - gate-direction constraints;
//! - routing objectives and cost models;
//! - algorithm configuration;
//! - resource limits;
//! - timeouts and iteration limits;
//! - deterministic execution;
//! - verification;
//! - transactional routing;
//! - unsupported operations;
//! - backend/hardware capability mismatches;
//! - invariant violations.
//!
//! # Architectural boundary
//!
//! This file intentionally does NOT depend on:
//!
//! - `types.rs`;
//! - `topology.rs`;
//! - `mapping.rs`;
//! - `cost.rs`;
//! - `config.rs`;
//! - `result.rs`;
//! - `layout.rs`;
//! - `router.rs`;
//! - `verification.rs`;
//! - `transpiler.rs`;
//! - the Quantum IR implementation;
//! - hardware-provider implementations;
//! - OpenQASM/frontend implementations;
//! - QEC implementations;
//! - scheduling implementations.
//!
//! This dependency direction is intentional.
//!
//! All of those modules may depend on this file, while this file remains
//! independent from them.
//!
//! # Stability rule
//!
//! This file is a foundational routing contract. Later routing files should
//! consume these error types rather than defining their own routing-specific
//! error enums.
//!
//! In particular, the old `TranspilerError` currently defined in
//! `transpiler.rs` should eventually be translated into this error model.
//! `transpiler.rs` must not become the owner of the routing error vocabulary.
//!
//! # Design goals
//!
//! The error model must:
//!
//! 1. preserve machine-readable failure classification;
//! 2. preserve structured diagnostic context;
//! 3. avoid requiring consumers to parse strings;
//! 4. provide deterministic `Display` output;
//! 5. remain usable by library, compiler, CLI, tests, and diagnostics;
//! 6. support future routing algorithms without adding ad-hoc errors;
//! 7. distinguish invalid input from routing impossibility;
//! 8. distinguish routing impossibility from resource exhaustion;
//! 9. distinguish user/configuration errors from internal invariant failures;
//! 10. remain independent from all later routing implementation files.
//!
//! # Rust compatibility
//!
//! Target: Rust 1.97.1.
//!
//! No nightly features are required.
//! No external dependencies are required.

// =============================================================================
// Result alias
// =============================================================================

/// Canonical result type for the quantum routing subsystem.
pub type RoutingResult<T> = Result<T, RoutingError>;

// =============================================================================
// Error severity
// =============================================================================

/// Severity classification for a routing failure.
///
/// This is metadata for diagnostics and tooling. It does not determine whether
/// an error is recoverable; callers should inspect the concrete error kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RoutingErrorSeverity {
    /// Invalid input/configuration that should normally be fixed by the caller.
    Error,

    /// A requested routing strategy cannot satisfy the current constraints,
    /// but another strategy may be able to do so.
    Recoverable,

    /// A routing invariant has been violated and execution must not continue.
    Internal,
}

impl RoutingErrorSeverity {
    /// Returns whether this error represents an internal invariant failure.
    pub const fn is_internal(self) -> bool {
        matches!(self, Self::Internal)
    }
}

impl std::fmt::Display for RoutingErrorSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Error => write!(f, "error"),
            Self::Recoverable => write!(f, "recoverable"),
            Self::Internal => write!(f, "internal"),
        }
    }
}

// =============================================================================
// Routing stage
// =============================================================================

/// Compiler/routing stage in which an error occurred.
///
/// Keeping this independent from the concrete routing implementation allows
/// diagnostics to identify where a failure occurred without depending on
/// `router.rs`, `layout.rs`, or `verification.rs`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RoutingStage {
    /// Input normalization/validation.
    InputValidation,

    /// Hardware topology construction or validation.
    Topology,

    /// Initial logical-to-physical placement.
    Layout,

    /// Logical-to-physical mapping manipulation.
    Mapping,

    /// Graph/path calculation.
    PathFinding,

    /// Candidate movement generation.
    CandidateGeneration,

    /// Routing algorithm execution.
    Routing,

    /// Movement generation/lowering boundary.
    Movement,

    /// Routing verification.
    Verification,

    /// Transaction commit/rollback.
    Transaction,

    /// Final compiler/transpiler integration.
    Transpilation,

    /// Configuration validation.
    Configuration,
}

impl std::fmt::Display for RoutingStage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::InputValidation => "input_validation",
            Self::Topology => "topology",
            Self::Layout => "layout",
            Self::Mapping => "mapping",
            Self::PathFinding => "path_finding",
            Self::CandidateGeneration => "candidate_generation",
            Self::Routing => "routing",
            Self::Movement => "movement",
            Self::Verification => "verification",
            Self::Transaction => "transaction",
            Self::Transpilation => "transpilation",
            Self::Configuration => "configuration",
        };

        write!(f, "{value}")
    }
}

// =============================================================================
// Error context
// =============================================================================

/// Optional structured context attached to a routing error.
///
/// This type intentionally contains primitive/owned values rather than routing
/// subsystem structs. That keeps `errors.rs` independent from `types.rs`,
/// `topology.rs`, and `mapping.rs`.
///
/// Context is diagnostic information. It must never be required to understand
/// the semantic error category.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RoutingErrorContext {
    /// Routing stage in which the error occurred.
    pub stage: Option<RoutingStage>,

    /// Logical qubit involved in the failure, when known.
    pub logical_qubit: Option<String>,

    /// Second logical qubit involved in the failure, when known.
    pub second_logical_qubit: Option<String>,

    /// Physical qubit involved in the failure, when known.
    pub physical_qubit: Option<usize>,

    /// Second physical qubit involved in the failure, when known.
    pub second_physical_qubit: Option<usize>,

    /// Gate/operation name, when known.
    pub gate: Option<String>,

    /// Operation index in the circuit, when known.
    pub operation_index: Option<usize>,

    /// Algorithm name, when known.
    pub algorithm: Option<String>,

    /// Topology/device name, when known.
    pub topology: Option<String>,

    /// Number of logical qubits involved.
    pub logical_qubit_count: Option<usize>,

    /// Number of available physical qubits.
    pub physical_qubit_count: Option<usize>,

    /// Configuration field involved in the failure.
    pub configuration_field: Option<String>,

    /// Human-readable diagnostic detail.
    pub detail: Option<String>,
}

impl RoutingErrorContext {
    /// Creates empty routing context.
    pub const fn new() -> Self {
        Self {
            stage: None,
            logical_qubit: None,
            second_logical_qubit: None,
            physical_qubit: None,
            second_physical_qubit: None,
            gate: None,
            operation_index: None,
            algorithm: None,
            topology: None,
            logical_qubit_count: None,
            physical_qubit_count: None,
            configuration_field: None,
            detail: None,
        }
    }

    pub fn with_stage(mut self, stage: RoutingStage) -> Self {
        self.stage = Some(stage);
        self
    }

    pub fn with_logical_qubit(mut self, qubit: impl Into<String>) -> Self {
        self.logical_qubit = Some(qubit.into());
        self
    }

    pub fn with_second_logical_qubit(
        mut self,
        qubit: impl Into<String>,
    ) -> Self {
        self.second_logical_qubit = Some(qubit.into());
        self
    }

    pub fn with_physical_qubit(mut self, qubit: usize) -> Self {
        self.physical_qubit = Some(qubit);
        self
    }

    pub fn with_second_physical_qubit(mut self, qubit: usize) -> Self {
        self.second_physical_qubit = Some(qubit);
        self
    }

    pub fn with_gate(mut self, gate: impl Into<String>) -> Self {
        self.gate = Some(gate.into());
        self
    }

    pub fn with_operation_index(mut self, index: usize) -> Self {
        self.operation_index = Some(index);
        self
    }

    pub fn with_algorithm(mut self, algorithm: impl Into<String>) -> Self {
        self.algorithm = Some(algorithm.into());
        self
    }

    pub fn with_topology(mut self, topology: impl Into<String>) -> Self {
        self.topology = Some(topology.into());
        self
    }

    pub fn with_logical_qubit_count(mut self, count: usize) -> Self {
        self.logical_qubit_count = Some(count);
        self
    }

    pub fn with_physical_qubit_count(mut self, count: usize) -> Self {
        self.physical_qubit_count = Some(count);
        self
    }

    pub fn with_configuration_field(
        mut self,
        field: impl Into<String>,
    ) -> Self {
        self.configuration_field = Some(field.into());
        self
    }

    pub fn with_detail(mut self, detail: impl Into<String>) -> Self {
        self.detail = Some(detail.into());
        self
    }
}

// =============================================================================
// Topology errors
// =============================================================================

/// Failure involving the physical hardware topology.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TopologyError {
    /// No physical qubits were provided.
    Empty,

    /// A physical qubit references a vertex that does not exist.
    MissingVertex {
        qubit: usize,
    },

    /// A self-loop was declared.
    SelfLoop {
        qubit: usize,
    },

    /// The same edge was declared more than once.
    DuplicateEdge {
        a: usize,
        b: usize,
    },

    /// A directed edge was declared in a topology that requires symmetry.
    AsymmetricEdge {
        from: usize,
        to: usize,
    },

    /// A physical qubit identifier is invalid.
    InvalidQubit {
        qubit: usize,
    },

    /// The topology contains an invalid edge endpoint.
    InvalidEdge {
        from: usize,
        to: usize,
    },

    /// The topology contains no path between required vertices.
    Disconnected {
        from: usize,
        to: usize,
    },

    /// The topology contains duplicate/invalid adjacency entries.
    InvalidAdjacency {
        qubit: usize,
    },

    /// The topology has inconsistent metadata.
    InvalidMetadata {
        field: String,
    },

    /// A gate is not supported on an otherwise connected pair.
    GateNotSupported {
        gate: String,
        from: usize,
        to: usize,
    },

    /// A directed gate is supported only in the opposite direction.
    GateDirectionUnsupported {
        gate: String,
        from: usize,
        to: usize,
    },

    /// A physical qubit is unavailable.
    QubitUnavailable {
        qubit: usize,
    },

    /// An edge is unavailable.
    EdgeUnavailable {
        from: usize,
        to: usize,
    },

    /// Hardware calibration data required by routing is missing.
    MissingCalibration {
        resource: String,
    },

    /// Hardware calibration data is malformed.
    InvalidCalibration {
        resource: String,
        reason: String,
    },
}

impl std::fmt::Display for TopologyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "topology is empty"),

            Self::MissingVertex { qubit } => {
                write!(f, "topology references missing physical qubit {qubit}")
            }

            Self::SelfLoop { qubit } => {
                write!(f, "physical qubit {qubit} has an invalid self-loop")
            }

            Self::DuplicateEdge { a, b } => {
                write!(f, "topology contains duplicate edge {a} <-> {b}")
            }

            Self::AsymmetricEdge { from, to } => {
                write!(f, "topology edge {from} -> {to} is not symmetric")
            }

            Self::InvalidQubit { qubit } => {
                write!(f, "invalid physical qubit {qubit}")
            }

            Self::InvalidEdge { from, to } => {
                write!(f, "invalid topology edge {from} -> {to}")
            }

            Self::Disconnected { from, to } => write!(
                f,
                "physical qubits {from} and {to} are disconnected"
            ),

            Self::InvalidAdjacency { qubit } => {
                write!(f, "invalid adjacency list for physical qubit {qubit}")
            }

            Self::InvalidMetadata { field } => {
                write!(f, "invalid topology metadata field `{field}`")
            }

            Self::GateNotSupported { gate, from, to } => write!(
                f,
                "gate `{gate}` is not supported between physical qubits {from} and {to}"
            ),

            Self::GateDirectionUnsupported { gate, from, to } => write!(
                f,
                "gate `{gate}` is not supported in direction {from} -> {to}"
            ),

            Self::QubitUnavailable { qubit } => {
                write!(f, "physical qubit {qubit} is unavailable")
            }

            Self::EdgeUnavailable { from, to } => {
                write!(f, "physical edge {from} -> {to} is unavailable")
            }

            Self::MissingCalibration { resource } => {
                write!(f, "missing calibration for `{resource}`")
            }

            Self::InvalidCalibration { resource, reason } => write!(
                f,
                "invalid calibration for `{resource}`: {reason}"
            ),
        }
    }
}

// =============================================================================
// Mapping errors
// =============================================================================

/// Failure involving logical-to-physical qubit mapping.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MappingError {
    /// A logical qubit is already mapped.
    LogicalAlreadyMapped {
        logical: String,
    },

    /// A physical qubit is already occupied.
    PhysicalAlreadyAssigned {
        physical: usize,
    },

    /// A logical qubit does not exist in the current mapping.
    UnknownLogicalQubit {
        logical: String,
    },

    /// A physical qubit does not exist in the current mapping/topology.
    UnknownPhysicalQubit {
        physical: usize,
    },

    /// A mapping operation would create a collision.
    Collision {
        logical: String,
        physical: usize,
    },

    /// Mapping state is internally inconsistent.
    Inconsistent {
        detail: String,
    },

    /// A mapping snapshot cannot be restored.
    RestoreFailed {
        detail: String,
    },

    /// A permutation contains invalid or duplicate physical positions.
    InvalidPermutation {
        detail: String,
    },
}

impl std::fmt::Display for MappingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LogicalAlreadyMapped { logical } => {
                write!(f, "logical qubit `{logical}` is already mapped")
            }

            Self::PhysicalAlreadyAssigned { physical } => {
                write!(f, "physical qubit {physical} is already assigned")
            }

            Self::UnknownLogicalQubit { logical } => {
                write!(f, "unknown logical qubit `{logical}`")
            }

            Self::UnknownPhysicalQubit { physical } => {
                write!(f, "unknown physical qubit {physical}")
            }

            Self::Collision { logical, physical } => write!(
                f,
                "mapping collision: logical qubit `{logical}` cannot occupy physical qubit {physical}"
            ),

            Self::Inconsistent { detail } => {
                write!(f, "inconsistent qubit mapping: {detail}")
            }

            Self::RestoreFailed { detail } => {
                write!(f, "failed to restore qubit mapping: {detail}")
            }

            Self::InvalidPermutation { detail } => {
                write!(f, "invalid qubit permutation: {detail}")
            }
        }
    }
}

// =============================================================================
// Layout errors
// =============================================================================

/// Failure while selecting an initial logical-to-physical layout.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LayoutError {
    /// The circuit requires more physical qubits than available.
    InsufficientPhysicalQubits {
        required: usize,
        available: usize,
    },

    /// No physical placement satisfies the requested constraints.
    NoValidLayout,

    /// A layout strategy was requested but is not available.
    UnsupportedStrategy {
        strategy: String,
    },

    /// The interaction graph is invalid.
    InvalidInteractionGraph {
        detail: String,
    },

    /// A layout strategy exceeded its configured resource limit.
    ResourceLimitExceeded {
        resource: String,
        limit: usize,
    },

    /// Layout search could not produce a valid candidate.
    SearchFailed {
        detail: String,
    },

    /// A noise-aware layout lacks the data required to evaluate candidates.
    MissingHardwareData {
        resource: String,
    },
}

impl std::fmt::Display for LayoutError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InsufficientPhysicalQubits {
                required,
                available,
            } => write!(
                f,
                "layout requires {required} physical qubits but only {available} are available"
            ),

            Self::NoValidLayout => {
                write!(f, "no valid initial qubit layout could be constructed")
            }

            Self::UnsupportedStrategy { strategy } => {
                write!(f, "unsupported layout strategy `{strategy}`")
            }

            Self::InvalidInteractionGraph { detail } => {
                write!(f, "invalid interaction graph: {detail}")
            }

            Self::ResourceLimitExceeded { resource, limit } => write!(
                f,
                "layout resource limit `{resource}` exceeded: limit={limit}"
            ),

            Self::SearchFailed { detail } => {
                write!(f, "layout search failed: {detail}")
            }

            Self::MissingHardwareData { resource } => {
                write!(f, "layout requires unavailable hardware data `{resource}`")
            }
        }
    }
}

// =============================================================================
// Path-finding errors
// =============================================================================

/// Failure while calculating a physical path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PathError {
    /// Start physical qubit does not exist.
    InvalidStart {
        qubit: usize,
    },

    /// Target physical qubit does not exist.
    InvalidTarget {
        qubit: usize,
    },

    /// No path exists.
    NoPath {
        from: usize,
        to: usize,
    },

    /// A weighted path requires an unavailable weight.
    MissingWeight {
        from: usize,
        to: usize,
    },

    /// A path contains an illegal vertex.
    InvalidPath {
        detail: String,
    },

    /// Path search exceeded a configured bound.
    SearchLimitExceeded {
        limit: usize,
    },
}

impl std::fmt::Display for PathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidStart { qubit } => {
                write!(f, "path start physical qubit {qubit} does not exist")
            }

            Self::InvalidTarget { qubit } => {
                write!(f, "path target physical qubit {qubit} does not exist")
            }

            Self::NoPath { from, to } => {
                write!(f, "no physical path exists from {from} to {to}")
            }

            Self::MissingWeight { from, to } => {
                write!(f, "missing path weight for edge {from} -> {to}")
            }

            Self::InvalidPath { detail } => {
                write!(f, "invalid physical path: {detail}")
            }

            Self::SearchLimitExceeded { limit } => {
                write!(f, "path search limit exceeded: {limit}")
            }
        }
    }
}

// =============================================================================
// Candidate errors
// =============================================================================

/// Failure while generating or evaluating routing candidates.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CandidateError {
    /// No legal candidate movement exists.
    NoCandidate,

    /// Candidate list is empty when a candidate is mandatory.
    EmptyCandidateSet,

    /// Candidate contains an invalid physical edge.
    InvalidEdge {
        a: usize,
        b: usize,
    },

    /// Candidate conflicts with current mapping.
    MappingConflict {
        a: usize,
        b: usize,
    },

    /// Candidate violates gate directionality.
    DirectionConflict {
        gate: String,
        a: usize,
        b: usize,
    },

    /// Candidate score cannot be evaluated.
    ScoreEvaluationFailed {
        detail: String,
    },

    /// Candidate generation exceeded its configured limit.
    CandidateLimitExceeded {
        limit: usize,
    },
}

impl std::fmt::Display for CandidateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoCandidate => {
                write!(f, "no legal routing candidate exists")
            }

            Self::EmptyCandidateSet => {
                write!(f, "routing candidate set is empty")
            }

            Self::InvalidEdge { a, b } => {
                write!(f, "candidate uses invalid physical edge {a} <-> {b}")
            }

            Self::MappingConflict { a, b } => write!(
                f,
                "candidate movement conflicts with mapping at physical qubits {a} and {b}"
            ),

            Self::DirectionConflict { gate, a, b } => write!(
                f,
                "candidate cannot satisfy direction of gate `{gate}` on {a} -> {b}"
            ),

            Self::ScoreEvaluationFailed { detail } => {
                write!(f, "candidate score evaluation failed: {detail}")
            }

            Self::CandidateLimitExceeded { limit } => {
                write!(f, "candidate limit exceeded: {limit}")
            }
        }
    }
}

// =============================================================================
// Routing algorithm errors
// =============================================================================

/// Failure specific to execution of a routing algorithm.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AlgorithmError {
    /// The requested algorithm is not registered/implemented.
    Unsupported {
        algorithm: String,
    },

    /// Algorithm cannot operate on the requested circuit/topology.
    Incompatible {
        algorithm: String,
        reason: String,
    },

    /// No legal routing decision can be made.
    NoLegalMove {
        algorithm: String,
    },

    /// The algorithm exceeded its iteration budget.
    IterationLimitExceeded {
        limit: usize,
    },

    /// The algorithm exceeded its SWAP budget.
    SwapLimitExceeded {
        limit: usize,
    },

    /// The algorithm exceeded its time budget.
    Timeout,

    /// The algorithm's internal search failed.
    SearchFailed {
        algorithm: String,
        detail: String,
    },

    /// A deterministic seed/configuration is invalid.
    InvalidSeed {
        seed: u64,
    },

    /// An algorithm-specific invariant was violated.
    InvariantViolation {
        algorithm: String,
        detail: String,
    },

    /// An algorithm failed to improve or produce a valid result.
    NoValidResult {
        algorithm: String,
    },
}

impl std::fmt::Display for AlgorithmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unsupported { algorithm } => {
                write!(f, "unsupported routing algorithm `{algorithm}`")
            }

            Self::Incompatible { algorithm, reason } => {
                write!(f, "routing algorithm `{algorithm}` is incompatible: {reason}")
            }

            Self::NoLegalMove { algorithm } => {
                write!(f, "routing algorithm `{algorithm}` found no legal move")
            }

            Self::IterationLimitExceeded { limit } => {
                write!(f, "routing iteration limit exceeded: {limit}")
            }

            Self::SwapLimitExceeded { limit } => {
                write!(f, "routing SWAP limit exceeded: {limit}")
            }

            Self::Timeout => {
                write!(f, "routing algorithm timed out")
            }

            Self::SearchFailed { algorithm, detail } => {
                write!(f, "routing algorithm `{algorithm}` failed: {detail}")
            }

            Self::InvalidSeed { seed } => {
                write!(f, "invalid routing seed: {seed}")
            }

            Self::InvariantViolation { algorithm, detail } => write!(
                f,
                "routing algorithm `{algorithm}` violated an invariant: {detail}"
            ),

            Self::NoValidResult { algorithm } => {
                write!(f, "routing algorithm `{algorithm}` produced no valid result")
            }
        }
    }
}

// =============================================================================
// Movement errors
// =============================================================================

/// Failure involving routing movement operations.
///
/// A routing movement is intentionally distinct from a final hardware gate.
/// For example, a `SwapMove` represents a state permutation; hardware lowering
/// may later decompose it into native gates.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MovementError {
    /// Physical endpoints are identical.
    SameQubit {
        qubit: usize,
    },

    /// Physical endpoints are not connected.
    NonAdjacent {
        a: usize,
        b: usize,
    },

    /// Requested movement is not supported by the target.
    Unsupported {
        movement: String,
    },

    /// A movement cannot be represented by the current hardware target.
    UnsupportedByTarget {
        movement: String,
    },

    /// Movement would create an invalid mapping.
    MappingViolation {
        detail: String,
    },

    /// Movement decomposition is unavailable.
    MissingDecomposition {
        movement: String,
    },

    /// Movement is invalid for the selected gate/circuit state.
    Invalid {
        movement: String,
        reason: String,
    },
}

impl std::fmt::Display for MovementError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SameQubit { qubit } => {
                write!(f, "movement cannot use the same physical qubit {qubit}")
            }

            Self::NonAdjacent { a, b } => {
                write!(f, "movement endpoints {a} and {b} are not adjacent")
            }

            Self::Unsupported { movement } => {
                write!(f, "unsupported routing movement `{movement}`")
            }

            Self::UnsupportedByTarget { movement } => write!(
                f,
                "routing target does not support movement `{movement}`"
            ),

            Self::MappingViolation { detail } => {
                write!(f, "movement violates mapping constraints: {detail}")
            }

            Self::MissingDecomposition { movement } => {
                write!(f, "no decomposition is available for movement `{movement}`")
            }

            Self::Invalid { movement, reason } => {
                write!(f, "invalid movement `{movement}`: {reason}")
            }
        }
    }
}

// =============================================================================
// Operation/gate errors
// =============================================================================

/// Failure involving a quantum operation that routing is expected to handle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OperationError {
    /// Operation contains no usable qubit operands.
    MissingOperands,

    /// Operation has an unsupported arity.
    UnsupportedArity {
        gate: String,
        arity: usize,
    },

    /// Operation contains an invalid logical operand.
    InvalidOperand {
        operand: String,
    },

    /// Operation contains a duplicate logical operand.
    DuplicateOperand {
        operand: String,
    },

    /// Gate name is empty or malformed.
    InvalidGateName,

    /// Gate is unknown at the routing boundary.
    UnsupportedGate {
        gate: String,
    },

    /// Gate has a direction requirement that cannot be satisfied.
    UnsupportedDirection {
        gate: String,
    },

    /// Operation cannot be routed without prior decomposition.
    RequiresDecomposition {
        gate: String,
        arity: usize,
    },

    /// Measurement/classical semantics cannot be preserved.
    MeasurementSemantics {
        detail: String,
    },

    /// Routing would alter an operation that must remain semantically stable.
    SemanticViolation {
        detail: String,
    },
}

impl std::fmt::Display for OperationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingOperands => {
                write!(f, "quantum operation has no operands")
            }

            Self::UnsupportedArity { gate, arity } => {
                write!(f, "gate `{gate}` has unsupported arity {arity}")
            }

            Self::InvalidOperand { operand } => {
                write!(f, "invalid quantum operand `{operand}`")
            }

            Self::DuplicateOperand { operand } => {
                write!(f, "quantum operand `{operand}` appears more than once")
            }

            Self::InvalidGateName => {
                write!(f, "quantum gate name is empty or invalid")
            }

            Self::UnsupportedGate { gate } => {
                write!(f, "unsupported quantum gate `{gate}`")
            }

            Self::UnsupportedDirection { gate } => {
                write!(f, "unsupported direction for gate `{gate}`")
            }

            Self::RequiresDecomposition { gate, arity } => write!(
                f,
                "gate `{gate}` with arity {arity} requires decomposition before routing"
            ),

            Self::MeasurementSemantics { detail } => {
                write!(f, "measurement semantics cannot be preserved: {detail}")
            }

            Self::SemanticViolation { detail } => {
                write!(f, "routing would violate operation semantics: {detail}")
            }
        }
    }
}

// =============================================================================
// Configuration errors
// =============================================================================

/// Invalid routing configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfigurationError {
    /// Requested algorithm name is invalid.
    InvalidAlgorithm {
        algorithm: String,
    },

    /// Requested objective is invalid.
    InvalidObjective {
        objective: String,
    },

    /// Numeric configuration value is outside its legal range.
    InvalidValue {
        field: String,
        reason: String,
    },

    /// A combination of options is incompatible.
    IncompatibleOptions {
        first: String,
        second: String,
    },

    /// A custom algorithm/model name is invalid.
    InvalidName {
        field: String,
        value: String,
    },

    /// A deterministic configuration is missing required information.
    NonDeterministicConfiguration {
        reason: String,
    },

    /// A configured resource bound is invalid.
    InvalidLimit {
        field: String,
        value: usize,
    },

    /// Configuration requires unavailable functionality.
    UnsupportedFeature {
        feature: String,
    },
}

impl std::fmt::Display for ConfigurationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidAlgorithm { algorithm } => {
                write!(f, "invalid routing algorithm `{algorithm}`")
            }

            Self::InvalidObjective { objective } => {
                write!(f, "invalid routing objective `{objective}`")
            }

            Self::InvalidValue { field, reason } => {
                write!(f, "invalid routing configuration `{field}`: {reason}")
            }

            Self::IncompatibleOptions { first, second } => {
                write!(f, "incompatible routing options `{first}` and `{second}`")
            }

            Self::InvalidName { field, value } => {
                write!(f, "invalid `{field}` name `{value}`")
            }

            Self::NonDeterministicConfiguration { reason } => {
                write!(f, "invalid deterministic configuration: {reason}")
            }

            Self::InvalidLimit { field, value } => {
                write!(f, "invalid routing limit `{field}`={value}")
            }

            Self::UnsupportedFeature { feature } => {
                write!(f, "unsupported routing configuration feature `{feature}`")
            }
        }
    }
}

// =============================================================================
// Cost-model errors
// =============================================================================

/// Failure while evaluating routing cost/objectives.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CostModelError {
    /// Cost model is not supported.
    UnsupportedModel {
        model: String,
    },

    /// Required hardware metric is missing.
    MissingMetric {
        metric: String,
    },

    /// A metric has an invalid value.
    InvalidMetric {
        metric: String,
    },

    /// A weighted objective contains invalid weights.
    InvalidWeights {
        detail: String,
    },

    /// A cost candidate cannot be evaluated.
    EvaluationFailed {
        detail: String,
    },

    /// Cost comparison cannot be performed.
    ComparisonFailed {
        detail: String,
    },
}

impl std::fmt::Display for CostModelError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnsupportedModel { model } => {
                write!(f, "unsupported routing cost model `{model}`")
            }

            Self::MissingMetric { metric } => {
                write!(f, "missing routing cost metric `{metric}`")
            }

            Self::InvalidMetric { metric } => {
                write!(f, "invalid routing cost metric `{metric}`")
            }

            Self::InvalidWeights { detail } => {
                write!(f, "invalid routing cost weights: {detail}")
            }

            Self::EvaluationFailed { detail } => {
                write!(f, "routing cost evaluation failed: {detail}")
            }

            Self::ComparisonFailed { detail } => {
                write!(f, "routing cost comparison failed: {detail}")
            }
        }
    }
}

// =============================================================================
// Resource errors
// =============================================================================

/// Resource exhaustion while routing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResourceError {
    /// Physical qubits are insufficient.
    InsufficientPhysicalQubits {
        required: usize,
        available: usize,
    },

    /// Routing iteration budget was exhausted.
    IterationLimitExceeded {
        limit: usize,
    },

    /// Candidate budget was exhausted.
    CandidateLimitExceeded {
        limit: usize,
    },

    /// SWAP budget was exhausted.
    SwapLimitExceeded {
        limit: usize,
    },

    /// Circuit operation budget was exceeded.
    OperationLimitExceeded {
        limit: usize,
    },

    /// Memory/resource policy rejected the requested operation.
    MemoryLimitExceeded {
        detail: String,
    },

    /// Time budget was exhausted.
    Timeout,

    /// A configured deadline was already expired.
    DeadlineExceeded,
}

impl std::fmt::Display for ResourceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InsufficientPhysicalQubits {
                required,
                available,
            } => write!(
                f,
                "requires {required} physical qubits but only {available} are available"
            ),

            Self::IterationLimitExceeded { limit } => {
                write!(f, "routing iteration limit exceeded: {limit}")
            }

            Self::CandidateLimitExceeded { limit } => {
                write!(f, "routing candidate limit exceeded: {limit}")
            }

            Self::SwapLimitExceeded { limit } => {
                write!(f, "routing SWAP limit exceeded: {limit}")
            }

            Self::OperationLimitExceeded { limit } => {
                write!(f, "routing operation limit exceeded: {limit}")
            }

            Self::MemoryLimitExceeded { detail } => {
                write!(f, "routing memory limit exceeded: {detail}")
            }

            Self::Timeout => {
                write!(f, "routing operation timed out")
            }

            Self::DeadlineExceeded => {
                write!(f, "routing deadline exceeded")
            }
        }
    }
}

// =============================================================================
// Verification errors
// =============================================================================

/// Failure detected by the routing verifier.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VerificationError {
    /// Input mapping is invalid.
    InvalidInitialMapping {
        detail: String,
    },

    /// Final mapping is invalid.
    InvalidFinalMapping {
        detail: String,
    },

    /// Mapping contains a physical collision.
    MappingCollision {
        physical: usize,
    },

    /// A routed operation is not legal on the topology.
    IllegalOperation {
        gate: String,
        physical_operands: Vec<usize>,
    },

    /// A routed operation violates gate directionality.
    IllegalDirection {
        gate: String,
        from: usize,
        to: usize,
    },

    /// A generated movement is illegal.
    IllegalMovement {
        movement: String,
        a: usize,
        b: usize,
    },

    /// Logical gate sequence was not preserved.
    GateSequenceMismatch {
        operation_index: usize,
    },

    /// Logical qubit semantics changed.
    QubitSemanticsChanged {
        logical: String,
    },

    /// Measurement mapping was changed incorrectly.
    MeasurementMismatch {
        operation_index: usize,
    },

    /// Classical output mapping was changed incorrectly.
    ClassicalMappingMismatch {
        operation_index: usize,
    },

    /// The routed circuit contains an unsupported operation.
    UnsupportedOperation {
        operation_index: usize,
        operation: String,
    },

    /// Verification discovered an invalid routing permutation.
    InvalidPermutation {
        detail: String,
    },

    /// Verification discovered an internal invariant failure.
    InvariantViolation {
        detail: String,
    },
}

impl std::fmt::Display for VerificationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidInitialMapping { detail } => {
                write!(f, "invalid initial mapping: {detail}")
            }

            Self::InvalidFinalMapping { detail } => {
                write!(f, "invalid final mapping: {detail}")
            }

            Self::MappingCollision { physical } => {
                write!(f, "routing verification found physical collision at {physical}")
            }

            Self::IllegalOperation {
                gate,
                physical_operands,
            } => write!(
                f,
                "gate `{gate}` is illegal on physical operands {physical_operands:?}"
            ),

            Self::IllegalDirection { gate, from, to } => write!(
                f,
                "gate `{gate}` has illegal physical direction {from} -> {to}"
            ),

            Self::IllegalMovement { movement, a, b } => write!(
                f,
                "routing movement `{movement}` is illegal on {a} <-> {b}"
            ),

            Self::GateSequenceMismatch { operation_index } => {
                write!(f, "gate sequence mismatch at operation {operation_index}")
            }

            Self::QubitSemanticsChanged { logical } => {
                write!(f, "routing changed semantics of logical qubit `{logical}`")
            }

            Self::MeasurementMismatch { operation_index } => {
                write!(
                    f,
                    "measurement semantics mismatch at operation {operation_index}"
                )
            }

            Self::ClassicalMappingMismatch { operation_index } => {
                write!(
                    f,
                    "classical mapping mismatch at operation {operation_index}"
                )
            }

            Self::UnsupportedOperation {
                operation_index,
                operation,
            } => write!(
                f,
                "unsupported routed operation `{operation}` at index {operation_index}"
            ),

            Self::InvalidPermutation { detail } => {
                write!(f, "invalid routing permutation: {detail}")
            }

            Self::InvariantViolation { detail } => {
                write!(f, "routing verification invariant violated: {detail}")
            }
        }
    }
}

// =============================================================================
// Transaction errors
// =============================================================================

/// Failure involving transactional routing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransactionError {
    /// A transaction was already active.
    AlreadyActive,

    /// A commit was requested without an active transaction.
    NotActive,

    /// Rollback could not restore the previous state.
    RollbackFailed {
        detail: String,
    },

    /// Commit failed validation.
    CommitFailed {
        detail: String,
    },

    /// A speculative state was mutated after its transaction became invalid.
    InvalidState,

    /// The transaction exceeded its allowed speculative resource budget.
    ResourceLimitExceeded {
        limit: usize,
    },
}

impl std::fmt::Display for TransactionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AlreadyActive => {
                write!(f, "routing transaction is already active")
            }

            Self::NotActive => {
                write!(f, "routing transaction is not active")
            }

            Self::RollbackFailed { detail } => {
                write!(f, "routing transaction rollback failed: {detail}")
            }

            Self::CommitFailed { detail } => {
                write!(f, "routing transaction commit failed: {detail}")
            }

            Self::InvalidState => {
                write!(f, "routing transaction is in an invalid state")
            }

            Self::ResourceLimitExceeded { limit } => {
                write!(
                    f,
                    "routing transaction resource limit exceeded: {limit}"
                )
            }
        }
    }
}

// =============================================================================
// Integration errors
// =============================================================================

/// Failure at the boundary between routing and another Zamani subsystem.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IntegrationError {
    /// Quantum IR cannot be converted into the routing input representation.
    InvalidInputRepresentation {
        detail: String,
    },

    /// Routing output cannot be converted back into the destination IR.
    InvalidOutputRepresentation {
        detail: String,
    },

    /// A required integration capability is unavailable.
    MissingCapability {
        capability: String,
    },

    /// Routing and hardware target representations are incompatible.
    HardwareTargetMismatch {
        detail: String,
    },

    /// Routing and Quantum IR representations are incompatible.
    QuantumIrMismatch {
        detail: String,
    },

    /// Routing and decomposition stages disagree about an operation.
    DecompositionBoundaryMismatch {
        detail: String,
    },

    /// Routing and scheduling disagree about timing semantics.
    SchedulingBoundaryMismatch {
        detail: String,
    },

    /// Routing and benchmarking integration cannot collect required metrics.
    BenchmarkingBoundaryMismatch {
        detail: String,
    },

    /// Routing and QEC integration cannot preserve required qubit roles.
    QecBoundaryMismatch {
        detail: String,
    },
}

impl std::fmt::Display for IntegrationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidInputRepresentation { detail } => {
                write!(f, "invalid routing input representation: {detail}")
            }

            Self::InvalidOutputRepresentation { detail } => {
                write!(f, "invalid routing output representation: {detail}")
            }

            Self::MissingCapability { capability } => {
                write!(f, "required integration capability is missing: {capability}")
            }

            Self::HardwareTargetMismatch { detail } => {
                write!(f, "routing/hardware target mismatch: {detail}")
            }

            Self::QuantumIrMismatch { detail } => {
                write!(f, "routing/Quantum IR mismatch: {detail}")
            }

            Self::DecompositionBoundaryMismatch { detail } => {
                write!(f, "routing/decomposition boundary mismatch: {detail}")
            }

            Self::SchedulingBoundaryMismatch { detail } => {
                write!(f, "routing/scheduling boundary mismatch: {detail}")
            }

            Self::BenchmarkingBoundaryMismatch { detail } => {
                write!(f, "routing/benchmarking boundary mismatch: {detail}")
            }

            Self::QecBoundaryMismatch { detail } => {
                write!(f, "routing/QEC boundary mismatch: {detail}")
            }
        }
    }
}

// =============================================================================
// Top-level error kind
// =============================================================================

/// Machine-readable routing failure category.
///
/// This is intentionally separate from `RoutingError` so callers can inspect
/// the category without parsing a diagnostic string.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RoutingErrorKind {
    /// Topology construction/validation failure.
    Topology(TopologyError),

    /// Logical/physical mapping failure.
    Mapping(MappingError),

    /// Initial-layout failure.
    Layout(LayoutError),

    /// Path-finding failure.
    Path(PathError),

    /// Candidate-generation failure.
    Candidate(CandidateError),

    /// Routing algorithm failure.
    Algorithm(AlgorithmError),

    /// Movement-generation failure.
    Movement(MovementError),

    /// Quantum operation/gate failure.
    Operation(OperationError),

    /// Routing configuration failure.
    Configuration(ConfigurationError),

    /// Cost-model failure.
    CostModel(CostModelError),

    /// Resource exhaustion.
    Resource(ResourceError),

    /// Verification failure.
    Verification(VerificationError),

    /// Transaction failure.
    Transaction(TransactionError),

    /// Integration boundary failure.
    Integration(IntegrationError),

    /// Generic invalid input not covered by a more specific category.
    InvalidInput {
        detail: String,
    },

    /// Unsupported routing capability.
    Unsupported {
        capability: String,
    },

    /// Internal routing invariant violation.
    InternalInvariant {
        detail: String,
    },
}

impl RoutingErrorKind {
    /// Returns the broad routing stage associated with this error.
    pub const fn stage(&self) -> RoutingStage {
        match self {
            Self::Topology(_) => RoutingStage::Topology,
            Self::Mapping(_) => RoutingStage::Mapping,
            Self::Layout(_) => RoutingStage::Layout,
            Self::Path(_) => RoutingStage::PathFinding,
            Self::Candidate(_) => RoutingStage::CandidateGeneration,
            Self::Algorithm(_) => RoutingStage::Routing,
            Self::Movement(_) => RoutingStage::Movement,
            Self::Operation(_) => RoutingStage::InputValidation,
            Self::Configuration(_) => RoutingStage::Configuration,
            Self::CostModel(_) => RoutingStage::Routing,
            Self::Resource(_) => RoutingStage::Routing,
            Self::Verification(_) => RoutingStage::Verification,
            Self::Transaction(_) => RoutingStage::Transaction,
            Self::Integration(_) => RoutingStage::Transpilation,
            Self::InvalidInput { .. } => RoutingStage::InputValidation,
            Self::Unsupported { .. } => RoutingStage::InputValidation,
            Self::InternalInvariant { .. } => RoutingStage::Routing,
        }
    }

    /// Returns the appropriate severity classification.
    pub const fn severity(&self) -> RoutingErrorSeverity {
        match self {
            Self::Algorithm(AlgorithmError::NoLegalMove { .. })
            | Self::Algorithm(AlgorithmError::NoValidResult { .. })
            | Self::Path(PathError::NoPath { .. })
            | Self::Candidate(CandidateError::NoCandidate)
            | Self::Candidate(CandidateError::EmptyCandidateSet) => {
                RoutingErrorSeverity::Recoverable
            }

            Self::InternalInvariant { .. }
            | Self::Algorithm(AlgorithmError::InvariantViolation { .. })
            | Self::Verification(VerificationError::InvariantViolation { .. }) => {
                RoutingErrorSeverity::Internal
            }

            _ => RoutingErrorSeverity::Error,
        }
    }

    /// Returns a stable machine-readable category name.
    pub const fn code(&self) -> &'static str {
        match self {
            Self::Topology(_) => "routing.topology",
            Self::Mapping(_) => "routing.mapping",
            Self::Layout(_) => "routing.layout",
            Self::Path(_) => "routing.path",
            Self::Candidate(_) => "routing.candidate",
            Self::Algorithm(_) => "routing.algorithm",
            Self::Movement(_) => "routing.movement",
            Self::Operation(_) => "routing.operation",
            Self::Configuration(_) => "routing.configuration",
            Self::CostModel(_) => "routing.cost_model",
            Self::Resource(_) => "routing.resource",
            Self::Verification(_) => "routing.verification",
            Self::Transaction(_) => "routing.transaction",
            Self::Integration(_) => "routing.integration",
            Self::InvalidInput { .. } => "routing.invalid_input",
            Self::Unsupported { .. } => "routing.unsupported",
            Self::InternalInvariant { .. } => "routing.internal_invariant",
        }
    }
}

impl std::fmt::Display for RoutingErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Topology(error) => write!(f, "{error}"),
            Self::Mapping(error) => write!(f, "{error}"),
            Self::Layout(error) => write!(f, "{error}"),
            Self::Path(error) => write!(f, "{error}"),
            Self::Candidate(error) => write!(f, "{error}"),
            Self::Algorithm(error) => write!(f, "{error}"),
            Self::Movement(error) => write!(f, "{error}"),
            Self::Operation(error) => write!(f, "{error}"),
            Self::Configuration(error) => write!(f, "{error}"),
            Self::CostModel(error) => write!(f, "{error}"),
            Self::Resource(error) => write!(f, "{error}"),
            Self::Verification(error) => write!(f, "{error}"),
            Self::Transaction(error) => write!(f, "{error}"),
            Self::Integration(error) => write!(f, "{error}"),

            Self::InvalidInput { detail } => {
                write!(f, "invalid routing input: {detail}")
            }

            Self::Unsupported { capability } => {
                write!(f, "unsupported routing capability: {capability}")
            }

            Self::InternalInvariant { detail } => {
                write!(f, "internal routing invariant violation: {detail}")
            }
        }
    }
}

// =============================================================================
// Top-level RoutingError
// =============================================================================

/// Canonical top-level error returned by the Zamani routing subsystem.
///
/// `RoutingError` contains:
///
/// - a structured machine-readable kind;
/// - optional diagnostic context.
///
/// The concrete routing implementation files should return this type through
/// `RoutingResult<T>`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoutingError {
    /// Machine-readable routing error category.
    pub kind: RoutingErrorKind,

    /// Optional structured diagnostic context.
    pub context: RoutingErrorContext,
}

impl RoutingError {
    /// Creates a routing error without additional context.
    pub fn new(kind: RoutingErrorKind) -> Self {
        Self {
            kind,
            context: RoutingErrorContext::new(),
        }
    }

    /// Creates a routing error with diagnostic context.
    pub fn with_context(
        kind: RoutingErrorKind,
        context: RoutingErrorContext,
    ) -> Self {
        Self { kind, context }
    }

    /// Returns the stable machine-readable error code.
    pub const fn code(&self) -> &'static str {
        self.kind.code()
    }

    /// Returns the routing stage associated with the error.
    pub const fn stage(&self) -> RoutingStage {
        self.kind.stage()
    }

    /// Returns the error severity.
    pub const fn severity(&self) -> RoutingErrorSeverity {
        self.kind.severity()
    }

    /// Returns true if this represents an internal invariant violation.
    pub const fn is_internal(&self) -> bool {
        self.severity().is_internal()
    }

    /// Adds/replaces diagnostic context.
    pub fn with_diagnostic_context(
        mut self,
        context: RoutingErrorContext,
    ) -> Self {
        self.context = context;
        self
    }

    // -------------------------------------------------------------------------
    // Topology constructors
    // -------------------------------------------------------------------------

    pub fn empty_topology() -> Self {
        Self::new(RoutingErrorKind::Topology(TopologyError::Empty))
    }

    pub fn invalid_topology(detail: impl Into<String>) -> Self {
        Self::new(RoutingErrorKind::Topology(
            TopologyError::InvalidMetadata {
                field: detail.into(),
            },
        ))
    }

    pub fn disconnected(from: usize, to: usize) -> Self {
        Self::new(RoutingErrorKind::Topology(
            TopologyError::Disconnected { from, to },
        ))
    }

    pub fn missing_topology_vertex(qubit: usize) -> Self {
        Self::new(RoutingErrorKind::Topology(
            TopologyError::MissingVertex { qubit },
        ))
    }

    pub fn duplicate_topology_edge(a: usize, b: usize) -> Self {
        Self::new(RoutingErrorKind::Topology(
            TopologyError::DuplicateEdge { a, b },
        ))
    }

    pub fn self_loop(qubit: usize) -> Self {
        Self::new(RoutingErrorKind::Topology(
            TopologyError::SelfLoop { qubit },
        ))
    }

    pub fn asymmetric_topology_edge(from: usize, to: usize) -> Self {
        Self::new(RoutingErrorKind::Topology(
            TopologyError::AsymmetricEdge { from, to },
        ))
    }

    pub fn gate_not_supported(
        gate: impl Into<String>,
        from: usize,
        to: usize,
    ) -> Self {
        Self::new(RoutingErrorKind::Topology(
            TopologyError::GateNotSupported {
                gate: gate.into(),
                from,
                to,
            },
        ))
    }

    pub fn gate_direction_unsupported(
        gate: impl Into<String>,
        from: usize,
        to: usize,
    ) -> Self {
        Self::new(RoutingErrorKind::Topology(
            TopologyError::GateDirectionUnsupported {
                gate: gate.into(),
                from,
                to,
            },
        ))
    }

    pub fn qubit_unavailable(qubit: usize) -> Self {
        Self::new(RoutingErrorKind::Topology(
            TopologyError::QubitUnavailable { qubit },
        ))
    }

    // -------------------------------------------------------------------------
    // Mapping constructors
    // -------------------------------------------------------------------------

    pub fn logical_qubit_already_mapped(
        logical: impl Into<String>,
    ) -> Self {
        Self::new(RoutingErrorKind::Mapping(
            MappingError::LogicalAlreadyMapped {
                logical: logical.into(),
            },
        ))
    }

    pub fn physical_qubit_already_assigned(physical: usize) -> Self {
        Self::new(RoutingErrorKind::Mapping(
            MappingError::PhysicalAlreadyAssigned { physical },
        ))
    }

    pub fn unknown_logical_qubit(
        logical: impl Into<String>,
    ) -> Self {
        Self::new(RoutingErrorKind::Mapping(
            MappingError::UnknownLogicalQubit {
                logical: logical.into(),
            },
        ))
    }

    pub fn unknown_physical_qubit(physical: usize) -> Self {
        Self::new(RoutingErrorKind::Mapping(
            MappingError::UnknownPhysicalQubit { physical },
        ))
    }

    pub fn mapping_collision(
        logical: impl Into<String>,
        physical: usize,
    ) -> Self {
        Self::new(RoutingErrorKind::Mapping(
            MappingError::Collision {
                logical: logical.into(),
                physical,
            },
        ))
    }

    pub fn inconsistent_mapping(detail: impl Into<String>) -> Self {
        Self::new(RoutingErrorKind::Mapping(
            MappingError::Inconsistent {
                detail: detail.into(),
            },
        ))
    }

    // -------------------------------------------------------------------------
    // Layout constructors
    // -------------------------------------------------------------------------

    pub fn insufficient_physical_qubits(
        required: usize,
        available: usize,
    ) -> Self {
        Self::new(RoutingErrorKind::Resource(
            ResourceError::InsufficientPhysicalQubits {
                required,
                available,
            },
        ))
    }

    pub fn no_valid_layout() -> Self {
        Self::new(RoutingErrorKind::Layout(LayoutError::NoValidLayout))
    }

    pub fn unsupported_layout_strategy(
        strategy: impl Into<String>,
    ) -> Self {
        Self::new(RoutingErrorKind::Layout(
            LayoutError::UnsupportedStrategy {
                strategy: strategy.into(),
            },
        ))
    }

    // -------------------------------------------------------------------------
    // Path constructors
    // -------------------------------------------------------------------------

    pub fn no_routing_path(from: usize, to: usize) -> Self {
        Self::new(RoutingErrorKind::Path(PathError::NoPath {
            from,
            to,
        }))
    }

    pub fn invalid_path(detail: impl Into<String>) -> Self {
        Self::new(RoutingErrorKind::Path(PathError::InvalidPath {
            detail: detail.into(),
        }))
    }

    // -------------------------------------------------------------------------
    // Candidate constructors
    // -------------------------------------------------------------------------

    pub fn no_candidate() -> Self {
        Self::new(RoutingErrorKind::Candidate(
            CandidateError::NoCandidate,
        ))
    }

    pub fn empty_candidate_set() -> Self {
        Self::new(RoutingErrorKind::Candidate(
            CandidateError::EmptyCandidateSet,
        ))
    }

    // -------------------------------------------------------------------------
    // Algorithm constructors
    // -------------------------------------------------------------------------

    pub fn unsupported_algorithm(
        algorithm: impl Into<String>,
    ) -> Self {
        Self::new(RoutingErrorKind::Algorithm(
            AlgorithmError::Unsupported {
                algorithm: algorithm.into(),
            },
        ))
    }

    pub fn algorithm_incompatible(
        algorithm: impl Into<String>,
        reason: impl Into<String>,
    ) -> Self {
        Self::new(RoutingErrorKind::Algorithm(
            AlgorithmError::Incompatible {
                algorithm: algorithm.into(),
                reason: reason.into(),
            },
        ))
    }

    pub fn routing_timeout() -> Self {
        Self::new(RoutingErrorKind::Algorithm(
            AlgorithmError::Timeout,
        ))
    }

    pub fn iteration_limit_exceeded(limit: usize) -> Self {
        Self::new(RoutingErrorKind::Resource(
            ResourceError::IterationLimitExceeded { limit },
        ))
    }

    pub fn swap_limit_exceeded(limit: usize) -> Self {
        Self::new(RoutingErrorKind::Resource(
            ResourceError::SwapLimitExceeded { limit },
        ))
    }

    pub fn candidate_limit_exceeded(limit: usize) -> Self {
        Self::new(RoutingErrorKind::Resource(
            ResourceError::CandidateLimitExceeded { limit },
        ))
    }

    // -------------------------------------------------------------------------
    // Operation constructors
    // -------------------------------------------------------------------------

    pub fn unsupported_gate(gate: impl Into<String>) -> Self {
        Self::new(RoutingErrorKind::Operation(
            OperationError::UnsupportedGate {
                gate: gate.into(),
            },
        ))
    }

    pub fn unsupported_arity(
        gate: impl Into<String>,
        arity: usize,
    ) -> Self {
        Self::new(RoutingErrorKind::Operation(
            OperationError::UnsupportedArity {
                gate: gate.into(),
                arity,
            },
        ))
    }

    pub fn requires_decomposition(
        gate: impl Into<String>,
        arity: usize,
    ) -> Self {
        Self::new(RoutingErrorKind::Operation(
            OperationError::RequiresDecomposition {
                gate: gate.into(),
                arity,
            },
        ))
    }

    pub fn invalid_operand(operand: impl Into<String>) -> Self {
        Self::new(RoutingErrorKind::Operation(
            OperationError::InvalidOperand {
                operand: operand.into(),
            },
        ))
    }

    // -------------------------------------------------------------------------
    // Movement constructors
    // -------------------------------------------------------------------------

    pub fn non_adjacent_movement(a: usize, b: usize) -> Self {
        Self::new(RoutingErrorKind::Movement(
            MovementError::NonAdjacent { a, b },
        ))
    }

    pub fn unsupported_movement(
        movement: impl Into<String>,
    ) -> Self {
        Self::new(RoutingErrorKind::Movement(
            MovementError::Unsupported {
                movement: movement.into(),
            },
        ))
    }

    pub fn missing_movement_decomposition(
        movement: impl Into<String>,
    ) -> Self {
        Self::new(RoutingErrorKind::Movement(
            MovementError::MissingDecomposition {
                movement: movement.into(),
            },
        ))
    }

    // -------------------------------------------------------------------------
    // Configuration constructors
    // -------------------------------------------------------------------------

    pub fn invalid_configuration(
        field: impl Into<String>,
        reason: impl Into<String>,
    ) -> Self {
        Self::new(RoutingErrorKind::Configuration(
            ConfigurationError::InvalidValue {
                field: field.into(),
                reason: reason.into(),
            },
        ))
    }

    pub fn incompatible_configuration(
        first: impl Into<String>,
        second: impl Into<String>,
    ) -> Self {
        Self::new(RoutingErrorKind::Configuration(
            ConfigurationError::IncompatibleOptions {
                first: first.into(),
                second: second.into(),
            },
        ))
    }

    // -------------------------------------------------------------------------
    // Verification constructors
    // -------------------------------------------------------------------------

    pub fn verification_failed(detail: impl Into<String>) -> Self {
        Self::new(RoutingErrorKind::Verification(
            VerificationError::InvariantViolation {
                detail: detail.into(),
            },
        ))
    }

    pub fn illegal_routed_operation(
        gate: impl Into<String>,
        physical_operands: Vec<usize>,
    ) -> Self {
        Self::new(RoutingErrorKind::Verification(
            VerificationError::IllegalOperation {
                gate: gate.into(),
                physical_operands,
            },
        ))
    }

    pub fn measurement_mismatch(operation_index: usize) -> Self {
        Self::new(RoutingErrorKind::Verification(
            VerificationError::MeasurementMismatch {
                operation_index,
            },
        ))
    }

    // -------------------------------------------------------------------------
    // Transaction constructors
    // -------------------------------------------------------------------------

    pub fn transaction_rollback_failed(
        detail: impl Into<String>,
    ) -> Self {
        Self::new(RoutingErrorKind::Transaction(
            TransactionError::RollbackFailed {
                detail: detail.into(),
            },
        ))
    }

    pub fn transaction_commit_failed(
        detail: impl Into<String>,
    ) -> Self {
        Self::new(RoutingErrorKind::Transaction(
            TransactionError::CommitFailed {
                detail: detail.into(),
            },
        ))
    }

    // -------------------------------------------------------------------------
    // Integration constructors
    // -------------------------------------------------------------------------

    pub fn quantum_ir_mismatch(detail: impl Into<String>) -> Self {
        Self::new(RoutingErrorKind::Integration(
            IntegrationError::QuantumIrMismatch {
                detail: detail.into(),
            },
        ))
    }

    pub fn hardware_target_mismatch(
        detail: impl Into<String>,
    ) -> Self {
        Self::new(RoutingErrorKind::Integration(
            IntegrationError::HardwareTargetMismatch {
                detail: detail.into(),
            },
        ))
    }

    pub fn decomposition_boundary_mismatch(
        detail: impl Into<String>,
    ) -> Self {
        Self::new(RoutingErrorKind::Integration(
            IntegrationError::DecompositionBoundaryMismatch {
                detail: detail.into(),
            },
        ))
    }

    // -------------------------------------------------------------------------
    // Generic constructors
    // -------------------------------------------------------------------------

    pub fn invalid_input(detail: impl Into<String>) -> Self {
        Self::new(RoutingErrorKind::InvalidInput {
            detail: detail.into(),
        })
    }

    pub fn unsupported_capability(
        capability: impl Into<String>,
    ) -> Self {
        Self::new(RoutingErrorKind::Unsupported {
            capability: capability.into(),
        })
    }

    pub fn internal_invariant(detail: impl Into<String>) -> Self {
        Self::new(RoutingErrorKind::InternalInvariant {
            detail: detail.into(),
        })
    }
}

impl std::fmt::Display for RoutingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.code(), self.kind)?;

        if let Some(stage) = self.context.stage {
            write!(f, " [stage={stage}]")?;
        }

        if let Some(algorithm) = &self.context.algorithm {
            write!(f, " [algorithm={algorithm}]")?;
        }

        if let Some(topology) = &self.context.topology {
            write!(f, " [topology={topology}]")?;
        }

        if let Some(operation_index) = self.context.operation_index {
            write!(f, " [operation={operation_index}]")?;
        }

        if let Some(gate) = &self.context.gate {
            write!(f, " [gate={gate}]")?;
        }

        if let Some(logical) = &self.context.logical_qubit {
            write!(f, " [logical={logical}]")?;
        }

        if let Some(physical) = self.context.physical_qubit {
            write!(f, " [physical={physical}]")?;
        }

        if let Some(detail) = &self.context.detail {
            write!(f, " [detail={detail}]")?;
        }

        Ok(())
    }
}

impl std::error::Error for RoutingError {}

// =============================================================================
// Conversion helpers
// =============================================================================

impl From<TopologyError> for RoutingError {
    fn from(error: TopologyError) -> Self {
        Self::new(RoutingErrorKind::Topology(error))
    }
}

impl From<MappingError> for RoutingError {
    fn from(error: MappingError) -> Self {
        Self::new(RoutingErrorKind::Mapping(error))
    }
}

impl From<LayoutError> for RoutingError {
    fn from(error: LayoutError) -> Self {
        Self::new(RoutingErrorKind::Layout(error))
    }
}

impl From<PathError> for RoutingError {
    fn from(error: PathError) -> Self {
        Self::new(RoutingErrorKind::Path(error))
    }
}

impl From<CandidateError> for RoutingError {
    fn from(error: CandidateError) -> Self {
        Self::new(RoutingErrorKind::Candidate(error))
    }
}

impl From<AlgorithmError> for RoutingError {
    fn from(error: AlgorithmError) -> Self {
        Self::new(RoutingErrorKind::Algorithm(error))
    }
}

impl From<MovementError> for RoutingError {
    fn from(error: MovementError) -> Self {
        Self::new(RoutingErrorKind::Movement(error))
    }
}

impl From<OperationError> for RoutingError {
    fn from(error: OperationError) -> Self {
        Self::new(RoutingErrorKind::Operation(error))
    }
}

impl From<ConfigurationError> for RoutingError {
    fn from(error: ConfigurationError) -> Self {
        Self::new(RoutingErrorKind::Configuration(error))
    }
}

impl From<CostModelError> for RoutingError {
    fn from(error: CostModelError) -> Self {
        Self::new(RoutingErrorKind::CostModel(error))
    }
}

impl From<ResourceError> for RoutingError {
    fn from(error: ResourceError) -> Self {
        Self::new(RoutingErrorKind::Resource(error))
    }
}

impl From<VerificationError> for RoutingError {
    fn from(error: VerificationError) -> Self {
        Self::new(RoutingErrorKind::Verification(error))
    }
}

impl From<TransactionError> for RoutingError {
    fn from(error: TransactionError) -> Self {
        Self::new(RoutingErrorKind::Transaction(error))
    }
}

impl From<IntegrationError> for RoutingError {
    fn from(error: IntegrationError) -> Self {
        Self::new(RoutingErrorKind::Integration(error))
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn routing_error_has_stable_code() {
        let error = RoutingError::empty_topology();

        assert_eq!(error.code(), "routing.topology");
        assert_eq!(error.stage(), RoutingStage::Topology);
        assert_eq!(error.severity(), RoutingErrorSeverity::Error);
    }

    #[test]
    fn disconnected_routing_error_is_recoverable() {
        let error = RoutingError::disconnected(0, 5);

        assert_eq!(error.code(), "routing.topology");
        assert_eq!(error.stage(), RoutingStage::Topology);

        // A disconnected topology is an input error at the top level, not an
        // internal invariant failure.
        assert!(!error.is_internal());
    }

    #[test]
    fn no_candidate_is_recoverable() {
        let error = RoutingError::no_candidate();

        assert_eq!(error.severity(), RoutingErrorSeverity::Recoverable);
        assert!(!error.is_internal());
    }

    #[test]
    fn invariant_error_is_internal() {
        let error = RoutingError::internal_invariant("mapping bijection violated");

        assert_eq!(
            error.severity(),
            RoutingErrorSeverity::Internal
        );
        assert!(error.is_internal());
        assert_eq!(
            error.code(),
            "routing.internal_invariant"
        );
    }

    #[test]
    fn context_is_preserved() {
        let context = RoutingErrorContext::new()
            .with_stage(RoutingStage::Routing)
            .with_algorithm("sabre")
            .with_topology("heavy-hex")
            .with_gate("cx")
            .with_logical_qubit("q0")
            .with_physical_qubit(4)
            .with_operation_index(17)
            .with_detail("no legal candidate");

        let error = RoutingError::with_context(
            RoutingErrorKind::Candidate(CandidateError::NoCandidate),
            context.clone(),
        );

        assert_eq!(error.context, context);
        assert_eq!(error.stage(), RoutingStage::CandidateGeneration);
    }

    #[test]
    fn display_contains_machine_readable_code() {
        let error = RoutingError::unknown_logical_qubit("q17");

        let message = error.to_string();

        assert!(message.contains("routing.mapping"));
        assert!(message.contains("q17"));
    }

    #[test]
    fn conversion_from_specific_error_works() {
        let error: RoutingError =
            TopologyError::SelfLoop { qubit: 3 }.into();

        assert_eq!(error.code(), "routing.topology");

        match error.kind {
            RoutingErrorKind::Topology(TopologyError::SelfLoop {
                qubit,
            }) => assert_eq!(qubit, 3),

            _ => panic!("unexpected routing error kind"),
        }
    }

    #[test]
    fn insufficient_qubits_is_structured() {
        let error = RoutingError::insufficient_physical_qubits(20, 12);

        match error.kind {
            RoutingErrorKind::Resource(
                ResourceError::InsufficientPhysicalQubits {
                    required,
                    available,
                },
            ) => {
                assert_eq!(required, 20);
                assert_eq!(available, 12);
            }

            _ => panic!("unexpected error kind"),
        }
    }

    #[test]
    fn unsupported_arity_is_structured() {
        let error = RoutingError::unsupported_arity("ccx", 3);

        match error.kind {
            RoutingErrorKind::Operation(
                OperationError::UnsupportedArity {
                    gate,
                    arity,
                },
            ) => {
                assert_eq!(gate, "ccx");
                assert_eq!(arity, 3);
            }

            _ => panic!("unexpected error kind"),
        }
    }

    #[test]
    fn timeout_is_structured() {
        let error = RoutingError::routing_timeout();

        match error.kind {
            RoutingErrorKind::Algorithm(AlgorithmError::Timeout) => {}

            _ => panic!("unexpected error kind"),
        }
    }

    #[test]
    fn transaction_errors_are_distinct() {
        let rollback =
            RoutingError::transaction_rollback_failed("state mismatch");

        let commit =
            RoutingError::transaction_commit_failed("verification failed");

        assert_eq!(
            rollback.code(),
            "routing.transaction"
        );

        assert_eq!(
            commit.code(),
            "routing.transaction"
        );

        assert_ne!(rollback, commit);
    }
}