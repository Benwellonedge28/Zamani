Zamani Grammar — Production Ambiguity and Determinism Specification

File: "grammar/validation/ambiguity.md"
Status: Normative production specification
Language: Zamani
Grammar technology: ANTLR4-compatible grammar architecture
Rust baseline: Rust 1.97 / Rust 1.97.1
Rust edition: Rust 2021
Safety: Safe Rust only; production implementation MUST NOT require "unsafe"
Primary objective: Deterministic, scalable, portable interpretation of Zamani source
Portability objective: "Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever" (POCO-REAF)

---

1. Purpose

This document defines the production ambiguity contract for the entire Zamani language grammar.

It specifies how Zamani MUST:

- prevent ambiguity;
- detect ambiguity;
- classify ambiguity;
- resolve ambiguity;
- defer legitimate semantic decisions to semantic analysis;
- preserve source structure;
- preserve deterministic diagnostics;
- preserve AST determinism;
- preserve canonical IR boundaries;
- prevent hardware-dependent parsing;
- prevent dialect-dependent accidental reinterpretation;
- prevent macro-dependent nondeterminism;
- prevent implementation-order-dependent overload resolution;
- preserve compatibility as the language evolves.

This document applies across:

- lexical analysis;
- tokenization;
- identifiers;
- keywords;
- operators;
- literals;
- expressions;
- types;
- declarations;
- statements;
- functions;
- modules;
- effects;
- memory;
- concurrency;
- classical computing;
- quantum computing;
- hybrid computing;
- HDL;
- hardware intent;
- distributed computing;
- AI;
- data;
- networking;
- security;
- resources;
- compilation;
- execution;
- interoperability;
- dialects;
- macros;
- metaprogramming;
- Sankofa/temporal features;
- nano computing;
- semantic analysis;
- AST construction;
- canonical IR lowering.

The objective is stronger than merely eliminating ANTLR warnings.

The production invariant is:

«For a fixed source text, language version, enabled features, dialect environment, and explicitly defined macro configuration, Zamani MUST produce one deterministic syntactic interpretation or one deterministic set of diagnostics.»

Where a decision genuinely belongs to semantic analysis rather than syntax, the grammar MUST preserve that distinction explicitly.

---

2. Authority and Integration

The ambiguity system operates inside the repository's existing authority model.

2.1 Authority hierarchy

The authoritative chain is:

grammar/specification/
        ↓
grammar/spec/
        ↓
grammar/Zamani.g4
        ↓
grammar/antlr/ZamaniParser.g4
grammar/antlr/ZamaniLexer.g4
        ↓
domain grammar directories
        ↓
src/lexer.rs
        ↓
src/parser.rs
        ↓
src/ast/
        ↓
semantic analysis
        ↓
canonical semantic representation
        ↓
canonical IR
        ↓
target-specific lowering

The existing:

grammar/validation/ambiguity-rules.md

MUST remain for compatibility.

This file, "ambiguity.md", is the expanded production contract.

The repository SHOULD progressively make "ambiguity-rules.md" a compatibility/redirect document pointing to this file rather than maintaining two independent normative ambiguity specifications.

No third ambiguity authority MAY be introduced.

---

3. Existing ANTLR Composition Must Be Preserved

The current repository intentionally uses:

grammar/Zamani.g4
        |
        +--> grammar/antlr/ZamaniParser.g4
        |
        +--> grammar/antlr/ZamaniLexer.g4

"Zamani.g4" MUST remain the root composition boundary.

It MUST NOT independently duplicate the detailed expression, type, quantum, HDL, or domain grammar.

Domain grammars MUST ultimately compose through the canonical parser/lexer hierarchy.

The ambiguity validator MUST therefore analyze the complete composed grammar, not merely individual ".g4" files in isolation.

A rule that appears unambiguous in isolation can become ambiguous after composition.

Therefore:

individual grammar validation
        +
composition validation
        =
production validation

---

4. Core Determinism Invariant

For a fixed:

source
language version
feature configuration
dialect configuration
macro configuration
grammar version

the syntactic result MUST be deterministic.

Conceptually:

Parse(
    source,
    language_version,
    feature_configuration,
    dialect_configuration,
    macro_configuration
)
=
one deterministic syntax result

or:

deterministically rejected

The parser MUST NOT choose an interpretation based on:

- CPU model;
- CPU count;
- GPU availability;
- FPGA availability;
- QPU availability;
- number of available qubits;
- QPU topology;
- memory capacity;
- network state;
- calibration;
- runtime state;
- queue state;
- filesystem contents;
- wall-clock time;
- randomness;
- environment variables;
- provider;
- vendor;
- optimization level;
- backend selection;
- scheduling policy;
- routing policy.

---

5. Parsing Is Not Semantic Resolution

Zamani MUST maintain the distinction:

lexical ambiguity
        ↓
syntactic ambiguity
        ↓
contextual syntax
        ↓
name resolution
        ↓
type resolution
        ↓
overload resolution
        ↓
resource/capability validation
        ↓
target realization

These are separate stages.

The parser MUST NOT perform semantic work merely to compensate for an ambiguous grammar.

Conversely, semantic analysis MUST NOT be forced to repair a grammar that has multiple structural interpretations.

The rule is:

«If two interpretations have different syntactic structure, the grammar must distinguish them.»

«If they have the same syntactic structure and differ only in meaning, semantic analysis owns the distinction.»

---

6. Ambiguity Classes

Zamani recognizes the following ambiguity classes.

6.1 Lexical ambiguity

Two lexer rules can recognize the same source sequence.

Examples include:

=
==
>
>=
>>
>>=

and keyword/identifier collisions.

Owner:

grammar/lexer/
grammar/antlr/ZamaniLexer.g4
src/lexer.rs

---

6.2 Token-boundary ambiguity

The same characters could be divided into tokens in different ways.

For example:

a+b

and:

a + b

MUST tokenize equivalently.

Whitespace MUST NOT arbitrarily alter token boundaries.

---

6.3 Syntactic ambiguity

The same token sequence admits multiple parse structures.

Example:

a + b * c

MUST have exactly one syntactic structure.

---

6.4 Precedence ambiguity

Multiple operators compete for ownership of operands.

The grammar MUST define a single precedence hierarchy.

---

6.5 Associativity ambiguity

Repeated operators MUST define whether grouping is:

left
right
non-associative

---

6.6 Contextual ambiguity

A token sequence has one structural interpretation only after considering a syntactic context.

Contextual interpretation MAY be used where explicitly specified.

It MUST NOT become arbitrary symbol-table-dependent parsing.

---

6.7 Name-resolution ambiguity

Two or more declarations can match the same name.

This is semantic, not grammatical.

The parser MUST preserve the name.

Semantic analysis resolves it.

---

6.8 Type ambiguity

A syntactically valid construct may have more than one possible type.

This belongs to type checking/inference.

---

6.9 Overload ambiguity

A call may match several valid declarations.

This belongs to overload resolution.

The parser produces a call structure; it does not select an overload based on declaration order.

---

6.10 Resource/capability ambiguity

A program may be valid but several targets may satisfy its requirements.

This MUST NOT change parsing.

Target selection belongs downstream.

---

6.11 Dialect ambiguity

Two active dialects may attempt to own the same syntax.

This MUST be rejected deterministically unless the dialect contract explicitly defines composition.

---

6.12 Macro ambiguity

Macro expansion can introduce syntax that conflicts with surrounding source.

Expansion MUST follow a deterministic phase and hygiene model.

---

7. Lexical Determinism

7.1 Longest-match rule

Where token candidates overlap, Zamani MUST use one documented deterministic policy.

For overlapping operators, the lexer SHOULD recognize the longest valid token.

Conceptually:

>>= 
>>
>

must not be selected arbitrarily.

The same applies to:

...
..
.

and any future multi-character operators.

---

8. Keyword Versus Identifier

Keywords MUST have one canonical vocabulary.

The canonical lexical authority is:

grammar/lexer/keywords.g4
grammar/lexer/keywords.md

Domain grammars MUST NOT independently redefine core keywords.

A keyword introduced by quantum, HDL, AI, networking, or another domain MUST undergo collision analysis before becoming globally reserved.

Where global reservation is unnecessary, Zamani SHOULD prefer:

- contextual keywords;
- qualified names;
- explicit namespaces;
- dialect-scoped vocabulary.

---

9. Existing Token Duplication Risks

The current frontend design contains distinct token concepts including:

Question
QuestionMark
Ampersand
BitAnd

These MUST NOT be merged merely because they share visual similarity.

The production rule is:

«Two token kinds MAY remain separate only when they represent genuinely distinct lexical or syntactic contracts.»

For example:

- "?" used as a lexical punctuation token;
- "?" used in an optional-type or try-expression context;

must be reconciled through a single canonical lexical policy rather than duplicated token semantics.

Likewise:

&

MUST have one lexical identity where possible, with syntactic context determining its meaning.

If "Ampersand" and "BitAnd" remain distinct token kinds, their exact lexical ownership MUST be documented and tested.

The validator MUST flag duplicate token spellings unless an explicit equivalence contract exists.

---

10. Operator Table Must Be Canonical

All operators MUST have one canonical record containing:

spelling
token
precedence
associativity
syntactic role
AST role
semantic role

The same operator MUST NOT receive different precedence in different domains.

For example:

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
=
+=
-=
*=
/=

must be globally coherent.

Domain-specific operators MAY be added through explicit extension contracts.

They MUST NOT silently redefine core operators.

---

11. Expression Parsing

The Rust parser currently uses a precedence-driven expression parser.

That implementation model is compatible with the production ambiguity contract provided that:

1. the precedence table is canonical;
2. every operator has exactly one precedence;
3. every operator has exactly one associativity;
4. postfix operators bind deterministically;
5. assignment precedence is explicitly defined;
6. range precedence is explicitly defined;
7. casts are explicitly defined;
8. try expressions are explicitly defined;
9. member access and calls bind deterministically;
10. parser behavior matches the grammar specification.

The grammar and "src/parser.rs" MUST NOT maintain independent precedence systems.

There MUST be one conformance matrix.

---

12. Canonical Expression Precedence

The exact precedence ordering MUST be maintained in:

grammar/expressions/precedence.md

and reflected in:

src/parser.rs

without semantic divergence.

The validator MUST check that every parser-supported operator is present in the canonical precedence contract.

No operator may exist only in:

src/parser.rs

without a specification entry.

No operator may exist only in:

grammar/expressions/

without frontend implementation status.

---

13. Assignment Versus Equality

The following MUST remain distinct:

x = y
x == y

Assignment MUST NOT be parsed as equality.

Compound assignment operators MUST also have explicit definitions.

The AST MUST preserve the distinction.

Semantic analysis MUST NOT be required to guess whether the source meant assignment or comparison.

---

14. Range Ambiguity

The following forms require explicit lexical and syntactic ownership:

a..b
a..=b
a.b
1.0
...

The lexer MUST deterministically distinguish:

.
..
..=
...

from decimal points and member access.

Range syntax MUST NOT depend on semantic type information.

---

15. Member Access Versus Range

A construct such as:

a.b

MUST remain distinguishable from:

a..b

The parser MUST NOT infer the distinction from the type of "a".

---

16. Generic Type Ambiguity

The current Rust parser supports generic syntax such as:

Foo<Bar>

and nested generic structures.

This MUST remain scalable.

Valid examples include:

Vector<T>
Map<K, V>
Tensor<T, Shape>
QuantumRegister<Qubit>
Map<String, Vector<Matrix<T>>>

The grammar MUST NOT impose a fixed generic nesting depth.

Any implementation recursion/resource limit is an implementation constraint, not a language-level maximum.

---

17. Generic Closers Versus Shift Operators

Zamani may encounter the classic conflict between:

>>
>>>

and nested generic closers:

Map<String, Vector<T>>

The language MUST define one deterministic strategy.

Acceptable strategies include:

1. lexical splitting;
2. parser-context interpretation;
3. dedicated generic-close tokens;
4. another formally specified mechanism.

The chosen strategy MUST be shared by:

grammar/lexer/
grammar/types/
grammar/expressions/
grammar/antlr/ZamaniLexer.g4
grammar/antlr/ZamaniParser.g4
src/lexer.rs
src/parser.rs

It MUST NOT be implemented differently in different frontend paths.

Regression tests MUST include arbitrary nested generic examples within available implementation resources.

---

18. Type Versus Expression Ambiguity

A construct such as:

Foo<Bar>

MUST NOT require arbitrary semantic lookup merely to decide whether it is structurally a type.

The parser MAY produce a generic syntactic form and semantic analysis may determine what "Foo" denotes.

But:

Foo < Bar > Baz

must follow the canonical expression/type grammar rather than a heuristic.

---

19. Type Versus Comparison Operators

Because "<" and ">" are both comparison operators and generic delimiters, the grammar MUST explicitly define the syntactic contexts in which each interpretation is legal.

The parser MUST NOT choose generic syntax simply because "Foo" happens to be a known type.

Likewise, it MUST NOT reject valid generic syntax merely because "Foo" is not yet resolved.

---

20. Declaration Versus Expression

Declaration and expression forms MUST be structurally distinguishable.

For example:

fn foo(...) { ... }

must not be interpreted as an ordinary expression.

A function call:

foo(...)

must remain a call regardless of whether "foo" is declared.

Name lookup occurs later.

---

21. Function Call Versus Function Declaration

The parser MUST NOT ask:

"Does foo already exist?"

to determine whether:

foo(...)

is a call.

The surrounding syntactic production determines whether the construct is a declaration.

This prevents:

- source-order dependence;
- symbol-table-dependent parsing;
- incremental-compilation instability.

---

22. Named Arguments

If named arguments are supported, the syntax MUST distinguish them from:

- assignment;
- type annotations;
- map entries;
- labels;
- pattern bindings.

For example, if:

foo(x: value)

is named-argument syntax, that meaning MUST be established by the call grammar.

The parser MUST NOT infer it from whether "x" happens to be a parameter name.

---

23. Parameter Lists

Parameter parsing MUST be deterministic for:

- ordinary parameters;
- mutable parameters;
- self parameters;
- typed parameters;
- generic parameters;
- default parameters;
- variadic parameters;
- constraints.

A parameter MAY have semantic constraints, but its structural role MUST be determined syntactically.

---

24. Lambda and Closure Ambiguity

Lambda/closure syntax MUST have an explicit grammar form.

If:

|x| x + 1

is supported, it MUST not conflict with:

|

as bitwise OR or pattern syntax.

The grammar MUST use sufficient surrounding structure to determine which construct is present.

---

25. Parenthesized Expression Versus Tuple

The grammar MUST distinguish:

(x)

from:

(x,)

and:

(x, y)

if tuples are supported.

The empty tuple:

()

must also have one canonical interpretation.

---

26. Block Versus Record/Object/Map Literal

If braces are used for blocks and data literals, the surrounding grammar MUST distinguish them.

For example:

if condition {
    ...
}

and:

{
    key: value
}

must not rely on symbol-table lookup to determine whether the braces are a block or value.

If ambiguity remains unavoidable, the language MUST introduce an explicit delimiter or introducer.

---

27. Pattern Matching

Pattern syntax MUST have deterministic ownership for:

- wildcard;
- identifier;
- literal;
- tuple;
- struct;
- enum;
- range;
- reference;
- guarded patterns;
- nested patterns.

A bare identifier in pattern position MUST have one syntactic interpretation.

Whether it denotes a binding or a constant is semantic.

---

28. "if" / "else" Association

Zamani MUST use one deterministic dangling-else policy.

The preferred rule is:

«An "else" associates with the nearest unmatched "if" in the same syntactic construct.»

Braces MUST continue to provide an explicit way to remove any visual uncertainty.

---

29. Optional Syntax

Optional constructs MUST NOT produce multiple parse trees.

For example:

type?

must have one syntactic interpretation if optional types use "?".

If "?" is also used for try expressions, the grammar MUST distinguish those contexts structurally.

The AST MUST preserve the selected syntactic role.

---

30. Try Expression Versus Optional Type

The current Rust parser distinguishes:

Expression::Try

and:

TypeExpr::Optional

The grammar contract MUST preserve this distinction.

Examples:

value?

in expression position:

try(value)

versus:

Type?

in type position:

Option<Type>

The parser MUST determine the syntactic context before AST construction.

---

31. Reference and Bitwise "&"

Because the frontend contains both:

Ampersand
BitAnd

the production grammar MUST establish whether these represent:

- one canonical token with multiple parser roles; or
- genuinely different lexer tokens.

The preferred architecture is:

one lexical spelling
        ↓
canonical token
        ↓
context-specific parser role

unless a lexer-level distinction is demonstrably necessary.

No domain grammar may independently redefine "&".

---

32. Pointer and Multiplication "*"

The same principle applies to:

*

which may represent:

- multiplication;
- dereference;
- pointer type;
- wildcard/pattern syntax;
- repetition.

These MUST be distinguished by syntactic context.

The lexer SHOULD emit one canonical token where possible.

The parser assigns its role.

---

33. Labels Versus Expressions

If labels are supported, their introducer MUST be distinct enough from ordinary expression syntax.

A label MUST NOT be recognized merely because the following identifier is unknown.

---

34. Semicolon Policy

Zamani MUST have one global semicolon policy.

If semicolons are mandatory:

statement ;

must be consistent.

If semicolons are optional:

the newline/termination rules MUST be specified centrally.

Individual domains MUST NOT invent independent semicolon insertion.

---

35. Newline Sensitivity

If newlines are syntactically relevant, that behavior MUST belong to the lexical/core specification.

A quantum grammar, HDL grammar, or AI grammar MUST NOT reinterpret newline independently.

---

36. Whitespace

Whitespace is non-semantic unless explicitly specified.

These MUST have equivalent syntax:

a+b

and:

a + b

Whitespace MUST NOT select:

- CPU versus GPU;
- classical versus quantum;
- HDL versus software;
- vendor dialect;
- optimization mode.

---

37. Comments

Comments MUST NOT repair invalid syntax.

Invalid:

apply H( // parser should ignore missing argument

must remain invalid.

Comments MUST NOT:

- terminate arbitrary constructs;
- close blocks;
- activate dialects;
- change precedence;
- create declarations.

Documentation comments may have explicitly specified metadata semantics, but those semantics must not change ordinary parsing unless the specification says so.

---

38. Attributes

Attributes MUST have one canonical introducer and one grammar.

Domain-specific attributes MUST reuse the universal attribute mechanism.

Quantum, HDL, AI, security, and hardware grammars MUST NOT create competing attribute syntax.

---

39. Pragmas

Pragmas MUST NOT mutate the parser invisibly.

A pragma MAY influence explicitly defined semantic behavior, but the grammar mode itself MUST remain deterministic.

Forbidden:

pragma magic_parser_mode

followed by source that acquires undocumented syntax.

Preferred:

pragma
    ↓
explicitly scoped declaration/configuration
    ↓
documented semantic effect

---

40. Module and Namespace Ambiguity

Qualified names MUST use one canonical grammar.

The existing core grammar areas:

grammar/core/names.g4
grammar/core/paths.g4
grammar/core/qualified-names.g4

MUST own qualified-name syntax.

Domain grammars MUST NOT create independent namespace separators.

---

41. Path Versus Operator Ambiguity

Filesystem paths, module paths, ranges, division, member access, and namespace separators MUST remain distinct.

The parser MUST NOT inspect the filesystem to resolve syntax.

This is required for:

- deterministic builds;
- sandboxing;
- reproducibility;
- cross-platform compilation;
- POCO-REAF.

---

42. Import Ambiguity

An import MUST have one canonical structure.

Whether the imported module exists is not a parsing question.

For example:

import foo::bar;

may be syntactically valid even when "foo::bar" cannot be found.

The latter is a module-resolution diagnostic.

---

43. Module Version Ambiguity

Version constraints MUST be represented explicitly.

The parser MUST NOT interpret an imported module differently based on whatever version happens to be installed.

Dependency resolution occurs outside parsing.

---

44. Effect Ambiguity

Effects MUST use the canonical effect grammar.

The parser identifies:

effect
effect operation
handler
effect set

Semantic analysis determines whether the effect is declared, supported, or handled.

Hardware capabilities MUST NOT alter parsing.

---

45. Memory Syntax

Memory syntax MUST distinguish:

ownership
borrowing
reference
allocation
address space
persistence
shared memory
distributed memory
accelerator memory
quantum memory

without requiring target hardware information.

A parser MUST NOT decide whether something is "GPU memory" because a GPU happens to be available.

---

46. Concurrency Ambiguity

Concurrency constructs such as:

async
await
spawn
parallel
channel
actor
task
pipeline

MUST have unique syntactic roles.

The grammar MUST NOT use a fixed number of threads or cores to determine meaning.

For example:

parallel {
    work()
}

is source intent.

How many workers are created is downstream.

---

47. Classical Computing Ambiguity

Classical domains MUST reuse the universal expression/type system.

Mathematical operations MUST NOT be duplicated into incompatible parser forms merely because they have different implementations.

For example:

matrix multiplication
FFT
gradient
optimization
statistics
linear algebra

may be represented through:

generic operation
typed operation
intrinsic
library call
capability

without creating conflicting grammar rules.

---

48. Quantum Grammar Invariant

Quantum is a domain of Zamani.

It is not a separate language.

Quantum syntax MUST enter the common:

lexer
→ parser
→ domain-neutral AST
→ semantic analysis
→ quantum::ir

pipeline.

No separate frontend quantum IR may be introduced merely to resolve grammar ambiguity.

---

49. Open-Ended Quantum Operations

The current quantum architecture correctly moves toward open-ended operations rather than a fixed gate enumeration.

The grammar MUST NOT require:

H
X
Y
Z
CNOT
CZ
SWAP
...

to be a finite parser-level enumeration.

The operation form SHOULD be structurally equivalent to:

operationSpecifier
quantumTargetList

with optional:

- parameters;
- modifiers;
- controls;
- adjoints;
- attributes;
- effects;
- result bindings.

Therefore these may be structurally valid:

apply H(q);
apply custom_gate(q);
apply vendor::operation(q0, q1);
apply operation(theta)(q);

The parser MUST NOT need to know whether the operation exists.

Semantic analysis resolves the operation.

---

50. Quantum Operation Resolution

For a quantum operation:

apply operation(args)(targets);

the parser owns:

- operation syntax;
- argument structure;
- target structure;
- modifier structure;
- source spans.

Semantic analysis owns:

- operation existence;
- signature;
- parameter types;
- target types;
- capability requirements;
- legality;
- dialect availability.

Backend owns:

- decomposition;
- physical gates;
- routing;
- scheduling;
- calibration.

---

51. Quantum Gate Versus Operation

A "gate" is a semantic/backend concept where applicable.

The source grammar SHOULD use the more general operation abstraction.

A new quantum technology MUST NOT require editing a finite:

quantumGate : H | X | ...

list merely to become syntactically representable.

---

52. Logical Versus Physical Quantum Resources

The source language MUST distinguish:

logical quantum resource

from:

physical target resource

when both are exposed.

Portable source syntax MUST default to logical semantics.

Physical qubit identifiers MUST be explicitly target-bound.

The parser MUST never infer physical mapping from:

q[0]

or another source-level index.

---

53. Quantum Resource Requirements

The following class of source expression is portable:

requires qubits >= n;

and:

requires capability("quantum.measurement");

The parser must preserve these as requirements.

It must not transform them into:

MAX_QUBITS

or physical allocation.

---

54. No Fixed Quantum Limits

The ambiguity validator MUST reject grammar constructs that encode universal quantum capacity such as:

MAX_QUBITS
MAX_LOGICAL_QUBITS
MAX_PHYSICAL_QUBITS
QUBIT_0
QUBIT_1
...

when used as the universal language resource model.

Explicit program values remain legal.

---

55. Quantum Measurement

Measurement syntax MUST be distinguishable from:

- ordinary function calls;
- assignments;
- declarations;
- classical reads.

Measurement results may enter ordinary classical expressions.

The parser MUST preserve the data dependency.

Semantic analysis determines whether the measurement is legal in the relevant quantum context.

---

56. Mid-Circuit Measurement

Mid-circuit measurement MUST NOT require parser knowledge of a specific QPU.

The syntax expresses source intent.

Whether a backend supports:

mid-circuit measurement

is a capability question.

---

57. Classical Feed-Forward

A construct such as:

when result == 1 {
    apply correction(q);
}

must be parsed independently of whether the current backend supports it.

Unsupported backend capability is not a syntax error.

---

58. Quantum Controls

Controlled operation syntax MUST describe transformation intent.

The grammar MUST NOT require a finite list of controlled gates.

For example:

control(operation)(control, target)

can be structurally valid.

Whether the operation admits a legal controlled realization is semantic/backend work.

---

59. Quantum Adjoint/Inverse

Adjoint and inverse modifiers MUST be syntactically composable without enumerating every operation.

The grammar MUST preserve:

adjoint(operation)

as an operation transformation.

The semantic layer determines whether an adjoint is defined.

---

60. QEC Ambiguity

QEC syntax expresses:

error-correction intent
fault-tolerance intent
logical-operation intent
noise constraints
reliability requirements

It MUST NOT encode:

- a finite code catalogue;
- physical topology;
- decoder implementation;
- fixed syndrome width;
- fixed number of physical qubits.

QEC implementation remains downstream.

---

61. HDL Ambiguity

HDL syntax MUST remain distinguishable from ordinary software syntax.

Hardware intent MAY be embedded into the unified language, but it MUST use explicit structural constructs.

HDL MUST NOT silently reinterpret ordinary expressions based on target availability.

---

62. Parameterized HDL

HDL syntax MUST support parameterized:

- widths;
- arrays;
- interfaces;
- modules;
- pipelines;
- memories;
- timing;
- clocks;
- generated structures.

It MUST NOT define universal fixed widths such as:

wire [31:0]

as the only legal model.

A concrete width may be program semantics.

A universal width ceiling is forbidden.

---

63. Hardware Resource Ambiguity

Hardware grammar MUST distinguish:

requirement
constraint
capability
preference
hint
realization

A source program may state:

requires capability("gpu.compute");

without identifying a physical GPU.

---

64. Distributed Computing

Distributed syntax MUST NOT encode a fixed node count.

The parser must accept resource-independent constructs such as:

distributed {
    ...
}

or the repository's equivalent canonical syntax.

The number of nodes is deployment information.

---

65. Networking

Network syntax MUST distinguish:

- address;
- endpoint;
- protocol;
- channel;
- request;
- response;
- service;
- route.

The parser MUST NOT query the network to decide which grammar production applies.

---

66. AI Grammar

AI syntax MUST not depend on a specific framework.

Frameworks such as:

- PyTorch;
- TensorFlow;
- JAX;
- vendor accelerators;
- future frameworks

are semantic/backend ecosystems.

The grammar should describe model/training/inference/data/tensor intent.

---

67. Tensor Ambiguity

Tensor syntax MUST be structurally generic.

Examples:

Tensor<T, shape>

must not imply a fixed rank.

A tensor shape may be:

- static;
- symbolic;
- inferred;
- dependent;
- runtime-known.

The grammar MUST NOT encode a maximum rank.

---

68. Resource Expressions

Resource expressions MUST be ordinary expressions or explicitly defined resource expressions.

The parser MUST NOT evaluate them using the current machine.

For example:

requires memory >= required_memory;

must remain source semantics.

The actual memory check belongs downstream.

---

69. Capability Names

Capability identifiers MUST be open-ended.

The grammar MUST NOT enumerate every hardware capability.

For example:

capability("quantum.measurement")
capability("tensor.compute")
capability("gpu.compute")

are data.

The parser does not need to know whether a provider currently implements them.

---

70. Dialects

A dialect MUST declare:

name
version
namespace
activation
owned syntax
extension points
semantic mapping
AST mapping
IR mapping
compatibility

Two active dialects MUST NOT silently define conflicting syntax.

If conflict cannot be resolved by explicit composition, compilation MUST produce:

DIALECT_SYNTAX_CONFLICT

rather than choosing one dialect.

---

71. Vendor Extensions

Vendor syntax MUST be namespaced whenever possible.

Preferred:

vendor::provider::operation

rather than globally reserving:

operation

Vendor availability MUST NOT alter core syntax.

---

72. Macro Ambiguity

Macro invocation MUST have one canonical form.

Macro expansion MUST be deterministic.

The macro system MUST define:

input tokens
→ invocation recognition
→ expansion
→ hygiene
→ resulting syntax
→ AST integration

The exact phase ordering MUST be shared by:

grammar/macros/
grammar/metaprogramming/
src/parser.rs

---

73. Macro Hygiene

Macro-generated identifiers MUST NOT accidentally capture identifiers from surrounding scopes.

Hygiene MUST protect:

- variables;
- functions;
- types;
- modules;
- effects;
- quantum resources;
- hardware resources;
- capabilities.

---

74. Macro Expansion Limits

The language MUST NOT define artificial semantic limits such as:

MAX_MACRO_DEPTH = 32

An implementation MAY protect itself against resource exhaustion.

Such limits MUST be:

- implementation/resource limits;
- configurable where practical;
- explicitly documented;
- distinguishable from language syntax;
- reported as implementation/resource failures.

They MUST NOT change the language's conceptual semantics.

---

75. Metaprogramming

Compile-time computation MUST be deterministic unless nondeterminism is explicitly represented by a language effect/capability.

Implicit dependencies on:

- time;
- random state;
- environment;
- filesystem;
- network;
- hardware;

are forbidden.

If requested explicitly, they must be represented in the semantic/effect/capability model.

---

76. Compile-Time Grammar Mutation

Compile-time code MUST NOT silently rewrite the grammar of already parsed source.

Grammar extension must occur through explicit:

- dialect;
- feature;
- macro;
- metaprogramming;
- syntax extension

contracts.

The parser MUST never acquire undocumented syntax because a compile-time function happened to execute.

---

77. Feature Gates

Feature gates MUST be explicit.

A feature gate may select whether syntax is enabled, but it MUST NOT select between two different meanings of the same valid source without a versioned language contract.

A feature gate must have:

feature ID
status
version
syntax owner
semantic owner
compatibility policy
diagnostic policy
tests

---

78. Version Ambiguity

Language version selection MUST occur before parsing.

The parser MUST NOT guess a language version from ambiguous syntax.

If source declares an explicit version, the version declaration MUST itself have a deterministic grammar.

---

79. Backward Compatibility

A new grammar production MUST undergo:

1. token collision analysis;
2. parser ambiguity analysis;
3. AST compatibility analysis;
4. semantic compatibility analysis;
5. dialect analysis;
6. migration analysis;
7. negative-test analysis.

Existing valid programs MUST NOT silently acquire a different parse tree merely because a new feature was added.

---

80. Reserved Space

Future syntax may be reserved explicitly.

Reserved syntax MUST NOT be treated as silently valid syntax.

The language should distinguish:

valid
experimental
reserved
deprecated
invalid

rather than treating unknown constructs as future syntax.

---

81. Error Recovery

Error recovery MUST be deterministic.

Given the same invalid source and parser configuration, the diagnostic sequence MUST be stable.

Recovery MUST NOT depend on:

- hash-map ordering;
- thread scheduling;
- hardware;
- network;
- filesystem;
- random choice.

---

82. Error Recovery Must Not Create Fake Syntax

The parser MUST NOT recover by silently inventing declarations, operators, or blocks that change semantic meaning.

Recovery nodes MAY be marked explicitly as synthetic/error nodes.

They MUST NOT be treated as valid program constructs.

---

83. Diagnostic Determinism

Diagnostics MUST preserve:

- source span;
- error category;
- stable diagnostic code;
- deterministic ordering;
- relevant expected tokens;
- actual token;
- context.

Diagnostics SHOULD identify ambiguity when ambiguity is the actual failure.

---

84. Ambiguity Diagnostic Categories

The validator/parser SHOULD use stable categories including:

AMBIGUOUS_LEXEM
AMBIGUOUS_TOKEN
AMBIGUOUS_OPERATOR
AMBIGUOUS_PRECEDENCE
AMBIGUOUS_ASSOCIATIVITY
AMBIGUOUS_DECLARATION
AMBIGUOUS_TYPE_EXPRESSION
AMBIGUOUS_PATTERN
AMBIGUOUS_DIALECT
AMBIGUOUS_MACRO
AMBIGUOUS_IMPORT
AMBIGUOUS_NAME
AMBIGUOUS_OVERLOAD
AMBIGUOUS_RESOURCE
AMBIGUOUS_CAPABILITY

Not all of these are parser errors.

The diagnostic category MUST identify the responsible layer.

---

85. AST Determinism

For every syntactically valid source program, the parser MUST produce one AST structure.

The AST MUST preserve enough information to distinguish constructs whose semantic resolution remains deferred.

The AST SHOULD preserve:

- source span;
- source ordering;
- qualified names;
- modifiers;
- attributes;
- generic arguments;
- expressions;
- operation designators;
- quantum targets;
- resource requirements;
- capability requirements;
- dialect metadata.

The AST MUST NOT silently collapse distinct syntactic constructs.

---

86. Domain-Neutral AST

The frontend AST remains domain-neutral.

Quantum, HDL, classical, AI, and other domains MAY have semantic annotations or structured nodes where justified, but the frontend MUST NOT become a collection of backend-specific ASTs.

The target architecture remains:

source
  ↓
domain-neutral AST
  ↓
semantic analysis
  ↓
canonical semantic representation
  ↓
canonical IR

---

87. Canonical Quantum IR Boundary

Quantum ambiguity resolution MUST terminate before the canonical quantum IR boundary.

The path is:

Zamani source
    ↓
lexer
    ↓
parser
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
target

The grammar MUST NOT create a competing frontend quantum IR.

---

88. Semantic Overload

It is acceptable for:

operation(...)

to remain semantically unresolved after parsing.

It is not acceptable for the parser to produce two different AST structures depending on which declaration exists.

The correct model is:

one syntax
    ↓
one AST
    ↓
candidate semantic interpretations
    ↓
deterministic semantic resolution

---

89. Overload Resolution Must Be Stable

If several overloads match, the semantic resolver MUST use a specified ranking algorithm.

It MUST NOT depend on:

- declaration order;
- file order;
- hash-map order;
- linker order;
- backend;
- optimization level;
- machine architecture.

If no unique candidate exists, emit:

AMBIGUOUS_OVERLOAD

with deterministic candidate ordering.

---

90. Resource Selection Is Not Parsing

Consider:

requires qubits >= n;

The parser accepts the structure.

The semantic layer validates the requirement's type and meaning.

The target layer checks whether resources exist.

A QPU with insufficient resources MUST NOT cause the source to acquire another parse.

---

91. POCO-REAF Invariant

The following source:

program

must preserve its source-level meaning when compiled for:

embedded
CPU
multicore
GPU
FPGA
ASIC
QPU
quantum simulator
accelerator
HPC
cluster
distributed system
cloud
future target

subject to:

- semantic compatibility;
- capability availability;
- resource feasibility;
- explicitly defined target constraints.

The parser MUST be target-independent.

---

92. Hardware Must Never Resolve Grammar

The following are forbidden parser inputs:

number_of_cpus
number_of_gpus
number_of_qubits
memory_size
device_count
topology
clock_rate
vendor
driver
calibration
runtime_queue

Hardware data may be consumed later by:

- compilation;
- routing;
- scheduling;
- resource analysis;
- HAL;
- runtime.

---

93. No Universal Capacity Constants

The ambiguity validator MUST reject universal grammar-level constructs corresponding to:

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

The repository's hard-coding validator remains authoritative for the broader audit:

grammar/validation/hardcoding-audit.md

This file defines the ambiguity consequences of such hard-coding.

---

94. Explicit Numeric Values Are Not Ambiguity

These are valid:

1024
1000000
1_000_000

when they are program values.

The ambiguity validator MUST NOT incorrectly classify ordinary numeric literals as hardware limits.

The distinction is:

program value

versus:

language implementation ceiling

---

95. Arbitrary Magnitude

Numeric grammar MUST support the numeric representations defined by the language specification without introducing target-size assumptions into syntax.

The parser MUST NOT reject a number merely because a particular machine cannot directly represent it in a native register.

Representation feasibility belongs to the type/semantic/compiler layers.

---

96. Quantum Literal Ambiguity

Quantum literals such as:

|0⟩
|1⟩
|+⟩
|-⟩
|ψ⟩

MUST have explicit lexical ownership.

The lexer MUST distinguish them from:

- bitwise OR;
- identifier syntax;
- ordinary delimiters.

Generalized state literals MUST be extensible without requiring a finite enumeration of states.

---

97. HDL Literal Ambiguity

HDL-specific literals MUST be explicit.

Widths may be semantic data.

The grammar MUST NOT make one fixed width the universal default merely because current hardware commonly uses it.

---

98. Attribute Versus Macro

Attributes and macros MUST have different canonical introducers.

A macro invocation MUST NOT accidentally become an attribute.

An attribute MUST NOT become executable source merely because a macro exists with the same name.

---

99. Directive Versus Expression

Compiler directives MUST have a dedicated syntax.

The existing ecosystem has discussed forms resembling:

unsafe!(evas:{...})

Such special forms MUST NOT be parsed accidentally as:

identifier
!
(
...
)

unless that ordinary expression grammar is intentionally the canonical representation.

Any special directive syntax MUST have an explicit grammar contract.

---

100. "unsafe" Policy

Zamani's production Rust compiler implementation is safe Rust.

The presence of an "unsafe" keyword in Zamani source, if retained for language-level semantics, MUST NOT imply that Rust implementation code may use "unsafe".

Language-level "unsafe" and Rust implementation safety are separate concepts.

The ambiguity validator MUST NOT permit a language feature to force unsafe Rust implementation.

---

101. Security-Sensitive Ambiguity

Ambiguity is a security concern.

The parser MUST reject syntax that could be interpreted differently by:

- compiler frontend;
- formatter;
- IDE;
- linter;
- semantic analyzer;
- optimizer;
- backend.

All tooling MUST consume the same canonical syntax contracts.

---

102. Formatter Stability

A formatter MUST NOT transform a valid source program into a form with a different parse tree.

Required property:

parse(format(source))
=
parse(source)

for every formatter-supported valid source.

The ambiguity test suite SHOULD include formatter round-trip tests.

---

103. Serialization Stability

AST serialization/deserialization MUST preserve syntactic distinctions relevant to semantics.

Round-trip property:

parse
→ AST
→ serialize
→ deserialize
→ equivalent AST

must hold.

---

104. Incremental Parsing

Incremental parsing MUST preserve the same result as parsing the complete source from scratch.

Conceptually:

parse(full_source)

and:

incrementally_parse(edits)

must produce equivalent syntax trees.

The result MUST NOT depend on stale symbol tables or target information.

---

105. Parallel Parsing

If parsing is parallelized internally, parallelism MUST NOT affect:

- parse result;
- AST ordering;
- diagnostic ordering;
- ambiguity classification.

Parallel execution is an implementation strategy, not language semantics.

---

106. Deterministic Collections

Internal Rust collections used by parser/validator code MUST NOT cause externally visible nondeterminism.

When diagnostics or ambiguity reports contain collections of candidates/rules, output ordering MUST be canonical.

Sorting MUST use stable language-defined identifiers, source positions, or another explicit deterministic key.

---

107. Recursion and Scalability

Grammar design MUST NOT impose arbitrary semantic limits such as:

MAX_NESTING
MAX_GENERIC_DEPTH
MAX_EXPRESSION_DEPTH
MAX_BLOCKS
MAX_DECLARATIONS
MAX_OPERANDS
MAX_TARGETS

The grammar must remain conceptually scalable.

The implementation MAY have resource protection against stack exhaustion or denial-of-service inputs.

Such protection is an implementation safety policy, not language semantics.

---

108. Stack Safety

Because Rust production code MUST use no "unsafe", parser implementations SHOULD avoid designs that require unbounded native-stack recursion for attacker-controlled source.

Where practical, deep structures SHOULD use iterative parsing or explicitly managed stacks.

A resource-exhaustion diagnostic MUST be distinct from:

syntax error

and:

ambiguous syntax

---

109. Parser Resource Limits

Implementation resource limits MAY exist for:

- memory;
- CPU;
- recursion;
- token count;
- diagnostic count;
- macro expansion;
- generated AST size.

They MUST be externally distinguishable from language restrictions.

The implementation MUST NOT pretend:

resource exhausted

means:

invalid Zamani syntax

---

110. No Fixed Quantum Parser Depth

Quantum circuit depth is program semantics.

The grammar MUST NOT define a universal circuit-depth limit.

---

111. No Fixed HDL Width

HDL width is program semantics.

The grammar MUST NOT define a universal maximum bus/register width.

---

112. No Fixed Tensor Rank

Tensor rank is program semantics.

The grammar MUST NOT define a universal rank maximum.

---

113. No Fixed Distributed Node Count

Distributed scale is deployment/resource semantics.

The grammar MUST NOT encode a fixed node maximum.

---

114. No Fixed Timeline Count

Temporal/multi-timeline constructs MUST NOT define a universal maximum number of timelines.

---

115. Future-Domain Safety

A future domain MUST be introducible without modifying unrelated core grammar.

For example, adding a future accelerator domain MUST NOT require changing:

- arithmetic precedence;
- function calls;
- module paths;
- basic types;
- ordinary identifiers.

Future domains should enter through explicit extension points.

---

116. Extension Points Must Be Narrow

An extension point MUST NOT become:

anything

or:

arbitrary token stream

without semantic ownership.

Every extension MUST identify:

syntax owner
AST mapping
semantic owner
IR mapping
compatibility
diagnostics
tests

---

117. Domain Composition

The following domains share the same syntax foundation:

classical
quantum
HDL
hybrid
AI
distributed
data
networking
security
hardware
resources
compile
execution
interoperability

No domain may redefine:

- identifier syntax;
- basic expression precedence;
- generic syntax;
- module paths;
- universal attributes;
- core literals

without an explicit compatibility contract.

---

118. Interoperability Formats

OpenQASM, QIR, LLVM/MLIR-oriented forms, HDL formats, foreign-language syntax, and other external representations are interoperability boundaries.

They MUST NOT silently become competing Zamani grammar authorities.

Import/export grammar MAY parse foreign formats, but canonical Zamani source semantics remain governed by Zamani.

---

119. Foreign Syntax

FFI declarations MAY expose foreign calling conventions and types.

They MUST NOT redefine ordinary Zamani expression syntax.

---

120. Semantic Diagnostics Versus Syntax Diagnostics

The compiler MUST distinguish:

SYNTAX_ERROR

from:

UNKNOWN_NAME
TYPE_ERROR
AMBIGUOUS_OVERLOAD
MISSING_CAPABILITY
INSUFFICIENT_RESOURCE
UNSUPPORTED_TARGET
DIALECT_CONFLICT

For example:

apply nonexistent_operation(q);

may be syntactically valid.

If no semantic declaration resolves it, the diagnostic is semantic.

It MUST NOT be reported as a parser ambiguity merely because the operation is unknown.

---

121. Negative Tests

Every ambiguity-sensitive construct MUST have negative tests.

Examples:

unterminated generic
malformed range
conflicting dialect
duplicate operator
ambiguous overload
invalid macro expansion
conflicting contextual keyword
invalid attribute placement
malformed quantum operation
malformed HDL declaration

Negative tests MUST assert diagnostic category where stable diagnostics are part of the contract.

---

122. Boundary Tests

Boundary tests MUST include:

- empty source;
- one-token source;
- deeply nested expressions;
- nested generics;
- nested blocks;
- long qualified names;
- long parameter lists;
- long quantum target lists;
- large tensor shape expressions;
- large resource expressions;
- deeply composed dialects within allowed implementation resources.

No boundary test may accidentally establish an artificial language maximum.

---

123. Scalability Tests

The ambiguity suite MUST contain parametrized/property-based scalability tests.

At minimum:

N declarations
N generic nesting levels
N expression terms
N function arguments
N quantum targets
N quantum operations
N HDL ports
N distributed resources
N dialect declarations
N macro expansions

where "N" is generated by the test harness rather than hard-coded as the language maximum.

Tests should validate monotonic scaling rather than establish a ceiling.

---

124. Determinism Tests

For every ambiguity-sensitive fixture:

parse(source)

MUST be executed repeatedly.

Results MUST be equivalent.

Where the parser is parallelized, tests MUST also compare:

single-threaded parse
parallel parse

where those modes exist.

---

125. Differential Tests

Where ANTLR and the Rust frontend both serve conformance purposes, differential tests SHOULD compare:

ANTLR parse result

against:

Rust parser result

at the semantic/conformance level.

Differences MUST be classified as:

- implementation gap;
- specification gap;
- intentional frontend difference;
- actual ambiguity.

They MUST NOT be silently ignored.

---

126. Lexer/Parser Conformance

The ambiguity validator MUST compare:

grammar/antlr/ZamaniLexer.g4

with:

src/lexer.rs

for:

- token spelling;
- token identity;
- longest-match behavior;
- keyword behavior;
- operator behavior;
- punctuation;
- literals.

Likewise:

grammar/antlr/ZamaniParser.g4

must be compared with:

src/parser.rs

for parser-supported constructs.

---

127. AST Conformance

Every ambiguity-sensitive grammar construct MUST map to one known AST representation.

The validator MUST detect:

grammar production
    without AST mapping

and:

AST construct
    without grammar/source representation

unless explicitly marked as generated/internal.

---

128. Quantum AST Conformance

Quantum syntax MUST map through the domain-neutral AST contract.

The AST must preserve, where applicable:

operation designator
qualified operation name
parameters
targets
controls
adjoints
results
measurement destination
classical dependencies
resource requirements
capability requirements
attributes
modifiers
source spans

The grammar MUST NOT require a finite "QuantumGate" enumeration.

---

129. Canonical IR Conformance

A grammar construct is not production-complete merely because it parses.

The conformance chain is:

grammar
→ AST
→ semantics
→ canonical IR

For quantum:

grammar
→ AST
→ semantic quantum operation
→ quantum::ir

For hardware:

grammar
→ AST
→ hardware intent
→ canonical semantic representation
→ target lowering

---

130. Ambiguity Must Not Be Resolved by Lowering

Optimization, routing, scheduling, QEC, resilience, ZQN, HAL, and backend lowering MUST NOT determine what source syntax means.

They may transform already-resolved semantics.

They MUST NOT retroactively choose between two source parses.

---

131. Optimization Invariant

For:

source → AST → semantic model → IR

optimization MUST preserve semantics.

If two source interpretations are possible, optimization is too late to decide which source interpretation was intended.

---

132. Routing Invariant

Quantum routing MUST NOT decide whether:

operation(a,b)

means one source construct or another.

It receives canonical quantum semantics after parsing and semantic analysis.

---

133. Scheduling Invariant

Scheduling MUST NOT influence parsing.

The source meaning MUST be identical whether the scheduler selects:

ASAP
ALAP
list scheduling
resource constrained scheduling
adaptive scheduling

or another policy.

---

134. QEC/ZQN Invariant

QEC and ZQN MAY validate or transform quantum semantics.

They MUST NOT introduce parser ambiguity.

Noise, calibration, reliability, and fault models are downstream concerns.

---

135. Hardware Availability Invariant

A source program MUST parse identically when:

GPU exists

and:

GPU does not exist

Likewise for:

- QPU;
- FPGA;
- ASIC;
- accelerator;
- cluster;
- network;
- memory capacity.

Only later capability/resource validation changes.

---

136. Reproducibility

A parse result MUST be reproducible from declared source/configuration inputs.

No hidden inputs are permitted.

The parser MUST NOT inspect:

HOME
PATH
environment variables
current directory
network
hardware
time
randomness

to choose a parse.

---

137. Security Boundary

The ambiguity validator itself MUST be deterministic and side-effect free.

It MUST NOT:

- execute Zamani code;
- execute macros with uncontrolled side effects;
- access hardware;
- access network;
- read arbitrary filesystem data;
- depend on environment variables.

Rust implementation MUST remain safe Rust.

---

138. Grammar-Level Duplicate Rule Detection

The validator MUST detect:

- duplicate parser rule names;
- duplicate lexer rule names;
- duplicate token names;
- duplicate token spellings;
- duplicate fragment names;
- conflicting imports;
- conflicting alternatives;
- unreachable alternatives;
- shadowed alternatives;
- impossible alternatives.

A duplicate is not automatically harmless because two rules "mean the same thing."

Ownership must be explicit.

---

139. Shadowed Alternatives

Consider:

rule
    : IDENTIFIER
    | specificIdentifier
    ;

If "specificIdentifier" is already covered by "IDENTIFIER", the validator MUST flag the shadowing.

The grammar author must either:

- reorder/reshape the grammar;
- remove the redundant alternative;
- document an explicit semantic predicate strategy where supported and safe.

---

140. Empty Alternatives

Empty alternatives MUST be explicitly justified.

Uncontrolled epsilon productions can create:

- ambiguous parses;
- infinite derivations;
- unexpected optionality;
- poor error recovery.

The validator MUST detect nullable cycles.

---

141. Nullable Cycles

The validator MUST detect grammar cycles in which every path is nullable.

Example pattern:

A : B ;
B : A ;

or more complex equivalent nullable cycles.

Such cycles MUST be rejected.

---

142. Left Recursion

ANTLR4 supports certain forms of left recursion, especially expression parsing.

Therefore:

«Left recursion is not automatically an error.»

The validator MUST distinguish:

supported direct left recursion

from:

unsupported indirect left recursion

and:

accidental recursion

Expression precedence must remain canonical.

---

143. Indirect Left Recursion

Cycles such as:

A → B
B → C
C → A

must be rejected unless the parser architecture explicitly proves that the cycle is valid and terminating.

---

144. Precedence Conflicts

A parser rule MUST NOT encode an operator at two incompatible precedence levels.

If:

+

appears in two expression tiers with different binding behavior, the validator MUST fail.

---

145. Associativity Conflicts

The same operator MUST NOT be:

left-associative

in one grammar branch and:

right-associative

in another.

---

146. Ambiguous Optional Sequences

Constructs like:

A : B? C? ;

may create multiple derivations if "B" and "C" overlap.

The validator MUST analyze nullable/overlapping alternatives rather than assuming optionality is harmless.

---

147. Prefix/Postfix Ambiguity

Prefix and postfix operators MUST have explicit ownership.

For example:

++x
x++

if supported must not conflict with ordinary "+" expressions.

---

148. Member Access and Calls

Postfix constructs MUST be ordered deterministically:

value.member
value(args)
value[index]
value?
value++

if supported.

The canonical postfix grammar must prevent multiple AST shapes for the same token stream.

---

149. Indexing Versus Generic Syntax

The parser MUST distinguish:

Type<T>

from:

value[index]

structurally.

Generic delimiters and indexing delimiters MUST not be mixed.

---

150. Array Type Versus Index Expression

The same brackets may be used in different contexts.

For example:

[T]

as a type and:

value[index]

as an expression are acceptable only because their surrounding grammar positions differ.

Semantic type lookup MUST NOT be required merely to determine bracket role.

---

151. Function Type Versus Grouped Type

Forms such as:

(A) -> B

must have one type grammar interpretation.

The parser MUST NOT confuse function types with parenthesized expressions.

---

152. Generic Constraints

Generic constraints MUST have a deterministic delimiter.

For example:

T: Trait

must not conflict with:

- labels;
- named arguments;
- type annotations;
- map entries.

Context determines role only where the grammar explicitly provides distinct productions.

---

153. Attribute Placement

Attributes MUST have explicitly defined attachment points.

An attribute must not sometimes attach to:

- next declaration;
- previous declaration;
- entire block;
- expression

based on parser heuristics.

---

154. Modifier Ordering

Modifiers MUST either:

1. have canonical ordering; or
2. be order-independent by grammar design.

If order is semantically irrelevant, the AST SHOULD canonicalize modifier representation.

If order is semantically relevant, grammar and specification MUST state it.

---

155. Declaration Prefix Ambiguity

If multiple declarations begin with the same modifiers/attributes, the parser MUST use an explicit discriminator.

It must not parse a generic prefix and then guess later based on symbol lookup.

---

156. Resource Declaration Ambiguity

Resource declarations MUST remain distinct from ordinary variables and expressions where necessary.

A resource name must not become a special token solely because a target backend exposes a resource of that name.

---

157. Capability Declaration Ambiguity

Capability identifiers are data.

The grammar MUST NOT reserve every known capability as a keyword.

New capabilities should generally be representable without modifying the lexer.

---

158. String-Based Capability Names

Constructs such as:

capability("gpu.compute")

avoid lexical explosion.

The parser should treat the capability string as data.

The capability registry is semantic/tooling infrastructure.

---

159. Hardware Target Names

Hardware target identifiers SHOULD remain data/qualified names rather than global keywords.

The grammar MUST NOT need updates whenever a new:

- CPU;
- GPU;
- FPGA;
- QPU;
- accelerator;
- vendor;
- architecture

is introduced.

---

160. Device Identifiers

Physical identifiers belong to target realization.

A target-specific language extension MAY expose them, but such syntax MUST be explicitly marked target-bound.

Portable source MUST NOT require physical identifiers.

---

161. Interoperability Ambiguity

External formats such as:

OpenQASM
QIR
Verilog
SystemVerilog
VHDL
C
C++
Rust
Python
Wasm

MUST remain explicit interoperability boundaries.

They MUST NOT silently alter Zamani's core parser.

---

162. Foreign Language Blocks

If Zamani supports foreign-language blocks, the block introducer MUST identify:

language
version
boundary
payload

The foreign payload MUST NOT be parsed as Zamani unless explicitly requested.

---

163. Embedded DSLs

Embedded DSLs MUST have explicit boundaries.

An embedded language cannot silently claim ordinary Zamani syntax.

---

164. Future Syntax

Future syntax MUST be introduced through:

proposal
→ semantic contract
→ AST contract
→ grammar
→ implementation
→ IR
→ tests
→ compatibility
→ stable

A feature appearing only in:

grammar/Zamani-Grammar.md

is not automatically legal syntax.

---

165. Historical Grammar Material

"grammar/Zamani-Grammar.md" remains useful as an extended/historical source.

Its constructs MUST be classified:

stable
proposed
experimental
deprecated
historical
not implemented

Unimplemented historical syntax MUST NOT create parser ambiguity.

---

166. Generated "grammar.md"

"grammar/grammar.md" describes actual implementation conformance.

It MUST NOT become a second normative grammar.

Ambiguity validation SHOULD compare it with:

Zamani.g4
lexer/parser implementation

and flag drift.

---

167. Rule Ownership

Every parser rule MUST have exactly one canonical owner.

For example:

expression

belongs to expressions/core composition.

Quantum grammar MAY expose:

quantumExpression

but must not redefine ordinary expression precedence.

HDL grammar MAY expose:

hdlExpression

only when that is a documented domain-specific construct.

---

168. Duplicate Domain Grammar

The repository contains several files with overlapping names such as:

quantum.g4
operations.g4
gates.g4
parameterized-operations.g4
controlled-operations.g4

These MUST have explicit ownership.

Multiple files MAY exist for maintainability, but they MUST NOT independently define competing public productions.

One composition file must own the public entry point.

---

169. Quantum Gate File

"grammar/quantum/gates.g4" MUST NOT become a finite universal gate catalogue.

If it exists, it should describe gate-related semantic syntax or compatibility forms.

Generic operation syntax remains the extensible path.

---

170. Duplicate Files Are Not Automatically Duplicate Authorities

The repository intentionally contains many specialized files.

The validator must distinguish:

fragment/support file

from:

public grammar owner

Every file should declare:

Purpose
Owns
Does Not Own
Public Rules
Imported Rules
Consumers

---

171. Integration Contract for This File

"grammar/validation/ambiguity.md" owns:

- ambiguity taxonomy;
- determinism requirements;
- cross-file ambiguity rules;
- validation invariants;
- classification;
- production acceptance criteria.

It does NOT own:

- lexical token definitions;
- AST definitions;
- semantic type rules;
- quantum IR;
- hardware topology;
- runtime scheduling.

Those remain owned by their existing files.

---

172. Integration With "grammar/validation/ambiguity-rules.md"

Do not rename or delete the existing file.

It SHOULD be converted into a compatibility entry point containing:

This document is retained for compatibility.
The canonical production ambiguity specification is:

grammar/validation/ambiguity.md

Until that update is made, both documents MUST remain semantically aligned.

No rule may be added to only one file.

---

173. Integration With "grammar/validation/grammar-validator.md"

The grammar validator MUST implement checks for:

- duplicate rules;
- duplicate tokens;
- duplicate token spellings;
- unreachable alternatives;
- shadowed alternatives;
- nullable cycles;
- unsupported left recursion;
- precedence conflicts;
- associativity conflicts;
- overlapping literals;
- keyword collisions;
- contextual keyword collisions;
- dialect conflicts;
- duplicate ownership;
- parser/lexer divergence.

---

174. Integration With "grammar/validation/hardcoding-audit.md"

The ambiguity validator delegates hardware-capacity detection to the hard-coding audit.

It adds the requirement that hardware-specific constants MUST NOT be used to choose syntax.

---

175. Integration With "grammar/validation/scalability-rules.md"

Scalability validation MUST confirm that ambiguity does not increase merely because program size increases.

For generated program families:

P(1)
P(10)
P(100)
P(N)

the grammar should preserve the same structural rules.

---

176. Integration With "grammar/validation/semantic-boundaries.md"

Semantic ambiguity MUST be deferred only when the AST can represent the unresolved construct unambiguously.

The rule is:

one syntax tree
+
deferred semantic identity

is valid.

two possible syntax trees
+
semantic guess

is invalid.

---

177. Integration With "grammar/expressions/precedence.md"

"precedence.md" is the canonical detailed expression precedence contract.

This file defines the ambiguity requirements around it.

The two files MUST agree.

---

178. Integration With "src/lexer.rs"

The Rust lexer MUST conform to:

grammar/lexer/
grammar/antlr/ZamaniLexer.g4

The ambiguity validator MUST detect divergence.

The lexer MUST remain safe Rust.

---

179. Integration With "src/parser.rs"

The Rust parser MUST conform to the canonical precedence, type, declaration, statement, and expression contracts.

The existing Pratt-style expression parser is acceptable as an implementation strategy.

Its precedence table MUST remain synchronized with the grammar specification.

---

180. Integration With "src/ast/"

The AST MUST preserve syntactic distinctions required for deterministic semantic resolution.

It MUST NOT collapse:

assignment
equality
range
member access
call
cast
try
optional type

into indistinguishable nodes.

---

181. Integration With Quantum Frontend

Quantum frontend formats such as OpenQASM are interoperability formats.

They MUST lower into the canonical Zamani quantum semantic model rather than creating a second source grammar authority.

---

182. Integration With "quantum::ir"

No ambiguity resolution may occur after entering canonical "quantum::ir".

The IR represents already-resolved meaning.

---

183. Integration With Routing

Routing receives resolved quantum operations.

It MUST NOT decide source-level operation identity.

---

184. Integration With Scheduling

Scheduling receives resolved dependencies and resources.

It MUST NOT determine source syntax.

---

185. Integration With QEC and ZQN

QEC and ZQN consume semantic quantum information.

They MUST NOT determine whether source syntax is valid.

---

186. Integration With HAL

HAL answers questions such as:

Can target T realize capability C?

It MUST NOT answer:

What did this source token mean?

---

187. Integration With Runtime

Runtime behavior MUST NOT retroactively alter the source parse.

Dynamic capabilities may affect execution feasibility, not source grammar.

---

188. Parser Configuration

Any parser configuration that changes syntax MUST be explicit and versioned.

Examples:

language version
enabled feature set
dialect set
macro configuration

Hidden parser configuration is prohibited.

---

189. Environment Independence

Parsing MUST produce the same result regardless of:

OS
CPU
GPU
QPU
filesystem
network
current directory
environment variables
time
locale
randomness

unless the language specification explicitly makes one of these a declared input.

---

190. Locale Independence

Identifiers, operators, keywords, and grammar punctuation MUST NOT depend on machine locale.

Unicode normalization policy MUST be explicit.

---

191. Unicode Ambiguity

Unicode identifiers MUST use a documented normalization/security policy.

Visually similar characters MUST NOT silently produce different meanings where the language policy intends canonicalization.

The lexer MUST preserve source spans correctly.

---

192. Source Span Preservation

Every token and AST construct involved in ambiguity diagnostics MUST preserve source spans.

At minimum:

start byte
end byte
line
column

or the canonical equivalent defined by the source-map contract.

---

193. Error Span Determinism

For the same source, an ambiguity diagnostic MUST identify the same source region.

Diagnostic spans MUST NOT change because parser alternatives are traversed in a different internal order.

---

194. Ambiguity Test Fixture Format

Every ambiguity fixture SHOULD record:

fixture ID
language version
feature configuration
dialects
source
expected result
expected AST class
expected diagnostic code
expected diagnostic span

---

195. Positive Fixture

A positive ambiguity fixture proves:

source
→ exactly one parse

It SHOULD also verify the relevant AST shape.

---

196. Negative Fixture

A negative ambiguity fixture proves:

source
→ deterministic rejection

The diagnostic should identify why.

---

197. Differential Fixture

A differential fixture proves that:

ANTLR

and:

Rust frontend

agree on the source's canonical syntactic interpretation.

---

198. Property-Based Ambiguity Testing

The validator SHOULD generate expressions and nested constructs automatically.

Properties include:

parse(source) is deterministic
format(source) preserves parse
AST serialization preserves structure
adding irrelevant whitespace preserves parse
comments preserve parse

---

199. Fuzzing

Grammar fuzzing SHOULD target:

- operators;
- generic delimiters;
- nested expressions;
- attributes;
- macros;
- dialect boundaries;
- quantum operations;
- HDL declarations;
- resource expressions.

A fuzzing failure MUST distinguish:

panic
timeout
resource exhaustion
syntax ambiguity
wrong AST
diagnostic instability

---

200. No Unsafe Fuzzing Infrastructure

The Rust parser and ambiguity validator MUST remain safe Rust.

Fuzzing infrastructure MUST NOT require "unsafe" to establish production correctness.

---

201. Performance and Ambiguity

A grammar can be deterministic and still catastrophically slow.

Production validation MUST therefore check for:

- pathological backtracking;
- exponential alternatives;
- pathological nullable combinations;
- excessive lookahead;
- ambiguous prefix sets;
- unnecessary semantic predicates.

---

202. Complexity Requirement

The grammar SHOULD be designed so ordinary parsing is approximately linear in source size for normal programs.

Where a construct requires more complex analysis, the complexity must be documented.

No parser optimization may change semantics.

---

203. Semantic Predicates

Semantic predicates SHOULD be minimized.

If used, they MUST be:

- deterministic;
- side-effect free;
- independent of hardware;
- independent of runtime state;
- independent of filesystem/network state;
- explicitly documented.

Semantic predicates MUST NOT become hidden semantic analyzers.

---

204. Target-Dependent Predicates Are Forbidden

Forbidden:

if GPU_available then parse X else parse Y

Forbidden:

if QPU_has_capability then interpret token as ...

Forbidden:

if memory >= ... then grammar branch A

Target information belongs downstream.

---

205. Resource-Dependent Predicates Are Forbidden

A resource requirement MAY be syntactically represented.

Resource availability MUST NOT determine the parse.

---

206. Dialect Selection Must Be Explicit

The parser MUST know the active dialect set from explicit configuration or source declarations.

It MUST NOT discover dialects by:

- scanning installed packages;
- inspecting filesystem contents;
- querying the network;
- inspecting hardware.

---

207. Package Discovery Is Not Parsing

A missing package produces a module/dependency diagnostic.

It does not make the source syntax ambiguous.

---

208. IDE Consistency

IDE parsing, compiler parsing, formatter parsing, LSP parsing, syntax highlighting, and diagnostics SHOULD consume the same canonical token/grammar definitions.

The IDE MUST NOT maintain a simplified grammar that can interpret source differently.

---

209. Syntax Highlighting

Syntax highlighting MAY be approximate for incomplete source.

It MUST NOT become a language authority.

---

210. Language Server Recovery

LSP error recovery may tolerate incomplete source.

That recovered interpretation MUST be explicitly treated as provisional.

It MUST NOT alter compiler semantics.

---

211. Completion

Code completion MAY use semantic information to suggest names.

It MUST NOT change the parser's interpretation of source already written.

---

212. Formatter

The formatter MUST use the same grammar contracts.

It MUST preserve AST equivalence.

---

213. Refactoring

Refactoring tools MUST operate on the AST/semantic model rather than text heuristics wherever syntax ambiguity matters.

---

214. Build Cache

Build-cache keys MUST include every input capable of changing parsing:

source
language version
grammar version
feature configuration
dialect versions
macro configuration
relevant compiler version

Hidden parser state is forbidden.

---

215. Incremental Compilation

Incremental compilation MUST invalidate syntax/AST caches whenever any grammar-affecting input changes.

---

216. Grammar Versioning

A grammar-affecting change MUST receive an appropriate language/grammar version change according to:

grammar/specification/language-version.md
grammar/compatibility/versions.md

---

217. Adding a New Operator

Before adding an operator:

1. reserve spelling;
2. compare with all existing token spellings;
3. determine lexical longest-match behavior;
4. assign precedence;
5. assign associativity;
6. determine prefix/infix/postfix roles;
7. define AST representation;
8. define semantic meaning;
9. check dialect collisions;
10. add positive tests;
11. add negative tests;
12. add round-trip tests;
13. run ambiguity validation.

---

218. Adding a New Keyword

Before adding a keyword:

1. search identifiers;
2. search existing keywords;
3. search dialect keywords;
4. determine whether contextual syntax is sufficient;
5. evaluate backward compatibility;
6. update lexer;
7. update parser;
8. update AST if necessary;
9. update semantic model;
10. update tests.

---

219. Adding a New Quantum Operation

Adding a quantum operation MUST NOT require adding a new grammar alternative if the operation fits the open operation contract.

Prefer:

operation name
+
parameters
+
targets
+
modifiers

over:

new parser keyword

for every operation.

---

220. Adding a New Hardware Target

Adding a hardware target MUST NOT require:

- new core keywords;
- new expression precedence;
- new basic types;
- new parser modes.

It should generally register:

capabilities
resources
constraints
lowering
backend

downstream.

---

221. Adding a New AI Framework

Adding an AI framework MUST NOT require new core grammar syntax unless the framework introduces genuinely new language semantics.

---

222. Adding a New HDL Backend

Adding a Verilog/SystemVerilog/VHDL/HLS backend MUST NOT alter the meaning of existing Zamani HDL source.

---

223. Ambiguity Review Checklist

Every new grammar feature MUST answer:

[ ] What tokens can begin it?
[ ] What tokens can end it?
[ ] What existing construct shares its prefix?
[ ] Can it be parsed without semantic lookup?
[ ] Does it introduce nullable alternatives?
[ ] Does it introduce left recursion?
[ ] Does it affect precedence?
[ ] Does it affect associativity?
[ ] Does it collide with keywords?
[ ] Does it collide with identifiers?
[ ] Does it collide with dialects?
[ ] Does it collide with macros?
[ ] Does it collide with operators?
[ ] Does it alter existing parse trees?
[ ] Does it require a language-version change?
[ ] Does it map to one AST structure?
[ ] Does it preserve source spans?
[ ] Does it preserve deterministic diagnostics?
[ ] Does it preserve POCO-REAF?
[ ] Does it introduce a hardware-dependent parse?
[ ] Does it introduce a fixed capacity?
[ ] Does it have positive tests?
[ ] Does it have negative tests?
[ ] Does it have boundary tests?
[ ] Does it have scalability tests?
[ ] Does it have determinism tests?
[ ] Does it have compatibility tests?

---

224. Production Acceptance Criteria

"grammar/validation/ambiguity.md" is considered integrated only when:

[ ] ambiguity taxonomy exists
[ ] lexical ambiguity rules exist
[ ] token-boundary rules exist
[ ] precedence rules exist
[ ] associativity rules exist
[ ] generic parsing rules exist
[ ] declaration/expression rules exist
[ ] type/expression rules exist
[ ] pattern rules exist
[ ] macro rules exist
[ ] dialect rules exist
[ ] quantum rules exist
[ ] HDL rules exist
[ ] hardware rules exist
[ ] resource rules exist
[ ] distributed rules exist
[ ] AI rules exist
[ ] interoperability rules exist
[ ] AST contract exists
[ ] semantic-boundary contract exists
[ ] IR boundary exists
[ ] parser/lexer conformance exists
[ ] deterministic diagnostics exist
[ ] scalability policy exists
[ ] hard-coding policy exists
[ ] Rust safety requirement exists
[ ] ANTLR composition is covered
[ ] existing ambiguity-rules.md is reconciled
[ ] positive tests exist
[ ] negative tests exist
[ ] boundary tests exist
[ ] scalability tests exist
[ ] determinism tests exist
[ ] compatibility tests exist

---

225. Required Validation Matrix

The production validator SHOULD produce a matrix like:

Layer| Question| Failure
Lexer| Can two tokens recognize the same source?| "AMBIGUOUS_TOKEN"
Lexer| Can tokenization differ by whitespace?| lexical failure
Parser| Can two parse trees result?| "AMBIGUOUS_SYNTAX"
Parser| Is precedence inconsistent?| "AMBIGUOUS_PRECEDENCE"
Parser| Is associativity inconsistent?| "AMBIGUOUS_ASSOCIATIVITY"
Parser| Is an alternative shadowed?| grammar failure
Parser| Is there a nullable cycle?| grammar failure
Parser| Is left recursion unsupported?| grammar failure
AST| Can two syntax forms collapse incorrectly?| AST conformance failure
Semantic| Are several meanings possible?| semantic diagnostic
Overload| Are several overloads equally valid?| "AMBIGUOUS_OVERLOAD"
Dialect| Do dialects collide?| "DIALECT_SYNTAX_CONFLICT"
Resource| Is target capacity insufficient?| resource diagnostic
Capability| Is a capability unavailable?| capability diagnostic
Backend| Can target realize semantics?| target diagnostic

---

226. What Counts as a Real Ambiguity

A construct is a real grammar ambiguity when the same configured parser can derive two structurally different syntax trees from the same source.

It is NOT a grammar ambiguity merely because:

- two functions have the same name;
- two types have compatible meanings;
- two hardware targets can execute it;
- multiple quantum implementations exist;
- several overloads match;
- multiple resources satisfy a requirement.

Those are semantic/resource/target concerns.

---

227. What Must Never Be Used to "Resolve" Ambiguity

The compiler MUST NOT solve grammar ambiguity by:

"pick the first declaration"
"pick the first hash-map entry"
"pick the first backend"
"pick the fastest device"
"pick the current QPU"
"pick the current GPU"
"pick the most capable target"
"pick whatever compiles"
"pick whatever runtime supports"
"pick based on optimization"
"pick based on machine size"

Such behavior violates deterministic semantics and POCO-REAF.

---

228. Canonical Rule

The production rule is:

«Syntax chooses structure.
Semantics chooses meaning.
Resource analysis chooses feasibility.
Compilation chooses realization.
Runtime chooses execution.»

No stage may silently assume the responsibilities of another.

---

229. Final Architecture

The complete ambiguity-safe architecture is:

                 Zamani Source
                       |
                       v
              Canonical Lexer
                       |
                       v
              Canonical Parser
                       |
             +---------+---------+
             |                   |
             v                   v
       Syntax validation     diagnostics
             |
             v
        Domain-neutral AST
             |
             v
       Semantic analysis
             |
      +------+------+------+------+------+
      |      |      |      |      |      |
    types effects resources quantum hardware ...
      |      |      |      |      |
      +------+------+------+------+------+
             |
             v
      Canonical semantic model
             |
             +-------------------+
             |                   |
             v                   v
        classical IR        quantum::ir
             |                   |
             +---------+---------+
                       |
                       v
                  optimization
                       |
              +--------+--------+
              |        |        |
           routing scheduling  QEC
              |        |        |
              +--------+--------+
                       |
                      ZQN
                       |
                      HAL
                       |
             target realization
                       |
       +-------+-------+-------+-------+
       |       |       |       |       |
      CPU     GPU     FPGA    QPU    future

---

230. Final Non-Negotiable Invariants

Zamani ambiguity handling MUST preserve all of the following:

1. Zamani remains one language.

2. Classical, quantum, HDL, AI, distributed, networking, security, data, and future domains share the same language foundation.

3. "grammar/Zamani.g4" remains the canonical root grammar.

4. "grammar/antlr/ZamaniParser.g4" remains the parser composition authority.

5. "grammar/antlr/ZamaniLexer.g4" remains the lexer composition authority.

6. Domain grammars do not become competing root grammars.

7. The existing "grammar/validation/ambiguity-rules.md" is not unnecessarily renamed or deleted.

8. "grammar/validation/ambiguity.md" provides the complete production ambiguity contract.

9. Lexical ambiguity is solved lexically.

10. Syntactic ambiguity is solved syntactically.

11. Semantic ambiguity is solved semantically.

12. Overload ambiguity is solved by deterministic overload resolution.

13. Resource feasibility is not syntax.

14. Hardware availability is not syntax.

15. Target selection is not syntax.

16. Quantum operation names remain open-ended.

17. A finite gate enumeration is not the universal quantum model.

18. Physical qubit allocation is downstream of portable source semantics.

19. QEC remains downstream.

20. ZQN remains downstream.

21. Routing remains downstream.

22. Scheduling remains downstream.

23. HAL remains downstream.

24. Hardware topology never determines source parsing.

25. No universal hardware capacity is encoded into grammar.

26. No universal "MAX_*" capacity constants define language meaning.

27. Program literals remain valid program semantics.

28. Generic nesting is not artificially bounded by grammar.

29. Expression precedence is canonical.

30. Operator associativity is canonical.

31. Token spellings are canonical.

32. Keyword ownership is canonical.

33. Contextual keywords are explicit.

34. Macro expansion is deterministic.

35. Macro hygiene is mandatory.

36. Dialect conflicts fail deterministically.

37. Vendor extensions do not silently steal core syntax.

38. Parser behavior is independent of hardware.

39. Parser behavior is independent of filesystem state.

40. Parser behavior is independent of network state.

41. Parser behavior is independent of wall-clock time.

42. Parser behavior is independent of randomness.

43. Parser behavior is independent of environment variables.

44. Parser behavior is independent of runtime state.

45. AST construction is deterministic.

46. Diagnostics are deterministic.

47. Formatting preserves parse equivalence.

48. Incremental parsing preserves full-parse semantics.

49. Parallel parsing preserves deterministic results.

50. Rust implementation remains Rust 1.97/1.97.1 compatible.

51. Rust implementation remains Rust 2021 compatible.

52. Production implementation uses no "unsafe".

53. Resource-exhaustion protection is distinguished from language restrictions.

54. Future domains enter through explicit extension contracts.

55. Every ambiguity-sensitive feature has positive, negative, boundary, scalability, compatibility, and determinism tests.

56. Every accepted syntax construct has a known AST contract.

57. Every AST construct has a known semantic contract.

58. Every semantic construct has a known canonical IR boundary.

59. No syntax-only feature is declared production-complete.

60. POCO-REAF remains an architectural invariant.

---

231. Production Definition of Done

This file and the associated validation system are complete only when the following statement is true:

For every supported Zamani language version and explicitly selected
dialect/feature configuration, every source program has exactly one
deterministic syntactic interpretation or deterministic diagnostics,
independent of target hardware, resource availability, runtime state,
filesystem state, network state, wall-clock time, randomness, and
environment state.

All unresolved meaning is represented explicitly in the AST and resolved
by the appropriate semantic layer.

No ambiguity is hidden behind implementation ordering, backend selection,
hardware availability, hash-map iteration, or runtime behavior.

The language remains scalable from the smallest computation to arbitrarily
large computations subject only to program semantics, implementation
resources, declared policies, and actual target resources.

The frontend remains safe Rust with no unsafe requirement.

Quantum semantics terminate at the canonical quantum::ir boundary.

Classical, quantum, HDL, hardware, AI, distributed, and future domains
remain composable parts of one language.

Therefore the ambiguity system protects:

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.

---

232. Integration Summary

After adding this file, the intended repository relationship is:

grammar/
│
├── DESIGN.md
│     └── architecture/invariants
│
├── Zamani.g4
│     └── canonical root composition
│
├── Zamani-Grammar.md
│     └── historical/extended feature source
│
├── grammar.md
│     └── implementation conformance
│
├── antlr/
│     ├── ZamaniLexer.g4
│     └── ZamaniParser.g4
│
├── lexer/
│     └── lexical authority
│
├── core/
│     └── universal syntax
│
├── expressions/
│     └── expression + precedence authority
│
├── types/
│     └── type syntax
│
├── quantum/
│     └── quantum syntax
│
├── hdl/
│     └── hardware-description syntax
│
├── hardware/
│     └── hardware intent
│
├── resources/
│     └── resource/capability intent
│
├── compatibility/
│     └── evolution/versioning
│
├── validation/
│     ├── ambiguity.md
│     ├── ambiguity-rules.md
│     ├── grammar-validator.md
│     ├── grammar-validation.md
│     ├── hardcoding-audit.md
│     ├── scalability-rules.md
│     └── semantic-boundaries.md
│
└── tests/
      └── conformance

The important integration rule is that "ambiguity.md" does not become another grammar. It is the cross-cutting contract that tells every existing grammar file how to remain deterministic, composable, scalable, and target-independent.