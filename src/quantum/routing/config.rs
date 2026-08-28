//! Zamani Quantum Routing Configuration
//!
//! Production configuration contract for the logical-to-physical quantum
//! routing subsystem.
//!
//! # Responsibilities
//!
//! This module defines:
//!
//! - routing algorithm selection;
//! - initial-layout strategy selection;
//! - routing objective selection;
//! - verification policy;
//! - multi-qubit operation policy;
//! - directed-gate policy;
//! - routing failure policy;
//! - deterministic/reproducible execution settings;
//! - search limits;
//! - SABRE/lookahead controls;
//! - candidate limits;
//! - trial configuration;
//! - bridge/move policy;
//! - configuration validation;
//! - stable configuration introspection;
//! - configuration compatibility checks.
//!
//! # Architectural boundary
//!
//! `config.rs` is intentionally independent of:
//!
//! - topology implementation;
//! - qubit mapping implementation;
//! - routing algorithms;
//! - layout algorithms;
//! - hardware providers;
//! - compiler IR;
//! - QuantumCircuit implementation;
//! - hardware calibration;
//! - scheduling;
//! - execution.
//!
//! Those components consume this configuration contract.
//!
//! The dependency direction is:
//!
//! ```text
//!                     config.rs
//!                         │
//!          ┌──────────────┼──────────────┐
//!          │              │              │
//!          ▼              ▼              ▼
//!       layout         router        algorithms
//!          │              │              │
//!          └──────────────┼──────────────┘
//!                         ▼
//!                    verification
//! ```
//!
//! # Stability rule
//!
//! This file is intended to be frozen before implementation of the routing
//! algorithms. Later routing files must adapt to this configuration API rather
//! than requiring this file to be rewritten.
//!
//! # Safety
//!
//! - No `unsafe` code.
//! - No global mutable state.
//! - No environment-variable-dependent defaults.
//! - No filesystem access.
//! - No network access.
//! - No hardware/provider access.
//! - No random-number generation.
//!
//! A seed is stored here, but random number generation belongs to the
//! algorithm implementation.
//!
//! # Rust compatibility
//!
//! Target: Rust 1.97.1.
//!
//! Edition: 2021.
//!
//! No nightly features are required.

// =============================================================================
// Imports
// =============================================================================

use std::fmt;
use std::time::Duration;

// =============================================================================
// Configuration constants
// =============================================================================
//
// These constants provide conservative production defaults and hard safety
// ceilings. They are configuration-policy limits, not hardware limits.
//
// Hardware-specific limits belong to topology/hardware modules.
// Circuit-specific limits belong to Quantum IR limits.
//
// Keeping these limits here prevents pathological routing requests from
// accidentally creating unbounded search work.

/// Default maximum number of routing iterations.
pub const DEFAULT_MAX_ITERATIONS: usize = 100_000;

/// Default maximum number of inserted SWAPs.
///
/// `None` remains the public representation for "no explicit SWAP cap".
pub const DEFAULT_MAX_SWAPS: Option<usize> = None;

/// Default lookahead depth.
pub const DEFAULT_LOOKAHEAD_DEPTH: usize = 4;

/// Default number of candidate moves considered per decision.
pub const DEFAULT_CANDIDATE_LIMIT: usize = 64;

/// Default number of SABRE forward/backward iterations.
pub const DEFAULT_SABRE_ITERATIONS: usize = 4;

/// Default number of routing trials per SABRE iteration.
pub const DEFAULT_SABRE_TRIALS: usize = 1;

/// Default number of independent layout trials.
pub const DEFAULT_LAYOUT_TRIALS: usize = 1;

/// Default routing timeout.
///
/// `None` means no wall-clock timeout imposed by this configuration.
/// Individual callers may impose an outer timeout.
pub const DEFAULT_TIMEOUT: Option<Duration> = None;

/// Maximum supported lookahead depth through the public configuration API.
///
/// This prevents accidental requests for enormous future windows.
pub const MAX_LOOKAHEAD_DEPTH: usize = 4096;

/// Maximum number of candidates that a single routing decision may request.
pub const MAX_CANDIDATE_LIMIT: usize = 1_000_000;

/// Maximum SABRE iteration count accepted by configuration validation.
pub const MAX_SABRE_ITERATIONS: usize = 1_000_000;

/// Maximum number of SABRE trials accepted by configuration validation.
pub const MAX_SABRE_TRIALS: usize = 1_000_000;

/// Maximum number of independent layout trials accepted by configuration
/// validation.
pub const MAX_LAYOUT_TRIALS: usize = 1_000_000;

/// Maximum number of routing iterations accepted by configuration validation.
pub const MAX_ROUTING_ITERATIONS: usize = 100_000_000;

/// Maximum custom algorithm name length.
pub const MAX_CUSTOM_ALGORITHM_NAME_LENGTH: usize = 128;

/// Maximum custom layout name length.
pub const MAX_CUSTOM_LAYOUT_NAME_LENGTH: usize = 128;

/// Maximum custom objective name length.
pub const MAX_CUSTOM_OBJECTIVE_NAME_LENGTH: usize = 128;

// =============================================================================
// Routing algorithm
// =============================================================================

/// Routing algorithm selected by `RoutingConfig`.
///
/// The enum intentionally describes policy, not implementation details.
///
/// Concrete implementations live in:
///
/// ```text
/// routing/algorithms/
/// ```
///
/// This means `config.rs` remains independent from those modules.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RoutingAlgorithm {
    /// Automatically choose a suitable production algorithm.
    ///
    /// The actual selection belongs to `router.rs`.
    Auto,

    /// Do not insert routing moves.
    ///
    /// The router must fail if the circuit is not already executable on the
    /// target topology.
    None,

    /// Deterministic greedy routing.
    Basic,

    /// Deterministic shortest-path routing.
    ShortestPath,

    /// Lookahead heuristic routing.
    Lookahead,

    /// SABRE-style heuristic routing.
    Sabre,

    /// Hardware/noise-aware heuristic routing.
    NoiseAware,

    /// Dynamic/adaptive routing.
    Dynamic,

    /// Externally registered/custom algorithm.
    ///
    /// The name is resolved by the routing algorithm registry. The config
    /// layer does not load plugins itself.
    Custom(String),
}

impl RoutingAlgorithm {
    /// Returns a stable machine-readable name.
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::None => "none",
            Self::Basic => "basic",
            Self::ShortestPath => "shortest_path",
            Self::Lookahead => "lookahead",
            Self::Sabre => "sabre",
            Self::NoiseAware => "noise_aware",
            Self::Dynamic => "dynamic",
            Self::Custom(_) => "custom",
        }
    }

    /// Returns whether this algorithm requires candidate search.
    pub const fn requires_candidate_search(&self) -> bool {
        matches!(
            self,
            Self::Lookahead
                | Self::Sabre
                | Self::NoiseAware
                | Self::Dynamic
                | Self::Custom(_)
        )
    }

    /// Returns whether this algorithm supports lookahead configuration.
    pub const fn supports_lookahead(&self) -> bool {
        matches!(
            self,
            Self::Lookahead
                | Self::Sabre
                | Self::NoiseAware
                | Self::Dynamic
                | Self::Custom(_)
        )
    }

    /// Returns whether the algorithm is deterministic when given deterministic
    /// inputs and a deterministic configuration.
    ///
    /// Custom algorithms are conservatively treated as potentially
    /// nondeterministic.
    pub const fn has_known_determinism(&self) -> bool {
        !matches!(self, Self::Custom(_))
    }
}

