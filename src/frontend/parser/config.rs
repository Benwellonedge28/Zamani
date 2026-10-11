
#![forbid(unsafe_code)]

//! Configuration contracts for the Zamani frontend parser.
//!
//! File: src/frontend/parser/config.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Define deterministic, target-independent configuration for parsing Zamani
//! source and token streams. The configuration is shared by parser entry
//! points, sessions, grammar productions, extension dispatch, and recovery.
//!
//! # Architectural boundary
//!
//! This module owns:
//! - language-version selection;
//! - strict versus recovery-oriented parsing policy;
//! - parser resource budgets;
//! - extension activation policy;
//! - compatibility behavior;
//! - validation of configuration values;
//! - documented defaults and builder methods.
//!
//! This module does not own:
//! - source text, source maps, tokens, token kinds, or source spans;
//! - lexical scanning, keyword recognition, or operator spelling;
//! - grammar productions, precedence tables, or parse state;
//! - AST definitions or AST construction;
//! - name resolution, type checking, effects, or contract evaluation;
//! - quantum operations, HDL primitives, hardware discovery, or scheduling;
//! - target selection, resource allocation, or runtime execution.
//!
//! # POCO-REAF and scalability
//!
//! Parser limits are implementation resource budgets, not language limits.
//! The default configuration does not impose explicit parser budgets.
//!
//! A caller may set finite budgets for untrusted source, interactive tools,
//! embedded environments, or other operational requirements. Exhaustion must
//! be reported by the component enforcing the budget; it must not silently
//! change the meaning of a program.
//!
//! No limit in this module represents a maximum number of qubits, CPUs,
//! threads, GPUs, FPGAs, devices, nodes, registers, tensor dimensions,
//! declarations, or computational operations.
//!
//! "Unlimited" means no explicit limit is configured for that dimension.
//! It does not promise unlimited memory, address space, stack, or execution
//! time. Implementations should prefer iterative algorithms where practical
//! and use configurable budgets when processing untrusted input.
//!
//! # Integration contract
//!
//! - `super::super::lexer::config::ResourceLimit` is the canonical resource
//!   limit type. This module reuses it rather than defining a competing type.
//! - `limits.rs` enforces `ParserLimits`; it does not redefine configuration.
//! - `context.rs` holds or shares the immutable `ParserConfig`.
//! - `session.rs` owns per-invocation mutable state and budget accounting.
//! - `token_cursor.rs` consumes canonical lexer tokens and enforces applicable
//!   token-consumption budgets.
//! - `recovery.rs` applies the selected recovery mode and recovery budgets.
//! - `extensions.rs` and `registry.rs` enforce the configured extension policy.
//! - `compatibility.rs` applies language-version and compatibility policy.
//! - `api.rs` validates configuration before starting a parse.
//! - Grammar modules consume configuration through the parser context/session;
//!   they must not create private, contradictory configuration defaults.
//! - Semantic analysis, IR lowering, and target realization interpret neither
//!   parser modes nor parser budgets as language-level resource constraints.
//!
//! The configuration is intentionally independent of grammar modules so that
//! grammar work can proceed without changing this file for every new domain.

use core::fmt;
use core::num::NonZeroUsize;
use std::collections::BTreeSet;

use super::super::lexer::config::ResourceLimit;

/// A language version requested by the caller.
///
/// The version identifies a language specification/compatibility edition.
/// It is not a compiler version or target-hardware version.
///
/// The parser API or compatibility layer must decide which versions are
/// implemented. This value does not itself imply that a version is supported.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LanguageVersion {
    /// Major language-version component.
    pub major: u32,

    /// Minor language-version component.
    pub minor: u32,

    /// Patch language-version component.
    pub patch: u32,
}

impl LanguageVersion {
    /// Creates a language version.
    #[must_use]
    pub const fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }
}

impl fmt::Display for LanguageVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

