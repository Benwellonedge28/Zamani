
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani Semantic Contract Model
//!
//! Path: `src/frontend/semantic/contracts/model.rs`
//!
//! ## PURPOSE
//!
//! Define the canonical, target-independent semantic representation of
//! contracts and their verification obligations.
//!
//! Contracts express conditions that must hold at defined program boundaries.
//! They do not, by themselves, prove that those conditions hold.
//!
//! ## AUTHORITY
//!
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/grammar-authority.md`
//! - `grammar/spec/poco-reaf.md`, where present and normative
//! - `grammar/Zamani.g4`
//! - `grammar/grammar.md`
//! - `src/frontend/ast/node/`
//!
//! The normative specification defines contract meaning. The canonical
//! grammar defines legal source syntax. The AST defines source structure,
//! identity, and source locations.
//!
//! ## OWNS
//!
//! - Contract kinds and clause roles.
//! - References to canonical AST nodes representing contract expressions.
//! - Contract metadata required by semantic analysis.
//! - Verification obligation identities and lifecycle.
//! - Explicit verification outcomes and their evidence references.
//! - Structural validation of the semantic contract representation.
//!
//! ## DOES_NOT_OWN
//!
//! - Contract parsing or AST construction.
//! - Expression evaluation or type inference.
//! - Symbol resolution.
//! - Theorem proving, SMT solving, or proof-kernel implementation.
//! - Runtime assertion execution.
//! - Effect, capability, resource, or authorization policy.
//! - Diagnostic rendering.
//! - Classical IR, quantum IR, HDL IR, or another IR.
//! - Target discovery, physical resource allocation, scheduling,
//!   routing, lowering, or runtime execution.
//!
//! ## INTEGRATION CONTRACT
//!
//! - Source references use the canonical AST `NodeId`.
//! - Source locations use the canonical AST `Span`.
//! - `contracts/preconditions.rs`, `postconditions.rs`, and
//!   `invariants.rs` may construct clauses using these types.
//! - Contract checking consumes clauses and emits verification outcomes.
//! - `model/facts.rs` may associate contract facts with AST nodes.
//! - `model/program.rs` may retain contract obligations using its
//!   obligation payload abstraction.
//! - `diagnostics/` owns user-facing diagnostic records.
//! - `pipeline.rs` determines when contract checking runs.
//!
//! This module does not assume that any of these consumers has already
//! been implemented. Its public types are independent of those consumers.
//!
//! ## SCALABILITY
//!
//! No hardware capacities, domain counts, contract-count limits, or
//! fixed-width resource quantities are defined here.
//!
//! Collections grow according to available memory. Operational limits
//! belong to explicit compiler policy and must be reported as incomplete
//! analysis, not as successful verification.
//!
//! A finite process cannot guarantee infinite computation. POCO-REAF means
//! this model does not impose arbitrary machine-size restrictions on the
//! meaning of a contract.
//!
//! ## VERIFICATION INVARIANT
//!
//! A contract is not proven merely because:
//!
//! - its syntax is valid;
//! - its expression has a valid type;
//! - its expression can be evaluated for one example;
//! - no counterexample has yet been found;
//! - a solver returned an inconclusive result;
//! - analysis stopped before checking it.
//!
//! Only explicit evidence from an authorized verification component may
//! produce `VerificationOutcome::Proven`. The verification component,
//! not this data model, is responsible for the soundness of that evidence.
//!
//! ## FREEZE CRITERIA
//!
//! This file is ready for integration when:
//!
//! 1. its imports match the canonical AST exports;
//! 2. its contract syntax mappings agree with normative specifications;
//! 3. contract and clause references are validated against the AST graph
//!    by the owning analysis pass;
//! 4. verification outcomes cannot be confused with target feasibility;
//! 5. unresolved or incomplete obligations cannot be represented as proven;
//! 6. all tests in this file pass;
//! 7. workspace compilation and downstream integration tests pass.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use crate::frontend::ast::node::{NodeId, Span};

/// Schema version for this semantic contract model.
///
/// This is independent of the Zamani language version and serialized AST
/// schema. Increment it when the contract-model compatibility contract
/// changes.
pub const CONTRACT_MODEL_SCHEMA_VERSION: u16 = 1;

/// Stable architectural name of this module.
pub const CONTRACT_MODEL_NAME: &str = "zamani.frontend.semantic.contracts.model";

/// Semantic role of a contract clause.
///
/// The variants describe when a condition applies, not how it is proved.
/// New source-level roles can be represented through [`ContractKind::Extension`]
/// when their semantics are registered by the owning language extension.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum ContractKind {
    /// A condition required before an operation or function executes.
    Precondition,

    /// A condition required after an operation or function completes.
    Postcondition,

    /// A condition that must remain true throughout a specified region.
    Invariant,

    /// A condition that must hold at a particular program point.
    Assertion,

    /// A condition required at the entry to a loop iteration.
    LoopInvariant,

    /// A condition required for a function's exceptional or failure exit.
    FailureCondition,

    /// An extension-defined contract kind.
    ///
    /// The identifier is namespaced, for example `domain.example.condition`.
    /// Its actual semantics must be supplied by a registered extension.
    Extension(ContractKindId),
}

impl ContractKind {
    /// Return whether this kind requires extension-defined semantics.
    #[must_use]
    pub const fn is_extension(&self) -> bool {
        matches!(self, Self::Extension(_))
    }
}

impl fmt::Display for ContractKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Precondition => f.write_str("precondition"),
            Self::Postcondition => f.write_str("postcondition"),
            Self::Invariant => f.write_str("invariant"),
            Self::Assertion => f.write_str("assertion"),
            Self::LoopInvariant => f.write_str("loop-invariant"),
            Self::FailureCondition => f.write_str("failure-condition"),
            Self::Extension(id) => write!(f, "extension:{id}"),
        }
    }
}

/// Validated, namespaced identifier for an extension-defined contract kind.
///
/// Identifiers are intentionally strings rather than a closed enum so new
/// computational domains can introduce contract kinds without modifying the
/// semantic core.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ContractKindId(String);

impl ContractKindId {
    /// Construct an extension contract-kind identifier.
    ///
    /// The identifier must contain at least two non-empty, dot-separated
    /// components. Components may contain ASCII letters, digits, `_`, and
    /// `-`. A component must not begin or end with `-`.
    ///
    /// Examples:
    ///
    /// - `domain.example`
    /// - `quantum.verification`
    /// - `hardware.timing`
    ///
    /// This validates identifier structure only. It does not register or
    /// authorize the extension.
    pub fn new(value: impl Into<String>) -> Result<Self, ContractModelError> {
        let value = value.into();

        let mut components = value.split('.');
        let first = components.next().unwrap_or_default();

        if first.is_empty() || components.clone().next().is_none() {
            return Err(ContractModelError::InvalidExtensionIdentifier {
                value,
            });
        }

        let valid = std::iter::once(first)
            .chain(components)
            .all(valid_identifier_component);

        if !valid {
            return Err(ContractModelError::InvalidExtensionIdentifier {
                value,
            });
        }

        Ok(Self(value))
    }

    /// Return the canonical identifier string.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consume the identifier and return its string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for ContractKindId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

fn valid_identifier_component(component: &str) -> bool {
    if component.is_empty()
        || component.starts_with('-')
        || component.ends_with('-')
    {
        return false;
    }

    component
        .bytes()
        .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_' || byte == b'-')
}

/// A reference to a condition expression in the canonical AST.
///
/// The expression remains owned by the AST. This model stores its identity
/// rather than copying syntax into a second semantic expression tree.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ContractExpression {
    node_id: NodeId,
    span: Span,
}

impl ContractExpression {
    /// Construct a reference to an AST expression.
    ///
    /// The caller must ensure the node identifies an expression in the
    /// relevant AST graph. Graph-membership and node-kind checks belong to
    /// structural validation and contract extraction.
    #[must_use]
    pub const fn new(node_id: NodeId, span: Span) -> Self {
        Self { node_id, span }
    }

    /// Return the referenced AST node identity.
    #[must_use]
    pub const fn node_id(self) -> NodeId {
        self.node_id
    }

    /// Return the source span associated with the expression.
    #[must_use]
    pub const fn span(self) -> Span {
        self.span
    }
}

/// Optional source-level message associated with a contract clause.
///
/// A message is explanatory metadata. It does not affect the truth value
/// of the contract and must never be used as a substitute for verification.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ContractMessage(String);

impl ContractMessage {
    /// Construct a contract message.
    ///
    /// Empty messages are permitted because a source construct may omit a
    /// message. Rendering policy belongs to diagnostics and tooling.
    #[must_use]
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Return the message text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consume the message and return its text.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

/// One semantic clause belonging to a contract.
///
/// `source_node` identifies the source construct that introduced the clause.
/// `condition` references the expression whose truth must be established.
/// `message` is optional explanatory text.
///
/// This structure does not assert that the condition is true or even
/// type-correct. Those decisions belong to semantic validation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContractClause {
    source_node: NodeId,
    source_span: Span,
    kind: ContractKind,
    condition: ContractExpression,
    message: Option<ContractMessage>,
}

impl ContractClause {
    /// Construct a contract clause from canonical source references.
    #[must_use]
    pub fn new(
        source_node: NodeId,
        source_span: Span,
        kind: ContractKind,
        condition: ContractExpression,
        message: Option<ContractMessage>,
    ) -> Self {
        Self {
            source_node,
            source_span,
            kind,
            condition,
            message,
        }
    }

    /// Return the AST node that introduced this clause.
    #[must_use]
    pub const fn source_node(&self) -> NodeId {
        self.source_node
    }

    /// Return the source span of the clause.
    #[must_use]
    pub const fn source_span(&self) -> Span {
        self.source_span
    }

    /// Return the semantic role of the clause.
    #[must_use]
    pub const fn kind(&self) -> &ContractKind {
        &self.kind
    }

    /// Return the referenced condition expression.
    #[must_use]
    pub const fn condition(&self) -> ContractExpression {
        self.condition
    }

    /// Return the optional explanatory message.
    #[must_use]
    pub fn message(&self) -> Option<&ContractMessage> {
        self.message.as_ref()
    }
}

/// A complete source-level contract attached to a canonical AST construct.
///
/// A contract may contain multiple clauses. The meaning of their
/// combination and ordering is determined by the normative language rules
/// and the owning contract checker, not by insertion order alone.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Contract {
    source_node: NodeId,
    source_span: Span,
    clauses: Vec<ContractClause>,
}

impl Contract {
    /// Construct a contract and validate its local structural invariants.
    ///
    /// This checks duplicate source-clause identities within the contract.
    /// It does not verify AST graph membership, expression types, or truth.
    pub fn new(
        source_node: NodeId,
        source_span: Span,
        clauses: Vec<ContractClause>,
    ) -> Result<Self, ContractModelError> {
        let mut seen = BTreeSet::new();

        for clause in &clauses {
            if !seen.insert(clause.source_node()) {
                return Err(ContractModelError::DuplicateClauseNode {
                    node_id: clause.source_node(),
                });
            }
        }

        Ok(Self {
            source_node,
            source_span,
            clauses,
        })
    }

    /// Return the AST node to which this contract belongs.
    #[must_use]
    pub const fn source_node(&self) -> NodeId {
        self.source_node
    }

    /// Return the source span of the contract.
    #[must_use]
    pub const fn source_span(&self) -> Span {
        self.source_span
    }

    /// Return the contract clauses in source/extraction order.
    #[must_use]
    pub fn clauses(&self) -> &[ContractClause] {
        &self.clauses
    }

    /// Return whether this contract has no clauses.
    ///
    /// Whether an empty contract is legal is a language-level rule.
    /// This method reports structure only.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.clauses.is_empty()
    }

    /// Return the number of clauses in the contract.
    #[must_use]
    pub fn clause_count(&self) -> usize {
        self.clauses.len()
    }

    /// Iterate over clauses of the requested kind.
    pub fn clauses_of_kind(
        &self,
        kind: &ContractKind,
    ) -> impl Iterator<Item = &ContractClause> {
        self.clauses.iter().filter(move |clause| clause.kind() == kind)
    }
}

/// Stable identity for one verification obligation.
///
/// The AST node identifies the source construct. The obligation identity
/// distinguishes multiple obligations arising from that construct.
///
/// This ID is local to an explicitly managed allocation domain; it is not
/// a globally unique or cross-compilation persistent identity.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ObligationId(u64);

impl ObligationId {
    /// Construct an obligation ID from a non-zero raw value.
    pub const fn new(raw: u64) -> Result<Self, ContractModelError> {
        if raw == 0 {
            Err(ContractModelError::ZeroObligationId)
        } else {
            Ok(Self(raw))
        }
    }

    /// Return the raw numeric identity.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }
}

/// Explicit allocator for verification-obligation identities.
///
/// The allocator is an ordinary value, not global mutable state. Callers
/// must use one allocator per identity domain and must not merge IDs from
/// independent allocators without a separate namespace.
///
/// Exhaustion is reported; IDs are never silently wrapped or reused.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObligationIdAllocator {
    next: Option<u64>,
}

impl Default for ObligationIdAllocator {
    fn default() -> Self {
        Self::new()
    }
}

impl ObligationIdAllocator {
    /// Create an allocator whose first issued ID is one.
    #[must_use]
    pub const fn new() -> Self {
        Self { next: Some(1) }
    }

    /// Allocate the next obligation identity.
    pub fn allocate(&mut self) -> Result<ObligationId, ContractModelError> {
        let raw = self
            .next
            .ok_or(ContractModelError::ObligationIdExhausted)?;

        self.next = raw.checked_add(1);

        Ok(ObligationId(raw))
    }
}

/// Origin of a verification obligation.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum ObligationOrigin {
    /// Derived from a precondition.
    Precondition,

    /// Derived from a postcondition.
    Postcondition,

    /// Derived from an invariant.
    Invariant,

    /// Derived from an assertion.
    Assertion,

    /// Derived from an extension-defined contract.
    Extension,

    /// Derived from another semantic rule that requires proof or checking.
    SemanticRule,
}

/// Verification outcome for a contract obligation.
///
/// Only `Proven` means the verification component claims to have established
/// the obligation under its documented soundness assumptions. Consumers must
/// separately decide whether that evidence is trusted and sufficient.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum VerificationOutcome {
    /// No verification decision has been made.
    Pending,

    /// The obligation has been established by the verification component.
    Proven {
        /// Optional identity of evidence produced by the verification system.
        evidence: Option<String>,
    },

    /// A counterexample or other valid refutation was found.
    Disproven {
        /// Optional reference to the counterexample or refutation artifact.
        evidence: Option<String>,
    },

    /// The verification component could not decide the obligation.
    Unknown {
        /// Optional explanation, not intended as a stable diagnostic code.
        reason: Option<String>,
    },

    /// Verification was not performed because the required capability,
    /// theory, or implementation was unavailable.
    Unsupported {
        /// Optional explanation of the unsupported verification requirement.
        reason: Option<String>,
    },

    /// Verification did not finish because an operational budget was reached.
    Incomplete {
        /// Optional explanation of the exhausted budget or interrupted work.
        reason: Option<String>,
    },
}

impl VerificationOutcome {
    /// Return whether the outcome explicitly records successful proof.
    #[must_use]
    pub const fn is_proven(&self) -> bool {
        matches!(self, Self::Proven { .. })
    }

    /// Return whether verification has reached a non-pending outcome.
    ///
    /// An outcome can be terminal without proving the obligation.
    #[must_use]
    pub const fn is_decided(&self) -> bool {
        !matches!(self, Self::Pending)
    }

    /// Return whether the obligation is not proven.
    #[must_use]
    pub const fn requires_attention(&self) -> bool {
        !matches!(self, Self::Proven { .. })
    }
}

/// A verification obligation derived from a contract clause.
///
/// The obligation retains canonical source references and an explicit
/// verification outcome. It does not embed a solver, a proof term, a
/// target-specific result, or runtime state.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContractObligation {
    id: ObligationId,
    contract_node: NodeId,
    clause_node: NodeId,
    condition: ContractExpression,
    origin: ObligationOrigin,
    outcome: VerificationOutcome,
}

impl ContractObligation {
    /// Create a pending obligation from a contract clause.
    #[must_use]
    pub fn pending(
        id: ObligationId,
        contract_node: NodeId,
        clause: &ContractClause,
        origin: ObligationOrigin,
    ) -> Self {
        Self {
            id,
            contract_node,
            clause_node: clause.source_node(),
            condition: clause.condition(),
            origin,
            outcome: VerificationOutcome::Pending,
        }
    }

    /// Return the obligation identity.
    #[must_use]
    pub const fn id(&self) -> ObligationId {
        self.id
    }

    /// Return the AST node owning the contract.
    #[must_use]
    pub const fn contract_node(&self) -> NodeId {
        self.contract_node
    }

    /// Return the AST node introducing the clause.
    #[must_use]
    pub const fn clause_node(&self) -> NodeId {
        self.clause_node
    }

    /// Return the condition expression reference.
    #[must_use]
    pub const fn condition(&self) -> ContractExpression {
        self.condition
    }

    /// Return the obligation origin.
    #[must_use]
    pub const fn origin(&self) -> ObligationOrigin {
        self.origin
    }

    /// Return the current verification outcome.
    #[must_use]
    pub const fn outcome(&self) -> &VerificationOutcome {
        &self.outcome
    }

    /// Replace the current outcome with a verification result.
    ///
    /// This method records the result; it does not authenticate the verifier
    /// or establish proof soundness. The caller must enforce the verification
    /// component's trust and evidence policy.
    pub fn record_outcome(&mut self, outcome: VerificationOutcome) {
        self.outcome = outcome;
    }

    /// Return whether this obligation has been proven.
    #[must_use]
    pub const fn is_proven(&self) -> bool {
        self.outcome.is_proven()
    }
}

/// Errors from constructing or validating the semantic contract model.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ContractModelError {
    /// An extension identifier is malformed or not namespaced.
    InvalidExtensionIdentifier {
        /// The rejected identifier.
        value: String,
    },

    /// A contract contains the same clause source-node identity more than once.
    DuplicateClauseNode {
        /// The duplicated AST node.
        node_id: NodeId,
    },

    /// Zero is reserved and cannot identify an obligation.
    ZeroObligationId,

    /// The obligation allocator exhausted its numeric identity space.
    ObligationIdExhausted,
}

impl fmt::Display for ContractModelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidExtensionIdentifier { value } => write!(
                f,
                "invalid namespaced contract-kind identifier: {value:?}"
            ),
            Self::DuplicateClauseNode { node_id } => write!(
                f,
                "contract contains duplicate clause node identity: {node_id:?}"
            ),
            Self::ZeroObligationId => {
                f.write_str("obligation identity zero is reserved")
            }
            Self::ObligationIdExhausted => {
                f.write_str("verification-obligation identity space exhausted")
            }
        }
    }
}

impl Error for ContractModelError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::ast::node::NodeId;

    fn node_ids() -> (NodeId, NodeId, NodeId) {
        let first = NodeId::first();
        let second = first.checked_next().expect("second node ID");
        let third = second.checked_next().expect("third node ID");
        (first, second, third)
    }

    fn span() -> Span {
        Span::default()
    }

    fn clause(
        source_node: NodeId,
        expression_node: NodeId,
        kind: ContractKind,
    ) -> ContractClause {
        ContractClause::new(
            source_node,
            span(),
            kind,
            ContractExpression::new(expression_node, span()),
            None,
        )
    }

    #[test]
    fn extension_kind_requires_a_namespaced_identifier() {
        assert!(ContractKindId::new("quantum.verification").is_ok());
        assert!(ContractKindId::new("domain.example-condition").is_ok());
        assert!(ContractKindId::new("single").is_err());
        assert!(ContractKindId::new("domain..condition").is_err());
        assert!(ContractKindId::new("domain.-condition").is_err());
    }

    #[test]
    fn expression_reference_preserves_canonical_node_identity() {
        let (source, expression, _) = node_ids();
        let reference = ContractExpression::new(expression, span());

        assert_eq!(reference.node_id(), expression);
        assert_ne!(reference.node_id(), source);
    }

    #[test]
    fn contract_rejects_duplicate_clause_node_identities() {
        let (contract_node, clause_node, expression_node) = node_ids();
        let first = clause(
            clause_node,
            expression_node,
            ContractKind::Precondition,
        );
        let second = clause(
            clause_node,
            expression_node,
            ContractKind::Postcondition,
        );

        let result = Contract::new(
            contract_node,
            span(),
            vec![first, second],
        );

        assert!(matches!(
            result,
            Err(ContractModelError::DuplicateClauseNode { .. })
        ));
    }

    #[test]
    fn contract_keeps_multiple_distinct_clauses() {
        let (contract_node, first_clause, expression) = node_ids();
        let second_clause = expression.checked_next().expect("next node");
        let second_expression = second_clause.checked_next().expect("next node");

        let contract = Contract::new(
            contract_node,
            span(),
            vec![
                clause(
                    first_clause,
                    expression,
                    ContractKind::Precondition,
                ),
                clause(
                    second_clause,
                    second_expression,
                    ContractKind::Postcondition,
                ),
            ],
        )
        .expect("valid contract structure");

        assert_eq!(contract.clause_count(), 2);
        assert_eq!(
            contract
                .clauses_of_kind(&ContractKind::Precondition)
                .count(),
            1
        );
    }

    #[test]
    fn new_obligation_starts_pending_not_proven() {
        let (contract_node, clause_node, expression_node) = node_ids();
        let clause = clause(
            clause_node,
            expression_node,
            ContractKind::Invariant,
        );
        let mut allocator = ObligationIdAllocator::new();
        let id = allocator.allocate().expect("first obligation ID");

        let obligation = ContractObligation::pending(
            id,
            contract_node,
            &clause,
            ObligationOrigin::Invariant,
        );

        assert!(!obligation.is_proven());
        assert!(matches!(
            obligation.outcome(),
            VerificationOutcome::Pending
        ));
    }

    #[test]
    fn only_explicit_proven_outcome_counts_as_proven() {
        let outcomes = [
            VerificationOutcome::Pending,
            VerificationOutcome::Disproven { evidence: None },
            VerificationOutcome::Unknown { reason: None },
            VerificationOutcome::Unsupported { reason: None },
            VerificationOutcome::Incomplete { reason: None },
        ];

        for outcome in outcomes {
            assert!(!outcome.is_proven());
        }

        assert!(
            VerificationOutcome::Proven { evidence: None }.is_proven()
        );
    }

    #[test]
    fn obligation_ids_are_checked_and_never_wrap() {
        assert!(ObligationId::new(0).is_err());
        assert_eq!(ObligationId::new(1).expect("non-zero ID").get(), 1);

        let mut allocator = ObligationIdAllocator::new();
        assert_eq!(allocator.allocate().expect("first ID").get(), 1);
        assert_eq!(allocator.allocate().expect("second ID").get(), 2);

        allocator.next = Some(u64::MAX);
        assert_eq!(
            allocator.allocate().expect("last ID").get(),
            u64::MAX
        );
        assert!(matches!(
            allocator.allocate(),
            Err(ContractModelError::ObligationIdExhausted)
        ));
    }

    #[test]
    fn unsupported_verification_is_not_success() {
        let outcome = VerificationOutcome::Unsupported {
            reason: Some("verification theory unavailable".to_owned()),
        };

        assert!(!outcome.is_proven());
        assert!(outcome.requires_attention());
        assert!(outcome.is_decided());
    }

    #[test]
    fn incomplete_verification_is_not_success() {
        let outcome = VerificationOutcome::Incomplete {
            reason: Some("analysis budget exhausted".to_owned()),
        };

        assert!(!outcome.is_proven());
        assert!(outcome.requires_attention());
    }
}
