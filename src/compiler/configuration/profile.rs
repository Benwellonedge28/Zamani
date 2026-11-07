
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Universal Meta-Compiler
//!
//! Compilation Profile Definitions and Resolution.
//!
//! Path:
//!     src/compiler/configuration/profile.rs
//!
//! Rust:
//!     Rust 1.97+
//!     Edition 2021
//!
//! # Purpose
//!
//! Provides reusable, versioned, domain-neutral compilation profiles.
//!
//! A compilation profile describes compiler intent and policy.
//!
//! It does not implement optimization, select physical hardware,
//! allocate resources, or execute programs.
//!
//! # Architectural ownership
//!
//! Canonical existing authorities:
//!
//! - crate::compiler_types::CompilerConfig
//! - crate::compiler_types::CompilationTarget
//! - crate::compiler_types::OptimizationLevel
//! - configuration::config
//! - configuration::target
//! - configuration::budget
//! - configuration::policy
//! - configuration::validation
//!
//! The frontend and semantic subsystem remain authoritative for
//! source-level profile declarations, inheritance and interpretation.
//!
//! # POCO-REAF
//!
//! Profiles are independent of:
//!
//! - CPU architecture and processor count.
//! - GPU, FPGA and QPU providers.
//! - Memory and storage capacities.
//! - Qubit counts.
//! - Tensor rank.
//! - Register widths.
//! - Distributed node counts.
//! - Physical topology.
//! - Operating systems.
//!
//! No universal hardware capacity ceilings are introduced.
//!
//! Actual feasibility is determined by capability providers,
//! compiler planning, backends and runtime.
//!
//! # Important distinction
//!
//! A profile's optimization level is not a guarantee that a particular
//! optimization is implemented or legal for every IR dialect.
//!
//! Recommended pass identifiers are advisory references.
//!
//! Pass availability and legality must be established by the
//! authoritative pass and optimization subsystems.
//!
//! # Grammar integration
//!
//! Source syntax:
//!     grammar/compile/profiles.g4
//!
//! Optimization intent:
//!     grammar/compile/optimization.g4
//!
//! Target-selection policy:
//!     grammar/compile/target-selection.g4
//!
//! Semantic interpretation:
//!     src/frontend/semantic/
//!
//! This module does not parse grammar or implement profile inheritance.
//!
//! # Determinism
//!
//! Registry iteration and profile lookup are deterministic.
//!
//! No global mutable state is used.
//!
//! # Safety
//!
//! Unsafe Rust is forbidden.

use std::collections::btree_map;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
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
// VERSIONS
// ============================================================================

/// Current compilation-profile schema version.
///
/// Independent of the language, grammar, compiler, IR and backend versions.
pub const PROFILE_SCHEMA_VERSION: u32 = 1;

/// Public compilation-profile API version.
pub const PROFILE_API_VERSION: u16 = 1;

// ============================================================================
// ERRORS
// ============================================================================

/// Compilation-profile errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProfileError {
    EmptyIdentifier,

    InvalidIdentifier {
        value: String,
        reason: String,
    },

    EmptyDescription,

    UnknownProfile {
        identifier: String,
    },

    DuplicateProfile {
        identifier: String,
    },

    InvalidPassIdentifier {
        identifier: String,
    },

    DuplicatePassIdentifier {
        identifier: String,
    },

    UnsupportedSchemaVersion {
        supplied: u32,
        supported: u32,
    },

    VerificationRequired {
        profile: String,
    },

    InvalidConfiguration {
        message: String,
    },

    ProfileNotRegistered {
        identifier: String,
    },
}

