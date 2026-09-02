Worked for 11s

Below is the complete replacement content I recommend for:

src/quantum/zqn/SCALABILITY.md

I corrected the earlier architecture against the current repository, including the fact that ZQN now already has the major subsystem directories, the canonical qubit identity rule in quantum::ir::qubit, the existing routing/noise-aware integration, and the current ZQN architecture contract.  The repository's IR also explicitly establishes that quantum::ir is the canonical semantic boundary and that it has no architectural maximum for qubits, operations, depth, topology, or technology.

I have also incorporated the current QIR/MLIR architectural principles: QIR separates a program's representation from target-specific instruction capabilities and supports dynamic resource management, while MLIR emphasizes generic interfaces so transformations do not need hard-coded knowledge of every concrete operation/dialect. 

ZQN Scalability Contract

Path: "src/quantum/zqn/SCALABILITY.md"
Subsystem: "crate::quantum::zqn"
Language: Rust
Edition: Rust 2021
Supported toolchains: Rust 1.97 and Rust 1.97.1
Safety: "#![forbid(unsafe_code)]"
Status: Production scalability contract
Primary objective: Write once, scale everywhere

---

1. Purpose

This document defines the mandatory scalability architecture and implementation contract for Zamani Quantum Noise (ZQN).

ZQN must be capable of representing and processing quantum-noise semantics for computations ranging from the smallest useful quantum system to the largest system that can actually be represented and processed by the available resources.

The architectural requirement is:

«A Zamani quantum program is written once and must not need to be rewritten merely because the target quantum machine becomes larger, smaller, physically different, distributed, logical, or technologically different.»

The corresponding ZQN requirement is:

«A ZQN noise model is expressed once at the semantic level and must be capable of being realized at any compatible system size and target technology without embedding an artificial finite machine-size ceiling.»

This document defines how that requirement is achieved without:

- hard-coded machine sizes;
- hard-coded qubit counts;
- vendor-specific assumptions;
- fixed gate arity;
- fixed topology;
- global mutable state;
- hidden randomness;
- unsafe Rust;
- mandatory full-system materialization;
- duplicated qubit identities;
- duplicated quantum semantics;
- implicit approximation;
- uncontrolled allocation.

---

2. Meaning of "infinity"

ZQN uses "infinity" in an architectural sense.

It does not claim that a physical computer has infinite:

- RAM;
- storage;
- CPU;
- address space;
- execution time;
- network bandwidth;
- quantum resources;
- simulator capacity.

Instead:

«ZQN has no artificial finite semantic upper bound on the size of a quantum system.»

Therefore there must be no architecture-level constants such as:

MAX_QUBITS
MAX_PHYSICAL_QUBITS
MAX_QUBIT_INDEX
MAX_GATES
MAX_OPERATIONS
MAX_DEPTH
MAX_CORRELATED_QUBITS
MAX_NOISE_EVENTS
MAX_FAULTS

used as semantic definitions.

A finite invocation may still have finite resource limits.

The distinction is fundamental:

                    ZQN SEMANTICS
                         │
                         │
             no artificial finite ceiling
                         │
                         ▼
                  RESOURCE POLICY
                         │
             ┌───────────┴───────────┐
             │                       │
       compiler limit          runtime limit
             │                       │
       memory limit            target limit
             │                       │
             └───────────┬───────────┘
                         ▼
                    actual execution

A resource limit says:

«"This particular invocation is not permitted to consume more than X."»

It must never mean:

«"Zamani can never represent a system larger than X."»

---

3. Relationship to the canonical Quantum IR

ZQN is not a second quantum intermediate representation.

The canonical semantic boundary remains:

crate::quantum::ir

The repository explicitly defines "quantum::ir" as the canonical hardware-independent semantic IR and states that it has no architectural maximum for qubits, operations, depth, topology, gate arity, or quantum technology.

Therefore:

quantum::ir
    =
    WHAT the computation means

ZQN
    =
    WHAT physical uncertainty/noise affects that computation

The dependency direction is:

Zamani source
      │
      ▼
quantum::frontend
      │
      ▼
quantum::ir
      │
      ├───────────────┐
      │               │
      ▼               ▼
 optimization      analysis
      │
      ▼
     ZQN
      │
      ├──────────────┬──────────────┐
      ▼              ▼              ▼
   routing       scheduling         QEC
      │              │              │
      └──────────────┼──────────────┘
                     ▼
              target / hardware
                     │
          ┌──────────┼──────────┐
          ▼          ▼          ▼
      simulator     QPU      emulator

ZQN must never redefine canonical program semantics merely to make noise handling convenient.

---

4. Canonical qubit identity

Whenever ZQN needs a qubit identity, it must use:

crate::quantum::ir::qubit::QubitId

for logical/canonical qubit identity where appropriate, and:

crate::quantum::ir::qubit::PhysicalQubitId

for physical qubit identity where appropriate.

ZQN must never define another:

struct QubitId(...);

or:

type QubitId = usize;

or equivalent wrapper whose purpose is to replace the canonical identity.

The repository's IR explicitly establishes "quantum::ir::qubit" as the authoritative qubit identity boundary.

ZQN-specific IDs are permitted only for ZQN-owned entities.

Examples:

NoiseModelId
ChannelId
FaultId
CalibrationId
CharacterizationId
ExperimentId
ObservationId
NoiseSnapshotId

These identify ZQN objects, not qubits.

---

5. Why "usize" must not become a semantic identity

This is forbidden as a universal identity model:

type QubitId = usize;

because:

1. it duplicates the canonical IR identity;
2. it ties semantics to host address-size assumptions;
3. it encourages fixed-index reasoning;
4. it makes distributed/resource identity ambiguous;
5. it encourages accidental machine-size limits;
6. it makes logical and physical identity easier to confuse.

A qubit identity must remain an explicit domain type.

---

6. No fixed system size

ZQN must never assume that a computation contains:

1 qubit
2 qubits
3 qubits
5 qubits
20 qubits
50 qubits
127 qubits
1000 qubits

or any other architectural maximum.

The number of resources is data.

For example:

NoiseModel
    +
ResourceSet
    +
OperationSet
    +
TargetCapabilities

determines the realized system.

The model itself does not encode a universal maximum.

---

7. No fixed gate arity

ZQN must not assume that quantum operations are always:

1-qubit
2-qubit

Noise must be capable of applying to arbitrary resource sets.

Examples include:

single-resource noise
two-resource correlated noise
N-resource correlated noise
global noise
collective noise
network-link noise
mode noise
logical-resource noise

The number of affected resources must be represented by data.

Therefore avoid structures such as:

TwoQubitNoise
ThreeQubitNoise
FourQubitNoise

as the fundamental architecture.

Prefer:

NoiseLocation
    └── arbitrary resource collection

or an equivalent extensible representation.

---

8. No fixed topology

ZQN must not assume:

line
grid
ring
heavy hex
all-to-all
nearest-neighbor

as a universal topology.

Topology belongs to target/resource descriptions.

ZQN may consume topology information when modeling:

- correlated noise;
- crosstalk;
- spatial correlations;
- transport noise;
- routing costs.

But topology must be supplied as target data.

Therefore:

ZQN
  +
TargetCapabilities
  +
TargetTopology

is valid.

Hard-coding a specific topology into ZQN is not.

---

9. No vendor-specific semantics

ZQN must not contain semantic implementations whose architecture is:

if IBM { ... }
if IonQ { ... }
if Rigetti { ... }
if Quantinuum { ... }
if Google { ... }
if AWS { ... }

Vendor-specific code belongs in:

crate::quantum::hardware

ZQN consumes abstract information such as:

TargetCapabilities
CalibrationSnapshot
NoiseObservation
TargetResource
ExecutionContext

This maintains the many-to-many architecture also used by QIR, where the program/profile and backend instruction capabilities are separate concerns.

---

10. Technology independence

ZQN must not equate quantum computing with:

qubit + gate + Pauli error

The scalability model must permit:

- gate-model computation;
- dynamic circuits;
- logical qubits;
- fault-tolerant computation;
- qudits;
- bosonic systems;
- continuous-variable systems;
- photonic systems;
- neutral atoms;
- trapped ions;
- superconducting systems;
- spin systems;
- measurement-based computation;
- analog computation;
- Hamiltonian simulation;
- annealing;
- distributed quantum computation;
- quantum networks;
- quantum communication;
- transport-based systems;
- future quantum modalities.

This requires resource- and operation-oriented noise semantics.

---

11. Resource-oriented noise

Noise must be attachable to abstract resources.

The conceptual abstraction is:

NoiseLocation
    │
    ├── logical resource
    ├── physical resource
    ├── qubit
    ├── qudit
    ├── mode
    ├── bosonic mode
    ├── operation
    ├── measurement
    ├── pulse
    ├── transport link
    ├── communication channel
    └── composite resource

This prevents ZQN from becoming permanently tied to today's qubit-only architectures.

Where the underlying resource is specifically a qubit, use the canonical:

quantum::ir::qubit::QubitId
quantum::ir::qubit::PhysicalQubitId

---

12. Representation polymorphism

A scalable noise framework must not require one mathematical representation for every system.

Possible representations include:

Kraus
Choi
superoperator
Liouville
Pauli transfer matrix
stochastic map
Lindblad generator
trajectory
symbolic channel
sparse channel
tensor representation
hardware-native representation

The choice must be determined by:

semantic requirements
+
target capabilities
+
available resources
+
requested precision
+
execution policy

not by a hard-coded global choice.

---

13. Semantic representation versus execution representation

These must remain separate.

For example:

abstract channel
      │
      ├── Kraus representation
      ├── Choi representation
      ├── stochastic representation
      ├── trajectory representation
      └── hardware-native representation

A user should be able to define a channel once.

A backend may choose an appropriate realization.

This is essential for scaling because materializing a full dense representation may be reasonable for a tiny subsystem and completely infeasible for a large one.

---

14. Never require full-system materialization

ZQN must support streaming and lazy processing.

Do not require:

Vec<Fault>

or:

Vec<NoiseEvent>

for the entire execution when the event space can be enormous.

The architecture must permit:

iterator
stream
lazy generator
bounded batch
windowed processing
chunked processing
on-demand evaluation

The implementation may materialize data when explicitly requested and resource policy permits it.

---

15. Materialization policy

Every potentially large operation should distinguish:

semantic object

from:

materialized representation

For example:

CorrelatedNoiseModel
      │
      ├── symbolic form
      ├── lazy evaluator
      ├── sparse realization
      └── materialized realization

This prevents an abstract model from accidentally allocating memory proportional to an entire machine merely because one consumer requested a local calculation.

---

16. Complexity must be explicit

Scalability does not mean every operation is O(1).

A physically meaningful global correlated channel may inherently require work proportional to system size.

Therefore ZQN must distinguish:

semantic scalability

from:

algorithmic complexity

The architecture guarantees:

«No artificial finite ceiling.»

It does not guarantee:

«Every operation has constant time or constant memory.»

Each implementation must document:

- asymptotic time complexity;
- asymptotic memory complexity;
- streaming behavior;
- materialization behavior;
- numerical complexity;
- parallelization behavior.

---

17. Complexity-aware API design

Public APIs should avoid forcing unnecessary materialization.

Prefer concepts equivalent to:

iter()
iter_mut()
stream()
sample()
evaluate()
apply()
estimate()

over APIs that always require complete collections.

Where collection materialization is necessary, it must be explicit.

---

18. Resource policy

Resource limits are mandatory for production safety.

The policy layer may limit:

memory
CPU time
wall time
sampling shots
serialized input size
number of generated events
number of iterations
materialized elements
tensor elements
correlation expansion
simulation trajectories

These belong to:

core::limits
core::context
runtime policy

and not to mathematical semantics.

---

19. Limits must be configurable

Do not write:

const MAX_FAULTS: usize = 1_000_000;

as a universal semantic rule.

Prefer:

ZqnLimits

with optional policy values.

Conceptually:

max_operations: Option<u64>
max_faults: Option<u64>
max_materialized_elements: Option<u128>
max_memory_bytes: Option<u64>
max_shots: Option<u64>
max_execution_time: Option<Duration>

"None" means:

«ZQN itself does not impose a limit for this category.»

It does not mean the operating system or target has infinite resources.

---

20. Policy must be inspectable

When a computation is rejected because of a resource limit, the error must identify:

what was requested
what limit was applied
which policy supplied the limit
what operation triggered it

Avoid opaque:

failed

or:

too large

errors.

---

21. Semantic failure versus resource failure

These must be different.

Example:

InvalidChannel

means the mathematical object is invalid.

Whereas:

ResourceLimitExceeded

means the object may be valid, but this invocation is not permitted to process it.

This distinction is required for scalable compilation and retry policies.

---

22. Cancellation

Potentially large ZQN operations must support cancellation where execution can take significant time.

Examples:

- characterization;
- tomography;
- simulation;
- Monte Carlo;
- large serialization;
- channel conversion;
- correlation expansion.

Cancellation belongs to execution context rather than mathematical semantics.

---

23. Parallel scalability

ZQN must support parallel execution without changing semantics.

Under deterministic execution:

1 worker
8 workers
64 workers

must produce the same semantic result when given the same:

program
noise model
target
calibration
seed
execution policy

unless the selected numerical algorithm explicitly declares nondeterministic reduction behavior.

---

24. Deterministic stochastic execution

No ZQN implementation may silently use a hidden global RNG.

Forbidden:

global random generator
thread-local semantic randomness
implicit random seed
time-derived seed

unless explicitly selected by the caller as a non-reproducible execution policy.

The reproducible contract should derive randomness from stable context.

Conceptually:

master seed
    +
program identity
    +
noise model identity
    +
calibration identity
    +
target identity
    +
operation identity
    +
resource identity
    +
shot index

produces the deterministic random stream.

---

25. Parallel RNG independence

A deterministic stochastic implementation must not depend on:

thread scheduling
worker count
task ordering
hash-map iteration order

for semantic random results.

For example:

run(seed, workers=1)

and:

run(seed, workers=64)

must be reproducibly equivalent under the deterministic execution contract.

---

26. Stable identities

Deterministic derivation requires stable identities.

ZQN objects should have stable identifiers for:

NoiseModel
Channel
Fault
Calibration
Observation
Experiment

Quantum resources use canonical IR identities.

Never derive semantic randomness from:

memory address
pointer value
hash-map iteration order
thread ID alone
process ID alone
wall-clock time

---

27. Stable ordering

Any externally observable ordering must be deterministic.

This includes:

- serialized fields;
- canonical collections;
- error reporting;
- hash input;
- generated fault ordering;
- reproducibility metadata.

If an unordered collection is semantically appropriate, serialization must still define canonical ordering where canonical persistence is required.

---

28. Distributed scalability

ZQN must support distributed quantum systems.

A distributed execution may contain:

node
    │
    ├── local quantum resources
    ├── local noise
    ├── local calibration
    └── network resources

Noise identity must therefore be capable of distinguishing:

global execution identity
node identity
resource identity
operation identity
shot identity

without creating another qubit identity system.

---

29. Network noise

Distributed systems require noise models for:

- quantum communication;
- photon loss;
- transport;
- channel attenuation;
- memory during communication;
- entanglement distribution;
- synchronization;
- network-dependent errors.

These must be modeled through abstract resources and links.

They must not be hard-coded around one network topology.

---

30. Temporal scalability

Noise must not be restricted to:

one independent error per gate

ZQN must represent:

time-dependent noise
drift
correlation in time
history dependence
memory effects
non-Markovian behavior

A noise model may therefore require an execution history or environment state.

That state must be explicit.

Do not hide it in global mutable state.

---

31. Spatial scalability

Spatial correlation must support arbitrary resource sets.

For example:

local correlation
regional correlation
long-range correlation
global collective noise
graph-based correlation

The correlation representation must be independent of a fixed number of resources.

---

32. Crosstalk scalability

Crosstalk must not be encoded as a fixed table for a particular machine.

Instead:

operation
+
resources
+
target topology
+
calibration
+
noise model

determines crosstalk behavior.

This permits the same noise semantics to work when a target has:

10 resources
100 resources
10,000 resources
distributed resources

without changing the model's architecture.

---

33. Calibration scalability

Calibration data must be scoped.

A calibration value may apply to:

one resource
resource group
operation type
device
subsystem
target
time interval
environment

It must not be assumed that one global value represents an entire machine.

A "CalibrationSnapshot" should identify:

target
scope
validity interval
parameters
uncertainty
provenance
version

---

34. Calibration lifetime

Calibration must never be assumed permanent.

Every calibration-dependent computation must be able to establish:

calibration identity
calibration version
validity interval

This prevents stale calibration data from silently becoming the basis for a large-scale execution.

---

35. Calibration and scalability

The same program may be executed against:

target A
target B
target C

with different calibration snapshots.

Therefore:

program semantics

must remain independent of:

calibration realization

The calibration subsystem modifies the physical realization, not the source program's meaning.

---

36. Approximation is never implicit

A target may not support the exact requested noise representation.

For example:

requested:
    correlated non-Markovian channel

target:
    independent Markovian Pauli channels

ZQN must not silently substitute the latter.

The result must be one of:

Exact
Approximate
BoundedApproximation
StatisticalApproximation
Unsupported

with explicit policy.

---

37. Approximation contracts

Every approximation must expose, where applicable:

requested model
realized model
approximation method
tolerance
error bound
confidence
assumptions

This is required for scientific reproducibility.

---

38. Scaling through approximation

Large systems may require scalable approximations.

For example:

exact dense representation

may become infeasible while:

sparse representation
tensor representation
trajectory sampling
local approximation
statistical representation

remains possible.

The architecture may select an approximation only when the approximation policy explicitly permits it.

---

39. Approximation must not alter source semantics

The user writes:

Zamani program

once.

If a target cannot implement the requested physical model exactly, the compiler/runtime may:

reject

or:

apply an explicitly declared approximation

but it must not rewrite the user's semantic intent invisibly.

---

40. Interface-driven extensibility

ZQN should use narrow interfaces/traits where behavior needs to vary.

This follows the same general extensibility principle used by MLIR interfaces: generic consumers should depend on capabilities rather than special-case every concrete implementation.

For example, consumers should conceptually ask:

Can this model provide a channel representation?
Can it sample?
Can it provide an error estimate?
Can it represent correlations?
Can it evaluate at time t?