impl Default for RoutingAlgorithm {
    fn default() -> Self {
        Self::Auto
    }
}

impl fmt::Display for RoutingAlgorithm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Custom(name) => write!(f, "custom:{name}"),
            _ => f.write_str(self.name()),
        }
    }
}

// =============================================================================
// Layout strategy
// =============================================================================

/// Initial logical-to-physical placement strategy.
///
/// Layout and routing remain separate compiler concepts even when SABRE or
/// another combined heuristic is used internally.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LayoutStrategy {
    /// Let the router choose an appropriate layout strategy.
    Auto,

    /// Map logical qubit `q[i]` to physical qubit `p[i]` where possible.
    Trivial,

    /// Prefer a highly connected physical region.
    Dense,

    /// Use the circuit interaction graph to choose a placement.
    InteractionGraph,

    /// Use hardware error/fidelity information when available.
    NoiseAware,

    /// Use SABRE-style bidirectional layout search.
    Sabre,

    /// Use a caller-provided mapping.
    ///
    /// The actual mapping is supplied to the router separately. This variant
    /// means the router must not replace it with an automatically generated
    /// initial layout.
    Fixed,

    /// External/custom layout strategy.
    Custom(String),
}

impl LayoutStrategy {
    /// Stable machine-readable name.
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Trivial => "trivial",
            Self::Dense => "dense",
            Self::InteractionGraph => "interaction_graph",
            Self::NoiseAware => "noise_aware",
            Self::Sabre => "sabre",
            Self::Fixed => "fixed",
            Self::Custom(_) => "custom",
        }
    }

    /// Returns whether this strategy requires circuit interaction analysis.
    pub const fn requires_interaction_analysis(&self) -> bool {
        matches!(
            self,
            Self::Dense
                | Self::InteractionGraph
                | Self::NoiseAware
                | Self::Sabre
                | Self::Custom(_)
        )
    }

    /// Returns whether the strategy expects a caller-provided mapping.
    pub const fn requires_external_mapping(&self) -> bool {
        matches!(self, Self::Fixed)
    }
}

impl Default for LayoutStrategy {
    fn default() -> Self {
        Self::Auto
    }
}

impl fmt::Display for LayoutStrategy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Custom(name) => write!(f, "custom:{name}"),
            _ => f.write_str(self.name()),
        }
    }
}

// =============================================================================
// Routing objective
// =============================================================================

/// Primary objective used to compare routing candidates.
///
/// The actual numerical implementation belongs to `cost.rs`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RoutingObjective {
    /// Minimize the number of inserted SWAP operations.
    SwapCount,

    /// Minimize routed circuit depth.
    Depth,

    /// Minimize physical execution duration.
    Duration,

    /// Minimize estimated execution error.
    Error,

    /// Maximize estimated fidelity.
    Fidelity,

    /// Combine multiple cost dimensions using configured weights.
    Weighted,

    /// Compare objectives in a fixed lexicographic order.
    Lexicographic,

    /// External/custom objective.
    Custom(String),
}

impl RoutingObjective {
    /// Stable machine-readable name.
    pub const fn name(&self) -> &'static str {
        match self {
            Self::SwapCount => "swap_count",
            Self::Depth => "depth",
            Self::Duration => "duration",
            Self::Error => "error",
            Self::Fidelity => "fidelity",
            Self::Weighted => "weighted",
            Self::Lexicographic => "lexicographic",
            Self::Custom(_) => "custom",
        }
    }

    /// Returns whether this objective requires hardware calibration data.
    pub const fn requires_hardware_properties(&self) -> bool {
        matches!(
            self,
            Self::Duration
                | Self::Error
                | Self::Fidelity
                | Self::Weighted
                | Self::Lexicographic
                | Self::Custom(_)
        )
    }
}

impl Default for RoutingObjective {
    fn default() -> Self {
        Self::SwapCount
    }
}

impl fmt::Display for RoutingObjective {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Custom(name) => write!(f, "custom:{name}"),
            _ => f.write_str(self.name()),
        }
    }
}

// =============================================================================
// Verification level
// =============================================================================

/// Verification strength applied to a routing result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum VerificationLevel {
    /// Do not run routing verification.
    ///
    /// This is intended only for controlled internal performance experiments.
    None,

    /// Verify cheap structural invariants.
    Basic,

    /// Verify normal production routing invariants.
    Standard,

    /// Perform the strongest available verification.
    ///
    /// Intended for CI, testing, debugging, and safety-critical compilation
    /// paths.
    Strict,
}

impl Default for VerificationLevel {
    fn default() -> Self {
        Self::Standard
    }
}

impl VerificationLevel {
    /// Returns whether any verification is requested.
    pub const fn enabled(self) -> bool {
        !matches!(self, Self::None)
    }

    /// Returns whether strict verification is requested.
    pub const fn is_strict(self) -> bool {
        matches!(self, Self::Strict)
    }

    /// Stable machine-readable name.
    pub const fn name(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Basic => "basic",
            Self::Standard => "standard",
            Self::Strict => "strict",
        }
    }
}

impl fmt::Display for VerificationLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

// =============================================================================
// Multi-qubit policy
// =============================================================================

/// Policy for operations containing more than two qubits.
///
/// Routing itself should not silently synthesize arbitrary multi-qubit gates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MultiQubitPolicy {
    /// Reject unsupported operations instead of making assumptions.
    Reject,

    /// Permit operations only when the hardware target explicitly reports
    /// support.
    NativeOnly,

    /// Allow a downstream decomposition boundary to handle unsupported
    /// operations.
    Decompose,

    /// Permit an algorithm-specific policy to decide.
    Auto,
}

impl Default for MultiQubitPolicy {
    fn default() -> Self {
        Self::NativeOnly
    }
}

impl MultiQubitPolicy {
    /// Stable machine-readable name.
    pub const fn name(self) -> &'static str {
        match self {
            Self::Reject => "reject",
            Self::NativeOnly => "native_only",
            Self::Decompose => "decompose",
            Self::Auto => "auto",
        }
    }
}

impl fmt::Display for MultiQubitPolicy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

// =============================================================================
// Direction policy
// =============================================================================

/// Policy for direction-sensitive hardware gates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DirectionPolicy {
    /// Require the requested gate direction to be directly supported.
    Strict,

    /// Permit a downstream gate-synthesis/decomposition stage to reverse the
    /// direction when a legal decomposition exists.
    AllowReversal,

    /// Let the router choose according to the available target capabilities.
    Auto,
}

impl Default for DirectionPolicy {
    fn default() -> Self {
        Self::Strict
    }
}

impl DirectionPolicy {
    /// Stable machine-readable name.
    pub const fn name(self) -> &'static str {
        match self {
            Self::Strict => "strict",
            Self::AllowReversal => "allow_reversal",
            Self::Auto => "auto",
        }
    }
}

impl fmt::Display for DirectionPolicy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

// =============================================================================
// Routing mode
// =============================================================================

/// Overall routing execution mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RoutingMode {
    /// Fail whenever the requested routing contract cannot be satisfied.
    Strict,

    /// Permit a configured algorithm to return the best valid route found
    /// before a search limit is reached.
    ///
    /// This does not permit returning an invalid circuit.
    BestEffort,

    /// Permit heuristic/approximate search policies while retaining output
    /// validity requirements.
    Approximate,
}

impl Default for RoutingMode {
    fn default() -> Self {
        Self::Strict
    }
}