/// Controls whether parsing stops at syntax errors or attempts recovery.
///
/// Recovery never means silently treating malformed syntax as valid.
/// Recovered parses must retain diagnostics and indicate that recovery occurred.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ParseMode {
    /// Stop after an unrecoverable syntax error.
    ///
    /// A parser may still report diagnostics discovered before termination.
    Strict,

    /// Attempt bounded, deterministic recovery after syntax errors.
    ///
    /// The result must distinguish a recovered parse from an error-free parse.
    #[default]
    Recovering,
}

/// Controls which registered grammar extensions may participate in parsing.
///
/// Extension implementation and registration are owned by the extension
/// registry, not by this configuration type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ExtensionPolicy {
    /// Permit only extensions explicitly enabled by the caller.
    ///
    /// An empty enabled-extension set means no non-core extension is enabled.
    #[default]
    Explicit,

    /// Permit every extension available in the supplied registry, subject
    /// to registry validation, version compatibility, and conflict checks.
    ///
    /// This does not load code, bypass safety checks, or authorize execution.
    AllowRegistered,
}

/// Controls how syntax from older or differing language editions is handled.
///
/// The compatibility layer owns the actual mapping and diagnostics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum CompatibilityMode {
    /// Accept syntax supported by the selected language version and reject
    /// syntax that is not supported by that version.
    #[default]
    Versioned,

    /// Permit explicitly implemented compatibility mappings for older syntax.
    ///
    /// This mode must not silently reinterpret syntax in a way that changes
    /// program meaning.
    AllowLegacy,

    /// Reject deprecated syntax even if a compatibility mapping exists.
    ForbidDeprecated,
}

/// Configurable implementation budgets for one parse operation.
///
/// Each field measures a distinct resource. `ResourceLimit::Unlimited`
/// disables the corresponding explicit budget. Enforcement and structured
/// budget-exhaustion diagnostics belong to the parser runtime.
///
/// Budgets are not restrictions on the language or the machine on which the
/// resulting program may execute.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ParserLimits {
    /// Maximum number of tokens consumed by one parse operation.
    ///
    /// The cursor and parser must use a consistent token-counting convention.
    pub tokens_consumed: ResourceLimit,

    /// Maximum parser-recursion or grammar-nesting depth explicitly tracked
    /// by the implementation.
    ///
    /// This is a safety budget, not a language-level nesting restriction.
    /// It does not protect untracked recursive call paths automatically.
    pub nesting_depth: ResourceLimit,

    /// Maximum number of speculative parsing steps.
    ///
    /// A step is an implementation-defined unit that must be documented by
    /// the parser runtime and counted consistently.
    pub speculative_steps: ResourceLimit,

    /// Maximum number of recovery operations performed by one parse.
    ///
    /// Recovery must also guarantee progress independently of this budget.
    pub recovery_operations: ResourceLimit,

    /// Maximum number of diagnostics retained for one parse.
    ///
    /// When exhausted, the result must expose that diagnostics were truncated
    /// or that parsing stopped because the budget was reached.
    pub diagnostics: ResourceLimit,

    /// Maximum number of AST or intermediate syntax nodes constructed during
    /// one parse, when the AST adapter can account for them reliably.
    ///
    /// This budget must not be used to estimate the number of operations
    /// that a compiled program may execute.
    pub syntax_nodes: ResourceLimit,

    /// Maximum number of parser work units.
    ///
    /// The runtime must document which operations count as work units.
    /// This budget is useful for bounding pathological inputs even when
    /// individual input dimensions are not bounded.
    pub work_units: ResourceLimit,
}

