Worked for 20s

Below is the complete proposed production-ready grammar/specification/syntax-model.md. It is written as a normative syntax-model contract, so it defines what syntax means structurally while deliberately leaving type checking, resource availability, quantum lowering, scheduling, routing, hardware realization, and runtime behavior to their owning subsystems.

I also reconciled it with the repository's current grammar authority model: grammar/Zamani.g4 is the canonical ANTLR concrete grammar, grammar/grammar.md describes implemented frontend behavior, grammar/Zamani-Grammar.md is extended design material, and grammar/specification/ is the normative specification layer.  The repository currently contains grammar/specification/ alongside the existing grammar surfaces.

Zamani Syntax Model

Path: "grammar/specification/syntax-model.md"
Status: Normative
Specification Layer: Language architecture / syntax
Language: Zamani
Compiler Baseline: Rust 1.97 / Rust 1.97.1
Safety Requirement: Safe Rust only; "unsafe" is prohibited
Grammar Technology: ANTLR4-compatible concrete grammar, with equivalent behavior permitted in the reference Rust frontend
Portability Model: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)
Scalability Model: From the smallest representable computation to arbitrarily large computations, limited only by the language's representational model, implementation resources, target capabilities, and physical reality.

---

1. Purpose

This document defines the normative syntax model of the Zamani programming language.

It establishes:

- what constitutes a Zamani source unit;
- how source text is structurally organized;
- how lexical elements compose into syntax;
- how declarations, expressions, statements, types, modules, effects, resources, quantum constructs, classical constructs, HDL constructs, hardware constructs, and future domains compose;
- how syntax remains independent of target-machine scale;
- how source syntax integrates with the AST;
- how syntax integrates with semantic analysis;
- how domain syntax reaches canonical IRs;
- how syntax remains compatible with POCO-REAF;
- how syntax evolves without creating incompatible language dialects;
- which concerns belong in syntax and which must remain downstream.

This document does not define the complete semantic behavior of every operation.

It defines the structural contract that all conforming Zamani frontends must implement.

The central rule is:

«Zamani syntax describes a computation and its declared intent, not the accidental physical characteristics of the machine on which that computation happens to execute.»

---

2. Normative Authority

The syntax model participates in the repository's grammar authority hierarchy.

The authority relationship is:

grammar/specification/
        │
        │ normative language specification
        ▼
syntax-model.md
        │
        ▼
grammar/Zamani.g4
        │
        ├── ANTLR lexer/parser
        │
        └── grammar validation
        │
        ▼
reference frontend
        │
        ▼
AST
        │
        ▼
semantic analysis
        │
        ├── type analysis
        ├── effect analysis
        ├── capability analysis
        ├── resource analysis
        └── domain validation
        │
        ▼
canonical semantic representations
        │
        ├── classical IR
        └── quantum::ir
        │
        ▼
optimization / QEC / ZQN / scheduling / routing
        │
        ▼
hardware / runtime / deployment

The repository currently has multiple grammar-related surfaces. They are not independent language authorities.

The required roles are:

Artifact| Authority| Responsibility
"grammar/specification/"| Normative| Language contracts
"grammar/specification/syntax-model.md"| Normative| Syntax architecture
"grammar/Zamani.g4"| Canonical concrete syntax| ANTLR grammar
"grammar/grammar.md"| Implementation conformance| Currently implemented frontend syntax
"grammar/Zamani-Grammar.md"| Design/reference| Extended and historical language material
"grammar/README.md"| Navigation| Grammar architecture
Rust lexer| Executable implementation| Tokenization
Rust parser| Executable implementation| Parsing
"src/ast/"| Structural representation| AST
semantic analysis| Executable semantics| Meaning validation
classical IR| Classical semantic boundary| Classical representation
"src/quantum/ir/"| Canonical quantum semantic boundary| Quantum representation

The existence of these artifacts MUST NOT create multiple incompatible definitions of Zamani.

---

3. Syntax Versus Semantics

Zamani MUST maintain a strict separation between:

syntax
semantics
types
effects
capabilities
resources
target realization
runtime behavior

A syntactically valid construct is not automatically semantically valid.

For example:

requires quantum;

may be syntactically valid.

Whether the selected execution environment provides the required capability is not a syntax question.

Similarly:

let q: qubit = allocate();

may be structurally valid while semantic/resource analysis determines whether the requested resource can actually be provided.

The following distinctions are mandatory:

syntactic validity
        ≠
type validity
        ≠
effect validity
        ≠
capability validity
        ≠
resource satisfiability
        ≠
target compatibility
        ≠
runtime availability

A frontend MUST NOT reject syntactically valid programs merely because the current machine lacks sufficient resources, unless the compilation mode explicitly requires target validation.

---

4. Source Model

A Zamani source program is an ordered sequence of source units.

The conceptual structure is:

Program
 ├── documentation
 ├── attributes
 ├── package/module declarations
 ├── imports
 ├── declarations
 ├── definitions
 └── executable/top-level constructs where permitted

A conforming implementation MUST preserve source ordering sufficiently to support:

- deterministic diagnostics;
- declaration visibility rules;
- source locations;
- documentation;
- attributes;
- reproducible AST construction;
- deterministic semantic analysis.

The syntax model MUST NOT require a fixed number of declarations.

The source program may contain:

zero declarations
one declaration
many declarations
arbitrarily many declarations

subject to available parser and host resources.

---

5. Compilation Unit

The canonical conceptual entry point is:

compilationUnit

A compilation unit represents one independently parseable Zamani source artifact.

A compilation unit may contain:

- an optional source-level version declaration where the language version system permits it;
- documentation;
- attributes;
- package information;
- module information;
- imports;
- exports;
- declarations;
- definitions;
- executable top-level constructs where permitted.

The grammar MUST have exactly one canonical root entry point for complete Zamani source.

Additional parser entry points may exist for tooling, incremental parsing, testing, or embedded DSLs, but they MUST NOT define alternate language roots.

---

6. Source Locations

Every syntactic node that can participate in diagnostics SHOULD preserve:

- start position;
- end position;
- source identifier;
- line;
- column;
- byte or character span as appropriate to the frontend representation.

Source locations are metadata.

They MUST NOT alter program semantics.

Source locations MUST remain deterministic for identical source input.

The frontend MUST NOT use source location as an implicit machine-resource selector.

---

7. Whitespace

Whitespace is generally syntactically insignificant except where required to separate lexical elements.

Implementations MUST support:

- spaces;
- tabs;
- line endings;
- platform-independent newline conventions.

The syntax MUST NOT depend on a particular operating-system line ending.

Where indentation is not part of Zamani's normative syntax, indentation MUST NOT determine program meaning.

Formatting MUST remain separable from parsing.

---

8. Comments and Documentation

Zamani supports comments and documentation as distinct concepts.

Comments:

// comment

or equivalent supported forms are ignored semantically.

Documentation comments are source metadata and MAY be attached to declarations.

Documentation MUST NOT change program execution semantics.

The grammar MUST preserve documentation where required by tooling.

A parser MUST NOT silently reinterpret arbitrary comments as executable constructs.

---

9. Identifiers

Zamani identifiers represent names.

An identifier may name:

- modules;
- packages;
- functions;
- variables;
- constants;
- types;
- traits;
- interfaces;
- structures;
- enums;
- resources;
- capabilities;
- quantum entities;
- hardware entities;
- HDL entities;
- domains;
- effects;
- user-defined symbols.

Identifiers MUST be represented independently from physical resource identifiers.

For example:

let q: qubit;

does not mean that "q" is physical qubit 0.

Likewise:

device quantum_backend;

does not imply a particular vendor device.

Names are semantic symbols.

Physical identity is resolved by downstream compilation or runtime systems.

---

10. Qualified Names

Zamani supports qualified names for namespace and module resolution.

Conceptually:

name
qualified_name

A qualified name may represent:

module::symbol
package::module::symbol
domain::type
namespace::resource

The exact separator MUST remain consistent across the canonical grammar.

Qualified names MUST NOT encode physical topology.

For example:

hardware::gpu

may identify a language-level hardware capability or abstraction.

It MUST NOT inherently identify:

PCI address
device number
vendor-specific handle

---

11. Keywords

Keywords are reserved syntactic words.

The keyword set MUST be centrally defined.

No domain-specific grammar file may independently introduce a keyword that conflicts with the language-wide lexical authority.

New keywords SHOULD be introduced only when:

1. the construct cannot be expressed cleanly using existing syntax;
2. contextual interpretation is insufficient;
3. the keyword has stable language-level meaning;
4. compatibility impact has been assessed.

Where practical, extensible concepts SHOULD prefer existing structural mechanisms such as:

- qualified names;
- attributes;
- annotations;
- declarations;
- capabilities;
- effects;
- dialect namespaces.

This prevents keyword explosion.

---

12. Literals

The syntax model supports literals appropriate to the language type system.

At minimum, literal families include:

