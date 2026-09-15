Zamani Extended Language Grammar and Language Design Reference

Path: "grammar/Zamani-Grammar.md"
Language: Zamani
Repository: "Benwellonedge28/Zamani"
Primary branch: "main"
Edition: Universal Computational Language
Rust implementation baseline: Rust 2021, Rust 1.97.1
Implementation safety policy: Safe Rust only; "unsafe" is prohibited
Primary portability objective: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)
Scalability objective: From the smallest meaningful computation to arbitrarily large computations subject only to program semantics, representational limits, declared constraints, implementation resource availability, target capabilities, and explicitly selected policies.

---

1. Status and Authority

This file is the extended Zamani language-design and grammar reference.

It preserves and integrates the broad language concepts historically associated with:

- Zamani;
- NIMBUS;
- Universal Trinity;
- Sankofa;
- temporal and multi-timeline computation;
- classical computation;
- quantum computation;
- hybrid quantum-classical computation;
- hardware description;
- hardware/software co-design;
- AI and machine learning;
- distributed computing;
- networking;
- scientific computing;
- data and tensor computation;
- security and cryptography;
- metaprogramming;
- interoperability;
- nano-oriented computation;
- future computational paradigms.

However:

«The presence of a construct in this document does not by itself make that construct legal Zamani syntax.»

The authoritative promotion path is:

language design
    ↓
normative specification
    ↓
feature contract
    ↓
lexical contract
    ↓
canonical Zamani.g4
    ↓
Rust lexer/parser
    ↓
domain-neutral AST
    ↓
semantic analysis
    ↓
canonical semantic model
    ↓
canonical IR
    ↓
compiler integration
    ↓
runtime/backend integration
    ↓
conformance tests
    ↓
stable language feature

The authority hierarchy is:

1. User-visible normative language specification
2. grammar/specification/
3. grammar/spec/
4. grammar/specification/features/
5. grammar/Zamani.g4
6. Rust lexer/parser/AST/semantic implementation
7. grammar/grammar.md
8. grammar/Zamani-Grammar.md
9. historical/proposal material

The ordering above requires an important interpretation:

- the normative specification defines intended language semantics;
- "Zamani.g4" defines the canonical ANTLR syntax representation;
- Rust frontend implementation establishes executable implementation conformance;
- "grammar.md" records implementation reality;
- this file records the complete extended design space and feature contracts;
- historical or proposed material must never silently become accepted syntax.

Where two artifacts disagree, the discrepancy is a conformance defect and must be resolved explicitly.

---

2. Fundamental Language Identity

Zamani is one programming language.

It is not a collection of unrelated:

- classical languages;
- quantum languages;
- HDL languages;
- AI languages;
- distributed languages;
- networking languages;
- accelerator languages;
- scientific languages;
- embedded languages.

All domains share the same foundational language model.

The shared foundation includes:

- source units;
- Unicode and lexical rules;
- identifiers;
- literals;
- names and paths;
- modules;
- imports and exports;
- declarations;
- types;
- expressions;
- statements;
- functions;
- effects;
- ownership;
- resource semantics;
- capabilities;
- diagnostics;
- source locations;
- versioning;
- compatibility;
- semantic validation;
- interoperability.

Domains extend the common language rather than replacing it.

---

3. POCO-REAF

3.1 Definition

POCO-REAF means:

«A programmer should be able to express the stable semantics of a computation once without rewriting the algorithm merely because the computation is later realized on a different size, architecture, device class, topology, deployment environment, or generation of hardware.»

The intended lifecycle is:

Program Once
     ↓
Compile Once
     ↓
Discover available capabilities/resources
     ↓
Select legal realization
     ↓
Optimize
     ↓
Lower
     ↓
Execute

POCO-REAF does not mean that every program is executable on every machine.

For example:

program requires capability X
target does not provide capability X

must produce a clear incompatibility result unless an explicitly permitted semantic-preserving alternative exists.

The compiler must never silently change program meaning merely to fit a target.

---

4. Scale From Atom to Everywhere

Zamani must support the same semantic language model across scales such as:

single value
    ↓
single operation
    ↓
function
    ↓
process
    ↓
thread/task
    ↓
device
    ↓
accelerator
    ↓
node
    ↓
cluster
    ↓
distributed system
    ↓
heterogeneous system
    ↓
planetary/cloud-scale system
    ↓
future computational environments

The language does not define an artificial maximum at any level.

The actual feasible scale is determined by:

- program semantics;
- type representation;
- compiler resources;
- runtime resources;
- target capabilities;
- explicitly declared constraints;
- resource availability;
- security policy;
- deployment policy;
- numerical representation;
- physical feasibility.

---

5. No Artificial Universal Hardware Limits

The language must not define universal constants such as:

MAX_QUBITS
MAX_CPUS
MAX_CORES
MAX_THREADS
MAX_GPUS
MAX_FPGAS
MAX_QPUS
MAX_NODES
MAX_MEMORY
MAX_STORAGE
MAX_REGISTERS
MAX_REGISTER_WIDTH
MAX_VECTOR_WIDTH
MAX_TENSOR_RANK
MAX_TENSOR_DIMENSION
MAX_ACCELERATORS
MAX_DEVICES
MAX_TIMELINES
MAX_PROCESSES
MAX_AGENTS
MAX_NETWORK_LINKS
MAX_GATE_COUNT

The same prohibition applies to hidden equivalents such as:

only 1024 qubits
only 64 CPUs
only 32 GPUs
only 8 FPGA devices
only 4096 nodes
only 64 tensor dimensions
only 32-bit registers

unless such a number is explicitly part of a particular target profile or particular type/program value, rather than a universal Zamani limitation.

---

6. Program Constants Are Not Language Limits

This is valid:

let n = 1024;

This is also valid:

let matrix = Matrix<1024, 1024>;

if those values are program semantics.

What is prohibited is a compiler/language architecture equivalent to:

Zamani only supports matrices <= 1024 × 1024.

Similarly:

allocate qubits[n]

is valid when "n" is program data.

A universal:

MAX_QUBITS = 1024

is not.

---

7. Semantic Intent vs Target Realization

Zamani separates:

semantic requirement
capability requirement
constraint
preference
hint
implementation decision
physical realization

These concepts must never be conflated.

7.1 Requirement

A program requires a property.

Example:

requires qubits >= n

7.2 Capability

A target must expose a capability.

Example:

requires capability("quantum.mid_circuit_measurement")

7.3 Constraint

A property must satisfy a bound.

Example:

requires latency <= budget

7.4 Preference

A realization is preferred but not mandatory.

Example:

prefer accelerator("quantum")

7.5 Hint

Information is supplied to optimization.

A hint may improve implementation but must not alter semantic correctness.

7.6 Implementation Decision

A compiler or deployment layer may eventually decide:

logical resource
    ↓
physical resource

For example:

logical qubit
    ↓
physical qubit 17

Physical placement is not the portable semantic identity of the logical qubit.

---

8. Canonical Compilation Architecture

The language architecture is:

Zamani Source
     ↓
Source map
     ↓
Lexer
     ↓
Token stream
     ↓
Parser
     ↓
Domain-neutral AST
     ↓
Structural validation
     ↓
Name/module resolution
     ↓
Type analysis
     ↓
Effect analysis
     ↓
Ownership/resource analysis
     ↓
Capability analysis
     ↓
Portability analysis
     ↓
Semantic validation
     ↓
Canonical semantic model
     ↓
Canonical IR
     ├── Classical semantics
     ├── quantum::ir
     └── HDL/Hardware semantics
     ↓
Optimization
     ↓
Lowering
     ├── routing
     ├── scheduling
     ├── resilience
     ├── QEC
     └── other domain lowering
     ↓
ZQN
     ↓
