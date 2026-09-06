Worked for 21s

I checked the repository directly before writing this rather than treating DETERMINISM.md as an isolated document. The repository currently has no src/quantum/resilience/DETERMINISM.md, while its existing resilience documentation establishes the surrounding architecture. The canonical IR explicitly places QubitId and PhysicalQubitId under quantum::ir::qubit, and the scheduling/ZQN layers follow that same identity boundary.

Below is the complete production-ready file. It is intentionally normative and is designed to integrate with DESIGN.md, SCALABILITY.md, SECURITY.md, the canonical IR, ZQN, hardware, routing, scheduling, optimization, QEC, checkpointing, telemetry, planning, recovery, and verification without introducing a second identity or determinism model.

Zamani Quantum Resilience — Determinism Specification

Path: "src/quantum/resilience/DETERMINISM.md"
Status: Normative production specification
Subsystem: "quantum::resilience"
Language: Rust
Edition: Rust 2021
Required toolchain: Rust 1.97 / Rust 1.97.1
Safety: "unsafe" forbidden
Canonical quantum identity: "crate::quantum::ir::qubit"
Primary objective: Reproducible, auditable, replayable, resource-aware resilience decisions without imposing artificial machine-size limits.

---

1. Purpose

This document defines the deterministic-execution and deterministic-decision contract for "quantum::resilience".

The purpose is not to make every quantum execution physically identical.

That is neither possible nor desirable on real quantum hardware.

The purpose is to make every deterministic decision made by the resilience subsystem reproducible from an explicitly defined input state.

The central rule is:

«Given the same canonical program identity, canonical IR identity, execution context, relevant resource/capability snapshot, observation set, policy, history snapshot, strategy versions, implementation version, and explicitly controlled randomness, the resilience subsystem MUST produce the same deterministic decision and the same deterministic decision provenance.»

This includes decisions involving:

- fault classification;
- incident aggregation;
- diagnosis;
- policy evaluation;
- recovery-plan generation;
- action ranking;
- adaptation scope;
- retry eligibility;
- migration eligibility;
- mitigation selection;
- checkpoint selection;
- verification policy;
- escalation;
- acceptance/rejection decisions.

Determinism is therefore a property of the decision system, not a claim that quantum hardware itself behaves deterministically.

---

2. Scope

This specification applies to all production code under:

src/quantum/resilience/

including:

api/
model/
detection/
diagnosis/
policy/
planning/
adaptation/
recovery/
mitigation/
verification/
state/
checkpoint/
telemetry/
history/
learning/
coordination/
serialization/
errors/
limits/
registry/

and their submodules.

It also defines the determinism contract used when resilience integrates with:

crate::quantum::ir
crate::quantum::ir::qubit
crate::quantum::zqn
crate::quantum::hardware
crate::quantum::routing
crate::quantum::scheduling
crate::quantum::optimization
crate::quantum::error_correction
crate::quantum::simulation

and the execution/runtime layer.

---

3. Determinism is not physical reproducibility

The following distinction is mandatory.

3.1 Decision determinism

Decision determinism means:

same defined inputs
        ↓
same deterministic decision

For example:

same fault evidence
same hardware snapshot
same policy
same planner version
same history
same seed
        ↓
same recovery plan

3.2 Execution reproducibility

Execution reproducibility means that an execution environment can reproduce an equivalent execution under sufficiently controlled conditions.

This may not be possible on real hardware because:

- quantum noise changes;
- calibration changes;
- device state changes;
- queue conditions change;
- hardware availability changes;
- environmental conditions change;
- provider execution may contain nondeterministic behavior.

Resilience MUST NOT falsely claim that physical execution is deterministic merely because its planner is deterministic.

3.3 Semantic reproducibility

Semantic reproducibility means that different physical realizations preserve the required canonical program semantics.

For example:

logical program
    ↓
device A realization

logical program
    ↓
device B realization

may produce different physical circuits while preserving the same accepted semantic contract.

This is the level required for "write once, run anywhere."

---

4. Normative determinism levels

The resilience subsystem MUST distinguish at least the following modes.

4.1 Strict deterministic mode

Strict deterministic mode requires all deterministic inputs to be explicitly bound.

A deterministic decision MUST NOT depend on:

- wall-clock time;
- process ID;
- thread scheduling;
- memory addresses;
- hash-map iteration order;
- unspecified filesystem ordering;
- network response ordering;
- backend response ordering;
- uncontrolled randomness;
- ambient global state;
- environment variables unless explicitly bound into the execution context;
- locale;
- host-specific behavior;
- floating-point reduction order where it can alter the decision.

If any required deterministic input is unavailable, strict deterministic mode MUST fail closed with a deterministic error.

It MUST NOT silently downgrade to best-effort behavior.

---

4.2 Reproducible mode

Reproducible mode requires deterministic decision inputs to be recorded and replayable, but permits explicitly declared environmental differences.

For example:

same program
same policy
same strategy versions
different physical hardware

may produce:

different valid plan

because the hardware capability snapshot is intentionally different.

The decision remains reproducible for that specific snapshot.

---

4.3 Best-effort deterministic mode

Best-effort mode permits selected nondeterministic inputs where the policy explicitly allows them.

Every such input MUST be recorded.

The system MUST label the resulting decision:

determinism = best_effort

It MUST NOT label the decision fully deterministic.

---

4.4 Non-deterministic mode

Non-deterministic strategies may be used where required by:

- stochastic optimization;
- randomized compilation;
- randomized mitigation;
- randomized sampling;
- exploration;
- adaptive learning;
- hardware-native randomized behavior.

Such randomness MUST remain explicit and auditable.

It MUST NOT leak invisibly into the deterministic planner.

---

5. Determinism hierarchy

The hierarchy is:

Canonical program
        ↓
Canonical IR
        ↓
Execution context
        ↓
Capability/resource snapshot
        ↓
Observation snapshot
        ↓
Policy
        ↓
Strategy versions
        ↓
History/state snapshot
        ↓
Explicit randomness
        ↓
Planner
        ↓
Deterministic decision
        ↓
Verification

Every level MUST have a defined identity.

---

6. Deterministic decision function

Conceptually, deterministic planning MUST behave as:

Decision =
    F(
        ProgramIdentity,
        IRIdentity,
        ExecutionContext,
        CapabilitySnapshot,
        ResourceSnapshot,
        ObservationSnapshot,
        PolicyIdentity,
        PolicyState,
        HistorySnapshot,
        StrategyIdentity,
        RegistryIdentity,
        ImplementationIdentity,
        RandomnessState
    )

The function MUST have no hidden inputs.

Therefore:

F(inputs) = decision

must be reproducible when all inputs are identical.

---

7. Deterministic input closure

A decision is deterministic only if its input set is closed.

An input is considered closed when its value is either:

1. explicitly supplied;
2. deterministically derived from another closed input;
3. cryptographically/content-addressably identified;
4. explicitly declared irrelevant.

A planner MUST NOT access undeclared ambient state.

For example, this is prohibited:

plan()
    -> reads system time
    -> reads global configuration
    -> reads environment variable
    -> reads unordered registry

without those values being part of the deterministic context.

---

8. Canonical identity

All quantum-resource identity used by resilience MUST follow the repository's canonical identity model.

The authoritative types are:

crate::quantum::ir::qubit::QubitId
crate::quantum::ir::qubit::PhysicalQubitId

where applicable.

Resilience MUST NOT create:

ResilienceQubitId
RecoveryQubitId
FaultQubitId
DetectorQubitId
PlannerQubitId

as competing canonical quantum identities.

A resilience-specific resource identifier may exist only when it represents a genuinely different resource domain.

For example:

IncidentId
RecoveryId
CheckpointId
ExecutionId

are not qubit identities.

---

9. Logical and physical identity

The following MUST NOT be assumed:

QubitId == PhysicalQubitId

or:

logical index == physical index

or:

QubitId(0) == PhysicalQubitId(0)

The correct relationship is:

canonical logical identity
        ↓
quantum::routing
        ↓
canonical physical identity
        ↓
quantum::hardware

Resilience may request remapping.

Resilience MUST NOT invent physical mappings.

---

10. Deterministic identifiers

Identifiers used in deterministic decisions MUST NOT depend on nondeterministic UUID generation.

For deterministic artifacts, prefer content-derived identifiers.

Conceptually:

ArtifactId =
    H(
        schema_version ||
        canonical_serialization ||
        domain
    )

The exact hashing implementation belongs to the repository's serialization/integrity contract.

Random identifiers may be used for operational correlation when explicitly classified as non-deterministic metadata, but they MUST NOT influence deterministic decisions.

---

11. Content identity versus operational identity

The system MUST distinguish:

Content identity

Identifies the content itself.

Examples:

ProgramHash
IRHash
PolicyHash
CapabilitySnapshotHash
ObservationSnapshotHash
PlanHash
CheckpointHash

Operational identity

Identifies an individual execution or event.

Examples:

ExecutionId
IncidentId
RecoveryId
AttemptId
TraceId

Operational identifiers MUST NOT accidentally become planning inputs.

For example:

ExecutionId = random UUID

must not change the selected recovery strategy.

---

12. Canonical serialization

Anything used as a deterministic input MUST have a canonical serialization.

Canonical serialization MUST define:

- field order;
- collection ordering;
- numeric representation;
- optional-field representation;
- enum representation;
- version representation;
- string normalization rules where applicable;
- absence versus empty semantics;
- nested-object ordering;
- duplicate handling.

Equivalent logical objects MUST serialize identically.

---

13. Collection ordering

Unordered collections MUST NOT directly influence deterministic decisions.

For example, this is unsafe:

HashMap<K, V>

followed by iteration whose order becomes planner order.

Instead:

collect
→ canonical ordering
→ deterministic processing

must be used.

Ordering MUST be based on stable semantic keys.

Possible ordering keys include:

canonical resource identity
operation identity
incident identity
timestamp plus stable event identity
content hash
canonical serialized representation

The choice must be explicitly documented for each collection.

---

14. HashMap and HashSet restriction

"HashMap" and "HashSet" MAY be used for efficient lookup.

They MUST NOT be used as an implicit ordering mechanism.

Forbidden:

for item in map.values() {
    plan(item);
}

when the order affects output.

Required:

lookup structure
+
explicit deterministic ordering

before any order-sensitive computation.

---

15. Sorting requirements

Every deterministic sort MUST use a total ordering.

A comparator that can return equality for distinct objects when their relative order influences the result is insufficient.

A deterministic tie-breaker MUST exist.