impl fmt::Display for ProfileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyIdentifier => {
                write!(f, "compilation profile identifier is empty")
            }

            Self::InvalidIdentifier { value, reason } => {
                write!(f, "invalid profile identifier `{value}`: {reason}")
            }

            Self::EmptyDescription => {
                write!(f, "compilation profile description is empty")
            }

            Self::UnknownProfile { identifier } => {
                write!(f, "unknown compilation profile `{identifier}`")
            }

            Self::DuplicateProfile { identifier } => {
                write!(f, "profile `{identifier}` is already registered")
            }

            Self::InvalidPassIdentifier { identifier } => {
                write!(f, "invalid optimization pass identifier `{identifier}`")
            }

            Self::DuplicatePassIdentifier { identifier } => {
                write!(f, "duplicate optimization pass identifier `{identifier}`")
            }

            Self::UnsupportedSchemaVersion {
                supplied,
                supported,
            } => {
                write!(
                    f,
                    "unsupported profile schema version {supplied}; \
                     supported version is {supported}"
                )
            }

            Self::VerificationRequired { profile } => {
                write!(f, "profile `{profile}` requires verification")
            }

            Self::InvalidConfiguration { message } => {
                write!(f, "invalid resolved compiler configuration: {message}")
            }

            Self::ProfileNotRegistered { identifier } => {
                write!(f, "profile `{identifier}` is not registered")
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

/// Stable open-world compilation-profile identifier.
///
/// Identifiers are case-sensitive and use canonical lowercase ASCII.
///
/// No arbitrary identifier-length ceiling is imposed.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CompilationProfileId(String);

impl CompilationProfileId {
    pub fn new(value: impl Into<String>) -> ProfileResult<Self> {
        let value = value.into();

        if value.is_empty() {
            return Err(ProfileError::EmptyIdentifier);
        }

        if value.trim() != value {
            return Err(ProfileError::InvalidIdentifier {
                value,
                reason: "surrounding whitespace is forbidden".into(),
            });
        }

        if !valid_identifier(&value) {
            return Err(ProfileError::InvalidIdentifier {
                value,
                reason: "identifier contains unsupported characters".into(),
            });
        }

        Ok(Self(value))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }

    #[must_use]
    pub fn is_generic(&self) -> bool {
        self.as_str() == "generic"
    }
}

impl fmt::Display for CompilationProfileId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl AsRef<str> for CompilationProfileId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl FromStr for CompilationProfileId {
    type Err = ProfileError;

    fn from_str(value: &str) -> ProfileResult<Self> {
        Self::new(value)
    }
}

fn valid_identifier(value: &str) -> bool {
    !value.is_empty()
        && value.bytes().all(|byte| {
            byte.is_ascii_lowercase()
                || byte.is_ascii_digit()
                || matches!(byte, b'_' | b'-' | b'.' | b':' | b'/')
        })
}

// ============================================================================
// PROFILE CLASSIFICATION
// ============================================================================

/// Descriptive optimization effort.
///
/// These are policy classifications, not resource limits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProfileEffort {
    Low,
    Moderate,
    High,
    VeryHigh,
}

impl ProfileEffort {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Low => "low",
            Self::Moderate => "moderate",
            Self::High => "high",
            Self::VeryHigh => "very_high",
        }
    }
}

/// Transformation safety policy.
///
/// These classifications describe requested policy.
/// They do not constitute proof of semantic preservation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProfileSafety {
    Conservative,
    Exact,
    Verified,
    Exploratory,
}

impl ProfileSafety {
    #[must_use]
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
// STABLE PASS IDENTIFIERS
// ============================================================================

pub const PASS_NORMALIZE: &str = "optimizer.normalize";

pub const PASS_CONSTANT_FOLD: &str = "optimizer.constant_fold";

pub const PASS_DEAD_CODE_ELIMINATION: &str =
    "optimizer.dead_code_elimination";

pub const PASS_COMMON_SUBEXPRESSION: &str =
    "optimizer.common_subexpression";

pub const PASS_LOOP_OPTIMIZATION: &str =
    "optimizer.loop_optimization";

pub const PASS_VERIFICATION: &str = "compiler.verification";

pub const PASS_IR_VALIDATION: &str = "compiler.ir_validation";

fn validate_pass_identifier(identifier: &str) -> ProfileResult<()> {
    if identifier.trim() != identifier || !valid_identifier(identifier) {
        return Err(ProfileError::InvalidPassIdentifier {
            identifier: identifier.to_owned(),
        });
    }

    Ok(())
}

// ============================================================================
// PROFILE SPECIFICATION
// ============================================================================

/// Immutable, versioned compilation-profile specification.
///
/// This is the compiler-side projection of resolved profile intent.
///
/// It is not a replacement for the richer source-level profile model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompilationProfileSpec {
    schema_version: u32,

    id: CompilationProfileId,

    description: String,

    optimization_level: OptimizationLevel,

    effort: ProfileEffort,

    safety: ProfileSafety,

    requires_verification: bool,

    recommended_passes: Vec<String>,
}

impl CompilationProfileSpec {
    /// Constructs a specification using the current schema version.
    pub fn new(
        id: CompilationProfileId,
        description: impl Into<String>,
        optimization_level: OptimizationLevel,
        effort: ProfileEffort,
        safety: ProfileSafety,
        requires_verification: bool,
        recommended_passes: Vec<String>,
    ) -> ProfileResult<Self> {
        Self::new_versioned(
            PROFILE_SCHEMA_VERSION,
            id,
            description,
            optimization_level,
            effort,
            safety,
            requires_verification,
            recommended_passes,
        )
    }

    /// Constructs a specification with an explicit schema version.
    pub fn new_versioned(
        schema_version: u32,
        id: CompilationProfileId,
        description: impl Into<String>,
        optimization_level: OptimizationLevel,
        effort: ProfileEffort,
        safety: ProfileSafety,
        requires_verification: bool,
        recommended_passes: Vec<String>,
    ) -> ProfileResult<Self> {
        if schema_version != PROFILE_SCHEMA_VERSION {
            return Err(ProfileError::UnsupportedSchemaVersion {
                supplied: schema_version,
                supported: PROFILE_SCHEMA_VERSION,
            });
        }

        let description = description.into();

        if description.trim().is_empty()
            || description.chars().any(char::is_control)
        {
            return Err(ProfileError::EmptyDescription);
        }

        let mut seen = BTreeSet::new();

        for identifier in &recommended_passes {
            validate_pass_identifier(identifier)?;

            if !seen.insert(identifier.as_str()) {
                return Err(ProfileError::DuplicatePassIdentifier {
                    identifier: identifier.clone(),
                });
            }
        }

        if safety == ProfileSafety::Verified && !requires_verification {
            return Err(ProfileError::VerificationRequired {
                profile: id.to_string(),
            });
        }

        Ok(Self {
            schema_version,
            id,
            description,
            optimization_level,
            effort,
            safety,
            requires_verification,
            recommended_passes,
        })
    }

    #[must_use]
    pub const fn schema_version(&self) -> u32 {
        self.schema_version
    }

    #[must_use]
    pub fn id(&self) -> &CompilationProfileId {
        &self.id
    }

    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }

    #[must_use]
    pub fn optimization_level(&self) -> &OptimizationLevel {
        &self.optimization_level
    }

    #[must_use]
    pub const fn effort(&self) -> ProfileEffort {
        self.effort
    }

    #[must_use]
    pub const fn safety(&self) -> ProfileSafety {
        self.safety
    }

    #[must_use]
    pub const fn requires_verification(&self) -> bool {
        self.requires_verification
    }

    #[must_use]
    pub fn recommended_passes(&self) -> &[String] {
        &self.recommended_passes
    }

    #[must_use]
    pub fn recommends_pass(&self, identifier: &str) -> bool {
        self.recommended_passes
            .iter()
            .any(|candidate| candidate == identifier)
    }

    #[must_use]
    pub const fn is_optimization_intensive(&self) -> bool {
        matches!(self.effort, ProfileEffort::High | ProfileEffort::VeryHigh)
    }

    #[must_use]
    pub const fn is_verified(&self) -> bool {
        matches!(self.safety, ProfileSafety::Verified)
    }

    /// Applies profile intent to an existing compiler configuration.
    ///
    /// The caller's target and unrelated settings are preserved.
    ///
    /// This operation does not discover or select hardware.
    pub fn apply_to(&self, base: &CompilerConfig) -> CompilerConfig {
        let mut resolved = base.clone();

        resolved.opt_level = self.optimization_level.clone();

        if self.requires_verification {
            resolved.verify = true;
        }

        resolved
    }

    /// Applies profile intent with an explicit optimization override.
    ///
    /// Mandatory verification cannot be disabled by this operation.
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

// ============================================================================
// REGISTRY
// ============================================================================

/// Deterministic, instance-owned profile registry.
///
/// No global mutable registry is used.
#[derive(Debug, Clone, Default)]
pub struct CompilationProfileRegistry {
    profiles: BTreeMap<CompilationProfileId, CompilationProfileSpec>,
}

