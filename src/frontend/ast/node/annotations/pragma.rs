//! Source-level pragma representation for the native Zamani frontend AST.
//!
//! # Architectural role
//!
//! A pragma is source-language metadata/directive syntax. It is **data**.
//! It is never an execution permission and must never cause filesystem,
//! network, process, hardware, quantum-device, calibration, or runtime
//! activity while constructing or validating the AST.
//!
//! The intended boundary is:
//!
//! ```text
//! Zamani source
//!     |
//!     v
//! lexer / parser
//!     |
//!     v
//! native AST
//!     |
//!     +-- Pragma  <- this file
//!     |
//!     v
//! structural validation
//!     |
//!     v
//! semantic analysis
//!     |
//!     v
//! semantic model
//!     |
//!     v
//! ZUIR / domain lowering
//! ```
//!
//! This module owns the **syntax-preserving representation** of a pragma.
//!
//! It does not:
//!
//! - execute pragmas;
//! - interpret pragma meaning;
//! - select hardware;
//! - select a quantum backend;
//! - inspect QPU topology;
//! - route quantum operations;
//! - schedule operations;
//! - perform calibration;
//! - perform QEC;
//! - perform optimization;
//! - access the filesystem;
//! - access the network;
//! - spawn processes;
//! - access environment variables;
//! - access credentials;
//! - mutate compiler-global state;
//! - depend on QIR;
//! - depend on LLVM;
//! - depend on MLIR;
//! - depend on a vendor backend.
//!
//! # Design goals
//!
//! This representation is deliberately:
//!
//! - source preserving;
//! - domain neutral;
//! - namespace aware;
//! - extensible;
//! - deterministic;
//! - serializable;
//! - independently testable;
//! - safe for untrusted source;
//! - compatible with Rust 1.97 / 1.97.1;
//! - `unsafe`-free;
//! - independent of machine size;
//! - independent of qubit count;
//! - independent of hardware topology;
//! - independent of backend instruction sets;
//! - independent of vendor-specific implementations.
//!
//! # POCO-REAF
//!
//! A pragma must never introduce a hidden machine-size restriction into the
//! native AST. A pragma may express a source-level constraint or compiler
//! intent, but interpretation of that intent belongs to later compiler
//! phases.
//!
//! Therefore this type contains no fields such as:
//!
//! - `max_qubits`;
//! - `device_id`;
//! - `backend_id`;
//! - `topology`;
//! - `gpu_count`;
//! - `cpu_count`;
//! - `fixed_register_size`.
//!
//! If a future Zamani pragma needs to describe such a concept, it should do
//! so through an extensible namespaced payload and the semantic/compiler
//! layers must decide whether the target can satisfy it.
//!
//! # Security boundary
//!
//! A pragma payload is never interpreted as a command by this module.
//!
//! For example, a payload containing:
//!
//! ```text
//! execute("...")
//! ```
//!
//! remains ordinary source data.
//!
//! It is the responsibility of a later, explicitly authorized compiler
//! subsystem to decide whether a recognized pragma is semantically valid.
//! Importing, parsing, validating, serializing, or traversing a pragma must
//! never execute its contents.
//!
//! # Source spans
//!
//! All source locations use the canonical native AST `SourceSpan`/`Span`
//! representation from:
//!
//! `crate::frontend::ast::node::source::span`
//!
//! The pragma deliberately stores separate spans for the whole pragma,
//! namespace/name, and optional payload where the parser can provide them.
//! This allows precise diagnostics without coupling the node to a diagnostic
//! renderer.
//!
//! # Integration contract
//!
//! ## Parser
//!
//! The parser constructs [`Pragma`] values after recognizing pragma syntax.
//! The parser owns lexical interpretation; this module owns structural
//! representation.
//!
//! ## Structural validation
//!
//! Structural validation may call [`Pragma::validate`] and/or validate the
//! enclosing AST node. This validation checks representation invariants only.
//! It does not interpret pragma semantics.
//!
//! ## Semantic analysis
//!
//! Semantic analysis consumes the validated pragma and resolves its namespace,
//! name and meaning using a separate pragma registry/semantic layer.
//!
//! ## ZUIR
//!
//! A pragma is lowered only when its semantics require a ZUIR-visible
//! representation. Unknown or non-semantic pragmas may be preserved as
//! frontend metadata according to the compiler's extension policy.
//!
//! This file must not import ZUIR.
//!
//! ## Visitors
//!
//! Visitors should treat the pragma as a leaf node unless its payload has been
//! represented using a future structured child-node representation. The
//! current payload is intentionally opaque source data.
//!
//! ## Serialization
//!
//! The type derives Serde serialization and must serialize deterministically.
//! No pointers, addresses, filesystem handles, runtime objects, or backend
//! handles are serialized.
//!
//! ## Diagnostics
//!
//! Diagnostics consume the stored spans. This file does not own diagnostic
//! presentation.
//!
//! # Dependency policy
//!
//! Allowed:
//!
//! - Rust standard library;
//! - Serde;
//! - canonical native AST source-span infrastructure.
//!
//! Forbidden:
//!
//! - semantic analysis;
//! - compiler driver;
//! - ZUIR;
//! - quantum IR;
//! - OpenQASM AST;
//! - QIR;
//! - LLVM;
//! - MLIR;
//! - runtime;
//! - backend APIs;
//! - hardware APIs;
//! - filesystem APIs;
//! - networking APIs;
//! - process APIs.
//!
//! # Rust compatibility
//!
//! Rust 1.97 / Rust 1.97.1.
//! Edition 2021.
//! Stable Rust only.
//! No `unsafe` code.
//!
//! # Scalability
//!
//! There is no fixed pragma count, fixed namespace count, fixed payload size,
//! fixed machine size, fixed qubit count, or fixed domain count encoded here.
//!
//! Compiler resource limits belong to configurable frontend/compiler policy.
//! They must not be hidden inside this AST node.
//!
//! "Infinity" therefore means that this representation does not introduce an
//! artificial finite computational-machine limit; actual execution remains
//! bounded by available resources and configured safety policies.

