Zamani Expression Grammar

Path: "grammar/expressions/README.md"
Language: Zamani
Repository: "Benwellonedge28/Zamani"
Grammar technology: ANTLR4
Rust baseline: Rust 1.97 / Rust 1.97.1, Edition 2021
Rust safety requirement: Zamani compiler/frontend implementation uses safe Rust only; "unsafe" is prohibited.
Portability objective: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF).
Scalability objective: The language imposes no artificial finite machine, collection, expression, dimension, topology, device, memory, processor, qubit, tensor-rank, register-width, or network-size ceiling. Actual feasibility is determined by program semantics, implementation capabilities, declared requirements, policies, and available resources.

---

1. Purpose

"grammar/expressions/" is the authoritative modular grammar subsystem for expression syntax in Zamani.

An expression is a syntactic construct that can produce, reference, transform, combine, select, invoke, construct, constrain, or otherwise participate in a computation.

The expression subsystem is deliberately domain-neutral.

It must support expression semantics used by:

- classical computation;
- systems programming;
- embedded systems;
- scientific computing;
- symbolic computation;
- numerical computation;
- vector, matrix, and tensor computation;
- parallel computation;
- HPC;
- distributed computation;
- networking;
- data processing;
- AI/ML;
- reasoning and inference;
- probabilistic computation;
- quantum computation;
- hybrid quantum-classical computation;
- HDL and hardware/software co-design;
- accelerators;
- cryptography and security;
- simulation;
- metaprogramming;
- interoperability;
- future computational domains.

The expression grammar describes source-language structure.

It does not decide how that structure is executed.

---

2. Fundamental Rule

The expression grammar describes what an expression means structurally, not where or how it executes.

Therefore:

source
  ↓
lexer
  ↓
expression grammar
  ↓
domain-neutral AST
  ↓
structural validation
  ↓
name resolution
  ↓
type analysis
  ↓
effect analysis
  ↓
capability analysis
  ↓
resource analysis
  ↓
contract analysis
  ↓
policy analysis
  ↓
semantic representation
  ↓
canonical IR
  ├── classical/domain IR
  └── quantum::ir
  ↓
optimization
  ↓
lowering
  ↓
routing / scheduling where applicable
  ↓
resilience / recovery where applicable
  ↓
ZQN where applicable
  ↓
HAL
  ↓
target realization

No expression grammar file may bypass this architecture.

---

3. Expression Subsystem Responsibilities

This directory owns:

- expression composition;
- expression precedence;
- expression associativity;
- expression-level operators;
- primary expressions;
- prefix expressions;
- postfix expressions;
- calls;
- indexing;
- member access;
- assignment expressions;
- conditional expressions;
- range expressions;
- lambda expressions;
- closure expressions;
- collection expressions;
- comprehension expressions;
- match expressions when they occur in expression position;
- expression-level asynchronous constructs;
- expression-level compile-time constructs;
- expression-level domain composition.

This directory does not own:

- lexical token definitions;
- identifier definitions;
- type definitions;
- declarations;
- function declarations;
- module declarations;
- statement syntax;
- semantic type checking;
- ownership checking;
- borrowing;
- lifetime analysis;
- effect inference;
- capability resolution;
- resource discovery;
- resource allocation;
- hardware topology;
- target selection;
- quantum physical mapping;
- QEC;
- routing;
- scheduling;
- calibration;
- optimization;
- runtime execution;
- code generation;
- ABI implementation;
- HAL implementation.

---

4. Repository Authority Model

The expression grammar is part of a larger language architecture.

The authority relationship is:

grammar/DESIGN.md
        │
        ▼
grammar/specification/
        │
        ▼
grammar/spec/
        │
        ▼
grammar/expressions/
        │
        ▼
grammar/Zamani.g4
        │
        ▼
generated parser
        │
        ▼
src/lexer.rs / src/parser.rs
        │
        ▼
frontend AST
        │
        ▼
semantic analysis
        │
        ▼
canonical IR

4.1 "grammar/DESIGN.md"

Owns repository-wide grammar architecture.

It defines:

- authority;
- modular grammar architecture;
- domain neutrality;
- AST boundaries;
- semantic boundaries;
- IR boundaries;
- portability;
- scalability;
- compatibility;
- hard-coding prohibitions.

Expression files must comply with it.

---

4.2 "grammar/specification/"

Owns normative human-readable language specifications.

Expression behavior that is intended to be language law must ultimately be represented here.

---

4.3 "grammar/spec/"

Owns detailed machine-checkable specification contracts.

Expression conformance must be represented here where appropriate.

---

4.4 "grammar/Zamani.g4"

Owns the root ANTLR composition.

It must compose expression syntax without becoming the owner of every individual expression feature.

The root grammar should remain intentionally thin.

It should primarily own:

- program/item composition;
- top-level dispatch;
- expression entry;
- type/declaration/statement dispatch;
- EOF.

It must not duplicate specialized expression rules.

---

4.5 "grammar/grammar.md"

Documents implementation/conformance status.

It must distinguish actual implementation from planned or specified syntax.

---

4.6 "grammar/Zamani-Grammar.md"

Contains historical, extended, proposed, experimental, deprecated, or otherwise non-authoritative material.

Its contents do not automatically become legal Zamani syntax.

---

5. Single Expression Authority

There must be exactly one public expression entry point.

The canonical public rule is:

expression

No other expression file may introduce a competing top-level "expression" rule.

Specialized files define components consumed by the expression composition root.

Conceptually:

expression
    ↓
assignment
    ↓
conditional
    ↓
range
    ↓
logical
    ↓
bitwise
    ↓
comparison
    ↓
shift
    ↓
additive
    ↓
multiplicative
    ↓
prefix
    ↓
postfix
    ↓
primary

The exact hierarchy must be synchronized with "precedence.md" and the normative specification.

---

6. Current Repository Ownership Corrections

The current repository contains overlapping or duplicated expression-related files and naming patterns.

These must be resolved before the expression subsystem can be considered production-ready.

Known examples include:

expression.g4
expressions.g4
range.g4
ranges.g4
lambda.g4
lambdas.g4
closure.g4
closures.g4
conditionals.g4
conditional-expressions.g4

The existence of a file does not make it authoritative.

The repository must establish one owner for every public grammar concept.

The required policy is:

expressions.g4
    owns expression composition

assignment grammar
    owns assignment implementation

conditionals.g4
    owns conditional-expression implementation

ranges.g4
    owns range implementation

prefix/unary grammar
    owns prefix implementation

postfix.g4
    owns postfix composition

calls grammar
    owns call implementation

indexing.g4
    owns indexing implementation

member-access grammar
    owns member access

literals.g4
    owns literal composition

match.g4
    owns match-expression integration

lambdas.g4
    owns lambda syntax

