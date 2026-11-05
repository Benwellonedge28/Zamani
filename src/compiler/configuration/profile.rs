
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Universal Meta-Compiler
//!
//! Compilation Profile Resolution
//!
//! Path:
//!     src/compiler/configuration/profile.rs
//!
//! Rust:
//!     Edition 2021
//!     Minimum version 1.97
//!
//! Safety:
//!     Unsafe Rust is forbidden.
//!
//! ============================================================================
//! PURPOSE
//! ============================================================================
//!
//! This module defines the canonical compilation-profile policy layer.
//!
//! A compilation profile describes reusable compilation intent.
//!
//! It determines:
//!
//! - Optimization policy.
//! - Expected compilation effort.
//! - Transformation safety posture.
//! - Verification requirements.
//! - Recommended optimization-pass identifiers.
//! - Profile compatibility.
//! - Deterministic configuration resolution.
//! - Custom profile registration.
//! - Profile identification and versioning.
//!
//! A profile is not a hardware target.
//!
//! A profile is not a backend.
//!
//! A profile is not an optimization implementation.
//!
//! A profile is not an execution environment.
//!
//! ============================================================================
//! ARCHITECTURAL OWNERSHIP
//! ============================================================================
//!
//! Existing authoritative owners:
//!
//! crate::compiler_types::CompilerConfig
//!     Owns the existing compiler configuration.
//!
//! crate::compiler_types::OptimizationLevel
//!     Owns the compiler optimization-level vocabulary.
//!
//! compiler::configuration::config
//!     Owns configuration construction and validation.
//!
//! compiler::configuration::target
//!     Owns extensible target descriptors.
//!
//! quantum::optimization::config
//!     Owns quantum-specific optimization profiles.
//!
//! quantum::optimization::profile
//!     Owns quantum-specific optimization-profile resolution.
//!
//! This module owns:
//!
//! - Domain-neutral compilation profile identifiers.
//! - Domain-neutral compilation profile specifications.
//! - Profile classification.
//! - Profile registry.
//! - Profile resolution.
//! - Configuration adaptation.
//! - Profile compatibility validation.
//!
//! This module does NOT own:
//!
//! - Target descriptors.
//! - Hardware capabilities.
//! - Resource allocation.
//! - Resource budgets.
//! - Backend selection.
//! - Backend implementation.
//! - Optimization algorithms.
//! - Optimization pass implementation.
//! - Quantum IR.
//! - Classical IR.
//! - HDL IR.
//! - AI IR.
//! - Runtime execution.
//! - Physical scheduling.
//! - Quantum routing.
//! - Quantum error correction.
//! - Source parsing.
//! - AST ownership.
//! - Semantic analysis.
//!
//! ============================================================================
//! POCO-REAF
//! ============================================================================
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.
//!
//! Profiles are independent of:
//!
//! - CPU architecture.
//! - CPU count.
//! - GPU vendor.
//! - GPU count.
//! - FPGA family.
//! - ASIC architecture.
//! - QPU provider.
//! - Qubit count.
//! - Distributed node count.
//! - Operating system.
//! - Memory capacity.
//! - Register width.
//! - Tensor rank.
//! - Accelerator count.
//! - Physical topology.
//!
//! No universal hardware capacity limits are introduced.
//!
//! Profiles may be used with tiny or extremely large workloads.
//!
//! Actual resource availability and feasibility are evaluated by
//! their respective compiler, capability, planning, and runtime owners.
//!
//! ============================================================================
//! DEPENDENCY DIRECTION
//! ============================================================================
//!
//! compiler_types
//!       |
//!       v
//! configuration::config
//!       |
//!       v
//! configuration::profile
//!       |
//!       +----> Compilation API
//!       |
//!       +----> Compilation pipeline
//!       |
//!       +----> Optimization planning
//!       |
//!       +----> Pass registry
//!       |
//!       +----> Artifact provenance
//!
//! This module deliberately does not depend on the compiler pipeline.
//!
//! It can therefore be completed before pipeline orchestration.
//!
//! ============================================================================

use std::collections::btree_map;
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;
use std::str::FromStr;

use crate::compiler_types::{
    CompilerConfig,
    OptimizationLevel,
};

use crate::compiler::configuration::config::{
    validate_compiler_config,
    ConfigurationValidationMode,
};

// ============================================================================
// VERSION
// ============================================================================

/// Version of the compilation-profile contract.
///
/// This is independent of:
///
/// - Zamani language version.
/// - Compiler release version.
/// - IR version.
/// - Backend ABI version.
/// - Target descriptor version.
///
/// Increment this version only when the interpretation of an existing
/// profile contract changes incompatibly.
pub const PROFILE_SCHEMA_VERSION: u32 = 1;

/// Stable API version for profile resolution.
pub const PROFILE_API_VERSION: u16 = 1;

// ============================================================================
// ERRORS
// ============================================================================

