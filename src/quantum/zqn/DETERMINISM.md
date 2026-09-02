Worked for 9s

Yes. I checked the current repository architecture and the existing ZQN/probability/core/characterization and quantum-level determinism conventions before writing this. In particular, the repository already has explicit seeded reproducibility patterns in QEC, benchmarking, characterization, and stochastic optimization, while ZQN's probability/core layers explicitly prohibit hidden/global RNGs and time-derived randomness.

The document below is therefore written as the normative src/quantum/zqn/DETERMINISM.md contract, rather than merely a discussion of randomness.

ZQN Determinism Contract

Path: "src/quantum/zqn/DETERMINISM.md"
Subsystem: Zamani Quantum Noise (ZQN)
Language: Rust 2021
Supported Rust: 1.97 / 1.97.1
Safety: "unsafe" forbidden
Status: Production architecture contract
Scope: Deterministic stochastic execution, reproducibility, replayability, parallel determinism, distributed determinism, characterization, simulation, calibration, fault generation, serialization, and cross-subsystem integration.

---

1. Purpose

This document defines the normative determinism contract for ZQN.

ZQN contains stochastic physical models. Stochastic does not mean irreproducible.

For a fixed deterministic execution specification, ZQN MUST produce the same logical stochastic realization regardless of:

- thread count;
- task scheduling;
- worker assignment;
- execution batching;
- streaming boundaries;
- process partitioning;
- distributed execution partitioning;
- backend-independent parallelization;
- memory allocation layout;
- machine address;
- wall-clock time;
- operating-system scheduling;
- unrelated executions.

The fundamental contract is:

same semantic input
+
same ZQN model
+
same model configuration
+
same calibration state
+
same target semantics
+
same determinism policy
+
same root seed
+
same execution identity
+
same ZQN algorithm/version
+
same relevant numerical configuration
=
same deterministic stochastic realization

If any determinism-relevant input changes, ZQN MUST NOT claim that the result is identical unless equivalence has been established explicitly.

---

2. Scope

This document governs determinism across:

probability/
channel/
fault/
noise/
operations/
calibration/
characterization/
simulation/
propagation/
target/
integration/
io/

It also defines integration requirements for:

quantum::ir
quantum::error_correction
quantum::routing
quantum::scheduling
quantum::hardware
quantum::memory
quantum::benchmarking
quantum::runtime

This document does NOT define:

- the mathematical definition of every noise channel;
- the canonical Quantum IR;
- routing algorithms;
- scheduling algorithms;
- QEC decoding algorithms;
- hardware-provider APIs;
- benchmark methodology.

It defines how those systems MUST preserve or explicitly declare determinism when they consume or produce ZQN stochastic behavior.

---

3. Fundamental principle

ZQN MUST distinguish four concepts:

deterministic semantics
deterministic execution
statistical reproducibility
physical repeatability

They are not equivalent.

3.1 Deterministic semantics

The same input specification means the same thing.

Example:

Depolarizing(p = 0.01)

always represents the same mathematical channel.

3.2 Deterministic execution

Given the same deterministic execution identity and root seed, ZQN generates the same stochastic realization.

3.3 Statistical reproducibility

Two executions may produce different individual samples while producing statistically equivalent distributions.

This is appropriate when deterministic replay was not requested.

3.4 Physical repeatability

A real QPU is not expected to produce identical physical measurement outcomes merely because the same program is executed twice.

Therefore ZQN MUST NOT claim that deterministic simulation is equivalent to deterministic physical hardware.

---

4. Determinism modes

ZQN MUST support explicit determinism policy.

The conceptual policy space is:

DeterminismPolicy
├── Deterministic
├── Statistical
├── Unspecified
└── External

A production implementation MAY expose more detailed variants, but the semantics MUST remain explicit.

4.1 Deterministic mode

The caller supplies a reproducibility identity containing sufficient information to reproduce the stochastic realization.

A root seed is REQUIRED.

No hidden entropy source is permitted.

4.2 Statistical mode

The caller requests statistically valid stochastic execution but does not require identical sample-by-sample replay.

The implementation MAY obtain entropy through an explicitly configured external mechanism.

It MUST NOT silently pretend that statistical mode is replayable.

4.3 Unspecified mode

This mode SHOULD be avoided for production scientific execution.

If supported for convenience, its result MUST be marked as non-reproducible unless the complete determinism inputs can be recovered.

4.4 External mode

Hardware or an external execution system supplies the physical outcomes.

ZQN records the available provenance but MUST NOT fabricate a deterministic seed for outcomes that originated externally.

---

5. Root seed

The root seed is the root of deterministic stochastic derivation.

Conceptually:

RootSeed
    │
    ├── program identity
    ├── model identity
    ├── calibration identity
    ├── target identity
    ├── execution identity
    └── shot identity
             │
             ▼
       derived streams

The root seed MUST be:

- explicitly supplied in deterministic mode;
- stored in reproducibility metadata;
- included in the execution identity;
- independent of wall-clock time;
- independent of process identity;
- independent of thread identity;
- independent of memory addresses;
- independent of allocation order.

The root seed MUST NOT be silently replaced by a generated seed.

---

6. No hidden randomness

ZQN MUST NEVER use:

thread_rng()

or an equivalent hidden global RNG for semantic stochastic behavior.

ZQN MUST NOT derive semantic randomness from:

system time
process ID
thread ID
memory address
pointer value
hash-map iteration order
OS scheduling
CPU core number
environment-variable ordering
uninitialized state

The existing ZQN probability architecture already establishes this principle: probability mathematics and random sampling are separate concerns, and the probability layer must not create hidden global RNG state or seed from time, process identity, thread identity, or memory addresses.

The ZQN core layer follows the same requirement and explicitly prohibits process-global random state and memory-address-based semantic identity.

---

7. RNG ownership

RNG state MUST have an explicit owner.

Preferred ownership:

execution context
        │
        ▼
deterministic sampling context
        │
        ▼
derived local RNG stream

Forbidden:

global RNG
    │
    ├── simulation
    ├── QEC
    ├── benchmark
    ├── characterization
    └── routing

A subsystem MUST NOT consume randomness belonging to another subsystem merely because both can access a shared RNG.

---

8. Deterministic stream derivation

ZQN MUST derive independent stochastic streams from stable semantic identities.

Conceptually:

derived_seed =
    Derive(
        root_seed,
        domain,
        program_identity,
        model_identity,
        calibration_identity,
        target_identity,
        operation_identity,
        resource_identity,
        shot_identity,
        stream_identity
    )

The exact derivation algorithm is an implementation detail of the ZQN determinism API, but it MUST be:

- deterministic;
- documented;
- versioned;
- independent of execution order;
- independent of thread count;
- independent of process layout;
- stable for the supported compatibility version.

---

9. Domain separation

Random streams MUST be domain-separated.

Examples:

noise
fault
measurement
characterization
simulation
benchmarking
calibration
trajectory
sampling

A stochastic draw used by one domain MUST NOT accidentally consume the stream intended for another.

Conceptually:

RootSeed
│
├── ZQN/noise
├── ZQN/fault
├── ZQN/measurement
├── ZQN/characterization
├── ZQN/simulation
└── ZQN/benchmarking

This prevents unrelated code changes from shifting every subsequent random draw.

---

10. Event-based rather than consumption-order-based randomness

This is a critical requirement.

ZQN MUST NOT define deterministic randomness as:

seed RNG
draw
draw
draw
draw
...

when the identity of each draw depends solely on previous RNG consumption.

That design breaks reproducibility when execution becomes parallel.

Instead, stochastic events MUST be addressable.

Conceptually:

randomness(
    root_seed,
    domain,
    operation_id,
    resource_id,
    shot_id,
    event_id
)

Therefore:

operation A
operation B
operation C

may execute in any valid parallel order while retaining their individual stochastic identities.

---

11. Why consumption-order RNG is forbidden

Suppose sequential execution produces:

A → random #1
B → random #2
C → random #3

and parallel execution produces:

B → random #1
A → random #2
C → random #3

The resulting physical realization changes even though the computation did not.

That violates ZQN's deterministic parallelism contract.

Therefore deterministic ZQN randomness MUST be derived from stable identities rather than execution order.

---

12. Stable identities

ZQN MUST use stable semantic identities.

For quantum resources, ZQN MUST use the canonical identifiers from:

crate::quantum::ir::qubit

where applicable.

In particular:

crate::quantum::ir::qubit::QubitId
crate::quantum::ir::qubit::PhysicalQubitId

remain authoritative.

ZQN MUST NOT create a competing generic:

zqn::QubitId

for the same semantic resource.

This is consistent with the quantum composition root, which establishes "quantum::ir::qubit" as the canonical quantum identity boundary.

---

13. ZQN-owned identities

ZQN MAY define identities for entities that ZQN itself owns.

Examples:

NoiseModelId
ChannelId
FaultId
CalibrationId
CalibrationSnapshotId
CharacterizationId
ExperimentId
ObservationId
NoiseEventId

These MUST NOT replace canonical quantum resource identities.

For example:

NoiseEventId
    references
PhysicalQubitId

rather than manufacturing another physical-qubit namespace.

---

14. Operation identity

A deterministic noise event MUST be associated with a stable operation identity whenever possible.

The identity SHOULD be derived from canonical IR semantics rather than vector position alone.

A robust conceptual identity is:

program identity
+
IR operation identity
+
semantic occurrence

The identity MUST survive valid execution parallelization.

It MUST NOT depend solely on:

thread number
worker number
Vec position after arbitrary transformation

unless that position itself is part of the canonical semantic identity.

---

15. Shot identity

Every independently sampled execution SHOULD have a stable shot identity.

Conceptually:

ShotId

may be:

0
1
2
...

but ZQN MUST NOT require shots to execute sequentially.

Therefore:

shot 100

must be reproducible whether executed:

first
last
on worker 1
on worker 1000
on another machine

provided all deterministic inputs are identical.

---

16. Parallel determinism

The following executions MUST be equivalent under deterministic mode:

1 worker
2 workers
4 workers
8 workers
64 workers

provided:

- the same deterministic execution specification is used;
- no target-dependent nondeterminism is introduced;
- floating-point reduction order is either fixed or its numerical tolerance is explicitly declared.

The result MUST NOT depend on task scheduling.

---

17. Parallel RNG design

A preferred conceptual architecture is:

RootSeed
   │
   ▼
ExecutionIdentity
   │
   ├── Shot 0
   │     ├── Operation A
   │     ├── Operation B
   │     └── Operation C
   │
   ├── Shot 1
   │     ├── Operation A
   │     ├── Operation B
   │     └── Operation C
   │
   └── Shot N

Each node derives its own stream.

Workers merely execute already-addressed work.

Workers MUST NOT decide semantic random identities.

---

18. Distributed determinism

Distributed execution MUST preserve the same identity model.

Conceptually:

global root seed
       │
       ▼
execution identity
       │
       ├── node A
       ├── node B
       ├── node C
       └── node N

Node identity MAY participate in stream derivation when the event is genuinely node-specific.

However, merely moving an event between nodes MUST NOT change its stochastic identity if the event itself is semantically unchanged.

Therefore:

event → node A

and:

event → node B

MUST produce the same stochastic realization when node placement is not part of the event's physical semantics.

---

19. Distributed execution and physical semantics

If node placement genuinely changes the physical model, then node identity MAY be a semantic input.

For example:

network link
node
physical resource
local calibration

may legitimately affect noise.

In that case the node/resource identity MUST be explicitly represented in the target/calibration/noise context.

It MUST NOT be introduced accidentally by the executor.

---

20. Streaming determinism

ZQN MUST support streaming execution.

A deterministic stream MUST NOT depend on batch size.

For example:

1000 events in one batch

and:

10 batches × 100 events

must produce the same event-level stochastic realization.

Likewise:

one iterator

and:

parallel chunks

must preserve event identity.

---

21. Lazy evaluation

Lazy stochastic generators MUST preserve the same deterministic identity semantics as eager generation.

These two executions:

generate all faults → consume

and:

generate fault when requested → consume

MUST agree when deterministic semantics are otherwise identical.

---

22. Fault batching

"fault/batch.rs" MUST NOT assign random outcomes merely according to the order in which a batch happens to be materialized.

Instead:

FaultEventId

or an equivalent stable event identity MUST determine the stochastic outcome.

This is essential when:

batch size

changes.

---

23. Correlated noise

Correlation MUST NOT be implemented by repeatedly consuming unrelated random streams in an order-sensitive manner.

A correlated event SHOULD have a shared semantic correlation identity.

Conceptually:

CorrelationEvent
├── correlation domain
├── participating resources
├── operation/time context
└── event identity

The participant set MUST be canonically ordered for identity purposes.

Ordering MUST NOT be inherited accidentally from a hash map or nondeterministic collection traversal.

---

24. Canonical ordering

Whenever ordering affects deterministic computation, ZQN MUST use an explicitly defined stable ordering.

Forbidden:

HashMap iteration order

as semantic ordering.

The repository's ZQN core contract already identifies unordered iteration as something that must not become a semantic ordering mechanism.

When a collection is mathematically unordered but an algorithm needs an ordering, ZQN MUST derive a canonical ordering from stable semantic identity.

---

25. Canonical resource ordering

For collections of quantum resources:

QubitId
PhysicalQubitId

or another authoritative resource identity MUST determine ordering where ordering is necessary.

Do not sort by:

memory address
pointer
thread
allocation sequence
hash-map bucket

---

26. Deterministic sampling API

The sampling API SHOULD conceptually separate:

distribution

from:

sampling context

For example:

Distribution
SamplingContext

rather than allowing a distribution object to own hidden RNG state.

The probability subsystem already establishes that probability mathematics and random sampling are separate concerns.

---

27. RNG state lifetime

A local RNG object MAY exist during a computation.

However:

RNG state

MUST NOT become the only source of semantic identity.

The deterministic identity MUST remain reconstructible from the execution specification.

This allows:

checkpoint
replay
parallel execution
distributed execution
partial execution

without relying on an opaque mutable RNG history.

---

28. Checkpointing

A deterministic execution SHOULD be checkpointable.

A checkpoint MUST contain sufficient information to continue or replay the deterministic computation.

At minimum, depending on execution stage:

ZQN schema/version
model identity
configuration identity
calibration identity
target identity
root seed
execution identity
current semantic position
shot identity
determinism policy
numerical configuration

If mutable RNG state is used internally for performance, checkpointing MUST additionally preserve the exact state required for continuation.

---

29. Replay

Replay MUST be a first-class concept.

A replay request should conceptually be:

original execution specification
+
recorded provenance
+
recorded deterministic inputs

rather than:

try to run it again with the same current configuration

Replay MUST fail explicitly if a required deterministic input is unavailable.

It MUST NOT silently substitute a different model or calibration state.

---

30. Reproducibility identity

ZQN SHOULD expose a canonical reproducibility identity.

Conceptually:

ReproducibilityIdentity {
    schema_version
    zqn_version
    model_identity
    model_configuration_identity
    calibration_identity
    target_identity
    program_identity
    root_seed
    execution_identity
    numerical_configuration
    algorithm_revision
}

