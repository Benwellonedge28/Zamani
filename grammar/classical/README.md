Zamani Classical Grammar

Path: "grammar/classical/README.md"
Domain: Classical computation
Language: Zamani
Grammar technology: ANTLR4
Implementation baseline: Rust 1.97 / Rust 1.97.1, Rust 2021
Safety: "unsafe" Rust is forbidden
Primary objective: Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)

---

1. Status

This document is the normative architecture and integration contract for the classical grammar domain.

It defines what belongs under:

grammar/classical/

and how those grammar components integrate with the rest of Zamani.

This document does not claim that every downstream classical capability is already implemented.

A grammar construct is production-ready only when:

1. its syntax is defined;
2. its AST representation exists or is explicitly scheduled;
3. semantic analysis defines its meaning;
4. its canonical IR lowering is defined;
5. diagnostics are defined;
6. compatibility behavior is defined;
7. positive, negative, boundary, and scalability tests exist;
8. downstream consumers have an explicit integration contract.

The repository currently contains a broad canonical "Zamani.g4", a separate implementation-oriented "grammar/grammar.md", a broad design/specification document, and a classical grammar composition directory. These surfaces must ultimately converge on one language definition rather than becoming competing languages. The repository's grammar design establishes "Zamani.g4" as the canonical ANTLR representation after reconciliation, while "grammar/grammar.md" serves as an implementation-conformance reference and "Zamani-Grammar.md" remains design/historical material unless features are formally promoted.

The classical grammar must follow that authority model.

---

2. Mission

Zamani is intended to describe computation independently of the machine on which that computation eventually executes.

The classical domain therefore provides syntax for expressing classical computation without embedding accidental hardware limitations.

The governing principle is:

«Zamani describes computation, intent, capabilities, constraints, and semantics—not arbitrary limitations of the machine currently available.»

Classical computation must be able to participate in:

- ordinary sequential computation;
- systems programming;
- numerical computing;
- scientific computing;
- vector and matrix computation;
- tensor computation;
- symbolic computation;
- parallel computation;
- distributed computation;
- AI/ML;
- accelerators;
- embedded systems;
- hardware/software co-design;
- quantum-classical programs;
- HDL integration;
- future computational domains.

The classical grammar is therefore a domain grammar, not a separate programming language.

---

3. POCO-REAF

The classical grammar participates in:

Program
   ↓
Compile Once
   ↓
Architecture-independent semantic representation
   ↓
Target realization
   ↓
Run Everywhere
   ↓
Run Anywhere
   ↓
Future-compatible execution

The source program must describe the computation rather than prematurely selecting a physical machine.

The same semantic program may eventually be realized on:

- a tiny embedded processor;
- a single CPU;
- a multicore CPU;
- a vector processor;
- a GPU;
- an FPGA;
- an ASIC;
- an accelerator;
- a heterogeneous system;
- a cluster;
- a supercomputer;
- a distributed system;
- an edge system;
- a cloud system;
- a simulator;
- a future architecture.

Changing the available machine must not require rewriting the algorithm merely because the machine has a different number of:

- cores;
- threads;
- devices;
- registers;
- vector lanes;
- memory capacity;
- nodes;
- accelerators;
- cache levels;
- processing elements.

Those are target/resource properties.

---

4. Absolute Scalability Rule

The classical grammar MUST NOT contain arbitrary machine-size limits.

It must not contain grammar-level concepts such as:

MAX_ELEMENTS
MAX_VECTOR_LENGTH
MAX_MATRIX_ROWS
MAX_MATRIX_COLUMNS
MAX_TENSOR_RANK
MAX_THREADS
MAX_CORES
MAX_GPUS
MAX_DEVICES
MAX_MEMORY
MAX_NODES
MAX_WORKERS
MAX_ACCELERATORS

nor hidden equivalents.

ANTLR repetition must be structural:

item*
item+
item?

rather than artificially bounded:

item{1,32}

unless a finite cardinality is genuinely part of the language semantics.

A mathematical or algorithmic bound explicitly written by the programmer is different from a compiler-imposed limit.

For example:

vector[n]

may express a program-level dimension.

It must not mean:

n <= compiler_maximum

unless the language specification explicitly defines such a semantic restriction.

Implementation resource exhaustion is not language semantics.

---

5. Safety Requirements

The grammar itself must contain:

- no embedded Rust actions;
- no target-specific parser code;
- no machine-dependent semantic predicates;
- no unsafe Rust;
- no assumptions about pointer width;
- no assumptions about integer width unless the type semantics require them;
- no physical addresses;
- no device IDs;
- no hard-coded resource capacities.

The Rust compiler/frontend implementation consuming the grammar must use safe Rust only.

The repository's grammar design explicitly requires Rust 1.97/1.97.1, Rust 2021, deterministic parsing, structured diagnostics, source spans, and no Rust "unsafe".

---

6. Architectural Ownership

6.1 "grammar/classical/" owns

This directory owns classical-domain source syntax that genuinely requires a classical grammar boundary.

It may own:

- classical domain declarations;
- classical computation regions, if the language defines explicit regions;
- classical-domain modifiers;
- classical computation annotations;
- classical-domain-specific constructs;
- mathematical-domain declarations where they are language syntax rather than ordinary library calls;
- vector declarations, if vector declaration syntax is intentionally language-level;
- matrix declarations, if matrix declaration syntax is intentionally language-level;
- tensor declarations, if tensor declaration syntax is intentionally language-level;
- classical accelerator intent syntax, if explicitly standardized as language syntax;
- classical grammar composition;
- classical grammar documentation;
- classical grammar tests.

6.2 It does NOT own

It does not own:

- the lexer;
- identifiers;
- keywords;
- literals;
- comments;
- Unicode rules;
- general expressions;
- operators;
- operator precedence;
- general types;
- function syntax;
- general declarations;
- modules;
- imports;
- general statements;
- ownership;
- borrowing;
- lifetimes;
- concurrency semantics;
- scheduling;
- optimization;
- hardware discovery;
- hardware topology;
- resource discovery;
- backend selection;
- runtime execution;
- classical algorithms;
- classical standard-library implementations;
- classical IR;
- quantum IR;
- QEC;
- ZQN;
- calibration;
- routing;
- target lowering.

Those responsibilities belong to other repository components.

---

7. Canonical Boundary

The architecture is:

Zamani source
      ↓
Canonical lexer
      ↓
Canonical parser
      ↓
Frontend AST
      ↓
Name/module resolution
      ↓
Semantic analysis
      ↓
Type analysis
      ↓
Effect analysis
      ↓
Capability/resource analysis
      ↓
Canonical semantic IR
      ↓
Optimization
      ↓
Scheduling / routing / resilience / target lowering
      ↓
Runtime / backend

The current repository's architecture explicitly establishes this separation and identifies the frontend, AST, semantic analysis, IR generation, optimization, scheduling, routing, ZQN, and hardware layers as separate concerns.

The classical grammar must remain upstream of those systems.

---

8. Critical Correction: Do Not Create Fake Grammar Boundaries

A production grammar must not be filled with rules that merely alias:

classicalExpression : expression ;
classicalValue      : expression ;
classicalCall       : expression ;
classicalAssignment : expression ;
classicalNumeric    : expression ;

unless those rules correspond to an actual parser boundary required by the canonical grammar.

Such aliases do not create semantic typing.

For example:

x + y

does not become numerical merely because it occurs under "classicalExpression".

Its meaning is determined by:

- name resolution;
- type inference;
- type checking;
- overload resolution;
- effect analysis;
- domain analysis;
- semantic lowering.

Therefore the production architecture should prefer:

shared syntax
     ↓
semantic classification

over creating redundant grammar wrappers.

This avoids duplicate syntax and prevents the grammar from becoming a pseudo-type-system.

---

9. Required Classical Grammar Files

The intended classical directory is:

grammar/classical/
├── README.md
├── classical.g4
├── scalar.g4
├── vector.g4
├── matrix.g4
├── tensor.g4
├── numerical.g4
├── symbolic.g4
└── classical-accelerators.g4

These files are justified as follows.

File| Status| Responsibility
"README.md"| Keep/rewrite| Normative classical-domain architecture
"classical.g4"| Keep/rewrite| Classical grammar composition boundary
"scalar.g4"| Keep/rewrite| Genuine scalar-specific syntax, if required
"vector.g4"| Keep/rewrite| Genuine vector-specific syntax
"matrix.g4"| Keep/rewrite| Genuine matrix-specific syntax
"tensor.g4"| Keep/rewrite| Genuine tensor-specific syntax
"numerical.g4"| Keep/rewrite| Genuine numerical-domain syntax
"symbolic.g4"| Keep/rewrite| Genuine symbolic-domain syntax
"classical-accelerators.g4"| Keep/rewrite| Portable accelerator intent syntax

No additional file should be created merely because the architecture diagram has a category.

A file must exist only when it owns a coherent grammar responsibility.

---

10. "classical.g4"

Purpose

"classical.g4" is the composition root for classical-domain grammar.

It must not become a second general Zamani grammar.

Owns

- classical domain entry points;
- composition of classical subgrammars;
- explicit classical-region syntax, if standardized;
- references to shared canonical grammar rules;
- domain-level grammar organization.

Does not own

- expressions;
- operators;
- types;
- statements;
- declarations;
- identifiers;
- literals;
- lexical tokens;
- mathematical algorithms.

Inputs

Canonical lexer tokens and canonical shared grammar rules.

Outputs

Classical parser structures consumed by the frontend AST construction layer.

Dependencies

Conceptually:

lexer
core
types
expressions
statements
declarations

plus the classical subgrammars.

Upstream contract

The canonical lexer and shared grammar own token spelling and general syntax.

Downstream consumers

- parser;
- AST construction;
- semantic analysis;
- classical semantic lowering.

IR contract

"classical.g4" does not produce IR.

It only provides syntax.

Compiler contract

The compiler interprets the resulting AST semantically.

Runtime contract

No direct runtime dependency.

Completion criteria

Complete when:

- all classical entry points are defined;
- no duplicated general syntax exists;
- no circular grammar dependency exists;
- ANTLR generation succeeds;
- canonical parser integration succeeds;
- all classical integration tests pass.

---

11. "scalar.g4"

Purpose

