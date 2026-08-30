//! Zamani Quantum Optimization — Configuration
//!
//! Production configuration contract for the logical quantum-circuit
//! optimizer.
//!
//! # Architectural position
//!
//! This module is intentionally independent from optimization algorithms.
//! It defines *policy* and *configuration*, not transformation logic.
//!
//! The intended dependency direction is:
//!
//! ```text
//!                         quantum::ir
//!                              │
//!                              ▼
//!                    optimization::config
//!                              │
//!          ┌───────────────────┼───────────────────┐
//!          ▼                   ▼                   ▼
//!       pipeline             passes              planner
//!          │                   │                   │
//!          └───────────────────┼───────────────────┘
//!                              ▼
//!                         optimized IR
//! ```
//!
//! `config.rs` must not depend on:
//!
//! - individual optimization passes;
//! - the optimization pipeline;
//! - rewrite engines;
//! - e-graphs;
//! - routing;
//! - scheduling;
//! - hardware backends;
//! - benchmark execution;
//! - frontend parsing;
//! - QPU APIs.
//!
//! This keeps the configuration contract stable while the optimizer evolves.
//!
//! # Canonical IR
//!
//! Optimization operates on `quantum::ir`. This module therefore does not
//! define `QuantumGate`, `QuantumOperation`, or another circuit representation.
//!
//! The canonical representation is owned by:
//!
//! `crate::quantum::ir`
//!
//! # Integration contract
//!
//! Later optimization modules should consume this file as follows:
//!
//! - `limits.rs` may re-export `OptimizationLimits`;
//! - `profile.rs` may re-export or resolve `OptimizationProfile`;
//! - `target.rs` resolves `TargetSelection` into a concrete target;
//! - `pass.rs` consumes pass-selection and policy fields;
//! - `pipeline.rs` consumes the complete `OptimizationConfig`;
//! - `planner.rs` uses the profile, level, objective and target;
//! - `context.rs` stores an immutable copy of the configuration for one run;
//! - `verification.rs` consumes `VerificationConfig`;
//! - `statistics.rs` reports the policy actually used;
//! - `provenance.rs` records the configuration fingerprint/serialized form;
//! - `serialization/config.rs` can serialize this public configuration
//!   directly.
//!
//! No future module should require this file to be modified merely because a
//! new optimization pass is added.
//!
//! # Safety
//!
//! This module uses safe Rust only.
//!
//! No `unsafe` code is required or permitted.
//!
//! # Rust compatibility
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021.
//!
//! No nightly features are used.

// =============================================================================
// Imports
// =============================================================================

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::quantum::ir::QuantumIrLimits;

// =============================================================================
// Configuration errors
// =============================================================================

/// Result type used when validating optimization configuration.
pub type OptimizationConfigResult<T> = Result<T, OptimizationConfigError>;

/// Errors produced by invalid optimizer configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OptimizationConfigError {
    /// A textual identifier is empty.
    EmptyIdentifier {
        /// Name of the invalid field.
        field: &'static str,
    },

    /// A textual identifier contains unsupported characters.
    InvalidIdentifier {
        /// Name of the invalid field.
        field: &'static str,

        /// Invalid identifier.
        value: String,
    },

    /// A numeric field must be greater than zero.
    ZeroValue {
        /// Name of the invalid field.
        field: &'static str,
    },

    /// A numeric field is outside its permitted range.
    OutOfRange {
        /// Name of the invalid field.
        field: &'static str,

        /// Supplied value.
        value: String,

        /// Human-readable constraint.
        constraint: &'static str,
    },

    /// A floating-point value is not finite.
    NonFiniteFloat {
        /// Name of the invalid field.
        field: &'static str,
    },

    /// A floating-point value is negative where that is prohibited.
    NegativeFloat {
        /// Name of the invalid field.
        field: &'static str,
    },

    /// A configuration combination is semantically invalid.
    ConflictingOptions {
        /// First conflicting option.
        first: &'static str,

        /// Second conflicting option.
        second: &'static str,
    },

    /// An IR resource policy is invalid.
    InvalidIrLimits {
        /// Underlying error rendered as stable text.
        message: String,
    },
}

impl fmt::Display for OptimizationConfigError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyIdentifier { field } => {
                write!(formatter, "optimization configuration field `{field}` must not be empty")
            }

            Self::InvalidIdentifier { field, value } => {
                write!(
                    formatter,
                    "optimization configuration field `{field}` contains invalid identifier `{value}`"
                )
            }

            Self::ZeroValue { field } => {
                write!(
                    formatter,
                    "optimization configuration field `{field}` must be greater than zero"
                )
            }

            Self::OutOfRange {
                field,
                value,
                constraint,
            } => {
                write!(
                    formatter,
                    "optimization configuration field `{field}` has invalid value {value}: {constraint}"
                )
            }

            Self::NonFiniteFloat { field } => {
                write!(
                    formatter,
                    "optimization configuration field `{field}` must be finite"
                )
            }

            Self::NegativeFloat { field } => {
                write!(
                    formatter,
                    "optimization configuration field `{field}` must not be negative"
                )
            }

            Self::ConflictingOptions { first, second } => {
                write!(
                    formatter,
                    "optimization configuration options `{first}` and `{second}` conflict"
                )
            }

            Self::InvalidIrLimits { message } => {
                write!(
                    formatter,
                    "invalid Quantum IR limits used by optimization: {message}"
                )
            }
        }
    }
}

impl std::error::Error for OptimizationConfigError {}

// =============================================================================
// Optimization level
// =============================================================================

/// Coarse optimization intensity.
///
/// Optimization levels describe the amount of compiler effort requested, not
/// a fixed list of passes. The planner is responsible for resolving a level
/// into an appropriate pipeline.
///
/// This distinction is important because the same level may legitimately map
/// to different passes for different target architectures.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum OptimizationLevel {
    /// Validate and normalize without performing substantive rewrites.
    O0,

    /// Cheap local simplification.
    O1,

    /// Standard production optimization.
    O2,

    /// Aggressive logical optimization.
    O3,

    /// Fault-tolerant resource optimization.
    Of,

    /// Prefer reduced circuit size/gate count.
    Os,

    /// Prefer reduced circuit depth.
    Od,

    /// Prefer reduced two-qubit operation count.
    Ot,
}

impl OptimizationLevel {
    /// Returns the conventional numeric optimization level where one exists.
    ///
    /// Fault-tolerant and objective-specific levels return `None` because they
    /// are semantic policies rather than numeric intensity levels.
    pub const fn numeric(self) -> Option<u8> {
        match self {
            Self::O0 => Some(0),
            Self::O1 => Some(1),
            Self::O2 => Some(2),
            Self::O3 => Some(3),
            Self::Of | Self::Os | Self::Od | Self::Ot => None,
        }
    }

