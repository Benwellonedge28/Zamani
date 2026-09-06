Worked for 12s

I checked the current Zamani repository structure and the existing resilience documentation/contracts, including the canonical quantum::ir::qubit identity boundary and the existing scalability/design requirements. The repository explicitly establishes QubitId and PhysicalQubitId under quantum::ir::qubit, rather than allowing resilience to define competing identities.  The existing resilience architecture also treats FAILURE_MODES.md as a normative companion to ARCHITECTURE.md, DESIGN.md, SCALABILITY.md, COMPATIBILITY.md, and the implementation modules.

I would make FAILURE_MODES.md normative, not merely a list of errors: every failure class below defines detection, evidence, diagnosis, containment, recovery/adaptation, verification, escalation, provenance, scalability behavior, and integration ownership. This also reflects current quantum execution practice: mitigation techniques have different overheads and are not universally beneficial, and resilience decisions therefore need capability/policy/verification gates rather than blindly applying a fixed technique. 

Zamani Quantum Resilience — Failure Modes Specification

Path: "src/quantum/resilience/FAILURE_MODES.md"
Status: Normative
Subsystem: "crate::quantum::resilience"
Language: Rust
Required Rust: Rust 1.97 / Rust 1.97.1
Edition: Rust 2021
Safety: "unsafe" forbidden
Canonical qubit identity: "crate::quantum::ir::qubit"
Primary purpose: Define every failure class that the resilience subsystem must detect, classify, contain, adapt to, recover from, verify, or safely escalate.

---

1. Purpose

This document defines the normative failure model for Zamani's quantum resilience subsystem.

It answers:

- What can fail?
- Where can it fail?
- How is the failure represented?
- How is it detected?
- How is it diagnosed?
- What evidence must be retained?
- What actions are permitted?
- What actions are forbidden?
- When can execution continue?
- When must execution be replanned?
- When must a resource be quarantined?
- When must execution migrate?
- When must execution stop?
- How is a recovered result verified?
- How is the failure recorded?
- How does the system remain scalable without hard-coded machine limits?

The governing invariant is:

«A failure must never be hidden merely because the system found a way to continue execution.»

A successful recovery is itself an observable event and must remain part of execution provenance.

---

2. Scope

"quantum::resilience" must account for failures across the complete execution lifecycle:

Zamani source
     |
     v
Frontend
     |
     v
Canonical Quantum IR
     |
     +--> Optimization
     |
     +--> QEC
     |
     +--> Routing
     |
     +--> Scheduling
     |
     v
Hardware / Simulator / Emulator
     |
     v
Execution
     |
     +--> Telemetry
     +--> Measurements
     +--> QEC signals
     +--> Backend status
     |
     v
Resilience
     |
     +--> Detection
     +--> Diagnosis
     +--> Policy
     +--> Planning
     +--> Adaptation
     +--> Recovery
     +--> Mitigation
     +--> Verification
     |
     v
Accepted / Degraded / Retried / Replanned / Escalated / Rejected

Failure modes therefore include, but are not limited to:

1. semantic failures;
2. IR failures;
3. compilation failures;
4. optimization failures;
5. routing failures;
6. scheduling failures;
7. hardware failures;
8. qubit failures;
9. gate failures;
10. measurement failures;
11. leakage;
12. loss;
13. erasure;
14. correlated faults;
15. crosstalk;
16. decoherence;
17. calibration drift;
18. topology changes;
19. QEC failures;
20. decoder failures;
21. logical failures;
22. mitigation failures;
23. simulator failures;
24. execution failures;
25. timeout failures;
26. resource exhaustion;
27. memory exhaustion;
28. classical-control failures;
29. communication failures;
30. distributed-system failures;
31. checkpoint failures;
32. serialization failures;
33. provenance failures;
34. telemetry failures;
35. detector failures;
36. diagnosis failures;
37. planning failures;
38. recovery failures;
39. verification failures;
40. security failures;
41. compatibility failures;
42. determinism failures;
43. plugin/extension failures;
44. unknown failures;
45. compound failures;
46. cascading failures;
47. correlated multi-resource failures.

---

3. Architectural ownership

Failure ownership must remain separated.

Failure domain| Authoritative subsystem| Resilience responsibility
Program semantics| "quantum::ir"| Detect consequences and prevent semantic corruption
Qubit identity| "quantum::ir::qubit"| Consume canonical identities
Noise/fault semantics| "quantum::zqn"| Consume normalized fault information
QEC| "quantum::error_correction"| Consume health/syndrome/logical-error signals
Routing| "quantum::routing"| Request rerouting
Scheduling| "quantum::scheduling"| Request rescheduling
Optimization| "quantum::optimization"| Request reoptimization
Hardware| "quantum::hardware"| Consume capabilities/status/telemetry
Simulation| quantum simulation subsystem| Use for validation/fault injection
Benchmarking| "quantum::benchmarking"| Consume historical characterization
Runtime/execution| runtime| Request retry/restart/resume/migration
Resilience| "quantum::resilience"| Detect, diagnose, decide, coordinate, verify

Resilience must not become a second implementation of another subsystem.

---

4. Canonical identity requirement

All resilience code that needs quantum identities must use:

crate::quantum::ir::qubit::QubitId
crate::quantum::ir::qubit::PhysicalQubitId

where those identities are semantically appropriate.

The canonical repository boundary explicitly places these identities under "quantum::ir::qubit".

Resilience must never create:

type QubitId = u64;

or:

struct ResilienceQubitId(...);

as a competing canonical identity.

A resource may have a different identity if it is genuinely a different domain:

BackendId
DeviceId
ExecutionId
IncidentId
CheckpointId
ControlChannelId

but those must not masquerade as qubit identities.

Logical and physical identities must remain distinct.

For example:

logical QubitId
      |
      | routing
      v
physical PhysicalQubitId

The numeric/index representation of either identity must never be assumed to equal the other.

---

5. Definition of a failure

A failure is:

«An observed or inferred condition that prevents, threatens, invalidates, degrades, delays, or makes unverifiable an execution relative to its declared semantic, policy, capability, security, or resource requirements.»

A failure does not necessarily mean execution must stop.

Examples:

transient backend timeout
    -> retry may be valid

physical qubit degraded
    -> remapping may be valid

readout noise increased
    -> mitigation may be valid

logical error detected
    -> QEC/recovery may be valid

semantic verification failed
    -> acceptance is forbidden

---

6. Failure lifecycle

Every failure follows the same conceptual lifecycle:

OBSERVE
   |
   v
NORMALIZE
   |
   v
DETECT
   |
   v
CORRELATE
   |
   v
DIAGNOSE
   |
   v
CLASSIFY
   |
   v
ASSESS IMPACT
   |
   v
CHECK POLICY
   |
   v
PLAN
   |
   v
VALIDATE PLAN
   |
   v
CONTAIN
   |
   v
ADAPT / RECOVER / MITIGATE
   |
   v
VERIFY
   |
   +----> ACCEPT
   |
   +----> DEGRADED_ACCEPT
   |
   +----> REPLAN
   |
   +----> RETRY
   |
   +----> ESCALATE
   |
   +----> REJECT

No failure may bypass verification merely because recovery succeeded operationally.

---

7. Failure classification

"errors/classification.rs" must provide stable classification semantics.

At minimum, the system must distinguish:

Transient
Persistent
Recoverable
NonRecoverable
Unknown
SafetyCritical
SemanticRisk
SecurityCritical
ResourceLimited
CapabilityMismatch
CompatibilityFailure
DeterminismFailure

Classification is not identical to severity.

For example:

Transient + Critical
Persistent + Degraded
Recoverable + Major
SemanticRisk + Critical

are all valid combinations.

---

8. Severity

"model/severity.rs" owns severity.

Recommended semantic levels:

Informational
Degraded
Major
Critical
Fatal

Severity must not directly determine the recovery action.

Instead:

severity
+
classification
+
policy
+
capabilities
+
impact
+
confidence
=
candidate actions

---

9. Confidence

Failures may be uncertain.

Every inferred diagnosis must preserve confidence.

Examples:

observed hardware timeout
confidence = high

suspected calibration drift
confidence = medium

suspected correlated fault
confidence = low

The planner must not treat a low-confidence hypothesis as an established fact unless policy explicitly permits the action.

---

10. Failure evidence

