Worked for 9s

Below is the complete production specification for:

src/quantum/scheduling/SCALABILITY.md

I checked the repository's existing scheduling IR and foundational types, including the explicit canonical-identity requirement to use quantum::ir::qubit::{QubitId, PhysicalQubitId} rather than creating scheduler-local qubit IDs.   I also checked the current stabilizer compatibility boundary, which is already designed not to own a second scheduling algorithm.

The design below additionally accounts for OpenQASM 3.1's explicit timing, delays, dynamic/classical control, and heterogeneous gate durations, and for modern scheduling architectures that separate ASAP/ALAP analysis, target constraints, and explicit idle-time representation.  Rust 1.97.1 is specifically supported; it is the preferred pinned baseline here because Rust 1.97.1 fixed an LLVM miscompilation issue present in 1.97.0. 

Zamani Quantum Scheduling — Scalability and Production Architecture

Path: "src/quantum/scheduling/SCALABILITY.md"

Status: Production architecture contract

Language: Rust

Required compiler baseline: Rust 1.97 / Rust 1.97.1

Preferred compiler: Rust 1.97.1

Edition: Rust 2021

Safety: "unsafe" forbidden

Primary objective:

«A Zamani quantum program is written once at the semantic level and can be compiled and scheduled for any compatible quantum execution target whose resources, topology, timing model, capabilities, and constraints are supplied by that target.»

"Any size" and "infinity" mean that the scheduling architecture introduces no artificial finite machine-size ceiling.

They do not mean that a finite compiler process can physically allocate infinite memory, execute infinite operations, or construct an infinite schedule.

A concrete compilation is necessarily bounded by the resources available to that compilation:

- host memory;
- host address space;
- CPU time;
- storage;
- target resources;
- target capabilities;
- explicit user/compiler limits;
- operating-system limits;
- distributed-system capacity.

Those limits must be explicit inputs or environmental constraints. They must never be hidden scheduler constants.

---

1. Purpose

This document defines the scalability contract for:

crate::quantum::scheduling

The scheduler is responsible for answering:

«WHEN can each already-defined quantum operation execute?»

It is not responsible for deciding:

«What does the program mean?»

That belongs to:

crate::quantum::ir

It is not responsible for deciding:

«Where should logical qubits be placed?»

That belongs to:

crate::quantum::routing

It is not responsible for deciding:

«What physical hardware exists?»

That belongs to:

crate::quantum::hardware

It is not responsible for deciding:

«What errors/noise model applies?»

That belongs to:

crate::quantum::zqn

It is not responsible for implementing:

«QEC decoding or complete QEC algorithms.»

That belongs to:

crate::quantum::error_correction

It is not responsible for actual execution.

That belongs to the runtime/hardware execution layers.

---

2. Core scalability invariant

The scheduler MUST NOT contain assumptions equivalent to:

const MAX_QUBITS: usize = 1000;
const MAX_OPERATIONS: usize = 1_000_000;
const MAX_ROUNDS: usize = 100;
const MAX_CHANNELS: usize = 64;
const MAX_DEPTH: usize = 1_000_000;

No scheduler module may introduce:

- maximum qubit counts;
- maximum physical qubit counts;
- maximum logical qubit counts;
- maximum operation counts;
- maximum dependency counts;
- maximum graph depth;
- maximum graph width;
- maximum resource count;
- maximum channel count;
- maximum control count;
- maximum QEC distance;
- maximum QEC rounds;
- maximum communication links;
- maximum distributed nodes;
- maximum scheduling horizon;
- maximum operation arity.

If a caller wants a limit, the caller supplies it through the explicit scheduling limits/configuration contract.

---

3. Meaning of "scale from atom to everywhere"

The architecture must support the same semantic program across targets such as:

one quantum degree of freedom
        ↓
one logical qubit
        ↓
small QPU
        ↓
large QPU
        ↓
multi-chip system
        ↓
multi-QPU system
        ↓
distributed quantum computer
        ↓
quantum data center
        ↓
quantum network
        ↓
future heterogeneous quantum system

The program remains semantically unchanged.

Only the compilation context changes.

Conceptually:

                    ONE ZAMANI PROGRAM
                           │
                           ▼
                    canonical quantum IR
                           │
                           ▼
                       optimization
                           │
                           ▼
                         routing
                           │
                           ▼
                       scheduling
                           │
             ┌─────────────┼─────────────┐
             ▼             ▼             ▼
         tiny target    large target   distributed target
             │             │             │
             ▼             ▼             ▼
          schedule      schedule      schedule
             │             │             │
             └─────────────┼─────────────┘
                           ▼
                       execution

The source program does not contain:

if machine_has_100_qubits

or:

use qubit 47

or:

reserve channel 7

unless those are explicit target-level constructs produced during lowering.

---

4. Canonical identity ownership

Scheduling MUST NOT define another logical or physical qubit identity.

Whenever scheduler code needs logical qubit identity, it MUST use:

use crate::quantum::ir::qubit::QubitId;

Whenever it needs physical qubit identity, it MUST use:

use crate::quantum::ir::qubit::PhysicalQubitId;

The repository already establishes "quantum::ir::qubit" as the canonical ownership boundary for these identities.

The following are prohibited:

pub type QubitId = u64;

inside scheduling.

Also prohibited:

struct SchedulerQubitId(...);

when it represents a quantum logical or physical qubit.

Scheduler-specific identities are allowed only for scheduler-owned concepts such as:

ScheduleId
DependencyId
ReservationId
EpochId
SchedulerSessionId

They must never masquerade as quantum identities.

---

5. Canonical operation ownership

Scheduling MUST NOT redefine:

Gate
QuantumOperation
QuantumCircuit
QubitId
PhysicalQubitId
OperationId
ResourceId

where canonical equivalents already exist.

The scheduling IR is a normalized scheduling view of canonical quantum IR.

The repository's scheduling IR explicitly establishes this separation.

The direction is:

quantum::ir
    │
    ▼
scheduling::adapters::ir
    │
    ▼
scheduling::ir

Never:

scheduling::ir
    │
    ▼
new quantum semantic IR

---

6. Production architecture

The complete scheduling subsystem is:

src/quantum/scheduling/
│
├── mod.rs
├── types.rs
├── errors.rs
├── config.rs
├── limits.rs
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
├── qec/
│   ├── mod.rs
│   ├── interface.rs
│   ├── syndrome.rs
│   ├── rounds.rs
│   └── stabilizer.rs
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

Every file is a separate responsibility.

---

7. Dependency layering

The subsystem MUST maintain this dependency direction:

FOUNDATION
    │
    ├── types
    ├── errors
    └── limits
    │
    ▼
DOMAIN MODELS
    │
    ├── timing
    ├── resources
    └── scheduling IR
    │
    ▼
CONSTRAINTS
    │
    ▼
