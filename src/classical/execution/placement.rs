
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Classical Execution Placement
//!
//! Path: `src/classical/execution/placement.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Represent abstract placement requests for downstream classical execution
//! planning. A request describes relationships, mandatory conditions,
//! constraints, preferences, and advisory hints for a logical subject.
//!
//! This module deliberately does not select or assign physical resources.
//!
//! ## Authority
//!
//! - `grammar/Zamani.g4`: grammar composition root.
//! - `grammar/antlr/ZamaniLexer.g4`: canonical lexer boundary.
//! - `grammar/antlr/ZamaniParser.g4`: canonical parser boundary.
//! - `grammar/resources/placement.g4`: reusable placement-intent syntax.
//! - `grammar/execution/placement.g4`: execution placement syntax.
//! - `grammar/distributed/placement.g4`: distributed placement composition.
//! - `grammar/specification/semantic-model.md`: target-independent semantics.
//! - `grammar/specification/execution-model.md`: execution boundaries.
//! - `grammar/specification/poco-reaf.md`: portability and scalability.
//! - `grammar/specification/scalability-model.md`: resource-parametric scaling.
//! - `src/classical/execution/plan.rs`: target-independent execution plans.
//! - `src/classical/execution/requirements.rs`: canonical requirements adapter.
//! - `src/classical/execution/capabilities.rs`: capability assessment.
//! - `src/frontend/semantic/resources/model.rs`: canonical resource semantics.
//!
//! The grammar defines syntax. The specifications define language meaning.
//! The frontend owns source and semantic identities. Execution planning
//! consumes normalized placement requests and determines feasibility.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - A target-independent placement request.
//! - An extensible placement property name.
//! - A logical placement subject reference.
//! - The classification of a placement clause as a requirement, constraint,
//!   preference, or hint.
//! - An opaque expression/value payload supplied by an upstream adapter.
//! - Insertion-order-preserving clause storage and validation.
//!
//! This module does not own:
//!
//! - AST nodes, source parsing, name resolution, or expression evaluation.
//! - A second resource or capability model.
//! - Physical device identifiers, device inventories, or topology.
//! - Target selection, physical placement, allocation, partitioning, or routing.
//! - Scheduling, runtime handles, or execution.
//! - Distributed replication, networking, quantum placement, or HDL realization.
//! - A universal list of placement properties or target types.
//!
//! ## Integration contract
//!
//! An upstream semantic adapter converts validated source constructs into
//! `PlacementRequest` values. It must resolve the logical subject and preserve
//! the meaning of each clause. The `value` payload is intentionally opaque:
//! this module does not parse, evaluate, or reinterpret expressions.
//!
//! Downstream feasibility and realization components consume the request,
//! canonical resource requirements, and capability evidence. They must
//! distinguish mandatory clauses from advisory preferences and hints.
//!
//! A successful construction means only that the request is structurally
//! valid. It does not mean that any target can satisfy it.
//!
//! ## Scalability
//!
//! There is no universal maximum number of clauses, subjects, properties,
//! devices, or target types. The request grows according to its input and
//! available memory. It makes no promise of literal infinite capacity.
//!
//! ## Determinism
//!
//! Clauses are preserved in insertion order, including duplicates. No
//! implicit sorting, merging, precedence resolution, or deduplication occurs.
//! A downstream evaluator must define any normalization policy explicitly.
//!
//! ## Completion status
//!
//! Proposed implementation. Compilation, integration, and conformance tests
//! must pass before this module is marked verified.

use core::fmt;
use std::error::Error;

/// Result type used by this module.
pub type PlacementResult<T> = Result<T, PlacementError>;

/// Classification of a placement clause.
///
/// These classifications are deliberately separate. A preference or hint
/// must not silently become a mandatory requirement, and a mandatory
/// requirement must not be discarded as advisory metadata.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum PlacementClauseKind {
    /// A mandatory placement condition that a valid realization must satisfy.
    Requirement,

    /// A condition that restricts the set of permitted realizations.
    Constraint,

    /// An optimization preference that may be traded off when policy permits.
    Preference,

    /// Advisory information that a planner may use but is not required to obey.
    Hint,
}

impl fmt::Display for PlacementClauseKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Requirement => "requirement",
            Self::Constraint => "constraint",
            Self::Preference => "preference",
            Self::Hint => "hint",
        })
    }
}

/// An extensible logical placement subject.
///
/// A subject identifies the logical computation, region, stage, workload, or
/// other entity to which placement clauses apply. It is not a physical device
/// identifier and is not interpreted by this module.
///
/// The string is validated for non-emptiness and control characters only.
/// Domain-specific name resolution belongs to the semantic adapter.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PlacementSubject(String);

impl PlacementSubject {
    /// Construct a logical placement subject.
    ///
    /// The original spelling is preserved. This constructor does not perform
    /// name resolution, case folding, or namespace lookup.
    ///
    /// # Errors
    ///
    /// Returns `PlacementError::EmptyValue` for an empty or whitespace-only
    /// subject, or `PlacementError::ControlCharacter` if it contains a
    /// control character.
    pub fn new(value: impl Into<String>) -> PlacementResult<Self> {
        let value = value.into();
        validate_non_empty("placement subject", &value)?;
        validate_no_control_characters("placement subject", &value)?;
        Ok(Self(value))
    }

    /// Return the subject's original textual representation.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for PlacementSubject {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// An extensible placement property identifier.
///
/// Examples of possible properties include locality, affinity, separation,
/// scope, target class, mobility, latency intent, or future domain-specific
/// properties. These examples are not a closed list.
///
/// A property name is not a guarantee that any planner or target supports it.
/// Support must be established by the appropriate semantic and feasibility
/// infrastructure.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PlacementProperty(String);

impl PlacementProperty {
    /// Construct a placement property name.
    ///
    /// Names may be qualified and may use future namespaces. This constructor
    /// does not require the property to be registered or supported.
    ///
    /// # Errors
    ///
    /// Returns an error for an empty name or a name containing control
    /// characters.
    pub fn new(value: impl Into<String>) -> PlacementResult<Self> {
        let value = value.into();
        validate_non_empty("placement property", &value)?;
        validate_no_control_characters("placement property", &value)?;
        Ok(Self(value))
    }

    /// Return the original property name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for PlacementProperty {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// An opaque placement value supplied by an upstream adapter.
///
/// The value may represent a resolved symbolic expression, a canonical
/// expression reference, a policy reference, or a serialized scalar.
/// This type does not define the expression language and must not be used
/// as a substitute for the canonical AST or semantic expression model.
///
/// Adapters should prefer a stable canonical representation or reference
/// where one is available, rather than relying on display text for semantic
/// equality or cross-version interchange.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PlacementValue(String);

impl PlacementValue {
    /// Construct a placement value from a non-empty opaque representation.
    ///
    /// The representation is preserved exactly. This method does not parse
    /// or evaluate it.
    ///
    /// # Errors
    ///
    /// Returns an error for an empty representation or one containing
    /// control characters.
    pub fn new(value: impl Into<String>) -> PlacementResult<Self> {
        let value = value.into();
        validate_non_empty("placement value", &value)?;
        validate_no_control_characters("placement value", &value)?;
        Ok(Self(value))
    }

    /// Return the opaque representation.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for PlacementValue {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// One placement clause attached to a logical subject.
///
/// A clause carries classification, property, and opaque value. It does not
/// determine satisfiability, priority between clauses, or target support.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlacementClause {
    kind: PlacementClauseKind,
    property: PlacementProperty,
    value: PlacementValue,
}

impl PlacementClause {
    /// Construct a placement clause from validated components.
    #[must_use]
    pub const fn new(
        kind: PlacementClauseKind,
        property: PlacementProperty,
        value: PlacementValue,
    ) -> Self {
        Self {
            kind,
            property,
            value,
        }
    }

    /// Return the clause classification.
    #[must_use]
    pub const fn kind(&self) -> PlacementClauseKind {
        self.kind
    }

    /// Return the property named by this clause.
    #[must_use]
    pub const fn property(&self) -> &PlacementProperty {
        &self.property
    }

    /// Return the opaque value associated with this clause.
    #[must_use]
    pub const fn value(&self) -> &PlacementValue {
        &self.value
    }
}

/// A target-independent placement request for one logical subject.
///
/// Requests retain all clauses in insertion order. Repeated clauses are
/// preserved because duplicates may carry separate source origins or
/// represent distinct declarations that require semantic reconciliation.
///
/// Construction does not imply that the clauses are satisfiable, that a
/// target supports their properties, or that any resources have been
/// reserved.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlacementRequest {
    subject: PlacementSubject,
    clauses: Vec<PlacementClause>,
}

impl PlacementRequest {
    /// Create an empty placement request for a logical subject.
    #[must_use]
    pub const fn new(subject: PlacementSubject) -> Self {
        Self {
            subject,
            clauses: Vec::new(),
        }
    }

    /// Return the logical subject.
    #[must_use]
    pub const fn subject(&self) -> &PlacementSubject {
        &self.subject
    }

    /// Return all clauses in insertion order.
    #[must_use]
    pub fn clauses(&self) -> &[PlacementClause] {
        &self.clauses
    }

    /// Return the number of clauses in this request.
    #[must_use]
    pub fn len(&self) -> usize {
        self.clauses.len()
    }

    /// Return whether the request contains no clauses.
    ///
    /// An empty request is structurally valid, but may be meaningless for a
    /// particular source construct. That decision belongs to semantic
    /// validation rather than this data container.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.clauses.is_empty()
    }

    /// Append a clause while preserving insertion order and duplicates.
    pub fn push(&mut self, clause: PlacementClause) {
        self.clauses.push(clause);
    }

    /// Append all clauses from another request with the same subject.
    ///
    /// The clauses from `other` are appended in their original order.
    /// No deduplication or semantic merging occurs.
    ///
    /// # Errors
    ///
    /// Returns `PlacementError::SubjectMismatch` if the subjects differ.
    pub fn append(&mut self, other: &Self) -> PlacementResult<()> {
        if self.subject != other.subject {
            return Err(PlacementError::SubjectMismatch {
                left: self.subject.as_str().to_owned(),
                right: other.subject.as_str().to_owned(),
            });
        }

        self.clauses.extend(other.clauses.iter().cloned());
        Ok(())
    }

    /// Return clauses of a requested classification in original order.
    ///
    /// The returned iterator borrows this request and allocates no
    /// intermediate collection.
    pub fn clauses_of_kind(
        &self,
        kind: PlacementClauseKind,
    ) -> impl Iterator<Item = &PlacementClause> {
        self.clauses.iter().filter(move |clause| clause.kind == kind)
    }

    /// Return whether this request contains a clause for the given property.
    ///
    /// This is a structural query only. It does not evaluate the property's
    /// value or determine whether the property is supported.
    #[must_use]
    pub fn contains_property(&self, property: &PlacementProperty) -> bool {
        self.clauses
            .iter()
            .any(|clause| clause.property == *property)
    }
}

/// Errors detected while constructing or combining placement requests.
///
/// These are structural errors only. Unsatisfied constraints, unsupported
/// properties, and unavailable resources belong to downstream feasibility
/// assessment and are not represented as constructor errors.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum PlacementError {
    /// A subject, property, or value was empty or whitespace-only.
    EmptyValue {
        /// The kind of value that was empty.
        field: &'static str,
    },

    /// A subject, property, or value contained a control character.
    ControlCharacter {
        /// The kind of value containing the character.
        field: &'static str,
    },

    /// Two requests referred to different logical subjects.
    SubjectMismatch {
        /// Subject of the request being extended.
        left: String,

        /// Subject of the appended request.
        right: String,
    },
}