Every normalized failure should preserve sufficient evidence for:

- diagnosis;
- audit;
- replay;
- verification;
- debugging;
- deterministic reproduction where possible.

Evidence may include:

event identity
timestamp
source
resource identity
logical identity
physical identity
execution identity
operation identity
measurement context
calibration snapshot/reference
capability snapshot/reference
fault model/reference
telemetry
error code
backend status
QEC signal
policy context
previous actions

Evidence retention must itself obey resource policies.

The system must not require unlimited in-memory history.

---

11. Failure domains

The following sections define the production failure taxonomy.

---

12. Program/input failures

12.1 Invalid program

Description

The input program violates the language or quantum IR contract.

Detection

Owned by:

frontend
quantum::ir
compiler

Resilience behavior

Resilience must not attempt to recover an invalid program.

Result:

REJECT

Integration

"api/request.rs" must reject invalid execution requests before execution.

"errors/error.rs" must preserve the originating compiler/IR error.

---

12.2 Invalid quantum IR

Examples:

invalid qubit reference
invalid operation
invalid operand
invalid measurement target
invalid control dependency
invalid register reference

Recovery

None unless a formally defined compiler repair exists.

Resilience must not silently mutate invalid IR.

Result:

REJECT

---

12.3 Semantic ambiguity

If the system cannot determine what a program means, execution must not continue.

Result:

REJECT

---

13. Qubit identity failures

13.1 Unknown logical qubit

A referenced "QubitId" does not exist in the canonical IR/program context.

Action

Reject the affected request.

---

13.2 Unknown physical qubit

A physical resource reported by hardware is not present in the current target capability snapshot.

Action

Treat the observation as stale or incompatible.

Do not silently map it to another qubit.

---

13.3 Logical/physical identity collision

If a system accidentally treats:

QubitId

as:

PhysicalQubitId

the execution must fail closed.

This is a semantic-safety failure.

---

14. IR integrity failures

Examples:

- corrupted IR;
- inconsistent operation references;
- invalid gate operands;
- invalid qubit lifetimes;
- inconsistent mappings;
- stale transformation metadata.

Action

invalidate affected artifact
invalidate dependent plan
recompile/reconstruct where possible
otherwise reject

The original semantic IR must remain immutable evidence.

---

15. Compiler failures

15.1 Compilation failure

Compilation cannot produce a valid executable representation.

Recovery candidates

retry compilation
change optimization profile
change target
change routing constraints
change resource allocation
migrate backend

Only actions permitted by policy may be attempted.

---

15.2 Compiler timeout

A compiler operation exceeds its execution budget.

Possible actions

continue if deadline permits
cancel
use alternative compilation strategy
reduce optimization scope
compile affected region
migrate
escalate

No hard-coded timeout is allowed.

Timeout comes from the execution/resource policy.

---

15.3 Compiler resource exhaustion

Examples:

- memory;
- CPU;
- compilation workspace;
- intermediate representation size.

Recovery

Prefer:

incremental compilation
partitioning
alternative representation
lower optimization level
distributed compilation
migration

where semantics permit.

---

16. Optimization failures

16.1 Optimization timeout

The optimization stage cannot finish within its budget.

Recovery

Use policy-controlled:

reduce optimization scope
switch optimization profile
skip optional pass
compile affected region
continue with verified unoptimized representation

---

16.2 Optimization semantic mismatch

An optimization produces output that cannot be proven compatible with the input semantics.

Action

reject optimized artifact
restore previous valid artifact
record optimizer provenance

Never accept an optimization solely because it improves cost.

---

16.3 Fault-tolerance optimization failure

If a fault-tolerant transformation becomes invalid under current hardware/QEC capabilities:

invalidate transformation
replan against current capabilities

---

17. Routing failures

17.1 No valid physical mapping

The logical program cannot currently be mapped to available physical resources.

Recovery

Potential actions:

reroute
reallocate
quarantine fewer resources
change target
migrate
recompile
partition
defer
abort

The routing algorithm belongs to "quantum::routing".

---

17.2 Topology disconnected

A resource failure creates an unusable connectivity component.

Recovery

detect topology change
        |
        v
invalidate affected plan
        |
        v
request rerouting
        |
        v
request rescheduling
        |
        v
verify

---

17.3 Stale routing plan

A routing plan was valid when created but the hardware changed before execution.

Rule

A stale plan must never execute without revalidation.

old plan
   |
   v
capability validation
   |
   +--> valid -> execute
   |
   +--> invalid -> replan

---

18. Scheduling failures

18.1 Scheduling infeasibility

No valid schedule satisfies current:

- timing;
- resource;
- dependency;
- hardware;
- QEC;
- control constraints.

Recovery

Request:

rescheduling
rerouting
recompilation
resource change
migration

---

18.2 Schedule invalidated by hardware change

A calibration or timing change invalidates an existing schedule.

Action

Invalidate the schedule and revalidate/rebuild it.

The scheduling subsystem owns schedule generation.

---

18.3 Timing violation

An execution violates a required timing constraint.

Classification

CapabilityMismatch

or:

HardwareFailure

depending on evidence.

---

19. Hardware failures

The hardware HAL is the authoritative source for hardware status.

Resilience consumes the status and determines impact.

---

19.1 Device unavailable

Possible causes:

- maintenance;
- outage;
- provider failure;
- power/control failure;
- network isolation.

Recovery

retry if transient
switch device
switch backend
migrate
defer
escalate

---

19.2 Partial device degradation

Only part of the device is unavailable.

Rule

Prefer local recovery where semantics permit.

affected region
      |
      v
local remapping
      |
      v
local rerouting
      |
      v
local rescheduling

Global recovery remains available when necessary.

---

19.3 Complete device failure

The device can no longer safely execute the workload.

Recovery

quarantine device
invalidate affected execution plans
migrate if possible
restore from valid checkpoint if applicable
verify

---

20. Physical-qubit failures

20.1 Physical qubit unavailable

The physical resource cannot participate.

Action

Mark resource unavailable through the resilience state model.

Do not alter the canonical logical program.

Request:

remapping/rerouting

---

20.2 Physical qubit degraded

The qubit remains usable but exceeds a policy-defined degradation condition.

Action

Possible:

avoid resource
reroute
change QEC allocation
change mitigation
continue

Thresholds must come from policy/capabilities/telemetry.

---

20.3 Physical-qubit identity changed

If hardware reports a changed identity or incompatible device generation:

invalidate stale resource references
refresh capabilities
rebuild affected plans

Never assume identifiers remain interchangeable across devices.

---

21. Gate failures

21.1 Gate execution failure

A gate did not execute according to its contract.

Recovery

Depends on operation semantics.

Possible:

retry
recompile
reroute
replace with compatible implementation
restart from valid boundary

A failed quantum operation cannot automatically be "undone".

---

21.2 Gate fidelity degradation

Observed gate performance falls outside the allowed policy envelope.

Action

Potentially:

avoid gate/resource
change decomposition
reroute
change optimization
mitigate
migrate

---

21.3 Unsupported gate

The current target cannot execute a required operation.

Recovery

Request lowering/recompilation against the target capability set.

---

22. Measurement failures

Measurement errors must be distinguished from execution failures.

Possible categories:

measurement unavailable
measurement timeout
readout error
readout calibration stale
invalid measurement result
measurement-channel failure

Recovery

Depending on workload:

repeat measurement
readout mitigation
recalibration
resource migration
re-execution
reject

Readout mitigation must not be assumed to produce unbiased results. Current IBM documentation explicitly notes that techniques such as ZNE may improve results without guaranteeing unbiasedness.

Therefore verification remains mandatory.

---

23. Leakage failures

Leakage means a physical system leaves the intended computational subspace.

Source

Canonical fault/noise/QEC subsystem.

Resilience behavior

Possible:

reset
quarantine
QEC response
remap
recompile
restart from valid boundary

Resilience does not implement the physical leakage model.

---

24. Loss failures

Loss means a quantum resource becomes unavailable or the quantum information is lost.

Action

Depending on hardware/QEC capability:

QEC recovery
replacement resource
remapping
migration
restart
abort

No generic "undo state" assumption is permitted.

---

25. Erasure failures

An erasure is a distinct fault semantic and must not be silently converted into a generic hardware failure.

Integration

The canonical fault/noise layer provides the fault semantics.

Resilience consumes the erasure signal and determines:

recoverable?
correctable?
replaceable?
re-executable?

---

26. Correlated faults

A large number of related low-level faults may represent one underlying incident.

Example:

many qubits
    |
    +--> common control fault
    |
    +--> thermal event
    |
    +--> crosstalk
    |
    +--> calibration event

Rule

Do not independently launch a global recovery for every correlated event.

"model/incident.rs" must aggregate related observations while preserving evidence.

The existing scalability contract specifically requires storm aggregation rather than launching thousands of independent global recoveries.

---

27. Crosstalk

Crosstalk may affect operations that are not directly faulty.

Detection

Use:

telemetry
benchmarking
ZQN
calibration
hardware characterization

Recovery

Potential actions:

change schedule
avoid simultaneous operations
reroute
change gate decomposition
change pulse/control strategy
migrate

The resilience layer requests these actions; it does not implement pulse-level hardware control.

---

28. Decoherence failures

Examples:

- relaxation;
- dephasing;
- idle-time degradation;
- environment-induced errors.

Recovery

Possible:

reschedule
reduce idle periods
dynamical decoupling where supported
reroute
reduce depth
change QEC strategy

Dynamical decoupling is hardware/schedule dependent and can even hurt performance when its inserted pulses introduce additional imperfections, so it must be selected by capability/policy rather than universally enabled.

---

29. Calibration failures

29.1 Stale calibration

If execution relies on calibration information older than its permitted validity window:

invalidate dependent plan
refresh calibration
recompile/reschedule as necessary

---

29.2 Calibration drift

Detect statistically or through hardware telemetry.

Recovery

refresh calibration
update target capabilities
revalidate plan
reschedule
reroute
mitigate
migrate

---

29.3 Calibration inconsistency

If two sources report incompatible calibration state:

do not silently choose one

Require trusted-source resolution.

---

30. QEC failures

Resilience does not replace QEC.

---

30.1 Syndrome extraction failure

Possible:

retry syndrome cycle
change QEC strategy
quarantine resource
increase recovery scope

---

30.2 Decoder failure

If decoding fails or cannot produce a valid result:

change decoder if policy permits
retry with valid data
increase recovery scope
mark logical result uncertain
reject if verification cannot succeed

---

30.3 QEC resource exhaustion

If insufficient ancillas/resources exist:

change code
reduce code distance if policy permits
migrate
defer
abort

Any reduction in protection must be explicitly represented and verified.

---

30.4 Logical error detection

A logical error indicator is safety-critical.

The system must not hide it merely because physical execution completed.

Possible:

QEC recovery
re-execution
checkpoint recovery
migration
reject

---

31. Logical-resource degradation

A logical qubit may remain operational while its protection quality degrades.

The resilience system must represent:

healthy
degraded
unstable
unavailable

and propagate the state to planning.

---

32. Mitigation failures

Mitigation is not correction.

---

32.1 Readout mitigation failure

Possible:

recalibrate
repeat calibration
switch mitigation method
repeat execution
reject mitigated result

---

32.2 ZNE failure

ZNE may fail because:

- insufficient noise scaling;
- extrapolation instability;
- excessive sampling overhead;
- invalid model assumptions;
- target capability mismatch.

Current IBM documentation explicitly describes ZNE as having sampling overhead and notes that it is not guaranteed to produce an unbiased result.

Therefore:

mitigation result
      |
      v
verification
      |
      +--> valid
      |
      +--> uncertain
      |
      +--> rejected

---

32.3 Probabilistic error cancellation failure

Possible causes:

- insufficient noise characterization;
- excessive sampling overhead;
- unstable quasi-probability estimation.

Action

Do not silently return an unverified result.

---

32.4 Twirling failure

Possible causes:

- unsupported operations;
- unsupported randomization;
- control limitations;
- excessive overhead.

Action

Fallback only if explicitly allowed by policy.

---

32.5 Dynamical-decoupling failure

Possible causes:

- insufficient idle time;
- unsupported pulse/control model;
- pulse imperfections;
- timing incompatibility.

The strategy must be rejected if its prerequisites are not satisfied.

---

33. Simulator failures

Simulation is itself a resource and can fail.

Examples:

state-vector memory exhaustion
tensor-network contraction explosion
simulation timeout
unsupported operation
numerical instability
distributed simulation failure

Recovery

Potentially:

alternative representation
partitioning
approximate simulation if policy allows
distributed simulation
reduce verification scope
migrate to hardware
abort

Approximation must never be silently substituted for exact simulation when exactness is required.

---

34. Numerical failures

Examples:

NaN
infinity
overflow
underflow
loss of precision
ill-conditioned extrapolation
unstable statistical estimator

Rule

Numerically invalid results must never be accepted.

---

35. Execution failures

Execution may fail before, during, or after submission.

---

35.1 Submission failure

Possible:

retry
resubmit
switch backend

subject to idempotency.

---

35.2 Execution-start failure

The backend accepted the job but could not begin execution.

Possible:

retry
reschedule
migrate

---

35.3 Mid-execution failure

This is more dangerous.

The system must determine whether the quantum state/execution boundary is recoverable.

It must not assume retrying from the beginning is equivalent to resuming.

---

35.4 Post-execution result failure

Execution completed but the result could not be retrieved.

Possible:

query status
retrieve again
resume result acquisition

Do not automatically execute the circuit again if duplicate execution could change semantics/cost or produce an unintended additional side effect.

---

36. Timeout failures

Timeouts must be classified by stage:

frontend timeout
compilation timeout
optimization timeout
routing timeout
scheduling timeout
submission timeout
queue timeout
execution timeout
measurement timeout
result retrieval timeout
verification timeout
checkpoint timeout
telemetry timeout

Timeout values must come from explicit policy/resource constraints.

Never hard-code:

3 retries
30 seconds
60 seconds

as universal resilience behavior.

---

37. Resource exhaustion

Resources may include:

physical qubits
logical qubits
memory
CPU
GPU
execution slots
classical channels
control channels
network bandwidth
storage
shots
time
energy
provider quota

Recovery

Possible:

partition
defer
migrate
reduce optional overhead
change mitigation
change compilation strategy
scale out

Resource exhaustion must not be confused with hardware failure.

---

38. Memory exhaustion

The resilience subsystem itself must not create unbounded memory consumption.

Forbidden architecture:

collect every telemetry event forever

Required architecture:

stream
  |
  +--> aggregate
  |
  +--> prioritize
  |
  +--> persist required evidence
  |
  +--> discard according to policy

Critical evidence must remain available.

---

39. Telemetry failures

39.1 Missing telemetry

Missing telemetry must not automatically mean:

healthy

It means:

unknown

The system must distinguish:

healthy
unknown
unavailable

---

39.2 Telemetry corruption

If telemetry integrity fails:

mark evidence untrusted
invalidate dependent diagnoses

---

39.3 Telemetry overload

Use:

backpressure
aggregation
sampling
priority
bounded buffering

according to policy.

Critical events must receive priority.

---

40. Detection failures

A detector itself can fail.

Examples:

detector crash
detector timeout
invalid detector output
false positive
false negative
stale detector state

Rule

Detector failure must not be interpreted as absence of faults.

The detector registry must support health state for detectors.

---

41. Diagnosis failures

41.1 Unknown root cause

It is valid for diagnosis to return:

Unknown

The system must not invent certainty.

Possible action:

safe containment
conservative recovery
escalation
abort

---

41.2 Conflicting diagnoses

If independent detectors disagree:

retain both hypotheses
reduce confidence
apply conservative policy

Do not silently choose the most convenient diagnosis.

---

42. Planning failures

42.1 No feasible recovery plan

If no policy-compliant plan exists:

ESCALATE

or:

REJECT

depending on policy.

---

42.2 Stale recovery plan

Any change to:

capabilities
topology
calibration
policy
resource availability
security state

may invalidate a plan.

The plan must be revalidated before execution.

---

42.3 Planner failure

If the planner itself fails:

do not improvise

Use:

safe fallback
manual escalation
abort

as configured.

---

43. Recovery failures

43.1 Retry failure

A retry must consume the retry budget.

The budget is supplied by policy.

No universal retry count exists.

---

43.2 Restart failure

If restarting from a safe boundary fails:

attempt alternate recovery
migrate
escalate
reject

---

43.3 Resume failure

A checkpoint may be valid but the target may no longer be compatible.

Check:

program identity
IR version
checkpoint schema
logical state requirements
hardware capability
QEC configuration
resource requirements
security policy

before resume.

---

43.4 Rollback failure

If rollback cannot restore a verified state:

do not claim rollback succeeded

The state becomes:

RecoveryFailed

---

43.5 Migration failure

Migration may fail because:

- target unavailable;
- target incompatible;
- insufficient resources;
- QEC incompatibility;
- checkpoint incompatibility;
- routing infeasibility;
- policy denial.

The original target must not be silently reused without revalidation.

---

44. Checkpoint failures

Checkpointing cannot mean blindly serializing an arbitrary unknown quantum state.

Checkpoint types must distinguish:

classical execution state
compiled representation
logical state
measurement boundary
QEC state
reconstructible state
provider-supported state

---

44.1 Checkpoint creation failure

Possible:

retry
use alternate storage
checkpoint at a later safe boundary
continue without checkpoint if policy permits
abort if checkpoint is mandatory

---

44.2 Checkpoint corruption

Integrity verification must fail closed.

Possible:

alternate checkpoint
reconstruct from earlier checkpoint
restart
reject

---

44.3 Checkpoint incompatibility

A checkpoint must not be restored merely because its file format is readable.

Compatibility must include semantic and execution compatibility.

---

45. Serialization failures

Failures include:

unknown schema version
corrupted encoding
missing field
invalid field
unsupported version
incompatible semantics

Recovery

Only compatible schemas may be accepted.

Do not silently drop fields that are semantically required.

---

46. Compatibility failures

Compatibility may fail between:

program
IR
compiler
optimizer
router
scheduler
QEC
hardware
backend
checkpoint
resilience schema

The compatibility subsystem must detect this before execution.

Result:

REPLAN
MIGRATE
RECOMPILE
or REJECT

---

47. Determinism failures

If deterministic mode is requested, failures include:

unexpected random source
hidden global RNG
non-deterministic ordering
unstable plan ranking
unstable serialization
concurrent race affecting decision

Action

The operation must be marked:

DeterminismFailure

and must not claim deterministic reproducibility.

---

48. Security failures

Security failures are always high priority.

Examples:

unauthenticated telemetry
tampered telemetry
forged hardware state
malicious recovery request
unauthorized backend
credential failure
checkpoint tampering
plugin compromise
provenance tampering
policy bypass

Rule

Security failure may override availability.

The system must not recover onto an untrusted resource simply because it is available.

---

49. Telemetry trust failure

A telemetry source must have an explicit trust state.

Conceptually:

Trusted
Authenticated
Untrusted
Compromised
Unknown

A compromised source must not be used as authoritative health evidence.

---

50. Backend/provider failures

Core resilience must not contain vendor-specific recovery logic.

Instead:

hardware/provider adapter
        |
        v
canonical hardware contract
        |
        v
resilience

Provider-specific behavior belongs under the hardware boundary.

---

51. Backend migration failure

Migration requires:

capability compatibility
IR compatibility
QEC compatibility
routing feasibility
scheduling feasibility
policy permission
security authorization
verification

A different backend is not automatically an equivalent backend.

---

52. Distributed failures

For distributed quantum systems, possible failures include:

node loss
QPU loss
link loss
latency spike
partition
split-brain
lease expiry
ownership conflict
clock/timing inconsistency
partial execution
duplicate execution

---

53. Distributed partition

A network partition must not cause two controllers to independently recover the same execution unless the coordination contract explicitly allows that behavior.

"coordination/ownership.rs" and "coordination/lease.rs" must establish execution ownership.

---

54. Duplicate recovery

Two resilience controllers must not simultaneously perform conflicting recovery actions against the same execution.

Use:

ExecutionId
IncidentId
RecoveryAttemptId
ownership/lease

as appropriate.

---

55. Duplicate execution

A retry may produce a second execution.

Therefore the system must distinguish:

safe-to-retry
unsafe-to-retry
unknown-idempotency

A retry must not be performed merely because an operation timed out.

---

56. Race conditions

A resource can change between:

detect
diagnose
plan
execute

Therefore every plan must contain sufficient preconditions to be revalidated immediately before execution.

---

57. Capability-change race

Example:

planner sees qubit available
        |
        v
qubit fails
        |
        v
old plan tries to execute

Forbidden.

Required:

plan
 |
 v
precondition validation
 |
 +--> invalid -> replan
 |
 +--> valid -> execute

---

58. Cascading failures

One failure can create another:

qubit failure
    |
    v
rerouting
    |
    v
higher circuit depth
    |
    v
timing failure
    |
    v
decoherence
    |
    v
logical error

The incident model must preserve causal relationships.

Do not create independent unrelated incidents when they form one cascade.

---

59. Failure storms

A failure storm is a rapid increase in related failures.

Examples:

many qubits fail
many readout channels degrade
many telemetry events appear
many backend jobs fail

Required behavior:

aggregate
correlate
identify common cause
contain
plan at appropriate scope

Do not launch one global recovery per low-level event.

This is essential for large-scale operation.

---

60. Correlated-resource failures

If multiple resources fail together:

qubit A
qubit B
coupling A-B
control channel

the system should investigate a common cause before applying independent repairs.

---

61. Recovery amplification failure

Recovery itself can create additional load.

Example:

1000 failures
   |
   v
1000 retries
   |
   v
backend overload
   |
   v
more failures

The planner must account for recovery-induced load.

This requires:

budgets
capacity
backpressure
coordination

---

62. Retry storm prevention

Retry policies must support:

budget
backoff
jitter where allowed
deduplication
global coordination
per-resource limits
per-incident limits

The actual values are policy/configuration inputs.

There must be no universal hard-coded retry count.

---

63. Verification failures

Verification is itself a failure domain.

Examples:

semantic verification failed
result verification failed
provenance incomplete
confidence below policy
invariant violated
expected observable outside bounds

Rule

Verification failure prevents acceptance.

---

64. Semantic verification failure

If the adapted computation cannot be established as semantically compatible:

REJECT

unless a formally specified alternative verification path exists.

---

65. Result-confidence failure

A result may be available but insufficiently trustworthy.

Possible:

repeat
mitigate
increase shots
change verification
escalate
reject

---

66. Provenance failure

If the system cannot establish where a result came from:

program
→ transformation
→ target
→ execution
→ recovery
→ mitigation
→ result

the result must not be represented as fully verified.

---

67. Acceptance failure

The final acceptance gate must be explicit.

Possible states:

ACCEPT
DEGRADED_ACCEPT
RETRY
REPLAN
RECOVER
ESCALATE
REJECT

Acceptance is not implied by:

job completed

or:

backend returned result

---

68. Unknown failures

Unknown failure is a valid state.

It must contain:

unknown cause
confidence
evidence
affected resources
impact
safe containment

The system should choose the safest policy-compatible action.

It must not invent a cause.

---

69. Compound failures

A single execution may simultaneously experience:

hardware degradation
+
routing failure
+
timeout
+
readout degradation

The incident model must support multiple contributing failures.

Recovery must be based on the combined state.

---

70. Failure precedence

When failures conflict, use the following conceptual priority:

Security
    |
Semantic correctness
    |
Safety
    |
QEC/logical correctness
    |
Capability validity
    |
Resource validity
    |
Availability
    |
Performance
    |
Cost

This is a policy precedence model, not an implementation-specific enum ordering.

Availability must never override semantic correctness.

---

71. Failure containment

Before recovery, the system should contain the failure when necessary.

Containment may include:

quarantine resource
freeze affected execution
invalidate stale plans
stop unsafe retries
isolate compromised telemetry
prevent duplicate ownership

Containment must be reversible where possible.

---

72. Quarantine

A resource may enter:

Healthy
Degraded
Unstable
Unavailable
Recovering
Quarantined
Retired

Quarantine must not require a fixed number of resources.

It must work for:

one qubit
one coupling
one QPU
one backend
one distributed node

or any dynamically discovered resource set.

---

73. Retired resources

A retired resource must not automatically return to service.

Return-to-service requires:

health validation
capability validation
security validation
calibration validation
policy approval

---

74. Recovery scope

Recovery scope must be dynamically selected.

Possible scopes:

operation
gate
qubit
coupling
QEC block
region
circuit
execution
device
backend
distributed workload

A local failure must not automatically force global recovery.

A global failure must not be incorrectly treated as local.

---

75. Incremental recovery

Where safe:

local failure
    |
    v
local adaptation
    |
    v
local verification

If local recovery fails:

regional recovery

then:

global recovery

then:

migration

then:

escalation/rejection

This creates scalable recovery behavior.

---

76. Recovery budget exhaustion

When any recovery budget is exhausted:

retry budget
time budget
shot budget
memory budget
compilation budget
mitigation budget
migration budget

the system must transition to the next policy-defined action.

It must not silently continue consuming unlimited resources.

---

77. Failure loops

The system must detect:

A fails
→ recover
→ B fails
→ recover
→ A fails
→ ...

Repeated equivalent incidents indicate a recovery loop.

The planner must terminate or change strategy according to policy.

---

78. Recovery-loop detection

Track:

incident identity
failure classification
resource
action
attempt
outcome

A recovery action that repeatedly fails should have decreasing priority unless policy explicitly says otherwise.

---

79. Learning failures

Learning must never be required for basic correctness.

If:

learning model unavailable
prediction unavailable
model confidence insufficient

the system must fall back to deterministic/policy-based behavior.

Learning can improve ranking.

It cannot bypass safety or verification.

---

80. Predictor failure

If a learned model predicts:

recovery probability = 99%

but verification fails, the verified failure wins.

Predictions are advisory.

---

81. Registry failures

A detector/strategy/recovery plugin may fail.

Examples:

registration conflict
unsupported interface
panic/error
invalid metadata
security violation
version incompatibility

The registry must isolate the failed extension where possible.

A failed optional plugin must not corrupt the core resilience engine.

---

82. Plugin trust failure

An extension must not gain unrestricted authority merely because it implements a trait.

Plugin capabilities must be constrained by the integration/security policy.

---

83. Serialization of failures

Failure records must be serializable when persistence is required.

Serialization must preserve:

classification
severity
confidence
resource identities
incident identity
evidence references
provenance
timestamps
recovery state
schema version

Serialization must not change failure semantics.

---

84. Version failures

A newer failure schema encountered by an older runtime must result in:

compatible interpretation

or:

explicit unsupported-version failure

Never silently reinterpret unknown fields as known semantics.

---

85. Resource-scaling failure

A resilience implementation itself fails to scale.

Examples:

O(N²) work where local work was sufficient
unbounded event storage
fixed arrays
fixed topology assumptions
fixed qubit counts
global locks
unbounded retry fan-out

These are architecture defects and must be treated as production failures.

---

86. Scalability invariants

The resilience subsystem must not contain:

MAX_QUBITS
MAX_PHYSICAL_QUBITS
MAX_LOGICAL_QUBITS
MAX_BACKENDS
MAX_DEVICES
MAX_INCIDENTS
MAX_EVENTS
MAX_OPERATIONS

as semantic architecture limits.

Resource limits may exist only as:

runtime configuration
policy
security boundary
target capability
resource availability

The existing resilience scalability specification makes this same distinction: "infinite" means no artificial finite ceiling imposed by resilience, while each execution remains bounded by actual resources.

---

87. Memory scalability

Failure handling must be streamable.

Required pattern:

event
 |
 v
normalize
 |
 v
classify
 |
 v
aggregate
 |
 +--> retain critical evidence
 |
 +--> persist selected history
 |
 +--> discard according to policy

Never require the complete lifetime history to remain in memory.

---

88. Computational scalability

For a localized incident:

local incident
    |
    v
local diagnosis
    |
    v
local plan
    |
    v
local recovery

For a global incident:

global incident
    |
    v
global diagnosis
    |
    v
global plan

The scope must be dynamic.

---

89. Distributed scalability

A distributed deployment must support:

one resilience controller
multiple controllers
multiple QPUs
multiple backends
multiple execution regions

without assuming a fixed number.

Coordination must prevent:

duplicate recovery
conflicting recovery
split-brain recovery

---

90. Failure storm scalability

A storm containing thousands or millions of low-level observations must be represented efficiently.

Required strategy:

raw events
    |
    v
streaming normalization
    |
    v
correlation
    |
    v
incident aggregation
    |
    v
bounded planning

The number of raw events must not automatically equal the number of recovery operations.

---

91. Backpressure

Telemetry and failure streams must support backpressure.

Policies may specify:

buffer
aggregate
sample
prioritize
drop noncritical
persist critical
escalate

Critical evidence must not be silently discarded.

---

92. Failure prioritization

Prioritization should consider:

severity
semantic impact
security impact
logical-error risk
resource impact
confidence
scope
urgency
recovery cost

A low-severity event affecting an entire fleet may deserve higher priority than a critical but isolated event if policy defines fleet availability as important.

---

93. Failure dependency graph

Compound failures should be represented conceptually as:

Fault
 |
 +--> Resource degradation
       |
       +--> topology change
              |
              +--> routing failure
                     |
                     +--> schedule change
                            |
                            +--> fidelity degradation
                                   |
                                   +--> logical error

The history subsystem should preserve these relationships.

---

94. Failure provenance

Every recovery action must record:

why action was selected
which failure caused it
which evidence supported it
which policy permitted it
which capabilities enabled it
which resources were affected
what changed
what verification was performed

This is mandatory for production debugging and auditability.

---

95. Failure observability

At minimum, resilience should expose events for:

failure_detected
failure_classified
incident_created
incident_updated
resource_quarantined
plan_created
plan_invalidated
recovery_started
recovery_completed
recovery_failed
mitigation_started
mitigation_completed
verification_started
verification_failed
verification_completed
execution_migrated
execution_restarted
execution_resumed
execution_rejected
execution_escalated

The exact telemetry schema belongs to "telemetry/event.rs".

---

96. Failure metrics

Metrics should include:

failure_count
incident_count
failure_rate
correlated_failure_rate
detection_latency
diagnosis_latency
planning_latency
recovery_latency
verification_latency
recovery_success_rate
recovery_failure_rate
false_positive_rate
false_negative_rate
resource_quarantine_count
migration_count
retry_count
retry_exhaustion_count
verification_rejection_count

Metrics must not require a fixed cardinality.

High-cardinality identities should be handled through the telemetry policy rather than unbounded labels.

---

97. Deterministic replay

A failure should be replayable when sufficient evidence exists.

Replay input may include:

program/IR identity
execution context
hardware snapshot
capability snapshot
telemetry
fault stream
policy
configuration
random seed

A deterministic replay must not depend on hidden mutable global state.

---

98. Fault injection

"tests/fault_injection.rs" must be capable of injecting every major failure class.

At minimum:

qubit failure
gate failure
readout failure
leakage
loss
erasure
correlated fault
crosstalk
calibration drift
topology failure
routing failure
scheduling failure
compiler failure
QEC failure
decoder failure
mitigation failure
backend outage
timeout
network failure
checkpoint corruption
telemetry loss
security failure
resource exhaustion
verification failure

Fault injection should operate through contracts rather than production-only hacks.

---

99. Property testing

The failure model should test invariants such as:

failure never silently becomes success

unverified result is never accepted

stale plan is never executed

unknown telemetry is never interpreted as healthy

logical identity is never silently converted into physical identity

resource count does not alter semantic identity

recovery does not bypass policy

security failure cannot be ignored by availability policy

checkpoint corruption cannot produce accepted state

---

100. Fuzzing

Inputs to fuzz include:

failure streams
telemetry
resource sets
topologies
capability changes
policy combinations
serialized failures
checkpoint metadata
recovery plans

Fuzzing must never permit:

panic
memory unsafety
undefined behavior
unsafe code
silent acceptance of invalid state

---

101. Rust safety contract

The resilience subsystem must compile under:

#![forbid(unsafe_code)]

No:

unsafe
unsafe fn
unsafe impl
unsafe block

No FFI requiring unsafe code belongs in the core resilience subsystem.

Provider-specific unsafe integration, if ever unavoidable, must remain outside the safe resilience boundary and expose a safe, validated contract.

---

102. Panic policy

Production resilience code must not use panic-based control flow for recoverable operational failures.

Prefer:

Result<T, ResilienceError>

for recoverable failures.

Invariant violations should be represented through explicit validation wherever possible.

---

