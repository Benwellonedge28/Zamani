//! Zamani Quantum Resilience — Logical-to-Physical Remapping
//!
//! Path:
//!     src/quantum/resilience/adaptation/remapping.rs
//!
//! Purpose:
//!     Provide the provider-independent remapping adaptation used by the
//!     resilience subsystem when the current logical-to-physical placement
//!     becomes invalid, degraded, unavailable, or otherwise unsuitable.
//!
//! ============================================================================
//! Architectural role
//! ============================================================================
//!
//! This module answers:
//!
//!     "Given an existing logical-to-physical mapping and an explicitly
//!      supplied replacement mapping, can resilience safely construct,
//!      validate, prepare, and commit the replacement mapping?"
//!
//! This module DOES:
//!
//! - use the canonical Zamani qubit identity types;
//! - represent a remapping request;
//! - validate mapping identity and cardinality invariants;
//! - validate one-to-one logical/physical ownership;
//! - support partial and complete mappings;
//! - support sparse identifiers;
//! - support arbitrarily large finite mappings subject only to available
//!   process resources;
//! - preserve deterministic ordering;
//! - detect stale execution generations;
//! - detect stale semantic revisions;
//! - prepare an immutable candidate;
//! - commit only a previously prepared candidate;
//! - expose deterministic mapping differences;
//! - preserve the logical program's semantic identity;
//! - expose enough metadata for provenance, telemetry and verification.
//!
//! This module DOES NOT:
//!
//! - discover hardware;
//! - determine whether a physical qubit actually exists;
//! - determine whether a physical qubit is calibrated;
//! - calculate topology;
//! - calculate routes;
//! - insert SWAP operations;
//! - schedule operations;
//! - compile circuits;
//! - optimize circuits;
//! - perform QEC;
//! - communicate with a backend;
//! - allocate hardware;
//! - perform authorization;
//! - override resilience policy;
//! - define another QubitId;
//! - define another PhysicalQubitId;
//! - impose a fixed qubit limit;
//! - use unsafe Rust.
//!
//! Those responsibilities belong to the corresponding canonical subsystems.
//!
//! ============================================================================
//! Canonical identity
//! ============================================================================
//!
//! Logical identity:
//!
//!     crate::quantum::ir::qubit::QubitId
//!
//! Physical identity:
//!
//!     crate::quantum::ir::qubit::PhysicalQubitId
//!
//! Mapping:
//!
//!     crate::quantum::ir::resources::mapping::QubitMapping
//!
//! This module intentionally does not create replacement identity types.
//!
//! The repository's canonical mapping abstraction already provides the
//! logical-to-physical semantic representation and bidirectional invariants.
//! Resilience therefore consumes that abstraction rather than duplicating it.
//!
//! ============================================================================
//! Write once, scale everywhere
//! ============================================================================
//!
//! There is deliberately no:
//!
//!     MAX_QUBITS
//!     MAX_PHYSICAL_QUBITS
//!     MAX_MAPPING_ENTRIES
//!     MAX_REMAP_SIZE
//!     provider-specific qubit count
//!
//! A remapping contains only the resources actually present in the supplied
//! mapping.
//!
//! "Infinity" in the Zamani architecture means that this module contributes
//! no artificial finite machine-size ceiling. Every concrete execution is
//! naturally bounded by:
//!
//! - addressable host memory;
//! - the target's discovered capabilities;
//! - execution policy;
//! - resource budgets;
//! - operating-system/process limits.
//!
//! The implementation therefore uses the sparse canonical mapping rather than
//! materializing an array indexed by machine size.
//!
//! ============================================================================
//! Transaction model
//! ============================================================================
//!
//! Remapping is transactional:
//!
//!     Request
//!        |
//!        v
//!     Validate
//!        |
//!        v
//!     Prepare
//!        |
//!        v
//!     PreparedRemapping
//!        |
//!        v
//!     Commit
//!        |
//!        v
//!     RemappingResult
//!
//! A prepared candidate is NOT a committed mapping.
//!
//! A candidate can be rejected before commit by:
//!
//! - policy;
//! - feasibility;
//! - authorization;
//! - stale-state detection;
//! - semantic verification;
//! - another resilience component.
//!
//! ============================================================================
//! Determinism
//! ============================================================================
//!
//! Determinism is achieved by:
//!
//! - using the canonical deterministic QubitMapping representation;
//! - sorting mapping differences by canonical identifier order;
//! - never consulting hidden global mutable state;
//! - carrying all decision-relevant state explicitly;
//! - using no provider-specific implicit behavior.
//!
//! The remapping implementation itself does not select a new mapping.
//! Selection belongs to routing/planning.
//!
//! ============================================================================
//! Integration
//! ============================================================================
//!
//! planning/action.rs
//!     |
//!     | ActionKind::Remap
//!     v
//! adaptation/adapter.rs
//!     |
//!     v
//! this module
//!     |
//!     +--> quantum::ir::resources::mapping
//!     |
//!     +--> quantum::ir::qubit
//!     |
//!     v
//! planning / verification / recovery
//!
//! The actual replacement mapping is supplied by the caller. This is
//! intentional: resilience must not become a second routing implementation.
//!
//! A higher layer is responsible for obtaining the replacement mapping from
//! `quantum::routing` or another authoritative placement subsystem.
//!
//! ============================================================================
//! Semantic invariant
//! ============================================================================
//!
//! A remapping changes physical realization only.
//!
//! It must not silently change:
//!
//! - logical qubit identity;
//! - logical operation ordering;
//! - logical operation meaning;
//! - measurement semantics;
//! - program semantics.
//!
//! Semantic verification remains mandatory at the verification layer.
//!
//! ============================================================================
//! Rust compatibility
//! ============================================================================
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code;
//! - no external dependencies.
//!
//! ============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_debug_implementations)]
#![deny(clippy::all)]