impl CompilationProfileRegistry {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a registry containing the standard profiles.
    pub fn with_builtins() -> Self {
        let mut registry = Self::new();

        for specification in builtin_profiles() {
            registry
                .register(specification)
                .expect("built-in profile definitions must be valid");
        }

        registry
    }

    /// Registers a profile without silently replacing an existing entry.
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

    /// Replaces an existing profile.
    ///
    /// Importantly, a failed replacement leaves the registry unchanged.
    pub fn replace(
        &mut self,
        profile: CompilationProfileSpec,
    ) -> ProfileResult<CompilationProfileSpec> {
        let identifier = profile.id().clone();

        if !self.profiles.contains_key(&identifier) {
            return Err(ProfileError::ProfileNotRegistered {
                identifier: identifier.to_string(),
            });
        }

        match self.profiles.insert(identifier, profile) {
            Some(previous) => Ok(previous),
            None => unreachable!("existing profile was checked before replacement"),
        }
    }

    #[must_use]
    pub fn get(
        &self,
        identifier: &CompilationProfileId,
    ) -> Option<&CompilationProfileSpec> {
        self.profiles.get(identifier)
    }

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

    #[must_use]
    pub fn contains(&self, identifier: &CompilationProfileId) -> bool {
        self.profiles.contains_key(identifier)
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.profiles.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.profiles.is_empty()
    }

    pub fn iter(
        &self,
    ) -> btree_map::Iter<'_, CompilationProfileId, CompilationProfileSpec> {
        self.profiles.iter()
    }

    pub fn remove(
        &mut self,
        identifier: &CompilationProfileId,
    ) -> Option<CompilationProfileSpec> {
        self.profiles.remove(identifier)
    }

    /// Resolves a profile against caller-supplied configuration.
    pub fn resolve_config(
        &self,
        identifier: &str,
        base: &CompilerConfig,
    ) -> ProfileResult<ResolvedCompilationProfile> {
        self.resolve_config_with_override(identifier, base, None)
    }

    /// Resolves a profile with an explicit optimization override.
    pub fn resolve_config_with_override(
        &self,
        identifier: &str,
        base: &CompilerConfig,
        optimization_override: Option<OptimizationLevel>,
    ) -> ProfileResult<ResolvedCompilationProfile> {
        let specification = self.get_by_name(identifier)?;

        let config = specification.apply_with_optimization_override(
            base,
            optimization_override,
        );

        validate_profile_compatibility(specification, &config)?;

        Ok(ResolvedCompilationProfile {
            profile_id: specification.id().clone(),
            schema_version: specification.schema_version(),
            config,
            recommended_passes: specification.recommended_passes().to_vec(),
            effort: specification.effort(),
            safety: specification.safety(),
        })
    }
}

// ============================================================================
// RESOLVED PROFILE
// ============================================================================

/// Immutable resolved profile snapshot.
#[derive(Debug, Clone)]
pub struct ResolvedCompilationProfile {
    profile_id: CompilationProfileId,

    schema_version: u32,

    config: CompilerConfig,

    recommended_passes: Vec<String>,

    effort: ProfileEffort,

    safety: ProfileSafety,
}

impl ResolvedCompilationProfile {
    #[must_use]
    pub fn profile_id(&self) -> &CompilationProfileId {
        &self.profile_id
    }

    #[must_use]
    pub const fn schema_version(&self) -> u32 {
        self.schema_version
    }

    #[must_use]
    pub fn config(&self) -> &CompilerConfig {
        &self.config
    }

    #[must_use]
    pub fn into_config(self) -> CompilerConfig {
        self.config
    }

    #[must_use]
    pub fn recommended_passes(&self) -> &[String] {
        &self.recommended_passes
    }

    #[must_use]
    pub const fn effort(&self) -> ProfileEffort {
        self.effort
    }

    #[must_use]
    pub const fn safety(&self) -> ProfileSafety {
        self.safety
    }
}

// ============================================================================
// COMPATIBILITY
// ============================================================================

/// Validates a resolved profile using the existing configuration authority.
///
/// This does not prove target availability, backend compatibility,
/// pass implementation, semantic preservation or physical feasibility.
pub fn validate_profile_compatibility(
    profile: &CompilationProfileSpec,
    config: &CompilerConfig,
) -> ProfileResult<()> {
    if profile.requires_verification() && !config.verify {
        return Err(ProfileError::VerificationRequired {
            profile: profile.id().to_string(),
        });
    }

    validate_compiler_config(
        config,
        ConfigurationValidationMode::Production,
    )
    .map_err(|error| ProfileError::InvalidConfiguration {
        message: error.to_string(),
    })
}

// ============================================================================
// BUILT-IN PROFILES
// ============================================================================

fn profile(
    identifier: &str,
    description: &str,
    optimization_level: OptimizationLevel,
    effort: ProfileEffort,
    safety: ProfileSafety,
    recommended_passes: &[&str],
) -> CompilationProfileSpec {
    CompilationProfileSpec::new(
        CompilationProfileId::new(identifier)
            .expect("built-in identifiers are valid"),
        description,
        optimization_level,
        effort,
        safety,
        true,
        recommended_passes
            .iter()
            .map(|identifier| (*identifier).to_owned())
            .collect(),
    )
    .expect("built-in profile specifications are valid")
}

fn builtin_profiles() -> Vec<CompilationProfileSpec> {
    vec![
        profile(
            "generic",
            "General-purpose portable compilation.",
            OptimizationLevel::Basic,
            ProfileEffort::Moderate,
            ProfileSafety::Exact,
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
            &[
                PASS_NORMALIZE,
                PASS_CONSTANT_FOLD,
                PASS_IR_VALIDATION,
                PASS_VERIFICATION,
            ],
        ),
    ]
}

// ============================================================================
// CONVENIENCE API
// ============================================================================

#[must_use]
pub fn standard_profile_registry() -> CompilationProfileRegistry {
    CompilationProfileRegistry::with_builtins()
}

pub fn resolve_standard_profile(
    identifier: &str,
    base: &CompilerConfig,
) -> ProfileResult<ResolvedCompilationProfile> {
    standard_profile_registry().resolve_config(identifier, base)
}

