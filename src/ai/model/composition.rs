//! Model composition for Zamani AI.
//!
//! This module describes how model instances are connected. It does not
//! execute models, allocate tensors, select hardware, schedule work, or define
//! a competing intermediate representation.
//!
//! # Ownership
//!
//! - Model definitions and signatures are owned by the canonical model layer.
//! - Semantic types and effects are owned by the frontend semantic layer.
//! - Universal computational operations are owned by ZUIR.
//! - Cross-domain composition is owned by `src/hybrid/`.
//! - Placement and distributed scheduling are owned by `src/distributed/`.
//! - Execution is owned by the runtime.
//!
//! This module stores stable references to those authorities and validates
//! composition-level structural invariants.
//!
//! # Scalability
//!
//! No fixed upper bound is imposed on the number of members, connections,
//! inputs, outputs, or composition levels. Actual memory and processing
//! requirements are determined by the host and by configurable operational
//! budgets outside this module.
//!
//! # Compatibility
//!
//! The module uses only the Rust standard library and safe Rust.

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fmt;

/// Stable identifier for a model composition.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CompositionId(String);

impl CompositionId {
    /// Creates a composition identifier.
    ///
    /// Identifiers must not be empty or contain surrounding whitespace.
    pub fn new(value: impl Into<String>) -> Result<Self, CompositionError> {
        let value = value.into();

        if value.is_empty() || value.trim() != value {
            return Err(CompositionError::InvalidIdentifier {
                kind: "composition",
                value,
            });
        }

        Ok(Self(value))
    }

    /// Returns the identifier as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for CompositionId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Stable identifier for a member within one composition.
///
/// A member ID is local to its containing composition. It is not a model
/// definition ID and must not be interpreted as a globally unique identity.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MemberId(String);

impl MemberId {
    /// Creates a member identifier.
    pub fn new(value: impl Into<String>) -> Result<Self, CompositionError> {
        let value = value.into();

        if value.is_empty() || value.trim() != value {
            return Err(CompositionError::InvalidIdentifier {
                kind: "member",
                value,
            });
        }

        Ok(Self(value))
    }

    /// Returns the identifier as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for MemberId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// A stable reference to a canonical model definition.
///
/// The composition layer does not own the referenced model definition.
/// Resolution is delegated to [`ModelResolver`].
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ModelRef(String);

impl ModelRef {
    /// Creates a model reference.
    pub fn new(value: impl Into<String>) -> Result<Self, CompositionError> {
        let value = value.into();

        if value.is_empty() || value.trim() != value {
            return Err(CompositionError::InvalidIdentifier {
                kind: "model reference",
                value,
            });
        }

        Ok(Self(value))
    }

    /// Returns the reference as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ModelRef {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// A stable reference to a port in a model signature.
///
/// Port references are names, not tensor allocations or semantic types.
/// The canonical signature resolver determines whether a port exists and
/// what its type means.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PortRef(String);

impl PortRef {
    /// Creates a port reference.
    pub fn new(value: impl Into<String>) -> Result<Self, CompositionError> {
        let value = value.into();

        if value.is_empty() || value.trim() != value {
            return Err(CompositionError::InvalidIdentifier {
                kind: "port",
                value,
            });
        }

        Ok(Self(value))
    }

    /// Returns the reference as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for PortRef {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Describes the role of a member in the composition.
///
/// These are declarative roles. They do not prescribe a hardware mapping or
/// an execution strategy.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MemberRole {
    /// A normal model invocation.
    Model,

    /// A nested composition referenced by identity.
    ///
    /// The nested composition must be resolved separately by the integration
    /// layer. This reference does not inline or copy its graph.
    NestedComposition(CompositionId),

    /// A named reusable component.
    ///
    /// Reuse is represented by multiple members referring to the same
    /// canonical model definition. Each member remains a distinct invocation.
    SharedComponent,
}

/// One member of a composition.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompositionMember {
    id: MemberId,
    model: ModelRef,
    role: MemberRole,
}

impl CompositionMember {
    /// Creates a member referring to a canonical model definition.
    pub fn model(
        id: MemberId,
        model: ModelRef,
    ) -> Self {
        Self {
            id,
            model,
            role: MemberRole::Model,
        }
    }

    /// Creates a member representing a nested composition.
    pub fn nested(
        id: MemberId,
        model: ModelRef,
        nested: CompositionId,
    ) -> Self {
        Self {
            id,
            model,
            role: MemberRole::NestedComposition(nested),
        }
    }

    /// Creates a reusable shared-component member.
    pub fn shared(
        id: MemberId,
        model: ModelRef,
    ) -> Self {
        Self {
            id,
            model,
            role: MemberRole::SharedComponent,
        }
    }

    /// Returns this member's local identifier.
    pub fn id(&self) -> &MemberId {
        &self.id
    }

    /// Returns the canonical model reference.
    pub fn model(&self) -> &ModelRef {
        &self.model
    }

    /// Returns the member role.
    pub fn role(&self) -> &MemberRole {
        &self.role
    }
}

/// Identifies one endpoint in a composition data-flow connection.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Endpoint {
    member: MemberId,
    port: PortRef,
}

impl Endpoint {
    /// Creates an endpoint.
    pub fn new(member: MemberId, port: PortRef) -> Self {
        Self { member, port }
    }

    /// Returns the member identifier.
    pub fn member(&self) -> &MemberId {
        &self.member
    }

    /// Returns the port reference.
    pub fn port(&self) -> &PortRef {
        &self.port
    }
}

/// A data-flow connection between two model ports.
///
/// A connection describes a dependency. It does not perform data transfer.
/// The runtime and lowering infrastructure realize the dependency.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Connection {
    source: Endpoint,
    destination: Endpoint,
}

impl Connection {
    /// Creates a directed connection from a source output to a destination
    /// input.
    pub fn new(source: Endpoint, destination: Endpoint) -> Self {
        Self {
            source,
            destination,
        }
    }

    /// Returns the source endpoint.
    pub fn source(&self) -> &Endpoint {
        &self.source
    }

    /// Returns the destination endpoint.
    pub fn destination(&self) -> &Endpoint {
        &self.destination
    }
}

/// A composition's externally visible input mapping.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct InputBinding {
    external: PortRef,
    destination: Endpoint,
}

impl InputBinding {
    /// Creates an external-input binding.
    pub fn new(external: PortRef, destination: Endpoint) -> Self {
        Self {
            external,
            destination,
        }
    }

    /// Returns the external input port.
    pub fn external(&self) -> &PortRef {
        &self.external
    }

    /// Returns the internal destination endpoint.
    pub fn destination(&self) -> &Endpoint {
        &self.destination
    }
}

/// A composition's externally visible output mapping.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct OutputBinding {
    source: Endpoint,
    external: PortRef,
}

impl OutputBinding {
    /// Creates an external-output binding.
    pub fn new(source: Endpoint, external: PortRef) -> Self {
        Self { source, external }
    }

    /// Returns the internal source endpoint.
    pub fn source(&self) -> &Endpoint {
        &self.source
    }

    /// Returns the external output port.
    pub fn external(&self) -> &PortRef {
        &self.external
    }
}

/// Declarative model composition.
///
/// Members are keyed by their local IDs, providing deterministic iteration.
/// Connections and bindings are stored in ordered sets to make the structural
/// representation deterministic.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModelComposition {
    id: CompositionId,
    members: BTreeMap<MemberId, CompositionMember>,
    connections: BTreeSet<Connection>,
    inputs: BTreeSet<InputBinding>,
    outputs: BTreeSet<OutputBinding>,
}

impl ModelComposition {
    /// Creates an empty composition.
    pub fn new(id: CompositionId) -> Self {
        Self {
            id,
            members: BTreeMap::new(),
            connections: BTreeSet::new(),
            inputs: BTreeSet::new(),
            outputs: BTreeSet::new(),
        }
    }

    /// Returns the composition identifier.
    pub fn id(&self) -> &CompositionId {
        &self.id
    }

    /// Returns the members in deterministic identifier order.
    pub fn members(&self) -> impl Iterator<Item = &CompositionMember> {
        self.members.values()
    }

