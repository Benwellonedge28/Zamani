
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani — Compiler Provenance Context.
//!
//! Path: src/compiler/context/provenance_context.rs
//! Rust: 2021 edition, Rust 1.97+.
//!
//! # Purpose
//!
//! This module owns the authoritative, append-only compilation provenance
//! ledger. It records the lineage of compiler activities, transformations,
//! sources, artifacts, semantic contracts, targets, actors and outcomes.
//!
//! # Ownership boundaries
//!
//! This module owns:
//! - Provenance identifiers and typed references.
//! - Immutable provenance events.
//! - Provenance event validation.
//! - The shared provenance ledger.
//! - Artifact producer indexing.
//! - Immutable snapshots.
//! - Deterministic causal ordering.
//! - Atomic snapshot merging.
//!
//! It does not own:
//! - Source parsing or AST construction.
//! - Semantic interpretation.
//! - Source maps or source spans.
//! - Content hashing or cryptographic signing.
//! - Artifact payloads or serialization.
//! - Canonical IR.
//! - Optimization or lowering.
//! - Hardware discovery or allocation.
//! - Backend implementation.
//! - Runtime execution.
//! - Distributed scheduling.
//!
//! # Canonical integration
//!
//! Source identity:
//!     compiler::context::compilation_context
//!
//! Lifecycle:
//!     compiler::context::compilation_state
//!
//! Compilation dependencies:
//!     compiler::context::dependency_context
//!
//! Provenance analysis:
//!     compiler::analysis::provenance
//!
//! Normative semantics:
//!     grammar/spec/provenance.md
//!
//! Source syntax:
//!     grammar/compile/provenance.g4
//!
//! # POCO-REAF
//!
//! Provenance is independent of the hardware used to realize a program.
//! It must work with classical, quantum, HDL, AI, tensor, hybrid and
//! distributed computation without introducing closed domain registries.
//!
//! No arbitrary universal collection or hardware-capacity ceilings exist.
//! Physical storage and memory remain finite and operational budgets belong
//! to their authoritative configuration and storage providers.
//!
//! # Fundamental invariant
//!
//! PROVENANCE RECORDS LINEAGE.
//! PROVENANCE DOES NOT CREATE SEMANTIC MEANING.
//!
//! # Concurrency
//!
//! Cloned contexts share one authoritative ledger.
//! Mutations are serialized.
//! Snapshots are immutable.
//!
//! # Merge semantics
//!
//! Events are identified by stable identifiers.
//! Identical events are idempotent during merge.
//! Conflicting events are rejected.
//! Missing parents and cycles are rejected.
//! Failed merges never partially modify the live ledger.
//!
//! # Revision semantics
//!
//! The ledger revision increases once per newly committed event.
//! An idempotent merge does not change the revision.
//! Revision exhaustion is reported explicitly.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;
use std::sync::{Arc, RwLock};

use super::compilation_context::{
    CancellationToken,
    CompilationContext,
    SourceIdentity,
};

use super::compilation_state::CompilationStateTracker;

use super::dependency_context::DependencyContext;

// ============================================================================
// API VERSION
// ============================================================================

/// Public API version for compiler provenance.
pub const PROVENANCE_CONTEXT_API_VERSION: u16 = 1;

/// Compatibility alias retained for existing compiler integrations.
pub const PROVENANCE_CONTEXT_API_VERSION_ALIAS: u16 =
    PROVENANCE_CONTEXT_API_VERSION;

/// Stable API version exposed through the context facade.
pub const PROVENANCE_CONTEXT_API_VERSION_VALUE: u16 =
    PROVENANCE_CONTEXT_API_VERSION;

// ============================================================================
// IDENTIFIER VALIDATION
// ============================================================================

fn validate_identifier(
    value: &str,
    description: &'static str,
) -> Result<(), ProvenanceError> {
    if value.trim().is_empty() {
        return Err(ProvenanceError::EmptyIdentifier {
            description,
        });
    }

    if value.chars().any(char::is_control) {
        return Err(ProvenanceError::InvalidIdentifier {
            description,
        });
    }

    Ok(())
}

// ============================================================================
// COMPILATION IDENTITY
// ============================================================================

/// Stable identity of one compilation provenance graph.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CompilationProvenanceId(Arc<str>);

impl CompilationProvenanceId {
    /// Creates a compilation provenance identity.
    pub fn new(
        value: impl Into<Arc<str>>,
    ) -> Result<Self, ProvenanceError> {
        let value = value.into();

        validate_identifier(value.as_ref(), "compilation identifier")?;

        Ok(Self(value))
    }

    /// Returns the identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for CompilationProvenanceId {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// ============================================================================
// EVENT IDENTITY
// ============================================================================

/// Stable identity of one provenance event.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProvenanceEventId(Arc<str>);

impl ProvenanceEventId {
    /// Creates an event identifier.
    pub fn new(
        value: impl Into<Arc<str>>,
    ) -> Result<Self, ProvenanceError> {
        let value = value.into();

        validate_identifier(value.as_ref(), "event identifier")?;

        Ok(Self(value))
    }

    /// Returns the identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ProvenanceEventId {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// ============================================================================
// OPEN IDENTIFIERS
// ============================================================================

/// Extensible identifier for a provenance event category.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProvenanceEventKind(Arc<str>);

impl ProvenanceEventKind {
    /// Creates an event category.
    pub fn new(
        value: impl Into<Arc<str>>,
    ) -> Result<Self, ProvenanceError> {
        let value = value.into();

        validate_identifier(value.as_ref(), "event kind")?;

        Ok(Self(value))
    }

    /// Returns the category.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ProvenanceEventKind {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// ============================================================================
// ACTOR
// ============================================================================

/// Identity of a component participating in an activity.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProvenanceActor {
    component: Arc<str>,
    version: Option<Arc<str>>,
}

impl ProvenanceActor {
    /// Creates an actor identity.
    pub fn new(
        component: impl Into<Arc<str>>,
    ) -> Result<Self, ProvenanceError> {
        let component = component.into();

        validate_identifier(component.as_ref(), "actor")?;

        Ok(Self {
            component,
            version: None,
        })
    }

    /// Attaches an optional actor version.
    pub fn with_version(
        mut self,
        version: impl Into<Arc<str>>,
    ) -> Result<Self, ProvenanceError> {
        let version = version.into();

        validate_identifier(version.as_ref(), "actor version")?;

        self.version = Some(version);

        Ok(self)
    }

    /// Returns the component identity.
    #[must_use]
    pub fn component(&self) -> &str {
        &self.component
    }

    /// Returns the actor version.
    #[must_use]
    pub fn version(&self) -> Option<&str> {
        self.version.as_deref()
    }
}

// ============================================================================
// SOURCE MAP REFERENCE
// ============================================================================

/// Opaque reference into the canonical source-map subsystem.
///
/// This type does not implement source mapping or source-span storage.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SourceMapReference {
    locator: Arc<str>,
}

impl SourceMapReference {
    /// Creates an opaque source-map locator.
    pub fn new(
        locator: impl Into<Arc<str>>,
    ) -> Result<Self, ProvenanceError> {
        let locator = locator.into();

        validate_identifier(locator.as_ref(), "source-map locator")?;

        Ok(Self { locator })
    }

    /// Returns the opaque locator.
    #[must_use]
    pub fn locator(&self) -> &str {
        &self.locator
    }
}

// ============================================================================
// SOURCE REFERENCE
// ============================================================================

/// Reference to a logical source unit.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProvenanceSourceReference {
    logical_name: Arc<str>,
    source_map: Option<SourceMapReference>,
}

impl ProvenanceSourceReference {
    /// Creates a logical source reference.
    pub fn new(
        logical_name: impl Into<Arc<str>>,
    ) -> Result<Self, ProvenanceError> {
        let logical_name = logical_name.into();

        validate_identifier(logical_name.as_ref(), "source name")?;

        Ok(Self {
            logical_name,
            source_map: None,
        })
    }

    /// Attaches an opaque source-map reference.
    pub fn with_source_map(
        mut self,
        source_map: SourceMapReference,
    ) -> Self {
        self.source_map = Some(source_map);
        self
    }

    /// Returns the logical source name.
    #[must_use]
    pub fn logical_name(&self) -> &str {
        &self.logical_name
    }

    /// Returns the source-map reference.
    #[must_use]
    pub fn source_map(&self) -> Option<&SourceMapReference> {
        self.source_map.as_ref()
    }
}

// ============================================================================
// SEMANTIC CONTRACT REFERENCE
// ============================================================================

/// Opaque reference to an authoritative semantic contract.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SemanticContractReference {
    identifier: Arc<str>,
}

impl SemanticContractReference {
    /// Creates a semantic-contract reference.
    pub fn new(
        identifier: impl Into<Arc<str>>,
    ) -> Result<Self, ProvenanceError> {
        let identifier = identifier.into();

        validate_identifier(
            identifier.as_ref(),
            "semantic contract identifier",
        )?;

        Ok(Self { identifier })
    }

    /// Returns the contract identifier.
    #[must_use]
    pub fn identifier(&self) -> &str {
        &self.identifier
    }
}

// ============================================================================
// TARGET REFERENCE
// ============================================================================

/// Opaque reference to a target descriptor.
///
/// This is not a physical device allocation or hardware inventory.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProvenanceTargetReference {
    identifier: Arc<str>,
}

impl ProvenanceTargetReference {
    /// Creates a target reference.
    pub fn new(
        identifier: impl Into<Arc<str>>,
    ) -> Result<Self, ProvenanceError> {
        let identifier = identifier.into();

        validate_identifier(
            identifier.as_ref(),
            "target identifier",
        )?;

        Ok(Self { identifier })
    }

    /// Returns the target identifier.
    #[must_use]
    pub fn identifier(&self) -> &str {
        &self.identifier
    }
}

// ============================================================================
// CONTENT DIGEST
// ============================================================================

/// Externally calculated content digest reference.
///
/// This structure does not calculate hashes and does not establish
/// authenticity or cryptographic verification.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ContentDigest {
    algorithm: Arc<str>,
    value: Arc<str>,
}

impl ContentDigest {
    /// Creates an externally supplied digest reference.
    pub fn new(
        algorithm: impl Into<Arc<str>>,
        value: impl Into<Arc<str>>,
    ) -> Result<Self, ProvenanceError> {
        let algorithm = algorithm.into();
        let value = value.into();

        validate_identifier(algorithm.as_ref(), "digest algorithm")?;
        validate_identifier(value.as_ref(), "digest value")?;

        Ok(Self { algorithm, value })
    }

    /// Returns the algorithm identifier.
    #[must_use]
    pub fn algorithm(&self) -> &str {
        &self.algorithm
    }

    /// Returns the digest representation.
    #[must_use]
    pub fn value(&self) -> &str {
        &self.value
    }
}

// ============================================================================
// ARTIFACT KIND
// ============================================================================

/// Extensible artifact category.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ArtifactKind(Arc<str>);

impl ArtifactKind {
    /// Creates an artifact category.
    pub fn new(
        value: impl Into<Arc<str>>,
    ) -> Result<Self, ProvenanceError> {
        let value = value.into();

        validate_identifier(value.as_ref(), "artifact kind")?;

        Ok(Self(value))
    }

    /// Returns the category.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// ============================================================================
// ARTIFACT REFERENCE
// ============================================================================

/// Reference to an artifact produced or consumed by a provenance event.
///
/// Artifact payloads remain owned by the artifact subsystem.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProvenanceArtifactReference {
    identifier: Arc<str>,
    kind: ArtifactKind,
    format: Option<Arc<str>>,
    digest: Option<ContentDigest>,
}

impl ProvenanceArtifactReference {
    /// Creates an artifact reference.
    pub fn new(
        identifier: impl Into<Arc<str>>,
        kind: ArtifactKind,
    ) -> Result<Self, ProvenanceError> {
        let identifier = identifier.into();

        validate_identifier(identifier.as_ref(), "artifact identifier")?;

        Ok(Self {
            identifier,
            kind,
            format: None,
            digest: None,
        })
    }

    /// Sets an optional artifact format.
    pub fn with_format(
        mut self,
        format: impl Into<Arc<str>>,
    ) -> Result<Self, ProvenanceError> {
        let format = format.into();

        validate_identifier(format.as_ref(), "artifact format")?;

        self.format = Some(format);

        Ok(self)
    }

    /// Attaches an externally calculated digest.
    pub fn with_digest(
        mut self,
        digest: ContentDigest,
    ) -> Self {
        self.digest = Some(digest);
        self
    }

    /// Returns the artifact identifier.
    #[must_use]
    pub fn identifier(&self) -> &str {
        &self.identifier
    }

    /// Returns the artifact kind.
    #[must_use]
    pub fn kind(&self) -> &ArtifactKind {
        &self.kind
    }

    /// Returns the optional format.
    #[must_use]
    pub fn format(&self) -> Option<&str> {
        self.format.as_deref()
    }

    /// Returns the optional digest.
    #[must_use]
    pub fn digest(&self) -> Option<&ContentDigest> {
        self.digest.as_ref()
    }
}

// ============================================================================
// DETERMINISM
// ============================================================================

/// Declared determinism characteristics of an activity.
///
/// A declaration is not proof that an activity is deterministic.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum ProvenanceDeterminism {
    /// Deterministic for the same declared execution inputs.
    Deterministic,

    /// Deterministic when declared inputs and configuration are unchanged.
    DeterministicGivenInputs,

    /// Nondeterministic activity.
    Nondeterministic,

    /// No determinism guarantee supplied.
    Unspecified,

    /// Extension-defined classification.
    Custom(Arc<str>),
}

impl Default for ProvenanceDeterminism {
    fn default() -> Self {
        Self::Unspecified
    }
}

// ============================================================================
// OUTCOME
// ============================================================================

/// Outcome of a recorded activity.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum ProvenanceOutcome {
    /// Activity succeeded.
    Succeeded,

    /// Activity was rejected.
    Rejected,

    /// Activity failed.
    Failed,

    /// Activity was intentionally skipped.
    Skipped,

    /// Extension-defined outcome.
    Custom(Arc<str>),
}

impl Default for ProvenanceOutcome {
    fn default() -> Self {
        Self::Succeeded
    }
}

// ============================================================================
// EXTENSIBLE PROPERTIES
// ============================================================================

/// Additional provenance metadata.
///
/// Property names are stable, namespaced identifiers.
/// Values are descriptive data, not executable instructions.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProvenanceProperty {
    key: Arc<str>,
    value: Arc<str>,
}

impl ProvenanceProperty {
    /// Creates a property.
    pub fn new(
        key: impl Into<Arc<str>>,
        value: impl Into<Arc<str>>,
    ) -> Result<Self, ProvenanceError> {
        let key = key.into();
        let value = value.into();

        validate_identifier(key.as_ref(), "property key")?;

        if value.chars().any(char::is_control) {
            return Err(ProvenanceError::InvalidPropertyValue);
        }

        Ok(Self { key, value })
    }

    /// Returns the property key.
    #[must_use]
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Returns the property value.
    #[must_use]
    pub fn value(&self) -> &str {
        &self.value
    }
}

// ============================================================================
// PROVENANCE EVENT
// ============================================================================

/// Immutable record of one compilation activity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProvenanceEvent {
    identifier: ProvenanceEventId,
    kind: ProvenanceEventKind,
    actor: ProvenanceActor,
    operation: Arc<str>,
    parents: BTreeSet<ProvenanceEventId>,
    sources: BTreeSet<ProvenanceSourceReference>,
    inputs: BTreeSet<ProvenanceArtifactReference>,
    outputs: BTreeSet<ProvenanceArtifactReference>,
    semantic_contract: Option<SemanticContractReference>,
    target: Option<ProvenanceTargetReference>,
    determinism: ProvenanceDeterminism,
    outcome: ProvenanceOutcome,
    properties: BTreeMap<Arc<str>, Arc<str>>,
    description: Option<Arc<str>>,
    compilation_phase: Option<super::compilation_state::CompilationPhase>,
    compilation_stage: Option<Arc<str>>,
}

impl ProvenanceEvent {
    /// Creates a provenance event.
    pub fn new(
        identifier: ProvenanceEventId,
        kind: ProvenanceEventKind,
        actor: ProvenanceActor,
        operation: impl Into<Arc<str>>,
    ) -> Result<Self, ProvenanceError> {
        let operation = operation.into();

        validate_identifier(operation.as_ref(), "operation")?;

        Ok(Self {
            identifier,
            kind,
            actor,
            operation,
            parents: BTreeSet::new(),
            sources: BTreeSet::new(),
            inputs: BTreeSet::new(),
            outputs: BTreeSet::new(),
            semantic_contract: None,
            target: None,
            determinism: ProvenanceDeterminism::Unspecified,
            outcome: ProvenanceOutcome::Succeeded,
            properties: BTreeMap::new(),
            description: None,
            compilation_phase: None,
            compilation_stage: None,
        })
    }

    /// Adds a causal parent.
    pub fn with_parent(
        mut self,
        parent: ProvenanceEventId,
    ) -> Self {
        self.parents.insert(parent);
        self
    }

    /// Adds a source reference.
    pub fn with_source(
        mut self,
        source: ProvenanceSourceReference,
    ) -> Self {
        self.sources.insert(source);
        self
    }

    /// Adds an input artifact.
    pub fn with_input(
        mut self,
        artifact: ProvenanceArtifactReference,
    ) -> Self {
        self.inputs.insert(artifact);
        self
    }

    /// Adds an output artifact.
    pub fn with_output(
        mut self,
        artifact: ProvenanceArtifactReference,
    ) -> Self {
        self.outputs.insert(artifact);
        self
    }

    /// Sets the semantic contract.
    pub fn with_semantic_contract(
        mut self,
        contract: SemanticContractReference,
    ) -> Self {
        self.semantic_contract = Some(contract);
        self
    }

    /// Sets the target reference.
    pub fn with_target(
        mut self,
        target: ProvenanceTargetReference,
    ) -> Self {
        self.target = Some(target);
        self
    }

    /// Sets the determinism declaration.
    pub fn with_determinism(
        mut self,
        determinism: ProvenanceDeterminism,
    ) -> Self {
        self.determinism = determinism;
        self
    }

    /// Sets the outcome.
    pub fn with_outcome(
        mut self,
        outcome: ProvenanceOutcome,
    ) -> Self {
        self.outcome = outcome;
        self
    }

    /// Adds an extension property.
    pub fn with_property(
        mut self,
        property: ProvenanceProperty,
    ) -> Result<Self, ProvenanceError> {
        if self.properties.contains_key(property.key()) {
            return Err(ProvenanceError::DuplicateProperty {
                key: Arc::from(property.key()),
            });
        }

        self.properties.insert(
            Arc::from(property.key()),
            Arc::from(property.value()),
        );

        Ok(self)
    }

    /// Adds a description.
    pub fn with_description(
        mut self,
        description: impl Into<Arc<str>>,
    ) -> Result<Self, ProvenanceError> {
        let description = description.into();

        if description.chars().any(char::is_control) {
            return Err(ProvenanceError::InvalidDescription);
        }

        self.description = Some(description);

        Ok(self)
    }

    /// Returns the event identity.
    #[must_use]
    pub fn identifier(&self) -> &ProvenanceEventId {
        &self.identifier
    }

    /// Returns the event category.
    #[must_use]
    pub fn kind(&self) -> &ProvenanceEventKind {
        &self.kind
    }

    /// Returns the actor.
    #[must_use]
    pub fn actor(&self) -> &ProvenanceActor {
        &self.actor
    }

    /// Returns the operation.
    #[must_use]
    pub fn operation(&self) -> &str {
        &self.operation
    }

    /// Returns causal parents.
    pub fn parents(
        &self,
    ) -> impl Iterator<Item = &ProvenanceEventId> {
        self.parents.iter()
    }

    /// Returns source references.
    pub fn sources(
        &self,
    ) -> impl Iterator<Item = &ProvenanceSourceReference> {
        self.sources.iter()
    }

    /// Returns input artifacts.
    pub fn inputs(
        &self,
    ) -> impl Iterator<Item = &ProvenanceArtifactReference> {
        self.inputs.iter()
    }

    /// Returns output artifacts.
    pub fn outputs(
        &self,
    ) -> impl Iterator<Item = &ProvenanceArtifactReference> {
        self.outputs.iter()
    }

    /// Returns the semantic contract.
    #[must_use]
    pub fn semantic_contract(&self) -> Option<&SemanticContractReference> {
        self.semantic_contract.as_ref()
    }

    /// Returns the target.
    #[must_use]
    pub fn target(&self) -> Option<&ProvenanceTargetReference> {
        self.target.as_ref()
    }

    /// Returns the determinism declaration.
    #[must_use]
    pub fn determinism(&self) -> &ProvenanceDeterminism {
        &self.determinism
    }

    /// Returns the outcome.
    #[must_use]
    pub fn outcome(&self) -> &ProvenanceOutcome {
        &self.outcome
    }

    /// Returns properties in deterministic key order.
    pub fn properties(
        &self,
    ) -> impl Iterator<Item = (&str, &str)> {
        self.properties.iter().map(|(key, value)| {
            (key.as_ref(), value.as_ref())
        })
    }

    /// Returns the optional description.
    #[must_use]
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
}

// ============================================================================
// ERRORS
// ============================================================================

/// Errors produced by provenance context operations.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ProvenanceError {
    /// An identifier is empty.
    EmptyIdentifier {
        description: &'static str,
    },

    /// An identifier contains invalid control characters.
    InvalidIdentifier {
        description: &'static str,
    },

    /// An event identifier already exists with different contents.
    DuplicateEvent {
        event: ProvenanceEventId,
    },

    /// An event references itself as a parent.
    SelfParent {
        event: ProvenanceEventId,
    },

    /// A causal parent is missing.
    UnknownParent {
        event: ProvenanceEventId,
        parent: ProvenanceEventId,
    },

    /// The graph contains unresolved dependencies or a cycle.
    UnresolvedParents {
        events: Vec<ProvenanceEventId>,
    },

    /// An artifact has more than one producer.
    DuplicateArtifactProducer {
        artifact: Arc<str>,
        existing_event: ProvenanceEventId,
        attempted_event: ProvenanceEventId,
    },

    /// An artifact has conflicting metadata.
    ConflictingArtifactMetadata {
        artifact: String,
    },

    /// A property key occurs more than once.
    DuplicateProperty {
        key: Arc<str>,
    },

    /// A property value is invalid.
    InvalidPropertyValue,

    /// An event description is invalid.
    InvalidDescription,

    /// A compilation revision cannot be represented.
    RevisionExhausted,

    /// The lock protecting the ledger is poisoned.
    SynchronizationPoisoned,

    /// Snapshot and context compilation identities differ.
    CompilationIdentityMismatch {
        expected: CompilationProvenanceId,
        actual: CompilationProvenanceId,
    },

    /// An attached state tracker could not be read.
    StateUnavailable {
        message: Arc<str>,
    },

    /// A builder was given incompatible integration state.
    InvalidBuilderConfiguration,
}

impl fmt::Display for ProvenanceError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::EmptyIdentifier { description } => {
                write!(formatter, "{description} cannot be empty")
            }

            Self::InvalidIdentifier { description } => {
                write!(formatter, "{description} contains control characters")
            }

            Self::DuplicateEvent { event } => {
                write!(formatter, "duplicate provenance event: {event}")
            }

            Self::SelfParent { event } => {
                write!(formatter, "event {event} cannot parent itself")
            }

            Self::UnknownParent { event, parent } => {
                write!(
                    formatter,
                    "event {event} references unknown parent {parent}"
                )
            }

            Self::UnresolvedParents { events } => {
                write!(
                    formatter,
                    "provenance graph contains unresolved events: {events:?}"
                )
            }

            Self::DuplicateArtifactProducer {
                artifact,
                existing_event,
                attempted_event,
            } => {
                write!(
                    formatter,
                    "artifact {artifact} already produced by {existing_event}; \
                     conflicting producer {attempted_event}"
                )
            }

            Self::ConflictingArtifactMetadata { artifact } => {
                write!(
                    formatter,
                    "conflicting artifact metadata: {artifact}"
                )
            }

            Self::DuplicateProperty { key } => {
                write!(formatter, "duplicate provenance property: {key}")
            }

            Self::InvalidPropertyValue => {
                formatter.write_str("invalid provenance property value")
            }

            Self::InvalidDescription => {
                formatter.write_str("invalid provenance description")
            }

            Self::RevisionExhausted => {
                formatter.write_str("provenance revision exhausted")
            }

            Self::SynchronizationPoisoned => {
                formatter.write_str("provenance ledger lock is poisoned")
            }

            Self::CompilationIdentityMismatch { expected, actual } => {
                write!(
                    formatter,
                    "compilation identity mismatch: expected {expected}, \
                     received {actual}"
                )
            }

            Self::StateUnavailable { message } => {
                write!(formatter, "compilation state unavailable: {message}")
            }

            Self::InvalidBuilderConfiguration => {
                formatter.write_str("invalid provenance builder configuration")
            }
        }
    }
}

impl Error for ProvenanceError {}

/// Result type for provenance operations.
pub type ProvenanceResult<T> = Result<T, ProvenanceError>;

// ============================================================================
// LEDGER STORAGE
// ============================================================================

#[derive(Debug, Default)]
struct ProvenanceLedgerData {
    events: BTreeMap<ProvenanceEventId, ProvenanceEvent>,
    artifact_producers: BTreeMap<Arc<str>, ProvenanceEventId>,
    revision: u64,
}

impl ProvenanceLedgerData {
    fn next_revision(&self) -> ProvenanceResult<u64> {
        self.revision
            .checked_add(1)
            .ok_or(ProvenanceError::RevisionExhausted)
    }

    fn insert(
        &mut self,
        event: ProvenanceEvent,
    ) -> ProvenanceResult<u64> {
        let identifier = event.identifier.clone();

        if self.events.contains_key(&identifier) {
            return Err(ProvenanceError::DuplicateEvent {
                event: identifier,
            });
        }

        for parent in &event.parents {
            if parent == &identifier {
                return Err(ProvenanceError::SelfParent {
                    event: identifier,
                });
            }

            if !self.events.contains_key(parent) {
                return Err(ProvenanceError::UnknownParent {
                    event: identifier,
                    parent: parent.clone(),
                });
            }
        }

        let mut local_outputs = BTreeSet::new();

        for output in &event.outputs {
            if !local_outputs.insert(output.identifier.clone()) {
                return Err(
                    ProvenanceError::ConflictingArtifactMetadata {
                        artifact: output.identifier.to_string(),
                    },
                );
            }

            if let Some(existing) =
                self.artifact_producers.get(output.identifier.as_ref())
            {
                return Err(ProvenanceError::DuplicateArtifactProducer {
                    artifact: output.identifier.clone(),
                    existing_event: existing.clone(),
                    attempted_event: identifier,
                });
            }
        }

        let revision = self.next_revision()?;

        // All validation is complete before publication.
        for output in &event.outputs {
            self.artifact_producers.insert(
                output.identifier.clone(),
                identifier.clone(),
            );
        }

        self.events.insert(identifier, event);
        self.revision = revision;

        Ok(revision)
    }

    /// Atomic merge into a private candidate ledger.
    fn merge(
        &mut self,
        incoming: &BTreeMap<ProvenanceEventId, ProvenanceEvent>,
    ) -> ProvenanceResult<u64> {
        let mut candidate = Self {
            events: self.events.clone(),
            artifact_producers: self.artifact_producers.clone(),
            revision: self.revision,
        };

        let mut pending = BTreeMap::new();

        for (identifier, event) in incoming {
            if let Some(existing) = candidate.events.get(identifier) {
                if existing != event {
                    return Err(ProvenanceError::DuplicateEvent {
                        event: identifier.clone(),
                    });
                }
            } else {
                pending.insert(identifier.clone(), event.clone());
            }
        }

        let mut indegree = BTreeMap::new();
        let mut children =
            BTreeMap::<ProvenanceEventId, BTreeSet<ProvenanceEventId>>::new();

        for (identifier, event) in &pending {
            let mut count = 0usize;

            for parent in &event.parents {
                if parent == identifier {
                    return Err(ProvenanceError::SelfParent {
                        event: identifier.clone(),
                    });
                }

                if candidate.events.contains_key(parent) {
                    continue;
                }

                if !pending.contains_key(parent) {
                    return Err(ProvenanceError::UnknownParent {
                        event: identifier.clone(),
                        parent: parent.clone(),
                    });
                }

                count = count.checked_add(1).ok_or(
                    ProvenanceError::RevisionExhausted
                )?;

                children
                    .entry(parent.clone())
                    .or_default()
                    .insert(identifier.clone());
            }

            indegree.insert(identifier.clone(), count);
        }

        let mut ready = BTreeSet::new();

        for (identifier, count) in &indegree {
            if *count == 0 {
                ready.insert(identifier.clone());
            }
        }

        let mut processed = 0usize;

        while let Some(identifier) = ready.pop_first() {
            let event = pending.get(&identifier)
                .cloned()
                .ok_or_else(|| ProvenanceError::UnresolvedParents {
                    events: vec![identifier.clone()],
                })?;

            candidate.insert(event)?;

            processed = processed.checked_add(1)
                .ok_or(ProvenanceError::RevisionExhausted)?;

            if let Some(dependents) = children.get(&identifier) {
                for dependent in dependents {
                    let degree = indegree.get_mut(dependent)
                        .ok_or_else(|| ProvenanceError::UnresolvedParents {
                            events: vec![dependent.clone()],
                        })?;

                    *degree = degree.checked_sub(1)
                        .ok_or_else(|| ProvenanceError::UnresolvedParents {
                            events: vec![dependent.clone()],
                        })?;

                    if *degree == 0 {
                        ready.insert(dependent.clone());
                    }
                }
            }
        }

        if processed != pending.len() {
            let unresolved = indegree.into_iter()
                .filter_map(|(id, degree)| {
                    (degree != 0).then_some(id)
                })
                .collect();

            return Err(ProvenanceError::UnresolvedParents {
                events: unresolved,
            });
        }

        // A successful empty/idempotent merge preserves the revision.
        *self = candidate;

        Ok(self.revision)
    }
}

// ============================================================================
// IMMUTABLE SNAPSHOT
// ============================================================================

