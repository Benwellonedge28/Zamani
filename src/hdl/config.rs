
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani HDL Configuration
//!
//! Path: `src/hdl/config.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: safe Rust only.
//!
//! ## Purpose
//!
//! Define explicit, deterministic configuration for HDL-domain analysis.
//! Configuration controls operational policy; it does not define HDL
//! language semantics or describe the physical capabilities of a target.
//!
//! ## Authority
//!
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4` and the HDL delegate grammars
//! - `grammar/specification/semantic-model.md`
//! - `grammar/validation/semantic-boundaries.md`
//! - `src/frontend/parser/domains/hdl.rs`
//! - `src/frontend/semantic/domains/hdl.rs`
//!
//! The grammar and normative specifications own syntax and meaning.
//! The existing parser owns parser-specific budgets such as delimiter
//! nesting and tokens per construct. The shared semantic session owns
//! diagnostic accumulation, cancellation and propagation of incomplete
//! analysis. This module must not duplicate those owners.
//!
//! ## Owns
//!
//! - Explicit budgets for HDL-domain analysis work.
//! - Warning-handling policy for consumers that support it.
//! - Policy for unknown semantic extensions.
//! - Stable defaults and accessors for these configuration values.
//!
//! ## Does not own
//!
//! - Parser limits, parser recovery, or lexer configuration.
//! - Diagnostic record types, codes, source spans, or diagnostic storage.
//! - Semantic validation rules or the canonical semantic model.
//! - Target capabilities, hardware capacity, resource allocation or
//!   physical implementation.
//! - Cancellation-token lifetime or runtime/session state.
//! - HDLIR structures, builders, verifiers, passes, or lowering.
//!
//! ## Budget semantics
//!
//! An absent budget (`None`) means that this configuration imposes no
//! budget for that counter. It does not promise infinite memory or runtime.
//! The host, allocator, caller, and other configured operational policies
//! still constrain execution.
//!
//! A present budget of zero is valid and permits zero units of the
//! corresponding work. Consumers must check budgets before doing work that
//! would exceed them. Counter arithmetic must use checked arithmetic or
//! saturating accounting; wraparound must never make exhausted work appear
//! available.
//!
//! These are operational limits, not language-level limits. They must not
//! be interpreted as maximum widths, signal counts, port counts, hierarchy
//! depths, module counts, clock counts, memory sizes, or target capacities.
//!
//! If a budget prevents a required check from completing, the caller must
//! mark the analysis incomplete and must not report successful complete
//! validation.
//!
//! ## Integration
//!
//! The HDL analysis/session layer consumes [`HdlConfig`] and enforces its
//! budgets. Existing parser limits remain configured through
//! `crate::frontend::parser::domains::hdl::HdlParserLimits`.
//!
//! The AST adapter and semantic validator must preserve the distinction
//! between invalid source, unsupported extensions, target infeasibility,
//! and incomplete analysis. Diagnostic conversion remains the responsibility
//! of the existing frontend diagnostic integration.
//!
//! ## Determinism
//!
//! Configuration contains no global state, clock, random seed, hardware
//! handle, or environment lookup. Equivalent configurations compare equal.
//! Analysis determinism remains the responsibility of the consuming passes.
//!
//! ## Freeze criteria
//!
//! Freeze this API when each budget has a documented owner and consumer,
//! budget exhaustion is propagated as incomplete analysis, extension policy
//! is mapped explicitly at the parser/semantic boundary, and repository
//! tests verify the default and boundary behavior.

use std::fmt;

/// Operational budgets for HDL-domain analysis.
///
/// Each optional budget limits an operational counter, not the size of a
/// valid hardware design. `None` means that this configuration does not set
/// that budget.
///
/// Parser-specific token and nesting budgets are deliberately excluded:
/// they are owned by the existing HDL parser configuration.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[non_exhaustive]
pub struct HdlOperationalBudgets {
    /// Maximum diagnostic records an owning HDL analysis session may emit.
    ///
    /// `None` means no limit from this configuration. `Some(0)` permits no
    /// diagnostic records; if a diagnostic is required but cannot be
    /// recorded, the session must still expose an incomplete result through
    /// its result/status channel.
    pub max_diagnostics: Option<usize>,

    /// Maximum units of analysis work.
    ///
    /// The consuming analysis defines and documents what counts as one unit.
    /// The same unit definition must be used consistently for a given pass
    /// and documented in its public contract.
    pub max_analysis_steps: Option<u64>,

    /// Maximum dependency edges materialized by one analysis session.
    ///
    /// This is an operational memory/work budget, not a semantic limit on
    /// the number of dependencies a program may describe.
    pub max_dependency_edges: Option<usize>,

    /// Maximum symbolic-expansion steps performed by an analysis session.
    ///
    /// This budget does not cap the language's expression size or symbolic
    /// expressiveness. Exhaustion means that the relevant analysis is
    /// incomplete.
    pub max_symbolic_expansion_steps: Option<u64>,
}

impl HdlOperationalBudgets {
    /// Construct an unlimited-by-configuration budget set.
    ///
    /// This does not disable host resource limits or budgets imposed by
    /// other compiler layers.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            max_diagnostics: None,
            max_analysis_steps: None,
            max_dependency_edges: None,
            max_symbolic_expansion_steps: None,
        }
    }

    /// Return whether all budgets are absent.
    ///
    /// This describes only the values in this structure; it is not a claim
    /// that the process or the complete compiler is resource-unbounded.
    #[must_use]
    pub const fn is_unlimited(&self) -> bool {
        self.max_diagnostics.is_none()
            && self.max_analysis_steps.is_none()
            && self.max_dependency_edges.is_none()
            && self.max_symbolic_expansion_steps.is_none()
    }
}

/// Policy for diagnostics classified as warnings.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum WarningPolicy {
    /// Preserve warning severity.
    #[default]
    Emit,

    /// Treat warnings as errors at the reporting/validation boundary.
    ///
    /// Consumers must apply this policy consistently and must not mutate
    /// the canonical diagnostic record in a way that loses its original
    /// classification. If the repository's diagnostic pipeline supports a
    /// separate warning-as-error status, prefer that representation.
    Deny,
}

/// Policy for an unknown HDL semantic extension.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum UnknownExtensionPolicy {
    /// Reject an extension whose semantics are not registered or understood.
    ///
    /// This is the conservative default. Preserving unknown syntax does not
    /// establish that the syntax is semantically valid or safely realizable.
    #[default]
    Reject,

    /// Permit an integration layer to preserve an unknown extension as
    /// opaque input when the parser and downstream consumers support that
    /// behavior.
    ///
    /// This option does not authorize the compiler to silently ignore the
    /// extension, claim complete semantic validation, or lower it as though
    /// its meaning were known.
    PreserveOpaque,
}

/// Policy options for HDL analysis and validation.
///
/// This structure contains policy, not mutable analysis state. A session
/// should copy or borrow it as needed and keep counters and diagnostics in
/// that session rather than modifying the configuration.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[non_exhaustive]
pub struct HdlPolicy {
    /// How warning-level conditions should affect validation.
    pub warnings: WarningPolicy,

    /// How unrecognized semantic extensions should be handled.
    pub unknown_extensions: UnknownExtensionPolicy,
}

/// Configuration for HDL-domain analysis.
///
/// This type deliberately has no target identity, physical resource count,
/// vendor selection, or HDLIR dependency. Target selection and feasibility
/// belong to their own downstream contracts.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[non_exhaustive]
pub struct HdlConfig {
    /// Operational budgets for HDL-domain analysis.
    pub budgets: HdlOperationalBudgets,

    /// Reporting and extension policy.
    pub policy: HdlPolicy,
}

impl HdlConfig {
    /// Create the default HDL configuration.
    ///
    /// Defaults impose no HDL-domain operational budgets, preserve warnings,
    /// and reject unknown semantic extensions. Parser-specific safety
    /// budgets remain controlled by the existing parser configuration.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a configuration with no budgets imposed by this module.
    ///
    /// Warnings are emitted and unknown semantic extensions are rejected.
    /// This does not disable parser, operating-system, allocator, or
    /// caller-provided resource limits.
    #[must_use]
    pub fn unlimited() -> Self {
        Self {
            budgets: HdlOperationalBudgets::unlimited(),
            policy: HdlPolicy::default(),
        }
    }

    /// Return whether this configuration imposes no HDL-domain budgets.
    ///
    /// This says nothing about limits imposed by other compiler layers.
    #[must_use]
    pub const fn has_unlimited_budgets(&self) -> bool {
        self.budgets.is_unlimited()
    }

    /// Return the configured warning policy.
    #[must_use]
    pub const fn warning_policy(&self) -> WarningPolicy {
        self.policy.warnings
    }

    /// Return the configured unknown-extension policy.
    #[must_use]
    pub const fn unknown_extension_policy(&self) -> UnknownExtensionPolicy {
        self.policy.unknown_extensions
    }
}

impl fmt::Display for WarningPolicy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Emit => f.write_str("emit"),
            Self::Deny => f.write_str("deny"),
        }
    }
}

impl fmt::Display for UnknownExtensionPolicy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Reject => f.write_str("reject"),
            Self::PreserveOpaque => f.write_str("preserve-opaque"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_configuration_has_no_hdl_domain_budgets() {
        let config = HdlConfig::default();

        assert!(config.has_unlimited_budgets());
        assert_eq!(config.budgets.max_diagnostics, None);
        assert_eq!(config.budgets.max_analysis_steps, None);
        assert_eq!(config.budgets.max_dependency_edges, None);
        assert_eq!(config.budgets.max_symbolic_expansion_steps, None);
    }

    #[test]
    fn default_policy_is_conservative() {
        let config = HdlConfig::default();

        assert_eq!(config.warning_policy(), WarningPolicy::Emit);
        assert_eq!(
            config.unknown_extension_policy(),
            UnknownExtensionPolicy::Reject
        );
    }

    #[test]
    fn zero_is_a_valid_explicit_operational_budget() {
        let config = HdlConfig {
            budgets: HdlOperationalBudgets {
                max_diagnostics: Some(0),
                max_analysis_steps: Some(0),
                max_dependency_edges: Some(0),
                max_symbolic_expansion_steps: Some(0),
            },
            ..HdlConfig::default()
        };

        assert!(!config.has_unlimited_budgets());
        assert_eq!(config.budgets.max_diagnostics, Some(0));
        assert_eq!(config.budgets.max_analysis_steps, Some(0));
        assert_eq!(config.budgets.max_dependency_edges, Some(0));
        assert_eq!(
            config.budgets.max_symbolic_expansion_steps,
            Some(0)
        );
    }

    #[test]
    fn budgets_can_be_configured_independently() {
        let config = HdlConfig {
            budgets: HdlOperationalBudgets {
                max_diagnostics: Some(128),
                max_analysis_steps: Some(10_000),
                max_dependency_edges: None,
                max_symbolic_expansion_steps: Some(2_000),
            },
            ..HdlConfig::default()
        };

        assert_eq!(config.budgets.max_diagnostics, Some(128));
        assert_eq!(config.budgets.max_analysis_steps, Some(10_000));
        assert_eq!(config.budgets.max_dependency_edges, None);
        assert_eq!(
            config.budgets.max_symbolic_expansion_steps,
            Some(2_000)
        );
    }

    #[test]
    fn configurations_have_value_semantics() {
        let first = HdlConfig::default();
        let second = HdlConfig::new();

        assert_eq!(first, second);
        assert_eq!(first, HdlConfig::unlimited());
    }

    #[test]
    fn warning_policy_is_explicit() {
        assert_eq!(WarningPolicy::default(), WarningPolicy::Emit);
        assert_eq!(WarningPolicy::Emit.to_string(), "emit");
        assert_eq!(WarningPolicy::Deny.to_string(), "deny");
    }

    #[test]
    fn unknown_extension_policy_is_explicit() {
        assert_eq!(
            UnknownExtensionPolicy::default(),
            UnknownExtensionPolicy::Reject
        );
        assert_eq!(
            UnknownExtensionPolicy::Reject.to_string(),
            "reject"
        );
        assert_eq!(
            UnknownExtensionPolicy::PreserveOpaque.to_string(),
            "preserve-opaque"
        );
    }

    #[test]
    fn unlimited_budget_constructor_matches_default_budget_values() {
        assert_eq!(
            HdlOperationalBudgets::unlimited(),
            HdlOperationalBudgets::default()
        );
        assert!(HdlOperationalBudgets::unlimited().is_unlimited());
    }

    #[test]
    fn config_does_not_claim_that_the_host_is_unbounded() {
        let config = HdlConfig::unlimited();

        // "Unlimited" means no limits set in this config only.
        assert!(config.has_unlimited_budgets());
    }
}
