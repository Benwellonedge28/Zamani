//! Zamani Classical Computing — Configuration Contract
//!
//! Path: src/classical/config.rs
//!
//! Status: Contract-first implementation; integration pending.
//! Language: Zamani
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Defines target-independent configuration for classical computation,
//! numerical semantics, optimization policy, reproducibility, verification,
//! parallelism, and explicit operational resource budgets.
//!
//! This module defines policy, not the algorithms that enforce that policy.
//!
//! # Architectural ownership
//!
//! This file exclusively owns:
//!
//! - ClassicalConfig and its configuration subtypes.
//! - Configuration validation and structured configuration errors.
//! - Stable configuration policy enums.
//! - Explicit, optional operational budgets.
//! - Validation of extensible optimization-pass identifiers.
//!
//! This file does not own:
//!
//! - Source syntax, parsing, AST construction, or source spans.
//! - Canonical classical values, types, identities, or IR.
//! - Hardware discovery, target capabilities, or physical placement.
//! - Optimization passes, compiler pipelines, or scheduling algorithms.
//! - Runtime allocation, operating-system resources, or execution.
//! - Diagnostic rendering or the repository-wide error-reporting system.
//!
//! # POCO-REAF and scalability
//!
//! No universal maximum is imposed on processor counts, memory, vector
//! widths, tensor rank, array sizes, operations, tasks, devices, or nodes.
//!
//! An absent budget means that this configuration imposes no limit for that
//! budget. It does not promise infinite resources: the host, target, security
//! policy, or another subsystem may still impose real limits.
//!
//! A configured budget is an operational policy, not a language-semantic
//! ceiling. The subsystem enforcing a budget must report exhaustion explicitly
//! and must never silently truncate data or change program meaning.
//!
//! Counts and byte quantities are represented using u64 for configuration
//! portability. A consumer must use checked conversions when mapping a value
//! into a narrower host representation.
//!
//! # Integration contract
//!
//! The intended dependency direction is:
//!
//! ```text
//! configuration / shared policy
//!              |
//!              v
//!      classical::config
//!              |
//!      +-------+--------+
//!      |       |        |
//!   analysis optimizer execution
//!      |       |        |
//!      +-------+--------+
//!              |
//!       canonical IR / runtime
//! ```
//!
//! Integration responsibilities:
//!
//! - `src/classical/mod.rs`: declare `pub mod config;` and selectively
//!   re-export `ClassicalConfig` if that matches the final public API.
//! - `src/lib.rs`: expose `pub mod classical;` only after the classical
//!   module tree is ready to compile as a whole.
//! - `src/classical/optimization/`: resolve optimization levels and pass
//!   selection into actual registered passes.
//! - `src/classical/numeric/`: implement numerical policies without
//!   redefining the configuration enums.
//! - `src/classical/concurrency/`: interpret parallelism policy.
//! - `src/classical/execution/`: combine these budgets with available
//!   resources and target capabilities.
//! - `src/classical/validation/`: validate program requirements against
//!   the applicable policies and target information.
//! - `src/classical/lowering/`: preserve program semantics while lowering.
//! - Shared diagnostics infrastructure: adapt `ClassicalConfigError` to
//!   repository-wide diagnostics when that interface is established.
//!
//! Consumers must not duplicate these types. If a shared configuration,
//! resource-budget, or numerical-policy type already becomes authoritative,
//! a reviewed compatibility migration must replace this definition with
//! a re-export rather than maintain two competing contracts.
//!
//! # Configuration stability
//!
//! Configuration is validated explicitly. Public fields make the policy
//! inspectable and straightforward to serialize, but callers must validate
//! configurations received from untrusted files or external APIs.
//!
//! Adding a new optional field should preserve the default behavior of older
//! serialized configurations. Changes to enum meanings or default semantics
//! require compatibility review.
//!
//! # Safety
//!
//! This module forbids unsafe Rust.
//! It performs no I/O, resource discovery, allocation on behalf of the
//! runtime, or execution of user programs.

