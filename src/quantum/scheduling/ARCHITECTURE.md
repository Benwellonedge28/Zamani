Worked for 8s

Yes. I reviewed the current Zamani repository structure, including the quantum root, canonical IR, routing architecture, and the scheduling files already present, and cross-checked the design against current compiler architecture documented by IBM/Qiskit/OpenQASM and Quantinuum/tket. The repository already has an important rule that this architecture must preserve: quantum::ir::qubit is the canonical owner of QubitId and PhysicalQubitId; scheduling must not create competing qubit identities.

The repository also already contains substantial scheduling scaffolding—ir, adapters, dynamic scheduling, constraints, planners, verification, and serialization—so the correct ARCHITECTURE.md should describe the actual target architecture and integration contracts, rather than proposing a second unrelated tree.

Below is the complete document.

Zamani Quantum Scheduling Architecture

Path: "src/quantum/scheduling/ARCHITECTURE.md"
Status: Production Architecture Specification
Target Rust: Rust 1.97 / Rust 1.97.1
Edition: Rust 2021
Safety: "unsafe" forbidden
Scope: Universal quantum scheduling from the smallest executable quantum system to arbitrarily large systems constrained only by actual available resources and explicit execution policies.

---

1. Purpose

"quantum::scheduling" is the authoritative scheduling subsystem of the Zamani quantum compiler.

Its responsibility is:

«Determine when executable quantum, classical, control, communication, QEC, and resource operations may occur while preserving program semantics and satisfying the capabilities, resources, timing rules, dependencies, and constraints of the selected execution target.»

Scheduling does not determine what the program means.

Scheduling does not determine where logical qubits are located.

Scheduling does not communicate directly with hardware.

Scheduling does not own quantum error-correction semantics.

Scheduling does not own canonical quantum identities.

The architectural separation is:

Zamani program
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
scheduling
      │
      ▼
hardware lowering
      │
      ▼
runtime
      │
      ▼
execution target

The core questions are:

IR          = WHAT does the computation mean?
optimization= WHAT equivalent representation is preferable?
routing     = WHERE can operations execute?
scheduling  = WHEN can operations execute?
hardware    = WHAT can this target actually execute?
runtime     = HOW is execution performed?

This separation is mandatory.

---

2. Architectural objective

The scheduler must support the Zamani principle:

«Write a quantum program once and specialize it for any compatible quantum execution target without embedding target size or topology into the program.»

The same program must be schedulable for:

one qubit
      │
few qubits
      │
single QPU
      │
large QPU
      │
multi-chip system
      │
multi-QPU system
      │
distributed quantum computer
      │
quantum network
      │
future quantum architectures

No scheduler implementation may encode a finite architectural machine-size limit.

"Infinite scalability" means:

«The scheduler introduces no artificial finite machine-size ceiling. Actual executions remain bounded by the resources, memory, address space, time, target capabilities, and explicit policies available to a particular compilation or execution.»

Therefore this architecture must never contain semantic constants such as:

const MAX_QUBITS: usize = ...;
const MAX_OPERATIONS: usize = ...;
const MAX_ROUNDS: usize = ...;
const MAX_CHANNELS: usize = ...;
const MAX_DEPTH: usize = ...;

If a limit is necessary, it must be supplied explicitly as:

caller policy
target capability
resource availability
security policy
execution budget
memory budget
deadline

and must never become part of Zamani's semantic definition.

---

3. Current repository relationship

The repository already establishes the canonical quantum architecture:

quantum::frontend
        │
        ▼
quantum::ir
        │
        ├── optimization
        ├── routing
        ├── scheduling
        └── analysis
                │
                ▼
             hardware
                │
                ▼
             runtime

The quantum root explicitly identifies scheduling as the owner of ordering and timing, while hardware timing capabilities are supplied through the hardware boundary.

The canonical IR likewise explicitly states that it does not decide:

- physical machine selection;
- physical qubit selection;
- routing;
- scheduling;
- hardware-native instruction selection;
- calibration;
- execution.

Those are downstream responsibilities.

The scheduling architecture must therefore integrate with those existing contracts instead of recreating them.

---

4. Canonical qubit identity

This is a non-negotiable rule.

Scheduling MUST use:

crate::quantum::ir::qubit::QubitId
crate::quantum::ir::qubit::PhysicalQubitId

where required.

Scheduling MUST NOT define:

struct QubitId(...);
struct PhysicalQubitId(...);

or equivalent replacements.

There must be exactly one authoritative implementation of quantum qubit identity.

The canonical IR explicitly establishes "quantum::ir::qubit" as the authoritative logical/physical qubit identity boundary.

The scheduling subsystem may define scheduler-owned identifiers such as:

ScheduleId
ReservationId
DependencyId
ResourceId
ConstraintId

but those are not replacements for quantum qubit identity.

---

5. Scheduling must not own logical-to-physical mapping

Routing answers:

logical qubit
      ↓
physical qubit

Scheduling answers:

mapped operation
      ↓
execution time

Therefore:

logical program
      │
      ▼
routing
      │
      ▼
mapped executable representation
      │
      ▼
scheduling

The routing subsystem already establishes itself as the owner of logical-to-physical placement and connectivity-aware transformation.

Do not duplicate mapping logic in scheduling.

---

6. Scheduling must not own hardware discovery

The scheduler must never:

connect to QPU
discover device
authenticate provider
query credentials
download calibration
open network socket
submit job

Instead:

quantum::hardware
       │
       ▼
target description
       │
       ▼
scheduling adapter
       │
       ▼
SchedulingContext

The scheduler consumes a snapshot or explicit target description.

This makes scheduling:

- deterministic;
- testable;
- offline-capable;
- provider-independent;
- simulator-independent;
- reusable.

---

7. Target-independent architecture

The scheduler must not assume a particular quantum technology.

It must be capable of consuming target descriptions for:

superconducting
trapped ion
neutral atom
photonic
spin
quantum dots
topological
bosonic
annealing
analog
measurement-based
modular
distributed
future architectures

The scheduling abstraction must therefore operate on:

operations
resources
capabilities
dependencies
timing
constraints
availability
communication
objectives

rather than on:

"IBM-style qubit"
"ion-trap qubit"
"superconducting pulse"

Those technology-specific details belong to target and hardware adapters.

---

8. Scheduling directory

The production scheduling subsystem is organized conceptually as:

src/quantum/scheduling/
│
├── ARCHITECTURE.md
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
├── algorithms/
│   ├── mod.rs
│   ├── asap.rs
│   ├── alap.rs
│   ├── list.rs
│   ├── cp.rs
│   ├── rcpsp.rs
│   └── adaptive.rs
│
├── plugins/
│   ├── mod.rs
│   ├── scheduler.rs
│   └── registry.rs
│
├── tests/
│   ├── unit/
│   ├── integration/
│   ├── property/
│   ├── regression/
│   ├── scalability/
│   ├── determinism/
│   └── fixtures/
│
└── stabilizer_scheduler.rs

The precise module list must follow the repository's actual files as they are created. No "mod.rs" should declare a nonexistent implementation.

---

9. Dependency order

The implementation order must minimize re-editing.

The preferred order is:

PHASE 1
types
errors
limits
timing primitives
resource primitives

PHASE 2
timing constraints
resource pools
resource calendars
reservations

PHASE 3
scheduler operation representation
dependencies
graph
critical path

PHASE 4
constraint contracts

PHASE 5
context
configuration
result