For example:

primary score
→ resource identity
→ operation identity
→ strategy identity
→ canonical serialized bytes

The exact keys depend on the object being ranked.

---

16. Stable tie-breaking

No planner may rely on incidental iteration order to resolve ties.

If two candidates have equal cost:

candidate A = cost 10
candidate B = cost 10

the planner MUST apply a documented deterministic tie-breaker.

It MUST NOT choose whichever candidate happened to be encountered first.

---

17. Floating-point determinism

Floating-point arithmetic requires special care.

Deterministic mode MUST avoid relying on unspecified floating-point reduction order.

For example:

(a + b) + c

may not be treated as interchangeable with:

a + (b + c)

when exact reproducibility matters.

Parallel reductions MUST either:

1. use a deterministic reduction tree; or
2. use an exact/fixed representation appropriate to the metric; or
3. be excluded from strict deterministic decisions.

The decision contract MUST specify which approach is used.

---

18. NaN and infinity handling

Deterministic comparisons MUST define behavior for:

NaN
+∞
-∞
missing
unknown
invalid

A comparison MUST NOT depend on platform-specific or library-specific ordering behavior.

The system MUST define whether such values:

reject
degrade confidence
trigger escalation
or participate in an explicitly defined ordering

---

19. Numeric precision

A production deterministic metric MUST define:

- precision;
- rounding;
- overflow behavior;
- underflow behavior;
- invalid-input behavior.

The planner MUST NOT silently change precision based on the host machine.

---

20. Randomness

Randomness is allowed only when explicitly represented.

Every random decision MUST have:

algorithm identity
seed
stream identity
draw position/state
purpose
version

where the underlying strategy requires those values.

A single global random generator MUST NOT be used by unrelated resilience components.

---

21. Randomness domains

Randomness MUST be domain-separated.

For example:

planner
mitigation
sampling
learning
fault injection
simulation

must not share an implicit random stream.

Conceptually:

MasterSeed
    ↓
domain derivation
    ├── planner stream
    ├── mitigation stream
    ├── sampling stream
    └── learning stream

One subsystem consuming additional random numbers MUST NOT silently change another subsystem's random sequence.

---

22. Seed derivation

If derived seeds are used, they MUST be deterministically derived from explicit parent inputs.

Conceptually:

child_seed =
    H(
        master_seed ||
        domain ||
        execution_identity ||
        strategy_identity
    )

The exact cryptographic primitive belongs to the repository's cryptographic/integrity layer.

---

23. Randomness and semantic behavior

Randomized physical realization MUST NOT silently alter semantic identity.

For example:

randomized routing
randomized twirling
randomized compilation

may change physical execution while the canonical logical program remains unchanged.

The randomization parameters MUST be recorded in provenance.

---

24. Concurrency

Concurrency MUST NOT change deterministic decisions.

This is one of the most important rules in this document.

Forbidden architecture:

worker completion order
        ↓
planner candidate order
        ↓
different decision

Required architecture:

parallel collection
        ↓
canonical normalization
        ↓
deterministic ordering
        ↓
deterministic reduction
        ↓
decision

---

25. Parallel execution

Parallelism is permitted.

However:

«Parallelism MUST be observationally equivalent to deterministic sequential evaluation for strict deterministic decisions.»

This means the following MUST produce the same result:

single-threaded planner

and:

multi-threaded planner

when supplied with identical deterministic inputs.

---

26. Parallel reduction

Every parallel reduction used by deterministic logic MUST define:

- partitioning;
- ordering;
- reduction tree;
- identity value;
- error behavior.

A scheduler MUST NOT be allowed to choose the reduction tree arbitrarily when numerical differences can alter the result.

---

27. Async execution

Asynchronous execution MUST NOT make completion order semantically significant.

For example:

detector A finishes first
detector B finishes second

must not automatically mean:

A gets priority over B

unless the ordering is explicitly defined.

Use:

event identity
logical timestamp
sequence number
canonical ordering key

instead.

---

28. Wall-clock time

Wall-clock time MUST NOT affect strict deterministic decisions.

For deterministic execution, time MUST be represented as explicit input.

Examples:

observation timestamp
snapshot timestamp
policy validity interval
deadline

are valid when they are bound into the deterministic context.

Calling:

SystemTime::now()

inside a deterministic decision path is prohibited.

---

29. Monotonic time

Monotonic clocks may be used for operational timeout enforcement.

However, measured elapsed time MUST NOT affect a strict deterministic plan unless the measured value is explicitly part of the deterministic input.

The distinction is:

operational timeout

versus:

deterministic decision input

They must not be conflated.

---

30. Environment variables

Environment variables MUST NOT silently influence deterministic planning.

If an environment variable is intentionally relevant, its resolved value MUST be captured in the execution context and provenance.

Otherwise it is an undeclared input and violates strict determinism.

---

31. Filesystem ordering

Filesystem traversal order is not a deterministic contract.

Any filesystem-derived input MUST be:

discover
→ normalize
→ canonicalize
→ sort
→ consume

Directory entry order MUST never directly determine:

- detector order;
- strategy order;
- registry order;
- recovery order;
- verification order.

---

32. Network determinism

Network responses are external observations.

They are deterministic inputs only after they have been captured into an explicit observation snapshot.

The planner MUST NOT depend on:

which network response arrives first

unless arrival order itself is part of the declared event model.

---

33. Backend response ordering

Hardware providers may return:

- measurements;
- jobs;
- telemetry;
- calibration records;
- errors

in arbitrary order.

Resilience MUST normalize them before deterministic processing.

The normalization contract MUST preserve:

- provider identity;
- event identity;
- timestamp;
- sequence information when authoritative;
- payload;
- provenance.

---

34. Telemetry determinism

Telemetry is an observation stream.

The deterministic processing model is:

raw telemetry
      ↓
authenticate/validate
      ↓
normalize
      ↓
deduplicate where defined
      ↓
canonical ordering
      ↓
aggregate
      ↓
detect

The detector MUST NOT depend on incidental arrival order unless the event model explicitly defines temporal order as semantically relevant.

---

35. Event ordering

Every telemetry event used for deterministic reasoning SHOULD have a stable ordering tuple.

Conceptually:

(
    source_identity,
    authoritative_sequence,
    event_time,
    event_identity
)

The actual fields depend on the telemetry contract.

If two events remain indistinguishable after all authoritative fields, their payloads MUST be canonically compared.

---

36. Duplicate events

Duplicate handling MUST be explicit.

The system must distinguish:

same event delivered twice

from:

two distinct events with equivalent payloads

Deduplication MUST be based on stable event identity or an explicitly defined canonical event identity.

It MUST NOT simply discard equal-looking events.

---

37. Fault ordering

Resilience consumes canonical fault semantics from ZQN.

Fault ordering MUST NOT create a second fault ontology.

The processing pipeline is:

ZQN fault
    ↓
resilience observation
    ↓
canonical ordering/aggregation
    ↓
incident

Fault identity and location MUST remain traceable to their source.

Where quantum identity is needed, use:

crate::quantum::ir::qubit::QubitId
crate::quantum::ir::qubit::PhysicalQubitId

as appropriate.

---

38. Incident aggregation determinism

Incident aggregation MUST be deterministic.

Given identical:

fault set
aggregation policy
correlation rules
time model
resource model

the same incident partition MUST be produced.

The aggregation algorithm MUST NOT depend on:

- input arrival order;
- hash iteration order;
- worker completion order;
- thread count.

---

39. Diagnosis determinism

Diagnosis MUST be deterministic when its evidence and model inputs are deterministic.

If multiple root-cause hypotheses have equal confidence, deterministic tie-breaking is required.

The output MUST preserve uncertainty.

Determinism does not mean pretending an uncertain diagnosis is certain.

---

40. Policy determinism

"policy/*" MUST be deterministic for a fixed policy snapshot.

A policy MUST have:

policy identity
policy version
configuration identity
validity state
constraints
objectives
budgets
safety rules

Policy evaluation MUST NOT silently consult mutable global configuration.

---

41. Policy changes

A policy change MUST NOT retroactively alter an already recorded deterministic decision.

Every decision MUST bind to the policy snapshot used.

Conceptually:

Decision
    |
    +-- policy_hash
    +-- policy_version
    +-- policy_snapshot

A later policy update creates a new decision context.

---

42. Planning determinism

"planning/planner.rs" MUST be a pure decision layer with respect to its declared inputs.

It MUST NOT:

- inspect global mutable state;
- read the clock;
- query the network directly;
- mutate registry order;
- mutate policy;
- generate hidden random values.

External information MUST be supplied through explicit context objects.

---

43. Candidate generation

Candidate recovery actions MUST be generated deterministically.

For example:

Retry
Restart
Remap
Reroute
Reschedule
Recompile
Reoptimize
Mitigate
Migrate
Quarantine
Abort

must be ordered by an explicit strategy registry ordering or canonical identity.

Plugin registration order MUST NOT define planner behavior.

---

44. Strategy registry determinism

"registry/*" MUST provide deterministic lookup.

A registry MUST NOT rely on:

dynamic loading order
filesystem order
thread race
hash-map iteration

to define strategy precedence.

Each strategy MUST have a stable identity and version.

---

45. Strategy versioning

Every deterministic strategy MUST expose stable identity information sufficient to reproduce its behavior.

At minimum:

strategy_id
strategy_version
implementation_version
configuration_identity

If a strategy is learned/adaptive, the model identity MUST also be recorded.

---

46. Plugin determinism

Plugins MUST NOT be allowed to inject hidden nondeterminism into strict deterministic mode.

A plugin participating in deterministic planning MUST declare:

- deterministic behavior;
- required inputs;
- randomness requirements;
- version;
- serialization behavior;
- ordering behavior;
- resource dependencies.

An undeclared nondeterministic plugin MUST be rejected from strict deterministic execution.

---

47. Registry mutation

Registries MUST be immutable for the lifetime of a deterministic decision.

The preferred model is:

mutable registry
      ↓
freeze
      ↓
registry snapshot
      ↓
deterministic planning

A registry update creates a new snapshot.

---

48. Capability snapshot determinism

Hardware capabilities are dynamic.

Therefore strict determinism MUST use a capability snapshot rather than continuously querying mutable hardware state during planning.

Conceptually:

hardware
   ↓
capability snapshot
   ↓
snapshot identity/hash
   ↓
planner

The snapshot MUST contain enough information to reproduce the relevant decision.

---

49. Resource snapshot determinism

The same principle applies to:

- available qubits;
- physical qubits;
- logical resources;
- control channels;
- execution slots;
- backend availability;
- topology;
- QEC resources;
- scheduling resources.

A planner MUST use an explicit resource snapshot.

---

50. Resource identity

Resource identity must not be inferred from position in a collection.

Where quantum qubit identity is required:

crate::quantum::ir::qubit::QubitId
crate::quantum::ir::qubit::PhysicalQubitId

must be used.

Resource collections may be reordered without changing resource identity.

---

51. Hardware changes during planning

Hardware state may change between snapshot and execution.

This is not a violation of planner determinism.

Instead:

snapshot A
    ↓
plan A
    ↓
hardware changes
    ↓
plan validation
    ↓
invalid
    ↓
new snapshot B
    ↓
plan B

The second plan is a new deterministic decision over a new input state.

---

52. Stale-plan prevention

Every executable recovery/adaptation plan MUST bind to the state snapshot against which it was generated.

Before execution:

plan.snapshot_identity
        ==
current.validated_snapshot_identity

must hold where the execution contract requires exact snapshot matching.

If not:

REVALIDATE

or:

REPLAN

must occur.

A stale plan MUST NOT execute merely because it was once valid.

---

53. Routing determinism

Resilience does not own routing.

However, when resilience requests routing, the routing invocation MUST contain deterministic inputs sufficient to reproduce the request.

At minimum, where relevant:

canonical IR identity
logical resource identities
physical capability snapshot
topology snapshot
routing policy
routing strategy identity
seed

The routing subsystem remains responsible for its own deterministic contract.

---

54. Scheduling determinism

Resilience does not own scheduling.

When requesting rescheduling, it MUST provide a stable execution context.

Scheduling MUST own:

- operation ordering;
- timing;
- resource conflicts;
- calibrated durations;
- schedule construction.

Resilience records the scheduling strategy/version used.

---

55. Optimization determinism

Resilience does not own optimization.

When requesting reoptimization, it MUST identify:

input IR
target capabilities
optimization profile
fault-tolerance requirements
optimization strategy/version
randomness state if applicable

The resulting IR MUST receive a new content identity.

---

56. Canonical IR identity

The deterministic chain MUST distinguish:

original IR
    ↓
adapted IR
    ↓
scheduled realization
    ↓
physical execution

The original semantic identity MUST remain recoverable.

A transformed IR MUST NOT overwrite the identity of the original program.

---

57. QEC determinism

Resilience does not implement QEC.

Where QEC participates in deterministic resilience decisions, the decision context MUST record:

QEC implementation
code identity
configuration
decoder identity
decoder version
logical resource identity
relevant syndrome snapshot
decoder confidence
policy
randomness if applicable

A QEC result with uncertainty MUST preserve that uncertainty.

---

58. Mitigation determinism

Mitigation strategies may be randomized.

Examples include:

- twirling;
- randomized compiling;
- probabilistic methods;
- sampling-based methods.

Their randomness MUST be explicitly controlled.

A deterministic mitigation decision means:

same mitigation configuration
same seed
same input state
same strategy version
        ↓
same mitigation plan

It does not necessarily mean the physical random samples are generated identically unless the execution contract explicitly requires that.

---

59. Verification determinism

Verification MUST be deterministic for identical evidence.

The acceptance decision:

ACCEPT
DEGRADED_ACCEPT
RETRY
RECOVER
ESCALATE
REJECT

MUST be determined by:

result
program identity
IR identity
execution context
verification policy
provenance
confidence
resource state

and no hidden inputs.

---

60. Verification is the determinism boundary

A deterministic recovery decision is not enough.

The final system must establish:

deterministic decision
        +
verified execution
        =
accepted outcome

The verifier remains authoritative for acceptance.

---

61. Provenance

Every deterministic decision MUST be reconstructible from provenance.

At minimum, provenance SHOULD bind:

program identity
original IR identity
adapted IR identity
execution identity
hardware identity
capability snapshot identity
resource snapshot identity
observation snapshot identity
fault/incident identity
policy identity
planner identity
strategy identities
registry snapshot identity
QEC identity
optimization identity
routing identity
scheduling identity
checkpoint identity where applicable
randomness identity where applicable
decision identity
verification identity

---

62. Provenance immutability

Once a deterministic decision is finalized, its provenance MUST be immutable.

Corrections MUST be represented as new records.

Do not mutate historical provenance in place.

---

63. Deterministic replay

The system MUST support deterministic replay for supported decision classes.

Replay should follow:

recorded provenance
        ↓
restore snapshots
        ↓
restore policy
        ↓
restore strategies
        ↓
restore registry
        ↓
restore randomness
        ↓
recompute
        ↓
compare

Expected outcome:

same decision identity

or a precisely classified incompatibility.

---

64. Replay is not re-execution

Replay of a resilience decision does not imply replaying the quantum hardware.

For example:

planner replay

can be deterministic while:

QPU execution replay

is not physically reproducible.

The documentation and APIs MUST keep these concepts separate.

---

65. Deterministic replay failure

Replay MUST NOT silently produce a different result and call it successful.

If replay cannot reproduce the decision because:

- schema changed;
- strategy changed;
- implementation changed;
- snapshot missing;
- plugin missing;
- capability information missing;
- random state missing;

the result MUST be classified explicitly.

Examples:

REPLAY_INCOMPLETE
REPLAY_INCOMPATIBLE
REPLAY_NON_DETERMINISTIC
REPLAY_INPUT_MISSING

These belong in the resilience error classification.

---

66. Version binding

Strict deterministic decisions MUST bind to relevant implementation versions.

At minimum, where behavior can affect the decision:

resilience version
strategy version
schema version
policy version
model version
plugin version

A Rust compiler version alone is insufficient to identify algorithmic behavior.

---

67. Rust toolchain

The implementation target is:

Rust 1.97
Rust 1.97.1
Rust 2021 edition

The resilience subsystem MUST NOT require nightly-only language features.

Production code MUST compile without "unsafe".

---

68. Unsafe prohibition

Every Rust source file in the subsystem MUST maintain the repository's safe-Rust requirement.

The resilience subsystem MUST NOT use:

unsafe

including:

- raw pointer operations;
- unsafe FFI;
- unsafe synchronization;
- unsafe memory access;
- unsafe deserialization;
- unsafe plugin loading.

Determinism MUST be achieved through explicit state and safe abstractions.

---

69. Hidden global state

Strict deterministic paths MUST NOT depend on mutable global state.

Forbidden examples include:

global mutable planner configuration
global random generator
global strategy registry
global current backend
global current time
global mutable cache whose contents affect ordering

Caches are allowed only if their presence/absence cannot change the result.

---

70. Cache determinism

A cache MUST be semantically transparent.

This MUST hold:

cache hit

and:

cache miss

produce the same logical result.

A cache MUST NOT change:

- candidate ordering;
- floating-point reduction order;
- strategy selection;
- retry count;
- acceptance;
- error classification.

---

71. Memoization

Memoization keys MUST include every input capable of changing the result.

A cache key that omits:

policy version
strategy version
capability snapshot
resource snapshot

when those affect the result is invalid.

---

72. History determinism

Historical data is an input.

Therefore "history/*" MUST expose snapshot semantics.

A deterministic planner MUST reason over:

HistorySnapshot

rather than a concurrently mutating history database.

The snapshot MUST have stable identity.

---

73. Learning and determinism

Learning introduces special requirements.

A learned predictor MAY influence planning.

However:

«Learning MUST NOT silently make strict deterministic decisions dependent on mutable external model state.»

The model identity MUST be bound.

For example:

model_id
model_version
model_hash
feature_schema
training-state identity

must be recorded where relevant.

---

74. Online learning

Online learning is inherently stateful.

Therefore online learning MUST be separated from strict deterministic replay.

A strict deterministic decision MUST use a frozen model snapshot.

Learning may later create:

new model snapshot

which creates a new decision environment.

---

75. Exploration versus production determinism

An exploration strategy may deliberately use randomness.

The production deterministic planner must either:

1. freeze the exploration seed and strategy state; or
2. classify the decision as non-deterministic.

It MUST NOT mislabel exploration as deterministic.

---

76. Distributed determinism

Distributed resilience introduces additional sources of nondeterminism:

- message ordering;
- node scheduling;
- network latency;
- leader changes;
- retries;
- duplicate messages;
- clock differences;
- partial failure.

Therefore distributed decisions require explicit logical ordering.

---

77. Logical clocks

Wall-clock time MUST NOT be used as the sole ordering mechanism for distributed deterministic decisions.

Where distributed ordering matters, use the coordination subsystem's authoritative mechanism, such as:

sequence number
logical clock
consensus order
causal ordering

as appropriate.

---

78. Distributed decision identity

A distributed resilience decision MUST identify:

coordinator
participants
input snapshot
coordination state
decision sequence
policy
strategy

The coordinator must not make decisions based solely on whichever worker responded first.

---

79. Duplicate distributed actions

Recovery actions may be retried because of network failures.

Actions MUST therefore support idempotency where possible.

An action identity should be stable for the intended operation.

For example:

RecoveryActionId

may be derived from:

recovery plan identity
+
action index
+
action parameters

rather than generated randomly.

---

80. Exactly-once versus at-least-once

The resilience architecture MUST NOT assume exactly-once distributed delivery unless the underlying infrastructure guarantees it.

The planner should tolerate:

duplicate
missing
delayed
reordered

messages according to the coordination contract.

The execution layer remains authoritative for whether an action actually occurred.

---

81. Recovery determinism

Recovery must be deterministic at the planning level.

Given the same:

incident
diagnosis
policy
state
capabilities
history
strategy versions
seed

the same recovery plan MUST be produced.

Execution may still encounter new external conditions.

Those conditions create a new observation snapshot and potentially a new decision.

---

82. Retry determinism

Retry count MUST never be hard-coded into the deterministic algorithm.

Forbidden:

for _ in 0..3

unless "3" is an explicit policy/configuration input.

Instead:

retry budget
retry policy
incident state
attempt history

determine retry eligibility.

---

83. Retry identity

Every retry attempt MUST be identifiable.

The planner must distinguish:

same recovery action retried

from:

new recovery strategy

This distinction affects history and policy.

---

84. Checkpoint determinism

Checkpoint selection MUST be deterministic for a fixed:

execution state
checkpoint inventory
checkpoint policy
integrity state
compatibility state

Checkpoint inventory ordering MUST be canonical.

---

85. Checkpoint compatibility

