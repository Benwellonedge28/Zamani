
#![forbid(unsafe_code)]

//! Configuration contracts for the Zamani frontend lexer.
//!
//! File: src/frontend/lexer/config.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Define the stable, target-independent configuration accepted by the
//! Zamani lexical frontend.
//!
//! # Responsibilities
//!
//! This module owns:
//! - lexical resource-budget configuration;
//! - source, token, and diagnostic retention policies;
//! - EOF emission policy;
//! - Unicode validation policy;
//! - malformed-input recovery policy;
//! - configuration validation and documented defaults.
//!
//! This module does not own:
//! - token identities or spellings;
//! - keyword or operator registries;
//! - dialect registration;
//! - source cursor movement;
//! - scanning algorithms;
//! - diagnostic construction;
//! - parser or AST behavior;
//! - quantum, classical, or HDL semantics;
//! - target discovery or hardware resource allocation.
//!
//! # Architectural boundary
//!
//! ```text
//! LexerConfig
//!     |
//!     +--> scanner.rs
//!     +--> cursor.rs
//!     +--> identifier.rs
//!     +--> number.rs
//!     +--> string.rs
//!     +--> character.rs
//!     +--> comment.rs
//!     +--> quantum.rs
//!     +--> annotation.rs
//!     +--> whitespace.rs
//!     +--> diagnostics.rs
//!     +--> recovery.rs
//!     |
//!     v
//! api.rs --> parser / tooling
//! ```
//!
//! Configuration describes policy. The consuming component remains
//! responsible for enforcing that policy and reporting violations.
//!
//! # Scalability
//!
//! `ResourceLimit::Unlimited` means that no explicit limit is configured
//! for that resource. It does not guarantee that the host can satisfy an
//! arbitrarily large allocation or input.
//!
//! The implementation remains constrained by available memory, address
//! space, execution time, platform APIs, and other real resources.
//! Callers processing untrusted input should explicitly configure budgets.
//!
//! There are no language-level limits on source size, token count,
//! identifier length, numeric literal length, qubits, processor count,
//! memory capacity, tensor rank, device count, or topology size.
//!
//! A lexical resource budget MUST NOT be interpreted as a language-level
//! semantic restriction.
//!
//! # Integration contract
//!
//! - `limits.rs` may re-export or wrap the resource-budget definitions
//!   from this module, but must not create a competing configuration type.
//! - `scanner.rs` consumes `LexerConfig` and enforces the applicable
//!   source, token, and lexical-state budgets.
//! - Literal and identifier scanners enforce their applicable token
//!   budgets without assigning semantic meaning to literal values.
//! - `diagnostics.rs` reports budget violations using the canonical
//!   diagnostic representation.
//! - `recovery.rs` implements the configured recovery policy.
//! - `token.rs` and `token_kind.rs` own token representation and identity.
//! - `api.rs` exposes the scanner through `TokenSource`.
//! - Registries and dialects provide classification rules; they do not
//!   override the resource budgets.
//! - The parser and semantic layers do not reinterpret these limits as
//!   fixed language or hardware capacities.
//!
//! This module deliberately has no dependency on scanner implementation
//! details or on a particular computational domain.

use core::fmt;
use core::num::NonZeroUsize;

/// A configurable implementation resource limit.
///
/// `Unlimited` disables the corresponding explicit budget. It does not
/// disable operating-system limits, allocator failures, or other physical
/// constraints.
///
/// `AtMost` represents an inclusive upper bound.
///
/// The limit is an implementation policy, not a language-level maximum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResourceLimit {
    /// Do not impose an explicit limit for this resource.
    Unlimited,

    /// Permit at most this many units.
    AtMost(NonZeroUsize),
}

impl ResourceLimit {
    /// Creates a bounded limit.
    ///
    /// Returns `None` for zero because a zero budget is generally
    /// unsuitable for the resources represented by this configuration.
    #[must_use]
    pub const fn at_most(value: usize) -> Option<Self> {
        match NonZeroUsize::new(value) {
            Some(value) => Some(Self::AtMost(value)),
            None => None,
        }
    }

    /// Returns an unlimited resource policy.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self::Unlimited
    }

    /// Returns the configured bound, or `None` if unlimited.
    #[must_use]
    pub const fn bound(self) -> Option<NonZeroUsize> {
        match self {
            Self::Unlimited => None,
            Self::AtMost(value) => Some(value),
        }
    }

    /// Returns whether `observed` is within this inclusive budget.
    #[must_use]
    pub const fn permits(self, observed: usize) -> bool {
        match self {
            Self::Unlimited => true,
            Self::AtMost(bound) => observed <= bound.get(),
        }
    }

    /// Returns whether `observed` exceeds this budget.
    #[must_use]
    pub const fn is_exceeded_by(self, observed: usize) -> bool {
        !self.permits(observed)
    }

    /// Returns whether this policy has an explicit bound.
    #[must_use]
    pub const fn is_bounded(self) -> bool {
        matches!(self, Self::AtMost(_))
    }
}

impl Default for ResourceLimit {
    fn default() -> Self {
        Self::Unlimited
    }
}

impl fmt::Display for ResourceLimit {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unlimited => formatter.write_str("unlimited"),
            Self::AtMost(value) => write!(formatter, "at most {}", value),
        }
    }
}

/// Resource budgets enforced by the lexical implementation.
///
/// Each field measures a different resource. Consumers must not silently
/// substitute one field for another.
///
/// Byte-oriented limits count UTF-8 bytes, not Unicode scalar values,
/// grapheme clusters, display columns, or UTF-16 code units.
///
/// Token counts refer to emitted tokens according to the scanner's
/// documented counting convention. EOF accounting must be consistent
/// across the scanner and its tests.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct LexerLimits {
    /// Maximum number of source UTF-8 bytes accepted for one input.
    pub source_bytes: ResourceLimit,

    /// Maximum number of tokens emitted for one input.
    pub tokens: ResourceLimit,

    /// Maximum UTF-8 byte length of an individual source token.
    pub token_bytes: ResourceLimit,

    /// Maximum number of numeric-literal digits examined for one token.
    ///
    /// This budget covers lexical processing, not the representable
    /// precision or range of a downstream numeric type.
    pub numeric_digits: ResourceLimit,

    /// Maximum UTF-8 byte length of a string or character literal's
    /// source representation.
    ///
    /// Escape decoding and semantic value construction remain separate.
    pub literal_bytes: ResourceLimit,

    /// Maximum nesting depth for lexical constructs that the scanner
    /// explicitly models as nested, such as nested block comments.
    ///
    /// A scanner that does not support nesting for a construct must not
    /// use this field to imply that the language supports it.
    pub lexical_nesting_depth: ResourceLimit,

    /// Maximum number of diagnostics retained or emitted for one input.
    ///
    /// Once this budget is reached, the diagnostics implementation must
    /// follow its documented truncation/error-reporting contract.
    pub diagnostics: ResourceLimit,
}

impl LexerLimits {
    /// Creates an unlimited set of explicit lexer budgets.
    ///
    /// This is appropriate for trusted workloads where the caller manages
    /// resource availability. For untrusted input, use `bounded` or set
    /// the relevant fields explicitly.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            source_bytes: ResourceLimit::Unlimited,
            tokens: ResourceLimit::Unlimited,
            token_bytes: ResourceLimit::Unlimited,
            numeric_digits: ResourceLimit::Unlimited,
            literal_bytes: ResourceLimit::Unlimited,
            lexical_nesting_depth: ResourceLimit::Unlimited,
            diagnostics: ResourceLimit::Unlimited,
        }
    }

    /// Creates a resource policy with explicit source and token budgets.
    ///
    /// The other limits remain unlimited and can be configured separately.
    #[must_use]
    pub const fn bounded(
        source_bytes: NonZeroUsize,
        tokens: NonZeroUsize,
        token_bytes: NonZeroUsize,
    ) -> Self {
        Self {
            source_bytes: ResourceLimit::AtMost(source_bytes),
            tokens: ResourceLimit::AtMost(tokens),
            token_bytes: ResourceLimit::AtMost(token_bytes),
            numeric_digits: ResourceLimit::Unlimited,
            literal_bytes: ResourceLimit::Unlimited,
            lexical_nesting_depth: ResourceLimit::Unlimited,
            diagnostics: ResourceLimit::Unlimited,
        }
    }
}

