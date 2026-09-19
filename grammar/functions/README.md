Zamani Function Grammar

Production Architecture, Ownership, Integration, and Completion Contract

Path: "grammar/functions/"
Language: Zamani
Grammar technology: ANTLR 4
Rust implementation baseline: Rust 1.97 / Rust 1.97.1
Rust edition: 2021
Safety requirement: "unsafe" Rust is prohibited
Architectural objective: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)
Scalability objective: From the smallest supported computation to arbitrarily large programs and machines, subject only to actual resource availability
Status: Production architecture contract

---

1. Purpose

The "grammar/functions/" directory owns the source-language syntax and composition of functions in Zamani.

A function is one of the fundamental units through which Zamani expresses reusable computation. Function syntax therefore has to remain independent of any particular:

- CPU;
- CPU count;
- CPU core count;
- thread count;
- GPU;
- GPU count;
- FPGA;
- ASIC;
- QPU;
- qubit count;
- memory capacity;
- storage capacity;
- network topology;
- node count;
- accelerator;
- vendor;
- operating system;
- runtime;
- scheduler;
- routing strategy;
- calibration state;
- physical device;
- deployment topology;
- compiler backend.

The function grammar must describe portable program structure and intent, not a machine on which that function happens to execute.

The governing architectural rule is:

«Function syntax describes computation; downstream semantic, compiler, resource, runtime, and hardware systems determine realization.»

This is a prerequisite for POCO-REAF.

---

2. Architectural Objective

The function subsystem participates in the complete Zamani pipeline:

Zamani source
    │
    ▼
canonical lexer
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
    ▼
name / scope / type / generic / effect / capability analysis
    │
    ▼
semantic model
    │
    ├───────────────┬──────────────────┬──────────────────┐
    ▼               ▼                  ▼
classical IR   quantum::ir       HDL/hardware IR
    │               │                  │
    └───────────────┴──────────────────┘
                    │
                    ▼
               optimization
                    │
          ┌─────────┴─────────┐
          ▼                   ▼
       routing            scheduling
          │                   │
          └─────────┬─────────┘
                    ▼
              resilience/QEC
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

"grammar/functions/" participates only in the source-language syntax portion of this pipeline.

It must not become a second semantic model, compiler IR, runtime language, hardware description system, or quantum IR.

---

3. Directory Ownership

This directory owns the syntax for:

- ordinary named functions;
- function declarations;
- function definitions;
- function signatures;
- parameters;
- return clauses;
- generic function parameters;
- function generic constraints;
- closures where specifically delegated here;
- asynchronous function syntax;
- generators;
- compile-time function syntax;
- foreign/external function declarations;
- function contracts;
- function-level modifiers;
- function-level attributes as composition points;
- function-level effect attachment;
- function-level semantic requirement/capability attachment where defined by the canonical grammar.

This directory does not own the meaning of:

- types;
- expressions;
- statements;
- names;
- attributes;
- modules;
- packages;
- effects;
- resources;
- capabilities;
- hardware;
- quantum operations;
- quantum states;
- QEC;
- ZQN;
- routing;
- scheduling;
- optimization;
- HAL;
- runtime execution;
- ABI implementation;
- linking;
- loading;
- deployment;
- target selection;
- physical resource allocation.

Those belong to their respective repository subsystems.

---

4. Actual File Set

The existing function grammar uses the following files and these names should be retained:

grammar/functions/
├── README.md
├── async.g4
├── closures.g4
├── compile-time-functions.g4
├── constraints.g4
├── foreign-functions.g4
├── functions.g4
├── generators.g4
├── generics.g4
├── parameters.g4
├── returns.g4
└── contracts.g4

The README must not introduce a parallel naming system such as:

return-types.g4
calling-conventions.g4

unless those files are actually created and formally promoted into the architecture.

Do not rename the existing files merely for naming consistency.

Existing filenames are retained for compatibility and repository continuity.

---

5. File Ownership Matrix

File| Owns
"functions.g4"| Canonical composition of ordinary named function declarations/definitions
"parameters.g4"| Function parameter syntax
"returns.g4"| Function return syntax
"generics.g4"| Generic parameter declaration syntax
"constraints.g4"| Function-level generic constraint syntax
"contracts.g4"| Function contract syntax
"closures.g4"| Closure/lambda syntax owned by this subsystem
"async.g4"| Asynchronous function syntax
"generators.g4"| Generator syntax
"compile-time-functions.g4"| Compile-time function syntax
"foreign-functions.g4"| Foreign/external function declaration syntax
"README.md"| Architecture, ownership, integration, invariants, and completion criteria

No file may silently become the owner of another file's grammar.

---

6. Single-Authority Rule

There must be exactly one authoritative grammar owner for every function-language production.

For example:

function declaration
    -> functions.g4

parameter list
    -> parameters.g4

return clause
    -> returns.g4

generic parameters
    -> generics.g4

function generic constraints
    -> constraints.g4

function contracts
    -> contracts.g4

async modifier
    -> async.g4

generator syntax
    -> generators.g4

compile-time function syntax
    -> compile-time-functions.g4

foreign function syntax
    -> foreign-functions.g4

The following must not be duplicated inside "functions.g4":

parameterList
genericParameterList
functionConstraintClause
functionContractClause
returnClause
typeExpression
expression
statement
attribute
identifier
qualifiedName

unless the architecture explicitly identifies "functions.g4" as their canonical owner.

---

7. "functions.g4"

7.1 Purpose

"functions.g4" is the canonical composition grammar for ordinary named functions.

It owns:

- function declaration composition;
- function definition composition;
- function signature composition;
- function name attachment;
- function modifier attachment;
- generic attachment;
- parameter attachment;
- return attachment;
- constraint attachment;
- effect attachment;
- contract attachment;
- body/prototype boundary.

It does not own the internals of those components.

---

7.2 Canonical Composition

Conceptually, a function is assembled as:

attributes*
modifiers*
function keyword
function name
generic parameters?
parameter list
return clause?
generic constraints?
effect clause?
contract*
implementation/prototype

For example:

fn add(a: Int, b: Int) -> Int {
    return a + b;
}

or:

async fn compute<T>(value: T) -> Result<T> {
    ...
}

The exact accepted spelling remains determined by the authoritative grammar and language specification.

---

7.3 "functions.g4" must not become a feature catalogue

