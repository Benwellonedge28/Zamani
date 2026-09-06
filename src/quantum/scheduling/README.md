Worked for 12s

I inspected the current Zamani repository structure, including quantum::ir, the existing scheduling tree, the current stabilizer compatibility facade, and Cargo.toml, and cross-checked the design against current OpenQASM timing/control semantics and platform-agnostic compiler architecture. The repository already contains most of the scheduling directories we previously designed; the README below is therefore written as the authoritative production contract for the implementation, rather than pretending the directory is still empty.

One repository issue that must also be corrected independently is the current invalid Cargo.toml value rust-version = "1.97.1" or "1.97"; Cargo requires one concrete version value.

Zamani Quantum Scheduling

Production Architecture, Contracts, Integration, Scalability and Implementation Specification

Path: "src/quantum/scheduling/README.md"

Subsystem: "crate::quantum::scheduling"

Language: Rust

Required Rust: Rust 1.97 / 1.97.1 as selected by the repository's final toolchain policy

Edition: Rust 2021

Safety: "#![forbid(unsafe_code)]"

Status target: Production-ready, target-independent, scalable scheduling infrastructure

---

1. Purpose

"quantum::scheduling" is the target-independent scheduling subsystem of the Zamani quantum compiler.

Its responsibility is:

«Given a semantically valid quantum program, its dependency structure, an execution target description, timing information, resource availability, and explicit scheduling policy, determine when executable operations may occur while preserving program semantics and satisfying every applicable constraint.»

Scheduling answers:

«WHEN can an operation execute?»

It does not answer:

«What does the program mean?»

That is owned by "quantum::ir".

It does not answer:

«Where should logical qubits be placed?»

That is owned by routing.

It does not answer:

«What physical hardware instruction or pulse implements the operation?»

That is owned by target/hardware lowering.

It does not answer:

«How is an error syndrome decoded?»

That belongs to QEC.

The intended dependency direction is:

Zamani source
      │
      ▼
quantum::frontend
      │
      ▼
quantum::ir
      │
      ▼
optimization
      │
      ▼
routing
      │
      ▼
quantum::scheduling
      │
      ▼
hardware lowering
      │
      ▼
runtime / backend
      │
      ▼
quantum machine / simulator / emulator

The scheduler must never reverse this dependency direction.

---

2. Fundamental Zamani Principle

Zamani quantum programs must be written once and specialized to available execution resources.

The source program must not contain assumptions such as:

machine has 127 qubits
machine has 1000 qubits
machine has exactly 8 control channels
machine has a square topology
gate X takes exactly 20 ns
measurement takes exactly 500 ns
machine uses superconducting qubits
machine uses one particular vendor
machine uses a particular number of modules

Instead:

Zamani program
      │
      ▼
canonical quantum IR
      │
      ▼
target description
      │
      ├── resources
      ├── capabilities
      ├── topology
      ├── timing
      ├── calibration
      ├── alignment
      ├── communication
      └── availability
      │
      ▼
routing
      │
      ▼
scheduling
      │
      ▼
target-specific executable representation

Therefore:

same source program
        +
different target
        =
different valid specialization

without changing the source program.

---

3. Meaning of "Infinity"

"Infinite scalability" does not mean that a physical computer can allocate infinite memory or execute an infinite number of operations.

It means:

«The scheduler contains no artificial finite architectural ceiling on machine size, qubit count, operation count, topology size, scheduling depth, resource count, QEC distance, communication nodes, or execution duration.»

Every concrete compilation remains bounded by:

- available memory;
- address space;
- compiler time;
- operating-system limits;
- explicit caller limits;
- target resources;
- target capabilities;
- network capacity;
- storage;
- execution environment.

The scheduler must therefore never introduce architectural constants such as:

const MAX_QUBITS: usize = 1000;
const MAX_OPERATIONS: usize = 1_000_000;
const MAX_CHANNELS: usize = 64;
const MAX_ROUNDS: usize = 100;

Such limits are prohibited.

If a limit is required, it must be an explicit invocation/resource/security policy.

---

4. Canonical IR Ownership

The authoritative quantum IR is:

crate::quantum::ir

The repository explicitly defines "quantum::ir" as the canonical semantic boundary and states that it does not decide physical machine selection, routing, scheduling, hardware instructions, calibration, execution, or QEC decoding.

The authoritative qubit identities are:

crate::quantum::ir::qubit::QubitId
crate::quantum::ir::qubit::PhysicalQubitId

New scheduling code MUST use those canonical identities.

Do not create:

SchedulingQubitId
SchedulerQubitId
PhysicalSchedulerQubitId
LocalQubitId

as competing semantic identities.

The repository already explicitly establishes "quantum::ir::qubit" as the authoritative qubit implementation.

The required relationship is:

quantum::ir::qubit::QubitId
        │
        ▼
logical identity
        │
        ▼
routing
        │
        ▼
quantum::ir::qubit::PhysicalQubitId
        │
        ▼
scheduling

A scheduler must never silently convert a logical "QubitId" into a physical identifier.

---

5. Scheduling Scope

The scheduling subsystem must support, without redesign:

- gate-based circuits;
- dynamic circuits;
- measurement-driven computation;
- classical control;
- conditional operations;
- reset;
- measurement;
- feedback;
- runtime-dependent operations;
- pulse-level semantic timing;
- resource-constrained execution;
- QEC;
- stabilizer rounds;
- communication;
- distributed quantum systems;
- modular QPUs;
- multi-QPU execution;
- networked quantum computing;
- simulators;
- emulators;
- heterogeneous quantum technologies;
- future execution technologies not yet known.

OpenQASM 3 demonstrates why scheduling cannot be designed only as a static gate DAG: the language includes classical control, runtime processing, explicit timing, durations, stretches, delays, boxes, barriers, and pulse-level timing concepts.

---

6. Scheduling Must Remain Separate from Routing

Routing answers:

«WHERE?»

Scheduling answers:

«WHEN?»

For example:

logical q0 → physical q7
logical q1 → physical q8

is routing.

Then:

operation(q7, q8)
start = T
duration = D

is scheduling.

The intended pipeline is:

canonical IR
      │
      ▼
optimization
      │
      ▼
routing
      │
      ▼
mapped executable representation
      │
      ▼
scheduling

The scheduler must not implement logical-to-physical mapping.

---

7. Scheduling Must Remain Separate from Hardware

The hardware subsystem owns target facts.

The scheduler consumes a target description.

Conceptually:

quantum::hardware
      │
      ▼
target capabilities
      │
      ▼
scheduling context
      │
      ▼
scheduler

Hardware may provide:

- supported operations;
- supported qubits;
- physical qubits;
- topology;
- gate durations;
- measurement durations;
- reset durations;
- control channels;
- readout channels;
- resource capacities;
- timing resolution;
- alignment requirements;
- calibration information;
- availability;
- maintenance windows;
- communication capabilities;
- technology-specific constraints.

The scheduler must not contain vendor SDK logic.

It must not contain authentication.

It must not contact hardware.

It must not discover hardware.

Those responsibilities remain outside this subsystem.

---

8. Scheduling Architecture

The production structure is:

src/quantum/scheduling/
│
├── README.md
│
├── mod.rs
├── types.rs
├── errors.rs
├── limits.rs
├── config.rs
├── context.rs
├── result.rs
│
├── ir/
│   ├── mod.rs
│   ├── operation.rs
│   ├── dependency.rs
│   ├── graph.rs
│   └── critical_path.rs
│
├── resources/
│   ├── mod.rs
│   ├── resource.rs
│   ├── pool.rs
│   ├── reservation.rs
│   ├── calendar.rs
│   └── availability.rs
│
├── timing/
│   ├── mod.rs
│   ├── duration.rs
│   ├── time.rs
│   ├── resolution.rs
│   ├── alignment.rs
│   ├── windows.rs
│   └── constraints.rs
│
├── constraints/
│   ├── mod.rs
│   ├── constraint.rs
│   ├── qubit.rs
│   ├── channel.rs
│   ├── measurement.rs
│   ├── reset.rs
│   ├── control.rs
│   ├── communication.rs
│   └── custom.rs
│
├── policies/
│   ├── mod.rs
│   ├── policy.rs
│   ├── asap.rs
│   ├── alap.rs
│   ├── priority.rs
│   ├── resource_aware.rs
│   └── hybrid.rs
│
├── planners/
│   ├── mod.rs
│   ├── planner.rs
│   ├── list.rs
│   ├── critical_path.rs
│   ├── resource_constrained.rs
│   └── event.rs
│
├── algorithms/
│   ├── mod.rs
│   ├── asap.rs
│   ├── alap.rs
│   ├── list.rs
│   ├── cp.rs
│   ├── rcpsp.rs
│   └── adaptive.rs
│
├── transformations/
│   ├── mod.rs
│   ├── delays.rs
│   ├── alignment.rs
│   ├── padding.rs
│   └── dynamical_decoupling.rs
│
├── verification/
│   ├── mod.rs
│   ├── structural.rs
│   ├── dependency.rs
│   ├── resource.rs
│   ├── timing.rs
│   ├── semantic.rs
│   └── verifier.rs
│
├── optimization/
│   ├── mod.rs
│   ├── makespan.rs
│   ├── depth.rs
│   ├── idle_time.rs
│   ├── fidelity.rs
│   ├── energy.rs
│   └── multi_objective.rs
│
├── dynamic/
│   ├── mod.rs
│   ├── classical.rs
│   ├── conditional.rs
│   ├── feedback.rs
│   └── runtime.rs
│
├── distributed/
│   ├── mod.rs
│   ├── node.rs
│   ├── link.rs
│   ├── communication.rs
│   └── network.rs
│
├── qec/
│   ├── mod.rs
│   ├── interface.rs
│   ├── syndrome.rs
│   ├── rounds.rs
│   └── stabilizer.rs
│
├── adapters/
│   ├── mod.rs
│   ├── ir.rs
│   ├── hardware.rs
│   ├── routing.rs
│   └── qec.rs
│
├── serialization/
│   ├── mod.rs
│   ├── schema.rs
│   ├── encode.rs
│   └── decode.rs
│
├── diagnostics/
│   ├── mod.rs
│   ├── trace.rs
│   ├── explain.rs
│   └── profile.rs
│
├── plugins/
│   ├── mod.rs
│   ├── scheduler.rs
│   └── registry.rs
│
├── tests/
│   ├── mod.rs
│   ├── unit/
│   ├── integration/
│   ├── property/
│   ├── regression/
│   ├── scalability/
│   ├── determinism/
│   └── fixtures/
│
└── stabilizer_scheduler.rs

The current repository already contains the major architectural groups including "adapters", "algorithms", "constraints", "diagnostics", "distributed", "dynamic", "ir", "optimization", "planners", "plugins", "policies", "qec", "resources", "serialization", "tests", "timing", "transformations", and "verification".

This README therefore defines what each group must mean and what its integration contract is.

---

9. File Completion Rule

Every scheduling file must have a frozen contract before implementation is considered complete.

A file is not complete merely because it compiles.

Each file must define:

1. responsibility;
2. public types;
3. public functions;
4. invariants;
5. ownership;
6. dependency direction;
7. input contract;
8. output contract;
9. error behavior;
10. deterministic behavior;
11. serialization behavior, if applicable;
12. thread-safety expectations;
13. scalability behavior;
14. resource behavior;
15. integration boundary;
16. test requirements;
17. forbidden responsibilities.

The file must not depend on undocumented assumptions in another scheduling file.

The purpose is to satisfy:

«Once a file is finished, adding another independent file must not require reopening the finished file merely to repair its contract.»

Only legitimate API evolution or discovered correctness defects may require later changes.

---

10. Implementation Dependency Order

Implementation must proceed from independent contracts toward composition.

Phase 1 — foundational contracts

Implement and freeze:

types.rs
errors.rs
limits.rs
timing/duration.rs
timing/time.rs
resources/resource.rs

These files must not depend on planner implementations.

---

Phase 2 — timing and resource primitives

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

Phase 3 — scheduler IR

Implement:

ir/operation.rs
ir/dependency.rs
ir/graph.rs
ir/critical_path.rs

The scheduler IR is a scheduling view over canonical "quantum::ir".

It is not a second semantic quantum IR.

---

Phase 4 — constraints

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

Phase 5 — composition contracts

Implement:

context.rs
config.rs
result.rs

---

Phase 6 — policies

Implement:

policies/policy.rs
policies/asap.rs
policies/alap.rs
policies/priority.rs
policies/resource_aware.rs
policies/hybrid.rs

---

Phase 7 — planners

Implement:

planners/planner.rs
planners/list.rs
planners/critical_path.rs
planners/resource_constrained.rs
planners/event.rs

---

Phase 8 — algorithms

Implement:

algorithms/asap.rs
algorithms/alap.rs
algorithms/list.rs
algorithms/cp.rs
algorithms/rcpsp.rs
algorithms/adaptive.rs

Algorithms must consume the frozen planner contracts rather than inventing parallel scheduling APIs.

---

Phase 9 — transformations and verification

Implement:

transformations/*
verification/*

Verification must become mandatory before a production schedule is accepted.

---

Phase 10 — objective optimization

Implement:

optimization/*

---

Phase 11 — dynamic, QEC and distributed scheduling

Implement:

dynamic/*
qec/*
distributed/*

---

Phase 12 — integration boundaries

Implement:

adapters/*
serialization/*
diagnostics/*
plugins/*

---

Phase 13 — compatibility

Only after the generic scheduler is stable:

stabilizer_scheduler.rs

must be reduced to a compatibility facade.

The current file already documents this intended role and explicitly rejects synthetic gates, fixed topology, fixed ancillas, fixed rounds, fixed qubit counts and hardware assumptions.

---

Phase 14 — composition root

"mod.rs" is finalized last.

It should compose completed modules and expose stable APIs.

It must not contain scheduling algorithms.

---

11. "types.rs"

Responsibility

Own scheduler-specific vocabulary that does not already belong to canonical quantum IR.

Potential types include:

ScheduleId
DependencyId
ReservationId
TimePoint
Duration
TimeInterval
Priority
Cost
Slack
Makespan

Do not redefine canonical:

QubitId
PhysicalQubitId
ClassicalBitId
OperationId

unless the canonical IR does not own a required identity.

If a semantic identity already exists in "quantum::ir", scheduling must use it.

Requirements

All semantic identifiers must be:

- strongly typed;
- comparable;
- hashable;
- serializable where needed;
- deterministic;
- explicit;
- free from machine-size assumptions.

Do not expose raw "usize" as the meaning of a quantum identity.

---

12. "errors.rs"

Own the scheduler error hierarchy.

It must represent at least:

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
Cancelled
Internal

Errors must carry structured information where useful:

operation
resource
time
constraint
cause
context

Error strings must never be used as program logic.

---

13. "limits.rs"

This file defines invocation-specific limits.

Examples:

maximum operations
maximum graph edges
maximum memory
maximum planning time
maximum schedule duration
maximum parallelism
deadline
cancellation
maximum diagnostics

Every limit must be optional.

There must be no universal machine-size limit.

Distinguish:

architectural capability

from:

caller policy

from:

security limit

from:

available physical resource

---

14. "config.rs"

"SchedulingConfig" controls scheduler behavior.

It must include concepts for:

policy
objective
determinism
seed
verification
optimization
parallelism
resource policy
timing policy
deadline
diagnostics
distributed scheduling
dynamic scheduling

Configuration must be explicit.

No global mutable configuration.

No hidden environment-dependent scheduling decisions.

A deterministic request must be reproducible from its declared inputs.

---

15. "context.rs"

"SchedulingContext" is the immutable compilation context.

It combines:

program
dependency information
target capabilities
resource model
timing model
constraints
routing result
calibration snapshot
availability
policy
objective
limits
determinism context

The context must be sufficient for the scheduler to make its decisions.

The scheduler must not secretly retrieve missing information from hardware.

---

16. "result.rs"

A production "ScheduleResult" must contain more than timestamps.

It must be able to represent:

scheduled operations
start times
finish times
durations
resource reservations
makespan
depth
critical path
slack
idle intervals
resource utilization
objective score
verification report
diagnostics
provenance
reproducibility metadata

The result must identify the target/context under which the schedule was produced.

---

17. "ir/operation.rs"

This is the scheduling view of an executable operation.

It must retain:

canonical source operation identity
operands
logical/physical identity
duration
resource requirements
precedence
timing constraints
conditions
metadata
provenance

It must not become a second quantum semantic IR.

---

18. "ir/dependency.rs"

Represent dependencies including:

quantum data dependency
classical dependency
measurement dependency
control dependency
resource dependency
ordering dependency

The dependency graph must support arbitrary fan-in and fan-out.

---

19. "ir/graph.rs"

The scheduler graph must support:

- DAG construction;
- predecessor lookup;
- successor lookup;
- topological traversal;
- cycle detection;
- ready-set calculation;
- deterministic traversal;
- scalable storage;
- incremental construction where useful.

Avoid recursion for unbounded graph depth.

Prefer iterative traversal.

---

20. "ir/critical_path.rs"

Calculate:

earliest start
earliest finish
latest start
latest finish
slack
critical path

It must not mutate canonical IR.

It must operate on the scheduling representation.

---

21. "resources/resource.rs"

Resources must be generic.

A resource may represent:

logical qubit
physical qubit
control channel
measurement channel
readout resonator
laser
microwave source
coupler
ancilla
classical processor
memory
communication link
network endpoint
cryogenic resource
vendor-specific abstract resource

Resource kinds must be extensible.

---

22. Resource Modes

Resources must support:

exclusive
shared
capacity-limited
consumable
reusable
hierarchical
time-dependent
conditional

The scheduler must never assume that all quantum resources are qubits.

---

23. "resources/pool.rs"

Represents resource pools.

Examples:

measurement channels
control channels
communication links
classical processors

Pool size comes from target capabilities.

Never hard-code the number of resources.

---

24. "resources/reservation.rs"

A reservation binds:

resource
operation
start
duration
finish
usage mode

Reservations must be conflict-checkable.

---

25. "resources/calendar.rs"

Represents time-varying resource availability.

Must support:

available
busy
disabled
maintenance
calibration
degraded
reserved
unknown

This prevents the scheduler from assuming resources are continuously available.

---

26. "resources/availability.rs"

Provides the query boundary:

is_available(resource, interval)

or equivalent semantics.

It must not contact hardware directly.

---

27. Timing Architecture

Timing must be target-aware but target-independent.

The OpenQASM timing model is an important reference: durations can be expressed in physical units or backend-dependent "dt", and target-specific calibration can determine actual durations later.

Zamani should preserve the same architectural distinction:

program timing intent
        +
target timing model
        =
resolved schedule timing

---

28. "timing/duration.rs"

Support:

fixed duration
symbolic duration
calibrated duration
target-dependent duration
interval duration
unknown duration

Do not encode:

X = 20ns
CNOT = 200ns
MEASURE = 500ns

in scheduler code.

Those values belong to the target description.

---

29. "timing/time.rs"

Provide checked:

TimePoint
Duration
TimeInterval

Requirements:

- checked addition;
- checked subtraction;
- no accidental negative execution duration;
- no silent overflow;
- explicit representation of unresolved timing where required.

---

30. "timing/resolution.rs"

Represent target timing resolution.

Examples:

continuous
nanoseconds
picoseconds
sample ticks
dt
rational resolution
custom target resolution

The scheduler must consume the target's resolution.

---

31. "timing/alignment.rs"

Support:

qubit alignment
channel alignment
measurement alignment
control alignment
frame alignment
box alignment
target-specific alignment

Alignment must be expressed as a target constraint, not a hard-coded scheduler constant.

---

32. "timing/windows.rs"

Represent:

release time
earliest start
latest start
earliest finish
latest finish
deadline
availability window

---

33. "timing/constraints.rs"

Combine temporal constraints and determine whether a proposed interval is legal.

The file must not own resource constraints.

Those belong to "constraints/*" and "resources/*".

---

34. "constraints/constraint.rs"

Provide the generic constraint interface.

Conceptually:

check(candidate, context)
explain(candidate, context)
priority()

A constraint must be inspectable.

When an operation cannot be scheduled, diagnostics must be able to explain why.

---

35. "constraints/qubit.rs"

Represent qubit occupancy and conflicts.

Use:

crate::quantum::ir::qubit::QubitId
crate::quantum::ir::qubit::PhysicalQubitId

as appropriate.

Never define replacement identities.

---

36. "constraints/channel.rs"

Represent shared control/readout resources.

Must support arbitrary target-defined capacity.

---

37. "constraints/measurement.rs"

Handle:

- measurement duration;
- readout capacity;
- measurement grouping;
- measurement dependencies;
- classical-result availability;
- target measurement alignment.

---

38. "constraints/reset.rs"

Handle reset readiness and reset timing.

Reset must not be treated as a universally zero-duration operation.

---

39. "constraints/control.rs"

Handle:

classical conditions
branch readiness
feedback
measurement-to-control dependency
runtime decisions

---

40. "constraints/communication.rs"

Represent:

local communication
inter-module communication
inter-chip communication
entanglement generation
teleportation
classical communication
synchronization
network latency

This is required for eventual distributed quantum systems.

---

41. "constraints/custom.rs"

Permit extensions without modifying the scheduler core.

Custom constraints must be:

- explicit;
- deterministic unless declared otherwise;
- explainable;
- composable;
- verifiable.

---

42. Policies

A policy says what the scheduler should optimize or prioritize.

A policy must not itself become a complete scheduler implementation.

---

43. "policies/asap.rs"

ASAP:

«Schedule each operation as early as legally possible.»

ASAP is a standard scheduling strategy in modern quantum transpilers.

---

44. "policies/alap.rs"

ALAP:

«Schedule operations as late as legally possible while satisfying the required completion constraints.»

Modern quantum compiler tooling likewise exposes ALAP scheduling.

---

45. "policies/priority.rs"

Priority strategies may use:

critical-path priority
deadline priority
resource pressure
measurement priority
communication priority
fidelity priority
user-defined priority

Priority must be explicit and deterministic when determinism is requested.

---

46. "policies/resource_aware.rs"

Prefer schedules that use scarce resources efficiently.

The scheduler must be capable of reasoning about resource contention rather than only dependency ordering.

---

47. "policies/hybrid.rs"

Compose policies:

ASAP + resource aware
ALAP + fidelity
critical path + resource pressure
communication aware + deadline

No policy may silently override another policy.

---

48. Planner Architecture

Planners perform scheduling.

The central contract is conceptually:

Planner::plan(
    scheduling_context
) -> Result<ScheduleResult, SchedulingError>

Planners must not:

- parse source code;
- contact hardware;
- authenticate;
- perform routing;
- decode QEC;
- synthesize arbitrary gates.

---

49. "planners/list.rs"

List scheduling should be the primary scalable general-purpose baseline.

Conceptually:

dependency graph
      │
      ▼
ready operations
      │
      ▼
priority selection
      │
      ▼
resource availability
      │
      ▼
earliest legal time
      │
      ▼
reserve resources
      │
      ▼
release newly-ready operations

This avoids constructing a giant fixed time-slot matrix.

---

50. "planners/critical_path.rs"

Critical-path scheduling uses dependency slack and criticality.

It must support:

- weighted operation duration;
- resource constraints;
- timing windows;
- deterministic ordering.

---

51. "planners/resource_constrained.rs"

Support resource-constrained project scheduling concepts.

The algorithm must distinguish:

dependency readiness

from:

resource readiness

---

52. "planners/event.rs"

Use event-driven advancement rather than repeatedly scanning every operation.

Important events include:

operation finished
resource released
measurement completed
classical result available
communication completed
runtime event received
availability changed

This is particularly important at large scale.

---

53. Algorithms

Algorithms are implementations of frozen planner contracts.

Required baseline algorithms:

ASAP
ALAP
list scheduling
critical-path scheduling
resource-constrained scheduling
adaptive scheduling

No algorithm may define a competing scheduler API.

---

54. Adaptive Scheduling

"algorithms/adaptive.rs" may choose a scheduling strategy according to:

dependency graph characteristics
resource pressure
operation density
communication pressure
timing constraints
target characteristics
QEC requirements
objective

It must not change quantum semantics.

Its decision must be observable through diagnostics/provenance.

---

55. Transformations

Scheduling transformations modify the scheduled representation only when explicitly requested or required by target constraints.

---

56. "transformations/delays.rs"

Represent idle periods explicitly when required.

This is important because explicit timing has semantic/compiler consequences: OpenQASM distinguishes implicit idle time from explicit "delay", and explicit delays constrain later optimization movement.

Zamani must therefore distinguish:

implicit idle time

from:

explicit delay instruction

---

57. "transformations/alignment.rs"

Transform ideal timing into target-valid timing.

Examples:

round start
channel boundary
sample boundary
measurement boundary
frame synchronization

---

58. "transformations/padding.rs"

Insert legal padding required by timing/resource constraints.

Padding must be semantically verified.

---

59. "transformations/dynamical_decoupling.rs"

Optional scheduling transformation.

It must not become part of the fundamental scheduler.

The target and configuration determine whether it is appropriate.

OpenQASM explicitly treats timing as relevant to dynamical decoupling and related experiments.

---

60. Verification

Verification is mandatory for production schedules.

A successful schedule must satisfy all applicable invariants.

---

61. "verification/structural.rs"

Check:

every required operation exists
no duplicate operation exists
all operation identities remain valid
all required metadata exists

---

62. "verification/dependency.rs"

For every dependency:

finish(predecessor) <= start(successor)

unless an explicitly defined dependency semantics permits another relationship.

Cycles must be rejected unless the representation is a dynamic control structure explicitly supported by the scheduler.

---

63. "verification/resource.rs"

Check:

resource usage <= capacity

for every resource and interval.

No exclusive resource may overlap.

No capacity violation may be ignored.

---

64. "verification/timing.rs"

Check:

- duration correctness;
- timing resolution;
- alignment;
- windows;
- release times;
- deadlines;
- non-negative resolved durations;
- target timing legality.

---

65. "verification/semantic.rs"

This is the highest-level invariant:

«Scheduling must not change the computation.»

Verify that scheduling preserves:

- operations;
- operands;
- logical identities;
- controls;
- measurement semantics;
- classical dependencies;
- dynamic control semantics;
- explicit timing semantics.

The scheduler may change when an operation executes, but not what computation it represents.

---

66. "verification/verifier.rs"

Aggregate all verification passes.

Production default:

construct schedule
      │
      ▼
verify
      │
 ┌────┴────┐
valid     invalid
 │           │
 ▼           ▼
return     error

An analysis/debug configuration may permit incomplete schedules, but production execution must not silently accept them.

---

67. Optimization Objectives

Scheduling optimization must be explicit.

Supported objectives include:

makespan
depth
idle time
resource utilization
estimated fidelity
energy
communication cost
multi-objective score

No objective weights may be hidden in source code.

---

68. "optimization/makespan.rs"

Minimize total execution duration:

makespan = maximum operation finish time

---

69. "optimization/depth.rs"

Minimize scheduled logical/physical depth where meaningful.

Depth must not be confused with wall-clock execution time.

---

70. "optimization/idle_time.rs"

Minimize unnecessary resource idle periods.

---

71. "optimization/fidelity.rs"

Use target-provided fidelity/noise estimates when available.

Scheduling must consume these through an integration boundary.

It must not implement an independent noise model.

---

72. "optimization/energy.rs"

Optional target-dependent objective.

The scheduler must not assume that every quantum technology exposes a meaningful energy cost.

---

73. "optimization/multi_objective.rs"

Support explicit objective composition.

For example:

minimize:
    makespan
    +
    weighted idle time
    +
    weighted error estimate
    +
    weighted communication cost

Weights must be configuration data.

---

74. Dynamic Scheduling

The scheduler must support both:

static scheduling

and:

runtime-dependent scheduling

OpenQASM 3 explicitly introduces runtime classical control and external classical computation mechanisms, so a production scheduler cannot assume all control flow resolves statically.

---

75. "dynamic/classical.rs"

Represent classical dependencies relevant to timing.

Examples:

measurement result
classical calculation
condition evaluation
feedback result

---

76. "dynamic/conditional.rs"

Represent:

if
else
switch
conditional gate
conditional measurement
conditional reset

Scheduling must preserve branch semantics.

---

77. "dynamic/feedback.rs"

Model:

quantum operation
      │
      ▼
measurement
      │
      ▼
classical processing
      │
      ▼
feedback
      │
      ▼
next quantum operation

Latency must come from target capabilities.

---

78. "dynamic/runtime.rs"

Represent operations whose exact execution time cannot be fully determined at static compilation.

The result may contain:

static schedule
+
runtime scheduling constraints

rather than pretending everything is statically known.

---

79. Distributed Scheduling

The architecture must scale conceptually:

single qubit
↓
single QPU
↓
multi-chip
↓
multi-module
↓
multi-QPU
↓
quantum network
↓
distributed quantum system

The source program remains unchanged.

Only the target/resource/communication description changes.

---

80. "distributed/node.rs"

Represent schedulable execution nodes.

A node may represent:

- QPU;
- QPU module;
- simulator partition;
- classical controller;
- quantum network endpoint.

---

81. "distributed/link.rs"

Represent:

quantum link
classical link
entanglement link
communication channel

with target-defined capacity and latency.

---

82. "distributed/communication.rs"

Represent communication operations:

send
receive
entanglement generation
teleportation
synchronization
classical feedback

---

83. "distributed/network.rs"

Represent the target communication topology.

No fixed number of nodes.

No fixed topology.

No assumption that the network is a grid.

---

84. QEC Scheduling

QEC must integrate with generic scheduling.

The correct architecture is:

QEC compiler/model
      │
      ▼
QEC requirements
      │
      ▼
scheduling::qec
      │
      ▼
generic scheduling

QEC must supply semantic requirements.

The generic scheduler determines timing/resource placement.

---

85. "qec/interface.rs"

Define the contract between QEC and scheduling.

It must represent:

round requirements
syndrome requirements
ancilla requirements
measurement requirements
dependency requirements
feedback requirements
resource requirements
timing requirements

---

86. "qec/syndrome.rs"

Represent syndrome extraction timing and dependencies.

Do not implement a decoder here.

---

87. "qec/rounds.rs"

Represent:

QEC round
round dependency
round duration constraints
round synchronization
measurement completion
next-round readiness

Do not hard-code:

distance = 3
rounds = 10
ancillas = fixed number

---

88. "qec/stabilizer.rs"

Support stabilizer-specific scheduling requirements.

It must obtain:

- topology;
- stabilizer definitions;
- qubit participation;
- ancilla resources;
- durations;
- measurement capabilities;
- round requirements

from the relevant QEC/target subsystems.

It must not manufacture a hardware topology.

---

89. "stabilizer_scheduler.rs"

This file is a compatibility facade.

It must not become a second scheduler.

The current repository version already documents the intended migration away from synthetic "H", "Measure", "Reset", comments representing CNOTs, fixed topology, synthetic qubits, and hard-coded assumptions.

The production relationship is:

stabilizer_scheduler.rs
        │
        ▼
qec::stabilizer
        │
        ▼
qec::interface
        │
        ▼
generic scheduler

It must never contain:

- ASAP;
- ALAP;
- list scheduling;
- RCPSP;
- critical-path scheduling;
- resource calendar logic;
- hardware discovery;
- pulse generation;
- QEC decoding.

---

90. Adapters

Adapters are mandatory because the scheduler must integrate with existing Zamani subsystems without contaminating scheduler core files with external implementation details.

---

91. "adapters/ir.rs"

Convert:

quantum::ir

into:

scheduling::ir

Requirements:

- use canonical IR types;
- preserve operation identity;
- preserve provenance;
- preserve qubit identity;
- preserve classical dependencies;
- preserve explicit timing;
- preserve dynamic control;
- never mutate canonical IR merely to schedule it.

---

92. "adapters/hardware.rs"

Convert:

quantum::hardware

into:

SchedulingTarget

The adapter must provide:

capabilities
durations
resources
timing resolution
alignment
availability
calibration snapshot
communication

The scheduler must not depend directly on a vendor backend.

---

93. "adapters/routing.rs"

Consume routing output.

Routing establishes:

logical → physical

The scheduler consumes that result.

No duplicate routing logic.

---

94. "adapters/qec.rs"

Convert QEC requirements into scheduling constraints/resource requirements.

No QEC decoder implementation.

---

95. ZQN / Noise Integration

Where the repository's ZQN subsystem supplies:

gate error
duration uncertainty
drift
crosstalk
temporal noise
calibration confidence

the scheduler may consume these as objective inputs or constraints.

The dependency must be:

ZQN
  │
  ▼
adapter
  │
  ▼
scheduling context

not:

scheduler implements its own ZQN

---

96. Serialization

Schedules must be reproducible and portable.

---

97. "serialization/schema.rs"

Define a versioned schedule schema.

The schema must contain enough information to distinguish:

schedule version
target identity
program identity
operation identity
resource identity
timing
constraints
provenance
objective
verification

Schema evolution must be explicit.

---

98. "serialization/encode.rs"

Serialize validated schedule data.

Never serialize hidden global scheduler state.

---

99. "serialization/decode.rs"

Deserialization must validate:

- schema version;
- identifiers;
- timing;
- resources;
- constraints;
- provenance;
- integrity.

Never deserialize unvalidated data directly into executable hardware operations.

---

100. Diagnostics

A production scheduler must explain decisions.

---

101. "diagnostics/trace.rs"

Record:

operation became ready
operation selected
operation delayed
resource conflict
constraint conflict
resource reserved
resource released
alignment adjustment
transformation
verification result

---

102. "diagnostics/explain.rs"

Provide explanations such as:

Operation O42 could not start at T=200
because resource R7 remains occupied until T=240.

or:

Operation O91 was shifted by 20 ns to satisfy
measurement-channel alignment.

Diagnostics must not require inspecting internal scheduler memory.

---

103. "diagnostics/profile.rs"

Measure:

planning time
dependency-analysis time
resource-analysis time
verification time
memory
operation count
edge count
resource count
conflict count
iterations

This integrates with the repository's benchmarking architecture.

---

104. Plugins

The scheduler must support replaceable algorithms.

Examples:

default Zamani scheduler
vendor scheduler
research scheduler
ML scheduler
custom heuristic
external optimizer

Plugins must implement a stable scheduler contract.

They must not modify canonical IR definitions.

---

105. Determinism

A deterministic scheduling configuration must guarantee:

same canonical input
+
same target snapshot
+
same configuration
+
same seed
=
same schedule

unless the target itself is explicitly dynamic.

Randomized algorithms must receive an explicit RNG/seed.

No hidden randomness.

No iteration-order-dependent behavior.

---

106. Parallel Scheduling

Parallelism is allowed.

However:

parallel implementation

must not imply:

nondeterministic result

when deterministic mode is requested.

A scalable implementation may parallelize:

dependency analysis
resource analysis
ready-set analysis
constraint evaluation
verification

while retaining deterministic arbitration.

---

107. Memory Scalability

Do not implement a schedule as:

qubits × maximum_time_slots

or another machine-size-dependent dense matrix.

Prefer:

operation → interval
resource → interval/reservation structure
dependency → adjacency structure
event → ordered event structure

Use sparse representations where appropriate.

Avoid unnecessary cloning of large graphs.

Avoid recursion for potentially unbounded graph depth.

---

108. Complexity

Dependency analysis should target:

O(V + E)

where:

V = operations
E = dependency edges

Scheduling with arbitrary resource constraints may be computationally difficult.

The architecture must therefore distinguish:

exact algorithms
heuristics
approximations
deterministic algorithms
stochastic algorithms

The API must report algorithm identity and objective quality.

The system must never falsely claim global optimality when a heuristic was used.

---

109. Timing Semantics

The scheduler must distinguish:

duration

from:

timing intent

and:

target-resolved duration

This is especially important because OpenQASM timing explicitly allows timing intent to remain independent of precise target gate durations.

Zamani should therefore support:

symbolic timing
      │
      ▼
target timing resolution
      │
      ▼
concrete schedule

---

110. Explicit Delays

An explicit delay is not merely an invisible gap.

It can constrain later transformations.

Therefore:

implicit idle

and:

explicit delay

must remain distinguishable.

This follows the timing semantics used by OpenQASM, where explicit delays participate in the program's timing structure.

---

111. Boxes / Scheduling Regions

The scheduler should eventually support timing regions equivalent in concept to:

scheduled region
boxed region
barrier region
synchronization region

A region may specify:

maximum duration
exact duration
minimum duration
alignment
synchronization

This is necessary for sophisticated timing intent and fault-tolerant execution.

OpenQASM's "box" construct demonstrates this style of timing constraint.

---

112. Pulse-Level Compatibility

The scheduler should remain above vendor pulse generation.

It may consume semantic pulse resources such as:

frame
port
waveform
channel

where the canonical IR exposes them.

OpenQASM/OpenPulse treats frames as virtual resources and leaves their physical implementation to backend compilation. Zamani should preserve the same separation.

---

113. Technology Independence

The scheduler must not assume:

superconducting
trapped ion
neutral atom
photonic
spin
topological
annealing
continuous-variable
bosonic
measurement-based
hybrid

Any technology-specific constraint must enter through:

target capabilities
resource model
timing model
constraints
adapter

not scheduler algorithms.

---

114. Program Portability

A Zamani program should be portable in the following sense:

source program
      │
      ▼
canonical semantic IR
      │
      ├──────── target A
      │            │
      │            ▼
      │        schedule A
      │
      ├──────── target B
      │            │
      │            ▼
      │        schedule B
      │
      └──────── target C
                   │
                   ▼
               schedule C

The schedules may differ.

The program semantics must remain equivalent.

This matches the broader architecture of platform-agnostic quantum compilation: compilation maps a universal abstraction onto restricted target capabilities, and different compilations may have different performance/noise characteristics.

---

115. Resource Scaling

For:

2 qubits
20 qubits
2,000 qubits
2,000,000 qubits

the scheduler must change only according to the target resource description.

It must not require:

scheduler_v2
scheduler_v3
scheduler_mega

for larger machines.

---

116. Qubit Scaling

Qubit count is target data.

The scheduler must never infer a machine size from:

QubitId maximum

or:

highest physical index

unless explicitly defined by the target adapter.

Sparse identifiers must be supported.

---

117. Operation Arity

Do not assume operations are only:

1-qubit
2-qubit

Operations may have arbitrary supported arity.

The target determines whether an operation is executable.

---

118. Measurement

Measurement is not necessarily instantaneous.

Its duration and resource requirements come from the target.

The scheduler must support:

measurement start
measurement duration
readout resource
classical result availability
feedback latency

---

119. Reset

Reset is an executable operation with target-defined timing/resource requirements.

It must participate in dependencies.

---

120. Classical Computation

Classical computation may overlap quantum computation.

The scheduler must represent the target-defined relationship between:

quantum execution
classical execution
feedback

OpenQASM 3 explicitly recognizes classical processors operating concurrently with quantum operations where supported.

---

121. Communication

Communication must be treated as a resource-consuming operation.

For distributed quantum systems:

communication latency
+
resource availability
+
dependency readiness

must jointly determine schedule legality.

---

122. Calibration

The scheduler consumes a calibration snapshot.

It must not silently mutate calibration.

If calibration changes during dynamic execution, that must appear as an explicit target/availability update.

---

123. Target Snapshot

A schedule must be associated with the target state from which it was derived.

The target snapshot should identify:

target identity
capability version
timing model version
calibration version
availability version

This is necessary for reproducibility.

---

124. Stale Schedule Detection

A runtime/backend must be able to determine whether a schedule was created against a target state that is no longer valid.

Possible causes:

calibration changed
resource disabled
timing changed
topology changed
capability changed
maintenance began

The scheduler itself should not silently execute stale data.

---

125. Cancellation

Long scheduling operations must support cancellation.

Cancellation must be explicit through the context/configuration.

No global cancellation flag.

---

126. Deadlines

The scheduler may receive a compilation deadline.

If the deadline cannot be met, it must return a structured result/error rather than silently degrading quality.

---

127. Quality vs. Completion

The architecture must distinguish:

valid schedule

from:

optimal schedule

A valid heuristic schedule may be returned if the caller explicitly permits it.

The result must identify:

algorithm
objective
quality metrics
verification status

---

128. Verification Invariants

For every scheduled operation:

start >= release_time

For every dependency:

finish(A) <= start(B)

For every exclusive resource:

no overlapping reservations

For every capacity-limited resource:

usage(t) <= capacity(t)

For every duration:

finish = start + duration

For every alignment requirement:

start satisfies alignment

For every target operation:

target supports operation

For measurement/feedback:

consumer cannot execute before required result availability

And globally:

scheduled semantics == source semantics

---

129. Testing

Testing is part of the architecture, not a later activity.

The scheduler must contain:

tests/unit
tests/integration
tests/property
tests/regression
tests/scalability
tests/determinism
tests/fixtures

---

130. Unit Tests

Every foundational type must have tests for:

- valid construction;
- invalid construction;
- equality;
- ordering;
- serialization where applicable;
- overflow;
- boundary cases;
- error behavior.

---

131. Integration Tests

At minimum:

quantum::ir → scheduler
routing → scheduler
hardware → scheduler
QEC → scheduler
scheduler → verification
scheduler → serialization
scheduler → runtime boundary

---

132. Property Tests

Important invariants:

no exclusive resource overlaps
dependency ordering is preserved
capacity is never exceeded
valid schedules verify
invalid schedules fail verification

---

133. Regression Tests

Every discovered scheduling defect must receive a permanent regression test.

---

134. Determinism Tests

Given identical:

IR
target snapshot
configuration
seed

the scheduler must produce identical schedules in deterministic mode.

Run the same test multiple times.

---

135. Scalability Tests

Scale:

operation count
qubit count
dependency edges
resource count
resource contention
QEC rounds
communication nodes

without changing scheduler source code.

---

136. Required Edge Cases

Tests must cover:

zero operations
one operation
one qubit
many qubits
parallel operations
serial operations
single-resource contention
multi-resource contention
capacity > 1
zero-duration operation
symbolic duration
unknown duration
measurement
reset
conditional operation
feedback
deadline
release time
alignment
communication
distributed operation
QEC round
large DAG
invalid DAG
cycle
missing resource
missing duration
unsupported operation
stale target
serialization round-trip
deterministic mode
randomized mode

---

137. Repository Integration

The scheduler must integrate with:

quantum::frontend
quantum::ir
quantum::optimization
quantum::routing
quantum::hardware
quantum::zqn
quantum::error_correction / QEC subsystem
quantum::benchmarking
runtime/backend

The direction must remain acyclic.

---

138. Canonical Integration Pipeline

The production pipeline is:

Zamani source
      │
      ▼
frontend
      │
      ▼
canonical quantum::ir
      │
      ▼
optimization
      │
      ▼
routing
      │
      ▼
mapped IR
      │
      ▼
scheduling::adapters::ir
      │
      ▼
dependency analysis
      │
      ▼
resource analysis
      │
      ▼
timing analysis
      │
      ▼
constraint analysis
      │
      ▼
planner
      │
      ▼
schedule
      │
      ▼
transformations
      │
      ▼
verification
      │
      ▼
optimization/objective refinement
      │
      ▼
final verification
      │
      ▼
hardware lowering
      │
      ▼
runtime

---

139. Benchmarking Integration

The scheduler must expose metrics that the existing benchmarking subsystem can consume.

At minimum:

planning time
makespan
depth
critical path
parallelism
resource utilization
idle time
communication overhead
verification time
memory
operation count
dependency count

Benchmarking must not reimplement scheduling internals.

---

140. Diagnostics Integration

Diagnostics should allow the compiler to answer:

Why was this operation delayed?
Why could these operations not run simultaneously?
Which resource caused contention?
Which timing constraint forced movement?
Why did the scheduler select this algorithm?
Why was the schedule rejected?

---

141. Plugin Integration

A plugin must receive a complete scheduling context.

It must not need to access:

global compiler state
hardware handles
private scheduler internals

A plugin must return a standard "ScheduleResult".

---

142. Thread Safety

The scheduler must avoid global mutable state.

Scheduler instances should own their state.

Read-only target/context data should be safely shareable where Rust's type system permits.

Plugins must document thread-safety.

No hidden static caches whose behavior affects correctness.

---

143. Unsafe Rust

Unsafe Rust is forbidden.

Every scheduling source file must remain compatible with:

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

No FFI or low-level optimization may introduce unsafe code into this subsystem.

If an external subsystem requires unsafe internals, that must remain behind a separate boundary and must not make scheduling itself unsafe.

---

144. Rust Compatibility

The scheduler must support:

Rust 1.97 / 1.97.1
Rust 2021
stable Rust
no nightly features
no unsafe code

The repository's "Cargo.toml" currently contains:

rust-version = "1.97.1" or "1.97"

which is not valid Cargo syntax. It must be changed to one concrete value before production compilation.

Recommended policy:

rust-version = "1.97.1"

if that is the project's required minimum toolchain.

Do not put version alternatives into "Cargo.toml".

---

145. Dependency Policy

Scheduling should prefer the repository's existing dependencies.

Do not add external scheduling frameworks merely to implement:

DAG
priority queue
interval checking
resource reservation

unless there is a demonstrated production requirement.

Core scheduling should remain independently buildable.

---

146. Serialization Stability

Public serialized schedule formats must be versioned.

Never make internal Rust struct layout itself the serialization contract.

---

147. API Stability

Public module paths are part of the Zamani API.

Adding an implementation file should not require unrelated modules to import private internals.

Prefer:

public contract
    ↓
private implementation

rather than:

every file imports every other file

---

148. Avoid Circular Dependencies

The desired dependency structure is:

types
errors
limits
   │
   ▼
timing/resources
   │
   ▼
scheduler IR
   │
   ▼
constraints
   │
   ▼
context/config/result
   │
   ▼
policies
   │
   ▼
planners
   │
   ▼
algorithms
   │
   ▼
transformations/verification
   │
   ▼
optimization
   │
   ▼
dynamic/QEC/distributed
   │
   ▼
adapters/serialization/diagnostics/plugins

No child should import the parent composition root merely to obtain a type that should have been defined lower in the hierarchy.

---

149. No Glob-Driven Architecture

Avoid broad:

pub use foo::*;

exports.

Use explicit exports.

This is consistent with the canonical IR architecture, which deliberately avoids ambiguous glob exports and duplicate type ownership.

---

150. No Hidden Hardware Assumptions

Prohibited:

if qubit < 127

let channels = 8;

const DT: u64 = 1;

match topology {
    Square127 => ...
}

unless such values are explicitly part of a target object supplied from outside scheduling.

---

151. No Fixed Topology

The scheduler must accept:

line
grid
heavy hex
all-to-all
tree
star
irregular graph
modular
network
custom

without source changes.

---

152. No Fixed Gate Set

The scheduler must not assume:

H
X
Y
Z
CX
CZ

are the only operations.

It must schedule whatever canonical operations the target and adapter can describe.

---

153. No Fixed QEC Code

The scheduler must support QEC requirements generically.

Surface codes are one possible implementation.

Other codes must be able to supply the same scheduling boundary.

---

154. No Fixed Number of Rounds

QEC round count comes from the QEC request.

Never:

for _ in 0..10

because ten rounds happened to be used during development.

---

155. No Fixed Number of Ancillas

Ancilla resources are supplied by QEC and target resource models.

---

156. No Fixed Number of Control Channels

Channel capacities are target data.

---

157. No Fixed Time Grid

Do not create:

Vec<TimeSlot>

based on a universal scheduler resolution.

Use target-provided resolution only when required.

---

158. Sparse Event-Based Design

For large schedules prefer:

events
intervals
reservations
dependency edges
ready sets

rather than enormous dense matrices.

This is critical for scaling.

---

159. Large-DAG Design

The implementation must avoid:

- recursive graph traversal;
- repeated complete graph scans;
- repeated full resource scans;
- unnecessary cloning;
- quadratic algorithms where a linear/sparse alternative exists.

Where an algorithm inherently has worse complexity, the API/documentation must state it.

---

160. Exact vs Heuristic Scheduling

The scheduler must expose algorithm identity.

For example:

algorithm = asap
optimality = feasible

or:

algorithm = rcpsp_heuristic
optimality = heuristic

Never claim:

optimal = true

without an actual proof/guarantee.

---

161. Resource Reservation Invariant

A resource reservation must always be associated with:

operation
resource
start
finish
usage

A schedule must never contain an unexplained resource occupation.

---

162. Provenance

Every scheduled operation should be traceable to:

canonical IR operation

and, where applicable:

source location
optimization transformation
routing transformation
QEC transformation
scheduling transformation

This is essential for compiler debugging.

---

163. Explainability

The scheduler should eventually be able to produce:

operation O
ready at T1
resource available at T2
alignment requires T3
selected start = T3

This is more useful than merely reporting:

start = T3

---

164. Schedule Reproducibility

A production schedule must be reproducible from:

program identity
target identity/version
target snapshot
configuration
algorithm
seed
calibration snapshot
resource snapshot

---

165. Schedule Validity Is Target-Specific

A schedule valid for target A may not be valid for target B.

Therefore:

schedule

must not be treated as universally executable.

The portable artifact is:

program / canonical IR

while the schedule is:

program specialized for target state

---

166. Runtime Re-Scheduling

The architecture must allow:

static schedule
       │
       ▼
runtime event
       │
       ▼
affected region
       │
       ▼
incremental rescheduling

without requiring recompilation of the entire program where target semantics permit.

---

167. Incremental Scheduling

Eventually support rescheduling only the affected region after:

resource failure
calibration change
dynamic measurement
communication delay
runtime branch
hardware degradation

This must preserve unaffected schedule regions whenever legal.

---

168. Partial Scheduling

The scheduler should eventually support scheduling:

whole program

or:

region
block
QEC round
dynamic branch
distributed partition

where the surrounding context supplies required boundary constraints.

---

169. Hierarchical Scheduling

For very large systems:

global schedule
      │
      ├── module schedule
      │       │
      │       └── local schedule
      │
      ├── module schedule
      │
      └── network schedule

must be possible.

This prevents a single monolithic scheduler representation from becoming mandatory at extreme scale.

---

170. Multi-Level Scheduling

The architecture should eventually support:

logical scheduling
physical scheduling
control scheduling
pulse/resource scheduling
network scheduling

without conflating their semantic responsibilities.

---

171. Hardware Technology Evolution

Adding a new quantum technology should require:

new target adapter
new capability/resource descriptions
possibly new constraints

not a rewrite of:

planner
dependency graph
ASAP
ALAP
verification

---

172. Simulator Compatibility

A simulator should be represented as a target.

The scheduler should therefore be capable of scheduling:

real QPU
simulator
emulator
hybrid target

through the same context abstraction.

---

173. Testing Against Simulators

Simulator schedules can validate:

semantic preservation
timing constraints
dependency correctness
dynamic control

without requiring hardware access.

---

174. Hardware Integration

Hardware integration must happen through:

adapters::hardware

and not by adding backend-specific branches throughout scheduling.

---

175. Compiler Integration

The compiler should treat scheduling as an explicit pass:

IR
 ↓
optimization
 ↓
routing
 ↓
scheduling
 ↓
verification
 ↓
lowering

The scheduler must not silently execute the program.

---

176. Runtime Integration

Runtime receives a validated target-specialized schedule.

Runtime owns:

execution
job submission
backend communication
monitoring
results

Scheduling owns:

when

not:

execute now

---

177. Security

Scheduling inputs may be untrusted.

Therefore:

- validate serialized data;
- validate plugin outputs;
- validate resource identities;
- validate durations;
- validate graph structure;
- validate arithmetic;
- enforce explicit resource limits;
- avoid panics for untrusted input;
- avoid unsafe code.

---

178. Failure Handling

The scheduler must fail explicitly for:

unsatisfiable dependencies
missing resource
missing timing
invalid operation
unsupported operation
capacity conflict
deadline failure
invalid alignment
invalid target
invalid dynamic dependency

It must not silently omit operations.

---

179. Empty Program

An empty program is valid if the surrounding compiler permits it.

The scheduler should return a valid zero-work schedule rather than panic.

---

180. Single Operation

A single legal operation must produce a schedule with:

valid start
valid duration
valid resource reservation
valid verification

---

181. Parallel Operations

Independent operations should be allowed to overlap when:

dependencies allow
resources allow
timing allows
target allows
policy allows

---

182. Serial Operations

Operations with dependencies or resource conflicts must be ordered.

---

183. Multi-Qubit Operations

All participating physical resources must be reserved for the appropriate interval.

No scheduler may assume that only the first or second operand consumes a resource.

---

184. Measurement and Feedback

The scheduler must support:

measurement
→ classical result
→ classical processing
→ conditional quantum operation

with target-defined latency.

---

185. Communication Scheduling

Distributed execution requires:

communication resource
communication duration
communication dependency

as first-class schedule data.

---

186. QEC Scheduling

QEC operations must participate in:

qubit resources
ancilla resources
measurement resources
classical resources
communication resources
round dependencies

---

187. Calibration-Aware Scheduling

Calibration may influence:

duration
fidelity
resource availability
alignment

but the scheduler must receive calibration through a target snapshot.

---

188. Noise-Aware Scheduling

Noise information may affect objective scoring.

For example:

schedule A:
shorter but noisier

schedule B:
longer but more reliable

The selected schedule depends on explicit objective configuration.

---

189. No Implicit Optimization

The scheduler must not silently optimize for fidelity, duration, energy or idle time.

The objective must be explicit.

---

190. No Implicit Randomization

Randomization must be explicitly requested.

---

191. No Global Registry Requirement

Plugin registries may exist, but the scheduler must be able to operate without mutable process-wide registries.

A registry should be passed explicitly where possible.

---

192. API Composition

The preferred high-level conceptual API is:

schedule(program, target, policy)

not:

schedule(program, 127, 100ns, 8)

The scheduler should receive:

program
+
target description
+
policy/configuration

---

193. Public API Principle

Public APIs should be small.

Complexity belongs inside:

context
planner
resource model
timing model
constraints

rather than requiring callers to know internal implementation structures.

---

194. README Contract

This README is the architectural contract.

Any implementation that contradicts this document must either:

1. be corrected; or
2. result in this README being intentionally revised through an architectural decision.

Do not silently create incompatible implementations.

---

195. Production Readiness Checklist

The scheduler is not production-ready until all applicable items below are true.

Architecture

- [ ] canonical scheduler vocabulary;
- [ ] canonical errors;
- [ ] no hard-coded machine limits;
- [ ] no duplicate qubit identities;
- [ ] "quantum::ir::qubit::QubitId" used correctly;
- [ ] physical qubit identity remains distinct;
- [ ] routing/scheduling separation;
- [ ] hardware/scheduling separation;
- [ ] QEC/scheduling separation;
- [ ] runtime/scheduling separation.

Timing

- [ ] duration model;
- [ ] time model;
- [ ] target resolution;
- [ ] alignment;
- [ ] windows;
- [ ] deadlines;
- [ ] symbolic timing;
- [ ] target-resolved timing;
- [ ] explicit delays.

Resources

- [ ] generic resources;
- [ ] resource pools;
- [ ] reservations;
- [ ] calendars;
- [ ] availability;
- [ ] capacity;
- [ ] shared resources;
- [ ] exclusive resources;
- [ ] hierarchical resources;
- [ ] communication resources.

Algorithms

- [ ] ASAP;
- [ ] ALAP;
- [ ] list scheduling;
- [ ] critical-path scheduling;
- [ ] resource-constrained scheduling;
- [ ] adaptive scheduling;
- [ ] deterministic mode;
- [ ] randomized mode with explicit seed.

Dynamic

- [ ] classical dependencies;
- [ ] conditional operations;
- [ ] feedback;
- [ ] runtime events;
- [ ] partial/static scheduling boundary.

QEC

- [ ] QEC interface;
- [ ] syndrome scheduling;
- [ ] round scheduling;
- [ ] stabilizer scheduling;
- [ ] no hard-coded code distance;
- [ ] no hard-coded ancilla count;
- [ ] no hard-coded rounds.

Distributed

- [ ] nodes;
- [ ] links;
- [ ] communication;
- [ ] network scheduling;
- [ ] multi-module support;
- [ ] multi-QPU support.

Verification

- [ ] structural verification;
- [ ] dependency verification;
- [ ] resource verification;
- [ ] timing verification;
- [ ] semantic verification;
- [ ] final verification.

Optimization

- [ ] makespan;
- [ ] depth;
- [ ] idle time;
- [ ] fidelity;
- [ ] energy;
- [ ] multi-objective.

Integration

- [ ] canonical IR adapter;
- [ ] routing adapter;
- [ ] hardware adapter;
- [ ] QEC adapter;
- [ ] ZQN integration boundary;
- [ ] compiler integration;
- [ ] runtime integration;
- [ ] benchmarking integration.

Production

- [ ] serialization;
- [ ] diagnostics;
- [ ] provenance;
- [ ] reproducibility;
- [ ] plugin API;
- [ ] cancellation;
- [ ] deadlines;
- [ ] security validation;
- [ ] no unsafe Rust;
- [ ] Rust 1.97/1.97.1 compatibility.

Testing

- [ ] unit tests;
- [ ] integration tests;
- [ ] property tests;
- [ ] regression tests;
- [ ] scalability tests;
- [ ] determinism tests;
- [ ] serialization round-trip tests;
- [ ] large-DAG tests;
- [ ] dynamic-circuit tests;
- [ ] distributed tests;
- [ ] QEC tests.

---

196. Final Architecture

The complete Zamani scheduling architecture is:

                       ZAMANI PROGRAM
                              │
                              ▼
                       quantum::frontend
                              │
                              ▼
                    ┌────────────────────┐
                    │    quantum::ir     │
                    │   canonical WHAT   │
                    └─────────┬──────────┘
                              │
                              ▼
                         optimization
                              │
                              ▼
                           routing
                         "WHERE?"
                              │
                              ▼
                    scheduling::adapters::ir
                              │
             ┌────────────────┼────────────────┐
             │                │                │
             ▼                ▼                ▼
        dependencies       resources         timing
             │                │                │
             └────────────────┼────────────────┘
                              │
                              ▼
                         constraints
                              │
                              ▼
                           policy
                              │
                              ▼
                           planner
                              │
             ┌────────────────┼─────────────────┐
             │                │                 │
             ▼                ▼                 ▼
            ASAP             ALAP          resource-aware
             │                │                 │
             └────────────────┼─────────────────┘
                              │
                              ▼
                         schedule result
                              │
                 ┌────────────┼────────────┐
                 │            │            │
                 ▼            ▼            ▼
              dynamic        QEC       distributed
                 │            │            │
                 └────────────┼────────────┘
                              │
                              ▼
                       transformations
                              │
                              ▼
                         verification
                              │
                              ▼
                         optimization
                              │
                              ▼
                      final verification
                              │
                              ▼
                    hardware-target lowering
                              │
                ┌─────────────┼─────────────┐
                │             │             │
                ▼             ▼             ▼
               QPU        simulator      emulator
                │             │             │
                └─────────────┼─────────────┘
                              │
                              ▼
                           runtime

---

197. The Core Zamani Contract

The entire subsystem can be reduced to one invariant:

PROGRAM
+
TARGET
+
RESOURCES
+
TIMING
+
CONSTRAINTS
+
POLICY
+
OBJECTIVE
        │
        ▼
    SCHEDULE

The program says:

«WHAT»

Routing says:

«WHERE»

Scheduling says:

«WHEN»

Hardware says:

«CAN THIS TARGET EXECUTE IT?»

Runtime says:

«EXECUTE IT»

This separation is mandatory.

---

198. Final Scalability Contract

A valid implementation must support the same architectural pipeline for:

1 qubit
        ↓
a few qubits
        ↓
small QPU
        ↓
large QPU
        ↓
multi-chip QPU
        ↓
multi-module system
        ↓
multi-QPU system
        ↓
distributed quantum network
        ↓
future heterogeneous quantum system

without modifying the Zamani program merely because the target became larger.

The scheduler specializes itself from target data.

It does not encode target assumptions.

---

199. Final Non-Negotiable Rules

The following rules are absolute:

1. No unsafe Rust.
2. No fixed machine-size limits.
3. No hard-coded qubit counts.
4. No hard-coded topology.
5. No hard-coded resource counts.
6. No hard-coded timing constants.
7. No duplicate "QubitId".
8. Use "quantum::ir::qubit::QubitId".
9. Use "PhysicalQubitId" for physical identity.
10. Do not silently map logical to physical qubits.
11. Do not implement routing inside scheduling.
12. Do not implement hardware discovery inside scheduling.
13. Do not implement QEC decoding inside scheduling.
14. Do not implement source parsing inside scheduling.
15. Do not create a second semantic quantum IR.
16. Do not hide optimization objectives.
17. Do not hide randomness.
18. Do not accept an unverified production schedule.
19. Do not silently discard an unschedulable operation.
20. Do not claim optimality for a heuristic algorithm.
21. Do not use dense machine-sized time-slot matrices as the universal representation.
22. Do not make hardware vendor APIs part of scheduler core.
23. Do not require a particular quantum technology.
24. Do not require a particular number of QEC rounds.
25. Do not require a particular QEC code.
26. Do not make distributed execution a separate incompatible scheduler.
27. Do not make dynamic circuits require a second scheduling architecture.
28. Do not introduce global mutable scheduler state.
29. Do not use undocumented cross-file assumptions.
30. Every public contract must specify its integration boundary.

---

200. Production Definition

"quantum::scheduling" may be declared production-ready only when:

canonical IR
      +
routing
      +
target capabilities
      +
resource model
      +
timing model
      +
constraints
      +
scheduler algorithms
      +
dynamic execution model
      +
QEC integration
      +
distributed integration
      +
verification
      +
optimization
      +
serialization
      +
diagnostics
      +
testing
      +
compiler/runtime integration

all operate under the contracts defined by this README.

The objective is not merely to implement an ASAP scheduler.

The objective is:

«Build a general quantum scheduling substrate in which a Zamani program expresses computation once, while scheduling specializes that computation to whatever finite execution resources are available—from the smallest quantum system to arbitrarily large heterogeneous and distributed quantum machines—without embedding machine-size, topology, timing, vendor, resource-count, or technology assumptions into the program or scheduler.»

This is the architectural definition of "write once, scale everywhere" for Zamani Quantum Scheduling.

---

External architectural references

Zamani's scheduler design is consistent with several established principles in current quantum compiler ecosystems:

- OpenQASM 3 explicitly separates timing intent from target-resolved durations and provides duration, stretch, delay, box and barrier timing constructs.
- OpenQASM 3 also supports classical control and runtime computation, reinforcing the requirement for dynamic scheduling.
- Current Qiskit exposes distinct ASAP and ALAP scheduling strategies, reinforcing the separation of scheduling policy from other compilation stages.
- TKET's compiler model emphasizes platform-agnostic compilation followed by target-specific constraints, which is consistent with Zamani's separation of canonical IR, routing, scheduling and hardware lowering.

---

Repository-specific integration note

The existing Zamani "quantum::ir" contract already states that canonical IR is hardware-independent, has no architectural maximum for qubits/operations/depth/topology, and assigns routing and scheduling to downstream systems.

The existing scheduling tree already contains the major production boundaries described above, including "adapters", "algorithms", "constraints", "diagnostics", "distributed", "dynamic", "ir", "optimization", "planners", "plugins", "policies", "qec", "resources", "serialization", "timing", "transformations", "verification", and tests.

The existing stabilizer scheduler has also already been moved toward the correct compatibility-facade model rather than remaining a second hard-coded stabilizer scheduling implementation.The key correction from the repository inspection is that this README should now be treated as the frozen contract for the scheduling tree that already exists, rather than creating yet another parallel architecture. The next implementation work should therefore be performed against this contract, file-by-file, starting with the lowest-dependency contracts and only composing them after their individual invariants are complete.