A checkpoint created under one environment MUST NOT be considered compatible merely because its format can be decoded.

Compatibility must include relevant:

program identity
IR identity
schema version
execution model
resource model
QEC model
hardware capability requirements
policy

The checkpoint subsystem owns the detailed compatibility contract.

Resilience consumes its result.

---

86. Serialization determinism

"serialization/*" MUST provide canonical representations for deterministic objects.

Encoding MUST NOT depend on:

- hash-map order;
- pointer addresses;
- thread order;
- host architecture where the schema requires portability;
- debug formatting;
- incidental field ordering.

---

87. Schema evolution

Schema changes create a new deterministic domain.

A replay using schema version "N" MUST NOT silently claim equivalence to version "N+1".

Compatibility must be explicitly established by:

serialization/version.rs
checkpoint/compatibility.rs

and the relevant compatibility contracts.

---

88. Deterministic errors

Errors participating in deterministic control flow MUST be classified deterministically.

The same input condition MUST produce the same:

error class
error code
retryability
severity
recovery eligibility

unless the policy explicitly introduces nondeterminism.

Error messages intended for humans may contain operational details that vary, but machine-readable error identity MUST remain stable.

---

89. Error ordering

When multiple errors occur simultaneously, their aggregation order MUST be deterministic.

The system MUST NOT select a primary error solely because it arrived first.

Use canonical severity and stable identity ordering.

---

90. Limits

"limits/*" MUST NOT introduce hidden deterministic limits.

Limits must come from:

resource capacity
policy
backend capability
runtime configuration
security policy
implementation constraints

If a resource is exhausted, the failure must be explicit.

The system MUST NOT silently pretend the resource does not exist.

---

91. Scalability

Determinism MUST scale with available resources.

There is no architectural machine-size maximum.

The subsystem MUST support:

one qubit
small QPU
large QPU
logical-QPU system
multi-QPU system
distributed quantum system

without source-code changes merely because the number of resources changes.

---

92. "Infinity" rule

"Infinite scale" means:

«No artificial maximum is encoded in resilience semantics.»

It does NOT mean:

infinite RAM
infinite CPU
infinite network
infinite QPU capacity

The actual execution environment remains finite.

The architecture must therefore scale until available resources or explicit operational constraints are exhausted.

---

93. Streaming determinism

Large systems MUST NOT require collecting an unbounded history in memory.

Streaming algorithms used by deterministic logic MUST have deterministic state transitions.

Conceptually:

state_n
+
event_n
=
state_(n+1)

The same ordered event stream MUST produce the same state.

---

94. Streaming checkpoints

Long-running deterministic streams SHOULD support checkpointing of detector/aggregator state.

The checkpoint must capture enough state to continue deterministically.

Restarting from a valid checkpoint MUST produce the same subsequent decision as uninterrupted processing, given the same subsequent ordered events.

---

95. Backpressure determinism

Backpressure policy MUST NOT make results depend on incidental scheduler timing.

For example:

worker slow
→ event dropped

must not happen silently if the same event would be preserved in another run.

Sampling/drop policies MUST be explicit and deterministic where strict reproducibility is required.

---

96. Deterministic sampling

If sampling is required:

sample policy
+
seed
+
stream identity
+
input identity

must define the selected sample.

A system MUST NOT use the current time as an implicit sampling seed in strict deterministic mode.

---

97. Deterministic degradation

Degradation decisions must depend on explicit:

resource state
policy
constraints
objective

not on arbitrary discovery order.

If multiple resources are equally suitable for quarantine, migration, or reuse, deterministic tie-breaking MUST apply.

---

98. Backend selection

Backend selection MUST be deterministic for a fixed:

backend capability snapshot
health snapshot
policy
cost model
security policy
workload requirements

If two backends are equivalent under all declared criteria, a stable identity tie-breaker MUST be used.

Provider name string ordering alone is acceptable only if provider identity is a canonical stable key.

---

99. Backend migration

Migration is a new decision context when the target capability state changes.

The planner MUST record:

source backend
destination backend
source capability snapshot
destination capability snapshot
migration policy

A migrated workload MUST retain its canonical program identity.

---

100. Hardware failure and deterministic recovery

Hardware failures are external events.

The deterministic contract is:

same observed hardware-failure snapshot
+
same policy/state
        ↓
same recovery decision

It does not require the hardware failure itself to happen identically.

---

101. Calibration drift

Calibration values are dynamic observations.

Strict replay MUST use the recorded calibration snapshot rather than re-querying current calibration.

Otherwise:

same replay request

could produce:

different plan

simply because the physical device changed.

---

102. Topology determinism

Topology must be represented canonically.

The same topology must have:

same nodes
same identities
same edges
same edge properties
same ordering

regardless of discovery order.

Where nodes are quantum resources, canonical identities must be preserved.

---

103. Physical qubit ordering

A list such as:

[PhysicalQubitId(17), PhysicalQubitId(2), PhysicalQubitId(9)]

MUST NOT be assumed to have semantic ordering.

When order matters, explicitly canonicalize it.

Do not use:

physical qubit numeric value

as a semantic ordering unless the hardware/IR contract defines it as such.

---

104. Operation ordering

Quantum operations MUST be ordered according to the canonical IR's semantics.

Resilience MUST NOT reorder operations merely to obtain deterministic output.

The canonical IR remains authoritative.

Where a transformed implementation is produced, its ordering must be validated by the appropriate compiler/optimization/scheduling subsystem.

---

105. State-machine determinism

"state/recovery.rs" MUST define explicit transitions.

For a fixed:

current state
event
policy
context

the next state MUST be deterministic.

Invalid transitions MUST produce deterministic errors.

---

106. State transition example

Conceptually:

IDLE
 ↓ incident
DETECTING
 ↓ evidence sufficient
DIAGNOSING
 ↓ diagnosis
PLANNING
 ↓ feasible plan
ADAPTING
 ↓ adaptation successful
RECOVERING
 ↓ execution
VERIFYING
 ↓ verified
COMPLETED

Failure paths must be explicit:

ESCALATED
FAILED
REJECTED

No implicit state transition may exist.

---

107. State persistence

Persisted resilience state MUST contain enough information to reproduce deterministic transitions.

A process restart MUST NOT change the next decision merely because:

in-memory collection ordering

changed.

---

108. Process restart determinism

Given an equivalent persisted state:

before restart

and:

after restart

the resilience subsystem MUST make the same deterministic decision.

This is a critical production requirement.

---

109. Crash recovery

Crash recovery MUST distinguish:

action planned

from:

action started

from:

action completed

from:

action verified

The state machine must not guess.

Unknown state MUST trigger explicit reconciliation.

---

110. Reconciliation determinism

After restart or network partition:

local state
+
authoritative execution state
+
policy

must be reconciled deterministically.

If conflicting evidence cannot be resolved deterministically, the system MUST escalate rather than guess.

---

111. Security and determinism

Security controls are part of the deterministic input context.

For example:

authorization state
trust state
resource quarantine state
credential validity
artifact integrity

may affect the decision.

These states must therefore be snapshotted where they influence deterministic planning.

---

112. Fail-closed behavior

If a required deterministic input is missing or corrupted:

DO NOT GUESS
DO NOT SUBSTITUTE CURRENT STATE
DO NOT SILENTLY RETRY
DO NOT CLAIM DETERMINISM

Instead:

return explicit deterministic error

or:

escalate

according to policy.

---

113. Unknown values

Unknown is a valid state.

The system MUST distinguish:

false

from:

unknown

and:

unavailable

and:

invalid

Deterministic decisions must define behavior for each.

---

114. Confidence

Confidence values MUST be deterministic for identical evidence.

Confidence does not mean certainty.

A deterministic system may deterministically conclude:

confidence = low

and then deterministically choose:

ESCALATE

That is correct behavior.

---

115. Learning from history

History may improve future decisions.

However, history updates MUST be versioned.

A new verified outcome creates a new historical snapshot.

It MUST NOT silently modify the historical state underlying an already-recorded decision.

---

116. Deterministic feedback

Learning feedback MUST reference:

decision identity
execution identity
verification result
observed outcome

A failed recovery MUST not be fed into the model merely because the execution completed.

Only the verified outcome should be used for authoritative feedback.

---

117. Deterministic benchmarking integration

Benchmark results used by resilience MUST have stable identities.

The benchmarking subsystem remains authoritative for benchmark methodology.

Resilience consumes benchmark observations such as:

historical reliability
fidelity
latency
error rate
recovery success

A benchmark snapshot used by a decision MUST be identifiable.

---

118. Simulation determinism

Simulation SHOULD be the primary environment for deterministic resilience testing.

A test should be able to define:

canonical program
+
synthetic hardware
+
synthetic capabilities
+
synthetic fault stream
+
policy
+
seed

and reproduce:

detection
diagnosis
planning
adaptation
recovery
verification

exactly.

---

119. Fault injection determinism

Fault injection MUST support explicit seeds and deterministic event schedules.

For example:

seed = S
fault schedule = F

must produce the same synthetic fault stream.

The same injection must result in the same resilience decision when all other inputs are identical.

---

120. Property-based determinism testing

The test suite MUST include generated inputs.

For each generated valid state:

decision(input)

must equal:

decision(clone(input))

where the clone is semantically identical.

The test suite should also permute unordered input collections.

The result MUST remain identical after canonical normalization.

---

121. Permutation tests

For every deterministic collection-processing algorithm:

input = [A, B, C, D]

should be tested against:

[A, B, D, C]
[D, C, B, A]
[C, A, D, B]

and other permutations.

After normalization:

decision(input)
==
decision(permutation(input))

must hold.

---

122. Concurrency tests

The test suite MUST compare:

single worker

against:

multiple workers

where parallelism is supported.

The deterministic result MUST be identical.

---

123. Replay tests

For every replayable decision:

original decision

must be compared with:

replayed decision

The expected identity MUST match.

---

124. Serialization round-trip tests

For every deterministic object:

object
→ encode
→ decode
→ object

must preserve deterministic identity.

Where canonical serialization is defined:

encode(object)
==
encode(decode(encode(object)))

must hold.

---

125. Cross-process tests

The same deterministic input should be processed by independent processes.

The decision identity MUST match.

This catches hidden process-local state.

---

126. Cross-machine tests

Where portable deterministic decisions are promised, test on supported target architectures.

The result MUST match where the specification guarantees architecture-independent behavior.

If an architecture-specific numerical implementation is unavoidable, that limitation MUST be explicitly declared rather than hidden.

---

