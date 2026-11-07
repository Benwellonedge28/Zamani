
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Compiler Provenance Analysis
//!
//! Path:
//!     src/compiler/analysis/provenance.rs
//!
//! Rust edition:
//!     2021
//!
//! Minimum Rust version:
//!     1.97
//!
//! Safety:
//!     Unsafe Rust is forbidden.
//!
//! # Purpose
//!
//! This module provides compiler-facing analysis of the canonical
//! compilation provenance ledger.
//!
//! It analyzes:
//!
//! - Compilation event lineage.
//! - Source-to-artifact relationships.
//! - Transformation history.
//! - Input and output artifact relationships.
//! - Semantic contract references.
//! - Target realization references.
//! - Compiler and extension actors.
//! - Determinism declarations.
//! - Success, rejection, failure and skipped outcomes.
//! - Provenance completeness and consistency.
//! - Parallel compilation lineage.
//!
//! # Canonical authority
//!
//! The authoritative provenance implementation is:
//!
//!     crate::compiler::context::provenance_context
//!
//! This module consumes:
//!
//!     ProvenanceContext
//!     ProvenanceSnapshot
//!     ProvenanceEvent
//!     ProvenanceEventId
//!     ProvenanceArtifactReference
//!
//! It MUST NOT introduce another mutable provenance ledger.
//!
//! # Ownership
//!
//! This module owns:
//!
//! - Provenance analysis algorithms.
//! - Immutable analysis reports.
//! - Deterministic event ordering.
//! - Artifact lineage indexes.
//! - Source lineage indexes.
//! - Provenance statistics.
//! - Local provenance consistency checks.
//!
//! It does NOT own:
//!
//! - Provenance event creation authority.
//! - Provenance ledger storage.
//! - Source-map storage.
//! - Source spans.
//! - Semantic identities.
//! - Semantic contracts.
//! - Canonical IR.
//! - Content hashing.
//! - Cryptographic signatures.
//! - Artifact serialization.
//! - Hardware discovery.
//! - Physical resource allocation.
//! - Backend implementation.
//! - Runtime execution.
//!
//! # Determinism
//!
//! All indexes and report collections use ordered collections.
//!
//! Topological ordering uses stable event identifiers as tie-breakers.
//!
//! Wall-clock timestamps and worker completion order are not used
//! to establish causal relationships.
//!
//! # Scalability
//!
//! No arbitrary maximum is imposed on:
//!
//! - Provenance events.
//! - Source references.
//! - Artifacts.
//! - Transformation records.
//! - Computing domains.
//! - Compiler providers.
//! - Distributed participants.
//! - Metadata properties.
//!
//! Collection sizes are limited by available memory, address space,
//! and explicitly configured operational budgets.
//!
//! Resource exhaustion MUST produce an explicit failure.
//!
//! Literal infinite storage or processing capacity is not promised.
//!
//! # Integration
//!
//! Parent:
//!
//!     crate::compiler::analysis
//!
//! Authoritative provenance:
//!
//!     crate::compiler::context::provenance_context
//!
//! Downstream consumers:
//!
//!     crate::compiler::diagnostics
//!     crate::compiler::optimization
//!     crate::compiler::artifacts
//!     crate::compiler::pipeline
//!     crate::compiler::security
//!
//! Source and semantic authorities remain in the frontend.
//!
//! Canonical IR authorities remain in their respective domains.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use crate::compiler::context::{
    CompilationProvenanceId,
    ContentDigest,
    ProvenanceArtifactReference,
    ProvenanceContext,
    ProvenanceDeterminism,
    ProvenanceEvent,
    ProvenanceEventId,
    ProvenanceOutcome,
    ProvenanceSnapshot,
    ProvenanceError as ContextProvenanceError,
};

// ============================================================================
// API VERSION
// ============================================================================

/// Public API version of compiler provenance analysis.
///
/// This version is independent of:
///
/// - Zamani language version.
/// - Grammar version.
/// - Semantic-model version.
/// - Provenance-context API version.
/// - ZUIR version.
/// - Domain IR versions.
/// - Backend ABI versions.
///
/// Increment when the public analysis contract changes incompatibly.
pub const PROVENANCE_ANALYSIS_API_VERSION: u16 = 1;

// ============================================================================
// ANALYSIS ERRORS
// ============================================================================

/// Errors produced by provenance analysis.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ProvenanceAnalysisError {
    /// The authoritative provenance context failed.
    Context(ContextProvenanceError),

    /// An event references a parent absent from the snapshot.
    UnknownParent {
        /// Event containing the invalid relationship.
        event: ProvenanceEventId,

        /// Missing parent event.
        parent: ProvenanceEventId,
    },

    /// A provenance event graph contains a cycle.
    CycleDetected,

    /// The same artifact identifier has inconsistent metadata.
    ///
    /// Artifact identity and metadata are checked independently of
    /// the artifact subsystem's serialization and integrity checks.
    ConflictingArtifactMetadata {
        /// Conflicting artifact identifier.
        artifact: String,
    },

    /// An analysis counter cannot represent its result.
    CounterOverflow {
        /// Counter that could not be incremented.
        counter: &'static str,
    },
}

impl fmt::Display for ProvenanceAnalysisError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Context(error) => {
                write!(
                    formatter,
                    "provenance context failure: {error}"
                )
            }

            Self::UnknownParent { event, parent } => {
                write!(
                    formatter,
                    "provenance event '{event}' references missing parent '{parent}'"
                )
            }

            Self::CycleDetected => {
                formatter.write_str(
                    "provenance event relationships contain a cycle"
                )
            }

            Self::ConflictingArtifactMetadata { artifact } => {
                write!(
                    formatter,
                    "conflicting provenance metadata for artifact '{artifact}'"
                )
            }

            Self::CounterOverflow { counter } => {
                write!(
                    formatter,
                    "provenance analysis counter overflow: {counter}"
                )
            }
        }
    }
}

impl Error for ProvenanceAnalysisError {
    fn source(
        &self,
    ) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Context(error) => Some(error),
            _ => None,
        }
    }
}

impl From<ContextProvenanceError> for ProvenanceAnalysisError {
    fn from(error: ContextProvenanceError) -> Self {
        Self::Context(error)
    }
}

/// Result type for provenance analysis operations.
pub type ProvenanceAnalysisResult<T> =
    Result<T, ProvenanceAnalysisError>;

// ============================================================================
// EVENT OUTCOME CLASSIFICATION
// ============================================================================

/// Compiler-facing classification of a provenance event outcome.
///
/// The authoritative outcome remains in ProvenanceEvent.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum EventOutcomeClass {
    /// Operation succeeded.
    Succeeded,

    /// Operation was rejected.
    Rejected,

    /// Operation failed.
    Failed,

    /// Operation was intentionally skipped.
    Skipped,

    /// An extension-defined outcome.
    Custom(String),

    /// An outcome introduced by a future context implementation.
    Other,
}

impl EventOutcomeClass {
    fn from_outcome(
        outcome: &ProvenanceOutcome,
    ) -> Self {
        match outcome {
            ProvenanceOutcome::Succeeded => Self::Succeeded,
            ProvenanceOutcome::Rejected => Self::Rejected,
            ProvenanceOutcome::Failed => Self::Failed,
            ProvenanceOutcome::Skipped => Self::Skipped,
            ProvenanceOutcome::Custom(value) => {
                Self::Custom(value.to_string())
            }
            _ => Self::Other,
        }
    }
}

// ============================================================================
// DETERMINISM CLASSIFICATION
// ============================================================================

/// Compiler-facing classification of a provenance determinism contract.
///
/// This records declarations, not independently verified guarantees.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum DeterminismClass {
    /// Deterministic operation.
    Deterministic,

    /// Deterministic when declared inputs and configuration are unchanged.
    DeterministicGivenInputs,

    /// Operation may produce different results.
    Nondeterministic,

    /// No determinism guarantee was supplied.
    Unspecified,

    /// Extension-defined determinism contract.
    Custom(String),

    /// A future context-defined determinism category.
    Other,
}

impl DeterminismClass {
    fn from_contract(
        contract: &ProvenanceDeterminism,
    ) -> Self {
        match contract {
            ProvenanceDeterminism::Deterministic => {
                Self::Deterministic
            }

            ProvenanceDeterminism::DeterministicGivenInputs => {
                Self::DeterministicGivenInputs
            }

            ProvenanceDeterminism::Nondeterministic => {
                Self::Nondeterministic
            }

            ProvenanceDeterminism::Unspecified => {
                Self::Unspecified
            }

            ProvenanceDeterminism::Custom(value) => {
                Self::Custom(value.to_string())
            }

            _ => Self::Other,
        }
    }
}

// ============================================================================
// EVENT SUMMARY
// ============================================================================

/// Immutable compiler-facing summary of one provenance event.
///
/// This type contains references and descriptions, not artifact payloads.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProvenanceEventSummary {
    identifier: ProvenanceEventId,

    kind: String,

    actor: String,

    actor_version: Option<String>,

    operation: String,

    parents: Vec<ProvenanceEventId>,

    sources: Vec<String>,

    inputs: Vec<String>,

    outputs: Vec<String>,

    semantic_contract: Option<String>,

    target: Option<String>,

    determinism: DeterminismClass,

    outcome: EventOutcomeClass,

    properties: BTreeMap<String, String>,

    description: Option<String>,
}

impl ProvenanceEventSummary {
    /// Returns the stable event identifier.
    #[must_use]
    pub fn identifier(&self) -> &ProvenanceEventId {
        &self.identifier
    }

    /// Returns the open-ended event category.
    #[must_use]
    pub fn kind(&self) -> &str {
        &self.kind
    }

    /// Returns the actor identifier.
    #[must_use]
    pub fn actor(&self) -> &str {
        &self.actor
    }

    /// Returns the actor version.
    #[must_use]
    pub fn actor_version(&self) -> Option<&str> {
        self.actor_version.as_deref()
    }

    /// Returns the operation identifier.
    #[must_use]
    pub fn operation(&self) -> &str {
        &self.operation
    }

    /// Returns causal parent events.
    pub fn parents(
        &self,
    ) -> impl Iterator<Item = &ProvenanceEventId> {
        self.parents.iter()
    }

    /// Returns logical source names.
    pub fn sources(
        &self,
    ) -> impl Iterator<Item = &str> {
        self.sources.iter().map(String::as_str)
    }

    /// Returns input artifact identifiers.
    pub fn inputs(
        &self,
    ) -> impl Iterator<Item = &str> {
        self.inputs.iter().map(String::as_str)
    }

    /// Returns output artifact identifiers.
    pub fn outputs(
        &self,
    ) -> impl Iterator<Item = &str> {
        self.outputs.iter().map(String::as_str)
    }

    /// Returns the semantic-contract identifier.
    #[must_use]
    pub fn semantic_contract(&self) -> Option<&str> {
        self.semantic_contract.as_deref()
    }

    /// Returns the target descriptor identifier.
    #[must_use]
    pub fn target(&self) -> Option<&str> {
        self.target.as_deref()
    }

    /// Returns the declared determinism classification.
    #[must_use]
    pub fn determinism(&self) -> &DeterminismClass {
        &self.determinism
    }

    /// Returns the recorded outcome.
    #[must_use]
    pub fn outcome(&self) -> &EventOutcomeClass {
        &self.outcome
    }

    /// Returns extension properties in deterministic key order.
    pub fn properties(
        &self,
    ) -> impl Iterator<Item = (&str, &str)> {
        self.properties
            .iter()
            .map(|(key, value)| {
                (key.as_str(), value.as_str())
            })
    }

    /// Returns the optional event description.
    #[must_use]
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
}

// ============================================================================
// ARTIFACT LINEAGE
// ============================================================================

/// Immutable summary of an artifact's provenance relationships.
///
/// The artifact subsystem remains responsible for actual artifact contents,
/// serialization, validation, portability and authenticity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactLineage {
    identifier: String,

    kind: String,

    format: Option<String>,

    digest: Option<ContentDigest>,

    producer: Option<ProvenanceEventId>,

    consumers: BTreeSet<ProvenanceEventId>,
}

impl ArtifactLineage {
    /// Returns the artifact identifier.
    #[must_use]
    pub fn identifier(&self) -> &str {
        &self.identifier
    }

    /// Returns the artifact category.
    #[must_use]
    pub fn kind(&self) -> &str {
        &self.kind
    }

    /// Returns the artifact format.
    #[must_use]
    pub fn format(&self) -> Option<&str> {
        self.format.as_deref()
    }

    /// Returns the externally supplied digest.
    #[must_use]
    pub fn digest(&self) -> Option<&ContentDigest> {
        self.digest.as_ref()
    }

    /// Returns the producing event, when known.
    #[must_use]
    pub fn producer(&self) -> Option<&ProvenanceEventId> {
        self.producer.as_ref()
    }

    /// Returns events that consumed this artifact.
    pub fn consumers(
        &self,
    ) -> impl Iterator<Item = &ProvenanceEventId> {
        self.consumers.iter()
    }
}

// ============================================================================
// SOURCE LINEAGE
// ============================================================================

/// Immutable summary of a source's provenance relationships.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceLineage {
    logical_name: String,

    source_map_locators: BTreeSet<String>,

    events: BTreeSet<ProvenanceEventId>,
}

impl SourceLineage {
    /// Returns the logical source name.
    #[must_use]
    pub fn logical_name(&self) -> &str {
        &self.logical_name
    }

    /// Returns available canonical source-map locators.
    ///
    /// These remain opaque references.
    pub fn source_map_locators(
        &self,
    ) -> impl Iterator<Item = &str> {
        self.source_map_locators
            .iter()
            .map(String::as_str)
    }

    /// Returns events associated with this source.
    pub fn events(
        &self,
    ) -> impl Iterator<Item = &ProvenanceEventId> {
        self.events.iter()
    }
}

// ============================================================================
// STATISTICS
// ============================================================================

/// Immutable statistics collected during provenance analysis.
///
/// These values describe observed collection sizes.
/// They are not capacity limits.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ProvenanceAnalysisStatistics {
    events: usize,

    successful_events: usize,

    rejected_events: usize,

    failed_events: usize,

    skipped_events: usize,

    other_outcomes: usize,

    deterministic_events: usize,

    conditional_deterministic_events: usize,

    nondeterministic_events: usize,

    unspecified_determinism_events: usize,

    other_determinism_events: usize,

    sources: usize,

    source_references: usize,

    artifacts: usize,

    artifact_inputs: usize,

    artifact_outputs: usize,

    targets: usize,

    semantic_contracts: usize,
}

impl ProvenanceAnalysisStatistics {
    /// Returns the number of events.
    #[must_use]
    pub const fn events(&self) -> usize {
        self.events
    }

    /// Returns successful event count.
    #[must_use]
    pub const fn successful_events(&self) -> usize {
        self.successful_events
    }

    /// Returns rejected event count.
    #[must_use]
    pub const fn rejected_events(&self) -> usize {
        self.rejected_events
    }

    /// Returns failed event count.
    #[must_use]
    pub const fn failed_events(&self) -> usize {
        self.failed_events
    }

    /// Returns skipped event count.
    #[must_use]
    pub const fn skipped_events(&self) -> usize {
        self.skipped_events
    }

    /// Returns events with other or extension-defined outcomes.
    #[must_use]
    pub const fn other_outcomes(&self) -> usize {
        self.other_outcomes
    }

    /// Returns events declared deterministic.
    #[must_use]
    pub const fn deterministic_events(&self) -> usize {
        self.deterministic_events
    }

    /// Returns events declared deterministic given their inputs.
    #[must_use]
    pub const fn conditional_deterministic_events(&self) -> usize {
        self.conditional_deterministic_events
    }

    /// Returns nondeterministic events.
    #[must_use]
    pub const fn nondeterministic_events(&self) -> usize {
        self.nondeterministic_events
    }

    /// Returns events without a determinism declaration.
    #[must_use]
    pub const fn unspecified_determinism_events(&self) -> usize {
        self.unspecified_determinism_events
    }

    /// Returns events with other determinism classifications.
    #[must_use]
    pub const fn other_determinism_events(&self) -> usize {
        self.other_determinism_events
    }

    /// Returns distinct logical source count.
    #[must_use]
    pub const fn sources(&self) -> usize {
        self.sources
    }

    /// Returns the total number of source references.
    #[must_use]
    pub const fn source_references(&self) -> usize {
        self.source_references
    }

    /// Returns distinct artifact count.
    #[must_use]
    pub const fn artifacts(&self) -> usize {
        self.artifacts
    }

    /// Returns artifact input reference count.
    #[must_use]
    pub const fn artifact_inputs(&self) -> usize {
        self.artifact_inputs
    }

    /// Returns artifact output reference count.
    #[must_use]
    pub const fn artifact_outputs(&self) -> usize {
        self.artifact_outputs
    }

    /// Returns the number of distinct target identifiers.
    #[must_use]
    pub const fn targets(&self) -> usize {
        self.targets
    }

    /// Returns the number of distinct semantic-contract identifiers.
    #[must_use]
    pub const fn semantic_contracts(&self) -> usize {
        self.semantic_contracts
    }
}

// ============================================================================
// ANALYSIS REPORT
// ============================================================================

/// Complete immutable result of one provenance analysis operation.
///
/// All collections are deterministically ordered.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProvenanceAnalysisReport {
    compilation: CompilationProvenanceId,

    source_revision: u64,

    events: BTreeMap<ProvenanceEventId, ProvenanceEventSummary>,

    topological_order: Vec<ProvenanceEventId>,

    artifacts: BTreeMap<String, ArtifactLineage>,

    sources: BTreeMap<String, SourceLineage>,

    event_kinds: BTreeMap<String, usize>,

    actors: BTreeMap<String, usize>,

    operations: BTreeMap<String, usize>,

    statistics: ProvenanceAnalysisStatistics,
}

impl ProvenanceAnalysisReport {
    /// Returns the compilation identity.
    #[must_use]
    pub fn compilation(&self) -> &CompilationProvenanceId {
        &self.compilation
    }

    /// Returns the ledger revision used for this analysis.
    ///
    /// This is the revision of the source snapshot, not an event timestamp.
    #[must_use]
    pub const fn source_revision(&self) -> u64 {
        self.source_revision
    }

    /// Returns the event count.
    #[must_use]
    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    /// Returns whether the report contains events.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    /// Returns an event summary by identifier.
    #[must_use]
    pub fn event(
        &self,
        identifier: &ProvenanceEventId,
    ) -> Option<&ProvenanceEventSummary> {
        self.events.get(identifier)
    }

    /// Returns event summaries in deterministic identifier order.
    pub fn events(
        &self,
    ) -> impl ExactSizeIterator<Item = &ProvenanceEventSummary> {
        self.events.values()
    }

    /// Returns event identifiers in deterministic causal order.
    ///
    /// Events with independent causal histories are ordered by identifier.
    pub fn topological_order(
        &self,
    ) -> impl ExactSizeIterator<Item = &ProvenanceEventId> {
        self.topological_order.iter()
    }

    /// Returns an artifact lineage summary.
    #[must_use]
    pub fn artifact(
        &self,
        identifier: &str,
    ) -> Option<&ArtifactLineage> {
        self.artifacts.get(identifier)
    }

    /// Returns artifacts in deterministic identifier order.
    pub fn artifacts(
        &self,
    ) -> impl ExactSizeIterator<Item = &ArtifactLineage> {
        self.artifacts.values()
    }

    /// Returns a source lineage summary.
    #[must_use]
    pub fn source(
        &self,
        logical_name: &str,
    ) -> Option<&SourceLineage> {
        self.sources.get(logical_name)
    }

    /// Returns sources in deterministic logical-name order.
    pub fn sources(
        &self,
    ) -> impl ExactSizeIterator<Item = &SourceLineage> {
        self.sources.values()
    }

    /// Returns counts by open-ended event kind.
    pub fn event_kinds(
        &self,
    ) -> impl Iterator<Item = (&str, usize)> {
        self.event_kinds
            .iter()
            .map(|(key, value)| (key.as_str(), *value))
    }

    /// Returns counts by actor.
    pub fn actors(
        &self,
    ) -> impl Iterator<Item = (&str, usize)> {
        self.actors
            .iter()
            .map(|(key, value)| (key.as_str(), *value))
    }

    /// Returns counts by operation.
    pub fn operations(
        &self,
    ) -> impl Iterator<Item = (&str, usize)> {
        self.operations
            .iter()
            .map(|(key, value)| (key.as_str(), *value))
    }

    /// Returns provenance analysis statistics.
    #[must_use]
    pub const fn statistics(
        &self,
    ) -> &ProvenanceAnalysisStatistics {
        &self.statistics
    }
}

// ============================================================================
// ANALYSIS ENGINE
// ============================================================================

/// Stateless compiler provenance analysis service.
///
/// The service has no global mutable state and is safe to use concurrently.
#[derive(Debug, Clone, Copy, Default)]
pub struct ProvenanceAnalysis;

impl ProvenanceAnalysis {
    /// Creates the analysis service.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Analyzes an existing provenance context.
    ///
    /// A consistent immutable snapshot is taken before analysis.
    ///
    /// Events recorded after snapshot creation belong to a later analysis.
    pub fn analyze_context(
        &self,
        context: &ProvenanceContext,
    ) -> ProvenanceAnalysisResult<ProvenanceAnalysisReport> {
        let snapshot = context.snapshot()?;
        self.analyze(&snapshot)
    }

    /// Analyzes an immutable authoritative provenance snapshot.
    ///
    /// This method does not mutate the source ledger.
    pub fn analyze(
        &self,
        snapshot: &ProvenanceSnapshot,
    ) -> ProvenanceAnalysisResult<ProvenanceAnalysisReport> {
        let mut report = ProvenanceAnalysisReport {
            compilation: snapshot.compilation().clone(),
            source_revision: snapshot.revision(),
            events: BTreeMap::new(),
            topological_order: Vec::new(),
            artifacts: BTreeMap::new(),
            sources: BTreeMap::new(),
            event_kinds: BTreeMap::new(),
            actors: BTreeMap::new(),
            operations: BTreeMap::new(),
            statistics: ProvenanceAnalysisStatistics::default(),
        };

        self.build_event_index(snapshot, &mut report)?;

        self.build_topological_order(snapshot, &mut report)?;

        self.build_artifact_index(snapshot, &mut report)?;

        self.build_source_index(snapshot, &mut report)?;

        self.calculate_statistics(&mut report)?;

        Ok(report)
    }

    // ------------------------------------------------------------------------
    // Event indexing
    // ------------------------------------------------------------------------

    fn build_event_index(
        &self,
        snapshot: &ProvenanceSnapshot,
        report: &mut ProvenanceAnalysisReport,
    ) -> ProvenanceAnalysisResult<()> {
        for event in snapshot.events() {
            let summary = summarize_event(event);

            let identifier = summary.identifier.clone();

            if report.events.insert(identifier, summary).is_some() {
                return Err(
                    ProvenanceAnalysisError::CounterOverflow {
                        counter: "duplicate event index",
                    }
                );
            }
        }

        Ok(())
    }

    // ------------------------------------------------------------------------
    // Topological ordering
    // ------------------------------------------------------------------------

    /// Produces a deterministic causal event ordering.
    ///
    /// Complexity:
    ///
    ///     O(V log V + E log V)
    ///
    /// where V is the number of events and E is the number of causal
    /// relationships.
    ///
    /// No recursive traversal is used.
    fn build_topological_order(
        &self,
        snapshot: &ProvenanceSnapshot,
        report: &mut ProvenanceAnalysisReport,
    ) -> ProvenanceAnalysisResult<()> {
        let mut in_degree =
            BTreeMap::<ProvenanceEventId, usize>::new();

        let mut children =
            BTreeMap::<
                ProvenanceEventId,
                BTreeSet<ProvenanceEventId>,
            >::new();

        for event in snapshot.events() {
            in_degree.insert(
                event.identifier().clone(),
                0,
            );
        }

        for event in snapshot.events() {
            let event_id = event.identifier();

            for parent in event.parents() {
                if !snapshot.contains_event(parent) {
                    return Err(
                        ProvenanceAnalysisError::UnknownParent {
                            event: event_id.clone(),
                            parent: parent.clone(),
                        }
                    );
                }

                if parent == event_id {
                    return Err(
                        ProvenanceAnalysisError::CycleDetected
                    );
                }

                let degree = in_degree
                    .get_mut(event_id)
                    .ok_or(
                        ProvenanceAnalysisError::CycleDetected
                    )?;

                *degree = degree.checked_add(1).ok_or(
                    ProvenanceAnalysisError::CounterOverflow {
                        counter: "causal in-degree",
                    }
                )?;

                children
                    .entry(parent.clone())
                    .or_default()
                    .insert(event_id.clone());
            }
        }

        // BTreeSet provides deterministic selection among independent
        // events without depending on insertion or worker completion order.
        let mut ready = BTreeSet::new();

        for (identifier, degree) in &in_degree {
            if *degree == 0 {
                ready.insert(identifier.clone());
            }
        }

        while let Some(identifier) = ready.iter().next().cloned() {
            ready.remove(&identifier);

            report.topological_order.push(
                identifier.clone()
            );

            if let Some(dependents) = children.get(&identifier) {
                for dependent in dependents {
                    let degree = in_degree
                        .get_mut(dependent)
                        .ok_or(
                            ProvenanceAnalysisError::CycleDetected
                        )?;

                    *degree = degree.checked_sub(1).ok_or(
                        ProvenanceAnalysisError::CycleDetected
                    )?;

                    if *degree == 0 {
                        ready.insert(dependent.clone());
                    }
                }
            }
        }

        if report.topological_order.len() != snapshot.event_count() {
            return Err(
                ProvenanceAnalysisError::CycleDetected
            );
        }

        Ok(())
    }

    // ------------------------------------------------------------------------
    // Artifact indexing
    // ------------------------------------------------------------------------

    fn build_artifact_index(
        &self,
        snapshot: &ProvenanceSnapshot,
        report: &mut ProvenanceAnalysisReport,
    ) -> ProvenanceAnalysisResult<()> {
        for event in snapshot.events() {
            let event_id = event.identifier();

            for artifact in event.inputs() {
                let entry = artifact_entry(
                    &mut report.artifacts,
                    artifact,
                );

                entry.consumers.insert(
                    event_id.clone()
                );
            }

            for artifact in event.outputs() {
                let entry = artifact_entry(
                    &mut report.artifacts,
                    artifact,
                );

                let producer = snapshot
                    .artifact_producer(artifact.identifier())
                    .cloned();

                if let Some(existing) = &entry.producer {
                    if Some(existing) != producer.as_ref() {
                        return Err(
                            ProvenanceAnalysisError::ConflictingArtifactMetadata {
                                artifact: artifact.identifier().to_owned(),
                            }
                        );
                    }
                }

                entry.producer = producer;
            }
        }

        Ok(())
    }

    // ------------------------------------------------------------------------
    // Source indexing
    // ------------------------------------------------------------------------

    fn build_source_index(
        &self,
        snapshot: &ProvenanceSnapshot,
        report: &mut ProvenanceAnalysisReport,
    ) -> ProvenanceAnalysisResult<()> {
        for event in snapshot.events() {
            let event_id = event.identifier();

            for source in event.sources() {
                let logical_name = source
                    .logical_name()
                    .to_owned();

                let entry = report
                    .sources
                    .entry(logical_name.clone())
                    .or_insert_with(|| SourceLineage {
                        logical_name,
                        source_map_locators: BTreeSet::new(),
                        events: BTreeSet::new(),
                    });

                if let Some(source_map) = source.source_map() {
                    entry.source_map_locators.insert(
                        source_map.locator().to_owned()
                    );
                }

                entry.events.insert(
                    event_id.clone()
                );
            }
        }

        Ok(())
    }

    // ------------------------------------------------------------------------
    // Statistics
    // ------------------------------------------------------------------------

    fn calculate_statistics(
        &self,
        report: &mut ProvenanceAnalysisReport,
    ) -> ProvenanceAnalysisResult<()> {
        let statistics = &mut report.statistics;

        statistics.events = report.events.len();

        statistics.sources = report.sources.len();

        statistics.artifacts = report.artifacts.len();

        for event in report.events.values() {
            increment(
                &mut report.event_kinds,
                event.kind.clone(),
                "event kinds",
            )?;

            increment(
                &mut report.actors,
                event.actor.clone(),
                "actors",
            )?;

            increment(
                &mut report.operations,
                event.operation.clone(),
                "operations",
            )?;

            match event.outcome {
                EventOutcomeClass::Succeeded => {
                    checked_increment(
                        &mut statistics.successful_events,
                        "successful events",
                    )?;
                }

                EventOutcomeClass::Rejected => {
                    checked_increment(
                        &mut statistics.rejected_events,
                        "rejected events",
                    )?;
                }

                EventOutcomeClass::Failed => {
                    checked_increment(
                        &mut statistics.failed_events,
                        "failed events",
                    )?;
                }

                EventOutcomeClass::Skipped => {
                    checked_increment(
                        &mut statistics.skipped_events,
                        "skipped events",
                    )?;
                }

                _ => {
                    checked_increment(
                        &mut statistics.other_outcomes,
                        "other outcomes",
                    )?;
                }
            }

            match event.determinism {
                DeterminismClass::Deterministic => {
                    checked_increment(
                        &mut statistics.deterministic_events,
                        "deterministic events",
                    )?;
                }

                DeterminismClass::DeterministicGivenInputs => {
                    checked_increment(
                        &mut statistics.conditional_deterministic_events,
                        "conditional deterministic events",
                    )?;
                }

                DeterminismClass::Nondeterministic => {
                    checked_increment(
                        &mut statistics.nondeterministic_events,
                        "nondeterministic events",
                    )?;
                }

                DeterminismClass::Unspecified => {
                    checked_increment(
                        &mut statistics.unspecified_determinism_events,
                        "unspecified determinism events",
                    )?;
                }

                _ => {
                    checked_increment(
                        &mut statistics.other_determinism_events,
                        "other determinism events",
                    )?;
                }
            }

            for _ in &event.sources {
                checked_increment(
                    &mut statistics.source_references,
                    "source references",
                )?;
            }

            for _ in &event.inputs {
                checked_increment(
                    &mut statistics.artifact_inputs,
                    "artifact inputs",
                )?;
            }

            for _ in &event.outputs {
                checked_increment(
                    &mut statistics.artifact_outputs,
                    "artifact outputs",
                )?;
            }

            if event.target.is_some() {
                // Distinct targets are calculated below.
            }

            if event.semantic_contract.is_some() {
                // Distinct semantic contracts are calculated below.
            }
        }

        let mut targets = BTreeSet::new();
        let mut contracts = BTreeSet::new();

        for event in report.events.values() {
            if let Some(target) = &event.target {
                targets.insert(target.clone());
            }

            if let Some(contract) = &event.semantic_contract {
                contracts.insert(contract.clone());
            }
        }

        statistics.targets = targets.len();

        statistics.semantic_contracts = contracts.len();

        Ok(())
    }
}

// ============================================================================
// EVENT CONVERSION
// ============================================================================

