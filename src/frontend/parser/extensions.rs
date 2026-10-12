
#![forbid(unsafe_code)]

//! Versioned grammar-extension contracts for the Zamani parser.
//!
//! File: src/frontend/parser/extensions.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! # Purpose
//!
//! Define stable interfaces through which core, domain, dialect, experimental,
//! and third-party grammar extensions participate in parsing.
//!
//! # Ownership
//!
//! This module owns:
//! - extension identifiers and version metadata;
//! - parser-extension API compatibility declarations;
//! - grammar extension points;
//! - extension descriptors and matching outcomes;
//! - the extension handler contract;
//! - the minimal parser-context interface available to handlers;
//! - structured extension errors.
//!
//! This module does not own:
//! - lexical scanning, token definitions, or token registries;
//! - extension registration, registry storage, or dispatch ordering;
//! - canonical AST definitions or AST construction policy;
//! - grammar productions belonging to the core parser;
//! - name resolution, type checking, effects, or contract evaluation;
//! - hardware discovery, resource negotiation, scheduling, or lowering.
//!
//! # Integration contract
//!
//! - `crate::frontend::lexer::token_kind::TokenKind` is the canonical token
//!   identity used by extension probes.
//! - `crate::frontend::lexer::span::Span` is reused for source locations.
//! - `registry.rs` owns registration, duplicate detection, compatibility
//!   checks, deterministic ordering, and extension selection.
//! - `context.rs` and `session.rs` provide the concrete implementation of
//!   `ExtensionParseContext` using the active parser session.
//! - `grammar.rs` and grammar modules invoke extensions only at declared
//!   extension points.
//! - `ast_adapter.rs` maps extension-produced syntax into the canonical AST
//!   where an intermediate syntax representation is used.
//! - `api.rs` exposes only the extension API intended to be public.
//!
//! The `N` type in `GrammarExtension<N>` is selected by the parser integration.
//! This module deliberately does not introduce a second AST.
//!
//! # Determinism and safety
//!
//! Extension matching must be deterministic for the same input, parser
//! configuration, registry snapshot, and extension implementations.
//! Implementations must not rely on registration order to resolve conflicts.
//! The registry must impose a deterministic ordering and reject unresolved
//! equal-priority ambiguity.
//!
//! A dispatcher must also verify that a successful parse makes progress
//! whenever the grammar contract requires input consumption. Resource budgets
//! and cancellation belong to the parser session and must apply to extension
//! work as well as core grammar work.
//!
//! # Scalability
//!
//! There are no hard-coded limits on the number of extensions, dialects,
//! operations, devices, qubits, processors, or other computing resources.
//! Actual resource use is governed by available memory, the configured parser
//! budget, and the capabilities of downstream compilation and execution layers.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use crate::frontend::lexer::span::Span;
use crate::frontend::lexer::token_kind::TokenKind;

// -----------------------------------------------------------------------------
// Extension identifiers
// -----------------------------------------------------------------------------

/// Stable, namespaced identifier for a grammar extension.
///
/// Examples include `core.quantum`, `vendor.hdl`, and `experimental.patterns`.
///
/// The identifier is opaque to the parser. It is not a hardware identifier,
/// AST variant, keyword, or semantic operation name.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExtensionId(String);

impl ExtensionId {
    /// Create a validated extension identifier.
    ///
    /// Names are case-sensitive and consist of non-empty dot-separated
    /// segments. Each segment may contain ASCII letters, digits, `_`, or `-`.
    /// Requiring a portable identifier format keeps ordering and comparison
    /// independent of locale.
    pub fn new(value: impl Into<String>) -> Result<Self, ExtensionError> {
        let value = value.into();

        if value.is_empty() {
            return Err(ExtensionError::new(
                ExtensionErrorKind::InvalidIdentifier,
                "extension identifier must not be empty",
            ));
        }

        if value.trim() != value {
            return Err(ExtensionError::new(
                ExtensionErrorKind::InvalidIdentifier,
                "extension identifier must not contain leading or trailing whitespace",
            ));
        }

        let valid = value.split('.').all(|segment| {
            !segment.is_empty()
                && segment.bytes().all(|byte| {
                    byte.is_ascii_alphanumeric() || byte == b'_' || byte == b'-'
                })
        });

        if !valid {
            return Err(ExtensionError::new(
                ExtensionErrorKind::InvalidIdentifier,
                "extension identifiers must contain non-empty dot-separated ASCII segments",
            ));
        }

        Ok(Self(value))
    }

