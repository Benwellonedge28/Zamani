
 //! Backend configuration for Zamani.
 //!
 //! # Responsibility
 //!
 //! This module defines backend-specific policy and adapts the existing
 //! `CompilerConfig` without introducing a second owner for its target,
 //! optimization level, debug-info flag, verification flag, IR-emission flag,
 //! or parallel-build flag.
 //!
 //! # Ownership boundaries
 //!
 //! This module does not:
 //! - select or discover targets;
 //! - determine the semantic meaning of a Zamani program;
 //! - allocate hardware resources or schedule computation;
 //! - execute external tools;
 //! - write artifacts to disk;
 //! - verify IR or claim that an artifact is valid;
 //! - impose universal limits on CPUs, qubits, devices, memory, tensor
 //!   dimensions, nodes, threads, or other hardware resources.
 //!
 //! Other modules must enforce configured policies and report violations
 //! through structured backend errors and diagnostics.
 //!
 //! # Resource budgets
 //!
 //! An absent budget means that this configuration imposes no limit for that
 //! dimension. It does not guarantee that the operating system, toolchain,
 //! runtime, or target has unlimited resources. A finite budget is an
 //! operational policy, not a language-semantic limit.
 //!
 //! # Toolchains and security
 //!
 //! Toolchain commands are represented as executable paths and argument
 //! vectors, never as shell command strings. This module does not launch them.
 //! The process runner must enforce toolchain policy, cancellation, timeouts,
 //! environment isolation, and output capture.
 //!
 //! # Compatibility
 //!
 //! `CompilerConfig` remains the source of truth for the legacy compiler
 //! settings. The backend policies in this module express additional backend
 //! behavior or explicit overrides; they do not mutate the compiler config.

use std::collections::BTreeMap;
use std::ffi::OsString;
use std::path::PathBuf;
use std::time::Duration;

use crate::compiler_types::CompilerConfig;

/// Policy for applying backend-specific optimization.
///
/// The optimization level itself remains owned by `CompilerConfig::opt_level`.
/// These variants describe how a backend may apply optimizations in addition
/// to the compiler's selected optimization level.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum OptimizationPolicy {
    /// Follow the existing compiler optimization setting.
    #[default]
    InheritCompiler,

    /// Do not add backend-specific optimization passes.
    ///
    /// This does not undo optimizations already performed by earlier compiler
    /// stages.
    DisableBackendSpecific,

    /// Require backend-specific transformations to use deterministic choices
    /// when the implementation and toolchain support that guarantee.
    ///
    /// A consumer must report when it cannot satisfy this policy.
    DeterministicOnly,
}

/// Policy for target artifact verification.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum VerificationPolicy {
    /// Require validation of the generated artifact before reporting success.
    ///
    /// If the selected target has no suitable verifier, the pipeline must fail
    /// explicitly rather than claim that validation succeeded.
    #[default]
    Required,

    /// Verify when a suitable verifier is available.
    ///
    /// If verification is unavailable, the result must accurately represent
    /// that the artifact was not verified.
    WhenAvailable,

    /// Explicitly request that backend artifact verification be skipped.
    ///
    /// This is an opt-out policy, not proof that an artifact is valid.
    Disabled,
}

/// Policy for including debug information in backend artifacts.
///
/// `CompilerConfig::debug_info` remains the default source of truth.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum DebugInfoPolicy {
    /// Follow `CompilerConfig::debug_info`.
    #[default]
    InheritCompiler,

    /// Request debug information regardless of the legacy flag.
    Include,

    /// Request that debug information be omitted.
    Omit,
}

impl DebugInfoPolicy {
    /// Resolve this policy against the existing compiler configuration.
    pub const fn resolve(self, compiler_debug_info: bool) -> bool {
        match self {
            Self::InheritCompiler => compiler_debug_info,
            Self::Include => true,
            Self::Omit => false,
        }
    }
}