    /// Returns the connections in deterministic order.
    pub fn connections(&self) -> impl Iterator<Item = &Connection> {
        self.connections.iter()
    }

    /// Returns external input bindings.
    pub fn inputs(&self) -> impl Iterator<Item = &InputBinding> {
        self.inputs.iter()
    }

    /// Returns external output bindings.
    pub fn outputs(&self) -> impl Iterator<Item = &OutputBinding> {
        self.outputs.iter()
    }

    /// Adds a member.
    ///
    /// Duplicate member IDs are rejected. The original member remains
    /// unchanged if insertion fails.
    pub fn add_member(
        &mut self,
        member: CompositionMember,
    ) -> Result<(), CompositionError> {
        let id = member.id.clone();

        if self.members.contains_key(&id) {
            return Err(CompositionError::DuplicateMember { member: id });
        }

        self.members.insert(id, member);
        Ok(())
    }

    /// Adds a connection.
    ///
    /// This method checks endpoint membership and rejects direct self-member
    /// connections. Full port existence, direction, and type checks are
    /// performed by [`ModelResolver`] during validation.
    pub fn add_connection(
        &mut self,
        connection: Connection,
    ) -> Result<(), CompositionError> {
        self.ensure_member_exists(connection.source.member())?;
        self.ensure_member_exists(connection.destination.member())?;

        if connection.source.member() == connection.destination.member() {
            return Err(CompositionError::SelfConnection {
                member: connection.source.member().clone(),
            });
        }

        if !self.connections.insert(connection.clone()) {
            return Err(CompositionError::DuplicateConnection {
                connection,
            });
        }

        Ok(())
    }

    /// Binds an external composition input to a member input.
    pub fn add_input(
        &mut self,
        binding: InputBinding,
    ) -> Result<(), CompositionError> {
        self.ensure_member_exists(binding.destination.member())?;

        if self
            .inputs
            .iter()
            .any(|existing| existing.external == binding.external)
        {
            return Err(CompositionError::DuplicateExternalInput {
                port: binding.external,
            });
        }

        self.inputs.insert(binding);
        Ok(())
    }

    /// Binds a member output to an external composition output.
    pub fn add_output(
        &mut self,
        binding: OutputBinding,
    ) -> Result<(), CompositionError> {
        self.ensure_member_exists(binding.source.member())?;

        if self
            .outputs
            .iter()
            .any(|existing| existing.external == binding.external)
        {
            return Err(CompositionError::DuplicateExternalOutput {
                port: binding.external,
            });
        }

        self.outputs.insert(binding);
        Ok(())
    }

    /// Performs local structural validation.
    ///
    /// This verifies member references and acyclicity. It does not claim
    /// semantic validity because model definitions and signatures must be
    /// resolved by the canonical model/semantic infrastructure.
    pub fn validate_structure(&self) -> Result<(), CompositionError> {
        for connection in &self.connections {
            self.ensure_member_exists(connection.source.member())?;
            self.ensure_member_exists(connection.destination.member())?;
        }

        for input in &self.inputs {
            self.ensure_member_exists(input.destination.member())?;
        }

        for output in &self.outputs {
            self.ensure_member_exists(output.source.member())?;
        }

        self.ensure_acyclic()
    }

    fn ensure_member_exists(
        &self,
        id: &MemberId,
    ) -> Result<(), CompositionError> {
        if self.members.contains_key(id) {
            Ok(())
        } else {
            Err(CompositionError::UnknownMember {
                member: id.clone(),
            })
        }
    }

