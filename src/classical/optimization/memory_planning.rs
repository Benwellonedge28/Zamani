
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! Zamani Classical Computing — Memory Planning.
//!
//! Path: `src/classical/optimization/memory_planning.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Unsafe Rust is forbidden.
//!
//! # Purpose
//!
//! Construct deterministic, target-independent memory layout plans from
//! logical storage requests and trusted lifetime-analysis evidence.
//!
//! The planner can reuse a storage slot only when:
//!
//! - both objects have statically known sizes;
//! - their memory spaces and allocation classes match;
//! - the slot has sufficient capacity and alignment;
//! - the analysis explicitly proves the objects' lifetimes disjoint in both
//!   directions; and
//! - the evidence belongs to the same analysis revision.
//!
//! Unknown or symbolic sizes are preserved without guessing their values.
//! They are not eligible for storage reuse by this implementation.
//!
//! # Authority
//!
//! - `grammar/specification/semantic-model.md`: semantic meaning.
//! - `grammar/specification/scalability-model.md`: scalability requirements.
//! - `grammar/spec/resources.md`: resource semantics.
//! - `grammar/memory/allocation.g4`: allocation syntax.
//! - `grammar/memory/lifetimes.g4`: lifetime syntax.
//! - `src/classical/memory/allocation.rs`: actual allocation boundary.
//! - `src/classical/memory/regions.rs`: semantic region relationships.
//! - `src/classical/memory/lifetime.rs`: lifetime constraints.
//! - `src/classical/optimization/legality.rs`: optimization legality.
//! - `src/classical/optimization/passes.rs`: optimization orchestration.
//! - `src/classical/execution/requirements.rs`: execution requirements.
//! - `src/classical/execution/capabilities.rs`: target capability assessment.
//! - `src/zuir/`: canonical universal IR, where selected by the pipeline.
//! - `src/quantum/ir/classical/`: existing canonical classical IR boundary.
//!
//! # Ownership
//!
//! This file owns logical storage requests, explicit lifetime-disjointness
//! evidence references, reusable-slot decisions, planning policy, the
//! immutable layout plan, budget assessment, and planning errors.
//!
//! It does not own source syntax, semantic type checking, borrow checking,
//! lifetime inference, alias analysis, canonical IR, actual allocation,
//! physical addresses, placement, scheduling, or runtime execution.
//!
//! # Resource and scalability contract
//!
//! No universal maximum is imposed on object count, memory-space identifiers,
//! allocation-class identifiers, or logical memory requirements.
//!
//! `u128` represents concrete byte quantities and explicit byte budgets;
//! it is not an assertion that every conceivable size is representable.
//! Symbolic and unknown sizes remain representable when exact byte quantities
//! cannot be established.
//!
//! Configured budgets are operational policy. They are not language-level
//! validity rules. The absence of a planner budget does not imply unlimited
//! physical memory.
//!
//! # Safety and transactionality
//!
//! This module never allocates target storage or mutates IR. It only creates
//! a plan. The downstream adapter must validate that its analysis evidence
//! remains current, verify the transformed canonical IR, and commit any
//! corresponding changes transactionally.
//!
//! A planning result does not itself reserve or allocate memory.

use core::fmt;
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;

/// Result type for memory-planning operations.
pub type MemoryPlanningResult<T> = Result<T, MemoryPlanningError>;

/// The size known for one logical storage object.
///
/// Symbolic expressions are opaque semantic expressions. This planner does
/// not parse, simplify, evaluate, or silently approximate them.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum MemorySize {
    /// An exact, statically known byte count.
    Exact(u128),

    /// A symbolic size expression requiring downstream evaluation.
    Symbolic(String),

    /// The size could not be established by the producing analysis.
    Unknown,
}

impl MemorySize {
    /// Creates a symbolic size expression.
    ///
    /// Empty expressions and expressions containing control characters are
    /// rejected. No fixed expression-length ceiling is imposed.
    pub fn symbolic(expression: impl Into<String>) -> MemoryPlanningResult<Self> {
        let expression = expression.into();

        if expression.trim().is_empty()
            || expression.chars().any(char::is_control)
        {
            return Err(MemoryPlanningError::InvalidIdentifier {
                field: "symbolic size expression",
                value: expression,
            });
        }

        Ok(Self::Symbolic(expression))
    }

    /// Returns the exact size, if known.
    #[must_use]
    pub const fn exact_bytes(&self) -> Option<u128> {
        match self {
            Self::Exact(bytes) => Some(*bytes),
            Self::Symbolic(_) | Self::Unknown => None,
        }
    }

    /// Returns whether the size is known exactly.
    #[must_use]
    pub const fn is_exact(&self) -> bool {
        matches!(self, Self::Exact(_))
    }
}

/// A logical storage object presented to the planner.
///
/// Identifiers are open-world strings rather than closed hardware enums.
/// `analysis_revision` binds lifetime evidence to the analysis snapshot from
/// which it was produced. Adapters must use stable, revision-specific values.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MemoryObject {
    id: String,
    size: MemorySize,
    alignment_bytes: u128,
    memory_space: String,
    allocation_class: String,
    lifetime_id: String,
    analysis_revision: String,
    proven_disjoint_with: BTreeSet<String>,
}

impl MemoryObject {
    /// Creates a logical storage request.
    ///
    /// `alignment_bytes` must be nonzero. The planner does not assume that
    /// every architecture uses the same alignment restrictions; the
    /// downstream target adapter validates target-specific requirements.
    pub fn new(
        id: impl Into<String>,
        size: MemorySize,
        alignment_bytes: u128,
        memory_space: impl Into<String>,
        allocation_class: impl Into<String>,
        lifetime_id: impl Into<String>,
        analysis_revision: impl Into<String>,
    ) -> MemoryPlanningResult<Self> {
        let id = id.into();
        let memory_space = memory_space.into();
        let allocation_class = allocation_class.into();
        let lifetime_id = lifetime_id.into();
        let analysis_revision = analysis_revision.into();

        validate_identifier("object id", &id)?;
        validate_identifier("memory space", &memory_space)?;
        validate_identifier("allocation class", &allocation_class)?;
        validate_identifier("lifetime id", &lifetime_id)?;
        validate_identifier("analysis revision", &analysis_revision)?;

        if alignment_bytes == 0 {
            return Err(MemoryPlanningError::InvalidAlignment {
                object_id: id,
                alignment_bytes,
            });
        }

        Ok(Self {
            id,
            size,
            alignment_bytes,
            memory_space,
            allocation_class,
            lifetime_id,
            analysis_revision,
            proven_disjoint_with: BTreeSet::new(),
        })
    }

    /// Adds a lifetime-analysis result proving disjointness from another
    /// object.
    ///
    /// This is a declaration of evidence, not a proof-producing analysis.
    /// The caller must only add the relation when an authoritative analysis
    /// has established it for the current analysis revision.
    ///
    /// The planner requires the relation to be recorded by both objects
    /// before it will reuse a slot. Missing or one-sided evidence therefore
    /// prevents reuse.
    pub fn prove_disjoint_from(
        &mut self,
        other_object_id: impl Into<String>,
    ) -> MemoryPlanningResult<()> {
        let other_object_id = other_object_id.into();
        validate_identifier("disjoint object id", &other_object_id)?;

        if other_object_id == self.id {
            return Err(MemoryPlanningError::SelfDisjointness {
                object_id: self.id.clone(),
            });
        }

        self.proven_disjoint_with.insert(other_object_id);
        Ok(())
    }

    /// Returns the logical object identifier.
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the requested size.
    #[must_use]
    pub fn size(&self) -> &MemorySize {
        &self.size
    }

    /// Returns the required byte alignment.
    #[must_use]
    pub const fn alignment_bytes(&self) -> u128 {
        self.alignment_bytes
    }

    /// Returns the semantic memory-space identifier.
    #[must_use]
    pub fn memory_space(&self) -> &str {
        &self.memory_space
    }

    /// Returns the allocation-class identifier.
    #[must_use]
    pub fn allocation_class(&self) -> &str {
        &self.allocation_class
    }

    /// Returns the lifetime identity supplied by semantic analysis.
    #[must_use]
    pub fn lifetime_id(&self) -> &str {
        &self.lifetime_id
    }

    /// Returns the revision to which this object's evidence belongs.
    #[must_use]
    pub fn analysis_revision(&self) -> &str {
        &self.analysis_revision
    }

    /// Returns the explicitly recorded lifetime-disjointness relations.
    #[must_use]
    pub fn proven_disjoint_with(&self) -> &BTreeSet<String> {
        &self.proven_disjoint_with
    }
}

/// Explicit operational budgets for one memory-planning run.
///
/// `None` means that this policy does not set a budget for that metric.
/// Physical target capacity and runtime allocation limits remain separate.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MemoryPlanningPolicy {
    max_reserved_bytes: Option<u128>,
    max_storage_slots: Option<u128>,
    allow_slot_reuse: bool,
}

impl MemoryPlanningPolicy {
    /// Creates a policy with no extra budgets and with safe slot reuse enabled.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            max_reserved_bytes: None,
            max_storage_slots: None,
            allow_slot_reuse: true,
        }
    }

    /// Sets the maximum total statically known bytes reserved by the plan.
    ///
    /// This is an operational budget, not a semantic limit.
    #[must_use]
    pub const fn with_max_reserved_bytes(mut self, limit: u128) -> Self {
        self.max_reserved_bytes = Some(limit);
        self
    }

    /// Sets the maximum number of storage slots in the plan.
    ///
    /// This is an operational budget, not a universal hardware limit.
    #[must_use]
    pub const fn with_max_storage_slots(mut self, limit: u128) -> Self {
        self.max_storage_slots = Some(limit);
        self
    }

    /// Enables or disables reuse of storage slots.
    ///
    /// Disabling reuse is useful for debugging, qualification, or a target
    /// policy that requires separate logical allocations.
    #[must_use]
    pub const fn with_slot_reuse(mut self, enabled: bool) -> Self {
        self.allow_slot_reuse = enabled;
        self
    }

    /// Returns the optional reserved-byte budget.
    #[must_use]
    pub const fn max_reserved_bytes(&self) -> Option<u128> {
        self.max_reserved_bytes
    }

    /// Returns the optional storage-slot budget.
    #[must_use]
    pub const fn max_storage_slots(&self) -> Option<u128> {
        self.max_storage_slots
    }

    /// Returns whether safe storage-slot reuse is enabled.
    #[must_use]
    pub const fn slot_reuse_enabled(&self) -> bool {
        self.allow_slot_reuse
    }
}

/// The result of checking the statically known portion of a memory budget.
///
/// `Unresolved` means at least one planned slot has a symbolic or unknown
/// size, so a configured total-byte budget cannot yet be fully assessed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum MemoryBudgetAssessment {
    /// No total-byte budget was configured.
    NotConfigured,

    /// All planned slot sizes are known and fit the configured budget.
    WithinBudget,

    /// At least one slot size is symbolic or unknown.
    Unresolved,
}

/// A planned storage slot.
///
/// Slots are logical layout decisions, not physical addresses or allocated
/// buffers. An execution or lowering adapter determines their realization.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MemorySlot {
    id: String,
    size: MemorySize,
    alignment_bytes: u128,
    memory_space: String,
    allocation_class: String,
    object_ids: Vec<String>,
}

impl MemorySlot {
    /// Returns the stable slot identifier for this plan.
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the planned size or size expression.
    #[must_use]
    pub fn size(&self) -> &MemorySize {
        &self.size
    }

    /// Returns the slot's required alignment.
    #[must_use]
    pub const fn alignment_bytes(&self) -> u128 {
        self.alignment_bytes
    }

    /// Returns the memory-space identifier.
    #[must_use]
    pub fn memory_space(&self) -> &str {
        &self.memory_space
    }

    /// Returns the allocation-class identifier.
    #[must_use]
    pub fn allocation_class(&self) -> &str {
        &self.allocation_class
    }

    /// Returns the logical objects assigned to this slot.
    #[must_use]
    pub fn object_ids(&self) -> &[String] {
        &self.object_ids
    }
}

/// Immutable result of a memory-planning run.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MemoryPlan {
    analysis_revision: String,
    object_to_slot: BTreeMap<String, String>,
    slots: BTreeMap<String, MemorySlot>,
    known_reserved_bytes: u128,
    unknown_size_slots: u128,
    budget_assessment: MemoryBudgetAssessment,
}

impl MemoryPlan {
    /// Returns the analysis revision on which this plan is based.
    #[must_use]
    pub fn analysis_revision(&self) -> &str {
        &self.analysis_revision
    }

    /// Returns the object-to-slot mapping in deterministic key order.
    #[must_use]
    pub fn object_to_slot(&self) -> &BTreeMap<String, String> {
        &self.object_to_slot
    }

    /// Returns planned slots in deterministic identifier order.
    #[must_use]
    pub fn slots(&self) -> &BTreeMap<String, MemorySlot> {
        &self.slots
    }

    /// Returns the slot assigned to an object, if present.
    #[must_use]
    pub fn slot_for(&self, object_id: &str) -> Option<&MemorySlot> {
        let slot_id = self.object_to_slot.get(object_id)?;
        self.slots.get(slot_id)
    }

    /// Returns the sum of exact slot capacities, excluding unknown sizes.
    ///
    /// This is not the total physical memory required when
    /// `unknown_size_slots` is nonzero.
    #[must_use]
    pub const fn known_reserved_bytes(&self) -> u128 {
        self.known_reserved_bytes
    }

    /// Returns the number of slots with non-exact sizes.
    #[must_use]
    pub const fn unknown_size_slots(&self) -> u128 {
        self.unknown_size_slots
    }

    /// Returns the total-byte budget assessment.
    #[must_use]
    pub const fn budget_assessment(&self) -> MemoryBudgetAssessment {
        self.budget_assessment
    }

    /// Returns whether every planned slot has a statically known size.
    #[must_use]
    pub const fn all_sizes_known(&self) -> bool {
        self.unknown_size_slots == 0
    }
}

/// Errors returned while validating or constructing a memory plan.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum MemoryPlanningError {
    /// A required identifier or symbolic expression is invalid.
    InvalidIdentifier {
        /// The field being validated.
        field: &'static str,
        /// The rejected value.
        value: String,
    },

    /// A storage object has zero alignment.
    InvalidAlignment {
        /// The object identifier.
        object_id: String,
        /// The invalid alignment.
        alignment_bytes: u128,
    },

    /// The same object identifier occurs more than once.
    DuplicateObjectId {
        /// The repeated identifier.
        object_id: String,
    },

    /// An object declares itself lifetime-disjoint from itself.
    SelfDisjointness {
        /// The object identifier.
        object_id: String,
    },

    /// A disjointness relation references an object not in this planning run.
    UnknownDisjointReference {
        /// The declaring object.
        object_id: String,
        /// The missing referenced object.
        referenced_object_id: String,
    },

    /// Evidence belongs to a different analysis revision.
    StaleAnalysisEvidence {
        /// The object with stale evidence.
        object_id: String,
        /// The expected revision.
        expected_revision: String,
        /// The object's revision.
        actual_revision: String,
    },

    /// The generated slot identifier could not be represented.
    SlotIdentifierExhausted,

    /// A checked arithmetic operation overflowed.
    ArithmeticOverflow {
        /// The operation that overflowed.
        operation: &'static str,
    },

    /// The known reserved byte count exceeds an explicit budget.
    ReservedBytesBudgetExceeded {
        /// Exact bytes planned for known-size slots.
        required_bytes: u128,
        /// Configured maximum.
        limit_bytes: u128,
    },

    /// The number of planned slots exceeds an explicit budget.
    StorageSlotBudgetExceeded {
        /// Number of slots required.
        required_slots: u128,
        /// Configured maximum.
        limit_slots: u128,
    },
}

impl fmt::Display for MemoryPlanningError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidIdentifier { field, value } => {
                write!(f, "invalid {field}: {value:?}")
            }
            Self::InvalidAlignment {
                object_id,
                alignment_bytes,
            } => write!(
                f,
                "object {object_id:?} has invalid zero alignment \
                 (alignment_bytes={alignment_bytes})"
            ),
            Self::DuplicateObjectId { object_id } => {
                write!(f, "duplicate memory object identifier: {object_id:?}")
            }
            Self::SelfDisjointness { object_id } => {
                write!(f, "object {object_id:?} cannot be disjoint from itself")
            }
            Self::UnknownDisjointReference {
                object_id,
                referenced_object_id,
            } => write!(
                f,
                "object {object_id:?} references unknown disjointness \
                 evidence target {referenced_object_id:?}"
            ),
            Self::StaleAnalysisEvidence {
                object_id,
                expected_revision,
                actual_revision,
            } => write!(
                f,
                "stale lifetime evidence for object {object_id:?}: \
                 expected revision {expected_revision:?}, \
                 got {actual_revision:?}"
            ),
            Self::SlotIdentifierExhausted => {
                f.write_str("memory-planning slot identifier exhausted")
            }
            Self::ArithmeticOverflow { operation } => {
                write!(f, "memory-planning arithmetic overflow during {operation}")
            }
            Self::ReservedBytesBudgetExceeded {
                required_bytes,
                limit_bytes,
            } => write!(
                f,
                "memory planning requires {required_bytes} known bytes, \
                 exceeding the configured budget of {limit_bytes} bytes"
            ),
            Self::StorageSlotBudgetExceeded {
                required_slots,
                limit_slots,
            } => write!(
                f,
                "memory planning requires {required_slots} slots, \
                 exceeding the configured budget of {limit_slots}"
            ),
        }
    }
}

impl Error for MemoryPlanningError {}

fn validate_identifier(
    field: &'static str,
    value: &str,
) -> MemoryPlanningResult<()> {
    if value.trim().is_empty() || value.chars().any(char::is_control) {
        return Err(MemoryPlanningError::InvalidIdentifier {
            field,
            value: value.to_owned(),
        });
    }

    Ok(())
}

/// Builds a deterministic memory layout from logical objects and analysis
/// evidence.
///
/// Objects are processed in identifier order. This makes slot selection
/// independent of input collection order. A slot is reused only when the
/// conditions documented on [`MemoryObject`] can be established from the
/// supplied records.
///
/// Symbolic and unknown sizes receive separate slots and prevent a configured
/// byte budget from being reported as fully satisfied. They are never
/// estimated as zero.
///
/// This function does not allocate storage, discover devices, or mutate IR.
pub fn plan_memory(
    analysis_revision: impl Into<String>,
    objects: impl IntoIterator<Item = MemoryObject>,
    policy: MemoryPlanningPolicy,
) -> MemoryPlanningResult<MemoryPlan> {
    let analysis_revision = analysis_revision.into();
    validate_identifier("analysis revision", &analysis_revision)?;

    let mut object_index = BTreeMap::<String, MemoryObject>::new();

    for object in objects {
        if object.analysis_revision != analysis_revision {
            return Err(MemoryPlanningError::StaleAnalysisEvidence {
                object_id: object.id,
                expected_revision: analysis_revision.clone(),
                actual_revision: object.analysis_revision,
            });
        }

        let id = object.id.clone();

        if object_index.insert(id.clone(), object).is_some() {
            return Err(MemoryPlanningError::DuplicateObjectId {
                object_id: id,
            });
        }
    }

    // Reject evidence references to absent objects rather than silently
    // treating misspelled identifiers as a valid proof.
    for object in object_index.values() {
        for referenced_id in &object.proven_disjoint_with {
            if !object_index.contains_key(referenced_id) {
                return Err(MemoryPlanningError::UnknownDisjointReference {
                    object_id: object.id.clone(),
                    referenced_object_id: referenced_id.clone(),
                });
            }
        }
    }

    let mut slots = BTreeMap::<String, MemorySlot>::new();
    let mut object_to_slot = BTreeMap::<String, String>::new();
    let mut next_slot_id = 0_u128;

    for object in object_index.values() {
        let reusable_slot_id = if policy.allow_slot_reuse {
            match object.size.exact_bytes() {
                Some(object_bytes) => slots
                    .iter()
                    .find(|(_, slot)| {
                        slot.size.exact_bytes().is_some_and(|slot_bytes| {
                            slot_bytes >= object_bytes
                        }) && slot.memory_space == object.memory_space
                            && slot.allocation_class == object.allocation_class
                            && slot.alignment_bytes >= object.alignment_bytes
                            && slot.object_ids.iter().all(|prior_id| {
                                let Some(prior) = object_index.get(prior_id) else {
                                    return false;
                                };

                                object.proven_disjoint_with.contains(prior_id)
                                    && prior
                                        .proven_disjoint_with
                                        .contains(&object.id)
                            })
                    })
                    .map(|(slot_id, _)| slot_id.clone()),
                None => None,
            }
        } else {
            None
        };

        if let Some(slot_id) = reusable_slot_id {
            let slot = slots
                .get_mut(&slot_id)
                .expect("selected slot must remain present");

            if object.alignment_bytes > slot.alignment_bytes {
                slot.alignment_bytes = object.alignment_bytes;
            }

            if let (
                MemorySize::Exact(slot_bytes),
                MemorySize::Exact(object_bytes),
            ) = (&slot.size, &object.size)
            {
                if object_bytes > slot_bytes {
                    slot.size = MemorySize::Exact(*object_bytes);
                }
            }

            slot.object_ids.push(object.id.clone());
            object_to_slot.insert(object.id.clone(), slot_id);
            continue;
        }

        let slot_id = format!("storage-slot-{next_slot_id}");
        next_slot_id = next_slot_id
            .checked_add(1)
            .ok_or(MemoryPlanningError::SlotIdentifierExhausted)?;

        slots.insert(
            slot_id.clone(),
            MemorySlot {
                id: slot_id.clone(),
                size: object.size.clone(),
                alignment_bytes: object.alignment_bytes,
                memory_space: object.memory_space.clone(),
                allocation_class: object.allocation_class.clone(),
                object_ids: vec![object.id.clone()],
            },
        );

        object_to_slot.insert(object.id.clone(), slot_id);
    }

    let slot_count = u128::try_from(slots.len()).map_err(|_| {
        MemoryPlanningError::ArithmeticOverflow {
            operation: "converting slot count",
        }
    })?;

    if let Some(limit_slots) = policy.max_storage_slots {
        if slot_count > limit_slots {
            return Err(MemoryPlanningError::StorageSlotBudgetExceeded {
                required_slots: slot_count,
                limit_slots,
            });
        }
    }

    let mut known_reserved_bytes = 0_u128;
    let mut unknown_size_slots = 0_u128;

    for slot in slots.values() {
        match slot.size {
            MemorySize::Exact(bytes) => {
                known_reserved_bytes = known_reserved_bytes.checked_add(bytes).ok_or(
                    MemoryPlanningError::ArithmeticOverflow {
                        operation: "summing known storage-slot capacities",
                    },
                )?;
            }
            MemorySize::Symbolic(_) | MemorySize::Unknown => {
                unknown_size_slots = unknown_size_slots.checked_add(1).ok_or(
                    MemoryPlanningError::ArithmeticOverflow {
                        operation: "counting unknown-size storage slots",
                    },
                )?;
            }
        }
    }

    let budget_assessment = match policy.max_reserved_bytes {
        None => MemoryBudgetAssessment::NotConfigured,
        Some(limit_bytes) if known_reserved_bytes > limit_bytes => {
            return Err(MemoryPlanningError::ReservedBytesBudgetExceeded {
                required_bytes: known_reserved_bytes,
                limit_bytes,
            });
        }
        Some(_) if unknown_size_slots > 0 => MemoryBudgetAssessment::Unresolved,
        Some(_) => MemoryBudgetAssessment::WithinBudget,
    };

    Ok(MemoryPlan {
        analysis_revision,
        object_to_slot,
        slots,
        known_reserved_bytes,
        unknown_size_slots,
        budget_assessment,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const REVISION: &str = "ir-revision-1";

    fn object(id: &str, bytes: u128) -> MemoryObject {
        MemoryObject::new(
            id,
            MemorySize::Exact(bytes),
            1,
            "host",
            "ordinary",
            format!("lifetime-{id}"),
            REVISION,
        )
        .expect("test object should be valid")
    }

    fn prove_mutual_disjointness(
        left: &mut MemoryObject,
        right: &mut MemoryObject,
    ) {
        left.prove_disjoint_from(right.id())
            .expect("left disjointness declaration should be valid");
        right
            .prove_disjoint_from(left.id())
            .expect("right disjointness declaration should be valid");
    }

    #[test]
    fn objects_without_disjointness_evidence_do_not_share_slots() {
        let plan = plan_memory(
            REVISION,
            [object("a", 16), object("b", 8)],
            MemoryPlanningPolicy::new(),
        )
        .expect("planning should succeed");

        assert_eq!(plan.slots().len(), 2);
        assert_ne!(plan.object_to_slot()["a"], plan.object_to_slot()["b"]);
        assert_eq!(plan.known_reserved_bytes(), 24);
    }

    #[test]
    fn mutually_proven_disjoint_objects_can_reuse_a_slot() {
        let mut a = object("a", 16);
        let mut b = object("b", 8);
        prove_mutual_disjointness(&mut a, &mut b);

        let plan = plan_memory(
            REVISION,
            [b, a],
            MemoryPlanningPolicy::new(),
        )
        .expect("planning should succeed");

        assert_eq!(plan.slots().len(), 1);
        assert_eq!(plan.object_to_slot()["a"], plan.object_to_slot()["b"]);
        assert_eq!(plan.known_reserved_bytes(), 16);
        assert_eq!(plan.slot_for("a").expect("slot exists").object_ids().len(), 2);
    }

    #[test]
    fn one_sided_disjointness_evidence_does_not_authorize_reuse() {
        let mut a = object("a", 16);
        let b = object("b", 8);

        a.prove_disjoint_from("b")
            .expect("reference should be accepted");

        let plan = plan_memory(
            REVISION,
            [a, b],
            MemoryPlanningPolicy::new(),
        )
        .expect("planning should conservatively succeed");

        assert_eq!(plan.slots().len(), 2);
    }

    #[test]
    fn objects_in_different_memory_spaces_do_not_share_slots() {
        let a = object("a", 16);
        let mut b = MemoryObject::new(
            "b",
            MemorySize::Exact(8),
            1,
            "other-space",
            "ordinary",
            "lifetime-b",
            REVISION,
        )
        .expect("test object should be valid");

        let mut a = a;
        prove_mutual_disjointness(&mut a, &mut b);

        let plan = plan_memory(
            REVISION,
            [a, b],
            MemoryPlanningPolicy::new(),
        )
        .expect("planning should succeed");

        assert_eq!(plan.slots().len(), 2);
    }

    #[test]
    fn incompatible_alignment_prevents_reuse() {
        let a = object("a", 16);
        let mut b = MemoryObject::new(
            "b",
            MemorySize::Exact(8),
            8,
            "host",
            "ordinary",
            "lifetime-b",
            REVISION,
        )
        .expect("test object should be valid");

        let mut a = a;
        prove_mutual_disjointness(&mut a, &mut b);

        let plan = plan_memory(
            REVISION,
            [a, b],
            MemoryPlanningPolicy::new(),
        )
        .expect("planning should succeed");

        assert_eq!(plan.slots().len(), 2);
    }

    #[test]
    fn symbolic_sizes_are_preserved_and_never_guessed_as_zero() {
        let symbolic = MemoryObject::new(
            "symbolic",
            MemorySize::symbolic("rows * columns * element_bytes")
                .expect("expression should be valid"),
            1,
            "host",
            "ordinary",
            "lifetime-symbolic",
            REVISION,
        )
        .expect("object should be valid");

        let plan = plan_memory(
            REVISION,
            [symbolic],
            MemoryPlanningPolicy::new().with_max_reserved_bytes(1024),
        )
        .expect("symbolic size should remain representable");

        assert_eq!(plan.known_reserved_bytes(), 0);
        assert_eq!(plan.unknown_size_slots(), 1);
        assert_eq!(
            plan.budget_assessment(),
            MemoryBudgetAssessment::Unresolved
        );
    }

    #[test]
    fn exact_budget_overrun_is_reported() {
        let result = plan_memory(
            REVISION,
            [object("a", 17)],
            MemoryPlanningPolicy::new().with_max_reserved_bytes(16),
        );

        assert!(matches!(
            result,
            Err(MemoryPlanningError::ReservedBytesBudgetExceeded {
                required_bytes: 17,
                limit_bytes: 16,
            })
        ));
    }

    #[test]
    fn slot_budget_overrun_is_reported() {
        let result = plan_memory(
            REVISION,
            [object("a", 1), object("b", 1)],
            MemoryPlanningPolicy::new().with_max_storage_slots(1),
        );

        assert!(matches!(
            result,
            Err(MemoryPlanningError::StorageSlotBudgetExceeded {
                required_slots: 2,
                limit_slots: 1,
            })
        ));
    }

    #[test]
    fn stale_analysis_revision_is_rejected() {
        let result = plan_memory(
            "ir-revision-2",
            [object("a", 8)],
            MemoryPlanningPolicy::new(),
        );

        assert!(matches!(
            result,
            Err(MemoryPlanningError::StaleAnalysisEvidence { .. })
        ));
    }

    #[test]
    fn unknown_disjointness_target_is_rejected() {
        let mut a = object("a", 8);
        a.prove_disjoint_from("missing")
            .expect("identifier syntax is valid");

        let result = plan_memory(
            REVISION,
            [a],
            MemoryPlanningPolicy::new(),
        );

        assert!(matches!(
            result,
            Err(MemoryPlanningError::UnknownDisjointReference { .. })
        ));
    }

    #[test]
    fn duplicate_object_ids_are_rejected() {
        let result = plan_memory(
            REVISION,
            [object("duplicate", 8), object("duplicate", 16)],
            MemoryPlanningPolicy::new(),
        );

        assert!(matches!(
            result,
            Err(MemoryPlanningError::DuplicateObjectId { .. })
        ));
    }

    #[test]
    fn zero_alignment_is_rejected() {
        let result = MemoryObject::new(
            "a",
            MemorySize::Exact(8),
            0,
            "host",
            "ordinary",
            "lifetime-a",
            REVISION,
        );

        assert!(matches!(
            result,
            Err(MemoryPlanningError::InvalidAlignment { .. })
        ));
    }

    #[test]
    fn zero_sized_objects_are_valid() {
        let plan = plan_memory(
            REVISION,
            [object("empty", 0)],
            MemoryPlanningPolicy::new(),
        )
        .expect("zero-sized object should be valid");

        assert_eq!(plan.known_reserved_bytes(), 0);
        assert!(plan.all_sizes_known());
    }

    #[test]
    fn disabling_reuse_keeps_disjoint_objects_separate() {
        let mut a = object("a", 16);
        let mut b = object("b", 8);
        prove_mutual_disjointness(&mut a, &mut b);

        let plan = plan_memory(
            REVISION,
            [a, b],
            MemoryPlanningPolicy::new().with_slot_reuse(false),
        )
        .expect("planning should succeed");

        assert_eq!(plan.slots().len(), 2);
    }

    #[test]
    fn planning_is_deterministic_for_different_input_orders() {
        let forward = plan_memory(
            REVISION,
            [object("b", 8), object("a", 16)],
            MemoryPlanningPolicy::new(),
        )
        .expect("planning should succeed");

        let reverse = plan_memory(
            REVISION,
            [object("a", 16), object("b", 8)],
            MemoryPlanningPolicy::new(),
        )
        .expect("planning should succeed");

        assert_eq!(forward, reverse);
    }

    #[test]
    fn large_byte_quantities_are_not_rejected_by_an_arbitrary_policy() {
        let plan = plan_memory(
            REVISION,
            [object("large", u128::MAX)],
            MemoryPlanningPolicy::new(),
        )
        .expect("one representable exact quantity should be accepted");

        assert_eq!(plan.known_reserved_bytes(), u128::MAX);
    }
}
