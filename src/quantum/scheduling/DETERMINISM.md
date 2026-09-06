Worked for 9s

Below is the complete production specification for src/quantum/scheduling/DETERMINISM.md. It is written to fit the repository’s current architecture: canonical qubit identity remains under quantum::ir::qubit, routing owns placement, hardware owns target capabilities/timing, and scheduling owns temporal/resource decisions. The repository is already targeting Rust 1.97.1/2021, with no unsafe requirement reflected in the existing scheduling and IR architecture.

Zamani Quantum Scheduling — Determinism Contract

Path: "src/quantum/scheduling/DETERMINISM.md"

Status: Production architecture specification

Target: Rust 1.97 / Rust 1.97.1, Rust 2021, stable Rust

Safety: "unsafe" forbidden

Scope: Deterministic, reproducible, scalable quantum scheduling

---

1. Purpose

This document defines the complete determinism contract for:

crate::quantum::scheduling

Determinism is a cross-cutting property of the scheduler.

It is not an optional implementation detail of one algorithm.

The scheduler must be capable of producing reproducible results when deterministic execution is requested, while also supporting explicitly non-deterministic, randomized, adaptive, parallel, distributed, and runtime-driven scheduling when those modes are requested.

The fundamental contract is:

same semantic program
+
same target snapshot
+
same routing result
+
same hardware/resource/timing description
+
same scheduling configuration
+
same scheduler implementation/version
+
same deterministic seed/state
+
same deterministic execution environment
=
same canonical schedule

When any of those inputs intentionally changes, a different schedule may legitimately result.

Determinism must therefore be defined over all scheduler-relevant inputs, not merely the source program.

---

2. Architectural position

Scheduling remains downstream of the canonical quantum IR.

The authoritative architecture is:

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
scheduling
     │
     ├── dependency analysis
     ├── resource analysis
     ├── timing analysis
     ├── constraint analysis
     ├── policy selection
     ├── planning
     ├── scheduling
     ├── transformation
     └── verification
     │
     ▼
hardware lowering
     │
     ▼
runtime / QPU / simulator / emulator

The repository's canonical IR explicitly defines itself as the semantic "WHAT" of computation and excludes scheduling, routing, hardware selection, and execution from IR ownership.

Routing remains responsible for:

«Where?»

Scheduling remains responsible for:

«When?»

Hardware remains responsible for:

«What can this target execute, and under what physical constraints?»

The existing routing architecture explicitly separates logical-to-physical mapping from scheduling.

The hardware architecture likewise exposes timing, capabilities, topology, calibration, resources, and scheduling constraints to downstream systems rather than embedding those assumptions in the scheduler.

---

3. Meaning of determinism

Determinism means that a scheduling invocation has a well-defined mapping:

SchedulingInput
    │
    ▼
SchedulingConfiguration
    │
    ▼
DeterministicScheduler
    │
    ▼
CanonicalSchedule

For deterministic mode:

D(input) = output

The scheduler must not produce:

D(input) = output_A
D(input) = output_B

merely because:

- a hash map happened to iterate differently;
- threads completed in a different order;
- a parallel worker won a race;
- a priority tie was unresolved;
- a random generator was implicitly seeded;
- resource candidates were enumerated in an unstable order;
- an allocator changed memory addresses;
- a provider response was reordered;
- a collection's insertion order changed;
- an operating-system scheduling decision changed execution order.

If multiple schedules have equal objective value, deterministic mode must define an explicit canonical tie-break order.

---

4. Determinism is not semantic identity

A critical distinction must be maintained:

semantic equivalence
        !=
schedule identity

Two schedules may execute exactly the same computation while having different:

- start times;
- resource selections;
- parallelism;
- routing decisions;
- idle intervals;
- operation ordering where unconstrained;
- optimization scores.

Therefore:

same semantics

does not automatically imply:

same schedule bytes

Deterministic scheduling additionally requires a canonical decision procedure.

---

5. Write once, scale everywhere

Zamani's source program must never contain machine-size assumptions merely to obtain deterministic scheduling.

The following are prohibited:

MAX_QUBITS
MAX_OPERATIONS
MAX_CHANNELS
MAX_RESOURCES
MAX_DEPTH
MAX_SCHEDULE_TIME
DEFAULT_TOPOLOGY
DEFAULT_DEVICE_SIZE
DEFAULT_CHANNEL_COUNT
DEFAULT_QEC_DISTANCE

Deterministic scheduling must work for:

1 qubit
2 qubits
10 qubits
1,000 qubits
1,000,000 qubits
distributed QPUs
quantum networks
future architectures

subject only to actual finite resources.

"Infinity" therefore means:

«The scheduler introduces no artificial finite machine-size ceiling.»

Every concrete compilation remains bounded by:

- available memory;
- address space;
- target resources;
- compilation time;
- operating-system limits;
- explicit caller limits;
- execution deadlines;
- distributed capacity;
- provider limits.

The scheduler must never confuse those practical limits with language-level or architecture-level limits.

---

6. Canonical identity requirements

Scheduler determinism depends critically on stable identities.

The authoritative qubit types are:

crate::quantum::ir::qubit::QubitId
crate::quantum::ir::qubit::PhysicalQubitId

No scheduling module may define another:

QubitId
PhysicalQubitId

The canonical IR explicitly establishes "quantum::ir::qubit" as the authoritative qubit identity implementation.

Therefore scheduling must use:

use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};

where applicable.

Logical and physical identity must never be silently conflated.

QubitId
    │
    │ routing
    ▼
PhysicalQubitId
    │
    │ scheduling
    ▼
timed physical operation

Scheduling must not perform hidden logical-to-physical mapping.

---

7. Operation identity

The canonical quantum IR owns canonical operation identity.

If the repository's canonical identity is:

crate::quantum::ir::core::identity::OperationId

scheduling must consume it rather than creating another semantic operation identity.

Scheduler-specific identities may exist for scheduler artifacts:

ScheduleId
DependencyId
ReservationId
EpochId
DecisionId

but they must never replace the canonical IR operation identity.

This is necessary for:

- reproducibility;
- provenance;
- debugging;
- regression tests;
- schedule comparison;
- serialization;
- benchmark correlation.

---

8. Canonical deterministic ordering

Every scheduler data structure that affects a scheduling decision must have a deterministic ordering.

This includes:

- ready-operation sets;
- dependency sets;
- predecessor lists;
- successor lists;
- resource candidates;
- resource reservations;
- timing windows;
- constraints;
- policy candidates;
- algorithm candidates;
- optimization candidates;
- plugin candidates;
- distributed nodes;
- communication links.

A scheduler must never rely on unspecified iteration order.

For example, this pattern is insufficient:

iterate HashMap
take first candidate

because hash-map iteration order is not a scheduling contract.

Instead, candidates must be explicitly ordered.

---

9. Canonical tie-breaking

Every scheduling algorithm must define what happens when two or more candidates have equal priority.

The tie-breaking hierarchy should be explicit and stable.

Recommended default:

1. hard constraint feasibility
2. earliest feasible start time
3. critical-path priority
4. explicit user priority
5. resource cost
6. operation dependency depth
7. canonical OperationId

The exact hierarchy must be owned by the selected policy.

The important invariant is:

«No equal-priority case may be resolved by incidental iteration order.»

The tie-breaker must be represented as part of the scheduler policy/configuration contract.

---

10. Canonical ordering must be explicit

The scheduler should expose a conceptual deterministic comparator:

compare(candidate_a, candidate_b)

The comparator must produce one of:

Less
Equal
Greater

and must obey:

antisymmetry
transitivity
total ordering where required

If a partial ordering is unavoidable, the scheduler must add a canonical stable tie-breaker before selecting an item.

---

11. Deterministic configuration

"config.rs" must contain explicit deterministic configuration.

Conceptually:

SchedulingConfig
├── deterministic
├── seed
├── tie_break_policy
├── parallelism
├── objective
├── verification
├── limits
├── timing_mode
├── resource_policy
└── diagnostics

Required properties:

- no hidden global configuration;
- no environment-dependent scheduling choices;
- no implicit random seed;
- no system-clock-based algorithmic decisions;
- no memory-address-based ordering;
- no thread-race-based ordering.

---

12. Deterministic mode

Deterministic mode should conceptually be:

deterministic = true

When enabled:

- all random decisions use caller-controlled state;
- all candidate ordering is canonical;
- parallel arbitration is deterministic;
- distributed results are canonically merged;
- serialization ordering is stable;
- diagnostics ordering is stable;
- schedule IDs are reproducible where derived from deterministic input;
- objective evaluation is deterministic;
- verification traversal is deterministic.

---

13. Randomized mode

Randomized scheduling is permitted.

It must never be implicit.

For example:

deterministic = false

may permit:

- randomized candidate selection;
- randomized multi-start algorithms;
- randomized heuristic search;
- stochastic optimization;
- randomized routing/scheduling cooperation.

However:

deterministic = false

must not mean:

«behavior is accidentally nondeterministic.»

It means:

«nondeterminism is explicitly permitted by the caller.»

---

14. Explicit random state

If an algorithm requires randomness, it must receive explicit random state.

Conceptually:

Scheduler
    │
    ├── deterministic mode
    │       └── canonical decision procedure
    │
    └── randomized mode
            └── explicit RNG context

The RNG must not be initialized from:

current time
thread ID
memory address
OS scheduler timing
uncontrolled entropy

when deterministic reproducibility is requested.

If a deterministic seed is supplied, the resulting random stream must be reproducible.

---

15. Seed is not the complete reproducibility key

A seed alone is insufficient.

The reproducibility identity must include at least:

source/program identity
IR identity/version
target identity/version
target capability snapshot
routing result
calibration snapshot
timing model
resource model
constraint set
scheduler configuration
algorithm identity/version
policy identity/version
objective identity/version
random seed/state
serialization schema
compiler/runtime version where relevant

Therefore:

seed = 42

does not guarantee identical output if the target changed.

---

16. Target snapshot requirement

Hardware may be dynamic.

A scheduler must distinguish:

target identity

from:

target snapshot

A deterministic compilation must consume a coherent snapshot.

The snapshot may include:

hardware revision
topology
instruction set
capabilities
timing
resource capacity
calibration identity
availability
alignment
communication characteristics

The existing hardware architecture explicitly treats calibration, topology, instruction sets, timing, capabilities, and provenance as hardware-layer concerns.

A schedule generated from target snapshot A must not be silently compared as byte-identical to a schedule generated from target snapshot B.

---

17. Calibration determinism

Calibration is dynamic hardware state.

Therefore:

same program
+
same scheduler configuration
+
different calibration

may legitimately produce:

different schedule

because fidelity-aware scheduling may make different decisions.

For reproducibility, the scheduler must record:

calibration snapshot identity
calibration version
calibration timestamp
validity information

where supplied by hardware.

It must not fabricate calibration identity.

---

18. Time determinism

The scheduler must not use the current wall clock to make algorithmic decisions.

Forbidden:

if current_time % 2 == 0

or:

random_seed = system_time

in deterministic mode.

The system clock may be recorded as provenance metadata, but it must not alter the deterministic schedule.

---

19. Timing model

The scheduler must support:

continuous time
discrete time
hardware ticks
sample periods
rational resolutions
symbolic durations
calibrated durations
interval/uncertain durations

No scheduling algorithm may assume:

1 ns

or:

dt = constant

unless that value came from the target timing model.

The hardware layer already defines timing as a target capability rather than a scheduler-wide constant.

---

20. Floating-point determinism

Floating-point calculations can produce platform-dependent results.

Therefore objective and fidelity calculations must define their reproducibility policy.

Where exact reproducibility is required:

- prefer integer/rational representations for exact timing;
- avoid unnecessary floating-point accumulation;
- use canonical comparison rules;
- define tolerance explicitly;
- avoid NaN-dependent ordering;
- never use approximate equality as an ordering relation;
- ensure equal-cost candidates still reach a canonical tie-breaker.

If floating-point is unavoidable, the algorithm must document the accepted reproducibility scope.

---

21. NaN and infinity

A deterministic comparator must never leave NaN ordering undefined.

Cost/score types must define behavior for:

NaN
+Infinity
-Infinity

Prefer rejecting invalid values at construction.

For physical scheduling quantities:

negative duration
NaN duration
negative capacity
NaN cost

must be rejected rather than normalized silently.

---

22. Integer overflow

Scheduling arithmetic must use checked operations.

Operations include:

start + duration
finish + latency
critical_path + duration
resource_capacity calculations
deadline calculations
time-window arithmetic

Overflow must produce a structured error.

The scheduler must never wrap:

u64::MAX + 1

into an apparently valid schedule.

This is especially important because the architecture intentionally permits very large workloads.

---

23. Deterministic dependency graph

"ir/graph.rs" must provide deterministic graph traversal.

Requirements:

- canonical operation ordering;
- deterministic predecessor ordering;
- deterministic successor ordering;
- deterministic topological sorting;
- deterministic cycle detection;
- stable critical-path traversal.

For equal candidates:

OperationId

must provide a stable fallback ordering.

---

24. Topological sorting

Topological ordering must not depend on insertion order.

If several nodes are simultaneously ready:

ready = {A, B, C}

the scheduler must choose:

canonical(A, B, C)

rather than:

first element returned by collection

A priority queue or ordered structure should be used where appropriate.

---

25. Cycle detection

Cycle detection must be deterministic.

If a cycle exists, the scheduler should report a deterministic diagnostic containing:

- representative operation;
- relevant dependency;
- stable operation IDs;
- dependency classification.

The exact diagnostic order must not depend on hash-map traversal.

---

26. Resource determinism

Resource allocation must be deterministic.

If multiple resources satisfy a requirement:

R1
R2
R3

the scheduler must use a canonical selection policy.

For example:

lowest canonical ResourceId

after policy-level scoring.

Never:

first resource returned by HashMap

---

27. Resource identity

Resource IDs must be stable.

Resource identity should come from the target/resource model.

Scheduling must not derive identity from:

- memory addresses;
- pointer values;
- object allocation order;
- process IDs;
- thread IDs.

Resource hierarchy must also have stable ordering.

For example:

device
 ├── module
 │    ├── channel
 │    └── channel
 └── module

must have canonical identifiers.

---

28. Reservation determinism

Reservations must have stable ordering.

A canonical reservation order should be based on:

start time
resource identity
operation identity
reservation identity

This ensures that two equivalent internal reservation sets serialize identically.

---

29. Parallel scheduling

Parallelism must not break deterministic output.

This is a critical production requirement.

Incorrect:

worker 1 finds candidate A
worker 2 finds candidate B
whichever locks first wins

That creates race-dependent schedules.

Correct:

workers
   │
   ▼
independent candidate evaluation
   │
   ▼
canonical result collection
   │
   ▼
deterministic arbitration
   │
   ▼
commit

Parallelism may accelerate evaluation, but only a deterministic arbitration stage may modify the canonical schedule.

---

30. Deterministic parallelism

A deterministic parallel scheduler should conceptually perform:

1. Construct canonical ready set.
2. Partition evaluation work deterministically.
3. Evaluate candidates in parallel.
4. Collect all results.
5. Sort candidates canonically.
6. Select the winning candidate.
7. Commit exactly one deterministic decision.
8. Update state.
9. Repeat.

Worker completion order must never decide the schedule.

---

31. Worker count independence

A particularly important production invariant is:

workers = 1

and:

workers = N

should produce the same result in deterministic mode.

Therefore:

deterministic schedule

must not depend on the number of worker threads.

This is essential for reproducibility across developer machines, CI systems, servers, and large compilation environments.

---

32. Distributed scheduling

Distributed scheduling must use the same principle.

