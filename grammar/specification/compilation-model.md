Below is the complete proposed content for grammar/specification/compilation-model.md. It is written as a normative compilation specification, while keeping syntax in the grammar, semantic meaning in semantic-model.md, execution behavior in execution-model.md, scalability in scalability-model.md, and POCO-REAF guarantees in poco-reaf.md.

Zamani Compilation Model

Path: "grammar/specification/compilation-model.md"
Status: Production Specification
Scope: Zamani language compilation semantics and architecture
Implementation Baseline: Rust 1.97 / Rust 1.97.1
Implementation Safety Requirement: No "unsafe" Rust
Primary Goal: Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)

---

1. Purpose

This document defines the normative compilation model for the Zamani programming language.

It specifies how a Zamani source program progresses from source text to portable semantic representation and, where required, to target-specific executable artifacts.

The compilation model MUST support:

- classical computation;
- quantum computation;
- hybrid quantum-classical computation;
- HDL and hardware description;
- hardware/software co-design;
- embedded systems;
- systems programming;
- parallel and concurrent computation;
- distributed computation;
- HPC;
- AI/ML;
- numerical and scientific computation;
- data processing;
- accelerators;
- networking;
- cryptography;
- heterogeneous systems;
- future computing models.

The compilation architecture MUST scale from the smallest supported computation to arbitrarily large programs and machines subject only to actual implementation, resource, representation, and execution constraints.

The language MUST NOT impose artificial finite limits on:

- qubits;
- classical values;
- registers;
- cores;
- threads;
- devices;
- accelerators;
- nodes;
- memory;
- tensor dimensions;
- program size;
- module count;
- operation count;
- deployment size;
- hardware topology.

Where a physical or implementation limitation exists, that limitation belongs to the appropriate compilation, target, resource, scheduling, hardware, or runtime layer rather than becoming an accidental language limitation.

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

are normative.

A requirement stated with MUST or MUST NOT is mandatory for a conforming implementation.

---

3. Compilation Principles

Zamani compilation is based on the following principles.

3.1 Source expresses intent

A Zamani source program expresses:

- computation;
- semantics;
- data;
- control;
- effects;
- resource requirements;
- capabilities;
- constraints;
- preferences;
- portability requirements;
- hardware semantics where explicitly requested.

Source code MUST NOT accidentally encode transient characteristics of the machine on which compilation happens.

---

3.2 Compilation preserves meaning

Compilation MUST preserve the semantic meaning of the source program.

Compilation MAY change:

- representation;
- layout;
- operation ordering where semantics permit it;
- physical mapping;
- gate decomposition;
- instruction selection;
- scheduling;
- memory placement;
- accelerator selection;
- communication placement;
- implementation strategy.

Compilation MUST NOT change observable program behavior except where the language explicitly permits implementation-dependent variation.

---

3.3 Compilation is layered

The compiler MUST be architected as a sequence of independently defined transformations.

The conceptual pipeline is:

Source
  ↓
Lexing
  ↓
Parsing
  ↓
Concrete Syntax Tree
  ↓
AST
  ↓
Name Resolution
  ↓
Semantic Analysis
  ↓
Type / Effect / Capability Analysis
  ↓
Canonical Semantic Representation
  ↓
Canonical IR
  ↓
Domain Lowering
  ↓
Optimization
  ↓
Resource / Capability Resolution
  ↓
Quantum Routing / Hardware Mapping
  ↓
Scheduling
  ↓
Target Lowering
  ↓
Executable / Deployable Artifact
  ↓
Runtime / Execution

The exact number of intermediate representations MAY vary by implementation.

The semantic boundaries MUST NOT.

---

4. Compilation Authority

Compilation responsibilities are divided among repository layers.

Concern| Owner
Lexical syntax| "grammar/lexer/"
Concrete syntax| "grammar/Zamani.g4"
Syntax specification| "grammar/specification/syntax-model.md"
Semantic meaning| "grammar/specification/semantic-model.md"
Compilation stages| this document
Execution semantics| "grammar/specification/execution-model.md"
Scalability| "grammar/specification/scalability-model.md"
POCO-REAF| "grammar/specification/poco-reaf.md"
AST| compiler/frontend implementation
Classical canonical representation| repository compiler/IR owner
Quantum canonical representation| "src/quantum/ir/"
Quantum error correction| "src/quantum/qec/"
Quantum noise/fault semantics| "src/quantum/zqn/"
Optimization| "src/quantum/optimization/" and corresponding universal compiler layers
Routing| routing subsystem
Scheduling| "src/quantum/scheduling/" and corresponding universal scheduling layers
Hardware capabilities| hardware abstraction layer
Calibration| hardware/calibration subsystem
Runtime execution| runtime subsystem
Resilience decisions| "src/quantum/resilience/"
Benchmarking| benchmarking subsystem
Target discovery| target/hardware infrastructure
Deployment| deployment/execution infrastructure

No layer MAY silently assume ownership of another layer's semantics.

---

5. Compilation Units

A compilation unit is the fundamental source-level input to compilation.

A compilation unit MAY contain:

- declarations;
- definitions;
- functions;
- types;
- modules;
- imports;
- exports;
- implementations;
- constants;
- compile-time declarations;
- classical computation;
- quantum computation;
- hardware descriptions;
- effects;
- resource requirements;
- target-independent constraints;
- dialect declarations.

The compilation model MUST NOT require a fixed number of declarations, modules, functions, qubits, or operations.

---

6. Source-to-Artifact Model

A compiler SHOULD conceptually distinguish at least the following artifacts:

Source Artifact
    ↓
Parsed Artifact
    ↓
Semantic Artifact
    ↓
Canonical IR Artifact
    ↓
Portable Compiled Artifact
    ↓
Target Artifact
    ↓
Deployment Artifact
    ↓
Execution Instance

These artifacts MUST NOT be conflated.

In particular:

«A target executable is not the same thing as the portable semantic program.»

This distinction is essential for POCO-REAF.

---

7. Compilation Context

Compilation operates within a compilation context.

A compilation context MAY contain:

- language version;
- enabled language features;
- imported modules;
- dialect registry;
- compiler version;
- IR version;
- target-independent semantic configuration;
- available capabilities;
- available resources;
- deployment constraints;
- optimization policy;
- numerical precision policy;
- security policy;
- reproducibility policy;
- target information.

The compilation context MUST distinguish between:

Semantic context

Information required to determine what the program means.

Realization context

Information required to determine how the program can be implemented on a particular environment.

These contexts MUST NOT be confused.

---

8. Target Independence

A program that does not explicitly require target-specific semantics MUST remain target-independent through semantic analysis.

For example:

requires quantum

MUST NOT implicitly mean:

requires device X
requires N qubits
requires topology Y
requires vendor Z

Likewise:

requires accelerator

MUST NOT imply a fixed GPU, FPGA, ASIC, or accelerator count.

Target selection is a realization concern unless the program explicitly enters a target-specific domain.

---

9. Target Descriptions

A target description provides information about a possible execution environment.

It MAY describe:

- architecture;
- instruction set;
- available devices;
- memory;
- processors;
- accelerators;
- quantum devices;
- FPGA resources;
- ASIC characteristics;
- network capabilities;
- topology;
- timing capabilities;
- supported operations;
- supported precisions;
- supported effects;
- supported dialects;
- resource limits;
- reliability;
- energy characteristics.

Target descriptions MUST be external to portable semantic source unless the source intentionally describes hardware.

A target description MUST NOT modify the fundamental meaning of a target-independent program.

---

10. Requirements, Constraints, Capabilities, Preferences and Hints

Compilation MUST preserve the distinction between:

requirement
constraint
capability
preference
hint

10.1 Requirement

A requirement states something that MUST be satisfied.

Example conceptual requirement:

requires quantum

10.2 Constraint

A constraint restricts acceptable realizations.

10.3 Capability

A capability describes what an environment can provide.

10.4 Preference

A preference indicates a desired realization but MAY be overridden.

10.5 Hint

A hint provides optimization information without becoming a semantic requirement.

Compilers MUST NOT silently convert:

- hints into requirements;
- preferences into requirements;
- capabilities into source semantics.

---

11. Compilation Strategies

A conforming implementation MAY support multiple compilation strategies.

Examples include:

- ahead-of-time compilation;
- just-in-time compilation;
- staged compilation;
- incremental compilation;
- distributed compilation;
- remote compilation;
- cross compilation;
- heterogeneous compilation;
- hardware synthesis;
- quantum compilation;
- hybrid compilation;
- adaptive compilation.

The strategy MUST NOT alter language semantics.

---

12. Frontend Compilation

The frontend is responsible for transforming source text into a semantically analyzable representation.

The frontend stages are:

Characters
  ↓
Tokens
  ↓
Parse Tree
  ↓
AST
  ↓
Name Resolution
  ↓
Semantic Validation

The frontend MUST report diagnostics with source provenance.

It MUST NOT:

- discover physical hardware as part of ordinary parsing;
- schedule quantum operations;
- route qubits;
- insert target-specific gates;
- choose devices;
- perform runtime recovery;
- execute programs;
- mutate canonical runtime state.

---

13. Lexer Integration

The lexer is governed by:

grammar/lexer/

The lexer owns lexical structure.

It MUST recognize the lexical categories defined by the language specification.

It MUST NOT determine target-specific semantics.

For example, a numeric literal MAY represent:

- integer;
- floating-point value;
- exact numeric value;
- duration;
- size;
- dimension;
- quantum parameter;
- hardware parameter.

Its semantic interpretation belongs to later analysis.

---

14. Parser Integration

"grammar/Zamani.g4" is the authoritative parser grammar once grammar authority is established by:

grammar/specification/grammar-authority.md

The parser MUST produce a structurally valid parse tree.

The parser MUST NOT become the semantic compiler.

Semantic validation MUST occur after parsing.

---

15. AST

The AST is the first compiler-owned semantic syntax representation.

An AST node SHOULD retain:

- source span;
- syntactic kind;
- names;
- child relationships;
- annotations;
- relevant source metadata;
- provenance.

An AST MUST NOT contain accidental target state.

The AST SHOULD NOT contain:

- physical device IDs;
- runtime handles;
- live quantum states;
- scheduler state;
- calibration state;
- execution results.

Those belong to downstream layers.

---

16. Name Resolution

Name resolution converts source names into semantic declarations.

It MUST resolve:

- modules;
- functions;
- types;
- variables;
- constants;
- operations;
- quantum declarations;
- hardware declarations;
- dialect symbols;
- imported symbols.

Resolution MUST be deterministic for the same source and compilation environment.

Ambiguous names MUST produce diagnostics.

---

17. Type Checking

Type checking validates compatibility between values and operations.

It MUST support the type model defined by:

grammar/specification/semantic-model.md
grammar/types/

Type checking MUST support scalable type parameters where appropriate.

For example:

Vector<N>
Matrix<M,N>
Tensor<Shape>
Register<N>

MUST NOT require a compiler-wide hard-coded maximum for "N", "M", or dimensions.

Actual implementation feasibility is a later concern.

---

18. Effect Checking

Effects are analyzed using:

grammar/effects/

Effects MAY describe:

- I/O;
- quantum operations;
- hardware interaction;
- network access;
- distributed communication;
- randomness;
- external services;
- security-sensitive operations;
- resource acquisition.

An effect declaration describes semantic behavior.

It MUST NOT directly select an implementation provider unless explicitly expressed through a target-specific dialect or deployment configuration.

---

19. Capability Checking

Capability checking determines whether the requested realization is possible within the supplied compilation environment.

For example:

requires quantum

is semantically valid independently of whether the target currently provides a quantum processor.

If no suitable target exists, compilation or deployment MAY fail with a capability error.

That failure MUST NOT imply that the source program itself was semantically invalid.

---

20. Resource Analysis

Resource analysis determines whether a realization can satisfy resource requirements.

Resources MAY include:

- memory;
- processing capacity;
- quantum resources;
- accelerator capacity;
- communication capacity;
- storage;
- energy;
- timing;
- concurrency;
- device availability.

Resource analysis MUST NOT introduce artificial language limits.

A program MAY require more resources than the selected environment can provide.

This is a realization failure, not necessarily a language failure.

---

21. Canonical Semantic Representation

After semantic validation, the compiler MUST lower the program into a canonical semantic representation.

This representation MUST:

- be independent of accidental parser structure;
- resolve names;
- preserve semantic identity;
- preserve source provenance;
- preserve effects;
- preserve relevant resource requirements;
- preserve quantum semantics;
- preserve hardware semantics where explicitly requested;
- preserve observable ordering;
- preserve required precision;
- preserve explicit nondeterminism.

---

22. Canonical IR

Canonical IR is the principal boundary between language semantics and backend implementation.

The grammar MUST NOT define a competing IR.

For quantum computation, the repository's canonical quantum boundary remains:

quantum::ir

Quantum grammar constructs MUST lower into the canonical quantum IR rather than introducing independent gate, qubit, circuit, or operation models in the grammar layer.

---

23. Classical IR Integration

Classical constructs MUST lower into the repository's canonical classical/compiler representation.

The grammar MUST NOT create an independent classical execution engine.

Classical IR MAY represent:

- scalar operations;
- aggregates;
- functions;
- control flow;
- memory;
- vectors;
- matrices;
- tensors;
- concurrency;
- parallelism;
- symbolic computation;
- accelerator operations.

---

24. Quantum Compilation

Quantum compilation MUST follow:

Zamani Quantum Syntax
        ↓
Quantum Semantic Analysis
        ↓
Canonical Quantum IR
        ↓
Quantum Optimization
        ↓
Routing / Mapping
        ↓
Scheduling
        ↓
Target Lowering
        ↓
Execution

The grammar MUST NOT perform these backend transformations.

---

25. Quantum Identity

Portable quantum programs SHOULD use logical quantum identity.