Defines scalar-specific syntax only where scalar computation requires syntax that cannot be expressed using shared expressions and types.

Owns

Potential scalar-specific constructs such as:

- explicit scalar-domain declarations;
- scalar-domain annotations;
- semantic scalar literals if a distinct syntax is formally required.

Does not own

- integer literal lexical syntax;
- floating-point literal lexical syntax;
- boolean literal syntax;
- arithmetic operators;
- comparison operators;
- general variable declarations;
- general types.

Those belong to shared grammar components.

Scalability

Scalar syntax must not assume:

32-bit
64-bit
128-bit

as a universal machine representation.

If Zamani defines semantic widths, those widths must be explicit language types.

Target widths belong to lowering.

Integration

scalar syntax
     ↓
shared type system
     ↓
semantic scalar type
     ↓
classical IR
     ↓
target lowering

Tests

Must include:

- minimal scalar program;
- every supported scalar form;
- invalid scalar form;
- boundary numeric values;
- arbitrary literal magnitude handling;
- type mismatch;
- cross-domain use;
- quantum-measurement-to-scalar use.

---

12. "vector.g4"

Purpose

Defines vector-specific syntax where vector semantics justify dedicated syntax.

Owns

- vector declarations;
- vector-domain syntax;
- vector-specific shape syntax;
- vector construction syntax where standardized.

Does not own

- array syntax;
- indexing;
- generic expressions;
- generic type syntax;
- numeric literals.

Those are shared.

Scalability

Never encode:

MAX_VECTOR_LENGTH

or any fixed vector length.

The grammar must support:

vector<T>
vector<T, N>
vector<T, n>
vector<T, runtime_shape>

only if each form has a defined semantic meaning.

Dimensions may be:

- constants;
- symbolic values;
- generic parameters;
- runtime values;
- dependent values where the type system supports them.

Semantic distinction

A vector dimension is not automatically a hardware allocation.

The distinction is:

program shape
≠
memory capacity
≠
hardware vector width

Integration

vector syntax
      ↓
type/shape analysis
      ↓
classical semantic representation
      ↓
classical IR
      ↓
optimization
      ↓
target lowering

---

13. "matrix.g4"

Purpose

Defines matrix-specific language syntax where matrix operations are language-level constructs.

Owns

- matrix declarations;
- matrix-specific shape expressions;
- matrix-domain declarations;
- standardized matrix literals, if needed.

Does not own

- generic indexing;
- arithmetic;
- multiplication operators;
- generic collection syntax;
- BLAS implementation selection;
- GPU selection.

Required semantic model

Matrix dimensions must be semantic values.

They may be:

- statically known;
- symbolic;
- generic;
- runtime-determined.

No grammar-level:

MAX_ROWS
MAX_COLUMNS

is permitted.

Integration

matrix syntax
      ↓
shape/type analysis
      ↓
semantic matrix operation
      ↓
classical IR
      ↓
optimization
      ↓
CPU/GPU/accelerator lowering

The grammar must not select a particular matrix library.

---

14. "tensor.g4"

Purpose

Defines tensor-specific syntax that is genuinely language-level.

Owns

- tensor declarations;
- tensor shape syntax;
- tensor-specific structural syntax;
- explicit tensor-domain constructs.

Does not own

- generic arrays;
- generic indexing;
- generic iteration;
- accelerator selection;
- memory layout implementation;
- SIMD width.

Scalability

Tensor rank must not be capped by an arbitrary grammar constant.

The grammar must permit structurally represented rank/shape information.

The implementation may have resource limits, but those are:

implementation/resource constraints

not:

language grammar restrictions

Semantic separation

The following are distinct:

tensor shape
tensor rank
tensor storage layout
tensor memory allocation
tensor execution device
tensor parallelization strategy

The grammar must not collapse them into one concept.

---

15. "numerical.g4"

Purpose

Provides syntax for numerical computation only where numerical intent is a language-level construct.

Candidate domains

- numerical transformations;
- numerical integration;
- differentiation;
- interpolation;
- numerical solving;
- Fourier transforms;
- signal-oriented numerical operations;
- numerical analysis directives.

Critical rule

The grammar must not become a catalog of every mathematical function.

For example, operations such as:

sin
cos
exp
log
sqrt

are normally better represented as typed functions/intrinsics/library operations rather than dedicated grammar productions.

Likewise, the presence of:

fft

does not automatically require a special keyword if a typed standard operation can express it.

Ownership test

A numerical operation belongs in grammar only if:

1. it has language-level syntax;
2. it has standardized language semantics;
3. ordinary function/intrinsic syntax is insufficient;
4. it must be recognized structurally by compilation;
5. its semantics are stable enough to be part of the language.

Otherwise it belongs in the standard library or intrinsic system.

Backend independence

The grammar must not select:

- BLAS;
- LAPACK;
- MKL;
- CUDA;
- ROCm;
- a CPU ISA;
- a GPU;
- SIMD width;
- a numerical vendor.

---

16. "symbolic.g4"

Purpose

Defines genuine symbolic-computation syntax.

Potential constructs

Where standardized as language features:

- symbolic declaration;
- symbolic binding;
- symbolic transformation;
- symbolic substitution;
- symbolic evaluation boundaries;
- compile-time symbolic computation.