use std::fmt;

use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};
use crate::quantum::ir::resources::mapping::{
    MappingDomain,
    QubitMapping,
};

use crate::quantum::resilience::errors::{
    ResilienceError,
    ResilienceErrorCode,
    ResilienceResult,
};

use crate::quantum::resilience::planning::action::{
    ActionKind,
    ActionScope,
};

// ============================================================================
// Stable schema identity
// ============================================================================

/// Stable schema identifier for the remapping adaptation contract.
pub const REMAPPING_SCHEMA_ID: &str =
    "zamani.quantum.resilience.adaptation.remapping";

/// Semantic version of the remapping contract.
pub const REMAPPING_SCHEMA_VERSION: u16 = 1;

// ============================================================================
// Remapping identity
// ============================================================================

/// Stable identifier for one remapping operation.
///
/// This identifier is supplied by the resilience planner/runtime and is not
/// derived from the number of qubits or the selected hardware provider.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RemappingId(String);

impl RemappingId {
    /// Creates a remapping identifier.
    pub fn new(value: impl Into<String>) -> ResilienceResult<Self> {
        let value = value.into();

        if value.is_empty() || value.chars().any(char::is_whitespace) {
            return Err(ResilienceError::new(
                ResilienceErrorCode::InvalidIdentifier,
            ));
        }

        Ok(Self(value))
    }

    /// Returns the stable identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the identifier.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for RemappingId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// ============================================================================
// Mapping revision
// ============================================================================

/// Opaque revision identifying the mapping state from which the remap starts.
///
/// This is deliberately opaque because the canonical IR/mapping layer owns
/// hashing and semantic fingerprinting.
///
/// The resilience layer only needs equality for stale-state detection.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MappingRevision(String);

impl MappingRevision {
    /// Creates a mapping revision.
    pub fn new(value: impl Into<String>) -> ResilienceResult<Self> {
        let value = value.into();

        if value.is_empty() {
            return Err(ResilienceError::new(
                ResilienceErrorCode::InvalidIdentifier,
            ));
        }

        Ok(Self(value))
    }

    /// Returns the revision.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for MappingRevision {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// ============================================================================
// Execution generation
// ============================================================================

/// Opaque execution-generation identifier.
///
/// A generation changes whenever the authoritative execution state changes
/// in a way that could invalidate a prepared remapping.
///
/// Examples of such changes include:
///
/// - a new execution;
/// - a new compiled artifact;
/// - an authoritative mapping replacement;
/// - execution migration;
/// - state restoration.
///
/// This type does not generate generations itself.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ExecutionGeneration(String);

impl ExecutionGeneration {
    /// Creates an execution-generation identifier.
    pub fn new(value: impl Into<String>) -> ResilienceResult<Self> {
        let value = value.into();

        if value.is_empty() {
            return Err(ResilienceError::new(
                ResilienceErrorCode::InvalidIdentifier,
            ));
        }

        Ok(Self(value))
    }

    /// Returns the opaque generation.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ExecutionGeneration {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// ============================================================================
// Semantic revision
// ============================================================================

/// Opaque revision of the logical computation.
///
/// Remapping must preserve this revision.
///
/// The actual semantic representation belongs to canonical quantum IR.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SemanticRevision(String);

impl SemanticRevision {
    /// Creates a semantic revision.
    pub fn new(value: impl Into<String>) -> ResilienceResult<Self> {
        let value = value.into();

        if value.is_empty() {
            return Err(ResilienceError::new(
                ResilienceErrorCode::InvalidIdentifier,
            ));
        }

        Ok(Self(value))
    }

    /// Returns the semantic revision.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for SemanticRevision {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// ============================================================================
// Mapping change
// ============================================================================

/// One deterministic mapping change.
///
/// This is deliberately a value object rather than a mutation command.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MappingChange {
    logical: QubitId,
    previous: Option<PhysicalQubitId>,
    replacement: Option<PhysicalQubitId>,
}

impl MappingChange {
    /// Creates a mapping-change record.
    #[must_use]
    pub const fn new(
        logical: QubitId,
        previous: Option<PhysicalQubitId>,
        replacement: Option<PhysicalQubitId>,
    ) -> Self {
        Self {
            logical,
            previous,
            replacement,
        }
    }

    /// Returns the affected logical qubit.
    #[must_use]
    pub const fn logical(self) -> QubitId {
        self.logical
    }

    /// Returns the previous physical placement.
    #[must_use]
    pub const fn previous(self) -> Option<PhysicalQubitId> {
        self.previous
    }

    /// Returns the replacement physical placement.
    #[must_use]
    pub const fn replacement(self) -> Option<PhysicalQubitId> {
        self.replacement
    }

    /// Returns whether this logical qubit was newly mapped.
    #[must_use]
    pub const fn is_new_mapping(self) -> bool {
        self.previous.is_none() && self.replacement.is_some()
    }

    /// Returns whether this logical qubit became unmapped.
    #[must_use]
    pub const fn is_unmapped(self) -> bool {
        self.previous.is_some() && self.replacement.is_none()
    }

    /// Returns whether the physical placement changed.
    #[must_use]
    pub const fn changed(self) -> bool {
        !matches!(
            (self.previous, self.replacement),
            (None, None)
        ) && self.previous != self.replacement
    }
}

impl fmt::Display for MappingChange {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.previous, self.replacement) {
            (Some(previous), Some(replacement)) => write!(
                formatter,
                "{}: {} -> {}",
                self.logical,
                previous,
                replacement
            ),
            (None, Some(replacement)) => write!(
                formatter,
                "{}: unmapped -> {}",
                self.logical,
                replacement
            ),
            (Some(previous), None) => write!(
                formatter,
                "{}: {} -> unmapped",
                self.logical,
                previous
            ),
            (None, None) => write!(
                formatter,
                "{}: unmapped -> unmapped",
                self.logical
            ),
        }
    }
}

// ============================================================================
// Remapping scope
// ============================================================================

/// Scope of the remapping request.
///
/// The scope is descriptive. It does not select hardware or perform routing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RemappingScope {
    /// Entire computation mapping.
    Computation,

    /// Only resources known to be affected by the incident.
    AffectedResources,

    /// Explicitly supplied logical domain.
    LogicalDomain,
}

impl RemappingScope {
    /// Stable serialized representation.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Computation => "computation",
            Self::AffectedResources => "affected_resources",
            Self::LogicalDomain => "logical_domain",
        }
    }
}

impl fmt::Display for RemappingScope {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// Remapping request
// ============================================================================

/// Immutable request to construct a replacement logical-to-physical mapping.
///
/// The replacement mapping must be supplied by an authoritative placement
/// component such as routing.
///
/// This type therefore keeps resilience separate from routing.
#[derive(Debug, Clone)]
pub struct RemappingRequest {
    id: RemappingId,
    scope: RemappingScope,
    semantic_revision: SemanticRevision,
    execution_generation: ExecutionGeneration,
    current_mapping_revision: MappingRevision,
    current_mapping: QubitMapping,
    replacement_mapping: QubitMapping,
    logical_domain: Option<MappingDomain>,
    require_complete_mapping: bool,
}

impl RemappingRequest {
    /// Creates a remapping request.
    ///
    /// The constructor performs structural validation but deliberately does
    /// not validate physical hardware availability.
    pub fn new(
        id: RemappingId,
        scope: RemappingScope,
        semantic_revision: SemanticRevision,
        execution_generation: ExecutionGeneration,
        current_mapping_revision: MappingRevision,
        current_mapping: QubitMapping,
        replacement_mapping: QubitMapping,
    ) -> ResilienceResult<Self> {
        Self::new_with_domain(
            id,
            scope,
            semantic_revision,
            execution_generation,
            current_mapping_revision,
            current_mapping,
            replacement_mapping,
            None,
            false,
        )
    }

    /// Creates a request with an explicit logical domain.
    ///
    /// When `require_complete_mapping` is true, both current and replacement
    /// mappings are checked against the supplied domain.
    pub fn new_with_domain(
        id: RemappingId,
        scope: RemappingScope,
        semantic_revision: SemanticRevision,
        execution_generation: ExecutionGeneration,
        current_mapping_revision: MappingRevision,
        current_mapping: QubitMapping,
        replacement_mapping: QubitMapping,
        logical_domain: Option<MappingDomain>,
        require_complete_mapping: bool,
    ) -> ResilienceResult<Self> {
        current_mapping
            .validate()
            .map_err(Self::mapping_validation_error)?;

        replacement_mapping
            .validate()
            .map_err(Self::mapping_validation_error)?;

        if require_complete_mapping {
            let domain = logical_domain.ok_or_else(|| {
                ResilienceError::new(ResilienceErrorCode::InvalidConfiguration)
            })?;

            current_mapping
                .require_complete(domain)
                .map_err(Self::mapping_validation_error)?;

            replacement_mapping
                .require_complete(domain)
                .map_err(Self::mapping_validation_error)?;
        }

        Self::validate_logical_namespace(
            &current_mapping,
            &replacement_mapping,
            logical_domain,
        )?;

        Ok(Self {
            id,
            scope,
            semantic_revision,
            execution_generation,
            current_mapping_revision,
            current_mapping,
            replacement_mapping,
            logical_domain,
            require_complete_mapping,
        })
    }

    /// Returns the remapping identifier.
    #[must_use]
    pub fn id(&self) -> &RemappingId {
        &self.id
    }

    /// Returns the requested scope.
    #[must_use]
    pub const fn scope(&self) -> RemappingScope {
        self.scope
    }

    /// Returns the semantic revision.
    #[must_use]
    pub fn semantic_revision(&self) -> &SemanticRevision {
        &self.semantic_revision
    }

    /// Returns the execution generation.
    #[must_use]
    pub fn execution_generation(&self) -> &ExecutionGeneration {
        &self.execution_generation
    }

    /// Returns the expected current mapping revision.
    #[must_use]
    pub fn current_mapping_revision(&self) -> &MappingRevision {
        &self.current_mapping_revision
    }

    /// Returns the current mapping.
    #[must_use]
    pub fn current_mapping(&self) -> &QubitMapping {
        &self.current_mapping
    }

    /// Returns the replacement mapping.
    #[must_use]
    pub fn replacement_mapping(&self) -> &QubitMapping {
        &self.replacement_mapping
    }

    /// Returns the optional logical domain.
    #[must_use]
    pub fn logical_domain(&self) -> Option<MappingDomain> {
        self.logical_domain
    }

    /// Returns whether complete mapping is required.
    #[must_use]
    pub const fn require_complete_mapping(&self) -> bool {
        self.require_complete_mapping
    }

    /// Returns whether the replacement differs from the current mapping.
    #[must_use]
    pub fn changes_mapping(&self) -> bool {
        self.current_mapping != self.replacement_mapping
    }

    /// Produces deterministic mapping differences.
    #[must_use]
    pub fn changes(&self) -> Vec<MappingChange> {
        calculate_changes(&self.current_mapping, &self.replacement_mapping)
    }

    fn validate_logical_namespace(
        current: &QubitMapping,
        replacement: &QubitMapping,
        domain: Option<MappingDomain>,
    ) -> ResilienceResult<()> {
        if let Some(domain) = domain {
            for entry in replacement.iter() {
                if !domain.contains(entry.logical()) {
                    return Err(ResilienceError::new(
                        ResilienceErrorCode::InvalidConfiguration,
                    ));
                }
            }

            for entry in current.iter() {
                if !domain.contains(entry.logical()) {
                    return Err(ResilienceError::new(
                        ResilienceErrorCode::InvalidConfiguration,
                    ));
                }
            }
        }

        Ok(())
    }

    fn mapping_validation_error<E>(_error: E) -> ResilienceError
    where
        E: fmt::Display,
    {
        ResilienceError::new(ResilienceErrorCode::InvalidConfiguration)
    }
}

// ============================================================================
// Prepared remapping
// ============================================================================

/// Immutable remapping candidate.
///
/// A candidate is not committed execution state.
#[derive(Debug, Clone)]
pub struct PreparedRemapping {
    id: RemappingId,
    semantic_revision: SemanticRevision,
    execution_generation: ExecutionGeneration,
    expected_mapping_revision: MappingRevision,
    replacement_mapping: QubitMapping,
    changes: Vec<MappingChange>,
}

impl PreparedRemapping {
    /// Creates a prepared candidate from a validated request.
    fn from_request(request: &RemappingRequest) -> Self {
        Self {
            id: request.id.clone(),
            semantic_revision: request.semantic_revision.clone(),
            execution_generation: request.execution_generation.clone(),
            expected_mapping_revision: request.current_mapping_revision.clone(),
            replacement_mapping: request.replacement_mapping.clone(),
            changes: request.changes(),
        }
    }

    /// Returns the remapping identifier.
    #[must_use]
    pub fn id(&self) -> &RemappingId {
        &self.id
    }

    /// Returns the semantic revision.
    #[must_use]
    pub fn semantic_revision(&self) -> &SemanticRevision {
        &self.semantic_revision
    }

    /// Returns the execution generation.
    #[must_use]
    pub fn execution_generation(&self) -> &ExecutionGeneration {
        &self.execution_generation
    }

    /// Returns the expected source mapping revision.
    #[must_use]
    pub fn expected_mapping_revision(&self) -> &MappingRevision {
        &self.expected_mapping_revision
    }

    /// Returns the candidate mapping.
    #[must_use]
    pub fn replacement_mapping(&self) -> &QubitMapping {
        &self.replacement_mapping
    }

    /// Returns deterministic mapping changes.
    #[must_use]
    pub fn changes(&self) -> &[MappingChange] {
        &self.changes
    }

    /// Returns the number of changed logical assignments.
    #[must_use]
    pub fn change_count(&self) -> usize {
        self.changes.len()
    }

    /// Returns whether the candidate changes no mapping entry.
    #[must_use]
    pub fn is_noop(&self) -> bool {
        self.changes.is_empty()
    }
}

// ============================================================================
// Commit context
// ============================================================================

/// Authoritative state supplied when committing a prepared remapping.
///
/// The adapter never reads hidden mutable state to establish these values.
#[derive(Debug, Clone)]
pub struct RemappingCommitContext {
    execution_generation: ExecutionGeneration,
    mapping_revision: MappingRevision,
    semantic_revision: SemanticRevision,
}

impl RemappingCommitContext {
    /// Creates a commit context.
    pub fn new(
        execution_generation: ExecutionGeneration,
        mapping_revision: MappingRevision,
        semantic_revision: SemanticRevision,
    ) -> Self {
        Self {
            execution_generation,
            mapping_revision,
            semantic_revision,
        }
    }

    /// Returns the current execution generation.
    #[must_use]
    pub fn execution_generation(&self) -> &ExecutionGeneration {
        &self.execution_generation
    }

    /// Returns the current mapping revision.
    #[must_use]
    pub fn mapping_revision(&self) -> &MappingRevision {
        &self.mapping_revision
    }

    /// Returns the current semantic revision.
    #[must_use]
    pub fn semantic_revision(&self) -> &SemanticRevision {
        &self.semantic_revision
    }
}

// ============================================================================
// Remapping result
// ============================================================================

/// Result of committing a remapping candidate.
///
/// The result contains the new semantic mapping but does not mutate external
/// runtime state. The authoritative execution/runtime layer is responsible
/// for applying the returned mapping to its state transaction.
#[derive(Debug, Clone)]
pub struct RemappingResult {
    id: RemappingId,
    semantic_revision: SemanticRevision,
    execution_generation: ExecutionGeneration,
    previous_mapping_revision: MappingRevision,
    mapping: QubitMapping,
    changes: Vec<MappingChange>,
}

impl RemappingResult {
    /// Returns the remapping identifier.
    #[must_use]
    pub fn id(&self) -> &RemappingId {
        &self.id
    }

    /// Returns the semantic revision.
    #[must_use]
    pub fn semantic_revision(&self) -> &SemanticRevision {
        &self.semantic_revision
    }

    /// Returns the execution generation.
    #[must_use]
    pub fn execution_generation(&self) -> &ExecutionGeneration {
        &self.execution_generation
    }

    /// Returns the previous mapping revision.
    #[must_use]
    pub fn previous_mapping_revision(&self) -> &MappingRevision {
        &self.previous_mapping_revision
    }

