
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Compiler Ownership Analysis
//!
//! Path: `src/compiler/analysis/ownership.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//!
//! PURPOSE
//!
//! Perform compiler-level, CFG-aware ownership dataflow analysis over
//! canonical ZUIR ownership contracts and recorded ownership events.
//!
//! This module complements, rather than replaces:
//!
//! - `src/zuir/ir/ownership.rs`:
//!   canonical ownership contracts and recorded ownership events;
//! - `src/zuir/analysis/cfg.rs`:
//!   canonical control-flow graph construction;
//! - `src/zuir/verify/ownership.rs`:
//!   structural ownership-index and region-capture verification;
//! - `src/zuir/ir/use_def.rs`:
//!   canonical SSA definition/use indexing;
//! - `src/compiler/borrow_checker.rs`:
//!   legacy lexical-scope ownership operations.
//!
//! OWNS
//!
//! - Compiler ownership-flow event adapters.
//! - Ownership-flow input and result contracts.
//! - CFG-aware forward dataflow.
//! - Conservative control-flow joins.
//! - Path-sensitive move, drop, borrow and linearity diagnostics.
//! - Analysis completeness reporting.
//!
//! DOES NOT OWN
//!
//! - Ownership kinds or ownership capabilities.
//! - Ownership contract storage.
//! - Canonical value, block or operation identifiers.
//! - CFG construction.
//! - SSA definition or use indexing.
//! - Source syntax or AST interpretation.
//! - Type inference or lifetime inference.
//! - Dialect-specific ownership semantics.
//! - Physical memory allocation or device ownership.
//! - Runtime resource management.
//!
//! IMPORTANT INTEGRATION CONTRACT
//!
//! The caller must supply:
//!
//! 1. A validated canonical CFG.
//! 2. The canonical ownership index.
//! 3. The set of values tracked by this CFG.
//! 4. Values available at CFG entry.
//! 5. Events in actual operation execution order within each block.
//! 6. Explicit definition events for values defined in this CFG.
//! 7. Explicit borrow-ending events.
//!
//! `OwnershipUseKind::BorrowShared` and `BorrowExclusive` describe ownership
//! events, but the canonical event itself does not encode a borrow-ending
//! point. This module therefore does not invent lexical or operation lifetime
//! boundaries. The frontend/IR adapter must supply `EndBorrow` events from
//! authoritative lifetime information.
//!
//! An unsupported extension event is never silently treated as unrestricted.
//!
//! SCALABILITY
//!
//! There are no fixed limits on blocks, edges, values, operations, borrows,
//! or hardware resources. Worklist processing is iterative and converges over
//! the finite input state space. Actual memory, address-space, time and
//! identifier limits remain properties of the host and caller's resource
//! policy. Literal infinity cannot be guaranteed.
//!
//! SAFETY
//!
//! Unsafe Rust is prohibited.

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use crate::zuir::analysis::cfg::ControlFlowGraph;
use crate::zuir::ids::{BlockId, OperationId, ValueId};
use crate::zuir::ir::{
    BlockTerminator,
    BorrowMutability,
    OwnershipIndex,
    OwnershipKind,
    OwnershipUse,
    OwnershipUseKind,
    ZuirBlock,
};

// =============================================================================
// Public input contracts
// =============================================================================

/// One additional ownership-flow event needed by compiler dataflow.
///
/// Canonical recorded ownership events remain represented by
/// `OwnershipUse`. The additional variants provide information that the
/// canonical ownership-use index does not currently encode.
///
/// `Define` must be supplied for every tracked value defined inside this
/// CFG. Values already available at entry belong in `initial_values`.
///
/// `Duplicate` represents an actual duplication/copy operation. A normal
/// `Read` is not a copy.
///
/// `EndBorrow` identifies the operation that originally created the borrow.
/// Its `operation` is the operation at which the borrow ends.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OwnershipFlowEvent {
    /// An event recorded in the canonical ownership index.
    Recorded(OwnershipUse),

    /// A value definition becomes available at this operation.
    Define {
        value: ValueId,
        operation: OperationId,
    },

    /// An operation duplicates a value.
    Duplicate {
        value: ValueId,
        operation: OperationId,
    },

    /// End a borrow created by `borrow_operation`.
    EndBorrow {
        value: ValueId,
        operation: OperationId,
        borrow_operation: OperationId,
    },
}

impl OwnershipFlowEvent {
    /// Return the value affected by this event.
    #[must_use]
    pub const fn value(&self) -> ValueId {
        match self {
            Self::Recorded(event) => event.value,
            Self::Define { value, .. }
            | Self::Duplicate { value, .. }
            | Self::EndBorrow { value, .. } => *value,
        }
    }

    /// Return the operation at which this event occurs.
    #[must_use]
    pub const fn operation(&self) -> OperationId {
        match self {
            Self::Recorded(event) => event.operation,
            Self::Define { operation, .. }
            | Self::Duplicate { operation, .. }
            | Self::EndBorrow { operation, .. } => *operation,
        }
    }
}

