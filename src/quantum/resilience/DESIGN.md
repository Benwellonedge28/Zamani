Worked for 26s

I checked the current repository rather than designing DESIGN.md in isolation. In particular, the existing canonical IR explicitly owns quantum::ir::qubit::{QubitId, PhysicalQubitId}, the hardware layer is provider-neutral and already separates technology/backend/workload, scheduling owns timing rather than semantics, optimization operates on canonical IR, and ZQN owns the canonical realized-fault model.

I also accounted for the current resilience/errors/error.rs contract: it already establishes stable error semantics, canonical qubit identities, no unsafe, Rust 1.97/1.97.1 compatibility, and independence from the rest of resilience.  Current quantum practice also reinforces separating resilience orchestration from mitigation mechanisms, while monitoring/calibration/benchmarking are continuous inputs to adaptive execution. 

Below is the complete src/quantum/resilience/DESIGN.md.

Zamani Quantum Resilience — Production Design

Path: "src/quantum/resilience/DESIGN.md"
Status: Normative architecture
Target: Rust 1.97 / Rust 1.97.1, Rust 2021
Safety: "unsafe" forbidden
Scope: Provider-neutral, hardware-independent, fault-aware, self-adaptive quantum execution resilience

---

1. Purpose

"quantum::resilience" is the execution-resilience subsystem of the Zamani quantum computing stack.

Its purpose is to allow a Zamani quantum program to be written against the logical semantics of the computation while the execution system dynamically responds to:

- quantum faults;
- hardware degradation;
- calibration drift;
- resource loss;
- routing changes;
- scheduling changes;
- backend failures;
- execution failures;
- QEC degradation;
- mitigation requirements;
- checkpoint/recovery requirements;
- distributed execution failures;
- capability changes;
- software failures;
- security failures;
- uncertain or conflicting observations.

The defining architectural objective is:

«A Zamani quantum program is written once against the canonical semantic model and remains semantically stable while the physical execution strategy changes according to available resources and verified runtime conditions.»

This means resilience must not become a collection of provider-specific retry mechanisms.

It is a decision, adaptation, recovery, and verification layer around the existing quantum subsystems.

---

2. Core architectural principle

The resilience subsystem follows:

                    Zamani Program
                          |
                          v
                  quantum::frontend
                          |
                          v
                    quantum::ir
                          |
             +------------+-------------+
             |            |             |
             v            v             v
       optimization     QEC           ZQN
             |            |             |
             +------------+-------------+
                          |
                          v
                       routing
                          |
                          v
                     scheduling
                          |
                          v
                    hardware HAL
                          |
                          v
                      execution
                          |
             +------------+-------------+
             |                          |
             v                          v
         observations                results
             |                          |
             +------------+-------------+
                          |
                          v
                  quantum::resilience
                          |
       +------------------+------------------+
       |                  |                  |
       v                  v                  v
    continue            adapt            recover
       |                  |                  |
       |          +-------+-------+          |
       |          |       |       |          |
       |          v       v       v          |
       |        route   schedule compile     |
       |                                      |
       +------------------+-------------------+
                          |
                          v
                      execute
                          |
                          v
                      verify
                          |
                    +-----+-----+
                    |           |
                    v           v
                  accept      repeat/escalate

The feedback loop is fundamental.

---

3. What resilience owns

"quantum::resilience" owns:

1. normalized resilience observations;
2. resilience-level incidents;
3. diagnosis;
4. resilience policy evaluation;
5. recovery planning;
6. adaptation orchestration;
7. recovery orchestration;
8. error mitigation orchestration;
9. recovery-state management;
10. checkpoint coordination;
11. verification gates;
12. provenance of resilience decisions;
13. resilience telemetry contracts;
14. recovery history;
15. optional predictive learning;
16. distributed resilience coordination;
17. resilience-specific serialization contracts;
18. resilience error contracts;
19. resilience registries;
20. resource-aware resilience limits.

It does not own the underlying quantum semantics.

---

4. What resilience must never own

The following remain authoritative elsewhere.

Responsibility| Owner
Quantum program semantics| "quantum::ir"
Logical/physical qubit identity| "quantum::ir::qubit"
Realized fault semantics| "quantum::zqn::fault"
Quantum error correction| "quantum::error_correction" / QEC subsystem
Routing| "quantum::routing"
Scheduling| "quantum::scheduling"
Optimization| "quantum::optimization"
Hardware capabilities| "quantum::hardware"
Hardware execution| "quantum::hardware" / runtime
Simulation| simulator subsystem
Benchmark methodology| benchmarking subsystem
Source parsing| frontend
Provider SDK behavior| hardware adapters
Credentials| hardware authentication boundary
Quantum channel semantics| ZQN/noise subsystem

Resilience orchestrates these systems.

It does not duplicate them.

---

5. Canonical identity rule

All resilience code requiring quantum qubit identity MUST use:

crate::quantum::ir::qubit::QubitId
crate::quantum::ir::qubit::PhysicalQubitId

No resilience module may introduce:

ResilienceQubitId
FaultQubitId
RecoveryQubitId
LogicalQubitId
PhysicalQubit
ResiliencePhysicalQubitId

or another competing identity abstraction.

The distinction is:

QubitId
    canonical quantum-program identity

PhysicalQubitId
    canonical physical-target identity

Resource identity
    identity of an abstract execution resource

OperationId
    canonical quantum operation identity

IncidentId
    resilience incident identity

RecoveryId
    recovery attempt identity

CheckpointId
    checkpoint identity

A resilience identifier must never replace a canonical quantum identifier.

---

6. Logical versus physical identity

The following conversion is explicitly prohibited inside resilience:

QubitId -> PhysicalQubitId

unless the operation is performed through an explicit routing/mapping contract.

The correct flow is:

QubitId
   |
   v
routing
   |
   v
PhysicalQubitId

Resilience may request remapping.

It must not invent the mapping algorithm.

---

7. Meaning of "scale from atom to everywhere"

"Infinity" is interpreted architecturally, not literally.

The system must not impose an artificial finite machine-size ceiling.

It must instead scale according to available resources.

The four dimensions are:

Computational scale
    one operation
    -> small circuit
    -> large circuit
    -> distributed workload

Physical scale
    microscopic device
    -> QPU
    -> multiple QPUs
    -> distributed quantum system

Logical scale
    physical qubits
    -> encoded qubits
    -> logical qubits
    -> fault-tolerant computation

Organizational scale
    one backend
    -> multiple devices
    -> heterogeneous fleet
    -> distributed execution fabric

No resilience file may encode a machine-size assumption.

---

8. Forbidden scalability patterns

The following are forbidden in production resilience code:

const MAX_QUBITS: usize = 127;

const MAX_PHYSICAL_QUBITS: usize = 1000;

if qubit_id == 127 {
    ...
}

for _ in 0..3 {
    retry();
}

if fidelity < 0.99 {
    recover();
}

match backend {
    Backend::SpecificProvider => ...
}

when used as a core resilience decision.

Instead use:

capabilities
policy
constraints
budgets
resource models
target metadata
runtime configuration
discovered health
verified observations

Concrete limits belong to explicit resource or policy contracts.

---

9. "Infinity" and finite execution

The architectural rule is:

«No artificial finite limit.»

It does not mean a process can allocate infinite memory or execute infinitely large circuits.

Actual execution is bounded by:

- available memory;
- CPU/GPU capacity;
- storage;
- target capacity;
- distributed capacity;
- execution deadline;
- user policy;
- security policy;
- backend limits;
- compiler limits;
- operating-system limits.

These constraints must be represented explicitly rather than disguised as semantic limits.

---

10. Resilience lifecycle

The canonical lifecycle is:

EXECUTION
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
ADAPT
    |
    v
RECOVER
    |
    v
VERIFY
    |
    +-----------> ACCEPT
    |
    +-----------> REPEAT
    |
    +-----------> ESCALATE
    |
    +-----------> REJECT

No stage may silently bypass verification.

---

11. Self-healing definition

Zamani self-healing is:

observe
-> identify
-> constrain
-> plan
-> adapt
-> recover
-> verify

It is not:

error
-> blindly retry

Nor:

error
-> hide error
-> return result

Nor:

error
-> change computation
-> claim success

---

12. Self-healing safety invariant

The primary invariant is:

«No recovery action may be accepted solely because it increases availability.»

An action must satisfy:

Semantic validity
+
Policy validity
+
Capability validity
+
Security validity
+
Verification validity

before its result may be accepted.

---

13. Architectural dependency direction

The dependency direction is:

quantum::ir
      |
      +----> ZQN
      |
      +----> optimization
      |
      +----> QEC
      |
      +----> routing
      |
      +----> scheduling
      |
      +----> hardware
      |
      +----> resilience

Resilience consumes contracts from those systems.

The underlying systems must not depend on concrete resilience implementations.

This prevents circular dependencies.

---

14. Resilience dependency graph

                         quantum::ir
                              |
             +----------------+----------------+
             |                |                |
             v                v                v
            ZQN             QEC        optimization
             |                |                |
             +----------------+----------------+
                              |
                    +---------+---------+
                    |                   |
                    v                   v
                  routing          scheduling
                    |                   |
                    +---------+---------+
                              |
                              v
                         hardware HAL
                              |
                              v
                          execution
                              |
                              v
                       observations
                              |
                              v
                     quantum::resilience
                              |
          +-------------------+-------------------+
          |                   |                   |
          v                   v                   v
      detection           diagnosis           policy
          |                   |                   |
          +-------------------+-------------------+
                              |
                              v
                           planning
                              |
                              v
                          adaptation
                              |
                              v
                           recovery
                              |
                              v
                         verification

---

15. Independent-first implementation rule

Every resilience file must be designed as a stable contract before dependent implementations are written.

The implementation order exists specifically to prevent:

file A implemented
file B implemented
file C forces redesign of A

Instead:

contract A
contract B
contract C
        |
        v
freeze interfaces
        |
        v
implement consumers

A file is considered complete only when:

1. its responsibility is unambiguous;
2. its public types are stable;
3. its dependency direction is stable;
4. its error contract is stable;
5. its scalability behavior is defined;
6. its determinism behavior is defined;
7. its security boundary is defined;
8. its serialization requirements are defined;
9. its integration points are documented;
10. its tests are defined;
11. it does not depend on a future implementation detail.

---

16. Directory architecture

The production structure is:

src/quantum/resilience/
├── mod.rs
├── README.md
├── ARCHITECTURE.md
├── DESIGN.md
├── SECURITY.md
├── SCALABILITY.md
├── COMPATIBILITY.md
├── DETERMINISM.md
├── FAILURE_MODES.md
├── RECOVERY_MODEL.md
├── OBSERVABILITY.md
│
├── api/
│   ├── mod.rs
│   ├── controller.rs
│   ├── request.rs
│   ├── response.rs
│   └── context.rs
│
├── model/
│   ├── mod.rs
│   ├── fault.rs
│   ├── incident.rs
│   ├── severity.rs
│   ├── health.rs
│   ├── degradation.rs
│   ├── capability.rs
│   ├── resource.rs
│   └── confidence.rs
│
├── detection/
│   ├── mod.rs
│   ├── detector.rs
│   ├── anomaly.rs
│   ├── threshold.rs
│   ├── statistical.rs
│   ├── drift.rs
│   ├── timeout.rs
│   ├── execution_failure.rs
│   ├── qec_signal.rs
│   └── hardware_signal.rs
│
├── diagnosis/
│   ├── mod.rs
│   ├── diagnostician.rs
│   ├── classifier.rs
│   ├── root_cause.rs
│   ├── correlation.rs
│   ├── localization.rs
│   └── confidence.rs
│
├── policy/
│   ├── mod.rs
│   ├── policy.rs
│   ├── constraints.rs
│   ├── objectives.rs
│   ├── budgets.rs
│   ├── escalation.rs
│   ├── retry.rs
│   └── safety.rs
│
├── planning/
│   ├── mod.rs
│   ├── planner.rs
│   ├── action.rs
│   ├── plan.rs
│   ├── cost.rs
│   ├── feasibility.rs
│   ├── ranking.rs
│   └── planner_state.rs
│
├── adaptation/
│   ├── mod.rs
│   ├── adapter.rs
│   ├── remapping.rs
│   ├── rerouting.rs
│   ├── rescheduling.rs
│   ├── recompilation.rs
│   ├── reoptimization.rs
│   ├── qec_adaptation.rs
│   └── backend_selection.rs
│
├── recovery/
│   ├── mod.rs
│   ├── recoverer.rs
│   ├── retry.rs
│   ├── restart.rs
│   ├── checkpoint.rs
│   ├── rollback.rs
│   ├── resume.rs
│   ├── migration.rs
│   └── compensation.rs
│
├── mitigation/
│   ├── mod.rs
│   ├── strategy.rs
│   ├── executor.rs
│   ├── selection.rs
│   ├── readout.rs
│   ├── zero_noise.rs
│   ├── probabilistic.rs
│   ├── twirling.rs
│   ├── dynamical_decoupling.rs
│   └── custom.rs
│
├── verification/
│   ├── mod.rs
│   ├── verifier.rs
│   ├── invariant.rs
│   ├── semantic.rs
│   ├── result.rs
│   ├── confidence.rs
│   ├── provenance.rs
│   └── acceptance.rs
│
├── state/
│   ├── mod.rs
│   ├── machine.rs
│   ├── execution.rs
│   ├── logical.rs
│   ├── physical.rs
│   ├── recovery.rs
│   └── persistence.rs
│
├── checkpoint/
│   ├── mod.rs
│   ├── checkpoint.rs
│   ├── snapshot.rs
│   ├── manifest.rs
│   ├── storage.rs
│   ├── integrity.rs
│   └── compatibility.rs
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
├── learning/
│   ├── mod.rs
│   ├── model.rs
│   ├── features.rs
│   ├── predictor.rs
│   ├── strategy.rs
│   └── feedback.rs
│
├── coordination/
│   ├── mod.rs
│   ├── coordinator.rs
│   ├── distributed.rs
│   ├── lease.rs
│   ├── ownership.rs
│   └── consensus.rs
│
├── serialization/
│   ├── mod.rs
│   ├── schema.rs
│   ├── encode.rs
│   ├── decode.rs
│   └── version.rs
│
├── errors/
│   ├── mod.rs
│   ├── error.rs
│   ├── codes.rs
│   └── classification.rs
│
├── limits/
│   ├── mod.rs
│   ├── limits.rs
│   ├── resource.rs
│   └── validation.rs
│
├── registry/
│   ├── mod.rs
│   ├── detector.rs
│   ├── strategy.rs
│   ├── recovery.rs
│   └── backend.rs
│
└── tests/
    ├── mod.rs
    ├── model.rs
    ├── detection.rs
    ├── diagnosis.rs
    ├── planning.rs
    ├── adaptation.rs
    ├── recovery.rs
    ├── mitigation.rs
    ├── verification.rs
    ├── checkpoint.rs
    ├── serialization.rs
    ├── determinism.rs
    ├── scalability.rs
    ├── fault_injection.rs
    └── end_to_end.rs

---

17. "errors/" contract

"errors/error.rs"

This is the foundational error contract.

It owns:

- stable error representation;
- stable error codes;
- category;
- severity;
- retryability;
- recoverability;
- diagnostic context;
- resource context;
- logical/physical qubit context;
- operation context;
- source error preservation;
- deterministic display.

The current implementation already follows the intended architectural direction.

It uses:

crate::quantum::ir::qubit::QubitId
crate::quantum::ir::qubit::PhysicalQubitId

and does not depend on higher-level resilience modules.

That independence must be preserved.

"errors/codes.rs"

"codes.rs" must not create a competing second error-code enum.

The canonical stable code identity remains the code type established by "error.rs".

"codes.rs" should provide:

- grouped code documentation;
- stable string-code mapping;
- compatibility helpers;
- code-family classification;
- lookup helpers.

If the repository chooses to move code ownership into "codes.rs" in the future, that is a compatibility change and must be explicitly versioned.

"errors/classification.rs"

Own:

Transient
Persistent
Recoverable
NonRecoverable
Unknown
SafetyCritical
SemanticRisk

and related machine-readable classifications.

It must not execute recovery.

---

18. "model/"

"model/resource.rs"

Represents generic execution resources.

Examples:

backend
device
QPU
logical qubit
physical qubit
coupling
control channel
memory
execution slot
classical resource
network path

The model must not assume all technologies have the same resource hierarchy.

"model/fault.rs"

Consumes canonical ZQN fault semantics.

It must not redefine ZQN.

The flow is:

ZQN Fault
    |
    v
resilience normalization
    |
    v
resilience incident

"model/incident.rs"

Correlates related faults.

One physical problem may generate many observations.

The incident model prevents:

100 correlated observations
-> 100 independent recovery operations

Instead:

100 observations
-> 1 correlated incident
-> 1 coordinated response

"model/severity.rs"

Defines severity independent of hardware/provider.

"model/health.rs"

Represents:

Unknown
Healthy
Degraded
Unstable
Unavailable
Recovering
Quarantined
Retired

"model/degradation.rs"

Represents partial capability loss.

Example:

usable resources:
N
   |
   v
N - a
   |
   v
N - b

without hard-coding the value of "N".

"model/capability.rs"

Consumes capability information from the hardware HAL.

It must not redefine the hardware capability source of truth.

"model/confidence.rs"

Every inferred diagnosis or prediction should carry confidence.

---

19. Detection design

Detection answers:

«What has been observed?»

It does not answer:

«What should we do?»

Detector interface

Detectors must be composable.

Possible detectors:

anomaly
threshold
statistical
drift
timeout
execution failure
QEC signal
hardware signal

No detector should directly invoke recovery.

---

20. Detection data model

Observations should contain enough information to establish:

observation identity
source
timestamp
resource
operation
execution
measurement
confidence
integrity
provenance

Observations must be distinguishable from diagnoses.

---

21. Diagnosis design

Diagnosis answers:

«What most likely happened?»

It must represent uncertainty.

A diagnosis is not equivalent to truth.

It should contain:

candidate cause
evidence
confidence
affected resources
scope
correlation
alternatives

The diagnosis layer must never silently convert uncertainty into certainty.

---

22. Policy design

Policy answers:

«What is allowed?»

Policy must remain separate from planning.

Examples:

correctness = strict
migration = allowed
mitigation = adaptive
availability = preferred
semantic_deviation = forbidden

Policy controls actions but does not execute them.

---

23. Constraints

Constraints describe requirements such as:

semantic equivalence
maximum logical error
execution deadline
resource budget
mitigation overhead
allowed migration
allowed recompilation
security restrictions

Constraints must be data-driven.

---

24. Objectives

Objectives can include:

correctness
fidelity
latency
availability
cost
energy
resource utilization
logical error probability

The planner may optimize across multiple objectives.

No single objective is universally dominant.

Correctness and safety remain hard constraints.

---

25. Budgets

Budgets are explicit.

Possible budgets include:

retry
time
shots
memory
qubits
compilation
mitigation
recovery
network
storage

The resilience core must not assume a universal retry count.

---

26. Planning

Planning answers:

«Given the diagnosis, policy, capabilities, and state, what should happen next?»

A plan must contain:

incident
diagnosis
preconditions
actions
expected effects
risk
cost
confidence
rollback/recovery strategy
verification requirements

Plans should be immutable once execution begins.

If conditions change, the plan becomes stale and a new plan must be produced.

---

27. Plan feasibility

Before execution, every action must be checked against current capabilities.

Example:

planned:
move computation to physical region A

capabilities changed

region A unavailable

=> plan stale

=> replan

The system must not blindly execute stale plans.

---

28. Adaptation

Adaptation translates a resilience decision into changes to execution.

It includes:

remapping
rerouting
rescheduling
recompilation
reoptimization
QEC adaptation
backend selection

Adaptation invokes the appropriate subsystem.

It does not reimplement those subsystems.

---

29. Remapping

Logical mapping:

QubitId
   |
   v
routing contract
   |
   v
PhysicalQubitId

Resilience may request a new mapping because of:

- physical qubit failure;
- coupling failure;
- calibration degradation;
- topology change;
- resource quarantine.

It must not implement its own routing algorithm.

---

30. Rerouting

Rerouting must consume the canonical routing subsystem.

Resilience supplies:

affected resources
new capabilities
constraints
current mapping
policy

Routing supplies:

new feasible mapping

---

31. Rescheduling

Scheduling must remain responsible for timing.

Resilience may request rescheduling because:

- a resource failed;
- calibration changed;
- routing changed;
- a timing constraint changed;
- mitigation inserted operations;
- QEC configuration changed.

The scheduler produces the new schedule.

---

32. Recompilation

Resilience may request:

affected-region recompilation

or:

whole-program recompilation

according to the compiler contract.

It must not manually mutate canonical IR.

---

33. Reoptimization

Optimization remains responsible for transformation.

Resilience provides a changed target/context.

Example:

old target
    |
    v
optimization
    |
    v
old implementation

hardware degradation
    |
    v
new target
    |
    v
optimization
    |
    v
new implementation

---

34. QEC adaptation

Resilience does not become a QEC decoder.

QEC owns:

encoding
syndrome extraction
decoding
logical correction
code-specific operations

Resilience decides whether to request:

different code
different distance
different decoder
different logical layout
different ancilla allocation

when the QEC contract supports it.

---

35. Backend selection

Backend selection must be capability-driven.

Core resilience must not contain:

if IBM
if IonQ
if Braket

Provider-specific behavior belongs below the hardware adapter boundary.

Selection should operate on capabilities.

---

36. Recovery

Recovery answers:

«How do we execute the chosen recovery plan?»

Possible operations:

retry
restart
checkpoint
rollback
resume
migration
compensation

Recovery must be subordinate to policy and verification.

---

37. Retry semantics

Retry is not universally safe.

A retry is permitted only if:

1. policy allows it;
2. execution semantics permit it;
3. the operation is repeatable or restartable;
4. required resources remain valid;
5. retry does not violate the quantum computation's semantic requirements;
6. retry budget remains;
7. verification remains possible.

---

38. Quantum restart semantics

Restart must use an explicitly defined restart boundary.

It must never assume that arbitrary quantum state can be recreated from an opaque checkpoint.

Valid restart boundaries may include:

program start
measurement boundary
classical-control boundary
verified logical checkpoint
reconstructible state
provider-supported state

---

39. Checkpointing

Checkpointing must distinguish:

classical state
compiled state
logical state
measurement boundary
QEC state
reconstructible state
provider-supported state

The architecture must never claim:

arbitrary quantum state
-> serialize bytes
-> restore exact state

without a formally supported mechanism.

---

40. Migration

Migration means moving execution to another compatible target.

Possible targets:

another physical region
another device
another QPU
another backend
simulator
emulator
logical resource
distributed resource

Migration must preserve semantic equivalence.

---

41. Compensation

Quantum compensation is not generic undo.

A compensating action must be mathematically justified by the workload semantics.

If compensation cannot guarantee correctness, the result must not be silently accepted.

---

42. Mitigation

Mitigation is distinct from QEC.

QEC attempts to preserve/correct logical computation.

Mitigation attempts to reduce the effect of noise on an obtained result or execution.

The mitigation subsystem should accommodate mechanisms such as:

readout mitigation
zero-noise extrapolation
probabilistic error cancellation/amplification
twirling
dynamical decoupling
future techniques

Current production quantum platforms already expose these as distinct execution-level techniques with different overheads and applicability.

---

43. Mitigation selection

Selection must consider:

noise characteristics
workload
observable
target capabilities
available calibration
execution budget
accuracy requirements
latency
shots

There is no universally optimal mitigation technique.

---

44. Mitigation overhead

Every mitigation strategy must expose its expected overhead.

Examples:

additional circuits
additional shots
additional gates
additional scheduling time
additional calibration
additional classical processing

The planner must account for this overhead.

Current quantum execution documentation explicitly notes that mitigation and suppression introduce preprocessing or sampling overhead and therefore require balancing result quality against execution cost.

---

45. Verification

Verification is mandatory.

The lifecycle is:

recovery
   |
   v
verification
   |
   +----> accept
   |
   +----> retry
   |
   +----> recover again
   |
   +----> escalate
   |
   +----> reject

No recovery result is automatically accepted.

---

46. Semantic verification

Semantic verification must compare the adapted execution against canonical semantics.

It should verify as applicable:

logical qubit identity
operation meaning
measurement meaning
classical control
observable semantics
required invariants
resource constraints

---

47. Result verification

Result verification checks:

result structure
expected result domain
measurement validity
statistical validity
confidence
provenance
integrity

---

48. Provenance

Every resilient execution should be able to reconstruct:

program identity
IR identity/hash
compiler version
optimization configuration
routing decision
schedule
target capability snapshot
hardware identity
calibration identity
fault observations
diagnosis
policy
recovery plan
adaptation
mitigation
QEC configuration
execution attempts
verification
final result

This is essential for scientific reproducibility and debugging.

---

49. State machine

The resilience state machine is:

Idle
 |
 v
Detecting
 |
 v
Diagnosing
 |
 v
Planning
 |
 v
Adapting
 |
 v
Recovering
 |
 v
Verifying
 |
 +----> Completed
 |
 +----> Detecting
 |
 +----> Escalated
 |
 +----> Failed

State transitions must be explicit.

No implicit recovery loop is permitted.

---

50. State ownership

"state/" owns current resilience state.

It does not own:

- canonical quantum state;
- hardware internal state;
- provider session state;
- QEC decoder internals.

It stores references/snapshots necessary for resilience coordination.

---

51. Telemetry

Telemetry is an input to resilience.

Events should represent:

execution started
execution completed
execution failed
resource degraded
resource recovered
calibration changed
fault observed
diagnosis changed
plan created
plan invalidated
adaptation performed
recovery performed
verification performed
result accepted
result rejected

---

52. Monitoring and drift

Hardware behavior is time-dependent.

The resilience architecture therefore treats monitoring, calibration, and benchmarking as continuous sources of state rather than one-time initialization.

This is consistent with current hardware practice: calibration parameters can drift due to environmental and control-system changes, requiring ongoing monitoring, recalibration, and benchmarking.

---

53. History

History stores:

incidents
executions
recovery attempts
verification outcomes
strategy outcomes

History must be append-oriented where possible.

Historical data must not be mutated merely to make a recovery appear successful.

---

54. Learning

Learning is optional.

Correctness must never depend on machine learning.

The hierarchy is:

hard safety constraints
        >
semantic constraints
        >
policy
        >
verified capabilities
        >
deterministic planning
        >
learned ranking

A learned model may rank plans.

It may not override:

safety
security
semantic verification
policy prohibition

---

55. Learning feedback

Only verified outcomes should become learning feedback.

Do not train a recovery strategy from:

unverified result

as though it were successful.

Use:

action
+
observed outcome
+
verification status
+
confidence

---

56. Distributed coordination

Distributed resilience becomes necessary when execution spans multiple resources.

It must support:

ownership
leases
coordination
distributed state
recovery ownership
resource reservation

Consensus is an abstraction unless a specific distributed algorithm is formally required.

Do not implement a bespoke consensus mechanism merely because the directory contains "consensus.rs".

---

57. Concurrency

Resilience must assume multiple observations and state changes can occur concurrently.

Example:

Detector A:
qubit degraded

Detector B:
backend unavailable

Detector C:
calibration changed

The system must merge or correlate observations before launching conflicting recoveries.

---

58. Recovery ownership

Only one authoritative recovery operation should own a given recovery scope at a time unless concurrent recovery is explicitly supported.

Otherwise:

Recovery A:
reroute

Recovery B:
migrate

Recovery C:
restart

could execute against stale assumptions.

---

59. Plan invalidation

A plan must be invalidated if relevant assumptions change.

Examples:

target capability changed
resource disappeared
calibration changed
security state changed
deadline expired
policy changed
new higher-severity incident

The system then returns to planning.

---

60. Checkpoint compatibility

Checkpoint restoration must verify:

schema compatibility
IR compatibility
program compatibility
target capability compatibility
QEC compatibility
execution compatibility
security/integrity

A checkpoint must never be restored solely because its bytes can be decoded.

---

61. Serialization

All persistent resilience objects require deterministic schema definitions.

Serialization must support:

schema identity
schema version
compatibility
integrity
deterministic encoding
forward compatibility policy
backward compatibility policy

Serialization format must not become part of the semantic model.

---

62. Determinism

Deterministic mode is a first-class requirement.

Given identical:

program
IR
hardware snapshot
capability snapshot
telemetry
policy
history
random seed

a deterministic planner should produce the same result.

Where nondeterminism is intentionally used, it must be:

explicit
seedable
observable
reproducible

---

63. Deterministic ordering

Whenever multiple candidates are otherwise equivalent, selection must use a deterministic tie-break rule.

For example:

primary score
secondary cost
tertiary risk
stable identity

Never rely on:

HashMap iteration order
thread scheduling
backend response ordering
network arrival order

for semantic decisions.

---

64. Security boundary

Resilience can become a dangerous control plane.

An attacker could attempt to forge:

health telemetry
fault reports
backend state
capabilities
checkpoint data
recovery requests

Therefore external observations require:

source identity
integrity
timestamp
trust classification
provenance

where applicable.

---

65. Secret handling

Resilience errors, telemetry, provenance and checkpoints must not expose:

API keys
credentials
tokens
private keys
passwords
authorization headers
session secrets
provider secrets

The existing resilience error contract explicitly treats secret leakage as a security boundary.

---

66. Plugin security

Future detectors, mitigation strategies, recovery strategies and backend adapters must not automatically receive unrestricted authority.

A plugin must have explicit capabilities.

Conceptually:

Plugin
  |
  +-- observe
  +-- analyze
  +-- plan
  +-- execute
  +-- migrate
  +-- persist

These permissions must be independently controllable.

---

67. Registry architecture

Registries provide extensibility.

Required registries:

DetectorRegistry
StrategyRegistry
RecoveryRegistry
BackendRegistry

Registries must not become global mutable singletons.

Prefer explicit registry instances passed through execution context.

---

68. Error architecture

Every fallible resilience operation should ultimately be representable as:

Result<T, ResilienceError>

Errors must carry enough information to answer:

what failed?
where?
why?
how severe?
retryable?
recoverable?
safe to continue?
what resource?
what operation?
what execution?

Display text is human-facing.

Stable machine decisions use structured fields.

---

69. Resource errors

Resource errors must remain generic.

Examples:

ResourceUnavailable
ResourceLost
CapabilityUnavailable
CapabilityChanged
ResourceStateChanged

They must not contain provider-specific semantics.

---

70. Qubit errors

When an error refers to a logical qubit:

QubitId

must be used.

When it refers to a physical qubit:

PhysicalQubitId

must be used.

This distinction prevents a major class of logical/physical mapping bugs.

---

71. No duplicate fault ontology

Resilience must consume:

quantum::zqn::fault

rather than defining:

ResilienceFault
NoiseFault
RecoveryFault

as competing physical fault models.

Resilience can define a resilience-level incident, but the underlying fault remains owned by ZQN.

---

72. Fault correlation

Multiple canonical faults may form one incident:

Fault A
Fault B
Fault C
Fault D
       |
       v
Correlator
       |
       v
Incident

This is essential at large scale.

A large machine may produce many correlated observations for one physical event.

---

73. Fault localization

Localization should operate over generic resources:

device
region
qubit
coupling
operation
gate
control channel
execution stage
backend
network path

It must not assume all architectures expose physical qubits in the same way.

---

74. Technology neutrality

The design must support:

superconducting
trapped ion
neutral atom
photonic
spin
topological
analog
annealing
logical/fault-tolerant
simulator
emulator
distributed systems
future technologies

The hardware layer already establishes this provider-neutral technology scope.

Resilience must inherit that abstraction rather than narrow it to gate-model QPUs.

---

75. Workload neutrality

Resilience must not assume every workload is a circuit.

It must be capable of coordinating:

gate circuits
dynamic circuits
pulse programs
analog workloads
annealing workloads
sampling
logical/fault-tolerant workloads

The concrete recovery mechanisms may differ by workload type.

---

76. Scheduling integration

Resilience asks scheduling:

Can this schedule be rebuilt?
What resources are affected?
What operations depend on them?
What timing constraints changed?
Can unaffected regions remain unchanged?

Scheduling owns the answer.

The scheduler must not become resilience-aware through hard-coded recovery logic.

---

77. Routing integration

Resilience asks routing for:

new feasible mapping

after:

resource loss
topology change
calibration degradation
migration
QEC layout change

Routing owns the mapping algorithm.

---

78. Optimization integration

Resilience supplies:

new target
new capabilities
new constraints
new fault-tolerance requirements

Optimization supplies:

new implementation

Optimization must continue to operate on canonical quantum IR.

The existing optimization architecture explicitly prohibits competing quantum semantic types and requires canonical IR identities.

---

79. QEC integration

QEC supplies:

syndrome observations
decoder outcomes
logical error signals
code capabilities
logical-resource state

Resilience supplies decisions such as:

continue
adapt
increase protection
change configuration
migrate
escalate

QEC remains authoritative for correction.

---

80. ZQN integration

ZQN supplies canonical:

Fault
FaultLocation
FaultClassification
FaultEffect
correlation information
leakage
loss
erasure

Resilience converts those into:

observations
incidents
diagnoses
plans

ZQN remains the authoritative fault ontology.

---

81. Hardware integration

Hardware supplies:

identity
technology
capabilities
topology
calibration
timing
health
execution
queue
result
status
telemetry

Resilience consumes those contracts.

Hardware must not contain the resilience planner.

The hardware HAL already defines itself as provider-neutral and isolates provider-specific behavior under adapters.

---

82. Benchmarking integration

Benchmarking supplies evidence such as:

fidelity
error rate
logical error rate
readout quality
gate quality
latency
stability
resource reliability

This information may influence:

diagnosis
plan cost
backend selection
mitigation selection
learning

but benchmark values must not override semantic safety constraints.

---

83. Calibration integration

Calibration is a changing execution input.

A calibration snapshot should be associated with:

execution
plan
provenance
verification

If a relevant calibration changes, the system must be able to invalidate stale assumptions.

Current hardware practice demonstrates why this matters: calibration parameters can drift over time and monitoring/calibration/benchmarking are continuously coordinated.

---

84. Simulation integration

Simulation must be able to test resilience without real hardware.

Conceptually:

program
+
synthetic capabilities
+
synthetic telemetry
+
canonical ZQN faults
+
policy
        |
        v
resilience
        |
        v
adapt/recover/verify

This enables deterministic fault-injection testing.

---

85. Fault injection

Fault injection must cover:

single-qubit failure
multi-qubit failure
correlated failure
leakage
loss
erasure
readout failure
gate failure
preparation failure
reset failure
timing failure
calibration drift
backend outage
queue timeout
execution timeout
routing failure
scheduling failure
compiler failure
QEC failure
decoder failure
network failure
checkpoint corruption
telemetry corruption
security failure

Fault injection must use canonical ZQN semantics wherever the fault is a quantum fault.

---

86. Recovery strategy hierarchy

The planner should consider strategies in a general hierarchy:

continue unchanged
      |
      v
local adaptation
      |
      v
local recovery
      |
      v
regional adaptation
      |
      v
global recompilation
      |
      v
backend/device migration
      |
      v
checkpoint restart
      |
      v
escalation
      |
      v
abort

The hierarchy is conceptual.

Policy determines which strategies are legal.

---

87. Local versus global adaptation

Not every failure requires recompiling the entire workload.

The system should support:

local rerouting
local rescheduling
local recompilation
local mitigation