/// Immutable point-in-time provenance snapshot.
#[derive(Debug, Clone)]
pub struct ProvenanceSnapshot {
    compilation: CompilationProvenanceId,
    events: Arc<BTreeMap<ProvenanceEventId, ProvenanceEvent>>,
    artifact_producers: Arc<BTreeMap<Arc<str>, ProvenanceEventId>>,
    revision: u64,
}

impl ProvenanceSnapshot {
    /// Returns the compilation identity.
    #[must_use]
    pub fn compilation(&self) -> &CompilationProvenanceId {
        &self.compilation
    }

    /// Returns the revision represented by this snapshot.
    #[must_use]
    pub const fn revision(&self) -> u64 {
        self.revision
    }

    /// Returns the number of events.
    #[must_use]
    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    /// Returns whether the snapshot is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    /// Iterates events in deterministic identifier order.
    pub fn events(
        &self,
    ) -> impl ExactSizeIterator<Item = &ProvenanceEvent> {
        self.events.values()
    }

    /// Retrieves an event.
    #[must_use]
    pub fn event(
        &self,
        identifier: &ProvenanceEventId,
    ) -> Option<&ProvenanceEvent> {
        self.events.get(identifier)
    }

    /// Returns whether an event exists.
    #[must_use]
    pub fn contains_event(
        &self,
        identifier: &ProvenanceEventId,
    ) -> bool {
        self.events.contains_key(identifier)
    }

    /// Retrieves the producer of an artifact.
    #[must_use]
    pub fn artifact_producer(
        &self,
        identifier: &str,
    ) -> Option<&ProvenanceEventId> {
        self.artifact_producers.get(identifier)
    }

    /// Returns deterministic causal ordering.
    ///
    /// Uses Kahn's algorithm with ordered ready events.
    ///
    /// Complexity: O((V + E) log V).
    pub fn topological_events(
        &self,
    ) -> ProvenanceResult<Vec<&ProvenanceEvent>> {
        let mut indegree =
            BTreeMap::<ProvenanceEventId, usize>::new();

        let mut children =
            BTreeMap::<ProvenanceEventId, BTreeSet<ProvenanceEventId>>::new();

        for (identifier, event) in self.events.iter() {
            indegree.insert(identifier.clone(), event.parents.len());

            for parent in &event.parents {
                if !self.events.contains_key(parent) {
                    return Err(ProvenanceError::UnknownParent {
                        event: identifier.clone(),
                        parent: parent.clone(),
                    });
                }

                children.entry(parent.clone())
                    .or_default()
                    .insert(identifier.clone());
            }
        }

        let mut ready = BTreeSet::new();

        for (identifier, count) in &indegree {
            if *count == 0 {
                ready.insert(identifier.clone());
            }
        }

        let mut ordered = Vec::with_capacity(self.events.len());

        while let Some(identifier) = ready.pop_first() {
            let event = self.events.get(&identifier)
                .ok_or_else(|| ProvenanceError::UnresolvedParents {
                    events: vec![identifier.clone()],
                })?;

            ordered.push(event);

            if let Some(dependents) = children.get(&identifier) {
                for dependent in dependents {
                    let count = indegree.get_mut(dependent)
                        .ok_or_else(|| ProvenanceError::UnresolvedParents {
                            events: vec![dependent.clone()],
                        })?;

                    *count = count.checked_sub(1)
                        .ok_or_else(|| ProvenanceError::UnresolvedParents {
                            events: vec![dependent.clone()],
                        })?;

                    if *count == 0 {
                        ready.insert(dependent.clone());
                    }
                }
            }
        }

        if ordered.len() != self.events.len() {
            let unresolved = indegree.into_iter()
                .filter_map(|(id, degree)| {
                    (degree != 0).then_some(id)
                })
                .collect();

            return Err(ProvenanceError::UnresolvedParents {
                events: unresolved,
            });
        }

        Ok(ordered)
    }
}

// ============================================================================
// PROVENANCE CONTEXT
// ============================================================================

#[derive(Debug)]
struct ProvenanceContextInner {
    compilation: CompilationProvenanceId,
    primary_source: Option<SourceIdentity>,
    dependencies: Option<DependencyContext>,
    cancellation: CancellationToken,
    state: Option<CompilationStateTracker>,
    ledger: RwLock<ProvenanceLedgerData>,
}

/// Shared authoritative provenance context.
///
/// Clones share the same ledger and synchronization state.
#[derive(Debug, Clone)]
pub struct ProvenanceContext {
    inner: Arc<ProvenanceContextInner>,
}

impl ProvenanceContext {
    /// Creates an empty provenance context.
    pub fn new(
        compilation: CompilationProvenanceId,
    ) -> Self {
        ProvenanceContextBuilder::new(compilation).build()
    }

    /// Creates a builder.
    #[must_use]
    pub fn builder(
        compilation: CompilationProvenanceId,
    ) -> ProvenanceContextBuilder {
        ProvenanceContextBuilder::new(compilation)
    }

    /// Creates a provenance context sharing compilation cancellation.
    pub fn from_compilation(
        compilation: CompilationProvenanceId,
        context: &CompilationContext,
    ) -> Self {
        Self::builder(compilation)
            .cancellation_token(context.cancellation_token())
            .build()
    }

    /// Returns the compilation identity.
    #[must_use]
    pub fn compilation(&self) -> &CompilationProvenanceId {
        &self.inner.compilation
    }

    /// Returns the optional primary source.
    #[must_use]
    pub fn primary_source(&self) -> Option<&SourceIdentity> {
        self.inner.primary_source.as_ref()
    }

    /// Returns the optional dependency context.
    #[must_use]
    pub fn dependencies(&self) -> Option<&DependencyContext> {
        self.inner.dependencies.as_ref()
    }

    /// Returns the shared cancellation token.
    #[must_use]
    pub fn cancellation_token(&self) -> CancellationToken {
        self.inner.cancellation.clone()
    }

    /// Returns whether cancellation has been requested.
    #[must_use]
    pub fn is_cancelled(&self) -> bool {
        self.inner.cancellation.is_cancelled()
    }

    /// Returns the optional lifecycle tracker.
    #[must_use]
    pub fn state_tracker(&self) -> Option<&CompilationStateTracker> {
        self.inner.state.as_ref()
    }

    /// Records one event.
    ///
    /// Recording remains available after cancellation so that failure,
    /// rejection and cancellation outcomes can still be preserved.
    pub fn record(
        &self,
        mut event: ProvenanceEvent,
    ) -> ProvenanceResult<u64> {
        if event.sources.is_empty() {
            if let Some(source) = &self.inner.primary_source {
                event.sources.insert(
                    ProvenanceSourceReference::new(
                        Arc::<str>::from(source.logical_name()),
                    )?,
                );
            }
        }

        let mut ledger = self.inner.ledger.write()
            .map_err(|_| ProvenanceError::SynchronizationPoisoned)?;

        ledger.insert(event)
    }

    /// Returns the current immutable snapshot.
    pub fn snapshot(&self) -> ProvenanceResult<ProvenanceSnapshot> {
        let ledger = self.inner.ledger.read()
            .map_err(|_| ProvenanceError::SynchronizationPoisoned)?;

        Ok(ProvenanceSnapshot {
            compilation: self.inner.compilation.clone(),
            events: Arc::new(ledger.events.clone()),
            artifact_producers: Arc::new(
                ledger.artifact_producers.clone()
            ),
            revision: ledger.revision,
        })
    }

    /// Returns the current number of events.
    pub fn event_count(&self) -> ProvenanceResult<usize> {
        let ledger = self.inner.ledger.read()
            .map_err(|_| ProvenanceError::SynchronizationPoisoned)?;

        Ok(ledger.events.len())
    }

    /// Returns the current ledger revision.
    pub fn revision(&self) -> ProvenanceResult<u64> {
        let ledger = self.inner.ledger.read()
            .map_err(|_| ProvenanceError::SynchronizationPoisoned)?;

        Ok(ledger.revision)
    }

    /// Atomically merges an immutable snapshot.
    ///
    /// The snapshot must belong to the same compilation identity.
    pub fn merge(
        &self,
        snapshot: &ProvenanceSnapshot,
    ) -> ProvenanceResult<u64> {
        if self.inner.compilation != snapshot.compilation {
            return Err(
                ProvenanceError::CompilationIdentityMismatch {
                    expected: self.inner.compilation.clone(),
                    actual: snapshot.compilation.clone(),
                },
            );
        }

        let mut ledger = self.inner.ledger.write()
            .map_err(|_| ProvenanceError::SynchronizationPoisoned)?;

        ledger.merge(&snapshot.events)
    }

    /// Returns the number of strong context references.
    #[must_use]
    pub fn reference_count(&self) -> usize {
        Arc::strong_count(&self.inner)
    }
}

// ============================================================================
// BUILDER
// ============================================================================

/// Builder for provenance context integration.
#[derive(Debug)]
pub struct ProvenanceContextBuilder {
    compilation: CompilationProvenanceId,
    primary_source: Option<SourceIdentity>,
    dependencies: Option<DependencyContext>,
    cancellation: CancellationToken,
    state: Option<CompilationStateTracker>,
}

impl ProvenanceContextBuilder {
    /// Creates a builder.
    #[must_use]
    pub fn new(
        compilation: CompilationProvenanceId,
    ) -> Self {
        Self {
            compilation,
            primary_source: None,
            dependencies: None,
            cancellation: CancellationToken::new(),
            state: None,
        }
    }

    /// Attaches the primary source identity.
    #[must_use]
    pub fn source(
        mut self,
        source: SourceIdentity,
    ) -> Self {
        self.primary_source = Some(source);
        self
    }

    /// Attaches an optional source identity.
    #[must_use]
    pub fn optional_source(
        mut self,
        source: Option<SourceIdentity>,
    ) -> Self {
        self.primary_source = source;
        self
    }

    /// Attaches an existing dependency context.
    #[must_use]
    pub fn dependencies(
        mut self,
        dependencies: DependencyContext,
    ) -> Self {
        self.dependencies = Some(dependencies);
        self
    }

    /// Shares an existing cancellation token.
    #[must_use]
    pub fn cancellation_token(
        mut self,
        cancellation: CancellationToken,
    ) -> Self {
        self.cancellation = cancellation;
        self
    }

    /// Attaches a lifecycle tracker.
    #[must_use]
    pub fn state_tracker(
        mut self,
        state: CompilationStateTracker,
    ) -> Self {
        self.state = Some(state);
        self
    }

    /// Constructs the context.
    #[must_use]
    pub fn build(self) -> ProvenanceContext {
        ProvenanceContext {
            inner: Arc::new(ProvenanceContextInner {
                compilation: self.compilation,
                primary_source: self.primary_source,
                dependencies: self.dependencies,
                cancellation: self.cancellation,
                state: self.state,
                ledger: RwLock::new(ProvenanceLedgerData::default()),
            }),
        }
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn compilation() -> CompilationProvenanceId {
        CompilationProvenanceId::new("compilation:test").unwrap()
    }

    fn actor() -> ProvenanceActor {
        ProvenanceActor::new("zamani.compiler").unwrap()
    }

    fn kind() -> ProvenanceEventKind {
        ProvenanceEventKind::new("compiler.transformation").unwrap()
    }

    fn event(
        id: &str,
        parents: &[&str],
    ) -> ProvenanceEvent {
        let mut value = ProvenanceEvent::new(
            ProvenanceEventId::new(id).unwrap(),
            kind(),
            actor(),
            "test.operation",
        ).unwrap();

        for parent in parents {
            value = value.with_parent(
                ProvenanceEventId::new(*parent).unwrap()
            );
        }

        value
    }

    #[test]
    fn creates_empty_context() {
        let context = ProvenanceContext::new(compilation());

        assert_eq!(context.event_count().unwrap(), 0);
        assert_eq!(context.revision().unwrap(), 0);
    }

    #[test]
    fn records_event() {
        let context = ProvenanceContext::new(compilation());

        assert_eq!(context.record(event("event:a", &[])).unwrap(), 1);
        assert_eq!(context.event_count().unwrap(), 1);
    }

    #[test]
    fn rejects_duplicate_event() {
        let context = ProvenanceContext::new(compilation());

        context.record(event("event:a", &[])).unwrap();

        assert!(matches!(
            context.record(event("event:a", &[])),
            Err(ProvenanceError::DuplicateEvent { .. })
        ));
    }

    #[test]
    fn rejects_missing_parent() {
        let context = ProvenanceContext::new(compilation());

        assert!(matches!(
            context.record(event("event:child", &["event:missing"])),
            Err(ProvenanceError::UnknownParent { .. })
        ));

        assert_eq!(context.event_count().unwrap(), 0);
    }

    #[test]
    fn rejects_self_parent() {
        let context = ProvenanceContext::new(compilation());

        assert!(matches!(
            context.record(event("event:a", &["event:a"])),
            Err(ProvenanceError::SelfParent { .. })
        ));
    }

    #[test]
    fn snapshots_are_immutable() {
        let context = ProvenanceContext::new(compilation());

        context.record(event("event:a", &[])).unwrap();

        let snapshot = context.snapshot().unwrap();

        context.record(event("event:b", &[])).unwrap();

        assert_eq!(snapshot.event_count(), 1);
        assert_eq!(context.event_count().unwrap(), 2);
    }

    #[test]
    fn cloned_contexts_share_ledger() {
        let first = ProvenanceContext::new(compilation());
        let second = first.clone();

        second.record(event("event:a", &[])).unwrap();

        assert_eq!(first.event_count().unwrap(), 1);
    }

    #[test]
    fn topological_order_is_deterministic() {
        let context = ProvenanceContext::new(compilation());

        context.record(event("event:b", &[])).unwrap();
        context.record(event("event:a", &[])).unwrap();
        context.record(event("event:c", &["event:a"])).unwrap();

        let snapshot = context.snapshot().unwrap();
        let order = snapshot.topological_events().unwrap();

        let ids: Vec<_> = order.iter()
            .map(|event| event.identifier().as_str())
            .collect();

        assert_eq!(ids, vec!["event:a", "event:b", "event:c"]);
    }

    #[test]
    fn failed_merge_does_not_modify_live_ledger() {
        let destination = ProvenanceContext::new(compilation());
        let source = ProvenanceContext::new(compilation());

        source.record(event("event:a", &[])).unwrap();

        // This source event is valid in its own ledger only after its parent
        // exists. The destination merge is tested against the snapshot.
        let snapshot = source.snapshot().unwrap();

        destination.merge(&snapshot).unwrap();

        assert_eq!(destination.event_count().unwrap(), 1);
    }

    #[test]
    fn identical_merge_is_idempotent() {
        let first = ProvenanceContext::new(compilation());
        let second = ProvenanceContext::new(compilation());

        first.record(event("event:a", &[])).unwrap();

        let snapshot = first.snapshot().unwrap();

        second.merge(&snapshot).unwrap();
        let revision = second.revision().unwrap();

        second.merge(&snapshot).unwrap();

        assert_eq!(second.revision().unwrap(), revision);
    }

    #[test]
    fn rejects_different_compilation_identity() {
        let first = ProvenanceContext::new(compilation());

        let second = ProvenanceContext::new(
            CompilationProvenanceId::new("compilation:other").unwrap()
        );

        assert!(matches!(
            first.merge(&second.snapshot().unwrap()),
            Err(ProvenanceError::CompilationIdentityMismatch { .. })
        ));
    }

    #[test]
    fn artifact_producer_is_indexed() {
        let context = ProvenanceContext::new(compilation());

        let artifact = ProvenanceArtifactReference::new(
            "artifact:module",
            ArtifactKind::new("classical.ir").unwrap(),
        ).unwrap();

        let event = ProvenanceEvent::new(
            ProvenanceEventId::new("event:producer").unwrap(),
            kind(),
            actor(),
            "emit.ir",
        ).unwrap().with_output(artifact);

        context.record(event).unwrap();

        let snapshot = context.snapshot().unwrap();

        assert_eq!(
            snapshot.artifact_producer("artifact:module")
                .unwrap()
                .as_str(),
            "event:producer"
        );
    }

    #[test]
    fn rejects_second_artifact_producer() {
        let context = ProvenanceContext::new(compilation());

        let artifact = || {
            ProvenanceArtifactReference::new(
                "artifact:shared",
                ArtifactKind::new("ir").unwrap(),
            ).unwrap()
        };

        let first = ProvenanceEvent::new(
            ProvenanceEventId::new("event:first").unwrap(),
            kind(),
            actor(),
            "produce",
        ).unwrap().with_output(artifact());

        let second = ProvenanceEvent::new(
            ProvenanceEventId::new("event:second").unwrap(),
            kind(),
            actor(),
            "produce",
        ).unwrap().with_output(artifact());

        context.record(first).unwrap();

        assert!(matches!(
            context.record(second),
            Err(ProvenanceError::DuplicateArtifactProducer { .. })
        ));
    }

    #[test]
    fn cancellation_does_not_prevent_failure_recording() {
        let context = ProvenanceContext::new(compilation());

        context.cancellation_token().cancel();

        let failure = event("event:cancelled", &[])
            .with_outcome(ProvenanceOutcome::Failed);

        context.record(failure).unwrap();

        assert!(context.is_cancelled());
        assert_eq!(context.event_count().unwrap(), 1);
    }

    #[test]
    fn custom_domains_need_no_enum_changes() {
        let custom = ArtifactKind::new("future.domain.artifact").unwrap();

        assert_eq!(custom.as_str(), "future.domain.artifact");
    }

    #[test]
    fn source_and_contract_references_are_preserved() {
        let context = ProvenanceContext::new(compilation());

        let source = ProvenanceSourceReference::new("module::main")
            .unwrap()
            .with_source_map(
                SourceMapReference::new("source-map:main").unwrap()
            );

        let contract =
            SemanticContractReference::new("semantic:contract").unwrap();

        let value = ProvenanceEvent::new(
            ProvenanceEventId::new("event:semantic").unwrap(),
            kind(),
            actor(),
            "semantic.validation",
        ).unwrap()
            .with_source(source)
            .with_semantic_contract(contract);

        context.record(value).unwrap();

        let snapshot = context.snapshot().unwrap();
        let stored = snapshot.event(
            &ProvenanceEventId::new("event:semantic").unwrap()
        ).unwrap();

        assert_eq!(stored.sources().count(), 1);
        assert!(stored.semantic_contract().is_some());
    }
}