/// Complete input to compiler ownership analysis.
///
/// `events` is keyed by block. Each block's event vector must be in actual
/// operation order, not identifier order or insertion order.
///
/// The event sequence must contain every canonical ownership event belonging
/// to a tracked value and a block in this CFG, exactly once.
///
/// The caller must build this sequence using canonical block operation order.
/// Do not sort operations by their numeric IDs.
///
/// `tracked_values` limits this analysis to values belonging to the current
/// ownership-analysis scope. Nested regions should be analyzed using their
/// own CFG and corresponding value scope.
///
/// `initial_values` represents values already available on entry, including
/// function parameters and explicitly available captures.
pub struct OwnershipAnalysisInput<'a> {
    /// Canonical ownership contracts and recorded ownership events.
    pub ownership: &'a OwnershipIndex,

    /// Canonical, already-constructed control-flow graph.
    pub cfg: &'a ControlFlowGraph,

    /// Canonical blocks used to establish operation ordering and exits.
    pub blocks: &'a [ZuirBlock],

    /// Ordered ownership events for each block.
    pub events: &'a BTreeMap<BlockId, Vec<OwnershipFlowEvent>>,

    /// Values whose ownership is checked by this analysis.
    pub tracked_values: &'a BTreeSet<ValueId>,

    /// Values available when entering the CFG.
    pub initial_values: &'a BTreeSet<ValueId>,
}

// =============================================================================
// Public diagnostics and reports
// =============================================================================

/// One deterministic ownership-analysis diagnostic.
///
/// Diagnostic codes are the stable machine-readable interface.
/// Human-readable messages may evolve.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct OwnershipAnalysisIssue {
    code: &'static str,
    value: Option<ValueId>,
    block: Option<BlockId>,
    operation: Option<OperationId>,
    message: String,
}

impl OwnershipAnalysisIssue {
    fn new(
        code: &'static str,
        value: Option<ValueId>,
        block: Option<BlockId>,
        operation: Option<OperationId>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            code,
            value,
            block,
            operation,
            message: message.into(),
        }
    }

    /// Return the stable diagnostic code.
    #[must_use]
    pub const fn code(&self) -> &'static str {
        self.code
    }

    /// Return the affected value, if known.
    #[must_use]
    pub const fn value(&self) -> Option<ValueId> {
        self.value
    }

    /// Return the affected block, if known.
    #[must_use]
    pub const fn block(&self) -> Option<BlockId> {
        self.block
    }

    /// Return the affected operation, if known.
    #[must_use]
    pub const fn operation(&self) -> Option<OperationId> {
        self.operation
    }

    /// Return the human-readable explanation.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

/// Result of one compiler ownership-analysis run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnershipAnalysisReport {
    issues: Vec<OwnershipAnalysisIssue>,
    complete: bool,
    reachable_blocks: usize,
    events_analyzed: usize,
    values_analyzed: usize,
}

impl OwnershipAnalysisReport {
    fn new() -> Self {
        Self {
            issues: Vec::new(),
            complete: true,
            reachable_blocks: 0,
            events_analyzed: 0,
            values_analyzed: 0,
        }
    }

    /// Return diagnostics in deterministic order.
    #[must_use]
    pub fn issues(&self) -> &[OwnershipAnalysisIssue] {
        &self.issues
    }

    /// Whether the analysis completed without encountering unknown semantics
    /// or missing information required by its contract.
    #[must_use]
    pub const fn is_complete(&self) -> bool {
        self.complete
    }

    /// Whether the analysis is complete and found no violations.
    ///
    /// An incomplete analysis is never reported as valid.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.complete && self.issues.is_empty()
    }

    /// Number of diagnostics.
    #[must_use]
    pub fn issue_count(&self) -> usize {
        self.issues.len()
    }

    /// Number of reachable CFG blocks processed.
    #[must_use]
    pub const fn reachable_blocks(&self) -> usize {
        self.reachable_blocks
    }

    /// Number of ownership events processed.
    #[must_use]
    pub const fn events_analyzed(&self) -> usize {
        self.events_analyzed
    }

    /// Number of tracked values.
    #[must_use]
    pub const fn values_analyzed(&self) -> usize {
        self.values_analyzed
    }
}

// =============================================================================
// Internal dataflow lattice
// =============================================================================

/// Abstract availability of a value at a program point.
///
/// `Maybe` means that different incoming paths disagree about whether the
/// value is available. A later read, move, or borrow must therefore be
/// treated conservatively.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Availability {
    /// The value has not been defined or made available on this path.
    Uninitialized,

    /// The value is available.
    Available,

    /// Ownership has been transferred or the value has been dropped.
    Moved,

    /// Incoming paths disagree about availability.
    Maybe,
}

impl Availability {
    /// Join two path states.
    ///
    /// The join is conservative: disagreement never becomes proof that a
    /// value is available.
    fn join(self, other: Self) -> Self {
        if self == other {
            self
        } else {
            Self::Maybe
        }
    }
}

