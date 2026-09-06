Worked for 24s

I inspected the current Zamani repository, including the existing scheduling tree, quantum::ir identity boundary, scheduling contracts, hardware/routing architecture, and Cargo.toml. The repository already contains much of the proposed scheduling directory structure, so this DESIGN.md should be treated as the normative architectural contract that freezes the responsibilities and integration boundaries before implementation continues.

One repository issue is outside the scheduler itself but must be corrected before the production build is considered valid: the current Cargo.toml contains rust-version = "1.97.1" or "1.97", which is not valid Cargo syntax; exactly one supported Rust version must be selected.  The repository's IR also explicitly establishes quantum::ir::qubit::QubitId / PhysicalQubitId as the canonical qubit identity boundary, so scheduling must not introduce competing qubit types.

Below is the complete proposed src/quantum/scheduling/DESIGN.md.

Zamani Quantum Scheduling

Production Architecture, Frozen Contracts, Integration, Scalability and Implementation Design

Path: "src/quantum/scheduling/DESIGN.md"

Subsystem: "crate::quantum::scheduling"

Language: Rust

Edition: Rust 2021

Supported compiler: Rust 1.97 or Rust 1.97.1, with exactly one version selected by repository policy

Safety: "#![forbid(unsafe_code)]"

Design status: Normative production architecture

Primary objective: Provide target-independent, resource-aware, timing-aware, dependency-aware, dynamically extensible quantum scheduling for Zamani.

---

1. Purpose

"crate::quantum::scheduling" is the scheduling layer of the Zamani quantum compiler and execution stack.

Its fundamental responsibility is:

«Given a semantically valid executable quantum representation, an execution-target description, resource availability, timing information, routing information, constraints, and an explicit scheduling policy, determine when operations may execute while preserving quantum-program semantics.»

Scheduling answers:

«WHEN can this operation execute?»

Scheduling does not answer:

«What does the quantum program mean?»

That belongs to:

crate::quantum::ir

Scheduling does not answer:

«Where should a logical operation execute?»

That belongs to:

crate::quantum::routing

Scheduling does not answer:

«What vendor-specific instruction, pulse, control sequence, or API executes the operation?»

That belongs to:

crate::quantum::hardware

and the target-specific lowering/backend layer.

Scheduling does not answer:

«How are quantum errors decoded?»

That belongs to the quantum error-correction subsystem.

The scheduler therefore sits between target-compatible executable representation and target-specific execution lowering.

---

2. Core Zamani Principle

Zamani quantum source programs are written against quantum semantics, not against the physical size or implementation details of one machine.

The intended model is:

Zamani program
      |
      v
canonical quantum IR
      |
      v
optimization
      |
      v
routing
      |
      v
scheduling
      |
      v
target-specific lowering
      |
      v
runtime
      |
      v
quantum target

The same source program may therefore be compiled for:

1-qubit target
2-qubit target
small QPU
large QPU
multi-chip system
multi-QPU system
distributed quantum computer
quantum network
future quantum architecture

without modifying the source program merely because the target changed.

Different targets may produce different:

- physical mappings;
- schedules;
- operation durations;
- resource reservations;
- communication plans;
- alignment;
- delays;
- calibration choices;
- execution times.

That is expected.

The invariant is:

same source semantics
        +
different target
        =
different valid specialization

while preserving the meaning of the computation.

---

3. Meaning of "From Atom to Everywhere"

The scheduling architecture must support the same conceptual model across multiple scales:

single operation
      |
single qubit
      |
small device
      |
large device
      |
multi-chip device
      |
multi-module QPU
      |
multi-QPU system
      |
quantum data center
      |
distributed quantum network
      |
future heterogeneous quantum infrastructure

"Infinity" means:

«The scheduler must contain no artificial finite architectural ceiling on the number of qubits, operations, resources, scheduling depth, topology size, QEC rounds, communication nodes, or execution duration.»

It does not mean physical hardware or computer memory is literally infinite.

A concrete invocation is bounded by:

- available address space;
- available memory;
- CPU capacity;
- compiler time;
- operating-system limits;
- target capacity;
- target capabilities;
- network capacity;
- storage;
- provider constraints;
- explicit caller limits;
- security limits;
- deadlines;
- cancellation.

Those are execution-resource constraints, not language-level machine-size constants.

---

4. Non-Negotiable Architectural Rules

The following rules are normative.

4.1 No artificial machine-size limits

Forbidden:

const MAX_QUBITS: usize = 1000;
const MAX_OPERATIONS: usize = 1_000_000;
const MAX_CHANNELS: usize = 64;
const MAX_QEC_ROUNDS: usize = 100;

unless the value is explicitly part of a caller-supplied safety/security policy rather than a scheduler architecture constant.

The scheduler must not silently impose such limits.

---

4.2 No fixed hardware topology

Forbidden:

4x4 grid
10x10 grid
127 qubits
133 qubits
fixed nearest-neighbour topology
fixed heavy-hex topology

Topology is supplied by routing/hardware.

---

4.3 No fixed gate arity

Do not assume:

one-qubit gate
two-qubit gate

only.

The scheduler must be able to represent operations with arbitrary operand/resource requirements.

---

4.4 No fixed timing unit

The scheduler must not assume:

nanoseconds
microseconds
picoseconds
1 ns clock
10 ns gate
fixed dt

Timing interpretation is target-specific.

---

4.5 No vendor logic in scheduling core

Forbidden in core scheduling modules:

IBM-specific scheduling logic
Google-specific scheduling logic
IonQ-specific scheduling logic
Quantinuum-specific scheduling logic
Rigetti-specific scheduling logic
vendor SDK calls
vendor credentials
vendor network calls

Those belong behind hardware adapters.

---

4.6 No global mutable scheduler state

Every scheduling invocation must own its state.

There must be no global:

CURRENT_SCHEDULE
GLOBAL_RESOURCE_CALENDAR
GLOBAL_HARDWARE
GLOBAL_SCHEDULER
GLOBAL_RNG
GLOBAL_PLUGIN_REGISTRY

---

4.7 No hidden randomness

Randomized scheduling algorithms must receive explicit randomness configuration.

A reproducible invocation must be determined by:

program
+
target snapshot
+
routing result
+
configuration
+
constraints
+
calibration snapshot
+
seed

---

4.8 No duplicate canonical identities

Scheduling must not create another:

QubitId
PhysicalQubitId
OperationId

where the canonical IR already owns that identity.

---

4.9 No unsafe Rust

The scheduling subsystem must use:

#![forbid(unsafe_code)]

and CI must reject unsafe code.

---

4.10 No semantic modification hidden inside scheduling

A scheduling pass may:

- assign time;
- reserve resources;
- insert explicitly requested scheduling delays;
- perform explicitly permitted scheduling transformations;
- align operations;
- add legal padding;
- perform explicitly configured timing-aware transformations.

It must not silently change quantum semantics.

---

5. Canonical Identity Boundary

The canonical qubit identity boundary is:

crate::quantum::ir::qubit

Scheduling must use:

crate::quantum::ir::qubit::QubitId
crate::quantum::ir::qubit::PhysicalQubitId

where applicable.

The repository explicitly requires new code to use these canonical identities rather than introducing another "QubitId".

The distinction is:

QubitId
    |
    +-- logical/canonical quantum-program identity

PhysicalQubitId
    |
    +-- physical target identity

OperationId
    |
    +-- canonical quantum IR operation identity

ScheduleId
    |
    +-- scheduler artifact identity

ReservationId
    |
    +-- scheduler resource reservation identity

DependencyId
    |
    +-- scheduler dependency-edge identity

Scheduling-specific IDs must never masquerade as quantum IR identities.

---

6. Logical-to-Physical Ownership

The correct flow is:

logical QubitId
       |
       v
routing
       |
       v
PhysicalQubitId
       |
       v
scheduling

Scheduling must never silently perform:

QubitId -> PhysicalQubitId

itself.

The mapping must already exist in the routing result or be explicitly supplied through the routing adapter.

---

7. Operation Identity Ownership

Canonical quantum operations belong to the canonical IR.

Scheduling must consume their canonical identity.

The scheduler must not create an unrelated semantic "OperationId".

Where the repository exposes the canonical operation identity through:

crate::quantum::ir

that identity remains authoritative.

Scheduler-local operation references may exist, but they must be explicitly described as scheduler views/references rather than new semantic operations.

---

8. Overall Architecture

The production architecture is:

                         Zamani source
                              |
                              v
                       quantum::frontend
                              |
                              v
                         quantum::ir
                              |
                              v
                       optimization
                              |
                              v
                          routing
                              |
                              v
                scheduling::adapters::ir
                              |
                              v
                    Scheduling IR View
                              |
            +-----------------+-----------------+
            |                 |                 |
            v                 v                 v
       dependencies       resources          timing
            |                 |                 |
            +-----------------+-----------------+
                              |
                              v
                         constraints
                              |
                              v
                           policy
                              |
                              v
                          planner
                              |
                              v
                         algorithm
                              |
                              v
                       candidate schedule
                              |
                              v
                        transformations
                              |
                              v
                          verification
                              |
                              v
                      schedule optimization
                              |
                              v
                      final verification
                              |
                              v
                    hardware/runtime lowering

---

9. Complete Directory Architecture

The intended scheduler tree is:

src/quantum/scheduling/
|
+-- DESIGN.md
+-- README.md
|
+-- mod.rs
+-- types.rs
+-- errors.rs
+-- config.rs
+-- limits.rs
+-- context.rs
+-- result.rs
|
+-- ir/
|   +-- mod.rs
|   +-- operation.rs
|   +-- dependency.rs
|   +-- graph.rs
|   +-- critical_path.rs
|
+-- resources/
|   +-- mod.rs
|   +-- resource.rs
|   +-- pool.rs
|   +-- reservation.rs
|   +-- calendar.rs
|   +-- availability.rs
|
+-- timing/
|   +-- mod.rs
|   +-- duration.rs
|   +-- time.rs
|   +-- resolution.rs
|   +-- alignment.rs
|   +-- windows.rs
|   +-- constraints.rs
|
+-- policies/
|   +-- mod.rs
|   +-- policy.rs
|   +-- asap.rs
|   +-- alap.rs
|   +-- priority.rs
|   +-- resource_aware.rs
|   +-- hybrid.rs
|
+-- planners/
|   +-- mod.rs
|   +-- planner.rs
|   +-- list.rs
|   +-- critical_path.rs
|   +-- resource_constrained.rs
|   +-- event.rs
|
+-- constraints/
|   +-- mod.rs
|   +-- constraint.rs
|   +-- qubit.rs
|   +-- channel.rs
|   +-- measurement.rs
|   +-- reset.rs
|   +-- control.rs
|   +-- communication.rs
|   +-- custom.rs
|
+-- transformations/
|   +-- mod.rs
|   +-- delays.rs
|   +-- alignment.rs
|   +-- padding.rs
|   +-- dynamical_decoupling.rs
|
+-- verification/
|   +-- mod.rs
|   +-- structural.rs
|   +-- dependency.rs
|   +-- resource.rs
|   +-- timing.rs
|   +-- semantic.rs
|   +-- verifier.rs
|
+-- optimization/
|   +-- mod.rs
|   +-- makespan.rs
|   +-- depth.rs
|   +-- idle_time.rs
|   +-- fidelity.rs
|   +-- energy.rs
|   +-- multi_objective.rs
|
+-- qec/
|   +-- mod.rs
|   +-- interface.rs
|   +-- syndrome.rs
|   +-- rounds.rs
|   +-- stabilizer.rs
|
+-- dynamic/
|   +-- mod.rs
|   +-- classical.rs
|   +-- conditional.rs
|   +-- feedback.rs
|   +-- runtime.rs
|
+-- distributed/
|   +-- mod.rs
|   +-- node.rs
|   +-- link.rs
|   +-- communication.rs
|   +-- network.rs
|
+-- adapters/
|   +-- mod.rs
|   +-- ir.rs
|   +-- hardware.rs
|   +-- routing.rs
|   +-- qec.rs
|
+-- serialization/
|   +-- mod.rs
|   +-- schema.rs
|   +-- encode.rs
|   +-- decode.rs
|
+-- diagnostics/
|   +-- mod.rs
|   +-- trace.rs
|   +-- explain.rs
|   +-- profile.rs
|
+-- algorithms/
|   +-- mod.rs
|   +-- asap.rs
|   +-- alap.rs
|   +-- list.rs
|   +-- cp.rs
|   +-- rcpsp.rs
|   +-- adaptive.rs
|
+-- plugins/
|   +-- mod.rs
|   +-- scheduler.rs
|   +-- registry.rs
|
+-- tests/
|   +-- mod.rs
|   +-- unit/
|   +-- integration/
|   +-- property/
|   +-- regression/
|   +-- scalability/
|   +-- determinism/
|   +-- fixtures/
|
+-- stabilizer_scheduler.rs

