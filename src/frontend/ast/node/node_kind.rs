//! Stable, extensible classification of native Zamani AST nodes.
//!
//! # Architectural role
//!
//! `NodeKind` identifies the *source-language kind* of an AST node. It does
//! not describe:
//!
//! - a quantum gate;
//! - a physical qubit;
//! - a hardware instruction;
//! - a backend;
//! - a compiler optimization;
//! - a scheduling operation;
//! - a QIR/LLVM/MLIR operation;
//! - a vendor;
//! - a processor topology;
//! - a quantum error-correction implementation.
//!
//! Those concerns belong to later compiler layers.
//!
//! # POCO-REAF
//!
//! The native AST must remain stable while the implementation target changes.
//! Consequently, the core node-kind set is intentionally small and the
//! representation has an extensible `Extension` form.
//!
//! A new language/domain extension therefore does not require adding another
//! variant to this file.
//!
//! # Design
//!
//! ```text
//! NodeKind
//! ├── Core(CoreNodeKind)
//! └── Extension(ExtensionNodeKind)
//!                       ├── namespace
//!                       └── name
//! ```
//!
//! `CoreNodeKind` contains only constructs owned by the native Zamani
//! language. `ExtensionNodeKind` provides an open namespace for language
//! extensions without coupling the AST to any particular computational
//! technology.
//!
//! # Dependency policy
//!
//! This file intentionally depends only on the Rust standard library.
//!
//! In particular it must not depend on:
//!
//! - parser;
//! - lexer;
//! - semantic analysis;
//! - ZUIR;
//! - quantum IR;
//! - quantum hardware;
//! - optimization;
//! - scheduling;
//! - runtime;
//! - backend providers;
//! - external quantum languages.
//!
//! # Integration contract
//!
//! `node.rs` should store a `NodeKind` and expose it through a method such as:
//!
//! ```text
//! fn node_kind(&self) -> NodeKind
//! ```
//!
//! Visitors and traversal code should dispatch on `NodeKind` without needing
//! to know how the eventual program is executed.
//!
//! Structural validation may validate extension identifiers. Semantic
//! analysis interprets the meaning of a node. ZUIR lowering determines how
//! that meaning is represented downstream.
//!
//! # Determinism
//!
//! `NodeKind` implements deterministic equality, hashing and ordering.
//! Extension kinds compare lexicographically by namespace and then name.
//!
//! # Scalability
//!
//! There is no fixed number of AST nodes, machines, qubits, registers,
//! backends, devices or computational resources represented here.
//!
//! Compiler resource limits, if required for hostile input protection, belong
//! to configurable compiler-limit infrastructure rather than this type.
//!
//! # Compatibility
//!
//! The representation is deliberately independent of the legacy
//! `src/ast/mod.rs` implementation. During AST migration, legacy variants can
//! map to the closest native core kind or to a namespaced extension kind.
//!
//! No unsafe code is used.
//!
//! # Rust
//!
//! Target: Rust 1.97 / Rust 1.97.1.
//!

use core::cmp::Ordering;
use core::fmt;
use core::hash::{Hash, Hasher};

/// The namespace reserved for the native Zamani language.
pub const ZAMANI_CORE_NAMESPACE: &str = "zamani";

/// Maximum length of an extension namespace or name.
///
/// This is a structural safety bound, not a semantic or computational
/// scalability limit.
///
/// The value is deliberately large enough for practical compiler use while
/// preventing pathological allocation/processing from malformed identifiers.
///
/// It does not restrict the number of different node kinds or AST nodes.
const MAX_EXTENSION_IDENTIFIER_BYTES: usize = 16 * 1024;

/// Identifies the source-level kind of an AST node.
///
/// `NodeKind` is intentionally open-ended through [`NodeKind::Extension`].
/// This prevents the core AST from becoming a closed enumeration of every
/// future language or computational-domain construct.
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum NodeKind {
    /// A construct belonging to the native Zamani language.
    Core(CoreNodeKind),

    /// A construct supplied by a registered language extension.
    ///
    /// The namespace and name are opaque to the AST core.
    Extension(ExtensionNodeKind),
}

