
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Frontend — Semantic Capability Requirements
//!
//! Path: `src/frontend/semantic/capabilities/requirements.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Convert validated, source-level AST capabilities into the canonical
//! semantic capability requirement model and retain their source provenance.
//!
//! This module owns requirement collection and AST-to-semantic conversion.
//! It does not own capability identity definitions, target evidence,
//! authorization, resource allocation, target selection, or IR lowering.
//!
//! ## AUTHORITY
//!
//! - `grammar/Zamani.g4`
//! - `grammar/antlr/ZamaniParser.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/grammar-authority.md`
//! - `grammar/specification/poco-reaf.md`
//! - `grammar/spec/quantum.md`
//! - `grammar/spec/hdl.md`
//! - `src/frontend/ast/node/capabilities/capability.rs`
//! - `src/frontend/semantic/capabilities/model.rs`
//!
//! The grammar and specifications govern meaning. The AST owns source
//! structure. `capabilities/model.rs` owns semantic capability values.
//!
//! ## OWNS
//!
//! - Conversion from validated AST capabilities to semantic requirements.
//! - Association of each requirement with its source AST node and span.
//! - Deterministic storage and iteration of requirements.
//! - Duplicate-source-node detection.
//! - Transactional batch insertion.
//!
//! ## DOES_NOT_OWN
//!
//! - Parsing capability syntax or defining another AST.
//! - Name/import resolution or capability-definition registration.
//! - A closed enumeration of capability names or computational domains.
//! - Target capabilities, evidence verification, or authorization.
//! - Hardware discovery, feasibility, placement, routing, or scheduling.
//! - Resource quantities or resource-constraint solving.
//! - Classical IR, `quantum::ir`, HDL IR, or runtime execution.
//!
//! ## INTEGRATION CONTRACT
//!
//! Input:
//!   `frontend::ast::node::capabilities::capability::Capability`
//!
//! Canonical output:
//!   `frontend::semantic::capabilities::model::CapabilityRequirement`
//!
//! Provenance:
//!   AST `NodeId` and canonical source `Span`.
//!
//! Downstream consumers:
//!   capability evidence/compatibility, semantic facts, domain validators,
//!   and semantic-model finalization.
//!
//! AST namespace/name strings are converted without case folding, alias
//! expansion, or implicit renaming. Name resolution and extension-definition
//! lookup remain separate semantic responsibilities.
//!
//! ## SCALABILITY
//!
//! There are no language-level limits on the number of requirements,
//! capabilities, namespaces, domains, or described hardware resources.
//! Collection size grows with the input and resources actually available.
//!
//! Rust collections and AST identities have finite representational limits.
//! Operational budgets belong to the semantic session/configuration layer.
//! Exhausting a budget must result in incomplete analysis, not a false
//! declaration of semantic validity.
//!
//! This module does not attempt to process literally infinite input.
//!
//! ## DETERMINISM
//!
//! Requirements are indexed by AST `NodeId` in a `BTreeMap`. Iteration is
//! therefore deterministic for a given AST identity assignment.
//!
//! Independent source nodes remain distinct even when they require the same
//! capability. A repeated insertion for the same node is idempotent only
//! when its requirement and source span are identical.
//!
//! ## ERROR HANDLING
//!
//! Invalid AST capabilities, invalid semantic identities/version constraints,
//! and conflicting reuse of an AST node identity return structured errors.
//! No ordinary malformed input is handled by panicking.
//!
//! ## FREEZE CRITERIA
//!
//! - `capabilities/mod.rs` declares and exports this module.
//! - AST conversion and version-boundary tests pass.
//! - Duplicate and transactional insertion tests pass.
//! - Semantic facts preserve the originating AST node.
//! - The semantic pipeline consumes this collection without consulting a
//!   physical target.
//! - The relevant workspace builds and tests pass on the supported toolchain.

use core::fmt;
use std::collections::BTreeMap;
use std::error::Error;

use crate::frontend::ast::node::capabilities::capability::{
    Capability as AstCapability,
    CapabilityError as AstCapabilityError,
    CapabilityVersion as AstCapabilityVersion,
    CapabilityVersionConstraint as AstVersionConstraint,
};
use crate::frontend::ast::node::node_id::NodeId;
use crate::frontend::ast::node::source::span::Span;