use serde::{Deserialize, Serialize};
use std::fmt;

use super::super::source::span::SourceSpan;

/// Result type used by pragma-local structural operations.
pub type PragmaResult<T> = Result<T, PragmaError>;

/// Structural errors for [`Pragma`].
///
/// These errors deliberately do not represent semantic pragma errors.
/// Semantic interpretation belongs to a later compiler phase.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PragmaError {
    /// A required textual component was empty.
    EmptyField {
        /// Name of the field containing invalid data.
        field: &'static str,
    },

    /// A namespace/name component contains an invalid separator structure.
    InvalidQualifiedName {
        /// The supplied qualified name.
        value: String,
    },

    /// A payload segment was structurally malformed.
    InvalidPayload {
        /// Human-readable structural reason.
        reason: &'static str,
    },

    /// A source span belongs to a different source unit from another span
    /// that is required to belong to the same pragma.
    SourceMismatch,

    /// The source span ordering is inconsistent.
    InvalidSpanOrdering,
}

impl fmt::Display for PragmaError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyField { field } => {
                write!(formatter, "pragma field `{field}` must not be empty")
            }

            Self::InvalidQualifiedName { value } => {
                write!(
                    formatter,
                    "invalid pragma qualified name `{value}`"
                )
            }

            Self::InvalidPayload { reason } => {
                write!(formatter, "invalid pragma payload: {reason}")
            }

            Self::SourceMismatch => {
                write!(
                    formatter,
                    "pragma source spans belong to different source units"
                )
            }

            Self::InvalidSpanOrdering => {
                write!(
                    formatter,
                    "pragma source spans have invalid ordering"
                )
            }
        }
    }
}

impl std::error::Error for PragmaError {}

/// A source-level pragma namespace.
///
/// Namespaces prevent the core AST from acquiring a closed list of all future
/// pragma domains.
///
/// Examples of possible namespaces are conceptually:
///
/// ```text
/// zamani
/// quantum
/// vendor.example
/// tool.example
/// future.domain
/// ```
///
/// The AST does not assign semantics to any namespace.
#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    Deserialize,
)]
pub struct PragmaNamespace {
    value: String,
}

impl PragmaNamespace {
    /// Creates an empty namespace.
    ///
    /// Empty namespaces are useful for source syntaxes where the pragma name
    /// is intentionally unqualified.
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            value: String::new(),
        }
    }

    /// Creates a namespace after structural validation.
    ///
    /// The namespace is source data. This method does not consult a registry.
    pub fn new(value: impl Into<String>) -> PragmaResult<Self> {
        let value = value.into();

        if value.is_empty() {
            return Ok(Self::empty());
        }

        validate_qualified_name(&value)?;

        Ok(Self { value })
    }

    /// Returns the namespace as source text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.value
    }

    /// Returns whether the namespace is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }
}