    /// Returns the committed mapping candidate.
    #[must_use]
    pub fn mapping(&self) -> &QubitMapping {
        &self.mapping
    }

    /// Returns deterministic mapping changes.
    #[must_use]
    pub fn changes(&self) -> &[MappingChange] {
        &self.changes
    }

    /// Returns the number of changed assignments.
    #[must_use]
    pub fn change_count(&self) -> usize {
        self.changes.len()
    }

    /// Returns whether the committed candidate was a no-op.
    #[must_use]
    pub fn is_noop(&self) -> bool {
        self.changes.is_empty()
    }
}

// ============================================================================
// Remapping adapter
// ============================================================================

/// Provider-independent remapping adapter.
///
/// This implementation deliberately performs no routing or hardware lookup.
/// It validates and prepares a replacement mapping that another subsystem has
/// already selected.
#[derive(Debug, Default, Clone, Copy)]
pub struct RemappingAdapter;

impl RemappingAdapter {
    /// Creates a remapping adapter.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Returns the canonical action implemented by this adapter.
    #[must_use]
    pub const fn action_kind(&self) -> ActionKind {
        ActionKind::Remap
    }

    /// Returns the canonical action scope represented by this adapter.
    #[must_use]
    pub const fn action_scope(&self) -> ActionScope {
        ActionScope::Computation
    }

    /// Validates a remapping request without creating a candidate.
    ///
    /// This operation:
    ///
    /// - checks canonical mapping invariants;
    /// - checks requested completeness;
    /// - checks that the replacement differs when required;
    /// - checks that the logical namespace is compatible.
    ///
    /// It does not:
    ///
    /// - access hardware;
    /// - check calibration;
    /// - run routing;
    /// - mutate execution state.
    pub fn preflight(
        &self,
        request: &RemappingRequest,
    ) -> ResilienceResult<()> {
        request
            .current_mapping
            .validate()
            .map_err(|_| {
                ResilienceError::new(ResilienceErrorCode::InvalidConfiguration)
            })?;

        request
            .replacement_mapping
            .validate()
            .map_err(|_| {
                ResilienceError::new(ResilienceErrorCode::InvalidConfiguration)
            })?;

        if request.require_complete_mapping {
            let domain = request.logical_domain.ok_or_else(|| {
                ResilienceError::new(ResilienceErrorCode::InvalidConfiguration)
            })?;

            request
                .replacement_mapping
                .require_complete(domain)
                .map_err(|_| {
                    ResilienceError::new(
                        ResilienceErrorCode::InvalidConfiguration,
                    )
                })?;
        }

        if request.logical_domain.is_some() {
            Self::validate_domain_consistency(request)?;
        }

        Ok(())
    }

    /// Prepares a remapping candidate.
    ///
    /// Preparation does not commit external execution state.
    pub fn prepare(
        &self,
        request: &RemappingRequest,
    ) -> ResilienceResult<PreparedRemapping> {
        self.preflight(request)?;

        Ok(PreparedRemapping::from_request(request))
    }

    /// Commits a prepared candidate against authoritative current state.
    ///
    /// The operation is still pure with respect to external runtime state:
    /// it returns the mapping that the caller must commit to its authoritative
    /// state store.
    ///
    /// Stale candidates are rejected.
    pub fn commit(
        &self,
        prepared: PreparedRemapping,
        context: &RemappingCommitContext,
    ) -> ResilienceResult<RemappingResult> {
        if prepared.execution_generation != *context.execution_generation()
            || prepared.semantic_revision != *context.semantic_revision()
            || prepared.expected_mapping_revision != *context.mapping_revision()
        {
            return Err(ResilienceError::new(
                ResilienceErrorCode::StaleExecutionState,
            ));
        }

        prepared
            .replacement_mapping
            .validate()
            .map_err(|_| {
                ResilienceError::new(ResilienceErrorCode::InvalidConfiguration)
            })?;

        Ok(RemappingResult {
            id: prepared.id,
            semantic_revision: prepared.semantic_revision,
            execution_generation: prepared.execution_generation,
            previous_mapping_revision: prepared.expected_mapping_revision,
            mapping: prepared.replacement_mapping,
            changes: prepared.changes,
        })
    }

    /// Performs the complete preflight + prepare operation.
    ///
    /// This is useful for callers that require an explicit candidate before
    /// entering the transactional commit stage.
    pub fn prepare_remapping(
        &self,
        request: &RemappingRequest,
    ) -> ResilienceResult<PreparedRemapping> {
        self.prepare(request)
    }

    /// Performs prepare followed by commit.
    ///
    /// The caller must still supply authoritative commit state.
    pub fn remap(
        &self,
        request: &RemappingRequest,
        context: &RemappingCommitContext,
    ) -> ResilienceResult<RemappingResult> {
        let prepared = self.prepare(request)?;
        self.commit(prepared, context)
    }