Does not own

- symbolic mathematics algorithms;
- CAS implementation;
- simplification algorithms;
- theorem provers;
- specific symbolic libraries.

Semantic model

source symbolic expression
        ↓
AST
        ↓
symbolic semantic analysis
        ↓
canonical representation
        ↓
symbolic optimizer / evaluator

The grammar must not assume one symbolic implementation.

---

17. "classical-accelerators.g4"

Purpose

Defines portable syntax for expressing classical accelerator intent, when such intent is part of the language.

Correct distinction

The following are different concepts:

requires acceleration

prefers accelerator capability

requires capability(X)

target(device)

The first three can be portable program intent.

A physical device selection is deployment/target information.

Never encode

GPU 0
GPU 1
CPU 0
FPGA 3
device 42

as universal classical semantics.

Accelerator abstraction

A program may express:

requires capability("parallel-compute")

or an equivalent standardized capability expression.

The actual realization may be:

CPU
GPU
FPGA
ASIC
accelerator
future device

depending on downstream capability matching.

Integration

accelerator intent
       ↓
capability analysis
       ↓
resource analysis
       ↓
optimization
       ↓
scheduling
       ↓
target lowering
       ↓
runtime dispatch

---

18. Mathematical Operations Must Not Become Grammar Explosion

The current root "Zamani.g4" contains extensive direct mathematical syntax, including vector, matrix, tensor, symbolic, calculus, statistical, numerical, signal-processing, and optimization operations.

That capability must be preserved deliberately, but production architecture must distinguish:

language syntax

from:

standard library API

and:

compiler intrinsic

and:

optimization

For example:

matrix.inverse(A)

can be a normal typed operation.

A dedicated keyword:

inverse A;

should exist only if the language specification requires special syntax or compiler semantics.

This prevents the grammar from growing indefinitely every time a new mathematical operation is added.

---

19. Classical Types

The classical grammar consumes the canonical type system.

It must not independently redefine:

- integer types;
- floating-point types;
- boolean types;
- arrays;
- tuples;
- maps;
- references;
- generic types;
- function types;
- resource types.

The repository's current grammar design already defines types as a shared subsystem and expects classical constructs to consume canonical type expressions.

Therefore:

classical/
      ↓
types/

is a dependency.

Not:

classical/
      ↓
second type system

---

20. Expressions

All ordinary expressions belong to the shared expression grammar.

Classical programs must naturally support:

- literals;
- identifiers;
- arithmetic;
- comparison;
- logical operations;
- bitwise operations;
- calls;
- indexing;
- member access;
- ranges;
- lambdas;
- comprehensions;
- conditional expressions;
- compile-time expressions.

The existing implementation-oriented grammar already defines a general precedence hierarchy for these operations.

The classical grammar must consume that hierarchy rather than copy it.

---

21. Statements

Classical computation consumes shared statements.

Examples:

let
const
if
while
for
match
return
break
continue

Concurrency statements belong to the concurrency subsystem.

Quantum statements belong to the quantum subsystem.

HDL statements belong to the HDL subsystem.

This prevents:

classical.g4

from becoming a second universal parser.

---

22. Functions

Classical functions use the canonical function grammar.

The classical grammar must not redefine:

- parameters;
- return types;
- generic parameters;
- function modifiers;
- function bodies;
- closures.

A classical function is simply a function whose semantic body/types/effects resolve to classical computation.

The parser should not need:

classicalFn

when:

fn

already represents the language-level concept.

---

23. Memory Integration

Classical computation may interact with:

ownership
borrowing
lifetimes
allocation
deallocation
shared memory
distributed memory
resource constraints

but these are owned by "grammar/memory/" and semantic analysis.

Classical grammar must not encode:

RAM = X
cache = Y
pointer_width = Z

as universal properties.

The difference is:

memory semantics

versus:

machine memory capacity

The former may be language semantics.

The latter belongs to the target/resource environment.

---

24. Concurrency Integration

Classical code must be capable of running concurrently.

Concurrency belongs to:

grammar/concurrency/

not classical grammar.

Supported integration includes:

- tasks;
- futures;
- actors;
- channels;
- synchronization;
- parallel loops;
- data parallelism;
- task parallelism;
- cancellation.

Classical grammar must never imply a fixed worker count.

For example:

parallel

does not mean:

16 threads

The execution planner decides the realization.

---

25. Distributed Integration

Classical programs may participate in distributed execution.

Distributed grammar owns:

- nodes;
- services;
- remote execution;
- replication;
- consistency;
- distributed communication;
- placement.

Classical grammar must not define:

node_count = 8

as an inherent language limitation.

A resource requirement may be expressed separately:

requires resources(...)

with resolution occurring downstream.

---

26. Quantum Integration

Classical computation is a first-class participant in hybrid quantum-classical programs.

Integration includes:

- measurement results;
- classical feed-forward;
- classical conditions;
- parameter generation;
- classical post-processing;
- adaptive quantum programs;
- dynamic circuits.

The boundary is:

Classical syntax
      ↓
AST
      ↓
semantic analysis
      ↓
classical semantic representation
      +
quantum semantic representation
      ↓
canonical IR

Quantum operations must ultimately use the canonical:

quantum::ir

boundary.

The classical grammar must never define:

Qubit
Gate
QuantumOperation
PhysicalQubit

as competing semantic representations.

The repository explicitly identifies "quantum::ir" as the canonical quantum semantic boundary and warns against duplicate frontend quantum representations.

---

27. HDL Integration

Classical computation may participate in hardware/software co-design.

However:

grammar/classical/

does not own:

- ports;
- wires;
- registers;
- clocks;
- hardware processes;
- hardware state machines;
- hardware timing;
- hardware modules.

Those belong to:

grammar/hdl/

The integration model is:

classical computation
        +
hardware intent
        ↓
semantic analysis
        ↓
co-design representation
        ↓
hardware/software lowering

This permits the same high-level computation to be realized as software, accelerator logic, or hardware where semantics permit.

---

28. AI/ML Integration

AI/ML is not a separate primitive type system hidden inside classical grammar.

AI syntax belongs to:

grammar/ai/

where required.

Classical grammar supplies the general computation foundation used by:

- model code;
- tensor operations;
- numerical kernels;
- training control;
- inference;
- differentiation;
- data transformations.

A tensor must not become a GPU-specific object.

---

29. Data Integration

Data syntax belongs to:

grammar/data/

Classical computation may consume:

- records;
- collections;
- streams;
- schemas;
- datasets;
- transformations.

The classical grammar must not duplicate those structures.

---

30. Resource Model

Classical grammar must integrate with the universal resource model.

The following concepts must remain distinct:

resource
requirement
constraint
capability
preference
hint
target
placement
performance
latency
energy
reliability
scalability
portability

For example:

requires parallel execution

does not mean:

requires 16 CPU cores

Similarly:

prefers acceleration

does not mean:

use GPU 0

This separation is essential to POCO-REAF.

---

31. Target Independence

The classical grammar must not know whether the eventual target is:

x86
ARM
RISC-V
GPU
FPGA
ASIC
QPU
simulator
cluster
cloud
embedded system
future architecture

Target information belongs to:

grammar/compile/
grammar/hardware/
grammar/resources/
grammar/execution/

and downstream compiler/runtime structures.

---

32. Semantic AST Contract

Parsing produces syntax structures.

Semantic analysis determines:

- resolved names;
- types;
- overloads;
- effects;
- capabilities;
- resource requirements;
- domain membership;
- constant evaluability;
- legality;
- ownership/resource behavior.

The AST must preserve enough source information for diagnostics and later semantic interpretation.

The parser must not prematurely erase whether syntax was:

- explicit;
- inferred;
- annotated;
- domain-specific;
- generic;
- cross-domain.

Source spans must be preserved.

---

33. Classical IR Contract

The classical grammar does not own the classical IR.

The lowering pipeline is:

source
  ↓
lexer
  ↓
parser
  ↓
AST
  ↓
semantic analysis
  ↓
canonical semantic representation
  ↓
classical IR / canonical execution representation

The IR must preserve the semantics required for:

- optimization;
- scheduling;
- resource analysis;
- target lowering;
- verification;
- runtime execution.

The grammar must never directly emit machine instructions.

---

34. Optimization Boundary

The grammar must not encode optimization implementation.

Do not make syntax inherently mean:

unroll
vectorize
SIMD
GPU
parallelize
fuse
tile
inline

unless a construct explicitly represents programmer intent and its semantics are defined independently of one particular implementation.

Optimization consumes semantic IR.

This allows the same source program to be optimized differently for different targets.

---

35. Scheduling Boundary

Scheduling is downstream.

The classical grammar must not encode fixed execution timing or machine schedules.

A source-level request such as:

parallel

can express intent.

The scheduler determines:

- ordering;
- resource allocation;
- timing;
- concurrency;
- synchronization;
- placement.

No fixed hardware topology belongs in classical grammar.

---

36. Hardware Boundary

Hardware grammar describes hardware intent.

Hardware abstraction describes available hardware.

Runtime describes actual execution.

Classical grammar must not perform hardware discovery.

This prevents the circular architecture:

classical grammar
    ↓
hardware
    ↓
grammar

The correct direction is:

grammar
    ↓
semantic model
    ↓
IR
    ↓
target/hardware analysis

---

37. Interoperability

Classical code may interoperate with:

- C;
- C++;
- Python;
- system interfaces;
- external libraries;
- accelerator APIs;
- other Zamani domains.

Interoperability syntax belongs to:

grammar/interoperability/

The classical grammar should not duplicate FFI syntax.

---

38. Diagnostics

Every classical grammar error must be reported through the canonical diagnostics architecture.

Diagnostics must identify:

- source file;
- source span;
- offending token;
- expected construct where known;
- actual construct;
- stable diagnostic identity;
- useful remediation where possible.

Do not emit:

unknown classical error

when a structured diagnostic can identify the failure.

The parser must never silently reinterpret invalid classical syntax as a different construct merely to continue.

---

39. Error Recovery

The grammar/parser integration must guarantee:

- deterministic recovery;
- forward progress;
- no infinite recovery loops;
- no panic-based malformed-source handling;
- useful error locations;
- bounded recovery work based on actual implementation resources rather than language semantics.

