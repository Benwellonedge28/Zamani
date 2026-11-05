
#![forbid(unsafe_code)]

//! Zamani Universal Meta-Compiler — Configuration Foundation
//!
//! File:
//!     src/compiler/configuration/config.rs
//!
//! Purpose:
//!     Provide the canonical construction, validation, and compatibility
//!     boundary for compiler configuration.
//!
//! Rust:
//!     Edition 2021
//!     Minimum version 1.97
//!
//! Safety:
//!     Unsafe Rust is forbidden.
//!
//! Architectural ownership:
//!
//!     crate::compiler_types
//!         owns CompilerConfig,
//!         CompilationTarget,
//!         OptimizationLevel.
//!
//!     compiler::configuration::config
//!         owns configuration construction,
//!         configuration validation,
//!         and compatibility adaptation.
//!
//!     compiler::configuration::budget
//!         will own configurable compilation resource budgets.
//!
//!     compiler::configuration::policy
//!         will own compilation policy definitions.
//!
//!     compiler::configuration::target
//!         will own extensible target descriptors and target policies.
//!
//!     compiler::configuration::profile
//!         will own named compilation profiles.
//!
//! This module deliberately does not define competing target,
//! optimization, semantic, capability, resource, or backend types.
//!
//! POCO-REAF:
//!     Configuration must not impose universal hardware capacity limits.
//!     Concrete target feasibility belongs to target and backend
//!     capability contracts.
//!
//! Compatibility:
//!     Existing CompilerConfig values remain usable without conversion.
//!     Existing compile(), compile_with_config(), and compile_source()
//!     signatures are not changed by this module.

use std::fmt;

use crate::compiler_types::{
    CompilationTarget,
    CompilerConfig,
    OptimizationLevel,
};

/// Stable configuration API version.
///
/// This identifies the compiler configuration interface. It is not
/// the Zamani language version, compiler version, IR version, or
/// backend ABI version.
pub const CONFIGURATION_API_VERSION: u16 = 1;

/// Describes whether configuration validation is being performed
/// for ordinary compatibility or production compilation.
///
/// This does not select a hardware target and does not alter
/// program semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConfigurationValidationMode {
    /// Validate structural configuration correctness.
    ///
    /// This mode is useful for library consumers and tooling that
    /// construct configurations before deciding how to compile.
    Standard,

    /// Validate configuration for the currently supported
    /// production compilation contract.
    ///
    /// Experimental optimization modes are rejected.
    Production,
}

/// A structured configuration validation failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfigurationError {
    /// The configuration contains an optimization mode that is
    /// not supported by the selected validation mode.
    UnsupportedOptimization {
        optimization: String,
        mode: ConfigurationValidationMode,
    },

    /// The configuration is missing a required explicit target.
    MissingTarget,

    /// A future configuration extension supplied an invalid value.
    InvalidValue {
        field: String,
        reason: String,
    },
}

impl fmt::Display for ConfigurationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedOptimization {
                optimization,
                mode,
            } => {
                write!(
                    formatter,
                    "optimization mode `{optimization}` is not \
                     supported in {mode:?} configuration validation"
                )
            }

            Self::MissingTarget => {
                formatter.write_str(
                    "compiler configuration requires an explicit target",
                )
            }

            Self::InvalidValue { field, reason } => {
                write!(
                    formatter,
                    "invalid compiler configuration field `{field}`: {reason}"
                )
            }
        }
    }
}

impl std::error::Error for ConfigurationError {}

/// Validates an existing canonical compiler configuration.
///
/// This function intentionally accepts the existing `CompilerConfig`
/// by reference. It does not create a replacement configuration type.
///
/// The current compiler pipeline always performs IR verification,
/// regardless of the `verify` field. Therefore this function does
/// not pretend that `verify = false` disables verification.
///
/// The `verify` field remains available for compatibility with the
/// existing public configuration contract. Its pipeline semantics
/// must be reconciled in the compiler orchestration layer.
pub fn validate_compiler_config(
    config: &CompilerConfig,
    mode: ConfigurationValidationMode,
) -> Result<(), ConfigurationError> {
    match (&config.opt_level, mode) {
        (
            OptimizationLevel::UltraAGI,
            ConfigurationValidationMode::Production,
        ) => Err(ConfigurationError::UnsupportedOptimization {
            optimization: "UltraAGI".to_owned(),
            mode,
        }),

        _ => Ok(()),
    }
}

/// Returns whether an optimization level is currently accepted
/// by the production compiler pipeline.
///
/// This is a compatibility and safety check, not an optimization
/// implementation or a guarantee that every accepted optimization
/// has been verified for every future IR dialect.
pub fn is_production_optimization_supported(
    level: &OptimizationLevel,
) -> bool {
    !matches!(level, OptimizationLevel::UltraAGI)
}

/// Builder for the repository's existing `CompilerConfig`.
///
/// The builder requires an explicit target. This avoids silently
/// selecting a particular processor architecture when a caller
/// is constructing a new configuration.
///
/// The builder does not own or define target capabilities.
/// It uses the canonical `CompilationTarget` type.
#[derive(Debug, Clone)]
pub struct CompilerConfigBuilder {
    target: Option<CompilationTarget>,
    opt_level: OptimizationLevel,
    debug_info: bool,
    verify: bool,
    emit_ir: bool,
    parallel: bool,
}

impl CompilerConfigBuilder {
    /// Creates a builder without choosing a target.
    ///
    /// A target must be supplied before `build()` succeeds.
    pub fn new() -> Self {
        Self {
            target: None,
            opt_level: OptimizationLevel::Basic,
            debug_info: true,
            verify: true,
            emit_ir: false,
            parallel: false,
        }
    }

    /// Creates a builder with an explicit compilation target.
    pub fn for_target(target: CompilationTarget) -> Self {
        Self::new().target(target)
    }

    /// Sets the explicit target.
    pub fn target(mut self, target: CompilationTarget) -> Self {
        self.target = Some(target);
        self
    }

    /// Sets the optimization level.
    pub fn optimization_level(
        mut self,
        level: OptimizationLevel,
    ) -> Self {
        self.opt_level = level;
        self
    }

    /// Enables or disables debug information.
    pub fn debug_info(mut self, enabled: bool) -> Self {
        self.debug_info = enabled;
        self
    }

    /// Sets the requested IR verification configuration.
    ///
    /// Important:
    /// The current compiler pipeline invokes IR verification
    /// unconditionally. Setting this field to false does not
    /// currently disable that verification.
    pub fn verify(mut self, enabled: bool) -> Self {
        self.verify = enabled;
        self
    }

    /// Requests intermediate representation output.
    ///
    /// The existing backend pipeline must be checked to determine
    /// whether this request is actually honored for each target.
    pub fn emit_ir(mut self, enabled: bool) -> Self {
        self.emit_ir = enabled;
        self
    }

    /// Requests parallel compilation.
    ///
    /// This is a request, not a guarantee that every compiler stage
    /// supports parallel execution.
    pub fn parallel(mut self, enabled: bool) -> Self {
        self.parallel = enabled;
        self
    }

    /// Builds the existing canonical `CompilerConfig`.
    ///
    /// A target is mandatory. No default hardware target is selected.
    pub fn build(self) -> Result<CompilerConfig, ConfigurationError> {
        let target = self
            .target
            .ok_or(ConfigurationError::MissingTarget)?;

        Ok(CompilerConfig {
            target,
            opt_level: self.opt_level,
            debug_info: self.debug_info,
            verify: self.verify,
            emit_ir: self.emit_ir,
            parallel: self.parallel,
        })
    }

    /// Builds a configuration and validates it for production use.
    pub fn build_production(
        self,
    ) -> Result<CompilerConfig, ConfigurationError> {
        let config = self.build()?;

        validate_compiler_config(
            &config,
            ConfigurationValidationMode::Production,
        )?;

        Ok(config)
    }
}

impl Default for CompilerConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Compatibility helpers for the existing canonical configuration.
///
/// These methods are implemented on the existing type rather than
/// introducing a duplicate configuration structure.
impl CompilerConfig {
    /// Validates this configuration for standard use.
    pub fn validate_configuration(
        &self,
    ) -> Result<(), ConfigurationError> {
        validate_compiler_config(
            self,
            ConfigurationValidationMode::Standard,
        )
    }

    /// Validates this configuration for production use.
    pub fn validate_production_configuration(
        &self,
    ) -> Result<(), ConfigurationError> {
        validate_compiler_config(
            self,
            ConfigurationValidationMode::Production,
        )
    }

    /// Returns whether this configuration requests parallel compilation.
    ///
    /// This reports configuration intent only. It does not establish
    /// that every pipeline stage supports parallel execution.
    pub const fn requests_parallel_compilation(&self) -> bool {
        self.parallel
    }

    /// Returns whether this configuration requests debug information.
    pub const fn requests_debug_information(&self) -> bool {
        self.debug_info
    }

    /// Returns whether this configuration requests IR emission.
    ///
    /// This reports the configured request, not proof that a backend
    /// has implemented the corresponding output behavior.
    pub const fn requests_ir_emission(&self) -> bool {
        self.emit_ir
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn explicit_target_config() -> CompilerConfig {
        CompilerConfigBuilder::for_target(
            CompilationTarget::X86_64Linux,
        )
        .build()
        .expect("explicit target configuration should build")
    }

    #[test]
    fn configuration_api_version_is_nonzero() {
        assert_ne!(CONFIGURATION_API_VERSION, 0);
    }

    #[test]
    fn builder_requires_an_explicit_target() {
        let result = CompilerConfigBuilder::new().build();

        assert!(matches!(
            result,
            Err(ConfigurationError::MissingTarget)
        ));
    }

    #[test]
    fn builder_accepts_an_explicit_target() {
        let config = explicit_target_config();

        assert_eq!(
            config.target,
            CompilationTarget::X86_64Linux
        );
    }

    #[test]
    fn builder_uses_safe_production_friendly_defaults() {
        let config = explicit_target_config();

        assert_eq!(config.opt_level, OptimizationLevel::Basic);
        assert!(config.debug_info);
        assert!(config.verify);
        assert!(!config.emit_ir);
        assert!(!config.parallel);
    }

    #[test]
    fn builder_preserves_explicit_configuration_values() {
        let config = CompilerConfigBuilder::for_target(
            CompilationTarget::Wasm32,
        )
        .optimization_level(OptimizationLevel::Aggressive)
        .debug_info(false)
        .verify(true)
        .emit_ir(true)
        .parallel(true)
        .build()
        .expect("configuration should build");

        assert_eq!(config.target, CompilationTarget::Wasm32);
        assert_eq!(
            config.opt_level,
            OptimizationLevel::Aggressive
        );
        assert!(!config.debug_info);
        assert!(config.verify);
        assert!(config.emit_ir);
        assert!(config.parallel);
    }

    #[test]
    fn production_validation_accepts_basic_optimization() {
        let config = explicit_target_config();

        assert!(config.validate_production_configuration().is_ok());
    }

    #[test]
    fn production_validation_accepts_aggressive_optimization() {
        let config = CompilerConfigBuilder::for_target(
            CompilationTarget::Arm64,
        )
        .optimization_level(OptimizationLevel::Aggressive)
        .build()
        .expect("configuration should build");

        assert!(config.validate_production_configuration().is_ok());
    }

    #[test]
    fn production_validation_rejects_experimental_optimization() {
        let config = CompilerConfigBuilder::for_target(
            CompilationTarget::X86_64Linux,
        )
        .optimization_level(OptimizationLevel::UltraAGI)
        .build()
        .expect("configuration construction should succeed");

        let result = config.validate_production_configuration();

        assert!(matches!(
            result,
            Err(ConfigurationError::UnsupportedOptimization {
                ..
            })
        ));
    }

    #[test]
    fn standard_validation_preserves_legacy_optimization_compatibility() {
        let config = CompilerConfig {
            opt_level: OptimizationLevel::UltraAGI,
            ..CompilerConfig::default()
        };

        assert!(config.validate_configuration().is_ok());
    }

    #[test]
    fn production_optimization_support_is_explicit() {
        assert!(is_production_optimization_supported(
            &OptimizationLevel::None
        ));

        assert!(is_production_optimization_supported(
            &OptimizationLevel::Basic
        ));

        assert!(is_production_optimization_supported(
            &OptimizationLevel::Aggressive
        ));

        assert!(!is_production_optimization_supported(
            &OptimizationLevel::UltraAGI
        ));
    }

    #[test]
    fn existing_configuration_fields_remain_accessible() {
        let config = explicit_target_config();

        assert!(!config.requests_parallel_compilation());
        assert!(config.requests_debug_information());
        assert!(!config.requests_ir_emission());
    }

    #[test]
    fn builder_supports_existing_target_variants() {
        let targets = vec![
            CompilationTarget::X86_64Linux,
            CompilationTarget::Arm64,
            CompilationTarget::Wasm32,
            CompilationTarget::QASM,
            CompilationTarget::NanoControl,
            CompilationTarget::MTSBytecode,
            CompilationTarget::LLVMIR,
            CompilationTarget::RiscV,
            CompilationTarget::UniversalIRExport(
                "custom-portable-format".to_owned(),
            ),
        ];

        for target in targets {
            let config = CompilerConfigBuilder::for_target(target.clone())
                .build()
                .expect("explicit target should be accepted");

            assert_eq!(config.target, target);
        }
    }

    #[test]
    fn standard_validation_does_not_reject_existing_configurations() {
        let config = CompilerConfig::default();

        assert!(config.validate_configuration().is_ok());
    }

    #[test]
    fn production_validation_does_not_change_configuration() {
        let config = explicit_target_config();
        let original_target = config.target.clone();
        let original_level = config.opt_level.clone();

        config
            .validate_production_configuration()
            .expect("configuration should be valid");

        assert_eq!(config.target, original_target);
        assert_eq!(config.opt_level, original_level);
    }
}