#![forbid(unsafe_code)]

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Current version of the serialized classical configuration schema.
///
/// This versions the configuration representation, not the Zamani language.
pub const CONFIG_SCHEMA_VERSION: u32 = 1;

/// Result type for classical configuration operations.
pub type ClassicalConfigResult<T> = Result<T, ClassicalConfigError>;

/// Structured failures produced while validating classical configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClassicalConfigError {
    /// The configuration schema version is not supported.
    UnsupportedSchemaVersion {
        /// Version supplied by the configuration.
        supplied: u32,
        /// Version supported by this implementation.
        supported: u32,
    },

    /// A numeric operational budget must be greater than zero when present.
    ZeroBudget {
        /// Name of the invalid field.
        field: &'static str,
    },

    /// A configured worker limit must be greater than zero.
    ZeroWorkerLimit,

    /// An optimization pass identifier is empty or malformed.
    InvalidPassIdentifier {
        /// Identifier supplied by the caller.
        identifier: String,
    },

    /// A pass cannot be both explicitly enabled and explicitly disabled.
    ConflictingPassPolicy {
        /// Identifier present in both sets.
        identifier: String,
    },
}

impl fmt::Display for ClassicalConfigError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion {
                supplied,
                supported,
            } => write!(
                formatter,
                "unsupported classical configuration schema version \
                 {supplied}; this implementation supports version {supported}"
            ),

            Self::ZeroBudget { field } => write!(
                formatter,
                "classical configuration field `{field}` must be greater \
                 than zero when a budget is specified"
            ),

            Self::ZeroWorkerLimit => {
                write!(formatter, "parallel worker limit must be greater than zero")
            }

            Self::InvalidPassIdentifier { identifier } => write!(
                formatter,
                "invalid optimization pass identifier `{identifier}`"
            ),

            Self::ConflictingPassPolicy { identifier } => write!(
                formatter,
                "optimization pass `{identifier}` is both enabled and disabled"
            ),
        }
    }
}

impl Error for ClassicalConfigError {}

/// Selects the broad optimization policy.
///
/// These levels describe intent. They do not directly enumerate or execute
/// optimization passes; the optimization subsystem resolves them using its
/// registered pass set, legality checks, and target-independent analyses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OptimizationLevel {
    /// Preserve the input representation except for mandatory correctness
    /// transformations and validation.
    None,

    /// Apply inexpensive, conservative transformations.
    Basic,

    /// Apply the implementation's standard legal optimization pipeline.
    Standard,

    /// Permit additional legal transformations when their preconditions hold.
    Aggressive,

    /// Prefer smaller representations or lower code size where applicable.
    Size,

    /// Prefer transformations compatible with the configured reproducibility
    /// and determinism requirements.
    Deterministic,
}

impl Default for OptimizationLevel {
    fn default() -> Self {
        Self::Standard
    }
}

/// Explicit selection overrides for extensible optimization passes.
///
/// Identifiers are stable, namespaced strings owned by the optimization
/// registry. This type deliberately does not contain a closed enum of passes:
/// future passes can be registered without editing this configuration file.
///
/// An empty `enabled` set means that the selected optimization level determines
/// the normal pass set. Entries in `disabled` always exclude those passes.
/// Explicitly enabled and disabled sets must not overlap.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PassSelection {
    /// Passes explicitly requested by the caller.
    pub enabled: BTreeSet<String>,

    /// Passes explicitly excluded by the caller.
    pub disabled: BTreeSet<String>,
}

impl PassSelection {
    /// Constructs an empty selection, leaving pass choice to the profile.
    pub fn new() -> Self {
        Self::default()
    }

    /// Explicitly enables a pass.
    ///
    /// Pass names are validated by [`ClassicalConfig::validate`].
    pub fn enable(&mut self, identifier: impl Into<String>) {
        self.enabled.insert(identifier.into());
    }

    /// Explicitly disables a pass.
    ///
    /// Pass names are validated by [`ClassicalConfig::validate`].
    pub fn disable(&mut self, identifier: impl Into<String>) {
        self.disabled.insert(identifier.into());
    }

