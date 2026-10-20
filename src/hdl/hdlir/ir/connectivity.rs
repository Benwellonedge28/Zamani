
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani HDLIR — Logical Connectivity
//!
//! Path: `src/hdl/hdlir/ir/connectivity.rs`
//! Edition: Rust 2021.
//! Minimum supported Rust version: Rust 1.97.
//! Safety: Safe Rust only.
//! Status: Proposed implementation; freeze after repository integration.
//!
//! ## Purpose
//!
//! Represent target-independent logical connections among HDLIR ports,
//! signals, values, and module-instance ports. Preserve connection kind,
//! ordered driver and sink endpoints, and explicit driver-resolution policy.
//!
//! A connection is a logical relationship. It does not allocate a physical
//! wire, pin, routing track, switch, register, or vendor-specific resource.
//!
//! ## Authority
//!
//! This module follows:
//!
//! - `grammar/DESIGN.md`.
//! - `grammar/spec/hdl.md`, especially signals, nets, ports, connectivity,
//!   multiple-driver semantics, portability, and scalability.
//! - `grammar/hdl/hdl.g4`.
//! - `grammar/hdl/nets.g4`.
//! - `grammar/hdl/wires.g4`.
//! - `grammar/hdl/signals.g4`.
//! - `grammar/hdl/ports.g4`.
//! - `grammar/specification/semantic-model.md`.
//! - `src/hdl/api.rs`.
//! - `src/hdl/ids.rs`.
//! - `src/hdl/hdlir/HDLIR-SPEC.md`.
//! - `src/hdl/hdlir/errors.rs`.
//! - `src/hdl/hdlir/ids.rs`.
//! - `src/hdl/hdlir/schema/registry.rs`.
//! - `src/hdl/hdlir/ir/operations.rs`.
//! - `src/hdl/hdlir/ir/ports.rs`.
//! - `src/hdl/hdlir/ir/values.rs`.
//! - `src/hdl/hdlir/types/signal.rs`.
//! - `src/zuir/ZUIR-SPEC.md`.
//!
//! Grammar owns source syntax. The canonical frontend owns source-level
//! meaning and legality. ZUIR owns universal computational meaning. HDLIR
//! owns the hardware-domain connectivity representation.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Typed logical endpoint references.
//! - Connection kinds and explicit driver-resolution policy.
//! - Checked local construction of connection records.
//! - An ordered connection collection with identity lookup.
//! - Local structural checks and an endpoint-reference validation hook.
//!
//! This module does not own:
//!
//! - Source parsing, name resolution, ASTs, or semantic validation.
//! - Signal, port, value, module, or instance declarations.
//! - The authoritative signal type or symbolic-width evaluator.
//! - The complete design graph or the authoritative driver/use index.
//! - Operation execution, behavioral-process semantics, or clock semantics.
//! - A universal list of resolution functions or their implementations.
//! - Physical wires, pins, placement, routing, synthesis, or target allocation.
//!
//! ## Verification boundary
//!
//! Local validation checks the record's own fields. It cannot prove that an
//! endpoint exists, that an instance actually declares a referenced port,
//! that the port direction permits the connection, or that endpoint types
//! and widths are compatible.
//!
//! Whole-graph verification must resolve references and apply the registered
//! dialect, type, direction, hierarchy, and multiple-driver contracts.
//! An indeterminate type-compatibility result is not proof of compatibility.
//!
//! A resolved connection stores a qualified resolver identity. This module
//! does not claim that the resolver is registered, available, deterministic,
//! or semantically correct. Those properties belong to schema and dialect
//! verification.
//!
//! ## Scalability and determinism
//!
//! There is no universal maximum number of connections, endpoints, drivers,
//! or sinks. Storage grows subject to available resources and Rust collection
//! representation limits. This module does not claim to process infinite
//! input.
//!
//! Endpoint and connection IDs are references, never implicit collection
//! indexes. Driver and sink ordering is preserved. No hash iteration order,
//! host pointer, or physical resource identity is used.
//!
//! ## Integration
//!
//! - `ir/mod.rs` must declare `pub mod connectivity;`.
//! - `ids.rs` owns `HdlirNetId`, `HdlirSignalId`, `HdlirValueId`,
//!   `HdlirPortId`, and `HdlirInstanceId`.
//! - `ir/ports.rs` owns logical port declarations and directions.
//! - `ir/values.rs` owns value definitions and operation-use indexing.
//! - The enclosing graph owns signal and instance declarations.
//! - `types/signal.rs` owns signal-type compatibility semantics.
//! - `schema/registry.rs` owns registered extension and resolver definitions.
//! - `verify/structure.rs` must check reference existence and ownership.
//! - `verify/connectivity.rs` must check endpoint direction, hierarchy,
//!   driver/sink legality, and applicable resolution contracts.
//! - `verify/types.rs` must check endpoint type and width compatibility.
//! - Builders must use checked constructors and preserve reference integrity.
//! - Serialization must use a versioned artifact contract, not Rust layout.
//!
//! The current HDLIR composition root and module registration must be repaired
//! separately before this file can compile as part of the crate.
//!
//! ## Freeze criteria
//!
//! - Local connection invariants and error behavior are tested.
//! - Whole-graph endpoint, direction, type, hierarchy, and resolution checks
//!   are implemented in their owning verification modules.
//! - The containing module tree compiles against the repository's real APIs.
//! - Formatting, build, tests, and Clippy pass on the supported toolchain.