impl ParserLimits {
    /// Creates a set of parser budgets without explicit ceilings.
    ///
    /// Use finite budgets when the caller needs an operational safeguard,
    /// particularly when processing untrusted input.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            tokens_consumed: ResourceLimit::Unlimited,
            nesting_depth: ResourceLimit::Unlimited,
            speculative_steps: ResourceLimit::Unlimited,
            recovery_operations: ResourceLimit::Unlimited,
            diagnostics: ResourceLimit::Unlimited,
            syntax_nodes: ResourceLimit::Unlimited,
            work_units: ResourceLimit::Unlimited,
        }
    }

    /// Creates a bounded policy for the most common input and recovery costs.
    ///
    /// All other dimensions remain unlimited and can be configured separately.
    #[must_use]
    pub const fn bounded(
        tokens_consumed: NonZeroUsize,
        nesting_depth: NonZeroUsize,
        diagnostics: NonZeroUsize,
    ) -> Self {
        Self {
            tokens_consumed: ResourceLimit::AtMost(tokens_consumed),
            nesting_depth: ResourceLimit::AtMost(nesting_depth),
            speculative_steps: ResourceLimit::Unlimited,
            recovery_operations: ResourceLimit::Unlimited,
            diagnostics: ResourceLimit::AtMost(diagnostics),
            syntax_nodes: ResourceLimit::Unlimited,
            work_units: ResourceLimit::Unlimited,
        }
    }
}

/// Immutable parser policy for one or more parse operations.
///
/// The configuration contains no source text, cursor, diagnostics, AST,
/// registry implementation, device information, or mutable parse state.
///
/// `enabled_extensions` contains stable extension identifiers. The registry
/// is responsible for resolving identifiers and validating their versions.
/// Identifiers are treated as opaque strings here; no domain or vendor list
/// is hard-coded in this module.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParserConfig {
    /// Requested language specification version.
    ///
    /// `None` delegates the default version to the parser API or compatibility
    /// layer. This module does not invent a current language version.
    pub language_version: Option<LanguageVersion>,

    /// Syntax-error behavior.
    pub mode: ParseMode,

    /// Parser resource budgets.
    pub limits: ParserLimits,

    /// Policy for registered syntax extensions.
    pub extension_policy: ExtensionPolicy,

    /// Extension identifiers explicitly permitted by this configuration.
    ///
    /// A deterministic ordered set is used so configuration inspection and
    /// diagnostics do not depend on hash iteration order.
    pub enabled_extensions: BTreeSet<String>,

    /// Legacy/deprecated syntax policy.
    pub compatibility: CompatibilityMode,

    /// Whether syntax explicitly marked experimental by the registry/spec
    /// may be parsed when its extension is enabled.
    ///
    /// This does not promote experimental syntax to stable language syntax.
    pub allow_experimental_syntax: bool,

    /// Whether parsing must consume the complete significant-token stream.
    ///
    /// For normal compilation entry points this should remain enabled.
    /// Specialized fragment parsers may implement their own termination
    /// contract instead of disabling this for whole-program parsing.
    pub require_eof: bool,
}

impl ParserConfig {
    /// Creates the default parser configuration.
    ///
    /// Defaults deliberately avoid choosing a language edition that may not
    /// yet be supported by the implementation. The public parse API must
    /// resolve `language_version: None` against its supported-version policy.
    #[must_use]
    pub fn new() -> Self {
        Self {
            language_version: None,
            mode: ParseMode::Recovering,
            limits: ParserLimits::unlimited(),
            extension_policy: ExtensionPolicy::Explicit,
            enabled_extensions: BTreeSet::new(),
            compatibility: CompatibilityMode::Versioned,
            allow_experimental_syntax: false,
            require_eof: true,
        }
    }

    /// Creates a configuration builder.
    #[must_use]
    pub fn builder() -> ParserConfigBuilder {
        ParserConfigBuilder {
            config: Self::new(),
        }
    }

    /// Sets the requested language version.
    #[must_use]
    pub fn with_language_version(mut self, version: LanguageVersion) -> Self {
        self.language_version = Some(version);
        self
    }

    /// Clears the explicitly requested language version.
    ///
    /// The public parser API then resolves the version using its documented
    /// default-version policy.
    #[must_use]
    pub fn with_default_language_version(mut self) -> Self {
        self.language_version = None;
        self
    }

    /// Sets strict or recovery-oriented parsing.
    #[must_use]
    pub const fn with_mode(mut self, mode: ParseMode) -> Self {
        self.mode = mode;
        self
    }

