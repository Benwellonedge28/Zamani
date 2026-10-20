
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani HDLIR — Checked Logical Module Builder
//!
//! Path: `src/hdl/hdlir/builder/module.rs`
//! Edition: Rust 2021.
//! Minimum supported Rust version: Rust 1.97.
//! Safety: Safe Rust only.
//! Status: Proposed implementation; freeze after repository integration
//! tests and the required workspace checks pass.
//!
//! ## Purpose
//!
//! Construct a logical HDLIR module through a checked, incremental API.
//! The builder delegates storage and local invariants to the canonical
//! `HdlirModule` defined in `ir/module.rs`.
//!
//! It supports:
//!
//! - Stable caller-supplied HDLIR module identity.
//! - Exact logical module names.
//! - Ordered parameter declarations.
//! - Ordered references to logical port declarations.
//! - Ordered references to body regions.
//! - Incremental or consuming builder workflows.
//! - Explicit structural validation and finalization.
//!
//! ## Architectural authority
//!
//! - `grammar/DESIGN.md`
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4`
//! - `grammar/hdl/ports.g4`
//! - `grammar/specification/semantic-model.md`
//! - `src/hdl/api.rs`
//! - `src/hdl/ids.rs`
//! - `src/hdl/hdlir/HDLIR-SPEC.md`
//! - `src/hdl/hdlir/errors.rs`
//! - `src/hdl/hdlir/ids.rs`
//! - `src/hdl/hdlir/ir/design.rs`
//! - `src/hdl/hdlir/ir/module.rs`
//! - `src/hdl/hdlir/ir/ports.rs`
//! - `src/hdl/hdlir/ir/regions.rs`
//! - `src/hdl/hdlir/types/`
//! - `src/hdl/hdlir/values/`
//! - `src/hdl/hdlir/builder/design.rs`
//! - `src/zuir/ZUIR-SPEC.md`
//!
//! The grammar owns source syntax. The frontend owns source-level semantic
//! truth. ZUIR owns the universal computational representation. HDLIR owns
//! the hardware-domain representation. This builder owns construction
//! orchestration only.
//!
//! ## Ownership
//!
//! This builder owns one `HdlirModule` while it is being constructed.
//! It does not introduce another module data model or duplicate the module's
//! parameter, port-reference, or body-region collections.
//!
//! It does not own:
//!
//! - Source parsing, source ASTs, name resolution, or semantic analysis.
//! - The authoritative type definitions or symbolic-width evaluator.
//! - Port declarations, port types, connectivity, or physical pins.
//! - Region, block, operation, value, process, clock, or memory definitions.
//! - Graph-wide reference resolution or whole-design verification.
//! - Device discovery, resource allocation, synthesis, placement, or routing.
//! - Serialization, persistent identities, or cross-build identity stability.
//!
//! ## Construction contract
//!
//! Each successful addition delegates to the canonical module API. A failed
//! addition does not append a partial declaration or reference.
//!
//! `build()` validates the module's local structural invariants. It does not
//! claim that referenced ports, regions, types, or values exist in the
//! enclosing graph. Those checks require graph-wide verification.
//!
//! An empty module is a legitimate intermediate construction state. Whether
//! it is complete or realizable is decided by the relevant semantic,
//! verification, and downstream compilation contracts.
//!
//! ## Scalability
//!
//! No fixed parameter, port, region, width, or module-count ceiling is
//! introduced here. Collection growth is governed by available resources
//! and the underlying Rust representation.
//!
//! IDs are opaque identities, never collection indexes or physical-resource
//! numbers. Symbolic widths and parameter values remain owned by their
//! canonical type and value representations.
//!
//! This builder does not claim that literally infinite input can be
//! materialized. Operational budgets belong to the applicable compilation
//! and verification boundaries. A budget failure must remain an explicit
//! failure, never successful finalization.
//!
//! ## Integration contract
//!
//! - `builder/mod.rs` registers this file as `pub mod module;`.
//! - `builder/design.rs` accepts the finalized `HdlirModule` through its
//!   existing `DesignModule` implementation.
//! - `ir/module.rs` remains the sole owner of module storage and local
//!   structural invariants.
//! - `ir/ports.rs` owns port definitions; this builder stores their IDs.
//! - `ir/regions.rs` owns region definitions; this builder stores their IDs.
//! - The whole-graph verifier resolves references and verifies ownership,
//!   type relationships, connectivity, and dialect-specific constraints.
//! - Semantic lowering creates declarations and IDs from canonical validated
//!   frontend facts; this builder does not parse source or resolve names.
//!
//! ## Failure behavior
//!
//! Invalid module names, invalid parameter declarations, duplicate parameter
//! names, repeated port IDs, repeated region IDs, and violated local
//! invariants return structured `HdlirError` values.
//!
//! The builder never silently replaces an existing declaration or infers
//! module identity, parameter bindings, port ownership, or entry-point status.
//!
//! ## Freeze criteria
//!
//! - Uses the canonical `HdlirModule` and existing HDLIR IDs.
//! - Supports consuming and incremental construction.
//! - Preserves declaration and reference ordering.
//! - Failed additions leave the module unchanged.
//! - Finalization revalidates local structural invariants.
//! - Works with the existing `DesignModule` and `DesignBuilder` contracts.
//! - Tests cover invalid inputs, duplicates, ordering, failed mutations,
//!   incremental construction, finalization, and large dynamic collections.
//! - Workspace formatting, compilation, tests, and Clippy pass.

use crate::hdl::hdlir::errors::HdlirResult;
use crate::hdl::hdlir::ids::{
    HdlirModuleId, HdlirPortId, HdlirRegionId, HdlirTypeId, HdlirValueId,
};
use crate::hdl::hdlir::ir::module::{
    HdlirModule, HdlirModuleParameter,
};

/// Checked builder for a logical HDLIR module.
///
/// `ModuleBuilder` delegates module storage and local invariant enforcement
/// to [`HdlirModule`]. It does not create an alternative representation.
///
/// Use the consuming methods when each construction step should produce a
/// new builder value. Use the `try_` methods when a builder must be retained
/// and updated incrementally.
///
/// # Example
///
/// ```ignore
/// let module = ModuleBuilder::new(module_id, "top")?
///     .add_parameter(parameter)?
///     .add_port(input_port_id)?
///     .add_body_region(body_region_id)?
///     .build()?;
/// # Ok::<(), crate::hdl::hdlir::errors::HdlirError>(())
/// ```
///
/// The example assumes that the IDs and parameter were created by their
/// canonical HDLIR owners. This builder does not allocate IDs or resolve
/// external references.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModuleBuilder {
    module: HdlirModule,
}

impl ModuleBuilder {
    /// Create a builder for a new logical module.
    ///
    /// The supplied identity must already have been allocated by the caller.
    /// The logical name is retained exactly if it is valid.
    ///
    /// # Errors
    ///
    /// Returns a structured HDLIR error if the module name is empty or
    /// consists only of whitespace.
    pub fn new(
        id: HdlirModuleId,
        name: impl Into<String>,
    ) -> HdlirResult<Self> {
        Ok(Self {
            module: HdlirModule::new(id, name)?,
        })
    }

    /// Start building from an existing module.
    ///
    /// This method validates local invariants before accepting the module.
    /// It does not validate references against an enclosing design graph.
    ///
    /// # Errors
    ///
    /// Returns an error if the supplied module violates a local structural
    /// invariant.
    pub fn from_module(module: HdlirModule) -> HdlirResult<Self> {
        module.validate_structure()?;
        Ok(Self { module })
    }

    /// Return the module's stable HDLIR identity.
    #[must_use]
    pub const fn id(&self) -> HdlirModuleId {
        self.module.id()
    }

    /// Return the module's exact logical name.
    #[must_use]
    pub fn name(&self) -> &str {
        self.module.name_str()
    }

    /// Return the number of parameter declarations.
    ///
    /// This is the length of the underlying host collection, not a
    /// language-level parameter limit.
    #[must_use]
    pub fn parameter_count(&self) -> usize {
        self.module.parameter_count()
    }

    /// Return the number of referenced port declarations.
    ///
    /// This is a host collection length, not a hardware-capacity limit.
    #[must_use]
    pub fn port_count(&self) -> usize {
        self.module.port_count()
    }

    /// Return the number of referenced body regions.
    ///
    /// This is a host collection length, not a hardware-capacity limit.
    #[must_use]
    pub fn body_region_count(&self) -> usize {
        self.module.body_region_count()
    }

    /// Return whether the module has no parameters, port references, or
    /// body-region references.
    ///
    /// An empty module is permitted during incremental construction.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.parameter_count() == 0
            && self.port_count() == 0
            && self.body_region_count() == 0
    }

    /// Borrow the module being constructed.
    ///
    /// The returned reference is read-only. Mutations should pass through
    /// this builder or the canonical module API.
    #[must_use]
    pub const fn as_module(&self) -> &HdlirModule {
        &self.module
    }

    /// Return the parameter declarations in declaration order.
    #[must_use]
    pub fn parameters(&self) -> &[HdlirModuleParameter] {
        self.module.parameters()
    }

    /// Return port references in declaration order.
    #[must_use]
    pub fn ports(&self) -> &[HdlirPortId] {
        self.module.ports()
    }

    /// Return body-region references in declaration order.
    #[must_use]
    pub fn body_regions(&self) -> &[HdlirRegionId] {
        self.module.body_regions()
    }

    /// Find a parameter by its exact name.
    ///
    /// No normalization, case folding, or source-level name resolution
    /// is performed.
    #[must_use]
    pub fn parameter(&self, name: &str) -> Option<&HdlirModuleParameter> {
        self.module.parameter(name)
    }

    /// Add a parameter declaration, consuming the builder.
    ///
    /// The canonical module validates the declaration and rejects a
    /// duplicate exact parameter name. Declaration order is preserved.
    ///
    /// # Errors
    ///
    /// Returns the canonical module error if the parameter name is
    /// duplicated or otherwise invalid.
    #[must_use = "handle the error or retain the returned builder"]
    pub fn add_parameter(
        mut self,
        parameter: HdlirModuleParameter,
    ) -> HdlirResult<Self> {
        self.try_add_parameter(parameter)?;
        Ok(self)
    }

    /// Add a parameter declaration through a mutable builder reference.
    ///
    /// A failed insertion leaves the module unchanged.
    ///
    /// # Errors
    ///
    /// Returns the canonical module error if the parameter name is
    /// duplicated or otherwise invalid.
    pub fn try_add_parameter(
        &mut self,
        parameter: HdlirModuleParameter,
    ) -> HdlirResult<&mut Self> {
        self.module.add_parameter(parameter)?;
        Ok(self)
    }

    /// Construct and add a parameter declaration, consuming the builder.
    ///
    /// This convenience method delegates declaration construction to
    /// [`HdlirModuleParameter::new`] and insertion to the canonical module.
    ///
    /// `type_id` must identify a type in the relevant HDLIR type namespace.
    /// `default_value`, when present, must identify a value in the relevant
    /// HDLIR value namespace. Existence and type compatibility are verified
    /// against the enclosing graph, not by this builder.
    ///
    /// # Errors
    ///
    /// Returns an error if the declaration name is invalid, the parameter
    /// name is duplicated, or the canonical module rejects the declaration.
    #[must_use = "handle the error or retain the returned builder"]
    pub fn parameter(
        self,
        name: impl Into<String>,
        type_id: HdlirTypeId,
        default_value: Option<HdlirValueId>,
    ) -> HdlirResult<Self> {
        self.add_parameter(HdlirModuleParameter::new(
            name,
            type_id,
            default_value,
        )?)
    }

    /// Construct and add a parameter declaration using a mutable builder.
    ///
    /// This method is convenient for incremental lowering. It preserves
    /// declaration order and does not evaluate symbolic parameter values.
    ///
    /// # Errors
    ///
    /// Returns an error if the declaration is invalid or its exact name
    /// duplicates an existing parameter.
    pub fn try_parameter(
        &mut self,
        name: impl Into<String>,
        type_id: HdlirTypeId,
        default_value: Option<HdlirValueId>,
    ) -> HdlirResult<&mut Self> {
        let parameter =
            HdlirModuleParameter::new(name, type_id, default_value)?;
        self.try_add_parameter(parameter)
    }

    /// Add a logical port reference, consuming the builder.
    ///
    /// The canonical module rejects a repeated port ID. The reference is
    /// appended only after the local uniqueness check succeeds.
    ///
    /// This method does not create a port declaration. Use the canonical
    /// port subsystem to construct the declaration and register it in the
    /// containing graph.
    ///
    /// # Errors
    ///
    /// Returns an error if the port ID is already referenced by this module.
    #[must_use = "handle the error or retain the returned builder"]
    pub fn add_port(
        mut self,
        port_id: HdlirPortId,
    ) -> HdlirResult<Self> {
        self.try_add_port(port_id)?;
        Ok(self)
    }

    /// Add a logical port reference through a mutable builder reference.
    ///
    /// A failed insertion leaves the existing port-reference list unchanged.
    ///
    /// # Errors
    ///
    /// Returns an error if the port ID is already referenced.
    pub fn try_add_port(
        &mut self,
        port_id: HdlirPortId,
    ) -> HdlirResult<&mut Self> {
        self.module.add_port(port_id)?;
        Ok(self)
    }

    /// Add a body-region reference, consuming the builder.
    ///
    /// The canonical module rejects repeated region IDs. Region ownership,
    /// existence, and contents are checked against the enclosing graph.
    ///
    /// This method does not create or mutate a region.
    ///
    /// # Errors
    ///
    /// Returns an error if the region ID is already referenced by this
    /// module.
    #[must_use = "handle the error or retain the returned builder"]
    pub fn add_body_region(
        mut self,
        region_id: HdlirRegionId,
    ) -> HdlirResult<Self> {
        self.try_add_body_region(region_id)?;
        Ok(self)
    }

    /// Add a body-region reference through a mutable builder reference.
    ///
    /// A failed insertion leaves the existing region-reference list
    /// unchanged.
    ///
    /// # Errors
    ///
    /// Returns an error if the region ID is already referenced.
    pub fn try_add_body_region(
        &mut self,
        region_id: HdlirRegionId,
    ) -> HdlirResult<&mut Self> {
        self.module.add_body_region(region_id)?;
        Ok(self)
    }

    /// Validate local structural invariants without consuming the builder.
    ///
    /// This delegates to `HdlirModule::validate_structure`. It does not
    /// resolve port, region, type, or value references against a containing
    /// design.
    ///
    /// # Errors
    ///
    /// Returns a structured HDLIR error if a local invariant is violated.
    pub fn validate_structure(&self) -> HdlirResult<()> {
        self.module.validate_structure()
    }

    /// Finalize the module after validating its local structure.
    ///
    /// This operation does not assert that the module has a body, has ports,
    /// is synthesizable, or is feasible on any target. Those properties are
    /// determined by the applicable language semantics, registered dialect
    /// contracts, graph verifier, and downstream realization stages.
    ///
    /// # Errors
    ///
    /// Returns a structured HDLIR error if local structural validation fails.
    pub fn build(self) -> HdlirResult<HdlirModule> {
        self.module.validate_structure()?;
        Ok(self.module)
    }
}

#[cfg(test)]
mod tests {
    use super::ModuleBuilder;
    use crate::hdl::hdlir::ids::{
        HdlirModuleId, HdlirPortId, HdlirRegionId, HdlirTypeId,
        HdlirValueId,
    };
    use crate::hdl::hdlir::ir::design::DesignModule;
    use crate::hdl::hdlir::ir::module::{
        HdlirModule, HdlirModuleParameter,
    };

    fn module_id(value: u128) -> HdlirModuleId {
        HdlirModuleId::new(value).expect("test ID must be non-zero")
    }

    fn port_id(value: u128) -> HdlirPortId {
        HdlirPortId::new(value).expect("test ID must be non-zero")
    }

    fn region_id(value: u128) -> HdlirRegionId {
        HdlirRegionId::new(value).expect("test ID must be non-zero")
    }

    fn type_id(value: u128) -> HdlirTypeId {
        HdlirTypeId::new(value).expect("test ID must be non-zero")
    }

    fn value_id(value: u128) -> HdlirValueId {
        HdlirValueId::new(value).expect("test ID must be non-zero")
    }

    fn builder() -> ModuleBuilder {
        ModuleBuilder::new(module_id(1), "top")
            .expect("valid module builder")
    }

    #[test]
    fn creates_an_empty_module_builder() {
        let builder = builder();

        assert_eq!(builder.id(), module_id(1));
        assert_eq!(builder.name(), "top");
        assert!(builder.is_empty());
        assert_eq!(builder.parameter_count(), 0);
        assert_eq!(builder.port_count(), 0);
        assert_eq!(builder.body_region_count(), 0);
        assert!(builder.validate_structure().is_ok());
    }

    #[test]
    fn rejects_empty_and_whitespace_only_module_names() {
        assert!(ModuleBuilder::new(module_id(1), "").is_err());
        assert!(ModuleBuilder::new(module_id(1), " \t\n ").is_err());
    }

    #[test]
    fn preserves_the_exact_module_name() {
        let builder = ModuleBuilder::new(module_id(1), " Top ")
            .expect("non-whitespace name is accepted by the canonical API");

        assert_eq!(builder.name(), " Top ");
    }

    #[test]
    fn adds_parameter_and_preserves_declaration_order() {
        let module = builder()
            .add_parameter(
                HdlirModuleParameter::new("WIDTH", type_id(1), None)
                    .expect("valid parameter"),
            )
            .expect("first parameter is accepted")
            .add_parameter(
                HdlirModuleParameter::new(
                    "DEPTH",
                    type_id(2),
                    Some(value_id(1)),
                )
                .expect("valid parameter"),
            )
            .expect("second parameter is accepted")
            .build()
            .expect("module builds");

        assert_eq!(
            module
                .parameters()
                .iter()
                .map(HdlirModuleParameter::name)
                .collect::<Vec<_>>(),
            vec!["WIDTH", "DEPTH"]
        );
    }

    #[test]
    fn rejects_duplicate_parameter_without_mutating_existing_module() {
        let mut builder = builder();

        builder
            .try_parameter("WIDTH", type_id(1), None)
            .expect("first parameter is accepted");

        let before = builder.parameters().to_vec();

        assert!(
            builder
                .try_parameter("WIDTH", type_id(2), None)
                .is_err()
        );

        assert_eq!(builder.parameters(), before.as_slice());
        assert_eq!(builder.parameter_count(), 1);
    }

    #[test]
    fn parameter_lookup_uses_exact_name_matching() {
        let mut builder = builder();

        builder
            .try_parameter("WIDTH", type_id(1), None)
            .expect("uppercase parameter is accepted");

        builder
            .try_parameter("width", type_id(1), None)
            .expect("distinct exact name is accepted");

        assert!(builder.parameter("WIDTH").is_some());
        assert!(builder.parameter("width").is_some());
        assert!(builder.parameter("Width").is_none());
    }

    #[test]
    fn rejects_invalid_parameter_name() {
        let mut builder = builder();

        assert!(
            builder
                .try_parameter(" \t ", type_id(1), None)
                .is_err()
        );

        assert_eq!(builder.parameter_count(), 0);
    }

    #[test]
    fn adds_ports_in_insertion_order() {
        let module = builder()
            .add_port(port_id(9))
            .expect("first port is accepted")
            .add_port(port_id(2))
            .expect("second port is accepted")
            .add_port(port_id(5))
            .expect("third port is accepted")
            .build()
            .expect("module builds");

        assert_eq!(
            module.ports(),
            &[port_id(9), port_id(2), port_id(5)]
        );
    }

    #[test]
    fn rejects_duplicate_port_reference_without_mutation() {
        let mut builder = builder();

        builder
            .try_add_port(port_id(4))
            .expect("first port reference is accepted");

        assert!(builder.try_add_port(port_id(4)).is_err());
        assert_eq!(builder.ports(), &[port_id(4)]);
        assert_eq!(builder.port_count(), 1);
    }

    #[test]
    fn adds_body_regions_in_insertion_order() {
        let module = builder()
            .add_body_region(region_id(7))
            .expect("first region is accepted")
            .add_body_region(region_id(3))
            .expect("second region is accepted")
            .build()
            .expect("module builds");

        assert_eq!(
            module.body_regions(),
            &[region_id(7), region_id(3)]
        );
    }

    #[test]
    fn rejects_duplicate_region_reference_without_mutation() {
        let mut builder = builder();

        builder
            .try_add_body_region(region_id(8))
            .expect("first region reference is accepted");

        assert!(builder.try_add_body_region(region_id(8)).is_err());
        assert_eq!(builder.body_regions(), &[region_id(8)]);
        assert_eq!(builder.body_region_count(), 1);
    }

    #[test]
    fn mutable_and_consuming_apis_produce_equivalent_modules() {
        let consuming = builder()
            .add_parameter(
                HdlirModuleParameter::new("WIDTH", type_id(1), None)
                    .expect("valid parameter"),
            )
            .expect("parameter is accepted")
            .add_port(port_id(2))
            .expect("port is accepted")
            .add_body_region(region_id(3))
            .expect("region is accepted")
            .build()
            .expect("module builds");

        let mut incremental = builder();

        incremental
            .try_parameter("WIDTH", type_id(1), None)
            .expect("parameter is accepted");
        incremental
            .try_add_port(port_id(2))
            .expect("port is accepted");
        incremental
            .try_add_body_region(region_id(3))
            .expect("region is accepted");

        let incremental = incremental.build().expect("module builds");

        assert_eq!(consuming, incremental);
    }

    #[test]
    fn accepts_a_valid_existing_module() {
        let module =
            HdlirModule::new(module_id(1), "existing").expect("valid module");

        let builder = ModuleBuilder::from_module(module)
            .expect("valid module is accepted");

        assert_eq!(builder.name(), "existing");
    }

    #[test]
    fn finalized_module_implements_the_existing_design_contract() {
        let module = builder().build().expect("module builds");

        assert_eq!(DesignModule::id(&module), module_id(1));
        assert_eq!(DesignModule::name(&module), "top");
    }

    #[test]
    fn construction_does_not_impose_a_fixed_parameter_or_port_limit() {
        let mut builder = builder();

        for index in 1_u128..=2_000 {
            builder
                .try_parameter(
                    format!("PARAM_{index}"),
                    type_id(1),
                    None,
                )
                .expect("unique parameter is accepted");

            builder
                .try_add_port(port_id(index))
                .expect("unique port reference is accepted");
        }

        assert_eq!(builder.parameter_count(), 2_000);
        assert_eq!(builder.port_count(), 2_000);
        assert!(builder.validate_structure().is_ok());
    }

    #[test]
    fn failed_addition_does_not_prevent_later_valid_construction() {
        let mut builder = builder();

        builder
            .try_add_port(port_id(1))
            .expect("first port reference is accepted");

        assert!(builder.try_add_port(port_id(1)).is_err());

        builder
            .try_add_port(port_id(2))
            .expect("different port reference is accepted");

        let module = builder.build().expect("module builds");

        assert_eq!(module.ports(), &[port_id(1), port_id(2)]);
    }
}