/// Abstract state for one value.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct ValueState {
    availability: Option<Availability>,
    active_borrows: BTreeMap<OperationId, BorrowMutability>,
}

impl ValueState {
    fn availability(&self) -> Availability {
        self.availability
            .unwrap_or(Availability::Uninitialized)
    }

    fn join_from(&mut self, other: &Self) -> bool {
        let previous = self.clone();

        self.availability = Some(
            self.availability()
                .join(other.availability()),
        );

        for (origin, mutability) in &other.active_borrows {
            self.active_borrows
                .entry(*origin)
                .or_insert(*mutability);
        }

        *self != previous
    }
}

/// State for all values tracked in one CFG.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct FlowState {
    values: BTreeMap<ValueId, ValueState>,
}

impl FlowState {
    fn new(
        tracked_values: &BTreeSet<ValueId>,
        initial_values: &BTreeSet<ValueId>,
    ) -> Self {
        let mut state = Self::default();

        for value in tracked_values {
            let availability = if initial_values.contains(value) {
                Availability::Available
            } else {
                Availability::Uninitialized
            };

            state.values.insert(
                *value,
                ValueState {
                    availability: Some(availability),
                    active_borrows: BTreeMap::new(),
                },
            );
        }

        state
    }

    fn value(&self, value: ValueId) -> ValueState {
        self.values
            .get(&value)
            .cloned()
            .unwrap_or_default()
    }

    fn value_mut(&mut self, value: ValueId) -> &mut ValueState {
        self.values.entry(value).or_default()
    }

    fn join_from(&mut self, other: &Self) -> bool {
        let previous = self.clone();

        for (value, other_state) in &other.values {
            self.value_mut(*value).join_from(other_state);
        }

        *self != previous
    }
}

// =============================================================================
// Event indexing and validation
// =============================================================================

type RecordedEventKey = (
    ValueId,
    BlockId,
    OperationId,
    String,
);

fn recorded_event_key(event: &OwnershipUse) -> RecordedEventKey {
    (
        event.value,
        event.block,
        event.operation,
        format!("{:?}", event.kind),
    )
}

fn insert_event_count(
    counts: &mut BTreeMap<RecordedEventKey, usize>,
    event: &OwnershipUse,
) {
    let count = counts
        .entry(recorded_event_key(event))
        .or_insert(0);

    *count += 1;
}

fn validate_recorded_event_coverage(
    input: &OwnershipAnalysisInput<'_>,
    issues: &mut BTreeSet<OwnershipAnalysisIssue>,
    complete: &mut bool,
) {
    let mut expected = BTreeMap::<RecordedEventKey, usize>::new();
    let mut actual = BTreeMap::<RecordedEventKey, usize>::new();

    for value in input.tracked_values {
        for event in input.ownership.uses_of(*value) {
            if input.cfg.contains_block(event.block) {
                insert_event_count(&mut expected, event);
            }
        }
    }

    for (block_id, events) in input.events {
        if !input.cfg.contains_block(*block_id) {
            issues.insert(OwnershipAnalysisIssue::new(
                "COMP-OWN-EVENT-BLOCK-OUTSIDE-CFG",
                None,
                Some(*block_id),
                None,
                format!(
                    "ownership events were supplied for block {block_id}, \
                     which is not part of the analyzed CFG"
                ),
            ));
            *complete = false;
            continue;
        }

        for event in events {
            if let OwnershipFlowEvent::Recorded(recorded) = event {
                if !input.tracked_values.contains(&recorded.value) {
                    continue;
                }

                if recorded.block != *block_id {
                    issues.insert(OwnershipAnalysisIssue::new(
                        "COMP-OWN-EVENT-BLOCK-MISMATCH",
                        Some(recorded.value),
                        Some(*block_id),
                        Some(recorded.operation),
                        "recorded ownership event is stored under a \
                         different block than its canonical block ID",
                    ));
                    *complete = false;
                    continue;
                }

                insert_event_count(&mut actual, recorded);
            }
        }
    }

    for (key, expected_count) in &expected {
        let actual_count = actual.get(key).copied().unwrap_or(0);

        if actual_count != *expected_count {
            let (value, block, operation, _) = key;

            issues.insert(OwnershipAnalysisIssue::new(
                "COMP-OWN-EVENT-COVERAGE",
                Some(*value),
                Some(*block),
                Some(*operation),
                format!(
                    "canonical ownership event occurs {expected_count} \
                     time(s), but the ordered flow input contains \
                     {actual_count} matching event(s)"
                ),
            ));
            *complete = false;
        }
    }

    for (key, actual_count) in &actual {
        let expected_count = expected.get(key).copied().unwrap_or(0);

        if actual_count > &expected_count {
            let (value, block, operation, _) = key;

            issues.insert(OwnershipAnalysisIssue::new(
                "COMP-OWN-UNREGISTERED-EVENT",
                Some(*value),
                Some(*block),
                Some(*operation),
                "the flow input contains an ownership event that is not \
                 represented by the canonical ownership index",
            ));
            *complete = false;
        }
    }
}