impl NodeKind {
    /// Creates a native Zamani node kind.
    #[must_use]
    pub const fn core(kind: CoreNodeKind) -> Self {
        Self::Core(kind)
    }

    /// Creates an extension node kind.
    ///
    /// # Errors
    ///
    /// Returns [`NodeKindError`] when either identifier violates the
    /// extension identifier rules.
    pub fn extension(
        namespace: impl Into<String>,
        name: impl Into<String>,
    ) -> Result<Self, NodeKindError> {
        Ok(Self::Extension(ExtensionNodeKind::new(namespace, name)?))
    }

    /// Returns the core node kind when this is a native node.
    #[must_use]
    pub const fn as_core(&self) -> Option<CoreNodeKind> {
        match self {
            Self::Core(kind) => Some(*kind),
            Self::Extension(_) => None,
        }
    }

    /// Returns the extension identifier when this is an extension node.
    #[must_use]
    pub fn as_extension(&self) -> Option<&ExtensionNodeKind> {
        match self {
            Self::Core(_) => None,
            Self::Extension(kind) => Some(kind),
        }
    }

    /// Returns `true` when this is a native Zamani node kind.
    #[must_use]
    pub const fn is_core(&self) -> bool {
        matches!(self, Self::Core(_))
    }

    /// Returns `true` when this is an extension node kind.
    #[must_use]
    pub const fn is_extension(&self) -> bool {
        matches!(self, Self::Extension(_))
    }

    /// Returns the namespace associated with this kind.
    ///
    /// Native Zamani kinds use [`ZAMANI_CORE_NAMESPACE`].
    #[must_use]
    pub fn namespace(&self) -> &str {
        match self {
            Self::Core(_) => ZAMANI_CORE_NAMESPACE,
            Self::Extension(kind) => kind.namespace(),
        }
    }

    /// Returns the stable, human-readable kind name.
    ///
    /// This is intended for diagnostics, debugging and tooling. It is not a
    /// parser token and must not be used as a hardware instruction name.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Core(kind) => kind.name(),
            Self::Extension(_) => "<extension>",
        }
    }

    /// Returns the extension's dynamic name.
    ///
    /// For native kinds this returns the same stable core name as
    /// [`NodeKind::name`].
    #[must_use]
    pub fn qualified_name(&self) -> String {
        match self {
            Self::Core(kind) => {
                let mut result = String::with_capacity(
                    ZAMANI_CORE_NAMESPACE.len() + 1 + kind.name().len(),
                );
                result.push_str(ZAMANI_CORE_NAMESPACE);
                result.push(':');
                result.push_str(kind.name());
                result
            }
            Self::Extension(kind) => kind.qualified_name(),
        }
    }

    /// Returns a stable machine-readable identifier.
    ///
    /// The format is:
    ///
    /// ```text
    /// namespace:name
    /// ```
    ///
    /// No target, device, qubit count or backend information is included.
    #[must_use]
    pub fn stable_id(&self) -> String {
        self.qualified_name()
    }
}

impl fmt::Debug for NodeKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Core(kind) => formatter
                .debug_tuple("NodeKind::Core")
                .field(kind)
                .finish(),

            Self::Extension(kind) => formatter
                .debug_tuple("NodeKind::Extension")
                .field(kind)
                .finish(),
        }
    }
}

impl fmt::Display for NodeKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Core(kind) => {
                write!(formatter, "{ZAMANI_CORE_NAMESPACE}:{}", kind.name())
            }
            Self::Extension(kind) => kind.fmt(formatter),
        }
    }
}

/// Native Zamani AST node categories.
///
/// This enum intentionally classifies **source-language structure**, not
/// implementation details.
///
/// The variants are deliberately coarse enough that adding a new computational
/// technology does not require changing this enum.
///
/// Technology-specific syntax belongs in [`ExtensionNodeKind`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[non_exhaustive]
pub enum CoreNodeKind {
    /// Complete source/program root.
    Program,

    /// Module or namespace declaration.
    Module,

    /// Top-level or nested declaration.
    Declaration,

    /// Function or callable declaration.
    Function,

    /// Variable binding.
    Variable,

    /// Constant declaration.
    Constant,

    /// Type declaration.
    TypeDeclaration,

    /// Struct declaration.
    Struct,

    /// Enum declaration.
    Enum,

    /// Trait/protocol declaration.
    Trait,

    /// Implementation declaration.
    Implementation,

    /// Interface declaration.
    Interface,

    /// Class declaration.
    Class,

    /// Type alias.
    TypeAlias,

    /// External declaration.
    ExternalDeclaration,

    /// Import declaration.
    Import,

    /// Use/reference declaration.
    Use,

    /// Generic parameter or generic declaration component.
    GenericParameter,

    /// Generic argument.
    GenericArgument,

    /// Function/call parameter.
    Parameter,

    /// Type expression.
    Type,

    /// Pattern.
    Pattern,

    /// Wildcard pattern.
    WildcardPattern,

    /// Identifier.
    Identifier,

    /// Literal value.
    Literal,

    /// Unary/prefix operator expression.
    UnaryExpression,

    /// Binary/infix operator expression.
    BinaryExpression,

    /// Assignment expression.
    Assignment,

    /// Function or operation call.
    CallExpression,

    /// Member/property access.
    MemberAccess,

    /// Indexing operation.
    IndexExpression,

    /// Type cast.
    CastExpression,

    /// Type ascription.
    TypeAscription,

    /// Conditional expression.
    ConditionalExpression,

    /// Block expression.
    BlockExpression,

    /// Match expression.
    MatchExpression,

    /// Loop expression.
    LoopExpression,

    /// Lambda/closure expression.
    LambdaExpression,

    /// Await expression.
    AwaitExpression,

    /// Async expression.
    AsyncExpression,

    /// Spawn/concurrency expression.
    SpawnExpression,

    /// Object/resource construction.
    ConstructionExpression,

    /// Range expression.
    RangeExpression,

    /// Array/list expression.
    ArrayExpression,

    /// Tuple expression.
    TupleExpression,

    /// Struct/object literal expression.
    StructExpression,

    /// Expression used as a statement.
    ExpressionStatement,

    /// Variable-binding statement.
    LetStatement,

    /// Constant-binding statement.
    ConstStatement,

    /// Return statement.
    ReturnStatement,

    /// Break statement.
    BreakStatement,

    /// Continue statement.
    ContinueStatement,

    /// While loop statement.
    WhileStatement,

    /// For loop statement.
    ForStatement,

    /// Match statement.
    MatchStatement,

    /// General statement.
    Statement,

    /// Attribute attached to a source construct.
    Attribute,

    /// Annotation attached to a source construct.
    Annotation,

    /// Compiler/source directive.
    Directive,

    /// Effect declaration or annotation.
    Effect,

    /// Capability declaration or requirement.
    Capability,

    /// Generic source-level resource declaration/reference.
    Resource,

    /// Domain declaration/reference.
    Domain,

    /// Macro declaration/invocation.
    Macro,

    /// Documentation node.
    Documentation,

    /// Error-recovery node created by the parser.
    Error,

    /// Explicitly preserved syntax that has not yet been interpreted.
    Opaque,
}