The exact Rust structure belongs in the appropriate ZQN core/simulation module, not in this documentation file.

---

31. Model identity

A model name alone is insufficient.

This is unsafe:

"depolarizing"

A reproducibility identity MUST distinguish at least:

model semantics
parameters
representation
schema/version
relevant implementation revision

For example, changing:

p = 0.01

to:

p = 0.02

MUST change the model/configuration identity.

---

32. Calibration identity

Noise depends on calibration.

Therefore a deterministic replay MUST identify the calibration state used by the model.

At minimum:

CalibrationSnapshotId

or an equivalent immutable identity MUST be recorded.

A replay MUST NOT silently use the latest calibration.

This is particularly important because calibration may drift.

---

33. Calibration snapshots

A calibration snapshot SHOULD be immutable for deterministic execution.

Conceptually:

CalibrationSnapshot
├── identity
├── source
├── validity interval
├── parameters
├── uncertainties
└── provenance

A mutable calibration object MUST NOT change the result of an execution that has already been assigned a deterministic calibration identity.

---

34. Time-dependent noise

Time-dependent noise requires special treatment.

There are two distinct concepts:

semantic execution time

and:

wall-clock time

Only semantic execution time MAY affect deterministic noise semantics.

Wall-clock time MUST NOT silently influence deterministic noise.

For example:

operation start = 10 ns
duration = 20 ns

may affect a drift model.

But:

system clock = 14:03:27

MUST NOT.

---

35. Drift

For deterministic replay, drift MUST be determined by an explicit model and explicit temporal context.

Conceptually:

drift(
    calibration_snapshot,
    semantic_time,
    resource_identity,
    model_parameters
)

not:

drift(current_system_time)

---

36. Non-Markovian noise

Non-Markovian noise may depend on execution history.

Therefore deterministic replay MUST preserve the relevant semantic history/environment state.

The environment state MUST NOT depend on:

thread scheduling

or:

execution order of semantically independent events

unless the ordering itself is part of the physical semantics.

---

37. Conditional noise

For noise conditioned on:

measurement result
classical value
operation result
environment state

the condition MUST be derived from the semantic execution state.

It MUST NOT depend on incidental runtime state.

---

38. Dynamic circuits

Dynamic quantum programs may branch based on measurements.

Determinism means:

same deterministic inputs
+
same stochastic measurement realization

produce the same branch.

ZQN MUST NOT independently invent branch decisions.

The canonical IR/runtime remains responsible for program control flow.

ZQN supplies the noise/measurement stochastic realization.

---

39. Measurement noise

Measurement noise MUST be independently addressable.

For example:

measurement event
+
resource identity
+
shot identity

must determine its stochastic realization.

The measurement subsystem MUST NOT consume arbitrary shared RNG state belonging to previous gates.

---

40. Reset noise

Reset stochasticity MUST have its own event identity.

Reset behavior MUST remain reproducible when:

gate count changes elsewhere

provided the reset's own semantic identity remains unchanged.

---

41. Idle noise

Idle noise is especially sensitive to scheduling.

The deterministic model MUST depend on:

semantic idle interval
resource identity
calibration
noise model

not on the order in which the scheduler happens to process idle regions.

---

42. Pulse noise

Pulse noise MUST use stable pulse/operation identities.

If pulse generation is deterministic and equivalent, noise sampling MUST remain deterministic even if pulses are executed by different workers.

---

43. Transport noise

Transport operations MUST use stable transport identities.

For example:

resource
source
destination
operation identity
transport interval

may participate in the stochastic identity.

The identity MUST come from semantic execution data rather than executor placement.

---

44. Characterization determinism

Characterization experiments MUST explicitly identify:

experiment
protocol
configuration
root seed
sampling policy
target
calibration

The repository already contains an explicit "ReproducibilityPolicy" in ZQN characterization, including caller-supplied seed handling.

That policy MUST integrate with the common ZQN determinism contract rather than creating a second incompatible determinism system.

---

45. Randomized benchmarking

Randomized benchmarking MUST use the same fundamental ZQN determinism principles.

The current repository already uses explicit seeded randomized benchmarking generation.

ZQN characterization MUST NOT introduce another unrelated RNG scheme.

The common model should be:

experiment seed
+
protocol identity
+
sequence identity
+
operation identity

→ deterministic random sequence.

---

46. Benchmarking integration

The broader benchmarking subsystem already has deterministic seed concepts and explicitly prohibits time-derived randomness and implicit global mutable RNGs.

Therefore:

benchmark seed

and:

ZQN root seed

MUST have a defined relationship.

Recommended:

BenchmarkSeed
      │
      ▼
Benchmark execution identity
      │
      ▼
ZQN derived domain seed

ZQN MUST NOT independently create another unrelated seed.

---

47. Random circuit generation

The existing benchmarking architecture explicitly treats the random algorithm version, generator revision, benchmark seed, and Quantum IR implementation as determinism-relevant inputs.

ZQN integration MUST preserve this principle.

If a generated circuit changes, the execution identity MUST change.

Otherwise replay could incorrectly claim equivalence between different circuits.

---

48. QEC integration

ZQN MUST integrate with:

quantum::error_correction

without creating a second deterministic RNG architecture.

The existing QEC noise implementation already provides explicit deterministic seed access.

During migration:

existing QEC seed policy
        │
        ▼
ZQN determinism adapter
        │
        ▼
canonical ZQN deterministic sampling

The QEC subsystem remains responsible for:

syndrome
decoding
correction
logical fault processing

ZQN remains responsible for:

physical noise
channels
fault realization
stochastic identity

---

49. QEC replay compatibility

QEC replay MUST be able to identify the ZQN noise configuration that generated physical faults.

A QEC replay record therefore SHOULD contain:

ZQN version
noise model identity
noise configuration identity
root seed
execution identity
calibration identity
target identity

This allows:

same physical fault realization

to be reconstructed for deterministic simulation.

---

50. Routing integration

Routing may query ZQN for noise costs.

Routing MUST NOT mutate ZQN's deterministic state merely by asking for a cost.

A cost query MUST be referentially transparent where the queried semantics are deterministic.

For example:

cost(operation, target, calibration)

must not consume stochastic samples.

If a stochastic estimate is explicitly requested, it MUST use a separately identified sampling context.

---

51. Scheduling integration

Scheduling MUST provide semantic timing information to ZQN.

ZQN MUST NOT infer scheduling semantics from executor order.

For example:

idle duration = 50 ns

is semantic.

worker 4 processed this operation first

is not semantic.

---

52. Hardware integration

Hardware adapters MUST distinguish:

deterministic configuration

from:

physical stochastic outcome

A real hardware backend MUST NOT fabricate deterministic outcomes simply because a seed was supplied.

For real QPU execution:

root seed

may identify the requested software experiment, but it does not control physical quantum randomness unless the hardware explicitly provides such a mechanism.

---

53. Hardware provenance

For physical hardware execution, record:

target identity
device identity
calibration identity
execution identity
submission identity
shot identity where available
timestamp

The timestamp is provenance, not a hidden random input.

---

54. Simulation integration

"simulation/" MUST consume explicit deterministic sampling context.

The simulator MUST NOT maintain a hidden global RNG.

Recommended conceptual flow:

SimulationRequest
       │
       ▼
ZqnContext
       │
       ▼
DeterminismContext
       │
       ▼
NoiseModel
       │
       ▼
event-addressed samples

---

55. Monte Carlo integration

Monte Carlo execution MUST be reproducible under deterministic mode.

Changing:

worker count
batch size
execution partition

MUST NOT change the addressed random samples.