when semantic and dependency analysis proves the unaffected regions remain valid.

Otherwise:

global recompilation

is allowed.

---

88. Plan granularity

Plans should be able to operate at:

operation
region
logical qubit
physical qubit
QEC block
circuit
program
device
backend
distributed execution

No fixed granularity is mandatory.

---

89. Resource model

Resource accounting must support arbitrary resource types.

A resource may have:

identity
kind
capacity
availability
health
ownership
capabilities
cost
location
dependencies

No global fixed resource registry is assumed.

---

90. Resource exhaustion

Resource exhaustion is not necessarily a failure.

The planner may choose:

degrade
partition
serialize
migrate
reschedule
reduce mitigation overhead
reduce optional optimization

if policy allows.

However, correctness-critical resource requirements cannot be degraded silently.

---

91. Graceful degradation

Example:

Target originally supports:
logical capacity = N

resource loss:
N -> N - k

If the computation remains feasible:

continue

If only a different implementation is possible:

adapt

If semantic requirements cannot be satisfied:

escalate/reject

---

92. No hidden semantic degradation

The system must never silently transform:

exact computation

into:

approximate computation

unless approximation is explicitly permitted by policy.

---

93. Approximation policy

If approximation is supported, it must be explicit.

The policy must specify:

allowed approximation
error bound
verification method
resource benefit

The result must retain provenance showing the approximation.

---

94. Security-aware recovery

Recovery actions must be authorized.

For example:

migrate

may require stronger privileges than:

re-read telemetry

The recovery planner must know whether an action is authorized before selecting it.

---

95. Telemetry trust

Telemetry should have a trust model.

Possible levels:

Trusted
Authenticated
Validated
Untrusted
Rejected

Untrusted observations may still be retained as evidence but must not automatically trigger privileged recovery.

---

96. Provenance integrity

Recovery history must be tamper-evident.

At minimum, provenance should support:

stable identifiers
content hashes where applicable
schema versions
timestamps
source identity
parent references

---

97. Observability

Every lifecycle transition should be observable.

Examples:

resilience.detected
resilience.diagnosed
resilience.plan.created
resilience.plan.invalidated
resilience.adaptation.started
resilience.adaptation.completed
resilience.recovery.started
resilience.recovery.completed
resilience.verification.started
resilience.verification.completed
resilience.result.accepted
resilience.result.rejected

Event names should be stable.

---

98. Metrics

Recommended metrics include:

fault count
incident count
diagnosis confidence
plan count
plan rejection count
adaptation count
recovery count
recovery success rate
verification failure rate
migration count
mitigation overhead
retry count
execution latency
recovery latency
resource degradation
backend availability
logical error rate

Metrics must not become hard-coded decision rules.

---

99. Memory scalability

The architecture must not assume that all telemetry, incidents, or history can remain in memory forever.

Production implementations should support:

streaming
bounded retention
persistent history
aggregation
sampling
partitioning
windowing
external storage

The semantic contract remains independent of the storage strategy.

---

100. Event scalability

Large machines may generate enormous observation streams.

Therefore:

event stream
    |
    v
filter
    |
    v
aggregate
    |
    v
correlate
    |
    v
incident

is preferable to:

every observation
-> immediate recovery

---

101. Correlated failures

The system must handle failures where one root cause produces many symptoms.

Example:

control-system degradation
        |
        +-- gate failures
        +-- readout degradation
        +-- timing anomalies
        +-- correlated qubit faults

Diagnosis must correlate these rather than producing unrelated independent recoveries.

---

102. Recovery storm prevention

Large systems must prevent recovery storms.

A recovery storm occurs when:

one incident
-> many detectors
-> many plans
-> many recovery operations

The incident coordinator must provide:

deduplication
correlation
ownership
cooldown policy
plan invalidation
coalescing

without embedding arbitrary universal timing constants.

---

103. Backpressure

Telemetry and recovery pipelines require backpressure.

If observation production exceeds processing capacity, the system must apply an explicit policy such as:

buffer
aggregate
sample
persist
drop low-priority observations
escalate

Critical events must not be silently discarded.

---

104. Large-scale coordination

At very large scale, resilience should support hierarchical coordination:

global coordinator
       |
       +-- region coordinator
       |       |
       |       +-- device coordinator
       |               |
       |               +-- resource coordinator
       |
       +-- another region

The architecture must not require one central process to hold every resource's complete state.

---

105. Partitioning

Resilience state may be partitioned by:

execution
device
region
backend
workload
logical resource

The partitioning strategy must remain implementation-specific.

The public contract must not assume a particular topology.

---

106. Distributed failure

Distributed resilience must handle:

network partition
message loss
duplicate messages
delayed messages
stale state
split ownership
lease expiration
partial execution

No recovery action may assume instantaneous global knowledge.

---

107. Stale information

Every time-sensitive observation should be capable of being evaluated for freshness.

A stale observation may remain historical evidence but should not automatically represent current state.

---

108. Capability snapshots

Planning should operate against a capability snapshot.

Conceptually:

CapabilitySnapshot A
       |
       v
plan
       |
       v
capabilities change
       |
       v
snapshot mismatch
       |
       v
invalidate plan

This prevents execution based on stale target assumptions.

---

109. Compatibility

Compatibility must cover:

Zamani version
IR version
resilience schema
checkpoint schema
hardware capability schema
QEC schema
backend adapter
serialization version

Compatibility checks must happen before destructive recovery actions.

---

110. Schema evolution

A schema change must specify:

old version
new version
compatibility direction
migration strategy
lossless/lossy behavior
rejection conditions

Unknown fields should be handled according to the serialization compatibility policy.

---

111. Rust compatibility

The design targets:

Rust 1.97
Rust 1.97.1
Rust 2021
stable Rust

No nightly features.

No "unsafe".

No compiler-specific undocumented behavior.

---

112. Unsafe-code prohibition

The entire resilience namespace should enforce:

#![forbid(unsafe_code)]

at appropriate crate/module boundaries.

The architecture must not depend on unsafe optimizations.

---

113. Allocation strategy

Production implementations should avoid unnecessary cloning of large:

IR
telemetry
fault collections
execution results
history
provenance

Use ownership and borrowing deliberately.

However, optimization for allocation must not introduce unsafe code or semantic ambiguity.

---

114. Generic collections

Data structures should be chosen based on actual access patterns.

Examples:

Vec
VecDeque
BTreeMap
HashMap
BTreeSet
HashSet
Arc

No fixed-size arrays for machine-scale resources unless the resource contract itself explicitly requires fixed cardinality.

---

115. Error handling

Errors must never be silently swallowed.

Forbidden:

let _ = recovery();

when the result determines correctness.

Errors must either:

handle
propagate
classify
record
escalate

according to policy.

---

116. Panic policy

Production resilience code should avoid panics for expected operational conditions.

Expected failures include:

resource unavailable
backend unavailable
invalid telemetry
timeout
checkpoint incompatibility
policy rejection

These must be represented as structured errors.

Panics should represent programming invariants that are genuinely impossible under a validated contract, and even those should be minimized.

---

117. API boundary

The public entry point is:

api/controller.rs

The controller coordinates the lifecycle.

It should not contain the implementation of:

detection
diagnosis
planning
routing
scheduling
recovery

---

118. Request

"api/request.rs" contains immutable request data:

program/execution identity
policy
target requirements
resource requirements
verification requirements
resilience mode
deadline
budget
determinism configuration

It should reference canonical objects rather than duplicate them.

---

119. Response

"api/response.rs" returns:

execution outcome
verification result
provenance
recovery history
diagnostic summary

The response must distinguish:

completed successfully
completed after recovery
completed with permitted degradation
rejected
failed
escalated

---

120. Context

"api/context.rs" supplies service contracts.

Conceptually:

IR
ZQN
QEC
routing
scheduling
optimization
hardware
execution
telemetry
policy
history
checkpoint

The context should use abstractions rather than concrete provider implementations.

---

121. Controller

The controller executes:

observe
diagnose
evaluate
plan
adapt
recover
verify

It should be orchestration only.

Business logic belongs in the appropriate child subsystem.

---

122. Registry lifecycle

Registries should be constructed before execution and treated as immutable during a deterministic execution unless dynamic registration is explicitly part of the execution model.

Changing strategy implementations during an active execution can invalidate determinism.

---

123. Deterministic registry behavior

Registry selection must have stable ordering.

If two strategies have equal priority, selection must use a stable deterministic tie-breaker.

---

124. Plugin versioning

Every externally loaded strategy should expose:

identity
version
capabilities
compatibility
security requirements

The planner should reject incompatible strategies before execution.

---

125. Strategy contracts

A recovery strategy should describe:

supported failure classes
required capabilities
semantic assumptions
resource requirements
expected cost
risk
verification requirements
security requirements

The strategy itself should not bypass policy.

---

126. Planning cost model

Cost must be multidimensional.

Possible dimensions:

execution time
queue time
shots
qubits
logical error probability
classical CPU
memory
energy
network
provider cost
compilation cost
mitigation overhead
recovery risk

Costs must remain comparable through an explicit policy/objective model.

---

127. Cost is not correctness

A cheaper plan must never beat a semantically valid plan if the cheaper plan violates correctness.

Therefore:

hard constraints
    >
soft objectives

---

128. Confidence

Confidence must not be confused with correctness.

Example:

diagnosis confidence = 99%

does not mean:

result correctness = 99%

These are different quantities.

---

129. Verification confidence

Verification confidence describes evidence supporting acceptance.

It must be based on:

verification method
observed evidence
statistical uncertainty
integrity
provenance

---

130. Statistical results

When mitigation or repeated execution produces estimates, resilience must preserve uncertainty.

Do not convert:

estimate ± uncertainty

into:

exact result

---

131. Mitigation and statistical uncertainty

Current mitigation techniques such as ZNE can improve estimates but are not guaranteed to produce unbiased results and can introduce additional execution overhead.

Therefore verification must preserve:

estimate
uncertainty
method
noise factors
sampling information

where applicable.

---

132. Runtime integration

The runtime should invoke resilience through the public controller.

Conceptually:

runtime
  |
  v
ResilienceController
  |
  +-- compile
  +-- route
  +-- schedule
  +-- execute
  +-- observe
  +-- recover
  +-- verify

The runtime remains the execution owner.

---

133. Quantum execution fabric

Long-term, the architecture should provide a higher-level:

Quantum Execution Fabric

composed of:

compiler
optimization
QEC
routing
scheduling
hardware
runtime
resilience

Resilience becomes an intrinsic execution property rather than a separate afterthought.

---

134. Write-once semantics

A normal Zamani quantum program should express:

what computation should happen

not:

which vendor
which physical qubit
which retry count
which calibration
which topology
which pulse

The intended abstraction is:

logical program
       |
       v
canonical IR
       |
       v
target-independent planning
       |
       v
target-specific lowering

---

135. Example logical execution

Conceptually:

Zamani program
    |
    v
canonical IR
    |
    v
target A
    |
    +-- route
    +-- schedule
    +-- execute
    |
    v
verified result

The same program may become:

same program
    |
    v
target B
    |
    +-- different route
    +-- different schedule
    +-- different native operations
    |
    v
verified result

The program semantics remain unchanged.

---

136. Dynamic target changes

During execution:

target A
   |
   v
resource degradation
   |
   v
detect
   |
   v
diagnose
   |
   v
replan
   |
   v
target A'
   |
   v
reroute/reschedule/recompile
   |
   v
continue

If target A cannot satisfy the requirements:

target A
   |
   v
migration
   |
   v
target B

---

137. Verification after migration

Migration always requires revalidation.

At minimum:

program compatibility
IR compatibility
target capability compatibility
mapping validity
schedule validity
QEC validity
provenance
result verification

---

138. Failure containment

A failed resource should not necessarily poison the entire computation.

The architecture should support:

quarantine resource

while preserving unaffected resources.

---

139. Quarantine

A quarantined resource should be excluded from new plans unless explicitly released.

Quarantine state must have:

reason
source
timestamp
confidence
scope
release conditions

---

140. Recovery loops

Recovery loops must be bounded by explicit policy.

The system must not contain hidden:

while true

recovery behavior.

Termination can occur through:

success
policy budget exhaustion
deadline
no feasible plan
safety rejection
verification failure
explicit cancellation

---

141. Cancellation

Cancellation must be first-class.

Cancellation should propagate through:

planning
adaptation
recovery
execution
verification

without leaving inconsistent ownership state.

---

142. Deadline handling

Deadlines should be propagated as explicit execution constraints.

If a recovery action cannot complete before the deadline, the planner must consider alternatives or escalate.

---

143. Resource reservation

If recovery requires scarce resources, the planner should be able to request reservations.

Reservations must be:

scoped
owned
expirable
observable
releasable

---

144. Lease expiration

A recovery operation must not continue indefinitely after its ownership lease expires.

The state machine must define safe behavior.

---

145. Recovery idempotency

Where possible, recovery actions should be idempotent.

For non-idempotent actions, the contract must explicitly identify that fact.

This is especially important for:

retry
migration
checkpoint restore
external execution submission

---

146. Duplicate execution prevention

The resilience layer must be able to distinguish:

same logical execution
same physical attempt
same recovery attempt
duplicate provider submission

This is essential for distributed execution.

---

147. Execution identity

Every execution should have a stable execution identity.

Every attempt should have an attempt identity.

Every recovery action should have a recovery identity.

This allows:

program
  -> execution
      -> attempt
          -> recovery
              -> verification

to be reconstructed.

---

148. Observability versus semantics

Telemetry must never change quantum semantics.

Logging:

must not

modify:

IR
routing
schedule
result

unless explicitly implemented as a policy-controlled execution mechanism.

---

149. Logging security

Logs must be treated as potentially persistent and externally visible.

Never log:

credentials
tokens
private keys
secrets
unredacted private data

---

150. Documentation requirements

Each production file must contain documentation covering:

purpose
ownership
non-ownership
dependencies
integration contract
scalability
determinism
security
errors
testing

This requirement is part of the design, not optional documentation style.

---

151. "mod.rs" requirements

Every "mod.rs" should be a composition boundary.

It should contain:

module declarations
selected public re-exports
module-level documentation

It should not contain:

business algorithms
global mutable state
provider logic
hidden initialization

---

152. Public API stability

Public module paths are part of the Zamani API.

Adding a child module should not require modifying unrelated modules.

Avoid broad glob re-exports.

Prefer explicit exports.

---

153. Serialization ownership

Serialization must not redefine semantic objects.

The flow is:

semantic object
    |
    v
serialization representation

not:

serialized object
    |
    v
new semantic model

---

154. Compatibility ownership

Compatibility logic belongs in compatibility modules.

Do not spread version checks throughout resilience.

Forbidden pattern:

if version == ...

through dozens of unrelated files.

Prefer:

CompatibilityChecker

with explicit contracts.

---

155. Testing architecture

Every subsystem must have:

unit tests
property tests where useful
fault injection
serialization tests
determinism tests
integration tests
scale tests
security tests
end-to-end tests

---

156. Model tests

Test:

valid resource
invalid resource
logical/physical distinction
empty collections
large collections
unknown states
confidence
severity
degradation
incident correlation

---

157. Detection tests

Test:

no anomaly
single anomaly
multiple anomalies
stale telemetry
conflicting telemetry
missing telemetry
large event streams

---

158. Diagnosis tests

Test:

single cause
multiple causes
correlated causes
unknown cause
low confidence
conflicting evidence

---

159. Planning tests

Test:

one feasible plan
multiple plans
no feasible plan
equal-cost plans
stale plans
changed capabilities
budget exhaustion

---

160. Adaptation tests

Test:

mapping change
topology change
schedule change
compiler change
optimization change
QEC change
backend migration

---

161. Recovery tests

Test:

successful retry
retry rejection
restart
resume
rollback
migration
checkpoint restore
compensation
verification failure

---

162. Mitigation tests

Test:

strategy available
strategy unavailable
insufficient capability
budget exceeded
statistical uncertainty
verification rejection

---

163. Verification tests

Test that:

valid recovery -> accepted
invalid recovery -> rejected
uncertain recovery -> inconclusive/escalated
semantic mismatch -> rejected
tampered provenance -> rejected

---

164. Fault injection tests

Fault injection must exercise the full lifecycle:

fault
 -> detection
 -> diagnosis
 -> policy
 -> planning
 -> adaptation
 -> recovery
 -> verification

This is more important than isolated unit tests.

---

165. Scalability tests

Scalability tests must generate resource counts rather than hard-code only a few machine sizes.

Conceptually:

resource count = generated according to test configuration

Test:

minimal
small
medium
large
very large
partitioned
distributed

The implementation must remain correct regardless of the selected finite test scale.

---

166. Memory scalability

Tests must detect:

unbounded history
unbounded event queues
quadratic correlation
quadratic copying
fixed-size buffers

where they violate configured resource policy.

---

167. Time scalability

Tests should detect algorithms that become impractical because of:

O(N²)
O(N³)

when a scalable alternative is required.

The design does not prohibit expensive algorithms universally; it requires the resource policy to make cost explicit.

---

168. Distributed scalability

Test:

one coordinator
multiple coordinators
partitioned resources
delayed messages
duplicate messages
partial failure
recovery ownership conflicts

---

169. Determinism tests

Given identical inputs:

same result
same plan
same ordering
same provenance identifiers where deterministic

must be produced in deterministic mode.

---

170. Replay

The system should support deterministic replay from:

program identity
capability snapshot
telemetry snapshot
policy
history
seed

Replay is essential for debugging recovery decisions.

---

171. Security tests

Security tests must include:

forged telemetry
tampered checkpoint
unauthorized recovery
malicious strategy
invalid capability advertisement
provenance tampering
secret leakage

---

172. Fuzzing

Fuzz suitable boundaries:

serialized resilience objects
error context
telemetry events
fault batches
diagnostic evidence
policy inputs
checkpoint metadata

The goal is to ensure malformed data cannot cause undefined behavior, panics, or silent acceptance.

---

173. No "unsafe"

Fuzzing and robustness must remain safe Rust.

The resilience subsystem must not introduce "unsafe" merely to optimize large workloads.

---

174. Failure-mode taxonomy

The design recognizes:

semantic failure
hardware failure
backend failure
resource failure
routing failure
scheduling failure
compiler failure
optimization failure
QEC failure
decoder failure
mitigation failure
checkpoint failure
serialization failure
network failure
security failure
coordination failure
unknown failure

Each must map into the structured error/diagnosis system.

---

175. Unknown failures

Unknown failures must be first-class.

The system must not fabricate a root cause merely because recovery requires a diagnosis.

A valid state is:

cause = unknown
confidence = low

followed by:

safe recovery
or
escalation

---

176. Safety-first escalation

If the system cannot establish sufficient confidence that a recovery action preserves semantics, it must escalate rather than guess.

This is particularly important for autonomous operation.

---

177. Graceful failure

When recovery is impossible, the system should produce:

structured error
diagnostic context
provenance
verification state
recovery history

rather than merely:

execution failed

---

178. Result states

A final execution should distinguish:

Accepted
AcceptedAfterRecovery
AcceptedWithPermittedDegradation
Rejected
Failed
Escalated
Cancelled

The exact enum belongs to the API/verification contract.

---

179. Semantic preservation

The strongest invariant is:

original semantics
        =
adapted semantics

unless an explicitly authorized approximation policy changes the contract.

---

180. Approximate execution

If approximate execution is permitted:

original semantics
        |
        v
approved approximation model
        |
        v
bounded deviation

The deviation must be:

explicit
bounded
verified
recorded

---

181. Current-industry alignment

The architecture deliberately separates resilience mechanisms because modern quantum platforms already treat:

dynamical decoupling
readout mitigation
twirling
ZNE
probabilistic techniques

as distinct execution strategies with different overhead and applicability.

Zamani should generalize that concept rather than copy provider-specific APIs.

---

182. Provider neutrality

Provider-specific concepts may exist in:

quantum::hardware::adapters

but not in core resilience planning.

This allows:

provider A
provider B
provider C
simulator
emulator
future hardware

to share the same resilience architecture.

---

183. Hardware independence

Resilience consumes capability descriptions.

For example:

supports_mid_circuit_measurement
supports_reset
supports_dynamic_control
supports_fault_tolerant_execution
supports_migration
supports_pulse_control

must come from capability contracts.

Do not infer capabilities from provider names.

---

184. Capability negotiation

Before executing an adaptation:

requested action
        |
        v
capability negotiation
        |
        +---- supported -> feasible
        |
        +---- unsupported -> reject/replan

---

185. Capability changes

Capabilities may change dynamically.

Therefore:

capability snapshot

must be associated with the plan.

If it changes:

plan invalidated

where the changed capability affects the plan.

---

186. Resource-aware policy

Policy may say:

prefer correctness
prefer availability
prefer latency
prefer cost

but correctness and security constraints remain hard boundaries.

---

187. No universal resilience level

Do not create a universal semantic:

resilience_level = 0..5

unless it is only a user-facing profile mapped to explicit policies.

The core model should remain expressive.

---

188. User-facing resilience profiles

A future Zamani language may provide:

resilience {
    correctness = strict
    migration = allowed
    mitigation = adaptive
}

These should compile into:

Policy
Constraints
Objectives
Budgets
Safety

rather than become hard-coded execution behavior.

---

189. Compiler integration

The compiler should preserve enough information for resilience to establish:

semantic identity
operation dependencies
resource requirements
control dependencies
measurement boundaries
verification requirements

Resilience must not reconstruct semantics from provider-specific output.

---

190. Canonical IR integration

The canonical IR remains the semantic source of truth.

Resilience may inspect it.

It must not replace it.

The IR architecture explicitly defines itself as the stable semantic boundary and intentionally excludes hardware, routing, scheduling, calibration and backend execution from the IR.

---

191. Optimization integration

Optimization output must remain traceable to input IR.

Resilience provenance should record optimization configuration and output identity.

If recovery causes reoptimization:

original optimization
    |
    v
new optimization

both must be recorded.

---

192. Routing integration

Routing changes must record:

original mapping
new mapping
affected resources
reason
capability snapshot

---

193. Scheduling integration

Scheduling changes must record:

original schedule
new schedule
timing changes
affected resources
reason

---

194. QEC integration

QEC changes must record:

code
distance
decoder
logical mapping
configuration
reason

where applicable.

---

195. Mitigation integration

Mitigation changes must record:

strategy
parameters
noise observations
additional execution
statistical processing
result uncertainty

---

196. Provenance chain

The final provenance chain should conceptually be:

Zamani source
   |
   v
canonical IR
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
QEC/lowering
   |
   v
target
   |
   v
execution
   |
   +---- observations
   |
   +---- faults
   |
   v
resilience diagnosis
   |
   v
plan
   |
   v
adaptation/recovery
   |
   v
execution attempt
   |
   v
verification
   |
   v
final result

---

197. Recovery history

Recovery history should retain:

why recovery began
what was observed
what was diagnosed
what policy allowed
what plan was selected
what action occurred
what happened
what verification concluded

This enables deterministic analysis and future learning.

---

198. No silent self-modification

Resilience may adapt execution.

It must not silently rewrite the user program's semantics.

Every semantic transformation must be represented by:

compiler/optimization/routing/QEC

contracts and provenance.

---

199. Long-running workloads

For long-running executions, resilience must support:

periodic observation
checkpointing where valid
incremental verification
resource drift detection
plan invalidation
migration
resume

without requiring the entire execution to be held in memory.

---

200. Streaming execution

The architecture must support streaming observation and incremental decisions.

A large execution must not require:

collect all telemetry
then analyze

when streaming is more appropriate.

---

201. Batch execution

Batch workloads should allow correlated planning.

For example:

many circuits
      |
      v
shared hardware incident
      |
      v
shared mitigation/recovery strategy

rather than treating every circuit independently.

---

202. Multi-program execution

If multiple programs share resources, resilience must respect ownership and scheduling boundaries.

One program's recovery must not silently consume resources reserved for another program.

---

203. Fairness

Where multiple executions compete for recovery resources, policy/scheduling should provide explicit fairness semantics.

Fairness must not be hidden inside recovery algorithms.

---

204. Backward compatibility

New resilience capabilities must not break existing programs unless a semantic contract explicitly changes.

Compatibility must be tested across:

IR
policy
serialization
checkpoint
hardware capability
runtime

---

205. Forward compatibility

Unknown future fields should be handled according to schema policy.

The architecture should permit new:

fault classes
hardware capabilities
mitigation strategies
recovery actions
QEC codes

without redesigning the entire resilience core.

---

206. Extensibility rule

Adding a new detector should normally require:

new detector implementation
registry registration
tests

not modification of:

planner
controller
core error model

unless a new semantic contract is genuinely required.

---

207. Adding a new recovery strategy

A new recovery strategy should provide:

identity
version
supported failures
required capabilities
cost model
risk model
execution contract
verification contract
security requirements

and register itself.

---

208. Adding a new hardware provider

Adding a provider must occur below the hardware adapter boundary.

Core resilience must remain unchanged.

---

209. Adding a new QEC code

A new QEC implementation belongs to the QEC subsystem.

Resilience consumes its capability and adaptation interface.

No new QEC algorithm belongs inside "resilience/qec_adaptation.rs".

---

210. Adding a new mitigation technique

A new mitigation strategy belongs under:

mitigation/

and implements the common strategy contract.

The planner should discover it through the registry.

---

211. Production acceptance criteria

"quantum::resilience" is production-ready only when:

Correctness

- semantic verification works;
- no silent semantic changes;
- recovery results are verified;
- canonical IR remains authoritative.

Scalability

- no artificial qubit limit;
- no artificial operation limit;
- no fixed machine-size assumptions;
- event processing can scale;
- state can be partitioned;
- distributed operation is supported by contracts.

Reliability

- retry;
- restart;
- resume;
- rollback;
- migration;
- checkpoint;
- graceful degradation;
- escalation.

Quantum correctness

- logical/physical identity separation;
- canonical ZQN fault semantics;
- QEC integration;
- mitigation separation;
- statistical uncertainty preservation.

Security

- authenticated observations;
- protected provenance;
- checkpoint integrity;
- authorization;
- plugin isolation;
- secret-safe diagnostics.

Determinism

- deterministic planning;
- stable ordering;
- replay;
- explicit randomness.

Observability

- events;
- metrics;
- traces;
- history;
- provenance.

Compatibility

- schema versioning;
- IR compatibility;
- capability compatibility;
- checkpoint compatibility.

Testing

- unit;
- property;
- fuzz;
- fault injection;
- simulation;
- integration;
- end-to-end;
- deterministic replay;
- scalability;
- security.

---

212. Implementation order

The implementation order is intentionally dependency-oriented.

Phase 1 — independent contracts

errors/error.rs
errors/codes.rs
errors/classification.rs

model/resource.rs
model/confidence.rs
model/severity.rs
model/fault.rs
model/incident.rs
model/health.rs
model/degradation.rs
model/capability.rs

Phase 2 — policy

policy/constraints.rs
policy/objectives.rs
policy/budgets.rs
policy/safety.rs
policy/retry.rs
policy/escalation.rs
policy/policy.rs

Phase 3 — observation

telemetry/event.rs
telemetry/metric.rs
telemetry/trace.rs
telemetry/health.rs

detection/detector.rs
detection/threshold.rs
detection/statistical.rs
detection/anomaly.rs
detection/drift.rs
detection/timeout.rs
detection/execution_failure.rs
detection/qec_signal.rs
detection/hardware_signal.rs

Phase 4 — diagnosis

diagnosis/classifier.rs
diagnosis/correlation.rs
diagnosis/localization.rs
diagnosis/root_cause.rs
diagnosis/confidence.rs
diagnosis/diagnostician.rs

Phase 5 — planning

planning/action.rs
planning/cost.rs
planning/feasibility.rs
planning/ranking.rs
planning/plan.rs
planning/planner_state.rs
planning/planner.rs

Phase 6 — adaptation

adaptation/remapping.rs
adaptation/rerouting.rs
adaptation/rescheduling.rs
adaptation/recompilation.rs
adaptation/reoptimization.rs
adaptation/qec_adaptation.rs
adaptation/backend_selection.rs
adaptation/adapter.rs

Phase 7 — recovery

recovery/retry.rs
recovery/restart.rs
recovery/checkpoint.rs
recovery/rollback.rs
recovery/resume.rs
recovery/migration.rs
recovery/compensation.rs
recovery/recoverer.rs

Phase 8 — mitigation

mitigation/strategy.rs
mitigation/selection.rs
mitigation/readout.rs
mitigation/zero_noise.rs
mitigation/probabilistic.rs
mitigation/twirling.rs
mitigation/dynamical_decoupling.rs
mitigation/custom.rs
mitigation/executor.rs

Phase 9 — verification

verification/invariant.rs
verification/semantic.rs
verification/result.rs
verification/confidence.rs
verification/provenance.rs
verification/acceptance.rs
verification/verifier.rs

Phase 10 — state/checkpoint

state/machine.rs
state/execution.rs
state/logical.rs
state/physical.rs
state/recovery.rs
state/persistence.rs

checkpoint/snapshot.rs
checkpoint/manifest.rs
checkpoint/integrity.rs
checkpoint/storage.rs
checkpoint/compatibility.rs
checkpoint/checkpoint.rs

Phase 11 — history/learning

history/incident.rs
history/execution.rs
history/recovery.rs
history/statistics.rs

learning/features.rs
learning/model.rs
learning/predictor.rs
learning/strategy.rs
learning/feedback.rs

Phase 12 — distributed

coordination/ownership.rs
coordination/lease.rs
coordination/distributed.rs
coordination/consensus.rs
coordination/coordinator.rs

Phase 13 — registries/API

registry/detector.rs
registry/strategy.rs
registry/recovery.rs
registry/backend.rs

api/request.rs
api/response.rs
api/context.rs
api/controller.rs

Phase 14 — serialization

serialization/schema.rs
serialization/version.rs
serialization/encode.rs
serialization/decode.rs

Phase 15 — limits

limits/resource.rs
limits/validation.rs
limits/limits.rs

Phase 16 — composition

Only after contracts are stable:

all mod.rs files
root mod.rs
README.md
ARCHITECTURE.md
SECURITY.md
SCALABILITY.md
COMPATIBILITY.md
DETERMINISM.md
FAILURE_MODES.md
RECOVERY_MODEL.md
OBSERVABILITY.md

---

213. File completion standard

A file is considered complete only if the following questions can be answered from that file and its declared interfaces:

What does this file own?

What does it explicitly not own?

What types does it consume?

What types does it produce?

What errors can it return?

How does it scale?

What is its deterministic behavior?

What are its security requirements?

What are its serialization requirements?

Which subsystem integrates with it?

Which subsystem must never depend on it?

How is it tested?

What happens when required capabilities are absent?

What happens when data is stale?

What happens when another subsystem changes?

If these answers are not established, the file is not complete.

---

214. No future-file dependency redesign

A later file may implement a previously declared interface.

It must not redefine the earlier contract merely because the implementation is inconvenient.

If a genuine architectural deficiency is discovered, it must be treated as an explicit design revision, not an implicit dependency change.

This protects the "finish one file once" development model.

---

215. Integration contract table

Resilience component| Integrates with| Direction
"model/fault.rs"| ZQN fault| consumes
"model/capability.rs"| hardware HAL| consumes
"model/resource.rs"| IR/hardware resources| consumes/contracts
"detection/*"| telemetry/QEC/hardware| consumes
"diagnosis/*"| detection/history| consumes
"policy/*"| user/runtime policy| consumes
"planning/*"| diagnosis/policy/capabilities| consumes
"adaptation/remapping.rs"| routing/hardware| requests
"adaptation/rerouting.rs"| routing| requests
"adaptation/rescheduling.rs"| scheduling| requests
"adaptation/recompilation.rs"| compiler/IR| requests
"adaptation/reoptimization.rs"| optimization| requests
"adaptation/qec_adaptation.rs"| QEC| requests
"adaptation/backend_selection.rs"| hardware registry| requests
"recovery/*"| runtime/execution| commands
"mitigation/*"| compiler/scheduler/hardware| requests
"verification/semantic.rs"| canonical IR| consumes
"checkpoint/*"| runtime/storage| coordinates
"telemetry/*"| runtime/hardware/QEC| consumes
"history/*"| persistence| stores
"learning/*"| history| consumes
"coordination/*"| distributed runtime| coordinates
"serialization/*"| resilience models| serializes
"errors/*"| all resilience components| shared contract
"registry/*"| extension implementations| discovers
"api/*"| runtime/compiler| public boundary

---

216. What resilience must never do

Never:

invent a qubit ID

Never:

hard-code a machine size

Never:

hard-code a provider

Never:

hard-code a universal retry count

Never:

hard-code a universal fidelity threshold

Never:

mutate canonical quantum semantics directly

Never:

implement routing inside resilience

Never:

implement scheduling inside resilience

Never:

implement QEC decoding inside resilience

Never:

redefine ZQN fault semantics

Never:

accept an unverified recovery result

Never:

treat uncertain diagnosis as certainty

Never:

hide an execution failure

Never:

restore an unsupported quantum checkpoint

Never:

allow learning to override safety

---

217. What resilience must always do

Always:

preserve canonical identity

Always:

preserve provenance

Always:

respect policy

Always:

validate capabilities

Always:

verify recovered results

Always:

represent uncertainty

Always:

remain provider-neutral

Always:

remain technology-neutral

Always:

remain resource-driven

Always:

support deterministic operation

Always:

support explicit failure

Always:

fail safely

---

218. Final architectural invariant

The complete system should satisfy:

Zamani Program
       |
       v
Canonical Quantum IR
       |
       v
Target-independent compilation
       |
       v
Target-specific realization
       |
       v
Execution
       |
       v
Observation
       |
       v
Resilience
       |
       +---- detect
       +---- diagnose
       +---- constrain
       +---- plan
       +---- adapt
       +---- recover
       +---- mitigate
       +---- verify
       |
       v
Verified Result

The program remains the stable semantic source.

The machine is replaceable.

The physical mapping is replaceable.

The schedule is replaceable.

The optimization is replaceable.

The backend is replaceable.

The mitigation strategy is replaceable.

The recovery strategy is replaceable.

The hardware can change.

The computation's semantics must remain stable.

---

219. Final design principle

The ultimate Zamani resilience invariant is:

«Write the quantum computation once. Discover the available resources. Construct a valid execution. Continuously observe reality. Adapt only within declared semantic and safety constraints. Recover when possible. Verify everything. Accept nothing merely because it completed.»

That is the architectural basis for a resilience subsystem capable of scaling from the smallest quantum execution to arbitrarily large finite systems constrained only by available resources, explicit policy, target capabilities, and the physical laws governing the underlying quantum technology.

---

220. Definition of production readiness

"src/quantum/resilience/" is production-ready when:

Every public contract is stable
        AND
Every recovery path is policy-controlled
        AND
Every adaptation path is capability-checked
        AND
Every result is verification-gated
        AND
Every quantum identity is canonical
        AND
Every fault is grounded in canonical ZQN semantics
        AND
Every physical decision is provider-neutral
        AND
No artificial machine-size limits exist
        AND
Deterministic replay is possible where requested
        AND
Security boundaries are enforced
        AND
Checkpoint semantics are physically valid
        AND
Distributed operation has explicit ownership semantics
        AND
Large-scale telemetry has bounded-resource behavior
        AND
Fault injection passes
        AND
Scalability tests pass
        AND
End-to-end execution passes

Only then should the resilience subsystem be considered production-ready.