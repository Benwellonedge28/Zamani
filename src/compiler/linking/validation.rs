
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani Compiler — Link-Time Validation
//!
//! Path: `src/compiler/linking/validation.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! ## PURPOSE
//!
//! Validate the consistency of independently compiled linking inputs.
//!
//! This module checks:
//! - canonical ZUIR module-local invariants;
//! - module and compilation-unit identity associations;
//! - canonical symbol ownership;
//! - module import declarations;
//! - module export declarations;
//! - external symbol resolution;
//! - provider visibility and export status;
//! - relocation references to canonical symbols;
//! - symbol-index internal consistency.
//!
//! ## OWNERSHIP
//!
//! This module owns cross-input link validation.
//!
//! It does not own:
//! - source parsing or name resolution;
//! - semantic type checking;
//! - ABI compatibility;
//! - canonical symbol identity allocation;
//! - symbol-index construction;
//! - duplicate strong-definition selection;
//! - relocation arithmetic or patching;
//! - object-file parsing;
//! - target-specific relocation encoding;
//! - section placement;
//! - physical resource allocation;
//! - runtime execution.
//!
//! ## CANONICAL INTEGRATION
//!
//! - `linking/symbols.rs` owns `LinkUnitId`, `SymbolDefinition`,
//!   `SymbolIndex`, and `SymbolLinkError`.
//! - `linking/relocation.rs` owns `Relocation` and `RelocationError`.
//! - `zuir/values/symbols.rs` owns canonical `Symbol` metadata.
//! - `zuir/ir/module.rs` owns canonical module imports and exports.
//! - semantic analysis owns type and source-level access validation.
//! - ABI verification owns calling convention and layout compatibility.
//!
//! ## SCALABILITY
//!
//! No fixed module, symbol, import, export, relocation, architecture,
//! device, or hardware-count ceiling is imposed.
//!
//! Explicit validation policies are operational choices, not language
//! semantic limits.
//!
//! Rust collections and the process address space remain finite.
//! Allocation failure and operating-system resource exhaustion are
//! implementation/resource failures, not language-level maximums.
//!
//! ## TRANSACTIONAL BEHAVIOR
//!
//! Validation does not mutate modules, symbols, relocations, or indexes.
//! A failed validation never leaves partially modified linking state.
//!
//! ## DETERMINISM
//!
//! Validation traverses caller-provided modules in their supplied order.
//! The report preserves that order. The canonical symbol index supplies
//! deterministic symbol resolution.
//!
//! ## TRUST BOUNDARY
//!
//! All decoded or externally supplied modules and symbol indexes must be
//! validated before they are trusted by later linking stages.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use crate::compiler::linking::relocation::{
    Relocation,
    RelocationError,
};

use crate::compiler::linking::symbols::{
    LinkUnitId,
    SymbolDefinition,
    SymbolIndex,
    SymbolLinkError,
};

use crate::zuir::ids::{ModuleId, SymbolId};

use crate::zuir::ir::module::{
    ModuleError,
    ZuirModule,
};

use crate::zuir::values::symbols::{
    SymbolKind,
    SymbolLinkage,
    SymbolState,
    SymbolVisibility,
};

// ============================================================================
// API
// ============================================================================

/// Public API version of the link validation subsystem.
pub const LINK_VALIDATION_API_VERSION: &str = "1.0.0";

/// Result type used by link validation.
pub type LinkValidationResult<T> = Result<T, LinkValidationError>;

// ============================================================================
// POLICY
// ============================================================================

/// Policy for resolving imports and external providers.
///
/// The default policy is deliberately strict:
///
/// - unresolved imports are errors;
/// - external definitions without an included ZUIR provider module
///   are rejected.
///
/// Runtime-provided symbols must be explicitly permitted by the caller.
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LinkValidationPolicy {
    /// Permit unresolved external imports.
    ///
    /// This is appropriate only when a later, explicitly identified
    /// dynamic-linking or runtime-provider stage is responsible for
    /// resolving them.
    pub allow_unresolved_external_imports: bool,

    /// Permit external definitions whose provider module is not part
    /// of the current set of ZUIR modules.
    ///
    /// This supports explicitly managed runtime, operating-system,
    /// foreign-function, or dynamic-library providers.
    ///
    /// It does not establish that such a provider actually exists.
    pub allow_external_providers: bool,

    /// Require every canonical symbol's optional owner_module field
    /// to agree with the ZUIR module associated with its link unit.
    ///
    /// When true, a missing owner_module is also rejected.
    pub require_symbol_owner_module: bool,
}

impl Default for LinkValidationPolicy {
    fn default() -> Self {
        Self {
            allow_unresolved_external_imports: false,
            allow_external_providers: false,
            require_symbol_owner_module: false,
        }
    }
}

impl LinkValidationPolicy {
    /// Strict policy suitable for ordinary static linking.
    pub const fn strict() -> Self {
        Self {
            allow_unresolved_external_imports: false,
            allow_external_providers: false,
            require_symbol_owner_module: false,
        }
    }

    /// Policy for a link stage that intentionally supports external
    /// runtime or dynamic-library providers.
    ///
    /// The caller remains responsible for validating the external
    /// provider contract at the appropriate later stage.
    pub const fn with_external_providers() -> Self {
        Self {
            allow_unresolved_external_imports: true,
            allow_external_providers: true,
            require_symbol_owner_module: false,
        }
    }

    /// Require explicit canonical symbol ownership metadata.
    pub const fn requiring_symbol_owners(mut self) -> Self {
        self.require_symbol_owner_module = true;
        self
    }
}

// ============================================================================
// INPUT
// ============================================================================

/// A canonical ZUIR module associated with its independent link unit.
///
/// The `LinkUnitId` is deliberately supplied by the caller. It must not
/// be inferred from `ModuleId`, because independently compiled inputs may
/// use separate ID allocators and may reuse numeric module identities.
#[derive(Debug, Clone, Copy)]
pub struct LinkValidationModule<'a, F> {
    unit: &'a LinkUnitId,
    module: &'a ZuirModule<F>,
}

impl<'a, F> LinkValidationModule<'a, F> {
    /// Construct a validation input.
    #[must_use]
    pub const fn new(
        unit: &'a LinkUnitId,
        module: &'a ZuirModule<F>,
    ) -> Self {
        Self { unit, module }
    }

    /// Return the compilation-unit identity.
    #[must_use]
    pub const fn unit(&self) -> &'a LinkUnitId {
        self.unit
    }

    /// Return the canonical module.
    #[must_use]
    pub const fn module(&self) -> &'a ZuirModule<F> {
        self.module
    }
}

// ============================================================================
// REPORT
// ============================================================================

/// Summary of a successful link validation.
///
/// Counts are descriptive observations of the validated input. They are
/// not fixed capacities or language limits.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct LinkValidationReport {
    /// Number of validated ZUIR modules.
    pub module_count: usize,

    /// Number of validated compilation units.
    pub unit_count: usize,

    /// Number of symbol definitions in the canonical index.
    pub symbol_count: usize,

    /// Number of module import references checked.
    pub import_count: usize,

    /// Number of module export references checked.
    pub export_count: usize,

    /// Number of relocation records checked.
    pub relocation_count: usize,

    /// Unresolved imports intentionally accepted by policy.
    ///
    /// Each entry contains the importing unit, symbol identity, and
    /// canonical qualified symbol name.
    pub permitted_unresolved_imports: Vec<UnresolvedImport>,
}

/// An unresolved external import accepted by explicit policy.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct UnresolvedImport {
    /// Importing compilation unit.
    pub unit: LinkUnitId,

    /// Imported symbol identity in the importing unit.
    pub symbol_id: SymbolId,

    /// Canonical qualified name.
    pub name: String,
}

// ============================================================================
// ERRORS
// ============================================================================

/// Structured link validation failures.
#[derive(Debug)]
#[non_exhaustive]
pub enum LinkValidationError {
    /// The canonical symbol index is internally inconsistent.
    InvalidSymbolIndex(SymbolLinkError),

    /// A module violates its canonical local invariants.
    InvalidModule {
        /// Compilation unit.
        unit: LinkUnitId,

        /// Canonical module identity.
        module_id: ModuleId,

        /// Underlying module validation error.
        source: ModuleError,
    },

    /// One link unit was assigned to multiple modules.
    DuplicateLinkUnit {
        /// Duplicated compilation unit.
        unit: LinkUnitId,
    },

    /// A canonical module identity appears more than once.
    DuplicateModuleId {
        /// Duplicated module identity.
        module_id: ModuleId,
    },

    /// A canonical module name appears more than once.
    DuplicateModuleName {
        /// Duplicated module name.
        name: String,
    },

    /// A symbol-index unit has no corresponding input module and is
    /// not permitted as an external provider.
    UnknownSymbolUnit {
        /// Unit containing the symbol.
        unit: LinkUnitId,
    },

    /// A symbol's declared owner does not match its associated module.
    SymbolOwnerMismatch {
        /// Compilation unit.
        unit: LinkUnitId,

        /// Symbol identity.
        symbol_id: SymbolId,

        /// Expected module identity.
        expected: ModuleId,

        /// Actual module identity.
        actual: ModuleId,
    },

