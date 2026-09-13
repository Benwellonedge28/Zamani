Zamani Function Grammar

Production Architecture and Integration Contract

Path: "grammar/functions/"

Language: Zamani

Grammar technology: ANTLR 4

Implementation baseline: Rust 1.97 / Rust 1.97.1, Edition 2021

Safety requirement: "unsafe" Rust is forbidden.

Architectural objective: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF).

Status: Production grammar subsystem specification.

---

1. Purpose

The "grammar/functions/" directory defines the complete source-level syntax of functions in Zamani.

Functions are one of the central composition mechanisms of the language. They must therefore be capable of representing portable computation across:

- classical computing;
- quantum computing;
- hybrid quantum-classical computing;
- HDL and hardware/software co-design;
- accelerators;
- embedded systems;
- distributed systems;
- parallel systems;
- HPC;
- AI/ML;
- numerical and symbolic computing;
- networking;
- cryptography;
- future execution models.

The function grammar describes what a function is syntactically, not how a particular machine executes it.

The directory must therefore remain independent of:

- CPU count;
- core count;
- thread count;
- GPU count;
- FPGA count;
- ASIC count;
- QPU count;
- qubit count;
- memory capacity;
- physical addresses;
- device IDs;
- topology;
- vendor;
- operating system;
- deployment size;
- scheduling strategy;
- routing strategy;
- optimization strategy;
- runtime implementation.

A function written once must remain semantically portable when the same program is lowered to different machines and execution environments.

---

2. Architectural Principle

The function grammar follows this boundary:

Zamani source
    |
    v
Canonical lexer
    |
    v
Function parser grammar
    |
    v
Canonical frontend AST
    |
    v
Name resolution
    |
    v
Type checking
    |
    v
Generic analysis
    |
    v
Effect/capability analysis
    |
    v
Semantic validation
    |
    +--------------------+
    |                    |
    v                    v
Classical IR        quantum::ir
    |                    |
    +---------+----------+
              |
              v
        Optimization
              |
              v
          Routing
              |
              v
         Scheduling
              |
              v
       Hardware HAL
              |
              v
           Runtime

The grammar is therefore syntax infrastructure, not the semantic execution engine.

---

3. Directory Ownership

The directory owns the syntax of:

- ordinary functions;
- function declarations;
- function signatures;
- parameters;
- return clauses;
- generic function parameters;
- generic function bounds;
- function constraints;
- closures;
- asynchronous functions;
- generators;
- compile-time functions;
- foreign/external function declarations;
- function modifiers;
- function-level attributes and integration points;
- function-level effect attachments;
- function-level capability/requirement attachments;
- function contracts;
- function-related syntax composition.

The directory does not own:

- lexical tokens;
- general expressions;
- general statements;
- general types;
- general declarations;
- module semantics;
- package semantics;
- name resolution;
- type inference;
- generic substitution;
- ownership;
- borrowing;
- lifetimes;
- effect semantics;
- capability resolution;
- resource allocation;
- target selection;
- hardware discovery;
- quantum IR;
- classical IR;
- QEC;
- ZQN;
- optimization;
- routing;
- scheduling;
- hardware calibration;
- runtime execution;
- ABI lowering;
- linking;
- dynamic loading;
- machine-specific resource limits.

Those responsibilities remain with their respective repository subsystems.

---

4. Current Function Grammar Files

The current directory contains:

grammar/functions/
├── async.g4
├── closures.g4
├── compile-time-functions.g4
├── constraints.g4
├── foreign-functions.g4
├── functions.g4
├── generators.g4
├── generics.g4
├── parameters.g4
└── returns.g4

A directory-level README was previously absent and is required to establish the contracts between these grammars.

The current "functions.g4" already defines itself as the canonical source-level function grammar and explicitly avoids owning type semantics, expressions, scheduling, routing, optimization, QEC, ZQN, quantum IR, runtime execution, ABI selection, and calling conventions. That ownership boundary is retained and strengthened here.

---

5. Required Final Directory

The target structure is:

grammar/functions/
├── README.md
├── functions.g4
├── parameters.g4
├── returns.g4
├── generics.g4
├── constraints.g4
├── closures.g4
├── async.g4
├── generators.g4
├── compile-time-functions.g4
├── foreign-functions.g4
└── tests/
    ├── README.md
    ├── functions/
    ├── parameters/
    ├── returns/
    ├── generics/
    ├── constraints/
    ├── closures/
    ├── async/
    ├── generators/
    ├── compile-time/
    ├── foreign/
    ├── negative/
    ├── boundary/
    ├── cross-domain/
    ├── determinism/
    └── roundtrip/

The test directories should only be created when the repository's grammar-test infrastructure requires them. They must not be empty placeholders.

---

6. File Ownership Matrix

File| Primary responsibility
"functions.g4"| Function declaration/definition/signature composition
"parameters.g4"| Parameter syntax
"returns.g4"| Return syntax
"generics.g4"| Function generic parameter syntax
"constraints.g4"| Function constraint syntax
"closures.g4"| Closure syntax
"async.g4"| Async syntax
"generators.g4"| Generator syntax
"compile-time-functions.g4"| Compile-time function syntax
"foreign-functions.g4"| Foreign/external function syntax
"README.md"| Directory architecture and integration contract
"tests/*"| Validation of the above contracts

No file may silently become a second owner of another file's grammar.