A logical qubit represents a semantic quantum resource.

A physical qubit represents a target-specific realization.

Physical identifiers MUST NOT leak into portable semantic identity.

A target may map:

logical qubit A

to different physical resources on different executions.

This mapping is a compilation concern.

---

26. Quantum Scalability

The compiler MUST NOT impose source-level limits such as:

MAX_QUBITS = 32
MAX_QUBITS = 64

Nor may it implicitly assume:

q[0]
q[1]

or any other fixed qubit layout.

Collection indexing MUST remain a general semantic operation.

If a target cannot provide sufficient qubits, the target realization MAY fail.

---

27. Quantum Operations

Quantum operations MUST be represented semantically.

Operation identity MUST NOT depend on arbitrary strings interpreted differently by different compiler stages.

Operations SHOULD resolve to declarations, intrinsic semantic operations, or registered dialect operations.

Unknown operations MUST produce proper diagnostics unless they are valid registered extensions.

They MUST NOT be silently emitted as comments or ignored.

---

28. Quantum Measurement

Measurement is a semantic operation.

The compiler MUST preserve:

- measurement target;
- measurement basis where applicable;
- classical destination;
- ordering;
- conditional dependencies;
- state-transition semantics.

The compiler MUST NOT automatically add measurements merely because a backend requires them unless such insertion is an explicit, semantics-preserving lowering transformation.

---

29. Dynamic Quantum Programs

Dynamic quantum programs MAY contain:

- mid-circuit measurement;
- classical feedback;
- conditional quantum operations;
- reset;
- loops;
- runtime-dependent control.

Compilation MUST preserve dependencies between quantum and classical operations.

A static circuit-only representation MUST NOT be used if it cannot faithfully represent required dynamic semantics.

---

30. QEC Integration

Quantum error-correction constructs MAY express:

- logical encoding intent;
- correction requirements;
- protection requirements;
- code families where explicitly semantic;
- fault-tolerance requirements;
- logical resource requirements.

The grammar MUST NOT implement QEC algorithms.

QEC algorithm selection and execution belong to:

src/quantum/qec/

Compilation MAY lower semantic QEC requirements into QEC-specific IR or compiler requests.

---

31. ZQN Integration

ZQN represents noise and fault semantics.

Compilation MAY consume ZQN information when performing noise-aware compilation.

However:

grammar → ZQN

does not mean the grammar owns noise semantics.

The ownership remains:

ZQN → faults/noise/execution conditions

Compilation consumes those descriptions when necessary.

Noise MUST NOT silently redefine the meaning of a quantum program.

---

32. Optimization

Optimization transforms an intermediate representation while preserving semantics.

Optimization MAY perform:

- dead-code elimination;
- constant folding;
- algebraic simplification;
- gate cancellation;
- gate decomposition;
- common subexpression elimination;
- tensor transformations;
- memory optimization;
- communication optimization;
- accelerator transformations.

Quantum optimization MUST operate on canonical quantum IR rather than a duplicate grammar-level gate model.

---

33. Optimization Correctness

Every optimization MUST satisfy semantic preservation.

Conceptually:

Meaning(before) ≡ Meaning(after)

subject to explicitly declared numerical, probabilistic, approximation, or implementation tolerances.

Optimizations MUST NOT silently weaken:

- correctness;
- security;
- precision;
- measurement semantics;
- effect semantics;
- ordering guarantees;
- hardware safety requirements.

---

34. Routing

Routing converts logical resources into physical resources.

For quantum programs this MAY include:

- logical-to-physical mapping;
- connectivity resolution;
- movement;
- swap insertion;
- topology-aware transformation.

Routing belongs downstream of semantic compilation.

The grammar MUST NOT encode a fixed topology.

---

35. Scheduling

Scheduling determines executable ordering and timing where required.

The scheduling subsystem MAY consume:

- dependency information;
- durations;
- resource requirements;
- hardware capabilities;
- calibration;
- timing constraints.

Scheduling MUST NOT change semantics unless timing itself is part of the declared semantic contract.

The grammar MUST NOT contain a fixed hardware timing grid.

---

36. Timing

Zamani distinguishes:

Semantic timing

Timing behavior that is part of program meaning.

Realization timing

Actual target timing.

Examples of realization timing include:

- hardware clock periods;
- pulse durations;
- instruction latency;
- network latency;
- device synchronization.

Realization timing belongs downstream.

Semantic duration units MUST remain target-independent until lowering.

---

37. HDL Compilation

HDL constructs MUST support compilation into:

- simulation;
- synthesis;
- hardware IR;
- FPGA implementation;
- ASIC implementation;
- hardware co-design pipelines.

HDL semantic constructs MUST remain distinct from physical implementation details.

For example:

clock

is a semantic construct.

A particular:

100 MHz oscillator

is a realization property unless explicitly described by the hardware program.

---

38. Hardware Description

When Zamani explicitly describes hardware, hardware semantics become part of the program.

The compiler MAY then lower:

hardware module
→ hardware IR
→ synthesis
→ placement
→ routing
→ bitstream / netlist / implementation

The hardware program itself MUST still distinguish parameters and generics from fixed implementation choices where possible.

---

39. Simulation vs Synthesis

A hardware description MAY be compiled for:

- simulation;
- synthesis;
- formal verification;
- emulation;
- FPGA;
- ASIC;
- hardware-in-the-loop.

Simulation semantics MUST NOT be confused with physical implementation semantics.

A simulator MUST NOT become the semantic authority for hardware programs.

---

40. Hybrid Compilation

Hybrid programs combine domains.

Examples include:

classical
+
quantum

or:

classical
+
quantum
+
accelerator

or:

classical
+
HDL
+
hardware
+
distributed

The compiler MUST preserve explicit boundaries between domains.

Each domain lowers through its canonical representation before cross-domain integration.

---

41. Cross-Domain IR

Cross-domain compilation MAY use a universal orchestration IR or a structured collection of domain IRs.

It MUST NOT duplicate domain ownership.

For example:

Universal semantic layer
        ↓
Classical IR
Quantum IR
Hardware IR
Data IR
Distributed IR
        ↓
Cross-domain lowering

"quantum::ir" remains authoritative for quantum semantics.

---

42. Distributed Compilation

Distributed programs MAY describe:

- nodes;
- services;
- messages;
- remote execution;
- replication;
- consistency;
- placement;
- fault tolerance.

The compiler MUST NOT assume a fixed node count.

Node count is determined by:

- program requirements;
- deployment configuration;
- available resources;
- runtime conditions.

---

43. Concurrency and Parallelism

Compilation MAY transform concurrent computation into different execution strategies.

Examples:

- threads;
- tasks;
- actors;
- processes;
- SIMD;
- vector operations;
- GPU kernels;
- distributed workers;
- quantum/classical concurrent regions.

Transformations MUST preserve the declared synchronization and memory semantics.

No fixed thread or core count may be embedded in the language unless explicitly declared as semantic hardware description.

---

44. AI/ML Compilation