use crate::frontend::semantic::capabilities::model::{
    CapabilityId as SemanticCapabilityName,
    CapabilityModelError,
    CapabilityRequirement,
    CapabilityVersion as SemanticCapabilityVersion,
    CapabilityVersionConstraint as SemanticVersionConstraint,
};

/// Stable architectural identity for this module.
pub const CAPABILITY_REQUIREMENTS_MODULE_NAME: &str =
    "zamani.frontend.semantic.capabilities.requirements";

/// Version of this module's in-memory collection contract.
///
/// This is independent of the Zamani language version, AST schema, capability
/// definition version, and serialized artifact schema.
pub const CAPABILITY_REQUIREMENTS_SCHEMA_VERSION: u16 = 1;

/// A semantic capability requirement associated with its source origin.
///
/// The requirement expresses program intent. Its presence does not establish
/// that a capability is registered, authorized, supported by a target, or
/// feasible to realize.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CapabilityRequirementRecord {
    source_node: NodeId,
    source_span: Span,
    requirement: CapabilityRequirement,
}

impl CapabilityRequirementRecord {
    /// Construct a record from a source identity and a semantic requirement.
    ///
    /// Callers supplying an already-resolved requirement are responsible for
    /// ensuring that the span actually belongs to `source_node`. This
    /// association can only be verified against the complete AST graph.
    #[must_use]
    pub const fn new(
        source_node: NodeId,
        source_span: Span,
        requirement: CapabilityRequirement,
    ) -> Self {
        Self {
            source_node,
            source_span,
            requirement,
        }
    }

    /// Return the source AST node identity.
    #[must_use]
    pub const fn source_node(&self) -> NodeId {
        self.source_node
    }

    /// Return the source span.
    #[must_use]
    pub const fn source_span(&self) -> &Span {
        &self.source_span
    }

    /// Return the canonical semantic requirement.
    #[must_use]
    pub const fn requirement(&self) -> &CapabilityRequirement {
        &self.requirement
    }

    /// Consume the record and return its parts.
    #[must_use]
    pub fn into_parts(
        self,
    ) -> (NodeId, Span, CapabilityRequirement) {
        (self.source_node, self.source_span, self.requirement)
    }
}

/// Deterministic collection of semantic capability requirements.
///
/// The source node is the collection key because two different constructs
/// requiring the same capability are distinct source obligations. This avoids
/// accidentally discarding source provenance or treating one declaration as
/// proof that every other declaration has been checked.
///
/// This collection deliberately does not merge requirements from different
/// nodes. Capability compatibility and logical constraint combination belong
/// to their respective downstream analyses.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CapabilityRequirements {
    records: BTreeMap<NodeId, CapabilityRequirementRecord>,
}

impl CapabilityRequirements {
    /// Create an empty requirement collection.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Build a collection from validated AST capability nodes.
    ///
    /// The batch is all-or-nothing: if any item is invalid or conflicts with
    /// another item in the batch, no partial collection is returned.
    ///
    /// # Errors
    ///
    /// Returns the first deterministic conversion or duplicate-node error
    /// encountered in the supplied iterator's order.
    pub fn from_ast<'a>(
        capabilities: impl IntoIterator<Item = &'a AstCapability>,
    ) -> Result<Self, CapabilityRequirementError> {
        let mut result = Self::new();
        result.extend_ast(capabilities)?;
        Ok(result)
    }

    /// Return the number of distinct source-node records.
    #[must_use]
    pub fn len(&self) -> usize {
        self.records.len()
    }

    /// Return whether the collection is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    /// Look up a requirement by its originating AST node.
    #[must_use]
    pub fn get(
        &self,
        source_node: NodeId,
    ) -> Option<&CapabilityRequirementRecord> {
        self.records.get(&source_node)
    }

    /// Iterate over records in ascending AST-node identity order.
    pub fn iter(
        &self,
    ) -> impl ExactSizeIterator<Item = &CapabilityRequirementRecord> {
        self.records.values()
    }

    /// Iterate over requirements matching a semantic capability identity.
    ///
    /// This is an allocation-free query. Results retain deterministic
    /// source-node ordering.
    pub fn for_capability<'a>(
        &'a self,
        capability: &'a SemanticCapabilityName,
    ) -> impl Iterator<Item = &'a CapabilityRequirementRecord> + 'a {
        self.records
            .values()
            .filter(move |record| {
                record.requirement().capability() == capability
            })
    }

    /// Convert and insert one AST capability.
    ///
    /// Re-inserting the same node with the same span and requirement is
    /// idempotent. Reusing a node identity for different data is rejected.
    ///
    /// # Errors
    ///
    /// Returns an error if AST structural validation fails, conversion fails,
    /// or an existing source node has conflicting requirement data.
    pub fn insert_ast(
        &mut self,
        capability: &AstCapability,
    ) -> Result<(), CapabilityRequirementError> {
        let record = Self::record_from_ast(capability)?;
        self.insert_record(record)
    }

    /// Convert and insert a batch of AST capabilities atomically.
    ///
    /// The original collection remains unchanged if any item fails.
    ///
    /// This uses copy-on-write at the collection level by staging into a
    /// separate map. It does not clone the AST or mutate source nodes.
    pub fn extend_ast<'a>(
        &mut self,
        capabilities: impl IntoIterator<Item = &'a AstCapability>,
    ) -> Result<(), CapabilityRequirementError> {
        let mut staged = self.records.clone();

        for capability in capabilities {
            let record = Self::record_from_ast(capability)?;
            Self::insert_into(&mut staged, record)?;
        }

        self.records = staged;
        Ok(())
    }

    /// Insert a record produced by another semantic pass.
    ///
    /// Use this only when the supplied capability identity and requirement
    /// have already passed the owning resolver/model's validation rules.
    /// This method does not claim that the capability exists or that a target
    /// can satisfy it.
    pub fn insert_record(
        &mut self,
        record: CapabilityRequirementRecord,
    ) -> Result<(), CapabilityRequirementError> {
        Self::insert_into(&mut self.records, record)
    }

    /// Consume the collection into deterministic record order.
    #[must_use]
    pub fn into_records(
        self,
    ) -> Vec<CapabilityRequirementRecord> {
        self.records.into_values().collect()
    }

    /// Return the stable architectural module name.
    #[must_use]
    pub const fn module_name() -> &'static str {
        CAPABILITY_REQUIREMENTS_MODULE_NAME
    }

    /// Return the schema version of the in-memory contract.
    #[must_use]
    pub const fn schema_version() -> u16 {
        CAPABILITY_REQUIREMENTS_SCHEMA_VERSION
    }

    fn record_from_ast(
        capability: &AstCapability,
    ) -> Result<CapabilityRequirementRecord, CapabilityRequirementError> {
        capability
            .validate_structure()
            .map_err(CapabilityRequirementError::InvalidAstCapability)?;

        let identity = SemanticCapabilityName::new(
            capability.namespace(),
            capability.name(),
        )
        .map_err(CapabilityRequirementError::InvalidSemanticCapability)?;

        let version = convert_version_constraint(
            capability.version_constraint(),
        )?;

        let requirement = CapabilityRequirement::with_version(
            identity,
            version,
        );

        Ok(CapabilityRequirementRecord::new(
            capability.node().id(),
            capability.node().span().clone(),
            requirement,
        ))
    }

    fn insert_into(
        records: &mut BTreeMap<NodeId, CapabilityRequirementRecord>,
        record: CapabilityRequirementRecord,
    ) -> Result<(), CapabilityRequirementError> {
        let source_node = record.source_node();

        if let Some(existing) = records.get(&source_node) {
            if existing == &record {
                return Ok(());
            }

            return Err(CapabilityRequirementError::ConflictingSourceNode {
                source_node,
            });
        }

        records.insert(source_node, record);
        Ok(())
    }
}

fn convert_version(
    version: AstCapabilityVersion,
) -> SemanticCapabilityVersion {
    SemanticCapabilityVersion::new(
        version.major(),
        version.minor(),
        version.patch(),
    )
}

fn convert_version_constraint(
    constraint: AstVersionConstraint,
) -> Result<SemanticVersionConstraint, CapabilityRequirementError> {
    match constraint {
        AstVersionConstraint::Any => Ok(SemanticVersionConstraint::Any),

        AstVersionConstraint::Exact(version) => {
            Ok(SemanticVersionConstraint::Exact(convert_version(version)))
        }

        AstVersionConstraint::AtLeast(version) => {
            Ok(SemanticVersionConstraint::AtLeast(convert_version(version)))
        }

        AstVersionConstraint::AtMost(version) => {
            Ok(SemanticVersionConstraint::AtMost(convert_version(version)))
        }

        AstVersionConstraint::Between { minimum, maximum } => {
            SemanticVersionConstraint::between(
                convert_version(minimum),
                convert_version(maximum),
            )
            .map_err(CapabilityRequirementError::InvalidSemanticCapability)
        }
    }
}