Different nodes may evaluate candidates concurrently.

Their arrival order must not determine the global schedule.

Instead:

Node A ─┐
Node B ─┼──► canonical merge ──► deterministic decision
Node C ─┘

The merge must use:

- stable node identity;
- stable operation identity;
- stable resource identity;
- stable candidate ordering;
- stable epoch/round identity.

---

33. Distributed failure handling

A deterministic distributed scheduler must not silently use:

whoever responds first

as an algorithmic decision.

If a deterministic distributed policy permits fallback, the fallback policy must itself be deterministic.

For example:

preferred node
↓
canonical fallback node

rather than:

first responding node

unless the caller explicitly selected latency-driven nondeterministic behavior.

---

34. Dynamic scheduling

Runtime-dependent scheduling is fundamentally different from static deterministic compilation.

A dynamic schedule may depend on:

measurement result
hardware state
runtime event
communication arrival
feedback

In that case, the scheduler must distinguish:

static determinism

from:

runtime determinism

A dynamic program may produce different runtime branches because its measured data differ.

That is not necessarily scheduler nondeterminism.

---

35. Dynamic branch determinism

For the same runtime state:

same measurement results
+
same target state
+
same runtime scheduler version
+
same configuration

the scheduler should select the same branch and timing decisions where deterministic execution is requested.

The dynamic scheduler must not randomly select a branch unless explicitly configured to do so.

---

36. Measurement ordering

Measurements are semantically significant.

Their ordering must be derived from:

- canonical IR dependencies;
- explicit control dependencies;
- resource constraints;
- measurement grouping rules.

The scheduler must never reorder measurements simply because a collection happened to expose them in another order.

---

37. Classical feedback

The chain:

measurement
   │
   ▼
classical result
   │
   ▼
classical processing
   │
   ▼
feedback
   │
   ▼
quantum operation

must have deterministic dependency representation.

If feedback latency is target-dependent, the target snapshot must define it.

---

38. QEC determinism

QEC scheduling must be deterministic when requested.

The generic scheduler may consume:

QEC round
syndrome dependency
ancilla requirement
measurement requirement
decoder latency
feedback requirement

but it must not invent QEC topology.

The existing stabilizer scheduler has correctly been transformed toward a compatibility facade rather than a second independent scheduler. It explicitly avoids fixed qubit counts, fixed ancillas, fixed topology, fixed rounds, and synthetic legacy operations.

The deterministic scheduler must therefore operate on explicit QEC inputs.

---

39. QEC round ordering

For QEC:

round 0
round 1
round 2
...
round N

must have explicit dependency semantics.

No scheduler may assume:

distance = 3

or derive the number of rounds from a hard-coded formula unless that formula belongs to the QEC subsystem and is explicitly provided as part of the scheduling request.

---

40. Routing determinism

Scheduling receives the routing result.

It must not silently reroute operations because a resource conflict occurs.

The architectural boundary remains:

routing
    = WHERE

scheduling
    = WHEN

The existing routing subsystem explicitly establishes this separation and provides deterministic/reproducible configuration as part of routing itself.

If routing is regenerated, that is a different compilation stage and must be recorded in provenance.

---

41. Routing result identity

The scheduling provenance must identify:

routing algorithm
routing configuration
routing version
initial mapping
final mapping
mapping version
routing result identity

A schedule cannot be considered reproducible if the routing result is unknown.

---

42. Hardware determinism

Hardware information must arrive through the hardware adapter.

The scheduler must not directly query:

- provider SDKs;
- network APIs;
- QPU status;
- credentials;
- live hardware endpoints.

The hardware architecture explicitly isolates provider-specific implementations below adapters and keeps core abstractions provider-neutral.

The scheduler should receive an immutable target snapshot.

---

43. Live hardware state

If a live target changes during scheduling:

calibration changed
resource disabled
channel unavailable
device degraded

the scheduler must not silently claim the old schedule remains reproducible.

Instead:

snapshot changed
     │
     ▼
schedule invalidated/replanned

according to the configured policy.

---

44. Availability determinism

Availability calendars must be represented explicitly.

For example:

Resource R1:
[0, 100] available
[100, 200] unavailable
[200, ∞) available

The ordering and interval representation must be canonical.

Overlapping availability intervals must have deterministic normalization.

---

45. Constraint ordering

Constraints may originate from:

IR
routing
hardware
QEC
dynamic control
communication
user configuration
plugins

Their evaluation order must not change semantic results.

Constraint evaluation should therefore be:

canonical collection
+
deterministic evaluation

rather than arbitrary plugin registration order.

---

46. Constraint conflicts

If two constraints conflict, the scheduler must produce deterministic diagnostics.

For example:

deadline requires start <= T1
resource availability requires start >= T2

If:

T2 > T1

the scheduler returns an unschedulable result.

It must not randomly choose one constraint to ignore.

---

47. Policy determinism

Every policy must define:

priority
tie-break
objective
feasibility rules

Examples:

ASAP
ALAP
critical-path
resource-aware
fidelity-aware
multi-objective
adaptive

The policy implementation must be deterministic in deterministic mode.

---

48. ASAP determinism

ASAP means:

«select the earliest feasible operation placement.»

When several operations can start at the same time:

canonical priority

must decide.

The implementation must not rely on insertion order.

---

49. ALAP determinism

ALAP must operate from a well-defined deadline/makespan boundary.

If several operations have equal latest-start opportunities, their ordering must be deterministic.

Backward traversal must also be deterministic.

---

50. Critical-path determinism

Critical-path analysis must produce stable:

earliest start
earliest finish
latest start
latest finish
slack
critical-path membership

When multiple critical paths have equal cost, the selected representative path must use canonical operation ordering.

---

51. Resource-constrained scheduling determinism

RCPSP-style scheduling can have many equivalent solutions.

The scheduler must therefore define:

objective
secondary objective
tertiary objective
canonical tie-break

For example:

minimize makespan
then minimize idle time
then minimize resource switching
then canonical OperationId

The exact objective hierarchy must be configuration-driven.

---

52. Adaptive scheduling determinism

Adaptive scheduling may inspect:

graph density
resource pressure
parallelism
target capabilities
communication cost
operation count
QEC requirements

The selection decision must be deterministic when deterministic mode is enabled.

For example:

if resource_pressure >= threshold:
    use resource-constrained planner
else:
    use list planner

must use explicitly defined values from configuration or derived target data.

Thresholds must not be hidden machine-size constants.

---

53. Algorithm selection identity

The schedule provenance must record:

algorithm name
algorithm version
algorithm configuration
policy name
policy version
objective version

This prevents a future algorithm change from being mistaken for nondeterminism.

---

54. Plugin determinism

Plugins must participate in the same determinism contract.

A plugin must declare whether it supports:

deterministic
randomized
parallel
distributed
incremental
runtime

execution.

A deterministic scheduling request must not silently invoke a plugin that cannot guarantee deterministic behavior.

---

55. Plugin ordering

Plugin discovery order must never determine scheduling behavior.

Do not use:

filesystem enumeration order
network discovery order
registration race order

as algorithm selection.

Plugins must have stable identifiers and explicit priorities.

---

56. Serialization determinism

"serialization/encode.rs" must produce canonical serialized schedules.

Equivalent deterministic schedule objects should serialize identically.

Canonicalization includes:

- field ordering where the format permits it;
- operation ordering;
- resource ordering;
- reservation ordering;
- constraint ordering;
- diagnostic ordering;
- stable metadata representation.

Serialization must not contain nondeterministic:

memory addresses
pointer values
thread IDs
random UUIDs
wall-clock timestamps

unless those values are explicitly declared non-semantic provenance.

---

57. Schedule IDs

A schedule ID should preferably be derived from canonical content when reproducibility requires it.

Conceptually:

ScheduleId =
Hash(
    canonical schedule
    +
    target identity
    +
    scheduler identity
    +
    configuration identity
)