CONTEXT / CONFIGURATION
    │
    ▼
POLICIES
    │
    ▼
PLANNERS
    │
    ▼
ALGORITHMS
    │
    ▼
TRANSFORMATIONS
    │
    ▼
VERIFICATION
    │
    ▼
OPTIMIZATION
    │
    ▼
ADAPTERS
    │
    ▼
EXTERNAL QUANTUM SUBSYSTEMS

No lower-level module may depend on a higher-level planner.

---

8. Foundational files

"types.rs"

Own only stable scheduler vocabulary:

ScheduleId
DependencyId
ReservationId
EpochId
SchedulerSessionId
TimePoint
Duration
Priority
Cost
Makespan
Slack
ScheduleStatus
SchedulingPhase

It MUST import canonical identities rather than recreate them:

use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};

and canonical operation/resource identities from their canonical location.

No hardware constants.

No algorithm logic.

No vendor logic.

No topology.

No QEC implementation.

The existing repository implementation already follows this foundational ownership direction.

---

9. "errors.rs"

Define structured scheduling errors.

At minimum:

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
Overflow
Internal

Every error that refers to a scheduler object should preserve structured identity.

Do not make algorithms inspect error strings.

---

10. "limits.rs"

This is the explicit limit boundary.

Limits MUST be optional.

Examples:

max_operations: Option<u64>
max_dependencies: Option<u64>
max_resources: Option<u64>
max_schedule_horizon: Option<Duration>
max_memory: Option<u64>
deadline: Option<TimePoint>
max_parallelism: Option<u64>

These are caller policies, not architectural ceilings.

An omitted limit means:

«No artificial scheduler limit was requested.»

It does not mean:

«Infinite physical memory exists.»

---

11. Timing scalability

The timing system MUST remain target-independent.

The architecture is:

canonical operation
       │
       ▼
target timing model
       │
       ├── duration
       ├── resolution
       ├── alignment
       ├── availability windows
       └── timing constraints
       │
       ▼
scheduler

Modern quantum languages already recognize explicit timing and timing-independent scheduling intent. OpenQASM 3.1 supports duration/stretch concepts, delays, boxes and barriers.

Therefore scheduling MUST NOT assume:

every gate = same duration

or:

all targets use nanoseconds

or:

all targets use one clock.

---

12. "timing/duration.rs"

Represent:

Known
Symbolic
Calibrated
Unknown

where required by the rest of the architecture.

A duration may originate from:

target instruction definition
calibration
operation metadata
timing constraint
runtime information

The scheduler must reject an unresolved duration when the selected algorithm requires a concrete duration.

It must not silently invent one.

---

13. "timing/time.rs"

Provide:

TimePoint
Duration
TimeInterval

All arithmetic MUST be checked.

Never use:

wrapping_add
wrapping_sub

for scheduling semantics.

Use:

checked_add
checked_sub

and propagate overflow as a structured error.

---

14. "timing/resolution.rs"

The target provides timing resolution.

Possible models:

continuous
rational
integer tick
device sample period
custom target timebase

Do not embed:

dt = 1

or:

dt = 1ns

in the scheduler.

---

15. "timing/alignment.rs"

Alignment is a target constraint.

Examples:

operation alignment
channel alignment
measurement alignment
frame alignment
sample alignment
control alignment

A schedule that is dependency-valid but violates alignment is not executable.

Modern Qiskit explicitly separates scheduling analysis from later constrained rescheduling against backend timing restrictions.

Zamani should maintain the same architectural separation.

---

16. "timing/windows.rs"

Represent:

release time
earliest start
latest start
earliest finish
latest finish
deadline
availability window

All values must remain optional where semantics allow.

---

17. Resource scalability

The resource model is fundamental to "atom to everywhere."

A quantum operation may consume:

logical qubits
physical qubits
control channels
readout channels
measurement resources
resonators
couplers
laser resources
microwave resources
ancillas
classical processors
feedback paths
communication links
network bandwidth
entanglement-generation resources
memory
cooldown resources

Therefore a scheduler MUST NOT assume that a quantum operation consumes only qubits.

---

18. "resources/resource.rs"

Define generic resource semantics.

Resource types include:

Exclusive
Shared
CapacityLimited
Consumable
Reusable
Hierarchical
TimeDependent

A resource has:

ResourceId
capacity
availability
requirements
constraints
metadata

No fixed number of resources is allowed.

---

19. "resources/pool.rs"

Pools represent scalable groups of equivalent or related resources.

Examples:

measurement channels
control channels
classical workers
communication links
ancilla pools

The pool size comes from the target.

The scheduler must never contain:

let channels = 8;

---

20. "resources/reservation.rs"

A reservation represents:

operation
resource
start
duration
finish
usage mode
capacity consumed

Reservations are the primary mechanism preventing resource collisions.

---

21. "resources/calendar.rs"

Calendars must support:

free
busy
reserved
disabled
maintenance
calibration
degraded
unknown

They must support sparse availability rather than allocating a giant time matrix.

This is essential for very long scheduling horizons.

---

22. "resources/availability.rs"

Availability may change.

Therefore the scheduler must support snapshots.

A compilation context should consume an immutable availability snapshot rather than continuously querying hardware.

This makes schedules reproducible.

---

23. Scheduling IR scalability

The scheduler IR is not the quantum semantic IR.

The pipeline is:

quantum::ir
      │
      ▼
adapters::ir
      │
      ▼
scheduling::ir

The repository already documents this ownership boundary.

The scheduling IR must retain:

canonical operation identity
canonical qubit identity
canonical physical-qubit identity
resource requirements
duration information
dependency information
provenance
metadata

---

24. "ir/operation.rs"

A scheduling operation should represent:

source operation
operation identity
operation class
logical operands
physical operands when routing has resolved them
resource requirements
duration requirement
timing windows
precedence information
classical dependencies
metadata
provenance

Never reconstruct quantum semantics from scheduler metadata.

---

25. "ir/dependency.rs"

Dependencies must support more than qubit overlap.

Required dependency classes include:

QuantumData
ClassicalData
ReadAfterWrite
WriteAfterRead
WriteAfterWrite
Measurement
Reset
Control
Resource
Communication
Barrier
ExplicitUser
QEC

This enables dynamic circuits and distributed systems.

---

26. "ir/graph.rs"

The graph must support:

predecessors
successors
indegree
outdegree
ready nodes
cycle detection
topological traversal
incremental validation

For static scheduling, the graph must be acyclic.

Dynamic control flow must not be incorrectly forced into a static DAG.

---

27. Graph memory scalability

Never construct a dense matrix representing all operation relationships.

Do not use:

NxN dependency matrix

for arbitrary programs.

Prefer sparse adjacency structures.

Target complexity for ordinary graph construction/analysis:

O(V + E)

where:

V = operations
E = dependencies

The architecture must remain correct even when "V" and "E" are very large.

---

28. Avoid time-slot matrices

Do not represent the schedule as:

qubit × time-slot

or:

resource × time-slot

because the schedule horizon can be much larger than the number of operations.

Use event/interval-based structures:

operation → interval
resource → reservations
dependency → edges

This makes sparse schedules efficient.

---

29. "ir/critical_path.rs"

Support:

earliest start
earliest finish
latest start
latest finish
slack
critical path
critical operations

All traversal should be iterative where graph depth can be large.

Do not rely on recursive DFS for arbitrary graph depth.

---

30. Context architecture

"context.rs" must aggregate immutable scheduling inputs.

Conceptually:

SchedulingContext
├── scheduling IR
├── dependency graph
├── target capabilities
├── timing model
├── resource model
├── availability snapshot
├── calibration snapshot
├── constraints
├── policy
├── objective
├── deterministic configuration
├── random seed where required
└── explicit limits

The scheduler should not discover hardware.

---

31. Configuration

"config.rs" contains declarative scheduler configuration.

Examples:

policy
objective
deterministic
seed
verification mode
optimization mode
timing mode
parallelism
distributed mode
diagnostic level
limits

No global mutable configuration.

No process-wide scheduler singleton.

---

32. Result scalability

"result.rs" must contain:

ScheduleId
schedule
operation timings
resource reservations
makespan
depth
idle intervals
critical path
objective score
verification result
provenance
diagnostics
reproducibility metadata

The result must make it possible to explain why a schedule was produced.

---

33. ASAP

"policies/asap.rs" and "algorithms/asap.rs" must implement:

«Earliest legal execution.»

The algorithm must consider:

dependencies
resource availability
timing windows
alignment
constraints
target capabilities

ASAP must never mean:

start = predecessor_finish

without checking resources and timing constraints.

ASAP is a policy, not the entire scheduler.

Modern quantum compiler scheduling exposes ASAP as a distinct scheduling strategy.

---

34. ALAP

ALAP means:

«Latest legal execution subject to the schedule's constraints/deadline.»

It requires:

deadline or makespan bound
dependency graph
durations
resource constraints
alignment
timing windows

It must not invent an implicit deadline.

If no meaningful latest boundary exists, the planner must return an explicit error or use a configured alternative policy.

---

35. List scheduling

List scheduling should be the primary general-purpose scalable heuristic.

Conceptually:

ready operations
      │
      ▼
priority selection
      │
      ▼
resource feasibility
      │
      ▼
timing feasibility
      │
      ▼
reserve
      │
      ▼
advance event frontier
      │
      └──────────► repeat

The implementation should avoid scanning every operation at every time coordinate.

---

36. Event-driven scheduling

For large systems, use event-driven progression.

Important events include:

operation finished
resource released
measurement completed
classical result available
communication completed
QEC round completed
availability changed
deadline reached

This is preferable to repeatedly incrementing a simulated clock through empty periods.

---

37. Resource-constrained scheduling

The scheduler must support resource-constrained planning.

This is essential because two operations may be:

quantum-data independent

yet still conflict because they share:

control channel
readout channel
resonator
laser
communication link
classical processor

Therefore:

dependency independent

does not imply:

execution independent

---

38. Exact versus heuristic scheduling

The architecture must distinguish:

Exact
Heuristic
Approximation
Deterministic heuristic
Stochastic heuristic
Adaptive

Global optimal resource-constrained scheduling may be computationally difficult.

Therefore Zamani must not claim:

«Every arbitrary scheduling problem is solved optimally.»

Instead the result must report:

algorithm
objective
constraints
quality metrics
optimality status

---

39. Adaptive scheduling

"algorithms/adaptive.rs" may choose a scheduling method based on:

graph width
graph depth
resource pressure
critical-path structure
communication density
QEC requirements
target characteristics
operation count
deadline

But adaptation MUST NOT change quantum semantics.

Adaptive decisions must remain reproducible when deterministic mode is enabled.

---

40. Determinism

Deterministic scheduling must support:

same program
+
same target snapshot
+
same calibration snapshot
+
same configuration
+
same seed
=
same schedule

Tie-breaking must be deterministic.

Do not rely on nondeterministic "HashMap" iteration for semantic scheduling decisions.

Where ordering matters, use stable ordering explicitly.

---

41. Parallel scheduling

Parallel analysis is allowed.

However:

parallel analysis

must not become:

nondeterministic scheduling

when deterministic mode is requested.

A safe model is:

parallel candidate analysis
        ↓
deterministic arbitration
        ↓
reservation

---

42. Distributed scheduling

Distributed scheduling must be represented as a first-class extension.

Support:

node
link
communication resource
network topology
remote operation
entanglement generation
teleportation
classical communication
synchronization

The global scheduler may partition work, but independent local schedules cannot simply be assumed to compose into a valid global schedule.

Cross-node constraints must be represented explicitly.

---

43. Dynamic circuits

The scheduler must support:

measurement
   ↓
classical computation
   ↓
condition
   ↓
quantum operation

A runtime dependency cannot be falsely represented as a compile-time ordering.

The architecture must distinguish:

static dependency

from:

runtime dependency

OpenQASM 3 explicitly supports classical feed-forward flow control and real-time classical computation, so Zamani's scheduler must not be restricted to static circuits.

---

44. Runtime scheduling

"dynamic/runtime.rs" must represent operations whose exact timing depends on runtime information.

Examples:

measurement result
runtime branch
feedback latency
dynamic resource availability
remote communication

The static compiler may produce a scheduling template containing unresolved runtime timing.

---

45. Explicit delays

"transformations/delays.rs" must make required idle intervals explicit when the target representation requires them.

The principle is:

schedule
   ↓
idle interval
   ↓
explicit Delay

This follows modern quantum scheduling practice, where scheduling can make idle periods explicit and then apply wall-time-sensitive transformations.

Delays must not be inserted merely because a particular backend implementation happens to use them internally.

The target representation decides whether explicit delay operations are required.

---

46. Alignment and padding

Scheduling transformation order should be:

initial schedule
      ↓
target constraints
      ↓
alignment
      ↓
padding/delay insertion
      ↓
optional wall-time transformations
      ↓
verification

Never modify timing without re-verifying all dependencies and resource constraints.

---

47. Dynamical decoupling

"transformations/dynamical_decoupling.rs" is optional.

It is not fundamental scheduling.

It should consume a valid schedule and apply a target-supported transformation.

It must never be required by the core scheduler.

---

48. Verification

Verification is mandatory for production schedules.

The following must be checked.

Structural

Every input operation represented.

No duplicate operation.

No missing operation.

Dependency

For every dependency:

finish(predecessor) <= start(successor)

unless the dependency explicitly represents a runtime condition.

Resource

For every resource:

usage <= capacity