    /// Return the canonical identifier string.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ExtensionId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// -----------------------------------------------------------------------------
// Extension versions and parser API compatibility
// -----------------------------------------------------------------------------

/// Semantic version metadata for an extension.
///
/// This is metadata, not a dependency on a particular versioning crate.
/// Compatibility policy is evaluated explicitly by the registry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExtensionVersion {
    major: u64,
    minor: u64,
    patch: u64,
}

impl ExtensionVersion {
    /// Construct a version from its numeric components.
    #[must_use]
    pub const fn new(major: u64, minor: u64, patch: u64) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    #[must_use]
    pub const fn major(self) -> u64 {
        self.major
    }

    #[must_use]
    pub const fn minor(self) -> u64 {
        self.minor
    }

    #[must_use]
    pub const fn patch(self) -> u64 {
        self.patch
    }
}

impl fmt::Display for ExtensionVersion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}.{}.{}",
            self.major, self.minor, self.patch
        )
    }
}

/// Parser extension API compatibility declaration.
///
/// An extension declares the parser API major version it targets and the
/// inclusive minor-version interval it supports. A missing upper bound means
/// that later minor versions of the same major version are accepted.
///
/// A major-version mismatch is incompatible. The registry must reject
/// incompatible extensions before dispatch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExtensionApiCompatibility {
    major: u64,
    min_minor: u64,
    max_minor: Option<u64>,
}

impl ExtensionApiCompatibility {
    /// Create a compatibility interval.
    pub fn new(
        major: u64,
        min_minor: u64,
        max_minor: Option<u64>,
    ) -> Result<Self, ExtensionError> {
        if max_minor.is_some_and(|maximum| maximum < min_minor) {
            return Err(ExtensionError::new(
                ExtensionErrorKind::InvalidCompatibilityRange,
                "maximum supported parser API minor version is below the minimum",
            ));
        }

        Ok(Self {
            major,
            min_minor,
            max_minor,
        })
    }

    /// Test whether this declaration supports a parser API version.
    #[must_use]
    pub const fn supports(self, major: u64, minor: u64) -> bool {
        if major != self.major || minor < self.min_minor {
            return false;
        }

        match self.max_minor {
            Some(maximum) => minor <= maximum,
            None => true,
        }
    }

    #[must_use]
    pub const fn major(self) -> u64 {
        self.major
    }

    #[must_use]
    pub const fn min_minor(self) -> u64 {
        self.min_minor
    }

    #[must_use]
    pub const fn max_minor(self) -> Option<u64> {
        self.max_minor
    }
}

// -----------------------------------------------------------------------------
// Grammar extension points
// -----------------------------------------------------------------------------

/// A location in the grammar where registered extensions may participate.
///
/// These points describe syntactic responsibilities rather than computing
/// domains. The registry and grammar decide which points are enabled for a
/// given language edition or dialect.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum ExtensionPoint {
    /// A top-level item or declaration.
    Item,

    /// A declaration inside a permitted declaration context.
    Declaration,

    /// A statement in a statement sequence.
    Statement,

    /// An expression prefix or primary expression.
    Expression,

    /// An operator or expression continuation.
    ExpressionContinuation,

    /// A type expression.
    Type,

    /// A pattern.
    Pattern,

    /// An attribute or annotation.
    Attribute,

    /// A domain-specific block or region.
    Region,

    /// A domain-specific construct inside a type or declaration.
    TypeMember,

    /// A grammar-defined embedded-language boundary.
    EmbeddedLanguage,
}

/// Result of checking whether an extension recognizes a grammar position.
///
/// Probing must be side-effect-free: it must not consume tokens, emit
/// diagnostics, mutate parser state, or execute user code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExtensionMatch {
    /// This extension does not recognize the current grammar position.
    NoMatch,

    /// This extension recognizes the current grammar position.
    Match,
}

// -----------------------------------------------------------------------------
// Extension descriptors
// -----------------------------------------------------------------------------

/// Declarative metadata for a registered grammar extension.
///
/// `priority` is an ordering hint for deterministic dispatch; it is not
/// permission to override a conflicting extension silently. If two enabled
/// handlers can claim the same position at the same effective priority and
/// the grammar does not explicitly permit that situation, the registry must
/// report an ambiguity.
///
/// Dialect requirements and declared conflicts use stable identifiers rather
/// than hard-coded Rust enum variants.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtensionDescriptor {
    id: ExtensionId,
    version: ExtensionVersion,
    api_compatibility: ExtensionApiCompatibility,
    priority: i32,
    extension_points: BTreeSet<ExtensionPoint>,
    required_dialects: BTreeSet<String>,
    conflicts: BTreeSet<ExtensionId>,
}

