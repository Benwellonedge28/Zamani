Worked for 14s

I inspected the current repository structure and the existing ZQN contracts rather than treating the earlier empty-directory description as still current. The repository now already contains substantial ZQN subsystem boundaries such as core, probability, channel, fault, noise, operations, calibration, characterization, simulation, propagation, target, integration, and io; the existing ZQN code also explicitly establishes quantum::ir::qubit::QubitId and PhysicalQubitId as canonical identities.

I also incorporated the repository's quantum composition-root rules, especially that quantum::ir remains the canonical semantic boundary and that ZQN must not become a second IR.  This is consistent with QIR's many-to-many language/processor architecture and its separation of backend capabilities from the program representation.  MLIR's interface/dialect model likewise supports keeping transformations dependent on generic interfaces rather than concrete implementations. 

Below is the complete proposed src/quantum/zqn/ARCHITECTURE.md. It deliberately corrects the earlier architecture where necessary, including the distinction between semantic limits and resource policies, canonical qubit identity, future quantum modalities, deterministic stochastic execution, approximation contracts, dependency direction, and integration boundaries.

Zamani Quantum Noise (ZQN) Architecture

Path: "src/quantum/zqn/ARCHITECTURE.md"
Subsystem: "crate::quantum::zqn"
Language: Rust
Edition: Rust 2021
Supported toolchains: Rust 1.97 / Rust 1.97.1
Safety requirement: "#![forbid(unsafe_code)]"
Status: Production architecture
Primary principle: Write once, scale everywhere

---

1. Purpose

ZQN means Zamani Quantum Noise.

ZQN is the authoritative subsystem for representing, validating, composing, applying, characterizing, propagating, and integrating quantum uncertainty and physical imperfection throughout the Zamani quantum stack.

ZQN exists to answer:

«What physical uncertainty, noise, fault, calibration uncertainty, stochastic effect, environmental effect, or approximation affects this quantum computation?»

ZQN does not answer:

«What does the quantum program mean?»

That question belongs to:

crate::quantum::ir

The canonical architecture is therefore:

                    Zamani source
                         │
                         ▼
                ┌─────────────────┐
                │ quantum::frontend│
                └────────┬────────┘
                         │
                         ▼
                ┌─────────────────┐
                │   quantum::ir  │
                │ canonical WHAT │
                └────────┬────────┘
                         │
          ┌──────────────┼──────────────┐
          │              │              │
          ▼              ▼              ▼
      algorithms    optimization     analysis
          │              │
          └───────┬──────┘
                  ▼
             ┌─────────┐
             │   ZQN   │
             │ physical│
             │ noise   │
             │ model   │
             └────┬────┘
                  │
      ┌───────────┼────────────┐
      │           │            │
      ▼           ▼            ▼
   routing    scheduling       QEC
      │           │            │
      └───────────┼────────────┘
                  ▼
          target / hardware
                  │
          ┌───────┼────────┐
          ▼       ▼        ▼
       simulator  QPU    emulator
          │       │        │
          └───────┼────────┘
                  ▼
               runtime
                  │
                  ▼
             observations
                  │
          ┌───────┼─────────────┐
          ▼       ▼             ▼
   characterization benchmarking analysis
          │
          ▼
      calibration
          │
          └──────────────► ZQN

---

2. Architectural invariants

The following rules are mandatory.

2.1 ZQN is not a second Quantum IR

ZQN must never become a competing representation of the quantum program.

The canonical semantic representation remains:

crate::quantum::ir

ZQN may reference canonical IR operations, resources, identifiers, timing information, and physical realizations.

ZQN must not introduce:

ZqnCircuit
ZqnProgram
ZqnGate
ZqnQubitId
ZqnMeasurementAst

as replacements for canonical quantum IR concepts.

ZQN represents noise semantics, not the whole quantum program.

---

2.2 Canonical qubit identity

ZQN must use the canonical quantum IR identity types wherever qubit identity is required:

crate::quantum::ir::qubit::QubitId
crate::quantum::ir::qubit::PhysicalQubitId

ZQN must never define:

struct QubitId(...);

or:

type QubitId = usize;

or any semantically equivalent competing identity.

The repository explicitly establishes "quantum::ir::qubit" as the canonical identity boundary.

ZQN-owned IDs are permitted only for ZQN objects:

NoiseModelId
ChannelId
FaultId
CalibrationId
CharacterizationId
ExperimentId
ObservationId
NoiseSnapshotId

Those identify ZQN entities, not quantum resources.

---

2.3 No semantic machine-size ceiling

ZQN must contain no architectural constants such as:

MAX_QUBITS
MAX_PHYSICAL_QUBITS
MAX_QUBIT_INDEX
MAX_GATES
MAX_OPERATIONS
MAX_DEPTH
MAX_CORRELATED_QUBITS

The semantic architecture has no artificial finite machine-size ceiling.

The intended rule is:

«A finite quantum computation may be represented whenever the available compiler, memory, runtime, simulator, target, storage, and physical resources are sufficient to represent and process it.»

"Infinity" means:

«no artificial finite machine-size ceiling is encoded by ZQN.»

It does not mean:

«an individual computer has infinite memory, CPU, storage, address space, or physical quantum resources.»

---

2.4 Resource limits are policy

Resource limits are allowed and required for safety.

They belong to explicit runtime/resource/security policies.

Examples:

maximum memory
maximum serialized input
maximum materialized tensor size
maximum sampling shots
maximum execution time
maximum fault batch size
maximum iteration count
maximum generated observations

These are operational policies.

They must never become claims about the maximum quantum computer Zamani can represent.

The distinction is:

SEMANTICS
    │
    └── no arbitrary machine-size ceiling

POLICY
    │
    └── finite limits may be imposed for safety

---

2.5 No unsafe Rust

Every ZQN source file must be safe Rust.

The root must contain:

#![forbid(unsafe_code)]

No ZQN module may use:

unsafe
unsafe fn
unsafe impl
unsafe trait
extern "C"
raw pointer dereferencing

unless the entire architecture is deliberately redesigned and the repository's safety contract is changed.

The current production target is strictly:

«zero unsafe Rust.»

---

3. Write-once-scale-everywhere contract

The central Zamani quantum requirement is:

«A program should be written once and scale to any size of quantum machine.»

The implementation must therefore follow:

Zamani source
      │
      ▼
canonical quantum IR
      │
      ▼
abstract ZQN model
      │
      ▼
target capabilities
      │
      ▼
target realization
      │
      ▼
execution

The source program must not contain:

if machine == X
if qubits == 5
if qubits == 127
if topology == ...
if vendor == ...

unless those distinctions are explicitly part of a user-selected target policy rather than the program's semantic meaning.

Machine size is runtime/target data.

---

4. Technology independence

ZQN must not assume that quantum computing means:

qubit + gate + Pauli error

The architecture must support present and future modalities.

The semantic layer must be extensible to:

- gate-model quantum computing;
- dynamic circuits;
- fault-tolerant quantum computing;
- logical qubits;
- physical qubits;
- qudits;
- continuous-variable systems;
- bosonic systems;
- photonic systems;
- neutral atoms;
- trapped ions;
- superconducting systems;
- spin systems;
- measurement-based quantum computing;
- analog quantum computing;
- Hamiltonian simulation;
- annealing;
- distributed quantum computing;
- networked quantum computing;
- quantum communication;
- transport-based systems;
- future quantum modalities.

The model must therefore attach noise to resources and operations, not merely to qubit numbers.

---

5. Noise versus fault

ZQN must distinguish:

noise
fault
error channel
observation
calibration uncertainty
approximation

They are not interchangeable.

A channel may describe a physical transformation.

A fault is a realized or modeled discrete failure.

An observation is measured evidence.

A calibration parameter is a physical estimate.

An approximation describes a deliberate deviation from requested semantics.

The architecture must not collapse all of these into one "Error" structure.

---

6. ZQN responsibilities

ZQN owns:

- probability semantics;
- probability distributions;
- uncertainty;
- quantum channels;
- channel representations;
- channel composition;
- fault semantics;
- fault locations;
- correlated faults;
- leakage;
- erasure;
- loss;
- preparation noise;
- gate noise;
- reset noise;
- measurement noise;
- idle noise;
- pulse noise;
- transport noise;
- temporal correlations;
- spatial correlations;
- crosstalk;
- drift;
- non-Markovian noise;
- conditional noise;
- calibration models;
- calibration snapshots;
- noise characterization;
- noise observations;
- deterministic stochastic execution contracts;
- error propagation;
- error budgets;
- noise-aware target compatibility;
- ZQN interoperability.

---

7. ZQN does not own

ZQN must not own:

- source-language parsing;
- frontend ASTs;
- canonical quantum IR;
- general compiler optimization;
- logical routing implementation;
- general scheduling implementation;
- QPU credentials;
- vendor SDKs;
- vendor network clients;
- backend transport;
- quantum hardware lifecycle;
- QEC decoder implementation;
- syndrome decoding;
- logical correction algorithms;
- benchmark orchestration;
- CLI;
- UI;
- application semantics.

These remain owned by their respective quantum subsystems.

---

8. Target architecture

The production directory is:

src/quantum/zqn/
│
├── ARCHITECTURE.md
├── README.md
├── SEMANTICS.md
├── SCALABILITY.md
├── DETERMINISM.md
├── COMPATIBILITY.md
├── SECURITY.md
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

This tree is an architectural target. A file must not be created merely because it appears in this document; it must have the contract described here and be implemented when its dependency prerequisites are complete.

---

9. Dependency layers

The conceptual dependency graph is:

core
  │
  ├── probability
  │
  ├── channel
  │
  └── fault
        │
        ▼
      noise
        │
        ├── operations
        ├── calibration
        └── characterization
              │
              ▼
          simulation
              │
              ▼
          propagation
              │
              ▼
            target
              │
              ▼
         integration
              │
              ▼
              io

This is a conceptual dependency graph, not permission for every module to import every other module.

Each module must depend on the narrowest interface necessary.

---

10. Forbidden dependency directions

The following are forbidden:

zqn::core → frontend
zqn::core → routing implementation
zqn::core → scheduling implementation
zqn::core → QEC decoder
zqn::core → hardware provider
zqn::core → benchmarking
zqn::core → UI
zqn::core → CLI

ZQN must not directly depend on:

IBM SDK
IonQ SDK
Rigetti SDK
Quantinuum SDK
Google Quantum SDK
AWS quantum SDK
Azure quantum SDK

Vendor integration belongs to:

crate::quantum::hardware

---

11. Core subsystem

"core/mod.rs"

Ownership

Composition boundary for foundational ZQN infrastructure.

Must contain

- child-module declarations;
- module-level architecture documentation;
- safety contract;
- dependency contract;
- canonical identity rule;
- scalability rule.

Must not contain

- domain implementation;
- RNG;
- hardware access;
- vendor logic;
- duplicated IDs;
- machine-size constants.

Completion criterion

Adding an independent ZQN subsystem must not require redesigning this file.

The current core contract already explicitly establishes this responsibility and canonical identity boundary.

---

12. "core/error.rs"

Owns the authoritative ZQN error vocabulary.

Required categories include:

InvalidProbability
InvalidDistribution
InvalidChannel
InvalidFault
InvalidNoiseModel
InvalidCalibration
InvalidCharacterization
UnsupportedRepresentation
UnsupportedCapability
CapabilityMismatch
ResourceLimitExceeded
NumericalFailure
NonFiniteValue
SerializationFailure
DeserializationFailure
ValidationFailure
CompatibilityFailure
Cancellation

It must provide:

pub type ZqnResult<T> = Result<T, ZqnError>;

No child subsystem may create a competing universal ZQN error type.

---

13. "core/ids.rs"

Owns only ZQN object identity.

Allowed:

NoiseModelId
ChannelId
FaultId
CalibrationId
CharacterizationId
ExperimentId
ObservationId
NoiseSnapshotId

Forbidden:

QubitId
PhysicalQubitId
QubitRef

Those belong to:

crate::quantum::ir::qubit

The repository's QEC/ZQN files already explicitly avoid introducing a ZQN-specific qubit identifier.

---

14. "core/metadata.rs"

Owns non-semantic metadata.

Examples:

name
description
labels
annotations
units
classification
source description

Metadata must never silently modify mathematical meaning.

Metadata must be serializable when attached to serializable public objects.

---

15. "core/version.rs"

Owns:

ZQN semantic version
schema version
serialization version
compatibility version

No other ZQN file may duplicate these authoritative constants.

Compatibility must be explicit rather than inferred from Rust struct layout.

---

16. "core/context.rs"

Defines explicit execution context.

Conceptually:

ZqnContext
├── limits
├── capabilities
├── calibration scope
├── determinism policy
├── numerical policy
├── cancellation
└── provenance

No global mutable context is allowed.

No hidden configuration may affect semantic execution.

---

17. "core/capabilities.rs"

Defines provider-neutral ZQN capabilities.

Capabilities may describe support for:

Kraus
Choi
Pauli transfer
Lindblad
stochastic channels
correlated noise
temporal correlations
spatial correlations
leakage
erasure
loss
readout noise
dynamic noise
conditional noise
continuous-time noise
calibration
characterization

Capabilities describe support.

They do not perform execution.

This mirrors the useful QIR distinction between a representation/profile and the target's available instruction/capability set.

---

18. "core/limits.rs"

Defines explicit resource policies.

Limits may include:

max_memory_bytes
max_serialized_bytes
max_operations
max_faults
max_distribution_entries
max_sampling_shots
max_tensor_elements
max_iterations
max_execution_time

Every limit must be optional/configurable.

"None" means no limit imposed by that policy layer.

These limits are not semantic machine limits.

---

19. "core/provenance.rs"

Every scientifically significant ZQN result should be traceable.

Provenance may identify:

ZQN version
model identity
configuration identity
calibration identity
target identity
experiment identity
software identity
source
timestamp
measurement source
characterization source

Provenance must never depend on:

memory address
pointer identity
process-local incidental ordering

---

20. Probability subsystem

Probability is foundational because stochastic quantum behavior must be represented without assuming every probability is merely an "f64".

---

21. "probability/probability.rs"

Defines probability values and numerical policy.

It must distinguish:

exact value
approximate value
bounded value
estimated value
statistical value

The implementation must reject invalid states rather than silently repairing them.

Forbidden:

negative probability → abs()
NaN → 0
Infinity → maximum

---

22. "probability/distribution.rs"

Defines general distributions.

Required operations:

validate
normalize
sample
support
expectation
variance
bounds

The abstraction must not assume:

2 outcomes
4 outcomes
Pauli outcomes
small finite state

Distributions must support lazy/streaming representations where materialization would be unreasonable.

