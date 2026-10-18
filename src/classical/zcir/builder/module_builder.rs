
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_debug_implementations)]

//! # ZCIR Module Builder
//!
//! Canonical path: `src/classical/zcir/builder/module_builder.rs`
//! Minimum Rust baseline: 1.97+
//!
//! ## Purpose
//!
//! Construct a classical-facing module using the canonical ZUIR module and
//! function representations. This builder is an integration boundary, not
//! an independent IR implementation.
//!
//! ## Authority
//!
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/poco-reaf.md`
//! - `grammar/validation/semantic-boundaries.md`
//! - `src/zuir/ids.rs`
//! - `src/zuir/ir/module.rs`
//! - `src/zuir/ir/function.rs`
//! - `src/classical/zcir/ir/module.rs`
//! - `src/classical/zcir/ir/function.rs`
//!
//! ## Ownership
//!
//! ZUIR owns module and function storage, typed identities, module naming,
//! local function validation, symbol sets, and metadata storage.
//!
//! This file owns the builder lifecycle, insertion-time consistency checks,
//! and builder-specific error reporting.
//!
//! ## Semantic boundary
//!
//! The frontend and semantic analyzer must resolve source meaning before
//! invoking this builder. It does not parse source, infer types, infer
//! effects, allocate hardware, schedule execution, or lower operations.
//!
//! ## Scalability
//!
//! No arbitrary module-size, function-count, import-count, or export-count
//! ceiling is introduced here. Storage grows according to available
//! resources and the canonical container's implementation limits.
//!
//! Literal infinite memory or computation is not promised. Resource
//! exhaustion, identifier exhaustion, and target infeasibility must be
//! reported at the layer that can detect them.
//!
//! ## Integration
//!
//! `builder/mod.rs` must declare `pub mod module_builder;` and re-export
//! `ModuleBuilder` and `ModuleBuilderError` if those names are intended
//! to be available from the parent builder namespace.
//!
//! `src/classical/zcir/ir/module.rs` supplies `ZcirModule` and
//! `ZcirModuleError`; `ir/function.rs` supplies `ZcirFunction` and
//! `ZcirFunctionError`. These are aliases of canonical ZUIR definitions.
//!
//! This builder verifies each function's local invariants and confirms that
//! its embedded FunctionId matches the ID used for module insertion.
//! It cannot establish that referenced regions, types, values, or symbols
//! exist. Run the authoritative full-IR verifier before optimization,
//! serialization of trusted artifacts, lowering, or execution.
//!
//! ## Freeze criteria
//!
//! This file is frozen only after its module declarations are integrated,
//! its tests compile against the actual repository, and the repository's
//! formatting, build, test, and lint gates pass.

use core::fmt;

use crate::classical::zcir::ir::function::{
    ZcirFunction,
    ZcirFunctionError,
};
use crate::classical::zcir::ir::module::{
    ZcirModule,
    ZcirModuleError,
};
use crate::zuir::ids::{
    FunctionId,
    ModuleId,
    SymbolId,
};

/// The canonical module type constructed by this builder.
pub type BuiltZcirModule = ZcirModule<ZcirFunction>;

/// Errors produced while constructing a ZCIR module.
///
/// Canonical module and function errors are preserved rather than flattened
/// into strings, so callers can make structured decisions about failures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModuleBuilderError {
    /// A function's embedded identity differs from its insertion key.
    FunctionIdMismatch {
        /// Identity supplied to the module insertion operation.
        expected: FunctionId,
        /// Identity stored in the canonical function.
        actual: FunctionId,
    },

    /// The canonical function rejected its own local invariants.
    InvalidFunction(ZcirFunctionError),

    /// The canonical module rejected an operation.
    Module(ZcirModuleError),
}

impl fmt::Display for ModuleBuilderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FunctionIdMismatch { expected, actual } => write!(
                f,
                "function insertion identity mismatch: key {expected}, \
                 function identity {actual}"
            ),
            Self::InvalidFunction(error) => {
                write!(f, "invalid function supplied to ZCIR module builder: {error}")
            }
            Self::Module(error) => {
                write!(f, "ZCIR module construction failed: {error}")
            }
        }
    }
}

impl std::error::Error for ModuleBuilderError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidFunction(error) => Some(error),
            Self::Module(error) => Some(error),
            Self::FunctionIdMismatch { .. } => None,
        }
    }
}

impl From<ZcirModuleError> for ModuleBuilderError {
    fn from(error: ZcirModuleError) -> Self {
        Self::Module(error)
    }
}

/// Builds a canonical ZUIR-backed classical module.
///
/// The builder takes explicit identity input. It never creates its own ID
/// allocator, because independent allocators could produce colliding IDs
/// when their entities are later combined.
///
/// The builder owns the module while it is being constructed. A successful
/// `finish` transfers ownership of the completed container to the caller.
#[derive(Debug, Clone)]
pub struct ModuleBuilder {
    module: BuiltZcirModule,
}

impl ModuleBuilder {
    /// Create a builder with an explicit module ID and validated name.
    ///
    /// The caller is responsible for ensuring that `id` is unique in the
    /// enclosing program's identity scope.
    pub fn new(
        id: ModuleId,
        name: impl Into<String>,
    ) -> Result<Self, ModuleBuilderError> {
        let module = BuiltZcirModule::named(id, name)
            .map_err(ModuleBuilderError::Module)?;

        Ok(Self { module })
    }

    /// Return the module's canonical identity.
    #[must_use]
    pub const fn id(&self) -> ModuleId {
        self.module.id()
    }

    /// Return the validated canonical module name.
    #[must_use]
    pub fn name(&self) -> &str {
        self.module.name().as_str()
    }

    /// Return the number of functions currently inserted.
    #[must_use]
    pub fn function_count(&self) -> usize {
        self.module.function_count()
    }

    /// Return whether the module contains no functions.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.module.is_empty()
    }

    /// Insert a canonical function under its own identity.
    ///
    /// The function is validated before the module is mutated. A duplicate
    /// identity, mismatched identity, or invalid function leaves the module
    /// unchanged and returns ownership of the rejected function.
    ///
    /// The returned error retains structured details. No function is
    /// silently replaced.
    pub fn insert_function(
        &mut self,
        function: ZcirFunction,
    ) -> Result<(), (ModuleBuilderError, ZcirFunction)> {
        let id = function.id();

        if let Err(error) = function.validate() {
            return Err((
                ModuleBuilderError::InvalidFunction(error),
                function,
            ));
        }

        match self.module.insert_function(id, function) {
            Ok(()) => Ok(()),
            Err((error, function)) => {
                Err((ModuleBuilderError::Module(error), function))
            }
        }
    }

    /// Insert a function with an explicitly supplied map key.
    ///
    /// This method is useful for adapters receiving an identity separately
    /// from a function payload. It rejects a mismatch before insertion.
    /// The caller retains the rejected function on every failure.
    pub fn insert_function_with_id(
        &mut self,
        expected: FunctionId,
        function: ZcirFunction,
    ) -> Result<(), (ModuleBuilderError, ZcirFunction)> {
        let actual = function.id();

        if expected != actual {
            return Err((
                ModuleBuilderError::FunctionIdMismatch { expected, actual },
                function,
            ));
        }

        if let Err(error) = function.validate() {
            return Err((
                ModuleBuilderError::InvalidFunction(error),
                function,
            ));
        }

        match self.module.insert_function(expected, function) {
            Ok(()) => Ok(()),
            Err((error, function)) => {
                Err((ModuleBuilderError::Module(error), function))
            }
        }
    }

    /// Declare an already-resolved imported symbol.
    ///
    /// Returns `true` if the import was newly added and `false` if it was
    /// already present. This records intent; it does not resolve the symbol
    /// or prove that the importing module is authorized to use it.
    pub fn add_import(&mut self, symbol: SymbolId) -> bool {
        self.module.add_import(symbol)
    }

    /// Remove an imported symbol.
    ///
    /// Returns `true` if the import existed.
    pub fn remove_import(&mut self, symbol: SymbolId) -> bool {
        self.module.remove_import(symbol)
    }

    /// Declare a symbol to be exported by this module.
    ///
    /// Returns `true` if newly inserted. Global uniqueness, visibility, and
    /// linkage are checked by the relevant verifier and linker.
    pub fn add_export(&mut self, symbol: SymbolId) -> bool {
        self.module.add_export(symbol)
    }

    /// Remove a symbol from this module's export set.
    ///
    /// Returns `true` if the export existed.
    pub fn remove_export(&mut self, symbol: SymbolId) -> bool {
        self.module.remove_export(symbol)
    }

    /// Set descriptive, non-semantic metadata.
    ///
    /// Do not encode types, effects, capabilities, resource requirements,
    /// authorization policy, or operation semantics in metadata.
    pub fn set_metadata(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Option<String>, ModuleBuilderError> {
        self.module
            .set_metadata(key, value)
            .map_err(ModuleBuilderError::Module)
    }

    /// Borrow the current canonical module without transferring ownership.
    ///
    /// Consumers must not treat this borrow as proof of full-IR validity.
    #[must_use]
    pub const fn as_module(&self) -> &BuiltZcirModule {
        &self.module
    }

    /// Finish construction and transfer the canonical module to the caller.
    ///
    /// This rechecks module-local invariants. It does not replace full-IR
    /// verification of referenced regions, values, types, symbols, effects,
    /// or cross-module relationships.
    pub fn finish(self) -> Result<BuiltZcirModule, ModuleBuilderError> {
        self.module
            .validate()
            .map_err(ModuleBuilderError::Module)?;

        // Function-local validation and identity/key consistency are
        // enforced at insertion time. Recheck them at this boundary as
        // defense in depth.
        for (key, function) in self.module.functions() {
            if key != function.id() {
                return Err(ModuleBuilderError::FunctionIdMismatch {
                    expected: key,
                    actual: function.id(),
                });
            }

            function
                .validate()
                .map_err(ModuleBuilderError::InvalidFunction)?;
        }

        Ok(self.module)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::classical::zcir::ir::function::ZcirFunctionSignature;

    fn module_id(raw: u128) -> ModuleId {
        ModuleId::new(raw).expect("test module ID must be nonzero")
    }

    fn function_id(raw: u128) -> FunctionId {
        FunctionId::new(raw).expect("test function ID must be nonzero")
    }

    fn symbol_id(raw: u128) -> SymbolId {
        SymbolId::new(raw).expect("test symbol ID must be nonzero")
    }

    fn declaration(id: u128, symbol: u128) -> ZcirFunction {
        ZcirFunction::declaration(
            function_id(id),
            symbol_id(symbol),
            ZcirFunctionSignature::new(Vec::new(), Vec::new()),
        )
    }

    fn builder() -> ModuleBuilder {
        ModuleBuilder::new(module_id(1), "classical.main")
            .expect("valid test module")
    }

    #[test]
    fn creates_empty_canonical_module() {
        let builder = builder();

        assert_eq!(builder.id(), module_id(1));
        assert_eq!(builder.name(), "classical.main");
        assert!(builder.is_empty());
        assert_eq!(builder.function_count(), 0);

        let module = builder.finish().expect("valid empty module");
        assert_eq!(module.id(), module_id(1));
        assert!(module.validate().is_ok());
    }

    #[test]
    fn inserts_canonical_function() {
        let mut builder = builder();

        builder
            .insert_function(declaration(10, 20))
            .expect("valid function");

        assert_eq!(builder.function_count(), 1);
        assert!(builder.as_module().contains_function(function_id(10)));

        let module = builder.finish().expect("valid module");
        assert_eq!(module.function_count(), 1);
    }

    #[test]
    fn explicit_identity_mismatch_is_rejected_without_mutation() {
        let mut builder = builder();
        let function = declaration(10, 20);

        let (error, returned) = builder
            .insert_function_with_id(function_id(11), function)
            .expect_err("mismatched identity must be rejected");

        assert_eq!(
            error,
            ModuleBuilderError::FunctionIdMismatch {
                expected: function_id(11),
                actual: function_id(10),
            }
        );
        assert_eq!(returned.id(), function_id(10));
        assert!(builder.is_empty());
    }

    #[test]
    fn duplicate_function_does_not_replace_existing_function() {
        let mut builder = builder();

        builder
            .insert_function(declaration(10, 20))
            .expect("first insertion");

        let (error, returned) = builder
            .insert_function(declaration(10, 21))
            .expect_err("duplicate identity must be rejected");

        assert_eq!(
            error,
            ModuleBuilderError::Module(
                ZcirModuleError::DuplicateFunction(function_id(10))
            )
        );
        assert_eq!(returned.id(), function_id(10));
        assert_eq!(builder.function_count(), 1);

        let stored = builder
            .as_module()
            .function(function_id(10))
            .expect("original function remains");

        assert_eq!(stored.symbol(), symbol_id(20));
    }

    #[test]
    fn imports_exports_and_metadata_use_canonical_storage() {
        let mut builder = builder();

        assert!(builder.add_import(symbol_id(2)));
        assert!(!builder.add_import(symbol_id(2)));
        assert!(builder.add_export(symbol_id(3)));
        assert!(!builder.add_export(symbol_id(3)));

        assert_eq!(
            builder.set_metadata("documentation", "entry point"),
            Ok(None)
        );
        assert_eq!(
            builder.set_metadata("documentation", "updated"),
            Ok(Some("entry point".to_owned()))
        );

        let module = builder.finish().expect("valid module");

        assert!(module.has_import(symbol_id(2)));
        assert!(module.has_export(symbol_id(3)));
        assert_eq!(module.metadata("documentation"), Some("updated"));
    }

    #[test]
    fn invalid_metadata_does_not_mutate_module_metadata() {
        let mut builder = builder();

        assert!(builder.set_metadata(" invalid", "value").is_err());
        assert_eq!(builder.as_module().metadata(" invalid"), None);
    }

    #[test]
    fn invalid_module_name_is_rejected_at_construction() {
        assert!(ModuleBuilder::new(module_id(1), "classical..main").is_err());
    }

    #[test]
    fn removal_preserves_canonical_module_invariants() {
        let mut builder = builder();
        let symbol = symbol_id(2);

        assert!(builder.add_import(symbol));
        assert!(builder.remove_import(symbol));
        assert!(!builder.remove_import(symbol));

        let module = builder.finish().expect("valid module");
        assert!(module.validate().is_ok());
    }

    #[test]
    fn function_iteration_remains_deterministic() {
        let mut builder = builder();

        builder.insert_function(declaration(30, 130)).expect("insert");
        builder.insert_function(declaration(10, 110)).expect("insert");
        builder.insert_function(declaration(20, 120)).expect("insert");

        let ids: Vec<_> = builder
            .as_module()
            .functions()
            .map(|(id, _)| id)
            .collect();

        assert_eq!(
            ids,
            vec![function_id(10), function_id(20), function_id(30)]
        );
    }
}
