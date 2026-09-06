//! Zamani Quantum Scheduling — Shared Test Fixtures
//!
//! This module contains deterministic, reusable workload descriptions for the
//! scheduling test suite.
//!
//! # Responsibility
//!
//! This file answers:
//!
//! > "How can every scheduling test construct the same classes of workloads
//! > without duplicating test-generation logic?"
//!
//! The fixture layer deliberately does NOT implement:
//!
//! - scheduling algorithms;
//! - resource allocation;
//! - dependency resolution;
//! - hardware discovery;
//! - routing;
//! - QEC;
//! - execution;
//! - optimization;
//! - benchmarking;
//! - production runtime state.
//!
//! It only describes deterministic test inputs.
//!
//! # Architectural position
//!
//! ```text
//! quantum::ir
//!      │
//!      │ canonical identities
//!      ▼
//! scheduling::tests::fixtures
//!      │
//!      ├──────────────► unit tests
//!      ├──────────────► integration tests
//!      ├──────────────► property tests
//!      ├──────────────► regression tests
//!      ├──────────────► scalability tests
//!      └──────────────► determinism tests
//! ```
//!
//! # Canonical identity rule
//!
//! Logical qubit identities MUST come from:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! ```
//!
//! This module therefore never creates a competing test-only `QubitId`.
//!
//! Scheduler operation identities use the canonical IR:
//!
//! ```text
//! crate::quantum::ir::core::identity::OperationId
//! ```
//!
//! The fixtures intentionally use those identities rather than inventing
//! scheduler-local quantum identities.
//!
//! # Scalability
//!
//! There are no production machine-size limits in this file.
//!
//! The fixture generators accept workload sizes supplied by the test calling
//! them. The requested size therefore belongs to the test workload, not to
//! the scheduling implementation.
//!
//! A test may generate:
//!
//! - zero operations;
//! - one operation;
//! - thousands of operations;
//! - millions of operations;
//! - any larger workload that the test environment can actually represent.
//!
//! No `MAX_QUBITS`, `MAX_OPERATIONS`, `MAX_RESOURCES`, or similar artificial
//! scheduler limit is encoded here.
//!
//! # Determinism
//!
//! All generated fixtures are deterministic.
//!
//! The generator does not use:
//!
//! - thread-local randomness;
//! - operating-system randomness;
//! - wall-clock time;
//! - global mutable state;
//! - hidden seeds.
//!
//! This makes failures reproducible.
//!
//! # Memory behaviour
//!
//! Fixtures use sparse representations:
//!
//! ```text
//! operations -> Vec<FixtureOperation>
//! dependencies -> Vec<FixtureDependency>
//! resources -> Vec<FixtureResource>
//! ```
//!
//! They do not construct a time-slot matrix such as:
//!
//! ```text
//! qubits × time × channels
//! ```
//!
//! This is intentional. Tests must exercise the scheduler's scalable
//! representation rather than teach tests to expect a non-scalable timeline.
//!
//! # Integration contract
//!
//! The fixture layer is deliberately one-way:
//!
//! ```text
//! FixtureCircuit
//!       │
//!       ├──► scheduling::adapters::ir
//!       ├──► scheduling::ir
//!       ├──► scheduling::planners
//!       └──► scheduling::verification
//! ```
//!
//! It must never import:
//!
//! - hardware providers;
//! - vendor SDKs;
//! - runtime executors;
//! - routing implementations;
//! - QEC decoders.
//!
//! Such systems should consume the fixture description through their existing
//! public adapters.
//!
//! # Rust contract
//!
//! - Rust 1.97 / 1.97.1
//! - Rust 2021 edition
//! - stable Rust
//! - no nightly features
//! - no unsafe code
//! - standard library only
//!
//! # Important design rule
//!
//! If a production scheduler test needs a new workload shape, add a fixture
//! constructor here instead of duplicating generation logic in several tests.
//!
//! If a test needs a new production scheduler type, do NOT add that type here.
//! Production types belong to their owning scheduling module.
//!
//! =============================================================================
//! Safety boundary
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::{BTreeMap, BTreeSet};

// =============================================================================
// Canonical repository identities
// =============================================================================

use crate::quantum::ir::core::identity::OperationId;
use crate::quantum::ir::qubit::QubitId;
use crate::quantum::scheduling::types::Duration;

// =============================================================================
// Fixture constants
// =============================================================================
//
// These are semantic fixture identifiers, not scheduler limits.
//
// In particular, none of these constants constrain how large a production
// schedule may be.
//

/// Stable resource identifier used by fixtures.
pub const FIXTURE_RESOURCE_START: u64 = 1;

/// Stable operation identifier origin used by fixtures.
///
/// `OperationId` remains the canonical repository-owned identity.
pub const FIXTURE_OPERATION_START: u64 = 1;

/// Stable logical-qubit identity origin used by fixtures.
pub const FIXTURE_QUBIT_START: u64 = 0;

/// Default abstract operation duration used by simple fixtures.
///
/// This is a test value only. It is NOT a hardware timing assumption.
pub const FIXTURE_DEFAULT_DURATION: u128 = 1;

// =============================================================================
// Fixture operation kind
// =============================================================================

/// Semantic classification used by scheduler tests.
///
/// This is intentionally smaller than the production quantum operation model.
/// Fixtures need only enough semantic information to express scheduling
/// constraints.
///
/// Production semantics remain owned by `quantum::ir`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FixtureOperationKind {
    /// A generic one-qubit operation.
    SingleQubit,

    /// A generic two-qubit operation.
    TwoQubit,

    /// A generic operation with an arbitrary number of operands.
    MultiQubit,

    /// Measurement.
    Measurement,

    /// Reset.
    Reset,

    /// Classical/zero-qubit scheduling event.
    Classical,

    /// Communication event.
    Communication,

    /// QEC-related event.
    Qec,

    /// Generic synchronization event.
    Synchronization,
}

// =============================================================================
// Fixture operation
// =============================================================================

/// A deterministic scheduler workload operation.
///
/// This is deliberately a *test description*, not a replacement for
/// `SchedulingOperation`.
///
/// Conversion into production scheduling representations belongs in the
/// appropriate adapter/test integration layer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FixtureOperation {
    /// Canonical IR operation identity.
    pub id: OperationId,

    /// Operation classification.
    pub kind: FixtureOperationKind,

    /// Canonical logical qubit operands.
    ///
    /// An empty vector is legal for classical, synchronization, and other
    /// zero-qubit events.
    pub qubits: Vec<QubitId>,

    /// Abstract duration.
    ///
    /// This is not a physical unit.
    pub duration: Duration,

    /// Stable fixture resource identifiers.
    ///
    /// Resource interpretation belongs to the resource adapter.
    pub resources: Vec<u64>,

    /// Whether this operation represents a dynamic/runtime event.
    pub dynamic: bool,

    /// Whether this operation is a measurement-producing event.
    pub produces_classical_result: bool,
}