fn validate_input(
    input: &OwnershipAnalysisInput<'_>,
    issues: &mut BTreeSet<OwnershipAnalysisIssue>,
    complete: &mut bool,
) {
    if let Err(error) = input.ownership.validate() {
        issues.insert(OwnershipAnalysisIssue::new(
            "COMP-OWN-INVALID-INDEX",
            None,
            None,
            None,
            format!("canonical ownership index is invalid: {error}"),
        ));
        *complete = false;
    }

    if input.cfg.entry().is_none() {
        issues.insert(OwnershipAnalysisIssue::new(
            "COMP-OWN-MISSING-CFG-ENTRY",
            None,
            None,
            None,
            "ownership dataflow requires an explicitly supplied CFG entry",
        ));
        *complete = false;
    }

    if input.cfg.is_empty() {
        issues.insert(OwnershipAnalysisIssue::new(
            "COMP-OWN-EMPTY-CFG",
            None,
            None,
            None,
            "ownership dataflow cannot analyze an empty CFG",
        ));
        *complete = false;
    }

    for value in input.initial_values {
        if !input.tracked_values.contains(value) {
            issues.insert(OwnershipAnalysisIssue::new(
                "COMP-OWN-INITIAL-VALUE-UNTRACKED",
                Some(*value),
                None,
                None,
                "an entry value is not included in the tracked-value set",
            ));
            *complete = false;
        }
    }

    for value in input.tracked_values {
        match input.ownership.contract(*value) {
            Some(contract) => {
                if let Err(error) = contract.validate() {
                    issues.insert(OwnershipAnalysisIssue::new(
                        "COMP-OWN-INVALID-CONTRACT",
                        Some(*value),
                        None,
                        None,
                        format!("ownership contract is invalid: {error}"),
                    ));
                    *complete = false;
                }

                if matches!(contract.kind, OwnershipKind::Extension(_)) {
                    issues.insert(OwnershipAnalysisIssue::new(
                        "COMP-OWN-UNKNOWN-CONTRACT-SEMANTICS",
                        Some(*value),
                        None,
                        None,
                        "the value uses an extension-defined ownership kind; \
                         its dialect-specific ownership semantics are not \
                         implemented by this generic analysis",
                    ));
                    *complete = false;
                }
            }
            None => {
                issues.insert(OwnershipAnalysisIssue::new(
                    "COMP-OWN-MISSING-CONTRACT",
                    Some(*value),
                    None,
                    None,
                    "tracked value has no canonical ownership contract",
                ));
                *complete = false;
            }
        }
    }

    validate_recorded_event_coverage(input, issues, complete);
}

// =============================================================================
// Block and event ordering
// =============================================================================

fn index_blocks<'a>(
    blocks: &'a [ZuirBlock],
) -> BTreeMap<BlockId, &'a ZuirBlock> {
    let mut result = BTreeMap::new();

    for block in blocks {
        result.entry(block.id()).or_insert(block);
    }

    result
}

fn validate_event_order(
    input: &OwnershipAnalysisInput<'_>,
    block: &ZuirBlock,
    events: &[OwnershipFlowEvent],
    issues: &mut BTreeSet<OwnershipAnalysisIssue>,
    complete: &mut bool,
) {
    let mut operation_positions = BTreeMap::<OperationId, usize>::new();

    for (position, operation) in block.operations().iter().enumerate() {
        if operation_positions
            .insert(*operation, position)
            .is_some()
        {
            issues.insert(OwnershipAnalysisIssue::new(
                "COMP-OWN-DUPLICATE-OPERATION-IN-BLOCK",
                None,
                Some(block.id()),
                Some(*operation),
                "the block contains the same operation identity more than once",
            ));
            *complete = false;
        }
    }

    let mut previous_position = None;

    for event in events {
        let operation = event.operation();

        let Some(position) = operation_positions.get(&operation).copied() else {
            issues.insert(OwnershipAnalysisIssue::new(
                "COMP-OWN-EVENT-OPERATION-NOT-IN-BLOCK",
                Some(event.value()),
                Some(block.id()),
                Some(operation),
                "ownership event refers to an operation that is not listed \
                 in its containing block",
            ));
            *complete = false;
            continue;
        };

        if let Some(previous) = previous_position {
            if position < previous {
                issues.insert(OwnershipAnalysisIssue::new(
                    "COMP-OWN-EVENT-ORDER",
                    Some(event.value()),
                    Some(block.id()),
                    Some(operation),
                    "ownership events are not in canonical block operation order",
                ));
                *complete = false;
            }
        }

        previous_position = Some(position);
    }

    let _ = input;
}

// =============================================================================
// Diagnostics and transfer helpers
// =============================================================================

fn push_issue(
    issues: &mut BTreeSet<OwnershipAnalysisIssue>,
    code: &'static str,
    value: ValueId,
    block: BlockId,
    operation: OperationId,
    message: impl Into<String>,
) {
    issues.insert(OwnershipAnalysisIssue::new(
        code,
        Some(value),
        Some(block),
        Some(operation),
        message,
    ));
}