/// Requested reproducibility guarantee.
///
/// This is a policy request, not a claim that every external toolchain can
/// produce byte-for-byte reproducible artifacts.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ReproducibilityPolicy {
    /// Do not request reproducibility beyond normal backend behavior.
    Disabled,

    /// Prefer reproducibility and report known limitations.
    #[default]
    BestEffort,

    /// Require the pipeline to establish the configured reproducibility
    /// contract or return an explicit failure.
    Required,
}

/// Policy governing use of external tools.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ExternalToolPolicy {
    /// Do not permit external tool execution.
    Disallow,

    /// Permit only tools explicitly described in this configuration.
    ///
    /// The process runner must reject an undeclared tool.
    #[default]
    ExplicitlyConfiguredOnly,
}

/// Where generated artifacts should be delivered.
///
/// Configuration never writes to these destinations. The output stage owns
/// all filesystem operations.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum OutputDestination {
    /// Return artifacts to the caller without requesting filesystem output.
    #[default]
    InMemory,

    /// Request that the output stage persist artifacts under this directory.
    Directory(PathBuf),
}

/// Policy for delivering generated artifacts.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct OutputPolicy {
    /// Destination requested for generated artifacts.
    pub destination: OutputDestination,

    /// Whether the output stage may replace existing files.
    ///
    /// The output stage must still protect against path traversal, symlink
    /// attacks, and races. This flag alone is not a security boundary.
    pub overwrite_existing: bool,

    /// Whether the output stage may create missing parent directories.
    pub create_parent_directories: bool,
}

/// A validated key naming a configurable resource budget.
///
/// Names are namespaced by convention, for example `compiler.work_units`,
/// `quantum.logical_qubits`, or `deployment.worker_count`. The set of names is
/// intentionally open so future domains do not require edits to this module.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BudgetKey(String);

impl BudgetKey {
    pub fn new(value: impl Into<String>) -> Result<Self, ConfigError> {
        let value = value.into();
        validate_key("budget", &value)?;
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for BudgetKey {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Configurable operational budgets.
///
/// `None` means that this configuration sets no limit for that dimension.
/// `Some(0)` is a real zero budget and must not be interpreted as unlimited.
///
/// The fixed fields cover common operational dimensions. `custom` supports
/// domain-specific dimensions without imposing a closed universal inventory.
/// Budget consumers own measurement and enforcement.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ResourceBudgets {
    /// Maximum accepted input size, in bytes, when enforced by the consumer.
    pub max_input_bytes: Option<u128>,

    /// Maximum generated artifact size, in bytes, when enforced by the
    /// consumer.
    pub max_output_bytes: Option<u128>,

    /// Maximum memory budget, in bytes, when the consumer can measure or
    /// constrain the relevant memory usage.
    pub max_memory_bytes: Option<u128>,

    /// Maximum amount of implementation-defined work.
    ///
    /// Consumers must document the meaning of their work units. Different
    /// units must not be silently compared as if they were interchangeable.
    pub max_work_units: Option<u128>,

    /// Maximum wall-clock duration for an operation, when supported by the
    /// operation's executor.
    pub max_wall_time: Option<Duration>,

    /// Additional namespaced, domain-specific finite budgets.
    pub custom: BTreeMap<BudgetKey, u128>,
}

impl ResourceBudgets {
    /// Set or replace a custom budget.
    ///
    /// Returns the previous value if the key was already present.
    pub fn set_custom(
        &mut self,
        key: BudgetKey,
        limit: u128,
    ) -> Option<u128> {
        self.custom.insert(key, limit)
    }

    /// Retrieve a custom budget, if configured.
    pub fn custom_limit(&self, key: &BudgetKey) -> Option<u128> {
        self.custom.get(key).copied()
    }

    /// Remove a custom budget.
    pub fn remove_custom(&mut self, key: &BudgetKey) -> Option<u128> {
        self.custom.remove(key)
    }
}

/// Validated identifier for a configured external toolchain.
///
/// This is a configuration key, not a target ID, artifact ID, or globally
/// unique identity. Those identities belong to `ids.rs`.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ToolchainKey(String);

impl ToolchainKey {
    pub fn new(value: impl Into<String>) -> Result<Self, ConfigError> {
        let value = value.into();
        validate_key("toolchain", &value)?;
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for ToolchainKey {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Declarative configuration for one external tool.
///
/// Arguments are separate operating-system strings, not a shell command.
/// This type does not execute the command or guarantee that the executable is
/// trusted, present, compatible, or safe.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ToolchainConfig {
    /// Executable path resolved by the process/toolchain layer.
    pub executable: PathBuf,

    /// Arguments passed individually to the executable.
    pub arguments: Vec<OsString>,

    /// Environment variables explicitly supplied to the process.
    ///
    /// The process runner must define how these interact with inherited
    /// environment variables and must never assume that this map alone
    /// guarantees hermetic execution.
    pub environment: BTreeMap<OsString, OsString>,

    /// Optional working directory for the process.
    pub working_directory: Option<PathBuf>,

    /// Whether the process runner may inherit the caller's environment.
    ///
    /// For reproducible or security-sensitive builds, prefer `false` and
    /// supply the required environment explicitly.
    pub inherit_environment: bool,
}

impl ToolchainConfig {
    pub fn new(executable: impl Into<PathBuf>) -> Result<Self, ConfigError> {
        let executable = executable.into();

        if executable.as_os_str().is_empty() {
            return Err(ConfigError::EmptyExecutablePath);
        }

        Ok(Self {
            executable,
            arguments: Vec::new(),
            environment: BTreeMap::new(),
            working_directory: None,
            inherit_environment: false,
        })
    }

    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.executable.as_os_str().is_empty() {
            return Err(ConfigError::EmptyExecutablePath);
        }

        if self
            .working_directory
            .as_ref()
            .is_some_and(|path| path.as_os_str().is_empty())
        {
            return Err(ConfigError::EmptyWorkingDirectory);
        }

        Ok(())
    }
}

/// Backend-specific policy combined with the existing compiler configuration.
///
/// The embedded `CompilerConfig` remains authoritative for target selection,
/// optimization level, legacy debug-info configuration, IR emission, and
/// parallel compilation. This wrapper adds policies needed at the backend
/// boundary without requiring those existing fields to be redefined.
#[derive(Clone, Debug)]
pub struct BackendConfig {
    /// Existing compiler settings. Do not create a second target or
    /// optimization-level source of truth elsewhere in the backend.
    pub compiler: CompilerConfig,

    /// Policy for additional backend-specific optimization.
    pub optimization: OptimizationPolicy,

    /// Policy for debug information in generated artifacts.
    pub debug_info: DebugInfoPolicy,

    /// Policy for validating generated artifacts.
    pub verification: VerificationPolicy,

    /// Requested artifact delivery behavior.
    pub output: OutputPolicy,

    /// Operational budgets to be enforced by the appropriate consumers.
    pub budgets: ResourceBudgets,

    /// Explicitly configured external toolchains.
    pub toolchains: BTreeMap<ToolchainKey, ToolchainConfig>,

    /// Policy for invoking external tools.
    pub external_tools: ExternalToolPolicy,

    /// Requested reproducibility guarantee.
    pub reproducibility: ReproducibilityPolicy,
}

impl BackendConfig {
    /// Create backend policy around an existing compiler configuration.
    ///
    /// No target is inferred from machine hardware. The target remains the
    /// one explicitly present in `compiler`.
    pub fn new(compiler: CompilerConfig) -> Self {
        Self {
            compiler,
            optimization: OptimizationPolicy::InheritCompiler,
            debug_info: DebugInfoPolicy::InheritCompiler,
            verification: VerificationPolicy::Required,
            output: OutputPolicy::default(),
            budgets: ResourceBudgets::default(),
            toolchains: BTreeMap::new(),
            external_tools: ExternalToolPolicy::ExplicitlyConfiguredOnly,
            reproducibility: ReproducibilityPolicy::BestEffort,
        }
    }

    /// Adapt the repository's existing compiler configuration.
    pub fn from_compiler_config(compiler: &CompilerConfig) -> Self {
        Self::new(compiler.clone())
    }

    /// Resolve the effective debug-info request.
    pub const fn effective_debug_info(&self) -> bool {
        self.debug_info.resolve(self.compiler.debug_info)
    }

    /// Register or replace an explicitly configured toolchain.
    ///
    /// The key is independent of a physical target name. For example, a
    /// project may configure distinct assembler and linker toolchains for the
    /// same target.
    pub fn set_toolchain(
        &mut self,
        key: ToolchainKey,
        toolchain: ToolchainConfig,
    ) -> Result<Option<ToolchainConfig>, ConfigError> {
        toolchain.validate()?;
        Ok(self.toolchains.insert(key, toolchain))
    }

    /// Look up a configured toolchain.
    pub fn toolchain(&self, key: &ToolchainKey) -> Option<&ToolchainConfig> {
        self.toolchains.get(key)
    }

    /// Validate internal configuration consistency.
    ///
    /// This does not check target capabilities, executable existence, external
    /// tool versions, output permissions, or actual budget enforcement.
    /// Those checks require the target, toolchain, filesystem, or execution
    /// context and belong to their respective integration layers.
    pub fn validate(&self) -> Result<(), ConfigError> {
        if let OutputDestination::Directory(path) = &self.output.destination {
            if path.as_os_str().is_empty() {
                return Err(ConfigError::EmptyOutputDirectory);
            }
        }

        for toolchain in self.toolchains.values() {
            toolchain.validate()?;
        }

        if self.external_tools == ExternalToolPolicy::Disallow
            && !self.toolchains.is_empty()
        {
            return Err(ConfigError::ToolchainsConfiguredButDisallowed);
        }

        Ok(())
    }

    /// Consume the wrapper and return the existing compiler configuration.
    ///
    /// Backend-specific policy is intentionally not folded into
    /// `CompilerConfig`; callers that need those policies must keep and pass
    /// the complete `BackendConfig` to the backend pipeline.
    pub fn into_compiler_config(self) -> CompilerConfig {
        self.compiler
    }
}

impl Default for BackendConfig {
    /// Preserve the repository's existing `CompilerConfig::default()` behavior.
    ///
    /// This default does not assert that the selected target is available on
    /// the current machine or that it supports a given program.
    fn default() -> Self {
        Self::new(CompilerConfig::default())
    }
}

/// Configuration validation failures.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ConfigError {
    InvalidKey {
        kind: &'static str,
        value: String,
    },
    EmptyExecutablePath,
    EmptyWorkingDirectory,
    EmptyOutputDirectory,
    ToolchainsConfiguredButDisallowed,
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidKey { kind, value } => {
                write!(formatter, "invalid {kind} key `{value}`")
            }
            Self::EmptyExecutablePath => {
                formatter.write_str("toolchain executable path must not be empty")
            }
            Self::EmptyWorkingDirectory => {
                formatter.write_str("toolchain working directory must not be empty")
            }
            Self::EmptyOutputDirectory => {
                formatter.write_str("output directory must not be empty")
            }
            Self::ToolchainsConfiguredButDisallowed => formatter.write_str(
                "toolchains are configured, but external tool execution is disallowed",
            ),
        }
    }
}

impl std::error::Error for ConfigError {}

/// Validate a stable, non-empty, ASCII key.
///
/// The allowed alphabet supports namespaced keys such as `quantum.qasm`,
/// `llvm.assembler`, and `deployment.worker-count`. Keys are not interpreted
/// as paths or commands.
fn validate_key(kind: &'static str, value: &str) -> Result<(), ConfigError> {
    let mut chars = value.chars();

    let valid_first = chars
        .next()
        .is_some_and(|ch| ch.is_ascii_alphanumeric());

    let valid_rest = chars.all(|ch| {
        ch.is_ascii_alphanumeric() || matches!(ch, '.' | '_' | '-' | ':' | '/')
    });

    if valid_first && valid_rest {
        Ok(())
    } else {
        Err(ConfigError::InvalidKey {
            kind,
            value: value.to_owned(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_wraps_existing_compiler_configuration() {
        let legacy = CompilerConfig::default();
        let config = BackendConfig::from_compiler_config(&legacy);

        assert_eq!(config.compiler.target, legacy.target);
        assert_eq!(config.compiler.opt_level, legacy.opt_level);
        assert_eq!(config.compiler.debug_info, legacy.debug_info);
        assert_eq!(config.compiler.verify, legacy.verify);
        assert_eq!(config.compiler.emit_ir, legacy.emit_ir);
        assert_eq!(config.compiler.parallel, legacy.parallel);
        assert_eq!(config.optimization, OptimizationPolicy::InheritCompiler);
    }

    #[test]
    fn absent_budgets_mean_no_limit_is_configured() {
        let budgets = ResourceBudgets::default();

        assert_eq!(budgets.max_input_bytes, None);
        assert_eq!(budgets.max_output_bytes, None);
        assert_eq!(budgets.max_memory_bytes, None);
        assert_eq!(budgets.max_work_units, None);
        assert_eq!(budgets.max_wall_time, None);
        assert!(budgets.custom.is_empty());
    }

    #[test]
    fn zero_is_a_finite_budget_not_unlimited() {
        let mut budgets = ResourceBudgets::default();
        budgets.max_work_units = Some(0);

        assert_eq!(budgets.max_work_units, Some(0));
    }

    #[test]
    fn custom_budgets_are_extensible_and_deterministically_ordered() {
        let mut budgets = ResourceBudgets::default();
        let qubits = BudgetKey::new("quantum.logical_qubits").unwrap();
        let workers = BudgetKey::new("distributed.worker-count").unwrap();

        assert_eq!(budgets.set_custom(qubits.clone(), 128), None);
        assert_eq!(budgets.set_custom(workers.clone(), 32), None);
        assert_eq!(budgets.custom_limit(&qubits), Some(128));
        assert_eq!(budgets.remove_custom(&workers), Some(32));
        assert_eq!(budgets.custom_limit(&workers), None);
    }

    #[test]
    fn empty_or_unsafe_key_syntax_is_rejected() {
        assert!(BudgetKey::new("").is_err());
        assert!(BudgetKey::new("../outside").is_err());
        assert!(ToolchainKey::new("llvm assembler").is_err());
        assert!(ToolchainKey::new("llvm.assembler").is_ok());
    }

    #[test]
    fn debug_info_policy_resolves_against_legacy_setting() {
        assert!(DebugInfoPolicy::InheritCompiler.resolve(true));
        assert!(!DebugInfoPolicy::InheritCompiler.resolve(false));
        assert!(DebugInfoPolicy::Include.resolve(false));
        assert!(!DebugInfoPolicy::Omit.resolve(true));
    }

    #[test]
    fn toolchain_arguments_are_individual_values() {
        let mut tool = ToolchainConfig::new("tools/assembler").unwrap();
        tool.arguments.push(OsString::from("--target"));
        tool.arguments.push(OsString::from("example-target"));

        assert_eq!(tool.arguments.len(), 2);
        assert_eq!(tool.arguments[0], OsString::from("--target"));
        assert_eq!(tool.arguments[1], OsString::from("example-target"));
        assert!(tool.validate().is_ok());
    }

    #[test]
    fn output_directory_must_not_be_empty() {
        let mut config = BackendConfig::default();
        config.output.destination = OutputDestination::Directory(PathBuf::new());

        assert_eq!(
            config.validate(),
            Err(ConfigError::EmptyOutputDirectory)
        );
    }

    #[test]
    fn disallowed_external_tools_cannot_be_configured() {
        let mut config = BackendConfig::default();
        let key = ToolchainKey::new("llvm.assembler").unwrap();
        let tool = ToolchainConfig::new("llvm-as").unwrap();

        config.set_toolchain(key, tool).unwrap();
        config.external_tools = ExternalToolPolicy::Disallow;

        assert_eq!(
            config.validate(),
            Err(ConfigError::ToolchainsConfiguredButDisallowed)
        );
    }

    #[test]
    fn default_configuration_is_internally_valid() {
        assert!(BackendConfig::default().validate().is_ok());
    }
}