impl RoutingMode {
    /// Stable machine-readable name.
    pub const fn name(self) -> &'static str {
        match self {
            Self::Strict => "strict",
            Self::BestEffort => "best_effort",
            Self::Approximate => "approximate",
        }
    }
}

impl fmt::Display for RoutingMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

// =============================================================================
// Weighted cost configuration
// =============================================================================

/// Weights for multi-objective routing.
///
/// All values must be finite and non-negative.
///
/// The actual interpretation belongs to `cost.rs`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RoutingWeights {
    /// SWAP insertion penalty.
    pub swap_count: f64,

    /// Circuit-depth penalty.
    pub depth: f64,

    /// Execution-duration penalty.
    pub duration: f64,

    /// Estimated-error penalty.
    pub error: f64,

    /// Fidelity penalty/reward component.
    ///
    /// The cost implementation defines the exact normalization.
    pub fidelity: f64,
}

impl RoutingWeights {
    /// Creates a new set of weights.
    pub const fn new(
        swap_count: f64,
        depth: f64,
        duration: f64,
        error: f64,
        fidelity: f64,
    ) -> Self {
        Self {
            swap_count,
            depth,
            duration,
            error,
            fidelity,
        }
    }

    /// Returns the default routing weighting.
    pub const fn default_values() -> Self {
        Self {
            swap_count: 1.0,
            depth: 0.0,
            duration: 0.0,
            error: 0.0,
            fidelity: 0.0,
        }
    }

    /// Returns whether all weights are valid.
    pub fn is_valid(&self) -> bool {
        self.swap_count.is_finite()
            && self.depth.is_finite()
            && self.duration.is_finite()
            && self.error.is_finite()
            && self.fidelity.is_finite()
            && self.swap_count >= 0.0
            && self.depth >= 0.0
            && self.duration >= 0.0
            && self.error >= 0.0
            && self.fidelity >= 0.0
    }

    /// Returns the sum of all weights.
    ///
    /// Returns `None` when the result is non-finite.
    pub fn sum(&self) -> Option<f64> {
        let sum = self.swap_count
            + self.depth
            + self.duration
            + self.error
            + self.fidelity;

        if sum.is_finite() {
            Some(sum)
        } else {
            None
        }
    }

    /// Returns whether all weights are zero.
    pub fn is_zero(&self) -> bool {
        self.swap_count == 0.0
            && self.depth == 0.0
            && self.duration == 0.0
            && self.error == 0.0
            && self.fidelity == 0.0
    }
}

impl Default for RoutingWeights {
    fn default() -> Self {
        Self::default_values()
    }
}

// =============================================================================
// Search limits
// =============================================================================

/// Hard limits governing routing search.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoutingLimits {
    /// Maximum number of algorithm iterations.
    pub max_iterations: usize,

    /// Maximum number of inserted SWAPs.
    ///
    /// `None` means no explicit SWAP limit.
    pub max_swaps: Option<usize>,

    /// Maximum wall-clock duration of the routing operation.
    ///
    /// `None` means no configuration-level timeout.
    pub timeout: Option<Duration>,

    /// Number of future interaction layers considered by lookahead
    /// algorithms.
    pub lookahead_depth: usize,

    /// Maximum number of candidate moves considered for one routing decision.
    pub candidate_limit: usize,

    /// Number of forward/backward SABRE iterations.
    pub sabre_iterations: usize,

    /// Number of routing trials for each SABRE iteration.
    pub sabre_trials: usize,

    /// Number of independent initial-layout trials.
    pub layout_trials: usize,
}

impl Default for RoutingLimits {
    fn default() -> Self {
        Self {
            max_iterations: DEFAULT_MAX_ITERATIONS,
            max_swaps: DEFAULT_MAX_SWAPS,
            timeout: DEFAULT_TIMEOUT,
            lookahead_depth: DEFAULT_LOOKAHEAD_DEPTH,
            candidate_limit: DEFAULT_CANDIDATE_LIMIT,
            sabre_iterations: DEFAULT_SABRE_ITERATIONS,
            sabre_trials: DEFAULT_SABRE_TRIALS,
            layout_trials: DEFAULT_LAYOUT_TRIALS,
        }
    }
}

impl RoutingLimits {
    /// Validates all search limits.
    pub fn validate(&self) -> Result<(), ConfigValidationError> {
        if self.max_iterations == 0 {
            return Err(ConfigValidationError::ZeroLimit {
                field: "max_iterations",
            });
        }

        if self.max_iterations > MAX_ROUTING_ITERATIONS {
            return Err(ConfigValidationError::LimitExceeded {
                field: "max_iterations",
                value: self.max_iterations,
                maximum: MAX_ROUTING_ITERATIONS,
            });
        }

        if let Some(max_swaps) = self.max_swaps {
            if max_swaps == 0 {
                // Zero is meaningful: explicitly prohibit inserted SWAPs.
                // Therefore it is valid.
            }
        }

        if let Some(timeout) = self.timeout {
            if timeout.is_zero() {
                return Err(ConfigValidationError::ZeroDuration {
                    field: "timeout",
                });
            }
        }

        if self.lookahead_depth == 0 {
            return Err(ConfigValidationError::ZeroLimit {
                field: "lookahead_depth",
            });
        }

        if self.lookahead_depth > MAX_LOOKAHEAD_DEPTH {
            return Err(ConfigValidationError::LimitExceeded {
                field: "lookahead_depth",
                value: self.lookahead_depth,
                maximum: MAX_LOOKAHEAD_DEPTH,
            });
        }

        if self.candidate_limit == 0 {
            return Err(ConfigValidationError::ZeroLimit {
                field: "candidate_limit",
            });
        }

        if self.candidate_limit > MAX_CANDIDATE_LIMIT {
            return Err(ConfigValidationError::LimitExceeded {
                field: "candidate_limit",
                value: self.candidate_limit,
                maximum: MAX_CANDIDATE_LIMIT,
            });
        }

        if self.sabre_iterations == 0 {
            return Err(ConfigValidationError::ZeroLimit {
                field: "sabre_iterations",
            });
        }

        if self.sabre_iterations > MAX_SABRE_ITERATIONS {
            return Err(ConfigValidationError::LimitExceeded {
                field: "sabre_iterations",
                value: self.sabre_iterations,
                maximum: MAX_SABRE_ITERATIONS,
            });
        }

        if self.sabre_trials == 0 {
            return Err(ConfigValidationError::ZeroLimit {
                field: "sabre_trials",
            });
        }

        if self.sabre_trials > MAX_SABRE_TRIALS {
            return Err(ConfigValidationError::LimitExceeded {
                field: "sabre_trials",
                value: self.sabre_trials,
                maximum: MAX_SABRE_TRIALS,
            });
        }

        if self.layout_trials == 0 {
            return Err(ConfigValidationError::ZeroLimit {
                field: "layout_trials",
            });
        }

        if self.layout_trials > MAX_LAYOUT_TRIALS {
            return Err(ConfigValidationError::LimitExceeded {
                field: "layout_trials",
                value: self.layout_trials,
                maximum: MAX_LAYOUT_TRIALS,
            });
        }

        Ok(())
    }
}

// =============================================================================
// Routing configuration
// =============================================================================