/// Compilation-profile errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProfileError {
    /// An identifier is empty.
    EmptyIdentifier,

    /// An identifier contains unsupported characters.
    InvalidIdentifier {
        value: String,
        reason: String,
    },

    /// A profile description is empty.
    EmptyDescription,

    /// A profile does not exist in the selected registry.
    UnknownProfile {
        identifier: String,
    },

    /// A profile identifier already exists.
    DuplicateProfile {
        identifier: String,
    },

    /// A profile contains an invalid pass identifier.
    InvalidPassIdentifier {
        identifier: String,
    },

    /// A profile contains duplicate pass identifiers.
    DuplicatePassIdentifier {
        identifier: String,
    },

    /// A production profile uses an unsupported optimization level.
    UnsupportedOptimization {
        profile: String,
        optimization: String,
    },

    /// A profile requires verification but the resulting configuration
    /// does not satisfy that requirement.
    VerificationRequired {
        profile: String,
    },

    /// The existing compiler configuration is invalid.
    InvalidConfiguration {
        message: String,
    },

    /// An explicit profile replacement was attempted for a missing entry.
    ProfileNotRegistered {
        identifier: String,
    },
}

impl fmt::Display for ProfileError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::EmptyIdentifier => {
                formatter.write_str(
                    "compilation profile identifier cannot be empty",
                )
            }

            Self::InvalidIdentifier { value, reason } => {
                write!(
                    formatter,
                    "invalid compilation profile identifier `{value}`: {reason}"
                )
            }

            Self::EmptyDescription => {
                formatter.write_str(
                    "compilation profile description cannot be empty",
                )
            }

            Self::UnknownProfile { identifier } => {
                write!(
                    formatter,
                    "unknown compilation profile `{identifier}`",
                )
            }

            Self::DuplicateProfile { identifier } => {
                write!(
                    formatter,
                    "compilation profile `{identifier}` is already registered",
                )
            }

            Self::InvalidPassIdentifier { identifier } => {
                write!(
                    formatter,
                    "invalid optimization-pass identifier `{identifier}`",
                )
            }

            Self::DuplicatePassIdentifier { identifier } => {
                write!(
                    formatter,
                    "duplicate optimization-pass identifier `{identifier}`",
                )
            }

            Self::UnsupportedOptimization {
                profile,
                optimization,
            } => {
                write!(
                    formatter,
                    "profile `{profile}` requests unsupported production optimization `{optimization}`",
                )
            }

            Self::VerificationRequired { profile } => {
                write!(
                    formatter,
                    "profile `{profile}` requires verification",
                )
            }

            Self::InvalidConfiguration { message } => {
                write!(
                    formatter,
                    "profile produced an invalid compiler configuration: {message}",
                )
            }

            Self::ProfileNotRegistered { identifier } => {
                write!(
                    formatter,
                    "compilation profile `{identifier}` is not registered",
                )
            }
        }
    }
}

impl Error for ProfileError {}

/// Result type for profile operations.
pub type ProfileResult<T> = Result<T, ProfileError>;

// ============================================================================
// IDENTIFIERS
// ============================================================================

/// Stable, extensible compilation-profile identifier.
///
/// This is intentionally a string-based identifier rather than a closed enum.
///
/// New profiles can be registered without modifying compiler source files.
///
/// Identifier comparison is case-sensitive.
///
/// Canonical identifiers use lowercase ASCII.
///
/// Allowed characters:
///
/// - a-z
/// - 0-9
/// - _
/// - -
/// - .
/// - :
/// - /
///
/// No arbitrary identifier-length limit is imposed.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CompilationProfileId(String);

impl CompilationProfileId {
    /// Constructs a validated profile identifier.
    pub fn new(
        value: impl Into<String>,
    ) -> ProfileResult<Self> {
        let value = value.into();

        if value.is_empty() {
            return Err(ProfileError::EmptyIdentifier);
        }

        if value.trim() != value {
            return Err(ProfileError::InvalidIdentifier {
                value,
                reason: "leading or trailing whitespace is forbidden"
                    .to_owned(),
            });
        }

        let valid = value.bytes().all(|byte| {
            byte.is_ascii_lowercase()
                || byte.is_ascii_digit()
                || matches!(
                    byte,
                    b'_' | b'-' | b'.' | b':' | b'/'
                )
        });

        if !valid {
            return Err(ProfileError::InvalidIdentifier {
                value,
                reason:
                    "only lowercase ASCII letters, digits, underscores, \
                     hyphens, periods and namespace separators are allowed"
                        .to_owned(),
            });
        }

        Ok(Self(value))
    }

    /// Returns the canonical identifier.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns the identifier as an owned string.
    pub fn into_string(self) -> String {
        self.0
    }

    /// Returns whether this is the generic profile.
    pub fn is_generic(&self) -> bool {
        self.as_str() == "generic"
    }
}

