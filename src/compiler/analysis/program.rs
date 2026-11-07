#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani Compiler — Program Analysis
//!
//! Path: src/compiler/analysis/program.rs
//!
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//!
//! PURPOSE
//!
//! Provides the compiler's program-level analysis index.
//!
//! This module aggregates observations produced by authoritative
//! repository subsystems without defining another program model,
//! semantic model, intermediate representation, or dependency graph.
//!
//! OWNERSHIP
//!
//! This module owns:
//!
//! - Program-level analysis observations.
//! - Compilation-unit analysis records.
//! - Domain participation observations.
//! - References to externally owned semantic facts.
//! - Analysis evidence and provenance references.
//! - Analysis completeness records.
//! - Program-analysis consistency validation.
//! - Analysis statistics.
//!
//! This module does NOT own:
//!
//! - Source syntax or AST.
//! - Name resolution.
//! - Type definitions.
//! - Semantic effects.
//! - Semantic ownership.
//! - Resource semantics.
//! - Capability definitions.
//! - Requirement semantics.
//! - Dependency graph storage.
//! - Dependency ordering.
//! - Canonical ZUIR entities.
//! - Domain-specific IR.
//! - Physical hardware discovery.
//! - Resource allocation.
//! - Backend selection.
//! - Runtime scheduling.
//!
//! CANONICAL AUTHORITIES
//!
//! Source syntax:
//!     grammar/Zamani.g4
//!
//! Normative semantic model:
//!     grammar/specification/semantic-model.md
//!
//! Resource semantics:
//!     grammar/spec/resources.md
//!
//! Scalability:
//!     grammar/specification/scalability-model.md
//!
//! POCO-REAF:
//!     grammar/specification/poco-reaf.md
//!
//! Compilation-unit identity:
//!     compiler/context/dependency_context.rs
//!
//! Dependency analysis:
//!     compiler/analysis/dependency.rs
//!
//! Semantic analysis:
//!     frontend/semantic/
//!
//! Canonical universal IR:
//!     zuir/
//!
//! EFFECTIVE OWNERSHIP
//!
//! Requirement, Capability, Constraint, Preference, and Hint
//! remain distinct concepts.
//!
//! Feasibility is established by the appropriate downstream
//! analysis and planning authorities.
//!
//! SCALABILITY
//!
//! This module introduces no arbitrary universal limits on:
//!
//! - Compilation units.
//! - Computational domains.
//! - Domain observations.
//! - Semantic references.
//! - Analysis records.
//! - Program size.
//! - Operations.
//! - Functions.
//! - Modules.
//! - Hardware resources.
//! - Quantum resources.
//! - Distributed participants.
//! - Tensor dimensions.
//!
//! Actual processing remains subject to available memory,
//! processing capacity, Rust representation limits, and
//! explicitly configured operational budgets.
//!
//! No finite implementation can promise literal infinity.
//!
//! DETERMINISM
//!
//! Ordered collections provide deterministic iteration.
//!
//! Equivalent observations produce equivalent analysis indexes.
//!
//! No randomized iteration order is used.
//!
//! SECURITY
//!
//! External references are opaque.
//!
//! This module never interprets an external identifier as proof
//! that the referenced entity exists or is valid.
//!
//! The owning subsystem must establish that evidence.
//!
//! SAFE RUST
//!
//! No unsafe Rust is permitted.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;
use std::sync::Arc;

use crate::compiler::context::dependency_context::CompilationUnitId;


// =============================================================================
// API VERSION
// =============================================================================

/// Public API version of the program-analysis contract.
///
/// This version is independent of the Zamani language version,
/// grammar version, semantic-model version and ZUIR version.
pub const PROGRAM_ANALYSIS_API_VERSION: u16 = 1;


// =============================================================================
// RESULT
// =============================================================================

/// Result type for program-analysis operations.
pub type ProgramAnalysisResult<T> =
    Result<T, ProgramAnalysisError>;


// =============================================================================
// ERRORS
// =============================================================================

/// Errors produced by program-analysis construction and validation.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ProgramAnalysisError {

    /// An identifier is empty or malformed.
    InvalidIdentifier {
        field: &'static str,
    },

    /// An external reference is invalid.
    InvalidReference {
        field: &'static str,
    },

    /// An analysis observation is invalid.
    InvalidObservation {
        reason: String,
    },

    /// A compilation unit was registered more than once.
    DuplicateCompilationUnit {
        id: CompilationUnitId,
    },

    /// An observation references an unknown compilation unit.
    UnknownCompilationUnit {
        id: CompilationUnitId,
    },

    /// A domain observation already exists.
    DuplicateDomainObservation {
        domain: DomainId,
        entity: EntityReference,
    },

    /// A semantic fact already exists.
    DuplicateSemanticFact,

    /// An analysis identifier already exists.
    DuplicateAnalysis {
        analysis: AnalysisId,
    },

    /// An analysis identifier does not exist.
    UnknownAnalysis {
        analysis: AnalysisId,
    },

    /// Analysis completeness contains inconsistent information.
    InvalidCompleteness {
        reason: String,
    },

    /// A program-analysis invariant was violated.
    InconsistentProgram {
        reason: String,
    },

    /// The program-analysis revision cannot be incremented.
    RevisionOverflow,
}

impl fmt::Display for ProgramAnalysisError {

    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {

        match self {

            Self::InvalidIdentifier { field } => {
                write!(
                    formatter,
                    "invalid identifier: {field}"
                )
            }

            Self::InvalidReference { field } => {
                write!(
                    formatter,
                    "invalid external reference: {field}"
                )
            }

            Self::InvalidObservation { reason } => {
                write!(
                    formatter,
                    "invalid analysis observation: {reason}"
                )
            }

            Self::DuplicateCompilationUnit { id } => {
                write!(
                    formatter,
                    "duplicate compilation unit: {id}"
                )
            }

            Self::UnknownCompilationUnit { id } => {
                write!(
                    formatter,
                    "unknown compilation unit: {id}"
                )
            }

            Self::DuplicateDomainObservation {
                domain,
                entity,
            } => {
                write!(
                    formatter,
                    "duplicate domain observation: {domain}, {entity}"
                )
            }

            Self::DuplicateSemanticFact => {
                formatter.write_str(
                    "duplicate semantic fact"
                )
            }

            Self::DuplicateAnalysis { analysis } => {
                write!(
                    formatter,
                    "duplicate analysis: {analysis}"
                )
            }

            Self::UnknownAnalysis { analysis } => {
                write!(
                    formatter,
                    "unknown analysis: {analysis}"
                )
            }

            Self::InvalidCompleteness { reason } => {
                write!(
                    formatter,
                    "invalid analysis completeness: {reason}"
                )
            }

            Self::InconsistentProgram { reason } => {
                write!(
                    formatter,
                    "inconsistent program analysis: {reason}"
                )
            }

            Self::RevisionOverflow => {
                formatter.write_str(
                    "program-analysis revision overflow"
                )
            }
        }
    }
}

impl Error for ProgramAnalysisError {}


// =============================================================================
// IDENTIFIER VALIDATION
// =============================================================================

fn validate_identifier(
    value: &str,
    field: &'static str,
) -> ProgramAnalysisResult<()> {

    if value.trim().is_empty()
        || value.trim() != value
        || value.chars().any(char::is_control)
    {
        return Err(
            ProgramAnalysisError::InvalidIdentifier {
                field,
            }
        );
    }

    Ok(())
}


// =============================================================================
// DOMAIN IDENTITY
// =============================================================================

/// Extensible computational-domain identity.
///
/// This is deliberately not a closed enumeration.
///
/// Examples:
///
/// - classical
/// - quantum
/// - hdl
/// - hybrid
/// - ai
/// - tensor
/// - distributed
/// - networking
/// - cryptography
/// - future.provider.domain
///
/// Introducing a new domain does not require changing this type.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash
)]
pub struct DomainId(Arc<str>);

impl DomainId {

    /// Create a computational-domain identifier.
    pub fn new(
        value: impl Into<Arc<str>>,
    ) -> ProgramAnalysisResult<Self> {

        let value = value.into();

        validate_identifier(
            &value,
            "computational domain",
        )?;

        Ok(Self(value))
    }

    /// Return the identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for DomainId {

    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {

        formatter.write_str(&self.0)
    }
}


// =============================================================================
// EXTERNAL ENTITY REFERENCE
// =============================================================================

/// Opaque reference to an entity owned by another subsystem.
///
/// The authority identifies the subsystem responsible for the entity.
///
/// The identifier identifies the entity within that subsystem.
///
/// This module does not establish whether the referenced entity
/// actually exists.
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

    authority: Arc<str>,

    identifier: Arc<str>,
}

impl EntityReference {

    /// Construct an external entity reference.
    pub fn new(
        authority: impl Into<Arc<str>>,
        identifier: impl Into<Arc<str>>,
    ) -> ProgramAnalysisResult<Self> {

        let authority = authority.into();
        let identifier = identifier.into();

        validate_identifier(
            &authority,
            "reference authority",
        )?;

        validate_identifier(
            &identifier,
            "reference identifier",
        )?;

        Ok(Self {
            authority,
            identifier,
        })
    }

    #[must_use]
    pub fn authority(&self) -> &str {
        &self.authority
    }

    #[must_use]
    pub fn identifier(&self) -> &str {
        &self.identifier
    }

    /// Validate local reference structure.
    pub fn validate(&self) -> ProgramAnalysisResult<()> {

        validate_identifier(
            &self.authority,
            "reference authority",
        )?;

        validate_identifier(
            &self.identifier,
            "reference identifier",
        )
    }
}

impl fmt::Display for EntityReference {

    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {

        write!(
            formatter,
            "{}:{}",
            self.authority,
            self.identifier
        )
    }
}


// =============================================================================
// EVIDENCE ORIGIN
// =============================================================================

/// Identifies the complete origin of an analysis observation.
///
/// This is a compiler-analysis evidence reference.
///
/// It does not replace the canonical provenance model.
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

    subsystem: Arc<str>,

    analysis: Arc<str>,

    entity: Option<EntityReference>,
}

impl EvidenceOrigin {

    pub fn new(
        subsystem: impl Into<Arc<str>>,
        analysis: impl Into<Arc<str>>,
        entity: Option<EntityReference>,
    ) -> ProgramAnalysisResult<Self> {

        let subsystem = subsystem.into();
        let analysis = analysis.into();

        validate_identifier(
            &subsystem,
            "evidence subsystem",
        )?;

        validate_identifier(
            &analysis,
            "evidence analysis",
        )?;

        if let Some(reference) = &entity {
            reference.validate()?;
        }

        Ok(Self {
            subsystem,
            analysis,
            entity,
        })
    }

    #[must_use]
    pub fn subsystem(&self) -> &str {
        &self.subsystem
    }

    #[must_use]
    pub fn analysis(&self) -> &str {
        &self.analysis
    }

    #[must_use]
    pub fn entity(&self) -> Option<&EntityReference> {
        self.entity.as_ref()
    }

    pub fn validate(&self) -> ProgramAnalysisResult<()> {

        validate_identifier(
            &self.subsystem,
            "evidence subsystem",
        )?;

        validate_identifier(
            &self.analysis,
            "evidence analysis",
        )?;

        if let Some(entity) = &self.entity {
            entity.validate()?;
        }

        Ok(())
    }
}


// =============================================================================
// ANALYSIS IDENTITY
// =============================================================================

/// Extensible analysis identity.
///
/// Examples:
///
/// structural
/// semantic
/// ownership
/// effects
/// capabilities
/// dependencies
/// program.portability
/// provider.custom_analysis
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash
)]
pub struct AnalysisId(Arc<str>);

impl AnalysisId {

    pub fn new(
        value: impl Into<Arc<str>>,
    ) -> ProgramAnalysisResult<Self> {

        let value = value.into();

        validate_identifier(
            &value,
            "analysis identifier",
        )?;

        Ok(Self(value))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for AnalysisId {

    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {

        formatter.write_str(&self.0)
    }
}


// =============================================================================
// ANALYSIS STATUS
// =============================================================================

/// State of an individual analysis.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq
)]
pub enum AnalysisStatus {

    /// Analysis has not started.
    NotStarted,

    /// Analysis is currently executing.
    InProgress,

    /// Analysis has completed successfully.
    Complete,

    /// Analysis completed only partially.
    Incomplete,

    /// The required analysis is unsupported.
    Unsupported,

    /// Analysis failed.
    Failed,
}


// =============================================================================
// ANALYSIS COMPLETENESS
// =============================================================================

/// Completeness information for an analysis.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnalysisCompleteness {

    status: AnalysisStatus,

    observations: usize,

    unresolved_items: usize,

    explanation: Option<Arc<str>>,
}

impl AnalysisCompleteness {

    /// Construct an analysis-completeness record.
    ///
    /// Local structural validation is performed by `validate`.
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
            explanation: explanation.map(Arc::from),
        }
    }

    pub fn not_started() -> Self {

        Self::new(
            AnalysisStatus::NotStarted,
            0,
            0,
            None,
        )
    }

    #[must_use]
    pub fn status(&self) -> AnalysisStatus {
        self.status
    }

    #[must_use]
    pub fn observations(&self) -> usize {
        self.observations
    }

    #[must_use]
    pub fn unresolved_items(&self) -> usize {
        self.unresolved_items
    }

    #[must_use]
    pub fn explanation(&self) -> Option<&str> {
        self.explanation.as_deref()
    }

    /// True only for explicitly completed analysis with no
    /// unresolved items.
    #[must_use]
    pub fn is_complete(&self) -> bool {

        self.status == AnalysisStatus::Complete
            && self.unresolved_items == 0
    }

    /// Whether the analysis can certify successful completion.
    #[must_use]
    pub fn is_successful(&self) -> bool {
        self.is_complete()
    }

    pub fn validate(&self) -> ProgramAnalysisResult<()> {

        if self.status == AnalysisStatus::Complete
            && self.unresolved_items != 0
        {
            return Err(
                ProgramAnalysisError::InvalidCompleteness {
                    reason:
                        "completed analysis has unresolved items".into(),
                }
            );
        }

        if self.status == AnalysisStatus::NotStarted
            && self.observations != 0
        {
            return Err(
                ProgramAnalysisError::InvalidCompleteness {
                    reason:
                        "not-started analysis contains observations".into(),
                }
            );
        }

        if self.status == AnalysisStatus::NotStarted
            && self.unresolved_items != 0
        {
            return Err(
                ProgramAnalysisError::InvalidCompleteness {
                    reason:
                        "not-started analysis contains unresolved items"
                            .into(),
                }
            );
        }

        Ok(())
    }
}


// =============================================================================
// COMPILATION UNIT ANALYSIS
// =============================================================================

/// Analysis information associated with one canonical compilation unit.
///
/// Compilation-unit identity is owned by:
///
/// `compiler/context/dependency_context.rs`
///
/// This module only records observations about that identity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompilationUnitAnalysis {

    id: CompilationUnitId,

    source: Option<EntityReference>,

    semantic_entity: Option<EntityReference>,

    ir_entity: Option<EntityReference>,

    completeness: AnalysisCompleteness,
}

impl CompilationUnitAnalysis {

    pub fn new(
        id: CompilationUnitId,
    ) -> Self {

        Self {
            id,
            source: None,
            semantic_entity: None,
            ir_entity: None,
            completeness: AnalysisCompleteness::not_started(),
        }
    }

    #[must_use]
    pub fn id(&self) -> &CompilationUnitId {
        &self.id
    }

    #[must_use]
    pub fn source(&self) -> Option<&EntityReference> {
        self.source.as_ref()
    }

    #[must_use]
    pub fn semantic_entity(&self) -> Option<&EntityReference> {
        self.semantic_entity.as_ref()
    }

    #[must_use]
    pub fn ir_entity(&self) -> Option<&EntityReference> {
        self.ir_entity.as_ref()
    }

    #[must_use]
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
// DOMAIN OBSERVATION
// =============================================================================

/// Evidence that an externally owned entity participates
/// in a particular computational domain.
///
/// The optional compilation-unit association identifies
/// where the observation belongs.
///
/// The domain remains an extensible identifier.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DomainObservation {

    domain: DomainId,

    entity: EntityReference,

    origin: EvidenceOrigin,

    compilation_unit: Option<CompilationUnitId>,
}

impl DomainObservation {

    pub fn new(
        domain: DomainId,
        entity: EntityReference,
        origin: EvidenceOrigin,
    ) -> ProgramAnalysisResult<Self> {

        entity.validate()?;
        origin.validate()?;

        Ok(Self {
            domain,
            entity,
            origin,
            compilation_unit: None,
        })
    }

    pub fn for_compilation_unit(
        mut self,
        unit: CompilationUnitId,
    ) -> Self {

        self.compilation_unit = Some(unit);

        self
    }

    #[must_use]
    pub fn domain(&self) -> &DomainId {
        &self.domain
    }

    #[must_use]
    pub fn entity(&self) -> &EntityReference {
        &self.entity
    }

    #[must_use]
    pub fn origin(&self) -> &EvidenceOrigin {
        &self.origin
    }

    #[must_use]
    pub fn compilation_unit(
        &self,
    ) -> Option<&CompilationUnitId> {

        self.compilation_unit.as_ref()
    }

    pub fn validate(&self) -> ProgramAnalysisResult<()> {

        self.entity.validate()?;
        self.origin.validate()?;

        Ok(())
    }
}


// =============================================================================
// DOMAIN OBSERVATION INDEX
// =============================================================================

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord
)]
struct DomainObservationKey {

    domain: DomainId,

    entity: EntityReference,

    compilation_unit: Option<CompilationUnitId>,
}

/// Deterministic index of domain participation observations.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DomainIndex {

    observations: BTreeMap<
        DomainObservationKey,
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

        observation.validate()?;

        let key = DomainObservationKey {
            domain: observation.domain.clone(),
            entity: observation.entity.clone(),
            compilation_unit: observation.compilation_unit.clone(),
        };

        if self.observations.contains_key(&key) {

            return Err(
                ProgramAnalysisError::DuplicateDomainObservation {
                    domain: observation.domain,
                    entity: observation.entity,
                }
            );
        }

        self.observations.insert(
            key,
            observation,
        );

        Ok(())
    }

    #[must_use]
    pub fn observations(
        &self,
    ) -> impl Iterator<Item = &DomainObservation> {

        self.observations.values()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.observations.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.observations.is_empty()
    }

    #[must_use]
    pub fn contains_domain(
        &self,
        domain: &DomainId,
    ) -> bool {

        self.observations
            .keys()
            .any(|key| &key.domain == domain)
    }

    #[must_use]
    pub fn domains(&self) -> BTreeSet<DomainId> {

        self.observations
            .keys()
            .map(|key| key.domain.clone())
            .collect()
    }

    #[must_use]
    pub fn for_domain<'a>(
        &'a self,
        domain: &'a DomainId,
    ) -> impl Iterator<Item = &'a DomainObservation> {

        self.observations
            .values()
            .filter(move |observation| {
                observation.domain() == domain
            })
    }
}


// =============================================================================
// SEMANTIC REFERENCE KIND
// =============================================================================

/// Classification of an externally owned semantic fact.
///
/// This classification does not redefine the meaning of that fact.
///
/// Requirement, Capability, Constraint, Preference and Hint
/// remain distinct categories.
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

    Hint,

    Contract,

    Resource,

    Policy,

    Provenance,

    Other(String),
}


// =============================================================================
// SEMANTIC FACT REFERENCE
// =============================================================================

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
        origin.validate()?;

        Ok(Self {
            kind,
            entity,
            origin,
        })
    }

    #[must_use]
    pub fn kind(&self) -> &SemanticReferenceKind {
        &self.kind
    }

    #[must_use]
    pub fn entity(&self) -> &EntityReference {
        &self.entity
    }

    #[must_use]
    pub fn origin(&self) -> &EvidenceOrigin {
        &self.origin
    }

    pub fn validate(&self) -> ProgramAnalysisResult<()> {

        self.entity.validate()?;
        self.origin.validate()?;

        Ok(())
    }
}


// =============================================================================
// SEMANTIC FACT IDENTITY
// =============================================================================

/// Complete semantic fact identity.
///
/// Unlike the previous implementation, evidence entity information
/// is not discarded during indexing.
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

    origin: EvidenceOrigin,
}


// =============================================================================
// ANALYSIS RECORD
// =============================================================================

/// One named program-level analysis result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnalysisRecord {

    id: AnalysisId,

    completeness: AnalysisCompleteness,

    origin: Option<EvidenceOrigin>,
}

impl AnalysisRecord {

    pub fn new(
        id: AnalysisId,
        completeness: AnalysisCompleteness,
        origin: Option<EvidenceOrigin>,
    ) -> ProgramAnalysisResult<Self> {

        completeness.validate()?;

        if let Some(origin) = &origin {
            origin.validate()?;
        }

        Ok(Self {
            id,
            completeness,
            origin,
        })
    }

    #[must_use]
    pub fn id(&self) -> &AnalysisId {
        &self.id
    }

    #[must_use]
    pub fn completeness(&self) -> &AnalysisCompleteness {
        &self.completeness
    }

    #[must_use]
    pub fn origin(&self) -> Option<&EvidenceOrigin> {
        self.origin.as_ref()
    }
}


// =============================================================================
// DEPENDENCY ANALYSIS SUMMARY
// =============================================================================

/// Summary of dependency-analysis evidence.
///
/// The canonical dependency graph remains owned by:
///
/// `compiler/context/dependency_context.rs`
///
/// Dependency analysis remains owned by:
///
/// `compiler/analysis/dependency.rs`
///
/// This structure carries aggregate observations only.
/// It never stores or reconstructs the dependency graph.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DependencyAnalysisSummary {

    compilation_units: usize,

    ordering_relationships: usize,

    non_ordering_relationships: usize,

    completeness: AnalysisCompleteness,
}

impl DependencyAnalysisSummary {

    pub fn new(
        compilation_units: usize,
        ordering_relationships: usize,
        non_ordering_relationships: usize,
        completeness: AnalysisCompleteness,
    ) -> ProgramAnalysisResult<Self> {

        completeness.validate()?;

        Ok(Self {
            compilation_units,
            ordering_relationships,
            non_ordering_relationships,
            completeness,
        })
    }

    #[must_use]
    pub fn compilation_units(&self) -> usize {
        self.compilation_units
    }

    #[must_use]
    pub fn ordering_relationships(&self) -> usize {
        self.ordering_relationships
    }

    #[must_use]
    pub fn non_ordering_relationships(&self) -> usize {
        self.non_ordering_relationships
    }

    #[must_use]
    pub fn completeness(&self) -> &AnalysisCompleteness {
        &self.completeness
    }

    pub fn validate(&self) -> ProgramAnalysisResult<()> {

        self.completeness.validate()?;

        Ok(())
    }
}


// =============================================================================
// PROGRAM ANALYSIS
// =============================================================================

/// Aggregate program-level compiler analysis.
///
/// This structure is NOT:
///
/// - A source AST.
/// - A semantic model.
/// - A ZUIR program.
/// - A dependency graph.
/// - A target realization.
/// - A hardware allocation plan.
///
/// It is an index of analysis observations produced by
/// authoritative repository subsystems.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ProgramAnalysis {

    units: BTreeMap<
        CompilationUnitId,
        CompilationUnitAnalysis,
    >,

    domains: DomainIndex,

    semantic_facts: BTreeSet<SemanticFactKey>,

    analyses: BTreeMap<
        AnalysisId,
        AnalysisRecord,
    >,

    dependency_summary: Option<DependencyAnalysisSummary>,

    revision: u64,
}

impl ProgramAnalysis {

    /// Create an empty program-analysis index.
    pub fn new() -> Self {
        Self::default()
    }

    // =========================================================================
    // REVISION
    // =========================================================================

    #[must_use]
    pub fn revision(&self) -> u64 {
        self.revision
    }

    fn advance_revision(&mut self) -> ProgramAnalysisResult<()> {

        self.revision = self.revision
            .checked_add(1)
            .ok_or(
                ProgramAnalysisError::RevisionOverflow
            )?;

        Ok(())
    }

    // =========================================================================
    // COMPILATION UNITS
    // =========================================================================

    pub fn add_unit(
        &mut self,
        unit: CompilationUnitAnalysis,
    ) -> ProgramAnalysisResult<()> {

        unit.validate()?;

        let id = unit.id.clone();

        if self.units.contains_key(&id) {

            return Err(
                ProgramAnalysisError::DuplicateCompilationUnit {
                    id,
                }
            );
        }

        self.advance_revision()?;

        self.units.insert(id, unit);

        Ok(())
    }

    #[must_use]
    pub fn unit(
        &self,
        id: &CompilationUnitId,
    ) -> Option<&CompilationUnitAnalysis> {

        self.units.get(id)
    }

    #[must_use]
    pub fn units(
        &self,
    ) -> &BTreeMap<
        CompilationUnitId,
        CompilationUnitAnalysis,
    > {

        &self.units
    }

    #[must_use]
    pub fn unit_count(&self) -> usize {
        self.units.len()
    }

    // =========================================================================
    // UNIT MUTATION
    // =========================================================================

    pub fn update_unit(
        &mut self,
        id: &CompilationUnitId,
        update: impl FnOnce(
            &mut CompilationUnitAnalysis
        ) -> ProgramAnalysisResult<()>,
    ) -> ProgramAnalysisResult<()> {

        let mut candidate = self.units
            .get(id)
            .cloned()
            .ok_or_else(|| {
                ProgramAnalysisError::UnknownCompilationUnit {
                    id: id.clone(),
                }
            })?;

        update(&mut candidate)?;

        candidate.validate()?;

        if &candidate.id != id {

            return Err(
                ProgramAnalysisError::InconsistentProgram {
                    reason:
                        "unit update changed canonical identity".into(),
                }
            );
        }

        self.advance_revision()?;

        self.units.insert(
            id.clone(),
            candidate,
        );

        Ok(())
    }

    // =========================================================================
    // DOMAIN OBSERVATIONS
    // =========================================================================

    #[must_use]
    pub fn domains(&self) -> &DomainIndex {
        &self.domains
    }

    pub fn add_domain_observation(
        &mut self,
        observation: DomainObservation,
    ) -> ProgramAnalysisResult<()> {

        if let Some(unit) = observation.compilation_unit() {

            if !self.units.contains_key(unit) {

                return Err(
                    ProgramAnalysisError::UnknownCompilationUnit {
                        id: unit.clone(),
                    }
                );
            }
        }

        // Validate and check duplicates before advancing revision.
        observation.validate()?;

        let key = DomainObservationKey {
            domain: observation.domain.clone(),
            entity: observation.entity.clone(),
            compilation_unit: observation.compilation_unit.clone(),
        };

        if self.domains.observations.contains_key(&key) {

            return Err(
                ProgramAnalysisError::DuplicateDomainObservation {
                    domain: observation.domain,
                    entity: observation.entity,
                }
            );
        }

        self.advance_revision()?;

        self.domains.insert(observation)?;

        Ok(())
    }

    // =========================================================================
    // SEMANTIC FACTS
    // =========================================================================

    pub fn add_semantic_fact(
        &mut self,
        fact: SemanticFactReference,
    ) -> ProgramAnalysisResult<bool> {

        fact.validate()?;

        let key = SemanticFactKey {
            kind: fact.kind,
            entity: fact.entity,
            origin: fact.origin,
        };

        if self.semantic_facts.contains(&key) {
            return Ok(false);
        }

        self.advance_revision()?;

        self.semantic_facts.insert(key);

        Ok(true)
    }

    #[must_use]
    pub fn semantic_fact_count(&self) -> usize {
        self.semantic_facts.len()
    }

    // =========================================================================
    // ANALYSIS RECORDS
    // =========================================================================

    pub fn register_analysis(
        &mut self,
        record: AnalysisRecord,
    ) -> ProgramAnalysisResult<()> {

        let id = record.id.clone();

        if self.analyses.contains_key(&id) {

            return Err(
                ProgramAnalysisError::DuplicateAnalysis {
                    analysis: id,
                }
            );
        }

        self.advance_revision()?;

        self.analyses.insert(
            id,
            record,
        );

        Ok(())
    }

    pub fn update_analysis(
        &mut self,
        id: &AnalysisId,
        completeness: AnalysisCompleteness,
    ) -> ProgramAnalysisResult<()> {

        completeness.validate()?;

        let record = self.analyses
            .get(id)
            .ok_or_else(|| {
                ProgramAnalysisError::UnknownAnalysis {
                    analysis: id.clone(),
                }
            })?;

        let updated = AnalysisRecord {
            id: record.id.clone(),
            completeness,
            origin: record.origin.clone(),
        };

        self.advance_revision()?;

        self.analyses.insert(
            id.clone(),
            updated,
        );

        Ok(())
    }

    #[must_use]
    pub fn analysis(
        &self,
        id: &AnalysisId,
    ) -> Option<&AnalysisRecord> {

        self.analyses.get(id)
    }

    #[must_use]
    pub fn analyses(
        &self,
    ) -> &BTreeMap<AnalysisId, AnalysisRecord> {

        &self.analyses
    }

    // =========================================================================
    // DEPENDENCY SUMMARY
    // =========================================================================

    pub fn set_dependency_summary(
        &mut self,
        summary: DependencyAnalysisSummary,
    ) -> ProgramAnalysisResult<()> {

        summary.validate()?;

        self.advance_revision()?;

        self.dependency_summary = Some(summary);

        Ok(())
    }

    #[must_use]
    pub fn dependency_summary(
        &self,
    ) -> Option<&DependencyAnalysisSummary> {

        self.dependency_summary.as_ref()
    }

    // =========================================================================
    // COMPLETENESS
    // =========================================================================

    /// Returns true only when at least one analysis is registered,
    /// all registered analyses are complete, every unit is complete,
    /// and the dependency summary, if present, is complete.
    ///
    /// Missing analysis is never interpreted as successful analysis.
    #[must_use]
    pub fn all_analyses_complete(&self) -> bool {

        !self.analyses.is_empty()

            && self.analyses
                .values()
                .all(|record| {
                    record.completeness.is_complete()
                })

            && self.units
                .values()
                .all(|unit| {
                    unit.completeness.is_complete()
                })

            && self.dependency_summary
                .as_ref()
                .map(|summary| {
                    summary.completeness.is_complete()
                })
                .unwrap_or(true)
    }

    // =========================================================================
    // VALIDATION
    // =========================================================================

    /// Validate all local program-analysis invariants.
    ///
    /// This does not replace:
    ///
    /// - Frontend semantic validation.
    /// - Canonical ZUIR verification.
    /// - Dependency graph validation.
    /// - Ownership verification.
    /// - Effect verification.
    /// - Capability feasibility assessment.
    pub fn validate(&self) -> ProgramAnalysisResult<()> {

        // ---------------------------------------------------------------------
        // Compilation units
        // ---------------------------------------------------------------------

        for (id, unit) in &self.units {

            if id != unit.id() {

                return Err(
                    ProgramAnalysisError::InconsistentProgram {
                        reason:
                            "unit index does not match unit identity".into(),
                    }
                );
            }

            unit.validate()?;
        }

        // ---------------------------------------------------------------------
        // Domain observations
        // ---------------------------------------------------------------------

        for observation in self.domains.observations() {

            observation.validate()?;

            if let Some(unit) = observation.compilation_unit() {

                if !self.units.contains_key(unit) {

                    return Err(
                        ProgramAnalysisError::UnknownCompilationUnit {
                            id: unit.clone(),
                        }
                    );
                }
            }
        }

        // ---------------------------------------------------------------------
        // Semantic facts
        // ---------------------------------------------------------------------

        for fact in &self.semantic_facts {

            fact.entity.validate()?;
            fact.origin.validate()?;
        }

        // ---------------------------------------------------------------------
        // Analysis records
        // ---------------------------------------------------------------------

        for (id, record) in &self.analyses {

            if id != record.id() {

                return Err(
                    ProgramAnalysisError::InconsistentProgram {
                        reason:
                            "analysis index does not match record identity"
                                .into(),
                    }
                );
            }

            record.completeness.validate()?;

            if let Some(origin) = &record.origin {
                origin.validate()?;
            }
        }

        // ---------------------------------------------------------------------
        // Dependency summary
        // ---------------------------------------------------------------------

        if let Some(summary) = &self.dependency_summary {
            summary.validate()?;
        }

        Ok(())
    }

    // =========================================================================
    // STATISTICS
    // =========================================================================

    #[must_use]
    pub fn statistics(&self) -> ProgramAnalysisStatistics {

        ProgramAnalysisStatistics {

            compilation_units: self.units.len(),

            domain_observations: self.domains.len(),

            semantic_references: self.semantic_facts.len(),

            registered_analyses: self.analyses.len(),

            dependency_summary_present:
                self.dependency_summary.is_some(),

            revision: self.revision,
        }
    }
}


// =============================================================================
// STATISTICS
// =============================================================================

/// Snapshot of program-analysis collection sizes.
///
/// These are measurements, not capacity limits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProgramAnalysisStatistics {

    pub compilation_units: usize,

    pub domain_observations: usize,

    pub semantic_references: usize,

    pub registered_analyses: usize,

    pub dependency_summary_present: bool,

    pub revision: u64,
}


// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {

    use super::*;

    fn unit_id(
        value: &str,
    ) -> CompilationUnitId {

        CompilationUnitId::new(value).unwrap()
    }

    fn unit(
        value: &str,
    ) -> CompilationUnitAnalysis {

        CompilationUnitAnalysis::new(
            unit_id(value)
        )
    }

    fn origin() -> EvidenceOrigin {

        EvidenceOrigin::new(
            "frontend.semantic",
            "program.analysis",
            None,
        )
        .unwrap()
    }

    fn complete() -> AnalysisCompleteness {

        AnalysisCompleteness::new(
            AnalysisStatus::Complete,
            1,
            0,
            None,
        )
    }

    // -------------------------------------------------------------------------
    // IDENTITIES
    // -------------------------------------------------------------------------

    #[test]
    fn accepts_valid_domain_identifier() {

        let domain = DomainId::new(
            "future.photonic.computing"
        )
        .unwrap();

        assert_eq!(
            domain.as_str(),
            "future.photonic.computing"
        );
    }

    #[test]
    fn rejects_empty_domain_identifier() {

        assert!(
            DomainId::new("").is_err()
        );

        assert!(
            DomainId::new(" ").is_err()
        );
    }

    #[test]
    fn rejects_control_characters() {

        assert!(
            DomainId::new("quantum\ninvalid").is_err()
        );
    }

    #[test]
    fn validates_external_references() {

        assert!(
            EntityReference::new(
                "",
                "entity"
            ).is_err()
        );

        assert!(
            EntityReference::new(
                "semantic",
                ""
            ).is_err()
        );

        assert!(
            EntityReference::new(
                "semantic",
                "entity.1"
            ).is_ok()
        );
    }

    // -------------------------------------------------------------------------
    // COMPLETENESS
    // -------------------------------------------------------------------------

    #[test]
    fn incomplete_analysis_is_not_success() {

        let completeness = AnalysisCompleteness::new(
            AnalysisStatus::Incomplete,
            10,
            2,
            Some("Unresolved observations".into()),
        );

        assert!(
            !completeness.is_complete()
        );
    }

    #[test]
    fn completed_analysis_rejects_unresolved_items() {

        let completeness = AnalysisCompleteness::new(
            AnalysisStatus::Complete,
            10,
            1,
            None,
        );

        assert!(
            completeness.validate().is_err()
        );
    }

    #[test]
    fn not_started_analysis_rejects_observations() {

        let completeness = AnalysisCompleteness::new(
            AnalysisStatus::NotStarted,
            1,
            0,
            None,
        );

        assert!(
            completeness.validate().is_err()
        );
    }

    // -------------------------------------------------------------------------
    // COMPILATION UNITS
    // -------------------------------------------------------------------------

    #[test]
    fn registers_compilation_unit() {

        let mut analysis = ProgramAnalysis::new();

        analysis.add_unit(
            unit("module.application")
        ).unwrap();

        assert_eq!(
            analysis.unit_count(),
            1
        );
    }

    #[test]
    fn rejects_duplicate_compilation_unit() {

        let mut analysis = ProgramAnalysis::new();

        analysis.add_unit(
            unit("module.application")
        ).unwrap();

        assert!(
            analysis.add_unit(
                unit("module.application")
            ).is_err()
        );
    }

    #[test]
    fn updates_unit_transactionally() {

        let mut analysis = ProgramAnalysis::new();

        let id = unit_id("module.application");

        analysis.add_unit(
            unit("module.application")
        ).unwrap();

        let result = analysis.update_unit(
            &id,
            |unit| {

                unit.set_completeness(
                    complete()
                )?;

                Ok(())
            }
        );

        assert!(result.is_ok());

        assert!(
            analysis.unit(&id)
                .unwrap()
                .completeness()
                .is_complete()
        );
    }

    #[test]
    fn failed_unit_update_preserves_original() {

        let mut analysis = ProgramAnalysis::new();

        let id = unit_id("module.application");

        analysis.add_unit(
            unit("module.application")
        ).unwrap();

        let before = analysis.revision();

        let result = analysis.update_unit(
            &id,
            |_unit| {

                Err(
                    ProgramAnalysisError::InvalidObservation {
                        reason: "intentional test failure".into(),
                    }
                )
            }
        );

        assert!(result.is_err());

        assert_eq!(
            analysis.revision(),
            before
        );
    }

    // -------------------------------------------------------------------------
    // DOMAINS
    // -------------------------------------------------------------------------

    #[test]
    fn supports_extensible_domains() {

        let mut analysis = ProgramAnalysis::new();

        analysis.add_unit(
            unit("module.application")
        ).unwrap();

        let entity = EntityReference::new(
            "frontend.semantic",
            "operation.1",
        )
        .unwrap();

        let observation = DomainObservation::new(
            DomainId::new(
                "future.photonic.computing"
            ).unwrap(),

            entity,

            origin(),
        )
        .unwrap()
        .for_compilation_unit(
            unit_id("module.application")
        );

        analysis.add_domain_observation(
            observation
        ).unwrap();

        assert!(
            analysis.domains().contains_domain(
                &DomainId::new(
                    "future.photonic.computing"
                ).unwrap()
            )
        );
    }

    #[test]
    fn rejects_domain_observation_for_unknown_unit() {

        let mut analysis = ProgramAnalysis::new();

        let observation = DomainObservation::new(
            DomainId::new("classical").unwrap(),

            EntityReference::new(
                "semantic",
                "operation.1",
            ).unwrap(),

            origin(),
        )
        .unwrap()
        .for_compilation_unit(
            unit_id("missing")
        );

        assert!(
            analysis.add_domain_observation(
                observation
            ).is_err()
        );
    }

    #[test]
    fn rejects_duplicate_domain_observations() {

        let mut analysis = ProgramAnalysis::new();

        let observation = DomainObservation::new(
            DomainId::new("classical").unwrap(),

            EntityReference::new(
                "semantic",
                "operation.1",
            ).unwrap(),

            origin(),
        )
        .unwrap();

        analysis.add_domain_observation(
            observation.clone()
        ).unwrap();

        assert!(
            analysis.add_domain_observation(
                observation
            ).is_err()
        );
    }

    // -------------------------------------------------------------------------
    // SEMANTIC FACTS
    // -------------------------------------------------------------------------

    #[test]
    fn stores_semantic_fact() {

        let reference = EntityReference::new(
            "frontend.semantic",
            "capability.quantum",
        )
        .unwrap();

        let fact = SemanticFactReference::new(
            SemanticReferenceKind::Capability,
            reference,
            origin(),
        )
        .unwrap();

        let mut analysis = ProgramAnalysis::new();

        assert!(
            analysis.add_semantic_fact(fact).unwrap()
        );

        assert_eq!(
            analysis.semantic_fact_count(),
            1
        );
    }

    #[test]
    fn identifies_duplicate_semantic_fact() {

        let reference = EntityReference::new(
            "frontend.semantic",
            "type.integer",
        )
        .unwrap();

        let fact = SemanticFactReference::new(
            SemanticReferenceKind::Type,
            reference,
            origin(),
        )
        .unwrap();

        let mut analysis = ProgramAnalysis::new();

        assert!(
            analysis.add_semantic_fact(
                fact.clone()
            ).unwrap()
        );

        assert!(
            !analysis.add_semantic_fact(
                fact
            ).unwrap()
        );
    }

    // -------------------------------------------------------------------------
    // ANALYSIS RECORDS
    // -------------------------------------------------------------------------

    #[test]
    fn registers_analysis_record() {

        let mut analysis = ProgramAnalysis::new();

        let record = AnalysisRecord::new(
            AnalysisId::new("structural").unwrap(),
            complete(),
            Some(origin()),
        ).unwrap();

        analysis.register_analysis(
            record
        ).unwrap();

        assert_eq!(
            analysis.analyses().len(),
            1
        );
    }

    #[test]
    fn rejects_duplicate_analysis_record() {

        let mut analysis = ProgramAnalysis::new();

        let record = AnalysisRecord::new(
            AnalysisId::new("structural").unwrap(),
            complete(),
            None,
        ).unwrap();

        analysis.register_analysis(
            record.clone()
        ).unwrap();

        assert!(
            analysis.register_analysis(
                record
            ).is_err()
        );
    }

    #[test]
    fn incomplete_program_is_not_complete() {

        let mut analysis = ProgramAnalysis::new();

        analysis.add_unit(
            unit("main")
        ).unwrap();

        assert!(
            !analysis.all_analyses_complete()
        );
    }

    #[test]
    fn completed_program_is_complete() {

        let mut analysis = ProgramAnalysis::new();

        let mut compilation_unit = unit("main");

        compilation_unit.set_completeness(
            complete()
        ).unwrap();

        analysis.add_unit(
            compilation_unit
        ).unwrap();

        analysis.register_analysis(
            AnalysisRecord::new(
                AnalysisId::new("structural").unwrap(),
                complete(),
                None,
            ).unwrap()
        ).unwrap();

        assert!(
            analysis.validate().is_ok()
        );

        assert!(
            analysis.all_analyses_complete()
        );
    }

    // -------------------------------------------------------------------------
    // DEPENDENCY INTEGRATION
    // -------------------------------------------------------------------------

    #[test]
    fn records_dependency_analysis_summary() {

        let mut analysis = ProgramAnalysis::new();

        let summary = DependencyAnalysisSummary::new(
            10,
            8,
            4,
            complete(),
        ).unwrap();

        analysis.set_dependency_summary(
            summary
        ).unwrap();

        assert!(
            analysis.dependency_summary().is_some()
        );
    }

    // -------------------------------------------------------------------------
    // REVISION
    // -------------------------------------------------------------------------

    #[test]
    fn revision_advances_when_state_changes() {

        let mut analysis = ProgramAnalysis::new();

        let initial = analysis.revision();

        analysis.add_unit(
            unit("main")
        ).unwrap();

        assert!(
            analysis.revision() > initial
        );
    }

    // -------------------------------------------------------------------------
    // STATISTICS
    // -------------------------------------------------------------------------

    #[test]
    fn reports_program_analysis_statistics() {

        let mut analysis = ProgramAnalysis::new();

        analysis.add_unit(
            unit("main")
        ).unwrap();

        let statistics = analysis.statistics();

        assert_eq!(
            statistics.compilation_units,
            1
        );

        assert_eq!(
            statistics.domain_observations,
            0
        );

        assert_eq!(
            statistics.semantic_references,
            0
        );
    }

    // -------------------------------------------------------------------------
    // VALIDATION
    // -------------------------------------------------------------------------

    #[test]
    fn validates_empty_program_analysis() {

        let analysis = ProgramAnalysis::new();

        assert!(
            analysis.validate().is_ok()
        );

        assert!(
            !analysis.all_analyses_complete()
        );
    }
}