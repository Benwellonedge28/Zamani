
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Universal Meta-Compiler — Compiler Configuration Foundation.
//!
//! Path:
//!     src/compiler/configuration/config.rs
//!
//! Rust:
//!     Edition 2021
//!     Minimum version: Rust 1.97
//!
//! Safety:
//!     Unsafe Rust is forbidden.
//!
//! # Purpose
//!
//! This module provides the construction, structural validation and
//! compatibility boundary for the repository's existing CompilerConfig.
//!
//! It does not define a competing compiler configuration model.
//!
//! # Canonical ownership
//!
//! crate::compiler_types:
//!     CompilerConfig
//!     CompilationTarget
//!     OptimizationLevel
//!
//! compiler::configuration::budget:
//!     CompilationBudget
//!     BudgetTracker
//!     Operational budget accounting
//!
//! compiler::configuration::target:
//!     TargetDescriptor
//!     TargetId
//!     Target metadata and structural validation
//!
//! compiler::configuration::policy:
//!     PolicyDocument
//!     Policy evaluation contracts
//!
//! compiler::configuration::profile:
//!     CompilationProfileSpec
//!     CompilationProfileRegistry
//!     Profile resolution
//!
//! compiler::configuration::validation:
//!     Aggregate configuration validation
//!
//! # Non-responsibilities
//!
//! This module does not own:
//!
//! - Source grammar or parsing;
//! - AST construction;
//! - Source-level semantic validation;
//! - Target capability discovery;
//! - Physical target selection;
//! - Backend registration or invocation;
//! - Resource allocation;
//! - Runtime scheduling;
//! - Optimization algorithms;
//! - Optimization pass registration;
//! - IR definitions;
//! - Physical execution.
//!
//! # POCO-REAF
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.
//!
//! This module introduces no universal hardware-capacity ceilings.
//!
//! It does not define fixed limits for:
//!
//! - CPUs, cores, workers or threads;
//! - GPUs, FPGAs, QPUs or accelerators;
//! - Distributed nodes or network participants;
//! - Memory or storage;
//! - Qubits or logical qubits;
//! - Tensor dimensions or tensor rank;
//! - Register widths;
//! - Modules, functions or IR operations;
//! - Targets, providers or computational domains.
//!
//! Operational compilation budgets belong to budget.rs.
//!
//! Target capabilities belong to their authoritative capability
//! providers and target descriptions.
//!
//! Physical availability belongs to planning and runtime systems.
//!
//! An absent budget does not promise infinite physical resources.
//!
//! # Grammar boundary
//!
//! Source-level target intent:
//!     grammar/compile/target.g4
//!
//! Target-selection policy:
//!     grammar/compile/target-selection.g4
//!
//! Compilation profiles:
//!     grammar/compile/profiles.g4
//!
//! Optimization intent:
//!     grammar/compile/optimization.g4
//!
//! The frontend and semantic subsystem own parsing, AST construction,
//! name resolution and semantic interpretation.
//!
//! This module consumes already constructed compiler configuration.
//!
//! # Compatibility
//!
//! Existing CompilerConfig fields and the existing builder API are
//! preserved.
//!
//! This module does not change the signatures of existing compile
//! entry points.
//!
//! The legacy CompilationTarget enum remains supported as a
//! compatibility representation.
//!
//! New extensible target descriptions belong to target.rs.
//!
//! # Important distinction
//!
//! Successful configuration validation means that the supplied
//! configuration is structurally acceptable under the selected
//! validation mode.
//!
//! It does not prove:
//!
//! - That a target exists;
//! - That a target is available;
//! - That a backend supports the target;
//! - That the program's requirements are satisfied;
//! - That resources are available;
//! - That compilation or execution will succeed.

use std::error::Error;
use std::fmt;

use crate::compiler_types::{
    CompilationTarget,
    CompilerConfig,
    OptimizationLevel,
};

// ============================================================================
// API VERSION
// ============================================================================

/// Version of the compiler configuration construction and validation API.
///
/// This is independent of the Zamani language version, grammar version,
/// compiler release version, IR version, target descriptor version and
/// backend ABI version.
pub const CONFIGURATION_API_VERSION: u16 = 2;

// ============================================================================
// VALIDATION MODE
// ============================================================================

/// Determines the validation contract applied to a compiler configuration.
///
/// Validation mode does not select a physical target and does not alter
/// the semantic meaning of a Zamani program.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConfigurationValidationMode {
    /// Validate structural correctness and preserve compatibility.
    Standard,

    /// Apply the current production configuration restrictions.
    ///
    /// Production validation is intentionally conservative.
    /// It does not certify backend support or target feasibility.
    Production,
}

// ============================================================================
// ERRORS
// ============================================================================

/// A compiler configuration construction or validation error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfigurationError {
    /// A caller attempted to build a new configuration without
    /// supplying an explicit legacy compilation target.
    MissingTarget,

    /// A legacy target contains an invalid payload.
    InvalidTarget {
        target: String,
        reason: String,
    },

    /// An optimization level is not accepted by the selected
    /// configuration validation mode.
    UnsupportedOptimization {
        optimization: String,
        mode: ConfigurationValidationMode,
    },

    /// A configuration field contains an invalid value.
    InvalidValue {
        field: String,
        reason: String,
    },
}