If a Monte Carlo algorithm deliberately changes its estimator because of execution configuration, that configuration MUST be part of the reproducibility identity.

---

56. Quantum trajectories

Quantum trajectories MUST use explicit trajectory identities.

Conceptually:

trajectory_id
+
shot_id
+
operation_id

must determine stochastic trajectory events.

---

57. Numerical determinism

Randomness is not the only source of nondeterminism.

Floating-point reductions can also vary with execution order.

Therefore ZQN MUST distinguish:

stochastic determinism

from:

numerical bitwise determinism

A production numerical algorithm MUST declare which guarantee it provides.

---

58. Bitwise determinism

When bitwise reproducibility is required:

same inputs

MUST produce identical serialized numerical results under the supported execution environment.

Reduction order MUST therefore be controlled.

Examples:

sum(a,b,c,d)

must not become:

((a+b)+c)+d

in one execution and:

(a+(b+c))+d

in another if bitwise equality is promised.

---

59. Tolerance-based determinism

Some numerical algorithms may provide mathematically equivalent results within an explicit tolerance rather than bitwise equality.

Such APIs MUST report:

numerical tolerance

and MUST NOT call the result bitwise deterministic.

---

60. Approximation

An approximate noise model MUST identify:

approximation method
approximation parameters
error tolerance
algorithm/version

A replay MUST use the same approximation specification if exact reproducibility is required.

---

61. Representation changes

Equivalent representations such as:

Kraus
Choi
Liouville
Pauli transfer

may produce numerically different intermediate values.

Therefore deterministic identity MUST distinguish:

semantic equivalence

from:

implementation identity

If a representation change is permitted to preserve the same result, the compatibility contract MUST explicitly define the acceptable equivalence tolerance.

---

62. Canonical serialization

The ZQN I/O layer MUST provide canonical serialization for determinism-relevant objects.

Canonical serialization MUST:

- use explicit field ordering;
- use stable representation;
- avoid hash-map ordering;
- encode numeric values unambiguously;
- include schema version;
- preserve required identity fields;
- preserve required precision information.

---

63. Hashing

If an object identity is derived from serialized semantics, hashing MUST operate on canonical serialization.

Never hash:

Rust struct memory layout

or:

pointer address

or:

debug output

as a semantic identity.

---

64. Versioning

The following MUST be distinguishable:

ZQN semantic version
ZQN schema version
deterministic algorithm version
random derivation version
serialization version

Changing the deterministic random derivation algorithm MUST NOT silently preserve the old identity.

It MUST either:

1. preserve the old algorithm for compatibility, or
2. increment the relevant determinism algorithm version.

---

65. Random algorithm version

A deterministic sequence is not fully identified by:

seed

alone.

It also depends on:

random derivation algorithm

and potentially:

sampling algorithm

Therefore the reproducibility identity MUST include the applicable algorithm version.

---

66. Sampling algorithm changes

Changing from:

algorithm A

to:

algorithm B

may produce a statistically equivalent distribution while producing different samples.

That is acceptable only if the compatibility contract declares:

statistical compatibility

rather than:

exact replay compatibility

---

67. Determinism and optimization

Compiler optimization MUST NOT silently alter deterministic stochastic semantics.

An optimization may reorder operations only when that transformation preserves the relevant semantic dependencies.

If reordering changes the physical noise semantics, the optimizer MUST treat that as a semantic consequence.

For example:

A
B

cannot be freely reordered if:

B

depends on the temporal noise caused by:

A

---

68. Determinism and routing

Routing changes physical resource identity.

Therefore a routing transformation MAY legitimately change the physical noise realization because:

logical resource

maps to a different:

PhysicalQubitId

That is not nondeterminism.

It is a different target realization.

The transformation MUST nevertheless be deterministic for the same:

IR
target
routing policy
noise model
calibration

---

69. Determinism and scheduling

Scheduling changes semantic timing.

Therefore:

schedule A

and:

schedule B

may legitimately produce different noise.

Again, this is not nondeterminism.

It is a different deterministic execution specification.

---

70. Determinism and QEC

QEC transformations may change:

operations
ancillas
syndrome extraction
timing
fault opportunities

The deterministic identity MUST therefore be attached to the actual semantic execution specification after the transformation that defines the physical fault model.

---

71. Determinism and hardware capabilities

Different targets may produce different deterministic noise realizations because they have different:

capabilities
topology
calibration
timing
native operations

Write-once-scale-everywhere means:

same source intent

not:

identical physical noise outcome on every target

The target realization remains target-dependent.

---

72. Scaling requirement

Determinism MUST have no architectural upper bound on:

number of qubits
number of physical resources
number of operations
number of shots
number of correlated resources
number of nodes
circuit depth
execution duration

There MUST be no:

MAX_QUBITS
MAX_SHOTS
MAX_OPERATIONS

inside the determinism semantics merely to make the API convenient.

Resource limits belong to:

core::limits
runtime policy
memory policy
target capabilities

---

73. Tiny-to-large execution

The same determinism contract MUST apply to:

one resource

through:

large distributed systems

without changing the semantic API.

Only implementation strategy may change.

For example:

small system → direct RNG stream
large system → counter/address-derived stream
distributed system → partitioned deterministic streams

may be valid implementation strategies if the declared deterministic algorithm contract remains stable.

---

74. "Infinity" clarification

ZQN uses "infinity" only as an architectural scalability principle.

It means:

«ZQN imposes no artificial finite semantic machine-size ceiling.»

It does NOT mean:

«an implementation can allocate infinite memory or execute an infinite computation.»

Actual execution remains bounded by:

memory
CPU
GPU
storage
network
hardware
time
numerical feasibility
runtime policy

This matches the broader quantum architecture's distinction between semantic scalability and actual resource capacity.

---

75. Resource limits and determinism

Resource exhaustion MUST NOT silently change semantics.

For example:

full exact representation

MUST NOT silently become:

approximate representation

because memory became insufficient.

Instead:

ResourceLimitExceeded

or an explicitly declared approximation policy MUST be returned.

---

76. Out-of-memory behavior

ZQN MUST prefer explicit failure over uncontrolled semantic degradation.

If an operation cannot be represented within the configured resource policy:

fail explicitly

rather than:

change mathematical semantics silently

---

77. Cancellation

Cancellation MUST be explicit.

A cancelled deterministic execution MUST be distinguishable from:

success

and:

different stochastic execution

A resumed execution MUST use a valid checkpoint/replay contract.

---

78. Retry semantics

Retries MUST NOT accidentally consume a new semantic random stream.

For deterministic execution:

retry(event)

must reproduce:

event

rather than produce:

next random event

unless the retry is explicitly defined as a new execution/shot.

---

79. Failure recovery

Distributed workers may fail.

The replacement worker MUST be able to reconstruct the same stochastic work item from its semantic identity.

Therefore:

work item identity

MUST be sufficient to reconstruct the deterministic random stream.

---

80. Task scheduling

Task schedulers MUST NOT influence semantic randomness.

Forbidden model:

worker receives next RNG state

Preferred model:

worker receives event identity
worker derives event RNG

---

81. Thread safety

Where semantically possible, deterministic ZQN contexts and immutable model objects SHOULD be:

Send + Sync

No global mutable RNG is permitted.

No global mutable calibration state may influence deterministic semantics.

---

82. No unsafe implementation

The entire ZQN determinism architecture MUST remain safe Rust.

The ZQN tree MUST contain:

#![forbid(unsafe_code)]

at an appropriate module boundary.

No deterministic guarantee may depend on:

raw pointers
unsafe memory manipulation
undefined behavior
unsafe FFI assumptions

