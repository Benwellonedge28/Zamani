
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani classical concurrency: determinism contracts and stable ordering.
//!
//! Rust baseline: 1.97+
//! Edition: 2021
//! Dependencies: Rust standard library only.
//!
//! # File contract
//!
//! Status: Proposed implementation; verification against the repository is
//! pending.
//!
//! Purpose:
//! - Represent determinism requirements independently of physical execution.
//! - Define explicit policies for observable ordering and numerical reductions.
//! - Provide stable logical event keys and deterministic event ordering.
//! - Reject conflicting event keys rather than silently overwrite events.
//! - Provide ordered reference reduction semantics.
//!
//! Owns:
//! - DeterminismMode and DeterminismPolicy.
//! - NumericReproducibility and ObservableOrder.
//! - OrderKey, OrderedEvent, OrderedEvents, and DeterminismError.
//! - Deterministic ordered reduction and event ordering.
//!
//! Does not own:
//! - Source syntax, lexer tokens, frontend AST, semantic types, or canonical IR.
//! - Task identity, task lifecycle, or cancellation.
//! - Physical execution order, scheduling, worker allocation, or placement.
//! - Floating-point formats, arithmetic, rounding modes, or numerical proofs.
//! - Random number generation or entropy sources.
//! - Distributed consensus, durable logs, clocks, or network protocols.
//!
//! Integration:
//! - grammar/concurrency/deterministic-parallelism.g4 preserves source intent.
//! - grammar/spec/determinism.md defines language-level determinism semantics.
//! - concurrency/task.rs owns TaskId and task lifecycle.
//! - concurrency/data_parallel.rs owns logical data domains and partitions.
//! - concurrency/futures.rs owns future composition.
//! - concurrency/channels.rs owns channel communication.
//! - concurrency/synchronization.rs and atomics.rs own synchronization.
//! - numeric/reproducibility.rs should own numerical reproducibility algorithms.
//! - Semantic analysis determines which policy a source construct requires.
//! - The canonical IR carries the validated semantic requirement.
//! - The optimizer, scheduler, and runtime must preserve that requirement.
//!
//! This module provides contract and ordering primitives. Merely using these
//! primitives does not prove that arbitrary program execution is deterministic.
//!
//! # POCO-REAF
//!
//! Logical ordering is independent of worker count, processor identity,
//! physical completion time, and target architecture.
//!
//! There is no fixed semantic limit on event count or key depth. Keys and event
//! collections consume finite host memory, and host integer representations
//! remain finite. Resource exhaustion must not be silently treated as success.
//!
//! # Determinism versus reproducibility
//!
//! Determinism concerns the observable behavior guaranteed by the language.
//! Numerical reproducibility may additionally require stable floating-point
//! operation order, specified rounding, controlled reduction algorithms, or
//! stronger arithmetic guarantees. This module does not claim bitwise
//! reproducibility for arbitrary floating-point operations.

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

/// Strength of the determinism guarantee requested by a program.
///
/// These values describe semantic intent. They do not prescribe a thread
/// count, execution target, scheduler, or implementation strategy.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DeterminismMode {
    /// The program has not requested an additional determinism guarantee.
    ///
    /// Other language rules still apply. This is not permission to violate
    /// any guarantees already required by the language.
    #[default]
    Unspecified,

    /// Prefer deterministic behavior where feasible, without asserting that
    /// every observable behavior is reproducible.
    BestEffort,

    /// Observable behavior covered by the language contract must be invariant
    /// under permitted changes in execution strategy.
    Deterministic,

    /// Require the stronger reproducibility contract selected by semantic
    /// analysis, including any numerical requirements that apply.
    Reproducible,
}

impl DeterminismMode {
    /// Whether this mode expresses an explicit determinism requirement.
    pub const fn is_explicit(self) -> bool {
        !matches!(self, Self::Unspecified)
    }

    /// Whether this mode requests the strongest reproducibility level.
    pub const fn requires_reproducibility(self) -> bool {
        matches!(self, Self::Reproducible)
    }
}