at all relevant intervals.

Timing

Check:

duration
alignment
release time
latest time
deadline
availability

Semantic

The schedule must preserve the semantics of the source program.

Scheduling must not:

remove gates
change operands
change controls
change measurements
change conditions
change quantum identity

unless an explicitly declared semantics-preserving transformation has been applied elsewhere.

---

49. Verification must happen twice

Production pipeline:

schedule
   ↓
verification
   ↓
transformations/optimization
   ↓
final verification

The second verification is mandatory because transformations may invalidate assumptions established by the first verification.

---

50. Optimization objectives

Scheduling optimization must be explicit.

Supported objectives may include:

makespan
depth
idle time
resource utilization
fidelity
energy
communication overhead
multi-objective score

No implicit weighting.

For example, never hard-code:

fidelity_weight = 0.7
time_weight = 0.3

Weights must be configuration.

---

51. Fidelity-aware scheduling

If "quantum::zqn" provides error/noise information, scheduling may consume:

gate error
duration-dependent error
crosstalk
drift
readout error
idle error
resource uncertainty

The scheduler must not recreate the ZQN model.

Use:

ZQN
 │
 ▼
scheduling adapter
 │
 ▼
objective/cost model

This keeps noise ownership outside scheduling.

---

52. Hardware integration

The hardware subsystem supplies:

operations
durations
qubits
topology
timing resolution
alignment
channels
resource capacities
measurement constraints
control constraints
availability
calibration
communication capability

The scheduler consumes these through:

adapters/hardware.rs

It must not depend on vendor SDKs directly.

---

53. Hardware technology independence

The scheduler must work without special-case branches such as:

if superconducting { ... }
if trapped_ion { ... }
if neutral_atom { ... }
if photonic { ... }

Technology-specific behavior belongs in target adapters.

The scheduler operates on capabilities and constraints.

This permits future technologies without modifying scheduler algorithms.

---

54. Routing integration

Routing answers:

«WHERE?»

Scheduling answers:

«WHEN?»

Pipeline:

logical program
      ↓
routing
      ↓
physical/mapped operations
      ↓
scheduling
      ↓
timed operations

Scheduling must never silently perform logical-to-physical routing.

If physical mapping is unavailable and required by the selected target, the adapter must reject the input.

---

55. QEC integration

The current "stabilizer_scheduler.rs" must remain a compatibility facade rather than becoming another scheduler implementation. The repository's current file explicitly establishes that design.

Correct architecture:

error_correction
       ↓
QEC scheduling requirements
       ↓
scheduling::qec
       ↓
scheduling::adapters::qec
       ↓
generic scheduling IR
       ↓
generic planner

The QEC layer supplies:

syndrome dependencies
ancilla requirements
round dependencies
measurement requirements
feedback requirements

The generic scheduler decides timing.

---

56. QEC scalability

Never hard-code:

distance = 3

or:

4 neighbors

or:

9 ancillas

or:

100 rounds

The QEC model must describe the actual requested code.

The scheduler only consumes the resulting requirements.

---

57. Distributed QEC

The architecture must eventually support:

local QEC
multi-module QEC
distributed syndrome extraction
remote stabilizer operations
communication-assisted QEC

without changing the scheduling abstraction.

---

58. Serialization

A production schedule needs a versioned serialization model.

Serialization must include enough information to reconstruct:

schedule identity
program identity/provenance
target identity
target snapshot/version
timing model
resource reservations
operation timings
constraints
algorithm
configuration
verification status

Deserialization must validate before creating an executable schedule.

Never deserialize untrusted schedule data directly into execution.

---

59. Diagnostics

Large schedules must be explainable.

Diagnostics should answer:

Why was operation X delayed?

Possible answer:

Operation X could execute at T=100,
but resource R7 was occupied until T=140.

Also:

Operation X waited for predecessor Y.
Operation X was aligned to boundary 16.
Operation X missed resource R3 and used R4.
Operation X was delayed by communication latency.

This is essential for production debugging.

---

60. Memory scalability

Avoid:

one object for every possible time slot

and:

dense resource × time matrices

Use sparse structures.

A schedule containing:

1,000,000 operations

should allocate structures proportional primarily to:

operations
dependencies
resources actually used
reservations actually made

rather than:

maximum possible time × maximum possible resources

---

61. Integer sizing

Semantic identities should not depend on host pointer width.

Use stable integer identity types where appropriate.

Time coordinates may require a wider representation than collection indices.

Never assume:

usize == semantic size

for externally meaningful identities.

Collection indices may use "usize" internally where appropriate, but they must not leak as semantic quantum identities.

---

62. Overflow handling

Every arithmetic operation involving:

time
duration
capacity
cost
counts
indices

must be evaluated for overflow where overflow could affect correctness.

Do not silently wrap.

Return:

SchedulingError::Overflow

or an equivalent structured error.

---

63. Cancellation

Large scheduling jobs must support cancellation.

The scheduling context should provide an optional cancellation mechanism.

Algorithms must periodically observe it at safe boundaries.

Cancellation must produce:

Cancelled

rather than returning a partial schedule as if it were valid.

A partial schedule may be returned only through an explicitly defined analysis/debug result type.

---

64. Deadlines

A scheduler deadline is different from an execution deadline.

The scheduler must distinguish:

compilation deadline

from:

schedule execution deadline

Do not conflate them.

---

65. Incremental scheduling

Production scheduling should eventually support:

existing schedule
+
new operations
+
changed resources
+
changed availability

without rebuilding everything unnecessarily.

However, incremental scheduling must preserve dependency and resource invariants.

The architecture should therefore expose an epoch model:

SchedulerSessionId
      ↓
EpochId
      ↓
ScheduleId

---

66. Calibration snapshots

A schedule depends on timing/calibration information.

Therefore the schedule provenance must identify the calibration snapshot used.

If calibration changes, the old schedule must not silently be treated as equivalent.

Pipeline:

hardware
   ↓
calibration snapshot
   ↓
SchedulingContext
   ↓
schedule

---

67. Target snapshots

The same principle applies to hardware capabilities.

A schedule must identify the target description used.

This makes:

reproducibility
debugging
regression testing

possible.

---

68. Plugin architecture

Custom schedulers must be plugins.

Examples:

vendor scheduler
research scheduler
ML scheduler
heuristic scheduler
exact optimizer
custom distributed scheduler

Plugins must consume the stable scheduler contracts.

They must not mutate canonical IR unexpectedly.

---

69. Plugin isolation

A plugin must not be allowed to:

change QubitId meaning
change canonical operation semantics
modify global scheduler state
bypass verification
bypass target capability checks

unless an explicit privileged integration boundary exists.

---

70. Algorithm selection

The planner registry may expose:

asap
alap
list
critical_path
resource_constrained
adaptive
custom

Algorithm selection must happen through configuration.

Do not hard-code:

if operations > 1000 { use_list(); }

unless such thresholds are explicit configuration parameters.

---

71. Performance architecture

A production scheduler should separate:

analysis
planning
reservation
transformation
verification

This enables profiling.

A result should be able to report:

analysis_time
planning_time
reservation_time
verification_time
transformation_time

without changing algorithm semantics.

---

72. Scalability classes

The test suite should define workload classes based on actual supplied resource limits, not scheduler constants.

Examples:

tiny
small
medium
large
very_large
distributed
stress

The exact sizes belong to test configuration.

They must not become production scheduler limits.

---

73. Complexity requirements

Baseline graph analysis should target:

O(V + E)

where feasible.

Resource scheduling complexity depends on:

resource count
resource capacity
dependency density
objective
constraints
algorithm

Algorithms must document expected complexity.

No algorithm may claim universal optimality.

---

74. Avoid recursion

Arbitrary user programs can create extremely deep dependency graphs.

Therefore production implementations must prefer iterative algorithms for:

DFS
BFS
topological sorting
critical-path analysis
graph validation
resource traversal

Recursion may be used only when the input depth is explicitly bounded by a validated caller limit.

---

75. No hidden global state

Forbidden:

static mut ...

and mutable singleton schedulers.

Also forbidden:

global hardware state
global calibration state
global target state
global random generator

Each compilation must receive explicit state through its context.

---

76. No unsafe Rust

Every scheduler source file must contain:

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

No:

unsafe

blocks.

No unsafe FFI inside scheduling.

Vendor FFI belongs behind hardware/provider boundaries and must not contaminate the scheduler core.

---

77. Rust 1.97.1 compatibility

The scheduler must compile on:

rustc 1.97.1

and remain compatible with Rust 1.97 where practical.

Rust 1.97.1 should be the CI baseline because it fixes an LLVM miscompilation issue from the 1.97 line.

Do not require:

nightly
unstable features
edition 2024-only APIs

for the scheduler.

---

78. Cargo/build policy

The scheduling subsystem must not require unsafe dependencies.

New dependencies must be justified against:

license
maintenance
MSRV
unsafe usage
compile-time cost
runtime cost
determinism
portability
security

Prefer standard-library implementations for foundational scheduler data structures where performance is sufficient.

---

79. Concurrency safety

Scheduler context objects should be immutable where possible.

Prefer:

owned immutable snapshots

over:

shared mutable state

Parallel algorithms should operate on independent analysis data and synchronize only at explicit arbitration/reservation boundaries.

---

80. Semantic immutability

Scheduling must not mutate the source quantum program merely to compute a schedule.

Use:

canonical IR
      ↓
read-only adapter
      ↓
scheduling representation

Transformations create explicit derived representations.

---

81. Provenance

Every scheduled operation should retain enough provenance to answer:

Which canonical operation produced this schedule entry?

For transformed operations:

Which transformation produced it?

For inserted delays:

Why was it inserted?

This is essential for debugging and verification.

---

82. Schedule identity

A "ScheduleId" must identify the produced schedule.

It must not be confused with:

ProgramId
OperationId
QubitId
PhysicalQubitId
ResourceId

One program can have many valid schedules.

---

83. Reproducibility

A reproducible schedule requires:

program identity
target identity/version
target snapshot
calibration snapshot
scheduler version
algorithm
configuration
seed

A serialized schedule should preserve this provenance.

---

84. Compatibility with explicit timing languages

The scheduler must be capable of consuming timing intent such as:

delay
barrier
timed block
stretch
earliest execution
latest execution
alignment

without assuming one physical timing model.

OpenQASM 3.1 explicitly defines timing constructs and delays, including timing-independent design intent.

---

85. Hardware lowering boundary

The final schedule is not necessarily the final hardware program.

Pipeline:

scheduled logical/physical operation
       ↓
hardware lowering
       ↓
native instruction representation
       ↓
pulse/control representation where required
       ↓
runtime execution

Scheduling should stop at its defined ownership boundary.

---

86. Simulation

A simulator can consume the schedule through an adapter.

Do not create simulator-specific scheduling branches.

The simulator should provide a target description just like hardware:

simulator target
      ↓
SchedulingContext
      ↓
generic scheduler

This provides realistic scheduling tests without vendor hardware.

---

87. Emulator

Emulators similarly expose:

capabilities
timing
resources
availability
constraints

rather than bypassing the scheduler.

---

88. Distributed target abstraction

A distributed target must appear to scheduling as:

resources
nodes
links
latencies
capacities
communication constraints

not as a special scheduler algorithm.

The same planner can then reason about local and remote operations.

---

89. Communication scheduling

Communication itself consumes resources.

Examples:

entanglement generation
teleportation
classical feed-forward
network link
switch
memory

These become reservations.

A remote quantum operation cannot be scheduled before its communication prerequisites are complete.

---

90. Resource hierarchy

Resources may be hierarchical:

system
 ├── module
 │    ├── chip
 │    │    ├── qubit
 │    │    └── channel
 │    └── controller
 └── network
      └── link

The resource model must support hierarchical constraints without assuming a particular hierarchy depth.

---

91. Capacity semantics

Capacity is not always binary.

A resource may permit:

capacity = 1
capacity = N
capacity = weighted capacity
capacity = time-varying capacity

Therefore resource usage must not always be represented as:

busy/free

A generic usage quantity is required.

---

92. Operation arity

The scheduler must not assume:

one-qubit
two-qubit

as the complete operation model.

Operations may involve:

1
2
3
N

operands, subject to target capability.

The scheduler must consume the operation's actual operand set.

---

93. Qubit lifetime

Scheduling must support:

allocated
active
idle
measured
reset
released

where the target model requires it.

A measured qubit may have different subsequent constraints from an untouched qubit.

---

94. Measurement semantics

Measurement may introduce:

quantum completion
readout resource usage
classical result availability
feedback latency
reset requirements

All must be represented explicitly.

Measurement must not be treated as merely another gate.

---

95. Reset semantics

Reset may require:

exclusive qubit access
control resource
duration
cooldown
measurement dependency

These are target-supplied constraints.

---

96. Classical feedback

A classical result may become available at:

measurement_end + classical_latency

The scheduler must model this as a dependency.

It must not assume:

classical latency = 0

---

97. Barriers

A barrier must be represented as an explicit scheduling constraint.

Do not implement a barrier as a vendor-specific delay.

---

98. Resource conflicts

Two operations conflict if:

same exclusive resource

or:

combined usage > capacity

or:

custom constraint rejects simultaneous execution

The third category is important for crosstalk and hardware-specific restrictions.

---

99. Custom constraints

"constraints/custom.rs" must allow targets/plugins to define constraints without modifying the scheduler core.

Examples:

operation A cannot overlap operation B
resource group X cannot exceed usage Y
operations of class C require guard time

Custom constraints must be:

deterministic where required
explainable
verifiable
serializable when persisted

---

100. Guard times

Some operations require spacing:

operation
   ↓
guard interval
   ↓
next operation

Guard intervals belong in timing/resource constraints, not hard-coded planner logic.

---

101. Idle-time representation

Idle time is meaningful.

It may represent:

decoherence exposure
cooldown
synchronization
communication waiting
resource contention
intentional delay

Therefore the scheduler result must preserve idle intervals when required.

---

102. Fidelity objective

If ZQN exposes idle/error information, idle time can become an optimization objective.

For example:

minimize makespan
subject to fidelity threshold

or:

maximize estimated fidelity
subject to deadline

The objective system must support such constraints without embedding one universal weighting.

---

103. Multi-objective optimization

Support:

lexicographic
weighted
Pareto
constraint-first

where appropriate.

The selected strategy must be explicit.

---

104. Verification of transformations

Every transformation must declare:

input assumptions
output guarantees
semantic-preservation claim
resource effects
timing effects

After transformation:

verification

must be rerun.

---

105. Plugin determinism

A plugin must declare whether it is:

deterministic
seeded stochastic
nondeterministic

A deterministic compilation cannot silently invoke an uncontrolled nondeterministic plugin.

---

106. Diagnostics and observability

Diagnostics must be optional.

Normal production compilation must not allocate enormous debug traces unless requested.

Use explicit levels:

off
errors
summary
detailed
trace
profile

---

107. Large-scale logging

Do not log every operation by default.

For million/billion-operation workloads, unrestricted per-operation logging can dominate compilation cost.

Use:

aggregated counters
sampled traces
bounded diagnostics
explicit full tracing

---

108. Benchmark integration

Scheduling metrics must integrate with the repository's benchmarking subsystem.

Useful metrics:

operation count
dependency count
resource count
schedule depth
makespan
parallelism
resource utilization
idle time
planning time
verification time
memory
communication overhead

The benchmark system consumes these metrics.

Scheduling does not own benchmarking protocols.

---

109. Test architecture

Required:

tests/
├── unit/
├── integration/
├── property/
├── regression/
├── scalability/
├── determinism/
└── fixtures/

---

110. Unit tests

Every foundational type and invariant must have unit tests.

Examples:

TimePoint checked addition
Duration checked subtraction
identity ordering
resource reservation
interval overlap
dependency creation
cycle detection
alignment validation

---

111. Property tests

Important properties:

No exclusive resource overlaps.
Every dependency is respected.
No negative schedule times.
No duration overflow is silently accepted.
Canonical qubit identity is preserved.
Scheduling does not create qubits.
Scheduling does not change operation operands.
Deterministic input produces deterministic schedule.

---

112. Regression tests

Every discovered scheduling defect becomes a permanent regression test.

Regression fixtures must preserve:

input
target
configuration
expected invariant

They should avoid brittle full-output comparisons where multiple equally valid schedules exist.

---

113. Determinism tests

Run the same:

program
target
configuration
seed

multiple times.

Compare canonicalized schedules.

The test must fail if scheduling decisions differ under deterministic configuration.

---

114. Scalability tests

Scalability tests must grow the workload rather than change production constants.

Test dimensions:

operation count
qubit count
dependency density
resource count
resource pressure
graph depth
graph width
QEC rounds
distributed nodes
communication edges

---

115. Stress testing

Stress tests must include:

very wide DAG
very deep DAG
high dependency density
high resource contention
many resources
many parallel operations
long scheduling horizon
large QEC workload
distributed communication

The scheduler must fail gracefully when explicit resource limits are exceeded.

---

116. Out-of-memory behavior

The scheduler cannot guarantee recovery from host-wide OOM.

However, it must avoid artificial allocations.

Where practical:

reserve
checked allocation planning
explicit limits
streaming/partitioning

should be used.

---

117. Partitioning

Very large workloads may be partitioned.

Possible partition dimensions:

dependency regions
physical modules
resource domains
QEC regions
distributed nodes

But partitioning must preserve cross-partition dependencies.

---

118. Hierarchical scheduling

Large machines may use:

global scheduling
      ↓
module scheduling
      ↓
chip scheduling
      ↓
local resource scheduling

The interfaces must remain generic.

This enables scaling without requiring one scheduler object to hold every low-level detail simultaneously.

---

119. Hierarchical result composition

Hierarchical schedules must preserve:

global ordering
local ordering
cross-level dependencies
resource reservations
provenance

A local schedule cannot violate a global reservation.

---

120. Streaming

For workloads too large to materialize fully, future scheduler versions may support streaming.

Streaming must preserve:

dependency correctness
resource correctness
determinism where requested

It must not silently discard provenance.

---

121. Incremental graph construction

The graph should eventually support adding operations incrementally.

Each mutation must validate:

operation identity
dependency endpoints
cycle constraints
resource references

A failed mutation must not leave the graph partially modified.

---

122. Dynamic graph model

Static DAG scheduling and dynamic execution must remain distinct.

Static:

A → B → C

Dynamic:

A
 ↓
measurement
 ↓
condition
 ├── B
 └── C

The scheduler must preserve branch semantics.

---

123. Branch convergence

When branches reconverge:

      B
     ↙
A → M
     ↘
      C
       ↓
       D

the scheduler must understand the runtime synchronization semantics.

It must not simply assume:

B and C both execute

unless the program semantics guarantee that.

---

124. Runtime branch resource planning

A conditional branch may require resources that are not simultaneously used.

The resource model should support conditional/resource-path semantics where necessary.

This prevents over-reserving resources for mutually exclusive runtime branches.

---

125. Security

The scheduler must treat imported schedules, plugins and serialized configurations as untrusted where applicable.

Never execute arbitrary code merely because schedule metadata contains a plugin name.

Plugin loading must go through an explicit trusted registry.

---

126. Serialization security

Deserialization must validate:

IDs
counts
durations
resource references
dependency references
numeric ranges
version
schema

before constructing an executable scheduling artifact.

---

127. Schema evolution

Serialized schedule formats must be versioned.

A future scheduler must be able to distinguish:

schema version
scheduler version
target version

Do not assume these are the same thing.

---

128. API stability

Stable public APIs should be exposed from:

scheduling::mod
scheduling::types
scheduling::context
scheduling::config
scheduling::result
scheduling::planners

Internal implementation details should not be unnecessarily re-exported.

---

129. "mod.rs"

"mod.rs" is the composition root.

It must contain:

module declarations
stable public exports
documentation

It must not contain:

algorithm implementation
hardware discovery
resource scheduling
timing algorithms
QEC algorithms

The repository's current scheduling IR follows this composition-root philosophy.

---

130. Stabilizer compatibility

"stabilizer_scheduler.rs" remains a compatibility boundary.