HAL
     ↓
Target realization
     ├── CPU
     ├── GPU
     ├── FPGA
     ├── QPU
     ├── embedded device
     ├── distributed system
     └── future target

The grammar exists at the beginning of this pipeline.

It must not absorb responsibilities belonging to later layers.

---

9. Grammar Boundary

The grammar describes:

- source structure;
- source-level relationships;
- portable program intent;
- declarations;
- expressions;
- statements;
- types;
- domain constructs;
- resource intent;
- capability requirements;
- compile/execution intent;
- interoperability declarations.

The grammar does not implement:

- optimization;
- scheduling;
- routing;
- calibration;
- physical qubit allocation;
- QEC algorithms;
- ZQN execution;
- HAL behavior;
- device discovery;
- runtime scheduling;
- physical placement;
- machine learning training;
- hardware synthesis;
- network routing;
- distributed consensus.

Those systems consume the semantic information produced after parsing.

---

10. Canonical Quantum Boundary

"quantum::ir" is the canonical quantum semantic boundary.

The grammar must never create a competing quantum semantic IR.

The intended path is:

Zamani quantum syntax
       ↓
generic AST operation
       ↓
semantic quantum operation
       ↓
quantum::ir
       ↓
optimization
       ↓
decomposition
       ↓
routing
       ↓
scheduling
       ↓
QEC/resilience
       ↓
ZQN
       ↓
HAL
       ↓
QPU realization

The grammar is therefore allowed to describe quantum syntax but must not become the owner of quantum execution semantics.

---

11. Generic Quantum Operations

Zamani must not make the language's future depend on an exhaustive enumeration such as:

H
X
Y
Z
T
S
CNOT
SWAP
...

A finite gate list cannot be the universal semantic model.

The preferred abstraction is:

operation name
operation namespace
parameters
operands
results
modifiers
attributes
effects
capabilities
source location

Conceptually:

apply operation to targets

where the operation may be:

H
X
vendor.operation
custom.operation
logical.operation
future.operation

The semantic layer resolves whether the operation exists, is valid, and can be realized.

This allows future quantum operations without rewriting the grammar.

---

12. Quantum Domain

The quantum language must support, where implemented:

- qubits;
- logical qubits;
- quantum registers;
- dynamically sized quantum collections;
- quantum states;
- amplitudes;
- observables;
- operations;
- parameterized operations;
- controlled operations;
- adjoint operations;
- measurement;
- reset;
- barriers;
- dynamic control;
- classical feed-forward;
- mid-circuit measurement;
- quantum channels;
- noise descriptions;
- error-correction intent;
- logical operations;
- fault-tolerance requirements;
- resource requirements;
- pulse intent;
- circuits;
- kernels;
- hybrid computation;
- quantum interoperability.

Quantum grammar must not impose a maximum number of qubits.

---

13. Quantum Types

Quantum types are semantic abstractions.

Examples include:

Qubit
Qubit<n>
QRegister<n>
QState<T>
QuantumRegister<T>
LogicalQubit
LogicalRegister<n>

The value of "n" is program semantics.

It is not a universal hardware maximum.

The semantic layer must distinguish:

logical qubit count
physical qubit availability
encoded qubit count
ancilla requirements
error-correction overhead
device capacity

Those are different concepts.

---

14. Quantum Measurements

Measurement must support:

- single-qubit measurement;
- multi-qubit measurement;
- observable measurement;
- basis specification;
- measurement results;
- classical destinations;
- repeated sampling;
- mid-circuit measurement;
- conditional execution.

Measurement semantics belong to the quantum semantic model.

The grammar must not decide how a target QPU physically performs measurement.

---

15. Quantum Control Flow

Quantum-classical control may include:

measurement
    ↓
classical result
    ↓
conditional
    ↓
quantum operation

This must integrate with ordinary Zamani control flow.

Quantum and classical control are therefore not separate programming languages.

---

16. Quantum Error Correction

Grammar may express intent such as:

requires error_correction(...)
requires fault_tolerance(...)
requires reliability(...)

but grammar does not implement QEC.

The ownership boundary is:

Grammar
    → expresses intent

Semantic analysis
    → validates intent

quantum::ir
    → represents canonical quantum semantics

QEC
    → detects/corrects errors and performs encoding/decoding responsibilities

ZQN
    → represents fault/noise semantics

HAL
    → exposes target capability/state

Routing
    → realizes logical connectivity

Scheduling
    → realizes temporal/resource order

No QEC algorithm may be embedded in the grammar.

---

17. Classical Computing

The classical domain includes:

- scalar computation;
- integer computation;
- floating-point computation;
- arbitrary supported numeric representations;
- vectors;
- matrices;
- tensors;
- linear algebra;
- numerical analysis;
- symbolic mathematics;
- calculus;
- statistics;
- optimization;
- signal processing;
- scientific computing;
- control systems;
- high-performance computing;
- parallel computation.

The grammar should express language-level semantics.

Large libraries of mathematical functions should generally be represented through:

typed operation
+
intrinsic
+
standard library
+
semantic capability

rather than turning every mathematical function into a keyword.

---

18. Mathematical Semantics

Zamani may represent:

- arithmetic;
- algebra;
- vectors;
- matrices;
- tensors;
- symbolic expressions;
- differentiation;
- integration;
- transforms;
- optimization;
- probability;
- statistics;
- numerical methods;
- signal processing.

The language must distinguish:

syntax
semantic operation
library operation
implementation intrinsic

For example, FFT implementation belongs to a library/intrinsic/semantic capability unless FFT itself acquires language-level semantics requiring dedicated syntax.

---

19. Tensor and Shape Semantics

Tensor syntax must support:

- symbolic shapes;
- runtime shapes where semantically valid;
- compile-time shapes;
- parameterized shapes;
- shape constraints;
- broadcasting;
- indexing;
- slicing;
- transformations;
- reductions;
- contraction;
- layout intent.

No universal tensor-rank or tensor-dimension maximum may be embedded in the language.

---

20. HDL

HDL is a first-class Zamani domain.

It must support, as implemented:

- modules;
- ports;
- signals;
- nets;
- registers;
- combinational logic;
- sequential logic;
- clocks;
- resets;
- timing;
- assertions;
- interfaces;
- protocols;
- state machines;
- pipelines;
- memories;
- parameters;
- generate constructs;
- simulation;
- synthesis;
- verification;
- physical intent;
- hardware/software co-design.

Existing HDL grammar files, including:

grammar/hdl/memories.g4

must be retained where they remain valid and integrated rather than unnecessarily renamed or duplicated.

---

21. HDL Scalability

HDL must not encode an artificial universal hardware size.

For example:

width = 32

may be a particular signal's declared width.

But:

Zamani hardware signals can never exceed 32 bits

is not acceptable as a universal language restriction.

Widths, array dimensions, pipeline stages, memories, ports, channels, and hardware resources must be parameterizable where their semantics permit.

---

22. Hardware/Software Co-Design

Zamani may describe both:

software algorithm

and:

hardware realization intent

in a common semantic framework.

The compiler may later derive:

CPU execution
GPU kernel
FPGA accelerator
ASIC realization
QPU realization
distributed realization

without forcing the source algorithm to be rewritten solely because the implementation target changed.

---

23. Hardware Intent

The hardware domain may express:

- target capabilities;
- compute capabilities;
- memory capabilities;
- communication capabilities;
- acceleration capabilities;
- quantum-device capabilities;
- timing properties;
- power constraints;
- thermal constraints;
- reliability requirements;
- calibration requirements;
- deployment requirements;
- topology constraints;
- negotiation policies.

It must not confuse capability declaration with physical placement.

---

24. Resource Model

Resources are represented abstractly.

Examples:

compute
memory
storage
communication
quantum
accelerator
energy
time
bandwidth
latency
precision
reliability

Resource expressions must be composable and parameterized.

No fixed number of resource instances is built into the grammar.

---

25. Resource Requirements

A program may express:

requires resource(...)

The compiler/runtime may determine how to satisfy that requirement.

A requirement must remain separate from:

physical resource identifier

and:

physical placement

---

26. Capability Model

Capabilities represent what a target can do.

Examples:

quantum.measure
quantum.mid_circuit_measurement
quantum.dynamic_control
tensor.acceleration
fpga.synthesis
distributed.communication
secure.execution

Capabilities must be namespaced and versionable.

Capability names must not become an exhaustive list of all future hardware.

Unknown capabilities may be represented semantically and rejected only when the program requires them and no compatible implementation exists.

---

27. Constraints

Constraints express conditions that must hold.

Examples:

latency
bandwidth
precision
reliability
energy
memory
communication
topology
security
fault tolerance

Constraints must remain distinct from requirements and preferences.

---

28. Preferences

Preferences guide implementation selection.

A preference:

- must not change semantic meaning;
- may be ignored if infeasible;
- should be visible to optimization/placement;
- should be deterministic when determinism is required.

---

29. Hints

Hints are optional implementation guidance.

A hint:

- does not establish semantic correctness;
- does not guarantee physical realization;
- must not override safety rules;
- must not bypass semantic validation.

---

30. Memory Model

Zamani's memory model may support:

- ownership;
- borrowing;
- references;
- allocation;
- deallocation;
- regions;
- shared memory;
- distributed memory;
- accelerator memory;
- persistent memory;
- address spaces;
- memory capabilities;
- quantum memory abstractions.

Memory syntax must remain independent of a particular machine's RAM or VRAM size.

---

31. Ownership and Safety

Ownership and borrowing must be represented semantically.

The compiler must be able to reason about:

- lifetime;
- aliasing;
- mutation;
- ownership transfer;
- resource ownership;
- concurrency safety;
- distributed ownership where supported.

Unsafe physical memory operations must not become implicit merely because a target provides them.

---

32. Concurrency

Zamani may support:

- asynchronous execution;
- tasks;
- futures;
- spawn;
- await;
- actors;
- channels;
- synchronization;
- parallel loops;
- data parallelism;
- task parallelism;
- pipelines;
- reductions;
- deterministic parallelism;
- distributed concurrency.

The language must not encode a fixed thread count.

This is valid:

parallel

A target may realize it using:

1 thread
8 threads
1000 threads
GPU execution
distributed execution
future hardware

subject to semantics and capabilities.

---

33. Determinism

Where deterministic semantics are promised, implementation choices must not silently alter observable program behavior.

The language must distinguish:

deterministic
nondeterministic
implementation-defined
unspecified
target-dependent

These categories must be explicitly documented.

---

34. Distributed Computing

Distributed semantics may include:

- nodes;
- processes;
- actors;
- services;
- messages;
- channels;
- replication;
- partitioning;
- consistency;
- transactions;
- collectives;
- fault tolerance;
- placement;
- deployment;
- distributed resources.

There is no universal maximum number of nodes.

---

35. Networking

Networking syntax may represent:

- endpoints;
- abstract addresses;
- protocols;
- channels;
- sockets;
- requests;
- responses;
- streams;
- service discovery;
- communication requirements;
- network capabilities.

The grammar must not force physical addresses into portable program semantics unless the programmer explicitly requests target-specific behavior.

---

36. Data

The data domain may support:

- collections;
- records;
- tables;
- schemas;
- streams;
- datasets;
- tensors;
- transformations;
- queries;
- pipelines;
- serialization;
- persistence;
- provenance.

Data semantics must remain independent of a particular storage vendor.

---

37. AI and Machine Learning

Zamani may represent:

- models;
- tensors;
- datasets;
- training;
- inference;
- optimization;
- automatic differentiation;
- probabilistic computation;
- neural computation;
- symbolic computation;
- agents;
- pipelines;
- distributed training;
- deployment;
- model capabilities.

The grammar must not make:

PyTorch
TensorFlow
JAX
CUDA
ROCm
vendor-specific framework

part of the core language semantics.

Framework integration belongs under interoperability and compiler/backend layers.

---

38. AI Agent Semantics

Agent syntax may describe:

- agent identity;
- capabilities;
- goals;
- observations;
- actions;
- policies;
- memory;
- planning;
- learning;
- inference;
- communication.

Runtime cognition must remain outside the grammar.

---

39. Sankofa

Sankofa concepts are retained as a language-design domain.

They may include:

- remember;
- recall;
- learn;
- history;
- provenance;
- temporal knowledge;
- wisdom;
- inference;
- consensus;
- inter-memory;
- ancestral/reference information.

The grammar describes the intent and syntax.

It does not implement memory storage.

For example:

remember x

must lower into semantic memory operations.

The parser must never itself maintain persistent memory.

---

40. Temporal Computation

Temporal semantics may represent:

- time;
- intervals;
- temporal values;
- temporal constraints;
- event ordering;
- temporal state;
- history;
- provenance;
- temporal queries.

No fixed timestamp width or finite temporal universe should be imposed by the grammar.

---

41. Multi-Timeline System

MTS concepts are retained as an optional advanced semantic domain.

They may include:

- timelines;
- slices;
- fork;
- merge;
- observe;
- rewind;
- speculative computation;
- counterfactual computation;
- fork/merge semantics;
- temporal observation.

There is no fixed number of timelines.

The runtime is responsible for actual timeline management.

The grammar only describes source intent.

---

42. Nano Computing

Nano-oriented concepts may include:

- atoms;
- molecules;
- materials;
- nano-agents;
- interactions;
- capabilities;
- protocols;
- assembly;
- deployment.

The grammar must not embed an immutable physical chemistry implementation.

For example, element names and orbital models belong to semantic libraries/domain models rather than forcing the parser to contain every future scientific entity.

---

43. Dependent Types

The language may support dependent type concepts such as:

Pi
Sigma
identity

but the syntax must be mapped into the common type system.

Dependent types must integrate with:

- type checking;
- generic parameters;
- const expressions;
- shape expressions;
- resource expressions;
- capability constraints.

They must not create a second type system.

---

44. Linear and Affine Types

Linear and affine concepts may be used for:

- resources;
- capabilities;
- quantum values;
- ownership;
- unique handles;
- communication endpoints.

The semantic analyzer, not the grammar, determines whether the constraints are satisfied.

---

45. Effect System

Effects may represent:

- I/O;
- allocation;
- mutation;
- concurrency;
- quantum execution;
- networking;
- randomness;
- persistence;
- security;
- external calls;
- hardware interaction.

Effects must be compositional.

Effect syntax must map to the semantic effect system.

---

46. Contracts

The language may support:

requires
ensures
invariant

Contracts are semantic constraints.

They must integrate with:

- type checking;
- verification;
- optimization safety;
- diagnostics;
- runtime checks where explicitly requested.

A contract must never be treated as a mere comment.

---

47. Compile-Time Computation

Compile-time computation may include:

- constant evaluation;
- type-level computation;
- shape computation;
- code generation;
- metaprogramming;
- reflection.

Compile-time computation must have explicit evaluation boundaries.

It must not silently become an unrestricted escape hatch around language safety.

---

48. Macros

Macros may manipulate:

- tokens;
- syntax trees;
- declarations;
- expressions;
- generated code.

Macros must be hygienic where hygiene is promised.

Macro expansion must still undergo:

parsing
↓
AST validation
↓
semantic analysis
↓
type/effect/resource checking

Macros must not bypass the semantic model.

---

49. Metaprogramming

Metaprogramming may support:

- reflection;
- introspection;
- quotation;
- unquotation;
- code generation;
- compile-time evaluation;
- type-level programming;
- schema generation.

It must not create a hidden second compiler.

---

50. Interoperability

Zamani may interoperate with:

- C;
- C++;
- Rust;
- Python;
- WebAssembly;
- OpenQASM;
- QIR;
- HDL formats;
- serialization formats;
- foreign ABIs.

Interoperability formats are not the canonical Zamani semantic model.

For example:

OpenQASM

is an input/output/interoperability format.

It is not the owner of Zamani's quantum semantics.

Likewise:

QIR
LLVM
MLIR

may be downstream/interoperability representations but must not replace "quantum::ir".

---

51. Dialects

A dialect is an explicit extension of Zamani.

Every dialect must define:

name
version
owner
purpose
syntax extensions
semantic extensions
AST mapping
semantic mapping
IR mapping
capabilities
feature gates
compatibility
diagnostics
tests

A dialect must not silently redefine:

- identifier syntax;
- type semantics;
- ownership;
- quantum semantics;
- resource semantics;
- source locations;
- diagnostics.

Dialects must remain interoperable with the canonical language.

---

52. Security

Security constructs may represent:

- identity;
- authentication;
- authorization;
- capabilities;
- policies;
- secrets;
- key management;
- signatures;
- hashes;
- cryptography;
- secure computation;
- zero-knowledge computation;
- provenance;
- trust.

The grammar should represent security semantics, not turn every cryptographic algorithm into a keyword.

---

53. Capability Security

Security-sensitive operations should be capability-controlled where appropriate.

A capability must identify an allowed semantic operation.

Possession of a capability is not automatically equivalent to possession of physical machine authority.

The runtime must enforce actual security boundaries.

---

54. Embedded Computing

Embedded programs must be able to describe:

- resources;
- timing;
- memory;
- peripherals;
- communication;
- power;
- reliability;
- hardware interfaces;
- deployment intent.

The source program should remain portable whenever the semantics permit.

Target-specific peripheral addresses belong in target/deployment configuration rather than being universal language assumptions.

---

55. Scientific Computing

Scientific computation may include:

- numerical methods;
- symbolic computation;
- differential equations;
- linear algebra;
- tensor computation;
- statistics;
- optimization;
- simulation;
- signal processing;
- physical models.

Scientific libraries must remain extensible without requiring grammar changes for every new algorithm.

---

56. Resource Negotiation

A program may express a resource requirement without determining its physical realization.

Conceptually:

require capability
require resource
prefer resource
constrain resource
hint resource

A compiler/runtime may negotiate:

available capabilities
available resources
cost
latency
energy
reliability
security
topology

The result must preserve program semantics.

---

57. Compilation and Deployment

Compilation syntax may express:

- target intent;
- optimization profiles;
- specialization;
- reproducibility;
- cross-compilation;
- artifact generation;
- deployment;
- provenance;
- deterministic builds;
- caching.

The program should not have to name a physical CPU/GPU/QPU unless the programmer deliberately requests a target-specific realization.

---

58. Execution

Execution intent may include:

- entry points;
- runtime environments;
- scheduling policies;
- placement policies;
- resilience;
- recovery;
- checkpointing;
- tracing;
- profiling;
- observability;
- lifecycle.

Execution constructs describe policy.

They do not implement the runtime.

---

59. Resilience

Resilience semantics may describe:

retry
recover
checkpoint
degrade
failover
escalate
reject

The existing resilience subsystem remains responsible for actual orchestration.

The grammar must not duplicate the resilience state machine implementation.

---

60. Quantum Resilience Integration

Quantum resilience must preserve the established ownership model:

Quantum grammar
    ↓
quantum::ir
    ↓
optimization
    ↓
routing
    ↓
scheduling
    ↓
QEC
    ↓
ZQN
    ↓
HAL

The grammar may express:

requires fault tolerance
requires error correction
requires reliability
requires noise tolerance

but does not execute these mechanisms.

---

61. HAL Boundary

HAL owns:

- device capabilities;
- device state;
- available resources;
- target interfaces;
- target-specific operations;
- calibration state;
- target execution.

The grammar does not own physical device identity.

A portable program should not require:

QPU_17
GPU_3
CPU_SOCKET_2
FPGA_4

unless the program is explicitly target-specific.

---

62. Routing Boundary

Routing owns physical realization.

For quantum computation this includes:

logical connectivity
    ↓
physical connectivity
    ↓
mapping
    ↓
movement/decomposition

The grammar must not encode physical qubit topology as the universal program model.

---

63. Scheduling Boundary

Scheduling owns:

- operation ordering;
- timing;
- resource conflicts;
- dependency graphs;
- ASAP/ALAP decisions;
- resource-aware scheduling;
- dynamic scheduling;
- distributed scheduling.

Grammar expresses constraints/preferences where needed.

Scheduling decides realization.

---

64. Optimization Boundary

Optimization may transform a program while preserving semantics.

It owns:

- simplification;
- fusion;
- decomposition;
- common-subexpression elimination;
- vectorization;
- tensor optimization;
- quantum optimization;
- hardware-aware optimization.

Optimization must not redefine source semantics.

---

65. Source Spans

Every grammar construct that can produce an AST node must have a source-span strategy.

Source spans must support:

- file identity;
- byte/character range;
- line/column information where required;
- diagnostics;
- macro-origin tracking where applicable;
- generated-source mapping where applicable.

Source spans must survive lowering where diagnostics require them.

---

66. Diagnostics

Diagnostics are part of the language contract.

Every production feature must define:

diagnostic category
error condition
source span
message
expected information
recovery behavior
related locations
suggested correction where appropriate

Diagnostics must be deterministic.

The grammar must not rely on arbitrary parser recovery to define language semantics.

---

67. Error Recovery

Parser recovery must:

- preserve valid later syntax where possible;
- avoid cascading diagnostics where possible;
- never silently reinterpret invalid source as a different valid program;
- preserve source locations;
- terminate deterministically.

Error recovery is not semantic acceptance.

---

68. Lexical Rules

The canonical lexical contract lives under:

grammar/lexer/
grammar/spec/lexical.md

The implementation lives in:

src/lexer.rs

The grammar and lexer must agree on:

- identifiers;
- keywords;
- literals;
- comments;
- Unicode;
- operators;
- delimiters;
- interpolation;
- numeric syntax;
- quantum literals;
- source locations.

---

69. Token Identity

Equivalent lexical concepts must not be duplicated without justification.

Potentially overlapping tokens such as:

Ampersand / BitAnd
Pipe / BitOr
Question / QuestionMark

must be audited.

A lexical spelling should have one canonical token identity unless contextual distinction is genuinely required.

---

70. Operators

Operator definitions must specify:

- spelling;
- token;
- precedence;
- associativity;
- operand categories;
- AST mapping;
- semantic meaning;
- overload rules;
- diagnostics.

The same operator must not accidentally acquire incompatible meanings in unrelated domains.

---

71. Literals

Literal support must cover:

- integers;
- floating-point values;
- strings;
- characters;
- booleans;
- bytes;
- raw strings;
- interpolated strings;
- structured literals;
- quantum literals;
- domain-specific literals where justified.

Literal representation must not introduce artificial hardware limits.

---

72. Quantum Literals

Quantum notation may support Dirac-style state notation.

Examples:

|0⟩
|1⟩
|+⟩
|-⟩
|ψ⟩

The parser recognizes structure.

The semantic layer determines whether the state is valid.

The grammar must not enumerate every possible future quantum state.

---

73. Names and Namespaces

Names must support:

- identifiers;
- qualified names;
- namespaces;
- modules;
- packages;
- imports;
- aliases;
- generic names;
- dialect-qualified names;
- capability-qualified names.

Names must not depend on target hardware identifiers.

---

74. Modules

Modules must support:

- declarations;
- imports;
- exports;
- namespaces;
- aliases;
- visibility;
- dependencies;
- versions;
- package metadata.

There is no fixed maximum module nesting depth or dependency count.

---

75. Functions

Functions may support:

- parameters;
- generic parameters;
- return types;
- effects;
- contracts;
- asynchronous execution;
- generators;
- closures;
- lambdas;
- calling conventions.

Calling conventions are semantic/ABI metadata and must not leak arbitrary backend assumptions into core syntax.

---

76. Types

The type system may contain:

primitive
named
generic
tuple
array
slice
function
reference
pointer
optional
result
never
dependent
linear
affine
effectful
resource
capability
quantum
tensor
hardware

All must map into one coherent semantic type system.

---

77. Genericity

Genericity must be preferred over domain-specific duplication.

Instead of:

QuantumMatrix
ClassicalMatrix
GPUMatrix
FPgaMatrix

where the semantics do not require separate types, use common generic abstractions with capabilities and constraints.

Domain-specific types remain valid where their semantics genuinely differ.

---

78. Expressions

Expressions must support, as applicable:

- literals;
- names;
- calls;
- indexing;
- member access;
- unary operators;
- binary operators;
- assignment;
- ranges;
- tuples;
- arrays;
- maps;
- conditionals;
- matches;
- lambdas;
- closures;
- blocks;
- async expressions;
- effect expressions;
- quantum expressions;
- metaprogramming expressions.

Expression precedence must be centralized.

---

79. Statements

Statements may include:

- declarations;
- expressions;
- conditionals;
- loops;
- matches;
- returns;
- exceptions;
- assertions;
- blocks;
- concurrency;
- resource operations;
- capability operations;
- quantum operations;
- hybrid operations;
- HDL operations;
- distributed operations;
- AI/data/network/security operations;
- compile/execution directives;
- memory operations;
- temporal operations;
- Sankofa operations.

Every statement must have an explicit semantic owner.

---

80. No Domain-Specific Parser Islands

Quantum, HDL, AI, networking, and other domains must not become independent parser languages.

The correct model is:

common Zamani syntax
        +
domain extension
        ↓
common AST
        ↓
domain semantic model

not:

Zamani
 ├── classical parser
 ├── quantum parser
 ├── HDL parser
 └── AI parser

unless a separate interoperability format explicitly requires a separate parser.

---

81. AST Contract

Every grammar production that contributes semantic structure must define:

grammar rule
    ↓
AST node
    ↓
semantic representation
    ↓
IR representation

The AST must remain domain-neutral where the syntax is structurally generic.

For operations, the preferred conceptual model is:

Operation {
    name
    namespace
    operands
    parameters
    results
    attributes
    modifiers
    effects
    capabilities
    source
}

Do not create a universal AST enum containing every possible future hardware or quantum operation.

---

82. AST Independence

The AST must not depend on:

- LLVM;
- QIR;
- MLIR;
- vendor SDKs;
- vendor hardware;
- physical qubit maps;
- calibration data;
- routing algorithms;
- scheduling algorithms;
- QEC implementation;
- target topology.

Those belong downstream.

---

83. Canonical IR

Canonical IR is where validated semantics become executable compiler representation.

The grammar does not define every IR instruction.

The semantic layer must establish the mapping.

The architecture permits:

Classical IR
quantum::ir
HDL/Hardware IR

without requiring three unrelated source languages.

---

84. Quantum IR

"quantum::ir" remains authoritative for quantum semantics.

No:

grammar quantum IR
frontend quantum IR
secondary quantum IR
vendor quantum IR

may replace it.

Interoperability representations may be translated to and from "quantum::ir".

---

85. HDL/Hardware IR

HDL syntax must lower into the appropriate canonical hardware semantic representation.

The source grammar should not become an implementation of synthesis.

---

86. Inter-Domain Integration

A feature that crosses domains must define all participating contracts.

For example:

AI
 ↓
Tensor
 ↓
Classical computation
 ↓
Quantum kernel
 ↓
Measurement
 ↓
Classical control

must remain one semantic program.

The integration must define:

- types;
- ownership;
- effects;
- resource requirements;
- capability requirements;
- data representation;
- source spans;
- lowering;
- runtime behavior;
- diagnostics;
- tests.

---

87. Security and Safety Invariants

Production Zamani implementation must be safe by construction where possible.

Rust implementation requirements:

Rust 2021
Rust 1.97.1
no unsafe

No production source may introduce:

unsafe
unsafe fn
unsafe impl
unsafe trait
unsafe { ... }

The Rust implementation must use safe abstractions.

---

88. Zamani "unsafe"

A source-language construct named "unsafe" must not be confused with Rust implementation safety.

If Zamani retains an "unsafe" language construct, it must have:

- a formal specification;
- semantic meaning;
- capability restrictions;
- security model;
- diagnostics;
- compiler implementation;
- runtime implementation;
- tests;
- compatibility policy.

An incomplete "unsafe" syntax is not acceptable as a production feature.

If Zamani is intended to provide a safe-only language, such a construct must be removed or explicitly restricted through the compatibility process.

---

89. Resource Feasibility

Compilation must distinguish:

syntactically valid
semantically valid
target-compatible
resource-feasible

For example:

requires 1,000,000 qubits

may be perfectly valid source.

A target with only 100 qubits may reject execution as infeasible.

That does not mean the language only supports 100 qubits.

---

90. Infinite-Scale Interpretation

“Scale to infinity” means:

«No artificial finite language ceiling is introduced where the underlying semantics can remain parameterized.»

It does not claim that physical hardware, memory, execution time, storage, or mathematical representation is literally infinite.

Therefore:

language scalability

is distinct from:

physical resource availability

and:

mathematical representability

---

91. Resource-Aware Compilation

The compiler may discover:

available memory
available processors
available accelerators
available QPUs
available communication
available bandwidth
available storage
available capabilities
available precision
available reliability

and select a realization.

The source program should not need rewriting solely because these values change.

---

92. Portability Profiles

Target profiles may be defined externally.

A profile can specify:

capabilities
resources
constraints
ABI
available libraries
deployment policy
security policy

A profile is not the Zamani language itself.

This keeps target-specific information out of the portable grammar.

---

93. Target-Specific Extensions

Target-specific constructs must be explicit.

They should:

- identify the target domain;
- identify their compatibility requirements;
- define fallback behavior;
- declare portability impact;
- avoid contaminating universal semantics.

Target-specific syntax must never silently become a universal requirement.

---

94. Compatibility

Every stable feature must define:

introduced version
semantic status
syntax status
AST status
IR status
compiler status
runtime status
deprecation status
migration path

Compatibility must be tested at:

source
lexer
parser
AST
semantic
IR
compiler
runtime

---

95. Feature Lifecycle

Every feature has one of:

PROPOSED
EXPERIMENTAL
IMPLEMENTED
STABLE
DEPRECATED
REMOVED
HISTORICAL
NOT_IMPLEMENTED

A feature may not be marked "STABLE" merely because it appears in this document.

---

96. Feature Completion Contract

Every feature must define:

File
Purpose
Status
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
Positive Tests
Negative Tests
Boundary Tests
Scalability Tests
Compatibility Tests
Determinism Tests
Hard-Coding Audit
Diagnostics
Security
Performance
Completion Criteria

This is mandatory.

It exists specifically so that a feature/file can be completed independently without later architectural rework.

---

97. Feature Manifests

Machine-readable feature contracts belong under:

grammar/specification/features/

Each manifest should identify:

id
name
status
version
syntax
grammar
lexer_tokens
ast_nodes
semantic_rules
ir_mapping
compiler_consumers
runtime_consumers
domain
capabilities
resource_requirements
negative_tests
boundary_tests
scalability_tests
compatibility
hard_coding_policy
diagnostics
security
performance

