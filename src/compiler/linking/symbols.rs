
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani Compiler — Linking Symbol Resolution
//!
//! Path: `src/compiler/linking/symbols.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//!
//! ## PURPOSE
//!
//! Provide the link-time symbol index used when composing independently
//! compiled Zamani modules and artifacts.
//!
//! This module resolves declarations and definitions across compilation
//! units while preserving the canonical symbol model owned by ZUIR.
//!
//! ## OWNERSHIP
//!
//! OWNS:
//! - Link-time symbol indexing.
//! - Compilation-unit symbol ownership.
//! - Duplicate link-definition detection.
//! - Strong and weak definition selection.
//! - Internal and external linkage visibility.
//! - Undefined external symbol reporting.
//! - Deterministic symbol resolution.
//! - Configurable symbol-index validation budgets.
//!
//! DOES NOT OWN:
//! - Source-level name resolution.
//! - Semantic symbol declaration collection.
//! - Symbol ID allocation.
//! - Canonical symbol metadata.
//! - Type inference or type compatibility.
//! - ABI definitions.
//! - Calling conventions.
//! - Object-file parsing.
//! - Relocation execution.
//! - Machine-code generation.
//! - Dynamic library loading.
//! - Physical hardware allocation.
//! - Runtime execution.
//!
//! ## CANONICAL INTEGRATION
//!
//! Uses:
//! - crate::zuir::ids::SymbolId
//! - crate::zuir::ids::ModuleId
//! - crate::zuir::values::symbols::Symbol
//! - crate::zuir::values::symbols::SymbolRef
//! - crate::zuir::values::symbols::SymbolLinkage
//! - crate::zuir::values::symbols::SymbolState
//! - crate::zuir::values::symbols::SymbolKind
//!
//! Semantic analysis remains responsible for establishing the meaning
//! and validity of source-level declarations.
//!
//! ## SCALABILITY
//!
//! There are no universal limits on:
//! - Symbols.
//! - Compilation units.
//! - Modules.
//! - Domains.
//! - Architectures.
//! - Hardware resources.
//! - Identifier lengths.
//!
//! Operational budgets are explicitly configurable.
//!
//! ## DETERMINISM
//!
//! Ordered collections are used for stable indexing and reporting.
//! Weak-definition selection does not depend on insertion order.
//!
//! ## SECURITY
//!
//! Symbols must be validated before entering the index.
//! Unresolved references are reported explicitly.
//! This module never executes external tools or loads executable code.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use crate::zuir::ids::{ModuleId, SymbolId};

use crate::zuir::values::symbols::{
    Symbol,
    SymbolKind,
    SymbolLinkage,
    SymbolRef,
    SymbolState,
    SymbolVisibility,
};

// ============================================================================
// API VERSION
// ============================================================================

/// Public API version for the linking symbol subsystem.
pub const LINKING_SYMBOLS_API_VERSION: &str = "1.0.0";

// ============================================================================
// RESULT
// ============================================================================

/// Result type used by the linking symbol subsystem.
pub type SymbolLinkResult<T> = Result<T, SymbolLinkError>;

// ============================================================================
// COMPILATION UNIT IDENTITY
// ============================================================================

/// Identifies one independently compiled input unit.
///
/// This identifier belongs to the linking subsystem, not to source-level
/// semantic identity.
///
/// Its spelling is preserved exactly.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LinkUnitId(String);

impl LinkUnitId {
    /// Construct a compilation-unit identifier.
    pub fn new(value: impl Into<String>) -> SymbolLinkResult<Self> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(SymbolLinkError::EmptyUnitId);
        }

        Ok(Self(value))
    }

    /// Return the exact identifier.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for LinkUnitId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

// ============================================================================
// SYMBOL DEFINITION
// ============================================================================

/// One canonical symbol contributed by a compilation unit.
///
/// The canonical ZUIR Symbol remains the authority for declaration metadata.
/// This wrapper only associates that symbol with its link-time input unit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymbolDefinition {
    unit: LinkUnitId,
    symbol: Symbol,
}