A malformed classical construct must not silently become a valid unrelated construct.

---

40. Versioning

Classical syntax must be versioned through the canonical Zamani language-version mechanism.

A classical feature cannot silently change meaning between language versions.

Changes must be classified as:

additive
compatible
deprecated
breaking
experimental
reserved

Version handling belongs to the language/specification system, while this directory documents classical-specific compatibility consequences.

---

41. Reserved Space

Keywords should not be added simply because a mathematical or hardware concept exists.

Before reserving a keyword, determine whether the concept can be expressed as:

function
intrinsic
type
attribute
annotation
effect
capability
resource requirement
standard-library operation

This keeps the core language stable.

---

42. Determinism

For identical:

source
language version
grammar version
lexer version

the parser must produce the same syntactic result.

No classical parser decision may depend on:

- CPU;
- GPU;
- machine size;
- thread count;
- memory availability;
- runtime device;
- wall-clock time;
- network state;
- random state.

---

43. Resource and Scale Semantics

The grammar must distinguish three categories.

43.1 Semantic size

Example:

matrix<n, m>

where "n" and "m" are part of the program's meaning.

43.2 Resource availability

Example:

available memory
available processors
available accelerator capacity

These are execution-environment properties.

43.3 Implementation limits

Example:

parser memory exhausted
integer representation overflow
backend cannot represent requested shape

These are implementation constraints.

They must not silently become language grammar limits.

---

44. Infinity and Practical Meaning

"Scale to infinity" means:

«the language imposes no arbitrary finite machine-scale ceiling on program semantics.»

It does not mean a finite machine can physically execute an infinite computation or allocate infinite memory.

Therefore the grammar must support programs whose scale is bounded by:

- program semantics;
- input size;
- available resources;
- implementation representation;
- explicit resource budgets.

The grammar itself must not impose artificial limits.

---

45. Hard-Coding Audit

Every classical grammar file must be audited for:

Forbidden accidental hard-coding

- fixed vector sizes;
- fixed matrix sizes;
- fixed tensor ranks;
- fixed thread counts;
- fixed worker counts;
- fixed GPU counts;
- fixed accelerator counts;
- fixed memory capacities;
- fixed device IDs;
- fixed node counts;
- fixed CPU counts;
- fixed architecture names embedded into semantics.

Allowed explicit semantics

A finite value is acceptable when it is genuinely part of program meaning.

For example:

matrix<3, 3>

is valid if the program intentionally defines a 3×3 matrix.

That does not impose:

MAX_MATRIX_SIZE = 3

on the language.

---

46. File Completion Contract

Every classical grammar file is considered complete only when all of the following are satisfied.

Purpose

The file has one clearly defined responsibility.

Ownership

Everything defined by the file belongs to that responsibility.

Non-ownership

Responsibilities belonging elsewhere are explicitly excluded.

Inputs

Every imported token/rule is known.

Outputs

Every exported parser rule has known consumers.

Dependencies

Dependencies are documented before implementation.

Upstream contracts

The file identifies which grammar subsystem must already exist.

Downstream contracts

The file identifies parser/AST/semantic/IR consumers.

AST contract

Every syntactic construct has a representation strategy.

Semantic contract

Its meaning is defined independently of the parser implementation.

IR contract

Its lowering destination is identified.

Compiler contract

Compiler stages consuming it are identified.

Runtime contract

Runtime impact is known, even if indirect.

Tests

Positive, negative, boundary, scalability, determinism, and cross-domain tests exist.

Compatibility

Version and migration behavior are defined.

Hard-coding audit

No accidental machine-scale limit remains.

---

47. Testing Architecture

The classical grammar requires all of the following.

grammar/tests/
├── classical/
├── positive/
├── negative/
├── boundary/
├── cross-domain/
├── scalability/
├── determinism/
├── compatibility/
└── roundtrip/

The exact physical test location may be consolidated if the repository's test architecture provides an equivalent structure.

---

48. Positive Tests

Must cover:

- scalar programs;
- vectors;
- matrices;
- tensors;
- numerical operations;
- symbolic operations;
- functions;
- generics;
- collections;
- control flow;
- memory interactions;
- concurrency interactions;
- accelerator intent;
- hybrid classical/quantum code;
- classical/HDL integration;
- classical/distributed programs;
- AI/data interactions.

---

49. Negative Tests

Must reject:

- malformed declarations;
- malformed dimensions;
- malformed tensor shapes;
- invalid type syntax;
- invalid resource expressions;
- malformed accelerator requests;
- invalid cross-domain combinations;
- malformed generic syntax;
- invalid literals;
- invalid language-version features.

Errors must be deterministic and structured.

---

50. Boundary Tests

Test:

- empty classical regions;
- single scalar;
- single-element vector;
- zero-sized structures where semantically legal;
- very large symbolic dimensions;
- deeply nested expressions;
- deeply nested generic types;
- large declaration sets;
- large function bodies;
- large tensor ranks;
- large source files.

The tests must verify that no arbitrary grammar constant is encountered.

---

51. Cross-Domain Tests

At minimum:

classical + quantum
classical + HDL
classical + hardware
classical + distributed
classical + AI
classical + data
classical + networking
classical + concurrency
classical + security
classical + quantum + distributed
classical + quantum + HDL + hardware