---

23. "probability/categorical.rs"

Owns finite categorical distributions.

The number of categories is data.

Never encode:

BinaryDistribution
FourStateDistribution
FixedQubitDistribution

as architectural primitives.

---

24. "probability/continuous.rs"

Supports continuous parameter uncertainty.

Examples:

normal
uniform
exponential
log-normal
parameterized distributions

The numerical backend must remain replaceable.

---

25. "probability/bounds.rs"

Represents:

lower bound
upper bound
confidence bound
deterministic error bound

Bounds must remain distinguishable from statistical confidence intervals.

---

26. "probability/statistics.rs"

Owns distribution statistics:

mean
variance
covariance
moments
quantiles
confidence intervals

Statistical estimates must carry enough metadata to distinguish:

population value
sample estimate
confidence interval
credible interval
deterministic bound

---

27. Channel subsystem

The channel subsystem defines quantum transformations induced by noise.

A channel is more general than a gate error.

---

28. "channel/channel.rs"

Defines the primary channel abstraction.

The abstraction must support:

input resources
output resources
representation
application
validation
composition
tensor product
metadata
provenance

The channel interface must not require a fixed number of qubits.

---

29. "channel/representation.rs"

Defines representation-neutral channel metadata.

Supported representation families include:

Kraus
Choi
superoperator
Liouville
Pauli transfer
stochastic map
Lindblad generator
process matrix

Representation choice must be based on:

mathematical validity
target capability
memory
performance
precision
requested approximation

not vendor identity.

---

30. "channel/kraus.rs"

Owns Kraus representation.

Must provide:

construction
validation
trace-preservation checks
composition
tensor product
application
conversion

Matrix dimensions derive from actual resource dimensions.

No hard-coded qubit count is permitted.

---

31. "channel/choi.rs"

Owns Choi representation.

Must support:

construction
complete-positivity validation
trace-preservation validation
conversion

Materialization must be governed by explicit resource limits.

---

32. "channel/process_matrix.rs"

Owns general process representations needed by characterization and future quantum technologies.

It must not assume every process can be reduced to a simple qubit gate channel.

---

33. "channel/pauli.rs"

Provides Pauli-specific channels.

Supported concepts:

I
X
Y
Z
Pauli strings
correlated Pauli channels

Pauli noise is a specialization.

It is not the universal ZQN noise model.

---

34. "channel/stochastic.rs"

Owns stochastic channels and stochastic process representations.

It must remain independent of simulation implementation.

---

35. "channel/lindblad.rs"

Represents continuous-time generators.

Conceptually:

dρ/dt = L(ρ)

The file owns the mathematical model.

Numerical integration belongs to simulation.

---

36. "channel/thermal.rs"

Owns thermal/no-temperature-dependent channel semantics.

Environmental parameters must be explicit.

No global temperature is allowed.

---

37. "channel/amplitude.rs"

Owns amplitude-related channels.

---

38. "channel/phase.rs"

Owns phase-related channels.

---

39. "channel/depolarizing.rs"

Owns depolarizing-family channels.

---

40. "channel/generalized.rs"

Owns extensible channel forms that do not fit the specialized categories.

This prevents the architecture from becoming trapped by today's standard noise taxonomy.

---

41. "channel/composition.rs"

Defines:

sequential composition
tensor composition
correlated composition
conditional composition

Composition must preserve:

metadata
provenance
uncertainty
approximation contract

where semantically appropriate.

---

42. Fault subsystem

A fault is distinct from a general quantum channel.

The fault subsystem represents realized or modeled discrete failures.

---

43. "fault/fault.rs"

Defines the generic fault abstraction.

Faults may represent:

physical
logical
environmental
transport
measurement
preparation
reset
timing
leakage
loss
erasure
correlated

The current repository already describes ZQN faults in this broad manner and explicitly excludes canonical IR and qubit identity ownership.

---

44. "fault/location.rs"

Defines where a fault occurs.

A location may refer to:

canonical logical resource
canonical physical resource
operation
measurement
reset
pulse
time interval
transport link
composite resource

For qubits, use canonical IR IDs.

No numeric index is assumed to be universally meaningful.

---

45. "fault/classification.rs"

Provides stable fault categories without requiring every future fault type to modify the base enum.

Classification must be extensible.

---

46. "fault/correlated.rs"

Correlation must support arbitrary resource sets.

Forbidden:

Correlated2QubitFault
Correlated3QubitFault
Correlated4QubitFault

Preferred conceptual model:

CorrelatedFault
├── resources
├── correlation model
└── realization

The number of resources is runtime data.

---

47. "fault/leakage.rs"

Owns leakage fault semantics.

It must not create another qubit identity system.

The current repository already documents this exact boundary.

---

48. "fault/erasure.rs"

Owns erasure fault semantics.

Erasure is distinct from generic loss and Pauli error.

The current repository already has a dedicated erasure semantics boundary.

---

49. "fault/loss.rs"

Owns loss semantics.

Loss may be relevant to:

photonic systems
transport
neutral atoms
network links
memory
communication

It must not be qubit-gate-specific.

---

50. "fault/batch.rs"

Supports large fault streams.

The API must allow:

iterator
stream
lazy generator
bounded batch

rather than requiring every fault to be stored in one unbounded "Vec".

---

51. Noise subsystem

The noise subsystem combines channels, faults, timing, correlations, and context into executable noise semantics.

---

52. "noise/model.rs"

This is the primary ZQN abstraction.

Conceptually:

NoiseModel
├── describe
├── validate
├── apply
├── sample
└── provenance

The model must not know:

IBM API
IonQ API
Rigetti API
QPU credentials
network transport

---

53. "noise/specification.rs"

Defines declarative noise configuration.

It must support categories such as:

preparation
gate
reset
measurement
idle
transport
pulse
correlated
leakage
loss
erasure
drift
conditional

It must remain source-language-independent.

The Zamani frontend may later lower source-level noise syntax into this specification.

---

54. "noise/application.rs"

Defines how a noise model attaches to a canonical operation or physical realization.

The conceptual relation is:

canonical operation
        +
noise model
        ↓
noise application

The canonical operation remains owned by "quantum::ir".

---

55. "noise/composition.rs"

Combines multiple noise models.

It must define whether composition means:

sequential
parallel
conditional
correlated
layered

without assuming independence unless independence is explicitly declared.

---

56. "noise/correlation.rs"

Defines generic correlation semantics.

Correlation may exist across:

resources
operations
time
measurement results
environment variables
execution sessions

---

57. "noise/temporal.rs"

Supports:

stationary noise
non-stationary noise
time-dependent noise
temporal correlation
drift
memory effects

---

58. "noise/spatial.rs"

Supports:

local correlation
graph-based correlation
long-range correlation
collective noise
topology-dependent correlation

Topology is supplied by the target/hardware subsystem.

---

59. "noise/crosstalk.rs"

Crosstalk describes unintended interaction among concurrently active resources.

Routing remains responsible for placement.

Scheduling remains responsible for timing.

ZQN provides the physical noise/cost information.

---

60. "noise/drift.rs"

Represents evolving parameters.

Drift must be time/context dependent.

It must not mutate a shared global model.

---

61. "noise/non_markovian.rs"

Supports memory-bearing noise.

Possible representations:

memory process
history-dependent process
memory kernel
environment state
correlated trajectory

The model must not force every physical process into independent per-operation noise.

---