    /// Returns whether this level may enable aggressive/global optimization.
    pub const fn is_aggressive(self) -> bool {
        matches!(
            self,
            Self::O3 | Self::Of | Self::Os | Self::Od | Self::Ot
        )
    }

    /// Returns whether this level is intended for fault-tolerant optimization.
    pub const fn is_fault_tolerant(self) -> bool {
        matches!(self, Self::Of)
    }

    /// Returns the canonical textual identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::O0 => "o0",
            Self::O1 => "o1",
            Self::O2 => "o2",
            Self::O3 => "o3",
            Self::Of => "of",
            Self::Os => "os",
            Self::Od => "od",
            Self::Ot => "ot",
        }
    }
}

impl Default for OptimizationLevel {
    fn default() -> Self {
        Self::O2
    }
}

// =============================================================================
// Optimization profile
// =============================================================================

/// High-level optimization profile.
///
/// Profiles describe compiler intent. They are resolved by `profile.rs` and
/// `planner.rs`; they do not directly contain implementation-specific passes.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum OptimizationProfile {
    /// General-purpose production optimization.
    Generic,

    /// Minimize compilation work while applying safe simplifications.
    FastCompile,

    /// General production balance between compile time and circuit quality.
    Balanced,

    /// Spend substantially more compiler effort searching for improvements.
    Aggressive,

    /// Prefer minimum logical depth.
    MinimumDepth,

    /// Prefer minimum total operation/gate count.
    MinimumGateCount,

    /// Prefer minimum two-qubit operation count.
    MinimumTwoQubit,

    /// Optimize logical fault-tolerant resource consumption.
    FaultTolerant,

    /// Prefer optimization characteristics useful for simulation.
    Simulation,

    /// Diagnostic/development profile.
    Debug,

    /// Require verification-oriented compilation.
    Verified,
}

impl Default for OptimizationProfile {
    fn default() -> Self {
        Self::Balanced
    }
}

impl OptimizationProfile {
    /// Returns the canonical profile identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Generic => "generic",
            Self::FastCompile => "fast_compile",
            Self::Balanced => "balanced",
            Self::Aggressive => "aggressive",
            Self::MinimumDepth => "minimum_depth",
            Self::MinimumGateCount => "minimum_gate_count",
            Self::MinimumTwoQubit => "minimum_two_qubit",
            Self::FaultTolerant => "fault_tolerant",
            Self::Simulation => "simulation",
            Self::Debug => "debug",
            Self::Verified => "verified",
        }
    }

    /// Returns whether the profile requests fault-tolerant optimization.
    pub const fn is_fault_tolerant(self) -> bool {
        matches!(self, Self::FaultTolerant)
    }

    /// Returns whether the profile is verification-oriented.
    pub const fn is_verification_oriented(self) -> bool {
        matches!(self, Self::Verified)
    }
}

// =============================================================================
// Optimization objective
// =============================================================================

/// Primary optimization objective.
///
/// An objective is deliberately independent from a concrete cost model. The
/// later `cost.rs` module maps these objectives onto target-specific costs.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum OptimizationObjective {
    /// Minimize total operations/gates.
    MinimizeGateCount,

    /// Minimize circuit depth.
    MinimizeDepth,

    /// Minimize two-qubit operations.
    MinimizeTwoQubitGates,

    /// Minimize two-qubit depth.
    MinimizeTwoQubitDepth,

    /// Minimize T gates.
    MinimizeTCount,

    /// Minimize T depth.
    MinimizeTDepth,

    /// Minimize measurement operations.
    MinimizeMeasurements,

    /// Minimize estimated execution duration.
    MinimizeDuration,

    /// Minimize estimated accumulated error.
    MinimizeError,

    /// Minimize logical qubit/ancilla demand.
    MinimizeWidth,

    /// Use a target-defined balanced objective.
    Balanced,

    /// Use a lexicographic objective selected by the planner/profile.
    Lexicographic,

    /// Preserve the original circuit's cost as closely as possible.
    PreserveCost,
}

impl Default for OptimizationObjective {
    fn default() -> Self {
        Self::Balanced
    }

    /// Returns a stable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MinimizeGateCount => "minimize_gate_count",
            Self::MinimizeDepth => "minimize_depth",
            Self::MinimizeTwoQubitGates => "minimize_two_qubit_gates",
            Self::MinimizeTwoQubitDepth => "minimize_two_qubit_depth",
            Self::MinimizeTCount => "minimize_t_count",
            Self::MinimizeTDepth => "minimize_t_depth",
            Self::MinimizeMeasurements => "minimize_measurements",
            Self::MinimizeDuration => "minimize_duration",
            Self::MinimizeError => "minimize_error",
            Self::MinimizeWidth => "minimize_width",
            Self::Balanced => "balanced",
            Self::Lexicographic => "lexicographic",
            Self::PreserveCost => "preserve_cost",
        }
    }
}

// =============================================================================
// Target selection
// =============================================================================

/// Selection of the optimization target.
///
/// The concrete target definition belongs to `targets/target.rs`.
///
/// Keeping the selector here as an identifier means configuration can be
/// constructed before a concrete target implementation exists.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TargetSelection {
    /// Let the planner choose a generic target based on the circuit.
    Auto,

    /// Hardware-independent logical optimization.
    Generic,

    /// Named target resolved by `targets/target.rs`.
    Named(String),
}

impl Default for TargetSelection {
    fn default() -> Self {
        Self::Auto
    }
}

impl TargetSelection {
    /// Creates a named target selection.
    pub fn named(value: impl Into<String>) -> Self {
        Self::Named(value.into())
    }

    /// Returns the target identifier if this is a named target.
    pub fn name(&self) -> Option<&str> {
        match self {
            Self::Named(value) => Some(value.as_str()),
            Self::Auto | Self::Generic => None,
        }
    }

    /// Validates the target selector.
    pub fn validate(&self) -> OptimizationConfigResult<()> {
        if let Self::Named(value) = self {
            validate_identifier("target", value)?;
        }

        Ok(())
    }
}

// =============================================================================
// Determinism
// =============================================================================

/// Determinism policy for optimization.
///
/// Deterministic operation is the default because compiler output should be
/// reproducible. Randomized algorithms must never silently use an uncontrolled
/// random seed.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum Determinism {
    /// No randomized optimization is permitted.
    Deterministic,

    /// Randomized optimization is permitted but uses the supplied seed.
    Seeded(u64),

    /// Randomized optimization may use an implementation-provided source of
    /// randomness.
    ///
    /// This mode is intentionally explicit and is not the default.
    Nondeterministic,
}

