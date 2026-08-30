//! Zamani Quantum Optimization — Production Optimization Profiles
//!
//! This module is the policy-resolution layer for quantum optimization.
//!
//! `config.rs` owns the serializable policy vocabulary (`OptimizationProfile`,
//! `OptimizationLevel`, `OptimizationObjective`, verification and execution
//! policies). This file owns the *meaning* of those profiles: their intended
//! objectives, safety posture, compiler effort, recommended pass identifiers,
//! and the deterministic configuration they produce.
//!
//! # Architectural boundary
//!
//! The dependency direction is intentionally one-way:
//!
//! ```text
//! quantum::ir
//!      │
//!      ▼
//! optimization::config
//!      │
//!      ▼
//! optimization::profile
//!      │
//!      ├──────────────► planner
//!      │
//!      ├──────────────► pipeline
//!      │
//!      └──────────────► registry
//! ```
//!
//! This module does **not** construct pass objects, inspect circuits, access
//! hardware, perform routing, schedule operations, execute QPUs, or perform
//! verification itself.
//!
//! The canonical Quantum IR remains `crate::quantum::ir`; this module never
//! introduces a second circuit or gate representation.
//!
//! # Stable profile contract
//!
//! A profile is a compiler policy, not a hard-coded pipeline. The planner may
//! omit a recommended pass when the circuit, target, available analyses, or
//! resource limits make it inapplicable. Conversely, a pass explicitly enabled
//! by `OptimizationConfig::enabled_passes` remains an explicit caller request
//! and is handled by the planner/registry.
//!
//! Recommended pass identifiers in this file are stable strings. They are not
//! Rust type names and therefore do not couple this file to future pass
//! implementations. The pass registry is responsible for resolving them.
//!
//! # Safety and compatibility
//!
//! - Rust 1.97 / 1.97.1;
//! - Rust 2021;
//! - safe Rust only;
//! - no `unsafe`;
//! - no global mutable state;
//! - no backend I/O;
//! - no randomness;
//! - no external runtime requirements.

use crate::quantum::optimization::config::{
    Determinism,
    FixedPointPolicy,
    OptimizationConfig,
    OptimizationLevel,
    OptimizationObjective,
    OptimizationProfile,
    Parallelism,
    RewritePolicy,
    TargetSelection,
    VerificationConfig,
    VerificationMode,
};

/// Version of the profile-policy contract.
///
/// Increment this only when the semantic meaning of an existing profile or
/// the stable profile contract changes. Adding a new profile does not require
/// changing this version because profiles are identified explicitly by name.
pub const PROFILE_SCHEMA_VERSION: u32 = 1;

// =============================================================================
// Stable pass identifiers
// =============================================================================

/// Stable identifier for canonical normalization.
pub const PASS_NORMALIZE: &str = "normalize";

/// Stable identifier for identity elimination.
pub const PASS_LOCAL_IDENTITY: &str = "local.identity";

/// Stable identifier for inverse/self-inverse cancellation.
pub const PASS_LOCAL_CANCELLATION: &str = "local.cancellation";

/// Stable identifier for generic inverse simplification.
pub const PASS_LOCAL_INVERSE: &str = "local.inverse";

/// Stable identifier for rotation fusion.
pub const PASS_LOCAL_ROTATION: &str = "local.rotation";

/// Stable identifier for commutation-based local optimization.
pub const PASS_LOCAL_COMMUTATION: &str = "local.commutation";

/// Stable identifier for peephole rewriting.
pub const PASS_LOCAL_PEEPHOLE: &str = "local.peephole";

/// Stable identifier for template rewriting.
pub const PASS_LOCAL_TEMPLATES: &str = "local.templates";

/// Stable identifier for local gate fusion.
pub const PASS_LOCAL_GATE_FUSION: &str = "local.gate_fusion";

/// Stable identifier for Clifford algebra optimization.
pub const PASS_CLIFFORD: &str = "algebra.clifford";

/// Stable identifier for phase-polynomial optimization.
pub const PASS_PHASE_POLYNOMIAL: &str = "algebra.phase_polynomial";

/// Stable identifier for diagonal algebra optimization.
pub const PASS_DIAGONAL: &str = "algebra.diagonal";

/// Stable identifier for single-qubit synthesis.
pub const PASS_SINGLE_QUBIT_SYNTHESIS: &str = "synthesis.single_qubit";

/// Stable identifier for two-qubit synthesis.
pub const PASS_TWO_QUBIT_SYNTHESIS: &str = "synthesis.two_qubit";

/// Stable identifier for target-aware decomposition.
pub const PASS_DECOMPOSITION: &str = "synthesis.decomposition";

/// Stable identifier for T-gate reduction.
pub const PASS_T_GATE_REDUCTION: &str = "fault_tolerant.t_gate_reduction";

/// Stable identifier for T-count optimization.
pub const PASS_T_COUNT: &str = "fault_tolerant.t_count";

/// Stable identifier for T-depth optimization.
pub const PASS_T_DEPTH: &str = "fault_tolerant.t_depth";

/// Stable identifier for constant parameter folding.
pub const PASS_PARAMETER_CONSTANT_FOLD: &str = "parameter.constant_fold";

/// Stable identifier for symbolic parameter simplification.
pub const PASS_PARAMETER_SYMBOLIC: &str = "parameter.symbolic";

/// Stable identifier for circuit-depth optimization.
pub const PASS_OPTIMIZE_DEPTH: &str = "passes.optimize_depth";

/// Stable identifier for circuit-width optimization.
pub const PASS_OPTIMIZE_WIDTH: &str = "passes.optimize_width";

/// Stable identifier for total gate-count optimization.
pub const PASS_OPTIMIZE_GATE_COUNT: &str = "passes.optimize_gate_count";

/// Stable identifier for two-qubit optimization.
pub const PASS_OPTIMIZE_TWO_QUBIT: &str = "passes.optimize_two_qubit";

/// Stable identifier for fault-tolerant optimization.
pub const PASS_OPTIMIZE_FAULT_TOLERANCE: &str =
    "passes.optimize_fault_tolerance";

// =============================================================================
// Profile policy classifications
// =============================================================================

/// Expected compiler effort for a profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProfileEffort {
    /// Minimal optimizer work.
    Low,

    /// Normal production work.
    Moderate,

    /// Significant search and transformation work.
    High,

    /// Maximum bounded compiler effort intended for difficult optimization.
    VeryHigh,
}

impl ProfileEffort {
    /// Stable textual identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Low => "low",
            Self::Moderate => "moderate",
            Self::High => "high",
            Self::VeryHigh => "very_high",
        }
    }
}

/// Safety posture of a profile's transformation policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProfileSafety {
    /// Only conservative, non-increasing rewrites are intended.
    Conservative,

    /// Exact semantics are required, while normal optimization is permitted.
    Exact,

    /// Every accepted rewrite is intended to be verified by policy.
    Verified,

    /// Bounded exploratory optimization is permitted; approximation remains
    /// disabled unless the caller explicitly enables it.
    Exploratory,
}

impl ProfileSafety {
    /// Stable textual identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Conservative => "conservative",
            Self::Exact => "exact",
            Self::Verified => "verified",
            Self::Exploratory => "exploratory",
        }
    }
}

/// Immutable description of one optimization profile.
///
/// This type contains policy metadata only. The pass identifiers are resolved
/// later by `registry.rs`; no implementation type is referenced here.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProfileSpec {
    /// Stable profile identifier.
    pub id: &'static str,

    /// Human-readable description.
    pub description: &'static str,

    /// Optimization intensity represented by the profile.
    pub level: OptimizationLevel,

    /// Default primary objective.
    pub objective: OptimizationObjective,

    /// Rewrite safety policy.
    pub rewrite_policy: RewritePolicy,

    /// Fixed-point policy.
    pub fixed_point: FixedPointPolicy,

    /// Verification mode required by the profile.
    pub verification_mode: VerificationMode,

    /// Target selection used by the profile.
    pub target: TargetSelection,

    /// Expected compiler effort.
    pub effort: ProfileEffort,

    /// Safety posture.
    pub safety: ProfileSafety,

    /// Whether equality-saturation/e-graph search is appropriate by default.
    pub allow_egraph: bool,

    /// Whether randomized optimization is allowed by the profile.
    pub allow_randomized: bool,

    /// Whether approximate transformations are allowed by the profile.
    pub allow_approximation: bool,

    /// Whether ancilla introduction is allowed by the profile.
    pub allow_ancillas: bool,

    /// Stable recommended pass identifiers, in preferred order.
    pub recommended_passes: &'static [&'static str],
}

impl ProfileSpec {
    /// Returns the number of recommended pass identifiers.
    pub const fn pass_count(self) -> usize {
        self.recommended_passes.len()
    }

    /// Returns true when the profile requires semantic verification rather than
    /// structural validation alone.
    pub const fn requires_semantic_verification(self) -> bool {
        !matches!(
            self.verification_mode,
            VerificationMode::None | VerificationMode::Structural
        )
    }

    /// Returns true when the profile can perform exploratory global search.
    pub const fn is_search_intensive(self) -> bool {
        self.allow_egraph
            || matches!(
                self.effort,
                ProfileEffort::High | ProfileEffort::VeryHigh
            )
    }

