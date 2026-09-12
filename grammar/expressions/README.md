Zamani Expression Grammar

Purpose

This directory defines the production-ready expression syntax of the Zamani programming language.

Expressions are syntactic constructs that produce, reference, transform, select, invoke, or otherwise compose values, computations, resources, capabilities, and domain-level expressions.

The expression grammar is designed for Zamani's universal computational model:

«One source program → one stable semantic meaning → many possible implementations and execution environments.»

The expression grammar MUST therefore describe language semantics and programmer intent without embedding accidental properties of a particular machine.

The expression grammar MUST support Zamani programs spanning:

- classical computing;
- quantum computing;
- hybrid quantum-classical computing;
- hardware description;
- hardware/software co-design;
- embedded systems;
- systems programming;
- distributed computing;
- parallel computing;
- HPC;
- AI/ML;
- numerical computing;
- symbolic computing;
- data processing;
- networking;
- cryptography;
- accelerators;
- cloud and edge execution;
- future computational domains.

This directory is part of the language grammar and is not an AST, semantic-analysis layer, IR, optimizer, scheduler, runtime, hardware abstraction, or execution engine.

---

1. Architectural Position

The expression grammar occupies the following position:

Source
  │
  ▼
Lexer
  │
  ▼
Parser / Grammar
  │
  ├── core/
  ├── types/
  ├── expressions/   ← this directory
  ├── statements/
  ├── declarations/
  ├── functions/
  ├── modules/
  └── domain grammar
  │
  ▼
AST / Syntax Representation
  │
  ▼
Name Resolution
  │
  ▼
Type / Effect / Capability Analysis
  │
  ▼
Semantic Representation
  │
  ├── Classical IR
  ├── quantum::ir
  ├── HDL / hardware representation
  ├── distributed representation
  └── other domain representations
  │
  ▼
Optimization
  │
  ▼
Routing / Scheduling
  │
  ▼
Hardware / Target Lowering
  │
  ▼
Runtime / Execution

The grammar MUST NOT reverse this dependency.

In particular:

grammar → semantic analysis
grammar → IR
grammar → optimization
grammar → scheduling
grammar → runtime
grammar → hardware discovery

is allowed through compilation architecture as a downstream relationship.

The following are prohibited:

IR → grammar
runtime → grammar
hardware discovery → grammar
scheduler → grammar
optimizer → grammar

The grammar may expose syntax required by those systems, but those systems must not become dependencies of the grammar definition itself.

---

2. Scope

This directory owns the syntax for expressions.

The directory contains the canonical expression entry point:

expressions.g4

and decomposes expression syntax into independently maintainable grammar modules.

The intended components include:

expressions.g4
literals.g4
identifiers.g4
unary.g4
binary.g4
arithmetic.g4
comparison.g4
logical.g4
bitwise.g4
assignment.g4
calls.g4
indexing.g4
member-access.g4
ranges.g4
conditionals.g4
lambdas.g4
comprehensions.g4
compile-time.g4

Additional expression grammar files may be introduced only when they establish a genuine ownership boundary.

No file should exist merely for organizational appearance.

---

3. Expression Ownership

3.1 This directory owns

This directory owns:

- expression syntax;
- expression composition;
- expression precedence;
- expression associativity;
- expression grammar entry points;
- expression-specific syntactic alternatives;
- expression delimiters where those delimiters belong to expression syntax;
- expression-level structural forms;
- syntactic composition of domain-independent and domain-specific expressions;
- syntactic representation of expression-oriented control constructs;
- expression-level compile-time forms;
- expression-level lambda syntax;
- expression-level calls;
- expression-level indexing;
- expression-level member access;
- expression-level ranges;
- expression-level operators.

---

4. Explicit Non-Ownership

This directory MUST NOT own:

- AST implementation;
- semantic analysis;
- type inference;
- type checking;
- ownership checking;
- borrowing;
- lifetime checking;
- effect checking;
- capability checking;
- resource allocation;
- hardware discovery;
- hardware topology;
- physical machine limits;
- quantum hardware limits;
- quantum backend selection;
- scheduling;
- routing;
- optimization;
- execution;
- runtime dispatch;
- QEC algorithms;
- ZQN fault semantics;
- canonical quantum IR;
- classical IR;
- HDL IR;
- machine-specific lowering;
- code generation;
- memory allocation;
- thread allocation;
- device allocation.

