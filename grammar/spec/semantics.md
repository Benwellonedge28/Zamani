

Zamani Semantic Specification

Path: "grammar/spec/semantics.md"
Language: Zamani
Semantic specification version: 1.0
Status: Production semantic contract
Implementation baseline: Rust 1.97 / Rust 1.97.1
Implementation safety: "unsafe" Rust is prohibited
Primary principle: semantic intent is target-independent and resource-parametric

---

1. Purpose

This document defines the canonical semantic contract of the Zamani programming language.

It specifies what a syntactically valid Zamani program means independently of:

- CPU architecture;
- GPU architecture;
- QPU architecture;
- quantum gate set;
- qubit count;
- classical memory capacity;
- accelerator count;
- node count;
- operating system;
- simulator implementation;
- hardware topology;
- compiler optimization level;
- backend implementation;
- distributed execution topology;
- physical implementation technology;
- currently available resources.

The semantic model exists between the Zamani source/AST representation and target-independent intermediate representations.

The canonical compilation model is:

Zamani Source
     │
     ▼
Lexical Analysis
     │
     ▼
Parsing
     │
     ▼
AST
     │
     ▼
Name Resolution
     │
     ▼
Semantic Analysis
     │
     ├── Type checking
     ├── Effect checking
     ├── Resource checking
     ├── Capability checking
     ├── Ownership / alias analysis
     ├── Quantum validity
     ├── Temporal validity
     ├── Module resolution
     └── Constant / compile-time evaluation
     │
     ▼
Canonical Semantic IR
     │
     ▼
IR Verification
     │
     ▼
Target-independent Optimization
     │
     ▼
Target-independent Lowering
     │
     ▼
Target-specific Lowering
     │
     ├── CPU
     ├── GPU
     ├── accelerator
     ├── simulator
     ├── QPU
     ├── photonic system
     ├── distributed system
     └── other supported execution substrates
     │
     ▼
Execution

The semantic specification MUST NOT require any particular backend.

---

2. Normative language

The following terms are normative:

- MUST — mandatory.
- MUST NOT — forbidden.
- SHOULD — recommended unless a documented reason exists otherwise.
- SHOULD NOT — normally forbidden unless justified.
- MAY — permitted but optional.
- IMPLEMENTATION DEFINED — determined by the implementation but documented.
- RESOURCE DEPENDENT — dependent on capabilities available to the compilation or execution environment.
- UNDEFINED — Zamani MUST NOT use this category for ordinary language semantics.
- UNREPRESENTABLE — a valid source-level meaning cannot be represented by a particular target without an explicit transformation, rejection, or fallback.

Zamani MUST prefer explicit diagnostics over undefined behavior.

---

3. Authority hierarchy

Zamani previously contained multiple competing descriptions of the language.

That is not permitted in the production architecture.

The semantic authority hierarchy is:

1. This semantic specification
2. Canonical syntax specification
3. Canonical lexical specification
4. AST invariants
5. Semantic/type/effect/resource rules implemented by the compiler
6. Canonical semantic IR
7. IR verifier
8. Target-independent lowering rules
9. Target-specific backend rules
10. Historical/aspirational documentation

No backend may redefine source-language meaning.

No optimization may change observable semantics.

No hardware backend may require source programs to encode hardware-specific resource dimensions unless the source explicitly requests a target-dependent operation.

---

4. Separation of responsibilities

4.1 Lexer

The lexer determines:

«Which source characters form tokens?»

It MUST NOT determine program meaning.

---

4.2 Parser

The parser determines:

«Does the token sequence conform to Zamani syntax?»

It MUST NOT perform target-specific semantic decisions.

---

4.3 AST

The AST determines:

«What syntactic structure did the programmer write?»

Every AST node MUST preserve sufficient source information for:

- diagnostics;
- tooling;
- semantic analysis;
- transformations;
- source mapping;
- reproducibility.

AST nodes MUST NOT encode assumptions about a particular machine.

The current AST already attaches "Span" information to program constructs and represents expressions, statements, patterns, literals, and type expressions. The semantic specification treats that source structure as input to semantic analysis rather than as the final semantic representation.

---

4.4 Semantic analysis

Semantic analysis determines:

«Is the AST meaningful and valid under Zamani's language rules?»

It owns:

- name resolution;
- declaration validation;
- type checking;
- generic constraints;
- effect checking;
- ownership/resource checking;
- quantum validity;
- temporal validity;
- capability validation;
- constant evaluation;
- control-flow validity;
- module visibility;
- exhaustiveness;
- ambiguity detection.

---

4.5 Semantic IR

The canonical semantic IR determines:

«What computation does this program represent?»

The IR MUST be independent of a particular physical machine.

The existing compiler already exposes IR generation, verification, optimization, and backend consumers as distinct stages.

---

4.6 Optimizer

The optimizer MAY transform representation.

It MUST preserve:

- observable results;
- type correctness;
- effect semantics;
- resource semantics;
- quantum semantics;
- ordering constraints;
- ownership constraints;
- determinism guarantees;
- explicit nondeterminism;
- failure behavior where specified.

---

4.7 Backend

A backend determines:

«How can the semantic program be realized on this target?»

A backend MUST NOT alter Zamani source semantics.

A backend MAY:

- decompose operations;
- route operations;
- schedule operations;
- allocate resources;
- select equivalent implementations;
- use target-specific instructions;
- use acceleration;
- distribute computation;
- simulate unavailable hardware.

---

5. Core semantic principle: intent over implementation

Zamani is intended to support:

Program Once
Compile Once
Run Everywhere
Anywhere
Forever

abbreviated as:

POCO-REAF

This does not mean that every physical target can execute every program without limitation.

Instead, it means:

«A valid Zamani program expresses computational intent independently of a fixed physical resource configuration.»

Therefore source code MUST NOT require assumptions such as:

exactly 5 qubits
exactly 8 CPU cores
exactly 16 GB RAM
exactly 1 GPU
exactly 32 SIMD lanes
exactly 100 nodes
exactly topology X
exactly gate set Y

unless the programmer explicitly requests such a constraint.

---

6. Resource-parametric semantics

Every resource-consuming operation is interpreted relative to an abstract resource environment.

Conceptually:

ResourceEnvironment {
    classical_memory
    quantum_capacity
    concurrent_capacity
    storage_capacity
    communication_capacity
    accelerator_capacity
    timing_capacity
    precision_capacity
    energy_capacity
    domain_capabilities
}

These are semantic capabilities, not hard-coded language constants.

A program MUST remain semantically valid when the available resource environment changes, provided the required semantic capabilities remain satisfiable.

For example:

program P

may be compiled against:

R1 = 4 qubits
R2 = 128 qubits
R3 = 1,000,000 logical qubits

without changing the source program.

The compiler MAY select different implementations for each resource environment.

---

7. "Infinity" and scalability

Zamani uses "atom to everywhere" and "tiny to infinity" as scalability goals.

The language MUST interpret this as:

«No artificial finite machine-size constant is part of the language semantics.»

It does not claim that a physical machine has infinite resources.

Therefore:

semantic scalability = unbounded by language design
physical execution = bounded by available resources

A conforming implementation MUST be able to represent resource quantities without imposing arbitrary source-language limits where the implementation's integer representation, memory model, or target does not require one.

If an implementation cannot satisfy a resource requirement, it MUST report a resource/capability diagnostic rather than silently changing program meaning.

---

8. Determinism

Zamani distinguishes three categories.

8.1 Deterministic computation

Given identical:

- source;
- inputs;
- semantic environment;
- explicit configuration;
- external observations;

the result MUST be identical.

---

8.2 Explicit nondeterminism

Nondeterminism MUST be represented explicitly through language semantics, effects, concurrency, randomness, quantum measurement, external systems, or other declared mechanisms.

---

8.3 Unspecified implementation choices

Compiler decisions such as:

- register allocation;
- instruction selection;
- gate decomposition;
- scheduling;
- memory placement;
- parallelization;