---

7. "functions.g4"

Purpose

"functions.g4" is the canonical composition point for ordinary Zamani functions.

It defines:

- function declarations;
- function definitions;
- function names;
- function modifiers;
- function signatures;
- generic attachment;
- parameter attachment;
- return attachment;
- effect attachment;
- contract attachment;
- implementation/body attachment.

Owns

functionDeclaration
functionSignature
functionName
functionModifier
functionImplementation

and the top-level composition of function components.

Does not own

It must not redefine:

typeExpression
expression
block
parameter
genericParameter
effect
capability
requirement
attribute

when those are owned elsewhere.

Dependencies

Conceptually:

lexer/tokens.g4
core/*
types/*
expressions/*
statements/*
functions/parameters.g4
functions/returns.g4
functions/generics.g4
functions/constraints.g4
effects/*

Integration

The aggregate parser must compose these components under one canonical token vocabulary.

No function grammar may define a competing lexer vocabulary.

---

8. "parameters.g4"

Purpose

Defines function parameter syntax.

It must support:

- named parameters;
- typed parameters;
- mutable parameters;
- default parameters where permitted;
- variadic parameters;
- parameter patterns when the canonical pattern system supports them;
- parameter attributes where the attribute system permits them;
- generic parameter references through the canonical type grammar.

It must not own

- type definitions;
- type semantics;
- expression semantics;
- generic substitution;
- ABI lowering;
- calling conventions;
- memory allocation.

Scalability

There must be no grammar limit such as:

MAX_PARAMETERS
MAX_ARGUMENTS
MAX_VARIADIC_ARGUMENTS

Repetition is represented through grammar repetition.

Implementation limits belong to explicit parser/compiler resource policies.

---

9. "returns.g4"

Purpose

Defines return syntax.

It owns:

- return type attachment;
- return-value syntax where appropriate;
- return-related function declaration composition.

The type itself is delegated to the canonical type grammar.

For example:

fn compute() -> int
fn transform<T>(value: T) -> T
fn measure() -> Measurement

The grammar must not enumerate every possible type.

This permits future types representing:

- classical values;
- tensors;
- quantum abstractions;
- logical resources;
- hardware abstractions;
- distributed values;
- future computational domains.

---

10. "generics.g4"

Purpose

Defines generic function parameter syntax.

It owns syntax such as:

<T>
<T, U>
<T: Numeric>
<T: Numeric + Comparable>

It must not own

- generic type semantics;
- type inference;
- substitution;
- monomorphization;
- specialization policy;
- trait solving;
- capability resolution.

Those belong to semantic/compiler infrastructure.

Scalability

No fixed generic arity may be encoded.

The grammar must not contain:

generic1
generic2
generic3
...

as a finite implementation scheme.

Recursive/repetitive grammar constructs must be used.

---

11. "constraints.g4"

Purpose

Defines function-level syntactic constraints.

Examples include constraints attached to:

- generic parameters;
- capabilities;
- requirements;
- effects;
- compile-time evaluation;
- function contracts.

Critical ownership rule

Function constraints must not duplicate the general type constraint system.

The function grammar consumes canonical constraint/type constructs wherever possible.

The distinction is:

Function grammar
    =
syntax attaching constraints to functions

Type/semantic system
    =
meaning and satisfiability of constraints

Lexer ownership

Tokens such as:

where
+
:
=

must come from the canonical lexer.

"constraints.g4" must not redefine them.

---

12. "closures.g4"

Purpose

Defines closure/lambda syntax.

The grammar must support portable closures without binding them to:

- stack layout;
- heap layout;
- CPU registers;
- calling convention;
- thread implementation;
- machine topology.

A closure is a language construct.

Its eventual representation is decided by later compilation stages.

Integration

Closures consume canonical:

expression
typeExpression
parameter
block

rules.

The closure grammar must not introduce alternative expression or type grammars.

---

13. "async.g4"

Purpose

Defines asynchronous function syntax.

It may express source-level concepts such as:

async fn ...

and associated function-level asynchronous declarations.

It does not own

- executors;
- schedulers;
- thread pools;
- event loops;
- runtime queues;
- CPU counts;
- GPU counts;
- distributed execution;
- cancellation implementation.

The runtime determines how asynchronous computation is executed.

POCO-REAF requirement

An async function must describe asynchronous semantics, not a particular execution mechanism.

Therefore:

async fn work()

must not imply:

use 8 threads
use CPU 0
use scheduler X

---

14. "generators.g4"

Purpose

Defines generator syntax.

Generators may represent:

- lazy computation;
- streams;
- iterators;
- resumable functions;
- producer computations.

The grammar must not encode:

- buffer sizes;
- worker counts;
- thread counts;
- queue capacity;
- physical memory.

Those are runtime/resource concerns.

---

15. "compile-time-functions.g4"

Purpose

Defines source syntax for compile-time functions and compile-time computation.

Compile-time functions may support:

- compile-time evaluation;
- constant generation;
- type-level computation where supported;
- compile-time metadata;
- specialization inputs;
- generated declarations.

Security boundary

Compile-time execution must not automatically imply unrestricted:

- filesystem access;
- network access;
- environment inspection;
- secret access;
- hardware access.

Such capabilities must be explicitly represented and checked through the language's capability/effect/security architecture.

The grammar describes syntax only.

The compiler must enforce the corresponding policy.

---

16. "foreign-functions.g4"

Purpose

Defines foreign/external function declarations.

A foreign declaration is an interface contract, not an implementation.

It may describe:

- external source language;
- external symbol;
- ABI metadata;
- linkage metadata;
- representation metadata;
- foreign parameters;
- foreign return types;
- foreign effects;
- foreign requirements;
- foreign capabilities;
- attributes.

The existing grammar already follows the correct architectural rule that foreign syntax must not perform linking, loading, hardware selection, scheduling, optimization, QEC, ZQN, or quantum-IR construction.

Critical rule

Do not turn foreign-function grammar into a list of hard-coded languages.

Do not require grammar changes merely because Zamani gains support for:

- C;
- C++;
- Rust;
- Python;
- Fortran;
- CUDA;
- OpenCL;
- OpenQASM;
- Verilog;
- SystemVerilog;
- a future language.

Language identity should remain extensible metadata.

---

17. Function Effects

Functions may interact with the effect system.

Examples of possible semantic effects include:

io
network
quantum
hardware
distributed
security

These names must not be exhaustively hard-coded into "functions.g4".

The grammar should provide an extensible attachment boundary.

The effect subsystem owns:

- effect definitions;
- effect semantics;
- effect composition;
- effect checking;
- effect inference;
- effect compatibility.

The function grammar merely attaches effects syntactically.

---

18. Capabilities and Requirements

A function may state semantic requirements.

Conceptually:

fn execute(...) requires ...

or the repository's canonical requirement syntax.

A requirement is not equivalent to selecting hardware.

For example:

requires quantum

must not mean:

use device X

and:

requires scalable_memory

must not mean:

allocate exactly N bytes

Requirements describe semantic/resource needs.

The resource, capability, target, and hardware subsystems determine whether and how those needs can be satisfied.

---

19. Function Contracts

Function contracts may express:

- preconditions;
- postconditions;
- invariants.

Example:

fn sqrt(x: Float) -> Float
    contract {
        requires(x >= 0);
        ensures(result >= 0);
    }
{
    ...
}

The grammar only recognizes the structure.

The semantic layer determines:

- whether the expression is valid;
- whether "result" is in scope;
- whether the contract is satisfiable;
- whether verification is possible;
- whether contracts execute at runtime;
- whether contracts are erased or retained.

---

20. Quantum Integration

Functions must be able to consume and return quantum abstractions without becoming a quantum grammar.

Examples conceptually include:

fn prepare(q: Qubit) -> Qubit
fn measure(q: Qubit) -> Measurement
fn transform<T>(value: T) -> T

The function grammar must not define:

Qubit
Gate
QuantumState
QuantumCircuit

unless those types are owned by "grammar/quantum/".

Likewise, function grammar must never define a canonical quantum representation.

The semantic lowering path is:

function syntax
      |
      v
frontend AST
      |
      v
semantic analysis
      |
      v
quantum::ir

"quantum::ir" remains the canonical quantum semantic boundary.

---

21. Classical Integration

Functions must support arbitrary classical values through the canonical type system.

The function grammar must not need changes when new classical types are added.

Examples include:

- scalar;
- vector;
- matrix;
- tensor;
- symbolic value;
- data structure;
- accelerator abstraction.

The function grammar only consumes their type syntax.

---

22. HDL Integration

Functions must be usable in hardware/software co-design.

However, ordinary function grammar must not become an HDL grammar.

HDL-specific syntax remains owned by:

grammar/hdl/*

Functions may interact with hardware constructs through:

- canonical types;
- effects;
- capabilities;
- requirements;
- attributes;
- interfaces;
- hardware function declarations where explicitly defined.

A function must not hard-code:

FPGA_COUNT
ASIC_COUNT
REGISTER_COUNT
PIPELINE_DEPTH
DEVICE_ID

as language-wide limits.

---

23. Distributed Integration

A function may conceptually execute in a distributed environment.

The grammar must not require:

node0
node1
node2

or any fixed number of nodes.

Distributed placement is a semantic/resource/deployment concern.

Function syntax may express portable distributed intent when the distributed grammar provides the canonical syntax.

---

24. AI and Accelerator Integration

Functions must be capable of operating on AI and accelerator types without special-case function grammar.

For example:

fn infer(model: Model, input: Tensor) -> Tensor

does not need to know whether execution occurs on:

- CPU;
- GPU;
- TPU-like accelerator;
- FPGA;
- quantum accelerator;
- future accelerator.

Target selection occurs later.

---

25. Hardware Independence

The following must never appear as grammar-level function limits:

MAX_CPU
MAX_GPU
MAX_FPGA
MAX_ASIC
MAX_QPU
MAX_QUBITS
MAX_THREADS
MAX_MEMORY
MAX_FUNCTIONS
MAX_PARAMETERS

The grammar must remain structurally unbounded subject only to implementation resource availability.

Any implementation limit must be represented separately as an explicit:

parser resource policy
compiler resource policy
runtime resource policy
deployment constraint

and never confused with language semantics.

---

26. No Machine-Specific Function Syntax

The following kinds of syntax are prohibited from ordinary function grammar unless explicitly defined as a separate target/deployment language:

fn foo on cpu0
fn foo on gpu3
fn foo using 64 threads
fn foo on qpu7
fn foo with 128 qubits
fn foo at address 0x...

Such information belongs to target/resource/deployment layers.

Portable function syntax should instead express intent such as:

requires capability(...)
requires resource(...)
prefers(...)
constraint(...)

where those constructs are defined by the canonical resource architecture.

---

27. Canonical Dependency Rules

The dependency direction must remain:

lexer
  |
  v
core
  |
  +--> types
  |
  +--> expressions
  |
  +--> statements
  |
  v
functions
  |
  +--> semantic analysis
  |
  +--> AST
  |
  v
IR

The following cycles are prohibited:

functions -> IR -> functions
functions -> runtime -> functions
functions -> hardware -> functions
functions -> quantum::ir -> functions

Function grammar may refer to the syntax contracts of those subsystems but must not depend on their runtime implementation.

---

28. AST Contract

Every function grammar construct must lower into a canonical frontend AST representation.

The AST should preserve, where semantically meaningful:

- source span;
- declaration identity;
- function name;
- modifiers;
- generic parameters;
- generic bounds;
- parameter order;
- parameter modifiers;
- parameter patterns;
- parameter types;
- defaults;
- variadic status;
- return type;
- effects;
- requirements;
- capabilities;
- contracts;
- body;
- foreign metadata;
- source provenance.

The AST must not silently discard source information needed by later semantic analysis.

---

29. Generic AST Integration

Generic parameter ordering must remain deterministic.

Generic bounds must preserve their source order unless the semantic layer explicitly canonicalizes them.

The function grammar must not decide whether:

T: A + B

is semantically equivalent to:

T: B + A

That is a semantic question.

---

30. Quantum AST Integration

A function such as:

fn execute(q: Qubit) -> Measurement

must produce ordinary function AST structures whose types refer to the canonical quantum type system.

The function grammar must not create a special:

QuantumFunction

syntax merely because a function has quantum parameters.

This preserves cross-domain composability.

---

31. Classical IR Integration

Classical functions lower through the repository's canonical classical semantic/IR infrastructure.

Function grammar does not create a separate function IR.

---

32. Quantum IR Integration

Quantum operations originating inside a function body eventually lower through:

quantum::ir

The function grammar does not own:

- gate representation;
- physical qubit representation;
- routing;
- scheduling;
- QEC;
- ZQN.

This prevents a second quantum architecture from forming inside the grammar.

---

33. Optimization Integration

Function grammar has no knowledge of optimization strategy.

Later optimization may:

- inline functions;
- eliminate dead functions;
- specialize generic functions;
- transform calls;
- optimize quantum operations;
- optimize classical operations;
- fuse accelerator operations.

Those transformations must operate on the appropriate semantic/IR representation.

---

34. Scheduling Integration

Function syntax does not determine execution order beyond language semantics.

Scheduling is responsible for target-specific:

- operation ordering;
- timing;
- resource conflicts;
- synchronization;
- alignment;
- dynamic execution constraints.

Function grammar must not encode hardware timing grids.

---

35. Routing Integration

Function grammar does not choose:

- physical qubits;
- communication paths;
- network routes;
- hardware topology.

Routing receives the appropriate semantic representation after analysis/lowering.

---

36. Hardware HAL Integration

Hardware capabilities are discovered and represented by the hardware abstraction layer.

The function grammar can express semantic requirements/capabilities through canonical syntax.

It must never inspect the hardware itself.

---

37. Runtime Integration

The runtime receives compiled semantic artifacts and execution information.

The grammar must not contain:

- runtime calls;
- runtime discovery;
- scheduler implementations;
- backend selection;
- device discovery.

---

38. Interoperability Integration

Foreign functions integrate through:

functions/foreign-functions.g4

and the canonical interoperability/ABI infrastructure.

The grammar describes declarations.

The compiler/runtime determines:

- ABI lowering;
- symbol resolution;
- linking;
- loading;
- marshaling;
- calling convention;
- target implementation.

---

39. Lexer Contract

All function grammars must consume the canonical lexer vocabulary.

They must not redefine lexer tokens.

Important examples include tokens corresponding to:

fn
pub
private
async
extern
mut
where
requires
ensures
invariant
with
effect

The actual canonical spelling and token ownership must come from:

grammar/lexer/

The function grammars must reference those canonical tokens rather than creating local substitutes.

---

40. ANTLR Contract

ANTLR grammar structure must remain valid according to ANTLR's parser/lexer grammar rules.

Parser grammars must use parser-rule naming conventions and a consistent token vocabulary. ANTLR's grammar model requires parser rules to begin with lowercase names and lexer rules with uppercase names.

The aggregate Zamani grammar must be responsible for composing delegate grammars.

There must be one authoritative token vocabulary.

---

41. Rust Contract

Generated parser integration must target:

Rust 1.97
Rust 1.97.1
Edition 2021

The implementation must be safe Rust.

The compiler/frontend crates should enforce:

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

No grammar file may require unsafe Rust.

No grammar feature may assume unsafe implementation techniques.

---

42. Determinism

Parsing must depend only on:

- source text;
- canonical lexer configuration;
- parser configuration.

It must not depend on:

- machine hardware;
- network state;
- filesystem state;
- wall-clock time;
- randomness;
- backend availability;
- device discovery.

The same source and grammar version must produce deterministic parse structure.

---

43. Diagnostics

Function grammar errors must preserve:

- source span;
- token position;
- expected construct;
- actual construct;
- grammar context;
- stable diagnostic category where provided by frontend infrastructure.

The grammar itself should not invent backend-specific diagnostics.

Semantic errors must remain distinguishable from syntax errors.

For example:

fn f(x: UnknownType)

may be syntactically valid but semantically invalid.

The parser must not pretend that type resolution is parsing.

---

44. Error Recovery

Parser recovery must not silently convert invalid function declarations into valid AST structures.

Malformed function declarations must remain diagnosable.

Particular attention is required for:

- generic delimiters;
- nested parameter lists;
- nested type expressions;
- nested expressions;
- closure syntax;
- function contracts;
- foreign declarations;
- variadic parameters.

---

45. Ambiguity Requirements

The function grammar must avoid competing productions that recognize the same source construct with materially different interpretations.

Particular care is required around:

generic parameters
<
>
closures
|
function types
parameters
variadic syntax
foreign declarations
attributes
contracts

Canonical shared rules must be reused rather than duplicated.

---

46. Variadic Functions

Variadic functions must not have a finite grammar limit.

A variadic declaration represents an unbounded sequence subject to semantic/runtime constraints.

For example:

fn collect(...values: T)

does not imply a maximum number of values.

The ABI and runtime may impose resource constraints at execution time.

Those constraints are not language grammar limits.

---

47. Default Parameters

Default arguments must use canonical expressions.

The grammar must not attempt to determine:

- constant evaluability;
- type compatibility;
- side-effect safety;
- hardware availability.

Those belong to semantic analysis.

---

48. Function Modifiers

Modifiers must be classified before implementation into:

Declaration modifiers

Examples:

pub
private
internal
protected

Execution/semantic modifiers

Examples:

async
const

Linking/interoperability modifiers

Examples:

extern

Type-system modifiers

Where applicable:

generic

Implementation hints

Examples:

inline

The grammar recognizes syntax.

Semantic analysis determines whether combinations are legal.

The grammar must not become the semantic compatibility matrix.

---

49. Modifier Compatibility

Invalid modifier combinations must be rejected by semantic validation unless the syntax itself is inherently impossible.

For example, whether a particular combination of:

extern
abstract
async
inline

is legal belongs to language semantics/compiler policy.

The parser should preserve valid structural combinations sufficiently for semantic diagnostics.

---

50. Compile-Time Functions and POCO-REAF

Compile-time computation must not make source programs permanently dependent on one compiler machine.

Compile-time functions should produce semantic results that can be serialized/reproduced deterministically where required.

Compile-time evaluation must therefore have explicit policy for:

- determinism;
- capabilities;
- resource limits;
- reproducibility;
- diagnostics;
- version compatibility.

A compile-time function must not silently inspect arbitrary hardware.

---

51. Foreign Functions and POCO-REAF

Foreign interfaces are inherently less portable than ordinary Zamani functions.

Therefore the language must distinguish:

portable semantic function

from:

foreign implementation boundary

A foreign declaration may carry target/interface metadata, but the core function semantics remain independent.

The compiler may later select an implementation appropriate to a target.

---

52. Security

Function grammar must not provide implicit access to:

- filesystem;
- network;
- credentials;
- secrets;
- arbitrary native code;
- hardware registers;
- privileged instructions.

Such operations must require explicit language-level capabilities/effects where the security architecture requires them.

Foreign-function declarations are especially security-sensitive because they cross the language safety boundary.

Their semantic validation must therefore remain explicit.

---

53. No Hidden Resource Semantics

A function declaration must never secretly allocate or reserve resources merely because syntax mentions a type.

For example:

fn work(q: Qubit)

does not allocate a physical qubit.

Likewise:

fn work(buffer: Buffer)

does not specify a fixed memory capacity.

Allocation is performed later by the appropriate resource/runtime subsystem.

---

54. Future-Proofing

Adding a new computational domain must not require rewriting ordinary function grammar.

For example, adding:

neuromorphic
photonic
analog
biological
optical
future accelerator

should primarily require:

- new types;
- new effects;
- new capabilities;
- new semantic/IR support;
- new lowering;
- new target integration.

It should not require redesigning function declaration syntax.

---

55. Testing Strategy

Every function grammar component requires:

Positive tests

Valid:

- simple functions;
- typed functions;
- generic functions;
- constrained functions;
- variadic functions;
- closures;
- async functions;
- generators;
- compile-time functions;
- foreign functions;
- functions using quantum types;
- functions using classical types;
- functions using HDL types;
- hybrid functions.

Negative tests

Invalid:

- malformed names;
- malformed parameter lists;
- duplicate delimiters;
- malformed generic lists;
- invalid return syntax;
- malformed constraints;
- malformed contracts;
- malformed foreign declarations;
- malformed variadic syntax.

Boundary tests

Test very small and very large source structures.

Do not encode artificial finite grammar limits merely to make tests easier.

Cross-domain tests

At minimum:

classical + quantum
classical + HDL
quantum + HDL
quantum + hardware
quantum + distributed
AI + quantum
AI + hardware
classical + quantum + distributed
classical + quantum + HDL + hardware

Determinism tests

Repeated parsing of identical input must produce equivalent parse structures.

Round-trip tests

Where a canonical printer/serializer exists:

source
  -> lexer
  -> parser
  -> AST
  -> printer
  -> parser

must preserve semantics.

---

56. Hard-Coding Audit

Every function grammar change must be checked for accidental limits.

Search for:

MAX_
LIMIT_
QUANTUM
QUBIT
CPU
GPU
FPGA
ASIC
THREAD
MEMORY
DEVICE
TOPOLOGY
ADDRESS
VENDOR

Every occurrence must be classified as:

1. language semantic requirement;
2. target-specific metadata;
3. resource constraint;
4. implementation limit;
5. accidental hard-coding;
6. test-only limitation;
7. documentation-only example.

Only the first six categories may be retained when justified.

Accidental hard-coding must be removed.

---

57. Repository Integration

The function grammar must integrate with:

grammar/lexer/
grammar/core/
grammar/types/
grammar/expressions/
grammar/statements/
grammar/declarations/
grammar/modules/
grammar/effects/
grammar/memory/
grammar/concurrency/
grammar/classical/
grammar/quantum/
grammar/hybrid/
grammar/hdl/
grammar/hardware/
grammar/distributed/
grammar/ai/
grammar/resources/
grammar/compile/
grammar/execution/
grammar/interoperability/
grammar/dialects/
grammar/macros/
grammar/metaprogramming/

The function directory must not duplicate concepts owned by those directories.

---

58. Repository-Level Compiler Integration

The grammar must ultimately integrate with:

frontend
AST
semantic analysis
type system
capability analysis
effect analysis
classical IR
quantum::ir
optimization
routing
scheduling
hardware HAL
resource management
runtime
interoperability
verification
diagnostics
testing

The dependency direction remains:

grammar
   ↓
AST
   ↓
semantic analysis
   ↓
IR
   ↓
compiler transformations
   ↓
target realization
   ↓
runtime

Never reverse this relationship.

---

59. QEC and ZQN Boundary

Function grammar may express functions that manipulate quantum resources.

It must not define QEC or ZQN semantics.

The boundaries remain:

Function grammar
    |
    v
semantic quantum representation
    |
    v
quantum::ir
    |
    +--> QEC
    |
    +--> ZQN
    |
    +--> optimization
    |
    +--> routing
    |
    +--> scheduling

QEC owns error correction.

ZQN owns fault/noise semantics.

The function grammar owns neither.

---

60. Scheduling Boundary

Function grammar does not determine:

- ASAP scheduling;
- ALAP scheduling;
- resource-constrained scheduling;
- pulse timing;
- hardware timing;
- dynamic-circuit timing.

A function body provides semantic ordering constraints.

The scheduling subsystem derives an executable schedule later.

---

61. Resource Boundary

Function syntax may state requirements.

Resource management determines actual availability.

Therefore:

function requirement
        !=
resource allocation

and:

capability declaration
        !=
hardware discovery

This distinction is mandatory for POCO-REAF.

---

62. Compatibility

Function syntax is part of the public Zamani language contract.

Breaking changes require:

- language-version documentation;
- migration guidance;
- compatibility classification;
- deprecated syntax policy;
- parser compatibility tests.

Existing valid function syntax must not be removed silently.

---

63. Versioning

Every future incompatible grammar modification must identify:

language version
grammar version
compatibility impact
migration path

The function directory must not create an independent incompatible versioning system.

It consumes the repository's canonical language-version contract.

---

64. Deprecation

Deprecated function syntax must have:

- a documented replacement;
- a deprecation version;
- a removal policy;
- diagnostics;
- migration guidance;
- compatibility tests while supported.

Deprecated syntax must not remain undocumented indefinitely.

---

65. Documentation Contract

This README is the directory-level architecture contract.

Individual ".g4" files remain authoritative for their own grammar productions.

Repository-level grammar documentation must describe the language as a whole.

Generated parser artifacts must never become the source of truth.

---

66. Independent File Completion Contract

A grammar file is not complete merely because ANTLR accepts it.

A file is complete only when:

- ownership is documented;
- non-ownership is documented;
- dependencies are known;
- token dependencies are canonical;
- rule dependencies are canonical;
- AST mapping is defined;
- semantic integration is defined;
- downstream consumers are identified;
- tests exist;
- negative tests exist;
- boundary tests exist;
- compatibility impact is known;
- scalability has been audited;
- hard-coded limits have been audited;
- deterministic behavior has been verified;
- cross-domain integration has been considered.

This ensures that completing one file does not require redesigning it later because another function grammar was implemented.

---

67. File Completion Records

Before implementing each file, establish:

File:
Purpose:
Owns:
Does Not Own:
Inputs:
Outputs:
Dependencies:
Upstream Contracts:
Downstream Consumers:
Public Grammar Contract:
AST Contract:
Semantic Contract:
IR Integration:
Compiler Integration:
Runtime Integration:
Tooling Integration:
Cross-Domain Integration:
Tests:
Negative Tests:
Boundary Tests:
Compatibility Requirements:
Scalability Requirements:
Hard-Coding Audit:
Completion Criteria:

These records should be reviewed before implementation begins.

---

68. Dependency-First Implementation Order

The recommended order inside this directory is:

1. README.md

2. functions.g4
3. parameters.g4
4. returns.g4

5. generics.g4
6. constraints.g4

7. closures.g4
8. async.g4
9. generators.g4

10. compile-time-functions.g4

11. foreign-functions.g4

12. function integration tests
13. cross-domain tests
14. determinism tests
15. round-trip tests

However, the repository-level dependency graph has priority.

For example, if "parameters.g4" requires a canonical type rule that is not yet final, the type grammar must be completed first.

The function grammar must not invent a temporary type system merely to unblock implementation.

---

69. Completion Gate for "functions.g4"

"functions.g4" is complete only when:

- ordinary declarations parse;
- ordinary definitions parse;
- signatures parse;
- modifiers are integrated;
- generics integrate;
- parameters integrate;
- returns integrate;
- effects integrate;
- contracts integrate;
- bodies integrate;
- foreign functions remain separate;
- no type rules are duplicated;
- no expression rules are duplicated;
- no machine-specific limits exist;
- AST mapping is documented;
- diagnostics are deterministic;
- all required tests pass.

---

70. Completion Gate for "parameters.g4"

Complete only when:

- ordinary parameters work;
- typed parameters work;
- modifier integration works;
- defaults work;
- variadics work;
- canonical types are reused;
- canonical expressions are reused;
- no argument-count limits exist;
- invalid combinations are test-covered;
- AST representation is stable.

---

71. Completion Gate for "returns.g4"

Complete only when:

- no-return-type functions work;
- typed return functions work;
- canonical type grammar is reused;
- generic return types work;
- quantum/classical/HDL types can flow through;
- no domain-specific duplication exists.

---

72. Completion Gate for "generics.g4"

Complete only when:

- generic parameter lists work;
- arbitrary generic arity is supported;
- bounds integrate with canonical constraints/types;
- nesting is not artificially limited;
- semantic substitution is not performed by grammar;
- generic AST representation is stable.

---

73. Completion Gate for "constraints.g4"

Complete only when:

- function constraints parse;
- canonical operators are reused;
- canonical type constraints are reused;
- duplicate constraint systems have been removed;
- semantic validation is outside grammar;
- generic constraints are deterministic.

---

74. Completion Gate for "closures.g4"

Complete only when:

- closure syntax parses;
- parameters reuse canonical parameter structures;
- expressions reuse canonical expression structures;
- captures remain semantic information;
- no runtime representation leaks into syntax.

---

75. Completion Gate for "async.g4"

Complete only when:

- async declarations parse;
- async semantics are represented in AST;
- executor/runtime decisions remain outside grammar;
- no thread/device count is encoded;
- cancellation integration is defined.

---

76. Completion Gate for "generators.g4"

Complete only when:

- generator declarations parse;
- yield/resume semantics are represented correctly;
- storage and scheduling remain outside grammar;
- arbitrary generator size is permitted.

---

77. Completion Gate for "compile-time-functions.g4"

Complete only when:

- compile-time declarations parse;
- capability boundaries are explicit;
- deterministic evaluation requirements are documented;
- resource policy is external;
- no implicit unrestricted environment access exists.

---

78. Completion Gate for "foreign-functions.g4"

Complete only when:

- external declarations parse;
- external symbol metadata works;
- language metadata is extensible;
- ABI metadata is extensible;
- linkage metadata is extensible;
- representation metadata is extensible;
- parameters and returns use canonical types;
- no hard-coded foreign language list exists;
- linking remains outside grammar;
- dynamic loading remains outside grammar;
- security boundaries are explicit.

The existing foreign grammar already establishes this separation and should be preserved rather than replaced with provider-specific syntax.

---

79. Cross-Domain Function Requirement

At least one integration fixture must demonstrate a function whose semantic signature crosses multiple domains.

Conceptually:

fn execute<T>(
    classical_input: T,
    quantum_resource: QuantumResource,
    hardware_target: HardwareCapability
) -> Result<T>

The exact syntax must use the canonical repository rules rather than introducing special function syntax.

This validates that the function layer remains domain-neutral.

---

80. POCO-REAF Acceptance Test

The function grammar passes the POCO-REAF architectural test only if a function's source syntax does not need to change solely because the implementation target changes from:

tiny embedded machine
        ↓
CPU
        ↓
multicore CPU
        ↓
GPU
        ↓
FPGA
        ↓
ASIC
        ↓
QPU
        ↓
simulator
        ↓
cluster
        ↓
cloud
        ↓
future architecture

The function's semantic meaning remains constant.

Only later compilation, lowering, placement, scheduling, routing, resource management, and runtime realization may change.

---

81. Scalability Acceptance Test

The grammar must scale according to available implementation resources.

There must be no grammar-level assumption that the language can only represent:

N functions
N parameters
N generic parameters
N nested calls
N quantum values
N distributed resources

for an arbitrary fixed "N".

Any practical parser recursion/resource limits must be explicit implementation policies and must never be presented as language semantics.

---

82. "Infinity" Clarification

"Scale to infinity" is interpreted architecturally as:

«No artificial finite language limit is encoded where the underlying computation model is conceptually unbounded.»

Actual execution remains bounded by available:

- memory;
- compute;
- parser resources;
- compiler resources;
- runtime resources;
- target resources;
- physical laws.

Those practical limits must not become accidental grammar limits.

---

83. What Must Never Be Added

Do not add:

MAX_PARAMETERS
MAX_FUNCTIONS
MAX_GENERIC_PARAMETERS
MAX_QUBITS
MAX_THREADS
MAX_DEVICES
MAX_NODES
MAX_GPUS
MAX_FPGAS
MAX_MEMORY

Do not add:

fn_on_gpu
fn_on_qpu
fn_on_cpu
fn_on_fpga

as permanent core language constructs merely to support individual targets.

Do not add separate:

QuantumFunction
GPUFunction
FPGAFunction
DistributedFunction
AI_Function

grammars when ordinary function semantics can express the same concept through canonical types/effects/capabilities.

---

84. Repository Consistency Rule

If another grammar directory already owns a concept, "functions/" must reference it.

Examples:

Type
    -> grammar/types/

Expression
    -> grammar/expressions/

Statement
    -> grammar/statements/

Effect
    -> grammar/effects/

Capability
    -> grammar/core/

Requirement
    -> grammar/core/resources or resources/

Module
    -> grammar/modules/

Quantum
    -> grammar/quantum/

Hardware
    -> grammar/hardware/

HDL
    -> grammar/hdl/

No duplicate semantic authority is permitted.

---

85. Generated Artifacts

Generated ANTLR parser artifacts must not be committed as the authoritative grammar source unless the repository explicitly requires generated sources.

When generated artifacts are committed, they must be reproducible from the grammar sources and their generation procedure must be documented.

The source ".g4" files remain authoritative.

---

86. Grammar Composition

The aggregate grammar must provide a single coherent parser.

Function delegates must not create isolated language islands.

The final parser should conceptually compose:

ZamaniTokens
      |
      +-- Core
      +-- Types
      +-- Expressions
      +-- Statements
      +-- Declarations
      +-- Modules
      +-- Functions
      +-- Effects
      +-- Memory
      +-- Concurrency
      +-- Classical
      +-- Quantum
      +-- Hybrid
      +-- HDL
      +-- Hardware
      +-- Distributed
      +-- AI
      +-- Data
      +-- Networking
      +-- Security
      +-- Resources
      +-- Compilation
      +-- Execution
      +-- Interoperability
      +-- Dialects
      +-- Macros
      +-- Metaprogramming

Function syntax must remain a reusable component within this architecture.

---

87. Review Checklist

Before merging any function grammar change, verify:

Ownership

- [ ] The file owns the construct it modifies.
- [ ] No second owner exists.
- [ ] Non-ownership is documented.

Syntax

- [ ] ANTLR grammar is valid.
- [ ] Canonical tokens are reused.
- [ ] Shared parser rules are reused.
- [ ] Ambiguity has been reviewed.

Semantics

- [ ] Syntax is separated from semantic validation.
- [ ] Type semantics remain outside function grammar.
- [ ] Effect semantics remain outside function grammar.
- [ ] Capability semantics remain outside function grammar.

Scalability

- [ ] No artificial machine limits exist.
- [ ] No fixed hardware count exists.
- [ ] No fixed qubit count exists.
- [ ] No fixed thread count exists.
- [ ] No fixed memory limit exists.
- [ ] No fixed device count exists.

Integration

- [ ] AST mapping is defined.
- [ ] Classical IR integration is defined.
- [ ] "quantum::ir" integration is preserved.
- [ ] QEC boundary is preserved.
- [ ] ZQN boundary is preserved.
- [ ] optimization boundary is preserved.
- [ ] routing boundary is preserved.
- [ ] scheduling boundary is preserved.
- [ ] hardware HAL boundary is preserved.
- [ ] runtime boundary is preserved.
- [ ] interoperability boundary is preserved.

Safety

- [ ] No unsafe Rust is required.
- [ ] Foreign boundaries are explicitly represented.
- [ ] Compile-time capabilities are explicit.
- [ ] No implicit privileged access exists.

Testing

- [ ] Positive tests exist.
- [ ] Negative tests exist.
- [ ] Boundary tests exist.
- [ ] Cross-domain tests exist.
- [ ] Determinism tests exist.
- [ ] Round-trip tests exist where supported.

---

88. Final Architectural Contract

The function grammar exists to express:

«portable computation, composition, abstraction, intent, constraints, and interfaces.»

It does not exist to encode:

«the accidental characteristics of today's hardware.»

Therefore:

Function
    |
    v
Portable semantic meaning
    |
    +--> classical realization
    +--> quantum realization
    +--> hybrid realization
    +--> HDL realization
    +--> accelerator realization
    +--> distributed realization
    +--> future realization

The source-level function remains stable while later compilation stages adapt it to available capabilities and resources.

---

89. Final POCO-REAF Principle

The "grammar/functions/" subsystem is production-ready only when the following statement is true:

«A Zamani function can be written once as a description of computation and intent, parsed deterministically, represented canonically, semantically validated, and lowered through the repository's existing compiler architecture without embedding arbitrary assumptions about the machine on which it will eventually execute.»

The architecture must preserve:

One source program
        ↓
One semantic meaning
        ↓
Many compilation strategies
        ↓
Many hardware configurations
        ↓
Many execution environments
        ↓
Many scales
        ↓
Future platforms

with:

No accidental hardware limits
No duplicated semantic authority
No duplicated quantum IR
No grammar-level resource ceilings
No hidden runtime assumptions
No unsafe implementation requirement
No provider-specific core syntax
No circular dependencies
Deterministic parsing
Explicit ownership
Explicit integration contracts
Stable evolution

The governing principle is:

«Zamani functions describe computation and intent, not arbitrary limitations of the machine currently available.»

That is the required foundation for:

Zamani — From Atom to Everywhere

and:

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever