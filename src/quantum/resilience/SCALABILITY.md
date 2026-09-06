Worked for 14s

I checked the current Benwellonedge28/Zamani repository through the GitHub integration and specifically verified the canonical qubit-identity direction: quantum::ir::qubit owns QubitId and PhysicalQubitId; compatibility aliases point back to it.  The repository also has explicit canonical physical-qubit hashing and logical/physical separation, which is important for a scalable resilience layer.

The scalability design below also follows the important execution-layer separation used by OpenQASM: logical program intent is distinct from hardware-specific mapping/scheduling, and timing intent can remain independent of calibrated hardware durations.  Current quantum execution systems likewise already distinguish job, batch, and session execution because workload scale and execution characteristics differ; resilience therefore must treat execution capacity as discovered/runtime state rather than fixed constants. 

Below is the complete proposed src/quantum/resilience/SCALABILITY.md. It is normative: implementation files should conform to it rather than independently inventing scalability rules.

# Zamani Quantum Resilience — Scalability Specification

**Path:** `src/quantum/resilience/SCALABILITY.md`

**Status:** Normative  
**Subsystem:** `quantum::resilience`  
**Language:** Rust  
**Required Rust version:** Rust 1.97 / Rust 1.97.1  
**Safety requirement:** `unsafe` code is forbidden  
**Canonical qubit identity:** `quantum::ir::qubit`  
**Primary objective:** Write once, execute across any supported quantum resource configuration without embedding machine-size assumptions.

---

## 1. Purpose

This document defines the scalability contract for Zamani's quantum resilience subsystem.

The subsystem MUST scale across:

- one physical qubit;
- small quantum processors;
- large quantum processors;
- modular quantum processors;
- logical-qubit systems;
- fault-tolerant quantum computers;
- heterogeneous quantum systems;
- multiple QPUs;
- distributed quantum systems;
- quantum/classical execution fabrics;
- simulators;
- emulators;
- future quantum architectures not yet known to Zamani.

The implementation MUST NOT require source-code changes merely because the target machine has a different:

- number of qubits;
- number of logical qubits;
- topology;
- gate set;
- connectivity;
- calibration;
- timing model;
- QEC configuration;
- backend;
- provider;
- execution model;
- number of execution nodes;
- number of quantum processors;
- number of classical control resources;
- resource capacity.

The governing principle is:

> A Zamani quantum program describes computational intent. Resource discovery, mapping, scheduling, adaptation, recovery, and execution determine how that intent is realized on the currently available resources.

---

# 2. Core scalability invariant

The resilience subsystem MUST obey the following invariant:

> Increasing or decreasing available resources MUST change execution planning and resource realization, not the semantic identity of the Zamani program.

Therefore:

```text
Same Zamani program
        |
        v
Same canonical semantic intent
        |
        +------------------------------+
        |                              |
        v                              v
Small machine                     Large machine
        |                              |
        v                              v
Different realization             Different realization
        |                              |
        +---------------+--------------+
                        |
                        v
                 Same semantics

The implementation MUST NOT encode a machine-specific implementation into the program's semantic identity.


---

3. Definition of "infinite" scalability

Zamani cannot literally guarantee execution on an infinite physical machine.

Therefore "infinity" means:

> The architecture has no artificial upper bound imposed by the resilience subsystem; execution is limited only by the resources and constraints exposed by the surrounding system and the physical/computational limits of the target environment.



The resilience subsystem MUST NOT introduce artificial limits such as:

const MAX_QUBITS: usize = 10000;
const MAX_BACKENDS: usize = 32;
const MAX_RETRIES: usize = 3;
const MAX_INCIDENTS: usize = 1024;
const MAX_TELEMETRY_EVENTS: usize = 1_000_000;

unless such limits are explicitly supplied by a runtime/configuration/resource policy.

Even then, the value MUST be treated as an operational constraint rather than a property of quantum computing itself.


---

4. Scalability dimensions

Resilience MUST scale along multiple independent dimensions.

4.1 Qubit-count scalability

1
→ small N
→ large N
→ very large N
→ dynamically discovered N

No algorithm may depend on a fixed number of qubits.


---

4.2 Logical-qubit scalability

The system MUST support:

physical qubit
        ↓
encoded qubit
        ↓
logical qubit
        ↓
logical-qubit array
        ↓
logical-qubit fabric

The resilience layer MUST distinguish physical and logical identity.

Canonical identity MUST come from:

quantum::ir::qubit

Specifically, implementations MUST use the canonical:

quantum::ir::qubit::QubitId
quantum::ir::qubit::PhysicalQubitId

where those identities are required.

No resilience module may introduce a competing canonical QubitId.


---

5. Logical versus physical scalability

The following distinction is mandatory:

Logical program identity
        |
        v
Canonical IR
        |
        v
Logical qubits
        |
        v
Routing / allocation
        |
        v
Physical qubits
        |
        v
Hardware execution

Resilience MUST NOT assume:

logical q0 == physical q0

or:

logical index == physical index

or:

QubitId == PhysicalQubitId

unless explicitly established by a lower-layer mapping.

A machine may map:

logical A -> physical 17
logical B -> physical 4
logical C -> physical 203

and resilience MUST operate correctly.


---

6. Canonical identity rule

The repository's canonical identity authority is:

quantum::ir::qubit

Existing compatibility layers may expose aliases, but resilience MUST prefer the canonical type.

The resilience subsystem MUST NOT create:

pub type QubitId = u64;

or:

pub struct ResilienceQubitId(...);

as a competing identity model.

If a resource identity is needed, it MUST either:

1. use the canonical IR identity; or


2. use an explicitly different resource-domain identifier whose semantics are clearly distinct.




---

7. Resource discovery

All machine-size information MUST come from discovered or configured capabilities.

The resilience subsystem MUST consume the hardware capability contract instead of maintaining its own machine inventory.

Examples of dynamic information include:

available qubits;

available logical qubits;

unavailable qubits;

topology;

connectivity;

native operations;

measurement capability;

reset capability;

mid-circuit measurement;

timing;

control channels;

QEC support;

decoder support;

memory;

execution capacity;

queue capacity;

parallelism;

provider capabilities;

simulator capabilities.


The source of truth remains the relevant lower-level subsystem.


---

8. No hard-coded hardware knowledge

Core resilience code MUST NOT contain logic such as:

match backend {
    Backend::SpecificVendor => ...
}

for normal operation.

It MUST NOT contain:

if qubit_id == 7 { ... }

or:

if qubit_count > 127 { ... }

or:

retry_three_times();

or:

if fidelity < 0.99 { ... }

Instead, these values must come through:

Capabilities
Policy
Constraints
Objectives
Budgets
Telemetry
Calibration
Discovery
Execution context


---

9. Resource model

model/resource.rs defines resource identity.

It MUST be able to represent resources without assuming their quantity.

Conceptually:

Resource
├── backend
├── device
├── quantum processor
├── logical qubit
├── physical qubit
├── coupling
├── control channel
├── measurement channel
├── execution slot
├── classical processing resource
└── distributed execution resource

The model MUST be extensible.

Adding a new resource category MUST NOT require rewriting the resilience planner.


---

10. Dynamic resource sets

Resource collections MUST be modeled as dynamic sets.

Good:

AvailableResources
    -> iterator
    -> query
    -> capability filter
    -> selection

Bad:

[Resource; 1000]

Bad:

const MAX_RESOURCES: usize = 4096;

Bad:

resources[127]

unless 127 is supplied dynamically as a discovered identifier.


---

11. Avoid index-based architecture

Indices may be used internally where the underlying subsystem explicitly defines them as indices.

However, resilience MUST NOT treat an index as a globally stable physical identity.

For example:

index 0

does not automatically mean:

physical qubit 0

Identity belongs to the canonical IR/hardware contracts.


---

12. Scalability and topology

Topology MUST be queried dynamically.

The resilience subsystem MUST support:

single node
linear
ring
grid
heavy-hex-like topology
all-to-all
modular
hierarchical
sparse
dynamic
heterogeneous
distributed

No routing algorithm belongs in resilience.

Resilience requests routing through the routing subsystem.


---

13. Topology changes

A topology may change because of:

hardware failure;

qubit quarantine;

calibration;

maintenance;

dynamic allocation;

device migration;

modular interconnect changes;

network failures;

distributed resource loss.


When topology changes:

Detect
  ↓
Diagnose
  ↓
Determine affected resources
  ↓
Request rerouting
  ↓
Request rescheduling
  ↓
Request recompilation if necessary
  ↓
Verify

Resilience MUST NOT implement an independent topology engine.


---

14. Degradation scalability

A large quantum system MUST be allowed to continue operating with fewer usable resources where the program constraints permit it.

Example:

Available resources:
1000 physical qubits

After degradation:
960

Then:

960 -> 900 -> 850 -> 700

The program may continue if the execution constraints remain satisfiable.

If the workload requires more resources than remain available:

DEFER
MIGRATE
RECOMPILE
PARTITION
ESCALATE
ABORT

must be selected according to policy.


---

15. Graceful degradation

model/degradation.rs MUST represent degradation without assuming a fixed degradation level.

It must support:

partial resource loss;

performance degradation;

fidelity degradation;

timing degradation;

connectivity degradation;

QEC degradation;

classical-resource degradation;

backend degradation;

network degradation.


The degradation model MUST be composable.

Example:

Hardware degradation
+
Topology degradation
+
Timing degradation
+
QEC degradation
=
combined capability state


---

16. Capability scalability

model/capability.rs MUST represent capabilities rather than machine models.

Capabilities are queried from:

quantum::hardware

and related execution contracts.

A capability may describe:

supports measurement
supports reset
supports mid-circuit measurement
supports dynamic control
supports QEC
supports migration
supports pulse control
supports operation X
supports operation Y
supports topology Z

The resilience layer does not assume which capabilities exist.


---

17. Capability negotiation

Every adaptive action MUST negotiate against current capabilities.

For example:

Plan:
reroute

        ↓

Current capabilities:
    connectivity = ...
    routing support = ...

        ↓

Feasible?

If capabilities change after planning:

OLD PLAN
   ↓
REVALIDATE
   ↓
VALID → execute
INVALID → replan

A stale plan MUST NOT execute merely because it was previously valid.


---

18. Resource-aware planning

planning/feasibility.rs MUST validate plans against the current resource model.

Feasibility includes:

available qubits;

available logical qubits;

connectivity;

gate support;

timing;

QEC requirements;

mitigation overhead;

execution slots;

memory;

classical processing;

backend constraints;

policy budgets.



---

19. Planning complexity

Planning MUST avoid unnecessary global work.

For a local fault affecting a small region, resilience SHOULD prefer:

local diagnosis
→ local adaptation
→ local verification

rather than:

rebuild entire universe

However, global replanning MUST remain available when local adaptation cannot preserve correctness.


---

20. Locality versus global recovery

Recovery scopes should be represented explicitly.

Possible scopes:

operation
region
logical qubit
physical qubit
QEC block
circuit
execution
device
backend
distributed workload

The scope MUST be discovered from the incident and plan.

It MUST NOT be fixed.


---

21. Incremental adaptation

Whenever semantics permit, resilience SHOULD adapt incrementally.

Examples:

one failed physical qubit
→ remap affected logical region

rather than:

recompile entire workload

However, the implementation MUST NOT preserve incremental behavior when doing so violates semantic or safety constraints.

Correctness takes priority over optimization.


---

22. Incremental recompilation

adaptation/recompilation.rs MUST support:

local recompilation
regional recompilation
whole-program recompilation

The scope is selected dynamically.

The compiler remains responsible for compilation.

Resilience only requests recompilation.


---

23. Incremental optimization

adaptation/reoptimization.rs MUST support:

affected-region optimization
global optimization
fault-tolerant optimization
target-specific optimization

Optimization remains owned by:

quantum::optimization

Resilience MUST NOT duplicate optimization passes.


---

24. Incremental routing

adaptation/rerouting.rs MUST support:

local rerouting
regional rerouting
global rerouting

depending on the topology change.

Routing remains owned by:

quantum::routing


---

25. Incremental scheduling

adaptation/rescheduling.rs MUST support:

local rescheduling
regional rescheduling
global rescheduling

depending on the affected resources and timing constraints.

Scheduling remains owned by:

quantum::scheduling

OpenQASM's separation between timing intent and hardware-specific calibrated duration reinforces this architectural boundary: resilience should trigger schedule adaptation while the scheduling layer determines the actual valid schedule.


---

26. QEC scalability

Resilience MUST NOT implement QEC algorithms.

QEC remains responsible for:

encoding;

syndrome extraction;

decoding;

correction;

code-specific operations;

logical error processing.


Resilience decides whether QEC configuration should change.

Possible actions include:

change code
change distance
change decoder
change layout
change ancilla allocation
change syndrome strategy
migrate logical qubit
quarantine region

The actual QEC implementation remains outside resilience.


---

27. Logical error scaling

A physical system becoming larger does not automatically mean that resilience should perform proportionally more work.

Resilience SHOULD operate on aggregated logical state where possible.

For example:

many physical faults
        ↓
QEC
        ↓
logical health signal
        ↓
resilience

rather than forcing resilience to independently reason about every underlying physical event.


---

28. Fault aggregation

model/incident.rs MUST support aggregation.

Example:

1000 correlated low-level events
            ↓
      one incident

The aggregation algorithm MUST be configurable and policy-driven.

It MUST preserve the underlying evidence needed for diagnosis and verification.

Aggregation MUST NOT destroy provenance.


---

29. Streaming telemetry

Telemetry MUST support streaming.

It MUST NOT require loading an unlimited event history into memory.

Bad:

let all_events = collect_everything_forever();

Good:

stream
→ normalize
→ aggregate
→ detect
→ persist selected evidence

Large deployments MUST be able to process telemetry incrementally.


---

30. Backpressure

telemetry/collector.rs MUST support backpressure.

If event production exceeds processing capacity, the system MUST have a defined policy for:

buffering;

sampling;

aggregation;

prioritization;

dropping noncritical telemetry;

preserving critical events;

escalation.


Critical fault evidence MUST NOT be silently discarded.


---

31. Telemetry prioritization

Events SHOULD have priority classes such as:

critical
high
normal
low
debug

These priorities MUST be policy-driven.

No fixed event count may be assumed.


---

32. Telemetry aggregation

At very large scale, individual event storage may become impractical.

The system SHOULD support:

counts
histograms
summaries
time windows
sketches
correlation identifiers
sampled evidence

while preserving exact evidence for safety-critical incidents where required.


---

33. Distributed scalability

Resilience MUST support distributed execution.

Conceptually:

Resilience Coordinator
                            |
          +-----------------+-----------------+
          |                 |                 |
        QPU A             QPU B             QPU C
          |                 |                 |
        local             local             local
      resilience        resilience        resilience

Local resilience SHOULD handle local failures.

Global resilience SHOULD coordinate only when the incident crosses resource boundaries.


---

34. Distributed ownership

coordination/ownership.rs MUST define who owns an active recovery operation.

An operation MUST NOT be executed simultaneously by multiple controllers unless explicitly designed for that behavior.


---

35. Leases

coordination/lease.rs MUST support expiring ownership.

A stale controller MUST NOT retain authority indefinitely.

Leases MUST support:

ownership identity;

expiration;

renewal;

fencing;

cancellation;

failure detection.



---

36. Fencing

Distributed recovery commands SHOULD carry an ownership/epoch/fencing token.

A stale command MUST be rejected.

This prevents:

Controller A owns recovery
Controller A crashes
Controller B takes ownership
Controller A returns
Controller A issues stale command

from corrupting execution state.


---

37. Distributed consistency

coordination/consensus.rs is an abstraction boundary.

It MUST NOT introduce a bespoke consensus algorithm merely because resilience is distributed.

If consensus is required, use an appropriate existing coordination mechanism.

The resilience subsystem should depend on the coordination contract rather than inventing a new distributed protocol.


---

38. Heterogeneous scalability

Zamani MUST support different quantum technologies through capability abstraction.

Examples include systems with different:

native gate sets;

connectivity;

timing;

measurement;

control;

QEC capabilities;

execution models.


Resilience MUST NOT assume one technology is the universal model.


---

39. Backend scalability

adaptation/backend_selection.rs MUST operate on capability compatibility.

Conceptually:

Program requirements
        +
Resilience requirements
        +
Current resource state
        ↓
Candidate backends
        ↓
Capability filtering
        ↓
Policy filtering
        ↓
Cost/risk ranking
        ↓
Selection

The core MUST remain provider-neutral.


---

40. Backend migration

Migration MUST preserve:

program identity
semantic intent
logical identity
provenance
policy
verification requirements

A migration MUST NOT silently change the computation.


---

41. Simulator scalability

The simulator is a valid execution target for resilience testing.

However, resilience MUST NOT assume that a simulator can scale exponentially with qubit count.

Simulation capabilities must be discovered.

For example, a simulator may support:

state vector
tensor network
stabilizer
trajectory
hybrid
distributed simulation

The resilience layer consumes the simulator capability contract.


---

42. Classical scalability

Quantum resilience itself contains substantial classical computation.

Therefore scalability MUST cover:

CPU;

memory;

network;

storage;

telemetry processing;

planning;

diagnosis;

verification;

serialization;

history;

distributed coordination.


A system can fail to scale even when the QPU itself is large if the resilience controller becomes a classical bottleneck.


---

43. Avoid centralized bottlenecks

A single global controller MUST NOT be required for every event in a very large system.

The architecture SHOULD support:

local processing
regional aggregation
global coordination

as needed.

The exact hierarchy MUST remain dynamically configurable.


---

44. Hierarchical resilience

Large deployments SHOULD support:

operation-level resilience
        ↓
logical-block resilience
        ↓
QPU-level resilience
        ↓
backend-level resilience
        ↓
fleet-level resilience
        ↓
distributed-system resilience

Each level should handle failures within its authority before escalating.


---

45. Escalation

policy/escalation.rs determines when local recovery is insufficient.

Example:

local qubit failure
→ local remapping

multiple correlated failures
→ regional rerouting

device failure
→ device migration

backend failure
→ backend migration

multi-backend failure
→ distributed recovery

no safe realization
→ controlled abort

These are examples, not hard-coded rules.


---

46. Memory scalability

Resilience MUST NOT maintain unbounded in-memory state.

Potentially unbounded structures include:

telemetry;

history;

incidents;

traces;

provenance;

learning data.


These MUST use explicit retention/storage policies.


---

47. Streaming state

Where possible:

stream
→ aggregate
→ persist
→ release

instead of:

collect indefinitely
→ process later


---

48. Chunking

Large data structures SHOULD support chunked processing.

Examples:

large telemetry stream
large circuit
large provenance record
large checkpoint
large distributed result

The implementation MUST NOT assume the complete workload can always fit in memory.


---

49. Lazy evaluation

Where useful, APIs SHOULD expose iterators or streaming interfaces rather than forcing eager allocation.

Examples:

resources()
events()
incidents()
operations()
qubits()
candidate_plans()

must be able to operate over large collections.


---

50. Parallelism

Resilience MAY parallelize independent work.

Examples:

diagnose independent regions
verify independent regions
evaluate candidate plans
process independent telemetry partitions

But parallel execution MUST preserve deterministic semantics when deterministic mode is requested.


---

51. Deterministic parallelism

If deterministic mode is enabled:

same input
+
same resource snapshot
+
same telemetry
+
same policy
+
same seed

MUST produce the same externally observable planning result.

Parallel execution MUST NOT make output dependent on nondeterministic completion order.


---

52. Ordering

When multiple events have equivalent timestamps, the system MUST use a deterministic secondary ordering when deterministic mode is required.

Possible ordering components include:

event identity
source identity
sequence number
causal order

The exact mechanism belongs to the telemetry/event contract.


---

53. Randomness

Randomized resilience algorithms MUST NOT use uncontrolled randomness.

Randomness MUST be supplied through an explicit source/seed abstraction.

This is required for:

reproducibility;

testing;

debugging;

deterministic replay.



---

54. Planning scalability

The planner MUST NOT evaluate every possible recovery plan when the search space is enormous.

It SHOULD support:

pruning;

feasibility filtering;

policy filtering;

cost bounds;

dominance filtering;

staged search;

configurable search depth;

incremental planning.


These mechanisms MUST be resource-aware.


---

55. Planning correctness

Optimization of planning time MUST NEVER remove mandatory safety checks.

The following remain mandatory:

semantic validity
capability validity
policy validity
security validity
verification requirements


---

56. Candidate plan explosion

The number of candidate recovery strategies may grow rapidly with system size.

Therefore:

candidate generation
→ early feasibility filtering
→ policy filtering
→ cost/risk ranking
→ bounded evaluation

SHOULD be used.


---

57. Cost model scalability

planning/cost.rs MUST not assume a single scalar cost.

A plan may have:

latency
resource cost
shot cost
QPU time
classical CPU
memory
energy
fidelity impact
logical error risk
migration risk
compilation cost

The objective system determines how these dimensions are combined.


---

58. Multi-objective scaling

policy/objectives.rs MUST support multiple simultaneous objectives.

The system MUST NOT hard-code:

minimize latency

as the universal objective.

One workload may prioritize:

correctness > fidelity > latency

while another may prioritize:

availability > cost > latency

The policy supplies the ordering or weighting.


---

59. Budget scalability

policy/budgets.rs MUST represent dynamic budgets.

Possible budgets:

time
shots
QPU time
compilation time
memory
network bandwidth
energy
recovery attempts
mitigation overhead
migration count

Budgets MUST be supplied externally.


---

60. Retry scalability

Retry behavior MUST NOT use fixed retry counts.

Bad:

for _ in 0..3 {
    retry();
}

Good:

retry policy
+
incident classification
+
remaining budget
+
expected success probability
+
semantic safety

determine whether another attempt is valid.


---

61. Infinite retry prevention

Although retries cannot be hard-coded, the system MUST prevent infinite recovery loops.

This should be achieved using dynamic:

budgets;

deadlines;

attempt identities;

progress tracking;

repeated-failure detection;

escalation policy.



---

62. Recovery progress

Every recovery attempt SHOULD record whether it:

improved state
maintained state
degraded state
failed
introduced a new incident

A repeated non-progressing recovery path SHOULD be terminated or escalated.


---

63. Recovery loop detection

The system MUST detect cycles such as:

A
→ B
→ A
→ B
→ A

without progress.

The cycle detector MUST use execution state/provenance rather than a fixed number of attempts.


---

64. Checkpoint scalability

Checkpoints MUST support:

small workloads;

large workloads;

distributed workloads;

incremental checkpoints;

versioned checkpoints;

integrity verification;

storage abstraction.


The checkpoint system MUST NOT assume arbitrary quantum state can always be serialized.


---

65. Checkpoint granularity

Supported checkpoint scopes SHOULD include:

program
execution
classical state
compiled representation
logical state
QEC state
measurement boundary
provider-supported state

The valid scope depends on the target execution model.


---

66. Checkpoint size

Large checkpoints MUST support:

chunking;

streaming;

deduplication where safe;

compression where appropriate;

incremental snapshots.


No fixed checkpoint-size constant may define the architecture.


---

67. Checkpoint compatibility

A checkpoint MUST be validated against:

IR version
program identity
resilience schema
hardware capabilities
QEC configuration
execution model
checkpoint schema

A checkpoint that cannot be safely restored MUST be rejected.


---

68. Provenance scalability

Every adaptive execution may produce a large provenance graph.

Therefore provenance SHOULD support:

graph structure
content hashes
references
incremental records
compressed evidence
external storage

The provenance model MUST remain verifiable without requiring every raw event to remain in memory.


---

69. Provenance identity

At minimum, provenance SHOULD bind:

program identity
IR identity
canonical IR hash
logical resources
physical mapping
target capabilities
policy
optimization
routing
schedule
QEC configuration
mitigation
fault evidence
diagnosis
recovery plan
execution
verification
result


---

70. Verification scalability

Verification MUST scale with workload size.

It SHOULD support:

local verification
regional verification
global verification
statistical verification
invariant checking
semantic checking

depending on the execution model.


---

71. Verification cannot be removed for scale

Large systems MUST NOT disable semantic verification simply because verification is expensive.

Instead, verification SHOULD become hierarchical or incremental.

Example:

local verification
+
aggregated global invariant verification

rather than no verification.


---

72. Statistical verification

When exact verification is impossible, the verification contract MUST explicitly state:

verification method
confidence
assumptions
sample size
uncertainty
acceptance criterion

A statistical result MUST NOT be presented as exact verification.


---

73. Mitigation scalability

Mitigation overhead can grow significantly with workload size.

Therefore mitigation selection MUST consider:

accuracy gain
sampling overhead
execution time
available resources
noise stability
budget

The resilience layer chooses whether mitigation is worthwhile; mitigation implementations remain separate.


---

74. QEC and mitigation separation

Scaling QEC does not mean automatically scaling mitigation.

QEC and mitigation have different semantics:

QEC
→ detects/corrects encoded quantum errors

Mitigation
→ reduces observed computational error without necessarily correcting the underlying state

The two MUST remain separate.


---

75. Learning scalability

learning/ is optional.

Correctness MUST NOT depend on machine learning.

A learned model may:

rank
predict
prioritize
estimate

but MUST NOT bypass:

policy
safety
capability validation
semantic verification
security checks


---

76. Learning data scalability

Learning data MUST be:

bounded by policy;

streamable;

versioned;

reproducible;

provenance-aware;

separable by workload/resource scope.


Large history MUST NOT require keeping all observations in memory.


---

77. Learning model portability

A learned model trained on one machine MUST NOT automatically be trusted on another machine.

The model MUST carry enough provenance to determine:

training environment;

feature schema;

hardware class;

model version;

calibration context;

validity period;

confidence.


Stale predictions MUST be downgraded or rejected according to policy.


---

78. Dynamic calibration

Calibration data can become stale.

Resilience MUST treat calibration as time-dependent state.

A plan generated from an old calibration snapshot MUST be revalidated before execution when required by policy.


---

79. Execution-mode scalability

The execution subsystem may expose different execution models.

Resilience MUST treat these as capabilities.

Possible modes include:

single job
batch
session
streaming
interactive
long-running
distributed

The resilience layer MUST NOT assume that every backend supports every mode.

Current quantum execution systems already expose different execution modes for different workload structures, reinforcing the need for capability-driven execution rather than a fixed execution assumption.


---

80. Queue scalability

Queue depth MUST be discovered.

Resilience MUST be able to reason about:

queue latency
execution latency
deadline
priority
session availability
batch opportunities

without assuming a fixed queue size.


---

81. Parallel workload scalability

Independent workloads SHOULD be parallelized when resources allow.

For example:

Workload A
Workload B
Workload C

may execute concurrently when the scheduler and hardware permit it.

Resilience must query scheduling capacity rather than assume parallelism.


---

82. Classical/quantum co-scheduling

Large workloads may involve:

classical optimization
→ quantum execution
→ classical analysis
→ quantum execution

Resilience MUST account for the full execution loop.

A quantum-only scalability model is insufficient.


---

83. Scheduling integration

Resilience MUST delegate scheduling to:

quantum::scheduling

It may request:

rebuild
reschedule
prioritize
partition
merge

but MUST NOT implement a second scheduler.


---

84. Routing integration

Resilience MUST delegate physical mapping and route generation to:

quantum::routing

It may provide:

failed resources
available resources
new capabilities
constraints

but the routing algorithm remains outside resilience.


---

85. Optimization integration

Resilience MUST delegate optimization to:

quantum::optimization

It may request a new optimization pass/profile when the target changes.

It MUST NOT duplicate optimization algorithms.


---

86. Hardware integration

Resilience MUST consume:

identity
technology
capabilities
instruction set
timing
topology
calibration
status
health
telemetry
execution
provider

from the hardware subsystem.

It MUST NOT recreate a second hardware abstraction layer.


---

87. ZQN integration

ZQN is the authoritative fault/noise semantics layer.

Resilience consumes ZQN fault information and transforms it into:

Incident
Diagnosis
Policy decision
Recovery plan

Resilience MUST NOT create a competing noise ontology.


---

88. QEC integration

Resilience consumes QEC health and logical-error information.

It MAY request QEC adaptation.

It MUST NOT implement code-specific correction logic.


---

89. Benchmarking integration

Benchmarking MAY provide:

historical reliability
fidelity
latency
stability
failure rate
resource efficiency

to the planner.

Benchmarking remains independent.

Historical benchmarks MUST NOT be treated as guarantees of current hardware behavior.


---

90. Simulation integration

Simulation SHOULD expose a way to test resilience with:

synthetic resources
synthetic topology
synthetic telemetry
synthetic faults

This enables scalability testing without physical hardware.


---

91. API scalability

api/controller.rs MUST NOT require callers to provide machine-specific information unless the caller intentionally overrides automatic discovery.

The preferred API model is:

program
+
requirements
+
policy

rather than:

program
+
127 qubits
+
backend X
+
physical map Y
+
retry count Z


---

92. Request scalability

api/request.rs MUST describe intent and constraints.

It SHOULD include:

program identity
execution requirements
resilience policy
resource preferences
deadlines
budgets
security requirements
verification requirements

It SHOULD NOT encode fixed hardware assumptions.


---

93. Context scalability

api/context.rs provides access to:

IR
hardware
routing
scheduling
optimization
QEC
execution
telemetry
policy
checkpointing

Dependencies SHOULD be represented as contracts/traits or stable interfaces.

Concrete implementation ownership remains outside resilience.


---

94. Registry scalability

Registries MUST support dynamic strategy discovery.

Examples:

DetectorRegistry
StrategyRegistry
RecoveryRegistry
BackendRegistry

Registry implementations MUST NOT depend on a fixed number of entries.


---

95. Plugin scalability

Plugins MUST be capability-scoped.

A plugin MUST declare:

identity
version
supported interface
required capabilities
provided capabilities
resource requirements
security requirements

Unknown or unauthorized capabilities MUST NOT be silently accepted.


---

96. Registry isolation

A faulty strategy/plugin MUST NOT be able to corrupt global resilience state.

The registry layer MUST support:

validation
version compatibility
capability checks
isolation
quarantine


---

97. Serialization scalability

Serialization MUST be:

deterministic where required;

versioned;

streaming-capable where useful;

bounded by resource policy;

resistant to malformed input;

compatible across supported versions.


It MUST NOT allocate attacker-controlled unbounded memory.


---

98. Serialization of large collections

Large structures MUST support incremental encoding/decoding where practical.

Examples:

telemetry
provenance
incident history
resource inventories
checkpoint metadata
execution traces


---

99. Deserialization safety

The decoder MUST validate:

schema version
lengths
counts
nested depth
resource references
identities
relationships

before constructing expensive structures.


---

100. No unsafe code

The resilience subsystem MUST contain no unsafe.

The implementation MUST NOT use:

unsafe
std::mem::transmute
raw pointer manipulation
unchecked FFI

for scalability.

The subsystem SHOULD enforce this through:

#![forbid(unsafe_code)]

at the appropriate module/crate boundary.


---

101. Safe Rust scalability

Rust ownership and borrowing SHOULD be used to prevent:

memory corruption;

use-after-free;

data races;

invalid ownership;

accidental aliasing.


Concurrent structures MUST use safe synchronization primitives.


---

102. Concurrency scalability

Concurrent resilience components MUST have clearly defined ownership.

Examples:

telemetry stream
incident store
planner
recovery controller
distributed coordinator

must not mutate shared global state without explicit synchronization.


---

103. Global state prohibition

Resilience MUST avoid process-global mutable state for:

current machine;

current backend;

current policy;

current qubit map;

current recovery;

current telemetry;

current incident.


Multiple independent executions MUST be able to coexist.


---

104. Multi-tenant scalability

The architecture MUST support independent execution contexts.

At minimum:

tenant/workload
program
execution
resource
policy
provenance

must be separable.

One workload's recovery state MUST NOT leak into another workload.


---

105. Isolation

A recovery action for workload A MUST NOT modify workload B unless explicitly coordinated.

This is especially important for:

shared QPUs;

shared simulators;

shared classical infrastructure;

distributed quantum networks.



---

106. Security and scalability

Scalability MUST NOT weaken security.

The system MUST continue to validate:

identity
authorization
integrity
provenance
capabilities

at larger scale.

Security checks SHOULD be hierarchical and cacheable where safe, rather than removed.


---

107. Resource-exhaustion defense

Every externally influenced quantity MUST be treated as potentially hostile or malformed.

Examples:

qubit count
event count
topology size
checkpoint size
history size
plugin metadata
strategy count
nested structures
serialization lengths

The system MUST apply dynamically supplied resource limits.


---

108. Dynamic limits

limits/limits.rs MUST represent operational limits.

It MUST NOT define universal quantum limits.

Examples:

maximum telemetry memory
maximum planner work
maximum checkpoint size
maximum concurrent recoveries
maximum history retention

are deployment policies, not laws of the architecture.


---

109. Resource limits versus scalability

A limit is not the same thing as a hard-coded architecture bound.

Correct:

deployment policy:
max_memory = discovered/configured value

Incorrect:

const MAX_MEMORY = 1_000_000;

The first allows deployment-specific scaling.

The second imposes an architectural ceiling.


---

110. Adaptive resource limits

Resource limits SHOULD be capable of changing at runtime when policy allows.

Example:

normal operation
→ high resource availability

degraded operation
→ lower available budget

emergency operation
→ emergency resource allocation


---

111. Backpressure over failure

When resources become constrained, resilience SHOULD prefer controlled degradation/backpressure over uncontrolled memory growth.

Possible actions:

delay
queue
sample
aggregate
partition
shed low-priority work
escalate
abort

The decision is policy-driven.


---

112. Workload partitioning

Large workloads MAY be partitioned when semantics permit.

Partitioning MUST be performed by the appropriate compiler/runtime/execution layer.

Resilience decides whether partitioning is a viable recovery/adaptation strategy.

It does not invent partition semantics.


---

113. Distributed workload partitioning

A distributed workload may be represented as:

Global program
    |
    +---- Region A
    +---- Region B
    +---- Region C
    +---- Region D

Each region MAY be independently adapted where semantic dependencies permit.

Cross-region dependencies MUST remain explicit.


---

114. Dependency-aware recovery

Resilience MUST understand dependency relationships.

If:

Region B depends on Region A

then recovering B independently may be invalid.

The planner MUST use the execution/circuit dependency graph supplied by the appropriate subsystem.


---

115. Avoid quadratic algorithms where possible

For large systems, resilience implementations SHOULD avoid unnecessary:

O(N²)
O(N³)

processing.

Examples:

all-pairs resource comparison;

all-pairs incident correlation;

exhaustive plan enumeration.


Where possible use:

indexes
graphs
partitions
spatial/topological locality
incremental algorithms
streaming algorithms
hierarchical aggregation


---

116. Complexity contracts

Each production algorithm SHOULD document expected complexity.

For example:

Detection:
O(events)

Local diagnosis:
O(affected_region)

Resource filtering:
O(resources)

Plan ranking:
O(candidate_plans log candidate_plans)

Exact complexity depends on implementation, but unbounded hidden complexity is unacceptable.


---

117. Large-resource identifiers

Resource identifiers MUST be capable of representing the identifier domain exposed by lower layers.

Do not truncate identities into small integer types merely for convenience.

Canonical identity remains owned by:

quantum::ir::qubit

for quantum IR qubit identities.


---

118. No fixed array architecture

The resilience subsystem MUST NOT architect around fixed arrays for scalable resource collections.

Avoid:

[QubitState; N]

when N is a machine-specific compile-time constant.

Prefer dynamic collections or streaming abstractions.


---

119. Serialization of identifiers

Serialized identities MUST preserve canonical identity semantics.

A serialized physical qubit MUST deserialize into the canonical physical identity type rather than a resilience-specific substitute.


---

120. History scalability

History MUST support retention policies.

Possible policies:

retain all
retain recent
retain aggregates
retain critical incidents
externalize old records
delete according to policy

The architecture MUST NOT assume infinite local storage.


---

121. Incident history compaction

Old incidents MAY be compacted into statistical summaries.

However, safety-critical provenance MUST remain available according to retention policy.

Compaction MUST preserve the ability to determine:

what happened
when
where
why
what action was taken
whether it succeeded
how the result was verified


---

122. Observability scalability

Observability MUST scale without turning into the bottleneck.

Support:

local telemetry
regional aggregation
central aggregation
distributed tracing
sampling
metrics
structured events


---

123. Trace correlation

Every execution should have stable correlation identifiers.

Example:

program
  ↓
execution
  ↓
incident
  ↓
plan
  ↓
adaptation
  ↓
recovery
  ↓
verification

The identifiers MUST allow distributed tracing without relying on a single global mutable state.


---

124. Recovery concurrency

Multiple recovery operations MAY run concurrently if their resource scopes do not conflict.

Before execution, each recovery action MUST establish resource ownership/conflict constraints.


---

125. Conflict detection

Example:

Recovery A:
uses Qubit 4

Recovery B:
uses Qubit 4

These actions cannot blindly execute concurrently.

The coordination layer must detect conflicts.


---

126. Recovery isolation

Independent regions SHOULD recover independently when possible.

This provides:

lower latency;

lower coordination cost;

greater availability;

better scalability.



---

127. Global recovery

Global recovery remains available when:

the fault is correlated;

the backend fails;

the topology changes globally;

QEC configuration changes globally;

the execution contract becomes invalid.



---

128. Adaptive hierarchy

The resilience architecture SHOULD dynamically choose:

local
regional
global
distributed

recovery scope.

It MUST NOT assume that one scope is always optimal.


---

129. Program-size scalability

The canonical Zamani program may grow significantly.

Resilience MUST NOT require copying the entire program for every incident.

Where possible:

immutable program identity
+
shared canonical IR
+
affected-region references

should be used.


---

130. Immutable source identity

The original program MUST remain immutable.

Adaptation creates new derived execution representations.

Conceptually:

Original Program
      |
      +--> Adaptation 1
      |
      +--> Adaptation 2
      |
      +--> Adaptation 3

The original semantic source remains the reference point.


---

131. Versioned execution representations

Each adapted execution SHOULD carry:

parent representation
version
reason
target capabilities
mapping
schedule
optimization
verification

This prevents adaptation from destroying history.


---

132. Canonical IR stability

Resilience MUST consume the canonical Zamani Quantum IR.

It MUST NOT create a second general-purpose circuit representation merely for resilience.

The canonical IR remains the semantic anchor.


---

133. IR hashing

Where the repository's canonical IR hashing facilities are available, resilience SHOULD use canonical hashes for:

program identity;

IR identity;

provenance;

checkpoint compatibility;

deterministic replay.


The repository already treats canonical logical/physical qubit identities as distinct identity concepts and provides canonical hashing infrastructure.


---

134. Scaling through references

Large structures SHOULD reference immutable objects by identity rather than repeatedly copying them.

For example:

Incident
→ ProgramId
→ ExecutionId
→ IR hash
→ resource snapshot ID

instead of embedding the entire program in every incident.


---

135. Copy avoidance

Production implementations SHOULD avoid unnecessary deep copies of:

canonical IR;

topology;

capability sets;

telemetry;

provenance;

checkpoints.


Ownership should be explicit.


---

136. Snapshot semantics

Resource snapshots SHOULD be immutable once captured.

A plan must be tied to the snapshot against which it was evaluated.

If the resource state changes:

snapshot mismatch
→ invalidate/revalidate


---

137. Stale-plan prevention

A recovery plan MUST NOT assume that the machine remains unchanged after planning.

Before execution:

plan
→ capability validation
→ state validation
→ execute

If validation fails:

replan


---

138. Time-aware scalability

Quantum hardware changes over time.

Therefore resource state is:

resource_state(t)

not a timeless property.

Telemetry, calibration and capabilities MUST carry appropriate temporal information.


---

139. Expiration

Policies MAY assign validity windows to:

calibration;

health;

predictions;

plans;

resource snapshots;

leases.


Expired information MUST be refreshed or explicitly accepted under policy.


---

140. Scalability under failure storms

The system MUST handle correlated failures.

Example:

1000 resources
↓
regional fault
↓
10000 telemetry events

The resilience system MUST aggregate the storm rather than launch 10000 independent global recoveries.


---

141. Incident storm control

The detector/diagnosis pipeline SHOULD support:

deduplication
correlation
coalescing
rate limiting
priority
suppression

while preserving critical evidence.


---

142. Recovery storm control

Recovery planning MUST prevent:

fault storm
→ recovery storm
→ resource exhaustion
→ secondary outage

The policy layer MUST be able to limit concurrent recovery work dynamically.


---

143. Cascading failures

Resilience MUST recognize that recovery itself can cause resource pressure.

Therefore every plan should estimate:

direct impact
+
secondary impact

where possible.


---

144. Failure domains

Resources SHOULD be grouped by failure domains where the hardware/runtime exposes them.

Examples:

qubit
coupling
module
QPU
host
network link
backend
region
provider

A correlated fault affecting one failure domain should not cause the planner to treat every resource independently.


---

145. Failure-domain-aware migration

When migrating away from a failed domain, the destination SHOULD be selected from a sufficiently independent failure domain when policy requires it.


---

146. Provider scalability

Provider diversity MUST be handled through adapters.

Core resilience MUST NOT depend on provider-specific APIs.

Provider adapters belong at integration boundaries.


---

147. Provider adapter contract

A provider adapter should expose standardized:

capabilities
health
telemetry
execution
status
resource identity
authentication

The resilience layer consumes those standardized contracts.


---

148. No provider-specific branching in planner

planning/planner.rs MUST NOT contain provider-specific branches.

Bad:

if backend == "provider_a" {
    ...
}

Good:

capabilities.supports(...)


---

149. Future hardware scalability

The architecture MUST support hardware that Zamani does not yet know about.

Therefore:

technology-specific implementation
→ capability adapter
→ standard hardware contract
→ resilience

not:

resilience
→ assumptions about today's hardware


---

150. New quantum architectures

A new architecture should require primarily:

hardware adapter
capability description
routing/scheduling integration
execution adapter

and SHOULD NOT require redesigning resilience's core domain model.


---

151. New QEC architectures

A new QEC architecture should integrate through:

QEC capability
QEC health
QEC adaptation
logical-error reporting

without rewriting resilience's planner.


---

152. New mitigation strategies

New mitigation strategies MUST implement the mitigation strategy contract.

They should be registrable through:

registry/strategy.rs

without changing:

planning/planner.rs


---

153. New detectors

New detector implementations MUST be registrable through:

registry/detector.rs

without changing the central detector contract.


---

154. New recovery strategies

New recovery implementations MUST integrate through:

registry/recovery.rs

without modifying existing recovery algorithms.


---

155. Version scalability

Every serialized public resilience structure MUST have a version.

Compatibility MUST be explicit.

Supported version relationships:

same version
backward compatible
forward compatible
migration required
unsupported


---

156. Version-independent semantics

Version numbers MUST NOT be used as substitutes for capability negotiation.

Two systems may have compatible schema versions but incompatible capabilities.

Both must be checked.


---

157. API compatibility

The public resilience API should remain stable even as implementations evolve.

Internal strategies may change without changing:

ResilienceRequest
ResilienceResponse
ResilienceController

unless a deliberate breaking release is made.


---

158. Feature evolution

New resilience features SHOULD be introduced through:

optional capabilities
optional policy fields
versioned extensions
registries

rather than hard-coded assumptions.


---

159. Testing scalability

tests/scalability.rs MUST test generated resource sizes.

It MUST NOT only contain:

1
10
100
1000

as manually chosen constants.

The tests should generate workloads and resource models from parameters.


---

160. Property-based scaling

Tests SHOULD verify properties such as:

for every valid resource count N:
    planner remains valid

and:

for every valid topology:
    mapping remains identity-correct

and:

for every valid resource reduction:
    planner either adapts or rejects safely


---

161. Generated topologies

Scalability tests SHOULD generate:

linear
ring
grid
sparse
dense
modular
random connected
partitioned
distributed

topologies.


---

162. Generated qubit identities

Tests MUST use canonical:

quantum::ir::qubit::QubitId
quantum::ir::qubit::PhysicalQubitId

where appropriate.

Tests MUST NOT introduce a competing test-only qubit identity type that hides integration errors.


---

163. Fault injection at scale

tests/fault_injection.rs SHOULD test:

single fault
multiple faults
correlated faults
regional faults
global faults
fault storms
repeated faults
fault during recovery
fault during verification


---

164. Recovery during recovery

The system MUST handle:

Recovery A starts
      ↓
new fault occurs
      ↓
re-evaluate

The new fault MUST NOT corrupt the previous recovery state.

The state machine determines whether to:

continue
pause
abort
rollback
replan
escalate


---

165. Deterministic replay

Large incidents MUST be replayable without the original physical hardware when possible.

Replay should use:

program
IR snapshot
resource snapshot
telemetry
policy
seed
configuration

to reproduce planning.


---

166. Replay scalability

Replay SHOULD support:

full replay
incident replay
region replay
planner-only replay
verification-only replay

rather than requiring full-system replay for every debugging operation.


---

167. Benchmarking scalability

Scalability benchmarks SHOULD measure:

detection throughput
diagnosis latency
planning latency
adaptation latency
verification latency
memory
CPU
telemetry throughput
recovery success rate

as functions of workload/resource size.


---

168. Complexity benchmarks

Benchmarks SHOULD measure:

N resources
M operations
F faults
E telemetry events
P candidate plans
D distributed nodes

rather than reporting only one hardware size.


---

169. Resource-efficiency metric

The resilience subsystem SHOULD expose efficiency metrics such as:

recovered work / recovery cost
verified result / execution cost
telemetry processed / CPU
incidents handled / memory

These are observability metrics, not correctness conditions.


---

170. Scalability acceptance

A scalability test MUST distinguish:

algorithmic limitation
resource limitation
deployment policy limitation
hardware limitation
backend limitation

The test MUST NOT incorrectly report a configured resource limit as an architectural maximum.


---

171. No false scalability claims

Zamani MUST NOT claim:

infinite qubits supported

if a particular backend cannot support them.

The correct statement is:

> The resilience architecture imposes no artificial fixed qubit ceiling; execution is constrained by available resources and declared policies.




---

172. Atom-scale operation

At minimum, the architecture MUST be able to represent a resource configuration containing one quantum resource.

Example conceptual configuration:

1 logical/physical resource

No subsystem may assume at least two qubits.


---

173. Small-system efficiency

For tiny systems, the architecture SHOULD avoid unnecessary distributed coordination.

A single local controller should be sufficient when the execution context is local.


---

174. Medium-scale operation

At medium scale, the system should support:

regional grouping
parallel detection
parallel verification
local recovery

without requiring fleet-wide coordination.


---

175. Large-scale operation

At large scale:

hierarchical resilience
streaming telemetry
partitioned diagnosis
local recovery
regional aggregation

SHOULD become available.


---

176. Fleet-scale operation

For many QPUs/backends:

fleet health
backend selection
resource federation
distributed coordination

may be used.


---

177. Distributed quantum scale

For distributed quantum systems:

local controller
+
regional controller
+
global coordinator

may be composed.

The architecture MUST NOT require all events to travel to one central process.


---

178. Quantum network scale

If future quantum networking is supported, resource models may include:

quantum link
entanglement resource
network node
memory
repeater
control path
classical control path

These should be introduced through the hardware/network capability contracts.

Resilience itself remains resource-agnostic.


---

179. Entanglement-resource scalability

Where entanglement is modeled as a resource, it MUST be represented dynamically.

The resilience layer MUST NOT assume:

one fixed network topology

or:

one fixed entanglement capacity


---

180. Network partition handling

Distributed resilience MUST distinguish:

resource failure

from:

loss of communication

A controller that cannot communicate with a resource MUST NOT automatically conclude that the quantum resource itself has failed.


---

181. Partition-safe behavior

During a distributed communication partition:

unknown

is not equivalent to:

failed

The state model MUST preserve uncertainty.


---

182. Unknown state scalability

Unknown resources must be represented explicitly.

Example:

Healthy
Degraded
Unavailable
Unknown

An unknown resource may be:

quarantined;

probed;

excluded temporarily;

retained for future validation.


The policy determines behavior.


---

183. Confidence scalability

Confidence must accompany:

detection;

diagnosis;

prediction;

verification.


A large system MUST NOT convert uncertainty into false certainty merely for simpler control flow.


---

184. Correlation scalability

Correlation SHOULD use indexed evidence rather than comparing every event with every other event.

Possible dimensions:

time
resource
operation
failure domain
execution
backend
topology
causal relationship


---

185. Time-window correlation

Correlation windows MUST be configurable.

No fixed:

5 seconds

assumption belongs in core resilience.


---

186. Spatial/topological correlation

When resource topology is available, correlation SHOULD use it.

For example:

nearby resources
shared coupling
shared control channel
shared module

may have stronger correlation than unrelated resources.


---

187. Resource-health aggregation

Large physical resource populations SHOULD be summarized into health domains.

Example:

physical qubits
    ↓
module health
    ↓
QPU health
    ↓
backend health

The exact hierarchy comes from the discovered resource model.


---

188. No fixed hierarchy

The hierarchy MUST remain dynamic.

Some architectures may have:

qubit → module → QPU

while another may have:

ion chain → trap → processor

and another:

logical block → tile → modular processor

Resilience consumes the exposed resource relationships.


---

189. Adaptation scope

Every adaptation action SHOULD state its scope.

Examples:

Local
Regional
Global
Distributed

This makes scalability decisions explicit.


---

190. Plan invalidation

Plans MUST be invalidated when assumptions change materially.

Examples:

resource disappears
capability changes
calibration changes beyond policy tolerance
lease expires
policy changes
security state changes


---

191. Atomic plan execution

Where the lower-level execution system supports it, recovery plans SHOULD be applied transactionally.

If transactional application is unavailable, the plan MUST define safe boundaries.


---

192. Partial recovery

If a multi-action recovery plan partially succeeds:

Action 1 succeeds
Action 2 fails
Action 3 not attempted

the state machine MUST record exactly that state.

It MUST NOT report the entire plan as successful.


---

193. Recovery composition

Large systems may require composed recovery:

remap
→ reroute
→ reschedule
→ recompile
→ execute
→ verify

Each action remains owned by its corresponding subsystem.


---

194. No duplicated algorithms

Resilience MUST NOT duplicate:

routing algorithms
scheduling algorithms
optimization passes
QEC decoders
hardware discovery
calibration
noise models
canonical IR
simulation engines
benchmark engines

Duplication creates inconsistent scaling behavior.


---

195. Contract-oriented architecture

Every resilience file MUST have:

input contract
output contract
invariants
resource assumptions
scalability assumptions
failure behavior
integration dependencies
ownership rules

before implementation is considered complete.

This directly supports the requirement:

> Finish one file without having to redesign it after another file is implemented.




---

196. File-specific scalability contract

The following files have the following scalability responsibilities.

model/resource.rs

Must represent arbitrarily sized resource sets.

Depends on:

canonical IR identity
hardware resource identity

Must not own:

hardware discovery


---

model/capability.rs

Must represent capabilities dynamically.

Consumes:

quantum::hardware

Must not contain provider-specific assumptions.


---

model/degradation.rs

Must represent arbitrary resource reduction.

Must support partial and hierarchical degradation.


---

model/health.rs

Must represent health at arbitrary resource scopes.


---

model/fault.rs

Must consume canonical ZQN fault semantics.


---

model/incident.rs

Must aggregate arbitrarily many related observations subject to dynamic resource limits.


---

model/confidence.rs

Must represent uncertainty without fixed confidence categories that prevent future extensions.


---

197. Detection scalability contracts

detection/detector.rs

Must define a stream-compatible detector interface.

detection/anomaly.rs

Must operate incrementally where possible.

detection/threshold.rs

Thresholds must come from policy/configuration.

detection/statistical.rs

Must avoid requiring the entire event history in memory.

detection/drift.rs

Must support time-dependent state.

detection/timeout.rs

Timeouts must come from execution/resource policy.

detection/execution_failure.rs

Must normalize backend failures without embedding provider logic.

detection/qec_signal.rs

Must consume QEC signals at physical or logical scope.

detection/hardware_signal.rs

Must consume dynamic hardware telemetry.


---

198. Diagnosis scalability contracts

diagnosis/classifier.rs

Must classify arbitrary fault domains.

diagnosis/correlation.rs

Must support indexed/partitioned correlation.

diagnosis/localization.rs

Must operate on canonical resource identities.

diagnosis/root_cause.rs

Must preserve uncertainty.

diagnosis/confidence.rs

Must produce confidence tied to evidence.

diagnosis/diagnostician.rs

Must compose diagnosis components without assuming fixed machine size.


---

199. Policy scalability contracts

policy/constraints.rs

Must express workload constraints independent of machine size.

policy/objectives.rs

Must support multiple objectives.

policy/budgets.rs

Must express dynamic resource budgets.

policy/escalation.rs

Must prevent uncontrolled recovery loops.

policy/retry.rs

Must avoid hard-coded retry counts.

policy/safety.rs

Must remain mandatory at every scale.

policy/policy.rs

Must compose the policy contract without embedding hardware assumptions.


---

200. Planning scalability contracts

planning/action.rs

Must describe abstract actions.

planning/plan.rs

Must be immutable/versioned.

planning/cost.rs

Must support multidimensional cost.

planning/feasibility.rs

Must evaluate current capabilities.

planning/ranking.rs

Must support bounded candidate ranking.

planning/planner_state.rs

Must avoid global mutable state.

planning/planner.rs

Must orchestrate planning without provider-specific branches.


---

201. Adaptation scalability contracts

adaptation/remapping.rs

Uses canonical logical/physical identity and delegates mapping.

adaptation/rerouting.rs

Delegates to routing.

adaptation/rescheduling.rs

Delegates to scheduling.

adaptation/recompilation.rs

Delegates to compiler/IR.

adaptation/reoptimization.rs

Delegates to optimization.

adaptation/qec_adaptation.rs

Delegates QEC configuration changes.

adaptation/backend_selection.rs

Uses dynamic capability negotiation.

adaptation/adapter.rs

Provides the stable adaptation orchestration contract.


---

202. Recovery scalability contracts

recovery/retry.rs

Must use policy/budgets rather than fixed attempts.

recovery/restart.rs

Must operate from safe execution boundaries.

recovery/checkpoint.rs

Must use checkpoint contracts.

recovery/rollback.rs

Must never assume arbitrary quantum state can be reversed.

recovery/resume.rs

Must validate checkpoint and resource compatibility.

recovery/migration.rs

Must preserve semantic identity.

recovery/compensation.rs

Must only perform mathematically valid compensating operations.

recovery/recoverer.rs

Must orchestrate recovery without implementing lower-layer algorithms.


---

203. Mitigation scalability contracts

mitigation/strategy.rs

Stable extensible interface.

mitigation/selection.rs

Capability/budget-aware selection.

mitigation/readout.rs

Must scale readout mitigation according to available data/resources.

mitigation/zero_noise.rs

Must make noise-scaling parameters configurable.

mitigation/probabilistic.rs

Must explicitly account for sampling/resource overhead.

mitigation/twirling.rs

Must obtain valid operations from target capabilities.

mitigation/dynamical_decoupling.rs

Must use scheduling/pulse contracts rather than hardware assumptions.

mitigation/custom.rs

Extension point for future techniques.

mitigation/executor.rs

Must manage resource-aware execution.


---

204. Verification scalability contracts

verification/invariant.rs

Must support local and global invariants.

verification/semantic.rs

Must compare against canonical semantic intent.

verification/result.rs

Must validate large result structures incrementally where possible.

verification/confidence.rs

Must expose uncertainty.

verification/provenance.rs

Must support large provenance graphs.

verification/acceptance.rs

Must remain the final safety gate.

verification/verifier.rs

Must compose verification without sacrificing mandatory checks.


---

205. State scalability contracts

state/machine.rs

Must represent arbitrary resource populations.

state/execution.rs

Must support concurrent executions.

state/logical.rs

Must use logical identity correctly.

state/physical.rs

Must use canonical physical identity.

state/recovery.rs

Must support nested/parallel recovery state where permitted.

state/persistence.rs

Must support large state through streaming/chunking/storage abstractions.


---

206. Checkpoint scalability contracts

checkpoint/checkpoint.rs

Stable checkpoint API.

checkpoint/snapshot.rs

Metadata/reference-oriented snapshot.

checkpoint/manifest.rs

Large manifest support.

checkpoint/storage.rs

Storage-independent interface.

checkpoint/integrity.rs

Scalable integrity verification.

checkpoint/compatibility.rs

Capability/schema compatibility.


---

207. Telemetry scalability contracts

telemetry/event.rs

Compact canonical events.

telemetry/metric.rs

Aggregatable metrics.

telemetry/trace.rs

Distributed trace correlation.

telemetry/health.rs

Hierarchical health observations.

telemetry/collector.rs

Streaming/backpressure.

telemetry/exporter.rs

Pluggable output.


---

208. History scalability contracts

history/incident.rs

Incremental incident storage.

history/execution.rs

Execution history.

history/recovery.rs

Recovery outcome history.

history/statistics.rs

Aggregated historical statistics.


---

209. Learning scalability contracts

learning/features.rs

Streaming feature extraction.

learning/model.rs

Versioned model interface.

learning/predictor.rs

Bounded prediction work.

learning/strategy.rs

Prediction-to-strategy mapping.

learning/feedback.rs

Verified feedback only.


---

210. Coordination scalability contracts

coordination/ownership.rs

Dynamic ownership.

coordination/lease.rs

Expiring resource ownership.

coordination/distributed.rs

Distributed coordination.

coordination/consensus.rs

Optional consensus abstraction.

coordination/coordinator.rs

High-level coordinator.


---

211. Serialization scalability contracts

serialization/schema.rs

Versioned schemas.

serialization/encode.rs

Streaming/chunk-capable encoding.

serialization/decode.rs

Bounded safe decoding.

serialization/version.rs

Compatibility rules.


---

212. Limit scalability contracts

limits/limits.rs

Deployment/runtime limits.

limits/resource.rs

Dynamic resource budgets.

limits/validation.rs

Runtime validation.

No file in this directory may define a universal maximum qubit count.


---

213. Registry scalability contracts

registry/detector.rs

Dynamic detector registry.

registry/strategy.rs

Dynamic strategy registry.

registry/recovery.rs

Dynamic recovery registry.

registry/backend.rs

Dynamic backend adapter registry.


---

214. API scalability contracts

api/request.rs

Program-level intent and constraints.

api/response.rs

Result + verification + provenance.

api/context.rs

Execution dependencies.

api/controller.rs

Top-level orchestration.


---

215. Module boundaries

The dependency direction MUST remain:

Canonical IR
     ↓
ZQN / Hardware / QEC / Routing / Scheduling / Optimization
     ↓
Resilience
     ↓
Execution

Resilience may request services from those systems.

They MUST NOT need to depend on concrete resilience implementation details.


---

216. Avoid circular dependencies

Forbidden architectural pattern:

resilience → scheduling
scheduling → resilience

Instead:

shared execution contract
        ↑       ↑
resilience   scheduling

or another acyclic contract arrangement.


---

217. quantum::mod.rs

The quantum root module MUST remain thin.

It should expose:

pub mod resilience;

and MUST NOT contain scalability logic.


---

218. resilience/mod.rs

The resilience root module MUST:

declare modules;

expose stable public APIs;

enforce safe Rust policy;

avoid business logic;

avoid hardware assumptions;

avoid machine-size constants.


It SHOULD enforce:

#![forbid(unsafe_code)]

for the resilience module where the repository's module structure permits.


---

219. Documentation scalability

SCALABILITY.md is the normative scalability contract.

Other documents must reference it rather than redefining machine-size semantics.

Relevant documents include:

ARCHITECTURE.md
DESIGN.md
SECURITY.md
COMPATIBILITY.md
DETERMINISM.md
FAILURE_MODES.md
RECOVERY_MODEL.md
OBSERVABILITY.md


---

220. Documentation consistency

No documentation may claim:

maximum = X qubits

unless X is explicitly a documented limit of a specific backend, deployment, test, or configuration.


---

221. Testing matrix

Production testing MUST cover:

Dimension	Required coverage

Qubit count	parameterized
Logical qubits	parameterized
Topology	generated
Fault count	parameterized
Incident count	parameterized
Telemetry volume	streaming
Backend count	parameterized
Distributed nodes	parameterized
Recovery concurrency	parameterized
Checkpoint size	parameterized
Program size	parameterized
Candidate plans	parameterized
Resource degradation	dynamic
Hardware migration	dynamic



---

222. Minimum scale test

The smallest valid system MUST be supported.

Example:

one quantum resource
one execution
one detector
one incident
one recovery
one verification

No algorithm may require a larger machine.


---

223. Large-scale test

Large-scale tests MUST generate resource counts dynamically.

The exact maximum test size depends on available CI/test hardware.

The architecture must not use the test size as its maximum supported size.


---

224. Stress testing

Stress tests SHOULD vary:

N = resources
M = operations
F = faults
E = telemetry
R = recovery operations
P = plans

independently.

This identifies which dimension causes the bottleneck.


---

225. Soak testing

Long-running tests MUST detect:

memory leaks;

state growth;

telemetry accumulation;

stale leases;

recovery loops;

history growth;

stale predictions;

resource ownership leaks.



---

226. Fault-storm testing

The test suite MUST generate correlated and simultaneous failures.

Expected behavior:

aggregate
diagnose
contain
recover
verify

rather than:

one global recovery per low-level event


---

227. Network-partition testing

Distributed tests MUST simulate:

node loss
network partition
delayed messages
duplicate messages
reordered messages
stale messages

and verify that stale recovery commands cannot corrupt current ownership.


---

228. Determinism tests

tests/determinism.rs MUST verify:

same input
→ same normalized state
→ same plan

when deterministic mode is enabled.


---

229. Serialization tests

tests/serialization.rs MUST verify that large resilience structures:

encode
→ decode
→ preserve semantics

without identity corruption.


---

230. Fuzzing

Production CI SHOULD fuzz:

telemetry
fault records
diagnoses
policies
plans
checkpoints
serialization
resource graphs
topologies

Malformed input MUST result in controlled errors rather than process failure.


---

231. Property testing

Property tests SHOULD verify:

no fixed qubit ceiling
logical identity preservation
physical identity preservation
plan invalidation
recovery idempotence where applicable
deterministic ranking
bounded resource behavior


---

232. Security scalability

Large-scale deployments increase attack surface.

Security MUST therefore scale with:

identity count
resource count
telemetry volume
plugin count
tenant count
backend count

Security architecture MUST NOT depend on a single trusted global machine.


---

233. Audit scalability

Audit data MUST support:

streaming
partitioning
retention
aggregation
tamper evidence
distributed correlation


---

234. Privacy scalability

Telemetry SHOULD be minimized.

Large systems can produce enormous sensitive datasets.

Only data necessary for:

operation
diagnosis
verification
security
audit

should be retained according to policy.


---

235. Side-channel awareness

Scaling resilience can expose:

resource availability;

topology;

failure domains;

workload characteristics;

execution timing.


Access to such information MUST follow the security/authorization model.


---

236. Resource federation

Large deployments may expose a federated resource pool:

Provider A
  ├── QPU 1
  └── QPU 2

Provider B
  ├── QPU 3
  └── Simulator 1

Local
  └── Emulator

Resilience should treat these as capability-bearing resources.


---

237. Federation neutrality

The planner MUST NOT assume all resources are equivalent.

It should compare:

capability
compatibility
cost
latency
fidelity
risk
availability
policy


---

238. Resource substitution

Two resources may be interchangeable only if their capabilities satisfy the program's requirements.

A larger resource is not automatically a valid substitute for a smaller one.


---

239. Semantic preservation during scaling

Scaling from:

1 → N

must preserve:

operation meaning;

logical identity;

measurement semantics;

classical control semantics;

QEC semantics;

declared constraints.



---

240. Timing preservation

Timing constraints MUST be interpreted through the scheduling/hardware timing contracts.

Resilience MUST NOT assume that:

gate duration = constant

across machines.

OpenQASM's timing model similarly distinguishes timing intent from hardware-specific implementation/calibration. 


---

241. Resource availability versus semantic availability

A machine may have enough qubits but still be incapable of executing the program.

Therefore:

qubit_count >= required_qubits

is insufficient.

Capability compatibility must also hold.


---

242. Feasibility predicate

Conceptually:

Feasible =
    resource_capacity
    AND topology
    AND instruction_set
    AND timing
    AND QEC
    AND execution_model
    AND policy
    AND security

This predicate must be implemented through contracts rather than hard-coded checks.


---

243. Resource-aware program realization

The overall process becomes:

Zamani Program
      ↓
Canonical IR
      ↓
Program Requirements
      ↓
Resource Discovery
      ↓
Capability Negotiation
      ↓
Optimization
      ↓
Routing
      ↓
Scheduling
      ↓
Execution
      ↓
Telemetry
      ↓
Resilience
      ↓
Adaptation if required
      ↓
Verification


---

244. The program remains stable

A program should not have to be rewritten because:

QPU A becomes unavailable

or:

QPU B has fewer usable qubits

or:

topology changes

or:

another backend becomes better

The lower layers adapt.


---

245. Scalability under backend replacement

If backend A is replaced by backend B:

same program
same semantic requirements
different capability snapshot
different realization

The resilience system MUST preserve semantic identity.


---

246. Scalability under hardware growth

If a system grows:

100 qubits
→ 1,000
→ 10,000
→ larger

the architecture MUST NOT require changing:

QubitId definitions
planner semantics
recovery state machine
policy interface
API contracts

Only resource data and lower-layer realization should change.


---

247. Scalability under hardware reduction

If a system shrinks:

1000
→ 900
→ 500
→ 100

the system SHOULD:

adapt

where feasible.

If not feasible:

escalate

rather than producing an invalid result.


---

248. Graceful impossibility

When a workload cannot fit available resources, resilience MUST return a structured reason.

Examples:

InsufficientQubits
UnsupportedTopology
UnsupportedInstruction
InsufficientTimingCapacity
InsufficientQecCapacity
BudgetExceeded
NoCompatibleBackend
SecurityPolicyViolation
VerificationUnavailable

The error taxonomy remains in errors/.


---

249. No silent truncation

The system MUST NEVER silently:

remove qubits;

remove operations;

reduce precision;

reduce QEC protection;

reduce verification;

disable mitigation;

lower security;

change semantics


merely to make a workload fit.

Any such adaptation requires explicit policy permission and must be represented in provenance.


---

250. Automatic scaling

Automatic scaling may mean:

discover more resources
select larger device
partition workload
parallelize independent work
increase QEC capacity
change backend

It must never mean:

silently change the computation


---

251. Resource elasticity

Where the execution environment supports elasticity, resilience MAY request:

resource expansion
resource contraction
backend migration
additional classical capacity
additional execution slots

The actual allocation belongs to the resource/execution subsystem.


---

252. Elastic recovery

A recovery plan may therefore contain:

AcquireResources
ReleaseResources
Migrate
Remap
Reroute
Reschedule
Recompile
Execute
Verify

provided the corresponding lower layers support them.


---

253. Scaling and cost

Scaling is not free.

The planner SHOULD consider:

additional QPU time
additional shots
additional compilation
additional network traffic
additional classical compute
additional mitigation
additional QEC overhead

The policy determines acceptable cost.


---

254. Scaling and fidelity

A larger machine may have different fidelity characteristics.

The planner MUST NOT assume:

larger == better

or:

more qubits == higher reliability

Actual capabilities and telemetry determine suitability.


---

255. Scaling and latency

A larger distributed system may introduce:

communication latency
coordination latency
queue latency
compilation latency

These belong in the cost/constraint model.


---

256. Scaling and energy

Where hardware reports energy/thermal constraints, these may become planning dimensions.

The resilience layer consumes those capabilities rather than embedding hardware-specific thermal logic.


---

257. Scaling and availability

Availability is one objective, not the only objective.

The system MUST NOT sacrifice semantic correctness simply to increase availability.

The invariant remains:

Semantic validity
+
Capability validity
+
Policy validity
+
Security validity
+
Verification validity

before acceptance.


---

258. Scalability and safety

At larger scale, the number of opportunities for failure increases.

Therefore safety checks must become:

hierarchical
incremental
parallel
cached where safe

rather than removed.


---

259. Scalability and correctness

Performance optimizations MUST NOT alter quantum semantics.

Any optimization must remain under the authority of:

canonical IR
optimization subsystem
verification subsystem


---

260. Production readiness criteria

quantum::resilience is NOT production-ready until all of the following are true.

Architecture

no artificial machine-size ceiling;

no circular subsystem dependency;

no duplicated quantum infrastructure;

dynamic capability discovery;

explicit logical/physical identity.


Rust

Rust 1.97 / 1.97.1 supported;

no unsafe;

safe ownership;

safe concurrency;

no unchecked FFI.


Scalability

parameterized resource counts;

dynamic topology;

dynamic capabilities;

streaming telemetry;

bounded memory;

hierarchical resilience;

distributed support;

scalable serialization.


Quantum correctness

canonical IR;

canonical QubitId;

canonical PhysicalQubitId;

semantic verification;

QEC integration;

routing integration;

scheduling integration;

optimization integration.


Resilience

detection;

diagnosis;

policy;

planning;

adaptation;

recovery;

mitigation;

verification.


Reliability

stale-plan prevention;

recovery-loop prevention;

checkpoint compatibility;

migration;

partial recovery handling;

fault-storm control.


Security

authenticated/validated resource information;

protected provenance;

safe plugin handling;

authorization;

tenant isolation;

resource exhaustion controls.


Testing

unit tests;

property tests;

fuzzing;

fault injection;

deterministic replay;

scalability tests;

distributed tests;

stress tests;

soak tests;

end-to-end tests.



---

261. Required production invariants

The following are mandatory invariants.

Invariant 1 — No artificial qubit ceiling

The resilience subsystem MUST NOT define a universal maximum qubit count.

Invariant 2 — Canonical identity

Quantum identities MUST use quantum::ir::qubit.

Invariant 3 — Logical/physical separation

Logical and physical resources MUST remain distinguishable.

Invariant 4 — Dynamic capabilities

Capabilities MUST be discovered/configured, not assumed.

Invariant 5 — No provider lock-in

Core resilience MUST remain provider-neutral.

Invariant 6 — No duplicated quantum infrastructure

Routing, scheduling, optimization, QEC, hardware and simulation remain owned by their respective subsystems.

Invariant 7 — No uncontrolled growth

Memory, telemetry, history and planner state MUST have resource-aware behavior.

Invariant 8 — No infinite recovery loops

Recovery MUST terminate, escalate, or make measurable progress according to policy.

Invariant 9 — No stale execution

Plans MUST be revalidated against current resource state.

Invariant 10 — No silent semantic changes

Adaptation MUST preserve semantics or explicitly reject the workload.

Invariant 11 — Verification before acceptance

Recovery success is not equivalent to result correctness.

Invariant 12 — No unsafe Rust

unsafe MUST NOT appear in the resilience subsystem.


---

262. Final architecture

The scalable resilience architecture is:

ZAMANI PROGRAM
                               |
                               v
                     CANONICAL QUANTUM IR
                               |
                               v
                    PROGRAM REQUIREMENTS
                               |
                               v
                    RESOURCE DISCOVERY
                               |
                 +-------------+-------------+
                 |             |             |
                 v             v             v
             Hardware        QEC        Execution Model
                 |             |             |
                 +-------------+-------------+
                               |
                               v
                       CAPABILITY MODEL
                               |
                               v
                    OPTIMIZATION / ROUTING
                               |
                               v
                         SCHEDULING
                               |
                               v
                           EXECUTION
                               |
                               v
                         TELEMETRY
                               |
                               v
                         DETECTION
                               |
                               v
                         DIAGNOSIS
                               |
                               v
                           POLICY
                               |
                               v
                           PLANNING
                               |
                               v
                          ADAPTATION
                               |
             +-----------------+------------------+
             |                 |                  |
             v                 v                  v
          Remap             Reroute           Reschedule
             |                 |                  |
             +-----------------+------------------+
                               |
                               v
                           RECOVERY
                               |
                               v
                          MITIGATION
                               |
                               v
                          VERIFICATION
                               |
                  +------------+------------+
                  |                         |
                  v                         v
               ACCEPT                    REPLAN
                  |                         |
                  v                         |
                RESULT <--------------------+


---

263. The ultimate scalability property

The intended Zamani programming model is:

PROGRAM
   |
   | written once
   v
CANONICAL SEMANTICS
   |
   | resource-independent
   v
EXECUTION FABRIC
   |
   +--> tiny quantum machine
   |
   +--> small QPU
   |
   +--> large QPU
   |
   +--> modular QPU
   |
   +--> fault-tolerant QPU
   |
   +--> simulator
   |
   +--> emulator
   |
   +--> multiple QPUs
   |
   +--> distributed quantum system
   |
   +--> future quantum architecture

The program remains stable.

The realization changes.


---

264. What "write once" means in Zamani

A Zamani developer SHOULD be able to write:

logical quantum program

without writing:

provider-specific qubit numbers
physical topology
retry counts
hardware-specific thresholds
backend-specific recovery
fixed QEC configuration
fixed scheduler assumptions
fixed machine size

The system determines those properties from:

program semantics
requirements
capabilities
policy
resources
telemetry


---

265. What "atom to everywhere" means

The architecture therefore supports the following conceptual progression:

SCALE
                       |
                       v
       +---------------+---------------+
       |                               |
   physical                       organizational
       |                               |
       v                               v
   one qubit                    one backend
       |                               |
       v                               v
      QPU                         backend fleet
       |                               |
       v                               v
 modular QPU                    heterogeneous fleet
       |                               |
       v                               v
 logical QPU                    distributed system
       |                               |
       +---------------+---------------+
                       |
                       v
                Quantum Fabric

No resilience abstraction should have to be redesigned merely because the system crosses one of these boundaries.


---

266. Final implementation rule

Every new resilience implementation MUST answer these questions before being merged:

1. Does it introduce a fixed machine-size assumption?


2. Does it introduce a provider-specific assumption?


3. Does it duplicate another quantum subsystem?


4. Does it use a noncanonical qubit identity?


5. Does it assume a fixed topology?


6. Does it assume a fixed number of retries?


7. Does it assume a fixed telemetry volume?


8. Does it require all data in memory?


9. Does it break deterministic replay?


10. Does it create a stale-plan risk?


11. Does it create a recovery loop?


12. Does it preserve semantic verification?


13. Does it work for one resource?


14. Does it work for arbitrarily large resource sets subject to available resources?


15. Does it remain valid if the backend changes?


16. Does it remain valid if the topology changes?


17. Does it remain valid if resources disappear?


18. Does it remain valid if resources are added?


19. Does it remain valid in distributed execution?


20. Does it remain safe without unsafe?



If any answer is "no", the implementation is not production-ready.


---

267. Final architectural statement

The resilience subsystem is scalable when:

machine size
topology
hardware technology
backend
provider
QEC configuration
calibration
resource availability
execution mode
fault distribution

are all runtime properties rather than compile-time assumptions.

The resilience subsystem MUST therefore be designed around:

canonical semantics
+
dynamic capabilities
+
resource discovery
+
policy
+
incremental adaptation
+
hierarchical recovery
+
distributed coordination
+
bounded resource consumption
+
verification

rather than around a particular quantum machine.

The fundamental contract is:

> Zamani programs are expressed once against logical quantum semantics. quantum::resilience adapts their realization to whatever valid quantum resources are available, from the smallest supported configuration to arbitrarily large heterogeneous and distributed systems, without imposing an artificial architectural ceiling.




---

268. Integration summary

The final ownership model is:

quantum::ir::qubit
    owns canonical qubit identity

quantum::ir
    owns canonical quantum semantics

quantum::zqn
    owns fault/noise semantics

quantum::hardware
    owns hardware/resource capabilities

quantum::routing
    owns logical → physical routing

quantum::scheduling
    owns execution scheduling/timing

quantum::optimization
    owns IR optimization

quantum::error_correction / QEC
    owns quantum error correction

quantum::simulation
    owns simulation

quantum::benchmarking
    owns benchmarking

quantum::resilience
    owns detection, diagnosis, policy,
    planning, adaptation, recovery,
    mitigation orchestration and verification

runtime/execution
    owns actual execution

The resilience subsystem therefore becomes the adaptive control plane above the existing quantum infrastructure rather than another implementation of that infrastructure.


---

269. Completion criterion

SCALABILITY.md is satisfied only when every production implementation under:

src/quantum/resilience/

can demonstrate:

No fixed machine-size assumptions
No fixed qubit ceiling
No provider lock-in
No logical/physical identity confusion
No uncontrolled memory growth
No mandatory centralized bottleneck
No uncontrolled recovery loops
No stale-plan execution
No silent semantic changes
No unsafe Rust
No duplicate quantum infrastructure
Dynamic resource discovery
Dynamic capability negotiation
Incremental adaptation
Hierarchical recovery
Distributed coordination
Deterministic replay
Scalable verification
Scalable telemetry
Scalable serialization
Parameterized testing

The resulting architecture is not "infinite" because it promises unlimited physical resources.

It is "infinite" in the architectural sense that the resilience subsystem itself does not impose a fixed machine-size ceiling. The available hardware, classical resources, execution policies, physical laws, and deployment limits determine how far a particular execution can scale.

### Repository integration points I explicitly corrected

A particularly important repository-specific correction is the qubit identity boundary. The repository's canonical IR explicitly identifies `quantum::ir::qubit::QubitId` / `PhysicalQubitId`, while compatibility modules alias back to those canonical types.   The scheduling layer also already distinguishes logical and physical references, so resilience should preserve that distinction rather than introduce another identity abstraction. 

The architecture also deliberately keeps resilience above routing, scheduling, hardware and QEC rather than duplicating them. This is important for the repository's existing separation of concerns and for scaling to hardware that Zamani does not yet know about. OpenQASM's logical-versus-physical execution model supports the same general separation. 10

I also incorporated current execution-model realities: large quantum workloads may use jobs, batches, sessions, parallel execution and changing resource availability, so `resilience` must treat execution capacity as dynamic rather than assuming one universal execution model. 11

The repository itself currently exposes the broader Zamani goal of "Omniversal Scale" and production-oriented resilience, so this document turns that aspiration into concrete engineering constraints rather than leaving it as a slogan. 

**One important implementation consequence:** I would now treat this `SCALABILITY.md` as a contract that must be frozen **before** implementing `model/resource.rs`, `model/capability.rs`, `limits/*`, `planning/*`, and `api/*`. That prevents those files from later introducing incompatible fixed-size assumptions.