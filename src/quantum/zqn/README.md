Worked for 15s

Yes. I checked the current repository structure rather than treating the earlier proposed tree as hypothetical. The repository now has the major ZQN subdirectories (core, probability, channel, fault, noise, operations, calibration, characterization, simulation, propagation, target, integration, io, and tests). The important existing defect is also confirmed: the current root module is literally named mod.rs  with a trailing space, not mod.rs.

I also checked the repository-level architecture and current external compiler/IR guidance. MLIR's current design strongly reinforces the use of stable interfaces, verification, extensibility, explicit dialect/module boundaries, and multiple representations rather than hard-coding transformations around concrete operation types. 

Below is the complete src/quantum/zqn/README.md I recommend using as the normative architecture and integration contract.

Zamani Quantum Noise — ZQN

Path: "src/quantum/zqn/"
Project: Zamani
Language: Rust 2021
Supported compiler: Rust 1.97 / Rust 1.97.1
Safety requirement: "unsafe" is forbidden
Status target: Production-ready architecture
Primary objective: Backend-independent, deterministic, scalable quantum noise and uncertainty infrastructure

---

1. Purpose

ZQN means Zamani Quantum Noise.

ZQN is the canonical Zamani subsystem for representing, validating, applying, characterizing, propagating, simulating, and exchanging quantum physical uncertainty.

ZQN exists to answer:

«Given the semantic quantum computation represented by Zamani's canonical quantum IR, what physical noise, uncertainty, faults, calibration state, environmental effects, and execution-dependent disturbances affect that computation?»

ZQN does not replace the canonical quantum IR.

The architectural boundary is:

Zamani source
     │
     ▼
quantum::frontend
     │
     ▼
quantum::ir
     │
     │ canonical computational meaning
     ▼
     ┌────────────────────────────┐
     │            ZQN             │
     │                            │
     │ probabilities              │
     │ quantum channels            │
     │ faults                      │
     │ noise models                │
     │ correlations                │
     │ calibration                 │
     │ characterization            │
     │ uncertainty                 │
     │ provenance                  │
     │ deterministic sampling      │
     └──────────────┬─────────────┘
                    │
       ┌────────────┼─────────────┐
       ▼            ▼             ▼
    routing      scheduling       QEC
       │            │             │
       └────────────┼─────────────┘
                    ▼
             hardware/runtime
                    │
          ┌─────────┼─────────┐
          ▼         ▼         ▼
      simulator     QPU     emulator
          │         │         │
          └─────────┼─────────┘
                    ▼
             observations
                    │
          ┌─────────┼──────────┐
          ▼         ▼          ▼
   characterization benchmarking analysis
          │
          ▼
      calibration
          │
          └──────────────► ZQN

---

2. Core architectural principle

The single most important ZQN rule is:

«The quantum program describes intent; quantum IR describes semantics; ZQN describes physical uncertainty; other subsystems decide placement, timing, fault tolerance, execution, and measurement.»

Therefore:

Subsystem| Owns
"quantum::frontend"| Source-language parsing/lowering
"quantum::ir"| Canonical quantum semantics
"zqn"| Noise, channels, faults, uncertainty, calibration, characterization
routing| Logical-to-physical placement
scheduling| Temporal ordering and resource timing
QEC| Syndrome processing, decoding, correction, logical fault handling
hardware| Target capabilities and physical execution
runtime| Execution orchestration
benchmarking| Experimental/benchmark methodology
memory/state| State representation and state evolution infrastructure

No subsystem should silently assume another subsystem's responsibilities.

---

3. Write once, scale everywhere

ZQN is designed around the requirement:

«A Zamani quantum program is written once and can scale from the smallest supported quantum resource to arbitrarily large systems subject only to the actual resources, target capabilities, numerical representation, execution policy, and physical hardware available.»

This means ZQN must never encode a semantic machine-size maximum.

Forbidden:

const MAX_QUBITS: usize = 1000;

Forbidden:

if qubits == 5 { ... }
if qubits == 20 { ... }
if qubits == 127 { ... }

Forbidden:

if vendor == "some_qpu" { ... }

Forbidden:

if technology == "superconducting" { ... }

Instead:

abstract computation
       +
abstract noise model
       +
target capabilities
       +
resource policy
       +
calibration
       +
execution context
       ↓
target realization

The system may impose runtime safety/resource policies, but those policies must never become part of the mathematical meaning of a ZQN model.

---

4. "Infinity" and scalability

"Scale to infinity" means:

«ZQN has no artificial architectural upper bound on the number of resources, operations, time steps, correlations, nodes, modes, or physical systems represented by its semantic model.»

It does not mean every backend can physically materialize an infinite state.

For example:

1 resource
10 resources
100 resources
10,000 resources
10,000,000 resources
distributed resources
future quantum resources

may require completely different representations.

Therefore ZQN must be representation-polymorphic.

Possible realizations include:

exact
symbolic
sparse
dense
sampled
stochastic
trajectory
tensorized
tensor-network
process representation
hardware-native
approximate
bounded
statistical
streaming
lazy

Representation selection must depend on:

requested semantics
+
target capabilities
+
resource policy
+
numerical requirements
+
execution context

not on arbitrary compile-time constants.

---

5. Canonical resource identity

ZQN must not introduce a second generic quantum-qubit identity system.

When a physical or logical qubit identity is required, ZQN must use the canonical identifiers defined by:

crate::quantum::ir::qubit

including the existing:

QubitId
PhysicalQubitId

where their semantics match the required use.

Do not create:

zqn::QubitId

as a duplicate of the canonical IR identity.

ZQN may define identifiers for entities that are genuinely owned by ZQN, for example:

NoiseModelId
ChannelId
FaultId
CalibrationId
CalibrationSnapshotId
CharacterizationId
ExperimentId
ObservationId

These identify ZQN objects, not quantum resources already owned by the canonical IR.

---

6. Ownership rule

ZQN owns:

- noise semantics;
- stochastic models;
- deterministic fault models;
- quantum channels;
- channel representations;
- probability distributions;
- correlations;
- temporal noise;
- spatial noise;
- crosstalk;
- leakage;
- erasure;
- loss;
- readout errors;
- preparation errors;
- reset errors;
- idle noise;
- pulse noise;
- transport noise;
- calibration state;
- calibration uncertainty;
- drift models;
- characterization results;
- uncertainty propagation;
- error budgets;
- reproducible noise sampling;
- noise-related provenance;
- target noise requirements/capabilities.

ZQN does not own:

- source parsing;
- the canonical quantum IR;
- vendor APIs;
- QPU credentials;
- QPU transport;
- routing algorithms;
- scheduling algorithms;
- QEC decoding;
- logical correction;
- benchmark methodology;
- application semantics;
- UI;
- CLI;
- cloud orchestration.

---

7. Required directory architecture

The production ZQN architecture is:

src/quantum/zqn/
│
├── README.md
│
├── mod.rs
├── prelude.rs
│
├── core/
│   ├── mod.rs
│   ├── error.rs
│   ├── ids.rs
│   ├── metadata.rs
│   ├── version.rs
│   ├── context.rs
│   ├── capabilities.rs
│   ├── limits.rs
│   └── provenance.rs
│
├── probability/
│   ├── mod.rs
│   ├── probability.rs
│   ├── distribution.rs
│   ├── categorical.rs
│   ├── continuous.rs
│   ├── bounds.rs
│   └── statistics.rs
│
├── channel/
│   ├── mod.rs
│   ├── channel.rs
│   ├── representation.rs
│   ├── kraus.rs
│   ├── choi.rs
│   ├── process_matrix.rs
│   ├── pauli.rs
│   ├── stochastic.rs
│   ├── lindblad.rs
│   ├── thermal.rs
│   ├── amplitude.rs
│   ├── phase.rs
│   ├── depolarizing.rs
│   ├── generalized.rs
│   └── composition.rs
│
├── fault/
│   ├── mod.rs
│   ├── fault.rs
│   ├── location.rs
│   ├── classification.rs
│   ├── correlated.rs
│   ├── leakage.rs
│   ├── erasure.rs
│   ├── loss.rs
│   └── batch.rs
│
├── noise/
│   ├── mod.rs
│   ├── model.rs
│   ├── specification.rs
│   ├── application.rs
│   ├── composition.rs
│   ├── correlation.rs
│   ├── temporal.rs
│   ├── spatial.rs
│   ├── crosstalk.rs
│   ├── drift.rs
│   ├── non_markovian.rs
│   └── conditional.rs
│
├── operations/
│   ├── mod.rs
│   ├── operation.rs
│   ├── gate.rs
│   ├── preparation.rs
│   ├── reset.rs
│   ├── measurement.rs
│   ├── idle.rs
│   ├── pulse.rs
│   └── transport.rs
│
├── calibration/
│   ├── mod.rs
│   ├── snapshot.rs
│   ├── parameter.rs
│   ├── device.rs
│   ├── gate.rs
│   ├── readout.rs
│   ├── measurement.rs
│   ├── drift.rs
│   ├── interpolation.rs
│   └── validation.rs
│
├── characterization/
│   ├── mod.rs
│   ├── experiment.rs
│   ├── protocol.rs
│   ├── observation.rs
│   ├── estimator.rs
│   ├── uncertainty.rs
│   ├── tomography.rs
│   ├── randomized_benchmarking.rs
│   └── process_characterization.rs
│
├── simulation/
│   ├── mod.rs
│   ├── engine.rs
│   ├── sampler.rs
│   ├── trajectory.rs
│   ├── channel_engine.rs
│   ├── monte_carlo.rs
│   ├── deterministic.rs
│   └── reproducibility.rs
│
├── propagation/
│   ├── mod.rs
│   ├── error_budget.rs
│   ├── uncertainty.rs
│   ├── fidelity.rs
│   ├── bounds.rs
│   ├── sensitivity.rs
│   └── accumulation.rs
│
├── target/
│   ├── mod.rs
│   ├── requirements.rs
│   ├── capabilities.rs
│   ├── compatibility.rs
│   ├── lowering.rs
│   └── validation.rs
│
├── integration/
│   ├── mod.rs
│   ├── ir.rs
│   ├── routing.rs
│   ├── scheduling.rs
│   ├── qec.rs
│   ├── hardware.rs
│   ├── memory.rs
│   ├── benchmarking.rs
│   └── runtime.rs
│
├── io/
│   ├── mod.rs
│   ├── schema.rs
│   ├── serialization.rs
│   ├── deserialization.rs
│   ├── canonical.rs
│   └── compatibility.rs
│
└── tests/
    ├── mod.rs
    ├── unit/
    ├── property/
    ├── differential/
    ├── determinism/
    ├── scaling/
    ├── compatibility/
    ├── integration/
    └── fixtures/

Files may only be added when their ownership and dependency boundary are defined.

---

8. Current repository correction

The repository currently contains:

src/quantum/zqn/mod.rs 

with a trailing space.

This is incorrect.

It must become:

src/quantum/zqn/mod.rs

The trailing-space path must not coexist with the correct path.

This is a filesystem/module correctness issue and must be fixed before relying on the ZQN module root.

---

9. Rust requirements

ZQN must compile under:

Rust 1.97
Rust 1.97.1
edition = 2021

No "unsafe" code is permitted.

Do not introduce:

unsafe { ... }

or:

unsafe fn

or:

unsafe trait

or unsafe FFI assumptions.

If an external dependency requires unsafe internally, ZQN itself must not expose unsafe semantics and must not depend on unsafe behavior for its correctness guarantees.

---

10. Error architecture

"core/error.rs" is the single ZQN error boundary.

It must define the authoritative ZQN result/error vocabulary.

It must cover at least:

invalid probability
invalid distribution
invalid channel
invalid fault
invalid noise model
invalid calibration
invalid characterization
unsupported representation
unsupported operation
capability mismatch
resource limit exceeded
numerical failure
non-finite value
validation failure
serialization failure
deserialization failure
compatibility failure
determinism failure
cancellation

Every ZQN subsystem must either:

1. use the ZQN error hierarchy, or
2. convert its lower-level error into it.

No subsystem should create an unrelated competing ZQN-wide error hierarchy.

---

11. Resource limits

"core/limits.rs" defines policy, not semantic machine limits.

Example categories:

maximum operations processed
maximum faults materialized
maximum distribution entries
maximum allocation budget
maximum sampling shots
maximum tensor elements
maximum execution duration
maximum correlation expansion

A limit may be:

Some(value)
None

where "None" means that ZQN itself does not impose that particular policy limit.

A runtime, user, backend, sandbox, or deployment may still impose limits.

The distinction is mandatory:

semantic capability
        ≠
resource policy

A machine with one million resources must not become impossible merely because ZQN's mathematical API was designed around a smaller number.

---

12. Numerical safety

ZQN must reject invalid numerical states.

Never silently convert:

NaN → 0
∞ → maximum
negative probability → absolute value

All public numerical constructors must establish their invariants.

Use:

- checked arithmetic;
- finite-value validation;
- explicit tolerances;
- explicit approximation;
- explicit error bounds;
- explicit confidence intervals.

Numerical approximation must never be silently presented as exact.

---

13. Probability subsystem

"probability/" is the mathematical foundation for stochastic ZQN models.

It must support:

- exact probabilities where practical;
- bounded probabilities;
- distributions;
- categorical distributions;
- continuous distributions;
- statistical estimates;
- confidence intervals;
- uncertainty bounds;
- expectation;
- variance;
- reproducible sampling.

Do not assume every noise model is:

two outcomes

or:

Pauli

or:

finite and tiny

---

14. Quantum-channel subsystem

"channel/" owns the mathematical abstraction of quantum channels.

The fundamental abstraction must be representation-independent.

Supported representations may include:

Kraus
Choi
process matrix
superoperator
Liouville representation
Pauli transfer
stochastic map
Lindblad generator
thermal channel
amplitude damping
phase damping
depolarizing
generalized channel

No representation is universally mandatory.

A channel representation must be selected based on:

mathematical validity
target capability
available resources
precision requirements
performance

---

15. Channel invariants

Where a type represents a physical quantum channel, validation must establish the appropriate mathematical invariants.

Depending on representation, this includes:

- complete positivity;
- trace preservation;
- dimensional consistency;
- finite numerical values;
- valid subsystem mapping;
- valid composition;
- valid tensor product.

Invalid channels must fail validation instead of entering downstream execution.

---

16. Channel composition

"channel/composition.rs" owns:

sequential composition
parallel composition
tensor product
correlated composition

It must not assume a fixed number of subsystems.

The number of resources is data.

Never introduce:

TwoQubitChannel
ThreeQubitChannel

as architectural primitives.

A general channel must work for arbitrary resource collections permitted by the representation and resource policy.

---

17. Fault subsystem

"fault/" represents discrete physical fault events.

A fault is not automatically equivalent to a channel.

Examples:

Pauli fault
leakage
erasure
loss
readout fault
preparation fault
transport fault
correlated fault
deterministic fault

A noise model may produce faults.

A channel describes physical transformation.

These concepts must remain distinct.

---

18. Fault location

"fault/location.rs" must support locations such as:

canonical quantum resource
physical resource
logical resource
operation
measurement
reset
pulse
time interval
communication link
composite resource

When a qubit identity is needed, use the canonical "quantum::ir::qubit" identifiers.

Do not invent a second qubit namespace.

---

19. Correlated faults

"fault/correlated.rs" must support arbitrary resource cardinality.

Do not define:

TwoQubitCorrelatedFault
ThreeQubitCorrelatedFault
FourQubitCorrelatedFault

Instead represent:

correlated fault
+
arbitrary resource set
+
correlation structure

This is necessary for:

- burst errors;
- correlated QEC failures;
- collective noise;
- spatial correlation;
- temporal correlation;
- distributed systems.

---

20. Noise-model subsystem

"noise/model.rs" is the central ZQN abstraction.

A noise model must be capable of answering questions such as:

What noise applies to this operation?
What channel represents it?
What fault distribution does it induce?
What resources does it affect?
What time interval matters?
What calibration is required?
What capabilities are required?
Can the requested realization be exact?
If approximate, what is the bound?

The noise model must remain independent of:

- QPU vendor;
- simulator implementation;
- scheduler implementation;
- router implementation;
- QEC decoder.

---

21. Declarative noise specification

"noise/specification.rs" represents noise configuration independently from execution.

It should be possible to construct a model from:

program configuration
calibration data
characterization result
user declaration
experiment result
generated model

The specification must not require a fixed target size.

---

22. Noise application

"noise/application.rs" defines how a noise model is associated with a semantic operation.

Noise may be attached to:

preparation
gate
measurement
reset
idle
pulse
transport
communication
logical operation
time interval

This must support dynamic and conditional computation.

---

23. Correlation model

"noise/correlation.rs" must represent arbitrary correlation structures.

Potential models include:

independent
pairwise
higher-order
graph-based
tensor-based
collective
spatial
temporal
spatiotemporal
environment-mediated

Do not make pairwise independence the default assumption unless explicitly declared.

---

24. Temporal noise

"noise/temporal.rs" must support:

stationary noise
non-stationary noise
time-dependent parameters
drift
temporal correlation
history-dependent behavior

The API must represent physical time without assuming one fixed clock resolution.

---

25. Spatial noise

"noise/spatial.rs" must support:

local correlation
arbitrary topology
long-range correlation
collective noise
topology-dependent correlation

The topology comes from the target/IR/resource layer.

ZQN must not hard-code:

line
grid
heavy-hex
ring
star

as the only supported structures.

---

26. Crosstalk

"noise/crosstalk.rs" represents unintended interaction between simultaneously active resources.

Crosstalk must be queried by the scheduler and routing layers, not used to replace those layers.

Example:

scheduler
    │
    │ proposed simultaneous operations
    ▼
ZQN
    │
    │ crosstalk cost/risk
    ▼
scheduler

---

27. Non-Markovian noise

"noise/non_markovian.rs" must not force all physical behavior into independent per-operation noise.

Support concepts such as:

memory
history dependence
environment state
memory kernels
process tensors
multi-time correlations

Approximations must explicitly state when a non-Markovian model is reduced to a Markovian model.

---

28. Conditional noise

"noise/conditional.rs" supports noise dependent on:

operation
resource
time
calibration state
measurement result
classical condition
environment
execution context

This is required for dynamic quantum programs.

---

29. Operations subsystem

"operations/" defines noise semantics around broad quantum operations.

It must not redefine the canonical quantum IR's operation model.

Instead:

quantum::ir operation
        +
ZQN operation-noise semantics

forms the noise-aware view.

---

30. Preparation noise

"operations/preparation.rs" owns:

- state-preparation error;
- initialization error;
- thermal preparation;
- leakage during preparation;
- imperfect preparation channels.

---

31. Reset noise

"operations/reset.rs" treats reset as its own operation.

Do not automatically define reset as:

measurement + X

unless an explicit target lowering declares that equivalence.

---

32. Measurement noise

"operations/measurement.rs" supports:

- assignment errors;
- asymmetric readout;
- state-dependent readout;
- correlated readout;
- measurement backaction;
- measurement uncertainty.

---

33. Idle noise

"operations/idle.rs" is essential for scheduler integration.

It must answer:

What happens to this resource while it waits?

Inputs may include:

resource
duration
calibration
environment
noise model

This enables noise-aware scheduling without coupling the scheduler to physical noise mathematics.

---

34. Pulse noise

"operations/pulse.rs" provides pulse-level noise semantics.

High-level programs must not be forced to use pulse descriptions.

The layering is:

high-level IR
      ↓
scheduled operation
      ↓
pulse realization
      ↓
pulse noise

---

35. Transport noise

"operations/transport.rs" supports physical systems in which quantum resources move.

Examples include:

- ion shuttling;
- photonic transport;
- quantum communication;
- memory movement;
- distributed quantum links.

This prevents ZQN from becoming gate-only.

---

36. Calibration

"calibration/" owns physical parameter state.

A calibration snapshot must be:

identified
versioned
timestamped
scoped
validated
provenance-aware
uncertainty-aware
validity-aware

A calibration is not assumed to remain valid forever.

---

37. Calibration snapshots

"calibration/snapshot.rs" must contain enough information to determine:

what was calibrated
for which target
for which resources
when
from what source
with what uncertainty
for what validity interval
under what model/version

---

38. Calibration parameters

"calibration/parameter.rs" represents generic calibrated values.

Do not hard-code one universal parameter vocabulary.

A parameter must support:

value
unit
uncertainty
validity
resource scope
provenance

---

39. Calibration drift

"calibration/drift.rs" models parameter evolution.

Drift may be:

linear
piecewise
stochastic
environment-dependent
empirically characterized
arbitrary model-defined

The model must not assume one universal drift law.

---

40. Characterization

"characterization/" answers:

«What is the actual noise behavior of the system?»

It owns:

- experiment definitions;
- protocols;
- observations;
- estimators;
- uncertainty;
- tomography;
- randomized benchmarking;
- process characterization.

Characterization produces information consumed by ZQN.

ZQN does not own the complete benchmarking methodology of the repository.

---

41. Characterization observations

Raw observations must remain distinguishable from inferred models.

The distinction is:

raw observation
      ↓
estimator
      ↓
statistical estimate
      ↓
noise model

Never store an estimate as though it were a directly measured fact.

---

42. Simulation

"simulation/" provides execution of ZQN semantics where a simulator is appropriate.

It must not become a second general-purpose quantum simulation architecture if an existing simulation engine already owns that responsibility.

ZQN should adapt to the existing simulation contracts.

This preserves the existing repository architecture.

---

43. Deterministic stochastic execution

ZQN must never rely on hidden global randomness.

Forbidden:

thread_rng()

as an implicit semantic source.

Forbidden:

global mutable RNG

A stochastic execution must receive its deterministic execution context from the caller.

---

44. Reproducibility

"simulation/reproducibility.rs" defines deterministic seed-material derivation.

The deterministic identity must incorporate the relevant execution coordinates, such as:

root seed
program identity
noise model identity
calibration identity
target identity
shot
operation
sample
partition

The exact coordinate set is part of the reproducibility contract.

Parallel execution must not change deterministic results merely because work was scheduled on different workers.

Therefore:

1 worker
8 workers
64 workers
distributed workers

must derive identical stochastic outcomes under the same deterministic policy.

---

45. Monte Carlo

"simulation/monte_carlo.rs" must support streaming execution.

Do not require all samples to exist simultaneously in memory.

Prefer:

iterator
stream
bounded batch
online accumulation

where appropriate.

This is essential for scaling to large shot counts.

---

46. Trajectory simulation

"simulation/trajectory.rs" supports stochastic trajectories without requiring a dense global state representation.

The trajectory abstraction must remain independent from a particular backend.

---

47. Channel engine

"simulation/channel_engine.rs" applies ZQN channels using a suitable representation.

It must not assume:

dense matrix

is always practical.

Representation selection must remain independent from the mathematical channel definition.

---

48. Propagation

"propagation/" determines how uncertainty/noise affects a computation.

It must support:

- error budgets;
- uncertainty propagation;
- fidelity analysis;
- bounds;
- sensitivity;
- accumulation.

---

49. Error budgets

"propagation/error_budget.rs" must allow users and compiler subsystems to ask:

How much error is available?
How much error is expected?
Where is the budget consumed?
Which resource consumes the most?

Error budgets must remain composable.

---

50. Fidelity

"propagation/fidelity.rs" may support appropriate metrics including:

state fidelity
process fidelity
average gate fidelity
entanglement fidelity
classical output distance
trace-distance-related quantities
diamond-distance bounds where computationally appropriate

No single metric is universally correct for every quantum technology.

---

51. Sensitivity

"propagation/sensitivity.rs" identifies parameters whose uncertainty most strongly affects the computation.

Consumers include:

optimization
routing
scheduling
calibration
QEC
benchmarking

---

52. Accumulation

"propagation/accumulation.rs" models accumulation over:

operations
layers
time
resources
logical computation
distributed communication

Never assume all error accumulation is simply:

sum(errors)

unless the mathematical assumptions justify it.

---

53. Target abstraction

The target subsystem is essential to write-once/scale-everywhere behavior.

The target declares:

capabilities
resource availability
supported channel representations
supported noise semantics
timing capabilities
calibration capabilities
measurement capabilities
approximation capabilities

ZQN must consume this abstract contract.

---

54. Requirements

"target/requirements.rs" describes what the requested computation/noise realization needs.

Examples:

requires correlated noise
requires dynamic noise
requires leakage model
requires calibration
requires bounded approximation
requires temporal resolution

---

55. Capabilities

"target/capabilities.rs" describes what the target can support.

It must not use vendor-specific branches.

Prefer:

CapabilitySet

over:

if vendor_name == ...

This follows the same general architectural principle used by extensible compiler IR systems: transformations should operate through abstract interfaces instead of embedding knowledge of every concrete operation/type.

---

56. Compatibility

"target/compatibility.rs" determines:

exactly supported
approximately supported
unsupported

An approximation must be explicit.

Never silently lower:

non-Markovian → Markovian
correlated → independent
continuous → discrete
exact → approximate

without recording the transformation and its declared error/bound.

---

57. Lowering

"target/lowering.rs" converts:

abstract ZQN model
        ↓
target-supported realization

This is where target-specific realization belongs.

Not in:

channel/*.rs
noise/*.rs
probability/*.rs

---

58. Integration with canonical quantum IR

"integration/ir.rs" is the principal ZQN/IR boundary.

The desired relationship is:

quantum::ir
     +
ZQN noise semantics
     ↓
noise-aware IR view / execution contract

Do not make the canonical IR depend on concrete ZQN implementations merely to represent ordinary quantum semantics.

The dependency should remain directional and minimal.

---

59. Integration with routing

"integration/routing.rs" exposes noise information to routing.

Routing may query:

gate error
readout error
idle error
crosstalk
duration-dependent error
correlation
calibration validity
uncertainty

ZQN provides the physical/noise information.

Routing owns the routing decision.

---

60. Integration with scheduling

"integration/scheduling.rs" exposes noise cost as a function of:

operation
resource
duration
concurrency
calibration
environment

The scheduler then decides ordering.

ZQN does not become the scheduler.

---

61. Integration with QEC

The repository already contains a QEC noise implementation.

The long-term ownership must be:

ZQN
 ├── probability
 ├── channels
 ├── faults
 ├── correlations
 ├── leakage
 ├── erasure
 ├── deterministic sampling
 └── physical noise semantics
          │
          ▼
QEC adapter
          │
          ├── syndrome
          ├── decoding
          ├── correction
          └── logical error analysis

Do not maintain two independent universal noise models.

The existing QEC noise implementation should migrate incrementally to consume ZQN.

Migration should use adapters first so existing QEC behavior is not unnecessarily broken.

---

62. Integration with hardware

"integration/hardware.rs" defines the abstract relationship between hardware and ZQN.

Hardware provides:

TargetCapabilities
CalibrationSnapshot
ObservedNoise
MeasurementResults
ExecutionMetadata

ZQN must never directly contain vendor API clients.

Do not create:

zqn/ibm.rs
zqn/ionq.rs
zqn/rigetti.rs
zqn/quantinuum.rs

Vendor implementations belong in the hardware/provider architecture.

---

63. Integration with memory/state

"integration/memory.rs" connects channel/fault semantics with the repository's quantum memory/state infrastructure.

ZQN owns:

what transformation/noise should occur

The memory/state subsystem owns:

how quantum state is represented and stored

This separation is mandatory for scalability.

---

64. Integration with benchmarking

"integration/benchmarking.rs" provides:

noise model
calibration
characterization
observations
uncertainty
error estimates

to the benchmarking subsystem.

Benchmarking owns experiment methodology.

ZQN owns the physical noise representation used by those experiments.

---

65. Integration with runtime

"integration/runtime.rs" defines the runtime contract.

Runtime supplies:

execution context
target
resource policy
clock/time
seed policy
cancellation
calibration

ZQN supplies:

noise realization
fault realization
channel
uncertainty
observation requirements

Runtime performs orchestration.

---

66. I/O architecture

"io/" provides stable interchange.

It must not serialize Rust implementation details as the external contract.

Use explicit:

schema version
semantic version
canonical representation
compatibility policy
migration policy

---

67. Canonical serialization

"io/canonical.rs" provides a deterministic canonical form.

Equivalent semantic objects must have a stable canonical representation where the relevant mathematical semantics permit it.

Canonical representation is required for:

hashing
cache identity
provenance
reproducibility
distributed execution
model comparison

---

68. Versioning

"core/version.rs" is authoritative for ZQN version information.

Do not scatter:

const VERSION: ...

across modules.

Version information must distinguish:

ZQN semantic version
schema version
serialization version
compatibility version

---

69. Provenance

Every important physical/noise object should be able to answer:

Where did this come from?

Possible provenance sources:

user_defined
measured
simulated
inferred
calibrated
synthesized
imported

Record relevant:

source
model identity
dataset identity
experiment identity
calibration identity
software/version
timestamp

This is required for scientific reproducibility.

---

70. Metadata

"core/metadata.rs" contains non-semantic descriptive information.

Examples:

name
description
labels
annotations
units
source references

Metadata must not silently change mathematical semantics.

---

71. Context

"core/context.rs" is the shared execution/configuration boundary.

It should aggregate relevant context such as:

limits
capabilities
calibration
determinism
provenance
cancellation

This prevents APIs from accumulating huge parameter lists while avoiding hidden global state.

---

72. No global mutable state

Forbidden:

GLOBAL_NOISE_MODEL
GLOBAL_CALIBRATION
GLOBAL_RNG
GLOBAL_TARGET
GLOBAL_LIMITS

ZQN behavior must be determined by explicit inputs/context.

---

73. Thread safety

Where semantically possible, ZQN value objects and models should be usable across threads.

Avoid internal mutable shared state.

A computation's result must not depend on accidental worker ordering.

---

74. Distributed execution

Distributed deterministic execution must derive stochastic identity from stable semantic coordinates.

Conceptually:

root seed
   +
program identity
   +
model identity
   +
calibration identity
   +
target identity
   +
partition
   +
operation
   +
resource
   +
shot

The exact derivation belongs in "simulation/reproducibility.rs".

No process-local random state may alter semantic reproducibility.

---

75. Resource governance

Every potentially expensive operation must have a resource policy.

Particularly:

matrix construction
tensor expansion
channel conversion
correlation expansion
sampling
tomography
serialization
deserialization
fault generation
Monte Carlo

A malicious or accidental configuration must not cause uncontrolled allocation merely because a mathematical object is valid in principle.

---

76. Security

ZQN must treat externally supplied noise models, calibration files, serialized channels, and characterization data as untrusted input.

Defend against:

- allocation bombs;
- enormous tensor dimensions;
- enormous distributions;
- enormous correlation graphs;
- pathological recursive data;
- NaN/Infinity injection;
- numerical overflow;
- numerical underflow;
- invalid dimensions;
- nonterminating generators;
- excessive sampling;
- malicious serialized data;
- resource-exhaustion attacks.

No unsafe code is permitted.

---

77. Technology neutrality

ZQN must not assume that quantum computing means only:

qubit
+
gate
+
Pauli error

The architecture must remain extensible to:

gate-model quantum computing
measurement-based quantum computing
analog quantum computing
Hamiltonian simulation
quantum annealing
continuous-variable systems
bosonic systems
fermionic systems
photonic systems
ion systems
neutral atoms
superconducting systems
spin systems
distributed quantum computing
quantum networks
fault-tolerant logical systems
future quantum technologies

This is one reason the noise location abstraction must not be restricted to a single qubit type.

---

78. Resource abstraction

Where a ZQN object must refer to a quantum resource, the semantic resource abstraction should be able to represent:

qubit
qudit
mode
bosonic mode
logical resource
physical resource
operation
measurement
channel
pulse
transport link
composite resource

The canonical "quantum::ir::qubit" identifiers remain authoritative whenever the resource is specifically an IR qubit/physical qubit.

---

79. Exact versus approximate execution

ZQN must explicitly distinguish:

Exact
Approximate
Bounded
Statistical
Unsupported

For approximate results, expose:

requested model
realized model
approximation method
error bound
confidence
assumptions

No silent approximation.

---

80. Mathematical equivalence

Where multiple channel representations describe the same physical map, ZQN should support conversion and differential validation.

Examples:

Kraus
   ↕
Choi
   ↕
superoperator
   ↕
Pauli transfer

Equivalent representations should agree within declared numerical tolerances.

---

81. Testing architecture

Testing is part of ZQN's implementation contract, not a final phase.

Required categories:

unit
property
differential
determinism
scaling
compatibility
integration
fuzz/robustness where repository infrastructure supports it

---

82. Unit tests

Test every public mathematical invariant.

Examples:

probability bounds
distribution normalization
channel dimensions
channel validity
fault validity
composition
tensor products
calibration validity
serialization
deserialization
version compatibility

---

83. Property tests

Required properties include:

serialize(deserialize(x)) == canonical(x)

where supported.

Also:

identity ∘ x == x
x ∘ identity == x

where mathematically applicable.

And:

valid probability ∈ [0, 1]

plus appropriate channel invariants.

---

84. Differential tests

Where multiple mathematically equivalent representations exist:

Kraus
Choi
superoperator
Pauli transfer

must be compared against the same observable predictions within declared tolerance.

---

85. Determinism tests

The same:

seed
program
noise model
calibration
target
execution policy

must produce identical deterministic results.

Test across:

single-thread execution
parallel execution
different partitioning
different worker counts

where the relevant execution adapters exist.

---

86. Scaling tests

Never define architecture around:

MAX_QUBITS

Instead generate test sizes.

The test infrastructure must demonstrate that the same semantic code works as the resource count grows until the configured test resource budget is reached.

Scaling tests should vary:

resource count
operation count
fault count
correlation size
shot count
time steps
distribution size

independently.

---

87. Compatibility tests

Test:

old schema → current schema
current schema → current schema
unsupported schema → explicit error

No silent interpretation of incompatible serialized data.

---

88. Integration tests

Required integration boundaries:

ZQN ↔ quantum::ir
ZQN ↔ routing
ZQN ↔ scheduling
ZQN ↔ QEC
ZQN ↔ hardware
ZQN ↔ memory/state
ZQN ↔ benchmarking
ZQN ↔ runtime

Each integration test must verify ownership boundaries as well as functional behavior.

---

89. File completion contract

Every production ZQN source file must have a frozen contract before it is considered complete.

That contract must specify:

1. Ownership
2. Non-ownership
3. Public API
4. Dependencies
5. Consumers
6. Invariants
7. Error behavior
8. Resource behavior
9. Determinism behavior
10. Serialization behavior
11. Thread-safety behavior
12. Scalability behavior
13. Integration contract
14. Tests
15. Migration/compatibility behavior where applicable

A later implementation must not require reopening a completed foundational file merely because another downstream implementation was added.

---

90. Required source-file documentation

Production source files should document their contracts in Rust documentation comments.

Recommended pattern:

//! # Ownership
//!
//! This module owns ...
//!
//! # Non-ownership
//!
//! This module does not own ...
//!
//! # Invariants
//!
//! ...
//!
//! # Integration
//!
//! ...
//!
//! # Scalability
//!
//! ...
//!
//! # Determinism
//!
//! ...
//!
//! # Resource safety
//!
//! ...
//!
//! # Errors
//!
//! ...
//!
//! # Serialization
//!
//! ...
//!
//! # Testing
//!
//! ...

This is especially important because ZQN is intentionally divided into many independently maintainable modules.

---

91. Dependency direction

The preferred dependency direction is:

core
  ↓
probability
  ↓
channel
  ↓
fault
  ↓
noise
  ↓
operations
  ↓
calibration / characterization
  ↓
simulation / propagation
  ↓
target
  ↓
integration
  ↓
io

This is a conceptual layering rule, not a requirement that every module import every preceding layer.

Use traits and small interfaces to prevent unnecessary coupling.

---

92. Forbidden dependencies

ZQN must not depend directly on:

frontend AST implementations
vendor QPU APIs
routing implementations
scheduler implementations
QEC decoder implementations
benchmark implementations
UI
CLI
application code

Integration modules may depend on the contracts necessary to connect ZQN to those systems.

The mathematical core must remain independent.

---

93. QIR relationship

ZQN is not QIR.

The intended relationship is:

Zamani source
      ↓
Zamani quantum::ir
      ↓
ZQN
      ↓
target lowering
      ↓
QIR / backend representation

QIR is designed as a common intermediate representation between quantum programming languages and heterogeneous quantum processors, with target/profile capabilities separated from the representation itself.

ZQN complements that architecture by providing the physical uncertainty/noise model rather than replacing the program IR.

---

94. MLIR relationship

ZQN is not required to become an MLIR dialect.

If Zamani later interoperates with MLIR, the intended relationship is:

Zamani IR
   ↓
ZQN semantics
   ↓
Zamani/MLIR representation
   ↓
MLIR transformations
   ↓
QIR / LLVM / target lowering

This architecture follows the useful MLIR principle that transformations should operate through generic interfaces rather than hard-coded knowledge of every concrete operation. MLIR explicitly uses interfaces to decouple transformations and analyses from individual operation/dialect implementations.

---

95. Extensibility rule

Adding a new noise technology should not require modifying unrelated foundational abstractions.

For example, adding a future:

new physical channel

should normally require a new channel implementation and registration/integration where necessary, not modification of:

probability core
canonical qubit identity
runtime
routing
scheduler
QEC decoder

unless a genuine interface capability is missing.

This is the same architectural goal behind extensible compiler IR systems: new semantics should be introduced without turning every transformation into a collection of special cases.

---

96. Registration and discovery

Where ZQN supports extensible model families, discovery must be capability-based.

Prefer:

NoiseModelRegistry
ChannelRegistry
RepresentationRegistry

only where a registry is actually required.

Do not make registration dependent on global mutable state.

Registries should be explicitly constructed or owned by the appropriate context.

---

97. Caching

Caches must be keyed by semantic identity.

Never cache solely by:

model name

A valid cache identity may require:

model identity
configuration identity
calibration identity
target identity
schema/version identity
precision policy

---

98. Calibration cache

Calibration caches must respect:

resource scope
validity interval
target identity
calibration version
environment where semantically relevant

A stale calibration must not silently become current.

---

99. Provenance and reproducibility

A reproducible ZQN execution should be able to identify:

ZQN version
model version
model hash
configuration hash
calibration identity
target identity
seed policy
seed/root seed
numerical precision
execution policy
software version

This metadata must remain separate from the mathematical result where appropriate, but must be attachable to it.

---

100. Migration of the existing QEC noise system

Do not immediately delete the existing QEC noise implementation.

Migration order:

existing QEC noise
       ↓
ZQN-compatible adapter
       ↓
QEC consumes ZQN
       ↓
duplicate physical-noise semantics removed

The QEC subsystem should continue owning:

syndrome
decoding
correction
logical error analysis

while ZQN owns universal physical noise semantics.

This prevents a destructive rewrite.

---

101. Migration of noise-aware routing

Existing noise-aware routing should eventually consume:

ZQN noise profile
ZQN error estimate
ZQN fidelity estimate
ZQN duration cost
ZQN crosstalk information

Routing retains ownership of the routing decision.

---

102. Migration of scheduling

Scheduling should consume:

ZQN idle-noise model
ZQN duration-dependent error
ZQN crosstalk model
ZQN calibration validity

and retain ownership of temporal scheduling.

---

103. Migration of hardware adapters

Hardware adapters should translate physical provider information into:

target capabilities
calibration snapshots
observations

rather than embedding vendor-specific logic inside ZQN.

---

104. Production implementation order

The implementation order is intentionally bottom-up.

Stage 0 — Repository correctness

First:

remove/fix src/quantum/zqn/mod.rs 

and establish the correct:

src/quantum/zqn/mod.rs

Then verify:

cargo fmt
cargo check
cargo test

with the repository's existing configuration.

---

Stage 1 — Core

Complete independently:

core/error.rs
core/version.rs
core/ids.rs
core/limits.rs
core/metadata.rs
core/provenance.rs
core/capabilities.rs
core/context.rs

No downstream implementation should redefine their responsibilities.

---

Stage 2 — Probability

Complete:

probability/probability.rs
probability/bounds.rs
probability/distribution.rs
probability/categorical.rs
probability/continuous.rs
probability/statistics.rs

---

Stage 3 — Channel foundations

Complete:

channel/representation.rs
channel/kraus.rs
channel/choi.rs
channel/process_matrix.rs
channel/pauli.rs
channel/stochastic.rs
channel/lindblad.rs
channel/composition.rs

Then specialized channels:

thermal.rs
amplitude.rs
phase.rs
depolarizing.rs
generalized.rs

---

Stage 4 — Faults

Complete:

fault/fault.rs
fault/location.rs
fault/classification.rs
fault/correlated.rs
fault/leakage.rs
fault/erasure.rs
fault/loss.rs
fault/batch.rs

---

Stage 5 — Noise

Complete:

noise/model.rs
noise/specification.rs
noise/application.rs
noise/composition.rs
noise/correlation.rs
noise/temporal.rs
noise/spatial.rs
noise/crosstalk.rs
noise/drift.rs
noise/non_markovian.rs
noise/conditional.rs

---

Stage 6 — Operations

Complete:

operations/operation.rs
operations/gate.rs
operations/preparation.rs
operations/reset.rs
operations/measurement.rs
operations/idle.rs
operations/pulse.rs
operations/transport.rs

---

Stage 7 — Calibration

Complete:

calibration/snapshot.rs
calibration/parameter.rs
calibration/device.rs
calibration/gate.rs
calibration/readout.rs
calibration/measurement.rs
calibration/drift.rs
calibration/interpolation.rs
calibration/validation.rs

---

Stage 8 — Characterization

Complete:

characterization/experiment.rs
characterization/protocol.rs
characterization/observation.rs
characterization/estimator.rs
characterization/uncertainty.rs
characterization/tomography.rs
characterization/randomized_benchmarking.rs
characterization/process_characterization.rs

---

Stage 9 — Simulation

Complete:

simulation/engine.rs
simulation/sampler.rs
simulation/trajectory.rs
simulation/channel_engine.rs
simulation/monte_carlo.rs
simulation/deterministic.rs
simulation/reproducibility.rs

These must integrate with existing repository simulation contracts instead of creating a competing simulation engine.

---

Stage 10 — Propagation

Complete:

propagation/error_budget.rs
propagation/uncertainty.rs
propagation/fidelity.rs
propagation/bounds.rs
propagation/sensitivity.rs
propagation/accumulation.rs

---

Stage 11 — Target

Complete:

target/requirements.rs
target/capabilities.rs
target/compatibility.rs
target/lowering.rs
target/validation.rs

---

Stage 12 — Integration

Only after foundational contracts are stable:

integration/ir.rs
integration/routing.rs
integration/scheduling.rs
integration/qec.rs
integration/hardware.rs
integration/memory.rs
integration/benchmarking.rs
integration/runtime.rs

---

Stage 13 — I/O

Complete:

io/schema.rs
io/serialization.rs
io/deserialization.rs
io/canonical.rs
io/compatibility.rs

---

Stage 14 — Public module surface

Finally stabilize:

mod.rs
prelude.rs

The root module should expose only deliberate public APIs.

---

Stage 15 — Tests

Complete:

tests/unit/
tests/property/
tests/differential/
tests/determinism/
tests/scaling/
tests/compatibility/
tests/integration/
tests/fixtures/

---

105. Root "mod.rs"

"src/quantum/zqn/mod.rs" is the ZQN composition root.

It must:

- declare ZQN submodules;
- document ZQN ownership;
- document non-ownership;
- expose stable APIs;
- avoid implementation logic;
- avoid global state;
- avoid vendor logic;
- avoid duplicate IDs.

Conceptually:

pub mod core;
pub mod probability;
pub mod channel;
pub mod fault;
pub mod noise;
pub mod operations;
pub mod calibration;
pub mod characterization;
pub mod simulation;
pub mod propagation;
pub mod target;
pub mod integration;
pub mod io;

pub mod prelude;

The exact exports must match the files that actually exist and compile.

---

106. "prelude.rs"

"prelude.rs" should export only stable high-value APIs.

It must not become a dumping ground for implementation types.

Typical stable concepts:

ZqnContext
ZqnError
ZqnResult
ZqnLimits
ZqnCapabilities
NoiseModel
NoiseSpecification
QuantumChannel
Fault
Probability
Distribution
CalibrationSnapshot
NoiseObservation
ErrorBudget

---

107. Public API stability

Do not expose internal implementation details simply because another ZQN file needs them.

Internal dependencies should remain internal where possible.

The public API should represent stable concepts rather than implementation structures.

---

108. No hard-coded target names

The following must not appear in ZQN semantic code:

IBM
IonQ
Rigetti
Quantinuum
Google
AWS
Azure

unless such a name occurs only in documentation describing an external integration boundary.

ZQN's semantic implementation must remain vendor-neutral.

---

109. No hard-coded topology

Do not assume:

N = fixed number

or:

topology = fixed graph

Topology is supplied by the target/IR resource layer.

---

110. No hard-coded gate set

ZQN must not define the universe as:

H
X
Y
Z
CNOT

These may have specialized channel implementations, but the general noise abstraction must support arbitrary operations/resources.

---

111. No automatic measurement semantics

ZQN must not introduce hidden measurement simply because a simulation needs a classical result.

Measurement belongs to the program/execution semantics.

Noise may affect measurement when measurement exists.

It must not invent it.

---

112. No hidden behavior

ZQN must never silently:

- insert measurement;
- insert reset;
- add a noise channel;
- change a probability;
- change a calibration;
- approximate a model;
- truncate a distribution;
- reduce correlations;
- change a seed;
- change precision.

Every such transformation must be explicit and represented in the execution/provenance contract.

---

113. API contract for approximation

An approximation should be represented conceptually as:

requested semantics
        ↓
approximation policy
        ↓
realized semantics
        +
declared bound
        +
assumptions
        +
provenance

This makes scientific claims auditable.

---

114. API contract for deterministic stochastic behavior

The semantic model must not itself own hidden randomness.

The execution layer supplies deterministic randomness.

Therefore:

NoiseModel

describes behavior.

SamplingContext

determines reproducible realization.

This separation allows:

same model
different seed

to produce different valid stochastic realizations while:

same model
same context

produces the same deterministic realization.

---

115. API contract for streaming

Large-scale ZQN APIs should prefer streaming abstractions whenever materializing all results would be unnecessarily expensive.

Examples:

FaultStream
SampleStream
ObservationStream

must be considered where the data may be unbounded or very large.

Do not force:

Vec<T>

for every potentially massive result.

---

116. API contract for lazy evaluation

Expensive mathematical representations should be constructible lazily where practical.

For example:

lazy channel composition
lazy fault generation
lazy sample generation
lazy tensor representation

This is essential for large systems.

---

117. API contract for cancellation

Potentially long-running operations must cooperate with the repository's cancellation/execution context.

Examples:

Monte Carlo
tomography
channel conversion
large correlation generation
serialization
characterization

must not assume unlimited execution time.

---

118. API contract for resource accounting

When an operation may allocate substantially, the operation must consult the active resource policy before committing to an uncontrolled allocation.

The error should be explicit:

ResourceLimitExceeded

rather than:

panic

---

119. Panic policy

Production ZQN library code must not use panic-based validation for ordinary invalid external/user input.

Prefer:

Result<T, ZqnError>

for recoverable failures.

Assertions may be used only for genuine internal invariants that cannot be violated by valid public API use.

---

120. Compatibility philosophy

Backward compatibility must preserve semantic meaning.

A schema upgrade must never silently reinterpret a model in a way that changes its physical meaning.

If migration cannot be performed safely:

CompatibilityFailure

must be returned.

---

121. Documentation requirements

The ZQN directory should ultimately contain:

README.md
ARCHITECTURE.md
SEMANTICS.md
SCALABILITY.md
DETERMINISM.md
COMPATIBILITY.md
SECURITY.md

This README is the primary architecture contract.

The additional documents should specialize the topics rather than contradict this document.

---

122. Production acceptance criteria

ZQN is not production-ready merely because:

cargo test

passes.

Production readiness requires:

Architecture

- no circular ownership;
- canonical IR remains canonical;
- no duplicate qubit identity;
- no vendor dependency in semantic ZQN;
- stable integration boundaries.

Safety

- zero ZQN "unsafe";
- checked numerical behavior;
- no hidden global mutable state;
- explicit resource governance.

Scalability

- no semantic machine-size constants;
- arbitrary resource collections;
- streaming support where required;
- representation polymorphism;
- configurable runtime limits.

Determinism

- explicit seed/context;
- reproducible sampling;
- worker-count-independent deterministic execution.

Scientific correctness

- explicit mathematical invariants;
- uncertainty;
- provenance;
- approximation contracts;
- calibration validity.

Interoperability

- IR integration;
- QEC integration;
- routing integration;
- scheduling integration;
- hardware integration;
- memory integration;
- benchmarking integration;
- runtime integration.

Compatibility

- versioned schemas;
- canonical serialization;
- migration policy.

Testing

- unit;
- property;
- differential;
- deterministic;
- scaling;
- compatibility;
- integration;
- fuzz/robustness where supported.

---

123. Final architecture

The production ZQN boundary is:

                         ZAMANI PROGRAM
                               │
                               ▼
                         QUANTUM FRONTEND
                               │
                               ▼
                    ┌─────────────────────┐
                    │   quantum::ir       │
                    │                     │
                    │ canonical semantics │
                    │ canonical resources │
                    │ QubitId              │
                    │ PhysicalQubitId      │
                    └──────────┬──────────┘
                               │
                               ▼
                    ┌─────────────────────┐
                    │        ZQN          │
                    │                     │
                    │ Probability         │
                    │ Distribution        │
                    │ Channels            │
                    │ Faults              │
                    │ Noise Models        │
                    │ Correlation         │
                    │ Calibration         │
                    │ Characterization    │
                    │ Uncertainty         │
                    │ Provenance          │
                    │ Determinism         │
                    └──────────┬──────────┘
                               │
             ┌─────────────────┼─────────────────┐
             │                 │                 │
             ▼                 ▼                 ▼
          ROUTING          SCHEDULING            QEC
             │                 │                 │
             └─────────────────┼─────────────────┘
                               │
                               ▼
                         TARGET MODEL
                               │
                    ┌──────────┼──────────┐
                    │          │          │
                    ▼          ▼          ▼
                SIMULATOR     QPU      EMULATOR
                    │          │          │
                    └──────────┼──────────┘
                               │
                               ▼
                           RUNTIME
                               │
                               ▼
                         OBSERVATIONS
                               │
             ┌─────────────────┼─────────────────┐
             │                 │                 │
             ▼                 ▼                 ▼
       CHARACTERIZATION   BENCHMARKING       ANALYSIS
             │
             ▼
        CALIBRATION
             │
             └──────────────────────► ZQN

---

124. Fundamental invariants

The following are permanent ZQN architectural invariants.

Invariant 1

"quantum::ir" remains the canonical semantic quantum representation.

Invariant 2

ZQN does not define a competing generic "QubitId".

Invariant 3

"quantum::ir::qubit::{QubitId, PhysicalQubitId}" remain authoritative wherever those identities are applicable.

Invariant 4

ZQN has no semantic machine-size maximum.

Invariant 5

Resource limits are runtime/configuration policy, not physical-semantic constants.

Invariant 6

ZQN contains no vendor APIs.

Invariant 7

ZQN contains no hidden global RNG.

Invariant 8

Deterministic stochastic execution must be reproducible.

Invariant 9

ZQN contains no "unsafe".

Invariant 10

Approximations are explicit.

Invariant 11

Invalid numerical states are rejected.

Invariant 12

Calibration has validity/provenance.

Invariant 13

Noise and faults remain distinct concepts.

Invariant 14

ZQN does not own routing, scheduling, QEC decoding, or benchmarking methodology.

Invariant 15

Adding a new physical noise model must not require rewriting unrelated foundational modules.

Invariant 16

No file may depend on an implementation detail that violates its documented ownership boundary.

---

125. Relationship to external compiler architecture

The architecture deliberately follows several established principles from extensible compiler infrastructures.

MLIR explicitly provides interfaces so transformations can work generically across different operations and dialects without hard-coding every concrete implementation.

MLIR also treats dialects as extensible groupings of operations, types, and attributes, with explicit verification and modularity.

Its language reference also distinguishes human-readable, in-memory, and serialized representations and provides mechanisms for versioning and transformation.

ZQN applies the same broad architectural lesson without making ZQN itself an MLIR dialect:

stable semantic contract
        +
interfaces
        +
verification
        +
multiple representations
        +
explicit lowering
        +
versioned serialization

---

126. The ultimate Zamani contract

A Zamani quantum programmer should be able to write:

one program

without writing separate versions for:

small machine
large machine
different qubit counts
different topology
different hardware technology
different calibration
different noise conditions
different simulator
different QPU
distributed execution
future supported quantum technology

The compiler/runtime stack determines the realization.

The program expresses what.

The target expresses what is available.

ZQN expresses what physical uncertainty exists.

Routing expresses where.

Scheduling expresses when.

QEC expresses how faults are handled.

Hardware expresses how the target executes.

Runtime expresses how the execution is orchestrated.

Benchmarking expresses how performance and physical behavior are measured.

This separation is the foundation of scalable Zamani quantum computing.

---

127. Definition of done for the ZQN architecture

ZQN reaches architectural completion when:

✓ correct module path exists
✓ no trailing-space module path remains
✓ Rust 1.97/1.97.1 builds
✓ no unsafe code
✓ canonical quantum::ir identifiers are reused
✓ no duplicate QubitId exists
✓ no semantic machine-size maximum exists
✓ limits are explicit policies
✓ probability foundation is complete
✓ channel abstraction is representation-independent
✓ faults are independent of channels
✓ noise model is backend-independent
✓ correlations are arbitrary-size
✓ temporal noise exists
✓ spatial noise exists
✓ crosstalk exists
✓ non-Markovian representation exists
✓ calibration exists
✓ characterization exists
✓ deterministic sampling exists
✓ reproducibility exists
✓ uncertainty propagation exists
✓ target capability negotiation exists
✓ exact/approximate distinction exists
✓ IR integration exists
✓ routing integration exists
✓ scheduling integration exists
✓ QEC adapter exists
✓ hardware adapter exists
✓ memory integration exists
✓ benchmarking integration exists
✓ runtime integration exists
✓ versioned serialization exists
✓ provenance exists
✓ canonical serialization exists
✓ unit tests exist
✓ property tests exist
✓ differential tests exist
✓ determinism tests exist
✓ scaling tests exist
✓ compatibility tests exist
✓ integration tests exist
✓ resource exhaustion is governed
✓ external/untrusted input is validated
✓ no hidden global state exists
✓ no vendor logic exists in ZQN

---

128. Final rule

Never optimize the architecture around today's largest quantum machine.

Optimize the architecture around:

stable semantics
+
unbounded resource descriptions
+
capability negotiation
+
representation polymorphism
+
streaming
+
deterministic execution
+
explicit resource policy
+
formal validation
+
scientific reproducibility
+
clean integration boundaries

That is what allows ZQN to grow from the smallest quantum experiment to arbitrarily large quantum systems as hardware and available computational resources grow.

ZQN is therefore a physical-noise and uncertainty layer over Zamani's canonical quantum semantics—not a second IR, not a vendor abstraction, not a simulator, and not a QEC implementation.The external compiler architecture supports this direction: MLIR explicitly emphasizes extensible abstractions and interfaces so transformations don't need special cases for every concrete operation, while its IR model separates in-memory, textual, and serialized representations. 