---

83. Memory addresses are never semantic

A memory address MUST NEVER influence:

NoiseModelId
FaultId
NoiseEventId
random seed
operation identity
serialization identity
canonical ordering

This explicitly follows the existing ZQN core rule against memory-address-based semantic identity.

---

84. Process and thread identity

Process and thread identities MUST NOT influence deterministic semantics.

They MAY appear in operational telemetry/provenance if useful, but they MUST NOT enter deterministic stochastic derivation.

---

85. Wall-clock time

Wall-clock time MUST NOT seed deterministic stochastic behavior.

Time MAY be recorded as provenance:

execution started at ...

but that timestamp MUST NOT silently affect:

noise event
fault event
sample
trajectory
benchmark sequence

---

86. Environment variables

Environment variables MUST NOT silently affect deterministic stochastic behavior.

If an environment setting is semantically relevant, it MUST be explicitly captured in the execution identity.

---

87. Hardware randomness

Hardware entropy may be used only when the caller explicitly requests a non-deterministic/statistical mode or explicitly provides it as an external entropy source.

ZQN MUST NOT silently mix:

root deterministic seed

with:

OS entropy

and still claim deterministic replay.

---

88. Cryptographic versus simulation randomness

ZQN stochastic simulation does not automatically require cryptographic randomness.

The determinism contract concerns:

reproducibility

not:

cryptographic unpredictability

If cryptographic randomness is ever required by another subsystem, it MUST have a separate explicit security contract.

---

89. Statistical validation

Deterministic replay MUST NOT be confused with statistical correctness.

Both are required:

determinism tests

and:

distribution tests

A deterministic but mathematically incorrect sampler is still incorrect.

---

90. Distribution invariants

Sampling implementations MUST preserve the mathematical distribution declared by the corresponding probability object.

Tests SHOULD verify:

normalization
bounds
support
moments

where applicable.

---

91. Reproducibility tests

Every stochastic ZQN implementation MUST have tests covering:

same seed → same result
different seed → permitted different result
same event identity → same result
reordered execution → same result
different worker count → same result
different batch size → same result
replay → same result
serialization round trip → same identity

---

92. Parallel determinism test

At minimum, test:

workers = 1
workers = 2
workers = 4
workers = implementation-defined larger count

The test MUST compare the deterministic event/result identity, not merely aggregate statistics.

---

93. Batch-size test

Compare:

batch = 1
batch = 7
batch = 32
batch = 1000

for the same deterministic execution.

Results MUST agree.

---

94. Ordering test

Generate a set of semantically independent events.

Execute them:

A B C D

then:

D B A C

and verify each event's stochastic realization is identical.

This test is particularly important for proving event-addressed randomness.

---

95. Distributed-equivalence test

Where distributed execution support exists:

single process

and:

partitioned execution

MUST produce equivalent deterministic event results.

The partition boundary MUST NOT affect semantic randomness.

---

96. Checkpoint/replay test

Test:

run
checkpoint
stop
restore
continue

against:

run uninterrupted

The deterministic result MUST agree.

---

97. Serialization test

Test:

object
↓
serialize
↓
deserialize
↓
canonical identity

The identity MUST remain unchanged.

If serialization changes a semantically relevant field, the identity MUST change.

---

98. Version compatibility tests

When deterministic algorithm compatibility is promised:

version N

must replay according to the declared compatibility rules.

When compatibility is intentionally broken:

version N+1

MUST identify the incompatibility explicitly.

---

99. Fuzz testing

Determinism-related parsers and serializers SHOULD be fuzzed for:

malformed seeds
invalid identities
invalid versions
huge identifiers
empty identifiers
duplicate identifiers
malformed serialized execution contexts

The invariant is:

no panic
no UB
no uncontrolled allocation
no silent semantic corruption

---

100. Property testing

Useful properties include:

derive(seed, identity) == derive(seed, identity)

always.

And:

derive(seed, identity_a)

must not depend on the presence or absence of unrelated:

identity_b

unless the specification explicitly makes them correlated.

---

101. Independence property

Adding an unrelated event MUST NOT shift the random stream of existing events.

Bad:

A → random #0
B → random #1

Adding C:

A → random #0
C → random #1
B → random #2

This changes B.

Preferred:

A → derive(A)
B → derive(B)

Adding C:

A → derive(A)
B → derive(B)
C → derive(C)

A and B remain unchanged.

---

102. Correlation exception

The independence rule does not prohibit intentional physical correlation.

If:

A

and:

B

are physically correlated, they MUST share a declared correlation domain/event identity.

That correlation is semantic.

It must not arise accidentally because two calls happened to use the same mutable RNG.

---

103. Determinism and correlation topology

Correlation participants MUST have canonical identity.

For example:

{Qubit 3, Qubit 1, Qubit 2}

must canonicalize identically to:

{Qubit 1, Qubit 2, Qubit 3}

when the mathematical correlation is an unordered set.

---

104. Deterministic maps and sets

Any collection participating in deterministic identity MUST have:

- stable serialization;
- stable canonical ordering;
- stable equality;
- stable hashing.

Rust's unspecified hash-map iteration order MUST never become part of the semantic result.

---

105. Floating-point environment

If ZQN promises bitwise numerical reproducibility, it MUST define the relevant floating-point environment.

Where the environment cannot be controlled portably, ZQN SHOULD provide a tolerance-based guarantee instead.

The API/documentation MUST state which guarantee applies.

---

106. CPU/GPU differences

A CPU and GPU implementation may produce numerically different values.

Therefore:

CPU == GPU

must not automatically mean:

bitwise identical

unless the implementation establishes that guarantee.

It may instead promise:

mathematically equivalent within declared tolerance

---

107. SIMD/vectorization

Vectorization MUST NOT change semantic stochastic identity.

A vectorized sampler MAY generate many events at once, but each event remains addressed by its semantic identity.

---

108. GPU execution

GPU execution MUST NOT use:

thread/block ID

as an accidental semantic seed.

GPU execution may use those identifiers internally for addressing, but the mapping from semantic event identity to random state MUST remain deterministic.

---

109. Accelerator independence

The same deterministic specification SHOULD be portable across:

CPU
GPU
accelerator
distributed workers

subject to the declared numerical compatibility contract.

---

110. Algorithmic implementation changes

An internal optimization may change:

RNG implementation

without changing the public deterministic sequence only if the deterministic algorithm compatibility contract permits it.

Otherwise the change MUST increment the relevant algorithm identity/version.

---

111. Stable random derivation

The random derivation algorithm MUST NOT be tied to Rust's:

DefaultHasher

or any implementation-defined hash behavior for long-term scientific identity.

If hashing participates in semantic random derivation, the algorithm MUST be explicitly specified and versioned.

---

112. Hash stability

Do not assume:

Rust Hash implementation

is a permanent scientific serialization contract.

ZQN SHOULD define a canonical deterministic encoding and an explicitly versioned derivation function.

---

113. Seed representation

The public seed representation MUST be unambiguous.

If a "u64" root seed is used by the current implementation, its:

width
endianness
encoding

must be defined for serialization/derivation purposes.

Future larger seed types MUST be possible without redesigning the architecture.

---

114. Scalability of seed derivation

The deterministic identity system MUST scale without requiring a preallocated RNG stream for every possible resource.

Do not allocate:

one RNG object per qubit

for a billion-resource machine merely to satisfy the API.

Prefer:

addressable derivation

and lazy/local state.

---

115. Memory scalability

Determinism metadata MUST scale with actual execution needs.

A system must not require a giant global table:

all future random events

before execution begins.

Event identities should be derivable on demand.

---

116. Infinite/long-running streams