impl Default for Determinism {
    fn default() -> Self {
        Self::Deterministic
    }
}

impl Determinism {
    /// Returns true if reproducible output is guaranteed by policy.
    pub const fn is_reproducible(self) -> bool {
        matches!(self, Self::Deterministic | Self::Seeded(_))
    }

    /// Returns the configured seed, when one exists.
    pub const fn seed(self) -> Option<u64> {
        match self {
            Self::Seeded(seed) => Some(seed),
            Self::Deterministic | Self::Nondeterministic => None,
        }
    }
}

// =============================================================================
// Parallelism
// =============================================================================

/// Parallel optimization policy.
///
/// Optimization passes remain responsible for determining whether their own
/// work is actually parallelizable.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum Parallelism {
    /// Never parallelize optimizer work.
    SingleThreaded,

    /// Allow the optimizer to choose a safe level of parallelism.
    Auto,

    /// Request a fixed number of worker threads.
    Fixed(usize),
}

impl Default for Parallelism {
    fn default() -> Self {
        Self::Auto
    }
}

impl Parallelism {
    /// Validates the parallelism policy.
    pub fn validate(self) -> OptimizationConfigResult<()> {
        if let Self::Fixed(threads) = self {
            if threads == 0 {
                return Err(OptimizationConfigError::ZeroValue {
                    field: "parallelism.fixed",
                });
            }
        }

        Ok(())
    }

    /// Returns the requested worker count.
    pub const fn worker_count(self) -> Option<usize> {
        match self {
            Self::SingleThreaded => Some(1),
            Self::Auto => None,
            Self::Fixed(value) => Some(value),
        }
    }
}

// =============================================================================
// Rewrite policy
// =============================================================================

/// Policy for rewrite application.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum RewritePolicy {
    /// Only apply rewrites whose cost is provably non-increasing.
    Conservative,

    /// Permit normal cost-improving and neutral rewrites.
    Balanced,

    /// Permit bounded exploratory rewrites.
    Aggressive,

    /// Require explicit semantic verification of rewrite results.
    Verified,
}

impl Default for RewritePolicy {
    fn default() -> Self {
        Self::Balanced
    }
}

// =============================================================================
// Fixed-point policy
// =============================================================================

/// Policy controlling repeated optimization passes.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum FixedPointPolicy {
    /// Run each selected pass once.
    Once,

    /// Repeat until no change occurs or the iteration budget is reached.
    UntilStable,

    /// Repeat only when the planner determines that another iteration is
    /// potentially beneficial.
    Adaptive,
}

impl Default for FixedPointPolicy {
    fn default() -> Self {
        Self::UntilStable
    }
}

// =============================================================================
// Verification mode
// =============================================================================

/// Semantic verification mode.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum VerificationMode {
    /// Perform no semantic equivalence checking.
    ///
    /// Structural validation is still expected at the pipeline boundary.
    None,

    /// Validate only structural IR invariants.
    Structural,

    /// Use exact equivalence where feasible.
    ExactWhenFeasible,

    /// Use probabilistic/differential verification.
    Probabilistic,

    /// Exhaustively verify small circuits and fail if they exceed the
    /// configured exhaustive limits.
    ExhaustiveSmall,

    /// Require verification of every accepted transformation.
    EveryRewrite,
}

impl Default for VerificationMode {
    fn default() -> Self {
        Self::ExactWhenFeasible
    }
}

// =============================================================================
// Verification configuration
// =============================================================================

/// Verification-specific configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct VerificationConfig {
    /// Verification strategy.
    pub mode: VerificationMode,

    /// Maximum number of qubits for exact/exhaustive verification.
    pub max_qubits: usize,

    /// Maximum number of operations for exact/exhaustive verification.
    pub max_operations: usize,

    /// Number of randomized trials for probabilistic verification.
    pub randomized_trials: usize,

    /// Numerical tolerance used by approximate verification.
    pub tolerance: f64,

    /// Whether verification failure is fatal.
    pub fail_on_error: bool,

    /// Whether verification metadata should be included in provenance.
    pub record_results: bool,
}

impl Default for VerificationConfig {
    fn default() -> Self {
        Self {
            mode: VerificationMode::ExactWhenFeasible,
            max_qubits: 16,
            max_operations: 256,
            randomized_trials: 128,
            tolerance: 1.0e-10,
            fail_on_error: true,
            record_results: true,
        }
    }
}

impl VerificationConfig {
    /// Validates verification settings.
    pub fn validate(&self) -> OptimizationConfigResult<()> {
        if self.max_qubits == 0 {
            return Err(OptimizationConfigError::ZeroValue {
                field: "verification.max_qubits",
            });
        }

        if self.max_operations == 0 {
            return Err(OptimizationConfigError::ZeroValue {
                field: "verification.max_operations",
            });
        }

        if self.randomized_trials == 0
            && matches!(self.mode, VerificationMode::Probabilistic)
        {
            return Err(OptimizationConfigError::ZeroValue {
                field: "verification.randomized_trials",
            });
        }

        if !self.tolerance.is_finite() {
            return Err(OptimizationConfigError::NonFiniteFloat {
                field: "verification.tolerance",
            });
        }

        if self.tolerance < 0.0 {
            return Err(OptimizationConfigError::NegativeFloat {
                field: "verification.tolerance",
            });
        }

        Ok(())
    }
}

// =============================================================================
// Optimization resource limits
// =============================================================================

/// Hard resource/work limits for one optimization invocation.
///
/// These are optimizer-specific limits. They intentionally do not replace
/// `quantum::ir::QuantumIrLimits`.
///
/// `QuantumIrLimits` protects the canonical IR itself. `OptimizationLimits`
/// protects potentially expensive transformation and search work.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(default)]
pub struct OptimizationLimits {
    /// Maximum number of passes that may execute.
    pub max_passes: usize,

    /// Maximum fixed-point iterations for one pass.
    pub max_iterations_per_pass: usize,

    /// Maximum total rewrite applications.
    pub max_total_rewrites: usize,

    /// Maximum operations accepted as optimizer input.
    pub max_circuit_operations: usize,

    /// Maximum qubits accepted as optimizer input.
    pub max_circuit_qubits: usize,

    /// Maximum e-graph nodes.
    pub max_egraph_nodes: usize,

    /// Maximum e-graph equivalence classes.
    pub max_egraph_classes: usize,

    /// Maximum synthesis depth requested by one synthesis operation.
    pub max_synthesis_depth: usize,

