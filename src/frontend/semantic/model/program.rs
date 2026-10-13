
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Semantic Model — Program
//!
//! Path: `src/frontend/semantic/model/program.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Define the program-level container for semantic facts derived from the
//! canonical Zamani frontend AST.
//!
//! This module is deliberately a container and lifecycle boundary. It does
//! not implement name resolution, type inference, effect checking, resource
//! analysis, contract checking, or domain-specific semantic rules.
//!
//! ## AUTHORITY
//!
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/grammar-authority.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/poco-reaf.md`
//! - `grammar/Zamani.g4`
//! - `src/frontend/ast/node/`
//!
//! The normative specifications govern semantic meaning. This file governs
//! only the organization and lifecycle of the program-level result.
//!
//! ## OWNS
//!
//! - The semantic program result container.
//! - The association between canonical AST node identities and semantic facts.
//! - Analysis-completion and semantic-validity states.
//! - Unresolved-obligation accounting.
//! - Deterministic iteration over node-associated facts.
//! - Checked model mutation and finalization invariants.
//!
//! ## DOES_NOT_OWN
//!
//! - AST nodes, AST storage, parsing, or source spans.
//! - Symbol tables or declaration resolution.
//! - Canonical semantic types, type inference, or unification.
//! - Effect, capability, resource, contract, or policy definitions.
//! - Domain-specific semantic rules.
//! - Diagnostic definitions or diagnostic rendering.
//! - Classical IR, quantum IR, HDL IR, or another intermediate representation.
//! - Target discovery, feasibility, allocation, placement, scheduling,
//!   routing, calibration, error correction, or runtime execution.
//!
//! ## INTEGRATION CONTRACT
//!
//! - The canonical AST root is
//!   `crate::frontend::ast::node::program::Program`.
//! - AST identity is supplied by the AST's `NodeId`; this module never
//!   manufactures or reinterprets AST identities.
//! - `facts.rs` may supply the concrete fact type used as `F`.
//! - The symbol, type, effect, capability, resource, contract, and domain
//!   modules own their respective fact types.
//! - `provenance.rs` may supply the concrete provenance type used as `P`.
//! - Obligation-producing modules may supply the concrete obligation type
//!   used as `O`.
//! - `api.rs` owns the public analysis result and diagnostics contract.
//! - `session.rs` owns work accounting, cancellation, and session lifecycle.
//! - `pipeline.rs` owns pass ordering and execution.
//! - `model/mod.rs` owns public model-module composition.
//!
//! The generic parameters allow the program container to be established
//! before every semantic subsystem has been implemented. They do not replace
//! the need for concrete, validated fact and obligation types in production.
//!
//! ## CORE INVARIANTS
//!
//! 1. A semantic result refers to the canonical AST root.
//! 2. AST node identity is not semantic entity identity.
//! 3. Facts are grouped by AST node identity and exposed in deterministic
//!    node order.
//! 4. Repeated facts for one node are permitted.
//! 5. Analysis completion and semantic validity are independent properties.
//! 6. An incomplete, aborted, or failed analysis cannot be finalized as valid.
//! 7. A valid result cannot contain unresolved obligations.
//! 8. This container never asserts that a target can execute the program.
//! 9. Symbolic requirements remain semantic facts; this module does not
//!    resolve them using the current machine's capacities.
//! 10. There are no language-level hardware-size ceilings.
//!
//! ## SCALABILITY
//!
//! Storage grows according to the semantic facts actually produced and the
//! resources available to the compiler process. No finite implementation can
//! guarantee processing literally infinite input.
//!
//! Configured work, memory, and diagnostic budgets belong to the configuration,
//! session, and owning subsystems. Budget exhaustion must be reflected by the
//! caller as incomplete analysis, not as a valid finalized program.
//!
//! ## DETERMINISM
//!
//! A `BTreeMap` provides stable key ordering for node-associated facts.
//! Fact order within each node is insertion order. Pass implementations must
//! themselves produce facts in deterministic order.
//!
//! This module does not use wall-clock time, randomness, environment state,
//! hardware discovery, or global mutable state.
//!
//! ## FREEZE CRITERIA
//!
//! This file is ready to freeze when:
//!
//! - the model module exports it;
//! - concrete fact, obligation, and provenance types are selected;
//! - the semantic pipeline populates it from the canonical AST;
//! - finalization occurs only after required checks complete;
//! - lifecycle and invariant tests pass;
//! - the workspace builds and its relevant tests pass on the supported
//!   Rust toolchain.

use core::fmt;
use std::collections::BTreeMap;
use std::error::Error;

use crate::frontend::ast::node::node_id::NodeId;
use crate::frontend::ast::node::program::Program as AstProgram;

/// Schema version of the in-memory semantic program contract.
///
/// This version is independent of the language edition, AST schema, compiler
/// release, extension versions, and serialized artifact format.
pub const SEMANTIC_PROGRAM_SCHEMA_VERSION: u16 = 1;

/// Overall completion state of semantic analysis.
///
/// This is not the same as semantic validity. A completed analysis may have
/// found errors; an incomplete analysis cannot certify validity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub enum AnalysisCompletion {
    /// Required analysis work has completed.
    #[default]
    Complete,

    /// Some required analysis work did not finish.
    Incomplete,

    /// Analysis was deliberately stopped.
    Aborted,

    /// An internal or integration failure prevented a reliable result.
    Failed,
}

impl AnalysisCompletion {
    /// Returns `true` only when all requested analysis work completed.
    #[must_use]
    pub const fn is_complete(self) -> bool {
        matches!(self, Self::Complete)
    }

    /// Returns `true` when the result is not a complete analysis.
    #[must_use]
    pub const fn is_incomplete(self) -> bool {
        !matches!(self, Self::Complete)
    }
}

impl fmt::Display for AnalysisCompletion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Complete => "complete",
            Self::Incomplete => "incomplete",
            Self::Aborted => "aborted",
            Self::Failed => "failed",
        })
    }
}

/// Semantic validity of the analyzed program.
///
/// Validity describes the semantic evidence currently available. It does not
/// describe target feasibility or guarantee successful execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub enum SemanticValidity {
    /// The available evidence does not establish validity or invalidity.
    ///
    /// This is the safe initial state and the state for partial analysis.
    #[default]
    Unknown,

    /// Analysis found one or more semantic errors.
    Invalid,

    /// Analysis completed without semantic errors or unresolved obligations.
    Valid,

    /// Some relevant facts or obligations remain unresolved.
    Unresolved,
}

impl SemanticValidity {
    /// Returns `true` only for a certified-valid semantic result.
    #[must_use]
    pub const fn is_valid(self) -> bool {
        matches!(self, Self::Valid)
    }
}

impl fmt::Display for SemanticValidity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Unknown => "unknown",
            Self::Invalid => "invalid",
            Self::Valid => "valid",
            Self::Unresolved => "unresolved",
        })
    }
}

/// A semantic fact associated with a canonical AST node.
///
/// The fact type is supplied by the owning semantic subsystem. For example,
/// a caller may use a resolved type fact, declaration binding, effect summary,
/// capability requirement, symbolic resource constraint, or domain-specific
/// semantic fact.
///
/// This wrapper deliberately does not define a competing semantic type system.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeFact<F> {
    node_id: NodeId,
    fact: F,
}

impl<F> NodeFact<F> {
    /// Create a fact association for an existing canonical AST node identity.
    ///
    /// This constructor cannot prove that the ID belongs to a particular AST
    /// graph. The AST storage/validation layer owns that check.
    #[must_use]
    pub const fn new(node_id: NodeId, fact: F) -> Self {
        Self { node_id, fact }
    }

    /// Return the canonical AST node identity associated with this fact.
    #[must_use]
    pub const fn node_id(&self) -> NodeId {
        self.node_id
    }

    /// Borrow the fact payload.
    #[must_use]
    pub const fn fact(&self) -> &F {
        &self.fact
    }

    /// Consume this association and return its fact payload.
    #[must_use]
    pub fn into_fact(self) -> F {
        self.fact
    }
}

/// A semantic obligation whose status can be tracked by the analysis.
///
/// `O` is owned by the semantic subsystem that defines the obligation.
/// This container tracks only whether the obligation has been discharged.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Obligation<O> {
    payload: O,
    discharged: bool,
}

impl<O> Obligation<O> {
    /// Create an obligation that has not yet been discharged.
    #[must_use]
    pub const fn pending(payload: O) -> Self {
        Self {
            payload,
            discharged: false,
        }
    }

    /// Return the obligation payload.
    #[must_use]
    pub const fn payload(&self) -> &O {
        &self.payload
    }

    /// Return whether the obligation has been discharged.
    #[must_use]
    pub const fn is_discharged(&self) -> bool {
        self.discharged
    }

    /// Mark the obligation as discharged after the owning checker has
    /// established the required evidence.
    ///
    /// This method records a decision; it does not itself prove the
    /// obligation's correctness.
    pub fn discharge(&mut self) {
        self.discharged = true;
    }

    /// Return the obligation to a pending state.
    ///
    /// This is useful when an incremental-analysis invalidation makes earlier
    /// evidence stale. The caller must invalidate dependent facts as well.
    pub fn reopen(&mut self) {
        self.discharged = false;
    }

    /// Consume the wrapper and return the obligation payload.
    #[must_use]
    pub fn into_payload(self) -> O {
        self.payload
    }
}

/// Errors returned by checked semantic-model operations.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum SemanticProgramError {
    /// The model cannot be marked valid before analysis is complete.
    IncompleteCannotBeValid {
        /// Current completion state.
        completion: AnalysisCompletion,
    },

    /// The model cannot be marked valid while obligations remain pending.
    UnresolvedObligations {
        /// Number of obligations that remain pending.
        count: usize,
    },

    /// A final validity decision conflicts with an earlier terminal state.
    ///
    /// The model must not silently overwrite a previous finalization decision.
    AlreadyFinalized {
        /// Current validity.
        current: SemanticValidity,
        /// Requested validity.
        requested: SemanticValidity,
    },

    /// The model cannot be marked complete after an abort or internal failure.
    InvalidCompletionTransition {
        /// Current completion state.
        current: AnalysisCompletion,
        /// Requested completion state.
        requested: AnalysisCompletion,
    },
}

impl fmt::Display for SemanticProgramError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IncompleteCannotBeValid { completion } => write!(
                f,
                "semantic program cannot be valid while analysis is {completion}"
            ),
            Self::UnresolvedObligations { count } => write!(
                f,
                "semantic program cannot be valid: {count} obligation(s) remain unresolved"
            ),
            Self::AlreadyFinalized { current, requested } => write!(
                f,
                "semantic program validity is already {current}; cannot change it to {requested}"
            ),
            Self::InvalidCompletionTransition { current, requested } => write!(
                f,
                "invalid semantic completion transition from {current} to {requested}"
            ),
        }
    }
}

impl Error for SemanticProgramError {}

/// Final or partial semantic interpretation of one canonical AST program.
///
/// `F` is the concrete fact type selected by the semantic-model integration.
/// `O` is the obligation payload type selected by the contract/constraint
/// integration. `P` is the provenance payload type selected by the provenance
/// subsystem.
///
/// These parameters avoid forcing this aggregate to own every semantic
/// subsystem's data structures. Production callers should use one consistent
/// concrete set of types throughout a compiler build.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticProgram<F, O = (), P = ()> {
    schema_version: u16,
    ast_root: NodeId,

    // BTreeMap guarantees deterministic node-key iteration. Each node may
    // carry multiple facts, in the deterministic order in which passes add
    // them.
    facts: BTreeMap<NodeId, Vec<F>>,

    obligations: Vec<Obligation<O>>,

    // Provenance remains opaque to this container and is interpreted by the
    // dedicated provenance subsystem.
    provenance: BTreeMap<NodeId, Vec<P>>,

    completion: AnalysisCompletion,
    validity: SemanticValidity,
}

impl<F, O, P> SemanticProgram<F, O, P> {
    /// Create an empty semantic model for the supplied canonical AST root.
    ///
    /// The model starts with unknown validity. It is not considered valid
    /// merely because construction succeeded.
    #[must_use]
    pub fn new(ast: &AstProgram) -> Self {
        Self {
            schema_version: SEMANTIC_PROGRAM_SCHEMA_VERSION,
            ast_root: ast.id(),
            facts: BTreeMap::new(),
            obligations: Vec::new(),
            provenance: BTreeMap::new(),
            completion: AnalysisCompletion::Incomplete,
            validity: SemanticValidity::Unknown,
        }
    }

    /// Return the semantic-model schema version.
    #[must_use]
    pub const fn schema_version(&self) -> u16 {
        self.schema_version
    }

    /// Return the canonical AST root identity.
    #[must_use]
    pub const fn ast_root(&self) -> NodeId {
        self.ast_root
    }

    /// Return the analysis completion state.
    #[must_use]
    pub const fn completion(&self) -> AnalysisCompletion {
        self.completion
    }

    /// Return the semantic validity state.
    #[must_use]
    pub const fn validity(&self) -> SemanticValidity {
        self.validity
    }

    /// Return whether the model is both complete and semantically valid.
    ///
    /// Target feasibility is deliberately not part of this predicate.
    #[must_use]
    pub const fn is_valid(&self) -> bool {
        self.completion.is_complete() && self.validity.is_valid()
    }

    /// Return the number of AST nodes that have at least one associated fact.
    #[must_use]
    pub fn fact_node_count(&self) -> usize {
        self.facts.len()
    }

    /// Return the total number of facts currently stored.
    ///
    /// If the count cannot be represented by `usize`, the count saturates.
    /// This is a reporting method, not a semantic limit.
    #[must_use]
    pub fn fact_count(&self) -> usize {
        self.facts
            .values()
            .fold(0usize, |total, node_facts| {
                total.saturating_add(node_facts.len())
            })
    }

    /// Return all fact associations in deterministic node-key order.
    ///
    /// Fact order within each node is their insertion order.
    pub fn iter_facts(&self) -> impl Iterator<Item = NodeFact<&F>> + '_ {
        self.facts.iter().flat_map(|(node_id, facts)| {
            facts
                .iter()
                .map(move |fact| NodeFact::new(*node_id, fact))
        })
    }

    /// Return facts associated with one AST node.
    #[must_use]
    pub fn facts_for(&self, node_id: NodeId) -> Option<&[F]> {
        self.facts.get(&node_id).map(Vec::as_slice)
    }

    /// Add a semantic fact for a canonical AST node.
    ///
    /// The caller must ensure that `node_id` belongs to the AST graph
    /// represented by this model. That graph-membership check requires the AST
    /// storage contract and therefore does not belong in this container.
    ///
    /// The fact is appended, preserving deterministic pass insertion order.
    ///
    /// Mutation invalidates a prior validity decision because new facts may
    /// change the semantic interpretation.
    pub fn add_fact(
        &mut self,
        node_id: NodeId,
        fact: F,
    ) -> Result<(), SemanticProgramError> {
        self.ensure_not_finalized_for_mutation()?;

        self.facts
            .entry(node_id)
            .or_default()
            .push(fact);

        self.validity = SemanticValidity::Unknown;
        self.completion = AnalysisCompletion::Incomplete;
        Ok(())
    }

    /// Return the number of registered obligations.
    #[must_use]
    pub fn obligation_count(&self) -> usize {
        self.obligations.len()
    }

    /// Return the number of obligations not yet discharged.
    #[must_use]
    pub fn pending_obligation_count(&self) -> usize {
        self.obligations
            .iter()
            .filter(|obligation| !obligation.is_discharged())
            .count()
    }

    /// Iterate over obligations in insertion order.
    pub fn obligations(&self) -> impl ExactSizeIterator<Item = &Obligation<O>> {
        self.obligations.iter()
    }

    /// Add an obligation that must be checked by its owning subsystem.
    ///
    /// Adding an obligation invalidates any previous validity decision.
    pub fn add_obligation(
        &mut self,
        payload: O,
    ) -> Result<(), SemanticProgramError> {
        self.ensure_not_finalized_for_mutation()?;

        self.obligations.push(Obligation::pending(payload));
        self.validity = SemanticValidity::Unresolved;
        self.completion = AnalysisCompletion::Incomplete;
        Ok(())
    }

    /// Access an obligation by its stable insertion index.
    ///
    /// This index is local to this model and must not be persisted as a
    /// cross-compilation identity.
    pub fn obligation_mut(
        &mut self,
        index: usize,
    ) -> Option<&mut Obligation<O>> {
        self.validity = SemanticValidity::Unknown;
        self.completion = AnalysisCompletion::Incomplete;
        self.obligations.get_mut(index)
    }

    /// Add provenance associated with an AST node.
    ///
    /// Provenance records must be supplied by the provenance subsystem.
    /// This container does not infer or manufacture provenance evidence.
    pub fn add_provenance(
        &mut self,
        node_id: NodeId,
        provenance: P,
    ) -> Result<(), SemanticProgramError> {
        self.ensure_not_finalized_for_mutation()?;

        self.provenance
            .entry(node_id)
            .or_default()
            .push(provenance);

        self.validity = SemanticValidity::Unknown;
        self.completion = AnalysisCompletion::Incomplete;
        Ok(())
    }

    /// Iterate over provenance in deterministic node-key order.
    pub fn iter_provenance(
        &self,
    ) -> impl Iterator<Item = (NodeId, &P)> + '_ {
        self.provenance.iter().flat_map(|(node_id, records)| {
            records.iter().map(move |record| (*node_id, record))
        })
    }

    /// Return provenance associated with a node.
    #[must_use]
    pub fn provenance_for(&self, node_id: NodeId) -> Option<&[P]> {
        self.provenance.get(&node_id).map(Vec::as_slice)
    }

    /// Mark the analysis as complete.
    ///
    /// Completion does not imply validity. The final semantic checks must
    /// explicitly call `finalize_validity` with the appropriate decision.
    pub fn mark_complete(&mut self) -> Result<(), SemanticProgramError> {
        match self.completion {
            AnalysisCompletion::Aborted | AnalysisCompletion::Failed => {
                Err(SemanticProgramError::InvalidCompletionTransition {
                    current: self.completion,
                    requested: AnalysisCompletion::Complete,
                })
            }
            AnalysisCompletion::Complete | AnalysisCompletion::Incomplete => {
                self.completion = AnalysisCompletion::Complete;
                if self.validity == SemanticValidity::Valid
                    && self.pending_obligation_count() > 0
                {
                    self.validity = SemanticValidity::Unresolved;
                }
                Ok(())
            }
        }
    }

    /// Mark analysis incomplete.
    ///
    /// Any previously certified validity is withdrawn.
    pub fn mark_incomplete(&mut self) {
        self.completion = AnalysisCompletion::Incomplete;
        self.validity = SemanticValidity::Unknown;
    }

    /// Mark analysis as deliberately aborted.
    pub fn mark_aborted(&mut self) {
        self.completion = AnalysisCompletion::Aborted;
        self.validity = SemanticValidity::Unknown;
    }

    /// Mark analysis as failed because a reliable result could not be
    /// produced.
    pub fn mark_failed(&mut self) {
        self.completion = AnalysisCompletion::Failed;
        self.validity = SemanticValidity::Unknown;
    }

    /// Finalize semantic validity after the required analysis passes finish.
    ///
    /// The model refuses to become valid unless:
    ///
    /// - analysis completed;
    /// - the requested state is `Valid`;
    /// - no obligations remain pending.
    ///
    /// The caller remains responsible for ensuring that all required
    /// semantic checks ran and that diagnostics contain no semantic errors.
    pub fn finalize_validity(
        &mut self,
        requested: SemanticValidity,
    ) -> Result<(), SemanticProgramError> {
        if self.validity.is_valid() {
            return Err(SemanticProgramError::AlreadyFinalized {
                current: self.validity,
                requested,
            });
        }

        if requested == SemanticValidity::Valid {
            if !self.completion.is_complete() {
                return Err(
                    SemanticProgramError::IncompleteCannotBeValid {
                        completion: self.completion,
                    },
                );
            }

            let pending = self.pending_obligation_count();
            if pending > 0 {
                return Err(SemanticProgramError::UnresolvedObligations {
                    count: pending,
                });
            }
        }

        self.validity = requested;
        Ok(())
    }

    /// Borrow all node-associated facts.
    #[must_use]
    pub fn facts(&self) -> &BTreeMap<NodeId, Vec<F>> {
        &self.facts
    }

    /// Borrow all provenance records.
    #[must_use]
    pub fn provenance(&self) -> &BTreeMap<NodeId, Vec<P>> {
        &self.provenance
    }

    fn ensure_not_finalized_for_mutation(
        &self,
    ) -> Result<(), SemanticProgramError> {
        if self.validity.is_valid() {
            return Err(SemanticProgramError::AlreadyFinalized {
                current: self.validity,
                requested: SemanticValidity::Unknown,
            });
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::ast::node::node_id::NodeId;
    use crate::frontend::ast::node::program::Program as AstProgram;
    use crate::frontend::ast::node::source::Span;

    fn ast_program() -> AstProgram {
        AstProgram::without_metadata(NodeId::first(), Span::default())
    }

    #[test]
    fn new_model_preserves_ast_root_and_starts_unvalidated() {
        let ast = ast_program();
        let model: SemanticProgram<String> = SemanticProgram::new(&ast);

        assert_eq!(model.ast_root(), ast.id());
        assert_eq!(model.schema_version(), SEMANTIC_PROGRAM_SCHEMA_VERSION);
        assert_eq!(model.validity(), SemanticValidity::Unknown);
        assert!(!model.is_valid());
    }

    #[test]
    fn facts_are_grouped_and_iterated_deterministically() {
        let ast = ast_program();
        let mut model: SemanticProgram<String> = SemanticProgram::new(&ast);
        let first = NodeId::first();
        let second = first.checked_next().expect("next node identity");

        model.add_fact(second, "second-a".to_owned()).expect("fact");
        model.add_fact(first, "first-a".to_owned()).expect("fact");
        model.add_fact(second, "second-b".to_owned()).expect("fact");

        let observed: Vec<_> = model
            .iter_facts()
            .map(|fact| (fact.node_id(), fact.fact().as_str()))
            .collect();

        assert_eq!(
            observed,
            vec![
                (first, "first-a"),
                (second, "second-a"),
                (second, "second-b"),
            ]
        );
    }

    #[test]
    fn incomplete_analysis_cannot_be_marked_valid() {
        let ast = ast_program();
        let mut model: SemanticProgram<String> = SemanticProgram::new(&ast);

        let error = model
            .finalize_validity(SemanticValidity::Valid)
            .expect_err("incomplete analysis cannot be valid");

        assert!(matches!(
            error,
            SemanticProgramError::IncompleteCannotBeValid { .. }
        ));
        assert!(!model.is_valid());
    }

    #[test]
    fn pending_obligations_prevent_valid_finalization() {
        let ast = ast_program();
        let mut model: SemanticProgram<String, String> = SemanticProgram::new(&ast);

        model
            .add_obligation("prove contract".to_owned())
            .expect("obligation");
        model.mark_complete().expect("complete analysis");

        let error = model
            .finalize_validity(SemanticValidity::Valid)
            .expect_err("pending obligation prevents validity");

        assert!(matches!(
            error,
            SemanticProgramError::UnresolvedObligations { count: 1 }
        ));
        assert!(!model.is_valid());
    }

    #[test]
    fn completed_model_can_be_finalized_when_obligations_are_discharged() {
        let ast = ast_program();
        let mut model: SemanticProgram<String, String> = SemanticProgram::new(&ast);

        model
            .add_obligation("prove contract".to_owned())
            .expect("obligation");

        model
            .obligation_mut(0)
            .expect("obligation exists")
            .discharge();

        model.mark_complete().expect("complete analysis");
        model
            .finalize_validity(SemanticValidity::Valid)
            .expect("valid finalization");

        assert!(model.is_valid());
    }

    #[test]
    fn aborted_analysis_cannot_be_completed() {
        let ast = ast_program();
        let mut model: SemanticProgram<String> = SemanticProgram::new(&ast);

        model.mark_aborted();

        assert!(matches!(
            model.mark_complete(),
            Err(SemanticProgramError::InvalidCompletionTransition { .. })
        ));
        assert!(!model.is_valid());
    }

    #[test]
    fn adding_a_fact_withdraws_prior_completion() {
        let ast = ast_program();
        let mut model: SemanticProgram<String> = SemanticProgram::new(&ast);

        model.mark_complete().expect("complete");
        model.add_fact(ast.id(), "new fact".to_owned()).expect("fact");

        assert_eq!(model.completion(), AnalysisCompletion::Incomplete);
        assert_eq!(model.validity(), SemanticValidity::Unknown);
    }
}
