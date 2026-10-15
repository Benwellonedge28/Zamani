#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR symbol model
//!
//! Canonical path: `src/zuir/values/symbols.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Represent resolved symbol identities and declaration metadata in ZUIR.
//! A symbol identifies a declaration; it is not the declaration's source AST,
//! a variable's storage location, or a hardware resource.
//!
//! ## OWNS
//!
//! - Symbol metadata and symbol-reference data structures.
//! - Symbol kind, visibility, linkage, definition state, and origin.
//! - Local representation validation and configurable validation budgets.
//! - Stable local validation errors.
//!
//! ## DOES_NOT_OWN
//!
//! - ID allocation or identity-scope management (`ids.rs`).
//! - Qualified-name representation (`schema/types.rs`).
//! - Name lookup, scope traversal, imports resolution, or overload selection.
//! - Semantic type definitions or type compatibility (`types/*`).
//! - Variable initialization or mutability (`values/variables.rs`).
//! - AST declarations, parsing, source spans, or diagnostics rendering.
//! - SSA use-def verification, ownership analysis, or physical allocation.
//!
//! ## INTEGRATION
//!
//! - `ids.rs` owns `SymbolId` and `ModuleId`.
//! - `schema/types.rs` owns `QualifiedName`.
//! - `types/type_def.rs` owns `TypeRef`.
//! - Semantic analysis resolves source names and creates symbol records.
//! - Module and dialect registries own name-to-symbol indexes and uniqueness.
//! - `values/variables.rs` refers to symbols by `SymbolId`.
//! - `ir/module.rs` and future symbol verification consume this model.
//! - Serialization validates decoded records before they are trusted.
//!
//! ## SCALABILITY AND POCO-REAF
//!
//! This module imposes no universal limit on symbols, name length, namespace
//! depth, domains, modules, devices, qubits, or execution resources.
//! Collections are dynamically sized. Optional validation budgets are
//! operational safeguards, not language-semantic limits.
//!
//! `SymbolId` is an identity token, not a stable cross-build fingerprint.
//! Identity uniqueness across independently allocated scopes must be enforced
//! by the owning program or registry.
//!
//! ## SEMANTIC BOUNDARY
//!
//! A locally valid symbol is not necessarily a valid symbol in a complete
//! program. The owning semantic model must establish that references resolve,
//! names are unique in their scopes, linkage and visibility are legal, types
//! are compatible, and declaration states are consistent.
//!
//! Unknown extension metadata remains descriptive data. It must not gain
//! executable semantics unless a registered, versioned contract defines them.
//!
//! ## FREEZE CRITERIA
//!
//! - Reuses canonical IDs, names, and type references.
//! - Does not implement a competing resolver or symbol table.
//! - Validates deserialized values explicitly.
//! - Has checked validation counters and configurable budgets.
//! - Preserves deterministic attribute ordering.
//! - Contains no hardware-specific assumptions or unsafe Rust.

use crate::zuir::ids::{ModuleId, SymbolId};
use crate::zuir::schema::types::QualifiedName;
use crate::zuir::types::type_def::TypeRef;

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

// =============================================================================
// Symbol kind
// =============================================================================

/// The semantic category of a declaration identified by a symbol.
///
/// This is a declaration-category vocabulary, not a closed catalogue of
/// computational operations. Domain-specific operations and types belong to
/// their registered dialects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum SymbolKind {
    Variable,
    Constant,
    Parameter,
    Function,
    Type,
    Module,
    Label,
    Import,
    Extension,
}

// =============================================================================
// Visibility and linkage
// =============================================================================

/// Source-level visibility of a declaration.
///
/// Enforcement belongs to semantic analysis and module verification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum SymbolVisibility {
    /// Visible only in the declaration's owning scope, subject to language rules.
    #[default]
    Private,

    /// Eligible for visibility outside the owning scope, subject to module
    /// exports, linkage, and access-control rules.
    Public,
}

/// Linkage policy for connecting declarations across module boundaries.
///
/// Linkage is independent of visibility: a public declaration is not
/// automatically exported, and external linkage does not grant source-level
/// access to every caller.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum SymbolLinkage {
    /// Definition belongs to its local linkage scope.
    #[default]
    Internal,

    /// Definition or declaration participates in external linkage.
    External,

    /// Declaration is intended to be exported by its owning module.
    Exported,

    /// The owning linker or module system defines weak-linkage behavior.
    Weak,
}

// =============================================================================
// Definition state and origin
// =============================================================================

/// State of a symbol declaration in the representation being built.
///
/// This records declaration status only. It does not prove that a definition
/// is complete, linked, executable, or semantically valid.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum SymbolState {
    /// A declaration exists, but its definition may not be available yet.
    #[default]
    Declared,

    /// A definition is present in the owning compilation representation.
    Defined,

    /// The symbol refers to a declaration imported from another module.
    Imported,

    /// The definition is supplied by an external integration or runtime.
    ExternallyDefined,
}

/// Where a symbol originated.
///
/// Qualified names here are descriptive origin references; resolving them
/// belongs to the module/import system.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
#[non_exhaustive]
pub enum SymbolOrigin {
    /// Declared in the current semantic input.
    #[default]
    Local,

    /// Imported from the named module or namespace.
    Imported { source: QualifiedName },

    /// Created by a compiler transformation or generated declaration.
    Generated { producer: QualifiedName },

    /// Supplied by an external environment.
    External,
}

impl SymbolOrigin {
    /// Validate origin metadata without attempting to resolve it.
    pub fn validate(&self) -> Result<(), SymbolValidationError> {
        match self {
            Self::Local | Self::External => Ok(()),
            Self::Imported { source } => source
                .validate()
                .map_err(|_| SymbolValidationError::InvalidOriginName),
            Self::Generated { producer } => producer
                .validate()
                .map_err(|_| SymbolValidationError::InvalidOriginName),
        }
    }

    fn text_parts(&self) -> impl Iterator<Item = &str> {
        let value = match self {
            Self::Local | Self::External => None,
            Self::Imported { source } => Some(source),
            Self::Generated { producer } => Some(producer),
        };

        value
            .into_iter()
            .flat_map(|name| name.segments().iter().map(String::as_str))
    }

    fn name_depth(&self) -> usize {
        match self {
            Self::Local | Self::External => 0,
            Self::Imported { source } => source.depth(),
            Self::Generated { producer } => producer.depth(),
        }
    }
}

// =============================================================================
// Symbol reference
// =============================================================================

/// A typed reference to a symbol identity.
///
/// `expected_kind` is an optional use-site expectation. It does not resolve
/// the symbol or establish that the referenced symbol exists. The owning
/// verifier must compare the expectation with the resolved symbol's kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SymbolRef {
    /// Canonical identity of the referenced symbol.
    pub id: SymbolId,

    /// Optional declaration category required by this use site.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_kind: Option<SymbolKind>,
}

impl SymbolRef {
    /// Construct an unconstrained symbol reference.
    #[must_use]
    pub const fn new(id: SymbolId) -> Self {
        Self {
            id,
            expected_kind: None,
        }
    }

    /// Construct a reference with an expected declaration category.
    #[must_use]
    pub const fn expecting(id: SymbolId, expected_kind: SymbolKind) -> Self {
        Self {
            id,
            expected_kind: Some(expected_kind),
        }
    }

    /// Return the referenced symbol identity.
    #[must_use]
    pub const fn id(self) -> SymbolId {
        self.id
    }

    /// Return the expected category, if one was specified.
    #[must_use]
    pub const fn expected_kind(self) -> Option<SymbolKind> {
        self.expected_kind
    }
}

// =============================================================================
// Symbol
// =============================================================================

/// Canonical metadata for a resolved symbol.
///
/// `name` is the declaration's qualified semantic name. `id` is its identity
/// within the allocation scope selected by the enclosing program or registry.
/// `ty` is optional because not every symbol category is a typed value, and
/// some declaration states may precede complete type resolution.
///
/// Fields are public for IR inspection and serialization interoperability.
/// Call [`Symbol::validate`] after construction and after deserialization.
/// Complete-program checks remain the responsibility of semantic analysis
/// and the ZUIR verifier.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Symbol {
    /// Canonical symbol identity.
    pub id: SymbolId,

    /// Qualified semantic name of the declaration.
    pub name: QualifiedName,

    /// Declaration category.
    pub kind: SymbolKind,

    /// Source-level visibility.
    #[serde(default)]
    pub visibility: SymbolVisibility,

    /// Cross-module linkage policy.
    #[serde(default)]
    pub linkage: SymbolLinkage,

    /// Declaration state in this representation.
    #[serde(default)]
    pub state: SymbolState,

    /// Origin of this declaration.
    #[serde(default)]
    pub origin: SymbolOrigin,

    /// Owning module, when the enclosing representation has one.
    ///
    /// The referenced module's existence and ownership rules are verified
    /// by the program/module layer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner_module: Option<ModuleId>,

    /// Enclosing symbol, such as a parent declaration or namespace symbol.
    ///
    /// Parent relationships must be checked for existence, valid nesting,
    /// and cycles by the owning symbol registry or verifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<SymbolId>,

    /// Semantic type reference, where applicable.
    ///
    /// This reference is not resolved or type-checked by this module.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ty: Option<TypeRef>,

    /// Namespaced descriptive metadata.
    ///
    /// The ordered map gives deterministic iteration. These strings do not
    /// alter semantics unless a separately versioned schema defines their
    /// meaning.
    #[serde(default)]
    pub attributes: BTreeMap<String, String>,
}

impl Symbol {
    /// Construct a symbol with conservative default policies.
    ///
    /// This constructor does not allocate an ID, resolve a name, validate
    /// uniqueness, or prove that the declaration is valid in its context.
    #[must_use]
    pub fn new(id: SymbolId, name: QualifiedName, kind: SymbolKind) -> Self {
        Self {
            id,
            name,
            kind,
            visibility: SymbolVisibility::Private,
            linkage: SymbolLinkage::Internal,
            state: SymbolState::Declared,
            origin: SymbolOrigin::Local,
            owner_module: None,
            parent: None,
            ty: None,
            attributes: BTreeMap::new(),
        }
    }

    /// Set the source-level visibility.
    #[must_use]
    pub fn with_visibility(mut self, visibility: SymbolVisibility) -> Self {
        self.visibility = visibility;
        self
    }

    /// Set the linkage policy.
    #[must_use]
    pub fn with_linkage(mut self, linkage: SymbolLinkage) -> Self {
        self.linkage = linkage;
        self
    }

    /// Set the declaration state.
    #[must_use]
    pub fn with_state(mut self, state: SymbolState) -> Self {
        self.state = state;
        self
    }

    /// Set the declaration origin.
    #[must_use]
    pub fn with_origin(mut self, origin: SymbolOrigin) -> Self {
        self.origin = origin;
        self
    }

    /// Associate this symbol with an owning module.
    #[must_use]
    pub fn with_owner_module(mut self, module: ModuleId) -> Self {
        self.owner_module = Some(module);
        self
    }

    /// Associate this symbol with an enclosing symbol.
    #[must_use]
    pub fn with_parent(mut self, parent: SymbolId) -> Self {
        self.parent = Some(parent);
        self
    }

    /// Attach a semantic type reference.
    #[must_use]
    pub fn with_type(mut self, ty: TypeRef) -> Self {
        self.ty = Some(ty);
        self
    }

    /// Borrow the symbol's identity.
    #[must_use]
    pub const fn id(&self) -> SymbolId {
        self.id
    }

    /// Borrow the qualified name.
    #[must_use]
    pub const fn name(&self) -> &QualifiedName {
        &self.name
    }

    /// Return the declaration category.
    #[must_use]
    pub const fn kind(&self) -> SymbolKind {
        self.kind
    }

    /// Return the source-level visibility.
    #[must_use]
    pub const fn visibility(&self) -> SymbolVisibility {
        self.visibility
    }

    /// Return the linkage policy.
    #[must_use]
    pub const fn linkage(&self) -> SymbolLinkage {
        self.linkage
    }

    /// Return the declaration state.
    #[must_use]
    pub const fn state(&self) -> SymbolState {
        self.state
    }

    /// Borrow the optional semantic type reference.
    #[must_use]
    pub const fn ty(&self) -> Option<&TypeRef> {
        self.ty.as_ref()
    }

    /// Insert descriptive metadata.
    ///
    /// A blank key is rejected. Duplicate keys replace their prior value and
    /// return it, matching ordinary map insertion behavior.
    pub fn insert_attribute(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Option<String>, SymbolValidationError> {
        let key = key.into();

        if key.trim().is_empty() {
            return Err(SymbolValidationError::EmptyAttributeKey);
        }

        Ok(self.attributes.insert(key, value.into()))
    }

    /// Validate local representation invariants without imposing an
    /// additional configured budget.
    pub fn validate(&self) -> Result<(), SymbolValidationError> {
        self.validate_with(&SymbolValidationBudget::unlimited())
    }

    /// Validate local representation invariants under an explicit budget.
    ///
    /// This method validates name/origin structures, parent self-reference,
    /// attribute keys, and the requested budget. It does not resolve names,
    /// inspect parent chains, or validate the optional semantic type.
    pub fn validate_with(
        &self,
        budget: &SymbolValidationBudget,
    ) -> Result<(), SymbolValidationError> {
        self.name
            .validate()
            .map_err(|_| SymbolValidationError::InvalidQualifiedName)?;

        self.origin.validate()?;

        if self.parent == Some(self.id) {
            return Err(SymbolValidationError::SelfParent);
        }

        let mut counter = ValidationCounter::new(budget);

        counter.visit_name(&self.name)?;
        counter.visit_name_depth(self.origin.name_depth())?;
        counter.add_text(self.origin.text_parts())?;

        for (key, value) in &self.attributes {
            if key.trim().is_empty() {
                return Err(SymbolValidationError::EmptyAttributeKey);
            }

            counter.add_text([key.as_str(), value.as_str()])?;
        }

        Ok(())
    }
}

// =============================================================================
// Validation budgets
// =============================================================================

/// Optional operational limits for validating one symbol record.
///
/// `None` means this budget imposes no explicit limit for that dimension.
/// It does not imply unlimited memory or execution time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SymbolValidationBudget {
    /// Maximum total number of qualified-name components visited, including
    /// the symbol's own name and any qualified origin name.
    pub max_name_segments: Option<usize>,

    /// Maximum combined UTF-8 byte length of names and attribute strings.
    pub max_text_bytes: Option<usize>,
}

impl SymbolValidationBudget {
    /// Construct a budget with no additional explicit limits.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            max_name_segments: None,
            max_text_bytes: None,
        }
    }

    /// Construct a budget with explicit optional limits.
    #[must_use]
    pub const fn new(
        max_name_segments: Option<usize>,
        max_text_bytes: Option<usize>,
    ) -> Self {
        Self {
            max_name_segments,
            max_text_bytes,
        }
    }
}

// =============================================================================
// Validation implementation
// =============================================================================

struct ValidationCounter<'a> {
    budget: &'a SymbolValidationBudget,
    name_segments: usize,
    text_bytes: usize,
}

impl<'a> ValidationCounter<'a> {
    fn new(budget: &'a SymbolValidationBudget) -> Self {
        Self {
            budget,
            name_segments: 0,
            text_bytes: 0,
        }
    }

    fn visit_name(&mut self, name: &QualifiedName) -> Result<(), SymbolValidationError> {
        self.visit_name_depth(name.depth())?;
        self.add_text(name.segments().iter().map(String::as_str))
    }

    fn visit_name_depth(&mut self, depth: usize) -> Result<(), SymbolValidationError> {
        self.name_segments = self
            .name_segments
            .checked_add(depth)
            .ok_or(SymbolValidationError::CounterOverflow)?;

        if self
            .budget
            .max_name_segments
            .is_some_and(|limit| self.name_segments > limit)
        {
            return Err(SymbolValidationError::NameBudgetExceeded);
        }

        Ok(())
    }

    fn add_text<'b>(
        &mut self,
        values: impl IntoIterator<Item = &'b str>,
    ) -> Result<(), SymbolValidationError> {
        for value in values {
            self.text_bytes = self
                .text_bytes
                .checked_add(value.len())
                .ok_or(SymbolValidationError::CounterOverflow)?;

            if self
                .budget
                .max_text_bytes
                .is_some_and(|limit| self.text_bytes > limit)
            {
                return Err(SymbolValidationError::TextBudgetExceeded);
            }
        }

        Ok(())
    }
}

// =============================================================================
// Errors
// =============================================================================

/// A deterministic local symbol-representation validation error.
///
/// Errors are independent of diagnostics rendering, source parser internals,
/// symbol resolution, and target backend implementation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum SymbolValidationError {
    /// The qualified symbol name is invalid.
    InvalidQualifiedName,

    /// A qualified name stored in origin metadata is invalid.
    InvalidOriginName,

    /// A symbol refers to itself as its immediate parent.
    SelfParent,

    /// An attribute key is empty or whitespace-only.
    EmptyAttributeKey,

    /// The validation counter overflowed.
    CounterOverflow,

    /// The configured qualified-name segment budget was exceeded.
    NameBudgetExceeded,

    /// The configured text-byte budget was exceeded.
    TextBudgetExceeded,
}

impl fmt::Display for SymbolValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidQualifiedName => {
                f.write_str("symbol has an invalid qualified name")
            }
            Self::InvalidOriginName => {
                f.write_str("symbol origin has an invalid qualified name")
            }
            Self::SelfParent => {
                f.write_str("a symbol cannot be its own immediate parent")
            }
            Self::EmptyAttributeKey => {
                f.write_str("symbol attribute key must not be empty")
            }
            Self::CounterOverflow => {
                f.write_str("symbol validation counter overflowed")
            }
            Self::NameBudgetExceeded => {
                f.write_str("symbol qualified-name segment budget exceeded")
            }
            Self::TextBudgetExceeded => {
                f.write_str("symbol text-byte budget exceeded")
            }
        }
    }
}

impl Error for SymbolValidationError {}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn symbol_id(value: u128) -> SymbolId {
        SymbolId::new(value).expect("test symbol ID must be nonzero")
    }

    fn qualified_name(value: &str) -> QualifiedName {
        QualifiedName::parse(value).expect("test qualified name must be valid")
    }

    #[test]
    fn creates_a_symbol_with_explicit_defaults() {
        let symbol = Symbol::new(
            symbol_id(1),
            qualified_name("app::calculate"),
            SymbolKind::Function,
        );

        assert_eq!(symbol.id(), symbol_id(1));
        assert_eq!(symbol.name().canonical_string(), "app::calculate");
        assert_eq!(symbol.kind(), SymbolKind::Function);
        assert_eq!(symbol.visibility(), SymbolVisibility::Private);
        assert_eq!(symbol.linkage(), SymbolLinkage::Internal);
        assert_eq!(symbol.state(), SymbolState::Declared);
        assert_eq!(symbol.validate(), Ok(()));
    }

    #[test]
    fn supports_external_and_generated_origins() {
        let imported = SymbolOrigin::Imported {
            source: qualified_name("library::math"),
        };
        let generated = SymbolOrigin::Generated {
            producer: qualified_name("compiler::inliner"),
        };

        assert_eq!(imported.validate(), Ok(()));
        assert_eq!(generated.validate(), Ok(()));
    }

    #[test]
    fn symbol_reference_can_state_an_expected_kind() {
        let reference = SymbolRef::expecting(symbol_id(7), SymbolKind::Function);

        assert_eq!(reference.id(), symbol_id(7));
        assert_eq!(reference.expected_kind(), Some(SymbolKind::Function));
    }

    #[test]
    fn rejects_self_parent_relationship() {
        let id = symbol_id(5);
        let symbol = Symbol::new(id, qualified_name("app::value"), SymbolKind::Variable)
            .with_parent(id);

        assert_eq!(symbol.validate(), Err(SymbolValidationError::SelfParent));
    }

    #[test]
    fn rejects_blank_attribute_keys() {
        let mut symbol =
            Symbol::new(symbol_id(2), qualified_name("app::value"), SymbolKind::Variable);

        assert_eq!(
            symbol.insert_attribute("   ", "metadata"),
            Err(SymbolValidationError::EmptyAttributeKey)
        );
    }

    #[test]
    fn replaces_duplicate_attribute_and_returns_previous_value() {
        let mut symbol =
            Symbol::new(symbol_id(3), qualified_name("app::value"), SymbolKind::Variable);

        assert_eq!(
            symbol.insert_attribute("docs::purpose", "first"),
            Ok(None)
        );
        assert_eq!(
            symbol.insert_attribute("docs::purpose", "second"),
            Ok(Some("first".to_owned()))
        );
        assert_eq!(
            symbol.attributes.get("docs::purpose").map(String::as_str),
            Some("second")
        );
    }

    #[test]
    fn validation_budget_rejects_excessive_name_depth() {
        let symbol = Symbol::new(
            symbol_id(4),
            qualified_name("app::math::numeric::calculate"),
            SymbolKind::Function,
        );
        let budget = SymbolValidationBudget::new(Some(2), None);

        assert_eq!(
            symbol.validate_with(&budget),
            Err(SymbolValidationError::NameBudgetExceeded)
        );
    }

    #[test]
    fn validation_budget_rejects_excessive_text() {
        let symbol = Symbol::new(
            symbol_id(6),
            qualified_name("app::value"),
            SymbolKind::Variable,
        );
        let budget = SymbolValidationBudget::new(None, Some(3));

        assert_eq!(
            symbol.validate_with(&budget),
            Err(SymbolValidationError::TextBudgetExceeded)
        );
    }

    #[test]
    fn allows_large_names_when_no_explicit_budget_is_configured() {
        let segments = std::iter::once("root".to_owned())
            .chain((0..256).map(|index| format!("namespace_{index}")));
        let name = QualifiedName::new(segments).expect("qualified name should be valid");
        let symbol = Symbol::new(symbol_id(8), name, SymbolKind::Type);

        assert_eq!(symbol.validate(), Ok(()));
    }

    #[test]
    fn visibility_and_linkage_are_independent_properties() {
        let symbol = Symbol::new(
            symbol_id(9),
            qualified_name("library::api"),
            SymbolKind::Function,
        )
        .with_visibility(SymbolVisibility::Public)
        .with_linkage(SymbolLinkage::External)
        .with_state(SymbolState::ExternallyDefined);

        assert_eq!(symbol.visibility(), SymbolVisibility::Public);
        assert_eq!(symbol.linkage(), SymbolLinkage::External);
        assert_eq!(symbol.state(), SymbolState::ExternallyDefined);
        assert_eq!(symbol.validate(), Ok(()));
    }
}