impl fmt::Display for PragmaNamespace {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.value)
    }
}

/// A pragma name.
///
/// A name is intentionally represented as source text rather than as an enum.
/// This prevents adding a new pragma from requiring modification of the
/// native AST.
#[derive(
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    Deserialize,
)]
pub struct PragmaName {
    value: String,
}

impl PragmaName {
    /// Creates a validated pragma name.
    pub fn new(value: impl Into<String>) -> PragmaResult<Self> {
        let value = value.into();

        if value.is_empty() {
            return Err(PragmaError::EmptyField { field: "name" });
        }

        validate_qualified_name(&value)?;

        Ok(Self { value })
    }

    /// Returns the source spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.value
    }
}

impl fmt::Display for PragmaName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.value)
    }
}

/// A single source-preserved pragma argument.
///
/// Arguments remain opaque strings at this AST layer. This is intentional:
/// interpreting them would make the native AST dependent on every future
/// pragma dialect.
///
/// Later semantic layers may parse an argument according to the resolved
/// pragma definition.
#[derive(
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    Deserialize,
)]
pub struct PragmaArgument {
    span: SourceSpan,
    value: String,
}

impl PragmaArgument {
    /// Creates an argument.
    ///
    /// The argument value is preserved exactly as supplied by the parser.
    #[must_use]
    pub fn new(span: SourceSpan, value: impl Into<String>) -> Self {
        Self {
            span,
            value: value.into(),
        }
    }

    /// Returns the argument source span.
    #[must_use]
    pub const fn span(&self) -> SourceSpan {
        self.span
    }

    /// Returns the source-preserved argument value.
    #[must_use]
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Consumes the argument and returns its value.
    #[must_use]
    pub fn into_value(self) -> String {
        self.value
    }
}

/// Source-preserved pragma payload.
///
/// The payload is represented as an ordered sequence rather than as a fixed
/// tuple or fixed number of fields. This is essential for extensibility.
///
/// No pragma-specific grammar is interpreted here.
///
/// For example, all of the following can be represented without changing this
/// type:
///
/// ```text
/// #pragma zamani optimize
/// #pragma quantum something q[0] q[1]
/// #pragma vendor.example calibration foo
/// #pragma future.example arbitrary future syntax
/// ```
///
/// The parser may choose to preserve a richer lexical form later. The
/// semantic layer decides whether a payload has meaning.
#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    Hash,
    PartialEq,
    Serialize,
    Deserialize,
)]
pub struct PragmaPayload {
    arguments: Vec<PragmaArgument>,
}

impl PragmaPayload {
    /// Creates an empty payload.
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            arguments: Vec::new(),
        }
    }

    /// Creates a payload from already parsed arguments.
    #[must_use]
    pub fn new(arguments: Vec<PragmaArgument>) -> Self {
        Self { arguments }
    }

    /// Returns the arguments without transferring ownership.
    #[must_use]
    pub fn arguments(&self) -> &[PragmaArgument] {
        &self.arguments
    }

    /// Returns the number of arguments.
    #[must_use]
    pub fn len(&self) -> usize {
        self.arguments.len()
    }

    /// Returns whether the payload has no arguments.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.arguments.is_empty()
    }

    /// Returns an iterator over arguments.
    pub fn iter(&self) -> std::slice::Iter<'_, PragmaArgument> {
        self.arguments.iter()
    }

    /// Appends an argument.
    ///
    /// This method performs no semantic interpretation.
    pub fn push(&mut self, argument: PragmaArgument) {
        self.arguments.push(argument);
    }

    /// Consumes the payload and returns its arguments.
    #[must_use]
    pub fn into_arguments(self) -> Vec<PragmaArgument> {
        self.arguments
    }
}

/// A complete source-level pragma.
///
/// # Representation
///
/// ```text
/// Pragma
/// ├── span
/// ├── namespace
/// ├── name
/// ├── payload
/// └── terminator span (optional)
/// ```
///
/// The node contains only source-level information.
///
/// It does not contain a resolved semantic ID, backend ID, target device,
/// resource allocation, or executable callback.
#[derive(
    Clone,
    Debug,
    Eq,
    Hash,
    PartialEq,
    Serialize,
    Deserialize,
)]
pub struct Pragma {
    /// Complete source span covering the pragma.
    span: SourceSpan,

    /// Optional namespace.
    namespace: PragmaNamespace,

    /// Pragma name.
    name: PragmaName,

    /// Optional source-preserved payload.
    payload: PragmaPayload,

    /// Optional span for the terminating syntax.
    ///
    /// This remains optional because different source dialects can have
    /// different pragma termination rules.
    terminator_span: Option<SourceSpan>,
}

impl Pragma {
    /// Creates a pragma without an explicit terminator span.
    ///
    /// The parser should use this constructor when the grammar treats the
    /// terminator as part of the surrounding statement or when no distinct
    /// terminator token exists.
    pub fn new(
        span: SourceSpan,
        namespace: PragmaNamespace,
        name: PragmaName,
        payload: PragmaPayload,
    ) -> PragmaResult<Self> {
        let pragma = Self {
            span,
            namespace,
            name,
            payload,
            terminator_span: None,
        };

        pragma.validate()?;
        Ok(pragma)
    }

    /// Creates a pragma with a separately tracked terminator span.
    pub fn with_terminator(
        span: SourceSpan,
        namespace: PragmaNamespace,
        name: PragmaName,
        payload: PragmaPayload,
        terminator_span: SourceSpan,
    ) -> PragmaResult<Self> {
        let pragma = Self {
            span,
            namespace,
            name,
            payload,
            terminator_span: Some(terminator_span),
        };

        pragma.validate()?;
        Ok(pragma)
    }

    /// Returns the complete pragma span.
    #[must_use]
    pub const fn span(&self) -> SourceSpan {
        self.span
    }

    /// Returns the namespace.
    #[must_use]
    pub fn namespace(&self) -> &PragmaNamespace {
        &self.namespace
    }

    /// Returns the pragma name.
    #[must_use]
    pub fn name(&self) -> &PragmaName {
        &self.name
    }

    /// Returns the payload.
    #[must_use]
    pub fn payload(&self) -> &PragmaPayload {
        &self.payload
    }

    /// Returns the optional terminator span.
    #[must_use]
    pub const fn terminator_span(&self) -> Option<SourceSpan> {
        self.terminator_span
    }

    /// Returns whether the pragma has a namespace.
    #[must_use]
    pub fn is_namespaced(&self) -> bool {
        !self.namespace.is_empty()
    }

    /// Returns the canonical qualified source name.
    ///
    /// The returned string is newly allocated because the namespace and name
    /// are stored independently.
    #[must_use]
    pub fn qualified_name(&self) -> String {
        if self.namespace.is_empty() {
            self.name.as_str().to_owned()
        } else {
            let mut result =
                String::with_capacity(
                    self.namespace.as_str().len()
                        + 1
                        + self.name.as_str().len(),
                );

            result.push_str(self.namespace.as_str());
            result.push('.');
            result.push_str(self.name.as_str());

            result
        }
    }

    /// Validates structural invariants.
    ///
    /// This method deliberately does not resolve or interpret the pragma.
    ///
    /// It is therefore safe to call during parsing, AST validation,
    /// deserialization validation, fuzzing, and tooling.
    pub fn validate(&self) -> PragmaResult<()> {
        if self.name.as_str().is_empty() {
            return Err(PragmaError::EmptyField { field: "name" });
        }

        validate_qualified_name(self.name.as_str())?;

        if !self.namespace.is_empty() {
            validate_qualified_name(self.namespace.as_str())?;
        }

        let source = self.span.source();

        if let Some(terminator) = self.terminator_span {
            if terminator.source() != source {
                return Err(PragmaError::SourceMismatch);
            }

            if terminator.start().as_raw() < self.span.start().as_raw()
                || terminator.end().as_raw() > self.span.end().as_raw()
            {
                return Err(PragmaError::InvalidSpanOrdering);
            }
        }

        for argument in self.payload.iter() {
            if argument.span().source() != source {
                return Err(PragmaError::SourceMismatch);
            }

            if argument.span().start().as_raw()
                < self.span.start().as_raw()
                || argument.span().end().as_raw()
                    > self.span.end().as_raw()
            {
                return Err(PragmaError::InvalidSpanOrdering);
            }
        }

        Ok(())
    }