/// Complete immutable routing policy.
///
/// `RoutingConfig` is the configuration contract consumed by:
///
/// - `layout.rs`;
/// - `algorithms/*`;
/// - `router.rs`;
/// - `verification.rs`;
/// - `transpiler.rs`;
/// - benchmarking;
/// - future routing plugins.
///
/// The struct is intentionally self-contained. It does not hold a topology,
/// circuit, mapping, hardware target, algorithm object, or verifier object.
#[derive(Debug, Clone, PartialEq)]
pub struct RoutingConfig {
    /// Algorithm selection.
    pub algorithm: RoutingAlgorithm,

    /// Initial logical-to-physical layout strategy.
    pub layout: LayoutStrategy,

    /// Primary routing objective.
    pub objective: RoutingObjective,

    /// Global routing execution mode.
    pub mode: RoutingMode,

    /// Output verification level.
    pub verification: VerificationLevel,

    /// Policy for unsupported >2-qubit operations.
    pub multi_qubit_policy: MultiQubitPolicy,

    /// Policy for direction-sensitive gates.
    pub direction_policy: DirectionPolicy,

    /// Search and resource limits.
    pub limits: RoutingLimits,

    /// Weighted objective coefficients.
    pub weights: RoutingWeights,

    /// Optional deterministic seed.
    ///
    /// `None` is permitted for deterministic algorithms that do not require
    /// randomness. For stochastic algorithms, the router may reject a
    /// configuration requesting deterministic execution without a seed.
    pub seed: Option<u64>,

    /// Whether the routing process must be deterministic.
    pub deterministic: bool,

    /// Whether inserted movement operations such as SWAPs may be used.
    pub allow_swap: bool,

    /// Whether bridge-style routing moves may be considered.
    pub allow_bridge: bool,

    /// Whether direction reversal through a later synthesis boundary may be
    /// used.
    ///
    /// This is retained separately from `direction_policy` as a direct
    /// movement/synthesis permission switch.
    pub allow_direction_reversal: bool,

    /// Whether the router should verify the final output.
    ///
    /// Normally this should remain synchronized with `verification`.
    pub verify_output: bool,

    /// Whether mapping state should be checked after every routing mutation.
    pub validate_mapping_after_move: bool,

    /// Whether topology/mapping invariants should be checked before routing.
    pub validate_input: bool,

    /// Whether the router may select unused physical qubits when the hardware
    /// has more physical qubits than logical qubits.
    pub allow_unused_physical_qubits: bool,

    /// Whether the router may leave logical qubits unmapped when they are not
    /// used by the circuit.
    pub allow_unmapped_idle_logical_qubits: bool,
}

impl Default for RoutingConfig {
    fn default() -> Self {
        Self {
            algorithm: RoutingAlgorithm::Auto,
            layout: LayoutStrategy::Auto,
            objective: RoutingObjective::SwapCount,
            mode: RoutingMode::Strict,
            verification: VerificationLevel::Standard,
            multi_qubit_policy: MultiQubitPolicy::NativeOnly,
            direction_policy: DirectionPolicy::Strict,
            limits: RoutingLimits::default(),
            weights: RoutingWeights::default(),
            seed: None,
            deterministic: true,
            allow_swap: true,
            allow_bridge: false,
            allow_direction_reversal: false,
            verify_output: true,
            validate_mapping_after_move: true,
            validate_input: true,
            allow_unused_physical_qubits: true,
            allow_unmapped_idle_logical_qubits: true,
        }
    }
}

impl RoutingConfig {
    /// Creates a production-default configuration.
    pub fn new() -> Self {
        Self::default()
    }

    // -------------------------------------------------------------------------
    // Algorithm
    // -------------------------------------------------------------------------

    /// Sets the routing algorithm.
    pub fn with_algorithm(mut self, algorithm: RoutingAlgorithm) -> Self {
        self.algorithm = algorithm;
        self
    }

    /// Sets the initial-layout strategy.
    pub fn with_layout(mut self, layout: LayoutStrategy) -> Self {
        self.layout = layout;
        self
    }

    /// Sets the routing objective.
    pub fn with_objective(mut self, objective: RoutingObjective) -> Self {
        self.objective = objective;
        self
    }

    /// Sets the routing mode.
    pub fn with_mode(mut self, mode: RoutingMode) -> Self {
        self.mode = mode;
        self
    }

    // -------------------------------------------------------------------------
    // Verification
    // -------------------------------------------------------------------------

    /// Sets the verification level.
    ///
    /// `verify_output` is automatically synchronized.
    pub fn with_verification(
        mut self,
        verification: VerificationLevel,
    ) -> Self {
        self.verification = verification;
        self.verify_output = verification.enabled();
        self
    }

    /// Explicitly enables or disables final verification.
    ///
    /// If disabled while the verification level is non-`None`, the
    /// configuration remains valid but deliberately skips the final pass.
    /// Production callers should normally use `with_verification`.
    pub fn with_verify_output(mut self, enabled: bool) -> Self {
        self.verify_output = enabled;
        self
    }

    // -------------------------------------------------------------------------
    // Operation policies
    // -------------------------------------------------------------------------

    /// Sets the multi-qubit operation policy.
    pub fn with_multi_qubit_policy(
        mut self,
        policy: MultiQubitPolicy,
    ) -> Self {
        self.multi_qubit_policy = policy;
        self
    }

    /// Sets the direction policy.
    pub fn with_direction_policy(
        mut self,
        policy: DirectionPolicy,
    ) -> Self {
        self.direction_policy = policy;
        self
    }

    /// Enables or disables SWAP insertion.
    pub fn with_swap(mut self, enabled: bool) -> Self {
        self.allow_swap = enabled;
        self
    }

    /// Enables or disables bridge routing.
    pub fn with_bridge(mut self, enabled: bool) -> Self {
        self.allow_bridge = enabled;
        self
    }

    /// Enables or disables direction reversal.
    pub fn with_direction_reversal(mut self, enabled: bool) -> Self {
        self.allow_direction_reversal = enabled;
        self
    }

    // -------------------------------------------------------------------------
    // Determinism
    // -------------------------------------------------------------------------

    /// Sets deterministic execution.
    pub fn with_deterministic(mut self, deterministic: bool) -> Self {
        self.deterministic = deterministic;
        self
    }

    /// Sets the algorithm seed.
    pub fn with_seed(mut self, seed: u64) -> Self {
        self.seed = Some(seed);
        self
    }

    /// Removes the explicit seed.
    pub fn without_seed(mut self) -> Self {
        self.seed = None;
        self
    }

    // -------------------------------------------------------------------------
    // Search limits
    // -------------------------------------------------------------------------

    /// Sets the maximum number of routing iterations.
    pub fn with_max_iterations(mut self, value: usize) -> Self {
        self.limits.max_iterations = value;
        self
    }

    /// Sets an explicit maximum number of inserted SWAPs.
    pub fn with_max_swaps(mut self, value: usize) -> Self {
        self.limits.max_swaps = Some(value);
        self
    }

    /// Removes the explicit SWAP limit.
    pub fn without_max_swaps(mut self) -> Self {
        self.limits.max_swaps = None;
        self
    }

