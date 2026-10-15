
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # ZUIR Lowering Context
//!
//! Path: `src/zuir/lowering/context.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: unsafe Rust is prohibited.
//!
//! PURPOSE
//!
//! Provide shared, deterministic state and contracts for lowering between
//! ZUIR and domain-specific or target-specific representations.
//!
//! AUTHORITY
//!
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/ids.rs`
//! - `src/zuir/version.rs`
//! - `src/zuir/config.rs`
//! - `src/zuir/errors.rs`
//! - `src/zuir/diagnostics.rs`
//! - `src/zuir/source_map.rs`
//! - `src/zuir/ir/`
//! - `src/zuir/dialect/`
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/compilation-model.md`
//! - `grammar/specification/execution-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/poco-reaf.md`
//!
//! OWNS
//!
//! - Lowering-local identity mappings.
//! - Explicit lowering legality and disposition contracts.
//! - Symbolic capability and resource requirements.
//! - Lowering provenance associations.
//! - Deterministic lowering diagnostics.
//! - Optional operational work accounting.
//!
//! DOES NOT OWN
//!
//! - Source AST construction or semantic analysis.
//! - Canonical ZUIR node definitions or verification algorithms.
//! - Destination IR node definitions or builder implementations.
//! - Dialect registration or operation semantics.
//! - Physical resource allocation, quantum routing, calibration,
//!   HDL synthesis, target scheduling, code generation, or execution.
//! - Global compiler configuration or mutable global state.
//!
//! INTEGRATION
//!
//! `lowering/mod.rs` declares this module and exports only the intended API.
//! `lowering/api.rs` owns source/destination orchestration.
//! `lowering/legality.rs` owns full legality analysis.
//! `lowering/diagnostics.rs` adapts these records to canonical diagnostics.
//! `lowering/provenance.rs` adapts origins to the canonical source map.
//! Domain adapters use the mapping and requirement APIs while retaining
//! ownership of their destination builders and semantic conversions.
//!
//! This file deliberately uses a small, standalone contract vocabulary.
//! It does not assume that unavailable sibling APIs have particular names
//! or signatures. Adapter code must explicitly bridge these contracts to
//! the repository's canonical IDs, diagnostics, and IR builders.
//!
//! SCALABILITY
//!
//! Collections are dynamically sized and ordered. No universal machine,
//! operation, qubit, register, tensor-rank, node, or device limit is imposed.
//! Optional work budgets are operational safeguards, not semantic limits.
//! Finite address space, allocator capacity, and execution time remain real.
//!
//! POCO-REAF
//!
//! The context records logical requirements, not physical allocations.
//! Target-dependent decisions belong to downstream analyses. Unsupported
//! semantics must not be silently erased or approximated.
//!
//! FREEZE CRITERIA
//!
//! - Mapping is deterministic and supports one-to-many lowering.
//! - Duplicate destination associations are idempotent.
//! - Invalid mapping categories are rejected.
//! - Requirements and capabilities remain symbolic.
//! - Provenance supports multiple origins.
//! - Budget exhaustion is explicit and does not mutate the budget.
//! - Tests cover invalid inputs, determinism, and large dynamic collections.
//! - Integration adapters compile against actual repository APIs.
//!
//! STATUS
//!
//! Proposed standalone contract implementation. Do not mark integrated or
//! production-verified until the parent module, adapters, and repository
//! test suite compile and pass.

use core::fmt;
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;

/// A qualified entity category used to validate lowering mappings.
///
/// The categories describe representation entities, not hardware resources.
/// Extension-defined entity categories use `Extension`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum EntityKind {
    /// A program.
    Program,
    /// A module.
    Module,
    /// A function or callable.
    Function,
    /// A region.
    Region,
    /// A basic block.
    Block,
    /// An operation or instruction.
    Operation,
    /// A value or SSA result.
    Value,
    /// A type definition.
    Type,
    /// A symbol.
    Symbol,
    /// A dialect declaration.
    Dialect,
    /// A source document or imported artifact.
    Source,
    /// An extension-defined entity category.
    Extension(String),
}