fn summarize_event(
    event: &ProvenanceEvent,
) -> ProvenanceEventSummary {
    ProvenanceEventSummary {
        identifier: event.identifier().clone(),

        kind: event.kind().as_str().to_owned(),

        actor: event.actor().component().to_owned(),

        actor_version: event
            .actor()
            .version()
            .map(str::to_owned),

        operation: event.operation().to_owned(),

        parents: event.parents().cloned().collect(),

        sources: event
            .sources()
            .map(|source| source.logical_name().to_owned())
            .collect(),

        inputs: event
            .inputs()
            .map(|artifact| artifact.identifier().to_owned())
            .collect(),

        outputs: event
            .outputs()
            .map(|artifact| artifact.identifier().to_owned())
            .collect(),

        semantic_contract: event
            .semantic_contract()
            .map(|contract| contract.identifier().to_owned()),

        target: event
            .target()
            .map(|target| target.identifier().to_owned()),

        determinism: DeterminismClass::from_contract(
            event.determinism()
        ),

        outcome: EventOutcomeClass::from_outcome(
            event.outcome()
        ),

        properties: event
            .properties()
            .map(|(key, value)| {
                (key.to_owned(), value.to_owned())
            })
            .collect(),

        description: event
            .description()
            .map(str::to_owned),
    }
}

// ============================================================================
// ARTIFACT INDEX HELPERS
// ============================================================================

fn artifact_entry<'a>(
    artifacts: &'a mut BTreeMap<String, ArtifactLineage>,
    reference: &ProvenanceArtifactReference,
) -> &'a mut ArtifactLineage {
    let identifier = reference.identifier().to_owned();

    artifacts.entry(identifier.clone()).or_insert_with(|| {
        ArtifactLineage {
            identifier,
            kind: reference.kind().as_str().to_owned(),
            format: reference.format().map(str::to_owned),
            digest: reference.digest().cloned(),
            producer: None,
            consumers: BTreeSet::new(),
        }
    })
}

// ============================================================================
// COUNTER HELPERS
// ============================================================================

fn checked_increment(
    value: &mut usize,
    counter: &'static str,
) -> ProvenanceAnalysisResult<()> {
    *value = value.checked_add(1).ok_or(
        ProvenanceAnalysisError::CounterOverflow {
            counter,
        }
    )?;

    Ok(())
}

fn increment(
    values: &mut BTreeMap<String, usize>,
    key: String,
    counter: &'static str,
) -> ProvenanceAnalysisResult<()> {
    let value = values.entry(key).or_insert(0);

    checked_increment(value, counter)
}

// ============================================================================
// PUBLIC API VERSION
// ============================================================================

/// Returns the provenance-analysis API version.
#[must_use]
pub const fn api_version() -> u16 {
    PROVENANCE_ANALYSIS_API_VERSION
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::compiler::context::{
        ArtifactKind,
        CompilationProvenanceId,
        ContentDigest,
        ProvenanceActor,
        ProvenanceArtifactReference,
        ProvenanceContext,
        ProvenanceDeterminism,
        ProvenanceEvent,
        ProvenanceEventId,
        ProvenanceEventKind,
        ProvenanceOutcome,
    };

    fn compilation_id() -> CompilationProvenanceId {
        CompilationProvenanceId::new(
            "compilation:provenance-analysis-test"
        )
        .expect("valid compilation identity")
    }

    fn context() -> ProvenanceContext {
        ProvenanceContext::new(compilation_id())
    }

    fn event(
        identifier: &str,
        parents: &[&str],
    ) -> ProvenanceEvent {
        let mut value = ProvenanceEvent::new(
            ProvenanceEventId::new(identifier)
                .expect("valid event identifier"),

            ProvenanceEventKind::new("compiler.transformation")
                .expect("valid event kind"),

            ProvenanceActor::new("zamani.compiler")
                .expect("valid actor"),

            "test.operation",
        )
        .expect("valid event");

        for parent in parents {
            value = value.with_parent(
                ProvenanceEventId::new(*parent)
                    .expect("valid parent identifier")
            );
        }

        value
    }

    fn artifact(
        identifier: &str,
    ) -> ProvenanceArtifactReference {
        ProvenanceArtifactReference::new(
            identifier,
            ArtifactKind::new("portable_ir")
                .expect("valid artifact kind"),
        )
        .expect("valid artifact reference")
    }

    fn analyzer() -> ProvenanceAnalysis {
        ProvenanceAnalysis::new()
    }

    #[test]
    fn api_version_is_available() {
        assert_eq!(
            api_version(),
            PROVENANCE_ANALYSIS_API_VERSION
        );

        assert_ne!(
            PROVENANCE_ANALYSIS_API_VERSION,
            0
        );
    }

    #[test]
    fn analyzes_an_empty_context() {
        let context = context();

        let report = analyzer()
            .analyze_context(&context)
            .expect("empty provenance should analyze");

        assert!(report.is_empty());

        assert_eq!(
            report.statistics().events(),
            0
        );
    }

    #[test]
    fn records_and_analyzes_event_lineage() {
        let context = context();

        context
            .record(event("event:source", &[]))
            .expect("source event should be recorded");

        context
            .record(event(
                "event:lowering",
                &["event:source"],
            ))
            .expect("lowering event should be recorded");

        let report = analyzer()
            .analyze_context(&context)
            .expect("lineage should analyze");

        assert_eq!(report.event_count(), 2);

        let order: Vec<&str> = report
            .topological_order()
            .map(ProvenanceEventId::as_str)
            .collect();

        assert_eq!(
            order,
            vec![
                "event:source",
                "event:lowering",
            ]
        );
    }

    #[test]
    fn topological_order_is_deterministic() {
        let context = context();

        context
            .record(event("event:z", &[]))
            .expect("event should be recorded");

        context
            .record(event("event:a", &[]))
            .expect("event should be recorded");

        context
            .record(event(
                "event:child",
                &["event:a", "event:z"],
            ))
            .expect("child should be recorded");

        let first = analyzer()
            .analyze_context(&context)
            .expect("analysis should succeed");

        let second = analyzer()
            .analyze_context(&context)
            .expect("analysis should succeed");

        assert_eq!(
            first.topological_order.collect::<Vec<_>>(),
            second.topological_order.collect::<Vec<_>>()
        );
    }

    #[test]
    fn detects_failed_and_rejected_events() {
        let context = context();

        context
            .record(
                event("event:failed", &[])
                    .with_outcome(ProvenanceOutcome::Failed)
            )
            .expect("event should be recorded");

        context
            .record(
                event("event:rejected", &[])
                    .with_outcome(ProvenanceOutcome::Rejected)
            )
            .expect("event should be recorded");

        let report = analyzer()
            .analyze_context(&context)
            .expect("analysis should succeed");

        assert_eq!(
            report.statistics().failed_events(),
            1
        );

        assert_eq!(
            report.statistics().rejected_events(),
            1
        );
    }

    #[test]
    fn analyzes_determinism_declarations() {
        let context = context();

        context
            .record(
                event("event:deterministic", &[])
                    .with_determinism(
                        ProvenanceDeterminism::Deterministic
                    )
            )
            .expect("event should be recorded");

        context
            .record(
                event("event:nondeterministic", &[])
                    .with_determinism(
                        ProvenanceDeterminism::Nondeterministic
                    )
            )
            .expect("event should be recorded");

        let report = analyzer()
            .analyze_context(&context)
            .expect("analysis should succeed");

        assert_eq!(
            report.statistics().deterministic_events(),
            1
        );

        assert_eq!(
            report.statistics().nondeterministic_events(),
            1
        );
    }

    #[test]
    fn analyzes_artifact_producers_and_consumers() {
        let context = context();

        let produced = event("event:producer", &[])
            .with_output(artifact("artifact:portable-ir"));

        context
            .record(produced)
            .expect("producer should be recorded");

        let consumer = event(
            "event:consumer",
            &["event:producer"],
        )
        .with_input(artifact("artifact:portable-ir"));

        context
            .record(consumer)
            .expect("consumer should be recorded");

        let report = analyzer()
            .analyze_context(&context)
            .expect("analysis should succeed");

        let lineage = report
            .artifact("artifact:portable-ir")
            .expect("artifact should exist");

        assert_eq!(
            lineage.producer()
                .map(ProvenanceEventId::as_str),
            Some("event:producer")
        );

        assert_eq!(
            lineage.consumers().count(),
            1
        );
    }

    #[test]
    fn artifact_digest_is_preserved() {
        let context = context();

        let digest = ContentDigest::new(
            "sha-256",
            "external-digest-value",
        )
        .expect("digest reference should be valid");

        let output = artifact("artifact:one")
            .with_digest(digest.clone());

        context
            .record(
                event("event:producer", &[])
                    .with_output(output)
            )
            .expect("event should be recorded");

        let report = analyzer()
            .analyze_context(&context)
            .expect("analysis should succeed");

        assert_eq!(
            report
                .artifact("artifact:one")
                .and_then(ArtifactLineage::digest),
            Some(&digest)
        );
    }

    #[test]
    fn report_preserves_source_revision() {
        let context = context();

        context
            .record(event("event:one", &[]))
            .expect("event should be recorded");

        let report = analyzer()
            .analyze_context(&context)
            .expect("analysis should succeed");

        assert_eq!(report.source_revision(), 1);
    }

    #[test]
    fn report_uses_an_immutable_snapshot() {
        let context = context();

        context
            .record(event("event:first", &[]))
            .expect("event should be recorded");

        let first = analyzer()
            .analyze_context(&context)
            .expect("first analysis should succeed");

        context
            .record(event("event:second", &[]))
            .expect("event should be recorded");

        let second = analyzer()
            .analyze_context(&context)
            .expect("second analysis should succeed");

        assert_eq!(first.event_count(), 1);
        assert_eq!(second.event_count(), 2);
    }

    #[test]
    fn event_categories_are_open_ended() {
        let context = context();

        let custom = ProvenanceEvent::new(
            ProvenanceEventId::new("event:future-domain")
                .expect("valid identifier"),

            ProvenanceEventKind::new(
                "future_computing_domain.specialized_transform"
            )
            .expect("valid open-ended category"),

            ProvenanceActor::new("external.provider")
                .expect("valid actor"),

            "future.operation",
        )
        .expect("valid event");

        context
            .record(custom)
            .expect("event should be recorded");

        let report = analyzer()
            .analyze_context(&context)
            .expect("analysis should succeed");

        assert_eq!(
            report
                .event_kinds()
                .find(|(kind, _)| {
                    *kind == "future_computing_domain.specialized_transform"
                })
                .map(|(_, count)| count),
            Some(1)
        );
    }

    #[test]
    fn analysis_does_not_mutate_the_authoritative_ledger() {
        let context = context();

        context
            .record(event("event:one", &[]))
            .expect("event should be recorded");

        let before = context
            .event_count()
            .expect("count should succeed");

        analyzer()
            .analyze_context(&context)
            .expect("analysis should succeed");

        let after = context
            .event_count()
            .expect("count should succeed");

        assert_eq!(before, after);
    }
}
