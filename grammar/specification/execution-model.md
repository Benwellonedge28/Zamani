Below is the complete production-ready content for:

github.com/Benwellonedge28/Zamani/grammar/specification/execution-model.md

It is designed to sit after semantic-model.md and integrate with compilation-model.md, scalability-model.md, poco-reaf.md, the domain grammar, and the repository’s compiler/runtime subsystems without making execution semantics depend on a particular machine, backend, scheduler, quantum processor, topology, or resource count.

Zamani Execution Model

Path: "grammar/specification/execution-model.md"
Status: Normative language specification
Scope: Zamani execution semantics across classical, quantum, hybrid, HDL, hardware, distributed, AI/ML, data, networking, accelerator, embedded, and future computing domains
Implementation baseline: Rust 1.97 / Rust 1.97.1
Implementation safety requirement: "unsafe" Rust MUST NOT be used
Primary objective: Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)

---

1. Purpose

This document defines the normative execution model of the Zamani programming language.

It specifies how a semantically valid Zamani program progresses from a portable computational description to an execution realization without making the source program dependent on:

- a particular processor;
- a particular quantum processor;
- a particular number of qubits;
- a particular number of cores;
- a particular number of threads;
- a particular accelerator;
- a particular memory capacity;
- a particular device;
- a particular topology;
- a particular clock;
- a particular network;
- a particular cluster size;
- a particular deployment;
- a particular scheduler;
- a particular compiler implementation;
- or any other temporary physical resource configuration.

The execution model establishes the boundary between:

program meaning
        ↓
execution requirements
        ↓
available capabilities/resources
        ↓
execution realization
        ↓
observable result

The fundamental rule is:

«Zamani source code describes what computation means. Execution infrastructure determines how that computation is realized on available resources.»

Execution MUST therefore preserve the semantic contract established by the language while allowing implementation-specific realization.

---

2. Normative Language

The terms:

- MUST
- MUST NOT
- REQUIRED
- SHALL
- SHALL NOT
- SHOULD
- SHOULD NOT
- MAY
- OPTIONAL

are normative.

Unless explicitly stated otherwise, an execution implementation claiming Zamani conformance MUST satisfy all requirements marked MUST, MUST NOT, REQUIRED, SHALL, or SHALL NOT.

---

3. Execution Model in One View

Zamani execution is modeled as:

Source Program
     │
     ▼
Parsing
     │
     ▼
AST
     │
     ▼
Semantic Analysis
     │
     ▼
Canonical Semantic Representation
     │
     ▼
Compilation / Lowering
     │
     ▼
Execution Artifact
     │
     ▼
Execution Context
     │
     ├── capabilities
     ├── resources
     ├── constraints
     ├── policies
     ├── permissions
     ├── environment
     └── runtime services
     │
     ▼
Execution Planning
     │
     ├── placement
     ├── routing
     ├── scheduling
     ├── lowering
     ├── adaptation
     └── dispatch
     │
     ▼
Execution
     │
     ▼
Observations / Results / Effects

The grammar is responsible only for expressing the source-level execution concepts.

The grammar MUST NOT implement execution.

---

4. Fundamental Separation

Execution MUST preserve the following ownership boundaries.

Concern| Owner
Syntax| "grammar/"
Semantic meaning| semantic analysis
Canonical semantic representation| compiler/IR
Quantum canonical representation| "quantum::ir"
Optimization| optimization subsystem
Physical realization| routing
Ordering and timing| scheduling
Fault/noise description| ZQN
Error correction| QEC
Hardware capabilities/state| hardware HAL
Resource allocation| resource management
Recovery decisions| resilience
Runtime dispatch| runtime
Deployment| execution/deployment infrastructure
Device discovery| hardware/runtime infrastructure
User source semantics| Zamani language specification

No execution component MAY redefine another subsystem's semantic ownership.

---

5. Execution Is a Realization of Meaning

Let:

P

be a semantically valid Zamani program.

Let:

M(P)

represent its canonical semantic meaning.

Let:

C

represent an execution capability environment.

Let:

R

represent available resources.

Let:

K

represent execution constraints.

Let:

Π

represent execution policies.

Execution can be modeled conceptually as:

E = Realize(M(P), C, R, K, Π)

The realization MUST NOT silently change:

M(P)

merely because:

C
R
K
Π

differ.

Instead, the implementation MUST either:

1. realize the program;
2. adapt the implementation while preserving semantics;
3. report that the required realization is unavailable;
4. execute under an explicitly permitted degraded semantic contract;
5. or reject execution.

---

6. No Universal Physical Machine Assumption

Zamani MUST NOT define a universal machine model requiring every target to provide the same physical resources.

The language MAY execute on:

- CPUs;
- multicore CPUs;
- GPUs;
- TPUs;
- FPGAs;
- ASICs;
- quantum processors;
- quantum simulators;
- analog or mixed-signal systems;
- embedded systems;
- clusters;
- supercomputers;
- distributed systems;
- cloud systems;
- edge systems;
- specialized accelerators;
- future computational architectures.

A target that cannot directly execute a particular semantic operation MUST use an appropriate lowering, emulation, decomposition, virtualization, remote execution mechanism, or report capability failure.

The grammar MUST NOT encode a finite list of architectures as the definition of execution.

---

7. Scale Model

Zamani has no language-defined finite machine-size ceiling.

The following MUST NOT be grammar-level execution limits:

MAX_QUBITS
MAX_CORES
MAX_THREADS
MAX_GPUS
MAX_NODES
MAX_MEMORY
MAX_REGISTERS
MAX_TENSOR_SIZE
MAX_DEVICES
MAX_PROGRAM_SIZE

An implementation MAY have resource or representation limits.

Such limits are implementation constraints, not language semantics.

For example:

available_memory = finite

does not imply:

Zamani arrays have a language-defined maximum size

Likewise:

target supports N qubits

does not imply:

Zamani supports only N qubits

The source language is bounded only by its defined semantic representation model and the practical resources required by a particular execution.

---

8. "Infinity" and Practical Limits

"Infinity" in Zamani scalability means:

«The language does not impose an arbitrary finite upper bound where the underlying semantic concept is inherently scalable.»

It does not mean that physical machines possess infinite resources.

Actual execution remains bounded by:

- available memory;
- available storage;
- available compute;
- available quantum resources;
- execution time;
- communication capacity;
- energy;
- device capabilities;
- implementation representation;
- operating-system limits;
- deployment policies;
- provider limits;
- physical laws.

Such limits MUST be represented as resource or capability facts rather than arbitrary grammar restrictions.

---

9. Execution Context

Every execution occurs relative to an execution context.

Conceptually:

ExecutionContext {
    capabilities
    resources
    constraints
    permissions
    policies
    environment
    services
    provenance
}

The concrete runtime representation is owned by the runtime/compiler infrastructure, not the grammar.

The execution context MUST be capable of describing resources dynamically.

It MUST NOT require fixed-size resource arrays.

For example, execution context must conceptually support:

resources = dynamically discovered collection

rather than:

cpu0
cpu1
gpu0
gpu1
qpu0

as a language-level assumption.

---

10. Capabilities

A capability describes something an execution environment can do.

Examples include:

quantum
classical
floating_point
vectorization
tensor_compute
parallel_execution
remote_execution
distributed_execution
mid_circuit_measurement
dynamic_quantum_control
hardware_synthesis
network_communication
secure_execution

Capabilities describe available functionality.

A capability is NOT automatically:

- a resource;
- a device;
- a physical identifier;
- a performance guarantee;
- a topology;
- a scheduling decision.

---

11. Requirements

A requirement states what the program needs for valid execution.

Examples:

requires quantum
requires dynamic_measurement
requires floating_point
requires communication

A requirement MUST NOT silently select a specific device.

For example:

requires quantum

MUST NOT mean:

use device "X"

or:

use exactly N qubits

or:

use topology T

unless those properties are explicitly part of the program's declared semantic contract.

---

12. Constraints

A constraint limits acceptable execution realizations.

Constraints MAY concern:

- precision;
- latency;
- energy;
- reliability;
- memory;
- communication;
- timing;
- topology;
- security;
- locality;
- cost;
- numerical error;
- quantum fidelity;
- resource availability.

Constraints MUST remain distinct from requirements.

For example:

requires quantum

and:

constrain latency

are different semantic concepts.

---

13. Preferences

Preferences influence implementation choice without necessarily making alternatives invalid.

Examples include:

prefer local
prefer parallel
prefer low_latency
prefer energy_efficiency
prefer accelerator
prefer quantum

A preference MUST NOT be interpreted as an unconditional requirement.

The runtime or compiler MAY choose another realization when required capabilities and semantic constraints remain satisfied.

---

14. Hints

Hints provide implementation guidance.

Hints:

- MUST NOT change program meaning;
- MUST NOT become mandatory requirements accidentally;
- MAY be ignored;
- MUST be distinguishable from semantic requirements.

This allows source programs to provide optimization guidance without destroying portability.

---

15. Resource Discovery

Hardware and execution resources MUST be discovered or supplied outside the core source semantics.

The execution system MAY obtain resource information from:

- hardware HAL;
- operating system;
- runtime;
- scheduler;
- deployment configuration;
- cluster manager;
- quantum provider;
- accelerator runtime;
- embedded platform;
- cloud environment.

The grammar MUST NOT perform hardware discovery.

The grammar MUST NOT require compile-time knowledge of the number of available resources unless the language construct explicitly makes that knowledge semantically observable.

---

16. Resource Negotiation

When multiple realizations satisfy the semantic contract, the execution system SHOULD select an appropriate realization according to:

1. required capabilities;
2. semantic constraints;
3. security requirements;
4. correctness requirements;
5. explicit policies;
6. resource availability;
7. preferences;
8. performance considerations;
9. implementation strategy.

Selection order MUST NOT be encoded as an arbitrary machine-specific grammar rule.

---

17. Execution Phases

A production execution system SHOULD conceptually distinguish the following phases:

1. Load
2. Validate
3. Resolve
4. Acquire
5. Prepare
6. Plan
7. Lower
8. Place
9. Route
10. Schedule
11. Dispatch
12. Execute
13. Observe
14. Verify
15. Recover or adapt when permitted
16. Finalize
17. Release

Not every program requires every phase.

The grammar expresses relevant intent; the execution system determines which phases are necessary.

---

18. Load

Loading obtains a program or execution artifact.

Loading MUST validate:

- artifact identity;
- language version;
- semantic version;
- artifact compatibility;
- required extensions;
- integrity metadata where applicable;
- provenance.

Loading MUST NOT silently execute incompatible artifacts.

---

19. Validation

Before execution, the system MUST establish that the artifact is executable under the selected context.

Validation MAY include:

- semantic validation;
- type validation;
- capability validation;
- effect validation;
- resource validation;
- security validation;
- target compatibility;
- numerical compatibility;
- quantum capability compatibility;
- hardware compatibility.

Validation failures MUST be distinguishable from runtime failures.

---

20. Resolution

Resolution maps abstract program references to semantic entities.

Examples:

module names
types
functions
operations
capabilities
effects
dialects
resources
services
interfaces

Resolution MUST NOT bind portable source semantics directly to arbitrary physical device identifiers.

Target-specific resolution belongs to later compilation/deployment stages.

---

21. Preparation

Preparation transforms a valid execution artifact into a form suitable for realization.

It MAY include:

- specialization;
- lowering;
- constant evaluation;
- resource acquisition;
- runtime initialization;
- communication setup;
- quantum session setup;
- accelerator initialization.

Preparation MUST preserve semantic meaning.

---

22. Planning

Planning determines how the program will be realized.

Planning may involve:

- placement;
- routing;
- scheduling;
- partitioning;
- parallelization;
- accelerator selection;
- communication planning;
- quantum mapping;
- error-correction strategy;
- fault mitigation;
- resilience policy.

Planning is NOT grammar semantics.

---

23. Optimization

Optimization MAY change implementation while preserving the semantic contract.

Examples include:

- instruction simplification;
- gate cancellation;
- circuit optimization;
- classical optimization;
- tensor optimization;
- accelerator lowering;
- communication optimization.

Optimization MUST preserve semantic equivalence as defined by "semantic-model.md".

---

24. Quantum Optimization Boundary

Quantum optimization MUST operate through the repository's canonical quantum representation.

The grammar MUST NOT define a competing quantum operation representation.

The architectural boundary is:

Zamani quantum syntax
        ↓
semantic analysis
        ↓
quantum::ir
        ↓
optimization
        ↓
routing
        ↓
scheduling
        ↓
ZQN / hardware / runtime

Existing temporary or duplicated quantum gate structures MUST NOT become a second semantic authority.

---

25. Placement

Placement maps abstract computations or resources to concrete execution resources.

Examples:

logical computation → physical processor
logical qubit → physical qubit
task → execution node
kernel → accelerator
signal → hardware resource
service → distributed node

Placement MUST remain downstream of semantic analysis.

A portable source program SHOULD NOT contain target-specific placement unless explicitly written as a target/deployment artifact or target-specific dialect.

---

26. Routing

Routing determines how computations move through available physical connectivity.

For quantum execution, routing may map logical qubit interactions onto physical connectivity.

For distributed execution, routing may map communication through network paths.

For accelerator execution, routing may determine data movement.

Routing MUST NOT redefine program semantics.

Routing belongs to the routing/physical-realization subsystem.

---

27. Scheduling

Scheduling determines execution order and timing where timing is not already fixed by language semantics.

The execution model MUST distinguish:

semantic ordering

from:

implementation scheduling

Scheduling MAY use:

- ASAP;
- ALAP;
- dependency scheduling;
- resource-aware scheduling;
- critical-path scheduling;
- event scheduling;
- distributed scheduling;
- hardware-specific scheduling.

The scheduling subsystem owns these decisions.

The grammar MUST NOT embed scheduler algorithms.

---

28. Timing

Zamani MUST distinguish:

1. semantic time;
2. logical ordering;
3. explicit program-visible timing;
4. implementation timing;
5. hardware clock/tick representation.

A duration expressed semantically MUST NOT automatically mean a specific hardware clock period.

For example:

duration = 1 microsecond

MUST NOT inherently mean:

exactly N device ticks

Conversion to hardware timing belongs to target lowering/scheduling.

---

29. Dispatch

Dispatch sends prepared work to an execution resource.

Possible destinations include:

- local CPU;
- accelerator;
- FPGA;
- quantum processor;
- simulator;
- embedded device;
- remote process;
- cluster node;
- cloud service;
- distributed worker.

Dispatch MUST use resolved capabilities and policies.

The source grammar MUST NOT directly perform dispatch.

---

30. Classical Execution

Classical execution follows the language's defined classical semantics.

It may execute:

- scalar operations;
- structured data;
- functions;
- control flow;
- recursion;
- vectors;
- matrices;
- tensors;
- numerical algorithms;
- symbolic computation;
- parallel computation;
- accelerator kernels.

The number of:

- cores;
- threads;
- vector lanes;
- accelerators;

MUST remain implementation-dependent unless explicitly observable by a language construct.

---

31. Parallel Execution

Zamani MAY express parallel intent.

The execution system determines how that intent is realized.

Possible realizations include:

single-thread execution
multithreading
SIMD
GPU execution
distributed execution
pipeline execution
hardware parallelism
quantum parallelism

Parallelism MUST NOT introduce observable races unless the language explicitly permits nondeterministic behavior.

Where deterministic semantics are promised, the runtime MUST preserve those semantics regardless of the number of execution workers.

---

32. Concurrency

Concurrent computations MAY execute independently.

The execution model distinguishes:

concurrency

from:

parallel physical execution

A concurrent program MAY execute sequentially when no semantic property requires parallel physical execution.

Conversely, an implementation MAY execute independent work concurrently when semantic rules permit it.

---

33. Synchronization

Synchronization establishes semantic ordering between concurrent operations.

Examples include:

- joins;
- barriers;
- channels;
- locks;
- atomic operations;
- futures;
- events;
- task dependencies;
- distributed acknowledgements.

Synchronization semantics MUST be defined independently of the number of workers.

---

34. Distributed Execution

Distributed execution MAY span an arbitrary number of execution domains.

The language MUST NOT define a fixed node count.

Conceptually:

logical computation
       ↓
partition
       ↓
placement
       ↓
communication
       ↓
execution
       ↓
aggregation

The number of nodes is an execution-context property.

Distributed semantics MUST explicitly define consistency and communication guarantees where they are observable.

The language MUST NOT assume a global total order where one is not semantically guaranteed.

---

35. Remote Execution

A computation MAY be executed remotely when its declared effects, security requirements, data policies, and capabilities permit remote execution.

Remote execution MUST preserve:

- input meaning;
- output meaning;
- declared effects;
- security constraints;
- required provenance;
- applicable failure semantics.

Network latency MUST NOT silently become part of program semantics unless timing is explicitly observable.