127. Cross-toolchain tests

The production target is Rust 1.97 / 1.97.1.

The CI contract SHOULD verify the supported toolchain versions.

The code MUST NOT depend on undefined behavior or compiler-specific ordering.

---

128. No "unsafe"

The deterministic test suite MUST include a repository-wide check that resilience source files contain no prohibited "unsafe" implementation.

This requirement is independent of determinism but forms part of the production contract.

---

129. Deterministic test categories

"src/quantum/resilience/tests/" MUST include tests covering at least:

model
detection
diagnosis
policy
planning
adaptation
recovery
mitigation
verification
checkpoint
serialization
determinism
scalability
fault injection
end-to-end

The dedicated:

tests/determinism.rs

is mandatory.

---

130. Required determinism tests

"tests/determinism.rs" MUST verify:

1. identical input → identical decision;
2. reordered observations → identical decision;
3. reordered resource collections → identical decision;
4. reordered registry discovery → identical decision;
5. single-threaded → same decision as parallel;
6. process restart → same decision;
7. serialization round trip → same decision;
8. explicit seed → same randomized strategy decision;
9. changed seed → potentially different randomized realization;
10. changed capability snapshot → new decision identity;
11. changed policy → new decision identity;
12. changed strategy version → new decision identity;
13. stale plan → deterministic rejection/replan;
14. missing deterministic input → deterministic error;
15. corrupted snapshot → deterministic rejection;
16. duplicate event handling → deterministic result;
17. distributed message reordering → deterministic result where the protocol guarantees order-independent behavior;
18. cache hit/miss → same result;
19. history snapshot replay → same result;
20. learned-model snapshot replay → same result.

---

131. Metamorphic tests

The suite SHOULD include metamorphic properties.

Examples:

Collection permutation

permute(input)

must not change the decision.

Equivalent serialization

Equivalent canonical objects must produce identical decision identity.

Cache transparency

Cache enabled/disabled must not change the decision.

Parallelism transparency

Worker count must not change the decision.

Snapshot identity

Changing an input that does not affect the relevant decision MUST NOT unnecessarily change the decision semantics.

---

132. Determinism and scalability

Determinism MUST remain valid as the number of resources increases.

The system must not become nondeterministic merely because:

N = 1

becomes:

N = 1,000,000

provided the same canonical input model and deterministic algorithms are used.

Resource limits may cause:

OUT_OF_RESOURCES

but must not cause silent nondeterministic behavior.

---

133. Bounded resource behavior

No deterministic algorithm is required to operate beyond available resources.

However, exhaustion must be explicit.

For example:

planning memory exhausted

must not become:

different plan because one candidate was silently dropped

unless the resource-exhaustion behavior itself is explicitly defined as part of the policy.

---

134. Deterministic resource exhaustion

Where an implementation must prune candidates because of bounded resources, pruning MUST be deterministic.

For example:

candidate priority
→ stable identity
→ deterministic cutoff

must determine which candidates survive.

The planner MUST NOT depend on allocation order.

---

135. Local versus global decisions

The resilience architecture SHOULD prefer the smallest sufficient scope.

For example:

one physical-qubit failure

may produce:

local remapping

instead of:

global recompilation

if semantics permit.

But the selected scope MUST be deterministic from the incident and policy.

---

136. Deterministic adaptation scope

The choice among:

operation
region
logical qubit
QEC block
circuit
execution
device
backend
distributed workload

must be based on explicit:

incident scope
dependency graph
policy
capabilities
cost
verification requirements

not incidental implementation behavior.

---

137. Cost-model determinism

"planning/cost.rs" MUST produce deterministic cost values for fixed inputs.

Cost models MUST specify:

- units;
- precision;
- missing-value behavior;
- normalization;
- overflow behavior;
- tie-breaking.

Costs from external systems must be snapshotted before strict planning.

---

138. Multi-objective ranking

If planning optimizes:

correctness
fidelity
latency
cost
resource usage
availability

the ranking semantics MUST be explicitly defined.

Do not rely on arbitrary floating-point weighted sums unless the policy specifies:

weights
precision
rounding
normalization
tie-breaking

---

139. Pareto decisions

If Pareto ranking is used, the resulting frontier MUST be canonically ordered.

The final selection MUST have a deterministic tie-breaker.

---

140. Policy objective changes

Changing an objective or objective weight creates a new policy identity.

A previous decision MUST remain associated with the old policy.

---

141. Deterministic escalation

Escalation decisions MUST be deterministic.

Given the same:

incident
diagnosis
policy
budget
attempt history

the same escalation outcome must be produced.

---

142. Recovery budget

Budgets are explicit inputs.

Examples:

retry budget
time budget
shot budget
compilation budget
mitigation budget
migration budget
recovery budget

No budget may be silently embedded in implementation code.

---

143. Attempt history

Attempt history must be ordered canonically.

A retry count must not be inferred from:

vector length

unless that vector itself has a deterministic canonical representation.

---

144. Idempotence

Where an operation is mathematically or operationally idempotent, repeating it should not change the deterministic decision context unexpectedly.

Where an operation is not idempotent, its execution state must be explicit.

---

145. Deterministic cancellation

Cancellation must have explicit semantics.

A cancelled operation MUST NOT be interpreted differently merely because cancellation arrived during a different scheduler tick.

The state transition must depend on the authoritative execution state.

---

146. Timeouts

Timeout decisions are operationally time-sensitive.

Strict replay must use the recorded timeout observation rather than trying to reproduce wall-clock timing.

For deterministic planning:

timeout_event

is the input.

The physical elapsed time that originally produced it is historical evidence.

---

147. Deadline-aware planning

A deadline may be part of the policy.

The deadline itself MUST be captured as an explicit value.

The planner must not call the current wall clock while evaluating a strict deterministic plan.

---

148. Observability

Telemetry, metrics, and traces MUST NOT alter deterministic behavior.

Instrumentation MUST be observationally transparent.

Adding logging MUST NOT change:

ordering
timing-sensitive planning
randomness
resource selection

for strict deterministic logic.

---

149. Logging

Logs MUST NOT be used as a hidden communication channel between deterministic components.

A component must receive explicit state rather than infer state from logs.

---

150. Metrics

Metrics collection MUST NOT mutate planner state.

Counters may be operationally updated, but their values MUST NOT affect deterministic decisions unless explicitly captured as decision inputs.

---

151. Tracing

Trace IDs are operational metadata.

They MUST NOT influence:

- strategy selection;
- candidate ranking;
- random seeds;
- resource allocation.

---

152. Security audit trail

Security audit records must preserve the deterministic decision context.

The audit record SHOULD contain:

decision identity
input snapshot identities
policy identity
strategy identities
authorization context
principal
result
verification

---

153. Deterministic auditability

An auditor should be able to answer:

Why was this resource selected?
Why was this recovery chosen?
Which policy was active?
Which fault evidence was used?
Which hardware snapshot was used?
Which strategy version was used?
Was randomness used?
Was the plan stale?
Why was the result accepted?

without reconstructing hidden process state.

---

154. Deterministic decision record

A production decision record SHOULD conceptually contain:

DecisionRecord {
    decision_id
    decision_kind
    program_identity
    ir_identity
    execution_identity
    resource_snapshot_identity
    capability_snapshot_identity
    observation_snapshot_identity
    policy_identity
    strategy_identities
    registry_identity
    implementation_identity
    randomness_identity
    decision
    verification_identity
}

The concrete Rust type belongs to the appropriate resilience API/provenance implementation.

---

155. Decision identity

The identity of a deterministic decision should be content-derived from the canonical decision record.

Changing any semantically relevant input should produce a different decision identity.

Changing irrelevant metadata should not.

---

156. Decision equality

Two decisions are equal only when their semantic decision content is equal.

Do not use:

pointer equality
allocation identity
process identity
thread identity

for deterministic equality.

---

157. Provenance graph

Production provenance SHOULD form a graph:

Program
  ↓
Canonical IR
  ↓
Policy
  ↓
Capabilities
  ↓
Observations
  ↓
Incident
  ↓
Diagnosis
  ↓
Plan
  ↓
Adaptation
  ↓
Execution
  ↓
Result
  ↓
Verification

Every node SHOULD have stable identity.

---

158. Deterministic replay boundary

Replay is guaranteed only for components whose contracts explicitly support it.

For example:

planner
policy evaluation
incident aggregation
diagnosis
verification

can have strong replay guarantees.

A physical QPU cannot necessarily provide identical replay.

The documentation MUST clearly state the boundary.

---

159. Compatibility with existing repository architecture

This specification integrates with the repository's existing architecture as follows.

Canonical IR

Use:

crate::quantum::ir

for program semantics.

Qubit identity

Use:

crate::quantum::ir::qubit

for canonical logical and physical qubit identities.

ZQN

Consume canonical fault semantics from:

crate::quantum::zqn

rather than creating a second fault ontology.

Hardware

Consume capabilities, health, topology, execution state, and device identity from:

crate::quantum::hardware

Routing

Request logical-to-physical adaptation from:

crate::quantum::routing

Scheduling

Request timing/resource rescheduling from:

crate::quantum::scheduling

Optimization

Request canonical-IR optimization from:

crate::quantum::optimization

QEC

Consume and coordinate with the existing QEC/error-correction subsystem.

Benchmarking

Consume benchmark observations without reimplementing benchmark methodology.

Simulation

Use simulation for deterministic fault/recovery testing.

---

160. Dependency direction

The architectural direction is:

quantum::ir
      ↓
ZQN / QEC / optimization
      ↓
routing / scheduling
      ↓
hardware
      ↓
execution
      ↓
observations
      ↓
quantum::resilience

Resilience orchestrates these contracts.

The lower layers MUST NOT depend on concrete resilience implementations.

This prevents circular dependencies.

---

161. No duplicate quantum IR

"quantum::resilience" MUST NOT define a second:

QuantumCircuit
QuantumOperation
Gate
QubitId
PhysicalQubitId
Measurement

as canonical representations.

The canonical IR remains authoritative.

---

162. No duplicate hardware model

Resilience MUST NOT create another hardware inventory.

Its "model/capability.rs" and "model/resource.rs" represent resilience-facing views/contracts, not a competing hardware authority.

The hardware subsystem remains authoritative.

---

163. No duplicate routing model

Resilience MAY represent a routing request/result reference.

It MUST NOT implement the canonical routing graph or routing algorithm.

---

164. No duplicate scheduler

Resilience MAY request rescheduling.

It MUST NOT become a second scheduler.

---

165. No duplicate optimizer

Resilience MAY request reoptimization.

It MUST NOT duplicate optimization passes.

---

166. Deterministic integration contract

Every integration call MUST define:

inputs
outputs
identity
version
snapshot semantics
failure semantics
determinism guarantees

before the integration is considered complete.

---

167. API requirement

"api/controller.rs" MUST expose a deterministic execution/resilience entry point whose context contains all required deterministic inputs.

Conceptually:

ResilienceRequest
    +
ResilienceContext
    +
Policy
    +
Snapshots
    +
RandomnessContext
        ↓
ResilienceController
        ↓
Decision / Plan / Verification

The controller MUST NOT obtain hidden state from global services.

---

168. Request immutability

A deterministic "ResilienceRequest" MUST be immutable after planning begins.

If requirements change:

new request

must be created.

---

169. Context immutability

A deterministic planning context MUST be treated as an immutable snapshot.

Mutable runtime state must be separated from deterministic decision state.

---

170. Snapshot lifecycle

The recommended model is:

live system
    ↓
capture
    ↓
validate
    ↓
freeze
    ↓
hash/identify
    ↓
plan

The planner then operates only on the frozen snapshot.

---

171. Snapshot consistency

Related snapshots MUST be mutually consistent.

For example:

capability snapshot
resource snapshot
topology snapshot
calibration snapshot

must not accidentally represent incompatible moments unless the execution model explicitly supports that.

The snapshot creation mechanism must define consistency semantics.

---

172. Snapshot freshness

Freshness is separate from determinism.

A stale snapshot can still be deterministic.

Therefore:

deterministic

does not mean:

current

The policy must decide whether stale data is acceptable.

---

173. Freshness policy

The planner MUST evaluate snapshot validity according to explicit policy.

Examples:

fresh enough
expired
unknown freshness
invalid

must be distinct states.

---

174. Determinism and availability

Determinism must not be achieved by blindly refusing all adaptation.

The architecture supports:

deterministic adaptation

where the current resource state is part of the input.

For example:

machine A unavailable
machine B available

can deterministically result in:

select B

provided the capability/resource snapshot says so.

---

175. Determinism and self-healing

Self-healing remains compatible with determinism:

fault
 ↓
observation snapshot
 ↓
diagnosis
 ↓
policy
 ↓
deterministic plan
 ↓
recovery
 ↓
new observation snapshot
 ↓
new deterministic decision

Each iteration is deterministic relative to its own explicit inputs.

---

176. Determinism and dynamic hardware

Dynamic hardware is not a contradiction.

The hardware state is simply another input.

Therefore:

same hardware snapshot
→ same decision

while:

different hardware snapshot
→ potentially different decision

is expected.

---

177. Determinism and "write once"

The user-facing program remains independent of the physical realization.

The program does not contain:

retry count
physical qubit number
provider-specific backend
hard-coded topology
fixed hardware size

unless the language semantics explicitly permit such constraints.

Resilience determines the realization.

---

178. Deterministic physical adaptation

When resources change:

logical program

remains stable.

Only:

mapping
schedule
optimization
QEC configuration
mitigation
backend

may change within their respective contracts.

The change must be provenance-bearing and verified.

---

179. Deterministic migration

A migration decision MUST be deterministic for a fixed:

source state
destination capabilities
policy
security state
cost model
program requirements

Migration execution itself remains subject to external conditions.

---

180. Deterministic quarantine

Resource quarantine selection MUST use stable resource identity and policy.

A quarantined resource MUST NOT be selected again until an explicit verified state transition returns it to service.

---

181. Deterministic recovery ordering

When several recovery actions are required, their order MUST be explicitly defined.

For example:

quarantine
→ reroute
→ reschedule
→ recompile
→ execute
→ verify

must not become:

whatever worker completed first

unless the architecture explicitly permits independent parallel actions.

---

182. Safe parallel recovery

Independent recovery actions MAY execute concurrently.

Their final state MUST be reconciled deterministically.

The system must define:

dependency graph
conflict rules
merge rules
failure rules
ordering

---

183. Conflict resolution

If two concurrent recovery actions conflict:

action A
action B

the conflict MUST be resolved using explicit deterministic policy.

Do not use lock acquisition order as the semantic winner.

---

184. Locking

Synchronization primitives are implementation mechanisms.

Lock acquisition order MUST NOT define semantic decision order.

Locks protect state.

They do not define planner semantics.

---

185. Deadlock and determinism

Deadlock avoidance is a reliability property.

However, changing lock timing MUST NOT change the deterministic logical result.

Where possible, immutable snapshots should reduce lock-dependent decision logic.

---

186. Atomic state transitions

State transitions that affect deterministic planning MUST be atomic from the perspective of the decision engine.

Partial state updates must not be visible as a valid deterministic snapshot.

---

187. Transaction boundaries

Checkpoint/state persistence SHOULD use explicit transaction boundaries.

A replayable snapshot must correspond to a complete logical state, not an arbitrary intermediate write.

---

188. Deterministic persistence

Persistence ordering must not affect logical state identity.

If two writes are semantically equivalent, the resulting canonical state must be identical.

---

189. Database independence

The deterministic contract MUST NOT depend on a particular database implementation.

Database results must be normalized into canonical snapshots before deterministic planning.

---

190. External service independence

Resilience MUST NOT directly embed assumptions about:

specific cloud provider
specific database
specific telemetry vendor
specific quantum provider

in deterministic core logic.

Adapters provide those integrations.

---

191. Provider neutrality

A provider-specific implementation may exist under an adapter boundary.

The core planner MUST operate on provider-neutral contracts.

---

192. Deterministic provider adaptation

Provider adapters MUST normalize provider-specific results into canonical contracts before they reach deterministic logic.

Provider quirks MUST NOT leak into core ordering semantics.

---

193. Error-provider normalization

Provider-specific errors must be mapped to stable resilience error classes.

The provider's raw error string MUST NOT determine recovery behavior.

---

194. Deterministic diagnostics

Human-readable diagnostic messages may differ across environments.

Machine-readable diagnostics MUST remain stable.

The planner should consume:

error class
error code
severity
recoverability
evidence

rather than raw text.

---

195. Deterministic observability export

Exporters must not change decision behavior.

A failed telemetry exporter MUST NOT silently alter the planner.

If observability is itself a mandatory policy requirement, failure must produce an explicit deterministic policy result.

---

196. Deterministic security state

Security decisions that affect resilience must be represented explicitly.

Examples:

resource trusted
resource quarantined
checkpoint verified
principal authorized
plugin approved

These become deterministic inputs when relevant.

---

197. Supply-chain determinism

A dependency update can alter behavior.

Therefore production provenance SHOULD identify:

application version
dependency lock state
strategy/plugin versions
schema versions

where required for reproducibility.

---

198. Build reproducibility

The project should maintain reproducible build practices where practical.

The deterministic runtime contract does not require byte-for-byte identical binaries, but production provenance should identify the build sufficiently to reproduce the decision implementation.

---

199. Compiler optimization

Compiler optimizations MUST NOT be relied upon for semantic ordering.

The source-level contract defines determinism.

Compiler optimizations may change implementation details while preserving the defined result.

---

200. Test oracle

A deterministic test MUST compare semantic decision identity, not incidental representation.

For example, differences in:

debug formatting
memory address
trace ID
allocation layout

must not fail a semantic determinism test.

---

201. Golden vectors

Production resilience SHOULD maintain deterministic golden vectors containing:

input snapshot
expected decision
expected provenance identity
expected error where applicable

Golden vectors provide regression protection across refactors.

---

202. Golden-vector versioning

Every golden vector MUST identify:

schema version
strategy version
policy version
implementation compatibility

A changed intended behavior requires a deliberate golden-vector update.

---

203. Fuzzing

Fuzzing MUST test:

- malformed snapshots;
- unordered data;
- duplicate events;
- conflicting observations;
- invalid identifiers;
- extreme resource counts;
- extreme numeric values;
- serialization failures;
- corrupted checkpoints;
- malformed policies.

A fuzz failure MUST never cause undefined behavior because "unsafe" is forbidden.

---

204. Extreme-scale testing

The test architecture SHOULD generate resource counts dynamically.

Do not make:

1000
10000
100000

the semantic limits.

They are merely test points.

The algorithms must operate based on the available resources and their representational limits.

---

205. Memory scalability

Deterministic algorithms SHOULD avoid retaining information that is not required for the decision.

Use:

streaming
aggregation
bounded state
incremental processing
snapshot references
content addressing

where appropriate.

Any bounded state must have explicit semantics.

---

206. CPU scalability

Parallel processing is allowed.

The deterministic result must not depend on worker count.

---

207. Distributed scalability

Distributed execution may partition the problem.

Partitioning MUST either:

1. be deterministic; or
2. produce results that are merged through a deterministic reduction.

---

208. Partitioning

A partition function MUST have stable semantics.

For example:

resource identity
→ deterministic partition

rather than:

thread ID
→ partition

---

209. Distributed reduction

Distributed partial results MUST be merged deterministically.

The merge operation must define:

ordering
identity
conflict resolution
error handling

---

210. Determinism under partial failure

A distributed system may lose workers.

The planner MUST distinguish:

worker result absent

from:

worker result = negative

Missing evidence MUST NOT silently become a value.

---

211. Consensus

If distributed consensus is required, resilience MUST use the established coordination abstraction.

It MUST NOT invent a second consensus model inside the planner.

The consensus result becomes an explicit deterministic input.

---

212. Recovery after network partition

A network partition may create divergent observations.

The system must reconcile them using the coordination contract.

If they cannot be reconciled safely:

ESCALATE

rather than guessing.

---

213. Deterministic learning history

History must be append-oriented where possible.

An event should not be rewritten merely to make a replay pass.

Corrections should be represented as explicit correction events.

---

214. Event sourcing compatibility

If event sourcing is used, replay order MUST be canonical.

Equivalent event streams must produce equivalent state.

---

215. Time-travel debugging

Production tooling SHOULD support reconstructing the resilience state at a recorded decision boundary.

This is especially valuable for:

fault
→ diagnosis
→ plan
→ recovery
→ verification

analysis.

---

216. Determinism diagnostics

When strict determinism fails, the system SHOULD identify the first differing input.

Useful categories include:

PROGRAM_CHANGED
IR_CHANGED
POLICY_CHANGED
CAPABILITIES_CHANGED
RESOURCES_CHANGED
OBSERVATIONS_CHANGED
HISTORY_CHANGED
STRATEGY_CHANGED
REGISTRY_CHANGED
MODEL_CHANGED
RANDOMNESS_CHANGED
SCHEMA_CHANGED
IMPLEMENTATION_CHANGED
SECURITY_STATE_CHANGED

---

217. Determinism mismatch report

A replay mismatch report SHOULD contain:

expected decision identity
actual decision identity
first differing input category
expected input identity
actual input identity
strategy identity
policy identity
schema identity

Sensitive information must follow the security policy.

---

218. Deterministic compatibility

"COMPATIBILITY.md" defines broader compatibility rules.

This document adds:

«A compatibility layer MUST NOT silently claim deterministic equivalence when a version change can alter decision semantics.»

Compatibility and determinism are related but distinct.

---

219. Backward compatibility

Older recorded decisions should remain interpretable where the repository's compatibility contract promises it.

If not possible:

REPLAY_INCOMPATIBLE

must be returned.

---

220. Forward compatibility

A newer resilience implementation MUST NOT assume it can replay an older snapshot unless the schema compatibility contract explicitly permits it.

---

221. Deterministic migrations

If a snapshot schema is migrated:

old schema
→ migration
→ new schema

the migration itself MUST be deterministic.

The migrated result must have a new schema identity.

---

222. No silent migration

A migration MUST NOT silently alter semantic values.

Any semantic transformation must be explicit and provenance-bearing.

---

223. API stability

The public resilience API must not expose hidden determinism requirements.

If a caller requests strict deterministic mode, the API must make required inputs explicit.

---

224. Builder/API validation

Request builders MUST reject incomplete deterministic contexts before planning.

Examples:

missing policy
missing capability snapshot
missing strategy identity
missing required randomness

must be detected early.

---

225. Fail-before-execute

A deterministic plan that lacks required provenance or validation MUST NOT reach hardware execution.

The execution boundary is a safety gate.

---

226. Deterministic plan validation

Before execution, validate:

plan identity
input snapshot identity
policy identity
capability validity
resource availability
security authorization
strategy compatibility

If any required condition changed:

revalidate

or:

replan

---

227. Deterministic verification after adaptation

Every adaptation creates a new artifact identity.

For example:

original IR
→ adapted IR

requires:

adapted IR identity

and semantic verification.

---

228. Semantic preservation

The verification layer MUST determine whether an adaptation preserves the required program semantics.

Resilience MUST NOT infer semantic equivalence merely because:

execution completed

---

229. Acceptance determinism

Given identical verification evidence and acceptance policy:

acceptance decision

must be identical.

---

230. Degraded acceptance

If the policy permits:

DEGRADED_ACCEPT

the exact conditions must be deterministic and recorded.

---

231. Abort determinism

Abort decisions must be explicit.

An abort caused by:

policy budget
security violation
semantic uncertainty
resource exhaustion
unrecoverable hardware state

must have stable machine-readable classification.

---

232. Deterministic default behavior

Defaults are dangerous if hidden.

Every default that can affect a deterministic decision MUST be:

- documented;
- versioned;
- included in policy/configuration identity.

---

233. Default ordering

Default strategy ordering MUST be explicit.

Do not rely on:

source-file order
module declaration order
registration order
linker order
filesystem order

unless that ordering is explicitly defined as stable by the contract.

---

234. Configuration

Configuration should be immutable for a decision.

If configuration changes:

new configuration snapshot

creates a new decision context.

---

235. Environment portability

Deterministic decisions should not depend on:

OS-specific path ordering
locale
timezone
host name
CPU count
thread count
memory address

unless explicitly captured as inputs.

---

236. CPU count

A planner MAY use multiple CPUs.

The CPU count MUST NOT change the semantic decision.

It may affect performance.

---

237. Memory pressure

Memory pressure MUST NOT silently change the algorithm's semantic result.

If memory constraints require fallback behavior, that fallback must be explicitly defined and deterministic.

---

238. Out-of-memory behavior

Out-of-memory conditions are operational failures.

The subsystem is not required to recover from arbitrary host-level OOM termination.

However, its own bounded resource management MUST fail explicitly where possible.

---

239. Deterministic resource admission

Admission decisions must be based on explicit resource state.

For example:

available memory
required memory
policy
priority

must produce a deterministic admission result.

---

240. Priority

Priority is a policy input.

If two requests have equal priority, deterministic tie-breaking is required.

---

241. Fairness

Fairness mechanisms MUST NOT accidentally become nondeterministic.

If fairness affects scheduling of resilience actions, its ordering contract must be explicit.

---

242. Starvation

Preventing starvation is a runtime property.

If starvation handling changes deterministic behavior, it must be represented in the explicit state model.

---

243. Deterministic lifecycle events

Lifecycle events must have stable event identities and explicit state transitions.

Examples:

incident_created
diagnosis_completed
plan_created
plan_invalidated
recovery_started
recovery_completed
verification_completed

---

244. Event schema version

Every persisted event MUST have a schema version.

The version is part of deterministic replay identity.

---

245. Canonical event payload

Event payloads must be canonically serialized before hashing.

Debug representations MUST NOT be used as canonical identity.

---

246. Security-sensitive randomness

Cryptographic randomness and deterministic reproducibility are different requirements.

A security-sensitive operation MUST NOT replace cryptographically secure randomness with a deterministic seed merely for replayability.

Where security and deterministic replay conflict, the operation must define a safe replay mechanism without weakening production security.

---

247. Production versus test randomness

Tests may use deterministic seeds.

Production security operations may require secure randomness.

The distinction must be explicit.

---

248. Deterministic simulation versus production security

A simulator may use a deterministic random generator.

A security-sensitive production component MUST follow its security contract.

Resilience must record which randomness domain was used.

---

249. No hidden entropy

Strict deterministic planning MUST NOT obtain entropy from:

OS random source
current time
thread timing
memory address
network timing

unless randomness is explicitly part of the mode.

---

250. Deterministic mode rejection

If strict deterministic mode encounters a component that requires uncontrolled randomness, it MUST return a deterministic incompatibility error.

It MUST NOT silently substitute a deterministic approximation unless the policy explicitly allows it.

---

251. Deterministic mode and external providers

If a provider cannot supply enough information to reconstruct a deterministic decision, resilience must classify the decision accordingly.

It must not fabricate missing state.

---

252. Provider nondeterminism

Provider behavior outside Zamani's control does not violate planner determinism.

The resulting observation is simply a new external input.

---

253. Deterministic observation capture

To replay a decision, external observations must be captured before the decision is made or otherwise reconstructed from authoritative history.

---

254. Observation immutability

Once an observation snapshot is bound to a decision, it must not be mutated.

Corrections create new observations.

---

255. Corrected observations

If telemetry is later determined to be wrong:

original observation
→ correction event
→ new observation state
→ new decision

The original decision remains historically valid for the evidence available at that time.

---

256. Temporal determinism

A deterministic decision is always relative to an explicit state boundary.

Therefore the system should use:

observation snapshot

rather than vague statements such as:

current hardware state

inside replayable planning.

---

257. Snapshot boundary identity

Every snapshot should have a stable identity.

A decision references the snapshot identity.

---

258. Snapshot composition

Composite snapshots should identify all constituent snapshots.

For example:

ExecutionContextSnapshot
 ├── capability snapshot
 ├── topology snapshot
 ├── calibration snapshot
 ├── policy snapshot
 ├── security snapshot
 └── observation snapshot

---

259. Snapshot completeness

The system MUST document whether a snapshot is:

complete
partial
best-effort

A partial snapshot must not be mislabeled complete.

---

260. Partial deterministic decisions

A decision can be deterministic over incomplete information.

For example:

unknown hardware status
→ deterministic policy says ESCALATE

That is valid.

Determinism does not require complete knowledge.

---

261. Deterministic uncertainty

Unknown information must remain unknown.

The planner may deterministically choose:

do not execute

because uncertainty violates the policy.

---

262. Deterministic safety preference

Where multiple outcomes are possible and safety policy requires conservative behavior, the conservative outcome must be selected deterministically.

---

263. Determinism and correctness

Correctness takes precedence over deterministic convenience.

A deterministic but semantically invalid plan MUST be rejected.

The correct priority is:

semantic correctness
>
security
>
verification
>
policy
>
deterministic reproducibility
>
performance

Determinism never authorizes an unsafe or incorrect execution.

---

264. Determinism and performance

A deterministic algorithm may be slower.

Performance optimizations are permitted only if they preserve the deterministic contract.

---

265. Deterministic fast paths

A fast path and slow path must produce equivalent semantic decisions.

If not, they are different strategies and must have different identities.

---

266. Strategy identity after optimization

An optimized implementation may change internal algorithms while preserving semantics.

If the output remains contractually identical, implementation provenance may change while semantic strategy identity remains stable.

If behavior can change, the strategy version must change.

---

267. Refactoring rule

A refactor MUST NOT alter deterministic outputs accidentally.

Golden vectors and replay tests should detect such regressions.

---

268. Algorithm replacement

Replacing a planner algorithm requires:

new implementation identity
new tests
comparison against old behavior where compatibility is required
explicit version decision

---

269. Determinism regression gate

CI MUST reject changes that unexpectedly alter deterministic golden decisions.

Intentional changes require updated version/provenance and tests.

---

270. CI requirements

Production CI SHOULD run:

cargo fmt --check
cargo check
cargo test
cargo clippy

using the supported Rust toolchain.

The exact repository CI policy remains authoritative.

---

271. Repeated-run testing

Determinism tests SHOULD run multiple times.

For example:

same test input
many process runs

must yield the same semantic result.

This catches hidden timing and ordering dependencies.

---

272. Stress testing

Deterministic stress tests should vary:

thread count
resource count
event count
input ordering
batch size
partitioning
cache state

without changing the expected semantic result.

---

273. Determinism under retries

The same deterministic decision must not change simply because an internal computation was retried.

Retries of pure computation must be semantically transparent.

---

274. Determinism under cache eviction

Cache eviction must not change the decision.

---

275. Determinism under process restart

Persisted decision context must be sufficient for replay after restart.

---

276. Determinism under node migration

If a distributed resilience worker moves to another node, the same deterministic decision must be produced from the same explicit snapshot.

---

277. Determinism under hardware migration

Changing hardware creates a new capability/resource snapshot and therefore potentially a new plan.

The program identity remains stable.

---

278. Determinism under QEC migration

Changing QEC implementation/configuration creates a new strategy/model context.

The logical program identity remains stable.

---

279. Determinism under optimization migration

Changing optimization strategy creates a new adapted-artifact identity.

The original canonical program identity remains stable.

---

280. Determinism under scheduling migration

Changing scheduling strategy changes the physical realization, not the logical program identity.

The schedule identity must be recorded.

---

281. Determinism and provenance chain

The complete chain should remain reconstructible:

program
→ IR
→ policy
→ capabilities
→ observations
→ fault
→ incident
→ diagnosis
→ plan
→ routing
→ scheduling
→ optimization
→ QEC
→ execution
→ result
→ verification

The exact participating stages vary by workload.

---

282. Minimal deterministic context

A deterministic decision context MUST contain the smallest complete set of inputs required by the decision.

Do not include irrelevant mutable state.

This makes replay:

smaller
faster
more stable
more auditable

---

283. Input relevance

Each deterministic input SHOULD document:

why it affects the decision

This prevents accidental dependency growth.

---

284. Dependency audit

Before a deterministic component is production-ready, reviewers MUST identify:

all inputs
all mutable state
all randomness
all external calls
all ordering dependencies
all clocks
all caches
all environment dependencies

---

285. Determinism audit checklist

A component is deterministic only if all answers below are satisfactory:

- Are all inputs explicit?
- Are all collections canonically ordered?
- Are ties explicitly resolved?
- Is randomness controlled?
- Is time explicit?
- Is global state absent?
- Are caches transparent?
- Are parallel reductions deterministic?
- Are external observations snapshotted?
- Is version identity recorded?
- Is serialization canonical?
- Is provenance complete?
- Is replay possible?
- Are errors stable?
- Is stale state rejected?
- Are resource identities canonical?
- Is "quantum::ir::qubit" used where required?
- Is "unsafe" absent?
- Are scalability limits externally supplied?

---

286. File-level integration requirements

The following files have explicit determinism responsibilities.

"api/request.rs"

Must carry deterministic-mode requirements and explicit context references.

"api/context.rs"

Must carry frozen deterministic snapshots rather than hidden live state.

"api/controller.rs"

Must orchestrate deterministic lifecycle without introducing hidden inputs.

"model/resource.rs"

Must use stable resource identity.

"model/fault.rs"

Must preserve ZQN identity/provenance.

"model/incident.rs"

Must deterministically aggregate faults.

"model/capability.rs"

Must represent snapshot-bound capabilities.

"detection/*"

Must normalize ordering and preserve observation identity.

"diagnosis/*"

Must provide deterministic classification and tie-breaking.

"policy/*"

Must be immutable/versioned during a decision.

"planning/*"

Must be deterministic for a fixed context.

"adaptation/*"

Must bind adaptation to source snapshots and target snapshots.

"recovery/*"

Must use stable action/recovery identities.

"mitigation/*"

Must explicitly control randomness where applicable.

"verification/*"

Must produce deterministic acceptance decisions from fixed evidence.

"state/*"

Must expose immutable snapshots to deterministic planning.

"checkpoint/*"

Must support deterministic serialization and replay compatibility.

"telemetry/*"

Must normalize external event ordering.

"history/*"

Must expose stable historical snapshots.

"learning/*"

Must freeze model state for strict deterministic planning.

"coordination/*"

Must define deterministic distributed ordering.

"serialization/*"

Must define canonical representations.

"errors/*"

Must define stable deterministic machine-readable errors.

"limits/*"

Must use explicit resource/policy limits rather than hidden constants.

"registry/*"

Must provide stable strategy ordering and immutable snapshots.

---

287. Root "mod.rs"

"src/quantum/resilience/mod.rs" MUST remain a module composition boundary.

It MUST NOT contain hidden global state or initialization whose order affects deterministic behavior.

---

288. Documentation integration

This file is normative together with:

ARCHITECTURE.md
DESIGN.md
SECURITY.md
SCALABILITY.md
COMPATIBILITY.md
FAILURE_MODES.md
RECOVERY_MODEL.md
OBSERVABILITY.md

If documents appear to conflict:

1. semantic correctness wins;
2. security constraints win;
3. canonical IR ownership wins;
4. explicit API contracts win;
5. this determinism specification governs deterministic behavior;
6. implementation details do not override normative contracts.

---

289. Conflict rule with scalability

"SCALABILITY.md" establishes that resilience must not impose artificial machine-size ceilings.

This document adds:

«Deterministic algorithms may be bounded by available execution resources, but those bounds must be explicit operational constraints and must not become hidden semantic limits.»

---

290. Conflict rule with security

"SECURITY.md" establishes fail-closed security behavior.

Therefore strict deterministic replay MUST NOT bypass:

authentication
authorization
integrity
semantic verification

for convenience.

---

291. Conflict rule with recovery

"RECOVERY_MODEL.md" defines recovery state transitions.

This document requires those transitions to be deterministic for fixed state/event/policy inputs.

---

292. Conflict rule with compatibility

"COMPATIBILITY.md" controls cross-version compatibility.

This document requires deterministic replay to fail explicitly when compatibility cannot be established.

---

293. Production definition

"quantum::resilience" is deterministic-production-ready only when:

all deterministic inputs are explicit
+
canonical identities are used
+
ordering is explicit
+
randomness is controlled
+
parallelism is deterministic
+
snapshots are immutable
+
provenance is complete
+
replay is tested
+
serialization is canonical
+
stale plans are rejected
+
security is preserved
+
scalability has no artificial semantic ceiling
+
unsafe code is absent

---

294. Required invariants

The following are hard production invariants.

D-001

Same deterministic input context MUST produce the same deterministic decision.

D-002

No hidden mutable global state may affect strict deterministic decisions.

D-003

No uncontrolled randomness may affect strict deterministic decisions.

D-004

No incidental collection iteration order may affect decisions.

D-005

No worker completion order may affect deterministic decisions.

D-006

No wall-clock reading may affect strict deterministic decisions unless explicitly captured as input.

D-007

No stale plan may execute without revalidation.

D-008

Canonical quantum identities MUST come from "quantum::ir::qubit".

D-009

Resilience MUST NOT create a competing canonical qubit identity.

D-010

Physical mapping MUST remain owned by routing/hardware contracts.

D-011

Deterministic decisions MUST have reproducible provenance.

D-012

Randomized strategies MUST expose their randomness contract.

D-013

Cache state MUST NOT change semantic results.

D-014

Parallel execution MUST preserve deterministic semantic results.

D-015

Distributed execution MUST use explicit deterministic ordering/reconciliation where required.

D-016

Missing deterministic evidence MUST NOT be silently fabricated.

D-017

Unknown MUST remain distinct from false.

D-018

Security controls MUST NOT be bypassed for replay.

D-019

Schema/version changes MUST be explicit.

D-020

Deterministic behavior MUST be tested under reordered inputs.

D-021

Deterministic behavior MUST be tested across process restarts.

D-022

Deterministic behavior MUST be tested under concurrency.

D-023

Deterministic behavior MUST be tested under serialization round trips.

D-024

Resource exhaustion MUST be explicit.

D-025

No artificial machine-size maximum may be encoded into resilience semantics.

D-026

Rust "unsafe" is forbidden.

D-027

Rust 2021 and Rust 1.97 / 1.97.1 are the supported implementation targets.

D-028

A deterministic decision MUST NOT be accepted merely because execution completed.

D-029

Verification remains the acceptance authority.

D-030

A deterministic plan that cannot be verified MUST NOT be represented as a verified result.

---

295. Final deterministic execution model

The complete production model is:

                  Zamani Program
                        |
                        v
                Canonical Quantum IR
                        |
                        v
                Capture Decision Context
                        |
        +---------------+----------------+
        |               |                |
        v               v                v
   Policy Snapshot  Capability       Resource
                    Snapshot         Snapshot
        |               |                |
        +---------------+----------------+
                        |
                        v
              Observation Snapshot
                        |
                        v
                Fault / Incident
                        |
                        v
                    Diagnosis
                        |
                        v
                 Deterministic Policy
                        |
                        v
                 Deterministic Plan
                        |
             +----------+----------+
             |          |          |
             v          v          v
          Routing   Scheduling Optimization
             |          |          |
             +----------+----------+
                        |
                        v
                       QEC
                        |
                        v
                    Mitigation
                        |
                        v
                     Execute
                        |
                        v
                   Observe Again
                        |
                        v
                    Verify
                        |
              +---------+---------+
              |                   |
              v                   v
           ACCEPT            REPLAN/ESCALATE

The important property is that each decision is bound to the state from which it was derived.

---

296. The fundamental Zamani guarantee

The ultimate guarantee of this architecture is:

«A Zamani quantum program does not become nondeterministically different merely because the resilience machinery is distributed, parallel, cached, restarted, migrated, scaled, or executed on a different number of resources.»

Instead:

same semantic program
+
same declared decision context
        ↓
same deterministic resilience decision

while:

different legitimate physical conditions
        ↓
different explicit decision context
        ↓
different valid physical realization

The program's semantic identity remains stable.

---

297. Final production rule

The resilience subsystem MUST always distinguish these three statements:

"I made the same decision."

"I performed the same physical execution."

"I obtained a semantically verified equivalent result."

They are not interchangeable.

Production Zamani resilience requires the first, may or may not be able to guarantee the second depending on the execution target, and MUST establish the third through verification before accepting a result.

That distinction is the foundation of deterministic, scalable, self-healing quantum execution.

---

298. Completion criterion

"src/quantum/resilience/DETERMINISM.md" is considered implemented only when the corresponding code and tests enforce this specification.

Documentation alone is not sufficient.

The production gate is:

DETERMINISM.md
        +
implementation
        +
canonical IR integration
        +
ZQN integration
        +
hardware integration
        +
routing integration
        +
scheduling integration
        +
optimization integration
        +
QEC integration
        +
checkpoint integration
        +
verification integration
        +
serialization integration
        +
fault-injection tests
        +
scalability tests
        +
replay tests
        +
concurrency tests
        +
CI
        =
production-ready determinism

No individual resilience implementation may weaken these requirements merely because a particular backend, simulator, QPU, or execution environment behaves differently.This version deliberately makes determinism a cross-cutting contract, not merely a statement that “same seed gives same result.” It also preserves the repository's existing canonical-identity rule: quantum::ir::qubit remains authoritative rather than introducing a resilience-specific qubit type.

The existing resilience SECURITY.md and SCALABILITY.md already establish the corresponding safe-Rust, canonical-identity, and no-artificial-limit principles, so this document is written to integrate with those rather than contradict them.