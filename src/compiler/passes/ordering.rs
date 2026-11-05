
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Compiler Pass Ordering
//!
//! Path: src/compiler/passes/ordering.rs
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//!
//! PURPOSE
//!
//! Validate explicit pass dependencies and calculate a deterministic,
//! dependency-respecting order for compiler passes.
//!
//! ARCHITECTURAL AUTHORITY
//!
//! Canonical pass contracts:
//!     crate::zuir::pass::contracts
//!
//! Operational pass contracts:
//!     crate::zuir::pass::manager
//!
//! Compiler-facing operational aliases:
//!     crate::compiler::passes::manager
//!
//! Compiler pass pipeline:
//!     crate::compiler::passes::pipeline
//!
//! Pass registration:
//!     crate::zuir::pass::registry
//!
//! Pass execution:
//!     crate::zuir::pass::manager
//!
//! OWNS
//!
//! - Explicit pass-to-pass dependency declarations.
//! - Dependency graph validation for a supplied pass set.
//! - Deterministic topological ordering.
//! - Topological dependency levels.
//! - Ordering-specific errors.
//!
//! DOES NOT OWN
//!
//! - Pass implementations.
//! - Pass identities or canonical descriptors.
//! - Pass registration.
//! - Pass execution.
//! - IR representations.
//! - IR verification.
//! - Analysis computation or result storage.
//! - Analysis invalidation.
//! - Optimization algorithms.
//! - Hardware discovery.
//! - Physical resource allocation.
//! - Runtime scheduling.
//!
//! ORDERING MODEL
//!
//! A dependency A -> B means that A must execute before B.
//!
//! Passes with no dependency path between them are ordered by their
//! original descriptor position. This provides stable deterministic
//! ordering without imposing an arbitrary ordering by domain or name.
//!
//! The returned levels represent dependency generations. They do not,
//! by themselves, authorize parallel execution. Side effects, resource
//! conflicts, pass safety, and execution policy must be checked separately.
//!
//! ANALYSIS MODEL
//!
//! Descriptor required_analyses and invalidated_analyses are not treated
//! as pass dependency declarations.
//!
//! The existing descriptors do not declare which pass produces each
//! analysis. Required-analysis availability remains the responsibility
//! of the pass manager and analysis subsystem.
//!
//! SCALABILITY
//!
//! No fixed pass-count ceiling is introduced.
//!
//! The implementation uses dynamically sized collections.
//!
//! The algorithm has O(V + E) graph traversal complexity, excluding
//! ordered-map/set operations used for deterministic bookkeeping.
//!
//! V is the number of supplied passes.
//! E is the number of distinct declared dependencies.
//!
//! Actual memory, address-space, and processing limits remain real
//! implementation constraints.
//!
//! SAFETY
//!
//! Unsafe Rust is forbidden.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

// -----------------------------------------------------------------------------
// Descriptor integration
// -----------------------------------------------------------------------------

/// A pass descriptor that can participate in dependency ordering.
///
/// This abstraction allows the ordering algorithm to consume both:
///
/// - canonical descriptors from `zuir::pass::contracts`;
/// - operational descriptors from the existing ZUIR pass manager.
///
/// It does not define another pass identity or descriptor.
///
/// Implementations must return a stable identity for the lifetime of
/// the descriptor snapshot.
pub trait PassOrderingDescriptor {
    /// The identity type used by the descriptor.
    type Id: Clone + Ord + fmt::Display;

    /// Return the stable identity of the pass.
    fn ordering_id(&self) -> &Self::Id;

    /// Validate descriptor metadata relevant to ordering.
    fn validate_for_ordering(&self) -> Result<(), String>;
}

/// Integrate canonical ZUIR pass descriptors.
impl PassOrderingDescriptor for crate::zuir::pass::contracts::PassDescriptor {
    type Id = crate::zuir::pass::contracts::PassId;

