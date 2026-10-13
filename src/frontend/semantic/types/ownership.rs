
#![forbid(unsafe_code)]

//! Zamani native frontend — ownership, borrowing, and move analysis.
//!
//! File: src/frontend/semantic/types/ownership.rs
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//!
//! # Authority and integration
//!
//! The canonical source AST is:
//! `crate::frontend::ast::node`.
//!
//! Source type syntax is owned by:
//! `crate::frontend::ast::node::types::TypeExpr`.
//!
//! Ownership and borrowing syntax is governed by the repository's
//! `grammar/memory/ownership.g4`, `grammar/memory/borrowing.g4`,
//! `grammar/memory/references.g4`, and `grammar/types/reference.g4`.
//!
//! The normative semantic contract is governed by
//! `grammar/specification/semantic-model.md`.
//!
//! This module does not parse source, define another AST, infer source types,
//! solve all lifetime constraints, allocate memory, or choose a target.
//! An AST/CFG adapter translates canonical source constructs into the events
//! below. This module checks their ownership and borrowing consequences.
//!
//! # Ownership modes
//!
//! * `Unrestricted`: no linear/affine consumption obligation. A move still
//!   invalidates the moved-from place unless it is reinitialized.
//! * `Affine`: a value may be consumed at most once; it need not be consumed.
//! * `Linear`: a value must be consumed exactly once on every reachable exit.
//!
//! These rules are semantic policy. The adapter must obtain a declared mode
//! from the canonical AST or the language's normative defaults; it must not
//! infer one from a target or hardware type.
//!
//! # Dataflow contract
//!
//! Analysis runs over a control-flow graph. The adapter must supply all
//! semantically relevant reads, writes, moves, borrows, borrow uses, loan
//! endings, drops, and scope exits. A missing event is an adapter defect.
//!
//! The analysis is conservative at joins: if a value or loan is available on
//! only some incoming paths, it is not treated as definitely available.
//!
//! # Scalability and operational limits
//!
//! There are no hardware-size, device-count, qubit-count, register-width,
//! tensor-rank, or ownership-count ceilings. Graph and state sizes grow with
//! the supplied program. Optional budgets are compiler operational policies;
//! exhausting one returns `Incomplete`, never a false successful result.
//!
//! # Important boundary
//!
//! This is a core dataflow engine, not a complete AST visitor. It is ready to
//! integrate once `semantic::traversal` and `semantic::model` establish their
//! shared interfaces. Do not duplicate `NodeId`, `Span`, `TypeExpr`, or
//! diagnostics in those modules.

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use crate::frontend::ast::node::{NodeId, Span};

// -----------------------------------------------------------------------------
// Stable identities and source provenance
// -----------------------------------------------------------------------------

/// Identity of a source-level storage place within one semantic analysis.
///
/// The AST adapter assigns these IDs deterministically. They are not addresses,
/// memory offsets, register numbers, device IDs, or runtime handles.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PlaceId(pub u64);

/// Identity of a borrow/loan within one function or analysis region.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct LoanId(pub u64);

/// Identity of a basic block in the adapter-provided control-flow graph.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BlockId(pub u64);

/// Identity of a lexical or semantic lifetime scope.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ScopeId(pub u64);

/// Source provenance shared with the canonical AST.
///
/// `node` and `span` refer to existing AST/source-map identities. This module
/// does not synthesize source locations or use byte offsets as semantic IDs.
#[derive(Clone, Debug, Default)]
pub struct SourceSite {
    pub node: Option<NodeId>,
    pub span: Option<Span>,
}

// -----------------------------------------------------------------------------
// Places and alias overlap
// -----------------------------------------------------------------------------

/// A statically known index when the adapter can prove it, or a dynamic index
/// when it cannot. Dynamic indices conservatively overlap other indices.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum IndexProjection {
    Constant(String),
    Dynamic,
}

/// A place projection. Unknown aliasing must be represented conservatively.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Projection {
    Field(String),
    Index(IndexProjection),
    Dereference,
    /// Extension-defined projections are conservatively treated as possibly
    /// overlapping unless the adapter canonicalizes them to known fields or
    /// known constant indices.
    Extension { namespace: String, key: String },
}

/// A place is a root binding plus a sequence of projections.
///
/// Examples include a local variable, a field, an array element, or a
/// dereferenced source-level pointer. It never contains a physical address.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PlacePath {
    pub root: PlaceId,
    pub projections: Vec<Projection>,
}

