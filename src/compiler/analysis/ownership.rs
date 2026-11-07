
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Compiler — Ownership and Borrow-Flow Analysis.
//!
//! Path: src/compiler/analysis/ownership.rs
//! Edition: Rust 2021
//! Minimum Rust: 1.97
//!
//! ARCHITECTURAL AUTHORITY
//!
//! Source ownership semantics:
//!     grammar/memory/ownership.g4
//!
//! Source lifetime semantics:
//!     grammar/memory/lifetimes.g4
//!
//! Normative semantics:
//!     grammar/specification/semantic-model.md
//!
//! Canonical ownership contracts:
//!     src/zuir/ir/ownership.rs
//!
//! Canonical control-flow graph:
//!     src/zuir/analysis/cfg.rs
//!
//! Canonical structural ownership verifier:
//!     src/zuir/verify/ownership.rs
//!
//! Canonical definition-use index:
//!     src/zuir/ir/use_def.rs
//!
//! PURPOSE
//!
//! Perform deterministic, iterative, CFG-aware ownership analysis.
//!
//! This module validates:
//!
//! - Value availability.
//! - Definitions.
//! - Moves.
//! - Copies.
//! - Drops.
//! - Shared borrows.
//! - Exclusive borrows.
//! - Borrow termination.
//! - Borrow conflicts.
//! - Use after move.
//! - Use before definition.
//! - Conditional availability.
//! - Linear consumption.
//! - Mandatory consumption.
//! - Ownership event consistency.
//! - Control-flow exit conditions.
//!
//! This module consumes canonical semantic information.
//! It does not invent source-level ownership rules.
//!
//! DOES NOT OWN
//!
//! - Source syntax.
//! - Semantic type definitions.
//! - Ownership contract definitions.
//! - Lifetime inference.
//! - CFG construction.
//! - SSA definition indexing.
//! - Physical memory allocation.
//! - Hardware ownership.
//! - Quantum allocation.
//! - Runtime resource management.
//! - Scheduling or routing.
//!
//! SCALABILITY
//!
//! No arbitrary universal limits on:
//!
//! - Values.
//! - Blocks.
//! - Operations.
//! - Borrow relationships.
//! - Control-flow edges.
//! - Ownership events.
//! - Computing domains.
//! - Hardware resources.
//!
//! The analysis uses dynamically sized collections and iterative
//! worklist processing.
//!
//! Actual execution is bounded by available resources.
//! Literal infinity is not physically achievable.
//!
//! UNKNOWN SEMANTICS
//!
//! Unknown ownership semantics never imply unrestricted ownership.
//!
//! Incomplete analysis never implies successful verification.

use std::collections::{
    BTreeMap,
    BTreeSet,
    VecDeque,
};

use crate::zuir::analysis::cfg::ControlFlowGraph;

use crate::zuir::ids::{
    BlockId,
    OperationId,
    ValueId,
};

use crate::zuir::ir::{
    BlockTerminator,
    BorrowMutability,
    OwnershipIndex,
    OwnershipKind,
    OwnershipUse,
    OwnershipUseKind,
    ZuirBlock,
};

// ============================================================================
// API VERSION
// ============================================================================

/// Independent version of the compiler ownership-analysis interface.
pub const OWNERSHIP_ANALYSIS_API_VERSION: u16 = 2;

// ============================================================================
// INPUT
// ============================================================================

/// Additional ownership-flow information required by CFG analysis.
///
/// The canonical ownership index remains the authority for recorded
/// ownership-use events.
///
/// Additional events represent information not currently represented
/// directly by OwnershipUse.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OwnershipFlowEvent {

    /// Canonical ownership event.
    Recorded(OwnershipUse),

    /// A value becomes available at an operation.
    Define {
        value: ValueId,
        operation: OperationId,
    },

    /// Explicit duplication of a value.
    Duplicate {
        value: ValueId,
        operation: OperationId,
    },

    /// Terminate a previously established borrow.
    EndBorrow {
        value: ValueId,
        operation: OperationId,
        borrow_operation: OperationId,
    },
}

impl OwnershipFlowEvent {

    /// Return the affected value.
    #[must_use]
    pub const fn value(&self) -> ValueId {
        match self {
            Self::Recorded(event) => event.value,

            Self::Define { value, .. }
            | Self::Duplicate { value, .. }
            | Self::EndBorrow { value, .. } => *value,
        }
    }

    /// Return the operation associated with the event.
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

/// Complete input to ownership analysis.
///
/// Events must follow actual canonical block operation order.
///
/// Numeric operation-ID sorting must never be used to reconstruct
/// execution order.
pub struct OwnershipAnalysisInput<'a> {