    fn ordering_id(&self) -> &Self::Id {
        &self.id
    }

    fn validate_for_ordering(&self) -> Result<(), String> {
        self.validate().map_err(|error| error.to_string())
    }
}

/// Integrate the existing operational descriptors.
///
/// These are intentionally not confused with the canonical descriptor
/// types. The ZUIR manager migration remains owned by the ZUIR subsystem.
impl PassOrderingDescriptor for super::manager::OperationalPassDescriptor {
    type Id = super::manager::OperationalPassId;

    fn ordering_id(&self) -> &Self::Id {
        &self.id
    }

    fn validate_for_ordering(&self) -> Result<(), String> {
        if self.description.trim().is_empty() {
            return Err("pass description must not be empty".to_string());
        }

        Ok(())
    }
}

// -----------------------------------------------------------------------------
// Dependency declarations
// -----------------------------------------------------------------------------

/// An explicit ordering dependency.
///
/// `prerequisite` must execute before `dependent`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PassDependency<Id> {
    /// Pass that must execute first.
    pub prerequisite: Id,

    /// Pass that depends on the prerequisite.
    pub dependent: Id,
}

impl<Id> PassDependency<Id> {
    /// Construct an explicit dependency.
    #[must_use]
    pub fn new(prerequisite: Id, dependent: Id) -> Self {
        Self {
            prerequisite,
            dependent,
        }
    }
}

// -----------------------------------------------------------------------------
// Ordering errors
// -----------------------------------------------------------------------------

/// Machine-readable pass-ordering failure category.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PassOrderingErrorKind {
    /// A pass descriptor contains invalid ordering metadata.
    InvalidDescriptor,

    /// The supplied pass collection contains duplicate identities.
    DuplicatePass,

    /// A dependency refers to a pass that is not in the supplied collection.
    MissingPass,

    /// A pass depends on itself.
    SelfDependency,

    /// The dependency graph contains a cycle.
    DependencyCycle,

    /// A graph counter or index cannot be represented.
    RepresentationLimitExceeded,

    /// A required allocation reservation failed.
    AllocationReservationFailed,

    /// The computed graph state is inconsistent.
    InvalidGraph,
}

/// Structured pass-ordering error.
///
/// `pass` identifies the pass primarily associated with the error.
///
/// `dependency` identifies the other pass involved, when applicable.
///
/// `blocked_passes` identifies the passes left unresolved by a cycle.
/// Some of these passes may depend on a cycle without belonging to the
/// cycle themselves.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PassOrderingError<Id> {
    pub kind: PassOrderingErrorKind,
    pub pass: Option<Id>,
    pub dependency: Option<Id>,
    pub blocked_passes: Vec<Id>,
    pub message: String,
}

impl<Id> PassOrderingError<Id> {
    fn new(
        kind: PassOrderingErrorKind,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            pass: None,
            dependency: None,
            blocked_passes: Vec::new(),
            message: message.into(),
        }
    }

    fn for_pass(
        kind: PassOrderingErrorKind,
        pass: Id,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            pass: Some(pass),
            dependency: None,
            blocked_passes: Vec::new(),
            message: message.into(),
        }
    }

    fn for_dependency(
        kind: PassOrderingErrorKind,
        pass: Id,
        dependency: Id,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            pass: Some(pass),
            dependency: Some(dependency),
            blocked_passes: Vec::new(),
            message: message.into(),
        }
    }
}

impl<Id: fmt::Display> fmt::Display for PassOrderingError<Id> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:?}: {}", self.kind, self.message)?;

        if let Some(pass) = &self.pass {
            write!(formatter, " [pass: {pass}]")?;
        }

        if let Some(dependency) = &self.dependency {
            write!(formatter, " [dependency: {dependency}]")?;
        }

        if !self.blocked_passes.is_empty() {
            write!(formatter, " [blocked passes: ")?;

            for (index, pass) in self.blocked_passes.iter().enumerate() {
                if index != 0 {
                    formatter.write_str(", ")?;
                }

                write!(formatter, "{pass}")?;
            }

            formatter.write_str("]")?;
        }

        Ok(())
    }
}

