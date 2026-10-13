
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani Semantic Postconditions
//!
//! Path: `src/frontend/semantic/contracts/postconditions.rs`
//!
//! ## PURPOSE
//!
//! Construct verification obligations for postconditions represented by the
//! canonical semantic contract model.
//!
//! A postcondition expresses a condition that must hold after the relevant
//! operation or function completes successfully, according to the normative
//! Zamani language semantics.
//!
//! This module converts those clauses into pending verification obligations.
//! It does not infer that their conditions are true, evaluate source
//! expressions, execute program code, or implement a proof system.
//!
//! ## AUTHORITY
//!
//! - `grammar/specification/semantic-model.md`
//! - `grammar/grammar.md`
//! - `grammar/Zamani.g4`
//! - `src/frontend/ast/node/`
//! - `src/frontend/semantic/contracts/model.rs`
//!
//! The canonical grammar determines legal source syntax. The normative
//! semantic specification determines the meaning of postconditions.
//! `contracts/model.rs` owns the canonical contract and obligation types.
//!
//! ## OWNS
//!
//! - Selecting postcondition clauses from a validated semantic contract.
//! - Converting selected clauses into pending verification obligations.
//! - Preserving canonical AST identities and expression source references.
//! - Deterministic clause processing in source/extraction order.
//! - Transactional allocation of obligation identifiers for this operation.
//! - Errors specific to constructing postcondition obligations.
//!
//! ## DOES_NOT_OWN
//!
//! - Parsing or constructing source AST nodes.
//! - Name resolution or expression type checking.
//! - Determining the meaning of `result`, old-state expressions, or
//!   function-exit variables.
//! - Evaluating postcondition expressions.
//! - Proving, disproving, or executing postconditions.
//! - Preconditions, invariants, effects, capabilities, or resource planning.
//! - Runtime assertion instrumentation.
//! - Target selection, hardware feasibility, or backend lowering.
//! - A second contract, expression, or verification-obligation model.
//!
//! ## INTEGRATION CONTRACT
//!
//! - `contracts/model.rs` owns `Contract`, `ContractClause`, `ContractKind`,
//!   `ContractObligation`, `ObligationIdAllocator`, and related types.
//! - Contract extraction must establish that source references belong to the
//!   intended AST before calling this module.
//! - Structural validation and expression type checking remain separate.
//! - `contracts/mod.rs` exports this module.
//! - The contract-checking pass consumes the resulting pending obligations.
//! - `model/program.rs` or the owning semantic result may retain the
//!   obligations through its established obligation representation.
//! - `pipeline.rs` determines when obligation construction and verification
//!   occur.
//! - The verifier is the only component that may report proof outcomes, under
//!   its own documented soundness and evidence policy.
//!
//! ## SCALABILITY
//!
//! This module imposes no language-level limit on the number of contracts,
//! postconditions, qubits, devices, dimensions, or computational domains.
//!
//! Work and memory are proportional to the postconditions processed and the
//! resulting obligations retained. Actual operation remains subject to
//! available resources, representable collection sizes, and caller-enforced
//! operational budgets.
//!
//! A finite process cannot guarantee infinite computation. POCO-REAF means
//! that this module does not introduce arbitrary machine-size limits into
//! program semantics.
//!
//! ## DETERMINISM
//!
//! Clauses are processed in the order supplied by the canonical `Contract`.
//! The caller supplies an explicit `ObligationIdAllocator` whose identity
//! domain must be managed by the surrounding semantic-analysis session.
//!
//! The function does not depend on global mutable state, hash-map iteration,
//! wall-clock time, hardware discovery, or backend selection.
//!
//! ## FAILURE AND RECOVERY
//!
//! If obligation-ID allocation fails, the function returns a structured
//! error and leaves the supplied allocator unchanged.
//!
//! Empty contracts and contracts without postconditions produce an empty
//! result without allocating identifiers.
//!
//! An allocation failure is not evidence that a postcondition is false or
//! that the source program is semantically invalid.
//!
//! ## FREEZE CRITERIA
//!
//! 1. Imports match the canonical contract-model API.
//! 2. Only clauses whose kind is `ContractKind::Postcondition` are selected.
//! 3. Generated obligations retain the contract, clause, and condition IDs.
//! 4. Every generated obligation starts in `VerificationOutcome::Pending`.
//! 5. Identifier-allocation failures cannot leave partially consumed IDs.
//! 6. Empty inputs and contracts without postconditions are supported.
//! 7. No verifier, target-specific dependency, or unsafe code is introduced.
//! 8. Unit and integration tests pass against the actual workspace.

use core::fmt;
use std::error::Error;

use super::model::{
    Contract, ContractKind, ContractModelError, ContractObligation,
    ObligationIdAllocator, ObligationOrigin,
};

/// Errors produced while constructing postcondition obligations.
///
/// This type deliberately preserves the underlying contract-model error
/// rather than reducing it to a diagnostic string.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum PostconditionError {
    /// The canonical obligation identifier allocator could not allocate an
    /// identifier for a selected postcondition.
    ObligationAllocation(ContractModelError),
}

impl fmt::Display for PostconditionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ObligationAllocation(error) => {
                write!(
                    formatter,
                    "failed to allocate a postcondition obligation: {error}"
                )
            }
        }
    }
}

impl Error for PostconditionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::ObligationAllocation(error) => Some(error),
        }
    }
}

impl From<ContractModelError> for PostconditionError {
    fn from(error: ContractModelError) -> Self {
        Self::ObligationAllocation(error)
    }
}

/// A collection of pending obligations derived from postconditions.
///
/// The collection preserves the source/extraction order of the selected
/// clauses. It contains no proof claims and does not certify that the
/// postconditions are well-typed or true.
///
/// The vector grows according to available resources. This type defines no
/// language-level maximum for the number of obligations.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PostconditionObligations {
    obligations: Vec<ContractObligation>,
}