    /// Returns whether a pass has an explicit enable override.
    pub fn is_enabled_explicitly(&self, identifier: &str) -> bool {
        self.enabled.contains(identifier)
    }

    /// Returns whether a pass has an explicit disable override.
    pub fn is_disabled_explicitly(&self, identifier: &str) -> bool {
        self.disabled.contains(identifier)
    }
}

/// Optimization policy, separate from transformation implementation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct OptimizationPolicy {
    /// Requested optimization level.
    pub level: OptimizationLevel,

    /// Explicit pass inclusion and exclusion overrides.
    pub passes: PassSelection,

    /// Request verification after each pass where the optimizer supports it.
    pub verify_after_each_pass: bool,

    /// Request preservation of available source/debug metadata.
    pub preserve_debug_information: bool,
}

impl Default for OptimizationPolicy {
    fn default() -> Self {
        Self {
            level: OptimizationLevel::Standard,
            passes: PassSelection::default(),
            verify_after_each_pass: true,
            preserve_debug_information: true,
        }
    }
}

/// Defines the behavior required when integer arithmetic overflows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IntegerOverflowPolicy {
    /// Report overflow according to the language's checked-arithmetic
    /// semantics. The consumer must not silently wrap or truncate.
    Checked,

    /// Permit wrapping only when the language operation explicitly specifies
    /// wrapping semantics.
    ExplicitWrapping,

    /// Require a representation capable of preserving the mathematical result
    /// where the implementation supports it; otherwise report a structured
    /// unsupported-representation or resource failure.
    ExpandWhenSupported,
}

impl Default for IntegerOverflowPolicy {
    fn default() -> Self {
        Self::Checked
    }
}

/// Specifies the floating-point semantic contract.
///
/// This setting does not itself implement floating-point arithmetic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FloatSemantics {
    /// Follow the floating-point semantics specified by the language and
    /// the selected representation.
    LanguageDefined,

    /// Require strict behavior and prohibit transformations that would violate
    /// the specified observable floating-point behavior.
    Strict,

    /// Permit relaxed transformations only when the language contract,
    /// explicit program annotations, and applicable policies allow them.
    Relaxed,
}

impl Default for FloatSemantics {
    fn default() -> Self {
        Self::LanguageDefined
    }
}

/// Specifies the reproducibility requirements for numerical computation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NumericalReproducibility {
    /// Use the semantics required by the program and language specification.
    LanguageDefined,

    /// Require stable operation ordering where the implementation can
    /// guarantee it.
    Deterministic,

    /// Require a documented reproducibility contract across supported
    /// implementations and targets; reject a realization that cannot satisfy
    /// that contract.
    CrossTarget,
}

impl Default for NumericalReproducibility {
    fn default() -> Self {
        Self::LanguageDefined
    }
}

/// Numerical behavior policy for classical computations.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct NumericalPolicy {
    /// Integer overflow behavior.
    pub integer_overflow: IntegerOverflowPolicy,

    /// Floating-point transformation and evaluation contract.
    pub float_semantics: FloatSemantics,

    /// Numerical reproducibility requirement.
    pub reproducibility: NumericalReproducibility,

    /// Whether floating-point contraction may be considered.
    ///
    /// This is permission, not a command. The optimizer must still prove that
    /// a transformation is legal under the selected floating-point semantics.
    pub allow_contraction: bool,
}

impl Default for NumericalPolicy {
    fn default() -> Self {
        Self {
            integer_overflow: IntegerOverflowPolicy::Checked,
            float_semantics: FloatSemantics::LanguageDefined,
            reproducibility: NumericalReproducibility::LanguageDefined,
            allow_contraction: false,
        }
    }
}

/// Requested parallel execution strategy.
///
/// This describes policy only. It does not discover processors, create
/// threads, or guarantee that a target can execute in parallel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "mode", content = "workers")]
pub enum ParallelismPolicy {
    /// Let the execution planner choose parallelism from dependencies,
    /// capabilities, workload characteristics, and available resources.
    Auto,

    /// Prefer sequential execution and do not request parallel expansion.
    Sequential,

    /// Permit no more than the specified number of workers for this policy.
    ///
    /// This is an explicit user-selected operational bound, not a universal
    /// hardware ceiling. A value of zero is rejected by validation.
    Limit(u64),
}

impl Default for ParallelismPolicy {
    fn default() -> Self {
        Self::Auto
    }
}

/// Controls verification requests for classical transformations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VerificationPolicy {
    /// Perform mandatory structural and semantic correctness checks.
    Required,

    /// Request additional verification when supported by the implementation.
    Enhanced,

    /// Require the implementation to report whether a requested verification
    /// could not be performed. Mandatory safety and validity checks remain
    /// mandatory.
    ReportUnavailable,
}

impl Default for VerificationPolicy {
    fn default() -> Self {
        Self::Required
    }
}

/// Controls the determinism expected from configuration-driven processing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeterminismPolicy {
    /// Preserve language-defined behavior; do not add stronger guarantees.
    LanguageDefined,

    /// Require deterministic decisions and output wherever the implementation
    /// can provide that guarantee.
    Deterministic,

    /// Require deterministic decisions and canonical output across supported
    /// implementations under the same documented inputs and environment.
    CrossImplementation,
}

impl Default for DeterminismPolicy {
    fn default() -> Self {
        Self::LanguageDefined
    }
}

/// Optional operational budgets for a classical processing operation.
///
/// `None` means no limit is imposed by this configuration field. Other
/// policies, the host, or the target may still impose constraints.
///
/// A budget consumer must account for its own units precisely and report
/// exhaustion. These fields never define the maximum size of a valid Zamani
/// program or data structure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ResourceBudget {
    /// Maximum elapsed processing time, in milliseconds, where the consumer
    /// can enforce an elapsed-time budget.
    pub max_elapsed_time_ms: Option<u64>,

    /// Maximum memory budget in bytes for the operation, where measurable and
    /// enforceable by the consuming subsystem.
    pub max_memory_bytes: Option<u64>,

    /// Maximum accounted work units. The consumer must document its unit.
    pub max_work_units: Option<u64>,

    /// Maximum evaluation steps for consumers that implement step accounting.
    pub max_evaluation_steps: Option<u64>,

    /// Maximum rewrite applications for transformation engines.
    pub max_rewrite_steps: Option<u64>,

    /// Maximum diagnostic records retained for this operation.
    pub max_diagnostics: Option<u64>,

    /// Maximum concurrently active tasks requested by the operation.
    pub max_parallel_tasks: Option<u64>,
}

impl ResourceBudget {
    /// Returns a budget with no configuration-imposed limits.
    ///
    /// This does not bypass host, target, or security restrictions.
    pub const fn unbounded() -> Self {
        Self {
            max_elapsed_time_ms: None,
            max_memory_bytes: None,
            max_work_units: None,
            max_evaluation_steps: None,
            max_rewrite_steps: None,
            max_diagnostics: None,
            max_parallel_tasks: None,
        }
    }

    /// Validates all configured budgets.
    pub fn validate(&self) -> ClassicalConfigResult<()> {
        let fields = [
            ("max_elapsed_time_ms", self.max_elapsed_time_ms),
            ("max_memory_bytes", self.max_memory_bytes),
            ("max_work_units", self.max_work_units),
            ("max_evaluation_steps", self.max_evaluation_steps),
            ("max_rewrite_steps", self.max_rewrite_steps),
            ("max_diagnostics", self.max_diagnostics),
            ("max_parallel_tasks", self.max_parallel_tasks),
        ];

        for (field, value) in fields {
            if value == Some(0) {
                return Err(ClassicalConfigError::ZeroBudget { field });
            }
        }

        Ok(())
    }
}

/// Complete target-independent configuration for classical computing.
///
/// This is an immutable-by-convention policy value: consumers should clone or
/// borrow it for an operation rather than mutate global configuration while
/// that operation is running.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ClassicalConfig {
    /// Serialized configuration schema version.
    pub schema_version: u32,

    /// Optimization policy.
    pub optimization: OptimizationPolicy,

    /// Numerical semantics policy.
    pub numerical: NumericalPolicy,

    /// Parallel execution policy.
    pub parallelism: ParallelismPolicy,

    /// Determinism requirement.
    pub determinism: DeterminismPolicy,

    /// Verification policy.
    pub verification: VerificationPolicy,

    /// Optional operational budgets.
    pub budgets: ResourceBudget,
}

impl Default for ClassicalConfig {
    fn default() -> Self {
        Self {
            schema_version: CONFIG_SCHEMA_VERSION,
            optimization: OptimizationPolicy::default(),
            numerical: NumericalPolicy::default(),
            parallelism: ParallelismPolicy::default(),
            determinism: DeterminismPolicy::default(),
            verification: VerificationPolicy::default(),
            budgets: ResourceBudget::default(),
        }
    }
}

impl ClassicalConfig {
    /// Creates the default configuration.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a configuration with optimization disabled.
    ///
    /// Mandatory validation and correctness checks remain enabled.
    pub fn without_optimizations() -> Self {
        Self {
            optimization: OptimizationPolicy {
                level: OptimizationLevel::None,
                passes: PassSelection::default(),
                verify_after_each_pass: true,
                preserve_debug_information: true,
            },
            ..Self::default()
        }
    }

    /// Validates the complete configuration.
    ///
    /// Call this after deserializing configuration from a file, network
    /// request, environment, or another untrusted source, and before passing
    /// it to a compiler or execution subsystem.
    pub fn validate(&self) -> ClassicalConfigResult<()> {
        if self.schema_version != CONFIG_SCHEMA_VERSION {
            return Err(ClassicalConfigError::UnsupportedSchemaVersion {
                supplied: self.schema_version,
                supported: CONFIG_SCHEMA_VERSION,
            });
        }

        if let ParallelismPolicy::Limit(0) = self.parallelism {
            return Err(ClassicalConfigError::ZeroWorkerLimit);
        }

        self.budgets.validate()?;
        self.validate_pass_selection()?;

        Ok(())
    }

    /// Returns whether the configuration requests no optimization passes
    /// beyond mandatory correctness processing.
    pub fn optimization_disabled(&self) -> bool {
        self.optimization.level == OptimizationLevel::None
            && self.optimization.passes.enabled.is_empty()
    }

    /// Validates extensible pass identifiers and checks for contradictory
    /// inclusion/exclusion overrides.
    fn validate_pass_selection(&self) -> ClassicalConfigResult<()> {
        for identifier in self
            .optimization
            .passes
            .enabled
            .iter()
            .chain(self.optimization.passes.disabled.iter())
        {
            if !is_valid_pass_identifier(identifier) {
                return Err(ClassicalConfigError::InvalidPassIdentifier {
                    identifier: identifier.clone(),
                });
            }
        }

        if let Some(identifier) = self
            .optimization
            .passes
            .enabled
            .intersection(&self.optimization.passes.disabled)
            .next()
        {
            return Err(ClassicalConfigError::ConflictingPassPolicy {
                identifier: identifier.clone(),
            });
        }

        Ok(())
    }
}

/// Returns whether an identifier follows the stable pass-ID character policy.
///
/// Pass IDs are non-empty ASCII strings containing only letters, digits,
/// underscores, hyphens, periods, colons, or forward slashes. Namespacing is
/// permitted; the identifier registry remains responsible for determining
/// whether a syntactically valid ID names an implemented pass.
fn is_valid_pass_identifier(identifier: &str) -> bool {
    !identifier.is_empty()
        && identifier.bytes().all(|byte| {
            byte.is_ascii_alphanumeric()
                || matches!(byte, b'_' | b'-' | b'.' | b':' | b'/')
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_configuration_is_valid() {
        assert!(ClassicalConfig::default().validate().is_ok());
    }

    #[test]
    fn no_optimization_profile_is_valid() {
        let config = ClassicalConfig::without_optimizations();

        assert!(config.validate().is_ok());
        assert!(config.optimization_disabled());
    }

    #[test]
    fn absent_budgets_do_not_impose_configuration_limits() {
        let budget = ResourceBudget::unbounded();

        assert!(budget.validate().is_ok());
        assert_eq!(budget.max_memory_bytes, None);
        assert_eq!(budget.max_work_units, None);
    }

    #[test]
    fn zero_budget_is_rejected() {
        let budget = ResourceBudget {
            max_work_units: Some(0),
            ..ResourceBudget::default()
        };

        assert_eq!(
            budget.validate(),
            Err(ClassicalConfigError::ZeroBudget {
                field: "max_work_units"
            })
        );
    }

    #[test]
    fn zero_worker_limit_is_rejected() {
        let config = ClassicalConfig {
            parallelism: ParallelismPolicy::Limit(0),
            ..ClassicalConfig::default()
        };

        assert_eq!(
            config.validate(),
            Err(ClassicalConfigError::ZeroWorkerLimit)
        );
    }

    #[test]
    fn conflicting_pass_overrides_are_rejected() {
        let mut selection = PassSelection::new();
        selection.enable("classical.constant-fold");
        selection.disable("classical.constant-fold");

        let config = ClassicalConfig {
            optimization: OptimizationPolicy {
                passes: selection,
                ..OptimizationPolicy::default()
            },
            ..ClassicalConfig::default()
        };

        assert_eq!(
            config.validate(),
            Err(ClassicalConfigError::ConflictingPassPolicy {
                identifier: "classical.constant-fold".to_owned()
            })
        );
    }

    #[test]
    fn malformed_pass_identifiers_are_rejected() {
        let mut selection = PassSelection::new();
        selection.enable("classical pass with spaces");

        let config = ClassicalConfig {
            optimization: OptimizationPolicy {
                passes: selection,
                ..OptimizationPolicy::default()
            },
            ..ClassicalConfig::default()
        };

        assert!(matches!(
            config.validate(),
            Err(ClassicalConfigError::InvalidPassIdentifier { .. })
        ));
    }

    #[test]
    fn unknown_schema_version_is_rejected() {
        let config = ClassicalConfig {
            schema_version: CONFIG_SCHEMA_VERSION + 1,
            ..ClassicalConfig::default()
        };

        assert!(matches!(
            config.validate(),
            Err(ClassicalConfigError::UnsupportedSchemaVersion { .. })
        ));
    }

    #[test]
    fn configuration_round_trips_through_json() {
        let config = ClassicalConfig {
            parallelism: ParallelismPolicy::Limit(3),
            budgets: ResourceBudget {
                max_memory_bytes: Some(16 * 1024),
                max_work_units: Some(500_000),
                ..ResourceBudget::default()
            },
            ..ClassicalConfig::default()
        };

        let encoded = serde_json::to_string(&config)
            .expect("configuration should serialize");
        let decoded: ClassicalConfig = serde_json::from_str(&encoded)
            .expect("configuration should deserialize");

        assert_eq!(decoded, config);
        assert!(decoded.validate().is_ok());
    }

    #[test]
    fn large_explicit_budgets_are_not_rejected_as_architectural_limits() {
        let config = ClassicalConfig {
            budgets: ResourceBudget {
                max_memory_bytes: Some(u64::MAX),
                max_work_units: Some(u64::MAX),
                ..ResourceBudget::default()
            },
            ..ClassicalConfig::default()
        };

        assert!(config.validate().is_ok());
    }

    #[test]
    fn deterministic_defaults_are_stable() {
        let first = ClassicalConfig::default();
        let second = ClassicalConfig::default();

        assert_eq!(first, second);
        assert_eq!(first.determinism, DeterminismPolicy::LanguageDefined);
        assert_eq!(
            first.numerical.reproducibility,
            NumericalReproducibility::LanguageDefined
        );
    }
}