PHASE 6
policy contracts
ASAP
ALAP
priority
resource-aware policies

PHASE 7
planner contracts
list planner
critical-path planner
resource-constrained planner
event planner

PHASE 8
algorithm wrappers

PHASE 9
transformations

PHASE 10
verification

PHASE 11
optimization objectives

PHASE 12
dynamic scheduling

PHASE 13
QEC integration

PHASE 14
distributed scheduling

PHASE 15
IR/hardware/routing/QEC adapters

PHASE 16
serialization

PHASE 17
diagnostics

PHASE 18
plugins

PHASE 19
compatibility facade

PHASE 20
module composition

PHASE 21
integration and scalability tests

The reason "mod.rs" is intentionally late is to make it a composition root rather than a constantly changing architectural scratchpad.

---

10. "types.rs"

Ownership

Defines scheduler-specific foundational vocabulary.

It may define

ScheduleId
DependencyId
ReservationId
ConstraintId
ResourceRequirementId
Priority
Cost
Slack

where those concepts are scheduler-specific.

It must not define

QubitId
PhysicalQubitId
Gate
QuantumOperation
QuantumCircuit

Those belong to canonical IR.

Requirements

Types must:

- be strongly typed;
- support deterministic comparison where meaningful;
- support hashing where required;
- avoid raw "usize" as semantic identity;
- support serialization if public schedule persistence requires it;
- avoid machine-size assumptions.

Integration

Consumers:

ir
resources
constraints
result
serialization
diagnostics

No consumer should need to reinterpret these identifiers.

---

11. "errors.rs"

Owns the canonical scheduler error hierarchy.

Required categories include:

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

Errors must contain structured context where available:

operation_id
resource_id
constraint_id
time
cause

Error strings must never be used as machine-readable control flow.

---

12. "limits.rs"

Limits are policies, not architectural constants.

It must distinguish:

compiler limits
security limits
memory limits
execution limits
time limits
parallelism limits
resource limits

Examples:

maximum operations
maximum graph memory
maximum planning time
maximum schedule duration
maximum parallel workers
deadline
cancellation

All limits must be optional.

Absence of a configured artificial limit must not cause the scheduler to manufacture one.

Actual target capacity remains target-owned.

---

13. "context.rs"

"SchedulingContext" is the primary immutable input to scheduling.

Conceptually:

SchedulingContext
├── executable program representation
├── dependency information
├── target capabilities
├── resource model
├── timing model
├── availability snapshot
├── constraints
├── scheduling policy
├── optimization objectives
├── reproducibility information
├── explicit limits
└── cancellation/deadline context

The context must not own a hardware connection.

It must not perform device discovery.

It must represent the target state supplied by the caller.

This makes compilation deterministic against a target snapshot.

---

14. "config.rs"

Owns declarative scheduler configuration.

It must support configuration of:

policy
objective
determinism
seed
verification
optimization
parallelism
distributed mode
timing behavior
resource behavior
diagnostics
limits

Configuration must be caller-owned.

No global scheduler configuration.

No environment-variable-only hidden behavior.

No implicit random seed.

---

15. "result.rs"

The schedule result must be richer than a list of timestamps.

It must be capable of representing:

scheduled operations
start times
finish times
durations
resource reservations
makespan
depth
critical path
idle intervals
resource utilization
objective values
verification status
diagnostics
provenance
reproducibility metadata

A successful result must identify the target/context against which it was produced sufficiently for reproducibility and validation.

---

16. "ir/"

The scheduling IR is an execution-planning view of canonical quantum IR.

It is not a second quantum semantic IR.

This distinction is mandatory.

The repository already contains scheduling IR and an IR adapter.

---

17. "ir/operation.rs"

Represents a schedulable operation.

It must retain:

scheduler operation identity
canonical source operation identity
operands
resource requirements
duration
precedence
timing windows
conditions
metadata
semantic classification

Quantum operands must ultimately use the canonical IR identities:

crate::quantum::ir::qubit::QubitId
crate::quantum::ir::qubit::PhysicalQubitId

as appropriate.

It must not define a scheduler-specific quantum gate type.

---

18. "ir/dependency.rs"

Must represent more than ordinary qubit data dependencies.

Supported dependency categories should include:

quantum data dependency
classical data dependency
measurement dependency
control dependency
resource dependency
communication dependency
QEC dependency
explicit barrier dependency

Dependency edges must be explicit and inspectable.

---

19. "ir/graph.rs"

Owns the scheduling dependency graph.

Required capabilities:

add node
add edge
predecessors
successors
indegree
outdegree
ready set
topological traversal
cycle detection
deterministic traversal
incremental construction

Baseline dependency-analysis complexity should target:

O(V + E)

where practical.

Avoid recursion whose stack usage grows with program size.

---

20. "ir/critical_path.rs"

Computes derived timing information:

earliest start
earliest finish
latest start
latest finish
slack
critical path

This supports:

ASAP
ALAP
critical-path scheduling
priority policies
deadline reasoning

---

21. "resources/"

Quantum scheduling is a resource-constrained scheduling problem, not merely a dependency-DAG problem.

Resources may include:

logical qubits
physical qubits
control channels
measurement channels
readout resonators
drive channels
lasers
microwave resources
classical processors
memory
feedback channels
communication links
entanglement resources
ancillas
cryogenic/control resources
module resources
network resources

The resource abstraction must therefore be technology-neutral.

---

22. "resources/resource.rs"

Defines:

Resource
ResourceKind
ResourceCapacity
ResourceRequirement
ResourceMode

Supported resource semantics should include:

exclusive
shared
capacity-limited
consumable
reusable
hierarchical
time-dependent
conditional

No fixed number of resource instances.

---

23. "resources/pool.rs"

Represents a collection of interchangeable or related resources.

Examples:

measurement channels
control channels
compute units
communication links

A pool may have dynamic capacity.

The scheduler must query the model rather than assume capacity.

---

24. "resources/reservation.rs"

A reservation represents:

resource
operation
start
finish
usage mode
capacity consumed

Reservations must be independently verifiable.

---

25. "resources/calendar.rs"

Represents time-dependent availability.

It must support:

available
busy
reserved
maintenance
calibration
disabled
degraded
unknown

This is essential for real systems where resources are not continuously available.

---

26. "resources/availability.rs"

Provides target/resource availability information to scheduling.

It must not contact hardware itself.

Instead:

hardware/runtime/provider
        │
        ▼
availability snapshot
        │
        ▼
SchedulingContext

---

27. "timing/"

Timing is a first-class scheduling domain.

The timing subsystem must support:

physical durations
logical durations
symbolic durations
target-calibrated durations
time intervals
time windows
deadlines
release times
alignment
resolution

It must not assume one global clock.

---

28. "timing/duration.rs"

Must represent operation duration without assuming a specific hardware technology.

Potential states include:

known
symbolic
target-derived
interval
unknown

An operation with unknown duration must not silently receive a fake default duration.

It must either:

remain symbolic

or fail with a structured scheduling error if the selected scheduling policy requires concrete timing.

---

29. "timing/time.rs"

Defines checked time arithmetic.

Required concepts:

TimePoint
Duration
TimeInterval

Requirements:

- checked addition;
- checked subtraction;
- no negative duration;
- no silent overflow;
- explicit representation of time domain.

---

30. "timing/resolution.rs"

Represents target timing granularity.

Possible representations include:

continuous
integer ticks
sample periods
rational units
target-defined resolution

The scheduler must never hard-code:

1 ns
1 ps
dt = ...

The target supplies timing resolution.

---

31. "timing/alignment.rs"

Represents constraints such as:

operation alignment
channel alignment
measurement alignment
control alignment
frame alignment
target-specific alignment

Alignment is a constraint, not an algorithm.

---

32. "timing/windows.rs"

Supports:

release time
earliest start
latest start
earliest finish
latest finish
deadline
availability window

Windows must be composable.

---

33. "timing/constraints.rs"

Provides temporal constraint composition.

Examples:

A before B
A finishes before T
B starts within window
operation aligned to resolution
measurement available before classical use

---

34. "constraints/"

Constraints must be modular.

The generic constraint contract must permit:

check
explain
severity
priority
scope

A constraint must be able to explain why a candidate schedule is invalid.

---

35. "constraints/qubit.rs"

Handles resource occupancy of quantum qubits.

It must use:

quantum::ir::qubit::QubitId
quantum::ir::qubit::PhysicalQubitId

as appropriate.

It must not assume:

all gates are one- or two-qubit

Multi-qubit operations must be supported according to their operand set and target capabilities.

---

36. "constraints/channel.rs"

Handles:

control-channel conflicts
drive-channel conflicts
measurement-channel conflicts
shared electronics
capacity-limited control resources

Channel counts come from the target.

---

37. "constraints/measurement.rs"

Handles:

measurement duration
measurement resource conflicts
readout grouping
measurement ordering
classical-result availability
measurement alignment

---

38. "constraints/reset.rs"

Handles reset dependencies and resource occupancy.

Reset must not be treated as universally zero-duration.

---

39. "constraints/control.rs"

Handles:

classical conditions
branch readiness
feedback
condition-dependent operations

This is necessary for dynamic quantum programs.

---

40. "constraints/communication.rs"

Handles:

inter-module communication
quantum network resources
entanglement generation
teleportation
classical communication
synchronization
network latency

This is necessary for the eventual transition from single-QPU scheduling to distributed quantum computing.

---

41. "constraints/custom.rs"

Provides an explicit extension point for target-specific constraints.

A target-specific rule must be installable without modifying the generic scheduler.

Vendor-specific constraints must enter through this boundary or an appropriate hardware adapter.

---

42. "policies/"

Policies define what scheduling preference is desired.

Algorithms define how that preference is achieved.

Do not combine these concepts.

---

43. "policies/policy.rs"

Defines the common policy contract.

A policy should be able to express:

priority
objective
tie-breaking
resource preference
timing preference
determinism requirements

---

44. "policies/asap.rs"

ASAP:

«Start each operation as early as legally possible.»

ASAP must respect:

dependencies
resources
timing windows
alignment
conditions
communication
target constraints

ASAP must never simply assign:

time = previous operation end

because independent operations may execute concurrently.

---

45. "policies/alap.rs"

ALAP:

«Schedule operations as late as possible without violating the schedule's constraints or deadline.»

It depends on valid:

critical path
latest-start calculations
deadline information
resource constraints

---

46. "policies/priority.rs"

Priority policies may consider:

critical-path position
slack
deadline
resource pressure
fidelity
communication cost
QEC importance
measurement readiness
user-defined priority

Tie-breaking must be deterministic when deterministic mode is enabled.

---

47. "policies/resource_aware.rs"

This policy prioritizes scarce resources.

Examples:

rare control channel
limited readout resource
shared communication link
high-value ancilla

It must obtain resource scarcity from the target/context.

Never assume a fixed number of channels.

---

48. "policies/hybrid.rs"

Combines independent policy dimensions.

Examples:

ASAP + fidelity
ASAP + resource pressure
ALAP + deadline
critical path + communication

Weights must be configuration values.

No magic weights.

---

49. "planners/"

Planners perform scheduling.

The planner must receive a complete scheduling context and return a schedule result.

---

50. "planners/planner.rs"

Defines the central scheduling contract.

Conceptually:

Planner
    plan(context) -> Result<ScheduleResult, SchedulingError>

The planner must not:

parse source
discover hardware
authenticate
perform routing
perform QEC decoding

---

51. "planners/list.rs"

List scheduling is the primary general-purpose scheduler.

Conceptually:

dependency graph
      │
      ▼
ready operations
      │
      ▼
priority policy
      │
      ▼
resource feasibility
      │
      ▼
select operation
      │
      ▼
reserve resources
      │
      ▼
advance event time
      │
      └──────► repeat

The ready set must be maintained incrementally where practical.

Do not rescan the entire graph for every operation.

---

52. "planners/critical_path.rs"

Uses critical-path information to prioritize operations.

It should integrate with:

slack
deadline
resource pressure

without embedding target-specific behavior.

---

53. "planners/resource_constrained.rs"

Handles resource-constrained project scheduling.

This is necessary because:

dependency legality

does not guarantee:

resource feasibility

The repository already recognizes this distinction in the scheduling architecture.

---

54. "planners/event.rs"

Use event-driven scheduling rather than giant time-slot scans.

Events may include:

operation completion
resource release
measurement completion
classical result availability
communication completion
calibration boundary
resource state change

This is more scalable than allocating a timeline proportional to:

qubits × execution duration

---

55. "algorithms/"

Algorithm modules are implementations of scheduler strategies.

Required initial strategies:

ASAP
ALAP
list scheduling
critical-path scheduling
resource-constrained scheduling
adaptive scheduling

Algorithms must use the canonical planner and policy interfaces.

They must not create competing quantum IR types.

---

56. Adaptive scheduling

"adaptive.rs" may select a strategy according to:

graph structure
resource pressure
parallelism
deadline
communication density
target characteristics
QEC density

The choice must be deterministic when deterministic mode is requested.

Adaptive scheduling must preserve semantics.

---

57. Complexity and optimality

The architecture must distinguish:

exact
heuristic
approximate
deterministic
stochastic
adaptive

Scheduling problems with arbitrary resource constraints can be computationally difficult.

Therefore Zamani must not promise:

«globally optimal schedules for every possible target.»

Instead the result must expose measurable schedule quality:

makespan
depth
resource utilization
idle time
communication overhead
objective value

---

58. "transformations/"

Scheduling transformations modify the scheduled representation while preserving semantics.

---

59. "transformations/delays.rs"

Explicitly materializes idle intervals.

A delay is not merely an implementation artifact when timing itself has semantic or physical consequences.

The transformation must preserve:

operation order
dependencies
resource legality
timing semantics

---

60. "transformations/alignment.rs"

Converts ideal schedule timing into target-compatible aligned timing.

Examples:

rounding to target ticks
channel alignment
measurement alignment
pulse/frame alignment

Alignment must be supplied by the target.

---

61. "transformations/padding.rs"

Adds legal padding where required by:

synchronization
alignment
target constraints
QEC
protocol boundaries

---

62. "transformations/dynamical_decoupling.rs"

Dynamical decoupling is an optional scheduling transformation.

It must not be part of core scheduling semantics.

Its implementation must be:

optional
target-aware
policy-controlled
verification-aware

It must never insert a hard-coded pulse sequence universally.

---

63. "verification/"

Verification is mandatory.

A scheduler must never claim a schedule is production-valid merely because timestamps were generated.

---

64. "verification/structural.rs"

Verify:

every required operation represented
no duplicate operation
no missing operation
valid operation identities
valid schedule structure

---

65. "verification/dependency.rs"

For every dependency:

finish(predecessor) <= start(successor)

unless the dependency explicitly permits another relation.

Cycles must be rejected unless represented by an explicitly supported dynamic/runtime construct.

---

66. "verification/resource.rs"

For every resource:

usage <= capacity

at every relevant point in time.

Exclusive resources must never overlap.

---

67. "verification/timing.rs"

Verify:

duration
start
finish
alignment
resolution
release times
deadlines
windows

No silent overflow.

No negative duration.

---

68. "verification/semantic.rs"

This is one of the most important components.

The invariant is:

scheduled semantics == input semantics

Scheduling may change:

order where legal
time
parallelism
idle intervals
resource reservations

It must not accidentally change:

operands
operations
measurement semantics
classical conditions
control dependencies
program meaning

---

69. "verification/verifier.rs"

Aggregates:

structural verification
dependency verification
resource verification
timing verification
semantic verification

A successful production schedule should pass all enabled mandatory verification stages.

---

70. "optimization/"

Scheduling optimization operates on the schedule rather than changing the quantum program's meaning.

---

71. "optimization/makespan.rs"

Minimize:

total execution time

subject to all constraints.

---

72. "optimization/depth.rs"

Minimize scheduled depth.

Depth must be derived from actual scheduled operations, not from a hard-coded notion of circuit layers.

---

73. "optimization/idle_time.rs"

Minimize:

qubit idle time
resource idle time

This is especially relevant for noisy hardware.

---

74. "optimization/fidelity.rs"

Allows scheduling decisions to incorporate target-provided fidelity/error information.

The scheduler must not own the noise model.

Noise information enters through an adapter such as the future/current ZQN boundary.

---

75. "optimization/energy.rs"

Optional objective for targets where energy/resource consumption is meaningful.

No universal energy model may be hard-coded.

---

76. "optimization/multi_objective.rs"

Supports multiple objectives:

makespan
idle time
fidelity
resource cost
communication cost
energy

with caller-supplied objective definitions.

No magic weights.

---

77. Dynamic scheduling

Static DAG scheduling is insufficient for all future Zamani quantum programs.

The scheduler must support:

static dependencies
+
classical conditions
+
runtime events
+
feedback
+
dynamic allocation

The repository already has dedicated dynamic scheduling modules, including "classical", "conditional", and "feedback".

---

78. "dynamic/classical.rs"

Models readiness caused by classical computation.

Example:

measurement
      │
      ▼
classical calculation
      │
      ▼
condition available
      │
      ▼
quantum operation

---

79. "dynamic/conditional.rs"

Represents operations whose execution depends on classical state.

It must not assume every branch is statically executable.

---

80. "dynamic/feedback.rs"

Models:

measurement
→ classical processing
→ feedback
→ quantum operation

It must not own hardware calendars or QPU communication. The repository already identifies those as outside this module's responsibilities.

---

81. "dynamic/runtime.rs"

Represents portions of scheduling that cannot be completely resolved until runtime.

This provides a clean boundary between:

compile-time scheduling

and:

runtime scheduling

---

82. QEC integration

QEC is not the generic scheduler.

The relationship is:

QEC
 │
 ├── operation requirements
 ├── syndrome dependencies
 ├── round constraints
 ├── ancilla requirements
 ├── measurement requirements
 └── feedback requirements
        │
        ▼
scheduling
        │
        ├── timing
        ├── resource allocation
        ├── ordering
        └── verification

QEC must not hard-code scheduling algorithms.

---

83. "qec/interface.rs"

Defines the contract between QEC and scheduling.

It must express:

QEC operations
QEC dependencies
round boundaries
resource requirements
measurement dependencies
feedback requirements
timing constraints

It must not assume surface codes.

---

84. "qec/syndrome.rs"

Models syndrome-extraction scheduling requirements.

It must support arbitrary:

syndrome structures
ancilla counts
measurement patterns
round structures

without fixed topology assumptions.

---

85. "qec/rounds.rs"

Represents:

round identity
round dependency
round timing
round resource requirements
round completion

No hard-coded number of rounds.

---

86. "qec/stabilizer.rs"

Owns stabilizer-specific scheduling requirements.

It may understand stabilizer semantics.

It must not become the generic scheduler.

It must not hard-code:

distance = 3
fixed stabilizer weight
fixed ancilla count
fixed lattice
fixed number of rounds
fixed hardware topology

---

87. "stabilizer_scheduler.rs"

The existing "stabilizer_scheduler.rs" must remain a compatibility facade.

It must not become a second scheduling engine.

The repository's current file explicitly establishes this intended role and states that the historical implementation directly generated synthetic H/Measure/Reset instructions and comments rather than a real executable stabilizer schedule.

The production relationship is:

stabilizer_scheduler.rs
        │
        ▼
qec interface/model
        │
        ▼
generic scheduling
        │
        ▼
verification

It must not implement:

ASAP
ALAP
list scheduling
RCPSP
critical path
resource allocation

---

88. Distributed scheduling

The same scheduler architecture must eventually operate on:

single QPU
      ↓
multi-module QPU
      ↓
multi-QPU
      ↓
distributed quantum system
      ↓
quantum network

---

89. "distributed/node.rs"

Represents an execution module/node.

A node may contain:

quantum resources
classical resources
control resources
local scheduler context

No fixed number of nodes.

---

90. "distributed/link.rs"

Represents communication links.

Properties may include:

capacity
latency
availability
direction
fidelity
resource cost

All target supplied.

---

91. "distributed/communication.rs"

Represents operations such as:

classical communication
entanglement generation
teleportation
synchronization
remote operation

Communication is a schedulable resource.

---

92. "distributed/network.rs"

Represents the distributed resource graph.

The scheduler must distinguish:

local operation

from:

remote operation

and must account for communication dependencies.

---

93. Adapters

Adapters are critical because they prevent scheduler internals from becoming coupled to the rest of the compiler.

The repository already has an "adapters/ir.rs" boundary that references canonical "QuantumCircuit", "Gate", "QuantumOperation", "QubitId", and gate parameters.

---

94. "adapters/ir.rs"

This is the canonical boundary:

quantum::ir
     │
     ▼
scheduling::adapters::ir
     │
     ▼
scheduling::ir

It must consume canonical:

quantum::ir::QuantumCircuit
quantum::ir::QuantumOperation
quantum::ir::Gate
quantum::ir::qubit::QubitId
quantum::ir::qubit::PhysicalQubitId

where applicable.

It must not define:

QuantumGate
SchedulerQubit
SchedulerCircuit

as semantic replacements.

---

95. "adapters/routing.rs"

Consumes routing output.

Conceptually:

logical program
      │
      ▼
routing
      │
      ▼
mapped program
      │
      ▼
routing adapter
      │
      ▼
scheduler

The scheduler must receive the resulting physical identities rather than re-running routing.

---

96. "adapters/hardware.rs"

Consumes hardware capabilities.

Expected information may include:

supported operations
physical qubits
resource capacities
durations
timing resolution
alignment
availability
communication
calibration snapshot
measurement constraints
control constraints

The scheduler must not depend on provider SDK types.

---

97. "adapters/qec.rs"

Converts QEC requirements into generic scheduling constraints/resources.

This prevents:

generic scheduler

from depending directly on a specific QEC implementation.

---

98. ZQN integration

When ZQN is available, its role is:

ZQN
 │
 ├── error information
 ├── drift
 ├── crosstalk
 ├── uncertainty
 ├── duration uncertainty
 ├── fidelity
 └── temporal/spatial noise
        │
        ▼
scheduling adapter
        │
        ▼
scheduling objectives/constraints

Scheduling must not recreate the ZQN noise model.

---

99. Serialization

The schedule must be persistable for:

reproducibility
debugging
caching
distributed compilation
offline workflows
benchmarking
audit

The repository already contains a scheduling serialization namespace and schema boundary.

---

100. "serialization/schema.rs"

Defines a versioned schedule schema.

It must represent:

schedule identity
operations
times
durations
resources
reservations
constraints
target metadata
provenance
verification
objective

Schema versioning must be explicit.

---

101. "serialization/encode.rs"

Encodes validated schedule objects.

It must not serialize invalid internal states as if they were production schedules.

---

102. "serialization/decode.rs"

Must validate before constructing trusted scheduling objects.

Malformed serialized schedules must fail cleanly.

No unchecked conversion.

No unsafe deserialization.

---

103. Diagnostics

Large quantum programs require explainability.

A scheduler must answer:

«Why did operation X execute at time T?»

---

104. "diagnostics/trace.rs"

Records scheduling decisions.

Examples:

operation became ready
operation rejected due to dependency
operation rejected due to resource
operation delayed by alignment
resource became available
operation selected

Tracing must be optional.

---

105. "diagnostics/explain.rs"

Produces explanations such as:

Operation 42 could not start at T because
resource R7 remained occupied until T2.

This is essential for debugging large schedules.

---

106. "diagnostics/profile.rs"

Measures:

dependency analysis time
planning time
verification time
optimization time
serialization time
memory use
operation count
dependency count
resource conflicts

Profiling must not alter schedule semantics.

---

107. Plugin architecture

The scheduler must allow custom scheduling strategies without editing its core.

Potential plugins:

vendor scheduler
research scheduler
ML scheduler
custom heuristic
exact optimizer
domain-specific scheduler
distributed scheduler

Plugins must implement explicit contracts.

No global mutable registry.

Prefer:

caller-owned registry

over:

global singleton registry

---

108. Determinism

Production deterministic mode must guarantee:

same canonical input
+
same target snapshot
+
same configuration
+
same seed
+
same scheduler version

produces the same schedule, subject to explicitly dynamic target information.

Randomized algorithms must receive an explicit seed/context.

No hidden global RNG.

---

109. Parallelism

Scheduling may use parallel computation for:

dependency analysis
candidate evaluation
independent subgraphs
heuristic trials
objective evaluation

But deterministic mode must have deterministic arbitration.

Parallelism must never change semantics.

---

110. Memory scalability

Do not represent a schedule as:

Vec<Vec<Operation>>

where the outer dimension represents every time slot.

Prefer:

operation → interval
resource → reservation structure
dependency → adjacency structure

The architecture must scale according to actual program/resource size rather than:

qubits × maximum theoretical execution time

---

111. Sparse data structures

Where applicable, prefer sparse representations for:

dependency graphs
resource usage
availability
topology
communication

Large machines often have sparse interaction/resource graphs.

---

112. Event-driven execution model

The preferred scheduling architecture is:

ready set
+
resource calendars
+
event queue

rather than repeatedly scanning all operations.

Conceptually:

while unfinished:
    release completed resources
    activate newly ready operations
    evaluate candidates
    select according to policy
    reserve resources
    schedule selected operation
    advance to next relevant event

The implementation must retain explicit cancellation/deadline handling.

---

113. Cancellation

Long scheduling jobs must be cancellable.

Cancellation must be explicit in the scheduling context.

A cancelled schedule must return:

SchedulingError::Cancelled

or the canonical equivalent.

No partial result may be mistaken for a valid complete schedule.

---

114. Transactional behavior

Scheduling must be transactional:

input
  │
  ▼
immutable planning state
  │
  ├── success → complete result
  │
  └── failure → error

Caller-owned IR must not be silently partially modified.

If a transformation API is required, it should operate on explicit owned values or transactional compiler units.

---

115. Barriers

A barrier must be represented as an explicit scheduling constraint.

A barrier may apply to:

all operations
subset of qubits
subset of resources
specific region

The scheduler must not treat barriers as ordinary gates.

The barrier's semantic meaning is ordering, not quantum-state transformation.

---

116. Measurement

Measurement is both:

quantum operation

and potentially:

classical data producer

Therefore the scheduler must model:

measurement duration
readout resource
classical result availability
conditional consumers
feedback

---

117. Reset

Reset must be modeled as an executable operation with:

duration
resource usage
dependencies
target capability

It must not receive a universal zero-duration assumption.

---

118. Multi-qubit operations

Do not assume a maximum gate arity.

The scheduler must support arbitrary operation operand sets subject to target capabilities and resource constraints.

For example:

1-qubit
2-qubit
3-qubit
N-qubit

are all representations of the same generic scheduling concept.

Whether a target can execute a particular operation is determined by target capabilities and prior synthesis/lowering.

---

119. Unsupported operations

The scheduler must not silently decompose unsupported operations.

The pipeline is:

unsupported operation
        │
        ▼
synthesis/decomposition
        │
        ▼
routing
        │
        ▼
scheduling

If an operation reaches scheduling and remains unsupported, return a structured error.

---

120. Scheduling versus synthesis

These responsibilities must remain separate.

synthesis:
    "How can this operation be represented using supported operations?"

scheduling:
    "When can those operations execute?"

---

121. Scheduling versus routing

The boundary is:

routing:
    WHERE?

scheduling:
    WHEN?

Routing may introduce movement operations.

Scheduling assigns timing and resource reservations to those operations.

This follows established compiler practice where target placement/routing and scheduling are separate compilation concerns. Quantinuum's documented compilation architecture similarly separates placement/routing from subsequent compilation passes.

---

122. Scheduling versus hardware

Hardware owns:

capability
topology
timing
calibration
availability
execution
provider integration

Scheduling consumes a target description.

It does not become a hardware abstraction layer.

---

123. Scheduling versus runtime

Compile-time scheduling determines:

planned execution

Runtime scheduling handles:

dynamic readiness
hardware state changes
feedback
runtime conditions

These must remain compatible but distinct.

---

124. Target adaptation

A Zamani source program:

program.snk

must not contain:

qpu_127
channel_7
dt_1ns
topology_grid_100

unless those are explicitly part of an application-level target constraint rather than an architectural requirement.

Instead:

program
+
target
+
policy

produces:

target-specialized schedule

---

125. Example portability model

Small target:

program
→ IR
→ routing
→ scheduling
→ hardware

Large target:

same program
→ same IR
→ different routing
→ different resources
→ different scheduling
→ hardware

Distributed target:

same program
→ same IR
→ distributed routing
→ communication resources
→ distributed scheduling
→ execution

The source program remains unchanged.

---

126. Optimization ordering

The preferred high-level compiler pipeline is:

frontend
   ↓
canonical IR
   ↓
logical optimization
   ↓
synthesis/decomposition as required
   ↓
routing
   ↓
scheduling
   ↓
schedule transformations
   ↓
schedule optimization
   ↓
verification
   ↓
hardware lowering
   ↓
runtime

The exact order may vary for a target, but the ownership boundaries must not.

Modern quantum compiler systems similarly treat compilation as a sequence of transformations that progressively solve target constraints and optimize execution.

---

127. Scheduling quality metrics

Every production schedule should be capable of reporting:

makespan
critical-path length
scheduled depth
parallelism
resource utilization
idle time
communication overhead
alignment overhead
inserted delays
estimated fidelity
objective score
planning time
verification time

These metrics integrate naturally with Zamani's benchmarking subsystem.

---

128. Benchmarking integration

The scheduling subsystem must not implement benchmarking.

Benchmarking consumes:

ScheduleResult

and derives scheduling metrics.

The direction is:

scheduling
     │
     ▼
ScheduleResult
     │
     ▼
benchmarking

Never:

scheduling → benchmarking internals

---

129. Formal schedule invariants

A valid schedule must satisfy:

For every operation:

start >= release_time

For every dependency:

finish(A) <= start(B)

For every exclusive resource:

no overlapping reservations

For every capacity-limited resource:

usage(t) <= capacity(t)

For every duration:

finish = start + duration

For every timing alignment:

start satisfies alignment

For every target capability:

operation is executable on target

For every classical dependency:

producer result available before consumer use

For every communication dependency:

communication completion precedes dependent operation

And globally:

scheduled semantic program
==
input semantic program

---

130. Validation levels

The scheduler should support:

structural validation
dependency validation
resource validation
timing validation
semantic validation
target validation

Production mode should enable all mandatory levels.

Analysis/research modes may selectively disable expensive checks when explicitly configured.

---

131. Security

The scheduler must be safe Rust.

Every scheduling module should enforce:

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

The scheduler must not use:

raw pointers
unsafe FFI
unsafe blocks
mutable statics
global mutable state
unchecked memory operations

Provider FFI belongs behind a separate hardware/provider boundary and must not contaminate scheduler code.

---

132. Rust compatibility

The architecture targets:

Rust 1.97
Rust 1.97.1
Rust 2021
stable toolchain
no nightly features
no unsafe

Avoid APIs introduced after the minimum supported compiler unless the repository explicitly raises its MSRV.

---

133. Thread safety

Scheduler objects should prefer immutable/shared input.

The scheduler must not depend on global mutable state.

Independent scheduler instances should be independently constructible.

If a type is intended to be shared between threads, its "Send"/"Sync" properties must follow naturally from its fields rather than from unsafe manual implementations.

---

134. Serialization safety

Only validated scheduling objects should be serialized as production schedules.

Deserialization must:

decode
→ validate schema
→ validate identities
→ validate intervals
→ validate resources
→ validate dependencies
→ construct trusted object

---

135. Reproducibility

A schedule should record sufficient provenance to reproduce or explain it:

compiler version
scheduler version
algorithm
policy
objective
seed
target snapshot identity
configuration
input identity/hash

The exact provenance format belongs to scheduling serialization/provenance integration.

---

136. Caching

Scheduling caches must be optional.

A cache key must include every input that can affect the schedule, including where applicable:

canonical input identity
target identity/version
target capability snapshot
routing result
timing model
resource model
policy
objective
scheduler version
seed
configuration

Never cache solely by source program.

---

137. Incremental scheduling

Future production implementations should support partial recompilation where practical.

If a program changes only in one region:

unchanged region
+
changed region

the scheduler may reuse valid information if dependency/resource correctness remains provable.

Caching must never compromise correctness.

---

138. Hierarchical scheduling

For extremely large systems, scheduling should support hierarchical decomposition:

global schedule
   │
   ├── module A
   ├── module B
   ├── module C
   └── ...

Then:

module schedule

can be refined independently while respecting global synchronization constraints.

This is preferable to requiring every scheduler operation to inspect every physical resource in the entire machine.

---

139. Partitioning

Partitioning must be target/resource-driven.

Possible partition boundaries:

QPU module
execution region
QEC region
communication domain
independent dependency component

Partitions must expose explicit boundary dependencies.

---

140. Dependency components

If a program contains independent connected components, the scheduler may schedule them independently.

Example:

component A
     │
     └── no dependency ── component B

They may execute concurrently if resources allow.

This is one of the primary sources of scalable parallelism.

---

141. Large-machine principle

The scheduler must not become slower merely because the target advertises unused resources.

For example:

program uses 10 qubits
target has 10,000,000 resources

The scheduler should not need to construct an enormous dense representation of all resources merely because they exist.

Target/resource discovery should support sparse or demand-driven views.

---

142. Tiny-machine principle

The architecture must also work for:

one operation
one qubit
one resource

without requiring large-target infrastructure.

All optional components must be usable with minimal resource models.

---

143. Resource discovery model

The scheduler should request only the target information needed by the current program/policy where practical.

Conceptually:

program requirements
       │
       ▼
target capability query
       │
       ▼
relevant resources
       │
       ▼
SchedulingContext

This avoids materializing unnecessary target state.

---

144. No hard-coded topology

Never embed:

line
grid
heavy-hex
all-to-all
lattice
surface-code layout

as the scheduler's default physical architecture.

Topology belongs to hardware/routing/QEC.

---

145. No hard-coded channel counts

Never assume:

4 channels
8 channels
16 channels

The target reports resource capacities.

---

146. No hard-coded timing

Never assume:

H = 20 ns
CNOT = 100 ns
measure = 1 us

Durations come from target capabilities/calibration or symbolic timing.

---

147. No hard-coded gate set

The scheduler must not assume a universal gate set.

Gate support is a target capability.

Gate decomposition belongs to synthesis.

---

148. No hard-coded QEC geometry

Scheduling must not assume:

surface code
color code
repetition code
specific lattice

QEC supplies scheduling requirements.

---

149. No hard-coded number of shots

Shots belong to execution/benchmark configuration.

They are not scheduling semantics.

---

150. No hard-coded machine size

The source language, canonical IR, and scheduler must not encode:

machine has N qubits

unless that is explicitly a target capability.

---

151. Dynamic resource capacity

Resources may change over time.

The scheduler should therefore permit:

capacity(t)

rather than assuming:

capacity = constant

Examples:

calibration period
maintenance
temporary failure
resource reservation
degraded hardware

---

152. Dynamic target state

For compile-time scheduling:

target snapshot

should be used.

For runtime scheduling:

live target state

may be consulted through runtime-owned interfaces.

The generic scheduler must remain independent of the mechanism used to obtain the state.

---

153. Fault handling

If a resource becomes unavailable after schedule generation, runtime must not silently pretend the original schedule remains valid.

Possible runtime actions:

pause
reschedule
retry
reroute
abort

Those policies belong to runtime/execution orchestration.

---

154. QEC and scheduling recovery

QEC recovery operations may create new scheduling constraints.

The scheduler must support:

new dependency
new operation
new resource requirement
new timing requirement

without requiring a fundamentally different scheduler.

---

155. Measurement-driven execution

Dynamic scheduling must support:

measure
→ classify
→ branch
→ execute

without requiring the complete branch timing to be statically known.

---

156. Conditional resources

Some resources may be required only if a branch is taken.

The resource model must therefore support conditional or deferred reservations.

---

157. Classical processing resources

Quantum scheduling should not treat classical computation as infinitely fast.

Where target semantics require it, classical processing can consume:

CPU
memory
controller
decoder
feedback channel

resources.

---

158. Real-time constraints

Some target operations may have hard real-time requirements.

The scheduling model must distinguish:

soft preference

from:

hard deadline

A missed hard deadline must cause a scheduling failure rather than being silently ignored.

---

159. Communication timing

Distributed operations must include:

communication latency
resource occupancy
availability
completion dependency

Communication cannot be treated as zero-time.

---

160. Network scalability

Network resources should use sparse graph representations.

A global network with millions of nodes must not require a dense matrix if only a small number of links are relevant to a particular program.

---

161. Schedule representation

A schedule should fundamentally be an interval/resource assignment:

Operation
    ↓
start
duration
finish
resources
dependencies

rather than a giant array indexed by time.

---

162. Event queue

The event queue should support efficient retrieval of the next relevant event.

Possible events:

operation finished
resource released
window opens
communication completes
measurement becomes available
runtime condition resolves

---

163. Priority queue determinism

When multiple events or operations are equally eligible, deterministic mode must define stable tie-breaking.

Tie-breaking should use stable semantic identities rather than memory addresses.

---

164. Resource reservation atomicity

When scheduling an operation requiring multiple resources:

resource A
resource B
resource C

reservation must be atomic from the scheduler's perspective.

Do not partially reserve A and B and then discover C is unavailable without rolling back.

---

165. Failed scheduling transaction

A failed candidate operation must leave no stale reservations.

This invariant must be tested.

---

166. Verification after transformations

Every transformation that can alter:

timing
resources
operation placement in time

must be followed by the appropriate verification layer.

---

167. Verification after optimization

Optimization is not trusted merely because it is internal.

The final schedule must be independently verified.

---

168. Schedule equivalence

The scheduler must preserve the semantic identity of the computation.

If scheduling transformations intentionally modify representation—for example explicit delays—that transformation must have a formally defined semantic relationship to the original program.

---

169. Test architecture

The repository should maintain:

tests/
├── unit/
├── integration/
├── property/
├── regression/
├── scalability/
├── determinism/
└── fixtures/

---

170. Unit tests

Every foundational file requires direct tests.

Examples:

duration arithmetic
time intervals
resource capacity
reservation conflicts
dependency edges
cycle detection
alignment
windows
priority ordering

---

171. Integration tests

Required boundaries:

IR → scheduling
routing → scheduling
hardware → scheduling
QEC → scheduling
dynamic → scheduling
distributed → scheduling
serialization → scheduling
scheduling → runtime
scheduling → benchmarking

---

172. Property tests

Important invariants:

no exclusive resource overlaps
dependency ordering is preserved
capacity is never exceeded
valid schedule remains valid after serialization round-trip
deterministic input produces deterministic output

---

173. Regression tests

Every discovered scheduler bug must receive a permanent regression test.

---

174. Scalability tests

Scale dimensions independently:

operation count
qubit count
dependency edge count
resource count
resource pressure
parallelism
QEC rounds
communication nodes
communication edges

Do not use artificial scheduler maximums merely to make tests convenient.

---

175. Determinism tests

Run:

same input
same target
same configuration
same seed

multiple times.

The schedules must compare equal under deterministic mode.

---

176. Required edge cases

At minimum:

zero operations
one operation
one qubit
many qubits
one resource
many resources
independent operations
fully serialized operations
parallel operations
resource conflict
resource capacity > 1
measurement
reset
conditional
feedback
barrier
deadline
release time
alignment
zero-duration operation where semantically valid
symbolic duration
unknown duration
cycle
invalid resource
unavailable resource
dynamic resource
distributed operation
QEC round
serialization round trip
cancellation

---

177. Integration with optimization

The scheduling subsystem must consume canonical IR produced by:

quantum::optimization

It must not inspect optimizer internals.

The optimizer must be able to change the logical program without requiring scheduler implementation changes.

---

178. Integration with routing

Routing produces the physical placement/movement representation.

Scheduling consumes it.

The scheduler must not re-run routing.

---

179. Integration with hardware

Hardware supplies:

capabilities
resources
timing
availability
calibration snapshot

Scheduling produces:

target-specialized schedule

Hardware lowering converts that schedule to executable target representation.

---

180. Integration with runtime

Runtime receives a validated schedule.

Runtime owns:

execution
submission
queue
hardware communication
feedback
cancellation
runtime state

Scheduling owns none of those mechanisms.

---

181. Integration with simulator

The simulator may consume a schedule.

The simulator must not require the scheduler to understand simulation internals.

---

182. Integration with benchmarking

Benchmarking consumes:

ScheduleResult

to measure:

depth
makespan
parallelism
resource utilization

---

183. Integration with QEC

QEC produces scheduling requirements.

Scheduling resolves:

when
where resources are occupied in time

QEC remains responsible for:

codes
syndromes
decoding
logical fault tolerance

The quantum root already assigns QEC those responsibilities rather than scheduling.

---

184. Integration with ZQN

ZQN supplies physical uncertainty and noise semantics.

Scheduling may use them for:

fidelity objective
noise-aware ordering
crosstalk avoidance
duration uncertainty

but does not own the noise model.

---

185. Integration with frontend

Frontend produces canonical IR.

Scheduling must never consume:

OpenQASM AST
Zamani parser AST
frontend tokens

directly.

The boundary is:

frontend
→ canonical IR
→ scheduler adapter

---

186. OpenQASM relationship

OpenQASM-style dynamic control and timing reinforce the need for a scheduler capable of handling:

timing
delays
classical conditions
measurement
feedback

rather than treating a quantum program as merely a static sequence of gates.

The scheduling architecture should therefore remain more general than a simple gate-list scheduler.

---

187. External compiler architecture

The separation between:

abstract program
→ target constraints
→ placement/routing
→ optimization
→ execution

is consistent with established quantum compiler architectures. Quantinuum's tket documentation describes compilation as solving target constraints and optimizing implementations, including placement and routing for physical architectures.

Zamani should preserve this separation while making scheduling an independently reusable subsystem.

---

188. No vendor coupling

No scheduler source file may import:

vendor SDK
provider credentials
provider API client
provider-specific job object

Vendor-specific logic belongs in:

quantum::hardware

or a hardware adapter.

---

189. No parser coupling

No scheduling algorithm may import:

frontend parser
lexer
OpenQASM parser
source AST

---

190. No benchmark coupling

No scheduling algorithm may import:

benchmark execution
statistical analysis
benchmark reporting

---

191. No QEC algorithm duplication

Generic scheduling must not implement:

decoder
syndrome decoding
logical recovery
code construction

---

192. No optimization duplication

Scheduling must not become a second gate optimizer.

It may optimize timing/resource placement, but gate algebra belongs to "quantum::optimization".

The repository's optimization architecture already explicitly separates scheduling from logical optimization.

---

193. API stability

The public API should be organized around stable concepts:

SchedulingContext
SchedulingConfig
SchedulingLimits
Scheduler/Planner
ScheduleResult
SchedulingError
Policy
Resource
Constraint

Internal algorithm implementation should remain replaceable.

---

194. Composition root

"src/quantum/scheduling/mod.rs" must be a composition root.

It should contain:

module declarations
documentation
selected stable exports

It must not contain:

scheduling algorithm
resource algorithm
timing algorithm
hardware discovery
global state

The quantum root already follows this composition-root philosophy.

---

195. Avoid wildcard exports

Do not use:

pub use algorithms::*;
pub use ir::*;
pub use resources::*;

Prefer explicit exports.

This prevents unrelated additions from accidentally changing the public API.

---

196. File completion rule

A scheduling file is considered complete only when:

1. Its ownership is documented.
2. Its inputs are defined.
3. Its outputs are defined.
4. Its invariants are defined.
5. Its error behavior is defined.
6. Its serialization behavior is defined if applicable.
7. Its thread-safety expectations are defined.
8. Its scalability expectations are defined.
9. Its dependencies are known.
10. Its integration boundary is known.
11. It does not duplicate another subsystem's responsibility.
12. Its tests exist.
13. It does not require future files to redefine its public contract.