/// Stable, namespace-qualified identity for a lowering entity.
///
/// The identity is local to the representation contract that creates it.
/// It is not a physical address, array index, allocation counter, or
/// cross-compilation content fingerprint.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EntityKey {
    kind: EntityKind,
    namespace: String,
    value: String,
}

impl EntityKey {
    /// Construct an entity key after validating its fields.
    pub fn new(
        kind: EntityKind,
        namespace: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self, LoweringContextError> {
        let namespace = namespace.into();
        let value = value.into();

        if namespace.trim().is_empty() {
            return Err(LoweringContextError::EmptyNamespace);
        }
        if value.trim().is_empty() {
            return Err(LoweringContextError::EmptyEntityIdentity);
        }
        if namespace.contains('\0') || value.contains('\0') {
            return Err(LoweringContextError::InvalidIdentity);
        }

        if let EntityKind::Extension(name) = &kind {
            if name.trim().is_empty() || name.contains('\0') {
                return Err(LoweringContextError::InvalidEntityKind);
            }
        }

        Ok(Self {
            kind,
            namespace,
            value,
        })
    }

    /// Return the entity category.
    #[must_use]
    pub fn kind(&self) -> &EntityKind {
        &self.kind
    }

    /// Return the representation namespace.
    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Return the opaque identity value.
    #[must_use]
    pub fn value(&self) -> &str {
        &self.value
    }
}

/// An explicit decision about whether and how an entity can be lowered.
///
/// This is a contract value. It does not itself prove that the destination
/// operation preserves the source operation's semantics.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum LoweringDisposition {
    /// Lowering is proven legal under the supplied contracts.
    Legal,
    /// Legality depends on conditions that must be checked explicitly.
    Conditional {
        /// Conditions that must be established before lowering succeeds.
        conditions: Vec<String>,
    },
    /// The source operation must be expanded into other operations.
    ExpansionRequired {
        /// A stable, qualified description of the required expansion.
        strategy: String,
    },
    /// This lowering path cannot represent the required semantics.
    Unsupported {
        /// Explanation suitable for diagnostics.
        reason: String,
    },
}

impl LoweringDisposition {
    /// Return whether the disposition authorizes direct lowering.
    ///
    /// Conditional, expansion-required, and unsupported decisions are not
    /// direct authorization to emit a destination operation.
    #[must_use]
    pub fn permits_direct_lowering(&self) -> bool {
        matches!(self, Self::Legal)
    }

    /// Return whether this disposition requires additional work.
    #[must_use]
    pub fn requires_follow_up(&self) -> bool {
        matches!(
            self,
            Self::Conditional { .. } | Self::ExpansionRequired { .. }
        )
    }
}

/// Relation used by a symbolic resource requirement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RequirementRelation {
    /// The available resource must be at least the requested quantity.
    AtLeast,
    /// The available resource must be at most the requested quantity.
    AtMost,
    /// The available resource must equal the requested quantity.
    Equal,
    /// The resource must be supported, without asserting a numeric quantity.
    Supported,
}

/// A symbolic resource requirement.
///
/// Quantities are expressed as validated, non-empty symbolic expressions
/// rather than host integers. Their grammar and evaluation belong to the
/// resource-analysis contract, not this context.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ResourceRequirement {
    dimension: String,
    relation: RequirementRelation,
    quantity: Option<String>,
    rationale: Option<String>,
}

impl ResourceRequirement {
    /// Construct a resource requirement.
    pub fn new(
        dimension: impl Into<String>,
        relation: RequirementRelation,
        quantity: Option<String>,
        rationale: Option<String>,
    ) -> Result<Self, LoweringContextError> {
        let dimension = dimension.into();
        if dimension.trim().is_empty() || dimension.contains('\0') {
            return Err(LoweringContextError::InvalidRequirement);
        }

        match (relation, quantity.as_deref()) {
            (RequirementRelation::Supported, None) => {}
            (RequirementRelation::Supported, Some(_)) => {
                return Err(LoweringContextError::InvalidRequirement);
            }
            (_, Some(value)) if !value.trim().is_empty() && !value.contains('\0') => {}
            (_, _) => return Err(LoweringContextError::InvalidRequirement),
        }

        let rationale = rationale
            .map(|value| value.trim().to_owned())
            .filter(|value| !value.is_empty());

        Ok(Self {
            dimension,
            relation,
            quantity,
            rationale,
        })
    }

    /// Return the resource dimension, such as `memory` or `qubits`.
    #[must_use]
    pub fn dimension(&self) -> &str {
        &self.dimension
    }

    /// Return the relation that must hold.
    #[must_use]
    pub fn relation(&self) -> RequirementRelation {
        self.relation
    }

    /// Return the symbolic quantity, if this relation requires one.
    #[must_use]
    pub fn quantity(&self) -> Option<&str> {
        self.quantity.as_deref()
    }

    /// Return the explanation, if supplied.
    #[must_use]
    pub fn rationale(&self) -> Option<&str> {
        self.rationale.as_deref()
    }
}

/// A source origin associated with a lowered entity.
///
/// Coordinates are opaque to this module. The canonical source-map adapter
/// validates source IDs and byte spans against the source-map contract.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProvenanceOrigin {
    source: EntityKey,
    start_byte: Option<u64>,
    end_byte: Option<u64>,
    description: Option<String>,
}

impl ProvenanceOrigin {
    /// Construct a source origin with an optional half-open byte range.
    pub fn new(
        source: EntityKey,
        start_byte: Option<u64>,
        end_byte: Option<u64>,
        description: Option<String>,
    ) -> Result<Self, LoweringContextError> {
        if source.kind() != &EntityKind::Source {
            return Err(LoweringContextError::InvalidProvenanceSource);
        }

        match (start_byte, end_byte) {
            (Some(start), Some(end)) if end < start => {
                return Err(LoweringContextError::InvalidSourceRange);
            }
            (Some(_), None) | (None, Some(_)) => {
                return Err(LoweringContextError::IncompleteSourceRange);
            }
            _ => {}
        }

        let description = description
            .map(|value| value.trim().to_owned())
            .filter(|value| !value.is_empty());

        Ok(Self {
            source,
            start_byte,
            end_byte,
            description,
        })
    }

    /// Return the source identity.
    #[must_use]
    pub fn source(&self) -> &EntityKey {
        &self.source
    }

    /// Return the source byte range, if available.
    #[must_use]
    pub fn byte_range(&self) -> Option<(u64, u64)> {
        self.start_byte.zip(self.end_byte)
    }

    /// Return the origin description, if available.
    #[must_use]
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
}

/// Operational policy for this lowering operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoweringPolicy {
    /// Optional maximum number of explicitly charged work units.
    ///
    /// `None` means that this policy imposes no work-unit budget; it does
    /// not promise infinite time, memory, or operating-system resources.
    pub work_budget: Option<u64>,

    /// Whether the caller requires a verified source representation.
    ///
    /// The caller must enforce this requirement through the canonical
    /// verifier before invoking a lowering adapter.
    pub require_verified_source: bool,

    /// Whether an unsupported conversion must fail immediately.
    ///
    /// This field does not authorize dropping semantics when false.
    /// A caller may continue analysis, but cannot claim successful lowering.
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

/// A deterministic diagnostic owned by this lowering context.
///
/// The canonical diagnostics adapter converts these records to the
/// repository-wide diagnostic representation.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct LoweringNote {
    code: String,
    message: String,
    subject: Option<EntityKey>,
}

impl LoweringNote {
    /// Construct a lowering diagnostic note.
    pub fn new(
        code: impl Into<String>,
        message: impl Into<String>,
        subject: Option<EntityKey>,
    ) -> Result<Self, LoweringContextError> {
        let code = code.into();
        let message = message.into();

        if code.trim().is_empty()
            || message.trim().is_empty()
            || code.contains('\0')
            || message.contains('\0')
        {
            return Err(LoweringContextError::InvalidDiagnostic);
        }

        Ok(Self {
            code,
            message,
            subject,
        })
    }

    /// Return the diagnostic code.
    #[must_use]
    pub fn code(&self) -> &str {
        &self.code
    }

    /// Return the diagnostic message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Return the associated entity, if any.
    #[must_use]
    pub fn subject(&self) -> Option<&EntityKey> {
        self.subject.as_ref()
    }
}

/// Errors produced by the local lowering-context contract.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum LoweringContextError {
    /// A namespace was empty or invalid.
    EmptyNamespace,
    /// An entity identity was empty.
    EmptyEntityIdentity,
    /// An entity identity contained invalid data.
    InvalidIdentity,
    /// An entity kind was invalid.
    InvalidEntityKind,
    /// Source and destination categories were incompatible.
    EntityKindMismatch {
        /// The source category.
        source: EntityKind,
        /// The destination category.
        destination: EntityKind,
    },
    /// A resource requirement was malformed.
    InvalidRequirement,
    /// A provenance source was not a source entity.
    InvalidProvenanceSource,
    /// A source range ended before it started.
    InvalidSourceRange,
    /// Only one endpoint of a source range was supplied.
    IncompleteSourceRange,
    /// A diagnostic record was malformed.
    InvalidDiagnostic,
    /// A lowering strategy name was invalid.
    InvalidStrategy,
    /// An attempt was made to charge work beyond the configured budget.
    WorkBudgetExceeded {
        /// Work already charged.
        consumed: u64,
        /// Work requested by the current operation.
        requested: u64,
        /// Configured work budget.
        budget: u64,
    },
    /// The work counter would overflow.
    WorkCounterOverflow,
}

impl fmt::Display for LoweringContextError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyNamespace => f.write_str("lowering entity namespace is empty"),
            Self::EmptyEntityIdentity => f.write_str("lowering entity identity is empty"),
            Self::InvalidIdentity => f.write_str("lowering entity identity is invalid"),
            Self::InvalidEntityKind => f.write_str("lowering entity kind is invalid"),
            Self::EntityKindMismatch { source, destination } => {
                write!(
                    f,
                    "cannot map entity category {source:?} to {destination:?}"
                )
            }
            Self::InvalidRequirement => {
                f.write_str("symbolic resource requirement is malformed")
            }
            Self::InvalidProvenanceSource => {
                f.write_str("provenance origin must reference a source entity")
            }
            Self::InvalidSourceRange => {
                f.write_str("provenance source range ends before it starts")
            }
            Self::IncompleteSourceRange => {
                f.write_str("provenance range requires both endpoints")
            }
            Self::InvalidDiagnostic => {
                f.write_str("lowering diagnostic code or message is invalid")
            }
            Self::InvalidStrategy => f.write_str("lowering strategy name is invalid"),
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
                f.write_str("lowering work counter overflow")
            }
        }
    }
}

impl Error for LoweringContextError {}

/// Shared state for one lowering invocation.
///
/// Entity mappings are one-to-many because one source operation may legally
/// expand into several destination operations. Mapping keys include their
/// representation namespace to avoid accidental cross-module collisions.
///
/// This context owns bookkeeping only. It does not own the source or
/// destination IR and does not execute lowering strategies.
#[derive(Debug, Clone)]
pub struct LoweringContext {
    policy: LoweringPolicy,
    mappings: BTreeMap<EntityKey, BTreeSet<EntityKey>>,
    dispositions: BTreeMap<EntityKey, LoweringDisposition>,
    capabilities: BTreeSet<String>,
    requirements: BTreeSet<ResourceRequirement>,
    provenance: BTreeMap<EntityKey, BTreeSet<ProvenanceOrigin>>,
    diagnostics: BTreeSet<LoweringNote>,
    consumed_work: u64,
}