impl CoreNodeKind {
    /// Returns the stable native name of this node kind.
    ///
    /// These names are part of the AST's tooling/debugging contract.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Program => "program",
            Self::Module => "module",
            Self::Declaration => "declaration",
            Self::Function => "function",
            Self::Variable => "variable",
            Self::Constant => "constant",
            Self::TypeDeclaration => "type-declaration",
            Self::Struct => "struct",
            Self::Enum => "enum",
            Self::Trait => "trait",
            Self::Implementation => "implementation",
            Self::Interface => "interface",
            Self::Class => "class",
            Self::TypeAlias => "type-alias",
            Self::ExternalDeclaration => "external-declaration",
            Self::Import => "import",
            Self::Use => "use",
            Self::GenericParameter => "generic-parameter",
            Self::GenericArgument => "generic-argument",
            Self::Parameter => "parameter",
            Self::Type => "type",
            Self::Pattern => "pattern",
            Self::WildcardPattern => "wildcard-pattern",
            Self::Identifier => "identifier",
            Self::Literal => "literal",
            Self::UnaryExpression => "unary-expression",
            Self::BinaryExpression => "binary-expression",
            Self::Assignment => "assignment",
            Self::CallExpression => "call-expression",
            Self::MemberAccess => "member-access",
            Self::IndexExpression => "index-expression",
            Self::CastExpression => "cast-expression",
            Self::TypeAscription => "type-ascription",
            Self::ConditionalExpression => "conditional-expression",
            Self::BlockExpression => "block-expression",
            Self::MatchExpression => "match-expression",
            Self::LoopExpression => "loop-expression",
            Self::LambdaExpression => "lambda-expression",
            Self::AwaitExpression => "await-expression",
            Self::AsyncExpression => "async-expression",
            Self::SpawnExpression => "spawn-expression",
            Self::ConstructionExpression => "construction-expression",
            Self::RangeExpression => "range-expression",
            Self::ArrayExpression => "array-expression",
            Self::TupleExpression => "tuple-expression",
            Self::StructExpression => "struct-expression",
            Self::ExpressionStatement => "expression-statement",
            Self::LetStatement => "let-statement",
            Self::ConstStatement => "const-statement",
            Self::ReturnStatement => "return-statement",
            Self::BreakStatement => "break-statement",
            Self::ContinueStatement => "continue-statement",
            Self::WhileStatement => "while-statement",
            Self::ForStatement => "for-statement",
            Self::MatchStatement => "match-statement",
            Self::Statement => "statement",
            Self::Attribute => "attribute",
            Self::Annotation => "annotation",
            Self::Directive => "directive",
            Self::Effect => "effect",
            Self::Capability => "capability",
            Self::Resource => "resource",
            Self::Domain => "domain",
            Self::Macro => "macro",
            Self::Documentation => "documentation",
            Self::Error => "error",
            Self::Opaque => "opaque",
        }
    }

    /// Returns the broad source-language category.
    ///
    /// This is useful for diagnostics and tooling without requiring callers
    /// to maintain another classification table.
    #[must_use]
    pub const fn category(self) -> CoreNodeCategory {
        match self {
            Self::Program | Self::Module => CoreNodeCategory::Program,

            Self::Declaration
            | Self::Function
            | Self::Variable
            | Self::Constant
            | Self::TypeDeclaration
            | Self::Struct
            | Self::Enum
            | Self::Trait
            | Self::Implementation
            | Self::Interface
            | Self::Class
            | Self::TypeAlias
            | Self::ExternalDeclaration
            | Self::Import
            | Self::Use
            | Self::GenericParameter
            | Self::GenericArgument
            | Self::Parameter => CoreNodeCategory::Declaration,

            Self::Type => CoreNodeCategory::Type,

            Self::Pattern | Self::WildcardPattern => CoreNodeCategory::Pattern,

            Self::Identifier | Self::Literal => CoreNodeCategory::PrimaryExpression,

            Self::UnaryExpression
            | Self::BinaryExpression
            | Self::Assignment
            | Self::CallExpression
            | Self::MemberAccess
            | Self::IndexExpression
            | Self::CastExpression
            | Self::TypeAscription
            | Self::ConditionalExpression
            | Self::BlockExpression
            | Self::MatchExpression
            | Self::LoopExpression
            | Self::LambdaExpression
            | Self::AwaitExpression
            | Self::AsyncExpression
            | Self::SpawnExpression
            | Self::ConstructionExpression
            | Self::RangeExpression
            | Self::ArrayExpression
            | Self::TupleExpression
            | Self::StructExpression => CoreNodeCategory::Expression,

            Self::ExpressionStatement
            | Self::LetStatement
            | Self::ConstStatement
            | Self::ReturnStatement
            | Self::BreakStatement
            | Self::ContinueStatement
            | Self::WhileStatement
            | Self::ForStatement
            | Self::MatchStatement
            | Self::Statement => CoreNodeCategory::Statement,

            Self::Attribute
            | Self::Annotation
            | Self::Directive
            | Self::Documentation => CoreNodeCategory::Metadata,

            Self::Effect => CoreNodeCategory::Effect,

            Self::Capability => CoreNodeCategory::Capability,

            Self::Resource => CoreNodeCategory::Resource,

            Self::Domain => CoreNodeCategory::Domain,

            Self::Macro => CoreNodeCategory::Macro,

            Self::Error => CoreNodeCategory::Recovery,

            Self::Opaque => CoreNodeCategory::Opaque,
        }
    }

    /// Returns the native namespace.
    #[must_use]
    pub const fn namespace(self) -> &'static str {
        ZAMANI_CORE_NAMESPACE
    }
}

/// Broad classification of native AST nodes.
///
/// This classification is intentionally smaller than [`CoreNodeKind`] so
/// tooling can perform broad dispatch without depending on individual syntax
/// variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[non_exhaustive]
pub enum CoreNodeCategory {
    /// Program/module structure.
    Program,

    /// Declarations and declaration components.
    Declaration,

    /// Type syntax.
    Type,

    /// Pattern syntax.
    Pattern,

    /// Primary expressions such as literals and identifiers.
    PrimaryExpression,

    /// General expressions.
    Expression,

    /// Statements.
    Statement,

    /// Attributes, annotations, directives and documentation.
    Metadata,

    /// Effect information.
    Effect,

    /// Capability information.
    Capability,

    /// Resource information.
    Resource,

    /// Computational-domain information.
    Domain,

    /// Macro syntax.
    Macro,

    /// Parser recovery.
    Recovery,

    /// Explicit opaque syntax.
    Opaque,
}

impl CoreNodeCategory {
    /// Returns the stable category name.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Program => "program",
            Self::Declaration => "declaration",
            Self::Type => "type",
            Self::Pattern => "pattern",
            Self::PrimaryExpression => "primary-expression",
            Self::Expression => "expression",
            Self::Statement => "statement",
            Self::Metadata => "metadata",
            Self::Effect => "effect",
            Self::Capability => "capability",
            Self::Resource => "resource",
            Self::Domain => "domain",
            Self::Macro => "macro",
            Self::Recovery => "recovery",
            Self::Opaque => "opaque",
        }
    }
}

impl fmt::Display for CoreNodeKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.name())
    }
}

/// An extensible, namespaced AST node kind.
///
/// Extension identifiers are intentionally opaque to the AST core. This is
/// what allows new computational domains, language features and syntax
/// families to be introduced without modifying `CoreNodeKind`.
///
/// The namespace is part of the identity. Therefore:
///
/// ```text
/// quantum:operation
/// hdl:operation
/// ai:operation
/// vendor.example:operation
/// ```
///
/// are distinct node kinds even when their names are identical.
#[derive(Clone, Eq)]
pub struct ExtensionNodeKind {
    namespace: String,
    name: String,
}

impl ExtensionNodeKind {
    /// Creates an extension node kind after validating both identifiers.
    ///
    /// # Errors
    ///
    /// Returns [`NodeKindError`] if either identifier is empty, malformed,
    /// contains forbidden separator characters, or exceeds the structural
    /// safety bound.
    pub fn new(
        namespace: impl Into<String>,
        name: impl Into<String>,
    ) -> Result<Self, NodeKindError> {
        let namespace = namespace.into();
        let name = name.into();

        validate_extension_identifier(
            ExtensionIdentifierKind::Namespace,
            &namespace,
        )?;

        validate_extension_identifier(
            ExtensionIdentifierKind::Name,
            &name,
        )?;

        Ok(Self { namespace, name })
    }

    /// Returns the extension namespace.
    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Returns the extension name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns `namespace:name`.
    #[must_use]
    pub fn qualified_name(&self) -> String {
        let mut result =
            String::with_capacity(self.namespace.len() + 1 + self.name.len());

        result.push_str(&self.namespace);
        result.push(':');
        result.push_str(&self.name);

        result
    }
}

impl PartialEq for ExtensionNodeKind {
    fn eq(&self, other: &Self) -> bool {
        self.namespace == other.namespace && self.name == other.name
    }
}

impl Hash for ExtensionNodeKind {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.namespace.hash(state);
        self.name.hash(state);
    }
}

impl PartialOrd for ExtensionNodeKind {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ExtensionNodeKind {
    fn cmp(&self, other: &Self) -> Ordering {
        self.namespace
            .cmp(&other.namespace)
            .then_with(|| self.name.cmp(&other.name))
    }
}

impl fmt::Debug for ExtensionNodeKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ExtensionNodeKind")
            .field("namespace", &self.namespace)
            .field("name", &self.name)
            .finish()
    }
}

impl fmt::Display for ExtensionNodeKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}:{}", self.namespace, self.name)
    }
}

/// Identifies which extension identifier is being validated.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ExtensionIdentifierKind {
    Namespace,
    Name,
}

impl ExtensionIdentifierKind {
    const fn label(self) -> &'static str {
        match self {
            Self::Namespace => "namespace",
            Self::Name => "name",
        }
    }
}

/// Validates a namespaced extension identifier.
///
/// The AST deliberately uses a conservative identifier grammar so serialized
/// names remain unambiguous and deterministic:
///
/// ```text
/// first character: ASCII letter or `_`
/// remaining characters: ASCII letters, digits, `_`, `-`, `.`, `/`
/// ```
///
/// `:` is forbidden because it is the namespace/name separator.
///
/// Unicode source identifiers remain supported by the *language AST*; this
/// restriction applies only to extension identity strings used as stable
/// compiler/tooling identifiers.
fn validate_extension_identifier(
    kind: ExtensionIdentifierKind,
    value: &str,
) -> Result<(), NodeKindError> {
    if value.is_empty() {
        return Err(NodeKindError::EmptyIdentifier {
            kind: kind.label(),
        });
    }

    if value.len() > MAX_EXTENSION_IDENTIFIER_BYTES {
        return Err(NodeKindError::IdentifierTooLong {
            kind: kind.label(),
            max_bytes: MAX_EXTENSION_IDENTIFIER_BYTES,
        });
    }

    let mut characters = value.chars();

    let first = characters
        .next()
        .expect("value was checked for emptiness");

    if !(first.is_ascii_alphabetic() || first == '_') {
        return Err(NodeKindError::InvalidFirstCharacter {
            kind: kind.label(),
            character: first,
        });
    }

    for character in characters {
        let allowed = character.is_ascii_alphanumeric()
            || matches!(character, '_' | '-' | '.' | '/');

        if !allowed {
            return Err(NodeKindError::InvalidCharacter {
                kind: kind.label(),
                character,
            });
        }
    }

    Ok(())
}

/// Errors produced while constructing an extensible node kind.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeKindError {
    /// An extension namespace/name was empty.
    EmptyIdentifier {
        /// `"namespace"` or `"name"`.
        kind: &'static str,
    },

    /// The identifier exceeded the structural safety limit.
    IdentifierTooLong {
        /// `"namespace"` or `"name"`.
        kind: &'static str,

        /// Maximum permitted byte length.
        max_bytes: usize,
    },

    /// The first identifier character was invalid.
    InvalidFirstCharacter {
        /// `"namespace"` or `"name"`.
        kind: &'static str,

        /// Invalid character.
        character: char,
    },

    /// A non-leading identifier character was invalid.
    InvalidCharacter {
        /// `"namespace"` or `"name"`.
        kind: &'static str,

        /// Invalid character.
        character: char,
    },
}

impl fmt::Display for NodeKindError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyIdentifier { kind } => {
                write!(formatter, "AST node-kind {kind} cannot be empty")
            }

            Self::IdentifierTooLong {
                kind,
                max_bytes,
            } => write!(
                formatter,
                "AST node-kind {kind} exceeds the maximum structural \
                 length of {max_bytes} bytes"
            ),

            Self::InvalidFirstCharacter { kind, character } => write!(
                formatter,
                "AST node-kind {kind} starts with invalid character {character:?}"
            ),

            Self::InvalidCharacter { kind, character } => write!(
                formatter,
                "AST node-kind {kind} contains invalid character {character:?}"
            ),
        }
    }
}

impl std::error::Error for NodeKindError {}

/// Convenient conversion from a native node kind.
impl From<CoreNodeKind> for NodeKind {
    fn from(value: CoreNodeKind) -> Self {
        Self::Core(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn core_kind_has_stable_namespace_and_name() {
        let kind = NodeKind::core(CoreNodeKind::Program);

        assert!(kind.is_core());
        assert!(!kind.is_extension());
        assert_eq!(kind.namespace(), "zamani");
        assert_eq!(kind.name(), "program");
        assert_eq!(kind.stable_id(), "zamani:program");
        assert_eq!(kind.to_string(), "zamani:program");
    }

    #[test]
    fn core_kind_round_trips_through_as_core() {
        let kind = NodeKind::core(CoreNodeKind::Function);

        assert_eq!(
            kind.as_core(),
            Some(CoreNodeKind::Function)
        );
        assert!(kind.as_extension().is_none());
    }

    #[test]
    fn extension_kind_is_open_ended() {
        let kind =
            NodeKind::extension("quantum", "operation")
                .expect("valid extension kind");

        assert!(kind.is_extension());
        assert!(!kind.is_core());
        assert_eq!(kind.namespace(), "quantum");
        assert_eq!(
            kind.as_extension()
                .expect("extension kind")
                .name(),
            "operation"
        );
        assert_eq!(kind.to_string(), "quantum:operation");
        assert_eq!(kind.stable_id(), "quantum:operation");
    }

    #[test]
    fn different_namespaces_are_different_kinds() {
        let quantum =
            NodeKind::extension("quantum", "operation")
                .expect("valid extension kind");

        let hdl =
            NodeKind::extension("hdl", "operation")
                .expect("valid extension kind");

        assert_ne!(quantum, hdl);
    }

    #[test]
    fn extension_equality_is_deterministic() {
        let first =
            NodeKind::extension("quantum", "operation")
                .expect("valid extension kind");

        let second =
            NodeKind::extension("quantum", "operation")
                .expect("valid extension kind");

        assert_eq!(first, second);
        assert_eq!(first.qualified_name(), second.qualified_name());
    }

    #[test]
    fn extension_order_is_deterministic() {
        let a =
            NodeKind::extension("a", "operation")
                .expect("valid extension kind");

        let b =
            NodeKind::extension("b", "operation")
                .expect("valid extension kind");

        assert!(a < b);
    }

    #[test]
    fn extension_namespace_cannot_contain_separator() {
        let result = NodeKind::extension("quantum:vendor", "operation");

        assert!(matches!(
            result,
            Err(NodeKindError::InvalidCharacter {
                kind: "namespace",
                character: ':',
            })
        ));
    }

    #[test]
    fn extension_name_cannot_contain_separator() {
        let result = NodeKind::extension("quantum", "operation:custom");

        assert!(matches!(
            result,
            Err(NodeKindError::InvalidCharacter {
                kind: "name",
                character: ':',
            })
        ));
    }

    #[test]
    fn extension_namespace_cannot_be_empty() {
        let result = NodeKind::extension("", "operation");

        assert!(matches!(
            result,
            Err(NodeKindError::EmptyIdentifier {
                kind: "namespace",
            })
        ));
    }

    #[test]
    fn extension_name_cannot_be_empty() {
        let result = NodeKind::extension("quantum", "");

        assert!(matches!(
            result,
            Err(NodeKindError::EmptyIdentifier {
                kind: "name",
            })
        ));
    }

    #[test]
    fn extension_identifier_requires_valid_first_character() {
        let result = NodeKind::extension("123quantum", "operation");

        assert!(matches!(
            result,
            Err(NodeKindError::InvalidFirstCharacter {
                kind: "namespace",
                character: '1',
            })
        ));
    }

    #[test]
    fn extension_identifier_accepts_stable_namespace_characters() {
        let result =
            NodeKind::extension(
                "vendor.example/quantum-v1",
                "operation.custom",
            );

        assert!(result.is_ok());
    }

    #[test]
    fn extension_identifier_rejects_whitespace() {
        let result = NodeKind::extension("quantum domain", "operation");

        assert!(matches!(
            result,
            Err(NodeKindError::InvalidCharacter {
                kind: "namespace",
                character: ' ',
            })
        ));
    }

    #[test]
    fn all_core_names_are_non_empty() {
        let kinds = [
            CoreNodeKind::Program,
            CoreNodeKind::Module,
            CoreNodeKind::Declaration,
            CoreNodeKind::Function,
            CoreNodeKind::Variable,
            CoreNodeKind::Constant,
            CoreNodeKind::TypeDeclaration,
            CoreNodeKind::Struct,
            CoreNodeKind::Enum,
            CoreNodeKind::Trait,
            CoreNodeKind::Implementation,
            CoreNodeKind::Interface,
            CoreNodeKind::Class,
            CoreNodeKind::TypeAlias,
            CoreNodeKind::ExternalDeclaration,
            CoreNodeKind::Import,
            CoreNodeKind::Use,
            CoreNodeKind::GenericParameter,
            CoreNodeKind::GenericArgument,
            CoreNodeKind::Parameter,
            CoreNodeKind::Type,
            CoreNodeKind::Pattern,
            CoreNodeKind::WildcardPattern,
            CoreNodeKind::Identifier,
            CoreNodeKind::Literal,
            CoreNodeKind::UnaryExpression,
            CoreNodeKind::BinaryExpression,
            CoreNodeKind::Assignment,
            CoreNodeKind::CallExpression,
            CoreNodeKind::MemberAccess,
            CoreNodeKind::IndexExpression,
            CoreNodeKind::CastExpression,
            CoreNodeKind::TypeAscription,
            CoreNodeKind::ConditionalExpression,
            CoreNodeKind::BlockExpression,
            CoreNodeKind::MatchExpression,
            CoreNodeKind::LoopExpression,
            CoreNodeKind::LambdaExpression,
            CoreNodeKind::AwaitExpression,
            CoreNodeKind::AsyncExpression,
            CoreNodeKind::SpawnExpression,
            CoreNodeKind::ConstructionExpression,
            CoreNodeKind::RangeExpression,
            CoreNodeKind::ArrayExpression,
            CoreNodeKind::TupleExpression,
            CoreNodeKind::StructExpression,
            CoreNodeKind::ExpressionStatement,
            CoreNodeKind::LetStatement,
            CoreNodeKind::ConstStatement,
            CoreNodeKind::ReturnStatement,
            CoreNodeKind::BreakStatement,
            CoreNodeKind::ContinueStatement,
            CoreNodeKind::WhileStatement,
            CoreNodeKind::ForStatement,
            CoreNodeKind::MatchStatement,
            CoreNodeKind::Statement,
            CoreNodeKind::Attribute,
            CoreNodeKind::Annotation,
            CoreNodeKind::Directive,
            CoreNodeKind::Effect,
            CoreNodeKind::Capability,
            CoreNodeKind::Resource,
            CoreNodeKind::Domain,
            CoreNodeKind::Macro,
            CoreNodeKind::Documentation,
            CoreNodeKind::Error,
            CoreNodeKind::Opaque,
        ];

        for kind in kinds {
            assert!(!kind.name().is_empty());
            assert_eq!(kind.namespace(), ZAMANI_CORE_NAMESPACE);
        }
    }

    #[test]
    fn categories_are_stable() {
        assert_eq!(
            CoreNodeKind::Program.category(),
            CoreNodeCategory::Program
        );

        assert_eq!(
            CoreNodeKind::Function.category(),
            CoreNodeCategory::Declaration
        );

        assert_eq!(
            CoreNodeKind::Literal.category(),
            CoreNodeCategory::PrimaryExpression
        );

        assert_eq!(
            CoreNodeKind::CallExpression.category(),
            CoreNodeCategory::Expression
        );

        assert_eq!(
            CoreNodeKind::ReturnStatement.category(),
            CoreNodeCategory::Statement
        );

        assert_eq!(
            CoreNodeKind::Resource.category(),
            CoreNodeCategory::Resource
        );

        assert_eq!(
            CoreNodeKind::Domain.category(),
            CoreNodeCategory::Domain
        );
    }

    #[test]
    fn no_hardware_specific_kind_is_required() {
        let generic_quantum_operation =
            NodeKind::extension("quantum", "operation")
                .expect("valid extension kind");

        let generic_resource =
            NodeKind::core(CoreNodeKind::Resource);

        assert_eq!(
            generic_quantum_operation.stable_id(),
            "quantum:operation"
        );

        assert_eq!(
            generic_resource.stable_id(),
            "zamani:resource"
        );
    }

    #[test]
    fn error_display_is_actionable() {
        let error = NodeKindError::EmptyIdentifier {
            kind: "namespace",
        };

        assert_eq!(
            error.to_string(),
            "AST node-kind namespace cannot be empty"
        );
    }
}