Do not create:

cpuFunction
gpuFunction
qpuFunction
fpgaFunction
cudaFunction
aiFunction
quantumFunction
hdlFunction
distributedFunction

as separate universal function languages.

The same function syntax must work across domains.

Domain meaning comes from:

- types;
- effects;
- capabilities;
- requirements;
- attributes;
- declarations;
- semantic analysis.

---

8. "parameters.g4"

8.1 Purpose

"parameters.g4" owns function parameter syntax.

It must support the parameter forms defined by the language specification without imposing machine-dependent limits.

Potential semantic parameter categories include:

- ordinary values;
- references;
- resources;
- capabilities;
- quantum values;
- tensors;
- distributed values;
- hardware abstractions;
- future domain values.

---

8.2 Ownership Boundary

"parameters.g4" does not own:

- type semantics;
- type inference;
- ownership;
- borrowing;
- lifetime analysis;
- memory allocation;
- ABI lowering;
- calling convention selection.

For example:

fn compute(value: Tensor<T>) -> Tensor<T>

gets its parameter structure from "parameters.g4", but "Tensor<T>" belongs to the canonical type system.

---

8.3 Scalability

There must be no grammar-level constants such as:

MAX_PARAMETERS
MAX_ARGUMENTS
MAX_GENERIC_PARAMETERS

Parameter lists use grammar repetition.

A function with:

0
1
2
...
many

parameters must be represented by the same language mechanism.

Actual implementation limits are resource-policy concerns, not language semantics.

---

9. "returns.g4"

9.1 Purpose

"returns.g4" owns function return syntax.

It defines the syntax attaching a result type or result structure to a function.

Examples include:

fn compute() -> Int
fn transform<T>(value: T) -> T
fn measure(q: Qubit) -> Measurement

The return type itself belongs to the canonical type system.

---

9.2 No Type Duplication

"returns.g4" must not enumerate:

Int
Float
Tensor
Qubit
Measurement
...

as return-specific grammar alternatives.

New types must become usable as function return types without changing this file.

---

10. "generics.g4"

10.1 Purpose

"generics.g4" owns the syntax for generic function parameters.

Examples:

<T>
<T, U>
<T: Numeric>
<T, U: Comparable>

The exact syntax is determined by the canonical generic/type specification.

---

10.2 Semantic Boundary

This file does not own:

- type inference;
- generic substitution;
- trait solving;
- specialization;
- monomorphization;
- associated-type resolution;
- capability resolution;
- constraint satisfiability.

The pipeline is:

generic syntax
    ↓
AST
    ↓
generic semantic model
    ↓
type/trait/capability solving
    ↓
compiler specialization/lowering

---

10.3 Unbounded Generic Arity

Never implement generics as:

generic1
generic2
generic3
...

Use repetition.

There must be no language-level maximum on generic parameter count.

---

11. "constraints.g4"

11.1 Purpose

"constraints.g4" owns function-level generic constraint syntax.

It connects a generic parameter to a canonical constraint/type-bound construct.

For example:

where T: Numeric

The grammar does not decide whether "T" was actually declared.

That is semantic analysis.

---

11.2 Important Separation

A function generic constraint is not automatically the same thing as:

- a resource requirement;
- a hardware requirement;
- a capability;
- an effect;
- a runtime condition.

The semantic system determines the distinction.

Do not allow this file to become a second general-purpose constraint language.

---

12. "contracts.g4"

12.1 Purpose

"contracts.g4" owns source-level function contract syntax such as:

contract {
    requires(...);
    ensures(...);
    invariant(...);
}

It does not own contract semantics.

---

12.2 Semantic Boundary

The semantic subsystem determines:

- identifier validity;
- type correctness;
- scope;
- contract consistency;
- satisfiability;
- verification;
- runtime checkability;
- compile-time checkability;
- effects of contract expressions.

Parsing successfully does not mean a contract is valid.

---

12.3 No Contract IR

Do not create a second universal IR merely for contracts.

Contracts become semantic metadata and obligations consumed by appropriate verification/compiler/runtime stages.

They must not create:

ContractIR
QuantumContractIR
HardwareContractIR

unless a future architecture explicitly establishes a canonical IR for such semantics.

---

13. "closures.g4"

13.1 Purpose

"closures.g4" owns closure/lambda syntax where closures are part of the function grammar subsystem.

Closures must remain independent of:

- stack layout;
- heap layout;
- register allocation;
- thread implementation;
- executor;
- CPU;
- GPU;
- QPU;
- FPGA;
- runtime scheduling.

A closure is a language construct.

Its representation is a compiler decision.

---

14. "async.g4"

14.1 Purpose

"async.g4" owns source-level asynchronous function syntax.

For example:

async fn compute(...) -> Result<T> {
    ...
}

The grammar expresses asynchronous semantics.

It does not select:

- executor;
- event loop;
- scheduler;
- worker count;
- thread count;
- CPU;
- GPU;
- accelerator;
- node;
- task placement.

---

14.2 POCO-REAF Requirement

The following must remain conceptually distinct:

async

and:

run on 8 threads

The first is language semantics.

The second is an implementation/deployment decision.

If Zamani eventually permits resource preferences, those belong to the resource/capability/deployment architecture.

---

15. "generators.g4"

15.1 Purpose

"generators.g4" owns generator syntax.

Generators may represent:

- lazy computation;
- iteration;
- streaming;
- resumable computation;
- producer semantics.

The grammar must not encode:

- queue capacity;
- buffer size;
- worker count;
- thread count;
- memory capacity;
- machine topology.

Those are downstream resource/runtime concerns.

---

16. "compile-time-functions.g4"

16.1 Purpose

This file owns source syntax for compile-time functions where Zamani exposes such a construct.

Compile-time functions may support:

- compile-time evaluation;
- compile-time generated values;
- compile-time metadata;
- type-level computation where specified;
- specialization inputs;
- generated declarations.

---

16.2 Security Boundary

Compile-time functions must not automatically receive unrestricted access to:

- filesystem;
- network;
- environment;
- secrets;
- credentials;
- hardware;
- external processes.

Capability/effect/security systems determine which operations are permitted.

The grammar describes syntax; it does not grant authority.

---

17. "foreign-functions.g4"

17.1 Purpose

"foreign-functions.g4" owns source-level declarations for externally implemented functions.