62. "noise/conditional.rs"

Supports noise conditioned on:

operation
resource
measurement outcome
classical condition
time
environment
calibration state
execution context

This is required for dynamic circuits.

---

63. Operations subsystem

Operations describe where noise attaches without becoming a second IR.

---

64. "operations/operation.rs"

Defines generic ZQN operation context.

It may reference canonical IR operation identity.

It must not recreate canonical gate/circuit semantics.

---

65. "operations/gate.rs"

Owns gate-level noise attachments.

A conceptual representation may contain:

ideal operation reference
noise channel
coherent error
duration
calibration reference

The ideal gate itself remains owned by "quantum::ir".

---

66. "operations/preparation.rs"

Owns preparation noise.

Examples:

state-preparation error
initialization error
thermal preparation
leakage during preparation

---

67. "operations/reset.rs"

Reset noise is independent of measurement noise.

The implementation must not assume:

reset = measure + conditional gate

unless that is an explicitly declared approximation.

---

68. "operations/measurement.rs"

Supports:

assignment error
asymmetric readout
correlated readout
state-dependent error
measurement backaction

---

69. "operations/idle.rs"

Represents noise during idle intervals.

The scheduler can ask:

noise(resource, duration, context)

without needing to know the mathematical noise implementation.

---

70. "operations/pulse.rs"

Supports pulse-level noise.

The architecture remains layered:

canonical operation
      ↓
scheduled operation
      ↓
pulse realization
      ↓
pulse noise

---

71. "operations/transport.rs"

Supports noise associated with:

ion shuttling
photonic transport
quantum links
distributed resources
memory movement
physical resource transport

---

72. Calibration subsystem

Calibration describes measured/estimated physical parameters.

Calibration is not noise itself, but it provides the parameters from which noise models may be constructed.

---

73. "calibration/snapshot.rs"

Defines immutable calibration snapshots.

A snapshot must identify:

target
resource scope
timestamp
validity interval
parameters
uncertainty
provenance

A snapshot must not be silently mutated.

---

74. "calibration/parameter.rs"

Generic calibrated parameter:

value
uncertainty
units
validity
provenance

No fixed universal parameter list.

---

75. "calibration/device.rs"

Maps abstract calibration information to target resources.

Hardware-specific resource identity remains owned by hardware.

---

76. "calibration/gate.rs"

Gate-specific calibration information.

---

77. "calibration/readout.rs"

Readout-specific calibration.

---

78. "calibration/measurement.rs"

Measurement-specific calibration.

---

79. "calibration/drift.rs"

Models calibration evolution.

---

80. "calibration/interpolation.rs"

Provides explicit interpolation between calibration observations.

Interpolation must carry numerical/error policy.

It must never silently claim exactness.

---

81. "calibration/validation.rs"

Validates:

parameter ranges
units
resource scope
validity intervals
internal consistency
uncertainty consistency

---

82. Characterization subsystem

Characterization answers:

«What does the physical system actually do?»

It converts observations into noise estimates.

---

83. "characterization/experiment.rs"

Defines experiment identity and requirements.

---

84. "characterization/protocol.rs"

Defines characterization protocol.

---

85. "characterization/observation.rs"

Stores raw/normalized observations.

Observations must retain provenance.

---

86. "characterization/estimator.rs"

Converts observations into parameter/channel/noise estimates.

Estimates must carry uncertainty.

---

87. "characterization/uncertainty.rs"

Defines uncertainty propagation from measurements into estimated noise models.

---

88. "characterization/tomography.rs"

Supports channel/process tomography.

The implementation must not assume a fixed number of qubits.

Resource policies govern materialization.

---

89. "characterization/randomized_benchmarking.rs"

Provides randomized benchmarking-related characterization.

Benchmark orchestration remains owned by the benchmarking subsystem where appropriate.

ZQN owns the noise characterization semantics.

---

90. "characterization/process_characterization.rs"

Provides general process-characterization abstractions for future quantum modalities.

---

91. Simulation subsystem

Simulation consumes ZQN semantics.

ZQN must not assume one simulator architecture.

---

92. "simulation/engine.rs"

Defines the simulator-facing execution contract.

The engine must support representation-specific execution without changing the noise model API.

---

93. "simulation/sampler.rs"

Provides sampling.

Sampling must use explicit deterministic context.

No hidden global RNG.

---

94. "simulation/trajectory.rs"

Supports trajectory-based stochastic simulation.

---

95. "simulation/channel_engine.rs"

Applies quantum channels.

The mathematical channel remains owned by "channel".

---

96. "simulation/monte_carlo.rs"

Provides Monte Carlo execution.

Shot count is an execution parameter, not a compile-time semantic limit.

---

97. "simulation/deterministic.rs"

Provides deterministic/no-randomness execution where mathematically possible.

---

98. "simulation/reproducibility.rs"

Defines reproducibility.

The conceptual identity is:

master seed
+
program identity
+
noise-model identity
+
configuration identity
+
calibration identity
+
target identity
+
operation identity
+
resource identity
+
shot identity

The same computation must produce the same stochastic realization under the same deterministic policy.

Parallel execution must not change the semantic result merely because execution was distributed across threads.

---

99. Global RNG prohibition

ZQN must never use:

global RNG
thread-local hidden RNG
process-global RNG
time-based implicit seed

A caller-controlled deterministic context must control stochastic execution.

This is especially important because the repository's QEC noise architecture already emphasizes caller-supplied deterministic seeds rather than hidden global randomness.

---

100. Propagation subsystem

Propagation answers:

«What does the noise imply for the computation's result?»

---

101. "propagation/error_budget.rs"

Defines error budgets.

It must support budgets across:

operation
layer
resource
time
logical circuit
distributed computation

---

102. "propagation/uncertainty.rs"

Propagates uncertainty through transformations.

It must distinguish:

physical uncertainty
statistical uncertainty
model uncertainty
numerical uncertainty
approximation error

---

103. "propagation/fidelity.rs"

Provides backend-independent fidelity concepts where mathematically appropriate.

Potential metrics include:

state fidelity
process fidelity
average gate fidelity
entanglement fidelity

Metrics must not be used outside their valid mathematical domains.

---

104. "propagation/bounds.rs"

Defines rigorous or declared bounds.

Every approximation must declare its bound or explicitly state that no certified bound is available.

---

105. "propagation/sensitivity.rs"

Determines which noise parameters most strongly influence a result.

This can feed:

optimization
routing
scheduling
calibration
QEC
benchmarking

---

106. "propagation/accumulation.rs"

Models accumulation of errors across:

operations
layers
time
resources
logical computation
distributed communication

The implementation must not assume error simply adds linearly.

---

107. Approximation contract

Approximation must always be explicit.

The architecture must distinguish:

Exact
Approximate
Bounded
Statistical
Unsupported

An approximation must identify:

requested semantics
realized semantics
approximation method
error bound if known
confidence if statistical
assumptions

ZQN must never silently downgrade an exact requested model to a simpler model.

---

108. Target subsystem

Target is the bridge between abstract ZQN semantics and actual target capabilities.

---

109. "target/requirements.rs"

Defines what a computation/noise model requires.

Examples:

required channel representation
required correlation
required temporal model
required leakage support
required precision
required approximation tolerance
required calibration freshness

---

110. "target/capabilities.rs"

Describes what a target supports.

It may include:

resource capabilities
channel capabilities
noise capabilities
timing capabilities
measurement capabilities
calibration capabilities
precision capabilities