    /// Maximum operations considered by semantic verification.
    pub max_verification_operations: usize,

    /// Maximum qubits considered by semantic verification.
    pub max_verification_qubits: usize,

    /// Maximum wall-clock optimization time in milliseconds.
    ///
    /// `None` is intentionally not supported. Production optimization always
    /// has an explicit upper bound.
    pub max_runtime_ms: u64,
}

impl Default for OptimizationLimits {
    fn default() -> Self {
        Self {
            max_passes: 256,
            max_iterations_per_pass: 64,
            max_total_rewrites: 10_000_000,
            max_circuit_operations: 1_000_000,
            max_circuit_qubits: 4096,
            max_egraph_nodes: 1_000_000,
            max_egraph_classes: 250_000,
            max_synthesis_depth: 256,
            max_verification_operations: 4096,
            max_verification_qubits: 16,
            max_runtime_ms: 300_000,
        }
    }
}

impl OptimizationLimits {
    /// Validates all optimizer resource limits.
    pub const fn validate(&self) -> OptimizationConfigResult<()> {
        if self.max_passes == 0 {
            return Err(OptimizationConfigError::ZeroValue {
                field: "limits.max_passes",
            });
        }

        if self.max_iterations_per_pass == 0 {
            return Err(OptimizationConfigError::ZeroValue {
                field: "limits.max_iterations_per_pass",
            });
        }

        if self.max_total_rewrites == 0 {
            return Err(OptimizationConfigError::ZeroValue {
                field: "limits.max_total_rewrites",
            });
        }

        if self.max_circuit_operations == 0 {
            return Err(OptimizationConfigError::ZeroValue {
                field: "limits.max_circuit_operations",
            });
        }

        if self.max_circuit_qubits == 0 {
            return Err(OptimizationConfigError::ZeroValue {
                field: "limits.max_circuit_qubits",
            });
        }

        if self.max_egraph_nodes == 0 {
            return Err(OptimizationConfigError::ZeroValue {
                field: "limits.max_egraph_nodes",
            });
        }

        if self.max_egraph_classes == 0 {
            return Err(OptimizationConfigError::ZeroValue {
                field: "limits.max_egraph_classes",
            });
        }

        if self.max_synthesis_depth == 0 {
            return Err(OptimizationConfigError::ZeroValue {
                field: "limits.max_synthesis_depth",
            });
        }

        if self.max_verification_operations == 0 {
            return Err(OptimizationConfigError::ZeroValue {
                field: "limits.max_verification_operations",
            });
        }

        if self.max_verification_qubits == 0 {
            return Err(OptimizationConfigError::ZeroValue {
                field: "limits.max_verification_qubits",
            });
        }

        if self.max_runtime_ms == 0 {
            return Err(OptimizationConfigError::ZeroValue {
                field: "limits.max_runtime_ms",
            });
        }

        Ok(())
    }

    /// Creates a conservative limit policy suitable for untrusted input.
    pub const fn conservative() -> Self {
        Self {
            max_passes: 64,
            max_iterations_per_pass: 16,
            max_total_rewrites: 100_000,
            max_circuit_operations: 100_000,
            max_circuit_qubits: 1024,
            max_egraph_nodes: 100_000,
            max_egraph_classes: 25_000,
            max_synthesis_depth: 64,
            max_verification_operations: 1024,
            max_verification_qubits: 12,
            max_runtime_ms: 30_000,
        }
    }

    /// Creates a development policy that permits larger optimization jobs.
    pub const fn development() -> Self {
        Self {
            max_passes: 512,
            max_iterations_per_pass: 128,
            max_total_rewrites: 50_000_000,
            max_circuit_operations: 5_000_000,
            max_circuit_qubits: 8192,
            max_egraph_nodes: 2_000_000,
            max_egraph_classes: 500_000,
            max_synthesis_depth: 512,
            max_verification_operations: 8192,
            max_verification_qubits: 20,
            max_runtime_ms: 600_000,
        }
    }
}

// =============================================================================
// Main configuration
// =============================================================================

/// Complete production optimization configuration.
///
/// This is the principal configuration object consumed by the future
/// optimization pipeline.
///
/// The structure intentionally contains only policy. Concrete pass
/// implementations, target objects, cost models, analyses and verification
/// engines remain separate.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct OptimizationConfig {
    /// Optimization intensity.
    pub level: OptimizationLevel,

    /// High-level optimization profile.
    pub profile: OptimizationProfile,

    /// Primary optimization objective.
    pub objective: OptimizationObjective,

    /// Target selection.
    pub target: TargetSelection,

    /// Explicitly enabled pass identifiers.
    ///
    /// Pass identifiers are strings here deliberately. This keeps configuration
    /// independent from the pass registry and allows configuration to be
    /// deserialized before plugins/registries are initialized.
    pub enabled_passes: Vec<String>,

    /// Explicitly disabled pass identifiers.
    pub disabled_passes: Vec<String>,

    /// Determinism policy.
    pub determinism: Determinism,

    /// Parallelism policy.
    pub parallelism: Parallelism,

    /// Rewrite application policy.
    pub rewrite_policy: RewritePolicy,

    /// Fixed-point execution policy.
    pub fixed_point: FixedPointPolicy,

    /// Verification policy.
    pub verification: VerificationConfig,

    /// Optimizer-specific resource limits.
    pub limits: OptimizationLimits,

    /// Canonical Quantum IR resource limits to use when the optimizer must
    /// validate or construct IR.
    pub ir_limits: QuantumIrLimits,

    /// Whether the optimizer is allowed to introduce ancillas when a
    /// transformation explicitly requires them.
    pub allow_ancillas: bool,

    /// Whether approximate transformations are permitted.
    ///
    /// Exact transformations remain the default.
    pub allow_approximation: bool,

    /// Maximum approximation error permitted when approximation is enabled.
    pub approximation_tolerance: f64,

    /// Whether barriers are treated as hard optimization boundaries.
    pub respect_barriers: bool,

    /// Whether measurement operations are treated as hard optimization
    /// boundaries.
    pub respect_measurements: bool,

    /// Whether reset operations are treated as hard optimization boundaries.
    pub respect_resets: bool,

    /// Whether classical control dependencies must be preserved exactly.
    pub preserve_classical_dependencies: bool,

    /// Whether global phase may be ignored when proving equivalence.
    ///
    /// This does not mean global phase is silently discarded. It only permits
    /// equivalence engines to use up-to-global-phase equivalence where the
    /// configured semantic contract permits it.
    pub allow_global_phase_equivalence: bool,
}

impl Default for OptimizationConfig {
    fn default() -> Self {
        Self::balanced()
    }
}