closures.g4
    owns closure-specific syntax where it differs materially from lambda syntax

quantum.g4
    owns expression-level quantum syntax

async.g4
    owns expression-level asynchronous syntax

Where two files express the same concept, one must become canonical and the other must be:

- removed;
- converted into a non-authoritative compatibility/reference file;
- or merged into the canonical owner.

There must never be two simultaneously authoritative implementations.

---

7. File Completion Contract

Every ".g4" file under this directory must be independently completable.

Each file must document:

File
Status
Purpose
Owns
Does Not Own
Inputs
Lexer Dependencies
Grammar Dependencies
Exports
Consumed By
AST Contract
Semantic Contract
Type Contract
Effect Contract
Capability Contract
Resource Contract
Contract Integration
Policy Integration
Provenance Integration
Classical Integration
Quantum Integration
HDL Integration
Compiler Integration
Runtime Integration
Tooling Integration
Diagnostics
Positive Tests
Negative Tests
Boundary Tests
Scalability Tests
Determinism Tests
Compatibility Tests
Hard-Coding Audit
Completion Criteria

This contract prevents a supposedly finished file from depending on undocumented future changes elsewhere.

---

8. Expression Composition Root

File

grammar/expressions/expressions.g4

Owns

- public "expression";
- expression precedence composition;
- expression-layer delegation;
- integration of specialized expression grammar rules.

Does not own

- detailed implementation of every specialized expression form;
- semantic validation;
- AST implementation;
- IR generation.

Dependencies

It may consume canonical rules from:

core/
types/
expressions/

and the canonical lexer.

Integration

Zamani.g4
    ↓
expression
    ↓
expressions.g4
    ↓
specialized expression rules

Completion

"expressions.g4" is complete only when it contains no duplicate implementation of specialized rules and provides one unambiguous precedence hierarchy.

---

9. Precedence Authority

File

grammar/expressions/precedence.md

This is the human-readable precedence authority.

The grammar and specification must agree with it.

A production-ready precedence hierarchy must explicitly define:

- precedence;
- associativity;
- operator family;
- lexical token;
- grammar owner;
- semantic category.

A typical structure is:

assignment
conditional
range
logical-or
logical-and
bitwise-or
bitwise-xor
bitwise-and
equality
relational
shift
additive
multiplicative
prefix
postfix
primary

If additional operators are introduced, they must be inserted through the specification process.

No file may silently invent another precedence system.

---

10. Associativity

Every operator must explicitly specify:

- left associativity;
- right associativity;
- non-associativity.

The parser must not rely on accidental ANTLR behavior.

For example:

a = b = c

must have one language-defined parse.

Likewise:

a - b - c

must have one language-defined association.

---

11. Lexer Contract

Expression grammar files consume the canonical lexer.

They must not define parser-local replacements for lexer tokens.

Operator spelling belongs to the lexical subsystem.

Expression precedence belongs here.

Semantic meaning belongs downstream.

Therefore:

lexical spelling
    ↓
token
    ↓
expression grammar
    ↓
AST
    ↓
semantic interpretation

Expression files must not introduce duplicate token definitions such as alternative names for already-authoritative operators.

---

12. Identifiers

Identifiers are language-wide constructs.

If identifier ownership resides in:

grammar/core/
grammar/lexer/

then expression files must consume that canonical rule.

Do not create separate identifier rules for:

- expressions;
- functions;
- modules;
- quantum operations;
- AI models;
- hardware;
- data;
- networking.

This ensures that user-defined names remain open-ended.

---

13. Literals

Literal syntax must be shared.

Expressions may consume:

- integer literals;
- floating-point literals;
- decimal literals;
- string literals;
- character literals;
- boolean literals;
- null/unit literals where specified;
- collection literals;
- quantum literals where specified;
- domain-specific literals registered through the language architecture.

The expression grammar must not impose host-machine representation limits.

For example, a numeric literal is source data.

Whether it becomes:

- an arbitrary-precision value;
- a target-native integer;
- a symbolic value;
- a compile-time constant;
- a tensor element;
- a hardware parameter;

is a downstream semantic/compiler decision.

---

14. Primary Expressions

Primary expressions are the fundamental expression values.

They may include:

identifier
literal
qualified name
parenthesized expression
tuple
array
map/record
lambda where specified
block where specified
match where specified
domain-neutral construction

Primary syntax must remain independent of physical hardware.

---

15. Parenthesized Expressions

Parenthesized expressions provide explicit grouping.

Example:

(a + b) * c

Parentheses must not create a new semantic type.

They establish syntax-level grouping only.

---

16. Tuple Expressions

Tuple expressions must support arbitrary tuple arity subject only to actual implementation/resource constraints.

The grammar must not contain a fixed tuple size such as:

tuple2
tuple3
tuple4

as the universal mechanism.

Tuple structure is semantic data.

---

17. Collection Expressions

Collection expressions must support arbitrary element counts.

This includes:

- arrays;
- sequences;
- vectors;
- maps;
- records where expression syntax is applicable;
- future collection abstractions.

The grammar must not impose a maximum collection length.

A source file containing many elements remains valid structurally even if a particular compiler invocation cannot process it due to external resource exhaustion.

---

18. Expression Lists

Expression lists must be open-ended:

expression
    (COMMA expression)*

or the equivalent repository-approved composition.

There must be no language-level maximum number of:

- arguments;
- tuple elements;
- array elements;
- map entries;
- generic arguments;
- indices;
- expressions.

---

19. Unary / Prefix Expressions

Prefix expressions own unary operators.

Examples include:

+x
-x
!x
~x

and other operators explicitly standardized by Zamani.

The grammar establishes syntax.

The semantic system determines:

- type;
- overload;
- effect;
- capability;
- resource requirement;
- domain interpretation.

---

20. Binary Expressions

Binary expressions must be organized by the canonical precedence hierarchy.

Possible categories include:

- arithmetic;
- logical;
- bitwise;
- comparison;
- equality;
- relational;
- shift;
- range;
- domain-neutral operators explicitly specified by the language.

The grammar must not encode target-specific interpretations.

For example:

a + b

may ultimately represent:

- scalar addition;
- vector addition;
- matrix addition;
- tensor addition;
- symbolic addition;
- distributed addition;
- accelerator operation;
- compile-time computation.

The grammar does not choose among these.

---

21. Assignment Expressions

Where assignment is defined as an expression, assignment syntax belongs to the expression system.

The grammar determines structural validity.

Semantic analysis determines:

- whether the target is assignable;
- whether mutation is permitted;
- type compatibility;
- ownership constraints;
- effect requirements;
- capability requirements;
- resource implications.

The expression grammar must not implement those checks.

---

22. Assignment Targets

The grammar may structurally accept assignment targets such as:

identifier
member access
index expression
other specified assignable forms

Whether a particular expression is actually assignable is a semantic question.

No finite hardware-specific target list is permitted.

---

23. Conditional Expressions

Canonical owner

grammar/expressions/conditionals.g4

"expressions.g4" composes the conditional expression into the precedence hierarchy.

It must not duplicate its implementation.

Conditional expressions must remain domain-neutral.

Their conditions can later be used for:

- classical decisions;
- AI decisions;
- resource selection;
- adaptive execution;
- quantum-classical control;
- simulation;
- policy evaluation.

---

24. Range Expressions

Canonical owner

One range grammar must become authoritative.

The repository currently contains both:

range.g4
ranges.g4

These must not remain competing authorities.

The canonical range implementation must consume the canonical lexer tokens:

DOT_DOT
DOT_DOT_EQ

Examples:

a .. b
a ..= b
a ..
a ..=
.. b
..= b
..
..=

Only forms explicitly specified by Zamani are legal.

---

25. Range Semantics

The grammar must never encode artificial endpoint values such as:

0
1
MIN
MAX
machine_word_max
type_max

for omitted endpoints.

An omitted endpoint remains syntactically absent.

The semantic layer determines its interpretation.

This is mandatory for scalability.

---

26. Range Precedence

A range endpoint must consume the appropriate lower-precedence expression layer.

It must not recursively consume the complete "expression" rule if doing so would re-enter assignment, conditional, or range precedence.

The range grammar must therefore integrate into the established precedence architecture rather than inventing a second expression hierarchy.

---

27. Calls

Calls operate on callable expressions.

Conceptually:

callable(arguments)

not merely:

identifier(arguments)

This permits:

function(...)
closure(...)
object.method(...)
factory(...) (...)

and future callable abstractions.

There is no grammar-level maximum number of arguments.

---

28. Generic Invocation

Generic invocation must integrate with:

grammar/types/
grammar/functions/
grammar/declarations/

The expression subsystem must consume the canonical generic/type grammar rather than redefining generic syntax.

Generic arity must not be artificially bounded.

---

29. Indexing

Indexing belongs to expression syntax.

Examples:

array[index]
matrix[row, column]
tensor[indices...]

The grammar must not hard-code a finite number of dimensions.

This allows the same syntax to represent:

- arrays;
- matrices;
- tensors;
- datasets;
- quantum registers;
- distributed collections;
- hardware abstractions.

Semantic analysis determines indexability and dimensionality.

---

30. Member Access

Member access applies to expressions.

Examples:

value.member
value.method(...)

The grammar must not enumerate possible members.

Member existence, visibility, type, dispatch, ownership, and effects belong downstream.

This supports:

- user-defined types;
- modules;
- interfaces;
- libraries;
- dialects;
- generated structures;
- future domains.

---

31. Postfix Expressions

"postfix.g4" owns postfix composition.

It must support arbitrary legal chains:

value(...)
value[index]
value.member
value.method(...)[index].member(...)

subject to the language grammar.

No language-level maximum nesting depth is defined.

---

32. Lambdas

The canonical lambda owner must be selected between the current duplicate candidates:

lambda.g4
lambdas.g4

Only one should remain authoritative.

Lambda syntax must integrate with:

functions/
types/
effects/
memory/
statements/

The expression grammar preserves:

- parameters;
- body;
- source locations;
- generic information where specified;
- expression structure.

Semantic analysis owns:

- capture;
- ownership;
- lifetime;
- parameter typing;
- return typing;
- effects;
- capabilities;
- resource requirements.

---

33. Closures

The canonical closure owner must likewise be selected between:

closure.g4
closures.g4

Closures must not create a second function/lambda semantic system.

Their relationship must be:

closure syntax
    ↓
common callable semantic model
    ↓
type/effect/ownership analysis

---

34. Pattern Matching

Pattern matching must integrate with the repository-wide pattern system.

Expression syntax may contain:

match

where the language defines match as an expression.

Pattern ownership must not be duplicated inside every expression file.

Patterns may ultimately support:

- literals;
- identifiers;
- tuples;
- records;
- variants;
- ranges;
- guards;
- structural patterns;
- future domain-neutral patterns.

---

35. Guards

Guards are expressions.

Guard expressions must use the normal expression grammar.

This is important because guards may need:

- comparisons;
- function calls;
- pattern-derived values;
- logical combinations;
- contracts;
- capability predicates;
- resource predicates;
- policy predicates.

The guard system must not invent a second expression language.

---

36. Query Expressions

Query syntax must be distinguished from semantic query execution.

A query expression may ultimately target:

- collections;
- structured data;
- knowledge;
- graph data;
- datasets;
- distributed data;
- external data sources.

The expression grammar describes structure.

The semantic/data layers determine:

- query model;
- execution strategy;
- data source;
- optimization;
- distribution;
- security;
- provenance.

---

37. Reasoning and Knowledge Operations

Operations such as:

infer(...)
deduce(...)
reason(...)
assert(...)
retract(...)
query(...)

must not automatically become dedicated expression keywords.

If they can be represented by ordinary callable expressions, ordinary identifiers are preferable.

This keeps the universal expression grammar open-ended.

Dedicated syntax should only be introduced when the language specification establishes that syntax is necessary for semantics, parsing, diagnostics, tooling, or safety.

---

38. Learning

Learning operations may appear as ordinary expressions or as specialized constructs if a future normative specification requires them.

The expression subsystem must be capable of carrying the resulting AST through semantic analysis.

Semantic analysis may associate:

- model;
- input;
- target;
- objective;
- data;
- effects;
- capabilities;
- resources;
- policy;
- provenance.

The expression grammar must not enumerate every learning algorithm.

---

39. Adaptation

Adaptation must remain controlled.

An expression representing adaptation must not imply unrestricted source-code self-modification.

The semantic pipeline must be able to attach:

policy
authorization
effect
capability
resource requirement
provenance
contract

to adaptation.

The expression grammar itself remains target-neutral.

---

40. Uncertainty and Probability

Probability, confidence, distributions, beliefs, and uncertainty should primarily be represented through:

types
operations
libraries
semantic models

rather than an ever-growing collection of parser keywords.

Examples can remain callable expressions:

probability(...)
distribution(...)
confidence(...)
belief(...)

The grammar must not prescribe a specific probabilistic implementation.

---

41. Evidence and Provenance

Expressions can participate in:

- claims;
- evidence predicates;
- derivations;
- decision conditions;
- explanations;
- verification conditions.

However, provenance itself belongs to the repository-wide provenance model.

The expression grammar must preserve enough source structure for downstream provenance.

---

42. Contracts

Expression syntax must be usable inside:

requires
ensures
invariant
assume
guarantee
property
assertion

Contract ownership belongs to:

grammar/validation/

Expressions provide the condition.

This separation allows arbitrary expressions to participate in contracts without adding contract-specific operators to the universal expression grammar.

---

43. Policies

Expressions must be usable inside policy conditions.

Policy ownership belongs to:

grammar/policies/
grammar/security/
grammar/resources/

Expressions provide predicates and computations.

Policies determine what those predicates mean operationally.

---

44. Capabilities

Expressions may appear in capability conditions such as:

capability(...)

The expression grammar must not know the finite universe of capabilities.

Capability names remain open-ended.

This is essential for future:

- processors;
- accelerators;
- QPUs;
- sensors;
- memory systems;
- networks;
- cryptographic hardware;
- future computational substrates.

---

45. Resource Requirements

Expressions may appear in resource requirements.

For example, semantic constructs may express requirements equivalent to:

memory >= required_memory

or:

resource(...)

The expression grammar must not define:

RAM = 64GB
VRAM = 24GB
register = 32-bit
run_on_8_threads

as universal assumptions.

A program expresses requirements.

The compiler negotiates realization.

---

46. Effects

Expressions may produce effects including, where defined by the effect system:

io
network
mutation
randomness
native
foreign
distributed
measurement
learning
adaptation
reflection
code_generation
simulation

The expression grammar does not decide whether an effect is permitted.

The effect subsystem performs that analysis.

---

47. Classical Integration

Expressions are the common syntax for classical computation.

They must support:

- scalar arithmetic;
- arbitrary-precision semantic values;
- collections;
- numerical computation;
- symbolic computation;
- functions;
- closures;
- pattern matching;
- data operations;
- compile-time computation.

No separate classical expression language should be required.

---

48. Quantum Integration

Quantum syntax must remain separate from physical quantum implementation.

Expression-level quantum constructs may be consumed by:

grammar/expressions/quantum.g4

but their semantic path is:

expression
    ↓
domain-neutral AST
    ↓
quantum semantic model
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
resilience/QEC where applicable
    ↓
ZQN
    ↓
HAL

The expression grammar must not enumerate:

H
X
Y
Z
CNOT

as the universal set of quantum operations.

Quantum operations must remain open-world through the quantum operation model.

The grammar must not know:

- physical qubit identifiers;
- vendor topology;
- calibration;
- coupling maps;
- physical gate availability;
- QEC implementation;
- QPU size.

---

49. Quantum-Classical Hybrid Expressions

The expression system must permit classical expressions to control quantum computation and quantum results to participate in classical computation.

Examples include semantic relationships equivalent to:

classical_value
    ↓
quantum operation parameter
    ↓
measurement
    ↓
classical value
    ↓
classical decision

The expression grammar must not hard-code a fixed host/device arrangement.

---

50. HDL Integration

Expression syntax may be used in:

- parameterization;
- expressions over signals;
- widths where semantically required;
- timing expressions;
- conditions;
- generate conditions;
- verification conditions;
- simulation expressions.

The expression grammar must not hard-code a universal register width, wire width, clock count, device count, or hardware capacity.

Hardware realization belongs to:

grammar/hdl/
grammar/hardware/

and downstream compiler/synthesis systems.

---

51. Tensor and Accelerator Integration

Tensor expressions must use the common expression system.

The grammar must not define a finite tensor rank.

An expression such as:

tensor[index...]

must remain semantically extensible.

Whether the expression lowers to:

- CPU;
- GPU;
- TPU-like accelerator;
- FPGA;
- ASIC;
- vector engine;
- distributed accelerator;

is a downstream decision.

---

52. AI and Symbolic Computation

The expression grammar must support AI and symbolic computation without becoming an AI-specific grammar.

The same expression constructs can represent:

model(input)
reason(...)
infer(...)
query(...)
learn(...)
adapt(...)
probability(...)
explain(...)

when these are ordinary callable operations.

Dedicated syntax is justified only when semantics require it.

This prevents keyword explosion and allows libraries and future domains to evolve independently.

---

53. Neural-Symbolic Composition

Expressions must be capable of composing:

symbolic computation
+
learned computation
+
reasoning
+
data
+
probability

without requiring a second expression language.

The semantic layer determines how the composition executes.

---

54. Multi-Agent Integration

Agents must not create a second actor syntax.

The integration is:

agent semantic model
    ↓
actor/concurrency model
    ↓
message
    ↓
scheduler/runtime

Expression syntax may construct, invoke, query, or communicate with an agent where the language specification permits it.

Actor lifecycle remains owned by:

grammar/concurrency/

---

55. Async Integration

"async.g4" owns expression-level asynchronous syntax.

The expression grammar must not encode:

- thread count;
- worker count;
- CPU affinity;
- GPU stream count;
- QPU schedule;
- cluster size.

For example:

spawn computation

describes program semantics.

It does not prescribe a particular machine realization.

---

56. Simulation

Expressions may invoke simulation facilities.

Simulation can represent:

- classical systems;
- quantum systems;
- hardware;
- distributed systems;
- AI systems;
- fault models;
- performance models.

Simulation remains an execution strategy.

It is not a second language.

---

57. Compile-Time Expressions

Compile-time expressions must remain explicitly distinguishable from ordinary runtime expressions where the specification requires that distinction.

The parser must not itself:

- execute code;
- inspect hardware;
- access the filesystem;
- access the network;
- invoke runtime devices;
- invoke QPUs;
- query target availability.

Those operations belong to compiler stages with explicit contracts.

---

58. Reflection and Metaprogramming

Expression syntax may participate in:

- reflection;
- compile-time computation;
- quotation;
- syntax trees;
- code generation;
- type-level computation.

Ownership belongs to:

grammar/metaprogramming/
grammar/macros/

The expression subsystem must provide the expression syntax consumed by those systems.

Reflection and code generation must carry appropriate effects/capabilities.

---

59. Interoperability

Expressions may represent calls into:

- foreign functions;
- FFI;
- ABI boundaries;
- data interchange systems;
- external services.

However, FFI semantics belong to:

grammar/interoperability/

The expression grammar only provides call/expression structure.

Foreign operations must participate in the effect and capability systems.

---

60. Dialects

Domain-specific expression syntax must be isolated through the repository's dialect architecture.

Examples include:

SQL
JSON query
XML query
vendor-specific hardware
specialized scientific notation
domain-specific extensions

A dialect must not silently redefine the universal expression grammar.

The integration model is:

dialect
    ↓
dialect parser/grammar
    ↓
Zamani semantic model
    ↓
canonical IR

where supported.

---

61. Open-World Extension Principle

The expression grammar must be open-world.

Adding a new:

- library;
- AI model;
- quantum operation;
- accelerator;
- hardware device;
- data source;
- protocol;
- mathematical operation;
- scientific method;