---

36. Faults and Failures

Execution failures are distinct from semantic errors.

Examples include:

resource unavailable
backend unavailable
network failure
device failure
timeout
memory exhaustion
quantum execution failure
hardware fault
provider failure
process termination
security rejection

A failure MUST NOT silently be represented as a successful program result.

---

37. Quantum Execution

Quantum execution follows the canonical quantum semantics defined elsewhere.

The conceptual pipeline is:

Zamani quantum syntax
        ↓
semantic validation
        ↓
quantum::ir
        ↓
optimization
        ↓
logical/physical realization
        ↓
routing
        ↓
scheduling
        ↓
ZQN / QEC / hardware integration
        ↓
runtime execution

The grammar MUST NOT directly encode:

- device topology;
- physical qubit count;
- pulse schedule;
- calibration values;
- backend-specific gate decomposition;
- hardware timing grid.

---

38. Logical Qubits

Portable quantum programs SHOULD operate primarily on logical quantum identities.

Logical qubits represent computational resources at the semantic level.

Their physical realization is determined later.

A logical quantum computation MAY therefore be realized on:

- different physical qubit counts;
- different topologies;
- different gate sets;
- different control systems;
- simulators;
- future quantum architectures.

The source semantics MUST remain stable when the physical mapping changes.

---

39. Physical Qubits

Physical qubit identifiers are target-scoped concepts.

They MAY exist in:

- hardware descriptions;
- deployment specifications;
- target-specific dialects;
- low-level quantum programs.

They MUST NOT leak into portable semantic identity accidentally.

A physical identifier MUST NOT be treated as a globally stable language-level identity.

---

40. Quantum Measurement

Quantum measurement is an observable execution event.

Measurement MAY produce nondeterministic outcomes according to the defined quantum semantics.

The runtime MUST expose the resulting classical observation according to the program's declared measurement semantics.

Measurement MUST NOT be silently inserted merely because a quantum program finishes.

Likewise, automatic measurement of all unmeasured qubits MUST NOT be a general language rule unless explicitly defined by a separate language construct.

---

41. Mid-Circuit Measurement

Mid-circuit measurement MUST be treated as a semantic event.

It may:

produce classical data
alter quantum state
control subsequent computation
terminate a branch
trigger classical logic

The execution system MUST preserve its ordering relative to dependent operations.

Scheduling MAY move physically independent operations, but MUST NOT violate semantic dependencies.

---

42. Reset

Quantum reset is a semantic operation when explicitly requested.

It MUST NOT be inserted automatically merely to simplify execution unless the transformation is proven semantically preserving under the language contract.

Hardware-specific reset mechanisms are downstream implementation details.

---

43. Dynamic Quantum Execution

Dynamic circuits MAY contain runtime-dependent control flow.

Conceptually:

quantum operation
      ↓
measurement
      ↓
classical value
      ↓
condition
      ↓
quantum/classical operation

The execution system MUST preserve this dependency.

A backend incapable of dynamic execution MAY:

1. lower the computation if semantic equivalence is preserved;
2. transform it into a supported representation;
3. use a simulator/emulation mechanism;
4. reject execution because the required capability is unavailable.

---

44. QEC Boundary

Quantum error correction belongs to the QEC subsystem.

The execution model MAY carry:

- QEC intent;
- QEC requirements;
- logical reliability requirements;
- correction policies;
- protected-region metadata.

It MUST NOT define QEC algorithms.

The grammar MUST NOT duplicate QEC implementation structures.

Execution integration is:

quantum semantics
      ↓
QEC intent
      ↓
QEC subsystem
      ↓
physical realization

---

45. ZQN Boundary

ZQN owns the semantic description of quantum noise and faults.

Execution MAY consume ZQN information to determine:

- execution risk;
- expected fault behavior;
- mitigation applicability;
- verification requirements.

ZQN MUST NOT become part of the grammar's core quantum operation semantics.

The execution model therefore distinguishes:

program meaning

from:

execution conditions

Noise or fault occurrence does not automatically redefine what the source program means.

---

46. Resilience Boundary

Resilience determines how execution should react to faults and changing conditions.

Possible actions include:

retry
restart
resume
rollback
remap
reroute
reschedule
recompile
reoptimize
change_qec
mitigate
switch_backend
quarantine_resource
abort

These actions belong to the resilience subsystem.

The grammar MAY express resilience requirements or policies.

It MUST NOT embed a particular recovery implementation.

---

47. Recovery and Checkpoints

Execution MAY create checkpoints.

A checkpoint MUST identify what is actually reconstructible.

The language MUST distinguish:

classical execution state
compiled program state
logical quantum checkpoint
measurement boundary
QEC-protected state
provider-supported state

The language MUST NOT imply that an arbitrary unknown quantum state can always be serialized, copied, or restored.

Recovery MUST respect the physical and semantic properties of the computation.

---

48. Mitigation

Execution MAY use error mitigation where declared or permitted.

Examples include:

- readout mitigation;
- zero-noise extrapolation;
- probabilistic error cancellation;
- twirling;
- dynamical decoupling.

Mitigation is distinct from QEC.

Where dynamical decoupling requires scheduling or pulse control, the execution system MUST delegate physical realization to scheduling/hardware infrastructure.

---

49. HDL Execution

HDL semantics may represent:

- combinational logic;
- sequential logic;
- signals;
- registers;
- clocks;
- state machines;
- memories;
- pipelines;
- interfaces;
- processes;
- timing relationships.

Execution of HDL may mean:

simulation
synthesis
formal verification
hardware generation
emulation
hardware deployment

These are different execution realizations of hardware semantics.

The source description MUST NOT silently become tied to a specific FPGA, ASIC process, clock frequency, or vendor.

---

50. Simulation Versus Physical Execution

Simulation and physical execution MAY produce different performance and timing characteristics while preserving the same semantic contract where applicable.

For example:

quantum simulator

and:

quantum processor

MAY differ in:

- performance;
- noise;
- timing;
- resource consumption;
- available capabilities.

Such differences MUST be represented through execution context and declared contracts.

---

51. Hardware/Software Co-Execution

A Zamani program MAY partition computation across:

software
hardware
quantum hardware
accelerators
network resources

The partitioning is an implementation decision unless explicitly declared as semantic behavior.

The source program SHOULD describe computational relationships rather than arbitrary physical placement.

---

52. Accelerator Execution

Accelerator execution MAY target:

- GPU;
- FPGA;
- ASIC;
- tensor accelerator;
- vector accelerator;
- quantum accelerator;
- future specialized hardware.

The execution system MUST select an available compatible accelerator.

The grammar MUST NOT require:

gpu_count = fixed value

or:

accelerator_id = fixed device

for portable execution.

---

53. Memory Semantics

Execution MUST distinguish semantic memory from physical memory.

The source language MAY describe:

- values;
- references;
- ownership;
- lifetimes;
- allocation;
- shared memory;
- distributed memory;
- persistent state.

Physical memory hierarchy is implementation-specific unless explicitly exposed by a language feature.

The execution system MAY map logical storage to:

- registers;
- caches;
- RAM;
- GPU memory;
- FPGA memory;
- distributed memory;
- persistent storage;
- future storage technologies.

---

54. Data Movement

Data movement MAY occur between:

CPU
GPU
FPGA
ASIC
QPU
memory
storage
network
distributed node

Data movement SHOULD be represented as an effect or implementation concern where appropriate.

The language MUST NOT assume that all resources share a single uniform memory space.

---

55. Networking

Network communication is an execution effect.

Network topology, routing, addresses, endpoints, and provider details belong to networking/deployment infrastructure unless explicitly part of the program's semantics.

The number of network nodes MUST NOT be fixed by the grammar.

---

56. Security

Execution MUST respect declared:

- permissions;
- identities;
- trust requirements;
- isolation;
- confidentiality;
- integrity;
- cryptographic requirements;
- data locality;
- privacy policies.

A runtime MUST NOT bypass security requirements merely to find an executable target.

Security failure MUST be observable as execution failure rather than silently ignored.

---

57. Capability Failure

A capability failure occurs when the execution context cannot satisfy a required semantic capability.

Example:

program requires dynamic quantum control

but:

target lacks dynamic quantum control

The system MUST NOT silently reinterpret the program as a different computation.

It MAY lower or emulate the behavior when that transformation is semantically valid.

Otherwise execution MUST be rejected.

---

58. Resource Failure

A resource failure occurs when the required capability exists but sufficient resources are unavailable.

Examples:

insufficient memory
insufficient compute capacity
insufficient quantum resources
insufficient storage
insufficient communication capacity

Resource failure is distinct from semantic invalidity.

A valid program can therefore fail to execute on an insufficient target.

---

59. Dynamic Resource Availability

Resources MAY change during execution.

Examples:

- worker loss;
- accelerator loss;
- QPU unavailability;
- network degradation;
- changing cloud capacity;
- thermal limitations;
- hardware degradation.

Execution systems MAY adapt when the semantic contract permits adaptation.

Such adaptation MUST NOT silently change program meaning.

---

60. Runtime Adaptation

Runtime adaptation MAY include:

resource migration
task migration
rescheduling
backend switching
recompilation
remapping
fault mitigation
replication
checkpoint recovery

Adaptation is valid only when the resulting execution remains within the program's semantic contract.

---

61. Determinism

Zamani distinguishes:

deterministic semantics

from:

deterministic physical execution

An implementation MUST preserve deterministic semantic behavior when the program requires it.

Physical execution MAY still vary in:

- scheduling;
- placement;
- latency;
- resource selection;
- instruction layout.

These variations are permitted only when they do not change observable semantics.

---

62. Nondeterminism

Nondeterminism MUST be explicit or semantically justified.

Sources may include:

- quantum measurement;
- randomness;
- concurrent scheduling;
- distributed races where explicitly permitted;
- external inputs;
- hardware events.

Where nondeterminism is part of the semantic model, conformance MUST be defined over permitted outcomes or distributions rather than requiring a single output.

---

63. Randomness

Randomness MUST be distinguishable from accidental nondeterminism.

A program MAY explicitly request:

- deterministic pseudorandomness;
- cryptographic randomness;
- hardware randomness;
- quantum randomness;
- externally supplied randomness.

The execution system MUST honor the declared semantics.

---

64. Numerical Execution

Numerical semantics MUST define the permitted precision and error behavior.

Execution MAY use:

- scalar floating point;
- arbitrary precision;
- vector arithmetic;
- tensor arithmetic;
- accelerator arithmetic;
- symbolic representations.

An implementation MUST NOT silently reduce precision when doing so violates the program's numerical contract.

Where numerical variation is permitted, it MUST be governed by explicit semantic tolerances or numerical contracts.

---

65. Observability

Execution correctness is evaluated through observable behavior.

Observable behavior may include:

- returned values;
- output data;
- measurements;
- mutations;
- I/O;
- messages;
- externally visible signals;
- persistent state;
- declared timing behavior;
- security events;
- failure states.

Internal implementation details are not observable unless the language explicitly exposes them.

---

66. Timing Observability

Execution time is not automatically semantic.

A program's physical runtime MAY vary across targets.

Timing becomes semantic only when the language explicitly defines it as observable, such as:

- hardware timing constructs;
- real-time deadlines;
- explicit synchronization;
- HDL timing;
- latency contracts.

Even then, implementation MUST distinguish semantic timing requirements from physical clock representation.

---

67. Semantic Equivalence

Two execution realizations are equivalent when they satisfy the same permitted observable behavior.

Conceptually:

Realization(A) ≡ Realization(B)

when all required observations remain equivalent under the program's declared contract.

For probabilistic or quantum computation, equivalence MAY require equality or permitted approximation of:

- probability distributions;
- measurement distributions;
- numerical results;
- expectation values;
- statistical properties.

---

68. Approximate Execution

Some domains inherently permit approximation.

Approximation MUST be explicit.

Examples include:

numerical tolerance
quantum approximation
simulation approximation
AI inference tolerance
resource/performance tradeoffs

An implementation MUST NOT silently convert an exact computation into an approximate one without a semantic rule permitting that transformation.

---

69. Execution Contracts

Every execution artifact SHOULD carry enough information to establish:

language version
semantic version
artifact version
required capabilities
constraints
effects
resource requirements
security requirements
provenance
compatibility information

The artifact MUST NOT need to contain transient physical facts unless those facts are explicitly part of its target/deployment contract.

---

70. Provenance

Execution artifacts MUST preserve provenance sufficient to determine:

- source identity;
- relevant source locations;
- language version;
- semantic version;
- compiler/toolchain identity where required;
- transformation lineage;
- dialect versions;
- relevant compilation configuration.

Provenance MUST NOT require secrets.

Sensitive credentials, tokens, private keys, or authentication material MUST NOT be embedded merely for provenance.

---

71. Compiler/Runtime Boundary

The compiler determines how a semantic program can be realized.

The runtime determines how a prepared execution artifact interacts with actual execution resources.

The boundary is:

Compiler
  ↓
Execution Artifact
  ↓
Runtime
  ↓
Execution Context
  ↓
Resources

The runtime MUST NOT need to reinterpret source grammar in order to execute an already validated artifact.

---

72. Grammar Boundary

The grammar's responsibility ends at syntactic representation.

The execution grammar MAY provide syntax for:

- execution declarations;
- deployment intent;
- resource requirements;
- capabilities;
- constraints;
- effects;
- scheduling hints;
- placement preferences;
- execution policies.

It MUST NOT encode:

- runtime algorithms;
- scheduler implementations;
- device discovery;
- QEC algorithms;
- noise models;
- hardware calibration;
- physical routing algorithms;
- compiler internals.

---

73. Execution Syntax and Semantic Ownership

Execution-related grammar constructs MUST be interpreted by semantic analysis.

For example:

requires
constraint
prefer
target
placement
execute
parallel
distributed

are syntax-level concepts.

Their semantic meaning belongs to the semantic/compiler model.

Their physical realization belongs to compilation/runtime infrastructure.

---

74. Compile Once

POCO-REAF requires careful interpretation of "compile once."

A single source program SHOULD compile into a portable semantic or intermediate artifact wherever possible.

However, this does NOT require one machine-specific executable binary to run unchanged on every possible architecture.

Instead:

Source
   ↓
Portable semantic artifact
   ↓
Target realization
   ↓
Execution

is the preferred model.

Target-specific specialization MAY occur without requiring source-code modification.

---

75. Run Everywhere

"Run Everywhere" means that a semantically portable program can be realized across compatible execution environments.

Portability depends on:

- required capabilities;
- semantic constraints;
- effects;
- numerical guarantees;
- security requirements;
- supported dialects;
- resource availability.

A target that lacks required capabilities is not required to execute the program.

---

76. Run Anywhere

Execution MAY occur:

- locally;
- remotely;
- on embedded hardware;
- on accelerators;
- on quantum processors;
- on simulators;
- on clusters;
- in clouds;
- across distributed systems;
- across heterogeneous environments.

The location of execution MUST NOT inherently change program semantics.

---

77. Run Forever

"Forever" means that Zamani's semantic model is designed for long-term evolution.

Future hardware MUST be able to implement existing semantics without requiring existing source programs to be rewritten solely because hardware changed.

Future capabilities SHOULD be introduced through:

- versioned extensions;
- dialects;
- capability declarations;
- target descriptions;
- new lowering strategies;
- new runtime implementations.

---

78. Execution and Versioning

The following versions MUST remain distinguishable:

language version
grammar version
semantic version
AST version
IR version
execution artifact version
dialect version
target interface version
runtime version

Changing one MUST NOT implicitly redefine the others.

Compatibility MUST be evaluated explicitly.

---

79. Backward Compatibility

A newer runtime SHOULD execute older compatible artifacts.

A newer compiler SHOULD preserve the meaning of older valid programs unless a documented language-version change explicitly changes semantics.

Breaking execution changes MUST be versioned and documented.

---

80. Forward Compatibility

Execution artifacts SHOULD be extensible enough to survive future target evolution.

Unknown optional metadata MAY be ignored when doing so is safe.

Unknown required semantic constructs MUST NOT be silently ignored.

---

81. Dialects

Execution dialects MAY extend the language for:

- hardware;
- quantum platforms;
- accelerators;
- distributed systems;
- embedded systems;
- future architectures.

A dialect MUST:

- have a namespace;
- have a version;
- declare capabilities;
- define ownership;
- define compatibility;
- avoid silently redefining core semantics.

Vendor-specific execution behavior MUST remain isolated from the portable core.

---

82. Execution Policies

Execution policies MAY determine:

- retry behavior;
- resource selection;
- locality;
- cost preferences;
- performance objectives;
- reliability objectives;
- resilience;
- backend selection;
- scheduling preferences.

Policies MUST be distinguished from program semantics.

Changing a policy MUST NOT silently change the mathematical or computational meaning of the program unless the policy is explicitly part of the program's semantic contract.

