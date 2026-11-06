#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani Compiler — Module Composition
//!
//! Canonical path: `src/compiler/linking/composition.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//!
//! ## PURPOSE
//!
//! Compose independently compiled canonical ZUIR modules into a deterministic
//! link-time composition without flattening or redefining their semantics.
//!
//! Composition validates:
//!
//! - module-local invariants;
//! - unique module identities;
//! - unique module names within the composition;
//! - compilation-unit identities;
//! - canonical symbol ownership;
//! - declared module exports;
//! - imported symbol declarations;
//! - cross-module exported symbol resolution;
//! - symbol-kind compatibility;
//! - optional unresolved external imports;
//! - consistency of the resulting symbol index.
//!
//! ## OWNERSHIP
//!
//! OWNS:
//! - Composition of independently compiled ZUIR modules.
//! - Composition-unit identity association.
//! - Duplicate module identity detection.
//! - Duplicate module-name detection.
//! - Module export/import consistency checks.
//! - Composition-level symbol-resolution validation.
//! - Deterministic composed-module indexing.
//!
//! DOES NOT OWN:
//! - Source parsing or AST construction.
//! - Source-level module-name resolution.
//! - Package or dependency downloading.
//! - Canonical module representation.
//! - Canonical symbol representation.
//! - Symbol ID allocation.
//! - Type compatibility or type inference.
//! - Semantic analysis.
//! - ABI or calling conventions.
//! - Object-file parsing.
//! - Section placement.
//! - Relocation execution.
//! - Target-specific code generation.
//! - Runtime loading or execution.
//! - Physical hardware allocation.
//!
//! ## CANONICAL INTEGRATION
//!
//! Uses:
//!
//! - `crate::zuir::ids::{ModuleId, SymbolId}`
//! - `crate::zuir::ir::module::ZuirModule`
//! - `crate::zuir::values::symbols::{SymbolLinkage, SymbolState, SymbolVisibility}`
//! - `crate::compiler::linking::symbols::{LinkUnitId, SymbolDefinition, SymbolIndex}`
//!
//! ## GRAMMAR BOUNDARY
//!
//! Source-level module declarations and imports are defined by:
//!
//! - `grammar/modules/modules.g4`
//! - `grammar/modules/imports.g4`
//! - `grammar/modules/exports.g4`
//! - `grammar/modules/dependencies.g4`
//!
//! These grammars define syntax, not link-time composition.
//!
//! The frontend and semantic layer must resolve source-level declarations
//! before this module receives canonical ZUIR modules.
//!
//! ## SCALABILITY
//!
//! No fixed maximum is imposed on:
//!
//! - modules;
//! - functions;
//! - symbols;
//! - imports;
//! - exports;
//! - compilation units;
//! - computational domains;
//! - target architectures;
//! - hardware resources.
//!
//! Collections grow according to available implementation resources.
//! Symbol-index operational budgets are supplied by the caller.
//!
//! Literal infinity cannot be guaranteed: memory, address space, execution
//! time, and physical resources are finite.
//!
//! ## DETERMINISM
//!
//! Ordered collections provide deterministic traversal and lookup.
//! Composition does not depend on input iteration order for its final module
//! and symbol indexes.
//!
//! ## TRANSACTIONAL BEHAVIOR
//!
//! Composition consumes its inputs and builds a private staged result.
//! A failed composition returns an error and exposes no partially composed
//! result.
//!
//! ## SEMANTIC PRESERVATION
//!
//! Modules remain separate canonical modules.
//! This file does not merge function bodies, rewrite IDs, or change program
//! semantics.
//!
//! Type compatibility, effect compatibility, ownership correctness,
//! full-program verification, and domain-specific correctness remain the
//! responsibility of their canonical authorities.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use crate::compiler::linking::symbols::{
    LinkUnitId,
    SymbolDefinition,
    SymbolIndex,
    SymbolIndexBudget,
    SymbolLinkError,
};

use crate::zuir::ids::{ModuleId, SymbolId};
use crate::zuir::ir::module::{ModuleError, ZuirModule};
use crate::zuir::values::symbols::{
    SymbolLinkage,
    SymbolState,
    SymbolVisibility,
};

// ============================================================================
// API VERSION
// ============================================================================

/// Public API version for the module-composition subsystem.
pub const COMPOSITION_API_VERSION: &str = "1.0.0";

// ============================================================================
// RESULT
// ============================================================================

/// Result type used by module-composition operations.
pub type CompositionResult<T> = Result<T, CompositionError>;

// ============================================================================
// COMPOSITION POLICY
// ============================================================================

/// Policies controlling module-composition validation.
///
/// These are operational and integration policies. They do not change
/// the meaning of Zamani programs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CompositionPolicy {
    /// Permit unresolved external imports.
    ///
    /// When enabled, an import without a matching module export may remain
    /// unresolved for a later external-linking or deployment stage.
    ///
    /// This must only be enabled when a later stage explicitly owns the
    /// responsibility for resolving those imports.
    ///
    /// It does not permit unresolved internal symbols.
    pub allow_unresolved_external_imports: bool,
}

impl CompositionPolicy {
    /// Require every module import to resolve to an exported definition.
    pub const fn strict() -> Self {
        Self {
            allow_unresolved_external_imports: false,
        }
    }

    /// Permit unresolved external imports for later resolution.
    pub const fn allow_external_imports() -> Self {
        Self {
            allow_unresolved_external_imports: true,
        }
    }
}

// ============================================================================
// INPUT UNIT
// ============================================================================

/// One independently compiled module and its canonical symbol definitions.
///
/// A compilation unit is distinct from a module identity:
///
/// - `LinkUnitId` identifies the input to the linker.
/// - `ModuleId` identifies the canonical ZUIR module.
///
/// A compilation unit may contain one canonical module in this composition
/// interface. Multi-module object inputs should be represented by multiple
/// composition units or by a higher-level artifact adapter.
#[derive(Debug, Clone)]
pub struct CompositionUnit<F> {
    unit: LinkUnitId,
    module: ZuirModule<F>,
    symbols: Vec<SymbolDefinition>,
}

impl<F> CompositionUnit<F> {
    /// Construct a composition unit.
    ///
    /// The module and symbol definitions are validated during composition.
    /// Symbol definitions must carry the same `LinkUnitId` supplied here.
    pub fn new(
        unit: LinkUnitId,
        module: ZuirModule<F>,
        symbols: impl IntoIterator<Item = SymbolDefinition>,
    ) -> Self {
        Self {
            unit,
            module,
            symbols: symbols.into_iter().collect(),
        }
    }

    /// Return the compilation-unit identity.
    pub fn unit(&self) -> &LinkUnitId {
        &self.unit
    }

    /// Return the canonical module.
    pub fn module(&self) -> &ZuirModule<F> {
        &self.module
    }

    /// Return the supplied canonical symbol definitions.
    pub fn symbols(&self) -> &[SymbolDefinition] {
        &self.symbols
    }
}

// ============================================================================
// COMPOSITION RESULT
// ============================================================================

/// Successfully composed canonical modules and their link-time symbol index.
///
/// The contained modules retain their original IDs, names, functions,
/// imports, exports, and descriptive metadata.
///
/// No module is flattened or rewritten.
#[derive(Debug, Clone)]
pub struct ModuleComposition<F> {
    modules: BTreeMap<ModuleId, ZuirModule<F>>,
    names: BTreeMap<String, ModuleId>,
    units: BTreeMap<ModuleId, LinkUnitId>,
    symbols: SymbolIndex,
    exported_symbols: BTreeSet<(LinkUnitId, SymbolId)>,
    function_count: usize,
}

impl<F> ModuleComposition<F> {
    /// Compose independently compiled modules.
    ///
    /// The operation is transactional: an error returns no partially
    /// composed result.
    ///
    /// Input ordering does not determine the final module traversal order.
    pub fn compose<I>(
        inputs: I,
        symbol_budget: SymbolIndexBudget,
        policy: CompositionPolicy,
    ) -> CompositionResult<Self>
    where
        I: IntoIterator<Item = CompositionUnit<F>>,
    {
        let mut modules = BTreeMap::new();
        let mut names = BTreeMap::new();
        let mut units = BTreeMap::new();
        let mut unit_ids = BTreeSet::new();

        let mut symbol_index = SymbolIndex::with_budget(symbol_budget);

        let mut function_count = 0usize;

        // Stage all module and symbol information.
        //
        // Nothing is exposed to callers until every validation succeeds.
        for input in inputs {
            input
                .module
                .validate()
                .map_err(CompositionError::InvalidModule)?;

            let module_id = input.module.id();
            let module_name = input.module.name().as_str().to_owned();

            if modules.contains_key(&module_id) {
                return Err(CompositionError::DuplicateModuleId {
                    id: module_id,
                });
            }

            if let Some(existing_id) = names.get(&module_name) {
                return Err(CompositionError::DuplicateModuleName {
                    name: module_name,
                    first: *existing_id,
                    second: module_id,
                });
            }

            if !unit_ids.insert(input.unit.clone()) {
                return Err(CompositionError::DuplicateCompilationUnit {
                    unit: input.unit,
                });
            }

            let next_function_count = function_count
                .checked_add(input.module.function_count())
                .ok_or(CompositionError::CounterOverflow)?;

            // Every symbol must belong to this input compilation unit.
            for definition in input.symbols {
                if definition.unit() != &input.unit {
                    return Err(
                        CompositionError::SymbolUnitMismatch {
                            expected: input.unit.clone(),
                            actual: definition.unit().clone(),
                            symbol: definition.id(),
                        },
                    );
                }

                symbol_index
                    .insert(definition)
                    .map_err(CompositionError::SymbolLink)?;
            }

            names.insert(module_name, module_id);
            units.insert(module_id, input.unit);
            modules.insert(module_id, input.module);

            function_count = next_function_count;
        }

        // The module and symbol indexes must agree before cross-module
        // relationships are validated.
        symbol_index
            .verify_consistency()
            .map_err(CompositionError::SymbolLink)?;

        // Validate that every declared export corresponds to a valid
        // externally linkable public definition.
        let exported_symbols = Self::collect_and_validate_exports(
            &modules,
            &units,
            &symbol_index,
        )?;

        // Validate imports against the composed exported-symbol set.
        Self::validate_imports(
            &modules,
            &units,
            &symbol_index,
            &exported_symbols,
            policy,
        )?;

        Ok(Self {
            modules,
            names,
            units,
            symbols: symbol_index,
            exported_symbols,
            function_count,
        })
    }

    // ------------------------------------------------------------------------
    // EXPORT VALIDATION
    // ------------------------------------------------------------------------

    fn collect_and_validate_exports(
        modules: &BTreeMap<ModuleId, ZuirModule<F>>,
        units: &BTreeMap<ModuleId, LinkUnitId>,
        symbols: &SymbolIndex,
    ) -> CompositionResult<BTreeSet<(LinkUnitId, SymbolId)>> {
        let mut exported = BTreeSet::new();

        for (module_id, module) in modules {
            let unit = units
                .get(module_id)
                .ok_or(CompositionError::IndexCorruption)?;

            for symbol_id in module.exports() {
                let definition = symbols
                    .get(unit, symbol_id)
                    .ok_or(
                        CompositionError::MissingExportDefinition {
                            module: *module_id,
                            symbol: symbol_id,
                        },
                    )?;

                let symbol = definition.symbol();

                if !matches!(
                    definition.state(),
                    SymbolState::Defined
                        | SymbolState::ExternallyDefined
                ) {
                    return Err(
                        CompositionError::ExportIsNotDefined {
                            module: *module_id,
                            symbol: symbol_id,
                        },
                    );
                }

                if symbol.visibility != SymbolVisibility::Public {
                    return Err(
                        CompositionError::ExportIsNotPublic {
                            module: *module_id,
                            symbol: symbol_id,
                        },
                    );
                }

                if definition.linkage() == SymbolLinkage::Internal {
                    return Err(
                        CompositionError::ExportHasInternalLinkage {
                            module: *module_id,
                            symbol: symbol_id,
                        },
                    );
                }

                exported.insert((unit.clone(), symbol_id));
            }
        }

        Ok(exported)
    }

    // ------------------------------------------------------------------------
    // IMPORT VALIDATION
    // ------------------------------------------------------------------------