Not every directory needs to be implemented simultaneously.

However, the public contracts must be frozen before implementation of dependent modules.

---

10. Dependency Layers

The scheduler is divided into dependency layers.

Layer 0 — Foundational types

types.rs
errors.rs
limits.rs
timing/duration.rs
timing/time.rs
resources/resource.rs

These must be independently stable.

---

Layer 1 — Timing and resource foundations

timing/resolution.rs
timing/alignment.rs
timing/windows.rs
timing/constraints.rs

resources/pool.rs
resources/reservation.rs
resources/calendar.rs
resources/availability.rs

---

Layer 2 — Scheduling IR

ir/operation.rs
ir/dependency.rs
ir/graph.rs
ir/critical_path.rs

---

Layer 3 — Constraints

constraints/constraint.rs
constraints/qubit.rs
constraints/channel.rs
constraints/measurement.rs
constraints/reset.rs
constraints/control.rs
constraints/communication.rs
constraints/custom.rs

---

Layer 4 — Invocation model

config.rs
context.rs
result.rs

---

Layer 5 — Policy

policies/*

---

Layer 6 — Planning

planners/*

---

Layer 7 — Algorithms

algorithms/*

---

Layer 8 — Transformations and verification

transformations/*
verification/*
optimization/*

---

Layer 9 — Extended execution

dynamic/*
distributed/*
qec/*

---

Layer 10 — Integration

adapters/*
serialization/*
diagnostics/*
plugins/*

---

Layer 11 — Compatibility

stabilizer_scheduler.rs

---

Layer 12 — Composition

mod.rs

---

11. "types.rs"

Responsibility

"types.rs" owns foundational scheduler vocabulary.

It may define:

ScheduleId
DependencyId
ReservationId
EpochId
SchedulerSessionId
TimePoint
Duration
TimeInterval
Priority
Cost
Makespan
Slack
ScheduleStatus
SchedulingPhase

It must not define:

QubitId
PhysicalQubitId
canonical OperationId
canonical ResourceId

Those belong to their canonical owners.

Requirements

All semantic identities must be strongly typed.

Do not expose semantic identity as raw "usize".

Collection indices may internally use "usize", but a collection index must never become a public semantic identity merely because it is convenient.

Integration

Imports canonical identities only where necessary:

use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};

and canonical IR identities through the canonical IR path actually exported by the repository.

Arithmetic

Time arithmetic must be checked.

Forbidden:

wrapping_add
wrapping_sub

for semantic scheduling arithmetic.

Completion criterion

No later scheduler module may need to redefine:

ScheduleId
DependencyId
ReservationId
EpochId
TimePoint
Duration

---

12. "errors.rs"

Responsibility

Defines the complete scheduler error taxonomy.

At minimum:

SchedulingError
  InvalidInput
  InvalidDependencyGraph
  CycleDetected
  MissingDuration
  InvalidDuration
  ResourceUnavailable
  ResourceConflict
  TimingConflict
  AlignmentViolation
  ConstraintViolation
  UnsupportedOperation
  Unschedulable
  DeadlineExceeded
  CapacityExceeded
  VerificationFailed
  SerializationError
  PluginError
  Cancellation
  Internal

Error requirements

Errors must be structured.

Where applicable include:

operation
resource
dependency
time
duration
constraint
target
phase
reason

The scheduler must never make control decisions by parsing human-readable error strings.

Integration

All scheduler modules return or propagate the canonical scheduler error type.

No module invents a second incompatible error hierarchy.

Completion criterion

A new scheduler subsystem can express its failure through the existing error taxonomy without editing this file except for a genuinely new semantic error category.

---

13. "limits.rs"

Responsibility

Explicit invocation and deployment limits.

Possible limits:

maximum operations
maximum dependencies
maximum planning time
maximum memory
maximum schedule duration
maximum planner iterations
maximum parallel workers
maximum diagnostics
deadline
cancellation

All are optional.

Critical distinction

These are:

caller policy
deployment policy
security policy
execution policy

They are not:

Zamani quantum-machine limits

Forbidden

MAX_QUBITS
MAX_GATES
MAX_CHANNELS
MAX_ROUNDS

as universal scheduler constants.

Completion criterion

A user can impose a limit without recompiling Zamani.

---

14. "config.rs"

Responsibility

Defines immutable scheduling configuration.

Conceptual contents:

SchedulingConfig
    policy
    objective
    deterministic
    seed
    limits
    timing mode
    verification mode
    transformation policy
    optimization configuration
    parallelism
    distributed configuration
    diagnostics configuration

Requirements

No hardware discovery.

No global mutable configuration.

No vendor defaults.

No fixed machine size.

Integration

Consumes policy/objective types from scheduler modules.

It must not import concrete hardware providers.

Completion criterion

A scheduler invocation can be completely described by configuration plus context without hidden process-wide state.

---

15. "context.rs"

Responsibility

Immutable snapshot of everything required to make one scheduling decision.

Conceptually:

SchedulingContext
    program
    target
    routing result
    timing model
    resource model
    calibration snapshot
    availability snapshot
    constraints
    policy
    objective
    reproducibility context
    limits
    cancellation/deadline

Critical property

The scheduler operates against a snapshot.

Hardware must not spontaneously change underneath a supposedly deterministic schedule without that change being represented as a new target/availability snapshot or dynamic event.

Integration

quantum::ir
quantum::routing
quantum::hardware
quantum::zqn
quantum::error_correction
        |
        v
scheduling adapters
        |
        v
SchedulingContext

The context must not itself perform hardware discovery.

---

16. "result.rs"

Responsibility

Canonical scheduler output.

A production result must contain sufficient information to:

- execute;
- inspect;
- verify;
- benchmark;
- serialize;
- reproduce;
- diagnose.

Conceptual contents:

ScheduleResult
    schedule
    operation timings
    resource reservations
    makespan
    depth
    idle intervals
    critical path
    objective metrics
    verification report
    provenance
    diagnostics
    reproducibility metadata

Requirements

A schedule result must preserve source operation identity.

It must be possible to answer:

Which source operation produced this scheduled operation?

Integration

Consumed by:

hardware lowering
runtime
benchmarking
diagnostics
serialization
verification
compiler

---

17. "ir/operation.rs"

Responsibility

Defines the scheduler's internal operation view.

This is not a second quantum semantic IR.

It should represent scheduling-relevant information such as:

source operation identity
operands
physical operands where routing has already mapped them
duration
resource requirements
dependencies
timing windows
conditions
metadata
semantic classification

Canonical qubits

Where qubit identity is required:

use crate::quantum::ir::qubit::QubitId;

and:

use crate::quantum::ir::qubit::PhysicalQubitId;

must be used where applicable.

Integration

Input:

adapters::ir

Output:

dependency graph
resource analysis
timing analysis

Completion criterion

No scheduling algorithm needs to inspect arbitrary details of canonical IR structures directly.

---

18. "ir/dependency.rs"

Responsibility

Represent scheduling dependencies.

Support:

RAW
WAR
WAW
quantum dependency
classical dependency
measurement dependency
control dependency
resource dependency
communication dependency
explicit user dependency

Each dependency must identify:

predecessor
successor
dependency kind
optional latency
provenance

Completion criterion

Any scheduling dependency can be represented without modifying planner algorithms.

---

19. "ir/graph.rs"

Responsibility

Scalable scheduling graph.

Requirements:

- predecessor access;
- successor access;
- topological traversal;
- cycle detection;
- deterministic traversal;
- incremental construction;
- efficient ready-set calculation;
- no recursion requirement for huge graphs.

Scalability

Do not build a structure proportional to:

qubits × maximum time

Represent relationships as graph edges.

Baseline dependency analysis target:

O(V + E)

where:

V = operations
E = dependencies

---

20. "ir/critical_path.rs"

Responsibility

Calculate:

earliest start
earliest finish
latest start
latest finish
slack
critical path
critical-path length

It supports:

- ASAP;
- ALAP;
- priority scheduling;
- deadline analysis;
- resource-aware heuristics.

---

21. "resources/resource.rs"

Responsibility

Generic resource abstraction.

A resource may be:

exclusive
shared
capacity-limited
consumable
reusable
hierarchical
composite
time-dependent
conditionally available
distributed

Examples:

logical qubit
physical qubit
ancilla
control channel
drive channel
measurement channel
resonator
coupler
laser
microwave source
optical channel
classical processor
classical memory
communication link
synchronization resource
future target-defined resource

The scheduler must not know in advance how many exist.

---

22. "resources/pool.rs"

Responsibility

Represents interchangeable or grouped resources.

Examples:

measurement channels
control channels
readout channels
accelerators
classical processors
communication paths

A pool may expose:

capacity
availability
selection policy
resource compatibility

No fixed pool size.

---

23. "resources/reservation.rs"

Responsibility

Represents one resource reservation.

Conceptually:

Reservation
    reservation id
    operation id
    resource id
    start
    duration
    end
    mode
    provenance

A reservation must be validated before being committed.

---

24. "resources/calendar.rs"

Responsibility

Represent resource occupancy over time.

Must support:

- interval insertion;
- interval removal where legal;
- overlap detection;
- capacity checking;
- efficient next-available-time lookup;
- deterministic queries.

Avoid a timeline array with one slot per time unit.

Use interval/event structures.

---

25. "resources/availability.rs"

Responsibility

Represent target/resource availability.

States may include:

available
busy
disabled
degraded
unknown
maintenance
reserved

Availability can change.

A new snapshot/event must therefore be able to invalidate a schedule or trigger replanning.

---

26. "timing/duration.rs"

Responsibility

Represent operation durations without embedding physical units.

Support:

known duration
symbolic duration
target-derived duration
calibrated duration
interval/bounded duration
unknown duration

The scheduler must not silently turn an unknown duration into a guessed constant.

Unknown duration must produce either:

- a symbolic schedule;
- an explicitly conservative bound;
- a target resolution step;
- or a structured scheduling error,

according to configuration.

---

27. "timing/time.rs"

Responsibility

Define:

TimePoint
Duration
TimeInterval

with checked arithmetic.

Required invariants:

duration >= 0
end >= start
finish = start + duration

unless explicitly representing an unresolved symbolic interval.

---

28. "timing/resolution.rs"

Responsibility

Represent target timing resolution.

Possible target models:

continuous
integer ticks
rational units
sample periods
custom target-defined resolution

No scheduler-level assumption about:

dt
nanoseconds
pulse samples
clock frequency

---

29. "timing/alignment.rs"

Responsibility

Represent target alignment requirements.

Examples:

operation alignment
channel alignment
measurement alignment
frame alignment
pulse alignment
multi-qubit synchronization

The alignment rule is target supplied.

---

30. "timing/windows.rs"

Responsibility

Represent temporal windows.

Support:

release time
earliest start
latest start
earliest finish
latest finish
deadline
availability window

All windows must be checked for consistency.

---

31. "timing/constraints.rs"

Responsibility

Compose temporal constraints.

Examples:

A before B
A starts no earlier than T
B must complete by deadline
operations must align
measurement must complete before feedback
communication must arrive before consumer

---

32. "policies/policy.rs"

Responsibility

Defines what the scheduler prefers.

A policy is not the scheduler algorithm.

Possible policy goals:

ASAP
ALAP
critical path
deadline aware
resource aware
fidelity aware
communication aware
multi-objective

---

33. "policies/asap.rs"

Responsibility

ASAP policy.

Goal:

«Start each operation as early as all constraints permit.»

It must still respect:

- dependencies;
- resources;
- timing;
- alignment;
- availability;
- classical latency;
- communication;
- target capabilities.

ASAP is therefore not simply:

start = predecessor_finish

---

34. "policies/alap.rs"

Responsibility

ALAP policy.

Goal:

«Start operations as late as possible while satisfying all constraints and the relevant schedule boundary.»

Must account for:

- deadlines;
- downstream dependencies;
- resource constraints;
- timing windows;
- alignment.

---

35. "policies/priority.rs"

Responsibility

Priority computation.

Priority may depend on:

critical path
slack
deadline
resource scarcity
fidelity
communication
measurement readiness
user-defined priority

Tie-breaking must be deterministic when deterministic mode is enabled.

---

36. "policies/resource_aware.rs"

Responsibility

Prioritize operations according to resource pressure.

For example:

rare channel
rare coupler
scarce measurement resource
communication link
shared classical processor

Resource scarcity comes from the target model.

No fixed notion of "rare".

---

37. "policies/hybrid.rs"

Responsibility

Compose policy dimensions.

Examples:

ASAP + resource aware
ALAP + fidelity
critical path + communication aware
deadline + resource aware

Weights must come from configuration.

No hidden weighting constants.

---

38. "planners/planner.rs"

Responsibility

Defines the stable planner contract.

Conceptually:

Planner
    plan(context) -> candidate schedule

The contract must specify:

- required inputs;
- candidate-output guarantees;
- failure behavior;
- cancellation;
- determinism;
- resource ownership;
- diagnostics;
- complexity expectations.

A planner must not execute hardware.

---

39. "planners/list.rs"

Responsibility

General scalable list scheduling.

Conceptual flow:

dependency graph
      |
      v
ready set
      |
      v
priority evaluation
      |
      v
resource availability
      |
      v
choose operation
      |
      v
reserve resources
      |
      v
advance event frontier
      |
      v
update ready set

This should be the primary general-purpose scalable scheduler.

---

40. "planners/critical_path.rs"

Responsibility

Critical-path-oriented scheduling.

Uses:

critical path
slack
resource availability

It must remain generic.

---

41. "planners/resource_constrained.rs"

Responsibility

Resource-constrained project scheduling.

It must handle:

dependencies
+
capacity constraints
+
resource calendars
+
timing windows

Exact global optimization may be computationally expensive.

The implementation must therefore expose algorithm quality/strategy rather than pretending arbitrary instances are always cheaply optimizable.

---

42. "planners/event.rs"

Responsibility

Event-driven scheduling.

Events include:

operation completion
resource release
measurement completion
classical result available
communication completion
QEC round completion
dynamic condition resolution
resource availability change

Event-driven scheduling avoids repeatedly scanning the complete schedule.

---

43. "algorithms/asap.rs"

Concrete ASAP algorithm.

Must use:

policy
planner
resource model
timing model
constraints

It must not duplicate any of those abstractions.

---

44. "algorithms/alap.rs"

Concrete ALAP algorithm.

It must consume the planner/policy interfaces rather than define another scheduling framework.

---

45. "algorithms/list.rs"

Concrete list scheduling implementation.

Must use:

ready queue
priority policy
resource calendar
dependency graph
timing model

Tie-breaking must be deterministic when requested.

---

46. "algorithms/cp.rs"

Critical-path algorithm.

Must use "ir::critical_path".

It must not independently reconstruct dependency semantics.

---

47. "algorithms/rcpsp.rs"

Resource-constrained scheduling algorithm.

Must support:

- multiple resource types;
- capacities;
- calendars;
- precedence;
- time windows;
- target constraints.

---

48. "algorithms/adaptive.rs"

Adaptive scheduler selection.

It may choose algorithms according to:

graph density
resource pressure
parallelism
critical path
operation count
communication ratio
dynamic-control density
QEC structure
target characteristics

However, the decision must be based on the supplied context.

It must not use machine-specific constants as hidden selection rules.

---

49. "constraints/constraint.rs"

Responsibility

Generic constraint contract.

Every constraint must support, conceptually:

check
explain
priority/severity
provenance

A constraint must be able to explain why a schedule candidate violates it.

---

50. "constraints/qubit.rs"

Handles qubit-related constraints.

Examples:

exclusive occupancy
multi-qubit overlap
logical/physical identity consistency
reset dependency
measurement occupancy

Must use:

crate::quantum::ir::qubit::QubitId
crate::quantum::ir::qubit::PhysicalQubitId

where required.

It must not define new qubit identities.

---

51. "constraints/channel.rs"

Handles:

control channels
drive channels
readout channels
shared electronics
optical channels
other target-defined channels

Channel capacities come from the target.

---

52. "constraints/measurement.rs"

Handles:

measurement duration
measurement resources
readout conflicts
measurement grouping
classical result availability
measurement alignment

---

53. "constraints/reset.rs"

Handles:

reset dependencies
reset duration
resource occupancy
post-reset readiness

---

54. "constraints/control.rs"

Handles classical control.

Examples:

if condition then operation
measurement-dependent gate
branch readiness
feedback
control-flow dependencies

---

55. "constraints/communication.rs"

Handles communication-related scheduling.

Examples:

classical communication
entanglement generation
teleportation
remote operation
link occupancy
network latency
synchronization

Communication must be represented explicitly.

---

56. "constraints/custom.rs"

Provides extensibility for target- or research-specific constraints.

Custom constraints must use stable contracts.

They must not require modifying core scheduling algorithms.

---

57. "transformations/delays.rs"

Responsibility

Materialize explicit delays where required.

A delay may be needed because:

- the input program explicitly requested it;
- target semantics require it;
- alignment requires it;
- an optimization policy intentionally materializes idle time.

The transformation must preserve semantics.

Timing-sensitive operations cannot simply be reordered around explicit timing barriers.

---

58. "transformations/alignment.rs"

Converts a logically valid schedule into one satisfying target alignment constraints.

It must never invent alignment rules.

---

59. "transformations/padding.rs"

Inserts legal padding operations where the target model permits them.

Padding must preserve semantic equivalence.

---

60. "transformations/dynamical_decoupling.rs"

Optional scheduling transformation.

It must be isolated from core scheduling semantics.

Dynamical decoupling is not required for every target.

The transformation must be enabled explicitly by policy/configuration or target capability.

---

61. "verification/structural.rs"

Checks:

all required operations represented
no duplicate operation scheduling
no missing operation
source identities preserved

The verifier must expose canonical operation/qubit identities.

---

62. "verification/dependency.rs"

Checks:

for every A -> B:
finish(A) <= start(B)

including:

- quantum dependencies;
- classical dependencies;
- measurement dependencies;
- communication dependencies;
- control dependencies.

---

63. "verification/resource.rs"

Checks:

resource usage <= capacity

for every relevant interval.

For exclusive resources:

no overlap

For capacity resources:

sum(usage) <= capacity

---

64. "verification/timing.rs"

Checks:

duration validity
start/finish arithmetic
windows
deadlines
alignment
target timing resolution

---

65. "verification/semantic.rs"

This is one of the most important modules.

It must establish:

«Scheduling did not change the computation.»

At minimum verify preservation of:

operation identity
operation kind
operands
controls
measurement semantics
classical conditions
dependency semantics
explicit timing semantics

Scheduling may change when something executes, but not what it means.

---

66. "verification/verifier.rs"

Aggregates all verification layers.

A production schedule must pass:

structural
dependency
resource
timing
semantic

verification unless the caller explicitly requests analysis-only behavior.

Verification must be independently executable from the planner.

---

67. "optimization/makespan.rs"

Measures/minimizes:

schedule completion time

---

68. "optimization/depth.rs"

Measures scheduled depth.

Depth must not be confused with raw IR gate count.

It is target/schedule dependent.

---

69. "optimization/idle_time.rs"

Measures:

qubit idle time
resource idle time
critical idle periods

---

70. "optimization/fidelity.rs"

Consumes target/ZQN-provided fidelity estimates.

Scheduling must not invent a noise model.

The relationship is:

ZQN
 |
 +-- error estimates
 +-- drift
 +-- crosstalk
 +-- timing uncertainty
 +-- calibration uncertainty
 |
 v
scheduling objective

---

71. "optimization/energy.rs"

Optional energy objective.

Energy estimates are target-dependent.

No universal hardware energy constants.

---

72. "optimization/multi_objective.rs"

Supports objectives such as:

makespan
depth
idle time
fidelity
energy
communication overhead
resource cost

Weights must be explicit.

Example concept:

cost =
    w_makespan * makespan
  + w_idle * idle
  + w_error * estimated_error
  + w_energy * energy
  + w_communication * communication

The actual formulation must be configurable rather than hard-coded.

---

73. QEC Integration

QEC scheduling must remain separate from generic scheduling.

The correct architecture is:

QEC subsystem
      |
      v
QEC scheduling requirements
      |
      v
scheduling::qec
      |
      v
generic scheduler

QEC supplies:

syndrome dependencies
ancilla requirements
round structure
measurement requirements
feedback requirements
round spacing

The generic scheduler determines actual timing.

---

74. "qec/interface.rs"

Defines the QEC-to-scheduler contract.

It must support:

QEC operation requirements
QEC round requirements
ancilla resources
syndrome dependencies
measurement dependencies
feedback requirements

No QEC decoder implementation.

---

75. "qec/syndrome.rs"

Models scheduling-relevant syndrome extraction.

Must support:

ancilla preparation
stabilizer interaction
measurement
classical availability
decoder readiness

No fixed number of stabilizers.

---

76. "qec/rounds.rs"

Models QEC rounds.

Must support:

round identity
round dependencies
round spacing
round resources
round completion

No hard-coded:

distance = 3
rounds = 100
ancillas = 4

---

77. "qec/stabilizer.rs"

Contains stabilizer-specific scheduling integration.

The existing "stabilizer_scheduler.rs" must not become a second scheduler.

Instead:

stabilizer description
       |
       v
QEC requirements
       |
       v
generic scheduler
       |
       v
ScheduleResult

---

78. Dynamic Scheduling

Static DAG scheduling is insufficient for the full Zamani architecture.

The scheduler must support:

static DAG
+
conditional edges
+
classical dependencies
+
runtime events
+
feedback

This allows dynamic circuits and measurement-based control.

---

79. "dynamic/classical.rs"

Models classical computations that affect scheduling.

Examples:

measurement result
classical expression
decoder result
branch condition
control value

---

80. "dynamic/conditional.rs"

Models:

if condition
else
switch
conditional gate
conditional measurement
conditional reset

Conditions must be explicit.

---

81. "dynamic/feedback.rs"

Models:

measurement
    |
    v
classical processing
    |
    v
feedback readiness
    |
    v
quantum operation

Feedback latency must come from the target.

---

82. "dynamic/runtime.rs"

Represents operations whose exact schedule cannot be fully determined at compile time.

The scheduler must support:

static schedule
+
runtime scheduling points

rather than forcing every dynamic program into a false static representation.

---

83. Distributed Scheduling

The architecture must support:

one QPU
multi-chip
multi-module
multi-QPU
distributed quantum system
quantum network

Distributed operations must be explicit.

Examples:

entanglement generation
teleportation
remote operation
classical message
synchronization

---

84. "distributed/node.rs"

Represents a schedulable execution node.

A node may be:

chip
module
QPU
simulator
classical controller
network endpoint
future target-defined compute node

No fixed number of nodes.

---

85. "distributed/link.rs"

Represents communication/entanglement links.

Must expose target-provided:

capacity
latency
availability
resource constraints
fidelity information where applicable

---

86. "distributed/communication.rs"

Models communication events.

Must distinguish:

quantum communication
classical communication
synchronization
entanglement distribution
teleportation

---

87. "distributed/network.rs"

Represents the scheduling-relevant network graph.

It must support:

nodes
links
routing output
availability
communication constraints

The scheduler must not duplicate the routing subsystem's topology algorithms.

---

88. Adapter Architecture

Adapters are mandatory.

They prevent the scheduler core from becoming coupled to every other quantum subsystem.

Required adapters:

adapters::ir
adapters::routing
adapters::hardware
adapters::qec

---

89. "adapters/ir.rs"

Integration:

quantum::ir
      |
      v
adapters::ir
      |
      v
scheduling::ir

This is the only layer that should need detailed knowledge of canonical IR structure.

It must:

- preserve canonical operation identity;
- preserve canonical qubit identity;
- preserve semantics;
- extract scheduling-relevant data;
- reject unsupported/invalid input;
- preserve provenance.

---

90. "adapters/routing.rs"

Integration:

quantum::routing
       |
       v
routing result
       |
       v
adapters::routing
       |
       v
scheduler

Routing owns:

logical -> physical mapping
connectivity realization
placement
routing transformations

Scheduling owns:

WHEN

The scheduler must consume routing results rather than duplicate routing algorithms.

---

91. "adapters/hardware.rs"

Integration:

quantum::hardware
       |
       v
hardware target snapshot
       |
       v
adapters::hardware
       |
       v
SchedulingContext

Hardware supplies:

supported operations
qubits
physical resources
durations
timing resolution
alignment
channels
capacity
availability
calibration
communication

The scheduler must never directly call a vendor SDK.

---

92. "adapters/qec.rs"

Integration:

quantum::error_correction
       |
       v
QEC scheduling requirements
       |
       v
adapters::qec
       |
       v
scheduling::qec

QEC decoding remains outside scheduling.

---

93. Hardware Technology Independence

The scheduler must work without special cases for:

superconducting
trapped ion
neutral atom
photonic
spin
topological
annealing
hybrid
future architectures

The target model expresses differences.

The scheduler should see:

resources
+
capabilities
+
timing
+
constraints

not vendor identity.

---

94. Scheduling and Hardware Boundary

Hardware answers:

«Can this target execute this operation under these conditions?»

Scheduling answers:

«When should it execute?»

The interaction is:

operation
    |
    v
hardware capability check
    |
    v
resource requirement
    |
    v
timing requirement
    |
    v
scheduler

---

95. Scheduling and Routing Boundary

Routing answers:

«WHERE?»

Scheduling answers:

«WHEN?»

Hardware answers:

«CAN?»

This separation must remain intact.

---

96. Scheduling and ZQN Boundary

ZQN may provide:

gate error estimates
timing uncertainty
drift
crosstalk
readout quality
resource quality
calibration confidence

Scheduling may consume those values as objective inputs.

Scheduling must not recreate the ZQN model.

---

97. Serialization

"serialization/" is responsible for scheduler artifacts.

It must not become a second quantum IR.

---

98. "serialization/schema.rs"

Defines versioned scheduling schema.

The schema must include:

schema version
schedule identity
source provenance
target identity/version
operations
timings
reservations
constraints
objective
verification
diagnostics metadata

---

99. "serialization/encode.rs"

Serializes validated scheduler artifacts.

It must not serialize arbitrary executable objects without validation.

---

100. "serialization/decode.rs"

Deserialization must:

1. validate schema version;
2. validate identities;
3. validate time values;
4. validate intervals;
5. validate resource references;
6. validate dependencies;
7. validate capacities;
8. validate provenance;
9. reject malformed/inconsistent schedules.

Only then may an internal schedule be constructed.

---

101. Diagnostics

A production scheduler must explain scheduling decisions.

The question:

«Why was operation X delayed?»

must be answerable.

Possible reasons:

dependency incomplete
resource occupied
resource capacity exhausted
alignment requirement
measurement latency
classical processing latency
communication latency
deadline constraint
target availability
policy preference
QEC constraint
dynamic condition

---

102. "diagnostics/trace.rs"

Records scheduling events.

Examples:

operation became ready
operation considered
candidate start calculated
resource rejected
constraint rejected
reservation created
operation completed
resource released

Trace verbosity must be configurable.

---

103. "diagnostics/explain.rs"

Provides causal explanations.

Example conceptual output:

Operation O42 starts at T180.

It could not start at T120 because:

- dependency O37 completes at T150;
- channel R7 is occupied until T180;
- target alignment requires start at an aligned boundary.

---

104. "diagnostics/profile.rs"

Measures:

planning time
graph construction time
resource analysis time
verification time
optimization time
memory usage where measurable
operation count
dependency count
resource conflict count
planner iterations

These metrics integrate with benchmarking.

---

105. Plugins

Plugins allow external schedulers without changing core architecture.

Examples:

research scheduler
vendor-specific heuristic
ML scheduler
external optimizer
experimental planner

Plugins must implement stable interfaces.

They must not mutate global scheduler state.

---

106. "plugins/scheduler.rs"

Defines the scheduler/plugin contract.

A plugin receives:

SchedulingContext

and produces:

candidate ScheduleResult

subject to verification.

---

107. "plugins/registry.rs"

Registry must be scoped to an explicit scheduler/compiler context.

No global mutable registry.

Plugin selection must be deterministic when deterministic mode is enabled.

---

108. "stabilizer_scheduler.rs"

The existing stabilizer scheduler must be migrated into a compatibility facade.

It must not remain an independent scheduler.

Required architecture:

legacy stabilizer API
        |
        v
stabilizer_scheduler.rs
        |
        v
qec/stabilizer.rs
        |
        v
generic scheduling
        |
        v
ScheduleResult

The compatibility layer may translate legacy configuration.

It must not contain a separate scheduling engine.

---

109. Why the Existing Stabilizer Scheduler Must Change

The historical design is too specialized because it directly creates placeholder operations for a stabilizer round.

That makes it difficult to support:

different QEC codes
different code distances
different hardware
different ancilla layouts
different measurement systems
different timing
distributed QEC
future fault-tolerant architectures

The correct separation is:

QEC determines WHAT must happen.

Routing determines WHERE it happens.

Scheduling determines WHEN it happens.

Hardware determines HOW the target executes it.

---

110. "mod.rs"

"mod.rs" is the composition root.

It must contain only:

- module declarations;
- stable public exports;
- subsystem documentation;
- compatibility exposure where necessary.

It must not contain:

- algorithms;
- hardware discovery;
- routing;
- QEC decoding;
- resource calendars;
- timing algorithms;
- global state;
- network access;
- filesystem access;
- random-number generation.

The root must remain boring and stable.

---

111. Public API Shape

The central public operation should conceptually be:

schedule(program, target, configuration)

not:

schedule(program, 127 qubits, 8 channels, 100 ns)

The first is target-driven.

The second is machine hard-coding.

The production API should make target specialization explicit.

---

112. Scheduling Request

The canonical request should conceptually contain:

program
target snapshot
routing result
configuration
constraints
availability
calibration
reproducibility context

The scheduler must reject incomplete target information rather than inventing missing values.

---

113. Scheduling Execution Pipeline

The complete scheduling pipeline is:

1. receive canonical executable program
2. validate input
3. consume routing result
4. consume target snapshot
5. build scheduling operation view
6. construct dependency graph
7. detect cycles
8. build resource requirements
9. resolve timing information
10. apply timing windows
11. load constraints
12. construct ready set
13. select scheduling policy
14. select planner/algorithm
15. produce candidate schedule
16. reserve resources
17. apply alignment
18. materialize explicit delays where required
19. apply permitted scheduling transformations
20. verify structure
21. verify dependencies
22. verify resources
23. verify timing
24. verify semantic preservation
25. optimize objective if requested
26. verify again
27. produce immutable ScheduleResult
28. pass result to hardware lowering/runtime

---

114. Dependency Graph Invariant

For every dependency:

A -> B

the final schedule must satisfy:

finish(A) <= start(B)

unless the dependency explicitly defines another legal synchronization rule.

---

115. Resource Invariant

For every exclusive resource:

interval(A) ∩ interval(B) = empty

For capacity-limited resources:

usage(t) <= capacity(t)

for every relevant time.

---

116. Timing Invariant

For a normal known-duration operation:

finish = start + duration

with checked arithmetic.

No overflow may silently wrap.

---

117. Alignment Invariant

Every operation subject to an alignment requirement must start/finish according to the target alignment model.

The scheduler must not approximate alignment.

---

118. Availability Invariant

An operation cannot consume a resource during an interval in which the target says that resource is:

disabled
maintenance
unavailable
reserved

unless an explicit target contract says otherwise.

---

119. Semantic Invariant

The final scheduled representation must preserve:

same quantum operations
same operands
same controls
same measurements
same conditions
same program meaning

Only scheduling properties may change.

---

120. Dynamic Circuit Invariant

For:

measure -> classical processing -> conditional operation

the conditional operation cannot start until the required classical result is available.

---

121. Distributed Invariant

A remote operation cannot start until all required:

quantum communication
entanglement
classical communication
synchronization

dependencies are satisfied.

---

122. Scalability Architecture

The scheduler must scale structurally rather than by increasing hard-coded constants.

Do not allocate:

qubits × time slots

as the basic schedule representation.

Prefer:

operation -> interval
resource -> interval set
dependency -> graph edge
event -> event structure

---

123. Memory Scalability

Avoid:

Vec<Vec<Operation>>

where the outer dimension represents every conceptual time slot.

Use sparse/event-based representations.

Memory should grow approximately with actual:

operations
dependencies
resources
reservations
events

rather than hypothetical empty time.

---

124. Graph Scalability

Dependency analysis should target:

O(V + E)

where practical.

Traversal should not require recursive stack depth proportional to circuit size.

Prefer iterative algorithms for potentially enormous graphs.

---

125. Resource Scalability

Resource models must support arbitrary capacity.

Example:

capacity = 1
capacity = 8
capacity = 1000
capacity = target supplied

No scheduler source code should change.

---

126. Qubit Scalability

No code should depend on:

qubit 0..127
qubit 0..1000

Qubit identity comes from:

quantum::ir::qubit

and target resources.

---

127. Operation Scalability

No universal maximum operation count.

A caller may impose:

max_operations = Some(...)

through explicit limits.

Without that policy, the scheduler should operate until actual resource constraints or representational limits are reached.

---

128. Scheduling Depth Scalability

Do not allocate a fixed-depth schedule array.

Depth is a result.

It must grow naturally with the schedule.

---

129. Parallel Scheduling

The architecture must support parallel planning.

Possible model:

dependency analysis
        |
        v
ready set
        |
        v
parallel candidate evaluation
        |
        v
deterministic arbitration
        |
        v
resource reservation

Parallel evaluation must not create race-dependent schedules.

---

130. Deterministic Mode

When:

deterministic = true

the scheduler must ensure deterministic:

- iteration;
- priority tie-breaking;
- resource selection;
- graph traversal;
- plugin selection;
- randomized algorithm seed usage.

The intended invariant is:

same program
+
same target snapshot
+
same routing
+
same configuration
+
same calibration snapshot
+
same seed
=
same schedule

unless target data explicitly represents dynamic state.

---

131. Randomized Mode

Randomized algorithms must receive explicit randomness state.

No hidden:

thread_rng()

inside a deterministic path.

The seed must be included in reproducibility metadata.

---

132. Incremental Scheduling

The architecture should support rescheduling after:

resource failure
calibration change
availability change
runtime measurement
communication delay
dynamic branch

A changed epoch should be represented by:

EpochId

or equivalent scheduler context version.

---

133. Dynamic Replanning

When a runtime event invalidates part of a schedule:

old schedule
     |
     v
event
     |
     v
affected region
     |
     v
incremental replanning
     |
     v
new schedule epoch

The scheduler should avoid recomputing unrelated work when an incremental algorithm is available.

---

134. Scheduling Objective

The scheduler must support multiple objectives.

Possible objective hierarchy:

feasibility
    >
semantic correctness
    >
hard constraints
    >
deadline
    >
user objective
    >
secondary optimization

Correctness and hard constraints must never be traded away merely to improve makespan.

---

135. Exact Versus Heuristic Algorithms

The architecture must distinguish:

exact
heuristic
approximate
deterministic heuristic
stochastic
adaptive

Global optimal resource-constrained scheduling may be computationally difficult.

The system must therefore expose:

algorithm used
quality metrics
objective value
termination reason

rather than claiming universal optimality.

---

136. Schedule Quality Metrics

Every production result should be able to report:

makespan
depth
critical path
parallelism
resource utilization
idle time
resource contention
communication overhead
alignment overhead
inserted delays
estimated fidelity
estimated energy
objective score
planning time
verification time

---

137. Benchmarking Integration

The scheduling result must be consumable by:

quantum::benchmarking

Benchmarking should not need to know the internal scheduler implementation.

It should be able to consume:

makespan
depth
resource utilization
idle time
communication overhead
verification status
planning cost

---

138. Compiler Integration

The compiler pipeline must eventually be:

Zamani source
     |
     v
frontend
     |
     v
quantum::ir
     |
     v
optimization
     |
     v
routing
     |
     v
scheduling
     |
     v
hardware lowering
     |
     v
runtime

No circular dependency.

---

139. Runtime Integration

Runtime consumes a verified schedule.

Runtime must not reinterpret scheduler semantics.

Runtime responsibilities include:

execution
device interaction
job submission
result collection
runtime events
dynamic feedback

Scheduler responsibilities remain:

when

---

140. Hardware Integration

The hardware subsystem provides a target snapshot.

The scheduler does not own:

authentication
provider credentials
QPU connections
network sessions
backend job submission
hardware discovery

---

141. Routing Integration

Routing must finish the logical-to-physical problem before ordinary physical scheduling.

Conceptually:

logical program
       |
       v
routing
       |
       v
mapped executable program
       |
       v
scheduling

Scheduling may consume routing metadata but must not silently reroute operations.

---

142. QEC Integration

The complete fault-tolerant pipeline should support:

logical program
       |
       v
QEC/fault-tolerant lowering
       |
       v
routing
       |
       v
scheduling
       |
       v
hardware

QEC scheduling requirements must be explicit.

---

143. Serialization and Reproducibility

A serialized schedule must identify:

program provenance
target snapshot
routing provenance
scheduler version
configuration
algorithm
seed
timing model
resource model
calibration snapshot
verification status

This makes schedules reproducible and auditable.

---

144. Security Requirements

The scheduler must:

- avoid unsafe Rust;
- validate deserialized data;
- avoid arbitrary code execution;
- avoid untrusted plugin execution unless explicitly authorized;
- enforce caller resource limits;
- avoid unbounded diagnostic output;
- detect arithmetic overflow;
- reject malformed graph structures;
- reject invalid resource references.

---

145. Failure Handling

A scheduler must fail closed for invalid production schedules.

Examples:

missing duration
unsupported operation
unknown required resource
cycle
deadline impossible
capacity exceeded
invalid timing
invalid target capability
semantic verification failure

must produce structured errors.

Never silently produce an invalid schedule.

---

146. Cancellation

Long-running scheduling must support cancellation.

Cancellation must be checked at defined safe points such as:

graph construction
planner iterations
resource search
optimization
verification
serialization

Cancellation must produce a distinct result/error rather than an apparently successful incomplete schedule.

---

147. Deadline Handling

A scheduling deadline must be distinct from the execution deadline when necessary.

The system should distinguish:

planning deadline
execution deadline
operation deadline
resource availability deadline

---

148. Target Snapshot Consistency

All target information used during one deterministic scheduling invocation should come from a coherent snapshot.

Do not mix:

old topology
new calibration
old timing
new availability

unless the context explicitly models such versions.

---

149. Calibration Integration

Calibration belongs to hardware/ZQN.

Scheduling consumes calibration-derived information such as:

duration
error estimate
availability
alignment
resource capability

A calibration change must be represented as a new context/snapshot.

---

150. Resource Hierarchies

Resources may be hierarchical.

Example:

QPU
 |
 +-- module
      |
      +-- channel
      |
      +-- qubit

Reservation of a parent resource may constrain children.

The resource model must represent hierarchy rather than hard-coding it.

---

151. Composite Resources

One operation may require:

qubit A
qubit B
control channel C
measurement channel D
synchronization resource E

All must be atomically reservable.

Partial reservation must not leave inconsistent scheduler state.

---

152. Resource Transactions

Resource reservation should conceptually support:

begin candidate
reserve all required resources
validate
commit

If any requirement fails:

rollback candidate

The scheduler must never partially commit a candidate operation.

---

153. Event Model

The scheduler's event model should support:

operation ready
operation start
operation finish
resource acquired
resource released
measurement available
classical result available
communication available
resource availability changed
deadline reached
cancellation

This enables both static and dynamic scheduling.

---

154. Explicit Timing Semantics

Explicit timing is part of program semantics when the source representation expresses it.

The scheduler must not treat an explicit delay as meaningless whitespace.

A timing constraint may act as a barrier against otherwise legal reordering.

This is particularly important for OpenQASM-style explicit timing and for future Zamani timing semantics.

---

155. Variable Durations

The scheduler must support operations whose duration depends on:

target
parameter
calibration
runtime value
resource
mode
operation variant

No fixed duration constants in scheduler algorithms.

---

156. Unknown Durations

Unknown duration must never silently become zero.

Allowed behavior must be configuration-driven:

symbolic scheduling
bounded scheduling
target resolution
explicit failure

---

157. Zero-Duration Operations

Zero duration may be legal for:

compiler-level event
logical synchronization
metadata event
classical scheduling marker

The legality of a specific quantum operation remains determined by its semantics and target capability.

---

158. Negative Duration

Negative physical duration is invalid.

Negative values must be rejected.

---

159. Time Overflow

All schedule arithmetic must use checked operations.

If:

start + duration

cannot be represented:

SchedulingError::InvalidDuration

or the appropriate overflow-specific structured error must be returned.

Never wrap.

---

160. Classical Processing Latency

The scheduler must model:

measurement
     |
     v
classical processing
     |
     v
feedback

where target latency is non-zero.

This is necessary for dynamic circuits.

---

161. Communication Latency

Distributed scheduling must model:

operation A
   |
   v
communication
   |
   v
remote readiness
   |
   v
operation B

Communication latency is a scheduling dependency/resource.

---

162. Multi-QPU Scheduling

The same program may be partitioned across QPUs.

The scheduler must consume:

routing/partition result
network topology
communication resources

rather than hard-code a particular number of QPUs.

---

163. Simulation and Emulation

Scheduling should work for:

real hardware
simulator
emulator
test target

provided each exposes a compatible target model.

The scheduler must not special-case simulation.

---

164. Plugin Safety

Plugin code is outside the scheduler's trust boundary unless explicitly configured.

The core scheduler must verify plugin-generated schedules exactly like internally generated schedules.

A plugin cannot bypass:

dependency verification
resource verification
timing verification
semantic verification

---

165. Verification Is Independent

A scheduler algorithm must not be considered correct merely because it internally claims correctness.

The verification subsystem must independently inspect the final candidate.

This protects against:

- algorithm bugs;
- plugin bugs;
- future optimization bugs;
- serialization corruption;
- adapter mistakes.

---

166. Testing Architecture

Tests must be divided into:

tests/unit
tests/integration
tests/property
tests/regression
tests/scalability
tests/determinism
tests/fixtures

---

167. Unit Tests

Every foundational file must have direct tests.

Examples:

time arithmetic
duration arithmetic
ID ordering
resource capacity
reservation overlap
window validation
alignment
dependency construction
cycle detection
critical path
constraint evaluation

---

168. Integration Tests

Required paths:

IR -> scheduler
routing -> scheduler
hardware -> scheduler
QEC -> scheduler
ZQN -> scheduler
scheduler -> runtime
scheduler -> benchmarking
serialization -> scheduler

---

169. Property Tests

Important invariants:

no dependency violation
no exclusive resource overlap
capacity never exceeded
no invalid duration
schedule semantics preserved
serialization round trip preserves schedule
deterministic mode is reproducible

---

170. Regression Tests

Every discovered production bug gets a permanent test.

Regression tests must identify:

bug
input
expected behavior
failure condition
fixed invariant

---

171. Determinism Tests

Run the same:

program
target
configuration
seed

multiple times.

The resulting schedule must be identical in deterministic mode.

---

172. Scalability Tests

Scale independently:

operations
qubits
dependencies
resources
parallelism
QEC rounds
distributed nodes
communication edges

The tests must not reveal hidden fixed limits.

---

173. Required Functional Test Matrix

At minimum:

empty program
single operation
single qubit
many qubits
single-qubit operations
two-qubit operations
N-qubit operations
measure
reset
conditional
classical feedback
parallel operations
resource conflict
resource capacity > 1
zero duration
symbolic duration
unknown duration
alignment
deadline
release window
cycle
invalid graph
unavailable resource
dynamic availability
QEC round
distributed operation
large DAG
deterministic scheduling
randomized scheduling
serialization round-trip

---

174. Production Invariants

Every successful schedule must satisfy:

all operations represented
all required dependencies satisfied
all resources valid
all capacities respected
all timing valid
all alignment valid
all target capabilities valid
all deadlines valid
all semantic checks pass

---

175. Performance Requirements

The architecture must optimize for:

low memory overhead
sparse representations
incremental readiness
efficient resource lookup
event-driven progression
parallel candidate evaluation
minimal unnecessary graph copies
minimal unnecessary schedule copies

---

176. Large-Scale Graph Processing

Avoid repeated full-graph scans.

Prefer:

ready queue
dependency counters
event frontier
resource release events

The scheduler should process only the information whose state has changed whenever possible.

---

177. Deterministic Data Structures

Where deterministic ordering is required, use data structures/orderings that explicitly guarantee deterministic behavior.

Do not depend accidentally on:

hash iteration order
thread scheduling order
platform-specific ordering

---

178. Cross-Platform Requirements

The scheduler must behave consistently across:

Linux
macOS
Windows

where Rust and the surrounding target stack support them.

No platform-specific timing assumptions in core scheduling.

---

179. Thread Safety

Scheduler contexts should be immutable wherever practical.

Candidate schedules and resource calendars should be owned by a scheduling invocation.

No global mutable state.

Independent scheduling invocations should be able to run concurrently.

---

180. API Stability

Public types should be intentionally small.

Implementation details should remain private.

Avoid exposing:

internal graph storage
internal heap representation
internal resource calendar implementation
internal algorithm scratch state

as public API.

---

181. Versioning

Changes to public scheduling contracts must follow semantic versioning/project API policy.

Serialization schemas must have independent schema versions.

A schedule produced by an older schema must either:

decode successfully
migrate explicitly

or fail with a structured compatibility error.

---

182. Compatibility Strategy

Historical APIs should be adapted rather than duplicated.

Example:

legacy API
    |
    v
compatibility adapter
    |
    v
new scheduling API

This prevents legacy APIs from permanently controlling architecture.

---

183. File Completion Contract

Every scheduler source file is considered complete only when:

[ ] responsibility defined
[ ] public API defined
[ ] dependencies frozen
[ ] ownership boundaries frozen
[ ] error behavior defined
[ ] invariants implemented
[ ] deterministic behavior defined
[ ] scalability behavior defined
[ ] serialization behavior defined where applicable
[ ] thread-safety behavior defined
[ ] integration points defined
[ ] tests implemented
[ ] documentation complete
[ ] no unsafe code
[ ] no hidden machine assumptions

A later implementation file must not require reopening the completed file merely to accommodate normal downstream implementation.

If a genuinely new architectural concept appears, that is an architecture change and must be treated explicitly rather than through accidental coupling.

---

184. Implementation Order

The implementation order is intentionally dependency-first.

Stage 0 — Repository/toolchain correctness

Before scheduler implementation:

Cargo.toml
rust-version
rust-toolchain.toml if adopted
CI unsafe-code enforcement

The current "Cargo.toml" Rust-version expression must be corrected to one valid Cargo version value.

---

Stage 1 — Independent foundations

Implement completely:

types.rs
errors.rs
limits.rs
timing/duration.rs
timing/time.rs
resources/resource.rs

These should not depend on planners.

---

Stage 2 — Timing/resource infrastructure

Implement:

timing/resolution.rs
timing/alignment.rs
timing/windows.rs
timing/constraints.rs

resources/pool.rs
resources/reservation.rs
resources/calendar.rs
resources/availability.rs

---

Stage 3 — Scheduling IR

Implement:

ir/operation.rs
ir/dependency.rs
ir/graph.rs
ir/critical_path.rs

---

Stage 4 — Constraints

Implement:

constraints/constraint.rs
constraints/qubit.rs
constraints/channel.rs
constraints/measurement.rs
constraints/reset.rs
constraints/control.rs
constraints/communication.rs
constraints/custom.rs

---

Stage 5 — Invocation/result model

Implement:

config.rs
context.rs
result.rs

---

Stage 6 — Policies

Implement:

policies/policy.rs
policies/asap.rs
policies/alap.rs
policies/priority.rs
policies/resource_aware.rs
policies/hybrid.rs

---

Stage 7 — Planners

Implement:

planners/planner.rs
planners/list.rs
planners/critical_path.rs
planners/resource_constrained.rs
planners/event.rs

---

Stage 8 — Algorithms

Implement:

algorithms/asap.rs
algorithms/alap.rs
algorithms/list.rs
algorithms/cp.rs
algorithms/rcpsp.rs
algorithms/adaptive.rs

---

Stage 9 — Transformations

Implement:

transformations/delays.rs
transformations/alignment.rs
transformations/padding.rs
transformations/dynamical_decoupling.rs

---

Stage 10 — Verification

Implement:

verification/structural.rs
verification/dependency.rs
verification/resource.rs
verification/timing.rs
verification/semantic.rs
verification/verifier.rs

Verification must become a hard production gate before extended integrations.

---

Stage 11 — Optimization

Implement:

optimization/makespan.rs
optimization/depth.rs
optimization/idle_time.rs
optimization/fidelity.rs
optimization/energy.rs
optimization/multi_objective.rs

---

Stage 12 — QEC/dynamic/distributed

Implement:

qec/*
dynamic/*
distributed/*

---

Stage 13 — Adapters

Implement:

adapters/ir.rs
adapters/routing.rs
adapters/hardware.rs
adapters/qec.rs

---

Stage 14 — Serialization/diagnostics/plugins

Implement:

serialization/*
diagnostics/*
plugins/*

---

Stage 15 — Legacy compatibility

Migrate:

stabilizer_scheduler.rs

into the generic scheduler architecture.

---

Stage 16 — Composition root

Only after all contracts are stable:

mod.rs

should expose the completed architecture.

---

185. Required Integration Order

The final integrated system must become:

frontend
   |
   v
quantum::ir
   |
   v
optimization
   |
   v
routing
   |
   v
scheduling::adapters::ir
   |
   v
dependency/resource/timing analysis
   |
   v
constraints
   |
   v
policy
   |
   v
planner
   |
   v
algorithm
   |
   v
transformations
   |
   v
verification
   |
   v
optimization
   |
   v
final verification
   |
   v
hardware lowering
   |
   v
runtime

---

186. No Circular Dependencies

The intended dependency direction is:

IR
 |
 v
optimization
 |
 v
routing
 |
 v
scheduling
 |
 v
hardware/runtime

Integration with external subsystems occurs through adapters.

The scheduler must not import runtime implementation merely to calculate a schedule.

---

187. Scheduler Does Not Execute Hardware

This distinction is mandatory.

Scheduling:

constructs plan

Runtime:

executes plan

Hardware:

describes target and performs target-specific operations

---

188. Scheduler Does Not Own Quantum Semantics

Quantum semantics remain in:

quantum::ir

The scheduler may inspect semantics necessary to establish dependencies and resource requirements, but it does not redefine them.

---

189. Scheduler Does Not Own Routing

Routing remains:

logical -> physical

Scheduling remains:

physical executable operation -> time/resource interval

---

190. Scheduler Does Not Own QEC Decoding

QEC scheduling integration may represent:

syndrome
measurement
round
feedback

but decoding belongs elsewhere.

---

191. Scheduler Does Not Own Noise Models

Noise belongs to ZQN.

Scheduler consumes noise-derived estimates through an adapter/objective interface.

---

192. Scheduler Does Not Own Frontend Syntax

No:

Zamani parser
OpenQASM parser
lexer
AST parser

belongs here.

The frontend produces canonical IR.

---

193. OpenQASM/TIMING Compatibility

The scheduling model must be capable of representing timing semantics used by modern quantum IRs, including:

delays
durations
stretches
timing constraints
alignment
measurement/classical control
dynamic execution

The scheduler must not assume that a quantum circuit is only an unordered collection of gates.

---

194. Explicit Delay Semantics

If the source explicitly requests:

delay(...)

the scheduler must preserve its timing meaning.

It may optimize around the delay only when the relevant semantics explicitly permit it.

---

195. Target-Specific Timing

The source program must not need to specify:

X = 20 ns
CX = 40 ns
measure = 500 ns

unless the program deliberately expresses a target-specific timing requirement.

Otherwise target timing supplies those values.

---

196. Program Portability Guarantee

Zamani's promise is:

«Write the computation once.»

The scheduler provides:

target specialization

not identical physical timing.

Therefore:

same source
different target
different physical schedule
same computational semantics

is the intended behavior.

---

197. Example: Small Target

Program
  |
  v
IR
  |
  v
small target
  |
  v
routing
  |
  v
resource-aware scheduling
  |
  v
execution

No scheduler source code changes.

---

198. Example: Large Target

same Program
  |
  v
same IR
  |
  v
large target
  |
  v
different routing
  |
  v
larger resource model
  |
  v
parallel scheduling
  |
  v
execution

---

199. Example: Distributed Target

same Program
  |
  v
same IR
  |
  v
distributed routing
  |
  v
communication-aware scheduling
  |
  +-- QPU A
  +-- QPU B
  +-- QPU C
  |
  v
execution

The source program remains unchanged.

---

200. Resource Adaptation

The scheduler must adapt automatically to target resources.

Example:

Target A:
    4 qubits
    2 channels

Target B:
    1000 qubits
    80 channels

Target C:
    multiple QPUs
    distributed links

The scheduler reads those target descriptions.

It does not contain:

if qubits == 4
if qubits == 1000

---

201. Topology Adaptation

Topology comes from routing/hardware.

No scheduling algorithm may assume:

linear
grid
ring
heavy-hex
all-to-all

unless supplied as the actual target topology.

---

202. Technology Adaptation

Timing/resource requirements can differ radically between technologies.

The scheduler handles this through:

resource model
timing model
constraint model
target capability model

not technology-specific scheduler branches.

---

203. Future Architecture Adaptation

A future target may introduce a new resource:

quantum memory
photonic bus
new control mechanism
new communication resource
new synchronization resource

It should be possible to represent it as a resource/capability without rewriting every scheduler algorithm.

---

204. Extensibility Rule

When a new hardware concept appears, first ask:

Is it a resource?
Is it a capability?
Is it a timing rule?
Is it a constraint?
Is it an adapter concern?

Only add a new scheduler abstraction when none of those correctly represent it.

---

205. Avoid Abstraction Explosion

Not every hardware property deserves a dedicated scheduler module.

The scheduler core should remain centered on:

operation
dependency
resource
time
constraint
policy
schedule
verification

Everything else should integrate around those primitives.

---

206. Production Readiness Gate

The scheduler must not be marked production-ready until all of the following are true:

[ ] canonical identities used
[ ] no duplicate QubitId
[ ] no duplicate PhysicalQubitId
[ ] no hidden machine limits
[ ] timing model complete
[ ] resource model complete
[ ] dependency graph complete
[ ] cycle detection complete
[ ] ASAP complete
[ ] ALAP complete
[ ] list scheduling complete
[ ] resource-constrained scheduling complete
[ ] dynamic scheduling complete
[ ] explicit delay handling complete
[ ] alignment complete
[ ] verification complete
[ ] semantic verification complete
[ ] QEC integration complete
[ ] distributed integration complete
[ ] routing adapter complete
[ ] hardware adapter complete
[ ] IR adapter complete
[ ] ZQN integration complete
[ ] deterministic mode complete
[ ] reproducibility complete
[ ] serialization complete
[ ] diagnostics complete
[ ] plugin contract complete
[ ] cancellation complete
[ ] deadline handling complete
[ ] scalability tests complete
[ ] property tests complete
[ ] regression tests complete
[ ] integration tests complete
[ ] compiler integration complete
[ ] runtime integration complete
[ ] benchmarking integration complete
[ ] no unsafe code
[ ] CI enforcement complete

---

207. CI Requirements

CI must run at minimum:

cargo fmt --check
cargo check
cargo test
cargo clippy
cargo doc

plus project-specific checks.

Unsafe code must be rejected.

The scheduler should use:

#![forbid(unsafe_code)]

where appropriate at the crate/module boundary.

---

208. Rust Toolchain Requirement

The repository must choose exactly one valid Rust toolchain policy.

Do not use:

rust-version = "1.97.1" or "1.97"

This is invalid Cargo syntax.

Use one valid value, for example:

rust-version = "1.97.1"

if that exact toolchain is the repository's supported minimum, or:

rust-version = "1.97"

if the project policy selects that minimum.

The final selected version must be verified against the actual Rust toolchain used by CI.

---

209. Dependency Policy

The scheduling core should minimize dependencies.

Prefer the Rust standard library for:

collections
ordering
arithmetic
synchronization primitives where needed

Existing repository dependencies may be used where already established, but no dependency should be introduced merely to implement a simple scheduler primitive.

---

210. "serde" Policy

If scheduler types derive serialization traits, serialization semantics must still be governed by:

serialization/*

Do not make a serialization implementation detail dictate scheduler architecture.

---

211. Error Dependency Policy

All scheduler modules must use the canonical scheduler error contract.

Avoid:

String
Box<dyn Error>
panic!
unwrap()

for expected scheduling failures.

Panics should not be part of normal invalid-input behavior.

---

212. Panic-Free Production Scheduling

Invalid:

graph
resource
duration
timing
target
configuration

must result in structured errors.

Do not rely on:

assert!
unwrap()
expect()

for user-controlled scheduling inputs.

---

213. Arithmetic Safety

Every potentially overflowing operation involving:

time
duration
resource quantity
count
capacity
cost

must be checked or proven safe.

---

214. Provenance

Every scheduled operation should retain provenance sufficient to trace:

source IR operation
routing mapping
scheduler decision
transformation

This enables debugging and semantic verification.

---

215. Explainability

A production scheduler must be explainable.

For every significant delay, it should be possible to identify one or more causes:

dependency
resource
timing
alignment
policy
QEC
communication
availability
deadline

---

216. Observability

Diagnostics must be configurable.

Possible levels:

off
errors
summary
normal
verbose
trace

No unlimited trace by default.

---

217. Deterministic Diagnostics

Diagnostic ordering must also be deterministic in deterministic mode.

Otherwise identical schedules may produce different debugging output.

---

218. Resource Conflict Explanation

When an operation cannot start at a requested time, the scheduler should be able to report:

resource
current reservation
blocking operation
blocking interval
next available time

---

219. Unschedulable Explanation

If no legal schedule exists, the error should explain the principal cause where feasible.

Examples:

deadline impossible
required resource absent
capacity insufficient
timing window empty
dependency cycle
unsupported operation
communication unavailable

---

220. Partial Schedules

A partial schedule may be produced only when explicitly requested for:

analysis
debugging
incremental planning
interactive scheduling

A normal production execution request must not accidentally treat an incomplete schedule as executable.

---

221. Schedule Immutability

Once a "ScheduleResult" is finalized and verified, its externally observable schedule should be immutable.

Any modification should produce a new scheduling result or explicitly re-enter scheduling/verification.

---

222. Reservation Atomicity

A scheduled operation requiring multiple resources must reserve them consistently.

Example:

Qubit A
Qubit B
Control channel C
Readout channel D

must not produce a state where only A and B were reserved if C failed.

---

223. Multi-Resource Operations

Operations may require arbitrary numbers of resources.

The scheduler must not assume:

one operation = one resource

or:

one operation = two qubits

---

224. Resource Modes

Resources may have modes.

Example:

exclusive
shared
capacity
conditional

The operation's resource requirement must specify the mode required.

---

225. Capacity Changes Over Time

A resource may have:

capacity = 8 at T0
capacity = 4 at T1
capacity = 0 during maintenance

The calendar/availability model must support time-varying capacity.

---

226. Resource Failure

If a resource becomes unavailable during dynamic execution:

availability event
       |
       v
affected schedule region
       |
       v
replanning

The system must not continue blindly.

---

227. Scheduler Epochs

Each incremental/dynamic schedule state may have an:

EpochId

so that:

epoch 1
epoch 2
epoch 3

can be distinguished without changing canonical operation identity.

---

228. Schedule Identity

A:

ScheduleId

identifies one schedule artifact.

The same source program can have many schedules:

program + target A -> schedule 1
program + target B -> schedule 2
program + new calibration -> schedule 3
program + different policy -> schedule 4

---

229. Scheduler Session Identity

A:

SchedulerSessionId

identifies a compilation/planning session.

One session may produce multiple schedule epochs/results.

---

230. Resource Identity

Scheduler resource IDs must correspond to the target/resource model.

Do not manufacture arbitrary physical resource identities inside the planner.

---

231. Qubit Resource Representation

A physical qubit may simultaneously have:

canonical PhysicalQubitId
+
resource representation

These are related but conceptually different.

The canonical physical qubit identity identifies the qubit.

The scheduler resource representation describes its scheduling capacity/occupancy.

---

232. Logical Qubit Representation

A logical "QubitId" identifies semantic program state.

Scheduling must not confuse it with:

resource capacity
physical location
hardware address

---

233. Address Independence

Do not treat:

QubitId

as:

array index
memory address
hardware address

unless the canonical IR explicitly defines such conversion at a separate boundary.

---

234. Routing Result Integrity

The routing adapter must preserve:

source logical identity
physical mapping
operation provenance

so that scheduling can verify physical operands.

---

235. Target Capability Validation

Before planning:

operation
+
operands
+
target

must be checked for compatibility.

If the target cannot execute the required operation/resource combination:

UnsupportedOperation

or an appropriate structured error must be returned.

---

236. Scheduling Versus Lowering

Scheduling may work on target-compatible abstract operations.

Hardware lowering converts:

scheduled abstract operation

into:

target-specific instruction/pulse/control representation

Scheduling must not become the pulse compiler.

---

237. Pulse-Level Compatibility

If Zamani eventually supports pulse-level scheduling, the same abstractions remain:

operation
resource
duration
dependency
timing
constraint

The resource model becomes richer.

The core scheduler does not need a new conceptual architecture.

---

238. Measurement-Based Quantum Computing

The scheduler must support programs where measurement determines future quantum operations.

This is handled through:

dynamic/classical.rs
dynamic/conditional.rs
dynamic/feedback.rs

not through a second scheduler.

---

239. Quantum Annealing / Non-Circuit Targets

The scheduler architecture should not assume every target is a gate circuit.

If a target exposes different executable resource/timing semantics, an adapter can construct a scheduling view appropriate to that execution model.

The scheduler's core remains:

operations
dependencies
resources
timing
constraints

---

240. Future Quantum Execution Models

New execution models should integrate through:

adapter
resource model
timing model
constraint model
planner

rather than requiring a rewrite of the scheduler root.

---

241. Quality-of-Schedule Reporting

The scheduler should report not merely:

success

but:

success
algorithm
makespan
depth
resource utilization
objective value
verification status
planning cost

---

242. Reproducibility Record

A production schedule should record:

scheduler version
algorithm
configuration hash/version
target snapshot identity
routing result identity
calibration identity
seed

where applicable.

---

243. Configuration Hash

Where reproducibility requires it, configuration can have a stable canonical representation and derived identity/hash.

The hash must not replace the actual configuration.

---

244. Target Snapshot Identity

A schedule should identify which target description produced it.

A schedule for:

target snapshot A

must not silently be considered equivalent to:

target snapshot B

---

245. Schedule Cache

Future caching may be built around:

program identity
target identity
routing identity
configuration identity
calibration identity

but caching must remain outside core schedule semantics.

---

246. Cache Invalidation

A cached schedule must be invalidated when relevant:

target
calibration
routing
timing
resource availability
constraints
configuration

change.

---

247. Incremental Compilation

The architecture should permit reuse of:

dependency graph
critical path
resource requirements

when the input changes only locally.

This is an optimization, not a semantic requirement.

---

248. Planner Independence

The planner must not depend on a specific objective.

For example:

list planner

can work with:

ASAP
ALAP
resource-aware
fidelity-aware

through policy interfaces.

---

249. Algorithm Independence

An algorithm must not directly depend on:

IBM
Google
IonQ
superconducting
surface code

It operates on the generic context.

---

250. Target Adapter Independence

A hardware adapter translates target-specific information into scheduler contracts.

This means adding a new backend should normally require:

new hardware adapter

not changes to:

list scheduler
ASAP
ALAP
dependency graph
resource calendar

---

251. Test Fixture Independence

Fixtures must describe target models rather than encode scheduler assumptions.

Examples:

small target fixture
large target fixture
multi-resource fixture
distributed fixture
dynamic fixture

The fixture values are test data, not production constants.

---

252. Scalability Fixtures

Scalability tests should generate target sizes programmatically.

Do not create production code such as:

fixture_100_qubits
fixture_1000_qubits

as architectural limits.

---

253. Property-Based Target Generation

Where practical, generate:

resource counts
topologies
capacities
durations
dependency graphs
communication networks

within test bounds.

The bounds belong to tests.

---

254. Test Bounds Are Not Production Limits

A test may use:

1000 operations

because that is practical for CI.

That does not mean:

MAX_OPERATIONS = 1000

in production.

---

255. Benchmark Separation

Benchmark workloads must not influence scheduler semantics.

Benchmarking measures.

Scheduling schedules.

---

256. Runtime Feedback Separation

Runtime events may trigger scheduling updates.

Runtime remains responsible for observing execution.

Scheduler remains responsible for planning/replanning.

---

257. Distributed Synchronization

Distributed scheduling must explicitly model synchronization.

Do not assume clocks are perfectly synchronized.

The target model must supply the required synchronization semantics.

---

258. Time Domains

A distributed target may contain multiple timing domains.

The timing model must support:

local time
global/synchronized time
conversion relationship
uncertainty

where required.

---

259. Clock Uncertainty

If timing uncertainty matters, it should be represented explicitly.

Do not silently assume:

zero clock skew
zero communication jitter
zero synchronization error

---

260. Scheduling Under Uncertainty

Future scheduling policies may use:

bounded duration
probabilistic duration
worst-case duration
expected duration
robust scheduling

The foundational timing model must be extensible enough to represent this without changing operation identity.

---

261. Robust Scheduling

A robust scheduler may choose a schedule that is not nominally shortest but remains valid under expected variation.

This is an optimization strategy, not a core semantic assumption.

---

262. Resource Contention

Resource contention must be measurable.

Diagnostics should identify:

most contended resources
most delayed operations
critical resource bottlenecks

This can guide future optimization.

---

263. Critical Resource Analysis

The scheduler should be able to determine whether a resource lies on the effective critical path.

This can support:

resource-aware policy
diagnostics
benchmarking
optimization

---

264. Deadlock Avoidance

Resource acquisition must avoid inconsistent partial allocation.

For multi-resource operations:

candidate
    |
    v
deterministic requirement ordering
    |
    v
validate all
    |
    v
commit atomically

This avoids resource reservation deadlocks.

---

265. Cycle Detection

The dependency graph must reject cycles before ordinary static scheduling.

For example:

A -> B
B -> C
C -> A

must return:

CycleDetected

rather than hang.

---

266. Dynamic Cycles

Runtime-generated dependency relationships must also be validated.

A dynamic event must not introduce an impossible dependency structure.

---

267. Empty Program

An empty program is a valid scheduling input if canonical IR semantics permit it.

Expected result:

zero operations
zero makespan
valid empty schedule
verified

---

268. Single Operation

The scheduler must correctly schedule a single operation with:

one resource
multiple resources
zero duration
known duration
symbolic duration

as applicable.

---

269. Parallel Operations

Independent operations may execute simultaneously if:

dependencies permit
resources permit
timing permits
target permits

The scheduler must not serialize them merely because they are adjacent in source order.

---

270. Source Ordering

Source order is not automatically a dependency.

Only semantic/explicit constraints should impose ordering.

This is important for parallelism.

---

271. Measurement Ordering

Measurement may create:

quantum dependency
classical dependency
resource dependency

depending on the target/program.

The scheduler must represent the correct relationship rather than assume all measurements serialize the entire circuit.

---

272. Reset Ordering

Reset must occupy whatever target resources its model specifies.

No universal reset duration.

---

273. Classical Control Ordering

A conditional operation must wait for the information it actually depends on.

Unrelated operations should remain parallel where legal.

---

274. Communication Ordering

Only the relevant operations should wait for communication.

Do not globally serialize a distributed schedule because one message is pending.

---

275. Resource Availability Windows

Operations must be placed inside legal windows.

Example:

resource available [T0,T1]
maintenance [T1,T2]
available [T2,T3]

The scheduler should naturally find legal intervals.

---

276. Deadline Feasibility

Before expensive optimization, the scheduler should be able to determine obvious infeasibility when possible.

If a deadline is impossible:

DeadlineExceeded

with explanation.

---

277. Planner Termination

Planners must have defined termination behavior.

Possible reasons:

completed
unschedulable
deadline
cancellation
limit reached
internal failure

---

278. Limit Reached

If the caller imposes:

max_planning_time
max_iterations
max_memory

the scheduler must report that the limit was reached rather than claiming optimal completion.

---

279. Quality on Early Termination

If a heuristic scheduler is stopped early, it may return a candidate only if:

candidate is valid

and the API explicitly allows partial optimization.

It must report:

optimization incomplete

in result metadata.

---

280. Final Schedule Contract

A production "ScheduleResult" must mean:

«This schedule has been constructed for the supplied context and has passed the requested verification level.»

It must never mean merely:

«The algorithm stopped.»

---

281. Final Verification

After any transformation or optimization:

candidate
   |
   v
verification

must run again.

Do not assume a previously valid schedule remains valid after transformation.

---

282. Semantic Verification After Delay Insertion

Explicit delay insertion must be verified because timing itself may be semantic.

---

283. Semantic Verification After Dynamical Decoupling

Dynamical decoupling must be verified under the target's semantics.

It must never be inserted automatically into a target that does not support it.

---

284. Hardware Capability Verification

The final schedule must be checked against the target snapshot.

A schedule generated for target A must not be submitted to target B without revalidation.

---

285. Runtime Submission Gate

Runtime submission should require:

verified schedule
+
matching target identity/capability snapshot

or explicitly perform a fresh validation.

---

286. Schedule-to-Hardware Lowering

The final architecture is:

ScheduleResult
      |
      v
hardware lowering
      |
      v
target executable representation
      |
      v
runtime

Scheduling must not own the final vendor instruction encoding.

---

287. Integration with Simulators

Simulators should expose target models.

Example:

simulator target
    |
    v
hardware adapter
    |
    v
scheduler

The scheduler should not need:

if simulator

logic in its algorithms.

---

288. Integration with Emulators

Same principle as simulators.

Emulation-specific behavior belongs outside core scheduling.

---

289. Integration with Benchmarking

Benchmarking may compare:

scheduler A
scheduler B
scheduler C

using the same target/program.

This is why planner and algorithm contracts must be replaceable.

---

290. Research Extensibility

Researchers should be able to implement a new algorithm without editing:

resource model
timing model
IR adapter
verification

They should implement the planner/algorithm contract and receive standard verification.

---

291. ML Scheduler Integration

An ML scheduler may propose candidate priorities/times.

The ML component does not get to bypass:

hard constraints
resource validation
semantic verification

ML is an optimization component, not a trust boundary.

---

292. Adaptive Scheduling

Adaptive algorithms may inspect:

graph structure
resource pressure
target features
noise information
communication topology

and select a strategy.

Selection must remain deterministic when configured deterministic.

---

293. Scheduling Policy Extensibility

A new policy should be addable without changing:

resource calendar
dependency graph
verification
hardware adapter

---

294. Constraint Extensibility

A new constraint should be addable without modifying every algorithm.

The planner asks the constraint system whether a candidate is legal.

---

295. Resource Extensibility

A new resource kind should be representable without adding a new scheduler algorithm.

This is one of the most important scalability properties.

---

296. Timing Extensibility

A new timing model should be representable without rewriting:

dependency graph
resource calendar
policy
verification

except where the new semantics genuinely require it.

---

297. Technology Extensibility

Adding a new quantum technology should primarily require:

hardware target model
adapter
possibly constraints
possibly timing/resource definitions

not a new scheduler.

---

298. Production Definition

"quantum::scheduling" is production-ready only when:

the scheduler can consume a canonical executable quantum program,
a target snapshot,
routing information,
resource/timing/constraint models,
and explicit scheduling policy,

produce a valid schedule,

verify that schedule independently,

and expose enough provenance and diagnostics
to explain and reproduce the result.

---

299. Final Ownership Table

Concern| Owner
Zamani syntax| "quantum::frontend"
Quantum semantics| "quantum::ir"
Canonical "QubitId"| "quantum::ir::qubit"
Canonical "PhysicalQubitId"| "quantum::ir::qubit"
Gate synthesis| optimization/synthesis
Logical-to-physical mapping| "quantum::routing"
Scheduling| "quantum::scheduling"
Timing model| scheduling + target adapter
Resource scheduling| scheduling
Hardware capabilities| "quantum::hardware"
Calibration| hardware/ZQN
Noise model| "quantum::zqn"
QEC semantics| "quantum::error_correction"
QEC scheduling interface| "scheduling::qec"
QEC decoding| "quantum::error_correction"
Hardware lowering| hardware/backend layer
Execution| runtime
Benchmarking| "quantum::benchmarking"
Scheduling diagnostics| "scheduling::diagnostics"

---

300. Final Architectural Rule

The entire scheduler can be summarized as:

                    WHAT?
                     |
                     v
              quantum::ir
                     |
                     v
                    WHERE?
                     |
                     v
                  routing
                     |
                     v
                    WHEN?
                     |
                     v
                scheduling
                     |
                     v
                     CAN?
                     |
                     v
                  hardware
                     |
                     v
                    HOW?
                     |
                     v
                  runtime

And the target-independent scalability principle is:

                 ONE ZAMANI PROGRAM
                         |
          +--------------+--------------+
          |              |              |
          v              v              v
      small target   large target   distributed target
          |              |              |
          v              v              v
       routing        routing        routing
          |              |              |
          v              v              v
     scheduling      scheduling     scheduling
          |              |              |
          +--------------+--------------+
                         |
                         v
                    execution

The source program remains the semantic source of truth.

The target supplies the resources.

Routing supplies placement.

Scheduling supplies time.

Hardware supplies capabilities.

QEC supplies fault-tolerance requirements.

ZQN supplies noise/uncertainty information.

Runtime performs execution.

---

301. Frozen File Contract Summary

The following contracts are frozen before implementation proceeds:

types.rs
    foundational scheduler vocabulary

errors.rs
    scheduler error taxonomy

limits.rs
    explicit caller/deployment limits

config.rs
    immutable invocation configuration

context.rs
    immutable scheduling snapshot

result.rs
    canonical schedule result

ir/operation.rs
    scheduling view of canonical operations

ir/dependency.rs
    scheduling dependency semantics

ir/graph.rs
    scalable dependency graph

ir/critical_path.rs
    critical-path analysis

resources/resource.rs
    generic resources

resources/pool.rs
    resource groups/capacities

resources/reservation.rs
    time/resource reservations

resources/calendar.rs
    interval resource occupancy

resources/availability.rs
    dynamic resource availability

timing/duration.rs
    target-independent durations

timing/time.rs
    checked time arithmetic

timing/resolution.rs
    target timing granularity

timing/alignment.rs
    alignment semantics

timing/windows.rs
    temporal windows

timing/constraints.rs
    temporal constraints

policies/*
    scheduling preferences

planners/*
    scheduling planning mechanisms

algorithms/*
    concrete algorithms

constraints/*
    hard/soft scheduling constraints

transformations/*
    timing-aware schedule transformations

verification/*
    independent schedule validation

optimization/*
    schedule objectives

qec/*
    QEC scheduling interface

dynamic/*
    dynamic/runtime scheduling

distributed/*
    distributed scheduling

adapters/*
    subsystem boundaries

serialization/*
    versioned schedule persistence

diagnostics/*
    observability/explanation

plugins/*
    external algorithm extension

stabilizer_scheduler.rs
    legacy compatibility facade

mod.rs
    composition root

---

302. Completion Principle

Once a file has satisfied its frozen contract:

responsibility
API
dependencies
invariants
errors
tests
integration
scalability
determinism
safety

it should be considered complete.

Adding another scheduler module must not require reopening it simply because that module was implemented later.

If reopening becomes necessary, the correct action is to identify the architectural contract violation rather than allowing uncontrolled coupling.

---

303. Final Acceptance Test

The scheduler architecture is accepted when the following thought experiment works without source-code changes to the scheduling algorithms:

Program A
+
Target A = tiny quantum device

then:

Program A
+
Target B = very large quantum device

then:

Program A
+
Target C = heterogeneous multi-QPU system

then:

Program A
+
Target D = distributed quantum network

The compiler may produce different:

routing
schedule
resource reservations
timings
communication

but the Zamani program itself remains unchanged.

That is the required meaning of:

«Write once, scale from atom to everywhere.»

---

304. Final Design Decision

"src/quantum/scheduling/" must not become a collection of special-purpose schedulers.

It must become a general scheduling platform based on:

canonical quantum IR
        +
dependency graph
        +
generic resources
        +
target-supplied timing
        +
constraints
        +
policy
        +
replaceable planner
        +
replaceable algorithms
        +
dynamic execution
        +
distributed communication
        +
QEC integration
        +
independent verification
        +
explicit optimization

with:

NO fixed machine size
NO fixed qubit count
NO fixed topology
NO fixed gate count
NO fixed gate arity
NO fixed channel count
NO fixed timing unit
NO vendor-specific core
NO global mutable state
NO hidden randomness
NO unsafe Rust
NO semantic changes hidden in scheduling

The scheduler's single fundamental question remains:

«WHEN?»

Everything else must enter through a well-defined integration boundary.The design above is consistent with the repository's existing scheduling contracts and canonical identity direction; in particular, the repository already states that new code must use quantum::ir::qubit::QubitId / PhysicalQubitId, while the existing scheduling verification code expects canonical operation and qubit identities.

For the external design basis, I also cross-checked the architecture against OpenQASM's explicit timing/delay model and its dynamic classical-control model, plus target-aware scheduling approaches used by current quantum compilers. Those references support treating timing, delays, alignment, classical feedback, and target capabilities as first-class scheduling concerns rather than hard-coded circuit assumptions.