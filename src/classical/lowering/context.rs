#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani Classical Lowering Context
//!
//! Path: `src/classical/lowering/context.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Provides deterministic, target-independent context for classical lowering.
//! It records representation-entity mappings, symbolic capability and
//! resource requirements, provenance, lowering decisions, and optional
//! operational work accounting.
//!
//! This is a bookkeeping and contract layer. It does not perform lowering
//! algorithms, construct IR nodes, or establish that a transformation is
//! semantically legal.
//!
//! ## Architectural authority
//!
//! - `grammar/specification/semantic-model.md` defines source semantics.
//! - `grammar/specification/compilation-model.md` defines compilation stages.
//! - `grammar/specification/scalability-model.md` defines scalability policy.
//! - `grammar/specification/poco-reaf.md` defines portability objectives.
//! - `src/zuir/ids.rs` owns canonical ZUIR identities.
//! - `src/zuir/lowering/context.rs` provides the corresponding ZUIR-layer
//!   lowering contract.
//! - `src/quantum/ir/classical/` remains the canonical classical boundary
//!   inside the quantum IR.
//! - `src/classical/error.rs` owns the shared classical error vocabulary.
//! - `src/classical/diagnostics.rs` owns classical diagnostic integration.
//!
//! ## Ownership
//!
//! This file owns only classical-lowering context records and their
//! invariants. It does not own ASTs, semantic types, canonical IR nodes,
//! source-map storage, compiler diagnostics, target capabilities, physical
//! resource allocation, optimization algorithms, or execution.
//!
//! ## Dependency policy
//!
//! This implementation uses the Rust standard library and does not assume
//! unfinished sibling APIs. This keeps the context independently testable.
//! Integration adapters must explicitly bridge its opaque entity identities,
//! provenance, requirements, and notes to canonical repository types.
//!
//! Entity keys are adapter-level identities, not replacements for
//! `crate::zuir::ids` or any domain-specific canonical ID.
//!
//! ## Scalability and POCO-REAF
//!
//! There are no fixed semantic ceilings for operations, values, functions,
//! devices, processors, arrays, tensors, or other computational entities.
//! Collections grow according to available memory. An optional work budget
//! is an explicit operational policy, not a language-semantic limit.
//!
//! No context can promise literal infinity: address-space limits, allocator
//! failures, and available resources remain real constraints.
//!
//! Requirements are symbolic. This module does not allocate resources or
//! infer that a target supports a requested capability.
//!
//! ## Integration
//!
//! - `src/classical/lowering/mod.rs` declares `pub mod context;`.
//! - `src/classical/lowering/api.rs` should orchestrate lowering operations.
//! - `src/classical/lowering/legality.rs` should own legality analysis.
//! - `src/classical/lowering/diagnostics.rs` should adapt `LoweringNote`
//!   and `LoweringContextError` to canonical diagnostics and errors.
//! - `src/classical/lowering/provenance.rs` should resolve opaque provenance
//!   into `crate::source_map::Span` and source-map identities.
//! - Domain adapters should map source entities to destination entities
//!   only after validating the applicable semantic contracts.
//! - `src/zuir/lowering/` and canonical IR builders remain authoritative for
//!   ZUIR-specific identities, verification, and representation construction.
//! - Scheduling, placement, hardware allocation, and runtime remain
//!   downstream responsibilities.
//!
//! ## Freeze criteria
//!
//! Mapping updates are deterministic and idempotent. Entity-kind mismatches
//! are rejected. Provenance supports multiple origins. Resource and
//! capability requirements remain symbolic. Budget exhaustion is explicit
//! and leaves the budget unchanged. Transactions roll back all context
//! mutations on failure.
//!
//! Passing these local tests does not establish whole-repository integration.

use core::fmt;
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;

/// Result type for operations on the classical lowering context.
pub type LoweringContextResult<T> = Result<T, LoweringContextError>;

/// A representation entity category used to validate lowering mappings.
///
/// These are logical representation categories, not hardware resources.
/// Future or extension-defined categories use `Extension`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum EntityKind {
    /// A program.
    Program,
    /// A module.
    Module,
    /// A function or callable.
    Function,
    /// A nested region.
    Region,
    /// A basic block.
    Block,
    /// An operation or instruction.
    Operation,
    /// An SSA value or equivalent represented value.
    Value,
    /// A type declaration or type entity.
    Type,
    /// A resolved symbol.
    Symbol,
    /// A source artifact.
    Source,
    /// An extension-defined entity category.
    Extension(String),
}