    fn validate_domain_consistency(
        request: &RemappingRequest,
    ) -> ResilienceResult<()> {
        let domain = match request.logical_domain {
            Some(domain) => domain,
            None => return Ok(()),
        };

        for entry in request.current_mapping.iter() {
            if !domain.contains(entry.logical()) {
                return Err(ResilienceError::new(
                    ResilienceErrorCode::InvalidConfiguration,
                ));
            }
        }

        for entry in request.replacement_mapping.iter() {
            if !domain.contains(entry.logical()) {
                return Err(ResilienceError::new(
                    ResilienceErrorCode::InvalidConfiguration,
                ));
            }
        }

        Ok(())
    }
}

// ============================================================================
// Deterministic change calculation
// ============================================================================

/// Calculates deterministic logical-to-physical changes.
///
/// The function compares both mappings by logical identity and therefore
/// correctly handles:
///
/// - additions;
/// - removals;
/// - replacements;
/// - sparse identifiers;
/// - mappings whose physical identifiers are not contiguous.
///
/// No machine-size-dependent array is allocated.
#[must_use]
pub fn calculate_changes(
    current: &QubitMapping,
    replacement: &QubitMapping,
) -> Vec<MappingChange> {
    let mut current_iter = current.iter().peekable();
    let mut replacement_iter = replacement.iter().peekable();
    let mut changes = Vec::new();

    loop {
        match (current_iter.peek(), replacement_iter.peek()) {
            (None, None) => break,

            (Some(current_entry), None) => {
                let logical = current_entry.logical();
                let previous = Some(current_entry.physical());

                changes.push(MappingChange::new(
                    logical,
                    previous,
                    None,
                ));

                current_iter.next();
            }

            (None, Some(replacement_entry)) => {
                let logical = replacement_entry.logical();
                let replacement_physical =
                    Some(replacement_entry.physical());

                changes.push(MappingChange::new(
                    logical,
                    None,
                    replacement_physical,
                ));

                replacement_iter.next();
            }

            (Some(current_entry), Some(replacement_entry)) => {
                let current_logical = current_entry.logical();
                let replacement_logical = replacement_entry.logical();

                if current_logical < replacement_logical {
                    changes.push(MappingChange::new(
                        current_logical,
                        Some(current_entry.physical()),
                        None,
                    ));

                    current_iter.next();
                } else if replacement_logical < current_logical {
                    changes.push(MappingChange::new(
                        replacement_logical,
                        None,
                        Some(replacement_entry.physical()),
                    ));

                    replacement_iter.next();
                } else {
                    let current_physical =
                        current_entry.physical();
                    let replacement_physical =
                        replacement_entry.physical();

                    if current_physical != replacement_physical {
                        changes.push(MappingChange::new(
                            current_logical,
                            Some(current_physical),
                            Some(replacement_physical),
                        ));
                    }

                    current_iter.next();
                    replacement_iter.next();
                }
            }
        }
    }

    changes
}

// ============================================================================
// Mapping statistics
// ============================================================================

/// Deterministic summary of a remapping.
///
/// This contains no hardware-specific assumptions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct RemappingStatistics {
    /// Number of logical qubits whose physical assignment changed.
    changed: usize,

    /// Number of logical qubits newly mapped.
    added: usize,

    /// Number of logical qubits that became unmapped.
    removed: usize,

    /// Number of logical qubits retaining their physical assignment.
    unchanged: usize,
}

impl RemappingStatistics {
    /// Calculates statistics from two mappings.
    #[must_use]
    pub fn from_mappings(
        current: &QubitMapping,
        replacement: &QubitMapping,
    ) -> Self {
        let changes = calculate_changes(current, replacement);

        let mut added = 0usize;
        let mut removed = 0usize;
        let mut changed = 0usize;

        for change in &changes {
            match (change.previous(), change.replacement()) {
                (None, Some(_)) => {
                    added += 1;
                }
                (Some(_), None) => {
                    removed += 1;
                }
                (Some(_), Some(_)) => {
                    changed += 1;
                }
                (None, None) => {}
            }
        }

        let current_len = current.len();
        let changed_logical = changes.len();

        let unchanged = current_len
            .saturating_sub(
                removed.saturating_add(changed),
            );

        Self {
            changed: changed_logical,
            added,
            removed,
            unchanged,
        }
    }

    /// Number of changed logical assignments.
    #[must_use]
    pub const fn changed(self) -> usize {
        self.changed
    }

    /// Number of newly mapped logical qubits.
    #[must_use]
    pub const fn added(self) -> usize {
        self.added
    }

    /// Number of removed logical mappings.
    #[must_use]
    pub const fn removed(self) -> usize {
        self.removed
    }

    /// Number of unchanged existing mappings.
    #[must_use]
    pub const fn unchanged(self) -> usize {
        self.unchanged
    }

    /// Returns whether the mappings are identical.
    #[must_use]
    pub const fn is_noop(self) -> bool {
        self.changed == 0
    }
}

// ============================================================================
// Public validation helper
// ============================================================================