The grammar describes syntax.

Semantic subsystems interpret that syntax.

---

5. Canonical Expression Entry Point

"expressions.g4" is the canonical expression grammar entry point.

Other expression files MUST NOT independently define competing top-level expression grammars.

Conceptually:

expression
    : assignmentExpression
    ;

The exact grammar must be compatible with the authoritative Zamani syntax specification and the existing parser architecture.

The expression entry point must be stable enough that downstream parser/AST infrastructure can depend on it without knowing how expression subcategories are internally organized.

---

6. Expression Precedence

Expression precedence MUST be explicit and deterministic.

The grammar must establish a single precedence model for:

1. primary expressions;
2. member access;
3. indexing;
4. calls;
5. postfix constructs;
6. unary operators;
7. exponentiation or other high-precedence operators where defined;
8. multiplicative operators;
9. additive operators;
10. shifts;
11. relational operators;
12. equality operators;
13. bitwise operators;
14. logical operators;
15. ranges;
16. conditional expressions;
17. assignment expressions.

The exact ordering MUST follow the authoritative Zamani language specification.

No individual expression subgrammar may silently establish a competing precedence model.

---

7. Associativity

Associativity MUST be explicitly defined for every binary operator class.

The grammar MUST NOT rely on accidental parser-generator behavior.

For example, an operator must have one explicitly defined semantic grammar policy:

left associative

or:

right associative

or:

non-associative

Operators whose chaining would be semantically ambiguous must be rejected syntactically or represented in a way that permits semantic validation.

---

8. Operator Ownership

Operators must have exactly one syntactic owner.

Operator spelling belongs to the lexical specification where applicable.

Operator composition belongs here.

Semantic meaning does not belong here.

For example:

+
-
*
/
%
==
!=
<
<=
>
>=
&&
||
!
~
&
|
^
<<
>>

may be lexed by "lexer/", while their expression-level precedence and composition are defined by the expression grammar.

The grammar MUST NOT attach machine-specific behavior to an operator.

---

9. No Machine-Size Assumptions

Expressions MUST NOT contain machine-size assumptions.

The grammar must not contain restrictions such as:

MAX_BITS = 64
MAX_ELEMENTS = 1024
MAX_DIMENSIONS = 32
MAX_QUBITS = 64
MAX_THREADS = 1024

or equivalent parser alternatives.

Likewise, the grammar must not encode:

q[0]
q[1]

as special quantum cases.

Indexing is generic expression syntax.

If a program refers to a quantum element, tensor element, array element, register element, hardware resource, distributed object, or other indexed entity, the expression grammar must parse the general expression.

Resource availability and validity are determined later.

---

10. Infinite-Scale Principle

"Infinity" in the Zamani scalability requirement means that the grammar must not impose an artificial finite machine-resource ceiling.

Actual execution is naturally bounded by:

- available memory;
- available compute;
- available storage;
- available quantum resources;
- available network resources;
- available compilation resources;
- implementation limits;
- target capabilities;
- runtime policies.

Those limitations MUST NOT be confused with grammar limits.

The grammar must therefore permit expression structures whose size is bounded only by the parser/compiler implementation and available resources rather than arbitrary Zamani machine constants.

---

11. Primary Expressions

Primary expressions form the foundation of expression composition.

They may include:

- literals;
- identifiers;
- qualified names;
- parenthesized expressions;
- tuples;
- arrays;
- maps;
- structured literals;
- lambda expressions;
- domain-specific expression forms;
- compile-time expressions where specified.

Primary expression syntax MUST remain independent of execution targets.

---

12. Identifiers

Identifier syntax belongs to the appropriate canonical identifier grammar.

"expressions/identifiers.g4" must not redefine the language's global identifier rules if "core/names.g4" already owns them.

The expression layer may reference the canonical identifier rule.

This avoids incompatible definitions between:

core/names.g4
expressions/identifiers.g4
declarations/
functions/
modules/

The repository must establish one authoritative identifier contract.

---

13. Literals

Literal syntax must be delegated to the appropriate literal grammar.

The expression grammar may consume:

- numeric literals;
- string literals;
- character literals;
- Boolean literals;
- quantum literals;
- duration literals;
- size literals;
- hardware-related literal forms;
- future literal categories.

Literal interpretation belongs to semantic/type layers.

For example, a numeric literal does not inherently mean:

CPU integer

It may participate in:

- classical arithmetic;
- arbitrary-precision arithmetic;
- tensor dimensions;
- compile-time computation;
- quantum parameters;
- resource expressions;
- hardware parameters;

depending on semantic context.

---

14. Unary Expressions

Unary expressions must support the operators defined by the Zamani language specification.

Examples may include:

+
-
!
~

Additional domain-specific unary operators may be introduced through the dialect mechanism.

The grammar must not encode a finite list of future machine capabilities.

---

15. Binary Expressions

Binary expressions must be structured according to the canonical precedence table.

Binary operators include categories such as:

- arithmetic;
- comparison;
- logical;
- bitwise;
- shifts;
- domain-defined operators where explicitly supported.

Binary syntax does not determine whether an operation executes:

- on a CPU;
- on a GPU;
- on an FPGA;
- on a quantum processor;
- in a distributed environment;
- symbolically;
- at compile time;
- through an accelerator.

That decision belongs downstream.

---

16. Assignment Expressions

Assignment syntax belongs to the expression layer only where Zamani defines assignment as an expression.

The grammar must distinguish assignment syntax from:

- declaration syntax;
- initialization syntax;
- mutation semantics;
- ownership semantics;
- memory allocation.

The semantic layer determines whether an assignment is legal.

The grammar determines only whether its structure is syntactically valid.

---

17. Calls

Call syntax must support arbitrary callable expressions rather than only identifiers.

Conceptually:

call
    : expression argumentList
    ;

This allows callable values, functions, closures, generic instantiations, member functions, and future callable abstractions.

The grammar must not restrict the number of arguments.

Argument count and type compatibility are semantic concerns.

---

18. Generic Calls

Where Zamani supports generic invocation, generic arguments must integrate with the canonical type/generic grammar.

The expression grammar must not duplicate generic type syntax.

Generic invocation must remain compatible with:

types/
functions/
declarations/

and semantic specialization.

---

19. Indexing

Indexing must be generic.

It must support expressions as indices where the language permits.

For example:

value[index]

must not be restricted to a fixed integer range.

This permits:

array[i]
matrix[row, column]
tensor[index]
register[position]
qubits[index]
resource[index]

without creating separate machine-specific grammar rules.

The number and type of indices are semantic concerns unless the language explicitly defines syntax-level restrictions.

---

20. Member Access

Member access must support expressions whose resulting value has members.

Examples include:

value.member
value.method(...)

The grammar must not enumerate all possible members.

Member existence is determined during name resolution/type checking.

This is essential for extensibility and dialect support.

---

21. Ranges

Range expressions must support the forms defined by the language.

A range must not encode a fixed maximum.

Conceptually:

start .. end
start ..= end

or whatever forms the authoritative specification defines.

Range semantics must remain independent of:

- processor width;
- memory capacity;
- array capacity;
- qubit count;
- machine size.

---

22. Conditional Expressions

If/then/else expression syntax belongs here when Zamani supports expression-oriented conditionals.

For example:

if condition then_value else else_value

The exact syntax must follow the authoritative language specification.

Conditional expression semantics must remain independent of execution architecture.

---

23. Lambda Expressions

Lambda syntax belongs to "lambdas.g4".

The expression entry point delegates to lambda syntax.

Lambda grammar must integrate with:

functions/parameters.g4
functions/generics.g4
types/function-types.g4
statements/blocks.g4
memory/ownership.g4
effects/

without importing semantic rules into the grammar.

Capture semantics, ownership, effects, lifetimes, and execution strategy are not grammar responsibilities.

---

24. Comprehensions

Comprehension syntax may support:

- collection expressions;
- filtering;
- transformation;
- nested iteration;
- domain-specific collection construction.

Comprehensions must not assume a fixed collection size.

They must remain usable for:

- classical collections;
- distributed collections;
- tensors;
- data streams;
- future collection abstractions.

---

25. Compile-Time Expressions

Compile-time expression syntax belongs to:

compile-time.g4

The grammar must distinguish syntactic compile-time constructs from ordinary runtime expressions.

Compile-time execution MUST NOT implicitly mean:

machine discovery

or:

runtime hardware binding

unless explicitly specified by the language.

Compile-time evaluation must have deterministic and auditable semantics.

---

26. Quantum Integration

Quantum expression syntax must remain generic.

Expressions may refer to:

- qubits;
- logical qubits;
- quantum registers;
- measurements;
- observables;
- parameters;
- quantum operations;
- classical conditions.

However, the expression grammar must not define quantum semantics.

For example, it may parse an expression equivalent to:

measure(qubit)

but it must not determine:

- how measurement is implemented;
- which QPU is selected;
- how many physical qubits exist;
- routing;
- scheduling;
- calibration;
- error correction;
- noise;
- mitigation.

Those belong to downstream quantum infrastructure.

The canonical quantum semantic boundary remains:

quantum::ir

The grammar must lower into the frontend/semantic pipeline that produces the canonical IR rather than creating a second quantum representation.

---

27. Classical Integration

Expressions must be capable of supporting classical computation without limiting future computational domains.

Classical expressions may participate in:

- scalar operations;
- vector operations;
- matrix operations;
- tensor operations;
- symbolic operations;
- numerical operations;
- control flow;
- accelerator invocation;
- compile-time computation.

The grammar must not require a particular machine representation.

---

28. Hybrid Quantum-Classical Integration

Expressions must allow classical and quantum computations to interact where permitted by the language semantics.

Examples include:

classical_condition
measurement_result
quantum_parameter
classical_parameter

The grammar must parse these relationships without deciding whether execution occurs:

- synchronously;
- asynchronously;
- locally;
- remotely;
- on a simulator;
- on quantum hardware.

That is determined downstream.

---

29. HDL Integration

Expression syntax may appear inside HDL constructs for:

- parameter expressions;
- widths;
- timing expressions;
- conditions;
- state transitions;
- hardware computations;
- compile-time hardware configuration.

The expression grammar must remain generic.

It must not encode fixed hardware widths.

For example, an HDL expression must be able to reference a parameterized width rather than forcing:

width = 32

or:

width = 64

into grammar semantics.

---

30. Hardware Integration

Expressions may participate in hardware/resource declarations and constraints.

However:

expression

must not become:

hardware discovery

The grammar may represent an expression such as a resource requirement.

Semantic/resource layers determine whether a target satisfies it.

---

31. Resource Expression Integration

Expression syntax is used by:

resources/

for expressions involving:

- quantities;
- constraints;
- requirements;
- preferences;
- capabilities;
- performance;
- latency;
- energy;
- reliability;
- scalability;
- portability.

These must remain semantic concepts.

The expression grammar provides reusable syntax; the resource subsystem owns their meaning.

---

32. Effects and Capabilities

Expressions may occur in effect/capability declarations or expressions where specified.

The grammar must not decide whether an expression:

- performs I/O;
- accesses hardware;
- invokes quantum computation;
- communicates over a network;
- mutates memory;
- consumes a resource.

Effect and capability analysis determines those properties.

---

33. Type Integration

Expression syntax must integrate with the canonical type grammar.

The expression layer may consume:

types/types.g4
types/function-types.g4
types/generic-types.g4
types/type-constraints.g4

but must not redefine those rules.

Type compatibility is a semantic responsibility.

---

34. Memory Integration

Expression syntax may appear in:

- allocation expressions;
- references;
- pointer-like constructs if supported;
- ownership-aware expressions;
- borrowing expressions.

The grammar must not implement ownership checking.

Ownership and borrowing are semantic analyses.

---

35. Concurrency Integration

Expressions may be used in:

- task creation;
- futures;
- channels;
- synchronization expressions;
- parallel constructs;
- actor operations.

The grammar must not determine:

- number of threads;
- number of workers;
- number of nodes;
- scheduling policy;
- processor affinity.

Those are execution/resource concerns.

---

36. Distributed Integration

Expressions may reference distributed abstractions.

The grammar must not encode:

node 0
node 1
node 2

as fixed architectural concepts.

Node selection, placement, replication, communication, and consistency belong to the distributed subsystem.

---

37. AI/Data Integration

Expression syntax must support expressions used by:

- tensor operations;
- model invocation;
- dataset transformations;
- inference;
- training;
- differentiation;
- pipelines.

Tensor rank, dimension, storage, placement, and accelerator choice are not grammar-level machine limits.

---

38. Networking Integration

Networking expressions may represent:

- endpoints;
- messages;
- protocols;
- channels;
- service calls.

Network topology and deployment remain outside the grammar.

---

39. Security Integration

Expressions may participate in:

- permission checks;
- capability expressions;
- identity expressions;
- cryptographic operations;
- policy expressions.

The grammar must not hard-code providers, credentials, keys, devices, or deployment-specific identities.

---

40. Dialect Integration

Domain-specific expression syntax must be extensible through the dialect system.

A dialect must not modify the meaning of core expressions invisibly.

Dialect extensions require:

- namespace ownership;
- versioning;
- registration;
- compatibility information;
- capability declaration;
- explicit syntax ownership.

Experimental syntax must be distinguishable from stable language syntax.

---

41. Error Handling

The grammar must fail deterministically on invalid expression syntax.

Parser errors must preserve sufficient source information for downstream diagnostics.

The expression grammar must not:

- silently reinterpret invalid expressions;
- emit semantic comments as substitutes for syntax;
- silently discard unknown operators;
- silently convert malformed expressions into valid ones.

Error recovery should be controlled by the parser infrastructure rather than by semantic hacks embedded in grammar rules.

---

42. Diagnostics

Expression parsing must provide sufficient source locations for:

- operator errors;
- unmatched delimiters;
- malformed calls;
- malformed indexing;
- malformed ranges;
- malformed conditional expressions;
- malformed lambdas;
- invalid expression nesting.

Diagnostics should be deterministic.

The grammar itself should not invent provider-specific semantic diagnostics.

---

43. Ambiguity Policy

The expression grammar must be designed to minimize ambiguity.

Where ambiguity cannot be removed syntactically, the language specification must define the deterministic interpretation.

Every ambiguous construct must have:

- documented precedence;
- documented associativity;
- documented disambiguation;
- parser tests.

No ambiguity may depend on parser implementation accidents.

---

44. ANTLR Integration

The grammar is intended for ANTLR-based parsing.

All expression grammar files must be compatible with the repository's selected ANTLR version and grammar architecture.

The grammar must not depend on Rust implementation details.

Generated parser code is an artifact.

It is not the semantic source of truth.

The grammar source remains authoritative.

ANTLR-generated code must not be manually modified.

---

45. Rust Integration

The repository's implementation target is:

Rust 1.97

or:

Rust 1.97.1

The grammar itself must remain language-tool independent where possible.

Rust code integrating the generated parser must:

- use safe Rust;
- contain no "unsafe";
- avoid machine-specific assumptions;
- avoid fixed resource limits;
- preserve source locations;
- expose deterministic parsing behavior;
- integrate with existing AST/frontend infrastructure.

The grammar must not require unsafe Rust to represent arbitrary expression structures.

---

46. AST Contract

The grammar produces parser structures consumed by the AST/frontend layer.

The AST layer owns:

- expression node representation;
- source spans;
- parsed operator representation;
- literal representation;
- call representation;
- indexing representation;
- lambda representation;
- conditional representation;
- expression metadata.

The grammar does not define Rust structs.

There must be a one-to-one documented mapping between grammar alternatives and AST expression categories wherever practical.

---

47. Semantic Contract

The semantic layer interprets parsed expressions.

It owns:

- name resolution;
- type inference;
- type checking;
- constant evaluation;
- effect analysis;
- capability analysis;
- ownership analysis;
- resource validation;
- domain validation;
- quantum semantic validation;
- hardware validation.

The grammar MUST NOT perform these tasks.

---

48. IR Contract

Expressions do not constitute an IR.

After semantic analysis:

AST expression
      │
      ▼
semantic interpretation
      │
      ├── classical IR
      ├── quantum::ir
      ├── hardware/HDL representation
      ├── distributed representation
      └── other domain IR

The grammar must never become a competing canonical semantic representation.

For quantum constructs specifically:

Zamani expression syntax
        ↓
frontend / semantic lowering
        ↓
quantum::ir

must remain the architectural boundary.

---

49. Optimization Contract

Optimization consumes semantic/IR representations.

Expression grammar must not:

- perform constant folding;
- cancel operations;
- rewrite quantum gates;
- optimize tensor operations;
- schedule operations;
- perform peephole optimization.

Those belong to optimization infrastructure.

---

50. Scheduling Contract

The expression grammar has no dependency on scheduling.

Expressions may express computations that later become scheduled operations.

Scheduling determines:

- ordering;
- timing;
- resource usage;
- synchronization;
- placement;
- delays;
- execution strategy.

No scheduler implementation detail may leak into expression grammar.

---

51. Hardware Contract

The expression grammar does not discover hardware.

Hardware abstraction determines:

- capabilities;
- available resources;
- supported operations;
- topology;
- target properties;
- calibration;
- runtime state.

Expressions express intent.

Hardware determines realization.

---

52. QEC and ZQN Contract

Expression grammar does not implement:

- quantum error correction;
- fault diagnosis;
- noise models;
- mitigation;
- recovery.

Quantum expression syntax may provide the source constructs consumed by those systems.

The downstream architecture remains:

Grammar
  ↓
AST
  ↓
Semantic analysis
  ↓
quantum::ir
  ↓
QEC / ZQN / optimization / routing / scheduling
  ↓
hardware/runtime

QEC and ZQN must not become grammar dependencies.

---

53. POCO-REAF Contract

Expressions must preserve the POCO-REAF principle:

Program Once
      ↓
Compile Once
      ↓
Run Everywhere
      ↓
Run Anywhere
      ↓
Run Forever

An expression must describe what computation means rather than permanently bind it to:

- a CPU model;
- GPU model;
- FPGA family;
- ASIC;
- QPU;
- simulator;
- cloud provider;
- node count;
- machine topology;
- physical address.

Target-specific information must be introduced through explicit target/resource/capability mechanisms.

---

54. Portability Rules

Portable expressions MUST NOT implicitly depend on:

- machine word size;
- endian representation;
- fixed register count;
- fixed vector width;
- fixed processor count;
- fixed accelerator count;
- fixed quantum processor size;
- fixed topology;
- fixed memory size.

When representation matters semantically, it must be explicit in the language type or resource model.

When representation does not matter semantically, it must remain implementation-defined or target-selected.

---

55. Determinism

Given identical:

- source;
- grammar version;
- parser configuration;
- language mode;

the parser must produce deterministic syntax results.

Expression parsing must not depend on:

- hardware;
- network availability;
- runtime state;
- random numbers;
- wall-clock time;
- device discovery.

---

56. Compatibility

Expression syntax is part of the public language contract.

Changes require:

- version classification;
- compatibility analysis;
- migration documentation where necessary;
- parser regression tests;
- negative tests for removed syntax;
- preservation of valid existing syntax unless intentionally deprecated.

Breaking changes must not be introduced merely because an implementation detail changed.

---

57. Reserved Syntax

Expression syntax must leave deliberate extension space for future Zamani capabilities.

Reserved syntax must be documented through:

grammar/compatibility/
grammar/specification/reserved-space.md
grammar/dialects/

Reserved syntax must not accidentally become valid semantics.

---

58. Security

Expression parsing must be safe.

The parser must not:

- execute arbitrary code;
- access files;
- access networks;
- query hardware;
- invoke external processes;
- perform runtime resource allocation.

Compile-time execution, if supported, must have a separate explicitly controlled execution model.

Parsing source code must remain a non-executing operation.

---

59. Resource Exhaustion

The parser must not impose arbitrary language-level machine limits.

However, the parser implementation may have operational safeguards against denial-of-service-style pathological input.

Such safeguards must be:

- implementation-level;
- configurable where appropriate;
- documented;
- separate from language semantics;
- distinguishable from grammar restrictions.

For example, a parser safety limit is not equivalent to:

Zamani supports only N nested expressions.

The former is an implementation/resource policy.

The latter is a language restriction.

---

60. Testing Requirements

Every expression grammar component must have dedicated tests.

Tests must include:

Positive tests

- literals;
- identifiers;
- unary expressions;
- binary expressions;
- assignments;
- calls;
- indexing;
- member access;
- ranges;
- conditionals;
- lambdas;
- comprehensions;
- compile-time expressions.

Negative tests

- malformed operators;
- invalid precedence structures;
- malformed calls;
- missing delimiters;
- malformed indexing;
- malformed ranges;
- malformed lambdas;
- invalid expression endings;
- invalid chaining.

Boundary tests

Tests must exercise:

- very small expressions;
- deeply nested expressions;
- long operator chains;
- large argument lists;
- large index expressions;
- large collection expressions;
- large generic expressions;
- large source files.

Tests must verify that no artificial machine-size limit is encoded by grammar rules.

---

61. Cross-Domain Tests

Expression tests must include combinations such as:

classical + quantum
classical + HDL
quantum + HDL
quantum + hardware
quantum + distributed
AI + quantum
AI + hardware
classical + quantum + distributed
classical + quantum + HDL + hardware

These tests verify that expression syntax remains composable across domains.

---

62. Scalability Tests

Scalability tests must verify that expression syntax does not contain assumptions about:

- qubit count;
- core count;
- thread count;
- node count;
- accelerator count;
- memory capacity;
- vector width;
- tensor dimensions;
- topology;
- hardware identity.

The tests should use generated/property-based inputs where appropriate rather than relying only on hand-written examples.

---

63. Round-Trip Tests

Where a canonical formatter/printer exists:

Source
  ↓
Lexer
  ↓
Parser
  ↓
AST
  ↓
Printer
  ↓
Parser

must preserve semantic structure.

Formatting changes are permitted.

Semantic changes are not.

---

64. Repository Integration

This directory integrates with:

grammar/lexer/
grammar/core/
grammar/types/
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

The relationship is syntactic composition.

These directories must not duplicate the canonical expression grammar.

---

65. Integration With Repository Frontends

The grammar must integrate with the existing Zamani frontend/parser architecture.

Existing frontend code must consume the canonical parser representation rather than inventing a separate expression parser for each domain.

If an existing frontend has custom expression parsing, it must be classified as:

- retained because it implements a genuinely external format;
- migrated to the Zamani parser;
- adapted as an importer;
- deprecated;
- removed.

It must not silently become a second Zamani expression language.

---

66. External Languages

External language grammars such as:

OpenQASM
Verilog
C
C++
Python

must remain interoperability/import grammars.

They must not redefine Zamani expression syntax.

Their expressions may be transformed into Zamani semantic structures through explicit import/lowering mechanisms.

---

67. Generated Artifacts

Generated ANTLR parser/lexer artifacts must not be treated as hand-authored grammar sources.

The repository must document:

- generation command;
- generator version;
- source grammar;
- generated artifact policy;
- reproducibility;
- CI verification.

Generated output must be reproducible from the grammar source.

---

68. File-Level Ownership

"expressions.g4"

Owns:

- canonical expression entry point;
- expression precedence;
- expression composition;
- references to expression subrules.

Does not own:

- individual literal definitions;
- global identifier definitions;
- semantic typing;
- IR.

"literals.g4"

Owns:

- expression-level literal aggregation.

It delegates literal details to canonical lexer/literal rules.

"identifiers.g4"

Owns only expression-context identifier references if such a separate rule is necessary.

It must not redefine global identifier syntax.

"unary.g4"

Owns unary-expression composition.

"binary.g4"

Owns binary-expression composition and shared precedence structure where appropriate.

"arithmetic.g4"

Owns arithmetic operator syntax.

"comparison.g4"

Owns comparison operator syntax.

"logical.g4"

Owns logical operator syntax.

"bitwise.g4"

Owns bitwise operator syntax.

"assignment.g4"

Owns assignment-expression syntax.

"calls.g4"

Owns call-expression syntax.

"indexing.g4"

Owns indexing-expression syntax.

"member-access.g4"

Owns member-access syntax.

"ranges.g4"

Owns range-expression syntax.

"conditionals.g4"

Owns conditional-expression syntax.

"lambdas.g4"

Owns lambda-expression syntax.

"comprehensions.g4"

Owns comprehension syntax.

"compile-time.g4"

Owns compile-time expression syntax.

---

69. Dependency Rule

The expression directory follows:

lexer
  ↓
core
  ↓
types
  ↓
expressions
  ↓
statements/declarations/functions
  ↓
domain syntax
  ↓
semantic analysis

where the actual repository grammar dependency graph may refine this ordering.

Expression files may depend on lower-level syntax contracts.

They must not depend on higher-level semantic implementations.

---

70. No Circular Dependencies

The following dependency cycles are forbidden:

expressions → quantum → expressions
expressions → hardware → expressions
expressions → runtime → expressions
expressions → IR → expressions
expressions → scheduler → expressions

Domain grammars may consume expression rules.

Expressions remain domain-independent at their core.

---

71. Hard-Coding Audit

Every expression grammar change must be reviewed for:

- fixed maximums;
- fixed index counts;
- fixed operand counts;
- fixed tensor dimensions;
- fixed qubit counts;
- fixed machine widths;
- fixed register counts;
- fixed resource counts;
- fixed device identifiers;
- fixed hardware topology;
- fixed deployment assumptions.

Each discovered constant must be classified as:

1. language semantic requirement;
2. lexical requirement;
3. parser implementation safeguard;
4. target/resource constraint;
5. test fixture;
6. documentation example;
7. accidental hard-coding.

Accidental hard-coding MUST be removed.

---

72. Documentation Contract

The expression grammar must be documented consistently in:

grammar/README.md
grammar/Zamani-Grammar.md
grammar/grammar.md
grammar/specification/syntax-model.md
grammar/specification/semantic-model.md

No documentation file may independently redefine expression syntax.

Where documentation is derived from grammar, the derivation relationship must be explicit.

---

73. Versioning

Expression syntax must be version-aware.

Language versions must be represented through the language's canonical version mechanism rather than scattered grammar constants.

Version changes must specify whether an expression feature is:

- stable;
- experimental;
- deprecated;
- reserved;
- removed.

---

74. Extension Model

Future expression capabilities should be added through one of:

1. extension of the core expression grammar;
2. explicitly registered dialect syntax;
3. domain grammar that consumes canonical expressions.

A new computational paradigm must not require rewriting unrelated expression syntax.

For example, introducing a future accelerator model should not require changing arithmetic expressions merely because execution occurs on a new device.

---

75. Semantic Stability

The expression grammar must prioritize semantic stability over implementation convenience.

A source expression should retain the same language-level meaning when compiled for:

small embedded machine
CPU
multicore CPU
GPU
FPGA
ASIC
QPU
quantum simulator
accelerator
cluster
supercomputer
cloud
future architecture

provided that the target satisfies the program's semantic requirements.

---

76. Completion Criteria

"expressions/README.md" is complete when:

- expression ownership is explicitly defined;
- non-ownership is explicit;
- expression precedence has one authoritative source;
- associativity is deterministic;
- all expression subfiles have defined ownership;
- identifier ownership is resolved;
- literal ownership is resolved;
- type integration is defined;
- lambda integration is defined;
- compile-time integration is defined;
- domain integration is defined;
- quantum integration is defined;
- "quantum::ir" remains the canonical quantum semantic boundary;
- no expression grammar defines machine limits;
- no expression grammar defines hardware topology;
- no expression grammar defines runtime behavior;
- ANTLR integration is defined;
- AST integration is defined;
- semantic integration is defined;
- IR integration is defined;
- optimization integration is defined;
- scheduling integration is defined;
- hardware integration is defined;
- runtime integration is defined;
- dialect integration is defined;
- interoperability boundaries are defined;
- security constraints are defined;
- determinism requirements are defined;
- compatibility requirements are defined;
- scalability requirements are defined;
- hard-coding auditing is defined;
- positive tests are defined;
- negative tests are defined;
- boundary tests are defined;
- cross-domain tests are defined;
- round-trip tests are defined;
- generated-artifact policy is defined;
- no circular grammar architecture exists.

---

77. Production-Readiness Gate

No expression grammar file may be considered production-ready merely because ANTLR accepts it.

A file is production-ready only when all of the following are true:

Syntax correctness
        +
Ownership correctness
        +
Precedence correctness
        +
AST compatibility
        +
Semantic compatibility
        +
Cross-domain compatibility
        +
Scalability
        +
Determinism
        +
Compatibility
        +
Diagnostics
        +
Security
        +
Testing
        +
Documentation
        +
Integration verification
        =
Production Ready

---

78. Final Architectural Principle

The expression grammar exists to express computation.

It does not exist to describe the machine currently executing the computation.

Therefore:

Zamani expression
        ↓
stable language meaning
        ↓
semantic interpretation
        ↓
target-independent representation
        ↓
target/resource-aware compilation
        ↓
hardware/runtime realization

The fundamental rule is:

«Expressions describe what the programmer means, not how many resources happen to be available today.»

This is essential to:

Zamani — From Atom to Everywhere

and to:

POCO-REAF

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.