impl EntityKind {
    fn validate(&self) -> LoweringContextResult<()> {
        if let Self::Extension(name) = self {
            validate_text(name).map_err(|_| LoweringContextError::InvalidEntityKind)?;
        }
        Ok(())
    }
}

/// A qualified, opaque identity for an entity in one representation.
///
/// The namespace identifies the representation or identity scope. The value
/// is opaque and must not be interpreted as an array index, memory address,
/// hardware identifier, source offset, or stable content fingerprint.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EntityKey {
    kind: EntityKind,
    namespace: String,
    value: String,
}

impl EntityKey {
    /// Constructs an entity key after validating its category and identity.
    pub fn new(
        kind: EntityKind,
        namespace: impl Into<String>,
        value: impl Into<String>,
    ) -> LoweringContextResult<Self> {
        kind.validate()?;
        let namespace = namespace.into();
        let value = value.into();
        validate_text(&namespace).map_err(|_| LoweringContextError::EmptyNamespace)?;
        validate_text(&value).map_err(|_| LoweringContextError::EmptyEntityIdentity)?;

        Ok(Self {
            kind,
            namespace,
            value,
        })
    }

    /// Returns the representation entity category.
    #[must_use]
    pub fn kind(&self) -> &EntityKind {
        &self.kind
    }

    /// Returns the representation namespace.
    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Returns the opaque entity identity.
    #[must_use]
    pub fn value(&self) -> &str {
        &self.value
    }
}

/// A relation for a symbolic resource requirement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RequirementRelation {
    /// Available capacity must be at least the requested quantity.
    AtLeast,
    /// Available capacity must be at most the requested quantity.
    AtMost,
    /// Available capacity must equal the requested quantity.
    Equal,
    /// The resource or facility must be supported; no quantity is asserted.
    Supported,
}

/// A symbolic resource requirement.
///
/// Quantity expressions are deliberately opaque. Their grammar, evaluation,
/// feasibility, and target-specific interpretation belong to resource
/// analysis, not to this context.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ResourceRequirement {
    dimension: String,
    relation: RequirementRelation,
    quantity: Option<String>,
    rationale: Option<String>,
}

impl ResourceRequirement {
    /// Constructs a valid symbolic resource requirement.
    ///
    /// `Supported` requires no quantity. Every numeric relation requires a
    /// non-empty symbolic quantity expression.
    pub fn new(
        dimension: impl Into<String>,
        relation: RequirementRelation,
        quantity: Option<String>,
        rationale: Option<String>,
    ) -> LoweringContextResult<Self> {
        let dimension = dimension.into();
        validate_text(&dimension).map_err(|_| LoweringContextError::InvalidRequirement)?;

        match (relation, quantity.as_deref()) {
            (RequirementRelation::Supported, None) => {}
            (RequirementRelation::Supported, Some(_)) => {
                return Err(LoweringContextError::InvalidRequirement);
            }
            (_, Some(expression)) => {
                validate_text(expression)
                    .map_err(|_| LoweringContextError::InvalidRequirement)?;
            }
            (_, None) => return Err(LoweringContextError::InvalidRequirement),
        }

        let rationale = normalize_optional_text(rationale)?;

        Ok(Self {
            dimension,
            relation,
            quantity,
            rationale,
        })
    }

    /// Returns the resource dimension, such as `memory` or `qubits`.
    #[must_use]
    pub fn dimension(&self) -> &str {
        &self.dimension
    }

    /// Returns the required relation.
    #[must_use]
    pub fn relation(&self) -> RequirementRelation {
        self.relation
    }

    /// Returns the symbolic quantity, if present.
    #[must_use]
    pub fn quantity(&self) -> Option<&str> {
        self.quantity.as_deref()
    }

    /// Returns the optional rationale.
    #[must_use]
    pub fn rationale(&self) -> Option<&str> {
        self.rationale.as_deref()
    }
}

/// A symbolic capability requirement.
///
/// This records intent only. It does not assert that any target provides the
/// capability. Names are open-ended so future capability namespaces do not
/// require changes to this enum.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CapabilityRequirement {
    identifier: String,
    rationale: Option<String>,
}

impl CapabilityRequirement {
    /// Constructs a capability requirement with a qualified identifier.
    pub fn new(
        identifier: impl Into<String>,
        rationale: Option<String>,
    ) -> LoweringContextResult<Self> {
        let identifier = identifier.into();
        validate_text(&identifier)
            .map_err(|_| LoweringContextError::InvalidCapability)?;
        let rationale = normalize_optional_text(rationale)?;

        Ok(Self {
            identifier,
            rationale,
        })
    }

    /// Returns the capability identifier.
    #[must_use]
    pub fn identifier(&self) -> &str {
        &self.identifier
    }

    /// Returns the optional rationale.
    #[must_use]
    pub fn rationale(&self) -> Option<&str> {
        self.rationale.as_deref()
    }
}

/// A source origin associated with a lowering entity.
///
/// Source identity and coordinates are opaque at this layer. The provenance
/// adapter must resolve them against the canonical source map before treating
/// them as source-map identifiers or spans.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProvenanceOrigin {
    source: EntityKey,
    byte_range: Option<(u64, u64)>,
    description: Option<String>,
}

impl ProvenanceOrigin {
    /// Constructs a source origin with an optional half-open byte range.
    ///
    /// The end offset may equal the start offset for an empty source range.
    pub fn new(
        source: EntityKey,
        byte_range: Option<(u64, u64)>,
        description: Option<String>,
    ) -> LoweringContextResult<Self> {
        if source.kind() != &EntityKind::Source {
            return Err(LoweringContextError::InvalidProvenanceSource);
        }

        if let Some((start, end)) = byte_range {
            if end < start {
                return Err(LoweringContextError::InvalidSourceRange);
            }
        }

        let description = normalize_optional_text(description)?;

        Ok(Self {
            source,
            byte_range,
            description,
        })
    }

    /// Returns the source entity key.
    #[must_use]
    pub fn source(&self) -> &EntityKey {
        &self.source
    }

    /// Returns the optional half-open byte range.
    #[must_use]
    pub fn byte_range(&self) -> Option<(u64, u64)> {
        self.byte_range
    }

    /// Returns the optional description.
    #[must_use]
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
}

/// A disposition recorded by the lowering context.
///
/// A disposition is evidence or intent supplied by an adapter. Merely
/// recording `Legal` does not prove semantic preservation; the legality
/// analysis and verifier remain authoritative.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum LoweringDisposition {
    /// The adapter reports that direct lowering is legal under its contract.
    Legal,
    /// Additional conditions must be proven before emission.
    Conditional {
        /// Conditions that must be checked.
        conditions: Vec<String>,
    },
    /// The source entity requires a semantic-preserving expansion.
    ExpansionRequired {
        /// Stable description of the expansion strategy.
        strategy: String,
    },
    /// This path cannot represent the required semantics.
    Unsupported {
        /// Explanation suitable for diagnostic adaptation.
        reason: String,
    },
}

impl LoweringDisposition {
    /// Returns whether this disposition alone permits direct lowering.
    ///
    /// The result is not a substitute for canonical legality verification.
    #[must_use]
    pub fn permits_direct_lowering(&self) -> bool {
        matches!(self, Self::Legal)
    }

    /// Returns whether further analysis or transformation is required.
    #[must_use]
    pub fn requires_follow_up(&self) -> bool {
        matches!(
            self,
            Self::Conditional { .. } | Self::ExpansionRequired { .. }
        )
    }
}

/// A structured note recorded during lowering.
///
/// Codes are stable machine-readable identifiers; messages are for humans.
/// The context stores notes deterministically and does not render diagnostics.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct LoweringNote {
    code: String,
    message: String,
    subject: Option<EntityKey>,
}

impl LoweringNote {
    /// Constructs a note with a non-empty code and message.
    pub fn new(
        code: impl Into<String>,
        message: impl Into<String>,
        subject: Option<EntityKey>,
    ) -> LoweringContextResult<Self> {
        let code = code.into();
        let message = message.into();
        validate_text(&code).map_err(|_| LoweringContextError::InvalidNote)?;
        validate_text(&message).map_err(|_| LoweringContextError::InvalidNote)?;

        Ok(Self {
            code,
            message,
            subject,
        })
    }

    /// Returns the machine-readable note code.
    #[must_use]
    pub fn code(&self) -> &str {
        &self.code
    }

    /// Returns the human-readable note message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns the optional subject entity.
    #[must_use]
    pub fn subject(&self) -> Option<&EntityKey> {
        self.subject.as_ref()
    }
}

/// Explicit operational policy for a lowering context.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoweringPolicy {
    /// Optional budget for explicitly charged work units.
    ///
    /// `None` means this policy imposes no work-unit budget. It does not
    /// guarantee unlimited time, memory, or operating-system resources.
    pub work_budget: Option<u64>,

    /// Whether the caller requires the source representation to have been
    /// verified before lowering begins.
    ///
    /// The caller must enforce this requirement using the canonical verifier.
    pub require_verified_source: bool,

    /// Whether the caller should stop immediately when a conversion is
    /// reported unsupported.
    ///
    /// Setting this to `false` permits continued analysis only; it never
    /// permits silently dropping semantics or claiming successful lowering.
    pub fail_on_unsupported: bool,
}

impl Default for LoweringPolicy {
    fn default() -> Self {
        Self {
            work_budget: None,
            require_verified_source: true,
            fail_on_unsupported: true,
        }
    }
}

/// A validated mapping between one source entity and its destination entities.
///
/// Multiple destination entities are allowed because a source operation may
/// lower into a sequence or structured expansion. Both ends retain their
/// representation namespaces and logical entity categories.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntityMapping {
    source: EntityKey,
    destinations: BTreeSet<EntityKey>,
}

impl EntityMapping {
    /// Returns the source entity.
    #[must_use]
    pub fn source(&self) -> &EntityKey {
        &self.source
    }

    /// Returns destination entities in deterministic order.
    #[must_use]
    pub fn destinations(&self) -> &BTreeSet<EntityKey> {
        &self.destinations
    }
}

/// Errors raised by local lowering-context contract validation.
///
/// The shared classical error layer remains responsible for subsystem-wide
/// error classification. The lowering diagnostics adapter should translate
/// these precise errors into that shared error and diagnostic vocabulary.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum LoweringContextError {
    /// A namespace is empty or contains only whitespace.
    EmptyNamespace,
    /// An entity identity is empty or contains only whitespace.
    EmptyEntityIdentity,
    /// An entity kind is malformed.
    InvalidEntityKind,
    /// An identity contains a forbidden NUL character.
    InvalidIdentity,
    /// Source and destination entity kinds do not match.
    EntityKindMismatch {
        /// Kind of the source entity.
        source: EntityKind,
        /// Kind of the destination entity.
        destination: EntityKind,
    },
    /// A symbolic resource requirement is malformed.
    InvalidRequirement,
    /// A capability identifier is malformed.
    InvalidCapability,
    /// A provenance source is not a source entity.
    InvalidProvenanceSource,
    /// A source range ends before it begins.
    InvalidSourceRange,
    /// A note has an empty code or message.
    InvalidNote,
    /// A disposition contains an invalid condition or explanation.
    InvalidDisposition,
    /// Adding work would exceed the configured operational budget.
    WorkBudgetExceeded {
        /// Previously charged work.
        consumed: u64,
        /// Requested additional work.
        requested: u64,
        /// Configured budget.
        budget: u64,
    },
    /// Work accounting exceeded the representable counter range.
    WorkCounterOverflow,
}

impl fmt::Display for LoweringContextError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyNamespace => f.write_str("lowering entity namespace is empty"),
            Self::EmptyEntityIdentity => f.write_str("lowering entity identity is empty"),
            Self::InvalidEntityKind => f.write_str("lowering entity kind is invalid"),
            Self::InvalidIdentity => f.write_str("lowering identity contains a NUL character"),
            Self::EntityKindMismatch { source, destination } => write!(
                f,
                "lowering entity kinds differ: source {source:?}, destination {destination:?}"
            ),
            Self::InvalidRequirement => f.write_str("symbolic resource requirement is invalid"),
            Self::InvalidCapability => f.write_str("capability requirement is invalid"),
            Self::InvalidProvenanceSource => {
                f.write_str("provenance must reference a source entity")
            }
            Self::InvalidSourceRange => {
                f.write_str("provenance source range ends before it begins")
            }
            Self::InvalidNote => f.write_str("lowering note requires a code and message"),
            Self::InvalidDisposition => {
                f.write_str("lowering disposition contains invalid text")
            }
            Self::WorkBudgetExceeded {
                consumed,
                requested,
                budget,
            } => write!(
                f,
                "lowering work budget exceeded: consumed {consumed}, \
                 requested {requested}, budget {budget}"
            ),
            Self::WorkCounterOverflow => {
                f.write_str("lowering work counter exceeded its representable range")
            }
        }
    }
}

impl Error for LoweringContextError {}

/// Deterministic, target-independent context for classical lowering.
///
/// All collections use ordered structures so iteration and serialized
/// adapter output can be deterministic. The context is owned by one lowering
/// operation; it contains no global or process-wide mutable state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoweringContext {
    policy: LoweringPolicy,
    consumed_work: u64,
    mappings: BTreeMap<EntityKey, BTreeSet<EntityKey>>,
    requirements: BTreeSet<ResourceRequirement>,
    capabilities: BTreeSet<CapabilityRequirement>,
    provenance: BTreeMap<EntityKey, BTreeSet<ProvenanceOrigin>>,
    dispositions: BTreeMap<EntityKey, LoweringDisposition>,
    notes: BTreeSet<LoweringNote>,
}

impl LoweringContext {
    /// Creates an empty context with the supplied operational policy.
    #[must_use]
    pub fn new(policy: LoweringPolicy) -> Self {
        Self {
            policy,
            consumed_work: 0,
            mappings: BTreeMap::new(),
            requirements: BTreeSet::new(),
            capabilities: BTreeSet::new(),
            provenance: BTreeMap::new(),
            dispositions: BTreeMap::new(),
            notes: BTreeSet::new(),
        }
    }

    /// Creates an empty context with the default policy.
    #[must_use]
    pub fn with_defaults() -> Self {
        Self::new(LoweringPolicy::default())
    }

    /// Returns the immutable lowering policy.
    #[must_use]
    pub fn policy(&self) -> &LoweringPolicy {
        &self.policy
    }

    /// Returns the amount of work explicitly charged so far.
    #[must_use]
    pub fn consumed_work(&self) -> u64 {
        self.consumed_work
    }

    /// Returns the optional configured work budget.
    #[must_use]
    pub fn work_budget(&self) -> Option<u64> {
        self.policy.work_budget
    }

    /// Charges explicitly measured work against the configured budget.
    ///
    /// The context is unchanged if the charge would exceed the budget or
    /// overflow the work counter. A charge of zero is valid.
    pub fn charge_work(&mut self, amount: u64) -> LoweringContextResult<()> {
        let next = self
            .consumed_work
            .checked_add(amount)
            .ok_or(LoweringContextError::WorkCounterOverflow)?;

        if let Some(budget) = self.policy.work_budget {
            if next > budget {
                return Err(LoweringContextError::WorkBudgetExceeded {
                    consumed: self.consumed_work,
                    requested: amount,
                    budget,
                });
            }
        }

        self.consumed_work = next;
        Ok(())
    }

    /// Adds or confirms a source-to-destination entity association.
    ///
    /// Repeating the same association is idempotent. A source may map to
    /// multiple destinations, but a mapping may not change entity category.
    pub fn map_entity(
        &mut self,
        source: EntityKey,
        destination: EntityKey,
    ) -> LoweringContextResult<()> {
        if source.kind() != destination.kind() {
            return Err(LoweringContextError::EntityKindMismatch {
                source: source.kind().clone(),
                destination: destination.kind().clone(),
            });
        }

        self.mappings
            .entry(source)
            .or_default()
            .insert(destination);
        Ok(())
    }

    /// Returns a mapping for a source entity, if one exists.
    #[must_use]
    pub fn mapping(&self, source: &EntityKey) -> Option<EntityMapping> {
        self.mappings.get(source).map(|destinations| EntityMapping {
            source: source.clone(),
            destinations: destinations.clone(),
        })
    }

    /// Returns all mappings in deterministic source order.
    #[must_use]
    pub fn mappings(&self) -> &BTreeMap<EntityKey, BTreeSet<EntityKey>> {
        &self.mappings
    }

    /// Adds a symbolic resource requirement.
    ///
    /// Duplicate requirements are idempotent.
    pub fn add_requirement(&mut self, requirement: ResourceRequirement) {
        self.requirements.insert(requirement);
    }

    /// Returns symbolic resource requirements in deterministic order.
    #[must_use]
    pub fn requirements(&self) -> &BTreeSet<ResourceRequirement> {
        &self.requirements
    }

    /// Adds a symbolic capability requirement.
    ///
    /// Duplicate requirements are idempotent.
    pub fn require_capability(&mut self, requirement: CapabilityRequirement) {
        self.capabilities.insert(requirement);
    }

    /// Returns capability requirements in deterministic order.
    #[must_use]
    pub fn capabilities(&self) -> &BTreeSet<CapabilityRequirement> {
        &self.capabilities
    }