- integer;
- floating-point;
- Boolean;
- character;
- string;
- byte/string sequences;
- null/unit/absence values where defined;
- duration;
- size;
- symbolic literals where defined;
- domain-specific literals where justified.

Literal syntax MUST NOT impose artificial runtime size limits.

For example, the grammar MUST NOT encode:

maximum integer literal length = N
maximum tensor dimension = N
maximum string length = N

unless the limitation is an unavoidable lexical implementation invariant.

Any implementation bound MUST be represented as an implementation/resource limit rather than language semantics.

---

13. Numeric Literals

Numeric literals MUST support explicit and inferable representation where defined by the type system.

The lexical grammar MUST distinguish:

integer
floating-point
scientific notation
base-specific integer forms

where supported.

Numeric representation is separate from target representation.

For example:

let x = 1_000_000;

does not require a particular CPU integer width.

Likewise:

let x = 1.0;

does not necessarily require a particular hardware floating-point format.

Type analysis determines the semantic type.

Target lowering determines representation.

---

14. String and Text Syntax

Strings MUST be syntactically distinct from identifiers and numeric literals.

The language MAY support:

- escaped strings;
- raw strings;
- multiline strings;
- interpolation;
- byte strings.

Encoding and Unicode behavior MUST be defined by the lexical specification and MUST NOT depend on the host platform's default encoding.

String contents MUST be preserved without target-specific assumptions.

---

15. Attributes and Annotations

Attributes and annotations provide extensible source metadata.

Conceptually:

@name
@name(value)
@namespace::name(value)

Attributes MAY communicate:

- compiler hints;
- optimization hints;
- domain metadata;
- interoperability metadata;
- diagnostics;
- ABI requirements;
- effect declarations;
- capability requirements;
- resource preferences;
- experimental status;
- deprecation;
- verification intent.

Attributes MUST be classified by semantic authority.

An attribute that changes program semantics MUST have a documented semantic contract.

An optimization hint MUST NOT silently become a correctness requirement.

Unknown attributes MUST be handled according to the language's compatibility policy.

---

16. Declarations

Declarations introduce names and language-level entities.

The syntax model supports extensible declaration families including:

- modules;
- imports;
- exports;
- packages;
- functions;
- constants;
- variables;
- types;
- aliases;
- structures;
- records;
- unions;
- enums;
- traits;
- interfaces;
- implementations;
- classes where supported;
- effects;
- capabilities;
- resources;
- quantum entities;
- hardware entities;
- HDL entities;
- domain declarations;
- macros;
- dialect declarations.

A declaration MUST have one clear syntactic owner.

Two independent grammar subsystems MUST NOT define the same declaration form differently.

---

17. Declarations Versus Statements

Declarations introduce or configure entities.

Statements describe executable or control behavior.

Expressions compute or denote values.

The fundamental structural distinction is:

declaration
statement
expression
type
pattern

A construct MUST NOT be duplicated across categories unless there is a deliberate and documented language reason.

For example, a function declaration is not merely a statement that happens to create a function.

---

18. Blocks

A block is a syntactic sequence enclosed by delimiters.

Conceptually:

{
    statement*
}

A block MAY contain zero or more statements.

Blocks MUST NOT impose a fixed maximum number of statements.

Nested blocks MAY be arbitrarily deep subject to parser implementation resources.

The parser MUST provide deterministic behavior when implementation resource limits are exceeded.

---

19. Statements

Statement families include:

- declarations;
- assignments;
- expression statements;
- conditionals;
- loops;
- pattern matching;
- returns;
- breaks;
- continues;
- assertions;
- exception/control transfer constructs;
- effect handlers;
- concurrency constructs;
- quantum control constructs;
- hardware control constructs;
- domain-specific statements where explicitly specified.

A statement's syntactic validity MUST NOT imply execution.

---

20. Expressions

Expressions represent values, computations, references, and deferred computations.

Expression categories include:

- literals;
- names;
- qualified names;
- unary expressions;
- binary expressions;
- arithmetic;
- comparison;
- logical operations;
- bitwise operations;
- assignment expressions where supported;
- function calls;
- indexing;
- slicing;
- member access;
- ranges;
- conditional expressions;
- lambdas;
- closures;
- comprehensions;
- constructor expressions;
- casts/conversions where defined;
- compile-time expressions;
- domain expressions.

Expression precedence MUST be explicitly defined.

Ambiguous precedence MUST NOT be delegated to implementation-specific parser behavior.

---

21. Operator Precedence

Operator precedence is part of concrete syntax.

The canonical grammar MUST encode precedence deterministically.

Where ANTLR supports left-recursive precedence rules, those rules SHOULD be used rather than duplicated expression alternatives.

Every operator MUST have:

- precedence;
- associativity;
- operand category;
- syntactic spelling;
- semantic owner.

The syntax specification MUST NOT define an operator that has no semantic owner.

---

22. Assignment

Assignment syntax MUST remain distinct from equality and comparison.

The language MAY support:

simple assignment
compound assignment
destructuring assignment
pattern assignment

Assignment syntax does not itself determine memory semantics.

Ownership, mutability, borrowing, aliasing, synchronization, and storage semantics belong to the semantic/type/memory layers.

---

23. Function Syntax

Functions provide reusable computation.

A function declaration conceptually contains:

visibility
modifiers
name
generic parameters
parameters
return type
effects
contracts
body

The syntax MUST support functions whose implementation is:

- classical;
- quantum;
- hybrid;
- hardware-oriented;
- distributed;
- asynchronous;
- compile-time;
- foreign;
- domain-specific.

A function's syntax MUST NOT encode the number of hardware resources required unless that requirement is explicitly part of the function's declared semantic contract.

---

24. Generic Syntax

Generics provide parameterized source structures.

Generic parameters MAY represent:

- types;
- values;
- compile-time entities;
- dimensions;
- symbolic quantities;
- capabilities;
- domain parameters.

Generic syntax MUST NOT be used as an excuse to encode finite machine limits.

For example:

fn process<T, N>(data: Vector<T, N>) { ... }

must not imply that "N" is bounded by a grammar-defined maximum.

Any valid bound is determined by semantic constraints, compilation feasibility, or target resources.

---

25. Type Syntax

Types are syntactic descriptions of semantic value categories.

The type syntax MAY include:

- primitive types;
- named types;
- tuples;
- arrays;
- vectors;
- matrices;
- tensors;
- functions;
- generics;
- references;
- options;
- results;
- algebraic types;
- resource types;
- quantum types;
- hardware types;
- user-defined types.

The grammar MUST NOT encode implementation layout as type syntax unless layout is explicitly part of the language contract.

---

26. Type Parameters Versus Runtime Dimensions

Zamani MUST distinguish:

compile-time structural parameter
runtime value
resource requirement
physical dimension

For example:

Vector<T, N>

may express a parameterized mathematical structure.

It does not necessarily mean:

allocate N physical registers now

Similarly:

qubit[N]

describes a source-level quantum collection.

Actual physical allocation belongs to semantic lowering, resource analysis, mapping, and runtime.

---

27. Arrays, Vectors, Matrices, and Tensors

Syntax MAY express multidimensional data structures.

Dimensions MUST be represented using language-level expressions or parameters rather than grammar-generated fixed limits.

The grammar MUST NOT enumerate allowed dimensions:

1
2
3
4
8
16
32
64

as a substitute for a scalable dimension model.

A dimension may be:

- literal;
- symbolic;
- generic;
- runtime-derived;
- inferred;
- constrained.

The type and semantic systems determine whether the dimension is valid.

---

28. Pattern Syntax

Patterns support structural matching and destructuring.

Patterns MAY include:

- literals;
- names;
- wildcards;
- tuples;
- arrays;
- lists;
- variants;
- type patterns;
- ranges;
- nested patterns;
- guards.

Pattern syntax MUST be deterministic and non-ambiguous.

Pattern matching semantics belong to the type/semantic layer.

---

29. Control Flow

Control-flow syntax includes:

if
else
while
do
for
match
return
break
continue

and future extensible control constructs.

Control-flow syntax MUST remain domain-neutral where possible.

A quantum conditional is not necessarily a different fundamental control-flow syntax.

Instead, domain semantics may attach to an existing control-flow construct where that preserves clarity and correctness.

---

30. Compile-Time Syntax

Zamani may distinguish compile-time computation from runtime computation.

Compile-time constructs MUST be syntactically explicit.

The language MAY support:

- compile-time expressions;
- compile-time functions;
- specialization;
- generated declarations;
- metaprogramming;
- compile-time assertions.

Compile-time execution MUST NOT implicitly perform machine discovery merely because compilation is occurring.

Target discovery belongs to target-aware compilation.

---

31. Modules

Modules provide namespace and compilation boundaries.

Module syntax MUST support:

- declarations;
- imports;
- exports;
- nested namespaces where supported;
- visibility;
- versioning metadata;
- dependencies.

Modules MUST NOT require a particular filesystem layout unless the package/module specification explicitly defines one.

---

32. Packages

Package syntax may identify distributable language artifacts.

A package declaration MAY contain:

- name;
- version;
- dependencies;
- repository metadata;
- licensing metadata;
- compatibility information.

Package metadata MUST NOT become executable language semantics.

Dependency resolution belongs to the package/build system.

---

33. Effects

Effects describe externally observable or semantically relevant behavior.

Effect syntax MAY describe:

- I/O;
- allocation;
- mutation;
- concurrency;
- networking;
- hardware interaction;
- quantum operations;
- distributed execution;
- security-sensitive operations;
- custom effects.

Effects MUST remain distinct from capabilities.

For example:

effect: quantum

describes behavior.

A capability:

requires: quantum

describes what an execution environment must provide.

---

34. Capabilities

Capability syntax describes required or available computational abilities.

Examples include conceptual capabilities such as:

quantum
classical
gpu
fpga
network
distributed
accelerator
simd
secure_execution

The exact capability namespace is extensible.

Capabilities MUST be semantic abstractions.

A capability MUST NOT inherently identify:

- vendor;
- device serial number;
- physical address;
- fixed topology;
- fixed number of resources.

Capability satisfaction belongs to semantic/target analysis.

---

35. Requirements

A requirement expresses something necessary for correctness or valid execution.

Conceptually:

requires <capability-or-condition>;

Requirements MUST be declarative.

They MUST NOT directly execute resource allocation.

Requirements MUST be composable.

A program may require multiple capabilities without implying a particular implementation.

---

36. Constraints

Constraints express conditions that must hold.

Examples include:

latency < bound
memory >= requirement
fidelity >= requirement
precision >= requirement
energy <= budget

The syntax MUST permit expressions rather than fixed enumerations.

A constraint is not automatically a target configuration.

Constraint solving belongs downstream.

---

37. Preferences

Preferences express desirable but negotiable properties.

Examples:

prefer low_latency;
prefer low_energy;
prefer locality;
prefer reliability;

A preference MUST NOT silently become a correctness requirement.

The compiler MAY ignore or reinterpret preferences when necessary to preserve correctness.

---

38. Hints

Hints provide non-authoritative optimization information.

Examples include:

hint locality;
hint parallel;
hint vectorize;
hint cache;
hint entangle;

Hints MUST NOT alter program semantics.

An implementation MAY ignore a hint.

If ignoring a hint can change correctness, it is not a hint and MUST be represented as a requirement or constraint instead.

---

39. Resource Syntax

Resource syntax describes abstract computational resources.

Resources MAY include:

- memory;
- processors;
- accelerators;
- qubits;
- logical qubits;
- physical qubits;
- communication channels;
- storage;
- bandwidth;
- timing capacity;
- energy budgets;
- execution slots.

Resource declarations MUST be abstract.

For example:

requires qubits >= expression;

may be valid.

But:

requires exactly 64 qubits;

must not be imposed merely because a current backend has 64 qubits.

If exact resource count is genuinely part of program semantics, it may be expressed explicitly by the programmer.

---

40. No Grammar-Level Resource Ceilings

The grammar MUST NOT define language-level ceilings such as:

MAX_QUBITS
MAX_CORES
MAX_THREADS
MAX_DEVICES
MAX_NODES
MAX_MEMORY
MAX_TENSOR_RANK
MAX_CIRCUIT_DEPTH
MAX_PROGRAM_SIZE

A parser implementation may have operational safeguards against denial-of-service or host-resource exhaustion.

Such safeguards MUST be:

1. implementation configuration;
2. explicit;
3. externally configurable;
4. reported diagnostically;
5. separate from language semantics.

A resource limit is not a language grammar limit.

---

41. Quantum Syntax

Quantum constructs are part of the Zamani language.

Quantum syntax MUST support an abstract quantum programming model without assuming a fixed physical machine.

Conceptual quantum syntax includes:

qubit
qubit collections
logical qubits
physical references
quantum registers
quantum operations
gates
parameterized operations
controlled operations
adjoints/inverses
measurement
reset
observables
circuits
dynamic circuits
mid-circuit measurement
classical conditions
quantum-classical interaction

The exact concrete forms are defined by the canonical grammar and quantum-specific syntax specifications.

---

42. Quantum Resource Identity

A source-level quantum identifier is not automatically a physical qubit identifier.

For example:

let q = qubit();

introduces an abstract quantum resource.

It does not imply:

physical qubit 0

Likewise:

let register = qubits(n);

does not imply a fixed hardware register.

Physical placement is downstream.

This separation is essential to POCO-REAF.

---

43. Logical and Physical Qubits

The syntax MAY distinguish:

logical qubit
physical qubit reference

when the programmer intentionally requires such a distinction.

A logical qubit represents an abstract computational degree of freedom.

A physical qubit represents a hardware-level realization.

The grammar MUST NOT silently convert one into the other.

The mapping belongs to:

routing
scheduling
QEC
hardware abstraction
target compilation

as appropriate.

---

44. Quantum Operations

Quantum operations are syntactic invocations of quantum behavior.

The syntax MAY express:

H(q)
X(q)
CNOT(control, target)
measure(q)
reset(q)
U(theta, phi, lambda, q)

or equivalent Zamani forms.

The grammar MUST NOT restrict gate arity through fixed alternatives such as:

q[0], q[1]

unless that exact syntax is the user's source expression.

The grammar describes references.

Semantic analysis validates operand counts and types.

---

45. Parameterized Quantum Operations

Quantum operations may contain symbolic parameters.

Parameters may be:

- literals;
- expressions;
- runtime values where supported;
- compile-time values;
- symbolic variables.

The grammar MUST not require parameters to be statically numeric unless the operation's semantic definition requires that.

---

46. Controlled and Composite Quantum Operations

Quantum syntax MUST support composition.

Conceptually:

controlled(operation)
adjoint(operation)
inverse(operation)
repeat(operation, expression)
compose(operation, ...)

where such forms are part of the language.

The syntax should permit arbitrarily large composite operations subject to implementation resources.

No grammar-level maximum depth is permitted.

---

47. Measurement

Measurement syntax MUST distinguish measurement from ordinary function calls where the language requires special semantic handling.

Measurement may produce classical information.

The syntax MUST support:

measurement result
measurement into classical storage
measurement condition
mid-circuit measurement

where supported.

Measurement semantics MUST remain separate from physical readout implementation.

---

48. Quantum-Classical Interaction

Zamani MUST allow classical and quantum computation to interact.

The syntax may express:

quantum operation
measure
classical condition
classical computation
quantum operation

within one source program.

The syntax MUST NOT force developers to split hybrid programs into unrelated languages.

However, semantic validation MUST enforce the rules governing:

- quantum state use;
- measurement;
- classical dependencies;
- control flow;
- synchronization;
- effects;
- resource ownership.

---

49. Dynamic Quantum Circuits

Dynamic circuits may contain:

measurement
classical branching
conditional quantum operations
reset
loops where semantically permitted

The grammar MUST express the source structure.

It MUST NOT require a particular backend execution model.

A backend may lower dynamic control into:

- native dynamic instructions;
- classical host control;
- compiled branching;
- deferred measurement;
- other semantically valid representations.

---

50. Quantum Error-Correction Syntax

Zamani MAY express QEC intent.

Examples of concepts include:

logical
fault_tolerant
error_correction
code
syndrome
decoder

where explicitly specified by the quantum language.

However, syntax MUST NOT become a duplicate QEC implementation model.

The architectural boundary is:

source-level QEC intent
        ↓
AST
        ↓
semantic validation
        ↓
quantum::ir
        ↓
QEC subsystem

QEC algorithms remain owned by the QEC subsystem.

---

51. ZQN and Noise Syntax

Zamani MAY provide declarative syntax for noise-aware computation.

Examples include:

noise_aware
noise_requirement
noise_constraint
noise_model

where supported.

The grammar MUST NOT duplicate ZQN's internal representation of:

- channels;
- faults;
- correlated faults;
- leakage;
- loss;
- erasure;
- calibration;
- execution semantics.

Source syntax describes intent.

ZQN provides downstream noise semantics.

---

52. Classical Syntax

Classical computing is a first-class part of Zamani.

The syntax supports:

- scalar computation;
- structured data;
- functions;
- generics;
- control flow;
- memory;
- concurrency;
- parallelism;
- numerical computing;
- symbolic computation;
- vectors;
- matrices;
- tensors;
- accelerator computation.

Classical constructs MUST remain independent of quantum constructs.

Quantum syntax extends the common language; it does not replace it.

---

53. HDL Syntax

HDL is also a first-class source domain.

HDL syntax MAY describe:

- modules;
- ports;
- signals;
- wires;
- registers;
- clocks;
- timing;
- combinational logic;
- sequential logic;
- processes;
- state machines;
- memories;
- pipelines;
- hardware interfaces;
- parameters;
- generics.

HDL syntax MUST describe hardware behavior and structure without requiring one physical implementation.

---

54. Hardware Targets

Target declarations describe intended execution environments.

A target MAY specify:

architecture
capability
ABI
instruction set
device class
accelerator class
execution model

Target syntax MUST NOT become part of portable program semantics unless the programmer explicitly declares target dependence.

A target-specific declaration is therefore an explicit portability boundary.

---

55. Target-Specific Versus Portable Syntax

Zamani MUST distinguish:

portable source
target-constrained source
target-specific source

Portable source expresses semantic requirements.

Target-constrained source adds explicit constraints.

Target-specific source intentionally binds itself to a particular environment.

The compiler MUST preserve this distinction in the AST/semantic model.

---

56. Hardware Resource Abstraction

Hardware syntax MUST support abstract resources.

For example:

requires accelerator;
requires memory >= amount;
requires quantum;
requires parallelism >= degree;

The syntax MUST NOT require:

use GPU 0
use QPU 3
use core 7
use node 12

unless the programmer explicitly requests target-specific behavior.

Even when explicit identifiers exist, they belong to a target/deployment namespace rather than the portable language core.

---

57. Scheduling Syntax

Scheduling-related syntax MAY express:

- ordering requirements;
- deadlines;
- latency constraints;
- synchronization;
- priorities;
- timing requirements;
- preferences.

The grammar MUST NOT implement scheduling.

Scheduling belongs to the scheduling subsystem.

The syntax provides declarative information consumed by scheduling.

This maintains:

source intent
    ↓
IR
    ↓
scheduler

rather than:

source syntax
    ↓
hard-coded schedule

---

58. Routing and Placement

Source syntax MAY express:

- locality;
- placement requirements;
- affinity;
- co-location;
- separation;
- topology requirements.

It MUST NOT directly implement routing algorithms.

Routing belongs to the routing/mapping layer.

For quantum programs:

logical qubit
      ↓
quantum::ir
      ↓
routing
      ↓
physical qubit

For distributed systems:

logical task
      ↓
IR
      ↓
placement
      ↓
physical node

---

59. Optimization Syntax

Optimization directives MAY express:

- optimization preferences;
- optimization goals;
- acceptable trade-offs;
- approximation requirements;
- performance hints.

The grammar MUST NOT encode optimization algorithms.

Optimization belongs to the optimization subsystem.

An optimization directive MUST NOT change semantics unless explicitly specified as a semantic transformation.

---

60. Memory Syntax

Memory-related syntax MAY express:

- ownership;
- borrowing;
- references;
- allocation;
- deallocation;
- shared memory;
- distributed memory;
- lifetime constraints.

Memory syntax describes language semantics.

It MUST NOT assume a fixed physical memory capacity.

Physical allocation belongs to runtime/target lowering.

---

61. Concurrency Syntax

Concurrency syntax MAY express:

- tasks;
- asynchronous operations;
- futures;
- actors;
- channels;
- synchronization;
- parallel loops;
- data parallelism;
- task parallelism;
- cancellation.

The syntax MUST NOT hard-code:

maximum threads = N
maximum tasks = N

Concurrency feasibility is determined by runtime and resource systems.

---

62. Distributed Syntax

Distributed constructs MAY describe:

- nodes;
- services;
- messages;
- communication;
- remote execution;
- replication;
- consistency;
- placement;
- distributed fault tolerance.

A distributed declaration does not imply a fixed cluster size.

The source program MUST remain scalable across different deployment sizes where semantics permit.

---

63. Networking Syntax

Networking syntax may express:

- endpoints;
- protocols;
- channels;
- messages;
- services;
- communication contracts.

Network addresses MAY be represented explicitly when they are part of program semantics.

However, portable networking SHOULD prefer:

service names
capabilities
logical endpoints
deployment bindings

over hard-coded physical addresses.

---

64. Security Syntax

Security syntax may express:

- permissions;
- capabilities;
- identities;
- cryptographic requirements;
- trust;
- privacy;
- isolation;
- secure execution.

Security declarations MUST be explicit.

A security requirement MUST NOT silently degrade into an optimization hint.

---

65. Interoperability Syntax

Zamani MAY interoperate with:

- C;
- C++;
- Rust;
- Python;
- OpenQASM;
- Verilog;
- other explicitly supported interfaces.

Interoperability syntax MUST identify external boundaries clearly.

External-language syntax MUST NOT redefine Zamani's core syntax.

The pipeline is:

Zamani syntax
      ↓
AST
      ↓
semantic boundary
      ↓
FFI / ABI / foreign representation

---

66. Foreign Functions

Foreign function declarations may describe:

- external name;
- ABI;
- calling convention;
- parameter types;
- return type;
- effects;
- safety requirements.

Foreign functions MUST be semantically marked as external boundaries.

A foreign function MUST NOT be assumed portable merely because its declaration is syntactically portable.

---

67. Macros

Macros provide source transformation facilities.

Macro syntax MUST distinguish:

macro declaration
macro invocation
macro expansion

Macro expansion MUST have deterministic hygiene and source mapping rules.

Macros MUST NOT bypass semantic validation.

Expanded source MUST pass through the appropriate semantic pipeline.

Macros MUST NOT create a hidden second language authority.

---

68. Metaprogramming

Metaprogramming MAY generate:

- declarations;
- types;
- functions;
- expressions;
- domain-specific constructs.

Generated syntax MUST eventually conform to the canonical syntax/AST model.

Metaprogramming MUST NOT introduce undocumented syntax that cannot be represented in the language's compatibility model.

---

69. Dialects

Zamani supports extensible dialects.

A dialect is a controlled extension of language syntax and semantics.

Dialect syntax MUST have:

- namespace;
- identity;
- version;
- lifecycle;
- compatibility information;
- ownership;
- registration mechanism.

A dialect MUST NOT silently redefine core Zamani syntax.

Dialect constructs SHOULD be namespaced where collision is possible.

Experimental dialects MUST be explicitly marked experimental.

---

70. Domain Composition

Zamani's syntax MUST permit domains to compose.

Examples include:

classical + quantum
classical + HDL
quantum + HDL
quantum + hardware
quantum + distributed
AI + quantum
AI + hardware
classical + quantum + distributed
classical + quantum + HDL + hardware

Domain composition MUST NOT require multiple independent parsers for one source program.

One Zamani source program has one language syntax.

Domain-specific semantics are attached after parsing.

---

71. Cross-Domain Type Composition

Types from different domains may interact when explicitly supported.

Examples include:

classical value
quantum measurement result
hardware resource handle
tensor
distributed reference

The syntax MUST represent these as part of the common type language where possible.

The semantic layer determines whether composition is valid.

---

72. Cross-Domain Effects

Cross-domain programs may have multiple effects.

For example:

quantum
io
network
hardware
distributed

The syntax MUST permit explicit effect composition.

Effects MUST NOT be inferred solely from target hardware.

They describe program behavior.

---

73. Resource Expressions

Resource expressions MUST be first-class expressions where the resource model requires them.

A resource quantity MAY be:

- literal;
- symbolic;
- parameterized;
- inferred;
- runtime-derived;
- target-derived.

For example:

requires qubits >= logical_qubits;

is structurally different from:

use physical_qubit(0);

The first expresses a requirement.

The second expresses an explicit physical realization.

---

74. No Implicit Hardware Binding

The following transformation is prohibited:

source qubit
    ↓
implicitly select physical q[0]

The compiler MUST NOT insert physical identities into the source language semantics merely because a target has numbered resources.

Hardware binding is a downstream decision.

This is one of the core requirements of POCO-REAF.

---

75. Syntax and Canonical AST

Every canonical syntax construct MUST map to exactly one documented AST ownership model.

The architecture is:

source
  ↓
tokens
  ↓
parse tree
  ↓
AST

The AST MUST preserve enough structure to distinguish:

- declarations;
- expressions;
- statements;
- types;
- effects;
- requirements;
- constraints;
- capabilities;
- domain constructs;
- source locations.

The AST MUST NOT require reparsing source text to recover semantics.

---

76. Grammar and AST Ownership

The grammar owns:

- concrete structure.

The AST owns:

- structured source representation.

Semantic analysis owns:

- meaning.

IR owns:

- canonical semantic representation.

The grammar MUST NOT create an AST-like second representation hidden inside parser actions.

---

77. Grammar and IR Ownership

The grammar MUST NOT create canonical IR nodes.

The pipeline is:

grammar
   ↓
AST
   ↓
semantic analysis
   ↓
IR

For quantum:

Zamani syntax
   ↓
AST
   ↓
quantum semantic analysis
   ↓
quantum::ir

For classical computation:

Zamani syntax
   ↓
AST
   ↓
semantic analysis
   ↓
classical IR

The grammar MUST NOT directly instantiate quantum or classical IR semantics.

---

78. Canonical Quantum IR Integration

"quantum::ir" remains the canonical semantic boundary for quantum computation.

The syntax model therefore guarantees:

quantum source
      ↓
AST
      ↓
semantic lowering
      ↓
quantum::ir

Downstream quantum components consume that representation.

They MUST NOT parse source syntax themselves.

They MUST NOT create alternate source-level gate/qubit models solely to bypass the canonical IR.

---

79. QEC Integration

The grammar may expose QEC intent.

After semantic analysis:

QEC intent
    ↓
quantum::ir / semantic representation
    ↓
QEC

QEC owns:

- syndrome processing;
- decoding;
- correction;
- code-specific algorithms;
- QEC resource handling.

The grammar does not own these algorithms.

The repository's QEC implementation already follows explicit ownership boundaries and central resource-limit contracts; the syntax layer must preserve the same separation.

---

80. ZQN Integration

The grammar may express declarative noise requirements.

After lowering:

source
 ↓
AST
 ↓
quantum::ir
 ↓
ZQN

ZQN owns noise execution semantics.

The grammar MUST NOT duplicate ZQN data structures.

---

81. Scheduling Integration

Scheduling syntax expresses constraints and preferences.

Scheduling consumes:

semantic program / IR
+
resource model
+
hardware capabilities
+
timing information

The grammar does not own:

- dependency scheduling algorithms;
- ASAP;
- ALAP;
- list scheduling;
- RCPSP;
- timing grid discovery;
- hardware resource allocation.

The source language only expresses relevant intent.

---

82. Hardware Integration

Hardware syntax describes:

capability
requirement
constraint
preference
target
interface

Hardware abstraction owns:

- device discovery;
- calibration;
- actual resource state;
- physical topology;
- device identifiers;
- backend capabilities.

The grammar MUST NOT perform hardware discovery.

---

83. Runtime Integration

Runtime execution consumes compiled/semantic artifacts.

The runtime MUST NOT need to know how a source construct was spelled.

Therefore:

source syntax
     ↓
AST
     ↓
semantic model
     ↓
IR
     ↓
compiled/runtime artifact

Runtime behavior MUST be independent of concrete syntax where semantics are equivalent.

---

84. Syntax Equivalence

Two syntactically different source forms MAY have the same semantics.

For example, future syntactic sugar may lower to the same AST or semantic operation.

The language MUST distinguish:

syntactic identity

from:

semantic identity

Compatibility rules SHOULD be defined in terms of semantic preservation wherever practical.

---

85. Determinism

For identical source text, language version, dialect set, and parser configuration, parsing MUST be deterministic.

The parser MUST NOT depend on:

- thread scheduling;
- hash-map iteration order;
- machine topology;
- available hardware;
- current time;
- random values;
- network responses.

Deterministic syntax is essential for:

- reproducible builds;
- caching;
- provenance;
- verification;
- debugging;
- distributed compilation.

---

86. Incremental Parsing

The syntax model SHOULD permit incremental parsing.

A source edit SHOULD require reparsing only the affected syntactic regions where the implementation supports incremental parsing.

Incremental parsing MUST preserve the same AST semantics as full parsing.

No incremental-only syntax is permitted.

---

87. Error Recovery

Parser error recovery is an implementation concern but MUST follow a stable diagnostic model.

The parser SHOULD recover sufficiently to report multiple independent syntax errors.

Recovery MUST NOT fabricate valid semantic nodes that can silently reach code generation.

Recovered nodes MUST be marked as invalid or incomplete.

---

88. Syntax Errors

Syntax errors MUST identify:

- source;
- location;
- unexpected token;
- expected syntactic category where determinable;
- diagnostic code;
- severity.

Diagnostics MUST be deterministic.

A syntax error MUST NOT expose internal parser implementation details unnecessarily.

---

89. Resource Exhaustion During Parsing

The language has no semantic maximum program size.

However, a parser implementation may encounter host limits.

Examples:

memory exhaustion
stack exhaustion
token buffer exhaustion
time budget
diagnostic budget

Such limits are implementation safeguards.

They MUST NOT be represented as language-level syntax restrictions.

Where possible, implementations SHOULD use bounded iterative algorithms rather than unnecessary recursion for attacker-controlled nesting.

---

90. Scalability

The syntax model is intentionally unbounded at the language level.

"Unbounded" means:

«The language does not define an arbitrary finite maximum for a construct merely for convenience of one implementation.»

It does not mean:

«Every implementation can execute an infinitely large program.»

Actual limits arise from:

available memory
available storage
available compute
parser implementation
compiler implementation
target capabilities
runtime resources
physical reality

This distinction is mandatory.

---

91. Atom-to-Everywhere Model

Zamani MUST support the conceptual scale spectrum:

single operation
    ↓
tiny program
    ↓
embedded computation
    ↓
single processor
    ↓
multicore
    ↓
accelerator
    ↓
GPU
    ↓
FPGA
    ↓
ASIC
    ↓
quantum processor
    ↓
heterogeneous system
    ↓
cluster
    ↓
supercomputer
    ↓
distributed system
    ↓
cloud
    ↓
future computational architecture

The source syntax MUST NOT require different fundamental language semantics for each scale.

---

92. POCO-REAF

POCO-REAF means:

Program Once
Compile Once
Run Everywhere
Anywhere
Forever

The syntax model supports POCO-REAF by ensuring:

1. source semantics are machine-independent where possible;
2. resource requirements are explicit;
3. target-specific constraints are explicit;
4. hardware identity is not silently embedded;
5. domain semantics are canonical;
6. IR boundaries are stable;
7. versioning is explicit;
8. downstream compilation may select different realizations.

POCO-REAF does not promise impossible execution.

If a program requires a capability absent from a target, execution MUST fail through a capability/resource diagnostic rather than requiring source rewriting.

---

93. Compile Once

"Compile Once" does not mean that one binary can execute natively on every architecture.

It means that the semantic program representation can remain stable while target realization changes.

The architecture SHOULD permit:

Zamani source
      ↓
canonical semantic representation
      ↓
portable artifact
      ↓
target realization

For example, the same quantum program may ultimately be mapped differently for different QPUs without changing its source semantics.

---

94. Target Adaptation

Target adaptation may change:

- layout;
- instruction selection;
- gate decomposition;
- scheduling;
- routing;
- memory strategy;
- vectorization;
- accelerator assignment;
- distributed placement;
- error-correction strategy.

Target adaptation MUST preserve the source semantics unless the programmer explicitly selected a non-equivalent approximation or target-specific behavior.

---

95. Approximation Syntax

Approximation MAY be explicitly declared.

A construct such as:

approximate

must define:

- acceptable error;
- quality requirement;
- semantic trade-off;
- verification requirements.

Approximation MUST NOT be introduced silently by hardware adaptation.

---

96. Versioning

Syntax is versioned through the language compatibility system.

A source version identifies the language contract under which the source is interpreted.

Versioning MUST distinguish:

language version
grammar version
AST schema version
IR version
backend version
runtime version

These versions MUST NOT be conflated.

---

97. Backward Compatibility

Compatible syntax evolution SHOULD preserve existing valid programs.

Breaking changes MUST be explicitly versioned.

A removed syntax construct MUST have:

- deprecation period where practical;
- migration guidance;
- compatibility classification;
- diagnostics.

The compatibility specification is the authority for lifecycle policy.

---

98. Reserved Syntax

Zamani MUST reserve namespace for future evolution.

Reserved syntax MUST NOT be accidentally consumed by unrelated dialects.

Reserved identifiers and punctuation SHOULD be minimized.

The language SHOULD prefer extensible structures over large permanently reserved keyword lists.

---

99. Experimental Syntax

Experimental syntax MUST be explicitly marked.

Experimental constructs:

- are not automatically stable;
- require lifecycle metadata;
- MUST NOT silently alter stable syntax;
- SHOULD be isolated through dialect/feature mechanisms.

An experimental feature MUST NOT create a second grammar authority.

---

100. Deprecated Syntax

Deprecated syntax remains parseable during its supported compatibility window.

The parser SHOULD produce a structured deprecation diagnostic.

Deprecation MUST NOT change semantics before the removal version.

---

101. Removed Syntax

Removed syntax MUST no longer be accepted in the corresponding language version.

Compatibility tooling MAY parse historical versions.

Historical compatibility MUST NOT contaminate the current grammar with ambiguous alternatives.

---

102. Dialect Isolation

A dialect MUST declare:

dialect identity
version
namespace
feature set
compatibility status

A dialect MUST NOT override the meaning of a core Zamani construct without an explicit language-versioned mechanism.

---

103. Syntax Ownership Rule

Every syntax construct MUST have one owner.

The owner is responsible for:

- grammar;
- documentation;
- AST mapping;
- semantic contract;
- tests;
- compatibility;
- diagnostics.

Downstream systems consume the construct but do not redefine its syntax.

---

104. No Duplicate Domain Grammars

The repository MUST NOT contain:

quantum grammar A
quantum grammar B
hardware grammar A
hardware grammar B

where both claim authority over the same syntax.

Domain grammar fragments may be physically separated for maintainability, but their composition MUST produce one canonical grammar.

---

105. Grammar Composition

If "Zamani.g4" is decomposed into imported grammar fragments, composition MUST remain deterministic.

Each fragment MUST have:

- documented ownership;
- explicit imports;
- no duplicate rule ownership;
- no conflicting token definitions;
- no hidden semantic actions;
- stable integration tests.

The composed grammar remains the canonical concrete syntax.

---

106. ANTLR Integration

The canonical grammar MUST conform to ANTLR4 grammar rules.

ANTLR grammar files MUST use:

- lowercase parser rule names;
- uppercase lexer rule names;
- explicit grammar declarations;
- explicit imports where used;
- deterministic precedence;
- no target-language-specific semantic actions unless absolutely necessary.

ANTLR-specific implementation details MUST NOT redefine Zamani semantics.

ANTLR itself permits combined grammars and separate lexer/parser grammars; Zamani may choose either structure, but the repository MUST retain one authoritative composed language grammar.

---

107. Rust Frontend Integration

The reference Rust frontend MUST remain semantically equivalent to the canonical grammar.

The implementation target is:

Rust 1.97 / Rust 1.97.1

Rust implementation MUST use safe Rust only.

The project MUST enforce:

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

or an equivalent workspace-level policy.

The grammar specification itself does not depend on Rust internals.

Rust is the implementation language, not the definition of Zamani semantics.

---

108. No "unsafe" Requirement

No Rust implementation of the Zamani frontend may require:

unsafe

for:

- lexer operation;
- parser operation;
- AST construction;
- syntax diagnostics;
- syntax validation;
- source formatting;
- syntax serialization.

If a third-party dependency internally uses unsafe code, the dependency policy must separately determine whether it is acceptable; Zamani's own grammar/frontend implementation MUST remain safe Rust.

---

109. Parser/AST Contract

The parser MUST output an AST conforming to the repository's AST ownership model.

The contract is:

Lexer
  ↓
Tokens
  ↓
Parser
  ↓
AST

The parser MUST NOT:

- perform hardware discovery;
- schedule operations;
- select a physical qubit;
- allocate runtime resources;
- perform QEC;
- perform noise simulation;
- optimize circuits;
- route hardware;
- execute code.

Those are downstream responsibilities.

---

110. Semantic Analysis Contract

Semantic analysis consumes the AST.

It determines:

- name resolution;
- type correctness;
- effect correctness;
- capability requirements;
- resource constraints;
- domain legality;
- control-flow validity;
- ownership rules;
- interoperability correctness.

Semantic analysis MUST NOT depend on a particular hardware instance unless performing explicit target validation.

---

111. Classical IR Contract

Classical constructs that lower to classical IR MUST do so through semantic lowering.

The grammar does not directly construct classical IR.

The resulting IR MUST preserve source semantics.

---

112. Quantum IR Contract

Quantum constructs lower through semantic analysis into "quantum::ir".

The grammar MUST NOT define a competing gate or qubit representation for downstream compilation.

The repository's canonical quantum boundary is therefore:

Zamani quantum syntax
        ↓
AST
        ↓
semantic analysis
        ↓
quantum::ir

This keeps optimization, QEC, ZQN, routing, scheduling, hardware, and runtime layers independent of source syntax.

---

113. Hardware and Runtime Contract

Hardware and runtime layers receive target-neutral or target-aware artifacts as appropriate.

They MUST NOT parse Zamani source to discover semantics.

If source-level metadata is required at runtime, it MUST be represented explicitly in a stable artifact format.

---

114. Provenance

Syntax-derived artifacts SHOULD preserve provenance.

Provenance may include:

- source identity;
- source version;
- language version;
- dialect versions;
- AST identity;
- semantic artifact identity;
- compiler identity;
- compilation configuration.

Provenance MUST NOT contain secrets unless explicitly designed for secure handling.

---

115. Deterministic Source Identity

Equivalent source inputs under the same language configuration SHOULD produce deterministic syntax representations.

Where hashing is used, canonical serialization MUST be defined.

Hashing is not itself syntax.

---

116. Security

The syntax model MUST be safe against malformed source input.

Implementations SHOULD protect against:

- pathological nesting;
- pathological token counts;
- enormous literals;
- pathological macro expansion;
- exponential parse behavior;
- uncontrolled diagnostics;
- memory exhaustion.

Security controls MUST be implementation limits rather than arbitrary language semantic restrictions.

---

117. Parser Resource Limits

Production frontends MAY expose configurable limits such as:

maximum source bytes
maximum token count
maximum nesting depth
maximum diagnostic count
maximum macro expansion budget
maximum parser work

These MUST be externally configurable.

They MUST NOT be encoded as language-level constants such as:

MAX_QUBITS
MAX_PROGRAM_SIZE

A parser limit MUST never change the meaning of a program that is successfully parsed.

---

118. Diagnostics

Every syntax error MUST have a stable diagnostic category/code.

Diagnostics SHOULD identify:

syntax
lexical
version
dialect
deprecated syntax
unsupported syntax
resource-limited parsing

Diagnostics MUST remain independent of target hardware.

A program cannot become syntactically invalid merely because the compiler is running on a smaller machine.

---

119. Testing Contract

Every syntax feature MUST have:

Positive tests

Valid examples.

Negative tests

Invalid examples.

Boundary tests

Smallest and largest practical structures.

Cross-domain tests

Combinations of language domains.

Compatibility tests

Historical and versioned syntax.

Determinism tests

Repeated parsing produces identical structural output.

Round-trip tests

Where serialization exists:

source
 ↓
parse
 ↓
AST
 ↓
canonical print
 ↓
parse

must preserve semantics.

---

120. Scalability Tests

The grammar test suite MUST verify absence of artificial semantic ceilings.

Test families SHOULD include:

one qubit
many qubits
parameterized qubits
large quantum registers
deep circuits
large classical arrays
large tensors
many declarations
many modules
many distributed nodes
large HDL structures
large dependency graphs
large expressions
large generated programs

The tests must distinguish:

language limit
implementation limit
test harness limit
host resource exhaustion

---

121. Cross-Domain Tests

At minimum, syntax conformance MUST test:

classical
quantum
hybrid
HDL
hardware
distributed
AI
data
networking
security
accelerators

and combinations such as:

classical + quantum
classical + HDL
quantum + HDL
quantum + hardware
quantum + distributed
AI + quantum
AI + hardware
classical + quantum + distributed
classical + quantum + HDL + hardware

---

122. Syntax Fuzzing

The production frontend SHOULD include syntax fuzzing.

Fuzzing MUST verify:

- no parser crashes;
- no panics for ordinary malformed input;
- deterministic diagnostics;
- no memory unsafety;
- no infinite loops;
- bounded recovery behavior.

The Rust frontend MUST remain safe Rust.

---

123. Grammar Ambiguity

Ambiguity MUST be treated as a defect unless explicitly intentional and resolved deterministically.

Potential ambiguity sources include:

- identifiers versus keywords;
- generic arguments versus comparison operators;
- annotations versus expressions;
- dialect constructs;
- macros;
- nested type syntax;
- pattern syntax;
- quantum control syntax.

Every intentional ambiguity MUST have a deterministic resolution rule.

---

124. Lexical/Parser Boundary

The lexer MUST recognize lexical units.

The parser MUST recognize syntactic relationships.

The lexer SHOULD NOT encode context-sensitive semantic decisions.

The parser SHOULD NOT reimplement lexical tokenization manually.

This separation improves:

- determinism;
- diagnostics;
- testing;
- maintainability;
- tooling.

---

125. Semantic Predicates

Semantic predicates inside the concrete grammar SHOULD be avoided.

They create coupling between:

syntax
semantic state
implementation language

If a construct can be parsed structurally and validated semantically later, semantic validation MUST occur downstream.

---

126. Parser Actions

Target-language-specific parser actions SHOULD be minimized or prohibited.

The preferred model is:

pure grammar
   ↓
parse tree
   ↓
AST builder

This permits:

- deterministic generation;
- multiple tooling targets;
- easier testing;
- easier grammar evolution.

---

127. Formatting

Formatting is not part of program semantics.

A formatter MUST be able to canonicalize syntax without changing meaning.

The canonical formatter SHOULD produce:

- stable whitespace;
- stable indentation;
- stable import ordering where permitted;
- stable attribute formatting;
- stable expression formatting.

Formatting MUST NOT change resource requirements.

---

128. Pretty-Printing

A pretty-printer MUST preserve semantic equivalence.

If the AST contains syntax that cannot be represented by the current canonical printer, that is a tooling defect.

The printer MUST NOT silently omit:

- attributes;
- type parameters;
- resource requirements;
- quantum operations;
- hardware constraints;
- effect declarations.

---

129. Serialization

AST serialization, if supported, is distinct from source syntax.

The serialized AST format MUST have its own schema/version.

Source syntax MUST NOT be reconstructed from serialized implementation internals.

---

130. Source-to-Source Transformations

Source transformations MUST preserve:

- semantics;
- source locations where possible;
- attributes;
- effects;
- capabilities;
- resource declarations;
- domain constructs.

A transformation that changes semantics MUST declare that fact.

---

131. Syntax and Optimization

Optimization may transform semantic representations.

It MUST NOT require source syntax changes for ordinary target adaptation.

For example:

Zamani source

must not need to change because one target requires a different gate decomposition.

The optimizer operates after semantic lowering.

---

132. Syntax and Routing

Likewise:

logical quantum program

does not need to be rewritten because hardware topology changes.

Routing operates downstream.

---

133. Syntax and Scheduling

Likewise:

source timing intent

must not become a hard-coded physical schedule.

Scheduling computes a valid realization from:

- dependencies;
- resources;
- constraints;
- target capabilities.

---

134. Syntax and Resilience

If resilience constructs are exposed by Zamani syntax, they MUST describe policy or intent.

The grammar MUST NOT own:

- retry algorithms;
- recovery orchestration;
- incident diagnosis;
- checkpoint implementation;
- backend switching algorithms.

Those remain downstream resilience responsibilities.

---

135. Checkpoint Syntax

If checkpoint/recovery declarations exist, they describe semantic recovery boundaries.

They MUST NOT imply that arbitrary quantum state can always be serialized.

The language MUST distinguish concepts such as:

classical execution checkpoint
compiled-program checkpoint
logical checkpoint
measurement boundary
QEC-supported checkpoint
provider-supported state checkpoint

The syntax MUST NOT promise impossible state persistence.

---

136. Security Boundary

Source syntax may declare security requirements.

Actual enforcement belongs to:

semantic validation
compiler
runtime
hardware security layer
deployment system

A source declaration alone MUST NOT be treated as proof that the requested security property has been enforced.

---

137. Capability Certificates

If capability certificates are represented syntactically, they identify claims or requirements.

They do not themselves constitute cryptographic proof.

Cryptographic verification belongs to the security/verification subsystem.

---

138. Hardware Independence

The syntax model MUST remain hardware-independent by default.

Hardware-specific behavior requires explicit syntax.

This principle applies to:

- CPU;
- GPU;
- FPGA;
- ASIC;
- QPU;
- accelerator;
- memory;
- network;
- distributed node;
- storage.

---

139. Explicit Hardware Binding

Explicit binding MAY exist for systems programming and deployment.

For example:

target-specific
device-specific
address-specific

constructs may be supported.

Such constructs MUST be visibly distinguishable from portable syntax.

They MUST carry an explicit portability consequence.

---

140. Future Hardware

The syntax MUST support future hardware through abstractions.

A future accelerator should not require rewriting the core grammar.

It should normally be representable as:

new capability
new target
new dialect
new backend
new lowering

rather than:

new fundamental language

---

141. Future Computational Domains

New domains MUST be able to integrate through:

types
effects
capabilities
resources
declarations
expressions
statements
dialects

where possible.

A new domain should not require modifying unrelated grammar constructs.

This minimizes long-term coupling.

---

142. Domain Extension Contract

Every new domain grammar contribution MUST specify:

domain
owner
syntax
AST mapping
semantic mapping
IR mapping
capabilities
resources
effects
diagnostics
tests
compatibility

It MUST also explicitly state what it does not own.

---

143. File-Level Completion Contract

This document is complete only when every syntax family defined here has corresponding:

1. concrete grammar rules;
2. AST mapping;
3. semantic contract;
4. diagnostics;
5. tests;
6. compatibility policy.

A syntax construct MUST NOT be declared stable solely because it appears in documentation.

---

144. Integration Contract for "grammar/Zamani.g4"

"grammar/Zamani.g4" MUST:

- implement this syntax model;
- remain the canonical ANTLR grammar;
- contain no arbitrary hardware ceilings;
- contain no semantic execution logic;
- expose one canonical compilation-unit entry point;
- maintain deterministic precedence;
- preserve source structure;
- integrate domain grammar fragments consistently.

If the grammar is split into imported files, the composed result MUST remain equivalent to the normative model.

---

145. Integration Contract for "grammar/grammar.md"

"grammar/grammar.md" MUST document the syntax actually implemented by the reference frontend.

It MUST NOT contradict this document.

If implementation lags specification, the difference MUST be explicit.

Example:

Specification: SUPPORTED
Implementation: PLANNED

is valid.

Silently documenting unimplemented syntax as implemented is not.

---

146. Integration Contract for "grammar/Zamani-Grammar.md"

"grammar/Zamani-Grammar.md" may contain:

- proposals;
- broad language vision;
- historical material;
- future constructs;
- examples;
- design alternatives.

However, every construct MUST have an identifiable lifecycle.

It MUST NOT override this syntax model.

---

147. Integration Contract for "grammar/specification/grammar-authority.md"

"grammar-authority.md" owns authority relationships.

This file owns the actual syntax model.

Therefore:

grammar-authority.md
    → determines who is authoritative

syntax-model.md
    → determines how syntax is structured

Neither document may duplicate the other's complete contents.

---

148. Integration Contract for "language-principles.md"

"language-principles.md" defines high-level language principles.

This file translates those principles into syntax architecture.

The syntax model MUST remain consistent with:

- semantic portability;
- scalability;
- extensibility;
- domain composition;
- POCO-REAF.

---

149. Integration Contract for "language-scope.md"

"language-scope.md" defines the domains Zamani intends to support.

This document defines how those domains enter syntax without creating independent languages.

A domain appearing in scope does not automatically mean every feature is already implemented.

---

150. Integration Contract for "language-version.md"

"language-version.md" owns version identity and lifecycle.

This document defines how syntax participates in versioning.

No syntax rule may invent an independent incompatible versioning mechanism.

---

151. Integration Contract for "compatibility.md"

"compatibility.md" owns compatibility policy.

This document defines syntax compatibility requirements.

Breaking syntax changes MUST follow the compatibility policy.

---

152. Integration With Tests

Each syntax rule MUST have test ownership.

Tests MUST be organized by feature/domain and include:

positive
negative
boundary
compatibility
determinism
round-trip
cross-domain
scalability

Tests MUST NOT use fixed hardware limits as substitutes for scalability tests.

---

153. Hard-Coding Audit

The following patterns are forbidden in the language grammar unless they represent genuine lexical or syntactic requirements:

MAX_QUBITS
MAX_CORES
MAX_THREADS
MAX_GPUS
MAX_FPGAS
MAX_DEVICES
MAX_NODES
MAX_MEMORY
MAX_TENSOR_SIZE
MAX_CIRCUIT_DEPTH
MAX_PROGRAM_SIZE

Also forbidden are hidden equivalents such as:

q0 | q1 | q2 | ... | q63

when those alternatives exist solely to encode hardware limits.

---

154. Legitimate Finite Bounds

Some syntax constructs may genuinely have finite forms.

Examples:

Boolean literals
fixed punctuation
finite escape forms
finite language operators

These are legitimate because the bound is part of the language construct itself.

A bound is not legitimate merely because one implementation finds it convenient.

---

155. Implementation Limits

Implementation limits are permitted for safety.

They MUST be:

- configurable where appropriate;
- documented;
- independently testable;
- distinguishable from language semantics;
- surfaced through diagnostics.

Example:

parser nesting budget

is an implementation safeguard.

It must not become:

Zamani permits at most N nested blocks.

---

156. Infinite and Unbounded Structures

The language may describe recursively or parametrically unbounded structures.

Examples include:

recursive types
recursive functions
generic structures
dynamic collections
distributed collections
quantum registers
tensor dimensions

"Unbounded" means no arbitrary language-defined ceiling.

All implementations remain constrained by finite resources.

---

157. Resource Availability Principle

A program is valid if its declared semantics are valid.

Whether it can execute on a particular target is determined separately.

Therefore:

program validity

must not be confused with:

target satisfiability

A compiler may report:

valid program
target cannot satisfy requirement

rather than requiring source modification.

This is central to POCO-REAF.

---

158. Example: Portable Quantum Program

Conceptually:

fn prepare() {
    let q = qubits(algorithm_width);

    H(q[0]);

    for i in 1..algorithm_width {
        CNOT(q[0], q[i]);
    }

    measure(q);
}

The syntax describes a parameterized computation.

It does not declare:

use QPU 7
use qubits 0..63
use topology X

The compiler can map it according to available resources.

If a target cannot satisfy the requirement, target analysis reports the incompatibility.

---

159. Example: Portable Classical Program

fn compute<T, N>(input: Vector<T, N>) -> Vector<T, N> {
    return transform(input);
}

The syntax does not require:

N <= 1024

unless such a bound is part of the function's mathematical contract.

The implementation may select:

- scalar execution;
- SIMD;
- GPU;
- accelerator;
- distributed execution.

---

160. Example: Hybrid Program

fn hybrid<T>(input: Vector<T, N>) {
    let parameters = optimize(input);

    quantum {
        let q = qubits(model_width);
        prepare(q, parameters);
        measure(q);
    }

    return parameters;
}

The syntax permits classical and quantum computation in one program.

The semantic system determines:

- types;
- effects;
- capabilities;
- resource requirements.

Quantum lowering reaches "quantum::ir".

---

161. Example: Hardware-Aware but Portable

fn compute(data: Tensor<f64>) {
    requires capability accelerator;

    prefer low_latency;

    execute(data);
}

This expresses intent.

It does not require a particular accelerator.

---

162. Example: Explicit Target Binding

A target-specific form may intentionally exist:

target "specific_environment" {
    ...
}

Such syntax is explicitly non-portable.

The language MUST distinguish this from portable source.

A target-specific construct MUST NOT silently contaminate unrelated source.

---

163. Example: HDL

Conceptually:

hardware module Processor<T> {
    input  clock;
    input  data: T;
    output result: T;

    process {
        result = transform(data);
    }
}

The syntax describes hardware behavior.

It does not inherently specify:

FPGA family
ASIC process node
exact LUT count
exact gate count
vendor device

unless explicitly requested.

---

164. Example: Distributed Computation

service compute<T>(input: T) {
    requires distributed;
    prefer locality;

    execute(input);
}

The source does not specify:

exactly 8 nodes
node 0
node 1
...
node 7

unless those are genuinely semantic requirements.

---

165. Example: Resource Requirement

requires {
    quantum;
    qubits >= logical_width;
    memory >= working_memory;
}

The grammar expresses requirements.

ResourceManager and target analysis determine satisfaction.

---

166. Semantic Preservation

Every target transformation MUST preserve the semantics represented by the source program unless an explicitly declared semantic relaxation exists.

The syntax model therefore establishes:

source syntax
     ↓
stable semantic intent
     ↓
many valid implementations

rather than:

source syntax
     ↓
one hard-coded machine implementation

---

167. Portability Classes

Every source artifact SHOULD be classifiable as:

portable
capability-constrained
resource-constrained
target-constrained
target-specific

This classification is semantic metadata, not necessarily syntax.

---

168. Portability Does Not Mean Universality of Execution

Zamani MUST NOT promise impossible execution.

A program requiring:

quantum capability

cannot execute on a classical-only target unless a valid semantic substitute exists.

POCO-REAF therefore means:

same source semantics
+
different valid realizations

not:

same physical execution on every machine

---

169. Future-Proofing

The syntax model MUST avoid assumptions tied to current technology.

Do not make fundamental syntax depend on:

- today's CPU architecture;
- today's GPU APIs;
- today's FPGA vendors;
- today's QPU gate sets;
- today's networking protocols;
- today's accelerator formats.

Such details belong to target-specific dialects, backends, or interoperability layers.

---

170. Language Evolution Principle

When a new technology appears, the preferred integration sequence is:

existing syntax abstraction
        ↓
capability
        ↓
resource model
        ↓
dialect if necessary
        ↓
semantic representation
        ↓
backend

Only introduce new core syntax when existing abstractions cannot express the new semantics clearly and safely.

---

171. Compatibility Principle

Adding a new target MUST NOT require changing existing portable source merely because the target has a new architecture.

Adding:

new CPU
new GPU
new QPU
new accelerator
new FPGA
new distributed platform

should normally require backend/target support rather than source-language changes.

---

172. Grammar Completeness

The canonical grammar is considered syntax-complete for a feature only when:

- the feature is specified;
- the grammar accepts valid forms;
- invalid forms are rejected;
- AST mapping exists;
- diagnostics exist;
- semantic ownership exists;
- tests exist;
- compatibility status exists;
- downstream integration is defined.

Documentation alone does not constitute completion.

---

173. Production Readiness

The syntax implementation is production-ready only when:

- there is one authoritative grammar;
- parser behavior is deterministic;
- grammar/AST ownership is explicit;
- semantic boundaries are explicit;
- no accidental machine-size limits exist;
- quantum syntax integrates with "quantum::ir";
- classical syntax integrates with classical IR;
- HDL syntax integrates with hardware compilation;
- resource syntax integrates with resource management;
- capability syntax integrates with capability analysis;
- effects integrate with effect analysis;
- target-specific behavior is explicit;
- compatibility is versioned;
- diagnostics are stable;
- malformed input is safely handled;
- tests cover every syntax family;
- Rust frontend code is safe Rust;
- Rust 1.97/1.97.1 compatibility is verified.

---

174. Completion Criteria for This Specification

"grammar/specification/syntax-model.md" is complete when:

1. every syntax family has a documented structural contract;
2. every domain has an ownership boundary;
3. quantum syntax explicitly integrates with "quantum::ir";
4. no grammar-level machine-size assumptions exist;
5. resource and capability syntax are distinct;
6. target-specific syntax is distinguishable from portable syntax;
7. POCO-REAF is structurally supported;
8. syntax does not own semantic algorithms;
9. grammar authority is unambiguous;
10. versioning is delegated to the language-version/compatibility specifications;
11. implementation limits are distinguished from language limits;
12. AST integration is defined;
13. IR integration is defined;
14. QEC/ZQN/scheduling/routing/hardware boundaries are defined;
15. tests can be derived directly from this document.

---

175. Required Downstream Integration

The following files/subsystems MUST consume this specification without redefining it:

grammar/Zamani.g4
grammar/grammar.md
grammar/Zamani-Grammar.md
grammar/specification/grammar-authority.md
grammar/specification/language-principles.md
grammar/specification/language-scope.md
grammar/specification/language-version.md
grammar/specification/compatibility.md

and, where implemented:

src/lexer.rs
src/parser.rs
src/ast/
src/semantic.rs
src/ir_gen.rs
src/ir_verify.rs
src/quantum/ir/
src/quantum/frontend/
src/quantum/qec/
src/quantum/zqn/
src/quantum/scheduling/
src/quantum/optimization/
src/quantum/hardware/
src/quantum/resilience/

The exact paths MUST follow the repository's current module ownership.

No downstream component may become a second grammar authority.

---

176. Dependency Direction

The mandatory dependency direction is:

syntax-model.md
        ↓
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
canonical IR
        ↓
domain compilation
        ↓
optimization
        ↓
QEC / ZQN / scheduling / routing
        ↓
hardware abstraction
        ↓
runtime
        ↓
deployment

Reverse dependencies are prohibited where they create circular authority.

For example:

grammar → runtime

is prohibited merely to determine whether syntax is valid.

---

177. Non-Ownership Summary

This specification explicitly does not own:

type-checking algorithms
resource allocation
hardware discovery
hardware calibration
routing algorithms
scheduling algorithms
optimization algorithms
QEC algorithms
noise simulation
runtime execution
device drivers
backend transport
deployment orchestration

It owns their source-level syntax contracts only.

---

178. Final Architectural Law

The definitive syntax principle is:

«Zamani syntax describes what computation means and what it requires; downstream systems decide how that computation is represented, optimized, scheduled, mapped, executed, and deployed.»

Therefore:

Program
   ↓
Semantic Intent
   ↓
Canonical Representation
   ↓
Target Adaptation
   ↓
Execution

not:

Program
   ↓
Current Hardware Assumptions
   ↓
Permanent Language Semantics

---

179. Final Scalability Law

Zamani syntax MUST scale from:

one value

to:

one program

to:

one processor

to:

many processors

to:

heterogeneous systems

to:

distributed systems

to:

quantum systems

to:

future computational systems

without requiring arbitrary grammar-level limits.

The language's practical scale is determined by:

program semantics
+
implementation capability
+
available resources
+
target capabilities
+
physical reality

not by arbitrary constants embedded in the grammar.

---

180. Final POCO-REAF Law

The intended programming model is:

                    ONE PROGRAM
                         │
                         ▼
                 ONE SEMANTIC MODEL
                         │
                         ▼
                 ONE PORTABLE ARTIFACT
                         │
          ┌──────────────┼──────────────┐
          ▼              ▼              ▼
        CPU             GPU            QPU
          │              │              │
          ▼              ▼              ▼
        FPGA           ASIC       DISTRIBUTED
          │              │              │
          └──────────────┼──────────────┘
                         ▼
                  MANY REALIZATIONS

The source program remains stable.

The realization adapts.

That is the syntax-level foundation of:

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF).

---

181. Normative Final Statement

A conforming Zamani implementation MUST treat syntax as a stable, extensible, deterministic representation of source-level computation.

It MUST NOT make temporary hardware characteristics into permanent language restrictions.

It MUST NOT require a separate language for classical, quantum, HDL, hardware, distributed, AI, or future computation.

It MUST preserve the canonical semantic boundaries of the repository.

It MUST preserve "quantum::ir" as the canonical quantum semantic boundary.

It MUST allow target adaptation without source rewriting whenever semantic requirements remain satisfiable.

It MUST distinguish:

requirement
capability
constraint
preference
hint
resource
target

It MUST distinguish:

logical resource
physical resource

It MUST distinguish:

portable semantics
target realization

It MUST distinguish:

language limits
implementation limits
resource limits
physical limits

It MUST remain compatible with safe Rust 1.97/1.97.1 frontend implementations.

And above all:

«Zamani source describes computation once. The compiler and runtime determine how that computation is realized wherever and at whatever scale the available resources and capabilities permit.»

This is the normative syntax foundation for Zamani — From Atom to Everywhere.