impl fmt::Display for PlacementError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyValue { field } => {
                write!(formatter, "{field} must not be empty")
            }
            Self::ControlCharacter { field } => {
                write!(formatter, "{field} must not contain control characters")
            }
            Self::SubjectMismatch { left, right } => {
                write!(
                    formatter,
                    "cannot combine placement requests for different subjects: \
                     {left:?} and {right:?}"
                )
            }
        }
    }
}

impl Error for PlacementError {}

fn validate_non_empty(field: &'static str, value: &str) -> PlacementResult<()> {
    if value.trim().is_empty() {
        return Err(PlacementError::EmptyValue { field });
    }

    Ok(())
}

fn validate_no_control_characters(
    field: &'static str,
    value: &str,
) -> PlacementResult<()> {
    if value.chars().any(char::is_control) {
        return Err(PlacementError::ControlCharacter { field });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn subject(value: &str) -> PlacementSubject {
        PlacementSubject::new(value).expect("test subject should be valid")
    }

    fn property(value: &str) -> PlacementProperty {
        PlacementProperty::new(value).expect("test property should be valid")
    }

    fn value(value: &str) -> PlacementValue {
        PlacementValue::new(value).expect("test value should be valid")
    }

    fn clause(
        kind: PlacementClauseKind,
        property_name: &str,
        expression: &str,
    ) -> PlacementClause {
        PlacementClause::new(
            kind,
            property(property_name),
            value(expression),
        )
    }

    #[test]
    fn empty_request_is_structurally_valid() {
        let request = PlacementRequest::new(subject("module::compute"));

        assert!(request.is_empty());
        assert_eq!(request.len(), 0);
        assert!(request.clauses().is_empty());
    }

    #[test]
    fn request_preserves_clause_insertion_order() {
        let mut request = PlacementRequest::new(subject("workload"));

        request.push(clause(
            PlacementClauseKind::Requirement,
            "placement::scope",
            "execution_region",
        ));
        request.push(clause(
            PlacementClauseKind::Preference,
            "placement::locality",
            "near_input",
        ));

        assert_eq!(request.len(), 2);
        assert_eq!(
            request.clauses()[0].property().as_str(),
            "placement::scope"
        );
        assert_eq!(
            request.clauses()[1].property().as_str(),
            "placement::locality"
        );
    }

    #[test]
    fn duplicate_clauses_are_preserved() {
        let mut request = PlacementRequest::new(subject("stage"));

        let repeated = clause(
            PlacementClauseKind::Constraint,
            "placement::affinity",
            "stage_a",
        );

        request.push(repeated.clone());
        request.push(repeated);

        assert_eq!(request.len(), 2);
        assert_eq!(request.clauses()[0], request.clauses()[1]);
    }

    #[test]
    fn clause_filter_preserves_relative_order() {
        let mut request = PlacementRequest::new(subject("stage"));

        request.push(clause(
            PlacementClauseKind::Hint,
            "placement::note",
            "first",
        ));
        request.push(clause(
            PlacementClauseKind::Requirement,
            "placement::scope",
            "region",
        ));
        request.push(clause(
            PlacementClauseKind::Hint,
            "placement::note",
            "second",
        ));

        let hints: Vec<_> = request
            .clauses_of_kind(PlacementClauseKind::Hint)
            .map(|item| item.value().as_str())
            .collect();

        assert_eq!(hints, vec!["first", "second"]);
    }

    #[test]
    fn append_preserves_order_and_duplicates() {
        let mut left = PlacementRequest::new(subject("compute"));
        left.push(clause(
            PlacementClauseKind::Requirement,
            "placement::scope",
            "region_a",
        ));

        let mut right = PlacementRequest::new(subject("compute"));
        right.push(clause(
            PlacementClauseKind::Preference,
            "placement::scope",
            "region_a",
        ));

        left.append(&right).expect("subjects should match");

        assert_eq!(left.len(), 2);
        assert_eq!(
            left.clauses()[0].kind(),
            PlacementClauseKind::Requirement
        );
        assert_eq!(
            left.clauses()[1].kind(),
            PlacementClauseKind::Preference
        );
    }

    #[test]
    fn append_rejects_different_subjects_without_mutating_request() {
        let mut left = PlacementRequest::new(subject("compute_a"));
        left.push(clause(
            PlacementClauseKind::Requirement,
            "placement::scope",
            "region_a",
        ));

        let right = PlacementRequest::new(subject("compute_b"));
        let original = left.clone();

        let result = left.append(&right);

        assert!(matches!(
            result,
            Err(PlacementError::SubjectMismatch { .. })
        ));
        assert_eq!(left, original);
    }

    #[test]
    fn property_lookup_is_structural() {
        let mut request = PlacementRequest::new(subject("compute"));
        request.push(clause(
            PlacementClauseKind::Hint,
            "future::placement::property",
            "symbolic_value",
        ));

        assert!(request.contains_property(&property(
            "future::placement::property"
        )));
        assert!(!request.contains_property(&property(
            "placement::unknown"
        )));
    }

    #[test]
    fn unknown_namespaces_are_accepted() {
        let request_subject = subject("future_domain::workload");
        let property_name = property("vendor::future::placement::metric");
        let opaque_value = value("symbolic::expression::reference");

        let mut request = PlacementRequest::new(request_subject);
        request.push(PlacementClause::new(
            PlacementClauseKind::Hint,
            property_name,
            opaque_value,
        ));

        assert_eq!(request.len(), 1);
    }

    #[test]
    fn empty_subject_is_rejected() {
        assert!(matches!(
            PlacementSubject::new(" \t "),
            Err(PlacementError::EmptyValue { .. })
        ));
    }

    #[test]
    fn empty_property_is_rejected() {
        assert!(matches!(
            PlacementProperty::new(""),
            Err(PlacementError::EmptyValue { .. })
        ));
    }

    #[test]
    fn empty_value_is_rejected() {
        assert!(matches!(
            PlacementValue::new("  "),
            Err(PlacementError::EmptyValue { .. })
        ));
    }

    #[test]
    fn control_characters_are_rejected() {
        assert!(matches!(
            PlacementSubject::new("workload\nname"),
            Err(PlacementError::ControlCharacter { .. })
        ));

        assert!(matches!(
            PlacementProperty::new("placement::\u{0007}scope"),
            Err(PlacementError::ControlCharacter { .. })
        ));

        assert!(matches!(
            PlacementValue::new("value\u{0000}"),
            Err(PlacementError::ControlCharacter { .. })
        ));
    }

    #[test]
    fn large_clause_sets_have_no_semantic_count_ceiling() {
        let mut request = PlacementRequest::new(subject("large_workload"));

        // This exercises growth beyond small inline collections without
        // defining a maximum or implying a universal resource guarantee.
        for index in 0..4096 {
            request.push(clause(
                PlacementClauseKind::Hint,
                "extension::placement::annotation",
                &format!("annotation_{index}"),
            ));
        }

        assert_eq!(request.len(), 4096);
        assert_eq!(
            request.clauses().last().map(|item| item.value().as_str()),
            Some("annotation_4095")
        );
    }

    #[test]
    fn request_clone_preserves_all_clauses() {
        let mut request = PlacementRequest::new(subject("compute"));
        request.push(clause(
            PlacementClauseKind::Constraint,
            "placement::separation",
            "tenant_group",
        ));

        let cloned = request.clone();

        assert_eq!(cloned, request);
    }
}