impl fmt::Display for CompilationProfileId {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl AsRef<str> for CompilationProfileId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl FromStr for CompilationProfileId {
    type Err = ProfileError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

// ============================================================================
// PROFILE CLASSIFICATION
// ============================================================================

/// Expected compiler effort.
///
/// This is descriptive policy metadata.
///
/// It is not a time limit, memory limit, or guaranteed execution cost.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProfileEffort {
    /// Prioritize fast compilation.
    Low,

    /// Ordinary production compilation.
    Moderate,

    /// Spend additional effort on optimization.
    High,

    /// Permit substantial bounded optimization exploration.
    VeryHigh,
}

impl ProfileEffort {
    /// Returns the stable textual identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Low => "low",
            Self::Moderate => "moderate",
            Self::High => "high",
            Self::VeryHigh => "very_high",
        }
    }
}

/// Transformation safety classification.
///
/// This describes the intended safety posture.
///
/// Actual semantic preservation must be established by the
/// transformation and verification subsystems.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProfileSafety {
    /// Conservative transformations.
    Conservative,

    /// Exact semantics are required.
    Exact,

    /// Verification is mandatory.
    Verified,

    /// Exploratory optimization, without implicit semantic approximation.
    Exploratory,
}

impl ProfileSafety {
    /// Returns the stable textual identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Conservative => "conservative",
            Self::Exact => "exact",
            Self::Verified => "verified",
            Self::Exploratory => "exploratory",
        }
    }
}

// ============================================================================
// STABLE OPTIMIZATION IDENTIFIERS
// ============================================================================

/// Stable normalization-pass identifier.
pub const PASS_NORMALIZE: &str = "optimizer.normalize";

/// Stable constant-folding identifier.
pub const PASS_CONSTANT_FOLD: &str = "optimizer.constant_fold";

/// Stable dead-code-elimination identifier.
pub const PASS_DEAD_CODE_ELIMINATION: &str =
    "optimizer.dead_code_elimination";

/// Stable common-subexpression-elimination identifier.
pub const PASS_COMMON_SUBEXPRESSION: &str =
    "optimizer.common_subexpression";

/// Stable loop-optimization identifier.
pub const PASS_LOOP_OPTIMIZATION: &str =
    "optimizer.loop_optimization";

/// Stable verification identifier.
pub const PASS_VERIFICATION: &str = "compiler.verification";

/// Stable intermediate-representation validation identifier.
pub const PASS_IR_VALIDATION: &str = "compiler.ir_validation";

// ============================================================================
// PROFILE SPECIFICATION
// ============================================================================

/// Immutable description of a compilation profile.
///
/// Profile specifications contain policy metadata only.
///
/// They do not construct optimization passes.
///
/// Pass identifiers are resolved by the pass registry or the
/// authoritative optimization subsystem.
///
/// A recommended pass is not proof that its implementation exists.
///
/// The consumer must establish pass availability and legality.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompilationProfileSpec {
    id: CompilationProfileId,

    description: String,

    optimization_level: OptimizationLevel,

    effort: ProfileEffort,

    safety: ProfileSafety,

    requires_verification: bool,

    recommended_passes: Vec<String>,
}

impl CompilationProfileSpec {
    /// Constructs a validated compilation-profile specification.
    pub fn new(
        id: CompilationProfileId,
        description: impl Into<String>,
        optimization_level: OptimizationLevel,
        effort: ProfileEffort,
        safety: ProfileSafety,
        requires_verification: bool,
        recommended_passes: Vec<String>,
    ) -> ProfileResult<Self> {
        let description = description.into();

        if description.trim().is_empty() {
            return Err(ProfileError::EmptyDescription);
        }

        let mut seen = BTreeMap::new();

        for identifier in &recommended_passes {
            validate_pass_identifier(identifier)?;

            if seen.insert(identifier.as_str(), ()).is_some() {
                return Err(
                    ProfileError::DuplicatePassIdentifier {
                        identifier: identifier.clone(),
                    },
                );
            }
        }

        if matches!(safety, ProfileSafety::Verified)
            && !requires_verification
        {
            return Err(ProfileError::InvalidIdentifier {
                value: id.to_string(),
                reason:
                    "verified safety classification requires verification"
                        .to_owned(),
            });
        }

        Ok(Self {
            id,
            description,
            optimization_level,
            effort,
            safety,
            requires_verification,
            recommended_passes,
        })
    }

    /// Returns the profile identifier.
    pub fn id(&self) -> &CompilationProfileId {
        &self.id
    }

    /// Returns the profile description.
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Returns the optimization level.
    pub fn optimization_level(&self) -> &OptimizationLevel {
        &self.optimization_level
    }

    /// Returns the expected compiler effort.
    pub const fn effort(&self) -> ProfileEffort {
        self.effort
    }

    /// Returns the transformation safety classification.
    pub const fn safety(&self) -> ProfileSafety {
        self.safety
    }

    /// Returns whether verification is mandatory.
    pub const fn requires_verification(&self) -> bool {
        self.requires_verification
    }

