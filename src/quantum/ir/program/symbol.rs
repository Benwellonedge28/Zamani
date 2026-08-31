//! Zamani Quantum IR — Universal Symbol System
//!
//! Production-grade, hardware-independent symbol identity, declaration,
//! namespace, import/export, alias and lookup infrastructure for the
//! canonical Zamani Quantum IR.
//!
//! # Architectural role
//!
//! `program::symbol` owns the semantic symbol layer used by QuantumProgram,
//! QuantumModule, functions, operations, regions, parameters, resources,
//! capabilities, types, constants, qubits and extension-defined entities.
//!
//! It owns:
//!
//! - stable symbol identity;
//! - symbol names;
//! - qualified names;
//! - namespaces;
//! - symbol kinds;
//! - visibility;
//! - declarations and definitions;
//! - aliases;
//! - imports;
//! - exports;
//! - external symbols;
//! - overload groups;
//! - deterministic symbol-table storage;
//! - symbol lookup;
//! - symbol-table validation;
//! - symbol-table snapshots;
//! - transactional bulk insertion;
//! - collision detection;
//! - namespace-aware resolution.
//!
//! It does NOT own:
//!
//! - operation semantics;
//! - gate semantics;
//! - qubit semantics;
//! - hardware allocation;
//! - routing;
//! - scheduling;
//! - optimization;
//! - pulse synthesis;
//! - calibration;
//! - execution;
//! - simulation;
//! - QEC decoding;
//! - frontend parsing;
//! - canonical serialization;
//! - cryptographic hashing.
//!
//! Those responsibilities belong to their respective IR/compiler layers.
//!
//! # Architectural position
//!
//! ```text
//! Zamani source
//!      │
//!      ▼
//! frontend
//!      │
//!      ▼
//! QuantumProgram
//!      │
//!      ▼
//! QuantumModule
//!      │
//!      ▼
//! SymbolTable
//!      │
//!      ├── names
//!      ├── namespaces
//!      ├── declarations
//!      ├── definitions
//!      ├── aliases
//!      ├── imports
//!      └── exports
//!      │
//!      ▼
//! canonical IR objects
//!      │
//!      ├── operations
//!      ├── regions
//!      ├── functions
//!      ├── parameters
//!      ├── qubits
//!      ├── resources
//!      └── extensions
//! ```
//!
//! # Universal-program principle
//!
//! The symbol system contains no architectural machine-size assumption.
//!
//! There is deliberately no:
//!
//! ```text
//! MAX_SYMBOLS
//! MAX_NAMESPACES
//! MAX_QUBITS
//! MAX_FUNCTIONS
//! MAX_OPERATIONS
//! ```
//!
//! A symbol table may contain one symbol or an arbitrarily large finite
//! collection subject only to:
//!
//! - host addressable memory;
//! - explicit compiler/security policies;
//! - serialization limits;
//! - caller-provided resource budgets.
//!
//! "Infinite hardware" is therefore never encoded as an artificial constant.
//!
//! # Canonical identity boundary
//!
//! Canonical semantic object identities are imported from
//! `quantum::ir::core::identity`.
//!
//! Logical/physical qubit identity is exclusively owned by:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! ```
//!
//! This file therefore imports `QubitId` from `quantum::ir::qubit` and never
//! defines another qubit identity.
//!
//! # Symbol identity
//!
//! A symbol is not the same thing as the object it names.
//!
//! For example:
//!
//! ```text
//! SymbolId(42)
//!     │
//!     └── names → OperationId(9001)
//! ```
//!
//! A symbol can therefore be renamed, aliased, imported or exported without
//! changing the identity of the underlying semantic object.
//!
//! # Determinism
//!
//! Symbol storage uses `BTreeMap` and `BTreeSet` rather than hash-map iteration.
//!
//! This guarantees deterministic ordering for:
//!
//! - lookup diagnostics;
//! - validation;
//! - symbol snapshots;
//! - serialization consumers;
//! - compiler reproducibility;
//! - distributed compilation;
//! - canonical hashing performed by higher layers.
//!
//! # Transactional mutation
//!
//! Bulk symbol insertion is transactional:
//!
//! ```text
//! validate every requested change
//!          │
//!          ├── failure → no state change
//!          │
//!          └── success → apply all changes
//! ```
//!
//! This prevents partially mutated symbol tables.
//!
//! # Name semantics
//!
//! Names are Unicode strings and are not restricted to ASCII.
//!
//! The symbol layer deliberately does not impose a language-specific lexical
//! grammar because source-language lexical validation belongs to the frontend.
//!
//! It does, however, reject empty names and names containing NUL.
//!
//! # Qualified names
//!
//! A qualified symbol name consists of namespace components followed by a
//! terminal symbol name.
//!
//! Example:
//!
//! ```text
//! zamani.quantum.linalg.qft
//! ```
//!
//! Components are stored structurally rather than as one opaque string so that
//! namespace-aware lookup remains deterministic.
//!
//! # Overloads
//!
//! Multiple declarations may share the same textual name when they belong to
//! different overload signatures.
//!
//! The table therefore distinguishes:
//!
//! ```text
//! name
//! signature
//! symbol
//! ```
//!
//! A plain lookup is intentionally ambiguous when multiple overloads exist.
//!
//! Callers must use signature-aware lookup in that case.
//!
//! # Visibility
//!
//! Symbol visibility is semantic:
//!
//! - Private;
//! - Internal;
//! - Public;
//! - External.
//!
//! Hardware visibility is unrelated and does not belong here.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no external dependencies;
//! - no `unsafe`.
//!
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler-enforced.
//!
//! # Integration contract
//!
//! `core::identity`
//!     Supplies canonical semantic object IDs.
//!
//! `qubit`
//!     Supplies canonical `QubitId`.
//!
//! `module`
//!     Owns the compilation-unit container and may embed/use a symbol table.
//!
//! `program`
//!     Owns the complete program containing modules.
//!
//! `operation`
//!     Provides operation identities referenced by operation symbols.
//!
//! `region`
//!     Provides region identities referenced by region symbols.
//!
//! `parameter`
//!     Provides parameter identities referenced by parameter symbols.
//!
//! `resource`
//!     Provides resource identities referenced by resource symbols.
//!
//! `capability`
//!     Provides capability identities referenced by capability symbols.
//!
//! `extension`
//!     Provides extension identities referenced by extension symbols.
//!
//! `validation`
//!     Performs whole-IR validation after this module's local validation.
//!
//! `serialization`
//!     Serializes symbol information using the stable public API.
//!
//! `hash`
//!     Computes canonical content fingerprints without placing hashing logic
//!     in this file.
//!
//! `provenance`
//!     Tracks transformations involving symbol declarations and definitions.
//!
//! `frontend`
//!     Creates symbols from source-language declarations.
//!
//! `optimization`
//!     Must preserve symbol identity and update references through its own
//!     transformation contracts.
//!
//! `routing` / `scheduling` / `hardware`
//!     Must not redefine symbol semantics.
//!
//! # Important ownership rule
//!
//! This file owns the relationship:
//!
//! ```text
//! name → symbol → referenced semantic object
//! ```
//!
//! It does not own the referenced object itself.
//!
//! -----------------------------------------------------------------------------
//! No hardware algorithms belong in this file.
//! -----------------------------------------------------------------------------
//!
//! -----------------------------------------------------------------------------
//! Example
//! -----------------------------------------------------------------------------
//!
//! ```rust
//! use super::symbol::*;
//!
//! let mut symbols = SymbolTable::new();
//!
//! let symbol = Symbol::new(
//!     SymbolId::new(1),
//!     "qft",
//!     SymbolKind::Function,
//!     SymbolVisibility::Public,
//!     SymbolTarget::Function(FunctionId::new(10)),
//! );
//!
//! symbols.insert(symbol).expect("valid symbol");
//!
//! let found = symbols.lookup("qft").expect("lookup succeeds");
//! assert_eq!(found.id(), SymbolId::new(1));
//! ```
//!

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use super::super::core::identity::{
    CapabilityId,
    ExtensionId,
    FunctionId,
    ModuleId,
    OperationId,
    ParameterId,
    RegionId,
    ResourceId,
};
use super::super::qubit::QubitId;

// =============================================================================
// Public result type
// =============================================================================

/// Result returned by symbol-table operations.
pub type SymbolResult<T> = Result<T, SymbolError>;

// =============================================================================
// Symbol ID
// =============================================================================

/// Stable identity of a symbol-table entry.
///
/// `SymbolId` is deliberately distinct from the identity of the semantic
/// object being named.
///
/// For example:
///
/// ```text
/// SymbolId(7)
/// OperationId(7)
/// ```
///
/// are different semantic identities.
///
/// The symbol ID is stable and independent of:
///
/// - collection position;
/// - insertion order;
/// - filesystem location;
/// - machine architecture;
/// - qubit count;
/// - hardware topology.
///
/// Allocation belongs to the owning program/compiler builder.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
)]
pub struct SymbolId(u64);

impl SymbolId {
    /// Creates an explicit symbol identity.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the underlying stable numeric identity.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }

    /// Returns the next representable identity without overflowing.
    #[must_use]
    pub const fn checked_next(self) -> Option<Self> {
        match self.0.checked_add(1) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }
}

impl From<u64> for SymbolId {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

impl From<SymbolId> for u64 {
    fn from(value: SymbolId) -> Self {
        value.value()
    }
}

impl fmt::Display for SymbolId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "symbol:{}", self.0)
    }
}

// =============================================================================
// Symbol kind
// =============================================================================

/// Semantic category of a symbol.
///
/// This classification is intentionally broader than gate-based quantum
/// computing so the same symbol infrastructure can represent dynamic,
/// pulse-level, analog, logical, distributed and future quantum models.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
)]
pub enum SymbolKind {
    /// Complete quantum/classical function or subroutine.
    Function,

    /// Quantum operation or gate definition.
    Operation,

    /// Structured IR region.
    Region,

    /// Symbolic runtime/compile-time parameter.
    Parameter,

    /// Logical qubit.
    Qubit,

    /// Resource declaration/reference.
    Resource,

    /// Capability requirement/reference.
    Capability,

    /// Extension-defined semantic object.
    Extension,

    /// Type declaration.
    Type,

    /// Named constant/value.
    Value,

    /// Imported external declaration.
    External,

    /// Namespace declaration.
    Namespace,

    /// Module declaration.
    Module,
}

impl fmt::Display for SymbolKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Function => "function",
            Self::Operation => "operation",
            Self::Region => "region",
            Self::Parameter => "parameter",
            Self::Qubit => "qubit",
            Self::Resource => "resource",
            Self::Capability => "capability",
            Self::Extension => "extension",
            Self::Type => "type",
            Self::Value => "value",
            Self::External => "external",
            Self::Namespace => "namespace",
            Self::Module => "module",
        };

        formatter.write_str(value)
    }
}

// =============================================================================
// Visibility
// =============================================================================

/// Semantic visibility of a symbol.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
)]
pub enum SymbolVisibility {
    /// Visible only within its defining namespace/module.
    Private,

    /// Visible within the containing compilation unit.
    Internal,

    /// Publicly exported by the containing module.
    Public,

    /// Declared externally and resolved by another compilation unit/system.
    External,
}

impl Default for SymbolVisibility {
    fn default() -> Self {
        Self::Private
    }
}

impl fmt::Display for SymbolVisibility {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Private => "private",
            Self::Internal => "internal",
            Self::Public => "public",
            Self::External => "external",
        };

        formatter.write_str(value)
    }
}

// =============================================================================
// Declaration state
// =============================================================================

/// Lifecycle state of a symbol declaration.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
)]
pub enum SymbolState {
    /// Declaration is known but has no local definition.
    Declared,

    /// Symbol has a local definition.
    Defined,

    /// Symbol is imported from another module.
    Imported,

    /// Symbol is an external declaration.
    External,

    /// Symbol has been explicitly deprecated.
    Deprecated,
}

impl SymbolState {
    /// Returns true when this state represents a usable declaration.
    #[must_use]
    pub const fn is_resolvable(self) -> bool {
        !matches!(self, Self::Deprecated)
    }

    /// Returns true when the symbol has a local semantic definition.
    #[must_use]
    pub const fn is_defined(self) -> bool {
        matches!(self, Self::Defined)
    }
}

// =============================================================================
// Symbol target
// =============================================================================

/// Canonical semantic object referenced by a symbol.
///
/// The symbol table owns the name; the target object owns the actual semantics.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
)]
pub enum SymbolTarget {
    /// Function definition.
    Function(FunctionId),

    /// Operation definition.
    Operation(OperationId),

    /// Region.
    Region(RegionId),

    /// Parameter.
    Parameter(ParameterId),

    /// Logical qubit.
    Qubit(QubitId),

    /// Resource.
    Resource(ResourceId),

    /// Capability.
    Capability(CapabilityId),

    /// Extension object.
    Extension(ExtensionId),

    /// Module.
    Module(ModuleId),

    /// Namespace-only declaration.
    Namespace,

    /// Type/value/external target without a currently canonical core ID.
    ///
    /// This is intentionally represented by the symbol ID itself rather than
    /// introducing a second object-ID system into this file.
    Symbol,
}

impl SymbolTarget {
    /// Returns the semantic category represented by this target.
    #[must_use]
    pub const fn kind(self) -> SymbolKind {
        match self {
            Self::Function(_) => SymbolKind::Function,
            Self::Operation(_) => SymbolKind::Operation,
            Self::Region(_) => SymbolKind::Region,
            Self::Parameter(_) => SymbolKind::Parameter,
            Self::Qubit(_) => SymbolKind::Qubit,
            Self::Resource(_) => SymbolKind::Resource,
            Self::Capability(_) => SymbolKind::Capability,
            Self::Extension(_) => SymbolKind::Extension,
            Self::Module(_) => SymbolKind::Module,
            Self::Namespace => SymbolKind::Namespace,
            Self::Symbol => SymbolKind::Value,
        }
    }
}

// =============================================================================
// Namespace
// =============================================================================

/// Structured namespace path.
///
/// A namespace is stored as components rather than a single opaque string.
///
/// Example:
///
/// ```text
/// zamani.quantum.algorithms
/// ```
///
/// is stored as:
///
/// ```text
/// [
///     "zamani",
///     "quantum",
///     "algorithms",
/// ]
/// ```
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
)]
pub struct NamespacePath {
    components: Vec<String>,
}

impl NamespacePath {
    /// Creates the root namespace.
    #[must_use]
    pub fn root() -> Self {
        Self {
            components: Vec::new(),
        }
    }

    /// Creates a namespace from components.
    ///
    /// Empty components and NUL-containing components are rejected.
    pub fn new<I, S>(components: I) -> SymbolResult<Self>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let mut result = Vec::new();

        for component in components {
            let component = component.into();

            validate_name_component(&component)?;

            result.push(component);
        }

        Ok(Self {
            components: result,
        })
    }

    /// Returns the number of namespace components.
    #[must_use]
    pub fn len(&self) -> usize {
        self.components.len()
    }

    /// Returns true when this is the root namespace.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.components.is_empty()
    }

    /// Returns namespace components.
    #[must_use]
    pub fn components(&self) -> &[String] {
        &self.components
    }

    /// Returns the final namespace component.
    #[must_use]
    pub fn last(&self) -> Option<&str> {
        self.components.last().map(String::as_str)
    }

    /// Creates a child namespace without modifying the existing path.
    pub fn child<S>(&self, component: S) -> SymbolResult<Self>
    where
        S: Into<String>,
    {
        let component = component.into();

        validate_name_component(&component)?;

        let mut components = self.components.clone();
        components.push(component);

        Ok(Self { components })
    }

    /// Creates a qualified symbol name under this namespace.
    pub fn qualify<S>(&self, symbol: S) -> SymbolResult<QualifiedName>
    where
        S: Into<String>,
    {
        let symbol = symbol.into();

        validate_name_component(&symbol)?;

        let mut components = self.components.clone();
        components.push(symbol);

        QualifiedName::from_components(components)
    }

    /// Returns the canonical dotted representation.
    #[must_use]
    pub fn as_string(&self) -> String {
        self.components.join(".")
    }
}

impl Default for NamespacePath {
    fn default() -> Self {
        Self::root()
    }
}

impl fmt::Display for NamespacePath {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.as_string())
    }
}

// =============================================================================
// Qualified name
// =============================================================================

/// Fully qualified symbol name.
///
/// The final component is always the symbol name. All preceding components
/// constitute the namespace.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
)]
pub struct QualifiedName {
    components: Vec<String>,
}

impl QualifiedName {
    /// Creates a qualified name from validated components.
    pub fn from_components<I, S>(components: I) -> SymbolResult<Self>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let mut result = Vec::new();

        for component in components {
            let component = component.into();

            validate_name_component(&component)?;

            result.push(component);
        }

        if result.is_empty() {
            return Err(SymbolError::EmptyQualifiedName);
        }

        Ok(Self {
            components: result,
        })
    }

    /// Creates a qualified name from a dotted string.
    ///
    /// Dotted namespace syntax is intentionally structural: empty components
    /// are rejected rather than silently normalized.
    pub fn parse<S>(name: S) -> SymbolResult<Self>
    where
        S: Into<String>,
    {
        let name = name.into();

        if name.is_empty() {
            return Err(SymbolError::EmptyQualifiedName);
        }

        let components = name
            .split('.')
            .map(str::to_owned)
            .collect::<Vec<_>>();

        Self::from_components(components)
    }

    /// Returns the root-level symbol name.
    #[must_use]
    pub fn symbol_name(&self) -> &str {
        self.components
            .last()
            .map(String::as_str)
            .unwrap_or("")
    }

    /// Returns the namespace portion.
    #[must_use]
    pub fn namespace(&self) -> NamespacePath {
        if self.components.len() <= 1 {
            return NamespacePath::root();
        }

        NamespacePath {
            components: self.components[..self.components.len() - 1]
                .to_vec(),
        }
    }

    /// Returns all components.
    #[must_use]
    pub fn components(&self) -> &[String] {
        &self.components
    }

    /// Returns the number of components.
    #[must_use]
    pub fn len(&self) -> usize {
        self.components.len()
    }

    /// Returns whether the name has no components.
    ///
    /// This is always false for successfully constructed values but is useful
    /// for generic APIs.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.components.is_empty()
    }

    /// Returns the canonical dotted representation.
    #[must_use]
    pub fn as_string(&self) -> String {
        self.components.join(".")
    }
}

impl fmt::Display for QualifiedName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.as_string())
    }
}

// =============================================================================
// Symbol signature
// =============================================================================

/// Stable semantic overload signature.
///
/// The symbol table does not interpret the signature. It only stores the
/// canonical signature key supplied by the owning semantic layer.
///
/// This keeps overload resolution independent from a particular quantum
/// architecture or type system implementation.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
)]
pub struct SymbolSignature {
    components: Vec<String>,
}

impl SymbolSignature {
    /// Creates an empty signature.
    ///
    /// An empty signature represents a non-overloaded declaration.
    #[must_use]
    pub fn empty() -> Self {
        Self {
            components: Vec::new(),
        }
    }

    /// Creates a signature from canonical components.
    pub fn new<I, S>(components: I) -> SymbolResult<Self>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let mut result = Vec::new();

        for component in components {
            let component = component.into();

            if component.contains('\0') {
                return Err(SymbolError::InvalidSignature);
            }

            result.push(component);
        }

        Ok(Self {
            components: result,
        })
    }

    /// Returns signature components.
    #[must_use]
    pub fn components(&self) -> &[String] {
        &self.components
    }

    /// Returns whether the signature is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.components.is_empty()
    }

    /// Returns a deterministic string representation.
    #[must_use]
    pub fn as_string(&self) -> String {
        if self.components.is_empty() {
            return String::new();
        }

        self.components.join(",")
    }
}

impl Default for SymbolSignature {
    fn default() -> Self {
        Self::empty()
    }
}

impl fmt::Display for SymbolSignature {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.as_string())
    }
}

// =============================================================================
// Symbol
// =============================================================================

/// Complete symbol-table entry.
///
/// A `Symbol` is the stable name-bearing declaration around a canonical IR
/// object. The underlying semantic object remains owned by its respective IR
/// subsystem.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Symbol {
    id: SymbolId,
    name: QualifiedName,
    kind: SymbolKind,
    visibility: SymbolVisibility,
    state: SymbolState,
    target: SymbolTarget,
    signature: SymbolSignature,
    alias_of: Option<SymbolId>,
    defining_module: Option<ModuleId>,
    documentation: Option<String>,
    attributes: BTreeMap<String, String>,
}

impl Symbol {
    /// Creates a symbol with default declaration state and empty signature.
    pub fn new<N>(
        id: SymbolId,
        name: N,
        kind: SymbolKind,
        visibility: SymbolVisibility,
        target: SymbolTarget,
    ) -> SymbolResult<Self>
    where
        N: Into<String>,
    {
        let qualified_name = QualifiedName::parse(name)?;

        Self::from_qualified_name(
            id,
            qualified_name,
            kind,
            visibility,
            SymbolState::Declared,
            target,
        )
    }

    /// Creates a symbol from a structured qualified name.
    pub fn from_qualified_name(
        id: SymbolId,
        name: QualifiedName,
        kind: SymbolKind,
        visibility: SymbolVisibility,
        state: SymbolState,
        target: SymbolTarget,
    ) -> SymbolResult<Self> {
        validate_target_kind(kind, target)?;

        Ok(Self {
            id,
            name,
            kind,
            visibility,
            state,
            target,
            signature: SymbolSignature::empty(),
            alias_of: None,
            defining_module: None,
            documentation: None,
            attributes: BTreeMap::new(),
        })
    }

    /// Returns the symbol identity.
    #[must_use]
    pub const fn id(&self) -> SymbolId {
        self.id
    }

    /// Returns the qualified name.
    #[must_use]
    pub fn name(&self) -> &QualifiedName {
        &self.name
    }

    /// Returns the terminal symbol name.
    #[must_use]
    pub fn short_name(&self) -> &str {
        self.name.symbol_name()
    }

    /// Returns the namespace.
    #[must_use]
    pub fn namespace(&self) -> NamespacePath {
        self.name.namespace()
    }

    /// Returns the semantic symbol kind.
    #[must_use]
    pub const fn kind(&self) -> SymbolKind {
        self.kind
    }

    /// Returns symbol visibility.
    #[must_use]
    pub const fn visibility(&self) -> SymbolVisibility {
        self.visibility
    }

    /// Returns declaration state.
    #[must_use]
    pub const fn state(&self) -> SymbolState {
        self.state
    }

    /// Returns the referenced semantic object.
    #[must_use]
    pub const fn target(&self) -> SymbolTarget {
        self.target
    }

    /// Returns the overload signature.
    #[must_use]
    pub fn signature(&self) -> &SymbolSignature {
        &self.signature
    }

    /// Returns the alias target if this symbol is an alias.
    #[must_use]
    pub const fn alias_of(&self) -> Option<SymbolId> {
        self.alias_of
    }

    /// Returns the defining module.
    #[must_use]
    pub const fn defining_module(&self) -> Option<ModuleId> {
        self.defining_module
    }

    /// Returns documentation.
    #[must_use]
    pub fn documentation(&self) -> Option<&str> {
        self.documentation.as_deref()
    }

    /// Returns deterministic symbol attributes.
    #[must_use]
    pub fn attributes(&self) -> &BTreeMap<String, String> {
        &self.attributes
    }

    /// Sets the overload signature.
    #[must_use]
    pub fn with_signature(
        mut self,
        signature: SymbolSignature,
    ) -> Self {
        self.signature = signature;
        self
    }

    /// Sets the declaration state.
    #[must_use]
    pub const fn with_state(
        mut self,
        state: SymbolState,
    ) -> Self {
        self.state = state;
        self
    }

    /// Sets the defining module.
    #[must_use]
    pub const fn with_defining_module(
        mut self,
        module: ModuleId,
    ) -> Self {
        self.defining_module = Some(module);
        self
    }

    /// Sets documentation.
    pub fn with_documentation<S>(
        mut self,
        documentation: S,
    ) -> SymbolResult<Self>
    where
        S: Into<String>,
    {
        let documentation = documentation.into();

        if documentation.contains('\0') {
            return Err(SymbolError::InvalidDocumentation);
        }

        self.documentation = Some(documentation);
        Ok(self)
    }

    /// Adds an attribute.
    pub fn with_attribute<K, V>(
        mut self,
        key: K,
        value: V,
    ) -> SymbolResult<Self>
    where
        K: Into<String>,
        V: Into<String>,
    {
        let key = key.into();
        let value = value.into();

        validate_metadata_key(&key)?;

        if value.contains('\0') {
            return Err(SymbolError::InvalidAttributeValue);
        }

        self.attributes.insert(key, value);
        Ok(self)
    }

    /// Creates an alias of this symbol.
    pub fn alias(
        &self,
        alias_id: SymbolId,
        alias_name: QualifiedName,
    ) -> SymbolResult<Self> {
        Ok(Self {
            id: alias_id,
            name: alias_name,
            kind: self.kind,
            visibility: self.visibility,
            state: SymbolState::Declared,
            target: self.target,
            signature: self.signature.clone(),
            alias_of: Some(self.id),
            defining_module: self.defining_module,
            documentation: self.documentation.clone(),
            attributes: self.attributes.clone(),
        })
    }

    /// Returns whether this symbol is an alias.
    #[must_use]
    pub const fn is_alias(&self) -> bool {
        self.alias_of.is_some()
    }

    /// Returns whether this symbol is public.
    #[must_use]
    pub const fn is_public(&self) -> bool {
        matches!(self.visibility, SymbolVisibility::Public)
    }

    /// Returns whether this symbol is external.
    #[must_use]
    pub const fn is_external(&self) -> bool {
        matches!(
            self.visibility,
            SymbolVisibility::External
        ) || matches!(self.state, SymbolState::External)
    }
}

// =============================================================================
// Symbol reference
// =============================================================================

/// Lightweight reference to a symbol.
///
/// This is intentionally copyable and contains no owned semantic object.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
)]
pub struct SymbolRef {
    id: SymbolId,
}

impl SymbolRef {
    /// Creates a symbol reference.
    #[must_use]
    pub const fn new(id: SymbolId) -> Self {
        Self { id }
    }

    /// Returns the referenced symbol ID.
    #[must_use]
    pub const fn id(self) -> SymbolId {
        self.id
    }
}

impl From<SymbolId> for SymbolRef {
    fn from(value: SymbolId) -> Self {
        Self::new(value)
    }
}

// =============================================================================
// Import
// =============================================================================

/// An imported symbol binding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymbolImport {
    source_module: ModuleId,
    source_name: QualifiedName,
    local_name: QualifiedName,
    visibility: SymbolVisibility,
    alias: Option<SymbolId>,
}

impl SymbolImport {
    /// Creates an import binding.
    pub fn new(
        source_module: ModuleId,
        source_name: QualifiedName,
        local_name: QualifiedName,
    ) -> Self {
        Self {
            source_module,
            source_name,
            local_name,
            visibility: SymbolVisibility::Internal,
            alias: None,
        }
    }

    /// Returns source module.
    #[must_use]
    pub const fn source_module(&self) -> ModuleId {
        self.source_module
    }

    /// Returns source symbol name.
    #[must_use]
    pub fn source_name(&self) -> &QualifiedName {
        &self.source_name
    }

    /// Returns local binding name.
    #[must_use]
    pub fn local_name(&self) -> &QualifiedName {
        &self.local_name
    }

    /// Returns import visibility.
    #[must_use]
    pub const fn visibility(&self) -> SymbolVisibility {
        self.visibility
    }

    /// Returns alias symbol if one exists.
    #[must_use]
    pub const fn alias(&self) -> Option<SymbolId> {
        self.alias
    }

    /// Sets import visibility.
    #[must_use]
    pub const fn with_visibility(
        mut self,
        visibility: SymbolVisibility,
    ) -> Self {
        self.visibility = visibility;
        self
    }

    /// Associates an alias symbol.
    #[must_use]
    pub const fn with_alias(
        mut self,
        alias: SymbolId,
    ) -> Self {
        self.alias = Some(alias);
        self
    }
}

// =============================================================================
// Export
// =============================================================================

/// An exported symbol binding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymbolExport {
    name: QualifiedName,
    symbol: SymbolId,
    visibility: SymbolVisibility,
}

impl SymbolExport {
    /// Creates a public export.
    #[must_use]
    pub fn new(
        name: QualifiedName,
        symbol: SymbolId,
    ) -> Self {
        Self {
            name,
            symbol,
            visibility: SymbolVisibility::Public,
        }
    }

    /// Returns exported name.
    #[must_use]
    pub fn name(&self) -> &QualifiedName {
        &self.name
    }

    /// Returns exported symbol.
    #[must_use]
    pub const fn symbol(&self) -> SymbolId {
        self.symbol
    }

    /// Returns visibility.
    #[must_use]
    pub const fn visibility(&self) -> SymbolVisibility {
        self.visibility
    }

    /// Sets export visibility.
    #[must_use]
    pub const fn with_visibility(
        mut self,
        visibility: SymbolVisibility,
    ) -> Self {
        self.visibility = visibility;
        self
    }
}

// =============================================================================
// Namespace declaration
// =============================================================================

/// Registered namespace declaration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Namespace {
    id: u64,
    path: NamespacePath,
    visibility: SymbolVisibility,
}

impl Namespace {
    /// Creates a namespace declaration.
    pub const fn new(
        id: u64,
        path: NamespacePath,
    ) -> Self {
        Self {
            id,
            path,
            visibility: SymbolVisibility::Private,
        }
    }

    /// Returns namespace identity.
    #[must_use]
    pub const fn id(&self) -> u64 {
        self.id
    }

    /// Returns namespace path.
    #[must_use]
    pub fn path(&self) -> &NamespacePath {
        &self.path
    }

    /// Returns namespace visibility.
    #[must_use]
    pub const fn visibility(&self) -> SymbolVisibility {
        self.visibility
    }

    /// Sets visibility.
    #[must_use]
    pub const fn with_visibility(
        mut self,
        visibility: SymbolVisibility,
    ) -> Self {
        self.visibility = visibility;
        self
    }
}

// =============================================================================
// Symbol table
// =============================================================================

/// Deterministic, namespace-aware symbol table.
///
/// The table is deliberately generic over semantic object ownership: it stores
/// symbol declarations and references, never the actual operation, qubit,
/// region, function or resource objects.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymbolTable {
    symbols: BTreeMap<SymbolId, Symbol>,
    by_name: BTreeMap<QualifiedName, BTreeSet<SymbolId>>,
    namespaces: BTreeMap<NamespacePath, Namespace>,
    imports: BTreeMap<QualifiedName, SymbolImport>,
    exports: BTreeMap<QualifiedName, SymbolExport>,
    aliases: BTreeMap<QualifiedName, SymbolId>,
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self::new()
    }
}

impl SymbolTable {
    /// Creates an empty deterministic symbol table.
    #[must_use]
    pub fn new() -> Self {
        Self {
            symbols: BTreeMap::new(),
            by_name: BTreeMap::new(),
            namespaces: BTreeMap::new(),
            imports: BTreeMap::new(),
            exports: BTreeMap::new(),
            aliases: BTreeMap::new(),
        }
    }

    /// Returns the number of registered symbols.
    #[must_use]
    pub fn len(&self) -> usize {
        self.symbols.len()
    }

    /// Returns true when no symbols are registered.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.symbols.is_empty()
    }

    /// Returns the number of registered namespaces.
    #[must_use]
    pub fn namespace_count(&self) -> usize {
        self.namespaces.len()
    }

    /// Returns the number of imports.
    #[must_use]
    pub fn import_count(&self) -> usize {
        self.imports.len()
    }

    /// Returns the number of exports.
    #[must_use]
    pub fn export_count(&self) -> usize {
        self.exports.len()
    }

    /// Returns a symbol by stable ID.
    #[must_use]
    pub fn get(
        &self,
        id: SymbolId,
    ) -> Option<&Symbol> {
        self.symbols.get(&id)
    }

    /// Returns a mutable symbol by stable ID.
    ///
    /// Direct mutable access is intentionally not exposed. Symbol-table
    /// invariants must remain enforceable through explicit mutation APIs.
    ///
    /// This method therefore does not exist.
    ///
    /// The comment is retained here as an architectural guardrail.
    ///
    /// Use `replace` instead.
    #[must_use]
    pub fn contains(
        &self,
        id: SymbolId,
    ) -> bool {
        self.symbols.contains_key(&id)
    }

    /// Inserts a symbol after validating all local invariants.
    pub fn insert(
        &mut self,
        symbol: Symbol,
    ) -> SymbolResult<SymbolRef> {
        self.validate_symbol_for_insert(&symbol)?;

        let id = symbol.id();
        let name = symbol.name().clone();

        self.symbols.insert(id, symbol);

        self.by_name
            .entry(name)
            .or_default()
            .insert(id);

        Ok(SymbolRef::new(id))
    }

    /// Transactionally inserts multiple symbols.
    ///
    /// If any symbol fails validation, no symbol is inserted.
    pub fn insert_batch<I>(
        &mut self,
        symbols: I,
    ) -> SymbolResult<Vec<SymbolRef>>
    where
        I: IntoIterator<Item = Symbol>,
    {
        let symbols = symbols.into_iter().collect::<Vec<_>>();

        self.validate_batch(&symbols)?;

        let mut refs = Vec::with_capacity(symbols.len());

        for symbol in symbols {
            let id = symbol.id();
            let name = symbol.name().clone();

            self.symbols.insert(id, symbol);

            self.by_name
                .entry(name)
                .or_default()
                .insert(id);

            refs.push(SymbolRef::new(id));
        }

        Ok(refs)
    }

    /// Replaces an existing symbol without changing its stable identity.
    pub fn replace(
        &mut self,
        symbol: Symbol,
    ) -> SymbolResult<()> {
        let id = symbol.id();

        if !self.symbols.contains_key(&id) {
            return Err(SymbolError::UnknownSymbolId { id });
        }

        self.validate_symbol_for_replace(&symbol)?;

        let old_name = self
            .symbols
            .get(&id)
            .map(|existing| existing.name().clone())
            .ok_or(SymbolError::UnknownSymbolId { id })?;

        if old_name != *symbol.name() {
            self.remove_name_index(id, &old_name);

            self.by_name
                .entry(symbol.name().clone())
                .or_default()
                .insert(id);
        }

        self.symbols.insert(id, symbol);

        Ok(())
    }

    /// Removes a symbol by ID.
    ///
    /// Exports and aliases referring to the removed symbol cause an error;
    /// this prevents dangling public references.
    pub fn remove(
        &mut self,
        id: SymbolId,
    ) -> SymbolResult<Symbol> {
        let symbol = self
            .symbols
            .get(&id)
            .ok_or(SymbolError::UnknownSymbolId { id })?;

        if self.exports.values().any(|export| export.symbol() == id) {
            return Err(SymbolError::ExportedSymbolCannotBeRemoved { id });
        }

        if self.aliases.values().any(|alias| *alias == id) {
            return Err(SymbolError::AliasedSymbolCannotBeRemoved { id });
        }

        let name = symbol.name().clone();

        let removed = self
            .symbols
            .remove(&id)
            .ok_or(SymbolError::UnknownSymbolId { id })?;

        self.remove_name_index(id, &name);

        Ok(removed)
    }

    /// Looks up a symbol by exact qualified name.
    ///
    /// If the name is overloaded, this returns `AmbiguousSymbol`.
    pub fn lookup<N>(
        &self,
        name: N,
    ) -> SymbolResult<SymbolRef>
    where
        N: Into<String>,
    {
        let name = QualifiedName::parse(name)?;

        self.lookup_qualified(&name)
    }

    /// Looks up a qualified name.
    pub fn lookup_qualified(
        &self,
        name: &QualifiedName,
    ) -> SymbolResult<SymbolRef> {
        let ids = self
            .by_name
            .get(name)
            .ok_or_else(|| SymbolError::UnknownSymbol {
                name: name.to_string(),
            })?;

        let ids = ids
            .iter()
            .filter_map(|id| {
                self.symbols.get(id).map(|symbol| (id, symbol))
            })
            .filter(|(_, symbol)| symbol.state().is_resolvable())
            .map(|(id, _)| *id)
            .collect::<Vec<_>>();

        match ids.as_slice() {
            [] => Err(SymbolError::UnknownSymbol {
                name: name.to_string(),
            }),
            [id] => Ok(SymbolRef::new(*id)),
            _ => Err(SymbolError::AmbiguousSymbol {
                name: name.to_string(),
                candidates: ids.clone(),
            }),
        }
    }

    /// Looks up a name with an exact overload signature.
    pub fn lookup_with_signature<N>(
        &self,
        name: N,
        signature: &SymbolSignature,
    ) -> SymbolResult<SymbolRef>
    where
        N: Into<String>,
    {
        let name = QualifiedName::parse(name)?;

        self.lookup_qualified_with_signature(
            &name,
            signature,
        )
    }

    /// Looks up a qualified name with an exact signature.
    pub fn lookup_qualified_with_signature(
        &self,
        name: &QualifiedName,
        signature: &SymbolSignature,
    ) -> SymbolResult<SymbolRef> {
        let ids = self
            .by_name
            .get(name)
            .ok_or_else(|| SymbolError::UnknownSymbol {
                name: name.to_string(),
            })?;

        let matches = ids
            .iter()
            .filter_map(|id| {
                self.symbols.get(id).map(|symbol| (id, symbol))
            })
            .filter(|(_, symbol)| {
                symbol.state().is_resolvable()
                    && symbol.signature() == signature
            })
            .map(|(id, _)| *id)
            .collect::<Vec<_>>();

        match matches.as_slice() {
            [] => Err(SymbolError::UnknownSymbolSignature {
                name: name.to_string(),
                signature: signature.clone(),
            }),
            [id] => Ok(SymbolRef::new(*id)),
            _ => Err(SymbolError::AmbiguousSymbol {
                name: name.to_string(),
                candidates: matches.clone(),
            }),
        }
    }

    /// Returns every symbol with an exact qualified name.
    #[must_use]
    pub fn lookup_all(
        &self,
        name: &QualifiedName,
    ) -> Vec<SymbolRef> {
        self.by_name
            .get(name)
            .map(|ids| {
                ids.iter()
                    .filter(|id| {
                        self.symbols
                            .get(id)
                            .map(|symbol| {
                                symbol.state().is_resolvable()
                            })
                            .unwrap_or(false)
                    })
                    .copied()
                    .map(SymbolRef::new)
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Looks up a symbol by terminal name.
    ///
    /// This method is intentionally separate from exact qualified lookup.
    /// Unqualified lookup can be ambiguous across namespaces.
    pub fn lookup_unqualified<N>(
        &self,
        name: N,
    ) -> SymbolResult<SymbolRef>
    where
        N: Into<String>,
    {
        let name = name.into();

        validate_name_component(&name)?;

        let mut matches = Vec::new();

        for symbol in self.symbols.values() {
            if symbol.short_name() == name
                && symbol.state().is_resolvable()
            {
                matches.push(symbol.id());
            }
        }

        match matches.as_slice() {
            [] => Err(SymbolError::UnknownSymbol { name }),
            [id] => Ok(SymbolRef::new(*id)),
            _ => Err(SymbolError::AmbiguousSymbol {
                name,
                candidates: matches,
            }),
        }
    }

    /// Registers a namespace.
    pub fn register_namespace(
        &mut self,
        namespace: Namespace,
    ) -> SymbolResult<()> {
        let path = namespace.path().clone();

        if self.namespaces.contains_key(&path) {
            return Err(SymbolError::DuplicateNamespace {
                name: path.to_string(),
            });
        }

        self.namespaces.insert(path, namespace);

        Ok(())
    }

    /// Returns a namespace by path.
    #[must_use]
    pub fn namespace(
        &self,
        path: &NamespacePath,
    ) -> Option<&Namespace> {
        self.namespaces.get(path)
    }

    /// Adds an import binding.
    pub fn add_import(
        &mut self,
        import: SymbolImport,
    ) -> SymbolResult<()> {
        let local_name = import.local_name().clone();

        if self.imports.contains_key(&local_name) {
            return Err(SymbolError::DuplicateImport {
                name: local_name.to_string(),
            });
        }

        self.imports.insert(local_name, import);

        Ok(())
    }

    /// Adds an export binding.
    pub fn add_export(
        &mut self,
        export: SymbolExport,
    ) -> SymbolResult<()> {
        let name = export.name().clone();

        if self.exports.contains_key(&name) {
            return Err(SymbolError::DuplicateExport {
                name: name.to_string(),
            });
        }

        if !self.symbols.contains_key(&export.symbol()) {
            return Err(SymbolError::UnknownExportSymbol {
                symbol: export.symbol(),
            });
        }

        self.exports.insert(name, export);

        Ok(())
    }

    /// Registers an alias for an existing symbol.
    pub fn add_alias<N>(
        &mut self,
        alias_name: N,
        target: SymbolId,
        alias_id: SymbolId,
    ) -> SymbolResult<SymbolRef>
    where
        N: Into<String>,
    {
        let alias_name = QualifiedName::parse(alias_name)?;

        if self.aliases.contains_key(&alias_name)
            || self.by_name.contains_key(&alias_name)
        {
            return Err(SymbolError::DuplicateSymbol {
                name: alias_name.to_string(),
            });
        }

        let original = self
            .symbols
            .get(&target)
            .ok_or(SymbolError::UnknownSymbolId {
                id: target,
            })?
            .clone();

        let alias = original.alias(alias_id, alias_name.clone())?;

        self.insert(alias)?;

        self.aliases.insert(alias_name, target);

        Ok(SymbolRef::new(alias_id))
    }

    /// Returns the symbol targeted by an alias.
    pub fn resolve_alias(
        &self,
        alias: SymbolId,
    ) -> SymbolResult<SymbolRef> {
        let symbol = self
            .symbols
            .get(&alias)
            .ok_or(SymbolError::UnknownSymbolId { id: alias })?;

        let mut current = symbol.id();
        let mut visited = BTreeSet::new();

        loop {
            if !visited.insert(current) {
                return Err(SymbolError::AliasCycle {
                    symbol: current,
                });
            }

            let current_symbol = self
                .symbols
                .get(&current)
                .ok_or(SymbolError::UnknownSymbolId {
                    id: current,
                })?;

            match current_symbol.alias_of() {
                Some(next) => current = next,
                None => return Ok(SymbolRef::new(current)),
            }
        }
    }

    /// Returns all symbols in deterministic ID order.
    pub fn symbols(&self) -> impl Iterator<Item = &Symbol> {
        self.symbols.values()
    }

    /// Returns all imports in deterministic name order.
    pub fn imports(&self) -> impl Iterator<Item = &SymbolImport> {
        self.imports.values()
    }

    /// Returns all exports in deterministic name order.
    pub fn exports(&self) -> impl Iterator<Item = &SymbolExport> {
        self.exports.values()
    }

    /// Returns all namespaces in deterministic path order.
    pub fn namespaces(&self) -> impl Iterator<Item = &Namespace> {
        self.namespaces.values()
    }

    /// Performs complete symbol-table-local validation.
    pub fn validate(&self) -> SymbolResult<()> {
        for symbol in self.symbols.values() {
            self.validate_symbol_for_insert(symbol)?;
        }

        for export in self.exports.values() {
            if !self.symbols.contains_key(&export.symbol()) {
                return Err(SymbolError::UnknownExportSymbol {
                    symbol: export.symbol(),
                });
            }
        }

        for (alias_name, target) in &self.aliases {
            if !self.symbols.contains_key(target) {
                return Err(SymbolError::UnknownAliasTarget {
                    name: alias_name.to_string(),
                    target: *target,
                });
            }
        }

        for symbol in self.symbols.values() {
            if symbol.is_alias() {
                self.resolve_alias(symbol.id())?;
            }
        }

        Ok(())
    }

    /// Returns a deterministic immutable snapshot.
    #[must_use]
    pub fn snapshot(&self) -> SymbolTableSnapshot {
        SymbolTableSnapshot {
            symbols: self.symbols.clone(),
            namespaces: self.namespaces.clone(),
            imports: self.imports.clone(),
            exports: self.exports.clone(),
            aliases: self.aliases.clone(),
        }
    }

    fn validate_batch(
        &self,
        symbols: &[Symbol],
    ) -> SymbolResult<()> {
        let mut ids = BTreeSet::new();
        let mut names: BTreeMap<
            QualifiedName,
            BTreeSet<SymbolId>,
        > = BTreeMap::new();

        for symbol in symbols {
            self.validate_symbol_for_insert(symbol)?;

            if !ids.insert(symbol.id()) {
                return Err(SymbolError::DuplicateSymbolId {
                    id: symbol.id(),
                });
            }

            names
                .entry(symbol.name().clone())
                .or_default()
                .insert(symbol.id());
        }

        for symbol in symbols {
            if self.symbols.contains_key(&symbol.id()) {
                return Err(SymbolError::DuplicateSymbolId {
                    id: symbol.id(),
                });
            }

            if let Some(existing) = self.by_name.get(symbol.name()) {
                if !existing.is_empty()
                    && symbol.signature().is_empty()
                {
                    return Err(SymbolError::DuplicateSymbol {
                        name: symbol.name().to_string(),
                    });
                }
            }
        }

        for (name, ids_for_name) in names {
            if ids_for_name.len() > 1 {
                let signatures = symbols
                    .iter()
                    .filter(|symbol| {
                        symbol.name() == &name
                    })
                    .map(|symbol| symbol.signature())
                    .collect::<Vec<_>>();

                let mut unique = BTreeSet::new();

                for signature in signatures {
                    if !unique.insert(signature.clone()) {
                        return Err(
                            SymbolError::DuplicateOverload {
                                name: name.to_string(),
                                signature: signature.clone(),
                            },
                        );
                    }
                }
            }
        }

        Ok(())
    }

    fn validate_symbol_for_insert(
        &self,
        symbol: &Symbol,
    ) -> SymbolResult<()> {
        if self.symbols.contains_key(&symbol.id()) {
            return Err(SymbolError::DuplicateSymbolId {
                id: symbol.id(),
            });
        }

        validate_target_kind(
            symbol.kind(),
            symbol.target(),
        )?;

        if symbol.short_name().is_empty() {
            return Err(SymbolError::EmptySymbolName);
        }

        if symbol.is_alias() {
            let target = symbol
                .alias_of()
                .ok_or(SymbolError::InvalidAlias)?;

            if target == symbol.id() {
                return Err(SymbolError::AliasCycle {
                    symbol: symbol.id(),
                });
            }

            if !self.symbols.contains_key(&target) {
                return Err(SymbolError::UnknownAliasTarget {
                    name: symbol.name().to_string(),
                    target,
                });
            }
        }

        if let Some(ids) = self.by_name.get(symbol.name()) {
            for existing_id in ids {
                let existing = self
                    .symbols
                    .get(existing_id)
                    .ok_or(
                        SymbolError::CorruptNameIndex {
                            id: *existing_id,
                        },
                    )?;

                if existing.signature() == symbol.signature() {
                    return Err(
                        SymbolError::DuplicateOverload {
                            name: symbol.name().to_string(),
                            signature: symbol.signature().clone(),
                        },
                    );
                }
            }
        }

        Ok(())
    }

    fn validate_symbol_for_replace(
        &self,
        symbol: &Symbol,
    ) -> SymbolResult<()> {
        validate_target_kind(
            symbol.kind(),
            symbol.target(),
        )?;

        if symbol.short_name().is_empty() {
            return Err(SymbolError::EmptySymbolName);
        }

        if symbol.is_alias() {
            let target = symbol
                .alias_of()
                .ok_or(SymbolError::InvalidAlias)?;

            if target == symbol.id() {
                return Err(SymbolError::AliasCycle {
                    symbol: symbol.id(),
                });
            }

            if !self.symbols.contains_key(&target) {
                return Err(SymbolError::UnknownAliasTarget {
                    name: symbol.name().to_string(),
                    target,
                });
            }
        }

        if let Some(ids) = self.by_name.get(symbol.name()) {
            for existing_id in ids {
                if *existing_id == symbol.id() {
                    continue;
                }

                let existing = self
                    .symbols
                    .get(existing_id)
                    .ok_or(
                        SymbolError::CorruptNameIndex {
                            id: *existing_id,
                        },
                    )?;

                if existing.signature() == symbol.signature() {
                    return Err(
                        SymbolError::DuplicateOverload {
                            name: symbol.name().to_string(),
                            signature: symbol.signature().clone(),
                        },
                    );
                }
            }
        }

        Ok(())
    }

    fn remove_name_index(
        &mut self,
        id: SymbolId,
        name: &QualifiedName,
    ) {
        if let Some(ids) = self.by_name.get_mut(name) {
            ids.remove(&id);

            if ids.is_empty() {
                self.by_name.remove(name);
            }
        }
    }
}

// =============================================================================
// Immutable snapshot
// =============================================================================

/// Immutable, deterministic symbol-table snapshot.
///
/// Useful for:
///
/// - compiler phases;
/// - diagnostics;
/// - incremental compilation;
/// - caching;
/// - parallel read-only analysis;
/// - reproducibility tests.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymbolTableSnapshot {
    symbols: BTreeMap<SymbolId, Symbol>,
    namespaces: BTreeMap<NamespacePath, Namespace>,
    imports: BTreeMap<QualifiedName, SymbolImport>,
    exports: BTreeMap<QualifiedName, SymbolExport>,
    aliases: BTreeMap<QualifiedName, SymbolId>,
}

impl SymbolTableSnapshot {
    /// Returns the number of symbols.
    #[must_use]
    pub fn len(&self) -> usize {
        self.symbols.len()
    }

    /// Returns a symbol by ID.
    #[must_use]
    pub fn get(
        &self,
        id: SymbolId,
    ) -> Option<&Symbol> {
        self.symbols.get(&id)
    }

    /// Returns symbols in deterministic order.
    pub fn symbols(&self) -> impl Iterator<Item = &Symbol> {
        self.symbols.values()
    }

    /// Returns namespaces in deterministic order.
    pub fn namespaces(
        &self,
    ) -> impl Iterator<Item = &Namespace> {
        self.namespaces.values()
    }

    /// Returns imports in deterministic order.
    pub fn imports(
        &self,
    ) -> impl Iterator<Item = &SymbolImport> {
        self.imports.values()
    }

    /// Returns exports in deterministic order.
    pub fn exports(
        &self,
    ) -> impl Iterator<Item = &SymbolExport> {
        self.exports.values()
    }

    /// Returns aliases in deterministic order.
    pub fn aliases(
        &self,
    ) -> impl Iterator<
        Item = (&QualifiedName, &SymbolId),
    > {
        self.aliases.iter()
    }
}

// =============================================================================
// Errors
// =============================================================================

/// Complete symbol-layer error contract.
///
/// Errors are explicit and deterministic. No lookup failure is silently
/// converted into an absent symbol.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SymbolError {
    /// Empty symbol/name component.
    EmptyName,

    /// Empty qualified name.
    EmptyQualifiedName,

    /// Name component contains NUL.
    InvalidName,

    /// Empty symbol name.
    EmptySymbolName,

    /// Invalid overload signature.
    InvalidSignature,

    /// Invalid documentation string.
    InvalidDocumentation,

    /// Invalid attribute key.
    InvalidAttributeKey,

    /// Invalid attribute value.
    InvalidAttributeValue,

    /// Duplicate symbol ID.
    DuplicateSymbolId {
        /// Conflicting identity.
        id: SymbolId,
    },

    /// Duplicate symbol name/signature.
    DuplicateSymbol {
        /// Conflicting name.
        name: String,
    },

    /// Duplicate overload.
    DuplicateOverload {
        /// Conflicting name.
        name: String,

        /// Conflicting signature.
        signature: SymbolSignature,
    },

    /// Unknown symbol by name.
    UnknownSymbol {
        /// Missing name.
        name: String,
    },

    /// Unknown symbol by ID.
    UnknownSymbolId {
        /// Missing identity.
        id: SymbolId,
    },

    /// Ambiguous symbol lookup.
    AmbiguousSymbol {
        /// Requested name.
        name: String,

        /// Candidate symbol identities.
        candidates: Vec<SymbolId>,
    },

    /// Unknown symbol/signature combination.
    UnknownSymbolSignature {
        /// Requested name.
        name: String,

        /// Requested signature.
        signature: SymbolSignature,
    },

    /// Duplicate namespace.
    DuplicateNamespace {
        /// Conflicting namespace.
        name: String,
    },

    /// Duplicate import.
    DuplicateImport {
        /// Conflicting local name.
        name: String,
    },

    /// Duplicate export.
    DuplicateExport {
        /// Conflicting export name.
        name: String,
    },

    /// Export references an unknown symbol.
    UnknownExportSymbol {
        /// Missing symbol.
        symbol: SymbolId,
    },

    /// Exported symbol cannot be removed.
    ExportedSymbolCannotBeRemoved {
        /// Protected symbol.
        id: SymbolId,
    },

    /// Aliased symbol cannot be removed.
    AliasedSymbolCannotBeRemoved {
        /// Protected symbol.
        id: SymbolId,
    },

    /// Alias target does not exist.
    UnknownAliasTarget {
        /// Alias name.
        name: String,

        /// Missing target.
        target: SymbolId,
    },

    /// Alias is structurally invalid.
    InvalidAlias,

    /// Alias cycle detected.
    AliasCycle {
        /// Symbol at which the cycle was detected.
        symbol: SymbolId,
    },

    /// Symbol-name index is internally inconsistent.
    CorruptNameIndex {
        /// Invalid indexed symbol.
        id: SymbolId,
    },

    /// Symbol kind and target do not agree.
    TargetKindMismatch {
        /// Declared symbol kind.
        kind: SymbolKind,

        /// Actual target kind.
        target: SymbolKind,
    },
}

impl fmt::Display for SymbolError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::EmptyName => {
                formatter.write_str("symbol name cannot be empty")
            }

            Self::EmptyQualifiedName => {
                formatter.write_str(
                    "qualified symbol name cannot be empty",
                )
            }

            Self::InvalidName => {
                formatter.write_str(
                    "symbol name contains an invalid NUL character",
                )
            }

            Self::EmptySymbolName => {
                formatter.write_str(
                    "terminal symbol name cannot be empty",
                )
            }

            Self::InvalidSignature => {
                formatter.write_str(
                    "symbol signature contains an invalid NUL character",
                )
            }

            Self::InvalidDocumentation => {
                formatter.write_str(
                    "symbol documentation contains an invalid NUL character",
                )
            }

            Self::InvalidAttributeKey => {
                formatter.write_str(
                    "symbol attribute key cannot be empty or contain NUL",
                )
            }

            Self::InvalidAttributeValue => {
                formatter.write_str(
                    "symbol attribute value contains an invalid NUL character",
                )
            }

            Self::DuplicateSymbolId { id } => {
                write!(formatter, "duplicate symbol identity {id}")
            }

            Self::DuplicateSymbol { name } => {
                write!(formatter, "duplicate symbol `{name}`")
            }

            Self::DuplicateOverload {
                name,
                signature,
            } => {
                write!(
                    formatter,
                    "duplicate overload `{name}` with signature `{signature}`"
                )
            }

            Self::UnknownSymbol { name } => {
                write!(formatter, "unknown symbol `{name}`")
            }

            Self::UnknownSymbolId { id } => {
                write!(formatter, "unknown symbol identity {id}")
            }

            Self::AmbiguousSymbol {
                name,
                candidates,
            } => {
                write!(
                    formatter,
                    "ambiguous symbol `{name}` with {} candidates",
                    candidates.len()
                )
            }

            Self::UnknownSymbolSignature {
                name,
                signature,
            } => {
                write!(
                    formatter,
                    "no symbol `{name}` matches signature `{signature}`"
                )
            }

            Self::DuplicateNamespace { name } => {
                write!(formatter, "duplicate namespace `{name}`")
            }

            Self::DuplicateImport { name } => {
                write!(formatter, "duplicate import `{name}`")
            }

            Self::DuplicateExport { name } => {
                write!(formatter, "duplicate export `{name}`")
            }

            Self::UnknownExportSymbol { symbol } => {
                write!(
                    formatter,
                    "export references unknown symbol {symbol}"
                )
            }

            Self::ExportedSymbolCannotBeRemoved { id } => {
                write!(
                    formatter,
                    "exported symbol {id} cannot be removed"
                )
            }

            Self::AliasedSymbolCannotBeRemoved { id } => {
                write!(
                    formatter,
                    "aliased symbol {id} cannot be removed"
                )
            }

            Self::UnknownAliasTarget { name, target } => {
                write!(
                    formatter,
                    "alias `{name}` references unknown target {target}"
                )
            }

            Self::InvalidAlias => {
                formatter.write_str("invalid symbol alias")
            }

            Self::AliasCycle { symbol } => {
                write!(
                    formatter,
                    "symbol alias cycle detected at {symbol}"
                )
            }

            Self::CorruptNameIndex { id } => {
                write!(
                    formatter,
                    "symbol name index references missing symbol {id}"
                )
            }

            Self::TargetKindMismatch {
                kind,
                target,
            } => {
                write!(
                    formatter,
                    "symbol kind `{kind}` does not match target kind `{target}`"
                )
            }
        }
    }
}

impl std::error::Error for SymbolError {}

// =============================================================================
// Validation helpers
// =============================================================================

fn validate_name_component(
    value: &str,
) -> SymbolResult<()> {
    if value.is_empty() {
        return Err(SymbolError::EmptyName);
    }

    if value.contains('\0') {
        return Err(SymbolError::InvalidName);
    }

    Ok(())
}

fn validate_metadata_key(
    value: &str,
) -> SymbolResult<()> {
    if value.is_empty() || value.contains('\0') {
        return Err(SymbolError::InvalidAttributeKey);
    }

    Ok(())
}

fn validate_target_kind(
    kind: SymbolKind,
    target: SymbolTarget,
) -> SymbolResult<()> {
    let target_kind = target.kind();

    let compatible = match (kind, target_kind) {
        (SymbolKind::Value, SymbolKind::Value) => true,
        (SymbolKind::Type, SymbolKind::Value) => true,
        (SymbolKind::External, _) => true,
        (SymbolKind::Namespace, SymbolKind::Namespace) => true,
        (SymbolKind::Module, SymbolKind::Module) => true,
        (left, right) => left == right,
    };

    if compatible {
        Ok(())
    } else {
        Err(SymbolError::TargetKindMismatch {
            kind,
            target: target_kind,
        })
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn function_symbol(
        id: u64,
        name: &str,
    ) -> Symbol {
        Symbol::new(
            SymbolId::new(id),
            name,
            SymbolKind::Function,
            SymbolVisibility::Private,
            SymbolTarget::Function(
                FunctionId::new(id),
            ),
        )
        .expect("valid test symbol")
    }

    #[test]
    fn symbol_id_is_stable_and_typed() {
        let id = SymbolId::new(42);

        assert_eq!(id.value(), 42);
        assert_eq!(id.to_string(), "symbol:42");
        assert_eq!(id.checked_next(), Some(SymbolId::new(43)));
    }

    #[test]
    fn qualified_name_is_structural() {
        let name =
            QualifiedName::parse("zamani.quantum.qft")
                .expect("valid qualified name");

        assert_eq!(name.symbol_name(), "qft");
        assert_eq!(
            name.namespace().as_string(),
            "zamani.quantum"
        );
        assert_eq!(
            name.as_string(),
            "zamani.quantum.qft"
        );
    }

    #[test]
    fn namespace_can_qualify_symbol() {
        let namespace =
            NamespacePath::new([
                "zamani",
                "quantum",
                "algorithms",
            ])
            .expect("valid namespace");

        let name = namespace
            .qualify("qft")
            .expect("valid symbol name");

        assert_eq!(
            name.to_string(),
            "zamani.quantum.algorithms.qft"
        );
    }

    #[test]
    fn table_insert_and_lookup_are_deterministic() {
        let mut table = SymbolTable::new();

        table
            .insert(function_symbol(1, "zamani.qft"))
            .expect("insert succeeds");

        let found = table
            .lookup("zamani.qft")
            .expect("lookup succeeds");

        assert_eq!(found.id(), SymbolId::new(1));
    }

    #[test]
    fn duplicate_name_is_rejected() {
        let mut table = SymbolTable::new();

        table
            .insert(function_symbol(1, "qft"))
            .expect("first insert succeeds");

        let error = table
            .insert(function_symbol(2, "qft"))
            .expect_err("duplicate must fail");

        assert!(matches!(
            error,
            SymbolError::DuplicateOverload { .. }
        ));
    }

    #[test]
    fn overloaded_symbols_require_signature_lookup() {
        let mut table = SymbolTable::new();

        let first = function_symbol(1, "qft")
            .with_signature(
                SymbolSignature::new(["f64"])
                    .expect("signature"),
            );

        let second = function_symbol(2, "qft")
            .with_signature(
                SymbolSignature::new([
                    "f64",
                    "usize",
                ])
                .expect("signature"),
            );

        table.insert(first).expect("first");
        table.insert(second).expect("second");

        assert!(matches!(
            table.lookup("qft"),
            Err(SymbolError::AmbiguousSymbol { .. })
        ));

        let found = table
            .lookup_with_signature(
                "qft",
                &SymbolSignature::new([
                    "f64",
                    "usize",
                ])
                .expect("signature"),
            )
            .expect("signature lookup");

        assert_eq!(found.id(), SymbolId::new(2));
    }

    #[test]
    fn batch_insert_is_transactional() {
        let mut table = SymbolTable::new();

        table
            .insert(function_symbol(1, "existing"))
            .expect("existing");

        let batch = vec![
            function_symbol(2, "new"),
            function_symbol(3, "existing"),
        ];

        assert!(table.insert_batch(batch).is_err());

        assert!(
            table.lookup("new").is_err(),
            "failed batch must not partially commit"
        );

        assert!(
            table.lookup("existing").is_ok(),
            "existing state must remain intact"
        );
    }

    #[test]
    fn alias_resolution_works() {
        let mut table = SymbolTable::new();

        table
            .insert(function_symbol(1, "qft"))
            .expect("target");

        let alias = table
            .add_alias(
                "quantum.qft",
                SymbolId::new(1),
                SymbolId::new(2),
            )
            .expect("alias");

        assert_eq!(alias.id(), SymbolId::new(2));

        let resolved = table
            .resolve_alias(alias.id())
            .expect("resolve alias");

        assert_eq!(
            resolved.id(),
            SymbolId::new(1)
        );
    }

    #[test]
    fn export_requires_existing_symbol() {
        let mut table = SymbolTable::new();

        let name =
            QualifiedName::parse("qft")
                .expect("valid name");

        let error = table
            .add_export(SymbolExport::new(
                name,
                SymbolId::new(99),
            ))
            .expect_err("missing symbol");

        assert!(matches!(
            error,
            SymbolError::UnknownExportSymbol { .. }
        ));
    }

    #[test]
    fn qubit_target_uses_canonical_qubit_id() {
        let symbol = Symbol::new(
            SymbolId::new(1),
            "q",
            SymbolKind::Qubit,
            SymbolVisibility::Private,
            SymbolTarget::Qubit(
                QubitId::new(7),
            ),
        )
        .expect("valid qubit symbol");

        assert_eq!(
            symbol.kind(),
            SymbolKind::Qubit
        );
    }

    #[test]
    fn symbol_table_validation_succeeds_for_valid_table() {
        let mut table = SymbolTable::new();

        table
            .insert(function_symbol(1, "qft"))
            .expect("insert");

        table
            .add_export(
                SymbolExport::new(
                    QualifiedName::parse("qft")
                        .expect("name"),
                    SymbolId::new(1),
                ),
            )
            .expect("export");

        table.validate().expect("valid table");
    }

    #[test]
    fn root_namespace_is_empty_but_valid() {
        let root = NamespacePath::root();

        assert!(root.is_empty());
        assert_eq!(root.to_string(), "");
    }

    #[test]
    fn aliases_cannot_form_self_cycle() {
        let mut table = SymbolTable::new();

        table
            .insert(function_symbol(1, "qft"))
            .expect("target");

        let alias = table
            .add_alias(
                "qft_alias",
                SymbolId::new(1),
                SymbolId::new(2),
            )
            .expect("alias");

        assert_eq!(
            table
                .resolve_alias(alias.id())
                .expect("resolve")
                .id(),
            SymbolId::new(1)
        );
    }
}