Worked for 16s

Yes. I checked the repository through GitHub rather than treating the earlier proposed tree as if it were still empty. The current repository already contains substantial ZQN structure, including core, probability, channel, fault, noise, operations, calibration, characterization, simulation, propagation, target, integration, and io; for example, the repository's current noise, fault, probability, calibration, simulation, and operations module boundaries already describe ZQN as the backend-independent noise/uncertainty layer.

I also verified the quantum composition boundary: quantum::ir is the canonical semantic boundary and quantum::ir::qubit::{QubitId, PhysicalQubitId} are the authoritative quantum identifiers.

Below is the complete proposed src/quantum/zqn/SEMANTICS.md. This version corrects an important issue in the earlier architecture: ZQN semantics must not merely describe “noise models”; it must define the mathematical and operational meaning of ideal quantum behavior + physical imperfections + uncertainty + realization + approximation + observation, while remaining independent of any particular simulator, QPU, gate set, qubit count, or vendor.

# ZQN Semantics

**Path:** `src/quantum/zqn/SEMANTICS.md`

**Subsystem:** Zamani Quantum Noise (ZQN)

**Status:** Production Architecture / Normative Semantic Contract

**Target Rust:** Rust 1.97 / Rust 1.97.1, Rust 2021, stable

**Safety:** `unsafe` is forbidden

---

# 1. Purpose

ZQN (Zamani Quantum Noise) defines the authoritative, backend-independent semantics for representing, validating, composing, applying, sampling, observing, estimating, and propagating physical imperfections and uncertainty affecting quantum computation.

ZQN is part of the Zamani quantum stack.

It is not the canonical quantum program representation.

The canonical semantic representation remains:

```text
crate::quantum::ir

ZQN answers a different question:

quantum::ir
    =
    What computation is being requested?

ZQN
    =
    What physical uncertainty, noise, fault,
    calibration variation, environmental effect,
    or imperfect realization affects that computation?

The distinction is fundamental.


---

2. Normative language

The words:

MUST

MUST NOT

REQUIRED

SHALL

SHALL NOT

SHOULD

SHOULD NOT

MAY


are normative.

When this document conflicts with an implementation shortcut, this document takes precedence unless the semantic contract is intentionally versioned and changed.


---

3. Architectural position

The authoritative quantum architecture is:

Zamani source
      │
      ▼
 quantum frontend
      │
      ▼
┌─────────────────────────┐
│      quantum::ir        │
│ canonical program       │
│ semantics               │
└────────────┬────────────┘
             │
             ├──────────────► algorithms
             │
             ├──────────────► optimization
             │
             ├──────────────► routing
             │
             ├──────────────► scheduling
             │
             └──────────────► ZQN
                                │
                    ┌───────────┼───────────┐
                    │           │           │
                    ▼           ▼           ▼
                 routing    scheduling     QEC
                    │           │           │
                    └───────────┼───────────┘
                                │
                                ▼
                         target/hardware
                                │
                    ┌───────────┼───────────┐
                    ▼           ▼           ▼
                 simulator     QPU       emulator
                    │           │           │
                    └───────────┼───────────┘
                                ▼
                             runtime
                                │
                                ▼
                           observations
                                │
                    ┌───────────┼───────────┐
                    ▼           ▼           ▼
             characterization benchmarking analysis
                    │
                    ▼
                calibration
                    │
                    └──────────────► ZQN

ZQN is therefore a cross-cutting physical-semantics subsystem.

It MUST NOT replace quantum::ir.


---

4. Core semantic separation

The following ownership is normative.

Subsystem	Owns

quantum::frontend	Source/external-format parsing
quantum::ir	Canonical quantum program semantics
quantum::algorithms	Algorithm construction
quantum::optimization	Semantics-preserving transformations
quantum::routing	Logical/physical placement
quantum::scheduling	Temporal ordering and scheduling
quantum::zqn	Noise, uncertainty, faults, channels, calibration semantics
quantum::error_correction	Encoding, syndrome, decoding, correction
quantum::memory	State/resource representation and memory management
quantum::hardware	Target capabilities, devices, providers, execution
quantum::benchmarking	Benchmark experiments, metrics, reporting
runtime/backend	Actual execution


No subsystem may silently acquire another subsystem's semantic ownership.


---

5. Canonical quantum identities

ZQN MUST NOT create a second semantic identity for quantum resources already defined by the canonical IR.

When qubit identity is required, ZQN MUST use:

crate::quantum::ir::qubit::QubitId

and, where physical identity is required:

crate::quantum::ir::qubit::PhysicalQubitId

ZQN MUST NOT introduce:

zqn::QubitId
zqn::PhysicalQubitId

as competing semantic identities.

ZQN-specific identities MAY exist where the identity represents a ZQN-owned object rather than a quantum resource.

Examples:

NoiseModelId
NoiseSnapshotId
ChannelId
FaultId
CalibrationId
CharacterizationId
ExperimentId

These identities identify ZQN objects, not qubits.


---

6. Fundamental semantic model

A quantum program does not become a physical execution merely because an ideal operation is specified.

ZQN models the transition:

ideal semantic operation
        │
        ▼
target realization
        │
        ▼
physical realization
        │
        ├── deterministic imperfections
        ├── stochastic imperfections
        ├── environmental effects
        ├── calibration uncertainty
        ├── temporal effects
        ├── spatial effects
        ├── correlations
        └── measurement uncertainty
        │
        ▼
physical outcome distribution

The central semantic abstraction is therefore:

IdealOperation
      +
PhysicalContext
      +
NoiseSpecification
      +
CalibrationState
      +
ExecutionContext
      =
PhysicalRealization

The physical realization MAY be represented as:

a quantum channel;

a stochastic process;

a fault realization;

a trajectory;

a pulse-level perturbation;

a measurement transformation;

an analog evolution;

a transport process;

a distributed communication process;

an explicitly approximate model;

an observation distribution.



---

7. What ZQN means by noise

Noise is any physical or modeled deviation from the intended quantum semantics that is relevant to execution, characterization, analysis, optimization, or fault tolerance.

Noise includes, but is not limited to:

stochastic error;

coherent error;

systematic error;

calibration error;

drift;

decoherence;

thermal effects;

measurement error;

preparation error;

reset error;

idle error;

pulse error;

transport error;

leakage;

loss;

erasure;

crosstalk;

spatial correlation;

temporal correlation;

non-Markovian behavior;

environmental coupling;

parameter uncertainty;

correlated faults;

state-dependent error;

history-dependent error.


Noise is not synonymous with random probability.

A deterministic calibration offset can be noise.

A coherent over-rotation can be noise.

A correlated environmental process can be noise.

A leakage event can be a fault realization of a noise process.


---

8. Noise is not the same as a fault

ZQN distinguishes:

Noise model
    =
    rule/process describing physical imperfections

Fault
    =
    realized or explicitly represented deviation/event

For example:

Depolarizing channel
        │
        ├── no fault
        ├── X fault
        ├── Y fault
        └── Z fault

Another example:

Leakage process
        │
        └── leakage event

A noise model MAY produce faults.

A fault MAY be represented independently for QEC or testing.

A channel does not need to expose an individual fault realization.


---

9. Channels

A quantum channel is a semantic transformation of quantum state or process information.

A channel MUST NOT be defined as merely:

gate + probability

The abstraction must be general enough for:

preparation channels;

gate channels;

idle channels;

measurement channels;

reset channels;

transport channels;

environmental channels;

correlated channels;

time-dependent channels;

continuous-time dynamics;

non-Markovian processes;

generalized process transformations.


A channel MAY be represented using:

Kraus operators;

Choi representation;

superoperator;

Pauli transfer representation;

stochastic representation;

Lindblad generator;

process matrix;

symbolic representation;

sparse representation;

tensor representation;

trajectory representation;

target-native representation.


No representation is universally mandatory.


---

10. Channel validity

Where a representation claims to represent a quantum channel, its semantic validity MUST be established according to the representation.

For a standard quantum channel, the implementation MUST validate the relevant mathematical properties, including where applicable:

complete positivity
trace preservation
finite numerical values
dimension compatibility
domain compatibility
codomain compatibility

A representation MAY intentionally represent:

a non-trace-preserving quantum operation;

a conditional branch;

an instrument element;

a postselected process;

a subnormalized state transformation.


Such objects MUST NOT be silently labeled as ordinary trace-preserving channels.

Their semantic category MUST be explicit.


---

11. Measurement semantics

Measurement is not merely a noisy gate.

ZQN MUST distinguish:

ideal measurement semantics
        +
measurement noise
        +
readout transformation
        +
measurement backaction

Possible effects include:

assignment error;

asymmetric readout;

correlated readout;

state-dependent readout;

detector loss;

detector dark events;

measurement backaction;

temporal detector drift;

context-dependent readout.


The ideal measurement remains owned by quantum::ir.

ZQN owns the physical imperfection model.


---

12. Preparation semantics

Preparation noise describes deviation between:

requested initial state

and:

physically prepared state

Preparation MAY involve:

thermal population;

state contamination;

coherent preparation error;

stochastic preparation error;

leakage;

loss;

correlated preparation effects.


Reset MUST be treated as its own semantic operation.

Reset MUST NOT automatically be reduced to:

measure + conditional gate

unless the target explicitly defines that equivalence and the transformation is proven valid.


---

13. Idle semantics

An idle period is a physical event.

Therefore:

idle(resource, duration)

MUST be representable independently of gate operations.

Idle noise MAY depend on:

duration;

absolute time;

relative time;

resource;

temperature;

calibration;

environment;

previous operations;

neighboring resources;

correlation history.


This is essential for scheduling integration.


---

14. Pulse semantics

ZQN MAY represent pulse-level imperfections without requiring the entire Zamani program to be pulse-level.

The semantic hierarchy may be:

Zamani source
    ↓
canonical IR
    ↓
logical operation
    ↓
scheduled operation
    ↓
pulse realization
    ↓
pulse noise

Pulse noise MUST NOT leak pulse-specific implementation assumptions into the canonical IR.


---

15. Transport semantics

Quantum computation is not restricted to stationary qubits.

ZQN MUST permit noise on:

transport;

shuttling;

movement;

optical transmission;

communication links;

distributed quantum resources;

state transfer;

teleportation resources.


Transport MAY produce:

loss;

decoherence;

delay;

phase error;

amplitude error;

correlated errors;

link-specific uncertainty.



---

16. Resource abstraction

ZQN MUST NOT assume that every future quantum technology consists only of qubits and gates.

Noise locations SHOULD therefore be representable generically.

Conceptually:

NoiseLocation
├── Qubit
├── Qudit
├── Mode
├── BosonicMode
├── LogicalResource
├── PhysicalResource
├── Operation
├── Measurement
├── Preparation
├── Reset
├── Pulse
├── Channel
├── TransportLink
├── TimeInterval
└── CompositeResource

The concrete Rust representation may evolve, but the semantic requirement is fixed:

> ZQN noise attaches to semantic physical resources, not merely to gate indices.




---

17. Technology neutrality

ZQN MUST be capable of representing noise affecting:

superconducting systems;

trapped ions;

neutral atoms;

photonic systems;

spin systems;

bosonic systems;

continuous-variable systems;

analog quantum systems;

annealing systems;

measurement-based systems;

distributed quantum systems;

fault-tolerant logical systems;

future quantum modalities.


ZQN MUST NOT contain vendor-specific semantics.

There MUST NOT be a semantic dependency on:

IBM
IonQ
Rigetti
Quantinuum
Google
AWS
Azure
NVIDIA
or any other vendor

Vendor adapters belong outside ZQN.


---

18. Write once, scale everywhere

ZQN SHALL have no semantic upper bound on:

qubit count;

physical resource count;

logical resource count;

operation count;

circuit depth;

topology size;

correlation-set size;

execution duration;

distributed-node count;

machine size.


This means:

N is data.

It must never mean:

N is a compile-time architectural constant.

The implementation MUST NOT contain semantic branches such as:

if qubits == 5 { ... }
if qubits == 20 { ... }
if qubits == 127 { ... }

or vendor-equivalent branches.


---

19. Infinity and resource availability

"Scale to infinity" means:

> ZQN imposes no artificial finite semantic machine-size ceiling.



It does NOT mean that:

RAM is infinite;

disk is infinite;

CPU is infinite;

GPU memory is infinite;

a QPU has infinite resources;

a simulator can materialize an infinite state vector.


Therefore:

semantic capacity
    ≠
physical resource capacity

A computation is valid if its semantics can be represented and its required execution can be supported by the available resources.


---

20. Resource policies

Resource limits MUST be policy, not semantics.

Examples:

max_memory_bytes
max_operations
max_faults
max_samples
max_distribution_entries
max_tensor_elements
max_execution_time

These MAY be finite.

They MUST be:

explicit;

inspectable;

configurable;

scoped;

propagated through execution;

distinguishable from semantic validity.


A resource failure MUST NOT mean:

"the quantum program is semantically invalid"

It means:

"this realization cannot be performed under the current resource policy"


---

21. Existing safety limits

Existing subsystems may contain safety boundaries such as:

MAX_QUBIT_INDEX
MAX_CORRELATED_QUBITS
MAX_FAULTS_PER_BATCH

These MUST NOT become semantic ZQN limits.

Where such constants are migrated into ZQN:

semantic model
      │
      ▼
resource policy
      │
      ▼
configured safety limit

is required.

A target with more resources must not require changing ZQN source code.


---

22. Lazy and streaming semantics

Large systems MUST NOT require materializing every object simultaneously.

APIs SHOULD support:

iterators;

streaming;

lazy generation;

bounded batches;

chunked processing;

incremental observations;

incremental statistics;

external storage;

distributed processing.


For example, a fault process should be capable of:

FaultStream

rather than requiring:

Vec<Fault>

for the entire computation.


---

23. Cancellation

Potentially unbounded work MUST support cancellation.

Cancellation belongs to the execution context/policy.

A cancelled operation MUST return an explicit cancellation result/error.

Cancellation MUST NOT be confused with:

mathematical invalidity;

unsupported semantics;

numerical failure.



---

24. Numerical semantics

ZQN MUST explicitly distinguish:

exact
approximate
bounded
statistical
unknown
unsupported

Numerical values MUST NOT silently change semantic category.


---

25. Non-finite values

Production ZQN MUST reject invalid numerical values where the mathematical domain requires finite values.

Examples:

NaN
+∞
-∞

MUST NOT silently become:

0
1
MAX
abs(value)

unless an explicitly documented conversion policy says so.


---

26. Probability semantics

Probability values MUST have explicit semantic meaning.

Where a probability is required:

0 <= p <= 1

MUST hold.

Invalid probability values MUST produce validation errors.

The implementation MUST distinguish:

probability
probability estimate
probability interval
probability distribution
probability density
likelihood
weight

These are not interchangeable.


---

27. Probability normalization

A probability distribution MUST define its normalization semantics.

For a finite categorical distribution:

sum(p_i) = 1

within an explicitly declared numerical tolerance where floating-point representation is used.

A distribution MUST NOT silently normalize invalid input unless the caller explicitly requests a normalization policy.

Implicit normalization can conceal scientific errors.


---

28. Continuous distributions

Continuous probability models MAY be represented by:

analytical distributions;

numerical distributions;

samples;

parameterized functions;

bounded intervals;

empirical distributions.


The semantic API MUST NOT assume a fixed distribution family.


---

29. Uncertainty semantics

ZQN distinguishes:

uncertainty

from:

noise

Noise describes physical deviation.

Uncertainty describes incomplete knowledge of a value/model.

Examples:

measured gate error = 0.0012
uncertainty = ±0.0001

The uncertainty itself is semantic data.


---

30. Sources of uncertainty

Uncertainty MAY originate from:

measurement statistics;

calibration uncertainty;

parameter estimation;

environmental variation;

finite sample size;

model uncertainty;

numerical approximation;

unknown correlation;

temporal drift.


The source MUST be preserved where relevant.


---

31. Approximation semantics

ZQN MUST NEVER silently replace an unsupported exact model with an approximation.

Every approximation MUST identify:

requested semantics
realized semantics
approximation method
assumptions
error bound or tolerance
confidence, where statistical

Supported approximation categories include:

Exact
Approximate
Bounded
Statistical
Unsupported


---

32. Error contracts

An approximation MAY specify:

absolute error
relative error
trace-distance bound
fidelity bound
total-variation bound
confidence interval
statistical confidence
numerical tolerance

The metric MUST be appropriate to the semantic object.

A generic "accuracy" number is insufficient.


---

33. Correlation semantics

Noise MUST NOT be assumed independent unless independence is explicitly part of the model.

ZQN MUST support:

independent noise
pairwise correlation
higher-order correlation
arbitrary correlation domains
spatial correlation
temporal correlation
space-time correlation
environment-mediated correlation


---

34. Correlated fault representation

The number of correlated resources is data.

The architecture MUST NOT create:

TwoQubitFault
ThreeQubitFault
FourQubitFault

as the fundamental model.

Instead:

CorrelatedFault {
    resources: ...
    correlation: ...
}

The resource collection may contain any supported finite number of resources subject only to execution/resource policy.


---

35. Spatial correlation

Spatial correlation MAY depend on:

graph distance;

physical distance;

connectivity;

resource type;

environmental domains;

arbitrary correlation functions.


ZQN MUST NOT hard-code a topology.

Topology is supplied by the target/hardware layer.


---

36. Temporal correlation

ZQN MUST support:

stationary noise;

nonstationary noise;

time-varying parameters;

drift;

temporal covariance;

history-dependent processes;

memory kernels.


Time MUST NOT be represented solely as an integer gate index.

Where timing is meaningful, ZQN should consume canonical scheduling/timing information.


---

37. Non-Markovian semantics

A non-Markovian model may depend on historical execution.

Conceptually:

current state
+
environment state
+
history
+
current operation
=
next physical realization

ZQN MUST NOT force such a process into independent per-operation probabilities.

A non-Markovian model MAY maintain an explicit environment/process state through an execution context.

That state MUST be explicit.

It MUST NOT be hidden global mutable state.


---

38. Crosstalk

Crosstalk represents unwanted physical interaction between resources caused by concurrent or nearby activity.

Crosstalk is not routing.

Routing answers:

Where should logical resources be placed?

ZQN answers:

What physical interaction/noise occurs given those resources and operations?

Routing MAY query ZQN to estimate crosstalk cost.

ZQN MUST NOT implement the routing algorithm.


---

39. Coherent errors

ZQN MUST support deterministic/coherent imperfections such as:

over-rotation;

under-rotation;

phase offset;

Hamiltonian mismatch;

systematic pulse distortion.


Coherent error MUST NOT be automatically converted into stochastic Pauli noise.

Such a conversion is an approximation and must be declared as such.


---

40. Stochastic errors

ZQN supports stochastic processes such as:

bit flip;

phase flip;

depolarization;

amplitude damping;

thermal relaxation;

stochastic leakage;

stochastic loss;

readout assignment errors.


Specific models are specializations.

They do not define the entire ZQN semantic universe.


---

41. Leakage

Leakage is a physical transition outside the intended computational subspace.

It MUST NOT be represented merely as:

X
Y
Z

unless an explicit approximation says so.

The leakage subsystem MUST identify:

computational subspace
leakage destination/domain
transition semantics
recovery/reset behavior
measurement behavior

No ZQN-specific qubit identifier is permitted.


---

42. Erasure

Erasure represents loss of usable information with an identifiable erasure event or erasure flag when such a flag is part of the model.

Erasure is distinct from:

Pauli error;

generic depolarization;

leakage;

physical loss.


The semantic distinction MUST be preserved because QEC can exploit erasure information.


---

43. Loss

Loss represents disappearance or failure of a quantum resource/state from the intended computational process.

Loss MAY occur during:

storage;

transport;

measurement;

communication;

preparation;

execution.


Loss MUST be distinguishable from an ordinary state transformation when the physical model requires that distinction.


---

44. Conditional noise

Noise MAY depend on:

operation;

state;

measurement result;

previous outcome;

time;

calibration;

environment;

resource;

concurrent operations;

execution context.


Conditional semantics MUST be explicit.

A conditional model MUST NOT secretly depend on global process state.


---

45. Dynamic circuits

Dynamic circuits can change future operations based on measurements.

ZQN MUST therefore support noise whose realization depends on:

branch
measurement result
classical condition
operation sequence
execution history

ZQN must consume the canonical IR's dynamic-control semantics rather than inventing a second dynamic-circuit representation.


---

46. Calibration semantics

Calibration describes the current estimated physical behavior of a target/resource.

A calibration snapshot SHOULD be:

immutable
versioned
time-aware
scoped
provenance-aware
uncertainty-aware

A calibration MUST identify its applicability.


---

47. Calibration validity

A calibration MAY have:

start time
end time
resource scope
operation scope
environmental assumptions
confidence
uncertainty

A calibration outside its validity interval MUST NOT silently be treated as current.


---

48. Calibration provenance

Every physically meaningful calibration should be traceable to:

source
measurement/experiment
timestamp
software/schema version
target identity
resource scope
estimation procedure
uncertainty

This enables scientific reproducibility.


---

49. Drift

Drift describes evolution of physical parameters over time.

Drift MAY be:

deterministic;

stochastic;

piecewise;

continuous;

environment-dependent;

resource-dependent.


Drift MUST NOT be implemented as a hidden mutable singleton.


---

50. Characterization

Characterization answers:

What physical behavior was observed?

ZQN characterization may include:

process tomography;

state tomography;

randomized benchmarking;

measurement characterization;

calibration experiments;

drift characterization;

leakage characterization;

transport characterization.


Characterization produces observations and estimates.

It MUST preserve uncertainty.


---

51. Observation versus model

ZQN MUST distinguish:

Observation

from:

Model

An observation is measured data.

A model is an interpretation/representation of physical behavior.

Example:

observations
    ↓
estimator
    ↓
noise model

The model MUST NOT erase the provenance of the observations that produced it.


---

52. Estimation

An estimator transforms observations into model parameters or distributions.

An estimator MUST define:

input observations;

estimator method;

assumptions;

output;

uncertainty;

convergence/validity conditions.


A numerical estimate MUST NOT be presented as an exact physical truth.


---

53. Simulation semantics

Simulation is an execution consumer of ZQN.

ZQN defines the physical semantics.

A simulator decides how to numerically realize those semantics.

Therefore:

ZQN channel
    ≠
simulation engine

and:

ZQN noise model
    ≠
specific simulator implementation


---

54. Existing simulation integration

The existing simulation architecture provides execution abstractions.

ZQN simulation adapters MUST consume the existing simulation contracts rather than introduce a competing simulation engine abstraction.

In particular, existing concepts such as:

SimulationExecutor
SimulationCoordinates
SimulationOperation
SimulationStepOutcome

remain owned by the simulation subsystem.

ZQN's:

simulation/deterministic.rs
simulation/reproducibility.rs

must adapt and provide ZQN semantics around those existing contracts rather than duplicate them.


---

55. Deterministic stochastic execution

Stochastic execution MUST be reproducible when deterministic mode is requested.

The caller MUST provide the root seed or deterministic seed material through the execution context/policy.

ZQN MUST NOT use:

thread_rng()

or an equivalent hidden/global RNG for semantic sampling.


---

56. Reproducibility context

A reproducibility context SHOULD incorporate stable identities such as:

root seed
program identity
noise-model identity
calibration identity
target identity
shot identity
operation identity
resource identity
sample/partition identity

The exact derivation belongs to:

quantum::zqn::simulation::reproducibility

and MUST be centralized.

Individual noise models MUST NOT invent competing seed derivation algorithms.


---

57. Parallel determinism

Under deterministic execution:

1 worker
8 workers
64 workers
distributed workers

MUST produce semantically equivalent deterministic results for the same declared execution context.

Worker scheduling MUST NOT change the random stream semantics.

Therefore ZQN randomness must be derived from stable coordinates, not worker-local mutable RNG order.


---

58. Distributed determinism

Distributed execution MAY derive deterministic domains from:

global seed
node identity
partition identity
resource identity
operation identity
shot identity

The derivation must be independent of:

network timing;

thread scheduling;

process scheduling;

task ordering.



---

59. Seed semantics

A seed identifies deterministic randomness.

A seed MUST NOT be interpreted as:

a global mutable RNG state

Instead:

seed + semantic coordinates
    =
deterministic random domain

This permits parallel and distributed reproducibility.


---

60. Error propagation

ZQN MUST distinguish:

semantic invalidity
unsupported semantics
target incompatibility
resource exhaustion
numerical failure
cancellation
serialization failure
determinism failure

These MUST NOT collapse into one generic error when callers need to react differently.


---

61. Resource failure

If a computation cannot be completed because of:

memory
time
sample budget
tensor size
allocation budget
fault budget

the result MUST identify this as a resource-policy failure.

It MUST NOT be reported as:

invalid quantum semantics


---

62. Target compatibility

ZQN MUST distinguish:

model is valid

from:

target can realize model exactly

These are different questions.

For example:

Requested:
    correlated non-Markovian noise

Target:
    independent Markovian channel support

The target is incompatible with the exact requested semantics.

ZQN MUST return incompatibility unless an explicit approximation policy authorizes conversion.


---

63. Target capability negotiation

The target subsystem supplies capabilities.

ZQN supplies requirements.

Conceptually:

ZQN requirements
        │
        ▼
target capabilities
        │
        ▼
compatibility analysis
        │
        ├── exact
        ├── approximable
        └── unsupported

This is the foundation of write-once/scale-everywhere execution.


---

64. Lowering semantics

Lowering transforms an accepted target-independent description into a target-compatible realization.

Lowering MUST preserve declared semantics.

If exact preservation is impossible, the result MUST explicitly state:

approximation
error bound
assumptions


---

65. Routing integration

quantum::zqn::integration::routing provides noise information to routing.

Examples include:

gate error
readout error
idle error
transport error
crosstalk
calibration uncertainty
expected fidelity
duration-dependent error
correlation cost

Routing remains responsible for choosing placement.

ZQN remains responsible for describing the physical consequences.


---

66. Scheduling integration

Scheduling can query ZQN using concepts equivalent to:

noise(resource, operation, duration, context)

Scheduling MAY use this information to optimize:

duration
fidelity
decoherence
crosstalk
calibration validity
transport cost

ZQN does not own scheduling policy.


---

67. QEC integration

ZQN is the canonical physical-noise semantic layer.

QEC owns:

code definitions;

encoding;

syndrome extraction;

syndrome processing;

decoding;

correction;

logical fault analysis.


QEC MUST consume ZQN rather than maintain a competing universal physical-noise ontology.


---

68. Migration from existing QEC noise

Existing:

quantum::error_correction::noise

functionality MUST NOT simply be copied into ZQN.

Migration should be:

existing QEC physical-noise implementation
              │
              ▼
ZQN semantic model
              │
              ▼
QEC adapter
              │
              ▼
QEC-specific fault representation

The adapter translates ZQN physical realizations into the representation required by QEC.

This prevents:

ZQN::Probability
QEC::Probability

ZQN::Fault
QEC::Fault

ZQN::NoiseModel
QEC::NoiseModel

from becoming permanently divergent semantic systems.


---

69. Memory integration

The memory subsystem owns state/resource representation.

ZQN supplies:

channel application
fault application
noise realization
transition semantics

Memory MUST NOT redefine the mathematical meaning of ZQN channels.

The direction is:

ZQN
 │
 ▼
memory transition/application interface
 │
 ▼
memory/state representation


---

70. Hardware integration

Hardware owns:

provider integration;

target discovery;

device identity;

capabilities;

native operations;

execution;

hardware lifecycle.


Hardware supplies ZQN with abstract information such as:

TargetCapabilities
CalibrationSnapshot
ObservedNoise
ExecutionMetadata

ZQN MUST NOT call vendor APIs.


---

71. Vendor isolation

The following MUST NOT occur inside ZQN:

vendor credentials
vendor SDK types
vendor network calls
vendor job submission
vendor device discovery
vendor provider-specific execution logic

Vendor adapters belong in the hardware/backend layer.


---

72. Benchmarking integration

Benchmarking consumes ZQN observations and models.

Possible flow:

benchmark workload
       │
       ▼
execution
       │
       ▼
raw observations
       │
       ▼
ZQN characterization
       │
       ▼
noise estimate
       │
       ▼
benchmark metrics

ZQN MUST NOT become dependent on the implementation of benchmarking.


---

73. Error budgets

ZQN can provide error budgets describing:

expected error
allowed error
uncertainty
dominant contributors
accumulation
sensitivity

An error budget MUST identify the metric in which the error is expressed.


---

74. Error accumulation

ZQN MUST NOT assume:

total_error = sum(individual_errors)

unless the mathematical model justifies that approximation.

Accumulation may depend on:

correlation;

coherent effects;

temporal effects;

interference;

cancellation;

nonlinear dynamics;

target topology.



---

75. Fidelity semantics

ZQN MAY expose multiple fidelity/error metrics.

Examples:

state fidelity;

process fidelity;

average gate fidelity;

entanglement fidelity;

trace-distance bounds;

diamond-distance bounds where computable;

total variation distance for classical distributions.


A metric MUST identify its mathematical definition.

No generic field called merely:

accuracy

should be used where a scientific metric is required.


---

76. Sensitivity semantics

Sensitivity analysis determines how strongly an output metric depends on physical parameters.

For example:

result sensitivity
        │
        ├── gate error
        ├── readout error
        ├── idle time
        ├── calibration parameter
        └── crosstalk

Sensitivity belongs to ZQN propagation analysis.

It does not change the canonical program semantics.


---

77. Representation polymorphism

ZQN MUST NOT require one universal numerical representation.

Depending on the problem and available resources, the physical model MAY be represented as:

exact
symbolic
dense
sparse
tensorized
stochastic
trajectory
sampled
analytical
hardware-native

The choice is an implementation/target decision provided semantic equivalence or approximation guarantees are preserved.


---

78. Representation selection

Representation selection MUST consider:

target capabilities;

available memory;

available compute;

requested precision;

required exactness;

structure/sparsity;

correlation;

system size;

execution mode.


A large system MUST NOT be forced into a representation whose memory requirement grows exponentially if a valid alternative representation exists.


---

79. No state-vector assumption

ZQN MUST NOT assume that the entire quantum state can be materialized.

The architecture must support:

local representations
tensor representations
trajectory representations
sampled representations
operator representations
hardware execution

This is necessary for scaling.


---

80. No fixed gate-set assumption

ZQN MUST NOT define its semantics around:

H
X
Y
Z
CNOT

or any other fixed gate list.

Those may be specialized channel/noise cases.

The canonical operation semantics come from quantum::ir.


---

81. No fixed arity assumption

A ZQN operation/noise process MAY involve:

one resource
two resources
N resources
a resource set
a spatial domain
a temporal interval
a distributed collection

Arity is data.


---

82. Analog semantics

For analog systems, ZQN MAY represent:

Hamiltonian uncertainty
continuous-time noise
control error
environment coupling
time-dependent perturbation

ZQN MUST NOT require conversion to a gate sequence merely to express physical noise.


---

83. Annealing semantics

For annealing/optimization systems, ZQN MAY represent:

control uncertainty;

thermal effects;

schedule perturbation;

coupling uncertainty;

readout errors;

transition errors.


The semantic model must not assume circuit gates.


---

84. Continuous-variable and bosonic semantics

ZQN MUST allow noise on:

modes;

bosonic states;

continuous variables;

loss channels;

phase-space processes;

Gaussian processes;

non-Gaussian processes where supported.


The mathematical representation may differ from qubit channels.


---

85. Fermionic semantics

ZQN MAY represent noise associated with:

fermionic modes;

particle loss;

mode decoherence;

Hamiltonian uncertainty;

transport.


It must not force these systems into a qubit-specific semantic API.


---

86. Measurement-based semantics

For measurement-based quantum computation, noise may affect:

resource state
measurement
measurement basis
classical feed-forward
photonic transport
detectors

ZQN must support these independently of ordinary gate circuits.


---

87. Distributed quantum semantics

Distributed systems may introduce:

network loss
latency
decoherence during transport
entanglement-generation failure
link correlation
node correlation
clock uncertainty

ZQN MUST permit distributed resources and links to participate in noise semantics.


---

88. Logical-level noise

ZQN MAY represent logical noise after QEC.

Logical noise MUST be distinguishable from physical noise.

For example:

physical noise
      ↓
QEC
      ↓
logical noise

The logical model may be derived from characterization or simulation.


---

89. Fault-tolerant semantics

ZQN does not own the QEC code.

It provides the physical and logical noise semantics consumed by QEC.

This allows:

same physical model
       ↓
different QEC codes

without changing the physical noise definition.


---

90. Provenance

Every major ZQN semantic object SHOULD support provenance.

Provenance can include:

origin
creator
source dataset
experiment
calibration
model version
software version
schema version
timestamp
target
resource scope

Provenance MUST NOT change the mathematical semantics of an object.


---

91. Canonical identity

Objects that participate in caching, reproducibility or distributed execution SHOULD have canonical identities.

Conceptually:

identity =
hash(
    canonical semantic representation
    +
    schema version
    +
    relevant configuration
)

A name alone is not an identity.


---

92. Canonicalization

Canonicalization MUST produce deterministic representation independent of:

hash-map iteration order;

thread order;

allocation order;

worker scheduling;

platform-specific incidental ordering.


Canonicalization belongs to:

quantum::zqn::io::canonical

not to every individual semantic module.


---

93. Serialization

Serialization MUST represent semantic data, not Rust implementation layout.

Therefore:

Rust struct layout

MUST NOT become the external schema.

Serialization MUST be versioned.

Deserialization MUST validate semantic invariants.

Untrusted serialized data MUST be treated as untrusted input.


---

94. Serialization compatibility

Schema compatibility MUST distinguish:

backward compatible
forward compatible
migration required
unsupported

A newer schema MUST NOT be silently interpreted as an older schema if semantics could change.


---

95. Deterministic serialization

Equivalent semantic objects MUST serialize canonically to equivalent canonical representations.

This enables:

hashing
cache keys
reproducibility
distributed execution
scientific provenance


---

96. Thread safety

ZQN semantic objects SHOULD be immutable after construction where practical.

Objects that can safely be shared SHOULD implement:

Send
Sync

where the semantics permit it.

Thread safety MUST NOT rely on:

global mutable state


---

97. Global-state prohibition

ZQN MUST NOT contain hidden global mutable semantic state.

Forbidden patterns include:

GLOBAL_NOISE_MODEL
GLOBAL_RNG
GLOBAL_CALIBRATION
GLOBAL_TARGET
GLOBAL_LIMITS
GLOBAL_SIMULATION_STATE

Execution context must be explicit.


---

98. Caching

Caching MAY be used for performance.

Cache keys MUST include every semantic input that affects the result.

A cache MUST NOT key solely on:

model name

When relevant, keys must incorporate:

model identity
configuration
calibration identity
target capability identity
precision
resource policy
schema version


---

99. Security

ZQN processes potentially untrusted data.

Security requirements include protection against:

allocation bombs;

pathological dimensions;

enormous distributions;

enormous correlation structures;

malformed serialized models;

NaN/Infinity injection;

integer overflow;

floating-point instability;

malicious calibration data;

pathological generators;

nonterminating computation;

excessive sampling requests.



---

100. Checked arithmetic

Resource and dimension calculations MUST use checked arithmetic where overflow is possible.

Expressions such as:

rows * columns

MUST NOT overflow silently.

This applies especially to:

tensor dimensions;

matrix sizes;

sample counts;

fault counts;

byte calculations;

index calculations.



---

101. Memory safety

ZQN MUST use safe Rust.

The entire subsystem MUST compile with:

#![forbid(unsafe_code)]

or an equivalent workspace-level enforcement.

No unsafe implementation is permitted as an optimization shortcut.


---

102. Rust compatibility

The implementation MUST remain compatible with:

Rust 1.97
Rust 1.97.1
Rust 2021
stable toolchain

No nightly-only feature may be required.


---

103. API design

Public APIs SHOULD be small and semantic.

The principal concepts are:

ZqnContext
ZqnLimits
ZqnCapabilities

NoiseModel
NoiseSpecification
NoiseApplication

QuantumChannel
ChannelRepresentation

Fault
FaultBatch

Probability
Distribution

CalibrationSnapshot
NoiseObservation
Characterization

ErrorBudget
Uncertainty

TargetNoiseRequirements
TargetNoiseCapabilities

Implementation-specific structures SHOULD remain private until their public stability is justified.


---

104. Core context semantics

core/context.rs provides execution context shared across ZQN operations.

It MAY contain references to:

resource policy
target capabilities
calibration snapshot
determinism policy
provenance
cancellation
precision policy

It MUST NOT become a global service locator.

Every dependency that affects semantics must be explicit.


---

105. Limits semantics

core/limits.rs owns resource policies.

A missing limit means:

no ZQN-imposed limit

not:

infinite physical resources

Examples:

max_operations: Option<u64>
max_faults: Option<u64>
max_memory_bytes: Option<u64>
max_samples: Option<u64>
max_distribution_entries: Option<u64>
max_tensor_elements: Option<u128>

The exact structure belongs to the implementation contract of limits.rs.


---

106. Capability semantics

Capabilities describe what a target or execution environment can support.

Capabilities MUST NOT be interpreted as semantic restrictions on the Zamani language.

For example:

target supports 128 physical qubits

means:

this target currently supports 128 resources

not:

Zamani supports only 128 qubits


---

107. Noise model lifecycle

A noise model should conceptually move through:

construction
    ↓
validation
    ↓
canonicalization
    ↓
compatibility checking
    ↓
application
    ↓
realization/sampling
    ↓
observation
    ↓
analysis

An invalid model MUST be rejected before execution where possible.


---

108. Model immutability

Once a noise model is validated and used as an execution input, semantic mutation SHOULD be prohibited.

If a new calibration changes the model:

old model
    ≠
new model

A new immutable snapshot/model identity should be produced.

This is essential for reproducibility.


---

109. Calibration does not mutate history

A calibration update MUST NOT retroactively change an existing execution record.

Instead:

CalibrationSnapshot A
        │
        ▼
Execution A

CalibrationSnapshot B
        │
        ▼
Execution B

Both remain reproducible.


---

110. Time semantics

When time affects noise, the model must specify what kind of time is used:

duration
relative time
absolute time
logical time
physical clock time
sample time
calibration timestamp

These MUST NOT be silently conflated.


---

111. Units

Physical quantities MUST carry explicit units or use a canonical unit contract.

Examples:

duration
frequency
energy
temperature
distance
probability rate

Unit conversion MUST be explicit and checked.


---

112. Precision

Numerical precision is part of reproducibility when it affects results.

Execution metadata SHOULD record:

precision
rounding mode where relevant
tolerance
approximation policy

ZQN MUST NOT claim exactness when the selected numerical representation is approximate.


---

113. Mathematical equivalence

Two representations may be considered equivalent if they produce equivalent semantic predictions under the declared metric and tolerance.

For example:

Kraus
    ↕
Choi
    ↕
superoperator

Differential tests SHOULD compare equivalent representations.


---

114. Composition

Noise composition must distinguish:

sequential composition
parallel/tensor composition
correlated composition
conditional composition
time evolution

The implementation MUST NOT use tensor product as a substitute for correlation.


---

115. Identity channel

The identity channel is a useful semantic identity.

Where mathematically valid:

I ∘ N = N
N ∘ I = N

should hold within representation/numerical guarantees.

Tests MUST verify this.


---

116. Composition metadata

Composition MUST preserve or combine:

provenance
uncertainty
approximation status
calibration references
target requirements
semantic identity

Metadata MUST NOT silently disappear.


---

117. Fault generation

Sampling a fault from a noise model MUST be explicit.

Conceptually:

NoiseModel
    +
SamplingContext
    ↓
Fault/Realization

The generated fault MUST contain enough information to interpret its location and type.


---

118. Sampling semantics

Sampling MUST specify:

sample domain
probability model
seed/context
number of samples
approximation

Sampling a distribution is not equivalent to computing the exact distribution.

The API must preserve that distinction.


---

119. Statistical semantics

Statistical results MUST distinguish:

estimate
sample count
variance/error
confidence
assumptions

A sample mean MUST NOT be presented as an exact probability.


---

120. Monte Carlo semantics

Monte Carlo execution MAY be used when exact computation is infeasible.

Monte Carlo results MUST expose:

sample count
seed policy
estimator
uncertainty
confidence
convergence criteria

Resource exhaustion MUST be distinguishable from statistical convergence.


---

121. Trajectory semantics

Trajectory simulation represents individual stochastic realizations.

A trajectory MUST be distinguishable from:

ensemble state
exact density operator
probability distribution

The simulator owns numerical execution.

ZQN owns the semantic noise process.


---

122. Deterministic mode

Deterministic mode means:

same semantic inputs
+
same deterministic context
=
same deterministic result

It does not mean the physical model becomes non-random.

It means randomness is reproducibly generated.


---

123. Reproducibility metadata

A reproducible result SHOULD record:

ZQN version
schema version
noise model identity
noise model configuration
calibration identity
target identity
program identity
seed
seed policy
precision
approximation policy
resource policy


---

124. Error taxonomy

ZQN errors SHOULD include categories equivalent to:

InvalidProbability
InvalidDistribution
InvalidChannel
InvalidFault
InvalidNoiseModel
InvalidCalibration
InvalidCharacterization
UnsupportedOperation
UnsupportedRepresentation
CapabilityMismatch
ResourceLimitExceeded
NumericalFailure
NonFiniteValue
SerializationFailure
DeserializationFailure
DeterminismViolation
Cancellation
ValidationFailure
CompatibilityFailure

The exact Rust enum belongs to core/error.rs.

No subsystem should create a competing global ZQN error taxonomy.


---

125. Integration error translation

Adapters MAY translate ZQN errors into subsystem-specific errors.

However, the semantic cause MUST remain recoverable.

For example:

ZQN CapabilityMismatch
        ↓
routing::RoutingError

is acceptable.

A generic:

"routing failed"

without the underlying semantic reason is insufficient.


---

126. IR integration

integration/ir.rs defines the boundary between:

canonical IR

and:

ZQN physical semantics

The preferred conceptual model is:

Canonical IR
    +
Noise Specification
    ↓
Noisy/Physical Execution View

The canonical IR MUST NOT become structurally dependent on the internal implementation of ZQN.


---

127. No OpenQASM-shaped ZQN semantics

ZQN MUST NOT define its semantic model around OpenQASM.

OpenQASM is a frontend/external format.

Its AST belongs to the OpenQASM frontend.

The flow is:

OpenQASM
    ↓
frontend AST
    ↓
canonical quantum::ir
    ↓
ZQN

not:

OpenQASM
    ↓
ZQN semantic AST


---

128. QIR relationship

ZQN is not QIR.

The relationship is:

Zamani IR
      │
      ├────────► ZQN
      │
      └────────► QIR export/lowering

QIR MAY serve as an interoperability layer downstream.

ZQN remains the Zamani physical-noise semantic model.


---

129. MLIR relationship

ZQN is not MLIR.

If Zamani later exposes ZQN through MLIR, the conceptual relationship is:

Zamani IR/ZQN
      ↓
Zamani MLIR representation
      ↓
MLIR transformations
      ↓
QIR / LLVM / target lowering

MLIR integration MUST NOT redefine ZQN semantics.


---

130. Integration with optimization

Optimization MAY use ZQN to evaluate whether transformations improve physical execution.

For example:

candidate A
    ↓
ZQN cost

candidate B
    ↓
ZQN cost

Optimization remains responsible for proving semantic equivalence.

ZQN only provides physical consequence information.


---

131. Integration with routing

Routing MAY use:

expected physical error
crosstalk
readout quality
transport error
calibration uncertainty
duration

ZQN MUST provide the physical estimate.

Routing chooses the placement.


---

132. Integration with scheduling

Scheduling MAY ask:

What noise occurs during this idle interval?
What is the expected physical error of this schedule?
What crosstalk is induced by concurrent operations?

ZQN answers through its semantic interfaces.


---

133. Integration with QEC

QEC MAY ask:

What faults can occur?
What are their probabilities?
Are they correlated?
Are they erasures?
Are they leakage events?
What is the logical effect?

ZQN provides the physical model.

QEC performs error-correction analysis.


---

134. Integration with memory

Memory MAY ask:

How does this channel transform the stored state?

ZQN provides channel semantics.

Memory chooses how to represent and execute that transformation.


---

135. Integration with hardware

Hardware MAY provide:

target capabilities
native operation descriptions
calibration
observations
device topology
timing
resource identities

ZQN consumes these abstractions.


---

136. Integration with benchmarking

Benchmarking MAY request:

noise profile
expected fidelity
characterization
error estimates
uncertainty
drift information

ZQN supplies semantic data.

Benchmarking owns the benchmark protocol.


---

137. Integration with runtime

Runtime owns:

execution lifecycle
resource allocation
clock
cancellation
job management
backend interaction

ZQN supplies:

physical model
realization
fault process
sampling policy

Runtime executes.


---

138. Integration with I/O

The I/O layer:

quantum::zqn::io

owns:

schemas;

serialization;

deserialization;

canonicalization;

compatibility.


Semantic modules MUST NOT each invent their own serialization format.


---

139. Integration with provenance

All major model creation and transformation operations SHOULD preserve provenance.

Example:

measured observations
      ↓
estimated channel
      ↓
calibrated noise model
      ↓
target realization
      ↓
execution

Every stage should remain traceable.


---

140. Integration with benchmarking and characterization

The relationship is intentionally bidirectional at the data-flow level but not through cyclic module dependencies:

characterization
      ↓
noise model
      ↓
execution
      ↓
benchmarking
      ↓
observations
      ↓
characterization

The Rust dependency graph should use narrow interfaces so this semantic feedback loop does not become a compile-time circular dependency.


---

141. Dependency rule

The preferred conceptual dependency direction is:

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

This is a conceptual layering rule, not a requirement that every layer directly import every previous layer.

Small traits and data contracts should be preferred.


---

142. Forbidden dependencies

ZQN MUST NOT depend semantically on:

frontend AST
vendor SDK
CLI
UI
benchmark implementation
routing implementation
scheduler implementation
QEC decoder implementation
specific simulator implementation

Adapters may depend on both sides where necessary.

The core semantic model must not.


---

143. Directory ownership

The intended semantic ownership is:

core/
    shared ZQN infrastructure

probability/
    probability and statistical primitives

channel/
    quantum channel mathematics

fault/
    realized fault semantics

noise/
    physical noise models and application

operations/
    operation-specific noise attachment

calibration/
    calibration state and validity

characterization/
    observations and estimation

simulation/
    ZQN simulation adapters/execution

propagation/
    uncertainty/error propagation

target/
    target requirements/capabilities/compatibility/lowering

integration/
    thin boundaries to other Zamani quantum subsystems

io/
    persistence/interchange

tests/
    ZQN-specific validation


---

144. core/error.rs

Owns the authoritative ZQN error taxonomy.

Must not own:

numerical algorithms;

channels;

sampling;

hardware errors.


Consumers:

all ZQN modules

Integration contract:

all ZQN errors
    ↓
core::error

Completion criteria:

all error categories defined;

no competing top-level ZQN error type;

errors distinguish semantic/resource/compatibility/numerical failures;

Rust 1.97 compatible.



---

145. core/version.rs

Owns:

ZQN semantic version;

schema version;

compatibility version.


No version constants should be scattered through ZQN.

Consumers:

io
canonicalization
compatibility
provenance
tests


---

146. core/ids.rs

Owns ZQN-specific identities only.

It MUST NOT define QubitId.

Canonical quantum identifiers come from:

crate::quantum::ir::qubit


---

147. core/limits.rs

Owns resource policy.

It MUST NOT encode semantic machine-size limits.

All expensive operations should be able to consume the relevant policy.


---

148. core/metadata.rs

Owns non-semantic descriptive metadata.

Metadata MUST NOT change mathematical meaning.


---

149. core/provenance.rs

Owns provenance structures.

It must permit scientific traceability without introducing dependencies on external storage systems.


---

150. core/context.rs

Owns explicit execution/model context.

It should carry:

limits
capabilities
calibration
determinism
provenance
cancellation
precision

It must not become a global singleton.


---

151. core/capabilities.rs

Owns ZQN capability vocabulary.

It MUST be provider-neutral.


---

152. probability/probability.rs

Owns validated probability values.

Required invariants:

finite
0 <= p <= 1

unless the type explicitly represents another mathematical quantity.


---

153. probability/distribution.rs

Owns generic distributions.

Must support:

normalization;

support;

sampling;

deterministic ordering;

resource-aware iteration.


Must not assume a fixed number of outcomes.


---

154. probability/categorical.rs

Owns categorical distributions.

It must support arbitrary finite outcome sets.

No hard-coded binary/quaternary assumptions.


---

155. probability/continuous.rs

Owns continuous distributions.

No fixed distribution family may be assumed.


---

156. probability/bounds.rs

Owns numerical/physical bounds.

Bounds must distinguish:

hard physical bound
numerical bound
confidence bound
resource bound


---

157. probability/statistics.rs

Owns statistical estimators.

Results should expose:

estimate
sample count
uncertainty
confidence

where applicable.


---

158. channel/channel.rs

Owns the generic quantum-channel semantic interface.

It must support arbitrary valid subsystem dimensions.

No fixed qubit count.


---

159. channel/representation.rs

Defines representation-independent channel concepts.

It must permit future representations without redesigning the channel abstraction.


---

160. channel/kraus.rs

Owns Kraus representation.

Must validate dimension and channel properties.

Large matrix allocations must be governed by resource policy.


---

161. channel/choi.rs

Owns Choi representation.

Must validate the relevant mathematical properties.


---

162. channel/process_matrix.rs

Owns generalized process representations.

It must not assume circuit gates.


---

163. channel/pauli.rs

Owns Pauli-based specializations.

Pauli noise is a specialization, not the fundamental ZQN model.


---

164. channel/stochastic.rs

Owns stochastic channel representations.

Must distinguish:

classical stochastic process

from:

quantum channel

where their semantics differ.


---

165. channel/lindblad.rs

Owns continuous-time generator semantics.

Numerical integration belongs to simulation.


---

166. channel/thermal.rs

Owns thermal-noise specialization.

Temperature and rates must be explicit model inputs.


---

167. channel/amplitude.rs

Owns amplitude-related channel specializations.

No hardware-specific assumptions.


---

168. channel/phase.rs

Owns phase/dephasing specializations.


---

169. channel/depolarizing.rs

Owns depolarizing specializations.


---

170. channel/generalized.rs

Owns generalized channel constructors that cannot be reduced to the specialized models.


---

171. channel/composition.rs

Owns sequential, tensor and correlated channel composition.

Must preserve:

dimensions;

validity;

provenance;

approximation metadata.



---

172. fault/fault.rs

Owns fault semantics.

A fault identifies an event/deviation, not merely a probability distribution.


---

173. fault/location.rs

Owns generic fault locations.

Must use canonical IR identifiers where applicable.

No duplicate qubit identifiers.


---

174. fault/classification.rs

Owns fault categories.

Categories must remain extensible.


---

175. fault/correlated.rs

Owns arbitrary-size correlated faults.

No fixed correlation arity.


---

176. fault/leakage.rs

Owns leakage semantics.

Must explicitly identify leakage outside the intended computational subspace.


---

177. fault/erasure.rs

Owns erasure semantics.

Erasure must remain distinguishable from generic Pauli noise.


---

178. fault/loss.rs

Owns physical loss semantics.


---

179. fault/batch.rs

Owns streaming/batched fault processing.

Must avoid mandatory whole-computation materialization.


---

180. noise/model.rs

Owns the principal NoiseModel semantic contract.

It must support:

description
validation
application
sampling
compatibility
provenance

It must not know vendor APIs.


---

181. noise/specification.rs

Owns declarative noise descriptions.

It must remain independent of source-language syntax.


---

182. noise/application.rs

Owns attachment of noise semantics to canonical operations/resources.


---

183. noise/composition.rs

Owns composition of multiple noise sources.

It must preserve correlation and approximation semantics.


---

184. noise/correlation.rs

Owns correlation-domain semantics.

No fixed number of resources.


---

185. noise/temporal.rs

Owns time-dependent noise.


---

186. noise/spatial.rs

Owns spatially dependent noise.

Topology is supplied by targets.


---

187. noise/crosstalk.rs

Owns crosstalk semantics.

Routing remains external.


---

188. noise/drift.rs

Owns drift models.


---

189. noise/non_markovian.rs

Owns memory-dependent noise semantics.

History must be explicit.


---

190. noise/conditional.rs

Owns conditional noise.

Dynamic-circuit semantics remain owned by canonical IR.


---

191. operations/operation.rs

Owns generic noise-bearing operation semantics.

It must remain independent of a fixed gate set.


---

192. operations/gate.rs

Owns gate-specific noise attachment.

The ideal gate remains owned by IR.


---

193. operations/preparation.rs

Owns preparation noise.


---

194. operations/reset.rs

Owns reset noise.


---

195. operations/measurement.rs

Owns measurement/readout noise.


---

196. operations/idle.rs

Owns duration-dependent idle noise.


---

197. operations/pulse.rs

Owns pulse-level noise.


---

198. operations/transport.rs

Owns transport noise.


---

199. calibration/snapshot.rs

Owns immutable calibration snapshots.


---

200. calibration/parameter.rs

Owns generic calibrated parameters.

Parameters should include units and uncertainty where relevant.


---

201. calibration/device.rs

Maps calibration information onto abstract target resources.


---

202. calibration/gate.rs

Owns gate-operation calibration data.


---

203. calibration/readout.rs

Owns readout calibration.


---

204. calibration/measurement.rs

Owns measurement calibration.


---

205. calibration/drift.rs

Owns calibration evolution.


---

206. calibration/interpolation.rs

Owns interpolation between calibration observations.

Interpolation must explicitly state approximation semantics.


---

207. calibration/validation.rs

Owns calibration consistency validation.


---

208. characterization/experiment.rs

Owns characterization experiment definitions.


---

209. characterization/protocol.rs

Owns protocol semantics.


---

210. characterization/observation.rs

Owns raw physical observations.

Raw observations must remain distinguishable from inferred models.


---

211. characterization/estimator.rs

Owns estimation from observations.


---

212. characterization/uncertainty.rs

Owns uncertainty estimation.


---

213. characterization/tomography.rs

Owns tomography semantics.

The actual computational implementation may use simulation/numerical services.


---

214. characterization/randomized_benchmarking.rs

Owns randomized-benchmarking protocol semantics.

It does not own general benchmarking orchestration.


---

215. characterization/process_characterization.rs

Owns general process characterization.


---

216. simulation/engine.rs

Owns ZQN-specific simulation integration only.

It must not duplicate the repository's primary simulation engine abstractions.


---

217. simulation/sampler.rs

Owns sampling interfaces.

Randomness comes from explicit deterministic context.


---

218. simulation/trajectory.rs

Owns trajectory semantics.


---

219. simulation/channel_engine.rs

Owns application of ZQN channel semantics to the existing simulation infrastructure.


---

220. simulation/monte_carlo.rs

Owns Monte Carlo orchestration.

It must expose statistical uncertainty.


---

221. simulation/deterministic.rs

Owns deterministic ZQN execution adaptation.

It must:

use existing simulation execution contracts;

accept explicit context;

avoid global RNG;

remain parallel-reproducible;

avoid hard-coded machine-size limits.



---

222. simulation/reproducibility.rs

Owns canonical deterministic seed derivation.

No other ZQN file may invent a second deterministic seed scheme.


---

223. propagation/error_budget.rs

Owns error-budget semantics.


---

224. propagation/uncertainty.rs

Owns uncertainty propagation.


---

225. propagation/fidelity.rs

Owns fidelity/error metrics.


---

226. propagation/bounds.rs

Owns mathematically justified bounds.


---

227. propagation/sensitivity.rs

Owns sensitivity analysis.


---

228. propagation/accumulation.rs

Owns error accumulation analysis.


---

229. target/requirements.rs

Defines what a computation/noise model requires.


---

230. target/capabilities.rs

Defines what a target supports.

No vendor-specific types.


---

231. target/compatibility.rs

Determines whether requirements can be satisfied.

It must distinguish:

exactly supported
approximately supported
unsupported


---

232. target/lowering.rs

Owns target realization.

Lowering must preserve declared approximation/error contracts.


---

233. target/validation.rs

Validates target compatibility before execution.


---

234. integration/ir.rs

Owns the narrow bridge between canonical IR and ZQN.

It must use:

crate::quantum::ir::qubit

where resource identity is needed.

It must not redefine IR semantics.


---

235. integration/routing.rs

Exposes physical-noise information to routing.

Routing policy remains outside ZQN.


---

236. integration/scheduling.rs

Exposes duration/time-dependent noise information.

Scheduling policy remains outside ZQN.


---

237. integration/qec.rs

Translates ZQN physical realizations into QEC-consumable faults.

It must not duplicate ZQN semantics.


---

238. integration/hardware.rs

Consumes hardware capabilities/calibration/observations.

It must not call provider APIs directly.


---

239. integration/memory.rs

Connects channel/fault semantics to memory/state transitions.


---

240. integration/benchmarking.rs

Provides noise/characterization data to benchmarking.


---

241. integration/runtime.rs

Connects ZQN execution semantics to runtime execution context.


---

242. io/schema.rs

Owns stable external schema definitions.


---

243. io/serialization.rs

Owns conversion from semantic objects into external representation.


---

244. io/deserialization.rs

Owns safe reconstruction from external data.

Untrusted input MUST be validated before resource-heavy construction.


---

245. io/canonical.rs

Owns deterministic canonical representation.


---

246. io/compatibility.rs

Owns schema compatibility and migration.


---

247. Semantic tests

Every mathematical invariant must have tests.

Probability:

0 <= p <= 1

Distribution:

normalization

Channels:

dimension correctness
complete positivity where applicable
trace preservation where applicable

Composition:

identity
associativity where mathematically applicable
tensor behavior


---

248. Property tests

Property tests SHOULD verify:

canonicalization(canonicalization(x)) = canonicalization(x)

serialize(deserialize(x))
    = canonical(x)

compose(identity, x)
    = x

compose(x, identity)
    = x

subject to the representation's declared mathematical semantics.


---

249. Differential tests

Equivalent representations SHOULD be compared.

Examples:

Kraus
Choi
superoperator
Pauli transfer

Observable predictions should agree within the declared tolerance.


---

250. Determinism tests

For a fixed context:

run(seed, model, program)

must equal:

run(seed, model, program)

across:

repeated executions;

different worker counts;

different task scheduling;

deterministic partitioning.



---

251. Scaling tests

Scaling tests MUST NOT define an architectural maximum.

Instead:

generated resource count N

is varied according to available test resources.

The implementation must behave semantically consistently for every supported finite N.


---

252. Fuzz testing

Fuzzing should cover:

serialized noise models;

distributions;

channel representations;

fault definitions;

correlation structures;

calibration input;

canonicalization;

compatibility input.


The objective is:

no panic
no UB
no uncontrolled allocation
no infinite loop
no silent semantic corruption


---

253. Security tests

Security tests MUST verify:

malicious dimensions are rejected;

integer overflow is prevented;

non-finite values are rejected;

huge allocations obey policy;

cancellation works;

malformed schemas fail safely;

untrusted calibration data cannot silently alter semantics.



---

254. Reproducibility tests

A reproducibility fixture should be able to record:

program identity
model identity
calibration identity
target identity
seed
schema
ZQN version
expected result

and reproduce the result later.


---

255. Scientific reproducibility

Scientific execution MUST record enough information to reconstruct the semantic execution.

At minimum where relevant:

program
noise model
calibration
target
seed
precision
approximation policy
resource policy
ZQN version
schema version


---

256. Compatibility testing

Compatibility tests MUST cover:

old schema → current
current → canonical
unsupported schema
future schema

No incompatible semantic change may be silently accepted.


---

257. API evolution

Public APIs should prefer additive extension.

When a semantic change is unavoidable:

version
migration
compatibility
tests

must be updated together.


---

258. File completion contract

A ZQN source file is considered complete only when its contract specifies:

1. Ownership


2. Non-ownership


3. Public API


4. Invariants


5. Inputs


6. Outputs


7. Dependencies


8. Consumers


9. Integration points


10. Error behavior


11. Resource behavior


12. Determinism


13. Serialization


14. Thread safety


15. Scalability


16. Tests


17. Compatibility



No implementation should be marked complete while any of these remain unspecified.


---

259. Frozen-contract principle

The purpose of this document is to make downstream implementation additive rather than disruptive.

Before implementing a file:

semantic contract
       ↓
API contract
       ↓
integration contract
       ↓
implementation
       ↓
tests

Once complete, downstream files should consume that contract.

A downstream implementation MUST NOT require reopening an unrelated foundational file merely because the downstream implementation was written later.

If a genuine semantic defect is discovered, the change must be treated as a versioned contract correction rather than an informal patch.


---

260. Independent implementation order

The recommended implementation order is:

1. core/error.rs
2. core/version.rs
3. core/ids.rs
4. core/limits.rs
5. core/metadata.rs
6. core/provenance.rs
7. core/capabilities.rs
8. core/context.rs

9. probability/probability.rs
10. probability/bounds.rs
11. probability/distribution.rs
12. probability/categorical.rs
13. probability/continuous.rs
14. probability/statistics.rs

15. channel/representation.rs
16. channel/kraus.rs
17. channel/choi.rs
18. channel/pauli.rs
19. channel/stochastic.rs
20. channel/process_matrix.rs
21. channel/lindblad.rs
22. channel/thermal.rs
23. channel/amplitude.rs
24. channel/phase.rs
25. channel/depolarizing.rs
26. channel/generalized.rs
27. channel/composition.rs
28. channel/channel.rs

29. fault/location.rs
30. fault/classification.rs
31. fault/fault.rs
32. fault/correlated.rs
33. fault/leakage.rs
34. fault/erasure.rs
35. fault/loss.rs
36. fault/batch.rs

37. noise/specification.rs
38. noise/correlation.rs
39. noise/temporal.rs
40. noise/spatial.rs
41. noise/drift.rs
42. noise/non_markovian.rs
43. noise/conditional.rs
44. noise/application.rs
45. noise/composition.rs
46. noise/model.rs

47. operations/*
48. calibration/*
49. characterization/*
50. propagation/*

51. simulation/reproducibility.rs
52. simulation/sampler.rs
53. simulation/channel_engine.rs
54. simulation/trajectory.rs
55. simulation/monte_carlo.rs
56. simulation/deterministic.rs
57. simulation/engine.rs

58. target/requirements.rs
59. target/capabilities.rs
60. target/compatibility.rs
61. target/validation.rs
62. target/lowering.rs

63. integration/ir.rs
64. integration/memory.rs
65. integration/routing.rs
66. integration/scheduling.rs
67. integration/qec.rs
68. integration/hardware.rs
69. integration/benchmarking.rs
70. integration/runtime.rs

71. io/schema.rs
72. io/canonical.rs
73. io/serialization.rs
74. io/deserialization.rs
75. io/compatibility.rs

76. subsystem mod.rs files
77. ZQN root mod.rs
78. prelude.rs
79. integration tests
80. property/differential/scaling/fuzz tests

The exact ordering can change only when a dependency requires it.


---

261. Why foundations come first

The first implementation group contains no dependency on routing, scheduling, QEC or hardware.

That means these files can be stabilized independently.

For example:

probability
    ↓
channel
    ↓
fault
    ↓
noise

can be developed before hardware integration exists.

This minimizes rework.


---

262. Root module responsibility

zqn/mod.rs is only a composition boundary.

It MUST NOT implement mathematical semantics.

It should declare:

core
probability
channel
fault
noise
operations
calibration
characterization
simulation
propagation
target
integration
io

after those modules have valid contracts.


---

263. prelude.rs

The prelude MUST contain only stable, high-value public concepts.

It MUST NOT become a dumping ground for every ZQN implementation type.


---

264. Module independence

Each mod.rs MUST:

document ownership;

declare children;

define its public boundary;

avoid implementation duplication;

avoid circular imports;

expose only stable interfaces.



---

265. No hidden coupling

A file MUST NOT depend on:

another file's private implementation detail

If two modules require communication, define a stable semantic interface.


---

266. No speculative public API

Do not expose APIs simply because a future feature might need them.

Future scalability should be supported through:

traits
extensible enums where appropriate
generic resource descriptions
capability negotiation
versioned schemas

not speculative public fields everywhere.


---

267. Extensibility principle

Adding a new quantum technology should preferably require:

new target capability
new representation/adapter where required
new tests

not rewriting:

probability
fault
noise
IR
routing

from scratch.


---

268. Extending resource types

Adding a new resource type should not require rewriting every noise model.

The architecture must use generic resource/location abstractions.

For example, adding:

BosonicMode

should not require modifying every existing qubit-specific noise model if that model is not applicable.

Instead compatibility should reject or accept it based on explicit capability/type semantics.


---

269. Extending channel representations

Adding a new channel representation should not change the generic channel contract.

Example:

existing:
Kraus
Choi
PauliTransfer

future:
new sparse representation

The new representation plugs into the representation abstraction.


---

270. Extending noise models

Adding:

new correlated noise

should not require modifying:

Fault
Probability
Calibration
Target
IR

unless the new semantics genuinely introduce a new foundational concept.


---

271. Exactness hierarchy

Every ZQN result should conceptually be classified as one of:

EXACT
APPROXIMATE
BOUNDED
STATISTICAL
UNSUPPORTED

This classification should propagate through transformations.

For example:

exact channel
    ↓
exact composition
    ↓
exact result

but:

exact channel
    ↓
Pauli approximation
    ↓
APPROXIMATE

must remain approximate.


---

272. Approximation propagation

If:

A ≈ B

with error bound ε, and B is transformed into C, the resulting contract must account for the propagated approximation.

No approximation metadata may disappear merely because an object was converted between representations.


---

273. Calibration uncertainty propagation

When a model depends on calibration:

calibration uncertainty
       ↓
noise uncertainty
       ↓
execution uncertainty
       ↓
result uncertainty

The uncertainty chain should remain traceable.


---

274. Correlation preservation

An optimization or representation conversion MUST NOT silently destroy correlations.

If correlation is intentionally discarded:

correlation approximation

must be declared.


---

275. Independence assumptions

Independence MUST be explicit.

A model must not assume:

P(A,B) = P(A)P(B)

unless that assumption is part of its semantic definition.


---

276. Temporal independence

Likewise, noise events across time MUST NOT be assumed independent without explicit justification.


---

277. Spatial independence

Noise across resources MUST NOT be assumed independent without explicit justification.


---

278. Measurement independence

Readout results MUST NOT be assumed independently distributed if correlated detector behavior is modeled.


---

279. Resource identity semantics

A physical resource identity is meaningful only within an appropriate target/context.

Therefore a PhysicalQubitId alone does not necessarily identify a globally unique physical device.

Target identity and resource identity may both be required.


---

280. Distributed identity

For distributed systems:

target identity
+
node identity
+
resource identity

may be required.

ZQN must not collapse these into a single integer.


---

281. No integer-only semantic resource model

Avoid APIs whose fundamental identity is:

usize

for quantum resources.

Canonical IR types must be used.

Integer indices may be implementation details after validation.


---

282. Addressing

Where an implementation needs compact indexing:

canonical ID
    ↓
validated mapping
    ↓
local index

The local index must not become the semantic identity.


---

283. Serialization of identifiers

Canonical quantum identifiers must serialize according to their canonical IR contract.

ZQN must not invent a conflicting encoding.


---

284. Target-specific resource mapping

Mapping:

QubitId → PhysicalQubitId

belongs to routing/target integration.

ZQN consumes the resulting physical identity when required.

ZQN does not perform logical-to-physical routing.


---

285. Program identity

Deterministic reproducibility SHOULD identify the canonical program.

Program identity should be based on canonical semantic representation rather than source formatting.

Thus:

equivalent source formatting

should not unnecessarily produce different program identities when the canonical IR is identical.


---

286. Model identity

Noise model identity should be based on canonical semantic representation.

Formatting or object allocation order must not change identity.


---

287. Calibration identity

Calibration identity must include the calibration snapshot/version relevant to the execution.


---

288. Target identity

Target identity must distinguish different hardware/resource configurations where physical behavior can differ.


---

289. Execution identity

An execution record may combine:

program identity
model identity
calibration identity
target identity
execution policy
seed

into a reproducible execution identity.


---

290. Cache semantics

A cached result is valid only if all semantic inputs affecting the result match.

Resource policy may also matter if the result itself depends on approximation/resource selection.


---

291. Deterministic canonical ordering

Any collection whose ordering affects:

hash
serialization
seed derivation
result identity

must have deterministic canonical ordering.


---

292. Hashing

Cryptographic hashes MAY be used for identities.

The hash algorithm and canonicalization procedure must be versioned where persisted identities depend on them.


---

293. Security of hashes

Hashes are identifiers/integrity aids.

They MUST NOT be treated as proof of physical correctness.


---

294. Scientific claims

ZQN MUST NOT claim that a noise model is physically accurate merely because:

it validates mathematically

Mathematical validity and empirical validity are different.


---

295. Empirical validity

A model may be:

mathematically valid
but empirically inaccurate

Characterization/provenance should make this distinction visible.


---

296. Model confidence

Where available, models SHOULD carry confidence or uncertainty information.


---

297. Model assumptions

A model SHOULD document assumptions such as:

Markovian
stationary
independent
Gaussian
weak-noise
thermal equilibrium
fixed calibration

Assumptions are part of scientific semantics.


---

298. Assumption violations

If execution conditions violate a model's declared assumptions, the system SHOULD report a validation/compatibility warning or error according to policy.

It must not silently pretend the model remains valid.


---

299. Runtime environment

Runtime context may provide:

clock
temperature
environment
resource state
target state
calibration
seed policy

ZQN may consume these explicitly.


---

300. No hidden environment reads

A noise model MUST NOT silently read:

system clock
thread ID
process ID
hardware randomness
environment variables
global mutable state

when those values affect deterministic semantics.

They must be explicit context inputs.


---

301. Real-time systems

If a hardware runtime requires real-time behavior, the runtime owns scheduling and deadline enforcement.

ZQN describes the physical consequences.

ZQN MUST NOT assume that every execution environment is real-time.


---

302. Hardware observations

Hardware observations should enter ZQN through provider-neutral structures.

Example conceptual flow:

hardware adapter
      ↓
ObservedNoise
      ↓
characterization
      ↓
NoiseModel


---

303. Model fitting

Model fitting may produce multiple candidate models.

The selected model MUST preserve:

selection method
observations
uncertainty
criteria


---

304. Model comparison

Models may be compared using explicit criteria.

Examples:

likelihood
prediction error
fidelity
complexity
cross-validation
physical plausibility

The comparison method is part of characterization.


---

305. Benchmarking separation

Benchmark protocols remain in benchmarking.

ZQN characterization may implement physical-noise characterization protocols.

The two must not become one subsystem.


---

306. Quantum Volume

Quantum Volume belongs to benchmarking.

ZQN may provide:

noise estimates
execution models
calibration
observations

but does not own Quantum Volume methodology.


---

307. Randomized benchmarking

Randomized benchmarking MAY exist in characterization as a physical characterization protocol.

Benchmarking orchestration remains in quantum::benchmarking.


---

308. Error mitigation

Error mitigation algorithms are not automatically ZQN semantics.

ZQN provides noise/error information.

Mitigation belongs to the appropriate optimization/algorithm/runtime subsystem.


---

309. Error correction versus mitigation

ZQN must distinguish:

noise modeling
fault correction
error mitigation

They are different layers.


---

310. Fault injection

Fault injection is a valid ZQN execution/testing use case.

It must use the same semantic fault model as ordinary noise realization.

This prevents tests from exercising an artificial model different from production.


---

311. Test-only noise

Test-only deterministic fault sources MAY exist.

They must remain explicitly marked as test/synthetic models.

They must not masquerade as empirical hardware noise.


---

312. Synthetic models

Synthetic models are useful for:

testing;

benchmarking;

algorithm analysis;

simulation;

educational execution.


They should carry provenance indicating that they are synthetic.


---

313. Production model provenance

A production hardware model should identify whether it came from:

measurement
simulation
calibration
fitted model
user declaration
synthetic generation


---

314. Numerical backend independence

ZQN semantics must not require one specific numerical backend.

A numerical implementation MAY use:

standard floating point;

arbitrary precision;

sparse structures;

specialized linear algebra;

hardware acceleration.


The semantic result must remain consistent with the declared precision/approximation contract.


---

315. Hardware acceleration

GPU/accelerator use is an implementation detail.

ZQN must not require CUDA or any other accelerator.


---

316. CPU parallelism

Parallelism is an execution optimization.

It must not alter deterministic semantics.


---

317. Distributed execution

Distributed execution is an execution strategy.

It must not alter the canonical noise semantics.


---

318. Failure atomicity

Where an operation mutates an owned state, failure SHOULD leave the state in a defined condition.

Prefer transactional or immutable construction for semantic objects.


---

319. Partial results

Long-running operations MAY expose partial results only if the API explicitly defines their semantic status.

Partial results must not be mistaken for complete results.


---

320. Streaming statistics

Statistics over huge experiments SHOULD support incremental accumulation.

An entire dataset must not always be stored in memory.


---

321. Reproducible streaming

Streaming execution must remain deterministic when deterministic mode is requested.

Chunk size must not change the semantic result unless the approximation contract explicitly permits it.


---

322. Parallel reductions

Floating-point reductions can be order-sensitive.

When deterministic numerical results are required, ZQN must define a deterministic reduction strategy or explicitly classify the result as numerically order-dependent.


---

323. Numerical reproducibility

Scientific reproducibility must distinguish:

bitwise reproducibility

from:

semantic/numerical reproducibility

The required level must be declared.


---

324. Cross-platform reproducibility

Where bitwise reproducibility cannot be guaranteed across architectures, the system should provide a declared numerical tolerance and deterministic semantic contract.


---

325. Model validation

Validation must occur at appropriate levels:

syntax
schema
mathematical
dimensional
resource
capability
physical-assumption


---

326. Validation ordering

A recommended order is:

schema
  ↓
basic numerical validity
  ↓
dimension/resource validity
  ↓
mathematical validity
  ↓
capability compatibility
  ↓
execution validation

This avoids expensive work on malformed input.


---

327. Expensive validation

Expensive mathematical validation MAY be governed by resource policy.

For example, complete positivity verification on a very large representation may require significant resources.

The system must distinguish:

proven valid
proven invalid
not validated due to policy/resource


---

328. Validation status

Where validation is deferred, the object must not falsely claim full validation.


---

329. Lazy validation

Large models MAY validate incrementally.

The semantic contract must identify whether validation is:

eager
lazy
partial
complete


---

330. Semantic immutability

A validated immutable semantic object is preferred.

If lazy validation caches results, the cache must not change semantic meaning.


---

331. Concurrency

Concurrent access to immutable models should be safe.

Mutable execution state belongs in explicit execution contexts.


---

332. No global calibration

Calibration is contextual.

A process must not silently use "the current calibration" from global state.

The relevant calibration snapshot must be supplied explicitly.


---

333. No global target

Likewise, a model must not silently query a global current target.

Target context is explicit.


---

334. No global limits

Resource policy is explicit.


---

335. No global seed

Deterministic seed policy is explicit.


---

336. No global clock for semantics

If time affects noise, the relevant time must be supplied by execution context.


---

337. Error budget integration with routing

Routing may use predicted error as an optimization objective.

The error estimate must identify:

model
calibration
target
time assumptions
uncertainty


---

338. Error budget integration with scheduling

Scheduling must account for time-dependent error.

An idle slot can be physically significant.


---

339. Error budget integration with QEC

QEC may transform physical error budgets into logical error estimates.

ZQN provides physical semantics.

QEC owns logical correction.


---

340. Error budget integration with benchmarking

Benchmarking can compare predicted versus observed error.

This enables model validation.


---

341. Feedback loop

The complete scientific loop is:

model
  ↓
execution
  ↓
observation
  ↓
characterization
  ↓
calibration/model update
  ↓
new model

The loop must preserve provenance.


---

342. Production readiness criteria

ZQN is production-ready only when all of the following are true:

Semantic

mathematical semantics are explicit;

physical assumptions are explicit;

approximations are explicit;

uncertainty is represented;

correlations are represented;

non-Markovian models are possible;

future modalities are not structurally excluded.


Scalability

no semantic machine-size maximum;

no fixed qubit count;

no fixed gate arity;

no mandatory full-state materialization;

streaming is possible;

resource policies are explicit.


Determinism

caller-controlled seed;

centralized derivation;

parallel determinism;

distributed determinism;

provenance.


Safety

no unsafe Rust;

checked arithmetic;

finite-value validation;

resource governance;

cancellation;

secure deserialization.


Integration

canonical IR remains authoritative;

canonical qubit IDs remain authoritative;

routing integrated;

scheduling integrated;

QEC integrated;

memory integrated;

simulation integrated;

hardware integrated;

benchmarking integrated;

runtime integrated;

IO integrated.


Scientific integrity

uncertainty;

calibration;

provenance;

characterization;

approximation/error contracts.



---

343. Final semantic invariant

The strongest invariant of ZQN is:

A ZQN model describes physical uncertainty and imperfect realization
without changing what the canonical Zamani program means.

Therefore:

same program
+
different target

may produce:

different physical realization
different noise
different execution statistics

while preserving the intended program semantics wherever the targets satisfy the required capabilities.


---

344. Final scalability invariant

The strongest scalability invariant is:

No finite machine size is part of ZQN semantics.

For every finite resource size N that the available:

compiler
runtime
memory
simulator
hardware
resource policy

can support, the same semantic program model remains valid.

Thus:

N = 1
N = 10
N = 100
N = 1,000
N = 1,000,000
N = ...

are resource instances, not different language semantics.


---

345. Final determinism invariant

For a declared deterministic execution context:

same program
+
same ZQN model
+
same calibration
+
same target
+
same seed
+
same deterministic policy

must produce the same declared deterministic result independently of worker scheduling.


---

346. Final approximation invariant

ZQN MUST NEVER silently turn:

exact

into:

approximate

without recording the approximation and its error contract.


---

347. Final identity invariant

Quantum resource identity remains owned by:

crate::quantum::ir::qubit

ZQN consumes those identities.

It does not replace them.


---

348. Final ownership invariant

The definitive ownership split is:

quantum::ir
    WHAT

ZQN
    WHAT PHYSICAL IMPERFECTION AFFECTS IT

optimization
    EQUIVALENT TRANSFORMATION

routing
    WHERE

scheduling
    WHEN

QEC
    HOW FAULTS ARE CORRECTED

hardware
    WHAT THE TARGET CAN DO

runtime
    HOW EXECUTION IS ORCHESTRATED

memory
    HOW STATE/RESOURCES ARE REPRESENTED

benchmarking
    HOW PERFORMANCE IS MEASURED


---

349. Final architectural invariant

ZQN MUST remain:

backend-independent
vendor-neutral
resource-policy-driven
deterministic when requested
numerically explicit
scientifically reproducible
representation-polymorphic
technology-neutral
safe Rust


---

350. Final "atom to everywhere" contract

The ultimate Zamani quantum execution contract is:

ONE ZAMANI PROGRAM
                          │
                          ▼
                   CANONICAL IR
                          │
                          ▼
                     ZQN MODEL
                          │
              ┌───────────┼───────────┐
              │           │           │
              ▼           ▼           ▼
          tiny target  large target  distributed target
              │           │           │
              ▼           ▼           ▼
          target A     target B      target C
              │           │           │
              └───────────┼───────────┘
                          ▼
                     EXECUTION
                          │
                          ▼
                     OBSERVATIONS

The program does not change merely because:

the machine becomes larger;

the topology changes;

the technology changes;

the vendor changes;

the noise model changes;

the calibration changes;

the execution becomes distributed;

the simulator changes;

the hardware changes.


Only the target realization, physical model, resource policy, and execution environment change.

That is the semantic foundation required for Zamani to move from the smallest supported quantum system to arbitrarily large finite systems constrained only by available resources.


---

351. Definition of complete

SEMANTICS.md is complete when every implementation file under src/quantum/zqn/ can answer, before implementation:

What does this file mean?

What does it own?

What does it not own?

What types does it expose?

What invariants must hold?

What inputs does it consume?

What outputs does it produce?

Who consumes it?

Who must never depend on it?

How does it integrate with IR?

How does it integrate with QEC?

How does it integrate with routing?

How does it integrate with scheduling?

How does it integrate with memory?

How does it integrate with simulation?

How does it integrate with hardware?

How does it integrate with benchmarking?

How is it deterministic?

How does it scale?

What resource limits apply?

How are errors represented?

How is uncertainty represented?

How is approximation represented?

How is it serialized?

How is provenance preserved?

How is it tested?

How is it extended without breaking existing code?

If any answer is missing, the implementation contract is incomplete.


---

352. Non-negotiable rules

The following rules are permanent unless this semantic specification is explicitly versioned:

1. quantum::ir remains the canonical semantic quantum IR.


2. ZQN does not become another IR.


3. quantum::ir::qubit::{QubitId, PhysicalQubitId} remain canonical.


4. ZQN does not define competing qubit identities.


5. No semantic machine-size limit exists.


6. Resource limits are explicit policies.


7. No vendor APIs exist inside ZQN.


8. No hidden global RNG exists.


9. Deterministic execution uses caller-controlled seed/context.


10. Parallel scheduling must not change deterministic results.


11. Approximation is explicit.


12. Uncertainty is explicit.


13. Correlation is explicit.


14. Non-Markovian behavior is representable.


15. Leakage, erasure and loss remain distinct semantic concepts.


16. ZQN is not gate-only.


17. ZQN is not qubit-only.


18. ZQN is not simulator-only.


19. ZQN is not hardware-specific.


20. ZQN is not QEC.


21. ZQN is not benchmarking.


22. ZQN is not routing.


23. ZQN is not scheduling.


24. ZQN is not QIR.


25. ZQN is not MLIR.


26. Serialization is versioned.


27. Provenance is preserved.


28. Expensive work is resource-governed.


29. Cancellation is supported for long-running operations.


30. Numerical overflow and non-finite values are handled explicitly.


31. Untrusted input is validated.


32. Rust unsafe is forbidden.


33. Rust 1.97/1.97.1 remains supported.


34. New quantum technologies must be extensible without redesigning the foundational semantic model.


35. Existing subsystem implementations must consume ZQN rather than duplicate its universal semantics.


36. Independent foundational files must be completable without requiring later architectural rewrites.


37. Integration contracts must be defined before implementation.


38. Semantic changes require explicit contract/version treatment.


39. No implementation detail may silently become language semantics.


40. A program is written once; target/resource realization is selected separately.



