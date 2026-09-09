//! Source-level literal expressions for the Zamani native AST.
//!
//! # Architectural contract
//!
//! This module owns the syntax representation of literals.
//!
//! It deliberately does **not**:
//! - evaluate literals;
//! - infer their semantic types;
//! - convert integers to `i64`/`u64`;
//! - convert floating-point values to `f32`/`f64`;
//! - impose machine word sizes;
//! - impose quantum-resource sizes;
//! - interpret backend-specific literal formats;
//! - depend on a quantum backend;
//! - depend on hardware;
//! - depend on ZUIR;
//! - depend on semantic analysis;
//! - depend on LLVM/QIR/MLIR;
//! - perform constant folding;
//! - perform target-specific lowering.
//!
//! The native AST represents **what was written**. Later compiler stages
//! determine what that source literal means.
//!
//! # Why literals preserve their source representation
//!
//! A source literal such as:
//!
//! ```text
//! 18446744073709551616
//! ```
//!
//! must not be forced into an `i64` merely because the host compiler happens
//! to use one. Likewise:
//!
//! ```text
//! 0.123456789012345678901234567890
//! ```
//!
//! must not be silently converted to `f64` during parsing.
//!
//! The AST therefore preserves the literal spelling as source text.
//! Semantic analysis can subsequently choose arbitrary precision,
//! checked representations, symbolic representations, target-specific
//! representations, or reject the literal according to the language's
//! semantic rules.
//!
//! # Quantum and future-domain neutrality
//!
//! This module intentionally contains no variants such as:
//!
//! ```text
//! QuantumLiteral
//! QubitLiteral
//! GateLiteral
//! HardwareLiteral
//! IBMQuantumLiteral
//! IonQLiteral
//! SurfaceCodeLiteral
//! ```
//!
//! Domain-specific literal syntaxes must use the extensible `Extension`
//! representation or a separate language-extension AST.
//!
//! This is necessary for Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
//! (POCO-REAF): adding a new quantum technology, accelerator, architecture,
//! numerical representation, or computational domain must not require
//! changing this core file.
//!
//! # Integration contract
//!
//! ```text
//! source
//!   ↓
//! lexer
//!   ↓
//! parser
//!   ↓
//! Literal
//!   ↓
//! structural AST validation
//!   ↓
//! semantic analysis
//!   ↓
//! semantic literal/value/type
//!   ↓
//! ZUIR
//!   ↓
//! domain lowering
//!   ↓
//! target/backend realization
//! ```
//!
//! The parser owns syntactic recognition.
//! This module owns source-level representation.
//! Semantic analysis owns interpretation.
//! ZUIR owns universal computational semantics.
//!
//! # Dependencies
//!
//! Allowed dependencies are deliberately limited to:
//! - the common AST `Node`;
//! - the common AST `NodeId` where construction requires it;
//! - the common AST `NodeMetadata`;
//! - standard-library types;
//! - `serde` serialization traits.
//!
//! This file must never depend on semantic analysis, compiler backends,
//! hardware, quantum IR, scheduling, routing, QEC, resilience, calibration,
//! execution, or target-specific representations.
//!
//! # Scalability
//!
//! There are no language-level fixed limits on:
//! - integer width;
//! - decimal precision;
//! - literal text length;
//! - literal collection size;
//! - namespace length;
//! - extension name length.
//!
//! Actual allocation/resource limits belong to configurable compiler policy,
//! parser limits, validation limits, or execution environments rather than
//! to the literal AST schema.
//!
//! # Determinism
//!
//! Literal serialization preserves the represented source spelling and uses
//! deterministic data structures for extensible metadata.
//!
//! # Security
//!
//! This module performs no unchecked numeric conversion. This is important
//! because literals originate from untrusted source input.
//!
//! In particular, parsing a very large integer must not overflow a fixed
//! machine integer merely because the AST is being constructed.
//!
//! # Source preservation
//!
//! `raw` contains the source spelling recognized by the lexer/parser.
//! The parser/source infrastructure remains responsible for associating the
//! expression with its canonical `Span` through `Node`.
//!
//! # Visitor/traversal contract
//!
//! Literals are leaf expressions and therefore have no child `NodeId`s.
//! Generic AST traversal should visit the `Expression` containing this
//! literal and then stop at this node.
//!
//! # Semantic lowering contract
//!
//! Every literal kind must have a defined semantic interpretation.
//! The semantic layer must consume the raw spelling rather than assuming
//! that this AST has already selected a target representation.
//!
//! Extension literals must be resolved through the extension registry.
//! Unknown extensions may be rejected, preserved, or deferred according to
//! the compiler's explicit extension policy.
//!
//! # Compatibility
//!
//! This representation supersedes legacy forms such as:
//!
//! ```text
//! Integer(i64, Span)
//! Float(f64, Span)
//! Quantum(String, Span)
//! Nano(String, Span)
//! MTS(String, Span)
//! ```
//!
//! In particular, quantum/nano/MTS-specific variants must not be recreated
//! here. Their source forms should be represented by generic literals or
//! namespaced extensions and interpreted downstream.

use serde::{Deserialize, Serialize};
use std::fmt;

use super::super::{
    metadata::NodeMetadata,
    node::Node,
    node_id::NodeId,
};
use crate::frontend::ast::source::Span;

/// A native Zamani literal expression.
///
/// `Literal` contains the common AST node header and a source-level literal
/// kind. It deliberately does not contain a target-language value.
///
/// # Invariant
///
/// `node.span` must cover the complete source representation of the literal.
///
/// The parser is responsible for establishing the span. Structural
/// validation may subsequently verify the span against the source map.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Literal {
    /// Common AST identity, source span, and metadata.
    pub node: Node,

    /// Source-level literal classification and spelling.
    pub kind: LiteralKind,
}

impl Literal {
    /// Construct a literal from a node identity, source span, and kind.
    ///
    /// This constructor performs no semantic conversion.
    pub fn new(id: NodeId, span: Span, kind: LiteralKind) -> Self {
        Self {
            node: Node::new(id, span),
            kind,
        }
    }

    /// Construct a literal with node metadata.
    pub fn with_metadata(
        id: NodeId,
        span: Span,
        metadata: NodeMetadata,
        kind: LiteralKind,
    ) -> Self {
        Self {
            node: Node::with_metadata(id, span, metadata),
            kind,
        }
    }

    /// Return the stable AST node ID.
    pub fn id(&self) -> NodeId {
        self.node.id()
    }

    /// Return the source span.
    pub fn span(&self) -> Span {
        self.node.span()
    }

    /// Return the optional node metadata.
    pub fn metadata(&self) -> Option<&NodeMetadata> {
        self.node.metadata()
    }

    /// Attach metadata to this literal.
    pub fn set_metadata(&mut self, metadata: NodeMetadata) {
        self.node.set_metadata(metadata);
    }

    /// Remove and return the literal's metadata.
    pub fn take_metadata(&mut self) -> Option<NodeMetadata> {
        self.node.take_metadata()
    }

    /// Return the literal kind.
    pub fn kind(&self) -> &LiteralKind {
        &self.kind
    }

    /// Return mutable access to the literal kind.
    pub fn kind_mut(&mut self) -> &mut LiteralKind {
        &mut self.kind
    }

    /// Return the original source spelling of this literal.
    ///
    /// This is intentionally not parsed into a machine numeric type.
    pub fn raw(&self) -> &str {
        self.kind.raw()
    }

    /// Returns `true` when this literal is an extensible namespaced literal.
    pub fn is_extension(&self) -> bool {
        matches!(self.kind, LiteralKind::Extension { .. })
    }

    /// Return the extension namespace if this is an extension literal.
    pub fn extension_namespace(&self) -> Option<&str> {
        match &self.kind {
            LiteralKind::Extension { namespace, .. } => Some(namespace),
            _ => None,
        }
    }

    /// Return the extension name if this is an extension literal.
    pub fn extension_name(&self) -> Option<&str> {
        match &self.kind {
            LiteralKind::Extension { name, .. } => Some(name),
            _ => None,
        }
    }

    /// Return the number of AST children.
    ///
    /// Literals are leaves, so this is always zero.
    pub const fn children_count(&self) -> usize {
        0
    }

    /// Return whether this literal has no AST children.
    pub const fn is_leaf(&self) -> bool {
        true
    }

    /// Validate local structural invariants.
    ///
    /// This deliberately does not perform semantic validation.
    ///
    /// `Ok(())` means the literal's local representation is structurally
    /// usable. Semantic validity belongs to the validation/semantic phases.
    pub fn validate_structure(&self) -> Result<(), LiteralStructureError> {
        if self.id().is_invalid() {
            return Err(LiteralStructureError::InvalidNodeId);
        }

        if self.span().is_empty() && self.raw().is_empty() {
            return Err(LiteralStructureError::EmptyLiteral);
        }

        match &self.kind {
            LiteralKind::Extension {
                namespace,
                name,
                ..
            } => {
                if namespace.is_empty() {
                    return Err(LiteralStructureError::EmptyExtensionNamespace);
                }

                if name.is_empty() {
                    return Err(LiteralStructureError::EmptyExtensionName);
                }
            }
            _ => {}
        }

        Ok(())
    }
}

/// Source-level classification of a literal.
///
/// The representation is intentionally syntax-oriented. In particular,
/// numeric values remain strings until semantic analysis chooses an
/// appropriate mathematical or machine representation.
///
/// Every variant carries its original source spelling through `raw`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LiteralKind {
    /// Integer literal.
    ///
    /// Examples:
    ///
    /// ```text
    /// 0
    /// 42
    /// 1_000_000
    /// 0xff
    /// 0b1010
    /// 0o755
    /// ```
    ///
    /// The AST does not impose an integer width.
    Integer {
        /// Exact source spelling.
        raw: String,

        /// Source radix.
        radix: IntegerRadix,
    },

    /// Floating-point/real-number literal.
    ///
    /// The spelling remains textual so semantic analysis can choose the
    /// required precision or mathematical representation.
    Float {
        /// Exact source spelling.
        raw: String,
    },

    /// String literal.
    ///
    /// The source spelling is retained rather than forcing an immediate
    /// decoding policy into the AST.
    String {
        /// Exact source spelling including the literal's delimiters.
        raw: String,
    },

    /// Character literal.
    ///
    /// The AST preserves the source spelling. Character decoding and Unicode
    /// semantic validation belong to the semantic layer.
    Char {
        /// Exact source spelling including delimiters.
        raw: String,
    },

    /// Boolean literal.
    Boolean {
        /// Exact source spelling.
        raw: String,

        /// Syntactic boolean value recognized by the parser.
        value: bool,
    },

    /// Null/nil-like literal.
    ///
    /// Its exact semantic type is determined later.
    Null {
        /// Exact source spelling.
        raw: String,
    },

    /// Unit literal.
    ///
    /// The exact syntax is language-defined; the AST preserves it without
    /// attaching target-specific semantics.
    Unit {
        /// Exact source spelling.
        raw: String,
    },

    /// An extensible, namespaced literal.
    ///
    /// This is the mechanism that prevents the core AST from requiring a new
    /// enum variant whenever a new computational domain or literal notation
    /// is introduced.
    ///
    /// Examples of possible *future* namespaces include:
    ///
    /// ```text
    /// quantum::...
    /// hdl::...
    /// numeric::...
    /// time::...
    /// accelerator::...
    /// domain::...
    /// ```
    ///
    /// These names are not built into this module.
    Extension {
        /// Extension namespace.
        namespace: String,

        /// Extension-defined literal name.
        name: String,

        /// Exact source spelling.
        raw: String,
    },
}

impl LiteralKind {
    /// Construct an integer literal while preserving its source spelling.
    pub fn integer(raw: impl Into<String>, radix: IntegerRadix) -> Self {
        Self::Integer {
            raw: raw.into(),
            radix,
        }
    }

    /// Construct a decimal integer literal.
    pub fn decimal_integer(raw: impl Into<String>) -> Self {
        Self::integer(raw, IntegerRadix::Decimal)
    }

    /// Construct a binary integer literal.
    pub fn binary_integer(raw: impl Into<String>) -> Self {
        Self::integer(raw, IntegerRadix::Binary)
    }

    /// Construct an octal integer literal.
    pub fn octal_integer(raw: impl Into<String>) -> Self {
        Self::integer(raw, IntegerRadix::Octal)
    }

    /// Construct a hexadecimal integer literal.
    pub fn hexadecimal_integer(raw: impl Into<String>) -> Self {
        Self::integer(raw, IntegerRadix::Hexadecimal)
    }

    /// Construct a floating-point literal.
    pub fn float(raw: impl Into<String>) -> Self {
        Self::Float { raw: raw.into() }
    }

    /// Construct a string literal.
    pub fn string(raw: impl Into<String>) -> Self {
        Self::String { raw: raw.into() }
    }

    /// Construct a character literal.
    pub fn character(raw: impl Into<String>) -> Self {
        Self::Char { raw: raw.into() }
    }

    /// Construct a boolean literal.
    pub fn boolean(value: bool, raw: impl Into<String>) -> Self {
        Self::Boolean {
            raw: raw.into(),
            value,
        }
    }

    /// Construct a null literal.
    pub fn null(raw: impl Into<String>) -> Self {
        Self::Null { raw: raw.into() }
    }

    /// Construct a unit literal.
    pub fn unit(raw: impl Into<String>) -> Self {
        Self::Unit { raw: raw.into() }
    }

    /// Construct an extensible literal.
    pub fn extension(
        namespace: impl Into<String>,
        name: impl Into<String>,
        raw: impl Into<String>,
    ) -> Self {
        Self::Extension {
            namespace: namespace.into(),
            name: name.into(),
            raw: raw.into(),
        }
    }

    /// Return the exact source spelling.
    pub fn raw(&self) -> &str {
        match self {
            Self::Integer { raw, .. }
            | Self::Float { raw }
            | Self::String { raw }
            | Self::Char { raw }
            | Self::Boolean { raw, .. }
            | Self::Null { raw }
            | Self::Unit { raw }
            | Self::Extension { raw, .. } => raw,
        }
    }

    /// Return the literal category.
    pub const fn category(&self) -> LiteralCategory {
        match self {
            Self::Integer { .. } => LiteralCategory::Integer,
            Self::Float { .. } => LiteralCategory::Float,
            Self::String { .. } => LiteralCategory::String,
            Self::Char { .. } => LiteralCategory::Character,
            Self::Boolean { .. } => LiteralCategory::Boolean,
            Self::Null { .. } => LiteralCategory::Null,
            Self::Unit { .. } => LiteralCategory::Unit,
            Self::Extension { .. } => LiteralCategory::Extension,
        }
    }

    /// Returns true when this is a numeric literal.
    pub const fn is_numeric(&self) -> bool {
        matches!(self, Self::Integer { .. } | Self::Float { .. })
    }

    /// Returns true when this is an extension literal.
    pub const fn is_extension(&self) -> bool {
        matches!(self, Self::Extension { .. })
    }

    /// Return the integer radix when applicable.
    pub const fn radix(&self) -> Option<IntegerRadix> {
        match self {
            Self::Integer { radix, .. } => Some(*radix),
            _ => None,
        }
    }

    /// Return the boolean value when this is a boolean literal.
    pub const fn boolean_value(&self) -> Option<bool> {
        match self {
            Self::Boolean { value, .. } => Some(*value),
            _ => None,
        }
    }
}

/// Syntactic radix of an integer literal.
///
/// This is syntax information, not a semantic numeric representation.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IntegerRadix {
    /// Binary (`0b...`).
    Binary,

    /// Octal (`0o...`).
    Octal,

    /// Decimal.
    Decimal,

    /// Hexadecimal (`0x...`).
    Hexadecimal,

    /// An explicitly recognized but non-standard/future radix.
    ///
    /// The value is the mathematical radix expressed as a positive
    /// non-zero integer. Validation of supported ranges belongs to the
    /// parser/semantic layer.
    Custom(u32),
}

impl IntegerRadix {
    /// Return the mathematical base where known.
    pub const fn base(self) -> u32 {
        match self {
            Self::Binary => 2,
            Self::Octal => 8,
            Self::Decimal => 10,
            Self::Hexadecimal => 16,
            Self::Custom(base) => base,
        }
    }

    /// Return whether this radix is one of the standard source-language
    /// radices.
    pub const fn is_standard(self) -> bool {
        matches!(
            self,
            Self::Binary
                | Self::Octal
                | Self::Decimal
                | Self::Hexadecimal
        )
    }
}

impl fmt::Display for IntegerRadix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Binary => f.write_str("binary"),
            Self::Octal => f.write_str("octal"),
            Self::Decimal => f.write_str("decimal"),
            Self::Hexadecimal => f.write_str("hexadecimal"),
            Self::Custom(base) => write!(f, "base-{base}"),
        }
    }
}

/// Coarse syntactic category of a literal.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LiteralCategory {
    /// Integer literal.
    Integer,

    /// Floating-point/real-number literal.
    Float,

    /// String literal.
    String,

    /// Character literal.
    Character,

    /// Boolean literal.
    Boolean,

    /// Null/nil-like literal.
    Null,

    /// Unit literal.
    Unit,

    /// Namespaced extension literal.
    Extension,
}

/// Structural errors local to a literal node.
///
/// These errors deliberately do not attempt to determine whether a literal
/// is semantically valid. For example, an arbitrarily large integer is not
/// structurally invalid merely because it cannot fit into a machine integer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LiteralStructureError {
    /// The common AST node has not been assigned a valid ID.
    InvalidNodeId,

    /// A literal contains no source spelling and therefore cannot represent
    /// a source literal.
    EmptyLiteral,

    /// An extension literal has no namespace.
    EmptyExtensionNamespace,

    /// An extension literal has no extension-defined name.
    EmptyExtensionName,
}

impl fmt::Display for LiteralStructureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidNodeId => {
                f.write_str("literal contains an invalid AST node ID")
            }
            Self::EmptyLiteral => {
                f.write_str("literal contains an empty source representation")
            }
            Self::EmptyExtensionNamespace => {
                f.write_str("literal extension namespace cannot be empty")
            }
            Self::EmptyExtensionName => {
                f.write_str("literal extension name cannot be empty")
            }
        }
    }
}

impl std::error::Error for LiteralStructureError {}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::frontend::ast::source::{source_id::SourceId, Span};

    fn span() -> Span {
        Span::new(SourceId::new(1), 0, 1)
    }

    #[test]
    fn preserves_arbitrarily_large_integer_spelling() {
        let source =
            "999999999999999999999999999999999999999999999999999999999999999999";

        let literal = Literal::new(
            NodeId::new(1),
            Span::new(SourceId::new(1), 0, source.len() as u64),
            LiteralKind::decimal_integer(source),
        );

        assert_eq!(literal.raw(), source);
        assert_eq!(literal.kind().category(), LiteralCategory::Integer);
        assert_eq!(literal.kind().radix(), Some(IntegerRadix::Decimal));
    }

    #[test]
    fn does_not_force_float_into_machine_precision() {
        let source = "0.123456789012345678901234567890123456789";

        let literal = Literal::new(
            NodeId::new(1),
            Span::new(SourceId::new(1), 0, source.len() as u64),
            LiteralKind::float(source),
        );

        assert_eq!(literal.raw(), source);
        assert_eq!(literal.kind().category(), LiteralCategory::Float);
    }

    #[test]
    fn preserves_string_source_spelling() {
        let literal = Literal::new(
            NodeId::new(1),
            span(),
            LiteralKind::string("\"hello\\nworld\""),
        );

        assert_eq!(literal.raw(), "\"hello\\nworld\"");
    }

    #[test]
    fn preserves_character_source_spelling() {
        let literal = Literal::new(
            NodeId::new(1),
            span(),
            LiteralKind::character("'\\n'"),
        );

        assert_eq!(literal.raw(), "'\\n'");
    }

    #[test]
    fn boolean_keeps_source_and_value() {
        let literal = Literal::new(
            NodeId::new(1),
            span(),
            LiteralKind::boolean(true, "true"),
        );

        assert_eq!(literal.raw(), "true");
        assert_eq!(literal.kind().boolean_value(), Some(true));
    }

    #[test]
    fn extension_literal_is_domain_neutral() {
        let literal = Literal::new(
            NodeId::new(1),
            span(),
            LiteralKind::extension(
                "future.domain",
                "resource",
                "future.domain::resource(1000000)",
            ),
        );

        assert!(literal.is_extension());
        assert_eq!(literal.extension_namespace(), Some("future.domain"));
        assert_eq!(literal.extension_name(), Some("resource"));
    }

    #[test]
    fn literal_is_a_leaf() {
        let literal = Literal::new(
            NodeId::new(1),
            span(),
            LiteralKind::decimal_integer("42"),
        );

        assert!(literal.is_leaf());
        assert_eq!(literal.children_count(), 0);
    }

    #[test]
    fn structure_validation_accepts_large_values() {
        let literal = Literal::new(
            NodeId::new(1),
            Span::new(SourceId::new(1), 0, 100),
            LiteralKind::decimal_integer(
                "999999999999999999999999999999999999999999999999999999999",
            ),
        );

        assert!(literal.validate_structure().is_ok());
    }

    #[test]
    fn structure_validation_rejects_invalid_node_id() {
        let literal = Literal::new(
            NodeId::INVALID,
            span(),
            LiteralKind::decimal_integer("42"),
        );

        assert_eq!(
            literal.validate_structure(),
            Err(LiteralStructureError::InvalidNodeId)
        );
    }

    #[test]
    fn structure_validation_rejects_empty_extension_namespace() {
        let literal = Literal::new(
            NodeId::new(1),
            span(),
            LiteralKind::extension("", "value", "value"),
        );

        assert_eq!(
            literal.validate_structure(),
            Err(LiteralStructureError::EmptyExtensionNamespace)
        );
    }

    #[test]
    fn serialization_round_trip_preserves_literal() {
        let literal = Literal::new(
            NodeId::new(1),
            Span::new(SourceId::new(1), 0, 10),
            LiteralKind::extension(
                "example",
                "future_literal",
                "example::future_literal(12345678901234567890)",
            ),
        );

        let encoded =
            serde_json::to_string(&literal).expect("literal serialization must succeed");

        let decoded: Literal =
            serde_json::from_str(&encoded).expect("literal deserialization must succeed");

        assert_eq!(literal, decoded);
    }

    #[test]
    fn radix_is_deterministic() {
        assert_eq!(IntegerRadix::Binary.base(), 2);
        assert_eq!(IntegerRadix::Octal.base(), 8);
        assert_eq!(IntegerRadix::Decimal.base(), 10);
        assert_eq!(IntegerRadix::Hexadecimal.base(), 16);
        assert_eq!(IntegerRadix::Custom(36).base(), 36);
    }
}