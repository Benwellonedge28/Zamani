
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Compiler Program Analysis
//!
//! Path: src/compiler/analysis/program.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! PURPOSE
//!
//! Provides the compiler's program-level analysis index.
//!
//! This module aggregates analysis observations concerning a program,
//! its compilation units, dependencies, participating domains and
//! references to externally owned semantic facts.
//!
//! It does not define program semantics or another intermediate
//! representation.
//!
//! ARCHITECTURAL AUTHORITY
//!
//! - grammar/specification/semantic-model.md
//! - grammar/spec/resources.md
//! - grammar/specification/scalability-model.md
//! - src/semantic.rs
//! - src/zuir/ir/program.rs
//! - src/zuir/ir/module.rs
//! - src/zuir/ids.rs
//! - src/zuir/version.rs
//!
//! OWNERSHIP
//!
//! This module owns:
//! - Program analysis observations.
//! - Compilation-unit analysis indexing.
//! - Dependency observations.
//! - Domain participation observations.
//! - Analysis completeness.
//! - References to external semantic facts.
//! - Deterministic dependency ordering.
//! - Local consistency validation.
//!
//! It does not own:
//! - Source syntax or AST.
//! - Semantic type definitions.
//! - Effects or resource semantics.
//! - Canonical ZUIR entities.
//! - Dialect registration.
//! - Hardware capabilities.
//! - Physical resource allocation.
//! - Backend selection.
//! - Runtime scheduling.
//!
//! SCALABILITY
//!
//! No artificial limit is imposed on:
//! - Compilation units.
//! - Dependency edges.
//! - Participating domains.
//! - Semantic references.
//! - Analysis observations.
//!
//! Actual capacity remains bounded by the host's address space,
//! allocator and available resources.
//!
//! Operational budgets belong to the compilation context.
//!
//! INTEGRATION
//!
//! The semantic analyzer supplies validated semantic references.
//! ZUIR supplies canonical IR identities.
//! Domain analyzers supply domain participation observations.
//! The dependency analyzer supplies compilation dependencies.
//! The feasibility analyzer consumes requirements and capabilities.
//! The pipeline consumes this module's analysis results.
//!
//! References are opaque. This module never interprets another
//! subsystem's identifiers or silently changes their meaning.

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fmt;

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by program analysis construction and validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProgramAnalysisError {
    EmptyIdentifier {
        field: &'static str,
    },

    DuplicateCompilationUnit {
        id: CompilationUnitId,
    },

    UnknownCompilationUnit {
        id: CompilationUnitId,
    },

    DuplicateDependency {
        dependency: DependencyEdge,
    },

    SelfDependency {
        id: CompilationUnitId,
    },

    DuplicateDomainObservation {
        domain: DomainId,
        entity: EntityReference,
    },

    InvalidReference {
        field: &'static str,
    },

    InvalidObservation {
        reason: String,
    },

    DependencyCycle {
        remaining: Vec<CompilationUnitId>,
    },

    InconsistentProgram {
        reason: String,
    },
}

impl fmt::Display for ProgramAnalysisError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyIdentifier { field } => {
                write!(f, "{field} must not be empty")
            }

            Self::DuplicateCompilationUnit { id } => {
                write!(f, "duplicate compilation unit: {id}")
            }

            Self::UnknownCompilationUnit { id } => {
                write!(f, "unknown compilation unit: {id}")
            }

            Self::DuplicateDependency { dependency } => {
                write!(f, "duplicate dependency: {dependency:?}")
            }

            Self::SelfDependency { id } => {
                write!(f, "compilation unit depends on itself: {id}")
            }

            Self::DuplicateDomainObservation { domain, entity } => {
                write!(
                    f,
                    "duplicate domain observation: {domain}, {entity}"
                )
            }

            Self::InvalidReference { field } => {
                write!(f, "invalid external reference: {field}")
            }

            Self::InvalidObservation { reason } => {
                write!(f, "invalid analysis observation: {reason}")
            }

            Self::DependencyCycle { remaining } => {
                write!(
                    f,
                    "dependency graph contains a cycle involving {} units",
                    remaining.len()
                )
            }

            Self::InconsistentProgram { reason } => {
                write!(f, "inconsistent program analysis: {reason}")
            }
        }
    }
}

impl std::error::Error for ProgramAnalysisError {}

pub type ProgramAnalysisResult<T> =
    Result<T, ProgramAnalysisError>;

// =============================================================================
// Identifiers
// =============================================================================

/// Opaque compiler analysis identifier.
///
/// This is not a ZUIR ProgramId, ModuleId or source AST identifier.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash
)]
pub struct CompilationUnitId(String);

impl CompilationUnitId {
    pub fn new(value: impl Into<String>) -> ProgramAnalysisResult<Self> {
        let value = value.into();

        if value.is_empty() || value.trim() != value {
            return Err(ProgramAnalysisError::EmptyIdentifier {
                field: "compilation unit identifier",
            });
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for CompilationUnitId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Extensible computational-domain identifier.
///
/// Examples include classical, quantum, AI, tensor, HDL,
/// distributed, networking and future domains.
///
/// These are identifiers, not a closed enumeration.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash
)]
pub struct DomainId(String);

impl DomainId {
    pub fn new(value: impl Into<String>) -> ProgramAnalysisResult<Self> {
        let value = value.into();

        if value.is_empty() || value.trim() != value {
            return Err(ProgramAnalysisError::EmptyIdentifier {
                field: "domain identifier",
            });
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for DomainId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

// =============================================================================
// External entity references
// =============================================================================

/// Opaque reference to an entity owned by another subsystem.
///
/// `authority` identifies the subsystem responsible for the entity.
///
/// `identifier` identifies the entity within that authority.
///
/// This structure does not validate the existence of the referenced
/// entity. The owning subsystem must perform that validation.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash
)]
pub struct EntityReference {
    authority: String,
    identifier: String,
}

impl EntityReference {
    pub fn new(
        authority: impl Into<String>,
        identifier: impl Into<String>,
    ) -> ProgramAnalysisResult<Self> {
        let authority = authority.into();
        let identifier = identifier.into();

        if authority.is_empty() || authority.trim() != authority {
            return Err(ProgramAnalysisError::InvalidReference {
                field: "authority",
            });
        }

        if identifier.is_empty() || identifier.trim() != identifier {
            return Err(ProgramAnalysisError::InvalidReference {
                field: "identifier",
            });
        }

        Ok(Self {
            authority,
            identifier,
        })
    }

    pub fn authority(&self) -> &str {
        &self.authority
    }

    pub fn identifier(&self) -> &str {
        &self.identifier
    }

    pub fn validate(&self) -> ProgramAnalysisResult<()> {
        if self.authority.is_empty()
            || self.identifier.is_empty()
            || self.authority.trim() != self.authority
            || self.identifier.trim() != self.identifier
        {
            return Err(ProgramAnalysisError::InvalidReference {
                field: "entity reference",
            });
        }

        Ok(())
    }
}

impl fmt::Display for EntityReference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.authority, self.identifier)
    }
}

// =============================================================================
// Analysis evidence
// =============================================================================

/// Identifies where an analysis observation originated.
///
/// This is provenance for compiler analysis, not a replacement
/// for the repository's canonical provenance model.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash
)]
pub struct EvidenceOrigin {
    subsystem: String,
    analysis: String,
    entity: Option<EntityReference>,
}

impl EvidenceOrigin {
    pub fn new(
        subsystem: impl Into<String>,
        analysis: impl Into<String>,
        entity: Option<EntityReference>,
    ) -> ProgramAnalysisResult<Self> {
        let subsystem = subsystem.into();
        let analysis = analysis.into();

        if subsystem.is_empty() || subsystem.trim() != subsystem {
            return Err(ProgramAnalysisError::InvalidObservation {
                reason: "invalid evidence subsystem".into(),
            });
        }

        if analysis.is_empty() || analysis.trim() != analysis {
            return Err(ProgramAnalysisError::InvalidObservation {
                reason: "invalid evidence analysis identifier".into(),
            });
        }

        if let Some(reference) = &entity {
            reference.validate()?;
        }

        Ok(Self {
            subsystem,
            analysis,
            entity,
        })
    }

    pub fn subsystem(&self) -> &str {
        &self.subsystem
    }

    pub fn analysis(&self) -> &str {
        &self.analysis
    }

    pub fn entity(&self) -> Option<&EntityReference> {
        self.entity.as_ref()
    }
}

// =============================================================================
// Analysis completeness
// =============================================================================

/// Describes how much of a particular analysis has completed.
///
/// Unknown or incomplete analysis must never be treated as success.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnalysisStatus {
    NotStarted,
    InProgress,
    Complete,
    Incomplete,
    Unsupported,
    Failed,
}

/// A report describing the completeness of a program analysis.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnalysisCompleteness {
    status: AnalysisStatus,
    observations: usize,
    unresolved_items: usize,
    explanation: Option<String>,
}

impl AnalysisCompleteness {
    pub fn new(
        status: AnalysisStatus,
        observations: usize,
        unresolved_items: usize,
        explanation: Option<String>,
    ) -> Self {
        Self {
            status,
            observations,
            unresolved_items,
            explanation,
        }
    }

    pub fn not_started() -> Self {
        Self::new(AnalysisStatus::NotStarted, 0, 0, None)
    }

    pub fn status(&self) -> AnalysisStatus {
        self.status
    }

    pub fn observations(&self) -> usize {
        self.observations
    }

    pub fn unresolved_items(&self) -> usize {
        self.unresolved_items
    }

    pub fn explanation(&self) -> Option<&str> {
        self.explanation.as_deref()
    }

    /// Whether the analysis has positively completed.
    pub fn is_complete(&self) -> bool {
        self.status == AnalysisStatus::Complete
            && self.unresolved_items == 0
    }

    pub fn validate(&self) -> ProgramAnalysisResult<()> {
        if self.status == AnalysisStatus::Complete
            && self.unresolved_items != 0
        {
            return Err(ProgramAnalysisError::InvalidObservation {
                reason:
                    "completed analysis cannot contain unresolved items"
                        .into(),
            });
        }

        Ok(())
    }
}

// =============================================================================
// Compilation units
// =============================================================================

/// A compiler-level compilation-unit description.
///
/// The actual source and AST remain owned by the frontend.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompilationUnit {
    id: CompilationUnitId,
    source: Option<EntityReference>,
    semantic_entity: Option<EntityReference>,
    ir_entity: Option<EntityReference>,
    completeness: AnalysisCompleteness,
}

impl CompilationUnit {
    pub fn new(id: CompilationUnitId) -> Self {
        Self {
            id,
            source: None,
            semantic_entity: None,
            ir_entity: None,
            completeness: AnalysisCompleteness::not_started(),
        }
    }

    pub fn id(&self) -> &CompilationUnitId {
        &self.id
    }

    pub fn source(&self) -> Option<&EntityReference> {
        self.source.as_ref()
    }

    pub fn semantic_entity(&self) -> Option<&EntityReference> {
        self.semantic_entity.as_ref()
    }

    pub fn ir_entity(&self) -> Option<&EntityReference> {
        self.ir_entity.as_ref()
    }

    pub fn completeness(&self) -> &AnalysisCompleteness {
        &self.completeness
    }

    pub fn set_source(
        &mut self,
        reference: EntityReference,
    ) -> ProgramAnalysisResult<()> {
        reference.validate()?;
        self.source = Some(reference);
        Ok(())
    }

    pub fn set_semantic_entity(
        &mut self,
        reference: EntityReference,
    ) -> ProgramAnalysisResult<()> {
        reference.validate()?;
        self.semantic_entity = Some(reference);
        Ok(())
    }

    pub fn set_ir_entity(
        &mut self,
        reference: EntityReference,
    ) -> ProgramAnalysisResult<()> {
        reference.validate()?;
        self.ir_entity = Some(reference);
        Ok(())
    }

    pub fn set_completeness(
        &mut self,
        completeness: AnalysisCompleteness,
    ) -> ProgramAnalysisResult<()> {
        completeness.validate()?;
        self.completeness = completeness;
        Ok(())
    }

    pub fn validate(&self) -> ProgramAnalysisResult<()> {
        self.completeness.validate()?;

        if let Some(reference) = &self.source {
            reference.validate()?;
        }

        if let Some(reference) = &self.semantic_entity {
            reference.validate()?;
        }

        if let Some(reference) = &self.ir_entity {
            reference.validate()?;
        }

        Ok(())
    }
}

// =============================================================================
// Dependency graph
// =============================================================================

/// Dependency relationship between compilation units.
///
/// The kind is extensible and interpreted by the subsystem that
/// produces and consumes the relationship.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash
)]
pub struct DependencyKind(String);

impl DependencyKind {
    pub fn new(
        value: impl Into<String>,
    ) -> ProgramAnalysisResult<Self> {
        let value = value.into();

        if value.is_empty() || value.trim() != value {
            return Err(ProgramAnalysisError::EmptyIdentifier {
                field: "dependency kind",
            });
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// An explicitly directed dependency.
///
/// `dependent` requires `prerequisite` to be available first.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash
)]
pub struct DependencyEdge {
    dependent: CompilationUnitId,
    prerequisite: CompilationUnitId,
    kind: DependencyKind,
}

impl DependencyEdge {
    pub fn new(
        dependent: CompilationUnitId,
        prerequisite: CompilationUnitId,
        kind: DependencyKind,
    ) -> ProgramAnalysisResult<Self> {
        if dependent == prerequisite {
            return Err(ProgramAnalysisError::SelfDependency {
                id: dependent,
            });
        }

        Ok(Self {
            dependent,
            prerequisite,
            kind,
        })
    }

    pub fn dependent(&self) -> &CompilationUnitId {
        &self.dependent
    }

    pub fn prerequisite(&self) -> &CompilationUnitId {
        &self.prerequisite
    }

    pub fn kind(&self) -> &DependencyKind {
        &self.kind
    }
}

/// Deterministic dependency graph.
///
/// This graph records relationships; it does not itself decide
/// whether cyclic dependencies are legal in the source language.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DependencyGraph {
    edges: BTreeSet<DependencyEdge>,
}

impl DependencyGraph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn edges(&self) -> &BTreeSet<DependencyEdge> {
        &self.edges
    }

    pub fn add(
        &mut self,
        edge: DependencyEdge,
    ) -> ProgramAnalysisResult<()> {
        if !self.edges.insert(edge.clone()) {
            return Err(ProgramAnalysisError::DuplicateDependency {
                dependency: edge,
            });
        }

        Ok(())
    }

    pub fn remove(&mut self, edge: &DependencyEdge) -> bool {
        self.edges.remove(edge)
    }

    pub fn dependencies_of(
        &self,
        id: &CompilationUnitId,
    ) -> impl Iterator<Item = &CompilationUnitId> {
        self.edges
            .iter()
            .filter(move |edge| &edge.dependent == id)
            .map(|edge| &edge.prerequisite)
    }

    pub fn dependents_of(
        &self,
        id: &CompilationUnitId,
    ) -> impl Iterator<Item = &CompilationUnitId> {
        self.edges
            .iter()
            .filter(move |edge| &edge.prerequisite == id)
            .map(|edge| &edge.dependent)
    }

    /// Validate that every edge references a known compilation unit.
    pub fn validate(
        &self,
        units: &BTreeMap<CompilationUnitId, CompilationUnit>,
    ) -> ProgramAnalysisResult<()> {
        for edge in &self.edges {
            if !units.contains_key(&edge.dependent) {
                return Err(ProgramAnalysisError::UnknownCompilationUnit {
                    id: edge.dependent.clone(),
                });
            }

            if !units.contains_key(&edge.prerequisite) {
                return Err(ProgramAnalysisError::UnknownCompilationUnit {
                    id: edge.prerequisite.clone(),
                });
            }
        }

        Ok(())
    }

    /// Produce a deterministic topological order.
    ///
    /// Prerequisites appear before their dependents.
    ///
    /// A cycle is reported explicitly; no dependency is silently
    /// discarded and no arbitrary ordering is invented.
    pub fn topological_order(
        &self,
        units: &BTreeMap<CompilationUnitId, CompilationUnit>,
    ) -> ProgramAnalysisResult<Vec<CompilationUnitId>> {
        self.validate(units)?;

        let mut indegree = BTreeMap::<CompilationUnitId, usize>::new();

        let mut outgoing =
            BTreeMap::<CompilationUnitId, BTreeSet<CompilationUnitId>>::new();

        for id in units.keys() {
            indegree.insert(id.clone(), 0);
            outgoing.insert(id.clone(), BTreeSet::new());
        }

        for edge in &self.edges {
            let prerequisite = edge.prerequisite.clone();
            let dependent = edge.dependent.clone();

            let neighbors = outgoing
                .get_mut(&prerequisite)
                .expect("validated prerequisite must exist");

            if neighbors.insert(dependent.clone()) {
                let count = indegree
                    .get_mut(&dependent)
                    .expect("validated dependent must exist");

                *count += 1;
            }
        }

        let mut ready = VecDeque::new();

        for (id, degree) in &indegree {
            if *degree == 0 {
                ready.push_back(id.clone());
            }
        }

        let mut ordered = Vec::with_capacity(units.len());

        while let Some(id) = ready.pop_front() {
            ordered.push(id.clone());

            if let Some(neighbors) = outgoing.get(&id) {
                for dependent in neighbors {
                    let degree = indegree
                        .get_mut(dependent)
                        .expect("dependent must exist");

                    *degree -= 1;

                    if *degree == 0 {
                        ready.push_back(dependent.clone());
                    }
                }
            }
        }

        if ordered.len() != units.len() {
            let remaining = indegree
                .into_iter()
                .filter_map(|(id, degree)| {
                    (degree != 0).then_some(id)
                })
                .collect();

            return Err(ProgramAnalysisError::DependencyCycle {
                remaining,
            });
        }

        Ok(ordered)
    }
}

// =============================================================================
// Domain participation
// =============================================================================

/// An observation that an externally owned entity participates
/// in a computational domain.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DomainObservation {
    domain: DomainId,
    entity: EntityReference,
    origin: EvidenceOrigin,
}

impl DomainObservation {
    pub fn new(
        domain: DomainId,
        entity: EntityReference,
        origin: EvidenceOrigin,
    ) -> ProgramAnalysisResult<Self> {
        entity.validate()?;

        Ok(Self {
            domain,
            entity,
            origin,
        })
    }

    pub fn domain(&self) -> &DomainId {
        &self.domain
    }

    pub fn entity(&self) -> &EntityReference {
        &self.entity
    }

    pub fn origin(&self) -> &EvidenceOrigin {
        &self.origin
    }
}

/// Deterministic index of domain participation.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DomainIndex {
    observations: BTreeMap<
        (DomainId, EntityReference),
        DomainObservation,
    >,
}

impl DomainIndex {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(
        &mut self,
        observation: DomainObservation,
    ) -> ProgramAnalysisResult<()> {
        let key = (
            observation.domain.clone(),
            observation.entity.clone(),
        );

        if self.observations.contains_key(&key) {
            return Err(
                ProgramAnalysisError::DuplicateDomainObservation {
                    domain: observation.domain,
                    entity: observation.entity,
                },
            );
        }

        self.observations.insert(key, observation);

        Ok(())
    }

    pub fn observations(
        &self,
    ) -> impl Iterator<Item = &DomainObservation> {
        self.observations.values()
    }

    pub fn contains_domain(&self, domain: &DomainId) -> bool {
        self.observations
            .keys()
            .any(|(candidate, _)| candidate == domain)
    }

    pub fn domains(&self) -> BTreeSet<DomainId> {
        self.observations
            .keys()
            .map(|(domain, _)| domain.clone())
            .collect()
    }

    pub fn len(&self) -> usize {
        self.observations.len()
    }

    pub fn is_empty(&self) -> bool {
        self.observations.is_empty()
    }
}

// =============================================================================
// External semantic fact references
// =============================================================================

/// Distinguishes the purpose of an externally owned semantic reference.
///
/// This does not define the meaning of the referenced object.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash
)]
pub enum SemanticReferenceKind {
    Type,
    Effect,
    Requirement,
    Capability,
    Constraint,
    Preference,
    Contract,
    Resource,
    Policy,
    Provenance,
    Other(String),
}

/// Reference to an externally owned semantic fact.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticFactReference {
    kind: SemanticReferenceKind,
    entity: EntityReference,
    origin: EvidenceOrigin,
}

impl SemanticFactReference {
    pub fn new(
        kind: SemanticReferenceKind,
        entity: EntityReference,
        origin: EvidenceOrigin,
    ) -> ProgramAnalysisResult<Self> {
        entity.validate()?;

        Ok(Self {
            kind,
            entity,
            origin,
        })
    }

    pub fn kind(&self) -> &SemanticReferenceKind {
        &self.kind
    }

    pub fn entity(&self) -> &EntityReference {
        &self.entity
    }

    pub fn origin(&self) -> &EvidenceOrigin {
        &self.origin
    }
}

// =============================================================================
// Program analysis
// =============================================================================

/// Aggregate program-level compiler analysis.
///
/// This is an analysis index, not a second program IR.
///
/// The semantic layer remains authoritative for program meaning.
/// ZUIR remains authoritative for canonical IR structure.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ProgramAnalysis {
    units: BTreeMap<CompilationUnitId, CompilationUnit>,
    dependencies: DependencyGraph,
    domains: DomainIndex,
    semantic_facts: BTreeSet<SemanticFactKey>,
    completeness: BTreeMap<String, AnalysisCompleteness>,
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord
)]
struct SemanticFactKey {
    kind: SemanticReferenceKind,
    entity: EntityReference,
    subsystem: String,
    analysis: String,
}

impl ProgramAnalysis {
    pub fn new() -> Self {
        Self::default()
    }

    // -------------------------------------------------------------------------
    // Compilation units
    // -------------------------------------------------------------------------

    pub fn add_unit(
        &mut self,
        unit: CompilationUnit,
    ) -> ProgramAnalysisResult<()> {
        unit.validate()?;

        let id = unit.id.clone();

        if self.units.contains_key(&id) {
            return Err(
                ProgramAnalysisError::DuplicateCompilationUnit { id }
            );
        }

        self.units.insert(id, unit);

        Ok(())
    }

    pub fn unit(
        &self,
        id: &CompilationUnitId,
    ) -> Option<&CompilationUnit> {
        self.units.get(id)
    }

    pub fn units(
        &self,
    ) -> &BTreeMap<CompilationUnitId, CompilationUnit> {
        &self.units
    }

    pub fn unit_count(&self) -> usize {
        self.units.len()
    }

    // -------------------------------------------------------------------------
    // Dependencies
    // -------------------------------------------------------------------------

    pub fn dependencies(&self) -> &DependencyGraph {
        &self.dependencies
    }

    pub fn add_dependency(
        &mut self,
        edge: DependencyEdge,
    ) -> ProgramAnalysisResult<()> {
        if !self.units.contains_key(&edge.dependent) {
            return Err(ProgramAnalysisError::UnknownCompilationUnit {
                id: edge.dependent,
            });
        }

        if !self.units.contains_key(&edge.prerequisite) {
            return Err(ProgramAnalysisError::UnknownCompilationUnit {
                id: edge.prerequisite,
            });
        }

        self.dependencies.add(edge)
    }

    pub fn dependency_order(
        &self,
    ) -> ProgramAnalysisResult<Vec<CompilationUnitId>> {
        self.dependencies.topological_order(&self.units)
    }

    // -------------------------------------------------------------------------
    // Domains
    // -------------------------------------------------------------------------

    pub fn domains(&self) -> &DomainIndex {
        &self.domains
    }

    pub fn add_domain_observation(
        &mut self,
        observation: DomainObservation,
    ) -> ProgramAnalysisResult<()> {
        self.domains.insert(observation)
    }

    // -------------------------------------------------------------------------
    // Semantic references
    // -------------------------------------------------------------------------

    pub fn add_semantic_fact(
        &mut self,
        fact: SemanticFactReference,
    ) -> bool {
        self.semantic_facts.insert(SemanticFactKey {
            kind: fact.kind,
            entity: fact.entity,
            subsystem: fact.origin.subsystem,
            analysis: fact.origin.analysis,
        })
    }

    pub fn semantic_fact_count(&self) -> usize {
        self.semantic_facts.len()
    }

    // -------------------------------------------------------------------------
    // Analysis completeness
    // -------------------------------------------------------------------------

    pub fn set_analysis_completeness(
        &mut self,
        analysis: impl Into<String>,
        completeness: AnalysisCompleteness,
    ) -> ProgramAnalysisResult<()> {
        let analysis = analysis.into();

        if analysis.is_empty() || analysis.trim() != analysis {
            return Err(ProgramAnalysisError::EmptyIdentifier {
                field: "analysis name",
            });
        }

        completeness.validate()?;

        self.completeness.insert(analysis, completeness);

        Ok(())
    }

    pub fn analysis_completeness(
        &self,
        analysis: &str,
    ) -> Option<&AnalysisCompleteness> {
        self.completeness.get(analysis)
    }

    pub fn analyses(
        &self,
    ) -> &BTreeMap<String, AnalysisCompleteness> {
        &self.completeness
    }

    /// Returns true only when all registered analyses have completed.
    ///
    /// An empty analysis collection is not proof of successful analysis.
    pub fn all_analyses_complete(&self) -> bool {
        !self.completeness.is_empty()
            && self.completeness.values().all(
                AnalysisCompleteness::is_complete
            )
    }

    // -------------------------------------------------------------------------
    // Validation
    // -------------------------------------------------------------------------

    /// Validate all local program-analysis invariants.
    ///
    /// This does not replace semantic verification or the canonical
    /// ZUIR verifier.
    pub fn validate(&self) -> ProgramAnalysisResult<()> {
        for (id, unit) in &self.units {
            if id != unit.id() {
                return Err(
                    ProgramAnalysisError::InconsistentProgram {
                        reason:
                            "compilation-unit index does not match unit ID"
                                .into(),
                    }
                );
            }

            unit.validate()?;
        }

        self.dependencies.validate(&self.units)?;

        for completeness in self.completeness.values() {
            completeness.validate()?;
        }

        for fact in &self.semantic_facts {
            if fact.entity.authority.is_empty()
                || fact.entity.identifier.is_empty()
                || fact.subsystem.is_empty()
                || fact.analysis.is_empty()
            {
                return Err(
                    ProgramAnalysisError::InconsistentProgram {
                        reason: "invalid semantic fact reference".into(),
                    }
                );
            }
        }

        Ok(())
    }

    /// Number of compilation units and observations indexed.
    pub fn statistics(&self) -> ProgramAnalysisStatistics {
        ProgramAnalysisStatistics {
            compilation_units: self.units.len(),
            dependency_edges: self.dependencies.edges.len(),
            domain_observations: self.domains.len(),
            semantic_references: self.semantic_facts.len(),
            registered_analyses: self.completeness.len(),
        }
    }
}

// =============================================================================
// Statistics
// =============================================================================

/// Snapshot of program analysis collection sizes.
///
/// These are measurements, not capacity limits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProgramAnalysisStatistics {
    pub compilation_units: usize,
    pub dependency_edges: usize,
    pub domain_observations: usize,
    pub semantic_references: usize,
    pub registered_analyses: usize,
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn unit(value: &str) -> CompilationUnit {
        CompilationUnit::new(
            CompilationUnitId::new(value).unwrap()
        )
    }

    fn dependency(
        dependent: &str,
        prerequisite: &str,
    ) -> DependencyEdge {
        DependencyEdge::new(
            CompilationUnitId::new(dependent).unwrap(),
            CompilationUnitId::new(prerequisite).unwrap(),
            DependencyKind::new("compilation").unwrap(),
        )
        .unwrap()
    }

    #[test]
    fn creates_empty_program_analysis() {
        let analysis = ProgramAnalysis::new();

        assert_eq!(analysis.unit_count(), 0);
        assert_eq!(analysis.semantic_fact_count(), 0);
        assert!(analysis.validate().is_ok());
    }

    #[test]
    fn rejects_empty_unit_identifier() {
        assert!(CompilationUnitId::new("").is_err());
        assert!(CompilationUnitId::new(" ").is_err());
    }

    #[test]
    fn rejects_empty_domain_identifier() {
        assert!(DomainId::new("").is_err());
    }

    #[test]
    fn rejects_empty_external_reference() {
        assert!(EntityReference::new("", "entity").is_err());
        assert!(EntityReference::new("semantic", "").is_err());
    }

    #[test]
    fn adds_unique_compilation_units() {
        let mut analysis = ProgramAnalysis::new();

        analysis.add_unit(unit("module.a")).unwrap();

        assert_eq!(analysis.unit_count(), 1);

        assert!(matches!(
            analysis.add_unit(unit("module.a")),
            Err(ProgramAnalysisError::DuplicateCompilationUnit { .. })
        ));
    }

    #[test]
    fn adds_valid_dependency() {
        let mut analysis = ProgramAnalysis::new();

        analysis.add_unit(unit("a")).unwrap();
        analysis.add_unit(unit("b")).unwrap();

        analysis.add_dependency(dependency("b", "a")).unwrap();

        assert_eq!(analysis.dependencies().edges().len(), 1);
    }

    #[test]
    fn rejects_unknown_dependency() {
        let mut analysis = ProgramAnalysis::new();

        analysis.add_unit(unit("a")).unwrap();

        assert!(matches!(
            analysis.add_dependency(dependency("a", "missing")),
            Err(ProgramAnalysisError::UnknownCompilationUnit { .. })
        ));
    }

    #[test]
    fn rejects_duplicate_dependency() {
        let mut analysis = ProgramAnalysis::new();

        analysis.add_unit(unit("a")).unwrap();
        analysis.add_unit(unit("b")).unwrap();

        let edge = dependency("b", "a");

        analysis.add_dependency(edge.clone()).unwrap();

        assert!(matches!(
            analysis.add_dependency(edge),
            Err(ProgramAnalysisError::DuplicateDependency { .. })
        ));
    }

    #[test]
    fn rejects_self_dependency() {
        let id = CompilationUnitId::new("a").unwrap();

        assert!(matches!(
            DependencyEdge::new(
                id.clone(),
                id.clone(),
                DependencyKind::new("compilation").unwrap()
            ),
            Err(ProgramAnalysisError::SelfDependency { .. })
        ));
    }

    #[test]
    fn calculates_deterministic_dependency_order() {
        let mut analysis = ProgramAnalysis::new();

        analysis.add_unit(unit("a")).unwrap();
        analysis.add_unit(unit("b")).unwrap();
        analysis.add_unit(unit("c")).unwrap();

        analysis.add_dependency(dependency("b", "a")).unwrap();
        analysis.add_dependency(dependency("c", "b")).unwrap();

        let order = analysis.dependency_order().unwrap();

        let a = CompilationUnitId::new("a").unwrap();
        let b = CompilationUnitId::new("b").unwrap();
        let c = CompilationUnitId::new("c").unwrap();

        assert_eq!(order, vec![a, b, c]);
    }

    #[test]
    fn reports_dependency_cycles() {
        let mut analysis = ProgramAnalysis::new();

        analysis.add_unit(unit("a")).unwrap();
        analysis.add_unit(unit("b")).unwrap();

        analysis.add_dependency(dependency("b", "a")).unwrap();
        analysis.add_dependency(dependency("a", "b")).unwrap();

        assert!(matches!(
            analysis.dependency_order(),
            Err(ProgramAnalysisError::DependencyCycle { .. })
        ));
    }

    #[test]
    fn accepts_extensible_computational_domains() {
        let mut index = DomainIndex::new();

        let entity = EntityReference::new(
            "semantic",
            "operation.1"
        )
        .unwrap();

        let origin = EvidenceOrigin::new(
            "semantic",
            "domain-analysis",
            Some(entity.clone())
        )
        .unwrap();

        let domain = DomainId::new(
            "future.photonic.computing"
        )
        .unwrap();

        index.insert(
            DomainObservation::new(
                domain.clone(),
                entity,
                origin
            )
            .unwrap()
        )
        .unwrap();

        assert!(index.contains_domain(&domain));
    }

    #[test]
    fn rejects_duplicate_domain_observation() {
        let mut index = DomainIndex::new();

        let entity = EntityReference::new(
            "semantic",
            "operation.1"
        )
        .unwrap();

        let domain = DomainId::new("classical").unwrap();

        let origin = EvidenceOrigin::new(
            "semantic",
            "analysis",
            None
        )
        .unwrap();

        let observation = DomainObservation::new(
            domain,
            entity,
            origin
        )
        .unwrap();

        index.insert(observation.clone()).unwrap();

        assert!(matches!(
            index.insert(observation),
            Err(ProgramAnalysisError::DuplicateDomainObservation { .. })
        ));
    }

    #[test]
    fn preserves_external_semantic_authority() {
        let reference = EntityReference::new(
            "semantic",
            "capability.quantum"
        )
        .unwrap();

        let origin = EvidenceOrigin::new(
            "semantic",
            "capability-analysis",
            Some(reference.clone())
        )
        .unwrap();

        let fact = SemanticFactReference::new(
            SemanticReferenceKind::Capability,
            reference,
            origin
        )
        .unwrap();

        let mut analysis = ProgramAnalysis::new();

        assert!(analysis.add_semantic_fact(fact));
        assert_eq!(analysis.semantic_fact_count(), 1);
    }

    #[test]
    fn does_not_treat_missing_analysis_as_complete() {
        let analysis = ProgramAnalysis::new();

        assert!(!analysis.all_analyses_complete());
    }

    #[test]
    fn rejects_completed_analysis_with_unresolved_items() {
        let mut analysis = ProgramAnalysis::new();

        let completeness = AnalysisCompleteness::new(
            AnalysisStatus::Complete,
            10,
            1,
            None,
        );

        assert!(analysis.set_analysis_completeness(
            "semantic",
            completeness
        ).is_err());
    }

    #[test]
    fn incomplete_analysis_is_not_success() {
        let mut analysis = ProgramAnalysis::new();

        analysis.set_analysis_completeness(
            "semantic",
            AnalysisCompleteness::new(
                AnalysisStatus::Incomplete,
                10,
                2,
                Some("Unresolved references".into()),
            ),
        ).unwrap();

        assert!(!analysis.all_analyses_complete());
    }

    #[test]
    fn validates_program_statistics() {
        let mut analysis = ProgramAnalysis::new();

        analysis.add_unit(unit("a")).unwrap();
        analysis.add_unit(unit("b")).unwrap();
        analysis.add_dependency(dependency("b", "a")).unwrap();

        let statistics = analysis.statistics();

        assert_eq!(statistics.compilation_units, 2);
        assert_eq!(statistics.dependency_edges, 1);
    }

    #[test]
    fn validates_complete_program_analysis() {
        let mut analysis = ProgramAnalysis::new();

        analysis.add_unit(unit("main")).unwrap();

        analysis.set_analysis_completeness(
            "structural",
            AnalysisCompleteness::new(
                AnalysisStatus::Complete,
                1,
                0,
                None,
            ),
        ).unwrap();

        assert!(analysis.validate().is_ok());
        assert!(analysis.all_analyses_complete());
    }
}