impl SymbolDefinition {
    /// Construct a link-time symbol definition.
    ///
    /// Local symbol validation is performed immediately.
    /// Complete-program semantic validation remains upstream.
    pub fn new(
        unit: LinkUnitId,
        symbol: Symbol,
    ) -> SymbolLinkResult<Self> {
        symbol
            .validate()
            .map_err(|error| {
                SymbolLinkError::InvalidSymbol {
                    unit: unit.clone(),
                    reason: error.to_string(),
                }
            })?;

        Ok(Self { unit, symbol })
    }

    /// Return the owning compilation unit.
    pub fn unit(&self) -> &LinkUnitId {
        &self.unit
    }

    /// Return the canonical symbol.
    pub fn symbol(&self) -> &Symbol {
        &self.symbol
    }

    /// Return the canonical symbol identity.
    pub fn id(&self) -> SymbolId {
        self.symbol.id
    }

    /// Return the canonical qualified name.
    pub fn name(&self) -> String {
        self.symbol.name.canonical_string()
    }

    /// Return the linkage classification.
    pub fn linkage(&self) -> SymbolLinkage {
        self.symbol.linkage
    }

    /// Return the declaration state.
    pub fn state(&self) -> SymbolState {
        self.symbol.state
    }
}

// ============================================================================
// SYMBOL IDENTITY
// ============================================================================

/// A link-time identity.
///
/// A symbol ID is scoped by its compilation unit because independent
/// allocators may issue identical numeric IDs.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct DefinitionKey {
    unit: LinkUnitId,
    id: SymbolId,
}

impl DefinitionKey {
    fn new(unit: LinkUnitId, id: SymbolId) -> Self {
        Self { unit, id }
    }
}

// ============================================================================
// LOOKUP KEY
// ============================================================================

/// Internal lookup key.
///
/// Internal symbols are isolated by compilation unit.
/// External symbols participate in the shared external namespace.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum LookupKey {
    Internal {
        unit: LinkUnitId,
        name: String,
    },

    External {
        name: String,
    },
}

impl LookupKey {
    fn for_definition(definition: &SymbolDefinition) -> Self {
        let name = definition.name();

        match definition.linkage() {
            SymbolLinkage::Internal => Self::Internal {
                unit: definition.unit.clone(),
                name,
            },

            SymbolLinkage::External
            | SymbolLinkage::Exported
            | SymbolLinkage::Weak => {
                Self::External { name }
            }
        }
    }
}

// ============================================================================
// CONFIGURATION
// ============================================================================

/// Optional operational budgets for the symbol index.
///
/// `None` means no additional explicit limit is imposed.
///
/// These values are not language or hardware limits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SymbolIndexBudget {
    /// Maximum number of indexed symbol definitions.
    pub max_symbols: Option<usize>,

    /// Maximum combined UTF-8 byte length of indexed symbol names.
    pub max_name_bytes: Option<usize>,

    /// Maximum number of indexed compilation units.
    pub max_units: Option<usize>,
}

impl SymbolIndexBudget {
    /// Construct a budget without additional explicit restrictions.
    pub const fn unlimited() -> Self {
        Self {
            max_symbols: None,
            max_name_bytes: None,
            max_units: None,
        }
    }

    /// Construct an explicitly configured budget.
    pub const fn new(
        max_symbols: Option<usize>,
        max_name_bytes: Option<usize>,
        max_units: Option<usize>,
    ) -> Self {
        Self {
            max_symbols,
            max_name_bytes,
            max_units,
        }
    }
}

// ============================================================================
// SYMBOL INDEX
// ============================================================================

/// Deterministic link-time symbol index.
///
/// This index does not allocate semantic IDs or modify canonical symbols.
///
/// It provides a separate link-time view of those symbols.
#[derive(Debug, Clone, Default)]
pub struct SymbolIndex {
    definitions: BTreeMap<DefinitionKey, SymbolDefinition>,

    names: BTreeMap<LookupKey, BTreeSet<DefinitionKey>>,

    units: BTreeSet<LinkUnitId>,

    name_bytes: usize,

    budget: SymbolIndexBudget,
}

impl SymbolIndex {
    /// Construct an empty index without additional operational limits.
    pub fn new() -> Self {
        Self::default()
    }

    /// Construct an empty index with explicit operational budgets.
    pub fn with_budget(budget: SymbolIndexBudget) -> Self {
        Self {
            definitions: BTreeMap::new(),
            names: BTreeMap::new(),
            units: BTreeSet::new(),
            name_bytes: 0,
            budget,
        }
    }

    /// Return the configured budget.
    pub const fn budget(&self) -> SymbolIndexBudget {
        self.budget
    }

    /// Return the number of indexed definitions.
    pub fn len(&self) -> usize {
        self.definitions.len()
    }

    /// Return whether the index contains no definitions.
    pub fn is_empty(&self) -> bool {
        self.definitions.is_empty()
    }

    /// Return the number of distinct compilation units.
    pub fn unit_count(&self) -> usize {
        self.units.len()
    }

    /// Return the total indexed symbol-name bytes.
    pub fn name_bytes(&self) -> usize {
        self.name_bytes
    }

    // ------------------------------------------------------------------------
    // INSERTION
    // ------------------------------------------------------------------------

    /// Insert one symbol definition.
    ///
    /// The operation validates all relevant conditions before mutating
    /// the indexes.
    ///
    /// Duplicate strong definitions are rejected.
    /// Multiple weak definitions are permitted and resolved deterministically.
    pub fn insert(
        &mut self,
        definition: SymbolDefinition,
    ) -> SymbolLinkResult<()> {
        let key = DefinitionKey::new(
            definition.unit.clone(),
            definition.id(),
        );

        if self.definitions.contains_key(&key) {
            return Err(SymbolLinkError::DuplicateSymbolIdentity {
                unit: definition.unit.clone(),
                id: definition.id(),
            });
        }

        let lookup = LookupKey::for_definition(&definition);

        let name = definition.name();

        let next_symbol_count = self
            .definitions
            .len()
            .checked_add(1)
            .ok_or(SymbolLinkError::CounterOverflow)?;

        let next_name_bytes = self
            .name_bytes
            .checked_add(name.len())
            .ok_or(SymbolLinkError::CounterOverflow)?;

        let introduces_unit = !self.units.contains(&definition.unit);

        let next_unit_count = self
            .units
            .len()
            .checked_add(usize::from(introduces_unit))
            .ok_or(SymbolLinkError::CounterOverflow)?;

        self.check_budget(
            next_symbol_count,
            next_name_bytes,
            next_unit_count,
        )?;

        self.validate_definition_conflicts(
            &lookup,
            &definition,
        )?;

        // All fallible validation has completed.
        // The following operations maintain the index invariants.

        self.names
            .entry(lookup)
            .or_default()
            .insert(key.clone());

        self.units.insert(definition.unit.clone());

        self.name_bytes = next_name_bytes;

        self.definitions.insert(key, definition);

        Ok(())
    }

    /// Insert multiple definitions.
    ///
    /// The operation is atomic: if any definition fails, the original
    /// index remains unchanged.
    pub fn insert_all<I>(
        &mut self,
        definitions: I,
    ) -> SymbolLinkResult<()>
    where
        I: IntoIterator<Item = SymbolDefinition>,
    {
        let mut staged = self.clone();

        for definition in definitions {
            staged.insert(definition)?;
        }

        *self = staged;

        Ok(())
    }

    // ------------------------------------------------------------------------
    // VALIDATION
    // ------------------------------------------------------------------------

    fn check_budget(
        &self,
        symbols: usize,
        name_bytes: usize,
        units: usize,
    ) -> SymbolLinkResult<()> {
        if self
            .budget
            .max_symbols
            .is_some_and(|limit| symbols > limit)
        {
            return Err(SymbolLinkError::SymbolBudgetExceeded {
                requested: symbols,
                limit: self.budget.max_symbols.unwrap_or_default(),
            });
        }

        if self
            .budget
            .max_name_bytes
            .is_some_and(|limit| name_bytes > limit)
        {
            return Err(SymbolLinkError::NameBudgetExceeded {
                requested: name_bytes,
                limit: self.budget.max_name_bytes.unwrap_or_default(),
            });
        }

        if self
            .budget
            .max_units
            .is_some_and(|limit| units > limit)
        {
            return Err(SymbolLinkError::UnitBudgetExceeded {
                requested: units,
                limit: self.budget.max_units.unwrap_or_default(),
            });
        }

        Ok(())
    }

    fn validate_definition_conflicts(
        &self,
        lookup: &LookupKey,
        incoming: &SymbolDefinition,
    ) -> SymbolLinkResult<()> {
        let Some(existing_keys) = self.names.get(lookup) else {
            return Ok(());
        };

        for existing_key in existing_keys {
            let Some(existing) = self.definitions.get(existing_key) else {
                return Err(SymbolLinkError::IndexCorruption);
            };

            if !Self::is_definition(existing)
                || !Self::is_definition(incoming)
            {
                continue;
            }

            let existing_linkage = existing.linkage();
            let incoming_linkage = incoming.linkage();

            if existing_linkage == SymbolLinkage::Weak
                || incoming_linkage == SymbolLinkage::Weak
            {
                continue;
            }

            return Err(SymbolLinkError::DuplicateStrongDefinition {
                name: incoming.name(),
                first_unit: existing.unit.clone(),
                second_unit: incoming.unit.clone(),
            });
        }

        Ok(())
    }

    fn is_definition(definition: &SymbolDefinition) -> bool {
        matches!(
            definition.state(),
            SymbolState::Defined
                | SymbolState::ExternallyDefined
        )
    }

    // ------------------------------------------------------------------------
    // LOOKUP
    // ------------------------------------------------------------------------

    /// Resolve an external symbol by qualified name.
    ///
    /// Internal symbols are never exposed through this lookup.
    ///
    /// Strong definitions take precedence over weak definitions.
    /// Multiple weak definitions are resolved by compilation-unit ordering.
    pub fn resolve_external(
        &self,
        name: &str,
    ) -> SymbolLinkResult<Option<&SymbolDefinition>> {
        let lookup = LookupKey::External {
            name: name.to_owned(),
        };

        self.resolve_key(&lookup)
    }

    /// Resolve a symbol within a compilation unit.
    ///
    /// Internal symbols are searched first.
    /// External symbols are searched when no local definition exists.
    pub fn resolve(
        &self,
        unit: &LinkUnitId,
        name: &str,
    ) -> SymbolLinkResult<Option<&SymbolDefinition>> {
        let local = LookupKey::Internal {
            unit: unit.clone(),
            name: name.to_owned(),
        };

        if let Some(definition) = self.resolve_key(&local)? {
            return Ok(Some(definition));
        }

        self.resolve_external(name)
    }

    fn resolve_key(
        &self,
        lookup: &LookupKey,
    ) -> SymbolLinkResult<Option<&SymbolDefinition>> {
        let Some(keys) = self.names.get(lookup) else {
            return Ok(None);
        };

        let mut strong = None;
        let mut weak = None;

        for key in keys {
            let definition = self
                .definitions
                .get(key)
                .ok_or(SymbolLinkError::IndexCorruption)?;

            if !Self::is_definition(definition) {
                continue;
            }

            match definition.linkage() {
                SymbolLinkage::Weak => {
                    if weak.is_none() {
                        weak = Some(definition);
                    }
                }

                SymbolLinkage::Internal
                | SymbolLinkage::External
                | SymbolLinkage::Exported => {
                    if strong.is_some() {
                        return Err(
                            SymbolLinkError::DuplicateStrongDefinition {
                                name: definition.name(),
                                first_unit: strong
                                    .map(|value: &SymbolDefinition| {
                                        value.unit.clone()
                                    })
                                    .unwrap_or_else(|| {
                                        definition.unit.clone()
                                    }),
                                second_unit: definition.unit.clone(),
                            },
                        );
                    }

                    strong = Some(definition);
                }
            }
        }

        Ok(strong.or(weak))
    }

    /// Resolve a canonical symbol reference within a compilation unit.
    ///
    /// The symbol ID is interpreted in the supplied unit's identity scope.
    /// The expected symbol kind is checked when supplied.
    pub fn resolve_reference(
        &self,
        unit: &LinkUnitId,
        reference: &SymbolRef,
    ) -> SymbolLinkResult<&SymbolDefinition> {
        let key = DefinitionKey::new(
            unit.clone(),
            reference.id,
        );

        let definition = self
            .definitions
            .get(&key)
            .ok_or_else(|| SymbolLinkError::UnresolvedSymbol {
                unit: unit.clone(),
                id: reference.id,
            })?;

        if let Some(expected) = reference.expected_kind {
            if definition.symbol().kind != expected {
                return Err(SymbolLinkError::SymbolKindMismatch {
                    name: definition.name(),
                    expected,
                    actual: definition.symbol().kind,
                });
            }
        }

        Ok(definition)
    }

    /// Return a definition by its unit-scoped identity.
    pub fn get(
        &self,
        unit: &LinkUnitId,
        id: SymbolId,
    ) -> Option<&SymbolDefinition> {
        self.definitions.get(
            &DefinitionKey::new(unit.clone(), id)
        )
    }

    // ------------------------------------------------------------------------
    // UNDEFINED SYMBOLS
    // ------------------------------------------------------------------------

    /// Return external declarations that do not have a matching definition.
    ///
    /// Internal declarations are not treated as cross-unit imports.
    pub fn undefined_external_symbols(
        &self,
    ) -> SymbolLinkResult<Vec<&SymbolDefinition>> {
        let mut result = Vec::new();

        for definition in self.definitions.values() {
            if !matches!(
                definition.state(),
                SymbolState::Declared | SymbolState::Imported
            ) {
                continue;
            }

            if definition.linkage() == SymbolLinkage::Internal {
                continue;
            }

            if self
                .resolve_external(&definition.name())?
                .is_none()
            {
                result.push(definition);
            }
        }

        Ok(result)
    }

    // ------------------------------------------------------------------------
    // EXPORTS
    // ------------------------------------------------------------------------

    /// Return externally visible definitions.
    ///
    /// Source-level visibility remains distinct from linkage.
    pub fn exported_symbols(
        &self,
    ) -> Vec<&SymbolDefinition> {
        self.definitions
            .values()
            .filter(|definition| {
                definition.symbol().visibility
                    == SymbolVisibility::Public
                    && definition.linkage()
                        != SymbolLinkage::Internal
                    && Self::is_definition(definition)
            })
            .collect()
    }

    // ------------------------------------------------------------------------
    // REMOVAL
    // ------------------------------------------------------------------------

    /// Remove one definition by unit-scoped identity.
    ///
    /// All affected indexes are updated consistently.
    pub fn remove(
        &mut self,
        unit: &LinkUnitId,
        id: SymbolId,
    ) -> SymbolLinkResult<Option<SymbolDefinition>> {
        let key = DefinitionKey::new(unit.clone(), id);

        let Some(definition) = self.definitions.get(&key) else {
            return Ok(None);
        };

        let lookup = LookupKey::for_definition(definition);
        let name_length = definition.name().len();

        let definition = self
            .definitions
            .remove(&key)
            .ok_or(SymbolLinkError::IndexCorruption)?;

        if let Some(keys) = self.names.get_mut(&lookup) {
            keys.remove(&key);

            if keys.is_empty() {
                self.names.remove(&lookup);
            }
        } else {
            return Err(SymbolLinkError::IndexCorruption);
        }

        self.name_bytes = self
            .name_bytes
            .checked_sub(name_length)
            .ok_or(SymbolLinkError::IndexCorruption)?;

        if !self
            .definitions
            .keys()
            .any(|existing| existing.unit == *unit)
        {
            self.units.remove(unit);
        }

        Ok(Some(definition))
    }

    // ------------------------------------------------------------------------
    // ITERATION
    // ------------------------------------------------------------------------

    /// Iterate over all definitions in deterministic order.
    pub fn definitions(
        &self,
    ) -> impl Iterator<Item = &SymbolDefinition> {
        self.definitions.values()
    }

    /// Iterate over compilation-unit identities.
    pub fn units(
        &self,
    ) -> impl Iterator<Item = &LinkUnitId> {
        self.units.iter()
    }

    // ------------------------------------------------------------------------
    // CONSISTENCY
    // ------------------------------------------------------------------------

    /// Verify the consistency of all internal indexes.
    ///
    /// This is useful after deserialization, incremental updates,
    /// and complex linking operations.
    pub fn verify_consistency(&self) -> SymbolLinkResult<()> {
        let mut calculated_name_bytes = 0usize;

        let mut expected_units = BTreeSet::new();

        let mut expected_names: BTreeMap<
            LookupKey,
            BTreeSet<DefinitionKey>,
        > = BTreeMap::new();

        for (key, definition) in &self.definitions {
            if key.unit != definition.unit
                || key.id != definition.id()
            {
                return Err(SymbolLinkError::IndexCorruption);
            }

            definition
                .symbol()
                .validate()
                .map_err(|error| {
                    SymbolLinkError::InvalidSymbol {
                        unit: definition.unit.clone(),
                        reason: error.to_string(),
                    }
                })?;

            calculated_name_bytes = calculated_name_bytes
                .checked_add(definition.name().len())
                .ok_or(SymbolLinkError::CounterOverflow)?;

            expected_units.insert(definition.unit.clone());

            expected_names
                .entry(LookupKey::for_definition(definition))
                .or_default()
                .insert(key.clone());
        }

        if calculated_name_bytes != self.name_bytes
            || expected_units != self.units
            || expected_names != self.names
        {
            return Err(SymbolLinkError::IndexCorruption);
        }

        self.check_budget(
            self.definitions.len(),
            self.name_bytes,
            self.units.len(),
        )?;

        Ok(())
    }
}

// ============================================================================
// ERRORS
// ============================================================================

/// Structured linking symbol errors.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum SymbolLinkError {
    /// Empty compilation-unit identifier.
    EmptyUnitId,

    /// Invalid canonical symbol.
    InvalidSymbol {
        /// Compilation unit.
        unit: LinkUnitId,

        /// Validation explanation.
        reason: String,
    },

    /// Duplicate unit-scoped symbol identity.
    DuplicateSymbolIdentity {
        /// Compilation unit.
        unit: LinkUnitId,

        /// Symbol identity.
        id: SymbolId,
    },

    /// Duplicate strong definitions.
    DuplicateStrongDefinition {
        /// Conflicting qualified name.
        name: String,

        /// First definition's compilation unit.
        first_unit: LinkUnitId,

        /// Second definition's compilation unit.
        second_unit: LinkUnitId,
    },

    /// A referenced symbol cannot be found.
    UnresolvedSymbol {
        /// Requesting compilation unit.
        unit: LinkUnitId,

        /// Requested identity.
        id: SymbolId,
    },

    /// The actual symbol category differs from the expected category.
    SymbolKindMismatch {
        /// Qualified name.
        name: String,

        /// Expected category.
        expected: SymbolKind,

        /// Actual category.
        actual: SymbolKind,
    },

    /// Configured symbol-count budget exceeded.
    SymbolBudgetExceeded {
        /// Requested count.
        requested: usize,

        /// Configured budget.
        limit: usize,
    },

    /// Configured name-byte budget exceeded.
    NameBudgetExceeded {
        /// Requested bytes.
        requested: usize,

        /// Configured budget.
        limit: usize,
    },

    /// Configured compilation-unit budget exceeded.
    UnitBudgetExceeded {
        /// Requested count.
        requested: usize,

        /// Configured budget.
        limit: usize,
    },

    /// An internal counter overflowed.
    CounterOverflow,

    /// Internal indexes are inconsistent.
    IndexCorruption,
}

impl fmt::Display for SymbolLinkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyUnitId => {
                write!(f, "compilation-unit identifier is empty")
            }

            Self::InvalidSymbol { unit, reason } => {
                write!(
                    f,
                    "invalid symbol in compilation unit '{unit}': {reason}"
                )
            }

            Self::DuplicateSymbolIdentity { unit, id } => {
                write!(
                    f,
                    "duplicate symbol identity '{id:?}' in unit '{unit}'"
                )
            }

            Self::DuplicateStrongDefinition {
                name,
                first_unit,
                second_unit,
            } => {
                write!(
                    f,
                    "duplicate strong definition '{name}' in \
                     '{first_unit}' and '{second_unit}'"
                )
            }

            Self::UnresolvedSymbol { unit, id } => {
                write!(
                    f,
                    "unresolved symbol '{id:?}' requested by '{unit}'"
                )
            }

            Self::SymbolKindMismatch {
                name,
                expected,
                actual,
            } => {
                write!(
                    f,
                    "symbol '{name}' has kind '{actual:?}', \
                     expected '{expected:?}'"
                )
            }

            Self::SymbolBudgetExceeded { requested, limit } => {
                write!(
                    f,
                    "symbol budget exceeded: requested {requested}, \
                     configured limit {limit}"
                )
            }

            Self::NameBudgetExceeded { requested, limit } => {
                write!(
                    f,
                    "symbol name-byte budget exceeded: requested \
                     {requested}, configured limit {limit}"
                )
            }

            Self::UnitBudgetExceeded { requested, limit } => {
                write!(
                    f,
                    "compilation-unit budget exceeded: requested \
                     {requested}, configured limit {limit}"
                )
            }

            Self::CounterOverflow => {
                write!(f, "symbol index counter overflow")
            }

            Self::IndexCorruption => {
                write!(f, "linking symbol index consistency failure")
            }
        }
    }
}

impl Error for SymbolLinkError {}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::zuir::schema::types::QualifiedName;

    fn unit(name: &str) -> LinkUnitId {
        LinkUnitId::new(name).expect("valid unit")
    }

    fn symbol(
        id: u128,
        name: &str,
        linkage: SymbolLinkage,
        state: SymbolState,
    ) -> Symbol {
        let id = SymbolId::new(id).expect("nonzero symbol ID");

        let name = QualifiedName::parse(name)
            .expect("valid qualified name");

        Symbol::new(id, name, SymbolKind::Function)
            .with_visibility(SymbolVisibility::Public)
            .with_linkage(linkage)
            .with_state(state)
    }

    fn definition(
        unit_name: &str,
        id: u128,
        name: &str,
        linkage: SymbolLinkage,
        state: SymbolState,
    ) -> SymbolDefinition {
        SymbolDefinition::new(
            unit(unit_name),
            symbol(id, name, linkage, state),
        )
        .expect("valid definition")
    }

    #[test]
    fn inserts_and_resolves_external_definition() {
        let mut index = SymbolIndex::new();

        index.insert(definition(
            "module_a",
            1,
            "math::calculate",
            SymbolLinkage::External,
            SymbolState::Defined,
        )).unwrap();

        assert!(
            index.resolve_external("math::calculate")
                .unwrap()
                .is_some()
        );
    }

    #[test]
    fn internal_symbols_are_unit_scoped() {
        let mut index = SymbolIndex::new();

        index.insert(definition(
            "module_a",
            1,
            "internal::helper",
            SymbolLinkage::Internal,
            SymbolState::Defined,
        )).unwrap();

        assert!(
            index.resolve(&unit("module_a"), "internal::helper")
                .unwrap()
                .is_some()
        );

        assert!(
            index.resolve(&unit("module_b"), "internal::helper")
                .unwrap()
                .is_none()
        );
    }

    #[test]
    fn duplicate_strong_definitions_are_rejected() {
        let mut index = SymbolIndex::new();

        index.insert(definition(
            "module_a",
            1,
            "math::calculate",
            SymbolLinkage::External,
            SymbolState::Defined,
        )).unwrap();

        let result = index.insert(definition(
            "module_b",
            2,
            "math::calculate",
            SymbolLinkage::External,
            SymbolState::Defined,
        ));

        assert!(matches!(
            result,
            Err(SymbolLinkError::DuplicateStrongDefinition { .. })
        ));
    }

    #[test]
    fn weak_definitions_can_coexist() {
        let mut index = SymbolIndex::new();

        index.insert(definition(
            "module_b",
            2,
            "math::optional",
            SymbolLinkage::Weak,
            SymbolState::Defined,
        )).unwrap();

        index.insert(definition(
            "module_a",
            1,
            "math::optional",
            SymbolLinkage::Weak,
            SymbolState::Defined,
        )).unwrap();

        assert!(index
            .resolve_external("math::optional")
            .unwrap()
            .is_some());
    }

    #[test]
    fn strong_definition_overrides_weak_definition() {
        let mut index = SymbolIndex::new();

        index.insert(definition(
            "module_a",
            1,
            "math::calculate",
            SymbolLinkage::Weak,
            SymbolState::Defined,
        )).unwrap();

        index.insert(definition(
            "module_b",
            2,
            "math::calculate",
            SymbolLinkage::External,
            SymbolState::Defined,
        )).unwrap();

        let resolved = index
            .resolve_external("math::calculate")
            .unwrap()
            .unwrap();

        assert_eq!(
            resolved.linkage(),
            SymbolLinkage::External
        );
    }

    #[test]
    fn declarations_are_reported_as_undefined() {
        let mut index = SymbolIndex::new();

        index.insert(definition(
            "module_a",
            1,
            "math::external",
            SymbolLinkage::External,
            SymbolState::Declared,
        )).unwrap();

        assert_eq!(
            index.undefined_external_symbols().unwrap().len(),
            1
        );
    }

    #[test]
    fn duplicate_identity_is_rejected() {
        let mut index = SymbolIndex::new();

        index.insert(definition(
            "module_a",
            1,
            "math::first",
            SymbolLinkage::Internal,
            SymbolState::Defined,
        )).unwrap();

        let result = index.insert(definition(
            "module_a",
            1,
            "math::second",
            SymbolLinkage::Internal,
            SymbolState::Defined,
        ));

        assert!(matches!(
            result,
            Err(SymbolLinkError::DuplicateSymbolIdentity { .. })
        ));
    }

    #[test]
    fn insertion_batch_is_atomic() {
        let mut index = SymbolIndex::new();

        index.insert(definition(
            "module_a",
            1,
            "math::calculate",
            SymbolLinkage::External,
            SymbolState::Defined,
        )).unwrap();

        let result = index.insert_all(vec![
            definition(
                "module_b",
                2,
                "math::other",
                SymbolLinkage::External,
                SymbolState::Defined,
            ),
            definition(
                "module_c",
                3,
                "math::calculate",
                SymbolLinkage::External,
                SymbolState::Defined,
            ),
        ]);

        assert!(result.is_err());
        assert_eq!(index.len(), 1);
    }

    #[test]
    fn removal_preserves_index_consistency() {
        let mut index = SymbolIndex::new();

        index.insert(definition(
            "module_a",
            1,
            "math::calculate",
            SymbolLinkage::External,
            SymbolState::Defined,
        )).unwrap();

        let removed = index.remove(&unit("module_a"), SymbolId::new(1).unwrap())
            .unwrap();

        assert!(removed.is_some());
        assert_eq!(index.len(), 0);
        assert!(index.verify_consistency().is_ok());
    }

    #[test]
    fn configured_budgets_are_enforced() {
        let mut index = SymbolIndex::with_budget(
            SymbolIndexBudget::new(
                Some(1),
                None,
                None,
            )
        );

        index.insert(definition(
            "module_a",
            1,
            "math::first",
            SymbolLinkage::Internal,
            SymbolState::Defined,
        )).unwrap();

        let result = index.insert(definition(
            "module_a",
            2,
            "math::second",
            SymbolLinkage::Internal,
            SymbolState::Defined,
        ));

        assert!(matches!(
            result,
            Err(SymbolLinkError::SymbolBudgetExceeded { .. })
        ));
    }

    #[test]
    fn large_symbol_names_are_not_artificially_limited() {
        let long_name = format!(
            "domain::{}",
            "x".repeat(16_384)
        );

        let mut index = SymbolIndex::new();

        index.insert(definition(
            "module_a",
            1,
            &long_name,
            SymbolLinkage::Internal,
            SymbolState::Defined,
        )).unwrap();

        assert_eq!(index.len(), 1);
    }

    #[test]
    fn empty_index_is_consistent() {
        let index = SymbolIndex::new();

        assert!(index.is_empty());
        assert!(index.verify_consistency().is_ok());
    }
}