    /// Associates a source origin with a lowered entity.
    ///
    /// Multiple origins are supported for inlining, expansion, fusion, and
    /// other transformations where one destination entity has several
    /// contributing source locations. Duplicate origins are idempotent.
    pub fn add_provenance(
        &mut self,
        entity: EntityKey,
        origin: ProvenanceOrigin,
    ) {
        self.provenance.entry(entity).or_default().insert(origin);
    }

    /// Returns provenance origins for an entity, if any.
    #[must_use]
    pub fn provenance(
        &self,
        entity: &EntityKey,
    ) -> Option<&BTreeSet<ProvenanceOrigin>> {
        self.provenance.get(entity)
    }

    /// Returns all provenance associations in deterministic entity order.
    #[must_use]
    pub fn all_provenance(
        &self,
    ) -> &BTreeMap<EntityKey, BTreeSet<ProvenanceOrigin>> {
        &self.provenance
    }

    /// Records a lowering disposition for an entity.
    ///
    /// A later record replaces the previous record for that entity. The
    /// caller must run legality analysis before treating `Legal` as authority
    /// to emit destination IR.
    pub fn set_disposition(
        &mut self,
        entity: EntityKey,
        disposition: LoweringDisposition,
    ) -> LoweringContextResult<()> {
        validate_disposition(&disposition)?;
        self.dispositions.insert(entity, disposition);
        Ok(())
    }

    /// Returns the recorded disposition for an entity, if present.
    #[must_use]
    pub fn disposition(
        &self,
        entity: &EntityKey,
    ) -> Option<&LoweringDisposition> {
        self.dispositions.get(entity)
    }

    /// Returns all recorded dispositions in deterministic entity order.
    #[must_use]
    pub fn dispositions(&self) -> &BTreeMap<EntityKey, LoweringDisposition> {
        &self.dispositions
    }

    /// Adds a structured lowering note.
    ///
    /// Duplicate notes are idempotent. Diagnostic rendering and conversion
    /// to the shared classical error vocabulary remain adapter responsibilities.
    pub fn add_note(&mut self, note: LoweringNote) {
        self.notes.insert(note);
    }

    /// Returns lowering notes in deterministic order.
    #[must_use]
    pub fn notes(&self) -> &BTreeSet<LoweringNote> {
        &self.notes
    }

    /// Runs a context update transactionally.
    ///
    /// The closure operates on a working copy. On success, the updated copy
    /// replaces this context; on error, every mutation is rolled back.
    ///
    /// This gives atomic context bookkeeping, not atomic backend I/O or IR
    /// mutation. The closure must not perform irreversible external effects.
    pub fn transaction<T, E>(
        &mut self,
        operation: impl FnOnce(&mut Self) -> Result<T, E>,
    ) -> Result<T, E> {
        let mut working = self.clone();
        match operation(&mut working) {
            Ok(value) => {
                *self = working;
                Ok(value)
            }
            Err(error) => Err(error),
        }
    }
}

impl Default for LoweringContext {
    fn default() -> Self {
        Self::with_defaults()
    }
}

fn validate_text(value: &str) -> Result<(), ()> {
    if value.trim().is_empty() || value.contains('\0') {
        Err(())
    } else {
        Ok(())
    }
}

fn normalize_optional_text(value: Option<String>) -> LoweringContextResult<Option<String>> {
    value
        .map(|text| {
            validate_text(&text).map_err(|_| LoweringContextError::InvalidIdentity)?;
            Ok(text.trim().to_owned())
        })
        .transpose()
}

