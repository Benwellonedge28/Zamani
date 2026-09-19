Zamani Function Grammar

Production Architecture, Ownership, Integration, and Completion Contract

Path: "grammar/functions/"
Language: Zamani
Grammar technology: ANTLR 4
Rust implementation baseline: Rust 1.97 / Rust 1.97.1
Rust edition: 2021
Rust safety requirement: "unsafe" Rust is prohibited
Architectural objective: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)
Scalability objective: From the smallest supported computation to arbitrarily large programs and machines, subject only to actual available resources and implementation capacity
Status: Production architecture contract

---

1. Purpose

"grammar/functions/" owns the source-language grammar for functions and function-level syntax in Zamani.

Functions are a universal abstraction. The same function mechanism must be capable of expressing computation involving:

- classical computation;
- quantum computation;
- hybrid quantum-classical computation;
- HDL and hardware/software co-design;
- AI/ML;
- tensors and data processing;
- distributed computation;
- parallel/HPC computation;
- networking;
- cryptography;
- scientific computation;
- embedded computation;
- accelerators;
- edge/cloud execution;
- future computational paradigms.

The function grammar therefore describes portable source-level computation and intent.

It must not describe a particular machine.

The governing rule is:

«Function syntax describes computation and portable intent; semantic analysis and downstream compilation systems determine how that computation is realized.»

This separation is mandatory for POCO-REAF.

---

2. Architectural Objective

The function grammar participates in the following repository-wide pipeline:

Zamani source
    │
    ▼
canonical lexical layer
    │
    ▼
canonical parser
    │
    ▼
domain-neutral frontend AST
    │
    ▼
structural validation
    │
    ├── name/scope analysis
    ├── type analysis
    ├── generic analysis
    ├── effect analysis
    ├── ownership/resource analysis
    ├── capability analysis
    ├── contract analysis
    └── portability analysis
    │
    ▼
semantic model
    │
    ├───────────────┬────────────────────┬───────────────────┐
    ▼               ▼                    ▼
classical IR    quantum::ir       HDL/hardware IR
    │               │                    │
    └───────────────┴────────────────────┘
                    │
                    ▼
               optimization
                    │
          ┌─────────┼─────────┐
          ▼         ▼         ▼
       routing   scheduling  resilience
                                │
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
                        target realization
                                │
                                ▼
                             runtime

"grammar/functions/" participates only in the source-language syntax and parser composition layer.

It must not become:

- a semantic type system;
- a compiler IR;
- a runtime language;
- an ABI implementation;
- a hardware model;
- a quantum IR;
- a QEC implementation;
- a routing implementation;
- a scheduler;
- a resource allocator;
- a linker;
- a deployment engine.

---

3. Repository Authority

The function grammar must follow the repository-wide authority model.

The authority chain is:

language specification
        │
        ▼
canonical grammar composition
        │
        ▼
ANTLR parser
        │
        ▼
frontend AST
        │
        ▼
semantic model
        │
        ▼
canonical IR/domain IR
        │
        ▼
compiler/runtime realization

The following files are not interchangeable authorities:

grammar/Zamani.g4
grammar/grammar.md
grammar/Zamani-Grammar.md
src/lexer.rs
src/parser.rs
src/ast/...

Their responsibilities must remain distinct.

"Zamani.g4" is the grammar composition root.

"grammar/grammar.md" describes implementation/specification conformance.

"Zamani-Grammar.md" remains a broader design/reference surface and must not silently introduce accepted syntax.

"src/lexer.rs", "src/parser.rs", and the frontend AST determine what the current Rust implementation actually accepts and represents.

A function feature is production-complete only when its specification, grammar, AST, semantic, IR, compiler, runtime, and tests are aligned.

---

4. Directory Ownership

4.1 This directory owns

"grammar/functions/" owns source syntax for:

- named functions;
- function declarations;
- function definitions;
- function signatures;
- parameters;
- return clauses;
- generic function parameters;
- generic function constraints;
- function contracts;
- closures where delegated to this directory;
- lambda/function-value syntax where delegated to this directory;
- asynchronous functions;
- generators;
- compile-time functions;
- foreign/external function declarations;
- function-level calling-convention attachments;
- function-level modifiers;
- function-level metadata attachment points;
- function-level effects where composed through the canonical effects grammar;
- function-level requirements/capability attachment points where specified by the language.

4.2 This directory does not own

It does not own the semantic meaning of:

- identifiers;
- names;
- paths;
- types;
- expressions;
- statements;
- blocks;
- modules;
- packages;
- imports;
- exports;
- effects;
- resources;
- capabilities;
- ownership;
- borrowing;
- lifetimes;
- memory allocation;
- hardware;
- topology;
- quantum operations;
- quantum state semantics;
- QEC;
- ZQN;
- routing;
- scheduling;
- optimization;
- HAL;
- runtime execution;
- linking;
- loading;
- deployment;
- target selection;
- physical device allocation;
- ABI implementation;
- register allocation;
- stack layout.

Those remain owned by their canonical repository subsystems.

---

5. Actual File Set

The current directory contains the following function-related files.

These existing filenames must be retained unless there is an independently justified repository-wide migration:

grammar/functions/
├── README.md
├── async.g4
├── calling-conventions.g4
├── closures.g4
├── compile-time-functions.g4
├── constraints.g4
├── contracts.g4
├── foreign-functions.g4
├── functions.g4
├── generators.g4
├── generics.g4
├── lambdas.g4
├── parameters.g4
└── returns.g4

No unnecessary renaming is required.

In particular:

- keep "returns.g4";
- keep "calling-conventions.g4";
- keep "lambdas.g4";
- keep "foreign-functions.g4";
- keep "compile-time-functions.g4".

The README must accurately describe the files that actually exist.

---

6. File Ownership Matrix

File| Canonical responsibility
"functions.g4"| Composition/framing of ordinary named function declarations and definitions
"parameters.g4"| Function parameter-list syntax
"returns.g4"| Function return-clause syntax
"generics.g4"| Generic function parameter syntax
"constraints.g4"| Function generic constraint syntax
"contracts.g4"| Function contract syntax
"closures.g4"| Closure syntax
"lambdas.g4"| Lambda/function-value syntax where distinct from closure framing
"async.g4"| Asynchronous function syntax/modifiers
"generators.g4"| Generator syntax
"compile-time-functions.g4"| Compile-time function declaration/invocation syntax
"foreign-functions.g4"| Function declarations whose implementation is external
"calling-conventions.g4"| Function-level calling-convention attachment syntax
"README.md"| Architecture, ownership, integration, invariants, validation and completion contract

No file may silently become the owner of another file's grammar.

---

7. Single-Authority Rule

Every grammar production must have exactly one canonical owner.

The intended ownership is:

function declaration
    → functions.g4

parameter list
    → parameters.g4

return clause
    → returns.g4

generic parameters
    → generics.g4

generic constraints
    → constraints.g4

contracts
    → contracts.g4

closures
    → closures.g4

lambdas
    → lambdas.g4

async function syntax
    → async.g4

generator syntax
    → generators.g4

compile-time functions
    → compile-time-functions.g4

foreign function declarations
    → foreign-functions.g4

calling-convention attachment
    → calling-conventions.g4

The following must not be independently redefined in multiple function files:

identifier
qualifiedName
typeExpression
expression
statement
block
attribute
parameterList
returnClause
genericParameterList
constraintClause
effectClause
contractClause

They must be consumed from their canonical owners.

---

8. "functions.g4"

8.1 Purpose

"functions.g4" is the composition root for ordinary named function declarations.

It owns function framing.

It does not own the internals of parameters, types, expressions, contracts, generics, effects, or statements.

Conceptually:

attributes*
modifiers*
function keyword
function name
generic parameters?
parameter list
return clause?
constraints?
effects?
contracts*
body | prototype

For example:

fn add(a: Int, b: Int) -> Int {
    return a + b;
}

or:

async fn compute<T>(value: T) -> Result<T> {
    ...
}

The exact accepted syntax is determined by the canonical grammar and language specification.

8.2 Function composition must remain domain-neutral

Do not create separate universal function grammars such as:

cpuFunction
gpuFunction
fpgaFunction
qpuFunction
cudaFunction
quantumFunction
aiFunction
hdlFunction
distributedFunction

The same function mechanism must serve every domain.

Domain meaning enters through:

- types;
- declarations;
- effects;
- capabilities;
- resource requirements;
- contracts;
- attributes;
- semantic analysis.

---

9. "parameters.g4"

Purpose

"parameters.g4" owns the syntax of function parameters.

It must permit arbitrary parameter-list length subject only to parser/compiler resource capacity.

The grammar must not impose:

MAX_PARAMETERS
MAX_ARGUMENTS
MAX_GENERIC_PARAMETERS

A function with:

zero parameters
one parameter
many parameters

must use the same grammar mechanism.

Semantic boundary

"parameters.g4" does not own:

- type semantics;
- type inference;
- ownership;
- borrowing;
- lifetimes;
- memory allocation;
- ABI lowering;
- calling convention selection;
- resource allocation.

Example:

fn process(value: Tensor<T>) -> Tensor<T>

The parameter structure belongs to "parameters.g4".

"Tensor<T>" belongs to the canonical type system.

---

10. "returns.g4"

Purpose

"returns.g4" owns the syntax attaching a result specification to a function.

Examples:

fn compute() -> Int

fn transform<T>(value: T) -> T

fn measure(q: Qubit) -> Measurement

The return type itself belongs to the canonical type system.

"returns.g4" must not enumerate concrete types.

Do not create return-specific lists such as:

Int
Float
Qubit
Tensor
Measurement
...

Every valid Zamani type must become usable as a function result through the canonical type-expression mechanism.

---

11. "generics.g4"

Purpose

"generics.g4" owns generic function parameter syntax.

Examples:

<T>

<T, U>

<T: Numeric>

The generic mechanism must be open-ended.

There must be no grammar-level maximum number of:

- generic parameters;
- generic arguments;
- bounds;
- associated constraints.

Does not own

"generics.g4" does not implement:

- type inference;
- generic substitution;
- trait solving;
- specialization;
- monomorphization;
- associated-type resolution;
- capability resolution;
- constraint solving.

The pipeline is:

generic syntax
    ↓
AST generic representation
    ↓
semantic generic environment
    ↓
constraint/type solving
    ↓
specialization/lowering where required

---

12. "constraints.g4"

Purpose

"constraints.g4" owns source syntax for constraints associated with generic functions.

Constraints may express requirements over:

- types;
- capabilities;
- traits/interfaces;
- effects;
- resources;
- semantic properties.

The grammar must represent constraints without hard-coding a finite list of future capabilities.

For example, the syntax may express a symbolic requirement, while semantic analysis determines whether the requirement is satisfiable.

Separation

A constraint is not automatically:

- a hardware allocation;
- a device selection;
- a compiler optimization;
- a scheduler decision.

For example:

requires capability("tensor.compute")

is fundamentally different from:

use GPU 0

The first expresses portable intent.

The second is target realization and must not become a universal function-language requirement.

---

13. "contracts.g4"

Purpose

"contracts.g4" owns function-level contract syntax.

Contracts may express source-level properties such as:

- preconditions;
- postconditions;
- invariants;
- behavioral requirements;
- purity/semantic guarantees where specified;
- resource/capability requirements where explicitly defined by the language specification.

Contracts must remain declarative.

They must not execute compiler/runtime actions while parsing.

Contract pipeline

contract syntax
    ↓
AST contract
    ↓
semantic validation
    ↓
verification/analysis
    ↓
optimization/lowering/runtime enforcement as appropriate

The grammar must not decide how a contract is proven or enforced.

---

14. "closures.g4"

Purpose

"closures.g4" owns closure syntax.

A closure must be capable of capturing values according to the language's semantic ownership model.

The grammar does not implement:

- capture analysis;
- ownership;
- borrowing;
- lifetime inference;
- heap allocation;
- stack allocation;
- closure representation;
- ABI lowering.

Those belong downstream.

Scalability

There must be no grammar-level maximum on:

- captured values;
- parameters;
- nesting depth;
- closure count.

Implementation resource exhaustion is not language semantics.

---

15. "lambdas.g4"

Purpose

"lambdas.g4" owns lambda/function-value syntax where that syntax is distinct from ordinary named functions and closures.

It must integrate with:

expressions/
functions/
types/
core/

without creating a second function type system.

The result of a lambda expression must map into the canonical AST representation for callable/function values.

The semantic layer determines:

- capture behavior;
- inferred types;
- effects;
- ownership;
- lifetime;
- calling behavior;
- lowering.

"lambdas.g4" must not create a lambda-specific IR.

---

16. "async.g4"

Purpose

"async.g4" owns source syntax for asynchronous functions.

It must compose with:

- "functions.g4";
- "parameters.g4";
- "generics.g4";
- "returns.g4";
- canonical types;
- expressions;
- statements;
- concurrency grammar.

Asynchronous syntax must not imply a fixed implementation strategy.

The grammar must not encode:

N threads
N cores
N tasks
N workers
N nodes

The runtime/scheduler determines realization.

The semantic model may distinguish concepts such as:

async computation
awaitable computation
task
future
stream
actor

without committing the grammar to a particular runtime implementation.

---

17. "generators.g4"

Purpose

"generators.g4" owns generator syntax.

Generators must support potentially unbounded logical sequences without a grammar-level size limit.

The grammar must not encode:

MAX_YIELDS
MAX_GENERATOR_DEPTH
MAX_ELEMENTS

The semantic/runtime layers determine:

- state-machine lowering;
- storage;
- scheduling;
- suspension/resumption;
- cancellation;
- resource usage.

---

18. "compile-time-functions.g4"

Purpose

"compile-time-functions.g4" owns syntax for functions explicitly designated to participate in compile-time computation.

It must remain separate from ordinary runtime functions while reusing the canonical:

- function syntax;
- parameters;
- generics;
- types;
- expressions;
- contracts;
- capabilities.

Compile-time execution must not become an unrestricted parser escape hatch.

The grammar does not grant permission for:

- arbitrary filesystem access;
- arbitrary network access;
- arbitrary process execution;
- secret extraction;
- hardware discovery;
- runtime device mutation.

Such capabilities require explicit language/security/compilation semantics.

The compile-time function model must remain deterministic where the language requires reproducible builds.

---

19. "foreign-functions.g4"

Purpose

"foreign-functions.g4" owns source syntax for functions whose implementation exists outside the current Zamani compilation unit or language.

Examples may include external functions implemented in:

- C;
- C++;
- Rust;
- Python;
- Fortran;
- HDL;
- another Zamani dialect;
- another interoperable computational representation.

The grammar must use symbolic identities rather than an exhaustive language list.

For example, the external language may be represented as metadata rather than:

C
C++
CUDA
Python
Rust
...

as permanently hard-coded grammar alternatives.

Ownership

This file does not own:

- linking;
- symbol lookup;
- dynamic loading;
- ABI implementation;
- foreign type layout;
- object-file generation;
- deployment.

Those belong to interoperability/compiler/runtime systems.

---

20. "calling-conventions.g4"

Purpose

"calling-conventions.g4" owns the function-level source syntax for referring to a calling convention.

It must not implement an ABI.

The distinction is mandatory:

calling-convention syntax
        ≠
calling-convention semantics
        ≠
ABI implementation
        ≠
target lowering

ABI contracts remain associated with:

grammar/interoperability/abi.g4

where appropriate.

Open-world requirement

The grammar must not permanently enumerate every known ABI.

Do not turn this into:

c
cdecl
stdcall
fastcall
thiscall
sysv64
win64
aapcs
vectorcall
...

as a finite universal language list.

Future ABIs and conventions must be representable without modifying the function grammar.

A convention reference is a symbolic source-level contract.

Its existence and target support are semantic/compiler concerns.

Hardware neutrality

A calling convention must not imply:

CPU 0
GPU 0
QPU 0
FPGA 0
physical register
physical address
specific core
specific node

Those are downstream target-realization concerns.

---

21. Function-Level Modifiers

Function modifiers must remain compositional.

Examples can include concepts such as:

- visibility;
- async;
- compile-time;
- external;
- static;
- abstract;
- virtual;
- override;
- domain-independent semantic modifiers defined by the specification.

A modifier must have exactly one semantic owner.

If a modifier is universally applicable, it belongs in the appropriate core/function composition grammar.

If it is domain-specific, it belongs to that domain's grammar and must integrate through the canonical function extension point.

Do not create a separate function language for every domain.

---

22. Attributes

Function attributes must use the canonical attribute grammar.

Do not redefine attribute syntax in every function file.

The relationship is:

function
    ↓
attribute attachment point
    ↓
canonical attribute grammar
    ↓
semantic attribute interpretation

Attributes may provide extensibility without forcing every future capability to become a new reserved keyword.

However, attributes must not become an uncontrolled semantic escape hatch.

Their namespace, ownership, versioning, validation and meaning must remain defined by the repository's attribute/specification system.

---

23. Effects

Functions may interact with the canonical effects system.

The function grammar only provides the attachment point.

Effect semantics remain owned by:

grammar/effects/

The relationship is:

function
    ↓
effect clause
    ↓
canonical effect grammar
    ↓
effect AST
    ↓
effect checking
    ↓
semantic model

The function grammar must not duplicate effect declarations or handlers.

---

24. Resource and Capability Integration

Function declarations must be capable of expressing portable resource/capability intent where the language specification permits it.

The important distinction is:

requirement
constraint
capability
preference
hint
implementation decision

These are not interchangeable.

For example:

requires capability("quantum.measurement")

is a portable semantic requirement.

It is not:

use QPU 0

Likewise:

requires memory >= required

must not become a grammar-level statement such as:

MAX_MEMORY = ...

Function syntax must remain independent of today's machine sizes.

---

25. Classical Integration

A classical function may lower toward the repository's classical IR.

The grammar must not know:

- CPU architecture;
- register width;
- number of cores;
- SIMD width;
- cache size;
- physical memory size.

The function grammar describes the computation.

Target-specific realization occurs later.

---

26. Quantum Integration

Quantum functions remain ordinary Zamani functions.

For example, a quantum function may conceptually have:

fn transform(q: Qubit) -> Measurement {
    ...
}

or generic/parameterized quantum computation.

The function grammar must not introduce:

quantumFunction
QuantumFunction
QuantumGateFunction
QPUFunction

as separate universal languages.

Quantum meaning is provided by the quantum type/operation/semantic systems.

The required lowering boundary is:

Zamani function syntax
        ↓
domain-neutral AST
        ↓
semantic quantum representation
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
        ↓
target realization

"grammar/functions/" must never create a second quantum IR.

---

27. Hybrid Quantum-Classical Integration

The same function mechanism must support:

classical computation
        ↓
quantum operation
        ↓
measurement
        ↓
classical decision
        ↓
quantum operation

without requiring separate function syntaxes.

Hybrid meaning is determined by:

- types;
- effects;
- quantum semantics;
- classical semantics;
- capabilities;
- resource requirements;
- semantic analysis.

The function grammar remains common.

---

28. HDL and Hardware Co-design Integration

Functions may represent computations associated with hardware/software co-design.

The grammar must not hard-code:

32-bit
64-bit
8 cores
16 lanes
N FPGAs
N accelerators
N memory banks

unless such values are explicitly part of the program's semantic data or parameterization.

A function can express hardware-related intent through the canonical hardware/resource systems.

For example, conceptually:

requires capability("accelerated.compute")

rather than binding the source program to a particular accelerator instance.

The function grammar remains target-neutral.

---

29. Distributed and Parallel Integration

Functions must support distributed and parallel computation without embedding topology.

There must be no function-level language rule equivalent to:

run_on_8_nodes
run_on_16_cores
run_on_gpu_0

unless a future specification explicitly defines such syntax as target-specific deployment intent rather than universal function semantics.

The portable model is:

function
    ↓
parallel/distributed semantic intent
    ↓
resource/capability model
    ↓
compiler
    ↓
placement/scheduling

The actual number of workers, nodes, devices and execution units is determined downstream.

---

30. AI/ML Integration

AI/ML functions must use the same function grammar.

Examples include functions that operate on:

- tensors;
- datasets;
- models;
- gradients;
- probabilistic values;
- agents;
- inference pipelines;
- training computations.

The function grammar must not enumerate:

PyTorchFunction
TensorFlowFunction
JAXFunction
CUDAFunction
...

Framework-specific semantics belong to interoperability/domain libraries and semantic systems.

---

31. Generic Scalability Rule

All function-related repetition must be represented structurally.

The grammar must not define artificial finite limits for:

functions
parameters
arguments
generic parameters
generic arguments
constraints
contracts
attributes
captures
effects
returns
nested functions
closures
lambdas
generator yields

A grammar such as:

X*
X?
X+

or equivalent structural composition is preferred over enumerated finite alternatives.

This permits source-language scaling from tiny programs to arbitrarily large programs, subject only to actual parser/compiler/resource capacity.

---

32. POCO-REAF

POCO-REAF means:

«Program Once, Compile Once, Run Everywhere, Anywhere, Forever.»

The function grammar contributes to this objective by expressing portable computation rather than target realization.

A function should describe:

what the computation means
what types it accepts
what it returns
what effects it has
what capabilities it requires
what semantic contracts it guarantees
what portable resource constraints matter

It should not unnecessarily describe:

which CPU
which core
which GPU
which FPGA
which QPU
which physical qubit
which memory bank
which network node
which register
which physical address

This is the central portability invariant of the directory.

---

33. Hard-Coding Prohibition

The function grammar must not contain universal implementation limits such as:

MAX_FUNCTIONS
MAX_PARAMETERS
MAX_ARGUMENTS
MAX_GENERIC_PARAMETERS
MAX_GENERIC_ARGUMENTS
MAX_CONSTRAINTS
MAX_CONTRACTS
MAX_CLOSURES
MAX_LAMBDAS
MAX_THREADS
MAX_CORES
MAX_GPUS
MAX_FPGAS
MAX_QPUS
MAX_QUBITS
MAX_NODES
MAX_MEMORY
MAX_STORAGE
MAX_DEVICES
MAX_ACCELERATORS

It must also not encode universal physical identifiers such as:

cpu0
gpu0
qpu0
fpga0
device0
physical_qubit0
memory_bank0
core0
register0

as part of ordinary portable function semantics.

---

34. Semantic Constants Versus Implementation Limits

The prohibition on hard-coding does not prohibit legitimate program constants.

For example:

const batch_size = 1024;

may be meaningful program semantics.

Likewise:

Tensor<1024, 1024>

may be legitimate program data/type information.

What is prohibited is turning that into a language implementation ceiling:

MAX_TENSOR_DIMENSION = 1024

or:

MAX_PARAMETERS = 1024

The distinction is:

program-defined value
        ≠
compiler-imposed universal limit

---

35. Target Independence

Function syntax must not depend on:

- operating system;
- CPU architecture;
- GPU architecture;
- FPGA family;
- QPU architecture;
- vendor;
- compiler backend;
- runtime implementation;
- deployment topology.

Target information may enter through the appropriate downstream target/resource/capability systems.

---

36. ABI Boundary

The function grammar must distinguish:

function signature

from:

ABI representation

and:

machine calling convention implementation

The function signature describes language semantics.

The ABI system determines how that signature crosses an external binary/interface boundary.

Therefore:

grammar/functions/
        │
        ▼
function-level ABI/calling-convention reference
        │
        ▼
grammar/interoperability/
        │
        ▼
ABI semantics
        │
        ▼
compiler/backend lowering

The function grammar must never define register allocation or physical calling sequences.

---

37. Frontend AST Contract

Every function grammar production must have an explicit AST mapping before that grammar production is considered complete.

The required conceptual mapping is:

grammar
    ↓
domain-neutral AST
    ↓
semantic function representation
    ↓
canonical/domain IR

The AST must preserve enough information for downstream semantic analysis, including as appropriate:

- function name;
- source span;
- modifiers;
- attributes;
- generic parameters;
- parameters;
- parameter patterns;
- parameter types;
- return specification;
- constraints;
- effects;
- contracts;
- calling-convention metadata;
- foreign metadata;
- body/prototype;
- compile-time/runtime classification;
- async/generator properties.

The grammar must not force the AST to contain target-specific implementation details.

---

38. Source Span Contract

Every function-related AST construct must retain accurate source provenance.

At minimum, semantic consumers must be able to locate:

- function declaration;
- function name;
- modifiers;
- generic parameters;
- each parameter;
- parameter types;
- return clause;
- constraints;
- effects;
- contracts;
- function body/prototype;
- relevant metadata.

Diagnostics must point to the smallest useful source region.

---

39. Semantic Contract

After parsing, semantic analysis must determine:

- name binding;
- scope;
- duplicate declarations;
- parameter validity;
- generic validity;
- constraint satisfaction;
- type correctness;
- return correctness;
- effect correctness;
- ownership/resource correctness;
- capability requirements;
- contract validity;
- foreign-function validity;
- ABI compatibility;
- compile-time/runtime legality;
- async/generator legality;
- domain-specific semantic constraints.

The parser must not perform these tasks.

---

40. IR Integration Contract

The function grammar must not introduce a function-specific universal IR.

Function information is lowered into the repository's canonical semantic/IR architecture.

Conceptually:

function AST
    ↓
semantic function model
    ↓
appropriate IR

For quantum:

function AST
    ↓
semantic quantum operations
    ↓
quantum::ir

For classical:

function AST
    ↓
classical semantic representation
    ↓
classical IR

For HDL/hardware:

function/co-design AST
    ↓
hardware semantic model
    ↓
HDL/hardware IR

---

41. Compiler Integration

The compiler consumes semantic function information after parsing.

The function grammar must provide enough information for downstream systems to perform:

- type checking;
- generic resolution;
- effect checking;
- optimization;
- specialization where required;
- inlining decisions;
- parallelization;
- vectorization;
- quantum optimization;
- routing;
- scheduling;
- resilience processing;
- ABI lowering;
- target lowering.

The grammar itself must not make these decisions.

---

42. Runtime Integration

Runtime behavior is not defined by parser grammar.

The runtime may need function metadata for:

- invocation;
- asynchronous execution;
- generators;
- closures;
- resource management;
- distributed execution;
- checkpointing;
- recovery;
- observability;
- cancellation;
- capability enforcement.

These are downstream runtime responsibilities.

---

43. Resource Integration

Function resource requirements must integrate with:

grammar/resources/
grammar/hardware/
grammar/execution/
grammar/compile/

The function grammar may provide a composition point.

It must not duplicate the resource grammar.

A portable function can state a requirement.

The resource system determines whether and how that requirement can be satisfied.

---

44. Capability Integration

Capabilities must remain symbolic and extensible.

The grammar should permit future capabilities without requiring a parser release merely because a new accelerator, device, execution model or computational substrate exists.

Therefore prefer symbolic capability references over exhaustive keyword lists.

Conceptually:

capability("some.future.capability")

is extensible.

A finite grammar list of every possible future device is not.

---

45. Determinism

Parsing must depend only on:

- source text;
- canonical token stream;
- grammar version;
- parser configuration that is explicitly part of the language contract.

Parsing must not depend on:

- available CPU;
- available GPU;
- QPU availability;
- hardware discovery;
- network state;
- filesystem state;
- installed libraries;
- environment variables;
- scheduler state;
- calibration state;
- randomness;
- wall-clock time.

Semantic/target resolution may depend on explicitly supplied compilation context, but that must occur after parsing.

---

46. Error and Recovery Contract

Function grammar errors must be diagnosable.

Diagnostics should distinguish at least:

lexical error
syntax error
structural error
semantic error
type error
generic error
contract error
effect error
capability error
resource error
interoperability error
ABI error
target realization error

The parser must not silently convert malformed function syntax into valid but different syntax.

Recovery must preserve useful source spans and must not fabricate semantic constructs.

---

47. Security and Safety

The grammar contains no executable Rust.

The Zamani compiler implementation must use:

Rust 1.97
or
Rust 1.97.1
Edition 2021

and must contain no "unsafe" Rust.

This means:

Rust implementation
    = safe Rust only

The presence of a source-language "unsafe" token in the current lexer does not authorize "unsafe" Rust implementation.

If Zamani eventually specifies an "unsafe" source-language feature, its semantics must be independently specified and validated.

The Rust compiler implementation must nevertheless remain safe Rust.

---

48. No Parser-Time External Effects

Function parsing must not:

- access the filesystem;
- access the network;
- discover hardware;
- invoke processes;
- load dynamic libraries;
- query GPUs;
- query QPUs;
- query FPGA devices;
- perform ABI resolution;
- allocate physical resources;
- execute user code.

These are compiler/tool/runtime responsibilities at explicitly defined later stages.

---

49. Interoperability Integration

The function grammar integrates with:

grammar/interoperability/

for:

- foreign functions;
- ABI contracts;
- FFI;
- external languages;
- calling conventions;
- external representations.

The function directory must not create competing interoperability semantics.

The direction is:

function syntax
        ↓
foreign/calling-convention attachment
        ↓
interoperability semantic model
        ↓
ABI/FFI validation
        ↓
target lowering

---

50. Modules Integration

Functions may be:

- module-private;
- public/exported;
- imported;
- re-exported;
- associated with namespaces.

Module ownership remains with:

grammar/modules/

The function grammar consumes canonical visibility/name/module constructs rather than redefining them.

---

51. Type-System Integration

Function parameter and return syntax must consume canonical type syntax from:

grammar/types/

The function grammar must never define its own parallel type language.

This guarantees that future types automatically become usable in:

- parameters;
- returns;
- generic constraints;
- closures;
- lambdas;
- foreign functions;

without rewriting every function grammar file.

---

52. Expression Integration

Function bodies and lambda expressions consume the canonical expression grammar.

The function grammar must not create another expression grammar.

The relationship is:

function
    ↓
function body
    ↓
block/statement grammar
    ↓
expression grammar

---

53. Statement Integration

Function bodies consume canonical statement/block grammar.

Do not redefine:

if
while
for
match
return
break
continue

inside function grammar unless the repository explicitly assigns that construct to the statement owner.

"return" as a statement remains a statement concern; "returns.g4" owns the function declaration's result clause.

This distinction is important:

-> T

is a return-type/result clause.

return expression;

is a return statement.

They must not be conflated.

---

54. Generic Function Versus Generic Type Ownership

Function generics belong to:

grammar/functions/generics.g4

Generic type semantics belong to:

grammar/types/

Hardware generics belong to the hardware/HDL domain where appropriate, but they must integrate with the canonical generic model rather than creating an unrelated generic language.

For example:

function generic parameter

may parameterize a type, resource abstraction, or semantic computation.

The meaning is resolved semantically.

---

55. Compile-Time Versus Runtime

The grammar must distinguish source-level compile-time functions from ordinary runtime functions where the specification requires that distinction.

However, the grammar must not turn compile-time execution into an unrestricted execution environment.

The compiler must retain explicit boundaries for:

- determinism;
- reproducibility;
- resource consumption;
- permissions;
- dependency tracking;
- diagnostics;
- caching;
- provenance.

---

56. Function Overloading

If Zamani supports function overloading, overload syntax belongs in the function/declaration system.

Overload resolution belongs in semantic analysis.

The grammar must not decide which overload is selected.

The semantic layer must account for:

- argument types;
- generic parameters;
- constraints;
- effects;
- capabilities;
- conversion rules;
- visibility;
- namespace/module context.

---

57. Recursion

Recursive functions are ordinary functions.

The grammar must not impose a maximum recursion depth.

Runtime stack/resource limits are implementation concerns.

Compile-time recursive evaluation may have implementation resource limits, but those limits must not redefine the source-language grammar.

---

58. Function Nesting

If nested functions are permitted, the grammar must represent them using the same function production.

There must be no arbitrary language-level limit such as:

MAX_NESTED_FUNCTION_DEPTH

Semantic legality of nested functions remains a language semantic concern.

---

59. Function Values

When functions are first-class values, the same canonical function/type model must be used.

The architecture should support:

named function
anonymous function
lambda
closure
function reference
higher-order function

without creating unrelated type/IR models.

---

60. Cross-Domain Function Principle

The following should all be expressible through the same function abstraction:

classical function
quantum function
hybrid function
HDL-associated computation
AI function
tensor function
distributed function
parallel function
network function
cryptographic function
scientific function
embedded function
accelerator function
future-domain function

The language must not multiply the function syntax merely because the computational domain changes.

---

61. Feature-Manifest Contract

Each substantial function feature should have a corresponding feature contract in the repository's feature/specification system where such manifests are introduced.

A completed feature contract should identify:

feature ID
feature name
status
language version
grammar owner
lexer requirements
AST representation
semantic representation
IR mapping
compiler consumers
runtime consumers
resource requirements
capability requirements
diagnostics
positive tests
negative tests
boundary tests
scalability tests
determinism tests
compatibility requirements
hard-coding audit

This allows a function feature to be completed independently without waiting for another feature to be redesigned later.

---

62. Per-File Completion Contract

Every file in "grammar/functions/" must be independently completable.

Before marking a file complete, it must have:

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
Diagnostics
Positive Tests
Negative Tests
Boundary Tests
Scalability Tests
Determinism Tests
Compatibility Tests
Security/Safety Audit
Hard-Coding Audit
Completion Criteria

A file is not complete merely because its grammar rules parse.

---

63. Dependency Direction

The preferred dependency direction is:

core
 │
 ├── names
 ├── attributes
 ├── blocks
 └── source units
        │
        ▼
types / expressions / statements
        │
        ▼
functions
        │
        ├── generics
        ├── parameters
        ├── returns
        ├── constraints
        ├── contracts
        ├── closures
        ├── lambdas
        ├── async
        ├── generators
        ├── compile-time
        ├── foreign
        └── calling conventions

Domain grammars consume function constructs.

The function grammar must not depend on concrete target implementations.

---

64. Avoiding Circular Grammar Ownership

The following must be avoided:

functions → types → functions
functions → expressions → functions
functions → statements → functions
functions → interoperability → functions

where both sides define the same production.

Integration points must be narrow and explicit.

For example:

functions.g4
    → parameterList

parameters.g4
    → typeExpression

types/*.g4
    → canonical typeExpression

This is composition, not mutual ownership.

---

65. Generated Grammar and Token Vocabulary

The function parser grammars must consume the repository's canonical token vocabulary.

They must not create a second lexer vocabulary merely for function syntax.

The canonical lexical authority remains the repository's lexer architecture.

Function grammar files should therefore:

- consume canonical tokens;
- avoid duplicate lexer rules;
- avoid defining function-specific duplicate tokens;
- avoid depending on generated implementation details that are not part of the grammar contract.

Any current mismatch between grammar token names and the actual Rust lexer/parser must be resolved through the repository's grammar integration work rather than hidden inside this README.

---

66. Compatibility

Function syntax must be versioned.

Compatibility work must account for:

language specification
Zamani.g4
function grammar components
lexer
parser
frontend AST
semantic model
IR
compiler
runtime
tests

A syntax change is not production-ready until the affected compatibility surface is identified.

Existing valid programs must not silently change meaning.

If a breaking change is required, it must be explicitly versioned and documented.

---

67. Negative Tests

Every function feature must test invalid forms.

Examples include:

missing function name
missing parameter delimiter
malformed parameter type
invalid generic syntax
invalid constraint
invalid return syntax
invalid contract
invalid async combination
invalid generator combination
invalid foreign declaration
invalid calling-convention attachment
duplicate parameter names where prohibited
invalid nesting
invalid modifier combinations

Tests must verify useful diagnostics rather than merely parser failure.

---

68. Boundary Tests

Boundary tests must cover:

zero parameters
one parameter
many parameters
zero generic parameters
many generic parameters
empty body where legal
large function bodies
nested functions where legal
deep generic structures
large contracts
large expressions
large capture sets
large generator structures
large foreign declarations
large metadata sets

No arbitrary finite boundary should be promoted into the language definition.

---

69. Scalability Tests

Scalability testing must verify that the grammar does not introduce artificial resource limits.

The test suite should progressively exercise:

tiny function
small function
large function
very large function
large parameter lists
large generic lists
large constraint sets
large nested structures
large source units
large collections of functions

The objective is:

grammar capability
    scales with actual implementation resources

rather than:

grammar stops at an arbitrary language-defined maximum

---

70. Determinism Tests

Given identical:

source
grammar version
lexer configuration
parser configuration

the parser must produce equivalent results.

Tests must ensure function parsing is independent of:

- CPU;
- GPU;
- QPU;
- FPGA;
- network;
- filesystem;
- environment;
- scheduler;
- runtime state.

---

71. Portability Tests

The function suite must include examples that demonstrate that identical source syntax can represent computation destined for different execution environments without changing the function grammar.

The target differences should be resolved downstream.

Conceptually:

same Zamani function
       │
       ├── CPU realization
       ├── GPU realization
       ├── FPGA realization
       ├── QPU realization
       ├── distributed realization
       └── future realization

The function syntax remains unchanged.

---

72. Quantum Portability Tests

Quantum function tests must avoid establishing an artificial maximum number of qubits.

Test categories should include:

single-qubit function
multi-qubit function
parameterized quantum function
generic quantum function
measurement-returning function
hybrid function
dynamic quantum control
logical-qubit-oriented function
custom quantum operation integration

The grammar must not contain a finite universal gate catalogue as the mechanism for extensibility.

Quantum operation semantics belong downstream.

---

73. Hardware Scalability Tests

Function tests must verify that hardware-associated functions remain portable.

Examples should cover symbolic:

capability requirements
resource requirements
memory requirements
accelerator requirements
latency constraints
reliability constraints
communication requirements

without embedding:

specific CPU
specific GPU
specific QPU
specific FPGA
specific physical qubit
specific core
specific register
specific address

---

74. No Vendor Lock-in

The function grammar must not require syntax changes for every new:

- CPU vendor;
- GPU vendor;
- FPGA vendor;
- QPU provider;
- accelerator;
- operating system;
- ABI;
- cloud provider;
- distributed runtime.

Vendor-specific functionality belongs behind explicit interoperability/dialect/capability mechanisms.

---

75. Dialect Integration

Future dialects may extend function-related syntax only through the canonical dialect mechanism.

A dialect must identify:

name
version
syntax extension
semantic extension
AST mapping
IR mapping
compatibility
capabilities
feature gates

A dialect must not silently redefine the core meaning of ordinary functions.

---

76. Diagnostics

Function diagnostics should identify the appropriate source-level issue without leaking implementation details unnecessarily.

Examples:

E-function-name
E-function-parameters
E-function-return
E-function-generic
E-function-constraint
E-function-contract
E-function-effect
E-function-async
E-function-generator
E-function-foreign
E-function-abi
E-function-capability
E-function-resource

Exact diagnostic identifiers belong to the repository's diagnostic contract.

---

77. Tooling Integration

The function grammar must support tooling such as:

- syntax highlighting;
- completion;
- formatting;
- documentation generation;
- symbol indexing;
- navigation;
- refactoring;
- diagnostics;
- AST inspection;
- semantic analysis.

Tooling must consume the canonical parser/AST rather than independently reimplementing function syntax.

---

78. Documentation Integration

Documentation generators should be able to obtain, where applicable:

function name
signature
generic parameters
constraints
parameters
returns
effects
contracts
attributes
visibility
foreign metadata
calling convention
documentation comments
source location

Documentation is derived from the canonical syntax/AST, not another grammar.

---

79. Repository Integration Contract

Before a function feature is considered production-ready, integration must be checked against at least:

grammar/Zamani.g4
grammar/specification/
grammar/spec/
grammar/core/
grammar/types/
grammar/expressions/
grammar/statements/
grammar/declarations/
grammar/modules/
grammar/effects/
grammar/memory/
grammar/concurrency/
grammar/resources/
grammar/compile/
grammar/execution/
grammar/interoperability/
grammar/dialects/
grammar/validation/
grammar/compatibility/
grammar/tests/

src/lexer.rs
src/parser.rs
src/frontend/ast/
semantic analysis
canonical IR
classical IR
quantum::ir
HDL/hardware IR
compiler
runtime
HAL
scheduling
routing
optimization
QEC
ZQN

Only the relevant downstream consumers need to change for a given feature, but their contracts must be known in advance.

---

80. Production Completion Gate

"grammar/functions/" is production-ready only when all of the following are true:

Authority

- [ ] There is one canonical ownership model.
- [ ] No function grammar file silently competes with another.
- [ ] "Zamani.g4" remains the composition root.
- [ ] "Zamani-Grammar.md" cannot silently introduce accepted syntax.
- [ ] "grammar.md" reflects implementation conformance.

Syntax

- [ ] Named functions are completely specified.
- [ ] Parameters are completely specified.
- [ ] Returns are completely specified.
- [ ] Generics are completely specified.
- [ ] Constraints are completely specified.
- [ ] Contracts are completely specified.
- [ ] Closures are completely specified.
- [ ] Lambdas are completely specified.
- [ ] Async functions are completely specified.
- [ ] Generators are completely specified.
- [ ] Compile-time functions are completely specified.
- [ ] Foreign functions are completely specified.
- [ ] Calling-convention attachment is completely specified.

Integration

- [ ] Every grammar production has an AST mapping.
- [ ] Every AST construct has a semantic mapping.
- [ ] Every semantic construct has an appropriate IR mapping.
- [ ] Function information reaches the compiler correctly.
- [ ] Runtime consumers are identified.
- [ ] Interoperability consumers are identified.
- [ ] Domain-specific consumers are identified.

Scalability

- [ ] No arbitrary function count limit exists.
- [ ] No arbitrary parameter count limit exists.
- [ ] No arbitrary generic count limit exists.
- [ ] No arbitrary constraint count limit exists.
- [ ] No arbitrary capture count limit exists.
- [ ] No arbitrary function-body size is defined by grammar.
- [ ] No hardware size is encoded into function syntax.
- [ ] No fixed topology is encoded.
- [ ] No fixed device count is encoded.

POCO-REAF

- [ ] Function syntax is target-independent.
- [ ] Resource requirements are separated from realization.
- [ ] Capabilities are separated from device selection.
- [ ] Requirements are separated from preferences.
- [ ] Preferences are separated from implementation decisions.
- [ ] Target-specific realization happens downstream.

Quantum

- [ ] Functions integrate with the canonical quantum semantic pipeline.
- [ ] "quantum::ir" remains the canonical quantum IR boundary.
- [ ] Function grammar does not create another quantum IR.
- [ ] No artificial qubit limit exists.
- [ ] No physical-qubit mapping is embedded in ordinary function syntax.
- [ ] QEC remains downstream.
- [ ] ZQN remains downstream.
- [ ] routing remains downstream.
- [ ] scheduling remains downstream.
- [ ] HAL remains downstream.

Safety

- [ ] Rust implementation targets Rust 1.97/1.97.1.
- [ ] Rust 2021 is maintained.
- [ ] No "unsafe" Rust is used.
- [ ] Grammar parsing has no external side effects.
- [ ] Parser does not perform hardware discovery.
- [ ] Parser does not perform network access.
- [ ] Parser does not execute user programs.

Validation

- [ ] Positive tests exist.
- [ ] Negative tests exist.
- [ ] Boundary tests exist.
- [ ] Scalability tests exist.
- [ ] Determinism tests exist.
- [ ] Portability tests exist.
- [ ] Compatibility tests exist.
- [ ] Diagnostics are validated.
- [ ] Hard-coding audit passes.
- [ ] Grammar ambiguity analysis passes.
- [ ] Unreachable-rule analysis passes.
- [ ] Duplicate-production analysis passes.
- [ ] AST coverage passes.
- [ ] semantic coverage passes.
- [ ] IR coverage passes.

---

81. Definition of Done for an Individual File

A function grammar file is DONE only when it can be frozen without requiring a later structural rewrite merely because another function file was subsequently implemented.

For each file:

1. Purpose fixed
2. Ownership fixed
3. Non-ownership fixed
4. Inputs fixed
5. Outputs fixed
6. Dependencies fixed
7. Upstream contracts fixed
8. Downstream consumers identified
9. Public grammar contract fixed
10. AST contract fixed
11. Semantic contract fixed
12. IR contract fixed
13. Compiler integration fixed
14. Runtime integration fixed
15. Cross-domain integration fixed
16. Diagnostics fixed
17. Positive tests fixed
18. Negative tests fixed
19. Boundary tests fixed
20. Scalability tests fixed
21. Determinism tests fixed
22. Compatibility tests fixed
23. Hard-coding audit passed
24. Safety audit passed
25. Completion criteria passed

If another file later needs to reinterpret the meaning of the completed file, that is an architectural failure and must be resolved through the owning contract rather than by silently changing the completed file.

---

82. Recommended Independent Completion Order

The function subsystem should be completed in dependency order.

Stage 1 — function contracts

1. "README.md"
2. "parameters.g4"
3. "returns.g4"
4. "generics.g4"
5. "constraints.g4"
6. "contracts.g4"

Stage 2 — callable forms

7. "closures.g4"
8. "lambdas.g4"
9. "async.g4"
10. "generators.g4"

Stage 3 — interoperability/execution forms

11. "calling-conventions.g4"
12. "foreign-functions.g4"
13. "compile-time-functions.g4"

Stage 4 — composition

14. "functions.g4"

Only after the component contracts are stable should "functions.g4" become the final composition point.

Stage 5 — repository integration

Then validate:

Zamani.g4
    ↓
lexer
    ↓
parser
    ↓
AST
    ↓
semantic analysis
    ↓
IR
    ↓
compiler
    ↓
runtime

This order minimizes re-editing.

---

83. What Must Never Be Added to This Directory

Do not add function grammar rules whose purpose is to encode:

CPU models
GPU models
QPU models
FPGA models
specific devices
physical qubits
physical registers
physical addresses
memory-bank identities
fixed core counts
fixed thread counts
fixed node counts
fixed topology
fixed accelerator counts
vendor-specific ABI implementation
QEC algorithms
routing algorithms
scheduling algorithms
calibration procedures
runtime execution
hardware discovery

unless the feature is explicitly a source-level symbolic interoperability/resource contract and is owned by the appropriate architecture.

---

84. Final Architectural Invariant

The invariant for "grammar/functions/" is:

ONE FUNCTION LANGUAGE
        │
        ├── classical
        ├── quantum
        ├── hybrid
        ├── HDL
        ├── hardware/software co-design
        ├── AI/ML
        ├── distributed
        ├── parallel/HPC
        ├── networking
        ├── cryptography
        ├── scientific
        ├── embedded
        ├── accelerator
        └── future computational paradigms

All of these use the same fundamental function abstraction.

The function grammar describes portable computation.

The semantic layer determines meaning.

The IR layer determines canonical representation.

Optimization determines better realization.

Routing determines physical realization where applicable.

Scheduling determines execution order/resources.

QEC determines quantum error correction.

ZQN determines fault/noise semantics.

HAL determines hardware capability/state interaction.

The compiler determines target lowering.

The runtime determines execution.

Therefore:

FUNCTION SYNTAX
      ↓
PORTABLE SEMANTIC INTENT
      ↓
CANONICAL AST
      ↓
SEMANTIC MODEL
      ↓
CANONICAL / DOMAIN IR
      ↓
OPTIMIZATION
      ↓
TARGET-DEPENDENT REALIZATION
      ↓
RUNTIME

The function grammar must never reverse that direction.

That is the boundary that allows Zamani to pursue:

«Program Once → Compile Once → Run Everywhere, Anywhere, Forever»

while scaling from the smallest computation to arbitrarily large computational systems, subject to the resources actually available rather than artificial limits embedded in the language grammar.