fn check_available(
    state: &ValueState,
    value: ValueId,
    block: BlockId,
    operation: OperationId,
    issues: &mut BTreeSet<OwnershipAnalysisIssue>,
) -> bool {
    match state.availability() {
        Availability::Available => true,

        Availability::Moved => {
            push_issue(
                issues,
                "COMP-OWN-USE-AFTER-MOVE",
                value,
                block,
                operation,
                "value is used after ownership was transferred or dropped",
            );
            false
        }

        Availability::Uninitialized => {
            push_issue(
                issues,
                "COMP-OWN-USE-BEFORE-DEFINITION",
                value,
                block,
                operation,
                "value is used before it is available on this control-flow path",
            );
            false
        }

        Availability::Maybe => {
            push_issue(
                issues,
                "COMP-OWN-POSSIBLE-USE-AFTER-MOVE",
                value,
                block,
                operation,
                "value is not available on every incoming control-flow path",
            );
            false
        }
    }
}

fn active_borrow_conflict(
    state: &ValueState,
    requested: BorrowMutability,
) -> bool {
    match requested {
        BorrowMutability::Shared => state
            .active_borrows
            .values()
            .any(|kind| *kind == BorrowMutability::Exclusive),

        BorrowMutability::Exclusive => {
            !state.active_borrows.is_empty()
        }
    }
}

fn check_capability(
    input: &OwnershipAnalysisInput<'_>,
    value: ValueId,
    operation: OperationId,
    block: BlockId,
    allowed: bool,
    action: &'static str,
    issues: &mut BTreeSet<OwnershipAnalysisIssue>,
) {
    if !allowed {
        push_issue(
            issues,
            "COMP-OWN-CAPABILITY-VIOLATION",
            value,
            block,
            operation,
            format!(
                "ownership contract does not permit this {action} operation"
            ),
        );
    }

    let _ = input;
}

// =============================================================================
// Transfer function
// =============================================================================

fn transfer_event(
    input: &OwnershipAnalysisInput<'_>,
    state: &mut FlowState,
    block: BlockId,
    event: &OwnershipFlowEvent,
    issues: &mut BTreeSet<OwnershipAnalysisIssue>,
    complete: &mut bool,
) {
    let value = event.value();
    let operation = event.operation();

    if !input.tracked_values.contains(&value) {
        return;
    }

    let Some(contract) = input.ownership.contract(value) else {
        return;
    };

    let value_state = state.value_mut(value);
    let capabilities = contract.capabilities;

    match event {
        OwnershipFlowEvent::Define { .. } => {
            // A definition generates a new available SSA value.
            // Borrow state is retained conservatively; a valid SSA producer
            // must not redefine an identity that is still borrowed.
            if !value_state.active_borrows.is_empty() {
                push_issue(
                    issues,
                    "COMP-OWN-REDEFINITION-WHILE-BORROWED",
                    value,
                    block,
                    operation,
                    "value is defined while a borrow of the same value is active",
                );
            }

            value_state.availability = Some(Availability::Available);
        }

        OwnershipFlowEvent::Duplicate { .. } => {
            if check_available(
                value_state,
                value,
                block,
                operation,
                issues,
            ) {
                check_capability(
                    input,
                    value,
                    operation,
                    block,
                    capabilities.may_copy,
                    "copy",
                    issues,
                );
            }
        }

        OwnershipFlowEvent::EndBorrow {
            borrow_operation,
            ..
        } => {
            if value_state
                .active_borrows
                .remove(borrow_operation)
                .is_none()
            {
                push_issue(
                    issues,
                    "COMP-OWN-END-UNKNOWN-BORROW",
                    value,
                    block,
                    operation,
                    format!(
                        "borrow ending operation refers to unknown or \
                         already-ended borrow created by operation \
                         {borrow_operation}"
                    ),
                );
            }
        }

        OwnershipFlowEvent::Recorded(recorded) => {
            match &recorded.kind {
                OwnershipUseKind::Read => {
                    if check_available(
                        value_state,
                        value,
                        block,
                        operation,
                        issues,
                    ) && value_state
                        .active_borrows
                        .values()
                        .any(|kind| *kind == BorrowMutability::Exclusive)
                    {
                        push_issue(
                            issues,
                            "COMP-OWN-READ-DURING-EXCLUSIVE-BORROW",
                            value,
                            block,
                            operation,
                            "cannot read a value while an exclusive borrow is active",
                        );
                    }
                }

                OwnershipUseKind::Write => {
                    if check_available(
                        value_state,
                        value,
                        block,
                        operation,
                        issues,
                    ) && value_state
                        .active_borrows
                        .values()
                        .any(|kind| *kind == BorrowMutability::Shared)
                    {
                        push_issue(
                            issues,
                            "COMP-OWN-WRITE-DURING-SHARED-BORROW",
                            value,
                            block,
                            operation,
                            "cannot mutate a value while a shared borrow is active",
                        );
                    }
                }

                OwnershipUseKind::Move => {
                    if check_available(
                        value_state,
                        value,
                        block,
                        operation,
                        issues,
                    ) {
                        check_capability(
                            input,
                            value,
                            operation,
                            block,
                            capabilities.may_move,
                            "move",
                            issues,
                        );

                        if !value_state.active_borrows.is_empty() {
                            push_issue(
                                issues,
                                "COMP-OWN-MOVE-WHILE-BORROWED",
                                value,
                                block,
                                operation,
                                "cannot move a value while any borrow is active",
                            );
                        } else if capabilities.may_move {
                            value_state.availability =
                                Some(Availability::Moved);
                        }
                    }
                }

                OwnershipUseKind::Drop => {
                    if check_available(
                        value_state,
                        value,
                        block,
                        operation,
                        issues,
                    ) {
                        check_capability(
                            input,
                            value,
                            operation,
                            block,
                            capabilities.may_drop,
                            "drop",
                            issues,
                        );

                        if !value_state.active_borrows.is_empty() {
                            push_issue(
                                issues,
                                "COMP-OWN-DROP-WHILE-BORROWED",
                                value,
                                block,
                                operation,
                                "cannot drop a value while any borrow is active",
                            );
                        } else if capabilities.may_drop {
                            value_state.availability =
                                Some(Availability::Moved);
                        }
                    }
                }

                OwnershipUseKind::BorrowShared
                | OwnershipUseKind::CaptureShared => {
                    if check_available(
                        value_state,
                        value,
                        block,
                        operation,
                        issues,
                    ) {
                        check_capability(
                            input,
                            value,
                            operation,
                            block,
                            capabilities.may_borrow_shared,
                            "shared borrow",
                            issues,
                        );

                        if active_borrow_conflict(
                            value_state,
                            BorrowMutability::Shared,
                        ) {
                            push_issue(
                                issues,
                                "COMP-OWN-SHARED-BORROW-CONFLICT",
                                value,
                                block,
                                operation,
                                "cannot create a shared borrow while an \
                                 exclusive borrow is active",
                            );
                        } else if capabilities.may_borrow_shared {
                            value_state.active_borrows.insert(
                                operation,
                                BorrowMutability::Shared,
                            );
                        }
                    }
                }

                OwnershipUseKind::BorrowExclusive
                | OwnershipUseKind::CaptureExclusive => {
                    if check_available(
                        value_state,
                        value,
                        block,
                        operation,
                        issues,
                    ) {
                        check_capability(
                            input,
                            value,
                            operation,
                            block,
                            capabilities.may_borrow_exclusive,
                            "exclusive borrow",
                            issues,
                        );

                        if active_borrow_conflict(
                            value_state,
                            BorrowMutability::Exclusive,
                        ) {
                            push_issue(
                                issues,
                                "COMP-OWN-EXCLUSIVE-BORROW-CONFLICT",
                                value,
                                block,
                                operation,
                                "cannot create an exclusive borrow while \
                                 another borrow is active",
                            );
                        } else if capabilities.may_borrow_exclusive {
                            value_state.active_borrows.insert(
                                operation,
                                BorrowMutability::Exclusive,
                            );
                        }
                    }
                }

                OwnershipUseKind::Extension(_) => {
                    push_issue(
                        issues,
                        "COMP-OWN-UNKNOWN-EVENT-SEMANTICS",
                        value,
                        block,
                        operation,
                        "ownership event belongs to an extension whose \
                         ownership semantics are not registered with this analysis",
                    );
                    *complete = false;
                }
            }
        }
    }
}

// =============================================================================
// CFG dataflow
// =============================================================================

fn is_normal_exit(block: &ZuirBlock) -> bool {
    matches!(block.terminator(), BlockTerminator::Return { .. })
}

fn is_unreachable_exit(block: &ZuirBlock) -> bool {
    matches!(block.terminator(), BlockTerminator::Unreachable)
}

fn analyze_dataflow(
    input: &OwnershipAnalysisInput<'_>,
    block_index: &BTreeMap<BlockId, &ZuirBlock>,
    report: &mut OwnershipAnalysisReport,
) {
    let Some(entry) = input.cfg.entry() else {
        return;
    };

    let mut issues = BTreeSet::<OwnershipAnalysisIssue>::new();

    let mut in_states = BTreeMap::<BlockId, FlowState>::new();
    let mut out_states = BTreeMap::<BlockId, FlowState>::new();

    let initial = FlowState::new(
        input.tracked_values,
        input.initial_values,
    );

    in_states.insert(entry, initial);

    let mut pending = VecDeque::<BlockId>::new();
    let mut queued = BTreeSet::<BlockId>::new();

    pending.push_back(entry);
    queued.insert(entry);

    while let Some(block_id) = pending.pop_front() {
        queued.remove(&block_id);

        let Some(block) = block_index.get(&block_id).copied() else {
            issues.insert(OwnershipAnalysisIssue::new(
                "COMP-OWN-CFG-BLOCK-MISSING",
                None,
                Some(block_id),
                None,
                "CFG references a block absent from the supplied block collection",
            ));
            report.complete = false;
            continue;
        };

        let Some(input_state) = in_states.get(&block_id).cloned() else {
            continue;
        };

        let mut output_state = input_state;

        if let Some(events) = input.events.get(&block_id) {
            for event in events {
                report.events_analyzed += 1;

                transfer_event(
                    input,
                    &mut output_state,
                    block_id,
                    event,
                    &mut issues,
                    &mut report.complete,
                );
            }
        }

        let output_changed = out_states
            .get(&block_id)
            .map_or(true, |previous| previous != &output_state);

        if !output_changed {
            continue;
        }

        out_states.insert(block_id, output_state.clone());

        for edge in input.cfg.successors(block_id) {
            let successor = edge.target();

            if !input.cfg.contains_block(successor) {
                issues.insert(OwnershipAnalysisIssue::new(
                    "COMP-OWN-CFG-INVALID-SUCCESSOR",
                    None,
                    Some(block_id),
                    None,
                    format!(
                        "CFG successor {successor} is not present in the graph"
                    ),
                ));
                report.complete = false;
                continue;
            }

            let changed = match in_states.get_mut(&successor) {
                Some(existing) => existing.join_from(&output_state),
                None => {
                    in_states.insert(successor, output_state.clone());
                    true
                }
            };

            if changed && queued.insert(successor) {
                pending.push_back(successor);
            }
        }
    }

    report.reachable_blocks = in_states.len();

    // Validate normal exits only after the dataflow fixed point is reached.
    // Unreachable terminators do not impose normal-return consumption rules.
    for (block_id, state) in &out_states {
        let Some(block) = block_index.get(block_id).copied() else {
            continue;
        };

        if is_unreachable_exit(block) {
            continue;
        }

        if !input.cfg.successors(*block_id).next().is_none() {
            continue;
        }

        if !is_normal_exit(block) {
            issues.insert(OwnershipAnalysisIssue::new(
                "COMP-OWN-UNCLASSIFIED-EXIT",
                None,
                Some(*block_id),
                None,
                "reachable terminal block is neither a return nor an \
                 explicitly unreachable terminator; exit semantics are unknown",
            ));
            report.complete = false;
            continue;
        }

        for value in input.tracked_values {
            let Some(contract) = input.ownership.contract(*value) else {
                continue;
            };

            let value_state = state.value(*value);

            if !value_state.active_borrows.is_empty() {
                issues.insert(OwnershipAnalysisIssue::new(
                    "COMP-OWN-BORROW-ESCAPES-EXIT",
                    Some(*value),
                    Some(*block_id),
                    None,
                    "a borrow remains active at a normal function exit",
                ));
            }

            if matches!(contract.kind, OwnershipKind::Linear) {
                match value_state.availability() {
                    Availability::Available => {
                        issues.insert(OwnershipAnalysisIssue::new(
                            "COMP-OWN-LINEAR-NOT-CONSUMED",
                            Some(*value),
                            Some(*block_id),
                            None,
                            "linear value remains available at a normal exit; \
                             it must be consumed or transferred on this path",
                        ));
                    }

                    Availability::Maybe => {
                        issues.insert(OwnershipAnalysisIssue::new(
                            "COMP-OWN-LINEAR-NOT-CONSUMED-ON-EVERY-PATH",
                            Some(*value),
                            Some(*block_id),
                            None,
                            "linear value is not consumed on every path \
                             reaching this normal exit",
                        ));
                    }

                    Availability::Uninitialized => {
                        issues.insert(OwnershipAnalysisIssue::new(
                            "COMP-OWN-LINEAR-NOT-AVAILABLE-AT-EXIT",
                            Some(*value),
                            Some(*block_id),
                            None,
                            "linear value is not available on every path \
                             reaching this normal exit",
                        ));
                    }

                    Availability::Moved => {}
                }
            }
        }
    }

    report.issues = issues.into_iter().collect();
}

// =============================================================================
// Static definition validation
// =============================================================================

fn validate_definitions(
    input: &OwnershipAnalysisInput<'_>,
    issues: &mut BTreeSet<OwnershipAnalysisIssue>,
    complete: &mut bool,
) {
    let mut definitions = BTreeMap::<ValueId, usize>::new();

    for (block, events) in input.events {
        if !input.cfg.contains_block(*block) {
            continue;
        }

        for event in events {
            if let OwnershipFlowEvent::Define { value, .. } = event {
                if input.tracked_values.contains(value) {
                    *definitions.entry(*value).or_insert(0) += 1;
                }
            }
        }
    }

    for value in input.tracked_values {
        if input.initial_values.contains(value) {
            if definitions.get(value).copied().unwrap_or(0) > 0 {
                issues.insert(OwnershipAnalysisIssue::new(
                    "COMP-OWN-ENTRY-VALUE-REDEFINED",
                    Some(*value),
                    None,
                    None,
                    "a value declared available at CFG entry also has a \
                     definition event inside the analyzed CFG",
                ));
            }

            continue;
        }

        match definitions.get(value).copied().unwrap_or(0) {
            0 => {
                issues.insert(OwnershipAnalysisIssue::new(
                    "COMP-OWN-MISSING-DEFINITION",
                    Some(*value),
                    None,
                    None,
                    "tracked non-entry value has no definition event in this CFG",
                ));
                *complete = false;
            }

            1 => {}

            count => {
                issues.insert(OwnershipAnalysisIssue::new(
                    "COMP-OWN-MULTIPLE-DEFINITIONS",
                    Some(*value),
                    None,
                    None,
                    format!(
                        "tracked SSA value has {count} definition events; \
                         canonical SSA identity must have one definition"
                    ),
                ));
            }
        }
    }
}

