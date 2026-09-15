Zamani Classical Computing Specification

Path: "grammar/spec/classical.md"
Status: Normative
Domain: Classical computation
Language: Zamani
Language-version authority: "grammar/specification/"
Grammar authority: "grammar/Zamani.g4" plus its canonical composed grammar components
Implementation baseline: Rust 1.97 / Rust 1.97.1, Rust 2021
Safety requirement: No "unsafe" Rust
Canonical quantum semantic boundary: "quantum::ir"
Primary classical implementation roots: "src/classical/", canonical classical semantic/IR components already present under "src/quantum/ir/classical/"

---

1. Purpose

This document is the normative specification contract for classical computation in Zamani.

It defines what the classical domain means, which classical concepts are expressible by Zamani source, how those concepts integrate with the universal language, and how they must flow through the frontend, semantic model, IR, compiler, runtime, resource system, scheduling system, hardware system, and interoperability layers.

This specification exists to prevent classical computing from becoming a second programming language embedded inside Zamani.

The classical domain is a semantic domain of the single Zamani language.

It therefore shares the universal:

- lexical system;
- identifiers;
- names and paths;
- expressions;
- types;
- declarations;
- statements;
- functions;
- modules;
- effects;
- memory model;
- concurrency model;
- resource model;
- capability model;
- diagnostics;
- source locations;
- versioning;
- compatibility;
- AST model;
- semantic model;
- IR infrastructure;
- compiler;
- runtime.

Classical computing must integrate with quantum, HDL, AI, distributed computing, networking, data processing, security, and future domains without requiring the programmer to rewrite the algorithm for a different scale or machine.

---

2. Normative status

The key words:

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

If this document conflicts with an implementation detail, the implementation detail is not authoritative.

If this document conflicts with a generated reference, the generated reference is not authoritative.

If this document conflicts with an experimental design document, the experimental document is not authoritative.

The authority hierarchy is:

language specification
        │
        ▼
grammar/spec/*.md
        │
        ▼
canonical grammar composition
        │
        ▼
frontend AST contract
        │
        ▼
semantic model
        │
        ▼
canonical IR
        │
        ▼
compiler / optimizer / scheduler / lowering
        │
        ▼
runtime / HAL / hardware / deployment

"grammar/Zamani-Grammar.md" MAY contain proposals or historical material, but it MUST NOT silently introduce normative classical syntax.

"grammar/grammar.md" MUST describe implementation conformance rather than compete with this document.

---

3. Classical-domain definition

Classical computation in Zamani means computation whose semantic values and operations obey classical computational semantics.

This includes, but is not limited to:

- Boolean computation;
- integer computation;
- arbitrary-width integer computation;
- floating-point computation;
- exact numerical computation;
- decimal computation;
- fixed-point computation;
- rational computation;
- complex-number computation;
- symbolic computation;
- scalar computation;
- vector computation;
- matrix computation;
- tensor computation;
- array computation;
- collection processing;
- sequence processing;
- stream processing;
- numerical analysis;
- scientific computation;
- statistics;
- signal processing;
- control computation;
- optimization;
- symbolic mathematics;
- data processing;
- parallel computation;
- distributed computation;
- accelerator computation;
- heterogeneous computation;
- compile-time computation;
- deterministic computation;
- probabilistic computation;
- stateful computation;
- functional computation;
- imperative computation;
- systems computation;
- embedded computation;
- high-performance computing;
- classical portions of hybrid quantum-classical programs.

This list is extensible.

The language MUST NOT require a new parser-level keyword for every mathematical function, numerical algorithm, processor architecture, accelerator, library, or future computational technique.

---

4. Architectural ownership

4.1 This file owns

This specification owns:

- classical computational meaning;
- classical-domain boundaries;
- classical type semantics;
- classical value semantics;
- classical numeric semantics;
- classical collection semantics;
- classical computational intent;
- classical scalability requirements;
- classical portability requirements;
- classical resource requirements;
- classical capability requirements;
- classical determinism requirements;
- classical parallelism semantics;
- classical accelerator abstraction;
- classical interoperability requirements;
- classical-to-quantum integration requirements;
- classical-to-HDL integration requirements;
- classical-to-resource-system integration;
- classical-to-compiler integration;
- classical-to-runtime integration;
- classical conformance requirements.

4.2 This file does not own

This file does NOT own:

- lexical token definitions;
- Unicode lexical rules;
- global operator spelling;
- global precedence;
- global expression grammar;
- global statement grammar;
- global declaration grammar;
- global function grammar;
- global module grammar;
- global type grammar;
- AST implementation;
- name-resolution implementation;
- type-inference implementation;
- optimizer implementation;
- scheduler implementation;
- hardware selection;
- physical memory layout;
- CPU selection;
- GPU selection;
- FPGA selection;
- accelerator selection;
- network placement;
- physical topology;
- quantum error correction;
- ZQN fault/noise semantics;
- HAL implementation;
- runtime implementation;
- vendor-specific APIs;
- ABI implementation;
- machine-specific register allocation.

Those responsibilities remain downstream.

---

5. Integration with existing classical grammar

The existing:

grammar/classical/classical.g4

is the classical-domain grammar composition boundary.

It MUST remain a thin domain composition layer.

It MUST NOT become a second expression grammar, type grammar, statement grammar, or declaration grammar.

It MAY provide semantic-domain wrappers such as:

classicalConstruct
classicalExpression
classicalValue
classicalType
classicalComputationRegion
classicalCondition
classicalNumericalExpression
classicalCollectionExpression
classicalParallelExpression
classicalDistributedExpression
classicalAcceleratorExpression

where those wrappers are required for parser composition.

The canonical syntax remains owned by the corresponding universal grammar components.

---

6. Integration with classical types

The existing:

grammar/types/classical-types.g4

owns source-level classical type syntax.

It already establishes the correct architectural principle:

source type
    ↓
semantic type
    ↓
canonical IR
    ↓
lowering
    ↓
target realization

Classical type syntax MUST NOT encode hardware implementation.

For example:

Vector<f64, N>
Matrix<f32, Rows, Cols>
Tensor<f64, D0, D1, D2>

describe source-level computational abstractions.

They do not mean:

N registers
Rows × Cols physical memory cells
D0 × D1 × D2 GPU lanes
one particular SIMD width
one particular CPU

---

7. POCO-REAF contract

Zamani classical programs MUST support:

Program Once
Compile Once
Run Everywhere
Anywhere
Forever

subject to the semantic requirements of the program, availability of compatible capabilities/resources, language compatibility, and target support.

The program MUST express computational meaning independently of the machine used to execute it.

The compiler/runtime MAY specialize the program for:

- CPU;
- GPU;
- FPGA;
- ASIC;
- DSP;
- accelerator;
- cluster;
- distributed system;
- edge device;
- cloud system;
- embedded system;
- future computational targets.

The source program MUST NOT need to be rewritten merely because:

- the number of CPUs changes;
- the number of cores changes;
- the number of threads changes;
- the amount of memory changes;
- a GPU becomes available;
- a GPU disappears;
- an FPGA becomes available;
- a different accelerator becomes available;
- the vector width changes;
- the machine topology changes;
- the network topology changes;
- the execution location changes;
- the available parallelism changes;
- the physical storage layout changes.

---

8. Hard-coding prohibition

The language implementation MUST NOT impose universal source-language limits such as:

MAX_CPUS
MAX_CORES
MAX_THREADS
MAX_GPUS
MAX_FPGAS
MAX_ACCELERATORS
MAX_NODES
MAX_MEMORY
MAX_VECTOR_LENGTH
MAX_MATRIX_ROWS
MAX_MATRIX_COLUMNS
MAX_TENSOR_RANK
MAX_TENSOR_DIMENSION
MAX_REGISTER_WIDTH
MAX_WORKGROUP_SIZE
MAX_CACHE_SIZE
MAX_STORAGE_SIZE

as semantic restrictions.

Similarly, the grammar MUST NOT contain universal constructs that imply:

CPU0
CPU1
GPU0
GPU1
core0
core1
node0
node1
register0
register1

as the only model of computation.

Physical identifiers MAY exist in target-specific deployment descriptions, hardware descriptions, diagnostics, or backend metadata.

They MUST NOT become universal assumptions of classical source semantics.

---

9. Resource scalability

Zamani must distinguish:

program semantics
resource requirements
resource availability
resource capabilities
resource constraints
resource preferences
implementation decisions

These concepts MUST NOT be conflated.

For example:

requires memory >= required_memory

is a semantic/resource requirement.

It is not equivalent to:

allocate physical address 0x...

Similarly:

requires capability("vector.compute")

does not mean:

use SIMD width 8

The compiler MAY select any compatible realization.

---

10. Classical values

A classical value is an abstract semantic value.

A value MAY represent:

- Boolean state;
- integer;
- floating-point value;
- decimal;
- fixed-point value;
- rational;
- complex number;
- symbolic expression;
- vector;
- matrix;
- tensor;
- array;
- sequence;
- map;
- set;
- record;
- user-defined value;
- function;
- stream;
- resource handle;
- capability-mediated value;
- future classical value.

Physical representation is not part of the abstract value unless explicitly required by the language semantics.

---

11. Boolean semantics

Boolean values represent classical truth values.

The semantic domain MUST contain at least:

true
false

Boolean operations MUST have well-defined semantics.

The implementation MAY lower Boolean computation to:

- integer operations;
- bit operations;
- vector operations;
- SIMD operations;
- GPU operations;
- FPGA logic;
- distributed operations;
- other compatible representations.

Such lowering MUST preserve observable semantics.

---

12. Integer semantics

Zamani integers MUST be defined semantically rather than by the capabilities of the compiler host.

The language SHOULD support:

- signed integers;
- unsigned integers;
- arbitrary-width integers where supported by the semantic model;
- statically specified widths;
- symbolic/value-parameterized widths where supported;
- checked arithmetic;
- wrapping arithmetic where explicitly requested;
- saturating arithmetic where explicitly requested;
- exact arithmetic where supported;
- conversion operations.

The language MUST NOT silently assume that a source integer is equivalent to:

i32
i64
u32
u64

unless that type is explicitly selected.

A target MAY choose a representation only when the representation preserves the specified semantics.

---

13. Integer overflow

Overflow behavior MUST be explicit or semantically defined.

An implementation MUST NOT silently change overflow semantics merely because the target machine uses a different native integer width.

Supported policies MAY include:

checked
wrapping
saturating
trapping
arbitrary_precision
exact

The exact language spellings belong to the type/expression specifications.

This document owns their semantic distinction.

---

14. Floating-point semantics

Floating-point types represent values governed by the floating-point semantics specified by the corresponding type contract.

The language MUST distinguish:

source precision
source rounding semantics
source exceptional-value semantics
target representation

The target representation is a lowering decision unless the program explicitly requires a particular representation.

Implementations MUST NOT silently replace a specified floating-point semantic model with a different model merely because hardware support differs.

Where exact reproducibility is requested, compiler and runtime transformations MUST preserve the specified reproducibility contract.

---

15. Exact numerical computation

Zamani MAY provide exact numerical types including:

Integer
Rational
Decimal
FixedPoint
Symbolic

Exactness MUST be semantic.

The compiler MUST NOT lower an exact computation into an approximate representation without an explicit semantic conversion or a permitted implementation guarantee.

---

16. Complex numbers

Complex numbers are classical values.

A complex value consists semantically of:

real component
imaginary component

The representation may be:

- software;
- CPU-native;
- vectorized;
- GPU-based;
- FPGA-based;
- accelerator-based;
- distributed.

The source program MUST NOT need to know which representation is selected.

---

17. Vectors

Vectors are ordered classical collections with a homogeneous or otherwise specified element type.

Examples:

Vector<T>
Vector<T, N>

The language MUST support symbolic/vector sizes where the type system permits them.

There is no universal maximum vector length.

Vector length MAY be:

- statically known;
- symbolic;
- runtime-known;
- data-dependent;
- target-specialized.

A vector operation MUST NOT inherently imply a specific SIMD width.

---

18. Matrices

Matrices are classical multidimensional mathematical values with explicitly defined row/column semantics.

Examples:

Matrix<T>
Matrix<T, Rows, Cols>

There is no universal matrix-size limit in the language grammar.

The compiler MAY select:

- tiling;
- blocking;
- vectorization;
- parallelization;
- distributed execution;
- accelerator execution;
- memory layout;
- sparse representation.

Those are implementation decisions.

---

19. Tensors

Tensors are classical multidimensional values.

The language MUST support arbitrary semantic rank subject only to implementation/resource availability.

The grammar MUST NOT establish a fixed maximum tensor rank.

The semantic model MUST permit:

Tensor<T>
Tensor<T, D0>
Tensor<T, D0, D1>
Tensor<T, D0, D1, D2>
...

The ellipsis represents semantic extensibility, not a finite parser-defined limit.

Tensor dimensions MAY be:

- constants;
- symbolic values;
- runtime values;
- dependent values;
- inferred values;
- resource-dependent values.

---

20. Shape semantics

Shape is semantic information.

The language SHOULD permit shape constraints such as:

Rows == Columns
A.cols == B.rows
Tensor.rank >= required_rank

without encoding machine limits.

Shape checking SHOULD occur as early as possible:

parse
  ↓
structural validation
  ↓
type/shape analysis
  ↓
semantic validation

Runtime shape checks MAY remain when compile-time proof is impossible.

---

21. Generic classical computation

Classical abstractions MUST support generic programming.

Generic parameters MAY represent:

- types;
- values;
- dimensions;
- shapes;
- precisions;
- policies;
- capabilities;
- effects;
- computational domains.

Generic arity MUST NOT have a language-level artificial maximum.

---

22. Collections

Classical collection semantics MAY include:

- arrays;
- sequences;
- vectors;
- maps;
- sets;
- streams;
- records;
- tuples;
- user-defined collections.

Collection semantics MUST remain independent of the physical representation.

For example:

Sequence<T>

does not prescribe:

- linked list;
- contiguous array;
- distributed sequence;
- persistent sequence;
- GPU buffer.

---

23. Iteration

Iteration MUST describe semantic traversal rather than a machine execution strategy.

A loop such as:

for item in items

MUST NOT imply:

one CPU core
one thread
serial execution forever

unless sequential semantics are explicitly required.

The compiler MAY transform semantically independent iterations into:

- vector operations;
- task parallelism;
- data parallelism;
- GPU kernels;
- FPGA pipelines;
- distributed execution.

Such transformations MUST preserve observable semantics.

---

24. Parallel classical computation

Classical parallelism is a first-class semantic capability.

The language SHOULD distinguish:

parallelism as semantic intent
parallelism as implementation strategy

A program MAY express:

parallel
data_parallel
task_parallel
pipeline
reduce
map
collective

without specifying a fixed number of workers.

The number of workers is determined by:

available resources
required capabilities
dependencies
scheduling policy
runtime conditions
target characteristics

---

25. Deterministic parallelism

When deterministic semantics are required, the compiler/runtime MUST preserve them.

Parallel execution MUST NOT change program meaning merely because:

- task order changes;
- thread count changes;
- core count changes;
- machine topology changes.

Where floating-point reduction order affects results, the semantic model MUST distinguish:

mathematically equivalent
bitwise reproducible
numerically bounded
implementation-defined

rather than falsely claiming that all parallel reductions are identical.

---

26. Concurrency

Classical concurrency integrates with:

grammar/concurrency/
grammar/spec/concurrency.md

Classical code MAY participate in:

- tasks;
- async operations;
- actors;
- channels;
- futures;
- dataflow;
- pipelines;
- distributed processes.

Concurrency semantics MUST remain independent of machine worker counts.

---

27. Memory semantics

Classical memory is governed by:

grammar/memory/
grammar/spec/type-system.md
grammar/spec/semantics.md
grammar/spec/resources.md

The language MAY express:

- ownership;
- borrowing;
- references;
- allocation;
- regions;
- persistence;
- shared memory;
- distributed memory;
- accelerator memory;
- memory requirements.

The source language MUST NOT require knowledge of:

- physical addresses;
- cache levels;
- NUMA node IDs;
- memory-controller IDs;
- DRAM bank IDs;
- GPU memory addresses.

Those belong downstream.

---

28. Memory scalability

The language MUST support programs whose data requirements range from tiny to extremely large, subject to available resources.

The compiler/runtime MAY reject execution when required resources are unavailable.

Such rejection MUST be a resource/capability result, not a hidden grammar limitation.

For example:

program requires memory M
target provides memory N

is a resource decision.

It MUST NOT become:

grammar only supports values <= X

---

29. Compile-time computation

Zamani MAY evaluate classical expressions during compilation when their semantics permit.

Compile-time evaluation MUST NOT be confused with runtime execution.

A compiler MAY use classical computation for:

- constant folding;
- symbolic evaluation;
- shape resolution;
- specialization;
- verification;
- metaprogramming;
- code generation;
- optimization.

The compiler MUST preserve language semantics.

---

30. Numerical algorithms

Algorithms such as:

- FFT;
- matrix multiplication;
- SVD;
- eigenvalue computation;
- optimization;
- differentiation;
- integration;
- statistics;
- signal processing;

SHOULD generally be represented as semantic operations, library functions, capabilities, or typed operations rather than requiring every algorithm to become a reserved keyword.

The language MAY introduce dedicated syntax only where the construct has language-level semantic meaning.

This prevents unbounded growth of the lexer and parser as new algorithms are invented.

---

31. Symbolic computation

Zamani classical computation MAY represent symbolic values.

Symbolic expressions MUST remain distinguishable semantically from ordinary runtime values where their behavior differs.

Symbolic computation MAY be used for:

- algebra;
- theorem-oriented computation;
- symbolic differentiation;
- symbolic integration;
- compile-time reasoning;
- shape reasoning;
- constraint solving;
- optimization;
- program generation.

---

32. Probabilistic classical computation

Probabilistic computation is classical unless the semantic model explicitly crosses into another domain.

Randomness MUST have explicit semantic treatment.

Where reproducibility is requested, the program MUST be able to specify an appropriate deterministic/reproducible policy without embedding a particular machine RNG implementation.

The language MUST NOT assume:

one RNG
one seed width
one entropy source

as universal architectural limitations.

---

33. Statistics and scientific computing

Classical semantics MAY include:

- distributions;
- samples;
- estimators;
- statistical transformations;
- numerical solvers;
- differential equations;
- signal processing;
- control systems.

These MUST use the common type, expression, function, effect, resource, and capability systems.

They MUST NOT create parallel versions of:

type
expression
function
module
memory
resource

---

34. Classical accelerator abstraction

Zamani MUST treat accelerators as capabilities rather than assumptions.

A classical program MAY require or prefer capabilities such as:

vector.compute
matrix.compute
tensor.compute
parallel.compute
sparse.compute
signal.compute
crypto.compute
ai.compute

The source program SHOULD NOT need to name:

GPU vendor
GPU model
device number
core count
warp width
SIMD width

unless explicitly writing target-specific code.

---

35. Accelerator portability

If a program requires:

capability("tensor.compute")

then the compiler/runtime MAY select:

- CPU;
- GPU;
- FPGA;
- ASIC;
- accelerator;
- distributed implementation;
- software fallback.

The implementation MUST preserve semantics.

A target without the required capability MUST produce a diagnostic or select a valid fallback when one exists.

---

36. Distributed classical computation

Classical computation MAY be distributed.

Distributed semantics belong to:

grammar/distributed/
grammar/spec/distributed.md

A classical computation MUST NOT assume a fixed node count.

Programs MAY describe:

- partitioning;
- replication;
- communication;
- consistency;
- fault tolerance;
- collective operations;
- distributed data;
- distributed tasks.

Placement is a downstream decision unless explicitly specified as part of program semantics.

---

37. Networking integration

Classical programs MAY communicate through:

grammar/networking/

Networking syntax MUST distinguish:

logical endpoint
physical address

where possible.

A portable program SHOULD express logical communication requirements.

Physical addresses belong to deployment/target configuration unless they are explicitly part of the application semantics.

---

38. Distributed scalability

A classical program SHOULD scale from:

one execution context

to:

many execution contexts

without source rewriting when its semantics permit parallel/distributed execution.

The runtime/compiler determines the available degree of parallelism.

---

39. Dataflow

Classical computation MAY be represented as dataflow.

Dataflow constructs SHOULD expose:

- inputs;
- outputs;
- dependencies;
- transformations;
- effects;
- resource requirements.

They SHOULD NOT require a fixed execution graph size.

---

40. Functional computation

Classical computation supports functional constructs through the universal function/expression system.

This includes:

- functions;
- closures;
- lambdas;
- higher-order functions;
- recursion;
- mapping;
- folding;
- composition;
- partial application where supported.

Classical semantics MUST integrate with the universal effect system.

---

41. Imperative computation

Classical computation also supports imperative semantics through the universal statement system.

This includes:

- assignment;
- sequencing;
- branching;
- loops;
- mutation;
- resource management;
- function invocation.

The classical domain MUST NOT create a second imperative grammar.

---

42. Effects

Classical operations MAY carry effects such as:

- mutation;
- I/O;
- allocation;
- communication;
- nondeterminism;
- randomness;
- external interaction;
- timing;
- synchronization.

Effects are specified by:

grammar/spec/effects.md

and implemented through the common semantic effect system.

Classical-specific effects MUST compose with universal effects rather than bypassing them.

---

43. Resource requirements

A classical program MAY declare semantic requirements.

Examples include:

requires memory >= M
requires capability("vector.compute")
requires capability("distributed.compute")
requires capability("exact.arithmetic")
requires capability("tensor.compute")

Requirements MUST be evaluated against target capabilities/resources downstream.

They MUST NOT become parser-level machine assumptions.

---

44. Requirement versus preference

Zamani MUST distinguish:

requirement
constraint
capability
preference
hint
implementation decision

For example:

requires capability("tensor.compute")

is different from:

prefers capability("tensor.compute")

and both are different from:

use physical_device(...)

The first two are portable intent.

The last is target-specific realization.

---

45. Classical hardware independence

The semantic model MUST NOT depend on:

- CPU architecture;
- instruction-set architecture;
- GPU architecture;
- FPGA family;
- ASIC implementation;
- cache structure;
- register file size;
- memory bus width;
- physical clock;
- machine topology.

These MAY influence optimization and lowering.

They MUST NOT redefine the source semantics.

---

46. Hardware-aware optimization

The compiler MAY inspect target information after semantic analysis.

For example:

source
  ↓
semantic classical computation
  ↓
target capabilities
  ↓
optimization
  ↓
vectorization
  ↓
parallelization
  ↓
memory optimization
  ↓
target lowering

The source program remains portable.

---

47. Embedded classical computation

Zamani classical programs MAY target constrained embedded systems.

The source language MUST NOT assume that every target has:

- an operating system;
- a heap;
- virtual memory;
- a filesystem;
- networking;
- dynamic allocation.

Such requirements MUST be represented explicitly when relevant.

The compiler/runtime MAY reject or specialize a program according to target capabilities.

---

48. High-performance computing

Zamani classical computation MUST support HPC-oriented execution without embedding HPC-specific machine sizes.

Potential execution targets include:

- multicore CPU;
- manycore CPU;
- GPU;
- accelerator;
- cluster;
- distributed system;
- heterogeneous system.

The source program describes computation and constraints.

The compiler/runtime determines realization.

---

49. Numerical reproducibility

Where reproducibility is requested, the language MUST provide a semantic mechanism for specifying it.

Possible levels include:

semantic reproducibility
numerical reproducibility
bitwise reproducibility
deterministic scheduling
deterministic reduction

The implementation MUST document which level is guaranteed.

The compiler MUST NOT claim bitwise reproducibility when target-dependent floating-point behavior prevents that guarantee.

---

50. Classical and quantum integration

Classical computation is a first-class component of hybrid programs.

The architecture is:

Zamani source
     │
     ├── classical semantics
     │
     └── quantum semantics
              │
              ▼
         canonical semantic model
              │
              ▼
         classical + quantum IR

The canonical quantum semantic boundary remains:

quantum::ir

The classical grammar MUST NOT create another quantum IR.

---

51. Measurement results

Quantum measurement results become classical values through the existing hybrid/quantum semantic boundary.

Conceptually:

quantum operation
      ↓
measurement
      ↓
classical value
      ↓
classical computation
      ↓
quantum control

The classical specification owns the semantics of the resulting classical value once it crosses the domain boundary.

Quantum measurement semantics remain owned by the quantum specification.

---

52. Classical feed-forward

Classical computation MAY consume quantum-derived values and determine subsequent control.

For example:

measure
    ↓
classical predicate
    ↓
classical decision
    ↓
quantum operation

The grammar MUST use the common expression/statement/control-flow model.

It MUST NOT create a second control-flow language solely for quantum-classical interaction.

---

53. Classical control of quantum computation

The hybrid system MAY allow classical values to control quantum operations.

The division of responsibility is:

classical semantics
    → classical value/control meaning

quantum semantics
    → quantum operation meaning

hybrid semantics
    → boundary and interaction

quantum::ir
    → canonical quantum semantic representation

---

54. Classical and HDL integration

Classical algorithms MAY participate in hardware/software co-design.

The source architecture is:

classical computation
       │
       ├── software intent
       │
       ├── accelerator intent
       │
       └── hardware/co-design intent

HDL syntax remains owned by:

grammar/hdl/

Hardware requirements remain owned by:

grammar/hardware/
grammar/resources/

Classical semantics MUST NOT embed HDL implementation details.

---

55. Classical and AI integration

AI/ML computation is built on classical semantics unless explicitly crossing another domain.

AI features SHOULD reuse:

- tensors;
- arrays;
- functions;
- higher-order operations;
- differentiation;
- optimization;
- data pipelines;
- parallelism;
- distributed computation;
- accelerator capabilities.

AI framework names MUST NOT become core language semantics.

---

56. Classical and data integration

Data operations MUST use the common type/expression/function/resource model.

Classical data computation MAY include:

- records;
- tables;
- arrays;
- tensors;
- streams;
- schemas;
- transformations;
- queries;
- pipelines.

Data storage and execution placement remain downstream concerns.

---

57. Classical and security integration

Classical computation MAY use:

- cryptographic operations;
- secure memory;
- identity;
- authorization;
- secure computation;
- zero-knowledge operations;
- provenance.

Security semantics belong to:

grammar/security/

Classical computation MUST NOT bypass the security/effect/capability model.

---

58. Canonical AST contract

Every classical syntactic construct MUST map into the universal frontend AST.

The AST MUST NOT contain unnecessary machine-specific classical nodes.

A conceptual operation should remain generic where possible:

Operation {
    name,
    namespace,
    operands,
    parameters,
    results,
    attributes,
    modifiers,
    effects,
    capabilities,
    source
}

The exact Rust representation is owned by the frontend AST implementation.

Classical grammar MUST provide enough source information for:

- source spans;
- names;
- operands;
- parameters;
- types;
- attributes;
- modifiers;
- effects;
- domain classification.

---

59. Semantic contract

After parsing, semantic analysis MUST determine:

- name resolution;
- type validity;
- type inference where supported;
- shape validity;
- dimensional consistency;
- numeric compatibility;
- effect correctness;
- ownership/resource correctness;
- capability requirements;
- domain classification;
- classical/quantum boundary validity;
- deterministic/reproducibility requirements;
- resource requirements.

The parser MUST NOT perform these semantic decisions.

---

60. IR contract

Classical syntax MUST lower into the canonical semantic/IR architecture.

The grammar MUST NOT introduce a parallel classical IR merely because a new grammar feature is added.

Where classical IR infrastructure already exists, new features MUST integrate with it.

The existing repository already has canonical classical semantic/IR structures beneath "src/quantum/ir/classical/", including classical bits, assignments, floating-point concepts and other classical representations.

New classical constructs MUST therefore map to the existing canonical architecture rather than creating duplicate representations.

---

61. IR layering

The intended conceptual flow is:

Zamani source
    ↓
lexer
    ↓
parser
    ↓
frontend AST
    ↓
structural validation
    ↓
semantic analysis
    ↓
canonical semantic model
    ↓
classical IR
    ↓
optimization
    ↓
resource analysis
    ↓
parallelization
    ↓
scheduling
    ↓
target lowering
    ↓
runtime

For hybrid programs:

                    ┌── classical IR
                    │
canonical semantics ┤
                    │
                    └── quantum::ir

---

62. Optimization contract

Optimization MUST preserve observable program semantics.

Optimizations MAY include:

- constant folding;
- dead-code elimination;
- common-subexpression elimination;
- algebraic simplification;
- loop transformation;
- vectorization;
- tiling;
- fusion;
- parallelization;
- distribution;
- accelerator mapping;
- memory optimization;
- specialization.

Optimization MUST NOT be encoded as source-language machine assumptions.

---

63. Scheduling contract

Scheduling is downstream.

The scheduler MAY determine:

- execution order;
- parallel degree;
- resource allocation;
- device placement;
- pipeline scheduling;
- accelerator scheduling;
- distributed scheduling.

A classical program MUST NOT need to specify the physical schedule merely to express its computation.

Explicit scheduling syntax MAY exist when timing/order is genuinely semantic.

---

64. Runtime contract

The runtime is responsible for executing the lowered representation.

It MAY:

- allocate resources;
- select available execution resources;
- manage memory;
- manage tasks;
- communicate;
- schedule work;
- interact with accelerators;
- interact with devices;
- collect results;
- report failures.

The grammar MUST NOT embed runtime implementation logic.

---

65. Failure semantics

Resource failure MUST be distinguishable from syntax failure.

Examples:

invalid syntax
invalid type
invalid shape
missing capability
insufficient resource
unsupported target
runtime failure
communication failure

A compiler MUST NOT report a valid portable program as syntactically invalid merely because the selected target lacks resources.

---

66. Resource insufficiency

A valid classical program MAY fail to execute on a target because resources are insufficient.

For example:

required memory > available memory

is not a grammar error.

It is a resource feasibility result.

The compiler/runtime SHOULD provide diagnostics identifying:

- requirement;
- available capability/resource;
- incompatibility;
- possible fallback;
- possible alternative target where known.

---

67. Infinite-scale principle

Zamani uses the term "infinity" as an architectural scalability principle, not as a promise of physically infinite hardware.

A language construct is scalable when its semantics are not artificially bounded by today's machine sizes.

Therefore:

N
Rows
Cols
Rank
Workers
Nodes
Memory
Data size
Parallel degree

MUST remain semantic/resource quantities rather than parser constants.

Actual execution is bounded by available resources and implementation capabilities.

---

68. Tiny-to-large execution

The same semantic program SHOULD be capable of execution on:

microcontroller
embedded processor
single CPU
multicore CPU
manycore system
GPU system
FPGA system
accelerator system
HPC cluster
distributed cloud
heterogeneous system
future architecture

when the target satisfies the program's semantic requirements.

No source rewrite should be required solely because scale changes.

---

69. Resource adaptation

The compiler/runtime MAY adapt:

parallelism
memory layout
tiling
vectorization
task granularity
communication
placement
scheduling
accelerator use

according to available resources.

Adaptation MUST preserve program semantics.

---

70. Capability negotiation

Capability negotiation MUST be separate from source semantics.

For example:

requires capability("tensor.compute")

may be satisfied by:

CPU implementation
GPU implementation
FPGA implementation
ASIC implementation
distributed implementation
software fallback

provided the required semantics are preserved.

---

71. Fallbacks

A program MAY specify fallback behavior where the language supports it.

Fallbacks MUST themselves be portable semantic programs.

Example conceptually:

prefer accelerated computation
fallback to classical software computation

The grammar MUST NOT assume that an accelerator is always available.

---

72. Vendor independence

The classical domain MUST NOT depend on vendor-specific compiler APIs.

Vendor-specific integration belongs to:

interoperability/
hardware/
dialects/
compile/

A vendor extension MUST explicitly declare its:

- syntax;
- semantic extension;
- capability;
- version;
- compatibility;
- lowering path.

---

73. Dialects

Classical dialects MAY extend the language.

A dialect MUST NOT silently alter core classical semantics.

A dialect SHOULD declare:

dialect name
version
syntax extension
semantic extension
AST mapping
IR mapping
capabilities
compatibility

Core Zamani programs MUST remain understandable without importing a dialect unless the dialect feature is explicitly used.

---

74. Interoperability

Classical computation MAY interoperate with:

- C;
- C++;
- Rust;
- Python;
- WebAssembly;
- external numerical libraries;
- foreign ABIs;
- other supported languages/formats.

Interop MUST be represented through the interoperability subsystem.

Foreign representations MUST NOT silently redefine Zamani's classical type semantics.

---

75. ABI independence

Source-level classical types MUST NOT be treated as identical to ABI types.

For example:

Integer

does not automatically mean a particular C ABI integer.

An explicit interoperability boundary MUST define any ABI mapping.

---

76. Unsafe Rust prohibition

The Zamani Rust implementation baseline is:

Rust 1.97 / Rust 1.97.1
edition 2021
safe Rust

No classical grammar implementation requires "unsafe".

Grammar files MUST contain no embedded Rust actions requiring unsafe operations.

Classical frontend/compiler/runtime implementation MUST remain safe Rust unless a future project-wide architectural decision explicitly changes this policy.

---

77. Parser safety

The parser implementation MUST:

- avoid unchecked indexing;
- avoid unchecked arithmetic;
- avoid uncontrolled recursion where practical;
- preserve source spans;
- provide deterministic diagnostics;
- avoid panics for valid user input;
- avoid undefined behavior;
- avoid "unsafe".

Parser resource limits MAY exist as implementation safety controls.

They MUST NOT silently redefine the language's semantic limits.

---

78. Parser resource limits

A parser MAY enforce operational limits for:

- memory exhaustion;
- stack exhaustion;
- compilation budgets;
- timeout budgets;
- denial-of-service protection.

Such limits MUST be:

1. implementation/configuration policy;
2. documented;
3. distinguishable from language semantics;
4. adjustable where appropriate;
5. prevented from becoming portability requirements.

---

79. Determinism

The classical parser and semantic pipeline MUST be deterministic for the same:

source
language version
dialect set
compiler configuration
semantic environment

unless explicit nondeterminism is part of the program semantics.

---

80. Source provenance

Classical semantic objects SHOULD retain provenance/source information sufficient for diagnostics.

At minimum, implementation structures should be able to identify:

- source file;
- source span;
- relevant syntax construct;
- semantic diagnostic location.

This is essential for compiler, tooling, IDE and verification integration.

---

81. Diagnostics

Diagnostics MUST distinguish:

lexical error
syntax error
type error
shape error
semantic error
effect error
resource error
capability error
portability error
interoperability error
target error
runtime error

Diagnostics SHOULD explain:

what happened
where it happened
why it happened
what contract was violated
what resource/capability is missing
what alternatives may exist

---

82. Compatibility

Classical language evolution MUST preserve source compatibility where practical.

Breaking changes MUST be recorded in:

grammar/compatibility/
grammar/spec/compatibility.md

Deprecated constructs MUST remain documented.

The implementation MUST NOT silently reinterpret old classical syntax with a different meaning.

---

83. Versioning

Classical features MUST be versioned through the universal Zamani versioning mechanism.

A classical feature MUST NOT invent an independent versioning scheme.

Experimental features MUST be distinguishable from stable features.

---

84. Feature lifecycle

A new classical feature follows:

proposal
    ↓
semantic design
    ↓
type contract
    ↓
syntax contract
    ↓
AST contract
    ↓
IR contract
    ↓
compiler contract
    ↓
runtime contract
    ↓
tests
    ↓
compatibility review
    ↓
stable

A feature MUST NOT become stable merely because its parser rule exists.

---

85. Feature completion contract

A classical feature is complete only when all of the following exist:

- syntax;
- lexical requirements;
- type contract;
- semantic contract;
- AST mapping;
- source-span mapping;
- diagnostic behavior;
- IR mapping;
- optimization behavior;
- resource behavior;
- capability behavior;
- compiler integration;
- runtime integration;
- tooling integration;
- interoperability behavior where applicable;
- positive tests;
- negative tests;
- boundary tests;
- scalability tests;
- determinism tests;
- compatibility tests;
- hard-coding audit.

---

86. Classical grammar files

The following existing files remain relevant:

grammar/classical/README.md
grammar/classical/classical.g4
grammar/classical/classical-accelerators.g4
grammar/types/classical-types.g4

They MUST reference this specification rather than independently redefine classical semantics.

The current classical grammar's architectural boundary is correct in principle: it delegates lexical tokens, core syntax, types, expressions, statements, declarations and functions to their owning subsystems rather than duplicating them.

---

87. Existing classical type grammar integration

"grammar/types/classical-types.g4" remains the source-level classical type grammar.

It MUST remain responsible for type syntax rather than numerical algorithms.

It already follows the required scalability principle by avoiding fixed:

- dimensions;
- rank;
- vector length;
- matrix dimensions;
- generic arity;
- parameter count;
- numeric width;
- container size.

That principle is normative for all future classical grammar additions.

---

88. Classical implementation integration

The repository currently contains:

src/classical/mod.rs

but it is currently minimal.

Therefore this specification MUST NOT claim that all classical semantics are already implemented.

Implementation work must progressively connect:

grammar
    ↓
frontend AST
    ↓
semantic classical model
    ↓
canonical IR
    ↓
src/classical/
    ↓
compiler
    ↓
runtime

The grammar/specification is therefore the contract that the implementation must satisfy.

---

89. Canonical classical IR integration

The repository already contains classical semantic/IR infrastructure under:

src/quantum/ir/classical/

including canonical classical concepts.

This is important because hybrid quantum-classical computation already needs a classical semantic representation.

New grammar features MUST reuse the canonical IR architecture.

They MUST NOT create:

grammar classical IR
frontend classical IR
quantum classical IR
compiler classical IR

as competing representations.

There should be one authoritative semantic representation at each architectural layer.

---

90. Quantum boundary

Classical semantics MAY appear inside quantum programs.

However:

classical semantics

and:

quantum semantics

must remain distinguishable.

The canonical quantum boundary remains:

quantum::ir

Classical syntax MUST lower into classical semantic constructs before hybrid/quantum lowering when appropriate.

---

91. Scheduling integration

The existing repository contains scheduler-visible classical computation specifically because classical computation can affect quantum execution timing.

The classical specification therefore permits semantic metadata such as:

latency-sensitive
real-time
feed-forward
deadline
ordering
dependency

where such constructs are genuinely part of the language.

The scheduler remains responsible for actual scheduling.

---

92. Optimization integration

The optimizer MAY specialize classical computation based on:

- target architecture;
- available resources;
- capabilities;
- data shapes;
- numerical properties;
- parallelism;
- locality;
- accelerator availability.

The optimizer MUST NOT modify source semantics.

---

93. Verification integration

Classical programs MAY be subject to:

- type verification;
- shape verification;
- effect verification;
- resource verification;
- determinism verification;
- numerical verification;
- contract verification;
- formal verification.

The grammar SHOULD preserve sufficient structure for downstream verification.

---

94. Security integration

Classical computations involving security-sensitive values SHOULD be able to carry semantic security information.

Examples include:

secret
confidential
public
authenticated
integrity_protected

Exact syntax belongs to "grammar/security/".

The classical specification defines their interaction with classical computation, not their cryptographic implementation.

---

95. Observability

Classical execution MAY expose:

- metrics;
- tracing;
- profiling;
- logging;
- provenance;
- diagnostics.

Instrumentation MUST NOT change program semantics except where the program explicitly requests observable instrumentation behavior.

---

96. Testing requirements

The classical grammar/specification MUST be tested through:

grammar/tests/classical/

and corresponding compiler/frontend tests.

The test suite MUST include:

positive/
negative/
boundary/
scalability/
determinism/
compatibility/

---

97. Positive tests

Positive tests MUST cover at least:

Boolean
integers
unsigned integers
floating-point
decimal
fixed-point
rational
complex
vectors
matrices
tensors
arrays
maps
sets
sequences
functions
generic functions
loops
conditionals
parallel computation
dataflow
compile-time computation
symbolic computation
distributed computation
accelerator intent
resource requirements
hybrid classical/quantum interaction
HDL/software co-design

---

98. Negative tests

Negative tests MUST cover:

- invalid types;
- invalid dimensions;
- invalid shape relationships;
- invalid conversions;
- invalid effects;
- invalid resource requirements;
- invalid capability requirements;
- invalid classical/quantum boundaries;
- invalid ownership;
- invalid interoperability;
- invalid dialect usage.

---

99. Boundary tests

Boundary tests MUST include:

empty collections
single-element collections
large symbolic dimensions
zero dimensions where legal
nested generics
deep namespace paths
large expression trees
large tensor ranks
large parameter lists
large collection expressions
large parallel regions

The test suite MUST distinguish legitimate large programs from implementation resource exhaustion.

---

100. Scalability tests

Scalability tests MUST verify that language semantics do not contain artificial limits.

Tests SHOULD use parameterized families rather than merely testing one large constant.

For example:

N = small
N = medium
N = large
N = symbolic
N = runtime-derived

The grammar itself MUST not require a new rule for each size.

---

101. Determinism tests

The implementation MUST test that the same source and semantic environment produce equivalent semantic results.

Where bitwise reproducibility is promised, tests MUST verify bitwise behavior.

Where only mathematical/numerical equivalence is promised, tests MUST use the appropriate tolerance or equivalence model.

---

102. Hard-coding audit

Every classical grammar change MUST be checked for:

MAX_
MIN_
CPU
GPU
FPGA
QPU
CORE
THREAD
LANE
REGISTER
CACHE
MEMORY
NODE
DEVICE
ADDRESS

These words are not automatically forbidden.

The audit determines whether they are:

semantic concepts
target concepts
resource concepts
implementation constants

Universal implementation constants MUST NOT leak into source semantics.

---

103. No artificial machine assumptions

The following MUST NOT be universal classical language assumptions:

CPU count
core count
thread count
register count
register width
SIMD width
cache size
RAM size
GPU count
GPU memory
FPGA resources
cluster node count
network link count
storage capacity

They belong to resource/target descriptions.

---

104. Infinite extensibility

The classical domain MUST remain open to future:

- numerical types;
- computing models;
- accelerators;
- memory systems;
- parallel architectures;
- distributed architectures;
- data models;
- scientific algorithms.

New capability MUST be integrable without rewriting the entire language grammar.

---

105. Generic operation model

Where a new operation does not require special language semantics, Zamani SHOULD prefer:

operation name
operands
parameters
results
attributes
effects
capabilities

over adding a new keyword.

This keeps the grammar stable while allowing the semantic/standard library ecosystem to grow.

---

106. Intrinsics

An intrinsic MAY provide compiler-recognized semantics for a classical operation.

An intrinsic MUST declare:

- type contract;
- semantic contract;
- effects;
- required capabilities;
- resource behavior;
- lowering behavior;
- fallback behavior where applicable;
- compatibility status.

The intrinsic mechanism MUST NOT become a hidden source of machine limits.

---

107. Standard library boundary

Classical algorithms that do not require language-level semantics SHOULD normally live in the standard library rather than the grammar.

Examples:

fft
svd
matrix_multiply
sort
search
statistics
optimization
signal_processing

The grammar defines the ability to invoke such operations.

The library defines the operation.

The compiler MAY optimize recognized operations.

---

108. Language-level operations

An operation deserves dedicated language-level syntax when it affects:

- parsing structure;
- binding;
- control flow;
- ownership;
- effects;
- type semantics;
- resource semantics;
- domain boundaries;
- compile-time semantics;
- interoperability semantics.

Otherwise it SHOULD remain an ordinary operation/intrinsic/library facility.

---

109. Classical resource abstraction

Classical resource requirements MUST be abstract.

Examples:

memory
compute
parallelism
storage
bandwidth
latency
energy
reliability
precision
capability

These are semantic requirements.

Physical realization remains downstream.

---

110. Resource negotiation

The compiler/runtime MAY negotiate resource realization.

For example:

requested capability
       ↓
available targets
       ↓
candidate realization
       ↓
resource feasibility
       ↓
optimization
       ↓
execution

The source program remains unchanged.

---

111. Graceful degradation

When a preference cannot be satisfied, the implementation MAY choose an alternative.

When a mandatory requirement cannot be satisfied, execution MUST fail clearly.

The implementation MUST NOT silently violate a mandatory semantic requirement.

---

112. Classical computation across time

Programs SHOULD remain valid across future machines provided:

- the language semantics remain compatible;
- required capabilities remain available or have valid fallbacks;
- dependencies remain compatible;
- the program does not explicitly require a removed target-specific facility.

This is part of POCO-REAF.

---

113. Source stability

A target upgrade MUST NOT require source rewriting merely because:

CPU → newer CPU
GPU → newer GPU
FPGA → newer FPGA
cluster → larger cluster
single-node → distributed
accelerator A → accelerator B

when the required semantic capabilities remain satisfiable.

---

114. Classical domain completion criteria

"grammar/spec/classical.md" is complete only when:

- classical semantics are defined;
- classical types are integrated;
- expressions are integrated;
- statements are integrated;
- functions are integrated;
- effects are integrated;
- memory is integrated;
- concurrency is integrated;
- resources are integrated;
- capabilities are integrated;
- distributed computing is integrated;
- accelerators are integrated;
- data is integrated;
- AI is integrated;
- networking is integrated;
- security is integrated;
- quantum/hybrid boundaries are integrated;
- HDL boundaries are integrated;
- AST mapping exists;
- semantic mapping exists;
- canonical IR mapping exists;
- compiler integration exists;
- runtime integration exists;
- diagnostics exist;
- compatibility rules exist;
- positive tests exist;
- negative tests exist;
- boundary tests exist;
- scalability tests exist;
- determinism tests exist;
- hard-coding audit passes;
- safe-Rust implementation requirements are satisfied.

---

115. Required integration matrix

The implementation MUST maintain the following conceptual mapping:

Layer| Classical responsibility
"grammar/spec/classical.md"| normative classical semantics
"grammar/classical/"| classical grammar composition
"grammar/types/classical-types.g4"| classical type syntax
"grammar/expressions/"| universal expression syntax
"grammar/statements/"| universal statement syntax
"grammar/functions/"| universal function syntax
"grammar/memory/"| memory semantics
"grammar/concurrency/"| concurrency semantics
"grammar/resources/"| resources/capabilities
"grammar/hardware/"| target capabilities
"grammar/quantum/"| quantum semantics
"grammar/hybrid/"| classical/quantum interaction
"grammar/hdl/"| hardware description
frontend AST| syntax representation
semantic analyzer| classical meaning
canonical IR| machine-independent semantics
"src/classical/"| classical implementation
"src/quantum/ir/classical/"| canonical classical IR components used by the quantum/hybrid architecture
optimizer| implementation improvement
scheduler| execution ordering/resources
compiler| lowering
runtime| execution
HAL| hardware interaction
tests| conformance

---

116. Integration rule for future files

Every new classical file MUST explicitly document:

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
Determinism Tests
Compatibility Tests
Hard-Coding Audit
Security Considerations
Performance Considerations
Completion Criteria

A file MUST NOT depend on an undocumented future contract.

---

117. Independent-completion rule

A classical file is independently complete when another contributor can implement or modify it without requiring an architectural decision to be invented later.

Before marking a file complete, its:

- inputs;
- outputs;
- ownership;
- AST mapping;
- semantic mapping;
- IR mapping;
- downstream consumers;
- test obligations

must already be defined.

This prevents:

grammar first
AST later
semantic model later
IR later
rewrite grammar

The desired process is:

contract first
     ↓
grammar
     ↓
AST
     ↓
semantic model
     ↓
IR
     ↓
implementation

with the contracts agreed before implementation begins.

---

118. Repository-wide integration dependencies

This document depends on:

grammar/spec/lexical.md
grammar/spec/syntax.md
grammar/spec/type-system.md
grammar/spec/semantics.md
grammar/spec/effects.md
grammar/spec/resources.md
grammar/spec/compatibility.md
grammar/spec/quantum.md
grammar/spec/hybrid.md
grammar/spec/concurrency.md
grammar/spec/distributed.md

and integrates with:

grammar/classical/
grammar/types/
grammar/expressions/
grammar/statements/
grammar/functions/
grammar/memory/
grammar/resources/
grammar/hardware/
grammar/compile/
grammar/execution/
grammar/quantum/
grammar/hybrid/
grammar/hdl/
grammar/ai/
grammar/data/
grammar/networking/
grammar/security/
grammar/interoperability/
grammar/dialects/
grammar/validation/
grammar/tests/

Implementation consumers include:

src/lexer.rs
src/parser.rs
src/frontend/
src/classical/
src/quantum/
src/compiler/
src/runtime/
src/hardware/
src/distributed/
src/hdl/
src/ir/

where applicable.

---

119. No duplicate authority

The following MUST remain distinct:

grammar/spec/classical.md
    = normative classical semantic contract

grammar/classical/*.g4
    = classical syntax composition

grammar/types/classical-types.g4
    = classical type syntax

frontend AST
    = syntax representation

semantic model
    = interpreted meaning

IR
    = compiler semantic representation

runtime
    = execution

No file may silently become another authority.

---

120. Production-readiness gate

Classical computing is production-ready only when the following statement can truthfully be made:

«A valid Zamani classical program expresses computational meaning independently of a particular machine, can be parsed deterministically, represented in the canonical AST, semantically validated, lowered into the canonical IR architecture, optimized and scheduled according to available capabilities/resources, executed safely by the runtime, and tested across small, large, heterogeneous, distributed and future-compatible targets without universal grammar-level hardware limits.»

Until that is true, the classical domain remains partially implemented regardless of how extensive "classical.g4" becomes.

---

121. Final architectural invariant

The central invariant is:

CLASSICAL SYNTAX
        ↓
CLASSICAL SEMANTICS
        ↓
MACHINE-INDEPENDENT REPRESENTATION
        ↓
RESOURCE/CAPABILITY ANALYSIS
        ↓
OPTIMIZATION
        ↓
SCHEDULING
        ↓
TARGET LOWERING
        ↓
RUNTIME
        ↓
ACTUAL MACHINE

Never:

CLASSICAL SYNTAX
        ↓
CPU/GPU/FPGA ASSUMPTIONS
        ↓
FIXED MACHINE LIMITS

And for hybrid computation:

                    ┌── Classical semantics
                    │
Zamani source ──────┤
                    │
                    └── Quantum semantics
                              ↓
                         quantum::ir
                              ↓
                    optimization / routing /
                    scheduling / resilience / ZQN
                              ↓
                             HAL
                              ↓
                         target hardware

The classical domain MUST remain a first-class computational domain without becoming a separate language.

The ultimate invariant is therefore:

One Zamani language
        +
One universal semantic architecture
        +
Classical + Quantum + HDL + Hybrid + Future domains
        +
Capability/resource-based realization
        +
No artificial machine limits
        +
Safe Rust implementation
        =
POCO-REAF

This document is the normative classical-domain contract. All future classical grammar, AST, semantic, IR, compiler, runtime, resource, hardware and test work MUST trace back to this contract.