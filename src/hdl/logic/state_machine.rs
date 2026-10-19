#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani HDL — State-Machine Contracts
//!
//! Path: `src/hdl/logic/state_machine.rs`
//! Rust baseline: 1.97 or later.
//! Edition: 2021.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Represent and structurally validate target-independent HDL state machines.
//! This is a semantic integration contract, not an AST, parser, or IR.
//!
//! ## Authority
//!
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/state_machines.g4`
//! - `grammar/hdl/sequential.g4`
//! - `grammar/hdl/processes.g4`
//! - `grammar/hdl/reset.g4`
//! - `src/frontend/ast/node/`
//! - `src/frontend/semantic/domains/hdl.rs`
//! - `src/hdl/logic/sequential.rs`
//! - `src/hdl/logic/process.rs`
//!
//! The grammar owns syntax. The canonical AST owns syntax-node identity and
//! source spans. The semantic layer owns name resolution, expression typing,
//! guard legality, action legality, and language-level correctness.
//!
//! ## Owns
//!
//! - Logical state and transition contracts.
//! - Initial and terminal state declarations.
//! - References to canonical guard and action AST nodes.
//! - Explicit reset intent and clock references.
//! - Structural validation, reachability, and conservative findings about
//!   potentially overlapping transitions.
//! - Explicit distinctions between invalid and inconclusive analyses.
//!
//! ## Does not own
//!
//! - Parsing or a replacement AST.
//! - General expressions, types, source spans, or semantic name resolution.
//! - Clock/reset declaration ownership or sequential event validation.
//! - Proof of guard exclusivity or arbitrary expression equivalence.
//! - Physical state encoding, register allocation, synthesis, scheduling,
//!   placement, routing, or hardware discovery.
//! - Simulation execution or runtime event scheduling.
//! - HDLIR structures, builders, verifiers, passes, serialization, or
//!   lowering. All HDLIR implementation remains under `src/hdl/hdlir/`.
//!
//! ## Integration
//!
//! An AST adapter creates these contracts using canonical `HdlId` identities
//! and `NodeId` references. It must preserve source ordering and source spans
//! in the canonical AST; this model does not copy or replace those spans.
//!
//! The semantic adapter resolves state references and checks guard/action
//! expressions through the existing semantic services. Clock and reset
//! declarations must be validated by their owning semantic contracts.
//!
//! ## Scalability
//!
//! No language-level maximum is imposed on states, transitions, guards,
//! actions, or machines. Reachability uses an iterative worklist rather than
//! recursive traversal. Collections grow subject to available resources.
//! Callers may impose configurable operational budgets; budget exhaustion
//! must be reported separately and must never imply successful validation.
//!
//! ## Completion boundary
//!
//! This file provides the state-machine model and local structural analysis.
//! Full production integration additionally requires module registration,
//! AST adaptation, semantic checking, diagnostics integration, and repository
//! regression tests. Structural validation alone does not prove HDL legality.

use core::fmt;
use std::collections::{BTreeMap, BTreeSet, VecDeque};

use crate::frontend::ast::node::node_id::NodeId;
use crate::frontend::semantic::domains::hdl::HdlId;

/// Version of the state-machine contract API.
///
/// Independent of the language version and any future HDLIR schema version.
pub const HDL_STATE_MACHINE_CONTRACT_VERSION: u16 = 1;

/// Logical classification of a declared state.
///
/// This classification does not select physical state encoding.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum StateKind {
    /// An ordinary state.
    Normal,

    /// A terminal state, intended to have no outgoing transitions.
    Terminal,
}

/// Logical reset behavior declared for a state machine.
///
/// This describes source-level intent only. The reset declaration itself,
/// its polarity, and its synchronous/asynchronous semantics remain owned by
/// the canonical reset and sequential semantic contracts.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ResetMode {
    /// Reset is intended to take effect synchronously.
    Synchronous,

    /// Reset is intended to take effect asynchronously.
    Asynchronous,
}

/// Explicit reset intent for a state machine.
///
/// `signal` and `initial_state` are canonical HDL declaration identities.
/// This object does not establish that the signal is a valid reset source.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StateMachineReset {
    signal: HdlId,
    initial_state: HdlId,
    mode: ResetMode,
}

impl StateMachineReset {
    /// Construct a reset-intent contract.
    ///
    /// Declaration resolution and compatibility with the enclosing clock
    /// domain must be checked by the semantic integration layer.
    #[must_use]
    pub fn new(
        signal: HdlId,
        initial_state: HdlId,
        mode: ResetMode,
    ) -> Self {
        Self {
            signal,
            initial_state,
            mode,
        }
    }

    /// Return the canonical reset declaration identity.
    #[must_use]
    pub fn signal(&self) -> &HdlId {
        &self.signal
    }

    /// Return the state intended to be entered during reset.
    #[must_use]
    pub fn initial_state(&self) -> &HdlId {
        &self.initial_state
    }

    /// Return the declared reset mode.
    #[must_use]
    pub const fn mode(&self) -> ResetMode {
        self.mode
    }
}

/// A declared logical state.
///
/// Entry and exit actions reference canonical AST nodes. Their expression
/// meaning and legality are determined by semantic analysis.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StateDeclaration {
    id: HdlId,
    kind: StateKind,
    entry_actions: Vec<NodeId>,
    exit_actions: Vec<NodeId>,
}

impl StateDeclaration {
    /// Construct a state declaration contract.
    ///
    /// The caller must supply the canonical identity of the declaration.
    /// This constructor does not resolve that identity against an AST.
    #[must_use]
    pub fn new(
        id: HdlId,
        kind: StateKind,
        entry_actions: Vec<NodeId>,
        exit_actions: Vec<NodeId>,
    ) -> Self {
        Self {
            id,
            kind,
            entry_actions,
            exit_actions,
        }
    }

    /// Return the canonical identity of this state.
    #[must_use]
    pub fn id(&self) -> &HdlId {
        &self.id
    }

    /// Return the logical classification of this state.
    #[must_use]
    pub const fn kind(&self) -> StateKind {
        self.kind
    }

    /// Return entry-action AST references in source order.
    #[must_use]
    pub fn entry_actions(&self) -> &[NodeId] {
        &self.entry_actions
    }

    /// Return exit-action AST references in source order.
    #[must_use]
    pub fn exit_actions(&self) -> &[NodeId] {
        &self.exit_actions
    }
}

/// A logical transition between declared states.
///
/// `event` is an optional source-level event label. Its spelling and
/// interpretation are governed by the parser and semantic adapter.
/// `guard` and `actions` refer to canonical AST nodes rather than defining
/// a second expression or statement representation.
///
/// A default transition is represented explicitly; it is not inferred from
/// the ordering of transitions.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StateTransition {
    source: HdlId,
    target: HdlId,
    event: Option<String>,
    guard: Option<NodeId>,
    actions: Vec<NodeId>,
    is_default: bool,
}

impl StateTransition {
    /// Construct a transition contract.
    ///
    /// This constructor preserves the supplied data. Structural validation
    /// checks state references and local consistency afterward.
    #[must_use]
    pub fn new(
        source: HdlId,
        target: HdlId,
        event: Option<String>,
        guard: Option<NodeId>,
        actions: Vec<NodeId>,
        is_default: bool,
    ) -> Self {
        Self {
            source,
            target,
            event,
            guard,
            actions,
            is_default,
        }
    }

    /// Return the source-state identity.
    #[must_use]
    pub fn source(&self) -> &HdlId {
        &self.source
    }

    /// Return the target-state identity.
    #[must_use]
    pub fn target(&self) -> &HdlId {
        &self.target
    }

    /// Return the optional event label.
    #[must_use]
    pub fn event(&self) -> Option<&str> {
        self.event.as_deref()
    }

    /// Return the optional guard AST reference.
    #[must_use]
    pub fn guard(&self) -> Option<&NodeId> {
        self.guard.as_ref()
    }

    /// Return action AST references in source order.
    #[must_use]
    pub fn actions(&self) -> &[NodeId] {
        &self.actions
    }

    /// Return whether this transition is explicitly declared as default.
    #[must_use]
    pub const fn is_default(&self) -> bool {
        self.is_default
    }
}

/// A target-independent logical state-machine declaration.
///
/// State and transition order is preserved as supplied by the AST adapter.
/// Identity lookup and analysis are deterministic independently of hash-map
/// iteration order.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StateMachine {
    id: HdlId,
    initial_state: HdlId,
    states: Vec<StateDeclaration>,
    transitions: Vec<StateTransition>,
    clock: Option<HdlId>,
    reset: Option<StateMachineReset>,
}

impl StateMachine {
    /// Construct a state-machine contract.
    ///
    /// Construction preserves potentially invalid input so validation can
    /// report multiple structural problems in one pass.
    ///
    /// The initial state must be declared, and all transition endpoints
    /// must refer to declared states. Those conditions are checked by
    /// [`validate_state_machine`].
    #[must_use]
    pub fn new(
        id: HdlId,
        initial_state: HdlId,
        states: Vec<StateDeclaration>,
        transitions: Vec<StateTransition>,
        clock: Option<HdlId>,
        reset: Option<StateMachineReset>,
    ) -> Self {
        Self {
            id,
            initial_state,
            states,
            transitions,
            clock,
            reset,
        }
    }

    /// Return the canonical state-machine identity.
    #[must_use]
    pub fn id(&self) -> &HdlId {
        &self.id
    }

    /// Return the declared initial-state identity.
    #[must_use]
    pub fn initial_state(&self) -> &HdlId {
        &self.initial_state
    }

    /// Return states in source declaration order.
    #[must_use]
    pub fn states(&self) -> &[StateDeclaration] {
        &self.states
    }

    /// Return transitions in source declaration order.
    #[must_use]
    pub fn transitions(&self) -> &[StateTransition] {
        &self.transitions
    }

    /// Return the optional canonical clock declaration identity.
    #[must_use]
    pub fn clock(&self) -> Option<&HdlId> {
        self.clock.as_ref()
    }

    /// Return optional reset intent.
    #[must_use]
    pub fn reset(&self) -> Option<&StateMachineReset> {
        self.reset.as_ref()
    }
}

/// Structural errors that prevent a state-machine contract from being valid.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[non_exhaustive]
pub enum StateMachineError {
    /// The same state identity is declared more than once.
    DuplicateState {
        /// Repeated state identity.
        state: HdlId,
    },

    /// The declared initial state is not present in the state list.
    UndeclaredInitialState {
        /// Missing initial-state identity.
        state: HdlId,
    },

    /// A transition refers to an undeclared source state.
    UndeclaredTransitionSource {
        /// Transition position in source order.
        transition_index: usize,
        /// Missing source identity.
        state: HdlId,
    },

    /// A transition refers to an undeclared target state.
    UndeclaredTransitionTarget {
        /// Transition position in source order.
        transition_index: usize,
        /// Missing target identity.
        state: HdlId,
    },

    /// A reset intent refers to an undeclared state.
    UndeclaredResetState {
        /// Missing reset-state identity.
        state: HdlId,
    },

    /// A terminal state has an outgoing transition.
    TerminalStateHasOutgoingTransition {
        /// Terminal state identity.
        state: HdlId,
        /// Outgoing transition index.
        transition_index: usize,
    },

    /// A default transition is also assigned an explicit event.
    DefaultTransitionHasEvent {
        /// Transition index.
        transition_index: usize,
    },

    /// A default transition has a guard.
    ///
    /// Default-transition guard semantics must be specified explicitly
    /// before this combination can be accepted as portable behavior.
    GuardedDefaultTransition {
        /// Transition index.
        transition_index: usize,
    },

    /// The same transition key occurs more than once.
    DuplicateTransition {
        /// First transition index.
        first_index: usize,
        /// Repeated transition index.
        duplicate_index: usize,
    },

    /// The state machine has more than one default transition for a source.
    MultipleDefaultTransitions {
        /// Source state identity.
        state: HdlId,
        /// First default transition index.
        first_index: usize,
        /// Additional default transition index.
        duplicate_index: usize,
    },
}

impl fmt::Display for StateMachineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateState { state } => {
                write!(f, "duplicate state declaration `{state}`")
            }
            Self::UndeclaredInitialState { state } => {
                write!(f, "initial state `{state}` is not declared")
            }
            Self::UndeclaredTransitionSource {
                transition_index,
                state,
            } => write!(
                f,
                "transition {transition_index} has undeclared source `{state}`"
            ),
            Self::UndeclaredTransitionTarget {
                transition_index,
                state,
            } => write!(
                f,
                "transition {transition_index} has undeclared target `{state}`"
            ),
            Self::UndeclaredResetState { state } => {
                write!(f, "reset state `{state}` is not declared")
            }
            Self::TerminalStateHasOutgoingTransition {
                state,
                transition_index,
            } => write!(
                f,
                "terminal state `{state}` has outgoing transition \
                 {transition_index}"
            ),
            Self::DefaultTransitionHasEvent { transition_index } => write!(
                f,
                "default transition {transition_index} cannot also \
                 declare an explicit event"
            ),
            Self::GuardedDefaultTransition { transition_index } => write!(
                f,
                "default transition {transition_index} cannot have a guard \
                 until guarded-default semantics are specified"
            ),
            Self::DuplicateTransition {
                first_index,
                duplicate_index,
            } => write!(
                f,
                "transition {duplicate_index} duplicates transition \
                 {first_index}"
            ),
            Self::MultipleDefaultTransitions {
                state,
                first_index,
                duplicate_index,
            } => write!(
                f,
                "state `{state}` has default transitions {first_index} \
                 and {duplicate_index}"
            ),
        }
    }
}

impl std::error::Error for StateMachineError {}

/// Non-fatal findings from state-machine analysis.
///
/// A finding is not automatically a source-language error. In particular,
/// unreachable states and potentially overlapping guards may be warnings
/// depending on the active language policy.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[non_exhaustive]
pub enum StateMachineFinding {
    /// A declared state cannot be reached from the initial state using the
    /// declared transitions.
    UnreachableState {
        /// State identity.
        state: HdlId,
    },

    /// Two transitions from the same state may be simultaneously enabled.
    ///
    /// This is conservative: without guard proof, the analysis cannot
    /// establish mutual exclusivity.
    PotentiallyOverlappingTransitions {
        /// Source state identity.
        state: HdlId,
        /// First transition index.
        first_index: usize,
        /// Second transition index.
        second_index: usize,
    },

    /// The analysis did not prove whether a behavior is valid.
    ///
    /// This variant is reserved for future semantic-adapter findings. It
    /// must not be interpreted as success.
    Inconclusive {
        /// Stable explanation code supplied by the analysis layer.
        code: String,
    },
}

/// Result of structural validation and local state-machine analysis.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct StateMachineValidation {
    errors: Vec<StateMachineError>,
    findings: Vec<StateMachineFinding>,
}

impl StateMachineValidation {
    /// Return structural errors in deterministic order.
    #[must_use]
    pub fn errors(&self) -> &[StateMachineError] {
        &self.errors
    }

    /// Return non-fatal or inconclusive findings in deterministic order.
    #[must_use]
    pub fn findings(&self) -> &[StateMachineFinding] {
        &self.findings
    }

    /// Return whether no structural errors were detected.
    ///
    /// This does not mean that guards, actions, clocks, reset declarations,
    /// or other semantic properties have been fully validated.
    #[must_use]
    pub fn has_no_structural_errors(&self) -> bool {
        self.errors.is_empty()
    }

    /// Return whether the result contains no errors or inconclusive findings.
    ///
    /// This is still only a statement about the analyses actually performed.
    #[must_use]
    pub fn is_conclusive(&self) -> bool {
        self.errors.is_empty()
            && !self.findings.iter().any(|finding| {
                matches!(finding, StateMachineFinding::Inconclusive { .. })
            })
    }

    /// Return whether any structural errors were detected.
    #[must_use]
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
}

/// Validate state references, transition structure, reset-state references,
/// reachability, and conservative transition-overlap conditions.
///
/// The analysis is iterative and deterministic. It does not resolve names,
/// type-check guards, validate action effects, prove guard exclusivity, or
/// establish clock/reset compatibility.
///
/// Callers must not treat a structurally clean result as proof of full HDL
/// semantic validity.
#[must_use]
pub fn validate_state_machine(
    machine: &StateMachine,
) -> StateMachineValidation {
    let mut result = StateMachineValidation::default();

    // The first declaration of an identity is canonical for this validation
    // pass. Duplicate declarations are reported rather than silently merged.
    let mut state_indices: BTreeMap<HdlId, usize> = BTreeMap::new();

    for (index, state) in machine.states.iter().enumerate() {
        if state_indices.insert(state.id.clone(), index).is_some() {
            result.errors.push(StateMachineError::DuplicateState {
                state: state.id.clone(),
            });
        }
    }

    if !state_indices.contains_key(&machine.initial_state) {
        result.errors.push(
            StateMachineError::UndeclaredInitialState {
                state: machine.initial_state.clone(),
            },
        );
    }

    if let Some(reset) = &machine.reset {
        if !state_indices.contains_key(&reset.initial_state) {
            result.errors.push(
                StateMachineError::UndeclaredResetState {
                    state: reset.initial_state.clone(),
                },
            );
        }
    }

    let mut adjacency: BTreeMap<HdlId, Vec<HdlId>> = BTreeMap::new();

    // Track exact structural keys without depending on NodeId ordering.
    // The key represents the declared endpoints, event label, and default
    // status. Guard expression identity is not used to prove exclusivity.
    let mut transition_keys: BTreeMap<
        (HdlId, HdlId, Option<String>, bool),
        usize,
    > = BTreeMap::new();

    let mut defaults: BTreeMap<HdlId, usize> = BTreeMap::new();

    for (index, transition) in machine.transitions.iter().enumerate() {
        let source_declared = state_indices.contains_key(&transition.source);
        let target_declared = state_indices.contains_key(&transition.target);

        if !source_declared {
            result.errors.push(
                StateMachineError::UndeclaredTransitionSource {
                    transition_index: index,
                    state: transition.source.clone(),
                },
            );
        }

        if !target_declared {
            result.errors.push(
                StateMachineError::UndeclaredTransitionTarget {
                    transition_index: index,
                    state: transition.target.clone(),
                },
            );
        }

        if transition.is_default {
            if transition.event.is_some() {
                result.errors.push(
                    StateMachineError::DefaultTransitionHasEvent {
                        transition_index: index,
                    },
                );
            }

            if transition.guard.is_some() {
                result.errors.push(
                    StateMachineError::GuardedDefaultTransition {
                        transition_index: index,
                    },
                );
            }

            if let Some(first_index) =
                defaults.insert(transition.source.clone(), index)
            {
                result.errors.push(
                    StateMachineError::MultipleDefaultTransitions {
                        state: transition.source.clone(),
                        first_index,
                        duplicate_index: index,
                    },
                );
            }
        }

        let key = (
            transition.source.clone(),
            transition.target.clone(),
            transition.event.clone(),
            transition.is_default,
        );

        if let Some(first_index) = transition_keys.get(&key).copied() {
            result.errors.push(
                StateMachineError::DuplicateTransition {
                    first_index,
                    duplicate_index: index,
                },
            );
        } else {
            transition_keys.insert(key, index);
        }

        if source_declared && target_declared {
            adjacency
                .entry(transition.source.clone())
                .or_default()
                .push(transition.target.clone());
        }

        if let Some(source_index) =
            state_indices.get(&transition.source).copied()
        {
            if machine.states[source_index].kind == StateKind::Terminal {
                result.errors.push(
                    StateMachineError::TerminalStateHasOutgoingTransition {
                        state: transition.source.clone(),
                        transition_index: index,
                    },
                );
            }
        }
    }

    // Detect transitions whose guards cannot be proven mutually exclusive.
    // The analysis does not attempt to prove arbitrary expressions.
    let mut by_source_event: BTreeMap<
        (HdlId, Option<String>, bool),
        Vec<usize>,
    > = BTreeMap::new();

    for (index, transition) in machine.transitions.iter().enumerate() {
        by_source_event
            .entry((
                transition.source.clone(),
                transition.event.clone(),
                transition.is_default,
            ))
            .or_default()
            .push(index);
    }

    for ((source, _, is_default), indices) in &by_source_event {
        if *is_default || indices.len() < 2 {
            continue;
        }

        for left_position in 0..indices.len() {
            for right_position in (left_position + 1)..indices.len() {
                let left_index = indices[left_position];
                let right_index = indices[right_position];
                let left = &machine.transitions[left_index];
                let right = &machine.transitions[right_index];

                // Two unguarded alternatives with the same source/event
                // are definitely ambiguous. Guarded alternatives remain
                // potentially overlapping unless a semantic prover proves
                // otherwise. This module deliberately does not do that.
                let both_unguarded =
                    left.guard.is_none() && right.guard.is_none();

                if both_unguarded
                    || left.guard.is_some()
                    || right.guard.is_some()
                {
                    result.findings.push(
                        StateMachineFinding::PotentiallyOverlappingTransitions {
                            state: source.clone(),
                            first_index: left_index,
                            second_index: right_index,
                        },
                    );
                }
            }
        }
    }

    // Reachability starts only when the initial state resolves. A missing
    // initial state is already an error and cannot seed a valid traversal.
    if state_indices.contains_key(&machine.initial_state) {
        let mut visited: BTreeSet<HdlId> = BTreeSet::new();
        let mut pending: VecDeque<HdlId> = VecDeque::new();

        visited.insert(machine.initial_state.clone());
        pending.push_back(machine.initial_state.clone());

        while let Some(current) = pending.pop_front() {
            if let Some(targets) = adjacency.get(&current) {
                for target in targets {
                    if visited.insert(target.clone()) {
                        pending.push_back(target.clone());
                    }
                }
            }
        }

        for state in &machine.states {
            if !visited.contains(&state.id) {
                result.findings.push(
                    StateMachineFinding::UnreachableState {
                        state: state.id.clone(),
                    },
                );
            }
        }
    }

    // Stable ordering makes diagnostics reproducible across runs.
    result.errors.sort();
    result.findings.sort();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn id(value: &str) -> HdlId {
        HdlId::new(value).expect("test identity must be non-empty")
    }

    fn state(value: &str, kind: StateKind) -> StateDeclaration {
        StateDeclaration::new(id(value), kind, Vec::new(), Vec::new())
    }

    fn transition(source: &str, target: &str) -> StateTransition {
        StateTransition::new(
            id(source),
            id(target),
            None,
            None,
            Vec::new(),
            false,
        )
    }

    fn machine(
        initial: &str,
        states: Vec<StateDeclaration>,
        transitions: Vec<StateTransition>,
    ) -> StateMachine {
        StateMachine::new(
            id("controller"),
            id(initial),
            states,
            transitions,
            None,
            None,
        )
    }

    #[test]
    fn accepts_a_minimal_machine() {
        let value = machine(
            "idle",
            vec![state("idle", StateKind::Normal)],
            Vec::new(),
        );

        let result = validate_state_machine(&value);

        assert!(result.has_no_structural_errors());
        assert!(result.findings().is_empty());
    }

    #[test]
    fn rejects_an_undeclared_initial_state() {
        let value = machine(
            "missing",
            vec![state("idle", StateKind::Normal)],
            Vec::new(),
        );

        let result = validate_state_machine(&value);

        assert!(result.errors().iter().any(|error| matches!(
            error,
            StateMachineError::UndeclaredInitialState { .. }
        )));
    }

    #[test]
    fn rejects_duplicate_state_declarations() {
        let value = machine(
            "idle",
            vec![
                state("idle", StateKind::Normal),
                state("idle", StateKind::Normal),
            ],
            Vec::new(),
        );

        let result = validate_state_machine(&value);

        assert!(result.errors().iter().any(|error| matches!(
            error,
            StateMachineError::DuplicateState { .. }
        )));
    }

    #[test]
    fn rejects_missing_transition_endpoints() {
        let value = machine(
            "idle",
            vec![state("idle", StateKind::Normal)],
            vec![transition("missing", "also_missing")],
        );

        let result = validate_state_machine(&value);

        assert!(result.errors().iter().any(|error| matches!(
            error,
            StateMachineError::UndeclaredTransitionSource { .. }
        )));
        assert!(result.errors().iter().any(|error| matches!(
            error,
            StateMachineError::UndeclaredTransitionTarget { .. }
        )));
    }

    #[test]
    fn reports_unreachable_states() {
        let value = machine(
            "idle",
            vec![
                state("idle", StateKind::Normal),
                state("active", StateKind::Normal),
                state("orphan", StateKind::Normal),
            ],
            vec![transition("idle", "active")],
        );

        let result = validate_state_machine(&value);

        assert!(result.findings().iter().any(|finding| matches!(
            finding,
            StateMachineFinding::UnreachableState { state }
                if state.as_str() == "orphan"
        )));
    }

    #[test]
    fn reachability_handles_cycles_iteratively() {
        let value = machine(
            "a",
            vec![
                state("a", StateKind::Normal),
                state("b", StateKind::Normal),
                state("c", StateKind::Normal),
            ],
            vec![
                transition("a", "b"),
                transition("b", "c"),
                transition("c", "a"),
            ],
        );

        let result = validate_state_machine(&value);

        assert!(result.has_no_structural_errors());
        assert!(!result.findings().iter().any(|finding| matches!(
            finding,
            StateMachineFinding::UnreachableState { .. }
        )));
    }

    #[test]
    fn rejects_outgoing_transitions_from_terminal_states() {
        let value = machine(
            "done",
            vec![
                state("done", StateKind::Terminal),
                state("active", StateKind::Normal),
            ],
            vec![transition("done", "active")],
        );

        let result = validate_state_machine(&value);

        assert!(result.errors().iter().any(|error| matches!(
            error,
            StateMachineError::TerminalStateHasOutgoingTransition { .. }
        )));
    }

    #[test]
    fn detects_duplicate_transition_declarations() {
        let value = machine(
            "idle",
            vec![
                state("idle", StateKind::Normal),
                state("active", StateKind::Normal),
            ],
            vec![
                transition("idle", "active"),
                transition("idle", "active"),
            ],
        );

        let result = validate_state_machine(&value);

        assert!(result.errors().iter().any(|error| matches!(
            error,
            StateMachineError::DuplicateTransition { .. }
        )));
    }

    #[test]
    fn reports_multiple_default_transitions() {
        let value = machine(
            "idle",
            vec![
                state("idle", StateKind::Normal),
                state("a", StateKind::Normal),
                state("b", StateKind::Normal),
            ],
            vec![
                StateTransition::new(
                    id("idle"),
                    id("a"),
                    None,
                    None,
                    Vec::new(),
                    true,
                ),
                StateTransition::new(
                    id("idle"),
                    id("b"),
                    None,
                    None,
                    Vec::new(),
                    true,
                ),
            ],
        );

        let result = validate_state_machine(&value);

        assert!(result.errors().iter().any(|error| matches!(
            error,
            StateMachineError::MultipleDefaultTransitions { .. }
        )));
    }

    #[test]
    fn results_are_deterministic() {
        let value = machine(
            "idle",
            vec![
                state("idle", StateKind::Normal),
                state("orphan_b", StateKind::Normal),
                state("orphan_a", StateKind::Normal),
            ],
            Vec::new(),
        );

        assert_eq!(
            validate_state_machine(&value),
            validate_state_machine(&value)
        );
    }

    #[test]
    fn reset_state_must_be_declared() {
        let value = StateMachine::new(
            id("controller"),
            id("idle"),
            vec![state("idle", StateKind::Normal)],
            Vec::new(),
            Some(id("clock")),
            Some(StateMachineReset::new(
                id("reset"),
                id("missing"),
                ResetMode::Synchronous,
            )),
        );

        let result = validate_state_machine(&value);

        assert!(result.errors().iter().any(|error| matches!(
            error,
            StateMachineError::UndeclaredResetState { .. }
        )));
    }
}