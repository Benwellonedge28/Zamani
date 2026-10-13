
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Frontend — Semantic Provenance
//!
//! Path: `src/frontend/semantic/model/provenance.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Provide one canonical, target-independent model for semantic provenance:
//! source origins, derivation, generation, attribution, dependencies,
//! decisions, verification evidence, and extensible lineage relationships.
//!
//! Provenance describes where a semantic entity or artifact came from and
//! which entities or activities contributed to it. It does not establish
//! that a claim is true, authenticate an agent, verify a digital signature,
//! or prove that an artifact is safe.
//!
//! ## AUTHORITY
//!
//! - `grammar/spec/provenance.md`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/validation/semantic-boundaries.md`
//! - `grammar/expressions/provenance.g4`
//! - `grammar/quantum/provenance.g4`
//! - `src/frontend/ast/node/node_id.rs`
//! - `src/frontend/ast/node/source/span.rs`
//! - `src/frontend/semantic/model/ids.rs`
//! - `src/frontend/semantic/model/program.rs`
//!
//! The grammar owns syntax. The AST owns source structure and node identity.
//! This file owns the semantic provenance data model and its invariants.
//!
//! ## OWNS
//!
//! - Provenance subjects, origins, relationship kinds, and records.
//! - Typed references to AST nodes and semantic entities.
//! - Deterministic provenance graph storage.
//! - Referential-integrity validation.
//! - Validation of derivation cycles.
//! - Explicit, checked provenance graph mutation.
//!
//! ## DOES_NOT_OWN
//!
//! - Parsing `provenance(...)` or any other source syntax.
//! - AST nodes, AST storage, node identity allocation, or source maps.
//! - Semantic ID allocation.
//! - Type checking, symbol resolution, or domain validation.
//! - Cryptographic hashing, signatures, attestation, or authentication.
//! - Trust decisions, policy authorization, or proof verification.
//! - Runtime tracing, persistent databases, or audit-log storage.
//! - Classical IR, `quantum::ir`, HDL IR, or another IR.
//! - Hardware discovery, resource allocation, scheduling, routing, QEC,
//!   lowering, or execution.
//!
//! ## INTEGRATION CONTRACT
//!
//! - AST references use `crate::frontend::ast::node::node_id::NodeId`.
//! - Source origins use the canonical AST `Span`.
//! - Semantic entity references use `SemanticId`.
//! - Provenance records use `ProvenanceId` from `model/ids.rs`.
//! - `model/program.rs` may store provenance records or references through
//!   its generic provenance parameter.
//! - `model/facts.rs` may associate facts with provenance without owning
//!   provenance semantics.
//! - `model/mod.rs` exports this module; it must not redefine these types.
//! - Domain validators translate their established results into these
//!   shared types rather than creating parallel provenance graphs.
//!
//! ## SCALABILITY
//!
//! No hardware or domain count is fixed. Collections grow with actual data
//! and available resources. Rust's finite address space, allocation failures,
//! and the `u64` identity representation remain implementation constraints.
//! This file introduces no artificial maximum provenance-record count.
//!
//! Graph validation uses iterative algorithms so a long lineage chain does
//! not consume the Rust call stack. Callers remain responsible for applying
//! configurable analysis-work and memory budgets.
//!
//! ## DETERMINISM
//!
//! Graph records and record IDs are ordered by `ProvenanceId`. Relationship
//! order within a record is preserved as supplied. The model uses no global
//! mutable state, wall-clock time, randomness, or machine inspection.
//!
//! Deterministic storage does not automatically make caller-supplied payloads
//! or external evidence deterministic.
//!
//! ## FREEZE CRITERIA
//!
//! - `model/ids.rs` exposes `SemanticId` and `ProvenanceId`.
//! - The canonical AST exposes `NodeId` and `Span` at the paths imported here.
//! - `model/mod.rs` exports this module.
//! - The module tests pass.
//! - The workspace builds and the relevant integration tests pass.
//!
//! This file does not claim the entire semantic subsystem is production-ready
//! merely because this module compiles.

use core::fmt;
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::error::Error;

use crate::frontend::ast::node::node_id::NodeId;
use crate::frontend::ast::node::source::Span;
use crate::frontend::semantic::model::ids::{
    ProvenanceId,
    SemanticId,
};

/// In-memory schema version for the semantic provenance model.
///
/// This version is independent of the source-language version, AST schema,
/// semantic-program schema, and downstream IR schema.
pub const PROVENANCE_MODEL_SCHEMA_VERSION: u16 = 1;

/// A subject whose provenance is being described.
///
/// These variants intentionally distinguish source syntax, semantic
/// entities, and externally named artifacts. A string is not used as a
/// substitute for an AST identity or a semantic entity identity.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum ProvenanceSubject {
    /// A node in the canonical frontend AST.
    AstNode(NodeId),

    /// An entity owned by the semantic model.
    SemanticEntity(SemanticId),

    /// An artifact identified by a caller-defined, qualified identifier.
    ///
    /// This is for artifacts that do not have an appropriate in-memory AST
    /// or semantic ID, such as a separately managed source artifact.
    Artifact(ArtifactReference),
}

impl ProvenanceSubject {
    /// Construct a subject for an AST node.
    #[must_use]
    pub const fn ast_node(node_id: NodeId) -> Self {
        Self::AstNode(node_id)
    }

    /// Construct a subject for a semantic entity.
    #[must_use]
    pub const fn semantic_entity(id: SemanticId) -> Self {
        Self::SemanticEntity(id)
    }

    /// Construct a subject for an external artifact.
    #[must_use]
    pub fn artifact(
        namespace: impl Into<String>,
        identifier: impl Into<String>,
    ) -> Result<Self, ProvenanceError> {
        Ok(Self::Artifact(ArtifactReference::new(
            namespace,
            identifier,
        )?))
    }
}

/// A qualified external artifact reference.
///
/// `namespace` and `identifier` are identifiers in the referenced system;
/// this type does not interpret them as filesystem paths, URLs, credentials,
/// content hashes, or proof of authenticity.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ArtifactReference {
    namespace: String,
    identifier: String,
}

impl ArtifactReference {
    /// Create a qualified artifact reference.
    ///
    /// Both fields must be nonempty after trimming and must contain no
    /// control characters. Their exact syntax is owned by the external
    /// artifact system.
    pub fn new(
        namespace: impl Into<String>,
        identifier: impl Into<String>,
    ) -> Result<Self, ProvenanceError> {
        let namespace = namespace.into();
        let identifier = identifier.into();

        validate_text("artifact namespace", &namespace)?;
        validate_text("artifact identifier", &identifier)?;

        Ok(Self {
            namespace,
            identifier,
        })
    }

    /// Return the artifact namespace.
    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Return the artifact identifier.
    #[must_use]
    pub fn identifier(&self) -> &str {
        &self.identifier
    }
}

/// The origin classification of a provenance record.
///
/// An origin is descriptive metadata. It is not cryptographic evidence and
/// does not establish the truth of the claim represented by the record.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ProvenanceOrigin {
    /// Directly associated with a canonical AST node and source span.
    Source(SourceOrigin),

    /// Created by a semantic, compiler, or other explicitly named activity.
    Generated(GeneratedOrigin),

    /// Originates from an independently identified artifact or authority.
    External(ExternalOrigin),

    /// The origin is not currently known or has not been recorded.
    Unknown,
}

impl Default for ProvenanceOrigin {
    fn default() -> Self {
        Self::Unknown
    }
}

/// Source-origin metadata using canonical frontend identities.
///
/// The span is the AST's source span. This type does not copy source text or
/// maintain a competing line/column coordinate system.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SourceOrigin {
    node_id: NodeId,
    span: Span,
}

impl SourceOrigin {
    /// Create a source origin for a canonical AST node.
    ///
    /// The caller must supply the span belonging to that node. Verifying
    /// that association requires AST storage and is deliberately outside
    /// this module.
    #[must_use]
    pub const fn new(node_id: NodeId, span: Span) -> Self {
        Self { node_id, span }
    }

    /// Return the canonical AST node identity.
    #[must_use]
    pub const fn node_id(&self) -> NodeId {
        self.node_id
    }

    /// Return the canonical source span.
    #[must_use]
    pub const fn span(&self) -> Span {
        self.span
    }
}

/// Describes the activity that generated an artifact or semantic result.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GeneratedOrigin {
    /// Qualified activity name, such as a compiler pass or registered tool.
    activity: String,

    /// Optional version supplied by the activity owner.
    activity_version: Option<String>,
}

impl GeneratedOrigin {
    /// Create a generated origin.
    ///
    /// Activity names are metadata, not executable commands. No code is
    /// loaded or executed by this constructor.
    pub fn new(
        activity: impl Into<String>,
        activity_version: Option<String>,
    ) -> Result<Self, ProvenanceError> {
        let activity = activity.into();
        validate_text("generating activity", &activity)?;

        if let Some(version) = activity_version.as_deref() {
            validate_text("activity version", version)?;
        }

        Ok(Self {
            activity,
            activity_version,
        })
    }

    /// Return the generating activity name.
    #[must_use]
    pub fn activity(&self) -> &str {
        &self.activity
    }

    /// Return the optional activity version.
    #[must_use]
    pub fn activity_version(&self) -> Option<&str> {
        self.activity_version.as_deref()
    }
}

/// Describes a claimed external origin.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExternalOrigin {
    /// Name of the external authority or system.
    authority: String,

    /// Opaque reference understood by that authority.
    reference: String,
}

impl ExternalOrigin {
    /// Create an external-origin claim.
    ///
    /// This records a claim only. It does not contact the authority, validate
    /// the reference, or establish authenticity.
    pub fn new(
        authority: impl Into<String>,
        reference: impl Into<String>,
    ) -> Result<Self, ProvenanceError> {
        let authority = authority.into();
        let reference = reference.into();

        validate_text("external authority", &authority)?;
        validate_text("external reference", &reference)?;

        Ok(Self {
            authority,
            reference,
        })
    }

    /// Return the external authority name.
    #[must_use]
    pub fn authority(&self) -> &str {
        &self.authority
    }

    /// Return the external reference.
    #[must_use]
    pub fn reference(&self) -> &str {
        &self.reference
    }
}

/// A provenance relationship kind.
///
/// Standard relationships have stable typed variants. `Extension` allows
/// new domain-specific relationships without editing this central enum for
/// every future computational domain.
///
/// Extension names must be qualified, for example `vendor.model-trained-by`.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum ProvenanceRelationKind {
    /// The subject was derived from the referenced provenance record.
    WasDerivedFrom,

    /// The subject was generated by the referenced activity or record.
    WasGeneratedBy,

    /// The subject used the referenced entity or artifact.
    Used,

    /// The subject is attributed to the referenced agent or entity.
    WasAttributedTo,

    /// The subject was associated with the referenced activity or agent.
    WasAssociatedWith,

    /// The subject was influenced by the referenced record.
    WasInfluencedBy,

    /// The subject was checked or verified by the referenced record.
    WasVerifiedBy,

    /// The subject was governed by the referenced policy or contract record.
    WasGovernedBy,

    /// A registered or namespaced extension relationship.
    Extension(String),
}

impl ProvenanceRelationKind {
    /// Construct a namespaced extension relationship.
    pub fn extension(
        qualified_name: impl Into<String>,
    ) -> Result<Self, ProvenanceError> {
        let name = qualified_name.into();
        validate_qualified_name("provenance relation", &name)?;
        Ok(Self::Extension(name))
    }

    /// Return whether this relationship represents derivation lineage.
    ///
    /// Derivation relationships are required to be acyclic by graph
    /// validation. Other relationships may be cyclic where their semantics
    /// permit it.
    #[must_use]
    pub const fn is_derivation(&self) -> bool {
        matches!(self, Self::WasDerivedFrom)
    }
}

/// One directed provenance relationship.
///
/// `target` refers to another provenance record. It is not a raw string and
/// is not implicitly interpreted as a hardware, device, or runtime ID.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProvenanceRelation {
    kind: ProvenanceRelationKind,
    target: ProvenanceId,
}

impl ProvenanceRelation {
    /// Create a relationship to another provenance record.
    #[must_use]
    pub const fn new(
        kind: ProvenanceRelationKind,
        target: ProvenanceId,
    ) -> Self {
        Self { kind, target }
    }

    /// Return the relationship kind.
    #[must_use]
    pub const fn kind(&self) -> &ProvenanceRelationKind {
        &self.kind
    }

    /// Return the referenced provenance record ID.
    #[must_use]
    pub const fn target(&self) -> ProvenanceId {
        self.target
    }
}

/// A deterministic, typed provenance record.
///
/// A record describes one subject. Relationships describe how the subject
/// relates to other records. The record does not claim that any referenced
/// evidence has been independently verified.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProvenanceRecord {
    id: ProvenanceId,
    subject: ProvenanceSubject,
    origin: ProvenanceOrigin,
    relations: Vec<ProvenanceRelation>,
    attributes: BTreeMap<String, String>,
}

impl ProvenanceRecord {
    /// Create a provenance record with an explicit ID and subject.
    ///
    /// ID allocation belongs to `model/ids.rs` and its caller. This function
    /// never invents IDs, silently substitutes IDs, or uses global state.
    #[must_use]
    pub fn new(id: ProvenanceId, subject: ProvenanceSubject) -> Self {
        Self {
            id,
            subject,
            origin: ProvenanceOrigin::Unknown,
            relations: Vec::new(),
            attributes: BTreeMap::new(),
        }
    }

    /// Return the record identity.
    #[must_use]
    pub const fn id(&self) -> ProvenanceId {
        self.id
    }

    /// Return the record subject.
    #[must_use]
    pub const fn subject(&self) -> &ProvenanceSubject {
        &self.subject
    }

    /// Return the record origin.
    #[must_use]
    pub const fn origin(&self) -> &ProvenanceOrigin {
        &self.origin
    }

    /// Return relationships in insertion order.
    #[must_use]
    pub fn relations(&self) -> &[ProvenanceRelation] {
        &self.relations
    }

    /// Return deterministic record attributes.
    #[must_use]
    pub fn attributes(&self) -> &BTreeMap<String, String> {
        &self.attributes
    }

    /// Replace the origin classification.
    #[must_use]
    pub fn with_origin(mut self, origin: ProvenanceOrigin) -> Self {
        self.origin = origin;
        self
    }

    /// Append one relationship.
    ///
    /// Referential integrity is checked by `ProvenanceGraph::validate`,
    /// because the referenced record may be inserted later.
    #[must_use]
    pub fn with_relation(mut self, relation: ProvenanceRelation) -> Self {
        self.relations.push(relation);
        self
    }

    /// Append multiple relationships in the supplied order.
    #[must_use]
    pub fn with_relations(
        mut self,
        relations: impl IntoIterator<Item = ProvenanceRelation>,
    ) -> Self {
        self.relations.extend(relations);
        self
    }

    /// Insert or replace an attribute using a validated key.
    ///
    /// Values are opaque descriptive metadata. Do not place credentials,
    /// private keys, access tokens, or other secrets in provenance.
    pub fn set_attribute(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Option<String>, ProvenanceError> {
        let key = key.into();
        let value = value.into();

        validate_qualified_name("provenance attribute key", &key)?;
        validate_text("provenance attribute value", &value)?;

        Ok(self.attributes.insert(key, value))
    }

    /// Builder-style attribute insertion.
    pub fn with_attribute(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self, ProvenanceError> {
        self.set_attribute(key, value)?;
        Ok(self)
    }

    /// Remove an attribute.
    pub fn remove_attribute(&mut self, key: &str) -> Option<String> {
        self.attributes.remove(key)
    }
}

/// Errors produced by provenance construction and graph validation.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ProvenanceError {
    /// A required text field is empty or contains only whitespace.
    EmptyField {
        /// Field name.
        field: &'static str,
    },

    /// A field contains control characters not accepted by this model.
    ControlCharacter {
        /// Field name.
        field: &'static str,
    },

    /// A qualified identifier does not meet this model's naming contract.
    InvalidQualifiedName {
        /// Field name.
        field: &'static str,
        /// Rejected value.
        value: String,
    },

    /// A record ID already exists in the graph.
    DuplicateRecord {
        /// Duplicate ID.
        id: ProvenanceId,
    },

    /// A relationship points to a record that does not exist.
    MissingReference {
        /// Record containing the invalid relationship.
        source: ProvenanceId,
        /// Referenced record that is absent.
        target: ProvenanceId,
    },

    /// A cycle exists in derivation relationships.
    DerivationCycle {
        /// One deterministic cycle path, including the repeated first ID
        /// at the end when available.
        path: Vec<ProvenanceId>,
    },
}

impl fmt::Display for ProvenanceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyField { field } => {
                write!(f, "provenance field `{field}` must not be empty")
            }
            Self::ControlCharacter { field } => {
                write!(
                    f,
                    "provenance field `{field}` contains a control character"
                )
            }
            Self::InvalidQualifiedName { field, value } => {
                write!(
                    f,
                    "invalid provenance {field} `{value}`; expected a \
                     qualified identifier"
                )
            }
            Self::DuplicateRecord { id } => {
                write!(f, "duplicate provenance record ID {id}")
            }
            Self::MissingReference { source, target } => {
                write!(
                    f,
                    "provenance record {source} references missing record {target}"
                )
            }
            Self::DerivationCycle { path } => {
                write!(f, "provenance derivation cycle: ")?;
                for (index, id) in path.iter().enumerate() {
                    if index != 0 {
                        f.write_str(" -> ")?;
                    }
                    write!(f, "{id}")?;
                }
                Ok(())
            }
        }
    }
}

impl Error for ProvenanceError {}

/// A validated collection of provenance records.
///
/// The graph owns records but not AST nodes, source text, semantic entities,
/// external artifacts, or cryptographic evidence. All record IDs are explicit
/// and graph iteration is deterministic.
///
/// Mutation is permitted during construction. Call `validate` before
/// treating the graph as referentially complete.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ProvenanceGraph {
    records: BTreeMap<ProvenanceId, ProvenanceRecord>,
}

impl ProvenanceGraph {
    /// Create an empty provenance graph.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Return the number of records currently stored.
    #[must_use]
    pub fn len(&self) -> usize {
        self.records.len()
    }

    /// Return whether the graph contains no records.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    /// Return a record by identity.
    #[must_use]
    pub fn get(&self, id: ProvenanceId) -> Option<&ProvenanceRecord> {
        self.records.get(&id)
    }

    /// Iterate over records in ascending ID order.
    pub fn iter(
        &self,
    ) -> impl ExactSizeIterator<Item = (&ProvenanceId, &ProvenanceRecord)> {
        self.records.iter()
    }

    /// Check whether a record ID exists.
    #[must_use]
    pub fn contains(&self, id: ProvenanceId) -> bool {
        self.records.contains_key(&id)
    }

    /// Insert a record without silently replacing an existing record.
    ///
    /// This operation checks identity uniqueness. References may point to
    /// records that will be inserted later; call `validate` after construction
    /// to check the complete graph.
    pub fn insert(
        &mut self,
        record: ProvenanceRecord,
    ) -> Result<(), ProvenanceError> {
        let id = record.id();

        if self.records.contains_key(&id) {
            return Err(ProvenanceError::DuplicateRecord { id });
        }

        self.records.insert(id, record);
        Ok(())
    }

    /// Remove a record by ID.
    ///
    /// Removing a record may leave dangling relationships. The graph must be
    /// validated again before it is used as a complete provenance graph.
    pub fn remove(&mut self, id: ProvenanceId) -> Option<ProvenanceRecord> {
        self.records.remove(&id)
    }

    /// Validate reference integrity and derivation acyclicity.
    ///
    /// This function is deterministic and iterative. It does not recursively
    /// traverse the graph and therefore does not use call-stack depth as a
    /// hidden limit on lineage length.
    pub fn validate(&self) -> Result<(), ProvenanceError> {
        self.validate_references()?;
        self.validate_derivation_acyclic()
    }

    /// Validate that every relationship target exists.
    pub fn validate_references(&self) -> Result<(), ProvenanceError> {
        for (source, record) in &self.records {
            for relation in record.relations() {
                let target = relation.target();

                if !self.records.contains_key(&target) {
                    return Err(ProvenanceError::MissingReference {
                        source: *source,
                        target,
                    });
                }
            }
        }

        Ok(())
    }

    /// Validate that `WasDerivedFrom` relationships are acyclic.
    ///
    /// Other relationship kinds may form cycles where the provenance
    /// specification permits them. Only derivation edges are constrained here.
    ///
    /// The algorithm uses Kahn's topological-sort method. It uses O(V + E)
    /// time and O(V) additional memory for the derivation subgraph, where V
    /// is the number of records and E is the number of derivation edges.
    pub fn validate_derivation_acyclic(&self) -> Result<(), ProvenanceError> {
        self.validate_references()?;

        let mut indegree: BTreeMap<ProvenanceId, usize> = self
            .records
            .keys()
            .copied()
            .map(|id| (id, 0_usize))
            .collect();

        let mut outgoing: BTreeMap<ProvenanceId, Vec<ProvenanceId>> =
            BTreeMap::new();

        for (source, record) in &self.records {
            for relation in record.relations() {
                if relation.kind().is_derivation() {
                    let target = relation.target();

                    let degree = indegree
                        .get_mut(&target)
                        .expect("references were validated");

                    *degree = degree.checked_add(1).ok_or_else(|| {
                        ProvenanceError::DerivationCycle {
                            path: vec![target, target],
                        }
                    })?;

                    outgoing.entry(*source).or_default().push(target);
                }
            }
        }

        for targets in outgoing.values_mut() {
            targets.sort_unstable();
        }

        let mut ready: BTreeSet<ProvenanceId> = indegree
            .iter()
            .filter_map(|(id, degree)| (*degree == 0).then_some(*id))
            .collect();

        let mut visited = 0_usize;

        while let Some(id) = ready.pop_first() {
            visited = visited.saturating_add(1);

            if let Some(targets) = outgoing.get(&id) {
                for target in targets {
                    let degree = indegree
                        .get_mut(target)
                        .expect("references were validated");

                    *degree -= 1;

                    if *degree == 0 {
                        ready.insert(*target);
                    }
                }
            }
        }

        if visited == self.records.len() {
            return Ok(());
        }

        Err(ProvenanceError::DerivationCycle {
            path: self.find_derivation_cycle(),
        })
    }

    /// Return a stable snapshot of all records.
    ///
    /// The returned vector is sorted by provenance ID. Its memory use is
    /// proportional to the number of records.
    #[must_use]
    pub fn to_sorted_vec(&self) -> Vec<ProvenanceRecord> {
        self.records.values().cloned().collect()
    }

    /// Find a deterministic derivation cycle after topological validation
    /// has established that one exists.
    ///
    /// This search is iterative to remain safe for very deep graphs.
    fn find_derivation_cycle(&self) -> Vec<ProvenanceId> {
        #[derive(Clone, Copy, PartialEq, Eq)]
        enum VisitState {
            Active,
            Complete,
        }

        let mut state: BTreeMap<ProvenanceId, VisitState> = BTreeMap::new();
        let mut parent: BTreeMap<ProvenanceId, ProvenanceId> = BTreeMap::new();

        for start in self.records.keys().copied() {
            if state.contains_key(&start) {
                continue;
            }

            let mut stack: Vec<(ProvenanceId, Vec<ProvenanceId>, usize)> =
                Vec::new();

            state.insert(start, VisitState::Active);
            stack.push((start, self.derivation_targets(start), 0));

            while let Some((current, targets, next_index)) = stack.last_mut() {
                if *next_index >= targets.len() {
                    let finished = *current;
                    state.insert(finished, VisitState::Complete);
                    stack.pop();
                    continue;
                }

                let target = targets[*next_index];
                *next_index += 1;

                match state.get(&target).copied() {
                    Some(VisitState::Active) => {
                        let mut path = vec![target];
                        let mut cursor = *current;
                        path.push(cursor);

                        while cursor != target {
                            let Some(previous) = parent.get(&cursor).copied()
                            else {
                                break;
                            };

                            cursor = previous;
                            path.push(cursor);
                        }

                        path.reverse();
                        path.push(target);
                        return path;
                    }
                    Some(VisitState::Complete) => {}
                    None => {
                        parent.insert(target, *current);
                        state.insert(target, VisitState::Active);
                        stack.push((target, self.derivation_targets(target), 0));
                    }
                }
            }
        }

        // Defensive fallback: the topological check found a cycle, so this
        // branch should not be reachable for an unchanged graph.
        Vec::new()
    }

    fn derivation_targets(&self, id: ProvenanceId) -> Vec<ProvenanceId> {
        let mut targets: Vec<_> = self
            .records
            .get(&id)
            .into_iter()
            .flat_map(|record| record.relations())
            .filter(|relation| relation.kind().is_derivation())
            .map(ProvenanceRelation::target)
            .collect();

        targets.sort_unstable();
        targets
    }
}

/// Validate a metadata string without interpreting its domain-specific
/// contents.
fn validate_text(
    field: &'static str,
    value: &str,
) -> Result<(), ProvenanceError> {
    if value.trim().is_empty() {
        return Err(ProvenanceError::EmptyField { field });
    }

    if value.chars().any(char::is_control) {
        return Err(ProvenanceError::ControlCharacter { field });
    }

    Ok(())
}

/// Validate a qualified metadata name.
///
/// Accepted names contain one or more dot-separated segments. Each segment
/// starts with an ASCII letter or underscore; subsequent characters may also
/// contain ASCII digits and hyphens. This contract applies to provenance
/// metadata keys and extension relationship names, not to Zamani source
/// identifiers.
fn validate_qualified_name(
    field: &'static str,
    value: &str,
) -> Result<(), ProvenanceError> {
    validate_text(field, value)?;

    let valid = value.split('.').all(|segment| {
        let mut chars = segment.chars();

        match chars.next() {
            Some(first) if first.is_ascii_alphabetic() || first == '_' => {
                chars.all(|ch| {
                    ch.is_ascii_alphanumeric() || ch == '_' || ch == '-'
                })
            }
            _ => false,
        }
    });

    if valid {
        Ok(())
    } else {
        Err(ProvenanceError::InvalidQualifiedName {
            field,
            value: value.to_owned(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn provenance_id(raw: u64) -> ProvenanceId {
        ProvenanceId::new(raw).expect("test provenance ID must be non-zero")
    }

    fn semantic_id(raw: u64) -> SemanticId {
        SemanticId::new(raw).expect("test semantic ID must be non-zero")
    }

    fn record(raw: u64) -> ProvenanceRecord {
        ProvenanceRecord::new(
            provenance_id(raw),
            ProvenanceSubject::semantic_entity(semantic_id(raw)),
        )
    }

    fn derived_from(source: u64, target: u64) -> ProvenanceRecord {
        record(source).with_relation(ProvenanceRelation::new(
            ProvenanceRelationKind::WasDerivedFrom,
            provenance_id(target),
        ))
    }

    #[test]
    fn record_identity_and_subject_are_typed() {
        let id = provenance_id(1);
        let subject = ProvenanceSubject::semantic_entity(semantic_id(7));
        let value = ProvenanceRecord::new(id, subject.clone());

        assert_eq!(value.id(), id);
        assert_eq!(value.subject(), &subject);
        assert!(value.relations().is_empty());
        assert!(value.attributes().is_empty());
    }

    #[test]
    fn artifact_references_reject_empty_or_control_text() {
        assert!(ArtifactReference::new("", "artifact").is_err());
        assert!(ArtifactReference::new("registry", " \t ").is_err());
        assert!(ArtifactReference::new("registry", "artifact\nname").is_err());

        let artifact =
            ArtifactReference::new("registry", "artifact-17").expect("valid");

        assert_eq!(artifact.namespace(), "registry");
        assert_eq!(artifact.identifier(), "artifact-17");
    }

    #[test]
    fn extension_relationships_are_validated_and_extensible() {
        assert!(ProvenanceRelationKind::extension("").is_err());
        assert!(ProvenanceRelationKind::extension("not valid").is_err());

        let relation =
            ProvenanceRelationKind::extension("ml.model-trained-by")
                .expect("valid extension name");

        assert_eq!(
            relation,
            ProvenanceRelationKind::Extension(
                "ml.model-trained-by".to_owned()
            )
        );
    }

    #[test]
    fn attributes_are_validated_and_deterministically_ordered() {
        let mut value = record(1);

        value
            .set_attribute("compiler.pass", "type-check")
            .expect("valid attribute");
        value
            .set_attribute("source.version", "1")
            .expect("valid attribute");

        assert!(value.set_attribute("", "x").is_err());
        assert!(value.set_attribute("bad key", "x").is_err());
        assert!(value.set_attribute("secret", "value\n").is_err());

        let keys: Vec<_> = value.attributes().keys().map(String::as_str).collect();

        assert_eq!(keys, vec!["compiler.pass", "source.version"]);
    }

    #[test]
    fn graph_insertion_rejects_duplicate_ids() {
        let mut graph = ProvenanceGraph::new();

        graph.insert(record(1)).expect("first insertion");

        assert_eq!(
            graph.insert(record(1)),
            Err(ProvenanceError::DuplicateRecord {
                id: provenance_id(1),
            })
        );

        assert_eq!(graph.len(), 1);
    }

    #[test]
    fn graph_iteration_is_ordered_by_record_id() {
        let mut graph = ProvenanceGraph::new();

        graph.insert(record(30)).expect("insert");
        graph.insert(record(2)).expect("insert");
        graph.insert(record(11)).expect("insert");

        let ids: Vec<_> = graph.iter().map(|(id, _)| id.get()).collect();

        assert_eq!(ids, vec![2, 11, 30]);
    }

    #[test]
    fn graph_rejects_missing_relationship_targets() {
        let mut graph = ProvenanceGraph::new();

        graph
            .insert(derived_from(1, 2))
            .expect("insert record");

        assert_eq!(
            graph.validate(),
            Err(ProvenanceError::MissingReference {
                source: provenance_id(1),
                target: provenance_id(2),
            })
        );
    }

    #[test]
    fn acyclic_derivation_graph_is_valid() {
        let mut graph = ProvenanceGraph::new();

        graph.insert(record(1)).expect("insert");
        graph.insert(derived_from(2, 1)).expect("insert");
        graph.insert(derived_from(3, 2)).expect("insert");

        assert!(graph.validate().is_ok());
    }

    #[test]
    fn derivation_cycles_are_rejected() {
        let mut graph = ProvenanceGraph::new();

        graph.insert(derived_from(1, 2)).expect("insert");
        graph.insert(derived_from(2, 1)).expect("insert");

        assert!(matches!(
            graph.validate(),
            Err(ProvenanceError::DerivationCycle { .. })
        ));
    }

    #[test]
    fn non_derivation_relationships_may_form_cycles() {
        let mut graph = ProvenanceGraph::new();

        graph
            .insert(record(1).with_relation(ProvenanceRelation::new(
                ProvenanceRelationKind::WasInfluencedBy,
                provenance_id(2),
            )))
            .expect("insert");

        graph
            .insert(record(2).with_relation(ProvenanceRelation::new(
                ProvenanceRelationKind::WasInfluencedBy,
                provenance_id(1),
            )))
            .expect("insert");

        assert!(graph.validate().is_ok());
    }

    #[test]
    fn generated_and_external_origins_validate_metadata() {
        let generated =
            GeneratedOrigin::new("compiler.semantic-analysis", Some("1".into()))
                .expect("valid origin");

        assert_eq!(generated.activity(), "compiler.semantic-analysis");
        assert_eq!(generated.activity_version(), Some("1"));

        assert!(GeneratedOrigin::new(" ", None).is_err());

        let external =
            ExternalOrigin::new("artifact.registry", "artifact-123")
                .expect("valid origin");

        assert_eq!(external.authority(), "artifact.registry");
        assert_eq!(external.reference(), "artifact-123");
        assert!(ExternalOrigin::new("registry", "\n").is_err());
    }

    #[test]
    fn source_origin_uses_canonical_ast_identity_and_span() {
        // Source spans and AST IDs are created by the canonical AST layer.
        // The test intentionally does not duplicate their representation.
        let node_id = NodeId::first();
        let span = Span::empty(
            crate::frontend::ast::node::source::SourceId::from_raw(1),
            crate::frontend::ast::node::source::SourceOffset::from_raw(0),
        )
        .expect("valid empty span");

        let origin = SourceOrigin::new(node_id, span);

        assert_eq!(origin.node_id(), node_id);
        assert_eq!(origin.span(), span);
    }

    #[test]
    fn removal_requires_revalidation() {
        let mut graph = ProvenanceGraph::new();

        graph.insert(record(1)).expect("insert");
        graph
            .insert(derived_from(2, 1))
            .expect("insert");

        assert!(graph.validate().is_ok());

        graph.remove(provenance_id(1)).expect("remove");

        assert!(matches!(
            graph.validate(),
            Err(ProvenanceError::MissingReference { .. })
        ));
    }

    #[test]
    fn empty_graph_is_valid() {
        assert!(ProvenanceGraph::new().validate().is_ok());
    }
}
