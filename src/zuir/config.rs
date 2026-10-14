
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Configuration and Policy
//!
//! Path: `src/zuir/config.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: 1.97
//!
//! ## PURPOSE
//!
//! Define immutable, deterministic configuration for ZUIR construction,
//! verification, transformation, compatibility, extension activation,
//! diagnostic retention, and explicitly budgeted compiler work.
//!
//! ## AUTHORITY
//!
//! This module conforms to:
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/errors.rs`
//! - `src/zuir/version.rs`
//! - `src/zuir/diagnostics.rs`
//! - `grammar/DESIGN.md`
//! - `grammar/spec/diagnostics.md`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/poco-reaf.md`
//!
//! ## OWNS
//!
//! - Immutable ZUIR configuration.
//! - Verification, compatibility, and extension policies.
//! - Explicit, optional operational work budgets.
//! - Budget keys, including namespaced extension-defined keys.
//! - Configuration validation and configuration-specific errors.
//!
//! ## DOES NOT OWN
//!
//! - IR nodes, identifier allocation, types, effects, or semantic meaning.
//! - Actual resource accounting or budget enforcement loops.
//! - Diagnostic rendering or source locations.
//! - Target discovery, physical resource allocation, or scheduling.
//! - Serialization formats, environment variables, or global mutable state.
//!
//! ## SCALABILITY
//!
//! An absent budget means that this configuration imposes no explicit limit
//! for that work dimension. It does not promise infinite memory, time,
//! address space, stack space, or operating-system resources.
//!
//! There are no universal hardware capacities or fixed program-size ceilings.
//! A caller may impose explicit operational budgets, and the responsible
//! subsystem must enforce them and report exhaustion without treating it as
//! proof that a program is semantically invalid.
//!
//! ## DETERMINISM
//!
//! Configuration has no clock, random, environment, filesystem, network,
//! hardware-discovery, or global-state dependencies. Ordered maps provide
//! deterministic iteration. Deterministic semantics and canonical output are
//! correctness requirements and cannot be disabled here.
//!
//! ## INTEGRATION
//!
//! - `api.rs` validates configuration before starting a ZUIR operation.
//! - Builders and verifiers read policy without owning global configuration.
//! - Pass managers enforce relevant work budgets and verify transformation
//!   preconditions and postconditions.
//! - `diagnostics.rs` applies diagnostic-retention budgets and reports
//!   truncation explicitly.
//! - Serialization applies serialized-byte budgets at the encoding boundary.
//! - Constant evaluation applies its work budget in the evaluator.
//! - Incremental compilation includes every result-affecting configuration
//!   value in its cache identity.
//! - Target capability and physical-resource checks remain downstream.
//!
//! This module intentionally does not import the builder, verifier, pass,
//! serialization, diagnostics, or backend implementations.

use core::fmt;
use std::collections::BTreeMap;

use crate::zuir::errors::{ErrorPhase, ZuirError, ZuirErrorKind, ZuirResult};

// =============================================================================
// Policy enums
// =============================================================================

/// How thoroughly ZUIR is verified at public operation boundaries.
///
/// These options choose when verification work is requested. They do not
/// make malformed IR valid, and they do not authorize consumers to trust
/// unverified input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub enum VerificationPolicy {
    /// Verify at public boundaries and after transformations that can affect
    /// structural or semantic invariants.
    #[default]
    Standard,

    /// Request additional verification at supported intermediate boundaries.
    /// This may cost more work but must not change program meaning.
    Thorough,

    /// Verify at explicitly selected boundaries only. This mode is suitable
    /// for controlled internal pipelines; consumers must still verify before
    /// trusting externally supplied or transformed IR.
    ExplicitBoundaries,
}

/// How incoming schema or dialect versions are handled.
///
/// This policy does not establish compatibility by itself. A compatibility
/// registry or migration implementation must prove that a particular version
/// can be accepted under the selected policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub enum CompatibilityPolicy {
    /// Accept only versions explicitly identified as exact matches.
    #[default]
    Exact,

    /// Also accept versions for which a compatibility contract explicitly
    /// establishes compatibility.
    Compatible,

    /// Permit a documented, available, and validated migration path.
    Migration,
}