impl ExtensionDescriptor {
    /// Construct a descriptor.
    pub fn new(
        id: ExtensionId,
        version: ExtensionVersion,
        api_compatibility: ExtensionApiCompatibility,
        extension_points: impl IntoIterator<Item = ExtensionPoint>,
    ) -> Result<Self, ExtensionError> {
        let extension_points: BTreeSet<_> = extension_points.into_iter().collect();

        if extension_points.is_empty() {
            return Err(ExtensionError::new(
                ExtensionErrorKind::NoExtensionPoints,
                "an extension must declare at least one grammar extension point",
            )
            .with_extension(id));
        }

        Ok(Self {
            id,
            version,
            api_compatibility,
            priority: 0,
            extension_points,
            required_dialects: BTreeSet::new(),
            conflicts: BTreeSet::new(),
        })
    }

    /// Set the deterministic dispatch priority.
    ///
    /// The registry must still detect ambiguity and conflicts. Priority alone
    /// must not silently resolve an explicitly forbidden conflict.
    #[must_use]
    pub fn with_priority(mut self, priority: i32) -> Self {
        self.priority = priority;
        self
    }

    /// Declare a dialect required to enable this extension.
    ///
    /// Dialect names are opaque, non-empty identifiers. Full dialect-version
    /// resolution belongs to the dialect/registry integration layer.
    pub fn requiring_dialect(
        mut self,
        dialect: impl Into<String>,
    ) -> Result<Self, ExtensionError> {
        let dialect = dialect.into();

        if dialect.is_empty() || dialect.trim() != dialect {
            return Err(ExtensionError::new(
                ExtensionErrorKind::InvalidDialectRequirement,
                "required dialect identifier must be non-empty and trimmed",
            )
            .with_extension(self.id.clone()));
        }

        self.required_dialects.insert(dialect);
        Ok(self)
    }

    /// Declare a conflict with another extension identifier.
    ///
    /// Conflict declarations are symmetric at registry-validation time even
    /// if only one extension declares the conflict.
    #[must_use]
    pub fn conflicting_with(mut self, other: ExtensionId) -> Self {
        if other != self.id {
            self.conflicts.insert(other);
        }
        self
    }

    #[must_use]
    pub fn id(&self) -> &ExtensionId {
        &self.id
    }

    #[must_use]
    pub const fn version(&self) -> ExtensionVersion {
        self.version
    }

    #[must_use]
    pub const fn api_compatibility(&self) -> ExtensionApiCompatibility {
        self.api_compatibility
    }

    #[must_use]
    pub const fn priority(&self) -> i32 {
        self.priority
    }

    #[must_use]
    pub fn extension_points(&self) -> &BTreeSet<ExtensionPoint> {
        &self.extension_points
    }

    #[must_use]
    pub fn required_dialects(&self) -> &BTreeSet<String> {
        &self.required_dialects
    }

    #[must_use]
    pub fn conflicts(&self) -> &BTreeSet<ExtensionId> {
        &self.conflicts
    }

    /// Return whether this extension declares a handler for the given point.
    #[must_use]
    pub fn supports_point(&self, point: ExtensionPoint) -> bool {
        self.extension_points.contains(&point)
    }

    /// Return whether this extension declares a conflict with another ID.
    #[must_use]
    pub fn conflicts_with(&self, other: &ExtensionId) -> bool {
        self.conflicts.contains(other)
    }

    /// Return whether the descriptor supports a parser API version.
    #[must_use]
    pub const fn supports_parser_api(&self, major: u64, minor: u64) -> bool {
        self.api_compatibility.supports(major, minor)
    }
}

// -----------------------------------------------------------------------------
// Parser context exposed to extension handlers
// -----------------------------------------------------------------------------

/// Restricted parser interface available to grammar extensions.
///
/// Implement this interface in the parser/session integration layer; do not
/// make extensions depend on a concrete parser cursor or mutable global state.
///
/// `lookahead_kind(0)` refers to the current token. A missing value means that
/// no token exists at that offset; the parser integration should use its
/// canonical EOF convention consistently.
///
/// The implementation must apply the same configured work, token-consumption,
/// nesting, and cancellation budgets to extension handlers as to core grammar
/// productions. `advance` must return a structured error rather than panic
/// when the stream or budget is exhausted.
pub trait ExtensionParseContext {
    /// Current cursor position, suitable for progress checks.
    ///
    /// The unit is defined by the concrete parser implementation and must
    /// remain stable for the lifetime of one parse session.
    fn position(&self) -> usize;