It must remain provider-neutral.

---

111. "target/compatibility.rs"

Determines:

exactly compatible
approximately compatible
compatible under declared bound
incompatible

It must never silently classify unsupported semantics as supported.

---

112. "target/lowering.rs"

Transforms:

abstract ZQN model
        ↓
target-supported realization

The lowering must be explicit.

---

113. "target/validation.rs"

Validates compatibility before execution.

Failures must occur before irreversible execution where possible.

---

114. Integration with canonical IR

"integration/ir.rs" is the primary semantic boundary.

The relationship is:

quantum::ir
      +
ZQN model
      ↓
noise-aware execution view

Do not make "quantum::ir" depend directly on every ZQN implementation.

Prefer:

IR operation reference
+
ZQN attachment/interface

This preserves the existing repository principle that "quantum::ir" is the canonical semantic boundary.

---

115. Integration with routing

"integration/routing.rs" exposes ZQN-derived costs.

Routing may consume:

gate error
readout error
idle error
crosstalk
duration
correlation
calibration uncertainty

The routing subsystem decides placement.

ZQN supplies physical noise information.

This avoids parallel competing noise semantics in "routing/noise_aware.rs".

---

116. Integration with scheduling

"integration/scheduling.rs" provides noise-aware scheduling information.

The scheduler may query:

noise(resource, operation, duration, context)

Scheduling remains responsible for ordering.

ZQN remains responsible for noise semantics.

---

117. Integration with QEC

The existing QEC subsystem already owns physical fault generation and QEC algorithms.

The long-term architecture is:

                  ZQN
                   │
        ┌──────────┴──────────┐
        ▼                     ▼
 physical noise          noise channels
        │
        ▼
 QEC physical-fault adapter
        │
        ▼
 syndrome generation
        │
        ▼
 decoder
        │
        ▼
 logical correction
        │
        ▼
 logical error analysis

ZQN must not take ownership of:

syndrome decoding
decoder implementation
logical correction
QEC algorithm selection

Instead, QEC consumes ZQN semantics.

The migration must be incremental so existing QEC behavior does not break.

---

118. QEC migration rule

Do not immediately delete existing QEC noise code.

Migration sequence:

CURRENT

QEC noise implementation
        │
        ▼
QEC

TARGET

ZQN universal noise semantics
        │
        ▼
QEC adapter
        │
        ▼
QEC algorithms

Then:

QEC noise.rs
        ↓
compatibility/adaptation layer
        ↓
eventual removal after all consumers migrate

No duplicate mathematical implementation should remain after migration.

---

119. Integration with hardware

"integration/hardware.rs" must be adapter-oriented.

Hardware provides:

TargetCapabilities
CalibrationSnapshot
ObservedNoise
ResourceMapping

ZQN consumes those abstractions.

ZQN must never call:

provider SDK
HTTP API
QPU credential store
hardware transport

directly.

---

120. Integration with memory

"integration/memory.rs" connects channel application to the quantum memory/state subsystem.

Memory owns state/resource representation.

ZQN owns channel/noise semantics.

The relationship is:

ZQN channel
     │
     ▼
channel application request
     │
     ▼
quantum memory

No second state representation should be created in ZQN.

---

121. Integration with benchmarking

"integration/benchmarking.rs" exposes:

NoiseObservation
NoiseCharacterization
CalibrationSnapshot
ErrorBudget

to benchmarking.

Benchmarking owns:

experiment orchestration
workload generation
execution contracts
statistics
metrics
reporting
regression analysis

ZQN owns physical noise semantics.

This prevents benchmarking from becoming the semantic foundation of ZQN.

---

122. Integration with runtime

"integration/runtime.rs" defines what runtime supplies:

execution context
clock
seed policy
target capabilities
resource policy
cancellation
calibration scope

ZQN supplies:

noise realization
channel
fault stream
observation semantics

Runtime performs orchestration.

---

123. I/O subsystem

Persistent representations must not be coupled to Rust memory layout.

---

124. "io/schema.rs"

Defines versioned external schema.

Schema identity must be independent from:

Rust struct field order
Rust memory layout
compiler version
pointer identity

---

125. "io/serialization.rs"

Converts validated semantic objects to external representation.

Serialization must be deterministic where canonical serialization is requested.

---

126. "io/deserialization.rs"

Loads external representations.

It must validate:

schema version
numeric validity
resource references
dimensions
limits
capabilities
semantic invariants

Untrusted data must never bypass validation.

---

127. "io/canonical.rs"

Defines canonical serialization used for:

hashing
identity
caching
provenance
reproducibility
comparison

Canonical form must not depend on hash-map iteration order.

---

128. "io/compatibility.rs"

Owns schema migrations.

Compatibility must distinguish:

same semantic meaning
compatible representation
lossy conversion
unsupported conversion

---

129. Hash identity

A noise model identity should conceptually derive from:

semantic model
+
configuration
+
schema version
+
calibration identity
+
relevant provenance

It must not derive from:

memory address
pointer value
process ID
thread ID
hash-map iteration order

This permits reliable caching and distributed execution.

---

130. Cache correctness

A cache key must include all semantic inputs relevant to the result.

At minimum where applicable:

model identity
configuration identity
calibration identity
target capability identity
numerical policy
approximation policy
schema version

Never cache merely by model name.

---

131. Calibration cache correctness

Calibration caches must account for:

target
resource
validity interval
calibration version
measurement context

Calibration must never be assumed permanent.

---

132. Security architecture

ZQN is a potential security boundary because it may process external noise models, calibration data, serialized objects, or user-supplied parameters.

It must defend against:

allocation bombs
huge distributions
huge correlated-resource sets
malicious tensor dimensions
NaN/Infinity
numerical overflow
numerical underflow
pathological iteration
nonterminating generators
malicious serialized models
malicious calibration data
resource exhaustion

---

133. Resource governance

Every potentially expensive operation must have a route to:

limits
cancellation
validation

No operation may assume unlimited memory.

No operation may materialize an exponentially large object merely because the mathematical object exists.

---

134. Lazy and streaming execution

For large systems, ZQN must prefer:

iterators
streams
lazy channels
lazy faults
sparse representations
chunked operations
externalized storage

when appropriate.

The semantic object and its materialized implementation must remain separate.

---

135. Sparse representations

Where a channel/fault/distribution is naturally sparse, ZQN should provide sparse representations.

It must not force:

dense matrix
dense tensor
dense probability vector

for every object.

---

136. Representation polymorphism

A single semantic model may be represented as:

exact
symbolic
dense
sparse
sampled
stochastic
trajectory
tensorized
hardware-native
approximate

Representation selection belongs to capability/resource policy.

This is consistent with extensible multi-level compiler architecture: MLIR uses abstractions and interfaces so transformations need not hard-code every concrete representation.

---

137. Parallelism

ZQN implementations should be "Send + Sync" where their semantics permit it.

No global mutable state.

No global calibration.

No global RNG.

No process-global noise model.

Independent executions must remain independent.

---

138. Deterministic parallelism

Given the same:

program
noise model
configuration
target
calibration
seed
shot identity

the result must not depend on:

thread count
task scheduling
worker ordering
CPU core

when deterministic execution is requested.

This is essential for scientific reproducibility.

---

139. Distributed execution

Distributed deterministic derivation must include stable identities such as:

global seed
node identity
resource identity
operation identity
shot identity

Never use machine-local random state as semantic identity.

---

140. Mathematical validation

Channels must validate appropriate invariants.

Examples:

probability bounds
normalization
complete positivity
trace preservation
dimension compatibility
composition compatibility

Validation must be explicit.

---

141. Numerical policy

The numerical layer must make precision explicit.

Potential policies:

exact
fixed precision
floating precision
interval
bounded approximation
statistical

No silent precision downgrade.

No silent NaN repair.

No silent overflow conversion.

---

142. Units

Physical quantities such as:

time
frequency
temperature
energy
rate
distance

must have explicit units or an unambiguous unit contract.

Unit conversion must not be hidden in arbitrary arithmetic.

---

143. Time

Time must never be represented solely as an unqualified "f64" where physical semantics matter.

A production representation must distinguish:

duration
absolute timestamp
relative interval
validity interval
sampling interval

and preserve units/precision.

---

144. Correlation scaling

Correlation must be represented independently from the number of resources.

The architecture must support:

local
global
graph
kernel
matrix
tensor
functional
history-dependent

correlation models.

The resource set determines size.

---

145. No hard-coded gate set

ZQN must never assume:

H
X
Y
Z
CNOT
CZ

are the universal operation set.

Those are specializations.

Noise can attach to arbitrary canonical operations and physical realizations.

---

146. No hard-coded topology

ZQN must not assume:

line
grid
ring
heavy-hex
all-to-all

as universal topology.

Topology belongs to target/hardware.

ZQN consumes topology-derived resource relationships where required.

---

147. No hard-coded vendor assumptions

No ZQN file may contain vendor-specific:

qubit count
gate name
device ID format
calibration format
transport protocol
credential

Vendor translation belongs outside ZQN.

---

148. Integration with QIR

ZQN is not QIR.

The relationship is:

Zamani IR
     │
     ├──────────────► ZQN
     │
     └──────────────► QIR exporter

QIR is a possible downstream interchange/lowering target.

QIR's architecture intentionally provides a many-to-many connection between languages and heterogeneous quantum processors.

ZQN must remain Zamani's physical-noise semantic layer.

---

149. Integration with MLIR

ZQN does not need to become an MLIR dialect immediately.

If MLIR interoperability is introduced later:

Zamani IR
     │
     ▼
ZQN-aware representation
     │
     ▼
MLIR dialect/interface
     │
     ▼
lowering
     │
     ▼
QIR / LLVM / target

The important architectural principle is that generic transformations should consume interfaces rather than hard-code every noise implementation. MLIR explicitly uses interfaces for this decoupling.

---

150. No dependency on MLIR

The current Rust ZQN implementation must remain usable without requiring MLIR.

MLIR interoperability is an integration target, not a semantic prerequisite.

---

151. File-completion contract

Every ZQN implementation file must be treated as independently completable.

Before implementation, every file must have:

1. ownership;
2. non-ownership;
3. public API;
4. invariants;
5. dependency contract;
6. consumer contract;
7. error contract;
8. resource contract;
9. determinism contract;
10. scalability contract;
11. serialization contract;
12. thread-safety contract;
13. integration contract;
14. test contract.

The objective is:

«Once a file is completed against its frozen contract, another independent file should not require reopening it merely because another subsystem was implemented.»

If a later integration reveals a genuine contract defect, the contract itself must be revised deliberately rather than repeatedly patching implementation details.

---

152. Standard file documentation contract

Every substantial ZQN source file should begin with documentation containing:

Ownership
Non-ownership
Invariants
Dependencies
Consumers
Integration
Scalability
Determinism
Resource safety
Errors
Serialization
Thread safety
Testing

Example:

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
//! # Testing
//!
//! ...

This documentation contract is mandatory for production ZQN files.

---

153. Testing architecture

Testing is part of implementation, not a final phase.

Required test classes:

unit
property
differential
determinism
scaling
compatibility
integration
fuzz

---

154. Unit tests

Every foundational file must test its own invariants.

Examples:

probability
distribution
channel
fault
noise
calibration
serialization

---

155. Property tests

Properties include:

probabilities remain valid
distributions normalize
channel dimensions remain compatible
identity composition behaves correctly
serialization round-trips
canonical serialization is stable
deterministic execution is reproducible

---

156. Differential tests

Equivalent representations should agree within declared numerical tolerance.

Examples:

Kraus
↕
Choi
↕
superoperator
↕
Pauli transfer

where conversion is mathematically supported.

---

157. Determinism tests

Test:

same seed + same model = same result

and:

1 worker
8 workers
64 workers

produce equivalent deterministic results.

---

158. Scaling tests

Do not create:

MAX_SUPPORTED_QUBITS

tests.

Instead generate resource sizes dynamically.

Tests should prove that behavior scales with input/resource size rather than depending on an architectural maximum.

---

159. Scaling definition

Scaling means:

N resources
→ algorithms operate over N

N operations
→ algorithms operate over N

N correlated resources
→ correlation representation scales with N

N shots
→ execution scales with N

N target resources
→ target descriptions scale with N

The implementation may eventually run out of resources.

That is an operational failure, not a semantic limitation.

---

160. Fuzz testing

Fuzz:

serialized noise models
probability values
distributions
channel dimensions
channel serialization
fault locations
correlation definitions
calibration data
schema versions

Fuzz acceptance criteria:

no panic
no unsafe behavior
no uncontrolled allocation
no infinite loop
no silent invalid-state acceptance

---

161. Security tests

Test:

huge input
malformed input
NaN
Infinity
negative dimensions
invalid probabilities
invalid correlations
invalid calibration
unsupported representations
resource exhaustion
cancellation

---

162. Compatibility tests

Every schema version must have tests proving:

supported old input
→ validated migration
→ current semantic model

Lossy migration must be explicit.

Unsupported migration must fail clearly.

---

163. Integration tests

Integration tests must prove:

IR → ZQN
ZQN → routing
ZQN → scheduling
ZQN → QEC
ZQN → hardware
ZQN → memory
ZQN → benchmarking
ZQN → runtime
ZQN → I/O

No integration test should require implementation-specific knowledge where an interface exists.

---

164. Acceptance criteria

ZQN is production-ready only when:

Semantic

- mathematical semantics are explicit;
- channels and faults are distinct;
- approximations are explicit;
- uncertainty is explicit.

Architectural

- ZQN is not a second IR;
- canonical IR remains authoritative;
- dependency directions are respected;
- no circular subsystem ownership exists.

Scalability

- no semantic machine-size ceiling;
- no hard-coded qubit counts;
- no fixed gate arity;
- no fixed topology;
- no fixed technology assumption;
- lazy/streaming strategies exist where necessary.

Determinism

- no hidden global RNG;
- deterministic stochastic execution is reproducible;
- parallel execution can remain reproducible.

Numerical safety

- invalid numerical states are rejected;
- precision is explicit;
- approximation is explicit;
- physical units are controlled.

Security

- untrusted input is validated;
- resource policies exist;
- cancellation exists;
- serialization is bounded;
- no unsafe Rust.

Interoperability

- canonical "quantum::ir::qubit" identities are used;
- QEC has an adapter boundary;
- routing has a noise interface;
- scheduling has a noise interface;
- hardware supplies capabilities/calibration;
- memory consumes channel semantics;
- benchmarking consumes observations.

Scientific reproducibility

- provenance exists;
- calibration identity exists;
- model identity exists;
- seed identity exists;
- numerical policy is recorded;
- schema version is recorded.