For long-running executions, deterministic event identities MUST support arbitrarily many events subject only to the actual identifier representation/resource limits.

No semantic constant such as:

MAX_RANDOM_EVENTS

should define the quantum architecture.

---

117. Identifier exhaustion

If an implementation's identifier representation has a finite range, exhaustion MUST result in explicit failure.

It MUST NOT silently wrap around.

For example:

event_id + 1

must use checked arithmetic where overflow could occur.

---

118. No silent wrapping

Deterministic identities MUST NOT silently wrap.

Forbidden:

255 + 1 → 0

when that would create an identity collision.

---

119. Collision resistance

If an identity is derived from hashing, the design MUST address collision risk appropriate to its role.

A hash used merely as an internal optimization identifier is different from a cryptographic content identity.

The implementation MUST NOT incorrectly claim cryptographic uniqueness where it only provides probabilistic collision resistance.

---

120. Canonical execution identity

An execution identity SHOULD conceptually include:

program
model
target
calibration
configuration
determinism policy
seed
algorithm versions

It may additionally include:

experiment
benchmark
optimization pipeline
routing result
schedule

when those affect physical execution.

---

121. Execution identity after transformations

If optimization, routing, scheduling, or QEC changes a semantically relevant part of the execution, the resulting execution identity MUST change or derive a new child identity.

This prevents replay from accidentally referring to the wrong physical realization.

---

122. Identity hierarchy

A useful conceptual hierarchy is:

ProgramIdentity
      │
      ▼
CompilationIdentity
      │
      ▼
ExecutionIdentity
      │
      ├── ShotIdentity
      │
      └── EventIdentity

This allows reproducibility at multiple levels.

---

123. Parent-child derivation

Child identities SHOULD derive from parent identities.

Conceptually:

program
  ↓
execution
  ↓
shot
  ↓
operation
  ↓
event

This gives deterministic hierarchy without requiring global mutable state.

---

124. Provenance integration

"core/provenance.rs" MUST be capable of recording the determinism-relevant inputs.

At minimum, provenance SHOULD identify:

determinism policy
root seed presence
root seed identity where permitted
model identity
calibration identity
target identity
algorithm versions

Sensitive secrets MUST NOT be placed into provenance merely because they happen to be available.

---

125. Security

A seed is not automatically a secret.

However, provenance and execution metadata may contain sensitive information.

ZQN MUST follow the broader repository security policy when deciding what can be serialized or exposed.

The determinism contract MUST NOT require exposing:

credentials
private keys
provider tokens

---

126. Privacy

Deterministic execution metadata SHOULD contain only what is necessary for replay and provenance.

Do not collect unrelated user data merely to make execution reproducible.

---

127. Deterministic logging

Logs MUST NOT influence deterministic semantics.

A debug log such as:

event generated

must not consume a random value or alter event identity.

---

128. Telemetry

Telemetry MUST be observational.

It MUST NOT modify:

RNG state
event identity
model identity
calibration identity

---

129. Error messages

Error formatting MUST NOT depend on unordered collection iteration if stable output is required.

Diagnostics SHOULD use canonical ordering.

---

130. Determinism and caching

A cached stochastic result MUST be keyed by all determinism-relevant inputs.

At minimum:

model identity
configuration identity
calibration identity
target identity
execution identity
seed
algorithm version

A cache MUST NOT use only:

model name

as its key.

---

131. Cache invalidation

Changing any determinism-relevant input MUST invalidate the applicable cached result.

Examples:

seed changed
calibration changed
model parameter changed
target changed
algorithm version changed

---

132. Deterministic memoization

Memoization is safe when:

function(input_identity)

is semantically deterministic.

Memoization MUST NOT introduce stateful RNG consumption that changes later results.

---

133. Replay across machines

Replay SHOULD be portable across machines when:

same supported ZQN version
same deterministic algorithm
same semantic input
same numerical compatibility profile

If machine-specific numerical behavior prevents bitwise replay, ZQN MUST report the weaker tolerance-based compatibility contract.

---

134. Replay across operating systems

Operating-system differences MUST NOT affect semantic random derivation.

Path separators, locale, environment ordering, and platform-specific formatting MUST NOT become semantic identity unless explicitly part of the input.

---

135. Locale

Deterministic serialization MUST NOT depend on locale.

Numeric serialization MUST use a canonical representation.

---

136. Unicode

If names/labels participate in identity, their canonical encoding MUST be defined.

Normalization differences MUST NOT silently produce two different identities for semantically identical identifiers.

---

137. Source program identity

If the source program participates in reproducibility, its identity SHOULD be based on canonical source or canonical lowered representation as appropriate.

Whitespace-only changes SHOULD NOT alter the semantic program identity if the frontend semantics treat them as equivalent.

---

138. Canonical IR identity

The preferred semantic identity should be derived from canonical:

quantum::ir

rather than frontend-specific AST layout.

This preserves the repository's architecture in which "quantum::ir" is the canonical semantic boundary.

---

139. Frontend independence

Different source representations that lower to the same canonical quantum semantics SHOULD be capable of producing the same canonical program identity.

Therefore:

Zamani source

and:

equivalent imported representation

may share semantic identity after canonicalization.

---

140. ZQN must not depend on frontend randomness

Parsing/lowering MUST NOT inject hidden random identities into ZQN.

If source generation itself is randomized, that randomness belongs to the generator and must be explicitly included in the generated program identity.

---

141. Integration with "core/context.rs"

"core/context.rs" SHOULD be the primary carrier for deterministic execution context.

Conceptually it may contain references/values for:

limits
capabilities
provenance
calibration
determinism
cancellation

The context MUST NOT contain a hidden global RNG.

---

142. Integration with "core/ids.rs"

"core/ids.rs" owns ZQN identities.

It MUST distinguish:

semantic resource identity

from:

ZQN object identity

Canonical quantum resources continue to use:

quantum::ir::qubit

identifiers.

---

143. Integration with "core/version.rs"

"core/version.rs" MUST expose the versions needed by the determinism contract.

At minimum:

ZQN version
schema version
deterministic derivation version

must be distinguishable.

---

144. Integration with "core/provenance.rs"

"core/provenance.rs" MUST be capable of storing deterministic provenance.

It MUST NOT itself generate randomness.

---

145. Integration with "core/limits.rs"

"core/limits.rs" governs resource safety.

It MUST NOT alter stochastic semantics merely because a limit is reached.

Resource failure is explicit.

---

146. Integration with "probability/"

"probability/" owns:

distribution mathematics
sampling abstractions

It does not own global random state.

The existing probability module explicitly establishes this separation.

---

147. Integration with "channel/"

Channels are mathematical objects.

Applying a stochastic channel requires a deterministic sampling context when a sample is needed.

Channel definitions themselves MUST remain deterministic.

---

148. Integration with "fault/"

Fault generation MUST use event-addressed deterministic sampling.

"FaultBatch" MUST remain independent of execution order.

---

149. Integration with "noise/"

"NoiseModel" MUST accept explicit context.

A noise model MUST NOT silently access:

global seed
global clock
global RNG
global calibration

---

150. Integration with "operations/"

Operations provide stable semantic locations for noise events.

Gate, measurement, reset, idle, pulse and transport noise MUST all use the same determinism framework.

---

151. Integration with "calibration/"

Calibration identity is a determinism input.

Calibration drift is deterministic only when its temporal/environmental inputs are explicitly specified.

---

152. Integration with "characterization/"

Characterization experiments MUST reuse the ZQN determinism framework.

Existing explicit reproducibility policy/seed behavior should be adapted rather than duplicated.

---

153. Integration with "simulation/"

