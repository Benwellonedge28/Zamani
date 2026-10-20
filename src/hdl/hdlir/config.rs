
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani HDLIR Configuration and Operational Policies
//!
//! Path: `src/hdl/hdlir/config.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: Rust 1.97.
//! Safety: Safe Rust only.
//! Status: HDLIR foundation; freeze after integration tests pass.
//!
//! ## PURPOSE
//!
//! Define immutable, deterministic configuration for HDLIR construction,
//! verification, compatibility, extension participation, diagnostic
//! retention, serialization, and explicitly budgeted compiler work.
//!
//! ## AUTHORITY
//!
//! This module conforms to:
//!
//! - `grammar/DESIGN.md`
//! - `grammar/spec/hdl.md`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `src/hdl/hdlir/HDLIR-SPEC.md`, when established
//! - `src/hdl/hdlir/compatibility.md`, when established
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/config.rs`, for compatible configuration-design principles
//!
//! The normative specifications own language and architecture semantics.
//! ZUIR owns the universal computational representation. HDLIR owns
//! hardware-domain representation policies and operational configuration.
//!
//! ## OWNS
//!
//! - Immutable HDLIR configuration.
//! - Verification, compatibility, extension, and diagnostic-limit policies.
//! - Explicit, optional operational work budgets.
//! - Stable budget identifiers and namespaced extension budget keys.
//! - Configuration-specific errors and validation.
//!
//! ## DOES NOT OWN
//!
//! - HDL syntax, parsing, ASTs, semantic analysis, or source spans.
//! - ZUIR configuration or universal compiler policy.
//! - Hardware capacities, target capabilities, or physical allocation.
//! - Runtime accounting, cancellation, scheduling, or parallel execution.
//! - IR structures, schema registration, verification algorithms,
//!   transformation implementations, or serialization formats.
//! - Environment variables, filesystem access, network access, or global
//!   mutable state.
//!
//! ## SCALABILITY
//!
//! An absent budget means that this configuration imposes no explicit
//! limit for the corresponding unit of work. It does not promise infinite
//! memory, execution time, address space, stack depth, or operating-system
//! resources.
//!
//! No universal maximum is imposed on signals, ports, modules, instances,
//! widths, clock domains, memories, operations, or physical resources.
//! Operational budgets constrain an invocation, not the language's
//! expressiveness or the capacity of every possible hardware target.
//!
//! Budgets use `u64` rather than `usize` so their configured values have
//! a stable numeric meaning across host architectures. Consumers must use
//! checked conversions when interacting with host-sized collection APIs.
//!
//! ## DETERMINISM
//!
//! Configuration has no clock, random, environment, filesystem, network,
//! hardware-discovery, or global-state dependencies. Budget entries are
//! stored in a `BTreeMap` and have stable key ordering.
//!
//! Determinism and semantic preservation are correctness requirements;
//! configuration cannot disable them.
//!
//! ## INTEGRATION
//!
//! - HDLIR builders consult relevant policies and enforce applicable budgets.
//! - Verification applies verification and nesting/work budgets.
//! - Schema and dialect registries consult compatibility and extension policy.
//! - Diagnostic collection applies the retention policy and reports omitted
//!   diagnostics explicitly.
//! - Serialization enforces serialized-byte and input-byte budgets.
//! - Symbolic evaluation applies its work budget in the evaluator.
//! - Pass managers enforce transformation-application budgets.
//! - Fingerprinting includes configuration values only when they affect
//!   the identity or validity of the artifact being fingerprinted.
//! - Target feasibility and physical-resource decisions remain downstream.
//!
//! This module intentionally does not import future HDLIR implementations.
//! It can be implemented and tested independently of the IR graph.
//!
//! ## FREEZE CRITERIA
//!
//! - Policies do not redefine hardware or source-language semantics.
//! - Missing budgets do not introduce arbitrary universal ceilings.
//! - Budget keys and iteration order are stable.
//! - Invalid extension keys are rejected.
//! - Budget exhaustion is explicit and cannot be confused with success.
//! - Configuration is immutable by value and has no hidden global state.
//! - Tests cover defaults, policy changes, invalid keys, zero budgets,
//!   exact limits, exhaustion, and deterministic iteration.

use core::fmt;
use std::collections::BTreeMap;

// =============================================================================
// Policies
// =============================================================================

/// Policy controlling when HDLIR verification should be requested.
///
/// This policy selects verification boundaries; it does not make malformed
/// IR valid or authorize a consumer to trust unverified external input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub enum VerificationPolicy {
    /// Verify at public operation boundaries and after transformations
    /// that can affect structural or semantic invariants.
    #[default]
    Standard,

    /// Request additional verification at supported intermediate boundaries.
    ///
    /// This may require more work but must not change program meaning.
    Thorough,

    /// Verify only at explicitly selected boundaries.
    ///
    /// This is intended for controlled internal pipelines. Consumers must
    /// still verify externally supplied or transformed IR before trusting it.
    ExplicitBoundaries,
}

/// Policy for handling incoming HDLIR schema and dialect versions.
///
/// This policy does not establish compatibility by itself. A version registry
/// or migration implementation must establish whether a particular artifact
/// can be accepted under the selected policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub enum CompatibilityPolicy {
    /// Accept only versions established to be exact matches.
    #[default]
    Exact,

    /// Also accept versions for which an explicit compatibility contract
    /// establishes that interpretation is safe.
    Compatible,

    /// Permit a documented and validated migration path.
    Migration,
}

/// Policy controlling which registered extension maturity levels may
/// participate in an HDLIR operation.
///
/// This policy does not register extensions or grant meaning to unknown
/// operations. The schema registry remains authoritative for those decisions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub enum ExtensionPolicy {
    /// Permit registered extensions marked stable by their registry.
    #[default]
    StableOnly,

    /// Permit registered experimental extensions whose definitions and
    /// compatibility contracts have been validated.
    AllowExperimental,

    /// Disallow extension-defined contributions for the relevant operation.
    Disabled,
}

/// Policy controlling behavior when the diagnostic-retention budget is
/// exhausted.
///
/// Even when truncation is allowed, the diagnostic subsystem must report
/// that diagnostics were omitted. This policy does not own diagnostic
/// storage, rendering, or source locations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub enum DiagnosticLimitPolicy {
    /// Continue where safe and report truncation explicitly.
    #[default]
    ReportTruncation,

    /// Fail the operation if required diagnostics cannot be retained.
    Fail,
}

// =============================================================================
// Work budgets
// =============================================================================

/// An optional maximum for one explicitly identified unit of compiler work.
///
/// `None` means this budget imposes no explicit policy limit. `Some(0)` is
/// valid and permits zero units of the associated work.
///
/// Budgets are operational policy, not hardware specifications. For example,
/// an IR-entity budget constrains one invocation; it does not define a maximum
/// number of entities a Zamani program may express.
///
/// This type performs pure comparisons. It does not accumulate work, reserve
/// memory, coordinate concurrent tasks, or enforce itself automatically.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct WorkBudget {
    limit: Option<u64>,
}

impl WorkBudget {
    /// Create a budget that imposes no explicit policy limit.
    ///
    /// Actual work remains bounded by the resources available to the process.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self { limit: None }
    }

    /// Create a budget with an explicit maximum amount of work.
    #[must_use]
    pub const fn bounded(limit: u64) -> Self {
        Self { limit: Some(limit) }
    }

    /// Return the configured limit, or `None` if no limit was configured.
    #[must_use]
    pub const fn limit(self) -> Option<u64> {
        self.limit
    }

    /// Return whether this budget imposes no explicit policy limit.
    #[must_use]
    pub const fn is_unlimited(self) -> bool {
        self.limit.is_none()
    }

    /// Return whether the measured amount is within this budget.
    #[must_use]
    pub const fn permits(self, amount: u64) -> bool {
        match self.limit {
            Some(limit) => amount <= limit,
            None => true,
        }
    }

    /// Check a measured amount against this budget.
    ///
    /// The caller is responsible for measuring cumulative work correctly
    /// and invoking this method at the appropriate enforcement boundary.
    ///
    /// # Errors
    ///
    /// Returns [`HdlirConfigError::BudgetExceeded`] when the measured amount
    /// exceeds the configured limit.
    pub fn check(
        self,
        kind: &BudgetKind,
        amount: u64,
    ) -> Result<(), HdlirConfigError> {
        if self.permits(amount) {
            return Ok(());
        }

        // `permits` can only be false for a bounded budget.
        let limit = self
            .limit
            .expect("a failed budget check must have a configured limit");

        Err(HdlirConfigError::BudgetExceeded {
            kind: kind.as_str().to_owned(),
            limit,
            observed: amount,
        })
    }
}

impl fmt::Display for WorkBudget {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.limit {
            Some(limit) => write!(formatter, "bounded({limit})"),
            None => formatter.write_str("unlimited"),
        }
    }
}

// =============================================================================
// Budget dimensions
// =============================================================================

/// A category of operational work that a caller may budget.
///
/// Built-in categories describe processing work rather than physical hardware
/// capacities. `Custom` permits registered extensions to introduce additional
/// budget dimensions without requiring changes to this enum.
///
/// Custom names must be namespaced, for example
/// `vendor.extension.work_units`.
///
/// The enum is non-exhaustive so future core dimensions can be introduced
/// without promising that the current set is complete.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum BudgetKind {
    /// Input bytes accepted by an operation that supports this budget.
    InputBytes,

    /// HDLIR entities processed by an operation that supports this budget.
    IrEntities,

    /// Nesting depth traversed by an operation that supports this budget.
    NestingDepth,

    /// Work units consumed by structural and semantic verification.
    VerificationWork,

    /// Number of transformation applications in a pass pipeline.
    TransformationApplications,

    /// Work units consumed by symbolic or compile-time evaluation.
    SymbolicEvaluationWork,

    /// Number of diagnostics retained by a diagnostic collector.
    Diagnostics,

    /// Bytes accepted or emitted by a serialization operation.
    SerializedBytes,

    /// Entries retained in extension or metadata structures.
    MetadataEntries,

    /// A budget dimension owned by a registered extension.
    ///
    /// Use [`BudgetKind::custom`] to construct this variant with a validated
    /// namespaced key. Configuration insertion also validates this value to
    /// guard against directly constructed invalid variants.
    Custom(String),
}

impl BudgetKind {
    /// Construct a validated, namespaced extension budget key.
    ///
    /// The accepted alphabet is ASCII lowercase letters, digits, `.`, `_`,
    /// and `-`. A key must contain at least one dot, must not start or end
    /// with a dot, and must not contain empty dot-separated segments.
    ///
    /// Examples of valid keys include `vendor.extension.work_units` and
    /// `org.example.verification_steps`.
    ///
    /// # Errors
    ///
    /// Returns [`HdlirConfigError::InvalidCustomBudgetName`] if the name
    /// does not meet the namespace rules.
    pub fn custom(name: impl Into<String>) -> Result<Self, HdlirConfigError> {
        let name = name.into();

        if !valid_custom_budget_name(&name) {
            return Err(HdlirConfigError::InvalidCustomBudgetName { name });
        }

        Ok(Self::Custom(name))
    }

    /// Return the stable configuration key for this budget dimension.
    ///
    /// These keys are suitable for deterministic configuration iteration,
    /// diagnostics, and cache-key construction. They do not, by themselves,
    /// determine which subsystem enforces the budget.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::InputBytes => "core.input_bytes",
            Self::IrEntities => "core.ir_entities",
            Self::NestingDepth => "core.nesting_depth",
            Self::VerificationWork => "core.verification_work",
            Self::TransformationApplications => {
                "core.transformation_applications"
            }
            Self::SymbolicEvaluationWork => "core.symbolic_evaluation_work",
            Self::Diagnostics => "core.diagnostics",
            Self::SerializedBytes => "core.serialized_bytes",
            Self::MetadataEntries => "core.metadata_entries",
            Self::Custom(name) => name,
        }
    }

    /// Return whether this is a built-in budget dimension.
    #[must_use]
    pub const fn is_builtin(&self) -> bool {
        !matches!(self, Self::Custom(_))
    }
}

impl fmt::Display for BudgetKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

fn valid_custom_budget_name(name: &str) -> bool {
    if name.is_empty()
        || name.trim() != name
        || !name.contains('.')
        || name.starts_with('.')
        || name.ends_with('.')
    {
        return false;
    }

    if name.split('.').any(str::is_empty) {
        return false;
    }

    name.bytes().all(|byte| {
        byte.is_ascii_lowercase()
            || byte.is_ascii_digit()
            || matches!(byte, b'.' | b'_' | b'-')
    })
}

// =============================================================================
// Configuration errors
// =============================================================================

/// An error encountered while creating, validating, or applying HDLIR
/// configuration.
///
/// Configuration errors are distinct from invalid source semantics, invalid
/// IR, unsupported target capabilities, and physical-realization failures.
/// A later HDLIR-wide error module may wrap this type, but this foundational
/// module does not depend on a not-yet-established error API.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum HdlirConfigError {
    /// A custom budget key did not meet the required namespace syntax.
    InvalidCustomBudgetName {
        /// The rejected key, retained for actionable diagnostics.
        name: String,
    },

    /// A measured amount exceeded its explicitly configured budget.
    BudgetExceeded {
        /// Stable key identifying the budget dimension.
        kind: String,

        /// Configured maximum.
        limit: u64,

        /// Measured amount that exceeded the maximum.
        observed: u64,
    },
}

impl fmt::Display for HdlirConfigError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidCustomBudgetName { name } => write!(
                formatter,
                "invalid HDLIR custom budget key `{name}`; expected a \
                 namespaced ASCII key such as `vendor.extension.work_units`"
            ),
            Self::BudgetExceeded {
                kind,
                limit,
                observed,
            } => write!(
                formatter,
                "HDLIR work budget `{kind}` exceeded: observed {observed}, \
                 configured limit {limit}"
            ),
        }
    }
}

impl std::error::Error for HdlirConfigError {}

// =============================================================================
// Canonical configuration
// =============================================================================

/// Immutable configuration for an HDLIR operation or compilation invocation.
///
/// The configuration contains policies and budget limits, not mutable
/// accounting state. Session-local counters, cancellation, and concurrent
/// resource coordination belong to the operation that performs the work.
///
/// The default configuration imposes no explicit work budgets. Applications
/// handling untrusted inputs should configure appropriate budgets and enforce
/// them at the relevant boundaries.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HdlirConfig {
    verification: VerificationPolicy,
    compatibility: CompatibilityPolicy,
    extensions: ExtensionPolicy,
    diagnostic_limit_policy: DiagnosticLimitPolicy,
    budgets: BTreeMap<BudgetKind, WorkBudget>,
}

impl Default for HdlirConfig {
    fn default() -> Self {
        Self {
            verification: VerificationPolicy::Standard,
            compatibility: CompatibilityPolicy::Exact,
            extensions: ExtensionPolicy::StableOnly,
            diagnostic_limit_policy: DiagnosticLimitPolicy::ReportTruncation,
            budgets: BTreeMap::new(),
        }
    }
}

impl HdlirConfig {
    /// Create the default configuration.
    ///
    /// All unspecified budgets are unlimited. This avoids imposing arbitrary
    /// universal design-size ceilings while allowing callers to configure
    /// explicit operational safeguards.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Return the configured verification policy.
    #[must_use]
    pub const fn verification_policy(&self) -> VerificationPolicy {
        self.verification
    }

    /// Return the configured compatibility policy.
    #[must_use]
    pub const fn compatibility_policy(&self) -> CompatibilityPolicy {
        self.compatibility
    }

    /// Return the configured extension policy.
    #[must_use]
    pub const fn extension_policy(&self) -> ExtensionPolicy {
        self.extensions
    }

    /// Return the configured diagnostic-limit policy.
    #[must_use]
    pub const fn diagnostic_limit_policy(&self) -> DiagnosticLimitPolicy {
        self.diagnostic_limit_policy
    }

    /// Return a configuration with a different verification policy.
    ///
    /// Verification correctness remains mandatory at the boundaries required
    /// by the applicable HDLIR contract. This setting selects additional
    /// verification behavior; it cannot authorize trusting malformed IR.
    #[must_use]
    pub fn with_verification_policy(
        mut self,
        policy: VerificationPolicy,
    ) -> Self {
        self.verification = policy;
        self
    }

    /// Return a configuration with a different compatibility policy.
    ///
    /// Selecting a policy does not itself establish compatibility or create
    /// a migration path. The compatibility subsystem must validate the
    /// specific versions and operations involved.
    #[must_use]
    pub fn with_compatibility_policy(
        mut self,
        policy: CompatibilityPolicy,
    ) -> Self {
        self.compatibility = policy;
        self
    }

    /// Return a configuration with a different extension policy.
    ///
    /// The registry must still validate extension registration, schema
    /// versions, and operation definitions.
    #[must_use]
    pub fn with_extension_policy(
        mut self,
        policy: ExtensionPolicy,
    ) -> Self {
        self.extensions = policy;
        self
    }

    /// Return a configuration with a different diagnostic-limit policy.
    #[must_use]
    pub fn with_diagnostic_limit_policy(
        mut self,
        policy: DiagnosticLimitPolicy,
    ) -> Self {
        self.diagnostic_limit_policy = policy;
        self
    }

    /// Return the budget configured for a dimension.
    ///
    /// An absent entry returns [`WorkBudget::unlimited`]. This does not mean
    /// the operation is exempt from actual process-resource limitations or
    /// from limits enforced by another explicitly configured subsystem.
    #[must_use]
    pub fn budget(&self, kind: &BudgetKind) -> WorkBudget {
        self.budgets
            .get(kind)
            .copied()
            .unwrap_or_else(WorkBudget::unlimited)
    }

    /// Return the explicitly configured budget entries in stable key order.
    ///
    /// The iterator excludes unspecified dimensions, which are unlimited.
    /// Consumers constructing deterministic configuration fingerprints
    /// should include the key and the exact optional limit for every entry
    /// that affects the result being cached.
    pub fn budgets(
        &self,
    ) -> impl ExactSizeIterator<Item = (&BudgetKind, &WorkBudget)> {
        self.budgets.iter()
    }

    /// Return a configuration with a budget set for one dimension.
    ///
    /// This method validates custom keys even if the caller constructed
    /// `BudgetKind::Custom` directly rather than using [`BudgetKind::custom`].
    ///
    /// Supplying `WorkBudget::unlimited()` explicitly is permitted. It is
    /// semantically equivalent to having no explicit limit for enforcement,
    /// although the explicit entry remains visible during configuration
    /// inspection and iteration.
    ///
    /// # Errors
    ///
    /// Returns [`HdlirConfigError::InvalidCustomBudgetName`] if a custom key
    /// is invalid.
    pub fn with_budget(
        mut self,
        kind: BudgetKind,
        budget: WorkBudget,
    ) -> Result<Self, HdlirConfigError> {
        if let BudgetKind::Custom(name) = &kind {
            if !valid_custom_budget_name(name) {
                return Err(HdlirConfigError::InvalidCustomBudgetName {
                    name: name.clone(),
                });
            }
        }

        self.budgets.insert(kind, budget);
        Ok(self)
    }

    /// Return a configuration with an explicit budget entry removed.
    ///
    /// After removal, the corresponding dimension has no explicit policy
    /// limit and [`HdlirConfig::budget`] returns an unlimited budget.
    #[must_use]
    pub fn without_budget(mut self, kind: &BudgetKind) -> Self {
        self.budgets.remove(kind);
        self
    }

    /// Check a measured amount against the budget configured for a dimension.
    ///
    /// This convenience method checks one measured value. It does not track
    /// cumulative work or automatically instrument the subsystem doing the
    /// work. Callers must maintain correct accounting and report incomplete
    /// operations when their budget is exhausted.
    ///
    /// # Errors
    ///
    /// Returns [`HdlirConfigError::BudgetExceeded`] when the configured
    /// budget is exceeded.
    pub fn check_budget(
        &self,
        kind: &BudgetKind,
        observed: u64,
    ) -> Result<(), HdlirConfigError> {
        self.budget(kind).check(kind, observed)
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_are_portable_and_have_no_explicit_budgets() {
        let config = HdlirConfig::default();

        assert_eq!(
            config.verification_policy(),
            VerificationPolicy::Standard
        );
        assert_eq!(
            config.compatibility_policy(),
            CompatibilityPolicy::Exact
        );
        assert_eq!(
            config.extension_policy(),
            ExtensionPolicy::StableOnly
        );
        assert_eq!(
            config.diagnostic_limit_policy(),
            DiagnosticLimitPolicy::ReportTruncation
        );
        assert_eq!(config.budgets().len(), 0);

        assert!(config
            .budget(&BudgetKind::IrEntities)
            .is_unlimited());
        assert!(config
            .budget(&BudgetKind::SerializedBytes)
            .is_unlimited());
    }

    #[test]
    fn policies_can_be_changed_without_global_state() {
        let config = HdlirConfig::new()
            .with_verification_policy(VerificationPolicy::Thorough)
            .with_compatibility_policy(CompatibilityPolicy::Compatible)
            .with_extension_policy(ExtensionPolicy::AllowExperimental)
            .with_diagnostic_limit_policy(DiagnosticLimitPolicy::Fail);

        assert_eq!(
            config.verification_policy(),
            VerificationPolicy::Thorough
        );
        assert_eq!(
            config.compatibility_policy(),
            CompatibilityPolicy::Compatible
        );
        assert_eq!(
            config.extension_policy(),
            ExtensionPolicy::AllowExperimental
        );
        assert_eq!(
            config.diagnostic_limit_policy(),
            DiagnosticLimitPolicy::Fail
        );
    }

    #[test]
    fn bounded_budget_accepts_exact_limit_and_rejects_excess() {
        let kind = BudgetKind::IrEntities;
        let budget = WorkBudget::bounded(12);

        assert!(budget.permits(0));
        assert!(budget.permits(12));
        assert!(!budget.permits(13));
        assert!(budget.check(&kind, 12).is_ok());

        assert_eq!(
            budget.check(&kind, 13),
            Err(HdlirConfigError::BudgetExceeded {
                kind: "core.ir_entities".to_owned(),
                limit: 12,
                observed: 13,
            })
        );
    }

    #[test]
    fn zero_budget_is_valid_and_permits_no_positive_work() {
        let budget = WorkBudget::bounded(0);
        let kind = BudgetKind::VerificationWork;

        assert!(budget.permits(0));
        assert!(!budget.permits(1));
        assert!(budget.check(&kind, 0).is_ok());
        assert!(matches!(
            budget.check(&kind, 1),
            Err(HdlirConfigError::BudgetExceeded { .. })
        ));
    }

    #[test]
    fn unlimited_budget_does_not_impose_a_policy_ceiling() {
        let budget = WorkBudget::unlimited();
        let kind = BudgetKind::IrEntities;

        assert!(budget.is_unlimited());
        assert!(budget.permits(0));
        assert!(budget.permits(u64::MAX));
        assert!(budget.check(&kind, u64::MAX).is_ok());
    }

    #[test]
    fn configuration_budget_defaults_to_unlimited() {
        let config = HdlirConfig::new();

        assert!(config
            .budget(&BudgetKind::NestingDepth)
            .is_unlimited());
        assert!(config
            .check_budget(&BudgetKind::NestingDepth, u64::MAX)
            .is_ok());
    }

    #[test]
    fn configuration_checks_its_configured_budget() {
        let kind = BudgetKind::SerializedBytes;
        let config = HdlirConfig::new()
            .with_budget(kind.clone(), WorkBudget::bounded(4096))
            .expect("built-in budget key is valid");

        assert!(config.check_budget(&kind, 4096).is_ok());
        assert_eq!(
            config.check_budget(&kind, 4097),
            Err(HdlirConfigError::BudgetExceeded {
                kind: "core.serialized_bytes".to_owned(),
                limit: 4096,
                observed: 4097,
            })
        );
    }

    #[test]
    fn custom_budget_names_are_validated() {
        assert_eq!(
            BudgetKind::custom("vendor.extension.work_units"),
            Ok(BudgetKind::Custom(
                "vendor.extension.work_units".to_owned()
            ))
        );

        for invalid in [
            "",
            "unqualified",
            ".vendor.extension",
            "vendor..extension",
            "vendor.extension.",
            "vendor.Extension.work",
            "vendor.extension work",
            " vendor.extension.work",
            "vendor.extension.work ",
        ] {
            assert!(matches!(
                BudgetKind::custom(invalid),
                Err(HdlirConfigError::InvalidCustomBudgetName { .. })
            ));
        }
    }

    #[test]
    fn direct_invalid_custom_variant_is_rejected_on_insertion() {
        let result = HdlirConfig::new().with_budget(
            BudgetKind::Custom("not-namespaced".to_owned()),
            WorkBudget::bounded(10),
        );

        assert!(matches!(
            result,
            Err(HdlirConfigError::InvalidCustomBudgetName { .. })
        ));
    }

    #[test]
    fn custom_budget_can_be_set_read_and_removed() {
        let kind = BudgetKind::custom("vendor.extension.work_units")
            .expect("custom budget key is valid");

        let config = HdlirConfig::new()
            .with_budget(kind.clone(), WorkBudget::bounded(25))
            .expect("validated budget can be configured");

        assert_eq!(config.budget(&kind).limit(), Some(25));
        assert_eq!(config.budgets().len(), 1);

        let updated = config.without_budget(&kind);
        assert!(updated.budget(&kind).is_unlimited());
        assert_eq!(updated.budgets().len(), 0);
    }

    #[test]
    fn budget_keys_have_stable_names() {
        assert_eq!(BudgetKind::InputBytes.as_str(), "core.input_bytes");
        assert_eq!(BudgetKind::IrEntities.as_str(), "core.ir_entities");
        assert_eq!(
            BudgetKind::VerificationWork.as_str(),
            "core.verification_work"
        );
        assert_eq!(
            BudgetKind::TransformationApplications.as_str(),
            "core.transformation_applications"
        );
        assert_eq!(
            BudgetKind::SymbolicEvaluationWork.as_str(),
            "core.symbolic_evaluation_work"
        );
        assert_eq!(
            BudgetKind::Custom("vendor.extension.work".to_owned()).as_str(),
            "vendor.extension.work"
        );
    }

    #[test]
    fn budget_iteration_is_stable_and_sorted_by_key() {
        let config = HdlirConfig::new()
            .with_budget(BudgetKind::SerializedBytes, WorkBudget::bounded(20))
            .expect("built-in key is valid")
            .with_budget(BudgetKind::InputBytes, WorkBudget::bounded(10))
            .expect("built-in key is valid")
            .with_budget(BudgetKind::Diagnostics, WorkBudget::bounded(5))
            .expect("built-in key is valid");

        let keys: Vec<&str> = config
            .budgets()
            .map(|(kind, _)| kind.as_str())
            .collect();

        assert_eq!(
            keys,
            vec![
                "core.diagnostics",
                "core.input_bytes",
                "core.serialized_bytes"
            ]
        );
    }

    #[test]
    fn equivalent_configurations_compare_equal() {
        let left = HdlirConfig::new()
            .with_budget(BudgetKind::IrEntities, WorkBudget::bounded(100))
            .expect("built-in key is valid")
            .with_budget(BudgetKind::InputBytes, WorkBudget::bounded(200))
            .expect("built-in key is valid");

        let right = HdlirConfig::new()
            .with_budget(BudgetKind::InputBytes, WorkBudget::bounded(200))
            .expect("built-in key is valid")
            .with_budget(BudgetKind::IrEntities, WorkBudget::bounded(100))
            .expect("built-in key is valid");

        assert_eq!(left, right);
    }

    #[test]
    fn explicit_unlimited_budget_is_equivalent_for_enforcement() {
        let kind = BudgetKind::IrEntities;
        let absent = HdlirConfig::new();
        let explicit = HdlirConfig::new()
            .with_budget(kind.clone(), WorkBudget::unlimited())
            .expect("built-in key is valid");

        assert!(absent.budget(&kind).is_unlimited());
        assert!(explicit.budget(&kind).is_unlimited());
        assert!(absent.check_budget(&kind, u64::MAX).is_ok());
        assert!(explicit.check_budget(&kind, u64::MAX).is_ok());

        // The configurations remain observably distinct because the latter
        // contains an explicit entry. Cache-key construction must encode
        // this distinction only if the caller's cache contract requires it.
        assert_ne!(absent, explicit);
    }
}