    /// Returns recommended pass identifiers.
    pub fn recommended_passes(&self) -> &[String] {
        &self.recommended_passes
    }

    /// Checks whether a pass is recommended.
    pub fn recommends_pass(&self, identifier: &str) -> bool {
        self.recommended_passes
            .iter()
            .any(|candidate| candidate == identifier)
    }

    /// Returns whether the profile is intended for substantial optimization.
    pub const fn is_optimization_intensive(&self) -> bool {
        matches!(
            self.effort,
            ProfileEffort::High | ProfileEffort::VeryHigh
        )
    }

    /// Returns whether the profile requires verified transformations.
    pub const fn is_verified(&self) -> bool {
        matches!(self.safety, ProfileSafety::Verified)
    }

    /// Applies this profile to an existing compiler configuration.
    ///
    /// The existing target is preserved.
    ///
    /// The existing debug-information setting is preserved.
    ///
    /// The existing IR-emission setting is preserved.
    ///
    /// The existing parallel-compilation request is preserved.
    ///
    /// The optimization level is resolved from this profile.
    ///
    /// Verification is enabled when required by the profile.
    ///
    /// This method does not select a physical target, allocate resources,
    /// invoke a backend, or execute optimization passes.
    pub fn apply_to(
        &self,
        base: &CompilerConfig,
    ) -> CompilerConfig {
        let mut resolved = base.clone();

        resolved.opt_level = self.optimization_level.clone();

        if self.requires_verification {
            resolved.verify = true;
        }

        resolved
    }

    /// Applies this profile while preserving an explicitly supplied
    /// optimization-level override.
    ///
    /// The override is useful when a caller intentionally wants to
    /// customize optimization intensity independently of the named profile.
    ///
    /// The profile's verification requirement cannot be disabled.
    pub fn apply_with_optimization_override(
        &self,
        base: &CompilerConfig,
        optimization_override: Option<OptimizationLevel>,
    ) -> CompilerConfig {
        let mut resolved = self.apply_to(base);

        if let Some(level) = optimization_override {
            resolved.opt_level = level;
        }

        if self.requires_verification {
            resolved.verify = true;
        }

        resolved
    }
}

fn validate_pass_identifier(
    identifier: &str,
) -> ProfileResult<()> {
    if identifier.is_empty()
        || identifier.trim() != identifier
        || !identifier.bytes().all(|byte| {
            byte.is_ascii_lowercase()
                || byte.is_ascii_digit()
                || matches!(
                    byte,
                    b'_' | b'-' | b'.' | b':' | b'/'
                )
        })
    {
        return Err(ProfileError::InvalidPassIdentifier {
            identifier: identifier.to_owned(),
        });
    }

    Ok(())
}

// ============================================================================
// PROFILE REGISTRY
// ============================================================================

/// Local compilation-profile registry.
///
/// This registry is deliberately instance-based.
///
/// It does not use global mutable state.
///
/// Different compiler services can therefore maintain independent
/// profile collections.
///
/// BTreeMap provides deterministic iteration and lookup ordering.
///
/// Custom profiles can be registered without modifying this module.
#[derive(Debug, Clone, Default)]
pub struct CompilationProfileRegistry {
    profiles: BTreeMap<
        CompilationProfileId,
        CompilationProfileSpec,
    >,
}

impl CompilationProfileRegistry {
    /// Creates an empty profile registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a registry populated with the standard profiles.
    ///
    /// Every built-in profile is constructed through the same
    /// validation contract used by custom profiles.
    pub fn with_builtins() -> Self {
        let mut registry = Self::new();

        for profile in builtin_profiles() {
            registry
                .register(profile)
                .expect("built-in profile definitions must be valid");
        }

        registry
    }

    /// Registers a new profile.
    ///
    /// Existing profiles cannot be silently overwritten.
    pub fn register(
        &mut self,
        profile: CompilationProfileSpec,
    ) -> ProfileResult<()> {
        let identifier = profile.id().clone();

        if self.profiles.contains_key(&identifier) {
            return Err(ProfileError::DuplicateProfile {
                identifier: identifier.to_string(),
            });
        }

        self.profiles.insert(identifier, profile);

        Ok(())
    }

    /// Replaces an already registered profile explicitly.
    ///
    /// This operation should be used only by configuration-management
    /// code that intentionally changes a profile definition.
    pub fn replace(
        &mut self,
        profile: CompilationProfileSpec,
    ) -> ProfileResult<CompilationProfileSpec> {
        let identifier = profile.id().clone();

        self.profiles
            .insert(identifier.clone(), profile)
            .ok_or_else(|| ProfileError::ProfileNotRegistered {
                identifier: identifier.to_string(),
            })
    }

    /// Retrieves a profile by identifier.
    pub fn get(
        &self,
        identifier: &CompilationProfileId,
    ) -> Option<&CompilationProfileSpec> {
        self.profiles.get(identifier)
    }

    /// Retrieves a profile using a string identifier.
    pub fn get_by_name(
        &self,
        identifier: &str,
    ) -> ProfileResult<&CompilationProfileSpec> {
        let identifier = CompilationProfileId::new(identifier)?;

        self.get(&identifier)
            .ok_or_else(|| ProfileError::UnknownProfile {
                identifier: identifier.to_string(),
            })
    }

    /// Checks whether a profile is registered.
    pub fn contains(
        &self,
        identifier: &CompilationProfileId,
    ) -> bool {
        self.profiles.contains_key(identifier)
    }

    /// Returns the number of registered profiles.
    pub fn len(&self) -> usize {
        self.profiles.len()
    }

    /// Returns whether the registry is empty.
    pub fn is_empty(&self) -> bool {
        self.profiles.is_empty()
    }

    /// Returns deterministic profile iteration.
    pub fn iter(
        &self,
    ) -> btree_map::Iter<
        '_,
        CompilationProfileId,
        CompilationProfileSpec,
    > {
        self.profiles.iter()
    }

    /// Removes a profile explicitly.
    ///
    /// Built-in profiles receive no special mutable global treatment.
    /// Registry owners decide which profiles may be removed.
    pub fn remove(
        &mut self,
        identifier: &CompilationProfileId,
    ) -> Option<CompilationProfileSpec> {
        self.profiles.remove(identifier)
    }

    /// Resolves a profile into an existing CompilerConfig.
    ///
    /// Target and unrelated caller configuration remain unchanged.
    pub fn resolve_config(
        &self,
        identifier: &str,
        base: &CompilerConfig,
    ) -> ProfileResult<ResolvedCompilationProfile> {
        let spec = self.get_by_name(identifier)?;

        let config = spec.apply_to(base);

        validate_profile_compatibility(spec, &config)?;

        Ok(ResolvedCompilationProfile {
            profile_id: spec.id().clone(),
            config,
            recommended_passes: spec.recommended_passes().to_vec(),
            effort: spec.effort(),
            safety: spec.safety(),
        })
    }

    /// Resolves a profile with an explicit optimization-level override.
    pub fn resolve_config_with_override(
        &self,
        identifier: &str,
        base: &CompilerConfig,
        optimization_override: Option<OptimizationLevel>,
    ) -> ProfileResult<ResolvedCompilationProfile> {
        let spec = self.get_by_name(identifier)?;

        let config = spec.apply_with_optimization_override(
            base,
            optimization_override,
        );

        validate_profile_compatibility(spec, &config)?;

        Ok(ResolvedCompilationProfile {
            profile_id: spec.id().clone(),
            config,
            recommended_passes: spec.recommended_passes().to_vec(),
            effort: spec.effort(),
            safety: spec.safety(),
        })
    }
}

// ============================================================================
// RESOLVED PROFILE
// ============================================================================

/// Fully resolved compilation-profile result.
///
/// This is an immutable snapshot of the selected profile and resulting
/// existing CompilerConfig.
///
/// It contains no target allocation or backend execution state.
#[derive(Debug, Clone)]
pub struct ResolvedCompilationProfile {
    profile_id: CompilationProfileId,

    config: CompilerConfig,

    recommended_passes: Vec<String>,

    effort: ProfileEffort,

    safety: ProfileSafety,
}

impl ResolvedCompilationProfile {
    /// Returns the resolved profile identifier.
    pub fn profile_id(&self) -> &CompilationProfileId {
        &self.profile_id
    }

    /// Returns the resolved existing compiler configuration.
    pub fn config(&self) -> &CompilerConfig {
        &self.config
    }

    /// Returns an owned compiler configuration.
    pub fn into_config(self) -> CompilerConfig {
        self.config
    }

    /// Returns the recommended optimization-pass identifiers.
    pub fn recommended_passes(&self) -> &[String] {
        &self.recommended_passes
    }

    /// Returns expected compilation effort.
    pub const fn effort(&self) -> ProfileEffort {
        self.effort
    }

    /// Returns the profile's safety classification.
    pub const fn safety(&self) -> ProfileSafety {
        self.safety
    }
}

// ============================================================================
// COMPATIBILITY VALIDATION
// ============================================================================

/// Validates that a resolved configuration satisfies its profile.
///
/// This function does not:
///
/// - Inspect the program.
/// - Inspect IR.
/// - Discover hardware.
/// - Check pass availability.
/// - Check physical resource feasibility.
/// - Execute verification.
/// - Perform optimization.
///
/// Those responsibilities belong to their respective subsystems.
pub fn validate_profile_compatibility(
    profile: &CompilationProfileSpec,
    config: &CompilerConfig,
) -> ProfileResult<()> {
    validate_compiler_config(
        config,
        ConfigurationValidationMode::Production,
    )
    .map_err(|error| ProfileError::InvalidConfiguration {
        message: error.to_string(),
    })?;

    if profile.requires_verification() && !config.verify {
        return Err(ProfileError::VerificationRequired {
            profile: profile.id().to_string(),
        });
    }

    if matches!(
        config.opt_level,
        OptimizationLevel::UltraAGI
    ) {
        return Err(ProfileError::UnsupportedOptimization {
            profile: profile.id().to_string(),
            optimization: "UltraAGI".to_owned(),
        });
    }

    Ok(())
}