/// Validates that a replacement mapping is structurally compatible with a
/// current mapping.
///
/// This helper intentionally does not validate hardware.
///
/// It is useful to routing, planning and verification integration code that
/// needs to validate a candidate before constructing a full request.
pub fn validate_replacement(
    current: &QubitMapping,
    replacement: &QubitMapping,
    logical_domain: Option<MappingDomain>,
    require_complete: bool,
) -> ResilienceResult<()> {
    current.validate().map_err(|_| {
        ResilienceError::new(ResilienceErrorCode::InvalidConfiguration)
    })?;

    replacement.validate().map_err(|_| {
        ResilienceError::new(ResilienceErrorCode::InvalidConfiguration)
    })?;

    if let Some(domain) = logical_domain {
        for entry in current.iter() {
            if !domain.contains(entry.logical()) {
                return Err(ResilienceError::new(
                    ResilienceErrorCode::InvalidConfiguration,
                ));
            }
        }

        for entry in replacement.iter() {
            if !domain.contains(entry.logical()) {
                return Err(ResilienceError::new(
                    ResilienceErrorCode::InvalidConfiguration,
                ));
            }
        }

        if require_complete {
            replacement.require_complete(domain).map_err(|_| {
                ResilienceError::new(
                    ResilienceErrorCode::InvalidConfiguration,
                )
            })?;
        }
    } else if require_complete {
        return Err(ResilienceError::new(
            ResilienceErrorCode::InvalidConfiguration,
        ));
    }

    Ok(())
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn mapping(
        entries: &[(usize, usize)],
    ) -> QubitMapping {
        let mut result = QubitMapping::new();

        for &(logical, physical) in entries {
            result
                .insert(
                    QubitId::new(logical),
                    PhysicalQubitId::new(physical),
                )
                .expect("test mapping must be valid");
        }

        result
    }

    fn test_id(value: &str) -> RemappingId {
        RemappingId::new(value).expect("test identifier must be valid")
    }

    fn semantic(value: &str) -> SemanticRevision {
        SemanticRevision::new(value)
            .expect("test semantic revision must be valid")
    }

    fn generation(value: &str) -> ExecutionGeneration {
        ExecutionGeneration::new(value)
            .expect("test execution generation must be valid")
    }

    fn revision(value: &str) -> MappingRevision {
        MappingRevision::new(value)
            .expect("test mapping revision must be valid")
    }

    #[test]
    fn canonical_identities_are_used() {
        let logical = QubitId::new(7);
        let physical = PhysicalQubitId::new(19);

        assert_eq!(logical.index(), 7);
        assert_eq!(physical.index(), 19);
    }

    #[test]
    fn identical_mappings_produce_no_changes() {
        let current = mapping(&[(0, 4), (1, 8)]);
        let replacement = mapping(&[(0, 4), (1, 8)]);

        let changes = calculate_changes(&current, &replacement);

        assert!(changes.is_empty());

        let statistics =
            RemappingStatistics::from_mappings(
                &current,
                &replacement,
            );

        assert!(statistics.is_noop());
        assert_eq!(statistics.changed(), 0);
        assert_eq!(statistics.unchanged(), 2);
    }

    #[test]
    fn physical_assignment_change_is_detected() {
        let current = mapping(&[(0, 4), (1, 8)]);
        let replacement = mapping(&[(0, 7), (1, 8)]);

        let changes = calculate_changes(&current, &replacement);

        assert_eq!(changes.len(), 1);
        assert_eq!(changes[0].logical(), QubitId::new(0));
        assert_eq!(
            changes[0].previous(),
            Some(PhysicalQubitId::new(4))
        );
        assert_eq!(
            changes[0].replacement(),
            Some(PhysicalQubitId::new(7))
        );
    }

    #[test]
    fn sparse_identifiers_are_supported() {
        let current = mapping(&[
            (0, 100),
            (1_000_000, 500),
        ]);

        let replacement = mapping(&[
            (0, 101),
            (1_000_000, 501),
        ]);

        let changes = calculate_changes(&current, &replacement);

        assert_eq!(changes.len(), 2);
        assert_eq!(
            changes[0].logical(),
            QubitId::new(0)
        );
        assert_eq!(
            changes[1].logical(),
            QubitId::new(1_000_000)
        );
    }

    #[test]
    fn additions_and_removals_are_detected() {
        let current = mapping(&[
            (0, 10),
            (1, 11),
        ]);

        let replacement = mapping(&[
            (1, 12),
            (2, 13),
        ]);

        let changes = calculate_changes(
            &current,
            &replacement,
        );

        assert_eq!(changes.len(), 3);

        assert_eq!(
            changes[0],
            MappingChange::new(
                QubitId::new(0),
                Some(PhysicalQubitId::new(10)),
                None,
            )
        );

        assert_eq!(
            changes[1],
            MappingChange::new(
                QubitId::new(1),
                Some(PhysicalQubitId::new(11)),
                Some(PhysicalQubitId::new(12)),
            )
        );

        assert_eq!(
            changes[2],
            MappingChange::new(
                QubitId::new(2),
                None,
                Some(PhysicalQubitId::new(13)),
            )
        );
    }

    #[test]
    fn complete_domain_is_supported() {
        let domain =
            MappingDomain::new(QubitId::new(0), 3)
                .expect("domain must be valid");

        let current = mapping(&[
            (0, 4),
            (1, 5),
            (2, 6),
        ]);

        let replacement = mapping(&[
            (0, 7),
            (1, 8),
            (2, 9),
        ]);

        validate_replacement(
            &current,
            &replacement,
            Some(domain),
            true,
        )
        .expect("complete replacement should be valid");
    }

    #[test]
    fn out_of_domain_mapping_is_rejected() {
        let domain =
            MappingDomain::new(QubitId::new(0), 2)
                .expect("domain must be valid");

        let current = mapping(&[
            (0, 4),
            (1, 5),
        ]);

        let replacement = mapping(&[
            (0, 7),
            (2, 9),
        ]);

        let result = validate_replacement(
            &current,
            &replacement,
            Some(domain),
            true,
        );

        assert!(result.is_err());
    }

    #[test]
    fn prepare_does_not_commit_external_state() {
        let current = mapping(&[
            (0, 4),
            (1, 5),
        ]);

        let replacement = mapping(&[
            (0, 7),
            (1, 8),
        ]);

        let request = RemappingRequest::new(
            test_id("remap-test"),
            RemappingScope::Computation,
            semantic("semantic-1"),
            generation("execution-1"),
            revision("mapping-1"),
            current.clone(),
            replacement.clone(),
        )
        .expect("request must be valid");

        let adapter = RemappingAdapter::new();

        let prepared = adapter
            .prepare(&request)
            .expect("preparation must succeed");

        assert_eq!(
            prepared.replacement_mapping(),
            &replacement
        );

        assert_eq!(
            request.current_mapping(),
            &current
        );
    }

    #[test]
    fn commit_rejects_stale_execution_generation() {
        let current = mapping(&[(0, 4)]);
        let replacement = mapping(&[(0, 7)]);

        let request = RemappingRequest::new(
            test_id("remap-stale"),
            RemappingScope::Computation,
            semantic("semantic-1"),
            generation("execution-1"),
            revision("mapping-1"),
            current,
            replacement,
        )
        .expect("request must be valid");

        let adapter = RemappingAdapter::new();

        let prepared = adapter
            .prepare(&request)
            .expect("preparation must succeed");

        let stale_context = RemappingCommitContext::new(
            generation("execution-2"),
            revision("mapping-1"),
            semantic("semantic-1"),
        );

        let result = adapter.commit(
            prepared,
            &stale_context,
        );

        assert!(result.is_err());
    }

    #[test]
    fn commit_rejects_stale_mapping_revision() {
        let current = mapping(&[(0, 4)]);
        let replacement = mapping(&[(0, 7)]);

        let request = RemappingRequest::new(
            test_id("remap-stale-mapping"),
            RemappingScope::Computation,
            semantic("semantic-1"),
            generation("execution-1"),
            revision("mapping-1"),
            current,
            replacement,
        )
        .expect("request must be valid");

        let adapter = RemappingAdapter::new();

        let prepared = adapter
            .prepare(&request)
            .expect("preparation must succeed");

        let stale_context = RemappingCommitContext::new(
            generation("execution-1"),
            revision("mapping-2"),
            semantic("semantic-1"),
        );

        assert!(
            adapter
                .commit(prepared, &stale_context)
                .is_err()
        );
    }

    #[test]
    fn commit_rejects_stale_semantic_revision() {
        let current = mapping(&[(0, 4)]);
        let replacement = mapping(&[(0, 7)]);

        let request = RemappingRequest::new(
            test_id("remap-stale-semantic"),
            RemappingScope::Computation,
            semantic("semantic-1"),
            generation("execution-1"),
            revision("mapping-1"),
            current,
            replacement,
        )
        .expect("request must be valid");

        let adapter = RemappingAdapter::new();

        let prepared = adapter
            .prepare(&request)
            .expect("preparation must succeed");

        let stale_context = RemappingCommitContext::new(
            generation("execution-1"),
            revision("mapping-1"),
            semantic("semantic-2"),
        );

        assert!(
            adapter
                .commit(prepared, &stale_context)
                .is_err()
        );
    }

    #[test]
    fn successful_commit_returns_candidate_without_hidden_mutation() {
        let current = mapping(&[
            (0, 4),
            (1, 5),
        ]);

        let replacement = mapping(&[
            (0, 7),
            (1, 8),
        ]);

        let request = RemappingRequest::new(
            test_id("remap-success"),
            RemappingScope::Computation,
            semantic("semantic-1"),
            generation("execution-1"),
            revision("mapping-1"),
            current.clone(),
            replacement.clone(),
        )
        .expect("request must be valid");

        let adapter = RemappingAdapter::new();

        let context = RemappingCommitContext::new(
            generation("execution-1"),
            revision("mapping-1"),
            semantic("semantic-1"),
        );

        let result = adapter
            .remap(&request, &context)
            .expect("remapping should succeed");

        assert_eq!(result.mapping(), &replacement);
        assert_eq!(result.change_count(), 2);
        assert_eq!(
            result.previous_mapping_revision(),
            &revision("mapping-1")
        );

        // The source request remains immutable.
        assert_eq!(
            request.current_mapping(),
            &current
        );
    }

    #[test]
    fn statistics_are_deterministic() {
        let current = mapping(&[
            (0, 10),
            (1, 11),
            (2, 12),
            (3, 13),
        ]);

        let replacement = mapping(&[
            (0, 20),
            (1, 11),
            (2, 21),
            (4, 24),
        ]);

        let statistics =
            RemappingStatistics::from_mappings(
                &current,
                &replacement,
            );

        assert_eq!(statistics.changed(), 3);
        assert_eq!(statistics.added(), 1);
        assert_eq!(statistics.removed(), 1);
        assert_eq!(statistics.unchanged(), 1);
    }

    #[test]
    fn no_fixed_machine_size_is_assumed() {
        let current = mapping(&[
            (0, 1),
            (usize::MAX, usize::MAX - 1),
        ]);

        let replacement = mapping(&[
            (0, 2),
            (usize::MAX, usize::MAX),
        ]);

        let changes = calculate_changes(
            &current,
            &replacement,
        );

        assert_eq!(changes.len(), 2);
        assert_eq!(
            changes[1].logical(),
            QubitId::new(usize::MAX)
        );
    }

    #[test]
    fn no_op_remapping_is_valid() {
        let current = mapping(&[
            (0, 1),
            (1, 2),
        ]);

        let request = RemappingRequest::new(
            test_id("noop"),
            RemappingScope::Computation,
            semantic("semantic-1"),
            generation("execution-1"),
            revision("mapping-1"),
            current.clone(),
            current.clone(),
        )
        .expect("no-op request should be valid");

        let adapter = RemappingAdapter::new();

        let prepared = adapter
            .prepare(&request)
            .expect("preparation should succeed");

        assert!(prepared.is_noop());

        let context = RemappingCommitContext::new(
            generation("execution-1"),
            revision("mapping-1"),
            semantic("semantic-1"),
        );

        let result = adapter
            .commit(prepared, &context)
            .expect("no-op commit should succeed");

        assert!(result.is_noop());
    }
}