rather than:

if model_type == A
if model_type == B
if model_type == C

---

41. Avoid enum explosion

An extensible architecture must avoid a single giant enum becoming the definition of all future quantum noise.

Avoid making the entire future system depend on:

enum NoiseKind {
    BitFlip,
    PhaseFlip,
    Depolarizing,
    ...
}

as the only representation.

Concrete noise families may use enums internally.

The top-level abstraction must remain extensible.

---

42. Open-world architecture

ZQN must be designed so new noise mechanisms can be introduced without modifying unrelated foundational files.

Adding:

new channel representation

must not require changing:

probability
fault
calibration
routing

unless their public contracts genuinely need a new capability.

This is the "finish one file and do not reopen it" requirement at architectural level.

---

43. File-level completion contract

Every ZQN source file must have a stable contract before implementation.

The contract must define:

1. ownership;
2. non-ownership;
3. public types;
4. public functions;
5. invariants;
6. dependencies;
7. consumers;
8. integration points;
9. error behavior;
10. resource behavior;
11. determinism behavior;
12. serialization behavior;
13. scaling behavior;
14. thread-safety behavior;
15. test obligations.

A downstream implementation must consume the contract instead of redefining it.

---

44. "core/limits.rs"

This file owns operational limits.

It must not own:

- quantum semantics;
- target capacity;
- hardware limits;
- mathematical maximums.

It provides configurable policy values.

Integration:

core::limits
      │
      ├── core::context
      ├── simulation
      ├── characterization
      ├── propagation
      └── io

It must be completed before resource-intensive subsystems are implemented.

---

45. "core/context.rs"

This file provides the common execution context.

It should conceptually combine:

limits
capabilities
determinism
cancellation
provenance
calibration context

It must not own any domain implementation.

Integration:

ZqnContext
    │
    ├── channel operations
    ├── fault generation
    ├── noise evaluation
    ├── simulation
    ├── characterization
    └── serialization

---

46. "core/capabilities.rs"

This file defines capability descriptions.

It must answer:

What can this execution environment represent or process?

It must not answer:

How does a vendor API work?

Capabilities may include:

channel representations
correlated noise
temporal noise
spatial noise
leakage
loss
readout noise
dynamic noise
continuous-time models
calibration

No vendor names belong here.

---

47. "core/provenance.rs"

Every scalable scientific workflow needs provenance.

This file must be capable of identifying:

source
model version
dataset identity
experiment identity
calibration identity
software version
timestamp

Provenance must be immutable once attached to a finalized scientific result.

---

48. "core/ids.rs"

Own only ZQN entity identifiers.

Use canonical IR identifiers for quantum resources.

Integration:

quantum::ir::qubit::QubitId
        │
        ▼
ZQN location/resource reference

NoiseModelId
ChannelId
FaultId
CalibrationId
ObservationId
        │
        ▼
ZQN-owned identity

---

49. Probability scalability

"probability/" must not assume every probability is best represented by "f64".

The system should distinguish:

exact value
approximate value
interval
bound
statistical estimate
confidence interval

This prevents precision assumptions from becoming architecture-wide restrictions.

---

50. Probability distributions

A distribution must support:

validation
normalization
sampling
expectation
variance
support

without assuming:

two outcomes
four outcomes
Pauli outcomes
small finite systems

Large distributions must support lazy or sparse forms where appropriate.

---

51. Channel scalability

"channel/" must support arbitrary subsystem dimension.

The dimension must derive from the represented resource/subsystem.

Never encode:

2x2
4x4
8x8

as architectural limits.

Those are mathematical dimensions for particular systems.

---

52. Kraus representation

"channel/kraus.rs" must support:

- arbitrary supported subsystem dimension;
- validation;
- trace-preservation checks;
- composition;
- tensor product;
- lazy/materialized representation where practical;
- resource-aware conversion.

Dense materialization must be explicitly bounded by resource policy.

---

53. Choi representation

"channel/choi.rs" must support:

- arbitrary supported dimension;
- complete positivity validation;
- trace-preservation validation;
- conversion;
- resource-aware materialization.

It must not assume a fixed qubit count.

---

54. Lindblad scalability

"channel/lindblad.rs" must represent continuous-time processes independently from a particular numerical integrator.

The file owns:

generator semantics
Hamiltonian contribution
jump operators
rates
time dependence

The numerical solver belongs to simulation.

This allows a single model to be processed by different execution strategies.

---

55. Channel composition

"channel/composition.rs" must support:

sequential composition
tensor composition
correlated composition

without imposing fixed arity.

Composition must preserve:

metadata
provenance
representation information
approximation contracts

where semantically applicable.

---

56. Fault scalability

"fault/" must distinguish discrete faults from continuous/noise-channel semantics.

It must support:

arbitrary resource locations
correlation
leakage
erasure
loss
measurement faults
preparation faults
transport faults

The number of affected resources is data.

---

57. Fault streaming

"fault/batch.rs" must not require an entire fault population to fit into memory.

It should support conceptual modes such as:

single fault
bounded batch
stream
iterator
lazy generator

This is essential for large Monte Carlo/QEC workloads.

---

58. Noise model scalability

"noise/model.rs" is the central abstract noise interface.

It must allow consumers to ask for:

description
validation
application
sampling
capabilities

without knowing the concrete model implementation.

The model must not know:

- vendor APIs;
- credentials;
- routing algorithm;
- scheduler implementation;
- decoder implementation.

---

59. Noise application

"noise/application.rs" must define how noise attaches to canonical operations/resources.

Conceptually:

canonical operation
        +
ZQN noise model
        +
execution context
        =
noise application

It must not replace the canonical operation with a second ZQN operation IR.

---

60. Correlation scalability

"noise/correlation.rs" must support arbitrary correlation domains.

The domain may be:

single resource
resource group
graph
region
global system
network

The number of resources must not be encoded into the type's architecture.

---

61. Temporal scalability

"noise/temporal.rs" must support:

stationary
non-stationary
time-dependent
drifting
temporally correlated
history-dependent

Time must be represented using the repository's established timing semantics where available.

Do not invent another incompatible time model.

---

62. Spatial scalability

"noise/spatial.rs" consumes resource/topology information.

It must not own the topology itself.

The dependency should be:

target/resources
       │
       ▼
ZQN spatial model

not:

ZQN
 └── hard-coded topology

---

63. Crosstalk scalability

"noise/crosstalk.rs" must support arbitrary simultaneous operations/resources.

Crosstalk should be evaluable using:

active resources
neighboring resources
operation characteristics
calibration
target topology

without a fixed device-specific table embedded in ZQN.

---

64. Non-Markovian scalability

"noise/non_markovian.rs" must permit explicit environment/history state.

The state must be:

owned by the model or execution context

and not:

global mutable state

This is essential for reproducibility and parallel execution.

---

65. Conditional noise

"noise/conditional.rs" must support noise conditioned on:

operation
measurement result
classical condition
time
calibration
environment
execution state

This enables dynamic circuits and adaptive execution without hard-coded machine assumptions.

---

66. Operation-level scalability

"operations/" provides noise semantics around operation categories.

It must not redefine the canonical operation model.

Conceptually:

IR operation
      │
      ▼
ZQN operation-noise attachment

not:

ZQN operation
      │
      ▼
replacement for IR operation

---

67. Preparation noise

"operations/preparation.rs" must support arbitrary preparation mechanisms.

It must not assume:

always |0>

as the only initialization semantics.

Preparation may be:

ground state
thermal state
prepared state
encoded state
logical state
hardware-specific state

---

68. Reset noise

Reset is its own semantic category.

Do not assume:

reset = measurement + conditional X

unless an explicit target transformation proves that equivalence under the relevant semantics.

---

69. Measurement noise

"operations/measurement.rs" must support:

assignment error
asymmetric readout
correlated readout
state-dependent error
measurement backaction

The number of measured resources must remain dynamic.

---

70. Idle noise

"operations/idle.rs" must be time-aware.

Conceptually:

resource
+
idle duration
+
calibration
+
noise model

determines the idle effect.

This is essential for noise-aware scheduling.

---

71. Pulse noise

"operations/pulse.rs" must remain optional to high-level programs.

The architecture is:

high-level semantic operation
        │
        ▼
scheduled operation
        │
        ▼
pulse realization
        │
        ▼
pulse noise

ZQN must not force every user program into pulse-level programming.

---

72. Transport noise

"operations/transport.rs" must support:

- physical transport;
- communication;
- shuttling;
- photonic transport;
- network links;
- future transport technologies.

No particular technology may become the semantic definition.

---

73. Calibration scalability

The calibration subsystem must be able to represent calibration information for:

one resource
many resources
operation
device
subsystem
network link
logical resource

without a fixed number of resources.

---

74. Characterization scalability

Characterization experiments may become extremely large.

Therefore:

experiment
observation
estimator
uncertainty

must support streaming and incremental processing where mathematically valid.

A characterization system must not require all raw observations to remain in RAM.

---

75. Tomography scalability

Tomography can scale extremely poorly.

Therefore the architecture must distinguish:

protocol definition

from:

execution/materialization strategy

Possible strategies include:

exact
sampled
compressed
sparse
local
distributed
incremental

The selected strategy must be explicit.

---

76. Simulation scalability