It must not implement:

ASAP
ALAP
list scheduling
RCPSP
critical-path scheduling
resource allocation
hardware routing

Its proper role is:

legacy QEC configuration
       ↓
QEC scheduling request
       ↓
generic scheduler

The current repository file explicitly documents this architectural migration.

---

131. File completion rule

Every scheduling file must be treated as a contract.

Before declaring a file complete, it must define:

Purpose
Ownership
Public API
Inputs
Outputs
Invariants
Errors
Dependencies
Thread-safety
Determinism
Serialization implications
Scalability implications
Integration boundary
Testing requirements
No-unsafe guarantee

A later file must not force a completed lower-level file to be redesigned merely because its contract was incomplete.

If a new requirement genuinely changes ownership, that is an architectural change and must be versioned/reviewed rather than patched opportunistically.

---

132. Required implementation order

The implementation order is:

Phase 1 — independent foundations

types.rs
errors.rs
limits.rs

Phase 2 — timing

timing/duration.rs
timing/time.rs
timing/resolution.rs
timing/alignment.rs
timing/windows.rs
timing/constraints.rs

Phase 3 — resources

resources/resource.rs
resources/pool.rs
resources/reservation.rs
resources/calendar.rs
resources/availability.rs

Phase 4 — scheduler IR

ir/operation.rs
ir/dependency.rs
ir/graph.rs
ir/critical_path.rs

Phase 5 — constraints

constraints/constraint.rs
constraints/qubit.rs
constraints/channel.rs
constraints/measurement.rs
constraints/reset.rs
constraints/control.rs
constraints/communication.rs
constraints/custom.rs

Phase 6 — composition

context.rs
config.rs
result.rs

Phase 7 — policies

policies/policy.rs
policies/asap.rs
policies/alap.rs
policies/priority.rs
policies/resource_aware.rs
policies/hybrid.rs

Phase 8 — planners

planners/planner.rs
planners/list.rs
planners/critical_path.rs
planners/resource_constrained.rs
planners/event.rs

Phase 9 — algorithms

algorithms/asap.rs
algorithms/alap.rs
algorithms/list.rs
algorithms/cp.rs
algorithms/rcpsp.rs
algorithms/adaptive.rs

Phase 10 — transformations

transformations/delays.rs
transformations/alignment.rs
transformations/padding.rs
transformations/dynamical_decoupling.rs

Phase 11 — verification

verification/structural.rs
verification/dependency.rs
verification/resource.rs
verification/timing.rs
verification/semantic.rs
verification/verifier.rs

Phase 12 — optimization

optimization/makespan.rs
optimization/depth.rs
optimization/idle_time.rs
optimization/fidelity.rs
optimization/energy.rs
optimization/multi_objective.rs

Phase 13 — QEC

qec/interface.rs
qec/syndrome.rs
qec/rounds.rs
qec/stabilizer.rs

Phase 14 — dynamic execution

dynamic/classical.rs
dynamic/conditional.rs
dynamic/feedback.rs
dynamic/runtime.rs

Phase 15 — distributed

distributed/node.rs
distributed/link.rs
distributed/communication.rs
distributed/network.rs

Phase 16 — adapters

adapters/ir.rs
adapters/hardware.rs
adapters/routing.rs
adapters/qec.rs

Phase 17 — persistence/observability

serialization/schema.rs
serialization/encode.rs
serialization/decode.rs
diagnostics/trace.rs
diagnostics/explain.rs
diagnostics/profile.rs

Phase 18 — plugins

plugins/scheduler.rs
plugins/registry.rs

Phase 19 — compatibility

stabilizer_scheduler.rs

Phase 20 — composition root

mod.rs

Phase 21 — test completion

unit
integration
property
regression
determinism
scalability

---

133. Integration contract with canonical IR

Required:

quantum::ir
     │
     ▼
scheduling::adapters::ir
     │
     ├── preserve OperationId
     ├── preserve QubitId
     ├── preserve PhysicalQubitId
     ├── preserve semantics
     └── derive scheduling metadata
     │
     ▼
scheduling::ir

The adapter is the only layer that should understand detailed canonical IR layout.

This isolates future IR evolution.

---

134. Integration contract with routing

quantum::routing
      │
      ▼
mapped operations
      │
      ▼
scheduling::adapters::routing
      │
      ▼
scheduling::ir

Routing owns:

logical → physical

Scheduling owns:

physical operation → time

---

135. Integration contract with hardware

quantum::hardware
      │
      ▼
HardwareCapabilities
      │
      ▼
adapters::hardware
      │
      ├── timing
      ├── resources
      ├── availability
      ├── alignment
      └── constraints
      │
      ▼
SchedulingContext

No hardware SDK calls inside scheduling algorithms.

---

136. Integration contract with ZQN

quantum::zqn
      │
      ▼
noise/error information
      │
      ▼
scheduler adapter/objective
      │
      ▼
fidelity-aware scheduling

ZQN remains the source of truth for noise.

---

137. Integration contract with QEC

quantum::error_correction
      │
      ▼
QEC execution requirements
      │
      ▼
scheduling::qec
      │
      ▼
scheduling::adapters::qec
      │
      ▼
generic scheduler

QEC must never create a separate scheduler architecture.

---

138. Integration contract with benchmarking

scheduler
   │
   ▼
ScheduleResult
   │
   ├── makespan
   ├── depth
   ├── idle
   ├── utilization
   ├── planning time
   └── verification time
   │
   ▼
benchmarking

Benchmarking observes scheduling.

Scheduling does not own benchmarking.

---

139. Integration contract with runtime

ScheduleResult
      │
      ▼
hardware lowering
      │
      ▼
runtime
      │
      ▼
execution

The runtime must reject schedules that fail final target validation.

---

140. Integration contract with simulator

simulator target
      │
      ▼
SchedulingContext
      │
      ▼
generic scheduler
      │
      ▼
scheduled simulator program

This allows simulator and hardware schedules to use the same architecture.

---

141. Final invariants

A production scheduler MUST satisfy all of these:

1. No artificial machine-size ceiling.
2. No hard-coded qubit count.
3. No hard-coded topology.
4. No hard-coded resource count.
5. No hard-coded timing.
6. No vendor-specific planner logic.
7. No duplicate "QubitId".
8. No duplicate "PhysicalQubitId".
9. No duplicate quantum semantic IR.
10. No unsafe Rust.
11. No global mutable scheduler state.
12. Checked arithmetic.
13. Explicit resource constraints.
14. Explicit timing constraints.
15. Explicit dependency constraints.
16. Static and dynamic execution are distinguished.
17. Distributed execution is representable.
18. QEC is integrated without coupling generic scheduling to one code.
19. Routing remains separate.
20. Hardware remains separate.
21. ZQN remains separate.
22. Verification is mandatory.
23. Deterministic mode is reproducible.
24. Algorithms are replaceable.
25. Objectives are configurable.
26. Serialization is versioned.
27. Diagnostics are explainable.
28. Large graphs use scalable sparse representations.
29. Large scheduling horizons do not require dense time-slot matrices.
30. Recursive algorithms are avoided for unbounded graph depth.
31. Explicit limits are caller-controlled.
32. Resource availability is target-supplied.
33. Calibration snapshots are represented.
34. Target snapshots are represented.
35. Schedule provenance is preserved.
36. Transformations are re-verified.
37. Plugins cannot silently alter quantum semantics.
38. Partial schedules are never reported as successful schedules.
39. Cancellation is explicit.
40. Failure is structured and diagnosable.