MAY differ between targets provided that observable program semantics remain equivalent.

---

9. Undefined behavior

Zamani MUST NOT rely on C/C++-style undefined behavior as a normal semantic mechanism.

Programs encountering invalid semantic conditions MUST result in one of:

1. compile-time diagnostic;
2. explicitly specified runtime failure;
3. explicitly specified effect;
4. explicitly specified nondeterminism;
5. explicit resource exhaustion;
6. explicit target capability failure.

The compiler MUST NOT silently produce arbitrary behavior.

---

10. Implementation safety

The Zamani compiler and runtime implementation MUST use safe Rust.

The production implementation MUST NOT use:

unsafe

or unsafe blocks/functions.

This requirement applies to:

- lexer;
- parser;
- AST;
- semantic analysis;
- type checker;
- IR;
- optimizer;
- compiler;
- runtime;
- quantum infrastructure;
- backends;
- tooling;
- LSP;
- package tooling.

Foreign-function boundaries MUST be isolated behind safe abstractions and MUST NOT require unsafe Rust in the Zamani implementation.

---

11. Memory semantics

Zamani source semantics MUST NOT expose implementation-specific pointer behavior.

The semantic model distinguishes:

Value
Reference
Owned resource
Borrowed resource
Shared resource
Linear resource
Affine resource
External resource

A reference is a semantic relationship.

It is not necessarily a native machine pointer.

A backend MAY implement a reference as:

- pointer;
- handle;
- index;
- capability;
- register;
- table entry;
- remote reference;
- distributed identifier.

The representation is target-specific.

---

12. Ownership

Where ownership is applicable, every owned resource has one logical owner.

Ownership transfer MUST be explicit in semantic analysis.

An ownership transfer invalidates the previous owner unless the type semantics explicitly permit copying or sharing.

The compiler MUST reject:

- use-after-move;
- invalid ownership duplication;
- invalid destruction;
- invalid resource aliasing.

---

13. Copy semantics

A value MAY be copied only when its semantic type permits copying.

A copy MUST preserve the source value's observable semantics.

For large or resource-backed objects, the compiler MAY implement a semantic copy using:

- physical copying;
- reference counting;
- persistent structures;
- copy-on-write;
- handles;
- distributed replication.

The source semantics remain unchanged.

---

14. Linear and affine semantics

A "linear" value MUST be consumed exactly once.

An "affine" value MUST be consumed at most once.

The compiler MUST track these properties through:

- assignments;
- function calls;
- branches;
- loops;
- closures;
- pattern matching;
- async operations;
- quantum operations;
- effect handlers.

A branch MUST NOT cause a linear resource to be consumed inconsistently.

---

15. Type semantics

Zamani types describe semantic properties, not machine representations.

A type determines:

- valid operations;
- valid conversions;
- resource behavior;
- ownership behavior;
- effect compatibility;
- generic constraints;
- representation constraints where explicitly specified.

The following categories are first-class semantic concepts:

primitive
compound
generic
function
reference
pointer-like abstraction
optional
result
never
unit
quantum
linear
affine
temporal
dependent
effectful
resource-bearing
external

---

16. Type inference

Type inference MAY infer omitted types where the semantic rules make the result unique.

Inference MUST NOT:

- depend on backend hardware;
- depend on arbitrary compiler ordering;
- silently select incompatible types;
- create target-specific source semantics.

If inference is ambiguous, compilation MUST fail with a diagnostic.

---

17. Untyped parameters

Historical parser behavior permits parameters without explicit type annotations.

For production semantics, an omitted parameter type MUST NOT mean "unrestricted dynamic value" by accident.

Instead:

fn f(x) { ... }

means:

«infer the type of "x" from all available semantic constraints.»

If the compiler cannot infer a unique valid type, it MUST issue a diagnostic.

A future explicitly dynamic type MAY be introduced as a semantic type, but dynamic behavior MUST NOT be inferred merely from omission.

---

18. Generic semantics

Generic declarations describe computations parameterized over types, values, effects, or resources.

A generic definition MUST be semantically valid for every parameterization satisfying its declared constraints.

For example:

fn identity<T>(x: T) -> T {
    x
}

does not assume a particular representation of "T".

The compiler MAY specialize it.

Specialization MUST preserve semantics.

---

19. Dependent semantics

Dependent constructs such as:

Π
Σ
Identity

represent relationships between values and types.

Dependent expressions MUST be validated by semantic checking rather than by the parser.

The grammar recognizes syntax.

The semantic system determines:

- well-formedness;
- scope;
- dependency;
- equality;
- universe/level constraints where supported;
- termination requirements for compile-time evaluation.

---

20. Optional values

"Optional<T>" represents either:

Some(T)

or:

None

An absent value MUST NOT be treated as an arbitrary value of "T".

Optional propagation syntax, when supported, is semantically equivalent to explicit propagation.

---

21. Result values

"Result<T, E>" represents either:

Ok(T)

or:

Err(E)

Errors MUST NOT silently become ordinary values.

A backend MAY represent "Result" using any equivalent representation.

---

22. Never

"Never" represents a computation that cannot produce an ordinary value.

Examples include:

- guaranteed termination through failure;
- explicit panic;
- non-returning control flow;
- infinite computation where semantically represented as non-returning.

A "Never" expression MAY inhabit a position requiring any type because it does not produce a value.

---

23. Effects

Effects describe observable computational behavior beyond pure value transformation.

Examples include:

IO
state
time
randomness
network
filesystem
hardware
quantum
measurement
concurrency
external systems

An effect MUST be represented in semantic analysis and/or canonical IR.

An effect MUST NOT be hidden merely because a backend implements it differently.

---

24. Effect declarations

An effect declaration introduces a semantic capability.

For example:

effect Measurement;

does not itself perform measurement.

It declares an effect that can subsequently be required, performed, or handled.

---

25. Effect handlers

An effect handler determines how an effect is interpreted within a scope.

Handlers MUST preserve:

- effect identity;
- value flow;
- control-flow semantics;
- resource rules.

A backend MAY lower a handler into:

- function calls;
- state machines;
- continuation structures;
- runtime dispatch;
- static specialization.

---

26. "unsafe"

The token "unsafe" may remain reserved for compatibility and future language evolution.

However, under this production semantic specification:

«No ordinary Zamani program may obtain undefined or unchecked semantics through "unsafe".»

The compiler MUST reject unsupported unsafe semantics with an explicit diagnostic.

Future low-level capability features MUST be introduced as formally specified safe abstractions.

This keeps the implementation aligned with the requirement that the Zamani compiler itself contain no Rust "unsafe".

---

27. Exceptions and failure

Zamani distinguishes:

ordinary value
Result failure
effect failure
panic/non-returning failure
resource failure
capability failure
compile-time error

These MUST NOT be conflated.

A target may implement them differently, but semantic identity must remain stable.

---

28. Control flow

Every control-flow construct has a semantic successor relation.

The compiler MUST construct a valid control-flow representation before backend lowering.

This includes:

- "if";
- "while";
- "for";
- "loop";
- "match";
- "return";
- "break";
- "continue";
- exception/effect handling;
- asynchronous suspension.

No backend may infer missing control-flow semantics from syntax.

---

29. Loops

A loop is semantically a potentially repeated computation.

The compiler MUST NOT assume a fixed iteration count unless proven by semantic analysis.

Optimizations MAY:

- unroll;
- vectorize;
- parallelize;
- fuse;
- eliminate;
- transform;

provided observable semantics remain equivalent.

---

30. Pattern matching

Patterns are semantic destructuring specifications.

A match is correct only when:

1. every reachable case is valid;
2. bindings are valid;
3. binding types are compatible;
4. guards are valid;
5. exhaustiveness requirements are satisfied where applicable.

The compiler SHOULD detect unreachable patterns.

---

31. Modules

A module defines a semantic namespace.

Module resolution MUST be independent of the backend.

Imports and uses determine visibility and name resolution.