must not normally require modifying the universal expression grammar.

Prefer:

identifier
qualified name
call
member access
operator
literal
attribute
type
capability
effect
resource requirement
dialect

over adding a new keyword for every capability.

---

62. No Application-Specific Keyword Explosion

The universal expression grammar must not become a catalog of application features.

Features such as:

- computer vision;
- sentiment analysis;
- robotics;
- payments;
- blockchain;
- VR;
- AR;
- administrative operations;
- legal workflows;
- domain-specific services;

should normally be implemented through:

- libraries;
- APIs;
- dialects;
- capabilities;
- policies;
- types;
- semantic models;
- external services.

The expression grammar remains universal.

---

63. Domain-Neutral AST Contract

The expression grammar must lower into a domain-neutral frontend AST.

Suitable AST categories include:

Literal
Identifier
QualifiedName
Unary
Binary
Assignment
Conditional
Range
Call
Index
Member
Tuple
Array
Map/Record
Lambda
Closure
Match
Construction
BlockExpression
AsyncExpression
CompileTimeExpression
QuantumExpression

Exact names must follow the existing AST implementation.

The AST must not become a physical execution representation.

Do not introduce AST nodes whose only purpose is:

specific QPU
specific CPU
specific GPU
specific FPGA
specific vendor gate
physical qubit mapping
routing result
calibration
scheduler decision
QEC layout

Those belong downstream.

---

64. Semantic Contract

After parsing, semantic analysis is responsible for:

- name resolution;
- type checking;
- type inference;
- overload resolution;
- generic resolution;
- ownership;
- borrowing;
- lifetime rules;
- effects;
- capabilities;
- resources;
- contracts;
- policies;
- provenance;
- domain selection;
- constant evaluation where applicable.

The expression grammar must not attempt to perform these operations.

---

65. Type Integration

Expression syntax must integrate with the canonical type system.

This includes support for semantic types such as:

- scalar types;
- tuples;
- records;
- arrays;
- slices;
- maps;
- functions;
- generics;
- references;
- option/result types;
- linear/affine types where specified;
- dependent/refinement information where specified;
- probability/distribution/uncertainty types where specified.

Expression grammar must not duplicate type definitions.

---

66. Effect Integration

Every expression must be capable of being represented in a semantic model that can carry effect information.

For example:

pure expression

and:

network operation

may have different effects even when both use ordinary call syntax.

The grammar does not decide purity.

---

67. Capability Integration

Expressions may require capabilities.

Examples:

tensor.compute
quantum.measurement
accelerator.compute
network.access
native.execute
foreign.call
reflection

Capability names must remain open-ended.

No finite capability registry belongs in the expression grammar.

---

68. Resource Integration

Expressions may contribute resource requirements.

Examples include requirements for:

- memory;
- compute;
- storage;
- communication;
- quantum resources;
- accelerator resources;
- topology;
- precision;
- timing;
- energy;
- reliability.

The expression grammar does not resolve these requirements.

---

69. Contract Integration

Expressions can be used by:

requires
ensures
invariant
assume
guarantee
property
assert

Contract checking occurs after parsing.

---

70. Policy Integration

Expressions can serve as policy predicates.

Policies can govern:

- execution;
- resources;
- security;
- adaptation;
- deployment;
- simulation;
- target selection;
- interoperability.

The expression subsystem provides the predicate language.

The policy subsystem owns policy semantics.

---

71. Provenance Integration

Expression source locations must survive into the AST sufficiently to support:

- diagnostics;
- source maps;
- reproducible builds;
- compiler transformations;
- explanation;
- evidence;
- provenance;
- IDE tooling;
- debugging.

The expression grammar must not discard source distinctions unnecessarily.

---

72. Deterministic Parsing

Given identical:

source
language version
grammar version
dialect configuration

the parser must produce the same structural result.

Expression parsing must not depend on:

- time;
- randomness;
- environment variables;
- filesystem state;
- network state;
- hardware discovery;
- scheduler state;
- target availability.

---

73. Scalability Requirements

The grammar must not establish language-level maximums for:

- expression count;
- expression nesting;
- argument count;
- tuple arity;
- collection size;
- generic parameter count;
- index count;
- tensor rank;
- identifier length;
- quantum register size;
- processor count;
- device count;
- node count;
- memory size;
- thread count;
- network size.

Implementation limits may exist due to:

- available memory;
- parser implementation;
- host process limits;
- operating-system constraints;
- compiler resource policies.

Those are implementation/resource concerns, not language semantics.

---

74. Symbolic Scalability

The grammar must support symbolic values rather than requiring physical values to be known during parsing.

Examples:

required_memory
required_qubits
required_parallelism
required_precision
required_topology

may be semantic expressions.

The parser must not force them into fixed machine constants.

---

75. POCO-REAF Contract

The expression language must allow one source expression to remain semantically stable while the compiler determines different realizations.

The same source-level computation may be realized on:

tiny embedded target
CPU
multicore CPU
GPU
FPGA
ASIC
accelerator
QPU
quantum simulator
HPC system
cluster
distributed system
cloud
future computational substrate

The expression itself must not encode a machine-specific realization unless the programmer explicitly chooses a target-specific construct through the appropriate subsystem.

---

76. Target Independence

The expression grammar must not contain target-selection logic such as:

run_on_cpu
run_on_gpu
run_on_8_threads
run_on_qpu_7
use_gpu_0
use_qubit_3

as universal expression constructs.

Target intent belongs to:

resources/
compile/
hardware/
execution/
dialects/

where appropriate.

---

77. Quantum Resource Independence

The expression grammar must not contain a fixed number of:

- qubits;
- gates;
- registers;
- quantum devices;
- quantum dimensions.

Quantum resource requirements are semantic/resource information.

Physical mapping occurs downstream.

---

78. HDL Resource Independence

The expression grammar must not define a universal:

- register width;
- bus width;
- wire width;
- memory size;
- number of ports;
- number of devices;
- clock count.

Those are target/resource properties.

---

79. Error Boundary

Parser diagnostics must describe structural syntax failures.

Examples:

unexpected token
missing delimiter
invalid expression structure
invalid operator placement
malformed argument list
malformed range
malformed lambda

Semantic diagnostics belong downstream:

type mismatch
unknown name
invalid assignment target
unavailable capability
insufficient resource
invalid effect
forbidden policy
invalid quantum operation
invalid ownership

The parser must never silently turn malformed syntax into a valid semantic node.

---

80. Error Recovery

ANTLR error recovery must not alter the accepted language silently.

Recovery is for diagnostics and parser continuation.

Production parsing must retain enough source information to report accurate diagnostics.

---

81. Security Boundary

Expression parsing itself must not execute arbitrary code.

Parsing must not:

- invoke native code;
- invoke FFI;
- access network;
- access filesystem;
- execute shell commands;
- invoke hardware;
- invoke QPUs;
- modify external state.

Any compile-time execution capability must be explicitly controlled downstream.

---

82. Safe Rust Requirement

The Rust implementation associated with expression parsing and semantic integration must target:

Rust 1.97 / Rust 1.97.1
Edition 2021

and must not require "unsafe".

Expression grammar changes must not introduce a requirement for unsafe Rust.

If an implementation requires an optimization that would normally tempt use of unsafe code, the safe implementation remains the architectural requirement.

---

83. Compiler Integration

The expression subsystem integrates with the compiler as:

lexer
  ↓
parser
  ↓
expression parse tree
  ↓
AST conversion
  ↓
structural validation
  ↓
semantic analysis
  ↓
typed expression
  ↓
effect analysis
  ↓
capability analysis
  ↓
resource analysis
  ↓
contract/policy analysis
  ↓
canonical semantic representation
  ↓
IR lowering

The grammar must never directly emit target code.

---

84. Classical IR Integration

Classical expressions may lower into the repository's canonical classical/domain IR.

The expression grammar does not own that IR.

The semantic lowering layer maps:

AST expression
    ↓
semantic expression
    ↓
classical IR

where appropriate.

---

85. Quantum IR Integration

Quantum expressions must ultimately enter the canonical:

quantum::ir

path.

There must not be a second competing quantum frontend IR created merely because an expression is quantum-related.

The boundary is:

expression AST
    ↓
quantum semantic analysis
    ↓
quantum::ir

After that point:

optimization
decomposition
routing
scheduling
resilience
ZQN
HAL

are outside this directory.

---

86. HDL Integration

HDL-related expressions follow:

expression AST
    ↓
HDL semantic model
    ↓
HDL/domain IR
    ↓
verification/simulation/synthesis
    ↓
target realization

Expression grammar does not own synthesis.

---

87. Runtime Integration

Runtime behavior is not implemented by this directory.

Runtime consumers may receive expression-derived semantic operations after compilation.

The expression grammar therefore must remain free of:

- scheduler logic;
- runtime state;
- memory allocator logic;
- device drivers;
- QPU control;
- network execution;
- thread creation.

---

88. Tooling Integration

The expression grammar must support downstream tooling including:

- formatter;
- parser diagnostics;
- IDE/LSP;
- syntax highlighting;
- code completion;
- navigation;
- refactoring;
- source maps;
- documentation tools;
- static analysis;
- compiler diagnostics;
- conformance tooling.

Stable source spans and deterministic parsing are mandatory.

---

89. Test Architecture

Expression tests must be organized independently of implementation details.

Recommended structure:

grammar/tests/
├── lexical/
├── parser/
│   └── expressions/
│       ├── primary/
│       ├── unary/
│       ├── binary/
│       ├── assignment/
│       ├── conditional/
│       ├── range/
│       ├── call/
│       ├── indexing/
│       ├── member/
│       ├── lambda/
│       ├── closure/
│       ├── collection/
│       ├── comprehension/
│       ├── match/
│       ├── async/
│       ├── compile-time/
│       └── quantum/
├── semantic/
├── negative/
├── boundary/
├── scalability/
├── determinism/
└── compatibility/

The exact existing repository test layout must be reused where already established rather than creating duplicate test hierarchies unnecessarily.

---

90. Required Expression Test Classes

Every expression feature requires:

Positive tests

Valid syntax must parse.

Negative tests

Invalid syntax must fail.

Precedence tests

Operators must associate according to specification.

Associativity tests

Left/right/non-associativity must be verified.

Boundary tests

Interaction with adjacent grammar systems must be tested.

Scalability tests

Tests must exercise generalized/unbounded forms without embedding artificial limits.

Determinism tests

Identical input/configuration must produce identical parsing behavior.

Compatibility tests

Previously stable syntax must remain stable unless deliberately versioned.

Cross-domain tests

Expression constructs must be tested inside:

- classical;
- quantum;
- hybrid;
- HDL;
- AI/data;
- distributed;
- networking;
- contracts;
- policies;
- resource requirements.

---

91. Mandatory Cross-Domain Examples

The expression subsystem must eventually be exercised through programs equivalent to:

minimal.zm
classical.zm
quantum.zm
hybrid.zm
hdl.zm
poco-reaf.zm

and additional scenarios covering:

reasoning
knowledge
learning
adaptation
uncertainty
contracts
policies
provenance
agents
sandboxing
simulation
FFI
pattern matching
generic types
advanced types
quantum-classical computation

These tests must use the common expression system rather than creating separate domain-specific expression languages.

---

92. Precedence Regression Tests

At minimum, the test suite must distinguish structures such as:

a + b * c
a * b + c
a < b == c
a && b || c
a | b & c
a << b + c
a .. b
a ..= b
condition ? a : b
a = b = c
f(a)[i].x

The exact supported operators must follow the specification.

These examples are structural regression tests, not semantic limitations.

---

93. Range Regression Tests

Range tests must verify:

a .. b
a ..= b
a ..
a ..=
.. b
..= b

where supported.

They must also verify that range syntax does not incorrectly consume assignment or conditional expressions.

---

94. Postfix Regression Tests

Tests must verify chains such as:

value.member
value[index]
value(arguments)
value.member(arguments)
value[index].member(arguments)

and arbitrary legal combinations.

---

95. Generic Regression Tests

Tests must verify that expression syntax correctly integrates with generic invocation without duplicating the type grammar.

---

96. Quantum Regression Tests

Quantum expression tests must verify that quantum syntax reaches the quantum semantic boundary without:

- hard-coded gate sets;
- fixed qubit counts;
- physical topology;
- vendor assumptions;
- QEC assumptions.

The final semantic target remains:

quantum::ir

---

97. AI / Reasoning Regression Tests

Expression tests must verify that ordinary callable syntax can express extensible operations such as:

infer(...)
deduce(...)
reason(...)
assert(...)
retract(...)
query(...)
learn(...)
adapt(...)
explain(...)

without requiring a new universal keyword for every operation.

---

98. Contract Regression Tests

Expressions must parse correctly inside:

requires
ensures
invariant
assume
guarantee
property

where supported by the validation grammar.

---

99. Resource Regression Tests

Expressions must be usable in resource predicates without embedding resource limits into the expression grammar.

For example, a resource condition may semantically contain expressions equivalent to:

memory >= required_memory

or:

capability("tensor.compute")

The expression parser does not decide whether a target satisfies them.

---

100. Policy Regression Tests

Expression predicates must be usable in policy conditions.

Policy evaluation remains outside parsing.

---

101. Hard-Coding Audit

No expression grammar file may contain artificial universal limits.

The following concepts are explicitly prohibited:

MAX_QUBITS
MAX_CPUS
MAX_GPUS
MAX_FPGAS
MAX_NODES
MAX_MEMORY
MAX_THREADS
MAX_TENSOR_RANK
MAX_REGISTER_WIDTH
MAX_NETWORK_SIZE
MAX_DEVICE_COUNT

The same prohibition applies to equivalent disguised constants.

Examples of prohibited architecture:

exprWithAtMost8Args
tensorRank <= 4
tuple1 | tuple2 | tuple3 | tuple4
maxIndexCount
maxQuantumOperands
maxDevices

unless such a limit is explicitly part of a temporary implementation diagnostic rather than the language grammar—and even then it must not define the language's semantic capacity.

---

102. No Artificial Nesting Ceiling

The grammar must not intentionally define:

expressionDepth1
expressionDepth2
expressionDepth3
...

or similar finite expansion schemes.

ANTLR/runtime/parser resource exhaustion is an implementation concern.

The language model remains recursively compositional.

---

103. No Domain Keyword Explosion

A new domain operation should not automatically require:

new keyword
new parser branch
new AST category

Prefer the existing extensible mechanisms:

identifier
qualified name
call
member access
operator
type
attribute
capability
effect
resource
dialect

A dedicated grammar construct is warranted only where its syntax has language-level semantic value.

---

104. Compatibility

Expression syntax is part of the public language surface.

Breaking changes require:

- specification update;
- conformance update;
- migration documentation;
- compatibility tests;
- diagnostics;
- versioning where appropriate.

Historical aliases must not be silently introduced.

Canonical lexer token names must be used.

---

105. Generated Grammar Integrity

Before accepting expression grammar changes:

1. all imported grammars must resolve;
2. all referenced rules must exist;
3. no duplicate public rules may exist;
4. no circular grammar dependency may exist;
5. no unreachable expression rules may remain unintentionally;
6. no token mismatch may exist;
7. generated parser code must build;
8. Rust frontend integration must build;
9. tests must pass;
10. no unsafe Rust may be introduced.

---

106. Repository-Wide Integration Checklist

Before this subsystem is considered complete, verify:

[ ] grammar/DESIGN.md agrees
[ ] grammar/README.md points here
[ ] grammar/grammar.md reflects actual status
[ ] grammar/Zamani-Grammar.md does not contradict normative syntax
[ ] grammar/Zamani.g4 imports/composes this subsystem correctly
[ ] lexer owns all expression tokens
[ ] core owns shared identifiers/names
[ ] types owns type syntax
[ ] statements owns statement syntax
[ ] declarations owns declaration syntax
[ ] functions owns function declarations
[ ] validation owns contracts
[ ] policies owns policies
[ ] resources owns resource requirements
[ ] effects owns effect semantics
[ ] security owns security semantics
[ ] concurrency owns concurrency semantics
[ ] execution owns execution semantics
[ ] quantum owns quantum semantics
[ ] hybrid owns hybrid semantics
[ ] HDL owns hardware-description semantics
[ ] interoperability owns FFI/ABI
[ ] metaprogramming owns reflection/code generation
[ ] AST remains domain-neutral
[ ] classical lowering is downstream
[ ] quantum lowering reaches quantum::ir
[ ] no expression grammar emits target code
[ ] no expression grammar selects hardware
[ ] no artificial capacity limits exist
[ ] no duplicate expression authority exists

---

107. Independent File Completion Protocol

An expression grammar file is considered complete only after all of the following are known before implementation is finalized:

PURPOSE:
    Why the file exists.

OWNS:
    Exact syntax owned here.

DOES_NOT_OWN:
    Exact syntax owned elsewhere.

INPUTS:
    Tokens/rules consumed.

EXPORTS:
    Rules provided to other grammar files.

AST_OWNER:
    Exact AST layer responsible.

SEMANTIC_OWNER:
    Exact semantic subsystem responsible.

TYPE_OWNER:
    Type interpretation.

EFFECT_OWNER:
    Effect interpretation.

CAPABILITY_OWNER:
    Capability interpretation.

RESOURCE_OWNER:
    Resource interpretation.

CONTRACT_OWNER:
    Contract interaction.

POLICY_OWNER:
    Policy interaction.

PROVENANCE_OWNER:
    Provenance interaction.

IR_OWNER:
    Canonical downstream IR.

QUANTUM_BOUNDARY:
    Exact interaction with quantum::ir, if applicable.

HDL_BOUNDARY:
    Exact interaction with HDL semantic processing, if applicable.

COMPILER_BOUNDARY:
    Exact compiler stage consuming it.

RUNTIME_BOUNDARY:
    Exact runtime consumer, if applicable.

TOOLING_BOUNDARY:
    IDE/LSP/formatter/debugger implications.

TEST_OWNER:
    Exact test location.

COMPATIBILITY:
    Versioning/deprecation behavior.

SCALABILITY:
    How unbounded language semantics are preserved.

HARD_CODING_AUDIT:
    Confirmation that no artificial capacity is encoded.

COMPLETION_CRITERIA:
    Exact conditions for DONE.

No file should depend on undocumented ownership that is discovered only after another file has been implemented.

---

108. Dependency Direction

The expression subsystem must maintain one-way architectural dependencies.

Preferred direction:

lexer
  ↓
core lexical rules
  ↓
expression grammar
  ↓
parser
  ↓
AST
  ↓
semantic analysis
  ↓
IR

Expression grammar must not depend on:

runtime
HAL
device drivers
QPU implementation
GPU implementation
FPGA implementation
scheduler implementation
physical routing
calibration

---

109. Circular Dependency Prohibition

No expression grammar file may create a cycle such as:

expressions
  ↓
quantum
  ↓
expressions

or:

expressions
  ↓
types
  ↓
expressions

unless the repository's ANTLR architecture explicitly supports the required composition without circular imports.

Where shared syntax is required, extract the genuinely shared rule into the appropriate lower-level owner rather than creating a cycle.

---

110. Expression-to-Statement Boundary

The repository must clearly distinguish:

expression

from:

statement

A construct must not be duplicated in both merely because it can conceptually perform computation.

If a feature has both expression and statement forms, each form must have explicit ownership and semantic correspondence.

---

111. Expression-to-Declaration Boundary

Declarations belong to:

grammar/declarations/
grammar/functions/
grammar/modules/
grammar/types/

Expressions may reference declared entities but must not redefine declaration syntax.

---

112. Expression-to-Type Boundary

Expressions consume types.

Types do not become expression syntax merely because type information may appear inside expressions.

The generic/type system must remain the canonical owner.

---

113. Expression-to-Resource Boundary

Expressions can produce resource requirements but do not own resource semantics.

The resource system resolves:

requirement
capability
constraint
preference
budget
hint
negotiation