ZQN simulation must never imply that every quantum system can be simulated by full state-vector materialization.

The simulator may select:

state vector
density matrix
trajectory
tensor network
stochastic sampling
sparse representation
hardware execution

according to capability and policy.

The ZQN semantic model must remain independent of that choice.

---

77. Monte Carlo scalability

Monte Carlo execution must support:

streaming samples
bounded batches
parallel sampling
deterministic seed partitioning
incremental statistics

It must not require:

Vec<all_samples>

unless explicitly requested.

---

78. Statistical convergence

Sampling-based scalability must report statistical uncertainty.

A result should distinguish:

estimated value
sample count
confidence/uncertainty
sampling method
seed/provenance

from an exact mathematical result.

---

79. Reproducibility

"simulation/reproducibility.rs" must make reproducibility first-class.

A reproducible execution should identify:

program identity
noise model identity
target identity
calibration identity
ZQN version
seed
shot index
numerical configuration

This information must be available to benchmarking and scientific provenance systems.

---

80. Propagation scalability

"propagation/" must support error analysis without assuming that total error is simply:

sum(all gate errors)

Some systems have:

- correlated errors;
- cancellation;
- coherent accumulation;
- non-Markovian effects;
- nonlinear sensitivity.

Therefore propagation must expose explicit assumptions.

---

81. Error budgets

"propagation/error_budget.rs" must be capable of representing:

global budget
per-operation budget
per-resource budget
per-layer budget
per-time budget
per-subsystem budget

without fixed dimensions.

---

82. Fidelity scalability

"propagation/fidelity.rs" must support multiple metrics.

Potential metrics include:

state fidelity
process fidelity
average gate fidelity
entanglement fidelity
distance/bound metrics
classical output distances

No single metric may be treated as universally sufficient.

---

83. Sensitivity scalability

"propagation/sensitivity.rs" must allow:

noise parameter
        ↓
observable/result sensitivity

without assuming a fixed number of parameters.

Parameters are data.

---

84. Accumulation scalability

"propagation/accumulation.rs" must support accumulation across:

operation
layer
time
resource
logical computation
distributed path

and must remain compatible with correlated/no-memory models.

---

85. Target scalability

The target subsystem is the bridge between:

abstract ZQN

and:

actual execution environment

It must expose:

requirements
capabilities
compatibility
lowering
validation

without owning vendor APIs.

---

86. Target requirements

"target/requirements.rs" describes what a computation requires.

Examples:

required channel representation
required correlation support
required temporal behavior
required precision
required approximation tolerance
required calibration validity

It must not contain a specific machine.

---

87. Target capabilities

"target/capabilities.rs" describes what a target can provide.

Examples:

supports correlated noise
supports leakage
supports readout model
supports dynamic noise
supports representation X
supports precision Y

The capability set must be extensible.

---

88. Compatibility

"target/compatibility.rs" answers:

Can requested semantics be realized on this target?

Possible results:

Exact
Approximate
BoundedApproximation
Unsupported

Compatibility checking must occur before execution.

---

89. Lowering

"target/lowering.rs" converts:

abstract ZQN semantics

into:

target-compatible realization

It must not mutate canonical source semantics.

---

90. Validation

"target/validation.rs" must reject:

unsupported representation
missing capability
invalid calibration
expired calibration
unbounded resource request
unsupported approximation

before execution where possible.

---

91. IR integration

"integration/ir.rs" is the primary bridge.

The relationship must be:

quantum::ir
      │
      ▼
ZQN annotation/application
      │
      ▼
noise-aware execution representation

The canonical IR remains authoritative.

ZQN must not create:

ZqnCircuit
ZqnProgram
ZqnGate

as replacement semantic models.

---

92. Routing integration

The repository already contains a dedicated noise-aware routing algorithm under:

src/quantum/routing/algorithms/noise_aware.rs

and its routing architecture explicitly recognizes noise-aware routing as a routing strategy.

The ZQN routing integration must therefore provide a canonical source of:

operation noise estimate
resource error estimate
correlation information
fidelity estimate
duration-dependent noise
crosstalk cost

Conceptually:

ZQN
 │
 ▼
RoutingNoiseEstimate
 │
 ▼
routing::algorithms::noise_aware

The existing ZQN integration architecture already identifies this adapter direction.

ZQN must not implement routing itself.

---

93. Scheduling integration

Scheduling must be able to query:

noise(resource, operation, time)

or the equivalent abstract interface.

The scheduler can then optimize:

duration
+
decoherence
+
crosstalk
+
calibration validity
+
error budget

without implementing the noise mathematics itself.

---

94. QEC integration

The long-term dependency should be:

ZQN
 │
 ▼
physical noise/fault realization
 │
 ▼
QEC adapter
 │
 ├── syndrome generation
 ├── decoder
 ├── correction
 └── logical error analysis

QEC owns:

error-correcting codes
syndrome processing
decoding
logical correction

ZQN owns universal physical noise semantics.

---

95. Migration from existing QEC noise

The existing QEC noise implementation must not be deleted prematurely.

Migration should be:

existing QEC noise
        │
        ▼
compatibility adapter
        │
        ▼
ZQN canonical noise model
        │
        ▼
QEC

This avoids breaking the existing QEC implementation while removing duplicate semantics.

The final architecture should have one authoritative implementation of:

probability
fault
correlation
leakage
erasure
sampling
deterministic stochastic execution

---

96. Memory integration

The existing quantum memory subsystem already has a conceptual place for general channel/noise application.

The dependency should be:

ZQN channel
      │
      ▼
memory/state application

not:

memory redefines ZQN channels

Memory owns state representation and state transitions.

ZQN owns noise semantics.

---

97. Benchmarking integration

Benchmarking must consume:

NoiseObservation
NoiseCharacterization
CalibrationSnapshot
ErrorBudget

rather than recreating separate noise definitions.

The direction is:

execution
   │
   ▼
observations
   │
   ▼
ZQN characterization
   │
   ▼
benchmarking

---

98. Hardware integration

Hardware adapters provide:

TargetCapabilities
CalibrationSnapshot
NoiseObservation

ZQN consumes those abstractions.

The direction must never become:

ZQN → vendor SDK

---

99. Runtime integration

Runtime supplies:

execution context
resource policy
clock
seed policy
cancellation
target capabilities

ZQN supplies:

noise realization
fault stream
channel application
observations
uncertainty

This keeps runtime execution separate from physical-noise semantics.

---

100. Serialization scalability

Serialization must be:

versioned
canonical
deterministic
stream-capable
resource-limited

It must never assume the entire object fits into memory if a streaming representation is possible.

---

101. Canonical serialization

"io/canonical.rs" must provide a stable representation for hashing and reproducibility.

Canonicalization must define:

field ordering
collection ordering
numeric representation
version identity
optional-field behavior

without depending on:

hash-map iteration order
memory address
thread scheduling

---

102. Serialization limits

Deserialization is an untrusted-input boundary.

It must protect against:

allocation bombs
gigantic arrays
gigantic correlation sets
deep nesting
malformed numeric values
NaN/∞ where invalid
integer overflow
pathological recursion

The parser must use explicit resource limits.

---

103. Streaming I/O

For large ZQN models:

deserialize(stream)
serialize(stream)

should be possible where the selected format supports it.

Do not require:

read entire file into memory

as the only implementation strategy.

---

104. Numerical scalability

ZQN must distinguish:

semantic exactness
numerical precision
statistical uncertainty
physical uncertainty
approximation error

These are different concepts.

They must not be collapsed into one floating-point number.

---

105. Numerical validation

Reject invalid values instead of silently repairing them.

Forbidden semantic behavior:

NaN → 0
∞ → maximum
negative probability → absolute value
invalid normalization → automatic normalization

unless the caller explicitly requests a documented repair operation.

---

106. Numerical tolerance

Tolerances must be explicit.

Do not use hidden global constants for:

epsilon
fidelity tolerance
trace tolerance
normalization tolerance

A tolerance belongs to:

validation policy
numerical context
algorithm configuration

as appropriate.

---

107. No unsafe Rust

Every ZQN Rust file must remain safe Rust.

The ZQN root must enforce:

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

No ZQN scalability feature may require:

unsafe
raw pointers
unsafe allocation
unsafe FFI
unsafe synchronization

If a future external interface requires FFI, it must be isolated behind a separately reviewed boundary and must not weaken the ZQN core safety contract.

The current Zamani IR architecture already establishes the no-unsafe Rust contract and Rust 1.97/1.97.1 compatibility.

---

108. Thread safety

Where semantically possible, immutable ZQN models should be:

Send
Sync

and safe to share between workers.

Mutable state must be explicit.

No semantic behavior may depend on:

global mutable cache
global mutable RNG
global calibration

---

109. Cache scalability

Caches must not become hidden semantic state.

A cache key must include all semantic inputs relevant to correctness.

Potential identity:

noise model identity
configuration identity
calibration identity
target identity
representation identity
precision policy

Never key solely on:

model name

---

110. Cache resource policy

Caches must be bounded by explicit policy.

A cache may use:

maximum bytes
maximum entries
TTL
LRU policy
caller-provided storage policy

A cache must never make otherwise valid computation semantically invalid.

---

111. Lazy evaluation

Where computation is expensive, ZQN should support lazy evaluation.

Examples:

lazy correlation expansion
lazy fault generation
lazy sampling
lazy distribution evaluation
lazy serialization

But laziness must not change semantic results.

---

112. Eager versus lazy equivalence

Where mathematically equivalent:

eager evaluation

and:

lazy evaluation

must produce equivalent results under the same deterministic policy and declared numerical tolerance.

---

113. Distributed determinism

For distributed execution, derive deterministic streams from stable identities.

Conceptually:

master seed
    +
node identity
    +
resource identity
    +
operation identity
    +
shot identity

The model must not use:

machine hostname
process start time
thread scheduling

as semantic random inputs.

---

114. Scaling across machine sizes

The intended execution flow is:

same Zamani program
        │
        ▼
same canonical IR
        │
        ▼
same abstract ZQN semantics
        │
        ├──────────────┐
        ▼              ▼
   small target    large target
        │              │
        ▼              ▼
 target lowering   target lowering
        │              │
        ▼              ▼
   execution       execution

Only target-dependent realization changes.

The program itself does not.

---

115. Scaling across technologies

Likewise:

same program
      │
      ▼
same IR
      │
      ▼
same ZQN semantics
      │
 ┌────┼──────┬─────────┐
 ▼    ▼      ▼         ▼
QPU  simulator emulator network

Each target advertises its capabilities.

The compiler/runtime determines compatibility.

---

116. Scaling across representations

A single semantic noise model may be realized as:

small system:
    exact dense representation

larger system:
    sparse representation

very large system:
    tensor/trajectory representation

hardware:
    native characterization

distributed:
    partitioned representation

The semantic model remains unchanged.

---

117. Scaling across execution strategies

ZQN must support:

single-thread execution
multi-thread execution
distributed execution
hardware execution
simulation
emulation
streaming execution
batch execution

without changing the source-level noise definition.

---

118. Scaling tests

The scaling test suite must not test a single hard-coded maximum.

Instead use generated system sizes.

Conceptually:

N = generated resource count

and verify:

result(N)

remains semantically valid for all sizes permitted by the test resource policy.

The architecture must not contain:

MAX_TEST_QUBITS

as a semantic rule.

---

119. Scaling test dimensions

Tests must vary:

resource count
operation count
depth
correlation size
fault count
shot count
distribution size
calibration size
serialization size
network size
worker count

independently where practical.

---

120. Parallel scaling tests

At minimum, deterministic tests should compare:

one worker
multiple workers

for the same execution.

The expected invariant is:

same semantic inputs
+
same deterministic execution policy
=
same result

within explicitly documented numerical tolerance.

---

121. Memory scaling tests

Tests must detect accidental:

O(N²)
O(N³)
O(2^N)

materialization where the algorithm is intended to be:

O(N)

or streaming.

The test must not assume a particular N as an architectural maximum.

---

122. Allocation discipline

Every potentially large allocation must have a reason.

Avoid:

collect()
clone()
to_vec()

in hot paths when a streaming representation is sufficient.

Before materialization, the implementation should have enough information to determine whether the requested allocation is permitted by resource policy.

---

123. Exponential representations

Quantum mathematics can naturally require exponential-size representations.

ZQN must not pretend otherwise.

For example, an exact dense state/channel representation may scale exponentially.

The correct architectural response is:

semantic abstraction
      │
      ├── exact dense realization
      ├── sparse realization
      ├── tensor realization
      ├── sampled realization
      └── hardware realization

not:

artificial MAX_QUBITS

---

124. "Unlimited" APIs

Avoid APIs named or documented as:

unlimited()
infinite()
max_capacity()

when they imply actual infinite resources.

Instead use:

unbounded_semantically

or document:

«No ZQN-imposed finite semantic limit.»

Actual execution remains resource-governed.

---

125. Error messages must be scalable

Errors must identify entities without dumping enormous objects.

For example, prefer:

NoiseModelId
ChannelId
QubitId
PhysicalQubitId
OperationId

over serializing an entire million-resource model into an error message.

---

126. Logging must be bounded

Production logging must never accidentally emit:

entire noise model
entire fault population
entire calibration dataset
entire channel matrix

for large systems.

Logs should use:

identities
counts
summaries
hashes
resource estimates

with explicit debug expansion.

---

127. Debug representations

"Debug" implementations should be bounded where objects can be enormous.

A debug representation must not itself become an accidental memory exhaustion vector.

Large structures should display summaries such as:

resource_count
operation_count
correlation_count
representation
model_id

rather than every element.

---

128. Hashing scalability

Canonical hashing must not require unnecessary duplicate copies.

Prefer streaming/incremental hashing when practical.

Hash identity should depend on semantic content, not:

memory address
process
thread
allocation order

---

129. Hash identity and calibration

If calibration affects a noise model's physical meaning, its identity must be included.

Conceptually:

NoiseModelIdentity
=
hash(
    schema version,
    model semantics,
    configuration,
    calibration identity,
    relevant target identity
)

The exact hash algorithm belongs to the repository's canonical hashing policy.

---

130. Security scalability

Large quantum systems create denial-of-service risks.

Potential attacks include:

gigantic probability tables
gigantic correlation graphs
gigantic channel matrices
deeply nested serialized models
huge fault batches
pathological distributions
nonterminating generators
numerical overflow

Every external-input path must enforce explicit resource policy.

---

131. Security and no unsafe

The no-unsafe contract is necessary but insufficient.

Safe Rust does not automatically prevent:

allocation exhaustion
CPU exhaustion
logical denial of service
numerical denial of service
serialization bombs

Therefore scalability security requires both:

memory safety
+
resource governance

---

132. Fuzzing

Fuzz targets must cover:

probability input
distribution input
channel serialization
channel deserialization
noise serialization
calibration input
fault generation
correlation definitions
target compatibility

Required invariant:

malformed input
    ↓
controlled error

not:

malformed input
    ↓
panic / uncontrolled allocation / infinite loop

---

133. Property testing

Important properties include:

valid probabilities remain within their contract
distributions normalize
identity channel behaves as identity
valid composition preserves declared invariants
serialization round-trips
canonicalization is deterministic
deterministic sampling is reproducible

---

134. Differential testing

Equivalent channel representations should be compared.

For example:

Kraus
  ↕
Choi
  ↕
superoperator
  ↕
Pauli transfer

when mathematically applicable.

The observable behavior must agree within declared numerical tolerance.

---

135. Regression testing

Every discovered scalability bug must receive a regression test.

Examples:

resource count overflow
large correlation set
parallel nondeterminism
serialization ordering
calibration expiry
large allocation
numeric overflow

---

136. Integration-test contract

Integration tests must verify that:

IR
  +
ZQN
  +
routing
  +
scheduling
  +
QEC
  +
hardware abstractions

compose without redefining identities or semantics.

---

137. Routing integration test

A routing test should verify:

IR operation
      ↓
ZQN noise estimate
      ↓
noise-aware router
      ↓
routing decision

and ensure that routing does not need a second independent noise model.

---

138. Scheduling integration test

Verify:

IR operation
      ↓
ZQN duration/noise information
      ↓
scheduler
      ↓
schedule

with no ZQN dependency on the scheduler implementation.

---

139. QEC integration test

Verify:

ZQN physical noise
      ↓
QEC adapter
      ↓
fault/syndrome representation
      ↓
QEC decoder

and verify that QEC does not create a competing probability/noise identity system.

---

140. Hardware integration test

Verify:

hardware adapter
      ↓
TargetCapabilities
CalibrationSnapshot
NoiseObservation
      ↓
ZQN

and ensure ZQN does not require vendor SDK knowledge.

---

141. Benchmarking integration test

Verify:

execution
      ↓
observations
      ↓
ZQN characterization
      ↓
benchmarking

with provenance and uncertainty retained.

---

142. File dependency order

The implementation order must preserve independent completion.

Recommended order:

1. core/error.rs
2. core/version.rs
3. core/ids.rs
4. core/limits.rs
5. core/metadata.rs
6. core/provenance.rs
7. core/capabilities.rs
8. core/context.rs