    /// Replaces all parser resource budgets.
    #[must_use]
    pub const fn with_limits(mut self, limits: ParserLimits) -> Self {
        self.limits = limits;
        self
    }

    /// Sets the extension activation policy.
    #[must_use]
    pub const fn with_extension_policy(mut self, policy: ExtensionPolicy) -> Self {
        self.extension_policy = policy;
        self
    }

    /// Enables one extension identifier.
    ///
    /// An empty identifier is rejected by `validate()`. This method does not
    /// register or load an extension.
    #[must_use]
    pub fn with_enabled_extension(
        mut self,
        extension_id: impl Into<String>,
    ) -> Self {
        self.enabled_extensions.insert(extension_id.into());
        self
    }

    /// Replaces the explicitly enabled extension set.
    #[must_use]
    pub fn with_enabled_extensions(
        mut self,
        extension_ids: impl IntoIterator<Item = String>,
    ) -> Self {
        self.enabled_extensions = extension_ids.into_iter().collect();
        self
    }

    /// Sets compatibility behavior.
    #[must_use]
    pub const fn with_compatibility(mut self, mode: CompatibilityMode) -> Self {
        self.compatibility = mode;
        self
    }

    /// Enables or disables registered experimental syntax.
    #[must_use]
    pub const fn with_experimental_syntax(mut self, allow: bool) -> Self {
        self.allow_experimental_syntax = allow;
        self
    }

    /// Sets whether a whole-input parse must reach EOF.
    #[must_use]
    pub const fn with_require_eof(mut self, required: bool) -> Self {
        self.require_eof = required;
        self
    }

    /// Validates configuration-level invariants.
    ///
    /// This method validates only values owned by this configuration.
    /// Whether a language version or extension is implemented is validated
    /// by the API/registry that has access to that information.
    pub fn validate(&self) -> Result<(), ParserConfigError> {
        for extension_id in &self.enabled_extensions {
            if extension_id.trim().is_empty() {
                return Err(ParserConfigError::EmptyExtensionId);
            }

            if extension_id.chars().any(char::is_whitespace) {
                return Err(ParserConfigError::WhitespaceInExtensionId {
                    extension_id: extension_id.clone(),
                });
            }
        }

        if self.extension_policy == ExtensionPolicy::AllowRegistered
            && !self.enabled_extensions.is_empty()
        {
            return Err(ParserConfigError::ConflictingExtensionPolicies);
        }

        Ok(())
    }
}

impl Default for ParserConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Incremental builder for [`ParserConfig`].
///
/// The builder is a value-only object. It does not inspect source files,
/// load extensions, initialize hardware, or mutate global parser state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParserConfigBuilder {
    config: ParserConfig,
}

impl ParserConfigBuilder {
    /// Sets the language version.
    #[must_use]
    pub fn language_version(mut self, version: LanguageVersion) -> Self {
        self.config.language_version = Some(version);
        self
    }

    /// Uses the parser API's default language-version policy.
    #[must_use]
    pub fn default_language_version(mut self) -> Self {
        self.config.language_version = None;
        self
    }

    /// Sets parser mode.
    #[must_use]
    pub const fn mode(mut self, mode: ParseMode) -> Self {
        self.config.mode = mode;
        self
    }

    /// Sets resource budgets.
    #[must_use]
    pub const fn limits(mut self, limits: ParserLimits) -> Self {
        self.config.limits = limits;
        self
    }

    /// Sets extension activation policy.
    #[must_use]
    pub const fn extension_policy(mut self, policy: ExtensionPolicy) -> Self {
        self.config.extension_policy = policy;
        self
    }

    /// Adds an explicitly enabled extension identifier.
    #[must_use]
    pub fn enable_extension(mut self, extension_id: impl Into<String>) -> Self {
        self.config.enabled_extensions.insert(extension_id.into());
        self
    }

    /// Replaces the explicitly enabled extension set.
    #[must_use]
    pub fn enabled_extensions(
        mut self,
        extension_ids: impl IntoIterator<Item = String>,
    ) -> Self {
        self.config.enabled_extensions = extension_ids.into_iter().collect();
        self
    }

