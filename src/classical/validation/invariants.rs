
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Classical Invariant Validation
//!
//! Path: `src/classical/validation/invariants.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: safe Rust only.
//!
//! ## Purpose
//!
//! Validate cross-object identity and reference invariants for classical
//! semantic and intermediate-representation models through a read-only
//! adapter.
//!
//! This module checks:
//!
//! - identity uniqueness;
//! - required references;
//! - reference existence;
//! - reference target-kind compatibility;
//! - ambiguous references caused by duplicate identities;
//! - forbidden self-references;
//! - duplicate references with the same role and target;
//! - explicit operational validation budgets;
//! - deterministic findings and validation completeness.
//!
//! ## Authority
//!
//! Source grammar:
//! - `grammar/antlr/ZamaniLexer.g4`
//! - `grammar/antlr/ZamaniParser.g4`
//!
//! Normative specifications:
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/poco-reaf.md`
//! - `grammar/validation/semantic-boundaries.md`
//!
//! Existing canonical validators:
//! - `src/frontend/ast/node/validation/invariants.rs`
//! - `src/frontend/semantic/validation/invariants.rs`
//! - `src/classical/validation/structural.rs`
//! - `src/classical/validation/types.rs`
//! - `src/classical/validation/effects.rs`
//! - `src/classical/validation/resources.rs`
//!
//! Canonical classical IR integration:
//! - `src/classical/zcir/`
//! - `src/quantum/ir/classical/`
//! - `src/zuir/`
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - A target-independent identity/reference validation adapter contract.
//! - Cross-object identity and reference consistency checks.
//! - Structured invariant findings and validation completeness reporting.
//! - Explicit operational budgets for this validation pass.
//!
//! This module does not own:
//!
//! - AST node identity, AST structure, or AST traversal.
//! - Semantic type representations or type inference.
//! - Resource quantities, resource feasibility, or capability definitions.
//! - Classical IR or ZUIR operation definitions.
//! - SSA dominance, control-flow terminator semantics, or effect inference.
//! - Name resolution, generic constraint solving, or overload resolution.
//! - Memory allocation, execution planning, scheduling, or hardware selection.
//!
//! Domain-specific invariants remain with their canonical owners.
//!
//! ## Important boundary
//!
//! This module validates metadata supplied by an adapter. It does not
//! recursively traverse a graph or attempt to replace the existing
//! structural validator.
//!
//! The adapter must expose canonical identities and authoritative references.
//! It must not invent IDs, silently omit references, or translate unresolved
//! semantic objects into apparently valid records.
//!
//! ## POCO-REAF and scalability
//!
//! No universal entity-count, reference-count, hardware-count, register-width,
//! collection-size, or program-size ceiling is imposed.
//!
//! Optional budgets limit one validation invocation, not valid language
//! programs. Budget exhaustion produces an incomplete report.
//!
//! Actual processing remains bounded by available memory, execution time,
//! and the representational capabilities of the host and canonical ID types.
//!
//! ## Determinism
//!
//! Findings follow input entity order and then reference order. Identity
//! lookup uses ordered maps; map iteration does not determine report order.
//! Entities and references are not silently reordered or deduplicated.
//!
//! ## Integration
//!
//! 1. The frontend parses source and constructs the canonical AST.
//! 2. Semantic analysis produces canonical semantic objects.
//! 3. The classical IR adapter exposes identities, kinds, and references.
//! 4. This validator checks cross-object consistency.
//! 5. Type, effect, resource, control-flow, and IR-specific validators run
//!    through their respective canonical interfaces.
//! 6. Lowering and execution planning consume only the guarantees actually
//!    established by those stages.
//!
//! This module is a generic foundation. A production adapter must be
//! implemented against the actual canonical IR before the complete classical
//! pipeline can claim these checks have been integrated.
//!
//! ## Completion contract
//!
//! This file is complete when its API is registered, canonical adapters are
//! integrated, invariant and budget tests pass, and the repository's supported
//! formatting, compilation, test, and lint checks succeed.

use core::fmt;
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;

/// Result type for classical invariant validation.
pub type InvariantValidationResult<T> = Result<T, InvariantValidationError>;

/// A stable category of cross-object invariant violation.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum InvariantViolationCode {
    /// An identity is invalid according to the adapter's identity contract.
    InvalidIdentity,

    /// More than one entity has the same canonical identity.
    DuplicateIdentity,

    /// A required reference has no target.
    MissingRequiredReference,

    /// A reference names an identity absent from the supplied model.
    UnresolvedReference,

    /// A reference resolves to an entity of an incompatible kind.
    ReferenceKindMismatch,

    /// A reference points to more than one entity because its identity
    /// is duplicated in the supplied model.
    AmbiguousReference,

    /// A reference points to its own entity when self-reference is forbidden.
    ForbiddenSelfReference,

    /// The same role refers to the same target more than once.
    DuplicateReference,

    /// The configured entity-validation budget was exceeded.
    EntityBudgetExceeded,
}

impl InvariantViolationCode {
    /// Return the stable machine-readable diagnostic code.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InvalidIdentity => {
                "classical.invariants.invalid_identity"
            }
            Self::DuplicateIdentity => {
                "classical.invariants.duplicate_identity"
            }
            Self::MissingRequiredReference => {
                "classical.invariants.missing_required_reference"
            }
            Self::UnresolvedReference => {
                "classical.invariants.unresolved_reference"
            }
            Self::ReferenceKindMismatch => {
                "classical.invariants.reference_kind_mismatch"
            }
            Self::AmbiguousReference => {
                "classical.invariants.ambiguous_reference"
            }
            Self::ForbiddenSelfReference => {
                "classical.invariants.forbidden_self_reference"
            }
            Self::DuplicateReference => {
                "classical.invariants.duplicate_reference"
            }
            Self::EntityBudgetExceeded => {
                "classical.invariants.entity_budget_exceeded"
            }
        }
    }
}

/// Identifies the location of a finding in the supplied model.
///
/// Indices are zero-based positions in the original input. They are
/// diagnostic locations, not canonical semantic identities.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum InvariantSubject {
    /// The finding applies to the model as a whole.
    Model,

    /// The finding applies to an entity.
    Entity {
        /// Zero-based position in the supplied entity collection.
        index: usize,
    },

    /// The finding applies to one entity reference.
    Reference {
        /// Zero-based position of the referencing entity.
        entity_index: usize,

        /// Zero-based position in that entity's reference collection.
        reference_index: usize,
    },
}

/// One structured invariant finding.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InvariantViolation {
    code: InvariantViolationCode,
    subject: InvariantSubject,
    message: String,
}

impl InvariantViolation {
    /// Construct a structured invariant finding.
    #[must_use]
    pub fn new(
        code: InvariantViolationCode,
        subject: InvariantSubject,
        message: impl Into<String>,
    ) -> Self {
        Self {
            code,
            subject,
            message: message.into(),
        }
    }

    /// Return the violation category.
    #[must_use]
    pub const fn code(&self) -> InvariantViolationCode {
        self.code
    }

    /// Return the stable machine-readable code.
    #[must_use]
    pub const fn code_str(&self) -> &'static str {
        self.code.as_str()
    }

    /// Return the input location associated with this finding.
    #[must_use]
    pub const fn subject(&self) -> InvariantSubject {
        self.subject
    }

    /// Return the human-readable explanation.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

/// Validation completeness for one invocation.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum InvariantValidationCompleteness {
    /// Every supplied entity within the input snapshot was examined.
    ///
    /// The report may still contain invariant violations.
    #[default]
    Complete,

    /// The validator did not examine the complete supplied model.
    Incomplete,
}

/// Explicit operational policy for invariant validation.
///
/// `None` does not mean that infinite memory or infinite execution time is
/// available. It means only that this policy introduces no additional
/// entity-count budget.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct InvariantValidationPolicy {
    max_entities: Option<usize>,
}

impl InvariantValidationPolicy {
    /// Construct a policy without an additional entity-count budget.
    #[must_use]
    pub const fn unbounded() -> Self {
        Self { max_entities: None }
    }

    /// Construct a policy with an optional operational entity budget.
    ///
    /// Exceeding this budget makes the report incomplete; it does not
    /// establish that the program or semantic model is inherently invalid.
    #[must_use]
    pub const fn new(max_entities: Option<usize>) -> Self {
        Self { max_entities }
    }

    /// Return the configured entity budget.
    #[must_use]
    pub const fn max_entities(self) -> Option<usize> {
        self.max_entities
    }
}

/// A canonical-model reference exposed to the invariant validator.
///
/// `role` identifies the semantic relationship, for example `operand`,
/// `callee`, `definition`, or `parent`. Its meaning is owned by the adapter
/// and canonical semantic model.
///
/// `expected_kind`, when present, requires the referenced entity to have
/// exactly that kind according to the kind type's equality semantics.
///
/// An absent target represents a missing reference. It is accepted only
/// when `required` is false.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InvariantReference<I, K> {
    role: String,
    target: Option<I>,
    expected_kind: Option<K>,
    required: bool,
    allow_self_reference: bool,
}

impl<I, K> InvariantReference<I, K> {
    /// Construct a reference to a target identity.
    #[must_use]
    pub fn to(
        role: impl Into<String>,
        target: I,
        expected_kind: Option<K>,
        required: bool,
        allow_self_reference: bool,
    ) -> Self {
        Self {
            role: role.into(),
            target: Some(target),
            expected_kind,
            required,
            allow_self_reference,
        }
    }

    /// Construct an explicitly absent reference.
    ///
    /// This is valid only for an optional relationship.
    #[must_use]
    pub fn absent(
        role: impl Into<String>,
        required: bool,
        expected_kind: Option<K>,
    ) -> Self {
        Self {
            role: role.into(),
            target: None,
            expected_kind,
            required,
            allow_self_reference: false,
        }
    }

    /// Return the relationship role.
    #[must_use]
    pub fn role(&self) -> &str {
        &self.role
    }

    /// Return the referenced canonical identity, if present.
    #[must_use]
    pub fn target(&self) -> Option<&I> {
        self.target.as_ref()
    }

    /// Return the expected target kind, if constrained.
    #[must_use]
    pub fn expected_kind(&self) -> Option<&K> {
        self.expected_kind.as_ref()
    }

    /// Return whether this reference must have a target.
    #[must_use]
    pub const fn is_required(&self) -> bool {
        self.required
    }

    /// Return whether a reference to the owning entity is permitted.
    #[must_use]
    pub const fn allows_self_reference(&self) -> bool {
        self.allow_self_reference
    }
}

/// A read-only entity view supplied by the canonical model adapter.
///
/// This is a validation view, not a replacement for a canonical AST node,
/// semantic declaration, ZUIR operation, or classical IR entity.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InvariantEntity<I, K> {
    id: I,
    kind: K,
    references: Vec<InvariantReference<I, K>>,
}

impl<I, K> InvariantEntity<I, K> {
    /// Construct an entity view using its canonical identity and kind.
    #[must_use]
    pub fn new(
        id: I,
        kind: K,
        references: Vec<InvariantReference<I, K>>,
    ) -> Self {
        Self {
            id,
            kind,
            references,
        }
    }

    /// Return the canonical identity.
    #[must_use]
    pub fn id(&self) -> &I {
        &self.id
    }

    /// Return the entity kind.
    #[must_use]
    pub fn kind(&self) -> &K {
        &self.kind
    }

    /// Return references in their authoritative input order.
    #[must_use]
    pub fn references(&self) -> &[InvariantReference<I, K>] {
        &self.references
    }
}

/// Read-only access to canonical classical entities.
///
/// Adapters must return a consistent snapshot for the duration of
/// validation. They must preserve canonical IDs, entity kinds, and all
/// semantically relevant references.
///
/// Invalid identity representations must be reported by the adapter using
/// `is_valid_identity`. This method must not silently normalize distinct
/// canonical identities into one identity.
pub trait ClassicalInvariantModel {
    /// Canonical identity type.
    type Id: Clone + Ord + fmt::Debug;

    /// Canonical entity-kind type.
    type Kind: Clone + Eq + fmt::Debug;

    /// Return the authoritative entity snapshot.
    fn entities(&self) -> &[InvariantEntity<Self::Id, Self::Kind>];

    /// Determine whether an identity is valid under the canonical ID policy.
    ///
    /// For opaque typed IDs, this normally delegates to the canonical ID
    /// implementation. It must not impose hardware or collection-size limits.
    fn is_valid_identity(&self, id: &Self::Id) -> bool;
}

/// Result of invariant validation.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct InvariantValidationReport {
    completeness: InvariantValidationCompleteness,
    violations: Vec<InvariantViolation>,
    entities_examined: usize,
    references_examined: usize,
}

impl InvariantValidationReport {
    /// Create an empty report.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Return the completeness state.
    #[must_use]
    pub const fn completeness(&self) -> InvariantValidationCompleteness {
        self.completeness
    }

    /// Return whether all supplied entities were examined.
    #[must_use]
    pub const fn is_complete(&self) -> bool {
        matches!(
            self.completeness,
            InvariantValidationCompleteness::Complete
        )
    }

    /// Return whether validation completed without violations.
    ///
    /// This result covers only the invariants implemented by this module.
    /// It is not proof of type correctness, control-flow correctness,
    /// resource feasibility, or executability.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.is_complete() && self.violations.is_empty()
    }

    /// Return all violations in deterministic report order.
    #[must_use]
    pub fn violations(&self) -> &[InvariantViolation] {
        &self.violations
    }

    /// Return the number of violations.
    #[must_use]
    pub fn violation_count(&self) -> usize {
        self.violations.len()
    }

    /// Return the number of entities actually examined.
    #[must_use]
    pub const fn entities_examined(&self) -> usize {
        self.entities_examined
    }

    /// Return the number of references actually examined.
    #[must_use]
    pub const fn references_examined(&self) -> usize {
        self.references_examined
    }

    /// Return whether a violation of the given category exists.
    #[must_use]
    pub fn has_violation(&self, code: InvariantViolationCode) -> bool {
        self.violations.iter().any(|violation| violation.code() == code)
    }

    fn mark_incomplete(&mut self) {
        self.completeness = InvariantValidationCompleteness::Incomplete;
    }

    fn push(&mut self, violation: InvariantViolation) {
        self.violations.push(violation);
    }
}

/// Validate identities and cross-object references in a canonical model.
///
/// The validator preserves entity and reference order. It does not perform
/// recursive graph traversal, cycle detection, type inference, or domain
/// semantic evaluation.
///
/// Duplicate identities are reported. References to duplicated identities
/// are reported as ambiguous instead of being resolved to an arbitrary
/// entity.
///
/// An entity budget is checked before validation begins. When exceeded,
/// only the permitted prefix is examined, and the report is incomplete.
///
/// # Complexity
///
/// Let `n` be the number of examined entities and `r` the number of references
/// in those entities. Identity indexing costs `O(n log n)` and reference
/// lookup costs `O(r log n)`, excluding allocation and diagnostic text costs.
///
/// No fixed language-level maximum is imposed on `n` or `r`.
pub fn validate_invariants<M>(
    model: &M,
    policy: InvariantValidationPolicy,
) -> InvariantValidationReport
where
    M: ClassicalInvariantModel + ?Sized,
{
    let mut report = InvariantValidationReport::new();
    let entities = model.entities();

    let count = policy
        .max_entities()
        .map_or(entities.len(), |limit| entities.len().min(limit));

    if count < entities.len() {
        report.mark_incomplete();
        report.push(InvariantViolation::new(
            InvariantViolationCode::EntityBudgetExceeded,
            InvariantSubject::Model,
            format!(
                "Invariant validation budget exhausted: {} entities supplied; \
                 this invocation may examine {count}.",
                entities.len()
            ),
        ));
    }

    // Index only the portion of the snapshot this invocation will examine.
    // A BTreeMap provides deterministic lookup and avoids hash iteration
    // affecting the output order.
    let mut indices: BTreeMap<M::Id, Vec<usize>> = BTreeMap::new();

    for (index, entity) in entities.iter().take(count).enumerate() {
        report.entities_examined += 1;

        if !model.is_valid_identity(entity.id()) {
            report.push(InvariantViolation::new(
                InvariantViolationCode::InvalidIdentity,
                InvariantSubject::Entity { index },
                format!(
                    "Entity at input index {index} has an invalid canonical identity: {:?}.",
                    entity.id()
                ),
            ));
        }

        indices
            .entry(entity.id().clone())
            .or_default()
            .push(index);
    }

    // Report duplicate IDs in original input order, not map order.
    for (index, entity) in entities.iter().take(count).enumerate() {
        if indices
            .get(entity.id())
            .is_some_and(|matches| matches.len() > 1)
        {
            report.push(InvariantViolation::new(
                InvariantViolationCode::DuplicateIdentity,
                InvariantSubject::Entity { index },
                format!(
                    "Canonical identity {:?} occurs more than once in the supplied model.",
                    entity.id()
                ),
            ));
        }
    }

    // Validate references in their authoritative entity/reference order.
    for (entity_index, entity) in entities.iter().take(count).enumerate() {
        let mut seen_references: BTreeSet<(String, M::Id)> = BTreeSet::new();

        for (reference_index, reference) in entity.references().iter().enumerate() {
            report.references_examined += 1;

            let Some(target) = reference.target() else {
                if reference.is_required() {
                    report.push(InvariantViolation::new(
                        InvariantViolationCode::MissingRequiredReference,
                        InvariantSubject::Reference {
                            entity_index,
                            reference_index,
                        },
                        format!(
                            "Required reference role {:?} has no target.",
                            reference.role()
                        ),
                    ));
                }

                continue;
            };

            if !reference.allows_self_reference() && target == entity.id() {
                report.push(InvariantViolation::new(
                    InvariantViolationCode::ForbiddenSelfReference,
                    InvariantSubject::Reference {
                        entity_index,
                        reference_index,
                    },
                    format!(
                        "Reference role {:?} points to its owning entity.",
                        reference.role()
                    ),
                ));
            }

            if !seen_references.insert((reference.role().to_owned(), target.clone())) {
                report.push(InvariantViolation::new(
                    InvariantViolationCode::DuplicateReference,
                    InvariantSubject::Reference {
                        entity_index,
                        reference_index,
                    },
                    format!(
                        "Reference role {:?} repeats the same target identity {:?}.",
                        reference.role(),
                        target
                    ),
                ));
            }

            let Some(target_indices) = indices.get(target) else {
                report.push(InvariantViolation::new(
                    InvariantViolationCode::UnresolvedReference,
                    InvariantSubject::Reference {
                        entity_index,
                        reference_index,
                    },
                    format!(
                        "Reference role {:?} targets missing identity {:?}.",
                        reference.role(),
                        target
                    ),
                ));
                continue;
            };

            if target_indices.len() != 1 {
                report.push(InvariantViolation::new(
                    InvariantViolationCode::AmbiguousReference,
                    InvariantSubject::Reference {
                        entity_index,
                        reference_index,
                    },
                    format!(
                        "Reference role {:?} targets identity {:?}, which occurs {} times.",
                        reference.role(),
                        target,
                        target_indices.len()
                    ),
                ));
                continue;
            }

            if let Some(expected_kind) = reference.expected_kind() {
                let target_index = target_indices[0];
                let actual_kind = entities[target_index].kind();

                if actual_kind != expected_kind {
                    report.push(InvariantViolation::new(
                        InvariantViolationCode::ReferenceKindMismatch,
                        InvariantSubject::Reference {
                            entity_index,
                            reference_index,
                        },
                        format!(
                            "Reference role {:?} expects target kind {:?}, but identity {:?} has kind {:?}.",
                            reference.role(),
                            expected_kind,
                            target,
                            actual_kind
                        ),
                    ));
                }
            }
        }
    }

    report
}

/// Errors returned when a caller requires a complete, violation-free report.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum InvariantValidationError {
    /// Validation was incomplete.
    Incomplete {
        /// Number of findings in the report.
        violation_count: usize,
    },

    /// Validation completed but detected invariant violations.
    Invalid {
        /// Number of findings in the report.
        violation_count: usize,
    },
}

impl fmt::Display for InvariantValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Incomplete { violation_count } => write!(
                formatter,
                "classical invariant validation is incomplete: \
                 {violation_count} finding(s)"
            ),
            Self::Invalid { violation_count } => write!(
                formatter,
                "classical invariant validation failed: \
                 {violation_count} violation(s)"
            ),
        }
    }
}

impl Error for InvariantValidationError {}

/// Require a complete, violation-free report.
///
/// Incomplete validation is never converted into success.
pub fn ensure_invariants_valid(
    report: &InvariantValidationReport,
) -> InvariantValidationResult<()> {
    if !report.is_complete() {
        return Err(InvariantValidationError::Incomplete {
            violation_count: report.violation_count(),
        });
    }

    if !report.violations().is_empty() {
        return Err(InvariantValidationError::Invalid {
            violation_count: report.violation_count(),
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
    struct TestId(u32);

    #[derive(Clone, Debug, Eq, PartialEq)]
    enum TestKind {
        Function,
        Value,
        Block,
    }

    struct TestModel {
        entities: Vec<InvariantEntity<TestId, TestKind>>,
    }

    impl ClassicalInvariantModel for TestModel {
        type Id = TestId;
        type Kind = TestKind;

        fn entities(&self) -> &[InvariantEntity<Self::Id, Self::Kind>] {
            &self.entities
        }

        fn is_valid_identity(&self, id: &Self::Id) -> bool {
            id.0 != 0
        }
    }

    fn entity(
        id: u32,
        kind: TestKind,
        references: Vec<InvariantReference<TestId, TestKind>>,
    ) -> InvariantEntity<TestId, TestKind> {
        InvariantEntity::new(TestId(id), kind, references)
    }

    fn reference(
        role: &str,
        target: u32,
        expected_kind: Option<TestKind>,
    ) -> InvariantReference<TestId, TestKind> {
        InvariantReference::to(
            role,
            TestId(target),
            expected_kind,
            true,
            false,
        )
    }

    #[test]
    fn empty_model_is_valid() {
        let model = TestModel {
            entities: Vec::new(),
        };

        let report =
            validate_invariants(&model, InvariantValidationPolicy::unbounded());

        assert!(report.is_complete());
        assert!(report.is_valid());
        assert_eq!(report.entities_examined(), 0);
        assert_eq!(report.references_examined(), 0);
        assert!(ensure_invariants_valid(&report).is_ok());
    }

    #[test]
    fn valid_reference_to_expected_kind_is_accepted() {
        let model = TestModel {
            entities: vec![
                entity(
                    1,
                    TestKind::Function,
                    vec![reference("entry", 2, Some(TestKind::Block))],
                ),
                entity(2, TestKind::Block, Vec::new()),
            ],
        };

        let report =
            validate_invariants(&model, InvariantValidationPolicy::unbounded());

        assert!(report.is_valid(), "{:?}", report.violations());
    }

    #[test]
    fn invalid_identity_is_reported() {
        let model = TestModel {
            entities: vec![entity(0, TestKind::Value, Vec::new())],
        };

        let report =
            validate_invariants(&model, InvariantValidationPolicy::unbounded());

        assert!(report.has_violation(InvariantViolationCode::InvalidIdentity));
        assert!(!report.is_valid());
    }

    #[test]
    fn duplicate_identities_are_reported() {
        let model = TestModel {
            entities: vec![
                entity(1, TestKind::Value, Vec::new()),
                entity(1, TestKind::Block, Vec::new()),
            ],
        };

        let report =
            validate_invariants(&model, InvariantValidationPolicy::unbounded());

        assert_eq!(
            report
                .violations()
                .iter()
                .filter(|v| v.code() == InvariantViolationCode::DuplicateIdentity)
                .count(),
            2
        );
    }

    #[test]
    fn unresolved_reference_is_reported() {
        let model = TestModel {
            entities: vec![entity(
                1,
                TestKind::Function,
                vec![reference("callee", 99, None)],
            )],
        };

        let report =
            validate_invariants(&model, InvariantValidationPolicy::unbounded());

        assert!(report.has_violation(InvariantViolationCode::UnresolvedReference));
    }

    #[test]
    fn duplicated_target_identity_is_ambiguous() {
        let model = TestModel {
            entities: vec![
                entity(
                    1,
                    TestKind::Function,
                    vec![reference("callee", 2, None)],
                ),
                entity(2, TestKind::Function, Vec::new()),
                entity(2, TestKind::Value, Vec::new()),
            ],
        };

        let report =
            validate_invariants(&model, InvariantValidationPolicy::unbounded());

        assert!(report.has_violation(InvariantViolationCode::AmbiguousReference));
    }

    #[test]
    fn wrong_target_kind_is_reported() {
        let model = TestModel {
            entities: vec![
                entity(
                    1,
                    TestKind::Function,
                    vec![reference("entry", 2, Some(TestKind::Block))],
                ),
                entity(2, TestKind::Value, Vec::new()),
            ],
        };

        let report =
            validate_invariants(&model, InvariantValidationPolicy::unbounded());

        assert!(report.has_violation(InvariantViolationCode::ReferenceKindMismatch));
    }

    #[test]
    fn required_missing_reference_is_reported() {
        let model = TestModel {
            entities: vec![entity(
                1,
                TestKind::Function,
                vec![InvariantReference::absent("entry", true, None)],
            )],
        };

        let report =
            validate_invariants(&model, InvariantValidationPolicy::unbounded());

        assert!(report.has_violation(
            InvariantViolationCode::MissingRequiredReference
        ));
    }

    #[test]
    fn forbidden_self_reference_is_reported() {
        let model = TestModel {
            entities: vec![entity(
                1,
                TestKind::Block,
                vec![reference("parent", 1, None)],
            )],
        };

        let report =
            validate_invariants(&model, InvariantValidationPolicy::unbounded());

        assert!(report.has_violation(
            InvariantViolationCode::ForbiddenSelfReference
        ));
    }

    #[test]
    fn duplicate_role_and_target_is_reported() {
        let model = TestModel {
            entities: vec![
                entity(
                    1,
                    TestKind::Function,
                    vec![
                        reference("callee", 2, None),
                        reference("callee", 2, None),
                    ],
                ),
                entity(2, TestKind::Function, Vec::new()),
            ],
        };

        let report =
            validate_invariants(&model, InvariantValidationPolicy::unbounded());

        assert!(report.has_violation(InvariantViolationCode::DuplicateReference));
    }

    #[test]
    fn distinct_roles_may_reference_the_same_target() {
        let model = TestModel {
            entities: vec![
                entity(
                    1,
                    TestKind::Function,
                    vec![
                        reference("entry", 2, None),
                        reference("fallback", 2, None),
                    ],
                ),
                entity(2, TestKind::Block, Vec::new()),
            ],
        };

        let report =
            validate_invariants(&model, InvariantValidationPolicy::unbounded());

        assert!(!report.has_violation(InvariantViolationCode::DuplicateReference));
    }

    #[test]
    fn budget_exhaustion_is_incomplete_not_valid() {
        let model = TestModel {
            entities: vec![
                entity(1, TestKind::Value, Vec::new()),
                entity(2, TestKind::Value, Vec::new()),
            ],
        };

        let report =
            validate_invariants(&model, InvariantValidationPolicy::new(Some(1)));

        assert!(!report.is_complete());
        assert!(!report.is_valid());
        assert_eq!(report.entities_examined(), 1);
        assert!(report.has_violation(
            InvariantViolationCode::EntityBudgetExceeded
        ));
        assert!(matches!(
            ensure_invariants_valid(&report),
            Err(InvariantValidationError::Incomplete { .. })
        ));
    }

    #[test]
    fn zero_budget_is_explicit() {
        let model = TestModel {
            entities: vec![entity(1, TestKind::Value, Vec::new())],
        };

        let report =
            validate_invariants(&model, InvariantValidationPolicy::new(Some(0)));

        assert!(!report.is_complete());
        assert_eq!(report.entities_examined(), 0);
    }

    #[test]
    fn report_order_is_deterministic() {
        let model = TestModel {
            entities: vec![
                entity(
                    1,
                    TestKind::Function,
                    vec![
                        reference("callee", 90, None),
                        reference("entry", 91, None),
                    ],
                ),
            ],
        };

        let first =
            validate_invariants(&model, InvariantValidationPolicy::unbounded());
        let second =
            validate_invariants(&model, InvariantValidationPolicy::unbounded());

        assert_eq!(first, second);
    }

    #[test]
    fn issue_codes_are_stable() {
        assert_eq!(
            InvariantViolationCode::DuplicateIdentity.as_str(),
            "classical.invariants.duplicate_identity"
        );
        assert_eq!(
            InvariantViolationCode::UnresolvedReference.as_str(),
            "classical.invariants.unresolved_reference"
        );
    }
}