    /// Applies the profile's policy to an existing configuration.
    ///
    /// Caller-specific resource limits, pass allow/deny lists, and IR limits
    /// are intentionally preserved. This allows a profile to be selected first
    /// and deployment-specific safety limits to be applied independently.
    pub fn apply_to(self, mut config: OptimizationConfig) -> OptimizationConfig {
        config.level = self.level;
        config.profile = profile_from_id(self.id);
        config.objective = self.objective;
        config.target = self.target.clone();
        config.rewrite_policy = self.rewrite_policy;
        config.fixed_point = self.fixed_point;
        config.allow_ancillas = self.allow_ancillas;
        config.allow_approximation = self.allow_approximation;

        config.approximation_tolerance = if self.allow_approximation {
            config.approximation_tolerance.max(1.0e-12)
        } else {
            0.0
        };

        config.verification.mode = self.verification_mode;

        config.determinism = if self.allow_randomized {
            config.determinism
        } else {
            Determinism::Deterministic
        };

        if matches!(self.effort, ProfileEffort::Low) {
            config.parallelism = Parallelism::SingleThreaded;
        }

        config
    }

    /// Builds the canonical configuration for this profile.
    ///
    /// This starts from `OptimizationConfig::balanced()` and then applies the
    /// profile policy. The configuration's production IR/resource limits are
    /// therefore retained unless the profile-specific policy explicitly changes
    /// them.
    pub fn to_config(self) -> OptimizationConfig {
        self.apply_to(OptimizationConfig::balanced())
    }
}

// =============================================================================
// Recommended pass sets
// =============================================================================

const FAST_COMPILE_PASSES: &[&str] = &[
    PASS_NORMALIZE,
    PASS_PARAMETER_CONSTANT_FOLD,
    PASS_LOCAL_IDENTITY,
    PASS_LOCAL_CANCELLATION,
];

const BALANCED_PASSES: &[&str] = &[
    PASS_NORMALIZE,
    PASS_PARAMETER_CONSTANT_FOLD,
    PASS_PARAMETER_SYMBOLIC,
    PASS_LOCAL_IDENTITY,
    PASS_LOCAL_CANCELLATION,
    PASS_LOCAL_INVERSE,
    PASS_LOCAL_ROTATION,
    PASS_LOCAL_PEEPHOLE,
    PASS_LOCAL_TEMPLATES,
    PASS_LOCAL_COMMUTATION,
    PASS_LOCAL_GATE_FUSION,
    PASS_CLIFFORD,
    PASS_PHASE_POLYNOMIAL,
    PASS_TWO_QUBIT_SYNTHESIS,
];

const AGGRESSIVE_PASSES: &[&str] = &[
    PASS_NORMALIZE,
    PASS_PARAMETER_CONSTANT_FOLD,
    PASS_PARAMETER_SYMBOLIC,
    PASS_LOCAL_IDENTITY,
    PASS_LOCAL_CANCELLATION,
    PASS_LOCAL_INVERSE,
    PASS_LOCAL_ROTATION,
    PASS_LOCAL_COMMUTATION,
    PASS_LOCAL_PEEPHOLE,
    PASS_LOCAL_TEMPLATES,
    PASS_LOCAL_GATE_FUSION,
    PASS_CLIFFORD,
    PASS_PHASE_POLYNOMIAL,
    PASS_DIAGONAL,
    PASS_SINGLE_QUBIT_SYNTHESIS,
    PASS_TWO_QUBIT_SYNTHESIS,
    PASS_DECOMPOSITION,
    PASS_OPTIMIZE_GATE_COUNT,
    PASS_OPTIMIZE_DEPTH,
    PASS_OPTIMIZE_TWO_QUBIT,
];

const DEPTH_PASSES: &[&str] = &[
    PASS_NORMALIZE,
    PASS_PARAMETER_CONSTANT_FOLD,
    PASS_LOCAL_IDENTITY,
    PASS_LOCAL_CANCELLATION,
    PASS_LOCAL_INVERSE,
    PASS_LOCAL_ROTATION,
    PASS_LOCAL_COMMUTATION,
    PASS_LOCAL_PEEPHOLE,
    PASS_LOCAL_GATE_FUSION,
    PASS_OPTIMIZE_DEPTH,
];

const GATE_COUNT_PASSES: &[&str] = &[
    PASS_NORMALIZE,
    PASS_PARAMETER_CONSTANT_FOLD,
    PASS_PARAMETER_SYMBOLIC,
    PASS_LOCAL_IDENTITY,
    PASS_LOCAL_CANCELLATION,
    PASS_LOCAL_INVERSE,
    PASS_LOCAL_ROTATION,
    PASS_LOCAL_PEEPHOLE,
    PASS_LOCAL_TEMPLATES,
    PASS_LOCAL_GATE_FUSION,
    PASS_CLIFFORD,
    PASS_PHASE_POLYNOMIAL,
    PASS_SINGLE_QUBIT_SYNTHESIS,
    PASS_TWO_QUBIT_SYNTHESIS,
    PASS_OPTIMIZE_GATE_COUNT,
];