impl fmt::Display for ConfigurationError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::MissingTarget => {
                formatter.write_str(
                    "a newly constructed compiler configuration \
                     requires an explicit target",
                )
            }

            Self::InvalidTarget { target, reason } => {
                write!(
                    formatter,
                    "invalid compilation target `{target}`: {reason}"
                )
            }

            Self::UnsupportedOptimization {
                optimization,
                mode,
            } => {
                write!(
                    formatter,
                    "optimization level `{optimization}` is not \
                     accepted in {mode:?} configuration validation"
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

impl Error for ConfigurationError {}

// ============================================================================
// LEGACY TARGET VALIDATION
// ============================================================================

/// Validates a legacy CompilationTarget value.
///
/// The legacy target enum is retained for compatibility with existing
/// compiler APIs.
///
/// Open-world target identities and extensible target metadata belong
/// to configuration::target::TargetDescriptor.
///
/// This function checks only the structural validity of the legacy
/// enum's payload. It does not check target availability, capabilities,
/// backend compatibility or physical feasibility.
fn validate_legacy_target(
    target: &CompilationTarget,
) -> Result<(), ConfigurationError> {
    match target {
        CompilationTarget::UniversalIRExport(identifier) => {
            if identifier.trim().is_empty() {
                return Err(ConfigurationError::InvalidTarget {
                    target: "UniversalIRExport".to_owned(),
                    reason: "export format identifier cannot be empty"
                        .to_owned(),
                });
            }

            if identifier.trim() != identifier {
                return Err(ConfigurationError::InvalidTarget {
                    target: identifier.clone(),
                    reason: "export format identifier cannot have \
                             surrounding whitespace"
                        .to_owned(),
                });
            }

            if identifier.chars().any(char::is_control) {
                return Err(ConfigurationError::InvalidTarget {
                    target: identifier.clone(),
                    reason: "export format identifier cannot contain \
                             control characters"
                        .to_owned(),
                });
            }

            Ok(())
        }

        CompilationTarget::X86_64Linux
        | CompilationTarget::Arm64
        | CompilationTarget::Wasm32
        | CompilationTarget::QASM
        | CompilationTarget::NanoControl
        | CompilationTarget::MTSBytecode
        | CompilationTarget::LLVMIR
        | CompilationTarget::RiscV => Ok(()),
    }
}

// ============================================================================
// CONFIGURATION VALIDATION
// ============================================================================

/// Validates an existing canonical compiler configuration.
///
/// This function borrows the existing CompilerConfig and does not
/// replace, mutate or normalize it.
///
/// Standard validation preserves legacy optimization-level compatibility.
///
/// Production validation rejects the currently designated experimental
/// UltraAGI optimization level.
///
/// This production restriction is a compatibility policy, not evidence
/// that other optimization levels are implemented for every domain.
///
/// Actual optimization availability and legality must be established
/// by the optimization and pass-management subsystems.
pub fn validate_compiler_config(
    config: &CompilerConfig,
    mode: ConfigurationValidationMode,
) -> Result<(), ConfigurationError> {
    validate_legacy_target(&config.target)?;

    if matches!(
        (&config.opt_level, mode),
        (
            OptimizationLevel::UltraAGI,
            ConfigurationValidationMode::Production
        )
    ) {
        return Err(ConfigurationError::UnsupportedOptimization {
            optimization: "UltraAGI".to_owned(),
            mode,
        });
    }

    Ok(())
}

/// Returns whether an optimization level is accepted by the current
/// production configuration policy.
///
/// This is not a claim that the optimization has been implemented,
/// verified or proven legal for every IR dialect.
///
/// Domain-specific legality and implementation availability must be
/// checked by the optimization and pass-management authorities.
#[must_use]
pub fn is_production_optimization_supported(
    level: &OptimizationLevel,
) -> bool {
    !matches!(level, OptimizationLevel::UltraAGI)
}

// ============================================================================
// BUILDER
// ============================================================================

/// Builder for the existing canonical CompilerConfig.
///
/// This builder does not define a second configuration structure.
///
/// A newly constructed configuration requires an explicit legacy target.
/// No host-specific target is silently selected.
///
/// The builder preserves the existing CompilerConfig field layout.
///
/// Operational budgets, target descriptors, policies and profiles
/// remain separately owned by their corresponding modules.
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
    /// Creates a builder without selecting a target.
    ///
    /// The caller must provide a target before building.
    #[must_use]
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

    /// Creates a builder with an explicit legacy target.
    #[must_use]
    pub fn for_target(target: CompilationTarget) -> Self {
        Self::new().target(target)
    }

    /// Creates a builder by adapting an existing CompilerConfig.
    ///
    /// This preserves all existing configuration values.
    ///
    /// It does not import or copy operational budgets, target descriptors,
    /// policies or profiles because those are separate contracts.
    #[must_use]
    pub fn from_existing(config: &CompilerConfig) -> Self {
        Self {
            target: Some(config.target.clone()),
            opt_level: config.opt_level.clone(),
            debug_info: config.debug_info,
            verify: config.verify,
            emit_ir: config.emit_ir,
            parallel: config.parallel,
        }
    }

    /// Sets the legacy compilation target.
    #[must_use]
    pub fn target(mut self, target: CompilationTarget) -> Self {
        self.target = Some(target);
        self
    }

    /// Sets the requested optimization level.
    #[must_use]
    pub fn optimization_level(
        mut self,
        level: OptimizationLevel,
    ) -> Self {
        self.opt_level = level;
        self
    }

    /// Enables or disables debug information generation.
    #[must_use]
    pub fn debug_info(mut self, enabled: bool) -> Self {
        self.debug_info = enabled;
        self
    }

    /// Sets the IR verification request.
    ///
    /// This field preserves the existing CompilerConfig contract.
    ///
    /// It does not guarantee that verification is skipped when false,
    /// or that verification is implemented by every pipeline route.
    /// The pipeline must enforce its mandatory verification obligations.
    #[must_use]
    pub fn verify(mut self, enabled: bool) -> Self {
        self.verify = enabled;
        self
    }

    /// Sets the request to emit intermediate representation output.
    ///
    /// This is a request only. The selected output path and backend
    /// must establish whether the requested representation is supported.
    #[must_use]
    pub fn emit_ir(mut self, enabled: bool) -> Self {
        self.emit_ir = enabled;
        self
    }

    /// Sets the request for parallel compilation.
    ///
    /// This is not a worker-count setting and does not guarantee that
    /// every compiler stage can execute concurrently.
    ///
    /// Actual parallel execution is controlled by the pipeline,
    /// available capacity, operational budgets and stage contracts.
    #[must_use]
    pub fn parallel(mut self, enabled: bool) -> Self {
        self.parallel = enabled;
        self
    }

    /// Builds the existing canonical CompilerConfig using standard
    /// structural validation.
    ///
    /// No default hardware target is selected.
    pub fn build(self) -> Result<CompilerConfig, ConfigurationError> {
        self.build_with_validation_mode(
            ConfigurationValidationMode::Standard,
        )
    }

    /// Builds the existing canonical CompilerConfig and validates it
    /// using the requested validation mode.
    pub fn build_with_validation_mode(
        self,
        mode: ConfigurationValidationMode,
    ) -> Result<CompilerConfig, ConfigurationError> {
        let target = self
            .target
            .ok_or(ConfigurationError::MissingTarget)?;

        let config = CompilerConfig {
            target,
            opt_level: self.opt_level,
            debug_info: self.debug_info,
            verify: self.verify,
            emit_ir: self.emit_ir,
            parallel: self.parallel,
        };

        validate_compiler_config(&config, mode)?;

        Ok(config)
    }

    /// Builds and validates a configuration under the current
    /// production configuration policy.
    pub fn build_production(
        self,
    ) -> Result<CompilerConfig, ConfigurationError> {
        self.build_with_validation_mode(
            ConfigurationValidationMode::Production,
        )
    }
}

impl Default for CompilerConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// COMPATIBILITY METHODS
// ============================================================================

/// Compatibility methods on the existing canonical CompilerConfig.
///
/// These methods do not introduce a replacement configuration type.
impl CompilerConfig {
    /// Validates this configuration using standard validation.
    pub fn validate_configuration(
        &self,
    ) -> Result<(), ConfigurationError> {
        validate_compiler_config(
            self,
            ConfigurationValidationMode::Standard,
        )
    }

    /// Validates this configuration using the current production policy.
    pub fn validate_production_configuration(
        &self,
    ) -> Result<(), ConfigurationError> {
        validate_compiler_config(
            self,
            ConfigurationValidationMode::Production,
        )
    }

    /// Returns whether parallel compilation was requested.
    ///
    /// This does not guarantee parallel stage availability.
    #[must_use]
    pub const fn requests_parallel_compilation(&self) -> bool {
        self.parallel
    }

    /// Returns whether debug information was requested.
    #[must_use]
    pub const fn requests_debug_information(&self) -> bool {
        self.debug_info
    }

    /// Returns whether IR emission was requested.
    ///
    /// This does not guarantee that the selected backend implements it.
    #[must_use]
    pub const fn requests_ir_emission(&self) -> bool {
        self.emit_ir
    }

    /// Returns whether verification was requested in this configuration.
    ///
    /// Mandatory pipeline verification obligations remain authoritative.
    #[must_use]
    pub const fn requests_verification(&self) -> bool {
        self.verify
    }
}

// ============================================================================
// TESTS
// ============================================================================

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
        assert!(matches!(
            CompilerConfigBuilder::new().build(),
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
    fn builder_preserves_existing_defaults() {
        let config = explicit_target_config();

        assert_eq!(config.opt_level, OptimizationLevel::Basic);
        assert!(config.debug_info);
        assert!(config.verify);
        assert!(!config.emit_ir);
        assert!(!config.parallel);
    }

    #[test]
    fn builder_preserves_explicit_values() {
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
    fn builder_adapts_existing_configuration_without_losing_values() {
        let original = CompilerConfig {
            target: CompilationTarget::UniversalIRExport(
                "portable-format".to_owned(),
            ),
            opt_level: OptimizationLevel::Aggressive,
            debug_info: false,
            verify: true,
            emit_ir: true,
            parallel: true,
        };

        let adapted = CompilerConfigBuilder::from_existing(&original)
            .build()
            .expect("existing configuration should remain valid");

        assert_eq!(adapted.target, original.target);
        assert_eq!(adapted.opt_level, original.opt_level);
        assert_eq!(adapted.debug_info, original.debug_info);
        assert_eq!(adapted.verify, original.verify);
        assert_eq!(adapted.emit_ir, original.emit_ir);
        assert_eq!(adapted.parallel, original.parallel);
    }

    #[test]
    fn standard_validation_accepts_basic_optimization() {
        assert!(
            explicit_target_config()
                .validate_configuration()
                .is_ok()
        );
    }

    #[test]
    fn production_validation_accepts_supported_optimization_levels() {
        for level in [
            OptimizationLevel::None,
            OptimizationLevel::Basic,
            OptimizationLevel::Aggressive,
        ] {
            let config = CompilerConfigBuilder::for_target(
                CompilationTarget::Arm64,
            )
            .optimization_level(level)
            .build_production()
            .expect("supported production configuration should build");

            assert!(
                config.validate_production_configuration().is_ok()
            );
        }
    }

    #[test]
    fn production_validation_rejects_experimental_optimization() {
        let result = CompilerConfigBuilder::for_target(
            CompilationTarget::X86_64Linux,
        )
        .optimization_level(OptimizationLevel::UltraAGI)
        .build_production();

        assert!(matches!(
            result,
            Err(ConfigurationError::UnsupportedOptimization { .. })
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
    fn empty_universal_ir_export_identifier_is_rejected() {
        let config = CompilerConfig {
            target: CompilationTarget::UniversalIRExport(
                "   ".to_owned(),
            ),
            ..CompilerConfig::default()
        };

        assert!(matches!(
            config.validate_configuration(),
            Err(ConfigurationError::InvalidTarget { .. })
        ));
    }

    #[test]
    fn universal_ir_export_identifier_with_control_characters_is_rejected() {
        let config = CompilerConfig {
            target: CompilationTarget::UniversalIRExport(
                "portable\nformat".to_owned(),
            ),
            ..CompilerConfig::default()
        };

        assert!(matches!(
            config.validate_configuration(),
            Err(ConfigurationError::InvalidTarget { .. })
        ));
    }

    #[test]
    fn valid_universal_ir_export_identifier_is_preserved() {
        let identifier = "custom-portable-format";

        let config = CompilerConfigBuilder::for_target(
            CompilationTarget::UniversalIRExport(
                identifier.to_owned(),
            ),
        )
        .build()
        .expect("valid export identifier should be accepted");

        assert_eq!(
            config.target,
            CompilationTarget::UniversalIRExport(
                identifier.to_owned()
            )
        );
    }

    #[test]
    fn every_existing_legacy_target_variant_is_supported() {
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
                .expect("valid legacy target should be accepted");

            assert_eq!(config.target, target);
        }
    }

    #[test]
    fn configuration_validation_does_not_mutate_input() {
        let config = explicit_target_config();
        let original_target = config.target.clone();
        let original_level = config.opt_level.clone();

        config
            .validate_production_configuration()
            .expect("configuration should be valid");

        assert_eq!(config.target, original_target);
        assert_eq!(config.opt_level, original_level);
    }

    #[test]
    fn configuration_accessors_report_requests() {
        let config = CompilerConfigBuilder::for_target(
            CompilationTarget::Wasm32,
        )
        .debug_info(false)
        .verify(true)
        .emit_ir(true)
        .parallel(true)
        .build()
        .expect("configuration should build");

        assert!(config.requests_parallel_compilation());
        assert!(!config.requests_debug_information());
        assert!(config.requests_ir_emission());
        assert!(config.requests_verification());
    }
}