AI/ML constructs MAY lower into representations for:

- CPU;
- GPU;
- TPU-like accelerators;
- FPGA;
- quantum accelerators;
- distributed systems;
- heterogeneous devices.

Tensor dimensions SHOULD be symbolic or parameterized when they are not semantically fixed.

The compiler MUST distinguish:

semantic tensor shape

from:

hardware execution tile

---

45. Data Compilation

Data operations MAY lower into:

- in-memory computation;
- streaming;
- vectorized computation;
- distributed data processing;
- accelerator execution.

The compiler MUST NOT impose artificial maximum dataset size.

Actual storage and processing limits belong to resource realization.

---

46. Networking Compilation

Network semantics MAY be lowered into:

- local communication;
- IPC;
- sockets;
- RDMA;
- message systems;
- distributed runtimes;
- hardware networking.

A semantic endpoint MUST NOT inherently identify a fixed machine.

---

47. Security Compilation

Security constructs MUST survive compilation.

Compiler transformations MUST preserve:

- authorization requirements;
- confidentiality requirements;
- integrity requirements;
- identity requirements;
- trust boundaries;
- cryptographic requirements;
- privacy constraints.

Compiler optimizations MUST NOT bypass security constraints.

---

48. Compile-Time Evaluation

Compile-time execution MUST be explicit and controlled.

Compile-time computation MAY perform:

- constant evaluation;
- type computation;
- generic specialization;
- macro expansion;
- compile-time generation;
- reflection.

Compile-time execution MUST NOT silently obtain arbitrary filesystem, network, device, or secret access.

Such capabilities MUST be explicit effects/capabilities where supported.

---

49. Macros

Macro expansion MUST occur in a well-defined phase.

A recommended conceptual sequence is:

source
↓
lex/parse
↓
macro recognition
↓
hygienic expansion
↓
AST
↓
semantic analysis

Macro expansion MUST preserve source provenance.

Generated code MUST remain subject to normal semantic validation.

Macros MUST NOT bypass type, security, capability, or effect checking.

---

50. Generics and Specialization

Generics MUST support scalable abstractions.

Specialization MAY occur:

- at compile time;
- at link time;
- at deployment time;
- at runtime.

Specialization MUST NOT require hard-coded finite domain sizes.

A generic program SHOULD be capable of being instantiated for different:

- data sizes;
- hardware sizes;
- qubit counts;
- tensor dimensions;
- node counts;
- accelerator configurations.

---

51. Portable Compilation Artifact

A portable compiled artifact represents target-independent compiled semantics where possible.

It SHOULD contain:

- language version;
- semantic version;
- IR version;
- module identity;
- symbol information as required;
- semantic metadata;
- provenance;
- requirements;
- constraints;
- capabilities;
- effects;
- target-independent compiled representation.

It MUST NOT unnecessarily contain:

- temporary device IDs;
- ephemeral calibration state;
- secrets;
- runtime credentials;
- transient resource handles.

---

52. Target Compilation

Target compilation specializes portable representation for an environment.

Conceptually:

Portable Artifact
        +
Target Description
        +
Available Resources
        +
Policies
        ↓
Target Artifact

Target compilation MAY perform:

- instruction selection;
- device selection;
- physical mapping;
- scheduling;
- memory placement;
- accelerator selection;
- code generation;
- hardware synthesis;
- deployment packaging.

---

53. Target-Specific Compilation

Target-specific behavior MUST remain isolated.

A target-specific dialect MAY introduce:

- device-specific operations;
- vendor-specific instructions;
- hardware-specific timing;
- physical addresses;
- device topology;
- target-specific constraints.

Such constructs MUST be explicitly identifiable as target-specific.

They MUST NOT silently become requirements of the core language.

---

54. Compilation Failure Categories

Compilation errors MUST distinguish at least:

LexicalError
ParseError
NameResolutionError
TypeError
SemanticError
EffectError
CapabilityError
ResourceError
ConstraintError
DialectError
IRLoweringError
OptimizationError
RoutingError
SchedulingError
TargetLoweringError
CodeGenerationError
DeploymentError

Exact repository error types MAY differ.

Provider-specific failures MUST NOT become the core language's semantic error taxonomy.

---

55. Semantic Failure vs Realization Failure

The compiler MUST distinguish:

Invalid program

The program cannot be assigned valid language semantics.

Unsupported realization

The program is valid, but the selected environment cannot realize it.

Example:

A program requiring quantum execution

may be semantically valid even when compiled against a target with no quantum capability.

The compiler SHOULD report:

target lacks required capability: quantum

rather than:

invalid quantum program

---

56. Resource Failure

A valid program MAY exceed available resources.

Examples:

- insufficient memory;
- insufficient qubits;
- insufficient accelerator capacity;
- insufficient nodes;
- insufficient storage;
- insufficient execution time;
- insufficient bandwidth.

Such conditions MUST NOT become hard-coded grammar limits.

---

57. Incremental Compilation

Implementations MAY support incremental compilation.

Incremental compilation MUST preserve the same semantic result as complete compilation for equivalent source and compilation contexts.

Caching MUST NOT alter program semantics.

Cache keys SHOULD incorporate all semantically relevant inputs.

---

58. Deterministic Compilation

For deterministic source and compilation inputs, compilation SHOULD be deterministic.

Deterministic behavior includes:

- parsing;
- name resolution;
- semantic diagnostics ordering where specified;
- canonicalization;
- IR generation;
- reproducible optimization where requested;
- artifact identity.

Parallel compiler implementation MUST NOT introduce accidental semantic nondeterminism.

---

59. Nondeterministic Programs

A program MAY intentionally contain nondeterminism.

Sources include:

- quantum measurement;
- random number generation;
- concurrent scheduling;
- distributed races where explicitly permitted;
- external events.

Compilation MUST preserve the declared nondeterminism.

For probabilistic computation, semantic equivalence MAY require preservation of probability distributions rather than identical individual executions.

---

60. Numerical Compilation

Numeric semantics MUST specify:

- representation;
- precision;
- rounding;
- overflow behavior;
- underflow behavior;
- permitted approximation.

A compiler MUST NOT silently replace exact semantics with approximate semantics unless the language contract explicitly permits that transformation.

Hardware-specific numerical formats are realization choices unless explicitly semantic.

---

61. Unit Compilation

Semantic units such as:

- duration;
- size;
- frequency;
- energy;
- angle;
- physical quantities

MUST have explicit conversion semantics.

Hardware-specific units such as:

clock ticks

MAY be introduced during target lowering.

---

62. Provenance

Compiled artifacts SHOULD preserve provenance sufficient to establish:

- originating source;
- source version;
- semantic identity;
- compilation version;
- IR version;
- transformation history where required;
- diagnostics;
- relevant configuration.

Provenance MUST NOT contain secrets unless explicitly required and securely protected.

---

63. Source Mapping

Compilation transformations SHOULD preserve source mappings.

Diagnostics from downstream stages SHOULD be traceable to:

target artifact
→ IR
→ semantic node
→ AST node
→ source span

This is particularly important for:

- quantum operations;
- generated HDL;
- optimized code;
- macro-generated code;
- distributed execution;
- hardware synthesis.

---

64. Optimization Provenance

When an optimization materially transforms code, the compiler SHOULD be able to associate the resulting representation with the originating semantic entities.

This enables:

- debugging;
- verification;
- reproducibility;
- performance analysis;
- correctness auditing.

---

65. Verification

Compilation MAY include verification stages.

Verification MAY validate:

- type correctness;
- semantic preservation;
- resource constraints;
- quantum circuit correctness;
- hardware correctness;
- security constraints;
- numerical tolerances;
- optimization preservation;
- scheduling constraints.

Verification MUST remain distinguishable from compilation itself.

---

66. Quantum Verification

Quantum transformations SHOULD support semantic verification appropriate to the representation.

Verification MAY consider:

- unitary equivalence;
- channel equivalence;
- measurement distribution;
- observable preservation;
- logical behavior;
- permitted approximation.

Exact equivalence MUST NOT be assumed where the semantics intentionally permit probabilistic or approximate behavior.

---

67. Hardware Verification

Hardware compilation MAY support:

- static checking;
- simulation;
- formal verification;
- timing verification;
- synthesis checks;
- equivalence checking.

Hardware verification MUST NOT require a fixed machine size.

---

68. Scheduling and Compilation Correctness

Scheduling transformations MUST preserve dependency constraints.

A scheduler MAY change:

when

an operation executes.

It MUST NOT change:

what

the program means.

Where timing is semantic, the scheduler MUST preserve semantic timing constraints.

---

69. Resilience Integration

Compilation MAY emit metadata describing:

- retry eligibility;
- recovery boundaries;
- checkpoint opportunities;
- resilience requirements;
- fault tolerance requirements;
- verification requirements.

The grammar/compiler MUST NOT implement runtime resilience decisions.

Runtime resilience belongs to:

src/quantum/resilience/

Resilience MAY decide to:

- retry;
- restart;
- resume;
- rollback;
- reroute;
- reschedule;
- recompile;
- switch backend;
- change protection strategy;
- mitigate;
- quarantine;
- abort.

These decisions MUST operate on compiler/runtime contracts rather than modifying source semantics.

---

70. Checkpoints

Compilation MUST distinguish checkpoint kinds.

Possible checkpoint categories include:

ClassicalExecutionCheckpoint
CompiledProgramCheckpoint
LogicalCheckpoint
MeasurementBoundary
QECCheckpoint
ProviderSupportedStateCheckpoint

The compiler MUST NOT imply that an arbitrary unknown quantum state can always be serialized and restored.

Quantum state checkpointing is valid only where supported by the semantic and execution model.

---

71. Compile Once

POCO-REAF does not mean that one target-specific binary is physically identical across all machines.

Instead:

One source semantic program
        ↓
One portable semantic/compiled representation
        ↓
Multiple target realizations

A target-specific artifact MAY differ because different machines require different:

- instructions;
- topology;
- scheduling;
- memory layout;
- communication;
- numerical implementations;
- quantum mappings;
- hardware implementations.

The semantics remain the stable contract.

---

72. Run Everywhere

A program MAY run on multiple environments when those environments satisfy its semantic requirements.

The compiler/runtime MAY adapt:

- resource allocation;
- target mapping;
- scheduling;
- implementation strategy;
- accelerator choice;
- physical quantum mapping;
- deployment topology.

Portability MUST be determined by explicit semantic requirements rather than arbitrary compiler assumptions.

---

73. Run Anywhere

The execution environment MAY be:

- local;
- embedded;
- remote;
- cloud;
- edge;
- cluster;
- supercomputer;
- accelerator;
- quantum computer;
- simulator;
- FPGA;
- ASIC;
- heterogeneous platform.

The compilation model MUST not require a particular execution location.

---

74. Run Forever

Long-term compatibility depends on preserving:

- semantic contracts;
- versioned IR;
- versioned dialects;
- explicit compatibility policies;
- provenance;
- migration rules;
- extension namespaces.

Future hardware MUST be able to provide a new realization of an existing semantic program without changing the program's meaning.

---

75. Scalability

The language defines no arbitrary finite upper bound on program scale.

Conceptually:

LanguageScaleLimit = ∞

means:

«the language specification does not impose an artificial finite maximum.»

It does not mean that a physical computer has infinite resources.

Actual execution remains constrained by:

- available memory;
- storage;
- processing capacity;
- quantum resources;
- hardware limits;
- execution time;
- communication capacity;
- implementation representation;
- operating-system limits;
- deployment policy;
- physical reality.

These are resource constraints, not grammar limits.

---

76. Compile-Time Resource Limits

The compiler itself MAY have implementation limits.

Examples include:

- host memory;
- stack capacity;
- integer representation;
- compiler process limits;
- operating-system constraints.

Such limits MUST NOT be presented as language semantics.

They MUST be documented as implementation constraints.

Where practical, implementations SHOULD use scalable data structures and avoid unnecessary fixed-size arrays.

---

77. No Accidental Hard-Coding

The compiler and grammar MUST be audited for accidental constants representing:

- maximum qubits;
- maximum cores;
- maximum devices;
- maximum nodes;
- maximum tensor dimensions;
- maximum register sizes;
- maximum module count;
- fixed hardware topology;
- fixed device IDs.

Every such constant MUST be classified as:

1. semantic requirement;
2. target-specific requirement;
3. resource constraint;
4. implementation limitation;
5. accidental hard-coding;
6. test-only limitation;
7. documentation-only limitation.

Accidental hard-coding MUST be removed.

---

78. Compile-Time Limits vs Semantic Limits

A compiler MAY reject a program because it cannot currently process it.

This does not establish a language limit.

For example:

compiler implementation cannot allocate enough memory

is not equivalent to:

Zamani programs may contain at most N declarations

The latter is prohibited unless it is a genuine semantic requirement.

---

79. Resource Negotiation

Resource negotiation SHOULD follow:

Program Requirements
        ↓
Target Capabilities
        ↓
Available Resources
        ↓
Constraints
        ↓
Preferences
        ↓
Compiler Decisions

Negotiation MUST preserve semantic requirements.

A preference MAY be rejected.

A mandatory requirement MUST NOT be silently rejected.

---

80. Compilation Policies

Compiler policies MAY control:

- optimization level;
- performance goals;
- energy goals;
- numerical tolerances;
- reproducibility;
- debugging;
- verification;
- target selection;
- resource preferences.

Policies MUST remain distinguishable from source semantics.

---

81. Portable vs Target-Specific Semantics

A program SHOULD be classified into semantic portability levels.

Level 1 — Fully portable semantics

No target-specific semantic dependency.

Level 2 — Capability-constrained

Requires capabilities but not a specific implementation.

Level 3 — Architecture-constrained

Requires a specific architecture class.

Level 4 — Device-constrained

Requires a particular device class or implementation.