/// Policy for observable ordering.
///
/// A deterministic computation need not serialize all operations. This policy
/// specifies only the ordering guarantee required for the events it covers.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum ObservableOrder {
    /// Preserve the ordering already required by source-language semantics.
    ///
    /// Independent operations may execute concurrently if their observable
    /// behavior remains valid.
    #[default]
    SemanticDependencies,

    /// Observable events covered by this policy must follow their assigned
    /// logical order, independently of physical completion order.
    LogicalKeyOrder,

    /// The relevant operation must preserve source-defined sequential order.
    ///
    /// This is stronger than ordinary deterministic behavior and may constrain
    /// legal optimization or parallel execution.
    SourceOrder,
}

/// Policy for numerical reproducibility.
///
/// Actual floating-point formats, rounding rules, exact arithmetic, and
/// numerical error guarantees belong to the numeric subsystem.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum NumericReproducibility {
    /// Apply the numerical semantics otherwise required by the program.
    #[default]
    LanguageDefined,

    /// Preserve the specified evaluation order for the covered operation.
    PreserveEvaluationOrder,

    /// Use a specified stable reduction strategy, when one has been defined
    /// for the operation and its numeric type.
    StableReduction,

    /// Require the numeric subsystem to establish the stronger reproducibility
    /// guarantee applicable to this operation.
    RequireReproducibleResult,
}

/// Declarative determinism policy.
///
/// This value describes requirements; it does not inspect program effects or
/// prove that a computation satisfies them. Semantic analysis and later
/// compiler/runtime stages must establish and preserve those guarantees.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct DeterminismPolicy {
    mode: DeterminismMode,
    observable_order: ObservableOrder,
    numeric_reproducibility: NumericReproducibility,
}

impl DeterminismPolicy {
    /// Construct an explicit policy.
    pub const fn new(
        mode: DeterminismMode,
        observable_order: ObservableOrder,
        numeric_reproducibility: NumericReproducibility,
    ) -> Self {
        Self {
            mode,
            observable_order,
            numeric_reproducibility,
        }
    }

    /// Policy with no additional determinism requirement.
    pub const fn unspecified() -> Self {
        Self::new(
            DeterminismMode::Unspecified,
            ObservableOrder::SemanticDependencies,
            NumericReproducibility::LanguageDefined,
        )
    }

    /// Policy requiring deterministic observable behavior while allowing
    /// independent work to execute concurrently.
    pub const fn deterministic() -> Self {
        Self::new(
            DeterminismMode::Deterministic,
            ObservableOrder::SemanticDependencies,
            NumericReproducibility::LanguageDefined,
        )
    }

    /// Policy requiring reproducibility and explicit source-order preservation.
    ///
    /// This is intentionally conservative. A language implementation may
    /// choose a less restrictive policy only when the semantic contract proves
    /// that doing so preserves the requested reproducibility guarantee.
    pub const fn reproducible() -> Self {
        Self::new(
            DeterminismMode::Reproducible,
            ObservableOrder::SourceOrder,
            NumericReproducibility::RequireReproducibleResult,
        )
    }

    pub const fn mode(self) -> DeterminismMode {
        self.mode
    }

    pub const fn observable_order(self) -> ObservableOrder {
        self.observable_order
    }

    pub const fn numeric_reproducibility(self) -> NumericReproducibility {
        self.numeric_reproducibility
    }

    pub const fn requires_determinism(self) -> bool {
        matches!(
            self.mode,
            DeterminismMode::Deterministic | DeterminismMode::Reproducible
        )
    }

    pub const fn requires_reproducibility(self) -> bool {
        self.mode.requires_reproducibility()
            || matches!(
                self.numeric_reproducibility,
                NumericReproducibility::RequireReproducibleResult
            )
    }

    /// Combine requirements conservatively.
    ///
    /// The result never weakens either input's mode, ordering, or numerical
    /// reproducibility requirement. This operation is suitable for combining
    /// nested requirements before semantic validation.
    pub const fn combine(self, other: Self) -> Self {
        Self {
            mode: stronger_mode(self.mode, other.mode),
            observable_order: stronger_order(
                self.observable_order,
                other.observable_order,
            ),
            numeric_reproducibility: stronger_numeric_policy(
                self.numeric_reproducibility,
                other.numeric_reproducibility,
            ),
        }
    }
}

const fn stronger_mode(
    left: DeterminismMode,
    right: DeterminismMode,
) -> DeterminismMode {
    if left as u8 >= right as u8 {
        left
    } else {
        right
    }
}

const fn stronger_order(
    left: ObservableOrder,
    right: ObservableOrder,
) -> ObservableOrder {
    use ObservableOrder::*;

    match (left, right) {
        (SourceOrder, _) | (_, SourceOrder) => SourceOrder,
        (LogicalKeyOrder, _) | (_, LogicalKeyOrder) => LogicalKeyOrder,
        _ => SemanticDependencies,
    }
}

const fn stronger_numeric_policy(
    left: NumericReproducibility,
    right: NumericReproducibility,
) -> NumericReproducibility {
    use NumericReproducibility::*;

    match (left, right) {
        (RequireReproducibleResult, _) | (_, RequireReproducibleResult) => {
            RequireReproducibleResult
        }
        (StableReduction, _) | (_, StableReduction) => StableReduction,
        (PreserveEvaluationOrder, _) | (_, PreserveEvaluationOrder) => {
            PreserveEvaluationOrder
        }
        _ => LanguageDefined,
    }
}

/// A hierarchical logical order key.
///
/// Keys compare lexicographically. A caller can represent nested execution
/// paths without encoding worker IDs, physical clocks, or machine addresses.
///
/// Example:
///
/// - `[3]` precedes `[4]`.
/// - `[3, 1]` precedes `[3, 2]`.
/// - `[3]` precedes `[3, 1]`.
///
/// Key components are logical identifiers supplied by a compiler, semantic
/// plan, or execution protocol. This type does not allocate globally unique
/// identities or guarantee that independently created keys will not collide.
///
/// The number of components is not fixed by this API. A key consumes memory
/// proportional to its depth.
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct OrderKey {
    components: Vec<u128>,
}

impl OrderKey {
    /// Construct a key from its logical path components.
    pub fn new(components: impl IntoIterator<Item = u128>) -> Self {
        Self {
            components: components.into_iter().collect(),
        }
    }

    /// Construct the empty root key.
    pub const fn root() -> Self {
        Self {
            components: Vec::new(),
        }
    }

    /// Return the logical components of this key.
    pub fn components(&self) -> &[u128] {
        &self.components
    }

    /// Return whether this is the root key.
    pub fn is_root(&self) -> bool {
        self.components.is_empty()
    }

    /// Return the number of components.
    pub fn depth(&self) -> usize {
        self.components.len()
    }

    /// Return a child key by appending one logical component.
    pub fn child(&self, component: u128) -> Self {
        let mut components = self.components.clone();
        components.push(component);
        Self { components }
    }

    /// Return a key with another path appended.
    pub fn extend(
        &self,
        suffix: impl IntoIterator<Item = u128>,
    ) -> Self {
        let mut components = self.components.clone();
        components.extend(suffix);
        Self { components }
    }
}

impl From<u128> for OrderKey {
    fn from(component: u128) -> Self {
        Self {
            components: vec![component],
        }
    }
}

impl From<Vec<u128>> for OrderKey {
    fn from(components: Vec<u128>) -> Self {
        Self { components }
    }
}

/// One logical event and the key used to order it.
///
/// The event payload is generic so this module does not invent a second
/// diagnostic, provenance, runtime event, or IR representation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OrderedEvent<T> {
    key: OrderKey,
    payload: T,
}

impl<T> OrderedEvent<T> {
    pub fn new(key: OrderKey, payload: T) -> Self {
        Self { key, payload }
    }

    pub fn key(&self) -> &OrderKey {
        &self.key
    }

    pub fn payload(&self) -> &T {
        &self.payload
    }

    pub fn into_parts(self) -> (OrderKey, T) {
        (self.key, self.payload)
    }
}

/// Error conditions for deterministic ordering and policy validation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DeterminismError {
    /// Multiple events were assigned the same logical key.
    DuplicateOrderKey {
        key: OrderKey,
    },

    /// A caller attempted an operation incompatible with the declared policy.
    PolicyViolation {
        message: String,
    },

    /// A checked operation exceeded the host's representational capacity.
    RepresentationLimit {
        operation: &'static str,
    },
}

impl std::fmt::Display for DeterminismError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DuplicateOrderKey { key } => {
                write!(f, "duplicate logical event order key: {key:?}")
            }
            Self::PolicyViolation { message } => {
                write!(f, "determinism policy violation: {message}")
            }
            Self::RepresentationLimit { operation } => {
                write!(
                    f,
                    "host representation limit reached during {operation}"
                )
            }
        }
    }
}

impl Error for DeterminismError {}

/// Events indexed by unique logical ordering keys.
///
/// Events are returned in key order, regardless of insertion order or
/// physical completion order. Inserting a duplicate key is an error; it never
/// silently overwrites an earlier event.
///
/// This is an in-memory utility, not a durable event log or distributed
/// ordering protocol. The caller is responsible for choosing keys whose
/// meaning is stable across the execution being compared.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OrderedEvents<T> {
    events: BTreeMap<OrderKey, T>,
}

impl<T> Default for OrderedEvents<T> {
    fn default() -> Self {
        Self {
            events: BTreeMap::new(),
        }
    }
}

impl<T> OrderedEvents<T> {
    pub fn new() -> Self {
        Self::default()
    }

    /// Number of currently stored events.
    pub fn len(&self) -> usize {
        self.events.len()
    }

    /// Whether no events are stored.
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    /// Insert an event with a unique logical key.
    ///
    /// On a duplicate, the new payload is returned to the caller and the
    /// existing event remains unchanged.
    pub fn insert(
        &mut self,
        key: OrderKey,
        payload: T,
    ) -> Result<(), (DeterminismError, T)> {
        use std::collections::btree_map::Entry;

        match self.events.entry(key) {
            Entry::Vacant(entry) => {
                entry.insert(payload);
                Ok(())
            }
            Entry::Occupied(entry) => {
                Err((
                    DeterminismError::DuplicateOrderKey {
                        key: entry.key().clone(),
                    },
                    payload,
                ))
            }
        }
    }

    /// Inspect the event associated with a key.
    pub fn get(&self, key: &OrderKey) -> Option<&T> {
        self.events.get(key)
    }

    /// Remove and return an event by key.
    pub fn remove(&mut self, key: &OrderKey) -> Option<T> {
        self.events.remove(key)
    }

    /// Iterate over events in ascending logical-key order.
    pub fn iter(&self) -> impl Iterator<Item = (&OrderKey, &T)> {
        self.events.iter()
    }

    /// Consume the collection and return events in ascending logical-key
    /// order.
    pub fn into_ordered(self) -> Vec<OrderedEvent<T>> {
        self.events
            .into_iter()
            .map(|(key, payload)| OrderedEvent { key, payload })
            .collect()
    }

    /// Consume the collection and yield ordered events lazily.
    ///
    /// The iterator owns the map. It avoids constructing a second vector of
    /// all events, although the original map itself still occupies memory.
    pub fn into_iter_ordered(
        self,
    ) -> impl Iterator<Item = OrderedEvent<T>> {
        self.events
            .into_iter()
            .map(|(key, payload)| OrderedEvent { key, payload })
    }
}

/// Reduce values in their supplied order without reassociating operations.
///
/// This is the reference implementation for ordered reductions. It does not
/// claim that an arbitrary reducer is associative, commutative, pure, or
/// numerically reproducible under every platform.
///
/// An empty iterator returns `None`; a singleton returns its only value
/// without calling the reducer.
pub fn reduce_ordered<I, F>(
    values: I,
    mut combine: F,
) -> Option<I::Item>
where
    I: IntoIterator,
    F: FnMut(I::Item, I::Item) -> I::Item,
{
    let mut values = values.into_iter();
    let first = values.next()?;

    Some(values.fold(first, |accumulator, value| {
        combine(accumulator, value)
    }))
}

/// Reduce values in order while propagating an error.
///
/// The first error encountered in iteration order is returned. Partial
/// accumulator state is not returned on failure.
pub fn try_reduce_ordered<I, F, T, E>(
    values: I,
    mut combine: F,
) -> Result<Option<T>, E>
where
    I: IntoIterator<Item = T>,
    F: FnMut(T, T) -> Result<T, E>,
{
    let mut values = values.into_iter();

    let Some(first) = values.next() else {
        return Ok(None);
    };

    values.try_fold(first, |accumulator, value| {
        combine(accumulator, value)
    }).map(Some)
}

/// Validate that a policy permits a requested implementation property.
///
/// This function only checks the supplied policy and property. It cannot
/// inspect a program, prove that its effects are deterministic, or certify
/// that a backend implementation satisfies the property.
pub fn require_logical_key_order(
    policy: DeterminismPolicy,
) -> Result<(), DeterminismError> {
    match policy.observable_order() {
        ObservableOrder::LogicalKeyOrder => Ok(()),
        ObservableOrder::SemanticDependencies | ObservableOrder::SourceOrder => {
            Err(DeterminismError::PolicyViolation {
                message: String::from(
                    "logical-key event ordering was requested, but the \
                     policy does not declare LogicalKeyOrder",
                ),
            })
        }
    }
}

