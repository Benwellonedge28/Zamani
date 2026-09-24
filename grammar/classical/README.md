Zamani Classical Computing Grammar

Path: "grammar/classical/README.md"
Language: Zamani
Domain: Classical computation
Grammar technology: ANTLR4
Implementation baseline: Rust 2021, Rust 1.97 / Rust 1.97.1
Safety requirement: Safe Rust only; Rust "unsafe" is prohibited
Primary portability objective: "Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever" (POCO-REAF)

---

1. Purpose

This document defines the production architecture, ownership boundaries, integration contracts, scalability rules, and completion requirements for the classical-computing grammar domain:

grammar/classical/

Classical computing is a semantic domain of the single Zamani language.

It is not a separate programming language.

The classical grammar must therefore integrate with the existing Zamani architecture rather than create a second:

- lexer;
- expression language;
- type system;
- declaration system;
- statement system;
- module system;
- semantic model;
- AST hierarchy;
- intermediate representation;
- compiler;
- runtime;
- hardware abstraction layer.

The classical domain must be capable of expressing computation ranging from extremely small systems to arbitrarily large systems, subject to the resources and capabilities actually available at compilation or execution time.

The governing principle is:

«The Zamani source program describes computation and its semantic requirements. The implementation determines how that computation is realized on available resources.»

This document is subordinate to the repository-wide architectural authority in:

grammar/DESIGN.md

It provides the classical-domain contract required by that architecture.

---

2. Authority Model

The classical grammar participates in the following authority hierarchy:

grammar/DESIGN.md
        |
        v
language specification
        |
        v
grammar/spec/
grammar/specification/
        |
        v
canonical lexical contracts
        |
        v
grammar/Zamani.g4
        |
        v
classical grammar composition
        |
        v
Rust lexer/parser implementation
        |
        v
domain-neutral frontend AST
        |
        v
semantic analysis
        |
        v
canonical semantic/IR representation
        |
        v
optimization
        |
        v
resource/capability analysis
        |
        v
scheduling / placement / distribution
        |
        v
target lowering
        |
        v
runtime / HAL / hardware

The repository-wide files retain their established roles:

File| Authority
"grammar/DESIGN.md"| Normative repository-wide grammar architecture
"grammar/README.md"| Grammar navigation and authority model
"grammar/Zamani.g4"| Canonical ANTLR composition/root grammar
"grammar/grammar.md"| Implementation-conformance reference
"grammar/Zamani-Grammar.md"| Historical/extended design reference
"grammar/classical/README.md"| Normative classical-domain architecture
"grammar/classical/*.g4"| Classical-domain syntax components
"grammar/classical/intrinsics.md"| Classical intrinsic semantic contract
"grammar/spec/*"| Formal language contracts
"grammar/tests/*"| Conformance evidence

Nothing in this README overrides "grammar/DESIGN.md".

---

3. Production Objective

The classical grammar must support one semantic program being realized across fundamentally different computational environments.

Conceptually:

                         ONE ZAMANI PROGRAM
                                |
                                v
                    DOMAIN-INDEPENDENT SEMANTICS
                                |
                    +-----------+-----------+
                    |                       |
                    v                       v
             Classical computation    Other domains
                    |                 quantum / HDL / AI /
                    |                 distributed / etc.
                    +-----------+-----------+
                                |
                                v
                     RESOURCE + CAPABILITY
                         ANALYSIS
                                |
                                v
                    TARGET-INDEPENDENT IR
                                |
                                v
                       OPTIMIZATION
                                |
             +------------------+------------------+
             |                  |                  |
             v                  v                  v
         scheduling          placement          lowering
             |                  |                  |
             +------------------+------------------+
                                |
                                v
                       TARGET REALIZATION
                                |
          +----------+----------+----------+----------+
          |          |          |          |          |
          v          v          v          v          v
         CPU        GPU        FPGA       ASIC       Future
          |          |          |          |        target
          +----------+----------+----------+----------+
                                |
                                v
                             RUNTIME

The source language must remain above the realization boundary.

---

4. POCO-REAF Contract

The classical domain participates in:

Program Once
     |
Compile Once
     |
Semantic representation
     |
Target/resource negotiation
     |
Target realization
     |
Run Everywhere
     |
Run Anywhere
     |
Future-compatible execution

POCO-REAF does not mean that one physical machine-code binary must magically execute natively on every architecture.

It means that the Zamani program's semantic intent remains portable, while target-specific realization may be selected, generated, specialized, or adapted later.

The language must therefore separate:

WHAT the program means

from:

HOW a particular environment realizes it

---

5. Absolute Scalability Rule

The classical grammar MUST NOT impose arbitrary machine or problem-size limits.

The grammar must not define universal limits such as:

MAX_ELEMENTS
MAX_VECTOR_LENGTH
MAX_MATRIX_ROWS
MAX_MATRIX_COLUMNS
MAX_TENSOR_RANK
MAX_TENSOR_DIMENSIONS
MAX_THREADS
MAX_CORES
MAX_CPUS
MAX_GPUS
MAX_FPGAS
MAX_ACCELERATORS
MAX_DEVICES
MAX_MEMORY
MAX_NODES
MAX_WORKERS
MAX_PROCESSES
MAX_TASKS
MAX_REGISTER_WIDTH
MAX_SIMD_WIDTH

Nor may equivalent limits be hidden under different names.

The following are prohibited as language-wide grammar restrictions:

vector length <= fixed_value
matrix rows <= fixed_value
matrix columns <= fixed_value
tensor rank <= fixed_value
threads <= fixed_value
nodes <= fixed_value
devices <= fixed_value

ANTLR grammar repetition must remain structurally open:

item*
item+
item?

rather than introducing arbitrary bounded repetition such as:

item{1,32}

unless the finite cardinality is itself an actual language semantic requirement.

---

6. Program Values Are Not Hardware Limits

The grammar must distinguish:

program-defined quantity

from:

implementation-defined resource capacity

For example:

let n = 1024;

is ordinary program semantics.

Likewise:

vector<n>
matrix<rows, columns>
tensor<shape>

may represent program-level dimensions.

They must not imply:

compiler maximum = n

or:

hardware capacity = n

The distinction is:

program shape
    !=
resource capacity
    !=
hardware execution width

---

7. What Classical Computing Owns

"grammar/classical/" owns source syntax that is genuinely specific to classical computational domains.

This includes, where standardized by the Zamani language:

- scalar-domain syntax;
- integer-domain syntax;
- floating-point-domain syntax;
- vector-domain syntax;
- matrix-domain syntax;
- tensor-domain syntax;
- numerical-computing syntax;
- symbolic-computing syntax;
- linear-algebra domain syntax;
- optimization-intent syntax;
- signal-processing domain syntax;
- statistics-domain syntax;
- scientific-computing syntax;
- control-system syntax;
- classical accelerator intent;
- classical-domain grammar composition;
- classical intrinsic documentation;
- classical grammar conformance contracts.

---

8. What Classical Computing Does Not Own

The classical directory does not own:

- lexical token definitions;
- Unicode rules;
- identifiers;
- names;
- qualified names;
- paths;
- comments;
- general literals;
- general operators;
- precedence;
- associativity;
- general expressions;
- assignment syntax;
- indexing syntax;
- general function calls;
- declarations;
- classes;
- traits;
- modules;
- imports;
- general statements;
- ownership;
- borrowing;
- lifetimes;
- generic type syntax;
- effect semantics;
- general concurrency semantics;
- resource discovery;
- hardware discovery;
- physical placement;
- scheduling algorithms;
- routing algorithms;
- optimization algorithms;
- runtime execution;
- hardware drivers;
- HAL implementation;
- QEC;
- ZQN;
- quantum IR;
- device calibration;
- vendor library implementation.

Those belong to their existing repository owners.

---

9. The Most Important Boundary: Syntax vs Semantics

The parser must not attempt to determine whether an expression is computationally:

- scalar;
- vector;
- matrix;
- tensor;
- symbolic;
- statistical;
- numerical;
- distributed;
- accelerator-backed;
- scientific;
- control-oriented.

For example:

x + y

remains an ordinary Zamani expression.

Semantic analysis determines the types and domain semantics of "x" and "y".

Likewise:

f(A, B)

must not require a parser-level catalogue containing every possible mathematical algorithm.

The semantic system can determine whether "f" denotes:

- matrix multiplication;
- tensor contraction;
- a user-defined function;
- an intrinsic;
- a library operation;
- a symbolic transformation;
- an accelerator operation;
- a future operation.

This is essential for an open-ended computing language.

---

10. Open-World Classical Operation Model

Classical computation must not be represented by an exhaustive grammar enumeration.

Avoid architectures equivalent to:

classicalOperation
    : ADD
    | SUBTRACT
    | FFT
    | SVD
    | CHOLESKY
    | GRADIENT_DESCENT
    | ...
    ;

That approach makes the language depend on today's inventory of algorithms.

Instead, operation invocation should normally use the canonical Zamani expression/call model.

Conceptually:

add(x, y)

linalg::matmul(A, B)

tensor::contract(A, B)

signal::transform(signal)

statistics::mean(values)

optimize::solve(problem)

future::operation(value)

The grammar recognizes the structure.

Semantic analysis determines:

- whether the operation exists;
- its signature;
- argument compatibility;
- return type;
- effects;
- capabilities;
- resource requirements;
- determinism;
- purity;
- lowering;
- diagnostics.

This is an open-world operation model.

---

11. Grammar Explosion Prevention

The classical grammar must not become a dictionary of mathematics.

Operations such as:

sin
cos
tan
exp
log
sqrt
abs
min
max
mean
variance
fft
svd
inverse
solve
gradient

should normally be represented as:

ordinary function
+
typed operation
+
intrinsic
+
standard-library capability

rather than becoming new parser keywords.

A dedicated grammar production is justified only when the construct has language-level syntax or semantics that cannot appropriately be represented by existing generic constructs.

The decision test is:

1. Is the construct genuinely language-level?
2. Does it have standardized Zamani semantics?
3. Does ordinary expression/call syntax fail to express its required semantics?
4. Does the compiler need structural knowledge beyond ordinary call resolution?
5. Is the semantic contract stable?
6. Does making it syntax improve correctness rather than merely convenience?

If the answer is no, prefer an intrinsic or library operation.

---

12. Existing Classical Directory

The actual repository currently contains the following classical files:

grammar/classical/
├── README.md
├── arithmetic.g4
├── classical-accelerators.g4
├── classical.g4
├── control.g4
├── floating-point.g4
├── integer.g4
├── intrinsics.md
├── linear-algebra.g4
├── matrix.g4
├── numeric.g4
├── numerical.g4
├── optimization.g4
├── scalar.g4
├── scientific-computing.g4
├── signal-processing.g4
├── statistics.g4
├── symbolic.g4
├── tensor.g4
└── vector.g4

These existing filenames should be retained.

No unnecessary renaming is required.

No parallel "classical2/", "math/", "numerics/", or replacement classical grammar tree should be created.

The current files must instead be reconciled under the ownership model defined here.

---

13. Classical File Ownership Matrix

File| Primary responsibility
"classical.g4"| Classical-domain composition boundary
"arithmetic.g4"| Classical arithmetic integration boundary
"integer.g4"| Integer-domain parser boundary
"floating-point.g4"| Floating-point parser boundary
"scalar.g4"| Scalar-domain integration
"vector.g4"| Vector-domain syntax
"matrix.g4"| Matrix-domain syntax
"tensor.g4"| Tensor-domain syntax
"numeric.g4"| General numeric-domain composition
"numerical.g4"| Numerical-computation domain
"linear-algebra.g4"| Linear-algebra integration
"optimization.g4"| Classical optimization intent
"signal-processing.g4"| Signal-processing domain
"statistics.g4"| Statistical-computation domain
"symbolic.g4"| Symbolic-computation domain
"scientific-computing.g4"| Scientific-computing domain
"control.g4"| Classical control-system domain
"classical-accelerators.g4"| Portable classical accelerator intent
"intrinsics.md"| Classical intrinsic semantic contract

No file may silently acquire another file's ownership.

---

14. "classical.g4"

Purpose

"classical.g4" is the classical-domain composition boundary.

It is not a second Zamani root grammar.

It must compose classical-domain syntax while reusing canonical language rules.

Owns

- classical domain entry points;
- classical-domain composition;
- classical computation regions where explicitly standardized;
- classical-domain operation boundaries;
- references to classical specialized grammar components.

Does not own

- general expressions;
- operators;
- precedence;
- identifiers;
- types;
- declarations;
- statements;
- modules;
- lexical tokens;
- algorithms.

Integration

Zamani.g4
      |
      v
canonical parser composition
      |
      v
classical.g4
      |
      +--> scalar.g4
      +--> vector.g4
      +--> matrix.g4
      +--> tensor.g4
      +--> numerical.g4
      +--> symbolic.g4
      +--> ...
      |
      v
domain-neutral AST

The exact ANTLR import/delegation mechanism must match the repository's canonical parser architecture.

There must be exactly one authoritative path into the classical domain.

AST Contract

Classical-specific syntax must lower into the existing domain-neutral frontend AST.

It must not create an independent classical AST hierarchy.

Semantic Contract

Semantic analysis classifies the resulting structures as classical computations.

IR Contract

This grammar produces no IR.

Completion Criteria

"classical.g4" is complete only when:

- its ownership boundary is unambiguous;
- all specialized classical grammar components have known integration points;
- no duplicate general grammar exists;
- no circular grammar dependency exists;
- all referenced parser rules have authoritative owners;
- canonical parser composition succeeds;
- AST mapping is defined;
- semantic mapping is defined;
- IR integration is defined;
- positive/negative/boundary/scalability tests exist.

---

15. "arithmetic.g4"

Purpose

Provides the classical-domain arithmetic integration boundary.

General arithmetic syntax remains owned by the canonical expression grammar.

Owns

Only arithmetic constructs requiring a genuine classical-domain boundary.

Does not own

- arithmetic tokenization;
- arithmetic precedence;
- general operators;
- numeric literals;
- general expression parsing;
- arithmetic algorithms.

Integration

canonical expression
        |
        v
arithmetic domain
        |
        v
semantic numeric operation
        |
        v
canonical IR

Scalability

No fixed arithmetic width may be assumed.

Do not equate arithmetic with:

32-bit
64-bit
128-bit

unless an explicit Zamani type defines that width semantically.

---

16. "integer.g4"

Purpose

Defines the parser-level classical integer boundary.

Owns

- integer-domain parser classification;
- integer-specific source constructs if standardized.

Does not own

- lexical integer spelling;
- digit scanning;
- arbitrary numeric magnitude rules;
- integer storage width;
- machine register width;
- overflow implementation;
- target instruction selection.

Lexical ownership remains with the canonical lexer.

Scalability

The grammar must not impose a maximum integer magnitude merely because a particular backend has a native integer width.

Semantic integer types may explicitly define widths where Zamani specifies such types.

Those semantic widths must not be confused with hardware register widths.

Integration

lexer
  |
  v
integer syntax
  |
  v
domain-neutral AST
  |
  v
integer semantic type
  |
  v
canonical IR
  |
  v
target lowering

---

17. "floating-point.g4"

Purpose

Defines parser-level floating-point-domain syntax.

Owns

- parser-level floating-point constructs;
- integration with canonical floating-point lexer tokens.

Does not own

- floating-point lexical scanning;
- machine floating-point width;
- hardware register width;
- IEEE implementation details unless part of language semantics;
- rounding implementation;
- numerical stability algorithms;
- target instruction selection.

Semantic ownership

Semantic analysis owns:

- precision;
- rounding semantics;
- overflow;
- underflow;
- NaN/infinity behavior;
- conversions;
- compatibility.

The grammar must not assume that all targets implement the same native floating-point formats.

---

18. "scalar.g4"

Purpose

Defines the classical scalar-domain boundary where a distinct parser boundary is justified.

Does not own

- general literals;
- general arithmetic;
- boolean syntax;
- general declarations;
- type-system implementation.

Integration

scalar syntax
      |
      v
canonical types
      |
      v
semantic scalar classification
      |
      v
canonical IR

Completion requirement

Any rule that merely aliases "expression" or "typeExpression" without creating a meaningful parser boundary must be removed or consolidated.

---

19. "vector.g4"

Purpose

Defines vector-specific syntax.

Owns

Where standardized:

- vector literals;
- vector construction;
- vector-domain shape syntax;
- vector-specific structural forms.

Does not own

- generic arrays;
- generic indexing;
- general slicing;
- generic expressions;
- numeric literals;
- SIMD implementation.

Scalability

Never define:

MAX_VECTOR_LENGTH

or any equivalent.

Vector dimensions may be:

- constant;
- symbolic;
- generic;
- runtime-derived;
- dependent where supported by the type system.

Critical distinction

vector length
    !=
hardware SIMD width

A vector of length "n" is a program-level semantic object.

The compiler may implement it using:

- scalar execution;
- vector instructions;
- multiple vector instructions;
- GPU execution;
- FPGA pipelines;
- distributed execution;
- another future mechanism.

The grammar must remain neutral.

---

20. "matrix.g4"

Purpose

Defines matrix-specific syntax.

Owns

- matrix construction;
- matrix literals where standardized;
- matrix shape declarations;
- matrix-specific structural forms.

Does not own

- generic indexing;
- multiplication operators;
- numerical algorithms;
- BLAS/LAPACK;
- CUDA;
- GPU selection;
- memory layout implementation.

Scalability

No:

MAX_MATRIX_ROWS
MAX_MATRIX_COLUMNS

or equivalent.

Matrix dimensions are semantic values.

They may be:

constant
symbolic
generic
runtime-derived
dependent

where supported.

Integration

matrix syntax
     |
     v
type/shape analysis
     |
     v
semantic matrix operation
     |
     v
canonical classical representation
     |
     v
optimization
     |
     v
target lowering

---

21. "tensor.g4"

Purpose

Defines tensor-domain syntax.

Owns

- tensor construction;
- tensor literals where standardized;
- tensor shape syntax;
- tensor dimension lists;
- tensor-domain structural forms.

Does not own

- generic collection syntax;
- generic indexing;
- tensor libraries;
- accelerator selection;
- storage allocation;
- memory layout implementation;
- SIMD width.

Scalability

There must be no grammar-level:

MAX_TENSOR_RANK
MAX_TENSOR_DIMENSIONS

or equivalent.

The grammar must support structurally represented shape information.

Semantic separation

The implementation must distinguish:

tensor rank
tensor shape
tensor storage layout
tensor allocation
tensor execution strategy
tensor device
tensor parallelization

These are not the same property.

---

22. "numeric.g4"

Purpose

General numerical-domain composition.

It provides a stable domain boundary without duplicating the expression grammar.

Owns

- numerical-domain entry points;
- numerical classification boundaries;
- numerical-domain composition.

Does not own

- every mathematical function;
- numeric lexical syntax;
- arithmetic precedence;
- numerical algorithm implementation;
- hardware selection.

Integration

canonical expression
        |
        v
numeric domain
        |
        v
semantic numeric analysis
        |
        v
canonical IR

---

23. "numerical.g4"

Purpose

Defines source-level numerical-computation intent where specialized syntax is genuinely justified.

Potential semantic domains include:

- numerical transformations;
- numerical integration;
- numerical differentiation;
- interpolation;
- numerical solving;
- Fourier-domain operations;
- numerical analysis;
- scientific numerical workflows.

Critical rule

It must not become a keyword catalogue.

Prefer:

fft(x)
solve(problem)
integrate(f, domain)
differentiate(f, x)

through canonical call/operation syntax when dedicated syntax is unnecessary.

The compiler may recognize these through the intrinsic system without making them permanent language keywords.

---

24. "linear-algebra.g4"

Purpose

Defines the classical linear-algebra integration boundary.

Owns

- linear-algebra domain classification;
- standardized structural syntax where needed;
- integration of vector/matrix/tensor semantic operations.

Does not own

- matrix grammar;
- vector grammar;
- tensor grammar;
- general operators;
- BLAS;
- LAPACK;
- vendor libraries;
- CPU instruction sets;
- GPU instruction sets.

Integration

vector/matrix/tensor syntax
          |
          v
linear algebra semantics
          |
          v
canonical representation
          |
          v
optimization
          |
          +--> CPU
          +--> GPU
          +--> FPGA
          +--> accelerator
          +--> distributed
          +--> future target

---

25. "optimization.g4"

Purpose

Defines classical optimization intent, not optimization algorithms.

It integrates with the repository's broader compilation/optimization architecture.

Owns

Potential source-level concepts such as:

- optimization problem boundaries;
- objectives;
- constraints;
- variables;
- optimization intent;
- solver-independent optimization declarations.

Does not own

- optimizer implementation;
- solver selection algorithm;
- hardware-specific optimization;
- scheduling;
- target selection.

A source construct expressing:

minimize objective
subject to constraints

must not mean:

use solver X
on GPU Y
with N threads

unless the programmer explicitly expresses such target-specific requirements.

---

26. "signal-processing.g4"

Purpose

Defines signal-processing source syntax where language-level syntax is justified.

Potential domains include:

- signals;
- streams;
- transformations;
- filtering;
- convolution;
- correlation;
- sampling;
- resampling;
- spectral processing;
- time/frequency representations.

Does not own

- DSP instruction sets;
- fixed sample sizes;
- fixed buffer sizes;
- hardware DSP blocks;
- vendor implementations;
- FFT implementation algorithms.

Scalability

No fixed:

MAX_SIGNAL_LENGTH
MAX_SAMPLE_COUNT
MAX_CHANNELS
MAX_BUFFER_SIZE

may be introduced as universal language limits.

---

27. "statistics.g4"

Purpose

Defines the classical statistical-computation boundary.

Owns

Statistical-domain composition and any genuinely language-level statistical constructs.

Does not own

- probability algorithm implementations;
- random-number-generator implementation;
- statistical libraries;
- fixed sample sizes;
- fixed dimensions;
- hardware selection.

Semantic integration

Statistical operations must use the canonical expression/type system and be represented by the domain-neutral AST.

---

28. "symbolic.g4"

Purpose

Defines symbolic-computation syntax.

Owns

Only genuine language-level symbolic constructs such as standardized:

- symbolic declarations;
- symbolic bindings;
- symbolic transformation boundaries;
- symbolic evaluation boundaries;
- compile-time symbolic forms.

Does not own

- CAS implementations;
- theorem provers;
- simplification algorithms;
- symbolic libraries;
- algebra systems.

Integration

source expression
       |
       v
domain-neutral AST
       |
       v
symbolic semantic analysis
       |
       v
canonical symbolic representation

The grammar must remain implementation-neutral.

---

29. "scientific-computing.g4"

Purpose

Defines the scientific-computing domain boundary.

Scientific computing may combine:

- numerical computation;
- symbolic computation;
- statistics;
- linear algebra;
- tensors;
- data;
- signal processing;
- optimization;
- control;
- parallelism;
- distributed execution.

Critical architectural rule

Scientific computing must compose existing domains.

It must not create:

ScientificExpression
ScientificType
ScientificAST
ScientificIR

merely to distinguish scientific programs.

A scientific computation remains a Zamani program using existing language semantics.

---

30. "control.g4"

Purpose

Defines classical control-system and cyber-physical computation syntax.

This is distinct from ordinary program control flow.

Ordinary:

if
while
for
match
return

belongs to the general statement grammar.

Classical control syntax may describe:

- control systems;
- signals;
- state-space models;
- feedback intent;
- controllers;
- plants;
- control objectives;
- sampling/timing requirements;
- system constraints.

Does not own

- ordinary "if";
- ordinary loops;
- general concurrency;
- hardware timing implementation;
- physical actuator implementation.

---

31. "classical-accelerators.g4"

Purpose

Defines portable classical accelerator intent.

An accelerator is a capability, not a fixed physical device.

Valid semantic direction

requires capability("accelerator.compute")

or another canonical resource/capability construct.

Invalid universal direction

use_gpu_0
use_gpu_1
use_cpu_0
use_fpga_3
device_42

Those are realization-specific concepts.

Integration

classical accelerator intent
            |
            v
capability analysis
            |
            v
resource analysis
            |
            v
optimization
            |
            v
scheduling
            |
            v
placement
            |
            v
target lowering
            |
            v
runtime

The grammar must not discover or allocate the accelerator.

---

32. "intrinsics.md"

"intrinsics.md" is the semantic contract for classical compiler intrinsics.

It must remain separate from parser syntax.

An intrinsic may be:

- compiler-recognized operation;
- compiler-recognized property;
- semantic conversion;
- optimization boundary;
- target-independent primitive;
- compiler-known capability.

The intrinsic system must remain open-ended.

It must not become a permanent enumeration of:

- CPU instructions;
- GPU instructions;
- SIMD instructions;
- vendor functions;
- BLAS routines;
- every mathematical function;
- every future accelerator primitive.

The intrinsic contract must define:

intrinsic identity
namespace
signature
operand types
result types
effects
capabilities
resource requirements
determinism
purity
constant-evaluation rules
semantic constraints
AST representation
IR representation
lowering requirements
target-independent meaning
target-specific implementations
diagnostics
version
compatibility

The parser must not execute an intrinsic.

---

33. Canonical Expression Integration

Classical grammar components must consume the repository's canonical expression grammar.

The classical domain must not create another precedence hierarchy.

There must be one authoritative meaning for:

+
-
*
/
%
**
==
!=
<
<=
>
>=
&&
||
&
|
^
<<
>>
...

where those operators are part of the canonical Zamani language.

Classical semantics are determined after parsing.

---

34. Canonical Type Integration

Classical grammar must reuse "grammar/types/".

Conceptual examples include:

Int
Float
Decimal
Vector<T>
Matrix<T, shape>
Tensor<T, shape>

and future types.

The classical grammar must not independently redefine generic type syntax.

Resource-bearing types such as:

Memory<T, size>

must be interpreted according to the canonical type/resource architecture.

The grammar must not transform type parameters into compiler maximums.

---

35. Shape Semantics

Classical numerical domains require a strong distinction between:

shape
dimension
rank
layout
capacity
allocation
execution width

For example:

Tensor<Float, shape>

describes semantic structure.

It does not automatically select:

- RAM;
- VRAM;
- cache;
- register storage;
- SIMD width;
- GPU memory;
- FPGA memory block.

The compiler and resource system determine feasible realization.

---

36. Resource Integration

Classical grammar integrates with:

grammar/resources/

and:

grammar/hardware/

rather than duplicating their syntax.

The conceptual distinction is:

classical computation
        |
        v
resource requirement
        |
        v
capability analysis
        |
        v
available resources
        |
        v
realization

Examples of portable intent include:

requires memory >= required_memory
requires capability("tensor.compute")
requires capability("vector.compute")
requires capability("distributed.compute")
requires capability("accelerator.compute")

The exact universal resource syntax remains owned by "grammar/resources/".

---

37. Requirement vs Preference vs Hint

The classical domain must preserve the distinction between:

requirement
constraint
capability
preference
hint
implementation decision

For example:

requires capability("vector.compute")

means the program requires a capability.

It does not mean:

use AVX

A preference may influence optimization.

A hint may improve realization.

Neither must become a semantic guarantee unless the language specification says so.

---

38. Hardware Integration

Classical grammar integrates with:

grammar/hardware/

Hardware syntax owns hardware intent.

Classical grammar describes computation.

The boundary is:

classical computation
        |
        v
semantic requirements
        |
        v
hardware capability matching
        |
        v
realization

Classical grammar must not enumerate:

- CPU models;
- GPU models;
- FPGA families;
- ASIC models;
- cache sizes;
- register counts;
- vector widths;
- device identifiers.

---

39. Compilation Integration

Classical grammar integrates with:

grammar/compile/

The source describes portable computation.

Compilation determines how that computation can be realized.

The classical grammar must not embed:

CUDA
ROCm
AVX
SSE
NEON
specific FPGA primitives
specific ASIC cells
vendor-specific instruction sets

as universal language constructs.

Such features belong to explicit interoperability/dialect/target mechanisms.

---

40. Execution Integration

Classical grammar integrates with:

grammar/execution/

Execution semantics may consume:

- resource requirements;
- capabilities;
- scheduling intent;
- placement intent;
- execution policies;
- resilience policies.

The classical grammar does not execute anything.

It must not:

- start processes;
- allocate devices;
- inspect the host;
- inspect environment variables;
- read files;
- contact networks;
- query hardware;
- invoke runtime functions.

---

41. Concurrency Integration

Classical computation can be:

- sequential;
- concurrent;
- asynchronous;
- parallel;
- data-parallel;
- task-parallel;
- pipelined;
- distributed.

However, concurrency syntax belongs to:

grammar/concurrency/

The classical domain consumes the semantic results.

Do not encode universal assumptions such as:

8 threads
16 cores
32 workers

into classical grammar.

---

42. Distributed Integration

Classical programs may execute across:

- one processor;
- many processors;
- many devices;
- many nodes;
- clusters;
- supercomputers;
- cloud infrastructure;
- future distributed environments.

Distributed syntax belongs to:

grammar/distributed/

Classical grammar must not define:

MAX_NODES

or fixed topology.

---

43. Quantum-Classical Integration

Classical computation is a fundamental part of hybrid quantum programs.

The boundary is:

classical computation
        |
        v
quantum operation
        |
        v
measurement
        |
        v
classical value
        |
        v
classical decision
        |
        v
quantum operation

Classical grammar must not duplicate quantum grammar.

Quantum syntax remains under:

grammar/quantum/

The canonical quantum semantic boundary remains:

quantum::ir

The classical grammar must not introduce:

ClassicalQuantumIR
HybridQuantumIR
SecondQuantumIR

or another competing representation.

---

44. HDL Integration

Classical computation may participate in hardware/software co-design.

The boundary is:

classical algorithm
        |
        v
semantic computation
        |
        +----------------+
        |                |
        v                v
classical realization   HDL/hardware intent

HDL syntax remains owned by:

grammar/hdl/

Classical grammar must not redefine:

- wires;
- registers;
- ports;
- hardware clocks;
- physical routing;
- HDL semantics.

Conversely, HDL must not force classical algorithms into fixed hardware assumptions.

---

45. AI/ML Integration

Classical computation provides the foundation for:

- tensor computation;
- numerical computation;
- optimization;
- data processing;
- training;
- inference;
- simulation.

AI-specific syntax remains under:

grammar/ai/

Classical grammar must not become framework-specific.

It must not encode:

PyTorch
TensorFlow
JAX
CUDA
ROCm

as universal semantic dependencies.

---

46. Data Integration

Classical computation may consume and produce data structures defined under:

grammar/data/

The classical grammar must not create a duplicate collection/data language.

---

47. Networking Integration

Classical programs may communicate over networks.

Networking syntax belongs to:

grammar/networking/

Classical grammar must not assume:

- fixed network size;
- fixed endpoint count;
- fixed bandwidth;
- fixed topology;
- fixed number of network devices.

---

48. Security Integration

Classical computation can have security requirements.

Security semantics remain under:

grammar/security/

A resource requirement is not permission.

For example:

requires accelerator

does not grant authorization to access an accelerator.

Similarly:

requires memory

does not imply ownership of physical memory.

---

49. Memory Integration

Classical data may reside in:

- registers;
- caches;
- local memory;
- shared memory;
- accelerator memory;
- distributed memory;
- persistent storage;
- future memory systems.

These are realization concerns unless explicitly represented as language semantics.

The classical grammar must not encode:

RAM = 64 GB
VRAM = 24 GB
register = 32 bit

as universal assumptions.

Memory ownership and borrowing remain under:

grammar/memory/

---

50. Mathematical Representation Strategy

The language should support mathematical computing without requiring a keyword for every mathematical concept.

Use the hierarchy:

general expression
       |
       v
typed operation
       |
       v
intrinsic / standard operation
       |
       v
semantic optimization
       |
       v
target implementation

For example:

A * B

may become matrix multiplication after type/shape analysis.

The grammar should not need a new keyword merely because the operands happen to be matrices.

Similarly:

fft(signal)

can be recognized semantically without requiring:

FFT

to become a permanent reserved keyword.

---

51. Classical Semantic Model

After parsing, semantic analysis should be able to classify classical operations using information including:

operation identity
qualified namespace
operand types
result types
shape
rank
dimensions
precision
effects
ownership
borrowing
resource requirements
capability requirements
determinism
purity
parallelism
distribution
memory behavior
numerical properties
target constraints

The parser should preserve the information necessary to perform this analysis.

---

52. Domain-Neutral AST Requirement

The existing frontend AST is intended to remain domain-neutral.

Classical grammar must therefore avoid creating AST types such as:

ClassicalMatrixAst
ClassicalTensorAst
ClassicalGpuAst
ClassicalCpuAst

unless the existing AST architecture explicitly establishes such nodes as the canonical semantic boundary.

Prefer generic nodes containing semantic information.

Conceptually:

Syntax
  |
  v
generic AST
  |
  v
semantic classification
  |
  +--> scalar
  +--> vector
  +--> matrix
  +--> tensor
  +--> numerical
  +--> symbolic
  +--> optimization
  +--> scientific

This keeps the frontend reusable.

---

53. AST Contract for Every Classical Construct

Every classical grammar construct must identify in advance:

grammar rule
    |
    v
AST node or existing generic node
    |
    v
semantic representation
    |
    v
canonical IR

No classical grammar feature may be considered complete while its AST mapping is merely:

TBD

The mapping may be generic, but it must be known.

---

54. Semantic Contract for Every Classical Construct

Each construct must specify:

- accepted types;
- rejected types;
- shape requirements;
- dimensional requirements;
- effect requirements;
- ownership requirements;
- capability requirements;
- resource requirements;
- determinism;
- numerical semantics;
- error conditions;
- cross-domain behavior;
- lowering requirements.

Parser acceptance alone does not imply semantic validity.

---

55. IR Contract

The classical grammar does not define IR.

The canonical pipeline is:

source
  |
  v
lexer
  |
  v
parser
  |
  v
domain-neutral AST
  |
  v
semantic analysis
  |
  v
canonical semantic representation
  |
  v
classical IR / canonical applicable IR
  |
  v
optimization
  |
  v
scheduling / placement / lowering
  |
  v
target

The exact canonical classical IR owner must remain the repository's compiler/IR architecture.

The grammar must never invent an independent IR merely because a new classical domain is added.

---

56. Numerical Semantics

Numerical semantics belong downstream from syntax.

They include, as applicable:

- precision;
- rounding;
- overflow;
- underflow;
- saturation;
- NaN;
- infinity;
- interval behavior;
- arbitrary precision;
- exact arithmetic;
- approximate arithmetic;
- numerical stability;
- reproducibility;
- deterministic reduction;
- error bounds.

The grammar should expose syntax sufficient to express these semantics but must not implement them.

---

57. Arbitrary Numeric Magnitude

The language must not assume that all source integers fit into the host machine's native integer type.

The lexical and semantic layers must support arbitrary representable program-level magnitudes according to the language specification.

The implementation may detect resource or representational exhaustion.

That is not permission to introduce:

language integer maximum = host integer maximum

as an accidental grammar rule.

---

58. Deterministic Parsing

Classical parsing must depend only on:

- source;
- token stream;
- language version;
- explicitly selected dialect configuration;
- grammar version.

It must not depend on:

- hardware;
- current memory availability;
- GPU availability;
- number of CPU cores;
- filesystem state;
- environment variables;
- network state;
- current time;
- randomness;
- runtime state.

The same valid source must produce the same syntactic result under the same language configuration.

---

59. Safety Contract

All classical grammar files must remain declarative.

They must contain:

- no embedded Rust actions;
- no unsafe Rust;
- no runtime calls;
- no filesystem operations;
- no network operations;
- no hardware discovery;
- no environment inspection;
- no secret access;
- no process execution;
- no allocation logic.

The Rust implementation must target:

Rust 2021
Rust 1.97
Rust 1.97.1

and must use safe Rust.

No "unsafe" block, "unsafe fn", "unsafe trait", or "unsafe impl" may be required by the classical grammar implementation.

---

60. ANTLR Architecture

The classical grammars must follow the repository's actual ANTLR composition architecture.

There must be one canonical lexer vocabulary.

Classical parser grammars may consume:

ZamaniLexer

or the repository's authoritative parser composition mechanism.

They must not create competing lexical vocabularies.

There must not be two unrelated definitions for the same token concept.

---

61. Token Ownership

Classical grammars must not create lexical duplicates for concepts already owned by:

grammar/lexer/

Examples of problems that must remain eliminated include duplicate conceptual tokens such as:

Question
QuestionMark

or:

Ampersand
BitAnd

where the distinction is accidental rather than semantic.

Classical syntax consumes canonical tokens.

---

62. Import and Dependency Direction

The intended dependency direction is:

lexer
  |
  v
core
  |
  +--> types
  +--> expressions
  +--> declarations
  +--> statements
  +--> functions
  +--> modules
  |
  v
classical
  |
  +--> classical subdomains
  |
  v
AST
  |
  v
semantic analysis
  |
  v
IR

Classical grammar must not create cycles such as:

classical -> expressions -> classical

or:

classical -> hardware -> classical

unless the canonical architecture explicitly provides a non-circular composition mechanism.

---

63. No Duplicate Grammar Authorities

These files must never become competing authorities:

grammar/Zamani.g4
grammar/classical/*.g4
grammar/grammar.md
grammar/Zamani-Grammar.md

Their roles remain:

Zamani.g4
    = canonical composition

classical/*.g4
    = domain grammar components

grammar.md
    = implementation conformance

Zamani-Grammar.md
    = historical/extended design

A feature documented only in "Zamani-Grammar.md" is not automatically legal Zamani syntax.

---

64. Feature Promotion

A proposed classical feature follows:

Zamani-Grammar.md
        |
        v
feature proposal
        |
        v
semantic design
        |
        v
AST contract
        |
        v
classical grammar
        |
        v
implementation
        |
        v
IR contract
        |
        v
tests
        |
        v
stable

No feature should bypass this process.

---

65. Classical Feature Independence

A classical feature must be independently completable.

Before implementing a grammar file or rule, its contract must already identify:

Purpose
Owns
Does Not Own
Inputs
Outputs
Dependencies
Upstream Contracts
Downstream Consumers
Grammar Contract
AST Contract
Semantic Contract
IR Contract
Compiler Integration
Runtime Integration
Resource Integration
Capability Integration
Cross-Domain Integration
Diagnostics
Positive Tests
Negative Tests
Boundary Tests
Scalability Tests
Determinism Tests
Compatibility
Hard-Coding Audit
Completion Criteria

This is the required file independence contract.

---

66. File Completion Does Not Depend on Future Rework

A classical grammar file must not be declared complete merely because its ANTLR rules currently compile.

It is complete only when its downstream contracts have already been specified.

For example, "matrix.g4" must already know:

which AST representation it feeds;
how shape semantics are represented;
how type analysis consumes it;
how semantic matrix operations are represented;
how IR lowering consumes those semantics;
how resources are represented;
how accelerators may consume them;
how distributed execution may consume them;
which tests prove scalability;

Adding another classical feature later must not require redesigning the already-completed matrix contract.

---

67. Cross-Domain Compatibility

Classical grammar must integrate with:

core/
types/
expressions/
statements/
declarations/
functions/
modules/
effects/
memory/
concurrency/
quantum/
hybrid/
hdl/
hardware/
resources/
distributed/
ai/
data/
networking/
security/
compile/
execution/
interoperability/
dialects/
macros/
metaprogramming/

The rule is:

«Classical computation consumes shared language infrastructure; it does not redefine it.»

---

68. Classical + Quantum

Tests must cover:

classical value -> quantum parameter
quantum measurement -> classical value
classical condition -> quantum control
quantum result -> classical computation
classical preprocessing -> quantum operation
quantum measurement -> classical postprocessing

Quantum lowering remains under the canonical:

quantum::ir

boundary.

---

69. Classical + HDL

Tests must cover:

classical algorithm
        |
        v
hardware intent
        |
        v
HDL representation

without requiring the classical grammar to know physical FPGA/ASIC implementation details.

---

70. Classical + AI

Tests must cover:

classical scalar
classical vector
classical matrix
classical tensor
classical optimization
classical data

interacting with AI-domain syntax without creating duplicate types or expressions.

---

71. Classical + Distributed

Tests must verify that the same semantic classical computation can be represented independently of:

node count
process count
worker count
network topology
device count

---

72. Classical + Hardware

A classical program may express requirements through the resource/capability system.

For example:

requires capability("vector.compute")

The compiler may later realize the operation using:

scalar CPU
vector CPU
GPU
FPGA
ASIC
accelerator
future processor

The classical grammar does not choose among them.

---

73. Classical + Execution

Execution policies may influence:

- scheduling;
- placement;
- parallelization;
- deployment;
- resilience;
- dispatch.

Classical grammar remains the description of computation.

---

74. Classical + Security

Security requirements must remain separate from computation.

For example:

requires capability("secure.compute")

is an intent/capability requirement.

It does not directly authorize access.

Authorization belongs to security semantics.

---

75. Classical + Memory

Classical values may use different memory models.

The grammar must not assume a particular memory hierarchy.

The semantic system determines memory behavior.

The target system realizes that behavior.

---

76. Classical + Macros

Macros may generate classical syntax.

Macro expansion must eventually pass through the same:

lexer/parser/AST/semantic/IR

contracts.

Macros must not create a bypass around classical semantic validation.

---

77. Classical + Metaprogramming

Compile-time generation may create:

- vectors;
- matrices;
- tensors;
- numerical expressions;
- algorithms;
- domain declarations.

The generated result must still obey the same classical grammar and semantic contracts.

---

78. Classical + Interoperability

Interoperability may connect Zamani to:

- C;
- C++;
- Rust;
- Python;
- WebAssembly;
- numerical libraries;
- foreign accelerators;
- external data systems.

Those are interoperability concerns.

They must not become permanent classical grammar dependencies.

---

79. Classical + Dialects

Vendor-specific or experimental classical features should be namespaced through the dialect architecture.

Conceptually:

standard::operation
vendor::operation
experimental::operation
future::operation

The standard classical grammar must not become a catalogue of every vendor feature.

---

80. Resource Availability Is Not Language Availability

A machine may lack resources required by a program.

That does not make the program syntactically invalid.

The stages are:

parse
  |
  v
semantic validity
  |
  v
resource feasibility
  |
  v
target realization

For example:

requires memory >= required_memory

may be syntactically and semantically valid even when a particular execution environment cannot satisfy it.

The appropriate downstream diagnostic is a resource-feasibility diagnostic, not a parser error.

---

81. Tiny-to-Large Scaling

The same language construct must remain meaningful on small and large systems.

Conceptually:

scalar
   |
single processor
   |
multicore
   |
vector processor
   |
GPU
   |
FPGA
   |
ASIC
   |
accelerator
   |
cluster
   |
supercomputer
   |
distributed system
   |
cloud
   |
future architecture

The language must not force the source program to encode the realization merely because one environment is currently available.

---

82. Scaling Is Resource-Driven

The implementation may determine:

available memory
available compute
available parallelism
available accelerator capability
available network
available storage
available numerical formats
available hardware features

The source program remains semantic.

This is the foundation for scaling from tiny systems toward extremely large systems.

---

83. No Artificial "Infinity" Claim

"Scale to infinity" means the language architecture must not impose arbitrary finite language limits.

It does not mean that physical computers have infinite resources.

Therefore:

language capacity

must remain conceptually unbounded where the semantics permit it, while:

actual execution

remains constrained by physical and implementation resources.

This distinction is mandatory for technically correct POCO-REAF.

---

84. Error Classification

Classical grammar must distinguish:

Syntax error

Example:

malformed tensor declaration

Type error

Example:

matrix + string

Shape error

Example:

incompatible matrix dimensions

Semantic error

Example:

invalid operation for the operand types

Capability error

Example:

required capability unavailable

Resource error

Example:

mandatory resource requirement cannot be satisfied

Backend error

Example:

target lowering cannot realize the requested semantics

A hardware/resource failure must never be reported as a grammar failure.

---

85. Diagnostics

Every classical construct must preserve source locations.

The parser must expose sufficient source information for the frontend to construct source spans.

Diagnostics must identify:

- source location;
- relevant construct;
- diagnostic category;
- actual problem;
- expected construct where useful;
- semantic context where available.

The grammar must not manufacture runtime/hardware diagnostics.

---

86. Determinism

Repeated parsing of identical source under identical configuration must be deterministic.

No classical parser rule may depend on:

randomness
clock
hardware
environment
filesystem
network
runtime state

This is required for reproducible builds and conformance.

---

87. Reproducibility

The grammar source must be reproducible.

Generated ANTLR output is derived output.

Generated artifacts are not language authorities.

Grammar changes must be reviewable from the canonical source files.

---

88. Compatibility

Every classical grammar change must be classified as:

additive
compatible
deprecated
breaking

Compatibility must cover:

- syntax;
- tokens;
- AST mapping;
- semantic meaning;
- IR meaning;
- diagnostics;
- formatting;
- dialect interaction.

A syntax change must not silently change an existing construct's semantic meaning.

---

89. Deprecation

Deprecated classical syntax must document:

original construct
replacement
first deprecated version
migration guidance
removal policy
compatibility impact

Deprecated syntax must not be removed merely because a new backend has a different preferred implementation.

---

90. Versioning

Classical grammar follows the Zamani language version.

Individual algorithms or hardware implementations must not independently redefine the language version.

Future classical features should be extensible without requiring breaking changes to existing programs wherever possible.

---

91. Testing Location

Primary classical conformance tests belong under:

grammar/tests/classical/

Existing global test organization must be respected.

The classical directory itself should not accumulate an unrelated second test hierarchy unless the repository architecture explicitly requires tests beside grammar sources.

Recommended structure:

grammar/tests/classical/
├── lexical/
├── syntax/
├── semantics/
├── scalar/
├── integer/
├── floating-point/
├── vector/
├── matrix/
├── tensor/
├── numerical/
├── linear-algebra/
├── optimization/
├── signal-processing/
├── statistics/
├── symbolic/
├── scientific-computing/
├── control/
├── accelerators/
├── cross-domain/
├── negative/
├── boundary/
├── scalability/
├── determinism/
└── compatibility/

This does not require renaming any existing classical source files.

---

92. Positive Tests

Every classical feature must have positive tests.

Examples:

minimal classical program
scalar computation
integer computation
floating-point computation
vector construction
matrix construction
tensor construction
symbolic expression
numerical operation
linear algebra
optimization
signal processing
statistics
scientific computation
control computation
accelerator intent

---

93. Negative Tests

Negative tests must cover:

- malformed expressions;
- malformed shapes;
- invalid dimensions;
- invalid type combinations;
- invalid operation calls;
- invalid argument counts;
- malformed domain syntax;
- malformed accelerator requirements;
- invalid cross-domain constructs;
- duplicate syntax;
- invalid namespaces;
- invalid resource expressions.

Parser errors and semantic errors must be tested separately.

---

94. Boundary Tests

Boundary tests must include:

smallest valid scalar
empty/non-empty collections where semantics permit
single-element vector
single-element matrix
symbolic vector dimension
symbolic matrix dimensions
symbolic tensor shape
large literal magnitude
large symbolic dimensions
nested expressions
deep operation composition
large argument lists
large generated structures

The tests must demonstrate that the grammar does not secretly establish finite machine limits.

---

95. Scalability Tests

Scalability tests must intentionally use symbolic quantities:

n
m
rows
columns
rank
problem_size
workload_size
parallelism
memory_required
node_count
device_count

The grammar must parse these without interpreting them as fixed machine capacities.

Tests must include expressions conceptually equivalent to:

vector<n>
matrix<rows, columns>
tensor<shape>

where those forms are part of the canonical type syntax.

---

96. Hard-Coding Audit

Every classical grammar change must undergo a hard-coding audit.

Search for:

MAX_

and equivalents involving:

CPU
CORE
THREAD
GPU
FPGA
ASIC
DEVICE
NODE
MEMORY
REGISTER
VECTOR
MATRIX
TENSOR
PROCESS
TASK
WORKER

Every occurrence must be classified.

Allowed categories may include:

1. genuine language semantic requirement;
2. explicit program value;
3. target-specific dialect;
4. test fixture;
5. documentation example;
6. resource constraint represented by the correct resource system.

Forbidden category:

accidental universal language limitation

---

97. Hardware Enumeration Audit

Classical grammar must be reviewed for accidental enumeration of:

CPU models
GPU models
FPGA families
ASIC families
accelerator models
instruction sets
vendor libraries

A new hardware device should not require rewriting the fundamental classical grammar simply because the device exists.

---

98. Algorithm Enumeration Audit

The same rule applies to algorithms.

Adding a new:

solver
FFT algorithm
optimization method
statistical method
linear algebra algorithm
signal transform
AI operator
scientific method

should normally not require adding a new reserved keyword.

Use the open operation/intrinsic model.

---

99. Intrinsic vs Grammar Decision

A new classical operation belongs in the intrinsic system rather than the grammar when:

- ordinary call syntax expresses it;
- semantic analysis can identify it;
- compiler optimization benefits from recognizing it;
- no new language-level structure is required.

A grammar extension is justified when the operation introduces syntax or semantics that cannot reasonably be expressed using existing language constructs.

---

100. Standard Library vs Intrinsic vs Grammar

The three layers must remain distinct.

Grammar
    |
    | language syntax
    v
Intrinsic
    |
    | compiler-known semantic operation
    v
Standard Library
    |
    | reusable implementation/API
    v
Backend
    |
    | target realization
    v
Hardware

Do not move an operation into grammar merely because it is useful.

---

101. Memory and Performance

Grammar performance must remain predictable.

Do not introduce grammar structures that cause avoidable exponential ambiguity.

Classical grammars must be checked for:

- ambiguous alternatives;
- left-recursive conflicts;
- unreachable rules;
- duplicate rules;
- duplicate token references;
- excessive backtracking;
- pathological nesting;
- unnecessary semantic predicates.

No semantic predicate should be used merely to query hardware.

---

102. Parser Safety

ANTLR grammar source must not contain embedded executable code.

The classical grammar must not perform:

hardware detection
file I/O
network I/O
allocation
execution
environment inspection

during parsing.

---

103. Rust Integration

The Rust frontend must remain compatible with:

Rust 1.97
Rust 1.97.1
Rust 2021

and safe Rust.

The grammar must not require compiler features newer than the declared baseline.

No "unsafe" implementation is permitted.

Rust-side AST construction, semantic analysis, IR generation, diagnostics, and compiler integration must remain outside the declarative grammar.

---

104. Classical IR Boundary

Classical grammar does not own the classical IR implementation.

The compiler architecture must provide the appropriate canonical representation.

The classical grammar must preserve enough semantic information to permit:

operation
type
shape
effects
requirements
capabilities
constraints

to reach semantic analysis and IR lowering.

---

105. Optimization Boundary

Optimization may transform implementation while preserving semantics.

For example:

matrix multiplication

could be implemented through different strategies.

The source grammar does not select the optimization algorithm.

Optimization may consider:

- available resources;
- capabilities;
- target architecture;
- cost;
- parallelism;
- memory;
- numerical constraints;
- performance constraints.

---

106. Scheduling Boundary

Scheduling determines execution order/resource timing.

Classical grammar must not embed a particular scheduler.

A programmer may express a requirement such as:

latency(...)
deadline(...)
throughput(...)

where supported by the canonical execution/resource systems.

The scheduler determines realization.

---

107. Placement Boundary

Classical grammar must not decide:

CPU 0
GPU 2
node 7
memory bank 3
FPGA region X

as universal semantics.

Placement is downstream.

---

108. Resilience Boundary

Classical programs may participate in resilient execution.

The grammar must not implement recovery.

Existing resilience vocabulary remains conceptually separate:

Unknown
Healthy
Degraded
Unstable
Unavailable
Recovering
Quarantined
Retired

and outcomes:

ACCEPT
DEGRADED_ACCEPT
RETRY
RECOVER
ESCALATE
REJECT

These are execution/resilience semantics, not classical parser behavior.

---

109. Classical Computation and ZQN

If classical computation interacts with quantum systems:

classical semantics
       |
       +--> quantum semantics
                |
                v
             quantum::ir
                |
                v
               QEC
                |
                v
               ZQN

The classical grammar must not implement:

- QEC;
- noise modelling;
- physical qubit routing;
- calibration;
- quantum scheduling.

Those remain downstream responsibilities.

---

110. Hardware-Software Co-Design

A production Zamani program may eventually describe:

algorithm
+
data
+
parallelism
+
memory intent
+
communication
+
accelerator intent
+
timing requirements
+
hardware intent

without turning all of those into one monolithic classical grammar.

The individual semantic domains must remain composable.

---

111. No Physical-Memory Assumptions

Never encode universal assumptions such as:

RAM = 64GB
VRAM = 24GB
cache = X
register = 32-bit

A machine may have:

- less;
- more;
- none of a particular class;
- multiple memory classes;
- unified memory;
- distributed memory;
- future memory technology.

The resource and hardware layers resolve those differences.

---

112. No Physical Vector Width Assumptions

A semantic vector must not be equated with:

SIMD width

A vector can be lowered using:

- scalar instructions;
- SIMD;
- vector processors;
- GPU threads;
- FPGA pipelines;
- ASIC datapaths;
- distributed execution.

The semantic vector remains unchanged.

---

113. No Fixed Tensor Rank

Tensor rank is program semantics.

Hardware tensor-engine capabilities are implementation properties.

Do not create:

MAX_TENSOR_RANK

as a language rule.

---

114. No Fixed Operation Inventory

The language must remain extensible.

A future operation must be representable without modifying the fundamental grammar merely because the operation did not exist when the grammar was authored.

Namespaces and open operation names are therefore important.

---

115. Namespace Strategy

Classical operations should support qualified semantic names where canonical Zamani name syntax allows them.

Conceptually:

linalg::matmul
tensor::contract
statistics::mean
signal::transform
optimization::solve
scientific::simulate
vendor::operation
future::operation

The parser recognizes names.

Semantic resolution determines whether a referenced operation exists and is compatible.

---

116. Vendor Extensions

Vendor extensions must not contaminate the universal classical grammar.

A vendor may provide:

vendor::operation

through the dialect/interoperability mechanisms.

Universal Zamani semantics remain stable.

---

117. Future Computing Models

Classical computation must be able to participate in future architectures not yet known today.

Examples may include:

- new accelerator types;
- new memory technologies;
- new processor organizations;
- new interconnects;
- new computational substrates;
- new distributed models;
- new heterogeneous systems.

The grammar should describe semantic computation, allowing future backends to realize it.

---

118. No Assumption That CPU Is the Default Forever

Classical computation does not mean:

CPU-only

A classical algorithm may execute on:

CPU
GPU
FPGA
ASIC
accelerator
distributed system
future architecture

The classical semantic domain therefore remains broader than CPU programming.

---

119. No Assumption That Classical Means Sequential

Classical computation may be:

parallel
concurrent
distributed
streaming
vectorized
pipelined
event-driven
accelerated
heterogeneous

Concurrency and execution domains provide the corresponding semantics.

---

120. No Assumption That Numerical Means Floating-Point

Numerical computation may use:

- integers;
- floating point;
- fixed point;
- decimal;
- arbitrary precision;
- exact arithmetic;
- symbolic values;
- intervals;
- complex values;
- domain-specific numeric representations.

The grammar must not hard-code one representation as universal.

---

121. No Assumption That Matrix Means Dense

Matrix semantics may represent:

- dense;
- sparse;
- structured;
- symbolic;
- distributed;
- block;
- compressed;
- future representations.

Storage format is a semantic/implementation concern unless explicitly represented.

---

122. No Assumption That Tensor Means Contiguous Memory

Tensor semantics must remain independent of physical layout.

The compiler may select an appropriate representation.

---

123. No Assumption That Parallelism Has a Fixed Shape

Parallelism may be:

task parallel
data parallel
pipeline parallel
model parallel
distributed
heterogeneous
nested
dynamic

The classical grammar consumes these through the appropriate canonical concurrency/execution/resource systems.

---

124. Testing the Actual Integration

Production readiness requires tests across the repository boundary:

classical grammar
    |
    v
Zamani.g4
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
semantic analysis
    |
    v
IR
    |
    v
compiler
    |
    v
runtime

A classical ".g4" file passing an isolated parser test is insufficient.

---

125. Required Classical Conformance Matrix

Every feature should be traceable through:

Layer| Required evidence
Specification| Feature defined
Lexer| Required tokens exist
Grammar| Syntax accepted
Negative grammar| Invalid syntax rejected
AST| Mapping exists
Semantics| Meaning defined
Type system| Type rules defined
Resource system| Requirements defined if applicable
Capability system| Capabilities defined if applicable
IR| Lowering defined
Compiler| Consumer defined
Runtime| Runtime contract defined if applicable
Tests| Positive/negative/boundary/scalability
Compatibility| Version behavior documented
Hard-coding| Audit passed

---

126. Production Definition of Done — "classical.g4"

Complete only when:

[ ] Classical composition boundary is unique.
[ ] No duplicate root grammar exists.
[ ] Imports/dependencies are deterministic.
[ ] Canonical tokens are used.
[ ] Canonical expressions are used.
[ ] Canonical types are used.
[ ] AST mapping exists.
[ ] Semantic mapping exists.
[ ] IR mapping exists.
[ ] Compiler integration is defined.
[ ] Runtime integration is defined.
[ ] Resource integration is defined.
[ ] Capability integration is defined.
[ ] Cross-domain integration is defined.
[ ] Positive tests exist.
[ ] Negative tests exist.
[ ] Boundary tests exist.
[ ] Scalability tests exist.
[ ] Determinism tests exist.
[ ] Compatibility is documented.
[ ] Hard-coding audit passes.

---

127. Production Definition of Done — Numeric Files

For:

arithmetic.g4
integer.g4
floating-point.g4
scalar.g4
numeric.g4

require:

[ ] lexical ownership is explicit;
[ ] type ownership is explicit;
[ ] expression ownership is explicit;
[ ] semantic numeric rules are defined;
[ ] no fixed machine width is assumed;
[ ] arbitrary program magnitude is handled correctly;
[ ] overflow semantics are defined downstream;
[ ] target representation is not confused with language semantics;
[ ] AST mapping exists;
[ ] IR mapping exists;
[ ] tests exist.

---

128. Production Definition of Done — Shape Files

For:

vector.g4
matrix.g4
tensor.g4

require:

[ ] shape semantics defined;
[ ] dimension semantics defined;
[ ] rank semantics defined where applicable;
[ ] indexing ownership is explicit;
[ ] layout is separated from shape;
[ ] allocation is separated from shape;
[ ] execution width is separated from shape;
[ ] no MAX_* limits;
[ ] symbolic dimensions supported where specified;
[ ] generic dimensions supported where specified;
[ ] runtime dimensions supported where specified;
[ ] AST mapping exists;
[ ] semantic mapping exists;
[ ] IR mapping exists;
[ ] scalability tests exist.

---

129. Production Definition of Done — Domain Algorithms

For:

numerical.g4
linear-algebra.g4
optimization.g4
signal-processing.g4
statistics.g4
symbolic.g4
scientific-computing.g4
control.g4

require:

[ ] domain boundary is genuine;
[ ] generic expression syntax is reused;
[ ] operation inventory is open;
[ ] algorithm implementation is not embedded;
[ ] hardware is not selected by grammar;
[ ] AST contract exists;
[ ] semantic contract exists;
[ ] intrinsic/library boundary is documented;
[ ] IR contract exists;
[ ] cross-domain behavior is defined;
[ ] tests exist;
[ ] scalability is tested.

---

130. Production Definition of Done — Accelerators

For:

classical-accelerators.g4

require:

[ ] accelerator means capability, not device identity;
[ ] resource requirements are separated from placement;
[ ] capabilities are separated from permissions;
[ ] no GPU/FPGA count limits exist;
[ ] no device IDs are embedded;
[ ] no vendor is assumed;
[ ] compiler integration exists;
[ ] scheduling integration exists;
[ ] hardware integration exists;
[ ] runtime integration exists;
[ ] fallback/degradation semantics are defined downstream;
[ ] tests cover multiple realization classes.

---

131. Production Definition of Done — "intrinsics.md"

Require:

[ ] intrinsic identity model exists;
[ ] namespace model exists;
[ ] signatures exist;
[ ] type semantics exist;
[ ] effects exist;
[ ] capabilities exist;
[ ] resource requirements exist;
[ ] determinism is specified;
[ ] AST mapping exists;
[ ] IR mapping exists;
[ ] target lowering is separate;
[ ] no fixed instruction inventory is required;
[ ] no vendor lock-in is required;
[ ] versioning exists;
[ ] compatibility exists;
[ ] tests exist.

---

132. Classical Hard-Coding Prohibition

The following are specifically forbidden as universal language limits:

MAX_CPUS
MAX_CORES
MAX_THREADS
MAX_GPUS
MAX_FPGAS
MAX_ASICS
MAX_ACCELERATORS
MAX_DEVICES
MAX_MEMORY
MAX_NODES
MAX_PROCESSES
MAX_TASKS
MAX_WORKERS
MAX_VECTOR_LENGTH
MAX_MATRIX_ROWS
MAX_MATRIX_COLUMNS
MAX_TENSOR_RANK
MAX_TENSOR_DIMENSIONS
MAX_REGISTER_WIDTH
MAX_NETWORK_SIZE

Equivalent hidden constants are equally forbidden.

---

133. Allowed Resource Constraints

The language may express actual program requirements.

Examples include:

requires qubits >= n
requires memory >= required_memory
requires capability("tensor.compute")
requires capability("gpu.compute")
requires capability("quantum.measurement")
requires topology(...)

These are requirements or constraints.

They are not universal compiler limits.

---

134. Explicit Physical Requirements

A program may intentionally contain target-specific requirements.

That does not make the language architecture wrong.

The important distinction is:

explicit source requirement

versus:

accidental compiler limitation

For example:

requires capability("specific-feature")

can intentionally restrict portability.

That restriction must be explicit and semantically documented.

---

135. Portability Diagnostics

Tooling should be able to identify unnecessary portability restrictions such as:

fixed device identity
fixed hardware topology
fixed physical resource
vendor-specific operation
fixed placement
fixed architecture assumption

Such constructs may be valid.

They should simply not be silently represented as universally portable semantics.

---

136. Resource Negotiation

The intended model is:

program
  |
  v
semantic requirements
  |
  v
resource/capability analysis
  |
  v
available environment
  |
  v
candidate realizations
  |
  v
constraints/preferences/hints
  |
  v
realization

The classical grammar remains above this process.

---

137. Target Negotiation

The compiler may determine whether a target can satisfy:

semantic requirements
+
capabilities
+
resource constraints
+
correctness requirements
+
performance requirements

The classical grammar does not make that decision.

---

138. Runtime Negotiation

Runtime may encounter resource changes.

The classical grammar must not prescribe runtime recovery.

Runtime/resilience systems determine whether execution:

continues
degrades
retries
recovers
migrates
escalates
rejects

according to their own semantics.

---

139. Classical Semantics Must Be Preserved

Optimizations may change:

instruction sequence
memory layout
parallelization
device
schedule
placement
algorithm implementation

but must preserve the semantic meaning of the Zamani program.

---

140. No Semantic Leakage

The following direction is forbidden:

hardware limitation
       |
       v
grammar restriction

The correct direction is:

program semantics
       |
       v
resource requirement
       |
       v
hardware capability
       |
       v
realization

---

141. Repository-Wide Integration Invariant

The classical domain must preserve this invariant:

ONE LANGUAGE
    |
    +-- classical
    +-- quantum
    +-- HDL
    +-- hybrid
    +-- AI
    +-- distributed
    +-- networking
    +-- security
    +-- data
    +-- future domains

These are domains of one language, not separate languages.

---

142. Classical Grammar and "Zamani.g4"

"Zamani.g4" remains the canonical composition root.

The classical domain must be integrated into it through the repository's canonical ANTLR composition mechanism.

"classical.g4" is not permitted to become a second root.

The root grammar should expose the classical domain through a stable dispatch point without copying every classical rule into "Zamani.g4".

---

143. Classical Grammar and "grammar.md"

"grammar.md" remains the implementation-conformance reference.

Classical features must eventually be represented there as:

SPECIFIED
IMPLEMENTED
PARTIALLY IMPLEMENTED
PLANNED
DEPRECATED

The README must not claim implementation status merely because syntax has been designed.

---

144. Classical Grammar and "Zamani-Grammar.md"

"Zamani-Grammar.md" remains historical/extended design material.

A classical feature found there is not automatically implemented.

Promotion remains:

design
  |
  v
proposal
  |
  v
semantic contract
  |
  v
AST
  |
  v
grammar
  |
  v
implementation
  |
  v
IR
  |
  v
tests
  |
  v
stable

---

145. Classical Grammar and "spec/"

The classical grammar must remain traceable to formal specifications including:

grammar/spec/lexical.md
grammar/spec/syntax.md
grammar/spec/type-system.md
grammar/spec/semantics.md
grammar/spec/resources.md
grammar/spec/portability.md
grammar/spec/classical.md
grammar/spec/compatibility.md

Where an existing file has a different exact path, that existing repository authority remains authoritative.

No duplicate specification authority should be created merely for classical computing.

---

146. Classical Grammar and Validation

"grammar/validation/" must eventually validate:

duplicate rules
duplicate tokens
unreachable rules
ambiguity
precedence conflicts
circular dependencies
missing AST mappings
missing semantic mappings
missing IR mappings
hard-coded limits
non-determinism
cross-domain inconsistencies

---

147. Classical Grammar and Tests

A feature is not complete because:

ANTLR generates a parser

It is complete when:

specification
  +
grammar
  +
lexer
  +
parser
  +
AST
  +
semantics
  +
IR
  +
compiler
  +
tests

agree.

---

148. Test Matrix

Every classical feature should be tested at minimum across:

positive
negative
boundary
scalability
determinism
compatibility
cross-domain

Where a formatter exists, add:

round-trip

Where code generation exists, add:

semantic-preservation

---

149. Scalability Test Principle

Tests must prove the absence of artificial limits rather than merely test a large number.

For example, a test using:

n

is more architecturally meaningful than testing only:

1024

because the latter can accidentally become a hidden maximum.

---

150. Infinite/Unbounded Structural Principle

Where the semantics are unbounded, grammar structures should remain structurally unbounded.

For example:

arguments
    : expression (COMMA expression)*
    ;

rather than:

arguments
    : expression
    | expression COMMA expression
    | expression COMMA expression COMMA expression
    | ...
    ;

The same principle applies to:

- dimensions;
- operands;
- declarations;
- statements;
- nested expressions;
- resource requirements;
- capabilities;
- transformations.

---

151. Deep Nesting

The grammar should not impose arbitrary nesting limits.

Actual parser-stack/resource limits are implementation concerns.

If a backend cannot process an extremely deep program, that is not a reason to encode an arbitrary grammar maximum.

---

152. Large Data Structures

The grammar must permit structurally large:

vectors
matrices
tensors
expressions
argument lists
programs
modules

without defining artificial language maximums.

Implementation resource exhaustion must be handled outside grammar semantics.

---

153. Performance Without Semantic Limits

Compiler/parser engineering may optimize:

- memory usage;
- parsing speed;
- AST construction;
- semantic analysis;
- IR generation.

These optimizations must not change the source language merely to accommodate one implementation's current capacity.

---

154. Classical File Independence Checklist

Before marking any existing classical file complete, verify:

[ ] Purpose defined
[ ] Ownership defined
[ ] Non-ownership defined
[ ] Inputs defined
[ ] Outputs defined
[ ] Dependencies defined
[ ] Upstream contracts defined
[ ] Downstream consumers defined
[ ] Grammar contract defined
[ ] AST contract defined
[ ] Semantic contract defined
[ ] IR contract defined
[ ] Compiler contract defined
[ ] Runtime contract defined
[ ] Resource contract defined
[ ] Capability contract defined
[ ] Cross-domain contract defined
[ ] Diagnostics defined
[ ] Compatibility defined
[ ] Scalability defined
[ ] Hard-coding audit defined
[ ] Positive tests defined
[ ] Negative tests defined
[ ] Boundary tests defined
[ ] Determinism tests defined
[ ] Completion criteria defined

---

155. Existing File Preservation Rule

Do not unnecessarily rename:

README.md
classical.g4
arithmetic.g4
classical-accelerators.g4
control.g4
floating-point.g4
integer.g4
intrinsics.md
linear-algebra.g4
matrix.g4
numeric.g4
numerical.g4
optimization.g4
scalar.g4
scientific-computing.g4
signal-processing.g4
statistics.g4
symbolic.g4
tensor.g4
vector.g4

The current repository already has meaningful decomposition.

The objective is to correct ownership and integration, not to rename files for aesthetic reasons.

---

156. No New Classical Subdirectories Required Yet

The existing classical directory already has substantial decomposition.

Do not create additional subdirectories merely to make the tree look more modular.

New subdirectories are justified only when there is a genuine independent ownership boundary.

For example, test organization belongs naturally under:

grammar/tests/classical/

rather than creating a second test hierarchy under:

grammar/classical/tests/

unless the repository-wide architecture explicitly requires it.

---

157. No Parallel Classical Grammar

Do not create:

grammar/classical2/
grammar/math/
grammar/numerics/
grammar/scientific/
grammar/computing/

to replace the existing directory.

The existing classical directory is the domain boundary.

---

158. No Second Expression Language

Do not introduce:

ClassicalExpression
ScientificExpression
NumericalExpression
TensorExpression
MatrixExpression

as independent precedence systems.

Use canonical:

expression

and classify semantics downstream.

---

159. No Second Type System

Do not introduce:

ClassicalType
NumericType
MatrixTypeSystem
TensorTypeSystem

as competing type systems.

Use:

grammar/types/

as the canonical type grammar and semantic type architecture.

---

160. No Classical IR Fragmentation

Classical subdomains must not each invent:

VectorIR
MatrixIR
TensorIR
NumericalIR
SignalIR
StatisticsIR
OptimizationIR

unless the compiler architecture explicitly establishes these as implementation-level representations under a single canonical IR architecture.

The grammar itself does not own those representations.

---

161. No Vendor Lock-In

Classical grammar must remain vendor-neutral.

Vendor-specific capabilities belong to:

dialects/
interoperability/
hardware/
resources/
compile/

as appropriate.

---

162. Future-Proof Operation Names

The operation model should permit future names without grammar redesign.

A future operation such as:

future::new_compute_operation(x)

should structurally fit the same operation model as today's operations.

Semantic validation determines whether it is available.

---

163. Capability Model

Classical capabilities may include concepts such as:

scalar.compute
vector.compute
matrix.compute
tensor.compute
symbolic.compute
numerical.compute
distributed.compute
accelerator.compute

These are capability concepts, not device names.

The canonical capability system remains authoritative.

---

164. Resource Model

Classical programs may require resources such as:

memory
compute
storage
bandwidth
latency
parallelism
accelerator capability

The resource system evaluates them.

The grammar must not discover actual resource quantities.

---

165. Semantic Resource Example

The intended separation is:

program:
    requires memory >= required_memory

followed by:

resource analysis:
    determine required_memory

followed by:

environment:
    determine available memory

followed by:

realization:
    choose an implementation

This separation is essential to POCO-REAF.

---

166. Classical Program Portability

A classical program should remain portable unless it explicitly introduces a portability-restricting semantic requirement.

For example:

vector computation

can remain portable.

A construct explicitly requiring:

vendor::specific_feature

may intentionally narrow portability.

That distinction must be visible to tooling.

---

167. Compiler Responsibility

The compiler is responsible for:

- semantic analysis;
- type checking;
- shape checking;
- effect analysis;
- capability analysis;
- resource analysis;
- optimization;
- lowering;
- target selection;
- code generation;
- diagnostics.

The grammar is responsible only for syntax.

---

168. Runtime Responsibility

The runtime is responsible for:

- execution;
- resource acquisition;
- dispatch;
- dynamic availability;
- runtime policies;
- recovery;
- observability;
- deployment.

The grammar has no runtime responsibility.

---

169. Hardware Responsibility

Hardware/HAL is responsible for:

- actual device capabilities;
- physical resources;
- device state;
- topology;
- calibration;
- hardware-specific realization.

Classical grammar remains hardware-independent.

---

170. Numerical Backend Responsibility

Numerical backends may choose:

- scalar implementation;
- vector implementation;
- GPU implementation;
- FPGA implementation;
- distributed implementation;
- specialized accelerator;
- future implementation.

The source semantic operation remains unchanged.

---

171. Classical Semantics Across Targets

For an operation:

linalg::matmul(A, B)

the realization could differ:

CPU:
    scalar/vector implementation

GPU:
    parallel implementation

FPGA:
    pipelined implementation

ASIC:
    dedicated datapath

distributed:
    partitioned implementation

The grammar does not change.

This is the intended POCO-REAF behavior.

---

172. Quantum Example

For:

classical result = measure(...);

the classical side receives a semantic value.

The quantum side remains governed by:

grammar/quantum/

and:

quantum::ir

The classical grammar does not need to know the physical QPU layout.

---

173. HDL Example

A classical algorithm may provide computation that is eventually implemented in hardware.

The classical grammar does not need to encode:

wire [31:0]

as a universal assumption.

Widths and hardware structure belong to the appropriate type/HDL/hardware semantics.

---

174. Documentation Examples

Examples using:

4
8
16
32
64
1024
4096

are demonstration values only.

Documentation must never imply that those values are language maximums unless explicitly defined as program semantics.

---

175. Completion Status Discipline

This README describes architecture.

It must not falsely mark every existing grammar file as fully implemented.

Actual status must be established through:

grammar.md

and repository conformance tests.

Recommended statuses:

SPECIFIED
IMPLEMENTED
PARTIALLY IMPLEMENTED
PLANNED
DEPRECATED

---

176. Classical Production Gate

The classical subsystem is production-ready only when:

SPECIFICATION
    |
    v
LEXER
    |
    v
GRAMMAR
    |
    v
PARSER
    |
    v
AST
    |
    v
SEMANTICS
    |
    v
RESOURCE/CAPABILITY ANALYSIS
    |
    v
IR
    |
    v
OPTIMIZATION
    |
    v
SCHEDULING/PLACEMENT
    |
    v
LOWERING
    |
    v
RUNTIME/HARDWARE

has a traceable contract for every stable feature.

---

177. Repository-Wide Production Gate

Before declaring classical grammar production-ready, verify:

[ ] grammar/DESIGN.md agrees
[ ] grammar/README.md agrees
[ ] grammar/Zamani.g4 integrates classical domain
[ ] grammar/grammar.md accurately reports status
[ ] Zamani-Grammar.md features have not silently become syntax
[ ] lexer tokens are canonical
[ ] AST is domain-neutral
[ ] semantic ownership is defined
[ ] canonical IR boundary is preserved
[ ] quantum::ir remains canonical
[ ] QEC remains separate
[ ] ZQN remains separate
[ ] routing remains separate
[ ] scheduling remains separate
[ ] hardware discovery remains separate
[ ] resource discovery remains separate
[ ] runtime remains separate
[ ] no fixed machine limits exist
[ ] no fixed algorithm inventory exists
[ ] no vendor lock-in exists
[ ] Rust 1.97/1.97.1 is supported
[ ] Rust 2021 is supported
[ ] unsafe Rust is not required
[ ] positive tests pass
[ ] negative tests pass
[ ] boundary tests pass
[ ] scalability tests pass
[ ] deterministic parsing tests pass
[ ] compatibility tests pass
[ ] cross-domain tests pass

---

178. Final Classical Architecture

The production architecture is:

                         ZAMANI SOURCE
                              |
                              v
                       CANONICAL LEXER
                              |
                              v
                      CANONICAL PARSER
                              |
                              v
                      classical.g4
                              |
              +---------------+----------------+
              |               |                |
              v               v                v
          scalar          vector            matrix
              |               |                |
              +---------------+----------------+
                              |
              +---------------+----------------+
              |               |                |
              v               v                v
           tensor        numerical        symbolic
              |               |                |
              +---------------+----------------+
                              |
              +---------------+----------------+
              |               |                |
              v               v                v
       linear algebra     optimization     scientific
              |               |                |
              +---------------+----------------+
                              |
                     DOMAIN-NEUTRAL AST
                              |
                              v
                    SEMANTIC ANALYSIS
                              |
        +---------------------+---------------------+
        |                     |                     |
        v                     v                     v
     types                effects              resources
        |                     |                     |
        +---------------------+---------------------+
                              |
                              v
                   CAPABILITY ANALYSIS
                              |
                              v
                  CANONICAL SEMANTIC IR
                              |
                              v
                         OPTIMIZATION
                              |
          +-------------------+-------------------+
          |                   |                   |
          v                   v                   v
      scheduling           placement           routing
          |                   |                   |
          +-------------------+-------------------+
                              |
                              v
                       TARGET LOWERING
                              |
       +----------+-----------+-----------+----------+
       |          |           |           |          |
       v          v           v           v          v
      CPU        GPU         FPGA        ASIC      Future
       |          |           |           |        target
       +----------+-----------+-----------+----------+
                              |
                              v
                           RUNTIME

---

179. Final Ownership Invariant

The permanent responsibility separation is:

GRAMMAR
    describes syntax

AST
    represents source structure

SEMANTIC ANALYSIS
    determines meaning

TYPE SYSTEM
    determines type correctness

EFFECT SYSTEM
    determines effects

RESOURCE SYSTEM
    determines requirements

CAPABILITY SYSTEM
    determines required capabilities

IR
    represents compiler semantics

OPTIMIZER
    improves realization

SCHEDULER
    orders execution

ROUTER
    determines mapping where applicable

RESILIENCE
    handles execution robustness

QEC
    handles quantum error correction

ZQN
    handles quantum noise/fault semantics

HARDWARE/HAL
    exposes actual target capabilities

COMPILER
    produces target realization

RUNTIME
    executes

Never reverse these responsibilities.

---

180. Ultimate Classical Principle

The classical grammar must preserve one fundamental invariant:

«One Zamani program describes one semantic computation; many machines may realize that computation differently.»

Therefore:

ONE PROGRAM
    |
    +--> tiny processor
    |
    +--> CPU
    |
    +--> multicore CPU
    |
    +--> vector processor
    |
    +--> GPU
    |
    +--> FPGA
    |
    +--> ASIC
    |
    +--> accelerator
    |
    +--> cluster
    |
    +--> supercomputer
    |
    +--> distributed system
    |
    +--> cloud
    |
    +--> edge
    |
    +--> simulator
    |
    +--> future architecture

without making today's machine characteristics into tomorrow's language restrictions.

The classical grammar therefore exists to describe:

WHAT COMPUTATION MEANS
WHAT TYPES MEAN
WHAT SHAPES MEAN
WHAT OPERATIONS MEAN
WHAT REQUIREMENTS EXIST
WHAT CAPABILITIES ARE REQUIRED
WHAT CONSTRAINTS APPLY

It does not prescribe:

WHICH CPU
WHICH GPU
WHICH FPGA
WHICH ASIC
WHICH DEVICE
WHICH CORE
WHICH THREAD
WHICH REGISTER
WHICH MEMORY BANK
WHICH NODE
WHICH TOPOLOGY
WHICH VENDOR
WHICH SCHEDULER
WHICH ROUTING ALGORITHM
WHICH OPTIMIZATION IMPLEMENTATION

Those decisions belong downstream.

---

181. Final POCO-REAF Invariant

The production classical architecture is therefore:

                  ZAMANI PROGRAM
                        |
                        v
                PORTABLE SEMANTICS
                        |
                        v
              CLASSICAL DOMAIN MODEL
                        |
                        v
             RESOURCE + CAPABILITY
                  REQUIREMENTS
                        |
                        v
                 CANONICAL IR
                        |
                        v
              TARGET INDEPENDENT
                 OPTIMIZATION
                        |
                        v
              TARGET REALIZATION
                        |
       +----------------+----------------+
       |                |                |
       v                v                v
      CPU              GPU              FPGA
       |                |                |
       +----------------+----------------+
                        |
              +---------+---------+
              |                   |
              v                   v
             ASIC                QPU/
                                 future
                        |
                        v
                     RUNTIME

The language meaning remains above the realization boundary.

The resource environment determines how that meaning is realized.

This is the classical-computing foundation required for:

«Zamani — From Atom to Everywhere»

and:

«Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF).»

The decisive rule is:

«Resource availability may constrain execution, but resource availability must never become an accidental constraint on the Zamani language itself.»

And the decisive classical-domain rule is:

«Classical computing is an open, target-independent semantic domain of Zamani—not a fixed catalogue of today's CPUs, algorithms, numerical libraries, accelerators, or hardware limits.»