/// Errors raised while constructing the capability-requirement collection.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum CapabilityRequirementError {
    /// The AST capability failed its structural invariants.
    InvalidAstCapability(AstCapabilityError),

    /// The semantic capability identity or version constraint was invalid.
    InvalidSemanticCapability(CapabilityModelError),

    /// One AST node identity was reused for conflicting requirement records.
    ConflictingSourceNode {
        /// The conflicting AST node identity.
        source_node: NodeId,
    },
}

impl fmt::Display for CapabilityRequirementError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidAstCapability(error) => {
                write!(f, "invalid AST capability requirement: {error}")
            }
            Self::InvalidSemanticCapability(error) => {
                write!(f, "invalid semantic capability requirement: {error}")
            }
            Self::ConflictingSourceNode { source_node } => {
                write!(
                    f,
                    "AST node {source_node} was associated with conflicting \
                     capability requirement data"
                )
            }
        }
    }
}

impl Error for CapabilityRequirementError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidAstCapability(error) => Some(error),
            Self::InvalidSemanticCapability(error) => Some(error),
            Self::ConflictingSourceNode { .. } => None,
        }
    }
}

impl From<AstCapabilityError> for CapabilityRequirementError {
    fn from(error: AstCapabilityError) -> Self {
        Self::InvalidAstCapability(error)
    }
}

impl From<CapabilityModelError> for CapabilityRequirementError {
    fn from(error: CapabilityModelError) -> Self {
        Self::InvalidSemanticCapability(error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::frontend::ast::node::capabilities::capability::{
        CapabilityVersion,
        CapabilityVersionConstraint,
    };
    use crate::frontend::ast::node::metadata::NodeMetadata;
    use crate::frontend::ast::node::source::span::SourceId;

    fn ast_capability(
        node_id: u64,
        version: AstVersionConstraint,
    ) -> AstCapability {
        let id = NodeId::new(node_id).expect("non-zero test node ID");
        let span = Span::from_usize(
            SourceId::from_raw(1),
            node_id as usize,
            (node_id as usize) + 1,
        )
        .expect("valid test span");

        AstCapability::with_version(
            id,
            span,
            NodeMetadata::default(),
            "zamani.quantum",
            "dynamic_control",
            version,
        )
        .expect("valid AST capability")
    }

    fn semantic_identity() -> SemanticCapabilityName {
        SemanticCapabilityName::new(
            "zamani.quantum",
            "dynamic_control",
        )
        .expect("valid semantic identity")
    }

    #[test]
    fn converts_ast_identity_and_version_without_renaming() {
        let capability = ast_capability(
            1,
            AstVersionConstraint::Exact(CapabilityVersion::new(2, 3, 4)),
        );

        let requirements =
            CapabilityRequirements::from_ast([&capability])
                .expect("valid requirements");

        let record = requirements
            .get(NodeId::new(1).expect("valid node ID"))
            .expect("requirement record");

        assert_eq!(record.requirement().capability(), &semantic_identity());
        assert_eq!(
            record.requirement().version_constraint(),
            SemanticVersionConstraint::Exact(
                SemanticCapabilityVersion::new(2, 3, 4)
            )
        );
        assert_eq!(record.source_span(), capability.node().span());
    }

    #[test]
    fn converts_all_version_constraint_forms() {
        let v1 = CapabilityVersion::new(1, 0, 0);
        let v2 = CapabilityVersion::new(2, 0, 0);
        let v3 = CapabilityVersion::new(3, 0, 0);

        let cases = [
            (
                AstVersionConstraint::Any,
                SemanticVersionConstraint::Any,
            ),
            (
                AstVersionConstraint::Exact(v2),
                SemanticVersionConstraint::Exact(
                    SemanticCapabilityVersion::new(2, 0, 0)
                ),
            ),
            (
                AstVersionConstraint::AtLeast(v2),
                SemanticVersionConstraint::AtLeast(
                    SemanticCapabilityVersion::new(2, 0, 0)
                ),
            ),
            (
                AstVersionConstraint::AtMost(v2),
                SemanticVersionConstraint::AtMost(
                    SemanticCapabilityVersion::new(2, 0, 0)
                ),
            ),
            (
                AstVersionConstraint::Between {
                    minimum: v1,
                    maximum: v3,
                },
                SemanticVersionConstraint::Between {
                    minimum: SemanticCapabilityVersion::new(1, 0, 0),
                    maximum: SemanticCapabilityVersion::new(3, 0, 0),
                },
            ),
        ];

        for (index, (source, expected)) in cases.into_iter().enumerate() {
            let node_id = u64::try_from(index + 1).expect("small test index");
            let capability = ast_capability(node_id, source);

            let requirements =
                CapabilityRequirements::from_ast([&capability])
                    .expect("valid version constraint");

            let record = requirements
                .get(NodeId::new(node_id).expect("valid node ID"))
                .expect("record");

            assert_eq!(record.requirement().version_constraint(), expected);
        }
    }

    #[test]
    fn repeated_identical_insertion_is_idempotent() {
        let capability = ast_capability(1, AstVersionConstraint::Any);
        let mut requirements = CapabilityRequirements::new();

        requirements.insert_ast(&capability).expect("first insertion");
        requirements.insert_ast(&capability).expect("identical insertion");

        assert_eq!(requirements.len(), 1);
    }

    #[test]
    fn different_nodes_remain_distinct_even_for_the_same_capability() {
        let first = ast_capability(1, AstVersionConstraint::Any);
        let second = ast_capability(2, AstVersionConstraint::Any);

        let requirements =
            CapabilityRequirements::from_ast([&first, &second])
                .expect("valid requirements");

        assert_eq!(requirements.len(), 2);
        assert_eq!(
            requirements.for_capability(&semantic_identity()).count(),
            2
        );
    }

    #[test]
    fn iteration_is_ordered_by_source_node_identity() {
        let first = ast_capability(9, AstVersionConstraint::Any);
        let second = ast_capability(2, AstVersionConstraint::Any);
        let third = ast_capability(5, AstVersionConstraint::Any);

        let requirements =
            CapabilityRequirements::from_ast([&first, &second, &third])
                .expect("valid requirements");

        let ids: Vec<u64> = requirements
            .iter()
            .map(|record| record.source_node().get())
            .collect();

        assert_eq!(ids, vec![2, 5, 9]);
    }

    #[test]
    fn failed_batch_does_not_partially_mutate_existing_collection() {
        let existing = ast_capability(1, AstVersionConstraint::Any);
        let new_record = ast_capability(2, AstVersionConstraint::Any);
        let conflicting = ast_capability(
            1,
            AstVersionConstraint::Exact(CapabilityVersion::new(2, 0, 0)),
        );

        let mut requirements = CapabilityRequirements::new();
        requirements
            .insert_ast(&existing)
            .expect("initial requirement");

        let before = requirements.clone();

        assert!(matches!(
            requirements.extend_ast([&new_record, &conflicting]),
            Err(CapabilityRequirementError::ConflictingSourceNode {
                source_node
            }) if source_node.get() == 1
        ));

        assert_eq!(requirements, before);
    }

    #[test]
    fn capability_query_returns_only_matching_identity() {
        let first = ast_capability(1, AstVersionConstraint::Any);

        let other = AstCapability::without_metadata(
            NodeId::new(2).expect("valid node ID"),
            Span::from_usize(
                SourceId::from_raw(1),
                2,
                3,
            )
            .expect("valid span"),
            "zamani.classical",
            "parallel_execution",
        )
        .expect("valid AST capability");

        let requirements =
            CapabilityRequirements::from_ast([&first, &other])
                .expect("valid requirements");

        assert_eq!(
            requirements.for_capability(&semantic_identity()).count(),
            1
        );
    }

    #[test]
    fn module_contract_is_stable_and_versioned() {
        assert_eq!(
            CapabilityRequirements::module_name(),
            CAPABILITY_REQUIREMENTS_MODULE_NAME
        );
        assert_eq!(
            CapabilityRequirements::schema_version(),
            1
        );
    }
}