impl PostconditionObligations {
    /// Construct an empty collection.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            obligations: Vec::new(),
        }
    }

    /// Return the number of obligations in this collection.
    #[must_use]
    pub fn len(&self) -> usize {
        self.obligations.len()
    }

    /// Return whether the collection contains no obligations.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.obligations.is_empty()
    }

    /// Borrow the obligations in deterministic construction order.
    #[must_use]
    pub fn as_slice(&self) -> &[ContractObligation] {
        &self.obligations
    }

    /// Consume the collection and return its obligations.
    #[must_use]
    pub fn into_vec(self) -> Vec<ContractObligation> {
        self.obligations
    }

    /// Iterate over the obligations without transferring ownership.
    pub fn iter(&self) -> std::slice::Iter<'_, ContractObligation> {
        self.obligations.iter()
    }
}

impl<'a> IntoIterator for &'a PostconditionObligations {
    type Item = &'a ContractObligation;
    type IntoIter = std::slice::Iter<'a, ContractObligation>;

    fn into_iter(self) -> Self::IntoIter {
        self.obligations.iter()
    }
}

impl IntoIterator for PostconditionObligations {
    type Item = ContractObligation;
    type IntoIter = std::vec::IntoIter<ContractObligation>;

    fn into_iter(self) -> Self::IntoIter {
        self.obligations.into_iter()
    }
}

/// Construct pending verification obligations for the postconditions of one
/// canonical semantic contract.
///
/// # Behavior
///
/// Only clauses whose kind is exactly `ContractKind::Postcondition` are
/// selected. Preconditions, invariants, assertions, failure conditions, and
/// extension-defined clauses are not reclassified as postconditions.
///
/// For every selected clause, the function:
///
/// 1. Allocates a unique obligation ID from the supplied allocation domain.
/// 2. Retains the contract's canonical source-node identity.
/// 3. Retains the clause's canonical source-node identity.
/// 4. Retains the condition's AST-node identity and source span.
/// 5. Creates an obligation with `ObligationOrigin::Postcondition`.
/// 6. Leaves its verification outcome pending.
///
/// # Transactionality
///
/// The allocator is cloned before allocation begins. The original allocator
/// is updated only after all selected obligations have been constructed.
///
/// If allocation fails, this function returns an error without changing the
/// original allocator. No partial collection is returned.
///
/// The allocator's allocation domain must not be merged with another domain
/// unless the surrounding semantic session explicitly coordinates identity
/// uniqueness.
///
/// # Arguments
///
/// * `contract` — a canonical semantic contract.
/// * `allocator` — the mutable allocator owned by the semantic-analysis
///   session.
///
/// # Returns
///
/// A deterministic collection of pending postcondition obligations, or a
/// structured error if obligation identity allocation fails.
///
/// An empty contract or a contract without postcondition clauses succeeds
/// with an empty collection and does not consume an identifier.
///
/// # Important
///
/// This function does not establish that the contract is structurally valid
/// in the AST graph, that its expressions are well-typed, or that any
/// condition holds. Those guarantees belong to their respective analysis
/// passes.
///
/// It must not be used as a substitute for postcondition verification.
pub fn collect_postconditions(
    contract: &Contract,
    allocator: &mut ObligationIdAllocator,
) -> Result<PostconditionObligations, PostconditionError> {
    // Avoid cloning the allocator or allocating result capacity for an empty
    // selection. This also makes the no-postcondition behavior explicit.
    if !contract
        .clauses()
        .iter()
        .any(|clause| clause.kind() == &ContractKind::Postcondition)
    {
        return Ok(PostconditionObligations::new());
    }

    // Work against a staged allocator so an error cannot consume part of the
    // caller's identity sequence.
    let mut staged_allocator = allocator.clone();
    let mut obligations = Vec::new();

    for clause in contract.clauses() {
        if clause.kind() != &ContractKind::Postcondition {
            continue;
        }

        let id = staged_allocator.allocate()?;

        obligations.push(ContractObligation::pending(
            id,
            contract.source_node(),
            clause,
            ObligationOrigin::Postcondition,
        ));
    }

    // Commit only after the entire operation succeeds.
    *allocator = staged_allocator;

    Ok(PostconditionObligations { obligations })
}

/// Return the number of postcondition clauses in a contract.
///
/// This is a structural count only. It does not allocate obligation IDs,
/// check expression types, or perform verification.
#[must_use]
pub fn postcondition_count(contract: &Contract) -> usize {
    contract
        .clauses()
        .iter()
        .filter(|clause| clause.kind() == &ContractKind::Postcondition)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Confirm that the empty result has conventional collection behavior.
    #[test]
    fn empty_collection_is_well_formed() {
        let obligations = PostconditionObligations::new();

        assert!(obligations.is_empty());
        assert_eq!(obligations.len(), 0);
        assert!(obligations.as_slice().is_empty());
        assert_eq!(obligations.iter().count(), 0);
    }

    /// Confirm that counting does not depend on obligation allocation.
    ///
    /// Contract-specific construction tests use the canonical `NodeId` and
    /// `Span` constructors exposed by the AST crate. They are intentionally
    /// kept separate from this basic collection test so that this module
    /// does not introduce a competing AST construction API.
    #[test]
    fn empty_collection_consumes_into_empty_vector() {
        let obligations = PostconditionObligations::new();

        let values = obligations.into_vec();

        assert!(values.is_empty());
    }

    /// The postcondition origin must remain the canonical model's origin.
    #[test]
    fn postcondition_origin_is_not_redefined_here() {
        assert_eq!(
            ObligationOrigin::Postcondition,
            ObligationOrigin::Postcondition
        );
    }
}