    /// Return a lookahead token kind, where offset zero is the current token.
    fn lookahead_kind(&self, offset: usize) -> Option<TokenKind>;

    /// Return a lookahead token's original lexeme.
    fn lookahead_lexeme(&self, offset: usize) -> Option<&str>;

    /// Return a lookahead token's canonical source span.
    fn lookahead_span(&self, offset: usize) -> Option<Span>;

    /// Consume one token subject to the active parser budget.
    fn advance(&mut self) -> Result<(), ExtensionError>;
}

// -----------------------------------------------------------------------------
// Extension handler contract
// -----------------------------------------------------------------------------

/// Versioned grammar-extension handler.
///
/// `N` is the syntax-node type chosen by the parser integration. It can be a
/// canonical AST node or a parser-owned intermediate syntax node. The
/// extension framework does not require every domain to share a separate AST.
///
/// Implementations must:
///
/// - return stable descriptor metadata for their lifetime;
/// - keep `probe` side-effect-free and deterministic;
/// - parse only at declared extension points;
/// - use the supplied context rather than creating another lexer/parser;
/// - preserve meaningful syntax for the AST adapter;
/// - report malformed recognized syntax as an error, not `NoMatch`;
/// - respect parser budgets and cancellation through the context;
/// - avoid global mutable state and nondeterministic dispatch behavior.
///
/// `Send + Sync` permits immutable extension definitions to be shared between
/// independent parse sessions. Per-parse mutable state belongs in the
/// supplied context, not in the extension object.
pub trait GrammarExtension<N>: Send + Sync {
    /// Return immutable extension metadata.
    fn descriptor(&self) -> &ExtensionDescriptor;

    /// Check whether this extension claims the current grammar position.
    ///
    /// The caller supplies a finite lookahead window determined by parser
    /// configuration. Extensions must not attempt to read additional tokens
    /// or mutate parser state during this method.
    fn probe(
        &self,
        point: ExtensionPoint,
        lookahead: &[TokenKind],
    ) -> ExtensionMatch;

    /// Parse the recognized construct and return its syntax representation.
    ///
    /// The registry/dispatcher must verify that the descriptor declares
    /// `point`, and must check input progress where the grammar requires
    /// consumption. It must not invoke `parse` after `probe` returns
    /// `NoMatch`.
    fn parse(
        &self,
        point: ExtensionPoint,
        context: &mut dyn ExtensionParseContext,
    ) -> Result<N, ExtensionError>;
}

// -----------------------------------------------------------------------------
// Structured extension errors
// -----------------------------------------------------------------------------

/// Machine-readable category for extension failures.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum ExtensionErrorKind {
    InvalidIdentifier,
    InvalidCompatibilityRange,
    NoExtensionPoints,
    InvalidDialectRequirement,
    IncompatibleParserApi,
    DuplicateExtension,
    ConflictingExtensions,
    AmbiguousMatch,
    UnsupportedExtensionPoint,
    InvalidExtensionResult,
    NoProgress,
    MalformedConstruct,
    ResourceLimitExceeded,
    Cancelled,
    InternalFailure,
}

/// Structured error produced while validating or executing an extension.
///
/// Source diagnostics are emitted by the parser's canonical diagnostic sink.
/// This type intentionally does not define a competing diagnostic/span
/// system; it may carry a canonical span for callers that have one.
#[derive(Debug, Clone)]
pub struct ExtensionError {
    kind: ExtensionErrorKind,
    message: String,
    extension: Option<ExtensionId>,
    span: Option<Span>,
}

impl ExtensionError {
    /// Construct an extension error without source location.
    #[must_use]
    pub fn new(kind: ExtensionErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
            extension: None,
            span: None,
        }
    }

    /// Attach the extension responsible for the error.
    #[must_use]
    pub fn with_extension(mut self, extension: ExtensionId) -> Self {
        self.extension = Some(extension);
        self
    }

    /// Attach a canonical source span when available.
    #[must_use]
    pub fn with_span(mut self, span: Span) -> Self {
        self.span = Some(span);
        self
    }

    #[must_use]
    pub const fn kind(&self) -> ExtensionErrorKind {
        self.kind
    }

    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    #[must_use]
    pub fn extension(&self) -> Option<&ExtensionId> {
        self.extension.as_ref()
    }

    #[must_use]
    pub fn span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
}

