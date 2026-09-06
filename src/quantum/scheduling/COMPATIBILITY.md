Worked for 18s

Yes. I inspected the current Zamani repository, including the scheduling root, the existing scheduling implementation, the canonical IR scheduling representation, canonical qubit ownership, routing, hardware HAL, and Cargo.toml. There is one particularly important repository-level point: src/quantum/ir/scheduling/ already owns the canonical schedule representation, so src/quantum/scheduling/COMPATIBILITY.md must explicitly prevent the two scheduling layers from becoming competing implementations. The repository also already establishes quantum::ir::qubit::{QubitId, PhysicalQubitId} as the authoritative qubit identity boundary.

Below is the complete document I recommend placing at:

src/quantum/scheduling/COMPATIBILITY.md

Zamani Quantum Scheduling — Compatibility Contract

Path: "src/quantum/scheduling/COMPATIBILITY.md"

Status: Production architecture contract

Target: Rust 1.97 / Rust 1.97.1, Rust 2021

Safety: "unsafe" forbidden

Scope: "crate::quantum::scheduling"

---

1. Purpose

This document defines the compatibility contract for Zamani's production quantum scheduling subsystem.

It exists to ensure that:

1. "src/quantum/scheduling/" can evolve without breaking the canonical Quantum IR;
2. scheduling remains independent of hardware vendors and machine size;
3. the same Zamani program can be compiled for different quantum targets;
4. scheduling does not duplicate routing, hardware, QEC, IR, optimization, or runtime ownership;
5. "quantum::ir::qubit" remains the sole canonical qubit-identity authority;
6. "quantum::ir::scheduling" remains the canonical schedule representation where that subsystem owns the representation;
7. scheduler algorithms remain replaceable;
8. compatibility code remains temporary, explicit, and isolated;
9. no artificial finite machine-size ceiling is introduced;
10. no "unsafe" Rust is introduced;
11. Rust 1.97 and Rust 1.97.1 remain supported;
12. future additions do not require unrelated scheduler files to be repeatedly rewritten.

This document is an architectural contract, not an implementation of scheduling.

---

2. Fundamental compatibility principle

Zamani has two related but distinct scheduling namespaces:

crate::quantum::ir::scheduling
        │
        │ canonical schedule representation
        ▼
      Schedule


crate::quantum::scheduling
        │
        │ scheduling engine
        ▼
      scheduling algorithm

They MUST NOT become competing scheduling systems.

The distinction is:

quantum::ir::scheduling
    = WHAT a completed schedule is

quantum::scheduling
    = HOW a schedule is constructed

The scheduler produces or populates the canonical schedule representation through an explicit adapter.

No second incompatible "Schedule" type may be introduced merely because the scheduling engine needs additional internal state.

---

3. Canonical ownership rules

The following concepts have one authoritative owner.

Concept| Authoritative owner
Quantum program semantics| "quantum::ir"
Quantum operation identity| "quantum::ir::core::identity::OperationId"
Logical qubit identity| "quantum::ir::qubit::QubitId"
Physical qubit identity| "quantum::ir::qubit::PhysicalQubitId"
Canonical schedule representation| "quantum::ir::scheduling"
Logical → physical mapping| "quantum::routing"
Hardware capability| "quantum::hardware"
Hardware timing| "quantum::hardware::timing"
Hardware topology| "quantum::hardware::topology"
Calibration| "quantum::hardware::calibration"
QEC semantics| quantum error-correction subsystem
Noise/ZQN semantics| ZQN/noise subsystem
Scheduling algorithms| "quantum::scheduling"
Schedule verification| "quantum::scheduling::verification"
Execution| hardware/runtime
Benchmark execution| "quantum::benchmarking"

No compatibility layer may silently create a second owner for one of these concepts.

---

4. Canonical qubit identity

This is a non-negotiable compatibility rule.

All scheduling code that requires a logical qubit MUST use:

crate::quantum::ir::qubit::QubitId

All scheduling code that requires an already-mapped physical qubit MUST use:

crate::quantum::ir::qubit::PhysicalQubitId

The scheduler MUST NOT define:

struct QubitId(...);
struct PhysicalQubitId(...);

or equivalent aliases that establish competing semantic ownership.

The repository explicitly identifies "quantum::ir::qubit" as the authoritative logical/physical qubit identity implementation.

The existing scheduling code already follows this requirement in its foundational types.

---

5. Logical versus physical identity

Compatibility code MUST preserve this distinction:

QubitId
    =
logical/canonical program identity


PhysicalQubitId
    =
target-specific physical identity

The scheduler MUST NOT perform:

QubitId → PhysicalQubitId

implicitly.

That conversion belongs to:

quantum::routing

The intended pipeline is:

canonical IR
    │
    ▼
logical operations
    │
    ▼
routing
    │
    ▼
logical → physical mapping
    │
    ▼
mapped operations
    │
    ▼
scheduling

Routing answers:

«WHERE?»

Scheduling answers:

«WHEN?»

The routing subsystem explicitly establishes routing as the logical-to-physical placement boundary.

---

6. Canonical operation identity

Scheduling MUST reuse:

crate::quantum::ir::core::identity::OperationId

It MUST NOT define another semantic operation identifier.

A scheduler-specific identifier is allowed only when it identifies a scheduler artifact rather than a quantum operation.

Examples:

OperationId
    = semantic operation

ScheduleId
    = schedule artifact

DependencyId
    = scheduler dependency edge

ReservationId
    = resource reservation

EpochId
    = dynamic scheduling epoch

SchedulerSessionId
    = scheduling compilation session

The distinction already exists in the scheduling foundational type architecture.

---

7. Canonical resource identity

Where a resource already has canonical IR identity, scheduling MUST consume that identity.

The scheduler MUST NOT reinterpret a semantic resource ID as:

array index
hardware address
qubit count
channel count
machine size

A resource identifier identifies a resource.

It does not specify how many resources exist.

---

8. Canonical schedule representation

The canonical completed schedule is owned by:

crate::quantum::ir::scheduling

In particular, the canonical representation includes concepts such as:

Schedule
ScheduledOperation
ScheduleResource
ScheduleResourceKind

The existing canonical schedule implementation explicitly defines itself as the representation of a completed scheduling result rather than the scheduling algorithm.

Therefore:

quantum::scheduling
        │
        │ produces
        ▼
quantum::ir::scheduling::Schedule

is the preferred relationship.

A second public:

struct Schedule

inside "quantum::scheduling" MUST NOT be created.

---

9. Internal scheduling representation

"quantum::scheduling" MAY maintain an internal scheduling representation when required for algorithmic efficiency.

For example:

scheduling::ir::SchedulingOperation
scheduling::ir::DependencyGraph
scheduling::ir::Task

These structures must contain only scheduling-relevant information.

They MUST NOT become a replacement for:

quantum::ir

They must be converted explicitly to/from canonical IR through:

scheduling::adapters::ir

The boundary is:

quantum::ir
      │
      ▼
scheduling::adapters::ir
      │
      ▼
scheduling::ir
      │
      ▼
scheduler
      │
      ▼
quantum::ir::scheduling::Schedule

---

10. No semantic duplication

The scheduler MUST NOT redefine:

Gate
QuantumOperation
QuantumCircuit
QubitId
PhysicalQubitId
OperationId
Measurement
ClassicalBitId

unless the type is explicitly a scheduler-specific view or adapter representation and cannot be mistaken for the canonical semantic type.

A scheduling descriptor is acceptable.

A competing semantic operation type is not.

---

11. Write-once, scale-everywhere contract

The fundamental Zamani guarantee is:

one source program
        │
        ▼
one canonical semantic representation
        │
        ├── small machine
        ├── medium machine
        ├── large machine
        ├── fault-tolerant machine
        ├── distributed machine
        ├── simulator
        └── future machine

The program itself MUST NOT encode assumptions about:

physical qubit count
control channel count
device clock
device topology
vendor
QPU size
number of modules
number of nodes

The target determines these properties.

---

12. Meaning of "infinity"

"Scale to infinity" means:

«Zamani scheduling introduces no artificial finite architectural machine-size limit.»

It does NOT mean a finite process can physically allocate infinitely many objects.

Actual execution is bounded by:

available memory
address space
CPU resources
compilation time
target capacity
operating-system limits
explicit deployment limits
execution deadlines
provider limits

These are deployment constraints.

They are not language-level limits.

---

13. Forbidden hard-coded limits

The scheduling subsystem MUST NOT contain architectural constants such as:

const MAX_QUBITS: usize = ...;
const MAX_OPERATIONS: usize = ...;
const MAX_CHANNELS: usize = ...;
const MAX_RESOURCES: usize = ...;
const MAX_ROUNDS: usize = ...;
const MAX_DEPTH: usize = ...;
const MAX_NODES: usize = ...;

unless a constant is explicitly part of an independently justified algorithmic/data-format requirement and is NOT presented as a machine-size limit.

Deployment limits must be represented by explicit configuration/policy.

---

14. Explicit limits

When limits are necessary, they must be supplied by:

QuantumIrLimits

or the scheduler's explicit scheduling-policy/limit configuration.

The semantic distinction is:

program capability
        ≠
target capability
        ≠
invocation limit

For example:

target has 10,000 qubits
invocation permits 2,000
program uses 500

The scheduler must operate on:

500 required

not allocate structures for:

10,000

unless the algorithm explicitly needs target-wide information.

---

15. Sparse resource representation

A scheduler MUST NOT automatically materialize every resource in a target.

Prefer:

operation → referenced resources
resource → reservations

over:

every machine resource
×
every time slot

This is necessary for very large sparse systems.

A target with billions of potential resources must not require a schedule to contain billions of empty resource records.

---

16. Time compatibility

Scheduling time is abstract.

The scheduling subsystem MUST NOT assume:

nanoseconds
microseconds
picoseconds
device ticks
pulse samples
fixed dt

unless explicitly supplied by the target timing model.

The canonical schedule representation already distinguishes semantic timing from physical interpretation.

The correct model is:

abstract schedule time
        │
        ▼
target timing interpretation
        │
        ▼
physical execution time

---

17. Timing resolution

Timing resolution must be supplied by the target.

Possible target models include:

continuous
integer ticks
rational units
sample periods
provider-defined resolution

The scheduler must not hard-code:

dt = 1 ns

or equivalent.

---

18. Checked temporal arithmetic

All scheduling arithmetic must be checked.

Never rely on wrapping arithmetic for:

start + duration
finish + latency
deadline calculations
critical-path accumulation
resource availability

Overflow must result in a structured error.

The existing scheduler architecture already requires checked temporal calculations.

---

19. Half-open intervals

Schedule intervals SHOULD use:

[start, end)

semantics.

Therefore:

[0, 10)
[10, 20)

do not overlap.

This is important for:

resource reuse
event processing
parallelism
deterministic conflict detection

The canonical schedule representation already establishes this interval model.

---

20. Resource model compatibility

A quantum machine is not merely a collection of qubits.

Scheduling must be able to consume abstract resources including, where supplied by the target:

logical qubits
physical qubits
ancillas
measurement channels
drive channels
control channels
resonators
couplers
lasers
microwave sources
optical channels
classical processors
classical memory
accelerators
communication links
synchronization resources
composite resources
future target-defined resources

The hardware HAL already establishes scheduling as a consumer of hardware timing/resource information rather than a hardware implementation itself.

---

21. Resource capacities

Resources may be:

exclusive
shared
capacity-limited
reusable
time-dependent
hierarchical
composite

The scheduler MUST NOT assume:

one resource = one qubit

or:

one operation = one resource

An operation may require an arbitrary resource set supplied by the target/compiler.

---

22. Resource ownership

Hardware owns physical resource semantics.

Scheduling owns:

when a resource is reserved
when it becomes available
whether candidate operations conflict

Scheduling does NOT own:

what a DAC is
what a resonator physically is
how a laser works
how a vendor channel is implemented

Those belong to hardware adapters.

---

23. Routing compatibility

Routing and scheduling are sequential but separate:

IR
 │
 ▼
routing
 │
 ▼
mapped IR
 │
 ▼
scheduling
 │
 ▼
scheduled mapped IR

Routing must not be implemented inside scheduling.

Scheduling must not perform logical-to-physical routing.

The scheduler may consume the result of routing through:

scheduling::adapters::routing

---

24. Routing output contract

A routing adapter must provide enough information for scheduling to know:

operation identity
logical operands
physical operands
movement operations
mapping state
target-relevant operation information

It must not require the scheduler to reconstruct the routing solution.

---

25. Hardware compatibility

Scheduling consumes target information from:

quantum::hardware

The hardware HAL already identifies timing, topology, calibration, capabilities, scheduling and resource estimation as separate hardware concerns.

The scheduler MUST NOT:

discover QPUs
authenticate providers
submit jobs
query provider SDKs
download calibration
manage credentials

Those responsibilities remain in hardware/provider infrastructure.

---

26. Hardware adapter boundary

The preferred integration is:

quantum::hardware
        │
        ▼
scheduling::adapters::hardware
        │
        ▼
SchedulingContext

The adapter converts target information into scheduler-compatible contracts.

It should provide, where applicable:

capabilities
timing
alignment
resource capacities
resource availability
operation durations
communication latency
calibration-derived scheduling information

No provider SDK types may cross into the scheduler core.

---

27. Calibration compatibility

Calibration is hardware state.

Scheduling may consume:

calibration snapshot identity
calibration timestamp
validity
operation duration
error estimate
availability

but must not own calibration.

A schedule created against one calibration snapshot must retain provenance sufficient to determine what target state it assumed.

Stale calibration must never silently satisfy a requirement for current calibration.

---

28. QEC compatibility

QEC must supply scheduling requirements.

The relationship is:

QEC
 │
 ├── syndrome dependencies
 ├── ancilla requirements
 ├── round constraints
 ├── measurement requirements
 └── feedback dependencies
        │
        ▼
scheduling::adapters::qec
        │
        ▼
generic scheduler

The scheduler MUST NOT become the QEC decoder.

It must not implement:

syndrome decoding
recovery selection
lattice generation
stabilizer-code discovery
surface-code geometry

unless those are explicitly part of a dedicated QEC subsystem.

---

29. Stabilizer scheduler compatibility

The historical:

stabilizer_scheduler.rs

must remain a compatibility facade.

It MUST NOT become a second scheduler.

The desired relationship is:

stabilizer_scheduler
        │
        ▼
QEC scheduling request
        │
        ▼
generic scheduling
        │
        ▼
canonical Schedule

The current repository has already moved this file toward a compatibility-facade model rather than direct synthetic gate generation.

The old behavior of directly generating placeholder H/Measure/Reset operations or synthetic CNOT comments must not be restored.

---

30. No hard-coded surface-code assumptions

Scheduling compatibility MUST NOT assume:

distance = 3
fixed stabilizer weight
fixed ancilla count
fixed number of rounds
fixed lattice geometry
fixed nearest-neighbor structure
fixed physical qubit arrangement

QEC supplies these properties.

---

31. Dynamic-circuit compatibility

The scheduler MUST NOT assume every program is a static DAG whose entire future is known before execution.

It must be capable of representing:

measurement
    │
    ▼
classical computation
    │
    ▼
condition
    │
    ▼
future operation

This requires support for:

classical dependencies
conditional operations
measurement latency
feedback latency
runtime events
dynamic scheduling

Static scheduling remains supported.

Dynamic scheduling is an extension of the same semantic model.

---

32. Distributed compatibility

The same scheduling abstractions must support:

single device
    ↓
multi-chip
    ↓
multi-module
    ↓
multi-QPU
    ↓
quantum network

Distributed communication must be represented explicitly.

Possible scheduler resources include:

communication link
entanglement resource
classical link
synchronization resource
remote operation resource

A distributed operation MUST NOT be hidden as a local gate.

---

33. Distributed scheduling boundary

The preferred model is:

distributed routing
        │
        ▼
distributed execution requirements
        │
        ▼
scheduling::distributed
        │
        ├── nodes
        ├── links
        ├── communication
        └── synchronization
        │
        ▼
generic scheduler

Local scheduling must remain usable without enabling distributed scheduling.

---

34. Algorithm compatibility

Scheduling algorithms are replaceable implementations.

The stable boundary should conceptually support:

ASAP
ALAP
list scheduling
critical-path scheduling
resource-constrained scheduling
adaptive scheduling
custom/plugin algorithms

Algorithms must not alter:

quantum semantics
qubit identity
operation identity
canonical IR ownership
hardware ownership
routing ownership

---

35. ASAP compatibility

ASAP means:

«place each operation as early as all constraints allow.»

ASAP must respect:

dependencies
resources
durations
timing windows
alignment
measurement latency
communication latency
deadlines where applicable

ASAP must not mean:

«put every operation at the earliest mathematically possible time while ignoring hardware resources.»

---

36. ALAP compatibility

ALAP means:

«place operations as late as constraints allow.»

ALAP must use a valid schedule horizon.

It must not invent an arbitrary horizon.

The horizon must come from:

deadline
existing schedule span
target constraint
caller-supplied bound
derived critical-path requirement

---

37. List scheduling compatibility

List scheduling should be the primary general-purpose scalable strategy.

Conceptually:

dependency graph
      │
      ▼
ready set
      │
      ▼
priority
      │
      ▼
resource availability
      │
      ▼
select operation
      │
      ▼
reserve resources
      │
      ▼
advance event frontier

The ready set must be deterministic when deterministic compilation is requested.

---

38. Critical-path compatibility

Critical-path analysis may supply:

earliest start
earliest finish
latest start
latest finish
slack
criticality

It must not itself own resource scheduling.

Critical-path analysis is an input to scheduling policy.

---

39. Resource-constrained scheduling

For complex hardware, scheduling is a resource-constrained temporal problem.

The scheduler must be able to distinguish:

dependency constraint

from:

resource constraint

For example:

A → B

means B depends on A.

But:

A uses channel C
B uses channel C

means A and B may have a resource conflict even if neither depends semantically on the other.

---

40. Optimality compatibility

The scheduler MUST NOT promise global optimality for arbitrary resource-constrained problems unless the selected algorithm mathematically guarantees it.

Scheduling may use:

exact algorithms
heuristics
approximations
deterministic heuristics
stochastic algorithms

The result must identify the strategy used.

---

41. Determinism

For deterministic mode:

same program
+
same target snapshot
+
same routing result
+
same configuration
+
same seed

must produce:

same schedule

where the target state itself is unchanged.

Scheduling decisions MUST NOT depend on:

HashMap iteration order
memory addresses
thread timing
OS scheduling
uninitialized state

The existing scheduler architecture already specifies deterministic ordering and operation-ID tie breaking.

---

42. Randomized algorithms

Randomized algorithms may exist.

They must receive an explicit random context/seed.

They must not use hidden global randomness.

The seed must be retained in schedule provenance when reproducibility requires it.

---

43. Parallel scheduling

Parallel implementation is allowed.

Parallel execution MUST NOT make deterministic scheduling depend on thread timing.

The architecture should distinguish:

parallel analysis

from:

parallel semantic decision making

A deterministic arbitration phase may be required after parallel candidate analysis.

---

44. Memory scalability

Do not build a timeline such as:

qubits × time slots

for the entire machine.

Prefer sparse/event-oriented structures:

operation → interval
resource → reservations
dependency → edges
event → affected resources

This is critical for large machines.

---

45. Iterative graph traversal

Potentially enormous dependency graphs MUST NOT depend on recursive traversal that can exhaust the stack.

Prefer iterative:

topological traversal
cycle detection
critical-path traversal
dependency propagation

where appropriate.

---

46. Dependency graph compatibility

The scheduler dependency graph must support:

operation nodes
predecessors
successors
dependency type

Possible dependency classes include:

quantum data dependency
classical dependency
measurement dependency
control dependency
resource dependency
communication dependency

A cycle must be rejected unless the dynamic scheduling model explicitly represents a legal runtime cycle/event construct.

---

47. Resource reservations

A reservation must identify:

operation
resource
start
end
mode

It must not mutate hardware.

Reservations describe the candidate schedule.

Actual execution occurs later.

---

48. Event-driven compatibility

Large schedules should prefer event-driven advancement where appropriate.

Events may include:

operation completion
resource release
measurement completion
classical result availability
communication completion
QEC round completion
deadline boundary

The scheduler must not repeatedly scan the entire schedule when an event frontier can advance the state efficiently.

---

49. Schedule verification

The scheduler must independently verify its output.

Verification must establish at minimum:

all required operations are represented
no operation is duplicated
all dependencies are satisfied
resource capacities are respected
timing arithmetic is valid
alignment requirements are satisfied
target requirements are respected
measurement dependencies are satisfied
dynamic dependencies are valid
distributed dependencies are valid

Where semantic equivalence checking is available, the scheduled representation must preserve program semantics.

---

50. Verification must be independent

A scheduler must not merely say:

algorithm believes schedule is valid

and treat that as sufficient.

The verification subsystem must independently check the produced result.

Preferred pipeline:

candidate schedule
      │
      ▼
structural verification
      │
      ▼
dependency verification
      │
      ▼
resource verification
      │
      ▼
timing verification
      │
      ▼
semantic verification
      │
      ▼
verified schedule

---

51. Schedule transformations

Scheduling-stage transformations must remain explicit.

Examples:

explicit delays
alignment
padding
dynamical decoupling

A transformation MUST NOT silently alter quantum semantics.

The scheduler should be able to distinguish:

schedule construction

from:

schedule transformation

---

52. Explicit delays

Idle time should be representable explicitly where required by downstream lowering.

The scheduler must not silently assume:

absence of operation = irrelevant

Idle time can matter for:

decoherence
dynamical decoupling
resource utilization
latency
fidelity
hardware control

---

53. Dynamical decoupling

Dynamical decoupling is optional scheduling-stage behavior.

It must not become a mandatory scheduler semantic.

The scheduler must be able to operate correctly with:

DD disabled
DD enabled

without changing the core scheduling contract.

---

54. Objective compatibility

Scheduling objectives may include:

makespan
depth
idle time
resource utilization
fidelity
energy
communication cost
latency
multi-objective cost

Objective weights must be explicit.

Never hard-code:

fidelity_weight = ...
duration_weight = ...

inside an algorithm.

---

55. Multi-objective scheduling

A multi-objective policy must make its objective ordering explicit.

For example:

primary:
    minimize makespan

secondary:
    minimize idle time

tertiary:
    maximize estimated fidelity

or:

weighted objective:
    w1 * time
  + w2 * error
  + w3 * communication

Weights are configuration.

---

56. Noise/ZQN compatibility

If the ZQN/noise subsystem supplies scheduling-relevant information, it must enter through an adapter.

Potential inputs:

gate error estimates
duration uncertainty
drift
crosstalk
temporal noise
calibration confidence

The relationship is:

ZQN
 │
 ▼
scheduling adapter
 │
 ▼
objective / constraint

The scheduler must not create a second noise model.

---

57. Serialization compatibility

Scheduling serialization must have one authoritative schema.

It must preserve enough information to reproduce/inspect the schedule, including where applicable:

schedule identity
IR version
operation identities
intervals
resources
dependencies
provenance
configuration identity
target identity
calibration identity
algorithm identity
seed
verification result

It must not serialize:

memory addresses
allocator state
Vec capacity
HashMap implementation details
temporary pointers
thread state

---

58. Compatibility versioning

Public scheduling contracts must be versionable.

A breaking change to:

public type meaning
public trait contract
serialization schema
semantic interpretation

requires an explicit compatibility decision.

Adding an optional field or non-breaking implementation may be a minor change.

The compatibility document must be updated when the contract itself changes.

---

59. Legacy compatibility

Legacy APIs may be preserved temporarily.

However:

legacy API
    ≠
legacy implementation

A legacy API may delegate to the production scheduler.

Legacy code must not cause the production architecture to retain obsolete semantic models indefinitely.

---

60. Legacy "stabilizer_scheduler" contract

Historical code may expect:

StabilizerScheduler::new(...)

That API may remain as a migration facade.

It must:

1. validate its metadata;
2. preserve caller intent;
3. avoid generating synthetic operations;
4. avoid inventing qubits;
5. avoid inventing resources;
6. avoid inventing timing;
7. avoid selecting a hardware topology;
8. delegate to generic QEC scheduling;
9. eventually be deprecable.

The existing compatibility implementation already documents this intended migration.

---

61. Compatibility adapters

The following adapters are required conceptually:

adapters/
├── ir.rs
├── routing.rs
├── hardware.rs
└── qec.rs

Additional adapters may be added without modifying the core algorithms.

---

62. IR adapter

"adapters::ir" owns:

quantum::ir
        ↓
SchedulingTask / SchedulingOperation

It must extract:

OperationId
operands
dependencies
semantic timing requirements
resource requirements
conditions
metadata

It must not change semantics.

It must use:

quantum::ir::qubit::QubitId
quantum::ir::qubit::PhysicalQubitId

where applicable.

---

63. Routing adapter

"adapters::routing" consumes the routing result.

It must not perform routing.

It must translate:

routing result
        ↓
scheduler operands/resources/dependencies

The adapter must preserve:

logical operation identity
physical placement
movement operations
mapping evolution

---

64. Hardware adapter

"adapters::hardware" consumes the provider-neutral hardware HAL.

It must translate:

hardware capabilities
hardware timing
hardware resources
hardware availability
hardware calibration
hardware communication

into:

SchedulingContext

The adapter must not import vendor SDK types into scheduling algorithms.

---

65. QEC adapter

"adapters::qec" translates:

QEC requirements

into:

scheduling constraints
resources
dependencies
round boundaries
feedback requirements

It must not implement QEC decoding.

---

66. Context immutability

A scheduling invocation should receive an immutable logical context containing:

program
target information
timing model
resource model
constraints
policy
objective
limits
provenance
cancellation/deadline

The scheduler may construct private mutable state.

It must not mutate caller-owned canonical IR or hardware objects.

---

67. Transactional behavior

A failed scheduling invocation must not partially modify caller-owned program state.

The model is:

caller state
     │
     ▼
immutable input
     │
     ▼
scheduler
   /   \
success failure
  │       │
  ▼       ▼
result   error

Failure must not leave a partially mutated canonical program.

---

68. Thread safety

The scheduling namespace must not use global mutable scheduler state.

Avoid:

global scheduler
global resource calendar
global algorithm registry
global RNG
global target

Prefer caller-owned:

Scheduler
SchedulingContext
Registry
Cache
RandomContext

where necessary.

---

69. Plugin compatibility

Custom scheduling algorithms may be supplied through the plugin boundary.

A plugin must consume stable scheduling contracts.

A plugin must not:

replace QubitId
replace OperationId
mutate global scheduler state
bypass verification
directly authenticate to hardware

unless a deliberately separate hardware plugin boundary exists.

---

70. Plugin isolation

A plugin failure must become a structured scheduler error.

Plugin-specific types must not leak into the canonical schedule representation.

---

71. Diagnostics compatibility

Production scheduling must be explainable.

Diagnostics should be able to answer:

Why was operation X delayed?

Possible answers:

dependency incomplete
resource occupied
alignment constraint
measurement latency
communication latency
deadline
availability window
policy preference
target restriction

Diagnostics must not alter scheduling semantics.

---

72. Provenance

A production schedule should retain enough provenance to identify:

source program identity
IR version
target identity
routing identity/version
hardware snapshot
calibration snapshot
scheduler version
algorithm
policy
objective
configuration
random seed

This is essential for reproducibility and debugging.

---

73. Error compatibility

Scheduling errors must be structured.

Do not require callers to parse error strings.

Errors should distinguish at least:

invalid input
duplicate operation
unknown dependency
dependency cycle
time overflow
resource conflict
resource unavailable
timing conflict
alignment failure
missing scheduling information
unsupported strategy
verification failure
limit exceeded
serialization failure
plugin failure
cancellation

---

74. No silent fallback

If required information is missing:

duration
resource
dependency
capability
timing resolution

the scheduler must not silently invent a value.

Bad:

missing duration → assume 1

Good:

missing duration → structured error

unless an explicitly configured policy defines a valid fallback.

---

75. No vendor assumptions

Core scheduling code must not contain:

IBM
IonQ
Rigetti
Quantinuum
Braket
specific device names

or equivalent vendor-specific assumptions.

Provider behavior belongs under hardware adapters.

The hardware architecture explicitly establishes provider isolation as a core HAL rule.

---

76. Technology independence

Scheduling must be capable of consuming targets representing different execution technologies.

Examples:

superconducting
trapped ion
neutral atom
photonic
spin
topological
analog
annealing
logical/FTQC
simulator
emulator
future architectures

The scheduler should care about supplied:

operations
resources
durations
constraints
communication
timing

rather than hard-coded physical technology assumptions.

---

77. Gate-arity independence

Do not assume:

all gates are 1-qubit

or:

all gates are 1- or 2-qubit

A target may support:

1-qubit
2-qubit
3-qubit
N-qubit

operations.

Unsupported operations must be handled by synthesis/decomposition before or at the appropriate lowering boundary.

Scheduling itself should not silently invent decompositions.

---

78. Non-circuit workloads

The broader IR is designed not to assume every quantum workload is a simple gate circuit.

Scheduling compatibility should therefore avoid APIs that fundamentally require:

Vec<Gate>

as the universal representation.

Where the workload has schedulable temporal/resource semantics, the scheduler should consume a scheduling descriptor produced by the relevant IR adapter.

---

79. Pulse compatibility

Pulse-level semantics may expose:

duration
channel
frame
alignment
capture
resource

to scheduling.

But pulse generation remains outside the generic scheduler.

The scheduler must not become a pulse compiler.

---

80. Analog compatibility

Analog workloads may expose:

control windows
resource occupancy
synchronization
duration
communication

without forcing them into a gate-only abstraction.

---

81. Annealing compatibility

Annealing workloads must not be forced through a conventional gate scheduling pipeline when their semantic model differs.

The adapter should determine what scheduling semantics actually apply.

---

82. Simulator compatibility

A simulator may have:

zero/abstract hardware latency

or a modeled target timing system.

The scheduler must consume the simulator's supplied target model.

It must not automatically assume simulation means:

all operations take zero time

unless the simulator target explicitly says so.

---

83. Runtime compatibility

The scheduler produces a plan.

It does not execute the plan.

The execution boundary is:

Schedule
   │
   ▼
hardware lowering
   │
   ▼
runtime
   │
   ▼
backend

Runtime may perform additional runtime scheduling for genuinely dynamic conditions.

---

84. Static versus runtime scheduling

The architecture should support:

compile-time scheduling
+
runtime scheduling

Compile-time scheduling resolves everything known statically.

Runtime scheduling resolves information unavailable until execution.

They must use compatible semantic contracts.

---

85. Deadline compatibility

A deadline must be explicit.

The scheduler must not invent:

deadline = arbitrary constant

If a deadline cannot be satisfied, the scheduler must return a structured failure or a clearly marked partial/analysis result according to configuration.

---

86. Availability windows

Resources may be available only during:

[start, end)

windows.

Scheduling must respect:

maintenance
calibration
cooldown
resource reservation
degraded periods
planned unavailability

The scheduler must not assume continuous hardware availability.

---

87. Dynamic resources

Resource availability may change.

The scheduling context must therefore support snapshots or dynamic/event-aware availability.

A static schedule must clearly identify the target state it assumes.

---

88. Cancellation

Long-running scheduling must support explicit cancellation where the public API provides it.

Cancellation must produce:

SchedulingError::Cancellation

or the canonical equivalent.

It must not leave caller-owned state partially modified.

---

89. Explicit memory/resource limits

Limits should be opt-in policy.

Examples:

maximum compiler memory
maximum scheduling time
maximum number of candidate states
maximum parallel workers
maximum search iterations

These are deployment controls.

They are not semantic limitations of Zamani.

---

90. Scalability architecture

For very large workloads, the architecture should support:

streaming input where possible
incremental graph construction
sparse resource structures
event-driven scheduling
partitioning
parallel analysis
distributed planning
checkpointing where appropriate

However, each optimization must preserve the same public semantic contract.

---

91. Incremental scheduling

Future implementations may schedule:

complete program

or:

new operations appended to an existing schedule

without changing operation identities.

Incremental scheduling must preserve existing reservations unless an explicitly permitted rescheduling policy is selected.

---

92. Rescheduling

Rescheduling may be triggered by:

calibration change
resource loss
resource recovery
dynamic condition
deadline change
communication change
runtime feedback

Rescheduling must identify which portion of the schedule was invalidated.

---

93. Compatibility with existing canonical scheduler code

The repository currently has an existing scheduling implementation under:

src/quantum/ir/scheduling/mod.rs

and a canonical schedule representation under:

src/quantum/ir/scheduling/schedule.rs

The existing engine already defines scheduling concepts such as:

SchedulingTask
SchedulingPriority
SchedulingError

and uses canonical IR operation/qubit/resource identities.

The new "src/quantum/scheduling/" architecture MUST therefore integrate with that existing implementation rather than blindly recreating its contracts.

Before introducing a second implementation, determine whether a proposed type already exists in:

quantum::ir::scheduling

If it does, reuse it or explicitly adapt it.

---

94. Preventing two scheduler implementations

The repository must converge toward:

ONE canonical scheduling semantic model
ONE canonical schedule representation
ONE production scheduling-engine boundary
MANY replaceable algorithms

It must NOT converge toward:

IR scheduler
+
compiler scheduler
+
QEC scheduler
+
hardware scheduler
+
stabilizer scheduler

all independently constructing incompatible schedules.

---

95. Compatibility bridge

If migration requires both namespaces temporarily:

quantum::ir::scheduling
          ▲
          │ adapter
          │
quantum::scheduling

The adapter must be explicit.

After migration, one implementation should become authoritative.

---

96. Module completion rule

Each scheduler source file must have a stable contract before implementation is considered complete.

A completed file must define:

purpose
ownership
public API
inputs
outputs
dependencies
error behavior
thread-safety
determinism
scalability
serialization behavior
integration boundary
non-responsibilities
tests

Adding another scheduler module must not require rewriting the completed file merely because the new module exists.

---

97. Dependency layering

The intended dependency direction is:

canonical IR
    │
    ▼
adapters
    │
    ▼
scheduler IR
    │
    ├──────────┬───────────┐
    ▼          ▼           ▼
resources   timing    constraints
    │          │           │
    └──────────┼───────────┘
               ▼
            context
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
        transformations
               │
               ▼
          verification
               │
               ▼
             result

Adapters to other subsystems remain explicit.

---

98. Forbidden dependency direction

The scheduler core must not depend upward on:

frontend parser
routing implementation
provider SDK
runtime implementation
benchmark implementation
CLI
credentials
network clients
filesystem execution

The scheduler may depend on stable contracts supplied through adapters.

---

99. No circular dependency

Avoid:

routing → scheduling → routing

or:

hardware → scheduling → hardware implementation

The correct model is:

routing
   ↓
routing result
   ↓
scheduler adapter
   ↓
scheduling

and:

hardware
   ↓
hardware capability
   ↓
scheduler adapter
   ↓
scheduling

---

100. Integration with optimization

The expected compiler order is:

frontend
   ↓
canonical IR
   ↓
optimization
   ↓
routing
   ↓
scheduling

Optimization must not need to understand physical scheduling details unless a deliberate scheduling-aware optimization is defined.

Scheduling must not reimplement optimization passes.

---

101. Integration with benchmarking

Benchmarking should consume:

ScheduleResult

and derive:

makespan
depth
parallelism
idle time
resource utilization
communication overhead
scheduler compilation time

Scheduling must not implement benchmark protocols.

The hardware architecture likewise defines benchmarking as a consumer rather than a dependency of hardware.

---

102. Integration with diagnostics

Diagnostics should consume scheduler events/results.

They must not change scheduler decisions.

The relationship is:

scheduler
    │
    ├── result
    ├── metrics
    └── trace
          │
          ▼
     diagnostics

---

103. Integration with serialization

Serialization must consume canonical scheduler/result types.

The relationship is:

ScheduleResult
     │
     ▼
serialization
     │
     ▼
versioned representation

The scheduler must not create a competing serialization format.

---

104. Integration with runtime

Runtime receives a verified schedule.

The runtime may reject execution if the target has changed since compilation.

The runtime must not assume:

schedule valid forever

because hardware state may change.

---

105. Integration with hardware validation

Before execution:

Schedule
   │
   ▼
hardware compatibility
   │
   ▼
hardware validation
   │
   ▼
execution

Scheduling validity and hardware executability are related but distinct.

---

106. Schedule validity versus hardware validity

A schedule may be structurally valid under its supplied resource model but fail against a changed hardware target.

Therefore:

scheduler verification
        ≠
final hardware validation

Both are required.

---

107. Target snapshot

A production schedule should be associated with the target snapshot against which it was generated.

Relevant information can include:

device identity
hardware revision
topology version
instruction-set version
timing version
calibration snapshot
capability version
adapter version

---

108. Reproducibility contract

For reproducibility, preserve:

program identity
IR version
target snapshot
routing result identity
scheduler version
strategy
configuration
objective
seed

The schedule must not depend on hidden mutable state.

---

109. Compatibility with Rust 1.97 / 1.97.1

All scheduling code must compile under:

Rust 1.97
Rust 1.97.1
Rust 2021
stable

Do not use nightly-only language features.

Do not introduce unnecessary MSRV increases through dependencies.

The repository package currently declares Rust 1.97.1/1.97 as its intended compiler baseline, so scheduler code must remain compatible with that baseline.

---

110. No unsafe

Every scheduling module MUST enforce:

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

No scheduler optimization is sufficient justification for unsafe code.

Do not introduce:

raw pointers
unsafe blocks
unsafe traits
unsafe FFI
unsafe allocation

into the scheduling core.

---

111. Collections

Use collections according to semantics.

Suitable examples:

BTreeMap
BTreeSet
Vec
VecDeque

where deterministic ordering or efficient sequential access is appropriate.

Hash-based collections may be used internally for performance where ordering does not affect semantic decisions.

Never let hash iteration order define a schedule.

---

112. Semantic identity versus collection index

"usize" may be used for:

Vec positions
collection capacities
internal offsets

but must not become an implicit semantic identity.

Never define:

qubit = vec index
operation = vector position
resource = array slot

as the public semantic model.

---

113. Operation ordering

When multiple ready operations are otherwise equivalent, deterministic scheduling should use stable ordering.

The preferred final tie-breaker is canonical operation identity.

This avoids:

thread race determines schedule

and:

HashMap order determines schedule

---

114. Resource ordering

Resources used in a schedule must have deterministic ordering for:

serialization
diagnostics
verification
hashing
comparison

Resource ordering must not imply physical execution priority unless the policy explicitly says so.

---

115. Priority semantics

Priority is advisory.

It MUST NOT override:

dependency constraints
resource capacity
timing constraints
semantic correctness

A high-priority operation that is not ready remains unschedulable until its constraints permit execution.

---

116. Fairness

Adaptive/priority schedulers should prevent indefinite starvation where practical.

If fairness is part of the selected policy, it must be explicit rather than an accidental side effect of collection ordering.

---

117. Deadlock detection

If the ready set is empty while unscheduled operations remain, the scheduler must determine whether this is caused by:

dependency cycle
resource deadlock
unavailable resource
timing impossibility
missing dynamic information

It must not loop indefinitely.

---

118. Infinite-loop protection

Every iterative scheduling algorithm must have a provable progress condition or explicit termination mechanism.

Possible progress events include:

operation scheduled
event frontier advanced
resource released
dynamic information arrived
search state exhausted
limit reached

---

119. Search limits

Algorithms such as:

RCPSP
adaptive search
stochastic search
multi-trial scheduling

may have explicit search limits.

Those limits belong to:

SchedulingConfig
SchedulingLimits
algorithm configuration

not hidden constants.

---

120. Algorithm registry

A registry may exist for algorithm selection.

It must be caller-owned or otherwise explicitly scoped.

Avoid:

global mutable algorithm registry

The registry must not alter canonical semantics.

---

121. Cache compatibility

Caches may be used for:

critical-path analysis
resource lookup
constraint evaluation
candidate ranking

Cache keys must include all semantic inputs that influence the cached result.

Never reuse a schedule derived from:

different target
different calibration
different policy
different objective

without an explicit compatibility proof.

---

122. Cache invalidation

Hardware/calibration changes may invalidate scheduling caches.

Cache entries must be associated with appropriate target/provenance identity.

---

123. Parallel/distributed cache isolation

Parallel workers must not corrupt shared scheduling state.

Prefer immutable shared inputs and isolated candidate states.

---

124. Testing contract

Every public scheduling contract must have tests.

Required categories:

unit
integration
property
regression
scalability
determinism
serialization

---

125. Minimum correctness tests

Test:

empty schedule
single operation
linear dependency
parallel operations
resource conflict
resource capacity > 1
unknown dependency
duplicate operation
self dependency
cycle
zero duration
large duration
time overflow
deadline
release time
alignment
measurement latency
conditional dependency
communication latency

---

126. Canonical qubit tests

Explicitly test that scheduling accepts:

quantum::ir::qubit::QubitId
quantum::ir::qubit::PhysicalQubitId

and does not require a scheduler-specific qubit identity.

---

127. Routing integration tests

Test:

logical program
    ↓
routing
    ↓
mapped program
    ↓
scheduling

Verify that:

logical identity remains stable
physical identity comes from routing
scheduler only assigns time

---

128. Hardware integration tests

Test targets with deliberately different:

qubit counts
timing resolutions
resource counts
durations
alignment requirements
topologies

The same program must remain semantically valid while schedules differ appropriately.

---

129. QEC integration tests

Test:

different code distances
different stabilizer weights
different ancilla counts
different round counts
different target topologies

No scheduler code should contain assumptions tied to one configuration.

---

130. Dynamic-circuit tests

Test:

measure → condition → operation

with:

different measurement latencies
different classical latencies
different feedback resources

---

131. Distributed tests

Test:

one node
multiple nodes
multiple links
link contention
communication latency
synchronization

---

132. Property testing

Important properties include:

every scheduled dependency is respected
exclusive resources never overlap
capacity is never exceeded
schedule intervals remain valid
canonical operation identities remain unchanged
adding unrelated operations does not invalidate unrelated reservations

---

133. Determinism tests

Run identical inputs multiple times.

Require identical:

schedule ordering
start times
end times
resource reservations
objective result
provenance

when deterministic mode is enabled.

---

134. Scalability tests

Do not merely test a fixed large machine.

Generate increasing workloads based on available test resources.

Scale dimensions such as:

operations
dependency edges
resources
parallelism
QEC rounds
modules
communication links

The test framework must not encode an architectural maximum.

---

135. Regression policy

Every discovered scheduler correctness bug must gain a regression test.

A regression test must reproduce the smallest useful failing case.

---

136. Serialization round-trip

Test:

ScheduleResult
    ↓
encode
    ↓
decode
    ↓
ScheduleResult

and verify semantic equality.

---

137. Compatibility test

The compatibility layer must be tested independently from the algorithm implementation.

This ensures a future algorithm replacement does not break legacy callers.

---

138. API stability

Public APIs should be deliberately small.

Prefer:

Scheduler
SchedulingContext
SchedulingConfig
SchedulingResult
SchedulingStrategy

over exposing every internal implementation structure.

Internal implementation may evolve without breaking users.

---

139. Stable public boundary

The public boundary should conceptually be:

SchedulingInput
        │
        ▼
Scheduler
        │
        ▼
SchedulingResult

with policies and target information supplied through explicit configuration/context.

---

140. Preferred high-level API

Conceptually:

schedule(program, target, policy)

rather than:

schedule(program, 128, 8, 1ns)

The second form hard-codes machine assumptions.

---

141. Target-driven specialization

The correct specialization model is:

same program
     +
target A
     ↓
schedule A

same program
     +
target B
     ↓
schedule B

The source program does not change.

---

142. No machine-specific source programs

Do not require:

program_for_127_qubits
program_for_1024_qubits
program_for_vendor_X

The compiler should specialize from the same semantic program.

---

143. "Anywhere" contract

The scheduler must be architecture-neutral enough to represent targets from:

one qubit

through:

small QPU
large QPU
multi-chip system
distributed quantum computer
quantum network

provided the relevant target capabilities can be described by the surrounding architecture.

---

144. Resource-driven scaling

Scaling must emerge from:

target resource model

not:

scheduler source-code constants

Therefore:

machine size
    → target description
    → resource model
    → routing
    → scheduling

rather than:

machine size
    → scheduler constant

---

145. Compatibility with hardware topology

Scheduling may consume topology-derived constraints, but topology ownership remains with hardware/routing.

Do not duplicate the topology graph in scheduling unless it is an intentionally reduced scheduling view.

---

146. Compatibility with hardware timing

Hardware supplies:

duration
latency
alignment
resolution
availability

Scheduling consumes these.

It does not determine them.

---

147. Compatibility with hardware instruction sets

Instruction-set support is determined by hardware/lowering.

The scheduler may reject a candidate requiring unavailable timing/resource semantics, but it should not synthesize unsupported instructions.

---

148. Compatibility with synthesis

The expected relationship is:

unsupported operation
        │
        ▼
synthesis/decomposition
        │
        ▼
supported operations
        │
        ▼
routing
        │
        ▼
scheduling

Scheduling should not silently become a synthesis engine.

---

149. Compatibility with optimization

Scheduling-stage optimization may modify timing arrangement.

It must not duplicate:

gate algebra
peephole optimization
T-gate synthesis
general gate decomposition

Those belong to the quantum optimization subsystem.

---

150. Semantic invariant

The central correctness requirement is:

scheduled semantics == input semantics

Scheduling may change:

when

but must not accidentally change:

what

---

151. Physical execution invariant

A successful scheduler result means:

the operations can be temporally arranged

under the supplied model.

It does not by itself prove:

the live QPU still has those resources

Final hardware validation remains mandatory.

---

152. Compatibility with changing hardware

If the hardware changes after compilation:

calibration
topology
availability
resource capacity
firmware
instruction set

the schedule may become invalid.

The runtime/hardware validation layer must detect this.

---

153. Partial schedules

The canonical schedule representation may represent:

empty
partial
complete

schedules.

The scheduling engine must distinguish:

successful complete schedule

from:

partial analysis result

A partial result must never be mistaken for executable output.

---

154. Failure atomicity

If scheduling fails:

caller-owned canonical IR

must remain unchanged.

Temporary scheduler state may be discarded.

---

155. Diagnostic explanations

A production scheduler should be capable of generating explanations such as:

Operation O42 could not start at T100 because
resource R7 is reserved until T140.

or:

Operation O81 cannot be scheduled because its
predecessor O64 has not completed.

Diagnostics are not merely logging; they are part of production operability.

---

156. Logging

Scheduler logging should be optional.

Avoid unconditional high-volume logging for large schedules.

Prefer structured diagnostics with configurable verbosity.

---

157. Performance metrics

The scheduler should report, where configured:

planning time
verification time
memory usage
number of operations
number of dependencies
number of resources
number of reservations
number of conflicts
number of reschedules
number of inserted delays

---

158. Benchmark integration

These metrics feed the existing benchmarking architecture.

Benchmarking must not be required to compile the scheduler core.

---

159. Documentation contract

Every scheduler module must document:

what it owns
what it does not own
inputs
outputs
failure modes
complexity
determinism
scalability
integration

This requirement prevents architectural drift.

---

160. File-completion contract

A file is considered complete only when:

- its public types are final for the current API;
- ownership is explicit;
- dependencies are explicit;
- integration points are defined;
- error behavior is defined;
- deterministic behavior is defined;
- thread-safety is defined;
- scalability requirements are defined;
- serialization expectations are defined;
- tests are defined;
- no later file requires semantic edits merely to make the completed file usable.

Adding a new algorithm should normally require changing:

algorithms/mod.rs

or the appropriate plugin/registry boundary, not rewriting foundational types.

---

161. Required source-tree relationship

The intended scheduling tree is:

src/quantum/scheduling/
├── COMPATIBILITY.md
├── mod.rs
├── types.rs
├── errors.rs
├── limits.rs
├── config.rs
├── context.rs
├── result.rs
│
├── ir/
├── resources/
├── timing/
├── policies/
├── planners/
├── constraints/
├── transformations/
├── verification/
├── optimization/
├── qec/
├── dynamic/
├── distributed/
├── adapters/
├── serialization/
├── diagnostics/
├── algorithms/
├── plugins/
├── tests/
└── stabilizer_scheduler.rs

The exact implementation tree may evolve, but ownership boundaries must remain.

---

162. Compatibility with "mod.rs"

"src/quantum/scheduling/mod.rs" remains the composition root.

It should not implement:

scheduler algorithms
resource calendars
dependency analysis
QEC algorithms
hardware access
serialization

Its job is:

module composition
public API boundary
documentation
stable exports

---

163. Compatibility with foundational "types.rs"

"types.rs" may own scheduler-specific:

ScheduleId
DependencyId
ReservationId
EpochId
SchedulerSessionId
TimePoint
Duration
Priority

but must continue importing canonical:

OperationId
ResourceId
QubitId
PhysicalQubitId

rather than redefining them.

The current repository implementation follows this ownership model.

---

164. Compatibility with existing "errors.rs"

"errors.rs" must remain the canonical scheduler error taxonomy.

Other modules should convert their failures into this structured error boundary rather than creating incompatible top-level scheduler errors.

---

165. Compatibility with existing "result.rs"

"result.rs" owns scheduler-facing result aggregation.

It should reference the canonical schedule representation rather than introducing another semantic schedule.

---

166. Compatibility with "quantum::ir::scheduling"

If both:

quantum::ir::scheduling

and:

quantum::scheduling

contain scheduler-related code during migration, "quantum::ir::scheduling" remains the canonical schedule representation boundary.

The engine must adapt to it.

---

167. Migration rule

Migration should follow:

legacy
  │
  ▼
compatibility adapter
  │
  ▼
canonical scheduling contracts
  │
  ▼
production engine

Never:

legacy
  │
  ▼
new independent implementation

if the latter creates competing semantics.

---

168. Deprecation rule

When a legacy API is retained:

1. document it;
2. mark it as compatibility;
3. route it through the canonical implementation;
4. add regression tests;
5. provide the replacement API;
6. remove it only through an explicit compatibility decision.

---

169. No compatibility leakage

Compatibility code must not infect the production scheduler with obsolete assumptions.

For example, a legacy API accepting:

distance

must not cause the generic scheduler to assume:

surface code

---

170. Security

Scheduling is not a security subsystem, but it must obey security constraints.

Do not allow:

unbounded allocation from untrusted input
unchecked deserialization
arbitrary plugin execution through a data-only API

Explicit resource limits may be used to protect compilation services.

---

171. Denial-of-service resistance

When scheduling untrusted programs, deployments should be able to specify:

memory limit
time limit
search limit
operation limit
dependency limit

These are explicit deployment policies.

They must not become universal language limits.

---

172. Serialization security

Deserialized scheduling data must be validated before use.

Never assume serialized schedules are trustworthy.

Validate:

IDs
intervals
dependencies
resources
counts
schema version
target compatibility

---

173. Plugin security

Plugins must have explicit ownership and lifecycle.

The core scheduler must not silently load arbitrary executable code merely because a schedule references a plugin name.

---

174. Reentrancy

A scheduler instance should be safe to invoke according to its documented ownership model without relying on hidden global mutable state.

Separate compilation requests should not contaminate each other.

---

175. No environmental dependence

Scheduling decisions must not depend silently on:

current time
host machine memory layout
environment variables
filesystem contents
network state
thread count

unless those values are explicitly part of the scheduling context/configuration.

---

176. Current time

If wall-clock deadlines matter, current time must be supplied explicitly by the runtime/context.

Pure compilation should remain reproducible.

---

177. Host parallelism

The number of host CPU workers may affect performance.

It must not affect deterministic semantic scheduling results.

---

178. Future quantum architectures

A future architecture may introduce a resource or constraint not known today.

It should be representable through:

generic resource
custom constraint
target capability
adapter
plugin

without changing the meaning of existing operations.

---

179. Extensibility principle

When a new quantum technology appears, the preferred change is:

new hardware capability
+
new adapter
+
possibly new constraint/resource type

not:

rewrite scheduler core

---

180. Compatibility with arbitrary resource types

The scheduler must be able to reason about future resources without requiring every resource to be a qubit.

This prevents:

qubit-centric scheduler architecture

from becoming the scalability ceiling.

---

181. Compatibility with hierarchical resources

Future machines may contain:

network
  └── module
       └── chip
            └── resource

The scheduler should allow hierarchical resource models where required.

---

182. Compatibility with composite resources

An operation may require:

qubit A
+
qubit B
+
control channel C
+
measurement resource D

The scheduler must reserve all required resources atomically.

---

183. Atomic reservation

A candidate operation must not be committed if only some required resources can be reserved.

Either:

all required reservations succeed

or:

none are committed

This prevents partial scheduler state.

---

184. Reservation conflict semantics

A conflict must identify:

operation A
operation B
resource
interval A
interval B

where practical.

This makes diagnostics and debugging scalable.

---

185. Resource capacity semantics

For capacity-limited resources:

usage(t) <= capacity(t)

must hold for the entire relevant interval.

Capacity may itself be time-dependent.

---

186. Time-dependent resources

Resources may transition:

available
busy
disabled
degraded
maintenance

Scheduling must consume the target's availability model.

---

187. Degraded hardware

A degraded resource may remain available with different cost/fidelity characteristics.

The scheduler must not assume:

available = ideal

If the hardware model exposes quality/cost, the policy may incorporate it.

---

188. Fidelity-aware scheduling

A fidelity-aware strategy may trade:

execution time

against:

estimated error

but must use supplied models.

It must not invent hardware error rates.

---

189. Communication-aware scheduling

For distributed targets, scheduling may optimize:

communication latency
link contention
entanglement generation
classical feedback
synchronization

These are resources/constraints, not hidden implementation details.

---

190. QEC-round synchronization

QEC scheduling must represent round boundaries explicitly where required.

For example:

round N
  ↓
syndrome measurement
  ↓
decoder/feedback
  ↓
round N+1

No fixed number of rounds is allowed.

---

191. Logical scheduling

Fault-tolerant scheduling may operate on:

logical operations
logical resources
QEC rounds
logical communication

before physical lowering.

The scheduler architecture must not force every logical operation to be immediately represented as a physical pulse.

---

192. Physical scheduling

After routing/lowering, scheduling may operate on:

physical qubits
channels
hardware resources
timing

The same scheduler abstractions should remain usable.

---

193. Multi-level scheduling

A future compiler may perform:

logical schedule
    ↓
physical schedule
    ↓
pulse schedule

These are separate stages.

The compatibility contract must prevent accidental conflation.

---

194. Schedule refinement

A schedule may be refined:

abstract interval
    ↓
target-aligned interval
    ↓
hardware-native timing

Refinement must preserve the original semantic operation identity.

---

195. Schedule transformation provenance

If a transformation inserts:

delay
padding
DD pulse
alignment adjustment

the result must retain provenance identifying that the transformation occurred.

---

196. Semantic operation identity preservation

Moving an operation:

[0, 10)

to:

[100, 110)

does not create a new semantic operation.

"OperationId" remains unchanged.

The canonical schedule representation explicitly establishes this distinction.

---

197. Schedule identity

A schedule artifact has its own identity.

Two schedules for the same program may differ because of:

target
calibration
policy
algorithm
seed
resource availability
objective

Therefore:

ProgramId != ScheduleId

---

198. Compatibility with canonical hashing

Schedule hashing must operate over canonical semantic content.

It must not depend on:

memory addresses
collection capacity
hash-table layout
temporary IDs

Canonical ordering must be established before semantic hashing.

---

199. Compatibility with IR versioning

A schedule must identify the IR version it was created from where the canonical schema requires it.

A scheduler must reject incompatible IR versions rather than silently interpreting different semantics.

---

200. Compatibility with future IR versions

The scheduler should be designed so that adding an IR operation does not require rewriting scheduling infrastructure if the new operation can be represented through existing scheduling descriptors.

Unsupported semantics should result in an explicit compatibility error.

---

201. Unsupported operations

When an operation cannot be scheduled because the scheduling subsystem lacks the required semantic information:

UnsupportedOperation

or the canonical equivalent must be returned.

Do not silently drop it.

---

202. Missing duration

If duration is mandatory for the selected scheduling mode and absent:

MissingSchedulingInformation

must be returned.

No arbitrary default duration.

---

203. Missing resource

If the operation's target execution requires a resource and none is supplied:

MissingSchedulingInformation

or a target-compatibility error must be returned.

---

204. Unknown resource

An unknown resource may be acceptable at an abstract scheduling stage if the resource namespace explicitly permits it.

It must be rejected before hardware execution if the target cannot resolve it.

---

205. Unknown dependency

An operation referencing a nonexistent predecessor must be rejected.

Do not silently treat it as a root node.

---

206. Cycles

Static dependency cycles must be rejected.

A cycle may only be accepted when explicitly represented as a valid dynamic/runtime control construct rather than an accidental dependency cycle.

---

207. Self-dependency

An operation cannot depend on itself unless a dedicated dynamic semantic explicitly defines such behavior.

Normal scheduling must reject it.

---

208. Zero-duration operations

Zero-duration operations may be legal.

They must be represented consistently.

The scheduler must not accidentally convert zero duration into a positive duration.

---

209. Negative duration

Negative duration is invalid.

It must never be represented as a valid scheduling interval.

---

210. Time overflow

If:

start + duration

cannot be represented, return a structured overflow error.

Never wrap.

---

211. Alignment

Alignment requirements must be explicit.

Examples:

channel alignment
operation alignment
measurement alignment
frame alignment
sample alignment

The scheduler must not assume all targets use the same alignment.

---

212. Measurement latency

Measurement completion time and classical-result availability may differ.

The scheduler must model:

measurement
    ↓
readout completion
    ↓
classical result
    ↓
feedback

when the target requires it.

---

213. Reset latency

Reset may require explicit duration/resource constraints.

The scheduler must consume target-supplied semantics.

---

214. Conditional execution

Conditions must be represented as dependencies/constraints.

The scheduler must not evaluate unknown runtime values during static compilation.

---

215. Runtime event compatibility

Runtime events must be represented as scheduling boundaries.

The static scheduler may produce:

event wait

rather than pretending the event's completion time is known.

---

216. Communication latency

Communication latency must be target-supplied.

Never hard-code:

network latency

into scheduling.

---

217. Distributed synchronization

Synchronization may require explicit shared resources or barriers.

The scheduler must model synchronization rather than assuming perfect global clocks.

---

218. Clock domains

Different target resources may use different clock domains.

The timing adapter must expose required synchronization relationships.

The generic scheduler must not assume one universal hardware clock unless the target declares one.

---

219. Resource calendars

A resource calendar may contain:

availability intervals
reservations
maintenance
calibration windows
exclusion windows

The scheduler must consume calendars rather than reconstruct hardware state.

---

220. Event calendar efficiency

For large schedules, resource calendars should support efficient search for:

next available interval
conflicting reservation
availability transition

without scanning every schedule entry.

---

221. Algorithm replacement

A caller should be able to change:

ASAP → ALAP

or:

list → critical-path

without changing:

canonical IR
routing
hardware model
schedule representation

---

222. Policy replacement

A policy should be replaceable without rewriting resource models.

For example:

resource-aware policy

and:

fidelity-aware policy

consume the same scheduling context.

---

223. Planner replacement

Planner contracts should remain stable while implementations evolve.

A planner should return a candidate result that can be independently verified.

---

224. Verification after optimization

If schedule optimization changes a valid schedule:

candidate
   ↓
optimization
   ↓
verification again

Never assume the optimization preserved all constraints merely because the input was valid.

---

225. Transformation atomicity

A failed transformation must not leave the schedule partially transformed.

Use transaction-like construction.

---

226. Provenance after transformation

Each transformed result must retain enough information to identify:

original schedule
transformation
configuration
target

where required.

---

227. Compatibility with compiler diagnostics

Scheduling errors should retain enough source/operation identity for compiler diagnostics to locate the originating operation.

The scheduler should not need source-language parsing to produce this information.

---

228. Source mapping

Source locations belong to IR metadata.

Scheduling should preserve operation identity so downstream diagnostics can resolve:

OperationId
    ↓
source metadata

---

229. No parser dependency

The scheduling core must not import:

ANTLR parser implementation
OpenQASM lexer
Zamani frontend parser

It consumes canonical IR/adapters.

---

230. No frontend ownership

Frontend owns:

syntax
parsing
source diagnostics
language grammar

Scheduling owns:

temporal/resource placement

---

231. No backend ownership

Backend owns:

device communication
submission
execution
job lifecycle
provider authentication

Scheduling only prepares the schedule.

---

232. No simulator ownership

Simulator behavior belongs to simulator infrastructure.

Scheduling may consume a simulator target.

---

233. No benchmark ownership

Scheduling reports scheduling metrics.

Benchmarking decides:

experiment
workload
shots
statistical method
benchmark protocol

---

234. No QEC decoder ownership

QEC decoder remains outside scheduling.

Scheduling may model decoder latency or feedback resources if supplied by the QEC/hardware contract.

---

235. No noise-model duplication

Noise estimates belong to ZQN/noise infrastructure.

Scheduling consumes them through an explicit interface.

---

236. No topology duplication

Topology belongs to routing/hardware.

Scheduling may consume topology-derived constraints.

---

237. No calibration duplication

Calibration belongs to hardware.

Scheduling consumes calibration-derived scheduling parameters.

---

238. No vendor SDK dependency

Scheduling core must compile without vendor SDKs.

Vendor adapters belong outside the core.

---

239. Offline compilation

The scheduler should be capable of compiling entirely offline when all required:

IR
target description
timing
resources
constraints

are locally available.

It should not require network access.

---

240. Runtime hardware discovery

If a target must be discovered dynamically, that discovery occurs outside the scheduler.

Then:

discovered target
    ↓
SchedulingContext
    ↓
scheduler

---

241. Configuration immutability

Scheduling configuration should be immutable for one scheduling invocation.

Changing policy halfway through an algorithm is prohibited unless explicitly implemented as a dynamic scheduling strategy.

---

242. Configuration provenance

The schedule should be able to identify the configuration used to generate it.

---

243. API compatibility with Rust ownership

Prefer ownership patterns compatible with Rust's safe borrowing model.

Do not introduce:

self-referential unsafe structures

for performance.

---

244. No unsafe optimization

Performance improvements must use safe Rust techniques:

efficient collections
preallocation when justified
iterative algorithms
parallelism
caching
partitioning

rather than unsafe memory manipulation.

---

245. Large-ID compatibility

Semantic IDs must not be limited to machine-size assumptions.

Scheduler-owned IDs should be sufficiently wide and independent of host pointer width.

The existing scheduling type design uses stable "u64" scheduler-specific identifiers and does not treat them as collection indices.

---

246. Time-range compatibility

Scheduling time must support large ranges without immediately constraining execution to small-machine assumptions.

Checked wide integer representations are preferred where already compatible with the canonical timing model.

---

247. Collection capacity is not semantic capacity

A Rust "Vec" capacity may be finite.

That is an implementation detail.

It must never be exposed as:

maximum number of qubits

or equivalent.

---

248. Explicit resource accounting

Resource utilization should be computed from actual reservations.

Do not infer:

all target resources are used

merely because they exist.

---

249. Sparse machine compatibility

A target may contain many resources but the program may use very few.

The scheduler must scale with the resources relevant to the workload where possible.

---

250. Dense machine compatibility

A target may expose dense connectivity and abundant resources.

The scheduler must not artificially serialize operations merely because an old small-machine assumption exists.

---

251. Highly constrained machine compatibility

A target may have:

few channels
strict timing
high communication cost

The scheduler must adapt through the resource model and constraints.

---

252. Highly parallel machine compatibility

A target may permit large parallelism.

The scheduler should exploit it where policy permits.

It must not impose a hidden maximum parallelism.

---

253. Parallelism metric

The result should be able to report parallelism without requiring a fixed machine-size model.

---

254. Makespan

Makespan is:

maximum scheduled finish time

over the relevant schedule span.

It must use canonical time semantics.

---

255. Depth

Depth must be explicitly defined for the selected workload/model.

It must not automatically mean:

number of Vec layers

unless the model defines that representation.

---

256. Idle time

Idle time should be computed against explicitly defined resources and intervals.

The scheduler must distinguish:

resource unavailable

from:

resource idle by policy

---

257. Resource utilization

Resource utilization must use:

actual reservation intervals

and target resource availability.

---

258. Schedule quality

A schedule result should expose measurable quality information such as:

makespan
depth
idle time
resource utilization
communication cost
estimated fidelity
verification status

---

259. Explainability

For any significantly delayed operation, diagnostics should preferably identify the dominant blocking factor.

This is essential for debugging large schedules.

---

260. Production readiness gate

"src/quantum/scheduling/" must NOT be declared production-ready until:

[ ] canonical identity ownership is enforced
[ ] no duplicate Schedule exists
[ ] no hard-coded machine-size limits exist
[ ] canonical timing ownership is clear
[ ] resource model is explicit
[ ] dependency model is explicit
[ ] routing boundary is explicit
[ ] hardware boundary is explicit
[ ] QEC boundary is explicit
[ ] dynamic scheduling exists
[ ] distributed scheduling contract exists
[ ] ASAP exists
[ ] ALAP exists
[ ] list scheduling exists
[ ] critical-path support exists
[ ] resource-constrained scheduling exists
[ ] verification exists
[ ] deterministic mode exists
[ ] explicit limits exist
[ ] diagnostics exist
[ ] provenance exists
[ ] serialization compatibility exists
[ ] plugin boundary exists
[ ] no unsafe exists
[ ] Rust 1.97/1.97.1 builds
[ ] unit tests exist
[ ] integration tests exist
[ ] property tests exist
[ ] regression tests exist
[ ] scalability tests exist
[ ] determinism tests exist
[ ] routing integration works
[ ] hardware integration works
[ ] QEC integration works
[ ] runtime integration works
[ ] benchmark integration works

---

261. Definition of production-ready

The scheduler is production-ready when it can take:

canonical Zamani quantum program
+
validated target/context
+
explicit scheduling policy

and produce:

verified canonical schedule

without:

hard-coded machine assumptions
unsafe code
vendor coupling
qubit-identity duplication
semantic IR duplication
hidden mutable state
silent fallbacks
unverified transformations

---

262. Final architecture

The final production pipeline is:

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
                    "WHERE?"
                         │
                         ▼
                scheduling::adapters
                         │
          ┌──────────────┼──────────────┐
          │              │              │
          ▼              ▼              ▼
      IR adapter     hardware       QEC/ZQN
          │          adapter        adapters
          └──────────────┼──────────────┘
                         ▼
                 SchedulingContext
                         │
             ┌───────────┼───────────┐
             ▼           ▼           ▼
        dependencies  resources    timing
             │           │           │
             └───────────┼───────────┘
                         ▼
                    constraints
                         │
                         ▼
                      policy
                         │
                         ▼
                     planner
                         │
                         ▼
                    algorithm
                         │
                         ▼
                   candidate
                         │
                         ▼
                  transformations
                         │
                         ▼
                    verification
                         │
                         ▼
            quantum::ir::scheduling::Schedule
                         │
                         ▼
                 hardware validation
                         │
                         ▼
                  hardware lowering
                         │
                         ▼
                       runtime
                         │
                         ▼
                        QPU

---

263. The ultimate compatibility invariant

The entire architecture must preserve this equation:

Zamani program
+
target description
+
constraints
+
policy
=
target-specific schedule

not:

Zamani program
+
hard-coded machine assumptions
=
schedule

Therefore:

same Zamani source
        │
        ├── Target A
        │      └── Schedule A
        │
        ├── Target B
        │      └── Schedule B
        │
        ├── Target C
        │      └── Schedule C
        │
        └── future target
               └── Schedule N

while:

program semantics

remain unchanged.

---

264. Final ownership statement

The production scheduling subsystem must always maintain:

IR
    = WHAT

routing
    = WHERE

scheduling
    = WHEN

hardware
    = CAN IT EXECUTE?

QEC
    = WHAT FAULT-TOLERANT STRUCTURE IS REQUIRED?

ZQN
    = WHAT NOISE/QUALITY INFORMATION APPLIES?

runtime
    = EXECUTE

benchmarking
    = MEASURE

The scheduler must never absorb the responsibilities of the other systems merely because doing so appears convenient.

---

265. Final non-negotiable rules

1. "quantum::ir::qubit::QubitId" is the canonical logical qubit identity.
2. "quantum::ir::qubit::PhysicalQubitId" is the canonical physical qubit identity.
3. No scheduler-specific qubit identity may replace either.
4. "quantum::ir::core::identity::OperationId" remains canonical.
5. "quantum::ir::scheduling" remains the canonical schedule representation boundary.
6. "quantum::scheduling" owns scheduling-engine behavior.
7. Routing owns logical-to-physical mapping.
8. Hardware owns target capabilities and physical constraints.
9. QEC owns QEC semantics.
10. ZQN/noise infrastructure owns noise models.
11. Runtime owns execution.
12. Benchmarking owns benchmark protocols.
13. No machine-size constants.
14. No vendor assumptions.
15. No fixed qubit count.
16. No fixed operation count.
17. No fixed resource count.
18. No fixed topology.
19. No fixed number of QEC rounds.
20. No fixed gate arity.
21. No hard-coded timing unit.
22. No hidden global scheduler state.
23. No hidden randomness.
24. No silent fallback for missing semantic information.
25. No unsafe Rust.
26. Rust 1.97/1.97.1 compatibility is mandatory.
27. All schedules must be independently verifiable.
28. All public compatibility APIs must have migration paths.
29. New algorithms must be replaceable without changing semantic ownership.
30. The same Zamani program must remain target-independent.

---

266. Completion statement

When this contract is satisfied, "src/quantum/scheduling/" is architecturally compatible with Zamani's broader quantum stack and can evolve from small-machine scheduling toward large-scale, fault-tolerant, modular, distributed and future quantum systems without encoding the physical size of those systems into the Zamani language or scheduler.

The goal is therefore not:

"support a bigger quantum computer"

but:

"make machine size a property of the target,
not a property of the Zamani program."

That is the compatibility boundary required for:

write once
compile many targets
scale with available resources
preserve semantics
remain safe
remain deterministic when requested
remain maintainable

for the lifetime of the Zamani quantum architecture.This contract also resolves the biggest repository-level issue I found: the new scheduler directory must not create a second competing Schedule/timing/qubit identity architecture. The canonical IR already explicitly assigns those ownership boundaries, while the existing quantum::scheduling implementation has already started using canonical OperationId, ResourceId, QubitId, and PhysicalQubitId.

I would treat this COMPATIBILITY.md as the contract that freezes those boundaries before the remaining scheduling files are implemented.