after parsing.

---

114. Expression-to-Execution Boundary

An expression describes computation.

Execution determines realization.

Therefore the same expression can participate in different execution plans without changing source semantics.

---

115. Expression-to-Optimization Boundary

Optimization must operate on semantic/IR representations.

The expression grammar must not encode optimizer decisions.

Examples of downstream decisions include:

- vectorization;
- fusion;
- parallelization;
- distribution;
- accelerator selection;
- quantum decomposition;
- scheduling.

---

116. Expression-to-Resilience Boundary

Expressions do not own:

- retry;
- recovery;
- degradation;
- quarantine;
- retirement.

Those belong to execution/resilience systems.

Expression semantics may, however, be inputs to those systems.

---

117. Expression-to-Provenance Boundary

The expression grammar supplies source structure and source locations.

The provenance system records:

source
derived_from
generated_by
transformed_by
verified_by
reason
evidence
decision
version

where applicable.

---

118. Expression-to-Explanation Boundary

Expressions may be included in explanations.

The expression subsystem must preserve enough structure to allow downstream tooling to explain:

- program decisions;
- compiler transformations;
- resource choices;
- quantum transformations;
- hardware mapping;
- policy outcomes.

---

119. Formatting Contract

Expression syntax must be sufficiently deterministic for the formatter to reconstruct canonical formatting.

The formatter must not need target-specific semantic knowledge merely to format an expression.

---

120. IDE/LSP Contract

Expression parsing must preserve source positions sufficiently for:

- completion;
- hover;
- diagnostics;
- go-to-definition;
- rename;
- semantic highlighting;
- code actions;
- refactoring.

---

121. Incremental Parsing

The expression grammar should be designed so that changes to a local expression do not require a language-wide grammar rewrite.

This reinforces modularity and supports incremental tooling.

---

122. Future-Proofing

Future computational systems should be representable without modifying the universal expression model merely because the hardware or domain is new.

The architecture must therefore prefer:

generic syntax
+
typed semantics
+
capabilities
+
resources
+
effects
+
policies
+
dialects

over:

one keyword per technology

---

123. Production Readiness Gate

"grammar/expressions/" is not production-ready merely because the ANTLR grammar generates a parser.

Production readiness requires all of:

Specification
    ↓
Lexer
    ↓
Grammar
    ↓
AST
    ↓
Structural validation
    ↓
Semantic analysis
    ↓
Type analysis
    ↓
Effect analysis
    ↓
Capability analysis
    ↓
Resource analysis
    ↓
Contract analysis
    ↓
Policy analysis
    ↓
Provenance
    ↓
Canonical IR
    ↓
Compiler integration
    ↓
Target-independent lowering
    ↓
Tests
    ↓
Compatibility

Every expression feature must be traceable through this pipeline.

---

124. Definition of DONE

The expression subsystem is complete only when:

[ ] One canonical expression entry rule exists.
[ ] One canonical precedence model exists.
[ ] Associativity is explicitly specified.
[ ] Lexer ownership is unambiguous.
[ ] Identifier ownership is unambiguous.
[ ] Literal ownership is unambiguous.
[ ] Assignment ownership is unambiguous.
[ ] Conditional ownership is unambiguous.
[ ] Range ownership is unambiguous.
[ ] Prefix ownership is unambiguous.
[ ] Postfix ownership is unambiguous.
[ ] Call ownership is unambiguous.
[ ] Indexing ownership is unambiguous.
[ ] Member-access ownership is unambiguous.
[ ] Lambda ownership is unambiguous.
[ ] Closure ownership is unambiguous.
[ ] Match ownership is unambiguous.
[ ] Query integration is unambiguous.
[ ] Async integration is unambiguous.
[ ] Compile-time integration is unambiguous.
[ ] Quantum integration is unambiguous.
[ ] HDL integration is unambiguous.
[ ] AI/data integration is open-ended.
[ ] Contract integration is defined.
[ ] Policy integration is defined.
[ ] Effect integration is defined.
[ ] Capability integration is defined.
[ ] Resource integration is defined.
[ ] Provenance integration is defined.
[ ] AST ownership is defined.
[ ] Semantic ownership is defined.
[ ] IR ownership is defined.
[ ] quantum::ir is the canonical quantum boundary.
[ ] No target-specific realization exists in universal expression grammar.
[ ] No physical hardware assumptions exist.
[ ] No artificial resource limits exist.
[ ] No fixed tensor-rank limit exists.
[ ] No fixed qubit limit exists.
[ ] No fixed processor limit exists.
[ ] No fixed node limit exists.
[ ] No fixed device limit exists.
[ ] No fixed argument limit exists.
[ ] No fixed tuple limit exists.
[ ] No fixed index-count limit exists.
[ ] No fixed expression-nesting language limit exists.
[ ] No duplicate public expression rule exists.
[ ] No duplicate expression feature is authoritative.
[ ] No circular grammar dependency exists.
[ ] All imports resolve.
[ ] Generated parser builds.
[ ] Rust 1.97/1.97.1 compatibility is verified.
[ ] No unsafe Rust is required.
[ ] Positive tests pass.
[ ] Negative tests pass.
[ ] Precedence tests pass.
[ ] Associativity tests pass.
[ ] Boundary tests pass.
[ ] Scalability tests pass.
[ ] Determinism tests pass.
[ ] Compatibility tests pass.
[ ] Classical integration passes.
[ ] Quantum integration passes.
[ ] Hybrid integration passes.
[ ] HDL integration passes.
[ ] AI/data integration passes.
[ ] Distributed integration passes.
[ ] Resource/contract/policy integration passes.
[ ] Tooling integration passes.

---

125. Final Architectural Rule

The expression subsystem must remain small in universal concepts but broad in semantic capability.

The goal is not to put every computational technology into expression grammar.

The goal is to provide a stable set of composable expression primitives from which those technologies can be expressed.

The universal model is:

Expression
    │
    ├── Value
    ├── Type
    ├── Operation
    ├── Effect
    ├── Capability
    ├── Resource
    ├── Requirement
    ├── Constraint
    ├── Contract
    ├── Policy
    ├── Evidence
    └── Provenance
           │
           ▼
    Domain-Neutral AST
           │
           ▼
    Semantic Model
           │
       ┌───┴────┐
       ▼        ▼
 Classical   quantum::ir
       │        │
       └───┬────┘
           ▼
      Optimization
           ▼
        Lowering
           ▼
   Routing/Scheduling
           ▼
      Resilience
           ▼
          ZQN
           ▼
          HAL
           ▼
   Target Realization

This is the boundary that makes the expression subsystem compatible with POCO-REAF.

The source expression describes computation. It does not describe the finite machine on which that computation happens.

That distinction must remain invariant throughout "grammar/expressions/".