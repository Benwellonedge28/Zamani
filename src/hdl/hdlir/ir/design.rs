
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani HDLIR — Design Graph
//!
//! Path: `src/hdl/hdlir/ir/design.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: Rust 1.97.
//! Safety: Safe Rust only.
//! Status: Core design-graph implementation; freeze after integration tests.
//!
//! ## Purpose
//!
//! Own the top-level HDLIR design, its logical module registry, optional
//! entry-module selection, and design-level structural invariants.
//!
//! ## Architectural authority
//!
//! - `grammar/DESIGN.md`
//! - `grammar/spec/hdl.md`
//! - `grammar/Zamani.g4`
//! - `grammar/hdl/hdl.g4`
//! - `grammar/specification/semantic-model.md`
//! - `src/hdl/api.rs`
//! - `src/hdl/ids.rs`
//! - `src/hdl/hdlir/HDLIR-SPEC.md`
//! - `src/hdl/hdlir/ids.rs`
//! - `src/hdl/hdlir/errors.rs`
//! - `src/hdl/hdlir/version.rs`
//! - `src/zuir/ZUIR-SPEC.md`
//!
//! The grammar owns syntax. The frontend owns canonical AST and semantic
//! meaning. ZUIR owns universal computational representation. HDLIR owns
//! the hardware-domain design graph.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - A design's HDLIR identity and logical name.
//! - An ordered-by-identity registry of logical module definitions.
//! - Optional entry-module selection.
//! - Module-ID and module-name uniqueness within a design.
//! - Design-level structural validation.
//! - A small integration trait implemented by the canonical HDLIR module
//!   definition type.
//!
//! This module does not own:
//!
//! - Source parsing, AST construction, name resolution, or semantic analysis.
//! - The implementation of a module's ports, operations, regions, or body.
//! - Signal connectivity, process semantics, clocks, resets, or memories.
//! - The universal ZUIR representation.
//! - Physical devices, pins, cells, placement, routing, or allocation.
//! - Target discovery, synthesis, scheduling, or execution.
//! - Artifact serialization or persistent cross-build identity.
//!
//! ## Registry contract
//!
//! The registry is indexed by `HdlirModuleId`. A module's ID and logical name
//! must be unique within a design. Names are compared exactly; this layer
//! does not perform source-language name resolution or case folding.
//!
//! A module definition is owned by the design registry. Removing a module
//! also clears its entry-point designation, if present. External references
//! to removed module IDs must be revalidated by their owning subsystem.
//!
//! The registry uses `BTreeMap` to make module iteration deterministic.
//! Iteration order is identity order, not source declaration order.
//! Source-order preservation belongs to the module/source-provenance contract.
//!
//! ## Scalability
//!
//! No fixed maximum number of modules is imposed. The registry grows subject
//! to available memory, representable identifiers, and explicit operational
//! policies enforced by the caller.
//!
//! This module does not convert hardware quantities into host-sized integers
//! or assume a fixed machine word size. It uses checked accounting where
//! counts must be represented as `u64`.
//!
//! An absent operational budget is not a promise of infinite resources.
//! No finite implementation can process literally infinite input.
//!
//! ## Integration contract
//!
//! The canonical `ir/module.rs` module type must implement `DesignModule`.
//! Its `id()` and `name()` methods must return stable identity and logical
//! name values for the lifetime of its registration.
//!
//! The module implementation remains the sole owner of module internals.
//! Builders may use this registry for checked module insertion and entry
//! selection. Verification must call `validate_structure()` before trusting
//! the design graph. Lowering must preserve module identities and provenance.
//!
//! This file intentionally does not import the not-yet-implemented module
//! definition, avoiding an assumed constructor or field layout.
//!
//! ## Failure behavior
//!
//! Duplicate IDs, duplicate logical names, invalid names, missing entry
//! modules, inconsistent registry keys, and incomplete designs are reported
//! explicitly. Invalid mutations leave the existing registry unchanged.
//!
//! This module does not claim that structurally valid designs are type-valid,
//! synthesizable, target-feasible, or physically implementable.
//!
//! ## Freeze criteria
//!
//! - All registry operations preserve uniqueness and ownership invariants.
//! - Entry selection can reference only a registered module.
//! - Removal cannot leave a dangling entry-point reference.
//! - Registry iteration is deterministic.
//! - Invalid operations do not partially mutate the registry.
//! - Tests cover duplicates, empty designs, entry points, removal, malformed
//!   module names, large dynamic registries, and structural corruption.
//! - Integration with the real module type and repository checks passes.

use core::fmt;
use std::collections::BTreeMap;

use crate::hdl::hdlir::errors::{HdlirError, HdlirResult};
use crate::hdl::hdlir::ids::{HdlirDesignId, HdlirModuleId};

/// Stable logical name of an HDLIR design.
///
/// Names are opaque identifiers at this layer. Source-language scoping,
/// normalization, and name resolution are owned by the frontend.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DesignName(String);

impl DesignName {
    /// Construct a design name from a non-empty, non-whitespace string.
    ///
    /// The original string is retained; this constructor does not trim,
    /// normalize, case-fold, or otherwise rewrite a name.
    ///
    /// # Errors
    ///
    /// Returns an HDLIR input error if the name is empty or consists only
    /// of whitespace.
    pub fn new(value: impl Into<String>) -> HdlirResult<Self> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(HdlirError::invalid_input(
                "design_name",
                "design name must not be empty or whitespace-only",
            ));
        }

        Ok(Self(value))
    }

    /// Return the exact stored name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for DesignName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Stable logical name of a module within a design.
///
/// Uniqueness is checked by [`HdlirDesign`] using exact string equality.
/// This type does not perform source-level name resolution.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ModuleName(String);

impl ModuleName {
    /// Construct a module name from a non-empty, non-whitespace string.
    ///
    /// The original string is preserved.
    ///
    /// # Errors
    ///
    /// Returns an HDLIR input error if the name is empty or consists only
    /// of whitespace.
    pub fn new(value: impl Into<String>) -> HdlirResult<Self> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(HdlirError::invalid_input(
                "module_name",
                "module name must not be empty or whitespace-only",
            ));
        }

        Ok(Self(value))
    }

    /// Return the exact stored name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ModuleName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Minimum contract required of a module definition registered in a design.
///
/// The canonical HDLIR module type in `ir/module.rs` must implement this
/// trait. This keeps design ownership independent of the module's internal
/// representation and prevents the design registry from duplicating it.
///
/// Implementations must return stable values while registered. Mutating a
/// registered module's ID or name through interior mutability violates this
/// contract and can invalidate registry invariants.
pub trait DesignModule {
    /// Return the module's HDLIR-owned identity.
    fn id(&self) -> HdlirModuleId;

    /// Return the module's exact logical name.
    fn name(&self) -> &str;
}

/// Top-level hardware-domain design.
///
/// A design owns module definitions and optionally designates one registered
/// module as its entry point. Module internals remain owned by the module
/// definition type.
///
/// The generic parameter allows the canonical module type to be integrated
/// without this file assuming its field layout or constructor API.
#[derive(Debug)]
pub struct HdlirDesign<M: DesignModule> {
    id: HdlirDesignId,
    name: DesignName,
    modules: BTreeMap<HdlirModuleId, M>,
    entry_module: Option<HdlirModuleId>,
}

impl<M: DesignModule> HdlirDesign<M> {
    /// Create an empty design with a validated identity and logical name.
    ///
    /// An empty design is a valid construction state but is not a complete
    /// design suitable for downstream lowering or realization.
    ///
    /// # Errors
    ///
    /// Returns an error if `name` is empty or whitespace-only.
    pub fn new(
        id: HdlirDesignId,
        name: impl Into<String>,
    ) -> HdlirResult<Self> {
        Ok(Self {
            id,
            name: DesignName::new(name)?,
            modules: BTreeMap::new(),
            entry_module: None,
        })
    }

    /// Return the design identity.
    #[must_use]
    pub const fn id(&self) -> HdlirDesignId {
        self.id
    }

    /// Return the design's logical name.
    #[must_use]
    pub fn name(&self) -> &DesignName {
        &self.name
    }

    /// Return the optional entry-module identity.
    #[must_use]
    pub const fn entry_module_id(&self) -> Option<HdlirModuleId> {
        self.entry_module
    }

    /// Return the number of registered modules as a `usize`.
    ///
    /// The returned value reflects the backing collection's host-sized
    /// representation; it is not a language-level capacity limit.
    #[must_use]
    pub fn module_count(&self) -> usize {
        self.modules.len()
    }

    /// Return whether the design currently has no registered modules.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.modules.is_empty()
    }

    /// Iterate over registered modules in deterministic identity order.
    ///
    /// The iteration order is by `HdlirModuleId`, not source declaration
    /// order. The iterator does not allocate a secondary collection.
    pub fn modules(&self) -> impl Iterator<Item = (&HdlirModuleId, &M)> {
        self.modules.iter()
    }

    /// Look up a module by identity.
    #[must_use]
    pub fn module(&self, id: HdlirModuleId) -> Option<&M> {
        self.modules.get(&id)
    }

    /// Look up a mutable module by identity.
    ///
    /// Callers must not mutate the registered module's ID or name. The
    /// `DesignModule` contract requires both to remain stable.
    #[must_use]
    pub fn module_mut(&mut self, id: HdlirModuleId) -> Option<&mut M> {
        self.modules.get_mut(&id)
    }

    /// Find a module by its exact logical name.
    ///
    /// Names are unique by registry invariant, so at most one module can
    /// match. No source-language name resolution is performed.
    #[must_use]
    pub fn module_by_name(&self, name: &str) -> Option<&M> {
        self.modules
            .values()
            .find(|module| module.name() == name)
    }

    /// Register a module definition.
    ///
    /// Registration is atomic with respect to the design registry: if an
    /// identity or name conflict is detected, the design remains unchanged.
    ///
    /// # Errors
    ///
    /// Returns a duplicate-entity error if another module already has the
    /// same identity or exact logical name.
    ///
    /// Returns an invalid-input error if the module name is empty or
    /// whitespace-only.
    pub fn insert_module(&mut self, module: M) -> HdlirResult<()> {
        let module_id = module.id();
        let module_name = module.name();

        if module_name.trim().is_empty() {
            return Err(HdlirError::invalid_input(
                "module_name",
                "module name must not be empty or whitespace-only",
            ));
        }

        if self.modules.contains_key(&module_id) {
            return Err(HdlirError::DuplicateEntity {
                entity_kind: "HDLIR module",
                identity: module_id.to_string(),
                scope: Some(self.name.to_string()),
            });
        }

        if let Some(existing) = self.module_by_name(module_name) {
            return Err(HdlirError::DuplicateEntity {
                entity_kind: "HDLIR module name",
                identity: module_name.to_owned(),
                scope: Some(format!(
                    "{} (existing module ID {})",
                    self.name,
                    existing.id()
                )),
            });
        }

        self.modules.insert(module_id, module);
        Ok(())
    }

    /// Remove a module by identity.
    ///
    /// If the removed module was the entry module, the entry designation is
    /// cleared in the same operation. The caller is responsible for checking
    /// references to the removed module in other modules or external tables.
    #[must_use]
    pub fn remove_module(&mut self, id: HdlirModuleId) -> Option<M> {
        let removed = self.modules.remove(&id);

        if removed.is_some() && self.entry_module == Some(id) {
            self.entry_module = None;
        }

        removed
    }

    /// Set the entry module to a module already registered in this design.
    ///
    /// This operation does not infer an entry point from module ordering.
    ///
    /// # Errors
    ///
    /// Returns a missing-entity error if the specified module is not
    /// registered in this design.
    pub fn set_entry_module(
        &mut self,
        id: HdlirModuleId,
    ) -> HdlirResult<()> {
        if !self.modules.contains_key(&id) {
            return Err(HdlirError::MissingEntity {
                entity_kind: "entry module",
                identity: id.to_string(),
                scope: Some(self.name.to_string()),
            });
        }

        self.entry_module = Some(id);
        Ok(())
    }

    /// Clear the entry-module designation.
    ///
    /// This is useful during construction or transformation. A complete
    /// design must designate an entry module before being handed to a stage
    /// whose contract requires one.
    pub fn clear_entry_module(&mut self) {
        self.entry_module = None;
    }

    /// Validate design-level registry invariants.
    ///
    /// This checks module IDs, module names, entry-point membership, and
    /// registry consistency. It does not replace module verification,
    /// type checking, connectivity checking, or process/clock verification.
    ///
    /// # Errors
    ///
    /// Returns a structured HDLIR error when a registry invariant is broken.
    pub fn validate_structure(&self) -> HdlirResult<()> {
        for (key, module) in &self.modules {
            if *key != module.id() {
                return Err(HdlirError::invalid_invariant(
                    "design_module_registry_key_matches_id",
                    format!(
                        "registry key {key} does not match module ID {}",
                        module.id()
                    ),
                ));
            }

            if module.name().trim().is_empty() {
                return Err(HdlirError::invalid_invariant(
                    "design_module_name_is_nonempty",
                    format!("module {} has an empty logical name", module.id()),
                ));
            }
        }

        // Duplicate names should be impossible through the public insertion
        // API. Checking here also protects consumers if a module violates
        // the stable-name contract or the design was created by a future
        // deserializer or migration path.
        let mut names = std::collections::BTreeSet::new();

        for module in self.modules.values() {
            if !names.insert(module.name()) {
                return Err(HdlirError::DuplicateEntity {
                    entity_kind: "HDLIR module name",
                    identity: module.name().to_owned(),
                    scope: Some(self.name.to_string()),
                });
            }
        }

        if let Some(entry_id) = self.entry_module {
            if !self.modules.contains_key(&entry_id) {
                return Err(HdlirError::MissingEntity {
                    entity_kind: "entry module",
                    identity: entry_id.to_string(),
                    scope: Some(self.name.to_string()),
                });
            }
        }

        Ok(())
    }

    /// Validate the minimum completeness conditions for a downstream stage
    /// that requires an entry point.
    ///
    /// This method is intentionally separate from `validate_structure()`:
    /// an empty or entry-less design can be a legitimate intermediate
    /// construction state, but it cannot satisfy this completeness contract.
    ///
    /// # Errors
    ///
    /// Returns an invariant error when the design has no modules or no
    /// entry module, and propagates structural validation failures.
    pub fn validate_complete(&self) -> HdlirResult<()> {
        self.validate_structure()?;

        if self.modules.is_empty() {
            return Err(HdlirError::invalid_invariant(
                "complete_design_has_modules",
                format!("design `{}` contains no modules", self.name),
            ));
        }

        if self.entry_module.is_none() {
            return Err(HdlirError::invalid_invariant(
                "complete_design_has_entry_module",
                format!("design `{}` has no entry module", self.name),
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{DesignModule, HdlirDesign};
    use crate::hdl::hdlir::ids::{HdlirDesignId, HdlirModuleId};

    #[derive(Debug)]
    struct TestModule {
        id: HdlirModuleId,
        name: String,
    }

    impl TestModule {
        fn new(id: u128, name: &str) -> Self {
            Self {
                id: HdlirModuleId::new(id).expect("non-zero test ID"),
                name: name.to_owned(),
            }
        }
    }

    impl DesignModule for TestModule {
        fn id(&self) -> HdlirModuleId {
            self.id
        }

        fn name(&self) -> &str {
            &self.name
        }
    }

    fn design() -> HdlirDesign<TestModule> {
        HdlirDesign::new(
            HdlirDesignId::new(1).expect("non-zero test ID"),
            "top_design",
        )
        .expect("valid design")
    }

    #[test]
    fn creates_an_empty_design_as_a_construction_state() {
        let design = design();

        assert!(design.is_empty());
        assert_eq!(design.module_count(), 0);
        assert_eq!(design.entry_module_id(), None);
        assert!(design.validate_structure().is_ok());
        assert!(design.validate_complete().is_err());
    }

    #[test]
    fn inserts_and_finds_a_module_by_id_and_name() {
        let mut design = design();
        let module = TestModule::new(1, "top");

        let id = module.id();
        design.insert_module(module).expect("insert succeeds");

        assert_eq!(design.module_count(), 1);
        assert_eq!(design.module(id).map(DesignModule::name), Some("top"));
        assert_eq!(
            design.module_by_name("top").map(DesignModule::id),
            Some(id)
        );
        assert!(design.validate_structure().is_ok());
    }

    #[test]
    fn rejects_duplicate_module_ids_without_replacing_the_existing_module() {
        let mut design = design();

        design
            .insert_module(TestModule::new(1, "first"))
            .expect("first insertion succeeds");

        assert!(design
            .insert_module(TestModule::new(1, "second"))
            .is_err());

        assert_eq!(
            design.module(HdlirModuleId::new(1).expect("non-zero ID"))
                .map(DesignModule::name),
            Some("first")
        );
    }

    #[test]
    fn rejects_duplicate_module_names() {
        let mut design = design();

        design
            .insert_module(TestModule::new(1, "worker"))
            .expect("first insertion succeeds");

        assert!(design
            .insert_module(TestModule::new(2, "worker"))
            .is_err());

        assert_eq!(design.module_count(), 1);
    }

    #[test]
    fn entry_module_must_already_be_registered() {
        let mut design = design();
        let missing = HdlirModuleId::new(7).expect("non-zero ID");

        assert!(design.set_entry_module(missing).is_err());
        assert_eq!(design.entry_module_id(), None);

        design
            .insert_module(TestModule::new(7, "top"))
            .expect("insert succeeds");

        design
            .set_entry_module(missing)
            .expect("registered module can be selected");

        assert_eq!(design.entry_module_id(), Some(missing));
        assert!(design.validate_complete().is_ok());
    }

    #[test]
    fn removing_the_entry_module_clears_the_entry_designation() {
        let mut design = design();
        let id = HdlirModuleId::new(1).expect("non-zero ID");

        design
            .insert_module(TestModule::new(1, "top"))
            .expect("insert succeeds");
        design.set_entry_module(id).expect("entry selection succeeds");

        assert!(design.remove_module(id).is_some());
        assert_eq!(design.entry_module_id(), None);
        assert!(design.validate_structure().is_ok());
    }

    #[test]
    fn rejects_empty_names() {
        assert!(HdlirDesign::<TestModule>::new(
            HdlirDesignId::new(1).expect("non-zero ID"),
            "   ",
        )
        .is_err());

        let mut design = design();
        assert!(design.insert_module(TestModule::new(1, " ")).is_err());
        assert!(design.is_empty());
    }

    #[test]
    fn module_iteration_is_in_identity_order() {
        let mut design = design();

        design
            .insert_module(TestModule::new(9, "nine"))
            .expect("insert succeeds");
        design
            .insert_module(TestModule::new(2, "two"))
            .expect("insert succeeds");
        design
            .insert_module(TestModule::new(5, "five"))
            .expect("insert succeeds");

        let ids: Vec<u128> = design
            .modules()
            .map(|(id, _)| id.get())
            .collect();

        assert_eq!(ids, vec![2, 5, 9]);
    }

    #[test]
    fn dynamic_registry_handles_many_modules_without_a_hardware_ceiling() {
        let mut design = design();

        for id in 1_u128..=2_000 {
            design
                .insert_module(TestModule::new(
                    id,
                    &format!("module_{id}"),
                ))
                .expect("unique module insertion succeeds");
        }

        assert_eq!(design.module_count(), 2_000);
        assert!(design.validate_structure().is_ok());
    }
}