A feature is not complete until its manifest is complete.

---

98. Independent-First Development

The recommended implementation order is:

1. language authority
2. specification contracts
3. lexical contracts
4. core grammar
5. expression grammar
6. type grammar
7. declaration grammar
8. statement grammar
9. functions
10. modules
11. effects
12. memory
13. concurrency
14. classical
15. quantum
16. hybrid
17. HDL
18. hardware
19. resources
20. distributed
21. AI
22. data
23. networking
24. security
25. interoperability
26. dialects
27. macros
28. metaprogramming
29. Sankofa
30. temporal/MTS
31. nano
32. canonical Zamani.g4 composition
33. implementation-conformance reference
34. validation
35. complete test matrix

The root grammar is finalized only after its component contracts exist.

---

99. Existing File Preservation

Existing filenames should not be unnecessarily renamed.

In particular:

grammar/Zamani.g4
grammar/Zamani-Grammar.md
grammar/grammar.md
grammar/DESIGN.md
grammar/README.md

remain.

Existing domain files should be retained when their concepts remain valid.

Files are removed only when:

1. they are demonstrably obsolete;
2. nothing consumes them;
3. their functionality is represented elsewhere;
4. compatibility impact has been assessed;
5. removal is explicitly documented.

---

100. "grammar/antlr/"

The repository may contain an "antlr/" directory.

It must not become a second authoritative grammar root.

The preferred architecture is:

grammar/Zamani.g4

as the canonical composition root.

"grammar/antlr/" may remain temporarily if tooling genuinely consumes it.

It should be removed only after repository-wide reference analysis proves it is obsolete.

No duplicate:

grammar/antlr/Zamani.g4
grammar/Zamani.g4

may simultaneously claim authority.

---

101. Generated Files

Generated artifacts must be identifiable.

A generated file must state:

generated from
generator version
source contract
generation command
do not edit manually

Generated documentation must not become an accidental source of truth.

---

102. "grammar/grammar.md"

"grammar.md" is the implementation-conformance reference.

It should be generated/validated against:

normative specification
+
Zamani.g4
+
lexer
+
parser
+
AST

It must distinguish:

SPECIFIED
IMPLEMENTED
PARTIALLY_IMPLEMENTED
PLANNED
DEPRECATED

This prevents future design from being confused with currently executable syntax.

---

103. Testing Contract

The grammar test suite must include:

lexical
syntax
expressions
types
declarations
control flow
functions
modules
effects
memory
concurrency
classical
quantum
hybrid
HDL
hardware
resources
distributed
AI
data
networking
security
interoperability
dialects
macros
metaprogramming
Sankofa
temporal
MTS
nano
compatibility
diagnostics
negative
boundary
scalability
determinism
portability

---

104. Positive Tests

Positive tests demonstrate legal programs.

Every stable feature must have representative positive tests.

Tests must include both:

minimal valid program

and:

realistic composed program

---

105. Negative Tests

Negative tests must prove that invalid programs are rejected.

Examples include:

- malformed syntax;
- invalid types;
- impossible constraints;
- missing capabilities;
- invalid resource expressions;
- illegal ownership;
- illegal quantum operations;
- invalid effect usage;
- invalid HDL constructs;
- incompatible interoperability declarations.

---

106. Boundary Tests

Boundary tests test semantic edges rather than artificial universal maxima.

Examples:

empty collection
single element
single qubit
single operation
zero-length valid construct where permitted
large symbolic value
nested generic types
deep module paths
large expression trees
large generated structures

Boundary tests must never establish arbitrary hardware ceilings.

---

107. Scalability Tests

Scalability tests must verify that the implementation does not accidentally introduce fixed limits.

They should exercise progressively larger:

- source files;
- declarations;
- expressions;
- modules;
- tensors;
- data;
- quantum registers;
- distributed nodes;
- concurrent tasks;
- hardware structures.

The expected outcome is:

resource exhaustion

rather than:

language-defined artificial maximum

when the implementation reaches resource limits.

---

108. Determinism Tests

Where deterministic behavior is promised, tests must verify:

same source
+
same language version
+
same semantic inputs
+
same compilation policy
=
same observable result

where deterministic semantics apply.

---

109. Compatibility Tests

Compatibility tests must compare:

version N
version N+1

for:

- syntax;
- semantics;
- AST;
- diagnostics;
- IR;
- interoperability;
- migration behavior.

---

110. Hard-Coding Audit

Every grammar and compiler feature must be audited for artificial limits.

Search targets include:

MAX_
LIMIT_
CAPACITY_
QUANTUM_COUNT
CPU_COUNT
GPU_COUNT
FPGA_COUNT
NODE_COUNT
THREAD_COUNT
TENSOR_MAX
REGISTER_MAX
DEVICE_COUNT

But the audit must also detect hidden hard-coding such as:

if qubits > 1024
if nodes == 8
physical_qubit(17)
gpu_0
cpu_7

unless the value is explicitly part of a target-specific realization.

---

111. Domain Integration Matrix

Every domain must define:

syntax
tokens
AST
semantic model
resource model
capability model
canonical IR
compiler consumers
runtime consumers
tests
compatibility

Domains must never be considered complete solely because their ".g4" files parse.

---

112. Classical Integration

classical syntax
    ↓
common AST
    ↓
classical semantic model
    ↓
Classical IR
    ↓
optimization
    ↓
target lowering

---

113. Quantum Integration

quantum syntax
    ↓
common AST Operation
    ↓
quantum semantic validation
    ↓
quantum::ir
    ↓
optimization
    ↓
routing
    ↓
scheduling
    ↓
QEC/resilience
    ↓
ZQN
    ↓
HAL

---

114. HDL Integration

HDL syntax
    ↓
common AST
    ↓
hardware semantic model
    ↓
HDL/Hardware IR
    ↓
optimization
    ↓
synthesis/simulation/verification
    ↓
target

---

115. Hybrid Integration

Hybrid programs may cross:

classical
↔
quantum
↔
AI
↔
data
↔
accelerator
↔
distributed

The same source program remains one semantic unit.

---

116. AI/Quantum Integration

A valid future pipeline may be:

dataset
 ↓
tensor/model
 ↓
classical preprocessing
 ↓
quantum kernel
 ↓
measurement
 ↓
classical optimization
 ↓
training

No second language is required.

---

117. Hardware/Quantum Integration

Quantum hardware requirements must be expressed through:

capabilities
resources
constraints
preferences

rather than universal physical identifiers.

---

118. Distributed/Quantum Integration

Distributed quantum programs may express:

distributed quantum computation
communication requirements
entanglement/communication capabilities
resource requirements
fault tolerance

Physical network realization remains downstream.

---

119. Networking/Distributed Integration

Networking syntax represents communication semantics.

Distributed execution determines:

node placement
routing
replication
failure handling

The source language should not require a fixed number of nodes.

---

120. Security/Distributed Integration

Security policies may constrain:

identity
authorization
communication
data placement
execution
secrets
trust

The runtime must enforce actual security.

---

121. Compiler Integration

Every stable syntax feature must specify its compiler consumer.

A feature is incomplete if:

parser accepts it

but:

compiler does not know what it means

Compiler lowering must be deterministic where required and semantics-preserving.

---

122. Runtime Integration

Runtime consumers must be identified for features involving:

- execution;
- resources;
- concurrency;
- networking;
- distributed computation;
- resilience;
- persistence;
- hardware;
- quantum execution.

Grammar must never silently imply runtime behavior that has no implementation.

---

123. Tooling Integration

Tooling must eventually consume the same contracts for:

- syntax highlighting;
- formatting;
- diagnostics;
- IDE completion;
- refactoring;
- documentation;
- static analysis;
- language servers.

Tooling must not maintain a separate unofficial grammar.

---

124. Documentation Integration

Documentation must distinguish:

normative
informative
experimental
historical
implementation-specific

A code example must not imply stability if its feature is experimental.

---

125. Production Acceptance Rule

A domain or feature is production-ready only when all applicable conditions hold:

[ ] specification exists
[ ] ownership defined
[ ] syntax defined
[ ] lexer contract defined
[ ] parser contract defined
[ ] AST mapping defined
[ ] semantic mapping defined
[ ] IR mapping defined
[ ] compiler consumer defined
[ ] runtime consumer defined
[ ] source spans defined
[ ] diagnostics defined
[ ] positive tests exist
[ ] negative tests exist
[ ] boundary tests exist
[ ] scalability tests exist
[ ] compatibility tests exist
[ ] determinism tests exist where applicable
[ ] hard-coding audit passes
[ ] security audit passes
[ ] performance expectations defined
[ ] cross-domain integration defined
[ ] feature manifest complete
[ ] implementation status verified

---

126. What Must Never Happen

Zamani must never evolve into:

one grammar for classical
+
another grammar for quantum
+
another grammar for HDL
+
another grammar for AI

nor:

one AST for classical
+
one quantum AST
+
one hardware AST

nor:

source
 ↓
vendor-specific representation

as the universal architecture.

The language must remain unified.

---

127. What May Be Specialized

Specialization is permitted after the canonical semantic boundary.

Examples:

Classical IR
quantum::ir
HDL/Hardware IR
vendor backend
target runtime

Specialization is an implementation concern.

It must not fragment the source language.

---

128. Future-Proofing

Future computational domains must be addable without redesigning the entire language.

A future domain should be able to define:

domain name
syntax extensions
semantic model
capabilities
resources
AST mapping
IR mapping
compiler integration
runtime integration
tests
compatibility

without changing the fundamental language architecture.

---

129. Extensibility Rule

New features should prefer:

generic syntax
+
typed semantics
+
capabilities
+
resources
+
operations

over:

new keyword for every feature

This prevents the language from becoming an unmaintainable dictionary of technologies.

---

130. Stable Core

The most stable parts of Zamani should be:

lexical model
identifiers
source locations
modules
types
expressions
statements
functions
effects
ownership
resource model
capability model
diagnostics
semantic model
interoperability boundaries

Domains should build upon this foundation.

---

131. Technology-Neutral Principle

The language should describe computation independently of today's:

- CPU vendors;
- GPU vendors;
- FPGA vendors;
- QPU vendors;
- accelerator APIs;
- AI frameworks;
- network vendors;
- cloud vendors;
- compiler backends.

Vendor integration belongs downstream.

---

132. Representation-Neutral Principle

The same semantic concept may be represented differently by different targets.

For example:

parallel computation

may become:

CPU threads
GPU lanes
FPGA pipelines
distributed tasks
quantum operations
future execution units

without changing source semantics.

---

133. Resource-Neutral Principle

The source program describes what it needs.

The compiler discovers what is available.

The runtime manages what is actually present.

The target realizes the computation.

Therefore:

source semantics
≠
resource inventory

---

134. Topology-Neutral Principle

Topology belongs downstream.

A portable program must not require:

8-node cluster
4-GPU topology
specific QPU connectivity
specific FPGA routing

unless explicitly target-specific.

---

135. Calibration-Neutral Principle

Calibration data belongs to target/runtime infrastructure.

It must not become part of universal source grammar.

Quantum programs may express requirements such as:

requires fidelity >= threshold

but the actual calibration state belongs to HAL/backend infrastructure.

---

136. Physical-Qubit-Neutral Principle

A logical quantum program operates on logical identities.

Physical mapping is downstream.

This ensures:

same source
+
different QPU
=
different legal physical mapping

without source rewriting.

---

137. Compiler Discovery

The compiler may discover:

resources
capabilities
libraries
accelerators
topology
timing
memory
precision
security

and choose an implementation.

The choice must respect semantic requirements.

---

138. Runtime Discovery

Runtime may discover changing conditions such as:

resource availability
device state
load
faults
network state
calibration
thermal state
power state

where the language/runtime contract permits dynamic adaptation.

Adaptation must preserve stated semantics.

---

139. Resilience and Recovery

Where resilience is declared, runtime may use:

checkpoint
retry
recover
migrate
failover
degrade
escalate

The grammar describes policy/intent.

The runtime implements recovery.

---

140. Observability

Execution constructs may support:

- logs;
- traces;
- metrics;
- profiling;
- provenance;
- diagnostics.

Observability must not change program semantics unless explicitly specified.

---

141. Provenance

Provenance may record:

source version
compiler version
language version
dependencies
target profile
optimization profile
artifact identity
runtime environment
execution identity

Provenance belongs to the build/execution ecosystem and should be accessible without becoming embedded into every language construct.

---

142. Reproducibility

Reproducible builds require:

stable source
stable dependencies
defined compiler
defined language version
defined configuration
defined optimization policy

When nondeterminism is permitted, its source must be explicit.

---

143. Deterministic Compilation

Where required:

same input
+
same compiler
+
same configuration
=
same semantic artifact

Target-specific lowerings may differ while preserving canonical semantics.

---

144. Error Semantics

Errors must be classified consistently.

Possible categories include:

lexical error
syntax error
name error
type error
effect error
ownership error
resource error
capability error
semantic error
target incompatibility
resource infeasibility
runtime error
security error

The grammar is responsible primarily for lexical/syntax structure.

---

145. Versioning

Language versions must be explicit.

Version changes must identify:

grammar changes
semantic changes
AST changes
IR changes
compiler changes
runtime changes
compatibility impact
migration path

---

146. Deprecation

Deprecated constructs must remain documented until their removal policy permits removal.

Deprecation must include:

reason
replacement
first deprecated version
planned removal
migration guidance
compatibility behavior

---

147. Historical Features

Historical NIMBUS, Universal Trinity, Sankofa, MTS, nano, and other concepts may remain documented for continuity.

Historical status must never imply implementation.

---

148. Experimental Features

Experimental features must be explicitly marked.

They must not silently become stable merely because they appear in "Zamani-Grammar.md".

Experimental syntax requires:

- feature identifier;
- feature gate;
- status;
- semantic contract;
- tests;
- compatibility policy.

---

149. Grammar Composition Rule

"grammar/Zamani.g4" is the canonical ANTLR composition root.

The modular files under:

grammar/core/
grammar/types/
grammar/expressions/
grammar/statements/
grammar/classical/
grammar/quantum/
grammar/hdl/
...

provide contracts and modular grammar components.

The root grammar must compose them consistently.

There must be no second authoritative root.

---

150. Grammar Modularity

Subdirectories should be created when they improve:

- ownership;
- maintainability;
- testability;
- discoverability;
- independent completion;
- integration traceability.

Directories must not be created merely to split files artificially.

Every directory must have a documented purpose.

---

151. File Independence

Each implementation/specification file must be independently completable.

Before implementation begins, it must know:

upstream contracts
downstream consumers
AST target
semantic target
IR target
tests
diagnostics
compatibility
hard-coding policy

This prevents:

finish file A
→ modify A after file B
→ modify A again after file C
→ modify A again after IR

The contracts must be decided first.

---

152. Integration Before Implementation

The preferred process is:

contract
 ↓
ownership
 ↓
interfaces
 ↓
AST
 ↓
semantic mapping
 ↓
IR mapping
 ↓
tests
 ↓
implementation

not:

write grammar
 ↓
discover AST later
 ↓
discover IR later
 ↓
rewrite grammar

---

153. Repository-Wide Traceability

Every stable feature must be traceable:

feature ID
 ↓
specification
 ↓
grammar
 ↓
lexer
 ↓
parser
 ↓
AST
 ↓
semantic analyzer
 ↓
IR
 ↓
compiler
 ↓
runtime
 ↓
tests

A missing link means the feature is incomplete.

---

154. Canonical Example: Portable Quantum Program

The source-level intent is conceptually:

allocate a logical quantum register
apply operations
measure
use classical result

The implementation may become:

logical qubits
 ↓
encoded qubits
 ↓
physical qubits
 ↓
routed operations
 ↓
scheduled pulses
 ↓
QEC
 ↓
ZQN
 ↓
HAL
 ↓
QPU

The programmer should not need to rewrite the algorithm merely because the QPU changes.

---

155. Canonical Example: Portable Parallel Program

Source:

parallel computation

Target A:

CPU threads

Target B:

GPU

Target C:

distributed cluster

Target D:

future accelerator

The semantic contract remains the same.

---

156. Canonical Example: Hardware Co-Design

A program may describe:

algorithm
+
parallelism
+
memory requirements
+
latency constraints
+
accelerator capability

The compiler may produce:

software implementation

or:

hardware accelerator

or:

hybrid implementation

depending on available capabilities and policies.

---

157. Canonical Example: AI

A model program may describe:

model
dataset
training
inference
tensor operations

The compiler may select:

CPU
GPU
TPU-like accelerator
FPGA
distributed accelerator
future accelerator

without requiring the source algorithm to become vendor-specific.

---

158. Canonical Example: HDL

A hardware description may specify:

parameterized datapath
pipeline
memory
interface
timing requirement
verification property

A target-specific synthesis flow later determines:

FPGA
ASIC
simulation
emulation
future hardware

---

159. Production Invariants

The following invariants are mandatory:

Invariant 1

One Zamani language.

Invariant 2

One canonical ANTLR composition root.

Invariant 3

One domain-neutral AST architecture.

Invariant 4

One canonical quantum semantic boundary: "quantum::ir".

Invariant 5

No universal hardware limits in grammar.

Invariant 6

Requirements, capabilities, constraints, preferences, hints, and realizations remain distinct.

Invariant 7

Domain implementations remain downstream from source semantics.

Invariant 8

Every feature has a complete traceability contract.

Invariant 9

Experimental syntax cannot silently become stable syntax.

Invariant 10

Generated documentation cannot become an accidental source of truth.

Invariant 11

Target-specific realization cannot redefine portable semantics.

Invariant 12

Rust implementation is Rust 2021 / Rust 1.97.1 / safe Rust only.

---

160. Production Readiness Checklist

"grammar/Zamani-Grammar.md" is integrated correctly only when:

[ ] no competing grammar authority exists
[ ] Zamani.g4 remains canonical ANTLR root
[ ] grammar.md is implementation-conformance documentation
[ ] this file is extended design/reference material
[ ] normative specification is authoritative
[ ] feature manifests exist
[ ] lexical authority is defined
[ ] token identity is deterministic
[ ] AST mapping exists for stable syntax
[ ] semantic mapping exists for stable syntax
[ ] IR mapping exists for stable syntax
[ ] quantum::ir remains canonical
[ ] quantum grammar does not hard-code gate lists
[ ] physical qubit mapping is downstream
[ ] QEC remains downstream
[ ] ZQN remains downstream
[ ] HAL remains downstream
[ ] routing remains downstream
[ ] scheduling remains downstream
[ ] optimization remains downstream
[ ] HDL remains target-independent
[ ] hardware resources remain abstract
[ ] AI remains framework-neutral
[ ] distributed computing has no fixed node limit
[ ] concurrency has no fixed thread limit
[ ] tensors have no artificial universal dimension limit
[ ] timelines have no artificial universal count
[ ] resource/capability semantics are separated
[ ] diagnostics have source spans
[ ] negative tests exist
[ ] boundary tests exist
[ ] scalability tests exist
[ ] compatibility tests exist
[ ] determinism tests exist where applicable
[ ] hard-coding audit passes
[ ] security audit passes
[ ] cross-domain integration is defined
[ ] Rust baseline is Rust 1.97.1
[ ] production Rust contains no unsafe

---

161. Final Architecture

The complete Zamani language architecture is:

                         ZAMANI SOURCE
                              │
                              ▼
                    ┌──────────────────┐
                    │ Lexical Contract │
                    └────────┬─────────┘
                             ▼
                          Lexer
                             │
                             ▼
                    ┌──────────────────┐
                    │ Canonical Grammar│
                    │    Zamani.g4     │
                    └────────┬─────────┘
                             ▼
                           Parser
                             │
                             ▼
                     Domain-Neutral AST
                             │
             ┌───────────────┼────────────────┐
             │               │                │
             ▼               ▼                ▼
        Name/Module       Type System      Effects
        Resolution
             │               │                │
             └───────────────┼────────────────┘
                             ▼
                  Ownership / Resources
                             │
                             ▼
                    Capabilities
                             │
                             ▼
                 Portability Analysis
                             │
                             ▼
                  Semantic Validation
                             │
                             ▼
                Canonical Semantic Model
                             │
             ┌───────────────┼────────────────┐
             │               │                │
             ▼               ▼                ▼
       Classical          quantum::ir       HDL /
          IR                                Hardware IR
             │               │                │
             └───────────────┼────────────────┘
                             ▼
                       Optimization
                             │
             ┌───────────────┼────────────────┐
             │               │                │
             ▼               ▼                ▼
          Routing        Scheduling       Resilience
             │               │                │
             └───────────────┼────────────────┘
                             ▼
                            QEC
                             │
                             ▼
                            ZQN
                             │
                             ▼
                            HAL
                             │
                             ▼
                     Target Realization
                             │
       ┌─────────────┬───────┼───────┬─────────────┐
       │             │       │       │             │
       ▼             ▼       ▼       ▼             ▼
      CPU           GPU     FPGA    QPU       Distributed /
                                             Future Targets

---

162. Final POCO-REAF Principle

The central Zamani rule is:

«The source program describes computation, semantics, correctness, capabilities, resources, constraints, preferences, and policies. It does not unnecessarily describe the physical machine on which those semantics will eventually be realized.»

Therefore:

PROGRAM ONCE
     ↓
SEMANTICALLY DEFINE ONCE
     ↓
COMPILE ONCE
     ↓
DISCOVER CAPABILITIES
     ↓
SELECT RESOURCES
     ↓
OPTIMIZE
     ↓
LOWER
     ↓
ROUTE
     ↓
SCHEDULE
     ↓
RESOLVE RESILIENCE/QEC/ZQN
     ↓
REALIZE THROUGH HAL
     ↓
RUN

The same source-level semantics can therefore scale from:

atom

through:

single device
single machine
heterogeneous machine
accelerator
quantum processor
embedded system
cluster
cloud
distributed system

and toward future computational environments, limited only by the actual semantics, representation, capabilities, policies, and resources involved.

---

163. Final Non-Negotiable Rule

Zamani must never solve today's hardware limitations by turning them into tomorrow's language limitations.

Do not encode:

today's maximum qubits
today's maximum CPUs
today's maximum GPUs
today's maximum FPGA resources
today's maximum tensor size
today's maximum memory
today's maximum network size
today's maximum nodes
today's maximum accelerators

as permanent language rules.

Instead encode:

semantic intent
+
capability requirements
+
resource requirements
+
constraints
+
preferences
+
hints

and let:

compiler
+
optimizer
+
router
+
scheduler
+
resilience
+
QEC
+
ZQN
+
HAL
+
runtime
+
target backend

determine the physical realization.

That is the architecture required for:

«Program Once → Compile Once → Run Everywhere → Anywhere → Forever.»