use std::collections::{BTreeMap, BTreeSet};

use crate::hdl::hdlir::errors::{HdlirError, HdlirResult};
use crate::hdl::hdlir::ids::{
    HdlirInstanceId, HdlirNetId, HdlirPortId, HdlirSignalId, HdlirValueId,
};
use crate::hdl::hdlir::schema::registry::QualifiedName;

/// A typed reference to a logical endpoint that can participate in a
/// hardware-domain connection.
///
/// Endpoint IDs refer to declarations in the enclosing HDLIR graph. This
/// enum does not contain endpoint definitions and does not imply that every
/// endpoint category is legal in every connection kind.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum HdlirEndpoint {
    /// A declared logical signal.
    Signal(HdlirSignalId),

    /// A graph-level HDLIR value, such as an operation result.
    ///
    /// The verifier must ensure that using the value as a connection endpoint
    /// is legal for the applicable dialect and value semantics.
    Value(HdlirValueId),

    /// A port declared by a logical module.
    Port(HdlirPortId),

    /// A port as exposed by a particular module instance.
    ///
    /// Both identities must resolve, and the referenced port must belong to
    /// the referenced instance's module interface. Merely finding both IDs
    /// independently is insufficient.
    InstancePort {
        /// The instance exposing the port.
        instance: HdlirInstanceId,

        /// The declared port in the instance's module interface.
        port: HdlirPortId,
    },
}

impl HdlirEndpoint {
    /// Return the stable diagnostic category for this endpoint variant.
    #[must_use]
    pub const fn entity_kind(self) -> &'static str {
        match self {
            Self::Signal(_) => "HDLIR signal endpoint",
            Self::Value(_) => "HDLIR value endpoint",
            Self::Port(_) => "HDLIR port endpoint",
            Self::InstancePort { .. } => "HDLIR instance-port endpoint",
        }
    }
}

/// The semantic category of a logical connection.
///
/// These variants distinguish different connectivity contracts. They do not
/// imply a particular physical implementation.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum HdlirConnectionKind {
    /// A logical continuous assignment relationship.
    ///
    /// Assignment evaluation and any language-specific conversion rules are
    /// defined by the applicable operation and dialect contracts.
    ContinuousAssignment,

    /// A structural binding, such as connecting an instance interface to
    /// another logical endpoint.
    StructuralBinding,

    /// A logical net whose multiple-driver behavior is explicitly resolved
    /// by a registered resolution contract.
    ///
    /// The presence of this variant does not itself identify a physical net
    /// or implement the resolution function.
    ResolvedNet,
}

/// The declared policy governing the number and interpretation of drivers.
///
/// A connection's driver policy is explicit. Multiple drivers are never
/// silently interpreted as wired resolution.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum HdlirDriverPolicy {
    /// Exactly one logical driver is required.
    SingleDriver,

    /// Multiple drivers are interpreted under the named resolution contract.
    ///
    /// `resolver` is a qualified schema identity. The graph verifier must
    /// establish that the relevant schema is registered, compatible, and
    /// legal for the connection and endpoint types.
    Resolved {
        /// Registered qualified name of the resolution contract.
        resolver: QualifiedName,
    },
}

/// One logical connection between one or more drivers and one or more sinks.
///
/// The connection ID uses the HDLIR net identity namespace. It is an
/// identity for this logical connectivity record, not a physical net number.
///
/// Drivers and sinks are ordered lists. The order is preserved exactly as
/// provided; the module does not sort endpoints or infer priority from their
/// position. If a resolver defines order-sensitive behavior, that contract
/// must explicitly define how endpoint order is interpreted.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HdlirConnection {
    id: HdlirNetId,
    kind: HdlirConnectionKind,
    driver_policy: HdlirDriverPolicy,
    drivers: Vec<HdlirEndpoint>,
    sinks: Vec<HdlirEndpoint>,
}

impl HdlirConnection {
    /// Construct a connection and check its local structural invariants.
    ///
    /// This constructor requires at least one driver and at least one sink.
    /// A single-driver connection must contain exactly one driver. A
    /// resolved connection must use `ResolvedNet` and name its resolver.
    /// Ordinary continuous assignments and structural bindings use the
    /// explicit `SingleDriver` policy.
    ///
    /// This method does not resolve endpoints or check their direction,
    /// types, widths, hierarchy, or resolver registration.
    ///
    /// # Errors
    ///
    /// Returns a structured error if the driver or sink list is empty,
    /// if a list repeats an endpoint, or if the connection kind and driver
    /// policy disagree.
    pub fn new(
        id: HdlirNetId,
        kind: HdlirConnectionKind,
        driver_policy: HdlirDriverPolicy,
        drivers: Vec<HdlirEndpoint>,
        sinks: Vec<HdlirEndpoint>,
    ) -> HdlirResult<Self> {
        let connection = Self {
            id,
            kind,
            driver_policy,
            drivers,
            sinks,
        };

        connection.validate_structure()?;
        Ok(connection)
    }

    /// Return the logical connection identity.
    #[must_use]
    pub const fn id(&self) -> HdlirNetId {
        self.id
    }

    /// Return the declared connection category.
    #[must_use]
    pub const fn kind(&self) -> HdlirConnectionKind {
        self.kind
    }

    /// Return the explicit driver policy.
    #[must_use]
    pub fn driver_policy(&self) -> &HdlirDriverPolicy {
        &self.driver_policy
    }

    /// Return driver endpoints in declaration order.
    #[must_use]
    pub fn drivers(&self) -> &[HdlirEndpoint] {
        &self.drivers
    }

    /// Return sink endpoints in declaration order.
    #[must_use]
    pub fn sinks(&self) -> &[HdlirEndpoint] {
        &self.sinks
    }

    /// Return the number of declared driver endpoints.
    ///
    /// This is a host collection length, not a language-level driver limit.
    #[must_use]
    pub fn driver_count(&self) -> usize {
        self.drivers.len()
    }

    /// Return the number of declared sink endpoints.
    ///
    /// This is a host collection length, not a language-level sink limit.
    #[must_use]
    pub fn sink_count(&self) -> usize {
        self.sinks.len()
    }

    /// Revalidate local invariants.
    ///
    /// This method is useful at trust boundaries such as deserialization or
    /// migration. It deliberately does not validate external graph references.
    ///
    /// # Errors
    ///
    /// Returns an error if endpoint lists or policy fields violate the local
    /// connection contract.
    pub fn validate_structure(&self) -> HdlirResult<()> {
        if self.drivers.is_empty() {
            return Err(HdlirError::invalid_input(
                "connection.drivers",
                "a logical connection must declare at least one driver",
            ));
        }

        if self.sinks.is_empty() {
            return Err(HdlirError::invalid_input(
                "connection.sinks",
                "a logical connection must declare at least one sink",
            ));
        }

        Self::validate_unique_endpoints(
            &self.drivers,
            "HDLIR connection driver endpoint",
            self.id,
        )?;

        Self::validate_unique_endpoints(
            &self.sinks,
            "HDLIR connection sink endpoint",
            self.id,
        )?;

        match (&self.kind, &self.driver_policy) {
            (
                HdlirConnectionKind::ContinuousAssignment
                | HdlirConnectionKind::StructuralBinding,
                HdlirDriverPolicy::SingleDriver,
            ) if self.drivers.len() == 1 => {}

            (
                HdlirConnectionKind::ContinuousAssignment
                | HdlirConnectionKind::StructuralBinding,
                HdlirDriverPolicy::SingleDriver,
            ) => {
                return Err(HdlirError::invalid_input(
                    "connection.drivers",
                    format!(
                        "connection {} uses a single-driver policy but declares \
                         {} drivers",
                        self.id,
                        self.drivers.len()
                    ),
                ));
            }

            (
                HdlirConnectionKind::ResolvedNet,
                HdlirDriverPolicy::Resolved { .. },
            ) => {
                // A resolved connection may currently have one driver.
                // Its resolution policy remains explicit, so adding another
                // driver does not silently change the connection's meaning.
            }

            (
                HdlirConnectionKind::ResolvedNet,
                HdlirDriverPolicy::SingleDriver,
            ) => {
                return Err(HdlirError::invalid_input(
                    "connection.driver_policy",
                    "a resolved net must name an explicit resolution contract",
                ));
            }

            (
                HdlirConnectionKind::ContinuousAssignment
                | HdlirConnectionKind::StructuralBinding,
                HdlirDriverPolicy::Resolved { .. },
            ) => {
                return Err(HdlirError::invalid_input(
                    "connection.driver_policy",
                    "a resolution contract is permitted only for a resolved net",
                ));
            }
        }

        Ok(())
    }

    /// Check that every endpoint resolves in the enclosing graph.
    ///
    /// The caller supplies the graph-aware resolver. For `InstancePort`, the
    /// resolver must check that the port belongs to the module instantiated by
    /// the referenced instance; checking the two IDs independently is not
    /// sufficient.
    ///
    /// This function does not check endpoint direction, type compatibility,
    /// driver legality, or resolution semantics.
    ///
    /// # Errors
    ///
    /// Returns `MissingEntity` for the first unresolved endpoint, preserving
    /// driver order followed by sink order for deterministic diagnostics.
    pub fn validate_endpoint_references(
        &self,
        mut resolves: impl FnMut(HdlirEndpoint) -> bool,
    ) -> HdlirResult<()> {
        for endpoint in self.drivers.iter().chain(self.sinks.iter()).copied() {
            if !resolves(endpoint) {
                return Err(HdlirError::MissingEntity {
                    entity_kind: endpoint.entity_kind(),
                    identity: format!("{endpoint:?}"),
                    scope: Some(self.id.to_string()),
                });
            }
        }

        Ok(())
    }

    fn validate_unique_endpoints(
        endpoints: &[HdlirEndpoint],
        entity_kind: &'static str,
        connection_id: HdlirNetId,
    ) -> HdlirResult<()> {
        let mut seen = BTreeSet::new();

        for endpoint in endpoints {
            if !seen.insert(*endpoint) {
                return Err(HdlirError::DuplicateEntity {
                    entity_kind,
                    identity: format!("{endpoint:?}"),
                    scope: Some(connection_id.to_string()),
                });
            }
        }

        Ok(())
    }
}

/// An ordered collection of logical HDLIR connections.
///
/// The vector preserves insertion order for deterministic iteration. The
/// identity index provides lookup without treating IDs as vector positions.
/// The index is maintained by the checked insertion and removal methods.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct HdlirConnectivity {
    connections: Vec<HdlirConnection>,
    by_id: BTreeMap<HdlirNetId, usize>,
}

impl HdlirConnectivity {
    /// Create an empty connectivity collection.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Construct a connectivity collection from an iterator of connections.
    ///
    /// Insertion order is preserved. A failed insertion returns an error
    /// rather than replacing an existing connection.
    ///
    /// # Errors
    ///
    /// Returns an error if any connection is structurally invalid or has an
    /// identity already present in the collection.
    pub fn from_connections<I>(connections: I) -> HdlirResult<Self>
    where
        I: IntoIterator<Item = HdlirConnection>,
    {
        let mut collection = Self::new();

        for connection in connections {
            collection.insert(connection)?;
        }

        Ok(collection)
    }

    /// Insert a connection without replacing an existing connection.
    ///
    /// Validation occurs before the collection is mutated. On failure, the
    /// existing collection remains unchanged.
    ///
    /// # Errors
    ///
    /// Returns an error if the connection violates local invariants or its
    /// ID is already present.
    pub fn insert(&mut self, connection: HdlirConnection) -> HdlirResult<()> {
        connection.validate_structure()?;

        if self.by_id.contains_key(&connection.id()) {
            return Err(HdlirError::DuplicateEntity {
                entity_kind: "HDLIR connection",
                identity: connection.id().to_string(),
                scope: None,
            });
        }

        let index = self.connections.len();
        let id = connection.id();

        self.connections.push(connection);
        self.by_id.insert(id, index);

        Ok(())
    }

    /// Look up a connection by its typed identity.
    #[must_use]
    pub fn get(&self, id: HdlirNetId) -> Option<&HdlirConnection> {
        self.by_id
            .get(&id)
            .and_then(|index| self.connections.get(*index))
    }

    /// Return whether the collection contains the given connection identity.
    #[must_use]
    pub fn contains(&self, id: HdlirNetId) -> bool {
        self.by_id.contains_key(&id)
    }

    /// Remove a connection by identity and return it if present.
    ///
    /// The relative order of the remaining connections is preserved.
    pub fn remove(&mut self, id: HdlirNetId) -> Option<HdlirConnection> {
        let index = self.by_id.remove(&id)?;
        let removed = self.connections.remove(index);

        // Removing from the vector shifts later positions. Rebuild the
        // identity index without changing the remaining declaration order.
        for (position, connection) in self.connections.iter().enumerate().skip(index) {
            self.by_id.insert(connection.id(), position);
        }

        Some(removed)
    }

    /// Iterate over connections in insertion order.
    pub fn iter(&self) -> impl ExactSizeIterator<Item = &HdlirConnection> {
        self.connections.iter()
    }

    /// Iterate over connection IDs in insertion order.
    pub fn ids(&self) -> impl ExactSizeIterator<Item = HdlirNetId> + '_ {
        self.connections.iter().map(HdlirConnection::id)
    }

    /// Return the number of connections in this collection.
    ///
    /// This is the current host collection length, not a language-level
    /// connection-count limit.
    #[must_use]
    pub fn len(&self) -> usize {
        self.connections.len()
    }

    /// Return whether the collection is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.connections.is_empty()
    }

    /// Validate every connection and the internal identity index.
    ///
    /// This checks local connection structure, uniqueness, ordering-index
    /// consistency, and identity lookup. It does not resolve endpoint
    /// references or establish whole-graph semantic validity.
    ///
    /// # Errors
    ///
    /// Returns a structured error if a connection or the internal index is
    /// inconsistent.
    pub fn validate_structure(&self) -> HdlirResult<()> {
        if self.connections.len() != self.by_id.len() {
            return Err(HdlirError::invalid_invariant(
                "connectivity.index_cardinality",
                format!(
                    "connection vector contains {} entries but identity index \
                     contains {} entries",
                    self.connections.len(),
                    self.by_id.len()
                ),
            ));
        }

        let mut seen = BTreeSet::new();

        for (index, connection) in self.connections.iter().enumerate() {
            connection.validate_structure()?;

            let id = connection.id();

            if !seen.insert(id) {
                return Err(HdlirError::DuplicateEntity {
                    entity_kind: "HDLIR connection",
                    identity: id.to_string(),
                    scope: None,
                });
            }

            if self.by_id.get(&id) != Some(&index) {
                return Err(HdlirError::invalid_invariant(
                    "connectivity.identity_index",
                    format!(
                        "connection identity {id} does not map to its actual \
                         position {index}"
                    ),
                ));
            }

            if self.get(id).map(HdlirConnection::id) != Some(id) {
                return Err(HdlirError::invalid_invariant(
                    "connectivity.lookup_consistency",
                    format!("connection identity {id} cannot be looked up"),
                ));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{
        HdlirConnection, HdlirConnectionKind, HdlirConnectivity, HdlirDriverPolicy,
        HdlirEndpoint,
    };
    use crate::hdl::hdlir::errors::HdlirError;
    use crate::hdl::hdlir::ids::{
        HdlirInstanceId, HdlirNetId, HdlirPortId, HdlirSignalId, HdlirValueId,
    };
    use crate::hdl::hdlir::schema::registry::QualifiedName;

    fn net_id(raw: u128) -> HdlirNetId {
        HdlirNetId::new(raw).expect("test IDs must be non-zero")
    }

    fn signal(raw: u128) -> HdlirEndpoint {
        HdlirEndpoint::Signal(
            HdlirSignalId::new(raw).expect("test IDs must be non-zero"),
        )
    }

    fn port(raw: u128) -> HdlirEndpoint {
        HdlirEndpoint::Port(
            HdlirPortId::new(raw).expect("test IDs must be non-zero"),
        )
    }

    fn value(raw: u128) -> HdlirEndpoint {
        HdlirEndpoint::Value(
            HdlirValueId::new(raw).expect("test IDs must be non-zero"),
        )
    }

    fn connection(
        id: u128,
        drivers: Vec<HdlirEndpoint>,
        sinks: Vec<HdlirEndpoint>,
    ) -> HdlirConnection {
        HdlirConnection::new(
            net_id(id),
            HdlirConnectionKind::ContinuousAssignment,
            HdlirDriverPolicy::SingleDriver,
            drivers,
            sinks,
        )
        .expect("valid test connection")
    }

    fn resolver() -> QualifiedName {
        QualifiedName::parse("structural.resolve_logic")
            .expect("valid qualified resolver name")
    }

    #[test]
    fn constructs_and_preserves_ordered_endpoints() {
        let first = signal(8);
        let second = value(3);
        let sink = port(5);

        let connection = connection(1, vec![first, second], vec![sink]);

        // A single-driver assignment may not contain two drivers.
        // Use the resolved-net contract to exercise ordered multi-driver data.
        assert!(connection.validate_structure().is_err());

        let resolved = HdlirConnection::new(
            net_id(2),
            HdlirConnectionKind::ResolvedNet,
            HdlirDriverPolicy::Resolved { resolver: resolver() },
            vec![first, second],
            vec![sink],
        )
        .expect("explicitly resolved connection");

        assert_eq!(resolved.drivers(), &[first, second]);
        assert_eq!(resolved.sinks(), &[sink]);
        assert_eq!(resolved.driver_count(), 2);
        assert_eq!(resolved.sink_count(), 1);
    }

    #[test]
    fn rejects_empty_driver_list() {
        let result = HdlirConnection::new(
            net_id(1),
            HdlirConnectionKind::ContinuousAssignment,
            HdlirDriverPolicy::SingleDriver,
            Vec::new(),
            vec![port(1)],
        );

        assert!(matches!(result, Err(HdlirError::InvalidInput { .. })));
    }

    #[test]
    fn rejects_empty_sink_list() {
        let result = HdlirConnection::new(
            net_id(1),
            HdlirConnectionKind::ContinuousAssignment,
            HdlirDriverPolicy::SingleDriver,
            vec![signal(1)],
            Vec::new(),
        );

        assert!(matches!(result, Err(HdlirError::InvalidInput { .. })));
    }

    #[test]
    fn rejects_duplicate_driver_endpoint() {
        let endpoint = signal(1);

        let result = HdlirConnection::new(
            net_id(1),
            HdlirConnectionKind::ContinuousAssignment,
            HdlirDriverPolicy::SingleDriver,
            vec![endpoint, endpoint],
            vec![port(1)],
        );

        assert!(matches!(result, Err(HdlirError::DuplicateEntity { .. })));
    }

    #[test]
    fn rejects_duplicate_sink_endpoint() {
        let endpoint = port(1);

        let result = HdlirConnection::new(
            net_id(1),
            HdlirConnectionKind::ContinuousAssignment,
            HdlirDriverPolicy::SingleDriver,
            vec![signal(1)],
            vec![endpoint, endpoint],
        );

        assert!(matches!(result, Err(HdlirError::DuplicateEntity { .. })));
    }

    #[test]
    fn rejects_implicit_multi_driver_resolution() {
        let result = HdlirConnection::new(
            net_id(1),
            HdlirConnectionKind::StructuralBinding,
            HdlirDriverPolicy::SingleDriver,
            vec![signal(1), signal(2)],
            vec![port(1)],
        );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_resolved_net_without_resolver_policy() {
        let result = HdlirConnection::new(
            net_id(1),
            HdlirConnectionKind::ResolvedNet,
            HdlirDriverPolicy::SingleDriver,
            vec![signal(1)],
            vec![port(1)],
        );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_resolver_policy_on_non_resolved_connection() {
        let result = HdlirConnection::new(
            net_id(1),
            HdlirConnectionKind::ContinuousAssignment,
            HdlirDriverPolicy::Resolved { resolver: resolver() },
            vec![signal(1)],
            vec![port(1)],
        );

        assert!(result.is_err());
    }

    #[test]
    fn preserves_instance_port_identity_as_a_pair() {
        let endpoint = HdlirEndpoint::InstancePort {
            instance: HdlirInstanceId::new(7).expect("non-zero instance ID"),
            port: HdlirPortId::new(3).expect("non-zero port ID"),
        };

        let connection = connection(1, vec![signal(1)], vec![endpoint]);

        assert_eq!(connection.sinks(), &[endpoint]);
    }

    #[test]
    fn endpoint_reference_validation_is_graph_supplied() {
        let connection = connection(1, vec![signal(1)], vec![port(2)]);

        assert!(connection
            .validate_endpoint_references(|endpoint| {
                matches!(endpoint, HdlirEndpoint::Signal(_) | HdlirEndpoint::Port(_))
            })
            .is_ok());

        let result = connection.validate_endpoint_references(|endpoint| {
            !matches!(endpoint, HdlirEndpoint::Port(_))
        });

        assert!(matches!(result, Err(HdlirError::MissingEntity { .. })));
    }

    #[test]
    fn connectivity_insertion_is_checked_and_ordered() {
        let first = connection(9, vec![signal(1)], vec![port(1)]);
        let second = connection(2, vec![signal(2)], vec![port(2)]);

        let mut connectivity = HdlirConnectivity::new();
        connectivity.insert(first).unwrap();
        connectivity.insert(second).unwrap();

        assert_eq!(connectivity.len(), 2);
        assert_eq!(connectivity.ids().collect::<Vec<_>>(), vec![net_id(9), net_id(2)]);
        assert!(connectivity.validate_structure().is_ok());
    }

    #[test]
    fn duplicate_connection_identity_does_not_replace_original() {
        let original = connection(1, vec![signal(1)], vec![port(1)]);
        let duplicate = connection(1, vec![signal(2)], vec![port(2)]);

        let mut connectivity = HdlirConnectivity::new();
        connectivity.insert(original.clone()).unwrap();

        assert!(matches!(
            connectivity.insert(duplicate),
            Err(HdlirError::DuplicateEntity { .. })
        ));
        assert_eq!(connectivity.get(net_id(1)), Some(&original));
        assert_eq!(connectivity.len(), 1);
    }

    #[test]
    fn removal_preserves_remaining_lookup_indexes() {
        let mut connectivity = HdlirConnectivity::from_connections([
            connection(4, vec![signal(1)], vec![port(1)]),
            connection(8, vec![signal(2)], vec![port(2)]),
            connection(12, vec![signal(3)], vec![port(3)]),
        ])
        .unwrap();

        assert!(connectivity.remove(net_id(8)).is_some());
        assert_eq!(
            connectivity.ids().collect::<Vec<_>>(),
            vec![net_id(4), net_id(12)]
        );
        assert!(connectivity.get(net_id(12)).is_some());
        assert!(connectivity.validate_structure().is_ok());
    }

    #[test]
    fn empty_connectivity_is_valid() {
        let connectivity = HdlirConnectivity::new();

        assert!(connectivity.is_empty());
        assert_eq!(connectivity.len(), 0);
        assert!(connectivity.validate_structure().is_ok());
    }

    #[test]
    fn a_single_driver_resolved_net_keeps_explicit_resolution_semantics() {
        let connection = HdlirConnection::new(
            net_id(1),
            HdlirConnectionKind::ResolvedNet,
            HdlirDriverPolicy::Resolved { resolver: resolver() },
            vec![signal(1)],
            vec![port(1)],
        )
        .unwrap();

        assert_eq!(connection.driver_count(), 1);
        assert!(matches!(
            connection.driver_policy(),
            HdlirDriverPolicy::Resolved { .. }
        ));
    }
}