"simulation/reproducibility.rs" MUST implement the execution-level reproducibility contract.

It SHOULD be the primary location for simulation-specific replay metadata.

---

154. Integration with "propagation/"

Error propagation calculations MUST distinguish:

deterministic mathematical bound

from:

sampled estimate

Sampled estimates require deterministic sampling context when replayability is requested.

---

155. Integration with "target/"

Target capabilities MUST be part of reproducibility when they affect realization.

A target capability change MUST NOT silently reuse an incompatible cached noise result.

---

156. Integration with "integration/ir.rs"

This module MUST map canonical IR identity into ZQN event identity without creating a competing IR identity.

Where canonical operation IDs already exist, ZQN SHOULD reference them.

---

157. Integration with "integration/routing.rs"

Routing output MUST be part of the physical execution identity when it changes physical resource assignment.

---

158. Integration with "integration/scheduling.rs"

Schedule/timing identity MUST be included whenever timing affects noise.

---

159. Integration with "integration/qec.rs"

QEC physical fault generation MUST use the same deterministic event identity framework.

Existing QEC seed-based APIs should be adapted through this integration boundary rather than creating an independent ZQN RNG.

---

160. Integration with "integration/hardware.rs"

Hardware execution MUST distinguish:

software experiment identity

from:

physical outcome

Hardware randomness MUST not be falsely represented as software deterministic randomness.

---

161. Integration with "integration/memory.rs"

Memory/state execution MUST preserve event identity when operations are materialized, copied, streamed, or transformed.

Memory layout MUST never affect stochastic semantics.

---

162. Integration with "integration/benchmarking.rs"

Benchmarking seeds and ZQN seeds MUST have a defined derivation hierarchy.

Existing benchmarking already has explicit seeded deterministic generators and prohibits time-derived or implicit global randomness.

---

163. Integration with "integration/runtime.rs"

Runtime owns:

execution lifecycle
parallel scheduling
cancellation
resource policy

ZQN owns:

semantic stochastic identity

Runtime MUST execute event identities without changing them.

---

164. Integration with "io/"

"io/serialization.rs" and "io/canonical.rs" MUST preserve all determinism-relevant information.

"io/compatibility.rs" MUST distinguish:

exact replay compatibility

from:

statistical compatibility

and:

unsupported compatibility

---

165. Integration with "tests/"

Testing MUST be organized into:

tests/determinism/
tests/property/
tests/scaling/
tests/compatibility/
tests/integration/

The determinism suite MUST cover both isolated ZQN behavior and cross-subsystem execution.

---

166. Required deterministic test matrix

At minimum:

Dimension| Test
Seed| same seed
Seed| different seed
Workers| 1 vs many
Batch| different batch sizes
Order| reordered events
Serialization| round trip
Replay| checkpoint/replay
Calibration| same snapshot
Calibration| changed snapshot
Target| same target
Target| changed target
Version| compatible version
Version| incompatible version
QEC| same physical fault realization
Benchmarking| same generated sequence
Characterization| same experiment realization
Scaling| small/large generated workloads

---

167. Deterministic golden fixtures

Fixtures MAY be used for stable deterministic sequences.

A golden fixture MUST record:

ZQN version
determinism algorithm version
root seed
model identity
configuration
expected result

Do not store only:

expected random number

without its generating context.

---

168. Golden fixture maintenance

If a deterministic algorithm intentionally changes:

old fixture

must remain available for compatibility testing when compatibility is promised.

Otherwise the fixture MUST be versioned rather than silently overwritten.

---

169. Differential testing

Equivalent implementations may be compared.

For example:

sequential
vs
parallel

and:

eager
vs
lazy

and:

CPU
vs
GPU

where applicable.

The comparison must use the declared equivalence contract.

---

170. Determinism invariant

The central invariant is:

For every deterministic event E,

Realize(E, D)

depends only on the deterministic specification D
and not on incidental execution state.

Incidental execution state includes:

thread
worker
allocation
wall-clock
process
batch
iteration order
logging
retry order

unless explicitly promoted into semantic input.

---

171. Semantic promotion rule

If a factor truly affects physical semantics, it MUST be promoted into the explicit deterministic specification.

For example:

physical node

may affect noise.

Then:

node identity

must be explicit.

The solution is NOT to let runtime state leak into the RNG.

---

172. No accidental nondeterminism

Any function that promises deterministic behavior MUST be reviewed for:

- unordered iteration;
- hidden mutable state;
- time;
- OS randomness;
- thread-local state;
- process state;
- floating-point reduction order;
- platform-dependent serialization;
- non-versioned algorithms;
- implicit calibration;
- implicit target selection.

---

173. API review rule

Every public stochastic API MUST answer:

1. Where does randomness come from?
2. Who owns it?
3. What is its identity?
4. What determines the stream?
5. Is replay possible?
6. Does parallelism change it?
7. Does batch size change it?
8. Does serialization preserve it?
9. What happens if a required identity is missing?
10. What compatibility guarantee is provided?

If any answer is undefined, the API is not production-ready.

---

174. No default hidden seed

A convenience constructor such as:

NoiseModel::new(...)

MUST NOT silently generate a seed and claim deterministic behavior.

Possible acceptable APIs are conceptually:

new_with_determinism(...)

or:

execute(context)

where the context explicitly specifies the policy.

---

175. Missing deterministic seed

If deterministic mode requires a root seed and none is supplied:

DeterminismFailure

or an equivalent explicit error MUST be returned.

Do not silently use:

time
OS entropy
random global state

---

176. Explicit statistical fallback

If the caller explicitly permits fallback:

deterministic preferred
statistical fallback allowed

the result MUST record that deterministic replay was not guaranteed.

---

177. Reproducibility report

Production executions SHOULD be able to produce a reproducibility report containing:

program identity
execution identity
ZQN version
schema version
determinism version
seed
model identity
calibration identity
target identity
numerical profile
approximation policy

This can be consumed by:

benchmarking
QEC
research workflows
CI
debugging
scientific publication tooling

---

178. Scientific publication support

A ZQN result intended for scientific reproducibility SHOULD be accompanied by sufficient information to reproduce the computation.

The reproducibility package should identify:

code/version
model
parameters
seed
calibration
target
algorithm
numerical assumptions

---

179. CI requirements

Continuous integration SHOULD run deterministic tests multiple times.

For example:

same test
same seed
multiple process runs

must produce identical deterministic output.

Where supported, CI SHOULD also test different thread counts.

---

180. Regression testing

A regression test MUST distinguish:

algorithm changed intentionally

from:

algorithm changed accidentally

A changed deterministic result is not automatically a failure, but it MUST be reviewed whenever exact replay compatibility is promised.

---

181. Production acceptance criteria

ZQN determinism is production-ready only when:

- no hidden global RNG exists;
- no time-derived deterministic randomness exists;
- no memory-address-derived randomness exists;
- canonical qubit identity is used;
- event-addressed randomness is implemented;
- parallel execution is deterministic;
- batch size does not alter deterministic samples;
- distributed partitioning does not alter deterministic samples;
- serialization preserves determinism metadata;
- replay is supported;
- versioning is explicit;
- calibration identity is preserved;
- target identity is preserved;
- numerical guarantees are declared;
- resource exhaustion cannot silently alter semantics;
- "unsafe" is forbidden;
- tests cover the above invariants.

---

182. Required source-file integration map

The determinism contract MUST be implemented across the following files without duplicating ownership.

core/
├── error.rs
│   └── DeterminismFailure / related errors
│
├── ids.rs
│   └── deterministic ZQN identities
│
├── version.rs
│   └── determinism algorithm version
│
├── context.rs
│   └── deterministic execution context
│
├── provenance.rs
│   └── replay/provenance metadata
│
└── limits.rs
    └── resource limits

probability/
└── sampling-related files
    └── explicit sampling context only

noise/
└── model.rs
    └── explicit deterministic context

fault/
└── fault.rs / batch.rs
    └── stable event identities

calibration/
└── snapshot.rs
    └── immutable calibration identity

characterization/
└── experiment.rs
    └── experiment reproducibility policy

simulation/
└── reproducibility.rs
    └── execution/replay contract

target/
└── compatibility.rs
    └── deterministic compatibility

integration/
├── ir.rs
├── qec.rs
├── routing.rs
├── scheduling.rs
├── hardware.rs
├── benchmarking.rs
└── runtime.rs
    └── deterministic integration boundaries

io/
├── canonical.rs
├── serialization.rs
├── deserialization.rs
└── compatibility.rs
    └── reproducibility persistence

tests/
└── determinism/
    └── end-to-end deterministic verification

---

183. File completion rule

A source file implementing deterministic behavior is complete only when:

1. its stochastic inputs are explicit;
2. its event identity is defined;
3. its parent identity is defined;
4. its RNG ownership is defined;
5. its parallel behavior is defined;
6. its serialization behavior is defined;
7. its version compatibility is defined;
8. its resource behavior is defined;
9. its numerical guarantee is defined;
10. its tests are defined;
11. its integration consumers are defined;
12. it does not require future modification merely because an unrelated ZQN subsystem is implemented.

---

184. Ownership summary

core/context.rs
    owns execution context

core/ids.rs
    owns ZQN identities

core/version.rs
    owns version identity

core/provenance.rs
    owns reproducibility provenance

probability/
    owns probability mathematics

simulation/reproducibility.rs
    owns simulation replay mechanics

noise/
    owns noise semantics

fault/
    owns fault semantics

calibration/
    owns calibration identity/state

integration/*
    owns cross-subsystem adapters

io/
    owns persistence/canonicalization

No two modules should independently define competing deterministic semantics.

---

185. Existing repository compatibility

The current Zamani repository already contains deterministic seed mechanisms in:

quantum/error_correction/noise.rs
quantum/error_correction/replay.rs
quantum/benchmarking/generators/*
quantum/benchmarking/protocols/*
quantum/optimization/stochastic/sampling.rs
quantum/zqn/characterization/*

Examples include explicit seeds in QEC and stochastic optimization, and seeded "StdRng" usage in randomized benchmarking.

These implementations should be converged toward the common ZQN determinism contract rather than creating incompatible parallel mechanisms.

---

186. Migration strategy

Migration MUST be incremental.

Stage 1

Define the common ZQN determinism vocabulary.

Stage 2

Adapt ZQN probability sampling.

Stage 3

Adapt ZQN noise/fault generation.

Stage 4

Adapt characterization.

Stage 5

Adapt simulation.

Stage 6

Adapt QEC through "integration/qec.rs".

Stage 7

Adapt benchmarking.

Stage 8

Adapt routing/scheduling.

Stage 9

Add distributed/replay tests.

Stage 10

Remove duplicate RNG semantics from downstream systems once compatibility adapters are proven.

---

187. Do not perform a dangerous migration

Do NOT immediately delete existing QEC or benchmarking seed APIs.

Instead:

existing API
     │
     ▼
compatibility adapter
     │
     ▼
ZQN determinism context

Then migrate consumers.

Only remove old APIs after:

all consumers migrated
tests passing
compatibility policy established

---

188. Final architecture

The final deterministic architecture is:

                    Zamani Source
                         │
                         ▼
                    Quantum IR
                         │
                         ▼
                 Execution Identity
                         │
          ┌──────────────┴──────────────┐
          │                             │
     Root Seed                    Calibration
          │                             │
          └──────────────┬──────────────┘
                         ▼
                  Determinism Context
                         │
          ┌──────────────┼──────────────┐
          ▼              ▼              ▼
       Shot 0          Shot 1         Shot N
          │              │              │
          ▼              ▼              ▼
       Events          Events         Events
          │              │              │
          └──────────────┼──────────────┘
                         ▼
                  ZQN Noise/Faults
                         │
          ┌──────────────┼──────────────┐
          ▼              ▼              ▼
        Routing      Scheduling        QEC
          │              │              │
          └──────────────┼──────────────┘
                         ▼
                  Hardware/Simulator
                         │
                         ▼
                    Observations
                         │
                         ▼
                Reproducibility Record

---

189. The core invariant

The entire subsystem can ultimately be summarized by:

deterministic_result =
    F(
        canonical_program,
        execution_identity,
        noise_model,
        noise_configuration,
        calibration_snapshot,
        target_semantics,
        determinism_algorithm_version,
        root_seed,
        numerical_profile
    )

and MUST NOT be:

F(
    ...,
    current_thread,
    current_process,
    current_time,
    memory_address,
    allocation_order,
    hash_map_iteration_order,
    hidden_global_rng
)

unless one of those values has explicitly been promoted into the semantic execution specification.

---

190. Final production rule

The definitive ZQN rule is:

«Randomness may be stochastic, but its identity must be deterministic whenever deterministic execution is requested.»

Therefore:

same semantic computation
+
same deterministic specification
=
same stochastic realization

regardless of whether the realization is produced:

sequentially
in parallel
lazily
in batches
on another worker
on another machine
through a checkpoint
through replay
at a different memory layout

subject only to the explicitly declared numerical, version, target, and resource compatibility contracts.

This is the determinism foundation required for ZQN to scale from the smallest quantum computation to arbitrarily large systems constrained only by actual available resources, while preserving Zamani's write-once, scale-everywhere architecture.

---

191. Definition of Done for "DETERMINISM.md"

This document is complete when the following architectural statements are true:

[✓] No hidden global RNG
[✓] No time-derived deterministic randomness
[✓] No memory-address-derived randomness
[✓] No thread-derived deterministic randomness
[✓] No process-derived deterministic randomness
[✓] No unordered semantic iteration
[✓] Explicit root seed
[✓] Explicit determinism policy
[✓] Event-addressed randomness
[✓] Stable shot identity
[✓] Stable operation identity
[✓] Canonical quantum resource identity
[✓] quantum::ir::qubit remains authoritative
[✓] Parallel determinism
[✓] Batch determinism
[✓] Streaming determinism
[✓] Distributed determinism
[✓] Checkpoint/replay semantics
[✓] Calibration identity
[✓] Target identity
[✓] Model identity
[✓] Algorithm versioning
[✓] Canonical serialization
[✓] Numerical determinism contract
[✓] Resource-limit separation
[✓] QEC integration
[✓] Benchmarking integration
[✓] Characterization integration
[✓] Routing integration
[✓] Scheduling integration
[✓] Hardware integration
[✓] Runtime integration
[✓] Memory integration
[✓] No unsafe Rust
[✓] No artificial machine-size ceiling
[✓] Tiny-to-large scalability
[✓] Explicit approximation semantics
[✓] Explicit statistical-vs-exact distinction
[✓] Property tests
[✓] Differential tests
[✓] Replay tests
[✓] Scaling tests

When these invariants are implemented, "DETERMINISM.md" becomes the normative contract against which every ZQN stochastic implementation can be reviewed.

It must be treated as an architectural contract, not as optional documentation.This is deliberately stricter than simply documenting StdRng/seeds: it makes event identity, parallel execution, calibration, target realization, versioning, replay, numerical behavior, and cross-subsystem integration part of determinism. That is necessary if ZQN is to remain valid when Zamani grows from small simulations to very large and distributed quantum systems.