Level 5 — Physical implementation-specific

Explicitly describes physical hardware behavior.

The compiler MUST preserve this distinction.

---

82. Dialects

Dialects extend Zamani.

A dialect MAY define:

- syntax;
- types;
- operations;
- attributes;
- effects;
- lowering rules;
- target-specific constructs.

A dialect MUST:

- have a namespace;
- have a version;
- declare compatibility;
- avoid silently redefining core semantics;
- define its lowering contract.

Vendor dialects MUST remain isolated from core semantics.

---

83. Future Extensions

New computing paradigms MUST be introducible without redesigning the core compilation pipeline.

The extension model SHOULD allow future domains such as:

- neuromorphic computing;
- photonic computing;
- biological computing;
- reversible computing;
- analog computing;
- optical computing;
- molecular computing;
- new accelerator architectures;
- future quantum architectures.

New domains SHOULD integrate through:

syntax
→ semantic model
→ domain IR
→ lowering
→ target realization

rather than by modifying unrelated domains.

---

84. Interoperability

Foreign languages and systems MAY be integrated through:

grammar/interoperability/

Possible integrations include:

- C;
- C++;
- Python;
- OpenQASM;
- Verilog;
- System interfaces;
- foreign ABIs.

Interop boundaries MUST explicitly define:

- data representation;
- ownership;
- calling convention;
- effects;
- errors;
- lifetime;
- security;
- target requirements.

---

85. OpenQASM Integration

OpenQASM input/output MUST be treated as an interoperability concern.

OpenQASM syntax MUST NOT become the canonical Zamani semantic representation.

The flow SHOULD be:

OpenQASM
  ↓
OpenQASM frontend
  ↓
Zamani quantum semantic representation
  ↓
quantum::ir

and, where exporting:

quantum::ir
  ↓
OpenQASM lowering
  ↓
OpenQASM

Exporter failures MUST be explicit errors.

Unknown operations MUST NOT be silently emitted as comments.

---

86. Compiler and Runtime Boundary

The compiler produces artifacts.

The runtime executes artifacts.

The compiler MUST NOT depend on runtime state to determine ordinary language syntax.

The runtime MUST NOT reinterpret source syntax independently of compiler semantics.

The boundary is:

Compiler
    ↓
Executable / Portable Artifact
    ↓
Runtime

---

87. Runtime Specialization

A runtime MAY specialize execution using:

- current resources;
- calibration;
- health;
- queue state;
- device availability;
- topology;
- environmental information.

Such specialization MUST preserve the semantic contract.

Runtime adaptation MUST NOT mutate the meaning of the original program.

---

88. Hardware Calibration

Calibration belongs to hardware infrastructure.

Compilation MAY consume calibration information for:

- noise-aware optimization;
- pulse selection;
- routing;
- scheduling;
- device selection.

Calibration MUST NOT become part of portable source semantics unless explicitly declared as such.

---

89. Benchmarking

Benchmarking MAY influence compiler policies.

For example, benchmarking may provide information about:

- performance;
- latency;
- fidelity;
- throughput;
- energy;
- reliability.

Benchmarking MUST NOT redefine semantic correctness.

---

90. Resource Management

Resource management MAY allocate:

- CPU resources;
- memory;
- accelerators;
- quantum resources;
- devices;
- nodes;
- communication channels.

The compiler MAY produce resource requirements.

Resource ownership and allocation belong to the appropriate resource/runtime subsystem.

---

91. Compilation Metadata

Compiled artifacts SHOULD expose structured metadata for:

- source provenance;
- semantic version;
- compiler version;
- IR version;
- dialect versions;
- requirements;
- constraints;
- capabilities;
- effects;
- optimization information;
- target information;
- verification state.

Metadata MUST be versioned.

---

92. Artifact Versioning

The following versions MUST remain independently identifiable:

Language Version
Grammar Version
Semantic Model Version
Compiler Version
IR Version
Dialect Version
Target Version
Artifact Version

A compiler update MUST NOT silently imply a language semantic change.

---

93. Compatibility

Compilation compatibility is governed by:

grammar/specification/compatibility.md
grammar/compatibility/

Backward compatibility SHOULD preserve source semantics unless an explicitly documented breaking language change occurs.

Compiler compatibility MUST distinguish:

- source compatibility;
- semantic compatibility;
- IR compatibility;
- artifact compatibility;
- target compatibility.

---

94. Deprecation

Deprecated constructs MUST remain identifiable.

The compiler SHOULD produce diagnostics containing:

- deprecated feature;
- replacement;
- version of deprecation;
- planned removal information where applicable.

Deprecation MUST NOT silently alter semantics.

---

95. Error Recovery

Parser error recovery MAY continue parsing to report multiple diagnostics.

However, recovered syntax MUST NOT be treated as valid semantic input.

A compiler MUST NOT produce a successful executable artifact from an unresolved fatal semantic error.

---

96. Security Requirements

The compiler implementation MUST:

- use Rust 1.97 or Rust 1.97.1;
- use no "unsafe" Rust;
- avoid arbitrary privileged operations;
- isolate compile-time effects;
- protect secrets;
- validate external inputs;
- avoid executing untrusted generated code during compilation unless explicitly sandboxed;
- preserve security constraints through lowering.

The language specification itself MUST NOT require "unsafe" implementation techniques.

---

97. No Hidden Execution

Parsing and semantic analysis MUST NOT silently execute:

- user hardware;
- quantum devices;
- network services;
- arbitrary external programs;
- privileged operations.

Compilation MAY explicitly invoke external tooling where the compilation environment permits it and the operation is declared/configured.

---

98. Compiler Architecture

The repository implementation SHOULD separate:

grammar/
frontend/
semantic/
ir/
optimization/
routing/
scheduling/
hardware/
codegen/
runtime/

The exact repository directories MAY differ.

The dependency direction MUST remain acyclic.

---

99. Dependency Direction

The intended dependency direction is:

Grammar
   ↓
Frontend
   ↓
Semantic Analysis
   ↓
Canonical IR
   ↓
Optimization
   ↓
Routing / Scheduling / Lowering
   ↓
Target / Hardware
   ↓
Runtime

Not:

Grammar → Runtime → Grammar

and not:

Grammar → Hardware → Grammar

and not:

Grammar → Quantum IR → Grammar

---

100. Grammar Ownership Boundary

Grammar owns:

- syntax;
- lexical structure;
- syntactic declarations;
- syntactic annotations;
- syntactic extension points.

Grammar does NOT own:

- runtime state;
- hardware discovery;
- calibration;
- scheduling algorithms;
- routing algorithms;
- QEC algorithms;
- noise simulation;
- resilience policy;
- canonical IR;
- execution.

---

101. Integration With "semantic-model.md"

This document depends on:

grammar/specification/semantic-model.md

"semantic-model.md" defines what Zamani constructs mean.

This document defines how those meanings are transformed during compilation.

Neither document should duplicate the other's authority.

---

102. Integration With "syntax-model.md"

syntax-model.md

defines the structural syntax.