impl LoweringContext {
    /// Create an empty context for one lowering invocation.
    #[must_use]
    pub fn new(policy: LoweringPolicy) -> Self {
        Self {
            policy,
            mappings: BTreeMap::new(),
            dispositions: BTreeMap::new(),
            capabilities: BTreeSet::new(),
            requirements: BTreeSet::new(),
            provenance: BTreeMap::new(),
            diagnostics: BTreeSet::new(),
            consumed_work: 0,
        }
    }

    /// Return the immutable lowering policy.
    #[must_use]
    pub fn policy(&self) -> &LoweringPolicy {
        &self.policy
    }

    /// Record a source-to-destination entity association.
    ///
    /// Multiple destination entities are supported for expansion. Repeating
    /// an existing association is idempotent. Different entity categories
    /// are rejected rather than silently conflated.
    pub fn record_mapping(
        &mut self,
        source: EntityKey,
        destination: EntityKey,
    ) -> Result<bool, LoweringContextError> {
        if source.kind() != destination.kind() {
            return Err(LoweringContextError::EntityKindMismatch {
                source: source.kind().clone(),
                destination: destination.kind().clone(),
            });
        }

        Ok(self
            .mappings
            .entry(source)
            .or_default()
            .insert(destination))
    }

    /// Return the destination entities associated with a source entity.
    ///
    /// The returned set is ordered and remains borrowed from this context.
    #[must_use]
    pub fn mapped_entities(
        &self,
        source: &EntityKey,
    ) -> Option<&BTreeSet<EntityKey>> {
        self.mappings.get(source)
    }

    /// Iterate over source-to-destination mappings deterministically.
    pub fn mappings(
        &self,
    ) -> impl Iterator<Item = (&EntityKey, &BTreeSet<EntityKey>)> {
        self.mappings.iter()
    }

    /// Record the legality decision for a source entity.
    pub fn record_disposition(
        &mut self,
        source: EntityKey,
        disposition: LoweringDisposition,
    ) -> Result<(), LoweringContextError> {
        match &disposition {
            LoweringDisposition::Conditional { conditions } => {
                if conditions.iter().any(|condition| condition.trim().is_empty()) {
                    return Err(LoweringContextError::InvalidStrategy);
                }
            }
            LoweringDisposition::ExpansionRequired { strategy } => {
                if strategy.trim().is_empty() || strategy.contains('\0') {
                    return Err(LoweringContextError::InvalidStrategy);
                }
            }
            LoweringDisposition::Unsupported { reason } => {
                if reason.trim().is_empty() {
                    return Err(LoweringContextError::InvalidStrategy);
                }
            }
            LoweringDisposition::Legal => {}
        }

        self.dispositions.insert(source, disposition);
        Ok(())
    }

    /// Get the recorded legality decision for an entity.
    #[must_use]
    pub fn disposition(
        &self,
        source: &EntityKey,
    ) -> Option<&LoweringDisposition> {
        self.dispositions.get(source)
    }

    /// Register a required capability name.
    ///
    /// This records a requirement; it does not assert that a target provides
    /// the capability. Capability discovery and satisfaction belong to the
    /// target-feasibility layer.
    pub fn require_capability(
        &mut self,
        capability: impl Into<String>,
    ) -> Result<bool, LoweringContextError> {
        let capability = capability.into();
        if capability.trim().is_empty() || capability.contains('\0') {
            return Err(LoweringContextError::InvalidRequirement);
        }
        Ok(self.capabilities.insert(capability))
    }