impl OptimizationConfig {
    // -------------------------------------------------------------------------
    // Standard constructors
    // -------------------------------------------------------------------------

    /// Creates the conservative O0 configuration.
    pub fn o0() -> Self {
        Self {
            level: OptimizationLevel::O0,
            profile: OptimizationProfile::FastCompile,
            objective: OptimizationObjective::PreserveCost,
            verification: VerificationConfig {
                mode: VerificationMode::Structural,
                ..VerificationConfig::default()
            },
            ..Self::balanced()
        }
    }

    /// Creates the O1 configuration.
    pub fn o1() -> Self {
        Self {
            level: OptimizationLevel::O1,
            profile: OptimizationProfile::FastCompile,
            ..Self::balanced()
        }
    }

    /// Creates the standard production O2 configuration.
    pub fn o2() -> Self {
        Self::balanced()
    }

    /// Creates the aggressive O3 configuration.
    pub fn o3() -> Self {
        Self {
            level: OptimizationLevel::O3,
            profile: OptimizationProfile::Aggressive,
            rewrite_policy: RewritePolicy::Aggressive,
            fixed_point: FixedPointPolicy::Adaptive,
            limits: OptimizationLimits::development(),
            ..Self::balanced()
        }
    }

    /// Creates a fault-tolerant optimization configuration.
    pub fn fault_tolerant() -> Self {
        Self {
            level: OptimizationLevel::Of,
            profile: OptimizationProfile::FaultTolerant,
            objective: OptimizationObjective::MinimizeTCount,
            rewrite_policy: RewritePolicy::Verified,
            fixed_point: FixedPointPolicy::UntilStable,
            verification: VerificationConfig {
                mode: VerificationMode::ExactWhenFeasible,
                ..VerificationConfig::default()
            },
            ..Self::balanced()
        }
    }

    /// Creates a minimum-depth configuration.
    pub fn minimum_depth() -> Self {
        Self {
            level: OptimizationLevel::Od,
            profile: OptimizationProfile::MinimumDepth,
            objective: OptimizationObjective::MinimizeDepth,
            ..Self::balanced()
        }
    }

    /// Creates a minimum-gate-count configuration.
    pub fn minimum_gate_count() -> Self {
        Self {
            level: OptimizationLevel::Os,
            profile: OptimizationProfile::MinimumGateCount,
            objective: OptimizationObjective::MinimizeGateCount,
            ..Self::balanced()
        }
    }

    /// Creates a minimum-two-qubit configuration.
    pub fn minimum_two_qubit() -> Self {
        Self {
            level: OptimizationLevel::Ot,
            profile: OptimizationProfile::MinimumTwoQubit,
            objective: OptimizationObjective::MinimizeTwoQubitGates,
            ..Self::balanced()
        }
    }

    /// Creates a deterministic balanced production configuration.
    pub fn balanced() -> Self {
        Self {
            level: OptimizationLevel::O2,
            profile: OptimizationProfile::Balanced,
            objective: OptimizationObjective::Balanced,
            target: TargetSelection::Auto,
            enabled_passes: Vec::new(),
            disabled_passes: Vec::new(),
            determinism: Determinism::Deterministic,
            parallelism: Parallelism::Auto,
            rewrite_policy: RewritePolicy::Balanced,
            fixed_point: FixedPointPolicy::UntilStable,
            verification: VerificationConfig::default(),
            limits: OptimizationLimits::default(),
            ir_limits: QuantumIrLimits::production(),
            allow_ancillas: false,
            allow_approximation: false,
            approximation_tolerance: 1.0e-10,
            respect_barriers: true,
            respect_measurements: true,
            respect_resets: true,
            preserve_classical_dependencies: true,
            allow_global_phase_equivalence: true,
        }
    }

    /// Creates a configuration intended for simulation.
    pub fn simulation() -> Self {
        Self {
            profile: OptimizationProfile::Simulation,
            target: TargetSelection::Generic,
            objective: OptimizationObjective::Balanced,
            ..Self::balanced()
        }
    }

    /// Creates a verification-oriented configuration.
    pub fn verified() -> Self {
        Self {
            profile: OptimizationProfile::Verified,
            rewrite_policy: RewritePolicy::Verified,
            verification: VerificationConfig {
                mode: VerificationMode::EveryRewrite,
                ..VerificationConfig::default()
            },
            ..Self::balanced()
        }
    }

    /// Creates a conservative configuration for untrusted input.
    pub fn untrusted_input() -> Self {
        Self {
            level: OptimizationLevel::O1,
            profile: OptimizationProfile::FastCompile,
            rewrite_policy: RewritePolicy::Conservative,
            fixed_point: FixedPointPolicy::Once,
            limits: OptimizationLimits::conservative(),
            verification: VerificationConfig {
                mode: VerificationMode::Structural,
                ..VerificationConfig::default()
            },
            ..Self::balanced()
        }
    }

    // -------------------------------------------------------------------------
    // Validation
    // -------------------------------------------------------------------------

    /// Validates the complete configuration.
    ///
    /// Validation is intentionally explicit and side-effect free. It does not
    /// resolve target names or pass names because those belong to later
    /// registries.
    pub fn validate(&self) -> OptimizationConfigResult<()> {
        self.target.validate()?;
        self.limits.validate()?;
        self.verification.validate()?;
        self.parallelism.validate()?;
        self.ir_limits
            .validate()
            .map_err(|error| OptimizationConfigError::InvalidIrLimits {
                message: error.to_string(),
            })?;

        validate_pass_list("enabled_passes", &self.enabled_passes)?;
        validate_pass_list("disabled_passes", &self.disabled_passes)?;

        if self
            .enabled_passes
            .iter()
            .any(|pass| self.disabled_passes.iter().any(|disabled| pass == disabled))
        {
            return Err(OptimizationConfigError::ConflictingOptions {
                first: "enabled_passes",
                second: "disabled_passes",
            });
        }

        if !self.approximation_tolerance.is_finite() {
            return Err(OptimizationConfigError::NonFiniteFloat {
                field: "approximation_tolerance",
            });
        }

        if self.approximation_tolerance < 0.0 {
            return Err(OptimizationConfigError::NegativeFloat {
                field: "approximation_tolerance",
            });
        }

        if self.allow_approximation
            && self.approximation_tolerance == 0.0
        {
            return Err(OptimizationConfigError::ConflictingOptions {
                first: "allow_approximation",
                second: "approximation_tolerance == 0",
            });
        }

        if !self.allow_approximation
            && self.approximation_tolerance != 0.0
        {
            return Err(OptimizationConfigError::ConflictingOptions {
                first: "allow_approximation == false",
                second: "approximation_tolerance != 0",
            });
        }

        if matches!(
            self.determinism,
            Determinism::Nondeterministic
        ) && matches!(
            self.profile,
            OptimizationProfile::Verified
        ) {
            return Err(OptimizationConfigError::ConflictingOptions {
                first: "determinism.nondeterministic",
                second: "profile.verified",
            });
        }

        if matches!(
            self.determinism,
            Determinism::Nondeterministic
        ) && matches!(
            self.verification.mode,
            VerificationMode::EveryRewrite
        ) {
            return Err(OptimizationConfigError::ConflictingOptions {
                first: "determinism.nondeterministic",
                second: "verification.every_rewrite",
            });
        }

        if self.level.is_fault_tolerant()
            && !matches!(
                self.objective,
                OptimizationObjective::MinimizeTCount
                    | OptimizationObjective::MinimizeTDepth
                    | OptimizationObjective::MinimizeGateCount
                    | OptimizationObjective::Balanced
                    | OptimizationObjective::Lexicographic
            )
        {
            return Err(OptimizationConfigError::ConflictingOptions {
                first: "level.of",
                second: "objective",
            });
        }

        if self.profile.is_fault_tolerant()
            && !self.preserve_classical_dependencies
        {
            return Err(OptimizationConfigError::ConflictingOptions {
                first: "profile.fault_tolerant",
                second: "preserve_classical_dependencies == false",
            });
        }

        Ok(())
    }

    // -------------------------------------------------------------------------
    // Builder API
    // -------------------------------------------------------------------------

    /// Sets the optimization level.
    pub fn with_level(mut self, level: OptimizationLevel) -> Self {
        self.level = level;
        self
    }

    /// Sets the optimization profile.
    pub fn with_profile(mut self, profile: OptimizationProfile) -> Self {
        self.profile = profile;
        self
    }

    /// Sets the primary optimization objective.
    pub fn with_objective(
        mut self,
        objective: OptimizationObjective,
    ) -> Self {
        self.objective = objective;
        self
    }

    /// Sets the target selector.
    pub fn with_target(mut self, target: TargetSelection) -> Self {
        self.target = target;
        self
    }

    /// Sets a named target.
    pub fn with_target_name(
        mut self,
        target: impl Into<String>,
    ) -> Self {
        self.target = TargetSelection::Named(target.into());
        self
    }

    /// Replaces the enabled-pass list.
    pub fn with_enabled_passes<I, S>(
        mut self,
        passes: I,
    ) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.enabled_passes =
            passes.into_iter().map(Into::into).collect();
        self
    }

    /// Replaces the disabled-pass list.
    pub fn with_disabled_passes<I, S>(
        mut self,
        passes: I,
    ) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.disabled_passes =
            passes.into_iter().map(Into::into).collect();
        self
    }

    /// Sets deterministic operation.
    pub fn deterministic(mut self) -> Self {
        self.determinism = Determinism::Deterministic;
        self
    }

    /// Sets seeded deterministic randomized operation.
    pub fn with_seed(mut self, seed: u64) -> Self {
        self.determinism = Determinism::Seeded(seed);
        self
    }

    /// Explicitly permits nondeterministic optimization.
    pub fn nondeterministic(mut self) -> Self {
        self.determinism = Determinism::Nondeterministic;
        self
    }

    /// Sets the parallelism policy.
    pub fn with_parallelism(
        mut self,
        parallelism: Parallelism,
    ) -> Self {
        self.parallelism = parallelism;
        self
    }

    /// Sets the rewrite policy.
    pub fn with_rewrite_policy(
        mut self,
        policy: RewritePolicy,
    ) -> Self {
        self.rewrite_policy = policy;
        self
    }

    /// Sets the fixed-point policy.
    pub fn with_fixed_point_policy(
        mut self,
        policy: FixedPointPolicy,
    ) -> Self {
        self.fixed_point = policy;
        self
    }

    /// Sets verification configuration.
    pub fn with_verification(
        mut self,
        verification: VerificationConfig,
    ) -> Self {
        self.verification = verification;
        self
    }

    /// Sets optimizer resource limits.
    pub fn with_limits(
        mut self,
        limits: OptimizationLimits,
    ) -> Self {
        self.limits = limits;
        self
    }

    /// Sets canonical Quantum IR limits.
    pub fn with_ir_limits(
        mut self,
        limits: QuantumIrLimits,
    ) -> Self {
        self.ir_limits = limits;
        self
    }

    /// Allows or prohibits ancilla introduction.
    pub fn allow_ancillas(mut self, value: bool) -> Self {
        self.allow_ancillas = value;
        self
    }

    /// Enables approximate transformations with the supplied tolerance.
    pub fn allow_approximation(
        mut self,
        tolerance: f64,
    ) -> Self {
        self.allow_approximation = true;
        self.approximation_tolerance = tolerance;
        self
    }

    /// Disables approximate transformations.
    pub fn exact_only(mut self) -> Self {
        self.allow_approximation = false;
        self.approximation_tolerance = 0.0;
        self
    }

    /// Controls barrier preservation.
    pub fn respect_barriers(mut self, value: bool) -> Self {
        self.respect_barriers = value;
        self
    }

    /// Controls measurement-boundary preservation.
    pub fn respect_measurements(mut self, value: bool) -> Self {
        self.respect_measurements = value;
        self
    }

    /// Controls reset-boundary preservation.
    pub fn respect_resets(mut self, value: bool) -> Self {
        self.respect_resets = value;
        self
    }

    /// Controls preservation of classical dependencies.
    pub fn preserve_classical_dependencies(
        mut self,
        value: bool,
    ) -> Self {
        self.preserve_classical_dependencies = value;
        self
    }

    /// Controls whether equivalence checking may ignore global phase.
    pub fn allow_global_phase_equivalence(
        mut self,
        value: bool,
    ) -> Self {
        self.allow_global_phase_equivalence = value;
        self
    }

    // -------------------------------------------------------------------------
    // Queries used by future pipeline/planner/context modules
    // -------------------------------------------------------------------------

    /// Returns true when a pass is explicitly enabled.
    pub fn is_pass_enabled(&self, pass_id: &str) -> bool {
        self.enabled_passes.iter().any(|value| value == pass_id)
    }

    /// Returns true when a pass is explicitly disabled.
    pub fn is_pass_disabled(&self, pass_id: &str) -> bool {
        self.disabled_passes.iter().any(|value| value == pass_id)
    }

    /// Returns whether the configuration explicitly mentions a pass.
    pub fn has_explicit_pass_policy(&self, pass_id: &str) -> bool {
        self.is_pass_enabled(pass_id)
            || self.is_pass_disabled(pass_id)
    }

    /// Returns whether optimization is deterministic/reproducible.
    pub const fn is_reproducible(&self) -> bool {
        self.determinism.is_reproducible()
    }

    /// Returns the random seed when configured.
    pub const fn seed(&self) -> Option<u64> {
        self.determinism.seed()
    }

    /// Returns whether approximate transformations are permitted.
    pub const fn permits_approximation(&self) -> bool {
        self.allow_approximation
    }

    /// Returns the configured approximation tolerance.
    pub const fn approximation_tolerance(&self) -> f64 {
        self.approximation_tolerance
    }

    /// Returns whether semantic verification should be performed.
    pub const fn requires_semantic_verification(&self) -> bool {
        !matches!(
            self.verification.mode,
            VerificationMode::None
                | VerificationMode::Structural
        )
    }

    /// Returns whether every rewrite requires verification.
    pub const fn verifies_every_rewrite(&self) -> bool {
        matches!(
            self.verification.mode,
            VerificationMode::EveryRewrite
        )
    }

    /// Returns whether global phase equivalence is permitted.
    pub const fn permits_global_phase_equivalence(&self) -> bool {
        self.allow_global_phase_equivalence
    }

    /// Returns the stable optimization-level identifier.
    pub const fn level_id(&self) -> &'static str {
        self.level.as_str()
    }

    /// Returns the stable profile identifier.
    pub const fn profile_id(&self) -> &'static str {
        self.profile.as_str()
    }

    /// Returns the stable objective identifier.
    pub const fn objective_id(&self) -> &'static str {
        self.objective.as_str()
    }

    /// Produces a configuration suitable for an untrusted compilation
    /// boundary.
    pub fn hardened(self) -> Self {
        Self {
            rewrite_policy: RewritePolicy::Conservative,
            fixed_point: FixedPointPolicy::Once,
            limits: OptimizationLimits::conservative(),
            verification: VerificationConfig {
                mode: VerificationMode::Structural,
                ..self.verification
            },
            allow_ancillas: false,
            allow_approximation: false,
            approximation_tolerance: 0.0,
            determinism: Determinism::Deterministic,
            parallelism: Parallelism::SingleThreaded,
            ..self
        }
    }
}

// =============================================================================
// Identifier validation
// =============================================================================

/// Validates a pass/target identifier.
///
/// Identifiers are intentionally restricted to ASCII letters, digits,
/// underscore, hyphen and period. This gives stable serialization and avoids
/// ambiguity in future configuration formats.
fn validate_identifier(
    field: &'static str,
    value: &str,
) -> OptimizationConfigResult<()> {
    if value.is_empty() {
        return Err(OptimizationConfigError::EmptyIdentifier {
            field,
        });
    }

    let valid = value
        .bytes()
        .all(|byte| {
            byte.is_ascii_alphanumeric()
                || matches!(byte, b'_' | b'-' | b'.')
        });

    if !valid {
        return Err(OptimizationConfigError::InvalidIdentifier {
            field,
            value: value.to_owned(),
        });
    }

    Ok(())
}

/// Validates a pass identifier collection.
fn validate_pass_list(
    field: &'static str,
    values: &[String],
) -> OptimizationConfigResult<()> {
    for value in values {
        validate_identifier(field, value)?;
    }

    Ok(())
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn balanced_configuration_is_valid() {
        let config = OptimizationConfig::balanced();

        assert!(
            config.validate().is_ok(),
            "balanced configuration must be valid"
        );
    }

    #[test]
    fn o0_configuration_is_valid() {
        let config = OptimizationConfig::o0();

        assert_eq!(config.level, OptimizationLevel::O0);
        assert!(config.validate().is_ok());
    }

    #[test]
    fn o3_configuration_is_valid() {
        let config = OptimizationConfig::o3();

        assert_eq!(config.level, OptimizationLevel::O3);
        assert_eq!(
            config.profile,
            OptimizationProfile::Aggressive
        );
        assert!(config.validate().is_ok());
    }

    #[test]
    fn fault_tolerant_configuration_is_valid() {
        let config = OptimizationConfig::fault_tolerant();

        assert!(config.level.is_fault_tolerant());
        assert!(config.profile.is_fault_tolerant());
        assert!(config.validate().is_ok());
    }

    #[test]
    fn minimum_depth_configuration_selects_depth_objective() {
        let config = OptimizationConfig::minimum_depth();

        assert_eq!(
            config.objective,
            OptimizationObjective::MinimizeDepth
        );
        assert!(config.validate().is_ok());
    }

    #[test]
    fn minimum_two_qubit_configuration_selects_two_qubit_objective() {
        let config = OptimizationConfig::minimum_two_qubit();

        assert_eq!(
            config.objective,
            OptimizationObjective::MinimizeTwoQubitGates
        );
        assert!(config.validate().is_ok());
    }

    #[test]
    fn empty_named_target_is_rejected() {
        let config =
            OptimizationConfig::balanced()
                .with_target_name("");

        assert!(matches!(
            config.validate(),
            Err(OptimizationConfigError::EmptyIdentifier {
                field: "target"
            })
        ));
    }

    #[test]
    fn invalid_target_identifier_is_rejected() {
        let config =
            OptimizationConfig::balanced()
                .with_target_name("target/name");

        assert!(matches!(
            config.validate(),
            Err(OptimizationConfigError::InvalidIdentifier {
                field: "target",
                ..
            })
        ));
    }

    #[test]
    fn zero_parallelism_is_rejected() {
        let config = OptimizationConfig::balanced()
            .with_parallelism(Parallelism::Fixed(0));

        assert!(matches!(
            config.validate(),
            Err(OptimizationConfigError::ZeroValue {
                field: "parallelism.fixed"
            })
        ));
    }

    #[test]
    fn duplicate_enabled_and_disabled_pass_is_rejected() {
        let config = OptimizationConfig::balanced()
            .with_enabled_passes(["local.cancellation"])
            .with_disabled_passes(["local.cancellation"]);

        assert!(matches!(
            config.validate(),
            Err(OptimizationConfigError::ConflictingOptions {
                first: "enabled_passes",
                second: "disabled_passes"
            })
        ));
    }

    #[test]
    fn approximate_mode_requires_nonzero_tolerance() {
        let config =
            OptimizationConfig::balanced()
                .allow_approximation(0.0);

        assert!(matches!(
            config.validate(),
            Err(OptimizationConfigError::ConflictingOptions {
                first: "allow_approximation",
                second: "approximation_tolerance == 0"
            })
        ));
    }

    #[test]
    fn non_approximate_mode_requires_zero_tolerance() {
        let config = OptimizationConfig {
            approximation_tolerance: 1.0e-9,
            allow_approximation: false,
            ..OptimizationConfig::balanced()
        };

        assert!(matches!(
            config.validate(),
            Err(OptimizationConfigError::ConflictingOptions {
                first: "allow_approximation == false",
                second: "approximation_tolerance != 0"
            })
        ));
    }

    #[test]
    fn verification_tolerance_must_be_finite() {
        let config = OptimizationConfig {
            verification: VerificationConfig {
                tolerance: f64::NAN,
                ..VerificationConfig::default()
            },
            ..OptimizationConfig::balanced()
        };

        assert!(matches!(
            config.validate(),
            Err(OptimizationConfigError::NonFiniteFloat {
                field: "verification.tolerance"
            })
        ));
    }

    #[test]
    fn nondeterministic_verified_configuration_is_rejected() {
        let config = OptimizationConfig {
            determinism: Determinism::Nondeterministic,
            profile: OptimizationProfile::Verified,
            ..OptimizationConfig::balanced()
        };

        assert!(matches!(
            config.validate(),
            Err(OptimizationConfigError::ConflictingOptions {
                first: "determinism.nondeterministic",
                second: "profile.verified"
            })
        ));
    }

    #[test]
    fn nondeterministic_every_rewrite_verification_is_rejected() {
        let config = OptimizationConfig {
            determinism: Determinism::Nondeterministic,
            verification: VerificationConfig {
                mode: VerificationMode::EveryRewrite,
                ..VerificationConfig::default()
            },
            ..OptimizationConfig::balanced()
        };

        assert!(matches!(
            config.validate(),
            Err(OptimizationConfigError::ConflictingOptions {
                first: "determinism.nondeterministic",
                second: "verification.every_rewrite"
            })
        ));
    }

    #[test]
    fn reproducibility_is_reported_correctly() {
        assert!(
            OptimizationConfig::balanced()
                .is_reproducible()
        );

        assert!(
            OptimizationConfig::balanced()
                .with_seed(42)
                .is_reproducible()
        );

        assert!(
            !OptimizationConfig::balanced()
                .nondeterministic()
                .is_reproducible()
        );
    }

    #[test]
    fn explicit_pass_policy_is_detectable() {
        let config = OptimizationConfig::balanced()
            .with_enabled_passes([
                "local.cancellation",
                "local.rotation",
            ]);

        assert!(
            config.is_pass_enabled("local.cancellation")
        );
        assert!(
            config.has_explicit_pass_policy("local.cancellation")
        );
        assert!(
            !config.is_pass_disabled("local.cancellation")
        );
    }

    #[test]
    fn hardened_configuration_is_valid() {
        let config = OptimizationConfig::balanced().hardened();

        assert_eq!(
            config.determinism,
            Determinism::Deterministic
        );
        assert_eq!(
            config.parallelism,
            Parallelism::SingleThreaded
        );
        assert!(!config.allow_approximation);
        assert!(config.validate().is_ok());
    }

    #[test]
    fn conservative_limits_are_valid() {
        let limits = OptimizationLimits::conservative();

        assert!(limits.validate().is_ok());
        assert!(limits.max_passes > 0);
        assert!(limits.max_runtime_ms > 0);
    }

    #[test]
    fn development_limits_are_valid() {
        let limits = OptimizationLimits::development();

        assert!(limits.validate().is_ok());
        assert!(
            limits.max_circuit_operations
                > OptimizationLimits::conservative()
                    .max_circuit_operations
        );
    }

    #[test]
    fn target_name_round_trip_is_available_without_target_module() {
        let target =
            TargetSelection::named("superconducting.cz");

        assert_eq!(
            target.name(),
            Some("superconducting.cz")
        );
        assert!(target.validate().is_ok());
    }

    #[test]
    fn optimization_level_identifiers_are_stable() {
        assert_eq!(OptimizationLevel::O0.as_str(), "o0");
        assert_eq!(OptimizationLevel::O1.as_str(), "o1");
        assert_eq!(OptimizationLevel::O2.as_str(), "o2");
        assert_eq!(OptimizationLevel::O3.as_str(), "o3");
        assert_eq!(OptimizationLevel::Of.as_str(), "of");
        assert_eq!(OptimizationLevel::Os.as_str(), "os");
        assert_eq!(OptimizationLevel::Od.as_str(), "od");
        assert_eq!(OptimizationLevel::Ot.as_str(), "ot");
    }

    #[test]
    fn profiles_have_stable_identifiers() {
        assert_eq!(
            OptimizationProfile::Balanced.as_str(),
            "balanced"
        );
        assert_eq!(
            OptimizationProfile::FaultTolerant.as_str(),
            "fault_tolerant"
        );
        assert_eq!(
            OptimizationProfile::Verified.as_str(),
            "verified"
        );
    }

    #[test]
    fn objective_identifiers_are_stable() {
        assert_eq!(
            OptimizationObjective::MinimizeGateCount.as_str(),
            "minimize_gate_count"
        );
        assert_eq!(
            OptimizationObjective::MinimizeDepth.as_str(),
            "minimize_depth"
        );
        assert_eq!(
            OptimizationObjective::MinimizeTCount.as_str(),
            "minimize_t_count"
        );
    }

    #[test]
    fn serde_round_trip_preserves_configuration() {
        let original = OptimizationConfig::balanced()
            .with_target_name("superconducting.cz")
            .with_enabled_passes([
                "local.cancellation",
                "local.rotation",
            ])
            .with_seed(1234);

        let encoded =
            serde_json::to_string(&original)
                .expect("configuration should serialize");

        let decoded: OptimizationConfig =
            serde_json::from_str(&encoded)
                .expect("configuration should deserialize");

        assert_eq!(decoded, original);
        assert!(decoded.validate().is_ok());
    }

    #[test]
    fn default_configuration_is_valid() {
        let config = OptimizationConfig::default();

        assert!(config.validate().is_ok());
        assert_eq!(
            config.level,
            OptimizationLevel::O2
        );
        assert_eq!(
            config.profile,
            OptimizationProfile::Balanced
        );
        assert_eq!(
            config.determinism,
            Determinism::Deterministic
        );
    }

    #[test]
    fn exact_only_clears_approximation_tolerance() {
        let config =
            OptimizationConfig::balanced()
                .allow_approximation(1.0e-8)
                .exact_only();

        assert!(!config.allow_approximation);
        assert_eq!(
            config.approximation_tolerance,
            0.0
        );
        assert!(config.validate().is_ok());
    }
}