/// Controls how the lexer exposes comments and whitespace to consumers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum TriviaPolicy {
    /// Emit comments and whitespace on the trivia channel when the
    /// scanner supports those token categories.
    ///
    /// This policy is useful for formatters, language servers, source
    /// reconstruction, documentation tooling, and refactoring.
    Preserve,

    /// Omit comments and whitespace from the emitted token stream.
    ///
    /// The scanner must still recognize them sufficiently to tokenize
    /// the surrounding source correctly.
    #[default]
    Skip,
}

/// Controls how the lexer handles malformed or disallowed Unicode input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum UnicodePolicy {
    /// Accept valid UTF-8 and apply the lexical specification's character
    /// classification rules. Preserve the original source spelling.
    ///
    /// This does not imply that every Unicode character is valid in an
    /// identifier or has syntactic significance.
    #[default]
    Specification,

    /// In addition to the specification's rules, reject Unicode
    /// characters or sequences explicitly prohibited by a stricter
    /// lexical security profile implemented by the scanner.
    ///
    /// This policy must not silently normalize or rewrite source text.
    Strict,
}

/// Controls how the scanner responds to lexical errors.
///
/// Error construction and diagnostic details remain owned by
/// `diagnostics.rs`. Actual input advancement and synchronization remain
/// owned by `recovery.rs` and `scanner.rs`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum RecoveryPolicy {
    /// Stop scanning when an unrecoverable lexical error occurs.
    Stop,

    /// Emit an error token where supported, then attempt deterministic
    /// recovery so later source can still be inspected.
    #[default]
    EmitErrorTokens,

    /// Skip malformed input according to the recovery implementation
    /// and continue without requiring an error token for every failure.
    SkipInvalidInput,
}

/// Controls whether an explicit EOF token is emitted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum EofPolicy {
    /// Emit one canonical EOF token at the end of the input.
    ///
    /// After it has been returned, subsequent token requests must not
    /// emit another EOF token.
    #[default]
    Emit,

    /// Signal end of input by exhausting the token source without
    /// emitting an explicit EOF token.
    ///
    /// Use only with consumers that explicitly support this convention.
    Omit,
}

/// Configuration for the Zamani lexical frontend.
///
/// This type contains policies and budgets only. It does not contain
/// source text, mutable scanner state, registered dialect implementations,
/// device information, or semantic execution state.
///
/// Prefer `LexerConfig::default()` followed by explicit field updates,
/// or use `LexerConfig::builder()` for incremental construction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LexerConfig {
    /// Resource budgets for one lexical input.
    pub limits: LexerLimits,

    /// Whether comments and whitespace are preserved in the token stream.
    pub trivia: TriviaPolicy,

    /// Unicode validation profile.
    pub unicode: UnicodePolicy,

    /// Error recovery behavior.
    pub recovery: RecoveryPolicy,

    /// Explicit EOF token convention.
    pub eof: EofPolicy,

    /// Whether an initial UTF-8 byte-order mark is accepted.
    ///
    /// Acceptance does not mean that the BOM is emitted as a token.
    /// The scanner must document whether the BOM is consumed or preserved
    /// in source mapping when this option is enabled.
    pub allow_initial_bom: bool,
}

