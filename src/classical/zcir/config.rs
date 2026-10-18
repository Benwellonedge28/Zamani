
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # ZCIR configuration and operational policy
//!
//! Path: `src/classical/zcir/config.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Unsafe Rust is forbidden.
//!
//! ## Purpose
//!
//! Define target-independent, deterministic configuration for ZCIR construction,
//! verification, compatibility, extension participation, diagnostics, and
//! explicitly budgeted compiler work.
//!
//! ## Authority
//!
//! This module follows the architecture established by:
//!
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/poco-reaf.md`
//! - `grammar/validation/semantic-boundaries.md`
//! - `src/zuir/config.rs`, where its contracts are applicable
//!
//! Grammar and semantic specifications define program meaning. This module
//! defines implementation policy and must never redefine that meaning.
//!
//! ## Owns
//!
//! - `ZcirConfig`, its defaults, and its validation.
//! - Verification, compatibility, extension, and diagnostic policies.
//! - Explicit optional work budgets and extensible budget dimensions.
//! - Configuration-specific errors.
//!
//! ## Does not own
//!
//! - AST, semantic types, IR nodes, operation definitions, or identity allocation.
//! - The canonical ZUIR configuration or universal IR implementation.
//! - Actual work accounting, memory allocation, or budget enforcement loops.
//! - Source maps, diagnostic rendering, provenance records, or serialization.
//! - Target discovery, device selection, scheduling, placement, or execution.
//! - Language-version or target-capability policy.
//!
//! ## Scalability
//!
//! An absent budget means that this configuration imposes no explicit limit
//! for that dimension. It does not guarantee infinite execution or memory.
//! Actual process, platform, security, and target constraints remain applicable.
//!
//! No universal ceiling is defined for operations, blocks, functions, values,
//! nesting, workers, devices, memory, tensor rank, or machine size.
//!
//! A configured budget is an operational policy, not a language restriction.
//! Its enforcing subsystem must report exhaustion explicitly and must not
//! silently truncate input or change program semantics.
//!
//! ## Determinism
//!
//! This module performs no I/O, environment inspection, hardware discovery,
//! clock reads, random generation, or global mutation. Budget entries are
//! stored in an ordered map. Identical configuration inputs produce identical
//! configuration values and validation results.
//!
//! ## Integration contract
//!
//! - `zcir/mod.rs`: declare `pub mod config;` after the module is implemented.
//! - `zcir/api.rs`: validate configuration at public operation boundaries.
//! - Builders: use the relevant budgets when constructing IR.
//! - Verifiers: apply verification and work policies without weakening invariants.
//! - Pass managers: account for pass applications and analysis work.
//! - Diagnostics: enforce retention budgets and report omitted diagnostics.
//! - Serialization: enforce byte budgets at the encoding/decoding boundary.
//! - Lowering adapters: preserve configuration-independent program meaning.
//! - Target planning: assess actual target capabilities and resources separately.
//! - Incremental compilation: include all result-affecting configuration in
//!   cache identity; do not hash only a subset of these fields.
//!
//! This module deliberately has no dependency on other proposed ZCIR files.
//! Consumers must reuse these definitions instead of introducing duplicate
//! ZCIR policy enums or work-budget types.
//!
//! ## Freeze rule
//!
//! This file is ready to freeze when its public contract is accepted, its
//! tests pass in the actual repository, and all consumers use its policy
//! definitions consistently. Repository-wide integration and conformance
//! still require independent verification.

use core::fmt;
use std::collections::BTreeMap;
use std::error::Error;

/// Current schema version for a serialized ZCIR configuration.
///
/// This identifies this configuration schema only. It is not the Zamani
/// language version, ZCIR IR version, ZUIR version, or package version.
pub const CONFIG_SCHEMA_VERSION: u32 = 1;

/// Result type for ZCIR configuration operations.
pub type ZcirConfigResult<T> = Result<T, ZcirConfigError>;

/// Policy for when IR verification should be requested.
///
/// None of these variants permits malformed IR to be treated as valid.
/// Externally supplied IR must be verified before a consumer trusts it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub enum VerificationPolicy {
    /// Verify at public boundaries and after transformations that may
    /// affect structural or semantic invariants.
    #[default]
    Standard,

    /// Request additional verification at supported intermediate boundaries.
    Thorough,

    /// Verify at explicitly selected internal boundaries.
    ///
    /// This is not permission to trust unverified external input.
    ExplicitBoundaries,
}

/// Policy for accepting an artifact's declared compatibility version.
///
/// A policy is not proof of compatibility. A compatibility implementation
/// must establish that the specific artifact can be accepted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub enum CompatibilityPolicy {
    /// Accept exact supported versions only.
    #[default]
    Exact,

    /// Also accept versions proven compatible by an explicit contract.
    Compatible,

    /// Permit a documented, available, and validated migration path.
    Migration,
}

/// Policy for extension and dialect participation.
///
/// Only extensions registered and validated by the responsible registry
/// may contribute operations or types. This policy does not register them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub enum ExtensionPolicy {
    /// Allow registered extensions declared stable by their registry.
    #[default]
    StableOnly,

    /// Also allow validated experimental extensions.
    AllowExperimental,

    /// Disallow extension-defined contributions for this operation.
    Disabled,
}

/// Policy for diagnostic retention-budget exhaustion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub enum DiagnosticLimitPolicy {
    /// Continue where safe and report that diagnostics were omitted.
    #[default]
    ReportTruncation,

    /// Fail an operation when its required diagnostic retention cannot
    /// be satisfied.
    Fail,
}

/// Optional upper bound on a specific unit of work.
///
/// `None` means no explicit limit from this budget.
/// `Some(0)` is valid and permits zero units.
///
/// Budgets are checked by consumers. This type does not count work,
/// reserve resources, or enforce limits across concurrent operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct WorkBudget {
    limit: Option<u64>,
}

impl WorkBudget {
    /// Create a budget that imposes no explicit limit.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self { limit: None }
    }

    /// Create a budget with an explicit maximum.
    #[must_use]
    pub const fn bounded(limit: u64) -> Self {
        Self { limit: Some(limit) }
    }

    /// Return the configured maximum, or `None` if unlimited.
    #[must_use]
    pub const fn limit(self) -> Option<u64> {
        self.limit
    }

    /// Whether this budget imposes no explicit limit.
    #[must_use]
    pub const fn is_unlimited(self) -> bool {
        self.limit.is_none()
    }

    /// Whether the supplied amount is within this budget.
    #[must_use]
    pub const fn permits(self, amount: u64) -> bool {
        match self.limit {
            Some(limit) => amount <= limit,
            None => true,
        }
    }

    /// Check a measured amount against this budget.
    ///
    /// The caller must measure cumulative work correctly and invoke this
    /// method at the appropriate enforcement boundary.
    pub fn check(
        self,
        amount: u64,
        description: impl Into<String>,
    ) -> ZcirConfigResult<()> {
        if self.permits(amount) {
            return Ok(());
        }

        Err(ZcirConfigError::BudgetExceeded {
            dimension: description.into(),
            measured: amount,
            limit: self.limit,
        })
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

/// A dimension of operational work that can be budgeted.
///
/// Built-in dimensions are stable names. Custom dimensions allow registered
/// extensions to define their own work units without changing this enum.
///
/// A custom dimension must be namespaced, for example
/// `vendor.extension.work_units`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum BudgetKind {
    /// Input bytes processed by an operation supporting this budget.
    InputBytes,

    /// IR entities processed by an operation supporting this budget.
    IrEntities,

    /// Nesting depth processed by an operation supporting this budget.
    NestingDepth,

    /// Number of pass applications in a transformation pipeline.
    PassApplications,

    /// Work units consumed by constant evaluation.
    ConstantEvaluationWork,

    /// Diagnostics retained by a diagnostic-producing operation.
    Diagnostics,

    /// Serialized or deserialized bytes processed by an operation.
    SerializedBytes,

    /// Entries retained in extensible metadata structures.
    MetadataEntries,

    /// Extension-defined work dimension.
    Custom(String),
}

impl BudgetKind {
    /// Construct a custom namespaced budget dimension.
    ///
    /// Accepted characters are lowercase ASCII letters, digits, `.`, `_`,
    /// and `-`. A key must contain at least one dot. Empty segments and
    /// leading or trailing separators are rejected.
    pub fn custom(name: impl Into<String>) -> ZcirConfigResult<Self> {
        let name = name.into();

        if !valid_custom_budget_name(&name) {
            return Err(ZcirConfigError::InvalidBudgetKind { name });
        }

        Ok(Self::Custom(name))
    }

    /// Return the stable configuration key for this dimension.
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

impl fmt::Display for BudgetKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Configuration-specific errors.
///
/// This type is local to the configuration boundary so this file can be
/// implemented before the proposed ZCIR error module is ready. A later
/// reviewed migration may adapt these variants to the canonical error API;
/// consumers must not maintain two competing public configuration contracts.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ZcirConfigError {
    /// The supplied configuration schema version is unsupported.
    UnsupportedSchemaVersion {
        /// Version supplied by the artifact.
        supplied: u32,
        /// Version supported by this implementation.
        supported: u32,
    },

    /// A custom budget dimension is malformed.
    InvalidBudgetKind {
        /// Invalid key supplied by the caller.
        name: String,
    },

    /// A work budget was exceeded.
    BudgetExceeded {
        /// Stable budget key or human-readable work description.
        dimension: String,
        /// Amount measured by the caller.
        measured: u64,
        /// Configured limit, if any.
        limit: Option<u64>,
    },
}

impl fmt::Display for ZcirConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion { supplied, supported } => write!(
                f,
                "unsupported ZCIR configuration schema version {supplied}; \
                 supported version is {supported}"
            ),
            Self::InvalidBudgetKind { name } => write!(
                f,
                "invalid custom ZCIR budget key {name:?}; expected a \
                 lowercase, namespaced key such as \
                 `vendor.extension.work_units`"
            ),
            Self::BudgetExceeded {
                dimension,
                measured,
                limit,
            } => match limit {
                Some(limit) => write!(
                    f,
                    "ZCIR work budget `{dimension}` exceeded: \
                     measured {measured}, configured limit {limit}"
                ),
                None => write!(
                    f,
                    "ZCIR work budget `{dimension}` was reported exceeded \
                     without a configured limit"
                ),
            },
        }
    }
}

impl Error for ZcirConfigError {}

/// Target-independent configuration for ZCIR operations.
///
/// The configuration is an immutable value from the perspective of its
/// consumers. Construct or modify it through the provided builder-style
/// methods, then pass a shared reference to the operation being performed.
///
/// No target or hardware identity belongs in this type. Requirements,
/// preferences, capabilities, and physical-resource availability are
/// resolved at their respective semantic and realization boundaries.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ZcirConfig {
    schema_version: u32,
    verification: VerificationPolicy,
    compatibility: CompatibilityPolicy,
    extensions: ExtensionPolicy,
    diagnostic_limit_policy: DiagnosticLimitPolicy,
    budgets: BTreeMap<BudgetKind, WorkBudget>,
}

impl Default for ZcirConfig {
    fn default() -> Self {
        Self {
            schema_version: CONFIG_SCHEMA_VERSION,
            verification: VerificationPolicy::Standard,
            compatibility: CompatibilityPolicy::Exact,
            extensions: ExtensionPolicy::StableOnly,
            diagnostic_limit_policy: DiagnosticLimitPolicy::ReportTruncation,
            budgets: BTreeMap::new(),
        }
    }
}

impl ZcirConfig {
    /// Create the default target-independent configuration.
    ///
    /// No explicit work budgets are installed by default. This avoids
    /// turning arbitrary development-machine assumptions into universal
    /// limits. Callers may configure operational budgets explicitly.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Return the configuration schema version.
    #[must_use]
    pub const fn schema_version(&self) -> u32 {
        self.schema_version
    }

    /// Return the verification policy.
    #[must_use]
    pub const fn verification_policy(&self) -> VerificationPolicy {
        self.verification
    }

    /// Return the compatibility policy.
    #[must_use]
    pub const fn compatibility_policy(&self) -> CompatibilityPolicy {
        self.compatibility
    }

    /// Return the extension policy.
    #[must_use]
    pub const fn extension_policy(&self) -> ExtensionPolicy {
        self.extensions
    }

    /// Return the diagnostic retention-limit policy.
    #[must_use]
    pub const fn diagnostic_limit_policy(&self) -> DiagnosticLimitPolicy {
        self.diagnostic_limit_policy
    }

    /// Return a read-only view of all configured budgets.
    #[must_use]
    pub fn budgets(&self) -> &BTreeMap<BudgetKind, WorkBudget> {
        &self.budgets
    }

    /// Return the configured budget for a dimension.
    ///
    /// An absent entry means that this configuration imposes no explicit
    /// limit for the requested dimension.
    #[must_use]
    pub fn budget(&self, kind: &BudgetKind) -> WorkBudget {
        self.budgets
            .get(kind)
            .copied()
            .unwrap_or_else(WorkBudget::unlimited)
    }

    /// Set the verification policy.
    #[must_use]
    pub fn with_verification_policy(mut self, policy: VerificationPolicy) -> Self {
        self.verification = policy;
        self
    }

    /// Set the compatibility policy.
    #[must_use]
    pub fn with_compatibility_policy(mut self, policy: CompatibilityPolicy) -> Self {
        self.compatibility = policy;
        self
    }

    /// Set the extension policy.
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

    /// Set or replace the budget for a dimension.
    ///
    /// Setting an unlimited budget removes any explicit limit for that
    /// dimension. No work is performed and no resources are reserved.
    #[must_use]
    pub fn with_budget(mut self, kind: BudgetKind, budget: WorkBudget) -> Self {
        if budget.is_unlimited() {
            self.budgets.remove(&kind);
        } else {
            self.budgets.insert(kind, budget);
        }
        self
    }

    /// Remove an explicit budget, restoring the unlimited policy for it.
    #[must_use]
    pub fn without_budget(mut self, kind: &BudgetKind) -> Self {
        self.budgets.remove(kind);
        self
    }

    /// Validate this configuration before using it at an API boundary.
    ///
    /// This validates the configuration schema and budget-key invariants.
    /// It does not validate IR, target feasibility, registered extensions,
    /// or compatibility of an external artifact.
    pub fn validate(&self) -> ZcirConfigResult<()> {
        if self.schema_version != CONFIG_SCHEMA_VERSION {
            return Err(ZcirConfigError::UnsupportedSchemaVersion {
                supplied: self.schema_version,
                supported: CONFIG_SCHEMA_VERSION,
            });
        }

        for kind in self.budgets.keys() {
            if let BudgetKind::Custom(name) = kind {
                if !valid_custom_budget_name(name) {
                    return Err(ZcirConfigError::InvalidBudgetKind {
                        name: name.clone(),
                    });
                }
            }
        }

        Ok(())
    }

    /// Return whether a measured amount is permitted by a configured budget.
    ///
    /// This method performs a pure policy check. It does not accumulate
    /// amounts across calls; the enforcing consumer owns accounting.
    #[must_use]
    pub fn permits(&self, kind: &BudgetKind, amount: u64) -> bool {
        self.budget(kind).permits(amount)
    }

    /// Check an amount against the configured budget for a dimension.
    pub fn check_budget(
        &self,
        kind: &BudgetKind,
        amount: u64,
    ) -> ZcirConfigResult<()> {
        self.budget(kind).check(amount, kind.as_str())
    }
}

fn valid_custom_budget_name(name: &str) -> bool {
    if name.is_empty() || !name.contains('.') {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_configuration_is_valid_and_has_no_explicit_budgets() {
        let config = ZcirConfig::default();

        assert_eq!(config.schema_version(), CONFIG_SCHEMA_VERSION);
        assert_eq!(config.verification_policy(), VerificationPolicy::Standard);
        assert_eq!(config.compatibility_policy(), CompatibilityPolicy::Exact);
        assert_eq!(config.extension_policy(), ExtensionPolicy::StableOnly);
        assert_eq!(
            config.diagnostic_limit_policy(),
            DiagnosticLimitPolicy::ReportTruncation
        );
        assert!(config.budgets().is_empty());
        assert!(config.validate().is_ok());
    }

    #[test]
    fn absent_budget_does_not_impose_a_limit() {
        let config = ZcirConfig::new();
        let kind = BudgetKind::IrEntities;

        assert!(config.budget(&kind).is_unlimited());
        assert!(config.permits(&kind, 0));
        assert!(config.permits(&kind, u64::MAX));
        assert!(config.check_budget(&kind, u64::MAX).is_ok());
    }

    #[test]
    fn bounded_budget_accepts_the_limit_and_rejects_excess() {
        let kind = BudgetKind::IrEntities;
        let config = ZcirConfig::new().with_budget(
            kind.clone(),
            WorkBudget::bounded(12),
        );

        assert!(config.permits(&kind, 0));
        assert!(config.permits(&kind, 12));
        assert!(!config.permits(&kind, 13));
        assert!(config.check_budget(&kind, 12).is_ok());

        let error = config.check_budget(&kind, 13).unwrap_err();
        assert!(matches!(
            error,
            ZcirConfigError::BudgetExceeded {
                measured: 13,
                limit: Some(12),
                ..
            }
        ));
    }

    #[test]
    fn zero_budget_is_valid_and_permits_zero_work() {
        let kind = BudgetKind::PassApplications;
        let config = ZcirConfig::new()
            .with_budget(kind.clone(), WorkBudget::bounded(0));

        assert!(config.permits(&kind, 0));
        assert!(!config.permits(&kind, 1));
        assert!(config.validate().is_ok());
    }

    #[test]
    fn unlimited_budget_removes_an_existing_explicit_limit() {
        let kind = BudgetKind::SerializedBytes;
        let config = ZcirConfig::new()
            .with_budget(kind.clone(), WorkBudget::bounded(64))
            .with_budget(kind.clone(), WorkBudget::unlimited());

        assert!(config.budgets().is_empty());
        assert!(config.permits(&kind, u64::MAX));
    }

    #[test]
    fn without_budget_restores_unlimited_behavior() {
        let kind = BudgetKind::InputBytes;
        let config = ZcirConfig::new()
            .with_budget(kind.clone(), WorkBudget::bounded(128))
            .without_budget(&kind);

        assert!(config.budget(&kind).is_unlimited());
        assert!(config.permits(&kind, u64::MAX));
    }

    #[test]
    fn custom_budget_names_are_validated() {
        assert!(BudgetKind::custom("vendor.extension.work_units").is_ok());
        assert!(BudgetKind::custom("org.example-pass.work_units").is_ok());

        for invalid in [
            "",
            "plain",
            ".vendor.work",
            "vendor.",
            "vendor..work",
            "Vendor.extension.work",
            "vendor.extension/work",
            "vendor extension.work",
        ] {
            assert!(
                BudgetKind::custom(invalid).is_err(),
                "expected {invalid:?} to be rejected"
            );
        }
    }

    #[test]
    fn custom_budget_can_be_configured_and_checked() {
        let kind = BudgetKind::custom("vendor.extension.work_units").unwrap();
        let config = ZcirConfig::new()
            .with_budget(kind.clone(), WorkBudget::bounded(7));

        assert_eq!(config.budget(&kind).limit(), Some(7));
        assert!(config.check_budget(&kind, 7).is_ok());
        assert!(config.check_budget(&kind, 8).is_err());
        assert!(config.validate().is_ok());
    }

    #[test]
    fn policies_can_be_composed_without_mutable_global_state() {
        let config = ZcirConfig::new()
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
        assert_eq!(config.diagnostic_limit_policy(), DiagnosticLimitPolicy::Fail);
        assert!(config.validate().is_ok());
    }

    #[test]
    fn configuration_clone_and_equality_are_deterministic() {
        let kind = BudgetKind::MetadataEntries;
        let config = ZcirConfig::new()
            .with_budget(kind, WorkBudget::bounded(512));

        assert_eq!(config, config.clone());
    }

    #[test]
    fn budget_order_is_stable() {
        let config = ZcirConfig::new()
            .with_budget(BudgetKind::SerializedBytes, WorkBudget::bounded(10))
            .with_budget(BudgetKind::InputBytes, WorkBudget::bounded(20));

        let keys: Vec<&str> = config
            .budgets()
            .keys()
            .map(BudgetKind::as_str)
            .collect();

        let mut sorted = keys.clone();
        sorted.sort_unstable();

        assert_eq!(keys, sorted);
    }

    #[test]
    fn work_budget_display_is_unambiguous() {
        assert_eq!(WorkBudget::unlimited().to_string(), "unlimited");
        assert_eq!(WorkBudget::bounded(42).to_string(), "bounded(42)");
    }

    #[test]
    fn error_display_does_not_require_parsing_for_internal_control_flow() {
        let error = ZcirConfigError::UnsupportedSchemaVersion {
            supplied: 9,
            supported: CONFIG_SCHEMA_VERSION,
        };

        assert!(error.to_string().contains("9"));
        assert!(error.to_string().contains(&CONFIG_SCHEMA_VERSION.to_string()));
    }
}