/// Which registered extension maturity levels may participate.
///
/// Registration and validation remain responsibilities of the dialect or
/// extension registry. This policy never grants an unregistered extension
/// executable meaning.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub enum ExtensionPolicy {
    /// Permit registered extensions marked stable by their registry.
    #[default]
    StableOnly,

    /// Permit registered experimental extensions when their definitions
    /// and compatibility contracts have been validated.
    AllowExperimental,

    /// Disallow extension-defined contributions for this operation.
    Disabled,
}

/// How diagnostic retention-budget exhaustion should affect a caller.
///
/// The diagnostic subsystem must still record that diagnostics were omitted.
/// This policy does not own diagnostic storage or rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub enum DiagnosticLimitPolicy {
    /// Continue where safe, while reporting truncation explicitly.
    #[default]
    ReportTruncation,

    /// Treat inability to retain required diagnostics as an operation failure.
    Fail,
}

// =============================================================================
// Work budgets
// =============================================================================

/// An optional limit on one explicitly identified unit of compiler work.
///
/// `None` means this budget imposes no explicit limit. `Some(0)` is a valid
/// budget that permits zero units of the associated work.
///
/// Budgets are policy, not hardware specifications. For example, a budget
/// on IR entities limits what one invocation is willing to process; it does
/// not declare a maximum number of entities that Zamani programs can express.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct WorkBudget {
    limit: Option<usize>,
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
    pub const fn bounded(limit: usize) -> Self {
        Self { limit: Some(limit) }
    }

    /// Return the configured limit, or `None` if unlimited.
    #[must_use]
    pub const fn limit(self) -> Option<usize> {
        self.limit
    }

    /// Whether this budget imposes no explicit limit.
    #[must_use]
    pub const fn is_unlimited(self) -> bool {
        self.limit.is_none()
    }

    /// Whether the supplied amount is within this budget.
    ///
    /// This is a pure check. It does not accumulate work, reserve resources,
    /// or enforce the budget across concurrent tasks.
    #[must_use]
    pub const fn permits(self, amount: usize) -> bool {
        match self.limit {
            Some(limit) => amount <= limit,
            None => true,
        }
    }

    /// Check a measured amount and return a structured error on exhaustion.
    ///
    /// The caller is responsible for measuring cumulative work correctly and
    /// invoking this method at the appropriate enforcement boundary.
    pub fn check(self, amount: usize, description: &str) -> ZuirResult<()> {
        if self.permits(amount) {
            return Ok(());
        }

        Err(
            ZuirError::new(
                ZuirErrorKind::LimitExceeded,
                format!("configured work budget exceeded: {description}"),
            )
            .with_phase(ErrorPhase::Configuration)
            .with_note(format!(
                "measured amount: {amount}; configured limit: {}",
                self.limit
                    .map(|value| value.to_string())
                    .unwrap_or_else(|| "unlimited".to_owned())
            )),
        )
    }
}

impl fmt::Display for WorkBudget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.limit {
            Some(limit) => write!(f, "bounded({limit})"),
            None => f.write_str("unlimited"),
        }
    }
}

// =============================================================================
// Budget dimensions
// =============================================================================

/// A category of compiler work for which a caller may configure a budget.
///
/// Built-in categories describe processing work, not physical hardware
/// capacities. `Custom` allows independently developed extensions to add
/// budget dimensions without modifying this enum.
///
/// Custom keys must be namespaced, for example `vendor.extension.work_units`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum BudgetKind {
    /// Input bytes accepted by an operation that supports this budget.
    InputBytes,

    /// IR entities processed by an operation that supports this budget.
    IrEntities,

    /// Nesting depth processed by an operation that supports this budget.
    NestingDepth,

    /// Total pass applications in a pass pipeline.
    PassApplications,

    /// Work units consumed by compile-time constant evaluation.
    ConstantEvaluationWork,

    /// Number of retained diagnostics.
    Diagnostics,

    /// Bytes emitted or accepted by a serialization operation.
    SerializedBytes,

    /// Entries retained in extensible metadata structures.
    MetadataEntries,

    /// A budget dimension owned by a registered extension.
    Custom(String),
}

impl BudgetKind {
    /// Construct a validated, namespaced extension budget key.
    ///
    /// Accepted characters are ASCII lowercase letters, digits, `.`, `_`,
    /// and `-`. A key must contain at least one dot, must not start or end
    /// with a separator, and must not contain empty dot-separated segments.
    pub fn custom(name: impl Into<String>) -> ZuirResult<Self> {
        let name = name.into();

        if !valid_custom_budget_name(&name) {
            return Err(configuration_error(
                "invalid custom budget key",
                "Use a nonempty namespaced key such as `vendor.extension.work_units`.",
            ));
        }

        Ok(Self::Custom(name))
    }

    /// Return a stable key for configuration, diagnostics, or cache identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::InputBytes => "core.input_bytes",
            Self::IrEntities => "core.ir_entities",
            Self::NestingDepth => "core.nesting_depth",
            Self::PassApplications => "core.pass_applications",
            Self::ConstantEvaluationWork => "core.constant_evaluation_work",
            Self::Diagnostics => "core.diagnostics",
            Self::SerializedBytes => "core.serialized_bytes",
            Self::MetadataEntries => "core.metadata_entries",
            Self::Custom(name) => name,
        }
    }
}

fn valid_custom_budget_name(name: &str) -> bool {
    if name.trim() != name || name.is_empty() || !name.contains('.') {
        return false;
    }

    if name.starts_with('.') || name.ends_with('.') {
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
// Canonical configuration
// =============================================================================

/// Immutable configuration for a ZUIR operation or compilation invocation.
///
/// The value is safe to clone and share between subsystems. It contains no
/// mutable accounting state; invocation-local counters and cancellation
/// mechanisms belong to the session or operation that performs the work.
///
/// The default configuration imposes no explicit work budgets. Applications
/// handling untrusted input should choose operational budgets appropriate to
/// their deployment and enforce them at the relevant boundaries.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ZuirConfig {
    verification: VerificationPolicy,
    compatibility: CompatibilityPolicy,
    extensions: ExtensionPolicy,
    diagnostic_limit_policy: DiagnosticLimitPolicy,
    budgets: BTreeMap<BudgetKind, WorkBudget>,
}

impl Default for ZuirConfig {
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

impl ZuirConfig {
    /// Create the default configuration.
    ///
    /// All unspecified budgets are unlimited. This avoids imposing an
    /// arbitrary universal size ceiling while leaving applications free to
    /// configure explicit operational safeguards.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Return the selected verification policy.
    #[must_use]
    pub const fn verification_policy(&self) -> VerificationPolicy {
        self.verification
    }

    /// Return the selected compatibility policy.
    #[must_use]
    pub const fn compatibility_policy(&self) -> CompatibilityPolicy {
        self.compatibility
    }

    /// Return the selected extension policy.
    #[must_use]
    pub const fn extension_policy(&self) -> ExtensionPolicy {
        self.extensions
    }

    /// Return the diagnostic retention-limit policy.
    #[must_use]
    pub const fn diagnostic_limit_policy(&self) -> DiagnosticLimitPolicy {
        self.diagnostic_limit_policy
    }

    /// Return whether deterministic behavior is required.
    ///
    /// Determinism is an architectural correctness contract, not an optional
    /// performance setting. This method intentionally has no setter.
    #[must_use]
    pub const fn deterministic(&self) -> bool {
        true
    }

    /// Return the budget for a dimension.
    ///
    /// An absent entry means unlimited. The returned value is copied, so
    /// callers cannot mutate this configuration through the result.
    #[must_use]
    pub fn budget(&self, kind: &BudgetKind) -> WorkBudget {
        self.budgets
            .get(kind)
            .copied()
            .unwrap_or_else(WorkBudget::unlimited)
    }

    /// Iterate over explicitly configured budgets in deterministic key order.
    pub fn configured_budgets(
        &self,
    ) -> impl ExactSizeIterator<Item = (&BudgetKind, &WorkBudget)> {
        self.budgets.iter()
    }

    /// Set the verification policy.
    #[must_use]
    pub fn with_verification_policy(mut self, policy: VerificationPolicy) -> Self {
        self.verification = policy;
        self
    }

    /// Set the version compatibility policy.
    #[must_use]
    pub fn with_compatibility_policy(mut self, policy: CompatibilityPolicy) -> Self {
        self.compatibility = policy;
        self
    }

    /// Set the extension activation policy.
    #[must_use]
    pub fn with_extension_policy(mut self, policy: ExtensionPolicy) -> Self {
        self.extensions = policy;
        self
    }

    /// Set the diagnostic retention-limit policy.
    #[must_use]
    pub fn with_diagnostic_limit_policy(
        mut self,
        policy: DiagnosticLimitPolicy,
    ) -> Self {
        self.diagnostic_limit_policy = policy;
        self
    }

    /// Set or replace a work budget.
    ///
    /// Passing `WorkBudget::unlimited()` removes any effective limit for this
    /// dimension while retaining an explicit entry in the configuration.
    /// Custom keys are validated again here so direct construction of the
    /// public `BudgetKind::Custom` variant cannot bypass validation.
    pub fn with_budget(
        mut self,
        kind: BudgetKind,
        budget: WorkBudget,
    ) -> ZuirResult<Self> {
        if let BudgetKind::Custom(name) = &kind {
            if !valid_custom_budget_name(name) {
                return Err(configuration_error(
                    "invalid custom budget key",
                    "Custom budget keys must be valid, namespaced ASCII identifiers.",
                ));
            }
        }

        self.budgets.insert(kind, budget);
        Ok(self)
    }

    /// Remove a configured budget, reverting that dimension to unlimited.
    #[must_use]
    pub fn without_budget(mut self, kind: &BudgetKind) -> Self {
        self.budgets.remove(kind);
        self
    }

    /// Validate configuration invariants.
    ///
    /// Most policy values are valid by construction. This method remains an
    /// explicit boundary for public APIs and protects against invalid custom
    /// budget keys created through public enum variants.
    pub fn validate(&self) -> ZuirResult<()> {
        for kind in self.budgets.keys() {
            if let BudgetKind::Custom(name) = kind {
                if !valid_custom_budget_name(name) {
                    return Err(configuration_error(
                        "invalid custom budget key",
                        "Custom budget keys must be valid, namespaced ASCII identifiers.",
                    ));
                }
            }
        }

        Ok(())
    }
}

fn configuration_error(message: &str, note: &str) -> ZuirError {
    ZuirError::new(ZuirErrorKind::ConfigurationInvalid, message)
        .with_phase(ErrorPhase::Configuration)
        .with_note(note)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_do_not_impose_explicit_work_limits() {
        let config = ZuirConfig::new();

        assert_eq!(config.verification_policy(), VerificationPolicy::Standard);
        assert_eq!(config.compatibility_policy(), CompatibilityPolicy::Exact);
        assert_eq!(config.extension_policy(), ExtensionPolicy::StableOnly);
        assert!(config.deterministic());

        for kind in [
            BudgetKind::InputBytes,
            BudgetKind::IrEntities,
            BudgetKind::NestingDepth,
            BudgetKind::PassApplications,
            BudgetKind::ConstantEvaluationWork,
            BudgetKind::Diagnostics,
            BudgetKind::SerializedBytes,
            BudgetKind::MetadataEntries,
        ] {
            assert!(config.budget(&kind).is_unlimited());
        }
    }

    #[test]
    fn finite_budget_accepts_values_at_or_below_limit() {
        let budget = WorkBudget::bounded(12);

        assert!(budget.permits(0));
        assert!(budget.permits(12));
        assert!(!budget.permits(13));
        assert_eq!(budget.limit(), Some(12));
    }

    #[test]
    fn zero_budget_is_valid_and_permits_no_work() {
        let budget = WorkBudget::bounded(0);

        assert!(budget.permits(0));
        assert!(!budget.permits(1));
    }

    #[test]
    fn unlimited_budget_permits_any_representable_amount() {
        let budget = WorkBudget::unlimited();

        assert!(budget.permits(0));
        assert!(budget.permits(usize::MAX));
    }

    #[test]
    fn budget_check_reports_operational_exhaustion() {
        let error = WorkBudget::bounded(2)
            .check(3, "test work")
            .expect_err("work above the configured budget must fail");

        assert_eq!(error.kind(), ZuirErrorKind::LimitExceeded);
        assert_eq!(error.context().phase(), Some(ErrorPhase::Configuration));
    }

    #[test]
    fn custom_budget_names_must_be_namespaced_and_valid() {
        assert!(BudgetKind::custom("vendor.domain.work").is_ok());
        assert!(BudgetKind::custom("extension_v2.compute.units").is_ok());

        assert!(BudgetKind::custom("").is_err());
        assert!(BudgetKind::custom("work").is_err());
        assert!(BudgetKind::custom(".vendor.work").is_err());
        assert!(BudgetKind::custom("vendor..work").is_err());
        assert!(BudgetKind::custom("vendor.work.").is_err());
        assert!(BudgetKind::custom("Vendor.work").is_err());
        assert!(BudgetKind::custom("vendor.work units").is_err());
    }

    #[test]
    fn configured_budgets_are_retrievable_and_replaceable() {
        let kind = BudgetKind::IrEntities;
        let config = ZuirConfig::new()
            .with_budget(kind.clone(), WorkBudget::bounded(100))
            .expect("built-in budget key is valid");

        assert_eq!(config.budget(&kind).limit(), Some(100));

        let replaced = config
            .with_budget(kind.clone(), WorkBudget::bounded(200))
            .expect("built-in budget key is valid");

        assert_eq!(replaced.budget(&kind).limit(), Some(200));
    }

    #[test]
    fn custom_budget_can_be_added_and_removed() {
        let kind = BudgetKind::custom("example.domain.units")
            .expect("valid extension budget name");

        let config = ZuirConfig::new()
            .with_budget(kind.clone(), WorkBudget::bounded(50))
            .expect("valid custom budget");

        assert_eq!(config.budget(&kind).limit(), Some(50));

        let removed = config.without_budget(&kind);
        assert!(removed.budget(&kind).is_unlimited());
    }

    #[test]
    fn invalid_custom_key_cannot_enter_through_public_variant() {
        let config = ZuirConfig::new()
            .with_budget(
                BudgetKind::Custom("invalid key".to_owned()),
                WorkBudget::bounded(10),
            );

        assert!(config.is_err());
    }

    #[test]
    fn builder_methods_preserve_unrelated_settings() {
        let config = ZuirConfig::new()
            .with_verification_policy(VerificationPolicy::Thorough)
            .with_compatibility_policy(CompatibilityPolicy::Compatible)
            .with_extension_policy(ExtensionPolicy::AllowExperimental)
            .with_diagnostic_limit_policy(DiagnosticLimitPolicy::Fail);

        assert_eq!(config.verification_policy(), VerificationPolicy::Thorough);
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
        assert!(config.deterministic());
        assert!(config.validate().is_ok());
    }

    #[test]
    fn budget_iteration_is_ordered_by_key() {
        let config = ZuirConfig::new()
            .with_budget(BudgetKind::SerializedBytes, WorkBudget::bounded(500))
            .expect("valid built-in key")
            .with_budget(BudgetKind::InputBytes, WorkBudget::bounded(100))
            .expect("valid built-in key");

        let keys: Vec<&str> = config
            .configured_budgets()
            .map(|(kind, _)| kind.as_str())
            .collect();

        let mut sorted = keys.clone();
        sorted.sort_unstable();

        assert_eq!(keys, sorted);
    }
}