impl<Id: fmt::Debug + fmt::Display> Error for PassOrderingError<Id> {}

// -----------------------------------------------------------------------------
// Ordering result
// -----------------------------------------------------------------------------

/// Validated, deterministic pass ordering.
///
/// `ordered_ids` is the flattened execution order.
///
/// `levels` groups passes by dependency generation.
///
/// `positions` maps each pass ID to its flattened position.
///
/// The result owns its IDs and does not borrow the descriptor collection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PassOrderingResult<Id> {
    ordered_ids: Vec<Id>,
    levels: Vec<Vec<Id>>,
    positions: BTreeMap<Id, usize>,
}

impl<Id: Ord> PassOrderingResult<Id> {
    /// Return all pass IDs in their calculated order.
    #[must_use]
    pub fn ordered_ids(&self) -> &[Id] {
        &self.ordered_ids
    }

    /// Return dependency levels.
    ///
    /// Passes in the same level have no dependency path between them
    /// in the supplied dependency graph.
    ///
    /// This does not prove that their implementations are safe to execute
    /// concurrently.
    #[must_use]
    pub fn levels(&self) -> &[Vec<Id>] {
        &self.levels
    }

    /// Return the zero-based position of a pass.
    #[must_use]
    pub fn position(&self, id: &Id) -> Option<usize> {
        self.positions.get(id).copied()
    }

    /// Return the number of passes in the ordering.
    #[must_use]
    pub fn len(&self) -> usize {
        self.ordered_ids.len()
    }

    /// Whether the ordering contains no passes.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.ordered_ids.is_empty()
    }

    /// Return the number of dependency levels.
    #[must_use]
    pub fn level_count(&self) -> usize {
        self.levels.len()
    }
}

// -----------------------------------------------------------------------------
// Pass ordering
// -----------------------------------------------------------------------------

/// Explicit dependency graph for compiler pass ordering.
///
/// This type does not own or register pass implementations.
///
/// Dependencies can be added in any order. The graph is validated against
/// the actual pass descriptors when `order` is called.
#[derive(Debug, Clone)]
pub struct PassOrdering<Id> {
    dependencies: BTreeSet<PassDependency<Id>>,
}

impl<Id: Ord> Default for PassOrdering<Id> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Id: Ord> PassOrdering<Id> {
    /// Create an empty dependency graph.
    #[must_use]
    pub fn new() -> Self {
        Self {
            dependencies: BTreeSet::new(),
        }
    }

    /// Return the number of distinct dependency declarations.
    #[must_use]
    pub fn dependency_count(&self) -> usize {
        self.dependencies.len()
    }

    /// Whether the graph has no dependency declarations.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.dependencies.is_empty()
    }

    /// Iterate over explicit dependencies in deterministic order.
    pub fn dependencies(
        &self,
    ) -> impl ExactSizeIterator<Item = &PassDependency<Id>> {
        self.dependencies.iter()
    }

    /// Add a dependency.
    ///
    /// Returns `true` when a new edge was inserted and `false` when the
    /// same edge was already present.
    ///
    /// Self-dependencies are rejected immediately.
    pub fn add_dependency(
        &mut self,
        prerequisite: Id,
        dependent: Id,
    ) -> Result<bool, PassOrderingError<Id>>
    where
        Id: Clone + fmt::Display,
    {
        if prerequisite == dependent {
            return Err(PassOrderingError::for_pass(
                PassOrderingErrorKind::SelfDependency,
                prerequisite,
                "a pass cannot depend on itself",
            ));
        }

        Ok(self.dependencies.insert(PassDependency::new(
            prerequisite,
            dependent,
        )))
    }

    /// Remove a dependency declaration.
    ///
    /// Returns whether the edge existed.
    pub fn remove_dependency(
        &mut self,
        prerequisite: &Id,
        dependent: &Id,
    ) -> bool
    where
        Id: Clone,
    {
        self.dependencies.remove(&PassDependency::new(
            prerequisite.clone(),
            dependent.clone(),
        ))
    }

    /// Calculate a deterministic topological ordering.
    ///
    /// The descriptor slice establishes the stable tie-breaking order.
    ///
    /// Every dependency must refer to a descriptor in the supplied slice.
    ///
    /// A dependency A -> B guarantees that A appears before B.
    ///
    /// No pass is executed by this method.
    pub fn order<D>(
        &self,
        descriptors: &[D],
    ) -> Result<PassOrderingResult<Id>, PassOrderingError<Id>>
    where
        D: PassOrderingDescriptor<Id = Id>,
        Id: Clone + fmt::Display,
    {
        let pass_count = descriptors.len();

        // Validate descriptors and build their stable input-order index.
        let mut indices = BTreeMap::<Id, usize>::new();

        for (index, descriptor) in descriptors.iter().enumerate() {
            descriptor.validate_for_ordering().map_err(|message| {
                PassOrderingError::for_pass(
                    PassOrderingErrorKind::InvalidDescriptor,
                    descriptor.ordering_id().clone(),
                    message,
                )
            })?;

            let id = descriptor.ordering_id().clone();

            if indices.insert(id.clone(), index).is_some() {
                return Err(PassOrderingError::for_pass(
                    PassOrderingErrorKind::DuplicatePass,
                    id,
                    "the supplied descriptor collection contains \
                     duplicate pass identities",
                ));
            }
        }

        // Resolve dependency IDs to descriptor indices.
        //
        // Adjacency lists contain integer indices so graph traversal does
        // not repeatedly compare or clone pass IDs.
        let mut adjacency: Vec<Vec<usize>> = Vec::new();
        adjacency.try_reserve(pass_count).map_err(|error| {
            PassOrderingError::new(
                PassOrderingErrorKind::AllocationReservationFailed,
                format!("unable to reserve adjacency storage: {error}"),
            )
        })?;

        adjacency.resize_with(pass_count, Vec::new);

        let mut indegree = Vec::<usize>::new();
        indegree.try_reserve(pass_count).map_err(|error| {
            PassOrderingError::new(
                PassOrderingErrorKind::AllocationReservationFailed,
                format!("unable to reserve dependency counters: {error}"),
            )
        })?;
        indegree.resize(pass_count, 0);

        // Count outgoing edges first, allowing adjacency vectors to reserve
        // their required capacity before insertion.
        let mut outgoing_counts = Vec::<usize>::new();
        outgoing_counts
            .try_reserve(pass_count)
            .map_err(|error| {
                PassOrderingError::new(
                    PassOrderingErrorKind::AllocationReservationFailed,
                    format!("unable to reserve edge counters: {error}"),
                )
            })?;
        outgoing_counts.resize(pass_count, 0);

        for dependency in &self.dependencies {
            if dependency.prerequisite == dependency.dependent {
                return Err(PassOrderingError::for_pass(
                    PassOrderingErrorKind::SelfDependency,
                    dependency.prerequisite.clone(),
                    "a pass cannot depend on itself",
                ));
            }

            let Some(&prerequisite_index) =
                indices.get(&dependency.prerequisite)
            else {
                return Err(PassOrderingError::for_dependency(
                    PassOrderingErrorKind::MissingPass,
                    dependency.dependent.clone(),
                    dependency.prerequisite.clone(),
                    "the prerequisite pass is not present in the \
                     supplied descriptor collection",
                ));
            };

            let Some(&dependent_index) = indices.get(&dependency.dependent)
            else {
                return Err(PassOrderingError::for_dependency(
                    PassOrderingErrorKind::MissingPass,
                    dependency.dependent.clone(),
                    dependency.prerequisite.clone(),
                    "the dependent pass is not present in the \
                     supplied descriptor collection",
                ));
            };

            outgoing_counts[prerequisite_index] = outgoing_counts
                [prerequisite_index]
                .checked_add(1)
                .ok_or_else(|| {
                    PassOrderingError::for_pass(
                        PassOrderingErrorKind::RepresentationLimitExceeded,
                        dependency.prerequisite.clone(),
                        "outgoing dependency count exceeds the \
                         representable index range",
                    )
                })?;

            indegree[dependent_index] = indegree[dependent_index]
                .checked_add(1)
                .ok_or_else(|| {
                    PassOrderingError::for_pass(
                        PassOrderingErrorKind::RepresentationLimitExceeded,
                        dependency.dependent.clone(),
                        "incoming dependency count exceeds the \
                         representable index range",
                    )
                })?;
        }

        for (index, count) in outgoing_counts.into_iter().enumerate() {
            adjacency[index]
                .try_reserve(count)
                .map_err(|error| {
                    PassOrderingError::for_pass(
                        PassOrderingErrorKind::AllocationReservationFailed,
                        descriptors[index].ordering_id().clone(),
                        format!(
                            "unable to reserve outgoing dependency storage: \
                             {error}"
                        ),
                    )
                })?;
        }

        // Populate adjacency lists.
        for dependency in &self.dependencies {
            let prerequisite_index = *indices
                .get(&dependency.prerequisite)
                .expect("dependency was validated above");

            let dependent_index = *indices
                .get(&dependency.dependent)
                .expect("dependency was validated above");

            adjacency[prerequisite_index].push(dependent_index);
        }

        // Sort adjacency indices for stable traversal.
        for successors in &mut adjacency {
            successors.sort_unstable();
        }

        // Kahn's topological algorithm.
        //
        // The ready set is ordered by original descriptor position.
        // Therefore, when multiple passes are available, their original
        // registration order is preserved.
        let mut ready = BTreeSet::<(usize, usize)>::new();

        for (index, count) in indegree.iter().copied().enumerate() {
            if count == 0 {
                ready.insert((index, index));
            }
        }

        let mut ordered_ids = Vec::<Id>::new();
        ordered_ids.try_reserve(pass_count).map_err(|error| {
            PassOrderingError::new(
                PassOrderingErrorKind::AllocationReservationFailed,
                format!("unable to reserve ordered pass storage: {error}"),
            )
        })?;

        let mut levels = Vec::<Vec<Id>>::new();

        while !ready.is_empty() {
            // All currently ready nodes form one dependency generation.
            let current = std::mem::take(&mut ready);

            let mut level = Vec::<Id>::new();
            level.try_reserve(current.len()).map_err(|error| {
                PassOrderingError::new(
                    PassOrderingErrorKind::AllocationReservationFailed,
                    format!("unable to reserve dependency level: {error}"),
                )
            })?;

            let mut next = BTreeSet::<(usize, usize)>::new();

            for (_, node_index) in current {
                level.push(
                    descriptors[node_index].ordering_id().clone(),
                );

                for &successor in &adjacency[node_index] {
                    let remaining = indegree[successor]
                        .checked_sub(1)
                        .ok_or_else(|| {
                            PassOrderingError::new(
                                PassOrderingErrorKind::InvalidGraph,
                                "dependency counter underflow detected",
                            )
                        })?;

                    indegree[successor] = remaining;

                    if remaining == 0 {
                        next.insert((successor, successor));
                    }
                }
            }

            ordered_ids.extend(level.iter().cloned());
            levels.push(level);
            ready = next;
        }

        // Any remaining nonzero indegree means a cycle exists or a pass
        // is blocked by a cycle.
        if ordered_ids.len() != pass_count {
            let mut blocked_passes = Vec::new();

            for (index, remaining) in indegree.iter().copied().enumerate() {
                if remaining != 0 {
                    blocked_passes.push(
                        descriptors[index].ordering_id().clone(),
                    );
                }
            }

            return Err(PassOrderingError {
                kind: PassOrderingErrorKind::DependencyCycle,
                pass: blocked_passes.first().cloned(),
                dependency: None,
                blocked_passes,
                message: "the dependency graph contains a cycle or passes \
                          blocked by a dependency cycle"
                    .to_string(),
            });
        }

        // Build the final position index.
        let mut positions = BTreeMap::<Id, usize>::new();

        for (position, id) in ordered_ids.iter().cloned().enumerate() {
            positions.insert(id, position);
        }

        Ok(PassOrderingResult {
            ordered_ids,
            levels,
            positions,
        })
    }
}