    fn ensure_acyclic(&self) -> Result<(), CompositionError> {
        let mut indegree: BTreeMap<MemberId, usize> = self
            .members
            .keys()
            .cloned()
            .map(|id| (id, 0))
            .collect();

        let mut outgoing: BTreeMap<MemberId, BTreeSet<MemberId>> =
            BTreeMap::new();

        for connection in &self.connections {
            let source = connection.source.member().clone();
            let destination = connection.destination.member().clone();

            let neighbors = outgoing.entry(source).or_default();

            // Multiple port connections between the same pair of members
            // represent one graph dependency for cycle detection.
            if neighbors.insert(destination.clone()) {
                let degree = indegree
                    .get_mut(&destination)
                    .ok_or_else(|| CompositionError::UnknownMember {
                        member: destination.clone(),
                    })?;

                *degree = degree.checked_add(1).ok_or(
                    CompositionError::DependencyCountOverflow {
                        member: destination,
                    },
                )?;
            }
        }

        let mut ready: VecDeque<MemberId> = indegree
            .iter()
            .filter_map(|(member, degree)| {
                (*degree == 0).then_some(member.clone())
            })
            .collect();

        let mut visited = 0usize;

        while let Some(member) = ready.pop_front() {
            visited = visited
                .checked_add(1)
                .ok_or(CompositionError::GraphSizeOverflow)?;

            if let Some(neighbors) = outgoing.get(&member) {
                for neighbor in neighbors {
                    let degree = indegree.get_mut(neighbor).ok_or_else(|| {
                        CompositionError::UnknownMember {
                            member: neighbor.clone(),
                        }
                    })?;

                    *degree = degree.checked_sub(1).ok_or_else(|| {
                        CompositionError::InvalidDependencyState {
                            member: neighbor.clone(),
                        }
                    })?;

                    if *degree == 0 {
                        ready.push_back(neighbor.clone());
                    }
                }
            }
        }

        if visited == self.members.len() {
            Ok(())
        } else {
            let cyclic_members = indegree
                .into_iter()
                .filter_map(|(member, degree)| (degree > 0).then_some(member))
                .collect();

            Err(CompositionError::CycleDetected {
                members: cyclic_members,
            })
        }
    }
}

/// Resolves canonical model information for composition validation.
///
/// Implement this adapter using the actual model definition and signature
/// APIs. It intentionally does not define a new semantic type system.
pub trait ModelResolver {
    /// Confirms that a model reference resolves to a canonical definition.
    fn model_exists(&self, model: &ModelRef) -> bool;

    /// Confirms that a named output port exists on the model.
    fn has_output_port(&self, model: &ModelRef, port: &PortRef) -> bool;

    /// Confirms that a named input port exists on the model.
    fn has_input_port(&self, model: &ModelRef, port: &PortRef) -> bool;

    /// Checks semantic compatibility between a source output and destination
    /// input.
    ///
    /// This must delegate type, shape, effect, and other semantic checks to
    /// the canonical semantic/type infrastructure. A `true` result means
    /// the resolver has established compatibility; it must not mean merely
    /// that the names happen to match.
    fn ports_compatible(
        &self,
        source_model: &ModelRef,
        source_port: &PortRef,
        destination_model: &ModelRef,
        destination_port: &PortRef,
    ) -> bool;
}

/// Validates a composition against the canonical model resolver.
///
/// Structural and semantic validation are kept separate so callers can
/// inspect an incomplete composition without falsely claiming it is valid.
pub fn validate<R: ModelResolver>(
    composition: &ModelComposition,
    resolver: &R,
) -> Result<(), CompositionError> {
    composition.validate_structure()?;

    for member in composition.members() {
        if !resolver.model_exists(member.model()) {
            return Err(CompositionError::UnresolvedModel {
                member: member.id().clone(),
                model: member.model().clone(),
            });
        }
    }

    for connection in composition.connections() {
        let source = composition
            .members
            .get(connection.source.member())
            .ok_or_else(|| CompositionError::UnknownMember {
                member: connection.source.member().clone(),
            })?;

        let destination = composition
            .members
            .get(connection.destination.member())
            .ok_or_else(|| CompositionError::UnknownMember {
                member: connection.destination.member().clone(),
            })?;

        if !resolver.has_output_port(
            source.model(),
            connection.source.port(),
        ) {
            return Err(CompositionError::UnknownOutputPort {
                member: source.id().clone(),
                port: connection.source.port().clone(),
            });
        }

        if !resolver.has_input_port(
            destination.model(),
            connection.destination.port(),
        ) {
            return Err(CompositionError::UnknownInputPort {
                member: destination.id().clone(),
                port: connection.destination.port().clone(),
            });
        }

        if !resolver.ports_compatible(
            source.model(),
            connection.source.port(),
            destination.model(),
            connection.destination.port(),
        ) {
            return Err(CompositionError::IncompatiblePorts {
                source: connection.source.clone(),
                destination: connection.destination.clone(),
            });
        }
    }

    for input in composition.inputs() {
        let member = composition
            .members
            .get(input.destination.member())
            .ok_or_else(|| CompositionError::UnknownMember {
                member: input.destination.member().clone(),
            })?;

        if !resolver.has_input_port(
            member.model(),
            input.destination.port(),
        ) {
            return Err(CompositionError::UnknownInputPort {
                member: member.id().clone(),
                port: input.destination.port().clone(),
            });
        }
    }

    for output in composition.outputs() {
        let member = composition
            .members
            .get(output.source.member())
            .ok_or_else(|| CompositionError::UnknownMember {
                member: output.source.member().clone(),
            })?;

        if !resolver.has_output_port(
            member.model(),
            output.source.port(),
        ) {
            return Err(CompositionError::UnknownOutputPort {
                member: member.id().clone(),
                port: output.source.port().clone(),
            });
        }
    }

    Ok(())
}

/// Composition-specific failures.
///
/// These errors describe composition invariants. Canonical semantic and
/// model-resolution failures should be translated at the integration layer
/// without losing their original diagnostic information.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CompositionError {
    InvalidIdentifier {
        kind: &'static str,
        value: String,
    },
    DuplicateMember {
        member: MemberId,
    },
    UnknownMember {
        member: MemberId,
    },
    SelfConnection {
        member: MemberId,
    },
    DuplicateConnection {
        connection: Connection,
    },
    DuplicateExternalInput {
        port: PortRef,
    },
    DuplicateExternalOutput {
        port: PortRef,
    },
    UnresolvedModel {
        member: MemberId,
        model: ModelRef,
    },
    UnknownInputPort {
        member: MemberId,
        port: PortRef,
    },
    UnknownOutputPort {
        member: MemberId,
        port: PortRef,
    },
    IncompatiblePorts {
        source: Endpoint,
        destination: Endpoint,
    },
    CycleDetected {
        members: Vec<MemberId>,
    },
    DependencyCountOverflow {
        member: MemberId,
    },
    GraphSizeOverflow,
    InvalidDependencyState {
        member: MemberId,
    },
}

impl fmt::Display for CompositionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidIdentifier { kind, value } => {
                write!(formatter, "invalid {kind} identifier: {value:?}")
            }
            Self::DuplicateMember { member } => {
                write!(formatter, "duplicate composition member: {member}")
            }
            Self::UnknownMember { member } => {
                write!(formatter, "unknown composition member: {member}")
            }
            Self::SelfConnection { member } => {
                write!(formatter, "self-member connection is not allowed: {member}")
            }
            Self::DuplicateConnection { connection } => write!(
                formatter,
                "duplicate connection: {}.{} -> {}.{}",
                connection.source.member(),
                connection.source.port(),
                connection.destination.member(),
                connection.destination.port(),
            ),
            Self::DuplicateExternalInput { port } => {
                write!(formatter, "duplicate external input binding: {port}")
            }
            Self::DuplicateExternalOutput { port } => {
                write!(formatter, "duplicate external output binding: {port}")
            }
            Self::UnresolvedModel { member, model } => {
                write!(formatter, "member {member} references unresolved model {model}")
            }
            Self::UnknownInputPort { member, port } => {
                write!(formatter, "unknown input port {member}.{port}")
            }
            Self::UnknownOutputPort { member, port } => {
                write!(formatter, "unknown output port {member}.{port}")
            }
            Self::IncompatiblePorts {
                source,
                destination,
            } => write!(
                formatter,
                "incompatible connection: {}.{} -> {}.{}",
                source.member(),
                source.port(),
                destination.member(),
                destination.port(),
            ),
            Self::CycleDetected { members } => {
                write!(formatter, "composition contains a cycle involving ")?;
                for (index, member) in members.iter().enumerate() {
                    if index > 0 {
                        formatter.write_str(", ")?;
                    }
                    write!(formatter, "{member}")?;
                }
                Ok(())
            }
            Self::DependencyCountOverflow { member } => {
                write!(formatter, "dependency count overflow for member {member}")
            }
            Self::GraphSizeOverflow => {
                formatter.write_str("composition graph size overflow")
            }
            Self::InvalidDependencyState { member } => {
                write!(formatter, "invalid dependency state for member {member}")
            }
        }
    }
}

impl std::error::Error for CompositionError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn member_id(value: &str) -> MemberId {
        MemberId::new(value).expect("valid member ID")
    }

    fn model_ref(value: &str) -> ModelRef {
        ModelRef::new(value).expect("valid model reference")
    }

    fn port_ref(value: &str) -> PortRef {
        PortRef::new(value).expect("valid port reference")
    }

    fn endpoint(member: &str, port: &str) -> Endpoint {
        Endpoint::new(member_id(member), port_ref(port))
    }

    fn member(value: &str) -> CompositionMember {
        CompositionMember::model(member_id(value), model_ref(value))
    }

    #[test]
    fn accepts_a_valid_chain() {
        let id = CompositionId::new("chain").expect("valid ID");
        let mut composition = ModelComposition::new(id);

        composition.add_member(member("first")).unwrap();
        composition.add_member(member("second")).unwrap();

        composition
            .add_connection(Connection::new(
                endpoint("first", "output"),
                endpoint("second", "input"),
            ))
            .unwrap();

        assert!(composition.validate_structure().is_ok());
    }

    #[test]
    fn rejects_duplicate_member_ids() {
        let mut composition =
            ModelComposition::new(CompositionId::new("composition").unwrap());

        composition.add_member(member("same")).unwrap();

        assert_eq!(
            composition.add_member(member("same")),
            Err(CompositionError::DuplicateMember {
                member: member_id("same"),
            })
        );
    }

    #[test]
    fn rejects_cycles() {
        let mut composition =
            ModelComposition::new(CompositionId::new("cycle").unwrap());

        composition.add_member(member("a")).unwrap();
        composition.add_member(member("b")).unwrap();

        composition
            .add_connection(Connection::new(
                endpoint("a", "out"),
                endpoint("b", "in"),
            ))
            .unwrap();

        composition
            .add_connection(Connection::new(
                endpoint("b", "out"),
                endpoint("a", "in"),
            ))
            .unwrap();

        assert!(matches!(
            composition.validate_structure(),
            Err(CompositionError::CycleDetected { .. })
        ));
    }

    #[test]
    fn allows_multiple_connections_between_the_same_members() {
        let mut composition =
            ModelComposition::new(CompositionId::new("multi-port").unwrap());

        composition.add_member(member("a")).unwrap();
        composition.add_member(member("b")).unwrap();

        composition
            .add_connection(Connection::new(
                endpoint("a", "out-1"),
                endpoint("b", "in-1"),
            ))
            .unwrap();

        composition
            .add_connection(Connection::new(
                endpoint("a", "out-2"),
                endpoint("b", "in-2"),
            ))
            .unwrap();

        assert!(composition.validate_structure().is_ok());
    }

    #[test]
    fn allows_shared_model_definitions_with_distinct_member_ids() {
        let mut composition =
            ModelComposition::new(CompositionId::new("shared").unwrap());

        composition
            .add_member(CompositionMember::shared(
                member_id("left"),
                model_ref("common-model"),
            ))
            .unwrap();

        composition
            .add_member(CompositionMember::shared(
                member_id("right"),
                model_ref("common-model"),
            ))
            .unwrap();

        assert_eq!(composition.members().count(), 2);
        assert!(composition.validate_structure().is_ok());
    }

    #[test]
    fn rejects_duplicate_external_input_names() {
        let mut composition =
            ModelComposition::new(CompositionId::new("bindings").unwrap());

        composition.add_member(member("model")).unwrap();

        composition
            .add_input(InputBinding::new(
                port_ref("input"),
                endpoint("model", "model-input"),
            ))
            .unwrap();

        assert!(matches!(
            composition.add_input(InputBinding::new(
                port_ref("input"),
                endpoint("model", "other-input"),
            )),
            Err(CompositionError::DuplicateExternalInput { .. })
        ));
    }
}