Zamani Syntax Specification

File: "grammar/specification/syntax.md"
Status: Normative
Specification role: Canonical human-readable syntax contract
Language: Zamani
Specification family: Zamani Universal Computing Language
Implementation baseline: Rust 1.97 / Rust 1.97.1, Rust 2021 edition
Implementation safety requirement: Zamani-owned Rust implementation MUST NOT use "unsafe"
Canonical grammar composition root: "grammar/Zamani.g4"

---

1. Purpose

This document defines the normative syntax of the Zamani programming language.

Zamani is a universal, compositional programming language intended to express:

- classical computation;
- quantum computation;
- hybrid quantum-classical computation;
- hardware description;
- hardware/software co-design;
- embedded and systems programming;
- distributed and parallel computation;
- HPC;
- AI and machine learning;
- data and tensor computation;
- networking;
- cryptography and security;
- scientific computation;
- accelerators;
- edge and cloud computation;
- future computational paradigms.

The syntax MUST support the principle:

«Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF).»

The syntax therefore describes portable computational intent, rather than a fixed physical machine.

The syntax MUST scale from the smallest supported computation to arbitrarily large computations subject only to:

1. the resources available at compilation, execution, or deployment;
2. the semantic requirements of the program;
3. implementation-defined practical limits that are not exposed as artificial language limits.

---

2. Normative Language

The following terms are normative:

- MUST — mandatory.
- MUST NOT — prohibited.
- SHOULD — recommended unless a documented reason exists otherwise.
- SHOULD NOT — discouraged unless justified.
- MAY — permitted but optional.
- IMPLEMENTATION LIMIT — a practical limitation of a particular implementation, not a language-level limit.
- PROGRAM SEMANTICS — meaning specified by the program.
- TARGET REALIZATION — mapping of portable semantics onto actual hardware/software resources.

---

3. Authority and Ownership

3.1 Syntax authority

The syntax architecture is divided into the following layers:

grammar/specification/syntax.md
                │
                ▼
       grammar components
                │
                ▼
         grammar/Zamani.g4
                │
                ▼
          lexer / parser
                │
                ▼
       src/frontend/ast/
                │
                ▼
       semantic analysis
                │
                ▼
       canonical semantic IR

"syntax.md" defines the language-level syntax contract.

"grammar/Zamani.g4" implements the ANTLR grammar composition.

The lexer defines actual tokenization.

The parser constructs the domain-neutral AST.

Semantic analysis determines meaning.

The IR represents meaning rather than source syntax.

---

4. Existing File Integration

This specification deliberately does not replace existing files.

4.1 "grammar/Zamani.g4"

"Zamani.g4" remains the canonical ANTLR composition root.

It MUST:

- define the root "program" structure;
- compose the modular grammar;
- expose universal declaration, statement, expression, and type entry points;
- enforce syntactic composition;
- remain compatible with the lexer/parser architecture.

It SHOULD NOT become a second specification containing independent semantic definitions.

---

4.2 "grammar/grammar.md"

"grammar.md" is the implementation-conformance reference.

It MUST describe what the current implementation actually accepts.

It MUST NOT silently override this specification.

Differences between this specification and the implementation MUST be represented as conformance gaps rather than hidden syntax changes.

---

4.3 "grammar/Zamani-Grammar.md"

"Zamani-Grammar.md" remains an extended/historical/design reference.

It is NOT an independent syntax authority.

Features appearing there MUST enter the language through the normal feature lifecycle:

proposal
  ↓
syntax definition
  ↓
AST contract
  ↓
semantic contract
  ↓
IR contract
  ↓
implementation
  ↓
tests
  ↓
compatibility review
  ↓
stable feature

---

4.4 "grammar/DESIGN.md"

"DESIGN.md" defines the architecture surrounding this syntax specification.

In particular, this specification depends upon:

- one language;
- deterministic parsing;
- domain-neutral AST;
- target independence;
- resource/capability separation;
- canonical quantum IR;
- no artificial hardware limits;
- explicit compatibility;
- explicit source spans;
- extensibility without fragmentation.

---

5. Syntax Design Principles

Zamani syntax MUST follow these principles.

5.1 One language

Quantum, classical, HDL, AI, distributed, networking, and other facilities are domains of one language.

They MUST NOT become unrelated sublanguages.

---

5.2 Composability

Language constructs MUST be composable.

For example:

classical computation
    ↓
quantum operation
    ↓
measurement
    ↓
classical decision
    ↓
hardware action

MUST be expressible without switching programming languages.

---

5.3 Target independence

Portable source syntax MUST NOT require knowledge of a particular:

- CPU;
- GPU;
- FPGA;
- QPU;
- accelerator;
- node;
- memory bank;
- physical qubit;
- register number;
- network interface;
- vendor-specific device.

---

5.4 Parametricity

Whenever a quantity can legitimately be determined from program inputs, types, configuration, capabilities, or runtime resources, syntax MUST permit it to remain symbolic or parameterized.

Examples include:

- vector length;
- tensor dimensions;
- number of workers;
- number of qubits;
- memory capacity;
- number of nodes;
- accelerator count;
- topology;
- hardware width;
- deployment scale.

---

5.5 No artificial language limits

The syntax MUST NOT establish universal limits such as:

MAX_QUBITS
MAX_CPUS
MAX_GPUS
MAX_FPGAS
MAX_NODES
MAX_THREADS
MAX_MEMORY
MAX_TENSOR_ELEMENTS
MAX_VECTOR_WIDTH
MAX_REGISTER_WIDTH
MAX_TIMELINES

A program MAY explicitly specify a number.

The language MUST NOT impose that number as a universal maximum.

---

6. Syntax Layers

Zamani syntax consists of the following conceptual layers:

Source
  │
  ├── lexical structure
  │
  ├── names and paths
  │
  ├── declarations
  │
  ├── types
  │
  ├── expressions
  │
  ├── statements
  │
  ├── modules
  │
  ├── effects
  │
  ├── resources/capabilities
  │
  ├── concurrency
  │
  └── domain constructs

Domain constructs MUST reuse common language foundations wherever possible.

---

7. Source Units

A Zamani program consists of one or more source units.

Conceptually:

program
    ::= sourceUnit*
       EOF
    ;

A source unit MAY contain:

- module declarations;
- imports;
- exports;
- attributes;
- declarations;
- functions;
- types;
- implementations;
- statements where permitted.

There MUST be no fixed maximum number of source constructs.

---

8. Source Locations

Every syntactic construct MUST be representable with source-location information.

At minimum, a parser implementation MUST be capable of associating:

- source file;
- starting position;
- ending position;

with syntactic constructs and diagnostics.

A conforming AST SHOULD preserve sufficient source information for:

- diagnostics;
- tooling;
- formatting;
- navigation;
- refactoring;
- provenance;
- debugging.

---

9. Identifiers

Identifiers name language entities.

They MAY identify:

- variables;
- functions;
- types;
- modules;
- fields;
- parameters;
- resources;
- capabilities;
- operations;
- domains;
- user-defined abstractions.

Identifiers MUST NOT be restricted to a fixed number of characters unless an implementation limitation is explicitly documented and does not change language semantics.

Unicode support SHOULD follow the lexical specification.

---

10. Qualified Names

Zamani MUST support qualified names.

Conceptually:

qualifiedName
    ::= identifier
      | qualifiedName "." identifier
    ;

Qualified names MAY identify:

module.function
namespace.type
domain.operation
vendor.operation
library.symbol

The number of qualification levels MUST NOT be artificially bounded.

---

11. Paths

Paths identify entities across module/package namespaces.

A path MAY contain:

- identifiers;
- generic arguments;
- namespace separators;
- package/module components.

Path syntax MUST remain independent of physical filesystem layout.

---

12. Attributes

Attributes attach metadata to language constructs.

Conceptually:

attributedItem
    ::= attribute* item
    ;

Attributes MAY describe:

- compilation intent;
- optimization;
- resources;
- capabilities;
- effects;
- interoperability;
- diagnostics;
- verification;
- hardware intent;
- execution policy;
- provenance.

Attributes MUST NOT be used as an uncontrolled escape hatch around language semantics.

---

13. Modifiers

Modifiers alter syntactic declarations where the relevant specification permits them.

Examples include conceptual categories such as:

- visibility;
- mutability;
- asynchronous behavior;
- purity/effects;
- concurrency;
- compile-time behavior;
- hardware intent.

Each modifier MUST have a defined semantic meaning.

A modifier MUST NOT exist solely because a backend happens to require it.

---

14. Declarations

Zamani declarations establish named entities.

The universal declaration model includes:

declaration
    ::= variableDeclaration
      | constantDeclaration
      | typeDeclaration
      | functionDeclaration
      | moduleDeclaration
      | resourceDeclaration
      | capabilityDeclaration
      | domainDeclaration
      | implementationDeclaration
      | ...
    ;

The exact domain-specific forms are defined by their respective grammar components.

There MUST be one universal declaration dispatch mechanism.

---

15. Variables and Bindings

Bindings associate names with values.

Conceptually:

let value = expression

and, where mutability is part of the language contract:

var value = expression

Binding syntax MUST remain independent of target hardware.

---

16. Constants

Constants represent compile-time or immutable semantic values according to the type and semantic system.

A constant MAY contain a machine-sized value when that value is part of the program.

This does not create a corresponding compiler-wide limit.

For example:

const n = 1024

is valid.

It MUST NOT imply:

MAX_VALUE = 1024

for the entire language.

---

17. Types

Type syntax is defined by "grammar/specification/types.md" and "grammar/spec/type-system.md".

The syntax MUST support, where provided by the type system:

- primitive types;
- named types;
- generic types;
- tuples;
- arrays;
- slices;
- records;
- functions;
- references;
- optionals;
- results;
- resource types;
- capability types;
- quantum types;
- tensor types;
- hardware-related semantic types;
- user-defined types.

Types MUST describe semantic properties rather than encode backend implementation details.

---

18. Generic Types

Zamani MUST support parameterized types.

Conceptually:

Container<T>
Tensor<T, Shape>
Array<T, N>
QuantumRegister<N>

Parameters MAY be:

- types;
- values;
- compile-time expressions;
- symbolic dimensions;
- capabilities;
- semantic constraints.

There MUST be no fixed number of generic parameters imposed by the language.

---

19. Dependent and Symbolic Dimensions

Where supported by the type system, dimensions MAY remain symbolic.

For example:

Tensor<Real, [rows, columns]>

or:

QubitRegister<n>

The syntax MUST NOT require the compiler to know the physical value of every parameter at parsing time.

---

20. Expressions

Expressions produce values or semantic operations.

Conceptually:

expression
    ::= assignmentExpression
    ;

The precedence hierarchy MUST be deterministic and centrally defined.

Expression parsing MUST NOT depend on domain-specific runtime state.

---

21. Expression Precedence

Precedence MUST be explicit and stable.

A typical hierarchy is:

primary
postfix
unary
multiplicative
additive
shift
comparison
equality
bitwise
logical
conditional
assignment

The final precedence table MUST be maintained in:

grammar/expressions/precedence.md

and implemented consistently by:

grammar/expressions/
grammar/Zamani.g4
src/lexer.rs
src/parser.rs

There MUST be one precedence model.

---

22. Literals

Zamani supports literal categories defined by the lexical specification.

These may include:

- integers;
- floating-point values;
- strings;
- characters;
- booleans;
- arrays;
- tuples;
- maps;
- domain-specific literals;
- quantum-state notation where defined.

Literal syntax MUST NOT impose artificial semantic bounds.

---

23. Numeric Literals

Numeric literal syntax MUST support values required by the language's numeric model.

The grammar MUST NOT establish a universal machine word size.

For example, syntax MUST NOT imply that all integers are:

32-bit

or:

64-bit

unless a particular type explicitly requests that semantic width.

---

24. Collections

Collection syntax MAY include:

- arrays;
- sequences;
- slices;
- maps;
- sets;
- records;
- streams;
- comprehensions.

The number of elements MUST NOT be syntactically limited.

---

25. Function Calls

Conceptually:

callExpression
    ::= expression "(" argumentList? ")"
    ;

The argument list MUST NOT have a universal fixed maximum.

Named arguments MAY be supported where specified by the function grammar.

---

26. Indexing

Conceptually:

value[index]

and multidimensional forms where supported:

tensor[i, j, k]

The number of dimensions MUST be semantic rather than a parser-wide fixed limit.

---

27. Ranges

Range syntax MAY express:

- bounded ranges;
- open-ended ranges;
- symbolic ranges;
- stepped ranges.

A range MUST NOT imply a fixed iteration count.

---

28. Lambda and Closure Syntax

Zamani MAY represent anonymous functions through lambda/closure syntax.

Conceptually:

|x| expression

or an equivalent syntax defined by the canonical grammar.

Closures MUST integrate with:

- type inference;
- ownership/resource semantics;
- effects;
- concurrency.

---

29. Blocks

A block is a sequence of statements or expressions within a lexical scope.

Conceptually:

block
    ::= "{" statementOrExpression* "}"
    ;

There MUST be no fixed maximum nesting depth at the language level.

---

30. Conditional Syntax

Conditional forms MUST support ordinary control flow and expression-oriented use where specified.

Examples conceptually include:

if condition {
    ...
} else {
    ...
}

and:

result = if condition {
    a
} else {
    b
}

---

31. Pattern Matching

Pattern matching MUST support extensible patterns without requiring a separate language for each domain.

Patterns MAY include:

- literals;
- identifiers;
- tuples;
- records;
- variants;
- ranges;
- destructuring;
- guards;
- domain-defined patterns.

---

32. Loops and Iteration

Zamani MUST support scalable iteration constructs.

Examples include conceptual:

for item in collection

while condition

loop

Iteration MUST NOT assume a fixed number of iterations.

---

33. Functions

Function declarations MUST support:

- names;
- parameters;
- return types;
- generic parameters;
- constraints;
- effects;
- resource requirements;
- capabilities;
- contracts;
- asynchronous execution where applicable.

Functions MUST be composable across supported domains.

---

34. Modules

Module syntax MUST support:

- declaration;
- imports;
- exports;
- aliases;
- namespaces;
- visibility;
- package integration;
- version constraints.

The module graph MUST NOT have a fixed maximum size or depth.

---

35. Effects

Effect syntax describes observable computational behavior.

Examples of semantic effect categories include:

- I/O;
- mutation;
- allocation;
- networking;
- concurrency;
- quantum execution;
- hardware interaction;
- randomness;
- persistence;
- external computation.

Effects MUST NOT directly encode implementation-specific APIs.

---

36. Capabilities

Capability syntax describes what an execution or compilation environment must support.

Conceptually:

requires capability("quantum.measurement")

or equivalent canonical syntax.

Capabilities MUST be distinct from physical resource identities.

For example:

requires capability("quantum.mid_circuit_measurement")

is portable intent.

This is different from:

use physical_qubit(17)

which is target realization.

---

37. Resource Requirements

Resource syntax expresses semantic resource requirements.

Examples include:

requires qubits >= n
requires memory >= required_memory
requires compute
requires communication

Resource quantities MAY be:

- constants;
- variables;
- symbolic expressions;
- type-derived values;
- runtime values where permitted;
- negotiated values.

Resource declarations MUST NOT establish compiler-wide maxima.

---

38. Requirement, Constraint, Preference, Hint

Zamani syntax MUST distinguish:

Requirement

Something necessary for valid execution.

requires capability("quantum.measurement")

Constraint

A condition that must hold.

requires latency <= bound

Preference

A desired but non-mandatory realization.

prefer accelerator("tensor")

Hint

Information that may improve implementation.

hint locality("high")

Implementation decision

A concrete realization chosen downstream.

map logical_resource -> physical_resource

These categories MUST NOT be conflated.

---

39. Classical Computing Syntax

Classical computation uses the universal syntax system.

Classical-specific constructs MAY include:

- numeric operations;
- vectors;
- matrices;
- tensors;
- symbolic computation;
- statistics;
- scientific operations;
- signal processing;
- numerical algorithms.

Mathematical functions SHOULD generally be represented through:

typed operation
+
library/intrinsic
+
semantic capability

rather than creating an unlimited keyword list.

---

40. Quantum Syntax

Quantum syntax is defined in:

grammar/quantum/
grammar/spec/quantum.md

Quantum syntax MUST describe quantum intent.

It MUST NOT encode a fixed hardware topology.

---

41. Quantum Operations

Zamani MUST support generic quantum operations.

The language MUST NOT require an exhaustive list such as:

H
X
Y
Z
CNOT
...

to be hard-coded into the grammar.

Instead, the syntax MUST support named and parameterized operations.

Conceptually:

apply operation to targets

or:

apply namespace.operation(parameters) to targets

This supports:

- standard operations;
- custom operations;
- parameterized operations;
- library operations;
- future operations;
- vendor-specific operations through interoperability mechanisms.

---

42. Quantum AST Integration

Quantum syntax MUST lower through the domain-neutral AST.

The preferred semantic shape is conceptually:

Operation {
    name
    namespace
    operands
    parameters
    results
    attributes
    modifiers
    effects
    capabilities
    source
}

The AST MUST NOT introduce a fixed:

enum QuantumGate {
    X,
    H,
    CNOT,
    ...
}

as the canonical language representation.

---

43. Canonical Quantum IR

Quantum source syntax MUST ultimately lower to the existing canonical:

quantum::ir

boundary.

The grammar MUST NOT create a competing quantum IR.

The intended pipeline is:

Zamani syntax
    ↓
domain-neutral AST
    ↓
semantic quantum model
    ↓
quantum::ir
    ↓
optimization
    ↓
decomposition
    ↓
routing
    ↓
scheduling
    ↓
QEC / resilience
    ↓
ZQN
    ↓
HAL
    ↓
target realization

---

44. Quantum Resource Independence

Quantum syntax MUST NOT impose:

MAX_QUBITS
MAX_QUBIT_REGISTER
MAX_CIRCUIT_DEPTH
MAX_GATES
MAX_SHOTS

A program MAY require a particular amount of resources.

That requirement is semantic.

The implementation determines whether the requested execution is feasible.

---

45. Logical and Physical Quantum Resources

The syntax SHOULD distinguish logical quantum resources from physical realization.

A program SHOULD be able to describe:

logical qubit
logical register
logical operation
logical circuit

without specifying:

physical qubit 17
physical coupler 4
specific QPU topology

Physical mapping belongs to routing/HAL/target realization.

---

46. Quantum Measurement

Measurement syntax MUST support the semantic act of measurement.

It MUST NOT automatically insert measurements merely because a program contains qubits.

Measurement occurs only when the source or semantic transformation explicitly requires it.

This prevents the earlier OpenQASM-style behavior in which all qubits were automatically measured.

---

47. Mid-Circuit Measurement and Feed-Forward

The syntax MUST permit:

quantum operation
→ measurement
→ classical result
→ classical decision
→ quantum operation

without requiring a separate language.

---

48. Quantum Control

Quantum control constructs MAY express:

- controlled operations;
- adjoint operations;
- parameterized operations;
- conditional execution;
- dynamic circuits;
- classical feed-forward.

These are semantic constructs and MUST lower through the canonical quantum representation.

---

49. QEC, ZQN, Routing, Scheduling and HAL

The syntax MAY express intent concerning:

- error correction;
- fault tolerance;
- noise;
- reliability;
- timing;
- scheduling;
- resource requirements.

However:

QEC owns error correction.

ZQN owns fault/noise semantics.

Routing owns physical realization.

Scheduling owns timing/order/resource scheduling.

HAL owns actual device capabilities and state.

Resilience owns recovery/orchestration.

The grammar MUST NOT duplicate these subsystems.

---

50. Hybrid Quantum-Classical Syntax

Hybrid computation MUST be expressible using ordinary language composition.

For example:

classical calculation
    ↓
quantum operation
    ↓
measurement
    ↓
classical branch
    ↓
quantum operation

No separate hybrid programming language is required.

---

51. HDL Syntax

HDL syntax is defined under:

grammar/hdl/
grammar/spec/hdl.md

HDL MUST support semantic descriptions of:

- modules;
- ports;
- signals;
- nets;
- registers;
- combinational logic;
- sequential logic;
- clocks;
- resets;
- timing;
- state machines;
- pipelines;
- memories;
- interfaces;
- protocols;
- verification;
- synthesis intent.

---

52. Parameterized Hardware

Hardware descriptions MUST support parameterization.

Examples include conceptual:

module Processor<Width>

memory<Depth>

vector<Width>

The grammar MUST NOT turn a particular width into a universal language limit.

---

53. Hardware/Software Co-Design

Zamani syntax MUST permit one program to describe coordinated:

- software;
- hardware;
- memory;
- communication;
- accelerator;
- timing;
- verification;
- deployment intent.

This allows a computation to remain semantically stable while realization changes from:

CPU
GPU
FPGA
ASIC
QPU
distributed system
future accelerator

---

54. Hardware Capabilities

Hardware syntax MUST express capabilities rather than assuming concrete devices.

For example:

requires capability("tensor.compute")

rather than:

use GPU0

Target-specific device selection belongs downstream.

---

55. Concurrency

Concurrency syntax MUST support:

- asynchronous operations;
- tasks;
- spawning;
- joining;
- channels;
- actors;
- synchronization;
- parallel loops;
- reductions;
- pipelines;
- distributed execution.

Concurrency syntax MUST NOT assume a fixed number of:

- CPUs;
- cores;
- threads;
- workers;
- processes.

---

56. Parallelism

Zamani MUST permit semantic parallelism.

Conceptually:

parallel for item in collection

means that independent work MAY be executed concurrently.

It does not mean:

run on exactly 8 threads

unless eight workers are explicitly part of program semantics.

---

57. Distributed Syntax

Distributed syntax MAY express:

- services;
- processes;
- actors;
- messages;
- channels;
- replication;
- partitioning;
- placement intent;
- consistency;
- transactions;
- fault tolerance;
- collective operations.

The language MUST NOT impose a fixed maximum number of nodes.

---

58. Topology

Topology syntax MAY describe semantic requirements or constraints.

For example:

requires connectivity(...)

The syntax MUST distinguish this from a concrete physical topology.

Physical topology discovery and realization belong to downstream systems.

---

59. AI and Machine Learning

AI syntax MAY support:

- models;
- datasets;
- tensors;
- training;
- inference;
- differentiation;
- probabilistic computation;
- symbolic computation;
- agents;
- learning;
- model deployment.

AI syntax MUST remain framework-independent.

The grammar MUST NOT become a syntax wrapper around:

- a single ML framework;
- a particular accelerator;
- a vendor runtime;
- a specific model format.

---

60. Data and Tensor Syntax

Zamani MUST support scalable data abstractions.

Tensor dimensions SHOULD be expressible symbolically.

For example:

Tensor<T, [batch, channels, height, width]>

The syntax MUST NOT impose a fixed:

MAX_RANK
MAX_DIMENSION
MAX_ELEMENTS

---

61. Networking

Networking syntax MAY express:

- endpoints;
- services;
- protocols;
- channels;
- streams;
- requests;
- responses;
- communication requirements.

Addresses SHOULD remain abstract where possible.

The language SHOULD distinguish:

communication intent

from:

physical interface selection

---

62. Security

Security syntax MAY express:

- identities;
- authorization;
- capabilities;
- policies;
- cryptographic intent;
- secrets;
- provenance;
- trust;
- secure computation.

Security constructs MUST integrate with the effect and capability system.

Secrets MUST NOT be required to appear directly in source when secure references are sufficient.

---

63. Compile-Time Syntax

Compilation-related syntax MAY express:

- target requirements;
- optimization intent;
- specialization;
- compilation profiles;
- reproducibility;
- deterministic builds;
- deployment requirements;
- artifact metadata.

Compilation syntax MUST describe intent rather than expose backend internals.

---

64. Runtime and Execution Syntax

Execution syntax MAY express:

- entry points;
- execution environments;
- scheduling policy;
- resource policy;
- resilience;
- checkpointing;
- observability;
- tracing;
- profiling;
- lifecycle.

Execution constructs MUST remain independent of a particular operating system or runtime implementation wherever possible.

---

65. Interoperability Syntax

Interoperability MAY support:

- C;
- C++;
- Rust;
- Python;
- WebAssembly;
- OpenQASM;
- QIR;
- HDL formats;
- other explicitly registered formats.

Interoperability formats are adapters.

They are NOT the canonical Zamani semantic model.

In particular:

OpenQASM → Zamani quantum semantics
QIR      → interoperability/downstream representation
LLVM     → backend representation
MLIR     → backend/intermediate interoperability

must not reverse the ownership model.

---

66. Macros

Macros MAY transform source syntax.

A macro system MUST preserve:

- hygiene;
- source locations;
- diagnostics;
- syntactic validity;
- semantic validation.

Macro expansion MUST NOT be used to bypass:

- type checking;
- effects;
- capabilities;
- resource validation;
- safety rules.

---

67. Metaprogramming

Metaprogramming MAY inspect or generate program structure where permitted.

It MUST remain subject to:

- phase rules;
- capability restrictions;
- reproducibility;
- security;
- diagnostics;
- semantic validation.

The syntax MUST NOT create an unrestricted escape from the language's safety model.

---

68. Dialects

Dialects MAY extend Zamani.

A dialect MUST identify:

- name;
- version;
- syntax additions;
- semantic additions;
- AST mapping;
- IR mapping;
- capabilities;
- compatibility;
- feature status.

A dialect MUST NOT silently redefine core Zamani syntax.

---

69. Extension Rules

Extensions MUST use explicit extension points.

An extension MUST NOT:

- redefine an existing token incompatibly;
- silently change operator precedence;
- change an existing construct's meaning;
- introduce ambiguous syntax;
- create a second AST representation;
- create a competing canonical IR.

---

70. Sankofa-Related Syntax

Existing Sankofa-inspired concepts from "Zamani-Grammar.md" MAY be retained where they provide useful language semantics.

These may include concepts such as:

- memory;
- recall;
- learning;
- history;
- temporal information;
- provenance;
- consensus;
- reasoning.

They MUST remain language constructs rather than parser-side state machines.

The implementation of memory, learning, or reasoning belongs outside the grammar.

Sankofa MUST NOT accidentally become a second canonical language architecture inside Zamani.

---

71. Temporal and Multi-Timeline Constructs

Where multi-timeline or temporal execution features are accepted, syntax MAY express:

- timeline creation;
- branching;
- speculative execution;
- fork/merge;
- observation;
- rewind;
- temporal relationships.

The syntax MUST NOT impose a fixed number of timelines or branches.

---

72. Nano and Future Domains

Future domains MAY be integrated through the same syntax architecture.

Examples include:

- molecular computation;
- nanoscale computation;
- neuromorphic systems;
- photonic computation;
- biological computation;
- analog computation;
- optical systems;
- future quantum architectures.

A new domain MUST reuse common:

- types;
- expressions;
- declarations;
- effects;
- resources;
- capabilities;
- concurrency;
- module;
- interoperability;

where applicable.

---

73. Domain Independence

No domain directory may redefine universal syntax unnecessarily.

For example:

expressions/

owns expression structure.

quantum/

adds quantum-specific semantic forms.

hdl/

adds hardware-specific semantic forms.

ai/

adds AI-specific semantic forms.

The domains MUST compose with the universal expression/type/declaration systems.

---

74. Generic Operations

Zamani SHOULD favor generic operations when an operation does not require unique language-level semantics.

Conceptually:

operation(name, parameters, operands)

can represent many operations.

Dedicated syntax SHOULD exist when it provides meaningful language semantics such as:

- control flow;
- declarations;
- resource requirements;
- effects;
- type construction;
- concurrency;
- quantum measurement;
- hardware structure.

The grammar SHOULD NOT become a catalog of every possible algorithm or library function.

---

75. Semantic Constants Versus Implementation Limits

The following is valid:

let n = 1000000
allocate qubits[n]

provided the type/semantic system permits it.

The following is prohibited as a universal language rule:

MAX_QUBITS = 1000000

Likewise:

Tensor<T, [1024, 1024]>

may be valid program semantics.

But:

all tensors <= 1024 × 1024

MUST NOT be a language rule.

---

76. Resource Scaling

All resource-related syntax MUST be scale-neutral.

A program may express:

workers = available_workers

rather than:

workers = 8

when its semantics permit dynamic scaling.

Likewise:

resources = discover(...)

MAY be represented where the runtime/resource model supports discovery.

---

77. Dynamic Resource Allocation

Where supported, resource syntax SHOULD allow resources to be:

- requested;
- acquired;
- released;
- partitioned;
- shared;
- migrated;
- scaled;
- negotiated.

The grammar MUST NOT require resources to be statically known unless the semantic construct explicitly requires static knowledge.

---

78. Resource Failure

Insufficient resources are not necessarily syntax errors.

For example:

requires qubits >= n

is syntactically valid regardless of whether the current machine has sufficient qubits.

The result of insufficient resources is determined during:

- semantic validation;
- compilation;
- scheduling;
- deployment;
- runtime;

according to the relevant subsystem.

---

79. Syntax Versus Semantics

The grammar answers:

«Is this source structurally valid Zamani syntax?»

Semantic analysis answers:

«What does it mean?»

Resource resolution answers:

«Can the requested computation be realized?»

Compilation answers:

«How should it be realized?»

Runtime answers:

«How should it execute now?»

Hardware/HAL answers:

«What does the actual target provide?»

These concerns MUST remain separated.

---

80. Syntax Versus AST

The AST is structural and domain-neutral.

Syntax MUST NOT require the AST to encode backend decisions.

For example:

apply H to q

may produce a generic operation node.

The AST MUST NOT directly encode:

physical_qpu = device_7
physical_qubit = 13
coupler = 2
pulse = ...

unless those are explicitly part of a target-specific downstream representation.

---

81. Syntax Versus IR

Syntax MUST lower into canonical semantic representations.

The grammar MUST NOT invent a new IR merely because a domain has new syntax.

For quantum:

syntax
 ↓
AST
 ↓
semantic quantum operation
 ↓
quantum::ir

For classical computation:

syntax
 ↓
AST
 ↓
classical semantic representation
 ↓
canonical classical IR

For HDL:

syntax
 ↓
AST
 ↓
hardware semantic representation
 ↓
HDL/hardware IR

---

82. Source-to-Target Contract

The complete pipeline is:

Zamani source
      ↓
lexer
      ↓
parser
      ↓
domain-neutral AST
      ↓
structural validation
      ↓
semantic analysis
      ↓
canonical semantic model
      ↓
canonical IR
      ↓
optimization
      ↓
resource/capability resolution
      ↓
routing / scheduling / resilience
      ↓
domain-specific realization
      ↓
HAL / backend
      ↓
execution

Syntax owns only the front portion of this pipeline.

---

83. Deterministic Parsing

The parser MUST behave deterministically for the same:

- source;
- language version;
- dialect set;
- feature configuration.

The parser MUST NOT depend on:

- current hardware;
- runtime state;
- network state;
- random values;
- current time.

---

84. Error Recovery

Parser error recovery MUST preserve useful diagnostics.

Diagnostics SHOULD identify:

- source location;
- expected syntax;
- encountered syntax;
- relevant context;
- stable error category/code where defined.

Recovery MUST NOT silently transform invalid source into valid but different semantics.

---

85. Ambiguity

The canonical grammar MUST avoid unresolved ambiguity.

Potential ambiguities involving:

- generic arguments;
- "<" and ">";
- operators;
- qualified names;
- macros;
- dialect extensions;
- quantum syntax;
- HDL syntax;

MUST be resolved explicitly.

---

86. Left Recursion

Grammar components MUST be compatible with the selected parser technology.

For ANTLR4 grammar composition, parser rules MUST be written in a form that the selected ANTLR implementation handles deterministically and correctly.

Any intentional precedence/left-recursive structure MUST be documented in:

grammar/expressions/precedence.md

---

87. Keyword Policy

Keywords MUST be reserved only when they have genuine language-level significance.

Domain libraries SHOULD NOT automatically become keywords.

For example, adding every quantum gate as a keyword is discouraged.

User-defined names SHOULD remain possible wherever the lexical rules permit them.

---

88. Operator Policy

Operators MUST have:

- one canonical spelling;
- one lexical interpretation;
- one precedence;
- one associativity;
- defined semantic meaning.

Different tokens MUST NOT accidentally represent the same operator semantics.

---

89. Compatibility

Syntax changes MUST follow:

new syntax
 ↓
version analysis
 ↓
compatibility classification
 ↓
migration/deprecation policy
 ↓
implementation
 ↓
conformance tests

Breaking syntax changes MUST NOT be introduced merely because a backend or implementation changed.

---

90. Feature Lifecycle

Every new syntax feature MUST have a status:

- "proposed"
- "experimental"
- "accepted"
- "stable"
- "deprecated"
- "removed"
- "historical"

A stable feature MUST have:

- syntax;
- lexer tokens;
- AST mapping;
- semantic rules;
- IR mapping;
- implementation status;
- positive tests;
- negative tests;
- boundary tests;
- scalability tests;
- compatibility status.

---

91. Feature Manifests

Where applicable, each substantial feature SHOULD have a machine-readable manifest under:

grammar/specification/features/

A feature manifest SHOULD identify:

id
name
status
version
syntax
grammar
lexer_tokens
ast_nodes
semantic_rules
ir_mapping
compiler_consumers
runtime_consumers
domain
capabilities
resource_requirements
negative_tests
boundary_tests
scalability_tests
compatibility
hard_coding_policy

This makes a feature independently completable.

---

92. Independent File Completion Contract

Every syntax component MUST have predetermined integration information.

Each grammar specification or grammar component SHOULD document:

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
Compatibility Tests
Determinism Tests
Hard-Coding Audit
Diagnostics
Security
Performance
Completion Criteria

A file is not complete merely because its grammar rule parses.

---

93. Hard-Coding Audit

Every syntax component MUST be audited for artificial limits.

The audit MUST check for concepts such as:

MAX_QUBITS
MAX_CPUS
MAX_CORES
MAX_THREADS
MAX_GPUS
MAX_FPGAS
MAX_NODES
MAX_MEMORY
MAX_STORAGE
MAX_TENSOR_RANK
MAX_TENSOR_ELEMENTS
MAX_REGISTER_WIDTH
MAX_VECTOR_WIDTH
MAX_TIMELINES
MAX_PROCESSES
MAX_ACCELERATORS

These MUST NOT become universal language restrictions.

The same audit MUST detect hard-coded physical identifiers such as:

GPU0
QPU0
QUBIT0
QUBIT1
NODE0
CORE0
MEMORY_BANK0

when used as universal language assumptions.

---

94. Legitimate Constants

The hard-coding prohibition does not prohibit ordinary program constants.

These are legitimate:

const iterations = 1000
const matrix_size = 4096
const qubits = 128
const timeout = 10s

provided they are program semantics.

The prohibition applies to artificial implementation-wide restrictions.

---

95. Security Boundary

Syntax MUST NOT silently grant authority.

For example, syntax representing:

network access
filesystem access
device access
secret access
hardware access

MUST integrate with the capability/effect/security system.

The existence of syntax MUST NOT imply unrestricted runtime permission.

---

96. Filesystem and Network Independence

The grammar/parser MUST NOT require arbitrary filesystem or network access to parse ordinary Zamani source.

Import resolution, package discovery, network access, or external resources belong to tooling/compiler/runtime policies.

---

97. Reproducibility

The same source with the same language version, dialect configuration, and declared semantic inputs MUST produce the same syntactic interpretation.

Source syntax MUST NOT depend on:

- random parser behavior;
- network ordering;
- filesystem enumeration order;
- hardware discovery;
- wall-clock time.

---

98. Performance

The grammar architecture SHOULD scale with program size.

Implementations SHOULD avoid unnecessary:

- repeated parsing;
- exponential ambiguity;
- global grammar scans;
- unbounded backtracking;
- domain-specific parser hacks.

Large programs SHOULD be supportable through:

- incremental parsing where appropriate;
- modular parsing;
- streaming;
- caching;
- source indexing;
- deterministic recovery.

These are implementation concerns and MUST NOT alter language semantics.

---

99. Tooling

Syntax design MUST support:

- syntax highlighting;
- formatting;
- language servers;
- completion;
- navigation;
- refactoring;
- diagnostics;
- documentation generation;
- static analysis.

Source spans and stable syntax categories are therefore part of the production contract.

---

100. Generated Grammar Policy

Generated files MUST NOT become independent authorities.

The dependency direction MUST remain clear:

normative specification
        ↓
canonical grammar
        ↓
generated artifacts

Generated output MUST NOT be edited manually unless the repository explicitly identifies it as source.

---

101. Canonical Grammar Composition

The preferred composition is:

Zamani.g4
│
├── lexer contracts
├── core grammar
├── declarations
├── types
├── expressions
├── statements
├── functions
├── modules
├── effects
├── memory
├── concurrency
├── classical
├── quantum
├── hybrid
├── hdl
├── hardware
├── distributed
├── ai
├── data
├── networking
├── security
├── resources
├── compile
├── execution
├── interoperability
├── dialects
├── macros
└── metaprogramming

The exact ANTLR import mechanism MUST be implemented consistently with the repository's parser architecture.

---

102. No Second Grammar Authority

The repository MUST NOT contain two independent canonical versions of Zamani syntax.

In particular:

grammar/Zamani.g4

and:

grammar/antlr/Zamani.g4

MUST NOT both claim authority.

If "grammar/antlr/" contains obsolete artifacts, they should be removed only after dependency/reference analysis confirms that they are unused.

---

103. Conformance With Rust Implementation

The grammar itself is language specification and is not Rust.

The reference compiler/parser implementation MUST target:

Rust 1.97

or:

Rust 1.97.1

and Rust 2021.

Zamani-owned Rust implementation code MUST NOT use:

unsafe

This prohibition applies to:

- compiler code;
- lexer;
- parser;
- AST;
- semantic analysis;
- grammar tooling;
- validators;
- test harnesses;
- generated Rust owned by the project;
- runtime components where governed by the same project safety policy.

---

104. External Dependencies

External dependencies MUST NOT redefine the Zamani language contract.

For the current Rust implementation baseline, existing dependencies such as:

- ANTLR Rust tooling;
- "serde";
- "thiserror";
- "anyhow";
- logging facilities;

are implementation mechanisms.

They do not become part of Zamani syntax.

---

105. OpenQASM Integration

OpenQASM support belongs under:

src/quantum/frontend/formats/openqasm/
grammar/interoperability/

OpenQASM syntax MUST be parsed as an external format and lowered into Zamani's semantic model.

OpenQASM MUST NOT redefine Zamani quantum syntax.

The OpenQASM frontend MUST NOT:

- restrict operations to H/X/CNOT;
- assume fixed qubit indices;
- automatically measure every qubit;
- represent unknown operations as comments;
- use fixed physical mappings.

---

106. QIR Integration

QIR is an interoperability/downstream representation.

It MUST NOT become the canonical source-language AST.

The relationship is:

Zamani
  ↓
Zamani semantic model
  ↓
quantum::ir
  ↓
QIR adapter where required

---

107. HDL Interoperability

HDL formats such as Verilog/SystemVerilog/VHDL-style ecosystems MAY be supported through interoperability adapters.

They MUST NOT become the canonical Zamani hardware semantic model.

---

108. Vendor Independence

Vendor-specific syntax MAY be introduced through explicit interoperability or dialect mechanisms.

It MUST NOT contaminate the portable core syntax.

For example, vendor-specific accelerator features SHOULD be represented through:

capability
dialect
interop
attribute
extension

rather than silently introducing universal keywords.

---

109. Portability Contract

The syntax is considered portable when a program does not need source modification merely because its target changes.

A conforming implementation SHOULD allow the same source to move among:

tiny CPU
large CPU system
GPU
FPGA
ASIC
QPU
HPC cluster
distributed system
edge device
cloud system
future computational target

when the program's semantic requirements can be satisfied.

---

110. POCO-REAF Interpretation

POCO-REAF does not mean that a single physical binary can magically execute on incompatible future machines without any realization layer.

It means:

1. source semantics are written once;
2. the source does not need hardware-specific rewrites;
3. semantic compilation produces a portable/stable representation;
4. target-specific realization happens through compiler/runtime infrastructure;
5. future targets can implement the established semantic contracts;
6. language compatibility is preserved through explicit versioning.

Therefore:

one source
    ↓
one semantic program
    ↓
many possible realizations

is the architectural objective.

---

111. Scaling Contract

The language MUST scale across:

program size
data size
memory size
compute capacity
parallelism
quantum resources
tensor dimensions
node count
device count
communication scale
storage
execution duration

subject to actual resource availability and semantic feasibility.

There MUST be no arbitrary syntax-level ceiling for these dimensions.

---

112. Tiny-to-Infinite Model

"Infinite" is interpreted as an architectural scalability objective, not a claim that finite physical machines can execute mathematically infinite work.

Zamani MUST permit:

- arbitrarily parameterized programs;
- scalable resource requests;
- lazy/streaming semantics where appropriate;
- symbolic dimensions;
- distributed decomposition;
- dynamic resource discovery;
- future target realization.

Actual execution remains bounded by physical and implementation resources.

---

113. Resource Discovery

Resource discovery belongs outside the parser.

Conceptually:

source
  ↓
resource intent
  ↓
resource discovery
  ↓
capability matching
  ↓
realization

The parser MUST NOT need to know whether the target has:

- 1 CPU;
- 1,000 CPUs;
- 1 QPU;
- 1,000 QPUs;
- 1 GB memory;
- 1 PB memory.

---

114. Resource Negotiation

Where deployment requires negotiation, syntax MAY express acceptable requirements and preferences.

For example:

requires capability("distributed.compute")
prefer locality("near-data")
prefer accelerator("tensor")

The actual realization belongs to deployment/resource management.

---

115. Scheduling Independence

Syntax MAY express scheduling intent:

parallel
pipeline
latency_bound
throughput_bound
priority

but MUST NOT directly encode the final physical schedule unless the construct explicitly belongs to a target-specific dialect.

The scheduling subsystem determines actual ordering/resource allocation.

---

116. Routing Independence

Quantum and distributed syntax MAY express connectivity requirements.

Physical routing MUST remain downstream.

A portable source program SHOULD describe:

logical operands
logical communication
connectivity requirements

rather than:

physical path 0 → 1 → 2

unless a target-specific realization is intentionally being written.

---

117. Resilience Independence

Syntax MAY express resilience policies.

The resilience subsystem determines:

- health state;
- recovery;
- escalation;
- acceptance;
- rejection;
- retry/recovery behavior.

The language MUST NOT duplicate resilience implementation logic.

---

118. Diagnostics Contract

Every parser diagnostic MUST be attributable to a source construct.

Diagnostics SHOULD be stable enough for tooling and tests.

Negative tests SHOULD verify:

- invalid tokens;
- malformed declarations;
- invalid expressions;
- ambiguous syntax;
- malformed generics;
- malformed quantum operations;
- invalid HDL constructs;
- invalid resource expressions;
- invalid dialect declarations.

---

119. Testing Contract

Every syntax feature MUST have:

Positive tests

Valid source must parse.

Negative tests

Invalid source must fail.

Boundary tests

Minimal and unusually large valid forms must be exercised.

Scalability tests

The feature must be tested with parameterized or very large values without establishing an artificial maximum.

Determinism tests

Repeated parsing must produce equivalent syntax/AST results.

Compatibility tests

Supported language versions must behave according to the compatibility contract.

---

120. Repository Integration Matrix

Syntax area| Primary grammar directory| Specification| AST| Semantic/IR
Lexical| "lexer/"| "lexical.md"| token layer| parser
Core| "core/"| "syntax.md"| core AST| semantic model
Expressions| "expressions/"| "syntax.md"| expression nodes| semantic IR
Types| "types/"| "types.md"| type nodes| type system
Statements| "statements/"| "syntax.md"| statement nodes| semantic model
Functions| "functions/"| "syntax.md"| function nodes| callable IR
Modules| "modules/"| "syntax.md"| module nodes| module system
Effects| "effects/"| "semantics.md"| effect metadata| effect system
Memory| "memory/"| "types.md" / "semantics.md"| resource metadata| memory model
Concurrency| "concurrency/"| "semantics.md"| concurrency nodes| execution IR
Classical| "classical/"| "spec/classical.md"| generic/domain nodes| classical IR
Quantum| "quantum/"| "spec/quantum.md"| generic operations| "quantum::ir"
Hybrid| "hybrid/"| "spec/"| shared AST| cross-domain IR
HDL| "hdl/"| "spec/hdl.md"| hardware nodes| HDL/hardware IR
Hardware| "hardware/"| "spec/resources.md"| intent metadata| target realization
Resources| "resources/"| "spec/resources.md"| resource metadata| resource manager
Distributed| "distributed/"| "spec/distributed.md"| distributed nodes| distributed IR
AI| "ai/"| "spec/ai.md"| model/tensor nodes| AI/data IR
Data| "data/"| "spec/"| data nodes| data IR
Networking| "networking/"| "spec/networking.md"| network nodes| network realization
Security| "security/"| "spec/security.md"| security metadata| security subsystem
Compile| "compile/"| "portability.md"| compile metadata| compiler
Execution| "execution/"| "semantics.md"| execution metadata| runtime
Interop| "interoperability/"| relevant spec| adapters| external IR
Dialects| "dialects/"| compatibility| extension nodes| dialect IR
Macros| "macros/"| syntax/semantics| expanded AST| compiler
Metaprogramming| "metaprogramming/"| semantics| generated AST| compiler

---

121. Canonical Ownership Boundaries

The following ownership rules are mandatory.

Lexer
    owns tokenization

Parser
    owns syntax recognition

AST
    owns source structure

Semantic analysis
    owns meaning

Type system
    owns type correctness

Effect system
    owns effects

Resource system
    owns resource requirements

Capability system
    owns capability requirements

Canonical IR
    owns semantic computational representation

Optimization
    owns implementation improvement

Routing
    owns physical realization

Scheduling
    owns timing/order/resource scheduling

QEC
    owns error correction

ZQN
    owns fault/noise semantics

HAL
    owns target capability/state

Runtime
    owns execution

No grammar component may silently assume ownership of another layer.

---

122. Completion Criteria for This File

"grammar/specification/syntax.md" is complete when:

- [x] syntax authority is defined;
- [x] relationship to "Zamani.g4" is defined;
- [x] relationship to "grammar.md" is defined;
- [x] relationship to "Zamani-Grammar.md" is defined;
- [x] AST boundary is defined;
- [x] semantic boundary is defined;
- [x] IR boundary is defined;
- [x] quantum syntax boundary is defined;
- [x] "quantum::ir" remains canonical;
- [x] resource syntax is target-independent;
- [x] capability syntax is target-independent;
- [x] requirement/constraint/preference/hint distinctions are defined;
- [x] no artificial hardware limits are permitted;
- [x] generic operations are supported;
- [x] domain composition is defined;
- [x] classical syntax integration is defined;
- [x] quantum syntax integration is defined;
- [x] hybrid syntax integration is defined;
- [x] HDL integration is defined;
- [x] hardware intent is defined;
- [x] distributed syntax is defined;
- [x] AI/data integration is defined;
- [x] networking/security integration is defined;
- [x] compilation/execution integration is defined;
- [x] interoperability is defined;
- [x] dialects are constrained;
- [x] macros/metaprogramming are constrained;
- [x] diagnostics are addressed;
- [x] determinism is addressed;
- [x] scalability is addressed;
- [x] POCO-REAF is formally interpreted;
- [x] hard-coding rules are explicit;
- [x] Rust 1.97/1.97.1 implementation baseline is recorded;
- [x] "unsafe" prohibition is recorded;
- [x] independent feature completion is supported;
- [x] downstream integration contracts are identified.

---

123. Required Follow-On Files

This document intentionally establishes the syntax architecture without duplicating all subordinate contracts.

The following files MUST refine it:

grammar/specification/lexical.md
grammar/specification/types.md
grammar/specification/semantics.md
grammar/specification/portability.md

grammar/spec/lexical.md
grammar/spec/syntax.md
grammar/spec/type-system.md
grammar/spec/semantics.md
grammar/spec/resources.md
grammar/spec/quantum.md
grammar/spec/classical.md
grammar/spec/hdl.md
grammar/spec/hybrid.md
grammar/spec/concurrency.md
grammar/spec/distributed.md
grammar/spec/ai.md
grammar/spec/data.md
grammar/spec/networking.md
grammar/spec/security.md
grammar/spec/interoperability.md
grammar/spec/diagnostics.md
grammar/spec/determinism.md
grammar/spec/versioning.md
grammar/spec/compatibility.md

The modular grammar implementation then belongs under:

grammar/lexer/
grammar/core/
grammar/types/
grammar/expressions/
grammar/statements/
grammar/declarations/
grammar/functions/
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
grammar/data/
grammar/networking/
grammar/security/
grammar/resources/
grammar/compile/
grammar/execution/
grammar/interoperability/
grammar/dialects/
grammar/macros/
grammar/metaprogramming/

Finally:

grammar/Zamani.g4

composes these contracts into the canonical ANTLR grammar.

---

124. Final Syntax Principle

The central rule of Zamani syntax is:

«Source code expresses computation, semantics, requirements, capabilities, constraints, preferences, and intent — not today's physical machine.»

Therefore Zamani source SHOULD say:

compute this

use this semantic operation

require this capability

require these resources

prefer this realization

preserve this correctness property

rather than:

use CPU 7

use GPU 3

use QPU 0

use physical qubit 17

use exactly 8 cores

use exactly 32 GB RAM

unless those concrete details are deliberately being expressed by a target-specific program or dialect.

The architecture is therefore:

                 ZAMANI SOURCE
                       │
                       ▼
              ┌─────────────────┐
              │  UNIVERSAL      │
              │  SYNTAX         │
              └─────────────────┘
                       │
                       ▼
                DOMAIN-NEUTRAL AST
                       │
                       ▼
              STRUCTURAL VALIDATION
                       │
                       ▼
               SEMANTIC ANALYSIS
                       │
          ┌────────────┼────────────┐
          ▼            ▼            ▼
      classical     quantum        HDL
          │            │            │
          │       quantum::ir       │
          │            │            │
          └────────────┼────────────┘
                       ▼
               CANONICAL IR LAYER
                       │
                       ▼
                 OPTIMIZATION
                       │
          ┌────────────┼─────────────┐
          ▼            ▼             ▼
       ROUTING      SCHEDULING    RESILIENCE
          │            │             │
          └────────────┼─────────────┘
                       ▼
                     ZQN
                       │
                       ▼
                     HAL
                       │
                       ▼
               TARGET REALIZATION
                       │
       ┌───────────────┼────────────────┐
       ▼               ▼                ▼
      CPU             GPU              QPU
       │               │                │
      FPGA            ASIC          DISTRIBUTED
       │               │                │
       └───────────────┼────────────────┘
                       ▼
                 FUTURE TARGETS

This is the syntax contract required for the rest of the grammar to scale from atom to everywhere without turning today's hardware, today's compiler, or today's quantum devices into permanent language limits.