// ============================================================================
// BUILT-IN PROFILES
// ============================================================================

/// Constructs the standard domain-neutral compilation profiles.
///
/// These profiles describe general compiler behavior.
///
/// Specialized quantum profiles remain owned by:
///
/// crate::quantum::optimization::config
///
/// and:
///
/// crate::quantum::optimization::profile
fn builtin_profiles() -> Vec<CompilationProfileSpec> {
    vec![
        profile(
            "generic",
            "General-purpose portable compilation.",
            OptimizationLevel::Basic,
            ProfileEffort::Moderate,
            ProfileSafety::Exact,
            true,
            &[
                PASS_NORMALIZE,
                PASS_CONSTANT_FOLD,
                PASS_IR_VALIDATION,
                PASS_VERIFICATION,
            ],
        ),
        profile(
            "fast_compile",
            "Prioritizes compilation speed and conservative optimization.",
            OptimizationLevel::None,
            ProfileEffort::Low,
            ProfileSafety::Conservative,
            true,
            &[
                PASS_NORMALIZE,
                PASS_IR_VALIDATION,
                PASS_VERIFICATION,
            ],
        ),
        profile(
            "balanced",
            "Balances compilation effort and optimization opportunities.",
            OptimizationLevel::Basic,
            ProfileEffort::Moderate,
            ProfileSafety::Exact,
            true,
            &[
                PASS_NORMALIZE,
                PASS_CONSTANT_FOLD,
                PASS_DEAD_CODE_ELIMINATION,
                PASS_COMMON_SUBEXPRESSION,
                PASS_IR_VALIDATION,
                PASS_VERIFICATION,
            ],
        ),
        profile(
            "aggressive",
            "Allows increased optimization effort while preserving exact semantics.",
            OptimizationLevel::Aggressive,
            ProfileEffort::High,
            ProfileSafety::Exact,
            true,
            &[
                PASS_NORMALIZE,
                PASS_CONSTANT_FOLD,
                PASS_DEAD_CODE_ELIMINATION,
                PASS_COMMON_SUBEXPRESSION,
                PASS_LOOP_OPTIMIZATION,
                PASS_IR_VALIDATION,
                PASS_VERIFICATION,
            ],
        ),
        profile(
            "debug",
            "Prioritizes debugging and diagnostic-friendly compilation.",
            OptimizationLevel::None,
            ProfileEffort::Low,
            ProfileSafety::Conservative,
            true,
            &[
                PASS_NORMALIZE,
                PASS_IR_VALIDATION,
                PASS_VERIFICATION,
            ],
        ),
        profile(
            "verified",
            "Requires verification-oriented compilation.",
            OptimizationLevel::Basic,
            ProfileEffort::High,
            ProfileSafety::Verified,
            true,
            &[
                PASS_NORMALIZE,
                PASS_CONSTANT_FOLD,
                PASS_IR_VALIDATION,
                PASS_VERIFICATION,
            ],
        ),
    ]
}

/// Internal constructor for trusted built-in definitions.
fn profile(
    identifier: &str,
    description: &str,
    optimization_level: OptimizationLevel,
    effort: ProfileEffort,
    safety: ProfileSafety,
    requires_verification: bool,
    recommended_passes: &[&str],
) -> CompilationProfileSpec {
    CompilationProfileSpec::new(
        CompilationProfileId::new(identifier)
            .expect("built-in profile identifiers must be valid"),
        description,
        optimization_level,
        effort,
        safety,
        requires_verification,
        recommended_passes
            .iter()
            .map(|identifier| (*identifier).to_owned())
            .collect(),
    )
    .expect("built-in profile definitions must be valid")
}

// ============================================================================
// CONVENIENCE FUNCTIONS
// ============================================================================

/// Returns the standard profile registry.
///
/// The returned registry is independently owned by the caller.
pub fn standard_profile_registry() -> CompilationProfileRegistry {
    CompilationProfileRegistry::with_builtins()
}

/// Resolves a standard profile into the existing compiler configuration.
pub fn resolve_standard_profile(
    identifier: &str,
    base: &CompilerConfig,
) -> ProfileResult<ResolvedCompilationProfile> {
    standard_profile_registry().resolve_config(identifier, base)
}