If a random UUID is used instead, it must be explicitly classified as a non-deterministic artifact identity rather than semantic schedule identity.

Do not mix those concepts.

---

58. Canonical schedule representation

A canonical schedule should order operations by a deterministic key.

A suitable conceptual ordering is:

start time
→
phase/epoch
→
resource identity
→
operation identity

The exact representation may differ internally, but serialized and compared schedules must use one canonical ordering.

---

59. Provenance

Every production schedule should be traceable to:

program identity
IR identity/version
routing result
target snapshot
calibration snapshot
scheduling configuration
policy
algorithm
objective
seed/random state if applicable
compiler version
schema version

This is required to distinguish:

true nondeterminism

from:

changed input

---

60. Diagnostics determinism

Diagnostic output must also be deterministic.

For the same deterministic input:

same schedule
+
same verification
=
same diagnostic ordering

Diagnostics should be ordered by stable identifiers.

For example:

Operation 12 delayed:
  dependency Operation 4 incomplete

must not sometimes appear before or after an unrelated diagnostic merely because worker threads completed differently.

---

61. Explanation determinism

"diagnostics/explain.rs" must explain scheduling decisions using stable causes.

Possible causes:

dependency
resource
alignment
measurement latency
communication latency
deadline
availability
policy
optimization
QEC constraint

The cause ordering must be canonical.

---

62. Trace determinism

"diagnostics/trace.rs" must distinguish:

logical scheduling decision

from:

internal implementation event

Thread scheduling events should not pollute canonical semantic traces.

For reproducibility, traces should be based on scheduling epochs/decisions rather than wall-clock timestamps.

---

63. Profiling determinism

Performance measurements are inherently environment-dependent.

Therefore profiling data must not be part of canonical schedule identity.

Separate:

semantic schedule

from:

performance profile

A schedule may be identical while:

planning_time
memory_usage
worker_count
CPU model

differ.

---

64. Deterministic verification

Verification must itself be deterministic.

The verifier must process:

operations
dependencies
resources
timing intervals
constraints

in canonical order.

If multiple violations exist, their reported order must be stable.

---

65. Semantic verification

The final verification layer must establish that scheduling did not change computation semantics.

At minimum:

same operation identities
same operands
same control dependencies
same measurement semantics
same classical dependencies
same target-compatible meaning

Scheduling may add explicit timing constructs such as delays where permitted, but those must preserve semantic intent.

---

66. Resource verification

Verify:

resource usage <= resource capacity

for every relevant interval.

This must not depend on iteration order.

For a capacity-1 resource:

A: [0,10]
B: [10,20]

is valid.

But:

A: [0,10]
B: [9,20]

is invalid.

Boundary semantics must be explicitly defined and tested.

---

67. Timing verification

Verify:

finish = start + duration

using checked arithmetic.

Verify:

start >= release
finish <= deadline

when those constraints exist.

Verify target alignment:

start mod resolution == required alignment

when the target requires discrete alignment.

---

68. Deterministic objective evaluation

Objective evaluation must use the same input ordering and numerical semantics.

For:

makespan
depth
idle time
fidelity
energy
communication cost

the calculation order must be stable.

If summation order can affect floating-point results, use a deterministic accumulation strategy.

---

69. Multi-objective determinism

A multi-objective optimizer must define whether it returns:

one canonical optimum

or:

a Pareto frontier

If one schedule is required, the selection from the frontier must use a canonical rule.

For example:

minimize makespan
then fidelity cost
then idle time
then OperationId ordering

---

70. Deterministic optimization passes

Scheduling-level transformations must be deterministic.

This includes:

delay insertion
alignment
padding
dynamical decoupling

Candidate insertion order must be canonical.

No transformation may use random insertion order unless explicitly requested.

---

71. Dynamical decoupling

Dynamical decoupling is optional.

If enabled, its sequence selection must be deterministic when deterministic mode is enabled.

The scheduler must record:

DD policy
DD sequence identity
DD configuration

in provenance.

It must not silently change semantic scheduling identity without recording the transformation.

---

72. Resource calendars

Resource calendars must have deterministic normalization.

If intervals overlap:

[0,10]
[5,20]

the normalized representation must always be the same.

If intervals touch:

[0,10]
[10,20]

the canonical representation must define whether they remain separate or merge.

This decision must not vary between implementations.

---

73. Memory scalability

Determinism must not require enormous memory overhead.

Do not create:

qubits × maximum_time

matrices merely to make ordering easy.

Prefer:

operation → interval
resource → ordered interval structure
dependency → adjacency

The scheduler must scale according to actual graph/resource size.

---

74. No recursion requirement

Deterministic traversal must remain scalable.

Avoid recursion for potentially enormous:

- dependency graphs;
- resource trees;
- distributed topologies;
- QEC graphs.

Use iterative traversal where practical.

This prevents stack depth from becoming an artificial scheduler limit.

---

75. Deterministic collections

Collection selection should follow these rules.

Use:

Vec

when stable sequence order is semantically meaningful.

Use:

BTreeMap
BTreeSet

when ordered lookup is useful.

Use:

HashMap
HashSet

only when iteration order does not affect behavior, or when their contents are subsequently canonically sorted before decisions.

Never let unordered collection iteration directly determine scheduling.

---

76. Hashing

Hashing must not be used as an ordering mechanism unless the hash function and canonical input encoding are explicitly fixed.

Do not:

sort_by(hash)

for scheduling priority unless the hash is itself part of the stable specification.

Canonical semantic ordering should use explicit IDs and defined comparators.

---

77. Stable sorting

When sorting candidates, the sort must have a complete deterministic comparator.

Do not rely on an unstable sort to break equal elements if equal elements can influence future scheduling.

If equal candidates are semantically indistinguishable, their identity still needs to be canonical for:

- diagnostics;
- serialization;
- provenance;
- reproducibility.

---

78. Deterministic cancellation

Cancellation must not corrupt deterministic scheduler state.

If cancellation occurs:

partial schedule
+
cancellation

must produce a structured cancellation result.

The scheduler must not leave:

partially committed hidden reservations

in caller-owned state.

The scheduling transaction should be:

input snapshot
     │
     ▼
planning
     │
     ├── success ──► committed result
     │
     └── failure ──► error

This is consistent with the routing subsystem's transaction-oriented architecture.

---

79. Deterministic failure

Failure must also be reproducible.

Given identical deterministic inputs, the scheduler should produce the same:

error category
error code
representative operation
representative resource
constraint identity
diagnostic ordering

It must not sometimes report:

ResourceUnavailable

and other times:

DeadlineExceeded

merely because validation order changed.

---

80. Error precedence

Define canonical validation/error precedence.

Recommended:

1. malformed input
2. invalid canonical IR reference
3. invalid dependency graph
4. invalid timing
5. invalid resources
6. invalid constraints
7. unsupported target operation
8. infeasibility
9. deadline failure
10. verification failure

The exact precedence belongs in "errors.rs", but it must be stable.

---

81. Deterministic unsupported-operation handling

If multiple unsupported operations exist, report them in canonical OperationId order.

Never report whichever operation happened to be encountered first by an unordered traversal.

---

82. Deterministic resource conflict handling

If multiple conflicts exist, report:

lowest canonical operation/resource pair

or another explicit conflict ordering.

This ensures reproducible diagnostics.

---

83. Reproducibility manifest

Production builds should be able to emit a reproducibility manifest.

Conceptually:

ReproducibilityManifest
├── program_hash
├── ir_schema
├── ir_hash
├── routing_hash
├── target_identity
├── target_snapshot_hash
├── calibration_identity
├── scheduler_schema
├── scheduler_version
├── policy
├── algorithm
├── objective
├── configuration_hash
├── deterministic
├── seed
├── plugin_versions
└── schedule_hash

This is metadata, not another quantum IR.

---

84. Schedule hash

The schedule hash must be calculated over canonical schedule content.

It must exclude volatile data such as:

wall-clock creation time
process ID
thread ID
memory address
CPU temperature
non-semantic profiling data

unless explicitly classified as part of the semantic artifact.

---

85. Environment capture

For strict reproducibility, capture the environment relevant to scheduling.

Potential fields:

Rust/compiler version
Zamani version
scheduler version
algorithm version
target model version
plugin versions
serialization schema
configuration

Do not include irrelevant machine-local values in semantic schedule identity.

---

86. Rust version

The scheduler must compile on:

Rust 1.97
Rust 1.97.1
Rust 2021

as required by the project.

The repository's "Cargo.toml" currently targets Rust 1.97.1/1.97 and Rust 2021.

Do not introduce newer-language features that make Rust 1.97/1.97.1 unsupported.

---

87. Unsafe code

The entire scheduling subsystem must forbid unsafe Rust.

Every scheduler module should inherit or explicitly enforce:

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

No unsafe implementation is justified merely for scalability.

If a dependency uses unsafe internally, that is a dependency implementation concern; Zamani scheduling source itself must contain no unsafe code.

---

88. Deterministic API design

The public API should conceptually resemble:

schedule(
    program,
    target,
    configuration
)

not:

schedule(
    program,
    127,
    100,
    8
)

The first is target-driven.

The second hard-codes machine assumptions.

---

89. Deterministic SchedulingContext

"context.rs" must carry an immutable scheduling snapshot.

Conceptually:

SchedulingContext
├── executable program
├── dependency graph
├── target
├── routing result
├── resource model
├── timing model
├── calibration snapshot
├── constraints
├── policy
├── objective
├── deterministic configuration
└── cancellation/deadline

The context must be immutable during deterministic planning.

If a value changes, it must be represented as a new context or explicit scheduling epoch.

---

90. Epochs

Dynamic and distributed schedulers may need epochs.

An epoch represents a coherent decision state:

Epoch 0
   ↓
decision
   ↓
Epoch 1
   ↓
decision

Each epoch should have a stable identity.

Within an epoch, deterministic candidate evaluation must see the same snapshot.

---

91. Incremental scheduling

When scheduling incrementally:

existing schedule
+
new operations

the scheduler must not arbitrarily reorder unrelated operations.

The incremental policy must explicitly define:

preserve existing schedule

versus:

reoptimize entire schedule

Deterministic mode must produce the same result for the same incremental request and snapshot.

---

92. Schedule comparison

A production scheduler should support comparing two schedules.

Comparison levels:

semantic equivalence
structural equality
canonical schedule equality
objective equality
performance equality

Do not conflate them.

For deterministic regression testing, canonical schedule equality is the strongest useful test.

---

93. Golden tests

"tests/determinism/" should contain canonical schedule fixtures.

For each fixture:

input
target snapshot
configuration
expected canonical schedule
expected schedule hash

The test should verify:

schedule == expected
hash == expected_hash

---

94. Worker-count tests

For every major algorithm, test:

workers = 1
workers = 2
workers = 4
workers = N

where N is determined by the test environment rather than hard-coded as a scheduler architecture limit.

Expected:

same deterministic schedule

---

95. Repeated-run tests

For deterministic mode:

run 1
run 2
run 3
...
run N

must produce identical:

schedule
schedule hash
verification
diagnostic ordering
canonical serialization

---

96. Randomized-mode tests

Randomized mode should test:

same seed → reproducible result
different seed → allowed to produce different result

Do not require different seeds to always produce different schedules; randomness only permits variation.

---

97. Property tests

Required properties include:

deterministic(input) == deterministic(input)

under repeated execution.

Also:

canonical_sort(candidates)

must be invariant under candidate insertion order.

For example:

[A, B, C]
[C, A, B]
[B, C, A]

must produce the same canonical candidate order.

---

98. Collection-order tests

Explicitly construct equivalent inputs using different insertion orders.

Example:

operations inserted A,B,C
operations inserted C,A,B
operations inserted B,C,A

Expected deterministic schedule:

identical

This test catches accidental dependency on collection insertion order.

---

99. Resource-order tests

Construct identical resource sets in different orders.

Expected:

same resource assignment

when deterministic mode is enabled.

---

100. Constraint-order tests

Provide constraints in different insertion orders.

Expected:

same feasibility result
same schedule
same diagnostics

provided the constraints are semantically identical.

---

101. Plugin-order tests

Load equivalent deterministic plugins in different registration orders.

The selected algorithm must remain identical if priorities and identifiers are unchanged.

---

102. Routing integration tests

Run:

same IR
+
same target
+
same deterministic routing config

through routing multiple times.

Then feed the result into scheduling.

Expected:

same routing result
same schedule

This verifies cross-subsystem determinism.

---

103. Hardware integration tests

Use a frozen hardware snapshot.

Then:

same program
+
same routing result
+
same hardware snapshot
+
same scheduler configuration

must produce the same schedule.

Live hardware should not be used for strict deterministic tests.

---

104. QEC integration tests

Create frozen QEC scheduling requests.

Verify:

same QEC request
+
same target
+
same scheduler configuration

produces the same schedule.

The test must not derive topology from an implicit code distance.

---

105. Distributed integration tests

Simulate different message arrival orders.

For deterministic mode:

arrival A,B,C
arrival C,A,B
arrival B,C,A

must result in the same canonical schedule if the semantic inputs are identical.

---

106. Dynamic-circuit tests

For a fixed sequence of measurement results:

measurement results = fixed vector

the dynamic scheduler must make the same decisions.

Different measurement results are legitimate semantic inputs and may produce different branches.

---

107. Serialization round-trip

Test:

schedule
↓
encode
↓
decode
↓
canonicalize

Expected:

decoded_schedule == original_schedule

and:

canonical_encoding(decoded_schedule)
==
canonical_encoding(original_schedule)

---

108. Hash round-trip

Verify:

schedule
↓
hash
↓
serialize
↓
deserialize
↓
hash

produces the same canonical schedule hash.

---

109. Cross-platform determinism

Where practical, CI should test deterministic schedules on supported environments.

The goal is:

Linux
Windows
macOS

producing the same canonical schedule for the same deterministic logical inputs.

If exact bitwise reproducibility cannot be guaranteed for a numerical algorithm, its reproducibility scope must be explicitly documented.

---

110. CPU independence

The semantic schedule must not depend on:

CPU model
SIMD width
cache size
core count
thread scheduling

when deterministic mode is enabled.

Performance may differ.

Schedule semantics must not.

---

111. Parallel reduction

Any reduction operation such as:

sum costs
min candidates
max score
aggregate resource usage

must have deterministic semantics.

For floating-point aggregation, use a deterministic reduction order.

Do not use unordered parallel reduction if its result influences schedule decisions.

---

112. Deterministic caches

Caches must never change semantic results.

Correct:

cache hit
cache miss

produce identical output.

Forbidden:

cache hit → candidate A
cache miss → candidate B

unless both paths are proven semantically and deterministically equivalent.

Cache keys must include every relevant input.

---

113. Cache invalidation

A cache entry must be invalidated when any scheduling-relevant input changes.

Possible key components:

program/IR hash
routing hash
target snapshot hash
timing model hash
resource model hash
constraints hash
algorithm version
policy version
objective version
configuration hash
plugin versions

---

114. Memoization

Memoized results must be canonical.

A scheduler must not use a memory address or object identity as part of memoization identity.

---

115. Deterministic diagnostics from caches

Cache hits must not remove required provenance.

A cached schedule must still identify:

why it is valid
which inputs generated it
which algorithm/version generated it

---

116. Deterministic scheduling transactions

A scheduling invocation should behave transactionally:

immutable input
      │
      ▼
planning
      │
      ▼
candidate schedule
      │
      ▼
verification
      │
      ├── success ──► result
      │
      └── failure ──► error

No caller-owned object should be partially mutated when planning fails.

---

117. No hidden global state

Scheduling must not use mutable global state for:

- current scheduler;
- current seed;
- current target;
- current resource calendar;
- caches;
- plugin registry;
- algorithm selection.

