Below is the complete production-oriented src/quantum/resilience/RECOVERY_MODEL.md. It is written as a normative contract for the Rust implementation, and is designed to integrate with the resilience files already defined: model/, detection/, diagnosis/, policy/, planning/, adaptation/, recovery/, mitigation/, verification/, state/, checkpoint/, telemetry/, history/, learning/, coordination/, serialization/, errors/, limits/, and registry/.

I have also aligned the model with current quantum execution practice: mitigation can introduce significant execution/sampling overhead, noise models can become stale, and resilience therefore has to treat resource/cost budgets and observation freshness as first-class concerns. 

# Zamani Quantum Resilience — Recovery Model

**Path:** `src/quantum/resilience/RECOVERY_MODEL.md`  
**Status:** Normative / Production Architecture  
**Version:** 1.0  
**Language:** Rust 2021  
**Supported Rust:** Rust 1.97 / Rust 1.97.1  
**Safety:** No `unsafe`  
**Scope:** Backend-independent quantum recovery and resilience orchestration

---

# 1. Purpose

This document defines the normative recovery model for:

`quantum::resilience`

The recovery subsystem is responsible for safely responding to execution failures, hardware degradation, quantum faults, resource changes, backend failures, QEC degradation, compiler/runtime failures, and other conditions that may prevent a Zamani quantum program from completing with acceptable semantic correctness.

The recovery model MUST support:

- one qubit;
- one logical qubit;
- small quantum devices;
- large QPUs;
- fault-tolerant quantum computers;
- heterogeneous quantum systems;
- distributed quantum systems;
- simulators;
- emulators;
- hybrid classical/quantum execution;
- dynamically changing hardware;
- partial resource availability;
- arbitrary resource sizes permitted by the available system resources.

The architecture MUST NOT contain a finite architectural maximum for:

- qubits;
- logical qubits;
- physical qubits;
- backends;
- recovery attempts;
- incidents;
- detectors;
- strategies;
- checkpoints;
- execution stages;
- recovery plans;
- resources.

Practical limits MUST come from:

- discovered hardware capabilities;
- available memory;
- available compute;
- available quantum resources;
- configured policies;
- execution budgets;
- provider limits;
- security policy;
- scheduling constraints;
- user-declared requirements.

"Infinite scale" therefore means:

> The architecture imposes no artificial finite scalability ceiling; execution remains bounded only by the resources and policies available to the deployment.

---

# 2. Core Principle

The central recovery invariant is:

> A recovery action MUST preserve the semantics of the original Zamani quantum program, or the system MUST explicitly report that semantic preservation could not be established.

Availability MUST never be treated as sufficient justification for accepting a recovered result.

A recovery action is acceptable only when:

```text
Semantic validity
        AND
Capability validity
        AND
Policy validity
        AND
Security validity
        AND
Provenance validity
        AND
Verification validity

are satisfied.


---

3. Scope

This document defines:

recovery states;

recovery transitions;

recovery plans;

recovery actions;

preconditions;

postconditions;

recovery budgets;

retry semantics;

restart semantics;

resume semantics;

rollback semantics;

migration semantics;

compensation semantics;

checkpoint interaction;

partial execution handling;

distributed recovery;

concurrent recovery;

stale-plan handling;

verification;

escalation;

deterministic recovery;

failure containment;

recovery storms;

recovery loops;

resource exhaustion;

recovery provenance;

recovery security;

recovery testing;

scalability.


This document does NOT redefine:

canonical quantum IR;

quantum fault semantics;

QEC algorithms;

routing algorithms;

scheduling algorithms;

optimization algorithms;

hardware implementations;

simulator implementations;

compiler internals.


Those responsibilities remain in their respective subsystems.


---

4. Architectural Ownership

The following ownership model is mandatory.

Responsibility	Owner

Canonical quantum representation	quantum::ir
Qubit identity	quantum::ir::qubit::QubitId
Quantum fault/noise semantics	quantum::zqn
Hardware identity/capabilities	quantum::hardware
Physical topology	quantum::hardware / routing
Routing	quantum::routing
Scheduling	quantum::scheduling
Optimization	quantum::optimization
QEC	quantum QEC subsystem
Compilation	compiler
Execution mechanics	runtime / hardware HAL
Simulation	simulation subsystem
Benchmarking	benchmarking subsystem
Recovery orchestration	quantum::resilience
Recovery policy	quantum::resilience::policy
Recovery planning	quantum::resilience::planning
Recovery execution	quantum::resilience::recovery
Recovery verification	quantum::resilience::verification
Recovery history	quantum::resilience::history


Resilience MUST orchestrate these systems rather than duplicate them.


---

5. Canonical Type Rule

There MUST NOT be a second resilience-specific qubit identity.

When resilience needs to refer to a qubit, it MUST use:

quantum::ir::qubit::QubitId

where the canonical IR exposes that path.

The following are forbidden:

resilience::QubitId
resilience::PhysicalQubitId
resilience::LogicalQubitId
resilience::QuantumGate
resilience::QuantumCircuit

unless those types represent genuinely different domain concepts and do not compete with canonical quantum types.

Recovery MUST reference canonical IR objects or stable identifiers derived from them.


---

6. Recovery Is an Orchestration Layer

Recovery is not:

retry()

Recovery is:

observe
    ↓
validate observation
    ↓
contain
    ↓
diagnose
    ↓
evaluate policy
    ↓
generate plans
    ↓
validate feasibility
    ↓
select plan
    ↓
reserve ownership
    ↓
execute adaptation/recovery
    ↓
verify
    ↓
accept / continue / recover again / escalate


---

7. Recovery Lifecycle

The normative lifecycle is:

EXECUTING
    |
    v
OBSERVED
    |
    v
CONTAINING
    |
    v
DIAGNOSING
    |
    v
POLICY_EVALUATION
    |
    v
PLANNING
    |
    v
PLAN_VALIDATION
    |
    v
OWNERSHIP_ACQUISITION
    |
    v
ADAPTING
    |
    v
RECOVERING
    |
    v
VERIFYING
    |
    +--------------------+
    |                    |
    v                    v
ACCEPTED              REJECTED
    |                    |
    v                    v
COMPLETED          REPLAN / ESCALATE

Additional states MAY exist, but the implementation MUST preserve the semantics of these states.


---

8. Recovery States

The canonical recovery states are:

Idle
Observed
Acknowledged
Containing
Diagnosing
PolicyEvaluation
Planning
PlanValidation
OwnershipAcquisition
Adapting
Recovering
Verifying
Accepted
Degraded
Escalated
Rejected
Terminal

These states MUST be represented explicitly rather than inferred from arbitrary flags.


---

9. State Meaning

9.1 Idle

No active recovery operation exists.


---

9.2 Observed

A failure or degradation has been observed.

No recovery action has yet been authorized.


---

9.3 Acknowledged

The observation has been accepted as a resilience event.

This does NOT mean the failure is confirmed.


---

9.4 Containing

The system is preventing further propagation.

Possible actions include:

isolating a resource;

stopping unsafe execution;

preventing reuse of a failed resource;

suspending dependent execution;

protecting checkpoint state;

preventing duplicate recovery.



---

9.5 Diagnosing

Evidence is being correlated and classified.

Diagnosis MAY remain uncertain.


---

9.6 PolicyEvaluation

The system determines what recovery actions are allowed.


---

9.7 Planning

One or more candidate recovery plans are generated.


---

9.8 PlanValidation

Plans are checked against current:

capabilities;

state;

policy;

security;

provenance;

budgets.



---

9.9 OwnershipAcquisition

A recovery operation obtains exclusive or appropriately coordinated authority over affected resources.


---

9.10 Adapting

The implementation is changed without changing program semantics.

Examples:

remapping;

rerouting;

rescheduling;

recompilation;

reoptimization;

QEC adaptation;

backend selection.



---

9.11 Recovering

The selected recovery action is executed.


---

9.12 Verifying

The result is tested against the acceptance criteria.


---

9.13 Accepted

Recovery succeeded and the resulting execution is verified.


---

9.14 Degraded

Execution continues with reduced capability while remaining within policy.


---

9.15 Escalated

Automatic recovery cannot safely establish correctness.

Human, higher-level orchestration, or another policy domain MAY be required.


---

9.16 Rejected

A recovery result exists but cannot be accepted.


---

9.17 Terminal

No further automatic recovery is permitted.


---

10. Recovery State Machine

Allowed transitions MUST be explicit.

Idle
  → Observed

Observed
  → Acknowledged
  → Escalated
  → Terminal

Acknowledged
  → Containing
  → Diagnosing
  → Escalated

Containing
  → Diagnosing
  → Escalated
  → Terminal

Diagnosing
  → PolicyEvaluation
  → Escalated
  → Terminal

PolicyEvaluation
  → Planning
  → Escalated
  → Terminal

Planning
  → PlanValidation
  → Escalated
  → Terminal

PlanValidation
  → OwnershipAcquisition
  → Planning
  → Escalated
  → Terminal

OwnershipAcquisition
  → Adapting
  → Recovering
  → Planning
  → Escalated

Adapting
  → Recovering
  → Planning
  → Escalated
  → Terminal

Recovering
  → Verifying
  → Planning
  → Escalated
  → Terminal

Verifying
  → Accepted
  → Degraded
  → Planning
  → Rejected
  → Escalated
  → Terminal

Accepted
  → Terminal

Degraded
  → Planning
  → Accepted
  → Escalated
  → Terminal

Rejected
  → Planning
  → Escalated
  → Terminal

Escalated
  → Terminal

Terminal
  → no automatic transition

An implementation MUST NOT allow arbitrary state transitions.


---

11. Recovery State Ownership

state/recovery.rs owns the state machine.

recovery/recoverer.rs owns recovery orchestration.

planning/plan.rs owns immutable recovery plans.

verification/acceptance.rs owns acceptance decisions.

No other file should independently invent recovery states.


---

12. Recovery Incident

Every recovery operation MUST be associated with an incident.

An incident MUST contain or reference:

IncidentId;

one or more FailureIds;

source;

affected resources;

logical/physical scope;

detection evidence;

diagnosis;

severity;

confidence;

state;

parent incident;

child incidents;

causal relationships;

provenance;

recovery history;

verification history.



---

13. Failure Identity

Every failure MUST have a stable identity.

Conceptually:

FailureId

A failure identity MUST NOT depend exclusively on wall-clock time.

It SHOULD incorporate stable information such as:

source;

failure class;

resource scope;

causal context;

normalized evidence;

execution identity.



---

14. Incident Correlation

Multiple observations MAY represent one incident.

Example:

Qubit A error
Qubit B error
Qubit C error
Calibration drift
Readout degradation

MAY represent:

one correlated hardware degradation incident

rather than independent recovery operations.

This prevents recovery storms.


---

15. Parent/Child Incidents

Incidents MUST support hierarchical relationships.

Example:

Backend outage
├── Device unavailable
├── Execution timeout
├── Result unavailable
└── Recovery migration required

The parent incident may own recovery coordination.

Child incidents provide evidence.


---

16. Recovery Action

A recovery action is a semantic operation requested by the planner.

Canonical action classes include:

Retry
Restart
Resume
Rollback
Remap
Reroute
Reschedule
Recompile
Reoptimize
ChangeQEC
Mitigate
SwitchBackend
QuarantineResource
Abort
Compensate

The action list MUST remain extensible.


---

17. Recovery Action Contract

Every recovery action MUST declare:

ActionIdentity
ActionVersion
Preconditions
RequiredCapabilities
RequiredResources
PolicyRequirements
ExpectedEffects
FailureModes
CostModel
RiskModel
RollbackCapability
Idempotence
VerificationRequirements
ProvenanceRequirements

No action should be treated as a generic opaque callback.


---

18. Preconditions

A recovery action MUST have explicit preconditions.

Examples:

required capability exists
affected resource is available
execution identity is known
checkpoint is compatible
policy permits migration
result has not already been accepted
ownership is valid
observation is fresh enough

If a precondition fails, the plan is stale or infeasible.

The action MUST NOT execute merely because it was previously planned.


---

19. Postconditions

Every recovery action MUST define expected postconditions.

Examples:

resource no longer quarantined
execution resumed
new schedule installed
new mapping installed
new backend selected
checkpoint restored

Postconditions are subsequently verified.


---

20. Recovery Plan

A recovery plan MUST be immutable after activation.

Conceptually:

RecoveryPlan
├── PlanId
├── PlanVersion
├── IncidentId
├── CreatedFromStateVersion
├── ObservationVersion
├── CapabilitySnapshot
├── PolicySnapshot
├── Preconditions
├── Actions
├── ExpectedEffects
├── Cost
├── Risk
├── VerificationPlan
├── RollbackPlan
└── Provenance


---

21. Plan Immutability

Once a plan is executing, its meaning MUST NOT silently change.

If conditions change:

old plan
    ↓
invalid
    ↓
new observation
    ↓
new plan

Do not mutate an active plan underneath the executor.


---

22. Stale Plan Detection

A plan MUST become invalid if material state changes.

Examples:

hardware capability changed;

affected qubit became unavailable;

topology changed;

calibration changed materially;

checkpoint became incompatible;

policy changed;

budget changed;

ownership expired;

execution identity changed;

required resource disappeared;

telemetry became invalid;

security state changed.


The implementation MUST use explicit state/capability versions or equivalent freshness mechanisms.


---

23. Stale Plan Rule

A stale plan MUST NOT be executed.

Required behavior:

detect stale
    ↓
stop plan
    ↓
record reason
    ↓
refresh observations
    ↓
re-diagnose if necessary
    ↓
re-plan


---

24. Retry Model

Retry is not synonymous with recovery.

Retry is permitted only when:

1. the operation is known to be retryable;


2. the operation is idempotent or protected by execution identity;


3. semantic duplication is understood;


4. policy permits retry;


5. resource budgets permit retry;


6. the failure is classified as potentially transient;


7. verification remains possible.




---

25. No Hard-Coded Retry Count

Forbidden:

retry three times

Forbidden:

const MAX_RETRIES = 3;

Instead:

RetryPolicy
    → configured retry budget
    → failure classification
    → historical evidence
    → resource budget

The number of retries MUST be policy-derived.


---

26. Retry Storm Prevention

The system MUST detect recovery amplification.

Example:

backend failure
→ 100 jobs retry
→ 100 jobs retry again
→ queue overload
→ more timeouts
→ more retries

This is a recovery storm.

The system MUST support:

shared incident correlation;

retry budgets;

exponential/backoff policies;

concurrency limits from policy;

admission control;

circuit breaking;

escalation.


These controls MUST be dynamic.


---

27. Idempotent Recovery

Where possible, recovery operations SHOULD have stable execution identity.

Conceptually:

ExecutionIdentity
+
RecoveryAttemptIdentity
+
ActionIdentity

A repeated request can then be recognized as:

same request

rather than:

new execution


---

28. Unknown Submission State

A particularly dangerous case is:

submit quantum job
       ↓
network failure
       ↓
client does not know whether provider received job

The system MUST NOT blindly resubmit.

Required procedure:

UNKNOWN_SUBMISSION
        ↓
query execution identity
        ↓
if found:
    continue tracking
else:
    evaluate policy
        ↓
    retry only if safe


---

29. Partial Execution

A partial quantum execution MUST NOT automatically be considered successful.

Examples:

some shots completed;

some circuit variants completed;

a batch partially executed;

some distributed partitions completed;

a provider returned incomplete results;

execution stopped after a subset of operations.


The recovery system MUST explicitly represent partial completion.


---

30. Quantum State Recovery

Generic quantum state serialization MUST NOT be assumed.

The system MUST distinguish:

classical execution state
compiled circuit state
logical checkpoint
measurement boundary
QEC state
provider-supported quantum state
reconstructible state

A checkpoint is valid only if the underlying execution environment supports restoration of the state it claims to represent.


---

31. Resume

Resume is allowed only from a valid recovery boundary.

A resume point MUST have:

known semantic state;

compatible program/IR;

compatible hardware capabilities;

compatible QEC configuration;

valid checkpoint or reconstructible state;

valid provenance;

valid policy authorization.



---

32. Restart

Restart means beginning execution again from an approved semantic boundary.

Restart MUST record:

original execution
restart reason
new execution identity
new target
new configuration
new policy snapshot

A restart MUST NOT erase the original failure history.


---

33. Rollback

Rollback MUST restore only states that can actually be restored.

Rollback MUST NOT imply that arbitrary quantum state can be reconstructed from metadata.

For quantum state, rollback MAY instead mean:

restore a valid checkpoint

or:

reconstruct from a semantic boundary

or:

restart the relevant computation


---

34. Migration

Migration means moving execution to another compatible resource.

Possible migration targets:

physical device;

QPU;

backend;

simulator;

emulator;

distributed partition;

logical resource.


Migration MUST use capability negotiation.

It MUST NOT contain provider-specific branching in the core recovery engine.


---

35. Migration Compatibility

Before migration, compare:

program requirements
IR version
operation requirements
topology requirements
timing requirements
measurement requirements
QEC requirements
logical resource requirements
precision requirements
policy constraints
security requirements

Only compatible targets may be selected.


---

36. Remapping

Logical-to-physical mapping belongs to the routing/hardware subsystem.

Resilience may request:

remap affected logical resources

but MUST NOT implement a competing routing algorithm.

Physical qubits MUST be identified using the canonical IR/hardware contracts, including:

quantum::ir::qubit::QubitId

where applicable.


---

37. Rerouting

When physical connectivity changes:

failure
    ↓
capability snapshot
    ↓
routing request
    ↓
new route
    ↓
schedule validation
    ↓
execution

Recovery MUST verify that the new route satisfies the original logical computation.


---

38. Rescheduling

Scheduling recovery MUST be requested from:

quantum::scheduling

The resilience layer provides:

affected resources;

changed capabilities;

constraints;

recovery intent.


The scheduler provides the new schedule.


---

39. Recompilation

Recompilation MAY be required when:

target capabilities change;

native instructions change;

topology changes;

QEC mode changes;

timing changes;

optimization assumptions become invalid.


Recompilation MUST begin from the canonical IR or another formally valid intermediate representation.


---

40. Reoptimization

Reoptimization MAY occur after a target change.

The optimizer remains responsible for optimization.

Resilience decides whether reoptimization is necessary.


---

41. QEC Adaptation

QEC adaptation MAY change:

code configuration;

code distance;

decoder;

logical layout;

ancilla allocation;

syndrome strategy;

fault-tolerant execution strategy.


Resilience MUST NOT implement the QEC algorithm itself.


---

42. Mitigation Versus Recovery

Mitigation is not recovery.

Mitigation generally attempts to improve result quality without necessarily eliminating the underlying fault.

Recovery changes execution conditions or execution strategy.

Example:

noise detected
    ↓
mitigation

versus:

device degraded
    ↓
migration

Both may participate in one recovery plan.


---

43. Mitigation Cost

Mitigation MAY increase:

circuit count;

sampling;

runtime;

classical processing;

QPU consumption;

financial cost.


Therefore mitigation MUST participate in policy and budget evaluation.

Current quantum systems explicitly expose tradeoffs between resilience/result quality and execution overhead; this is why mitigation cannot be treated as an unlimited free recovery action.


---

44. Recovery Ordering

A planner SHOULD prefer the least disruptive feasible action according to policy.

A generic ordering may be:

continue
    ↓
localized adaptation
    ↓
mitigation
    ↓
reschedule
    ↓
reroute
    ↓
remap
    ↓
recompile
    ↓
reoptimize
    ↓
retry
    ↓
restart/resume
    ↓
migrate
    ↓
change QEC
    ↓
escalate

This is NOT a hard-coded priority.

Policy and cost/risk models determine actual ordering.


---

45. Recovery Cost

Recovery cost SHOULD be multidimensional.

Possible dimensions:

latency
quantum execution time
classical computation
shots
qubits
logical qubits
physical qubits
energy
memory
network bandwidth
financial cost
error probability
semantic risk

The cost model MUST be extensible.


---

46. Recovery Risk

Risk MUST be distinct from cost.

A plan can be:

cheap but unsafe

or:

expensive but safe

Safety policy MUST take precedence over cost optimization.


---

47. Verification Is Mandatory

Every recovery operation MUST eventually enter:

Verifying

unless it terminates before producing a result.

No recovery result may bypass verification.


---

48. Verification Layers

Verification SHOULD include:

1. Structural verification
2. Resource verification
3. Capability verification
4. Semantic verification
5. Result verification
6. Provenance verification
7. Policy verification
8. Security verification
9. Statistical/confidence verification


---

49. Structural Verification

Check:

result exists;

result schema is valid;

required fields exist;

no malformed data;

execution identity matches;

serialization is valid.



---

50. Resource Verification

Check:

required resources existed;

no unauthorized resource was used;

affected resources are accounted for;

resource state is consistent.



---

51. Capability Verification

Verify that the execution used capabilities compatible with the plan.


---

52. Semantic Verification

The recovered execution MUST remain semantically equivalent to the intended computation to the degree required by the program and policy.

The canonical source is:

quantum::ir

Resilience MUST NOT create a separate semantic definition.


---

53. Result Verification

Results MUST be checked for:

completeness;

consistency;

expected shape;

valid measurement interpretation;

statistical sufficiency;

provider/result integrity.



---

54. Provenance Verification

Verify:

program
IR
IR version
compiler
optimizer
routing
schedule
hardware
capabilities
calibration
fault observations
recovery plan
actions
mitigation
QEC configuration
execution
result
verification

are linked consistently.


---

55. Acceptance Decision

The final decision MUST be explicit.

Possible decisions:

ACCEPT
ACCEPT_DEGRADED
RETRY
REPLAN
ESCALATE
REJECT
ABORT

The exact enum SHOULD be owned by:

verification::acceptance

and reused by the API/state layer.


---

56. No Silent Degradation

The system MUST NOT silently change:

logical precision;

algorithm semantics;

QEC requirements;

resource guarantees;

result confidence;

execution target;


merely to make a job succeed.

If degradation is permitted, it MUST be represented explicitly.


---

57. Degraded Execution

A degraded execution MAY continue when:

remaining capabilities
+
policy
+
semantic requirements

still permit valid execution.

Example:

original capacity: dynamically discovered
failure removes resources
remaining capacity: reduced
program still fits

The system may continue.

If it no longer fits:

ESCALATE

or apply another permitted recovery strategy.


---

58. Resource Shrinkage

Recovery MUST support:

resource_available
→ resource_degraded
→ resource_unavailable

and the reverse:

resource_unavailable
→ resource_recovered
→ resource_available

without assuming a fixed topology.


---

59. Recovery and Dynamic Topology

Topology MUST be treated as versioned state.

A plan based on topology version T is invalid when the material topology becomes version T+1.

This applies to:

coupling;

control channels;

device availability;

network links;

distributed resources.



---

60. Recovery and Calibration

Calibration changes can invalidate a recovery plan.

A plan MUST reference the calibration/capability snapshot on which it was based.

If calibration changes materially:

invalidate
→ reobserve
→ replan

Noise-learning data MAY also become stale and MUST NOT be reused indefinitely. Current quantum execution systems explicitly warn that learned noise models become stale after time or changing conditions.


---

61. Recovery and Telemetry

Telemetry MUST be treated as evidence, not absolute truth.

Telemetry may be:

missing;

delayed;

contradictory;

corrupted;

duplicated;

malicious;

stale.


The recovery system MUST track observation freshness and trust.


---

62. Contradictory Telemetry

Example:

sensor A: healthy
sensor B: failed
sensor C: unavailable

The system MUST NOT arbitrarily choose one.

It SHOULD:

correlate
→ evaluate source trust
→ diagnose
→ quantify uncertainty
→ select safe action

When uncertainty is too high:

ESCALATE


---

63. Missing Telemetry

Missing telemetry is itself an operational condition.

It MUST NOT be interpreted automatically as:

healthy

or:

failed

The policy determines how missing evidence is handled.


---

64. Recovery Confidence

Every automatically inferred recovery decision SHOULD have confidence metadata.

Confidence MUST NOT replace verification.

For example:

diagnosis confidence = high

does not mean:

result verified = true


---

65. Unknown Failure

Unknown failures MUST be handled safely.

Required behavior:

unknown failure
    ↓
contain
    ↓
preserve evidence
    ↓
avoid unsafe automatic action
    ↓
attempt bounded diagnosis
    ↓
escalate if unresolved

Unknown MUST NOT mean:

ignore


---

66. Compound Failures

The system MUST support multiple simultaneous faults.

Example:

calibration drift
+
qubit loss
+
routing infeasibility
+
queue timeout

Recovery MUST identify:

symptoms;

contributing faults;

root-cause hypotheses;

dependent failures;

independent failures.


It MUST avoid repeatedly recovering symptoms when the root cause remains active.


---

67. Correlated Failure Domains

Recovery MUST support dynamic fault domains:

logical qubit
physical qubit
gate
coupling
control channel
device
backend
cluster
region
network
distributed domain

The actual domains MUST come from the hardware/resource model.


---

68. Failure Containment

Containment MUST:

1. stop propagation;


2. protect valid state;


3. prevent duplicate recovery;


4. isolate affected resources where possible;


5. preserve evidence;


6. maintain provenance;


7. avoid unnecessary disruption to unaffected computation.




---

69. Blast Radius

Every recovery plan SHOULD estimate its blast radius.

Examples:

single qubit
affected gate neighborhood
logical block
circuit partition
execution
device
backend
distributed region

The planner SHOULD prefer smaller blast radius when all other policy constraints are equal.


---

70. Recovery Concurrency

Multiple recovery operations MAY run concurrently if their resource scopes do not conflict.

They MUST NOT concurrently mutate the same protected recovery state without coordination.


---

71. Conflict Detection

Before execution, compare:

resource versions
ownership
plan version
state version
capability version
policy version

If incompatible:

plan invalid

and replan.


---

72. Distributed Ownership

Distributed recovery MUST use explicit ownership.

Possible mechanisms:

lease
lock
versioned ownership
consensus
transactional state

The implementation MUST choose the mechanism appropriate to the deployment.


---

73. Lease Expiration

If a recovery lease expires:

stop action where safely possible
invalidate plan
record ownership loss
reconcile state
replan

Never assume continued authority after lease expiration.


---

74. Split Brain

Distributed recovery MUST protect against two controllers believing they own the same resource.

A split-brain condition MUST cause:

containment
+
ownership reconciliation
+
safe escalation

rather than competing recovery actions.


---

75. Recovery Loop Detection

A recovery loop occurs when:

failure
→ recovery
→ same failure
→ same recovery
→ ...

The system MUST detect repeated states/plans/fingerprints.

Detection MAY use:

plan fingerprints;

state fingerprints;

failure fingerprints;

execution history;

policy budgets.


There MUST NOT be an unconditional infinite automatic recovery loop.


---

76. Recovery Loop Handling

When a loop is detected:

stop repeating plan
    ↓
classify as recovery-loop
    ↓
try a materially different feasible strategy
    ↓
otherwise escalate


---

77. Recovery Budget

Recovery MUST consume explicit budgets where configured.

Possible budgets:

time
shots
classical compute
quantum compute
memory
network
energy
financial cost
recovery attempts
mitigation overhead
compilation overhead

No universal hard-coded values are permitted.


---

78. Budget Exhaustion

When a budget is exhausted:

record exhaustion
→ determine alternative policy
→ degrade if valid
→ otherwise escalate/reject

The system MUST NOT silently exceed the budget.


---

79. Recovery Priority

Recovery priority is policy-defined.

Possible objectives include:

correctness
availability
latency
fidelity
cost
energy
resource preservation

A high-priority objective MUST NOT bypass safety verification.


---

80. Deterministic Recovery

Deterministic mode MUST be supported.

For equivalent inputs, the planner MUST produce the same decision when deterministic execution is requested.

The decision inputs MUST include:

canonical IR
IR version
IR hash
capability snapshot
telemetry snapshot
policy version
strategy registry versions
state snapshot
history snapshot
explicit random seed
ordering semantics


---

81. Deterministic Ordering

If ordering affects decisions:

use an explicitly deterministic ordering;

do not rely on hash-map iteration order;

do not depend on thread scheduling;

do not depend on wall-clock timing;

do not use hidden global mutable state.


BTreeMap, sorted collections, or explicitly ordered sequences SHOULD be used where decision order matters.


---

82. Randomness

Any randomness affecting recovery MUST be explicit.

It MUST have:

seed
algorithm/version
purpose
provenance

Randomness MUST NOT be obtained implicitly from ambient process state in deterministic mode.


---

83. Floating-Point Decisions

Floating-point values MUST NOT be compared naively when doing semantic acceptance decisions.

Tolerance MUST come from:

policy
domain semantics
measurement uncertainty

where applicable.


---

84. Recovery Replay

A deterministic recovery incident SHOULD be replayable from recorded data.

Replay inputs SHOULD include:

canonical IR
capability snapshot
telemetry/event stream
policy
history
state
strategy versions
seed
ordering

Replay MUST NOT require live hardware to reproduce the planner's logical decision.


---

85. Recovery Provenance

Every recovery action MUST produce provenance.

At minimum:

who/what initiated
why
when
incident
diagnosis
policy
plan
action
resource scope
input versions
output versions
verification
result


---

86. Security

Recovery is a security-sensitive subsystem.

Threats include:

forged telemetry;

malicious backend;

compromised controller;

malicious plugin;

checkpoint tampering;

result tampering;

replay attacks;

unauthorized migration;

privilege escalation;

malicious recovery strategy.



---

87. Telemetry Trust

Telemetry SHOULD have:

source identity
authentication
integrity
freshness
trust classification
provenance

Untrusted telemetry MUST NOT authorize destructive recovery by itself.


---

88. Checkpoint Security

Checkpoint data MUST support:

integrity verification;

schema validation;

compatibility validation;

provenance;

anti-replay semantics;

authorization.


Corrupt or untrusted checkpoints MUST NOT be restored.


---

89. Recovery Authorization

Every potentially destructive operation SHOULD require authorization according to policy.

Examples:

migration;

resource quarantine;

backend switching;

checkpoint restore;

program restart;

policy relaxation.



---

90. Plugin Security

Recovery and mitigation strategies MAY be extensible.

Plugins MUST NOT be trusted merely because they are registered.

The registry should support:

identity
version
capabilities
permissions
trust
compatibility


---

91. No Unsafe Rust

The resilience subsystem MUST NOT use:

unsafe

including:

unsafe blocks;

unsafe fn;

unsafe impl;

raw pointer manipulation;

unsafe FFI.


Recovery correctness MUST be implemented using safe Rust abstractions.


---

92. Rust Compatibility

The implementation MUST target:

Rust 2021
Rust 1.97
Rust 1.97.1

Do not require APIs introduced after the supported MSRV unless the repository's declared toolchain is deliberately raised.


---

93. Error Handling

Public resilience operations SHOULD use the repository's canonical error type.

Conceptually:

Result<T, ResilienceError>

Errors MUST preserve:

stable code;

source context;

classification;

retryability;

provenance where appropriate.


Errors and incidents are different concepts.


---

94. Error Versus Incident

An error represents an operation failure.

An incident represents a system-level condition.

Example:

timeout error
    ↓
execution incident
    ↓
diagnosis
    ↓
recovery

Do not use ResilienceError as a replacement for the incident model.


---

95. Integration with errors/

errors/error.rs:

owns the canonical error object.

errors/codes.rs:

owns stable machine-readable error codes.

errors/classification.rs:

owns recoverability/retryability classification.

model/fault.rs:

owns resilience-level normalized fault references.

model/incident.rs:

owns correlated operational incidents.


---

96. Integration with detection/

Detection produces observations.

Detection MUST NOT execute recovery.

Flow:

telemetry
→ detection
→ normalized observation
→ incident correlation


---

97. Integration with diagnosis/

Diagnosis consumes:

observations
history
capabilities
topology
execution state

and produces:

diagnosis

Diagnosis MUST NOT directly mutate hardware.


---

98. Integration with policy/

Policy decides:

allowed
forbidden
preferred
budgeted
required

Policy MUST NOT directly execute actions.


---

99. Integration with planning/

Planner consumes:

incident
diagnosis
state
capabilities
policy
history
budgets

and produces:

RecoveryPlan


---

100. Integration with adaptation/

Adaptation changes the execution realization.

Files:

remapping.rs
rerouting.rs
rescheduling.rs
recompilation.rs
reoptimization.rs
qec_adaptation.rs
backend_selection.rs

must invoke the corresponding external subsystem contracts.


---

101. Integration with recovery/

recovery/recoverer.rs coordinates action execution.

The individual files:

retry.rs
restart.rs
checkpoint.rs
rollback.rs
resume.rs
migration.rs
compensation.rs

implement distinct recovery semantics.


---

102. Integration with verification/

Every completed recovery MUST be sent to:

verification::verifier

before acceptance.


---

103. Integration with state/

state/recovery.rs owns lifecycle state.

state/execution.rs owns execution state.

state/logical.rs owns logical resource state.

state/physical.rs owns physical resource state.

No recovery implementation should maintain a second hidden state machine.


---

104. Integration with checkpoint/

Checkpointing is a supporting recovery mechanism.

Checkpoint files own:

snapshot
manifest
storage
integrity
compatibility

Recovery decides when checkpoint restore is appropriate.


---

105. Integration with telemetry/

Telemetry supplies:

events
metrics
traces
health

Recovery consumes these observations.

Telemetry MUST remain independently useful for debugging even when recovery fails.


---

106. Integration with history/

Every recovery attempt SHOULD be persisted.

History records:

incident
plan
action
result
verification

This enables future planning and deterministic replay.


---

107. Integration with learning/

Learning MAY consume verified historical outcomes.

Learning MUST NOT:

bypass policy;

bypass verification;

mutate semantic requirements;

authorize unsafe actions.



---

108. Integration with coordination/

Distributed recovery uses:

ownership
leases
coordination
consensus where required

No distributed recovery operation may assume local exclusive ownership unless explicitly established.


---

109. Integration with serialization/

All persisted/replayed recovery objects require:

schema
version
encoding
decoding

Serialization MUST preserve semantic identity.


---

110. Integration with limits/

Limits MUST be dynamic.

Examples:

available qubits
available memory
execution time
shots
network bandwidth

must be obtained from capabilities/configuration/policy.


---

111. Integration with registry/

The registry provides extension points for:

detectors
strategies
recovery implementations
backend adapters

Registry lookup MUST be version- and capability-aware.


---

112. Recovery and Canonical IR

The canonical IR MUST remain the source of truth for program semantics.

Recovery may produce:

new mapping
new schedule
new optimized IR
new target-specific realization

but the recovered result MUST remain traceable to the original canonical computation.


---

113. IR Hash

A recovery operation SHOULD retain a canonical identity for the program representation.

Conceptually:

OriginalIRHash
CurrentIRHash

A changed IR does not automatically mean semantic change.

Semantic verification is still required.


---

114. Recovery and Optimization

If optimization changes the IR:

original IR
→ optimizer
→ optimized IR

the provenance chain MUST preserve both.

Recovery MUST know which representation is being executed.


---

115. Recovery and Routing

A route change MUST preserve:

logical operation ordering
required interactions
resource constraints

Routing remains the authority on physical realization.


---

116. Recovery and Scheduling

A schedule change MUST preserve:

dependency ordering
timing constraints
resource exclusivity
measurement/reset semantics

Scheduling remains authoritative.


---

117. Recovery and QEC

QEC state is part of the recovery context.

Recovery MUST account for:

code
decoder
syndrome state
logical resource state
physical resource state
fault history

when QEC is involved.


---

118. Recovery and ZQN

ZQN remains authoritative for quantum fault semantics.

Recovery consumes normalized ZQN information.

Examples include:

leakage;

loss;

erasure;

correlated faults;

gate faults;

measurement faults;

decoherence;

noise channels.


Resilience converts those observations into operational recovery decisions.


---

119. Recovery and Hardware

Hardware provides:

identity
capabilities
status
health
topology
calibration
execution

Recovery requests changes through hardware contracts.

Recovery MUST NOT embed provider-specific implementations in its core.


---

120. Recovery and Simulation

The simulator SHOULD support recovery fault injection.

A simulated recovery cycle should be able to execute:

program
→ fault
→ detection
→ diagnosis
→ planning
→ recovery
→ verification

without real quantum hardware.


---

121. Recovery and Benchmarking

Benchmarking SHOULD provide historical evidence for:

failure rates
recovery success
latency
mitigation overhead
resource consumption
backend reliability

Recovery SHOULD be able to use this information through policy/planning interfaces.


---

122. Graceful Degradation

When resources shrink:

discover remaining capabilities
→ determine whether program still fits
→ adapt
→ verify

If the program still satisfies all requirements:

continue

Otherwise:

migrate
or
replan
or
escalate


---

123. Scale Independence

Recovery algorithms MUST avoid:

fixed arrays based on qubit count
fixed topology sizes
fixed incident counts
fixed backend counts
fixed recovery counts

Use:

iterators;

streams;

dynamically sized collections;

sparse representations;

hierarchical aggregation;

partitioning;

pagination;

bounded configured windows.



---

124. Large-Scale Incident Correlation

A naïve implementation must NOT correlate every resource with every other resource.

Avoid unconditional:

O(N²)

correlation over the entire system.

Prefer:

fault domains
topology locality
time windows
causal relationships
hierarchical aggregation
partitioning

The exact strategy remains configurable.


---

125. Distributed Scale

At large scale, recovery MAY be partitioned:

global controller
    |
    +-- region
    |    +-- backend
    |         +-- device
    |
    +-- region
         +-- backend
              +-- device

Each layer can manage local recovery while the parent layer handles cross-domain incidents.


---

126. Hierarchical Recovery

Recovery SHOULD support:

local recovery
    ↓
device recovery
    ↓
backend recovery
    ↓
regional recovery
    ↓
global recovery

Escalation happens only when local recovery cannot safely solve the incident.


---

127. Recovery Scope

Every recovery operation MUST explicitly identify scope.

Examples:

resource
operation
circuit region
execution
device
backend
distributed domain

This prevents accidental global recovery for a local failure.


---

128. Recovery Blast Radius Constraint

A plan SHOULD specify:

maximum affected scope

as a policy constraint.

If an action would exceed it:

plan rejected

unless policy explicitly allows escalation.


---

129. Backpressure

Telemetry and failure streams may exceed processing capacity.

The system MUST support:

bounded queues;

backpressure;

aggregation;

sampling where policy permits;

prioritization;

overflow handling.


Backpressure MUST NOT silently discard safety-critical incidents.


---

130. Event Ordering

Event ordering MUST use explicit ordering semantics.

Possible mechanisms:

monotonic sequence
logical clock
causal ordering
timestamp + sequence

Wall-clock time alone MUST NOT be assumed to establish causality.


---

131. Duplicate Events

Duplicate failure observations MUST be deduplicated where possible.

Deduplication MUST preserve evidence rather than simply deleting events.


---

132. Lost Events

If telemetry loss is detected:

record observability degradation

Recovery policy decides whether execution may continue.


---

133. Recovery Under Observability Failure

If the system cannot observe enough state to establish safe recovery:

do not guess

It should:

contain
→ obtain additional evidence
→ use a safe fallback if explicitly permitted
→ escalate


---

134. Recovery Under Security Failure

If integrity/authentication fails:

contain
→ quarantine affected evidence/resource
→ preserve forensic information
→ revoke affected authority if policy requires
→ escalate

Do not execute a recovery plan derived from compromised state.


---

135. Recovery Under Policy Failure

Examples:

contradictory constraints
impossible objective
invalid budget
missing authorization

The system MUST reject the plan.

It MUST NOT invent a new policy.


---

136. Recovery Under Capability Failure

If a required capability disappears:

invalidate plan
→ rediscover capabilities
→ replan


---

137. Recovery Under Compiler Failure

If recompilation fails:

record compiler failure
→ determine whether prior executable remains valid
→ try another feasible strategy
→ escalate

Never execute partially compiled output unless the compiler contract explicitly marks it valid.


---

138. Recovery Under Routing Failure

If routing cannot realize the computation:

try alternate feasible route
→ reschedule
→ recompile if needed
→ migrate if permitted
→ escalate

Do not invent physical mappings inside resilience.


---

139. Recovery Under Scheduling Failure

If no valid schedule exists:

re-evaluate capabilities
→ re-route if necessary
→ recompile if necessary
→ consider migration
→ escalate


---

140. Recovery Under QEC Failure

If QEC cannot provide a valid correction path:

contain logical resource
→ evaluate alternate QEC strategy
→ migrate if possible
→ restart from valid checkpoint if possible
→ escalate


---

141. Recovery Under Hardware Failure

Hardware failure MAY cause:

resource quarantine
→ remap
→ reroute
→ reschedule
→ migrate

depending on policy.


---

142. Recovery Under Backend Outage

The system SHOULD:

confirm outage
→ preserve execution identity
→ discover compatible backend
→ evaluate migration
→ recompile/reroute/reschedule
→ execute
→ verify


---

143. Recovery Under Simulator Fallback

A simulator MAY be used only when policy permits it.

A simulator result MUST NOT automatically be presented as equivalent to hardware execution unless the semantic and result requirements allow that interpretation.


---

144. Recovery Under Partial Distributed Failure

For distributed execution:

healthy partitions
+
failed partitions

must be represented separately.

Recovery may:

isolate failed partitions;

recompute affected partitions;

re-route;

re-coordinate;

restart affected work.


Unrelated successful work SHOULD be preserved when semantically valid.


---

145. Compensation

Compensation is not generic undo.

A compensation action MUST have a formally understood semantic effect.

If no valid compensation exists:

do not fabricate one

Escalate instead.


---

146. Recovery Transaction Boundary

Recovery SHOULD define explicit transaction boundaries around:

plan activation
resource reservation
execution modification
result acceptance

A failure between these stages MUST leave a recoverable state.


---

147. Two-Phase Recovery

For high-risk operations, the architecture SHOULD support:

Prepare
    ↓
Validate
    ↓
Commit

For example:

prepare migration
→ validate target
→ commit migration

This reduces partial state changes.


---

148. Recovery Atomicity

An action MUST declare whether it is:

atomic
partially atomic
non-atomic

Non-atomic actions require explicit intermediate-state handling.


---

149. Recovery Compensation for Partial Actions

If a non-atomic action fails halfway:

detect intermediate state
→ determine safe completion or rollback
→ verify

Do not assume the action either fully succeeded or fully failed.


---

150. Recovery Journal

A production implementation SHOULD maintain an append-only recovery journal containing:

observation
state transition
plan
authorization
action start
action completion
action failure
verification
acceptance

The journal supports:

debugging;

replay;

auditing;

incident analysis.



---

151. Journal Integrity

The journal SHOULD support integrity protection and sequence validation.

Missing or reordered records MUST be detectable.


---

152. Recovery History

history/recovery.rs MUST retain enough information to determine:

which strategy was used
why it was selected
whether it succeeded
what it cost
what verification found


---

153. Learning From Recovery

Learning MAY use:

successful recovery
failed recovery
cost
latency
hardware condition
fault type

Only verified outcomes SHOULD become trusted training feedback.


---

154. Learning Safety

A learned strategy MUST still pass:

capability validation
policy validation
security validation
semantic validation
verification


---

155. Recovery Strategy Versioning

Every recovery strategy MUST have:

strategy identity
version
compatibility
configuration
provenance

A plan MUST record which strategy version produced it.


---

156. Strategy Upgrade

When a strategy implementation changes:

new version

must not silently reinterpret old recovery plans.

Old plans SHOULD be invalidated if their semantics changed.


---

157. Recovery Serialization

Serialized recovery objects MUST contain enough version information to determine compatibility.

At minimum:

schema version
object version
strategy version
IR version
capability schema version

where applicable.


---

158. Recovery Deserialization

Untrusted serialized data MUST be validated before use.

Malformed or incompatible data MUST produce an error.

It MUST NOT cause undefined recovery behavior.


---

159. No Implicit Defaults

Production recovery MUST avoid dangerous hidden defaults.

Examples of unacceptable implicit assumptions:

retry three times
use first backend
use physical qubit zero
assume healthy if telemetry missing
assume checkpoint valid
assume migration safe
assume result complete

Defaults MAY exist only when explicitly defined as safe policy defaults.


---

160. Program Transparency

Normal Zamani programs SHOULD NOT need to know the physical recovery process.

A program should describe:

logical computation

rather than:

physical recovery strategy

unless advanced control is explicitly requested.


---

161. Write Once, Run Everywhere

The recovery architecture must allow:

same Zamani program

to execute against:

small simulator
small QPU
large QPU
fault-tolerant QPU
distributed QPU
future quantum architecture

without changing the semantic program merely because hardware scale changes.


---

162. No Hardware-Specific Core Logic

Forbidden inside core resilience:

if IBM
if Rigetti
if IonQ
if Quantinuum
if physical_qubit == ...

Provider-specific behavior belongs in hardware adapters.


---

163. Resource Discovery

The recovery engine MUST obtain resource information through discovery/capability interfaces.

It MUST NOT encode assumptions about:

number of qubits;

topology;

gate set;

timing;

control channels;

measurement;

reset;

QEC;

backend limits.



---

164. Resource Identity

Resources SHOULD be represented by stable identities.

Possible resources:

logical qubit
physical qubit
coupling
gate resource
device
backend
execution slot
memory
network path

The identity system must remain extensible.


---

165. Qubit Identity

When canonical IR qubits are referenced:

quantum::ir::qubit::QubitId

MUST be used.

Physical hardware identifiers MAY be distinct hardware-layer identities.

The recovery model MUST not confuse logical and physical identity.


---

166. Logical/Physical Separation

Example:

logical qubit L
    ↓
QEC encoding
    ↓
physical qubits P1..Pn

Failure of one physical qubit does not automatically mean the logical qubit failed.

The QEC subsystem determines logical impact.

Resilience consumes that determination.


---

167. Recovery of Correlated Faults

Correlated faults MUST be treated as potentially larger than the sum of independent faults.

The planner SHOULD use:

fault correlation
fault domain
topology
history
QEC information

to determine recovery scope.


---

168. Recovery From Leakage

Leakage MUST be represented through canonical fault semantics.

Possible recovery:

reset
replace resource
QEC response
reroute
restart

depends on capabilities and policy.


---

169. Recovery From Loss

Loss/erasure recovery MUST use the QEC/hardware semantics appropriate to the platform.

Do not implement platform-specific physics in resilience.


---

170. Recovery From Measurement Failure

Possible actions:

repeat measurement
mitigate readout
recalibrate
rerun affected execution
switch resource

The selected action depends on policy and evidence.


---

171. Recovery From Gate Failure

Possible actions:

retry
reroute
recompile
reoptimize
change implementation
migrate

Verification remains mandatory.


---

172. Recovery From Drift

Drift may require:

refresh calibration
refresh noise model
rebenchmark
reschedule
recompile
reroute

The system MUST NOT blindly reuse stale calibration or noise information.


---

173. Recovery From Timeout

A timeout MUST be classified before retry.

Possibilities:

queue delay
backend execution
network
classical processing
provider outage
unknown

Each may require different recovery.


---

174. Recovery From Unknown Result

An unknown result is not a failed result and not a successful result.

State:

RESULT_UNKNOWN

until resolved.


---

175. Recovery From Duplicate Result

Duplicate results MUST be correlated using execution identity.

Do not accidentally merge distinct executions.


---

176. Recovery From Conflicting Results

Conflicting results MUST be retained as evidence.

The system must determine:

which execution
which configuration
which calibration
which result version

before acceptance.


---

177. Recovery and Statistical Confidence

Quantum results often involve sampling.

Verification MUST consider:

sample count;

statistical uncertainty;

mitigation overhead;

confidence requirements;

requested precision.


Statistical uncertainty MUST NOT be confused with hardware failure.


---

178. Recovery and Mitigation Failure

If mitigation itself fails:

record mitigation failure
→ determine whether unmitigated execution is acceptable
→ try alternate mitigation
→ replan
→ escalate

Do not automatically treat mitigation failure as program failure.


---

179. Recovery and QEC Threshold Breach

If logical error behavior exceeds the permitted policy boundary:

contain affected logical resources
→ evaluate QEC adaptation
→ evaluate migration
→ evaluate restart
→ escalate

Threshold values MUST come from QEC/policy, not hard-coded resilience constants.


---

180. Recovery and Resource Exhaustion

Resource exhaustion MUST be represented explicitly.

Examples:

memory
CPU
QPU time
shots
qubits
control channels
network
storage
budget

Recovery may:

reduce concurrency;

defer work;

migrate;

partition;

reschedule;

degrade where allowed.



---

181. Recovery Under Classical Resource Failure

The recovery engine itself may fail because classical resources are exhausted.

The architecture SHOULD therefore:

bound memory;

stream events;

avoid unbounded queues;

use incremental processing;

preserve safety-critical state.



---

182. Recovery Under Recovery Engine Failure

If resilience itself fails:

preserve execution evidence
preserve state
avoid unsafe continuation
surface failure

The system MUST NOT silently continue as though resilience succeeded.


---

183. Recovery Watchdog

A higher-level runtime MAY monitor recovery progress.

A watchdog SHOULD detect:

stuck recovery;

stalled verification;

expired lease;

unbounded retry;

missing state transition.



---

184. Watchdog Independence

The watchdog SHOULD NOT depend exclusively on the recovery state it is monitoring.

Otherwise a corrupted recovery state may also disable the watchdog.


---

185. Recovery Timeout

Recovery operations MAY have policy-defined deadlines.

If exceeded:

stop or contain safely
→ record timeout
→ replan or escalate

No hard-coded universal timeout.


---

186. Recovery Progress

Long-running recovery SHOULD expose progress.

Progress MAY include:

current phase
completed actions
remaining actions
resource status
verification status

Progress MUST NOT imply success.


---

187. Recovery Cancellation

Cancellation MUST be explicit.

A cancellation request must specify:

who
why
scope
authority

Cancellation MUST preserve provenance.


---

188. Safe Cancellation

If an action cannot be safely cancelled immediately:

mark cancellation requested
→ reach safe boundary
→ stop
→ verify state


---

189. Recovery Priority Inversion

Large low-priority recovery tasks MUST NOT indefinitely block critical incidents.

Scheduling of recovery itself should respect policy priorities.


---

190. Starvation Prevention

A continuous stream of small incidents MUST NOT permanently starve larger incidents.

The recovery scheduler SHOULD support fairness policies.


---

191. Recovery Admission Control

Before beginning a recovery operation, the system SHOULD ask:

Do we have enough resources to recover safely?

If not:

queue
degrade
migrate
or escalate


---

192. Recovery Plan Ranking

Candidate plans SHOULD be ranked using:

feasibility
safety
semantic risk
expected success
cost
latency
blast radius
confidence
policy preference

Safety and semantic constraints MUST be hard constraints, not merely scores.


---

193. Hard Versus Soft Constraints

Hard constraint:

must preserve semantics

Soft objective:

prefer lower latency

A soft objective MUST NOT override a hard constraint.


---

194. Recovery Plan Explainability

A selected recovery plan SHOULD be explainable.

The provenance should identify:

incident
diagnosis
constraints
candidate plans
why selected
why alternatives rejected


---

195. Recovery Auditability

Production deployments SHOULD be able to reconstruct:

what happened
why it happened
what was observed
what was decided
what changed
what result was obtained
why result was accepted


---

196. Recovery and Privacy

Telemetry and provenance MUST avoid exposing secrets unnecessarily.

Credentials, authentication material, private program data, and sensitive backend information MUST not be stored in ordinary incident logs unless explicitly required and protected.


---

197. Recovery and Secret Handling

Recovery objects MUST reference credentials rather than embedding them.

Secrets SHOULD remain under the authentication/credential subsystem.


---

198. Recovery and Multi-Tenancy

If Zamani executes workloads for multiple tenants, recovery MUST preserve tenant isolation.

A recovery action for tenant A MUST NOT:

access tenant B's data;

consume unauthorized tenant B resources;

expose tenant B telemetry.



---

199. Recovery and Fair Resource Allocation

Distributed systems SHOULD apply policy-based resource fairness.

Recovery MUST NOT automatically consume all available resources without authorization.


---

200. Recovery API Contract

api/controller.rs SHOULD expose a high-level lifecycle equivalent to:

submit
observe
recover
verify
result

The exact Rust API is implementation-defined, but the semantic contract is fixed by this document.


---

201. Request Contract

api/request.rs MUST contain sufficient immutable information to determine:

program identity
execution requirements
policy
resource constraints
resilience requirements
determinism requirements
security context

It MUST NOT require physical qubit identifiers for ordinary logical programs.


---

202. Response Contract

api/response.rs MUST expose:

execution identity
status
result
verification
provenance
degradation
recovery history
diagnostic information

Sensitive information MUST be filtered according to policy.


---

203. Context Contract

api/context.rs provides access to external subsystem contracts.

It SHOULD reference interfaces rather than concrete providers.


---

204. Controller Contract

api/controller.rs coordinates:

detection
diagnosis
policy
planning
adaptation
recovery
verification

It MUST NOT contain provider-specific hardware logic.


---

205. Recovery Executor Contract

recovery/recoverer.rs receives an already validated plan.

It MUST:

1. verify plan freshness;


2. acquire ownership;


3. execute actions;


4. record state;


5. emit telemetry;


6. produce execution evidence;


7. hand results to verification.




---

206. Executor Failure

If recovery execution fails:

record action failure
→ preserve partial state
→ determine safe continuation
→ replan or escalate

Never erase partial execution state.


---

207. Verification Failure

If verification fails:

REJECT

or:

REPLAN

or:

ESCALATE

according to policy.

It MUST NOT become:

ACCEPT

automatically.


---

208. Verification Uncertainty

If verification cannot establish correctness:

UNVERIFIED

must be represented.

Unverified results MUST NOT be silently reported as verified.


---

209. Recovery Result Classes

The implementation SHOULD distinguish:

VerifiedSuccess
VerifiedDegraded
Unverified
Failed
Rejected
Unknown

This prevents binary success/failure semantics from hiding important quantum execution states.


---

210. Recovery Completion

A recovery operation is complete only when:

execution outcome known
+
state reconciled
+
provenance recorded
+
verification completed
+
ownership released


---

211. Ownership Release

After recovery:

release lease
release reservation
update resource state
persist outcome

A failed cleanup MUST be recorded.


---

212. Recovery Reconciliation

After any recovery action, the system SHOULD reconcile:

expected state
versus
observed state

This is particularly important after partial actions.


---

213. State Versioning

State SHOULD carry monotonically increasing versions or equivalent concurrency controls.

Example:

state version 41
plan created against 41
execution changes state to 42
plan is now stale


---

214. Compare-and-Swap Semantics

Where supported, state updates SHOULD use version checks.

Conceptually:

update only if current_version == expected_version

This prevents lost updates.


---

215. Recovery Under Concurrent Faults

If another fault appears while recovery is running:

new incident

must be correlated with the active recovery.

The planner determines whether:

continue
cancel
replan
escalate


---

216. Recovery Under Cascading Failure

If recovery causes another failure:

action
→ secondary fault

the secondary fault MUST be linked to the original recovery attempt.

This allows the planner to identify harmful strategies.


---

217. Recovery Strategy Blacklisting

Repeatedly harmful strategies MAY be temporarily avoided according to history/policy.

This must be dynamic.

Do not hard-code permanent blacklists.


---

218. Recovery Strategy Circuit Breaker

A strategy that repeatedly fails MAY be temporarily disabled.

The registry SHOULD expose:

enabled
disabled
quarantined

strategy states.


---

219. Recovery Quarantine

A resource may be quarantined when:

confidence of failure is high
continued use is unsafe
policy permits isolation

Quarantine status MUST be explicit.


---

220. Resource Reintroduction

A quarantined resource MUST NOT automatically return to service.

Reintroduction requires:

health evidence
capability validation
possibly recalibration
possibly benchmarking
policy authorization


---

221. Recovery From False Positive

If a healthy resource was incorrectly quarantined:

incident recorded
diagnosis updated
resource validated

Learning/history may use this outcome.


---

222. Recovery From False Negative

If a resource was incorrectly considered healthy:

contain
investigate
expand incident scope if required

The system MUST account for possible propagation.


---

223. Recovery and Calibration Refresh

Calibration refresh SHOULD be represented as an adaptation action rather than hidden side effect.

This ensures provenance.


---

224. Recovery and Benchmarking Refresh

Benchmarking MAY be requested to establish whether a resource is safe to reintroduce.


---

225. Recovery and Noise Learning

Noise-learning data MUST include freshness and provenance.

If stale:

invalidate

and relearn where policy permits.


---

226. Recovery and Mitigation Ensembles

Mitigation strategies may execute multiple related circuits.

Recovery MUST track:

parent execution
child executions
ensemble relationship
aggregation method

This prevents partial mitigation results from being mistaken for final results.


---

227. Recovery and Sampling Overhead

When mitigation expands the execution workload, the planner MUST account for:

additional shots
additional circuits
additional execution time
additional memory


---

228. Recovery and Statistical Guarantees

If a recovery strategy changes sampling requirements, verification MUST recalculate confidence.

Do not reuse old confidence values blindly.


---

229. Recovery and Cost Constraints

A recovery that technically succeeds but exceeds an explicit cost policy MUST NOT automatically be accepted.


---

230. Recovery and User Intent

User-declared semantic requirements are higher priority than automatic optimization.

For example:

strict correctness

must not be silently changed into:

best effort


---

231. Policy Relaxation

Policy relaxation MUST be explicit.

Example:

strict
→ degraded permitted

requires authorization.


---

232. Recovery Escalation

Escalation occurs when:

no safe plan exists;

diagnosis is insufficient;

resources are unavailable;

verification fails;

security is uncertain;

budgets are exhausted;

recovery loops are detected;

distributed ownership is inconsistent.



---

233. Escalation Information

An escalation MUST contain:

incident
current state
diagnosis
evidence
attempted plans
failed actions
remaining options
reason automatic recovery stopped


---

234. Terminal Failure

Terminal failure means:

automatic recovery cannot continue safely

It does NOT necessarily mean:

hardware permanently destroyed


---

235. Recovery Metrics

Production telemetry SHOULD expose:

recovery_attempts
recovery_successes
recovery_failures
recovery_latency
verification_failures
plan_staleness
retry_rate
recovery_loop_rate
migration_rate
degradation_rate
escalation_rate
resource_quarantine_rate

Metrics MUST be dynamically aggregatable.


---

236. Recovery SLOs

Deployments MAY define policy-level objectives for:

availability
recovery time
verification latency
successful recovery percentage

These are deployment policies, not architectural constants.


---

237. Recovery Testing

Every recovery mechanism MUST be tested independently.

Required categories:

unit
property
integration
fault injection
simulation
end-to-end
concurrency
deterministic replay
scalability
security
serialization


---

238. Required Fault Injection

At minimum test:

physical qubit failure
logical resource degradation
gate failure
measurement failure
readout degradation
leakage
loss
erasure
correlated fault
crosstalk
calibration drift
backend outage
network outage
timeout
partial result
duplicate submission
unknown submission
checkpoint corruption
stale plan
policy conflict
resource exhaustion
telemetry corruption
malicious telemetry
strategy failure
verification failure


---

239. Recovery Loop Test

Inject:

fault
→ recovery
→ identical fault

Verify:

loop detected

and:

automatic recovery eventually stops

according to policy.


---

240. Stale Plan Test

Create:

plan at state N

then change state to:

N+1

Verify the old plan is rejected.


---

241. Concurrent Recovery Test

Create two plans affecting the same resource.

Verify:

only one obtains ownership

and the other is invalidated/replanned.


---

242. Partial Execution Test

Force failure after a subset of operations.

Verify:

partial
!=
successful

and recovery begins from a valid boundary.


---

243. Duplicate Submission Test

Cause submission uncertainty.

Verify:

no unsafe duplicate execution

unless policy explicitly allows it.


---

244. Checkpoint Corruption Test

Corrupt checkpoint data.

Verify:

integrity failure
→ checkpoint rejected


---

245. Malicious Telemetry Test

Inject forged health information.

Verify:

trust validation
→ containment
→ rejection/escalation


---

246. Deterministic Replay Test

Record:

IR
capabilities
telemetry
policy
history
seed
strategy versions

Replay.

Verify identical planner output in deterministic mode.


---

247. Scalability Test

Tests MUST generate arbitrary resource counts.

The tests MUST NOT rely only on fixed machine sizes.

They SHOULD exercise:

single-resource
small
medium
large
very-large
distributed

through parameterized/generated workloads.


---

248. Memory Scalability

The implementation MUST avoid retaining all historical telemetry indefinitely in memory.

Use:

streaming;

bounded windows;

persistent storage;

aggregation;

pagination.



---

249. Incident Scalability

Incident correlation MUST support partitioning.

A global incident MUST NOT require loading every resource into one process.


---

250. Recovery Scalability

Recovery actions SHOULD operate on affected scopes rather than entire systems whenever possible.


---

251. No Artificial Infinity

The implementation MUST NOT claim literal infinite execution.

The correct contract is:

> No finite scalability ceiling is imposed by the resilience architecture itself.



Actual execution remains bounded by physical and computational resources.


---

252. File-Level Completion Contracts

The following files are the normative ownership boundaries.

state/recovery.rs

Must completely define:

recovery states;

allowed transitions;

state invariants;

versioning;

transition validation.


No other file may invent a competing recovery state machine.


---

recovery/recoverer.rs

Must completely define:

recovery orchestration;

plan execution;

action sequencing;

ownership checks;

state updates;

telemetry;

error propagation.


It must consume the state machine rather than redefine it.


---

planning/plan.rs

Must completely define:

immutable plan structure;

plan identity;

versions;

preconditions;

actions;

expected effects;

verification requirements.


It must not execute plans.


---

planning/planner.rs

Must completely define:

candidate generation;

feasibility integration;

policy integration;

ranking integration;

stale-state protection.


It must not directly execute hardware operations.


---

policy/policy.rs

Must completely define:

policy evaluation contract;

allowed actions;

constraints;

objectives;

budgets;

escalation rules.


It must not perform recovery.


---

verification/verifier.rs

Must completely define:

verification pipeline;

invariant checking;

semantic validation;

result validation;

provenance validation.


It must not silently accept unverified output.


---

verification/acceptance.rs

Must completely define:

acceptance states;

acceptance criteria;

degraded acceptance;

rejection;

escalation.



---

checkpoint/checkpoint.rs

Must completely define:

checkpoint lifecycle;

checkpoint identity;

checkpoint validation;

restore eligibility.


It must not assume arbitrary quantum state can be serialized.


---

model/resource.rs

Must completely define resilience resource references.

It must use canonical repository resource identities where available.


---

model/fault.rs

Must reference canonical ZQN fault semantics.

It must not create a second quantum fault ontology.


---

253. Dependency Direction

The preferred dependency direction is:

model
  ↑
telemetry/detection
  ↑
diagnosis
  ↑
policy
  ↑
planning
  ↑
adaptation/recovery/mitigation
  ↑
verification
  ↑
api/controller

Supporting systems:

state
checkpoint
history
serialization
coordination
registry

may interact through stable contracts.

Avoid cycles.


---

254. Dependency Inversion

Core resilience interfaces SHOULD depend on abstractions rather than concrete implementations.

For example:

Planner
    → CapabilityProvider

not:

Planner
    → IBMBackend


---

255. No Circular Dependency

The following MUST NOT occur:

hardware → resilience → hardware implementation

or:

routing → resilience → routing implementation

Instead:

hardware contract
       ↑
resilience
       ↓
routing contract


---

256. Root Module

resilience/mod.rs MUST remain thin.

It should:

declare modules;

re-export stable public types;

expose public interfaces.


It MUST NOT contain recovery algorithms.


---

257. Public API Stability

Public recovery types SHOULD be intentionally small.

Internal implementation details should remain private.


---

258. Versioning

Breaking changes to public recovery contracts require a version change.

The serialization layer MUST distinguish schema compatibility from implementation compatibility.


---

259. Backward Compatibility

A newer resilience implementation SHOULD be able to read compatible historical recovery records.

It MUST reject incompatible data safely.


---

260. Forward Compatibility

Unknown serialized fields SHOULD be handled according to repository-wide serialization policy.

Unknown semantic values MUST NOT be silently interpreted as safe.


---

261. Documentation Requirements

The implementation is incomplete until these documents agree:

README.md
ARCHITECTURE.md
DESIGN.md
SECURITY.md
SCALABILITY.md
COMPATIBILITY.md
DETERMINISM.md
FAILURE_MODES.md
RECOVERY_MODEL.md
OBSERVABILITY.md

Contradictory documents indicate an incomplete contract.


---

262. Production Readiness Gate

Recovery is production-ready only when:

every recovery state is defined;

every transition is validated;

plans are immutable;

stale plans are rejected;

retries are policy-driven;

partial execution is represented;

duplicate submissions are controlled;

checkpoint validity is verified;

migration is capability-driven;

recovery is semantically verified;

provenance is complete;

deterministic replay works;

distributed ownership is protected;

recovery loops are detected;

resource exhaustion is bounded;

unknown failures are safely escalated;

telemetry is treated as potentially untrusted;

no hard-coded machine limits exist;

no provider-specific core logic exists;

no unsafe Rust is used;

canonical quantum::ir::qubit::QubitId is used where appropriate;

canonical ZQN fault semantics are reused;

routing/scheduling/optimization/QEC remain independently owned;

tests cover fault injection;

scalability tests are parameterized;

serialization is versioned;

security controls are enforced.



---

263. Final Recovery Invariant

The most important rule in the entire subsystem is:

A recovery action may improve availability,
but only verification can establish correctness.

Therefore:

Failure
   ↓
Contain
   ↓
Diagnose
   ↓
Policy
   ↓
Plan
   ↓
Validate
   ↓
Adapt / Recover
   ↓
Verify
   ↓
Accept

never:

Failure
   ↓
Retry
   ↓
Assume success


---

264. Final Architecture

The complete recovery architecture is:

Zamani Program
                          |
                          v
                  Canonical Quantum IR
                          |
                          v
                    Execution Fabric
                          |
          +---------------+---------------+
          |               |               |
       Routing        Scheduling      Optimization
          |               |               |
          +---------------+---------------+
                          |
                          v
                       QEC/ZQN
                          |
                          v
                     Hardware HAL
                          |
                          v
                       Execute
                          |
              +-----------+-----------+
              |                       |
          Telemetry                 Result
              |                       |
              v                       v
          Detection              Verification
              |
              v
          Diagnosis
              |
              v
            Policy
              |
              v
           Planning
              |
       +------+------+
       |             |
   Adaptation     Recovery
       |             |
       +------+------+
              |
              v
           Execute
              |
              v
          Verification
              |
        +-----+------+
        |            |
      Accept       Replan
        |            |
        v            |
     Result <--------+


---

265. Fundamental Zamani Contract

The final contract for quantum::resilience is:

> A Zamani quantum program describes the computation once. The resilience system dynamically determines how that computation can continue safely as resources, hardware, topology, calibration, noise, QEC state, execution conditions, and backend availability change.



The program remains logically stable.

The physical realization may change.

The recovery strategy may change.

The backend may change.

The number of available resources may change.

The topology may change.

The QEC configuration may change.

The execution schedule may change.

The optimization may change.

But:

semantic intent
+
verified correctness
+
provenance
+
policy
+
security

remain the controlling invariants.


---

266. Final Rule

The implementation MUST never optimize for:

"make the job appear successful"

It MUST optimize for:

"produce the best safely verifiable result permitted
by the program, policy, available resources and evidence."

That is the production recovery contract for Zamani.

This model deliberately treats recovery as **a verified state machine rather than a retry subsystem**. It also keeps mitigation separate from recovery, which matches current quantum platforms where techniques such as dynamical decoupling, twirling, TREX, ZNE and PEC have different purposes and overheads. 1

The next implementation should therefore start with the **independent contract files**—especially `errors/*`, `model/*`, `policy/*`, and `state/recovery.rs`—before implementing `recovery/recoverer.rs`. That ordering prevents later files from forcing redesign of the recovery semantics.