fn validate_disposition(
    disposition: &LoweringDisposition,
) -> LoweringContextResult<()> {
    match disposition {
        LoweringDisposition::Legal => Ok(()),
        LoweringDisposition::Conditional { conditions } => {
            if conditions.is_empty() {
                return Err(LoweringContextError::InvalidDisposition);
            }
            for condition in conditions {
                validate_text(condition)
                    .map_err(|_| LoweringContextError::InvalidDisposition)?;
            }
            Ok(())
        }
        LoweringDisposition::ExpansionRequired { strategy } => {
            validate_text(strategy)
                .map_err(|_| LoweringContextError::InvalidDisposition)
        }
        LoweringDisposition::Unsupported { reason } => {
            validate_text(reason).map_err(|_| LoweringContextError::InvalidDisposition)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn key(kind: EntityKind, namespace: &str, value: &str) -> EntityKey {
        EntityKey::new(kind, namespace, value).expect("valid test entity")
    }

    #[test]
    fn rejects_empty_and_nul_identities() {
        assert!(matches!(
            EntityKey::new(EntityKind::Operation, " ", "op"),
            Err(LoweringContextError::EmptyNamespace)
        ));
        assert!(matches!(
            EntityKey::new(EntityKind::Value, "source", ""),
            Err(LoweringContextError::EmptyEntityIdentity)
        ));
        assert!(matches!(
            EntityKey::new(EntityKind::Value, "source", "bad\0id"),
            Err(LoweringContextError::EmptyEntityIdentity)
        ));
    }

    #[test]
    fn mapping_is_idempotent_and_supports_expansion() {
        let source = key(EntityKind::Operation, "source-ir", "op-1");
        let first = key(EntityKind::Operation, "destination-ir", "op-2");
        let second = key(EntityKind::Operation, "destination-ir", "op-3");
        let mut context = LoweringContext::default();

        context
            .map_entity(source.clone(), first.clone())
            .expect("mapping is valid");
        context
            .map_entity(source.clone(), first.clone())
            .expect("duplicate mapping is valid");
        context
            .map_entity(source.clone(), second.clone())
            .expect("one-to-many mapping is valid");

        let mapping = context.mapping(&source).expect("mapping exists");
        assert_eq!(mapping.destinations().len(), 2);
        assert!(mapping.destinations().contains(&first));
        assert!(mapping.destinations().contains(&second));
    }

    #[test]
    fn rejects_entity_kind_mismatch_without_mutation() {
        let source = key(EntityKind::Operation, "source", "op");
        let destination = key(EntityKind::Value, "destination", "value");
        let mut context = LoweringContext::default();

        assert!(matches!(
            context.map_entity(source, destination),
            Err(LoweringContextError::EntityKindMismatch { .. })
        ));
        assert!(context.mappings().is_empty());
    }

    #[test]
    fn resource_requirements_remain_symbolic() {
        let requirement = ResourceRequirement::new(
            "memory",
            RequirementRelation::AtLeast,
            Some("required_bytes(program_size)".to_owned()),
            Some("Derived from the input workload".to_owned()),
        )
        .expect("valid symbolic requirement");

        assert_eq!(requirement.quantity(), Some("required_bytes(program_size)"));
        assert!(ResourceRequirement::new(
            "memory",
            RequirementRelation::AtLeast,
            None,
            None,
        )
        .is_err());
        assert!(ResourceRequirement::new(
            "memory",
            RequirementRelation::Supported,
            Some("1024".to_owned()),
            None,
        )
        .is_err());
    }

    #[test]
    fn capability_namespace_is_open_ended() {
        let capability = CapabilityRequirement::new(
            "future.accelerator.specialized_operation",
            None,
        )
        .expect("future capability is valid");

        assert_eq!(
            capability.identifier(),
            "future.accelerator.specialized_operation"
        );
    }

    #[test]
    fn provenance_supports_multiple_origins() {
        let entity = key(EntityKind::Operation, "target-ir", "op");
        let source = key(EntityKind::Source, "source-map", "file-a");
        let first = ProvenanceOrigin::new(
            source.clone(),
            Some((0, 4)),
            Some("first contribution".to_owned()),
        )
        .expect("valid origin");
        let second = ProvenanceOrigin::new(
            source,
            Some((8, 12)),
            Some("second contribution".to_owned()),
        )
        .expect("valid origin");

        let mut context = LoweringContext::default();
        context.add_provenance(entity.clone(), first);
        context.add_provenance(entity.clone(), second);

        assert_eq!(
            context.provenance(&entity).expect("origins exist").len(),
            2
        );
    }

    #[test]
    fn provenance_rejects_invalid_ranges_and_non_source_entities() {
        let not_source = key(EntityKind::Operation, "ir", "op");
        assert!(matches!(
            ProvenanceOrigin::new(not_source, None, None),
            Err(LoweringContextError::InvalidProvenanceSource)
        ));

        let source = key(EntityKind::Source, "source-map", "file");
        assert!(matches!(
            ProvenanceOrigin::new(source, Some((9, 2)), None),
            Err(LoweringContextError::InvalidSourceRange)
        ));
    }

    #[test]
    fn work_budget_failure_does_not_change_consumed_work() {
        let mut context = LoweringContext::new(LoweringPolicy {
            work_budget: Some(5),
            ..LoweringPolicy::default()
        });

        context.charge_work(4).expect("within budget");
        let error = context.charge_work(2).expect_err("budget must be enforced");

        assert!(matches!(
            error,
            LoweringContextError::WorkBudgetExceeded {
                consumed: 4,
                requested: 2,
                budget: 5
            }
        ));
        assert_eq!(context.consumed_work(), 4);
    }

    #[test]
    fn work_counter_overflow_is_reported_without_wrapping() {
        let mut context = LoweringContext::new(LoweringPolicy::default());
        context.consumed_work = u64::MAX;

        assert_eq!(
            context.charge_work(1),
            Err(LoweringContextError::WorkCounterOverflow)
        );
        assert_eq!(context.consumed_work(), u64::MAX);
    }

    #[test]
    fn transaction_commits_on_success() {
        let mut context = LoweringContext::default();
        let source = key(EntityKind::Operation, "source", "op");
        let destination = key(EntityKind::Operation, "target", "op");

        let result = context.transaction(|working| {
            working.map_entity(source.clone(), destination.clone())?;
            working.charge_work(3)?;
            Ok::<_, LoweringContextError>(())
        });

        assert!(result.is_ok());
        assert!(context.mapping(&source).is_some());
        assert_eq!(context.consumed_work(), 3);
    }

    #[test]
    fn transaction_rolls_back_every_mutation_on_error() {
        let mut context = LoweringContext::default();
        let source = key(EntityKind::Operation, "source", "op");
        let destination = key(EntityKind::Operation, "target", "op");

        let result = context.transaction(|working| {
            working.map_entity(source.clone(), destination.clone())?;
            working.charge_work(3)?;
            Err::<(), _>(LoweringContextError::InvalidNote)
        });

        assert_eq!(result, Err(LoweringContextError::InvalidNote));
        assert!(context.mappings().is_empty());
        assert_eq!(context.consumed_work(), 0);
    }

    #[test]
    fn dispositions_require_explanations() {
        let mut context = LoweringContext::default();
        let entity = key(EntityKind::Operation, "source", "op");

        assert!(matches!(
            context.set_disposition(
                entity.clone(),
                LoweringDisposition::Unsupported {
                    reason: " ".to_owned()
                }
            ),
            Err(LoweringContextError::InvalidDisposition)
        ));

        context
            .set_disposition(
                entity.clone(),
                LoweringDisposition::Legal,
            )
            .expect("valid disposition");

        assert!(
            context
                .disposition(&entity)
                .expect("disposition exists")
                .permits_direct_lowering()
        );
    }

    #[test]
    fn duplicate_requirements_and_notes_are_idempotent() {
        let requirement = ResourceRequirement::new(
            "memory",
            RequirementRelation::AtLeast,
            Some("input_size".to_owned()),
            None,
        )
        .expect("valid requirement");
        let note = LoweringNote::new("lowering.info", "Recorded", None)
            .expect("valid note");

        let mut context = LoweringContext::default();
        context.add_requirement(requirement.clone());
        context.add_requirement(requirement);
        context.add_note(note.clone());
        context.add_note(note);

        assert_eq!(context.requirements().len(), 1);
        assert_eq!(context.notes().len(), 1);
    }

    #[test]
    fn ordered_maps_make_iteration_deterministic() {
        let mut context = LoweringContext::default();
        let sources = ["z", "a", "m"];

        for source in sources {
            context
                .map_entity(
                    key(EntityKind::Value, "source", source),
                    key(EntityKind::Value, "target", source),
                )
                .expect("valid mapping");
        }

        let ordered: Vec<&str> = context
            .mappings()
            .keys()
            .map(EntityKey::value)
            .collect();

        assert_eq!(ordered, vec!["a", "m", "z"]);
    }

    #[test]
    fn dynamic_collections_have_no_semantic_count_limit() {
        let mut context = LoweringContext::default();

        // Exercise dynamic growth without defining a production capacity.
        for index in 0_u64..2_000 {
            context
                .map_entity(
                    key(
                        EntityKind::Operation,
                        "source",
                        &format!("operation-{index}"),
                    ),
                    key(
                        EntityKind::Operation,
                        "target",
                        &format!("operation-{index}"),
                    ),
                )
                .expect("valid dynamically sized mapping");
        }

        assert_eq!(context.mappings().len(), 2_000);
    }
}