    /// Sets a routing timeout.
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.limits.timeout = Some(timeout);
        self
    }

    /// Removes the routing timeout.
    pub fn without_timeout(mut self) -> Self {
        self.limits.timeout = None;
        self
    }

    /// Sets lookahead depth.
    pub fn with_lookahead_depth(mut self, value: usize) -> Self {
        self.limits.lookahead_depth = value;
        self
    }

    /// Sets the candidate limit.
    pub fn with_candidate_limit(mut self, value: usize) -> Self {
        self.limits.candidate_limit = value;
        self
    }

    /// Sets the number of SABRE forward/backward iterations.
    pub fn with_sabre_iterations(mut self, value: usize) -> Self {
        self.limits.sabre_iterations = value;
        self
    }

    /// Sets the number of SABRE routing trials.
    pub fn with_sabre_trials(mut self, value: usize) -> Self {
        self.limits.sabre_trials = value;
        self
    }

    /// Sets the number of independent layout trials.
    pub fn with_layout_trials(mut self, value: usize) -> Self {
        self.limits.layout_trials = value;
        self
    }

    // -------------------------------------------------------------------------
    // Cost configuration
    // -------------------------------------------------------------------------

    /// Sets weighted routing coefficients.
    pub fn with_weights(mut self, weights: RoutingWeights) -> Self {
        self.weights = weights;
        self
    }

    /// Sets all weighted objective coefficients.
    pub fn with_weight_values(
        mut self,
        swap_count: f64,
        depth: f64,
        duration: f64,
        error: f64,
        fidelity: f64,
    ) -> Self {
        self.weights = RoutingWeights::new(
            swap_count,
            depth,
            duration,
            error,
            fidelity,
        );
        self
    }

    // -------------------------------------------------------------------------
    // Validation controls
    // -------------------------------------------------------------------------

    /// Enables/disables input validation.
    pub fn with_input_validation(mut self, enabled: bool) -> Self {
        self.validate_input = enabled;
        self
    }

    /// Enables/disables mapping validation after every move.
    pub fn with_mapping_validation(mut self, enabled: bool) -> Self {
        self.validate_mapping_after_move = enabled;
        self
    }

    /// Controls whether unused physical qubits may remain unassigned.
    pub fn with_unused_physical_qubits(
        mut self,
        allowed: bool,
    ) -> Self {
        self.allow_unused_physical_qubits = allowed;
        self
    }

    /// Controls whether idle logical qubits may remain unmapped.
    pub fn with_unmapped_idle_logical_qubits(
        mut self,
        allowed: bool,
    ) -> Self {
        self.allow_unmapped_idle_logical_qubits = allowed;
        self
    }

    // -------------------------------------------------------------------------
    // Derived policy
    // -------------------------------------------------------------------------

    /// Returns whether this configuration requires a hardware-aware cost model.
    pub const fn requires_hardware_properties(&self) -> bool {
        self.objective.requires_hardware_properties()
            || matches!(
                self.algorithm,
                RoutingAlgorithm::NoiseAware
            )
            || matches!(
                self.layout,
                LayoutStrategy::NoiseAware
            )
    }

    /// Returns whether this configuration requires lookahead state.
    pub const fn requires_lookahead(&self) -> bool {
        self.algorithm.supports_lookahead()
            && self.limits.lookahead_depth > 0
    }

    /// Returns whether the configuration permits speculative routing.
    pub const fn permits_speculative_routing(&self) -> bool {
        matches!(
            self.algorithm,
            RoutingAlgorithm::Lookahead
                | RoutingAlgorithm::Sabre
                | RoutingAlgorithm::NoiseAware
                | RoutingAlgorithm::Dynamic
                | RoutingAlgorithm::Custom(_)
        )
    }

    /// Returns whether the configuration can insert movement operations.
    pub const fn can_insert_moves(&self) -> bool {
        self.allow_swap || self.allow_bridge
    }

    /// Returns whether routing is allowed to return an approximate result.
    pub const fn permits_approximation(&self) -> bool {
        matches!(self.mode, RoutingMode::Approximate)
    }

    /// Returns whether the configuration requires a fixed caller-supplied
    /// layout.
    pub const fn requires_fixed_layout(&self) -> bool {
        matches!(self.layout, LayoutStrategy::Fixed)
    }

    /// Returns whether a caller must provide a deterministic seed for a
    /// stochastic/custom algorithm.
    pub fn requires_seed(&self) -> bool {
        self.deterministic
            && !self.algorithm.has_known_determinism()
    }

    // -------------------------------------------------------------------------
    // Validation
    // -------------------------------------------------------------------------

    /// Validates the complete routing configuration.
    ///
    /// This function performs only configuration-level validation.
    ///
    /// It deliberately does not inspect:
    ///
    /// - a circuit;
    /// - a topology;
    /// - a mapping;
    /// - hardware calibration;
    /// - an algorithm implementation.
    ///
    /// Those validations belong to the consuming subsystem.
    pub fn validate(&self) -> Result<(), ConfigValidationError> {
        self.limits.validate()?;

        if !self.weights.is_valid() {
            return Err(ConfigValidationError::InvalidWeights);
        }

        self.validate_algorithm()?;
        self.validate_objective()?;
        self.validate_layout()?;
        self.validate_operation_policies()?;
        self.validate_determinism()?;
        self.validate_verification()?;

        Ok(())
    }

    fn validate_algorithm(&self) -> Result<(), ConfigValidationError> {
        if let RoutingAlgorithm::Custom(name) = &self.algorithm {
            validate_custom_name(
                name,
                "algorithm",
                MAX_CUSTOM_ALGORITHM_NAME_LENGTH,
            )?;
        }

        if matches!(self.algorithm, RoutingAlgorithm::None)
            && self.allow_swap
        {
            // This is not an error: `None` means the algorithm itself will not
            // route, while the generic configuration can still describe a
            // target that permits SWAPs. The router is responsible for
            // rejecting a circuit that requires them.
        }

        if !self.algorithm.supports_lookahead()
            && self.limits.lookahead_depth != DEFAULT_LOOKAHEAD_DEPTH
            && self.limits.lookahead_depth > 0
        {
            // Deliberately accepted.
            //
            // A future/custom algorithm may consume the field. Configuration
            // must remain forward-compatible rather than rejecting harmless
            // unused values.
        }

        Ok(())
    }

    fn validate_objective(&self) -> Result<(), ConfigValidationError> {
        if let RoutingObjective::Custom(name) = &self.objective {
            validate_custom_name(
                name,
                "objective",
                MAX_CUSTOM_OBJECTIVE_NAME_LENGTH,
            )?;
        }

        if matches!(self.objective, RoutingObjective::Weighted)
            && self.weights.is_zero()
        {
            return Err(ConfigValidationError::ZeroObjectiveWeights);
        }

        Ok(())
    }

    fn validate_layout(&self) -> Result<(), ConfigValidationError> {
        if let LayoutStrategy::Custom(name) = &self.layout {
            validate_custom_name(
                name,
                "layout",
                MAX_CUSTOM_LAYOUT_NAME_LENGTH,
            )?;
        }

        Ok(())
    }

    fn validate_operation_policies(
        &self,
    ) -> Result<(), ConfigValidationError> {
        if self.allow_direction_reversal
            && matches!(
                self.direction_policy,
                DirectionPolicy::Strict
            )
        {
            return Err(
                ConfigValidationError::ConflictingDirectionPolicy,
            );
        }

        if self.allow_bridge
            && !self.allow_swap
            && matches!(
                self.algorithm,
                RoutingAlgorithm::Basic
                    | RoutingAlgorithm::ShortestPath
                    | RoutingAlgorithm::Lookahead
                    | RoutingAlgorithm::Sabre
            )
        {
            return Err(
                ConfigValidationError::BridgeWithoutSupportedMovePolicy,
            );
        }

        Ok(())
    }

    fn validate_determinism(
        &self,
    ) -> Result<(), ConfigValidationError> {
        if self.deterministic
            && self.requires_seed()
            && self.seed.is_none()
        {
            return Err(ConfigValidationError::SeedRequiredForDeterminism);
        }

        Ok(())
    }

    fn validate_verification(
        &self,
    ) -> Result<(), ConfigValidationError> {
        if self.verification.enabled() && !self.verify_output {
            // Explicitly allowed.
            //
            // `verification` describes the available verification level,
            // while `verify_output` is the execution switch. This permits
            // callers to reuse a strict configuration while temporarily
            // disabling the final pass for a controlled benchmark.
        }

        if matches!(self.verification, VerificationLevel::Strict)
            && !self.validate_mapping_after_move
        {
            return Err(
                ConfigValidationError::StrictVerificationRequiresMappingValidation,
            );
        }

        Ok(())
    }
}

