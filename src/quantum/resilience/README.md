Yes. I checked the current repository contract before writing this. In particular, quantum::ir explicitly establishes quantum::ir::qubit::QubitId and PhysicalQubitId as the canonical identities, forbids duplicate qubit types, and enforces #![forbid(unsafe_code)].

I also checked the existing src/quantum/resilience/errors/error.rs; it already contains a substantial production-oriented error contract.  The README therefore needs to describe that actual contract, rather than documenting a hypothetical resilience module.

Below is the complete src/quantum/resilience/README.md. It is written as a normative integration document: it explains what the subsystem owns, what every child directory owns, dependency direction, scalability rules, error semantics, QubitId rules, implementation order, testing, security, and the integration contract with the rest of Zamani.

Zamani Quantum Resilience

Production Architecture, Integration Contract, Scalability and Operational Guide

"quantum::resilience" is the production resilience and autonomous-recovery subsystem of the Zamani quantum computing architecture.

Its purpose is to ensure that a Zamani quantum program can remain semantically correct and executable while the underlying execution environment changes because of:

- quantum faults;
- noise;
- leakage;
- loss;
- erasure;
- correlated faults;
- QEC degradation;
- calibration drift;
- hardware degradation;
- resource loss;
- routing failure;
- scheduling failure;
- compiler failure;
- backend failure;
- communication failure;
- execution timeout;
- transient infrastructure failure;
- changing hardware capabilities;
- changing topology;
- heterogeneous quantum resources;
- distributed execution failures;
- mitigation failures;
- checkpoint/recovery failures.

The fundamental design objective is:

«Write the quantum program once. Allow the implementation to adapt to any compatible quantum machine and to changing machine conditions without introducing artificial machine-size limits or changing the program's intended semantics.»

The architectural meaning of "infinite scale" is:

«Zamani introduces no artificial finite machine-size ceiling. Every concrete execution remains bounded by the physical, computational, memory, time, policy, security and resource limits actually available to that execution.»

---

1. Scope

"quantum::resilience" is the orchestration and decision layer between the quantum execution stack and the mechanisms that observe, adapt, recover and verify execution.

Conceptually:

                         Zamani Program
                               │
                               ▼
                       Quantum Frontend
                               │
                               ▼
                      Canonical Quantum IR
                               │
               ┌───────────────┼────────────────┐
               │               │                │
               ▼               ▼                ▼
          Optimization       QEC             Analysis
               │               │                │
               └───────────────┼────────────────┘
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
                           Execution
                               │
                 ┌─────────────┴─────────────┐
                 │                           │
                 ▼                           ▼
             Telemetry                    Results
                 │                           │
                 └─────────────┬─────────────┘
                               ▼
                         RESILIENCE
                               │
          ┌────────────────────┼────────────────────┐
          ▼                    ▼                    ▼
       Detect               Diagnose              Verify
          │                    │                    │
          └────────────────────┼────────────────────┘
                               ▼
                             Policy
                               │
                               ▼
                             Plan
                               │
                               ▼
                            Adapt
                               │
              ┌────────────────┼────────────────┐
              ▼                ▼                ▼
           Reroute         Reschedule       Recompile
              │                │                │
              └────────────────┼────────────────┘
                               ▼
                            Recover
                               │
                               ▼
                           Mitigate
                               │
                               ▼
                           Verify
                               │
                    ┌──────────┴──────────┐
                    ▼                     ▼
                  ACCEPT                REPEAT

Resilience therefore does not replace the other quantum subsystems.

It coordinates them.

---

2. Core architectural principle

The resilience subsystem must preserve the following separation:

ZQN
    describes quantum faults/noise semantics

QEC
    detects and corrects quantum errors

IR
    describes computational semantics

Optimization
    transforms the implementation while preserving semantics

Routing
    maps logical requirements to physical resources

Scheduling
    determines execution ordering and timing

Hardware HAL
    describes and executes against target capabilities

Simulation
    provides simulated execution environments

Benchmarking
    measures execution/resource behavior

Resilience
    decides when and how these capabilities should be
    used to preserve a valid execution

No subsystem should silently absorb another subsystem's responsibilities.

---

3. What resilience is not

"quantum::resilience" must not become a second implementation of:

- quantum IR;
- quantum gates;
- qubit identity;
- QEC;
- decoders;
- noise models;
- fault ontologies;
- routing algorithms;
- scheduling algorithms;
- optimization passes;
- hardware drivers;
- provider SDKs;
- simulation engines;
- benchmark engines;
- compiler frontends.

Instead, resilience consumes stable contracts exposed by those subsystems.

This prevents architectural duplication and keeps the system extensible.

---

4. Canonical quantum identity

The authoritative qubit identity implementation is:

crate::quantum::ir::qubit

The canonical types are:

crate::quantum::ir::qubit::QubitId
crate::quantum::ir::qubit::PhysicalQubitId

Resilience must never introduce another:

QubitId
PhysicalQubitId
LogicalQubitId

implementation that duplicates the canonical IR identity model.

The canonical IR explicitly establishes "quantum::ir::qubit" as the authoritative implementation and retains root-level exports only as compatibility aliases.

When a resilience error needs to identify a qubit, it must use the canonical identity types.

For example:

use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};

A logical identity and a physical identity must remain distinguishable.

This prevents errors such as:

logical qubit 7

being accidentally interpreted as:

physical hardware qubit 7

---

5. Rust contract

The resilience subsystem targets:

- Rust 1.97;
- Rust 1.97.1;
- Rust 2021 edition;
- stable Rust;
- no nightly-only features;
- no "unsafe".

The subsystem must enforce:

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

Where appropriate, individual modules should additionally enforce strict compiler diagnostics.

The existing resilience error implementation already follows this safety model.

---

6. Scalability contract

The architecture must scale without source-code changes from:

one qubit
        ↓
small quantum device
        ↓
large QPU
        ↓
fault-tolerant logical machine
        ↓
multiple QPUs
        ↓
heterogeneous quantum fleet
        ↓
distributed quantum infrastructure

No resilience source file may contain architectural assumptions such as:

const MAX_QUBITS: usize = 127;

or:

const MAX_QUBITS: usize = 1000;

or:

if qubit_id == 127

or:

for _ in 0..3 {
    retry();
}

or:

if fidelity < 0.95 {
    ...
}

unless such values are explicitly supplied by:

- target capabilities;
- execution configuration;
- policy;
- security policy;
- resource availability;
- user constraints;
- discovered runtime state.

A constant is not automatically bad.

A hard-coded architectural assumption is bad.

For example:

const ERROR_CODE: u16 = 71;

is acceptable when it is part of a stable error protocol.

By contrast:

const MAX_SUPPORTED_QUBITS: usize = 10000;

would violate the scalability contract.

---

7. Four-dimensional scalability

Resilience must scale along four independent dimensions.

7.1 Computational scale

single operation
    ↓
single circuit
    ↓
large program
    ↓
large workload
    ↓
distributed workload

7.2 Physical scale

single qubit
    ↓
small QPU
    ↓
large QPU
    ↓
multiple QPUs
    ↓
distributed quantum infrastructure

7.3 Logical scale

physical qubits
    ↓
encoded qubits
    ↓
logical qubits
    ↓
fault-tolerant computation

7.4 Organizational scale

one execution backend
    ↓
multiple devices
    ↓
multiple backend implementations
    ↓
heterogeneous quantum fleet

No one of these dimensions should be treated as the only meaning of scale.

---

8. The resilience lifecycle

The canonical lifecycle is:

EXECUTION
    ↓
DETECT
    ↓
DIAGNOSE
    ↓
POLICY
    ↓
PLAN
    ↓
ADAPT
    ↓
RECOVER
    ↓
MITIGATE
    ↓
VERIFY
    ↓
ACCEPT

If verification fails:

VERIFY
   │
   ├── ACCEPT
   │
   ├── DEGRADED_ACCEPT
   │
   ├── RETRY
   │
   ├── REPLAN
   │
   ├── ESCALATE
   │
   └── REJECT

The lifecycle must not contain an unconditional:

failure → retry

Recovery is a policy-driven decision.

---

9. The safety invariant

The central safety invariant is:

«No recovery action may be accepted solely because it increases availability.»

An action must satisfy the required combination of:

semantic validity
+
policy validity
+
capability validity
+
security validity
+
verification validity

Only then can the result be accepted.

This prevents a system from "healing" itself into an incorrect computation.

---

10. Directory architecture

The complete resilience subsystem is organized as:

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

Only modules that physically exist should be declared by "mod.rs".

The repository's canonical IR explicitly follows this principle: parent modules should not declare speculative modules that do not exist.

---

11. "README.md"

This document.

It defines:

- purpose;
- scope;
- architecture;
- integration contracts;
- scalability;
- safety;
- implementation order;
- operational expectations.

It is normative documentation, not executable code.

---

12. "mod.rs"

"mod.rs" owns only:

- module declarations;
- stable public re-exports;
- public API composition.

It must not contain:

- recovery algorithms;
- detection logic;
- provider logic;
- hardware logic;
- business logic.

The root module must remain thin.

---

13. "api/"

"api/controller.rs"

Owns the primary resilience orchestration interface.

It coordinates:

execution
→ detection
→ diagnosis
→ policy
→ planning
→ adaptation
→ recovery
→ verification

It should depend on contracts rather than concrete backend implementations.

---

"api/request.rs"

Defines an immutable resilience request.

It should represent:

- program/execution identity;
- requested resilience guarantees;
- policy;
- resource requirements;
- target constraints;
- verification requirements;
- deterministic execution requirements.

It must not embed a particular vendor SDK.

---

"api/response.rs"

Defines the immutable resilience result.

It should contain or reference:

- execution outcome;
- final result;
- verification outcome;
- recovery history;
- provenance;
- degradation status;
- confidence;
- final target information.

---

"api/context.rs"

Defines the execution context.

It provides access to the required subsystem contracts without making resilience own them.

Expected integrations include:

quantum::ir
quantum::zqn
quantum::qec
quantum::routing
quantum::scheduling
quantum::optimization
quantum::hardware
quantum::simulation
quantum::benchmarking
runtime/execution

---

14. "model/"

The model layer defines resilience-domain vocabulary.

It should be stable and implementation-independent.

---

"model/fault.rs"

Defines a resilience-level fault representation.

It must consume the canonical fault semantics supplied by ZQN.

It must not recreate:

- leakage;
- erasure;
- loss;
- correlated fault;
- noise-channel semantics.

Those belong to ZQN.

Resilience converts fault observations into operational incidents and decisions.

---

"model/incident.rs"

Groups related faults into an operational incident.

Example:

physical qubit A failure
physical qubit B degradation
coupling degradation
readout degradation

may represent one correlated hardware incident rather than four independent recovery operations.

This becomes increasingly important as systems scale.

---

"model/severity.rs"

Defines provider-independent severity.

Example:

Informational
Degraded
Major
Critical
Fatal

Severity describes impact.

It does not itself determine the recovery action.

---

"model/health.rs"

Defines health states for arbitrary resources:

Unknown
Healthy
Degraded
Unstable
Unavailable
Recovering
Quarantined
Retired

Resources may include:

- backend;
- device;
- QPU;
- logical qubit;
- physical qubit;
- coupling;
- gate capability;
- control channel;
- memory;
- execution resource.

No fixed resource count is permitted.

---

"model/degradation.rs"

Represents partial capability loss.

Example:

available resources:
100%
 ↓
96%
 ↓
82%
 ↓
70%

The resilience system should continue execution when policy and semantic constraints remain satisfiable.

---

"model/capability.rs"

Represents capabilities consumed from the hardware HAL.

Examples:

- qubit capacity;
- gate support;
- connectivity;
- timing;
- measurement;
- reset;
- dynamic control;
- QEC support;
- logical-qubit support;
- mitigation support.

This model must not duplicate the hardware capability implementation.

---

"model/resource.rs"

Defines generic resource identity.

Examples:

Backend
Device
QPU
LogicalQubit
PhysicalQubit
Coupling
ControlChannel
ExecutionSlot
Memory
NetworkPath

Resource identity must remain extensible.

---

"model/confidence.rs"

Represents confidence associated with:

- observations;
- diagnoses;
- predictions;
- recovery decisions;
- verification.

The system must distinguish:

known
probable
uncertain
unknown

rather than treating every inference as fact.

---

15. "detection/"

Detection converts observations into normalized resilience signals.

---

"detection/detector.rs"

Defines the detector interface.

A detector consumes observations and produces normalized events.

Multiple detectors must be composable.

---

"detection/anomaly.rs"

Provides generic anomaly detection.

Possible approaches:

- rules;
- statistics;
- historical baselines;
- learned models.

Machine learning must not be mandatory for correctness.

---

"detection/threshold.rs"

Provides policy/configuration-driven thresholds.

Bad:

if fidelity < 0.95

Good:

threshold = policy.minimum_acceptable_fidelity()

The threshold belongs to policy/configuration, not the detector implementation.

---

"detection/statistical.rs"

Supports:

- moving statistics;
- variance;
- confidence intervals;
- distribution changes;
- outlier detection;
- sequential detection.

---

"detection/drift.rs"

Detects changes in:

- calibration;
- noise;
- timing;
- readout;
- gate behavior;
- topology;
- resource availability.

Quantum hardware changes over time, so monitoring and recalibration are fundamental operational concerns.

---

"detection/timeout.rs"

Normalizes:

- compilation timeout;
- queue timeout;
- execution timeout;
- measurement timeout;
- backend timeout;
- distributed coordination timeout.

---

"detection/execution_failure.rs"

Normalizes execution failures from the hardware/runtime boundary.

---

"detection/qec_signal.rs"

Consumes QEC observations such as:

- syndrome information;
- decoder confidence;
- logical error indicators;
- leakage;
- erasure;
- code-health signals.

It does not implement QEC decoding.