    /// Sets compatibility behavior.
    #[must_use]
    pub const fn compatibility(mut self, mode: CompatibilityMode) -> Self {
        self.config.compatibility = mode;
        self
    }

    /// Enables or disables experimental syntax.
    #[must_use]
    pub const fn allow_experimental_syntax(mut self, allow: bool) -> Self {
        self.config.allow_experimental_syntax = allow;
        self
    }

    /// Sets the whole-input EOF requirement.
    #[must_use]
    pub const fn require_eof(mut self, required: bool) -> Self {
        self.config.require_eof = required;
        self
    }

    /// Validates and returns the completed configuration.
    pub fn build(self) -> Result<ParserConfig, ParserConfigError> {
        self.config.validate()?;
        Ok(self.config)
    }
}

/// Configuration construction or validation failure.
///
/// These errors describe invalid configuration values or combinations.
/// They are distinct from source syntax errors and parser resource exhaustion.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParserConfigError {
    /// An explicitly enabled extension identifier is empty.
    EmptyExtensionId,

    /// An extension identifier contains whitespace.
    WhitespaceInExtensionId {
        /// The invalid identifier.
        extension_id: String,
    },

    /// `AllowRegistered` was selected together with an explicit allow-list.
    ///
    /// Choose either an explicit allow-list or all registered extensions.
    ConflictingExtensionPolicies,
}

impl fmt::Display for ParserConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyExtensionId => {
                f.write_str("extension identifier must not be empty")
            }
            Self::WhitespaceInExtensionId { extension_id } => {
                write!(
                    f,
                    "extension identifier must not contain whitespace: {extension_id:?}"
                )
            }
            Self::ConflictingExtensionPolicies => f.write_str(
                "allow-registered extension policy cannot be combined with an explicit extension allow-list",
            ),
        }
    }
}

impl std::error::Error for ParserConfigError {}

#[cfg(test)]
mod tests {
    use super::{
        CompatibilityMode, ExtensionPolicy, LanguageVersion, ParseMode, ParserConfig,
        ParserConfigError, ParserLimits,
    };
    use crate::frontend::lexer::config::ResourceLimit;
    use core::num::NonZeroUsize;

    #[test]
    fn default_configuration_is_deterministic() {
        assert_eq!(ParserConfig::default(), ParserConfig::new());

        let config = ParserConfig::new();
        assert_eq!(config.language_version, None);
        assert_eq!(config.mode, ParseMode::Recovering);
        assert_eq!(config.limits, ParserLimits::unlimited());
        assert_eq!(config.extension_policy, ExtensionPolicy::Explicit);
        assert!(config.enabled_extensions.is_empty());
        assert_eq!(config.compatibility, CompatibilityMode::Versioned);
        assert!(!config.allow_experimental_syntax);
        assert!(config.require_eof);
        assert!(config.validate().is_ok());
    }

    #[test]
    fn unlimited_parser_budgets_do_not_impose_explicit_ceilings() {
        let limits = ParserLimits::unlimited();

        assert!(limits.tokens_consumed.permits(usize::MAX));
        assert!(limits.nesting_depth.permits(usize::MAX));
        assert!(limits.speculative_steps.permits(usize::MAX));
        assert!(limits.recovery_operations.permits(usize::MAX));
        assert!(limits.diagnostics.permits(usize::MAX));
        assert!(limits.syntax_nodes.permits(usize::MAX));
        assert!(limits.work_units.permits(usize::MAX));
    }

    #[test]
    fn bounded_budgets_are_inclusive_and_independent() {
        let limits = ParserLimits::bounded(
            NonZeroUsize::new(100).expect("nonzero"),
            NonZeroUsize::new(20).expect("nonzero"),
            NonZeroUsize::new(10).expect("nonzero"),
        );

        assert!(limits.tokens_consumed.permits(100));
        assert!(!limits.tokens_consumed.permits(101));
        assert!(limits.nesting_depth.permits(20));
        assert!(!limits.nesting_depth.permits(21));
        assert!(limits.diagnostics.permits(10));
        assert!(!limits.diagnostics.permits(11));

        assert_eq!(limits.speculative_steps, ResourceLimit::Unlimited);
        assert_eq!(limits.recovery_operations, ResourceLimit::Unlimited);
        assert_eq!(limits.syntax_nodes, ResourceLimit::Unlimited);
        assert_eq!(limits.work_units, ResourceLimit::Unlimited);
    }