---

165. Implementation order

The implementation order is deliberately dependency-first.

Phase 0 — Contract foundation

Complete independently:

core/error.rs
core/version.rs
core/ids.rs
core/limits.rs
core/metadata.rs
core/provenance.rs
core/context.rs
core/capabilities.rs

No simulator, hardware, routing, QEC, or benchmarking dependency is allowed here.

---

Phase 1 — Probability

Complete:

probability/probability.rs
probability/bounds.rs
probability/distribution.rs
probability/categorical.rs
probability/continuous.rs
probability/statistics.rs

These should be usable without hardware.

---

Phase 2 — Channel mathematics

Complete:

channel/representation.rs
channel/kraus.rs
channel/choi.rs
channel/process_matrix.rs
channel/pauli.rs
channel/stochastic.rs
channel/lindblad.rs
channel/thermal.rs
channel/amplitude.rs
channel/phase.rs
channel/depolarizing.rs
channel/generalized.rs
channel/composition.rs
channel/channel.rs

---

Phase 3 — Fault semantics

Complete:

fault/location.rs
fault/classification.rs
fault/fault.rs
fault/leakage.rs
fault/erasure.rs
fault/loss.rs
fault/correlated.rs
fault/batch.rs

---

Phase 4 — Noise model

Complete:

noise/specification.rs
noise/application.rs
noise/correlation.rs
noise/temporal.rs
noise/spatial.rs
noise/crosstalk.rs
noise/drift.rs
noise/non_markovian.rs
noise/conditional.rs
noise/composition.rs
noise/model.rs

---

Phase 5 — Operation integration

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

Phase 6 — Calibration

Complete:

calibration/parameter.rs
calibration/snapshot.rs
calibration/device.rs
calibration/gate.rs
calibration/readout.rs
calibration/measurement.rs
calibration/drift.rs
calibration/interpolation.rs
calibration/validation.rs

---

Phase 7 — Characterization

Complete:

characterization/experiment.rs
characterization/protocol.rs
characterization/observation.rs
characterization/uncertainty.rs
characterization/estimator.rs
characterization/tomography.rs
characterization/randomized_benchmarking.rs
characterization/process_characterization.rs

---

Phase 8 — Simulation

Complete:

simulation/engine.rs
simulation/sampler.rs
simulation/trajectory.rs
simulation/channel_engine.rs
simulation/monte_carlo.rs
simulation/deterministic.rs
simulation/reproducibility.rs

---

Phase 9 — Propagation

Complete:

propagation/error_budget.rs
propagation/uncertainty.rs
propagation/fidelity.rs
propagation/bounds.rs
propagation/sensitivity.rs
propagation/accumulation.rs

---

Phase 10 — Target

Complete:

target/requirements.rs
target/capabilities.rs
target/compatibility.rs
target/lowering.rs
target/validation.rs

---

Phase 11 — Integration

Complete:

integration/ir.rs
integration/routing.rs
integration/scheduling.rs
integration/qec.rs
integration/hardware.rs
integration/memory.rs
integration/benchmarking.rs
integration/runtime.rs

Only after their underlying subsystem contracts are stable.

---

Phase 12 — I/O

Complete:

io/schema.rs
io/serialization.rs
io/deserialization.rs
io/canonical.rs
io/compatibility.rs

---

Phase 13 — Composition

Complete:

prelude.rs
mod.rs

"mod.rs" should be the composition root, not the place where domain behavior is implemented.

---

Phase 14 — Complete validation

Run:

unit tests
property tests
differential tests
determinism tests
scaling tests
compatibility tests
integration tests
fuzz tests
security tests

Then validate the entire quantum workspace.

---

166. ZQN maturity levels

ZQN 0 — Foundation

core
probability
IDs
errors
versioning
limits
provenance

ZQN 1 — Noise mathematics

channels
faults
distributions
composition

ZQN 2 — Physical noise

gate
measurement
idle
leakage
loss
erasure
correlation
crosstalk

ZQN 3 — Calibration

calibration
drift
characterization
uncertainty

ZQN 4 — Simulation

sampling
Monte Carlo
trajectory
deterministic execution
reproducibility

ZQN 5 — Compiler integration

IR
routing
scheduling
optimization consumers

ZQN 6 — Fault tolerance

QEC adapter
logical noise
logical error analysis

ZQN 7 — Hardware

capabilities
calibration
target validation
target lowering

ZQN 8 — Distributed/future quantum

network noise
transport
bosonic
continuous-variable
analog
measurement-based
future modalities

ZQN 9 — Production

versioned schemas
formal invariants
fuzzing
property testing
differential testing
deterministic parallelism
resource governance
security
provenance
reproducibility

---

167. Relationship to existing Zamani quantum architecture

The repository's quantum composition root establishes:

frontend
    ↓
quantum::ir
    ↓
algorithms / optimization / routing / scheduling / QEC
    ↓
hardware
    ↓
runtime

and explicitly keeps "quantum::ir" as the canonical semantic boundary.

ZQN is therefore a cross-cutting physical-noise subsystem:

                         quantum::ir
                              │
                              ▼
                             ZQN
                    ┌─────────┼─────────┐
                    │         │         │
                    ▼         ▼         ▼
                 routing  scheduling    QEC
                    │         │         │
                    └─────────┼─────────┘
                              ▼
                           hardware

ZQN does not replace the existing quantum architecture.

It supplies a missing common semantic layer.

---

168. Existing ZQN repository state

The current repository already contains explicit ZQN module boundaries including:

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

For example, the repository already describes "probability/mod.rs" as the authoritative probability composition boundary.

Likewise:

- "noise/mod.rs" defines backend-independent noise semantics.
- "calibration/mod.rs" defines the calibration namespace boundary.
- "simulation/mod.rs" defines the simulation composition boundary.
- "characterization/mod.rs" defines the characterization subsystem.
- "operations/mod.rs" defines the operations boundary.
- "propagation/mod.rs" defines the propagation boundary.
- "io/mod.rs" defines the persistence/interchange boundary.
- "target/mod.rs" already explicitly frames target-independent ZQN descriptions and target-supported realization.

Therefore this "ARCHITECTURE.md" is a consolidated production contract for the existing direction, rather than proposing that all of those directories be blindly recreated.

---

169. Required correction to the malformed root path

The repository search currently exposes the root ZQN file as:

src/quantum/zqn/mod.rs 

with a trailing space in the path.

This is a repository hygiene issue and must be corrected before the ZQN root can be treated as a normal Rust module.

The intended path is exactly:

src/quantum/zqn/mod.rs

There must not be both:

mod.rs
mod.rs 

The trailing-space path must be removed/renamed using the repository's version-control workflow.

---

170. Root module integration

After the malformed filename is corrected and the ZQN root compiles, "src/quantum/mod.rs" must expose:

pub mod zqn;

only after the ZQN module tree is internally valid.

This is consistent with the existing quantum composition-root policy: incomplete placeholders must not be exposed as stable public modules.

---

171. Rust 1.97 / 1.97.1 compatibility

ZQN must compile under:

Rust 1.97
Rust 1.97.1
Rust 2021
stable toolchain

No nightly-only language features.

No unstable compiler features.

No unsafe code.

Every dependency added to ZQN must be checked against the repository's supported Rust version.

---

172. Public API stability

The stable public API should remain intentionally small.

Core public concepts should eventually include:

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
NoiseCharacterization