9. probability/*
10. channel/representation.rs
11. channel/kraus.rs
12. channel/choi.rs
13. channel/process_matrix.rs
14. channel/pauli.rs
15. channel/stochastic.rs
16. channel/lindblad.rs
17. channel/composition.rs

18. fault/*
19. noise/*
20. operations/*

21. calibration/*
22. characterization/*

23. simulation/*
24. propagation/*

25. target/*

26. integration/*

27. io/*

28. prelude.rs
29. mod.rs
30. tests/*

The parent modules should not declare modules before their implementations/contracts are actually present.

---

143. Why foundational files come first

Foundational files establish:

errors
identities
limits
versioning
metadata
provenance
capabilities
context

Once those contracts are frozen, downstream files can depend on them without repeatedly reopening their definitions.

This directly satisfies the project requirement that a file should be considered complete once its contract is established and implemented.

---

144. Dependency minimization

Each file must depend on the smallest possible abstraction.

For example:

probability

should not depend on:

routing
hardware
QEC
simulation

Likewise:

core

must remain below domain implementations.

This prevents circular dependency pressure.

---

145. No "future module" dependency

A file must never import a module merely because the architecture document says it will exist later.

Instead:

contract first
implementation second
integration third

This prevents the current repository from becoming permanently uncompilable during staged development.

---

146. Module composition

"mod.rs" files must primarily provide:

module declarations
public API selection
documentation

They should not become implementation dumping grounds.

The domain implementation belongs in the dedicated files.

---

147. "prelude.rs"

The ZQN prelude must expose only stable high-value abstractions.

It should not glob-export the entire subsystem.

This avoids accidental API coupling and name collisions as ZQN evolves.

---

148. API stability

Public module paths should be treated as API.

Breaking changes require explicit compatibility policy.

Internal implementation changes should not require downstream modules to change when the public contract remains stable.

---

149. Versioning

ZQN must maintain:

semantic version
schema version
compatibility version

These are different concepts.

A serialization schema change does not necessarily mean a semantic model change.

---

150. Schema evolution

New fields should be designed so older readers can reject or safely ignore them according to the declared compatibility policy.

Breaking changes require a major schema/version transition.

No consumer should have to guess what a serialized model means.

---

151. Backward compatibility

Compatibility adapters belong in:

io/compatibility.rs

or:

core/compatibility boundary

as appropriate.

Compatibility logic must not contaminate mathematical semantics.

---

152. API compatibility with existing repository

ZQN must integrate with existing canonical repository types instead of introducing replacements.

Especially:

quantum::ir::qubit::QubitId
quantum::ir::qubit::PhysicalQubitId

must remain canonical.

Existing routing, memory, QEC, hardware, benchmarking and IR components should consume ZQN through adapters.

---

153. Existing routing architecture

The repository already exposes:

routing::algorithms::noise_aware

and routing documentation identifies noise-aware routing as a distinct routing strategy.

Therefore the ZQN scalability architecture must preserve this separation:

ZQN = noise semantics
routing = placement algorithm

---

154. Existing routing topology

Routing owns:

topology
mapping
moves
placement

ZQN consumes topology information only when needed for noise evaluation.

The dependency must not reverse into:

ZQN owns routing topology

---

155. Existing IR architecture

The current IR architecture explicitly separates:

semantic WHAT

from:

target
routing
scheduling
hardware
simulation
QEC
optimization

and requires canonical qubit identity through "quantum::ir::qubit".

ZQN must preserve these boundaries exactly.

---

156. QIR interoperability principle

QIR's architecture explicitly separates:

profile capabilities

from:

quantum instruction set

and supports dynamic allocation for cases where resource counts are not known statically.

ZQN should adopt the same principle conceptually:

semantic requirement
      │
      ▼
target capability
      │
      ▼
compatible realization

ZQN itself does not need to become QIR.

---

157. MLIR interoperability principle

MLIR's interface model demonstrates the importance of allowing analyses and transformations to depend on generic capabilities rather than hard-coded knowledge of every concrete operation/dialect.

ZQN should apply the same architectural lesson:

consumer
   │
   ▼
capability/interface
   │
   ▼
concrete noise implementation

rather than:

consumer
   │
   ├── model A special case
   ├── model B special case
   ├── model C special case
   └── model D special case

---

158. Scaling across future standards

ZQN must not make current interchange standards its semantic definition.

It may later interoperate with:

QIR
MLIR
OpenQASM
other quantum IRs
vendor representations
scientific data formats

but these are interchange/lowering layers.

The canonical Zamani semantic boundary remains:

quantum::ir

and the canonical Zamani noise boundary remains:

quantum::zqn

---

159. No source-language dependence

ZQN must not know how a Zamani source program was written.

It may receive semantic information from:

frontend
compiler
IR

but must not parse:

Zamani source
OpenQASM source
Python
vendor DSL

inside its mathematical core.

---

160. No application dependence

ZQN must work equally for:

chemistry
optimization
cryptography
simulation
machine learning
communication
error correction
scientific computation

without knowing application semantics.

---

161. No algorithm dependence

ZQN must not depend on:

VQE
QAOA
Shor
Grover
quantum simulation

or any particular algorithm.

Algorithms consume ZQN information when needed.

---

162. No optimizer dependence

Optimization may use:

noise estimates
fidelity estimates
error budgets

but ZQN must not own optimization passes.

The dependency remains:

ZQN
  ↓
optimization consumer

not:

ZQN
  ↓
optimizer implementation

---

163. No scheduler dependence

ZQN provides physical timing/noise information.

Scheduling determines the schedule.

This allows future schedulers to be added without modifying ZQN's mathematical core.

---

164. No decoder dependence

QEC decoders consume physical/logical error information.

ZQN must not know which decoder is selected.

This allows:

MWPM
belief propagation
union-find
tensor-network decoders
neural decoders
future decoders

without ZQN changes.

---

165. No simulator dependence

The same ZQN model must be usable by:

state-vector simulator
density-matrix simulator
tensor simulator
trajectory simulator
hardware

without changing the model.

---

166. No hardware dependence

A hardware backend should implement or expose the necessary target abstractions.

ZQN must remain hardware-independent.

---

167. Scalability of observations

Raw observations may become enormous.

Therefore characterization must support:

streaming observation
incremental estimator
compressed observation
distributed aggregation

where statistically valid.

---

168. Incremental statistics

Statistics should permit online updates where mathematically appropriate.

Examples:

count
mean
variance
histogram
confidence interval

This prevents storing all samples solely to calculate simple aggregate metrics.

---

169. Distributed aggregation

Statistics should support merging partial results when mathematically valid.

Conceptually:

worker A → partial statistics
worker B → partial statistics
worker C → partial statistics
               │
               ▼
        merged statistics

The merge must preserve the statistical contract.

---

170. Reproducible distributed statistics

When deterministic execution is requested, the reduction order must either be deterministic or the numerical tolerance must explicitly account for order-dependent floating-point reductions.

Do not claim bitwise identity unless the implementation actually guarantees it.

---

171. Precision scalability

A large computation may require different numerical strategies.

ZQN must permit:

exact arithmetic
high precision
standard precision
approximate precision
interval arithmetic

where supported.

Precision selection must be explicit.

---

172. Precision must not leak into semantics

A channel's mathematical meaning must not change merely because:

f32
f64
high precision

is selected.

The representation may change.

The semantic object does not.

---

173. Error propagation must expose uncertainty

A result should distinguish:

physical uncertainty
statistical uncertainty
numerical error
approximation error

This is necessary for trustworthy large-scale scientific computation.

---

174. Provenance across scaling

When a computation is lowered from:

small target

to:

large target

the resulting artifact should retain provenance identifying:

source program
IR identity
ZQN model identity
target identity
calibration identity
execution identity

---

175. Resource-aware planning

Before an expensive realization, ZQN should be able to estimate:

memory
operations
samples
representation size

where practical.

This enables the runtime to reject impossible requests before starting an expensive operation.

---

176. Resource estimation is advisory

An estimate must not be treated as a universal guarantee.

Actual resource use can depend on:

runtime
data distribution
adaptive algorithms
target behavior
compression

Therefore distinguish:

estimated resource usage

from:

enforced resource limit

---

177. Adaptive execution

Some ZQN operations may adapt based on observations.

For example:

sampling
characterization
calibration
Monte Carlo

The adaptation policy must be explicit and deterministic when deterministic mode is requested.

---

178. Adaptive execution and reproducibility

A deterministic adaptive algorithm must use stable decisions.

Avoid decisions based on:

unordered iteration
thread timing
non-deterministic reduction

unless the execution mode explicitly allows nondeterminism.

---

179. Global state prohibition

ZQN must not use global mutable semantic state for:

current noise model
current calibration
current RNG
current target
current limits

All semantic context must be passed explicitly or through an immutable execution context.

---

180. Environment isolation

Two simultaneous executions must be able to use:

different noise models
different targets
different calibrations
different limits
different seeds

without interfering with each other.

This is mandatory for concurrent compilation and execution.

---

181. Reentrancy

ZQN operations should be reentrant.

A ZQN computation started inside another ZQN computation must not corrupt:

global state
RNG
calibration
limits
metadata

---

182. Snapshot semantics

Calibration and configuration should be snapshot-based where reproducibility matters.

A finalized execution should refer to immutable:

CalibrationSnapshot

rather than a mutable live calibration object.

---

183. Live versus snapshot calibration

A live calibration service may exist outside ZQN.

ZQN receives a snapshot:

live hardware calibration
        │
        ▼
CalibrationSnapshot
        │
        ▼
ZQN execution

This prevents a calibration update halfway through a deterministic computation from silently changing its semantics.

---

184. Scaling and mutation

Immutable models are preferred.

If mutation is required:

builder
      ↓
validated immutable model

is preferable to:

globally mutable model

This makes concurrency and reproducibility substantially safer.

---

185. Builders

Builders may be used for complex models.

A builder should:

construct
validate
freeze

into an immutable semantic object.

Validation should occur before the object becomes externally visible.

---

186. Validation layers

Validation should occur at appropriate boundaries:

construction validation
    ↓
model validation
    ↓
target compatibility validation
    ↓
execution validation

Do not make every low-level operation repeat the entire validation cost unnecessarily.

---

187. Validation scalability

Validation must not accidentally materialize huge structures merely to check a property that can be checked incrementally.

Prefer:

streaming validation
incremental validation
short-circuit validation

where possible.

---

188. Short-circuit behavior

If an invalid property is discovered early, validation should stop unless the caller explicitly requests complete diagnostic collection.

This prevents unnecessary work on enormous models.

---

189. Diagnostic scalability

Complete diagnostics can themselves become enormous.

Therefore support:

first error
bounded errors
all errors

according to explicit policy.

---

190. Error aggregation

An aggregated error must have a bounded policy.

Never allow:

one million repeated errors

to be generated merely because a large input contains one million instances of the same invalid condition.

---

191. API ergonomics at scale

Public APIs should make expensive operations visible.

Prefer names/documentation that distinguish:

validate()
validate_with_limits()
materialize()
materialize_with_limits()
sample()
sample_with_context()

where necessary.

The caller must be able to understand when an operation may become expensive.

---

192. No accidental copies

Large semantic objects should avoid unnecessary deep copies.

Use:

references
Arc
Cow
iterators
views
slices
streaming abstractions

where appropriate and safe.

This does not mean every type must use "Arc".

Use the simplest safe ownership model that satisfies the contract.

---

193. Ownership and lifetimes

The API must avoid lifetime designs that make large-scale integration unnecessarily fragile.

Prefer owned immutable semantic objects at subsystem boundaries where that improves independent use.

Borrowing may be used for short-lived operations.

---

194. "Arc" usage

"Arc" may be used for immutable shared models where useful.

It must not become a substitute for clear ownership.

Do not use:

Arc<Mutex<...>>

as a universal architecture.

Prefer immutable data and explicit state transitions.

---

195. Async considerations

If asynchronous execution is eventually introduced, ZQN semantics must remain independent of the async runtime.

ZQN must not hard-code:

Tokio
async-std
specific executor

into the mathematical core.

Execution adapters may integrate with the repository's chosen runtime.

---

196. No filesystem dependence

Core ZQN semantics must not require:

filesystem
network
environment variables
process-global state

to function.

I/O belongs to "io/" or external adapters.

---

197. No network dependence

ZQN must not require network access for:

channel mathematics
probability
fault semantics
validation
deterministic simulation

Hardware services may exist outside ZQN.

---

198. Offline capability

A complete ZQN model must be usable offline once all required model/calibration data is locally available.

This is important for:

simulation
testing
reproducibility
scientific analysis

---

199. Cross-platform scalability

The ZQN core must not depend on:

Linux-only APIs
Windows-only APIs
macOS-only APIs
specific CPU architecture
specific GPU architecture

unless isolated in a backend-specific implementation.

---

200. Integer-width independence

Do not assume a universal maximum that arises merely from:

u32
usize

where a larger semantic identity is required.

Use repository-defined identity types and appropriately sized counters.

Resource policies may still use bounded integer types, but their bounds are operational.

---

201. Overflow handling

Every potentially overflowing calculation must use checked or otherwise validated arithmetic.

Never rely on release-mode wrapping for resource calculations.

This includes:

matrix dimensions
allocation sizes
sample counts
correlation counts
serialized sizes
operation counts

---

202. Dimension arithmetic

Before allocating a representation whose size depends on multiple dimensions:

validate dimensions
check multiplication
check allocation policy

before materialization.

---

203. No implicit exponential allocation

Operations such as:

tensor product
Kronecker product
Choi construction
dense state/channel conversion

must not silently allocate enormous structures.

They must pass through resource policy.

---

204. Sparse scalability

Where the mathematical structure allows sparsity, ZQN should support sparse representations.

Sparse representation must not be assumed universally valid.

Its use must preserve semantic correctness.

---

205. Tensor scalability

Tensor representations should remain abstract from any one tensor-network implementation.

The semantic layer should define what is represented.

The simulator/execution layer selects the tensor implementation.

---

206. Sampling scalability

Sampling is often the only practical strategy for very large systems.

ZQN must therefore treat sampling as a first-class execution representation.

Sampling must expose:

seed
sample count
sampling algorithm
uncertainty

as appropriate.

---

207. Streaming sampling

Sampling APIs should permit:

for sample in sampler {
    ...
}

or equivalent streaming semantics.

Do not force all samples into memory.

---

208. Shot scalability

Shot count must be runtime/configuration data.

Never encode:

MAX_SHOTS

as a semantic limit.

A caller may impose a finite shot limit through resource policy.

---

209. Fault-stream scalability

The same principle applies to faults.

A million or billion possible fault events must not require one giant in-memory vector if the execution only needs a stream.

---

210. Correlation expansion

Some correlated models have compact representations whose explicit expansion is enormous.

ZQN must preserve the compact form when possible.

Expansion must be:

explicit
resource-limited
cancelable

---

211. Global noise models

A global model may conceptually apply to an entire machine.

It must not require enumerating every resource merely to state the model.

For example:

global correlated environment

may be represented symbolically.

Consumers can request a realization appropriate to their capabilities.

---

212. Local noise models

Likewise, local noise must not require constructing a global representation.

A consumer operating on one resource should be able to query the relevant local noise without materializing unrelated resources.

---

213. Hierarchical noise

Large systems may naturally use:

global model
    ↓
regional model
    ↓
subsystem model
    ↓
resource model
    ↓
operation model

The architecture must allow hierarchical composition.

---

214. Hierarchical calibration

Calibration may similarly be hierarchical:

target
 └── subsystem
      └── resource
           └── operation

Resolution should be selected according to the operation being evaluated.

---

215. Conflict resolution

If multiple noise/calibration sources apply, precedence must be explicit.

Do not rely on:

insertion order
hash-map order
file order

unless that ordering is part of the declared semantic contract.

---

216. Composition rules

Noise composition must define:

order
scope
precedence
correlation behavior
calibration precedence
approximation behavior

before implementation.

---

217. Avoid hidden transformations

ZQN must not silently:

normalize
truncate
clip
approximate
drop correlations
drop metadata
drop uncertainty

without an explicit policy.

---

218. Metadata scalability

Metadata may become large.

Metadata APIs must support:

bounded metadata
structured metadata
lazy metadata where appropriate

and must not make huge arbitrary strings a mandatory part of every object.

---

219. Provenance scalability

Provenance should use references/identities rather than copying entire datasets into every object.

For example:

dataset_id
experiment_id
calibration_id

rather than embedding gigabytes of raw source data.

---

220. Observation identity

Every observation should be identifiable without relying on array position alone.

This matters for distributed and incremental characterization.

---

221. Experiment identity

Experiments must be reproducible from:

experiment identity
protocol
configuration
target
calibration
seed

where stochastic execution is involved.

---

222. Large experiment execution

Characterization must be able to execute:

one experiment
batch
stream
distributed experiment set

without changing the semantic experiment definition.

---

223. Benchmark scalability

Benchmarking must not assume:

small circuits
small devices
small shot counts

The benchmark definition remains abstract.

Execution policy determines how much data is collected.

---

224. Benchmark provenance

Every benchmark result should be able to identify:

program
noise model
target
calibration
execution policy
sample count
ZQN version

where relevant.

---

225. Integration stability

A completed ZQN file should not require reopening when a downstream integration is added, provided the integration uses the published contract.

For example:

new router

should consume:

RoutingNoiseEstimate

rather than requiring changes to:

channel.rs
probability.rs
fault.rs

---

226. New hardware stability

Adding a new hardware technology should require:

new hardware adapter

and possibly:

new capability implementation

but should not require rewriting the fundamental noise model.

---

227. New noise model stability

Adding a new noise family should normally require:

new noise implementation

plus the necessary registration/export integration.

It should not require modifying unrelated:

routing
QEC
benchmarking

implementations.

---

228. New representation stability

Adding a new channel representation should normally require:

new representation implementation

plus explicit conversion/interface support.

Existing semantic models should remain valid.

---

229. New quantum modality stability

Adding support for a future quantum technology must not require redefining:

QubitId

or the existing channel/fault abstractions.

The resource model must already permit extension.

---

230. Definition of scalable

ZQN is architecturally scalable when all of the following are true:

no semantic machine-size ceiling
no fixed topology
no fixed gate arity
no vendor dependence
no duplicate qubit identity
no mandatory full-system materialization
no hidden RNG
no global mutable semantics
no implicit approximation
no unsafe Rust
explicit resource governance
streaming where appropriate
representation polymorphism
target capability negotiation
deterministic execution
versioned persistence

---

231. Definition of production ready

ZQN must not be declared production-ready merely because:

cargo test

passes.

Production readiness requires:

Semantic

Mathematical invariants are explicit and tested.

Architectural

Dependency direction is enforced.

Scalability

No artificial machine-size ceiling exists.

Numerical

Invalid and non-finite values are controlled.

Determinism

Reproducible stochastic execution exists.

Resource safety

Expensive operations are policy-governed.

Security

Untrusted input cannot trivially exhaust resources.

Interoperability

IR, routing, scheduling, QEC, memory, hardware and benchmarking have stable adapters.

Scientific reproducibility

Provenance and uncertainty are preserved.

Compatibility

Schemas and versions are explicit.

Maintainability

Files have stable ownership contracts.

---

232. Mandatory CI checks

The ZQN CI pipeline should eventually verify:

cargo fmt --check
cargo check
cargo test
cargo clippy
cargo doc

plus:

unsafe-code rejection
property tests
fuzz tests
determinism tests
scaling tests
serialization tests
integration tests

The exact repository CI commands must follow the repository's existing workflow configuration.

---

233. Unsafe-code CI invariant

CI must fail if ZQN introduces unsafe code.

The source-level contract is:

#![forbid(unsafe_code)]

This must remain enforced.

---

234. API documentation

All public ZQN APIs must document:

purpose
arguments
return value
errors
complexity
resource behavior
determinism
thread safety
scaling

For expensive operations, documentation must explicitly say whether the operation:

allocates
streams
materializes
parallelizes

---

235. Mathematical documentation

Each channel/fault/noise family must document:

mathematical definition
parameter domain
invariants
composition behavior
representation
approximation behavior

This belongs primarily in "SEMANTICS.md" and the relevant source module documentation.

---

236. Scalability documentation

Each source file must answer:

What scales?
What does not?
What is materialized?
What is lazy?
What are the resource costs?
What are the policy controls?

A file is incomplete if these are unknown.

---

237. File-level Definition of Done

A ZQN file is complete only when:

[ ] ownership documented
[ ] non-ownership documented
[ ] dependencies frozen
[ ] consumers identified
[ ] public API complete
[ ] invariants complete
[ ] errors complete
[ ] limits complete
[ ] deterministic behavior complete
[ ] serialization behavior complete
[ ] scaling behavior complete
[ ] concurrency behavior complete
[ ] tests complete
[ ] documentation complete
[ ] integration contract complete

Once these are satisfied, downstream files must consume the contract instead of reopening the completed file merely to accommodate avoidable architectural omissions.

---

238. Directory-level Definition of Done

A directory is complete when:

all files have contracts
all module exports are stable
all internal dependencies compile
all invariants are tested
all public APIs are documented
all integration points are specified

and:

cargo check
cargo test
cargo clippy

pass for the applicable repository configuration.

---

239. ZQN completion gates

The subsystem should progress through:

Gate 0
Foundational contracts

Gate 1
Probability

Gate 2
Channel mathematics

Gate 3
Fault semantics

Gate 4
Noise composition

Gate 5
Operations

Gate 6
Calibration

Gate 7
Characterization

Gate 8
Simulation

Gate 9
Propagation

Gate 10
Target compatibility

Gate 11
Repository integrations

Gate 12
Serialization

Gate 13
Scalability/security testing

Gate 14
Production release

No later gate should force redesign of a prior foundational contract unless a genuine mathematical or repository-level defect is discovered.

---

240. Final scalability architecture

The final architecture is:

                         ZAMANI PROGRAM
                               │
                               ▼
                       QUANTUM FRONTEND
                               │
                               ▼
                      ┌─────────────────┐
                      │   QUANTUM IR    │
                      │                 │
                      │ canonical WHAT  │
                      └────────┬────────┘
                               │
                  ┌────────────┼────────────┐
                  │            │            │
                  ▼            ▼            ▼
             algorithms   optimization   analysis
                  │            │
                  └──────┬─────┘
                         ▼
                ┌──────────────────┐
                │       ZQN        │
                │                  │
                │ probability      │
                │ uncertainty      │
                │ channels         │
                │ faults           │
                │ noise models     │
                │ correlations     │
                │ calibration      │
                │ characterization │
                │ propagation      │
                │ provenance       │
                └────────┬─────────┘
                         │
             ┌───────────┼──────────────┐
             │           │              │
             ▼           ▼              ▼
          ROUTING    SCHEDULING        QEC
             │           │              │
             └───────────┼──────────────┘
                         ▼
                TARGET CAPABILITIES
                         │
                         ▼
                TARGET LOWERING
                         │
              ┌──────────┼──────────┐
              ▼          ▼          ▼
          SIMULATOR      QPU      EMULATOR
              │          │          │
              └──────────┼──────────┘
                         ▼
                      RUNTIME
                         │
                         ▼
                    OBSERVATIONS
                         │
              ┌──────────┼──────────┐
              ▼          ▼          ▼
       CHARACTERIZATION BENCHMARKING ANALYSIS
              │
              ▼
         CALIBRATION
              │
              └──────────────► ZQN

---

241. The ultimate write-once contract

The complete Zamani quantum stack must obey:

WRITE ONCE
     │
     ▼
CANONICAL SEMANTICS
     │
     ▼
ABSTRACT NOISE
     │
     ▼
CAPABILITY NEGOTIATION
     │
     ▼
TARGET REALIZATION

The following must not change merely because the machine becomes larger:

Zamani source semantics
canonical IR semantics
ZQN semantic definitions
noise-model meaning
mathematical invariants
public identity contracts

What may change is:

target
resource count
topology
calibration
representation
execution strategy
parallelism
simulation method
hardware realization
resource policy

---

242. The ultimate "atom to everywhere" contract

ZQN must be capable of progressing conceptually through:

one resource
      ↓
small subsystem
      ↓
device
      ↓
multi-device system
      ↓
distributed quantum system
      ↓
quantum network
      ↓
logical/fault-tolerant system
      ↓
future heterogeneous quantum architecture

without changing its fundamental semantic architecture.

The limiting factor is allowed to be:

available resources
target capabilities
algorithmic complexity
physical feasibility
explicit execution policy

but never:

an arbitrary ZQN machine-size constant

---

243. Final non-negotiable rules

The following are permanent ZQN architectural invariants:

1. "quantum::ir" remains the canonical quantum semantic IR.

2. "quantum::ir::qubit::QubitId" remains the canonical logical/canonical qubit identity where applicable.

3. "quantum::ir::qubit::PhysicalQubitId" remains the canonical physical qubit identity where applicable.

4. ZQN must never define a competing "QubitId".

5. ZQN must contain no artificial semantic maximum for machine size.

6. Operational resource limits are allowed and must be explicit.

7. Resource limits are never quantum-machine capability definitions.

8. No fixed qubit count may be assumed.

9. No fixed gate arity may be assumed.

10. No fixed topology may be assumed.

11. No vendor-specific semantics may exist in ZQN.

12. No hidden global RNG may exist.

13. Deterministic stochastic execution must be reproducible.

14. Parallel execution must not silently alter deterministic semantics.

15. No unsafe Rust is permitted.

16. No mandatory full-system materialization may exist where streaming/lazy processing is semantically possible.

17. Exactness and approximation must be explicitly distinguished.

18. Numerical error, statistical uncertainty, physical uncertainty and approximation error must remain distinguishable.

19. Calibration must be versioned and scoped.

20. Provenance must survive execution and serialization.

21. Serialization must be versioned and deterministic.

22. Untrusted input must be resource-governed.

23. ZQN must not own routing algorithms.

24. ZQN must not own scheduling algorithms.

25. ZQN must not own QEC decoders.

26. ZQN must not own hardware vendor APIs.

27. ZQN must not own frontend syntax.

28. ZQN must not become a second quantum IR.

29. New quantum technologies must be addable without redesigning the foundational model.

30. New noise models must be addable without modifying unrelated subsystems.

31. New target machines must be addable through capability/adapter boundaries.

32. Completed foundational files must expose stable contracts so downstream additions do not require avoidable re-editing.

33. "Infinity" means no artificial finite semantic ceiling, not infinite physical resources.

34. Every concrete execution remains finite and resource-governed.

35. A program written once must remain semantically portable across compatible machine sizes and technologies.

---

244. Final architectural statement

ZQN is therefore not:

a collection of hard-coded error rates

and not:

a qubit simulator

and not:

a vendor noise API

and not:

a second quantum IR

It is:

                    ZQN
                     │
                     ▼
       UNIVERSAL QUANTUM UNCERTAINTY
                     │
        ┌────────────┼────────────┐
        ▼            ▼            ▼
     CHANNELS       FAULTS     UNCERTAINTY
        │            │            │
        └────────────┼────────────┘
                     ▼
               NOISE MODELS
                     │
        ┌────────────┼────────────┐
        ▼            ▼            ▼
    CALIBRATION  CHARACTERIZATION PROPAGATION
        │            │            │
        └────────────┼────────────┘
                     ▼
             TARGET CAPABILITIES
                     │
                     ▼
               EXECUTION

with the governing principle:

«ZQN defines no artificial finite boundary on the size, topology, technology, or complexity of a quantum system. It represents physical uncertainty semantically and delegates finite resource constraints to explicit execution policies and target capabilities.»

Therefore the intended Zamani contract is:

                 ONE PROGRAM
                     │
                     ▼
               ONE SEMANTICS
                     │
                     ▼
                ONE ZQN MODEL
                     │
                     ▼
        ┌────────────┼────────────┐
        ▼            ▼            ▼
      tiny         large       distributed
        │            │            │
        ▼            ▼            ▼
     target A      target B     target C
        │            │            │
        └────────────┼────────────┘
                     ▼
              SAME PROGRAM
              SAME INTENT
              SAME SEMANTICS

The only legitimate constraints are the resources and capabilities actually available to the particular compilation/execution.

That is the production scalability contract for "src/quantum/zqn/".This version is deliberately a contract document rather than another architecture wishlist: it fixes the ownership boundaries before implementation, explicitly integrates quantum::ir::qubit, routing, scheduling, QEC, memory, hardware, benchmarking, runtime, serialization and the existing ZQN directories, and treats resource limits as execution policy rather than machine-size limits. The current repository's ZQN tree already contains the major subsystem boundaries this contract is designed around.