#[must_use]
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

    fn custom_profile(identifier: &str) -> CompilationProfileSpec {
        CompilationProfileSpec::new(
            CompilationProfileId::new(identifier).unwrap(),
            "Custom test profile",
            OptimizationLevel::Basic,
            ProfileEffort::Moderate,
            ProfileSafety::Exact,
            true,
            vec![PASS_NORMALIZE.to_owned()],
        )
        .unwrap()
    }

    #[test]
    fn identifiers_are_open_world() {
        assert!(CompilationProfileId::new("organization.ai.profile").is_ok());
        assert!(CompilationProfileId::new("future:quantum:profile").is_ok());
    }

    #[test]
    fn identifiers_have_no_arbitrary_length_ceiling() {
        let value = format!("future.{}", "x".repeat(10_000));
        assert!(CompilationProfileId::new(value).is_ok());
    }

    #[test]
    fn identifiers_reject_invalid_characters() {
        assert!(CompilationProfileId::new("").is_err());
        assert!(CompilationProfileId::new("Balanced").is_err());
        assert!(CompilationProfileId::new("bad profile").is_err());
    }

    #[test]
    fn schema_versions_are_validated() {
        assert!(CompilationProfileSpec::new_versioned(
            PROFILE_SCHEMA_VERSION + 1,
            CompilationProfileId::new("future.version").unwrap(),
            "Version test",
            OptimizationLevel::Basic,
            ProfileEffort::Moderate,
            ProfileSafety::Exact,
            true,
            Vec::new(),
        ).is_err());
    }

    #[test]
    fn standard_profiles_are_available() {
        let registry = standard_profile_registry();

        for name in [
            "generic",
            "fast_compile",
            "balanced",
            "aggressive",
            "debug",
            "verified",
        ] {
            assert!(registry.get_by_name(name).is_ok());
        }
    }

    #[test]
    fn registry_iteration_is_deterministic() {
        let registry = standard_profile_registry();

        let identifiers: Vec<_> = registry
            .iter()
            .map(|(id, _)| id.as_str())
            .collect();

        let mut sorted = identifiers.clone();
        sorted.sort_unstable();

        assert_eq!(identifiers, sorted);
    }

    #[test]
    fn duplicate_registration_is_rejected() {
        let mut registry = CompilationProfileRegistry::new();

        registry.register(custom_profile("custom.test")).unwrap();

        assert!(matches!(
            registry.register(custom_profile("custom.test")),
            Err(ProfileError::DuplicateProfile { .. })
        ));
    }

    #[test]
    fn failed_replacement_does_not_mutate_registry() {
        let mut registry = CompilationProfileRegistry::new();

        let result = registry.replace(custom_profile("missing.profile"));

        assert!(matches!(
            result,
            Err(ProfileError::ProfileNotRegistered { .. })
        ));

        assert!(registry.is_empty());
    }

    #[test]
    fn successful_replacement_returns_previous_profile() {
        let mut registry = CompilationProfileRegistry::new();

        registry.register(custom_profile("custom.replace")).unwrap();

        let replacement = CompilationProfileSpec::new(
            CompilationProfileId::new("custom.replace").unwrap(),
            "Replacement",
            OptimizationLevel::Aggressive,
            ProfileEffort::High,
            ProfileSafety::Exact,
            true,
            vec![PASS_NORMALIZE.to_owned()],
        ).unwrap();

        let previous = registry.replace(replacement).unwrap();

        assert_eq!(previous.id().as_str(), "custom.replace");
    }

    #[test]
    fn unknown_profiles_are_rejected() {
        assert!(matches!(
            standard_profile_registry().get_by_name("unknown"),
            Err(ProfileError::UnknownProfile { .. })
        ));
    }

    #[test]
    fn profile_preserves_target_and_unrelated_configuration() {
        let base = base_config();

        let resolved = resolve_standard_profile("balanced", &base).unwrap();

        assert_eq!(resolved.config().target, base.target);
        assert_eq!(resolved.config().debug_info, base.debug_info);
        assert_eq!(resolved.config().emit_ir, base.emit_ir);
        assert_eq!(resolved.config().parallel, base.parallel);
    }

    #[test]
    fn mandatory_verification_cannot_be_disabled() {
        let base = base_config();

        let resolved = resolve_standard_profile("verified", &base).unwrap();

        assert!(resolved.config().verify);
        assert_eq!(resolved.safety(), ProfileSafety::Verified);
    }

    #[test]
    fn optimization_override_is_supported() {
        let registry = standard_profile_registry();

        let resolved = registry.resolve_config_with_override(
            "balanced",
            &base_config(),
            Some(OptimizationLevel::None),
        ).unwrap();

        assert_eq!(
            resolved.config().opt_level,
            OptimizationLevel::None
        );
    }

    #[test]
    fn required_verification_cannot_be_bypassed_by_override() {
        let registry = standard_profile_registry();

        let resolved = registry.resolve_config_with_override(
            "verified",
            &base_config(),
            Some(OptimizationLevel::Basic),
        ).unwrap();

        assert!(resolved.config().verify);
    }

    #[test]
    fn custom_pass_names_are_supported() {
        let profile = custom_profile("organization.ai.custom");

        assert!(profile.recommends_pass(PASS_NORMALIZE));
    }

    #[test]
    fn duplicate_pass_names_are_rejected() {
        let result = CompilationProfileSpec::new(
            CompilationProfileId::new("custom.duplicate").unwrap(),
            "Duplicate test",
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
    fn verified_safety_requires_verification() {
        let result = CompilationProfileSpec::new(
            CompilationProfileId::new("custom.verified").unwrap(),
            "Verified test",
            OptimizationLevel::Basic,
            ProfileEffort::High,
            ProfileSafety::Verified,
            false,
            Vec::new(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn custom_profiles_do_not_share_global_state() {
        let mut first = CompilationProfileRegistry::new();
        let second = CompilationProfileRegistry::new();

        first.register(custom_profile("custom.isolated")).unwrap();

        assert!(first.get_by_name("custom.isolated").is_ok());
        assert!(second.get_by_name("custom.isolated").is_err());
    }

    #[test]
    fn generic_profile_is_target_independent() {
        let resolved = resolve_standard_profile("generic", &base_config()).unwrap();

        assert_eq!(
            resolved.config().target,
            base_config().target
        );
    }
}
