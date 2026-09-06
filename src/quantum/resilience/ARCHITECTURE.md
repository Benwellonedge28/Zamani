Worked for 23s

I checked the repository structure and existing resilience/scheduling/optimization/ZQN documentation, including the canonical quantum::ir::qubit identity rule. The architecture below also incorporates current OpenQASM timing/dynamic-control concepts and current IBM resilience practice, while keeping Zamani provider-neutral.   

I have also corrected an important issue from the earlier proposal: resilience must not invent parallel quantum identities, fault models, scheduling IR, hardware models, or QEC semantics. It orchestrates the canonical contracts already owned elsewhere.

Zamani Quantum Resilience Architecture

Path: "src/quantum/resilience/ARCHITECTURE.md"
Status: Normative production architecture
Target: Rust 1.97 / Rust 1.97.1
Edition: Rust 2021
Safety: "unsafe" is forbidden
Scope: Provider-independent, hardware-independent, scalable quantum execution resilience
Primary objective: Write a Zamani quantum program once and execute it across any compatible quantum target, from the smallest available quantum resource to arbitrarily large systems limited only by explicitly available resources and policies.

---

1. Purpose

"quantum::resilience" is the execution-resilience orchestration subsystem of Zamani.

Its responsibility is to maintain the correctness, availability, recoverability, observability, and verifiability of quantum execution when execution conditions change.

It responds to conditions including:

- physical faults;
- logical faults;
- QEC signals;
- hardware degradation;
- calibration drift;
- resource loss;
- topology changes;
- routing failures;
- scheduling failures;
- compiler failures;
- backend failures;
- execution failures;
- timeouts;
- communication failures;
- measurement degradation;
- readout errors;
- noise changes;
- mitigation requirements;
- checkpoint/recovery requirements;
- distributed execution failures;
- capability changes;
- security failures;
- conflicting observations;
- uncertain diagnoses.

The central principle is:

«The Zamani program describes the computation. The resilience subsystem adapts the physical execution strategy without changing the program's intended semantics.»

Resilience therefore does not mean "retry everything."

It means:

OBSERVE
   ↓
DETECT
   ↓
DIAGNOSE
   ↓
APPLY POLICY
   ↓
PLAN
   ↓
ADAPT
   ↓
RECOVER / CONTINUE
   ↓
VERIFY
   ↓
ACCEPT / RETRY / ESCALATE / REJECT

No successful result is accepted merely because execution eventually completed.

---

2. Architectural objective

The target execution model is:

Zamani Program
      │
      ▼
Quantum Frontend
      │
      ▼
Canonical Quantum IR
      │
      ├──────────────┐
      ▼              ▼
Optimization        QEC semantics
      │              │
      └──────┬───────┘
             ▼
           Routing
             │
             ▼
         Scheduling
             │
             ▼
        Hardware HAL
             │
             ▼
          Runtime
             │
             ▼
         Execution
             │
       ┌─────┴─────┐
       ▼           ▼
 Observations     Results
       │           │
       └─────┬─────┘
             ▼
       Quantum Resilience
             │
     ┌───────┼────────┐
     ▼       ▼        ▼
   Continue Adapt   Recover
             │
       ┌─────┼─────┐
       ▼     ▼     ▼
    Route Schedule Compile
             │
             ▼
          Execute
             │
             ▼
          Verify
             │
       ┌─────┴─────┐
       ▼           ▼
    Accept      Repeat/Escalate

The feedback loop is a first-class architectural feature.

---

3. Definition of "write once, scale everywhere"

A Zamani program must not contain machine-specific assumptions such as:

use physical qubit 7
use backend X
use exactly 127 qubits
retry three times
if fidelity < 0.99
route q0 to physical 4
schedule gate at 100 ns

Instead, the program expresses:

- logical computation;
- semantic requirements;
- optional resource requirements;
- correctness requirements;
- resilience objectives;
- acceptable trade-offs.

The compilation/execution infrastructure determines how those requirements can be satisfied on the selected target.

The same source program may therefore be specialized for:

one-qubit system
small QPU
large QPU
multi-chip QPU
heterogeneous QPU fleet
logical-qubit system
fault-tolerant system
distributed quantum system
quantum network
future quantum architecture

without changing its quantum semantics.

---

4. Meaning of "infinity"

"Infinite scalability" is an architectural requirement, not a claim that physical hardware has infinite resources.

The requirement is:

«"quantum::resilience" must introduce no artificial finite machine-size ceiling.»

Actual executions are naturally bounded by:

- available memory;
- CPU/GPU resources;
- target resources;
- available qubits;
- available logical qubits;
- communication capacity;
- execution time;
- storage;
- operating-system limits;
- provider limits;
- caller budgets;
- security limits;
- configured resource policies.

Those are runtime constraints, not Zamani semantic limits.

Therefore the resilience subsystem MUST NOT contain architectural constants such as:

const MAX_QUBITS: usize = 127;
const MAX_PHYSICAL_QUBITS: usize = 1000;
const MAX_RECOVERY_ATTEMPTS: usize = 3;
const MAX_INCIDENTS: usize = 100;

A limit is valid only when it originates from:

target capability
caller policy
execution budget
security policy
resource availability
memory budget
deadline
provider capability

---

5. Mandatory safety requirements

All resilience code MUST:

- compile on Rust 1.97 and Rust 1.97.1;
- remain compatible with Rust 2021;
- contain no "unsafe";
- avoid undefined behavior;
- avoid hidden global mutable state;
- avoid provider-specific assumptions in core modules;
- avoid fixed hardware sizes;
- use canonical quantum identities;
- preserve canonical quantum semantics;
- expose deterministic behavior where requested;
- make non-determinism explicit;
- preserve provenance;
- verify adapted execution;
- distinguish recoverable from non-recoverable failures;
- never silently alter program semantics;
- never silently discard faults;
- never treat uncertain diagnosis as certainty;
- never return an unverified recovered result as fully trusted.

---

6. Ownership boundaries

The following ownership boundaries are normative.

Responsibility| Authoritative subsystem
Zamani quantum semantics| "quantum::ir"
Logical qubit identity| "quantum::ir::qubit"
Physical qubit identity| "quantum::ir::qubit"
Quantum operations| "quantum::ir"
Quantum circuits| "quantum::ir"
Gate semantics| "quantum::ir"
Fault semantics| "quantum::zqn::fault"
Noise semantics| ZQN/noise subsystem
QEC| QEC subsystem
Routing| "quantum::routing"
Scheduling| "quantum::scheduling"
Optimization| "quantum::optimization"
Hardware capabilities| "quantum::hardware"
Hardware topology| "quantum::hardware"
Hardware calibration| "quantum::hardware"
Hardware execution| hardware/runtime boundary
Simulation| quantum simulation subsystem
Benchmarking| quantum benchmarking subsystem
Resilience decisions| "quantum::resilience"
Recovery orchestration| "quantum::resilience"
Verification of resilience actions| "quantum::resilience"

Resilience consumes these contracts.

It must not duplicate them.

---

7. Canonical quantum identity rule

This is a hard repository-wide invariant.

Where resilience needs a logical qubit identifier it MUST use:

crate::quantum::ir::qubit::QubitId

Where resilience needs a physical qubit identifier it MUST use:

crate::quantum::ir::qubit::PhysicalQubitId

The resilience subsystem MUST NOT define:

struct ResilienceQubitId(...);
struct LogicalQubitId(...);
struct PhysicalQubitId(...);
struct RecoveryQubitId(...);

or equivalent competing identities.

The existing Zamani architecture explicitly establishes canonical quantum identities under "quantum::ir::qubit"; scheduling and optimization documentation likewise prohibit competing quantum representations.

Resilience-specific identifiers are permitted only for resilience-owned concepts:

IncidentId
RecoveryId
PlanId
CheckpointId
ObservationId
DecisionId

These are not quantum identities.

---

8. Logical-to-physical mapping boundary

Resilience MUST NOT implement its own logical-to-physical mapping algorithm.

The valid flow is:

QubitId
   │
   ▼
quantum::routing
   │
   ▼
PhysicalQubitId

Resilience may request:

remap
reroute
recompile

but routing owns the actual mapping algorithm.

This prevents:

- duplicate topology logic;
- duplicate qubit allocation;
- inconsistent mappings;
- stale physical assignments;
- conflicting routing decisions.

---

9. Fault ownership

The canonical physical/realized fault model belongs to:

quantum::zqn::fault

The existing repository's ZQN fault architecture already defines canonical fault semantics including fault locations and specialized cases such as leakage, erasure, loss, and correlated faults.

Resilience MUST NOT create another physical fault ontology.

The distinction is:

ZQN Fault
    ↓
Resilience Observation
    ↓
Resilience Incident
    ↓
Diagnosis
    ↓
Recovery Plan

ZQN describes what happened.

Resilience decides what to do about it.

---

10. Resilience is not QEC

QEC owns:

- encoding;
- syndrome extraction;
- decoding;
- code-specific correction;
- logical error correction;
- code-specific ancilla behavior;
- decoder semantics.

Resilience owns:

- deciding whether QEC should be invoked;
- responding to QEC degradation;
- selecting an allowed QEC configuration;
- reacting to logical error signals;
- requesting code-distance/resource changes;
- deciding whether migration is required;
- deciding whether execution must stop.