    /// Canonical ownership contracts and recorded events.
    pub ownership: &'a OwnershipIndex,

    /// Canonical CFG.
    pub cfg: &'a ControlFlowGraph,

    /// Canonical ZUIR blocks.
    pub blocks: &'a [ZuirBlock],

    /// Ordered ownership-flow events.
    pub events:
        &'a BTreeMap<BlockId, Vec<OwnershipFlowEvent>>,

    /// Values included in the current analysis scope.
    pub tracked_values: &'a BTreeSet<ValueId>,

    /// Values available at CFG entry.
    pub initial_values: &'a BTreeSet<ValueId>,
}

// ============================================================================
// DIAGNOSTICS
// ============================================================================

/// One deterministic ownership diagnostic.
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

    #[must_use]
    pub const fn code(&self) -> &'static str {
        self.code
    }

    #[must_use]
    pub const fn value(&self) -> Option<ValueId> {
        self.value
    }

    #[must_use]
    pub const fn block(&self) -> Option<BlockId> {
        self.block
    }

    #[must_use]
    pub const fn operation(&self) -> Option<OperationId> {
        self.operation
    }

    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

// ============================================================================
// REPORT
// ============================================================================

/// Result of ownership-flow analysis.
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

    #[must_use]
    pub fn issues(&self) -> &[OwnershipAnalysisIssue] {
        &self.issues
    }

    /// Whether all required analysis information was available.
    #[must_use]
    pub const fn is_complete(&self) -> bool {
        self.complete
    }

    /// True only when analysis is complete and violation-free.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.complete && self.issues.is_empty()
    }

    #[must_use]
    pub fn issue_count(&self) -> usize {
        self.issues.len()
    }

    #[must_use]
    pub const fn reachable_blocks(&self) -> usize {
        self.reachable_blocks
    }

    #[must_use]
    pub const fn events_analyzed(&self) -> usize {
        self.events_analyzed
    }

    #[must_use]
    pub const fn values_analyzed(&self) -> usize {
        self.values_analyzed
    }
}

// ============================================================================
// OWNERSHIP LATTICE
// ============================================================================

/// Abstract availability of one value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Availability {

    /// No valid definition has reached this point.
    Uninitialized,

    /// The value is available.
    Available,

    /// The value has been moved or consumed.
    Moved,

    /// Incoming paths disagree.
    Maybe,
}

impl Availability {

    /// Conservative control-flow join.
    fn join(self, other: Self) -> Self {

        if self == other {
            return self;
        }

        Self::Maybe
    }
}

/// Abstract borrow state.
///
/// If the same borrow origin reaches a join with conflicting
/// mutability information, exclusive access is retained
/// conservatively.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct BorrowFact {
    mutability: BorrowMutability,
}

impl BorrowFact {

    fn join(self, other: Self) -> Self {

        if self.mutability == other.mutability {
            return self;
        }

        Self {
            mutability: BorrowMutability::Exclusive,
        }
    }
}

/// State for one value.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct ValueState {

    availability: Option<Availability>,

    active_borrows: BTreeMap<OperationId, BorrowFact>,
}

impl ValueState {

    fn availability(&self) -> Availability {

        self.availability
            .unwrap_or(Availability::Uninitialized)
    }

    fn join_from(&mut self, other: &Self) -> bool {

        let previous = self.clone();

        self.availability = Some(
            self.availability().join(other.availability())
        );

        for (origin, incoming) in &other.active_borrows {

            self.active_borrows
                .entry(*origin)
                .and_modify(|existing| {
                    *existing = existing.join(*incoming);
                })
                .or_insert(*incoming);
        }

        *self != previous
    }
}

/// State of all values in the analyzed scope.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct FlowState {

    values: BTreeMap<ValueId, ValueState>,
}

impl FlowState {

