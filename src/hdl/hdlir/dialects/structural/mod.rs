
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani HDLIR Structural Dialect
//!
//! Path: `src/hdl/hdlir/dialects/structural/mod.rs`
//! Edition: Rust 2021.
//! Minimum supported Rust version: 1.97.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Represent target-independent structural hardware composition:
//!
//! - Logical instances of declared HDLIR modules.
//! - Instance parameter bindings.
//! - Explicit bindings between instance ports and enclosing logical
//!   endpoints.
//! - Deterministic structural inventories and local invariant checking.
//! - Conversion of supported directional port bindings into the canonical
//!   HDLIR connectivity representation.
//!
//! ## Normative authority
//!
//! - `grammar/DESIGN.md`
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4`
//! - `grammar/hdl/hardware-modules.g4`
//! - `grammar/hdl/hardware-generics.g4`
//! - `grammar/hdl/parameters.g4`
//! - `grammar/hdl/ports.g4`
//! - `grammar/hdl/interfaces.g4`
//! - `src/hdl/hdlir/HDLIR-SPEC.md`
//! - `src/hdl/hdlir/compatibility.md`
//! - `src/hdl/hdlir/ids.rs`
//! - `src/hdl/hdlir/ir/design.rs`
//! - `src/hdl/hdlir/ir/module.rs`
//! - `src/hdl/hdlir/ir/ports.rs`
//! - `src/hdl/hdlir/ir/connectivity.rs`
//! - `src/hdl/hdlir/values/parameters.rs`, when registered.
//!
//! The grammar owns source syntax. The frontend semantic model owns source
//! meaning and name resolution. The HDLIR core owns the canonical design,
//! module, port, and connectivity primitives. This dialect composes those
//! primitives; it does not replace them.
//!
//! ## Ownership
//!
//! This module owns structural instance records, explicit parameter
//! bindings, structural port-binding records, and validation of the
//! relationships represented by those records.
//!
//! It does not own:
//!
//! - Parsing, AST nodes, or source-level name resolution.
//! - A competing design, module, port, or connectivity IR.
//! - Type inference or independent parameter-expression evaluation.
//! - Physical cells, pins, wires, placement, or routing.
//! - Synthesis, technology mapping, or timing closure.
//! - Vendor primitives or target-specific runtime handles.
//! - Hardware allocation or target-feasibility decisions.
//!
//! ## Scalability
//!
//! There are no universal limits on instances, ports, parameters, or
//! structural connections. Collections grow with the design and available
//! memory. Host collection limits and allocation failure are implementation
//! constraints, not language-level hardware ceilings.
//!
//! Parameter values remain symbolic HDLIR value identities. This module
//! does not evaluate widths or force symbolic hardware quantities into
//! concrete machine integers.
//!
//! ## Integration
//!
//! This module consumes the existing `HdlirDesign`, `HdlirModule`,
//! `HdlirPortList`, and connectivity types. Its parent dialect composition
//! root should expose this module only after those canonical modules are
//! registered in the repaired `src/hdl/hdlir/mod.rs` module tree.
//!
//! The structural records are deliberately separate from `HdlirModule`
//! because the existing core module does not itself own an instance list.
//! A later builder or verified graph owner can store these records alongside
//! the canonical core design without duplicating its module definitions.
//!
//! ## Directional binding rule
//!
//! For an instance input, the enclosing endpoint drives the instance port.
//! For an instance output, the instance port drives the enclosing endpoint.
//! An inout binding cannot be reduced to an ordinary single-driver binding
//! without additional semantics. This module therefore requires a separate,
//! explicit resolved-connectivity contract for inout rather than guessing.
//!
//! ## Freeze criteria
//!
//! - Existing core IR types remain authoritative.
//! - IDs and names are validated within explicit scopes.
//! - Missing modules, ports, parameters, and duplicate bindings fail clearly.
//! - Parameter values remain symbolic.
//! - Inout semantics are never silently weakened.
//! - No physical-realization behavior is introduced.
//! - Unit, integration, formatting, compilation, and lint tests pass after
//!   the file is registered in the actual repository.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::hdl::hdlir::errors::{HdlirError, HdlirResult};
use crate::hdl::hdlir::ids::{
    HdlirInstanceId,
    HdlirModuleId,
    HdlirNetId,
    HdlirPortId,
};
use crate::hdl::hdlir::ir::connectivity::{
    HdlirConnection,
    HdlirConnectionKind,
    HdlirDriverPolicy,
    HdlirEndpoint,
};
use crate::hdl::hdlir::ir::design::HdlirDesign;
use crate::hdl::hdlir::ir::module::{HdlirModule, HdlirModuleParameter};
use crate::hdl::hdlir::ir::ports::{
    HdlirPort,
    HdlirPortDirection,
    HdlirPortList,
};

/// A validated logical instance name.
///
/// Names are retained exactly. This type does not perform source-level
/// name resolution, case folding, or Unicode normalization.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct StructuralInstanceName(String);

impl StructuralInstanceName {
    /// Construct a logical instance name.
    ///
    /// # Errors
    ///
    /// Returns an error if the name is empty, whitespace-only, or has
    /// leading or trailing whitespace.
    pub fn new(value: impl Into<String>) -> HdlirResult<Self> {
        let value = value.into();

        if value.trim().is_empty() || value.trim() != value {
            return Err(HdlirError::invalid_input(
                "structural_instance.name",
                "instance name must be nonempty and have no leading or trailing whitespace",
            ));
        }

        Ok(Self(value))
    }

    /// Return the exact instance name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for StructuralInstanceName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// An explicit binding from a module parameter name to an HDLIR value.
///
/// The value is a reference to the canonical HDLIR value system. This record
/// does not evaluate the value or independently establish type compatibility.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StructuralParameterBinding {
    name: String,
    value: crate::hdl::hdlir::ids::HdlirValueId,
}

impl StructuralParameterBinding {
    /// Construct a parameter binding.
    ///
    /// # Errors
    ///
    /// Returns an error if the parameter name is empty or whitespace-only,
    /// or contains leading or trailing whitespace.
    pub fn new(
        name: impl Into<String>,
        value: crate::hdl::hdlir::ids::HdlirValueId,
    ) -> HdlirResult<Self> {
        let name = name.into();

        if name.trim().is_empty() || name.trim() != name {
            return Err(HdlirError::invalid_input(
                "structural_parameter.name",
                "parameter name must be nonempty and have no leading or trailing whitespace",
            ));
        }

        Ok(Self { name, value })
    }

    /// Return the exact parameter name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the canonical HDLIR value identity.
    #[must_use]
    pub const fn value(&self) -> crate::hdl::hdlir::ids::HdlirValueId {
        self.value
    }
}

/// A logical instance of a declared HDLIR module.
///
/// This record refers to the canonical module definition. It does not copy
/// that definition or imply that a physical cell has been allocated.
///
/// Parameter bindings are kept in a deterministic name-keyed map. An
/// unbound parameter may be resolved through its declared default or a
/// later specialization stage.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StructuralInstance {
    id: HdlirInstanceId,
    name: StructuralInstanceName,
    module_id: HdlirModuleId,
    parameters: BTreeMap<String, StructuralParameterBinding>,
}

impl StructuralInstance {
    /// Construct an instance referencing a canonical module identity.
    pub fn new(
        id: HdlirInstanceId,
        name: impl Into<String>,
        module_id: HdlirModuleId,
    ) -> HdlirResult<Self> {
        Ok(Self {
            id,
            name: StructuralInstanceName::new(name)?,
            module_id,
            parameters: BTreeMap::new(),
        })
    }

    /// Return the logical instance identity.
    #[must_use]
    pub const fn id(&self) -> HdlirInstanceId {
        self.id
    }

    /// Return the logical instance name.
    #[must_use]
    pub fn name(&self) -> &StructuralInstanceName {
        &self.name
    }

    /// Return the referenced canonical module identity.
    #[must_use]
    pub const fn module_id(&self) -> HdlirModuleId {
        self.module_id
    }

    /// Return parameter bindings in deterministic name order.
    #[must_use]
    pub fn parameters(
        &self,
    ) -> impl ExactSizeIterator<Item = (&str, &StructuralParameterBinding)> {
        self.parameters
            .iter()
            .map(|(name, binding)| (name.as_str(), binding))
    }

    /// Look up a parameter binding by its exact name.
    #[must_use]
    pub fn parameter(&self, name: &str) -> Option<&StructuralParameterBinding> {
        self.parameters.get(name)
    }

    /// Insert a parameter binding without replacing an existing binding.
    ///
    /// # Errors
    ///
    /// Returns an error when the name is already bound.
    pub fn bind_parameter(
        &mut self,
        binding: StructuralParameterBinding,
    ) -> HdlirResult<()> {
        let name = binding.name().to_owned();

        if self.parameters.contains_key(&name) {
            return Err(HdlirError::DuplicateEntity {
                entity_kind: "structural parameter binding",
                identity: format!("instance `{}` parameter `{name}`", self.name),
                scope: Some(format!("instance {}", self.id)),
            });
        }

        self.parameters.insert(name, binding);
        Ok(())
    }

    /// Verify parameter bindings against a canonical module signature.
    ///
    /// This checks that every explicitly bound parameter exists and that
    /// every parameter without a default has an explicit binding. It does not
    /// evaluate values or establish type compatibility; those checks belong
    /// to the canonical value and semantic verification layers.
    ///
    /// # Errors
    ///
    /// Returns a structured error for an unknown parameter or an omitted
    /// required parameter.
    pub fn validate_parameters(&self, module: &HdlirModule) -> HdlirResult<()> {
        if self.module_id != module.id() {
            return Err(HdlirError::invalid_invariant(
                "structural_instance.module_identity",
                format!(
                    "instance `{}` references module {}, but the supplied module has identity {}",
                    self.name,
                    self.module_id,
                    module.id()
                ),
            ));
        }

        for name in self.parameters.keys() {
            if module.parameter(name).is_none() {
                return Err(HdlirError::invalid_input(
                    "structural_parameter.name",
                    format!(
                        "instance `{}` binds unknown parameter `{name}` of module `{}`",
                        self.name,
                        module.name_str()
                    ),
                ));
            }
        }

        for parameter in module.parameters() {
            if parameter.default_value().is_none()
                && !self.parameters.contains_key(parameter.name())
            {
                return Err(HdlirError::invalid_input(
                    "structural_parameter.binding",
                    format!(
                        "instance `{}` omits required parameter `{}` of module `{}`",
                        self.name,
                        parameter.name(),
                        module.name_str()
                    ),
                ));
            }
        }

        Ok(())
    }
}

/// A structural binding between one instance port and an enclosing endpoint.
///
/// `connection_id` identifies the logical connectivity record that represents
/// the binding. It is not a physical net number.
///
/// The enclosing endpoint may be a logical signal, value, or enclosing-module
/// port. The exact legality of that endpoint must be checked by the canonical
/// graph verifier.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StructuralPortBinding {
    connection_id: HdlirNetId,
    instance_id: HdlirInstanceId,
    port_id: HdlirPortId,
    enclosing_endpoint: HdlirEndpoint,
}

impl StructuralPortBinding {
    /// Construct a structural port binding.
    ///
    /// An instance-port endpoint cannot be used as the enclosing endpoint
    /// because this record already identifies its instance port. A binding
    /// between two instances can instead be expressed as two explicit
    /// bindings to a shared enclosing logical signal or through an explicit
    /// canonical connectivity record.
    ///
    /// # Errors
    ///
    /// Returns an error for a self-referential instance-port endpoint.
    pub fn new(
        connection_id: HdlirNetId,
        instance_id: HdlirInstanceId,
        port_id: HdlirPortId,
        enclosing_endpoint: HdlirEndpoint,
    ) -> HdlirResult<Self> {
        if let HdlirEndpoint::InstancePort { instance, port } = enclosing_endpoint {
            if instance == instance_id && port == port_id {
                return Err(HdlirError::invalid_input(
                    "structural_port_binding.endpoint",
                    "a port cannot be bound to itself",
                ));
            }
        }

        Ok(Self {
            connection_id,
            instance_id,
            port_id,
            enclosing_endpoint,
        })
    }

    /// Return the logical connection identity.
    #[must_use]
    pub const fn connection_id(&self) -> HdlirNetId {
        self.connection_id
    }

    /// Return the instance identity.
    #[must_use]
    pub const fn instance_id(&self) -> HdlirInstanceId {
        self.instance_id
    }

    /// Return the declared module port identity.
    #[must_use]
    pub const fn port_id(&self) -> HdlirPortId {
        self.port_id
    }

    /// Return the enclosing logical endpoint.
    #[must_use]
    pub const fn enclosing_endpoint(&self) -> HdlirEndpoint {
        self.enclosing_endpoint
    }

    /// Convert a directional port binding into canonical single-driver
    /// structural connectivity.
    ///
    /// For an input port, the enclosing endpoint is the driver and the
    /// instance port is the sink. For an output port, the instance port is
    /// the driver and the enclosing endpoint is the sink.
    ///
    /// Inout ports require explicit resolved-connectivity semantics and are
    /// deliberately rejected by this method. No resolver is inferred.
    ///
    /// This method does not verify endpoint existence, signal types,
    /// symbolic widths, hierarchy, or the legality of the enclosing endpoint.
    /// Those are responsibilities of the canonical connectivity verifier.
    ///
    /// # Errors
    ///
    /// Returns an error if the port is bidirectional and cannot be represented
    /// by this single-driver binding contract.
    pub fn to_connection(
        &self,
        port: &HdlirPort,
    ) -> HdlirResult<HdlirConnection> {
        if port.id() != self.port_id {
            return Err(HdlirError::invalid_invariant(
                "structural_port_binding.port_identity",
                format!(
                    "binding references port {}, but supplied port has identity {}",
                    self.port_id,
                    port.id()
                ),
            ));
        }

        let instance_endpoint = HdlirEndpoint::InstancePort {
            instance: self.instance_id,
            port: self.port_id,
        };

        let (drivers, sinks) = match port.direction() {
            HdlirPortDirection::Input => (
                vec![self.enclosing_endpoint],
                vec![instance_endpoint],
            ),
            HdlirPortDirection::Output => (
                vec![instance_endpoint],
                vec![self.enclosing_endpoint],
            ),
            HdlirPortDirection::InOut => {
                return Err(HdlirError::invalid_input(
                    "structural_port_binding.direction",
                    format!(
                        "inout port `{}` requires an explicit resolved-connectivity contract",
                        port.name()
                    ),
                ));
            }
        };

        HdlirConnection::new(
            self.connection_id,
            HdlirConnectionKind::StructuralBinding,
            HdlirDriverPolicy::SingleDriver,
            drivers,
            sinks,
        )
    }
}

/// Deterministic structural records for one HDLIR design.
///
/// This collection deliberately stores instances and bindings separately
/// from the canonical module definitions. The owning compilation graph is
/// responsible for storing the resulting records and canonical connectivity
/// together.
///
/// Insertion order is retained for instances and bindings. Lookup indexes
/// provide duplicate detection without introducing hardware-specific limits.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct StructuralDialect {
    instances: Vec<StructuralInstance>,
    instance_index: BTreeMap<HdlirInstanceId, usize>,
    instance_names: BTreeMap<String, HdlirInstanceId>,
    bindings: Vec<StructuralPortBinding>,
    binding_index: BTreeSet<(HdlirInstanceId, HdlirPortId)>,
    connection_ids: BTreeSet<HdlirNetId>,
}

impl StructuralDialect {
    /// Create an empty structural dialect inventory.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Return the number of logical instances.
    #[must_use]
    pub fn instance_count(&self) -> usize {
        self.instances.len()
    }

    /// Return whether there are no registered instances.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.instances.is_empty()
    }

    /// Iterate over instances in insertion order.
    #[must_use]
    pub fn instances(&self) -> impl ExactSizeIterator<Item = &StructuralInstance> {
        self.instances.iter()
    }

    /// Look up an instance by its logical identity.
    #[must_use]
    pub fn instance(&self, id: HdlirInstanceId) -> Option<&StructuralInstance> {
        self.instance_index
            .get(&id)
            .and_then(|index| self.instances.get(*index))
    }

    /// Look up an instance by its exact logical name.
    #[must_use]
    pub fn instance_by_name(&self, name: &str) -> Option<&StructuralInstance> {
        self.instance_names
            .get(name)
            .and_then(|id| self.instance(*id))
    }

    /// Insert an instance without replacing an existing identity or name.
    ///
    /// # Errors
    ///
    /// Returns a duplicate-entity error if either identity or name is already
    /// present. A failed insertion leaves the inventory unchanged.
    pub fn insert_instance(&mut self, instance: StructuralInstance) -> HdlirResult<()> {
        let id = instance.id();
        let name = instance.name().as_str().to_owned();

        if self.instance_index.contains_key(&id) {
            return Err(HdlirError::DuplicateEntity {
                entity_kind: "structural instance identity",
                identity: id.to_string(),
                scope: Some("structural dialect".to_owned()),
            });
        }

        if self.instance_names.contains_key(&name) {
            return Err(HdlirError::DuplicateEntity {
                entity_kind: "structural instance name",
                identity: name,
                scope: Some("structural dialect".to_owned()),
            });
        }

        let index = self.instances.len();
        self.instances.push(instance);
        self.instance_index.insert(id, index);
        self.instance_names.insert(name, id);
        Ok(())
    }

    /// Return the number of explicit instance-port bindings.
    #[must_use]
    pub fn binding_count(&self) -> usize {
        self.bindings.len()
    }

    /// Iterate over bindings in insertion order.
    #[must_use]
    pub fn bindings(
        &self,
    ) -> impl ExactSizeIterator<Item = &StructuralPortBinding> {
        self.bindings.iter()
    }

    /// Insert a binding without replacing an existing instance-port binding.
    ///
    /// A port of a given instance may have only one explicit binding in this
    /// structural inventory. More elaborate multi-endpoint semantics must be
    /// represented by canonical connectivity and its registered contracts.
    ///
    /// # Errors
    ///
    /// Returns an error if the instance is unknown, the port is already bound,
    /// or the connection identity is already used.
    pub fn insert_binding(
        &mut self,
        binding: StructuralPortBinding,
    ) -> HdlirResult<()> {
        let instance_id = binding.instance_id();
        let port_id = binding.port_id();
        let connection_id = binding.connection_id();

        if self.instance(instance_id).is_none() {
            return Err(HdlirError::MissingEntity {
                entity_kind: "structural instance",
                identity: instance_id.to_string(),
                scope: Some("structural dialect".to_owned()),
            });
        }

        if self.binding_index.contains(&(instance_id, port_id)) {
            return Err(HdlirError::DuplicateEntity {
                entity_kind: "structural instance-port binding",
                identity: format!("{instance_id}:{port_id}"),
                scope: Some("structural dialect".to_owned()),
            });
        }

        if self.connection_ids.contains(&connection_id) {
            return Err(HdlirError::DuplicateEntity {
                entity_kind: "structural connection identity",
                identity: connection_id.to_string(),
                scope: Some("structural dialect".to_owned()),
            });
        }

        self.binding_index.insert((instance_id, port_id));
        self.connection_ids.insert(connection_id);
        self.bindings.push(binding);
        Ok(())
    }

    /// Validate instance references, parameter signatures, and port bindings
    /// against the canonical design and port inventory.
    ///
    /// This checks the structural relationships represented by this dialect.
    /// It does not claim full graph verification, type compatibility,
    /// connectivity completeness, cycle legality, timing closure, or target
    /// feasibility.
    ///
    /// # Errors
    ///
    /// Returns the first detected missing entity or violated invariant.
    pub fn validate(
        &self,
        design: &HdlirDesign<HdlirModule>,
        ports: &HdlirPortList,
    ) -> HdlirResult<()> {
        let mut seen_instance_names = BTreeSet::new();
        let mut seen_instance_ids = BTreeSet::new();

        for instance in &self.instances {
            if !seen_instance_ids.insert(instance.id()) {
                return Err(HdlirError::DuplicateEntity {
                    entity_kind: "structural instance identity",
                    identity: instance.id().to_string(),
                    scope: Some("structural dialect validation".to_owned()),
                });
            }

            if !seen_instance_names.insert(instance.name().as_str()) {
                return Err(HdlirError::DuplicateEntity {
                    entity_kind: "structural instance name",
                    identity: instance.name().to_string(),
                    scope: Some("structural dialect validation".to_owned()),
                });
            }

            let module = design.module(instance.module_id()).ok_or_else(|| {
                HdlirError::MissingEntity {
                    entity_kind: "structural instance module",
                    identity: instance.module_id().to_string(),
                    scope: Some(format!("instance `{}`", instance.name())),
                }
            })?;

            instance.validate_parameters(module)?;
        }

        let mut seen_binding_keys = BTreeSet::new();
        let mut seen_connection_ids = BTreeSet::new();

        for binding in &self.bindings {
            if !seen_binding_keys.insert((binding.instance_id(), binding.port_id())) {
                return Err(HdlirError::DuplicateEntity {
                    entity_kind: "structural instance-port binding",
                    identity: format!("{}:{}", binding.instance_id(), binding.port_id()),
                    scope: Some("structural dialect validation".to_owned()),
                });
            }

            if !seen_connection_ids.insert(binding.connection_id()) {
                return Err(HdlirError::DuplicateEntity {
                    entity_kind: "structural connection identity",
                    identity: binding.connection_id().to_string(),
                    scope: Some("structural dialect validation".to_owned()),
                });
            }

            let instance = self.instance(binding.instance_id()).ok_or_else(|| {
                HdlirError::MissingEntity {
                    entity_kind: "structural instance",
                    identity: binding.instance_id().to_string(),
                    scope: Some("structural port binding".to_owned()),
                }
            })?;

            let module = design.module(instance.module_id()).ok_or_else(|| {
                HdlirError::MissingEntity {
                    entity_kind: "structural instance module",
                    identity: instance.module_id().to_string(),
                    scope: Some(format!("instance `{}`", instance.name())),
                }
            })?;

            if !module.ports().contains(&binding.port_id()) {
                return Err(HdlirError::MissingEntity {
                    entity_kind: "port in referenced module interface",
                    identity: binding.port_id().to_string(),
                    scope: Some(format!("module `{}`", module.name_str())),
                });
            }

            if ports.get(binding.port_id()).is_none() {
                return Err(HdlirError::MissingEntity {
                    entity_kind: "canonical HDLIR port definition",
                    identity: binding.port_id().to_string(),
                    scope: Some(format!("module `{}`", module.name_str())),
                });
            }
        }

        Ok(())
    }

    /// Convert every binding into its canonical single-driver connection.
    ///
    /// Inout ports fail explicitly because they require separately registered
    /// resolved-connectivity semantics. The caller must combine these
    /// connections with the canonical connectivity collection and run the
    /// complete graph verifier before treating the design as verified.
    ///
    /// # Errors
    ///
    /// Returns an error if a referenced port is missing or a binding cannot
    /// be represented by the single-driver connection contract.
    pub fn to_connections(
        &self,
        ports: &HdlirPortList,
    ) -> HdlirResult<Vec<HdlirConnection>> {
        let mut connections = Vec::with_capacity(self.bindings.len());

        for binding in &self.bindings {
            let port = ports.get(binding.port_id()).ok_or_else(|| {
                HdlirError::MissingEntity {
                    entity_kind: "canonical HDLIR port definition",
                    identity: binding.port_id().to_string(),
                    scope: Some("structural dialect conversion".to_owned()),
                }
            })?;

            connections.push(binding.to_connection(port)?);
        }

        Ok(connections)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instance_name_rejects_empty_and_padded_names() {
        assert!(StructuralInstanceName::new("").is_err());
        assert!(StructuralInstanceName::new("  ").is_err());
        assert!(StructuralInstanceName::new(" instance").is_err());
        assert!(StructuralInstanceName::new("instance ").is_err());
        assert_eq!(
            StructuralInstanceName::new("instance_α")
                .expect("valid logical name")
                .as_str(),
            "instance_α"
        );
    }

    #[test]
    fn parameter_binding_rejects_empty_names() {
        // The identity value is irrelevant to name validation; no ID is
        // allocated or consumed by this test.
        let value = crate::hdl::hdlir::ids::HdlirValueId::new(1)
            .expect("nonzero HDLIR value ID");

        assert!(StructuralParameterBinding::new("", value).is_err());
        assert!(StructuralParameterBinding::new(" ", value).is_err());
        assert!(StructuralParameterBinding::new("WIDTH ", value).is_err());
        assert!(StructuralParameterBinding::new("WIDTH", value).is_ok());
    }

    #[test]
    fn structural_inventory_starts_empty() {
        let dialect = StructuralDialect::new();

        assert!(dialect.is_empty());
        assert_eq!(dialect.instance_count(), 0);
        assert_eq!(dialect.binding_count(), 0);
        assert_eq!(dialect.instances().len(), 0);
        assert_eq!(dialect.bindings().len(), 0);
    }

    #[test]
    fn instance_name_ordering_is_deterministic() {
        let first = StructuralInstanceName::new("alpha").expect("valid name");
        let second = StructuralInstanceName::new("beta").expect("valid name");

        assert!(first < second);
        assert_eq!(first, StructuralInstanceName::new("alpha").expect("valid name"));
    }
}