impl fmt::Display for ExtensionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:?}: {}", self.kind, self.message)?;

        if let Some(extension) = &self.extension {
            write!(formatter, " [extension: {extension}]")?;
        }

        Ok(())
    }
}

impl Error for ExtensionError {}

// -----------------------------------------------------------------------------
// Unit tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::{
        ExtensionApiCompatibility, ExtensionDescriptor, ExtensionErrorKind, ExtensionId,
        ExtensionPoint, ExtensionVersion,
    };

    fn id(value: &str) -> ExtensionId {
        ExtensionId::new(value).expect("test extension identifier should be valid")
    }

    fn api() -> ExtensionApiCompatibility {
        ExtensionApiCompatibility::new(1, 0, Some(4))
            .expect("test compatibility interval should be valid")
    }

    #[test]
    fn accepts_namespaced_extension_ids() {
        assert_eq!(id("core.quantum").as_str(), "core.quantum");
        assert_eq!(id("vendor_x.hdl-2").as_str(), "vendor_x.hdl-2");
    }

    #[test]
    fn rejects_invalid_extension_ids() {
        for value in ["", ".quantum", "quantum.", "core..quantum", "core quantum", " core"] {
            assert!(
                ExtensionId::new(value).is_err(),
                "identifier should be rejected: {value:?}"
            );
        }
    }

    #[test]
    fn versions_have_numeric_ordering() {
        assert!(ExtensionVersion::new(1, 10, 0) > ExtensionVersion::new(1, 9, 9));
        assert!(ExtensionVersion::new(2, 0, 0) > ExtensionVersion::new(1, 99, 99));
    }

    #[test]
    fn parser_api_compatibility_checks_major_and_minor_ranges() {
        let compatibility = api();

        assert!(compatibility.supports(1, 0));
        assert!(compatibility.supports(1, 4));
        assert!(!compatibility.supports(1, 5));
        assert!(!compatibility.supports(2, 0));
        assert!(!compatibility.supports(1, 0 + 0) == false || compatibility.supports(1, 0));
    }

    #[test]
    fn rejects_reversed_compatibility_ranges() {
        let error = ExtensionApiCompatibility::new(1, 5, Some(3))
            .expect_err("reversed compatibility interval must fail");

        assert_eq!(error.kind(), ExtensionErrorKind::InvalidCompatibilityRange);
    }

    #[test]
    fn descriptor_requires_at_least_one_extension_point() {
        let error = ExtensionDescriptor::new(
            id("example.empty"),
            ExtensionVersion::new(1, 0, 0),
            api(),
            [],
        )
        .expect_err("empty extension-point set must fail");

        assert_eq!(error.kind(), ExtensionErrorKind::NoExtensionPoints);
    }

    #[test]
    fn descriptor_keeps_deterministic_metadata() {
        let descriptor = ExtensionDescriptor::new(
            id("vendor.quantum"),
            ExtensionVersion::new(2, 1, 0),
            api(),
            [ExtensionPoint::Item, ExtensionPoint::Region],
        )
        .expect("descriptor should be valid")
        .with_priority(20)
        .requiring_dialect("quantum")
        .expect("dialect name should be valid")
        .conflicting_with(id("legacy.quantum"));

        assert_eq!(descriptor.id().as_str(), "vendor.quantum");
        assert_eq!(descriptor.version(), ExtensionVersion::new(2, 1, 0));
        assert_eq!(descriptor.priority(), 20);
        assert!(descriptor.supports_point(ExtensionPoint::Item));
        assert!(descriptor.required_dialects().contains("quantum"));
        assert!(descriptor.conflicts_with(&id("legacy.quantum")));
        assert!(!descriptor.conflicts_with(&id("other.extension")));
    }

    #[test]
    fn descriptor_does_not_self_register_as_a_conflict() {
        let own_id = id("example.extension");

        let descriptor = ExtensionDescriptor::new(
            own_id.clone(),
            ExtensionVersion::new(1, 0, 0),
            api(),
            [ExtensionPoint::Expression],
        )
        .expect("descriptor should be valid")
        .conflicting_with(own_id.clone());

        assert!(!descriptor.conflicts_with(&own_id));
    }

    #[test]
    fn unbounded_minor_compatibility_is_explicit() {
        let compatibility = ExtensionApiCompatibility::new(3, 2, None)
            .expect("open-ended compatibility interval should be valid");

        assert!(compatibility.supports(3, 2));
        assert!(compatibility.supports(3, u64::MAX));
        assert!(!compatibility.supports(2, u64::MAX));
        assert!(!compatibility.supports(3, 1));
    }
}