const TWO_QUBIT_PASSES: &[&str] = &[
    PASS_NORMALIZE,
    PASS_LOCAL_IDENTITY,
    PASS_LOCAL_CANCELLATION,
    PASS_LOCAL_INVERSE,
    PASS_LOCAL_COMMUTATION,
    PASS_LOCAL_TEMPLATES,
    PASS_CLIFFORD,
    PASS_PHASE_POLYNOMIAL,
    PASS_TWO_QUBIT_SYNTHESIS,
    PASS_OPTIMIZE_TWO_QUBIT,
];

const FAULT_TOLERANT_PASSES: &[&str] = &[
    PASS_NORMALIZE,
    PASS_PARAMETER_CONSTANT_FOLD,
    PASS_PARAMETER_SYMBOLIC,
    PASS_LOCAL_IDENTITY,
    PASS_LOCAL_CANCELLATION,
    PASS_LOCAL_INVERSE,
    PASS_LOCAL_ROTATION,
    PASS_CLIFFORD,
    PASS_PHASE_POLYNOMIAL,
    PASS_T_GATE_REDUCTION,
    PASS_T_COUNT,
    PASS_T_DEPTH,
    PASS_OPTIMIZE_FAULT_TOLERANCE,
];

const SIMULATION_PASSES: &[&str] = &[
    PASS_NORMALIZE,
    PASS_PARAMETER_CONSTANT_FOLD,
    PASS_PARAMETER_SYMBOLIC,
    PASS_LOCAL_IDENTITY,
    PASS_LOCAL_CANCELLATION,
    PASS_LOCAL_ROTATION,
    PASS_LOCAL_PEEPHOLE,
    PASS_LOCAL_TEMPLATES,
    PASS_CLIFFORD,
    PASS_TWO_QUBIT_SYNTHESIS,
];

const DEBUG_PASSES: &[&str] = &[
    PASS_NORMALIZE,
    PASS_PARAMETER_CONSTANT_FOLD,
    PASS_LOCAL_IDENTITY,
];

const VERIFIED_PASSES: &[&str] = &[
    PASS_NORMALIZE,
    PASS_PARAMETER_CONSTANT_FOLD,
    PASS_PARAMETER_SYMBOLIC,
    PASS_LOCAL_IDENTITY,
    PASS_LOCAL_CANCELLATION,
    PASS_LOCAL_INVERSE,
    PASS_LOCAL_ROTATION,
    PASS_LOCAL_PEEPHOLE,
    PASS_LOCAL_TEMPLATES,
    PASS_CLIFFORD,
    PASS_PHASE_POLYNOMIAL,
];

// =============================================================================
// Canonical profile specifications
// =============================================================================

const GENERIC_SPEC: ProfileSpec = ProfileSpec {
    id: "generic",
    description:
        "Hardware-independent production optimization with conservative, broadly applicable transformations.",
    level: OptimizationLevel::O2,
    objective: OptimizationObjective::Balanced,
    rewrite_policy: RewritePolicy::Balanced,
    fixed_point: FixedPointPolicy::UntilStable,
    verification_mode: VerificationMode::ExactWhenFeasible,
    target: TargetSelection::Generic,
    effort: ProfileEffort::Moderate,
    safety: ProfileSafety::Exact,
    allow_egraph: false,
    allow_randomized: false,
    allow_approximation: false,
    allow_ancillas: false,
    recommended_passes: BALANCED_PASSES,
};

const FAST_COMPILE_SPEC: ProfileSpec = ProfileSpec {
    id: "fast_compile",
    description:
        "Low-latency optimization using cheap, deterministic local simplification.",
    level: OptimizationLevel::O1,
    objective: OptimizationObjective::PreserveCost,
    rewrite_policy: RewritePolicy::Conservative,
    fixed_point: FixedPointPolicy::Once,
    verification_mode: VerificationMode::Structural,
    target: TargetSelection::Auto,
    effort: ProfileEffort::Low,
    safety: ProfileSafety::Conservative,
    allow_egraph: false,
    allow_randomized: false,
    allow_approximation: false,
    allow_ancillas: false,
    recommended_passes: FAST_COMPILE_PASSES,
};

