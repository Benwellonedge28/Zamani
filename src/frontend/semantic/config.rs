
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Semantic Analysis Configuration
//!
//! File: `src/frontend/semantic/config.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Define the immutable, deterministic configuration contract shared by the
//! semantic analyzer, analysis session, pipeline, diagnostics, type system,
//! symbol resolver, effect checker, capability checker, resource analyzer,
//! contract validator, domain validators, and semantic extensions.
//!
//! ## STATUS
//!
//! Proposed implementation for the native frontend semantic subsystem.
//! This file is intentionally independent of semantic modules that have not
//! yet been integrated. Its API is the contract those modules should consume.
//!
//! ## AUTHORITY
//!
//! - `grammar/specification/semantic-model.md`
//! - `grammar/validation/semantic-boundaries.md`
//! - `grammar/specification/compatibility.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/poco-reaf.md`
//! - `grammar/DESIGN.md`
//! - The canonical grammar and canonical native AST.
//!
//! The normative specifications take precedence over implementation
//! convenience and explanatory examples.
//!
//! ## OWNS
//!
//! - Language-version selection.
//! - Semantic analysis mode.
//! - Compatibility policy.
//! - Extension activation policy.
//! - Diagnostic policy.
//! - Explicit, configurable semantic-analysis work budgets.
//! - Configuration validation and builder-style updates.
//!
//! ## DOES_NOT_OWN
//!
//! - AST, source spans, tokens, or source maps.
//! - Symbol tables, type representations, or inference algorithms.
//! - Effect, capability, resource, or contract semantics.
//! - Domain-specific semantic rules.
//! - Physical resource discovery or allocation.
//! - Target feasibility, device selection, placement, routing, or scheduling.
//! - IR definitions, optimization, lowering, or runtime execution.
//! - Mutable state for an individual analysis.
//!
//! ## POCO-REAF AND SCALABILITY
//!
//! This module defines no universal finite ceiling for:
//!
//! - qubits, processors, cores, threads, devices, or accelerators;
//! - registers, machine widths, tensor dimensions, or tensor rank;
//! - memory, storage, nodes, links, or communication capacity;
//! - declarations, functions, types, or computational domains.
//!
//! An unset budget means that this configuration imposes no explicit policy
//! limit for that budget. It does not promise infinite memory, address space,
//! stack space, processing time, or operating-system resources.
//!
//! Finite budgets are opt-in operational safeguards. They must never silently
//! truncate semantic facts or turn incomplete analysis into a valid result.
//! The analysis session or the component performing work must enforce each
//! configured budget and report exhaustion explicitly.
//!
//! ## DETERMINISM
//!
//! Configuration contains no randomness, clock access, environment access,
//! filesystem access, network access, hardware discovery, or global mutable
//! state. Identical configuration values have identical meaning.
//!
//! Deterministic analysis and diagnostic ordering are mandatory pipeline
//! responsibilities; callers cannot disable those correctness guarantees
//! through this configuration.
//!
//! ## INTEGRATION CONTRACT
//!
//! - `api.rs` validates configuration before starting analysis.
//! - `context.rs` exposes a shared, read-only configuration reference.
//! - `session.rs` owns mutable, per-invocation accounting.
//! - `pipeline.rs` selects passes according to the declared analysis mode.
//! - `diagnostics/` applies the diagnostic policy and reports budget exhaustion.
//! - `symbols/`, `types/`, `effects/`, `capabilities/`, `resources/`,
//!   `contracts/`, and `domains/` consume the same configuration contract.
//! - `incremental/` includes every configuration value that affects semantic
//!   results in cache identity or invalidation.
//! - Domain extensions must not introduce competing global configuration.
//! - Target planning and runtime code must not reinterpret semantic budgets as
//!   source-language or hardware-capacity limits.
//!
//! This file deliberately has no dependency on those consumers, allowing
//! their interfaces to be implemented against this stable contract.

use core::fmt;
use core::num::NonZeroUsize;

/// Semantic language version.
///
/// This is a version of the language's normative semantic contract, not a
/// compiler release, crate version, grammar-file revision, or hardware version.
///
/// A version is a value, not proof that the implementation supports it.
/// `api.rs` must validate requested versions against the versions actually
/// implemented by the compiler.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LanguageVersion {
    major: u32,
    minor: u32,
    patch: u32,
}

impl LanguageVersion {
    /// Construct a semantic language version.
    #[must_use]
    pub const fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Return the major version component.
    #[must_use]
    pub const fn major(self) -> u32 {
        self.major
    }

    /// Return the minor version component.
    #[must_use]
    pub const fn minor(self) -> u32 {
        self.minor
    }

    /// Return the patch version component.
    #[must_use]
    pub const fn patch(self) -> u32 {
        self.patch
    }
}

impl fmt::Display for LanguageVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

/// How semantic analysis treats errors and incomplete information.
///
/// This setting controls the analysis workflow, not the definition of valid
/// Zamani programs. Strict mode must not accept an invalid program. Recovery
/// mode may continue analysis to collect further diagnostics, but its result
/// must retain an explicit invalid, unresolved, or incomplete status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AnalysisMode {
    /// Prefer fail-fast behavior when a fatal analysis error prevents safe
    /// continuation. Recoverable diagnostics may still be accumulated.
    Strict,

    /// Continue independent analysis after recoverable errors to provide
    /// useful diagnostics for editors and interactive tools.
    #[default]
    Recovery,
}

/// Policy for language-version compatibility.
///
/// Version support must be checked against the implementation's declared
/// compatibility matrix. This enum does not imply that any version is
/// implemented.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum CompatibilityPolicy {
    /// Require the requested version to be explicitly supported.
    #[default]
    Exact,

    /// Permit a compatible version only when the implementation's
    /// compatibility registry establishes that compatibility.
    Compatible,

    /// Permit a documented migration path when the implementation provides
    /// and validates that migration.
    Migration,
}

/// Policy for semantic extensions and domain contributions.
///
/// Extension identifiers, versions, and definitions belong to the extension
/// registry. This policy determines whether registered contributions may be
/// used; it does not define or hard-code the set of available domains.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ExtensionPolicy {
    /// Only stable, explicitly registered extensions may participate.
    #[default]
    StableOnly,

    /// Allow explicitly registered experimental extensions when the
    /// implementation and caller have enabled their use.
    AllowExperimental,

    /// Disallow semantic extensions for this analysis.
    Disabled,
}

/// Policy for diagnostics produced during semantic analysis.
///
/// Diagnostic limits are operational safeguards. Reaching a limit must
/// produce an explicit incomplete-analysis indication; the pipeline must
/// not silently report that no additional errors exist.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum DiagnosticPolicy {
    /// Retain diagnostics up to the configured budget and preserve the fact
    /// that additional diagnostics may have been omitted.
    #[default]
    BoundedByLimits,

    /// Treat an inability to retain required diagnostics as a fatal analysis
    /// failure rather than continuing with a potentially misleading result.
    FailOnLimit,
}

/// A configurable budget for one semantic-analysis work dimension.
///
/// `None` means no explicit policy limit is imposed by this budget.
/// `Some(limit)` specifies the maximum accepted amount of work or retained
/// data for the corresponding dimension.
///
/// This type is deliberately generic. It does not identify any hardware
/// capacity or computational domain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct WorkBudget {
    limit: Option<NonZeroUsize>,
}

