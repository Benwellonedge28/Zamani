Zamani Classical Grammar

Status

Production architecture document

Path: "grammar/classical/README.md"

Domain: Classical computation

Language: Zamani

Grammar technology: ANTLR4

Implementation baseline: Rust 1.97 / Rust 1.97.1, Rust 2021

Safety requirement: No "unsafe" Rust.

Grammar safety: No embedded Rust actions, no target-specific parser code, and no semantic predicates whose behavior depends on a target machine.

Architectural role: Classical-domain syntax composition.

---

1. Purpose

"grammar/classical/" defines the source-language grammar boundary for classical computation in Zamani.

It exists to make classical computation a first-class domain of the universal Zamani language while preserving the central Zamani principle:

«Zamani describes computation and intent, not the accidental characteristics of the machine currently available.»

The classical grammar must therefore support programs ranging from extremely small computations to computations whose eventual resource requirements are bounded only by the available implementation resources.

The grammar must not impose artificial machine-scale limits.

The classical domain must remain composable with:

- quantum computing;
- hybrid quantum-classical computing;
- HDL;
- hardware/software co-design;
- distributed computing;
- parallel computing;
- HPC;
- AI/ML;
- data processing;
- accelerators;
- networking;
- cryptography;
- scientific computing;
- embedded computing;
- future Zamani computational domains.

The classical grammar is consequently a domain composition layer, not a separate programming language.

---

2. POCO-REAF

The classical grammar participates in:

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever

or:

POCO-REAF

A classical Zamani program expresses its semantic computation once.

The same source semantics must be capable of being lowered for different execution environments without requiring the programmer to rewrite the computation merely because the available machine changes.

Possible execution environments include:

- tiny embedded systems;
- single CPUs;
- multicore CPUs;
- vector processors;
- GPUs;
- FPGAs;
- ASICs;
- classical accelerators;
- heterogeneous processors;
- quantum-classical systems;
- clusters;
- supercomputers;
- distributed systems;
- edge systems;
- cloud systems;
- future execution architectures.

The grammar must not encode assumptions about which of these environments will eventually execute the program.

---

3. Scope

This directory owns classical-domain syntax composition.

It does not own the entire Zamani language.

The overall architecture is:

Zamani source
     |
     v
Canonical lexer
     |
     v
Canonical parser grammar
     |
     +-----------------------------+
     |                             |
     v                             v
General syntax                 Classical domain
     |                             |
     v                             v
AST / syntax model --------> semantic analysis
                                   |
                                   v
                            canonical semantic IR
                                   |
             +---------------------+----------------------+
             |                     |                      |
             v                     v                      v
      Classical IR            quantum::ir           other IRs
             |                     |                      |
             +---------------------+----------------------+
                                   |
                                   v
                              optimization
                                   |
                                   v
                              scheduling
                                   |
                                   v
                         routing / target lowering
                                   |
                                   v
                               execution

The classical grammar must never become a replacement for this architecture.

---

4. Ownership

4.1 This directory owns

"grammar/classical/" owns:

- classical-domain grammar composition;
- classical computation entry points;
- classical-domain parser boundaries;
- classical scalar-domain syntax hooks;
- classical vector-domain syntax hooks;
- classical matrix-domain syntax hooks;
- classical tensor-domain syntax hooks;
- classical numerical-computation syntax hooks;
- classical symbolic-computation syntax hooks;
- classical accelerator-domain syntax hooks;
- classical-domain integration boundaries;
- classical-domain grammar documentation;
- classical-domain grammar tests.

The individual files within this directory own their specific subdomains.

For example:

scalar.g4
    scalar-domain syntax

vector.g4
    vector-domain syntax

matrix.g4
    matrix-domain syntax

tensor.g4
    tensor-domain syntax

numerical.g4
    numerical-computation syntax

symbolic.g4
    symbolic-computation syntax

classical-accelerators.g4
    accelerator-intent syntax

"classical.g4" owns the composition boundary.

---

5. Non-ownership

The classical grammar does not own:

- lexer definitions;
- token spelling;
- Unicode policy;
- identifier spelling;
- operator precedence;
- general expressions;
- general type syntax;
- general statements;
- general declarations;
- function declarations;
- module declarations;
- package management;
- memory implementation;
- ownership implementation;
- borrowing implementation;
- lifetime implementation;
- concurrency implementation;
- scheduling;
- routing;
- optimization;
- hardware discovery;
- hardware topology;
- calibration;
- backend selection;
- runtime execution;
- resource discovery;
- classical algorithms;
- classical IR;
- "quantum::ir";
- QEC;
- ZQN;
- quantum hardware;
- physical device identifiers;
- physical addresses;
- target-specific implementation details.

These responsibilities remain with their respective repository subsystems.

---

6. Canonical grammar boundary

The classical grammar must consume canonical language abstractions wherever they already exist.

The classical domain must not duplicate:

- "expression";
- "typeExpression";
- identifier syntax;
- literals;
- generic parameter syntax;
- function syntax;
- declaration syntax;
- module syntax;
- statement syntax;
- operator precedence.

Where a canonical grammar component exists, classical grammar files must compose it.

This prevents multiple incompatible definitions of the same Zamani language feature.

---

7. Integration with the canonical lexer

The classical grammar must consume tokens produced by the canonical Zamani lexer.

It must not create an independent lexer.

The lexer is responsible for:

- keywords;
- identifiers;
- literals;
- operators;
- punctuation;
- comments;
- source locations;
- lexical errors.

The classical grammar is responsible only for arranging those tokens into classical-domain syntactic structures.

The classical grammar therefore must use the repository's canonical token vocabulary.

A classical grammar file must never silently introduce a second spelling for an existing token.

---

8. Integration with expressions

General expression syntax belongs to the shared expression grammar.

Classical computation must be able to consume canonical expressions.

This allows classical computation to use:

- literals;
- variables;
- function calls;
- arithmetic;
- comparison;
- logical operations;
- bitwise operations;
- indexing;
- member access;
- ranges;
- conditional expressions;
- lambda expressions;
- collection expressions;
- compile-time expressions;
- future expression extensions.

A new expression feature should therefore normally be implemented in the shared expression layer rather than duplicated in "grammar/classical/".

The classical grammar may expose a semantic-domain wrapper around an expression when a stable classical parser boundary is required.

---

9. Integration with types

Classical type syntax is owned by the common type system.

The classical grammar must consume canonical type expressions.

Relevant repository integration includes:

grammar/types/types.g4
grammar/types/primitive-types.g4
grammar/types/composite-types.g4
grammar/types/generic-types.g4
grammar/types/array-types.g4
grammar/types/tuple-types.g4
grammar/types/reference-types.g4
grammar/types/resource-types.g4
grammar/types/classical-types.g4

The repository already identifies classical types such as vector and matrix forms in the shared type layer.

Classical grammar files must not redefine those types merely to make them available to classical constructs.

Instead:

classical grammar
        |
        v
canonical typeExpression
        |
        v
semantic type checking

---

10. Classical scalar computation

The scalar domain must support semantic computation over scalar values without imposing machine-dependent limits.

Examples include:

- integers;
- unsigned integers;
- floating-point values;
- decimal values;
- arbitrary-precision semantic numeric types where supported;
- complex values;
- boolean values;
- character values;
- user-defined scalar-like types.

The grammar must not encode a machine-specific integer width as the universal language meaning.

For example, syntax must not imply:

int = exactly 32 bits

unless such a width is explicitly part of the language's semantic type definition.

Target-specific widths belong to type lowering and target descriptions.

---

11. Vector computation

Vectors must be expressible without imposing a fixed maximum length.

The language must support:

- statically described shapes where shape is semantic;
- dynamically determined lengths;
- symbolic lengths;
- generic lengths;
- runtime-determined lengths;
- empty collections where semantically valid;
- arbitrarily large collections subject to implementation resources.

The grammar must never define:

MAX_VECTOR_LENGTH

or an equivalent machine limit.

A vector's size is a program property, input property, type-level property, runtime property, or resource constraint—not a parser limitation.

---

12. Matrix computation

Matrices must support:

- row/column dimensions;
- symbolic dimensions;
- generic dimensions;
- runtime dimensions;
- rectangular matrices;
- square matrices;
- empty dimensions where the semantic type permits them;
- nested matrix expressions;
- composition with vector and tensor computations.

The grammar must not impose:

MAX_ROWS
MAX_COLUMNS
MAX_MATRIX_SIZE

or equivalent limits.

A matrix dimension belongs to the semantic program/resource model.

---

13. Tensor computation

Tensor syntax must remain independent of machine capacity.

The grammar must support tensor expressions whose:

- rank;
- dimensions;
- shape;
- storage representation;
- layout;
- execution strategy

are not unnecessarily fixed by source syntax.

The grammar must not define a maximum tensor rank merely because a particular implementation currently has a finite internal representation.

If an implementation has a practical limit, that limit must be represented as an explicit implementation/resource constraint.

It must not become a hidden language rule.

---

14. Numerical computing

"numerical.g4" may provide syntax for numerical-domain intent.

Examples include:

- numerical transformations;
- numerical integration;
- differentiation;
- interpolation;
- Fourier operations;
- signal operations;
- numerical solving;
- optimization-related numerical constructs.

The grammar describes the requested computation.

It must not select:

- a CPU;
- a GPU;
- a specific library;
- a specific BLAS implementation;
- a particular SIMD width;
- a particular accelerator;
- a particular numerical backend.

Those decisions belong downstream.

---

15. Symbolic computing

"symbolic.g4" provides symbolic-computation syntax.

The grammar may represent intent such as:

- symbolic expressions;
- symbolic variables;
- substitution;
- simplification;
- expansion;
- factoring;
- equation solving;
- differentiation;
- integration;
- symbolic transformations.

Symbolic semantics belong to semantic analysis and the appropriate compiler/IR layers.

The grammar must not assume a specific symbolic mathematics implementation.

---

16. Classical accelerators

Classical accelerator syntax must describe intent and requirements, not physical device selection.

Valid conceptual distinctions include:

requires acceleration

versus:

requires capability(...)

versus:

prefers capability(...)

versus:

targets(...)

The grammar must not make an accelerator request equivalent to:

use GPU 0

or:

use device X

or:

use exactly N GPUs

unless such information is explicitly part of a target/deployment specification.

Accelerator selection belongs downstream to:

- capability analysis;
- resource management;
- optimization;
- scheduling;
- target lowering;
- hardware abstraction;
- runtime dispatch.

---

17. Resource independence

Classical syntax must remain independent of physical resource capacity.

The grammar must not hard-code:

- CPU count;
- core count;
- thread count;
- register count;
- register width;
- SIMD width;
- GPU count;
- accelerator count;
- memory capacity;
- cache capacity;
- NUMA layout;
- cluster size;
- node count;
- network topology;
- device identifiers;
- physical addresses.

The repository's resource grammar owns resource-oriented source concepts.

The classical grammar consumes those abstractions where appropriate.

---

18. Semantic requirements versus implementation choices

A fundamental rule is:

«A semantic requirement must never be confused with an implementation choice.»

For example:

requires parallel execution

is different from:

use 16 threads

Likewise:

requires vector capability

is different from:

use AVX-512

And:

requires accelerator capability

is different from:

use GPU device 0

The first category belongs to portable program semantics.

The second category belongs to target/resource/deployment configuration.

---

19. Classical and quantum integration

Classical computation must be able to participate in hybrid quantum-classical programs.

The classical grammar must therefore remain compatible with:

- quantum measurement results;
- classical conditions;
- classical feed-forward;
- runtime classical computation;
- parameter computation;
- classical control of quantum operations;
- quantum result post-processing.

The classical grammar must not import or redefine "quantum::ir".

The architecture remains:

classical syntax
       |
       v
semantic analysis
       |
       +-------------------+
       |                   |
       v                   v
classical semantics   quantum semantics
                           |
                           v
                      quantum::ir

The classical grammar provides syntax.

The quantum subsystem provides quantum semantics.

---

20. Classical and HDL integration

Classical computation must be usable in hardware/software co-design.

However, classical grammar must not redefine HDL syntax.

The separation is:

classical/
    software computation

hdl/
    hardware semantics

hardware/
    target/resource semantics

Cross-domain constructs are interpreted by the appropriate semantic layer.

This allows the same computation to be considered for:

- software execution;
- hardware implementation;
- accelerator implementation;
- heterogeneous execution.

---

21. Classical and distributed computation

The classical grammar must not encode a fixed number of nodes.

Distributed constructs belong to the distributed grammar.

Classical code may be consumed by distributed execution semantics.

Therefore:

classical computation
        +
distributed intent
        |
        v
semantic analysis
        |
        v
distributed execution planning

No fixed cluster size belongs in classical grammar.

---

22. Classical and concurrency integration

Concurrency constructs belong to the concurrency grammar.

Classical computations must be capable of participating in:

- tasks;
- futures;
- actors;
- channels;
- parallel execution;
- data parallelism;
- task parallelism;
- synchronization;
- cancellation.

The classical grammar must not duplicate concurrency syntax.

The existing repository already treats classical computation as an ordinary consumer of concurrency intent.

---

23. Classical and AI/data integration

Classical expressions and values must be usable by:

- AI models;
- tensor computations;
- datasets;
- transformations;
- inference;
- training;
- numerical pipelines;
- symbolic computation.

AI and data grammars own their respective domain syntax.

Classical grammar supplies the general computational foundation.

---

24. AST contract

The grammar produces parser structures.

It does not define the final semantic AST/IR.

The AST layer must preserve enough information to distinguish:

- source locations;
- syntax;
- names;
- expressions;
- types;
- declarations;
- classical-domain boundaries;
- cross-domain boundaries;
- annotations;
- source metadata.

Semantic classification must occur after parsing.

For example, the parser should not decide that:

x + y

is necessarily numerical merely because it contains "+".

Semantic analysis determines the resolved types and operation.

---

25. IR contract

The classical grammar does not define classical IR.

The compilation architecture must lower parsed classical syntax into the repository's canonical semantic representation.

Where the repository provides a classical IR, semantic lowering maps classical AST constructs into it.

For quantum participation:

source
  |
  v
parser
  |
  v
AST
  |
  v
semantic analysis
  |
  +---- classical semantics
  |
  +---- quantum semantics
             |
             v
        quantum::ir

"grammar/classical/" must never create a second quantum representation.

---

26. Optimization boundary

Optimization must consume semantic IR.

The classical grammar must not encode optimization strategies.

For example, the grammar must not force:

- loop unrolling;
- SIMD;
- GPU execution;
- vectorization;
- constant folding;
- instruction selection;
- register allocation;
- fusion;
- tiling.

Such decisions belong to optimization and lowering.

The repository's optimization subsystem therefore remains downstream of grammar and semantic analysis.

---

27. Scheduling boundary

Scheduling is downstream.

The classical grammar must not encode:

- processor count;
- thread count;
- fixed execution slots;
- machine timing;
- physical latency;
- hardware topology.

Scheduling determines an execution plan after semantic lowering and resource/capability analysis.

---

28. Hardware boundary

Hardware-specific information belongs to hardware/resource/target systems.

The classical grammar must not own:

- device discovery;
- calibration;
- hardware topology;
- physical placement;
- device IDs;
- hardware addresses;
- backend-specific limits.

This ensures POCO-REAF remains possible.

---

29. Runtime boundary

Runtime execution is not a grammar responsibility.

Runtime determines how a compiled semantic representation is actually executed against available resources.

The runtime must be able to discover or receive:

- available capabilities;
- available resources;
- execution environment;
- deployment context;
- target information;
- runtime constraints.

None of those properties should be silently embedded into classical syntax.

---

30. Error model

Grammar errors must describe syntactic failures.

Examples include:

- malformed classical construct;
- unexpected token;
- missing delimiter;
- malformed expression composition;
- malformed domain construct;
- malformed classical declaration boundary.

Semantic errors belong to semantic analysis.

Resource errors belong to resource management.

Compilation errors belong to compilation.

Runtime failures belong to runtime.

Hardware failures belong to hardware/runtime/resilience.

The grammar must not encode provider-specific runtime error codes.

---

31. Determinism

Parsing must be deterministic for the same:

source
+
grammar version
+
lexer version

The grammar must not depend on:

- machine identity;
- runtime hardware;
- random values;
- network state;
- backend discovery;
- current time.

The same valid source must therefore produce the same syntactic interpretation.

---

32. Scalability

The classical grammar must scale in language representation without introducing artificial finite machine limits.

It must not define:

MAX_ELEMENTS
MAX_VECTOR_LENGTH
MAX_MATRIX_ROWS
MAX_MATRIX_COLUMNS
MAX_TENSOR_RANK
MAX_THREADS
MAX_CORES
MAX_DEVICES
MAX_MEMORY
MAX_ARGUMENTS
MAX_PARAMETERS

or equivalent constructs.

A grammar repetition such as:

item*

or:

item (COMMA item)*

describes an unbounded language construct subject to parser/compiler implementation resources.

Implementation resource limits are not semantic language limits.

---

33. Important distinction: finite implementation versus infinite language

"Scale to infinity" means the language must not impose an arbitrary finite semantic ceiling.

It does not mean a physical machine has infinite:

- memory;
- execution time;
- storage;
- bandwidth;
- processing capacity.

The correct contract is:

Zamani language
    |
    | no artificial machine-scale ceiling
    v
implementation
    |
    | constrained by available resources
    v
actual execution

A program may therefore be arbitrarily large in the language model while a particular execution environment may reject it because its resources are insufficient.

That is a resource/execution decision, not a grammar restriction.

---

34. Versioning

Classical grammar evolution must be compatible with Zamani language versioning.

Changes must be classified as:

- additive;
- compatible;
- incompatible;
- deprecated;
- removed;
- reserved.

A grammar feature must not silently change meaning between versions.

Any syntax migration must be documented in:

grammar/compatibility/
grammar/specification/

where appropriate.

---

35. Reserved syntax

The classical domain must preserve reserved space for future evolution.

Reserved syntax must not accidentally consume namespaces required by:

- quantum computing;
- HDL;
- hardware;
- distributed systems;
- AI;
- networking;
- security;
- future computational domains.

The language must evolve without requiring large-scale grammar rewrites whenever a new computational paradigm is introduced.

---

36. Dependency contract

The intended dependency direction is:

lexer
  |
  v
core
  |
  v
types / expressions
  |
  v
statements / declarations / functions
  |
  v
classical
  |
  +---- quantum
  +---- HDL
  +---- hardware
  +---- distributed
  +---- AI
  +---- data

Classical grammar may depend on lower-level shared syntax.

It must not make lower-level syntax depend back on classical grammar.

In particular, avoid:

classical -> expressions -> classical

or:

classical -> quantum -> classical

or:

classical -> hardware -> classical

Cross-domain semantics belong above individual grammar domains.

---

37. Repository integration matrix

Subsystem| Classical grammar relationship
Canonical lexer| Consumes canonical tokens
Core grammar| Consumes names/metadata/path abstractions
Types| Consumes canonical "typeExpression"
Expressions| Consumes canonical "expression"
Statements| Provides general statement composition
Declarations| Provides declaration composition
Functions| Provides function composition
Modules| Provides module/package structure
Effects| Provides effect/capability semantics
Memory| Provides memory semantics
Concurrency| Provides concurrency semantics
Classical grammar| Classical domain composition
Quantum grammar| Cross-domain consumer; no ownership transfer
HDL grammar| Cross-domain consumer; no duplication
Hardware grammar| Capability/target boundary
Distributed grammar| Distribution boundary
AI grammar| AI-domain integration
Data grammar| Data-domain integration
Resources grammar| Resource requirements/capabilities
Compile grammar| Compilation intent
Execution grammar| Execution/deployment intent
Classical IR| Semantic lowering target
"quantum::ir"| Quantum semantic boundary; classical grammar never owns it
QEC| No direct grammar ownership
ZQN| No direct grammar ownership
Optimization| Consumes lowered semantic representation
Scheduling| Consumes executable semantic representation
Hardware HAL| Resolves physical capabilities
Runtime| Executes compiled representation
Resilience| Handles execution/recovery decisions downstream
Tests| Validates grammar and integration

---

38. Relationship to existing classical files

The classical directory is intentionally decomposed.

Expected responsibilities are:

classical.g4
    Classical-domain composition and integration boundary.

scalar.g4
    Scalar computation syntax.

vector.g4
    Vector-domain syntax.

matrix.g4
    Matrix-domain syntax.

tensor.g4
    Tensor-domain syntax.

numerical.g4
    Numerical computation syntax.

symbolic.g4
    Symbolic computation syntax.

classical-accelerators.g4
    Accelerator-oriented classical intent.

The existing repository already contains these classical-domain components, including vector and tensor grammar files.

Each file must remain independently completable.

---

39. Rule for new classical features

When adding a classical language feature, determine first:

1. Is it lexical?
2. Is it a general expression?
3. Is it a general type?
4. Is it a general statement?
5. Is it a declaration?
6. Is it a function feature?
7. Is it a classical-domain feature?
8. Is it a resource/capability feature?
9. Is it a compiler feature?
10. Is it an execution feature?

Only a genuinely classical-domain syntax feature belongs here.

This prevents "grammar/classical/" from becoming a dumping ground.

---

40. Hard-coding audit

Every classical grammar change must be checked for:

Machine-size hard-coding

- CPU count;
- core count;
- thread count;
- GPU count;
- accelerator count;
- node count.

Memory hard-coding

- fixed memory capacity;
- cache size;
- buffer capacity;
- address-space assumptions.

Vector/matrix/tensor hard-coding

- fixed vector length;
- fixed matrix dimensions;
- fixed tensor rank;
- fixed tensor dimensions.

Architecture hard-coding

- CPU architecture;
- GPU architecture;
- SIMD width;
- register width;
- instruction-set assumptions.

Deployment hard-coding

- device identifiers;
- addresses;
- cluster topology;
- cloud provider;
- execution location.

Temporal hard-coding

- fixed hardware timing;
- fixed latency;
- fixed throughput.

Every discovered restriction must be classified as:

1. language semantic requirement;
2. target requirement;
3. resource constraint;
4. implementation limitation;
5. accidental hard-coding;
6. test-only limitation;
7. documentation-only limitation.

Accidental hard-coding must be removed.

---

41. Testing requirements

Every classical grammar rule requires positive and negative coverage.

Tests must cover:

Scalar

- valid scalar expressions;
- invalid scalar syntax;
- numeric boundaries;
- arbitrary supported literal forms.

Vector

- empty/vector edge cases where semantically valid;
- symbolic lengths;
- generic lengths;
- runtime lengths;
- large conceptual lengths;
- invalid vector syntax.

Matrix

- rectangular matrices;
- square matrices;
- symbolic dimensions;
- generic dimensions;
- dynamic dimensions;
- malformed matrices.

Tensor

- varying rank;
- symbolic shapes;
- generic shapes;
- dynamic shapes;
- large conceptual ranks;
- invalid tensor syntax.

Numerical

- valid numerical constructs;
- invalid numerical constructs;
- composition with expressions.

Symbolic

- symbolic expressions;
- transformations;
- invalid syntax.

Accelerator

- capability-oriented syntax;
- preference syntax where supported;
- absence of device hard-coding;
- invalid target-specific syntax.

---

42. Boundary tests

The classical grammar must test the smallest valid programs and progressively larger structures.

Examples include:

single scalar
single expression
single vector
single matrix
single tensor
deeply nested expression
large declaration sequence
large collection expression
large generic structure
large classical computation

The test suite must distinguish parser scalability from implementation resource limits.

A test must never establish an accidental language maximum merely because the test fixture happens to contain a particular size.

---

43. Cross-domain tests

The classical grammar must be tested in combination with:

classical + quantum
classical + HDL
classical + hardware
classical + distributed
classical + AI
classical + data
classical + networking
classical + security
classical + accelerator
classical + quantum + distributed
classical + quantum + HDL + hardware
classical + AI + quantum + hardware

These tests verify that domain composition does not introduce circular grammar dependencies.

---

44. Round-trip testing

Where the repository provides a canonical printer/serializer:

source
  |
  v
lexer
  |
  v
parser
  |
  v
AST
  |
  v
printer/serializer
  |
  v
parser

must preserve intended syntax/semantic structure.

Formatting differences are acceptable where the language specification permits them.

Semantic changes are not acceptable.

---

45. Determinism testing

The same source must be parsed repeatedly into equivalent syntax structures.

Tests must detect:

- ambiguous grammar alternatives;
- nondeterministic parser behavior;
- machine-dependent parsing;
- accidental dependence on external state.

---

46. Compiler integration

The grammar is only the front end.

The complete pipeline is:

.zm source
   |
   v
Zamani lexer
   |
   v
Zamani parser
   |
   v
syntax AST
   |
   v
semantic analysis
   |
   v
classical semantic representation
   |
   v
canonical IR
   |
   +-------------------+
   |                   |
   v                   v
classical IR      cross-domain IR
   |                   |
   +---------+---------+
             |
             v
        optimization
             |
             v
        scheduling
             |
             v
       target lowering
             |
             v
          runtime

The grammar must not skip semantic analysis by directly embedding implementation behavior in parser actions.

---

47. Runtime integration

Runtime capabilities may determine:

- available resources;
- available accelerators;
- execution placement;
- scheduling;
- backend selection;
- deployment;
- performance characteristics.

The classical grammar must not encode these runtime properties.

This is necessary for POCO-REAF.

---

48. Rust requirements

The grammar itself is ANTLR grammar source.

Rust 1.97 / Rust 1.97.1 applies to the repository's parser/compiler integration.

The Rust implementation must:

- compile on the supported Rust version;
- use Rust 2021;
- contain no "unsafe";
- avoid unsafe FFI assumptions;
- avoid parser actions containing Rust code;
- preserve deterministic behavior;
- expose structured diagnostics;
- avoid architecture-specific assumptions.

The grammar must remain target-neutral.

---

49. Security requirements

Classical grammar parsing must not:

- access files;
- access networks;
- execute programs;
- invoke hardware;
- access environment variables;
- invoke external commands;
- perform dynamic loading.

Parsing is a pure language-processing concern.

External effects belong to explicitly authorized downstream systems.

---

50. Diagnostics

Diagnostics should eventually expose:

- source span;
- token;
- expected syntax;
- actual syntax;
- grammar version;
- diagnostic category;
- stable diagnostic identifier.

Parser diagnostics must not expose provider-specific hardware errors.

Semantic diagnostics must be emitted by semantic analysis.

---

51. Compatibility

Existing valid classical syntax must not be removed silently.

Before changing a construct:

1. identify existing consumers;
2. identify current syntax;
3. identify intended semantics;
4. determine compatibility impact;
5. preserve valid behavior where possible;
6. migrate structurally misplaced features;
7. deprecate explicitly when necessary;
8. document removals.

The shared grammar documentation and authority files must remain synchronized with these decisions.

The repository already contains a grammar-authority layer that requires classical operations to remain composable with quantum and hardware domains.

---

52. What must never be placed here

Do not add:

MAX_CPU
MAX_CORES
MAX_THREADS
MAX_VECTOR_LENGTH
MAX_MATRIX_ROWS
MAX_MATRIX_COLUMNS
MAX_TENSOR_RANK
MAX_MEMORY
MAX_GPUS
MAX_QUBITS
DEVICE_0
GPU_0
CPU_0
FIXED_TOPOLOGY
FIXED_ADDRESS

or equivalent hidden restrictions.

Do not add:

- physical device IDs;
- provider-specific hardware names as semantic requirements;
- machine-specific timing;
- hard-coded accelerator counts;
- fixed cluster sizes;
- backend discovery;
- scheduling algorithms;
- routing algorithms;
- QEC algorithms;
- ZQN noise models;
- runtime recovery logic.

---

53. Completion contract

"grammar/classical/README.md" is complete when it establishes:

- classical grammar ownership;
- non-ownership;
- dependency direction;
- lexer integration;
- expression integration;
- type integration;
- AST boundary;
- semantic boundary;
- IR boundary;
- compiler boundary;
- runtime boundary;
- quantum integration;
- HDL integration;
- hardware integration;
- distributed integration;
- AI/data integration;
- resource integration;
- scalability rules;
- POCO-REAF rules;
- hard-coding rules;
- versioning rules;
- diagnostics rules;
- security rules;
- testing rules;
- determinism rules;
- cross-domain rules;
- file-level responsibilities.

No later classical grammar file should need to redefine these architectural contracts.

---

54. File-level completion rule

Before any file in this directory is considered complete, it must have:

Purpose
Ownership
Non-ownership
Inputs
Outputs
Dependencies
Upstream contracts
Downstream consumers
Public grammar contract
AST contract
Semantic contract
IR integration
Compiler integration
Runtime integration
Tooling integration
Cross-domain integration
Positive tests
Negative tests
Boundary tests
Compatibility tests
Determinism tests
Scalability tests
Hard-coding audit
Completion criteria

This guarantees that completing one file does not require redesigning it after another classical grammar file is implemented.

---

55. Dependency-first implementation order

The recommended order is:

1. grammar/specification/*
        |
        v
2. canonical lexer
        |
        v
3. grammar/core/*
        |
        v
4. grammar/types/*
        |
        v
5. grammar/expressions/*
        |
        v
6. grammar/statements/*
        |
        v
7. grammar/declarations/*
        |
        v
8. grammar/functions/*
        |
        v
9. grammar/classical/scalar.g4
        |
        v
10. grammar/classical/vector.g4
        |
        v
11. grammar/classical/matrix.g4
        |
        v
12. grammar/classical/tensor.g4
        |
        v
13. grammar/classical/numerical.g4
        |
        v
14. grammar/classical/symbolic.g4
        |
        v
15. grammar/classical/classical-accelerators.g4
        |
        v
16. grammar/classical/classical.g4
        |
        v
17. classical integration tests
        |
        v
18. quantum / hybrid / HDL / hardware integration

"classical.g4" should compose already-defined stable classical components rather than becoming the place where every classical feature is invented.

---

56. Integration invariant

The following invariant must always hold:

General language syntax
        ≠
Classical domain semantics
        ≠
Classical IR
        ≠
Hardware implementation
        ≠
Runtime execution

Each layer has one responsibility.

---

57. Architectural invariant

The following must remain true:

Zamani source
      |
      v
portable semantics
      |
      v
canonical representation
      |
      v
capability/resource analysis
      |
      v
optimization
      |
      v
scheduling
      |
      v
target realization
      |
      v
execution

Not:

Zamani source
      |
      v
machine-specific syntax

---

58. Final design principle

The classical grammar exists to express what computation means, not which machine happens to perform it.

Therefore:

one Zamani program
        |
        v
one semantic meaning
        |
        +----------+----------+----------+----------+
        |          |          |          |          |
       CPU        GPU       FPGA       ASIC      future
        |          |          |          |          |
        +----------+----------+----------+----------+
                           |
                           v
                    available resources

The same principle extends across quantum, HDL, distributed, AI, accelerator, and future computing domains.

---

59. Final objective

"grammar/classical/" must make classical computation a stable, portable, scalable component of:

Zamani — From Atom to Everywhere

while preserving:

- POCO-REAF;
- semantic stability;
- hardware independence;
- classical/quantum interoperability;
- classical/HDL interoperability;
- resource independence;
- deterministic parsing;
- strong diagnostics;
- explicit ownership;
- repository-wide integration;
- extensibility;
- compatibility;
- maintainability;
- no accidental scalability limits;
- no "unsafe";
- Rust 1.97 / 1.97.1 compatibility;
- future computational evolution.

The fundamental invariant is:

«A Zamani classical program describes computation. The compiler, resource system, scheduler, hardware abstraction, and runtime determine how that computation is realized on the resources that actually exist.»