Resilience MUST NOT implement a decoder.

---

11. Resilience is not mitigation

Error mitigation and error suppression are execution techniques.

Resilience decides whether and when to use them.

The mitigation subsystem owns:

- readout mitigation;
- zero-noise extrapolation;
- probabilistic error cancellation;
- twirling;
- dynamical decoupling;
- future mitigation strategies.

Current quantum execution practice explicitly treats techniques such as dynamical decoupling, measurement mitigation, twirling, ZNE and PEC as selectable noise-management mechanisms with different overhead/correctness trade-offs.

Resilience therefore asks:

Which strategy is appropriate?

Mitigation answers:

How is that strategy executed?

---

12. Resilience is not scheduling

Scheduling owns:

- temporal ordering;
- durations;
- resource conflicts;
- timing constraints;
- execution windows;
- scheduling policies.

Resilience may request:

reschedule

but must not duplicate scheduling algorithms.

OpenQASM demonstrates why explicit timing and hardware-dependent duration resolution need to remain separate from higher-level program semantics.

---

13. Resilience is not routing

Routing owns:

- connectivity;
- placement;
- movement;
- mapping;
- topology-aware transformations.

Resilience only detects that an existing route may no longer be valid and requests an alternative.

---

14. Resilience is not optimization

Optimization owns equivalent transformations of canonical quantum IR.

Resilience may request:

reoptimization

after target conditions change.

Optimization MUST continue operating on:

crate::quantum::ir::Gate
crate::quantum::ir::QuantumOperation
crate::quantum::ir::QuantumCircuit
crate::quantum::ir::qubit::QubitId

and must not create a resilience-specific quantum circuit representation.

The repository's optimization architecture already explicitly prohibits competing quantum semantic types.

---

15. Resilience is not hardware

Hardware owns:

- device identity;
- technology;
- capabilities;
- topology;
- timing;
- calibration;
- instruction sets;
- execution;
- backend/provider adapters;
- health information originating from the target.

Resilience consumes snapshots/contracts from hardware.

Core resilience code MUST NOT contain:

IBM
Rigetti
Quantinuum
QuEra
IonQ
specific provider IDs
specific QPU sizes
specific topology shapes

Provider-specific behavior belongs under hardware adapters.

The repository's hardware architecture already separates provider-neutral contracts from concrete adapters.

---

16. Resilience directory

The production architecture is:

src/quantum/resilience/
│
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

The directory is extensible.

A file is added only when it introduces a distinct stable responsibility.

---

17. Dependency layers

The dependency direction MUST be acyclic.

errors
  ↓
model
  ↓
policy
  ↓
telemetry / detection
  ↓
diagnosis
  ↓
planning
  ↓
adaptation
  ↓
recovery / mitigation
  ↓
verification
  ↓
state / checkpoint / history
  ↓
API

Cross-cutting infrastructure:

serialization
limits
registry
coordination
learning

must depend only on stable contracts and must not create circular dependencies.

---

18. Forbidden dependency direction

The following are forbidden:

hardware → resilience implementation
routing → resilience implementation
scheduling → resilience implementation
optimization → resilience implementation
ZQN fault model → resilience implementation
canonical IR → resilience implementation

Those systems may expose contracts that resilience consumes.

This ensures resilience remains an orchestration layer instead of becoming a dependency of every quantum subsystem.

---

19. "errors/"

"errors/codes.rs"

Owns stable machine-readable resilience error codes.

Examples:

RESILIENCE-DET-*
RESILIENCE-DIAG-*
RESILIENCE-PLAN-*
RESILIENCE-ADAPT-*
RESILIENCE-REC-*
RESILIENCE-MIT-*
RESILIENCE-VERIFY-*
RESILIENCE-CHK-*
RESILIENCE-SER-*
RESILIENCE-SEC-*

Codes must remain stable after publication.

Provider names MUST NOT appear in core resilience error codes.

Integration:

all resilience modules
        ↓
errors::codes

---

"errors/classification.rs"

Defines semantic error classes such as:

Transient
Recoverable
Persistent
NonRecoverable
Unknown
SafetyCritical
SemanticRisk
SecurityViolation

This classification must be independent of any provider.

Integration:

detection
diagnosis
policy
planning
recovery
verification

---

"errors/error.rs"

Owns the canonical resilience error type.

It must support:

- stable code;
- category;
- severity;
- source error;
- retryability;
- recoverability;
- semantic risk;
- structured context;
- provenance.

It must never rely on string parsing for control flow.

---

"errors/mod.rs"

Only:

- declares error modules;
- re-exports stable public error contracts.

It contains no business logic.

---

20. "model/"

The model layer is the stable vocabulary of resilience.

It must not depend on concrete recovery implementations.

---

"model/resource.rs"

Represents resilience-visible execution resources.

Possible resource categories:

Backend
Device
LogicalQubit
PhysicalQubit
Coupling
ControlChannel
MeasurementChannel
ClassicalResource
Memory
ExecutionSlot
CommunicationLink
QECResource

It must support dynamic resource sets.

Quantum qubit references MUST use:

crate::quantum::ir::qubit::QubitId
crate::quantum::ir::qubit::PhysicalQubitId

when those canonical identities are applicable.

It must not impose a maximum resource count.

---

"model/confidence.rs"

Represents uncertainty.

A diagnosis or observation must be able to express:

confidence
evidence
source
uncertainty

The system must never convert "unknown" into "healthy."

---

"model/severity.rs"

Defines resilience severity independently from actions.

Suggested semantic states:

Informational
Degraded
Major
Critical
Fatal

Severity does not itself determine recovery.

Policy does.

---

"model/fault.rs"

Represents a resilience-normalized view of a canonical ZQN fault.

It should reference rather than replace ZQN semantics.

Conceptually:

ZQN Fault
   ↓
normalized resilience observation
   ↓
Resilience Fault View

It must preserve:

- canonical fault identity;
- fault location;
- fault classification;
- confidence;
- source;
- timestamp/ordering metadata;
- provenance.

---

"model/incident.rs"

Groups multiple related faults/observations into a resilience incident.

This prevents:

100 correlated failures

from becoming:

100 independent recoveries

The incident model must support:

- correlation;
- causality hypotheses;
- affected resources;
- severity;
- confidence;
- lifecycle state.

---

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

Health applies to abstract resources rather than a fixed hardware topology.

---

"model/degradation.rs"

Represents partial resource degradation.

Example:

available capacity
      ↓
100%
      ↓
95%
      ↓
80%
      ↓
60%

The resilience planner must determine whether the computation can continue.

---

"model/capability.rs"

Represents the capabilities currently available to resilience.

This is a view over hardware/runtime capabilities, not a replacement for the hardware capability model.

It must support:

- qubit capacity;
- logical capacity;
- physical capacity;
- instruction availability;
- connectivity;
- timing;
- measurement;
- reset;
- dynamic control;
- QEC capabilities;
- mitigation capabilities;
- migration capabilities;
- checkpoint capabilities.

No fixed size.

---

21. "detection/"

Detection answers:

«"What appears to have happened?"»

It does not decide recovery.

---

"detection/detector.rs"

Defines the detector contract.

A detector consumes observations and emits normalized resilience events.

It must support multiple detectors concurrently.

No detector may assume it is the sole source of truth.

---

"detection/anomaly.rs"

Generic anomaly detection.

Supported approaches may include:

- statistical;
- rule-based;
- historical;
- model-based.

Machine learning is optional.

Correctness MUST NOT depend on ML.

---

"detection/threshold.rs"

Threshold detection.

Thresholds must come from:

policy
configuration
target capability
calibration
benchmark

Never hard-code semantic values such as:

if fidelity < 0.99

---

"detection/statistical.rs"

Supports:

- distributions;
- variance;
- confidence intervals;
- outlier detection;
- sequential change detection;
- population changes.

---

"detection/drift.rs"

Detects:

- calibration drift;
- gate fidelity drift;
- readout drift;
- timing drift;
- noise drift;
- topology/capability drift.

---

"detection/timeout.rs"

Normalizes:

- compilation timeout;
- queue timeout;
- execution timeout;
- communication timeout;
- measurement timeout;
- backend response timeout.

---

"detection/execution_failure.rs"

Normalizes failures returned by the execution boundary.

It must not contain provider-specific retry logic.

---

"detection/qec_signal.rs"

Consumes QEC signals such as:

- syndrome observations;
- logical-error indicators;
- decoder confidence;
- leakage indicators;
- erasure indicators;
- logical failure probability.

It does not implement decoding.

---

"detection/hardware_signal.rs"

Consumes hardware health/capability/telemetry snapshots.

The hardware subsystem remains the authority for hardware facts.

---

22. "diagnosis/"

Diagnosis answers:

«"What is the most plausible explanation?"»

Diagnosis must represent uncertainty.

---

"diagnosis/classifier.rs"

Classifies incidents such as:

Noise
HardwareFailure
ResourceLoss
RoutingFailure
SchedulingFailure
BackendFailure
ExecutionFailure
QECDegradation
CompilerFailure
CommunicationFailure
Timeout
SecurityFailure
Unknown

The classification must be extensible.

---

"diagnosis/correlation.rs"

Correlates multiple observations.

This is essential for large systems where one root cause can affect many resources.

---

"diagnosis/localization.rs"

Identifies affected scope:

backend
device
region
logical qubit
physical qubit
coupling
operation
execution stage
communication path

Canonical quantum identities must be used.

---

"diagnosis/root_cause.rs"

Represents a causal hypothesis.

It must distinguish:

observed fact
inference
hypothesis
confidence

The system must never report an inferred cause as an observed fact.

---

"diagnosis/confidence.rs"

Calculates confidence from available evidence.

Low-confidence diagnoses may result in:

observe more
safe retry
conservative recovery
escalation

rather than aggressive adaptation.

---

"diagnosis/diagnostician.rs"

Composes:

observations
+
history
+
capabilities
+
topology
+
calibration
+
execution context

into:

Diagnosis

---

23. "policy/"

Policy defines what the system is allowed and expected to do.

Policy does not execute recovery.

---

"policy/constraints.rs"

Defines semantic constraints such as:

- required correctness;
- maximum tolerated logical error;
- maximum execution time;
- allowed resource usage;
- migration permission;
- recompile permission;
- mitigation permission;
- QEC adaptation permission.

---

"policy/objectives.rs"

Objectives may include:

Correctness
Fidelity
Availability
Latency
Cost
Energy
ResourceUsage
LogicalErrorProbability

Objectives must support multi-objective planning.

---

"policy/budgets.rs"

Budgets include:

time
shots
compilation
memory
resource
mitigation
recovery
migration

There must be no implicit retry count.

---

"policy/retry.rs"

Retry policy must distinguish:

safe retry
unsafe retry
semantically ambiguous retry
non-retryable failure

A retry is allowed only when replay semantics are known.

---

"policy/escalation.rs"

Defines when automatic recovery must stop.

Possible outcomes:

continue
recover
retry
migrate
escalate
abort

---

"policy/safety.rs"

The safety policy is a hard gate.

It must prevent actions that:

- alter program semantics;
- exceed explicit constraints;
- bypass verification;
- destroy provenance;
- hide a failure;
- exceed security authority;
- use untrusted resources;
- restore incompatible checkpoints.

---

"policy/policy.rs"

Composes the complete policy.

Policy is immutable for a given planning decision.

A policy may be replaced for a future execution, but a recovery decision must retain the policy snapshot that produced it.

---

24. "planning/"

Planning converts:

diagnosis
+
policy
+
capabilities
+
state
+
history
+
cost

into:

RecoveryPlan

---

"planning/action.rs"

Defines canonical actions:

Continue
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
Escalate

Actions are descriptions.

They are not implementations.

---

"planning/plan.rs"

An immutable recovery plan must contain:

- incident;
- diagnosis;
- policy snapshot;
- preconditions;
- actions;
- expected effects;
- cost;
- risk;
- confidence;
- verification requirements;
- rollback/recovery path;
- provenance.

---

"planning/cost.rs"

Provides a provider-independent cost model.

Possible dimensions:

execution time
compilation time
shots
memory
qubits
logical qubits
energy
financial cost
expected error
migration cost
verification cost

No provider-specific price logic belongs here.

---

"planning/feasibility.rs"

Determines whether a proposed action can satisfy:

- capability requirements;
- resource requirements;
- policy;
- timing;
- semantic constraints;
- security constraints.

---

"planning/ranking.rs"

Ranks feasible plans.

Ranking must be deterministic when deterministic mode is requested.

---

"planning/planner_state.rs"

Stores planner-local state.

It must be immutable where possible.

No global mutable planner state.

---

"planning/planner.rs"

The main planner composes all planning contracts.

It does not directly execute anything.

---

25. "adaptation/"

Adaptation modifies the execution strategy while preserving program semantics.

---

"adaptation/remapping.rs"

Requests logical-to-physical remapping through routing.

It must not implement its own mapping algorithm.

---

"adaptation/rerouting.rs"

Requests routing recomputation after:

- resource failure;
- topology change;
- capability change;
- calibration degradation.

---

"adaptation/rescheduling.rs"

Requests scheduling recomputation after:

- duration change;
- resource loss;
- routing change;
- calibration change;
- QEC change;
- mitigation insertion.

The existing scheduling subsystem explicitly owns timing and must continue to use canonical IR identities rather than defining competing quantum identities.

---

"adaptation/recompilation.rs"

Requests recompilation when:

- target capabilities change;
- instruction set changes;
- routing changes;
- QEC configuration changes;
- optimization requirements change.

Resilience must not implement a second compiler.

---

"adaptation/reoptimization.rs"

Requests optimization using canonical quantum IR.

No resilience-specific "QuantumCircuit".

---

"adaptation/qec_adaptation.rs"

Requests an allowed QEC change.

Possible dimensions:

code
distance
decoder
layout
ancilla allocation
syndrome strategy
logical resource allocation

The actual QEC algorithms remain outside resilience.

---

"adaptation/backend_selection.rs"

Selects a compatible target based on capabilities and policy.

It must never contain:

if provider == X

as core policy.

---

"adaptation/adapter.rs"

Defines the adaptation orchestration boundary.

It coordinates requests to:

routing
scheduling
optimization
compiler
QEC
hardware

without owning their implementations.

---

26. "recovery/"

Recovery turns an approved plan into execution actions.

---

"recovery/recoverer.rs"

Main recovery orchestration interface:

Incident
    ↓
RecoveryPlan
    ↓
PreconditionCheck
    ↓
Execute
    ↓
Verify

---

"recovery/retry.rs"

Retries only operations whose semantics permit replay.

The implementation MUST NOT assume that all quantum executions are safely replayable.

---

"recovery/restart.rs"

Restarts from an approved safe boundary.

---

"recovery/checkpoint.rs"

Coordinates checkpoint-aware recovery.

It does not own checkpoint serialization.

---

"recovery/rollback.rs"

Restores a valid prior execution state where the execution model supports it.

---

"recovery/resume.rs"

Resumes from a verified checkpoint or execution boundary.

---

"recovery/migration.rs"

Coordinates migration between compatible targets.

Migration must verify:

semantic compatibility
resource compatibility
checkpoint compatibility
QEC compatibility
result compatibility
security authorization

---

"recovery/compensation.rs"

Provides mathematically valid compensating actions where rollback is impossible.

It must never treat quantum state as ordinary mutable application state.

---

27. Quantum checkpoint rule

Arbitrary unknown quantum state MUST NOT be represented as though it were always serializable.

A checkpoint may represent:

classical execution state
compiled representation
logical execution boundary
measurement boundary
QEC state where explicitly supported
provider-supported resumable state
reconstructible computation state

It must not falsely claim:

serialize arbitrary unknown quantum state

This distinction is fundamental.

---

28. "mitigation/"

Mitigation is optional and policy-driven.

---

"mitigation/strategy.rs"

Defines the strategy contract.

A strategy must declare:

- required capabilities;
- expected overhead;
- expected benefit;
- semantic restrictions;
- verification requirements.

---

"mitigation/selection.rs"

Selects a mitigation strategy according to:

observed noise
hardware capabilities
workload
cost
accuracy target
policy

---

"mitigation/readout.rs"

Readout/measurement mitigation.

---

"mitigation/zero_noise.rs"

Zero-noise extrapolation.

Noise factors and extrapolators are configuration/policy inputs.

They must not be hard-coded.

---

"mitigation/probabilistic.rs"

Probabilistic error mitigation/cancellation abstractions.

---

"mitigation/twirling.rs"

Gate/measurement twirling abstractions.

---

"mitigation/dynamical_decoupling.rs"

Dynamical decoupling integration.

This must consume scheduling/hardware timing contracts rather than manipulating hardware directly.

Dynamical decoupling is inherently timing-sensitive; current quantum tooling treats it as a scheduling-aware error-suppression transformation.

---

"mitigation/custom.rs"

Extension point for future strategies.

---

"mitigation/executor.rs"

Executes an approved mitigation strategy through the normal compilation/scheduling/hardware pipeline.

---

29. "verification/"

Verification is the final trust boundary.

---

"verification/invariant.rs"

Defines invariants that must remain true after adaptation.

Examples:

logical identities preserved
operation semantics preserved
measurement semantics preserved
required operations preserved
resource constraints respected
QEC constraints respected
policy constraints respected

---

"verification/semantic.rs"

Compares the adapted executable representation against canonical program semantics.

It must detect semantic drift.

---

"verification/result.rs"

Validates result structure and consistency.

---

"verification/confidence.rs"

Computes confidence in the final result.

A result may be:

verified
verified_with_degradation
uncertain
rejected

---

"verification/provenance.rs"

Every accepted result must be traceable to:

source program
program identity/hash
canonical IR identity
optimization profile
routing decision
schedule
target identity
capability snapshot
calibration snapshot where applicable
fault observations
diagnosis
policy
plan
adaptations
recovery actions
mitigation
QEC configuration
execution
verification

---

"verification/acceptance.rs"

Final decision:

ACCEPT
DEGRADED_ACCEPT
RETRY
RECOVER
ESCALATE
REJECT

No implicit success.

---

"verification/verifier.rs"

Composes all verification checks.

Verification MUST be performed after recovery/adaptation before a result becomes trusted.

---

30. "state/"

---

"state/machine.rs"

Tracks high-level target/resource state.

---

"state/execution.rs"

Tracks current execution state.

---

"state/logical.rs"

Tracks logical resource state.

It must use canonical logical qubit identities.

---

"state/physical.rs"

Tracks physical resource state.

It must use:

crate::quantum::ir::qubit::PhysicalQubitId

where applicable.

---

"state/recovery.rs"

Defines the resilience state machine:

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

Transitions must be explicit.

---

"state/persistence.rs"

Persists state using the serialization/checkpoint boundary.

No hidden global state.

---

31. "checkpoint/"

---

"checkpoint/checkpoint.rs"

Defines the high-level checkpoint contract.

---

"checkpoint/snapshot.rs"

Stores checkpoint metadata and state references.

---

"checkpoint/manifest.rs"

Records:

- schema;
- content;
- versions;
- dependencies;
- target compatibility;
- hashes.

---

"checkpoint/storage.rs"

Storage abstraction.

It must not require a particular filesystem/cloud provider.

---

"checkpoint/integrity.rs"

Validates checkpoint integrity.

---

"checkpoint/compatibility.rs"

Determines whether a checkpoint is compatible with:

- current program;
- IR version;
- target;
- QEC configuration;
- resilience schema.

---

32. "telemetry/"

Telemetry is observation, not decision.

---

"telemetry/event.rs"

Canonical event model.

---

"telemetry/metric.rs"

Metrics may include:

error rate
logical error rate
fidelity
readout error
gate error
latency
queue time
recovery rate
retry rate
failure rate
mitigation overhead
verification failure rate

Metrics must be extensible.

---

"telemetry/trace.rs"

End-to-end trace:

program
→ compile
→ route
→ schedule
→ execute
→ fault
→ detect
→ diagnose
→ recover
→ verify

---

"telemetry/health.rs"

Health observations.

---

"telemetry/collector.rs"

Collects observations from:

- hardware;
- runtime;
- QEC;
- execution;
- compiler;
- scheduling;
- routing.

---

"telemetry/exporter.rs"

Provides export interfaces without forcing the core to depend on a particular observability vendor.

---

33. "history/"

History provides evidence for future decisions.

---

"history/incident.rs"

Incident history.

---

"history/execution.rs"

Execution outcomes.

---

"history/recovery.rs"

Recovery outcomes.

---

"history/statistics.rs"

Aggregated historical statistics.

History MUST NOT silently become a source of semantic truth.

It is evidence.

---

34. "learning/"

Learning is optional.

Correctness MUST NOT depend on machine learning.

---

"learning/features.rs"

Produces deterministic features from approved observations.

---

"learning/model.rs"

Defines an abstract prediction model.

---

"learning/predictor.rs"

May predict:

failure probability
recovery success probability
expected fidelity
expected latency

---

"learning/strategy.rs"

Uses predictions to rank strategies.

---

"learning/feedback.rs"

Feeds only verified outcomes into learning.

A failed recovery must not be recorded as a successful recovery merely because execution returned a result.

---

35. "coordination/"

Distributed resilience is required for future multi-QPU execution.

---

"coordination/ownership.rs"

Determines which resilience controller owns a decision.

---

"coordination/lease.rs"

Provides time-bounded ownership of recovery actions/resources.

---

"coordination/distributed.rs"

Coordinates resilience across execution nodes.

---

"coordination/consensus.rs"

Defines the contract where distributed agreement is actually required.

The subsystem MUST NOT invent a consensus algorithm merely for architectural completeness.

---

"coordination/coordinator.rs"

Coordinates distributed resilience operations.

---

36. "serialization/"

All externally persisted resilience objects require versioned serialization.

---

"serialization/schema.rs"

Defines serialized schema.

It must not redefine quantum semantics.

---

"serialization/version.rs"

Schema versioning.

Must support explicit compatibility policies.

---

"serialization/encode.rs"

Deterministic encoding.

---

"serialization/decode.rs"

Validated decoding.

Untrusted serialized data must be treated as hostile input.

---

"serialization/mod.rs"

Only composition/re-export.

---

37. "limits/"

Limits are explicit policies.

---

"limits/resource.rs"

Resource limits derived from actual available resources.

---

"limits/validation.rs"

Checks requested actions against limits.

---

"limits/limits.rs"

Composes:

resource limits
time limits
memory limits
security limits
execution limits
policy limits

No architectural maximums.

---

38. "registry/"

Registries enable extension without modifying core orchestration.

---

"registry/detector.rs"

Detector registration.

---

"registry/strategy.rs"

Mitigation/adaptation strategy registration.

---

"registry/recovery.rs"

Recovery strategy registration.

---

"registry/backend.rs"

Backend capability/adaptation integration registration.

Registries must be:

- explicitly scoped;
- deterministic when required;
- validated;
- collision-safe;
- version-aware.

---

39. "api/"

The API layer is the only preferred public orchestration boundary.

---

"api/request.rs"

Immutable resilience request.

Contains references/contracts for:

- program/executable representation;
- target context;
- policy;
- constraints;
- objectives;
- execution requirements.

---

"api/context.rs"

Execution context.

It must provide contracts for:

canonical IR
ZQN
QEC
routing
scheduling
optimization
hardware
runtime
telemetry

It must not own concrete provider connections.

---

"api/response.rs"

Immutable response containing:

execution outcome
verification
provenance
recovery history
diagnostics
resource usage
confidence

---

"api/controller.rs"

Top-level orchestration:

observe
→ detect
→ diagnose
→ policy
→ plan
→ adapt
→ recover
→ verify

The controller coordinates.

It does not implement every algorithm.

---

40. Integration with canonical IR

Resilience receives canonical IR from:

crate::quantum::ir

It must never create a second semantic circuit model.

All references to quantum operations must ultimately resolve to canonical IR.

The repository's scheduling architecture explicitly states that scheduling must not introduce another "QuantumOperation", "QuantumCircuit", "Gate", or "QubitId".

The same rule applies to resilience.

---

41. Integration with "quantum::ir::qubit"

The canonical identity boundary is:

crate::quantum::ir::qubit::QubitId
crate::quantum::ir::qubit::PhysicalQubitId

Any resilience file involving qubits must import those exact canonical types.

Examples:

use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};

No local aliases that create a competing semantic type.

Type aliases are acceptable only when they remain aliases to the canonical identity and do not create a new semantic identity.

---

42. Integration with ZQN

The integration is:

quantum::zqn::fault
        │
        ▼
resilience::model::fault
        │
        ▼
incident
        │
        ▼
diagnosis

ZQN remains responsible for fault semantics.

The ZQN module itself explicitly describes the fault layer as a semantic provider rather than an orchestrator.

Resilience is the orchestrator.

---

43. Integration with hardware

The integration is:

hardware
    │
    ├── identity
    ├── technology
    ├── capabilities
    ├── instruction set
    ├── timing
    ├── topology
    ├── calibration
    ├── health
    └── execution
          │
          ▼
resilience

Resilience consumes snapshots/contracts.

It must not directly implement provider APIs.

---

44. Integration with routing

The integration is:

resilience
    │
    └── request reroute/remap
              │
              ▼
       quantum::routing
              │
              ▼
       mapped executable form

Resilience must not know routing algorithm internals.

---

45. Integration with scheduling

The integration is:

resilience
    │
    └── request reschedule
              │
              ▼
     quantum::scheduling
              │
              ▼
          schedule

Scheduling remains the owner of temporal execution.

---

46. Integration with optimization

The integration is:

resilience
    │
    └── request reoptimization
              │
              ▼
     quantum::optimization
              │
              ▼
      canonical quantum IR

Optimization remains canonical-IR based.

---

47. Integration with QEC

The integration is:

QEC
 │
 ├── syndrome
 ├── decoder
 ├── logical state
 ├── logical error indicators
 └── code capabilities
          │
          ▼
     resilience
          │
          ├── continue
          ├── adapt
          ├── migrate
          └── escalate

Resilience never implements the decoder.

---

48. Integration with simulation

The simulator must be capable of injecting:

fault
noise
resource loss
timing changes
backend failures
QEC failures

and observing the complete resilience lifecycle without physical hardware.

This enables deterministic fault-injection testing.

---

49. Integration with benchmarking

Benchmarking provides evidence such as:

historical error rates
logical error rates
execution latency
resource stability
calibration stability
recovery success rate
mitigation overhead

Resilience may consume benchmark evidence.

Benchmarking remains the authority for benchmark methodology.

---

50. Integration with runtime

Runtime is responsible for actual execution.

Resilience may command:

execute
retry
resume
restart
migrate
cancel

through a runtime contract.

Resilience must not duplicate runtime process management.

---

51. Dynamic execution

Quantum programs may include runtime classical control.

OpenQASM 3 explicitly supports classical feed-forward and real-time classical computation, and its timing model separates program timing intent from target-specific timing realization.

Zamani resilience must therefore treat execution as potentially dynamic.

A resilience decision may occur:

before execution
between stages
between QEC rounds
after measurement
after classical feedback
between shots
between batches
after an execution failure

It must not assume every quantum workload is one static circuit.

---

52. Resilience during dynamic circuits

For dynamic execution:

quantum operation
      ↓
measurement
      ↓
classical result
      ↓
condition
      ↓
next quantum operation

resilience must preserve:

- dependency;
- causality;
- measurement semantics;
- classical condition semantics;
- timing constraints;
- QEC control semantics.

A recovery action that invalidates a classical condition is not a valid recovery.

---

53. Distributed quantum execution

For distributed execution:

Program
  │
  ▼
Global logical representation
  │
  ├── QPU A
  ├── QPU B
  ├── QPU C
  └── ...

Resilience must handle:

- node failure;
- communication failure;
- link degradation;
- resource partition;
- synchronization failure;
- partial execution;
- ownership conflicts;
- migration.

No fixed number of nodes may be assumed.

---

54. Failure containment

A local fault should remain local whenever possible.

Example:

one physical qubit fails

must not automatically imply:

entire distributed computation fails

The planner must determine affected scope.

This requires:

fault localization
dependency analysis
resource dependency analysis
logical dependency analysis

---

55. Correlated failures

A correlated fault must be represented as a correlated incident.

For example:

multiple qubits
multiple gates
same control channel
same calibration region

may share one cause.

The planner must avoid independent recovery actions that amplify the original failure.

---

56. Recovery safety

Every recovery action must have:

preconditions
execution action
expected effects
postconditions
verification requirements
failure path
rollback/recovery path

No action is valid merely because its implementation returned "Ok".

---

57. Retry semantics

A retry is valid only when all required conditions are satisfied.

Examples:

transient backend transport error
+
execution known not to have committed

may allow retry.

But:

unknown quantum state after partial execution

does not automatically allow replay.

The retry subsystem must therefore understand execution boundaries and commitment state.

---

58. Backend migration

Migration must distinguish:

same semantic target
different physical target
different QEC target
different instruction set
different topology
different timing model
different noise model

A migrated execution must be revalidated.

---

59. Semantic equivalence requirement

After adaptation:

original program semantics
        ≡
adapted execution semantics

within the explicitly declared semantic model.

This is a stronger requirement than:

same number of gates

or:

same schedule length

---

60. Provenance requirement

Every resilience decision must be reproducible from recorded information.

At minimum:

program identity
IR identity
target identity
capability snapshot
policy identity
policy version
observations
diagnosis
plan
adaptations
recovery actions
mitigation
QEC state/configuration
verification
result

---

61. Determinism

Deterministic mode must be explicit.

Given equivalent:

program
IR
target snapshot
observations
policy
history snapshot
random seed

the planner must produce the same result.

If randomness is deliberately used:

- seed must be explicit;
- seed must be recorded;
- random choices must be provenance-tracked.

No hidden randomness.

---

62. Concurrency

Concurrency is allowed.

However:

- shared mutable state must be synchronized;
- ownership must be explicit;
- recovery actions must be serialized when they conflict;
- duplicate recovery must be prevented;
- deterministic mode must define ordering.

The architecture must never rely on unspecified thread scheduling for semantic behavior.

---

63. Memory scalability

Large workloads must not require unnecessary duplication of:

QuantumCircuit
QuantumOperation
QubitId arrays
telemetry
history
diagnostic evidence

Where appropriate:

- use references;
- use streaming;
- use iterators;
- use bounded buffers from explicit policies;
- use incremental aggregation;
- use compact identifiers;
- avoid cloning large canonical IR unnecessarily.

No unsafe memory optimization is permitted.

---

64. Streaming telemetry

Telemetry must support streaming.

The resilience system must not require:

store every event forever

in memory.

Retention belongs to explicit policy.

The system should support:

stream
→ aggregate
→ persist
→ discard

when permitted.

---

65. Large-scale incident handling

Incident aggregation must avoid:

O(number_of_all_historical_faults)

work for every new event where unnecessary.

The implementation should support:

- indexed resource identity;
- time windows;
- correlation windows;
- incremental aggregation;
- hierarchical incidents.

The architecture must remain resource-count independent.

---

66. Hierarchical resilience

Large systems should support:

system
 ├── region
 │    ├── device
 │    │    ├── qubit
 │    │    └── channel
 │    └── ...
 └── ...

A local incident can be handled locally.

A regional incident can be escalated.

A system-wide incident can be globally coordinated.

The number of hierarchy levels must not be hard-coded.

---

67. Resource abstraction

Resource IDs must be abstract enough to support future technologies.

Examples:

Qubit
PhysicalQubit
LogicalQubit
Coupling
OpticalMode
IonChain
Atom
ControlChannel
MeasurementChannel
CommunicationLink
Memory
ExecutionSlot

Technology-specific semantics belong in hardware capability contracts.

---

68. Capability negotiation

Before any adaptation:

requested action
        ↓
capability check
        ↓
feasible?

If not:

try another plan

The planner must never assume a capability exists.

---

69. Graceful degradation

When resources decrease:

full capacity
    ↓
degraded capacity
    ↓
replanning

The program should continue if its declared constraints remain satisfiable.

If not:

escalate

or:

reject

rather than silently changing semantics.

---

70. Security architecture

Telemetry is untrusted until authenticated/validated.

Checkpoints are untrusted until integrity-verified.

Plugins are untrusted until authorized.

Recovery actions require authorization.

Security concerns include:

- forged telemetry;
- replayed telemetry;
- tampered checkpoints;
- malicious backend reports;
- compromised plugins;
- malicious recovery instructions;
- provenance forgery;
- unauthorized migration;
- resource hijacking.

---

71. Trust levels

Observations should be able to express:

Trusted
Authenticated
Validated
Unverified
Conflicting
Unknown

An unverified observation must not automatically trigger high-impact recovery.

---

72. Plugin security

Plugins must not gain unrestricted access to:

- arbitrary process memory;
- credentials;
- unrelated resources;
- unapproved backends;
- hidden state.

The architecture must expose only the capabilities required by the plugin.

No "unsafe" plugin boundary is permitted in core resilience.

---

73. Observability

Every major transition should be observable:

DetectionStarted
DetectionCompleted
IncidentCreated
DiagnosisProduced
PlanGenerated
AdaptationRequested
RecoveryStarted
RecoveryCompleted
VerificationStarted
VerificationCompleted
ResultAccepted
ResultRejected
Escalated

Events must have stable identifiers and schema versions.

---

74. Auditability

An auditor must be able to answer:

What happened?
When?
Where?
Why was it classified this way?
What evidence existed?
What policy applied?
What action was chosen?
What alternatives existed?
What changed?
Was the result verified?
Why was it accepted?

---

75. Failure modes

The architecture must explicitly handle:

Unknown fault
Transient fault
Persistent fault
Correlated fault
Resource loss
Topology loss
Calibration drift
Readout degradation
Execution timeout
Queue timeout
Compiler timeout
Routing failure
Scheduling failure
QEC failure
Backend failure
Network failure
Checkpoint failure
Serialization failure
Security failure
Verification failure
Planner failure
Recovery failure
Conflicting telemetry
Insufficient resources
Unsupported target

---

76. Unknown failures

Unknown failures are first-class.

The system must not force an unknown failure into:

transient

just to enable recovery.

Correct behavior may be:

observe
isolate
conservative retry
escalate
abort

according to policy.

---

77. Recovery state machine

The normative state machine is:

                 ┌─────────────┐
                 │    IDLE     │
                 └──────┬──────┘
                        │
                        ▼
                 ┌─────────────┐
                 │  DETECTING  │
                 └──────┬──────┘
                        │
                        ▼
                 ┌─────────────┐
                 │ DIAGNOSING  │
                 └──────┬──────┘
                        │
                        ▼
                 ┌─────────────┐
                 │  PLANNING   │
                 └──────┬──────┘
                        │
                        ▼
                 ┌─────────────┐
                 │  ADAPTING   │
                 └──────┬──────┘
                        │
                        ▼
                 ┌─────────────┐
                 │  RECOVERING │
                 └──────┬──────┘
                        │
                        ▼
                 ┌─────────────┐
                 │ VERIFYING   │
                 └──────┬──────┘
                        │
             ┌──────────┼──────────┐
             ▼          ▼          ▼
          ACCEPT      REPEAT    ESCALATE
             │          │          │
             ▼          │          ▼
          COMPLETE      │         FAILED
                        │
                        └──→ DETECTING

Any state transition must be explicit.

---

78. No silent transitions

The following are forbidden:

failure → silently retry
failure → silently migrate
failure → silently change QEC
failure → silently change program
failure → silently suppress evidence

Every significant recovery action must be represented in provenance.

---

79. Mitigation overhead

Mitigation may increase:

- circuit count;
- shot count;
- execution time;
- compilation time;
- scheduling complexity.

Therefore mitigation selection must consider total cost.

Current quantum tooling explicitly exposes resilience trade-offs in terms of increased sampling/execution overhead versus result quality.

---

80. Scheduling interaction with mitigation

A mitigation strategy such as dynamical decoupling can add operations during idle periods.

Therefore:

resilience
    ↓
mitigation selection
    ↓
transformation request
    ↓
scheduling
    ↓
hardware lowering

Resilience must not directly insert timing-dependent operations into hardware instructions.

---

81. QEC interaction with scheduling

QEC may introduce:

- syndrome rounds;
- ancilla operations;
- measurements;
- resets;
- classical feedback.

These must pass through the normal scheduling contract.

Resilience may request QEC adaptation, but scheduling remains responsible for timing.

Dynamic quantum control requires the architecture to preserve the dependency between measurements and subsequent conditioned operations. OpenQASM 3 explicitly models this type of real-time classical control.

---

82. Compiler integration

Resilience may request:

partial recompilation
regional recompilation
full recompilation

The compiler must expose a contract that identifies:

affected region
required target capabilities
required semantic constraints

Resilience must not manipulate compiler internals.

---

83. Partial recovery

Recovery should prefer the smallest valid affected region.

Possible scope:

operation
layer
logical qubit
QEC region
subcircuit
execution stage
shot
batch
whole program

The scope must be derived from dependency analysis.

---

84. Shot-level recovery

For sampling workloads, one failed shot does not necessarily require abandoning the entire experiment.

Policy may permit:

discard failed shot
replace shot
increase sampling
re-run affected batch

provided the statistical semantics and declared accuracy requirements remain valid.

---

85. Statistical result integrity

Mitigation/recovery must preserve statistical provenance.

If the original experiment requested:

N samples

and recovery changes:

N

or sampling distribution, the final result must record that change.

No hidden statistical manipulation.

---

86. Result confidence

A recovered result must carry enough metadata to distinguish:

raw result
mitigated result
QEC-protected result
recovered result
replayed result
partially degraded result

The caller must be able to determine how the result was produced.

---

87. Backend selection

Backend selection must be capability-driven.

The planner should reason over:

required capabilities
available capabilities
policy
cost
fidelity
availability
latency
security

not provider names.

---

88. Hardware fleet scaling

The architecture must support:

one target
    ↓
many targets
    ↓
heterogeneous targets
    ↓
distributed execution

The number of backends must not be a compile-time constant.

---

89. Target snapshot

A resilience decision must operate against a target snapshot or equivalent consistent capability view.

This avoids:

read capability A
target changes
read capability B
plan based on A+B

without noticing the inconsistency.

---

90. Snapshot identity

A target snapshot should have a stable identity containing:

target identity
capability version
topology version
calibration version where applicable
timestamp/epoch
health version

The exact hardware-specific representation remains owned by hardware.

---

91. Time model

Resilience must distinguish:

wall-clock time
target execution time
logical execution time
deadline
timeout
duration
timestamp
ordering/sequence

It must not assume that all target clocks have the same resolution.

OpenQASM's timing model reinforces the need to represent timing intent separately from hardware-resolved timing.

---

92. Clock and timestamp integrity

Telemetry timestamps must identify their source and time basis where possible.

The resilience system must not assume that clocks are perfectly synchronized.

Distributed recovery must therefore support explicit ordering/epoch mechanisms.

---

93. Resource reservation

Recovery actions that modify resources must reserve them through the appropriate resource contract.

Examples:

new physical qubits
new QPU
new communication link
new execution slot
new ancilla

Resilience must not directly mutate hardware resource ownership.

---

94. Recovery idempotency

Where possible, recovery actions should be idempotent.

Every recovery action should have a unique identity:

RecoveryId

Repeated delivery of the same action must not accidentally execute it multiple times.

---

95. Cancellation

Long-running resilience operations must support cancellation where the underlying execution contract supports it.

Cancellation must itself be observable and verified.

---

96. Deadlines

Every planning/execution request may carry a deadline.

If no deadline is supplied, resilience must not manufacture an arbitrary semantic deadline.

System-level limits may still apply.

---

97. Resource exhaustion

If resilience itself becomes resource constrained:

memory pressure
CPU pressure
telemetry overload
history overload
planning timeout

it must degrade observability/planning according to explicit policy rather than crash unpredictably.

---

98. Deterministic degradation

When optional features cannot run, the system must have deterministic fallback behavior.

For example:

ML predictor unavailable
      ↓
use deterministic policy ranking

not:

random strategy

---

99. Learning fallback

Learning is never a prerequisite for correctness.

If:

learning model unavailable

the system must continue using:

policy
capability
cost
deterministic planning

---

100. Plugin fallback

If a plugin fails:

plugin failure

must not corrupt core resilience state.

The plugin may be:

disabled
quarantined
replaced

according to policy.

---

101. Serialization compatibility

Serialized resilience data must include:

schema version
component version
compatibility information

Unknown fields should be handled according to the versioning policy.

Incompatible data must fail explicitly.

---

102. No semantic duplication in serialization

Serialization modules must not redefine:

QuantumCircuit
QuantumOperation
Gate
QubitId
PhysicalQubitId

The scheduling architecture already follows this principle, and resilience must do the same.

Serialization is representation, not ownership.

---

103. Testing strategy

Testing must occur at multiple levels.

unit
property
fuzz
integration
fault injection
determinism
scalability
simulation
end-to-end
distributed
security
serialization
replay

---

104. Unit tests

Every implementation file must test:

- valid input;
- invalid input;
- empty input;
- boundary conditions;
- conflicting observations;
- unknown state;
- serialization;
- determinism;
- error propagation.

---

105. Property tests

Properties should include:

no invalid recovery plan accepted
no semantic invariant violated
no duplicate recovery identity
no invalid resource allocation
deterministic planner remains deterministic
serialization round-trip preserves semantics

---

106. Fault injection

"tests/fault_injection.rs" must support:

single-qubit fault
multi-qubit fault
correlated fault
leakage
loss
erasure
gate failure
measurement failure
readout failure
calibration drift
resource loss
topology loss
routing failure
scheduling failure
QEC failure
backend failure
network failure
timeout
checkpoint corruption
telemetry corruption

Canonical ZQN faults should be used rather than creating fake resilience-only quantum fault objects.

---

107. Scalability testing

Tests must not rely only on fixed examples such as:

1
10
100
1000

Instead, use generated resource models.

The tests must demonstrate that algorithms operate over arbitrary valid resource counts.

Test dimensions include:

qubits
operations
incidents
resources
backends
QEC rounds
distributed nodes
telemetry events

---

108. "Infinity" scalability test principle

The test requirement is:

«No test may reveal a resilience algorithm whose correctness depends on a fixed maximum machine size.»

The implementation should scale until an explicit resource limit is reached.

That limit belongs to the test environment, not the resilience semantics.

---

109. Determinism tests

For deterministic mode:

same input
+
same target snapshot
+
same observations
+
same policy
+
same seed
=
same plan

Repeated executions must produce equivalent results.

---

110. Replay testing

A recorded incident must be replayable.

Replay:

observations
+
target snapshot
+
policy
+
history snapshot

must reproduce the original resilience decision in deterministic mode.

---

111. Security tests

Tests must include:

forged telemetry
tampered checkpoint
invalid serialization
plugin failure
unauthorized migration
invalid recovery action
replayed recovery command
conflicting health signals
malicious resource identity

---

112. Integration test graph

The minimum end-to-end test is:

canonical IR
    ↓
optimization
    ↓
routing
    ↓
scheduling
    ↓
hardware abstraction
    ↓
execution simulator
    ↓
fault injection
    ↓
resilience detection
    ↓
diagnosis
    ↓
planning
    ↓
adaptation
    ↓
re-execution
    ↓
verification

---

113. Production readiness gate

"quantum::resilience" MUST NOT be declared production-ready until:

Correctness

- semantic verification works;
- no silent failure exists;
- recovery preserves semantics;
- uncertainty is represented;
- result acceptance is explicit.

Scalability

- no artificial qubit limit;
- no artificial operation limit;
- no fixed backend count;
- no fixed topology size;
- streaming telemetry works;
- large incidents are supported.

Safety

- no "unsafe";
- no hidden mutation;
- no unauthorized recovery;
- no unverified result acceptance.

Compatibility

- Rust 1.97;
- Rust 1.97.1;
- Rust 2021;
- canonical IR;
- canonical qubit identities;
- ZQN;
- routing;
- scheduling;
- optimization;
- hardware;
- QEC.

Reliability

- retry;
- restart;
- resume;
- checkpoint;
- rollback;
- migration;
- degraded execution;
- escalation.

Observability

- events;
- metrics;
- traces;
- provenance;
- audit history.

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

114. Implementation order

The implementation order is intentionally contract-first.

Phase 1 — independent foundational contracts

1. "errors/codes.rs"
2. "errors/classification.rs"
3. "errors/error.rs"
4. "model/resource.rs"
5. "model/confidence.rs"
6. "model/severity.rs"
7. "model/fault.rs"
8. "model/incident.rs"
9. "model/health.rs"
10. "model/degradation.rs"
11. "model/capability.rs"

These establish the vocabulary.

---

Phase 2 — policy contracts

12. "policy/constraints.rs"
13. "policy/objectives.rs"
14. "policy/budgets.rs"
15. "policy/retry.rs"
16. "policy/escalation.rs"
17. "policy/safety.rs"
18. "policy/policy.rs"

These establish what resilience is allowed to do.

---

Phase 3 — telemetry and detection

19. "telemetry/event.rs"
20. "telemetry/metric.rs"
21. "telemetry/trace.rs"
22. "telemetry/health.rs"
23. "detection/detector.rs"
24. "detection/threshold.rs"
25. "detection/statistical.rs"
26. "detection/anomaly.rs"
27. "detection/drift.rs"
28. "detection/timeout.rs"
29. "detection/execution_failure.rs"
30. "detection/qec_signal.rs"
31. "detection/hardware_signal.rs"

---

Phase 4 — diagnosis

32. "diagnosis/classifier.rs"
33. "diagnosis/correlation.rs"
34. "diagnosis/localization.rs"
35. "diagnosis/root_cause.rs"
36. "diagnosis/confidence.rs"
37. "diagnosis/diagnostician.rs"

---

Phase 5 — planning

38. "planning/action.rs"
39. "planning/cost.rs"
40. "planning/feasibility.rs"
41. "planning/ranking.rs"
42. "planning/plan.rs"
43. "planning/planner_state.rs"
44. "planning/planner.rs"

---

Phase 6 — adaptation

45. "adaptation/remapping.rs"
46. "adaptation/rerouting.rs"
47. "adaptation/rescheduling.rs"
48. "adaptation/recompilation.rs"
49. "adaptation/reoptimization.rs"
50. "adaptation/qec_adaptation.rs"
51. "adaptation/backend_selection.rs"
52. "adaptation/adapter.rs"

---

Phase 7 — recovery

53. "recovery/retry.rs"
54. "recovery/restart.rs"
55. "recovery/checkpoint.rs"
56. "recovery/rollback.rs"
57. "recovery/resume.rs"
58. "recovery/migration.rs"
59. "recovery/compensation.rs"
60. "recovery/recoverer.rs"

---

Phase 8 — mitigation

61. "mitigation/strategy.rs"
62. "mitigation/selection.rs"
63. "mitigation/readout.rs"
64. "mitigation/zero_noise.rs"
65. "mitigation/probabilistic.rs"
66. "mitigation/twirling.rs"
67. "mitigation/dynamical_decoupling.rs"
68. "mitigation/custom.rs"
69. "mitigation/executor.rs"

---

Phase 9 — verification

70. "verification/invariant.rs"
71. "verification/semantic.rs"
72. "verification/result.rs"
73. "verification/confidence.rs"
74. "verification/provenance.rs"
75. "verification/acceptance.rs"
76. "verification/verifier.rs"

---

Phase 10 — state

77. "state/machine.rs"
78. "state/execution.rs"
79. "state/logical.rs"
80. "state/physical.rs"
81. "state/recovery.rs"
82. "state/persistence.rs"

---

Phase 11 — checkpointing

83. "checkpoint/snapshot.rs"
84. "checkpoint/manifest.rs"
85. "checkpoint/integrity.rs"
86. "checkpoint/storage.rs"
87. "checkpoint/compatibility.rs"
88. "checkpoint/checkpoint.rs"

---

Phase 12 — history and learning

89. "history/incident.rs"
90. "history/execution.rs"
91. "history/recovery.rs"
92. "history/statistics.rs"
93. "learning/features.rs"
94. "learning/model.rs"
95. "learning/predictor.rs"
96. "learning/strategy.rs"
97. "learning/feedback.rs"

Learning remains optional.

---

Phase 13 — distributed coordination

98. "coordination/ownership.rs"
99. "coordination/lease.rs"
100. "coordination/distributed.rs"
101. "coordination/consensus.rs"
102. "coordination/coordinator.rs"

Consensus implementation must remain replaceable.

---

Phase 14 — registries and public API

103. "registry/detector.rs"
104. "registry/strategy.rs"
105. "registry/recovery.rs"
106. "registry/backend.rs"
107. "api/request.rs"
108. "api/context.rs"
109. "api/response.rs"
110. "api/controller.rs"

---

Phase 15 — serialization and limits

111. "serialization/schema.rs"
112. "serialization/version.rs"
113. "serialization/encode.rs"
114. "serialization/decode.rs"
115. "limits/resource.rs"
116. "limits/validation.rs"
117. "limits/limits.rs"

---

Phase 16 — composition

Only after stable contracts exist:

118. subdirectory "mod.rs" files;
119. root "resilience/mod.rs";
120. integration tests;
121. end-to-end tests;
122. scalability tests;
123. documentation synchronization.

---

115. "mod.rs" rule

Every "mod.rs" must remain a composition boundary.

It may:

- declare modules;
- re-export stable public types;
- document module ownership.

It must not:

- implement business logic;
- define duplicate quantum types;
- perform hardware discovery;
- create hidden global state;
- perform recovery;
- contain provider-specific behavior.

---

116. Root "quantum/mod.rs" integration

Once resilience is stable:

pub mod resilience;

must be added to:

src/quantum/mod.rs

No resilience implementation belongs in the quantum root.

---

117. API stability

The stable public surface should be intentionally small.

Prefer exposing:

ResilienceController
ResilienceRequest
ResilienceResponse
ResiliencePolicy
RecoveryPlan
VerificationResult
ResilienceError

rather than exposing every internal implementation.

Internal detector/planner/recovery types should remain internal unless there is a demonstrated external integration requirement.

---

118. Extension model

Future strategies must be addable without rewriting the controller.

For example:

new detector
new QEC strategy
new mitigation
new recovery mechanism
new backend
new predictor

should be registered through stable contracts.

The controller should not require:

match strategy {
    StrategyA => ...
    StrategyB => ...
    StrategyC => ...
}

for every future extension.

---

119. Provider neutrality

Core resilience MUST compile and operate without any concrete quantum provider SDK.

Provider integration belongs below:

quantum::hardware

or its adapters.

This enables:

hardware
simulator
emulator
test backend
future backend

to all participate in the same resilience architecture.

---

120. Simulator-first validation

Every resilience mechanism that can be tested without hardware should be testable against a simulator/mock execution contract.

The simulator should be able to model:

healthy target
degraded target
failed target
changing target

This makes recovery logic testable without real hardware.

---

121. Resource-aware planning

The planner must consider all required resources, not merely qubit count.

A plan may require:

qubits
logical qubits
ancilla
classical controller
measurement channels
control channels
memory
communication links
execution slots
time
shots

The resource model must remain extensible.

---

122. Cost-aware planning

A recovery plan is not automatically better because it has higher fidelity.

The planner must be able to compare:

correctness
fidelity
latency
cost
resource overhead
energy
risk

according to policy.

---

123. Multi-objective planning

The architecture must support objective functions such as:

maximize correctness
maximize fidelity
minimize latency
minimize cost
minimize resource usage
minimize expected failure probability

The selected policy determines priority.

No objective is globally hard-coded.

---

124. Graceful fallback hierarchy

A generic fallback order may be policy-configurable:

continue
    ↓
local adaptation
    ↓
local recovery
    ↓
recompile
    ↓
reroute
    ↓
reschedule
    ↓
mitigate
    ↓
QEC adaptation
    ↓
backend migration
    ↓
escalate
    ↓
abort

This is a conceptual model, not a mandatory fixed order.

Actual ordering is determined by policy and feasibility.

---

125. Recovery must preserve provenance

Every adaptation must record:

what changed
why it changed
who/what authorized it
which observation triggered it
which policy allowed it
which plan selected it
what verification followed

---

126. Recovery must not hide faults

A successful recovery does not erase the original incident.

The final record must retain:

fault
incident
diagnosis
recovery
verification

This is necessary for:

- auditing;
- learning;
- benchmarking;
- debugging;
- reproducibility.

---

127. Recovery and benchmarking

Recovered executions must be distinguishable from clean executions.

Otherwise benchmarking statistics become contaminated.

Benchmarking should be able to ask:

clean execution rate
recovered execution rate
mitigated execution rate
QEC-protected execution rate
failed execution rate

---

128. Resilience and optimization interaction

Optimization may change the physical manifestation of a program.

Therefore after optimization changes:

resource requirements
timing
noise exposure
QEC requirements

may change.

Resilience must treat the resulting canonical representation as authoritative.

It must not attempt to reason from stale pre-optimization assumptions.

---

129. Resilience and routing interaction

If a physical resource disappears:

old route
   ↓
invalid

resilience requests:

new route

Then scheduling and verification must run against the new route.

---

130. Resilience and scheduling interaction

If a gate duration changes:

old schedule
   ↓
possibly invalid

resilience requests:

new schedule

It must not patch timestamps directly unless scheduling explicitly provides such a safe contract.

---

131. Resilience and QEC interaction

If logical error probability rises:

observe
   ↓
diagnose
   ↓
policy
   ↓
possible QEC adaptation

Possible responses:

increase protection
change code
change layout
change decoder
reroute
migrate
abort

Only capabilities and policy determine what is valid.

---

132. Resilience and calibration

Calibration changes are observations.

Resilience may respond by:

invalidate schedule
invalidate route
recompile
reschedule
rerun calibration-dependent analysis
migrate

The calibration subsystem remains authoritative for calibration data.

---

133. Resilience and topology changes

If topology changes:

capability snapshot
       ↓
new topology
       ↓
existing mapping may be invalid

The planner must request rerouting.

It must not assume topology is static.

---

134. Resilience and hardware replacement

A physical device can be replaced while logical computation remains valid.

The architecture therefore distinguishes:

program identity
logical resource identity
physical resource identity
device identity
execution identity

A device change must not automatically imply program change.

---

135. Resilience and logical qubits

Logical qubits are not equivalent to physical qubits.

One logical qubit may consume multiple physical resources.

Resilience must therefore reason over both:

logical resource requirements
physical resource requirements

without inventing competing qubit identity types.

---

136. Resource hierarchy

The model should support:

logical qubit
    ↓
physical qubit set
    ↓
device
    ↓
backend
    ↓
execution fleet

The exact mapping is supplied by QEC/hardware/routing contracts.

---

137. No assumptions about encoding

Resilience must not assume:

one logical qubit = fixed number of physical qubits

because code distance and architecture may vary.

---

138. No assumptions about topology

Resilience must not assume:

linear
grid
heavy-hex
all-to-all
nearest-neighbor

Topology comes from hardware/routing.

---

139. No assumptions about gate duration

Resilience must not assume:

1 ns
10 ns
100 ns

Durations come from target timing contracts.

---

140. No assumptions about error thresholds

The system must not globally define:

0.99
0.999
0.9999

as universal correctness thresholds.

Thresholds depend on:

- workload;
- error model;
- policy;
- QEC;
- application requirements.

---

141. No assumptions about retry count

There is no universal:

retry three times

rule.

Retry count may be:

zero
one
many
unbounded under explicit budget

depending on policy and resource availability.

---

142. No assumptions about shot count

Shot counts are workload and policy parameters.

Resilience may adjust them only when the statistical contract permits it and the adjustment is recorded.

---

143. No assumptions about backend count

Backend selection must operate over a dynamically discovered set.

---

144. No assumptions about distributed topology

Distributed coordination must operate over arbitrary nodes/links.

---

145. No assumptions about quantum technology

The core model must not assume:

superconducting
ion
neutral atom
photonic
spin
topological
bosonic
annealing

A technology-specific feature becomes available only through a capability contract.

---

146. Compatibility with OpenQASM-style execution models

Zamani's resilience architecture must be capable of supporting:

- runtime classical control;
- timing intent;
- target-dependent durations;
- delays;
- dynamic circuits;
- measurement-conditioned operations;
- hardware-specific lowering.

OpenQASM explicitly separates language/program semantics from the execution environment and allows implementations to support differing runtime capability subsets.

Zamani should preserve this separation while applying it to its stronger canonical IR architecture.

---

147. Compatibility with current error-mitigation practice

The architecture must be able to represent strategies such as:

readout mitigation
twirling
zero-noise extrapolation
probabilistic error cancellation
dynamical decoupling

without making any one strategy mandatory.

Current IBM documentation explicitly describes these as distinct resilience/noise-management mechanisms with configurable overhead and trade-offs.

---

148. Verification over availability

The most important resilience invariant is:

«Availability is never sufficient evidence of correctness.»

The system must not say:

execution completed
→ therefore result is correct

Instead:

execution completed
→ verify
→ determine confidence
→ accept/reject

---

149. Semantic safety invariant

The strongest invariant is:

AcceptedResult
⇒
SemanticallyValid
∧
PolicyValid
∧
CapabilityValid
∧
SecurityValid
∧
Verified

---

150. Recovery invariant

A recovery operation must satisfy:

RecoveryAction
⇒
Authorized
∧
Feasible
∧
SemanticallySafe
∧
ProvenanceTracked
∧
Verifiable

---

151. Deterministic planning invariant

When deterministic mode is enabled:

same inputs
+
same policy
+
same target snapshot
+
same observations
+
same seed
=
equivalent decision

---

152. Scalability invariant

For any valid resource size "N":

N is not encoded into resilience semantics.

Resource-dependent behavior must be discovered/configured.

---

153. Identity invariant

For every canonical quantum qubit:

exactly one QubitId
exactly one PhysicalQubitId where applicable

Resilience must reference the canonical identities.

---

154. Fault invariant

There must be exactly one authoritative physical fault semantic model:

quantum::zqn::fault

Resilience may normalize it but must not replace it.

---

155. Dependency invariant

Core resilience may depend on stable contracts.

Core quantum semantics must not depend on resilience implementations.

---

156. Security invariant

Untrusted observations cannot directly execute privileged recovery actions.

The path must be:

observation
→ validation
→ diagnosis
→ policy
→ authorization
→ plan
→ execution
→ verification

---

157. Production file-completion rule

Before considering any individual resilience source file complete, the implementation must document and test:

Responsibility
Public API
Dependencies
Dependency direction
Error behavior
Ownership
Scalability
Determinism
Security
Serialization
Integration points
Failure modes
Tests

This ensures a later file does not require redesigning the completed contract.

---

158. File integration matrix

File| Consumes| Produces| Must not own
"model/resource.rs"| canonical identities/capabilities| resource model| hardware discovery
"model/fault.rs"| ZQN fault| resilience fault view| fault semantics
"model/incident.rs"| observations| incident| recovery
"model/capability.rs"| hardware capabilities| resilience capability view| hardware model
"detection/*"| telemetry| observations| recovery
"diagnosis/*"| observations/history| diagnosis| execution
"policy/*"| constraints| policy decisions| execution
"planning/*"| diagnosis/policy/capability| plan| execution
"adaptation/*"| plan| adaptation requests/results| routing/scheduling algorithms
"recovery/*"| approved plan| recovery result| quantum semantics
"mitigation/*"| policy/capabilities| mitigation execution| resilience orchestration
"verification/*"| original/adapted execution| verification| recovery
"checkpoint/*"| execution state| checkpoint| arbitrary quantum-state serialization
"telemetry/*"| observations| events/metrics| policy
"learning/*"| verified history| predictions| correctness
"coordination/*"| distributed state| coordination decisions| quantum semantics
"serialization/*"| resilience objects| encoded state| quantum semantics
"registry/*"| plugins| registered capabilities| hidden global mutation
"api/*"| all stable contracts| public resilience operation| provider-specific behavior

---

159. Final architecture

The final Zamani quantum stack is:

┌────────────────────────────────────────────────────────────┐
│                    ZAMANI LANGUAGE                         │
├────────────────────────────────────────────────────────────┤
│                    QUANTUM FRONTEND                        │
├────────────────────────────────────────────────────────────┤
│                  CANONICAL QUANTUM IR                      │
│             quantum::ir::qubit owns IDs                   │
├────────────────────────────────────────────────────────────┤
│ Algorithms │ Optimization │ QEC │ ZQN                     │
├────────────────────────────────────────────────────────────┤
│ Routing │ Scheduling │ Resource Estimation                 │
├────────────────────────────────────────────────────────────┤
│                    RESILIENCE                              │
│                                                            │
│ Detect → Diagnose → Policy → Plan → Adapt                 │
│                    ↓                                       │
│ Recover → Mitigate → Verify → Learn                        │
├────────────────────────────────────────────────────────────┤
│                    HARDWARE HAL                            │
├────────────────────────────────────────────────────────────┤
│ Runtime │ Simulator │ Emulator │ Distributed Execution     │
├────────────────────────────────────────────────────────────┤
│             Quantum Execution Targets                      │
└────────────────────────────────────────────────────────────┘

The critical invariant is:

«The Zamani source program remains stable while routing, scheduling, optimization, QEC configuration, mitigation, hardware, topology, calibration, backend, and recovery strategy may change underneath it.»

---

160. Definition of production-ready resilience

"src/quantum/resilience/" is production-ready only when it satisfies all of the following simultaneously:

Canonical quantum semantics
        +
Canonical QubitId / PhysicalQubitId
        +
Canonical ZQN fault model
        +
Provider-neutral hardware integration
        +
Routing integration
        +
Scheduling integration
        +
Optimization integration
        +
QEC integration
        +
Runtime integration
        +
Dynamic execution support
        +
Fault detection
        +
Diagnosis
        +
Policy
        +
Planning
        +
Adaptation
        +
Recovery
        +
Mitigation
        +
Verification
        +
Checkpointing
        +
Provenance
        +
Observability
        +
Determinism
        +
Security
        +
Distributed coordination
        +
Scalability
        +
Fault injection
        +
Replay
        +
End-to-end verification

The implementation must never trade semantic correctness for apparent availability.

The final design principle is therefore:

«Zamani resilience does not merely make failed quantum programs run again. It continuously determines whether the computation can still be executed correctly, how it can be adapted when the physical world changes, and whether the resulting answer deserves to be trusted.»

---

External architectural references

OpenQASM 3 provides the relevant model for runtime classical control, explicit timing, target-dependent execution capabilities, and separation of language semantics from the execution environment.

IBM Quantum's current documentation provides practical reference points for error mitigation/suppression, including readout mitigation, twirling, zero-noise extrapolation, probabilistic techniques, and dynamical decoupling.

IBM also documents resilience as a configurable trade-off between result quality and execution/sampling overhead, reinforcing the need for policy-driven mitigation rather than hard-coded resilience behavior.This version is intentionally normative rather than aspirational: it makes quantum::ir::qubit and quantum::zqn::fault authoritative, keeps routing/scheduling/optimization/hardware/QEC as separate owners, and makes resilience the orchestration + verification layer. That matches the repository's existing architectural direction.