State belongs to caller-owned scheduler/context instances.

This is required for both determinism and thread safety.

---

118. Thread safety

Independent scheduling invocations should be able to run concurrently without affecting one another.

For example:

Scheduler A ──► Target A
Scheduler B ──► Target B
Scheduler C ──► Target C

must not share hidden mutable scheduling state.

---

119. Reentrancy

The scheduler should be reentrant.

Calling:

schedule(A)

must not alter the result of a later:

schedule(B)

unless explicit shared state was supplied by the caller.

---

120. Deterministic cancellation/deadlines

A deadline may affect whether compilation finishes.

The deadline itself must not change the semantic schedule if it is only an execution limit.

Distinguish:

deadline as scheduling constraint

from:

deadline as compiler execution timeout

The former affects schedule semantics.

The latter affects whether the scheduler finishes.

---

121. Timeout behavior

If the scheduler reaches a compilation timeout:

SchedulingError::Timeout

or equivalent structured failure should be returned.

Do not return a schedule that is incomplete but looks valid.

Unless explicitly configured for partial planning, partial results must not be represented as successful final schedules.

---

122. Deterministic partial schedules

If analysis mode allows partial schedules, they must explicitly state:

complete = false

and include:

scheduled operations
unscheduled operations
reason
frontier

Ordering must remain canonical.

---

123. Scalability and determinism

Determinism must not require O(N²) memory or time merely because the machine is large.

The architecture should target:

dependency analysis ≈ O(V + E)

where applicable.

Resource-constrained optimization may be computationally difficult.

The scheduler should therefore distinguish:

exact
heuristic
approximate
adaptive
randomized

algorithms.

Determinism means the selected algorithm behaves reproducibly; it does not imply globally optimal scheduling for every possible machine.

---

124. Large-machine ordering

For huge machines, never construct artificial structures such as:

Vec<Vec<TimeSlot>>

covering all possible resources and all possible time positions.

Use sparse representations:

resource → reservations
operation → interval
dependency → edges

This is necessary for the "tiny to everywhere" requirement.

---

125. Deterministic sparse structures

Sparse structures must still have canonical iteration order when they affect decisions.

For example:

active_resources

may be stored efficiently, but candidates must be canonically ordered before arbitration.

---

126. Resource capacity greater than one

For a resource:

capacity = N

the scheduler must choose resources deterministically when multiple slots are equivalent.

It must never assume:

capacity = 1

or a fixed number of slots.

---

127. Hierarchical resources

Determinism must support:

device
 ├── module
 │    ├── channel
 │    └── channel
 └── module

A child resource allocation must respect parent capacity.

Parent/child traversal must be canonical.

---

128. Composite resources

An operation may require:

qubit A
qubit B
channel C
readout R

The complete resource requirement must be evaluated atomically.

Do not allocate some resources, fail another, and leave hidden state behind.

---

129. Deterministic atomic reservation

Reservation must be:

check all
   │
   ▼
commit all

or:

reject all

No partial reservation may remain after failure.

---

130. Communication determinism

For distributed quantum computing, communication operations must be represented explicitly.

Examples:

entanglement generation
teleportation
classical communication
synchronization
remote gate

Their timing and resource requirements must be target-supplied.

Communication candidate selection must use stable node/link identities.

---

131. Communication tie-breaking

If multiple network paths have identical objective cost, select using canonical:

path identity
node identity
link identity

rather than network response order.

---

132. Topology determinism

Scheduling must consume topology information from routing/hardware.

It must not invent topology.

Topology representation must have stable identifiers.

The routing architecture explicitly treats physical connectivity and mapping as first-class concepts.

---

133. Technology independence

Deterministic scheduling must work without knowing whether the target is:

superconducting
trapped-ion
neutral-atom
photonic
spin
topological
annealing
analog
logical/FTQC
distributed
simulator
emulator
future technology

The hardware layer explicitly targets this broad technology-independent model.

The scheduler sees:

capabilities
resources
timing
constraints

not vendor-specific assumptions.

---

134. Vendor independence

No scheduler algorithm may contain:

if IBM
if IonQ
if Braket
if VendorX

Vendor-specific information belongs under hardware adapters.

The scheduler must consume normalized target capabilities.

---

135. Simulator determinism

A simulator target should be able to use exactly the same scheduling pipeline.

The target snapshot must identify simulator semantics.

A simulator's deterministic state must not be confused with scheduling determinism.

---

136. Emulator determinism

Hardware-oriented emulators should expose target constraints through the same hardware scheduling adapter.

The scheduler should not contain emulator-specific branches.

---

137. Benchmarking integration

The benchmarking subsystem should consume:

ScheduleResult

and measure:

makespan
depth
idle time
resource utilization
communication overhead
planning time
verification time

Benchmarking must not modify scheduler decisions.

The hardware architecture likewise keeps benchmarking downstream rather than making hardware depend on benchmarking.

---

138. Benchmark reproducibility

A scheduling benchmark must record:

program hash
target snapshot
scheduler version
algorithm
configuration
seed
worker count
schedule hash

This allows meaningful comparison.

---

139. Regression protection

Every discovered nondeterminism bug must result in a regression test.

A regression fixture should contain:

input
configuration
target snapshot
expected schedule
expected hash

where practical.

---

140. Determinism test directory

The target scheduling tree must include:

tests/
├── unit/
├── integration/
├── property/
├── regression/
├── scalability/
├── determinism/
└── fixtures/

"determinism/" owns tests specifically proving reproducibility.

---

141. File-by-file integration contract

The following files must implement the determinism contract independently.

"types.rs"

Must define stable scheduler-owned identities and values.

Must not define:

QubitId
PhysicalQubitId
OperationId

when those already belong to canonical IR.

Must provide deterministic ordering for scheduler-owned types.

Integration:

types
↓
all scheduling modules

No later file should need to redefine ordering.

---

"errors.rs"

Must define deterministic error categories and precedence.

Must provide stable structured diagnostic fields.

Integration:

all modules
↓
SchedulingError

No module should invent its own incompatible error hierarchy.

---

"limits.rs"

Must represent caller/deployment limits.

Must never become machine-size constants.

Integration:

limits
↓
context
↓
planner

---

"config.rs"

Must freeze deterministic behavior.

It must contain:

deterministic mode
seed/random state policy
tie-break policy
parallelism policy
algorithm
objective
verification

Integration:

config
↓
context
↓
planner/algorithm

---

"context.rs"

Must contain the immutable snapshot required for deterministic planning.

Integration:

IR
routing
hardware
QEC
timing
resources
configuration
        ↓
SchedulingContext

No planner should independently query another subsystem.

---

"result.rs"

Must contain canonical output and reproducibility metadata.

Integration:

planner
verification
diagnostics
serialization
benchmarking
        ↓
ScheduleResult

---

142. "ir/operation.rs"

Must produce deterministic scheduling operation views.

Each operation must retain:

canonical OperationId
canonical qubits
physical mapping where already supplied
duration
resource requirements
dependencies
constraints
metadata

Integration:

quantum::ir
↓
adapters::ir
↓
ir::operation

No duplicate quantum operation semantics.

---

143. "ir/dependency.rs"

Must define deterministic dependency classification.

Examples:

quantum dependency
classical dependency
measurement dependency
control dependency
resource dependency

Integration:

IR
↓
dependency analysis
↓
planner

---

144. "ir/graph.rs"

Must provide deterministic:

topological traversal
ready set
cycle detection
predecessor access
successor access

No unordered iteration may determine behavior.

---

145. "ir/critical_path.rs"

Must calculate deterministic:

critical path
slack
earliest times
latest times

Equivalent inputs must produce equivalent critical-path results.

---

146. "resources/resource.rs"

Must define canonical resource semantics.

Must support:

exclusive
shared
capacity-limited
consumable
reusable
hierarchical
composite

Resource identity must be stable.

Integration:

hardware adapter
routing
QEC
distributed
↓
resource model

---

147. "resources/pool.rs"

Must maintain deterministic resource candidate ordering.

It must not rely on insertion order.

---

148. "resources/reservation.rs"

Must provide deterministic interval reservations.

Reservations must be atomically committed.

---

149. "resources/calendar.rs"

Must normalize resource availability deterministically.

---

150. "resources/availability.rs"

Must represent:

available
busy
disabled
degraded
unknown

in a stable form.

Dynamic updates must create new snapshots/epochs rather than silently mutating deterministic planning state.

---

151. "timing/duration.rs"

Must represent target-independent duration.

Must reject invalid values.

Must support exact representations where required for reproducibility.

---

152. "timing/time.rs"

Must provide checked time arithmetic.

Overflow/underflow must become structured errors.

---

153. "timing/resolution.rs"

Must consume target-provided resolution.

No fixed nanosecond assumption.

---

154. "timing/alignment.rs"

Must deterministically normalize operation start times to target alignment.

Equivalent candidates must resolve identically.

---

155. "timing/windows.rs"

Must represent:

release
earliest start
latest start
deadline
availability

without machine-size assumptions.

---

156. "timing/constraints.rs"

Must combine timing requirements deterministically.

---

157. "policies/policy.rs"

Must define the canonical policy contract.

The contract must specify:

priority
objective
tie-breaking
determinism support

---

158. "policies/asap.rs"

Must provide deterministic ASAP behavior.

Equal-ready operations require canonical tie-breaking.

---

159. "policies/alap.rs"

Must provide deterministic ALAP behavior.

Backward traversal must be canonical.

---

160. "policies/priority.rs"

Must allow explicit caller-defined priority.

Equal priority must fall back to canonical ordering.

---

161. "policies/resource_aware.rs"

Must score resource pressure deterministically.

---

162. "policies/hybrid.rs"

Must compose policies in a fixed order.

No implicit policy ordering.

---

163. "planners/planner.rs"

Must define the stable planner trait.

The planner must declare:

deterministic support
parallel support
runtime support

where applicable.

---

164. "planners/list.rs"

Must implement deterministic list scheduling.

The ready queue must use explicit ordering.

---

165. "planners/critical_path.rs"

Must use deterministic critical-path information.

---

166. "planners/resource_constrained.rs"

Must provide deterministic candidate/resource arbitration.

---

167. "planners/event.rs"

Must process events using:

time
event kind
resource identity
operation identity

as stable ordering keys.

---

168. "algorithms/*"

Every algorithm must independently declare:

algorithm ID
algorithm version
deterministic behavior
randomness requirements
parallel behavior
resource assumptions
complexity characteristics

No algorithm may rely on another algorithm's undocumented tie-breaker.

---

169. "transformations/*"

Every transformation must:

preserve semantics
be deterministic when requested
record provenance

---

170. "verification/*"

Verification must itself be deterministic.

All reported violations must have canonical ordering.

---

171. "optimization/*"

Optimization must define:

objective
numerical semantics
tie-breaking
determinism

and must never depend on incidental collection order.

---

172. "qec/*"

QEC scheduling contracts must consume explicit QEC requirements.

No hidden:

distance → topology

conversion belongs here unless explicitly delegated to the QEC subsystem.

---

173. "dynamic/*"

Dynamic scheduling must separate:

runtime-dependent behavior

from:

algorithmic nondeterminism

---

174. "distributed/*"

Must define deterministic distributed arbitration.

Network arrival order cannot decide semantic schedule in deterministic mode.

---

175. "adapters/ir.rs"

This is the canonical boundary between:

quantum::ir

and:

scheduling

It must preserve canonical IDs and source provenance.

It must use:

crate::quantum::ir::qubit::QubitId

and:

crate::quantum::ir::qubit::PhysicalQubitId

where applicable.

---

176. "adapters/routing.rs"

Must consume routing results without taking routing ownership.

No hidden rerouting.

Must preserve mapping provenance.

---

177. "adapters/hardware.rs"

Must convert the hardware snapshot into scheduler timing/resource constraints.

It must not communicate with providers directly.

---

178. "adapters/qec.rs"

Must convert QEC requirements into generic scheduling constraints.

No second scheduler.

---

179. "serialization/schema.rs"

Must version the schedule format.

Schema changes must be explicit.

---

180. "serialization/encode.rs"

Must produce canonical deterministic serialization.

---

181. "serialization/decode.rs"

Must validate before reconstructing scheduler state.

---

182. "diagnostics/trace.rs"

Must use stable event identities and ordering.

---

183. "diagnostics/explain.rs"

Must provide deterministic causal explanations.

---

184. "diagnostics/profile.rs"

Must keep performance metrics separate from semantic schedule identity.

---

185. "plugins/scheduler.rs"

Must expose deterministic capability metadata.

---

186. "plugins/registry.rs"

Must use caller-owned registry state.

Plugin order must be canonical.

No global mutable registry.

---

187. "stabilizer_scheduler.rs"

This remains a compatibility facade.

It must not implement an independent scheduling algorithm.

Its role is:

legacy QEC configuration
        ↓
QEC scheduling request
        ↓
generic scheduler

The existing file already establishes this intended compatibility role and explicitly forbids hard-coded qubits, ancillas, topology, rounds, timing, and vendor assumptions.

---

188. "mod.rs"

Must remain a composition root.

It should not implement deterministic algorithms.

Adding a new algorithm inside an existing algorithm namespace should not require changing unrelated scheduling modules.

---

189. "tests/determinism/"

Must test:

same input
same output

same seed
same output

different insertion order
same output

different worker count
same output

different resource enumeration order
same output

different constraint enumeration order
same output

different distributed message order
same output

serialization round-trip
same output

---

190. "tests/scalability/"

Must test increasing workloads without imposing architectural limits.

Scale dimensions:

operations
qubits
resources
dependencies
parallelism
QEC rounds
distributed nodes

Tests must obtain limits from the environment/configuration rather than scheduler constants.

---

191. "tests/regression/"

Every discovered determinism defect becomes a permanent regression test.

---

192. "tests/property/"

Property tests must explicitly attack ordering instability.

---

193. "tests/fixtures/"

Fixtures must contain frozen target snapshots and canonical schedule expectations.

No live provider dependency.

---

194. Determinism acceptance criteria

The scheduler is not production-ready until:

[ ] deterministic mode exists
[ ] randomized mode is explicit
[ ] no hidden random seed
[ ] no wall-clock scheduling decisions
[ ] canonical candidate ordering
[ ] canonical resource ordering
[ ] canonical dependency ordering
[ ] canonical constraint ordering
[ ] deterministic tie-breaking
[ ] deterministic graph traversal
[ ] deterministic resource allocation
[ ] deterministic timing decisions
[ ] deterministic policy selection
[ ] deterministic algorithm selection
[ ] deterministic plugin selection
[ ] deterministic parallel arbitration
[ ] worker-count independence
[ ] deterministic distributed arbitration
[ ] deterministic dynamic behavior for equal runtime state
[ ] deterministic QEC scheduling
[ ] deterministic serialization
[ ] deterministic diagnostics
[ ] deterministic verification
[ ] deterministic objective evaluation
[ ] canonical schedule hashing
[ ] reproducibility manifest
[ ] target snapshot identity
[ ] routing provenance
[ ] calibration provenance
[ ] regression fixtures
[ ] property tests
[ ] scalability tests
[ ] no unsafe code
[ ] Rust 1.97/1.97.1 compatibility
[ ] canonical quantum::ir::qubit identities

---

195. Required invariants

The following invariants are mandatory.

Invariant A — Same deterministic input

same complete input
→
same schedule

Invariant B — Collection order independence

same semantic set
+
different insertion order
→
same schedule

Invariant C — Worker independence

same input
+
different worker count
→
same schedule

Invariant D — Cache independence

cache hit

and:

cache miss

must produce the same semantic schedule.