impl FixtureOperation {
    /// Creates a generic operation.
    #[must_use]
    pub fn new(
        id: OperationId,
        kind: FixtureOperationKind,
        qubits: Vec<QubitId>,
        duration: Duration,
    ) -> Self {
        Self {
            id,
            kind,
            qubits,
            duration,
            resources: Vec::new(),
            dynamic: false,
            produces_classical_result: false,
        }
    }

    /// Adds a resource requirement.
    #[must_use]
    pub fn with_resource(mut self, resource_id: u64) -> Self {
        self.resources.push(resource_id);
        self.resources.sort_unstable();
        self.resources.dedup();
        self
    }

    /// Adds several resource requirements.
    #[must_use]
    pub fn with_resources<I>(mut self, resources: I) -> Self
    where
        I: IntoIterator<Item = u64>,
    {
        self.resources.extend(resources);
        self.resources.sort_unstable();
        self.resources.dedup();
        self
    }

    /// Marks the operation as dynamically resolved.
    #[must_use]
    pub const fn dynamic(mut self, value: bool) -> Self {
        self.dynamic = value;
        self
    }

    /// Marks the operation as producing a classical result.
    #[must_use]
    pub const fn produces_classical_result(mut self, value: bool) -> Self {
        self.produces_classical_result = value;
        self
    }

    /// Returns the operation's canonical identity.
    #[must_use]
    pub const fn id(&self) -> OperationId {
        self.id
    }

    /// Returns whether the operation uses a particular qubit.
    #[must_use]
    pub fn uses_qubit(&self, qubit: QubitId) -> bool {
        self.qubits.contains(&qubit)
    }

    /// Returns whether the operation uses a particular resource.
    #[must_use]
    pub fn uses_resource(&self, resource_id: u64) -> bool {
        self.resources.binary_search(&resource_id).is_ok()
    }
}

// =============================================================================
// Fixture dependency
// =============================================================================

/// A deterministic dependency edge between fixture operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FixtureDependency {
    /// Predecessor operation.
    pub predecessor: OperationId,

    /// Successor operation.
    pub successor: OperationId,
}

impl FixtureDependency {
    /// Creates a dependency edge.
    #[must_use]
    pub const fn new(predecessor: OperationId, successor: OperationId) -> Self {
        Self {
            predecessor,
            successor,
        }
    }
}

// =============================================================================
// Fixture resource
// =============================================================================

/// Abstract resource used by scheduling fixtures.
///
/// The production resource model remains owned by
/// `scheduling::resources`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FixtureResource {
    /// Stable test resource identity.
    pub id: u64,

    /// Capacity available to the fixture.
    pub capacity: u64,

    /// Whether simultaneous use is allowed up to `capacity`.
    pub shareable: bool,
}

impl FixtureResource {
    /// Creates a resource with the requested capacity.
    #[must_use]
    pub const fn new(id: u64, capacity: u64, shareable: bool) -> Self {
        Self {
            id,
            capacity,
            shareable,
        }
    }

    /// Creates an exclusive resource.
    #[must_use]
    pub const fn exclusive(id: u64) -> Self {
        Self {
            id,
            capacity: 1,
            shareable: false,
        }
    }

    /// Creates a capacity-limited shared resource.
    #[must_use]
    pub const fn shared(id: u64, capacity: u64) -> Self {
        Self {
            id,
            capacity,
            shareable: true,
        }
    }
}

// =============================================================================
// Fixture circuit
// =============================================================================

/// Complete deterministic scheduling workload description.
///
/// `FixtureCircuit` is the main reusable fixture object.
///
/// It deliberately stores semantic test information rather than constructing
/// scheduler implementation internals. This keeps the fixture contract stable
/// while planners, policies, resource calendars, and scheduling algorithms
/// evolve.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct FixtureCircuit {
    /// Schedulable operations.
    pub operations: Vec<FixtureOperation>,

    /// Explicit dependency edges.
    pub dependencies: Vec<FixtureDependency>,

    /// Resources exposed to the fixture workload.
    pub resources: Vec<FixtureResource>,
}