This document MUST NOT become the normative source for parser productions.

If a syntax rule changes, "syntax-model.md" and the authoritative grammar MUST be updated according to the grammar authority policy.

---

103. Integration With "execution-model.md"

Compilation ends at the executable/deployable artifact boundary.

Execution semantics begin when that artifact is instantiated.

The execution model defines:

- runtime state;
- dispatch;
- execution;
- resource acquisition;
- runtime failures;
- dynamic adaptation.

---

104. Integration With "scalability-model.md"

Scalability requirements defined there constrain compiler architecture.

In particular:

- no artificial finite machine limits;
- symbolic dimensions where appropriate;
- scalable collections;
- scalable IR;
- resource-driven realization.

---

105. Integration With "poco-reaf.md"

"poco-reaf.md" defines the long-term portability promise.

This document implements that promise through:

source semantics
→ portable representation
→ target realization

POCO-REAF MUST NOT be interpreted as a promise that every target can execute every program.

It means that the program's semantic definition is not rewritten merely because its realization changes.

---

106. Integration With "extensibility.md"

New compilation domains MUST be extensible through registered domain contracts.

A new domain SHOULD define:

- syntax;
- semantic constructs;
- domain IR;
- lowering;
- capabilities;
- target interfaces;
- verification;
- tests.

It MUST NOT duplicate universal concepts unnecessarily.

---

107. Integration With "reserved-space.md"

Reserved keywords, syntax forms, namespaces, and semantic extension points MUST be respected.

Compiler extensions MUST NOT consume reserved space without updating the language specification.

---

108. Integration With Quantum Subsystems

The compilation contract with the quantum subsystem is:

grammar/quantum/
        ↓
frontend
        ↓
semantic quantum model
        ↓
quantum::ir
        ↓
optimization
        ↓
routing
        ↓
scheduling
        ↓
ZQN-aware realization where requested
        ↓
hardware
        ↓
runtime

The compiler MUST NOT create an alternate quantum gate/qubit representation that competes with "quantum::ir".

---

109. Integration With QEC

Compiler output MAY contain QEC requirements.

The QEC subsystem determines how those requirements are realized.

The compiler MUST NOT embed:

- a specific QEC implementation;
- fixed code distance;
- fixed number of ancillas;
- fixed syndrome schedule;

unless such values are explicitly semantic requirements in the source.

---

110. Integration With ZQN

ZQN MAY provide:

- fault models;
- noise models;
- correlated-fault information;
- leakage;
- loss;
- erasure;
- execution conditions.

Compilation MAY use this information for adaptive or noise-aware realization.

The semantic source meaning remains independent of temporary noise conditions.

---

111. Integration With Scheduling

Scheduling consumes:

- canonical operations;
- dependencies;
- resource requirements;
- timing semantics;
- target capabilities.

The grammar provides declarations and semantic information but does not implement scheduling algorithms.

The existing stabilizer scheduler MUST remain an adapter/compatibility layer rather than becoming the grammar's scheduling authority.

---

112. Integration With Optimization

Optimization consumes canonical IR.

Existing optimization components that define duplicate temporary gate structures MUST migrate toward the canonical IR boundary.

The grammar MUST NOT depend directly on those temporary representations.

---

113. Integration With Hardware

Hardware compilation consumes:

- target descriptions;
- capabilities;
- constraints;
- topology;
- timing;
- resources.

The grammar MAY describe hardware explicitly through the HDL/hardware domains.

Portable programs SHOULD remain independent of concrete device identity unless explicitly target-specific.

---

114. Integration With Resilience

Resilience consumes execution and compiler metadata.

Compilation SHOULD expose:

- semantic identity;
- provenance;
- checkpoint boundaries;
- retry/recovery eligibility;
- verification requirements;
- resource requirements.

Resilience decides how to recover.

The compiler does not own recovery policy.

---

115. Integration With Runtime

The runtime receives a validated artifact.

The runtime MUST NOT need to reconstruct missing semantic information from arbitrary source text.

Artifacts SHOULD contain sufficient semantic metadata for execution and diagnostics.

---

116. Integration With Tooling

Compiler tooling MAY include:

- formatter;
- syntax highlighter;
- language server;
- documentation generator;
- static analyzer;
- dependency analyzer;
- semantic inspector;
- IR viewer;
- diagnostic renderer.

Tooling SHOULD consume the same authoritative grammar and semantic contracts.

Tooling MUST NOT become an alternate language authority.

---

117. Integration With Tests

Tests MUST be divided into:

lexer
parser
AST
semantic
type
effect
capability
resource
IR
optimization
quantum
HDL
hardware
hybrid
distributed
AI
data
networking
security
interoperability
dialects
macros
metaprogramming
scalability
determinism
roundtrip
compatibility

---

118. Compilation Test Requirements

Every compilation transformation MUST have:

Positive tests

Valid programs compile successfully.

Negative tests

Invalid programs fail with the correct category.

Boundary tests

Very small and very large representable inputs are tested.

Cross-domain tests

Multiple computation domains compile together.

Determinism tests

Equivalent compilation inputs produce deterministic results when deterministic compilation is requested.

Semantic preservation tests

Transformations preserve intended semantics.

---

119. Scalability Tests

Scalability tests MUST avoid assuming a fixed maximum.

Tests SHOULD generate parameterized cases covering increasing:

- qubit counts;
- array dimensions;
- tensor sizes;
- module counts;
- operation counts;
- nodes;
- resources;
- concurrent tasks.

The test infrastructure MAY impose practical test-run limits.

Those limits MUST be test harness limits, not language limits.

---

120. Hard-Coding Audit

Every compiler release SHOULD perform an automated search for:

MAX_
MAX_QUBITS
MAX_CORES
MAX_THREADS
MAX_DEVICES
MAX_NODES
MAX_MEMORY
FIXED_TOPOLOGY
DEVICE_ID
QUBIT_0
QUBIT_1

This is only an audit heuristic.

Equivalent hard-coding MUST also be detected even if different names are used.

---

121. Compilation Conformance

A compiler conforms to this model if:

1. It preserves semantic meaning.
2. It separates source semantics from target realization.
3. It supports canonical IR boundaries.
4. It does not impose accidental machine limits.
5. It preserves quantum semantic identity.
6. It distinguishes resource failure from semantic invalidity.
7. It supports target-specific lowering without contaminating portable semantics.
8. It supports versioned artifacts.
9. It preserves provenance.
10. It provides deterministic compilation where promised.
11. It preserves explicit nondeterminism.
12. It maintains acyclic architecture.
13. It supports extensibility.
14. It maintains security boundaries.
15. Its implementation uses safe Rust only.

---

122. File-Level Integration Contract

This document establishes the following integration contracts.

File / subsystem| Compilation responsibility
"grammar/Zamani.g4"| Authoritative syntax
"grammar/specification/syntax-model.md"| Syntax meaning
"grammar/specification/semantic-model.md"| Semantic meaning
"grammar/specification/compilation-model.md"| Compilation transformations
"grammar/specification/execution-model.md"| Runtime execution
"grammar/specification/scalability-model.md"| Scale guarantees
"grammar/specification/poco-reaf.md"| Long-term portability
"grammar/specification/extensibility.md"| Extension architecture
"grammar/types/"| Type syntax
"grammar/expressions/"| Expression syntax
"grammar/statements/"| Statement syntax
"grammar/quantum/"| Quantum syntax
"grammar/hdl/"| HDL syntax
"grammar/hardware/"| Hardware syntax
"grammar/resources/"| Resource syntax
"grammar/compile/"| Compilation-control syntax
"grammar/execution/"| Execution-control syntax
"grammar/interoperability/"| External language/system syntax
"src/quantum/ir/"| Canonical quantum semantic representation
"src/quantum/qec/"| QEC implementation
"src/quantum/zqn/"| Noise/fault semantics
"src/quantum/optimization/"| Quantum optimization
"src/quantum/scheduling/"| Quantum scheduling
hardware HAL| Hardware capability/state
resilience| Recovery/adaptation decisions
runtime| Execution

---

123. What This Document Does Not Own

This document does NOT define:

- exact lexer tokens;
- exact parser productions;
- complete AST implementation;
- QEC algorithms;
- noise algorithms;
- routing algorithms;
- scheduling algorithms;
- optimization algorithms;
- hardware discovery;
- calibration algorithms;
- runtime APIs;
- simulator internals;
- device drivers;
- vendor-specific instruction sets.

Those belong to their respective specifications and implementations.

---

124. Implementation Requirements

The compiler implementation MUST use:

Rust 1.97 or Rust 1.97.1

and MUST NOT use:

unsafe

Rust.

Compiler architecture SHOULD favor:

- ownership-safe data structures;
- immutable intermediate representations where practical;
- explicit error types;
- deterministic collections where semantic ordering matters;
- streaming/incremental processing where useful;
- bounded external effects;
- explicit resource accounting.

---

125. No Artificial Compiler Ceiling

Compiler APIs MUST avoid designs such as:

const MAX_QUBITS: usize = 64;
const MAX_NODES: usize = 1024;

when those constants represent language capacity rather than a genuine implementation requirement.

Where a compiler needs an implementation guard, it MUST:

1. identify the guard as an implementation limitation;
2. prevent it from becoming language semantics;
3. expose an appropriate diagnostic;
4. allow future implementations to increase or remove the limit.

---

126. Representation Scalability

IR and compiler data structures SHOULD prefer:

- dynamically sized collections;
- symbolic dimensions;
- parameterized resource models;
- sparse representations where appropriate;
- streaming transformations;
- incremental processing;
- lazy computation where beneficial.

Compiler architecture MUST NOT assume that the entire future execution machine fits into a fixed-size compile-time structure.

---

127. Resource-Aware Compilation

Resource-aware compilation SHOULD use:

Requirements
+
Constraints
+
Capabilities
+
Resources
+
Preferences
+
Optimization Policy

to determine a realization.

It MUST NOT rewrite the source semantics simply because resources differ.

---

128. Semantic Preservation Across Scale

The following should remain the same semantic program:

P

when realized on:

small target
large target
distributed target
heterogeneous target
quantum target
classical target
future target

provided the target satisfies the program's semantic requirements.

Different implementations MAY have different performance, precision, availability, latency, or failure characteristics where the language contract permits those differences.

---

129. Future-Proofing

The compiler architecture MUST avoid assuming that today's computing models are the final models.

Future target classes SHOULD be integrated by adding:

new dialect
new domain IR
new lowering
new target backend

rather than redesigning the language's foundational semantics.

---

130. Production Completion Criteria

"grammar/specification/compilation-model.md" is complete only when:

- compilation phases are explicitly defined;
- semantic and implementation concerns are separated;
- AST boundaries are defined;
- canonical IR ownership is defined;
- quantum compilation ownership is defined;
- QEC integration is defined;
- ZQN integration is defined;
- optimization integration is defined;
- routing integration is defined;
- scheduling integration is defined;
- hardware integration is defined;
- runtime boundaries are defined;
- resilience integration is defined;
- resource negotiation is defined;
- capability checking is defined;
- effect checking is defined;
- target specialization is defined;
- portability is defined;
- versioning is defined;
- provenance is defined;
- deterministic compilation is defined;
- nondeterminism is defined;
- scalability requirements are defined;
- hard-coding prohibitions are defined;
- security requirements are defined;
- interoperability is defined;
- dialect integration is defined;
- compile-time execution boundaries are defined;
- macro integration is defined;
- testing requirements are defined;
- failure categories are defined;
- no unresolved ownership conflict remains.

No later grammar file should need to redefine the fundamental compilation architecture established here.

---

131. Implementation Order

Compilation infrastructure SHOULD be implemented only after the following foundations are stable:

1. language-principles.md
2. language-scope.md
3. language-version.md
4. grammar-authority.md
5. syntax-model.md
6. semantic-model.md
7. scalability-model.md
8. compatibility.md
9. compilation-model.md
10. execution-model.md
11. poco-reaf.md
12. extensibility.md
13. reserved-space.md

After these contracts stabilize, implementation can proceed through:

lexer
↓
core
↓
types
↓
expressions
↓
statements
↓
declarations
↓
functions
↓
modules
↓
effects
↓
memory
↓
concurrency
↓
classical
↓
quantum
↓
hybrid
↓
HDL
↓
hardware
↓
distributed
↓
AI/data
↓
networking/security
↓
resources
↓
compile
↓
execution
↓
interoperability
↓
dialects
↓
macros
↓
metaprogramming
↓
validation
↓
integration tests

---

132. Final Compilation Principle

Zamani compilation is fundamentally a transformation of meaning into realizations.

The canonical model is:

                         ┌── Classical realization
                         │
                         ├── Quantum realization
                         │
                         ├── Hybrid realization
                         │
                         ├── HDL realization
                         │
                         ├── FPGA realization
                         │
                         ├── ASIC realization
                         │
                         ├── GPU realization
                         │
                         ├── Distributed realization
                         │
                         ├── Embedded realization
                         │
                         └── Future realization
                         ↑
                Target-specific lowering
                         ↑
                  Portable IR
                         ↑
                Semantic program
                         ↑
                   Zamani source

The fundamental invariant is:

«One source program defines one semantic computation. Compilation may produce many physical realizations, but changing the realization must not require rewriting the computation's meaning.»

Therefore:

Program Once
      ↓
Compile Once
      ↓
Portable Semantic Artifact
      ↓
Many Target Realizations
      ↓
Many Machines
      ↓
Many Scales
      ↓
Many Execution Environments
      ↓
Future Architectures

This is the compilation foundation of:

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever

and the governing architectural principle remains:

«Zamani describes computation and intent; compilation discovers how that computation can be realized by the resources actually available.»

The language defines no artificial machine ceiling.

The machine defines the resources available to a particular realization.

The compiler bridges the two without confusing them.