/// Returns whether a profile is a standard built-in profile.
pub fn is_builtin_profile(identifier: &str) -> bool {
    standard_profile_registry()
        .get_by_name(identifier)
        .is_ok()
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::compiler_types::CompilationTarget;

    fn base_config() -> CompilerConfig {
        CompilerConfig {
            target: CompilationTarget::UniversalIRExport(
                "portable-test".to_owned(),
            ),
            opt_level: OptimizationLevel::Basic,
            debug_info: true,
            verify: false,
            emit_ir: true,
            parallel: true,
        }
    }

    fn custom_profile(
        identifier: &str,
    ) -> CompilationProfileSpec {
        CompilationProfileSpec::new(
            CompilationProfileId::new(identifier)
                .expect("identifier should be valid"),
            "Custom test profile",
            OptimizationLevel::Basic,
            ProfileEffort::Moderate,
            ProfileSafety::Exact,
            true,
            vec![PASS_NORMALIZE.to_owned()],
        )
        .expect("profile should be valid")
    }

    #[test]
    fn profile_api_versions_are_nonzero() {
        assert_ne!(PROFILE_API_VERSION, 0);
        assert_ne!(PROFILE_SCHEMA_VERSION, 0);
    }

    #[test]
    fn profile_identifier_accepts_namespaced_extensions() {
        let identifier =
            CompilationProfileId::new("organization.domain.profile")
                .expect("identifier should be valid");

        assert_eq!(
            identifier.as_str(),
            "organization.domain.profile"
        );
    }

    #[test]
    fn profile_identifier_rejects_empty_values() {
        assert!(matches!(
            CompilationProfileId::new(""),
            Err(ProfileError::EmptyIdentifier)
        ));
    }

    #[test]
    fn profile_identifier_rejects_uppercase_values() {
        assert!(matches!(
            CompilationProfileId::new("Balanced"),
            Err(ProfileError::InvalidIdentifier { .. })
        ));
    }

    #[test]
    fn profile_identifier_rejects_whitespace() {
        assert!(matches!(
            CompilationProfileId::new("balanced profile"),
            Err(ProfileError::InvalidIdentifier { .. })
        ));
    }

    #[test]
    fn profile_registry_contains_standard_profiles() {
        let registry = standard_profile_registry();

        for identifier in [
            "generic",
            "fast_compile",
            "balanced",
            "aggressive",
            "debug",
            "verified",
        ] {
            assert!(
                registry.get_by_name(identifier).is_ok(),
                "missing profile {identifier}"
            );
        }
    }

    #[test]
    fn profile_registry_is_deterministically_ordered() {
        let registry = standard_profile_registry();

        let identifiers = registry
            .iter()
            .map(|(identifier, _)| identifier.as_str())
            .collect::<Vec<_>>();

        let mut sorted = identifiers.clone();
        sorted.sort_unstable();

        assert_eq!(identifiers, sorted);
    }

    #[test]
    fn profile_registry_rejects_duplicate_registration() {
        let mut registry = CompilationProfileRegistry::new();

        registry
            .register(custom_profile("custom.test"))
            .expect("first registration should succeed");

        let result =
            registry.register(custom_profile("custom.test"));

        assert!(matches!(
            result,
            Err(ProfileError::DuplicateProfile { .. })
        ));
    }

    #[test]
    fn profile_registry_supports_custom_profiles() {
        let mut registry = standard_profile_registry();

        registry
            .register(custom_profile("organization.custom"))
            .expect("custom profile should register");

        assert!(
            registry
                .get_by_name("organization.custom")
                .is_ok()
        );
    }

    #[test]
    fn profile_registry_rejects_unknown_profiles() {
        let registry = standard_profile_registry();

        assert!(matches!(
            registry.get_by_name("unknown.profile"),
            Err(ProfileError::UnknownProfile { .. })
        ));
    }

    #[test]
    fn generic_profile_resolves_deterministically() {
        let base = base_config();

        let first = resolve_standard_profile("generic", &base)
            .expect("generic profile should resolve");

        let second = resolve_standard_profile("generic", &base)
            .expect("generic profile should resolve");

        assert_eq!(
            first.profile_id(),
            second.profile_id()
        );

        assert_eq!(
            first.config().opt_level,
            second.config().opt_level
        );
    }

    #[test]
    fn profile_preserves_existing_target() {
        let base = base_config();

        let resolved = resolve_standard_profile(
            "balanced",
            &base,
        )
        .expect("profile should resolve");

        assert_eq!(
            resolved.config().target,
            base.target
        );
    }

    #[test]
    fn profile_preserves_caller_debug_information() {
        let mut base = base_config();
        base.debug_info = false;

        let resolved = resolve_standard_profile(
            "balanced",
            &base,
        )
        .expect("profile should resolve");

        assert!(!resolved.config().debug_info);
    }

    #[test]
    fn profile_preserves_ir_emission_request() {
        let base = base_config();

        let resolved = resolve_standard_profile(
            "balanced",
            &base,
        )
        .expect("profile should resolve");

        assert!(resolved.config().emit_ir);
    }

    #[test]
    fn profile_preserves_parallel_compilation_request() {
        let base = base_config();

        let resolved = resolve_standard_profile(
            "balanced",
            &base,
        )
        .expect("profile should resolve");

        assert!(resolved.config().parallel);
    }

    #[test]
    fn verification_profiles_enable_verification() {
        let base = base_config();

        for identifier in [
            "generic",
            "balanced",
            "aggressive",
            "verified",
        ] {
            let resolved = resolve_standard_profile(
                identifier,
                &base,
            )
            .expect("profile should resolve");

            assert!(
                resolved.config().verify,
                "{identifier} must enable verification"
            );
        }
    }

    #[test]
    fn fast_compile_preserves_mandatory_verification() {
        let base = base_config();

        let resolved = resolve_standard_profile(
            "fast_compile",
            &base,
        )
        .expect("profile should resolve");

        assert!(resolved.config().verify);
        assert_eq!(
            resolved.effort(),
            ProfileEffort::Low
        );
    }

    #[test]
    fn aggressive_profile_uses_existing_optimization_level() {
        let base = base_config();

        let resolved = resolve_standard_profile(
            "aggressive",
            &base,
        )
        .expect("profile should resolve");

        assert_eq!(
            resolved.config().opt_level,
            OptimizationLevel::Aggressive
        );
    }

    #[test]
    fn explicit_optimization_override_is_preserved() {
        let base = base_config();

        let registry = standard_profile_registry();

        let resolved = registry
            .resolve_config_with_override(
                "balanced",
                &base,
                Some(OptimizationLevel::None),
            )
            .expect("override should resolve");

        assert_eq!(
            resolved.config().opt_level,
            OptimizationLevel::None
        );
    }

    #[test]
    fn verified_profile_cannot_disable_verification() {
        let base = base_config();

        let resolved = resolve_standard_profile(
            "verified",
            &base,
        )
        .expect("profile should resolve");

        assert!(resolved.config().verify);
        assert_eq!(
            resolved.safety(),
            ProfileSafety::Verified
        );
    }

    #[test]
    fn experimental_optimization_is_rejected_in_production() {
        let mut base = base_config();

        base.opt_level = OptimizationLevel::UltraAGI;

        let registry = standard_profile_registry();

        let result = registry.resolve_config_with_override(
            "balanced",
            &base,
            Some(OptimizationLevel::UltraAGI),
        );

        assert!(result.is_err());
    }

    #[test]
    fn custom_pass_identifiers_are_extensible() {
        let profile = CompilationProfileSpec::new(
            CompilationProfileId::new("custom.ai").unwrap(),
            "AI-oriented profile",
            OptimizationLevel::Basic,
            ProfileEffort::High,
            ProfileSafety::Exact,
            true,
            vec![
                "organization.ai.specialized_pass".to_owned(),
            ],
        )
        .expect("custom pass identifiers should be supported");

        assert!(
            profile.recommends_pass(
                "organization.ai.specialized_pass"
            )
        );
    }

    #[test]
    fn duplicate_pass_identifiers_are_rejected() {
        let result = CompilationProfileSpec::new(
            CompilationProfileId::new("custom.duplicate").unwrap(),
            "Duplicate pass test",
            OptimizationLevel::Basic,
            ProfileEffort::Moderate,
            ProfileSafety::Exact,
            true,
            vec![
                PASS_NORMALIZE.to_owned(),
                PASS_NORMALIZE.to_owned(),
            ],
        );

        assert!(matches!(
            result,
            Err(ProfileError::DuplicatePassIdentifier { .. })
        ));
    }

    #[test]
    fn empty_descriptions_are_rejected() {
        let result = CompilationProfileSpec::new(
            CompilationProfileId::new("custom.empty").unwrap(),
            " ",
            OptimizationLevel::Basic,
            ProfileEffort::Moderate,
            ProfileSafety::Exact,
            true,
            Vec::new(),
        );

        assert!(matches!(
            result,
            Err(ProfileError::EmptyDescription)
        ));
    }

    #[test]
    fn profile_error_display_is_stable() {
        let error = ProfileError::UnknownProfile {
            identifier: "missing".to_owned(),
        };

        assert_eq!(
            error.to_string(),
            "unknown compilation profile `missing`"
        );
    }

    #[test]
    fn custom_profile_can_be_replaced_explicitly() {
        let mut registry = CompilationProfileRegistry::new();

        registry
            .register(custom_profile("custom.replace"))
            .unwrap();

        let replacement = CompilationProfileSpec::new(
            CompilationProfileId::new("custom.replace").unwrap(),
            "Replacement profile",
            OptimizationLevel::Aggressive,
            ProfileEffort::High,
            ProfileSafety::Exact,
            true,
            vec![PASS_NORMALIZE.to_owned()],
        )
        .unwrap();

        let previous = registry
            .replace(replacement)
            .expect("registered profile should be replaceable");

        assert_eq!(
            previous.id().as_str(),
            "custom.replace"
        );
    }
}