impl LexerConfig {
    /// Creates the default configuration.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            limits: LexerLimits::unlimited(),
            trivia: TriviaPolicy::Skip,
            unicode: UnicodePolicy::Specification,
            recovery: RecoveryPolicy::EmitErrorTokens,
            eof: EofPolicy::Emit,
            allow_initial_bom: true,
        }
    }

    /// Creates a configuration builder.
    #[must_use]
    pub const fn builder() -> LexerConfigBuilder {
        LexerConfigBuilder {
            config: Self::new(),
        }
    }

    /// Replaces the resource budgets.
    #[must_use]
    pub const fn with_limits(mut self, limits: LexerLimits) -> Self {
        self.limits = limits;
        self
    }

    /// Replaces the trivia policy.
    #[must_use]
    pub const fn with_trivia_policy(mut self, policy: TriviaPolicy) -> Self {
        self.trivia = policy;
        self
    }

    /// Replaces the Unicode policy.
    #[must_use]
    pub const fn with_unicode_policy(mut self, policy: UnicodePolicy) -> Self {
        self.unicode = policy;
        self
    }

    /// Replaces the recovery policy.
    #[must_use]
    pub const fn with_recovery_policy(mut self, policy: RecoveryPolicy) -> Self {
        self.recovery = policy;
        self
    }

    /// Replaces the EOF policy.
    #[must_use]
    pub const fn with_eof_policy(mut self, policy: EofPolicy) -> Self {
        self.eof = policy;
        self
    }

    /// Sets whether an initial UTF-8 BOM is accepted.
    #[must_use]
    pub const fn with_initial_bom(mut self, allowed: bool) -> Self {
        self.allow_initial_bom = allowed;
        self
    }

    /// Checks configuration-level invariants.
    ///
    /// Every current configuration value is representable and has a
    /// defined policy. Runtime input and resource-budget violations are
    /// not configuration errors; they are handled by the scanner.
    ///
    /// This method exists as a stable validation boundary so additional
    /// cross-field constraints can be introduced deliberately later.
    pub const fn validate(&self) -> Result<(), ConfigError> {
        Ok(())
    }
}

impl Default for LexerConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Incremental builder for `LexerConfig`.
///
/// The builder does not allocate memory, inspect source text, access the
/// filesystem, initialize hardware, or perform dialect registration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LexerConfigBuilder {
    config: LexerConfig,
}

impl LexerConfigBuilder {
    /// Sets all resource budgets.
    #[must_use]
    pub const fn limits(mut self, limits: LexerLimits) -> Self {
        self.config.limits = limits;
        self
    }

    /// Sets the trivia policy.
    #[must_use]
    pub const fn trivia(mut self, policy: TriviaPolicy) -> Self {
        self.config.trivia = policy;
        self
    }

    /// Sets the Unicode policy.
    #[must_use]
    pub const fn unicode(mut self, policy: UnicodePolicy) -> Self {
        self.config.unicode = policy;
        self
    }

    /// Sets the recovery policy.
    #[must_use]
    pub const fn recovery(mut self, policy: RecoveryPolicy) -> Self {
        self.config.recovery = policy;
        self
    }

    /// Sets the EOF convention.
    #[must_use]
    pub const fn eof(mut self, policy: EofPolicy) -> Self {
        self.config.eof = policy;
        self
    }

    /// Sets whether an initial UTF-8 BOM is accepted.
    #[must_use]
    pub const fn allow_initial_bom(mut self, allowed: bool) -> Self {
        self.config.allow_initial_bom = allowed;
        self
    }

    /// Validates and returns the completed configuration.
    pub const fn build(self) -> Result<LexerConfig, ConfigError> {
        match self.config.validate() {
            Ok(()) => Ok(self.config),
            Err(error) => Err(error),
        }
    }
}

/// Configuration construction or validation error.
///
/// This error is intentionally separate from lexical diagnostics:
/// invalid source text is not a configuration-construction failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConfigError {
    /// Reserved for invalid cross-field combinations introduced by
    /// future versions of the configuration contract.
    InvalidCombination,
}

impl fmt::Display for ConfigError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidCombination => {
                formatter.write_str("incompatible lexer configuration options")
            }
        }
    }
}

impl std::error::Error for ConfigError {}

