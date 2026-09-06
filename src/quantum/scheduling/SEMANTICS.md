Worked for 14s

Yes. I checked the current repository structure and the existing scheduling/IR contracts before writing this. The current tree already contains substantial scheduling documentation and implementations, including ARCHITECTURE.md, DESIGN.md, COMPATIBILITY.md, DETERMINISM.md, SCALABILITY.md, types.rs, context.rs, config.rs, the algorithm/policy layers, and QEC adapters.

I also verified that the repository's canonical quantum identity is quantum::ir::qubit::QubitId / PhysicalQubitId, and that downstream modules are explicitly forbidden from introducing competing qubit identity types.

Below is the complete proposed src/quantum/scheduling/SEMANTICS.md. It is deliberately written as a normative contract so the Rust files can be implemented against it without later semantic redesign.

# Zamani Quantum Scheduling — Semantic Contract

**Path:** `src/quantum/scheduling/SEMANTICS.md`  
**Subsystem:** `crate::quantum::scheduling`  
**Language:** Rust  
**Edition:** Rust 2021  
**Required Rust baseline:** Rust 1.97.1  
**Safety model:** Safe Rust only; `unsafe` forbidden  
**Status:** Normative production semantic specification  
**Scope:** Generic quantum scheduling for Zamani, from the smallest executable quantum system to arbitrarily large systems constrained only by actual resources, target capabilities, and explicit execution policy.

---

## 1. Purpose

This document defines the semantic meaning of scheduling in Zamani.

It is the normative contract for:

- `types.rs`
- `errors.rs`
- `config.rs`
- `limits.rs`
- `context.rs`
- `result.rs`
- `ir/`
- `resources/`
- `timing/`
- `policies/`
- `planners/`
- `constraints/`
- `transformations/`
- `verification/`
- `optimization/`
- `qec/`
- `dynamic/`
- `distributed/`
- `adapters/`
- `serialization/`
- `diagnostics/`
- `algorithms/`
- `plugins/`
- `stabilizer_scheduler.rs`
- scheduling tests and integration layers.

The scheduler is responsible for determining **when** executable quantum operations and their associated classical/resource activities occur.

It is not responsible for defining what a quantum program means.

The fundamental architecture is:

```text
Zamani source
    |
    v
quantum::frontend
    |
    v
canonical quantum::ir
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
    |
    v
execution target

Scheduling therefore operates on an already meaningful quantum computation.


---

2. Core semantic principle

The fundamental scheduling invariant is:

SCHEDULED SEMANTICS == SOURCE SEMANTICS

Scheduling may change execution timing and legal execution ordering.

Scheduling MUST NOT silently change the computation.

A valid scheduler transforms:

what must happen

into:

when it happens

subject to:

dependencies
resources
timing
target capabilities
constraints
execution policy

The scheduler therefore answers:

> WHEN?



Routing answers:

> WHERE?



Hardware answers:

> CAN IT EXECUTE?



Runtime answers:

> EXECUTE IT.



These responsibilities MUST remain distinct.


---

3. Semantic ownership

The following ownership model is normative.

Concern	Owner

Zamani syntax	quantum::frontend
Quantum program meaning	quantum::ir
Canonical qubit identity	quantum::ir::qubit
Gate semantics	quantum::ir::gate
Measurement semantics	quantum::ir::measurement
Optimization	quantum::optimization
Logical-to-physical mapping	quantum::routing
Execution ordering	quantum::scheduling
Execution timing	quantum::scheduling
Resource reservations	quantum::scheduling
Hardware capabilities	quantum::hardware
Hardware communication	hardware/runtime layer
Noise/uncertainty model	quantum::zqn
QEC semantics	quantum::error_correction
QEC decoding	QEC subsystem
Runtime execution	runtime
Benchmarking	benchmarking subsystem
Scheduling diagnostics	quantum::scheduling::diagnostics


No scheduling implementation may silently absorb another subsystem's semantic ownership.


---

4. What scheduling means

A schedule is a mapping from executable activities to temporal/resource intervals.

Conceptually:

operation
    |
    +-- start time
    +-- duration
    +-- end time
    +-- required resources
    +-- dependencies
    +-- timing constraints
    +-- execution condition
    +-- provenance

A schedule is valid only when every scheduled activity satisfies every applicable semantic constraint.

For an operation o:

start(o)
duration(o)
finish(o)

must satisfy:

finish(o) = start(o) + duration(o)

using checked arithmetic.


---

5. Scheduling is a specialization, not a rewrite

A Zamani program is written independently of a specific machine.

The same program may be scheduled for:

one-qubit device
two-qubit device
small QPU
large QPU
multi-chip system
distributed quantum computer
quantum network
future quantum architecture

The source program does not need to encode:

number of physical qubits
number of control channels
topology
clock frequency
gate duration
measurement latency
communication latency
resource count

Those are target properties.

The semantic flow is:

same program
    |
    +---- target A --> routing A --> scheduling A
    |
    +---- target B --> routing B --> scheduling B
    |
    +---- target C --> routing C --> scheduling C

The schedules may differ.

The computation must not.


---

6. Meaning of "write once, scale everywhere"

Zamani's portability guarantee is semantic portability, not identical physical timing.

A source program describes the intended computation.

Target-specific compilation determines:

mapping
timing
resource allocation
communication
alignment
error-management strategy

Therefore:

PROGRAM
    !=
MACHINE SCHEDULE

The scheduler MUST never require the source program to contain machine-specific scheduling constants merely to make the program executable.


---

7. No artificial scalability ceiling

The scheduling architecture MUST NOT impose an artificial maximum number of:

qubits
operations
resources
channels
dependencies
QEC rounds
nodes
links
schedule depth

The following patterns are prohibited as semantic limits:

const MAX_QUBITS: usize = ...;
const MAX_OPERATIONS: usize = ...;
const MAX_CHANNELS: usize = ...;
const MAX_ROUNDS: usize = ...;
const MAX_DEPTH: usize = ...;

This does not prohibit operational safeguards.

There is a fundamental distinction between:

architectural semantic limit

and:

explicit execution/resource budget

A caller MAY impose:

memory budget
CPU budget
deadline
cancellation
maximum materialization budget
maximum diagnostic output

Those are execution policies.

They are not limitations on what Zamani means.


---

8. Meaning of "infinity"

"Scale to infinity" means:

> No finite machine-size ceiling is encoded into scheduling semantics.



Actual execution remains constrained by:

available memory
available CPU
address space
operating-system limits
target resources
target capabilities
network capacity
storage
execution deadline
explicit resource policy

The scheduler MUST therefore scale until an actual resource or explicitly requested execution budget prevents further computation.

When that happens, it MUST report a structured failure.

It MUST NOT:

silently truncate;

silently drop operations;

invent resources;

wrap counters;

return an incomplete successful schedule.



---

9. Canonical quantum identity

All scheduling code MUST use the canonical quantum identity types.

The canonical paths are:

crate::quantum::ir::qubit::QubitId
crate::quantum::ir::qubit::PhysicalQubitId
crate::quantum::ir::qubit::QubitRef

The repository explicitly establishes this identity boundary.

Scheduling MUST NOT define another:

QubitId
PhysicalQubitId
QubitRef

or semantically equivalent replacement.

This rule applies to:

scheduler types;

resources;

constraints;

adapters;

QEC;

routing integration;

serialization;

tests;

compatibility layers.



---

10. Logical and physical qubits

The scheduler MUST preserve the distinction between:

QubitId

and:

PhysicalQubitId

Logical identity belongs to the quantum program.

Physical identity belongs to the execution target.

The expected flow is:

logical program
    |
    v
canonical QubitId
    |
    v
routing
    |
    v
PhysicalQubitId
    |
    v
scheduling

Scheduling MUST NOT silently invent or modify the logical-to-physical mapping.

If a mapping is required and absent, scheduling MUST return a structured error.


---

11. Scheduler-local identities

Scheduling MAY define scheduler-owned identifiers such as:

ScheduleId
SchedulingOperationId
ReservationId
DependencyId
ResourceId
EventId

These identifiers represent scheduler concepts.

They MUST NOT replace canonical quantum identities.

For example:

SchedulingOperationId

identifies a scheduling record.

It does not become a replacement for the canonical quantum IR operation identity.

Every scheduling operation SHOULD retain provenance to its source IR operation.


---

12. Provenance

Every executable scheduled activity MUST be traceable to its origin.

Conceptually:

ScheduleEntry
    |
    +--> source program
    +--> source operation
    +--> source IR identity
    +--> mapping provenance
    +--> scheduling decision

Inserted scheduling artifacts such as delays or padding MUST explicitly identify their origin as:

scheduler-generated

rather than pretending to be source operations.

This distinction is essential for:

semantic verification;

diagnostics;

serialization;

debugging;

reproducibility;

benchmarking.



---

13. Canonical IR boundary

Scheduling MUST consume canonical quantum IR rather than defining another quantum language.

The scheduler MAY create an internal scheduling view.

That view MUST contain only information required for scheduling.

Conceptually:

quantum::ir
    |
    v
adapters::ir
    |
    v
SchedulingOperation

The scheduling view MAY include:

operation identity
operands
physical mapping
dependencies
duration
resource requirements
conditions
timing constraints
provenance

It MUST NOT redefine quantum gate semantics.


---

14. Internal scheduling IR

ir/operation.rs represents scheduling concerns.

A scheduling operation MUST be capable of representing:

source operation
operands
resource requirements
duration
precedence
release time
deadline
conditions
classical dependencies
communication dependencies
QEC metadata
provenance

It MUST support arbitrary operation arity.

The scheduler MUST NOT assume that every operation acts on:

one qubit

or:

two qubits

An operation may have any arity permitted by the canonical IR and target capability model.


---

15. Dependency semantics

Scheduling is constrained by a dependency relation.

A dependency:

A -> B

means:

B cannot execute before the semantic condition represented by A -> B is satisfied.

For ordinary precedence:

finish(A) <= start(B)

must hold.

Dependencies MAY represent:

quantum data dependencies;

classical data dependencies;

measurement readiness;

reset completion;

control dependencies;

feedback dependencies;

resource dependencies;

communication completion;

QEC phase dependencies.



---

16. Dependency graph

ir/graph.rs represents scheduler-relevant precedence.

For static scheduling, the graph SHOULD be acyclic.

A cycle in a graph that requires acyclicity is an error:

CycleDetected

The scheduler MUST NOT break a cycle arbitrarily.

It MUST NOT:

drop an edge;

drop an operation;

reorder the cycle;

return a partial successful schedule.



---

17. Dynamic dependencies

Not every quantum program is a static DAG.

Zamani scheduling MUST support:

static dependencies
+
conditional dependencies
+
runtime events
+
classical feedback

Therefore dynamic scheduling MAY contain dependencies whose resolution occurs during execution.

The semantic distinction is:

static dependency

versus:

runtime dependency

The scheduler MUST preserve both.


---

18. Classical control

A quantum operation may depend on a classical result.

For example:

measure q0 -> c0

if c0:
    X q1

The semantic requirement is:

measurement completion
    ->
classical result availability
    ->
condition evaluation
    ->
conditional operation readiness

Scheduling MUST NOT schedule the conditional operation before the required classical value is available.

The scheduler does not need to execute arbitrary classical programs merely to establish this dependency.

It may represent the condition symbolically.


---

19. Measurement semantics

Measurement is not merely another gate.

A measurement may create:

quantum completion
classical data availability
readout resource occupancy
feedback readiness

Therefore scheduling MUST represent all relevant measurement dependencies.

A measurement consumer MUST NOT be considered ready merely because the physical measurement pulse has started.

The target timing model determines when the result becomes available.


---

20. Reset semantics

Reset establishes a new qubit state only after reset has completed according to the target semantics.

Operations dependent on reset completion MUST wait for reset readiness.

Reset may require:

physical resources;

measurement;

feedback;

cooling;

initialization;

alignment.


These are target/context properties, not scheduler constants.


---

21. Resource semantics

A resource is anything whose availability constrains execution.

Resources MAY include:

physical qubits
control channels
measurement channels
readout resonators
lasers
microwave channels
couplers
ancillas
classical processors
memory
communication links
network capacity
synchronization resources
target-specific resources

The resource system MUST be generic.

No resource type may assume a particular quantum technology.


---

22. Resource capacity

A resource has a capacity.

For a resource R:

usage(R, t) <= capacity(R, t)

must hold for every relevant time.

Capacity MAY be:

fixed
time-varying
availability-dependent
mode-dependent
hierarchical
shared
exclusive

The scheduler MUST obtain capacity from the target/resource context.


---

23. Exclusive resources

An exclusive resource cannot serve incompatible activities simultaneously.

For:

A: [10,20)
B: [15,25)

on an exclusive resource:

R

the schedule is invalid.

For compatible non-overlapping intervals:

A: [10,20)
B: [20,30)

the schedule may be valid.

The exact boundary convention MUST be defined consistently by the resource implementation.

The recommended interval convention is:

[start, end)

so an operation ending at t does not occupy the resource at t.


---

24. Shared resources

A resource may have capacity greater than one.

Example:

capacity = N

Then:

active users <= N

must hold.

The scheduler MUST NOT encode:

N = 8

or any other universal value.


---

25. Hierarchical resources

Some systems have resource hierarchies.

Example:

device
 |
 +-- module A
 |    +-- channel 1
 |    +-- channel 2
 |
 +-- module B
      +-- channel 3

An operation may consume:

module capacity
+
channel capacity

simultaneously.

The resource model MUST be capable of expressing parent/child relationships without assuming a fixed hierarchy depth.


---

26. Resource aliases

Different names MUST NOT accidentally identify different physical resources.

Resource identity must be explicit.

Adapters MAY translate:

hardware resource identifier
    ->
scheduler ResourceId

but the mapping MUST be validated.

Two resources MUST NOT be treated as equivalent merely because their display names match.


---

27. Routing boundary

Routing determines where operations execute.

Scheduling determines when they execute.

The integration is:

canonical IR
    |
    v
optimization
    |
    v
routing
    |
    v
mapped executable IR
    |
    v
scheduling

Scheduling MUST consume routing results.

It MUST NOT implement an alternative hidden routing system.


---

28. Routing validation

Before scheduling, the routing adapter MUST establish that:

logical operands
    ->
valid physical operands

and that the mapped operations are compatible with the target topology/capability model.

If the routing result is invalid:

scheduling MUST fail

rather than silently repairing the mapping.

Repair belongs to routing or an explicitly authorized transformation stage.


---

29. Hardware boundary

Scheduling does not own hardware discovery.

The hardware subsystem supplies target information.

Conceptually:

quantum::hardware
    |
    v
HardwareCapabilities / target snapshot
    |
    v
adapters::hardware
    |
    v
SchedulingContext

The scheduler consumes:

supported operations
durations
timing resolution
alignment
resources
capacity
availability
communication capabilities

It does not directly communicate with the QPU.


---

30. No vendor dependency in scheduling core

The scheduling core MUST NOT directly depend on:

vendor SDKs
vendor authentication
vendor credentials
vendor network protocols
vendor execution clients

Vendor-specific behavior belongs in adapters/hardware/runtime integration.

This keeps scheduling technology-independent.


---

31. Hardware target snapshot

Scheduling SHOULD operate on an immutable target snapshot.

The snapshot represents the target state used to make scheduling decisions.

It may contain:

capabilities
topology
timing
resources
availability
calibration-derived properties
communication properties
version
identity
provenance

The scheduler MUST NOT assume the target remains unchanged after scheduling unless the execution contract explicitly guarantees that property.


---

32. Dynamic hardware

If target resources can change while a schedule is being produced or executed, the scheduler MUST represent that explicitly.

Possible states include:

available
busy
disabled
degraded
unknown

An unknown resource state MUST NOT automatically be treated as available.

The policy must determine whether unknown state causes:

failure
conservative exclusion
rescheduling
runtime decision


---

33. Timing semantics

Timing is represented independently of a particular physical clock.

The scheduler must support target-provided:

continuous time
discrete time
clock ticks
sample periods
rational resolution
target-defined timing units

The scheduler MUST NOT assume a universal:

1 ns
10 ns
1 us
dt


---

34. Duration semantics

An operation duration may be:

known
symbolic
target-dependent
calibrated
interval-valued
runtime-determined
unknown

The scheduler MUST NOT fabricate a duration when the target contract does not provide one.

If scheduling requires a concrete duration and none can be established, it MUST return a structured error.


---

35. Duration validity

A concrete duration MUST satisfy the timing model's validity rules.

Unless explicitly supported by the target:

negative duration

is invalid.

Non-finite values such as:

NaN
+infinity
-infinity

MUST NOT be accepted as ordinary physical durations.


---

36. Time arithmetic

All scheduling time arithmetic MUST be checked.

The scheduler MUST prevent:

overflow
underflow
invalid intervals
negative durations
invalid deadlines

For:

finish = start + duration

overflow MUST produce a structured error.

It MUST NOT wrap.


---

37. Time intervals

The canonical scheduler interval SHOULD be:

[start, end)

This provides deterministic resource occupancy semantics.

An interval is valid only when:

start <= end

unless the type explicitly represents a different semantic object.

Zero-duration activities MAY exist if the target contract permits them.


---

38. Timing resolution

A target may require:

start % resolution == 0

or another alignment rule.

Timing resolution belongs to:

timing::resolution

and is supplied through the target/context.

No timing grid may be hard-coded into a scheduler algorithm.


---

39. Alignment semantics

Alignment constraints may apply to:

operation starts
operation ends
channels
measurement
control pulses
frames
communication
modules

A candidate schedule is invalid if it violates a required alignment constraint.

Alignment MUST be checked both:

during scheduling

and:

during final verification


---

40. Release times

An operation MAY have a release time:

start(o) >= release(o)

No policy may schedule an operation before its release.


---

41. Deadlines

An operation MAY have a deadline.

The exact deadline semantics MUST be explicit.

For a finish deadline:

finish(o) <= deadline(o)

For a start deadline:

start(o) <= deadline(o)

The scheduler MUST NOT confuse these meanings.


---

42. Scheduling policies

Policies define scheduling preferences.

Examples:

ASAP
ALAP
priority
critical-path
resource-aware
hybrid
adaptive

Policies MUST NOT change the semantic meaning of operations.

They only select among legal schedules.


---

43. ASAP semantics

ASAP means:

> Schedule each operation as early as possible while satisfying all applicable constraints under the selected arbitration/resource policy.



ASAP does not mean:

ignore resources

or:

ignore timing

A resource-aware ASAP scheduler must still respect resources.


---

44. ALAP semantics

ALAP means:

> Schedule operations as late as possible while satisfying the relevant scheduling horizon, dependencies, resource constraints, and timing requirements.



ALAP requires a meaningful scheduling horizon.

That horizon MUST be explicit or derivable from the target/context.


---

45. List scheduling

List scheduling operates conceptually as:

dependency analysis
    |
    v
ready set
    |
    v
priority selection
    |
    v
resource feasibility
    |
    v
reservation
    |
    v
resource release
    |
    v
next ready set

The algorithm MUST never select an operation that violates:

dependency
resource
timing
target capability
condition readiness


---

46. Critical-path semantics

Critical path identifies dependency structure that constrains the minimum possible makespan.

Critical-path analysis MAY be used for:

priority
ASAP
ALAP
diagnostics
optimization

Critical-path analysis MUST NOT be interpreted as the complete solution to resource-constrained scheduling.


---

47. Resource-constrained scheduling

Real quantum hardware often has constraints beyond qubit dependencies.

The scheduler MUST therefore support:

dependency constraints
+
resource constraints
+
timing constraints

simultaneously.

The architecture MUST permit exact, heuristic, approximate, deterministic, and adaptive strategies.

No algorithm may falsely claim global optimality unless it has actually established it.


---

48. Optimization semantics

Optimization changes the selection of a legal schedule.

It MUST NOT change the source computation unless an explicitly authorized transformation stage is invoked.

Possible objectives include:

minimum makespan
minimum depth
minimum idle time
maximum estimated fidelity
minimum energy
minimum communication
multi-objective optimization


---

49. Multi-objective semantics

A multi-objective scheduler may optimize:

makespan
idle time
fidelity
energy
communication
resource utilization

Weights or priorities MUST come from explicit configuration.

The scheduler MUST NOT silently embed universal weights.


---

50. ZQN integration

ZQN owns the noise/uncertainty model.

Scheduling may consume ZQN-derived information such as:

duration uncertainty
gate error estimates
drift
crosstalk
fidelity estimates
resource reliability

through an adapter.

The scheduler MUST NOT duplicate the canonical ZQN model.

The integration is:

ZQN
 |
 v
noise/uncertainty information
 |
 v
scheduling adapter
 |
 v
objective/constraint
 |
 v
scheduler


---

51. Fidelity-aware scheduling

A fidelity-aware scheduler may prefer:

slightly longer schedule

over:

shorter but lower-quality schedule

if the configured objective says so.

This is still scheduling optimization.

It MUST NOT silently rewrite gate semantics.


---

52. QEC semantics

QEC scheduling is an extension of generic scheduling.

The relationship is:

logical program
    |
    v
QEC compilation
    |
    v
fault-tolerant operations
    |
    v
routing
    |
    v
scheduling

QEC supplies scheduling requirements.

Generic scheduling supplies timing/resource coordination.


---

53. QEC constraints

QEC scheduling may need:

ancilla readiness
syndrome extraction
stabilizer interaction ordering
measurement readiness
classical decoder readiness
round boundaries
feedback
communication

These MUST be represented explicitly.


---

54. No hard-coded QEC topology

Scheduling MUST NOT assume:

surface code distance = 3

or:

four neighbors

or:

fixed ancilla count

or:

fixed number of rounds

or any other particular QEC architecture.

Those values belong to QEC configuration and target data.


---

55. Stabilizer scheduler compatibility

stabilizer_scheduler.rs MUST NOT become the generic scheduling engine.

Its long-term semantic role is:

stabilizer/QEC request
    |
    v
QEC scheduling adapter
    |
    v
generic scheduler

It may remain as a compatibility API while the QEC architecture migrates.

It MUST NOT maintain a second independent scheduling semantics.


---

56. Dynamic circuit semantics

Zamani scheduling MUST support programs in which future operations depend on runtime information.

Examples:

measure
conditional gate
feedback
repeat-until-success
runtime decision

The scheduler distinguishes:

compile-time known timing

from:

runtime-resolved timing

The latter MUST NOT be falsely represented as statically known.


---

57. Runtime scheduling

dynamic/runtime.rs may represent activities whose exact start time cannot be known until runtime.

For example:

measurement
    |
    v
classical processing
    |
    v
conditional decision
    |
    v
operation

The scheduler establishes the dependency and legal timing constraints.

The runtime determines the actual event timing when execution information becomes available.


---

58. Feedback semantics

Feedback is:

quantum event
    ->
classical result
    ->
classical processing
    ->
control decision
    ->
quantum event

Each boundary must be represented.

The scheduler MUST NOT assume zero classical latency unless the target explicitly guarantees it.


---

59. Distributed quantum semantics

The scheduler MUST be capable of representing distributed execution.

A distributed system may contain:

nodes
modules
links
quantum channels
classical channels
communication resources
entanglement resources

The scheduler MUST treat communication as a schedulable activity when its timing/resources affect execution.


---

60. Distributed operations

Distributed operations may require:

entanglement generation
teleportation
remote gate
classical communication
synchronization
resource reservation

The scheduler MUST preserve the dependencies between these activities.


---

61. Distributed scheduling does not mean independent scheduling

Independent local scheduling is not equivalent to globally valid distributed scheduling.

The global schedule must account for:

cross-node dependencies
communication latency
link capacity
synchronization
resource contention

The architecture may use:

global scheduler
+
local planners

but the final result must satisfy global constraints.


---

62. Communication semantics

Communication resources are first-class resources when they affect execution.

A communication activity may have:

source
destination
duration
capacity
availability
latency
ordering
dependency

No fixed number of links may be assumed.


---

63. Scheduler transformations

Transformations may include:

delay insertion
alignment
padding
dynamical decoupling

Transformations are distinct from core scheduling decisions.

A transformation MUST be explicitly enabled or required by the target/policy.


---

64. Delay semantics

A delay represents intentional idle time.

A scheduler-generated delay MUST preserve computation semantics.

It may be used to:

align operations
reserve timing
represent idle periods
satisfy control requirements

Delay insertion MUST NOT violate:

resource constraints
timing constraints
deadlines
QEC constraints


---

65. Dynamical decoupling

Dynamical decoupling is not fundamental scheduling semantics.

It is a target/policy-dependent transformation.

It MUST be implemented independently from the core scheduler.

The scheduler may request or apply it only when explicitly authorized.


---

66. Verification semantics

A production schedule is not valid merely because a planner produced it.

It MUST pass verification.

Verification must include, as applicable:

structural verification
dependency verification
resource verification
timing verification
semantic verification

The result is successful only after required verification succeeds.


---

67. Structural verification

Structural verification checks:

all required source operations represented
no forbidden duplicate
no missing operation
valid operation identities
valid references
valid provenance

Inserted scheduler artifacts must be distinguishable from source operations.


---

68. Dependency verification

For every dependency:

A -> B

the verifier checks the applicable semantic ordering.

Ordinary precedence:

finish(A) <= start(B)

must hold.


---

69. Resource verification

For every resource:

usage(t) <= capacity(t)

must hold.

For exclusive resources:

no incompatible overlapping reservations

must hold.

Verification MUST operate independently of planner assumptions.


---

70. Timing verification

Timing verification checks:

start
duration
finish
resolution
alignment
release times
deadlines
availability windows
measurement latency
feedback latency

All arithmetic must remain valid.


---

71. Semantic verification

Semantic verification is the strongest check.

It must establish that scheduling did not silently alter:

operations
operands
conditions
measurements
resets
QEC semantics
source-to-target mapping

Permitted differences include legal timing and scheduler-generated timing artifacts.


---

72. Two-stage schedule publication

The recommended production flow is:

candidate schedule
    |
    v
verification
    |
    +---- failure --> reject
    |
    v
immutable/published schedule

An unverified candidate MUST NOT be exposed as the final successful schedule.


---

73. Partial schedules

A partial schedule MAY exist internally.

It MUST NOT be represented as:

successful final schedule

unless the API explicitly defines a partial/analysis result.

This prevents downstream runtime components from accidentally executing incomplete schedules.


---

74. Schedule result semantics

result.rs MUST provide enough information to determine:

what was scheduled
when it executes
what resources it consumes
why it was selected
whether it was verified

The result SHOULD include:

ScheduleId
operation timings
reservations
makespan
depth
critical path
idle intervals
objective score
verification report
provenance
diagnostics
reproducibility information


---

75. Determinism

When deterministic mode is enabled:

same source
+
same target snapshot
+
same routing result
+
same scheduling configuration
+
same seed

must produce the same schedule.

Tie-breaking MUST therefore be deterministic.

Iteration over unordered collections MUST NOT accidentally introduce nondeterminism.


---

76. Randomized algorithms

Randomized scheduling algorithms MAY exist.

They MUST receive explicit randomness context.

They MUST NOT use hidden process-global randomness.

The random seed SHOULD be recorded in schedule provenance when reproducibility is requested.


---

77. Concurrency semantics

Scheduling implementations MAY use parallel computation.

Parallelism MUST NOT change scheduling semantics.

Concurrent implementations must produce equivalent results under deterministic configuration.

The scheduler MUST avoid global mutable scheduling state.

Scheduler state belongs to the scheduling invocation/context.


---

78. Recursion and enormous graphs

Scheduler graph algorithms MUST NOT depend on recursion depth proportional to arbitrary program size.

Large dependency graphs can exceed the process stack.

Implementations SHOULD prefer:

iterative traversal
explicit work stacks
queues
priority queues
event structures

over unbounded recursion.


---

79. Complexity semantics

Dependency analysis SHOULD target:

O(V + E)

where:

V = operations/nodes
E = dependencies

This is a complexity target, not a guarantee for every scheduling algorithm.

Resource-constrained optimization can be computationally difficult.

The scheduler therefore MUST distinguish:

exact
heuristic
approximate
deterministic
stochastic
adaptive

algorithms.


---

80. Optimality claims

An algorithm MUST NOT report:

optimal

unless the optimization contract actually establishes optimality under the specified model.

Otherwise it should report an appropriate quality classification.

For example:

heuristic
feasible
approximate
best-known
bounded


---

81. Memory scalability

The scheduler MUST avoid dense representations whose size is proportional to:

qubits × total execution time

unless explicitly requested and feasible.

Preferred structures include:

dependency graph
event queues
interval/resource calendars
sparse reservations
lazy ready sets


---

82. Streaming semantics

Large workloads SHOULD support incremental processing where possible.

Examples:

incremental dependency analysis
incremental scheduling
incremental verification
incremental serialization

The implementation must not require full materialization when semantics do not require it.


---

83. Resource exhaustion

A program may be valid but too large for the available resources.

For example:

semantically valid
+
target compatible
+
insufficient host memory

must produce an operational failure, not a semantic reinterpretation.

The scheduler MUST distinguish:

invalid program

from:

valid but currently infeasible


---

84. Explicit execution budgets

limits.rs may provide invocation-level limits such as:

memory budget
CPU budget
deadline
maximum materialization
maximum optimization iterations
maximum diagnostic output
maximum pending events

These are host/execution constraints.

They MUST NOT be used to define Zamani's semantic maximum machine size.


---

85. Serialization semantics

Serialized schedules are data, not executable authority.

Deserialization MUST validate:

schema
identities
operations
timings
resources
dependencies
provenance
target compatibility

before accepting the schedule.

A serialized schedule MUST NOT be trusted merely because it came from a previous process.


---

86. Versioning

Schedule serialization MUST be versioned.

A schedule MUST identify sufficient information to determine:

schema version
scheduler semantic version
target identity/version where required
source/program provenance where permitted
configuration

Breaking semantic changes require an explicit version transition.


---

87. Diagnostics semantics

Diagnostics are observational.

They MUST NOT determine schedule semantics.

diagnostics/ may explain:

why an operation was delayed
which resource blocked it
which dependency blocked it
which policy selected it

Diagnostics MUST be derived from scheduling decisions rather than becoming hidden scheduling inputs.


---

88. Sensitive diagnostics

Diagnostics MAY reveal:

program structure
target topology
calibration information
resource availability
optimization strategy

Therefore diagnostics MUST support an appropriate privacy/redaction policy.

Secrets and credentials MUST never appear in scheduler diagnostics.


---

89. Plugin semantics

Plugins MAY provide:

scheduler algorithms
policies
resource models
optimization strategies
diagnostic extensions

A plugin MUST conform to the scheduler's semantic contracts.

Plugins MUST NOT bypass:

canonical IR
verification
resource validation
timing validation
security policy

unless an explicitly documented trusted boundary says otherwise.


---

90. Plugin isolation

Untrusted external data MUST NOT become executable plugin code automatically.

Plugin registration and execution are separate concepts.

A registry may identify trusted compiled implementations.

It MUST NOT treat arbitrary serialized configuration as executable logic.


---

91. Adapter semantics

Adapters translate between subsystem contracts.

Required adapters include:

adapters::ir
adapters::hardware
adapters::routing
adapters::qec

An adapter MUST:

1. validate source information;


2. translate it;


3. preserve semantic identity;


4. preserve provenance;


5. reject incompatible information;


6. produce scheduler-native representations.



Adapters MUST NOT silently invent missing semantics.


---

92. IR adapter

adapters/ir.rs translates canonical quantum IR into scheduling views.

It MUST preserve:

operation identity
qubit identity
operands
conditions
measurement semantics
reset semantics
provenance

It MUST use:

quantum::ir::qubit::QubitId
quantum::ir::qubit::PhysicalQubitId

where applicable.


---

93. Hardware adapter

adapters/hardware.rs translates hardware target capabilities into scheduler resources/timing.

It may supply:

operation support
durations
timing resolution
alignment
resource capacities
availability
communication
calibration-derived scheduling information

It MUST NOT expose credentials to the scheduler core.


---

94. Routing adapter

adapters/routing.rs consumes the canonical routing result.

It must establish:

logical operation
    ->
physical operands

without implementing routing again.


---

95. QEC adapter

adapters/qec.rs translates QEC requirements into generic scheduling concepts.

It may produce:

dependencies
resource requirements
round constraints
measurement requirements
feedback requirements

Generic scheduling remains responsible for timing/resource coordination.


---

96. No circular dependencies

The intended dependency direction is:

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
hardware/runtime

QEC/ZQN/hardware information enters scheduling through explicit contracts/adapters.

Scheduling MUST NOT create circular ownership such as:

scheduler owns routing
routing owns scheduler

or:

scheduler owns hardware execution
hardware owns scheduler internals


---

97. Source-level semantics versus target semantics

A source-level property is something that must remain true independent of the target.

Examples:

operation meaning
measurement meaning
classical condition meaning
logical qubit identity
program ordering constraints

A target-level property may vary:

physical qubit
duration
channel
alignment
resource capacity
communication latency

Scheduling MUST keep these domains separate.


---

98. Target capability validation

Before scheduling an operation, the system must establish that the target supports the operation.

Capability information may include:

gate support
arity
allowed operands
duration
resources
timing
measurement
reset
feedback
communication

An unsupported operation MUST result in a structured compatibility error.


---

99. Arbitrary gate arity

The scheduler MUST support arbitrary operation arity.

It MUST NOT contain assumptions such as:

if operands.len() == 1

or:

if operands.len() == 2

as the universal semantic model.

Specialized optimizations may recognize common arities, but the generic representation must remain arbitrary.


---

100. Empty programs

An empty program is a valid edge case if accepted by the canonical IR.

Its schedule should have:

zero scheduled operations

and an appropriate makespan according to the defined time-origin/horizon semantics.

It MUST NOT panic.


---

101. Single-operation programs

A one-operation program must be schedulable if:

operation valid
target compatible
resources available
timing valid

This must work regardless of operation arity.


---

102. Large programs

A large program must be handled using the same semantic model.

No separate "large quantum computer" semantics may be introduced.

Scaling should come from:

data structures
algorithms
parallelism
streaming
resource-aware planning

rather than from a second scheduler.


---

103. Topology independence

The scheduler MUST NOT assume:

line
grid
ring
heavy-hex
all-to-all
surface-code lattice

as a universal topology.

Topology is target/routing data.

The scheduler consumes the mapped operations and resource constraints resulting from that target.


---

104. Technology independence

The scheduler MUST be able to represent targets based on:

superconducting
trapped-ion
neutral-atom
photonic
spin
topological
annealing
hybrid
future architectures

without changing the semantic core.

Technology-specific details enter through target capabilities/resources/adapters.


---

105. Calibration semantics

Calibration information may affect:

duration
availability
fidelity
resource constraints
alignment

The scheduler may consume a calibration snapshot.

It MUST NOT mutate the authoritative calibration database.

Calibration ownership remains outside scheduling.


---

106. Schedule validity under changing calibration

If calibration changes after scheduling, the schedule may become stale.

The execution layer MUST determine whether:

schedule remains valid

or:

rescheduling required

The scheduler should retain sufficient target provenance to support that decision.


---

107. Availability windows

Resources may have:

available intervals
maintenance intervals
calibration intervals
disabled intervals
reservation intervals

A schedule may use a resource only during legal availability.

Availability MUST be evaluated using interval semantics rather than fixed-size time-slot arrays.


---

108. Resource reservation semantics

A reservation represents:

resource
operation/activity
start
end
mode

A reservation is valid only if the resource model accepts it.

Reservation creation MUST be atomic from the perspective of the scheduling algorithm.

A failed reservation MUST NOT leave a partially committed resource state.


---

109. Event-driven scheduling

planners/event.rs may use events such as:

operation ready
operation started
operation completed
resource released
measurement result available
communication completed
feedback ready

Events MUST be ordered deterministically where their timestamps are equal and deterministic mode is enabled.


---

110. Equal-time arbitration

When multiple activities are ready at the same time, policy determines which is selected.

Tie-breaking MUST be explicit.

Possible tie-breakers include:

critical-path priority
user priority
operation identity
resource locality
deadline
fidelity

The scheduler MUST NOT depend on incidental hash-map iteration order.


---

111. Semantic equivalence of schedules

Two schedules are semantically equivalent when they execute equivalent operations under equivalent dependencies and conditions, even if their timestamps differ.

For example:

Schedule A:
A at 0
B at 10

Schedule B:
A at 20
B at 30

may be semantically equivalent if no deadline/resource/observable timing property distinguishes them.

Timing-sensitive programs may make timing itself semantically significant.

The verification layer must account for this.


---

112. Timing can be observable

Timing is not always merely an optimization detail.

Timing may affect:

measurement
feedback
QEC
communication
decoherence
pulse behavior
synchronization
deadline
runtime control

Therefore the scheduler MUST treat timing constraints as potentially semantic.

It must never assume:

all delays are harmless


---

113. Explicit idle periods

Idle time MAY be represented implicitly by gaps.

It MAY also be materialized as explicit delay operations.

The choice MUST be controlled by the schedule representation/target contract.

If delays are materialized, they must preserve the schedule's timing and resource semantics.


---

114. Semantic effect of delays

A delay can be semantically meaningful for:

decoherence
phase evolution
synchronization
QEC
measurement
control

Therefore a delay transformation MUST be target-aware.

The scheduler MUST NOT insert arbitrary delays solely because they make a data structure easier to represent.


---

115. Classical execution

Classical processing may be represented as a resource/activity when its latency affects quantum scheduling.

Examples:

measurement decoding
feedback computation
control decision
network coordination

The scheduler must not assume classical computation is instantaneous unless guaranteed by the target.


---

116. Measurement-to-feedback latency

For:

measurement
    ->
classical processing
    ->
feedback

the minimum latency must be obtained from the target model.

No universal zero-latency assumption is permitted.


---

117. Communication latency

For distributed execution:

send
    ->
network
    ->
receive

latency and resource constraints are target/network properties.

They must be represented explicitly when they affect schedule validity.


---

118. QEC round semantics

A QEC round is a semantic grouping supplied by the QEC layer.

Scheduling MAY use round boundaries as constraints.

It MUST NOT assume a universal number of operations or qubits per round.


---

119. QEC feedback

If QEC requires feedback:

syndrome
    ->
measurement
    ->
classical result
    ->
decoder/decision
    ->
correction

the scheduler must preserve the dependency chain.

It must not schedule correction before the required result is available.


---

120. Security boundary

Security requirements are complementary to these semantics.

Scheduling MUST:

validate inputs
check arithmetic
avoid unsafe Rust
avoid hidden resource limits
avoid secrets
avoid arbitrary execution
verify output

A security failure MUST NOT be converted into a semantic success.


---

121. Safe Rust requirement

The scheduling subsystem MUST use safe Rust.

No scheduling source may use:

unsafe

including:

unsafe {}
unsafe fn
unsafe impl
unsafe trait

The implementation should enforce this at the strongest applicable crate/workspace lint level.

The semantic model assumes memory-safe implementation.


---

122. No hidden machine assumptions

Forbidden:

MAX_QUBITS
MAX_GATES
MAX_CHANNELS
MAX_ROUNDS
MAX_DEPTH
DEFAULT_DEVICE_TOPOLOGY
DEFAULT_GATE_DURATION
DEFAULT_CONTROL_CHANNEL_COUNT

when these are treated as universal semantic truths.

Defaults MAY exist only when they are explicit configuration defaults and do not masquerade as target capabilities.


---

123. Configuration semantics

config.rs defines invocation policy.

It may specify:

algorithm
policy
objective
determinism
seed
verification level
optimization
parallelism
resource budgets
deadline
diagnostics

Configuration MUST NOT mutate global scheduler behavior.


---

124. Context semantics

context.rs represents the immutable scheduling environment.

It should contain references/snapshots for:

program
routing result
target
resources
timing
constraints
policy
objective
QEC information
ZQN information
cancellation/deadline
reproducibility

The context is the bridge between independent scheduler files.


---

125. Error semantics

Scheduling errors MUST be structured.

Important classes include:

InvalidInput
InvalidDependencyGraph
CycleDetected
UnsupportedOperation
MissingDuration
InvalidDuration
ResourceUnavailable
ResourceConflict
TimingConflict
AlignmentViolation
ConstraintViolation
Unschedulable
CapacityExceeded
DeadlineExceeded
ResourceLimitExceeded
Cancelled
VerificationFailed
SerializationError
PluginError

Error strings MUST NOT be used as machine-readable API contracts.


---

126. Unschedulable versus invalid

These are different.

Invalid:

program violates semantic rules

Unschedulable:

program is valid
+
target/context cannot satisfy required constraints

Example:

valid operation
+
required resource unavailable
=
unschedulable

not:

invalid quantum program


---

127. Target-incompatible versus resource-unavailable

These are also different.

Target-incompatible:

device cannot perform operation

Resource-unavailable:

device can perform operation
but required resource cannot be allocated under current conditions

The error hierarchy should preserve this distinction.


---

128. Verification failure

If a planner generates an invalid schedule:

VerificationFailed

must be returned.

The system MUST NOT silently repair the schedule unless the repair is an explicitly defined transformation stage.


---

129. No silent semantic repair

The scheduler MUST NOT silently:

remove an operation
change an operand
change a gate
change a condition
change a measurement
change a QEC operation
invent a resource
invent a duration

to make scheduling succeed.

Such behavior would violate:

scheduled semantics == source semantics


---

130. Explicit transformations

If the compiler wants to:

decompose
route
rewrite
synthesize
insert correction

those transformations belong to their owning subsystems.

Scheduling transformations are limited to explicitly defined timing/resource transformations.


---

131. Optimization boundary

Scheduling optimization may reorder independent operations.

It MUST NOT reorder dependent operations.

If:

A -> B

then a scheduling optimization cannot place:

B before A

unless the dependency itself has been legitimately transformed upstream.


---

132. Commutation

A scheduler may exploit commutation only if the semantic contract establishes that the operations can be reordered.

It MUST NOT assume all independent-looking operations commute.

Quantum semantic independence and graph independence are not necessarily identical.


---

133. Resource dependencies versus semantic dependencies

Two operations may have no quantum data dependency but still conflict because:

same resource

Therefore scheduling dependencies may be introduced by resource constraints.

These are scheduler constraints, not necessarily source-level quantum dependencies.


---

134. Reservation-induced ordering

If resource arbitration chooses:

A before B

because both require the same exclusive resource, the scheduler must record enough information to explain this ordering.

This supports:

diagnostics
verification
reproducibility


---

135. Objective determinism

If two schedules have equal objective score, the scheduler must use a deterministic tie-breaker when deterministic mode is enabled.

This prevents mathematically equivalent schedules from becoming nondeterministic merely because of collection ordering.


---

136. Reproducibility

A production schedule SHOULD record:

source/program identity
target snapshot identity
routing identity
scheduler version
configuration identity
algorithm
policy
seed
relevant target metadata

Sensitive information must be redacted according to security policy.


---

137. Schedule identity

A ScheduleId identifies a particular schedule artifact.

It does not become a quantum program identity.

Schedule identity SHOULD be derived from the schedule/provenance context rather than arbitrary hidden global state.


---

138. Benchmarking integration

The scheduler MUST expose metrics sufficient for benchmarking.

At minimum:

makespan
depth
idle time
resource utilization
critical path
operation count
reservation count
communication overhead
optimization score
verification time
planning time

Benchmarking consumes these metrics.

Scheduling does not own benchmark methodology.


---

139. Diagnostics integration

Diagnostics may explain:

operation delayed because predecessor
operation delayed because resource
operation delayed because alignment
operation delayed because communication
operation delayed because measurement
operation selected because critical path

Diagnostics must be observational and reproducible.


---

140. Testing semantics

Every semantic invariant MUST have tests.

Tests are divided into:

unit
integration
property
regression
scalability
determinism


---

141. Required semantic test categories

At minimum:

empty program
single operation
single qubit
multiple qubits
arbitrary arity
parallel independent operations
dependent operations
resource conflicts
resource capacities
measurement
reset
conditional execution
feedback
communication
QEC
dynamic scheduling
alignment
deadlines
release times
invalid duration
cycle
unsupported operation
unavailable resource
large dependency graph
deterministic scheduling
serialization round trip


---

142. Property invariants

Property tests SHOULD establish:

no dependency violation
no resource-capacity violation
no invalid timing
no missing source operation
no duplicate semantic operation
no invalid qubit identity
no schedule overflow


---

143. Scalability tests

Scalability tests MUST vary:

number of operations
number of qubits
number of resources
dependency density
parallelism
QEC rounds
distributed nodes
communication events

They MUST NOT stop at an arbitrary architecture constant merely because the test author selected one.

Test budgets may be finite.

Semantic limits must not be.


---

144. Fuzzing

The scheduler should be fuzz-tested with malformed:

IR
graphs
timings
resources
constraints
serialized schedules
target descriptions
QEC metadata
distributed topologies

The required property is:

malformed input
    ->
structured rejection

not:

panic
undefined behavior
silent corruption


---

145. Panic policy

Production scheduling code SHOULD avoid panics for user-controlled or target-controlled invalid data.

Expected invalid states must return structured errors.

Assertions may be used for internal invariants that indicate programmer bugs, but external invalid input must not be converted into process-terminating behavior where a recoverable error is possible.


---

146. Integer representation

Semantic IDs and quantities must use types appropriate to their domain.

usize may be used for actual host collection indexing.

Conversions to usize MUST be checked.

A semantic count MUST NOT become an unchecked allocation size.


---

147. No unsafe FFI assumptions

Scheduling must remain independent of:

C ABI
raw pointers
FFI memory ownership
unsafe vendor libraries

Vendor integration belongs outside the scheduling core.


---

148. Thread safety

Scheduler state SHOULD be safe to move/share according to the actual API requirements.

The core must not depend on:

global mutable singleton
global current target
global current schedule
global current random seed

Each invocation owns its semantic state.


---

149. Cancellation semantics

Cancellation means:

the caller no longer requires completion of this scheduling invocation.

Cancellation MUST NOT imply:

partial result is valid

unless explicitly returned as an analysis artifact.


---

150. Deadline semantics

When the scheduler cannot complete before its configured deadline:

DeadlineExceeded

must be returned.

A deadline must never cause:

invalid schedule

to be returned as successful output.


---

151. Resource policy semantics

Resource policies determine what the host permits the scheduler to consume.

They do not change quantum program semantics.

For example:

max_memory = 8 GiB

means:

this invocation may use at most 8 GiB

not:

Zamani quantum programs cannot exceed 8 GiB


---

152. Host-resource exhaustion

If the process cannot obtain required memory/CPU resources, the scheduler should fail cleanly where the host environment allows it.

It must not use unsafe memory tricks to bypass resource limits.


---

153. Serialization/deserialization trust boundary

Deserialization is untrusted until validation completes.

The sequence must be:

bytes
 |
 v
schema validation
 |
 v
structural validation
 |
 v
semantic validation
 |
 v
target compatibility
 |
 v
trusted schedule representation


---

154. Schedule execution trust boundary

A schedule must not be sent to runtime/hardware merely because it was successfully decoded.

The execution pipeline must ensure:

decoded
+
validated
+
target-compatible
+
verified

before execution.


---

155. Runtime boundary

Scheduling ends at a valid execution plan.

Runtime owns:

device connection
execution
job submission
monitoring
cancellation
hardware responses

The scheduler does not become a runtime client.


---

156. Hardware authentication boundary

Credentials, tokens, certificates, and provider authentication MUST remain outside scheduling.

The scheduler should see capability/resource information, not secrets.


---

157. Distributed security boundary

Distributed scheduler metadata may be sensitive.

The scheduler must treat remote target/resource descriptions as untrusted data.

Authentication and encrypted transport belong to the distributed/runtime infrastructure.

Scheduling is responsible for semantic validation of received scheduling information.


---

158. Target identity

A schedule should identify the target snapshot sufficiently to prevent accidental execution against an incompatible target.

At minimum, the execution integration should be able to determine:

which target
which target version/snapshot
which capabilities
which timing model

were assumed.


---

159. Stale schedules

A schedule may become stale when:

calibration changes
resource availability changes
target topology changes
hardware capability changes
network changes

The runtime/hardware layer must determine whether the schedule remains valid.

The scheduler SHOULD expose target provenance to support that decision.


---

160. Schedule portability

A schedule is target-specific.

A Zamani source program is target-independent.

Therefore:

source portability

does not imply:

serialized schedule portability across arbitrary machines

A schedule generated for target A MUST NOT automatically be executed on target B.


---

161. Schedule re-targeting

To execute the same source program on another target:

source/canonical IR
    ->
target-specific routing
    ->
target-specific scheduling

should occur again.

This preserves the write-once program model.


---

162. Schedule cache semantics

If schedules are cached, the cache key MUST account for all semantics that affect scheduling, including as applicable:

source/program identity
target snapshot
routing result
scheduler version
policy
configuration
objective
seed
relevant calibration

A stale cache entry MUST NOT silently replace a newly required schedule.


---

163. Compatibility

Scheduling compatibility is based on semantic contracts.

A new scheduler implementation is compatible if it preserves:

source semantics
dependency semantics
resource semantics
timing semantics
result contract

Algorithmic improvements may produce different legal schedules without breaking semantic compatibility.


---

164. Algorithm replacement

The scheduler architecture must permit replacing:

ASAP
ALAP
list
critical-path
resource-constrained
adaptive

implementations without changing:

quantum IR
hardware contracts
routing contracts
QEC contracts
runtime contracts

This is why planners, policies, and algorithms are separated.


---

165. Policy versus algorithm

A policy answers:

> What should be preferred?



An algorithm answers:

> How do we construct the schedule?



For example:

policy = critical-path priority
algorithm = list scheduling

These concepts must not be conflated.


---

166. Planner versus algorithm

A planner owns the orchestration of scheduling decisions.

An algorithm may provide a particular scheduling strategy.

The planner must consume common:

operations
dependencies
resources
timing
constraints

rather than creating a second representation.


---

167. Constraint engine semantics

A constraint must be able to answer:

is candidate legal?

and preferably:

why is it illegal?

Constraints must be composable.

A new target-specific constraint should be implementable without modifying every scheduler algorithm.


---

168. Constraint ordering

Constraint evaluation order may be optimized.

The final semantic result must be identical.

A fast-failing constraint may run before a slower constraint, but this must not change whether the candidate is actually valid.


---

169. Constraint diagnostics

Constraint violations should identify enough context to explain:

operation
resource
time
constraint
reason

Diagnostics must not require parsing human-readable error strings.


---

170. Resource calendar semantics

resources/calendar.rs represents time-varying resource occupancy.

It should support:

reservation insertion
reservation removal where explicitly permitted
overlap queries
availability queries
capacity queries

The calendar must not assume a fixed timeline length.


---

171. Event calendar scalability

Large schedules should use sparse event/resource structures.

The scheduler MUST NOT allocate one object for every theoretical time unit.

This allows:

long idle periods
short operations
huge schedules

without making memory proportional to wall-clock duration.


---

172. Time origin

The scheduling system must define a consistent time origin.

A common semantic choice is:

t = 0

at the beginning of the scheduled execution window.

The actual hardware timestamp is supplied by runtime/execution.

The scheduler must not confuse logical schedule time with wall-clock system time.


---

173. Wall-clock independence

Scheduling should normally operate on logical execution time.

It must not use:

SystemTime::now()

as an implicit scheduling semantic input.

If real-time scheduling is required, that fact must be explicitly represented in the dynamic/runtime contract.


---

174. Static scheduling

Static scheduling means all required timing information is known sufficiently before execution.

The scheduler can produce:

complete operation timeline

subject to target assumptions.


---

175. Dynamic scheduling

Dynamic scheduling means some decisions depend on runtime events.

The scheduler may produce:

partial static plan
+
runtime decision points

rather than pretending all timing is known.


---

176. Adaptive scheduling

Adaptive scheduling may respond to:

target state
resource state
noise information
runtime feedback

but adaptive behavior must remain inside an explicit policy/algorithm contract.

It must not create hidden side effects in the scheduling core.


---

177. Semantic boundary of adaptive scheduling

An adaptive scheduler may change:

future timing
future resource allocation
future strategy

if the program explicitly permits adaptation.

It MUST NOT change an unconditional program operation into a different semantic operation without authorization.


---

178. Resource locality

Resource-aware scheduling may prefer operations that reduce:

communication
movement
routing overhead

but routing remains the authority over physical placement.

Scheduling may use placement information.

It must not silently remap qubits.


---

179. Communication-aware scheduling

Scheduling may coordinate communication and computation:

communication
    |
    v
resource availability
    |
    v
remote operation

The schedule must preserve the communication dependency.


---

180. Quantum network scheduling

For quantum-network targets, resources may include:

entanglement links
repeaters
memory
classical channels
synchronization

No network topology is hard-coded.


---

181. Resource hierarchy and distributed systems

The same resource model must represent:

qubit
chip
module
QPU
node
cluster
network

when those entities constrain execution.

The model should therefore support hierarchical resources without introducing architecture-specific assumptions.


---

182. Semantic scaling model

The intended scaling hierarchy is:

atom
  |
  v
qubit
  |
  v
small QPU
  |
  v
large QPU
  |
  v
multi-QPU
  |
  v
quantum data center
  |
  v
quantum network
  |
  v
future distributed quantum infrastructure

The scheduler's semantic model remains the same.

Only target/resource/context information changes.


---

183. No second scheduler for large systems

The project MUST NOT introduce:

small_scheduler
large_scheduler
distributed_scheduler

as unrelated semantic systems.

There may be different algorithms/planners for scalability.

They must implement the same scheduler contracts.


---

184. Parallel scheduling semantics

Parallel algorithms may partition work.

The partitioning MUST preserve:

dependency correctness
resource correctness
timing correctness
deterministic arbitration

where deterministic mode is requested.


---

185. Partitioning

A large graph may be partitioned for analysis.

Partitioning MUST NOT cut semantic dependencies without representing the cross-partition dependency explicitly.


---

186. Distributed scheduler implementation

A distributed implementation may use:

global coordinator
local schedulers

or another architecture.

All local decisions must remain subordinate to the global semantic contract when operations cross boundaries.


---

187. Failure semantics

If any mandatory phase fails:

validation
planning
reservation
transformation
verification

the final schedule is unsuccessful.

Failure must propagate as a structured error.


---

188. No partial commit

A scheduler must not publish half-completed resource reservations as if scheduling succeeded.

Internal transactional behavior should ensure:

candidate state
    |
    v
verify
    |
    v
commit/publish

rather than exposing inconsistent intermediate state.


---

189. Immutability after publication

Once a schedule is published as a successful result, it SHOULD be immutable.

If modification is required:

old schedule
    ->
new scheduling transformation
    ->
new verified schedule

rather than hidden mutation.


---

190. Semantic equality

Two scheduling structures should be distinguishable between:

structural equality

and:

semantic equivalence

Structural equality means identical representation.

Semantic equivalence means equivalent computation/timing/resource behavior under the defined model.

The verifier should use the appropriate notion for each check.


---

191. Provenance of inserted operations

Scheduler-generated operations must identify their cause.

Examples:

Delay
AlignmentPadding
DynamicalDecouplingPulse
Synchronization
Communication

The provenance must say:

generated by scheduling transformation

and, where possible:

reason
source operation(s)
constraint
policy


---

192. No accidental source mutation

Scheduling adapters should preferably operate on views/copies/immutable references according to the repository's IR ownership model.

Scheduling must not unexpectedly mutate canonical source IR merely to construct a schedule.

If a transformed IR is required, that transformation must be explicit.


---

193. Compiler integration

The compiler pipeline should invoke scheduling after the program has sufficient target information.

Expected flow:

frontend
    ->
canonical IR
    ->
optimization
    ->
routing
    ->
scheduling
    ->
verification
    ->
hardware lowering

The exact compiler orchestration belongs outside scheduling.


---

194. Runtime integration

Runtime receives a verified target-compatible schedule.

Runtime must not assume that every schedule is executable without checking target identity/capability contracts.


---

195. Hardware integration

Hardware adapters provide:

target snapshot
resource model
timing model
capabilities
availability

Scheduling returns:

resource/time plan

Hardware execution translates that plan into executable target instructions.


---

196. QEC integration

QEC provides:

fault-tolerant operation structure
constraints
round information
measurement dependencies
feedback

Scheduling provides:

when
where resources are occupied in time

Routing provides:

where physically


---

197. ZQN integration

ZQN provides:

uncertainty
noise
fidelity
drift

Scheduling may use these as:

constraint
objective
cost

without duplicating ZQN semantics.


---

198. Benchmark integration

Benchmarking should be able to ask:

how long?
how much parallelism?
how much idle time?
how much resource utilization?
what fidelity estimate?
how much communication?

without knowing scheduler implementation details.


---

199. API stability

Public scheduling APIs should expose semantic concepts, not implementation details.

Prefer:

Schedule
SchedulingContext
SchedulingConfig
SchedulingPolicy
SchedulingResult
Resource
TimePoint
Duration
Constraint

over exposing:

internal heap layout
internal graph storage
internal planner queues


---

200. Public API ownership

Each public type must have one authoritative owner.

For example:

QubitId -> quantum::ir::qubit

not:

quantum::ir::qubit::QubitId
quantum::scheduling::QubitId
quantum::hardware::QubitId

This rule prevents type fragmentation.


---

201. Rust version

The scheduling subsystem is designed for:

Rust 1.97.1
Rust 2021 edition

It must not require nightly-only features.

The implementation must use only stable language/library functionality available under the selected project baseline.


---

202. Safe arithmetic requirement

Scheduling calculations must use checked arithmetic where overflow could invalidate semantics.

This includes:

time
duration
counts
indices
capacities
memory estimates
serialization lengths
optimization iterations


---

203. Floating-point semantics

If floating-point values are used for scheduling objectives or estimates:

NaN
infinity
negative zero
rounding
comparison

must be explicitly handled.

Exact physical timing SHOULD prefer integer/rational representations when practical.


---

204. Physical timing precision

The scheduler must preserve enough precision for the target's timing contract.

It must not silently round a timing value in a way that violates:

alignment
duration
deadline
resource occupancy


---

205. Symbolic timing

Symbolic timing may be represented when the target or program requires it.

Before hardware execution, every symbol that must be concrete must be resolved or the execution layer must explicitly support runtime resolution.

The scheduler must not silently replace unresolved timing with arbitrary constants.


---

206. Timing uncertainty

A duration may be represented as:

interval
distribution
estimate

when supported.

A scheduler using uncertain timing must define whether it optimizes:

expected
worst-case
best-case
confidence-bound

behavior.

The choice must be explicit.


---

207. Resource uncertainty

Likewise, resource availability may be uncertain.

The scheduler policy must explicitly choose:

conservative
probabilistic
adaptive
runtime-resolved

handling.

Unknown must not silently become available.


---

208. Semantic compatibility with future hardware

New hardware types must be integrated by adding target/resource/timing adapters rather than rewriting scheduling semantics.

If a future machine introduces a novel resource:

new resource kind

should be added through the resource model.

The core scheduler should remain generic.


---

209. Future-proof operation model

A scheduling operation must be able to represent future quantum operations without requiring the scheduler to understand their physical implementation.

The generic representation should focus on:

identity
operands
dependencies
resources
duration
conditions
timing
provenance


---

210. Unknown operations

An unknown operation may be carried through the scheduling representation if its target/resource/timing contract is available.

If execution requires semantics that cannot be established, scheduling must fail explicitly.


---

211. Semantic extension mechanism

Future semantic requirements should be represented using explicit extension points:

custom constraints
custom resources
custom timing constraints
plugins
metadata
adapters

Extensions must not bypass canonical validation.


---

212. No metadata-driven semantic corruption

Arbitrary metadata must not override authoritative:

qubit identity
operation semantics
resource identity
timing constraints
target capabilities

Metadata is not automatically authoritative.


---

213. Schedule explanations

diagnostics/explain.rs should make decisions explainable.

For example:

Operation O42 starts at T100 because:

- predecessor O17 completes at T80;
- resource R7 is occupied until T100;
- target alignment requires T100;
- policy selected O42 before O43.

The explanation is derived from actual scheduler state.


---

214. Auditability

A production schedule should be auditable.

Given:

source
target snapshot
configuration
algorithm
seed

the project should be able to reconstruct or explain the schedule where deterministic mode applies.


---

215. Security-sensitive provenance

Provenance must not expose secrets.

It may identify:

target ID
configuration hash
algorithm
schedule ID

but must not contain:

credentials
tokens
private keys
authentication material


---

216. Scheduling contract for every implementation file

Every implementation file in this subsystem MUST have:

Inputs
Outputs
Invariants
Errors
Dependencies
Thread-safety contract
Ownership rules
Scalability expectations
Determinism behavior
Integration boundary

This is necessary to satisfy the project requirement that a file can be completed without later semantic rework merely because another subsystem is added.


---

217. File contract: types.rs

Owns:

scheduler identity types
schedule-local scalar types
semantic wrappers

Must not own:

hardware semantics
qubit identity
routing
algorithms

Integration is through public scheduler types.


---

218. File contract: errors.rs

Owns all scheduler errors.

Every downstream file uses it rather than inventing independent error enums.


---

219. File contract: limits.rs

Owns explicit execution/resource budgets.

It must never define artificial quantum-machine limits.


---

220. File contract: context.rs

Owns the immutable input environment.

It integrates:

IR
routing
hardware
ZQN
QEC
policy
constraints
resources
timing

through stable adapter contracts.


---

221. File contract: config.rs

Owns user/invocation scheduling configuration.

It must not own target capabilities.


---

222. File contract: result.rs

Owns the final schedule artifact and its metrics/provenance/verification.

It must not own runtime execution.


---

223. File contract: ir/

Owns scheduler-specific views and dependency structures.

It must not redefine canonical quantum semantics.


---

224. File contract: resources/

Owns generic resource accounting and reservations.

It must not know vendor-specific device APIs.


---

225. File contract: timing/

Owns generic temporal arithmetic and constraints.

Target-specific timing enters through context/adapters.


---

226. File contract: policies/

Owns scheduling preference.

Policies cannot violate semantic constraints.


---

227. File contract: planners/

Owns schedule construction orchestration.

Planners use common IR/resource/timing contracts.


---

228. File contract: constraints/

Owns composable legality rules.

Constraints must be independently testable.


---

229. File contract: transformations/

Owns explicit schedule transformations.

Transformations must be verified after application.


---

230. File contract: verification/

Owns independent validation of produced schedules.

Verification must not trust planner assumptions.


---

231. File contract: optimization/

Owns schedule objective calculations.

Optimization must select among legal schedules.


---

232. File contract: qec/

Owns scheduling-facing QEC structures.

It must not become the QEC decoder.


---

233. File contract: dynamic/

Owns runtime-dependent scheduling semantics.

It must distinguish static from runtime-resolved timing.


---

234. File contract: distributed/

Owns distributed scheduling abstractions.

It must not hard-code a network topology.


---

235. File contract: adapters/

Owns subsystem translation.

Adapters are the only intended location for detailed external representation knowledge.


---

236. File contract: serialization/

Owns versioned schedule persistence.

It must validate before trust.


---

237. File contract: diagnostics/

Owns observation and explanation.

It must not make scheduling decisions.


---

238. File contract: algorithms/

Owns concrete scheduling algorithms.

Algorithms consume stable planner/resource/IR contracts.


---

239. File contract: plugins/

Owns extension registration and plugin interfaces.

Plugins must conform to the same scheduler semantic contract.


---

240. File contract: stabilizer_scheduler.rs

Owns backward compatibility for the previous stabilizer-specific API.

It must delegate to the generic scheduling/QEC architecture.

It must not contain a second generic scheduler.


---

241. mod.rs contract

mod.rs is the composition/public-export boundary.

It should contain:

module declarations
documentation
public re-exports

It should not contain substantial scheduling algorithms.


---

242. Semantic integration graph

The completed subsystem should integrate as:

┌──────────────┐
                    │ quantum::ir  │
                    └──────┬───────┘
                           |
                           v
                    ┌──────────────┐
                    │ optimization │
                    └──────┬───────┘
                           |
                           v
                    ┌──────────────┐
                    │   routing    │
                    └──────┬───────┘
                           |
                           v
                 ┌────────────────────┐
                 │     scheduling     │
                 │                    │
                 │ dependencies       │
                 │ resources         │
                 │ timing             │
                 │ constraints        │
                 │ policies           │
                 │ planners           │
                 │ verification       │
                 └───────┬────────────┘
                         |
             ┌───────────┼───────────┐
             |           |           |
             v           v           v
           QEC          ZQN       hardware
             |           |           |
             └───────────┼───────────┘
                         |
                         v
                       runtime

No circular semantic ownership is permitted.


---

243. Final semantic invariant set

A production schedule MUST satisfy all applicable invariants:

1. Every required source operation is represented.

2. No forbidden semantic operation is introduced.

3. Canonical qubit identities are preserved.

4. Logical-to-physical mapping is not silently changed.

5. Every required dependency is satisfied.

6. Every resource capacity constraint is satisfied.

7. Every exclusive-resource conflict is absent.

8. Every duration is valid.

9. Every time calculation is overflow-safe.

10. Every alignment requirement is satisfied.

11. Every release time is respected.

12. Every deadline is respected when required.

13. Every measurement consumer waits for result readiness.

14. Every classical feedback dependency is respected.

15. Every communication dependency is respected.

16. Every QEC scheduling dependency is respected.

17. Every distributed scheduling constraint is respected.

18. Every target capability requirement is satisfied.

19. No hidden machine-size limit exists.

20. No unsafe Rust is required.

21. No hidden global mutable scheduling state exists.

22. Deterministic mode is reproducible.

23. The final schedule passes required verification.

24. The final schedule remains traceable to source semantics.

25. A failed schedule is never represented as successful execution input.


---

244. Canonical execution equation

The conceptual scheduler function is:

Schedule =
    S(
        Program,
        Target,
        Routing,
        Resources,
        Timing,
        Constraints,
        Policy,
        Objective,
        ExecutionLimits
    )

where:

Program

defines what must happen.

Target

defines what the machine can provide.

Routing

defines where operations execute.

Resources

define what may execute concurrently.

Timing

defines when execution is legal.

Constraints

define additional legality requirements.

Policy

defines preference.

Objective

defines optimization goals.

ExecutionLimits

define how much host computation may be consumed.


---

245. What the scheduler is allowed to change

The scheduler may change:

operation start time
operation finish time
legal ordering of independent operations
resource assignment within the supplied resource model
idle intervals
alignment padding
explicit delay representation
scheduling-only transformations

only where those changes are permitted by the relevant contracts.


---

246. What the scheduler is never allowed to change silently

The scheduler may never silently change:

program meaning
canonical qubit identity
logical-to-physical mapping
gate semantics
measurement semantics
classical condition semantics
QEC semantics
source operation operands
target capability meaning


---

247. Production completion definition

src/quantum/scheduling/ is semantically production-ready only when:

[ ] canonical IR integration works
[ ] canonical QubitId/PhysicalQubitId are used
[ ] no duplicate quantum identity exists
[ ] dependency semantics are implemented
[ ] resource semantics are implemented
[ ] timing semantics are implemented
[ ] arbitrary operation arity is supported
[ ] static scheduling works
[ ] dynamic scheduling works
[ ] distributed scheduling model exists
[ ] QEC integration works
[ ] ZQN integration works
[ ] routing integration works
[ ] hardware integration works
[ ] ASAP works
[ ] ALAP works
[ ] list scheduling works
[ ] resource-constrained scheduling works
[ ] adaptive scheduling has a stable contract
[ ] explicit delay semantics work
[ ] alignment works
[ ] verification works
[ ] semantic preservation is tested
[ ] deterministic mode works
[ ] serialization is versioned
[ ] diagnostics are explainable
[ ] plugins obey the semantic contract
[ ] scalability tests pass
[ ] property tests pass
[ ] regression tests pass
[ ] no unsafe code exists
[ ] no artificial machine-size limits exist
[ ] checked arithmetic is used
[ ] resource exhaustion is handled explicitly
[ ] cancellation/deadlines are handled
[ ] target provenance exists
[ ] final schedules are verified before execution


---

248. Definition of done for each individual file

A scheduler file is considered complete only when:

1. Its semantic responsibility is explicitly defined.

2. Its public types are final for the current contract.

3. Its inputs are explicitly defined.

4. Its outputs are explicitly defined.

5. Its error behavior is defined.

6. Its ownership model is defined.

7. Its thread-safety behavior is defined.

8. Its deterministic behavior is defined.

9. Its scalability behavior is defined.

10. Its integration dependencies are defined.

11. It uses canonical IR identities where required.

12. It does not duplicate another subsystem's ownership.

13. It has unit/property/regression coverage appropriate to its role.

14. It does not require later semantic redesign merely because another
    scheduling file is implemented.

15. It is compatible with the stable scheduler contracts defined here.


---

249. Final architecture principle

The scheduler is not a collection of gate-specific timing hacks.

It is a general execution-planning system.

Its fundamental abstraction is:

WHAT
  |
  | canonical quantum semantics
  v
WHERE
  |
  | routing
  v
WHEN
  |
  | scheduling
  v
HOW
  |
  | hardware lowering
  v
EXECUTE

For Zamani:

write program once
        |
        v
canonical quantum semantics
        |
        +------------------------------+
        |                              |
        v                              v
small target                      enormous target
        |                              |
        v                              v
target-specific routing          target-specific routing
        |                              |
        v                              v
target-specific scheduling       target-specific scheduling
        |                              |
        +---------------+--------------+
                        |
                        v
                    execution

The program remains the source of truth.

The target supplies the resources.

Routing supplies physical placement.

Scheduling supplies temporal/resource coordination.

Verification proves that the resulting schedule is legal.

Runtime executes the verified result.

That separation is the foundation for Zamani's requirement that one quantum program can scale from the smallest machine to arbitrarily large quantum infrastructure without hard-coding the machine into the language or scheduling subsystem.


---

250. Normative closing rule

Whenever a future scheduling feature is proposed, it MUST answer these questions before implementation:

1. What semantic problem does it solve?

2. Which subsystem owns that semantic?

3. Does it preserve:
   scheduled semantics == source semantics?

4. Does it introduce a machine-size assumption?

5. Does it introduce a technology assumption?

6. Does it require a new canonical identity?

7. If so, why can an existing canonical identity not be used?

8. Does it require routing knowledge?

9. Does it require hardware knowledge?

10. Does it require QEC knowledge?

11. Does it require ZQN knowledge?

12. Does it require runtime knowledge?

13. Which file owns the new concept?

14. What are its inputs and outputs?

15. What are its failure modes?

16. How does it scale?

17. How does it behave under resource exhaustion?

18. How is it verified?

19. How is it tested?

20. How is it integrated without changing completed files?

If these questions cannot be answered, the feature is not ready to enter the production scheduling architecture.


---

End of semantic contract

A few important repository-specific corrections are embodied above:

- **No duplicate qubit types:** the repository itself says new code must use `quantum::ir::qubit::QubitId` and `PhysicalQubitId`; scheduling must follow that rather than creating scheduler-local qubit identities. 
- **Scheduling remains distinct from diagnostics:** the existing scheduling diagnostics contract already describes diagnostics as observational rather than decision-making infrastructure. 
- **Scheduling transformations remain separate from core semantics:** the existing design explicitly keeps transformations such as dynamical decoupling outside core scheduling semantics. 
- **The existing QEC adapter direction is retained:** the repository already has a scheduling/QEC adapter intended to preserve QEC-specific identity without polluting generic scheduling semantics. 
- **Checked timing arithmetic is retained as a hard invariant:** the existing ASAP policy already specifies that temporal overflow must become a structured error rather than produce an invalid schedule. 

This `SEMANTICS.md` should therefore serve as the **semantic freeze document** for the implementation files: individual Rust files can implement against these contracts rather than redefining what scheduling means later.