ErrorBudget
Uncertainty

TargetNoiseRequirements
TargetNoiseCapabilities

Implementation-specific types should remain private until there is a demonstrated need to expose them.

---

173. Prelude policy

"prelude.rs" should re-export stable concepts only.

It must not export:

internal helper types
test fixtures
backend internals
private adapters
unstable implementation details

The prelude is an ergonomic API, not an implementation dump.

---

174. API evolution

New functionality should prefer:

new trait capability
new optional representation
new extension type

over modifying foundational types in ways that force every existing consumer to change.

Breaking changes require:

version update
compatibility decision
migration documentation
tests

---

175. No circular ownership

The following must remain distinct:

IR       → meaning
ZQN      → noise
Routing  → placement
Schedule → time
QEC      → fault tolerance
Hardware → capabilities
Runtime  → execution
Memory   → state/resources
Benchmark→ measurement

If two subsystems appear to own the same semantic concept, one ownership contract must be corrected.

---

176. The ultimate execution model

The desired complete pipeline is:

                    Zamani program
                           │
                           ▼
                    source frontend
                           │
                           ▼
                    canonical IR
                           │
             ┌─────────────┼─────────────┐
             │             │             │
             ▼             ▼             ▼
        optimization    analysis      ZQN model
             │                           │
             └─────────────┬─────────────┘
                           ▼
                    noise-aware IR view
                           │
                ┌──────────┼───────────┐
                │          │           │
                ▼          ▼           ▼
             routing   scheduling      QEC
                │          │           │
                └──────────┼───────────┘
                           ▼
                    target selection
                           │
                           ▼
                  capability validation
                           │
                           ▼
                       lowering
                           │
                           ▼
                    runtime execution
                           │
                ┌──────────┼───────────┐
                │          │           │
                ▼          ▼           ▼
             simulator     QPU      distributed
                │          │           │
                └──────────┼───────────┘
                           ▼
                      observations
                           │
                ┌──────────┼───────────┐
                ▼          ▼           ▼
        characterization benchmarking analysis
                │
                ▼
             calibration
                │
                └──────────────► ZQN

---

177. Final architectural guarantee

The production ZQN architecture must guarantee:

«A Zamani program expresses computational intent once.»

«"quantum::ir" expresses its canonical semantics.»

«ZQN expresses physical uncertainty and noise.»

«Optimization transforms equivalent representations.»

«Routing chooses placement.»

«Scheduling chooses time.»

«QEC provides fault tolerance.»

«Hardware declares capabilities.»

«Runtime orchestrates execution.»

«Memory manages state/resources.»

«Benchmarking measures behavior.»

No individual machine size, vendor, topology, native gate set, or current quantum technology should be embedded into the source-language semantics.

The same program must therefore be capable of being lowered toward:

tiny simulator
small QPU
large QPU
distributed quantum system
fault-tolerant machine
future quantum architecture

subject only to:

semantic compatibility
target capabilities
declared approximation policy
available computational resources
available physical resources
explicit operational/security limits

---

178. Final definition of "infinity"

For ZQN:

SUPPORTED SIZE = any finite size representable
                 and processable by available resources

There is no architectural constant:

N_max

Therefore:

1 resource
10 resources
1,000 resources
1,000,000 resources
1,000,000,000 resources
...

are all semantically valid sizes.

Whether a particular implementation can execute them is determined by:

memory
CPU
GPU
distributed resources
storage
runtime policy
target capability
physical hardware
simulation representation

not by a ZQN source-level ceiling.

---

179. Final production rule

The most important rule in this document is:

«Never encode a limitation as a semantic fact when it is actually an implementation or resource-policy constraint.»

Therefore:

"this target has 127 physical qubits"

is valid target data.

"ZQN supports 127 qubits"

is invalid architectural design.

Likewise:

"this simulator permits 1,000,000 samples"

is valid resource policy.

"ZQN supports at most 1,000,000 samples"

is invalid semantic design.

This distinction is what allows Zamani to scale from the smallest quantum computation to arbitrarily large finite computations as resources become available.

---

180. Production completion checklist

Before declaring ZQN production-ready, all of the following must be true:

[ ] malformed trailing-space mod.rs path corrected
[ ] zqn exposed from quantum/mod.rs only after compilation validity
[ ] #![forbid(unsafe_code)] enforced
[ ] no competing QubitId exists
[ ] canonical quantum::ir::qubit::QubitId used
[ ] canonical PhysicalQubitId used
[ ] no machine-size semantic constants
[ ] resource limits are explicit policy
[ ] no global mutable state
[ ] no hidden RNG
[ ] deterministic stochastic execution implemented
[ ] parallel determinism tested
[ ] channel semantics validated
[ ] probability semantics validated
[ ] fault semantics separated from channels
[ ] correlation model scalable
[ ] leakage supported
[ ] erasure supported
[ ] loss supported
[ ] measurement noise supported
[ ] preparation noise supported
[ ] reset noise supported
[ ] idle noise supported
[ ] pulse noise supported
[ ] transport noise supported
[ ] crosstalk supported
[ ] drift supported
[ ] non-Markovian model supported
[ ] conditional noise supported
[ ] calibration snapshots versioned
[ ] calibration uncertainty represented
[ ] characterization integrated
[ ] provenance implemented
[ ] approximation contracts implemented
[ ] target requirements implemented
[ ] target capabilities implemented
[ ] compatibility validation implemented
[ ] target lowering implemented
[ ] IR integration implemented
[ ] routing integration implemented
[ ] scheduling integration implemented
[ ] QEC adapter implemented
[ ] hardware adapter implemented
[ ] memory integration implemented
[ ] benchmarking integration implemented
[ ] runtime integration implemented
[ ] serialization implemented
[ ] canonical serialization implemented
[ ] schema compatibility implemented
[ ] unit tests implemented
[ ] property tests implemented
[ ] differential tests implemented
[ ] determinism tests implemented
[ ] scaling tests implemented
[ ] compatibility tests implemented
[ ] integration tests implemented
[ ] fuzzing implemented
[ ] security/resource-exhaustion tests implemented
[ ] Rust 1.97 compilation verified
[ ] Rust 1.97.1 compilation verified
[ ] no nightly features
[ ] no unsafe code
[ ] documentation complete
[ ] public API reviewed
[ ] dependency graph verified
[ ] no circular ownership
[ ] no vendor coupling
[ ] no second quantum IR

---

181. Architectural conclusion

ZQN should ultimately be understood as:

                 ZQN
                  │
      ┌───────────┼───────────┐
      │           │           │
 probability   channels     faults
      │           │           │
      └───────────┼───────────┘
                  │
                noise
                  │
       ┌──────────┼──────────┐
       │          │          │
 calibration characterization simulation
       │          │          │
       └──────────┼──────────┘
                  │
             propagation
                  │
               target
                  │
             integration

while:

quantum::ir

remains the canonical answer to:

"What does this computation mean?"

and ZQN remains the canonical answer to:

"What physical uncertainty and noise affect this computation?"

That separation is the foundation required for Zamani to support write once, scale everywhere quantum programming without embedding today's machine sizes, vendors, gate sets, topologies, or noise assumptions into the language architecture.The external architecture also supports this direction: QIR explicitly targets interoperability across heterogeneous quantum processors and separates target profiles/capabilities from the quantum instruction set, while MLIR's interface model is specifically intended to prevent transformations from being coupled to every concrete implementation. 