Invariant E — Serialization independence

encode → decode

must preserve canonical schedule identity.

Invariant F — Target explicitness

target state

must always be an explicit scheduling input.

Invariant G — No machine-size constants

No fixed qubit/resource/time ceiling exists in scheduling architecture.

Invariant H — Canonical qubit identity

Only:

quantum::ir::qubit::QubitId
quantum::ir::qubit::PhysicalQubitId

are authoritative.

Invariant I — Routing/scheduling separation

routing = where
scheduling = when

Invariant J — Hardware isolation

Scheduler does not directly communicate with providers.

Invariant K — No unsafe

Scheduler source contains no unsafe Rust.

---

196. What must never happen

The following are architecture violations:

thread race decides schedule

HashMap iteration decides schedule

filesystem order decides plugin priority

network response order decides deterministic schedule

current time seeds deterministic scheduling

memory address affects candidate priority

worker completion order affects schedule

provider API response order affects schedule

cache hit changes schedule semantics

different insertion order changes schedule

different worker count changes schedule

logical qubit silently becomes physical qubit

scheduler invents topology

scheduler invents hardware timing

scheduler hard-codes machine size

scheduler embeds vendor logic

stabilizer scheduler implements a second scheduler

QEC distance silently determines hardware topology

floating-point NaN determines candidate ordering

integer overflow wraps into valid timing

partial schedule is returned as successful final schedule

---

197. Determinism versus optimization

Determinism does not mean selecting the globally optimal schedule.

A deterministic heuristic may consistently produce:

Schedule A

while another exact algorithm produces:

Schedule B

Both can be deterministic.

Therefore record:

algorithm
objective
quality metrics

alongside the schedule.

---

198. Determinism versus performance

A deterministic scheduler may be parallel.

The goal is:

parallel computation
+
deterministic arbitration

not:

serial computation

Determinism must therefore be designed into the architecture rather than achieved merely by disabling parallelism.

---

199. Determinism versus scalability

A deterministic implementation must remain scalable.

It must not achieve determinism by:

global lock
single-thread execution
full machine-sized matrix
full timeline expansion
global serialization of every candidate

unless explicitly selected as a small-target fallback.

The scalable design is:

parallel candidate evaluation
        ↓
canonical reduction
        ↓
deterministic state transition

---

200. Final integration architecture

The complete deterministic pipeline is:

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
              routing result snapshot
                         │
                         ▼
                scheduling::adapters
                         │
              ┌──────────┼──────────┐
              ▼          ▼          ▼
           IR data    resources    timing
              │          │          │
              └──────────┼──────────┘
                         ▼
                    constraints
                         │
                         ▼
                     policies
                         │
                         ▼
                     planner
                         │
                ┌────────┼─────────┐
                ▼        ▼         ▼
             ASAP      ALAP      RCPSP
                │        │         │
                └────────┼─────────┘
                         ▼
                 deterministic
                    arbitration
                         │
                         ▼
                  candidate schedule
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
             ┌───────────┼────────────┐
             ▼           ▼            ▼
        serialization diagnostics benchmarking
             │
             ▼
       hardware lowering
             │
             ▼
           runtime

---

201. Final "same program, any machine" contract

The deterministic scheduler participates in Zamani's fundamental portability model:

                    ONE ZAMANI PROGRAM
                           │
          ┌────────────────┼─────────────────┐
          ▼                ▼                 ▼
       small QPU        large QPU       distributed QPU
          │                │                 │
          ▼                ▼                 ▼
       routing          routing           routing
          │                │                 │
          ▼                ▼                 ▼
     scheduling       scheduling        scheduling
          │                │                 │
          ▼                ▼                 ▼
       schedule          schedule          schedule

The schedules do not have to be identical.

They must preserve the same program semantics while respecting each target's:

topology
resources
timing
capabilities
availability
calibration
communication
QEC requirements

If the target snapshots are identical, deterministic scheduling must produce the same canonical schedule.

---

202. Production definition of determinism

For Zamani, production determinism means:

«Given an identical canonical program, identical routing result, identical target snapshot, identical resource/timing/constraint state, identical scheduler configuration, identical algorithm/policy/objective versions, identical deterministic random state, and identical relevant scheduler implementation, the scheduler produces an identical canonical schedule independent of collection insertion order, worker completion order, thread count, cache state, or distributed message arrival order.»

This is the definitive contract.

---

203. Final production rule

The scheduler must never ask:

How many qubits does Zamani support?

It must ask:

What resources does this target expose for this execution?

It must never ask:

Which vendor am I compiling for?

It must ask:

What capabilities and constraints does this target snapshot expose?

It must never ask:

What fixed machine size was this scheduler written for?

It must ask:

What resources are available now?

It must never resolve a tie by accident.

It must resolve every tie through a documented canonical policy.

It must never use randomness accidentally.

It must use randomness only when explicitly requested.

It must never allow parallelism to determine semantics.

It must use parallelism only as an implementation mechanism behind deterministic arbitration.

It must never create another quantum identity.

It must use:

crate::quantum::ir::qubit::QubitId
crate::quantum::ir::qubit::PhysicalQubitId

where those identities are required.

It must never introduce:

MAX_QUBITS
MAX_GATES
MAX_CHANNELS
MAX_RESOURCES
MAX_DEPTH
MAX_ROUNDS

as architectural limits.

It must remain safe Rust.

It must remain compatible with Rust 1.97/1.97.1.

And it must remain capable of scaling from the smallest executable quantum workload to arbitrarily large workloads constrained only by the resources actually available.

---

204. File completion definition

"DETERMINISM.md" is satisfied only when every scheduler implementation can answer all of these questions before implementation:

What are the deterministic inputs?

What is the canonical ordering?

What is the tie-breaker?

Where does randomness come from?

What is the seed/state?

What target snapshot is being used?

What resource snapshot is being used?

What timing snapshot is being used?

What routing result is being used?

What QEC requirements are being used?

How are dependencies ordered?

How are resources ordered?

How are constraints ordered?

How is parallelism made deterministic?

How is distributed execution made deterministic?

How is dynamic execution handled?

How is serialization canonicalized?

How is the schedule hashed?

How is provenance recorded?

How are errors ordered?

How is verification ordered?

How is scalability preserved?

How is cache independence guaranteed?

How is worker-count independence guaranteed?

How is insertion-order independence guaranteed?

How is cross-platform behavior handled?

What Rust version is supported?

Does the implementation contain unsafe code?

Does it introduce any machine-size constant?

Does it introduce a duplicate QubitId or PhysicalQubitId?

Does it cross another subsystem without an explicit adapter?

If any answer is:

implementation-defined

without a documented deterministic contract, the implementation is not yet production-ready.

---

205. Final architectural guarantee

The completed Zamani scheduling subsystem must provide:

semantic portability
        +
target specialization
        +
resource awareness
        +
timing awareness
        +
dependency correctness
        +
QEC awareness
        +
dynamic-circuit support
        +
distributed support
        +
deterministic execution
        +
reproducibility
        +
parallel scalability
        +
verification
        +
provenance
        +
zero unsafe code
        +
zero artificial machine-size limits

The scheduler therefore becomes a deterministic, target-specializing temporal execution planner, rather than a fixed-size circuit scheduler.

The canonical architectural rule is:

quantum::ir
    = WHAT

quantum::routing
    = WHERE

quantum::scheduling
    = WHEN

quantum::hardware
    = WHAT THE TARGET CAN ACTUALLY DO

quantum::error_correction
    = HOW FAULT TOLERANCE IS REPRESENTED/PROVIDED

runtime
    = EXECUTE

benchmarking
    = MEASURE

Determinism must preserve those boundaries rather than collapsing them into one implementation.

End of "DETERMINISM.md".This specification is intentionally aligned with the repository’s existing separation of canonical IR, routing, scheduling, and hardware, including the current quantum::ir::qubit identity boundary and the existing stabilizer compatibility-facade direction.