// =============================================================================
// Configuration validation errors
// =============================================================================

/// Errors produced when a `RoutingConfig` is internally inconsistent or
/// violates configuration-level safety limits.
///
/// This error type intentionally contains no topology/circuit-specific
/// variants. Those belong to `errors.rs` in the routing subsystem.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfigValidationError {
    /// A required numeric limit is zero.
    ZeroLimit {
        field: &'static str,
    },

    /// A duration was configured as zero.
    ZeroDuration {
        field: &'static str,
    },

    /// A numeric limit exceeds the configuration safety ceiling.
    LimitExceeded {
        field: &'static str,
        value: usize,
        maximum: usize,
    },

    /// One or more objective weights are NaN, infinite, or negative.
    InvalidWeights,

    /// Weighted objective was selected without any active weight.
    ZeroObjectiveWeights,

    /// Direction reversal was enabled while strict direction policy was
    /// selected.
    ConflictingDirectionPolicy,

    /// Bridge-only routing was requested for an algorithm that requires
    /// another movement strategy.
    BridgeWithoutSupportedMovePolicy,

    /// Deterministic execution was requested for an algorithm whose
    /// determinism cannot be guaranteed without an explicit seed.
    SeedRequiredForDeterminism,

    /// Strict verification requires mapping validation after every move.
    StrictVerificationRequiresMappingValidation,

    /// A custom identifier is empty.
    EmptyCustomName {
        field: &'static str,
    },

    /// A custom identifier exceeds its permitted size.
    CustomNameTooLong {
        field: &'static str,
        length: usize,
        maximum: usize,
    },

    /// A custom identifier contains forbidden whitespace.
    InvalidCustomName {
        field: &'static str,
    },
}

impl fmt::Display for ConfigValidationError {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::ZeroLimit { field } => {
                write!(f, "routing configuration: '{field}' must be greater than zero")
            }

            Self::ZeroDuration { field } => {
                write!(f, "routing configuration: '{field}' must be greater than zero")
            }

            Self::LimitExceeded {
                field,
                value,
                maximum,
            } => {
                write!(
                    f,
                    "routing configuration: '{field}' value {value} exceeds maximum {maximum}"
                )
            }

            Self::InvalidWeights => {
                write!(
                    f,
                    "routing configuration: objective weights must be finite and non-negative"
                )
            }

            Self::ZeroObjectiveWeights => {
                write!(
                    f,
                    "routing configuration: weighted objective requires at least one non-zero weight"
                )
            }

            Self::ConflictingDirectionPolicy => {
                write!(
                    f,
                    "routing configuration: direction reversal cannot be enabled with strict direction policy"
                )
            }

            Self::BridgeWithoutSupportedMovePolicy => {
                write!(
                    f,
                    "routing configuration: selected routing algorithm requires SWAP movement support when bridge-only movement is requested"
                )
            }

            Self::SeedRequiredForDeterminism => {
                write!(
                    f,
                    "routing configuration: deterministic execution requires an explicit seed for the selected algorithm"
                )
            }

            Self::StrictVerificationRequiresMappingValidation => {
                write!(
                    f,
                    "routing configuration: strict verification requires mapping validation after every move"
                )
            }

            Self::EmptyCustomName { field } => {
                write!(
                    f,
                    "routing configuration: custom {field} name cannot be empty"
                )
            }

            Self::CustomNameTooLong {
                field,
                length,
                maximum,
            } => {
                write!(
                    f,
                    "routing configuration: custom {field} name length {length} exceeds maximum {maximum}"
                )
            }

            Self::InvalidCustomName { field } => {
                write!(
                    f,
                    "routing configuration: custom {field} name contains invalid whitespace"
                )
            }
        }
    }
}

impl std::error::Error for ConfigValidationError {}

// =============================================================================
// Configuration helpers
// =============================================================================

fn validate_custom_name(
    name: &str,
    field: &'static str,
    maximum: usize,
) -> Result<(), ConfigValidationError> {
    if name.is_empty() {
        return Err(ConfigValidationError::EmptyCustomName { field });
    }

    if name.len() > maximum {
        return Err(ConfigValidationError::CustomNameTooLong {
            field,
            length: name.len(),
            maximum,
        });
    }

    if name.trim() != name || name.chars().any(char::is_whitespace) {
        return Err(ConfigValidationError::InvalidCustomName { field });
    }

    Ok(())
}

// =============================================================================
// Stable configuration fingerprint input
// =============================================================================