    fn validate_imports(
        modules: &BTreeMap<ModuleId, ZuirModule<F>>,
        units: &BTreeMap<ModuleId, LinkUnitId>,
        symbols: &SymbolIndex,
        exported_symbols: &BTreeSet<(LinkUnitId, SymbolId)>,
        policy: CompositionPolicy,
    ) -> CompositionResult<()> {
        for (module_id, module) in modules {
            let importing_unit = units
                .get(module_id)
                .ok_or(CompositionError::IndexCorruption)?;

            for imported_id in module.imports() {
                let imported = symbols
                    .get(importing_unit, imported_id)
                    .ok_or(
                        CompositionError::MissingImportDeclaration {
                            module: *module_id,
                            symbol: imported_id,
                        },
                    )?;

                if !matches!(
                    imported.state(),
                    SymbolState::Declared | SymbolState::Imported
                ) {
                    return Err(
                        CompositionError::ImportHasDefinitionState {
                            module: *module_id,
                            symbol: imported_id,
                        },
                    );
                }

                if imported.linkage() == SymbolLinkage::Internal {
                    return Err(
                        CompositionError::ImportHasInternalLinkage {
                            module: *module_id,
                            symbol: imported_id,
                        },
                    );
                }

                let imported_symbol = imported.symbol();
                let imported_name = imported.name();

                let resolved = symbols
                    .resolve_external(&imported_name)
                    .map_err(CompositionError::SymbolLink)?;

                let Some(provider) = resolved else {
                    if policy.allow_unresolved_external_imports {
                        continue;
                    }

                    return Err(
                        CompositionError::UnresolvedImport {
                            module: *module_id,
                            symbol: imported_id,
                            name: imported_name,
                        },
                    );
                };

                if provider.unit() == importing_unit {
                    return Err(
                        CompositionError::SelfProvidedImport {
                            module: *module_id,
                            symbol: imported_id,
                            name: imported_name,
                        },
                    );
                }

                let provider_symbol = provider.symbol();

                if provider_symbol.visibility != SymbolVisibility::Public {
                    return Err(
                        CompositionError::ImportProviderNotPublic {
                            module: *module_id,
                            symbol: imported_id,
                            provider: provider.unit().clone(),
                            name: imported_name,
                        },
                    );
                }

                if provider.linkage() == SymbolLinkage::Internal {
                    return Err(
                        CompositionError::ImportProviderNotExternallyLinked {
                            module: *module_id,
                            symbol: imported_id,
                            provider: provider.unit().clone(),
                            name: imported_name,
                        },
                    );
                }

                if !matches!(
                    provider.state(),
                    SymbolState::Defined
                        | SymbolState::ExternallyDefined
                ) {
                    return Err(
                        CompositionError::ImportProviderNotDefined {
                            module: *module_id,
                            symbol: imported_id,
                            provider: provider.unit().clone(),
                            name: imported_name,
                        },
                    );
                }

                if !exported_symbols.contains(&(
                    provider.unit().clone(),
                    provider.id(),
                )) {
                    return Err(
                        CompositionError::ImportProviderDidNotExport {
                            module: *module_id,
                            symbol: imported_id,
                            provider: provider.unit().clone(),
                            provider_symbol: provider.id(),
                            name: imported_name,
                        },
                    );
                }

                if provider_symbol.kind != imported_symbol.kind {
                    return Err(
                        CompositionError::ImportKindMismatch {
                            module: *module_id,
                            symbol: imported_id,
                            name: imported_name,
                        },
                    );
                }

                // Type compatibility, overload resolution, effect
                // compatibility, and access-control semantics are deliberately
                // delegated to semantic analysis and the canonical verifier.
            }
        }

        Ok(())
    }

    // ------------------------------------------------------------------------
    // ACCESSORS
    // ------------------------------------------------------------------------

    /// Return the number of composed modules.
    pub fn module_count(&self) -> usize {
        self.modules.len()
    }

    /// Return whether the composition contains no modules.
    pub fn is_empty(&self) -> bool {
        self.modules.is_empty()
    }

    /// Return the total number of functions across composed modules.
    pub fn function_count(&self) -> usize {
        self.function_count
    }

    /// Return the number of indexed symbol definitions.
    pub fn symbol_count(&self) -> usize {
        self.symbols.len()
    }

    /// Return the number of distinct compilation units.
    pub fn compilation_unit_count(&self) -> usize {
        self.symbols.unit_count()
    }

    /// Return the number of module-declared exported symbols.
    pub fn exported_symbol_count(&self) -> usize {
        self.exported_symbols.len()
    }

    /// Borrow a composed module by canonical identity.
    pub fn module(&self, id: ModuleId) -> Option<&ZuirModule<F>> {
        self.modules.get(&id)
    }

    /// Borrow a composed module by canonical module name.
    pub fn module_by_name(&self, name: &str) -> Option<&ZuirModule<F>> {
        let id = self.names.get(name)?;
        self.modules.get(id)
    }

    /// Return the compilation-unit identity associated with a module.
    pub fn compilation_unit(&self, id: ModuleId) -> Option<&LinkUnitId> {
        self.units.get(&id)
    }

    /// Iterate over modules in deterministic ModuleId order.
    pub fn modules(
        &self,
    ) -> impl ExactSizeIterator<Item = (ModuleId, &ZuirModule<F>)> + '_ {
        self.modules
            .iter()
            .map(|(id, module)| (*id, module))
    }

    /// Borrow the canonical link-time symbol index.
    pub fn symbol_index(&self) -> &SymbolIndex {
        &self.symbols
    }

    /// Iterate over module-declared exported symbol identities.
    ///
    /// Each identity is scoped by its compilation unit.
    pub fn exported_symbols(
        &self,
    ) -> impl Iterator<Item = (&LinkUnitId, SymbolId)> + '_ {
        self.exported_symbols
            .iter()
            .map(|(unit, symbol)| (unit, *symbol))
    }

    /// Consume the composition and return its canonical modules and
    /// link-time symbol index.
    pub fn into_parts(
        self,
    ) -> (
        BTreeMap<ModuleId, ZuirModule<F>>,
        SymbolIndex,
    ) {
        (self.modules, self.symbols)
    }

    /// Revalidate composition-level indexes.
    ///
    /// This method is useful after deserialization or when a composition
    /// has crossed a persistence boundary.
    pub fn validate(&self) -> CompositionResult<()> {
        if self.modules.len() != self.names.len()
            || self.modules.len() != self.units.len()
        {
            return Err(CompositionError::IndexCorruption);
        }

        let mut calculated_functions = 0usize;

        for (id, module) in &self.modules {
            module
                .validate()
                .map_err(CompositionError::InvalidModule)?;

            if module.id() != *id {
                return Err(CompositionError::IndexCorruption);
            }

            let name = module.name().as_str();

            if self.names.get(name) != Some(id) {
                return Err(CompositionError::IndexCorruption);
            }

            if !self.units.contains_key(id) {
                return Err(CompositionError::IndexCorruption);
            }

            calculated_functions = calculated_functions
                .checked_add(module.function_count())
                .ok_or(CompositionError::CounterOverflow)?;
        }

        if calculated_functions != self.function_count {
            return Err(CompositionError::IndexCorruption);
        }

        self.symbols
            .verify_consistency()
            .map_err(CompositionError::SymbolLink)?;

        // The exported set is derived from the canonical module export
        // declarations. Recompute it to detect inconsistent persisted state.
        let calculated_exports = Self::collect_and_validate_exports(
            &self.modules,
            &self.units,
            &self.symbols,
        )?;

        if calculated_exports != self.exported_symbols {
            return Err(CompositionError::IndexCorruption);
        }

        Ok(())
    }
}

// ============================================================================
// ERRORS
// ============================================================================

/// Structured module-composition errors.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum CompositionError {
    /// A canonical module failed its local validation.
    InvalidModule(ModuleError),

    /// A module identity already exists in this composition.
    DuplicateModuleId {
        /// Conflicting module identity.
        id: ModuleId,
    },

    /// A module name already exists in this composition.
    DuplicateModuleName {
        /// Conflicting canonical module name.
        name: String,

        /// First module identity.
        first: ModuleId,

        /// Second module identity.
        second: ModuleId,
    },

    /// A compilation-unit identity was reused.
    DuplicateCompilationUnit {
        /// Conflicting compilation unit.
        unit: LinkUnitId,
    },

    /// A symbol was associated with a different compilation unit.
    SymbolUnitMismatch {
        /// Expected compilation unit.
        expected: LinkUnitId,

        /// Actual compilation unit.
        actual: LinkUnitId,

        /// Canonical symbol identity.
        symbol: SymbolId,
    },

    /// An export has no canonical symbol definition.
    MissingExportDefinition {
        /// Owning module.
        module: ModuleId,

        /// Exported symbol identity.
        symbol: SymbolId,
    },

    /// An exported symbol is not defined.
    ExportIsNotDefined {
        /// Owning module.
        module: ModuleId,

        /// Exported symbol identity.
        symbol: SymbolId,
    },

    /// An exported symbol is not publicly visible.
    ExportIsNotPublic {
        /// Owning module.
        module: ModuleId,

        /// Exported symbol identity.
        symbol: SymbolId,
    },

    /// An internal symbol was declared as a module export.
    ExportHasInternalLinkage {
        /// Owning module.
        module: ModuleId,

        /// Exported symbol identity.
        symbol: SymbolId,
    },

    /// An import has no canonical declaration in its importing unit.
    MissingImportDeclaration {
        /// Importing module.
        module: ModuleId,

        /// Imported symbol identity.
        symbol: SymbolId,
    },

    /// An import refers to a definition rather than an import declaration.
    ImportHasDefinitionState {
        /// Importing module.
        module: ModuleId,

        /// Imported symbol identity.
        symbol: SymbolId,
    },

    /// An internal symbol was declared as an import.
    ImportHasInternalLinkage {
        /// Importing module.
        module: ModuleId,

        /// Imported symbol identity.
        symbol: SymbolId,
    },

    /// No external definition satisfies an import.
    UnresolvedImport {
        /// Importing module.
        module: ModuleId,

        /// Imported symbol identity.
        symbol: SymbolId,

        /// Canonical qualified name.
        name: String,
    },

    /// An import resolves to a symbol in its own compilation unit.
    SelfProvidedImport {
        /// Importing module.
        module: ModuleId,

        /// Imported symbol identity.
        symbol: SymbolId,

        /// Canonical qualified name.
        name: String,
    },

    /// The selected provider is not publicly visible.
    ImportProviderNotPublic {
        /// Importing module.
        module: ModuleId,

        /// Imported symbol identity.
        symbol: SymbolId,

        /// Provider compilation unit.
        provider: LinkUnitId,

        /// Canonical qualified name.
        name: String,
    },

    /// The selected provider does not have external linkage.
    ImportProviderNotExternallyLinked {
        /// Importing module.
        module: ModuleId,

        /// Imported symbol identity.
        symbol: SymbolId,

        /// Provider compilation unit.
        provider: LinkUnitId,

        /// Canonical qualified name.
        name: String,
    },

    /// The selected provider has not reached a definition state.
    ImportProviderNotDefined {
        /// Importing module.
        module: ModuleId,

        /// Imported symbol identity.
        symbol: SymbolId,

        /// Provider compilation unit.
        provider: LinkUnitId,

        /// Canonical qualified name.
        name: String,
    },

    /// The selected provider did not declare the symbol as an export.
    ImportProviderDidNotExport {
        /// Importing module.
        module: ModuleId,

        /// Imported symbol identity.
        symbol: SymbolId,

        /// Provider compilation unit.
        provider: LinkUnitId,

        /// Provider symbol identity.
        provider_symbol: SymbolId,

        /// Canonical qualified name.
        name: String,
    },

    /// The imported symbol and provider have different symbol kinds.
    ImportKindMismatch {
        /// Importing module.
        module: ModuleId,

        /// Imported symbol identity.
        symbol: SymbolId,

        /// Canonical qualified name.
        name: String,
    },

    /// A canonical symbol-index operation failed.
    SymbolLink(SymbolLinkError),

    /// A checked counter exceeded the host representation.
    CounterOverflow,

    /// An internal composition index is inconsistent.
    IndexCorruption,
}

impl fmt::Display for CompositionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidModule(error) => {
                write!(f, "invalid module: {error}")
            }

            Self::DuplicateModuleId { id } => {
                write!(f, "duplicate module identity: {id}")
            }

            Self::DuplicateModuleName {
                name,
                first,
                second,
            } => {
                write!(
                    f,
                    "duplicate module name '{name}' for modules \
                     {first} and {second}"
                )
            }

            Self::DuplicateCompilationUnit { unit } => {
                write!(f, "duplicate compilation unit: {unit}")
            }

            Self::SymbolUnitMismatch {
                expected,
                actual,
                symbol,
            } => {
                write!(
                    f,
                    "symbol {symbol} belongs to compilation unit \
                     '{actual}', expected '{expected}'"
                )
            }

            Self::MissingExportDefinition { module, symbol } => {
                write!(
                    f,
                    "module {module} exports symbol {symbol}, \
                     but no canonical definition was supplied"
                )
            }

            Self::ExportIsNotDefined { module, symbol } => {
                write!(
                    f,
                    "module {module} exports symbol {symbol}, \
                     but it is not defined"
                )
            }

            Self::ExportIsNotPublic { module, symbol } => {
                write!(
                    f,
                    "module {module} exports non-public symbol {symbol}"
                )
            }

            Self::ExportHasInternalLinkage { module, symbol } => {
                write!(
                    f,
                    "module {module} exports internally linked symbol {symbol}"
                )
            }

            Self::MissingImportDeclaration { module, symbol } => {
                write!(
                    f,
                    "module {module} imports symbol {symbol}, \
                     but no canonical import declaration was supplied"
                )
            }

            Self::ImportHasDefinitionState { module, symbol } => {
                write!(
                    f,
                    "module {module} import {symbol} has a definition state"
                )
            }

            Self::ImportHasInternalLinkage { module, symbol } => {
                write!(
                    f,
                    "module {module} imports internally linked symbol {symbol}"
                )
            }

            Self::UnresolvedImport {
                module,
                symbol,
                name,
            } => {
                write!(
                    f,
                    "module {module} has unresolved import {symbol} \
                     ('{name}')"
                )
            }

            Self::SelfProvidedImport {
                module,
                symbol,
                name,
            } => {
                write!(
                    f,
                    "module {module} imports its own symbol {symbol} ('{name}')"
                )
            }

            Self::ImportProviderNotPublic {
                module,
                symbol,
                provider,
                name,
            } => {
                write!(
                    f,
                    "module {module} imports '{name}', but provider \
                     '{provider}' is not public"
                )
            }

            Self::ImportProviderNotExternallyLinked {
                module,
                symbol,
                provider,
                name,
            } => {
                write!(
                    f,
                    "module {module} imports '{name}', but provider \
                     '{provider}' has internal linkage"
                )
            }

            Self::ImportProviderNotDefined {
                module,
                symbol,
                provider,
                name,
            } => {
                write!(
                    f,
                    "module {module} imports '{name}', but provider \
                     '{provider}' is not defined"
                )
            }

            Self::ImportProviderDidNotExport {
                module,
                symbol,
                provider,
                provider_symbol,
                name,
            } => {
                write!(
                    f,
                    "module {module} imports '{name}', but provider \
                     '{provider}' did not export symbol {provider_symbol}"
                )
            }

            Self::ImportKindMismatch {
                module,
                symbol,
                name,
            } => {
                write!(
                    f,
                    "module {module} imports '{name}' with an incompatible \
                     symbol kind (symbol {symbol})"
                )
            }

            Self::SymbolLink(error) => {
                write!(f, "symbol-linking failure: {error}")
            }

            Self::CounterOverflow => {
                f.write_str("composition counter overflow")
            }

            Self::IndexCorruption => {
                f.write_str("module-composition index is inconsistent")
            }
        }
    }
}

impl Error for CompositionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidModule(error) => Some(error),
            Self::SymbolLink(error) => Some(error),
            _ => None,
        }
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::zuir::ids::ModuleId;
    use crate::zuir::ir::module::ZuirModule;

    fn module_id(value: u128) -> ModuleId {
        ModuleId::new(value).expect("valid non-zero module ID")
    }

    fn unit(value: &str) -> LinkUnitId {
        LinkUnitId::new(value).expect("valid compilation-unit ID")
    }

    fn module(id: u128, name: &str) -> ZuirModule<()> {
        ZuirModule::named(module_id(id), name)
            .expect("valid module")
    }

    fn input(id: u128, name: &str, unit_name: &str) -> CompositionUnit<()> {
        CompositionUnit::new(
            unit(unit_name),
            module(id, name),
            std::iter::empty(),
        )
    }

    #[test]
    fn composes_an_empty_module_set() {
        let result = ModuleComposition::<()>::compose(
            std::iter::empty(),
            SymbolIndexBudget::unlimited(),
            CompositionPolicy::strict(),
        )
        .expect("empty composition is valid");

        assert!(result.is_empty());
        assert_eq!(result.module_count(), 0);
        assert_eq!(result.function_count(), 0);
        assert_eq!(result.symbol_count(), 0);
    }

    #[test]
    fn composes_multiple_modules() {
        let result = ModuleComposition::compose(
            vec![
                input(1, "core.math", "unit.math"),
                input(2, "core.data", "unit.data"),
            ],
            SymbolIndexBudget::unlimited(),
            CompositionPolicy::strict(),
        )
        .expect("distinct modules should compose");

        assert_eq!(result.module_count(), 2);
        assert!(result.module_by_name("core.math").is_some());
        assert!(result.module_by_name("core.data").is_some());
    }

    #[test]
    fn rejects_duplicate_module_ids() {
        let result = ModuleComposition::compose(
            vec![
                input(1, "core.math", "unit.math"),
                input(1, "core.data", "unit.data"),
            ],
            SymbolIndexBudget::unlimited(),
            CompositionPolicy::strict(),
        );

        assert!(matches!(
            result,
            Err(CompositionError::DuplicateModuleId { .. })
        ));
    }

    #[test]
    fn rejects_duplicate_module_names() {
        let result = ModuleComposition::compose(
            vec![
                input(1, "core.math", "unit.math"),
                input(2, "core.math", "unit.math.second"),
            ],
            SymbolIndexBudget::unlimited(),
            CompositionPolicy::strict(),
        );

        assert!(matches!(
            result,
            Err(CompositionError::DuplicateModuleName { .. })
        ));
    }

    #[test]
    fn rejects_duplicate_compilation_units() {
        let result = ModuleComposition::compose(
            vec![
                input(1, "core.math", "shared.unit"),
                input(2, "core.data", "shared.unit"),
            ],
            SymbolIndexBudget::unlimited(),
            CompositionPolicy::strict(),
        );

        assert!(matches!(
            result,
            Err(CompositionError::DuplicateCompilationUnit { .. })
        ));
    }

    #[test]
    fn module_lookup_is_deterministic() {
        let result = ModuleComposition::compose(
            vec![
                input(20, "core.second", "unit.second"),
                input(10, "core.first", "unit.first"),
            ],
            SymbolIndexBudget::unlimited(),
            CompositionPolicy::strict(),
        )
        .expect("composition should succeed");

        let ids: Vec<_> = result.modules().map(|(id, _)| id).collect();

        assert_eq!(ids, vec![module_id(10), module_id(20)]);
    }

    #[test]
    fn composition_can_be_revalidated() {
        let result = ModuleComposition::compose(
            vec![
                input(1, "core.math", "unit.math"),
                input(2, "core.data", "unit.data"),
            ],
            SymbolIndexBudget::unlimited(),
            CompositionPolicy::strict(),
        )
        .expect("composition should succeed");

        assert!(result.validate().is_ok());
    }

    #[test]
    fn composition_preserves_module_boundaries() {
        let result = ModuleComposition::compose(
            vec![
                input(1, "core.math", "unit.math"),
                input(2, "core.data", "unit.data"),
            ],
            SymbolIndexBudget::unlimited(),
            CompositionPolicy::strict(),
        )
        .expect("composition should succeed");

        assert!(result.module(module_id(1)).is_some());
        assert!(result.module(module_id(2)).is_some());

        assert_ne!(
            result.module(module_id(1)).map(ZuirModule::id),
            result.module(module_id(2)).map(ZuirModule::id),
        );
    }
}