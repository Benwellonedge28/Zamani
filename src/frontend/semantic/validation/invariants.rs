
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani Semantic Invariants
//!
//! Path: `src/frontend/semantic/validation/invariants.rs`
//!
//! PURPOSE
//!
//! Validate cross-object invariants of the canonical semantic contract
//! model. This module checks the internal consistency of contracts and
//! verification obligations without claiming that a condition is true.
//!
//! AUTHORITY
//!
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/grammar.md`
//! - `grammar/Zamani.g4`
//! - `src/frontend/ast/node/`
//! - `src/frontend/semantic/contracts/model.rs`
//!
//! OWNERSHIP
//!
//! This module owns:
//!
//! - Cross-contract identity consistency.
//! - Contract-clause and obligation reference consistency.
//! - Duplicate obligation identity detection.
//! - Contract-kind and obligation-origin consistency.
//! - Explicit reporting of obligations that remain unproven.
//! - Deterministic, structured invariant violations.
//!
//! This module does not own:
//!
//! - Parsing or AST construction.
//! - AST child enumeration, traversal, or structural validation.
//! - Name resolution or type inference.
//! - Contract expression evaluation or theorem proving.
//! - Verification evidence authentication.
//! - Effect, capability, or resource checking.
//! - Hardware feasibility, placement, scheduling, routing, or lowering.
//! - Classical, quantum, HDL, or other domain IRs.
//!
//! INTEGRATION CONTRACT
//!
//! Input types are owned by `contracts/model.rs`. AST identity and span
//! types remain owned by the canonical AST. The semantic pipeline calls
//! this validator after contract extraction and obligation generation,
//! and before accepting the semantic result as complete.
//!
//! `src/frontend/ast/node/validation/invariants.rs` remains the owner of
//! AST structural invariants. The two modules must not be conflated.
//!
//! SCALABILITY
//!
//! No hardware capacities or language-level collection ceilings are
//! introduced. Collections grow according to available memory.
//!
//! The validator is iterative. Its work is proportional to the number
//! of supplied contracts, clauses, and obligations, apart from ordered
//! map/set operations. Allocation failure is not converted into a
//! successful validation result.
//!
//! A finite process cannot guarantee infinite computation. The goal is
//! to avoid arbitrary language-level limits and preserve correctness
//! for every input the host can process.
//!
//! DETERMINISM
//!
//! Violations are emitted in input traversal order. Ordered collections
//! are used for identity lookup, so hash iteration cannot reorder results.
//!
//! SOUNDNESS
//!
//! Structural consistency is not proof. An obligation is considered
//! proven only when its recorded outcome is explicitly `Proven`.
//! Pending, unknown, unsupported, incomplete, and disproven outcomes
//! remain unproven.
//!
//! FREEZE CRITERIA
//!
//! - Public types use the canonical contract model.
//! - Invalid references and duplicate IDs are reported deterministically.
//! - No failed invariant is silently accepted.
//! - Verification status is never inferred from absence of diagnostics.
//! - The module remains independent of domain IRs and hardware.
//! - Unit and integration tests pass with the repository's actual module
//!   declarations and supported Rust toolchain.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::frontend::semantic::contracts::model::{
    Contract, ContractKind, ContractObligation, ObligationId, ObligationOrigin,
    VerificationOutcome,
};
use crate::frontend::ast::node::NodeId;

/// Stable architectural name of this validator.
pub const SEMANTIC_INVARIANTS_NAME: &str =
    "zamani.frontend.semantic.validation.invariants";

/// Version of this validator's report schema.
///
/// This version is independent of the Zamani language version and the
/// contract-model schema version.
pub const SEMANTIC_INVARIANTS_SCHEMA_VERSION: u16 = 1;

/// Stable classification of a semantic invariant violation.
///
/// These codes describe defects in the semantic model, not source-language
/// diagnostic wording. Consumers should match the code instead of parsing
/// the human-readable message.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum InvariantViolationCode {
    /// Two supplied contracts claim the same source node.
    DuplicateContractNode,

    /// Two clauses within the supplied contract set claim the same source
    /// node, even if they belong to different contracts.
    DuplicateClauseNode,

    /// An obligation references a contract not present in the supplied set.
    UnknownContractReference,

    /// An obligation references a clause not belonging to its contract.
    UnknownClauseReference,

    /// An obligation's condition node differs from its source clause.
    ConditionReferenceMismatch,

    /// More than one supplied obligation uses the same obligation ID.
    DuplicateObligationId,

    /// The obligation origin is incompatible with the clause kind.
    OriginKindMismatch,
}

impl InvariantViolationCode {
    /// Return the stable machine-readable code.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DuplicateContractNode => "SEM-INV-001",
            Self::DuplicateClauseNode => "SEM-INV-002",
            Self::UnknownContractReference => "SEM-INV-003",
            Self::UnknownClauseReference => "SEM-INV-004",
            Self::ConditionReferenceMismatch => "SEM-INV-005",
            Self::DuplicateObligationId => "SEM-INV-006",
            Self::OriginKindMismatch => "SEM-INV-007",
        }
    }
}

impl fmt::Display for InvariantViolationCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// One machine-readable semantic invariant violation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InvariantViolation {
    code: InvariantViolationCode,
    node: Option<NodeId>,
    related_node: Option<NodeId>,
    obligation_id: Option<ObligationId>,
    message: String,
}

impl InvariantViolation {
    fn new(
        code: InvariantViolationCode,
        node: Option<NodeId>,
        related_node: Option<NodeId>,
        obligation_id: Option<ObligationId>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            code,
            node,
            related_node,
            obligation_id,
            message: message.into(),
        }
    }

    /// Return the stable violation code.
    #[must_use]
    pub const fn code(&self) -> InvariantViolationCode {
        self.code
    }

    /// Return the primary AST node, if applicable.
    #[must_use]
    pub const fn node(&self) -> Option<NodeId> {
        self.node
    }

    /// Return a related AST node, if applicable.
    #[must_use]
    pub const fn related_node(&self) -> Option<NodeId> {
        self.related_node
    }

    /// Return the affected obligation ID, if applicable.
    #[must_use]
    pub const fn obligation_id(&self) -> Option<ObligationId> {
        self.obligation_id
    }

    /// Return a descriptive message.
    ///
    /// The message is for humans and is not a stable API identifier.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for InvariantViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.code, self.message)?;

        if let Some(node) = self.node {
            write!(f, " [node: {node:?}]")?;
        }

        if let Some(related) = self.related_node {
            write!(f, " [related node: {related:?}]")?;
        }

        if let Some(id) = self.obligation_id {
            write!(f, " [obligation: {}]", id.get())?;
        }

        Ok(())
    }
}

/// Summary of semantic invariant validation.
///
/// `violations` concerns structural consistency. `unproven_obligations`
/// counts supplied obligations whose outcomes are not explicitly proven.
/// These are separate measures: a consistent model may contain obligations
/// that still require verification.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct InvariantReport {
    violations: Vec<InvariantViolation>,
    obligations_checked: usize,
    proven_obligations: usize,
    unproven_obligations: usize,
}

impl InvariantReport {
    /// Create an empty report.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            violations: Vec::new(),
            obligations_checked: 0,
            proven_obligations: 0,
            unproven_obligations: 0,
        }
    }

    /// Return all violations in deterministic encounter order.
    #[must_use]
    pub fn violations(&self) -> &[InvariantViolation] {
        &self.violations
    }

    /// Return whether structural invariant validation succeeded.
    ///
    /// This does not mean that all verification obligations are proven.
    #[must_use]
    pub fn is_consistent(&self) -> bool {
        self.violations.is_empty()
    }

    /// Return whether any structural invariant failed.
    #[must_use]
    pub fn has_violations(&self) -> bool {
        !self.violations.is_empty()
    }

    /// Return the number of structural violations.
    #[must_use]
    pub fn violation_count(&self) -> usize {
        self.violations.len()
    }

    /// Return the number of supplied obligations examined.
    #[must_use]
    pub const fn obligations_checked(&self) -> usize {
        self.obligations_checked
    }

    /// Return the number of obligations explicitly recorded as proven.
    #[must_use]
    pub const fn proven_obligations(&self) -> usize {
        self.proven_obligations
    }

    /// Return the number of obligations not explicitly proven.
    #[must_use]
    pub const fn unproven_obligations(&self) -> usize {
        self.unproven_obligations
    }

    /// Return whether every supplied obligation is explicitly proven.
    ///
    /// This is a status summary, not independent proof verification.
    #[must_use]
    pub const fn all_obligations_proven(&self) -> bool {
        self.unproven_obligations == 0
    }

    fn push(&mut self, violation: InvariantViolation) {
        self.violations.push(violation);
    }
}

/// Validate cross-object invariants for a set of semantic contracts and
/// their verification obligations.
///
/// This function does not mutate its inputs and does not invoke a solver.
/// It checks:
///
/// - contract and clause source-node uniqueness across the supplied set;
/// - that every obligation references a supplied contract;
/// - that every obligation references a clause of that contract;
/// - that the obligation references the same condition node as the clause;
/// - that obligation IDs are unique within this validation batch;
/// - that obligation origin is compatible with contract kind.
///
/// Empty contracts are not rejected here: whether they are legal is a
/// language-level rule owned by the normative semantic specification.
///
/// # Identity scope
///
/// Duplicate checks apply only to the supplied batch. Callers must pass the
/// complete identity domain they intend to validate. Separate batches cannot
/// detect collisions between each other.
///
/// # Complexity
///
/// Expected time is `O((C + L + O) log (C + L + O))` for `C` contracts,
/// `L` clauses, and `O` obligations, using ordered identity indexes.
/// Memory use is `O(C + L + O)`, excluding the supplied objects.
///
/// This is not an infinite-resource guarantee. The function introduces no
/// domain-specific or hardware-capacity limit.
pub fn validate_semantic_invariants(
    contracts: &[Contract],
    obligations: &[ContractObligation],
) -> InvariantReport {
    let mut report = InvariantReport::new();

    // Each clause record is keyed by its owning contract and source identity.
    // Keep the source condition node and clause kind for later consistency
    // checks. The canonical contract model owns the actual clause values.
    let mut contract_nodes = BTreeSet::new();
    let mut clause_nodes = BTreeSet::new();
    let mut clauses_by_owner: BTreeMap<
        (NodeId, NodeId),
        (NodeId, ContractKind),
    > = BTreeMap::new();

    for contract in contracts {
        let contract_node = contract.source_node();

        if !contract_nodes.insert(contract_node) {
            report.push(InvariantViolation::new(
                InvariantViolationCode::DuplicateContractNode,
                Some(contract_node),
                None,
                None,
                "multiple contracts use the same source-node identity",
            ));
        }

        for clause in contract.clauses() {
            let clause_node = clause.source_node();

            if !clause_nodes.insert(clause_node) {
                report.push(InvariantViolation::new(
                    InvariantViolationCode::DuplicateClauseNode,
                    Some(clause_node),
                    Some(contract_node),
                    None,
                    "multiple clauses use the same source-node identity",
                ));
            }

            // Preserve the first occurrence deterministically. A duplicate
            // is already reported above; replacing it would make subsequent
            // diagnostics depend on which duplicate was processed last.
            clauses_by_owner
                .entry((contract_node, clause_node))
                .or_insert_with(|| {
                    (clause.condition().node_id(), clause.kind().clone())
                });
        }
    }

    let mut obligation_ids = BTreeSet::new();

    for obligation in obligations {
        report.obligations_checked =
            report.obligations_checked.saturating_add(1);

        if obligation.outcome().is_proven() {
            report.proven_obligations =
                report.proven_obligations.saturating_add(1);
        } else {
            report.unproven_obligations =
                report.unproven_obligations.saturating_add(1);
        }

        let obligation_id = obligation.id();
        let contract_node = obligation.contract_node();
        let clause_node = obligation.clause_node();

        if !obligation_ids.insert(obligation_id) {
            report.push(InvariantViolation::new(
                InvariantViolationCode::DuplicateObligationId,
                Some(clause_node),
                Some(contract_node),
                Some(obligation_id),
                "multiple obligations use the same obligation identity",
            ));
        }

        if !contract_nodes.contains(&contract_node) {
            report.push(InvariantViolation::new(
                InvariantViolationCode::UnknownContractReference,
                Some(contract_node),
                Some(clause_node),
                Some(obligation_id),
                "obligation references a contract absent from the supplied set",
            ));

            // Without a known owning contract, clause membership and
            // origin-kind consistency cannot be established.
            continue;
        }

        let Some((condition_node, kind)) =
            clauses_by_owner.get(&(contract_node, clause_node))
        else {
            report.push(InvariantViolation::new(
                InvariantViolationCode::UnknownClauseReference,
                Some(clause_node),
                Some(contract_node),
                Some(obligation_id),
                "obligation references a clause not owned by its contract",
            ));
            continue;
        };

        if *condition_node != obligation.condition().node_id() {
            report.push(InvariantViolation::new(
                InvariantViolationCode::ConditionReferenceMismatch,
                Some(obligation.condition().node_id()),
                Some(*condition_node),
                Some(obligation_id),
                "obligation condition does not match its source clause",
            ));
        }

        if !origin_matches_kind(obligation.origin(), kind) {
            report.push(InvariantViolation::new(
                InvariantViolationCode::OriginKindMismatch,
                Some(clause_node),
                Some(contract_node),
                Some(obligation_id),
                format!(
                    "obligation origin {:?} is incompatible with contract kind {:?}",
                    obligation.origin(),
                    kind,
                ),
            ));
        }
    }

    report
}

/// Determine whether an obligation origin is compatible with its clause kind.
///
/// This is a consistency check, not a claim about the truth of the clause.
/// Extension-defined contract kinds must use the extension origin because
/// this core validator cannot assume the extension's semantics.
fn origin_matches_kind(origin: ObligationOrigin, kind: &ContractKind) -> bool {
    match kind {
        ContractKind::Precondition => origin == ObligationOrigin::Precondition,
        ContractKind::Postcondition => origin == ObligationOrigin::Postcondition,
        ContractKind::Invariant | ContractKind::LoopInvariant => {
            origin == ObligationOrigin::Invariant
        }
        ContractKind::Assertion => origin == ObligationOrigin::Assertion,
        ContractKind::Extension(_) => origin == ObligationOrigin::Extension,
        ContractKind::FailureCondition => {
            origin == ObligationOrigin::SemanticRule
        }
        // ContractKind is non-exhaustive. A future core kind must not be
        // accepted by accident until its origin semantics are specified.
        _ => false,
    }
}

/// Return whether an outcome explicitly represents a proof.
///
/// This helper intentionally does not infer proof from absence of a
/// counterexample, completion of analysis, or a successful structural check.
#[must_use]
pub fn outcome_is_proven(outcome: &VerificationOutcome) -> bool {
    matches!(outcome, VerificationOutcome::Proven { .. })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn violation_codes_are_stable_and_distinct() {
        let codes = [
            InvariantViolationCode::DuplicateContractNode,
            InvariantViolationCode::DuplicateClauseNode,
            InvariantViolationCode::UnknownContractReference,
            InvariantViolationCode::UnknownClauseReference,
            InvariantViolationCode::ConditionReferenceMismatch,
            InvariantViolationCode::DuplicateObligationId,
            InvariantViolationCode::OriginKindMismatch,
        ];

        let unique: BTreeSet<_> = codes.iter().map(|code| code.as_str()).collect();

        assert_eq!(unique.len(), codes.len());
        assert!(codes.iter().all(|code| code.as_str().starts_with("SEM-INV-")));
    }

    #[test]
    fn empty_report_is_consistent_and_has_no_unproven_obligations() {
        let report = InvariantReport::new();

        assert!(report.is_consistent());
        assert!(!report.has_violations());
        assert_eq!(report.violation_count(), 0);
        assert_eq!(report.obligations_checked(), 0);
        assert_eq!(report.proven_obligations(), 0);
        assert_eq!(report.unproven_obligations(), 0);
        assert!(report.all_obligations_proven());
    }

    #[test]
    fn only_explicit_proven_outcomes_count_as_proven() {
        let proven = VerificationOutcome::Proven { evidence: None };
        let pending = VerificationOutcome::Pending;
        let unknown = VerificationOutcome::Unknown { reason: None };
        let unsupported = VerificationOutcome::Unsupported { reason: None };
        let incomplete = VerificationOutcome::Incomplete { reason: None };
        let disproven = VerificationOutcome::Disproven { evidence: None };

        assert!(outcome_is_proven(&proven));
        assert!(!outcome_is_proven(&pending));
        assert!(!outcome_is_proven(&unknown));
        assert!(!outcome_is_proven(&unsupported));
        assert!(!outcome_is_proven(&incomplete));
        assert!(!outcome_is_proven(&disproven));
    }

    #[test]
    fn a_consistent_report_is_not_itself_a_proof() {
        let mut report = InvariantReport::new();
        report.obligations_checked = 1;
        report.unproven_obligations = 1;

        assert!(report.is_consistent());
        assert!(!report.all_obligations_proven());
        assert_eq!(report.unproven_obligations(), 1);
    }
}