    /// Returns an iterator over the source payload arguments.
    pub fn arguments(
        &self,
    ) -> std::slice::Iter<'_, PragmaArgument> {
        self.payload.iter()
    }

    /// Returns the number of payload arguments.
    #[must_use]
    pub fn argument_count(&self) -> usize {
        self.payload.len()
    }
}

/// Trait used by the native AST for source-spanned nodes.
///
/// This implementation deliberately stays local to the source AST contract.
pub trait Spanned {
    /// Returns the node's complete source span.
    fn span(&self) -> SourceSpan;
}

impl Spanned for Pragma {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

impl Spanned for PragmaArgument {
    fn span(&self) -> SourceSpan {
        self.span
    }
}

/// Validates a namespaced identifier without imposing a finite vocabulary.
///
/// The grammar intentionally accepts ASCII identifier components here. If the
/// Zamani lexer defines a broader identifier grammar, the parser may construct
/// these values through a future lexical-validation boundary without changing
/// the representation.
///
/// Qualified components are separated by `.`.
///
/// Examples:
///
/// ```text
/// optimize
/// quantum.optimize
/// vendor.example.directive
/// ```
///
/// Invalid examples:
///
/// ```text
/// ""
/// ".foo"
/// "foo."
/// "foo..bar"
/// "foo bar"
/// ```
fn validate_qualified_name(value: &str) -> PragmaResult<()> {
    if value.is_empty() {
        return Err(PragmaError::EmptyField {
            field: "qualified_name",
        });
    }

    for component in value.split('.') {
        if component.is_empty() {
            return Err(PragmaError::InvalidQualifiedName {
                value: value.to_owned(),
            });
        }

        let mut characters = component.chars();

        let first = characters.next().ok_or_else(|| {
            PragmaError::InvalidQualifiedName {
                value: value.to_owned(),
            }
        })?;

        if !(first == '_'
            || first.is_ascii_alphabetic())
        {
            return Err(PragmaError::InvalidQualifiedName {
                value: value.to_owned(),
            });
        }

        for character in characters {
            if !(character == '_'
                || character.is_ascii_alphanumeric())
            {
                return Err(PragmaError::InvalidQualifiedName {
                    value: value.to_owned(),
                });
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn span(start: usize, end: usize) -> SourceSpan {
        SourceSpan::from_usize(
            crate::frontend::ast::node::source::span::SourceId::from_raw(
                1,
            ),
            start,
            end,
        )
        .expect("test span must be representable")
    }

    #[test]
    fn empty_namespace_is_valid() {
        let namespace = PragmaNamespace::new("")
            .expect("empty namespace is allowed");

        assert!(namespace.is_empty());
        assert_eq!(namespace.as_str(), "");
    }

    #[test]
    fn qualified_namespace_is_preserved() {
        let namespace =
            PragmaNamespace::new("quantum.compiler")
                .expect("namespace must be valid");

        assert_eq!(namespace.as_str(), "quantum.compiler");
    }

    #[test]
    fn pragma_name_is_preserved() {
        let name =
            PragmaName::new("optimize")
                .expect("pragma name must be valid");

        assert_eq!(name.as_str(), "optimize");
    }

    #[test]
    fn qualified_names_are_extensible() {
        let name =
            PragmaName::new("future.domain.directive")
                .expect("future namespaces must remain representable");

        assert_eq!(
            name.as_str(),
            "future.domain.directive"
        );
    }

    #[test]
    fn invalid_qualified_names_are_rejected() {
        for value in [
            "",
            ".foo",
            "foo.",
            "foo..bar",
            "foo bar",
            "1foo",
            "foo/bar",
        ] {
            assert!(
                PragmaName::new(value).is_err(),
                "expected `{value}` to be rejected"
            );
        }
    }

    #[test]
    fn payload_is_ordered_and_unbounded_by_schema() {
        let mut payload = PragmaPayload::empty();

        for index in 0..10_000usize {
            payload.push(PragmaArgument::new(
                span(index, index),
                index.to_string(),
            ));
        }

        assert_eq!(payload.len(), 10_000);
        assert_eq!(
            payload.arguments()[9_999].value(),
            "9999"
        );
    }

    #[test]
    fn pragma_round_trip_through_serde() {
        let namespace =
            PragmaNamespace::new("quantum.compiler")
                .expect("namespace");

        let name =
            PragmaName::new("directive")
                .expect("name");

        let payload = PragmaPayload::new(vec![
            PragmaArgument::new(span(10, 14), "q[0]"),
            PragmaArgument::new(span(15, 19), "q[1]"),
        ]);

        let pragma =
            Pragma::new(
                span(0, 20),
                namespace,
                name,
                payload,
            )
            .expect("pragma");

        let encoded =
            serde_json::to_string(&pragma)
                .expect("serialization");

        let decoded: Pragma =
            serde_json::from_str(&encoded)
                .expect("deserialization");

        assert_eq!(pragma, decoded);
        assert_eq!(
            decoded.qualified_name(),
            "quantum.compiler.directive"
        );
    }

    #[test]
    fn pragma_validation_never_executes_payload() {
        let namespace =
            PragmaNamespace::new("test")
                .expect("namespace");

        let name =
            PragmaName::new("payload")
                .expect("name");

        let payload =
            PragmaPayload::new(vec![
                PragmaArgument::new(
                    span(0, 1),
                    "execute(\"rm -rf /\")",
                ),
            ]);

        let pragma =
            Pragma::new(
                span(0, 30),
                namespace,
                name,
                payload,
            )
            .expect("pragma must remain data");

        assert_eq!(
            pragma.arguments().next().expect("argument").value(),
            "execute(\"rm -rf /\")"
        );
    }

    #[test]
    fn source_spans_must_belong_to_same_source() {
        let namespace =
            PragmaNamespace::new("test")
                .expect("namespace");

        let name =
            PragmaName::new("directive")
                .expect("name");

        let different_source =
            SourceSpan::from_usize(
                crate::frontend::ast::node::source::span::SourceId::from_raw(
                    2,
                ),
                2,
                3,
            )
            .expect("span");

        let payload =
            PragmaPayload::new(vec![
                PragmaArgument::new(
                    different_source,
                    "x",
                ),
            ]);

        let result =
            Pragma::new(
                span(0, 4),
                namespace,
                name,
                payload,
            );

        assert_eq!(
            result,
            Err(PragmaError::SourceMismatch)
        );
    }

    #[test]
    fn_argument_spans_must_be_inside_pragma_span() {
        let namespace =
            PragmaNamespace::new("test")
                .expect("namespace");

        let name =
            PragmaName::new("directive")
                .expect("name");

        let payload =
            PragmaPayload::new(vec![
                PragmaArgument::new(
                    span(10, 12),
                    "outside",
                ),
            ]);

        let result =
            Pragma::new(
                span(0, 5),
                namespace,
                name,
                payload,
            );

        assert_eq!(
            result,
            Err(PragmaError::InvalidSpanOrdering)
        );
    }

    #[test]
    fn terminator_span_must_be_inside_complete_span() {
        let namespace =
            PragmaNamespace::new("test")
                .expect("namespace");

        let name =
            PragmaName::new("directive")
                .expect("name");

        let result =
            Pragma::with_terminator(
                span(0, 5),
                namespace,
                name,
                PragmaPayload::empty(),
                span(5, 6),
            );

        assert_eq!(
            result,
            Err(PragmaError::InvalidSpanOrdering)
        );
    }

    #[test]
    fn empty_payload_is_valid() {
        let pragma =
            Pragma::new(
                span(0, 5),
                PragmaNamespace::empty(),
                PragmaName::new("once")
                    .expect("name"),
                PragmaPayload::empty(),
            )
            .expect("pragma");

        assert!(pragma.payload().is_empty());
        assert_eq!(
            pragma.qualified_name(),
            "once"
        );
    }

    #[test]
    fn qualified_name_is_deterministic() {
        let pragma =
            Pragma::new(
                span(0, 5),
                PragmaNamespace::new("a.b")
                    .expect("namespace"),
                PragmaName::new("c")
                    .expect("name"),
                PragmaPayload::empty(),
            )
            .expect("pragma");

        assert_eq!(
            pragma.qualified_name(),
            "a.b.c"
        );

        assert_eq!(
            pragma.qualified_name(),
            "a.b.c"
        );
    }
}