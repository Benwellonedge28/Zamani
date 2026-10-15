
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Module
//!
//! Canonical path: `src/zuir/ir/module.rs`
//! Edition: Rust 2021
//! Minimum Rust baseline: Rust 1.97 or later.
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Define the canonical module-level container for ZUIR. A module owns its
//! identity, validated name, callable declarations, symbol imports and
//! exports, and descriptive metadata.
//!
//! The function payload is generic so this module can be implemented before
//! `ir/function.rs` is finalized. Once the canonical function type exists,
//! the IR facade may expose `ZuirModule<ZuirFunction>` as its standard type.
//!
//! ## AUTHORITY
//!
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/ids.rs`
//! - `src/zuir/ir/program.rs`
//! - `src/zuir/schema/metadata.rs`
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/specification/semantic-model.md`
//!
//! ## OWNS
//!
//! - Module identity and module naming.
//! - Function membership keyed by typed FunctionId.
//! - Module import/export symbol references.
//! - Descriptive, non-semantic module metadata.
//! - Deterministic collection traversal.
//! - Local membership and naming invariants.
//!
//! ## DOES_NOT_OWN
//!
//! - Source parsing, AST construction, or name resolution.
//! - Function-body, block, operation, type, or SSA semantics.
//! - Dialect registration or executable extension behavior.
//! - Global symbol resolution or link-time uniqueness.
//! - Full-program verification or target feasibility.
//! - Physical resource allocation, scheduling, or runtime execution.
//!
//! ## DEPENDENCIES
//!
//! Uses canonical typed IDs from `ids.rs` and standard-library collections.
//! Serde is used for representation persistence. It does not depend on
//! builders, verifiers, target backends, or quantum-specific identities.
//!
//! ## INTEGRATION
//!
//! `program.rs` stores modules using ModuleId keys. Insert a module using
//! `module.id()` as the key. The program container remains responsible for
//! checking duplicate module membership.
//!
//! `function.rs` supplies the eventual canonical function payload.
//! `verify/full.rs` and `verify/structural.rs` validate cross-entity
//! relationships, function bodies, references, and global invariants.
//! The symbol resolver validates import/export resolution and visibility.
//! The serializer must call `validate()` after decoding.
//!
//! The frontend grammar defines syntax; semantic analysis supplies resolved
//! identities and meaning. This module does not interpret source syntax.
//!
//! ## SCALABILITY
//!
//! Collections grow dynamically with available resources. No hardware,
//! function-count, import-count, export-count, or device limit is imposed.
//! Rust collection capacities, address space, and available memory remain
//! finite. Operational budgets must be enforced by the caller.
//!
//! ## FREEZE CRITERIA
//!
//! - Names are validated during construction and deserialization.
//! - Function IDs cannot silently overwrite existing declarations.
//! - Import/export sets are deterministic and duplicate-free.
//! - Decoded modules can be explicitly revalidated.
//! - Metadata is descriptive only.
//! - No unsafe code or hardware-specific assumptions are introduced.
//! - Unit, serialization, integration, and repository tests pass.

use core::fmt;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

use crate::zuir::ids::{FunctionId, ModuleId, SymbolId};

// =============================================================================
// Module names
// =============================================================================

/// A validated module name.
///
/// The name is a logical identifier, not a filesystem path or target symbol.
/// It is intentionally independent of any particular operating system,
/// device, runtime, or source-language naming convention.
///
/// Accepted syntax:
///
/// - One or more dot-separated segments.
/// - Each segment begins with an ASCII letter or underscore.
/// - Remaining characters are ASCII letters, digits, or underscores.
///
/// Examples: `main`, `math.linear`, `hardware.controller`.
///
/// Names are not silently normalized. In particular, whitespace, empty
/// segments, leading/trailing dots, and path separators are rejected.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(transparent)]
pub struct ModuleName(String);

impl ModuleName {
    /// Construct a module name after validating its spelling.
    pub fn new(value: impl Into<String>) -> Result<Self, ModuleError> {
        let value = value.into();

        if !is_valid_module_name(&value) {
            return Err(ModuleError::InvalidModuleName(value));
        }

        Ok(Self(value))
    }

    /// Return the canonical module name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consume the wrapper and return the validated string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }

    /// Revalidate this name at a trust boundary.
    pub fn validate(&self) -> Result<(), ModuleError> {
        if is_valid_module_name(&self.0) {
            Ok(())
        } else {
            Err(ModuleError::InvalidModuleName(self.0.clone()))
        }
    }
}

impl<'de> Deserialize<'de> for ModuleName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::new(value).map_err(serde::de::Error::custom)
    }
}

impl fmt::Display for ModuleName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

fn is_valid_module_name(value: &str) -> bool {
    if value.is_empty() || value.trim() != value {
        return false;
    }

    value.split('.').all(|segment| {
        let mut chars = segment.chars();

        let Some(first) = chars.next() else {
            return false;
        };

        if !(first.is_ascii_alphabetic() || first == '_') {
            return false;
        }

        chars.all(|ch| ch.is_ascii_alphanumeric() || ch == '_')
    })
}

// =============================================================================
// Module errors
// =============================================================================

/// Errors returned by module-local construction and mutation.
///
/// These errors describe module-container invariants. Conversion into the
/// repository-wide `ZuirError` belongs at the established public API boundary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModuleError {
    /// The module name is malformed.
    InvalidModuleName(String),

    /// A function with this identity already exists.
    DuplicateFunction(FunctionId),

    /// A function with this identity does not exist.
    UnknownFunction(FunctionId),

    /// An import or export set contains a forbidden identity.
    InvalidSymbolId(SymbolId),

    /// A metadata key is empty or contains surrounding whitespace.
    InvalidMetadataKey(String),
}

impl fmt::Display for ModuleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidModuleName(name) => {
                write!(f, "invalid module name: {name:?}")
            }
            Self::DuplicateFunction(id) => {
                write!(f, "function {id} already belongs to this module")
            }
            Self::UnknownFunction(id) => {
                write!(f, "function {id} does not belong to this module")
            }
            Self::InvalidSymbolId(id) => {
                write!(f, "invalid symbol identity: {id}")
            }
            Self::InvalidMetadataKey(key) => {
                write!(f, "invalid module metadata key: {key:?}")
            }
        }
    }
}

impl std::error::Error for ModuleError {}

// =============================================================================
// Module container
// =============================================================================

/// The canonical ZUIR module container.
///
/// `F` is the canonical function representation supplied by `ir/function.rs`.
/// This generic parameter avoids introducing a competing function schema.
///
/// Functions are keyed by typed identity, with deterministic traversal through
/// `BTreeMap`. Insertion never silently replaces an existing function.
///
/// Imports and exports reference resolved symbol identities. Their actual
/// resolution, visibility, linkage, and cross-module compatibility are
/// checked by the symbol resolver and authoritative verifier.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(bound(
    serialize = "F: Serialize",
    deserialize = "F: Deserialize<'de>"
))]
pub struct ZuirModule<F> {
    id: ModuleId,
    name: ModuleName,

    /// Function declarations owned by this module.
    functions: BTreeMap<FunctionId, F>,

    /// Resolved symbols required from other modules or linked components.
    imports: BTreeSet<SymbolId>,

    /// Symbols this module makes available to permitted consumers.
    exports: BTreeSet<SymbolId>,

    /// Descriptive, non-semantic metadata.
    metadata: BTreeMap<String, String>,
}

impl<F> ZuirModule<F> {
    /// Create an empty module with a typed identity and validated name.
    ///
    /// The caller owns identity allocation and must ensure that the ID is
    /// unique in the containing program's identity scope.
    pub fn new(
        id: ModuleId,
        name: ModuleName,
    ) -> Self {
        Self {
            id,
            name,
            functions: BTreeMap::new(),
            imports: BTreeSet::new(),
            exports: BTreeSet::new(),
            metadata: BTreeMap::new(),
        }
    }

    /// Construct a module after validating a string name.
    pub fn named(
        id: ModuleId,
        name: impl Into<String>,
    ) -> Result<Self, ModuleError> {
        Ok(Self::new(id, ModuleName::new(name)?))
    }

    /// Return the module identity.
    #[must_use]
    pub const fn id(&self) -> ModuleId {
        self.id
    }

    /// Return the validated module name.
    #[must_use]
    pub fn name(&self) -> &ModuleName {
        &self.name
    }

    /// Return the number of functions owned by this module.
    #[must_use]
    pub fn function_count(&self) -> usize {
        self.functions.len()
    }

    /// Return whether the module has no functions.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.functions.is_empty()
    }

    /// Check whether a function identity belongs to this module.
    #[must_use]
    pub fn contains_function(&self, id: FunctionId) -> bool {
        self.functions.contains_key(&id)
    }

    /// Insert a function without replacing an existing function.
    ///
    /// On duplicate identity, return both the error and the original incoming
    /// value so callers can recover it without cloning potentially large IR.
    pub fn insert_function(
        &mut self,
        id: FunctionId,
        function: F,
    ) -> Result<(), (ModuleError, F)> {
        if self.functions.contains_key(&id) {
            return Err((ModuleError::DuplicateFunction(id), function));
        }

        self.functions.insert(id, function);
        Ok(())
    }

    /// Borrow a function by identity.
    #[must_use]
    pub fn function(&self, id: FunctionId) -> Option<&F> {
        self.functions.get(&id)
    }

    /// Mutably borrow a function by identity.
    ///
    /// Any mutation may invalidate previously established verification
    /// results. Callers must re-run the authoritative verifier before
    /// accepting the module for downstream compilation.
    pub fn function_mut(&mut self, id: FunctionId) -> Option<&mut F> {
        self.functions.get_mut(&id)
    }

    /// Remove a function and return ownership to the caller.
    pub fn remove_function(&mut self, id: FunctionId) -> Option<F> {
        self.functions.remove(&id)
    }

    /// Iterate over functions in deterministic FunctionId order.
    pub fn functions(
        &self,
    ) -> impl ExactSizeIterator<Item = (FunctionId, &F)> + '_ {
        self.functions
            .iter()
            .map(|(id, function)| (*id, function))
    }

    /// Return the read-only function map for verification and serialization.
    #[must_use]
    pub fn functions_by_id(&self) -> &BTreeMap<FunctionId, F> {
        &self.functions
    }

    /// Declare a resolved symbol import.
    ///
    /// Duplicate imports are idempotent. The resolver must separately verify
    /// that the symbol exists and that the importing module is authorized
    /// to use it.
    pub fn add_import(&mut self, symbol: SymbolId) -> bool {
        self.imports.insert(symbol)
    }

    /// Remove an import declaration.
    pub fn remove_import(&mut self, symbol: SymbolId) -> bool {
        self.imports.remove(&symbol)
    }

    /// Check whether a symbol is imported.
    #[must_use]
    pub fn has_import(&self, symbol: SymbolId) -> bool {
        self.imports.contains(&symbol)
    }

    /// Iterate over imported symbol identities in deterministic order.
    pub fn imports(&self) -> impl ExactSizeIterator<Item = SymbolId> + '_ {
        self.imports.iter().copied()
    }

    /// Return the read-only import set.
    #[must_use]
    pub fn imports_set(&self) -> &BTreeSet<SymbolId> {
        &self.imports
    }

    /// Declare a symbol export.
    ///
    /// This records module intent; it does not grant access or prove global
    /// uniqueness. The linker and verifier enforce those rules.
    pub fn add_export(&mut self, symbol: SymbolId) -> bool {
        self.exports.insert(symbol)
    }

    /// Remove an export declaration.
    pub fn remove_export(&mut self, symbol: SymbolId) -> bool {
        self.exports.remove(&symbol)
    }

    /// Check whether a symbol is exported.
    #[must_use]
    pub fn has_export(&self, symbol: SymbolId) -> bool {
        self.exports.contains(&symbol)
    }

    /// Iterate over exported symbol identities in deterministic order.
    pub fn exports(&self) -> impl ExactSizeIterator<Item = SymbolId> + '_ {
        self.exports.iter().copied()
    }

    /// Return the read-only export set.
    #[must_use]
    pub fn exports_set(&self) -> &BTreeSet<SymbolId> {
        &self.exports
    }

    /// Insert descriptive metadata.
    ///
    /// Metadata must not be used to encode operation semantics, types,
    /// effects, resource constraints, capabilities, or authorization policy.
    /// Those require their canonical typed representations.
    ///
    /// Returns the previous value if the key was already present.
    pub fn set_metadata(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Option<String>, ModuleError> {
        let key = key.into();

        if key.is_empty() || key.trim() != key {
            return Err(ModuleError::InvalidMetadataKey(key));
        }

        Ok(self.metadata.insert(key, value.into()))
    }

    /// Read descriptive metadata by key.
    #[must_use]
    pub fn metadata(&self, key: &str) -> Option<&str> {
        self.metadata.get(key).map(String::as_str)
    }

    /// Iterate over metadata in deterministic key order.
    pub fn metadata_entries(
        &self,
    ) -> impl ExactSizeIterator<Item = (&str, &str)> + '_ {
        self.metadata
            .iter()
            .map(|(key, value)| (key.as_str(), value.as_str()))
    }

    /// Return a read-only view of the metadata map.
    #[must_use]
    pub fn metadata_map(&self) -> &BTreeMap<String, String> {
        &self.metadata
    }

    /// Validate module-local invariants.
    ///
    /// Call this after decoding untrusted data and before handing a module to
    /// a verifier, optimizer, serializer, or lowering pipeline.
    ///
    /// This method does not replace full IR verification. It checks only
    /// invariants owned by this module container.
    pub fn validate(&self) -> Result<(), ModuleError> {
        self.name.validate()?;

        for key in self.metadata.keys() {
            if key.is_empty() || key.trim() != key {
                return Err(ModuleError::InvalidMetadataKey(key.clone()));
            }
        }

        // BTreeMap and BTreeSet already enforce unique keys and entries.
        // Function bodies, symbol definitions, import resolution, export
        // visibility, and cross-module consistency belong to the verifier.
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zuir::ids::FunctionId;

    fn module_id(value: u128) -> ModuleId {
        ModuleId::new(value).expect("nonzero module ID")
    }

    fn function_id(value: u128) -> FunctionId {
        FunctionId::new(value).expect("nonzero function ID")
    }

    fn symbol_id(value: u128) -> SymbolId {
        SymbolId::new(value).expect("nonzero symbol ID")
    }

    #[test]
    fn constructs_valid_module_name() {
        let module = ZuirModule::<String>::named(module_id(1), "math.linear")
            .expect("valid module name");

        assert_eq!(module.name().as_str(), "math.linear");
        assert_eq!(module.id(), module_id(1));
        assert!(module.is_empty());
    }

    #[test]
    fn rejects_invalid_module_names() {
        for invalid in [
            "",
            ".main",
            "main.",
            "main..math",
            "main/math",
            "main module",
            " main",
            "main ",
        ] {
            assert!(
                ModuleName::new(invalid).is_err(),
                "accepted invalid module name: {invalid:?}"
            );
        }
    }

    #[test]
    fn function_insertion_never_replaces_existing_function() {
        let mut module =
            ZuirModule::named(module_id(1), "main").expect("valid module");

        module
            .insert_function(function_id(1), "first".to_owned())
            .expect("first insertion");

        let error = module
            .insert_function(function_id(1), "second".to_owned())
            .expect_err("duplicate function must be rejected");

        assert_eq!(error.0, ModuleError::DuplicateFunction(function_id(1)));
        assert_eq!(error.1, "second");
        assert_eq!(module.function(function_id(1)).map(String::as_str), Some("first"));
    }

    #[test]
    fn removal_returns_function_ownership() {
        let mut module =
            ZuirModule::named(module_id(1), "main").expect("valid module");

        module
            .insert_function(function_id(1), "body".to_owned())
            .expect("insert function");

        assert_eq!(
            module.remove_function(function_id(1)),
            Some("body".to_owned())
        );
        assert!(module.is_empty());
    }

    #[test]
    fn imports_and_exports_are_unique_and_deterministic() {
        let mut module =
            ZuirModule::<()>::named(module_id(1), "main").expect("valid module");

        assert!(module.add_import(symbol_id(2)));
        assert!(!module.add_import(symbol_id(2)));
        assert!(module.add_import(symbol_id(1)));

        assert!(module.add_export(symbol_id(3)));
        assert!(!module.add_export(symbol_id(3)));

        assert_eq!(
            module.imports().collect::<Vec<_>>(),
            vec![symbol_id(1), symbol_id(2)]
        );
        assert_eq!(module.exports().collect::<Vec<_>>(), vec![symbol_id(3)]);
    }

    #[test]
    fn metadata_is_validated_and_replace_is_explicit_in_result() {
        let mut module =
            ZuirModule::<()>::named(module_id(1), "main").expect("valid module");

        assert!(module.set_metadata(" ", "value").is_err());
        assert_eq!(module.set_metadata("owner", "compiler").unwrap(), None);
        assert_eq!(
            module.set_metadata("owner", "optimizer").unwrap(),
            Some("compiler".to_owned())
        );
        assert_eq!(module.metadata("owner"), Some("optimizer"));
    }

    #[test]
    fn validation_accepts_well_formed_module() {
        let module =
            ZuirModule::<()>::named(module_id(1), "main").expect("valid module");

        assert!(module.validate().is_ok());
    }

    #[test]
    fn serialized_module_round_trip_preserves_contents() {
        let mut module =
            ZuirModule::named(module_id(1), "main").expect("valid module");

        module
            .insert_function(function_id(1), "body".to_owned())
            .expect("insert function");
        module.add_import(symbol_id(4));
        module.add_export(symbol_id(5));
        module.set_metadata("purpose", "test").expect("metadata");

        let encoded = serde_json::to_string(&module).expect("serialize");
        let decoded: ZuirModule<String> =
            serde_json::from_str(&encoded).expect("deserialize");

        decoded.validate().expect("validate decoded module");
        assert_eq!(module, decoded);
    }
}