---

"detection/hardware_signal.rs"

Consumes hardware HAL observations:

- status;
- health;
- calibration;
- telemetry;
- capability changes;
- topology changes.

---

16. "diagnosis/"

Diagnosis answers:

«What is probably wrong?»

It must preserve uncertainty.

---

"diagnosis/diagnostician.rs"

Coordinates diagnosis.

Inputs:

observations
+
execution context
+
hardware state
+
QEC state
+
history

Output:

diagnosis

---

"diagnosis/classifier.rs"

Classifies failures such as:

- quantum fault;
- hardware failure;
- backend failure;
- routing failure;
- scheduling failure;
- compiler failure;
- resource exhaustion;
- QEC failure;
- timeout;
- unknown.

---

"diagnosis/root_cause.rs"

Represents root-cause hypotheses.

It must never claim certainty where the evidence does not justify certainty.

---

"diagnosis/correlation.rs"

Correlates related observations.

This is required for large-scale systems where thousands or millions of observations may arise from one underlying incident.

---

"diagnosis/localization.rs"

Locates problems at the appropriate abstraction level:

backend
device
QPU
region
logical qubit
physical qubit
coupling
gate
execution stage

Use canonical "QubitId" and "PhysicalQubitId" where applicable.

---

"diagnosis/confidence.rs"

Calculates diagnosis confidence.

---

17. "policy/"

Policy defines what the system is allowed and expected to do.

Policy does not execute actions.

---

"policy/policy.rs"

Central policy abstraction.

It answers:

- what is permitted?
- what is preferred?
- what is forbidden?
- when should recovery happen?
- when must execution stop?

---

"policy/constraints.rs"

Defines semantic and operational constraints.

Examples:

- preserve logical semantics;
- maximum tolerated logical error;
- maximum execution duration;
- allowed migration;
- allowed recompilation;
- resource constraints;
- verification requirements.

---

"policy/objectives.rs"

Defines optimization objectives.

Possible objectives:

correctness
fidelity
latency
availability
cost
energy
resource usage
logical error probability

Objectives must be composable.

---

"policy/budgets.rs"

Defines configurable budgets for:

- retries;
- execution time;
- shots;
- qubits/resources;
- compilation;
- mitigation;
- recovery;
- migration.

There must be no universal hard-coded retry count.

---

"policy/escalation.rs"

Defines when autonomous recovery must stop and escalate.

---

"policy/retry.rs"

Defines when retry is semantically valid.

A retry is not automatically safe.

For example:

transient backend failure

may permit retry.

But:

unknown semantic corruption

may require verification or termination instead.

---

"policy/safety.rs"

The strongest policy boundary.

It must reject actions that could:

- violate semantics;
- exceed authority;
- exceed resource policy;
- destroy provenance;
- bypass verification;
- hide failures;
- use untrusted resources;
- violate security requirements.

---

18. "planning/"

Planning determines what should happen next.

---

"planning/action.rs"

Defines abstract actions:

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

These are descriptions, not implementations.

---

"planning/plan.rs"

Defines an immutable recovery plan containing:

- incident;
- diagnosis;
- preconditions;
- actions;
- expected effects;
- risk;
- cost;
- confidence;
- rollback strategy;
- verification requirements.

---

"planning/cost.rs"

Defines provider-neutral cost dimensions.

Possible dimensions:

- time;
- shots;
- resources;
- energy;
- logical error probability;
- financial cost;
- compilation effort.

---

"planning/feasibility.rs"

Determines whether a proposed plan can actually execute with current capabilities.

---

"planning/ranking.rs"

Ranks feasible plans according to policy objectives.

---

"planning/planner_state.rs"

Stores state required for deterministic planning and replay.

---

"planning/planner.rs"

Combines:

policy
+
diagnosis
+
capabilities
+
state
+
history
+
cost
+
constraints

to produce a recovery plan.

---

19. "adaptation/"

Adaptation modifies the implementation while preserving program semantics.

This is one of the most important components for the "write once, scale everywhere" objective.

---

"adaptation/remapping.rs"

Recomputes logical-to-physical mappings.

It must never assume:

logical q0 = physical q0

or:

logical q7 = physical q7

Mappings must come from routing/resource capabilities.

---

"adaptation/rerouting.rs"

Requests new routing when:

- physical resources fail;
- topology changes;
- couplings degrade;
- capabilities change.

The routing subsystem owns the routing algorithm.

---

"adaptation/rescheduling.rs"

Requests schedule reconstruction after:

- timing changes;
- routing changes;
- resource failures;
- calibration changes.

The scheduling subsystem owns scheduling algorithms.

---

"adaptation/recompilation.rs"

Requests recompilation against a changed target.

The canonical IR remains the semantic source.

---

"adaptation/reoptimization.rs"

Requests appropriate optimization passes after target changes.

Resilience does not duplicate optimization passes.

---

"adaptation/qec_adaptation.rs"

Requests alternative QEC configurations when allowed.

Potential variables include:

- code configuration;
- logical layout;
- code distance;
- decoder;
- ancilla allocation;
- syndrome strategy.

QEC remains the owner of QEC implementation.

---

"adaptation/backend_selection.rs"

Selects a compatible target based on:

- capabilities;
- policy;
- resource availability;
- security;
- reliability;
- compatibility.

No vendor-specific branching belongs in the core.

---

"adaptation/adapter.rs"

Defines the generic adaptation boundary.

---

20. "recovery/"

Recovery performs approved recovery actions.

---

"recovery/recoverer.rs"

Coordinates recovery execution.

Conceptually:

Incident
   ↓
RecoveryPlan
   ↓
Precondition validation
   ↓
Action execution
   ↓
Verification

---

"recovery/retry.rs"

Executes policy-approved retries.

Retry must not be represented as a fixed loop.

---

"recovery/restart.rs"

Restarts from an explicitly safe boundary.

---

"recovery/checkpoint.rs"

Coordinates checkpoint-aware recovery.

Checkpoint implementation itself belongs to "checkpoint/".

---

"recovery/rollback.rs"

Restores a valid prior state where rollback semantics exist.

---

"recovery/resume.rs"

Resumes from a valid checkpoint or execution boundary.

---

"recovery/migration.rs"

Moves execution to another compatible resource when semantics permit.

Potential targets include:

- another QPU;
- another device;
- another backend;
- simulator;
- emulator;
- logical resource pool.

---

"recovery/compensation.rs"

Defines mathematically valid compensating actions where rollback is impossible.

Quantum state must never be treated like arbitrary classical mutable state.

---

21. "mitigation/"

Mitigation is deliberately separate from QEC.

Mitigation may include:

- readout mitigation;
- zero-noise extrapolation;
- probabilistic error cancellation;
- twirling;
- dynamical decoupling;
- custom techniques.

The strategy must be selected according to workload, hardware capability, policy and cost.

---

"mitigation/strategy.rs"

Defines the generic mitigation contract.

---

"mitigation/executor.rs"

Executes approved mitigation strategies.

---

"mitigation/selection.rs"

Selects a mitigation strategy.

---

"mitigation/readout.rs"

Readout/measurement mitigation.

---

"mitigation/zero_noise.rs"

Zero-noise extrapolation abstraction.

Noise factors and extrapolation policies must remain configurable.

---

"mitigation/probabilistic.rs"

Probabilistic error cancellation and related approaches.

---

"mitigation/twirling.rs"

Gate twirling/randomized compiling abstractions.

---

"mitigation/dynamical_decoupling.rs"

Dynamical decoupling.

It should integrate with scheduling/pulse capabilities rather than directly manipulate a hardware implementation.

---

"mitigation/custom.rs"

Extension point for future mitigation strategies.

---

22. "verification/"

Verification is the final defense against incorrect autonomous recovery.

---

"verification/verifier.rs"

Coordinates verification.

---

"verification/invariant.rs"

Checks invariants such as:

- logical qubit identity;
- required operations;
- measurement semantics;
- resource constraints;
- required program properties.

---

"verification/semantic.rs"

Checks that adaptation preserved canonical program semantics.

The canonical IR remains the semantic authority.

---

"verification/result.rs"

Validates returned results.

---

"verification/confidence.rs"

Calculates confidence in the result.

---

"verification/provenance.rs"

Records:

original program
IR identity/hash
compiler version
optimization
routing
schedule
target
hardware state
calibration state
fault observations
diagnosis
adaptation
recovery
mitigation
QEC state
result
verification

This creates an auditable execution chain.

---

"verification/acceptance.rs"

Defines final acceptance states:

ACCEPT
DEGRADED_ACCEPT
RETRY
REPLAN
ESCALATE
REJECT

A recovered result must not become accepted merely because execution completed.

---

23. "state/"

State management separates machine state from execution state.

---

"state/machine.rs"

Current machine state.

---

"state/execution.rs"

Current execution state.

---

"state/logical.rs"

Logical resource state.

---

"state/physical.rs"

Physical resource state.

This is where canonical physical qubit identities must be used.

---

"state/recovery.rs"

Recovery state machine:

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

---

"state/persistence.rs"

Persistence contract for resilience state.

---

24. "checkpoint/"

Quantum checkpointing requires special semantic treatment.

Do not assume an arbitrary unknown quantum state can simply be serialized and restored.

Checkpoint categories should distinguish:

classical execution state
compiled program state
logical checkpoint
measurement boundary
QEC state
reconstructible state
provider-supported state

---

"checkpoint/checkpoint.rs"

Main checkpoint abstraction.

---

"checkpoint/snapshot.rs"

Snapshot metadata.

---

"checkpoint/manifest.rs"

Content manifest.

---

"checkpoint/storage.rs"

Storage abstraction.

Storage must remain independent from any particular cloud provider.

---

"checkpoint/integrity.rs"

Integrity verification.

---

"checkpoint/compatibility.rs"

Determines whether a checkpoint can be restored under a new target/configuration.

---

25. "telemetry/"

Telemetry provides observation without coupling resilience to one observability technology.

---

"telemetry/event.rs"

Canonical resilience event model.

---

"telemetry/metric.rs"

Metrics may include:

- fidelity;
- logical error rate;
- physical error rate;
- readout error;
- gate error;
- latency;
- queue time;
- retry rate;
- recovery rate;
- failure rate;
- degradation;
- mitigation overhead.

---

"telemetry/trace.rs"

End-to-end execution trace.

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
- scheduler;
- routing.

---

"telemetry/exporter.rs"

Exports telemetry without making the resilience core dependent on one monitoring platform.

---

26. "history/"

Historical information improves diagnosis and planning.

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

Aggregated statistics.

History must remain bounded by explicit resource policies when necessary; it must not imply an architectural infinite-memory requirement.

---

27. "learning/"

Learning is optional.

Correctness must never depend on a machine-learning model being available.

---

"learning/model.rs"

Generic prediction-model interface.

---

"learning/features.rs"

Feature extraction.

---

"learning/predictor.rs"

Possible predictions:

- failure probability;
- recovery success probability;
- expected fidelity;
- expected cost;
- expected latency.

---

"learning/strategy.rs"

Learns strategy-selection preferences.

---

"learning/feedback.rs"

Feeds verified outcomes back into future decisions.

Critical rule:

«Learned predictions may influence ranking, but may never override safety, semantic validation or verification.»

---

28. "coordination/"

Required when resilience operates across distributed resources.

---

"coordination/coordinator.rs"

Coordinates distributed resilience operations.

---

"coordination/distributed.rs"

Distributed execution/recovery abstraction.

---

"coordination/lease.rs"

Resource leases.

---

"coordination/ownership.rs"

Recovery ownership.

---

"coordination/consensus.rs"

Consensus abstraction where required.

Do not implement a custom consensus protocol merely because distributed execution exists.

---

29. "serialization/"

Every persisted or transported resilience object needs deterministic schema handling.

---

"serialization/schema.rs"

Defines serialization schemas.

---

"serialization/encode.rs"

Encoding.

---

"serialization/decode.rs"

Decoding.

---

"serialization/version.rs"

Schema compatibility/versioning.

Serialization must not become a second semantic representation.

---

30. "errors/"

The error subsystem is intentionally dependency-light.

The existing "errors/error.rs" already defines a production-oriented error contract including:

- stable machine-readable error codes;
- categories;
- severity;
- retryability;
- recovery eligibility;
- structured diagnostics;
- logical and physical qubit identification;
- resource identification;
- operation identification;
- underlying error preservation;
- deterministic formatting;
- no hardware-size assumptions;
- no provider-specific behavior.

This makes the error layer appropriate as one of the first independently completed resilience components.

---

"errors/error.rs"

Owns the principal:

ResilienceError

contract.

Every fallible public resilience operation should ultimately be representable as:

Result<T, ResilienceError>

or preserve the equivalent structured error without discarding information.

Machine consumers must use structured fields such as:

code
category
severity
retryability
recoverability

and must not parse display text.

Display text is for humans.

The existing implementation also deliberately prevents underlying source errors from automatically being rendered into public display text, reducing accidental leakage of provider/backend internals.

---

Error security

Error messages and context must never contain:

- credentials;
- API keys;
- access tokens;
- private keys;
- passwords;
- authorization headers;
- session secrets;
- raw pointers;
- memory addresses;
- private authentication material;
- unredacted sensitive program information.

Errors may cross:

module
process
machine
network
telemetry

boundaries.

Therefore error serialization must be treated as a security boundary.

---

Error code stability

Error codes are protocol identifiers.

Once released, an error-code identity must not silently be reused for another meaning.

Human-readable text may evolve.

Machine-readable code semantics must remain compatible according to the project's compatibility policy.

---

"errors/classification.rs"

Defines cross-cutting classifications such as:

Transient
Persistent
Recoverable
NonRecoverable
Unknown
SafetyCritical
SemanticRisk

These classifications must not be confused with severity.

For example:

Critical + Recoverable

is valid.

Likewise:

Warning + NonRecoverable

may be valid.

---

31. "limits/"

Limits are execution-policy constraints, not machine-size architecture.

---

"limits/limits.rs"

Defines configured limits.

Examples:

- execution budget;
- recovery budget;
- telemetry retention;
- memory budget;
- planning budget.

It must not define a universal maximum qubit count.

---

"limits/resource.rs"

Represents resource constraints supplied by the target/runtime.

---

"limits/validation.rs"

Validates requested operations against:

policy
+
available resources
+
target capabilities
+
security policy

---

32. "registry/"

Registries provide extension points.

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

Backend integration registration.

Registries must not weaken safety.

An extension cannot bypass:

authorization
+
policy
+
semantic validation
+
verification

---

33. Dependency direction

The fundamental dependency rule is:

                    quantum::ir
                         ▲
                         │
                         │ consumes
                         │
ZQN ───────────────► RESILIENCE ◄────────────── Hardware HAL
                         │
              ┌──────────┼──────────┐
              │          │          │
              ▼          ▼          ▼
           Routing   Scheduling   Optimization
              │          │          │
              └──────────┼──────────┘
                         ▼
                       QEC
                         │
                         ▼
                      Runtime

More precisely:

resilience → consumes contracts
other subsystems → do not depend on concrete resilience implementations

This prevents circular dependencies.

---

34. Integration with canonical IR

The canonical IR is the semantic source of truth.

The repository explicitly defines "quantum::ir" as the stable semantic boundary between the Zamani quantum frontend and downstream compilation/execution systems. It describes what a computation means rather than choosing the physical machine, routing, scheduling, hardware or backend.

Resilience therefore must:

- consume canonical IR;
- preserve semantic identity;
- never define another gate model;
- never define another qubit identity model;
- never mutate semantic meaning merely to recover availability.

When an implementation changes, the canonical program meaning must remain stable.

---

35. Integration with ZQN

ZQN is the canonical source for quantum fault/noise semantics.

Resilience consumes ZQN information such as:

fault classification
fault location
correlated faults
leakage
loss
erasure
noise observations

Resilience then transforms those observations into:

incident
diagnosis
policy decision
recovery plan
verification requirement

Resilience must not create a competing fault ontology.

---

36. Integration with QEC

QEC owns:

- encoding;
- syndrome extraction;
- decoding;
- correction;
- code-specific logic;
- logical error handling.

Resilience owns decisions such as:

Should QEC configuration change?
Should the execution migrate?
Should code distance change?
Should the computation stop?
Should another logical resource be selected?

This distinction must remain explicit.

---

37. Integration with hardware

The hardware HAL owns:

- identity;
- capabilities;
- topology;
- timing;
- calibration;
- execution;
- status;
- health;
- telemetry;
- provider adapters.

Resilience consumes those capabilities.

Resilience must not contain:

if provider == ...

or:

if hardware_model == ...

inside its core orchestration.

Provider-specific behavior belongs inside hardware/backend adapters.

---

38. Integration with routing

Resilience detects when routing becomes invalid and requests rerouting.

It must not implement routing algorithms.

For example:

resource failure
      ↓
resilience
      ↓
routing request
      ↓
new mapping
      ↓
semantic verification

---

39. Integration with scheduling

When timing or resource availability changes:

resilience
      ↓
scheduler
      ↓
new schedule
      ↓
verification

Resilience should not reproduce scheduling algorithms.

---

40. Integration with optimization

When the target changes:

canonical IR
      ↓
new target capabilities
      ↓
optimization
      ↓
new implementation

Optimization remains responsible for transformation.

Resilience decides when optimization should be invoked.

---

41. Integration with benchmarking

Benchmarking supplies evidence for:

- reliability;
- historical failure probability;
- fidelity;
- calibration stability;
- execution latency;
- resource behavior.

Resilience can use those measurements for planning and strategy ranking.

Benchmarking does not become a recovery engine.

---

42. Integration with simulation

Simulation must support resilience testing without physical hardware.

A simulation scenario should be able to represent:

program
+
target model
+
fault model
+
telemetry
+
policy

and execute:

detect
→ diagnose
→ plan
→ adapt
→ recover
→ verify

This allows large-scale testing before deployment.

---

43. Integration with the Zamani language

Ordinary Zamani programs should not need to know the physical hardware.

Conceptually:

quantum program {
    ...
}

describes the computation.

Advanced users may specify semantic resilience policies such as:

resilience {
    correctness = strict
    migration = allowed
    mitigation = adaptive
}

These are policies.

They must not become hardware instructions.

A user should not need to write:

use_backend(...)
use_qubit_7(...)
retry_three_times(...)
route_q0_to_q13(...)

to obtain portable execution.

---

44. Write-once semantics

The intended programming model is:

program
    ↓
canonical semantic representation
    ↓
target adaptation
    ↓
execution

not:

program
    ↓
provider-specific instructions

The program should express:

what computation is required

rather than:

which physical qubit must perform it

This aligns with the canonical IR architecture, which explicitly separates computation semantics from physical target selection, routing, scheduling and hardware execution.

---

45. Graceful degradation

A production resilience engine must support partial degradation.

Example:

required resources:     R
available resources:    A

If:

A satisfies R

execution may continue.

If:

A does not satisfy R

the planner may consider:

rerouting
rescheduling
recompilation
QEC adaptation
mitigation
migration

If no valid plan exists:

ESCALATE / REJECT

The system must never silently violate the program's requirements.

---

46. Recovery is not always possible

Resilience must explicitly represent:

recoverable
non-recoverable
unknown

Examples of potentially recoverable failures:

transient backend outage
temporary resource loss
stale calibration
routing invalidation
temporary execution timeout

Examples that may require rejection:

semantic corruption
unverified result
irrecoverable logical error
tampered checkpoint
unauthorized resource
insufficient capabilities

The actual decision must remain policy-driven.

---

47. Checkpoint semantics

A checkpoint must never claim to restore quantum state when the underlying execution model cannot actually restore it.

Valid checkpoint boundaries must be explicitly represented.

For example:

measurement boundary
classical state boundary
provider-supported quantum state
reconstructible logical state

must remain distinguishable.

---

48. Determinism

Deterministic operation must be supported where requested.

Given identical:

program
IR
hardware snapshot
telemetry
policy
configuration
random seed

a deterministic planner should produce the same decision.

Non-deterministic algorithms must have explicitly controlled sources of randomness.

Randomness must never be used as an excuse for irreproducible recovery.

---

49. Provenance

Every adaptive execution should be traceable.

At minimum:

program identity
IR identity
target identity
capability snapshot
calibration snapshot
fault observations
diagnosis
policy
plan
adaptations
recovery actions
mitigation
QEC configuration
execution result
verification result

A production recovery without provenance is not sufficiently auditable.

---

50. Security model

Resilience is a security-sensitive subsystem because it can change where and how computation executes.

Threats include:

forged telemetry
malicious backend
compromised device
tampered checkpoint
malicious plugin
false health report
false failure report
resource hijacking
unauthorized migration
result tampering

Security controls must include:

- authenticated observations;
- integrity protection;
- authorization;
- provenance;
- trust levels;
- checkpoint integrity;
- plugin restrictions;
- recovery authorization.

An untrusted observation must not automatically trigger a privileged recovery action.

---

51. Observability

Every important resilience transition should produce structured events.

For example:

execution.started
fault.detected
incident.created
diagnosis.completed
policy.evaluated
plan.created
adaptation.started
recovery.started
recovery.completed
verification.started
verification.completed
result.accepted
result.rejected
execution.escalated

Events should contain stable identifiers rather than relying exclusively on human-readable strings.

---

52. Large-scale operation

At large scale, resilience must avoid algorithms that require:

all historical events in memory

or:

all qubits represented in a single fixed array

or:

global synchronization for every local fault

where unnecessary.

Prefer:

- streaming telemetry;
- incremental aggregation;
- sparse resource representations;
- partitioned state;
- hierarchical diagnosis;
- regional recovery;
- bounded retention;
- lazy evaluation;
- deterministic identifiers;
- incremental verification.

A large quantum machine should not require the same recovery architecture as a one-qubit device merely because both use the same API.

---

53. Locality of recovery

When possible:

local fault
    ↓
local diagnosis
    ↓
local adaptation
    ↓
local verification

should be preferred over:

one qubit fails
    ↓
recompile entire universe

However, local recovery must never violate global semantic constraints.

The planner decides the required recovery scope.

---

54. Hierarchical resilience

Large systems should support:

operation-level
    ↓
qubit-level
    ↓
region-level
    ↓
device-level
    ↓
backend-level
    ↓
fleet-level
    ↓
distributed-system-level

A failure should be handled at the smallest safe scope.

If local recovery is impossible, escalation proceeds upward.

---

55. Extension model

Future technologies must be addable without modifying the resilience core.

Examples:

new detector
new decoder
new mitigation
new recovery strategy
new hardware architecture
new backend
new QEC code
new prediction model

The registry and trait boundaries exist for this purpose.

Extensions must conform to:

policy
security
semantic validation
provenance
verification

---

56. No vendor lock-in

Core resilience must not contain vendor-specific concepts as architectural requirements.

Forbidden pattern:

match backend {
    VendorA => ...
    VendorB => ...
}

inside generic resilience logic.

Preferred:

capability discovery
+
generic backend contract
+
adapter

This allows future hardware architectures to be added without redesigning resilience.

---

57. Error handling contract

All fallible operations must preserve:

what failed
where it failed
why it failed
severity
retryability
recoverability
relevant resource
relevant operation
relevant qubit
safe context
underlying cause

Do not collapse every failure into:

String

or:

anyhow::Error

at the architectural boundary.

Rich structured errors are required for autonomous recovery.

---

58. Error information must remain safe

Errors may be logged, transported and persisted.

Therefore:

human display

must not automatically expose:

source internals
credentials
backend secrets
private metadata

Machine-readable fields should be used for automation.

Human-readable messages are not stable protocols.

The existing error implementation explicitly establishes this distinction.

---

59. Implementation order

To satisfy the requirement that each file can be completed independently without later architectural redesign, implementation should proceed from low-dependency contracts upward.

Phase 1 — independent contracts

errors/error.rs
errors/classification.rs

model/resource.rs
model/confidence.rs
model/severity.rs
model/fault.rs
model/incident.rs
model/health.rs
model/degradation.rs
model/capability.rs

These define the vocabulary.

---

Phase 2 — policy

policy/constraints.rs
policy/objectives.rs
policy/budgets.rs
policy/safety.rs
policy/retry.rs
policy/escalation.rs
policy/policy.rs

---

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

---

Phase 4 — diagnosis

diagnosis/classifier.rs
diagnosis/correlation.rs
diagnosis/localization.rs
diagnosis/root_cause.rs
diagnosis/confidence.rs
diagnosis/diagnostician.rs

---

Phase 5 — planning

planning/action.rs
planning/cost.rs
planning/feasibility.rs
planning/ranking.rs
planning/plan.rs
planning/planner_state.rs
planning/planner.rs

---

Phase 6 — adaptation

adaptation/remapping.rs
adaptation/rerouting.rs
adaptation/rescheduling.rs
adaptation/recompilation.rs
adaptation/reoptimization.rs
adaptation/qec_adaptation.rs
adaptation/backend_selection.rs
adaptation/adapter.rs

---

Phase 7 — recovery

recovery/retry.rs
recovery/restart.rs
recovery/checkpoint.rs
recovery/rollback.rs
recovery/resume.rs
recovery/migration.rs
recovery/compensation.rs
recovery/recoverer.rs

---

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

---

Phase 9 — verification

verification/invariant.rs
verification/semantic.rs
verification/result.rs
verification/confidence.rs
verification/provenance.rs
verification/acceptance.rs
verification/verifier.rs

---

Phase 10 — state and checkpointing

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

---

Phase 11 — history and learning

history/incident.rs
history/execution.rs
history/recovery.rs
history/statistics.rs

learning/features.rs
learning/model.rs
learning/predictor.rs
learning/strategy.rs
learning/feedback.rs

---

Phase 12 — distributed resilience

coordination/ownership.rs
coordination/lease.rs
coordination/distributed.rs
coordination/consensus.rs
coordination/coordinator.rs

---

Phase 13 — registries and API

registry/detector.rs
registry/strategy.rs
registry/recovery.rs
registry/backend.rs

api/request.rs
api/response.rs
api/context.rs
api/controller.rs

---

Phase 14 — serialization

serialization/schema.rs
serialization/version.rs
serialization/encode.rs
serialization/decode.rs

---

Phase 15 — limits

limits/resource.rs
limits/validation.rs
limits/limits.rs

---

Phase 16 — module composition

Only after the actual implementation files exist:

errors/mod.rs
model/mod.rs
detection/mod.rs
diagnosis/mod.rs
policy/mod.rs
planning/mod.rs
adaptation/mod.rs
recovery/mod.rs
mitigation/mod.rs
verification/mod.rs
state/mod.rs
checkpoint/mod.rs
telemetry/mod.rs
history/mod.rs
learning/mod.rs
coordination/mod.rs
serialization/mod.rs
limits/mod.rs
registry/mod.rs
api/mod.rs

Then:

quantum/resilience/mod.rs

---

60. Why this ordering matters

Each lower layer establishes contracts for the layer above it.

For example:

errors
    ↓
model
    ↓
policy
    ↓
detection
    ↓
diagnosis
    ↓
planning
    ↓
adaptation
    ↓
recovery
    ↓
verification
    ↓
API

Therefore later files should consume earlier contracts rather than forcing earlier files to be rewritten.

This is the intended meaning of:

«Finish each file once, then integrate it without redesigning it when another file appears.»

---

61. Testing requirements

Production readiness requires more than compilation.

Every subsystem should have:

- unit tests;
- property tests;
- boundary tests;
- invalid-input tests;
- deterministic replay tests;
- serialization tests;
- fault-injection tests;
- integration tests.

---

62. Fault-injection matrix

Resilience must be tested against:

single-qubit failure
multi-qubit failure
correlated fault
leakage
loss
erasure
gate failure
readout failure
calibration drift
timing drift
topology change
resource loss
backend outage
backend timeout
network failure
compiler failure
routing failure
scheduler failure
QEC degradation
decoder failure
checkpoint corruption
checkpoint incompatibility
telemetry corruption
unauthorized recovery
verification failure

ZQN should provide canonical quantum fault semantics for quantum-specific injections.

---

63. Scalability testing

Tests must not consist only of:

1
10
100
1000

hard-coded cases.

Instead, test parameterized resource sizes and generated workloads.

The test framework must verify that algorithms do not depend on:

fixed qubit numbers
fixed topology
fixed gate counts
fixed retry counts
fixed provider names

The actual upper bound should come from the test environment's available resources.

---

64. Property testing

Useful properties include:

Semantic preservation

adapt(program) ≡ program

under the defined semantic equivalence relation.

Recovery idempotence where applicable

Repeatedly applying an already-completed recovery should not corrupt state.

Determinism

Same inputs under deterministic mode produce the same decision.

Verification safety

No unverified recovery result can become "ACCEPT".

Resource correctness

No plan can require resources unavailable under its capability snapshot.

Error stability

The same semantic failure maps to the same stable error code.

---

65. Fuzzing

Fuzz:

- telemetry;
- fault observations;
- diagnosis evidence;
- policy combinations;
- recovery plans;
- serialized checkpoints;
- serialized errors;
- resource graphs;
- topology changes;
- capability changes.

The goal is to ensure malformed external information cannot crash or corrupt the resilience engine.

---

66. Security testing

Test:

forged telemetry
tampered checkpoint
malicious plugin
invalid resource identity
unauthorized migration
invalid recovery request
corrupted result
conflicting observations
malformed serialized data

The expected behavior must be safe rejection or controlled escalation.

---

67. Observability testing

Every major state transition should be observable.

Tests should verify:

incident ID
execution ID
plan ID
recovery ID
verification ID
provenance

remain correlated.

---

68. Deterministic replay

A production incident should be replayable from:

program identity
IR
hardware snapshot
capability snapshot
telemetry
fault observations
policy
configuration
random seed

Replay should produce equivalent decisions under deterministic mode.

This is essential for debugging autonomous recovery.

---

69. Production readiness gates

"quantum::resilience" must not be declared production-ready until all of the following are satisfied.

Correctness

- semantic preservation;
- invariant checking;
- no silent corruption;
- verification before acceptance.

Scalability

- no artificial machine-size ceiling;
- dynamic resource discovery;
- sparse representations where appropriate;
- hierarchical recovery;
- bounded memory behavior;
- streaming telemetry.

Reliability

- retry;
- restart;
- checkpoint;
- rollback;
- resume;
- migration;
- graceful degradation.

Quantum correctness

- canonical qubit identities;
- logical/physical separation;
- QEC integration;
- fault-model integration;
- mitigation separation;
- result verification.

Security

- authenticated observations;
- authorization;
- checkpoint integrity;
- secure provenance;
- plugin restrictions;
- safe error handling.

Compatibility

- versioned schemas;
- IR compatibility;
- checkpoint compatibility;
- target capability negotiation;
- backend abstraction.

Observability

- events;
- metrics;
- traces;
- history;
- provenance.

Testing

- unit;
- property;
- fuzz;
- fault injection;
- simulation;
- integration;
- deterministic replay;
- scale testing.

---

70. Relationship to OpenQASM and target independence

Zamani's architecture follows the important distinction between program semantics and target-specific execution.

The canonical Zamani IR describes the computation rather than hard-coding the physical machine.

Consequently:

Zamani program
       ↓
canonical IR
       ↓
target capabilities
       ↓
routing
       ↓
scheduling
       ↓
hardware execution

rather than:

Zamani program
       ↓
fixed hardware instructions

Resilience operates at the level where target changes can be detected and acted upon without changing the semantic program.

---

71. Error mitigation versus QEC

These must remain separate concepts.

QEC
    changes the representation/execution strategy
    to detect/correct quantum errors

Mitigation
    attempts to reduce or compensate observable
    computational error without being equivalent to
    full quantum error correction

Resilience
    decides whether and when either should be used

This separation permits future error-suppression and mitigation techniques without changing the QEC architecture.

---

72. Self-healing definition

Zamani should use the term self-healing carefully.

Self-healing means:

observe
→ reason
→ adapt
→ recover
→ verify

It does not mean:

automatically hide failures

It does not mean:

always retry

It does not mean:

always migrate

It does not mean:

return the best-looking result

It means:

«Autonomously restore a valid execution when possible while preserving semantic correctness, policy constraints, security and provenance.»

---

73. Failure escalation

The recovery system should follow a hierarchy such as:

local correction
      ↓
local adaptation
      ↓
regional adaptation
      ↓
device adaptation
      ↓
backend migration
      ↓
distributed migration
      ↓
escalation
      ↓
safe termination

The smallest valid recovery scope should normally be preferred.

---

74. No silent semantic downgrade

If the requested program requires:

strict correctness

the system must not silently switch to:

lower-quality approximate execution

without policy authorization.

Likewise, if the user permits degraded operation, that degradation must be explicitly represented in the result.

---

75. No silent provider migration

Migration between backends must be represented in provenance.

The user may write one program, but the execution history must record:

initial target
→ failure
→ new target
→ adaptation
→ verification

This preserves auditability.

---

76. Resource identity versus resource capacity

These concepts must remain separate.

Identity:

which resource?

Capacity:

what can it do?

State:

what condition is it in?

Policy:

may we use it?

Resilience must never combine these into one hard-coded abstraction.

---

77. Capability negotiation

Before recovery, the planner should establish:

required capability
        versus
available capability

For example:

program requires:
    operation A
    connectivity B
    measurement C
    timing D

candidate target provides:
    operation A
    connectivity B
    measurement C
    timing D

Then migration is feasible.

If a required capability is absent:

migration rejected

rather than silently changing semantics.

---

78. Version compatibility

Compatibility must consider:

Zamani language version
IR version
resilience schema
checkpoint schema
hardware capability schema
QEC configuration
backend interface
compiler version

Compatibility decisions must be explicit.

---

79. Serialization compatibility

Serialized objects must be treated as external input.

Therefore:

decode
→ validate
→ verify version
→ verify integrity
→ validate semantics

before use.

Never deserialize and immediately execute.

---

80. State-machine correctness

Recovery transitions must be explicit.

Invalid transitions must fail.

For example:

Idle → Recovering

must not occur unless the required planning and authorization stages have been satisfied, unless an explicitly documented emergency path exists.

---

81. Concurrency

At large scale, multiple incidents may occur simultaneously.

Resilience must support:

independent incident recovery

where safe.

It must also detect:

conflicting recovery plans

when two recoveries affect the same resources.

---

82. Recovery ownership

Distributed systems require ownership.

A recovery action must have an owner or lease so that two resilience controllers do not simultaneously execute incompatible actions.

---

83. Resource quarantine

A suspected failed resource may be temporarily quarantined.

For example:

physical resource
      ↓
suspected unstable
      ↓
quarantine
      ↓
diagnostics/calibration
      ↓
healthy
      ↓
return to pool

Quarantine must be reversible where appropriate and recorded in provenance.

---

84. Learning safety boundary

Learning must never become an uncontrolled authority.

Correct:

learned model
     ↓
plan ranking
     ↓
policy
     ↓
verification

Incorrect:

learned model
     ↓
execute arbitrary recovery

---

85. API stability

Public resilience APIs should be deliberately small.

Prefer:

ResilienceController
ResilienceRequest
ResilienceResponse
ResilienceContext
ResilienceError

over exposing every internal implementation type.

Internal implementations can evolve without breaking application code.

---

86. No global mutable state

The resilience core should avoid process-wide mutable state.

Prefer explicit:

context
registry
state store
policy
history

dependencies.

This improves:

- deterministic tests;
- concurrency;
- isolation;
- replay;
- distributed execution;
- embedding.

---

87. No hidden I/O

Domain-model files should not silently perform:

- network calls;
- filesystem operations;
- backend calls;
- cloud API calls.

I/O belongs at explicit integration boundaries.

---

88. No hidden retries

A low-level function must not silently retry an operation unless its contract explicitly says so.

Otherwise:

planner

cannot accurately reason about:

cost
latency
budget

and users cannot understand why execution took longer.

---

89. No hidden mutation

Recovery plans should be immutable once committed for execution.

If conditions change:

old plan
    ↓
stale
    ↓
re-diagnose/replan

rather than silently mutating the plan underneath the executor.

---

90. Plan staleness

Plans should be associated with the state/capability snapshot against which they were produced.

If:

capabilities changed

then:

plan → stale

and the planner must determine whether it remains valid.

---

91. Semantic equivalence

Adaptation should rely on the canonical IR's semantic equivalence rules.

For example:

original implementation

and:

rerouted implementation

may differ structurally while representing the same computation.

Verification must check semantic equivalence, not merely byte-for-byte equality.

---

92. Hardware abstraction

The resilience layer must never assume:

superconducting
trapped ion
neutral atom
photonic
spin
annealing
continuous variable

as the only possible architectures.

The target is described through capabilities and contracts.

This allows future quantum architectures to participate without rewriting resilience.

---

93. Beyond circuit-only computation

The architecture must not assume that every quantum computation is a simple:

Vec<Gate>

circuit.

The canonical IR is explicitly designed as a universal structured representation capable of representing circuit and other computational models.

Resilience should therefore operate on execution contracts and semantic program representations rather than assuming gate-list semantics everywhere.

---

94. Resource-aware planning

The planner must consider:

available qubits
available logical qubits
available physical qubits
connectivity
timing
memory
control channels
execution slots
backend availability

but it must receive these dynamically.

No fixed resource model is permitted.

---

95. Memory scalability

For large systems, avoid:

one giant object containing every telemetry sample

or:

one global vector of every event ever generated

unless explicitly bounded by policy.

Use:

- streaming;
- partitioning;
- aggregation;
- persistence;
- summarization;
- bounded caches.

---

96. Recovery cost awareness

A recovery action can be worse than the original failure.

For example:

minor transient issue

should not automatically trigger:

full recompilation
full migration
large-scale mitigation

The planner must consider cost and risk.

---

97. Verification cost awareness

Verification itself consumes resources.

The architecture should permit:

verification depth

to be policy-driven.

However:

«Resource constraints must never justify falsely claiming that an unverified result is verified.»

---

98. Availability versus correctness

Correctness takes precedence unless policy explicitly defines a permitted degraded mode.

Conceptually:

correct result
    > degraded but explicitly permitted result
    > unavailable result
    > incorrect result

An incorrect result must never be presented as successful merely because it is available.

---

99. Final architectural contract

The entire subsystem can be summarized as:

                     Zamani Quantum Program
                                │
                                ▼
                         Canonical Quantum IR
                                │
                                ▼
                     Target-independent execution
                                │
                                ▼
                    ┌────────────────────────┐
                    │       RESILIENCE       │
                    │                        │
                    │ Detect                 │
                    │ Diagnose               │
                    │ Policy                 │
                    │ Plan                   │
                    │ Adapt                  │
                    │ Recover                │
                    │ Mitigate               │
                    │ Verify                 │
                    └───────────┬────────────┘
                                │
               ┌────────────────┼────────────────┐
               │                │                │
               ▼                ▼                ▼
            Routing         Scheduling          QEC
               │                │                │
               └────────────────┼────────────────┘
                                ▼
                         Hardware / Simulator
                                │
                                ▼
                              Result
                                │
                                ▼
                            Verification
                                │
                       ┌────────┴────────┐
                       ▼                 ▼
                    ACCEPT             REPEAT

The central invariant remains:

«The Zamani program expresses the computation. Resilience adapts the execution without changing the computation's intended semantics.»

---

100. Definition of production readiness

"src/quantum/resilience/" is production-ready only when:

✓ no unsafe code
✓ no artificial quantum-machine size limit
✓ no hard-coded provider assumptions
✓ no duplicate QubitId
✓ canonical IR integration
✓ canonical ZQN fault integration
✓ QEC integration
✓ hardware capability integration
✓ routing integration
✓ scheduling integration
✓ optimization integration
✓ simulation integration
✓ benchmarking integration
✓ policy-driven recovery
✓ structured errors
✓ deterministic mode
✓ provenance
✓ checkpoint integrity
✓ migration
✓ graceful degradation
✓ verification before acceptance
✓ security boundary
✓ fault injection
✓ property testing
✓ fuzz testing
✓ scale testing
✓ deterministic replay
✓ distributed coordination support
✓ versioned serialization
✓ observable recovery lifecycle

Only after these conditions are satisfied should the subsystem be treated as a production execution-resilience layer.

---

101. Final principle

The architectural objective of Zamani is not:

write once
run on one particular QPU

It is:

write once
        ↓
preserve semantics
        ↓
discover available resources
        ↓
compile/adapt
        ↓
route
        ↓
schedule
        ↓
execute
        ↓
detect
        ↓
diagnose
        ↓
recover/adapt
        ↓
verify
        ↓
continue

Therefore:

«A Zamani quantum program should remain stable while the physical implementation underneath it can change continuously.»

The resilience subsystem is the mechanism that makes that principle operationally reliable.

It does not make an infinite physical machine possible. It ensures that Zamani itself does not impose an artificial finite machine-size ceiling and can exploit whatever compatible quantum resources are actually available, from the smallest execution environment to arbitrarily large systems subject to their real resource constraints.This README is intentionally aligned with the existing repository's canonical IR rules rather than introducing a parallel architecture; in particular, quantum::ir::qubit remains the sole authoritative qubit identity boundary.