const BALANCED_SPEC: ProfileSpec = ProfileSpec {
    id: "balanced",
    description:
        "Default production profile balancing compiler cost and circuit quality.",
    level: OptimizationLevel::O2,
    objective: OptimizationObjective::Balanced,
    rewrite_policy: RewritePolicy::Balanced,
    fixed_point: FixedPointPolicy::UntilStable,
    verification_mode: VerificationMode::ExactWhenFeasible,
    target: TargetSelection::Auto,
    effort: ProfileEffort::Moderate,
    safety: ProfileSafety::Exact,
    allow_egraph: false,
    allow_randomized: false,
    allow_approximation: false,
    allow_ancillas: false,
    recommended_passes: BALANCED_PASSES,
};

const AGGRESSIVE_SPEC: ProfileSpec = ProfileSpec {
    id: "aggressive",
    description:
        "High-effort bounded logical optimization for maximum circuit improvement.",
    level: OptimizationLevel::O3,
    objective: OptimizationObjective::Lexicographic,
    rewrite_policy: RewritePolicy::Aggressive,
    fixed_point: FixedPointPolicy::Adaptive,
    verification_mode: VerificationMode::ExactWhenFeasible,
    target: TargetSelection::Auto,
    effort: ProfileEffort::VeryHigh,
    safety: ProfileSafety::Exploratory,
    allow_egraph: true,
    allow_randomized: false,
    allow_approximation: false,
    allow_ancillas: false,
    recommended_passes: AGGRESSIVE_PASSES,
};

const MINIMUM_DEPTH_SPEC: ProfileSpec = ProfileSpec {
    id: "minimum_depth",
    description:
        "Prioritizes logical depth and critical-path reduction while preserving exact semantics.",
    level: OptimizationLevel::Od,
    objective: OptimizationObjective::MinimizeDepth,
    rewrite_policy: RewritePolicy::Balanced,
    fixed_point: FixedPointPolicy::Adaptive,
    verification_mode: VerificationMode::ExactWhenFeasible,
    target: TargetSelection::Auto,
    effort: ProfileEffort::High,
    safety: ProfileSafety::Exact,
    allow_egraph: false,
    allow_randomized: false,
    allow_approximation: false,
    allow_ancillas: false,
    recommended_passes: DEPTH_PASSES,
};

const MINIMUM_GATE_COUNT_SPEC: ProfileSpec = ProfileSpec {
    id: "minimum_gate_count",
    description:
        "Prioritizes reduction of total logical operation count.",
    level: OptimizationLevel::Os,
    objective: OptimizationObjective::MinimizeGateCount,
    rewrite_policy: RewritePolicy::Balanced,
    fixed_point: FixedPointPolicy::Adaptive,
    verification_mode: VerificationMode::ExactWhenFeasible,
    target: TargetSelection::Auto,
    effort: ProfileEffort::High,
    safety: ProfileSafety::Exact,
    allow_egraph: false,
    allow_randomized: false,
    allow_approximation: false,
    allow_ancillas: false,
    recommended_passes: GATE_COUNT_PASSES,
};

const MINIMUM_TWO_QUBIT_SPEC: ProfileSpec = ProfileSpec {
    id: "minimum_two_qubit",
    description:
        "Prioritizes reduction of expensive multi-qubit interactions, especially two-qubit operations.",
    level: OptimizationLevel::Ot,
    objective: OptimizationObjective::MinimizeTwoQubitGates,
    rewrite_policy: RewritePolicy::Balanced,
    fixed_point: FixedPointPolicy::Adaptive,
    verification_mode: VerificationMode::ExactWhenFeasible,
    target: TargetSelection::Auto,
    effort: ProfileEffort::High,
    safety: ProfileSafety::Exact,
    allow_egraph: false,
    allow_randomized: false,
    allow_approximation: false,
    allow_ancillas: false,
    recommended_passes: TWO_QUBIT_PASSES,
};

const FAULT_TOLERANT_SPEC: ProfileSpec = ProfileSpec {
    id: "fault_tolerant",
    description:
        "Optimizes logical fault-tolerant resource costs, with emphasis on Clifford+T, T-count and T-depth.",
    level: OptimizationLevel::Of,
    objective: OptimizationObjective::MinimizeTCount,
    rewrite_policy: RewritePolicy::Verified,
    fixed_point: FixedPointPolicy::UntilStable,
    verification_mode: VerificationMode::ExactWhenFeasible,
    target: TargetSelection::Generic,
    effort: ProfileEffort::VeryHigh,
    safety: ProfileSafety::Verified,
    allow_egraph: false,
    allow_randomized: false,
    allow_approximation: false,
    allow_ancillas: false,
    recommended_passes: FAULT_TOLERANT_PASSES,
};

const SIMULATION_SPEC: ProfileSpec = ProfileSpec {
    id: "simulation",
    description:
        "Reduces logical work and especially interaction overhead useful for statevector and circuit simulation.",
    level: OptimizationLevel::O2,
    objective: OptimizationObjective::Balanced,
    rewrite_policy: RewritePolicy::Balanced,
    fixed_point: FixedPointPolicy::UntilStable,
    verification_mode: VerificationMode::ExactWhenFeasible,
    target: TargetSelection::Generic,
    effort: ProfileEffort::Moderate,
    safety: ProfileSafety::Exact,
    allow_egraph: false,
    allow_randomized: false,
    allow_approximation: false,
    allow_ancillas: false,
    recommended_passes: SIMULATION_PASSES,
};

const DEBUG_SPEC: ProfileSpec = ProfileSpec {
    id: "debug",
    description:
        "Minimal deterministic normalization and simplification intended for compiler diagnostics and development.",
    level: OptimizationLevel::O0,
    objective: OptimizationObjective::PreserveCost,
    rewrite_policy: RewritePolicy::Conservative,
    fixed_point: FixedPointPolicy::Once,
    verification_mode: VerificationMode::Structural,
    target: TargetSelection::Generic,
    effort: ProfileEffort::Low,
    safety: ProfileSafety::Conservative,
    allow_egraph: false,
    allow_randomized: false,
    allow_approximation: false,
    allow_ancillas: false,
    recommended_passes: DEBUG_PASSES,
};

const VERIFIED_SPEC: ProfileSpec = ProfileSpec {
    id: "verified",
    description:
        "Verification-first optimization requiring semantic checking for accepted transformations.",
    level: OptimizationLevel::O2,
    objective: OptimizationObjective::Balanced,
    rewrite_policy: RewritePolicy::Verified,
    fixed_point: FixedPointPolicy::UntilStable,
    verification_mode: VerificationMode::EveryRewrite,
    target: TargetSelection::Generic,
    effort: ProfileEffort::High,
    safety: ProfileSafety::Verified,
    allow_egraph: false,
    allow_randomized: false,
    allow_approximation: false,
    allow_ancillas: false,
    recommended_passes: VERIFIED_PASSES,
};

// =============================================================================
// Public profile resolution API
// =============================================================================

impl OptimizationProfile {
    /// Returns the immutable specification for this profile.
    pub const fn spec(self) -> &'static ProfileSpec {
        match self {
            Self::Generic => &GENERIC_SPEC,
            Self::FastCompile => &FAST_COMPILE_SPEC,
            Self::Balanced => &BALANCED_SPEC,
            Self::Aggressive => &AGGRESSIVE_SPEC,
            Self::MinimumDepth => &MINIMUM_DEPTH_SPEC,
            Self::MinimumGateCount => &MINIMUM_GATE_COUNT_SPEC,
            Self::MinimumTwoQubit => &MINIMUM_TWO_QUBIT_SPEC,
            Self::FaultTolerant => &FAULT_TOLERANT_SPEC,
            Self::Simulation => &SIMULATION_SPEC,
            Self::Debug => &DEBUG_SPEC,
            Self::Verified => &VERIFIED_SPEC,
        }
    }

    /// Builds a production configuration for this profile.
    pub fn to_config(self) -> OptimizationConfig {
        self.spec().to_config()
    }

    /// Applies this profile to an existing caller-owned configuration.
    ///
    /// Explicit pass allow/deny lists and resource limits are retained. This
    /// is the preferred method when an application has its own deployment
    /// constraints.
    pub fn apply_to(self, config: OptimizationConfig) -> OptimizationConfig {
        self.spec().apply_to(config)
    }

    /// Returns the stable recommended pass identifiers.
    pub const fn recommended_passes(self) -> &'static [&'static str] {
        self.spec().recommended_passes
    }

    /// Returns whether this profile expects semantic verification.
    pub const fn requires_semantic_verification(self) -> bool {
        self.spec().requires_semantic_verification()
    }

    /// Returns the expected compiler effort.
    pub const fn effort(self) -> ProfileEffort {
        self.spec().effort
    }

    /// Returns the safety posture.
    pub const fn safety(self) -> ProfileSafety {
        self.spec().safety
    }

    /// Returns whether e-graph/equality-saturation search is appropriate.
    pub const fn allows_egraph(self) -> bool {
        self.spec().allow_egraph
    }

    /// Returns the profile's default objective.
    pub const fn objective(self) -> OptimizationObjective {
        self.spec().objective
    }

    /// Returns the profile's default optimization level.
    pub const fn level(self) -> OptimizationLevel {
        self.spec().level
    }
}

/// Returns all currently supported production profiles in stable order.
pub const fn all_profiles() -> &'static [OptimizationProfile] {
    &[
        OptimizationProfile::Generic,
        OptimizationProfile::FastCompile,
        OptimizationProfile::Balanced,
        OptimizationProfile::Aggressive,
        OptimizationProfile::MinimumDepth,
        OptimizationProfile::MinimumGateCount,
        OptimizationProfile::MinimumTwoQubit,
        OptimizationProfile::FaultTolerant,
        OptimizationProfile::Simulation,
        OptimizationProfile::Debug,
        OptimizationProfile::Verified,
    ]
}

/// Resolves a stable profile identifier without accepting aliases that could
/// make serialized configurations ambiguous.
pub fn parse_profile(value: &str) -> Option<OptimizationProfile> {
    match value.trim() {
        "generic" => Some(OptimizationProfile::Generic),
        "fast_compile" => Some(OptimizationProfile::FastCompile),
        "balanced" => Some(OptimizationProfile::Balanced),
        "aggressive" => Some(OptimizationProfile::Aggressive),
        "minimum_depth" => Some(OptimizationProfile::MinimumDepth),
        "minimum_gate_count" => Some(OptimizationProfile::MinimumGateCount),
        "minimum_two_qubit" => Some(OptimizationProfile::MinimumTwoQubit),
        "fault_tolerant" => Some(OptimizationProfile::FaultTolerant),
        "simulation" => Some(OptimizationProfile::Simulation),
        "debug" => Some(OptimizationProfile::Debug),
        "verified" => Some(OptimizationProfile::Verified),
        _ => None,
    }
}

/// Resolves a stable profile identifier and returns its immutable specification.
pub fn spec_for_name(value: &str) -> Option<&'static ProfileSpec> {
    parse_profile(value).map(OptimizationProfile::spec)
}

/// Validates that a configuration is compatible with its selected profile.
///
/// This deliberately performs only profile-level policy validation. Target
/// names, pass availability, IR validity, and concrete hardware capabilities
/// remain the responsibility of their owning modules.
pub fn validate_profile_compatibility(
    config: &OptimizationConfig,
) -> Result<(), &'static str> {
    let spec = config.profile.spec();

    if config.level != spec.level {
        return Err("optimization level does not match the selected profile");
    }

    if matches!(spec.safety, ProfileSafety::Verified)
        && !matches!(config.rewrite_policy, RewritePolicy::Verified)
    {
        return Err("verified profile requires verified rewrite policy");
    }

    if !spec.allow_approximation && config.allow_approximation {
        return Err("selected profile does not permit approximate transformations");
    }

    if !spec.allow_ancillas && config.allow_ancillas {
        return Err("selected profile does not permit ancilla introduction");
    }

    if !spec.allow_randomized
        && !matches!(config.determinism, Determinism::Deterministic)
    {
        return Err("selected profile requires deterministic optimization");
    }

    if matches!(
        spec.verification_mode,
        VerificationMode::EveryRewrite
    ) && !matches!(
        config.verification.mode,
        VerificationMode::EveryRewrite
    ) {
        return Err("selected profile requires every-rewrite verification");
    }

    Ok(())
}

/// Returns the verification configuration implied by a profile while
/// preserving caller-selected verification resource limits.
pub fn verification_for_profile(
    profile: OptimizationProfile,
    base: &VerificationConfig,
) -> VerificationConfig {
    let mut verification = base.clone();
    verification.mode = profile.spec().verification_mode;
    verification
}

/// Returns true when a pass identifier is part of a profile's recommended
/// policy.
pub fn recommends_pass(
    profile: OptimizationProfile,
    pass_id: &str,
) -> bool {
    profile
        .recommended_passes()
        .iter()
        .any(|candidate| *candidate == pass_id)
}

/// Returns true for profiles whose primary objective is explicitly
/// objective-specific rather than balanced.
pub const fn is_objective_specific(
    profile: OptimizationProfile,
) -> bool {
    matches!(
        profile,
        OptimizationProfile::MinimumDepth
            | OptimizationProfile::MinimumGateCount
            | OptimizationProfile::MinimumTwoQubit
            | OptimizationProfile::FaultTolerant
    )
}

/// Maps a canonical profile identifier back to the enum.
///
/// This helper is private to profile resolution so the public API never needs
/// a second source of profile names.
fn profile_from_id(id: &str) -> OptimizationProfile {
    match id {
        "generic" => OptimizationProfile::Generic,
        "fast_compile" => OptimizationProfile::FastCompile,
        "balanced" => OptimizationProfile::Balanced,
        "aggressive" => OptimizationProfile::Aggressive,
        "minimum_depth" => OptimizationProfile::MinimumDepth,
        "minimum_gate_count" => OptimizationProfile::MinimumGateCount,
        "minimum_two_qubit" => OptimizationProfile::MinimumTwoQubit,
        "fault_tolerant" => OptimizationProfile::FaultTolerant,
        "simulation" => OptimizationProfile::Simulation,
        "debug" => OptimizationProfile::Debug,
        "verified" => OptimizationProfile::Verified,
        _ => OptimizationProfile::Balanced,
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_declared_profile_has_a_spec() {
        for profile in all_profiles() {
            assert_eq!(profile.spec().id, profile.as_str());
            assert!(!profile.spec().description.is_empty());
            assert!(profile.spec().pass_count() > 0);
        }
    }

    #[test]
    fn profile_names_round_trip() {
        for profile in all_profiles() {
            let parsed = parse_profile(profile.as_str());
            assert_eq!(parsed, Some(*profile));
        }
    }

    #[test]
    fn balanced_profile_is_deterministic_and_exact_by_default() {
        let config = OptimizationProfile::Balanced.to_config();

        assert_eq!(
            config.profile,
            OptimizationProfile::Balanced
        );
        assert_eq!(config.level, OptimizationLevel::O2);
        assert_eq!(
            config.objective,
            OptimizationObjective::Balanced
        );
        assert!(matches!(
            config.determinism,
            Determinism::Deterministic
        ));
        assert!(matches!(
            config.verification.mode,
            VerificationMode::ExactWhenFeasible
        ));
        assert!(
            validate_profile_compatibility(&config).is_ok()
        );
    }

    #[test]
    fn fast_compile_is_conservative() {
        let config = OptimizationProfile::FastCompile.to_config();

        assert_eq!(config.level, OptimizationLevel::O1);
        assert!(matches!(
            config.rewrite_policy,
            RewritePolicy::Conservative
        ));
        assert_eq!(
            config.verification.mode,
            VerificationMode::Structural
        );
        assert_eq!(
            OptimizationProfile::FastCompile.recommended_passes(),
            FAST_COMPILE_PASSES
        );
    }

    #[test]
    fn aggressive_profile_is_bounded_but_search_intensive() {
        let spec = OptimizationProfile::Aggressive.spec();

        assert!(spec.allow_egraph);
        assert!(spec.is_search_intensive());
        assert!(!spec.allow_approximation);
        assert!(!spec.allow_randomized);
    }

    #[test]
    fn fault_tolerant_profile_selects_t_count() {
        let config =
            OptimizationProfile::FaultTolerant.to_config();

        assert_eq!(config.level, OptimizationLevel::Of);
        assert_eq!(
            config.objective,
            OptimizationObjective::MinimizeTCount
        );
        assert!(matches!(
            config.rewrite_policy,
            RewritePolicy::Verified
        ));
        assert!(recommends_pass(
            OptimizationProfile::FaultTolerant,
            PASS_T_GATE_REDUCTION
        ));
        assert!(recommends_pass(
            OptimizationProfile::FaultTolerant,
            PASS_T_COUNT
        ));
    }

    #[test]
    fn verified_profile_requires_every_rewrite_verification() {
        let config = OptimizationProfile::Verified.to_config();

        assert_eq!(
            config.verification.mode,
            VerificationMode::EveryRewrite
        );
        assert_eq!(
            config.rewrite_policy,
            RewritePolicy::Verified
        );
        assert!(
            validate_profile_compatibility(&config).is_ok()
        );
    }

    #[test]
    fn incompatible_profile_settings_are_rejected() {
        let mut config = OptimizationProfile::Balanced.to_config();
        config.allow_approximation = true;

        assert!(
            validate_profile_compatibility(&config).is_err()
        );
    }

    #[test]
    fn caller_limits_and_pass_overrides_survive_profile_application() {
        let mut config = OptimizationConfig::balanced();

        config.enabled_passes =
            vec!["custom.pass".to_string()];
        config.limits.max_passes = 7;

        let applied =
            OptimizationProfile::MinimumDepth.apply_to(config);

        assert_eq!(
            applied.enabled_passes,
            vec!["custom.pass"]
        );
        assert_eq!(applied.limits.max_passes, 7);
        assert_eq!(
            applied.profile,
            OptimizationProfile::MinimumDepth
        );
        assert_eq!(
            applied.objective,
            OptimizationObjective::MinimizeDepth
        );
    }

    #[test]
    fn verification_helper_preserves_resource_limits() {
        let mut base = VerificationConfig::default();

        base.max_qubits = 9;
        base.randomized_trials = 77;

        let result = verification_for_profile(
            OptimizationProfile::Verified,
            &base,
        );

        assert_eq!(result.max_qubits, 9);
        assert_eq!(result.randomized_trials, 77);
        assert_eq!(
            result.mode,
            VerificationMode::EveryRewrite
        );
    }

    #[test]
    fn stable_pass_ids_are_non_empty() {
        for profile in all_profiles() {
            for pass in profile.recommended_passes() {
                assert!(!pass.trim().is_empty());
            }
        }
    }
}