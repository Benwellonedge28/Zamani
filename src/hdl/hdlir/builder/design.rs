
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani HDLIR — Checked Design Builder
//!
//! Path: `src/hdl/hdlir/builder/design.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: Rust 1.97.
//! Safety: Safe Rust only.
//! Status: Proposed implementation; freeze after repository integration tests.
//!
//! ## Purpose
//!
//! Provide a checked construction API over the canonical `HdlirDesign`
//! registry. This builder coordinates design creation, module registration,
//! entry-module selection, removal, and final structural validation.
//!
//! ## Authority
//!
//! - `grammar/DESIGN.md`
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4`
//! - `grammar/Zamani.g4`
//! - `src/hdl/api.rs`
//! - `src/hdl/hdlir/HDLIR-SPEC.md`
//! - `src/hdl/hdlir/errors.rs`
//! - `src/hdl/hdlir/ids.rs`
//! - `src/hdl/hdlir/ir/design.rs`
//! - `src/zuir/ZUIR-SPEC.md`
//!
//! The grammar owns syntax. The frontend owns source-level semantics.
//! ZUIR owns the universal computational representation. HDLIR owns the
//! hardware-domain graph. This builder owns construction orchestration only.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - The checked design-builder API.
//! - Construction-stage completeness policy.
//! - Delegation of module registration and identity checks to `HdlirDesign`.
//! - Explicit finalization of structurally valid or complete designs.
//!
//! This module does not own:
//!
//! - A second design or module registry.
//! - Module internals, ports, operations, or connectivity.
//! - ID allocation or identity persistence.
//! - Source parsing, name resolution, or semantic validation.
//! - Target discovery, resource allocation, synthesis, placement, or routing.
//! - Serialization or transformations.
//!
//! ## Integration contract
//!
//! The canonical design graph is `crate::hdl::hdlir::ir::design::HdlirDesign`.
//! Module definitions must implement its `DesignModule` trait.
//!
//! All insertion, duplicate detection, entry-point selection, and structural
//! validation delegate to the canonical design graph. This builder must not
//! duplicate or weaken those invariants.
//!
//! ## Construction states
//!
//! A design may be structurally valid while still being incomplete.
//!
//! `build_structural` permits a valid intermediate construction state.
//! `build` requires a registered entry module and at least one module.
//!
//! Neither operation claims that module bodies are type-correct, that the
//! design is synthesizable, or that a particular target can realize it.
//!
//! ## Scalability
//!
//! No fixed module count or hardware capacity is imposed here.
//! Storage grows according to the canonical design registry and available
//! resources. Identifier exhaustion and allocation failure must never be
//! interpreted as successful construction.
//!
//! ## Safety
//!
//! No unsafe Rust is used. All invalid construction requests return explicit
//! errors. This builder does not silently replace duplicate modules or infer
//! an entry point from registry ordering.

use crate::hdl::hdlir::errors::HdlirResult;
use crate::hdl::hdlir::ids::{HdlirDesignId, HdlirModuleId};
use crate::hdl::hdlir::ir::design::{DesignModule, HdlirDesign};

/// Checked builder for a hardware-domain design.
///
/// `M` is the canonical module-definition type used by the caller. This
/// generic parameter avoids coupling design construction to an assumed
/// module representation or constructor.
///
/// The builder owns one canonical `HdlirDesign<M>` and delegates all
/// registry invariants to that type.
///
/// # Examples
///
/// ```ignore
/// let builder = DesignBuilder::<MyModule>::new(design_id, "top")?;
/// let design = builder
///     .insert_module(top_module)?
///     .set_entry_module(top_module_id)?
///     .build()?;
/// # Ok::<(), crate::hdl::hdlir::errors::HdlirError>(())
/// ```
#[derive(Debug)]
pub struct DesignBuilder<M: DesignModule> {
    design: HdlirDesign<M>,
}

impl<M: DesignModule> DesignBuilder<M> {
    /// Create a builder for a new, empty design.
    ///
    /// The supplied identity must already have been allocated by the
    /// caller according to the HDLIR identity contract.
    ///
    /// This method does not allocate IDs, parse source, or infer names.
    ///
    /// # Errors
    ///
    /// Returns an error if the logical design name is empty or
    /// whitespace-only.
    pub fn new(
        id: HdlirDesignId,
        name: impl Into<String>,
    ) -> HdlirResult<Self> {
        Ok(Self {
            design: HdlirDesign::new(id, name)?,
        })
    }

    /// Return the identity of the design being constructed.
    #[must_use]
    pub fn design_id(&self) -> HdlirDesignId {
        self.design.id()
    }

    /// Return the logical name of the design being constructed.
    #[must_use]
    pub fn design_name(&self) -> &str {
        self.design.name().as_str()
    }

    /// Return the number of registered modules.
    ///
    /// This is the backing collection's host-sized count, not a language-level
    /// hardware limit.
    #[must_use]
    pub fn module_count(&self) -> usize {
        self.design.module_count()
    }

    /// Return whether the design has no registered modules.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.design.is_empty()
    }

    /// Return the currently selected entry-module identity, if any.
    #[must_use]
    pub fn entry_module_id(&self) -> Option<HdlirModuleId> {
        self.design.entry_module_id()
    }

    /// Borrow the canonical design under construction.
    ///
    /// This is a read-only view. Use the builder methods for mutations so
    /// every public mutation passes through the established registry checks.
    #[must_use]
    pub fn as_design(&self) -> &HdlirDesign<M> {
        &self.design
    }

    /// Look up a registered module by identity.
    #[must_use]
    pub fn module(&self, id: HdlirModuleId) -> Option<&M> {
        self.design.module(id)
    }

    /// Look up a registered module by its exact logical name.
    ///
    /// Name resolution and source-language scoping remain frontend concerns.
    #[must_use]
    pub fn module_by_name(&self, name: &str) -> Option<&M> {
        self.design.module_by_name(name)
    }

    /// Iterate over registered modules in deterministic identity order.
    ///
    /// This does not allocate a secondary collection.
    pub fn modules(&self) -> impl Iterator<Item = (&HdlirModuleId, &M)> {
        self.design.modules()
    }

    /// Register a module in the design.
    ///
    /// The canonical registry checks that its identity and logical name are
    /// unique. A failed insertion leaves the existing registry unchanged.
    ///
    /// Module internals remain owned by `M`; this builder does not copy or
    /// reinterpret them.
    ///
    /// # Errors
    ///
    /// Returns the canonical registry error for a duplicate identity, a
    /// duplicate name, or an invalid module name.
    pub fn insert_module(mut self, module: M) -> HdlirResult<Self> {
        self.design.insert_module(module)?;
        Ok(self)
    }

    /// Register a module using a mutable builder reference.
    ///
    /// This variant supports incremental construction without consuming the
    /// builder. On failure, the canonical registry remains unchanged.
    ///
    /// # Errors
    ///
    /// Returns the canonical registry error if registration fails.
    pub fn try_insert_module(&mut self, module: M) -> HdlirResult<&mut Self> {
        self.design.insert_module(module)?;
        Ok(self)
    }

    /// Select a registered module as the design entry point.
    ///
    /// Entry selection is explicit; it is never inferred from insertion
    /// order, identity order, or module naming.
    ///
    /// # Errors
    ///
    /// Returns an error if the requested module is not registered.
    pub fn set_entry_module(mut self, id: HdlirModuleId) -> HdlirResult<Self> {
        self.design.set_entry_module(id)?;
        Ok(self)
    }

    /// Select the entry module using a mutable builder reference.
    ///
    /// # Errors
    ///
    /// Returns an error if the requested module is not registered. A failed
    /// selection leaves the previous entry designation unchanged.
    pub fn try_set_entry_module(
        &mut self,
        id: HdlirModuleId,
    ) -> HdlirResult<&mut Self> {
        self.design.set_entry_module(id)?;
        Ok(self)
    }

    /// Clear the current entry-module designation.
    ///
    /// The design may subsequently be finalized with `build_structural`,
    /// but `build` will require an entry module to be selected again.
    pub fn clear_entry_module(&mut self) {
        self.design.clear_entry_module();
    }

    /// Remove a registered module.
    ///
    /// Removing the selected entry module also clears the entry designation,
    /// as specified by the canonical design graph.
    ///
    /// The caller remains responsible for references to the removed module
    /// held outside the design registry.
    ///
    /// Returns the removed module, or `None` if the ID was not registered.
    pub fn remove_module(&mut self, id: HdlirModuleId) -> Option<M> {
        self.design.remove_module(id)
    }

    /// Validate the current design's structural registry invariants.
    ///
    /// This checks module identity consistency, unique logical names, and
    /// entry-point membership. It does not replace operation, type,
    /// connectivity, process, clock, or resource verification.
    ///
    /// # Errors
    ///
    /// Returns the canonical structured HDLIR error if an invariant fails.
    pub fn validate_structure(&self) -> HdlirResult<()> {
        self.design.validate_structure()
    }

    /// Finalize a structurally valid design that may be incomplete.
    ///
    /// This operation permits an empty design or a design without an entry
    /// module, provided its structural registry invariants hold. It is
    /// intended for intermediate construction, import, or staged lowering.
    ///
    /// Consumers requiring an executable or realizable top-level design
    /// must use `build` and the relevant downstream verification stages.
    ///
    /// # Errors
    ///
    /// Returns the canonical structural validation error if the registry
    /// violates its invariants.
    pub fn build_structural(self) -> HdlirResult<HdlirDesign<M>> {
        self.design.validate_structure()?;
        Ok(self.design)
    }

    /// Finalize a complete design with at least one module and a valid entry.
    ///
    /// The canonical graph validates structural invariants and completeness.
    /// No target feasibility or synthesis guarantee is implied.
    ///
    /// # Errors
    ///
    /// Returns an error if the design is empty, has no entry module, or
    /// violates a structural invariant.
    pub fn build(self) -> HdlirResult<HdlirDesign<M>> {
        self.design.validate_complete()?;
        Ok(self.design)
    }
}

#[cfg(test)]
mod tests {
    use super::DesignBuilder;
    use crate::hdl::hdlir::errors::HdlirResult;
    use crate::hdl::hdlir::ids::{HdlirDesignId, HdlirModuleId};
    use crate::hdl::hdlir::ir::design::DesignModule;

    #[derive(Debug, Eq, PartialEq)]
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

    fn builder() -> HdlirResult<DesignBuilder<TestModule>> {
        DesignBuilder::new(
            HdlirDesignId::new(1).expect("non-zero design ID"),
            "test_design",
        )
    }

    #[test]
    fn builder_creates_a_valid_empty_construction_state() {
        let builder = builder().expect("builder creation succeeds");

        assert!(builder.is_empty());
        assert_eq!(builder.module_count(), 0);
        assert_eq!(builder.entry_module_id(), None);
        assert!(builder.validate_structure().is_ok());
        assert!(builder.build().is_err());
        assert!(builder.build_structural().is_ok());
    }

    #[test]
    fn builder_registers_modules_and_selects_entry() {
        let top = TestModule::new(1, "top");
        let top_id = top.id;

        let design = builder()
            .expect("builder creation succeeds")
            .insert_module(top)
            .expect("module insertion succeeds")
            .set_entry_module(top_id)
            .expect("entry selection succeeds")
            .build()
            .expect("complete design builds");

        assert_eq!(design.module_count(), 1);
        assert_eq!(design.entry_module_id(), Some(top_id));
        assert!(design.validate_complete().is_ok());
    }

    #[test]
    fn duplicate_module_registration_is_rejected() {
        let mut builder = builder().expect("builder creation succeeds");

        builder
            .try_insert_module(TestModule::new(1, "first"))
            .expect("first insertion succeeds");

        assert!(
            builder
                .try_insert_module(TestModule::new(1, "second"))
                .is_err()
        );

        assert_eq!(builder.module_count(), 1);
        assert_eq!(
            builder
                .module(HdlirModuleId::new(1).expect("non-zero ID"))
                .map(DesignModule::name),
            Some("first")
        );
    }

    #[test]
    fn duplicate_module_names_are_rejected() {
        let mut builder = builder().expect("builder creation succeeds");

        builder
            .try_insert_module(TestModule::new(1, "worker"))
            .expect("first insertion succeeds");

        assert!(
            builder
                .try_insert_module(TestModule::new(2, "worker"))
                .is_err()
        );

        assert_eq!(builder.module_count(), 1);
    }

    #[test]
    fn missing_entry_module_is_rejected_without_mutation() {
        let mut builder = builder().expect("builder creation succeeds");
        let missing = HdlirModuleId::new(99).expect("non-zero ID");

        assert!(builder.try_set_entry_module(missing).is_err());
        assert_eq!(builder.entry_module_id(), None);
    }

    #[test]
    fn removing_the_entry_module_clears_its_designation() {
        let mut builder = builder().expect("builder creation succeeds");
        let top = TestModule::new(1, "top");
        let top_id = top.id;

        builder
            .try_insert_module(top)
            .expect("insertion succeeds");
        builder
            .try_set_entry_module(top_id)
            .expect("entry selection succeeds");

        assert!(builder.remove_module(top_id).is_some());
        assert_eq!(builder.entry_module_id(), None);
        assert!(builder.validate_structure().is_ok());
    }

    #[test]
    fn structural_finalization_does_not_require_an_entry() {
        let design = builder()
            .expect("builder creation succeeds")
            .insert_module(TestModule::new(1, "worker"))
            .expect("module insertion succeeds")
            .build_structural()
            .expect("structural finalization succeeds");

        assert_eq!(design.entry_module_id(), None);
        assert!(design.validate_structure().is_ok());
        assert!(design.validate_complete().is_err());
    }

    #[test]
    fn module_iteration_is_deterministic() {
        let mut builder = builder().expect("builder creation succeeds");

        builder
            .try_insert_module(TestModule::new(9, "nine"))
            .expect("insertion succeeds");
        builder
            .try_insert_module(TestModule::new(2, "two"))
            .expect("insertion succeeds");
        builder
            .try_insert_module(TestModule::new(5, "five"))
            .expect("insertion succeeds");

        let ids: Vec<u128> = builder
            .modules()
            .map(|(id, _)| id.get())
            .collect();

        assert_eq!(ids, vec![2, 5, 9]);
    }

    #[test]
    fn builder_scales_with_dynamic_module_count() {
        let mut builder = builder().expect("builder creation succeeds");

        for id in 1_u128..=2_000 {
            builder
                .try_insert_module(TestModule::new(
                    id,
                    &format!("module_{id}"),
                ))
                .expect("unique module insertion succeeds");
        }

        assert_eq!(builder.module_count(), 2_000);
        assert!(builder.validate_structure().is_ok());
    }
}