---

83. Failure Policy

Execution failure handling MUST distinguish:

retryable
recoverable
degradable
fatal
unknown

The classification belongs to runtime/resilience infrastructure.

The grammar MAY declare whether failure is:

- acceptable;
- retryable;
- recoverable;
- required to abort.

It MUST NOT hard-code a particular recovery algorithm.

---

84. Cancellation

Cancellation is an execution event.

A cancelled computation MUST produce a defined execution state.

The runtime MAY support:

cooperative cancellation
forced cancellation
deadline cancellation
resource-driven cancellation
user cancellation

Cancellation MUST NOT be confused with successful completion.

---

85. Resource Release

Execution MUST define resource-release behavior for resources acquired by the program.

This may include:

- memory;
- files;
- network connections;
- accelerator sessions;
- quantum sessions;
- hardware resources;
- distributed leases.

Resource release MUST occur according to the language's ownership/effect semantics and runtime guarantees.

---

86. Execution Effects

Effects describe externally relevant execution behavior.

Examples:

IO
network
hardware
quantum
distributed
security
persistent_state
randomness
external_service

Effect information MAY influence where and how execution occurs.

A target MUST NOT execute an effect requiring unavailable or unauthorized capabilities.

---

87. External State

Programs interacting with external state cannot generally guarantee identical outputs across all executions.

The execution model therefore distinguishes:

pure computation

from:

environment-dependent computation

External state MUST be represented through explicit effects or semantic inputs where applicable.

---

88. Reproducibility

A program MAY request reproducibility.

Reproducibility MAY require:

- deterministic random seeds;
- fixed semantic versions;
- stable input data;
- deterministic scheduling semantics;
- controlled numerical behavior;
- fixed dialect versions;
- captured execution configuration.

Physical identity need not be fixed unless required by the semantic contract.

---

89. Security-Critical Execution

Security-sensitive computations MUST NOT silently migrate to resources that violate security requirements.

Examples include:

trusted execution
data locality
confidential computation
cryptographic isolation
identity requirements

Execution planning MUST respect these constraints.

---

90. Execution in Embedded Systems

An embedded target MAY have extremely limited resources.

The language MUST NOT require source-level redesign merely because the target has:

- less memory;
- fewer compute resources;
- fewer accelerators;
- limited networking;
- different instruction sets.

Compilation MAY specialize the program to the target.

If the target cannot satisfy the program's requirements, execution MUST fail explicitly.

---

91. Execution on Large Systems

The same semantic program MAY be realized on:

- multicore machines;
- many-accelerator systems;
- clusters;
- supercomputers;
- distributed infrastructures.

Scaling MAY be achieved through:

- parallelization;
- partitioning;
- vectorization;
- replication;
- distribution;
- accelerator execution;
- quantum execution.

The source language MUST NOT require source changes merely because the available resource scale increases.

---

92. Elastic Execution

Where permitted, execution MAY scale resources dynamically.

For example:

small workload → small realization
large workload → large realization

without changing source semantics.

Elasticity MUST be governed by resource policies and execution infrastructure.

---

93. Resource Exhaustion

Resource exhaustion is an execution condition.

It MUST NOT become undefined language behavior.

The runtime SHOULD report:

- resource category;
- operation affected;
- relevant requirement;
- whether recovery is possible;
- provenance;
- execution state.

It MUST NOT falsely report successful completion.

---

94. No Hidden Execution Transformations

The execution system MUST NOT silently introduce semantically observable transformations such as:

- automatic measurement;
- automatic reset;
- arbitrary approximation;
- hidden data loss;
- hidden synchronization;
- hidden communication;
- hidden mutation;
- hidden nondeterminism.

Transformations are valid only when:

1. explicitly requested;
2. semantically implied;
3. permitted by a documented optimization rule;
4. or proven semantically equivalent.

---

95. Execution Verification

Where required by the program or execution policy, the system SHOULD verify results.

Verification MAY include:

- invariant checking;
- type/state validation;
- numerical checks;
- quantum result validation;
- QEC verification;
- hardware verification;
- formal verification;
- statistical verification;
- provenance validation.

Verification is distinct from execution itself.

---

96. Observability and Telemetry

Runtime telemetry MAY include:

- execution duration;
- resource utilization;
- latency;
- throughput;
- failures;
- retries;
- recovery;
- hardware health;
- quantum error rates;
- logical error rates;
- readout error;
- gate error;
- queue time.

Telemetry MUST NOT silently become program semantics unless explicitly exposed through the language.

---

97. Execution Logging

Logs MUST be distinguishable from program output.

Diagnostics SHOULD preserve:

- execution identity;
- artifact identity;
- operation identity;
- source provenance;
- failure category;
- resource context.

Secrets MUST NOT be written into logs merely because they were available to the runtime.

---

98. Deterministic Planning

Where deterministic execution planning is required, equivalent inputs MUST produce deterministic planning results.

Determinism MUST NOT depend on:

- hash-map iteration order;
- nondeterministic device discovery order;
- thread race timing;
- provider response ordering.

Canonical ordering MUST be defined by the relevant subsystem.

---

99. Resource Ordering

When multiple resources are semantically equivalent, the runtime MAY select any valid resource unless the program explicitly constrains selection.

If deterministic selection is required, the runtime MUST use a documented deterministic ordering.

The language MUST NOT define arbitrary device IDs merely to force deterministic execution.

---

100. Execution State Machine

Conceptually, an execution MAY transition through:

Created
   ↓
Validated
   ↓
Resolved
   ↓
Prepared
   ↓
Planned
   ↓
Dispatched
   ↓
Running
   ├── Paused
   ├── Recovering
   ├── Adapting
   └── Failed
   ↓
Completed
   ↓
Finalized

Not all implementations must expose these states publicly.

The states are semantic execution concepts, not a requirement for a particular runtime data structure.

---

101. Pause and Resume

Pause/resume MAY be supported when the execution resource permits it.

For quantum state, pause/resume semantics MUST respect the actual ability of the target to preserve the relevant state.

The language MUST NOT imply arbitrary serialization of quantum state.

---

102. Migration

A computation MAY migrate between resources if:

- its semantic state is transferable;
- the target satisfies required capabilities;
- security requirements remain satisfied;
- observable semantics remain valid.

Migration of arbitrary physical quantum state MUST NOT be assumed possible.

---

103. Execution and Persistence

Persistent execution state MUST be explicitly defined.

A runtime MUST distinguish:

persistent classical data

from:

transient execution state

and:

non-serializable physical state

Persistence MUST NOT be inferred merely from variable existence.

---

104. AI/ML Execution

AI/ML execution MAY involve:

- model construction;
- training;
- inference;
- tensor computation;
- automatic differentiation;
- accelerator execution;
- distributed training.

The execution system MAY map these operations to CPU, GPU, FPGA, ASIC, quantum, or future accelerators.

Model semantics MUST remain distinct from accelerator implementation.

---

105. Data Execution

Data computations MAY operate over:

- collections;
- streams;
- records;
- tensors;
- distributed datasets;
- persistent stores.

The execution system MAY partition data dynamically.

The number of records, nodes, shards, or partitions MUST NOT be fixed by the grammar.

---

106. Future Computing

The execution model MUST remain open to future computational paradigms.

A future execution model MUST be integrable through:

new capability
new resource type
new dialect
new lowering
new runtime
new target interface

without requiring a redesign of the entire language.

Unknown future hardware MUST NOT invalidate the core execution model merely because it is not named in the current grammar.

---

107. Execution and Interoperability

Foreign systems MAY be invoked through interoperability mechanisms.

A foreign operation MUST declare, where applicable:

- interface;
- ABI;
- effects;
- ownership;
- data representation;
- security requirements;
- failure behavior;
- version.

The execution system MUST NOT assume foreign operations are pure or portable unless declared.

---

108. Compile-Time Versus Runtime Execution

Compile-time execution and runtime execution MUST remain distinct.

Compile-time computation MAY determine:

- types;
- constants;
- generated code;
- specialization;
- static resource expressions.

Runtime computation occurs against actual execution context.

Compile-time execution MUST NOT require arbitrary runtime hardware discovery unless explicitly supported by a capability/effect model.

---

109. Compile-Time Safety

Compile-time evaluation MUST be deterministic where the language promises deterministic compilation.

It MUST NOT have unrestricted access to:

- filesystem;
- network;
- credentials;
- external services;

unless explicitly authorized through language effects/capabilities.

This prevents compilation from becoming an uncontrolled execution environment.

---

110. Execution Security Boundary

The compiler MUST NOT assume that an execution target is trusted merely because it satisfies capability requirements.

Trust is a separate property.

Execution selection MAY require:

capability
+
trust
+
permission
+
security policy

all to be satisfied.

---

111. Resource Identity

Runtime resource identifiers MUST be scoped.

For example:

physical device identifier

is not equivalent to:

semantic computation identity

The latter MUST remain stable across compatible realizations.

This distinction is critical for POCO-REAF.

---

112. Operation Identity

Semantic operations SHOULD have stable identities through the compiler/IR pipeline.

An operation identity MUST NOT depend solely on:

- physical instruction address;
- hardware device ID;
- memory address;
- scheduler slot.

This permits provenance across optimization, routing, scheduling, and runtime.

---

113. Execution Provenance

A runtime SHOULD be able to relate:

source construct
   ↓
AST node
   ↓
semantic entity
   ↓
IR entity
   ↓
optimized entity
   ↓
scheduled/routed entity
   ↓
runtime operation

This mapping supports:

- diagnostics;
- verification;
- debugging;
- reproducibility;
- resilience;
- auditing.

---

114. Optimization and Provenance

Optimization MAY combine, remove, or transform operations.

The resulting artifact SHOULD preserve provenance sufficient to explain the transformation.

Optimization MUST NOT destroy semantic traceability unnecessarily.

---

115. Execution and Diagnostics

Diagnostics MUST distinguish:

Source error

The program is syntactically invalid.

Semantic error

The program violates language semantics.

Capability error

The target lacks a required capability.

Resource error

Required resources are unavailable.

Security error

Execution violates security policy.

Deployment error

The requested deployment cannot be established.

Runtime error

Execution failed after dispatch.

Verification error

The result failed required verification.

These categories MUST NOT be collapsed into a generic "execution failed" message.

---

116. Execution Error Stability

Core semantic error categories MUST remain target-neutral.

Provider-specific errors MAY be attached as implementation details.

The core language MUST NOT depend on provider-specific numeric error codes.

---

117. Graceful Degradation

Degradation MAY be allowed when explicitly permitted.

For example:

preferred accelerator unavailable

may fall back to:

CPU

if the semantic contract permits it.

However:

required quantum capability unavailable

MUST NOT silently fall back to classical computation if doing so changes program semantics.

---

118. Portability Classes

Execution systems SHOULD distinguish at least:

Class A — Semantic portability

The same source semantics are realizable on the target.

Class B — Representation portability

The same portable artifact can be lowered to the target.

Class C — Execution portability

The runtime can execute the lowered artifact on the target.

Class D — Exact observational equivalence

The target produces the same required observations.

Class E — Contractual approximation

The target produces observations within explicitly permitted tolerances.

This avoids making unrealistic claims that every target is physically identical.

---

119. Target Specialization

Target specialization is permitted.

Examples include:

- instruction selection;
- vectorization;
- gate decomposition;
- FPGA synthesis;
- ASIC synthesis;
- GPU kernel generation;
- quantum mapping;
- distributed partitioning.

Specialization MUST remain downstream of portable semantic meaning.

---

120. Target-Specific Source

A developer MAY intentionally write target-specific source.

Such source MUST be distinguishable from portable source.

Target-specific constructs SHOULD use:

- explicit target declarations;
- target dialects;
- hardware namespaces;
- deployment specifications;
- explicit physical identifiers.

This prevents target-specific behavior from accidentally becoming part of the universal core language.

---

121. Execution Contracts for Quantum Backends

A quantum backend contract MAY describe:

supported operations
supported measurement modes
dynamic control
supported QEC modes
supported precision
timing capabilities
resource availability
noise characteristics

These are backend capabilities.

They MUST NOT redefine the canonical meaning of the source program.

---

122. Execution Contracts for Hardware

Hardware targets MAY expose:

ports
interfaces
timing
clock domains
memory
accelerators
compute resources
constraints
synthesis capabilities

These belong to the hardware/target model.

They MUST NOT force portable source programs to embed a fixed machine topology.

---

123. Execution Contracts for Distributed Systems

Distributed targets MAY expose:

node capabilities
communication capabilities
consistency models
failure domains
placement constraints
resource availability

The execution system MAY use these to construct a valid realization.

---

124. Execution Contracts for Embedded Systems

Embedded targets MAY expose:

memory capacity
compute capability
peripheral availability
timing requirements
power constraints
hardware interfaces

The compiler MAY specialize the artifact accordingly.

The source semantics MUST remain independent of those transient capacities unless explicitly target-specific.

---

125. No Source Rewriting for Scale

Increasing or decreasing available resources MUST NOT require semantic source rewriting.

For example, a computation should not need separate source programs merely because it executes on:

1 worker

versus:

many workers

unless the developer explicitly requests different semantics.

---

126. Scale-Aware Compilation

Compilation MAY specialize a program according to resource availability.

For example:

small resources
    → sequential realization

larger resources
    → parallel realization

or:

small quantum backend
    → decomposition/partitioning

larger quantum backend
    → larger direct realization

The specialization MUST preserve semantic meaning.

---

127. Execution and Resource Expressions

Resource expressions MAY be symbolic.

For example, a program may semantically require:

resources proportional to input size

rather than:

resources = fixed number

The compiler/runtime MAY evaluate such expressions against the execution context.

---

128. Dynamic Resource Requirements

Resource requirements MAY depend on:

- input shape;
- workload size;
- algorithmic complexity;
- runtime state;
- data size;
- quantum circuit structure.

Such requirements MUST be resolved without introducing arbitrary grammar ceilings.

---

129. Execution of Generic Programs

Generic programs MUST remain independent of a particular resource scale.

A generic algorithm MAY instantiate differently for:

small input
large input
small machine
large machine
CPU
GPU
quantum accelerator
distributed system

without changing its semantic definition.

---

130. Execution and Shape

Shapes MAY be:

- static;
- symbolic;
- inferred;
- dynamic.

The language MUST distinguish:

semantic shape constraint

from:

current hardware capacity

For example, a tensor may have a symbolic dimension without forcing the compiler to assume a fixed physical tensor size.

---

131. Execution and Memory Pressure

Memory pressure is an execution condition.

The compiler MAY:

- tile;
- stream;
- partition;
- spill;
- distribute;
- recompute;
- use an accelerator.

Such transformations MUST preserve semantics.

---

132. Execution and Energy

Energy MAY be expressed as:

- requirement;
- constraint;
- preference;
- telemetry.

Energy is not automatically a semantic property of every computation.

The runtime MAY optimize energy while preserving the program's semantic contract.

---

133. Execution and Reliability

Reliability MAY be expressed as a requirement or constraint.

The runtime MAY respond through:

- redundancy;
- QEC;
- replication;
- retries;
- verification;
- migration;
- backend switching.

The execution model does not define the algorithms used to achieve reliability.

---

134. Execution and Resilience

Resilience MAY observe:

execution failures
hardware health
resource health
noise
latency
verification results

and determine an appropriate action.

Resilience MUST remain separate from the core execution semantics.

---

135. Execution and Scheduling/Resilience Interaction

The architecture is:

Execution
   │
   ├── Scheduling
   ├── Routing
   ├── QEC
   ├── ZQN
   ├── Hardware
   └── Resilience

These systems cooperate through defined contracts.

They MUST NOT become mutually recursive grammar authorities.

---

136. Runtime Ownership

The runtime owns:

- execution context;
- resource handles;
- dispatch;
- runtime state;
- execution lifecycle;
- runtime errors;
- telemetry;
- resource release;
- runtime adaptation.

The grammar MUST NOT define runtime data structures.

---

137. Compiler Ownership

The compiler owns:

- semantic lowering;
- target-independent transformation;
- target specialization;
- artifact generation;
- compatibility checking;
- lowering;
- code generation.

The compiler MUST consume canonical semantic representations rather than reparse source grammar during normal execution.

---

138. IR Ownership

IR owns canonical computational representation.

Quantum semantics MUST use:

quantum::ir

as the canonical quantum semantic boundary.

The grammar MUST NOT become an IR.

Execution artifacts MAY be derived from IR but MUST NOT require grammar rules at runtime.

---

139. Hardware HAL Ownership

Hardware HAL owns:

- hardware discovery;
- capabilities;
- hardware state;
- calibration;
- device interfaces;
- resource availability.

The grammar MUST NOT duplicate hardware discovery or calibration logic.

---

140. Scheduling Ownership

Scheduling owns:

- dependency ordering;
- timing realization;
- resource-aware scheduling;
- event ordering;
- alignment;
- dynamic scheduling.

The grammar MAY express scheduling requirements or hints.

It MUST NOT define scheduling algorithms.

---

141. Routing Ownership

Routing owns physical realization of connectivity.

For quantum systems this includes logical-to-physical realization.

For distributed systems it may include communication placement/path selection.

The grammar MUST NOT hard-code routing algorithms.

---

142. Optimization Ownership

Optimization owns implementation improvements.

The execution model only establishes that optimizations MUST preserve semantics.

---

143. QEC Ownership

QEC owns error detection/correction algorithms.

Execution MAY invoke QEC but MUST NOT implement QEC semantics in the execution grammar.

---

144. ZQN Ownership

ZQN owns quantum fault/noise semantics.

Execution consumes those descriptions when needed.

---

145. Resilience Ownership

Resilience owns decisions about:

- retry;
- recovery;
- adaptation;
- backend switching;
- quarantine;
- mitigation;
- escalation;
- abort.

The grammar MAY declare policy intent but MUST NOT own recovery algorithms.

---

146. Resource Management Ownership

Resource management owns:

- allocation;
- leases;
- reservations;
- release;
- capacity accounting;
- resource lifecycle.

The execution grammar only expresses resource intent.

---

147. Execution Artifact

A production execution artifact SHOULD contain:

semantic identity
language version
semantic version
artifact version
canonical representation
requirements
constraints
effects
capability requirements
provenance
dialect metadata
compatibility metadata

It SHOULD NOT require:

fixed device IDs
fixed physical addresses
fixed machine topology
fixed hardware calibration

unless the artifact is intentionally target-specific.

---

148. Execution Artifact Immutability

Once published, an execution artifact SHOULD be immutable.

A new target realization SHOULD produce a new derived artifact rather than mutating the original portable semantic artifact.

This supports:

- reproducibility;
- provenance;
- caching;
- verification;
- POCO-REAF.

---

149. Caching

Compilers and runtimes MAY cache:

- semantic analysis;
- IR;
- optimized artifacts;
- target-specific artifacts;
- compiled kernels.

Cache keys MUST include all semantic inputs that affect the result.

A cache MUST NOT accidentally reuse an incompatible target artifact.

---

150. Cache Portability

Portable artifacts SHOULD remain reusable across targets.

Target-specific artifacts MAY require:

target identity
target capabilities
target interface version
compiler version
dialect version

in their compatibility key.

---

151. Execution and Recompilation

Recompilation MAY occur when:

- target capabilities change;
- resource availability changes;
- a backend becomes unavailable;
- a dialect changes;
- a security policy changes;
- resilience requires adaptation.

Recompilation MUST preserve the source semantic contract.

---

152. Execution and Dynamic Compilation

Dynamic compilation MAY occur during execution.

It MUST remain constrained by:

- declared permissions;
- compiler policy;
- semantic compatibility;
- security rules;
- runtime capabilities.

Dynamic compilation MUST NOT become an implicit mechanism for bypassing language safety.

---

153. Safe Rust Requirement

The Zamani implementation MUST be implementable using safe Rust.

The implementation MUST target:

Rust 1.97

or preferably the repository's pinned:

Rust 1.97.1

The implementation MUST NOT require "unsafe".

This specification is language-semantic and therefore MUST NOT depend on Rust-specific memory behavior.

Rust implementation details belong to the compiler/runtime implementation.

---

154. No Unsafe Language Construct

Zamani's execution model MUST NOT introduce a grammar construct whose only purpose is to bypass execution safety.

The existing "unsafe.g4" proposal MUST therefore be treated carefully.

If Zamani does not define an unsafe language mode, that grammar file MUST NOT create an unrestricted unsafe execution escape hatch merely because Rust has an "unsafe" keyword.

If a future unsafe/low-level feature is introduced, it MUST have an explicit specification, effect model, capability model, security model, and compatibility policy.

---

155. Foreign Execution

Foreign calls MAY interact with:

- C;
- C++;
- Python;
- system APIs;
- hardware APIs;
- external runtimes.

Foreign calls MUST be explicitly represented and MUST declare relevant effects and compatibility requirements.

They MUST NOT silently bypass the execution model.

---

156. Execution and OpenQASM

OpenQASM interoperability MAY lower imported programs into Zamani's canonical quantum semantics.

The execution model MUST NOT make OpenQASM the internal quantum representation.

The integration is:

OpenQASM
   ↓
Zamani frontend
   ↓
semantic validation
   ↓
quantum::ir
   ↓
execution pipeline

---

157. Execution and HDL Interoperability

Verilog/VHDL/SystemVerilog-style inputs MAY be imported through interoperability mechanisms.

Imported constructs MUST be translated into Zamani's semantic/hardware representation.

External HDL syntax MUST NOT become the definition of Zamani execution semantics.

---

158. Execution and Documentation

Documentation MUST distinguish:

language semantics

from:

implementation behavior

Examples MUST NOT accidentally establish undocumented machine limits.

Documentation MUST NOT contain examples implying:

N is the maximum

when N is merely an example.

---

159. Execution Examples

Examples SHOULD demonstrate scalability.

A good example conceptually shows:

same program
     ↓
small target
     ↓
large target
     ↓
heterogeneous target
     ↓
distributed target

without changing semantic source.

---

160. Forbidden Execution Assumptions

The following are forbidden as core language assumptions:

exact CPU count
exact core count
exact thread count
exact GPU count
exact FPGA count
exact node count
exact QPU count
exact qubit count
exact memory size
exact register count
exact topology
exact device ID
exact hardware address
exact accelerator count
exact clock frequency
exact deployment location

unless explicitly represented as target-specific semantics.

---

161. Hard-Coding Audit

Every execution-related construct MUST be audited against:

1. genuine semantic requirement;
2. target-specific requirement;
3. resource constraint;
4. implementation limitation;
5. accidental hard-coding;
6. test-only limitation;
7. documentation-only limitation.

Only categories 1–3 may normally appear as semantic declarations.

Category 4 belongs to implementation documentation.

Category 5 MUST be removed.

Categories 6–7 MUST NOT be mistaken for language limits.

---

162. Execution Conformance

An implementation conforms to this execution model if it:

- preserves defined semantics;
- distinguishes capabilities from resources;
- distinguishes requirements from preferences;
- supports dynamic resource environments;
- avoids fixed machine limits;
- reports capability/resource failures explicitly;
- preserves provenance;
- respects security;
- integrates with canonical IR;
- does not require grammar interpretation at runtime;
- supports target specialization;
- supports scalable execution;
- does not require "unsafe" Rust.

---

163. Execution Test Matrix

The execution subsystem MUST be tested against:

Minimal execution

smallest valid program

Classical execution

scalar
structured
parallel
numerical
tensor

Quantum execution

logical qubits
parameterized operations
measurement
dynamic circuits
mid-circuit control

Hybrid execution

classical → quantum
quantum → classical
classical control → quantum

Hardware execution

HDL simulation
synthesis
hardware realization

Distributed execution

single node
multiple nodes
elastic resources
node failure

Accelerator execution

CPU
GPU
FPGA
specialized accelerator

Failure execution

resource failure
capability failure
backend failure
network failure
verification failure

Resilience execution

retry
recovery
remapping
rescheduling
backend switching
abort

---

164. Scalability Tests

The test suite MUST verify that execution remains valid for increasingly large:

- program structures;
- data structures;
- qubit collections;
- tensor dimensions;
- task graphs;
- distributed workloads;
- hardware descriptions.

Tests MUST NOT use a small arbitrary maximum as evidence of language scalability.

---

165. Determinism Tests

Tests MUST verify:

same source
+
same semantic inputs
+
same deterministic execution context
=
same semantic result

where deterministic semantics are required.

The test suite MUST also verify that nondeterministic constructs are correctly classified.

---

166. Cross-Domain Tests

Required cross-domain execution tests include:

classical + quantum
classical + HDL
quantum + HDL
quantum + hardware
quantum + distributed
AI + quantum
AI + hardware
classical + quantum + distributed
classical + quantum + HDL + hardware

These tests MUST verify that domain boundaries remain explicit and do not create competing semantic authorities.

---

167. Round-Trip Execution Tests

Where applicable:

source
 ↓
lexer
 ↓
parser
 ↓
AST
 ↓
semantic model
 ↓
IR
 ↓
artifact
 ↓
execution

must preserve the intended semantic contract.

Execution artifacts MUST NOT require the original source grammar to be interpreted again.

---

168. Repository Integration Contract

This file integrates with the repository as follows.

Component| Integration
"grammar/Zamani.g4"| Defines syntax that can express execution concepts
"grammar/specification/semantic-model.md"| Defines meaning of execution-related semantic constructs
"grammar/specification/compilation-model.md"| Defines compilation and lowering boundary
"grammar/specification/scalability-model.md"| Defines unbounded-by-language scalability
"grammar/specification/poco-reaf.md"| Defines long-term portability model
"grammar/specification/compatibility.md"| Defines compatibility/version contracts
"grammar/specification/extensibility.md"| Defines future execution extensions
"grammar/core/capabilities.g4"| Syntax for capability declarations
"grammar/core/requirements.g4"| Syntax for execution requirements
"grammar/core/constraints.g4"| Syntax for constraints
"grammar/core/hints.g4"| Syntax for non-binding implementation hints
"grammar/resources/*"| Resource semantics and declarations
"grammar/execution/*"| Execution-oriented syntax
"grammar/hardware/*"| Hardware target descriptions
"grammar/quantum/*"| Quantum execution intent
"grammar/hdl/*"| Hardware execution semantics
"grammar/distributed/*"| Distributed execution intent
"grammar/effects/*"| Observable execution effects
"src/quantum/ir"| Canonical quantum semantic boundary
"src/quantum/optimization"| Quantum optimization
"src/quantum/scheduling"| Scheduling/timing realization
"src/quantum/hardware"| Hardware capabilities/state
"src/quantum/qec"| Error correction
"src/quantum/zqn"| Fault/noise semantics
"src/quantum/resilience"| Recovery/adaptation decisions
runtime| Actual dispatch and execution
compiler| Lowering and artifact generation

---

169. Explicit Non-Dependencies

The grammar execution model MUST NOT directly depend on:

- a particular runtime implementation;
- a particular scheduler;
- a particular hardware provider;
- a particular QPU;
- a particular GPU;
- a particular CPU;
- a particular operating system;
- a particular cloud;
- a particular compiler backend;
- a particular device topology.

Likewise:

runtime → grammar

MUST NOT be required merely to execute a compiled artifact.

---

170. Integration Graph

The intended architecture is:

                 ┌────────────────────┐
                 │   Zamani Source    │
                 └─────────┬──────────┘
                           │
                           ▼
                 ┌────────────────────┐
                 │ Grammar / Parser   │
                 └─────────┬──────────┘
                           │
                           ▼
                 ┌────────────────────┐
                 │ Semantic Analysis  │
                 └─────────┬──────────┘
                           │
              ┌────────────┴────────────┐
              ▼                         ▼
     Classical Semantic              quantum::ir
       Representation                    │
              │                         │
              └────────────┬────────────┘
                           ▼
                    Canonical IR
                           │
                           ▼
                     Optimization
                           │
                           ▼
              ┌────────────┴────────────┐
              │                         │
           Routing                 Scheduling
              │                         │
              └────────────┬────────────┘
                           ▼
                  Target Lowering
                           │
              ┌────────────┼────────────┐
              ▼            ▼            ▼
           CPU/GPU       FPGA/ASIC      QPU
              │            │            │
              └────────────┼────────────┘
                           ▼
                         Runtime
                           │
             ┌─────────────┼─────────────┐
             ▼             ▼             ▼
          Resources       ZQN/QEC      Resilience
             │             │             │
             └─────────────┼─────────────┘
                           ▼
                       Observation

No arrow from runtime back into grammar is required for normal execution.

---

171. Dependency Graph

The specification dependencies are:

language-principles
        │
        ▼
language-scope
        │
        ▼
language-version
        │
        ├──────────────┐
        ▼              ▼
semantic-model    syntax-model
        │
        ▼
compilation-model
        │
        ▼
execution-model
        │
        ├───────────────┐
        ▼               ▼
scalability-model    poco-reaf
        │               │
        └───────┬───────┘
                ▼
          extensibility
                │
                ▼
          compatibility

Execution semantics therefore build on semantic and compilation contracts rather than defining them independently.

---

172. File Completion Contract

This file is considered complete only when:

- execution semantics are defined;
- execution ownership is explicit;
- grammar ownership is explicit;
- compiler ownership is explicit;
- runtime ownership is explicit;
- quantum execution boundaries are explicit;
- "quantum::ir" remains canonical;
- QEC ownership is explicit;
- ZQN ownership is explicit;
- scheduling ownership is explicit;
- routing ownership is explicit;
- resilience ownership is explicit;
- hardware ownership is explicit;
- distributed execution is defined;
- accelerator execution is defined;
- HDL execution is defined;
- resource/capability semantics are defined;
- failure semantics are defined;
- deterministic execution is defined;
- nondeterministic execution is defined;
- provenance is defined;
- versioning is defined;
- POCO-REAF is integrated;
- scalability is defined without artificial finite limits;
- target specialization is separated from source semantics;
- security boundaries are defined;
- compile-time and runtime execution are separated;
- interoperability is defined;
- testing requirements are defined;
- no circular grammar/IR/runtime dependency is introduced;
- no "unsafe" implementation requirement exists.

---

173. Required Companion-File Integration

The following files MUST be aligned with this document before the grammar implementation is considered complete:

specification/semantic-model.md
specification/compilation-model.md
specification/scalability-model.md
specification/poco-reaf.md
specification/compatibility.md
specification/extensibility.md
core/capabilities.g4
core/requirements.g4
core/constraints.g4
core/hints.g4
effects/*
resources/*
compile/*
execution/*
hardware/*
quantum/*
distributed/*
hdl/*
dialects/*

Those files MUST consume this execution model rather than independently redefining execution semantics.

---

174. Implementation Rule

When implementing an individual execution grammar file, its design MUST already be derivable from the contracts in this document.

For every execution grammar file, the developer MUST be able to identify:

Purpose
Owns
Does Not Own
Inputs
Outputs
Dependencies
Upstream Contracts
Downstream Consumers
Public Grammar Contract
AST Contract
Semantic Contract
IR Integration
Compiler Integration
Runtime Integration
Tooling Integration
Cross-Domain Integration
Tests
Negative Tests
Boundary Tests
Compatibility
Scalability
Hard-Coding Audit
Completion Criteria

No execution grammar file should require later redesign merely because another domain file is implemented.

---

175. Final Execution Principle

The Zamani execution model is therefore:

Program
   ↓
Meaning
   ↓
Requirements + Constraints + Capabilities + Effects
   ↓
Portable Semantic Artifact
   ↓
Target Realization
   ↓
Resource-Aware Execution
   ↓
Observation

The machine is an execution environment.

It is not the definition of the program.

---

176. POCO-REAF Guarantee Boundary

Zamani's strongest portability promise is:

«The semantic program is written once.»

Compilation and execution infrastructure MAY create different physical realizations for different environments.

Therefore:

ONE SOURCE SEMANTICS
        ↓
MANY VALID COMPILATIONS
        ↓
MANY VALID TARGET REALIZATIONS
        ↓
MANY EXECUTION ENVIRONMENTS

The developer should not need to rewrite the program merely because the machine changes.

---

177. Ultimate Architectural Rule

Zamani MUST follow:

«One program → one semantic meaning → many valid execution realizations.»

Therefore:

Atom
  ↓
Embedded
  ↓
CPU
  ↓
Multicore
  ↓
GPU
  ↓
FPGA
  ↓
ASIC
  ↓
Quantum
  ↓
Hybrid
  ↓
Cluster
  ↓
Supercomputer
  ↓
Distributed
  ↓
Cloud
  ↓
Future architectures

are execution realizations, not separate languages.

The grammar MUST describe the computation and its semantic intent.

The compiler MUST determine how that computation can be realized.

The runtime MUST determine how the selected realization executes on available resources.

The hardware layer MUST describe what the machine can actually provide.

The resource system MUST describe what is available.

The scheduling and routing systems MUST determine physical execution.

QEC MUST protect quantum computation.

ZQN MUST describe quantum faults and noise.

Resilience MUST determine how the system adapts to execution failure and changing conditions.

No layer may silently assume that the machine currently available is the permanent definition of the language.

---

178. Final Statement

Zamani execution is not "run this program on this machine."

It is:

Run this semantic computation
on any execution environment
that can satisfy its contract,
using whatever valid realization
is required by the available resources,
while preserving its defined meaning.

That is the execution foundation required for:

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)

and for Zamani's architectural objective:

From Atom to Everywhere.