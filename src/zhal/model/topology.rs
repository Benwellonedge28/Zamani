
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Hardware Abstraction Layer
//!
//! ## Physical Hardware Topology Model
//!
//! Path: `src/zhal/model/topology.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Defines the provider-independent representation of observed physical
//! hardware topology throughout the Zamani computing architecture.
//!
//! Supported physical structures include:
//!
//! - Classical processors and memory systems.
//! - Quantum processors and physical components.
//! - FPGA and ASIC systems.
//! - GPUs, TPUs, NPUs and other accelerators.
//! - Embedded systems.
//! - Sensors and actuators.
//! - Local and remote computing systems.
//! - Distributed heterogeneous systems.
//! - Neuromorphic systems.
//! - Photonic systems.
//! - Analog and in-memory computing.
//! - Future hardware technologies.
//!
//! ## Architectural ownership
//!
//! This module owns:
//!
//! - Physical topology identities.
//! - Physical topology revisions.
//! - Physical topology nodes.
//! - Physical interconnect descriptions.
//! - Physical hierarchy relationships.
//! - Connection directionality.
//! - Topology construction and validation.
//! - Immutable topology snapshots.
//! - Structural topology queries.
//! - Deterministic topology representation.
//!
//! This module does NOT own:
//!
//! - Source-language topology declarations.
//! - Semantic topology requirements.
//! - Hardware discovery algorithms.
//! - Hardware provider registration.
//! - Provider lifecycle.
//! - Physical resource allocation.
//! - Resource reservations.
//! - Device scheduling.
//! - Network routing.
//! - Quantum routing.
//! - Quantum coupling semantics.
//! - HDL structural semantics.
//! - Hardware calibration.
//! - Hardware execution.
//! - Runtime orchestration.
//! - Distributed execution.
//!
//! ## Existing authorities
//!
//! Hardware topology identity:
//!     crate::zhal::identity::HardwareTopologyId
//!
//! Physical device identity:
//!     crate::zhal::identity::HardwareDeviceIdentity
//!
//! Physical device descriptions:
//!     crate::zhal::model::device
//!
//! Physical resources:
//!     crate::zhal::model::resource
//!
//! Quantum topology:
//!     crate::quantum::hardware::topology
//!
//! HDL structure:
//!     crate::hdl::hdlir
//!
//! Source-level topology intent:
//!     grammar/hardware/topology.g4
//!
//! Runtime resource management:
//!     crate::zur
//!
//! Distributed execution:
//!     crate::distributed
//!
//! ## Grammar boundary
//!
//! grammar/hardware/topology.g4 describes logical topology intent.
//!
//! This module describes observed physical topology.
//!
//! A logical topology node is not automatically a physical device.
//!
//! A physical topology node is not a source-language declaration.
//!
//! Semantic analysis, resource negotiation, placement and routing
//! remain separate architectural responsibilities.
//!
//! ## Fundamental invariants
//!
//! 1. Every topology has a stable identity.
//! 2. Every topology has an explicit revision.
//! 3. Every node has a unique identity within its topology.
//! 4. Every interconnect has a unique identity within its topology.
//! 5. Every interconnect endpoint must reference an existing node.
//! 6. Every hierarchy parent must reference an existing node.
//! 7. A node cannot be its own parent.
//! 8. Hierarchical relationships must be acyclic.
//! 9. Parallel physical interconnects are permitted.
//! 10. Connection directionality is preserved.
//! 11. Unknown information is never fabricated.
//! 12. Topology observations do not imply resource reservations.
//! 13. Topology observations do not guarantee continuing availability.
//! 14. New hardware categories do not require modifications to this module.
//!
//! ## Scalability
//!
//! No arbitrary universal maximum is imposed on:
//!
//! - Topology nodes.
//! - Topology links.
//! - Topology hierarchy depth.
//! - Device categories.
//! - Interconnect categories.
//! - Identifier lengths.
//! - Metadata entries.
//! - Hardware domains.
//!
//! Collections grow according to available memory and platform limits.
//!
//! Operations requiring traversal have costs proportional to the
//! topology being examined.
//!
//! This module does not promise literal infinite storage or computation.
//!
//! ## Serialization
//!
//! Topology identifiers and revisions are serialized explicitly.
//!
//! Deserialization validates topology invariants before construction.
//!
//! The schema version is distinct from the topology revision.
//!
//! ## Security
//!
//! Topology descriptions are descriptive information.
//!
//! They are not authorization credentials.
//!
//! Metadata must not contain passwords, private keys, access tokens,
//! cookies or other secrets.
//!
//! Topology membership does not establish hardware access permission.
//!
//! ## Dependencies
//!
//! - Rust standard library.
//! - serde.
//! - crate::zhal::identity.
//!
//! No hardware SDK is required.

use core::fmt;

use std::collections::{BTreeMap, BTreeSet};

use serde::de::Error as SerdeDeError;
use serde::{Deserialize, Deserializer, Serialize};

use crate::zhal::identity::{
    HardwareDeviceIdentity,
    HardwareTopologyId,
};

// ============================================================================
// COMPONENT IDENTITY
// ============================================================================

/// Stable module identity.
pub const COMPONENT: &str = "zhal.model.topology";

/// Topology schema version.
///
/// This is independent of the topology revision, language version,
/// compiler version and Cargo package version.
pub const TOPOLOGY_SCHEMA_VERSION: u16 = 1;

/// Stable topology schema identifier.
pub const TOPOLOGY_SCHEMA_ID: &str = "zamani.zhal.physical_topology";

// ============================================================================
// ERROR MODEL
// ============================================================================