#[cfg(test)]
mod tests {
    use super::{
        EofPolicy, LexerConfig, LexerLimits, RecoveryPolicy, ResourceLimit,
        TriviaPolicy, UnicodePolicy,
    };
    use core::num::NonZeroUsize;

    #[test]
    fn default_configuration_is_deterministic() {
        assert_eq!(LexerConfig::default(), LexerConfig::new());
        assert_eq!(LexerConfig::new().validate(), Ok(()));
    }

    #[test]
    fn unlimited_budget_permits_any_observed_usize() {
        let limit = ResourceLimit::Unlimited;

        assert!(limit.permits(0));
        assert!(limit.permits(usize::MAX));
        assert!(!limit.is_bounded());
        assert_eq!(limit.bound(), None);
    }

    #[test]
    fn bounded_budget_is_inclusive() {
        let limit = ResourceLimit::at_most(8).expect("nonzero budget");

        assert!(limit.permits(0));
        assert!(limit.permits(8));
        assert!(!limit.permits(9));
        assert!(limit.is_bounded());
        assert_eq!(limit.bound().map(NonZeroUsize::get), Some(8));
    }

    #[test]
    fn zero_is_not_a_valid_nonzero_budget() {
        assert_eq!(ResourceLimit::at_most(0), None);
    }

    #[test]
    fn bounded_limits_leave_unconfigured_dimensions_unlimited() {
        let limits = LexerLimits::bounded(
            NonZeroUsize::new(1024).expect("nonzero source budget"),
            NonZeroUsize::new(128).expect("nonzero token budget"),
            NonZeroUsize::new(256).expect("nonzero lexeme budget"),
        );

        assert!(limits.source_bytes.permits(1024));
        assert!(!limits.source_bytes.permits(1025));
        assert!(limits.tokens.permits(128));
        assert!(!limits.tokens.permits(129));
        assert!(limits.token_bytes.permits(256));
        assert!(!limits.token_bytes.permits(257));

        assert_eq!(limits.numeric_digits, ResourceLimit::Unlimited);
        assert_eq!(limits.literal_bytes, ResourceLimit::Unlimited);
        assert_eq!(
            limits.lexical_nesting_depth,
            ResourceLimit::Unlimited
        );
        assert_eq!(limits.diagnostics, ResourceLimit::Unlimited);
    }

    #[test]
    fn builder_preserves_selected_policies() {
        let config = LexerConfig::builder()
            .trivia(TriviaPolicy::Preserve)
            .unicode(UnicodePolicy::Strict)
            .recovery(RecoveryPolicy::Stop)
            .eof(EofPolicy::Omit)
            .allow_initial_bom(false)
            .build()
            .expect("configuration should be valid");

        assert_eq!(config.trivia, TriviaPolicy::Preserve);
        assert_eq!(config.unicode, UnicodePolicy::Strict);
        assert_eq!(config.recovery, RecoveryPolicy::Stop);
        assert_eq!(config.eof, EofPolicy::Omit);
        assert!(!config.allow_initial_bom);
    }

    #[test]
    fn fluent_configuration_methods_compose() {
        let config = LexerConfig::new()
            .with_trivia_policy(TriviaPolicy::Preserve)
            .with_recovery_policy(RecoveryPolicy::SkipInvalidInput)
            .with_eof_policy(EofPolicy::Emit)
            .with_unicode_policy(UnicodePolicy::Specification)
            .with_initial_bom(true);

        assert_eq!(config.trivia, TriviaPolicy::Preserve);
        assert_eq!(config.recovery, RecoveryPolicy::SkipInvalidInput);
        assert_eq!(config.eof, EofPolicy::Emit);
        assert_eq!(config.unicode, UnicodePolicy::Specification);
        assert!(config.allow_initial_bom);
    }

    #[test]
    fn config_is_copyable_and_has_no_scanner_state() {
        let original = LexerConfig::new();
        let copied = original;

        assert_eq!(original, copied);
    }
}