    #[test]
    fn language_version_has_stable_display() {
        assert_eq!(LanguageVersion::new(1, 2, 3).to_string(), "1.2.3");
    }

    #[test]
    fn builder_composes_independent_policies() {
        let config = ParserConfig::builder()
            .language_version(LanguageVersion::new(1, 0, 0))
            .mode(ParseMode::Strict)
            .compatibility(CompatibilityMode::AllowLegacy)
            .allow_experimental_syntax(true)
            .require_eof(true)
            .enable_extension("vendor.quantum")
            .build()
            .expect("configuration should be valid");

        assert_eq!(config.language_version, Some(LanguageVersion::new(1, 0, 0)));
        assert_eq!(config.mode, ParseMode::Strict);
        assert_eq!(config.compatibility, CompatibilityMode::AllowLegacy);
        assert!(config.allow_experimental_syntax);
        assert!(config.enabled_extensions.contains("vendor.quantum"));
    }

    #[test]
    fn empty_extension_identifier_is_rejected() {
        let result = ParserConfig::builder()
            .enable_extension(" ")
            .build();

        assert_eq!(result, Err(ParserConfigError::EmptyExtensionId));
    }

    #[test]
    fn whitespace_in_extension_identifier_is_rejected() {
        let result = ParserConfig::builder()
            .enable_extension("vendor quantum")
            .build();

        assert_eq!(
            result,
            Err(ParserConfigError::WhitespaceInExtensionId {
                extension_id: "vendor quantum".to_owned(),
            })
        );
    }

    #[test]
    fn allow_registered_conflicts_with_explicit_allow_list() {
        let result = ParserConfig::builder()
            .extension_policy(ExtensionPolicy::AllowRegistered)
            .enable_extension("vendor.quantum")
            .build();

        assert_eq!(
            result,
            Err(ParserConfigError::ConflictingExtensionPolicies)
        );
    }

    #[test]
    fn allow_registered_works_without_an_explicit_allow_list() {
        let config = ParserConfig::builder()
            .extension_policy(ExtensionPolicy::AllowRegistered)
            .build()
            .expect("policy should be valid");

        assert_eq!(config.extension_policy, ExtensionPolicy::AllowRegistered);
        assert!(config.enabled_extensions.is_empty());
    }

    #[test]
    fn fluent_methods_preserve_unrelated_settings() {
        let config = ParserConfig::new()
            .with_language_version(LanguageVersion::new(2, 1, 0))
            .with_mode(ParseMode::Strict)
            .with_compatibility(CompatibilityMode::ForbidDeprecated)
            .with_experimental_syntax(false)
            .with_require_eof(false);

        assert_eq!(config.language_version, Some(LanguageVersion::new(2, 1, 0)));
        assert_eq!(config.mode, ParseMode::Strict);
        assert_eq!(config.compatibility, CompatibilityMode::ForbidDeprecated);
        assert!(!config.allow_experimental_syntax);
        assert!(!config.require_eof);
    }

    #[test]
    fn extension_set_is_deterministic() {
        let first = ParserConfig::builder()
            .enable_extension("vendor.zeta")
            .enable_extension("vendor.alpha")
            .build()
            .expect("configuration should be valid");

        let second = ParserConfig::builder()
            .enable_extension("vendor.alpha")
            .enable_extension("vendor.zeta")
            .build()
            .expect("configuration should be valid");

        assert_eq!(first, second);
        assert_eq!(
            first.enabled_extensions.iter().map(String::as_str).collect::<Vec<_>>(),
            vec!["vendor.alpha", "vendor.zeta"]
        );
    }
}