    fn new(
        tracked: &BTreeSet<ValueId>,
        initial: &BTreeSet<ValueId>,
    ) -> Self {

        let mut state = Self::default();

        for value in tracked {

            let availability =
                if initial.contains(value) {
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

    fn value_mut(
        &mut self,
        value: ValueId,
    ) -> &mut ValueState {

        self.values.entry(value).or_default()
    }

    fn value(
        &self,
        value: ValueId,
    ) -> ValueState {

        self.values
            .get(&value)
            .cloned()
            .unwrap_or_default()
    }

    fn join_from(&mut self, other: &Self) -> bool {

        let previous = self.clone();

        for (value, incoming) in &other.values {

            self.value_mut(*value).join_from(incoming);
        }

        *self != previous
    }
}

// ============================================================================
// EVENT INDEXING
// ============================================================================

type RecordedEventKey = (
    ValueId,
    BlockId,
    OperationId,
    String,
);

fn recorded_event_key(
    event: &OwnershipUse,
) -> RecordedEventKey {

    (
        event.value,
        event.block,
        event.operation,
        format!("{:?}", event.kind),
    )
}

fn increment_event_count(
    counts: &mut BTreeMap<RecordedEventKey, usize>,
    event: &OwnershipUse,
) {

    let count = counts
        .entry(recorded_event_key(event))
        .or_insert(0);

    *count = count.saturating_add(1);
}

// ============================================================================
// DIAGNOSTIC HELPERS
// ============================================================================

fn issue(
    issues: &mut BTreeSet<OwnershipAnalysisIssue>,
    code: &'static str,
    value: Option<ValueId>,
    block: Option<BlockId>,
    operation: Option<OperationId>,
    message: impl Into<String>,
) {

    issues.insert(
        OwnershipAnalysisIssue::new(
            code,
            value,
            block,
            operation,
            message,
        )
    );
}

fn value_issue(
    issues: &mut BTreeSet<OwnershipAnalysisIssue>,
    code: &'static str,
    value: ValueId,
    block: BlockId,
    operation: OperationId,
    message: impl Into<String>,
) {

    issue(
        issues,
        code,
        Some(value),
        Some(block),
        Some(operation),
        message,
    );
}

// ============================================================================
// VALIDATION
// ============================================================================

fn validate_event_coverage(
    input: &OwnershipAnalysisInput<'_>,
    issues: &mut BTreeSet<OwnershipAnalysisIssue>,
    complete: &mut bool,
) {

    let mut expected =
        BTreeMap::<RecordedEventKey, usize>::new();

    let mut actual =
        BTreeMap::<RecordedEventKey, usize>::new();

    // Collect canonical events for this analysis scope.
    for value in input.tracked_values {

        for event in input.ownership.uses_of(*value) {

            if input.cfg.contains_block(event.block) {

                increment_event_count(
                    &mut expected,
                    event,
                );
            }
        }
    }

    // Validate the supplied ordered event stream.
    for (block, events) in input.events {

        if !input.cfg.contains_block(*block) {

            issue(
                issues,
                "COMP-OWN-EVENT-BLOCK-OUTSIDE-CFG",
                None,
                Some(*block),
                None,
                "ownership events refer to a block outside the analyzed CFG",
            );

            *complete = false;
            continue;
        }

        for event in events {

            match event {

                OwnershipFlowEvent::Recorded(recorded) => {

                    if !input.tracked_values.contains(&recorded.value) {

                        value_issue(
                            issues,
                            "COMP-OWN-EVENT-VALUE-UNTRACKED",
                            recorded.value,
                            *block,
                            recorded.operation,
                            "ownership event references a value outside the analyzed scope",
                        );

                        *complete = false;
                        continue;
                    }

                    if recorded.block != *block {

                        value_issue(
                            issues,
                            "COMP-OWN-EVENT-BLOCK-MISMATCH",
                            recorded.value,
                            *block,
                            recorded.operation,
                            "event block differs from its canonical block identity",
                        );

                        *complete = false;
                        continue;
                    }

                    increment_event_count(
                        &mut actual,
                        recorded,
                    );
                }

                _ => {

                    if !input.tracked_values.contains(&event.value()) {

                        issue(
                            issues,
                            "COMP-OWN-EVENT-VALUE-UNTRACKED",
                            Some(event.value()),
                            Some(*block),
                            Some(event.operation()),
                            "additional event references an untracked value",
                        );

                        *complete = false;
                    }
                }
            }
        }
    }

    for (key, expected_count) in &expected {

        let actual_count =
            actual.get(key).copied().unwrap_or(0);

        if actual_count != *expected_count {

            issue(
                issues,
                "COMP-OWN-EVENT-COVERAGE",
                Some(key.0),
                Some(key.1),
                Some(key.2),
                format!(
                    "expected {expected_count} canonical events, received {actual_count}"
                ),
            );

            *complete = false;
        }
    }

    for (key, actual_count) in &actual {

        let expected_count =
            expected.get(key).copied().unwrap_or(0);

        if actual_count > expected_count {

            issue(
                issues,
                "COMP-OWN-UNREGISTERED-EVENT",
                Some(key.0),
                Some(key.1),
                Some(key.2),
                "flow input contains an event absent from the canonical ownership index",
            );

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

        issue(
            issues,
            "COMP-OWN-INVALID-INDEX",
            None,
            None,
            None,
            format!("canonical ownership index is invalid: {error}"),
        );

        *complete = false;
    }

    if input.cfg.is_empty() {

        issue(
            issues,
            "COMP-OWN-EMPTY-CFG",
            None,
            None,
            None,
            "ownership analysis requires a nonempty CFG",
        );

        *complete = false;
    }

    if input.cfg.entry().is_none() {

        issue(
            issues,
            "COMP-OWN-MISSING-CFG-ENTRY",
            None,
            None,
            None,
            "ownership analysis requires an explicit CFG entry",
        );

        *complete = false;
    }

    for value in input.initial_values {

        if !input.tracked_values.contains(value) {

            issue(
                issues,
                "COMP-OWN-INITIAL-VALUE-UNTRACKED",
                Some(*value),
                None,
                None,
                "initial value is not part of the tracked-value set",
            );

            *complete = false;
        }
    }

    for value in input.tracked_values {

        match input.ownership.contract(*value) {

            Some(contract) => {

                if let Err(error) = contract.validate() {

                    issue(
                        issues,
                        "COMP-OWN-INVALID-CONTRACT",
                        Some(*value),
                        None,
                        None,
                        format!("invalid ownership contract: {error}"),
                    );

                    *complete = false;
                }

                if matches!(
                    contract.kind,
                    OwnershipKind::Extension(_)
                ) {

                    issue(
                        issues,
                        "COMP-OWN-UNKNOWN-CONTRACT-SEMANTICS",
                        Some(*value),
                        None,
                        None,
                        "extension ownership semantics are not registered with this analysis",
                    );

                    *complete = false;
                }
            }

            None => {

                issue(
                    issues,
                    "COMP-OWN-MISSING-CONTRACT",
                    Some(*value),
                    None,
                    None,
                    "tracked value has no canonical ownership contract",
                );

                *complete = false;
            }
        }
    }

    validate_event_coverage(
        input,
        issues,
        complete,
    );
}

// ============================================================================
// BLOCK INDEXING
// ============================================================================

fn index_blocks<'a>(
    blocks: &'a [ZuirBlock],
    cfg: &ControlFlowGraph,
    issues: &mut BTreeSet<OwnershipAnalysisIssue>,
    complete: &mut bool,
) -> BTreeMap<BlockId, &'a ZuirBlock> {

    let mut index = BTreeMap::new();

    for block in blocks {

        if index.insert(block.id(), block).is_some() {

            issue(
                issues,
                "COMP-OWN-DUPLICATE-BLOCK",
                None,
                Some(block.id()),
                None,
                "block identity occurs more than once",
            );

            *complete = false;
        }
    }

    for block_id in cfg.block_order() {

        if !index.contains_key(block_id) {

            issue(
                issues,
                "COMP-OWN-CFG-BLOCK-MISSING",
                None,
                Some(*block_id),
                None,
                "CFG block does not exist in the supplied canonical block collection",
            );

            *complete = false;
        }
    }

    index
}

// ============================================================================
// EVENT ORDER
// ============================================================================

fn validate_event_order(
    block: &ZuirBlock,
    events: &[OwnershipFlowEvent],
    issues: &mut BTreeSet<OwnershipAnalysisIssue>,
    complete: &mut bool,
) {

    let mut positions =
        BTreeMap::<OperationId, usize>::new();

    for (position, operation) in
        block.operations().iter().enumerate()
    {

        if positions.insert(*operation, position).is_some() {

            issue(
                issues,
                "COMP-OWN-DUPLICATE-OPERATION-IN-BLOCK",
                None,
                Some(block.id()),
                Some(*operation),
                "operation identity occurs more than once in a block",
            );

            *complete = false;
        }
    }

    let mut previous = None;

    for event in events {

        let operation = event.operation();

        let Some(position) = positions.get(&operation).copied()
        else {

            issue(
                issues,
                "COMP-OWN-EVENT-OPERATION-NOT-IN-BLOCK",
                Some(event.value()),
                Some(block.id()),
                Some(operation),
                "ownership event refers to an operation absent from its block",
            );

            *complete = false;
            continue;
        };

        if let Some(previous_position) = previous {

            if position < previous_position {

                issue(
                    issues,
                    "COMP-OWN-EVENT-ORDER",
                    Some(event.value()),
                    Some(block.id()),
                    Some(operation),
                    "ownership events do not follow canonical operation order",
                );

                *complete = false;
            }
        }

        previous = Some(position);
    }
}

// ============================================================================
// AVAILABILITY
// ============================================================================

fn check_available(
    state: &ValueState,
    value: ValueId,
    block: BlockId,
    operation: OperationId,
    issues: &mut BTreeSet<OwnershipAnalysisIssue>,
) -> bool {

    match state.availability() {

        Availability::Available => true,

        Availability::Uninitialized => {

            value_issue(
                issues,
                "COMP-OWN-USE-BEFORE-DEFINITION",
                value,
                block,
                operation,
                "value is not initialized on this control-flow path",
            );

            false
        }

        Availability::Moved => {

            value_issue(
                issues,
                "COMP-OWN-USE-AFTER-MOVE",
                value,
                block,
                operation,
                "value was previously moved or consumed",
            );

            false
        }

        Availability::Maybe => {

            value_issue(
                issues,
                "COMP-OWN-CONDITIONAL-AVAILABILITY",
                value,
                block,
                operation,
                "value is not available on every incoming control-flow path",
            );

            false
        }
    }
}

// ============================================================================
// BORROW VALIDATION
// ============================================================================

fn has_exclusive_borrow(
    state: &ValueState,
) -> bool {

    state.active_borrows
        .values()
        .any(|borrow| {
            borrow.mutability == BorrowMutability::Exclusive
        })
}

fn borrow_conflict(
    state: &ValueState,
    requested: BorrowMutability,
) -> bool {

    match requested {

        BorrowMutability::Shared => {
            has_exclusive_borrow(state)
        }

        BorrowMutability::Exclusive => {
            !state.active_borrows.is_empty()
        }
    }
}

fn begin_borrow(
    state: &mut ValueState,
    value: ValueId,
    block: BlockId,
    operation: OperationId,
    requested: BorrowMutability,
    allowed: bool,
    issues: &mut BTreeSet<OwnershipAnalysisIssue>,
) {

    if !allowed {

        value_issue(
            issues,
            "COMP-OWN-BORROW-CAPABILITY",
            value,
            block,
            operation,
            "ownership contract prohibits the requested borrow",
        );

        return;
    }

    if borrow_conflict(state, requested) {

        value_issue(
            issues,
            "COMP-OWN-BORROW-CONFLICT",
            value,
            block,
            operation,
            "requested borrow conflicts with an active borrow",
        );

        return;
    }

    if state.active_borrows.contains_key(&operation) {

        value_issue(
            issues,
            "COMP-OWN-DUPLICATE-BORROW-ORIGIN",
            value,
            block,
            operation,
            "borrow origin already exists for this value",
        );

        return;
    }

    state.active_borrows.insert(
        operation,
        BorrowFact {
            mutability: requested,
        },
    );
}

// ============================================================================
// TRANSFER
// ============================================================================

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

    let Some(contract) =
        input.ownership.contract(value)
    else {
        return;
    };

    let capabilities = contract.capabilities;

    let value_state = state.value_mut(value);

    match event {

        OwnershipFlowEvent::Define { .. } => {

            if !value_state.active_borrows.is_empty() {

                value_issue(
                    issues,
                    "COMP-OWN-REDEFINITION-WHILE-BORROWED",
                    value,
                    block,
                    operation,
                    "cannot redefine a value while a borrow remains active",
                );
            }

            value_state.availability =
                Some(Availability::Available);
        }

        OwnershipFlowEvent::Duplicate { .. } => {

            if check_available(
                value_state,
                value,
                block,
                operation,
                issues,
            ) {

                if !capabilities.may_copy {

                    value_issue(
                        issues,
                        "COMP-OWN-COPY-PROHIBITED",
                        value,
                        block,
                        operation,
                        "ownership contract prohibits copying",
                    );
                }

                if has_exclusive_borrow(value_state) {

                    value_issue(
                        issues,
                        "COMP-OWN-COPY-DURING-EXCLUSIVE-BORROW",
                        value,
                        block,
                        operation,
                        "cannot copy through the owner while an exclusive borrow is active",
                    );
                }
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

                value_issue(
                    issues,
                    "COMP-OWN-END-UNKNOWN-BORROW",
                    value,
                    block,
                    operation,
                    format!(
                        "borrow origin {borrow_operation} is unknown or already ended"
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
                    ) && has_exclusive_borrow(value_state) {

                        value_issue(
                            issues,
                            "COMP-OWN-READ-DURING-EXCLUSIVE-BORROW",
                            value,
                            block,
                            operation,
                            "cannot access the owner while an exclusive borrow is active",
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
                    ) && !value_state.active_borrows.is_empty() {

                        value_issue(
                            issues,
                            "COMP-OWN-WRITE-WHILE-BORROWED",
                            value,
                            block,
                            operation,
                            "cannot mutate the owner while a borrow is active",
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

                        if !capabilities.may_move {

                            value_issue(
                                issues,
                                "COMP-OWN-MOVE-PROHIBITED",
                                value,
                                block,
                                operation,
                                "ownership contract prohibits moving",
                            );
                        } else if !value_state.active_borrows.is_empty() {

                            value_issue(
                                issues,
                                "COMP-OWN-MOVE-WHILE-BORROWED",
                                value,
                                block,
                                operation,
                                "cannot move a value while a borrow is active",
                            );
                        } else {

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

                        if !capabilities.may_drop {

                            value_issue(
                                issues,
                                "COMP-OWN-DROP-PROHIBITED",
                                value,
                                block,
                                operation,
                                "ownership contract prohibits implicit dropping",
                            );
                        } else if !value_state.active_borrows.is_empty() {

                            value_issue(
                                issues,
                                "COMP-OWN-DROP-WHILE-BORROWED",
                                value,
                                block,
                                operation,
                                "cannot drop a value while a borrow remains active",
                            );
                        } else {

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

                        begin_borrow(
                            value_state,
                            value,
                            block,
                            operation,
                            BorrowMutability::Shared,
                            capabilities.may_borrow_shared,
                            issues,
                        );
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

                        begin_borrow(
                            value_state,
                            value,
                            block,
                            operation,
                            BorrowMutability::Exclusive,
                            capabilities.may_borrow_exclusive,
                            issues,
                        );
                    }
                }

                OwnershipUseKind::Extension(_) => {

                    value_issue(
                        issues,
                        "COMP-OWN-UNKNOWN-EVENT-SEMANTICS",
                        value,
                        block,
                        operation,
                        "extension ownership event has no registered transfer semantics",
                    );

                    *complete = false;
                }
            }
        }
    }
}

// ============================================================================
// EXIT ANALYSIS
// ============================================================================

fn is_normal_exit(block: &ZuirBlock) -> bool {

    matches!(
        block.terminator(),
        BlockTerminator::Return { .. }
    )
}

fn is_unreachable_exit(block: &ZuirBlock) -> bool {

    matches!(
        block.terminator(),
        BlockTerminator::Unreachable
    )
}

fn validate_exit(
    input: &OwnershipAnalysisInput<'_>,
    block: BlockId,
    state: &FlowState,
    issues: &mut BTreeSet<OwnershipAnalysisIssue>,
) {

    for value in input.tracked_values {

        let Some(contract) =
            input.ownership.contract(*value)
        else {
            continue;
        };

        let value_state = state.value(*value);

        if !value_state.active_borrows.is_empty() {

            issue(
                issues,
                "COMP-OWN-BORROW-ESCAPES-EXIT",
                Some(*value),
                Some(block),
                None,
                "borrow remains active at a normal function exit",
            );
        }

        if !contract.capabilities.must_consume {
            continue;
        }

        match value_state.availability() {

            Availability::Moved => {}

            Availability::Available => {

                issue(
                    issues,
                    "COMP-OWN-MUST-CONSUME",
                    Some(*value),
                    Some(block),
                    None,
                    "value requiring consumption remains available at function exit",
                );
            }

            Availability::Maybe => {

                issue(
                    issues,
                    "COMP-OWN-MUST-CONSUME-PATH",
                    Some(*value),
                    Some(block),
                    None,
                    "mandatory consumption is not established on every incoming path",
                );
            }

            Availability::Uninitialized => {

                issue(
                    issues,
                    "COMP-OWN-MUST-CONSUME-UNAVAILABLE",
                    Some(*value),
                    Some(block),
                    None,
                    "mandatory-consumption value is unavailable at function exit",
                );
            }
        }
    }
}

// ============================================================================
// DATAFLOW
// ============================================================================

fn analyze_dataflow(
    input: &OwnershipAnalysisInput<'_>,
    blocks: &BTreeMap<BlockId, &ZuirBlock>,
    issues: &mut BTreeSet<OwnershipAnalysisIssue>,
    report: &mut OwnershipAnalysisReport,
) {

    let Some(entry) = input.cfg.entry() else {
        return;
    };

    let mut incoming =
        BTreeMap::<BlockId, FlowState>::new();

    let mut outgoing =
        BTreeMap::<BlockId, FlowState>::new();

    incoming.insert(
        entry,
        FlowState::new(
            input.tracked_values,
            input.initial_values,
        ),
    );

    let mut pending = VecDeque::new();

    let mut queued = BTreeSet::new();

    pending.push_back(entry);
    queued.insert(entry);

    while let Some(block_id) = pending.pop_front() {

        queued.remove(&block_id);

        let Some(block) = blocks.get(&block_id).copied()
        else {
            continue;
        };

        let Some(mut state) =
            incoming.get(&block_id).cloned()
        else {
            continue;
        };

        if let Some(events) =
            input.events.get(&block_id)
        {

            for event in events {

                transfer_event(
                    input,
                    &mut state,
                    block_id,
                    event,
                    issues,
                    &mut report.complete,
                );
            }
        }

        let changed = outgoing
            .get(&block_id)
            .map_or(true, |previous| previous != &state);

        if !changed {
            continue;
        }

        outgoing.insert(block_id, state.clone());

        for edge in input.cfg.successors(block_id) {

            let successor = edge.target();

            if !input.cfg.contains_block(successor) {

                issue(
                    issues,
                    "COMP-OWN-CFG-INVALID-SUCCESSOR",
                    None,
                    Some(block_id),
                    None,
                    format!(
                        "CFG successor {successor} is not present in the graph"
                    ),
                );

                report.complete = false;
                continue;
            }

            // Block-argument ownership transfer is not represented by
            // OwnershipUse, which currently associates events with
            // OperationId rather than CFG edges.
            //
            // Never silently claim completeness for an edge that carries
            // SSA arguments.
            if !edge.arguments().is_empty() {

                issue(
                    issues,
                    "COMP-OWN-EDGE-TRANSFER-UNMODELED",
                    None,
                    Some(block_id),
                    None,
                    "CFG edge carries SSA block arguments, but explicit edge ownership transfer facts are not available",
                );

                report.complete = false;
            }

            let changed = match incoming.get_mut(&successor) {

                Some(existing) => {
                    existing.join_from(&state)
                }

                None => {
                    incoming.insert(
                        successor,
                        state.clone(),
                    );

                    true
                }
            };

            if changed && queued.insert(successor) {

                pending.push_back(successor);
            }
        }
    }

    report.reachable_blocks = incoming.len();

    // Count events once per reachable block rather than once per
    // worklist iteration.
    for block in incoming.keys() {

        if let Some(events) = input.events.get(block) {

            report.events_analyzed =
                report.events_analyzed.saturating_add(events.len());
        }
    }

    // Check normal exits after reaching the fixed point.
    for (block_id, state) in &outgoing {

        let Some(block) = blocks.get(block_id).copied()
        else {
            continue;
        };

        if is_unreachable_exit(block) {
            continue;
        }

        if input.cfg.successors(*block_id).next().is_some() {
            continue;
        }

        if is_normal_exit(block) {

            validate_exit(
                input,
                *block_id,
                state,
                issues,
            );

        } else {

            issue(
                issues,
                "COMP-OWN-UNCLASSIFIED-EXIT",
                None,
                Some(*block_id),
                None,
                "reachable terminal block has no recognized normal-return or unreachable semantics",
            );

            report.complete = false;
        }
    }
}

// ============================================================================
// STATIC DEFINITION VALIDATION
// ============================================================================

fn validate_definitions(
    input: &OwnershipAnalysisInput<'_>,
    issues: &mut BTreeSet<OwnershipAnalysisIssue>,
    complete: &mut bool,
) {

    let mut definitions =
        BTreeMap::<ValueId, usize>::new();

    for (block, events) in input.events {

        if !input.cfg.contains_block(*block) {
            continue;
        }

        for event in events {

            if let OwnershipFlowEvent::Define { value, .. } = event {

                if input.tracked_values.contains(value) {

                    let count =
                        definitions.entry(*value).or_insert(0);

                    *count = count.saturating_add(1);
                }
            }
        }
    }

    for value in input.tracked_values {

        let count =
            definitions.get(value).copied().unwrap_or(0);

        if input.initial_values.contains(value) {

            if count > 0 {

                issue(
                    issues,
                    "COMP-OWN-ENTRY-VALUE-REDEFINED",
                    Some(*value),
                    None,
                    None,
                    "entry value also has a definition inside the analyzed CFG",
                );
            }

            continue;
        }

        match count {

            0 => {

                issue(
                    issues,
                    "COMP-OWN-MISSING-DEFINITION",
                    Some(*value),
                    None,
                    None,
                    "tracked non-entry value has no definition event",
                );

                *complete = false;
            }

            1 => {}

            _ => {

                issue(
                    issues,
                    "COMP-OWN-MULTIPLE-DEFINITIONS",
                    Some(*value),
                    None,
                    None,
                    "tracked SSA value has multiple static definitions",
                );

                *complete = false;
            }
        }
    }
}

// ============================================================================
// PUBLIC ENTRY POINT
// ============================================================================

/// Analyze ownership across a validated canonical CFG.
///
/// The caller must supply:
///
/// - A validated OwnershipIndex.
/// - A canonical CFG.
/// - The canonical blocks used by the CFG.
/// - Complete ordered ownership events.
/// - Tracked values.
/// - Entry values.
/// - Authoritative borrow-ending information.
///
/// The analysis is iterative and deterministic.
///
/// No arbitrary iteration ceiling is imposed.
///
/// Unknown ownership semantics or missing information prevent a
/// successful completeness result.
///
/// IMPORTANT:
///
/// This analysis does not independently infer source-level lifetimes.
/// Lifetime information belongs to frontend semantic analysis.
#[must_use]
pub fn analyze_ownership(
    input: &OwnershipAnalysisInput<'_>,
) -> OwnershipAnalysisReport {

    let mut report = OwnershipAnalysisReport::new();

    report.values_analyzed =
        input.tracked_values.len();

    let mut issues =
        BTreeSet::<OwnershipAnalysisIssue>::new();

    validate_input(
        input,
        &mut issues,
        &mut report.complete,
    );

    let blocks = index_blocks(
        input.blocks,
        input.cfg,
        &mut issues,
        &mut report.complete,
    );

    for block_id in input.cfg.block_order() {

        let Some(block) = blocks.get(block_id).copied()
        else {
            continue;
        };

        if let Some(events) =
            input.events.get(block_id)
        {

            validate_event_order(
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

    if input.cfg.entry().is_some()
        && !input.cfg.is_empty()
    {

        analyze_dataflow(
            input,
            &blocks,
            &mut issues,
            &mut report,
        );
    }

    // Preserve diagnostics from every analysis phase.
    report.issues = issues.into_iter().collect();

    report
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn availability_join_preserves_equal_states() {

        assert_eq!(
            Availability::Available.join(
                Availability::Available
            ),
            Availability::Available
        );

        assert_eq!(
            Availability::Moved.join(
                Availability::Moved
            ),
            Availability::Moved
        );
    }

    #[test]
    fn availability_join_is_conservative() {

        assert_eq!(
            Availability::Available.join(
                Availability::Moved
            ),
            Availability::Maybe
        );

        assert_eq!(
            Availability::Available.join(
                Availability::Uninitialized
            ),
            Availability::Maybe
        );
    }

    #[test]
    fn exclusive_borrow_conflicts_with_shared_borrow() {

        let mut state = ValueState::default();

        let origin = OperationId::new(1)
            .expect("valid operation ID");

        state.active_borrows.insert(
            origin,
            BorrowFact {
                mutability: BorrowMutability::Exclusive,
            },
        );

        assert!(
            borrow_conflict(
                &state,
                BorrowMutability::Shared
            )
        );
    }

    #[test]
    fn shared_borrows_can_coexist() {

        let mut state = ValueState::default();

        let first = OperationId::new(1)
            .expect("valid operation ID");

        let second = OperationId::new(2)
            .expect("valid operation ID");

        state.active_borrows.insert(
            first,
            BorrowFact {
                mutability: BorrowMutability::Shared,
            },
        );

        state.active_borrows.insert(
            second,
            BorrowFact {
                mutability: BorrowMutability::Shared,
            },
        );

        assert!(
            !borrow_conflict(
                &state,
                BorrowMutability::Shared
            )
        );

        assert!(
            borrow_conflict(
                &state,
                BorrowMutability::Exclusive
            )
        );
    }

    #[test]
    fn conflicting_borrow_facts_join_conservatively() {

        let shared = BorrowFact {
            mutability: BorrowMutability::Shared,
        };

        let exclusive = BorrowFact {
            mutability: BorrowMutability::Exclusive,
        };

        assert_eq!(
            shared.join(exclusive).mutability,
            BorrowMutability::Exclusive
        );
    }

    #[test]
    fn joining_flow_states_preserves_all_borrow_origins() {

        let value = ValueId::new(1)
            .expect("valid value ID");

        let first = OperationId::new(1)
            .expect("valid operation ID");

        let second = OperationId::new(2)
            .expect("valid operation ID");

        let mut left = FlowState::default();
        let mut right = FlowState::default();

        left.value_mut(value).active_borrows.insert(
            first,
            BorrowFact {
                mutability: BorrowMutability::Shared,
            },
        );

        right.value_mut(value).active_borrows.insert(
            second,
            BorrowFact {
                mutability: BorrowMutability::Exclusive,
            },
        );

        assert!(left.join_from(&right));

        let joined = left.value(value);

        assert_eq!(
            joined.active_borrows.len(),
            2
        );
    }

    #[test]
    fn ending_a_borrow_removes_only_its_origin() {

        let first = OperationId::new(1)
            .expect("valid operation ID");

        let second = OperationId::new(2)
            .expect("valid operation ID");

        let mut state = ValueState::default();

        state.active_borrows.insert(
            first,
            BorrowFact {
                mutability: BorrowMutability::Shared,
            },
        );

        state.active_borrows.insert(
            second,
            BorrowFact {
                mutability: BorrowMutability::Shared,
            },
        );

        assert!(state.active_borrows.remove(&first).is_some());

        assert!(
            state.active_borrows.contains_key(&second)
        );
    }
}