impl FixtureCircuit {
    /// Creates an empty fixture.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            operations: Vec::new(),
            dependencies: Vec::new(),
            resources: Vec::new(),
        }
    }

    /// Returns the number of operations.
    #[must_use]
    pub fn operation_count(&self) -> usize {
        self.operations.len()
    }

    /// Returns the number of dependency edges.
    #[must_use]
    pub fn dependency_count(&self) -> usize {
        self.dependencies.len()
    }

    /// Returns the number of resources.
    #[must_use]
    pub fn resource_count(&self) -> usize {
        self.resources.len()
    }

    /// Adds an operation.
    ///
    /// Duplicate operation identities are rejected.
    pub fn add_operation(
        &mut self,
        operation: FixtureOperation,
    ) -> Result<(), FixtureError> {
        if self.operations.iter().any(|existing| existing.id == operation.id) {
            return Err(FixtureError::DuplicateOperation(operation.id));
        }

        self.operations.push(operation);
        Ok(())
    }

    /// Adds a dependency after validating both endpoints.
    ///
    /// Self-dependencies are rejected because they cannot represent a valid
    /// scheduling DAG.
    pub fn add_dependency(
        &mut self,
        dependency: FixtureDependency,
    ) -> Result<(), FixtureError> {
        if dependency.predecessor == dependency.successor {
            return Err(FixtureError::SelfDependency(dependency.predecessor));
        }

        if !self.contains_operation(dependency.predecessor) {
            return Err(FixtureError::MissingOperation(dependency.predecessor));
        }

        if !self.contains_operation(dependency.successor) {
            return Err(FixtureError::MissingOperation(dependency.successor));
        }

        if self.dependencies.contains(&dependency) {
            return Err(FixtureError::DuplicateDependency(dependency));
        }

        self.dependencies.push(dependency);
        Ok(())
    }

    /// Adds a resource.
    ///
    /// Duplicate resource identifiers are rejected.
    pub fn add_resource(
        &mut self,
        resource: FixtureResource,
    ) -> Result<(), FixtureError> {
        if self
            .resources
            .iter()
            .any(|existing| existing.id == resource.id)
        {
            return Err(FixtureError::DuplicateResource(resource.id));
        }

        if resource.capacity == 0 {
            return Err(FixtureError::ZeroResourceCapacity(resource.id));
        }

        self.resources.push(resource);
        Ok(())
    }

    /// Returns whether an operation exists.
    #[must_use]
    pub fn contains_operation(&self, operation: OperationId) -> bool {
        self.operations.iter().any(|candidate| candidate.id == operation)
    }

    /// Returns whether a resource exists.
    #[must_use]
    pub fn contains_resource(&self, resource_id: u64) -> bool {
        self.resources
            .iter()
            .any(|candidate| candidate.id == resource_id)
    }

    /// Returns an operation by identity.
    #[must_use]
    pub fn operation(&self, operation: OperationId) -> Option<&FixtureOperation> {
        self.operations
            .iter()
            .find(|candidate| candidate.id == operation)
    }

    /// Returns all canonical qubit identities referenced by this fixture.
    #[must_use]
    pub fn qubits(&self) -> Vec<QubitId> {
        let mut qubits = BTreeSet::new();

        for operation in &self.operations {
            qubits.extend(operation.qubits.iter().copied());
        }

        qubits.into_iter().collect()
    }

    /// Returns a deterministic operation map.
    ///
    /// The returned map is useful for assertions and diagnostics because it
    /// does not depend on insertion order.
    #[must_use]
    pub fn operations_by_id(&self) -> BTreeMap<OperationId, &FixtureOperation> {
        self.operations
            .iter()
            .map(|operation| (operation.id, operation))
            .collect()
    }

    /// Returns a deterministic dependency list sorted by endpoints.
    #[must_use]
    pub fn sorted_dependencies(&self) -> Vec<FixtureDependency> {
        let mut dependencies = self.dependencies.clone();
        dependencies.sort_unstable();
        dependencies
    }

    /// Performs fixture-level structural validation.
    ///
    /// This is deliberately weaker than production schedule verification.
    /// It verifies that the fixture itself is well-formed before being passed
    /// into the real scheduler.
    pub fn validate(&self) -> Result<(), FixtureError> {
        let operation_ids: BTreeSet<_> =
            self.operations.iter().map(|operation| operation.id).collect();

        if operation_ids.len() != self.operations.len() {
            return Err(FixtureError::DuplicateOperationIdentity);
        }

        let resource_ids: BTreeSet<_> =
            self.resources.iter().map(|resource| resource.id).collect();

        if resource_ids.len() != self.resources.len() {
            return Err(FixtureError::DuplicateResourceIdentity);
        }

        for resource in &self.resources {
            if resource.capacity == 0 {
                return Err(FixtureError::ZeroResourceCapacity(resource.id));
            }
        }

        let mut dependencies = BTreeSet::new();

        for dependency in &self.dependencies {
            if dependency.predecessor == dependency.successor {
                return Err(FixtureError::SelfDependency(dependency.predecessor));
            }

            if !operation_ids.contains(&dependency.predecessor) {
                return Err(FixtureError::MissingOperation(dependency.predecessor));
            }

            if !operation_ids.contains(&dependency.successor) {
                return Err(FixtureError::MissingOperation(dependency.successor));
            }

            if !dependencies.insert(*dependency) {
                return Err(FixtureError::DuplicateDependency(*dependency));
            }
        }

        for operation in &self.operations {
            for resource in &operation.resources {
                if !resource_ids.contains(resource) {
                    return Err(FixtureError::MissingResource(*resource));
                }
            }
        }

        Ok(())
    }

    /// Returns whether the explicit dependency graph is acyclic.
    ///
    /// This uses iterative Kahn-style processing and therefore does not depend
    /// on call-stack depth.
    #[must_use]
    pub fn is_acyclic(&self) -> bool {
        let mut indegree: BTreeMap<OperationId, usize> = self
            .operations
            .iter()
            .map(|operation| (operation.id, 0))
            .collect();

        let mut outgoing: BTreeMap<OperationId, Vec<OperationId>> = BTreeMap::new();

        for dependency in &self.dependencies {
            let Some(value) = indegree.get_mut(&dependency.successor) else {
                return false;
            };

            *value += 1;

            outgoing
                .entry(dependency.predecessor)
                .or_default()
                .push(dependency.successor);
        }

        for successors in outgoing.values_mut() {
            successors.sort_unstable();
        }

        let mut ready = BTreeSet::new();

        for (&operation, &degree) in &indegree {
            if degree == 0 {
                ready.insert(operation);
            }
        }

        let mut visited = 0usize;

        while let Some(operation) = ready.pop_first() {
            visited += 1;

            if let Some(successors) = outgoing.get(&operation) {
                for successor in successors {
                    let Some(degree) = indegree.get_mut(successor) else {
                        return false;
                    };

                    *degree -= 1;

                    if *degree == 0 {
                        ready.insert(*successor);
                    }
                }
            }
        }

        visited == self.operations.len()
    }

    /// Produces a deterministic topological ordering of the fixture.
    ///
    /// Returns `None` if the fixture contains a cycle or invalid dependency.
    #[must_use]
    pub fn topological_order(&self) -> Option<Vec<OperationId>> {
        if self.validate().is_err() {
            return None;
        }

        let mut indegree: BTreeMap<OperationId, usize> = self
            .operations
            .iter()
            .map(|operation| (operation.id, 0))
            .collect();

        let mut outgoing: BTreeMap<OperationId, Vec<OperationId>> = BTreeMap::new();

        for dependency in &self.dependencies {
            *indegree.get_mut(&dependency.successor)? += 1;

            outgoing
                .entry(dependency.predecessor)
                .or_default()
                .push(dependency.successor);
        }

        for successors in outgoing.values_mut() {
            successors.sort_unstable();
        }

        let mut ready = BTreeSet::new();

        for (&operation, &degree) in &indegree {
            if degree == 0 {
                ready.insert(operation);
            }
        }

        let mut order = Vec::with_capacity(self.operations.len());

        while let Some(operation) = ready.pop_first() {
            order.push(operation);

            if let Some(successors) = outgoing.get(&operation) {
                for successor in successors {
                    let degree = indegree.get_mut(successor)?;

                    if *degree == 0 {
                        return None;
                    }

                    *degree -= 1;

                    if *degree == 0 {
                        ready.insert(*successor);
                    }
                }
            }
        }

        if order.len() == self.operations.len() {
            Some(order)
        } else {
            None
        }
    }

    /// Returns all dependency edges entering an operation.
    #[must_use]
    pub fn predecessors(&self, operation: OperationId) -> Vec<OperationId> {
        let mut result: Vec<_> = self
            .dependencies
            .iter()
            .filter_map(|dependency| {
                (dependency.successor == operation).then_some(dependency.predecessor)
            })
            .collect();

        result.sort_unstable();
        result
    }

    /// Returns all dependency edges leaving an operation.
    #[must_use]
    pub fn successors(&self, operation: OperationId) -> Vec<OperationId> {
        let mut result: Vec<_> = self
            .dependencies
            .iter()
            .filter_map(|dependency| {
                (dependency.predecessor == operation).then_some(dependency.successor)
            })
            .collect();

        result.sort_unstable();
        result
    }
}

// =============================================================================
// Fixture errors
// =============================================================================

/// Errors produced while constructing or validating a test fixture.
///
/// These are test-fixture errors, not production scheduling errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FixtureError {
    /// Two operations have the same identity.
    DuplicateOperation(OperationId),

    /// Duplicate operation identity discovered during validation.
    DuplicateOperationIdentity,

    /// A dependency refers to an operation that does not exist.
    MissingOperation(OperationId),

    /// A dependency refers to itself.
    SelfDependency(OperationId),

    /// The same dependency was inserted twice.
    DuplicateDependency(FixtureDependency),

    /// Two resources have the same identity.
    DuplicateResource(u64),

    /// Duplicate resource identity discovered during validation.
    DuplicateResourceIdentity,

    /// A resource has zero capacity.
    ZeroResourceCapacity(u64),

    /// An operation refers to an unknown resource.
    MissingResource(u64),
}

// =============================================================================
// Basic fixture constructors
// =============================================================================

/// Creates an empty workload.
///
/// This is the canonical zero-operation fixture.
#[must_use]
pub fn empty_fixture() -> FixtureCircuit {
    FixtureCircuit::new()
}

/// Creates one independent operation.
#[must_use]
pub fn single_operation_fixture() -> FixtureCircuit {
    let mut fixture = FixtureCircuit::new();

    let operation = FixtureOperation::new(
        OperationId::new(FIXTURE_OPERATION_START),
        FixtureOperationKind::SingleQubit,
        vec![QubitId::new(FIXTURE_QUBIT_START)],
        Duration::new(FIXTURE_DEFAULT_DURATION),
    );

    fixture
        .add_operation(operation)
        .expect("single-operation fixture must be valid");

    fixture
}

/// Creates `operation_count` independent single-qubit operations.
///
/// Each operation receives its own canonical logical qubit.
///
/// This is useful for measuring scheduler parallelism and verifying that
/// independent work is not artificially serialized.
#[must_use]
pub fn independent_operations(operation_count: usize) -> FixtureCircuit {
    let mut fixture = FixtureCircuit::new();

    for index in 0..operation_count {
        let index = u64::try_from(index)
            .expect("fixture operation count must fit in canonical test identity");

        let operation_id = OperationId::new(
            FIXTURE_OPERATION_START
                .checked_add(index)
                .expect("fixture operation identity overflow"),
        );

        let qubit = QubitId::new(
            FIXTURE_QUBIT_START
                .checked_add(index)
                .expect("fixture qubit identity overflow"),
        );

        fixture
            .add_operation(FixtureOperation::new(
                operation_id,
                FixtureOperationKind::SingleQubit,
                vec![qubit],
                Duration::new(FIXTURE_DEFAULT_DURATION),
            ))
            .expect("generated independent operation must be unique");
    }

    fixture
}

/// Creates a linear dependency chain.
///
/// ```text
/// 0 -> 1 -> 2 -> 3 -> ...
/// ```
///
/// Every operation uses the same canonical qubit. This creates maximum
/// precedence serialization while avoiding an artificial scheduler limit.
#[must_use]
pub fn linear_chain(operation_count: usize) -> FixtureCircuit {
    let mut fixture = FixtureCircuit::new();

    let qubit = QubitId::new(FIXTURE_QUBIT_START);

    let mut previous = None;

    for index in 0..operation_count {
        let index = u64::try_from(index)
            .expect("fixture operation count must fit in canonical test identity");

        let operation_id = OperationId::new(
            FIXTURE_OPERATION_START
                .checked_add(index)
                .expect("fixture operation identity overflow"),
        );

        fixture
            .add_operation(FixtureOperation::new(
                operation_id,
                FixtureOperationKind::SingleQubit,
                vec![qubit],
                Duration::new(FIXTURE_DEFAULT_DURATION),
            ))
            .expect("generated chain operation must be unique");

        if let Some(previous_id) = previous {
            fixture
                .add_dependency(FixtureDependency::new(previous_id, operation_id))
                .expect("generated chain dependency must be valid");
        }

        previous = Some(operation_id);
    }

    fixture
}

/// Creates a wide DAG with no dependencies.
///
/// This represents maximum available logical parallelism.
#[must_use]
pub fn wide_dag(width: usize) -> FixtureCircuit {
    independent_operations(width)
}

/// Creates a layered DAG.
///
/// Every operation in one layer depends on every operation in the previous
/// layer.
///
/// This provides a controlled mixture of parallelism and synchronization.
///
/// `layers == 0` or `width == 0` produces an empty fixture.
#[must_use]
pub fn layered_dag(layers: usize, width: usize) -> FixtureCircuit {
    let mut fixture = FixtureCircuit::new();

    if layers == 0 || width == 0 {
        return fixture;
    }

    let width_u64 =
        u64::try_from(width).expect("fixture width must fit in canonical test identity");

    let mut previous_layer = Vec::with_capacity(width);

    for layer in 0..layers {
        let layer_u64 =
            u64::try_from(layer).expect("fixture layer count must fit in canonical identity");

        let mut current_layer = Vec::with_capacity(width);

        for position in 0..width {
            let position_u64 = u64::try_from(position)
                .expect("fixture layer width must fit in canonical identity");

            let ordinal = layer_u64
                .checked_mul(width_u64)
                .and_then(|value| value.checked_add(position_u64))
                .expect("layered fixture identity overflow");

            let operation_id = OperationId::new(
                FIXTURE_OPERATION_START
                    .checked_add(ordinal)
                    .expect("fixture operation identity overflow"),
            );

            let qubit = QubitId::new(
                FIXTURE_QUBIT_START
                    .checked_add(position_u64)
                    .expect("fixture qubit identity overflow"),
            );

            fixture
                .add_operation(FixtureOperation::new(
                    operation_id,
                    FixtureOperationKind::SingleQubit,
                    vec![qubit],
                    Duration::new(FIXTURE_DEFAULT_DURATION),
                ))
                .expect("generated layered operation must be unique");

            current_layer.push(operation_id);
        }

        for &predecessor in &previous_layer {
            for &successor in &current_layer {
                fixture
                    .add_dependency(FixtureDependency::new(predecessor, successor))
                    .expect("generated layered dependency must be valid");
            }
        }

        previous_layer = current_layer;
    }

    fixture
}

/// Creates a diamond DAG.
///
/// ```text
///       A
///      / \
///     B   C
///      \ /
///       D
/// ```
///
/// This is a canonical test for dependency-preserving parallel scheduling.
#[must_use]
pub fn diamond() -> FixtureCircuit {
    let mut fixture = FixtureCircuit::new();

    let ids = [
        OperationId::new(FIXTURE_OPERATION_START),
        OperationId::new(FIXTURE_OPERATION_START + 1),
        OperationId::new(FIXTURE_OPERATION_START + 2),
        OperationId::new(FIXTURE_OPERATION_START + 3),
    ];

    for (index, operation_id) in ids.iter().copied().enumerate() {
        let qubit = QubitId::new(
            FIXTURE_QUBIT_START
                .checked_add(
                    u64::try_from(index).expect("diamond index must fit canonical identity"),
                )
                .expect("diamond qubit identity overflow"),
        );

        fixture
            .add_operation(FixtureOperation::new(
                operation_id,
                FixtureOperationKind::SingleQubit,
                vec![qubit],
                Duration::new(FIXTURE_DEFAULT_DURATION),
            ))
            .expect("diamond operation must be unique");
    }

    fixture
        .add_dependency(FixtureDependency::new(ids[0], ids[1]))
        .expect("diamond edge must be valid");

    fixture
        .add_dependency(FixtureDependency::new(ids[0], ids[2]))
        .expect("diamond edge must be valid");

    fixture
        .add_dependency(FixtureDependency::new(ids[1], ids[3]))
        .expect("diamond edge must be valid");

    fixture
        .add_dependency(FixtureDependency::new(ids[2], ids[3]))
        .expect("diamond edge must be valid");

    fixture
}

/// Creates a two-qubit interaction workload.
///
/// Operations alternate over adjacent qubit pairs.
#[must_use]
pub fn two_qubit_chain(operation_count: usize) -> FixtureCircuit {
    let mut fixture = FixtureCircuit::new();

    if operation_count == 0 {
        return fixture;
    }

    let qubit_count = operation_count
        .checked_add(1)
        .expect("fixture qubit count overflow");

    let mut previous = None;

    for index in 0..operation_count {
        let index_u64 =
            u64::try_from(index).expect("fixture operation count must fit canonical identity");

        let first = QubitId::new(
            FIXTURE_QUBIT_START
                .checked_add(index_u64)
                .expect("fixture qubit identity overflow"),
        );

        let second = QubitId::new(
            FIXTURE_QUBIT_START
                .checked_add(index_u64)
                .and_then(|value| value.checked_add(1))
                .expect("fixture qubit identity overflow"),
        );

        let operation_id = OperationId::new(
            FIXTURE_OPERATION_START
                .checked_add(index_u64)
                .expect("fixture operation identity overflow"),
        );

        let operation = FixtureOperation::new(
            operation_id,
            FixtureOperationKind::TwoQubit,
            vec![first, second],
            Duration::new(FIXTURE_DEFAULT_DURATION),
        );

        fixture
            .add_operation(operation)
            .expect("generated two-qubit operation must be unique");

        if let Some(previous_id) = previous {
            fixture
                .add_dependency(FixtureDependency::new(previous_id, operation_id))
                .expect("generated two-qubit dependency must be valid");
        }

        previous = Some(operation_id);
    }

    debug_assert_eq!(fixture.qubits().len(), qubit_count);

    fixture
}

/// Creates a measurement dependency pattern.
///
/// ```text
/// quantum operation
///       │
///       ▼
/// measurement
///       │
///       ▼
/// classical operation
/// ```
#[must_use]
pub fn measurement_feedback() -> FixtureCircuit {
    let mut fixture = FixtureCircuit::new();

    let qubit = QubitId::new(FIXTURE_QUBIT_START);

    let gate = OperationId::new(FIXTURE_OPERATION_START);
    let measurement = OperationId::new(FIXTURE_OPERATION_START + 1);
    let feedback = OperationId::new(FIXTURE_OPERATION_START + 2);

    fixture
        .add_operation(FixtureOperation::new(
            gate,
            FixtureOperationKind::SingleQubit,
            vec![qubit],
            Duration::new(FIXTURE_DEFAULT_DURATION),
        ))
        .expect("gate fixture must be valid");

    fixture
        .add_operation(
            FixtureOperation::new(
                measurement,
                FixtureOperationKind::Measurement,
                vec![qubit],
                Duration::new(FIXTURE_DEFAULT_DURATION),
            )
            .produces_classical_result(true),
        )
        .expect("measurement fixture must be valid");

    fixture
        .add_operation(
            FixtureOperation::new(
                feedback,
                FixtureOperationKind::Classical,
                Vec::new(),
                Duration::new(FIXTURE_DEFAULT_DURATION),
            )
            .dynamic(true),
        )
        .expect("feedback fixture must be valid");

    fixture
        .add_dependency(FixtureDependency::new(gate, measurement))
        .expect("measurement dependency must be valid");

    fixture
        .add_dependency(FixtureDependency::new(measurement, feedback))
        .expect("feedback dependency must be valid");

    fixture
}

/// Creates a workload containing measurement and reset operations.
#[must_use]
pub fn measurement_reset_sequence(operation_count: usize) -> FixtureCircuit {
    let mut fixture = FixtureCircuit::new();

    let qubit = QubitId::new(FIXTURE_QUBIT_START);
    let mut previous = None;

    for index in 0..operation_count {
        let index_u64 =
            u64::try_from(index).expect("fixture operation count must fit canonical identity");

        let operation_id = OperationId::new(
            FIXTURE_OPERATION_START
                .checked_add(index_u64)
                .expect("fixture operation identity overflow"),
        );

        let kind = match index % 3 {
            0 => FixtureOperationKind::SingleQubit,
            1 => FixtureOperationKind::Measurement,
            _ => FixtureOperationKind::Reset,
        };

        let operation = FixtureOperation::new(
            operation_id,
            kind,
            vec![qubit],
            Duration::new(FIXTURE_DEFAULT_DURATION),
        );

        fixture
            .add_operation(operation)
            .expect("generated measurement/reset operation must be unique");

        if let Some(previous_id) = previous {
            fixture
                .add_dependency(FixtureDependency::new(previous_id, operation_id))
                .expect("generated measurement/reset dependency must be valid");
        }

        previous = Some(operation_id);
    }

    fixture
}

/// Creates an exclusive-resource conflict workload.
///
/// All operations are otherwise independent, but every operation requires the
/// same exclusive resource. A correct scheduler must serialize them.
#[must_use]
pub fn exclusive_resource_conflict(operation_count: usize) -> FixtureCircuit {
    let mut fixture = independent_operations(operation_count);

    fixture
        .add_resource(FixtureResource::exclusive(FIXTURE_RESOURCE_START))
        .expect("exclusive fixture resource must be valid");

    for operation in &mut fixture.operations {
        operation
            .resources
            .push(FIXTURE_RESOURCE_START);
    }

    fixture
}

/// Creates a shared-resource workload.
///
/// Every operation uses the same resource, whose capacity determines the
/// maximum legal simultaneous occupancy.
#[must_use]
pub fn shared_resource_workload(
    operation_count: usize,
    capacity: u64,
) -> Result<FixtureCircuit, FixtureError> {
    if capacity == 0 {
        return Err(FixtureError::ZeroResourceCapacity(
            FIXTURE_RESOURCE_START,
        ));
    }

    let mut fixture = independent_operations(operation_count);

    fixture.add_resource(FixtureResource::shared(
        FIXTURE_RESOURCE_START,
        capacity,
    ))?;

    for operation in &mut fixture.operations {
        operation.resources.push(FIXTURE_RESOURCE_START);
    }

    Ok(fixture)
}

/// Creates a resource-aware workload containing independent operations and
/// several resource pools.
///
/// The number of resources is supplied by the caller and is not constrained by
/// this fixture layer.
#[must_use]
pub fn resource_pressure(
    operation_count: usize,
    resource_count: usize,
) -> FixtureCircuit {
    let mut fixture = independent_operations(operation_count);

    for index in 0..resource_count {
        let index_u64 =
            u64::try_from(index).expect("fixture resource count must fit canonical identity");

        let resource_id = FIXTURE_RESOURCE_START
            .checked_add(index_u64)
            .expect("fixture resource identity overflow");

        fixture
            .add_resource(FixtureResource::exclusive(resource_id))
            .expect("generated resource must be unique");
    }

    if resource_count == 0 {
        return fixture;
    }

    let resource_count_u64 =
        u64::try_from(resource_count).expect("fixture resource count must fit identity");

    for (index, operation) in fixture.operations.iter_mut().enumerate() {
        let operation_index =
            u64::try_from(index).expect("fixture operation count must fit identity");

        let resource_offset = operation_index % resource_count_u64;

        let resource_id = FIXTURE_RESOURCE_START
            .checked_add(resource_offset)
            .expect("fixture resource identity overflow");

        operation.resources.push(resource_id);
    }

    fixture
}

// =============================================================================
// Qubit fixtures
// =============================================================================

/// Returns canonical logical qubit identities.
///
/// This function intentionally returns the repository's
/// `quantum::ir::qubit::QubitId` rather than defining a fixture-specific
/// qubit identity.
#[must_use]
pub fn qubits(count: usize) -> Vec<QubitId> {
    (0..count)
        .map(|index| {
            let index = u64::try_from(index)
                .expect("fixture qubit count must fit canonical identity");

            QubitId::new(
                FIXTURE_QUBIT_START
                    .checked_add(index)
                    .expect("fixture qubit identity overflow"),
            )
        })
        .collect()
}

// =============================================================================
// Scaling fixtures
// =============================================================================

/// Named workload shape for scalability tests.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FixtureShape {
    /// Independent operations.
    Wide,

    /// Fully serialized chain.
    Linear,

    /// Layered parallel workload.
    Layered,

    /// Two-qubit interaction chain.
    TwoQubit,

    /// Measurement/feedback workload.
    Dynamic,

    /// Measurement/reset workload.
    MeasurementReset,

    /// Exclusive-resource contention.
    ExclusiveResource,

    /// Shared-resource contention.
    SharedResource,
}

/// Parameters for a generated scaling workload.
///
/// These are test parameters only. They are not scheduler limits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScalingProfile {
    /// Number of operations.
    pub operations: usize,

    /// Number of layers for layered workloads.
    pub layers: usize,

    /// Width for layered workloads.
    pub width: usize,

    /// Resource capacity for shared-resource workloads.
    pub resource_capacity: u64,

    /// Requested workload shape.
    pub shape: FixtureShape,
}

impl ScalingProfile {
    /// Creates an independent-operation scaling profile.
    #[must_use]
    pub const fn wide(operations: usize) -> Self {
        Self {
            operations,
            layers: 0,
            width: 0,
            resource_capacity: 1,
            shape: FixtureShape::Wide,
        }
    }

    /// Creates a linear-chain scaling profile.
    #[must_use]
    pub const fn linear(operations: usize) -> Self {
        Self {
            operations,
            layers: 0,
            width: 0,
            resource_capacity: 1,
            shape: FixtureShape::Linear,
        }
    }

    /// Creates a layered scaling profile.
    #[must_use]
    pub const fn layered(layers: usize, width: usize) -> Self {
        Self {
            operations: layers.saturating_mul(width),
            layers,
            width,
            resource_capacity: 1,
            shape: FixtureShape::Layered,
        }
    }

    /// Creates a two-qubit scaling profile.
    #[must_use]
    pub const fn two_qubit(operations: usize) -> Self {
        Self {
            operations,
            layers: 0,
            width: 0,
            resource_capacity: 1,
            shape: FixtureShape::TwoQubit,
        }
    }

    /// Creates a dynamic-circuit scaling profile.
    #[must_use]
    pub const fn dynamic(operations: usize) -> Self {
        Self {
            operations,
            layers: 0,
            width: 0,
            resource_capacity: 1,
            shape: FixtureShape::Dynamic,
        }
    }

    /// Creates a measurement/reset scaling profile.
    #[must_use]
    pub const fn measurement_reset(operations: usize) -> Self {
        Self {
            operations,
            layers: 0,
            width: 0,
            resource_capacity: 1,
            shape: FixtureShape::MeasurementReset,
        }
    }

    /// Creates an exclusive-resource profile.
    #[must_use]
    pub const fn exclusive_resource(operations: usize) -> Self {
        Self {
            operations,
            layers: 0,
            width: 0,
            resource_capacity: 1,
            shape: FixtureShape::ExclusiveResource,
        }
    }