    /// Iterate over required capabilities deterministically.
    pub fn required_capabilities(&self) -> impl Iterator<Item = &str> {
        self.capabilities.iter().map(String::as_str)
    }

    /// Record a symbolic resource requirement.
    pub fn require_resource(
        &mut self,
        requirement: ResourceRequirement,
    ) -> bool {
        self.requirements.insert(requirement)
    }

    /// Iterate over symbolic resource requirements deterministically.
    pub fn resource_requirements(
        &self,
    ) -> impl Iterator<Item = &ResourceRequirement> {
        self.requirements.iter()
    }

    /// Associate an additional source origin with a destination entity.
    ///
    /// Multiple origins are preserved so transformations and expansions
    /// can retain provenance from more than one source construct.
    pub fn add_provenance(
        &mut self,
        destination: EntityKey,
        origin: ProvenanceOrigin,
    ) -> bool {
        self.provenance
            .entry(destination)
            .or_default()
            .insert(origin)
    }

    /// Return the known origins of a destination entity.
    #[must_use]
    pub fn provenance(
        &self,
        destination: &EntityKey,
    ) -> Option<&BTreeSet<ProvenanceOrigin>> {
        self.provenance.get(destination)
    }

    /// Add a deterministic lowering diagnostic.
    pub fn add_diagnostic(&mut self, note: LoweringNote) -> bool {
        self.diagnostics.insert(note)
    }

    /// Iterate over diagnostics in deterministic order.
    pub fn diagnostics(&self) -> impl Iterator<Item = &LoweringNote> {
        self.diagnostics.iter()
    }

    /// Charge work units against the optional configured budget.
    ///
    /// On failure, the consumed-work counter is unchanged. The caller chooses
    /// what constitutes a work unit and must charge consistently. This is
    /// accounting, not an automatic CPU-time or memory monitor.
    pub fn charge_work(&mut self, units: u64) -> Result<(), LoweringContextError> {
        let next = self
            .consumed_work
            .checked_add(units)
            .ok_or(LoweringContextError::WorkCounterOverflow)?;

        if let Some(budget) = self.policy.work_budget {
            if next > budget {
                return Err(LoweringContextError::WorkBudgetExceeded {
                    consumed: self.consumed_work,
                    requested: units,
                    budget,
                });
            }
        }

        self.consumed_work = next;
        Ok(())
    }

    /// Return the number of work units charged so far.
    #[must_use]
    pub const fn consumed_work(&self) -> u64 {
        self.consumed_work
    }

    /// Return the remaining work budget, or `None` when unbounded by policy.
    #[must_use]
    pub fn remaining_work(&self) -> Option<u64> {
        self.policy
            .work_budget
            .map(|budget| budget.saturating_sub(self.consumed_work))
    }

    /// Whether any entity has been explicitly marked unsupported.
    #[must_use]
    pub fn has_unsupported_entities(&self) -> bool {
        self.dispositions
            .values()
            .any(|disposition| matches!(disposition, LoweringDisposition::Unsupported { .. }))
    }

    /// Whether this context has recorded any diagnostics.
    #[must_use]
    pub fn has_diagnostics(&self) -> bool {
        !self.diagnostics.is_empty()
    }

    /// Clear invocation-local state while preserving the configured policy.
    ///
    /// Use this only when the caller deliberately begins a new lowering
    /// invocation. Do not reuse a context across unrelated compilations.
    pub fn reset(&mut self) {
        self.mappings.clear();
        self.dispositions.clear();
        self.capabilities.clear();
        self.requirements.clear();
        self.provenance.clear();
        self.diagnostics.clear();
        self.consumed_work = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn key(kind: EntityKind, namespace: &str, value: &str) -> EntityKey {
        EntityKey::new(kind, namespace, value).expect("valid test entity")
    }

    #[test]
    fn mapping_is_idempotent_and_supports_expansion() {
        let mut context = LoweringContext::new(LoweringPolicy::default());
        let source = key(EntityKind::Operation, "zui", "op-1");
        let first = key(EntityKind::Operation, "classical", "op-a");
        let second = key(EntityKind::Operation, "classical", "op-b");

        assert!(context
            .record_mapping(source.clone(), first.clone())
            .expect("mapping succeeds"));
        assert!(!context
            .record_mapping(source.clone(), first)
            .expect("duplicate mapping is idempotent"));
        assert!(context
            .record_mapping(source.clone(), second)
            .expect("expansion mapping succeeds"));
        assert_eq!(context.mapped_entities(&source).map(BTreeSet::len), Some(2));
    }

    #[test]
    fn incompatible_entity_categories_are_rejected() {
        let mut context = LoweringContext::new(LoweringPolicy::default());
        let source = key(EntityKind::Value, "zui", "v1");
        let destination = key(EntityKind::Operation, "target", "i1");

        assert!(matches!(
            context.record_mapping(source, destination),
            Err(LoweringContextError::EntityKindMismatch { .. })
        ));
    }

    #[test]
    fn work_budget_failure_does_not_charge_excess_work() {
        let mut context = LoweringContext::new(LoweringPolicy {
            work_budget: Some(10),
            ..LoweringPolicy::default()
        });

        context.charge_work(7).expect("within budget");
        assert!(matches!(
            context.charge_work(4),
            Err(LoweringContextError::WorkBudgetExceeded { .. })
        ));
        assert_eq!(context.consumed_work(), 7);
        assert_eq!(context.remaining_work(), Some(3));
    }

    #[test]
    fn symbolic_resource_requirements_are_not_hardware_limits() {
        let requirement = ResourceRequirement::new(
            "qubits",
            RequirementRelation::AtLeast,
            Some("logical_qubit_count(program)".to_owned()),
            Some("derived from program semantics".to_owned()),
        )
        .expect("valid symbolic requirement");

        let mut context = LoweringContext::new(LoweringPolicy::default());
        assert!(context.require_resource(requirement.clone()));
        assert!(!context.require_resource(requirement));
        assert_eq!(context.resource_requirements().count(), 1);
    }

    #[test]
    fn provenance_preserves_multiple_origins() {
        let mut context = LoweringContext::new(LoweringPolicy::default());
        let destination = key(EntityKind::Operation, "target", "op-1");
        let source_a = key(EntityKind::Source, "source", "file-a.zm");
        let source_b = key(EntityKind::Source, "source", "file-b.zm");

        let origin_a = ProvenanceOrigin::new(
            source_a,
            Some(0),
            Some(4),
            Some("first origin".to_owned()),
        )
        .expect("valid origin");
        let origin_b = ProvenanceOrigin::new(
            source_b,
            Some(2),
            Some(8),
            Some("second origin".to_owned()),
        )
        .expect("valid origin");

        assert!(context.add_provenance(destination.clone(), origin_a));
        assert!(context.add_provenance(destination.clone(), origin_b));
        assert_eq!(context.provenance(&destination).map(BTreeSet::len), Some(2));
    }

    #[test]
    fn deterministic_collections_deduplicate_diagnostics() {
        let mut context = LoweringContext::new(LoweringPolicy::default());
        let note = LoweringNote::new(
            "ZMN-LOWERING-001",
            "conversion requires a capability check",
            None,
        )
        .expect("valid diagnostic");

        assert!(context.add_diagnostic(note.clone()));
        assert!(!context.add_diagnostic(note));
        assert_eq!(context.diagnostics().count(), 1);
    }

    #[test]
    fn invalid_source_ranges_are_rejected() {
        let source = key(EntityKind::Source, "source", "example.zm");
        assert!(matches!(
            ProvenanceOrigin::new(source, Some(9), Some(4), None),
            Err(LoweringContextError::InvalidSourceRange)
        ));
    }
}
