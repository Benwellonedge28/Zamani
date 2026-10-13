
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani Semantic Invariants
//!
//! Path: `src/frontend/semantic/contracts/invariants.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Select invariant clauses from the canonical semantic contract model and
//! construct pending verification obligations for them.
//!
//! This module supports ordinary invariants and loop invariants. It preserves
//! their canonical AST identities, source spans, source order, and obligation
//! identity. It does not prove that an invariant holds.
//!
//! ## AUTHORITY
//!
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/grammar-authority.md`
//! - `grammar/grammar.md`
//! - `grammar/Zamani.g4`
//! - `grammar/statements/contract.g4`
//! - `grammar/functions/contracts.g4`
//! - `grammar/validation/invariants.g4`
//! - `grammar/contracts/ast-mappings.yaml`
//! - `src/frontend/ast/node/`
//! - `src/frontend/semantic/contracts/model.rs`
//!
//! Normative specifications define semantic meaning. The canonical grammar
//! defines source syntax. The AST defines source identity and locations.
//! `contracts/model.rs` owns contract and verification-obligation types.
//!
//! ## OWNS
//!
//! - Selecting ordinary and loop-invariant clauses.
//! - Creating pending invariant verification obligations.
//! - Preserving invariant kinds and source references.
//! - Reporting complete versus budget-limited collection.
//! - Transactional obligation-ID allocation.
//! - Invariant-specific result and error types.
//!
//! ## DOES_NOT_OWN
//!
//! - Parsing, grammar productions, or AST construction.
//! - AST traversal or AST graph-membership validation.
//! - Name resolution, expression typing, or effect inference.
//! - Evaluation of source expressions.
//! - Theorem proving, solver implementation, or proof authentication.
//! - Runtime invariant instrumentation or assertion execution.
//! - Policy, authorization, capability discovery, or resource planning.
//! - Target feasibility, hardware selection, or physical allocation.
//! - Diagnostic rendering or diagnostic-code ownership.
//! - Classical IR, `quantum::ir`, HDL IR, or backend lowering.
//!
//! ## INTEGRATION CONTRACT
//!
//! - `contracts/model.rs` is the sole owner of `Contract`, `ContractClause`,
//!   `ContractKind`, `ContractObligation`, and obligation identity types.
//! - The caller must supply a contract already extracted from the canonical
//!   AST. AST graph membership and node-kind checks belong to structural
//!   validation and contract extraction.
//! - `ObligationIdAllocator` is supplied by the semantic-analysis session.
//! - The diagnostic subsystem translates `InvariantError` into canonical
//!   diagnostics.
//! - The semantic pipeline decides when invariant obligations are collected
//!   and which verification component processes them.
//! - Verification components record outcomes using the canonical model.
//! - Collection never changes a verification outcome from `Pending`.
//!
//! ## SCALABILITY
//!
//! No hardware capacities, domain counts, invariant-count ceilings, or
//! language-level program-size limits are imposed.
//!
//! The default policy imposes no additional invariant-count budget. A caller
//! may supply an explicit operational budget. Budget exhaustion produces an
//! explicitly incomplete result, never a successful proof.
//!
//! A finite process cannot perform infinite work. POCO-REAF means this module
//! introduces no arbitrary machine-size restriction into invariant meaning.
//!
//! ## DETERMINISM
//!
//! Reports preserve the order of clauses in the supplied contract. Selection
//! does not depend on hardware, target availability, time, randomness,
//! environment variables, filesystem state, or network state.
//!
//! ## ERROR HANDLING
//!
//! Obligation-ID exhaustion returns an error. Allocation is transactional:
//! the caller's allocator is updated only if collection succeeds.
//! Operational-budget exhaustion is a successful, explicitly incomplete
//! analysis result rather than an allocation error.
//!
//! ## FREEZE CRITERIA
//!
//! 1. Imports match the canonical contract-model API.
//! 2. Ordinary and loop invariants are selected without reclassifying other
//!    contract kinds.
//! 3. Every emitted obligation retains the canonical contract, clause, and
//!    expression references.
//! 4. All emitted obligations remain pending.
//! 5. Allocation failure leaves the supplied allocator unchanged.
//! 6. Budget exhaustion identifies the first unprocessed invariant.
//! 7. Tests pass on the supported Rust toolchain.
//! 8. `contracts/mod.rs` exports this module.
//! 9. The semantic pipeline propagates incomplete results and errors.
//! 10. Workspace compilation and semantic integration tests pass.

use std::error::Error;
use std::fmt;

use crate::frontend::ast::node::{NodeId, Span};

use super::model::{
    Contract, ContractClause, ContractKind, ContractModelError, ContractObligation,
    ObligationIdAllocator, ObligationOrigin,
};

/// Policy for collecting invariant verification obligations.
///
/// `max_invariants: None` means this module imposes no additional invariant
/// count budget. Other compiler safety budgets remain independently owned by
/// the semantic configuration and pipeline.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct InvariantOptions {
    max_invariants: Option<usize>,
}

impl InvariantOptions {
    /// Create options without an additional invariant-count budget.
    #[must_use]
    pub const fn unrestricted() -> Self {
        Self {
            max_invariants: None,
        }
    }

    /// Create options with an explicit maximum number of invariants to
    /// collect during this invocation.
    ///
    /// A budget of zero collects no invariants. If an invariant exists,
    /// the result is incomplete and identifies that invariant as the next
    /// unprocessed clause.
    #[must_use]
    pub const fn with_max_invariants(maximum: usize) -> Self {
        Self {
            max_invariants: Some(maximum),
        }
    }

    /// Return the configured invariant-count budget.
    ///
    /// `None` means this module imposes no additional count budget.
    #[must_use]
    pub const fn max_invariants(self) -> Option<usize> {
        self.max_invariants
    }
}

/// The canonical semantic kind of an invariant clause.
///
/// The original `ContractKind` is retained instead of flattening loop
/// invariants into ordinary invariants.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InvariantKind {
    kind: ContractKind,
}

impl InvariantKind {
    /// Construct a classification from a supported canonical contract kind.
    ///
    /// Returns `None` for contract kinds that are not invariants.
    #[must_use]
    pub fn from_contract_kind(kind: &ContractKind) -> Option<Self> {
        match kind {
            ContractKind::Invariant | ContractKind::LoopInvariant => {
                Some(Self { kind: kind.clone() })
            }
            _ => None,
        }
    }

    /// Return the original canonical contract kind.
    #[must_use]
    pub const fn as_contract_kind(&self) -> &ContractKind {
        &self.kind
    }

    /// Return whether this is a loop invariant.
    #[must_use]
    pub const fn is_loop_invariant(&self) -> bool {
        matches!(self.kind, ContractKind::LoopInvariant)
    }
}

/// One collected invariant and its pending verification obligation.
///
/// This record preserves source references from the canonical contract
/// model. It does not contain a copied expression tree or proof result.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InvariantReport {
    contract_node: NodeId,
    clause_node: NodeId,
    clause_span: Span,
    kind: InvariantKind,
    obligation: ContractObligation,
}

impl InvariantReport {
    fn from_clause(
        contract: &Contract,
        clause: &ContractClause,
        obligation: ContractObligation,
    ) -> Option<Self> {
        let kind = InvariantKind::from_contract_kind(clause.kind())?;

        Some(Self {
            contract_node: contract.source_node(),
            clause_node: clause.source_node(),
            clause_span: clause.source_span(),
            kind,
            obligation,
        })
    }

    /// Return the AST node owning the contract.
    #[must_use]
    pub const fn contract_node(&self) -> NodeId {
        self.contract_node
    }

    /// Return the AST node introducing the invariant clause.
    #[must_use]
    pub const fn clause_node(&self) -> NodeId {
        self.clause_node
    }

    /// Return the source span of the invariant clause.
    #[must_use]
    pub const fn clause_span(&self) -> Span {
        self.clause_span
    }

    /// Return the invariant's canonical semantic kind.
    #[must_use]
    pub const fn kind(&self) -> &InvariantKind {
        &self.kind
    }

    /// Return the canonical verification obligation.
    #[must_use]
    pub const fn obligation(&self) -> &ContractObligation {
        &self.obligation
    }

    /// Return whether the associated obligation has been proven by a
    /// verification component.
    ///
    /// Collection alone never establishes this state.
    #[must_use]
    pub const fn is_proven(&self) -> bool {
        self.obligation.is_proven()
    }
}

/// Completion state for invariant collection.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum InvariantCompletion {
    /// Every invariant in the contract was collected.
    Completed,

    /// Collection stopped before all invariants were collected.
    ///
    /// `next_clause` is the canonical AST identity of the first invariant
    /// that was not collected. This is not a statement about that
    /// invariant's validity or truth.
    Incomplete {
        /// Identity of the first unprocessed invariant clause.
        next_clause: NodeId,
    },
}

impl InvariantCompletion {
    /// Return whether collection completed for every invariant in the
    /// supplied contract.
    #[must_use]
    pub const fn is_complete(self) -> bool {
        matches!(self, Self::Completed)
    }
}

/// Complete result of collecting invariant obligations from one contract.
///
/// Reports preserve source/extraction order. When collection is incomplete,
/// the result identifies the first unprocessed invariant. Unprocessed
/// invariants are not implicitly accepted, rejected, or proven.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InvariantAnalysis {
    reports: Vec<InvariantReport>,
    completion: InvariantCompletion,
}

impl InvariantAnalysis {
    /// Construct an empty, complete result.
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            reports: Vec::new(),
            completion: InvariantCompletion::Completed,
        }
    }

    /// Return collected invariant reports in source/extraction order.
    #[must_use]
    pub fn reports(&self) -> &[InvariantReport] {
        &self.reports
    }

    /// Consume the result and return its reports.
    #[must_use]
    pub fn into_reports(self) -> Vec<InvariantReport> {
        self.reports
    }

    /// Return the collection completion state.
    #[must_use]
    pub const fn completion(&self) -> InvariantCompletion {
        self.completion
    }

    /// Return whether every invariant in the contract was collected.
    #[must_use]
    pub const fn is_complete(&self) -> bool {
        self.completion.is_complete()
    }

    /// Return the number of collected invariant obligations.
    #[must_use]
    pub fn len(&self) -> usize {
        self.reports.len()
    }

    /// Return whether no invariant obligations were collected.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.reports.is_empty()
    }

    /// Iterate over collected invariant reports.
    pub fn iter(&self) -> std::slice::Iter<'_, InvariantReport> {
        self.reports.iter()
    }
}

impl<'a> IntoIterator for &'a InvariantAnalysis {
    type Item = &'a InvariantReport;
    type IntoIter = std::slice::Iter<'a, InvariantReport>;

    fn into_iter(self) -> Self::IntoIter {
        self.reports.iter()
    }
}

/// Errors raised while collecting invariant obligations.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum InvariantError {
    /// The canonical obligation allocator could not issue another identity.
    ObligationAllocation(ContractModelError),
}

impl fmt::Display for InvariantError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ObligationAllocation(error) => {
                write!(f, "failed to allocate invariant obligation: {error}")
            }
        }
    }
}

impl Error for InvariantError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::ObligationAllocation(error) => Some(error),
        }
    }
}

impl From<ContractModelError> for InvariantError {
    fn from(error: ContractModelError) -> Self {
        Self::ObligationAllocation(error)
    }
}

/// Return whether a canonical contract kind represents an invariant.
///
/// Ordinary invariants and loop invariants are selected. Preconditions,
/// postconditions, assertions, failure conditions, and extension-defined
/// kinds are not silently reclassified as invariants.
#[must_use]
pub fn is_invariant_kind(kind: &ContractKind) -> bool {
    matches!(
        kind,
        ContractKind::Invariant | ContractKind::LoopInvariant
    )
}

/// Count invariant clauses without allocating obligation identities.
///
/// This is a structural count only. It does not inspect AST graph membership,
/// check predicate types or effects, or prove any condition.
#[must_use]
pub fn invariant_count(contract: &Contract) -> usize {
    contract
        .clauses()
        .iter()
        .filter(|clause| is_invariant_kind(clause.kind()))
        .count()
}

/// Collect pending verification obligations for invariants in a contract.
///
/// # Behavior
///
/// Only clauses whose kind is exactly `ContractKind::Invariant` or
/// `ContractKind::LoopInvariant` are selected. All other contract kinds are
/// ignored by this collector.
///
/// For every selected clause, this function:
///
/// 1. Allocates an identity from the caller-supplied allocator.
/// 2. Retains the contract and clause AST identities.
/// 3. Retains the canonical expression reference through the obligation.
/// 4. Retains the invariant's original semantic kind in the report.
/// 5. Uses `ObligationOrigin::Invariant`.
/// 6. Leaves verification pending.
///
/// # Transactionality
///
/// Collection uses a staged copy of the allocator. If allocation fails, the
/// original allocator is unchanged and no partial analysis is returned.
///
/// When an explicit count budget is reached, an incomplete analysis is
/// returned. IDs for collected obligations are committed because those
/// obligations are present in the result. The first unprocessed invariant
/// is identified in `InvariantCompletion::Incomplete`.
///
/// # Important
///
/// This function does not prove invariants, evaluate expressions, establish
/// that expressions are Boolean or pure, or validate that referenced nodes
/// belong to the intended AST graph. Those guarantees belong to the
/// appropriate semantic and structural analysis passes.
pub fn collect_invariants(
    contract: &Contract,
    allocator: &mut ObligationIdAllocator,
    options: InvariantOptions,
) -> Result<InvariantAnalysis, InvariantError> {
    let selected_count = invariant_count(contract);

    if selected_count == 0 {
        return Ok(InvariantAnalysis::empty());
    }

    let mut staged_allocator = allocator.clone();
    let mut reports = Vec::new();
    let mut collected = 0_usize;

    for clause in contract.clauses() {
        if !is_invariant_kind(clause.kind()) {
            continue;
        }

        if options
            .max_invariants()
            .is_some_and(|maximum| collected >= maximum)
        {
            // No obligation is created for this clause. The completion
            // state explicitly records that collection stopped here.
            *allocator = staged_allocator;

            return Ok(InvariantAnalysis {
                reports,
                completion: InvariantCompletion::Incomplete {
                    next_clause: clause.source_node(),
                },
            });
        }

        let id = staged_allocator.allocate()?;
        let obligation = ContractObligation::pending(
            id,
            contract.source_node(),
            clause,
            ObligationOrigin::Invariant,
        );

        // `is_invariant_kind` and `InvariantKind::from_contract_kind` use
        // the same selection contract. Keep the check fallible so future
        // changes cannot silently introduce a mismatched report.
        let report = InvariantReport::from_clause(contract, clause, obligation)
            .expect("selected invariant kind must produce an invariant report");

        reports.push(report);
        collected = collected
            .checked_add(1)
            .ok_or(ContractModelError::ObligationIdExhausted)?;
    }

    *allocator = staged_allocator;

    Ok(InvariantAnalysis {
        reports,
        completion: InvariantCompletion::Completed,
    })
}

/// Collect invariant obligations without imposing an additional invariant
/// count budget.
///
/// The caller remains responsible for compiler-wide operational budgets.
pub fn collect_all_invariants(
    contract: &Contract,
    allocator: &mut ObligationIdAllocator,
) -> Result<InvariantAnalysis, InvariantError> {
    collect_invariants(contract, allocator, InvariantOptions::unrestricted())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::ast::node::NodeId;

    fn node_ids(count: usize) -> Vec<NodeId> {
        let mut ids = Vec::with_capacity(count);

        if count == 0 {
            return ids;
        }

        let mut next = NodeId::first();
        ids.push(next);

        while ids.len() < count {
            next = next.checked_next().expect("test node identity");
            ids.push(next);
        }

        ids
    }

    fn clause(
        source_node: NodeId,
        expression_node: NodeId,
        kind: ContractKind,
    ) -> ContractClause {
        ContractClause::new(
            source_node,
            Span::default(),
            kind,
            super::super::model::ContractExpression::new(
                expression_node,
                Span::default(),
            ),
            None,
        )
    }

    fn sample_contract() -> Contract {
        let ids = node_ids(7);

        Contract::new(
            ids[0],
            Span::default(),
            vec![
                clause(ids[1], ids[2], ContractKind::Precondition),
                clause(ids[3], ids[4], ContractKind::Invariant),
                clause(ids[5], ids[6], ContractKind::LoopInvariant),
            ],
        )
        .expect("valid test contract")
    }

    #[test]
    fn selects_only_ordinary_and_loop_invariants() {
        let contract = sample_contract();

        assert_eq!(invariant_count(&contract), 2);
        assert!(is_invariant_kind(&ContractKind::Invariant));
        assert!(is_invariant_kind(&ContractKind::LoopInvariant));
        assert!(!is_invariant_kind(&ContractKind::Precondition));
        assert!(!is_invariant_kind(&ContractKind::Postcondition));
        assert!(!is_invariant_kind(&ContractKind::Assertion));
    }

    #[test]
    fn collection_preserves_order_and_canonical_identity() {
        let contract = sample_contract();
        let mut allocator = ObligationIdAllocator::new();

        let result = collect_all_invariants(&contract, &mut allocator)
            .expect("collection succeeds");

        assert!(result.is_complete());
        assert_eq!(result.len(), 2);
        assert_eq!(
            result.reports()[0].clause_node(),
            contract.clauses()[1].source_node()
        );
        assert_eq!(
            result.reports()[1].clause_node(),
            contract.clauses()[2].source_node()
        );
        assert!(!result.reports()[0].kind().is_loop_invariant());
        assert!(result.reports()[1].kind().is_loop_invariant());
    }

    #[test]
    fn collection_creates_pending_obligations_not_proofs() {
        let contract = sample_contract();
        let mut allocator = ObligationIdAllocator::new();

        let result = collect_all_invariants(&contract, &mut allocator)
            .expect("collection succeeds");

        for report in result.reports() {
            assert_eq!(
                report.obligation().outcome(),
                &super::super::model::VerificationOutcome::Pending
            );
            assert!(!report.is_proven());
            assert_eq!(
                report.obligation().origin(),
                ObligationOrigin::Invariant
            );
        }
    }

    #[test]
    fn explicit_budget_reports_first_unprocessed_invariant() {
        let contract = sample_contract();
        let mut allocator = ObligationIdAllocator::new();

        let result = collect_invariants(
            &contract,
            &mut allocator,
            InvariantOptions::with_max_invariants(1),
        )
        .expect("budget exhaustion is an incomplete result, not an error");

        assert_eq!(result.len(), 1);
        assert!(!result.is_complete());

        assert_eq!(
            result.completion(),
            InvariantCompletion::Incomplete {
                next_clause: contract.clauses()[2].source_node(),
            }
        );
    }

    #[test]
    fn zero_budget_does_not_create_obligations() {
        let contract = sample_contract();
        let mut allocator = ObligationIdAllocator::new();

        let result = collect_invariants(
            &contract,
            &mut allocator,
            InvariantOptions::with_max_invariants(0),
        )
        .expect("zero budget is supported");

        assert!(result.reports().is_empty());
        assert_eq!(
            result.completion(),
            InvariantCompletion::Incomplete {
                next_clause: contract.clauses()[1].source_node(),
            }
        );

        // No obligation ID was needed because nothing was collected.
        assert_eq!(
            allocator.allocate().expect("first ID remains available").get(),
            1
        );
    }

    #[test]
    fn no_invariants_is_a_complete_empty_result() {
        let ids = node_ids(3);

        let contract = Contract::new(
            ids[0],
            Span::default(),
            vec![clause(
                ids[1],
                ids[2],
                ContractKind::Precondition,
            )],
        )
        .expect("valid contract");

        let mut allocator = ObligationIdAllocator::new();
        let result = collect_all_invariants(&contract, &mut allocator)
            .expect("empty selection succeeds");

        assert!(result.is_complete());
        assert!(result.is_empty());

        assert_eq!(
            allocator.allocate().expect("first ID remains available").get(),
            1
        );
    }

    #[test]
    fn collection_is_transactional_when_identity_space_is_exhausted() {
        let contract = sample_contract();

        // The canonical allocator exposes only sequential allocation. Exhaust
        // it before the collection starts, then ensure a failed collection
        // does not mutate the supplied allocator.
        let mut allocator = ObligationIdAllocator::new();

        while allocator.allocate().is_ok() {}

        let before = allocator.clone();
        let result = collect_all_invariants(&contract, &mut allocator);

        assert!(result.is_err());
        assert_eq!(allocator, before);
    }

    #[test]
    fn explicit_budget_does_not_change_the_meaning_of_an_invariant() {
        let contract = sample_contract();
        let mut allocator = ObligationIdAllocator::new();

        let result = collect_invariants(
            &contract,
            &mut allocator,
            InvariantOptions::with_max_invariants(1),
        )
        .expect("collection returns explicit completion");

        assert!(!result.is_complete());
        assert_eq!(result.len(), 1);
        assert!(!result.reports()[0].is_proven());
    }
}
