Worked for 8s

Below is the complete production-oriented OBSERVABILITY.md, aligned with the repository’s existing resilience architecture, canonical quantum::ir::qubit identity model, determinism contract, scalability contract, and ownership boundaries. The repository currently does not contain the target file, so this is written as a new file rather than a patch. The existing architecture explicitly makes quantum::ir::qubit::QubitId and PhysicalQubitId canonical and prohibits competing quantum identities.

Zamani Quantum Resilience — Observability Specification

Path: "src/quantum/resilience/OBSERVABILITY.md"
Status: Normative production specification
Subsystem: "quantum::resilience"
Language: Rust
Edition: Rust 2021
Required toolchain: Rust 1.97 / Rust 1.97.1
Safety: "unsafe" forbidden
Canonical quantum identity: "crate::quantum::ir::qubit"
Scope: Provider-independent, hardware-independent, scalable quantum-execution observability

---

1. Purpose

This document defines the production observability contract for "quantum::resilience".

Observability exists to answer, with sufficient evidence and without changing program semantics:

1. What execution was requested?
2. What canonical quantum program/IR was executed?
3. Which resources were selected?
4. What capabilities were available?
5. What was observed?
6. Which faults or anomalies were detected?
7. How confident was the system?
8. What diagnosis was produced?
9. Which policy was applied?
10. Which resilience plan was selected?
11. Which adaptations occurred?
12. Which recovery actions occurred?
13. Which mitigation techniques were applied?
14. Which QEC signals were observed?
15. What result was produced?
16. Was the result verified?
17. Why was the result accepted, degraded-accepted, retried, escalated, or rejected?
18. Can the execution be reconstructed, audited, and deterministically replayed where the determinism contract permits it?

The observability subsystem MUST therefore provide a coherent evidence chain:

Program
   ↓
Canonical IR
   ↓
Compilation
   ↓
Routing
   ↓
Scheduling
   ↓
Hardware/Simulator
   ↓
Execution
   ↓
Observations
   ↓
Detection
   ↓
Diagnosis
   ↓
Policy
   ↓
Planning
   ↓
Adaptation
   ↓
Recovery / Mitigation
   ↓
Verification
   ↓
Decision
   ↓
Result

Observability MUST NOT be an afterthought added only after failures occur.

It is a first-class part of the resilience architecture.

---

2. Core principle

The fundamental observability principle is:

«Every resilience decision MUST be explainable from a bounded, identifiable, integrity-protected set of observations and contextual inputs, without exposing unnecessary sensitive information.»

Observability therefore has four simultaneous requirements:

Visibility
+
Causality
+
Integrity
+
Privacy

A system that emits enormous amounts of telemetry but cannot explain why a recovery decision occurred is not sufficiently observable.

A system that records everything but permits telemetry to influence decisions without authentication or provenance is unsafe.

A system that explains decisions but cannot operate at large scale is not production-ready.

---

3. Relationship to the rest of the resilience architecture

Observability does not own every piece of information it records.

Ownership remains:

Information| Authoritative subsystem
Quantum semantics| "quantum::ir"
Logical qubit identity| "quantum::ir::qubit::QubitId"
Physical qubit identity| "quantum::ir::qubit::PhysicalQubitId"
Quantum operations| "quantum::ir"
Fault semantics| "quantum::zqn::fault"
QEC semantics| QEC subsystem
Routing| "quantum::routing"
Scheduling| "quantum::scheduling"
Optimization| "quantum::optimization"
Hardware capabilities| "quantum::hardware"
Hardware topology| "quantum::hardware"
Calibration| "quantum::hardware"
Execution| hardware/runtime boundary
Benchmarking| quantum benchmarking subsystem
Resilience decisions| "quantum::resilience"
Observability model| "quantum::resilience::telemetry"
Persistent incident history| "quantum::resilience::history"
Provenance| "quantum::resilience::verification::provenance"

Observability records and correlates information from these boundaries.

It MUST NOT silently redefine their semantics.

---

4. Observability directory

The production resilience architecture contains:

src/quantum/resilience/
│
├── telemetry/
│   ├── mod.rs
│   ├── event.rs
│   ├── metric.rs
│   ├── trace.rs
│   ├── health.rs
│   ├── collector.rs
│   └── exporter.rs
│
├── history/
│   ├── mod.rs
│   ├── incident.rs
│   ├── execution.rs
│   ├── recovery.rs
│   └── statistics.rs
│
├── verification/
│   └── provenance.rs
│
├── state/
│   └── persistence.rs
│
└── serialization/
    ├── schema.rs
    ├── encode.rs
    ├── decode.rs
    └── version.rs

The telemetry subsystem is the real-time observation boundary.

The history subsystem is the durable historical boundary.

The verification/provenance subsystem is the evidence and audit boundary.

The serialization subsystem defines interoperable representation.

---

5. Observability layers

Observability MUST be separated into distinct layers.

5.1 Events

Events represent discrete occurrences.

Examples:

execution.started
execution.submitted
execution.accepted
execution.failed
fault.detected
incident.created
diagnosis.completed
plan.created
adaptation.started
adaptation.completed
recovery.started
recovery.completed
mitigation.applied
verification.completed
resource.degraded
resource.quarantined
backend.unavailable
checkpoint.created
checkpoint.restored
policy.escalated

Events are not metrics.

---

5.2 Metrics

Metrics represent measurable quantities over time.

Examples:

execution duration
queue duration
compilation duration
routing duration
scheduling duration
gate count
qubit count
logical qubit count
physical qubit count
shot count
failure rate
retry rate
recovery rate
verification failure rate
logical error rate
readout error rate
detector confidence
diagnosis confidence
resource utilization
telemetry loss rate

Metrics MUST have explicit units and aggregation semantics.

---

5.3 Traces

Traces represent causal execution paths.

A trace may represent:

one program
one execution
one recovery cycle
one incident
one migration
one distributed operation

Traces connect events and spans.

---

5.4 Health observations

Health observations represent the state of resources or subsystems.

Examples:

healthy
degraded
unstable
unavailable
recovering
quarantined
retired
unknown

Health is state, not merely a metric.

---

5.5 Provenance

Provenance records why a result or decision exists.

It MUST connect:

program identity
IR identity
execution identity
resource identity
capability snapshot
observation snapshot
policy identity
strategy identity
adaptation
recovery
mitigation
verification
result

---

6. Canonical observability identity model

Observability MUST distinguish multiple identity domains.

6.1 Quantum identity

The canonical quantum identity MUST come from:

crate::quantum::ir::qubit

Use:

crate::quantum::ir::qubit::QubitId
crate::quantum::ir::qubit::PhysicalQubitId

where applicable.

The resilience subsystem MUST NOT introduce:

ResilienceQubitId
TelemetryQubitId
ObservationQubitId
RecoveryQubitId
FaultQubitId

as competing quantum identities.

This follows the repository-wide canonical identity rule.

---

6.2 Resilience identity

Resilience may define identities for resilience-owned objects:

ExecutionId
ObservationId
EventId
IncidentId
DiagnosisId
DecisionId
PlanId
RecoveryId
AttemptId
CheckpointId
TraceId
SpanId

These identify resilience objects, not qubits.

---

6.3 Content identity

Content-addressed identities may include:

ProgramHash
IRHash
PolicyHash
CapabilitySnapshotHash
ObservationSnapshotHash
PlanHash
CheckpointHash
ResultHash
ProvenanceHash

These MUST be independent of operational identifiers.

---

7. Event contract

"telemetry/event.rs" owns the canonical resilience event model.

Every event MUST contain enough information to answer:

what happened?
when did it happen?
where did it happen?
to which execution?
from which source?
with what confidence?
with what severity?
under which schema?

A production event conceptually contains:

Event
├── event_id
├── event_type
├── schema_version
├── execution_id
├── trace_id
├── timestamp
├── sequence
├── source
├── resource_scope
├── severity
├── confidence
├── causality
├── payload
├── provenance
└── integrity

The exact Rust representation belongs in "telemetry/event.rs".

---

8. Event immutability

Once emitted, an event MUST be immutable.

Corrections MUST be represented as new events.

For example:

diagnosis.completed

followed by:

diagnosis.corrected

rather than mutating historical data.

This is necessary for:

- auditability;
- deterministic replay;
- incident reconstruction;
- forensic analysis;
- distributed consistency.

---

9. Event ordering

Events from concurrent sources may arrive out of order.

Observability MUST NOT assume arrival order equals causal order.

Every event should therefore provide, where available:

event identity
source identity
logical sequence
timestamp
parent/correlation identity

When deterministic reconstruction is required, events MUST be canonically ordered according to the determinism contract.

The repository's determinism specification explicitly prohibits using asynchronous completion order or unordered collection iteration as an implicit decision input.

---

10. Time model

Observability must distinguish:

wall-clock time
monotonic elapsed time
logical time
source-provided event time
ingestion time

These MUST NOT be conflated.

10.1 Wall-clock time

Used for:

- human-readable timestamps;
- operational correlation;
- external audit.

It MUST NOT silently influence strict deterministic decisions.

---

10.2 Monotonic time

Used for:

- duration measurement;
- timeout enforcement;
- latency measurement.

Monotonic measurements MUST NOT be presented as globally comparable timestamps.

---

10.3 Logical time

Used when ordering events in a deterministic distributed workflow.

---

10.4 Ingestion time

Records when the observability subsystem received an observation.

Ingestion time is not necessarily the event occurrence time.

---

11. Event causality

Events SHOULD support explicit causal relationships.

Conceptually:

fault.detected
      │
      ▼
incident.created
      │
      ▼
diagnosis.completed
      │
      ▼
plan.created
      │
      ▼
reroute.started
      │
      ▼
reroute.completed
      │
      ▼
verification.completed

This allows an operator or automated analysis system to reconstruct why an action occurred.

---

12. Event severity

Severity MUST use the canonical resilience severity model.

It MUST NOT be encoded as arbitrary provider-specific strings.

Examples:

Informational
Degraded
Major
Critical
Fatal

Severity indicates impact.

It does not itself determine recovery.

Policy decides what action follows severity.

---

13. Confidence

Observations MUST be capable of carrying confidence.

Examples:

hardware failure confidence = 0.94
drift confidence = 0.71
QEC logical-fault confidence = 0.99

The exact representation belongs to:

model/confidence.rs

Observability records it.

It MUST NOT silently convert uncertain observations into certain facts.

---

14. Unknown values

Observability MUST distinguish:

known
unknown
not applicable
not observed
invalid
redacted
unavailable

These states MUST NOT be collapsed into one null/zero value.

For example:

fidelity = 0

must not automatically mean:

fidelity unknown

Zero may be a legitimate measurement.

---

15. Resource scope

Observations MUST be able to apply to different resource scopes.

At minimum:

global
fleet
backend
device
region
logical qubit
physical qubit
coupling
control channel
execution
operation
measurement
QEC block
checkpoint

No fixed hierarchy may be assumed.

Future hardware architectures may introduce new resource types.

The resource identity model therefore comes from the resilience resource/capability contracts.

---

16. Operation-level observability

Where execution infrastructure provides operation identity, telemetry SHOULD identify:

canonical operation identity
operation position
logical qubits
physical qubits
gate/instruction identity
duration
result
error indicators

Logical and physical identities MUST remain separate.

If physical mapping is unavailable to the observability layer, it MUST record that mapping information as unavailable rather than inventing it.

---

17. Canonical IR integration

Observability MUST integrate with:

crate::quantum::ir

The preferred chain is:

QuantumCircuit
    ↓
content identity
    ↓
execution
    ↓
telemetry

Observability MUST NOT create a second circuit representation.

It may record:

IR hash
operation count
qubit count
logical resource identities
semantic version
compiler version

where those values are supplied by the canonical IR/compiler contracts.

---

18. ZQN integration

Quantum fault observations MUST preserve canonical ZQN semantics.

The correct flow is:

ZQN Fault
    ↓
Observation
    ↓
Incident
    ↓
Diagnosis

Observability MUST NOT redefine:

leakage
loss
erasure
correlated fault
fault location
noise semantics

The repository architecture already establishes ZQN as the authoritative fault-semantic boundary.

---

19. Hardware integration

"telemetry/collector.rs" may consume hardware observations.

Possible observations include:

device state
availability
calibration version
calibration age
gate fidelity
readout quality
timing
queue state
topology version
capability changes
resource degradation
execution errors

The telemetry layer MUST record the source and snapshot identity.

It MUST NOT assume a specific provider.

It MUST NOT contain:

if IBM ...
if IonQ ...
if Rigetti ...

in the provider-neutral core.

Provider-specific conversion belongs in hardware adapters.

---

20. Routing integration

Routing-related observability should record:

routing request identity
input IR identity
resource/capability snapshot
mapping identity
route identity
routing outcome
failure
cost

It MUST NOT duplicate the routing algorithm.

When a reroute occurs, the trace SHOULD connect:

old mapping
fault/degradation
reroute request
new mapping
verification

---

21. Scheduling integration

Scheduling telemetry should expose:

schedule identity
schedule version
timing model identity
resource constraints
duration model
schedule generation result
schedule invalidation reason
rescheduling result

The observability layer MUST NOT invent gate durations.

Those come from the scheduling/hardware timing contracts.

---

22. Optimization integration

Optimization telemetry SHOULD record:

optimization profile
pass pipeline identity
input IR identity
output IR identity
pass sequence
cost before
cost after
semantic verification status

A resilience-triggered reoptimization MUST be distinguishable from an initial optimization.

---

23. QEC integration

QEC observability MUST support:

code identity
code version
logical resource identity
syndrome summary
decoder identity
decoder confidence
logical error indicators
correction status
resource degradation

Observability MUST NOT expose sensitive raw state unless explicitly authorized.

It MUST NOT implement QEC decoding.

---

24. Detection integration

Detection modules emit observations/events into the telemetry boundary.

Examples:

threshold exceeded
statistical anomaly
drift detected
execution timeout
backend failure
QEC signal
hardware signal

A detector SHOULD provide:

detector identity
detector version
input observation identity
detector configuration identity
output classification
confidence

This is essential for reproducing why an incident was detected.

---

25. Diagnosis integration

Diagnosis telemetry MUST capture:

diagnosis identity
input incident
candidate causes
selected cause
confidence
evidence references
diagnostician identity/version

The complete raw evidence need not always be duplicated.

References to immutable observation records are preferable at scale.

---

26. Policy integration

A resilience decision MUST record the policy identity used.

At minimum:

policy identity
policy version
policy hash
constraints identity
objective identity
budget identity
safety policy identity

The telemetry system MUST NOT merely record:

"policy = default"

if multiple versions of the default policy can exist.

---

27. Planning integration

A plan event SHOULD record:

plan identity
incident identity
diagnosis identity
candidate count
selected action(s)
plan cost
risk
confidence
preconditions
verification requirements
planner identity/version

At large scale, individual candidate plans SHOULD NOT necessarily be emitted as high-cardinality metrics.

They can be stored as structured trace/event data when needed.

---

28. Adaptation integration

Adaptation events should identify:

adaptation type
affected resources
input artifact identity
output artifact identity
reason
capability snapshot
policy
verification result

Supported adaptation categories include:

remapping
rerouting
rescheduling
recompilation
reoptimization
QEC adaptation
backend selection

---

29. Recovery integration

Recovery telemetry MUST distinguish:

attempt
action
outcome
reason
verification

Examples:

retry.started
retry.completed
restart.started
resume.completed
rollback.completed
migration.completed
checkpoint.restored
compensation.completed

A recovery action MUST never appear successful solely because the underlying API returned successfully.

The action's semantic verification result must be represented separately.

---

30. Mitigation integration

Mitigation telemetry SHOULD record:

strategy identity
strategy version
target execution identity
noise assumptions
configuration identity
overhead
result
verification

Examples:

readout mitigation
zero-noise extrapolation
probabilistic error cancellation
twirling
dynamical decoupling
custom strategy

Randomized mitigation MUST record its explicit randomness identity according to "DETERMINISM.md".

---

31. Verification integration

Verification is the final trust boundary.

Telemetry MUST record:

verification identity
verification policy
invariants evaluated
semantic verification status
result verification status
confidence
acceptance decision
failure reason

Possible final states:

ACCEPT
DEGRADED_ACCEPT
RETRY
REPLAN
ESCALATE
REJECT

A successful execution with failed verification MUST remain a failed verification outcome.

---

32. Metric model

"telemetry/metric.rs" owns metric definitions.

Every metric MUST define:

name
schema/version
type
unit
scope
source
aggregation semantics
validity semantics
privacy classification
cardinality expectations

Metric types MAY include:

counter
gauge
histogram
distribution
event-derived statistic
ratio
quantile

---

33. Metric units

Units MUST be explicit.

Examples:

seconds
nanoseconds
shots
qubits
operations
bytes
probability
ratio
percentage
energy units
cost units

The observability layer MUST NOT expose ambiguous fields such as:

time
rate
cost

without a defined unit.

---

34. Quantum-specific metrics

The system SHOULD support metrics such as:

physical_qubit_count
logical_qubit_count
active_qubit_count
available_qubit_count
gate_count
two_qubit_gate_count
depth
circuit_duration
logical_error_rate
physical_error_rate
readout_error_rate
leakage_rate
erasure_rate
loss_rate
syndrome_error_rate
decoder_failure_rate

These MUST be supplied by authoritative subsystems where applicable.

---

35. Resilience metrics

The system SHOULD support:

resilience_detection_count
incident_count
diagnosis_count
recovery_attempt_count
recovery_success_count
recovery_failure_count
migration_count
reroute_count
reschedule_count
recompile_count
mitigation_count
verification_failure_count
accepted_result_count
degraded_accept_count
escalation_count
rejection_count

These are observability measurements.

They are not themselves control decisions.

---

36. Latency metrics

At minimum, where measurable:

queue latency
compilation latency
optimization latency
routing latency
scheduling latency
submission latency
execution latency
telemetry latency
detection latency
diagnosis latency
planning latency
adaptation latency
recovery latency
verification latency
end-to-end latency

Durations MUST use monotonic timing internally.

---

37. Error budgets

Observability SHOULD expose resource/error budgets without turning them into hard-coded limits.

Examples:

retry budget remaining
time budget remaining
shot budget remaining
compilation budget remaining
mitigation budget remaining
recovery budget remaining

The actual limits belong to policy/configuration.

Observability reports the current state.

---

38. High-cardinality data

Quantum systems can produce extremely high-cardinality dimensions:

qubit identity
operation identity
execution identity
shot identity
trace identity
incident identity

These MUST NOT automatically become metric labels.

High-cardinality data belongs in:

events
traces
structured records
history

rather than unbounded metric dimensions.

This is essential for scalability.

---

39. Cardinality rule

A metric MUST NOT create a new persistent time-series dimension for every:

qubit
execution
shot
incident
checkpoint
trace

unless the deployment explicitly provides sufficient cardinality capacity.

The observability architecture MUST remain functional when the number of quantum resources grows arbitrarily.

---

40. Shot-level observability

Individual-shot telemetry is potentially enormous.

Therefore:

raw shot data

MUST NOT be mandatory for normal production observability.

The system SHOULD support configurable levels:

disabled
sampled
aggregated
full

Full capture MUST require explicit resource/policy authorization.

---

41. Sampling

Sampling MUST be explicit and auditable.

A sampling decision SHOULD record:

sampling policy
sampling rate
sampling algorithm
seed/randomness identity if applicable
reason

Sampling MUST NOT silently cause an operator to believe that all observations were captured.

---

42. Adaptive sampling

Adaptive sampling MAY increase observability during incidents.

Example:

healthy
→ low-volume telemetry

degraded
→ increased telemetry

critical
→ detailed telemetry

However, adaptive sampling MUST itself be observable.

The system should record:

sampling policy transition
reason
previous level
new level

---

43. Telemetry backpressure

"telemetry/collector.rs" and related infrastructure MUST define behavior when telemetry production exceeds collection capacity.

Possible policies:

block
buffer
sample
aggregate
drop low-priority data
spill to durable storage
escalate

The policy MUST be explicit.

Critical resilience events MUST NOT be silently dropped merely because telemetry is busy.

---

44. Loss accounting

Telemetry loss itself is an observable event.

The system MUST be able to report:

events produced
events accepted
events dropped
events sampled
events delayed
events rejected

If telemetry is incomplete, the observability state MUST indicate that it is incomplete.

---

45. Telemetry health

"telemetry/health.rs" MUST represent the health of the observability pipeline itself.

At minimum:

collector health
buffer health
exporter health
storage health
drop rate
latency
integrity status

This prevents the system from assuming:

no telemetry = no failures

---

46. Collector contract

"telemetry/collector.rs" is responsible for collecting observations from:

hardware
runtime
QEC
routing
scheduling
optimization
compiler
simulation
benchmarking
resilience detectors

The collector MUST normalize observations into the canonical telemetry model.

It MUST NOT:

- implement provider-specific resilience decisions;
- mutate canonical quantum state;
- silently change fault semantics;
- silently discard source identity.

---

47. Collector isolation

A failing telemetry source MUST NOT automatically crash quantum execution.

The collector should support isolated source failures.

For example:

hardware telemetry unavailable

does not necessarily mean:

quantum execution must crash

unless policy requires that telemetry source for safety.

---

48. Critical telemetry

Some telemetry is safety-critical.

Examples:

resource quarantine
verification failure
security violation
checkpoint integrity failure
semantic mismatch
untrusted hardware state

Critical events MUST receive stronger durability/integrity guarantees than ordinary diagnostic metrics.

---

49. Exporter contract

"telemetry/exporter.rs" is the external observability boundary.

It MUST be provider-neutral.

The core resilience system MUST NOT require a specific:

monitoring vendor
metrics backend
logging platform
tracing platform
cloud
database

Exporters SHOULD be replaceable.

---

50. Export failure

Exporter failure MUST NOT automatically imply execution failure.

The system MUST distinguish:

quantum execution failure

from:

observability export failure

However, if policy requires guaranteed auditability, an exporter/storage failure MAY become a resilience incident.

That decision belongs to policy.

---

51. Multiple exporters

The architecture SHOULD support multiple exporters simultaneously.

Example:

local audit storage
+
operational metrics
+
distributed tracing

One exporter failing MUST NOT necessarily disable the others.

---

52. Privacy

Observability MUST follow data minimization.

Telemetry MUST NOT automatically record:

secret keys
credentials
private authentication material
raw sensitive payloads
unnecessary user data

Quantum state information may itself be sensitive.

Raw state or detailed measurement information MUST be subject to explicit policy.

---

53. Security classification

Observability data SHOULD support classification such as:

public
internal
restricted
confidential
secret

The exact classification model belongs to the security architecture.

Telemetry consumers MUST respect it.

---

54. Redaction

Sensitive fields MUST support redaction.

Redaction MUST be represented explicitly.

It MUST NOT look like:

missing
unknown
zero
empty

A consumer should be able to distinguish:

not observed

from:

observed but intentionally redacted

---

55. Integrity

Important observability records SHOULD support integrity protection.

At minimum, integrity-sensitive records SHOULD have:

schema identity
content identity/hash
source identity
sequence
parent/reference identity
integrity metadata

Cryptographic implementation belongs to the repository's established security/integrity layer rather than being duplicated inside telemetry.

---

56. Trust boundaries

Telemetry arriving from external systems MUST be treated as untrusted until validated.

Potentially untrusted sources include:

hardware providers
remote backends
distributed nodes
plugins
external monitoring systems
network services

The system MUST distinguish:

observed
validated
trusted
untrusted
conflicting

---

57. Conflicting observations

Two sources may disagree.

Example:

source A: qubit healthy
source B: qubit degraded

The telemetry subsystem MUST preserve both observations.

It MUST NOT silently overwrite one with the other.

Diagnosis/policy decides how the conflict is handled.

---

58. Provenance

Every resilience-critical observation SHOULD have provenance sufficient to identify:

source
source version
schema version
collection path
timestamp
resource scope
configuration identity
integrity status

The provenance subsystem remains authoritative for complete decision provenance.

Telemetry references provenance rather than duplicating large objects unnecessarily.

---

59. Determinism

Observability MUST integrate directly with "DETERMINISM.md".

Observability MUST NOT introduce hidden nondeterministic inputs into deterministic planning.

Forbidden:

telemetry arrival order

being used as implicit priority.

Forbidden:

HashMap iteration order

being used as event ordering.

Forbidden:

current wall-clock time

being used to alter a strict deterministic decision.

The repository's determinism specification requires explicit input closure, deterministic ordering, stable tie-breaking, controlled randomness, and concurrency-independent deterministic decisions.

---

60. Deterministic event ordering

For replay, observations SHOULD be normalized according to:

logical sequence
→ stable source identity
→ event identity
→ canonical serialization

The exact ordering MUST be specified by the implementation and remain stable across supported Rust versions.

---

61. Randomized observability

Sampling or telemetry randomization MAY use randomness.

If that randomness can influence deterministic behavior, it MUST be explicitly included in the deterministic context.

Random streams SHOULD be domain-separated from:

planner
mitigation
learning
simulation
fault injection

so telemetry sampling cannot accidentally alter another subsystem's random sequence.

---

62. Trace model

"telemetry/trace.rs" owns distributed/cross-subsystem tracing.

A trace SHOULD have:

TraceId
root operation
parent-child spans
start/end timing
status
attributes
events
links

The implementation MUST avoid assuming that one process equals one trace.

A trace may span:

compiler
routing
scheduler
runtime
hardware
QEC
resilience
verification

---

63. Span model

Useful spans include:

compile
optimize
route
schedule
submit
queue
execute
collect
detect
diagnose
plan
adapt
recover
mitigate
verify
persist
export

Spans MUST be bounded enough that very large executions do not create unmanageable tracing overhead.

---

64. Trace sampling

Trace sampling MUST be policy-driven.

Possible policies:

always
probabilistic
incident-only
error-only
adaptive
disabled

The selected policy MUST be observable.

---

65. Incident tracing

Every incident SHOULD have a trace linking:

triggering observation
related observations
diagnosis
plan
actions
verification
final state

This becomes the primary operational path for debugging resilience behavior.

---

66. Recovery tracing

Recovery attempts SHOULD form child spans/events beneath the incident trace.

Example:

Incident
 ├── diagnosis
 ├── plan
 ├── reroute
 │    ├── route generation
 │    └── route verification
 ├── execution
 └── result verification

---

67. Distributed observability

For distributed quantum execution, observability MUST support:

node identity
resource identity
execution identity
trace identity
causal relationships
clock uncertainty
partial failure
partition
reconnection

It MUST NOT assume globally synchronized clocks.

---

68. Distributed event identity

Distributed systems MUST avoid generating colliding identifiers.

Event identity should be content-derived or otherwise globally unique according to the repository's identity contract.

A node-local counter alone MUST NOT be treated as globally unique.

---

69. Partial failure

Distributed observability MUST represent partial visibility.

Example:

node A observable
node B unavailable
node C delayed

This MUST NOT be represented as:

all nodes healthy

because only A reported healthy.

---

70. History integration

"history/" stores durable historical records.

Telemetry SHOULD feed:

history::incident
history::execution
history::recovery
history::statistics

History SHOULD retain enough information to support:

trend analysis
recovery effectiveness
failure recurrence
strategy evaluation
capacity planning
learning
post-incident analysis

---

71. History versus telemetry

These must remain distinct.

Telemetry:

high-volume
real-time
operational
possibly sampled

History:

durable
auditable
selected
long-lived

A telemetry exporter failure MUST NOT be confused with historical persistence failure.

---

72. Learning integration

"learning/" may consume observability-derived history.

However:

telemetry
→ learning

MUST NOT become:

unverified telemetry
→ automatic control decision

Only validated/authorized observations should influence learning models.

Learned predictions remain advisory unless policy explicitly allows their use.

---

73. Metrics versus decisions

A metric MUST NOT itself be a policy.

For example:

fidelity = 0.93

does not inherently mean:

recover

The correct chain is:

metric
→ observation
→ detection
→ diagnosis
→ policy
→ plan

This prevents observability from becoming a hidden control subsystem.

---

74. No hard-coded scale

Observability MUST contain no artificial quantum-size ceiling.

Forbidden:

const MAX_QUBITS: usize = 127;
const MAX_DEVICES: usize = 100;
const MAX_EVENTS: usize = 1_000_000;

when these represent architectural limits.

The architecture already defines "infinite scale" as introducing no artificial finite machine-size ceiling; actual execution is bounded only by available resources, policy, memory, time, provider, security, and operating-system constraints.

---

75. Resource-aware observability

Observability itself consumes:

CPU
memory
storage
network bandwidth
I/O
GPU resources where applicable

It MUST therefore be resource-aware.

At large scale, the system SHOULD automatically prefer:

aggregation
sampling
compression
batching
streaming
bounded buffering

according to policy.

---

76. Memory safety at scale

The observability implementation MUST NOT require retaining the complete lifetime of an execution in memory.

Large streams SHOULD be processed incrementally.

Avoid designs such as:

Vec<EveryObservationEverProduced>

without an explicit resource bound.

The system should support streaming or bounded persistence.

---

77. Backpressure hierarchy

A production implementation SHOULD prioritize:

1. security-critical events;
2. semantic verification events;
3. recovery/failure events;
4. resource health events;
5. ordinary execution events;
6. debug/detail events.

Dropping lower-priority telemetry MUST be observable.

---

78. Bounded buffering

Any in-memory buffer MUST have a policy-controlled bound.

The bound MUST come from:

resource policy
memory budget
deployment configuration
runtime availability

not an arbitrary architectural quantum-size constant.

---

79. Compression

Large telemetry streams MAY be compressed.

Compression MUST NOT change semantic content.

Compression metadata SHOULD include:

algorithm identity
version
input schema
integrity

---

80. Batching

Events MAY be batched for efficiency.

Batching MUST preserve:

event identity
ordering information
causality
schema version
integrity

A batch is a transport optimization, not a semantic replacement for individual events.

---

81. Serialization

Observability records MUST use the resilience serialization contract.

Anything required for:

replay
audit
cross-process communication
persistence
distributed tracing

MUST have versioned serialization.

Canonical serialization MUST be deterministic where the object participates in deterministic decisions.

---

82. Schema versioning

Every externally persisted/exported observability schema MUST be versioned.

Schema changes MUST define:

compatibility
migration
unknown fields
removed fields
renamed fields
default semantics

Existing records MUST NOT become unreadable merely because a new telemetry field was added.

---

83. Rust compatibility

All observability implementation code MUST compile on:

Rust 1.97
Rust 1.97.1
Rust 2021

No nightly-only language feature may be required.

No "unsafe" code is permitted.

The subsystem MUST retain:

#![forbid(unsafe_code)]

at the appropriate crate/module boundary.

---

84. Error integration

Telemetry errors MUST integrate with:

quantum::resilience::errors

Telemetry failures should distinguish:

collection failure
serialization failure
buffer exhaustion
integrity failure
export failure
storage failure
schema incompatibility
permission failure
source failure

Telemetry failure MUST NOT be reported as quantum execution failure unless the relevant policy explicitly makes observability safety-critical.

---

85. Error observability

Every resilience error that crosses a production boundary SHOULD generate an observable event.

The event SHOULD include:

stable error code
classification
severity
retryability
execution identity
source
context

Sensitive internal details MUST remain redacted according to security policy.

---

86. Security events

Security-related observability SHOULD include:

untrusted telemetry
integrity failure
authentication failure
authorization failure
tampered checkpoint
invalid provenance
plugin violation
policy violation
unexpected resource transition

Security events MUST be protected from ordinary telemetry dropping where required by policy.

---

87. Audit trail

A production resilience execution SHOULD provide an audit chain:

Execution
  ↓
Observations
  ↓
Incident
  ↓
Diagnosis
  ↓
Policy
  ↓
Plan
  ↓
Actions
  ↓
Verification
  ↓
Final decision

Every link MUST be identifiable.

---

88. Decision explainability

A resilience decision SHOULD be explainable in structured form:

Decision:
    migrate_backend

Reason:
    current_target_unavailable

Evidence:
    observation IDs [...]

Diagnosis:
    backend_unavailable

Confidence:
    [...]

Policy:
    policy hash [...]

Constraints:
    [...]

Alternatives:
    [...]

Selected action:
    migration

Verification:
    required

This information belongs in structured provenance/events, not merely free-form logs.

---

89. Human-readable logs

Human-readable logs MAY exist, but they are not the canonical observability representation.

The canonical representation is structured events/metrics/traces.

Human logs SHOULD reference stable IDs rather than copying large objects.

Example:

execution=...
incident=...
plan=...
recovery=...

---

90. Logging volume

Debug logging MUST NOT be required for correctness.

The system MUST remain operational with verbose logging disabled.

Debug logs may assist diagnosis but MUST NOT be a hidden data dependency.

---

91. No semantic dependence on telemetry exporters

Core resilience decisions MUST NOT depend on whether:

Prometheus
OpenTelemetry
file exporter
cloud exporter
database exporter

is installed.

The internal observability contract must remain provider-neutral.

---

92. Testing requirements

The observability subsystem MUST have dedicated tests.

At minimum:

event model tests
metric tests
trace tests
health tests
collector tests
exporter tests
serialization tests
schema compatibility tests
ordering tests
determinism tests
privacy tests
integrity tests
backpressure tests
sampling tests
large-scale tests
fault-injection tests

---

93. Event tests

Test:

valid event
invalid event
missing identity
unknown resource
unknown severity
unknown confidence
out-of-order events
duplicate events
corrupted event
schema mismatch

---

94. Metric tests

Test:

unit correctness
aggregation
empty input
missing input
NaN
infinity
overflow
underflow
invalid values
high cardinality
large resource counts

Numeric behavior MUST comply with the determinism specification.

---

95. Trace tests

Test:

single trace
nested spans
missing parent
late event
out-of-order event
distributed trace
partial node failure
trace sampling
trace reconstruction

---

96. Backpressure tests

Simulate:

normal rate
high event rate
extreme event rate
slow exporter
failed exporter
full buffer
recovery after exporter failure

Verify that:

critical events remain protected
loss is reported
execution semantics are unaffected

unless policy explicitly couples execution safety to telemetry availability.

---

97. Determinism tests

Given identical:

program identity
IR identity
execution context
resource snapshot
capability snapshot
observations
policy
strategy versions
history
randomness

the observability normalization and deterministic reconstruction MUST produce identical results.

Concurrency MUST NOT change deterministic ordering.

---

98. Scalability tests

The tests MUST be parameterized by discovered/generated resource counts.

They MUST NOT be based only on:

10
100
1000
10000

as architectural limits.

Test the same implementation against increasingly large generated workloads until the available test resources are exhausted.

The test must validate:

no hard-coded qubit ceiling
no fixed topology assumption
no fixed event capacity
no fixed number of devices

---

99. Qubit identity tests

Tests MUST specifically verify that:

QubitId

and:

PhysicalQubitId

remain distinct.

Tests MUST detect accidental conversions or equality assumptions.

Where physical qubits are involved, tests MUST import the canonical types from:

crate::quantum::ir::qubit

rather than creating local replacements.

---

100. Fault-injection observability tests

Fault injection SHOULD cover:

single-qubit fault
multi-qubit fault
correlated fault
leakage
loss
erasure
readout degradation
calibration drift
resource disappearance
routing failure
schedule invalidation
compiler failure
backend outage
timeout
QEC degradation
checkpoint corruption
telemetry failure
export failure

Quantum faults SHOULD use canonical ZQN semantics.

---

101. Replay

Observability MUST support reconstruction of a resilience incident from persisted evidence where sufficient data was retained.

Replay SHOULD reconstruct:

observations
diagnosis
policy
plan
adaptation
verification

without depending on current hardware state.

---

102. Replay versus re-execution

Replay of resilience decisions is different from rerunning the quantum computation.

A replay can reproduce:

decision logic

without reproducing:

physical quantum noise

Observability documentation MUST never imply otherwise.

---

103. Incident reconstruction

An incident reconstruction SHOULD answer:

What was running?
Where was it running?
What changed?
What was observed?
What was trusted?
What failed?
Why was it diagnosed that way?
Why was the recovery chosen?
What changed physically?
Was semantics preserved?
Was the result verified?

---

104. Observability completeness

The system SHOULD be able to classify an execution as:

fully_observed
partially_observed
observation_degraded
observation_unavailable

A partially observed execution MUST NOT automatically be treated as fully observed.

---

105. Observation confidence

The system SHOULD expose aggregate confidence carefully.

It MUST NOT compute:

average confidence

and treat that as proof that all observations are reliable.

Critical evidence may require individual confidence analysis.

---

106. Health aggregation

When aggregating health:

qubit health
→ region health
→ device health
→ backend health

the aggregation rule MUST be explicitly defined.

Do not assume:

one failed qubit = failed device

or:

99% healthy qubits = healthy device

unless policy/capability semantics define that relationship.

---

107. Degraded states

Observability MUST support partial degradation.

Example:

resource capacity:
100%
  ↓
96%
  ↓
87%
  ↓
73%

The telemetry model must represent the current capability state without assuming that any particular percentage is universally acceptable.

Policy decides acceptability.

---

108. Topology changes

A topology change MUST be observable as a versioned resource/capability transition.

Example:

topology_version A
       ↓
fault/resource loss
       ↓
topology_version B

Routing and resilience can then correlate rerouting decisions with the topology transition.

---

109. Calibration changes

Calibration observations SHOULD identify:

calibration identity
version
timestamp
affected resources
validity interval
source

Resilience can then establish whether an execution used:

calibration A

or:

calibration B

without embedding calibration semantics in telemetry.

---

110. Backend migration

A migration trace MUST preserve:

source backend identity
source capability snapshot
destination backend identity
destination capability snapshot
reason
program identity
IR identity
adaptation identity
verification result

This is necessary for write-once/run-anywhere provenance.

---

111. Simulator observability

Simulation MUST use the same observability contract where possible.

A simulator should produce equivalent structural telemetry:

execution
fault
diagnosis
plan
recovery
verification

This enables resilience testing without physical quantum hardware.

---

112. Benchmarking integration

Benchmarking may consume observability data for:

latency
reliability
recovery overhead
resource efficiency
mitigation overhead
verification cost

Observability MUST NOT redefine benchmark methodology.

The benchmarking subsystem remains authoritative for benchmark semantics.

---

113. Resource estimation integration

Resource estimation SHOULD expose estimates into observability:

estimated qubits
estimated depth
estimated execution time
estimated shots
estimated memory
estimated communication

Observed values should remain distinguishable from estimates.

---

114. Estimate versus measurement

Never represent:

estimated_time

as:

time

without qualification.

Observability MUST preserve:

estimated
measured
predicted
simulated
observed

as distinct semantic states.

---

115. Learning feedback

Verified outcomes MAY be fed into learning.

The telemetry chain should preserve:

prediction
actual observation
verification outcome
strategy outcome

so that learning systems can determine whether a prediction was actually correct.

---

116. No hidden feedback loop

Telemetry MUST NOT silently become a control loop.

The explicit control chain remains:

observation
→ detection
→ diagnosis
→ policy
→ planning
→ action

not:

metric
→ arbitrary callback
→ hardware mutation

---

117. Operational dashboards

A deployment MAY expose dashboards, but dashboards are consumers of the observability contract.

The core subsystem MUST NOT depend on a dashboard.

Recommended dashboard dimensions:

fleet health
backend health
device health
execution success
verification success
incident rate
recovery success
resource degradation
telemetry health

---

118. Quantum-resource dashboards

Large-scale dashboards SHOULD aggregate resources.

They should not require displaying every qubit simultaneously.

Possible views:

fleet summary
device summary
region summary
fault hotspots
degraded-resource distribution
logical error distribution

Detailed qubit-level information can be requested on demand.

---

119. Alerting

Alerting belongs above the observability layer.

Telemetry may expose:

threshold breach
critical event
health transition
integrity failure
verification failure

An alerting system determines notification behavior.

The core resilience module MUST NOT require a specific alerting service.

---

120. Observability policy

A production deployment SHOULD allow configuration of:

event level
metric level
trace level
sampling
retention
privacy
redaction
buffering
export
compression
durability

Configuration must be validated and included in provenance when it affects deterministic behavior.

---

121. Retention

Retention MUST be policy-driven.

Possible retention classes:

ephemeral
short-term
operational
audit
long-term

Quantum program/result data may require different retention policies from ordinary metrics.

---

122. Storage

The telemetry subsystem MUST use a storage abstraction.

It MUST NOT assume:

local disk
cloud database
specific SQL database
specific cloud

Storage backends belong behind interfaces.

---

123. Offline operation

Observability SHOULD support environments without network access.

A local collector may:

buffer
persist
aggregate
later export

When export becomes available.

This aligns with the broader Zamani architecture's offline-capable design goals.

---

124. Storage exhaustion

If local observability storage reaches capacity, behavior MUST be policy-defined.

Possible responses:

rotate
compress
aggregate
drop low-priority events
pause noncritical tracing
escalate
fail closed

Critical audit requirements may require fail-closed behavior.

---

125. Clock uncertainty

Distributed observations SHOULD preserve clock uncertainty where applicable.

Do not manufacture precise global event ordering from unsynchronized clocks.

Use causal identifiers and logical ordering where required.

---

126. Duplicate events

At-least-once delivery may create duplicates.

Consumers MUST be able to identify duplicates using stable event identity.

Deduplication MUST NOT accidentally delete genuinely distinct events.

---

127. Idempotency

Persistence/export operations SHOULD be idempotent where practical.

For example:

store(EventId)

repeatedly should not create multiple semantic copies of the same immutable event.

---

128. Recovery of observability itself

The observability subsystem must itself be resilient.

It SHOULD support:

collector restart
buffer recovery
export retry
storage recovery
schema recovery
partial exporter failure

Observability recovery MUST remain separate from quantum computation recovery.

---

129. Meta-observability

The system MUST observe itself.

At minimum:

telemetry throughput
telemetry latency
buffer utilization
drop rate
export failure rate
serialization failure rate
storage failure rate
collector health

Otherwise observability failures can remain invisible.

---

130. Resource overhead

Every observability component SHOULD expose or estimate its own overhead.

Examples:

CPU overhead
memory overhead
storage overhead
network overhead
latency overhead

The system MUST be able to reduce observability detail when resource policy requires it.

---

131. Observability overhead versus correctness

Observability optimization MUST NOT remove information required for:

semantic verification
security
audit
recovery correctness
deterministic replay

unless the relevant policy explicitly permits reduced guarantees.

---

132. Privacy versus debugging

Detailed telemetry MAY improve debugging but increase privacy/security risk.

Therefore:

maximum debugging detail

MUST NOT automatically mean:

maximum production safety

Production defaults should favor minimum necessary data.

---

133. External observability adapters

Future integrations may include:

metrics systems
distributed tracing systems
SIEM systems
audit systems
local files
databases
message queues

These must be adapters/exporters.

Core resilience must remain independent of them.

---

134. Plugin safety

Observability exporters/plugins MUST NOT receive unrestricted access to:

credentials
raw quantum state
private keys
unredacted results
internal recovery state

unless explicitly authorized.

Plugin interfaces must expose the minimum required information.

---

135. Versioned source identity

A source should identify:

component
implementation
version
configuration identity

For example:

detector identity
detector version
hardware adapter identity
hardware adapter version

This allows historical results to remain explainable after software updates.

---

136. Configuration identity

Configurations affecting observability SHOULD be content-addressed or otherwise stably identified.

Examples:

sampling configuration
retention configuration
redaction policy
export configuration
metric configuration
trace configuration

---

137. Decision provenance

Observability MUST integrate with:

verification/provenance.rs

The provenance record should connect:

ProgramHash
IRHash
ExecutionId
CapabilitySnapshotHash
ObservationSnapshotHash
PolicyHash
StrategyIdentity
PlanHash
RecoveryId
ResultHash
VerificationResult

The exact structure belongs to the provenance module.

Telemetry should reference it.

---

138. No duplicated provenance authority

Do not create:

TelemetryProvenance

as a second authoritative provenance model if "verification/provenance.rs" already owns that responsibility.

Telemetry records provenance references.

---

139. Compatibility

Observability must integrate with:

COMPATIBILITY.md

Schema compatibility MUST be defined for:

event
metric
trace
health
checkpoint
provenance
history

Old telemetry MUST remain interpretable where compatibility policy requires it.

---

140. Architecture integration

The complete integration path is:

quantum::ir
      │
      ├── canonical program/IR identity
      │
      ▼
compiler / optimization
      │
      ▼
routing
      │
      ▼
scheduling
      │
      ▼
hardware/runtime
      │
      ├───────────────┐
      │               │
      ▼               ▼
execution          telemetry
      │               │
      ▼               ▼
results          observations
      │               │
      └───────┬───────┘
              ▼
       quantum::resilience
              │
      ┌───────┼────────┐
      ▼       ▼        ▼
 detection diagnosis verification
      │       │        │
      └───────┼────────┘
              ▼
          planning
              │
              ▼
          adaptation
              │
              ▼
           recovery
              │
              ▼
         verification
              │
              ▼
          provenance
              │
              ▼
           history

---

141. Integration with "api/controller.rs"

The controller is the top-level orchestration boundary.

It SHOULD emit lifecycle events for:

request received
execution started
resilience cycle started
resilience cycle completed
final decision

The controller MUST NOT manually construct provider-specific telemetry.

---

142. Integration with "model/*"

Model objects provide:

fault
incident
severity
health
degradation
capability
resource
confidence

Telemetry records these semantics.

It MUST NOT redefine them.

---

143. Integration with "detection/*"

Each detector SHOULD identify itself and produce structured evidence.

Telemetry captures:

detector
input
output
confidence
time
scope

---

144. Integration with "diagnosis/*"

Diagnosis MUST emit enough structured information for post-incident reconstruction.

Telemetry records references rather than unnecessarily duplicating large evidence structures.

---

145. Integration with "policy/*"

Policy identity and effective configuration MUST be observable.

A policy change during execution MUST create an observable state transition if it affects the resilience decision.

---

146. Integration with "planning/*"

Every selected recovery plan MUST be identifiable.

The plan should reference:

incident
diagnosis
policy
capabilities
constraints
objectives
budget

---

147. Integration with "adaptation/*"

Adaptation events MUST identify the affected artifact and resulting artifact.

For example:

IR_A
→ rerouting
→ mapped IR_B

The identity relationship must remain explicit.

---

148. Integration with "recovery/*"

Recovery actions MUST emit lifecycle events.

A recovery completion event is not equivalent to verification success.

Both MUST be separately represented.

---

149. Integration with "mitigation/*"

Mitigation strategy and configuration MUST be observable.

Randomized mitigation MUST record its randomness provenance where required.

---

150. Integration with "verification/*"

Verification is mandatory for acceptance.

Observability MUST capture the final verification state.

---

151. Integration with "state/*"

State transitions SHOULD emit events.

For example:

Idle
→ Detecting
→ Diagnosing
→ Planning
→ Adapting
→ Recovering
→ Verifying
→ Completed

Invalid state transitions MUST be observable as errors.

---

152. Integration with "checkpoint/*"

Checkpoint operations SHOULD emit:

checkpoint.created
checkpoint.validated
checkpoint.persisted
checkpoint.restored
checkpoint.rejected
checkpoint.integrity_failed

Checkpoint integrity failures are security/recovery-significant events.

---

153. Integration with "history/*"

History consumes selected durable observability records.

History SHOULD NOT require every debug event.

---

154. Integration with "learning/*"

Learning consumes verified historical observations.

It SHOULD retain links between:

prediction
observation
decision
outcome
verification

---

155. Integration with "coordination/*"

Distributed coordination SHOULD emit:

ownership acquired
lease acquired
lease lost
coordination conflict
node unavailable
consensus transition

The observability model must remain provider-independent.

---

156. Integration with "registry/*"

Registries SHOULD expose stable strategy identities.

Telemetry must record strategy identity/version when a registered detector, mitigation, recovery strategy, or backend adapter participates in a decision.

---

157. Integration with "limits/*"

Observability SHOULD expose:

limit evaluated
resource budget
remaining budget
limit violation

The actual limits remain owned by the limits/policy contracts.

---

158. Integration with "serialization/*"

Every persisted/exported object MUST use versioned serialization.

Deterministic serialization is required for objects participating in deterministic identity or replay.

---

159. Integration with "errors/*"

Errors MUST retain stable error codes and classifications.

Telemetry SHOULD reference the stable code instead of depending on free-form error strings.

---

160. Integration with "SCALABILITY.md"

The observability implementation MUST comply with the scalability rule:

«The resilience subsystem introduces no artificial finite machine-size ceiling.»

This includes telemetry itself.

It is not sufficient for execution to scale if observability crashes because the number of qubits, events, devices, or traces exceeds a fixed internal array.

---

161. Integration with "DETERMINISM.md"

The observability subsystem MUST preserve:

input closure
canonical ordering
stable identifiers
explicit randomness
deterministic serialization
concurrency-independent reconstruction

Observability MUST never become a hidden nondeterministic input.

---

162. Integration with "SECURITY.md"

Security policy governs:

authentication
authorization
redaction
integrity
retention
plugin access
sensitive state
audit records

Observability implements the required hooks and metadata.

---

163. Integration with "FAILURE_MODES.md"

Every documented failure mode SHOULD map to at least one observable representation.

For example:

hardware failure
→ hardware.failure event

routing failure
→ routing.failure event

telemetry failure
→ telemetry.failure event

verification failure
→ verification.failed event

No critical failure mode should be operationally invisible.

---

164. Integration with "RECOVERY_MODEL.md"

Recovery state transitions MUST be observable.

The telemetry trace should correspond to the recovery state machine without changing its semantics.

---

165. Production event lifecycle

A canonical execution should produce an evidence flow similar to:

execution.started
      ↓
compile.completed
      ↓
route.completed
      ↓
schedule.completed
      ↓
execution.submitted
      ↓
execution.started
      ↓
observation.received
      ↓
fault.detected
      ↓
incident.created
      ↓
diagnosis.completed
      ↓
plan.created
      ↓
adaptation.started
      ↓
adaptation.completed
      ↓
recovery.started
      ↓
recovery.completed
      ↓
execution.completed
      ↓
verification.completed
      ↓
execution.accepted

Not every execution produces every event.

The lifecycle is conditional.

---

166. Normal execution

A healthy execution may be:

request
→ compile
→ route
→ schedule
→ execute
→ verify
→ accept

Resilience telemetry must not require a fault to exist.

---

167. Failed execution

A failed execution may be:

execute
→ failure
→ detect
→ diagnose
→ plan
→ recover
→ verify
→ accept

or:

execute
→ failure
→ diagnose
→ no safe recovery
→ reject

Both must be observable.

---

168. Multiple recovery cycles

An execution may have:

incident 1
→ recovery 1
→ verification

incident 2
→ recovery 2
→ verification

incident 3
→ escalation

The observability model MUST support arbitrary numbers of resilience cycles.

No fixed recovery-attempt count is permitted.

---

169. Infinite-scale semantic requirement

The phrase "infinite scale" means:

no artificial finite architecture limit

It does NOT mean:

unbounded memory
unbounded storage
unbounded network
unbounded hardware

Observability MUST gracefully operate up to the resources actually supplied.

---

170. Resource exhaustion behavior

If observability resources are exhausted, the system MUST follow configured policy.

It MUST NOT:

panic
corrupt state
silently drop critical evidence
change quantum semantics

unless explicitly specified by a controlled failure policy.

---

171. Panic avoidance

Production observability code SHOULD avoid uncontrolled panics from:

malformed telemetry
unknown schema
resource exhaustion
export failure
external input

Errors should use the resilience error contract.

---

172. No "unsafe"

No observability implementation may use:

unsafe

or rely on unsafe FFI internally.

External integrations must remain behind safe abstractions.

---

173. Thread safety

Shared observability state MUST use safe Rust synchronization primitives.

There must be no hidden mutable global telemetry state.

The preferred design is dependency injection of:

collector
sink
storage
clock
configuration

where required.

---

174. Testability

Every observability component SHOULD be testable without a real QPU.

Use:

mock collector
mock exporter
synthetic events
synthetic hardware snapshots
synthetic ZQN faults
deterministic clocks
deterministic randomness

This permits exhaustive resilience testing.

---

175. Deterministic clock injection

Where timing affects testable logic, tests SHOULD inject an explicit clock abstraction.

Production code MUST NOT make deterministic behavior depend on:

SystemTime::now()

inside strict deterministic decision paths.

---

176. Test event sources

Tests should be able to generate:

single event
event stream
out-of-order stream
duplicated stream
corrupted stream
high-volume stream
distributed stream

---

177. Property testing

Observability should use property-based tests where appropriate.

Properties include:

serialization round trip
identity preservation
ordering stability
deduplication stability
no event corruption
no qubit identity collision
bounded-memory behavior

---

178. Fuzzing

Fuzz:

event decoding
metric decoding
trace decoding
schema versions
malformed source data
unknown enum values
large collections
large payload metadata

The fuzz target MUST remain safe Rust.

---

179. Compatibility testing

Test telemetry generated by:

current version
previous supported version
future-compatible unknown fields

where compatibility policy requires.

---

180. Replay tests

Persist a deterministic observation sequence.

Then verify:

original sequence
=
reconstructed sequence

where canonical replay is supported.

---

181. Security testing

Test:

tampered event
forged source
invalid provenance
unauthorized exporter
redaction bypass
malformed schema
malicious high-cardinality input
resource exhaustion

---

182. Observability acceptance criteria

"quantum::resilience" MUST NOT be considered production-ready until:

- every resilience-critical decision has identifiable evidence;
- events have stable identity;
- logical and physical qubit identities remain canonical;
- high-cardinality telemetry does not become unbounded metric cardinality;
- telemetry loss is itself observable;
- collector/exporter failure is isolated;
- deterministic reconstruction is supported where required;
- sensitive data can be redacted;
- telemetry has schema versioning;
- critical records have appropriate integrity guarantees;
- distributed events can be correlated;
- observability overhead is resource-aware;
- no fixed quantum-machine size is embedded;
- no "unsafe" is used;
- Rust 1.97/1.97.1 compatibility is maintained;
- fault injection covers observability failure;
- large-scale tests pass;
- verification/provenance integration is complete.

---

183. Required implementation files

"telemetry/mod.rs"

Must:

- declare only existing telemetry modules;
- expose stable public telemetry contracts;
- avoid business logic;
- avoid global mutable state.

Integration:

resilience/mod.rs
→ telemetry/mod.rs

---

"telemetry/event.rs"

Must own:

- canonical event structure;
- event identity;
- event type;
- schema version;
- timestamp representation;
- source;
- resource scope;
- severity;
- confidence;
- causal references;
- payload abstraction;
- integrity metadata.

Must use canonical quantum identities when qubits are referenced.

Integration:

model/*
detection/*
diagnosis/*
planning/*
adaptation/*
recovery/*
verification/*

---

"telemetry/metric.rs"

Must own:

- metric identity;
- type;
- unit;
- scope;
- aggregation semantics;
- validity;
- cardinality metadata.

Must not own resilience decisions.

---

"telemetry/trace.rs"

Must own:

- trace identity;
- span identity;
- parent-child relationships;
- causal links;
- bounded attributes;
- status;
- timing.

Must support distributed execution.

---

"telemetry/health.rs"

Must own observability-facing health representation for:

collector
buffer
exporter
storage
source

It may consume the canonical resilience health model.

---

"telemetry/collector.rs"

Must own:

- collection;
- normalization;
- validation;
- source isolation;
- backpressure;
- sampling;
- loss accounting.

It must not own recovery decisions.

---

"telemetry/exporter.rs"

Must own:

- external export abstraction;
- exporter lifecycle;
- export failure isolation;
- batching;
- optional compression;
- retry behavior;
- exporter health.

It must remain provider-neutral.

---

184. Implementation dependency order

To satisfy the requirement that each file can be completed without later redesign, implement contracts in this order:

1. model/resource.rs
2. model/confidence.rs
3. model/severity.rs
4. model/health.rs
5. model/fault.rs
6. model/incident.rs

7. serialization/version.rs
8. serialization/schema.rs
9. serialization/encode.rs
10. serialization/decode.rs

11. telemetry/event.rs
12. telemetry/metric.rs
13. telemetry/trace.rs
14. telemetry/health.rs

15. telemetry/collector.rs
16. telemetry/exporter.rs

17. verification/provenance.rs
18. history/incident.rs
19. history/execution.rs
20. history/recovery.rs
21. history/statistics.rs

22. telemetry/mod.rs
23. resilience/mod.rs

24. integration tests
25. determinism tests
26. scalability tests
27. fault-injection tests
28. end-to-end tests

The important dependency rule is:

domain contracts
    ↓
serialization contracts
    ↓
telemetry structures
    ↓
collection/export
    ↓
history/provenance
    ↓
integration

This prevents telemetry files from having to be redesigned later because a foundational identity or schema was changed.

---

185. Final observability invariant

The production invariant is:

«If a resilience decision can materially affect quantum execution, the system must be able to identify the decision, its causal context, its relevant observations, its policy, its selected action, and its verification outcome, subject to explicit privacy, security, retention, and resource policies.»

The second invariant is:

«Observability must never silently become a source of semantic change.»

The third invariant is:

«Observability must scale with the available resources without introducing an artificial finite quantum-machine size.»

The fourth invariant is:

«The canonical quantum identities remain "crate::quantum::ir::qubit::QubitId" and "crate::quantum::ir::qubit::PhysicalQubitId"; observability must not create competing quantum identities.»

The fifth invariant is:

«Telemetry loss, uncertainty, sampling, redaction, and incomplete observation must themselves be represented rather than hidden.»

The sixth invariant is:

«Deterministic resilience decisions must remain deterministic regardless of telemetry arrival order, concurrency, exporter configuration, or incidental collection ordering.»

The final execution model is therefore:

                    ZAMANI PROGRAM
                          │
                          ▼
                   CANONICAL IR
                          │
          ┌───────────────┼────────────────┐
          ▼               ▼                ▼
     Optimization       Routing        Scheduling
          │               │                │
          └───────────────┼────────────────┘
                          ▼
                    EXECUTION
                          │
              ┌───────────┴───────────┐
              ▼                       ▼
        QUANTUM RESULT           OBSERVATIONS
                                      │
                         ┌────────────┼────────────┐
                         ▼            ▼            ▼
                      Events       Metrics       Traces
                         │            │            │
                         └────────────┼────────────┘
                                      ▼
                                 DETECTION
                                      │
                                  DIAGNOSIS
                                      │
                                   POLICY
                                      │
                                   PLANNING
                                      │
                                  ADAPTATION
                                      │
                                   RECOVERY
                                      │
                                  MITIGATION
                                      │
                                 VERIFICATION
                                      │
                         ┌────────────┴────────────┐
                         ▼                         ▼
                     PROVENANCE                  HISTORY
                         │
                         ▼
                  ACCEPT / DEGRADED /
                  RETRY / REPLAN /
                  ESCALATE / REJECT

The resulting architecture makes observability an evidence system rather than merely a logging system.

That distinction is essential for production-grade autonomous quantum resilience.This is designed to fit the repository's existing architecture rather than create a parallel observability model. In particular, the existing resilience architecture already establishes canonical quantum::ir::qubit identities, provider-neutral ownership boundaries, and explicit scalability/determinism requirements.