    /// Creates a shared-resource profile.
    #[must_use]
    pub const fn shared_resource(operations: usize, capacity: u64) -> Self {
        Self {
            operations,
            layers: 0,
            width: 0,
            resource_capacity: capacity,
            shape: FixtureShape::SharedResource,
        }
    }

    /// Builds the fixture represented by this profile.
    pub fn build(self) -> Result<FixtureCircuit, FixtureError> {
        let fixture = match self.shape {
            FixtureShape::Wide => independent_operations(self.operations),

            FixtureShape::Linear => linear_chain(self.operations),

            FixtureShape::Layered => layered_dag(self.layers, self.width),

            FixtureShape::TwoQubit => two_qubit_chain(self.operations),

            FixtureShape::Dynamic => {
                let mut fixture = measurement_feedback();

                if self.operations > fixture.operation_count() {
                    let additional = self.operations - fixture.operation_count();

                    let mut extra = independent_operations(additional);

                    let offset = fixture
                        .operations
                        .last()
                        .map(|operation| operation.id)
                        .and_then(|id| u64::try_from(id.index()).ok());

                    if let Some(offset) = offset {
                        for operation in &mut extra.operations {
                            let value = u64::try_from(operation.id.index())
                                .expect("fixture identity must fit u64");

                            operation.id = OperationId::new(
                                offset
                                    .checked_add(value)
                                    .expect("dynamic fixture identity overflow"),
                            );
                        }
                    }

                    fixture.operations.extend(extra.operations);
                }

                fixture
            }

            FixtureShape::MeasurementReset => {
                measurement_reset_sequence(self.operations)
            }

            FixtureShape::ExclusiveResource => {
                exclusive_resource_conflict(self.operations)
            }

            FixtureShape::SharedResource => {
                shared_resource_workload(self.operations, self.resource_capacity)?
            }
        };

        fixture.validate()?;

        Ok(fixture)
    }
}

// =============================================================================
// Canonical workload families
// =============================================================================

/// Returns the smallest set of fixtures required by the core scheduling test
/// matrix.
///
/// This is useful when a test suite needs representative coverage without
/// introducing a machine-size assumption.
#[must_use]
pub fn canonical_fixture_set() -> Vec<FixtureCircuit> {
    vec![
        empty_fixture(),
        single_operation_fixture(),
        linear_chain(4),
        diamond(),
        wide_dag(4),
        layered_dag(3, 3),
        two_qubit_chain(4),
        measurement_feedback(),
        measurement_reset_sequence(6),
        exclusive_resource_conflict(4),
    ]
}

/// Returns deterministic scaling profiles for CI tests.
///
/// These values are intentionally modest because they describe CI workload,
/// not scheduler capacity.
///
/// A dedicated scalability benchmark may construct substantially larger
/// profiles at runtime.
#[must_use]
pub fn canonical_scaling_profiles() -> Vec<ScalingProfile> {
    vec![
        ScalingProfile::wide(1),
        ScalingProfile::wide(8),
        ScalingProfile::wide(64),
        ScalingProfile::linear(1),
        ScalingProfile::linear(8),
        ScalingProfile::linear(64),
        ScalingProfile::layered(4, 4),
        ScalingProfile::layered(8, 8),
        ScalingProfile::two_qubit(32),
        ScalingProfile::dynamic(8),
        ScalingProfile::measurement_reset(16),
        ScalingProfile::exclusive_resource(32),
        ScalingProfile::shared_resource(32, 4),
    ]
}

// =============================================================================
// Fixture assertions
// =============================================================================

/// Asserts that a fixture is structurally valid.
///
/// Panics with a descriptive message suitable for test failures.
pub fn assert_valid_fixture(fixture: &FixtureCircuit) {
    fixture
        .validate()
        .unwrap_or_else(|error| panic!("invalid scheduling fixture: {error:?}"));
}

/// Asserts that a fixture is a valid DAG.
pub fn assert_acyclic_fixture(fixture: &FixtureCircuit) {
    assert_valid_fixture(fixture);

    assert!(
        fixture.is_acyclic(),
        "fixture dependency graph unexpectedly contains a cycle"
    );

    let order = fixture
        .topological_order()
        .expect("acyclic fixture must have a topological ordering");

    assert_eq!(
        order.len(),
        fixture.operation_count(),
        "topological ordering must contain every fixture operation"
    );
}

/// Asserts that every dependency is represented in the supplied order.
pub fn assert_topological_order(
    fixture: &FixtureCircuit,
    order: &[OperationId],
) {
    assert_eq!(
        order.len(),
        fixture.operation_count(),
        "topological order length must equal operation count"
    );

    let positions: BTreeMap<OperationId, usize> = order
        .iter()
        .copied()
        .enumerate()
        .map(|(position, operation)| (operation, position))
        .collect();

    assert_eq!(
        positions.len(),
        order.len(),
        "topological order must not contain duplicate operation identities"
    );

    for operation in &fixture.operations {
        assert!(
            positions.contains_key(&operation.id),
            "operation {} is missing from supplied topological order",
            operation.id
        );
    }

    for dependency in &fixture.dependencies {
        let predecessor = positions
            .get(&dependency.predecessor)
            .expect("dependency predecessor must exist in topological order");

        let successor = positions
            .get(&dependency.successor)
            .expect("dependency successor must exist in topological order");

        assert!(
            predecessor < successor,
            "dependency {} -> {} is violated by supplied order",
            dependency.predecessor,
            dependency.successor
        );
    }
}