impl PlacePath {
    #[must_use]
    pub const fn root(root: PlaceId) -> Self {
        Self {
            root,
            projections: Vec::new(),
        }
    }

    #[must_use]
    pub fn field(&self, name: impl Into<String>) -> Self {
        let mut result = self.clone();
        result.projections.push(Projection::Field(name.into()));
        result
    }

    #[must_use]
    pub fn index(&self, index: IndexProjection) -> Self {
        let mut result = self.clone();
        result.projections.push(Projection::Index(index));
        result
    }

    #[must_use]
    pub fn dereference(&self) -> Self {
        let mut result = self.clone();
        result.projections.push(Projection::Dereference);
        result
    }

    /// Returns true when either path may designate overlapping storage.
    ///
    /// Distinct named fields and distinct statically known indices are
    /// disjoint. Dynamic indices, dereferences, and extension projections are
    /// conservative. A parent overlaps every descendant.
    #[must_use]
    pub fn may_overlap(&self, other: &Self) -> bool {
        if self.root != other.root {
            return false;
        }

        for (left, right) in self.projections.iter().zip(&other.projections) {
            if left == right {
                continue;
            }

            match (left, right) {
                (Projection::Field(a), Projection::Field(b)) if a != b => {
                    return false;
                }
                (
                    Projection::Index(IndexProjection::Constant(a)),
                    Projection::Index(IndexProjection::Constant(b)),
                ) if a != b => return false,
                _ => return true,
            }
        }

        // One path is a prefix of the other, so one designates the parent
        // object or storage containing the other.
        true
    }

    fn is_prefix_of(&self, other: &Self) -> bool {
        self.root == other.root
            && self.projections.len() <= other.projections.len()
            && self
                .projections
                .iter()
                .zip(&other.projections)
                .all(|(left, right)| left == right)
    }
}

// -----------------------------------------------------------------------------
// Ownership modes and scopes
// -----------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OwnershipMode {
    Unrestricted,
    Affine,
    Linear,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BorrowKind {
    Shared,
    Mutable,
}

/// Parent relationships for semantic scopes.
///
/// A scope outlives itself and its descendants. The adapter must register each
/// scope exactly once and must not construct cyclic parent relationships.
#[derive(Clone, Debug, Default)]
pub struct ScopeTree {
    parents: BTreeMap<ScopeId, Option<ScopeId>>,
}

impl ScopeTree {
    pub fn insert(
        &mut self,
        scope: ScopeId,
        parent: Option<ScopeId>,
    ) -> Result<(), ScopeTreeError> {
        if self.parents.contains_key(&scope) {
            return Err(ScopeTreeError::DuplicateScope(scope));
        }
        if let Some(parent_id) = parent {
            if !self.parents.contains_key(&parent_id) {
                return Err(ScopeTreeError::UnknownParent {
                    scope,
                    parent: parent_id,
                });
            }
            if parent_id == scope {
                return Err(ScopeTreeError::Cycle(scope));
            }
        }
        self.parents.insert(scope, parent);
        Ok(())
    }

    #[must_use]
    pub fn contains(&self, scope: ScopeId) -> bool {
        self.parents.contains_key(&scope)
    }

    /// Whether `owner` outlives `borrower`.
    #[must_use]
    pub fn outlives(&self, owner: ScopeId, borrower: ScopeId) -> bool {
        let mut current = Some(borrower);
        let mut visited = BTreeSet::new();

        while let Some(scope) = current {
            if scope == owner {
                return true;
            }
            if !visited.insert(scope) {
                // Defensive cycle handling: malformed scope graphs are never
                // accepted as evidence that a borrow is safe.
                return false;
            }
            match self.parents.get(&scope) {
                Some(parent) => current = *parent,
                None => return false,
            }
        }
        false
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ScopeTreeError {
    DuplicateScope(ScopeId),
    UnknownParent { scope: ScopeId, parent: ScopeId },
    Cycle(ScopeId),
}

// -----------------------------------------------------------------------------
// Adapter-provided ownership events and control flow
// -----------------------------------------------------------------------------

/// Events emitted by the native AST/CFG adapter.
///
/// The adapter must preserve evaluation order and emit explicit loan-end events
/// according to the lifetime analysis contract. This event model is not a
/// second source AST.
#[derive(Clone, Debug)]
pub enum OwnershipEvent {
    Declare {
        place: PlacePath,
        mode: OwnershipMode,
        scope: ScopeId,
        initialized: bool,
        site: SourceSite,
    },
    Read {
        place: PlacePath,
        site: SourceSite,
    },
    Move {
        place: PlacePath,
        site: SourceSite,
    },
    Write {
        place: PlacePath,
        site: SourceSite,
    },
    Borrow {
        loan: LoanId,
        place: PlacePath,
        kind: BorrowKind,
        borrower_scope: ScopeId,
        site: SourceSite,
    },
    UseBorrow {
        loan: LoanId,
        site: SourceSite,
    },
    EndBorrow {
        loan: LoanId,
        site: SourceSite,
    },
    Mutate {
        place: PlacePath,
        site: SourceSite,
    },
    Drop {
        place: PlacePath,
        site: SourceSite,
    },
    ScopeExit {
        scope: ScopeId,
        site: SourceSite,
    },
}

#[derive(Clone, Debug)]
pub struct BasicBlock {
    pub id: BlockId,
    pub events: Vec<OwnershipEvent>,
    pub successors: Vec<BlockId>,
}

#[derive(Clone, Debug)]
pub struct ControlFlowGraph {
    pub entry: BlockId,
    pub blocks: BTreeMap<BlockId, BasicBlock>,
    pub scopes: ScopeTree,
}

impl ControlFlowGraph {
    /// Validate graph references and the entry block before dataflow begins.
    pub fn validate(&self) -> Result<(), GraphError> {
        if !self.blocks.contains_key(&self.entry) {
            return Err(GraphError::MissingEntry(self.entry));
        }

        for (key, block) in &self.blocks {
            if *key != block.id {
                return Err(GraphError::BlockKeyMismatch {
                    key: *key,
                    block: block.id,
                });
            }
            let mut unique = BTreeSet::new();
            for successor in &block.successors {
                if !self.blocks.contains_key(successor) {
                    return Err(GraphError::MissingSuccessor {
                        block: *key,
                        successor: *successor,
                    });
                }
                if !unique.insert(*successor) {
                    return Err(GraphError::DuplicateSuccessor {
                        block: *key,
                        successor: *successor,
                    });
                }
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GraphError {
    MissingEntry(BlockId),
    BlockKeyMismatch { key: BlockId, block: BlockId },
    MissingSuccessor { block: BlockId, successor: BlockId },
    DuplicateSuccessor { block: BlockId, successor: BlockId },
}

// -----------------------------------------------------------------------------
// Diagnostics and operational policy
// -----------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum OwnershipDiagnosticCode {
    InvalidControlFlowGraph,
    InvalidScope,
    ConflictingDeclaration,
    ReadBeforeInitialization,
    UseAfterMove,
    PossiblyUnavailable,
    ConflictingBorrow,
    AccessWhileMutablyBorrowed,
    MutationWhileBorrowed,
    MoveWhileBorrowed,
    UnknownLoan,
    LoanNotDefinitelyActive,
    BorrowOutlivesOwner,
    LinearValueNotConsumed,
    LinearValueDropped,
    AnalysisBudgetExceeded,
}

#[derive(Clone, Debug)]
pub struct OwnershipDiagnostic {
    pub code: OwnershipDiagnosticCode,
    pub block: Option<BlockId>,
    pub event_index: Option<usize>,
    pub place: Option<PlacePath>,
    pub loan: Option<LoanId>,
    pub site: SourceSite,
    pub message: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AnalysisStatus {
    Complete,
    Incomplete,
}

#[derive(Clone, Debug)]
pub struct OwnershipConfig {
    /// Maximum number of basic-block transfer steps. `None` means no explicit
    /// compiler budget; actual execution remains bounded by available resources.
    pub max_dataflow_steps: Option<usize>,
    /// Maximum emitted diagnostics. `None` means no explicit diagnostic cap.
    /// Hitting a cap marks the report incomplete rather than silently passing.
    pub max_diagnostics: Option<usize>,
}

impl Default for OwnershipConfig {
    fn default() -> Self {
        Self {
            max_dataflow_steps: None,
            max_diagnostics: None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct OwnershipReport {
    pub status: AnalysisStatus,
    pub diagnostics: Vec<OwnershipDiagnostic>,
    pub dataflow_steps: usize,
}

impl OwnershipReport {
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.status == AnalysisStatus::Complete
    }

    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.is_complete() && self.diagnostics.is_empty()
    }
}

// -----------------------------------------------------------------------------
// Internal dataflow lattice
// -----------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Availability {
    Uninitialized,
    Available,
    Moved,
    Dropped,
    MaybeUnavailable,
}

impl Availability {
    fn join(self, other: Self) -> Self {
        if self == other {
            self
        } else {
            Self::MaybeUnavailable
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum LoanPresence {
    Active,
    MaybeActive,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ActiveLoan {
    place: PlacePath,
    kind: BorrowKind,
    borrower_scope: ScopeId,
    presence: LoanPresence,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct FlowState {
    places: BTreeMap<PlacePath, Availability>,
    loans: BTreeMap<LoanId, ActiveLoan>,
}

impl FlowState {
    fn join(&self, other: &Self) -> Self {
        let mut result = Self::default();
        let keys: BTreeSet<_> = self
            .places
            .keys()
            .chain(other.places.keys())
            .cloned()
            .collect();

        for key in keys {
            let left = self
                .places
                .get(&key)
                .copied()
                .unwrap_or(Availability::Uninitialized);
            let right = other
                .places
                .get(&key)
                .copied()
                .unwrap_or(Availability::Uninitialized);
            result.places.insert(key, left.join(right));
        }

        let loan_ids: BTreeSet<_> = self
            .loans
            .keys()
            .chain(other.loans.keys())
            .copied()
            .collect();

        for loan_id in loan_ids {
            match (self.loans.get(&loan_id), other.loans.get(&loan_id)) {
                (Some(left), Some(right)) if left == right => {
                    result.loans.insert(loan_id, left.clone());
                }
                (Some(left), Some(right))
                    if left.place == right.place
                        && left.kind == right.kind
                        && left.borrower_scope == right.borrower_scope =>
                {
                    let mut loan = left.clone();
                    loan.presence = LoanPresence::MaybeActive;
                    result.loans.insert(loan_id, loan);
                }
                (Some(left), None) | (None, Some(left)) => {
                    let mut loan = left.clone();
                    loan.presence = LoanPresence::MaybeActive;
                    result.loans.insert(loan_id, loan);
                }
                (Some(left), Some(_)) => {
                    // A reused loan ID with different meanings is not
                    // considered definitely active.
                    let mut loan = left.clone();
                    loan.presence = LoanPresence::MaybeActive;
                    result.loans.insert(loan_id, loan);
                }
                (None, None) => {}
            }
        }
        result
    }
}

#[derive(Clone, Debug)]
struct PlaceInfo {
    mode: OwnershipMode,
    scope: ScopeId,
}

// -----------------------------------------------------------------------------
// Public analysis entry point
// -----------------------------------------------------------------------------

/// Analyze ownership and borrowing over a canonical CFG.
///
/// The result is valid only when `status == Complete` and diagnostics are
/// empty. A caller must not treat an incomplete result as successful analysis.
pub fn analyze_ownership(
    cfg: &ControlFlowGraph,
    config: &OwnershipConfig,
) -> OwnershipReport {
    let mut diagnostics = Vec::new();

    if let Err(error) = cfg.validate() {
        diagnostics.push(OwnershipDiagnostic {
            code: OwnershipDiagnosticCode::InvalidControlFlowGraph,
            block: None,
            event_index: None,
            place: None,
            loan: None,
            site: SourceSite::default(),
            message: format!("invalid ownership control-flow graph: {error:?}"),
        });
        return OwnershipReport {
            status: AnalysisStatus::Incomplete,
            diagnostics,
            dataflow_steps: 0,
        };
    }

    let (place_info, declaration_diagnostics) = collect_declarations(cfg);
    diagnostics.extend(declaration_diagnostics);

    let mut incoming: BTreeMap<BlockId, FlowState> = BTreeMap::new();
    let mut worklist = VecDeque::new();
    incoming.insert(cfg.entry, FlowState::default());
    worklist.push_back(cfg.entry);

    let mut steps = 0usize;
    let mut incomplete = false;

    while let Some(block_id) = worklist.pop_front() {
        if let Some(limit) = config.max_dataflow_steps {
            if steps >= limit {
                incomplete = true;
                break;
            }
        }
        steps = match steps.checked_add(1) {
            Some(value) => value,
            None => {
                incomplete = true;
                break;
            }
        };

        let Some(block) = cfg.blocks.get(&block_id) else {
            incomplete = true;
            break;
        };
        let mut output = incoming
            .get(&block_id)
            .cloned()
            .unwrap_or_default();

        for event in &block.events {
            let _ = transfer_event(&mut output, event, &place_info, &cfg.scopes);
        }

        for successor in &block.successors {
            let changed = match incoming.get(successor) {
                Some(existing) => {
                    let joined = existing.join(&output);
                    if &joined != existing {
                        incoming.insert(*successor, joined);
                        true
                    } else {
                        false
                    }
                }
                None => {
                    incoming.insert(*successor, output.clone());
                    true
                }
            };

            if changed {
                worklist.push_back(*successor);
            }
        }
    }

    if incomplete {
        diagnostics.push(OwnershipDiagnostic {
            code: OwnershipDiagnosticCode::AnalysisBudgetExceeded,
            block: None,
            event_index: None,
            place: None,
            loan: None,
            site: SourceSite::default(),
            message: "ownership analysis did not reach a fixed point within the configured operational budget".into(),
        });
        return finish_report(
            AnalysisStatus::Incomplete,
            diagnostics,
            steps,
            config.max_diagnostics,
        );
    }

    // Emit diagnostics only after the dataflow fixed point has been reached.
    // This prevents transient intermediate states from producing false errors.
    let reachable: BTreeSet<_> = incoming.keys().copied().collect();

    for block_id in reachable {
        let Some(block) = cfg.blocks.get(&block_id) else {
            continue;
        };
        let mut state = incoming.get(&block_id).cloned().unwrap_or_default();

        for (event_index, event) in block.events.iter().enumerate() {
            let event_diagnostics = transfer_event(
                &mut state,
                event,
                &place_info,
                &cfg.scopes,
            );
            for mut diagnostic in event_diagnostics {
                diagnostic.block = Some(block_id);
                diagnostic.event_index = Some(event_index);
                diagnostics.push(diagnostic);
            }
        }
    }

    finish_report(
        AnalysisStatus::Complete,
        diagnostics,
        steps,
        config.max_diagnostics,
    )
}

fn finish_report(
    mut status: AnalysisStatus,
    mut diagnostics: Vec<OwnershipDiagnostic>,
    dataflow_steps: usize,
    maximum: Option<usize>,
) -> OwnershipReport {
    diagnostics.sort_by(|left, right| {
        (
            left.block,
            left.event_index,
            left.code,
            left.place.as_ref(),
            left.loan,
        )
            .cmp(&(
                right.block,
                right.event_index,
                right.code,
                right.place.as_ref(),
                right.loan,
            ))
    });

    diagnostics.dedup_by(|left, right| {
        left.code == right.code
            && left.block == right.block
            && left.event_index == right.event_index
            && left.place == right.place
            && left.loan == right.loan
    });

    if let Some(limit) = maximum {
        if diagnostics.len() > limit {
            diagnostics.truncate(limit);
            diagnostics.push(OwnershipDiagnostic {
                code: OwnershipDiagnosticCode::AnalysisBudgetExceeded,
                block: None,
                event_index: None,
                place: None,
                loan: None,
                site: SourceSite::default(),
                message: "ownership diagnostic budget exhausted; analysis is incomplete".into(),
            });
            status = AnalysisStatus::Incomplete;
        }
    }

    OwnershipReport {
        status,
        diagnostics,
        dataflow_steps,
    }
}

fn collect_declarations(
    cfg: &ControlFlowGraph,
) -> (BTreeMap<PlaceId, PlaceInfo>, Vec<OwnershipDiagnostic>) {
    let mut info = BTreeMap::new();
    let mut diagnostics = Vec::new();

    for block in cfg.blocks.values() {
        for (event_index, event) in block.events.iter().enumerate() {
            if let OwnershipEvent::Declare {
                place,
                mode,
                scope,
                site,
                ..
            } = event
            {
                if !place.projections.is_empty() {
                    diagnostics.push(OwnershipDiagnostic {
                        code: OwnershipDiagnosticCode::ConflictingDeclaration,
                        block: Some(block.id),
                        event_index: Some(event_index),
                        place: Some(place.clone()),
                        loan: None,
                        site: site.clone(),
                        message: "a declaration must identify a root place, not a projected field or element".into(),
                    });
                    continue;
                }

                if !cfg.scopes.contains(*scope) {
                    diagnostics.push(OwnershipDiagnostic {
                        code: OwnershipDiagnosticCode::InvalidScope,
                        block: Some(block.id),
                        event_index: Some(event_index),
                        place: Some(place.clone()),
                        loan: None,
                        site: site.clone(),
                        message: "declaration refers to an unregistered scope".into(),
                    });
                    continue;
                }

                match info.get(&place.root) {
                    Some(existing)
                        if existing.mode != *mode || existing.scope != *scope =>
                    {
                        diagnostics.push(OwnershipDiagnostic {
                            code: OwnershipDiagnosticCode::ConflictingDeclaration,
                            block: Some(block.id),
                            event_index: Some(event_index),
                            place: Some(place.clone()),
                            loan: None,
                            site: site.clone(),
                            message: "the same place identity has inconsistent ownership declarations".into(),
                        });
                    }
                    Some(_) => {}
                    None => {
                        info.insert(
                            place.root,
                            PlaceInfo {
                                mode: *mode,
                                scope: *scope,
                            },
                        );
                    }
                }
            }
        }
    }

    (info, diagnostics)
}

// -----------------------------------------------------------------------------
// Event transfer
// -----------------------------------------------------------------------------

fn transfer_event(
    state: &mut FlowState,
    event: &OwnershipEvent,
    place_info: &BTreeMap<PlaceId, PlaceInfo>,
    scopes: &ScopeTree,
) -> Vec<OwnershipDiagnostic> {
    let mut diagnostics = Vec::new();

    match event {
        OwnershipEvent::Declare {
            place,
            initialized,
            scope,
            site,
            ..
        } => {
            if !scopes.contains(*scope) || !place_info.contains_key(&place.root) {
                diagnostics.push(diag(
                    OwnershipDiagnosticCode::InvalidScope,
                    Some(place.clone()),
                    None,
                    site,
                    "declaration has no valid registered scope or ownership record",
                ));
            } else {
                state.places.insert(
                    place.clone(),
                    if *initialized {
                        Availability::Available
                    } else {
                        Availability::Uninitialized
                    },
                );
            }
        }

        OwnershipEvent::Read { place, site } => {
            check_available(state, place, site, &mut diagnostics);
            for loan in overlapping_loans(state, place) {
                if loan.kind == BorrowKind::Mutable {
                    diagnostics.push(diag(
                        OwnershipDiagnosticCode::AccessWhileMutablyBorrowed,
                        Some(place.clone()),
                        None,
                        site,
                        "cannot access the owner while an overlapping mutable loan is active",
                    ));
                    break;
                }
            }
        }

        OwnershipEvent::Move { place, site } => {
            check_available(state, place, site, &mut diagnostics);
            if !overlapping_loans(state, place).is_empty() {
                diagnostics.push(diag(
                    OwnershipDiagnosticCode::MoveWhileBorrowed,
                    Some(place.clone()),
                    None,
                    site,
                    "cannot move a place while an overlapping loan is active",
                ));
            } else if place_is_available(state, place) {
                state.places.insert(place.clone(), Availability::Moved);
            }
        }

        OwnershipEvent::Write { place, site } => {
            if !overlapping_loans(state, place).is_empty() {
                diagnostics.push(diag(
                    OwnershipDiagnosticCode::MutationWhileBorrowed,
                    Some(place.clone()),
                    None,
                    site,
                    "cannot write a place while an overlapping loan is active",
                ));
            } else {
                state.places.insert(place.clone(), Availability::Available);
                if place.projections.is_empty() {
                    // Whole-place reinitialization supersedes stale partial
                    // move states under that root.
                    state.places.retain(|candidate, _| {
                        candidate.root != place.root || candidate == place
                    });
                }
            }
        }

        OwnershipEvent::Borrow {
            loan,
            place,
            kind,
            borrower_scope,
            site,
        } => {
            let mut valid = true;
            check_available(state, place, site, &mut diagnostics);
            if !place_is_available(state, place) {
                valid = false;
            }

            let Some(owner) = place_info.get(&place.root) else {
                diagnostics.push(diag(
                    OwnershipDiagnosticCode::ReadBeforeInitialization,
                    Some(place.clone()),
                    Some(*loan),
                    site,
                    "cannot borrow a place without a resolved declaration",
                ));
                valid = false;
                // Do not create a loan from an unresolved place.
                if !valid {
                    continue_event();
                }
                if !valid {
                    // The branch below deliberately does not register the loan.
                }
                // Continue to common event completion.
                if !valid {
                    // No further action is required.
                }
                // Keep control flow in this arm without panicking.
                if !valid {
                    // Intentionally empty.
                }
                // The missing declaration is already diagnosed.
                if !valid {
                    // Avoid a synthetic active loan.
                }
                // Rust has no fall-through label; continue with the arm.
                if !valid {
                    // This block is intentionally explicit for auditability.
                }
                // End the arm after this condition below.
            };

            if !scopes.contains(*borrower_scope) {
                diagnostics.push(diag(
                    OwnershipDiagnosticCode::InvalidScope,
                    Some(place.clone()),
                    Some(*loan),
                    site,
                    "borrow refers to an unregistered borrower scope",
                ));
                valid = false;
            }

            if let Some(owner) = place_info.get(&place.root) {
                if !scopes.outlives(owner.scope, *borrower_scope) {
                    diagnostics.push(diag(
                        OwnershipDiagnosticCode::BorrowOutlivesOwner,
                        Some(place.clone()),
                        Some(*loan),
                        site,
                        "the owner scope does not outlive the borrower's scope",
                    ));
                    valid = false;
                }
            }

            if state.loans.contains_key(loan) {
                diagnostics.push(diag(
                    OwnershipDiagnosticCode::ConflictingBorrow,
                    Some(place.clone()),
                    Some(*loan),
                    site,
                    "loan identity is already active or may be active",
                ));
                valid = false;
            }

            for existing in overlapping_loans(state, place) {
                let conflict = *kind == BorrowKind::Mutable
                    || existing.kind == BorrowKind::Mutable;
                if conflict {
                    diagnostics.push(diag(
                        OwnershipDiagnosticCode::ConflictingBorrow,
                        Some(place.clone()),
                        Some(*loan),
                        site,
                        "overlapping mutable and shared/mutable loans are not permitted",
                    ));
                    valid = false;
                    break;
                }
            }

            if valid {
                state.loans.insert(
                    *loan,
                    ActiveLoan {
                        place: place.clone(),
                        kind: *kind,
                        borrower_scope: *borrower_scope,
                        presence: LoanPresence::Active,
                    },
                );
            }
        }

        OwnershipEvent::UseBorrow { loan, site } => match state.loans.get(loan) {
            Some(active) if active.presence == LoanPresence::Active => {
                let _ = active;
            }
            Some(_) => diagnostics.push(diag(
                OwnershipDiagnosticCode::LoanNotDefinitelyActive,
                None,
                Some(*loan),
                site,
                "borrow is not definitely active on every incoming control-flow path",
            )),
            None => diagnostics.push(diag(
                OwnershipDiagnosticCode::UnknownLoan,
                None,
                Some(*loan),
                site,
                "borrow use has no active loan",
            )),
        },

        OwnershipEvent::EndBorrow { loan, site } => {
            if state.loans.remove(loan).is_none() {
                diagnostics.push(diag(
                    OwnershipDiagnosticCode::UnknownLoan,
                    None,
                    Some(*loan),
                    site,
                    "cannot end a loan that is not active",
                ));
            }
        }

        OwnershipEvent::Mutate { place, site } => {
            check_available(state, place, site, &mut diagnostics);
            if !overlapping_loans(state, place).is_empty() {
                diagnostics.push(diag(
                    OwnershipDiagnosticCode::MutationWhileBorrowed,
                    Some(place.clone()),
                    None,
                    site,
                    "cannot mutate a place while an overlapping loan is active",
                ));
            }
        }

        OwnershipEvent::Drop { place, site } => {
            check_available(state, place, site, &mut diagnostics);
            if !overlapping_loans(state, place).is_empty() {
                diagnostics.push(diag(
                    OwnershipDiagnosticCode::MoveWhileBorrowed,
                    Some(place.clone()),
                    None,
                    site,
                    "cannot drop a place while an overlapping loan is active",
                ));
            } else if place_is_available(state, place) {
                match place_info.get(&place.root).map(|info| info.mode) {
                    Some(OwnershipMode::Linear) => diagnostics.push(diag(
                        OwnershipDiagnosticCode::LinearValueDropped,
                        Some(place.clone()),
                        None,
                        site,
                        "a linear value cannot be discarded without being consumed",
                    )),
                    _ => {
                        state.places.insert(place.clone(), Availability::Dropped);
                    }
                }
            }
        }

        OwnershipEvent::ScopeExit { scope, site } => {
            if !scopes.contains(*scope) {
                diagnostics.push(diag(
                    OwnershipDiagnosticCode::InvalidScope,
                    None,
                    None,
                    site,
                    "scope exit refers to an unregistered scope",
                ));
            }

            for (place_id, info) in place_info {
                if info.scope != *scope {
                    continue;
                }
                let place = PlacePath::root(*place_id);
                let availability = state
                    .places
                    .get(&place)
                    .copied()
                    .unwrap_or(Availability::Uninitialized);

                if info.mode == OwnershipMode::Linear
                    && availability != Availability::Moved
                {
                    diagnostics.push(diag(
                        OwnershipDiagnosticCode::LinearValueNotConsumed,
                        Some(place.clone()),
                        None,
                        site,
                        "linear value was not definitely consumed exactly once before scope exit",
                    ));
                }

                if availability == Availability::Available
                    && info.mode != OwnershipMode::Linear
                {
                    state.places.insert(place.clone(), Availability::Dropped);
                }
            }

            let owner_roots: BTreeSet<_> = place_info
                .iter()
                .filter_map(|(id, info)| (info.scope == *scope).then_some(*id))
                .collect();

            for (loan_id, loan) in &state.loans {
                if owner_roots.contains(&loan.place.root) {
                    diagnostics.push(diag(
                        OwnershipDiagnosticCode::BorrowOutlivesOwner,
                        Some(loan.place.clone()),
                        Some(*loan_id),
                        site,
                        "an active borrow remains when its owner scope exits",
                    ));
                }
            }
        }
    }

    diagnostics
}

fn continue_event() {
    // A named no-op keeps adapter-side error recovery explicit without
    // requiring a panic or early process termination.
}

fn diag(
    code: OwnershipDiagnosticCode,
    place: Option<PlacePath>,
    loan: Option<LoanId>,
    site: &SourceSite,
    message: &str,
) -> OwnershipDiagnostic {
    OwnershipDiagnostic {
        code,
        block: None,
        event_index: None,
        place,
        loan,
        site: site.clone(),
        message: message.to_owned(),
    }
}

fn overlapping_loans<'a>(
    state: &'a FlowState,
    place: &PlacePath,
) -> Vec<&'a ActiveLoan> {
    state
        .loans
        .values()
        .filter(|loan| loan.place.may_overlap(place))
        .collect()
}

fn place_is_available(state: &FlowState, place: &PlacePath) -> bool {
    let Some(status) = state.places.get(place).copied() else {
        return false;
    };
    if status != Availability::Available {
        return false;
    }

    // An unavailable ancestor invalidates access to a descendant.
    for (candidate, availability) in &state.places {
        if candidate != place
            && candidate.is_prefix_of(place)
            && *availability != Availability::Available
        {
            return false;
        }
    }

    // A partially moved/dropped child makes a whole-parent read unsafe.
    for (candidate, availability) in &state.places {
        if place.is_prefix_of(candidate)
            && candidate != place
            && *availability != Availability::Available
        {
            return false;
        }
    }

    true
}

fn check_available(
    state: &FlowState,
    place: &PlacePath,
    site: &SourceSite,
    diagnostics: &mut Vec<OwnershipDiagnostic>,
) {
    match state.places.get(place).copied() {
        None | Some(Availability::Uninitialized) => diagnostics.push(diag(
            OwnershipDiagnosticCode::ReadBeforeInitialization,
            Some(place.clone()),
            None,
            site,
            "place is read before it is initialized",
        )),
        Some(Availability::Moved) | Some(Availability::Dropped) => {
            diagnostics.push(diag(
                OwnershipDiagnosticCode::UseAfterMove,
                Some(place.clone()),
                None,
                site,
                "place is used after being moved or dropped",
            ));
        }
        Some(Availability::MaybeUnavailable) => diagnostics.push(diag(
            OwnershipDiagnosticCode::PossiblyUnavailable,
            Some(place.clone()),
            None,
            site,
            "place is not definitely available on every incoming control-flow path",
        )),
        Some(Availability::Available) if !place_is_available(state, place) => {
            diagnostics.push(diag(
                OwnershipDiagnosticCode::PossiblyUnavailable,
                Some(place.clone()),
                None,
                site,
                "place or one of its overlapping subplaces is unavailable",
            ));
        }
        Some(Availability::Available) => {}
    }
}
