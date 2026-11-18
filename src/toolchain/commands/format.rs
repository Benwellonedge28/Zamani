
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Toolchain — Source Formatting Command
//!
//! Path: `src/toolchain/commands/format.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Provides the toolchain-facing source-formatting orchestration API.
//!
//! This module coordinates registered formatting providers. It does not
//! implement a second lexer, parser, grammar, AST, or semantic model.
//!
//! ## Ownership
//!
//! This module owns:
//! - Formatting request and response contracts.
//! - Formatting operation modes.
//! - Provider selection and compatibility checks.
//! - Formatting orchestration.
//! - Semantic-equivalence verification requirements.
//! - Optional formatter-idempotence verification.
//! - Structured toolchain error propagation.
//!
//! This module does not own:
//! - Zamani lexical rules or canonical token identities.
//! - Grammar productions or operator precedence.
//! - AST node definitions or AST traversal.
//! - Semantic analysis or semantic equivalence algorithms.
//! - Source parsing implementation.
//! - Compiler transformations or optimization.
//! - Target selection, hardware discovery, or execution.
//! - Filesystem traversal or persistent file replacement.
//!
//! ## Integration authorities
//!
//! - `crate::toolchain::errors` owns toolchain errors.
//! - `crate::toolchain::result` exposes the canonical result contract.
//! - `crate::frontend::lexer` owns canonical lexical identities.
//! - `crate::frontend::parser` owns parsing contracts.
//! - `crate::frontend::ast` owns canonical AST structures.
//! - `grammar/` owns the normative Zamani language grammar.
//!
//! A concrete formatter provider must integrate with the canonical
//! frontend and AST. This module does not assume an unverified formatter
//! implementation exists.
//!
//! ## Semantic preservation
//!
//! A successful formatting operation requires the provider to verify
//! semantic equivalence between the original and formatted source.
//!
//! The provider must preserve:
//! - Canonical token identities.
//! - Identifier and keyword classification.
//! - Expression precedence and associativity.
//! - AST structure and semantic meaning.
//! - Domain-specific syntax.
//! - Comments and documentation according to the formatter contract.
//! - Source constructs supported by the selected language edition.
//!
//! Formatting must not perform optimization or semantic rewriting.
//!
//! ## POCO-REAF
//!
//! Formatting is independent of physical target realization.
//!
//! The same formatting API supports classical, quantum, HDL, AI, hybrid,
//! distributed, networking, cryptographic, and future source constructs.
//!
//! New domains are supported through compatible formatter providers and
//! language extensions, not hard-coded domain switches.
//!
//! ## Scalability
//!
//! This module introduces no arbitrary maximum for:
//! - Source length.
//! - Number of formatting requests.
//! - Number of dialects.
//! - Number of language constructs.
//! - Number of registered providers.
//! - Number of computing domains.
//! - Expression nesting or AST node count.
//!
//! Concrete implementations remain subject to available process memory,
//! address space, execution time, and explicitly configured operational
//! budgets.
//!
//! ## Concurrency
//!
//! Providers are borrowed for the duration of an operation.
//! No global formatter registry, singleton, hidden worker pool, or
//! implicit mutable state is introduced.
//!
//! Concurrent use is governed by the provider's `Send` and `Sync`
//! implementation and its own documented concurrency contract.
//!
//! ## File operations
//!
//! This module formats source text and returns the resulting text.
//! It does not write files, overwrite user data, or recursively traverse
//! directories. Filesystem orchestration belongs to the command/CLI
//! integration layer, which must implement explicit write policies.
//!
//! ## Failure behavior
//!
//! Missing provider support, provider failures, semantic-equivalence
//! failures, and idempotence failures are never reported as success.
//!
//! Check-only mode reports whether formatting would change the source.
//! It does not modify source files.

use std::collections::{BTreeMap, BTreeSet};

use crate::toolchain::errors::{Result, ToolchainError};

// ============================================================================
// IDENTIFIERS
// ============================================================================

/// Identifies a source-formatting provider.
///
/// The identifier is an open-ended provider identity, not a closed list
/// of formatter vendors.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FormatterId(String);

impl FormatterId {
    /// Creates a formatter identity.
    ///
    /// # Errors
    ///
    /// Returns an error when the identity is empty or contains only
    /// whitespace.
    pub fn new(value: impl Into<String>) -> Result<Self> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(ToolchainError::invalid_input(
                "formatter identity must not be empty",
            )
            .with_operation("format.identity"));
        }

        Ok(Self(value))
    }

    /// Returns the canonical provider identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// ============================================================================
// FORMATTING OPTIONS
// ============================================================================

/// Provider-neutral formatting configuration.
///
/// Provider-specific settings use namespaced keys.
///
/// For example:
///
/// `zamani.indentation_width`
///
/// `provider.example.line_width`
///
/// Unknown settings are not silently interpreted by this module.
/// Their validation belongs to the selected provider.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct FormatOptions {
    /// Whether to verify that formatting is idempotent.
    ///
    /// When enabled, the provider formats its own output again.
    /// The second result must be identical to the first.
    pub verify_idempotence: bool,

    /// Whether the operation is a check-only operation.
    ///
    /// Check-only mode returns the formatted output and indicates whether
    /// the original source would change. It does not write anything.
    pub check_only: bool,

    /// Explicit provider-specific configuration.
    ///
    /// A BTreeMap provides deterministic key ordering.
    pub settings: BTreeMap<String, String>,

    /// Explicit language dialect identities.
    ///
    /// Dialects are identifiers, not a closed enumeration.
    pub dialects: BTreeSet<String>,

    /// Optional language edition or version.
    pub language_version: Option<String>,
}

impl FormatOptions {
    /// Creates default formatting options.
    ///
    /// Semantic-equivalence verification is mandatory regardless
    /// of this configuration.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Enables or disables check-only mode.
    #[must_use]
    pub fn with_check_only(mut self, enabled: bool) -> Self {
        self.check_only = enabled;
        self
    }

    /// Enables or disables idempotence verification.
    #[must_use]
    pub fn with_idempotence_verification(
        mut self,
        enabled: bool,
    ) -> Self {
        self.verify_idempotence = enabled;
        self
    }

    /// Sets the language version.
    #[must_use]
    pub fn with_language_version(
        mut self,
        version: impl Into<String>,
    ) -> Self {
        self.language_version = Some(version.into());
        self
    }

    /// Adds a language dialect.
    #[must_use]
    pub fn with_dialect(
        mut self,
        dialect: impl Into<String>,
    ) -> Self {
        self.dialects.insert(dialect.into());
        self
    }

    /// Adds or replaces a provider-specific setting.
    #[must_use]
    pub fn with_setting(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        self.settings.insert(key.into(), value.into());
        self
    }

    /// Enables check-only mode.
    #[must_use]
    pub fn check_only(mut self) -> Self {
        self.check_only = true;
        self
    }
}

// ============================================================================
// REQUEST
// ============================================================================

/// Source-formatting request.
///
/// The source is owned by the request so that callers can construct
/// requests independently of their original input buffers.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FormatRequest {
    /// Original Zamani source text.
    pub source: String,

    /// Optional source name for diagnostics.
    ///
    /// This is descriptive metadata and does not imply filesystem access.
    pub source_name: Option<String>,

    /// Formatting options.
    pub options: FormatOptions,
}

impl FormatRequest {
    /// Creates a formatting request.
    #[must_use]
    pub fn new(source: impl Into<String>) -> Self {
        Self {
            source: source.into(),
            source_name: None,
            options: FormatOptions::default(),
        }
    }

    /// Associates a source name with the request.
    #[must_use]
    pub fn with_source_name(
        mut self,
        name: impl Into<String>,
    ) -> Self {
        self.source_name = Some(name.into());
        self
    }

    /// Associates formatting options with the request.
    #[must_use]
    pub fn with_options(mut self, options: FormatOptions) -> Self {
        self.options = options;
        self
    }

    /// Validates request structure.
    ///
    /// Empty source is valid input: a provider must decide how an empty
    /// source unit is formatted according to the language contract.
    ///
    /// # Errors
    ///
    /// Returns an error for malformed configuration metadata.
    pub fn validate(&self) -> Result<()> {
        if let Some(version) = &self.options.language_version {
            if version.trim().is_empty() {
                return Err(ToolchainError::invalid_input(
                    "language version must not be empty",
                )
                .with_operation("format.request.validate"));
            }
        }

        for dialect in &self.options.dialects {
            if dialect.trim().is_empty() {
                return Err(ToolchainError::invalid_input(
                    "dialect identity must not be empty",
                )
                .with_operation("format.request.validate"));
            }
        }

        for key in self.options.settings.keys() {
            if key.trim().is_empty() {
                return Err(ToolchainError::invalid_input(
                    "formatting setting keys must not be empty",
                )
                .with_operation("format.request.validate"));
            }
        }

        Ok(())
    }
}

// ============================================================================
// PROVIDER CONTRACT
// ============================================================================

/// Contract implemented by a canonical source-formatting provider.
///
/// A provider may implement the Zamani native source formatter or adapt
/// another compatible formatter.
///
/// A provider must not claim support for syntax that it cannot preserve.
///
/// The provider is responsible for parsing original and formatted source
/// through the canonical frontend and verifying semantic equivalence.
///
/// This contract deliberately avoids prescribing a particular parser
/// constructor or AST root type.
pub trait FormatProvider {
    /// Returns the stable provider identity.
    fn id(&self) -> &FormatterId;

    /// Determines whether this provider supports the requested source
    /// version, dialects, and formatting configuration.
    ///
    /// Returning false means that another provider may be selected.
    ///
    /// An error indicates that provider compatibility could not be
    /// determined reliably.
    fn supports(&self, options: &FormatOptions) -> Result<bool>;

    /// Formats a source unit.
    ///
    /// The implementation must:
    /// - Follow the normative Zamani grammar.
    /// - Preserve canonical token identities.
    /// - Preserve expression precedence and associativity.
    /// - Preserve AST structure and semantic meaning.
    /// - Avoid semantic optimization.
    /// - Preserve comments and documentation according to the language
    ///   formatting contract.
    /// - Avoid silently discarding unsupported syntax.
    fn format(
        &self,
        source: &str,
        options: &FormatOptions,
    ) -> Result<String>;

    /// Verifies semantic equivalence between the original and formatted
    /// source.
    ///
    /// A production provider must parse both inputs using the canonical
    /// Zamani frontend and compare their canonical semantic structure.
    ///
    /// Textual equality alone is not a valid semantic-equivalence test.
    ///
    /// This method is mandatory: providers cannot silently bypass
    /// semantic-preservation verification.
    fn verify_semantic_equivalence(
        &self,
        original: &str,
        formatted: &str,
        options: &FormatOptions,
    ) -> Result<()>;
}

// ============================================================================
// RESULT CONTRACT
// ============================================================================

/// Outcome of a formatting operation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FormatStatus {
    /// Formatting completed and produced identical source text.
    Unchanged,

    /// Formatting completed and produced different source text.
    Changed,
}

/// Structured result of source formatting.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FormatResponse {
    /// Formatted source text.
    pub formatted_source: String,

    /// Provider that performed the operation.
    pub formatter: FormatterId,

    /// Formatting outcome.
    pub status: FormatStatus,

    /// Whether semantic equivalence was verified.
    ///
    /// A successfully constructed response always has this set to true.
    pub semantic_equivalence_verified: bool,

    /// Whether idempotence was verified.
    ///
    /// False means that idempotence verification was not requested.
    pub idempotence_verified: bool,

    /// Whether the caller requested check-only behavior.
    pub check_only: bool,
}

impl FormatResponse {
    /// Returns true if the source would need modification.
    #[must_use]
    pub const fn would_change(&self) -> bool {
        matches!(self.status, FormatStatus::Changed)
    }

    /// Returns true if the operation completed without changing the source.
    #[must_use]
    pub const fn is_unchanged(&self) -> bool {
        matches!(self.status, FormatStatus::Unchanged)
    }

    /// Returns true if all requested verification completed.
    #[must_use]
    pub const fn is_verified(&self) -> bool {
        self.semantic_equivalence_verified
            && (!self.check_only || !self.formatted_source.is_empty()
                || matches!(self.status, FormatStatus::Unchanged))
            && (!self.idempotence_verified || self.semantic_equivalence_verified)
    }
}

// ============================================================================
// FORMAT COMMAND
// ============================================================================