The semantic system MUST reject:

- unresolved modules;
- unresolved names;
- duplicate definitions where prohibited;
- illegal visibility;
- circular dependencies where unsupported.

Module paths MUST NOT encode machine-local assumptions into language semantics.

---

32. Compilation units

A Zamani program MAY consist of:

one source file
many source files
one module
many modules
one package
many packages
distributed compilation units

The semantic result MUST be independent of the physical organization of source files, provided the logical module graph is equivalent.

---

33. Names and identity

Identifiers are source-level names.

The semantic resolver assigns each declaration a unique internal identity.

The internal identity MUST NOT depend solely on textual spelling.

This prevents collisions caused by:

- scopes;
- modules;
- generic instantiation;
- generated declarations;
- macros;
- separate compilation.

---

34. Attributes

Attributes modify semantic metadata.

An attribute MUST have a formally defined owner and effect.

Unknown attributes MUST NOT silently change program meaning.

They MUST either:

- be explicitly accepted as inert metadata;
- be handled by a registered extension;
- or generate a diagnostic.

---

35. Compile-time evaluation

Compile-time evaluation is semantic computation performed before target execution.

Compile-time evaluation MUST be:

- deterministic unless explicitly declared otherwise;
- resource-bounded;
- side-effect controlled;
- reproducible where reproducibility is required.

The compiler MUST NOT allow arbitrary host-system effects merely because an expression occurs during compilation.

---

36. Macros and generated code

Generated syntax MUST enter the same semantic pipeline as ordinary source.

Generated code MUST NOT bypass:

- lexical validation;
- parsing;
- name resolution;
- type checking;
- effect checking;
- resource checking;
- IR verification.

Macro expansion MUST have deterministic source mapping.

---

37. Quantum semantics

Quantum computing is a semantic domain, not a fixed gate vocabulary.

The language MUST describe quantum intent.

The language MUST NOT define its ultimate quantum model as:

H
X
Y
Z
CNOT
T
S
SWAP
...

only.

Those may be operations available on some targets.

They are not the semantic ceiling of Zamani.

The semantic model instead represents quantum operations abstractly.

Conceptually:

QuantumOperation {
    operation_identity
    operands
    parameters
    preconditions
    postconditions
    effects
    resource_requirements
    measurement_behavior
}

The actual implementation MAY use a richer internal structure.

---

38. Quantum states

A quantum state is a semantic state.

It MUST NOT be represented in the language specification as a fixed-size array.

For example, the semantic meaning of an "n"-qubit state is not:

[f64; 2^n]

because that would impose a particular simulator representation.

A backend MAY use:

- state vectors;
- stabilizer representations;
- tensor networks;
- decision diagrams;
- matrix product states;
- trajectory methods;
- hardware-native states;
- symbolic representations;
- distributed representations.

---

39. Qubit identity

A qubit is a logical quantum resource.

Logical qubit identity MUST be independent of:

- physical qubit number;
- chip coordinates;
- hardware register;
- simulator array index.

For example:

q

is a logical resource.

A backend may map it to:

physical[7]

or:

physical[42]

or a distributed logical resource.

This mapping MUST NOT alter source semantics.

---

40. Quantum allocation

Quantum resources are acquired through semantic allocation.

The source program SHOULD express:

allocate quantum resources required by computation

rather than:

allocate physical qubit 17

unless an explicitly target-bound API is being used.

---

41. Quantum operation parameters

Quantum operations MAY be:

- named;
- parameterized;
- symbolic;
- composite;
- controlled;
- adjoint;
- inverse;
- conditional;
- dynamically selected.

The semantic system MUST validate parameter domains.

For example, a rotation parameter may have a real-valued semantic domain without forcing the backend to represent it as a particular floating-point width.

---

42. Operation decomposition

A semantic quantum operation may be decomposed into lower-level operations.

For example:

HighLevelOperation
       ↓
Target-independent decomposition
       ↓
Target operation set
       ↓
Hardware operation set

Decomposition MUST preserve semantic equivalence.

The compiler MUST NOT require source code to be rewritten merely because a target has a different native operation set.

---

43. Controlled operations

Controlled operations are semantic transformations.

The language MUST represent control relationships independently of hardware-specific controlled-gate instructions.

A backend MAY implement control through:

- native controlled operations;
- decomposition;
- conditional execution;
- pulse synthesis;
- classical feedback;
- simulation.

---

44. Adjoint and inverse semantics

Where an operation has an inverse, the inverse MUST be represented semantically.

The compiler MAY:

- use a native inverse;
- reverse a decomposition;
- synthesize another equivalent operation.

The resulting computation MUST remain equivalent.

---

45. Quantum measurement

Measurement is an observable effect.

It MAY introduce nondeterminism.

Measurement MUST NOT be treated as an ordinary deterministic pure function.

A measurement operation semantically produces an outcome according to the declared quantum measurement model.

The execution environment MUST preserve the specified probability distribution.

---

46. Quantum entanglement

Entanglement is a semantic relationship between quantum resources.

It MUST NOT require the source language to know physical coupling topology.

A backend is responsible for realizing entanglement using the available target capabilities.

If the target cannot realize the required semantic operation, compilation/execution MUST fail explicitly or select a valid supported transformation.

---

47. Quantum noise

Noise is part of the execution environment unless explicitly modeled by the program.

The semantic distinction is:

ideal semantic operation

versus:

physical execution noise

A noise model MAY be supplied to:

- simulation;
- verification;
- optimization;
- calibration;
- execution;
- benchmarking.

Noise MUST NOT silently redefine the ideal source-level operation.

---

48. Quantum error correction

Quantum error correction is a semantic/resource transformation.

A program specifies logical computation.

The QEC layer MAY transform:

logical qubits

into:

encoded physical resources

The source program MUST NOT need to know the physical code distance unless it explicitly requests a target-dependent configuration.

---

49. Surface-code semantics

Surface-code constructs MUST be interpreted as semantic error-correction/resource declarations.

They MUST NOT hard-code:

MAX_QUBITS
MAX_DISTANCE
MAX_LAYERS
MAX_TILES

into the language.

Any implementation limits belong to the resource/capability layer.

---

50. Quantum topology

Topology is a backend property.

The semantic layer MUST NOT require:

q0 connected to q1

as a universal source-language assumption.

Routing is a compilation problem.

The compiler MAY insert:

- swaps;
- teleportation;
- remapping;
- movement;
- decomposition;
- communication;
- other equivalent transformations.

---

51. Quantum scheduling

Scheduling is target/resource dependent.

The source semantic model describes dependencies and resource requirements.

The scheduler determines:

- ordering;
- concurrency;
- timing;
- resource allocation;
- synchronization.

No fixed maximum schedule length belongs in the grammar or semantic core.

---

52. Quantum precision

Precision is a semantic requirement.

A program MAY require:

exact
symbolic
bounded-error
approximate

semantics.

The backend determines how the requirement is realized.

A backend MUST reject an implementation whose numerical error violates an explicit source-level semantic requirement.

---

53. Quantum equivalence

Two quantum programs may be considered semantically equivalent when they produce equivalent observable behavior under the specified model.

Equivalence may consider:

- state transformation;
- measurement distributions;
- classical outputs;
- effects;
- resource obligations.

Compiler optimization MUST preserve the relevant equivalence relation.

---

54. Mathematics

Mathematical constructs are semantic abstractions.

A vector is not necessarily a contiguous machine array.

A matrix is not necessarily a dense matrix.

A tensor is not necessarily fully materialized.

A symbolic expression is not necessarily evaluated immediately.

Backends MAY select:

- dense;
- sparse;
- symbolic;
- distributed;
- accelerated;
- approximate;
- lazy;
- streamed representations.

---

55. Numeric semantics

Numeric operations MUST specify:

- domain;
- precision;
- overflow behavior;
- rounding;
- exceptional values;
- conversion rules.

The language MUST NOT silently depend on the host CPU's native integer or floating-point representation.

---

56. Integer semantics