These tests verify that domain grammar composition does not create incompatible syntax.

---

52. Quantum-Classical Boundary Test

A representative semantic flow is:

classical value
      ↓
quantum parameter
      ↓
quantum operation
      ↓
measurement
      ↓
classical value
      ↓
classical condition
      ↓
further quantum operation

The grammar must permit the syntax.

Semantic analysis determines whether the values and operations are valid.

The canonical quantum IR remains the quantum semantic boundary.

---

53. Hardware-Classical Boundary Test

A representative flow:

classical algorithm
      ↓
parallel/accelerator intent
      ↓
semantic analysis
      ↓
capability matching
      ↓
resource analysis
      ↓
optimization
      ↓
hardware lowering

The classical grammar must not choose the hardware prematurely.

---

54. POCO-REAF Test

A POCO-REAF test should use the same source semantics while varying the target description.

Conceptually:

same source
   ↓
target A
target B
target C
target D

The parser result and semantic meaning must remain unchanged.

Only downstream realization is allowed to differ.

---

55. Repository Integration

The classical grammar must integrate with the existing repository architecture.

Relevant repository surfaces include:

grammar/
src/lexer.rs
src/parser.rs
src/ast/
src/semantic.rs
src/ir_gen.rs
src/ir_verify.rs
src/classical/
src/compiler/
src/quantum/
src/hdl/
src/distributed/
src/ai/

The repository currently exposes "src/classical/mod.rs" as a minimal classical module, so grammar work must be coordinated with semantic/IR implementation rather than assuming a mature classical backend already exists.

The AST is currently centralized under "src/ast/mod.rs", making preservation of a single frontend AST architecture especially important.

The repository also has a dedicated "src/ir_gen.rs" and "src/ir_verify.rs"; classical syntax should therefore lower through the existing IR architecture rather than introducing a grammar-owned execution representation.

---

56. Dependency Graph

The classical grammar must follow:

specification
      ↓
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
effects / capabilities
      ↓
memory / concurrency
      ↓
classical
      ↓
hybrid / HDL / hardware / distributed / AI / data
      ↓
compile
      ↓
execution
      ↓
interoperability

Within classical:

classical.g4
   ↓
scalar.g4
vector.g4
matrix.g4
tensor.g4
numerical.g4
symbolic.g4
classical-accelerators.g4

Only genuine dependencies should be introduced.

---

57. Classical Internal Dependency Graph

The preferred dependency structure is:

shared lexer
      ↓
shared core syntax
      ↓
shared types / expressions
      ↓
classical.g4
      ↓
+----------+----------+----------+----------+
|          |          |          |          |
scalar   vector     matrix     tensor   numerical
                                      |
                                      ↓
                                  symbolic
                                      |
                                      ↓
                              accelerators

Where a subgrammar depends only on shared constructs, it must not depend on unrelated classical subgrammars.

For example:

vector.g4

must not require:

tensor.g4

unless the language semantics genuinely require tensor syntax for vector parsing.

---

58. No Circular Grammar Dependencies

Forbidden:

classical → quantum → classical
classical → hardware → classical
classical → IR → classical
classical → runtime → classical

Allowed:

classical grammar
       ↓
shared syntax
       ↓
AST
       ↓
semantic analysis
       ↓
IR
       ↓
quantum/hardware/runtime

---

59. ANTLR Integration

The production grammar must have one canonical token vocabulary.

Classical grammar files must not independently redefine lexical tokens.

The current repository already has a broad canonical lexer implementation in "src/lexer.rs", and the grammar design explicitly requires a canonical lexical vocabulary.

The ANTLR composition system must therefore ensure that:

one token meaning
=
one canonical lexical definition

Domain names such as mathematical functions should generally remain identifiers unless they genuinely require reserved lexical treatment.

---

60. Root Grammar Integration

"grammar/Zamani.g4" currently contains direct mathematical declarations and operations, including fixed integer dimension forms in several mathematical productions.

Those constructs must be reconciled with the modular classical grammar.

The final architecture must not leave two competing implementations such as:

Zamani.g4
    → one vector language

classical/vector.g4
    → another vector language

Instead:

canonical grammar authority
        ↓
classical modular composition
        ↓
one semantic definition

The reconciliation must preserve valid existing Zamani features while removing accidental duplication.

---

61. Existing Grammar Compatibility

The current implementation-oriented grammar records a broad set of classical constructs, including scalar types, generic types, arrays, tuples, expressions, functions, patterns, and mathematical facilities.

Before changing syntax:

1. identify existing syntax;
2. identify parser support;
3. identify AST support;
4. identify semantic support;
5. identify IR support;
6. identify tests;
7. determine whether the syntax is normative;
8. preserve valid behavior;
9. migrate structurally misplaced behavior;
10. deprecate incompatible syntax explicitly.

No valid existing capability may be silently discarded.

---

62. What Must Not Be Done

Do not:

- create a second classical AST;
- create a second type system;
- create a second expression grammar;
- create a second lexer;
- hard-code machine capacities;
- hard-code accelerator IDs;
- hard-code CPU counts;
- hard-code vector widths;
- hard-code tensor rank;
- select hardware in classical syntax;
- make mathematical functions into endless keywords;
- put optimization algorithms into grammar;
- put scheduling algorithms into grammar;
- put runtime behavior into grammar;
- put classical IR into grammar;
- use embedded Rust actions;
- use "unsafe";
- create empty placeholder grammar files;
- duplicate quantum semantics.

---

63. Production File Checklist

"README.md"

Must define:

- ownership;
- non-ownership;
- file map;
- dependency graph;
- scalability rules;
- hard-coding policy;
- AST contract;
- semantic contract;
- IR contract;
- compiler integration;
- runtime integration;
- testing;
- compatibility.

"classical.g4"

Must define:

- classical composition;
- genuine classical entry points;
- subgrammar integration.

"scalar.g4"

Must define:

- scalar-specific syntax only;
- no machine width assumptions.

"vector.g4"

Must define:

- vector-specific syntax;
- symbolic/runtime dimensions;
- no fixed vector maximum.

"matrix.g4"

Must define:

- matrix-specific syntax;
- symbolic/runtime dimensions;
- no fixed matrix maximum.

"tensor.g4"

Must define:

- tensor-specific syntax;
- arbitrary semantic rank;
- shape abstraction;
- no fixed tensor rank.

"numerical.g4"

Must define:

- genuine numerical language constructs;
- no backend-specific algorithms.

"symbolic.g4"

Must define:

- symbolic language constructs;
- no CAS implementation assumptions.

"classical-accelerators.g4"

Must define:

- portable accelerator intent;
- capability integration;
- no physical device selection.

---

64. Definition of Done

"grammar/classical/" is production-ready only when:

Grammar

- ANTLR generation succeeds.
- All grammar imports resolve.
- No duplicate token definitions exist.
- No ambiguous classical alternatives remain unintentionally.
- No unreachable classical productions remain.
- No artificial scale limits exist.

AST

- Every accepted semantic construct has an AST representation.
- Source spans are preserved.
- No lossy conversion occurs.

Semantics

- Classical domain classification is defined.
- Type checking is defined.
- Shape checking is defined where applicable.
- Effect checking is defined.
- Capability/resource checking is defined.
- Cross-domain rules are defined.

IR

- Every supported construct has a lowering contract.
- Classical operations have a canonical semantic representation.
- Quantum constructs remain behind "quantum::ir".
- No duplicate frontend IR exists.

Compiler

- Parsing integrates with the canonical frontend.
- Semantic analysis consumes the AST.
- IR generation consumes semantic information.
- Verification can validate generated representations.

Runtime

- No classical grammar rule directly depends on a runtime device.
- Runtime resource selection occurs downstream.

Safety

- Rust 1.97/1.97.1 compatibility is maintained.
- Rust 2021 is maintained.
- No "unsafe".
- No embedded Rust actions.
- No target-dependent parser behavior.

Testing

- Positive tests pass.
- Negative tests pass.
- Boundary tests pass.
- Scalability tests pass.
- Determinism tests pass.
- Cross-domain tests pass.
- Compatibility tests pass.
- Round-trip tests pass where a canonical printer exists.

---

65. Final Classical Architecture

The final architecture is:

                    ZAMANI SOURCE
                          │
                          ▼
                  CANONICAL LEXER
                          │
                          ▼
                  CANONICAL PARSER
                          │
             ┌────────────┴────────────┐
             │                         │
             ▼                         ▼
       SHARED LANGUAGE             CLASSICAL
         SYNTAX                    DOMAIN SYNTAX
             │                         │
             └────────────┬────────────┘
                          ▼
                         AST
                          │
                          ▼
              NAME / TYPE / EFFECT
                / CAPABILITY ANALYSIS
                          │
                          ▼
              CANONICAL SEMANTIC IR
                          │
          ┌───────────────┼────────────────┐
          │               │                │
          ▼               ▼                ▼
     CLASSICAL IR     quantum::ir     Other Domains
          │               │                │
          └───────────────┼────────────────┘
                          ▼
                     OPTIMIZATION
                          │
                          ▼
             RESOURCE / RESILIENCE
                          │
                          ▼
              ROUTING / SCHEDULING
                          │
                          ▼
                   TARGET LOWERING
                          │
          ┌───────────────┼────────────────────┐
          │               │                    │
          ▼               ▼                    ▼
         CPU             GPU/FPGA             QPU
          │               │                    │
          └───────────────┼────────────────────┘
                          ▼
                     EXECUTION

The classical grammar therefore has one fundamental responsibility:

«Express classical computation without turning the characteristics of today's machine into permanent language semantics.»

Its scalability boundary is the semantics of the program, not the number of processors, cores, threads, devices, bytes, vector lanes, nodes, or accelerators available today.

The ultimate invariant is:

ONE PROGRAM
     ↓
ONE SEMANTIC MEANING
     ↓
ONE PORTABLE LANGUAGE MODEL
     ↓
MANY COMPILATION TARGETS
     ↓
MANY HARDWARE CONFIGURATIONS
     ↓
MANY SCALES
     ↓
MANY EXECUTION ENVIRONMENTS
     ↓
FUTURE PLATFORMS

That is the classical-domain contribution to:

Zamani — From Atom to Everywhere

and:

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF).