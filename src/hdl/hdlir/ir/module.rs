
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani HDLIR — Logical Module Definitions
//!
//! Path: `src/hdl/hdlir/ir/module.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: 1.97.
//! Safety: Safe Rust only.
//! Status: Initial module-definition implementation; freeze after repository
//! integration and conformance tests pass.
//!
//! ## Purpose
//!
//! Represent a logical hardware module independently of physical hardware,
//! target technology, and source syntax. A module owns its HDLIR identity,
//! exact logical name, parameter declarations, ordered port references, and
//! ordered body-region references.
//!
//! ## Authority
//!
//! - `grammar/DESIGN.md`
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4`
//! - `grammar/hdl/ports.g4`
//! - `grammar/specification/semantic-model.md`
//! - `src/hdl/api.rs`
//! - `src/hdl/hdlir/HDLIR-SPEC.md`
//! - `src/hdl/hdlir/ids.rs`
//! - `src/hdl/hdlir/errors.rs`
//! - `src/hdl/hdlir/ir/design.rs`
//! - `src/zuir/ZUIR-SPEC.md`
//!
//! Grammar and frontend semantic contracts remain authoritative for source
//! meaning. ZUIR remains the universal computational representation.
//! HDLIR owns the hardware-domain representation.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - The HDLIR module identity and exact logical name.
//! - Ordered module-parameter declarations.
//! - Ordered references to the module's port declarations.
//! - Ordered references to the module's body regions.
//! - Local identity/name uniqueness and basic structural invariants.
//! - The `DesignModule` integration contract implementation.
//!
//! This module does not own:
//!
//! - Parsing, source ASTs, name resolution, or semantic validation.
//! - Port direction, signal types, connectivity, or physical pin assignment.
//! - The implementation of region bodies, blocks, operations, or values.
//! - Parameter expression evaluation or concrete width calculation.
//! - Clock synthesis, scheduling, placement, routing, or resource allocation.
//! - Target discovery, synthesis, simulation, or execution.
//! - Serialization or persistent cross-compilation identity.
//!
//! ## Scalability
//!
//! There are no language-level limits on the number of parameters, ports,
//! or body regions. Collections grow subject to available memory and the
//! limits of the running implementation.
//!
//! Parameter types and defaults are references to HDLIR-owned entities.
//! This file does not force symbolic values into concrete host integers.
//!
//! Collection lengths use the standard library's host-sized representation.
//! Such lengths describe allocated storage; they are not hardware limits.
//!
//! ## Integration
//!
//! `ir/design.rs` consumes this module through `DesignModule`.
//! The future `ir/ports.rs` owns the definitions referenced by `ports()`.
//! The future `ir/regions.rs` owns the regions referenced by `body_regions()`.
//! Parameter type and default-value definitions remain owned by the HDLIR
//! type and value subsystems.
//!
//! A containing design and the HDLIR verifier must resolve all references
//! and validate cross-object invariants before lowering or realization.
//!
//! ## Failure behavior
//!
//! Invalid names, duplicate parameter names, duplicate port references,
//! duplicate region references, and inconsistent local structure produce
//! structured errors. Failed additions leave the module unchanged.
//!
//! A structurally valid module is not necessarily type-valid, synthesizable,
//! target-feasible, or physically implementable.
//!
//! ## Freeze criteria
//!
//! - Implements `DesignModule` with stable ID and name access.
//! - Rejects empty or whitespace-only logical names.
//! - Preserves exact names without implicit normalization.
//! - Preserves declaration/reference ordering.
//! - Rejects duplicate parameter names and repeated port/region IDs.
//! - Exposes no mutable access to identity-defining fields.
//! - Uses existing HDLIR IDs and error contracts.
//! - Has positive, negative, boundary, and integration tests.
//! - Passes repository formatting, build, test, and lint checks.

use crate::hdl::hdlir::errors::{HdlirError, HdlirResult};
use crate::hdl::hdlir::ids::{
    HdlirModuleId, HdlirPortId, HdlirRegionId, HdlirTypeId, HdlirValueId,
};
use crate::hdl::hdlir::ir::design::DesignModule;

/// A checked logical module name.
///
/// Names are preserved exactly. This type does not perform source-language
/// name resolution, case folding, Unicode normalization, or trimming.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ModuleName(String);

impl ModuleName {
    /// Construct a module name.
    ///
    /// # Errors
    ///
    /// Returns an error if the name is empty or whitespace-only.
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

impl core::fmt::Display for ModuleName {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// A module parameter declaration.
///
/// `type_id` refers to a type owned by the HDLIR type subsystem.
/// `default_value`, when present, refers to a value owned by the HDLIR value
/// subsystem. The referenced value may be symbolic.
///
/// The parameter name is local to the module. Source-level resolution,
/// expression evaluation, constraint solving, and specialization belong to
/// their respective frontend or HDLIR stages.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HdlirModuleParameter {
    name: String,
    type_id: HdlirTypeId,
    default_value: Option<HdlirValueId>,
}

impl HdlirModuleParameter {
    /// Construct a parameter declaration.
    ///
    /// # Errors
    ///
    /// Returns an error if `name` is empty or whitespace-only.
    pub fn new(
        name: impl Into<String>,
        type_id: HdlirTypeId,
        default_value: Option<HdlirValueId>,
    ) -> HdlirResult<Self> {
        let name = name.into();

        if name.trim().is_empty() {
            return Err(HdlirError::invalid_input(
                "parameter_name",
                "parameter name must not be empty or whitespace-only",
            ));
        }

        Ok(Self {
            name,
            type_id,
            default_value,
        })
    }

    /// Return the exact parameter name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the referenced HDLIR type identity.
    #[must_use]
    pub const fn type_id(&self) -> HdlirTypeId {
        self.type_id
    }

    /// Return the optional default-value identity.
    #[must_use]
    pub const fn default_value(&self) -> Option<HdlirValueId> {
        self.default_value
    }
}

/// A logical hardware module definition.
///
/// A module is a logical design entity, not a physical device, FPGA region,
/// ASIC cell, or placed hardware instance. Its identity and name remain stable
/// while it is registered in a design.
///
/// Ports and body regions are stored as ordered references. Their definitions
/// and internal contents remain owned by the corresponding IR components.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HdlirModule {
    id: HdlirModuleId,
    name: ModuleName,
    parameters: Vec<HdlirModuleParameter>,
    ports: Vec<HdlirPortId>,
    body_regions: Vec<HdlirRegionId>,
}

impl HdlirModule {
    /// Construct an empty logical module.
    ///
    /// An empty module is a permitted intermediate construction state. The
    /// verifier or builder determines whether it is complete for a particular
    /// compilation stage.
    ///
    /// # Errors
    ///
    /// Returns an error if `name` is empty or whitespace-only.
    pub fn new(
        id: HdlirModuleId,
        name: impl Into<String>,
    ) -> HdlirResult<Self> {
        Ok(Self {
            id,
            name: ModuleName::new(name)?,
            parameters: Vec::new(),
            ports: Vec::new(),
            body_regions: Vec::new(),
        })
    }

    /// Return this module's stable HDLIR identity.
    #[must_use]
    pub const fn id(&self) -> HdlirModuleId {
        self.id
    }

    /// Return the module's exact logical name.
    #[must_use]
    pub fn name_value(&self) -> &ModuleName {
        &self.name
    }

    /// Return the module's exact logical name as a string slice.
    #[must_use]
    pub fn name_str(&self) -> &str {
        self.name.as_str()
    }

    /// Return the module's parameter declarations in declaration order.
    #[must_use]
    pub fn parameters(&self) -> &[HdlirModuleParameter] {
        &self.parameters
    }

    /// Return ordered references to the module's port declarations.
    ///
    /// Port direction, type, and connectivity are owned by the port and
    /// connectivity subsystems.
    #[must_use]
    pub fn ports(&self) -> &[HdlirPortId] {
        &self.ports
    }

    /// Return ordered references to the module's body regions.
    ///
    /// Region contents and operation semantics are owned by the IR region,
    /// block, and operation subsystems.
    #[must_use]
    pub fn body_regions(&self) -> &[HdlirRegionId] {
        &self.body_regions
    }

    /// Return the number of declared parameters.
    ///
    /// This is a host collection length, not a language-level capacity limit.
    #[must_use]
    pub fn parameter_count(&self) -> usize {
        self.parameters.len()
    }

    /// Return the number of referenced ports.
    #[must_use]
    pub fn port_count(&self) -> usize {
        self.ports.len()
    }

    /// Return the number of referenced body regions.
    #[must_use]
    pub fn body_region_count(&self) -> usize {
        self.body_regions.len()
    }

    /// Add a parameter declaration.
    ///
    /// Parameter names are unique within the module and compared exactly.
    /// The declaration is appended only after all checks succeed.
    ///
    /// # Errors
    ///
    /// Returns an error if a parameter with the same exact name already
    /// exists in this module.
    pub fn add_parameter(
        &mut self,
        parameter: HdlirModuleParameter,
    ) -> HdlirResult<()> {
        if self
            .parameters
            .iter()
            .any(|existing| existing.name() == parameter.name())
        {
            return Err(HdlirError::DuplicateEntity {
                entity_kind: "HDLIR module parameter name",
                identity: parameter.name().to_owned(),
                scope: Some(self.name.to_string()),
            });
        }

        self.parameters.push(parameter);
        Ok(())
    }

    /// Add a reference to a port declaration.
    ///
    /// The order of insertion is preserved. This method verifies local
    /// uniqueness only; the containing design/verifier must resolve the
    /// referenced port and check ownership and type compatibility.
    ///
    /// # Errors
    ///
    /// Returns an error if this port identity is already present.
    pub fn add_port(&mut self, port_id: HdlirPortId) -> HdlirResult<()> {
        if self.ports.contains(&port_id) {
            return Err(HdlirError::DuplicateEntity {
                entity_kind: "HDLIR module port reference",
                identity: port_id.to_string(),
                scope: Some(self.name.to_string()),
            });
        }

        self.ports.push(port_id);
        Ok(())
    }

    /// Add a reference to a body region.
    ///
    /// The order of insertion is preserved. The region's existence, ownership,
    /// and semantic validity must be checked by the containing graph verifier.
    ///
    /// # Errors
    ///
    /// Returns an error if this region identity is already present.
    pub fn add_body_region(
        &mut self,
        region_id: HdlirRegionId,
    ) -> HdlirResult<()> {
        if self.body_regions.contains(&region_id) {
            return Err(HdlirError::DuplicateEntity {
                entity_kind: "HDLIR module body-region reference",
                identity: region_id.to_string(),
                scope: Some(self.name.to_string()),
            });
        }

        self.body_regions.push(region_id);
        Ok(())
    }

    /// Find a parameter by its exact name.
    #[must_use]
    pub fn parameter(&self, name: &str) -> Option<&HdlirModuleParameter> {
        self.parameters
            .iter()
            .find(|parameter| parameter.name() == name)
    }

    /// Validate this module's local structural invariants.
    ///
    /// This does not resolve referenced ports, regions, types, or values.
    /// Those checks require the containing IR graph and belong to graph
    /// verification.
    ///
    /// # Errors
    ///
    /// Returns a structured error if any local invariant is violated.
    pub fn validate_structure(&self) -> HdlirResult<()> {
        if self.name.as_str().trim().is_empty() {
            return Err(HdlirError::invalid_invariant(
                "module_name_is_nonempty",
                format!("module {} has an empty logical name", self.id),
            ));
        }

        for (index, parameter) in self.parameters.iter().enumerate() {
            if parameter.name().trim().is_empty() {
                return Err(HdlirError::invalid_invariant(
                    "module_parameter_name_is_nonempty",
                    format!(
                        "module {} has an empty parameter name at index {index}",
                        self.id
                    ),
                ));
            }

            if self.parameters[..index]
                .iter()
                .any(|previous| previous.name() == parameter.name())
            {
                return Err(HdlirError::invalid_invariant(
                    "module_parameter_names_are_unique",
                    format!(
                        "module {} contains duplicate parameter name {:?}",
                        self.id,
                        parameter.name()
                    ),
                ));
            }
        }

        for (index, port_id) in self.ports.iter().enumerate() {
            if self.ports[..index].contains(port_id) {
                return Err(HdlirError::invalid_invariant(
                    "module_port_references_are_unique",
                    format!(
                        "module {} contains duplicate port reference {}",
                        self.id, port_id
                    ),
                ));
            }
        }

        for (index, region_id) in self.body_regions.iter().enumerate() {
            if self.body_regions[..index].contains(region_id) {
                return Err(HdlirError::invalid_invariant(
                    "module_body_region_references_are_unique",
                    format!(
                        "module {} contains duplicate body-region reference {}",
                        self.id, region_id
                    ),
                ));
            }
        }

        Ok(())
    }
}

impl DesignModule for HdlirModule {
    /// Return this module's HDLIR identity.
    fn id(&self) -> HdlirModuleId {
        self.id
    }

    /// Return this module's exact logical name.
    fn name(&self) -> &str {
        self.name.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn module_id(value: u128) -> HdlirModuleId {
        HdlirModuleId::new(value).expect("test module ID must be non-zero")
    }

    fn port_id(value: u128) -> HdlirPortId {
        HdlirPortId::new(value).expect("test port ID must be non-zero")
    }

    fn region_id(value: u128) -> HdlirRegionId {
        HdlirRegionId::new(value).expect("test region ID must be non-zero")
    }

    fn type_id(value: u128) -> HdlirTypeId {
        HdlirTypeId::new(value).expect("test type ID must be non-zero")
    }

    fn value_id(value: u128) -> HdlirValueId {
        HdlirValueId::new(value).expect("test value ID must be non-zero")
    }

    #[test]
    fn constructs_module_and_implements_design_contract() {
        let module = HdlirModule::new(module_id(1), "top")
            .expect("module should be constructed");

        assert_eq!(module.id(), module_id(1));
        assert_eq!(module.name(), "top");
        assert_eq!(module.name_str(), "top");
        assert!(module.parameters().is_empty());
        assert!(module.ports().is_empty());
        assert!(module.body_regions().is_empty());
        assert!(module.validate_structure().is_ok());
    }

    #[test]
    fn rejects_empty_and_whitespace_only_names() {
        assert!(HdlirModule::new(module_id(1), "").is_err());
        assert!(HdlirModule::new(module_id(1), " \t\n ").is_err());
        assert!(ModuleName::new("").is_err());
    }

    #[test]
    fn preserves_exact_module_and_parameter_names() {
        let mut module = HdlirModule::new(module_id(1), " Top ")
            .expect("non-whitespace name should be preserved");

        let parameter = HdlirModuleParameter::new(
            " Width ",
            type_id(1),
            Some(value_id(1)),
        )
        .expect("non-whitespace parameter name should be preserved");

        module
            .add_parameter(parameter)
            .expect("parameter should be added");

        assert_eq!(module.name(), " Top ");
        assert_eq!(module.parameters()[0].name(), " Width ");
    }

    #[test]
    fn rejects_duplicate_parameter_names_without_mutation() {
        let mut module = HdlirModule::new(module_id(1), "worker")
            .expect("module should be constructed");

        module
            .add_parameter(
                HdlirModuleParameter::new("WIDTH", type_id(1), None)
                    .expect("parameter should be constructed"),
            )
            .expect("first parameter should be added");

        let before = module.parameters().to_vec();

        let result = module.add_parameter(
            HdlirModuleParameter::new("WIDTH", type_id(2), None)
                .expect("parameter should be constructed"),
        );

        assert!(result.is_err());
        assert_eq!(module.parameters(), before.as_slice());
    }

    #[test]
    fn parameter_names_use_exact_comparison() {
        let mut module = HdlirModule::new(module_id(1), "worker")
            .expect("module should be constructed");

        module
            .add_parameter(
                HdlirModuleParameter::new("WIDTH", type_id(1), None)
                    .expect("parameter should be constructed"),
            )
            .expect("first parameter should be added");

        module
            .add_parameter(
                HdlirModuleParameter::new("width", type_id(1), None)
                    .expect("parameter should be constructed"),
            )
            .expect("distinct exact name should be accepted");

        assert_eq!(module.parameter_count(), 2);
        assert!(module.parameter("WIDTH").is_some());
        assert!(module.parameter("width").is_some());
    }

    #[test]
    fn rejects_duplicate_port_references_without_mutation() {
        let mut module = HdlirModule::new(module_id(1), "worker")
            .expect("module should be constructed");

        module
            .add_port(port_id(1))
            .expect("first port should be added");

        assert!(module.add_port(port_id(1)).is_err());
        assert_eq!(module.ports(), &[port_id(1)]);
    }

    #[test]
    fn rejects_duplicate_body_region_references_without_mutation() {
        let mut module = HdlirModule::new(module_id(1), "worker")
            .expect("module should be constructed");

        module
            .add_body_region(region_id(1))
            .expect("first region should be added");

        assert!(module.add_body_region(region_id(1)).is_err());
        assert_eq!(module.body_regions(), &[region_id(1)]);
    }

    #[test]
    fn preserves_parameter_port_and_region_order() {
        let mut module = HdlirModule::new(module_id(1), "worker")
            .expect("module should be constructed");

        for id in [3, 1, 2] {
            module
                .add_parameter(
                    HdlirModuleParameter::new(
                        format!("P{id}"),
                        type_id(id),
                        None,
                    )
                    .expect("parameter should be constructed"),
                )
                .expect("parameter should be added");

            module
                .add_port(port_id(id))
                .expect("port should be added");

            module
                .add_body_region(region_id(id))
                .expect("region should be added");
        }

        assert_eq!(
            module
                .parameters()
                .iter()
                .map(HdlirModuleParameter::name)
                .collect::<Vec<_>>(),
            vec!["P3", "P1", "P2"]
        );
        assert_eq!(module.ports(), &[port_id(3), port_id(1), port_id(2)]);
        assert_eq!(
            module.body_regions(),
            &[region_id(3), region_id(1), region_id(2)]
        );
        assert!(module.validate_structure().is_ok());
    }

    #[test]
    fn parameter_default_is_optional_and_symbolic() {
        let without_default =
            HdlirModuleParameter::new("WIDTH", type_id(1), None)
                .expect("parameter should be constructed");

        let with_default =
            HdlirModuleParameter::new("DEPTH", type_id(2), Some(value_id(9)))
                .expect("parameter should be constructed");

        assert_eq!(without_default.default_value(), None);
        assert_eq!(with_default.default_value(), Some(value_id(9)));
    }

    #[test]
    fn design_module_trait_exposes_stable_identity_and_name() {
        let module = HdlirModule::new(module_id(7), "compute")
            .expect("module should be constructed");

        let contract: &dyn DesignModule = &module;
        assert_eq!(contract.id(), module_id(7));
        assert_eq!(contract.name(), "compute");
    }
}