---

142. Production definition

"src/quantum/scheduling/" may be declared production ready only when:

FOUNDATIONS
[ ] types
[ ] errors
[ ] limits
[ ] config
[ ] context
[ ] result

IR
[ ] operation
[ ] dependency
[ ] graph
[ ] critical path

TIMING
[ ] duration
[ ] time
[ ] resolution
[ ] alignment
[ ] windows
[ ] constraints

RESOURCES
[ ] resource
[ ] pool
[ ] reservation
[ ] calendar
[ ] availability

CONSTRAINTS
[ ] qubit
[ ] channel
[ ] measurement
[ ] reset
[ ] control
[ ] communication
[ ] custom

POLICIES
[ ] ASAP
[ ] ALAP
[ ] priority
[ ] resource-aware
[ ] hybrid

PLANNERS
[ ] list
[ ] critical path
[ ] resource constrained
[ ] event-driven

ALGORITHMS
[ ] ASAP
[ ] ALAP
[ ] list
[ ] CP
[ ] RCPSP
[ ] adaptive

TRANSFORMATIONS
[ ] delays
[ ] alignment
[ ] padding
[ ] DD

VERIFICATION
[ ] structural
[ ] dependency
[ ] resource
[ ] timing
[ ] semantic
[ ] final verifier

OPTIMIZATION
[ ] makespan
[ ] depth
[ ] idle time
[ ] fidelity
[ ] energy
[ ] multi-objective

DYNAMIC
[ ] classical
[ ] conditional
[ ] feedback
[ ] runtime

QEC
[ ] interface
[ ] syndrome
[ ] rounds
[ ] stabilizer

DISTRIBUTED
[ ] node
[ ] link
[ ] communication
[ ] network

ADAPTERS
[ ] IR
[ ] hardware
[ ] routing
[ ] QEC

PERSISTENCE
[ ] schema
[ ] encode
[ ] decode

DIAGNOSTICS
[ ] trace
[ ] explain
[ ] profile

PLUGINS
[ ] scheduler
[ ] registry

TESTING
[ ] unit
[ ] integration
[ ] property
[ ] regression
[ ] determinism
[ ] scalability
[ ] stress

INTEGRATION
[ ] quantum::ir
[ ] quantum::routing
[ ] quantum::hardware
[ ] quantum::zqn
[ ] quantum::error_correction
[ ] quantum::benchmarking
[ ] runtime
[ ] simulator

---

143. Final architecture

The production architecture is:

                         ZAMANI PROGRAM
                               │
                               ▼
                         quantum::frontend
                               │
                               ▼
                         canonical quantum IR
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
                               ▼
                    scheduling internal IR
                               │
              ┌────────────────┼─────────────────┐
              │                │                 │
              ▼                ▼                 ▼
         dependencies       resources          timing
              │                │                 │
              └────────────────┼─────────────────┘
                               ▼
                         constraints
                               │
                               ▼
                            policy
                               │
                               ▼
                           planner
                               │
                ┌──────────────┼──────────────┐
                ▼              ▼              ▼
              ASAP            ALAP          adaptive
                │              │              │
                └──────────────┼──────────────┘
                               ▼
                        resource-aware
                           scheduling
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
                       ScheduleResult
                               │
              ┌────────────────┼─────────────────┐
              ▼                ▼                 ▼
           hardware         simulator        emulator
              │                │                 │
              └────────────────┼─────────────────┘
                               ▼
                            runtime

The fundamental abstraction remains:

                         PROGRAM
                            │
                            ▼
                         TARGET
                            │
                            ▼
                          POLICY
                            │
                            ▼
                        SCHEDULER
                            │
                            ▼
                         SCHEDULE

not:

program + fixed machine size + fixed timing constants

---

144. The ultimate scalability guarantee

The scheduler guarantees:

«No artificial scheduler-defined finite machine size.»

It does not guarantee:

«Infinite physical resources.»

The distinction is essential.

For a target with:

2 qubits

the scheduler uses the 2-qubit target description.

For:

100 qubits

it uses the 100-qubit target description.

For:

1,000,000 qubits

it uses the million-qubit target description.

For:

distributed QPUs

it uses the distributed resource/network description.

The source program remains:

THE SAME PROGRAM

The scheduler changes only because:

TARGET CAPABILITIES
+
RESOURCES
+
TIMING
+
CONSTRAINTS
+
POLICY

changed.

That is the correct meaning of write once, scale from atom to everywhere.

---

145. Non-negotiable architectural rule

When adding any future scheduling feature, ask these questions before adding code:

1. Is this quantum semantics?
   → quantum::ir

2. Is this logical-to-physical mapping?
   → quantum::routing

3. Is this hardware capability?
   → quantum::hardware

4. Is this noise/error information?
   → quantum::zqn

5. Is this QEC semantics?
   → quantum::error_correction

6. Is this "when can it execute?"
   → quantum::scheduling

7. Is this execution?
   → runtime

8. Is this benchmarking?
   → benchmarking

If the answer is scheduling, determine whether the feature belongs to:

timing
resources
constraints
policy
planner
algorithm
transformation
verification
optimization
dynamic
distributed
QEC integration
adapter
diagnostics

before creating code.

Never put unrelated functionality into "mod.rs".

Never make a scheduler algorithm responsible for a subsystem that already has an owner.

---

146. Final engineering principle

The scheduler is not a collection of gate-specific timing hacks.

It is a general resource-constrained temporal execution framework for quantum programs.

Its abstraction is:

WHAT
 │
 ▼
canonical quantum IR
 │
 ▼
WHERE
 │
 ▼
routing
 │
 ▼
WHEN
 │
 ▼
scheduling
 │
 ▼
CAN IT ACTUALLY RUN?
 │
 ▼
hardware validation
 │
 ▼
EXECUTE
 │
 ▼
runtime

Therefore:

same Zamani program
        +
different target description
        +
different available resources
        +
different timing model
        +
different constraints
        +
different scheduling policy
        =
different valid target schedule

without modifying the semantic Zamani program.

That is the scalability contract that "src/quantum/scheduling/" must enforce.