Integer types represent mathematical integer domains subject to the declared type's range semantics.

If an operation cannot be represented by the selected integer type, the language MUST specify one of:

- compile-time rejection;
- checked failure;
- explicit wrapping;
- arbitrary-precision promotion;
- another explicitly declared behavior.

Silent target-dependent overflow is forbidden.

---

57. Floating-point semantics

Floating-point operations MUST specify whether they are:

- exact mathematical operations;
- IEEE-like operations;
- implementation-defined approximations;
- symbolic;
- arbitrary precision.

The source program MUST NOT accidentally change semantics because it moved from CPU to GPU or another target.

---

58. Concurrency

Concurrency is semantic independence between computations.

The compiler MAY map concurrent computations to:

- threads;
- tasks;
- processes;
- actors;
- SIMD lanes;
- GPU work items;
- distributed nodes;
- quantum/classical execution contexts.

The source program MUST NOT depend on the implementation's particular concurrency mechanism unless explicitly using a target-specific abstraction.

---

59. Data races

The semantic model MUST prevent undefined data races.

Mutable shared state requires an explicitly valid synchronization/ownership model.

The compiler MUST reject invalid conflicting access where the semantic rules require exclusivity.

---

60. Async and await

"async" describes a computation that may suspend.

"await" observes completion or progress of an asynchronous computation.

The semantic model does not require:

- OS threads;
- futures;
- event loops;
- kernel scheduling.

Those are implementation strategies.

---

61. Spawn

"spawn" creates an independently scheduled computation.

The language MUST specify:

- ownership transfer;
- lifetime;
- result observation;
- failure propagation;
- cancellation behavior.

A backend MAY implement it with any valid concurrency mechanism.

---

62. Distributed semantics

Distributed execution is an implementation of a single semantic program unless the source explicitly exposes distribution.

A compiler MAY partition computation across:

- processes;
- machines;
- regions;
- clusters;
- quantum processors;
- accelerators.

Communication costs are resource considerations unless the source program explicitly observes them.

---

63. Time and temporal semantics

"zamani" and "sasa" represent temporal semantic scopes.

They MUST NOT be reduced to arbitrary filesystem or wall-clock operations.

Temporal semantics MUST specify:

- temporal reference;
- observation point;
- ordering;
- persistence;
- reproducibility.

A physical clock is an execution capability.

---

64. External reality

Interactions with external systems are effects.

Examples:

hardware
network
filesystem
sensors
devices
human interaction
financial systems
external APIs

The semantic layer MUST represent such interactions explicitly.

No optimizer may remove an externally observable effect unless equivalence has been proven.

---

65. AI and cognitive semantics

AI/cognitive constructs are semantic computations.

They MUST NOT imply a particular model provider, neural architecture, accelerator, or inference engine.

A semantic AI operation may require capabilities such as:

model inference
learning
memory
reasoning
planning
generation
retrieval

The execution backend chooses how those capabilities are provided.

---

66. Nano and biological semantics

Nano/bionano constructs describe semantic operations or constraints.

They MUST NOT assume a particular physical fabrication technology.

Physical execution requires a target capability model.

If no compatible capability exists, the compiler MUST issue a capability diagnostic.

---

67. Sankofa semantics

"remember", "recall", "learn", "infer", "wisdom", "zamani", and related constructs represent semantic memory/knowledge operations.

They MUST be modeled as explicit computation/effects.

They MUST NOT implicitly access arbitrary host memory, files, networks, or external data.

Memory provenance SHOULD be preserved where the operation requires it.

---

68. Knowledge provenance

Knowledge-bearing values MAY carry provenance metadata.

Where provenance is semantically relevant, transformations MUST preserve it.

A backend MAY store provenance:

- inline;
- externally;
- through metadata;
- through content-addressed identifiers.

---

69. Security semantics

Security properties are semantic constraints.

The compiler MUST distinguish:

data
capability
authority
identity
credential
secret
public information

A secret MUST NOT become observable merely through optimization, logging, diagnostics, serialization, or debugging.

---

70. Capability semantics

Capabilities represent permission to perform an operation.

A capability MUST be explicitly available before an effect requiring it is executed.

The compiler SHOULD reject statically impossible capability requirements.

Runtime capability checks MAY be used when static proof is impossible.

---

71. Cryptographic semantics

Cryptographic primitives MUST specify semantic security and correctness requirements independently from implementation libraries.

A backend MAY use hardware acceleration or software implementations.

Changing the implementation MUST NOT silently weaken the specified security semantics.

---

72. Resource failures

Resource exhaustion is not undefined behavior.

Examples:

insufficient memory
insufficient quantum capacity
insufficient execution slots
insufficient precision
insufficient storage
insufficient communication capacity

must result in an explicit resource failure.

---

73. Resource negotiation

Compilation and execution MAY negotiate resources.

Conceptually:

Program Requirements
        │
        ▼
Capability Discovery
        │
        ▼
Resource Planning
        │
        ▼
Lowering
        │
        ▼
Execution

The source program remains unchanged.

---

74. Adaptive compilation

A compiler MAY generate different implementations for different resource environments.

For example:

small target
    → compact algorithm

medium target
    → parallel algorithm

large target
    → distributed algorithm

provided all implementations preserve the same semantic contract.

---

75. Specialization

Specialization MAY occur based on:

- known types;
- known values;
- resource capabilities;
- target features;
- proven invariants.

Specialization MUST NOT change observable source semantics.

---

76. Optimization

Optimizations are valid only if they preserve the canonical semantic model.

Examples:

constant folding
dead-code elimination
common-subexpression elimination
loop optimization
vectorization
parallelization
quantum gate cancellation
quantum decomposition
routing
scheduling
memory optimization
distributed partitioning

All such transformations are semantic-preserving transformations.

---

77. Canonical IR requirement

All semantic domains MUST lower into the canonical Zamani IR model rather than introducing isolated competing representations.

In particular:

quantum frontend
quantum optimizer
quantum scheduler
quantum hardware
quantum simulator
quantum benchmark

MUST share the same semantic quantum boundary.

They MUST NOT each invent incompatible "QuantumGate"/operation models.

The existing compiler architecture already uses "IrModule", "IrFunction", "IrInstruction", "IrRegister", "IrType", and "IrValue" across verification, optimization, backends, JIT, and WebAssembly components.

---

78. IR verification

Every semantic IR module MUST be verified before optimization or target lowering.

Verification MUST establish at least:

- valid control flow;
- valid register references;
- valid types;
- valid operand counts;
- valid ownership;
- valid resource relationships;
- valid quantum relationships;
- valid effect relationships;
- valid function signatures.

Invalid IR MUST NOT reach a production backend.

---

79. Semantic preservation

Every transformation:

AST → semantic representation
semantic representation → IR
IR → optimized IR
optimized IR → target IR
target IR → machine/runtime representation

MUST document its preservation invariant.

The compiler MUST be designed so that semantic bugs are detected at the earliest possible boundary.

---

80. Diagnostics

Semantic diagnostics MUST contain enough information to identify:

- source file;
- source span;
- primary problem;
- relevant semantic rule;
- expected condition;
- actual condition;
- actionable remediation where possible.

Diagnostics MUST NOT expose secrets.

Diagnostics SHOULD identify resource/capability failures distinctly from language errors.

---

81. Error categories

At minimum:

LexicalError
ParseError
NameError
TypeError
EffectError
OwnershipError
ResourceError
CapabilityError
QuantumError
TemporalError
ModuleError
ConstEvalError
MacroError
IrVerificationError
BackendCapabilityError

These categories may map into the repository's existing diagnostic architecture.

---

82. Recovery

Parser recovery MAY continue after syntax errors.

Semantic analysis MUST NOT pretend that a recovered/incomplete AST is valid.

IDE tooling MAY operate on partial semantic information.

Production compilation MUST fail when required semantic information is unavailable.

---

83. Versioning

Semantic changes MUST be versioned.

A language version determines:

- syntax;
- semantic rules;
- type behavior;
- effect rules;
- compatibility guarantees.

A compiler MUST NOT silently interpret old source under incompatible semantics.

---

84. Backward compatibility

Compatible language evolution SHOULD preserve:

source meaning

rather than merely preserving parser acceptance.

A previously valid program that changes meaning without an explicit language-version change is a semantic compatibility break.

---

85. Forward compatibility

Unknown future constructs MUST NOT be accidentally interpreted as existing constructs.

Reserved keywords MAY be introduced ahead of implementation.

Unimplemented reserved syntax MUST produce a clear diagnostic.

---

86. Grammar/implementation status

Every language feature MUST have an explicit lifecycle:

specified
parsed
represented
semantically validated
lowered
verified
optimized
executed
tested
stable

A grammar declaration MUST NOT imply that the complete feature is implemented.

This resolves discrepancies such as lexical token kinds existing before their complete lexer/parser/semantic implementation.

---

87. No semantic duplication

The following must have one semantic authority each:

types
effects
quantum operations
resource requirements
ownership
temporal behavior
module identity
diagnostics
IR operations

Documentation may describe them in multiple places, but definitions MUST NOT conflict.

---

88. Hardware abstraction

Hardware-specific information belongs below the semantic boundary.

For example:

logical qubit
    ↓
physical qubit mapping

abstract operation
    ↓
target operation

parallel computation
    ↓
thread/GPU/distributed implementation

abstract memory
    ↓
RAM/cache/device/remote storage

---

89. Explicit target binding

Zamani MAY provide target-specific APIs.

Such APIs MUST be explicitly identifiable as target-dependent.

For example, a program may deliberately request:

hardware-specific capability

but this must not contaminate ordinary target-independent semantics.

Target-bound code SHOULD be isolated behind explicit modules/capabilities.

---

90. POCO-REAF contract

A program qualifies for target-independent compilation when:

1. its semantics do not require unavailable target-specific capabilities;
2. all required operations have valid semantic representations;
3. all resource requirements can be satisfied or transformed;
4. all effects are supported;
5. all required precision guarantees are satisfiable.

The compiler MAY produce different machine-level implementations.

The source program remains unchanged.

---

91. Portability levels

Zamani implementations SHOULD classify programs as:

Level 0 — Pure semantic

No external resources.

Maximum portability.

Level 1 — Standard capabilities

Uses standardized Zamani capabilities.

Portable across conforming implementations supporting those capabilities.

Level 2 — Resource-parametric

Requires resources but not particular physical identities.

Example:

requires N logical qubits

Level 3 — Capability-specific

Requires a capability class.

Example:

requires fault-tolerant quantum execution

Level 4 — Target-specific

Requires a particular physical implementation.

Such code is intentionally non-universal.

---

92. Scalability invariant

No semantic rule may contain an arbitrary source-language ceiling such as:

MAX_QUBITS = 1024
MAX_NODES = 64
MAX_THREADS = 256
MAX_TENSOR_RANK = 32
MAX_MEMORY = ...

Limits belong to:

compiler configuration
runtime configuration
target capability
resource environment
security policy

not the semantic language definition.

---

93. Algorithmic scalability

The semantic model must remain valid for:

one value
one operation
one qubit
one processor
one device
one node
many nodes
many accelerators
many QPUs
distributed systems
heterogeneous systems

The implementation MAY introduce practical limits.

Those limits MUST be explicit and MUST NOT become language semantics.

---

94. Streaming and laziness

Large computations SHOULD support semantic representations that do not require full materialization.

For example:

stream
iterator
generator
lazy sequence
distributed collection
symbolic tensor

may represent logically enormous data without requiring equivalent physical memory.

---

95. Recursive structures

Recursive source structures MUST NOT impose arbitrary semantic depth limits.

Implementations MAY enforce resource limits to prevent exhaustion.

Such limits are implementation/resource limits, not language meaning.

---

96. Infinite computations

A program may semantically describe an unbounded computation.

Examples:

loop { ... }

or an infinite stream.

The compiler MUST NOT assume termination unless proven.

Resource exhaustion or cancellation must be handled according to runtime semantics.

---

97. Cancellation

Long-running or asynchronous computation SHOULD support explicit cancellation semantics.

Cancellation MUST define:

- whether destructors/finalizers run;
- whether effects are completed;
- ownership cleanup;
- child-task behavior;
- quantum-resource release;
- external-resource release.

---

98. Reproducibility

A reproducible build SHOULD be determined by:

source
language version
dependency versions
compiler version
semantic configuration
explicit compilation options

Target-specific decisions MAY differ when they do not alter semantic output.

---

99. Hashing and identity

Semantic artifacts MAY be content-addressed.

A semantic hash MUST be calculated over canonical representation rather than:

- memory addresses;
- backend-specific pointers;
- unordered maps with unstable ordering;
- timestamps unless explicitly included.

---

100. Serialization

Semantic representations SHOULD have canonical serialization.

Serialization MUST preserve semantic meaning.

A serialized AST or IR MUST NOT rely on Rust-specific memory layout.

---

101. ABI independence

The language semantics MUST NOT assume a single ABI.

ABIs belong to target lowering.

A semantic function:

fn f(x: T) -> U

has a source-level signature independent of whether a backend uses:

- registers;
- stack;
- message passing;
- RPC;
- GPU calling conventions;
- QPU invocation;
- distributed handles.

---

102. FFI

Foreign functions are effects/capabilities.

FFI declarations MUST describe the semantic contract.

The compiler MUST NOT assume that a foreign implementation is semantically safe merely because it exists.

---

103. Host independence

The compiler MUST NOT depend on:

- host endianness;
- host pointer width;
- host CPU instruction set;
- host quantum hardware;
- host filesystem layout;

for source-language meaning.

---

104. Resource-aware compilation

The compiler SHOULD expose a resource model conceptually similar to:

Requirements
    +
Capabilities
    +
Constraints
    =
Valid execution plan

This model is preferred over hard-coded machine-size branches.

---

105. Graceful degradation

If multiple semantically equivalent implementations exist, the compiler MAY choose the best implementation supported by available resources.

Example:

large memory:
    materialized algorithm

small memory:
    streaming algorithm

The result MUST remain semantically equivalent.

If no valid implementation exists, compilation MUST fail explicitly.

---

106. Quantum graceful degradation

Similarly:

native gate available
        ↓
use native gate

otherwise
        ↓
decompose

otherwise
        ↓
synthesize

otherwise
        ↓
alternative semantic realization

otherwise
        ↓
explicit capability failure

The compiler MUST NOT silently replace a quantum operation with a merely approximate operation unless approximation is permitted by its semantic contract.

---

107. Approximation

Approximate execution MUST be explicit.

An approximation contract SHOULD specify:

error bound
probability/confidence
norm/metric
precision
resource tradeoff

A backend MUST NOT silently weaken an exact computation into an approximation.

---

108. Measurement and classical interaction

Hybrid classical/quantum programs are valid.

The semantic model MUST support:

quantum computation
       ↓
measurement
       ↓
classical value
       ↓
classical control
       ↓
quantum operation

The boundary must be explicit in the semantic model.

---

109. Classical control of quantum computation

Classical conditions may select quantum operations.

Such conditions MUST respect:

- measurement dependencies;
- synchronization;
- effect ordering;
- resource ownership.

A compiler MAY transform classical/quantum control while preserving those dependencies.

---

110. Quantum/classical aliasing

Quantum resources MUST NOT be treated as ordinary copyable classical values.

The semantic system MUST explicitly represent:

- ownership;
- aliasing;
- measurement;
- consumption;
- entanglement;
- control relationships.

---

111. Quantum resource lifetime

Quantum resources have semantic lifetimes.

A logical qubit MUST NOT be silently reused while still semantically live.

Measurement, reset, deallocation, or transfer MUST establish the relevant state transition.

---

112. Quantum reset

Reset is a semantic operation.

It is not equivalent to merely assigning a classical variable.

A backend MAY implement reset through native reset, measurement plus conditional operation, dissipation, or another valid transformation.

---

113. Quantum scheduling equivalence

Reordering quantum operations is legal only when semantic dependencies permit it.

Two independent operations MAY be parallelized.

Entangled/shared-resource operations generally create dependencies that must be preserved.

The optimizer MUST use semantic dependencies rather than textual ordering alone.

---

114. Quantum routing

Routing is not source semantics.

A routing layer may transform:

logical q0, q1

into:

physical p3, p7

with additional operations.

The transformation MUST preserve logical behavior.

---

115. Hardware calibration

Calibration is target information.

It belongs to:

hardware
backend
execution environment

and MUST NOT redefine the source-level semantic identity of an operation.

---

116. Quantum benchmarking

Benchmarks measure implementations.

Benchmark results MUST NOT alter source semantics.

Benchmarking may inform:

- backend selection;
- decomposition selection;
- scheduling;
- calibration;
- resource planning.

---

117. Simulation

A simulator is a backend.

It MUST implement the same semantic contract as hardware execution for the subset it claims to support.

Simulation-specific behavior MUST be explicitly identified.

---

118. Formal semantic equivalence

Where practical, transformations SHOULD be validated by semantic equivalence tests.

At minimum, compiler testing SHOULD include:

source → AST
source → semantic model
semantic model → IR
IR → verified IR
IR → optimized IR
IR → backend

and cross-target equivalence tests.

---

119. Differential testing

The implementation SHOULD support differential testing between:

- interpreter/reference execution;
- simulator;
- optimized execution;
- backend execution;
- alternate backends.

For deterministic programs, outputs must agree.

For nondeterministic/quantum programs, distributions or specified equivalence properties must agree.

---

120. Property testing

Semantic invariants SHOULD be property-tested.

Examples:

optimization preserves result
routing preserves logical quantum behavior
serialization round-trips
type checking is deterministic
IR verification rejects malformed IR
resource scaling does not change source semantics

---

121. Fuzzing

The lexer/parser/semantic pipeline SHOULD be fuzz-tested for:

- crashes;
- hangs;
- stack exhaustion;
- malformed Unicode;
- pathological nesting;
- enormous literals;
- malformed quantum syntax;
- malformed types;
- malformed patterns.

A malformed program MUST result in diagnostics rather than memory unsafety or undefined compiler behavior.

---

122. Compiler resource limits

The compiler MAY enforce configurable limits such as:

maximum parser nesting
maximum diagnostic count
maximum macro expansion
maximum compile-time evaluation
maximum memory consumption
maximum compilation time

These are implementation safeguards.

They MUST NOT alter the language's semantic definition.

---

123. Stack safety

Production compiler components SHOULD avoid recursion where unbounded source nesting could cause host stack exhaustion.

Where recursion is semantically natural, implementations SHOULD use explicit work structures when necessary.

This is particularly important for:

- deeply nested expressions;
- types;
- generic types;
- patterns;
- quantum operation graphs;
- macro expansion;
- IR graphs.

---

124. Memory safety

The Rust implementation MUST use safe ownership and borrowing.

No compiler component may rely on:

- use-after-free;
- unchecked pointer arithmetic;
- aliased mutable memory;
- unsafe transmutation.

---

125. Thread safety

Compiler components intended for concurrent execution MUST use safe Rust synchronization and ownership primitives.

Semantic results SHOULD be immutable after publication.

---

126. Incremental compilation

Semantic analysis SHOULD support incremental operation where practical.

A source change SHOULD invalidate only the semantic regions affected by:

- changed declarations;
- dependency changes;
- type changes;
- effect changes;
- module changes.

Incremental compilation MUST preserve full-compilation semantics.

---

127. Separate compilation

Modules MAY be compiled separately.

A compiled module's semantic interface MUST include enough information to validate dependent modules without requiring target-specific implementation details.

---

128. Caching

Semantic and IR caches MUST be keyed by semantic inputs.

A cache MUST NOT incorrectly reuse an artifact when any semantic input has changed.

Possible inputs include:

source hash
language version
compiler semantic version
dependency interface hashes
feature configuration
explicit target constraints

---

129. Security boundary

Semantic analysis MUST occur before execution of generated code.

Generated or imported code MUST NOT bypass the normal validation pipeline.

---

130. Trust model

Imported code may be:

trusted
untrusted
partially trusted
sandboxed
capability-limited

The semantic model MUST preserve the declared trust boundary.

---

131. Package semantics

Packages provide modules and dependency interfaces.

Dependency resolution belongs to the build/package system.

The language semantic model consumes the resolved module graph.

Package manager behavior MUST NOT silently change language semantics.

---

132. Standard library

The standard library is a semantic capability provider.

A standard-library operation may be implemented differently across targets.

Its documented semantic contract must remain stable.

---

133. Backend contract

Every backend MUST declare:

supported semantic features
supported effects
resource constraints
precision guarantees
failure modes
target-specific limitations

A backend MUST reject unsupported semantic requirements explicitly.

---

134. Backend independence

Adding a new backend MUST NOT require changes to the meaning of existing source programs.

This is a core architectural invariant.

---

135. Compiler phases

A production compiler SHOULD implement the following conceptual phases:

lex
parse
AST validation
name resolution
module resolution
type checking
effect checking
resource checking
capability checking
semantic lowering
IR verification
optimization
target lowering
code generation
execution/package emission

The exact internal module layout may evolve without changing this semantic contract.

---

136. Canonical source-to-IR boundary

The semantic lowering boundary is the critical architectural seam:

AST
 ↓
Semantic Analysis
 ↓
Canonical Semantic IR

After this boundary, backends MUST NOT need to understand arbitrary source syntax.

---

137. AST-to-IR correctness

Every AST construct that is semantically accepted MUST either:

1. lower to a canonical IR representation; or
2. produce a formally specified compilation artifact.

A construct MUST NOT be accepted merely to become a comment, ignored node, or silently discarded semantic operation.

---

138. Unsupported constructs

If syntax exists but semantics are not implemented:

parse
  ↓
unsupported-feature diagnostic

is preferred over:

parse
  ↓
silently ignore

---

139. Comments and documentation

Comments have no runtime semantics unless explicitly defined as documentation metadata.

Compiler-generated comments in IR MUST NOT be used as a substitute for semantic operations.

---

140. Semantic annotations

Annotations may communicate:

- optimization hints;
- resource requirements;
- effects;
- capabilities;
- documentation;
- target preferences.

An annotation MUST NOT silently change ordinary language semantics unless its specification explicitly says so.

---

141. Target hints

A target hint is advisory unless declared as a hard requirement.

For example:

prefer GPU

does not mean:

must run only on GPU

unless explicitly specified.

---

142. Hard requirements

A program MAY declare hard semantic requirements.

If a target cannot satisfy them, compilation fails.

This is preferable to silently producing an invalid approximation.

---

143. Resource expressions

Future resource declarations SHOULD describe requirements symbolically.

For example:

requires quantum.capacity >= n

rather than:

requires QPU_MODEL_X

unless the program deliberately requires a physical target.

---

144. Resource scaling

Resource requirements SHOULD be expressed as functions of program parameters where possible.

For example:

quantum_capacity >= f(input_size)

rather than fixed constants.

This permits a single program to scale across resource environments.

---

145. Input-size independence

The compiler MUST NOT bake a particular input size into the semantic meaning of a general program.

For example:

sort(data)

does not semantically mean:

sort_exactly_1024_items

unless the program explicitly constrains its input.

---

146. Machine-size independence

Likewise:

parallel_for items

does not semantically mean:

run_on_exactly_8_threads

The runtime/compiler selects an appropriate implementation.

---

147. Heterogeneous execution

A single semantic program MAY contain:

classical computation
quantum computation
AI computation
GPU computation
distributed computation
external-device computation

provided the semantic interfaces are explicit.

The compiler determines valid partitioning.