/// Classification of physical topology errors.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum TopologyErrorKind {
    /// A required identifier is invalid.
    InvalidIdentifier,

    /// A required descriptive field is invalid.
    InvalidDescription,

    /// A node identity is duplicated.
    DuplicateNode,

    /// An interconnect identity is duplicated.
    DuplicateLink,

    /// An interconnect references a nonexistent endpoint.
    MissingEndpoint,

    /// A hierarchy relationship references a nonexistent parent.
    MissingParent,

    /// A node references itself as its parent.
    SelfParent,

    /// The hierarchy contains a cycle.
    HierarchyCycle,

    /// A node has an invalid physical device association.
    InvalidDeviceAssociation,

    /// A topology revision is invalid.
    InvalidRevision,

    /// A serialized topology has an unsupported schema.
    UnsupportedSchema,

    /// The topology is internally inconsistent.
    InvalidTopology,
}

/// Structured topology validation error.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyError {
    kind: TopologyErrorKind,
    subject: Option<String>,
    message: String,
}

impl TopologyError {
    fn new(
        kind: TopologyErrorKind,
        subject: Option<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            subject,
            message: message.into(),
        }
    }

    /// Returns the error classification.
    #[must_use]
    pub const fn kind(&self) -> TopologyErrorKind {
        self.kind
    }

    /// Returns the affected identity, when available.
    #[must_use]
    pub fn subject(&self) -> Option<&str> {
        self.subject.as_deref()
    }

    /// Returns the explanatory message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for TopologyError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            formatter,
            "{}: {}",
            COMPONENT,
            self.message
        )?;

        if let Some(subject) = &self.subject {
            write!(
                formatter,
                " [subject: {}]",
                subject
            )?;
        }

        Ok(())
    }
}

impl std::error::Error for TopologyError {}

/// Result type for topology operations.
pub type TopologyResult<T> = Result<T, TopologyError>;

fn topology_error(
    kind: TopologyErrorKind,
    subject: Option<&str>,
    message: impl Into<String>,
) -> TopologyError {
    TopologyError::new(
        kind,
        subject.map(str::to_owned),
        message,
    )
}

// ============================================================================
// COMMON VALIDATION
// ============================================================================

fn validate_identifier(
    value: &str,
    field: &'static str,
) -> TopologyResult<()> {
    if value.is_empty() {
        return Err(topology_error(
            TopologyErrorKind::InvalidIdentifier,
            Some(value),
            format!("{} must not be empty", field),
        ));
    }

    if value.trim() != value {
        return Err(topology_error(
            TopologyErrorKind::InvalidIdentifier,
            Some(value),
            format!(
                "{} must not contain surrounding whitespace",
                field
            ),
        ));
    }

    if value.chars().any(char::is_control) {
        return Err(topology_error(
            TopologyErrorKind::InvalidIdentifier,
            Some(value),
            format!(
                "{} must not contain control characters",
                field
            ),
        ));
    }

    if value.chars().any(char::is_whitespace) {
        return Err(topology_error(
            TopologyErrorKind::InvalidIdentifier,
            Some(value),
            format!(
                "{} must not contain whitespace",
                field
            ),
        ));
    }

    Ok(())
}

fn validate_description(
    value: &str,
    field: &'static str,
) -> TopologyResult<()> {
    if value.trim().is_empty() {
        return Err(topology_error(
            TopologyErrorKind::InvalidDescription,
            None,
            format!("{} must not be empty", field),
        ));
    }

    if value.chars().any(char::is_control) {
        return Err(topology_error(
            TopologyErrorKind::InvalidDescription,
            None,
            format!(
                "{} must not contain control characters",
                field
            ),
        ));
    }

    Ok(())
}

// ============================================================================
// OPEN-WORLD IDENTIFIERS
// ============================================================================

/// Defines an extensible physical topology identifier.
///
/// Identifiers are case-sensitive and preserve their original spelling.
macro_rules! define_topology_identifier {
    (
        $name:ident,
        $description:literal,
        $field:literal
    ) => {
        #[doc = $description]
        #[derive(
            Clone,
            Debug,
            Eq,
            Hash,
            Ord,
            PartialEq,
            PartialOrd,
            Serialize,
        )]
        #[serde(transparent)]
        pub struct $name(String);

        impl $name {
            /// Creates a validated identifier.
            ///
            /// No arbitrary maximum identifier length is imposed.
            pub fn new(
                value: impl Into<String>,
            ) -> TopologyResult<Self> {
                let value = value.into();

                validate_identifier(
                    &value,
                    $field,
                )?;

                Ok(Self(value))
            }

            /// Returns the exact identifier.
            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }

            /// Returns the identifier's byte length.
            #[must_use]
            pub fn len(&self) -> usize {
                self.0.len()
            }

            /// Returns whether the identifier is empty.
            ///
            /// Valid instances cannot be empty.
            #[must_use]
            pub const fn is_empty(&self) -> bool {
                false
            }

            /// Consumes the identifier and returns its string.
            #[must_use]
            pub fn into_string(self) -> String {
                self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(
                &self,
                formatter: &mut fmt::Formatter<'_>,
            ) -> fmt::Result {
                formatter.write_str(self.as_str())
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(
                deserializer: D,
            ) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let value = String::deserialize(deserializer)?;

                Self::new(value)
                    .map_err(D::Error::custom)
            }
        }
    };
}

/// Identity of an individual physical topology node.
define_topology_identifier!(
    TopologyNodeId,
    "Stable identity of a physical topology node.",
    "topology node identifier"
);

/// Identity of an individual physical interconnect.
define_topology_identifier!(
    TopologyLinkId,
    "Stable identity of a physical topology interconnect.",
    "topology link identifier"
);

/// Open-world classification of physical topology nodes.
define_topology_identifier!(
    TopologyNodeKind,
    "Extensible classification of physical topology nodes.",
    "topology node kind"
);

/// Open-world classification of physical interconnects.
define_topology_identifier!(
    TopologyLinkKind,
    "Extensible classification of physical topology interconnects.",
    "topology link kind"
);

/// Revision of an observed physical topology.
///
/// This is an opaque provider-independent revision identifier.
///
/// It is deliberately not represented by a bounded integer counter.
/// Providers may use their own revision schemes.
define_topology_identifier!(
    TopologyRevision,
    "Revision identity of an observed physical topology.",
    "topology revision"
);

// ============================================================================
// PHYSICAL NODE
// ============================================================================

/// A physical node within a hardware topology.
///
/// A node may represent a physical device, subsystem, interconnect
/// endpoint, memory component, processing component or another
/// provider-described physical entity.
///
/// `kind` is open-world. New hardware technologies do not require
/// changes to this type.
///
/// `device` optionally associates the node with an existing canonical
/// provider-scoped hardware device identity.
///
/// Not every physical component must be exposed as an independent
/// standard-library device.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct TopologyNode {
    id: TopologyNodeId,
    kind: TopologyNodeKind,
    parent: Option<TopologyNodeId>,
    device: Option<HardwareDeviceIdentity>,
    attributes: BTreeMap<String, String>,
}

impl TopologyNode {
    /// Creates a physical topology node.
    pub fn new(
        id: TopologyNodeId,
        kind: TopologyNodeKind,
    ) -> Self {
        Self {
            id,
            kind,
            parent: None,
            device: None,
            attributes: BTreeMap::new(),
        }
    }

    /// Sets the physical hierarchy parent.
    #[must_use]
    pub fn with_parent(
        mut self,
        parent: TopologyNodeId,
    ) -> Self {
        self.parent = Some(parent);
        self
    }

    /// Associates the node with an existing hardware device.
    #[must_use]
    pub fn with_device(
        mut self,
        device: HardwareDeviceIdentity,
    ) -> Self {
        self.device = Some(device);
        self
    }

    /// Adds or replaces a descriptive attribute.
    ///
    /// Attribute names must be valid nonempty identifiers.
    ///
    /// Attribute values must be nonempty and contain no control
    /// characters.
    ///
    /// Attributes are descriptive and must not contain secrets.
    pub fn with_attribute(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> TopologyResult<Self> {
        let key = key.into();
        let value = value.into();

        validate_identifier(
            &key,
            "topology node attribute name",
        )?;

        validate_description(
            &value,
            "topology node attribute value",
        )?;

        self.attributes.insert(key, value);

        Ok(self)
    }

    /// Returns the node identity.
    #[must_use]
    pub fn id(&self) -> &TopologyNodeId {
        &self.id
    }

    /// Returns the physical node classification.
    #[must_use]
    pub fn kind(&self) -> &TopologyNodeKind {
        &self.kind
    }

    /// Returns the hierarchy parent.
    #[must_use]
    pub fn parent(&self) -> Option<&TopologyNodeId> {
        self.parent.as_ref()
    }

    /// Returns the associated physical device identity.
    #[must_use]
    pub fn device(&self) -> Option<&HardwareDeviceIdentity> {
        self.device.as_ref()
    }

    /// Returns descriptive attributes.
    #[must_use]
    pub fn attributes(&self) -> &BTreeMap<String, String> {
        &self.attributes
    }
}

impl<'de> Deserialize<'de> for TopologyNode {
    fn deserialize<D>(
        deserializer: D,
    ) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct WireNode {
            id: TopologyNodeId,
            kind: TopologyNodeKind,
            parent: Option<TopologyNodeId>,
            device: Option<HardwareDeviceIdentity>,
            attributes: BTreeMap<String, String>,
        }

        let wire = WireNode::deserialize(deserializer)?;

        let mut node = Self::new(
            wire.id,
            wire.kind,
        );

        node.parent = wire.parent;
        node.device = wire.device;

        for (key, value) in wire.attributes {
            node = node
                .with_attribute(key, value)
                .map_err(D::Error::custom)?;
        }

        Ok(node)
    }
}

// ============================================================================
// CONNECTION DIRECTION
// ============================================================================

/// Directionality of a physical interconnect.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    Deserialize,
)]
pub enum LinkDirection {
    /// The connection is represented as source-to-target.
    ///
    /// Traversal in the reverse direction is not implied.
    Directed,

    /// The physical connection is represented without a direction.
    Undirected,

    /// The physical connection supports both directions.
    Bidirectional,
}

impl LinkDirection {
    /// Returns whether traversal is permitted from source to target.
    #[must_use]
    pub const fn permits_forward(self) -> bool {
        true
    }

    /// Returns whether traversal is permitted from target to source.
    #[must_use]
    pub const fn permits_reverse(self) -> bool {
        !matches!(self, Self::Directed)
    }
}

/// Controls interpretation of connection direction during
/// structural topology inspection.
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    Hash,
    PartialEq,
)]
pub enum TraversalMode {
    /// Respect directed interconnects.
    #[default]
    RespectDirection,

    /// Treat all interconnects as undirected physical adjacency.
    ///
    /// This mode does not establish that reverse execution or
    /// communication is supported.
    IgnoreDirection,
}

// ============================================================================
// PHYSICAL INTERCONNECT
// ============================================================================

/// A physical interconnect between two topology nodes.
///
/// Parallel interconnects are permitted.
///
/// For example, two devices may have multiple independent physical
/// connections with different technologies or characteristics.
///
/// Link properties are descriptive. They do not establish measured
/// performance, guaranteed bandwidth or successful communication.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TopologyLink {
    id: TopologyLinkId,
    kind: TopologyLinkKind,
    source: TopologyNodeId,
    target: TopologyNodeId,
    direction: LinkDirection,
    attributes: BTreeMap<String, String>,
}

impl TopologyLink {
    /// Creates a physical interconnect.
    pub fn new(
        id: TopologyLinkId,
        kind: TopologyLinkKind,
        source: TopologyNodeId,
        target: TopologyNodeId,
        direction: LinkDirection,
    ) -> Self {
        Self {
            id,
            kind,
            source,
            target,
            direction,
            attributes: BTreeMap::new(),
        }
    }

    /// Adds or replaces a descriptive interconnect attribute.
    pub fn with_attribute(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> TopologyResult<Self> {
        let key = key.into();
        let value = value.into();

        validate_identifier(
            &key,
            "topology link attribute name",
        )?;

        validate_description(
            &value,
            "topology link attribute value",
        )?;

        self.attributes.insert(key, value);

        Ok(self)
    }

    /// Returns the link identity.
    #[must_use]
    pub fn id(&self) -> &TopologyLinkId {
        &self.id
    }

    /// Returns the link classification.
    #[must_use]
    pub fn kind(&self) -> &TopologyLinkKind {
        &self.kind
    }

    /// Returns the source endpoint.
    #[must_use]
    pub fn source(&self) -> &TopologyNodeId {
        &self.source
    }

    /// Returns the target endpoint.
    #[must_use]
    pub fn target(&self) -> &TopologyNodeId {
        &self.target
    }

    /// Returns the connection directionality.
    #[must_use]
    pub const fn direction(&self) -> LinkDirection {
        self.direction
    }

    /// Returns descriptive attributes.
    #[must_use]
    pub fn attributes(&self) -> &BTreeMap<String, String> {
        &self.attributes
    }
}

// ============================================================================
// IMMUTABLE PHYSICAL TOPOLOGY
// ============================================================================

/// Immutable, validated physical hardware topology.
///
/// Topologies are provider-independent structural descriptions.
///
/// They do not perform discovery, routing, resource allocation,
/// scheduling, execution or physical hardware communication.
///
/// A new observation should produce a new topology snapshot rather
/// than silently mutating an existing snapshot.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct HardwareTopology {
    schema_version: u16,
    id: HardwareTopologyId,
    revision: TopologyRevision,
    nodes: BTreeMap<TopologyNodeId, TopologyNode>,
    links: BTreeMap<TopologyLinkId, TopologyLink>,
}

impl HardwareTopology {
    /// Constructs a validated immutable physical topology.
    ///
    /// An empty topology is valid. It can represent an environment
    /// in which no physical devices have yet been observed.
    ///
    /// # Errors
    ///
    /// Returns an error for duplicate identities, missing endpoints,
    /// missing hierarchy parents or hierarchy cycles.
    pub fn new(
        id: HardwareTopologyId,
        revision: TopologyRevision,
        nodes: impl IntoIterator<Item = TopologyNode>,
        links: impl IntoIterator<Item = TopologyLink>,
    ) -> TopologyResult<Self> {
        let mut node_map = BTreeMap::new();

        for node in nodes {
            let node_id = node.id.clone();

            if node_map.insert(node_id.clone(), node).is_some() {
                return Err(topology_error(
                    TopologyErrorKind::DuplicateNode,
                    Some(node_id.as_str()),
                    "duplicate topology node identity",
                ));
            }
        }

        let mut link_map = BTreeMap::new();

        for link in links {
            let link_id = link.id.clone();

            if link_map.insert(link_id.clone(), link).is_some() {
                return Err(topology_error(
                    TopologyErrorKind::DuplicateLink,
                    Some(link_id.as_str()),
                    "duplicate topology link identity",
                ));
            }
        }

        let topology = Self {
            schema_version: TOPOLOGY_SCHEMA_VERSION,
            id,
            revision,
            nodes: node_map,
            links: link_map,
        };

        topology.validate()?;

        Ok(topology)
    }

    /// Returns the topology schema version.
    #[must_use]
    pub const fn schema_version(&self) -> u16 {
        self.schema_version
    }

    /// Returns the stable topology identity.
    #[must_use]
    pub fn id(&self) -> &HardwareTopologyId {
        &self.id
    }

    /// Returns the topology revision.
    #[must_use]
    pub fn revision(&self) -> &TopologyRevision {
        &self.revision
    }

    /// Returns the number of physical nodes.
    #[must_use]
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Returns the number of physical interconnects.
    #[must_use]
    pub fn link_count(&self) -> usize {
        self.links.len()
    }

    /// Returns whether the topology contains no nodes.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Returns whether the topology contains a node.
    #[must_use]
    pub fn contains_node(
        &self,
        id: &TopologyNodeId,
    ) -> bool {
        self.nodes.contains_key(id)
    }

    /// Returns whether the topology contains a link.
    #[must_use]
    pub fn contains_link(
        &self,
        id: &TopologyLinkId,
    ) -> bool {
        self.links.contains_key(id)
    }

    /// Looks up a physical node.
    #[must_use]
    pub fn node(
        &self,
        id: &TopologyNodeId,
    ) -> Option<&TopologyNode> {
        self.nodes.get(id)
    }

    /// Looks up a physical interconnect.
    #[must_use]
    pub fn link(
        &self,
        id: &TopologyLinkId,
    ) -> Option<&TopologyLink> {
        self.links.get(id)
    }

    /// Iterates over nodes in deterministic identity order.
    pub fn nodes(
        &self,
    ) -> impl ExactSizeIterator<Item = &TopologyNode> {
        self.nodes.values()
    }

    /// Iterates over links in deterministic identity order.
    pub fn links(
        &self,
    ) -> impl ExactSizeIterator<Item = &TopologyLink> {
        self.links.values()
    }

    /// Returns the direct children of a node.
    ///
    /// Results are in deterministic identity order.
    #[must_use]
    pub fn children(
        &self,
        parent: &TopologyNodeId,
    ) -> Vec<&TopologyNode> {
        self.nodes
            .values()
            .filter(|node| {
                node.parent.as_ref() == Some(parent)
            })
            .collect()
    }

    /// Returns the hierarchy ancestors of a node.
    ///
    /// The immediate parent is returned first.
    ///
    /// A validated topology cannot contain hierarchy cycles.
    #[must_use]
    pub fn ancestors(
        &self,
        id: &TopologyNodeId,
    ) -> Vec<&TopologyNode> {
        let mut result = Vec::new();

        let mut current = self.node(id);

        while let Some(node) = current {
            let Some(parent_id) = node.parent() else {
                break;
            };

            current = self.node(parent_id);

            if let Some(parent) = current {
                result.push(parent);
            }
        }

        result
    }

    /// Returns all links incident to a node.
    ///
    /// Directionality is preserved in the returned link descriptors.
    #[must_use]
    pub fn incident_links(
        &self,
        id: &TopologyNodeId,
    ) -> Vec<&TopologyLink> {
        self.links
            .values()
            .filter(|link| {
                link.source == *id || link.target == *id
            })
            .collect()
    }

    /// Returns direct neighbouring nodes.
    ///
    /// The selected traversal mode determines how directed links
    /// are interpreted.
    ///
    /// Results are deduplicated and deterministically ordered.
    #[must_use]
    pub fn neighbours(
        &self,
        id: &TopologyNodeId,
        mode: TraversalMode,
    ) -> Vec<&TopologyNode> {
        if !self.contains_node(id) {
            return Vec::new();
        }

        let mut neighbours = BTreeSet::new();

        for link in self.links.values() {
            if link.source == *id {
                if let Some(target) = self.node(&link.target) {
                    neighbours.insert(target.id());
                }
            }

            if link.target == *id
                && (mode == TraversalMode::IgnoreDirection
                    || link.direction.permits_reverse())
            {
                if let Some(source) = self.node(&link.source) {
                    neighbours.insert(source.id());
                }
            }
        }

        neighbours
            .into_iter()
            .filter_map(|node_id| self.node(node_id))
            .collect()
    }

    /// Validates all topology invariants.
    ///
    /// This method is also used after deserialization.
    pub fn validate(&self) -> TopologyResult<()> {
        if self.schema_version != TOPOLOGY_SCHEMA_VERSION {
            return Err(topology_error(
                TopologyErrorKind::UnsupportedSchema,
                None,
                "unsupported physical topology schema",
            ));
        }

        for node in self.nodes.values() {
            if let Some(parent_id) = node.parent() {
                if node.id() == parent_id {
                    return Err(topology_error(
                        TopologyErrorKind::SelfParent,
                        Some(node.id().as_str()),
                        "a topology node cannot be its own parent",
                    ));
                }

                if !self.nodes.contains_key(parent_id) {
                    return Err(topology_error(
                        TopologyErrorKind::MissingParent,
                        Some(parent_id.as_str()),
                        "topology hierarchy references a missing parent",
                    ));
                }
            }

            for (key, value) in node.attributes() {
                validate_identifier(
                    key,
                    "topology node attribute name",
                )?;

                validate_description(
                    value,
                    "topology node attribute value",
                )?;
            }
        }

        for link in self.links.values() {
            if !self.nodes.contains_key(link.source()) {
                return Err(topology_error(
                    TopologyErrorKind::MissingEndpoint,
                    Some(link.source().as_str()),
                    "topology link references a missing source node",
                ));
            }

            if !self.nodes.contains_key(link.target()) {
                return Err(topology_error(
                    TopologyErrorKind::MissingEndpoint,
                    Some(link.target().as_str()),
                    "topology link references a missing target node",
                ));
            }

            for (key, value) in link.attributes() {
                validate_identifier(
                    key,
                    "topology link attribute name",
                )?;

                validate_description(
                    value,
                    "topology link attribute value",
                )?;
            }
        }

        self.validate_hierarchy()?;

        Ok(())
    }

    /// Validates hierarchy acyclicity.
    ///
    /// The implementation uses iterative traversal rather than
    /// recursive calls, avoiding call-stack growth with hierarchy depth.
    fn validate_hierarchy(&self) -> TopologyResult<()> {
        // 0 = unseen
        // 1 = currently visiting
        // 2 = completely validated
        let mut states: BTreeMap<TopologyNodeId, u8> =
            BTreeMap::new();

        for start in self.nodes.keys() {
            if states.get(start) == Some(&2) {
                continue;
            }

            let mut path = Vec::new();
            let mut current = Some(start);

            while let Some(id) = current {
                match states.get(id).copied().unwrap_or(0) {
                    2 => break,

                    1 => {
                        return Err(topology_error(
                            TopologyErrorKind::HierarchyCycle,
                            Some(id.as_str()),
                            "physical topology hierarchy contains a cycle",
                        ));
                    }

                    _ => {}
                }

                states.insert(id.clone(), 1);
                path.push(id.clone());

                current = self
                    .nodes
                    .get(id)
                    .and_then(TopologyNode::parent)
                    .cloned();
            }

            for id in path {
                states.insert(id, 2);
            }
        }

        Ok(())
    }

    /// Returns whether every node belongs to one weakly connected
    /// physical component.
    ///
    /// An empty topology is considered connected by the conventional
    /// vacuous interpretation.
    ///
    /// Directed links are treated as undirected for this structural
    /// connectivity query.
    #[must_use]
    pub fn is_weakly_connected(&self) -> bool {
        if self.nodes.len() <= 1 {
            return true;
        }

        let Some(start) = self.nodes.keys().next() else {
            return true;
        };

        let mut visited = BTreeSet::new();
        let mut pending = vec![start.clone()];

        while let Some(current) = pending.pop() {
            if !visited.insert(current.clone()) {
                continue;
            }

            for neighbour in self.neighbours(
                &current,
                TraversalMode::IgnoreDirection,
            ) {
                if !visited.contains(neighbour.id()) {
                    pending.push(neighbour.id().clone());
                }
            }
        }

        visited.len() == self.nodes.len()
    }

    /// Returns the directedly reachable nodes from a starting node.
    ///
    /// The starting node is included.
    ///
    /// Returns an empty collection if the requested node does not exist.
    #[must_use]
    pub fn reachable(
        &self,
        start: &TopologyNodeId,
        mode: TraversalMode,
    ) -> Vec<&TopologyNode> {
        if !self.contains_node(start) {
            return Vec::new();
        }

        let mut visited = BTreeSet::new();
        let mut pending = vec![start.clone()];

        while let Some(current) = pending.pop() {
            if !visited.insert(current.clone()) {
                continue;
            }

            for neighbour in self.neighbours(&current, mode) {
                if !visited.contains(neighbour.id()) {
                    pending.push(neighbour.id().clone());
                }
            }
        }

        visited
            .iter()
            .filter_map(|id| self.node(id))
            .collect()
    }
}

// ============================================================================
// VALIDATED DESERIALIZATION
// ============================================================================

impl<'de> Deserialize<'de> for HardwareTopology {
    fn deserialize<D>(
        deserializer: D,
    ) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct WireTopology {
            schema_version: u16,
            id: HardwareTopologyId,
            revision: TopologyRevision,
            nodes: BTreeMap<TopologyNodeId, TopologyNode>,
            links: BTreeMap<TopologyLinkId, TopologyLink>,
        }

        let wire = WireTopology::deserialize(deserializer)?;

        if wire.schema_version != TOPOLOGY_SCHEMA_VERSION {
            return Err(D::Error::custom(
                "unsupported physical topology schema version",
            ));
        }

        // Reject mismatched map keys and embedded identities.
        for (key, node) in &wire.nodes {
            if key != node.id() {
                return Err(D::Error::custom(
                    "topology node map key does not match node identity",
                ));
            }
        }

        for (key, link) in &wire.links {
            if key != link.id() {
                return Err(D::Error::custom(
                    "topology link map key does not match link identity",
                ));
            }
        }

        HardwareTopology::new(
            wire.id,
            wire.revision,
            wire.nodes.into_values(),
            wire.links.into_values(),
        )
        .map_err(D::Error::custom)
    }
}

// ============================================================================
// BUILDER
// ============================================================================

/// Mutable construction helper for immutable hardware topologies.
///
/// The builder accumulates descriptions and performs complete validation
/// when `build` is called.
///
/// It does not discover hardware or reserve physical resources.
#[derive(Clone, Debug)]
pub struct HardwareTopologyBuilder {
    id: HardwareTopologyId,
    revision: TopologyRevision,
    nodes: Vec<TopologyNode>,
    links: Vec<TopologyLink>,
}

impl HardwareTopologyBuilder {
    /// Creates a new topology builder.
    #[must_use]
    pub fn new(
        id: HardwareTopologyId,
        revision: TopologyRevision,
    ) -> Self {
        Self {
            id,
            revision,
            nodes: Vec::new(),
            links: Vec::new(),
        }
    }

    /// Adds a physical topology node.
    #[must_use]
    pub fn add_node(
        mut self,
        node: TopologyNode,
    ) -> Self {
        self.nodes.push(node);
        self
    }

    /// Adds a physical interconnect.
    #[must_use]
    pub fn add_link(
        mut self,
        link: TopologyLink,
    ) -> Self {
        self.links.push(link);
        self
    }

    /// Builds and validates an immutable topology.
    pub fn build(self) -> TopologyResult<HardwareTopology> {
        HardwareTopology::new(
            self.id,
            self.revision,
            self.nodes,
            self.links,
        )
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn topology_id() -> HardwareTopologyId {
        HardwareTopologyId::new("test.hardware.topology")
            .expect("valid topology identity")
    }

    fn revision() -> TopologyRevision {
        TopologyRevision::new("revision.initial")
            .expect("valid revision")
    }

    fn node_id(value: &str) -> TopologyNodeId {
        TopologyNodeId::new(value)
            .expect("valid node identity")
    }

    fn link_id(value: &str) -> TopologyLinkId {
        TopologyLinkId::new(value)
            .expect("valid link identity")
    }

    fn node(value: &str) -> TopologyNode {
        TopologyNode::new(
            node_id(value),
            TopologyNodeKind::new("hardware.component")
                .expect("valid node kind"),
        )
    }

    fn link(
        id: &str,
        source: &str,
        target: &str,
        direction: LinkDirection,
    ) -> TopologyLink {
        TopologyLink::new(
            link_id(id),
            TopologyLinkKind::new("hardware.interconnect")
                .expect("valid link kind"),
            node_id(source),
            node_id(target),
            direction,
        )
    }

    fn empty_topology() -> HardwareTopology {
        HardwareTopology::new(
            topology_id(),
            revision(),
            [],
            [],
        )
        .expect("valid empty topology")
    }

    // ------------------------------------------------------------------------
    // Identity and extensibility
    // ------------------------------------------------------------------------

    #[test]
    fn identifiers_are_open_world() {
        let kind = TopologyNodeKind::new(
            "future.photonic.optical_component",
        )
        .unwrap();

        assert_eq!(
            kind.as_str(),
            "future.photonic.optical_component",
        );
    }

    #[test]
    fn identifiers_preserve_exact_spelling() {
        let id = node_id("Vendor.Custom.Component");

        assert_eq!(
            id.as_str(),
            "Vendor.Custom.Component",
        );
    }

    #[test]
    fn identifiers_reject_empty_values() {
        assert!(
            TopologyNodeId::new("").is_err()
        );
    }

    #[test]
    fn identifiers_reject_whitespace() {
        assert!(
            TopologyNodeId::new("node one").is_err()
        );

        assert!(
            TopologyNodeId::new(" node").is_err()
        );
    }

    #[test]
    fn identifiers_reject_control_characters() {
        assert!(
            TopologyNodeId::new("node\none").is_err()
        );
    }

    // ------------------------------------------------------------------------
    // Topology construction
    // ------------------------------------------------------------------------

    #[test]
    fn empty_topology_is_valid() {
        let topology = empty_topology();

        assert!(topology.is_empty());
        assert_eq!(topology.node_count(), 0);
        assert_eq!(topology.link_count(), 0);
    }

    #[test]
    fn topology_accepts_one_node() {
        let topology = HardwareTopology::new(
            topology_id(),
            revision(),
            [node("processor")],
            [],
        )
        .unwrap();

        assert_eq!(topology.node_count(), 1);
    }

    #[test]
    fn topology_rejects_duplicate_nodes() {
        let result = HardwareTopology::new(
            topology_id(),
            revision(),
            [node("processor"), node("processor")],
            [],
        );

        assert_eq!(
            result.unwrap_err().kind(),
            TopologyErrorKind::DuplicateNode,
        );
    }

    #[test]
    fn topology_rejects_duplicate_link_ids() {
        let result = HardwareTopology::new(
            topology_id(),
            revision(),
            [node("a"), node("b")],
            [
                link("connection", "a", "b", LinkDirection::Directed),
                link("connection", "a", "b", LinkDirection::Bidirectional),
            ],
        );

        assert_eq!(
            result.unwrap_err().kind(),
            TopologyErrorKind::DuplicateLink,
        );
    }

    // ------------------------------------------------------------------------
    // Interconnects
    // ------------------------------------------------------------------------

    #[test]
    fn topology_rejects_missing_source() {
        let result = HardwareTopology::new(
            topology_id(),
            revision(),
            [node("b")],
            [link("connection", "a", "b", LinkDirection::Directed)],
        );

        assert_eq!(
            result.unwrap_err().kind(),
            TopologyErrorKind::MissingEndpoint,
        );
    }

    #[test]
    fn topology_rejects_missing_target() {
        let result = HardwareTopology::new(
            topology_id(),
            revision(),
            [node("a")],
            [link("connection", "a", "b", LinkDirection::Directed)],
        );

        assert_eq!(
            result.unwrap_err().kind(),
            TopologyErrorKind::MissingEndpoint,
        );
    }

    #[test]
    fn parallel_interconnects_are_supported() {
        let topology = HardwareTopology::new(
            topology_id(),
            revision(),
            [node("a"), node("b")],
            [
                link("optical", "a", "b", LinkDirection::Bidirectional),
                link("electrical", "a", "b", LinkDirection::Directed),
            ],
        )
        .unwrap();

        assert_eq!(topology.link_count(), 2);
    }

    #[test]
    fn link_directionality_is_preserved() {
        let topology = HardwareTopology::new(
            topology_id(),
            revision(),
            [node("a"), node("b")],
            [link("connection", "a", "b", LinkDirection::Directed)],
        )
        .unwrap();

        assert_eq!(
            topology.link(&link_id("connection")).unwrap().direction(),
            LinkDirection::Directed,
        );
    }

    #[test]
    fn incident_links_include_both_endpoints() {
        let topology = HardwareTopology::new(
            topology_id(),
            revision(),
            [node("a"), node("b")],
            [link("connection", "a", "b", LinkDirection::Directed)],
        )
        .unwrap();

        assert_eq!(
            topology.incident_links(&node_id("a")).len(),
            1,
        );

        assert_eq!(
            topology.incident_links(&node_id("b")).len(),
            1,
        );
    }

    // ------------------------------------------------------------------------
    // Traversal
    // ------------------------------------------------------------------------

    #[test]
    fn directed_traversal_respects_direction() {
        let topology = HardwareTopology::new(
            topology_id(),
            revision(),
            [node("a"), node("b")],
            [link("connection", "a", "b", LinkDirection::Directed)],
        )
        .unwrap();

        assert_eq!(
            topology.neighbours(
                &node_id("a"),
                TraversalMode::RespectDirection,
            ).len(),
            1,
        );

        assert!(
            topology.neighbours(
                &node_id("b"),
                TraversalMode::RespectDirection,
            ).is_empty()
        );
    }

    #[test]
    fn undirected_inspection_ignores_direction() {
        let topology = HardwareTopology::new(
            topology_id(),
            revision(),
            [node("a"), node("b")],
            [link("connection", "a", "b", LinkDirection::Directed)],
        )
        .unwrap();

        assert_eq!(
            topology.neighbours(
                &node_id("b"),
                TraversalMode::IgnoreDirection,
            ).len(),
            1,
        );
    }

    #[test]
    fn reachable_includes_start_node() {
        let topology = HardwareTopology::new(
            topology_id(),
            revision(),
            [node("a"), node("b")],
            [link("connection", "a", "b", LinkDirection::Directed)],
        )
        .unwrap();

        let reachable = topology.reachable(
            &node_id("a"),
            TraversalMode::RespectDirection,
        );

        assert_eq!(reachable.len(), 2);
    }

    #[test]
    fn unknown_node_has_no_neighbours() {
        let topology = empty_topology();

        assert!(
            topology.neighbours(
                &node_id("unknown"),
                TraversalMode::IgnoreDirection,
            ).is_empty()
        );
    }

    // ------------------------------------------------------------------------
    // Hierarchy
    // ------------------------------------------------------------------------

    #[test]
    fn hierarchical_topology_is_supported() {
        let root = node("machine");

        let child = node("processor")
            .with_parent(node_id("machine"));

        let topology = HardwareTopology::new(
            topology_id(),
            revision(),
            [root, child],
            [],
        )
        .unwrap();

        assert_eq!(
            topology.children(&node_id("machine")).len(),
            1,
        );

        assert_eq!(
            topology.ancestors(&node_id("processor")).len(),
            1,
        );
    }

    #[test]
    fn hierarchy_rejects_missing_parent() {
        let result = HardwareTopology::new(
            topology_id(),
            revision(),
            [
                node("processor")
                    .with_parent(node_id("missing")),
            ],
            [],
        );

        assert_eq!(
            result.unwrap_err().kind(),
            TopologyErrorKind::MissingParent,
        );
    }

    #[test]
    fn hierarchy_rejects_self_parent() {
        let result = HardwareTopology::new(
            topology_id(),
            revision(),
            [
                node("processor")
                    .with_parent(node_id("processor")),
            ],
            [],
        );

        assert_eq!(
            result.unwrap_err().kind(),
            TopologyErrorKind::SelfParent,
        );
    }

    #[test]
    fn hierarchy_rejects_cycles() {
        let result = HardwareTopology::new(
            topology_id(),
            revision(),
            [
                node("a").with_parent(node_id("b")),
                node("b").with_parent(node_id("a")),
            ],
            [],
        );

        assert_eq!(
            result.unwrap_err().kind(),
            TopologyErrorKind::HierarchyCycle,
        );
    }

    // ------------------------------------------------------------------------
    // Connectivity
    // ------------------------------------------------------------------------

    #[test]
    fn connected_topology_is_recognized() {
        let topology = HardwareTopology::new(
            topology_id(),
            revision(),
            [node("a"), node("b"), node("c")],
            [
                link("ab", "a", "b", LinkDirection::Bidirectional),
                link("bc", "b", "c", LinkDirection::Bidirectional),
            ],
        )
        .unwrap();

        assert!(topology.is_weakly_connected());
    }

    #[test]
    fn disconnected_topology_is_recognized() {
        let topology = HardwareTopology::new(
            topology_id(),
            revision(),
            [node("a"), node("b"), node("c")],
            [link("ab", "a", "b", LinkDirection::Directed)],
        )
        .unwrap();

        assert!(!topology.is_weakly_connected());
    }

    #[test]
    fn empty_topology_is_weakly_connected() {
        assert!(empty_topology().is_weakly_connected());
    }

    // ------------------------------------------------------------------------
    // Descriptive attributes
    // ------------------------------------------------------------------------

    #[test]
    fn node_attributes_are_extensible() {
        let node = node("processor")
            .with_attribute(
                "architecture.family",
                "future.architecture",
            )
            .unwrap();

        assert_eq!(
            node.attributes()
                .get("architecture.family")
                .unwrap(),
            "future.architecture",
        );
    }

    #[test]
    fn node_attributes_reject_invalid_names() {
        assert!(
            node("processor")
                .with_attribute("", "value")
                .is_err()
        );
    }

    #[test]
    fn link_attributes_are_extensible() {
        let link = link(
            "connection",
            "a",
            "b",
            LinkDirection::Bidirectional,
        )
        .with_attribute("transport.kind", "optical")
        .unwrap();

        assert_eq!(
            link.attributes().get("transport.kind").unwrap(),
            "optical",
        );
    }

    // ------------------------------------------------------------------------
    // Builder
    // ------------------------------------------------------------------------

    #[test]
    fn builder_creates_valid_topology() {
        let topology = HardwareTopologyBuilder::new(
            topology_id(),
            revision(),
        )
        .add_node(node("a"))
        .add_node(node("b"))
        .add_link(link(
            "connection",
            "a",
            "b",
            LinkDirection::Bidirectional,
        ))
        .build()
        .unwrap();

        assert_eq!(topology.node_count(), 2);
        assert_eq!(topology.link_count(), 1);
    }

    // ------------------------------------------------------------------------
    // Serialization
    // ------------------------------------------------------------------------

    #[test]
    fn topology_serialization_round_trips() {
        let original = HardwareTopologyBuilder::new(
            topology_id(),
            revision(),
        )
        .add_node(node("a"))
        .add_node(node("b"))
        .add_link(link(
            "connection",
            "a",
            "b",
            LinkDirection::Bidirectional,
        ))
        .build()
        .unwrap();

        let encoded = serde_json::to_string(&original).unwrap();

        let restored: HardwareTopology =
            serde_json::from_str(&encoded).unwrap();

        assert_eq!(original, restored);
    }

    #[test]
    fn topology_deserialization_rejects_invalid_hierarchy() {
        let serialized = serde_json::json!({
            "schema_version": TOPOLOGY_SCHEMA_VERSION,
            "id": "test.hardware.topology",
            "revision": "revision.initial",
            "nodes": {
                "a": {
                    "id": "a",
                    "kind": "hardware.component",
                    "parent": "missing",
                    "device": null,
                    "attributes": {}
                }
            },
            "links": {}
        });

        assert!(
            serde_json::from_value::<HardwareTopology>(
                serialized
            ).is_err()
        );
    }
}