103. Error integration

All resilience failures must integrate with:

errors/error.rs
errors/classification.rs
errors/codes.rs

The error model must distinguish:

failure source
failure classification
severity
retryability
recoverability
security significance
semantic significance

The existing resilience architecture already identifies "errors/error.rs" as the canonical resilience error boundary.

---

104. Policy integration

Failures must never directly choose arbitrary actions.

The flow is:

Failure
   |
   v
Diagnosis
   |
   v
Policy
   |
   v
Candidate actions
   |
   v
Feasibility
   |
   v
Plan

This prevents detectors from becoming hidden recovery engines.

---

105. Planning integration

"planning/feasibility.rs" must revalidate:

resources
capabilities
topology
timing
QEC
policy
security
budgets

immediately before executing a plan.

---

106. Adaptation integration

Failures may trigger:

adaptation/remapping.rs
adaptation/rerouting.rs
adaptation/rescheduling.rs
adaptation/recompilation.rs
adaptation/reoptimization.rs
adaptation/qec_adaptation.rs
adaptation/backend_selection.rs

These modules request transformations from the authoritative quantum subsystems.

They do not duplicate those algorithms.

---

107. Recovery integration

Recovery actions include:

retry
restart
checkpoint
rollback
resume
migration
compensation

These belong to:

recovery/*

Failure modes describe when those actions may be considered.

---

108. Verification integration

Every recovery path must eventually enter:

verification/verifier.rs

with:

verification/invariant.rs
verification/semantic.rs
verification/result.rs
verification/confidence.rs
verification/provenance.rs
verification/acceptance.rs

A recovery action is not a successful recovery until verification establishes the required result state.

---

109. QEC integration

QEC remains responsible for quantum error correction.

Resilience consumes:

syndrome signals
logical health
decoder confidence
logical error indicators
code/resource state

and chooses whether adaptation/recovery is necessary.

---

110. ZQN integration

ZQN is the canonical source for quantum fault/noise semantics.

Resilience should consume ZQN's:

fault
location
classification
correlation
leakage
loss
erasure
noise
uncertainty

rather than defining competing fault semantics.

---

111. Hardware integration

Hardware supplies:

identity
technology
capabilities
instruction support
timing
topology
calibration
health
status
telemetry
execution

Resilience consumes these contracts.

It must not hard-code provider behavior into the core planner.

---

112. Routing integration

When physical resources change:

resilience
    |
    v
routing request
    |
    v
new mapping
    |
    v
verification

No resilience-specific routing algorithm should be introduced.

---

113. Scheduling integration

When timing/resources change:

resilience
    |
    v
scheduling request
    |
    v
new schedule
    |
    v
verification

Current quantum execution practice reinforces this separation: techniques such as dynamical decoupling depend on the actual scheduled timing and hardware pulse capabilities.

---

114. Optimization integration

When the target changes:

old target
   |
   v
new capabilities
   |
   v
optimization request

The optimizer remains responsible for transformation.

Resilience remains responsible for deciding that transformation is needed.

---

115. Benchmarking integration

Historical benchmarking may provide evidence about:

fidelity
latency
failure probability
resource stability
mitigation performance
backend stability

But benchmark results must not be treated as immutable truth.

Current telemetry and capability state take precedence where appropriate.

---

116. Simulation integration

Every recovery strategy should be testable in simulated environments.

The simulator should support:

synthetic faults
synthetic topology changes
synthetic calibration drift
synthetic backend outages

without requiring production hardware.

---

117. Acceptance rule

The final acceptance rule is:

ACCEPT

only when the required conditions are satisfied.

At minimum:

semantic validity
+
policy validity
+
capability validity
+
security validity
+
verification validity

Otherwise:

DEGRADED_ACCEPT
RETRY
REPLAN
ESCALATE
REJECT

as dictated by policy.

---

118. Degraded acceptance

"DEGRADED_ACCEPT" must never mean:

we don't know whether it is correct

It means:

the result remains within explicitly declared degraded correctness bounds

Those bounds must be policy-defined and recorded in provenance.

---

119. No silent fallback

Forbidden:

hardware fails
→ silently use simulator

unless migration to a simulator is explicitly permitted.

Forbidden:

QEC fails
→ silently reduce protection

Forbidden:

verification fails
→ return result anyway

Forbidden:

telemetry unavailable
→ assume healthy

---

120. No silent semantic changes

A resilience action must not change:

program meaning
logical qubit identity
measurement semantics
required observable
required invariants

without an explicit transformation contract and verification.

---

121. Failure and migration

Migration must be considered a transformation of execution environment, not a semantic transformation of the program.

same program
same canonical IR
different realization

The new realization must be independently validated.

---

122. Failure and machine size

A larger machine does not imply a different resilience semantic model.

The same failure model must work for:

one qubit
10 qubits
1,000 qubits
1,000,000 qubits
distributed logical resources

subject only to actual available resources.

The resilience architecture must therefore avoid all fixed-size assumptions.

---

123. Failure and topology size

Topology structures must be dynamically represented.

Do not write:

[[bool; 127]; 127]

or equivalent architecture-specific arrays.

Use the routing/hardware topology contracts.

---

124. Failure and operation count

Do not assume a fixed number of operations.

The failure model must support:

one operation
large circuit
streaming workload
long-running execution
distributed workload

---

125. Failure and incident count

Do not impose a semantic maximum on incidents.

Operational systems may impose configurable resource budgets.

The distinction is:

architecture:
unbounded by artificial semantic constants

deployment:
bounded by configured resources

---

126. Failure and concurrency

Multiple incidents may occur concurrently.

The system must support:

incident A
incident B
incident C

without requiring a global sequential lock.

However, conflicting actions on the same resource must be coordinated.

---

127. Resource conflict

Two recovery plans may attempt:

qubit X

simultaneously.

The planner/coordinator must detect resource conflicts and either:

serialize
merge
cancel one
replan

according to policy.

---

128. Incident merging

Two incidents should be merged when evidence shows a common cause.

Two incidents must remain separate when merging would destroy important distinctions.

The merge operation must preserve all source evidence.

---

129. Incident splitting

A previously merged incident may later be decomposed if diagnosis reveals multiple independent causes.

The history must retain the relationship.

---

130. Failure aging

Failures may become stale.

A diagnosis based on old telemetry should not automatically remain authoritative forever.

Failure evidence should carry temporal validity.

---

131. Stale evidence

If evidence is older than its policy-defined validity:

mark stale
reduce confidence
refresh
or reject its use

No universal fixed age is permitted.

---

132. Failure recovery ordering

Recovery actions should generally follow:

contain
→ preserve evidence
→ validate state
→ smallest safe adaptation
→ verify
→ escalate scope if necessary

This is a preference, not a universal algorithm.

Correctness overrides minimality.

---

133. Global failure

If the entire execution environment becomes invalid:

device
backend
cluster
distributed region

local recovery is insufficient.

The system must escalate to migration or external recovery.

---

134. Catastrophic failure

Examples:

no trusted backend
no valid checkpoint
semantic state lost
security compromise
irrecoverable logical error
verification impossible

Result:

REJECT

with complete provenance.

---

135. Security versus recovery

If the only available recovery target is untrusted:

availability must lose

The system must not execute sensitive quantum workloads on an unauthorized target.

---

136. Recovery authorization

Recovery actions that can alter:

backend
resource ownership
security boundary
checkpoint
execution identity

must require appropriate authorization according to the security architecture.

---

137. Auditability

Every failure and recovery must be auditable.

At minimum:

incident_id
execution_id
failure
classification
severity
confidence
evidence
diagnosis
policy
candidate plans
selected plan
actions
resources
verification
final state

---

138. Failure-code stability

Machine-readable failure codes must be stable.

Human-readable text may evolve.

Consumers must use stable codes rather than parsing messages.

Codes belong to:

errors/codes.rs

---

139. Error conversion

Lower-level errors may be converted into resilience errors.

Conversion must preserve:

source
classification
retryability
semantic significance
security significance

It must not erase the underlying error.

---

140. Error aggregation

Multiple lower-level errors should be represented as an incident rather than recursively nesting an unbounded error tree.

Use references/identities where appropriate.

---

141. Failure fan-out

One failure can affect many dependent resources.

The system should represent dependency relationships rather than materializing redundant failure objects for every downstream consequence.

---

142. Failure fan-in

Many observations may represent one cause.

The incident model must support fan-in.

events
  \ | /
   \|/
 incident

---

143. Resource health transitions

Health transitions should be explicit:

Unknown
  ↓
Healthy
  ↓
Degraded
  ↓
Unstable
  ↓
Unavailable
  ↓
Recovering
  ↓
Healthy

or:

Unavailable
  ↓
Quarantined
  ↓
Retired

Invalid transitions must be rejected.

---

144. Recovery state transitions

Recovery state must be explicit:

Idle
Detecting
Diagnosing
Planning
Adapting
Recovering
Verifying
Completed
Escalated
Failed

A recovery must not jump directly from:

Detected

to:

Accepted

without the required intermediate contracts.

---

145. Failure-state persistence

If the process crashes during recovery, persisted state must indicate whether:

action started
action completed
verification completed

A restarted controller must not blindly repeat an unknown action.

---

146. Crash consistency

Recovery state persistence must be designed so that:

crash before action

and:

crash after action

can be distinguished where technically possible.

---

147. Recovery idempotency

Where possible, recovery actions should be idempotent.

Where they are not idempotent, the action contract must explicitly declare that fact.

This is especially important for:

execution
migration
resource allocation
external side effects

---

148. Unknown action completion

If the controller cannot determine whether an action completed:

DO NOT blindly retry

Instead:

query state
reconcile
then decide

---

149. Failure reconciliation

After controller restart:

persisted state
+
current hardware state
+
current execution state
+
current capabilities
=
reconciled resilience state

The system must not assume persisted state is still current.

---

150. Recovery after controller restart

A controller restart must:

1. load persisted state;
2. validate schema;
3. validate current capabilities;
4. validate execution ownership;
5. reconcile resource state;
6. identify incomplete recovery actions;
7. replan if necessary;
8. continue only after safety checks.

---

151. Failure under resource pressure

When the resilience subsystem itself is under resource pressure, it must prioritize:

security-critical failures
semantic-critical failures
logical correctness failures
critical hardware failures
recovery coordination
verification
ordinary telemetry
historical analytics
learning

Optional learning/analytics must not starve recovery correctness.

---

152. Failure observability under pressure

If telemetry must be dropped:

drop low-value telemetry first

Never silently drop:

security events
semantic failures
logical-error indicators
recovery state transitions
verification failures
checkpoint integrity failures

---

153. Failure isolation

A faulty detector must not corrupt:

planner
recovery
verification

A faulty learning model must not corrupt:

policy
security
verification

A faulty backend must not corrupt:

canonical IR

A faulty plugin must not corrupt:

core state

---

154. Dependency failure

If a dependency becomes unavailable:

routing unavailable
scheduling unavailable
QEC unavailable
hardware unavailable
telemetry unavailable

resilience must explicitly represent the dependency failure.

It must not reinterpret it as successful completion.

---

155. Dependency recovery

When a dependency becomes available again:

refresh capability
revalidate state
revalidate plans

Do not automatically resume stale operations.

---

156. Recovery strategy selection

Strategy selection must consider:

failure type
severity
confidence
scope
policy
cost
resource availability
capabilities
history
security
verification requirements

Current quantum systems similarly expose multiple resilience/mitigation techniques with different cost/accuracy trade-offs rather than a single universal recovery method.

---

157. No universal best recovery

There is no universally correct:

retry
reroute
recompile
mitigate
migrate

The correct action depends on the current execution context.

Therefore the architecture must select dynamically.

---

158. Recovery cost

Recovery planning should consider:

time
shots
memory
qubits
logical error probability
classical computation
energy
provider/resource cost

Cost must not override mandatory correctness constraints.

---

159. Recovery confidence

Each plan should have confidence based on:

diagnosis confidence
capability certainty
historical evidence
strategy validity
verification feasibility

Low-confidence plans should be handled conservatively.

---

160. Failure-induced semantic risk

A failure is especially dangerous if adaptation can change:

observable
measurement
logical state
control flow
program result

Such adaptations require stronger verification.

---

161. Mid-circuit/control-flow failures

If the quantum execution model supports dynamic classical control, failures can occur inside control-flow regions.

The resilience engine must preserve:

classical state
control dependencies
measurement outcomes
branch conditions

when recovery/resume is supported.

A generic "restart circuit" is not necessarily equivalent to "resume computation."

---

162. Batch/session/distributed execution

Execution environments may support different workload modes.

Resilience must not assume a single job model.

Current quantum services distinguish job, session, and batch execution because recovery and workload coordination differ across these modes.

Zamani must represent execution mode as runtime context/capability rather than a hard-coded assumption.

---

163. Failure in batch execution

One failed member of a batch must not automatically invalidate every unrelated member.

Dependency relationships determine scope.

---

164. Failure in iterative/session execution

A session may have accumulated state, calibration context, or workload history.

A failure may require:

resume session
restart iteration
restart session
migrate session

according to execution semantics.

---

165. Failure in distributed execution

A failed node must not automatically imply global failure.

Determine:

dependency
redundancy
ownership
QEC protection
partitionability

first.

---

166. Failure containment boundary

The smallest safe containment boundary should be preferred.

Possible:

operation
region
logical qubit
physical qubit
QEC block
device
backend
execution fabric

---

167. Failure escalation

Escalation should progress according to policy:

local
→ regional
→ device
→ backend
→ distributed
→ operator
→ rejection

Not every deployment needs every level.

---

168. Operator escalation

If automated recovery cannot establish correctness, the system should expose:

what failed
what was attempted
why recovery stopped
what evidence exists
what remains uncertain

This is more useful than merely reporting:

quantum execution failed

---

169. Failure reporting

User-visible reporting should distinguish:

execution failed
execution recovered
execution degraded
result unverified
result rejected

Do not report a recovered result as though no failure occurred.

---

170. Privacy

Failure evidence may contain sensitive execution metadata.

Persistence/export must obey the repository security/privacy model.

Do not expose:

credentials
secrets
private hardware details
sensitive workload data

through ordinary failure telemetry.

---

171. Checkpoint security

Checkpoint integrity must be verified before use.

Tampered checkpoints are security failures and potentially semantic failures.

---

172. Failure provenance immutability

Once an accepted result is produced, its provenance must not be silently rewritten.

Corrections should create a new provenance event/version.

---

173. Failure history retention

History retention is policy-controlled.

Do not require infinite history in memory.

Long-term history should use storage contracts.

---

174. Historical data failure

If history storage is unavailable:

core correctness must continue if policy permits

unless audit/history persistence is itself a mandatory safety requirement.

The failure must nevertheless be observable.

---

175. Learning-history separation

A failed or unverified execution must not automatically become positive training feedback.

Only verified outcomes should update learning models.

---

176. Failure feedback loop

Valid feedback:

failure
→ recovery
→ verification
→ outcome
→ historical record
→ learning

Invalid feedback:

failure
→ attempted recovery
→ no verification
→ train model that recovery succeeded

---

177. Self-healing safety principle

The system must never optimize for:

availability at any cost

The correct objective is:

preserve valid quantum computation
while maximizing acceptable availability
within declared constraints

---

178. Failure acceptance matrix

Conceptually:

Condition| Continue| Adapt| Recover| Accept
transient timeout| possible| possible| yes| after verification
physical qubit degraded| possible| yes| possible| after verification
invalid IR| no| no| no| no
logical error| generally no| possible| yes| after verification
stale calibration| no| yes| possible| after verification
untrusted telemetry| no trust| possible| possible| only with sufficient trusted evidence
checkpoint corruption| no from checkpoint| possible| possible| no until verified
security compromise| generally no| contain| possible| only after security verification
unknown failure| policy-dependent| possible| possible| only after verification
semantic verification failure| no| replan| possible| no

---

179. Production invariant: no silent corruption

The following must always hold:

unknown != healthy

completed != correct

retrieved != verified

recovered != accepted

mitigated != unbiased

recompiled != semantically equivalent

migrated != compatible

retried != safe

Each must be explicitly established.

---

180. Production invariant: stale state cannot execute

Any change to:

hardware
topology
calibration
capability
policy
security
resource availability
QEC state

may invalidate a plan.

Plans must therefore carry sufficient preconditions for revalidation.

---

181. Production invariant: canonical semantics survive failure

The canonical IR remains the semantic source of truth.

Recovery changes the realization:

mapping
schedule
target
optimization
QEC configuration
mitigation

not the original program meaning.

---

182. Production invariant: no artificial scale ceiling

The resilience subsystem must not contain machine-size constants.

"Scale to infinity" means:

no artificial finite semantic limit

while actual executions remain bounded by:

available resources
policy
security
physics
runtime capacity

---

183. Production invariant: safe Rust

All resilience code must remain safe Rust.

Target:

Rust 1.97 / 1.97.1
Rust 2021
#![forbid(unsafe_code)]

No unsafe implementation is required by the failure model.

---

184. Required integration by file

The failure model integrates with the following files:

Failure specification| Required implementation
classification| "errors/classification.rs"
stable codes| "errors/codes.rs"
canonical error| "errors/error.rs"
failure identity| "model/fault.rs"
incidents| "model/incident.rs"
severity| "model/severity.rs"
health| "model/health.rs"
degradation| "model/degradation.rs"
resource identity| "model/resource.rs"
confidence| "model/confidence.rs"
detection| "detection/*"
diagnosis| "diagnosis/*"
policy| "policy/*"
planning| "planning/*"
adaptation| "adaptation/*"
recovery| "recovery/*"
mitigation| "mitigation/*"
verification| "verification/*"
state| "state/*"
checkpoints| "checkpoint/*"
telemetry| "telemetry/*"
history| "history/*"
learning| "learning/*"
coordination| "coordination/*"
serialization| "serialization/*"
resource limits| "limits/*"
extension points| "registry/*"
public orchestration| "api/controller.rs"

---

185. Integration with "quantum::ir::qubit"

All files that identify quantum qubits must use the canonical types.

At minimum:

model/fault.rs
model/resource.rs
model/capability.rs
state/logical.rs
state/physical.rs
adaptation/remapping.rs
diagnosis/localization.rs
telemetry/event.rs
verification/provenance.rs

where the corresponding identity is actually required.

No file may define a competing canonical qubit type.

---

186. Integration with ZQN

The following failure categories should be capable of consuming canonical ZQN semantics:

noise
fault
leakage
loss
erasure
correlation
crosstalk
drift
uncertainty

Resilience converts those observations into operational incidents.

It does not redefine the underlying physical semantics.

---

187. Integration with QEC

QEC supplies:

syndrome
decoder state
logical health
logical error signals
code configuration

Resilience supplies:

adaptation decisions
recovery decisions
migration decisions
policy constraints

---

188. Integration with hardware

Hardware supplies:

capability
health
status
topology
calibration
execution result

Resilience supplies:

requested adaptation
resource quarantine
execution recovery
migration request

---

189. Integration with routing

Routing owns:

logical-to-physical placement
connectivity transformation
route generation

Resilience owns:

when routing must be reconsidered
why routing was invalidated
whether rerouting is permitted

---

190. Integration with scheduling

Scheduling owns:

ordering
timing
resource conflicts
schedule generation

Resilience owns:

when schedule is invalid
why it is invalid
whether rescheduling is necessary

---

191. Integration with optimization

Optimization owns:

representation-preserving transformations
cost reduction
fault-tolerant transformations

Resilience owns:

when reoptimization is required
what target constraints changed
whether reoptimization is permitted

---

192. Integration with benchmarking

Benchmarking may provide:

historical performance
failure probability
fidelity estimates
resource stability
mitigation performance

Resilience must treat these as evidence, not immutable truth.

---

193. Integration with API

"api/controller.rs" must coordinate:

observe
→ detect
→ diagnose
→ policy
→ plan
→ validate
→ execute adaptation/recovery
→ verify

It must not embed provider-specific behavior.

---

194. Integration with "mod.rs"

The resilience root should only expose completed modules.

No speculative:

pub mod foo;

may exist without an actual compatible module.

The repository's quantum root explicitly follows this composition-root rule.

---

195. Testing requirements

"tests/" must cover:

unit
property
fuzz
fault injection
integration
deterministic replay
scalability
security
serialization
checkpoint recovery
distributed recovery

---

196. Minimum fault-injection matrix

At minimum test:

1 physical qubit failure
multiple physical qubit failures
single gate failure
many gate failures
readout failure
leakage
loss
erasure
correlated failure
topology partition
calibration drift
QEC degradation
decoder failure
compiler failure
routing failure
scheduler failure
backend failure
network failure
checkpoint corruption
telemetry failure
security failure
verification failure

---

197. Scale testing

Tests must run over dynamically generated sizes.

Do not encode tests around only:

127
1000
10000

as architecture limits.

Use generated resource models.

The architecture should work for:

N = 1
N = arbitrary small
N = large
N = distributed

subject to the test environment's actual capacity.

---

198. Determinism testing

Given equivalent:

program
hardware snapshot
telemetry
policy
seed

deterministic mode must produce equivalent decisions.

The system must not depend on hidden global state.

---

199. Security testing

Test:

forged telemetry
tampered checkpoint
unauthorized migration
malicious plugin
invalid recovery request
provenance tampering
policy bypass
resource impersonation

---

200. Final production acceptance criteria

"FAILURE_MODES.md" and its implementation are production-ready only when:

Correctness

- failure classes are explicit;
- failures cannot be silently accepted;
- semantic correctness is protected;
- recovered results are verified.

Scalability

- no fixed qubit limits;
- no fixed topology assumptions;
- no fixed backend count;
- no fixed incident count;
- streaming telemetry;
- bounded memory;
- aggregation of failure storms;
- local-to-global recovery scope.

Integration

- canonical "quantum::ir::qubit" identities;
- ZQN fault semantics;
- QEC signals;
- hardware capabilities;
- routing;
- scheduling;
- optimization;
- execution;
- benchmarking;
- simulation.

Safety

- Rust 1.97/1.97.1;
- Rust 2021;
- "#![forbid(unsafe_code)]";
- no unsafe dependencies in the core;
- no silent fallback;
- no unsafe retry;
- no unverified acceptance.

Reliability

- checkpoint/recovery;
- migration;
- retry policy;
- failure-loop detection;
- recovery budgets;
- reconciliation after controller restart.

Security

- trusted telemetry;
- checkpoint integrity;
- recovery authorization;
- provenance;
- plugin isolation;
- security-first precedence.

Observability

- incidents;
- failure events;
- recovery events;
- verification events;
- provenance;
- metrics;
- deterministic replay where possible.

---

201. Final invariant

The entire failure model reduces to this:

                 FAILURE
                    |
                    v
                DETECT
                    |
                    v
               DIAGNOSE
                    |
                    v
                 POLICY
                    |
                    v
                  PLAN
                    |
                    v
             VALIDATE PLAN
                    |
             +------+------+
             |             |
           valid         invalid
             |             |
             v             v
          RECOVER        REPLAN
             |
             v
          VERIFY
             |
       +-----+-----+
       |           |
     valid       invalid
       |           |
       v           v
    ACCEPT       REPLAN
                   |
                   v
               ESCALATE
                   |
                   v
                REJECT

The central rule is:

«Resilience exists to preserve valid quantum computation, not merely to keep a machine running.»

Therefore:

Availability
    must never override
Semantic correctness.

Recovery
    must never override
Verification.

Optimization
    must never override
Safety.

Scaling
    must never introduce
Hard-coded machine assumptions.

Physical identity
    must never replace
Canonical logical identity.

Unknown
    must never be interpreted as
Healthy.

Completed
    must never be interpreted as
Correct.

This specification is intentionally independent of any particular quantum vendor, processor size, topology, QEC code, simulator, or future quantum technology. It is therefore compatible with Zamani's "write once, scale everywhere" objective: the program remains expressed against canonical semantics while failures alter the execution realization through the appropriate routing, scheduling, optimization, QEC, hardware, runtime, and verification contracts.

Current quantum practice reinforces this separation: error suppression and mitigation techniques have distinct assumptions and overheads, and execution systems expose different execution modes and resilience mechanisms rather than treating all failures as the same kind of retry.This version is intended to be the normative contract that the implementation files conform to, rather than documentation that has to be rewritten after those files are implemented. It also aligns with the repository's existing scalability contract, which explicitly requires dynamic resources, logical/physical separation, plan revalidation, incremental adaptation, streaming telemetry, and aggregation of failure storms. 