// =============================================================================
// Public entry point
// =============================================================================

/// Analyze ownership over a canonical control-flow graph.
///
/// This analysis requires a CFG built and validated by
/// `crate::zuir::analysis::cfg::ControlFlowGraph::build`.
///
/// The function is deterministic for the same CFG, ownership index, tracked
/// value set, and ordered event input.
///
/// It uses an iterative worklist. Cycles and loops are handled by fixed-point
/// propagation; no recursion or arbitrary iteration limit is used.
///
/// This analysis does not claim complete source-level lifetime inference.
/// Borrow-ending events must be supplied by the authoritative frontend or IR
/// adapter.
#[must_use]
pub fn analyze_ownership(
    input: &OwnershipAnalysisInput<'_>,
) -> OwnershipAnalysisReport {
    let mut report = OwnershipAnalysisReport::new();

    report.values_analyzed = input.tracked_values.len();

    let mut issues = BTreeSet::<OwnershipAnalysisIssue>::new();

    validate_input(
        input,
        &mut issues,
        &mut report.complete,
    );

    let block_index = index_blocks(input.blocks);

    for block_id in input.cfg.block_order() {
        let Some(block) = block_index.get(block_id).copied() else {
            issues.insert(OwnershipAnalysisIssue::new(
                "COMP-OWN-CFG-BLOCK-MISSING",
                None,
                Some(*block_id),
                None,
                "CFG block is missing from the supplied canonical block collection",
            ));
            report.complete = false;
            continue;
        };

        if let Some(events) = input.events.get(block_id) {
            validate_event_order(
                input,
                block,
                events,
                &mut issues,
                &mut report.complete,
            );
        }
    }

    validate_definitions(
        input,
        &mut issues,
        &mut report.complete,
    );

    report.issues = issues.into_iter().collect();

    if input.cfg.entry().is_some() && !input.cfg.is_empty() {
        analyze_dataflow(input, &block_index, &mut report);
    }

    report
}

// =============================================================================
// Unit tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn availability_join_preserves_equal_states() {
        assert_eq!(
            Availability::Available.join(Availability::Available),
            Availability::Available
        );

        assert_eq!(
            Availability::Moved.join(Availability::Moved),
            Availability::Moved
        );

        assert_eq!(
            Availability::Uninitialized.join(
                Availability::Uninitialized
            ),
            Availability::Uninitialized
        );
    }

    #[test]
    fn availability_join_is_conservative() {
        assert_eq!(
            Availability::Available.join(Availability::Moved),
            Availability::Maybe
        );

        assert_eq!(
            Availability::Available.join(
                Availability::Uninitialized
            ),
            Availability::Maybe
        );

        assert_eq!(
            Availability::Maybe.join(Availability::Available),
            Availability::Maybe
        );
    }

    #[test]
    fn shared_borrows_conflict_with_exclusive_borrows() {
        let mut state = ValueState::default();

        let first = OperationId::new(1).expect("nonzero operation ID");
        let second = OperationId::new(2).expect("nonzero operation ID");

        state.active_borrows.insert(
            first,
            BorrowMutability::Shared,
        );

        assert!(active_borrow_conflict(
            &state,
            BorrowMutability::Exclusive
        ));

        assert!(!active_borrow_conflict(
            &state,
            BorrowMutability::Shared
        ));

        state.active_borrows.insert(
            second,
            BorrowMutability::Exclusive,
        );

        assert!(active_borrow_conflict(
            &state,
            BorrowMutability::Shared
        ));
    }

    #[test]
    fn flow_state_join_unions_borrow_information() {
        let value = ValueId::new(1).expect("nonzero value ID");
        let first = OperationId::new(1).expect("nonzero operation ID");
        let second = OperationId::new(2).expect("nonzero operation ID");

        let mut left = FlowState::default();
        let mut right = FlowState::default();

        left.value_mut(value)
            .active_borrows
            .insert(first, BorrowMutability::Shared);

        right.value_mut(value)
            .active_borrows
            .insert(second, BorrowMutability::Exclusive);

        assert!(left.join_from(&right));

        let joined = left.value(value);

        assert_eq!(joined.active_borrows.len(), 2);
        assert!(joined.active_borrows.contains_key(&first));
        assert!(joined.active_borrows.contains_key(&second));
    }
}