// -----------------------------------------------------------------------------
// Unit tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    use crate::zuir::pass::contracts::{
        PassDescriptor,
        PassId,
        PassVersion,
    };

    fn id(value: &str) -> PassId {
        PassId::new(value).expect("valid test pass ID")
    }

    fn descriptor(value: &str) -> PassDescriptor {
        PassDescriptor::new(
            id(value),
            PassVersion::new(1, 0, 0),
            format!("Test pass {value}"),
        )
    }

    #[test]
    fn empty_input_produces_empty_order() {
        let ordering = PassOrdering::<PassId>::new();

        let result = ordering.order(&[]).expect("empty graph is valid");

        assert!(result.is_empty());
        assert_eq!(result.level_count(), 0);
        assert!(result.ordered_ids().is_empty());
    }

    #[test]
    fn preserves_input_order_without_dependencies() {
        let ordering = PassOrdering::<PassId>::new();

        let passes = vec![
            descriptor("core.third"),
            descriptor("core.first"),
            descriptor("core.second"),
        ];

        let result = ordering.order(&passes).expect("valid ordering");

        assert_eq!(
            result.ordered_ids(),
            &[
                id("core.third"),
                id("core.first"),
                id("core.second"),
            ]
        );
    }

    #[test]
    fn respects_explicit_dependency() {
        let mut ordering = PassOrdering::<PassId>::new();

        ordering
            .add_dependency(id("core.parse"), id("core.lower"))
            .expect("valid dependency");

        let passes = vec![
            descriptor("core.lower"),
            descriptor("core.parse"),
        ];

        let result = ordering.order(&passes).expect("valid ordering");

        assert_eq!(
            result.ordered_ids(),
            &[id("core.parse"), id("core.lower")]
        );
    }

    #[test]
    fn preserves_registration_order_among_ready_passes() {
        let mut ordering = PassOrdering::<PassId>::new();

        ordering
            .add_dependency(id("core.alpha"), id("core.omega"))
            .expect("valid dependency");

        let passes = vec![
            descriptor("core.omega"),
            descriptor("core.beta"),
            descriptor("core.alpha"),
            descriptor("core.gamma"),
        ];

        let result = ordering.order(&passes).expect("valid ordering");

        assert_eq!(
            result.ordered_ids(),
            &[
                id("core.beta"),
                id("core.alpha"),
                id("core.gamma"),
                id("core.omega"),
            ]
        );
    }

    #[test]
    fn calculates_dependency_levels() {
        let mut ordering = PassOrdering::<PassId>::new();

        ordering
            .add_dependency(id("core.parse"), id("core.semantic"))
            .expect("valid dependency");

        ordering
            .add_dependency(id("core.semantic"), id("core.lower"))
            .expect("valid dependency");

        let passes = vec![
            descriptor("core.lower"),
            descriptor("core.parse"),
            descriptor("core.semantic"),
            descriptor("core.report"),
        ];

        let result = ordering.order(&passes).expect("valid ordering");

        assert_eq!(
            result.levels(),
            &[
                vec![id("core.parse"), id("core.report")],
                vec![id("core.semantic")],
                vec![id("core.lower")],
            ]
        );

        assert_eq!(result.position(&id("core.parse")), Some(0));
        assert_eq!(result.position(&id("core.lower")), Some(3));
    }

    #[test]
    fn rejects_duplicate_pass_ids() {
        let ordering = PassOrdering::<PassId>::new();

        let passes = vec![
            descriptor("core.duplicate"),
            descriptor("core.duplicate"),
        ];

        let error = ordering.order(&passes).unwrap_err();

        assert_eq!(
            error.kind,
            PassOrderingErrorKind::DuplicatePass
        );
    }

    #[test]
    fn rejects_missing_prerequisite() {
        let mut ordering = PassOrdering::<PassId>::new();

        ordering
            .add_dependency(id("core.missing"), id("core.lower"))
            .expect("dependency declaration");

        let passes = vec![descriptor("core.lower")];

        let error = ordering.order(&passes).unwrap_err();

        assert_eq!(error.kind, PassOrderingErrorKind::MissingPass);
        assert_eq!(error.pass, Some(id("core.lower")));
    }

    #[test]
    fn rejects_missing_dependent() {
        let mut ordering = PassOrdering::<PassId>::new();

        ordering
            .add_dependency(id("core.parse"), id("core.missing"))
            .expect("dependency declaration");

        let passes = vec![descriptor("core.parse")];

        let error = ordering.order(&passes).unwrap_err();

        assert_eq!(error.kind, PassOrderingErrorKind::MissingPass);
    }

    #[test]
    fn rejects_self_dependency() {
        let mut ordering = PassOrdering::<PassId>::new();

        let error = ordering
            .add_dependency(id("core.parse"), id("core.parse"))
            .unwrap_err();

        assert_eq!(
            error.kind,
            PassOrderingErrorKind::SelfDependency
        );
    }

    #[test]
    fn rejects_dependency_cycles() {
        let mut ordering = PassOrdering::<PassId>::new();

        ordering
            .add_dependency(id("core.parse"), id("core.semantic"))
            .expect("dependency");

        ordering
            .add_dependency(id("core.semantic"), id("core.lower"))
            .expect("dependency");

        ordering
            .add_dependency(id("core.lower"), id("core.parse"))
            .expect("dependency");

        let passes = vec![
            descriptor("core.parse"),
            descriptor("core.semantic"),
            descriptor("core.lower"),
        ];

        let error = ordering.order(&passes).unwrap_err();

        assert_eq!(
            error.kind,
            PassOrderingErrorKind::DependencyCycle
        );

        assert_eq!(error.blocked_passes.len(), 3);
    }

    #[test]
    fn duplicate_dependency_is_idempotent() {
        let mut ordering = PassOrdering::<PassId>::new();

        assert!(
            ordering
                .add_dependency(id("core.parse"), id("core.lower"))
                .unwrap()
        );

        assert!(
            !ordering
                .add_dependency(id("core.parse"), id("core.lower"))
                .unwrap()
        );

        assert_eq!(ordering.dependency_count(), 1);
    }

    #[test]
    fn removes_existing_dependency() {
        let mut ordering = PassOrdering::<PassId>::new();

        ordering
            .add_dependency(id("core.parse"), id("core.lower"))
            .expect("dependency");

        assert!(
            ordering.remove_dependency(
                &id("core.parse"),
                &id("core.lower")
            )
        );

        assert!(ordering.is_empty());
    }

    #[test]
    fn canonical_descriptor_validation_is_enforced() {
        let ordering = PassOrdering::<PassId>::new();

        let mut invalid = descriptor("core.invalid");
        invalid.description.clear();

        let error = ordering.order(&[invalid]).unwrap_err();

        assert_eq!(
            error.kind,
            PassOrderingErrorKind::InvalidDescriptor
        );
    }
}