// =============================================================================
// Tests for the fixture layer itself
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_fixture_is_valid() {
        let fixture = empty_fixture();

        assert_valid_fixture(&fixture);
        assert_acyclic_fixture(&fixture);
        assert!(fixture.operations.is_empty());
        assert!(fixture.dependencies.is_empty());
        assert!(fixture.resources.is_empty());
    }

    #[test]
    fn single_operation_fixture_is_valid() {
        let fixture = single_operation_fixture();

        assert_valid_fixture(&fixture);
        assert_acyclic_fixture(&fixture);

        assert_eq!(fixture.operation_count(), 1);
        assert_eq!(fixture.qubits(), vec![QubitId::new(0)]);
    }

    #[test]
    fn linear_fixture_is_topologically_ordered() {
        let fixture = linear_chain(32);

        assert_acyclic_fixture(&fixture);

        let order = fixture
            .topological_order()
            .expect("linear fixture must be acyclic");

        assert_topological_order(&fixture, &order);
    }

    #[test]
    fn diamond_fixture_preserves_parallel_branching() {
        let fixture = diamond();

        assert_acyclic_fixture(&fixture);

        assert_eq!(fixture.operation_count(), 4);
        assert_eq!(fixture.dependency_count(), 4);

        let order = fixture
            .topological_order()
            .expect("diamond must be acyclic");

        assert_topological_order(&fixture, &order);
    }

    #[test]
    fn layered_fixture_is_acyclic() {
        let fixture = layered_dag(5, 5);

        assert_acyclic_fixture(&fixture);

        assert_eq!(fixture.operation_count(), 25);
        assert_eq!(fixture.qubits().len(), 5);
    }

    #[test]
    fn independent_operations_have_no_dependencies() {
        let fixture = independent_operations(128);

        assert_valid_fixture(&fixture);
        assert!(fixture.is_acyclic());

        assert_eq!(fixture.dependency_count(), 0);
        assert_eq!(fixture.operation_count(), 128);
        assert_eq!(fixture.qubits().len(), 128);
    }

    #[test]
    fn two_qubit_fixture_uses_canonical_qubit_identity() {
        let fixture = two_qubit_chain(8);

        assert_valid_fixture(&fixture);
        assert_acyclic_fixture(&fixture);

        assert_eq!(fixture.qubits().len(), 9);
    }

    #[test]
    fn measurement_feedback_has_classical_dependency() {
        let fixture = measurement_feedback();

        assert_valid_fixture(&fixture);
        assert_acyclic_fixture(&fixture);

        assert_eq!(fixture.operation_count(), 3);
        assert_eq!(fixture.dependency_count(), 2);

        let measurement = OperationId::new(FIXTURE_OPERATION_START + 1);
        let feedback = OperationId::new(FIXTURE_OPERATION_START + 2);

        assert_eq!(fixture.predecessors(feedback), vec![measurement]);
    }

    #[test]
    fn resource_conflict_fixture_has_one_exclusive_resource() {
        let fixture = exclusive_resource_conflict(16);

        assert_valid_fixture(&fixture);

        assert_eq!(fixture.resource_count(), 1);
        assert_eq!(fixture.operations.len(), 16);

        for operation in &fixture.operations {
            assert!(operation.uses_resource(FIXTURE_RESOURCE_START));
        }
    }

    #[test]
    fn shared_resource_capacity_is_preserved() {
        let fixture =
            shared_resource_workload(16, 4).expect("shared resource fixture must be valid");

        assert_valid_fixture(&fixture);

        assert_eq!(fixture.resources[0].capacity, 4);
        assert!(fixture.resources[0].shareable);
    }

    #[test]
    fn zero_resource_capacity_is_rejected() {
        let result = shared_resource_workload(1, 0);

        assert_eq!(
            result,
            Err(FixtureError::ZeroResourceCapacity(
                FIXTURE_RESOURCE_START
            ))
        );
    }

    #[test]
    fn duplicate_operation_is_rejected() {
        let mut fixture = FixtureCircuit::new();

        let operation = FixtureOperation::new(
            OperationId::new(1),
            FixtureOperationKind::SingleQubit,
            vec![QubitId::new(0)],
            Duration::new(1),
        );

        fixture
            .add_operation(operation.clone())
            .expect("first operation must succeed");

        assert_eq!(
            fixture.add_operation(operation),
            Err(FixtureError::DuplicateOperation(OperationId::new(1)))
        );
    }

    #[test]
    fn missing_dependency_endpoint_is_rejected() {
        let mut fixture = FixtureCircuit::new();

        fixture
            .add_operation(FixtureOperation::new(
                OperationId::new(1),
                FixtureOperationKind::SingleQubit,
                vec![QubitId::new(0)],
                Duration::new(1),
            ))
            .expect("operation must be valid");

        let result =
            fixture.add_dependency(FixtureDependency::new(
                OperationId::new(1),
                OperationId::new(2),
            ));

        assert_eq!(
            result,
            Err(FixtureError::MissingOperation(OperationId::new(2)))
        );
    }

    #[test]
    fn self_dependency_is_rejected() {
        let mut fixture = FixtureCircuit::new();

        fixture
            .add_operation(FixtureOperation::new(
                OperationId::new(1),
                FixtureOperationKind::SingleQubit,
                vec![QubitId::new(0)],
                Duration::new(1),
            ))
            .expect("operation must be valid");

        assert_eq!(
            fixture.add_dependency(FixtureDependency::new(
                OperationId::new(1),
                OperationId::new(1),
            )),
            Err(FixtureError::SelfDependency(OperationId::new(1)))
        );
    }

    #[test]
    fn deterministic_topological_order_is_stable() {
        let fixture = layered_dag(8, 8);

        let first = fixture
            .topological_order()
            .expect("fixture must be acyclic");

        let second = fixture
            .topological_order()
            .expect("fixture must be acyclic");

        assert_eq!(first, second);
    }

    #[test]
    fn canonical_fixture_set_is_valid() {
        for fixture in canonical_fixture_set() {
            assert_valid_fixture(&fixture);
            assert_acyclic_fixture(&fixture);
        }
    }

    #[test]
    fn canonical_scaling_profiles_are_valid() {
        for profile in canonical_scaling_profiles() {
            let fixture = profile.build().expect("canonical profile must build");

            assert_valid_fixture(&fixture);
            assert_acyclic_fixture(&fixture);
        }
    }

    #[test]
    fn fixture_qubits_are_unique_and_canonical() {
        let generated = qubits(1024);

        let unique: BTreeSet<_> = generated.iter().copied().collect();

        assert_eq!(generated.len(), unique.len());

        for (index, qubit) in generated.iter().copied().enumerate() {
            let expected =
                QubitId::new(u64::try_from(index).expect("test index must fit canonical ID"));

            assert_eq!(qubit, expected);
        }
    }

    #[test]
    fn scaling_fixture_does_not_encode_a_scheduler_limit() {
        let small = linear_chain(1);
        let larger = linear_chain(1_024);

        assert!(small.is_acyclic());
        assert!(larger.is_acyclic());

        assert_eq!(small.operation_count(), 1);
        assert_eq!(larger.operation_count(), 1_024);
    }

    #[test]
    fn topological_order_contains_all_operations_for_large_sparse_fixture() {
        let fixture = wide_dag(4_096);

        let order = fixture
            .topological_order()
            .expect("wide fixture must be acyclic");

        assert_topological_order(&fixture, &order);
    }
}