A foreign function declaration is an interface contract, not an implementation.

It may expose metadata for:

- external language;
- external symbol;
- linkage;
- representation;
- ABI;
- parameters;
- return types;
- effects;
- capabilities;
- requirements;
- attributes.

---

17.2 Open-World Foreign Interoperability

Do not turn the grammar into a fixed list such as:

C
C++
Rust
Python
Fortran
CUDA
OpenCL
...

as hard-coded parser alternatives.

A new interoperable language must not require rewriting the fundamental function grammar merely because the ecosystem expands.

Interoperability metadata belongs to the interoperability subsystem.

---

18. Effects Integration

Functions may attach effects through the canonical effects grammar.

The function subsystem may compose an effect clause, but does not own effect semantics.

Effects may eventually describe operations involving:

- I/O;
- networking;
- quantum computation;
- hardware interaction;
- distributed communication;
- security;
- persistence;
- future domains.

Do not hard-code every possible future effect into "functions.g4".

---

19. Capability and Resource Integration

Function syntax may participate in the canonical capability/resource model.

The critical distinction is:

semantic requirement
        ≠
implementation decision

For example:

requires capability("quantum.measurement")

can express a portable requirement.

It must not mean:

use QPU 0

Likewise:

requires resource(...)

must not implicitly select:

- a physical device;
- a physical qubit;
- a specific GPU;
- a CPU core;
- a memory bank;
- a network node.

Resource realization belongs downstream.

---

20. Classical Computing Integration

Functions must be domain-neutral enough to support:

- scalar computation;
- integer computation;
- floating point;
- vectors;
- matrices;
- tensors;
- symbolic mathematics;
- numerical methods;
- statistics;
- signal processing;
- scientific computing;
- optimization;
- data processing.

No changes to function grammar should be necessary merely because a new classical type or library is introduced.

---

21. Quantum Integration

Quantum functions must use ordinary function mechanisms.

For example:

fn prepare(q: Qubit) -> Qubit

or:

fn measure(q: Qubit) -> Measurement

The function grammar does not own:

- "Qubit";
- "Measurement";
- quantum gates;
- quantum states;
- circuit semantics;
- physical qubits;
- coupling maps;
- calibration;
- routing;
- scheduling;
- QEC;
- ZQN;
- HAL.

Those belong to their respective subsystems.

---

21.1 Canonical Quantum Boundary

Quantum lowering remains:

function syntax
    ↓
domain-neutral AST
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
QEC/resilience
    ↓
ZQN
    ↓
HAL
    ↓
target realization

"quantum::ir" remains the canonical quantum semantic boundary.

"grammar/functions/" must never introduce another quantum IR.

---

22. Hybrid Quantum-Classical Integration

A function may combine classical and quantum computation:

classical input
    ↓
quantum operation
    ↓
measurement
    ↓
classical computation
    ↓
quantum operation

The function grammar does not need separate:

quantumFunction
hybridFunction
classicalFunction

constructs.

The existing generic function model is the integration point.

---

23. HDL and Hardware/Software Co-Design

Functions must be usable in programs involving:

- hardware/software co-design;
- accelerators;
- FPGA computation;
- ASIC-oriented computation;
- hardware interfaces;
- embedded systems;
- HDL constructs.

However, function syntax must not encode universal machine limits such as:

MAX_REGISTER_WIDTH
MAX_PORTS
MAX_PIPELINE_DEPTH
MAX_FPGA_RESOURCES
MAX_ACCELERATORS

Hardware intent belongs to:

grammar/hdl/
grammar/hardware/
grammar/resources/

and their semantic/compiler consumers.

---

24. Distributed and Parallel Integration

Functions must be capable of representing computations eventually executed across:

- one processor;
- many processors;
- many machines;
- clusters;
- cloud systems;
- edge systems;
- distributed accelerators;
- future computational substrates.

No function grammar rule may require:

node0
node1
node2

or impose a maximum node count.

Placement and deployment belong downstream.

---

25. AI/ML and Data Integration

Functions must naturally accept and return:

- tensors;
- models;
- datasets;
- streams;
- symbolic values;
- probabilistic values;
- AI agents;
- accelerator abstractions.

For example:

fn infer(model: Model, input: Tensor) -> Tensor

must not need to know whether realization occurs on:

- CPU;
- GPU;
- accelerator;
- FPGA;
- distributed cluster;
- future hardware.

Framework-specific syntax does not belong in the universal function grammar.

---

26. Security Integration

Function declarations may participate in security semantics through:

- capabilities;
- effects;
- contracts;
- attributes;
- policies;
- foreign interfaces.

The function grammar must not itself execute security checks or access secrets.

Security semantics belong to the security and semantic-analysis subsystems.

---

27. Memory and Ownership Integration

Function parameters and results must remain compatible with the canonical memory/type system.

The function grammar must not independently define:

- ownership;
- borrowing;
- lifetimes;
- allocation;
- memory regions;
- address spaces.

Those systems consume function AST information downstream.

This avoids two competing ownership models.

---

28. Function Body Integration

Function bodies must use the canonical block/statement/expression grammar.

The function grammar should establish only the boundary:

function declaration
    ->
function signature
    ->
function body OR declaration/prototype terminator

It must not recreate:

expression
statement
block

inside "functions.g4".

---

29. Attributes

Attributes are owned by the canonical attribute grammar.

Functions may permit attributes to attach to the function declaration.

The function grammar does not need to understand every possible attribute.

This is essential for future extensibility.

A new domain should be able to add an attribute without forcing every core function grammar rule to change.

Semantic analysis determines whether a particular attribute is valid on a function.

---

30. Modifiers

Core function modifiers may be defined where the language specification requires fixed syntax.

However, the modifier system must not become a catalogue of:

- vendors;
- accelerators;
- devices;
- hardware models;
- operating systems;
- deployment environments.

Open-ended metadata belongs to attributes/extensions.

---

31. AST Contract

Every function grammar construct must have a predetermined mapping into the existing domain-neutral frontend AST.

At minimum, the AST must be capable of preserving:

- complete source span;
- function name;
- attributes;
- modifiers;
- generic parameters;
- generic bounds;
- parameter ordering;
- parameter modifiers;
- parameter patterns;
- parameter types;
- defaults where supported;
- variadic state where supported;
- return type;
- effects;
- requirements;
- capabilities;
- contracts;
- body;
- declaration/prototype state;
- foreign metadata where applicable;
- compile-time metadata where applicable;
- source provenance.

The grammar must not introduce backend-specific AST types such as:

QuantumFunction
CpuFunction
GpuFunction
QpuFunction
FpgaFunction
HardwareFunction

unless the domain-neutral AST architecture explicitly establishes such a concept.

The established architectural preference is a generic AST with domain semantics represented through canonical types, operations, attributes, effects, capabilities, and semantic models.

---

32. AST → Semantic Model Contract

The parser must preserve enough information for semantic analysis to determine:

- declaration identity;
- scope;
- name resolution;
- overload resolution where supported;
- generic validity;
- type validity;
- parameter legality;
- return legality;
- effect legality;
- capability legality;
- resource requirements;
- contract legality;
- async legality;
- generator legality;
- compile-time restrictions;
- foreign declaration legality;
- ownership;
- borrowing;
- lifetimes;
- recursion;
- determinism;
- domain legality.

The grammar must not perform these semantic decisions.

---

33. AST → IR Contract

The function grammar does not directly lower to a domain IR.

The correct path is:

function grammar
    ↓
parse tree
    ↓
domain-neutral AST
    ↓
semantic model
    ↓
canonical domain IR

Possible downstream destinations include:

classical IR
quantum::ir
HDL/hardware IR
future domain IR

The function grammar itself must not create a universal function IR.

---

34. Quantum IR Contract

A quantum function is lowered into "quantum::ir" only after semantic analysis.

The function grammar must not know:

- physical qubit IDs;
- routing;
- gate decomposition;
- coupling topology;
- calibration;
- pulse realization;
- scheduling;
- QEC implementation;
- ZQN semantics;
- HAL state.

Those remain separate architectural responsibilities.

---

35. Compiler Integration

The compiler may consume function information for:

- name resolution;
- type checking;
- generic instantiation;
- specialization;
- optimization;
- effect analysis;
- capability analysis;
- resource analysis;
- contract verification;
- lowering;
- code generation;
- provenance;
- deterministic compilation.

The grammar must not perform compiler work.

---

36. Runtime Integration

The runtime may consume lowered function information for:

- invocation;
- scheduling;
- asynchronous execution;
- resource acquisition;
- distributed placement;
- checkpointing;
- resilience;
- observability;
- tracing;
- recovery.

The grammar must not depend on runtime state.

Parsing must remain valid even if:

- no target exists yet;
- no hardware is currently available;
- no GPU is installed;
- no QPU is available;
- the runtime is offline.

Target realization occurs later.

---

37. Tooling Integration

Tooling must consume the canonical parser/AST representation rather than reimplementing function parsing.

The grammar must therefore preserve sufficient source spans and structure for:

- IDE navigation;
- syntax highlighting;
- diagnostics;
- documentation generation;
- code completion;
- refactoring;
- formatting;
- semantic inspection;
- contract visualization;
- generic inspection;
- call hierarchy;
- cross-domain tooling.

---

38. Error and Diagnostic Contract

Syntax errors belong to parsing.

Semantic errors belong to semantic analysis.

Examples of parser-level errors:

missing function name
missing parameter delimiter
malformed generic parameter syntax
malformed return clause
malformed constraint syntax
malformed contract
missing function body where required
unexpected token
missing delimiter

Examples of semantic errors:

unknown function
duplicate function declaration
unknown generic parameter
invalid type
invalid effect
unsatisfied capability
unsatisfied resource requirement
invalid contract
invalid ownership
invalid lifetime
illegal async operation
illegal generator operation
invalid foreign declaration

The grammar must not encode semantic diagnostics as grammar alternatives.

---

39. Error Recovery

The grammar must remain structured enough for ANTLR's recovery mechanisms to identify meaningful synchronization points.

Good recovery boundaries include:

- function declarations;
- parameter lists;
- generic parameter lists;
- return clauses;
- constraint clauses;
- contract blocks;
- function bodies.

No embedded Rust parser actions are permitted merely to implement recovery.

---

40. Determinism

Given the same:

- source text;
- canonical lexer version;
- grammar version;
- parser configuration;

the function grammar must produce deterministic parser structure.

Parsing must never depend on:

- current CPU;
- GPU availability;
- QPU availability;
- hardware topology;
- runtime state;
- network state;
- calibration;
- scheduler state;
- random numbers;
- external files;
- environment variables.

---

41. Security

The function grammar must contain:

- no embedded Rust execution;
- no "unsafe";
- no filesystem access;
- no network access;
- no process spawning;
- no shell execution;
- no hardware discovery;
- no secret access;
- no environment-dependent semantic decisions;
- no arbitrary code execution.

The generated Rust parser must be usable under the repository's Rust 1.97 / 1.97.1 baseline without requiring "unsafe" application code.

---

42. POCO-REAF Contract

The function grammar must not impose artificial limits on:

- number of functions;
- number of parameters;
- number of generic parameters;
- number of constraints;
- number of attributes;
- number of effects;
- number of contracts;
- function nesting;
- source-program size;
- quantum objects;
- classical objects;
- distributed objects;
- resources;
- capabilities.

This does not mean an implementation has infinite physical memory or infinite compilation time.

It means:

«The language must not invent artificial finite limits where the computation and implementation can naturally scale with available resources.»

Therefore:

tiny program
    ↓
same language semantics
    ↓
larger program
    ↓
same language semantics
    ↓
larger machine
    ↓
same program
    ↓
different machine
    ↓
same source semantics

is the intended model.

---

43. Hard-Coding Prohibition

The function grammar must not contain universal constants such as:

MAX_FUNCTIONS
MAX_PARAMETERS
MAX_GENERIC_PARAMETERS
MAX_THREADS
MAX_CORES
MAX_GPUS
MAX_FPGAS
MAX_QPUS
MAX_QUBITS
MAX_MEMORY
MAX_NODES
MAX_DEVICES
MAX_REGISTER_WIDTH

It must also not encode universal machine identities such as:

cpu0
gpu0
qpu0
fpga0
device0
physical_qubit0

as intrinsic function-language constructs.

If a specific implementation has a parser/compiler/runtime resource limit, that limit must be represented outside the language grammar as an implementation/resource policy.

---

44. Semantic Requirement vs Implementation Decision

The function subsystem must preserve the distinction:

WHAT THE PROGRAM REQUIRES
        vs.
HOW THE TARGET SATISFIES IT

Examples:

requires capability("quantum.measurement")

is semantic intent.

use qpu7

is target realization.

Likewise:

requires parallel execution

is semantic intent.

use exactly 64 CPU threads

is implementation/deployment intent.

The first category may belong in the portable language architecture.

The second belongs downstream unless explicitly expressed as an intentional deployment constraint.

---

45. Cross-Domain Integration Rule

A function must remain usable across:

classical
quantum
hybrid
HDL
hardware
distributed
parallel/HPC
AI/ML
data
networking
security
embedded
edge
cloud
scientific computing
accelerators
future computational domains

without creating a separate function language for each domain.

The function layer is a universal composition mechanism.

Domain-specific semantics are attached through canonical domain systems.

---

46. Future-Domain Rule

A future computational domain must be able to use functions without requiring modifications to the fundamental function model merely because the domain is new.

For example, a future domain should be able to define a type:

future::Value

and then use:

fn compute(value: future::Value) -> future::Value

without modifying:

functions.g4
parameters.g4
returns.g4

unless the future domain genuinely introduces new function syntax.

This is a key requirement for long-term POCO-REAF.

---

47. No Domain-Specific Function Duplication

Do not create separate copies such as:

quantum/functions.g4
classical/functions.g4
hdl/functions.g4
ai/functions.g4
gpu/functions.g4
qpu/functions.g4

merely to represent functions in those domains.

If a domain genuinely requires special syntax, that syntax must be narrowly scoped and integrated with the canonical function architecture rather than creating a second function model.

---

48. Generic Function Model

The function model must remain open to generic values from any supported domain.

Conceptually:

fn transform<T>(value: T) -> T

may operate on:

classical value
quantum value
tensor
distributed value
hardware abstraction
data structure
future-domain value

depending on semantic constraints.

The grammar must not decide which domain wins.

---

49. Compile-Time / Runtime Separation

A compile-time function is not automatically a runtime function.

A runtime function is not automatically compile-time.

The semantic/compiler layers determine:

- evaluation phase;
- allowed effects;
- capabilities;
- determinism;
- available resources;
- generated artifacts;
- specialization;
- caching;
- reproducibility.

The grammar only describes the source construct.

---

50. Foreign / Native Boundary

A foreign function declaration must remain declarative.

The grammar must not:

- invoke the foreign compiler;
- load a library;
- resolve a symbol;
- inspect the host machine;
- select an ABI automatically;
- access the network;
- perform linking.

Those are compiler/toolchain/runtime responsibilities.

---

51. Calling Convention Boundary

Calling-convention syntax must not be invented inside ordinary function grammar merely to expose backend details.

If Zamani later standardizes portable calling-convention metadata, it must be introduced through the appropriate interoperability/ABI specification and integrated through canonical attributes or explicitly owned grammar.

The function grammar must not become a backend ABI catalogue.

---

52. Compatibility

The existing function filenames are retained.

The compatibility subsystem owns:

- language versions;
- feature gates;
- deprecation;
- migration;
- compatibility matrices;
- removal policy.

Relevant integration points include:

grammar/compatibility/
grammar/specification/
grammar/spec/
grammar/Zamani.g4
grammar/lexer/
src/lexer.rs
src/parser.rs
src/frontend/ast/

A syntax change is not complete until its impact on these consumers has been considered.

---

53. Specification Integration

The function grammar must trace back to the authoritative specification.

At minimum, function semantics must be represented consistently across:

grammar/specification/
grammar/spec/
grammar/functions/
grammar/Zamani.g4

No design document may silently introduce syntax that is absent from the authoritative grammar.

Likewise, the implementation grammar must not silently introduce semantics that are absent from the specification.

---

54. "Zamani-Grammar.md" Integration

"Zamani-Grammar.md" may contain broader or historical language designs.

It is not permitted to silently become a second authority for function syntax.

Any proposed function feature from that document must follow:

proposal
    ↓
language specification
    ↓
AST contract
    ↓
semantic contract
    ↓
canonical grammar
    ↓
tests
    ↓
implementation

Only then is it accepted language syntax.

---

55. "grammar.md" Integration

"grammar/grammar.md" describes implementation conformance.

It must reflect the actual implemented parser.

The function subsystem therefore needs traceability between:

functions/*.g4
    ↓
Zamani.g4
    ↓
generated parser
    ↓
src/parser.rs / frontend
    ↓
grammar/grammar.md

If "grammar.md" claims syntax that the parser does not accept, that is an implementation-conformance defect.

If the parser accepts undocumented syntax, that is a specification/conformance defect requiring classification.

---

56. "Zamani.g4" Integration

"Zamani.g4" remains the composition root.

It must integrate the function subsystem rather than reproduce its rules.

Conceptually:

Zamani.g4
    ↓
function declaration entry point
    ↓
functions.g4
    ├── parameters.g4
    ├── returns.g4
    ├── generics.g4
    ├── constraints.g4
    ├── contracts.g4
    ├── async.g4
    ├── generators.g4
    ├── closures.g4
    ├── compile-time-functions.g4
    └── foreign-functions.g4

The exact ANTLR composition mechanism must follow the repository's actual parser architecture.

No duplicate function productions should survive in "Zamani.g4".

---

57. Lexer Integration

The function grammars consume the canonical lexer vocabulary.

They must not define competing lexer tokens.

Keyword spelling belongs to the canonical lexer authority.

For example, tokens representing:

fn
async
where
return

must come from the canonical lexical system.

The function parser must not create a private vocabulary.

---

58. AST Integration with "src/frontend/ast/"

The function grammar must map into the existing domain-neutral AST architecture.

The AST must remain independent of:

- LLVM;
- QIR;
- MLIR;
- vendor APIs;
- QEC implementation;
- routing;
- physical qubit maps;
- calibration;
- target hardware.

A function is represented as a language-level construct first.

Domain-specific lowering occurs later.

---

59. Integration with Quantum Frontend

Quantum source formats such as OpenQASM remain format frontends rather than replacements for Zamani function syntax.

A Zamani function containing quantum computation ultimately follows the canonical quantum path.

The function grammar must not import or depend directly on a particular OpenQASM implementation.

---

60. Integration with QEC, ZQN, Routing, Scheduling, HAL

The separation must remain:

Function grammar
    =
source function structure

semantic analysis
    =
meaning and legality

quantum::ir
    =
canonical quantum semantic boundary

optimization
    =
implementation improvement

routing
    =
physical realization

scheduling
    =
time/order/resource scheduling

QEC
    =
error detection/correction

ZQN
    =
fault/noise semantics

HAL
    =
hardware capability/state boundary

No function grammar file may duplicate those responsibilities.

---

61. Integration with Resources

A function can participate in resource requirements, but the grammar does not allocate resources.

The downstream resource system determines:

- availability;
- capability matching;
- capacity;
- placement;
- negotiation;
- scaling;
- deployment.

This enables the same source function to be realized on different machines according to available resources.

---

62. Integration with Concurrency

"async", generators, parallelism, actors, tasks, and other concurrency constructs must remain semantically distinct from machine resource counts.

For example:

async fn process(...)

does not imply:

N threads

The concurrency subsystem determines the semantics.

The runtime decides realization.

---

63. Integration with Modules

Functions are declarations within the canonical module/name-resolution system.

The function grammar must not independently implement:

- module resolution;
- imports;
- exports;
- package lookup;
- dependency resolution.

Those belong to "grammar/modules/" and compiler/module infrastructure.

---

64. Integration with Effects

Function effects must be represented through the canonical effect system.

The function grammar only attaches effect syntax.

Effect semantics remain outside this directory.

This allows new effects to be added without rewriting the fundamental function grammar.

---

65. Integration with Memory

Parameter and return syntax must remain compatible with:

grammar/memory/
grammar/types/

The function grammar must not invent another ownership or lifetime model.

---

66. Integration with Interoperability

Foreign functions integrate with:

grammar/interoperability/

rather than embedding every foreign language and ABI directly into "functions/".

The integration boundary must preserve:

- source symbol;
- external language metadata;
- representation metadata;
- ABI metadata;
- effect metadata;
- capability requirements;
- provenance.

---

67. Integration with Dialects

A dialect may extend function syntax only through the canonical dialect-extension mechanism.

A dialect must not silently fork the function grammar.

A dialect extension must identify:

- dialect name;
- version;
- syntax extension;
- AST mapping;
- semantic mapping;
- compatibility;
- feature gate;
- downstream consumers.

---

68. Integration with Macros and Metaprogramming

Macros may generate functions.

Metaprogramming may inspect function structure where the language permits it.

Neither system may bypass:

lexing
parsing
AST construction
structural validation
semantic validation

Generated functions must therefore enter the same canonical pipeline as handwritten functions.

---

69. Source Provenance

Function AST nodes must preserve source provenance sufficiently for:

- diagnostics;
- IDE tooling;
- generated-code tracing;
- macro expansion;
- compile-time generation;
- reproducibility;
- verification;
- debugging.

Source spans must not be discarded merely because the function is later lowered.

---

70. Deterministic Ordering

Where function syntax contains ordered collections, source order must be preserved unless a later semantic phase explicitly defines canonicalization.

Examples:

- parameter order;
- generic parameter order;
- attributes;
- constraints;
- effects;
- contracts.

The grammar must not silently reorder these.

---

71. No Semantic Reordering in Grammar

The parser must not decide that:

T: A + B

is equivalent to:

T: B + A

or that:

effect A, B

is equivalent to:

effect B, A

unless the language specification explicitly defines that equivalence.

Parsing preserves structure.

Semantic analysis determines meaning.

---

72. Scalability Model

The intended scalability model is:

small source
    ↓
same grammar
    ↓
larger source
    ↓
same grammar
    ↓
larger computational resource
    ↓
same program semantics

The grammar must not use artificial finite alternatives to represent scalable concepts.

Prefer:

item*

or:

item (separator item)*

over finite enumerations.

---

73. "Infinity" Interpretation

Zamani's scalability objective does not claim that physical computers possess infinite resources.

It means:

«No artificial language-level ceiling should prevent a valid computation from scaling when the implementation and target environment have sufficient resources.»

Therefore the relevant limiting factors are actual:

- memory;
- compute;
- storage;
- compilation resources;
- runtime resources;
- device capacity;
- network capacity;
- physical constraints.

The grammar must not invent additional limits.

---

74. Performance and Resource Safety

Grammar scalability must not be confused with unlimited parser resource consumption.

Implementations may use resource policies to prevent:

- denial-of-service inputs;
- pathological nesting;
- compiler exhaustion;
- memory exhaustion;
- excessive diagnostic generation.

Such limits must be implementation/security policies and must not alter the language's conceptual semantics.

Where a parser implementation imposes a limit, it must be:

1. documented;
2. configurable where appropriate;
3. diagnosable;
4. tested;
5. distinguished from a language limitation.

---

75. Rust 1.97 / 1.97.1 Requirement

The grammar subsystem's Rust consumers must remain compatible with:

Rust 1.97
Rust 1.97.1
Edition 2021

The grammar files themselves must not depend on Rust implementation details.

Generated/parser integration must not require "unsafe".

No function grammar feature is complete if its implementation requires unsafe Rust.

---

76. No Embedded Actions

Function grammars should remain declarative ANTLR grammar wherever possible.

Do not embed Rust code into the grammar to perform:

- semantic analysis;
- hardware discovery;
- resource allocation;
- name lookup;
- type inference;
- optimization;
- runtime execution;
- external process execution.

This keeps the grammar deterministic and portable.

---

77. Testing Contract

Function grammar testing must cover at least:

grammar/functions/
├── positive
├── negative
├── boundary
├── scalability
├── determinism
├── compatibility
├── cross-domain
└── round-trip

Tests should integrate with the repository's existing grammar-test infrastructure rather than creating a competing test runner.

---

78. Positive Tests

Positive coverage must include representative forms of:

- ordinary functions;
- empty parameter lists;
- multiple parameters;
- return values;
- generic functions;
- constraints;
- attributes;
- modifiers;
- effects;
- contracts;
- closures;
- async functions;
- generators;
- compile-time functions;
- foreign functions;
- classical functions;
- quantum functions;
- hybrid functions;
- HDL/hardware-facing functions;
- distributed functions;
- AI/data functions.

---

79. Negative Tests

Negative tests must verify rejection of malformed syntax, including cases such as:

missing function name
missing parameter delimiter
missing closing parenthesis
malformed generic list
malformed constraint
malformed return clause
malformed contract
invalid function terminator
malformed async declaration
malformed generator syntax
malformed foreign declaration

Negative tests must not confuse semantic invalidity with syntactic invalidity.

---

80. Boundary Tests

Boundary tests must cover:

- zero parameters;
- one parameter;
- many parameters;
- zero generic parameters;
- many generic parameters;
- nested generic types;
- many constraints;
- many attributes;
- many effects;
- many contracts;
- large function bodies;
- deeply nested expressions;
- deeply nested blocks;
- long identifiers;
- long qualified names;
- Unicode identifiers where supported;
- cross-domain types.

---

81. Scalability Tests

Scalability tests must verify that no artificial grammar ceiling exists for:

- parameter count;
- generic parameter count;
- constraints;
- attributes;
- contracts;
- functions per module;
- source size;
- nested generic structure;
- nested function bodies where permitted.

Tests should scale according to available test resources rather than encoding a false universal maximum.

---

82. Cross-Domain Tests

Function tests must include combinations such as:

classical parameter + classical return
quantum parameter + quantum return
classical parameter + quantum return
quantum parameter + classical return
hybrid computation
tensor parameter
hardware abstraction parameter
distributed value parameter
AI model parameter
data pipeline value
security-sensitive capability

The goal is to prove that the function grammar remains domain-neutral.

---

83. Determinism Tests

Repeated parsing of identical source must produce equivalent:

tokens
parse tree
AST structure
source spans

where the parser configuration is unchanged.

No test should require hardware availability.

---

84. Round-Trip Tests

Where the repository provides formatting/serialization support:

source
  ↓
lexer
  ↓
parser
  ↓
AST
  ↓
formatter/serializer
  ↓
parser

must preserve semantic structure.

Round-trip tests must not require byte-for-byte equality when formatting intentionally canonicalizes whitespace or formatting.

---

85. Compatibility Tests

When function syntax changes, compatibility tests must verify:

- previously valid stable syntax;
- intentionally deprecated syntax;
- feature-gated syntax;
- migration behavior;
- parser behavior;
- AST behavior;
- semantic behavior.

No compatibility change is complete merely because the grammar compiles.

---

86. Feature-Level Closure

Every function feature must have a complete contract before being considered finished.

For example, adding async syntax is not complete merely because "async.g4" parses.

Completion requires:

syntax
  ↓
lexer contract
  ↓
AST contract
  ↓
semantic contract
  ↓
effect/concurrency contract
  ↓
compiler contract
  ↓
runtime contract
  ↓
tooling contract
  ↓
tests
  ↓
compatibility
  ↓
hard-coding audit

This is the required independently completable file/feature model.

---

87. Per-File Completion Contract

Every ".g4" file in this directory must be independently documented against:

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
Diagnostics
Security
Performance
Hard-Coding Audit
Completion Criteria

The file must be designed so that another subsystem can integrate it without requiring its owner to redesign the file afterward.

---

88. Dependency Direction

The intended dependency direction is:

lexer
  ↓
core
  ↓
types / expressions / statements
  ↓
functions
  ↓
frontend AST
  ↓
semantic analysis
  ↓
canonical IR
  ↓
compiler
  ↓
runtime

Function grammar must not introduce reverse dependencies such as:

functions → runtime
functions → HAL
functions → quantum::ir
functions → scheduler
functions → routing
functions → QEC
functions → ZQN

The function subsystem may be consumed by those systems through AST/semantic/IR contracts.

It must not depend on their implementation.

---

89. Repository Integration Matrix

System| Function grammar relationship
"grammar/Zamani.g4"| Composition root consumes function grammar
"grammar/lexer/"| Supplies canonical tokens
"grammar/core/"| Supplies names, attributes, blocks, common syntax
"grammar/types/"| Supplies type syntax
"grammar/expressions/"| Supplies expression syntax
"grammar/statements/"| Supplies statements and bodies
"grammar/effects/"| Supplies effect syntax
"grammar/memory/"| Supplies memory/type semantics downstream
"grammar/concurrency/"| Supplies broader concurrency semantics
"grammar/resources/"| Supplies resource/capability concepts
"grammar/hardware/"| Supplies hardware intent
"grammar/classical/"| Supplies classical domain semantics
"grammar/quantum/"| Supplies quantum syntax/semantics
"grammar/hybrid/"| Supplies hybrid semantics
"grammar/hdl/"| Supplies HDL syntax
"grammar/distributed/"| Supplies distributed semantics
"grammar/ai/"| Supplies AI semantics
"grammar/data/"| Supplies data semantics
"grammar/networking/"| Supplies networking semantics
"grammar/security/"| Supplies security semantics
"grammar/compile/"| Supplies compilation/deployment intent
"grammar/execution/"| Supplies execution intent
"grammar/interoperability/"| Supplies FFI/ABI integration
"grammar/dialects/"| Supplies controlled extensions
"grammar/macros/"| Generates/consumes function syntax
"grammar/metaprogramming/"| Inspects/generates function structures
"grammar/validation/"| Validates grammar correctness
"grammar/compatibility/"| Owns language compatibility
"grammar/tests/"| Provides conformance tests
"src/frontend/ast/"| Canonical domain-neutral AST
"src/parser.rs"| Parser implementation consumer
"src/lexer.rs"| Lexer implementation consumer
"quantum::ir"| Downstream quantum semantic boundary; never grammar dependency
compiler| Downstream semantic/lowering consumer
runtime| Downstream execution consumer
HAL| Downstream hardware boundary

---

90. What Must Never Be Added Here

Do not add:

MAX_QUBITS
MAX_CPUS
MAX_CORES
MAX_THREADS
MAX_GPUS
MAX_FPGAS
MAX_QPUS
MAX_NODES
MAX_MEMORY
MAX_TENSOR_SIZE
MAX_REGISTER_WIDTH
MAX_DEVICES

Do not add universal:

cpuFunction
gpuFunction
qpuFunction
fpgaFunction
vendorFunction
cudaFunction

Do not add:

physicalQubitFunction
physicalDeviceFunction
hardwareAddressFunction

Do not implement:

routing
scheduling
QEC
ZQN
HAL
calibration
optimization
resource allocation

inside function grammar.

Do not introduce a second function AST.

Do not introduce a second quantum IR.

Do not duplicate lexer rules.

Do not duplicate type grammar.

Do not duplicate expression grammar.

Do not duplicate statement grammar.

Do not allow a historical/design document to silently become executable syntax.

---

91. Production Acceptance Criteria

"grammar/functions/" is production-ready only when all of the following are true.

Architecture

- [ ] "functions.g4" is the canonical function composition grammar.
- [ ] Existing filenames remain stable unless a separately approved migration requires otherwise.
- [ ] Every grammar rule has exactly one authoritative owner.
- [ ] No duplicate function language exists elsewhere.
- [ ] "Zamani.g4" composes the function subsystem.
- [ ] The function subsystem does not become a second root grammar.

Lexer

- [ ] All tokens originate from the canonical lexer.
- [ ] No duplicate lexer vocabulary exists.
- [ ] Keyword ownership is unambiguous.

AST

- [ ] Every function construct has an AST mapping.
- [ ] Source spans are preserved.
- [ ] Function ordering information is preserved.
- [ ] Generic information is preserved.
- [ ] Parameter information is preserved.
- [ ] Return information is preserved.
- [ ] Effect/contract/requirement metadata is preserved.
- [ ] No backend-specific function AST is introduced.

Semantics

- [ ] Name resolution is downstream.
- [ ] Type checking is downstream.
- [ ] Generic solving is downstream.
- [ ] Effect checking is downstream.
- [ ] Capability checking is downstream.
- [ ] Resource analysis is downstream.
- [ ] Ownership/borrowing is downstream.
- [ ] Domain legality is downstream.

IR

- [ ] Function grammar does not define an IR.
- [ ] Classical lowering uses the canonical classical IR.
- [ ] Quantum lowering uses "quantum::ir".
- [ ] HDL/hardware lowering uses the canonical domain boundary.
- [ ] No duplicate quantum IR exists.

Compiler

- [ ] Generic specialization is downstream.
- [ ] Optimization is downstream.
- [ ] Target selection is downstream.
- [ ] Routing is downstream.
- [ ] Scheduling is downstream.
- [ ] QEC is downstream.
- [ ] ZQN is downstream.
- [ ] HAL integration is downstream.

Runtime

- [ ] Runtime execution is not performed by grammar.
- [ ] Async runtime selection is downstream.
- [ ] Resource allocation is downstream.
- [ ] Deployment is downstream.

Safety

- [ ] No embedded Rust execution.
- [ ] No "unsafe" Rust requirement.
- [ ] No filesystem access.
- [ ] No network access.
- [ ] No process execution.
- [ ] No hardware discovery.
- [ ] No secret access.

Scalability

- [ ] No artificial function-count limit.
- [ ] No artificial parameter-count limit.
- [ ] No artificial generic-count limit.
- [ ] No artificial constraint-count limit.
- [ ] No artificial contract-count limit.
- [ ] No artificial domain limit.
- [ ] No artificial machine-size limit.
- [ ] Repetition is used instead of finite enumeration where appropriate.
- [ ] Parser resource limits are implementation policies, not language semantics.

POCO-REAF

- [ ] Function syntax is target-independent.
- [ ] Same function source can participate in classical execution.
- [ ] Same function source can participate in quantum execution.
- [ ] Same function source can participate in hybrid execution.
- [ ] Same function model can participate in HDL/hardware co-design.
- [ ] Same function model can scale across distributed execution.
- [ ] New computational domains can consume functions without rewriting the universal function model.
- [ ] Hardware selection occurs downstream.
- [ ] Physical realization occurs downstream.

Testing

- [ ] Positive tests exist.
- [ ] Negative tests exist.
- [ ] Boundary tests exist.
- [ ] Scalability tests exist.
- [ ] Determinism tests exist.
- [ ] Compatibility tests exist.
- [ ] Cross-domain tests exist.
- [ ] Round-trip tests exist where supported.
- [ ] Tests do not encode artificial hardware limits.

---

92. Definition of Done

A function grammar file is not done merely because ANTLR accepts it.

A file is done when:

syntax
  +
ownership
  +
dependencies
  +
lexer contract
  +
AST contract
  +
semantic contract
  +
IR integration
  +
compiler integration
  +
runtime integration
  +
tooling integration
  +
cross-domain integration
  +
diagnostics
  +
security
  +
determinism
  +
scalability
  +
compatibility
  +
positive tests
  +
negative tests
  +
boundary tests
  +
hard-coding audit

are all resolved.

After that point, another function grammar file may integrate with it through the already-defined contract, rather than requiring the completed file to be redesigned.

That is the required independently-completable architecture.

---

93. Final Architectural Rule

The central rule for "grammar/functions/" is:

«Functions are universal language-level computation units, not descriptions of particular machines.»

Therefore:

Zamani function
      ↓
portable syntax
      ↓
domain-neutral AST
      ↓
semantic meaning
      ↓
canonical domain IR
      ↓
optimization
      ↓
routing / scheduling / resilience
      ↓
resource/capability realization
      ↓
HAL
      ↓
target hardware/runtime

The source function must not need to know whether its eventual realization is:

one tiny processor
many processors
GPU
many GPUs
FPGA
ASIC
QPU
distributed cluster
edge device
cloud deployment
hybrid quantum-classical system
future computational substrate

The language expresses what the computation is and what it requires.

The compiler, resource system, runtime, and target stack determine how that computation is realized.

That separation is the function subsystem's core contribution to:

Program Once → Compile Once → Run Everywhere → Anywhere → Forever (POCO-REAF).

---

94. Authority Statement

For function syntax, the authority chain is:

language specification
        ↓
canonical lexical specification
        ↓
canonical function grammar
        ↓
Zamani.g4 composition
        ↓
generated parser
        ↓
domain-neutral frontend AST
        ↓
semantic analysis
        ↓
canonical IR
        ↓
compiler/runtime

Within "grammar/functions/":

functions.g4
parameters.g4
returns.g4
generics.g4
constraints.g4
contracts.g4
async.g4
closures.g4
generators.g4
compile-time-functions.g4
foreign-functions.g4

are the authoritative owners of their respective function-language syntax.

"README.md" is authoritative for the directory architecture and integration contract, but it does not replace the actual grammar rules.

No other document, generated reference, historical grammar, vendor grammar, or domain grammar may silently override this ownership model.