impl WorkBudget {
    /// Construct an unlimited policy budget.
    ///
    /// Actual execution remains bounded by available system resources.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self { limit: None }
    }

    /// Construct a finite budget.
    ///
    /// Returns `None` for zero because a zero budget is generally better
    /// represented by an explicitly rejected configuration than by a
    /// seemingly usable budget. Use `WorkBudget::unlimited()` when no
    /// explicit policy limit is desired.
    #[must_use]
    pub const fn bounded(limit: NonZeroUsize) -> Self {
        Self { limit: Some(limit) }
    }

    /// Construct a finite budget from a `usize`.
    ///
    /// Zero is rejected. The caller must choose a positive budget or
    /// explicitly select an unlimited budget.
    #[must_use]
    pub const fn from_usize(limit: usize) -> Option<Self> {
        match NonZeroUsize::new(limit) {
            Some(value) => Some(Self::bounded(value)),
            None => None,
        }
    }

    /// Return the configured maximum, or `None` if unlimited.
    #[must_use]
    pub const fn limit(self) -> Option<NonZeroUsize> {
        self.limit
    }

    /// Return whether this budget imposes no explicit limit.
    #[must_use]
    pub const fn is_unlimited(self) -> bool {
        self.limit.is_none()
    }

    /// Return whether `amount` is within this budget.
    ///
    /// This is a pure check, not accounting. The analysis session must track
    /// cumulative use and call this check at the relevant enforcement point.
    #[must_use]
    pub const fn permits(self, amount: usize) -> bool {
        match self.limit {
            Some(limit) => amount <= limit.get(),
            None => true,
        }
    }
}

/// Configurable semantic-analysis resource budgets.
///
/// These dimensions describe implementation work, not language expressiveness
/// or hardware capacity. All budgets are unlimited by default so the language
/// is not silently restricted by arbitrary universal constants.
///
/// Callers processing untrusted source can explicitly configure finite budgets.
/// The pipeline must report which budget was exhausted and must mark the
/// result incomplete or failed as appropriate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct SemanticLimits {
    /// Total semantic work units charged by analysis passes.
    ///
    /// The pipeline must define a stable charging convention before using this
    /// field for deterministic results or reproducible builds.
    total_work: WorkBudget,

    /// Number of semantic facts or entities created during one analysis.
    ///
    /// This is an implementation safeguard, not a maximum program size in the
    /// Zamani language.
    semantic_entities: WorkBudget,

    /// Number of dependency edges processed by semantic graph algorithms.
    dependency_edges: WorkBudget,

    /// Maximum observed recursive semantic-analysis depth.
    ///
    /// Iterative implementations can avoid consuming call-stack depth, but
    /// must still respect this budget if they use it as a semantic-work policy.
    recursion_depth: WorkBudget,

    /// Number of constraint-solver steps charged during analysis.
    constraint_steps: WorkBudget,

    /// Number of constant-evaluation steps charged during analysis.
    constant_evaluation_steps: WorkBudget,

    /// Number of diagnostics retained by the analysis session.
    diagnostics: WorkBudget,

    /// Number of extension contributions examined during analysis.
    extension_contributions: WorkBudget,
}

impl SemanticLimits {
    /// Construct limits with no explicit policy ceilings.
    ///
    /// This is the default POCO-REAF configuration. It does not imply that
    /// the host has unlimited memory, time, or address space.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            total_work: WorkBudget::unlimited(),
            semantic_entities: WorkBudget::unlimited(),
            dependency_edges: WorkBudget::unlimited(),
            recursion_depth: WorkBudget::unlimited(),
            constraint_steps: WorkBudget::unlimited(),
            constant_evaluation_steps: WorkBudget::unlimited(),
            diagnostics: WorkBudget::unlimited(),
            extension_contributions: WorkBudget::unlimited(),
        }
    }

    /// Return the total-work budget.
    #[must_use]
    pub const fn total_work(self) -> WorkBudget {
        self.total_work
    }

    /// Return the semantic-entity budget.
    #[must_use]
    pub const fn semantic_entities(self) -> WorkBudget {
        self.semantic_entities
    }

    /// Return the dependency-edge budget.
    #[must_use]
    pub const fn dependency_edges(self) -> WorkBudget {
        self.dependency_edges
    }

    /// Return the recursion-depth budget.
    #[must_use]
    pub const fn recursion_depth(self) -> WorkBudget {
        self.recursion_depth
    }

    /// Return the constraint-solver-step budget.
    #[must_use]
    pub const fn constraint_steps(self) -> WorkBudget {
        self.constraint_steps
    }

    /// Return the constant-evaluation-step budget.
    #[must_use]
    pub const fn constant_evaluation_steps(self) -> WorkBudget {
        self.constant_evaluation_steps
    }

    /// Return the diagnostic-retention budget.
    #[must_use]
    pub const fn diagnostics(self) -> WorkBudget {
        self.diagnostics
    }

    /// Return the extension-contribution budget.
    #[must_use]
    pub const fn extension_contributions(self) -> WorkBudget {
        self.extension_contributions
    }

    /// Set the total-work budget.
    #[must_use]
    pub const fn with_total_work(mut self, budget: WorkBudget) -> Self {
        self.total_work = budget;
        self
    }

    /// Set the semantic-entity budget.
    #[must_use]
    pub const fn with_semantic_entities(mut self, budget: WorkBudget) -> Self {
        self.semantic_entities = budget;
        self
    }

    /// Set the dependency-edge budget.
    #[must_use]
    pub const fn with_dependency_edges(mut self, budget: WorkBudget) -> Self {
        self.dependency_edges = budget;
        self
    }

    /// Set the recursion-depth budget.
    #[must_use]
    pub const fn with_recursion_depth(mut self, budget: WorkBudget) -> Self {
        self.recursion_depth = budget;
        self
    }

    /// Set the constraint-solver-step budget.
    #[must_use]
    pub const fn with_constraint_steps(mut self, budget: WorkBudget) -> Self {
        self.constraint_steps = budget;
        self
    }

    /// Set the constant-evaluation-step budget.
    #[must_use]
    pub const fn with_constant_evaluation_steps(
        mut self,
        budget: WorkBudget,
    ) -> Self {
        self.constant_evaluation_steps = budget;
        self
    }

    /// Set the diagnostic-retention budget.
    #[must_use]
    pub const fn with_diagnostics(mut self, budget: WorkBudget) -> Self {
        self.diagnostics = budget;
        self
    }

    /// Set the extension-contribution budget.
    #[must_use]
    pub const fn with_extension_contributions(
        mut self,
        budget: WorkBudget,
    ) -> Self {
        self.extension_contributions = budget;
        self
    }
}

/// Immutable semantic-analysis configuration.
///
/// Construct a configuration with `SemanticConfig::default()` and customize
/// it with the `with_*` methods. The configuration is designed to be shared
/// read-only across passes; mutable accounting belongs to `session.rs`.
///
/// Configuration equality is meaningful and may be used as part of an
/// incremental-analysis cache key. Any additional semantic option introduced
/// in the future must be included in equality and hashing if it can affect
/// semantic results.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SemanticConfig {
    language_version: LanguageVersion,
    analysis_mode: AnalysisMode,
    compatibility_policy: CompatibilityPolicy,
    extension_policy: ExtensionPolicy,
    diagnostic_policy: DiagnosticPolicy,
    limits: SemanticLimits,
}

impl SemanticConfig {
    /// Construct a configuration for the specified semantic language version.
    ///
    /// This constructor does not assert that the requested version is
    /// implemented. The public analysis API must perform that validation.
    #[must_use]
    pub const fn new(language_version: LanguageVersion) -> Self {
        Self {
            language_version,
            analysis_mode: AnalysisMode::Recovery,
            compatibility_policy: CompatibilityPolicy::Exact,
            extension_policy: ExtensionPolicy::StableOnly,
            diagnostic_policy: DiagnosticPolicy::BoundedByLimits,
            limits: SemanticLimits::unlimited(),
        }
    }

    /// Return the requested language version.
    #[must_use]
    pub const fn language_version(self) -> LanguageVersion {
        self.language_version
    }

    /// Return the selected analysis mode.
    #[must_use]
    pub const fn analysis_mode(self) -> AnalysisMode {
        self.analysis_mode
    }

    /// Return the compatibility policy.
    #[must_use]
    pub const fn compatibility_policy(self) -> CompatibilityPolicy {
        self.compatibility_policy
    }

    /// Return the extension policy.
    #[must_use]
    pub const fn extension_policy(self) -> ExtensionPolicy {
        self.extension_policy
    }

    /// Return the diagnostic policy.
    #[must_use]
    pub const fn diagnostic_policy(self) -> DiagnosticPolicy {
        self.diagnostic_policy
    }

    /// Return all semantic-analysis budgets.
    #[must_use]
    pub const fn limits(self) -> SemanticLimits {
        self.limits
    }

    /// Select strict or recovery-oriented analysis.
    #[must_use]
    pub const fn with_analysis_mode(mut self, mode: AnalysisMode) -> Self {
        self.analysis_mode = mode;
        self
    }

    /// Select the language-version compatibility policy.
    #[must_use]
    pub const fn with_compatibility_policy(
        mut self,
        policy: CompatibilityPolicy,
    ) -> Self {
        self.compatibility_policy = policy;
        self
    }

    /// Select the extension activation policy.
    #[must_use]
    pub const fn with_extension_policy(
        mut self,
        policy: ExtensionPolicy,
    ) -> Self {
        self.extension_policy = policy;
        self
    }

    /// Select diagnostic-retention behavior.
    #[must_use]
    pub const fn with_diagnostic_policy(
        mut self,
        policy: DiagnosticPolicy,
    ) -> Self {
        self.diagnostic_policy = policy;
        self
    }

    /// Replace the semantic-analysis budgets.
    #[must_use]
    pub const fn with_limits(mut self, limits: SemanticLimits) -> Self {
        self.limits = limits;
        self
    }

    /// Return the default configuration for an explicitly supplied version.
    ///
    /// The caller must supply the language version rather than relying on an
    /// implicit version that may change as the language evolves.
    #[must_use]
    pub const fn for_version(language_version: LanguageVersion) -> Self {
        Self::new(language_version)
    }
}

impl Default for SemanticConfig {
    /// Default to the initial semantic language version.
    ///
    /// This default is a bootstrap convention, not a claim that all syntax or
    /// semantics for that version have been implemented. A production API
    /// must reject unsupported versions and must update this default only
    /// through the language-versioning process.
    fn default() -> Self {
        Self::new(LanguageVersion::new(1, 0, 0))
    }
}

/// Errors that can be detected by validating configuration itself.
///
/// This type deliberately does not include unsupported-language-version
/// errors because support depends on the implementation's compatibility
/// registry, which belongs to the API/compatibility layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConfigError {
    /// The configuration requests experimental extensions while the
    /// implementation has not enabled experimental support.
    ExperimentalExtensionsNotEnabled,

    /// The configuration's diagnostic policy is incompatible with the
    /// caller's requirement for partial results.
    PartialResultsNotPermitted,
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ExperimentalExtensionsNotEnabled => {
                f.write_str(
                    "experimental extensions were requested but are not enabled",
                )
            }
            Self::PartialResultsNotPermitted => {
                f.write_str(
                    "partial semantic results are not permitted by this configuration",
                )
            }
        }
    }
}

impl std::error::Error for ConfigError {}

impl SemanticConfig {
    /// Validate invariants that can be checked without consulting the
    /// implementation's version or extension registries.
    ///
    /// Registry-dependent validation remains the responsibility of `api.rs`
    /// and the compatibility/extension subsystems.
    ///
    /// The current configuration is intentionally constructed through
    /// invariant-preserving values, so this function is a stable integration
    /// point for future cross-field checks.
    pub fn validate(&self) -> Result<(), ConfigError> {
        // Registry-dependent checks must not be guessed here.
        // In particular, an experimental policy is not itself an error:
        // whether it is enabled depends on the caller and implementation.
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn language_version_has_stable_components_and_display() {
        let version = LanguageVersion::new(2, 3, 4);

        assert_eq!(version.major(), 2);
        assert_eq!(version.minor(), 3);
        assert_eq!(version.patch(), 4);
        assert_eq!(version.to_string(), "2.3.4");
    }

    #[test]
    fn default_configuration_is_explicit_and_hardware_independent() {
        let config = SemanticConfig::default();

        assert_eq!(config.language_version(), LanguageVersion::new(1, 0, 0));
        assert_eq!(config.analysis_mode(), AnalysisMode::Recovery);
        assert_eq!(
            config.compatibility_policy(),
            CompatibilityPolicy::Exact
        );
        assert_eq!(config.extension_policy(), ExtensionPolicy::StableOnly);
        assert_eq!(
            config.diagnostic_policy(),
            DiagnosticPolicy::BoundedByLimits
        );

        let limits = config.limits();
        assert!(limits.total_work().is_unlimited());
        assert!(limits.semantic_entities().is_unlimited());
        assert!(limits.dependency_edges().is_unlimited());
        assert!(limits.recursion_depth().is_unlimited());
        assert!(limits.constraint_steps().is_unlimited());
        assert!(limits.constant_evaluation_steps().is_unlimited());
        assert!(limits.diagnostics().is_unlimited());
        assert!(limits.extension_contributions().is_unlimited());
    }

    #[test]
    fn budgets_distinguish_unlimited_from_finite_limits() {
        let unlimited = WorkBudget::unlimited();

        assert!(unlimited.is_unlimited());
        assert!(unlimited.permits(0));
        assert!(unlimited.permits(usize::MAX));

        let finite = WorkBudget::from_usize(10).expect("positive budget");

        assert!(!finite.is_unlimited());
        assert!(finite.permits(0));
        assert!(finite.permits(10));
        assert!(!finite.permits(11));
        assert!(WorkBudget::from_usize(0).is_none());
    }

    #[test]
    fn finite_budgets_are_opt_in_and_composable() {
        let limits = SemanticLimits::unlimited()
            .with_total_work(
                WorkBudget::from_usize(50_000).expect("positive budget"),
            )
            .with_dependency_edges(
                WorkBudget::from_usize(10_000).expect("positive budget"),
            );

        assert_eq!(limits.total_work().limit().map(NonZeroUsize::get), Some(50_000));
        assert_eq!(
            limits.dependency_edges().limit().map(NonZeroUsize::get),
            Some(10_000)
        );
        assert!(limits.semantic_entities().is_unlimited());
    }

    #[test]
    fn builder_methods_preserve_unmodified_options() {
        let config = SemanticConfig::new(LanguageVersion::new(1, 2, 0))
            .with_analysis_mode(AnalysisMode::Strict)
            .with_compatibility_policy(CompatibilityPolicy::Compatible)
            .with_extension_policy(ExtensionPolicy::AllowExperimental)
            .with_diagnostic_policy(DiagnosticPolicy::FailOnLimit);

        assert_eq!(config.language_version(), LanguageVersion::new(1, 2, 0));
        assert_eq!(config.analysis_mode(), AnalysisMode::Strict);
        assert_eq!(
            config.compatibility_policy(),
            CompatibilityPolicy::Compatible
        );
        assert_eq!(
            config.extension_policy(),
            ExtensionPolicy::AllowExperimental
        );
        assert_eq!(
            config.diagnostic_policy(),
            DiagnosticPolicy::FailOnLimit
        );
        assert!(config.limits().total_work().is_unlimited());
    }

    #[test]
    fn configuration_is_deterministic_and_hashable() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        fn fingerprint<T: Hash>(value: &T) -> u64 {
            let mut hasher = DefaultHasher::new();
            value.hash(&mut hasher);
            hasher.finish()
        }

        let first = SemanticConfig::default();
        let second = SemanticConfig::default();

        assert_eq!(first, second);
        assert_eq!(fingerprint(&first), fingerprint(&second));
    }

    #[test]
    fn configuration_validation_accepts_valid_defaults() {
        assert!(SemanticConfig::default().validate().is_ok());
    }

    #[test]
    fn analysis_modes_do_not_change_the_language_version() {
        let strict = SemanticConfig::default()
            .with_analysis_mode(AnalysisMode::Strict);
        let recovery = SemanticConfig::default()
            .with_analysis_mode(AnalysisMode::Recovery);

        assert_eq!(strict.language_version(), recovery.language_version());
        assert_ne!(strict.analysis_mode(), recovery.analysis_mode());
    }
}