    /// A symbol is missing required owner-module metadata.
    MissingSymbolOwner {
        /// Compilation unit.
        unit: LinkUnitId,

        /// Symbol identity.
        symbol_id: SymbolId,
    },

    /// A module import has no matching local canonical symbol record.
    MissingImportSymbol {
        /// Importing compilation unit.
        unit: LinkUnitId,

        /// Imported symbol identity.
        symbol_id: SymbolId,
    },

    /// An import record has an invalid declaration state.
    InvalidImportState {
        /// Importing compilation unit.
        unit: LinkUnitId,

        /// Imported symbol identity.
        symbol_id: SymbolId,

        /// Actual state.
        state: SymbolState,
    },

    /// An import record has internal linkage and cannot be resolved
    /// through the external symbol namespace.
    InternalImport {
        /// Importing compilation unit.
        unit: LinkUnitId,

        /// Imported symbol identity.
        symbol_id: SymbolId,
    },

    /// An external import has no matching definition.
    UnresolvedImport {
        /// Importing compilation unit.
        unit: LinkUnitId,

        /// Imported symbol identity.
        symbol_id: SymbolId,

        /// Canonical name.
        name: String,
    },

    /// The selected external provider is not publicly visible.
    ProviderNotPublic {
        /// Importing compilation unit.
        importing_unit: LinkUnitId,

        /// Provider compilation unit.
        provider_unit: LinkUnitId,

        /// Canonical name.
        name: String,
    },

    /// The provider's symbol is not exported by its ZUIR module.
    ProviderDoesNotExport {
        /// Provider compilation unit.
        provider_unit: LinkUnitId,

        /// Provider symbol identity.
        symbol_id: SymbolId,

        /// Canonical name.
        name: String,
    },

    /// An imported symbol's kind differs from its provider's kind.
    ImportKindMismatch {
        /// Importing compilation unit.
        importing_unit: LinkUnitId,

        /// Canonical name.
        name: String,

        /// Expected kind.
        expected: SymbolKind,

        /// Actual provider kind.
        actual: SymbolKind,
    },

    /// A module export has no matching local canonical symbol record.
    MissingExportSymbol {
        /// Exporting compilation unit.
        unit: LinkUnitId,

        /// Exported symbol identity.
        symbol_id: SymbolId,
    },

    /// A module attempts to export a declaration that is not a definition.
    ExportIsNotDefined {
        /// Exporting compilation unit.
        unit: LinkUnitId,

        /// Exported symbol identity.
        symbol_id: SymbolId,

        /// Actual declaration state.
        state: SymbolState,
    },

    /// A module attempts to export an internally linked symbol.
    ExportHasInternalLinkage {
        /// Exporting compilation unit.
        unit: LinkUnitId,

        /// Exported symbol identity.
        symbol_id: SymbolId,
    },

    /// A module attempts to export a non-public symbol.
    ExportIsNotPublic {
        /// Exporting compilation unit.
        unit: LinkUnitId,

        /// Exported symbol identity.
        symbol_id: SymbolId,
    },

    /// A relocation record violates its own structural invariants.
    InvalidRelocation(RelocationError),

    /// A relocation references a symbol that is absent from its unit.
    InvalidRelocationSymbol {
        /// Relocation's input unit.
        unit: LinkUnitId,

        /// Referenced symbol identity.
        symbol_id: SymbolId,

        /// Underlying symbol resolution error.
        source: SymbolLinkError,
    },

    /// An internal counter overflowed.
    CounterOverflow,
}

impl fmt::Display for LinkValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidSymbolIndex(error) => {
                write!(f, "invalid link symbol index: {error}")
            }

            Self::InvalidModule {
                unit,
                module_id,
                source,
            } => {
                write!(
                    f,
                    "invalid module {module_id:?} in unit '{unit}': {source}"
                )
            }

            Self::DuplicateLinkUnit { unit } => {
                write!(f, "link unit '{unit}' is associated with multiple modules")
            }

            Self::DuplicateModuleId { module_id } => {
                write!(f, "duplicate module identity {module_id:?}")
            }

            Self::DuplicateModuleName { name } => {
                write!(f, "duplicate module name '{name}'")
            }

            Self::UnknownSymbolUnit { unit } => {
                write!(f, "symbol index contains unknown unit '{unit}'")
            }

            Self::SymbolOwnerMismatch {
                unit,
                symbol_id,
                expected,
                actual,
            } => {
                write!(
                    f,
                    "symbol {symbol_id:?} in unit '{unit}' belongs to \
                     module {actual:?}, expected {expected:?}"
                )
            }

            Self::MissingSymbolOwner { unit, symbol_id } => {
                write!(
                    f,
                    "symbol {symbol_id:?} in unit '{unit}' has no owner module"
                )
            }

            Self::MissingImportSymbol { unit, symbol_id } => {
                write!(
                    f,
                    "module in unit '{unit}' imports missing symbol {symbol_id:?}"
                )
            }

            Self::InvalidImportState {
                unit,
                symbol_id,
                state,
            } => {
                write!(
                    f,
                    "import {symbol_id:?} in unit '{unit}' has invalid state \
                     {state:?}"
                )
            }

            Self::InternalImport { unit, symbol_id } => {
                write!(
                    f,
                    "unit '{unit}' imports internal symbol {symbol_id:?}"
                )
            }

            Self::UnresolvedImport {
                unit,
                symbol_id,
                name,
            } => {
                write!(
                    f,
                    "unresolved import '{name}' ({symbol_id:?}) in unit '{unit}'"
                )
            }

            Self::ProviderNotPublic {
                importing_unit,
                provider_unit,
                name,
            } => {
                write!(
                    f,
                    "provider '{provider_unit}' for '{name}' imported by \
                     '{importing_unit}' is not public"
                )
            }

            Self::ProviderDoesNotExport {
                provider_unit,
                symbol_id,
                name,
            } => {
                write!(
                    f,
                    "provider '{provider_unit}' does not export '{name}' \
                     ({symbol_id:?})"
                )
            }

            Self::ImportKindMismatch {
                importing_unit,
                name,
                expected,
                actual,
            } => {
                write!(
                    f,
                    "import '{name}' in '{importing_unit}' expects \
                     {expected:?}, provider supplies {actual:?}"
                )
            }

            Self::MissingExportSymbol { unit, symbol_id } => {
                write!(
                    f,
                    "module in unit '{unit}' exports missing symbol {symbol_id:?}"
                )
            }

            Self::ExportIsNotDefined {
                unit,
                symbol_id,
                state,
            } => {
                write!(
                    f,
                    "export {symbol_id:?} in unit '{unit}' is not a definition \
                     (state {state:?})"
                )
            }

            Self::ExportHasInternalLinkage { unit, symbol_id } => {
                write!(
                    f,
                    "unit '{unit}' exports internally linked symbol {symbol_id:?}"
                )
            }

            Self::ExportIsNotPublic { unit, symbol_id } => {
                write!(
                    f,
                    "unit '{unit}' exports non-public symbol {symbol_id:?}"
                )
            }

            Self::InvalidRelocation(error) => {
                write!(f, "invalid relocation: {error}")
            }

            Self::InvalidRelocationSymbol {
                unit,
                symbol_id,
                source,
            } => {
                write!(
                    f,
                    "relocation in unit '{unit}' references invalid \
                     symbol {symbol_id:?}: {source}"
                )
            }

            Self::CounterOverflow => {
                write!(f, "link validation counter overflow")
            }
        }
    }
}

impl Error for LinkValidationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidSymbolIndex(error) => Some(error),
            Self::InvalidModule { source, .. } => Some(source),
            Self::InvalidRelocation(error) => Some(error),
            Self::InvalidRelocationSymbol { source, .. } => Some(source),
            _ => None,
        }
    }
}

// ============================================================================
// VALIDATOR
// ============================================================================

/// Validate canonical modules, symbols, imports, and exports.
///
/// The caller must supply one unique `LinkUnitId` for each input module.
/// Symbols in the index are associated with those same unit identities.
///
/// This function does not mutate any input.
///
/// `F` is the canonical function payload used by the ZUIR module.
/// The validator deliberately does not inspect or redefine function bodies.
pub fn validate_modules<'a, F, I>(
    modules: I,
    symbols: &SymbolIndex,
    policy: LinkValidationPolicy,
) -> LinkValidationResult<LinkValidationReport>
where
    I: IntoIterator<Item = LinkValidationModule<'a, F>>,
{
    symbols
        .verify_consistency()
        .map_err(LinkValidationError::InvalidSymbolIndex)?;

    let mut report = LinkValidationReport::default();

    let mut modules_by_unit: BTreeMap<
        LinkUnitId,
        &'a ZuirModule<F>,
    > = BTreeMap::new();

    let mut modules_by_id: BTreeSet<ModuleId> = BTreeSet::new();

    let mut module_names: BTreeSet<String> = BTreeSet::new();

    // ------------------------------------------------------------
    // Phase 1: Validate and index module inputs.
    // ------------------------------------------------------------

    for input in modules {
        let unit = input.unit;
        let module = input.module;

        module
            .validate()
            .map_err(|source| LinkValidationError::InvalidModule {
                unit: unit.clone(),
                module_id: module.id(),
                source,
            })?;

        if !modules_by_id.insert(module.id()) {
            return Err(LinkValidationError::DuplicateModuleId {
                module_id: module.id(),
            });
        }

        let name = module.name().as_str().to_owned();

        if !module_names.insert(name.clone()) {
            return Err(LinkValidationError::DuplicateModuleName {
                name,
            });
        }

        if modules_by_unit
            .insert(unit.clone(), module)
            .is_some()
        {
            return Err(LinkValidationError::DuplicateLinkUnit {
                unit: unit.clone(),
            });
        }

        report.module_count = report
            .module_count
            .checked_add(1)
            .ok_or(LinkValidationError::CounterOverflow)?;
    }

    report.unit_count = modules_by_unit.len();
    report.symbol_count = symbols.len();

    // ------------------------------------------------------------
    // Phase 2: Validate symbol ownership.
    // ------------------------------------------------------------

    for definition in symbols.definitions() {
        let Some(module) = modules_by_unit.get(definition.unit()) else {
            if policy.allow_external_providers {
                continue;
            }

            return Err(LinkValidationError::UnknownSymbolUnit {
                unit: definition.unit().clone(),
            });
        };

        let symbol = definition.symbol();

        match symbol.owner_module {
            Some(owner) if owner != module.id() => {
                return Err(LinkValidationError::SymbolOwnerMismatch {
                    unit: definition.unit().clone(),
                    symbol_id: definition.id(),
                    expected: module.id(),
                    actual: owner,
                });
            }

            None if policy.require_symbol_owner_module => {
                return Err(LinkValidationError::MissingSymbolOwner {
                    unit: definition.unit().clone(),
                    symbol_id: definition.id(),
                });
            }

            _ => {}
        }
    }

    // ------------------------------------------------------------
    // Phase 3: Validate module exports.
    // ------------------------------------------------------------

    for (unit, module) in &modules_by_unit {
        for symbol_id in module.exports() {
            let definition = symbols
                .get(unit, symbol_id)
                .ok_or_else(|| LinkValidationError::MissingExportSymbol {
                    unit: unit.clone(),
                    symbol_id,
                })?;

            let symbol = definition.symbol();

            if !matches!(
                definition.state(),
                SymbolState::Defined | SymbolState::ExternallyDefined
            ) {
                return Err(LinkValidationError::ExportIsNotDefined {
                    unit: unit.clone(),
                    symbol_id,
                    state: definition.state(),
                });
            }

            if definition.linkage() == SymbolLinkage::Internal {
                return Err(LinkValidationError::ExportHasInternalLinkage {
                    unit: unit.clone(),
                    symbol_id,
                });
            }

            if symbol.visibility != SymbolVisibility::Public {
                return Err(LinkValidationError::ExportIsNotPublic {
                    unit: unit.clone(),
                    symbol_id,
                });
            }

            report.export_count = report
                .export_count
                .checked_add(1)
                .ok_or(LinkValidationError::CounterOverflow)?;
        }
    }

    // ------------------------------------------------------------
    // Phase 4: Validate imports and external providers.
    // ------------------------------------------------------------

    for (unit, module) in &modules_by_unit {
        for symbol_id in module.imports() {
            let import = symbols
                .get(unit, symbol_id)
                .ok_or_else(|| LinkValidationError::MissingImportSymbol {
                    unit: unit.clone(),
                    symbol_id,
                })?;

            if !matches!(
                import.state(),
                SymbolState::Declared | SymbolState::Imported
            ) {
                return Err(LinkValidationError::InvalidImportState {
                    unit: unit.clone(),
                    symbol_id,
                    state: import.state(),
                });
            }

            if import.linkage() == SymbolLinkage::Internal {
                return Err(LinkValidationError::InternalImport {
                    unit: unit.clone(),
                    symbol_id,
                });
            }

            report.import_count = report
                .import_count
                .checked_add(1)
                .ok_or(LinkValidationError::CounterOverflow)?;

            let name = import.name();

            let provider = symbols
                .resolve_external(&name)
                .map_err(LinkValidationError::InvalidSymbolIndex)?;

            let Some(provider) = provider else {
                if !policy.allow_unresolved_external_imports {
                    return Err(LinkValidationError::UnresolvedImport {
                        unit: unit.clone(),
                        symbol_id,
                        name,
                    });
                }

                report.permitted_unresolved_imports.push(
                    UnresolvedImport {
                        unit: unit.clone(),
                        symbol_id,
                        name,
                    },
                );

                continue;
            };

            // A declaration cannot satisfy an import. The symbol index
            // normally returns definitions only, but retain this check
            // at the validation trust boundary.
            if !matches!(
                provider.state(),
                SymbolState::Defined | SymbolState::ExternallyDefined
            ) {
                return Err(LinkValidationError::UnresolvedImport {
                    unit: unit.clone(),
                    symbol_id,
                    name,
                });
            }

            if provider.symbol().visibility != SymbolVisibility::Public {
                return Err(LinkValidationError::ProviderNotPublic {
                    importing_unit: unit.clone(),
                    provider_unit: provider.unit().clone(),
                    name,
                });
            }

            if import.symbol().kind != provider.symbol().kind {
                return Err(LinkValidationError::ImportKindMismatch {
                    importing_unit: unit.clone(),
                    name,
                    expected: import.symbol().kind,
                    actual: provider.symbol().kind,
                });
            }

            // A provider in an included ZUIR module must explicitly export
            // the symbol. A provider without an included module is treated
            // as an external provider only when policy permits it.
            match modules_by_unit.get(provider.unit()) {
                Some(provider_module) => {
                    if !provider_module.has_export(provider.id()) {
                        return Err(
                            LinkValidationError::ProviderDoesNotExport {
                                provider_unit: provider.unit().clone(),
                                symbol_id: provider.id(),
                                name,
                            },
                        );
                    }
                }

                None if policy.allow_external_providers => {}

                None => {
                    return Err(LinkValidationError::UnknownSymbolUnit {
                        unit: provider.unit().clone(),
                    });
                }
            }
        }
    }

    // ------------------------------------------------------------
    // Phase 5: Validate declarations not represented in module
    // import sets.
    //
    // The canonical symbol index can contain standalone external
    // declarations. They must not silently disappear from validation.
    // ------------------------------------------------------------

    if !policy.allow_unresolved_external_imports {
        let unresolved = symbols
            .undefined_external_symbols()
            .map_err(LinkValidationError::InvalidSymbolIndex)?;

        if let Some(definition) = unresolved.first() {
            return Err(LinkValidationError::UnresolvedImport {
                unit: definition.unit().clone(),
                symbol_id: definition.id(),
                name: definition.name(),
            });
        }
    }

    Ok(report)
}

// ============================================================================
// RELOCATION VALIDATION
// ============================================================================

/// Validate relocation records against the canonical symbol index.
///
/// Relocation arithmetic, target legality, section bounds, final addresses,
/// and relocation patching remain the responsibility of `relocation.rs`
/// and the target-specific linker.
///
/// This function verifies that every relocation:
/// - satisfies its local structural invariants;
/// - refers to a symbol present in its declared input unit;
/// - satisfies the expected symbol kind, when supplied.
pub fn validate_relocations<'a, I>(
    relocations: I,
    symbols: &SymbolIndex,
) -> LinkValidationResult<usize>
where
    I: IntoIterator<Item = &'a Relocation>,
{
    symbols
        .verify_consistency()
        .map_err(LinkValidationError::InvalidSymbolIndex)?;

    let mut count = 0usize;

    for relocation in relocations {
        relocation
            .validate()
            .map_err(LinkValidationError::InvalidRelocation)?;

        symbols
            .resolve_reference(relocation.unit(), &relocation.symbol())
            .map_err(|source| {
                LinkValidationError::InvalidRelocationSymbol {
                    unit: relocation.unit().clone(),
                    symbol_id: relocation.symbol().id,
                    source,
                }
            })?;

        count = count
            .checked_add(1)
            .ok_or(LinkValidationError::CounterOverflow)?;
    }

    Ok(count)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::compiler::linking::symbols::{
        SymbolDefinition,
        SymbolIndex,
    };

    use crate::zuir::ids::SymbolId;

    use crate::zuir::schema::types::QualifiedName;

    use crate::zuir::values::symbols::{
        Symbol,
        SymbolKind,
    };

    fn unit(name: &str) -> LinkUnitId {
        LinkUnitId::new(name).expect("valid link unit")
    }

    fn symbol(
        id: u128,
        name: &str,
        linkage: SymbolLinkage,
        state: SymbolState,
    ) -> Symbol {
        let id = SymbolId::new(id).expect("nonzero symbol identity");

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
        .expect("valid canonical symbol")
    }

    fn module(
        id: u128,
        name: &str,
    ) -> ZuirModule<()> {
        let id = ModuleId::new(id).expect("nonzero module identity");

        ZuirModule::named(id, name)
            .expect("valid module")
    }

    #[test]
    fn validates_empty_inputs() {
        let symbols = SymbolIndex::new();

        let report = validate_modules(
            std::iter::empty::<LinkValidationModule<'_, ()>>(),
            &symbols,
            LinkValidationPolicy::strict(),
        )
        .expect("empty input is valid");

        assert_eq!(report.module_count, 0);
        assert_eq!(report.symbol_count, 0);
    }

    #[test]
    fn validates_module_without_symbols() {
        let unit = unit("main-unit");
        let module = module(1, "main");
        let symbols = SymbolIndex::new();

        let report = validate_modules(
            [LinkValidationModule::new(&unit, &module)],
            &symbols,
            LinkValidationPolicy::strict(),
        )
        .expect("valid module");

        assert_eq!(report.module_count, 1);
        assert_eq!(report.unit_count, 1);
    }

    #[test]
    fn rejects_duplicate_link_units() {
        let unit = unit("shared-unit");

        let first = module(1, "first");
        let second = module(2, "second");

        let symbols = SymbolIndex::new();

        let result = validate_modules(
            [
                LinkValidationModule::new(&unit, &first),
                LinkValidationModule::new(&unit, &second),
            ],
            &symbols,
            LinkValidationPolicy::strict(),
        );

        assert!(matches!(
            result,
            Err(LinkValidationError::DuplicateLinkUnit { .. })
        ));
    }

    #[test]
    fn rejects_duplicate_module_ids() {
        let first_unit = unit("first-unit");
        let second_unit = unit("second-unit");

        let first = module(1, "first");
        let second = module(1, "second");

        let symbols = SymbolIndex::new();

        let result = validate_modules(
            [
                LinkValidationModule::new(&first_unit, &first),
                LinkValidationModule::new(&second_unit, &second),
            ],
            &symbols,
            LinkValidationPolicy::strict(),
        );

        assert!(matches!(
            result,
            Err(LinkValidationError::DuplicateModuleId { .. })
        ));
    }

    #[test]
    fn rejects_duplicate_module_names() {
        let first_unit = unit("first-unit");
        let second_unit = unit("second-unit");

        let first = module(1, "shared.name");
        let second = module(2, "shared.name");

        let symbols = SymbolIndex::new();

        let result = validate_modules(
            [
                LinkValidationModule::new(&first_unit, &first),
                LinkValidationModule::new(&second_unit, &second),
            ],
            &symbols,
            LinkValidationPolicy::strict(),
        );

        assert!(matches!(
            result,
            Err(LinkValidationError::DuplicateModuleName { .. })
        ));
    }

    #[test]
    fn validates_a_public_export() {
        let unit = unit("provider");
        let mut module = module(1, "provider");

        let definition = definition(
            "provider",
            10,
            "math::calculate",
            SymbolLinkage::Exported,
            SymbolState::Defined,
        );

        module.add_export(definition.id());

        let mut symbols = SymbolIndex::new();
        symbols.insert(definition).expect("insert symbol");

        let report = validate_modules(
            [LinkValidationModule::new(&unit, &module)],
            &symbols,
            LinkValidationPolicy::strict(),
        )
        .expect("valid export");

        assert_eq!(report.export_count, 1);
    }

    #[test]
    fn rejects_an_export_without_a_symbol_record() {
        let unit = unit("provider");
        let mut module = module(1, "provider");

        module.add_export(
            SymbolId::new(10).expect("nonzero symbol identity"),
        );

        let symbols = SymbolIndex::new();

        let result = validate_modules(
            [LinkValidationModule::new(&unit, &module)],
            &symbols,
            LinkValidationPolicy::strict(),
        );

        assert!(matches!(
            result,
            Err(LinkValidationError::MissingExportSymbol { .. })
        ));
    }

    #[test]
    fn rejects_unresolved_imports_by_default() {
        let unit = unit("consumer");
        let mut module = module(1, "consumer");

        let imported = definition(
            "consumer",
            20,
            "math::calculate",
            SymbolLinkage::External,
            SymbolState::Imported,
        );

        module.add_import(imported.id());

        let mut symbols = SymbolIndex::new();
        symbols.insert(imported).expect("insert import");

        let result = validate_modules(
            [LinkValidationModule::new(&unit, &module)],
            &symbols,
            LinkValidationPolicy::strict(),
        );

        assert!(matches!(
            result,
            Err(LinkValidationError::UnresolvedImport { .. })
        ));
    }

    #[test]
    fn explicitly_permitted_unresolved_imports_are_reported() {
        let unit = unit("consumer");
        let mut module = module(1, "consumer");

        let imported = definition(
            "consumer",
            20,
            "runtime::service",
            SymbolLinkage::External,
            SymbolState::Imported,
        );

        module.add_import(imported.id());

        let mut symbols = SymbolIndex::new();
        symbols.insert(imported).expect("insert import");

        let report = validate_modules(
            [LinkValidationModule::new(&unit, &module)],
            &symbols,
            LinkValidationPolicy {
                allow_unresolved_external_imports: true,
                allow_external_providers: true,
                require_symbol_owner_module: false,
            },
        )
        .expect("explicitly permitted unresolved import");

        assert_eq!(report.permitted_unresolved_imports.len(), 1);
    }

    #[test]
    fn validates_provider_export_and_import() {
        let provider_unit = unit("provider");
        let consumer_unit = unit("consumer");

        let mut provider_module = module(1, "provider");
        let mut consumer_module = module(2, "consumer");

        let provider = definition(
            "provider",
            10,
            "math::calculate",
            SymbolLinkage::Exported,
            SymbolState::Defined,
        );

        let imported = definition(
            "consumer",
            20,
            "math::calculate",
            SymbolLinkage::External,
            SymbolState::Imported,
        );

        provider_module.add_export(provider.id());
        consumer_module.add_import(imported.id());

        let mut symbols = SymbolIndex::new();

        symbols.insert(provider).expect("provider symbol");
        symbols.insert(imported).expect("import symbol");

        let report = validate_modules(
            [
                LinkValidationModule::new(
                    &provider_unit,
                    &provider_module,
                ),
                LinkValidationModule::new(
                    &consumer_unit,
                    &consumer_module,
                ),
            ],
            &symbols,
            LinkValidationPolicy::strict(),
        )
        .expect("valid provider and consumer");

        assert_eq!(report.module_count, 2);
        assert_eq!(report.import_count, 1);
        assert_eq!(report.export_count, 1);
    }

    #[test]
    fn rejects_provider_that_does_not_export_symbol() {
        let provider_unit = unit("provider");
        let consumer_unit = unit("consumer");

        let provider_module = module(1, "provider");
        let mut consumer_module = module(2, "consumer");

        let provider = definition(
            "provider",
            10,
            "math::calculate",
            SymbolLinkage::External,
            SymbolState::Defined,
        );

        let imported = definition(
            "consumer",
            20,
            "math::calculate",
            SymbolLinkage::External,
            SymbolState::Imported,
        );

        consumer_module.add_import(imported.id());

        let mut symbols = SymbolIndex::new();

        symbols.insert(provider).expect("provider symbol");
        symbols.insert(imported).expect("import symbol");

        let result = validate_modules(
            [
                LinkValidationModule::new(
                    &provider_unit,
                    &provider_module,
                ),
                LinkValidationModule::new(
                    &consumer_unit,
                    &consumer_module,
                ),
            ],
            &symbols,
            LinkValidationPolicy::strict(),
        );

        assert!(matches!(
            result,
            Err(LinkValidationError::ProviderDoesNotExport { .. })
        ));
    }

    #[test]
    fn rejects_import_kind_mismatch() {
        let provider_unit = unit("provider");
        let consumer_unit = unit("consumer");

        let mut provider_module = module(1, "provider");
        let mut consumer_module = module(2, "consumer");

        let provider = definition(
            "provider",
            10,
            "math::calculate",
            SymbolLinkage::Exported,
            SymbolState::Defined,
        );

        let imported = SymbolDefinition::new(
            consumer_unit.clone(),
            Symbol::new(
                SymbolId::new(20).expect("nonzero identity"),
                QualifiedName::parse("math::calculate")
                    .expect("valid qualified name"),
                SymbolKind::Variable,
            )
            .with_visibility(SymbolVisibility::Public)
            .with_linkage(SymbolLinkage::External)
            .with_state(SymbolState::Imported),
        )
        .expect("valid import");

        provider_module.add_export(provider.id());
        consumer_module.add_import(imported.id());

        let mut symbols = SymbolIndex::new();

        symbols.insert(provider).expect("provider");
        symbols.insert(imported).expect("import");

        let result = validate_modules(
            [
                LinkValidationModule::new(
                    &provider_unit,
                    &provider_module,
                ),
                LinkValidationModule::new(
                    &consumer_unit,
                    &consumer_module,
                ),
            ],
            &symbols,
            LinkValidationPolicy::strict(),
        );

        assert!(matches!(
            result,
            Err(LinkValidationError::ImportKindMismatch { .. })
        ));
    }
}