/// Stable textual representation of configuration policy.
///
/// This is intentionally not a cryptographic hash.
///
/// `result.rs` or a future reproducibility module may hash this representation
/// using the repository's approved hashing infrastructure.
///
/// Keeping serialization here deterministic avoids requiring `serde` in the
/// routing core contract.
impl RoutingConfig {
    /// Returns a deterministic configuration descriptor.
    ///
    /// The returned string contains configuration policy and search limits,
    /// but deliberately excludes:
    ///
    /// - timestamps;
    /// - process IDs;
    /// - memory addresses;
    /// - topology;
    /// - circuit data;
    /// - hardware calibration.
    ///
    /// Those belong to their respective reproducibility domains.
    pub fn stable_descriptor(&self) -> String {
        let seed = match self.seed {
            Some(value) => value.to_string(),
            None => "none".to_string(),
        };

        let max_swaps = match self.limits.max_swaps {
            Some(value) => value.to_string(),
            None => "none".to_string(),
        };

        let timeout_ns = match self.limits.timeout {
            Some(value) => value.as_nanos().to_string(),
            None => "none".to_string(),
        };

        let algorithm = self.algorithm.to_string();
        let layout = self.layout.to_string();
        let objective = self.objective.to_string();

        format!(
            concat!(
                "algorithm={algorithm};",
                "layout={layout};",
                "objective={objective};",
                "mode={mode};",
                "verification={verification};",
                "multi_qubit={multi_qubit};",
                "direction={direction};",
                "max_iterations={max_iterations};",
                "max_swaps={max_swaps};",
                "timeout_ns={timeout_ns};",
                "lookahead_depth={lookahead_depth};",
                "candidate_limit={candidate_limit};",
                "sabre_iterations={sabre_iterations};",
                "sabre_trials={sabre_trials};",
                "layout_trials={layout_trials};",
                "weight_swap={weight_swap:.17e};",
                "weight_depth={weight_depth:.17e};",
                "weight_duration={weight_duration:.17e};",
                "weight_error={weight_error:.17e};",
                "weight_fidelity={weight_fidelity:.17e};",
                "seed={seed};",
                "deterministic={deterministic};",
                "allow_swap={allow_swap};",
                "allow_bridge={allow_bridge};",
                "allow_direction_reversal={allow_direction_reversal};",
                "verify_output={verify_output};",
                "validate_mapping_after_move={validate_mapping_after_move};",
                "validate_input={validate_input};",
                "allow_unused_physical_qubits={allow_unused_physical_qubits};",
                "allow_unmapped_idle_logical_qubits={allow_unmapped_idle_logical_qubits}"
            ),
            algorithm = algorithm,
            layout = layout,
            objective = objective,
            mode = self.mode,
            verification = self.verification,
            multi_qubit = self.multi_qubit_policy,
            direction = self.direction_policy,
            max_iterations = self.limits.max_iterations,
            max_swaps = max_swaps,
            timeout_ns = timeout_ns,
            lookahead_depth = self.limits.lookahead_depth,
            candidate_limit = self.limits.candidate_limit,
            sabre_iterations = self.limits.sabre_iterations,
            sabre_trials = self.limits.sabre_trials,
            layout_trials = self.limits.layout_trials,
            weight_swap = self.weights.swap_count,
            weight_depth = self.weights.depth,
            weight_duration = self.weights.duration,
            weight_error = self.weights.error,
            weight_fidelity = self.weights.fidelity,
            seed = seed,
            deterministic = self.deterministic,
            allow_swap = self.allow_swap,
            allow_bridge = self.allow_bridge,
            allow_direction_reversal = self.allow_direction_reversal,
            verify_output = self.verify_output,
            validate_mapping_after_move = self.validate_mapping_after_move,
            validate_input = self.validate_input,
            allow_unused_physical_qubits = self.allow_unused_physical_qubits,
            allow_unmapped_idle_logical_qubits =
                self.allow_unmapped_idle_logical_qubits,
        )
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_configuration_is_valid() {
        let config = RoutingConfig::default();

        assert_eq!(
            config.validate(),
            Ok(())
        );

        assert!(config.deterministic);
        assert!(config.verify_output);
        assert_eq!(
            config.verification,
            VerificationLevel::Standard
        );
        assert!(config.allow_swap);
        assert!(!config.allow_bridge);
    }

    #[test]
    fn default_weights_are_valid() {
        let weights = RoutingWeights::default();

        assert!(weights.is_valid());
        assert_eq!(weights.sum(), Some(1.0));
        assert!(!weights.is_zero());
    }

    #[test]
    fn negative_weight_is_rejected() {
        let config = RoutingConfig::default().with_weight_values(
            -1.0,
            0.0,
            0.0,
            0.0,
            0.0,
        );

        assert_eq!(
            config.validate(),
            Err(ConfigValidationError::InvalidWeights)
        );
    }

    #[test]
    fn nan_weight_is_rejected() {
        let config = RoutingConfig::default().with_weight_values(
            f64::NAN,
            0.0,
            0.0,
            0.0,
            0.0,
        );

        assert_eq!(
            config.validate(),
            Err(ConfigValidationError::InvalidWeights)
        );
    }

    #[test]
    fn infinite_weight_is_rejected() {
        let config = RoutingConfig::default().with_weight_values(
            f64::INFINITY,
            0.0,
            0.0,
            0.0,
            0.0,
        );

        assert_eq!(
            config.validate(),
            Err(ConfigValidationError::InvalidWeights)
        );
    }

    #[test]
    fn weighted_objective_requires_nonzero_weights() {
        let config = RoutingConfig::default()
            .with_objective(RoutingObjective::Weighted)
            .with_weights(RoutingWeights::new(
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
            ));

        assert_eq!(
            config.validate(),
            Err(ConfigValidationError::ZeroObjectiveWeights)
        );
    }

    #[test]
    fn weighted_objective_accepts_valid_weights() {
        let config = RoutingConfig::default()
            .with_objective(RoutingObjective::Weighted)
            .with_weights(RoutingWeights::new(
                1.0,
                0.5,
                0.2,
                0.1,
                0.3,
            ));

        assert!(config.validate().is_ok());
    }

    #[test]
    fn zero_max_swaps_is_valid() {
        let config = RoutingConfig::default()
            .with_max_swaps(0);

        assert!(config.validate().is_ok());
        assert_eq!(config.limits.max_swaps, Some(0));
    }

    #[test]
    fn zero_iterations_are_rejected() {
        let config = RoutingConfig::default()
            .with_max_iterations(0);

        assert_eq!(
            config.validate(),
            Err(ConfigValidationError::ZeroLimit {
                field: "max_iterations"
            })
        );
    }

    #[test]
    fn excessive_iterations_are_rejected() {
        let config = RoutingConfig::default()
            .with_max_iterations(MAX_ROUTING_ITERATIONS + 1);

        assert_eq!(
            config.validate(),
            Err(ConfigValidationError::LimitExceeded {
                field: "max_iterations",
                value: MAX_ROUTING_ITERATIONS + 1,
                maximum: MAX_ROUTING_ITERATIONS,
            })
        );
    }

    #[test]
    fn zero_lookahead_is_rejected() {
        let config = RoutingConfig::default()
            .with_lookahead_depth(0);

        assert_eq!(
            config.validate(),
            Err(ConfigValidationError::ZeroLimit {
                field: "lookahead_depth"
            })
        );
    }

    #[test]
    fn excessive_lookahead_is_rejected() {
        let value = MAX_LOOKAHEAD_DEPTH + 1;

        let config = RoutingConfig::default()
            .with_lookahead_depth(value);

        assert_eq!(
            config.validate(),
            Err(ConfigValidationError::LimitExceeded {
                field: "lookahead_depth",
                value,
                maximum: MAX_LOOKAHEAD_DEPTH,
            })
        );
    }

    #[test]
    fn zero_candidate_limit_is_rejected() {
        let config = RoutingConfig::default()
            .with_candidate_limit(0);

        assert_eq!(
            config.validate(),
            Err(ConfigValidationError::ZeroLimit {
                field: "candidate_limit"
            })
        );
    }

    #[test]
    fn timeout_must_be_nonzero() {
        let config = RoutingConfig::default()
            .with_timeout(Duration::ZERO);

        assert_eq!(
            config.validate(),
            Err(ConfigValidationError::ZeroDuration {
                field: "timeout"
            })
        );
    }

    #[test]
    fn custom_algorithm_name_must_be_valid() {
        let config = RoutingConfig::default()
            .with_algorithm(
                RoutingAlgorithm::Custom(
                    "sabre custom".to_string()
                ),
            );

        assert_eq!(
            config.validate(),
            Err(ConfigValidationError::InvalidCustomName {
                field: "algorithm"
            })
        );
    }

    #[test]
    fn custom_algorithm_name_cannot_be_empty() {
        let config = RoutingConfig::default()
            .with_algorithm(
                RoutingAlgorithm::Custom(String::new())
            );

        assert_eq!(
            config.validate(),
            Err(ConfigValidationError::EmptyCustomName {
                field: "algorithm"
            })
        );
    }

    #[test]
    fn custom_layout_name_is_validated() {
        let config = RoutingConfig::default()
            .with_layout(
                LayoutStrategy::Custom(
                    "my_layout".to_string()
                ),
            );

        assert!(config.validate().is_ok());
    }

    #[test]
    fn custom_objective_name_is_validated() {
        let config = RoutingConfig::default()
            .with_objective(
                RoutingObjective::Custom(
                    "my_objective".to_string()
                ),
            );

        assert!(config.validate().is_ok());
    }

    #[test]
    fn strict_direction_policy_rejects_explicit_reversal() {
        let config = RoutingConfig::default()
            .with_direction_reversal(true);

        assert_eq!(
            config.validate(),
            Err(ConfigValidationError::ConflictingDirectionPolicy)
        );
    }

    #[test]
    fn reversal_is_valid_with_allow_reversal_policy() {
        let config = RoutingConfig::default()
            .with_direction_policy(
                DirectionPolicy::AllowReversal,
            )
            .with_direction_reversal(true);

        assert!(config.validate().is_ok());
    }

    #[test]
    fn strict_verification_requires_mapping_validation() {
        let config = RoutingConfig::default()
            .with_verification(VerificationLevel::Strict)
            .with_mapping_validation(false);

        assert_eq!(
            config.validate(),
            Err(
                ConfigValidationError::
                    StrictVerificationRequiresMappingValidation
            )
        );
    }

    #[test]
    fn verification_level_synchronizes_verify_output() {
        let config = RoutingConfig::default()
            .with_verification(VerificationLevel::None);

        assert!(!config.verify_output);

        let config = RoutingConfig::default()
            .with_verification(VerificationLevel::Strict);

        assert!(config.verify_output);
    }

    #[test]
    fn seed_is_retained() {
        let config = RoutingConfig::default()
            .with_seed(42);

        assert_eq!(config.seed, Some(42));
    }

    #[test]
    fn seed_can_be_removed() {
        let config = RoutingConfig::default()
            .with_seed(42)
            .without_seed();

        assert_eq!(config.seed, None);
    }

    #[test]
    fn sabre_configuration_is_exposed() {
        let config = RoutingConfig::default()
            .with_algorithm(RoutingAlgorithm::Sabre)
            .with_sabre_iterations(8)
            .with_sabre_trials(20)
            .with_layout_trials(10)
            .with_seed(1234);

        assert!(config.validate().is_ok());
        assert_eq!(config.limits.sabre_iterations, 8);
        assert_eq!(config.limits.sabre_trials, 20);
        assert_eq!(config.limits.layout_trials, 10);
        assert_eq!(config.seed, Some(1234));
    }

    #[test]
    fn sabre_requires_lookahead() {
        let config = RoutingConfig::default()
            .with_algorithm(RoutingAlgorithm::Sabre);

        assert!(config.requires_lookahead());
    }

    #[test]
    fn noise_aware_requires_hardware_properties() {
        let config = RoutingConfig::default()
            .with_algorithm(RoutingAlgorithm::NoiseAware);

        assert!(config.requires_hardware_properties());
    }

    #[test]
    fn duration_objective_requires_hardware_properties() {
        let config = RoutingConfig::default()
            .with_objective(RoutingObjective::Duration);

        assert!(config.requires_hardware_properties());
    }

    #[test]
    fn fixed_layout_requires_external_mapping() {
        let config = RoutingConfig::default()
            .with_layout(LayoutStrategy::Fixed);

        assert!(config.requires_fixed_layout());
    }

    #[test]
    fn stable_descriptor_is_deterministic() {
        let first = RoutingConfig::default()
            .with_algorithm(RoutingAlgorithm::Sabre)
            .with_seed(42)
            .stable_descriptor();

        let second = RoutingConfig::default()
            .with_algorithm(RoutingAlgorithm::Sabre)
            .with_seed(42)
            .stable_descriptor();

        assert_eq!(first, second);
    }

    #[test]
    fn different_seed_changes_descriptor() {
        let first = RoutingConfig::default()
            .with_seed(1)
            .stable_descriptor();

        let second = RoutingConfig::default()
            .with_seed(2)
            .stable_descriptor();

        assert_ne!(first, second);
    }

    #[test]
    fn routing_algorithm_names_are_stable() {
        assert_eq!(RoutingAlgorithm::Auto.name(), "auto");
        assert_eq!(RoutingAlgorithm::None.name(), "none");
        assert_eq!(RoutingAlgorithm::Basic.name(), "basic");
        assert_eq!(
            RoutingAlgorithm::ShortestPath.name(),
            "shortest_path"
        );
        assert_eq!(
            RoutingAlgorithm::Lookahead.name(),
            "lookahead"
        );
        assert_eq!(RoutingAlgorithm::Sabre.name(), "sabre");
        assert_eq!(
            RoutingAlgorithm::NoiseAware.name(),
            "noise_aware"
        );
        assert_eq!(
            RoutingAlgorithm::Dynamic.name(),
            "dynamic"
        );
    }

    #[test]
    fn layout_names_are_stable() {
        assert_eq!(LayoutStrategy::Auto.name(), "auto");
        assert_eq!(LayoutStrategy::Trivial.name(), "trivial");
        assert_eq!(LayoutStrategy::Dense.name(), "dense");
        assert_eq!(
            LayoutStrategy::InteractionGraph.name(),
            "interaction_graph"
        );
        assert_eq!(
            LayoutStrategy::NoiseAware.name(),
            "noise_aware"
        );
        assert_eq!(LayoutStrategy::Sabre.name(), "sabre");
        assert_eq!(LayoutStrategy::Fixed.name(), "fixed");
    }

    #[test]
    fn objective_names_are_stable() {
        assert_eq!(
            RoutingObjective::SwapCount.name(),
            "swap_count"
        );
        assert_eq!(RoutingObjective::Depth.name(), "depth");
        assert_eq!(
            RoutingObjective::Duration.name(),
            "duration"
        );
        assert_eq!(RoutingObjective::Error.name(), "error");
        assert_eq!(
            RoutingObjective::Fidelity.name(),
            "fidelity"
        );
        assert_eq!(
            RoutingObjective::Weighted.name(),
            "weighted"
        );
        assert_eq!(
            RoutingObjective::Lexicographic.name(),
            "lexicographic"
        );
    }

    #[test]
    fn all_default_limits_are_valid() {
        assert!(RoutingLimits::default().validate().is_ok());
    }

    #[test]
    fn approximation_mode_is_detected() {
        let config = RoutingConfig::default()
            .with_mode(RoutingMode::Approximate);

        assert!(config.permits_approximation());
    }

    #[test]
    fn strict_mode_is_not_approximate() {
        let config = RoutingConfig::default();

        assert!(!config.permits_approximation());
    }

    #[test]
    fn move_policy_is_detected() {
        let config = RoutingConfig::default();

        assert!(config.can_insert_moves());
    }

    #[test]
    fn move_policy_can_disable_all_moves() {
        let config = RoutingConfig::default()
            .with_swap(false)
            .with_bridge(false);

        assert!(!config.can_insert_moves());
    }

    #[test]
    fn default_configuration_is_deterministic() {
        let config = RoutingConfig::default();

        assert!(config.deterministic);
        assert!(config.algorithm.has_known_determinism());
    }

    #[test]
    fn none_algorithm_is_valid() {
        let config = RoutingConfig::default()
            .with_algorithm(RoutingAlgorithm::None);

        assert!(config.validate().is_ok());
    }

    #[test]
    fn fixed_layout_is_valid_without_embedding_mapping() {
        let config = RoutingConfig::default()
            .with_layout(LayoutStrategy::Fixed);

        assert!(config.validate().is_ok());
    }

    #[test]
    fn native_only_is_default_multi_qubit_policy() {
        assert_eq!(
            MultiQubitPolicy::default(),
            MultiQubitPolicy::NativeOnly
        );
    }

    #[test]
    fn strict_direction_is_default() {
        assert_eq!(
            DirectionPolicy::default(),
            DirectionPolicy::Strict
        );
    }

    #[test]
    fn standard_verification_is_default() {
        assert_eq!(
            VerificationLevel::default(),
            VerificationLevel::Standard
        );
    }
}