This is the mechanism for achieving:

«Finish a file once without reopening it merely because another file was implemented later.»

---

197. Dependency freeze rule

Before implementing a file, its dependency direction must be frozen.

For example:

types
  ↓
resources
  ↓
planner
  ↓
algorithm

must not later become:

types
  ↔
algorithm

Circular dependencies indicate a broken ownership boundary.

---

198. Interface-first implementation

Every major subsystem should be implemented in this order:

contract
↓
types
↓
errors
↓
invariants
↓
implementation
↓
tests
↓
integration

Not:

implementation
↓
discover architecture problems
↓
rewrite interfaces

---

199. Production readiness gate

"quantum::scheduling" is not production-ready until all of the following are true:

[ ] canonical scheduler types
[ ] canonical errors
[ ] explicit limits
[ ] no machine-size constants
[ ] timing model
[ ] resource model
[ ] availability model
[ ] dependency graph
[ ] critical-path analysis
[ ] constraint engine
[ ] ASAP
[ ] ALAP
[ ] list scheduling
[ ] resource-constrained scheduling
[ ] event-driven scheduling
[ ] dynamic scheduling
[ ] explicit delays
[ ] alignment
[ ] QEC integration
[ ] distributed scheduling
[ ] IR adapter
[ ] routing adapter
[ ] hardware adapter
[ ] QEC adapter
[ ] ZQN integration
[ ] verification
[ ] semantic verification
[ ] deterministic mode
[ ] reproducibility
[ ] serialization
[ ] diagnostics
[ ] plugins
[ ] cancellation
[ ] scalability tests
[ ] property tests
[ ] regression tests
[ ] integration tests
[ ] compiler integration
[ ] runtime integration
[ ] benchmarking integration
[ ] Rust 1.97 compatibility
[ ] Rust 1.97.1 compatibility
[ ] no unsafe
[ ] API documentation

---

200. Final canonical architecture

The production architecture is:

                         ZAMANI PROGRAM
                              │
                              ▼
                    ┌──────────────────┐
                    │ quantum::frontend│
                    └────────┬─────────┘
                             │
                             ▼
                    ┌──────────────────┐
                    │   quantum::ir    │
                    │   canonical WHAT │
                    └────────┬─────────┘
                             │
                    optimization
                             │
                             ▼
                    ┌──────────────────┐
                    │     routing      │
                    │      WHERE?      │
                    └────────┬─────────┘
                             │
                             ▼
                    ┌──────────────────┐
                    │   scheduling     │
                    │      WHEN?       │
                    └────────┬─────────┘
                             │
              ┌──────────────┼──────────────┐
              │              │              │
              ▼              ▼              ▼
          dependencies    resources       timing
              │              │              │
              └──────────────┼──────────────┘
                             │
                             ▼
                         policies
                             │
                             ▼
                         planners
                             │
                             ▼
                         schedule
                             │
                             ▼
                       verification
                             │
                             ▼
                     schedule optimization
                             │
                             ▼
                       final verification
                             │
                 ┌───────────┼───────────┐
                 │           │           │
                 ▼           ▼           ▼
               QEC          ZQN       distributed
                 │           │           │
                 └───────────┼───────────┘
                             │
                             ▼
                       hardware adapter
                             │
                             ▼
                     hardware lowering
                             │
                             ▼
                           runtime
                             │
                 ┌───────────┼───────────┐
                 ▼           ▼           ▼
              simulator      QPU       emulator

---

201. The fundamental public scheduling model

The central conceptual API should remain:

schedule(
    program,
    target,
    policy
)

not:

schedule(
    program,
    127 qubits,
    8 channels,
    100ns gate time
)

The first means:

program
+
target capabilities
+
scheduling policy

The second embeds machine assumptions into the compiler.

The first is the Zamani architecture.

---

202. The final scalability invariant

The most important invariant in this document is:

Zamani source semantics
        ≠
physical machine size

The same source program must be able to produce:

small-target schedule
large-target schedule
distributed-target schedule
future-target schedule

without changing the source program merely because the target has a different number of physical resources.

Only target specialization changes:

resources
topology
timing
availability
constraints
routing
schedule

The program's semantic meaning does not.

---

203. Final ownership table

Concern| Owner
Source syntax| "quantum::frontend"
Canonical quantum semantics| "quantum::ir"
Canonical qubit identity| "quantum::ir::qubit"
Gate algebra| "quantum::optimization"
Synthesis/decomposition| synthesis/optimization
Logical → physical mapping| "quantum::routing"
Ordering| "quantum::scheduling"
Timing| "quantum::scheduling"
Resource reservation| "quantum::scheduling"
Scheduling constraints| "quantum::scheduling"
Target capabilities| "quantum::hardware"
Calibration| "quantum::hardware"
Noise semantics| "quantum::zqn"
QEC algorithms| "quantum::error_correction"
QEC scheduling constraints| "quantum::scheduling::qec"
Hardware execution| hardware/runtime
Runtime feedback| runtime
Benchmarking| "quantum::benchmarking"
Schedule diagnostics| "quantum::scheduling::diagnostics"

---

204. Absolute architectural prohibitions

The following are forbidden inside the scheduling implementation:

MAX_QUBITS
MAX_PHYSICAL_QUBITS
MAX_OPERATIONS
MAX_ROUNDS
MAX_CHANNELS
MAX_DEPTH
fixed topology
fixed timing
fixed gate durations
fixed gate set
fixed QEC distance
fixed stabilizer geometry
fixed ancilla count
vendor SDK imports
provider credentials
hardware network access
hardware discovery
duplicate QubitId
duplicate PhysicalQubitId
duplicate QuantumOperation
duplicate QuantumCircuit
duplicate quantum Gate type
global mutable scheduler state
hidden random seeds
unsafe Rust
silent unsupported-gate decomposition
silent resource overcommit
silent deadline violation
silent dependency violation
partial successful schedules

---

205. Final architectural statement

"src/quantum/scheduling/" is not a collection of ASAP/ALAP implementations.

It is Zamani's universal execution-planning layer.

Its fundamental abstraction is:

semantic operations
+
dependencies
+
resources
+
timing
+
constraints
+
target capabilities
+
policy
+
objective
        │
        ▼
     schedule

It must remain independent of:

machine size
vendor
technology
topology
gate set
QEC code
noise model
execution provider

Those are inputs to specialization, not properties of the scheduling language.

The resulting architecture permits:

one Zamani program
       │
       ├── tiny quantum machine
       ├── large quantum machine
       ├── fault-tolerant machine
       ├── modular machine
       ├── distributed quantum computer
       └── future quantum architecture

with the source program remaining semantically unchanged.

That is the required foundation for write once, scale from atom to everywhere.The key repository-specific correction is that this architecture does not replace the existing scheduling tree with a competing design. It formalizes the scheduling modules already appearing in the repository and makes their contracts consistent with the canonical IR, routing, hardware, QEC, dynamic, verification, and serialization boundaries. The current scheduling files already reference the canonical QuantumOperation/QuantumCircuit concepts, and the repository's own search results explicitly show the requirement not to introduce competing QubitId/PhysicalQubitId/operation representations.

External compiler architecture supports the same separation: tket describes compilation as solving target constraints while separating placement/routing from other compilation passes, and its documentation emphasizes that high-level programs should not require users to know the exact device architecture. 

I would treat this ARCHITECTURE.md as the contract that freezes the scheduling subsystem before individual .rs files are finalized.