/// Validate that a policy permits a stable reduction strategy.
///
/// This checks the declared numerical policy only. The numeric subsystem must
/// still supply a suitable reducer and prove that it satisfies the contract.
pub fn require_stable_reduction(
    policy: DeterminismPolicy,
) -> Result<(), DeterminismError> {
    match policy.numeric_reproducibility() {
        NumericReproducibility::StableReduction
        | NumericReproducibility::RequireReproducibleResult => Ok(()),

        NumericReproducibility::LanguageDefined
        | NumericReproducibility::PreserveEvaluationOrder => {
            Err(DeterminismError::PolicyViolation {
                message: String::from(
                    "stable reduction requires an explicit stable-reduction \
                     or reproducible-result numerical policy",
                ),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_policy_is_unspecified() {
        let policy = DeterminismPolicy::default();

        assert_eq!(policy.mode(), DeterminismMode::Unspecified);
        assert!(!policy.requires_determinism());
        assert!(!policy.requires_reproducibility());
    }

    #[test]
    fn reproducible_policy_requires_reproducible_result() {
        let policy = DeterminismPolicy::reproducible();

        assert!(policy.requires_determinism());
        assert!(policy.requires_reproducibility());
        assert_eq!(policy.observable_order(), ObservableOrder::SourceOrder);
        assert_eq!(
            policy.numeric_reproducibility(),
            NumericReproducibility::RequireReproducibleResult
        );
    }

    #[test]
    fn policy_combination_never_weakens_requirements() {
        let left = DeterminismPolicy::deterministic();
        let right = DeterminismPolicy::reproducible();
        let combined = left.combine(right);

        assert_eq!(combined.mode(), DeterminismMode::Reproducible);
        assert_eq!(combined.observable_order(), ObservableOrder::SourceOrder);
        assert_eq!(
            combined.numeric_reproducibility(),
            NumericReproducibility::RequireReproducibleResult
        );
    }

    #[test]
    fn key_order_is_lexicographic() {
        let keys = [
            OrderKey::new([4, 0]),
            OrderKey::new([2, 9]),
            OrderKey::new([2, 3]),
            OrderKey::new([2]),
        ];

        let mut sorted = keys;
        sorted.sort();

        assert_eq!(
            sorted,
            [
                OrderKey::new([2]),
                OrderKey::new([2, 3]),
                OrderKey::new([2, 9]),
                OrderKey::new([4, 0]),
            ]
        );
    }

    #[test]
    fn keys_support_deep_paths_and_large_components() {
        let key = OrderKey::new([u128::MAX, 0, 7])
            .extend([1, 2, 3, 4, 5]);

        assert_eq!(key.depth(), 8);
        assert_eq!(key.components()[0], u128::MAX);
        assert_eq!(key.components()[7], 5);
    }

    #[test]
    fn child_key_does_not_mutate_parent() {
        let parent = OrderKey::new([7, 11]);
        let child = parent.child(13);

        assert_eq!(parent.components(), &[7, 11]);
        assert_eq!(child.components(), &[7, 11, 13]);
    }

    #[test]
    fn events_are_returned_in_logical_order_not_insert_order() {
        let mut events = OrderedEvents::new();

        events.insert(OrderKey::from(9), "third").unwrap();
        events.insert(OrderKey::from(2), "first").unwrap();
        events.insert(OrderKey::from(5), "second").unwrap();

        let ordered = events.into_ordered();

        assert_eq!(
            ordered
                .into_iter()
                .map(|event| event.into_parts())
                .collect::<Vec<_>>(),
            vec![
                (OrderKey::from(2), "first"),
                (OrderKey::from(5), "second"),
                (OrderKey::from(9), "third"),
            ]
        );
    }

    #[test]
    fn duplicate_event_key_does_not_overwrite_original() {
        let key = OrderKey::new([1, 2]);
        let mut events = OrderedEvents::new();

        events.insert(key.clone(), "original").unwrap();

        let (error, rejected) = events
            .insert(key.clone(), "duplicate")
            .expect_err("duplicate key must fail");

        assert_eq!(rejected, "duplicate");
        assert_eq!(
            error,
            DeterminismError::DuplicateOrderKey { key: key.clone() }
        );
        assert_eq!(events.get(&key), Some(&"original"));
        assert_eq!(events.len(), 1);
    }

    #[test]
    fn removing_event_does_not_change_other_events() {
        let mut events = OrderedEvents::new();

        events.insert(OrderKey::from(1), 10).unwrap();
        events.insert(OrderKey::from(2), 20).unwrap();

        assert_eq!(events.remove(&OrderKey::from(1)), Some(10));
        assert_eq!(events.get(&OrderKey::from(2)), Some(&20));
        assert_eq!(events.len(), 1);
    }

    #[test]
    fn ordered_reduction_preserves_input_order() {
        let result = reduce_ordered(["a", "b", "c"], |left, right| {
            format!("({left}+{right})")
        });

        assert_eq!(result.as_deref(), Some("((a+b)+c)"));
    }

    #[test]
    fn ordered_reduction_handles_empty_and_singleton_inputs() {
        assert_eq!(reduce_ordered(Vec::<u64>::new(), u64::wrapping_add), None);
        assert_eq!(reduce_ordered([42_u64], u64::wrapping_add), Some(42));
    }

    #[test]
    fn fallible_reduction_returns_first_error_in_input_order() {
        let values = [2_i32, 3, 0, 4];

        let result = try_reduce_ordered(values, |left, right| {
            if right == 0 {
                Err("zero operand")
            } else {
                Ok(left + right)
            }
        });

        assert_eq!(result, Err("zero operand"));
    }

    #[test]
    fn stable_reduction_requires_explicit_policy() {
        assert!(require_stable_reduction(
            DeterminismPolicy::deterministic()
        )
        .is_err());

        let policy = DeterminismPolicy::new(
            DeterminismMode::Deterministic,
            ObservableOrder::SemanticDependencies,
            NumericReproducibility::StableReduction,
        );

        assert!(require_stable_reduction(policy).is_ok());
    }

    #[test]
    fn logical_order_requires_matching_policy() {
        let policy = DeterminismPolicy::new(
            DeterminismMode::Deterministic,
            ObservableOrder::LogicalKeyOrder,
            NumericReproducibility::LanguageDefined,
        );

        assert!(require_logical_key_order(policy).is_ok());
        assert!(
            require_logical_key_order(DeterminismPolicy::deterministic()).is_err()
        );
    }

    #[test]
    fn empty_event_collection_is_valid() {
        let events = OrderedEvents::<String>::new();

        assert!(events.is_empty());
        assert_eq!(events.len(), 0);
        assert_eq!(events.into_ordered().len(), 0);
    }
}