---

148. Cross-domain composition

Domains such as:

- mathematics;
- quantum;
- AI;
- nano;
- temporal;
- distributed;
- hardware;

MUST compose through common semantic concepts:

types
values
effects
resources
capabilities
operations
control flow
data flow

They MUST NOT become independent mini-languages with incompatible semantics.

---

149. Domain extensions

Future language domains MAY be introduced.

Every extension MUST define:

1. syntax;
2. AST representation;
3. semantic rules;
4. type rules;
5. effects;
6. resource requirements;
7. canonical IR mapping;
8. verification rules;
9. optimization invariants;
10. backend contract;
11. tests.

---

150. No keyword-driven architecture

Adding a keyword does not constitute adding a semantic feature.

Every keyword must ultimately map to a well-defined semantic construct.

A keyword without semantic ownership MUST NOT be added merely because it appears in an aspirational grammar.

---

151. Current lexer/AST reconciliation

The repository currently contains a substantially broader token vocabulary than the implementation-oriented grammar alone describes, including quantum, noise, surface-code, omniversal, AI/system, and other domain tokens.

Therefore the canonical semantic process MUST classify every token/construct as:

implemented + semantically defined
implemented + awaiting semantic integration
reserved
deprecated
removed

A token MUST NOT become semantic merely because a lexer enum exists.

---

152. Parser/AST reconciliation

The parser currently dispatches constructs including quantum circuits, noise models, surface code, nano agents, Sankofa memory, effects, language declarations, omniversal constructs, and advanced declarations.

Every such construct MUST have a corresponding semantic rule.

If the construct cannot yet lower to canonical IR, it MUST produce an explicit unsupported-feature diagnostic rather than pretending to be production-ready.

---

153. AST semantic reconciliation

The current AST contains domain-specific nodes such as:

QuantumCircuit
NoiseModel
SurfaceCode
NanoAgent
SankofaMemory
EffectDeclaration
Omniversal*
ASI/AESI/ASESI

as well as generic type and expression structures.

These nodes are syntax/semantic input structures.

They MUST NOT become isolated backend APIs.

Their final meaning MUST flow through the canonical semantic/IR model.

---

154. IR consumers

The repository's optimizer, verifier, backend, JIT, and WebAssembly backend consume the IR model.

Therefore the semantic specification requires:

AST semantics
      ↓
canonical IR semantics
      ↓
all consumers

and forbids:

AST
 ├── quantum-specific IR
 ├── optimizer-specific quantum model
 ├── scheduler-specific quantum model
 ├── hardware-specific quantum model
 └── benchmark-specific quantum model

---

155. Verification-before-optimization invariant

The pipeline MUST be:

semantic lowering
       ↓
IR verification
       ↓
optimization

not:

semantic lowering
       ↓
optimization
       ↓
hope the result is valid

The repository already identifies IR verification as the stage between generation and optimization/backend processing.

---

156. Optimization correctness invariant

Every optimizer pass MUST satisfy:

ValidInput
    ⇒
ValidOutput

and:

Semantics(Input) == Semantics(Output)

subject to explicitly declared approximation or nondeterminism contracts.

---

157. Backend correctness invariant

Every backend MUST satisfy:

Semantics(Source)
==
Semantics(TargetExecution)

for the subset of semantics it claims to support.

---

158. Failure correctness

If equality cannot be established, the compiler MUST prefer:

diagnostic

over:

silent semantic degradation

---

159. Production conformance

An implementation is semantically production-ready only when:

- syntax is canonical;
- semantics are canonical;
- AST mappings are complete;
- unsupported constructs are explicit;
- type rules are deterministic;
- effects are tracked;
- resource requirements are explicit;
- quantum semantics are target-independent;
- IR is canonical;
- IR is verified;
- optimizations preserve semantics;
- backends declare capabilities;
- unsafe Rust is absent;
- tests cover semantic invariants.

---

160. Required semantic test matrix

The repository SHOULD maintain semantic tests covering at least:

core/
types/
generics/
ownership/
linear/
affine/
effects/
modules/
control_flow/
patterns/
async/
concurrency/
resources/
quantum/
quantum_measurement/
quantum_entanglement/
quantum_routing/
quantum_qec/
quantum_noise/
mathematics/
nano/
temporal/
memory/
AI/
distributed/
security/
IR/
optimization/
backend/
cross_target/

---

161. Cross-resource test

The same semantic program SHOULD be tested against multiple abstract resource environments.

For example:

tiny
small
medium
large
distributed
heterogeneous

The compiler may produce different implementations.

The semantic result must remain equivalent.

---

162. Cross-backend test

A portable program SHOULD be executed through multiple supported backends.

The test compares semantic results, not machine representation.

---

163. Cross-optimization test

For every optimization level:

-O0
-O1
-O2
-O3

or the implementation's equivalent levels, semantic results MUST agree.

---

164. Quantum cross-target test

A logical quantum program SHOULD be checked against:

ideal simulator
noise simulator
logical QEC simulator
hardware backend
alternate hardware backend

where supported.

Measurement distributions and specified observables must remain within the declared semantic contract.

---

165. Resource exhaustion test

The implementation MUST distinguish:

invalid program

from:

valid program requiring unavailable resources

The former is a language error.

The latter is a resource/capability error.

---

166. Deterministic diagnostic test

The same invalid source under the same semantic environment MUST produce equivalent diagnostics.

Diagnostic ordering SHOULD be deterministic.

---

167. No hidden limits test

Tests SHOULD deliberately exceed historical implementation assumptions.

Examples:

many declarations
deep generic structures
large quantum logical programs
large modules
large tensors
many concurrent operations
large IR graphs

When resources are insufficient, the implementation must fail gracefully rather than corrupting semantics.

---

168. Documentation synchronization

The following files MUST remain consistent:

grammar/spec/lexical.md
grammar/spec/syntax.md
grammar/spec/semantics.md
grammar/README.md
grammar/Zamani.g4
grammar/grammar.md
src/lexer.rs
src/parser.rs
src/ast/
semantic analysis
IR generation
IR verification
tests

However, they do not all have equal authority.

The semantic contract is defined here.

The syntax contract is defined by "syntax.md".

The lexical contract is defined by "lexical.md".

Implementation code must conform to those contracts.

---

169. ANTLR relationship

ANTLR grammar files may be used for:

- tooling;
- external parsers;
- validation;
- editor support;
- grammar testing.

ANTLR MUST NOT silently define a different language from the canonical Zamani syntax.

If the ANTLR grammar differs from the canonical parser:

canonical specification wins

and the divergence MUST be fixed.

---

170. Reference implementation relationship

The Rust lexer/parser are executable implementations.

They are not permitted to silently become a second language specification.

When implementation and specification disagree:

1. identify the intended semantic behavior;
2. update the canonical specification if the design is wrong;
3. update implementation;
4. add regression tests.

---

171. Historical documents

Historical documents such as broad aspirational grammar specifications may contain useful future concepts.

They MUST NOT be treated as accepted source semantics unless incorporated into the canonical specification.

This prevents aspirational constructs from accidentally becoming compiler promises.

---

172. Semantic feature lifecycle

A new feature is production-ready only after:

Design
 ↓
Syntax
 ↓
AST
 ↓
Semantic rules
 ↓
IR
 ↓
Verification
 ↓
Optimization
 ↓
Backend contract
 ↓
Tests
 ↓
Documentation
 ↓
Stable

---

173. Minimal implementation principle

A feature MUST NOT require every backend to implement every physical realization.

Instead:

semantic feature
       ↓
capability requirement
       ↓
backend support

This allows the language to evolve without making every backend universal.

---

174. Universal semantic substrate

Zamani's semantic model is therefore:

Values
Types
Operations
Effects
Resources
Capabilities
Control Flow
Data Flow
Temporal Relations

These are the fundamental semantic primitives.

Quantum computing, AI, mathematics, nano computation, distributed computation, and hardware interaction are domains built upon these primitives.

---

175. Atom-to-everywhere invariant

A Zamani program written for the smallest valid execution environment MUST retain the same semantic identity when executed on a larger environment.

Scaling MAY change:

- performance;
- parallelism;
- physical representation;
- scheduling;
- decomposition;
- distribution;
- resource allocation.

Scaling MUST NOT silently change:

- types;
- values;
- effects;
- logical quantum behavior;
- ownership;
- observable results.

---

176. Resource monotonicity

Where a larger resource environment provides a superset of required capabilities, the compiler SHOULD be able to preserve the same program semantics without requiring source changes.

Conceptually:

R_small ⊆ R_large

Program(P, R_small)
Program(P, R_large)

Semantics(P, R_small)
==
Semantics(P, R_large)

unless the program explicitly observes resource characteristics.

---

177. Explicit resource observation

A program MAY explicitly request information about its execution environment.

Such observation becomes part of observable semantics.

Therefore:

program that asks "how many processors exist?"

is intentionally resource-sensitive.

A program that does not ask this question remains resource-parametric.

---

178. Target-sensitive semantics

Target-specific behavior MUST be explicit.

A target-sensitive construct MUST identify:

- the target dependency;
- the required capability;
- whether fallback is permitted;
- whether portability is reduced.

---

179. Semantic purity

A pure function:

f(x) -> y

must depend only on its semantic inputs.

It MUST NOT implicitly read:

- wall-clock time;
- random state;
- hardware state;
- global mutable state;
- external resources.

Such dependencies require explicit effects.

---

180. Referential transparency

Pure expressions may be replaced by equivalent values.

This property permits aggressive optimization.

Effectful expressions cannot be freely reordered or eliminated.

---

181. Evaluation order

Zamani MUST define evaluation ordering wherever it affects observable behavior.

Pure expressions may be reordered subject to semantic equivalence.

Effectful expressions MUST preserve required ordering.

Quantum operations with dependencies MUST preserve their semantic dependency graph.

---

182. Side effects

Side effects include observable changes to:

- memory;
- files;
- network;
- devices;
- quantum state;
- external systems;
- shared state;
- temporal state.

Side effects MUST be represented in semantic analysis and/or IR.

---

183. Aliasing

Aliasing is permitted only when consistent with ownership and mutation rules.

The optimizer MUST account for aliasing before reordering or eliminating operations.

---

184. Constant folding

Constant folding is valid only when the compile-time result is semantically equivalent to runtime evaluation.

Operations involving runtime effects MUST NOT be folded unless their semantics explicitly permit compile-time evaluation.

---

185. Dead code elimination

Code may be eliminated only if it has no observable semantic effect.

Effectful operations MUST NOT be eliminated merely because their returned value is unused.

---

186. Common subexpression elimination

An expression may be reused only if:

- it is pure;
- its inputs are unchanged;
- reuse preserves resource semantics.

Quantum operations generally require special effect/resource analysis before duplication or elimination.

---

187. Parallelization

Parallelization is valid only when semantic dependencies permit it.

Two computations may run concurrently if their observable behavior is equivalent to the required sequential semantics.

---

188. Distribution

Distribution is valid only when communication and ordering semantics are preserved.

A distributed backend MUST account for:

- message ordering;
- failure;
- duplication;
- retries;
- ownership;
- external effects.

---

189. Fault tolerance

Fault tolerance is a runtime/backend property unless explicitly requested by source semantics.

A backend may add:

- replication;
- retries;
- error correction;
- checkpointing;
- redundancy.

These must preserve source semantics.

---

190. Persistence

Persistent state must have explicit lifetime semantics.

The compiler MUST NOT assume that all memory is ephemeral or persistent.

---

191. Serialization compatibility

Semantic serialization MUST define:

- version;
- schema;
- type identity;
- value representation;
- compatibility rules.

---

192. Semantic ABI

Zamani MAY define a canonical semantic ABI for interoperability.

Such an ABI is distinct from machine ABIs.

The semantic ABI describes:

- types;
- effects;
- ownership;
- resources;
- invocation semantics.

Machine ABI lowering remains target-specific.

---

193. Interoperability

Interoperability layers MUST translate between semantic contracts.

They MUST NOT bypass type/effect/resource checking.

---

194. Compiler implementation requirements

The Rust implementation targeting Rust 1.97/1.97.1 MUST:

- use safe Rust;
- avoid "unsafe";
- use checked conversions where required;
- avoid uncontrolled recursion for unbounded source structures;
- avoid panic-driven normal error handling;
- preserve source spans;
- use deterministic semantic ordering;
- expose explicit compiler errors.

The repository's "Cargo.toml" identifies Rust 1.97 as the intended baseline, so all implementation guidance in this specification assumes that baseline.

---

195. Panic policy

Compiler panics MUST be reserved for internal invariants that indicate a compiler defect.

User input MUST result in structured diagnostics.

Production compiler code SHOULD avoid "unwrap()"/"expect()" on user-controlled or externally fallible data.

---

196. Exhaustiveness

Semantic enums representing language concepts SHOULD be handled exhaustively.

When adding a semantic construct, the compiler should fail compilation of its own implementation until all required consumers have been updated.

This prevents silent semantic omission.

---

197. Feature gates

Experimental semantic features MAY be feature-gated.

A disabled feature MUST produce a clear diagnostic.

It MUST NOT be parsed and silently ignored.

---

198. Stable core

The following should form the stable semantic core:

values
types
functions
modules
control flow
effects
resources
capabilities
ownership
canonical IR
verification

Advanced domains build on this core.

---

199. Advanced domains

The following are extensions of the core semantic model:

quantum
mathematics
AI/cognitive
nano/bionano
temporal
distributed
HDL
hardware
metaprogramming
symbolic computation

They MUST NOT bypass the core semantic pipeline.

---

200. Final semantic contract

A conforming Zamani implementation MUST satisfy:

Source
  ↓
Canonical Syntax
  ↓
Canonical AST
  ↓
Canonical Semantics
  ↓
Canonical IR
  ↓
Verified IR
  ↓
Semantics-Preserving Transformations
  ↓
Target Lowering
  ↓
Execution

The defining invariants are:

No conflicting semantic authorities.

No machine-size constants in source semantics.

No hard-coded quantum hardware model.

No backend-specific meaning in the language core.

No silent unsupported features.

No undefined behavior as a normal language mechanism.

No unsafe Rust implementation.

No semantic changes caused merely by optimization.

No semantic changes caused merely by target selection.

No requirement to rewrite source for larger or smaller machines.

No requirement to know physical topology at source level.

No fixed qubit ceiling.

No fixed node ceiling.

No fixed memory ceiling.

No fixed accelerator ceiling.

No fixed tensor-size ceiling.

No fixed program-size ceiling in the language specification.

The fundamental Zamani invariant is:

PROGRAM
   │
   │ expresses intent
   ▼
SEMANTICS
   │
   │ independent of physical realization
   ▼
CANONICAL IR
   │
   │ resource/capability aware
   ▼
TARGET IMPLEMENTATION

Therefore:

«Zamani programs are written against semantic capabilities and computational intent, not against the dimensions of today's machine.»

A valid program may scale from the smallest available execution environment to substantially larger heterogeneous environments without changing its source-level meaning.

The physical implementation may change.

The resource allocation may change.

The decomposition may change.

The scheduling may change.

The topology may change.

The representation may change.

The backend may change.

The semantic program does not.

This is the semantic foundation required for:

Program Once → Compile Once → Run Everywhere / Anywhere / Forever (POCO-REAF)

subject only to the explicit capabilities, resources, effects, precision requirements, and target constraints declared or required by the program.This is intentionally written as the semantic contract, rather than duplicating the syntax grammar. The key repository integration point is that src/parser.rs currently accepts a substantially broader set of constructs than a simple core language, while src/ast/mod.rs already has corresponding domain nodes; those constructs now need to converge through one semantic model rather than independently acquiring backend semantics.