/// Formats a Zamani source unit using an explicitly supplied provider.
///
/// This is the principal toolchain formatting entry point.
///
/// It does not:
/// - Discover providers globally.
/// - Instantiate a parser implicitly.
/// - Execute external commands.
/// - Read or write files.
/// - Select a hardware target.
/// - Perform compilation.
/// - Claim success without semantic verification.
///
/// The caller is responsible for selecting an appropriately registered
/// provider through the canonical provider registry.
///
/// # Errors
///
/// Returns a structured toolchain error when:
/// - The request is invalid.
/// - The provider identity is invalid.
/// - The provider does not support the request.
/// - Formatting fails.
/// - Semantic equivalence cannot be established.
/// - Requested idempotence verification fails.
pub fn format_source<P>(
    provider: &P,
    request: &FormatRequest,
) -> Result<FormatResponse>
where
    P: FormatProvider + ?Sized,
{
    request
        .validate()
        .map_err(|error| error.with_operation("format.request.validate"))?;

    let formatter = FormatterId::new(provider.id().as_str())
        .map_err(|error| error.with_operation("format.provider.identity"))?;

    let supported = provider
        .supports(&request.options)
        .map_err(|error| {
            error
                .with_operation("format.provider.compatibility")
                .with_context(format!(
                    "formatter provider: {}",
                    formatter.as_str()
                ))
        })?;

    if !supported {
        return Err(
            ToolchainError::provider_failed(format!(
                "formatter provider '{}' does not support the requested language version or dialect configuration",
                formatter.as_str()
            ))
            .with_operation("format.provider.compatibility"),
        );
    }

    let formatted_source = provider
        .format(&request.source, &request.options)
        .map_err(|error| {
            error
                .with_operation("format.execute")
                .with_context(format!(
                    "formatter provider: {}",
                    formatter.as_str()
                ))
        })?;

    // Semantic verification is mandatory and cannot be disabled
    // through FormatOptions.
    provider
        .verify_semantic_equivalence(
            &request.source,
            &formatted_source,
            &request.options,
        )
        .map_err(|error| {
            error
                .with_operation("format.verify_semantics")
                .with_context(format!(
                    "formatter provider: {}",
                    formatter.as_str()
                ))
        })?;

    // Idempotence is an additional canonical-formatting guarantee.
    // It does not replace semantic-equivalence verification.
    let idempotence_verified =
        if request.options.verify_idempotence {
            let repeated = provider
                .format(&formatted_source, &request.options)
                .map_err(|error| {
                    error
                        .with_operation("format.verify_idempotence")
                        .with_context("second formatting pass failed")
                })?;

            if repeated != formatted_source {
                return Err(
                    ToolchainError::provider_failed(
                        "formatter is not idempotent: repeated formatting changed its output",
                    )
                    .with_operation("format.verify_idempotence")
                    .with_context(format!(
                        "formatter provider: {}",
                        formatter.as_str()
                    )),
                );
            }

            // Verify that the repeated output also retains the
            // original program's meaning.
            provider
                .verify_semantic_equivalence(
                    &request.source,
                    &repeated,
                    &request.options,
                )
                .map_err(|error| {
                    error
                        .with_operation("format.verify_idempotence_semantics")
                })?;

            true
        } else {
            false
        };

    let status = if formatted_source == request.source {
        FormatStatus::Unchanged
    } else {
        FormatStatus::Changed
    };

    Ok(FormatResponse {
        formatted_source,
        formatter,
        status,
        semantic_equivalence_verified: true,
        idempotence_verified,
        check_only: request.options.check_only,
    })
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    struct TestFormatter {
        id: FormatterId,
    }

    impl TestFormatter {
        fn new() -> Self {
            Self {
                id: FormatterId::new("test.zamani").expect("valid identity"),
            }
        }
    }

    impl FormatProvider for TestFormatter {
        fn id(&self) -> &FormatterId {
            &self.id
        }

        fn supports(&self, _options: &FormatOptions) -> Result<bool> {
            Ok(true)
        }

        fn format(
            &self,
            source: &str,
            _options: &FormatOptions,
        ) -> Result<String> {
            Ok(source.trim().to_owned())
        }

        fn verify_semantic_equivalence(
            &self,
            original: &str,
            formatted: &str,
            _options: &FormatOptions,
        ) -> Result<()> {
            if original.trim() == formatted.trim() {
                Ok(())
            } else {
                Err(ToolchainError::provider_failed(
                    "test formatter changed source meaning",
                ))
            }
        }
    }

    struct UnsupportedFormatter {
        id: FormatterId,
    }

    impl FormatProvider for UnsupportedFormatter {
        fn id(&self) -> &FormatterId {
            &self.id
        }

        fn supports(&self, _options: &FormatOptions) -> Result<bool> {
            Ok(false)
        }

        fn format(
            &self,
            _source: &str,
            _options: &FormatOptions,
        ) -> Result<String> {
            panic!("unsupported providers must never be invoked")
        }

        fn verify_semantic_equivalence(
            &self,
            _original: &str,
            _formatted: &str,
            _options: &FormatOptions,
        ) -> Result<()> {
            panic!("unsupported providers must never be verified")
        }
    }

    #[test]
    fn formatter_identity_rejects_empty_values() {
        assert!(FormatterId::new("").is_err());
        assert!(FormatterId::new("   ").is_err());
        assert!(FormatterId::new("zamani.formatter").is_ok());
    }

    #[test]
    fn formatting_returns_verified_source() {
        let formatter = TestFormatter::new();

        let request = FormatRequest::new("  fn main() {}  ");

        let response = format_source(&formatter, &request)
            .expect("formatting must succeed");

        assert_eq!(response.formatted_source, "fn main() {}");
        assert_eq!(response.status, FormatStatus::Changed);
        assert!(response.semantic_equivalence_verified);
        assert!(!response.idempotence_verified);
        assert!(!response.would_change().not());
    }

    #[test]
    fn unchanged_source_is_reported_correctly() {
        let formatter = TestFormatter::new();

        let request = FormatRequest::new("fn main() {}");

        let response = format_source(&formatter, &request)
            .expect("formatting must succeed");

        assert_eq!(response.status, FormatStatus::Unchanged);
        assert!(response.is_unchanged());
        assert!(!response.would_change());
    }

    #[test]
    fn check_only_does_not_modify_input() {
        let formatter = TestFormatter::new();

        let original = "  fn main() {}  ";

        let request = FormatRequest::new(original)
            .with_options(FormatOptions::default().check_only());

        let response = format_source(&formatter, &request)
            .expect("check-only formatting must succeed");

        assert!(response.check_only);
        assert!(response.would_change());
        assert_eq!(original, "  fn main() {}  ");
    }

    #[test]
    fn idempotence_verification_succeeds_for_canonical_output() {
        let formatter = TestFormatter::new();

        let request = FormatRequest::new("  fn main() {}  ")
            .with_options(
                FormatOptions::default()
                    .with_idempotence_verification(true),
            );

        let response = format_source(&formatter, &request)
            .expect("canonical formatting must be idempotent");

        assert!(response.idempotence_verified);
        assert!(response.semantic_equivalence_verified);
    }

    #[test]
    fn unsupported_provider_is_rejected() {
        let formatter = UnsupportedFormatter {
            id: FormatterId::new("unsupported.formatter")
                .expect("valid identity"),
        };

        let request = FormatRequest::new("fn main() {}");

        let error = format_source(&formatter, &request)
            .expect_err("unsupported provider must be rejected");

        assert_eq!(
            error.operation(),
            Some("format.provider.compatibility")
        );
    }

    #[test]
    fn dialects_are_open_ended() {
        let options = FormatOptions::default()
            .with_dialect("classical")
            .with_dialect("quantum.custom")
            .with_dialect("future.domain");

        assert_eq!(options.dialects.len(), 3);
        assert!(options.dialects.contains("future.domain"));
    }

    #[test]
    fn provider_settings_are_deterministic() {
        let options = FormatOptions::default()
            .with_setting("provider.z.line_width", "100")
            .with_setting("provider.a.indentation", "spaces");

        let keys: Vec<_> = options.settings.keys().collect();

        assert!(keys[0] < keys[1]);
    }

    #[test]
    fn empty_source_is_a_valid_formatting_input() {
        let formatter = TestFormatter::new();

        let request = FormatRequest::new("");

        let response = format_source(&formatter, &request)
            .expect("empty source must be handled by the provider");

        assert_eq!(response.formatted_source, "");
        assert!(response.is_unchanged());
    }

    #[test]
    fn invalid_language_version_is_rejected() {
        let formatter = TestFormatter::new();

        let request = FormatRequest::new("fn main() {}")
            .with_options(
                FormatOptions::default().with_language_version(" "),
            );

        assert!(format_source(&formatter, &request).is_err());
    }
}
