Zamani Lexer — Delimiter Specification

Path: "grammar/lexer/delimiters.md"
Language: Zamani
Status: Normative / Production Architecture
Specification Version: 1.0
Implementation Baseline: Rust 1.97 / Rust 1.97.1
Safety: Safe Rust only; Rust "unsafe" is prohibited
Primary Goal: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)

---

0. File Contract

Purpose

This file defines the canonical lexical contract for Zamani delimiters and punctuation.

A delimiter is a lexical token whose primary purpose is to establish boundaries, grouping, separation, structure, or syntactic containment.

This file defines:

- delimiter spellings;
- canonical token identities;
- delimiter categories;
- longest-match interactions with operators;
- delimiter/identifier interactions;
- delimiter/literal interactions;
- delimiter/comment interactions;
- delimiter/source-span requirements;
- parser integration;
- AST integration;
- diagnostics;
- compatibility;
- scalability;
- Unicode policy;
- tooling requirements;
- conformance tests;
- migration requirements for the existing lexer.

This file does not define:

- semantic meaning of declarations;
- type checking;
- quantum semantics;
- QEC;
- ZQN;
- routing;
- scheduling;
- HAL;
- hardware topology;
- resource allocation;
- compiler optimization;
- runtime behavior;
- target-specific syntax.

---

1. Architectural Authority

The lexical authority chain is:

grammar/DESIGN.md
        |
        v
grammar/spec/lexical.md
        |
        +-----------------------------+
        |                             |
        v                             v
grammar/lexer/delimiters.md   other lexer contracts
        |                             |
        +-------------+---------------+
                      |
                      v
          grammar/antlr/ZamaniLexer.g4
                      |
                      v
             grammar/antlr/Core.g4
                      |
                      v
                frontend AST
                      |
                      v
             semantic analysis
                      |
                      v
               canonical IR

The canonical executable ANTLR lexer currently identified by the repository is:

grammar/antlr/ZamaniLexer.g4

The parser counterpart is:

grammar/antlr/Core.g4

The normative lexical specification is:

grammar/spec/lexical.md

The root grammar/composition layer remains:

grammar/Zamani.g4

This file does not become another lexer.

It is the normative delimiter contract consumed by those implementations.

---

2. Ownership

2.1 This file owns

This file owns:

- delimiter classification;
- punctuation classification;
- delimiter spelling;
- delimiter token identity;
- delimiter precedence against identifier/literal recognition;
- delimiter interaction with operators;
- delimiter source-span requirements;
- delimiter lexical diagnostics;
- delimiter compatibility rules;
- delimiter conformance requirements.

2.2 This file does not own

This file does not own:

- operator precedence;
- expression semantics;
- AST node definitions;
- type semantics;
- name resolution;
- module resolution;
- resource semantics;
- quantum semantics;
- HDL semantics;
- hardware semantics;
- runtime semantics.

Operators remain owned by:

grammar/lexer/operators.md

Identifiers remain owned by:

grammar/lexer/identifiers.md

Keywords remain owned by:

grammar/lexer/keywords.md

Unicode rules remain owned by:

grammar/lexer/unicode.md

The complete lexical authority remains:

grammar/spec/lexical.md

---

3. Production Delimiter Set

The canonical Zamani delimiter/punctuation vocabulary is:

Spelling| Canonical token| Category
"("| "LPAREN"| grouping
")"| "RPAREN"| grouping
"{"| "LBRACE"| block/record body
"}"| "RBRACE"| block/record body
"["| "LBRACKET"| indexing/collection/attribute
"]"| "RBRACKET"| indexing/collection/attribute
","| "COMMA"| separator
"."| "DOT"| member/path/access punctuation
":"| "COLON"| type/field/constraint separator
";"| "SEMICOLON"| statement/declaration terminator
"@"| "AT"| attribute/annotation introducer
"#"| "HASH"| attribute/directive introducer
"_"| "UNDERSCORE"| standalone wildcard punctuation

These are the canonical delimiter identities.

No other file may introduce another token identity for the same spelling in the same lexical mode.

---

4. Delimiters Are Not Operators

The following are delimiters:

(
)
{
}
[
]
,
.
:
;
@
#
_

The following are operators:

+
-
*
/
%
=
!
~
&
|
^
?
<
>
==
!=
<=
>=
&&
||
<<
>>
+=
-=
*=
/=
..
..=
::
->
=>

The distinction is architectural.

A delimiter establishes syntactic structure.

An operator represents an operation or operator expression.

For example:

foo(a, b)

uses:

LPAREN
COMMA
RPAREN

while:

a + b

uses:

PLUS

A delimiter must not acquire mathematical, quantum, hardware, or runtime semantics merely because it appears in a semantic construct.

---

5. Canonical Delimiter Rules

5.1 Parentheses

(  -> LPAREN
)  -> RPAREN

Parentheses are used for:

- function calls;
- parameter lists;
- argument lists;
- grouping;
- tuple syntax where applicable;
- conditional/grouped expressions;
- generic parser structures where explicitly specified;
- quantum operation arguments;
- HDL constructs where explicitly specified.

Example:

fn add(a, b) {
    return a + b;
}

Lexically:

FN
IDENTIFIER
LPAREN
IDENTIFIER
COMMA
IDENTIFIER
RPAREN
LBRACE
...
RBRACE

The delimiter layer does not determine whether a parenthesized sequence is a function call, tuple, expression, parameter list, or another construct.

That belongs to the parser.

---

6. Curly Braces

{  -> LBRACE
}  -> RBRACE

Curly braces delimit structures such as:

- function bodies;
- blocks;
- module bodies;
- class bodies;
- struct bodies;
- record bodies;
- enum bodies;
- trait bodies;
- implementation bodies;
- effect bodies;
- contract bodies;
- quantum circuit bodies;
- HDL bodies;
- resource declarations;
- dialect bodies;
- macro bodies.

Example:

fn compute() {
    let x = 42;
}

The lexer only emits:

LBRACE
RBRACE

It does not decide what the body means.

---

7. Square Brackets

[  -> LBRACKET
]  -> RBRACKET

Square brackets support syntax including:

- indexing;
- array literals;
- collection literals;
- attributes;
- annotations;
- ranges where specified;
- generic domain-specific constructs;
- resource expressions;
- quantum indexing;
- tensor indexing;
- HDL array declarations.

Examples:

values[0]

values[i]

[1, 2, 3]

#[inline]

#[requires(capability("quantum.measurement"))]

The delimiter layer does not decide whether "[]" represents:

- an index;
- an array;
- an attribute;
- a range;
- a tensor slice;
- another semantic construct.

That is parser/semantic responsibility.

---

8. Comma

, -> COMMA

The comma is a separator.

It may separate:

- function parameters;
- function arguments;
- tuple elements;
- array elements;
- generic parameters;
- type arguments;
- imports;
- exports;
- fields;
- enum variants;
- quantum targets;
- operation parameters;
- resource requirements;
- HDL ports;
- distributed resources;
- configuration values.

Example:

fn f(a, b, c) {}

The comma does not imply a maximum number of elements.

The grammar must not introduce:

MAX_PARAMETERS
MAX_ARGUMENTS
MAX_ELEMENTS
MAX_TARGETS

or equivalent universal limits.

---

9. Dot

. -> DOT

The dot is punctuation when used as member/path/access syntax.

Examples:

module.member
object.field
resource.capacity
device.capability

However, "." may also participate in lexical constructs such as floating-point literals.

Therefore:

1.5

must not tokenize as:

INTEGER
DOT
INTEGER

when the complete floating-point token is valid.

The numeric-literal specification owns this interaction.

The lexer must apply longest-valid-token recognition.

---

10. Dot and Range Operators

The following tokens must be distinguished:

.    -> DOT
..   -> RANGE
..=  -> RANGE_INCLUSIVE

The longest valid operator wins.

Examples:

.
..
..=

must produce:

DOT
RANGE
RANGE_INCLUSIVE

respectively.

The delimiter specification does not own range semantics.

Range semantics belong to:

grammar/expressions/

and the semantic/type system.

---

11. Colon

: -> COLON

The colon is a structural separator.

Examples include:

x: T
field: Type
parameter: Type
name: value

It may participate in:

- type annotations;
- field declarations;
- parameter declarations;
- generic constraints;
- resource declarations;
- capability declarations;
- match patterns;
- labels if such syntax is formally adopted;
- HDL port/type declarations;
- semantic constraints.

The colon must remain one canonical lexical token.

---

12. Double Colon

"::" is not a delimiter.

It is an operator/path separator token owned by the operator specification:

:: -> DOUBLE_COLON

Therefore:

:

is:

COLON

while:

::

is:

DOUBLE_COLON

Example:

module::type::function

must tokenize as:

IDENTIFIER
DOUBLE_COLON
IDENTIFIER
DOUBLE_COLON
IDENTIFIER

The parser later determines whether the sequence represents a module path, namespace, qualified name, or another construct.

---

13. Semicolon

; -> SEMICOLON

"SEMICOLON" is the canonical token identity.

This resolves an existing integration inconsistency: the canonical lexer exposes "SEMICOLON", while parts of "Core.g4" currently refer to "SEMI". The repository must standardize on one identity rather than maintaining two lexical tokens for the same spelling.

The preferred canonical identity is:

SEMICOLON

Parser grammar may use a parser-level alias only if there is a compelling compatibility reason, but a second lexer token must not be created.

Examples:

let x = 1;
return x;

The semicolon does not necessarily imply a mandatory terminator everywhere.

Optional semicolon policy belongs to the syntax specification.

---

14. Statement-Termination Policy

Delimiter recognition and statement termination are separate concerns.

The lexer recognizes:

;

as:

SEMICOLON

The parser decides whether a semicolon is:

- required;
- optional;
- permitted;
- redundant;
- invalid.

The lexer must never infer statement boundaries from:

- CPU architecture;
- operating system;
- indentation;
- machine size;
- runtime;
- target hardware.

If future Zamani adopts automatic semicolon insertion or newline-sensitive syntax, that must be an explicit lexical/syntax feature and must not alter the meaning of existing delimiter tokens.

---

15. At Sign

@ -> AT

"AT" is the canonical standalone attribute/annotation introducer.

This is important because the current lexer contains a "NANO_ANNOTATION" token that can consume:

@identifier

as one token.

That creates a conflict with the parser's existing attribute structure:

AT identifier

The production architecture must therefore choose one model.

The canonical Zamani model is:

AT
IDENTIFIER

rather than:

NANO_ANNOTATION

for generic attribute syntax.

Therefore:

@atom

should lex as:

AT
IDENTIFIER("atom")

unless a future explicitly standardized lexical form requires a dedicated token.

This keeps annotations compositional and allows:

@atom
@molecule
@custom_domain
@vendor_extension

without adding a new lexer token for every annotation family.

---

16. Hash

# -> HASH

"HASH" must be added to the canonical lexer vocabulary because the normative syntax specification defines attribute syntax using:

#[...]

For example:

#[inline]
#[deprecated]
#[experimental]
#[target(...)]
#[requires(...)]
#[capability(...)]

The canonical lexical sequence is:

HASH
LBRACKET
...
RBRACKET

This is preferable to making:

#[inline]

one monolithic lexer token.

The parser should own the structure after lexicalization.

---

17. Attribute Syntax

The lexical architecture supports:

#[attribute]

as:

HASH
LBRACKET
IDENTIFIER
RBRACKET

and:

#[attribute(argument)]

as:

HASH
LBRACKET
IDENTIFIER
LPAREN
...
RPAREN
RBRACKET

This is compatible with the normative syntax specification's attribute model.

Attribute semantics remain outside this file.

---

18. Attribute Extensibility

The delimiter layer must not enumerate attributes.

These must remain legal lexical names:

inline
deprecated
experimental
target
requires
capability
portable
quantum
hdl
hardware
resource
vendor_extension
future_feature

provided the corresponding parser/semantic rules allow them.

The lexer must not require:

INLINE_ATTRIBUTE
TARGET_ATTRIBUTE
QUANTUM_ATTRIBUTE
GPU_ATTRIBUTE
QPU_ATTRIBUTE

for every attribute.

This is essential for dialect extensibility and POCO-REAF.

---

19. Underscore

_ -> UNDERSCORE

The standalone underscore has a distinct syntactic role as a wildcard.

For example:

match value {
    _ => fallback
}

The canonical parser already requires an "UNDERSCORE" token for wildcard patterns.

Therefore the lexer must provide:

UNDERSCORE

for a standalone "_".

---

20. Underscore and Identifiers

Zamani identifiers may begin with "_".

Therefore these must remain possible identifiers:

_
_value
__value
_value2

The critical lexical rule is:

_       -> UNDERSCORE
_value  -> IDENTIFIER

The lexer must use longest-valid-token matching.

Consequently:

_value

must not become:

UNDERSCORE
IDENTIFIER("value")

It must become one:

IDENTIFIER("_value")

while a standalone:

_

becomes:

UNDERSCORE

This resolves the current "UNDERSCORE"/"IDENTIFIER" ambiguity without eliminating Unicode-aware identifiers.

---

21. Delimiter Precedence

Delimiter precedence must be determined by the following principles.

21.1 Longest valid token

A valid longer lexical token wins over its valid shorter prefix.

Examples:

..=  -> RANGE_INCLUSIVE
..   -> RANGE
.    -> DOT

::   -> DOUBLE_COLON
:    -> COLON

->   -> ARROW
-    -> MINUS

=>   -> FAT_ARROW
=    -> ASSIGN

21.2 Complete lexical forms

A delimiter must not steal characters belonging to a complete literal.

For example:

1.5

must remain one valid floating-point literal.

Likewise:

0..10

must be recognized according to numeric/range lexical rules rather than arbitrarily splitting at the first dot.

---

22. Delimiter Precedence Table

Input| Canonical tokenization
"("| "LPAREN"
")"| "RPAREN"
"{"| "LBRACE"
"}"| "RBRACE"
"["| "LBRACKET"
"]"| "RBRACKET"
","| "COMMA"
"."| "DOT"
":"| "COLON"
";"| "SEMICOLON"
"@"| "AT"
"#"| "HASH"
"_"| "UNDERSCORE"
"_name"| "IDENTIFIER"
".."| "RANGE"
"..="| "RANGE_INCLUSIVE"
"::"| "DOUBLE_COLON"
"->"| "ARROW"
"=>"| "FAT_ARROW"

---

23. Delimiters and Quantum Syntax

Delimiters must remain generic.

For example:

apply H to q[0];

contains:

APPLY
IDENTIFIER(H)
IDENTIFIER(q)
LBRACKET
INTEGER(0)
RBRACKET
SEMICOLON

The delimiter layer does not know:

- what "q" means;
- whether "q" is quantum;
- whether index "0" is physical;
- how many qubits exist;
- what hardware is used;
- what gate "H" means.

This preserves the canonical quantum architecture:

source
  |
lexer
  |
parser
  |
domain-neutral AST
  |
semantic analysis
  |
quantum::ir
  |
optimization
  |
routing
  |
scheduling
  |
QEC / resilience / ZQN
  |
HAL
  |
target realization

No delimiter may encode a physical quantum limit.

---

24. Delimiters and Classical Computing

The same delimiter tokens are used for classical programs:

fn compute(values: Tensor) {
    let result = values[0];
    return result;
}

The lexical layer does not distinguish classical indexing from quantum indexing.

That distinction belongs to semantic/type analysis.

---

25. Delimiters and HDL

The same delimiter system supports hardware-description syntax:

module Device {
    input clock: Clock;
    output result: Signal;
}

The lexer recognizes delimiters only.

It does not decide whether:

clock
Signal
Device

represent hardware.

That is the responsibility of the HDL semantic layer.

---

26. Delimiters and Distributed Computing

The delimiter system must support arbitrary scalable collections:

nodes[node_id]
channels[channel_id]
services[service_id]

No delimiter rule may impose a fixed number of:

- nodes;
- services;
- channels;
- processes;
- actors;
- endpoints.

---

27. Delimiters and AI/Data/Tensor Computing

The same syntax supports:

tensor[i, j]
tensor[i, j, k]
tensor[a, b, c, d]

The lexer must not impose a fixed tensor rank.

Similarly:

dataset[partition]
model[layer]
pipeline[stage]

remain ordinary delimiter-based syntax.

Semantic systems determine their meaning.

---

28. Delimiters and Resource Expressions

Resource syntax may use the same delimiters:

requires(
    capability("quantum.measurement"),
    memory(required_memory),
    communication(required_bandwidth)
);

The delimiter layer does not determine resource availability.

It merely recognizes:

LPAREN
COMMA
RPAREN
SEMICOLON

---

29. Delimiters and Module Paths

A module path may use:

DOUBLE_COLON

rather than multiple delimiter tokens.

Example:

quantum::ir::Operation

Tokenization:

IDENTIFIER
DOUBLE_COLON
IDENTIFIER
DOUBLE_COLON
IDENTIFIER

Name resolution is downstream.

---

30. Delimiters and Generic Syntax

Angle brackets are not delimiters.

They are operator/token vocabulary owned by the operator specification and type grammar.

The lexer must distinguish:

<
>
<<
>>
<=
>=

where supported.

The delimiter specification must not introduce:

LT
GT

as delimiter tokens.

If the canonical lexer requires "LT" and "GT" parser tokens, they belong to the operator contract.

The current repository must ensure that single "<" and ">" are represented consistently with "Core.g4" generic/type productions.

---

31. Delimiters and Comments

Delimiter-looking characters inside comments are not delimiter tokens.

Example:

// { [ ( @ # ) ] }

must produce comment trivia only.

Likewise:

/*
{
[
(
#
@
]
)
}
*/

must not emit delimiter tokens to the parser.

Comment recognition is owned by:

grammar/lexer/comments.md

but delimiter conformance tests must include comment interactions.

---

32. Delimiters and Strings

Delimiter characters inside strings are string contents.

Example:

"({[]},.:;@#_)"

must produce one string literal token.

It must not produce delimiter tokens.

The string lexer therefore has lexical precedence over ordinary delimiter recognition while inside a string.

---

33. Delimiters and Character Literals

Likewise:

'('
')'
'{'
'}'
'['
']'
','
'.'
':'
';'
'@'
'#'
'_'

must be character literals where valid character syntax exists.

The delimiter characters must not escape their literal context.

---

34. Delimiters and Quantum State Notation

Quantum state syntax may contain punctuation such as:

|0⟩
|1⟩

The vertical bar:

|

is an operator token, not a delimiter.

Therefore quantum-state lexical handling must remain coordinated with:

grammar/lexer/operators.md
grammar/lexer/quantum-literals.md

The delimiter specification does not claim ownership of "|".

---

35. Delimiters and Unicode

The canonical ASCII delimiter set is stable:

( ) { } [ ] , . : ; @ # _

Unicode punctuation must not silently become an equivalent delimiter.

For example:

（
）
［
］
｛
｝

must not automatically become:

LPAREN
RPAREN
LBRACKET
RBRACKET
LBRACE
RBRACE

unless a future Zamani language version explicitly adopts those spellings.

This prevents Unicode normalization from silently changing syntax.

---

36. Confusable Delimiters

The lexer must distinguish:

(

from visually similar Unicode characters.

The same applies to:

[
]
{
}
,
.
:
;
@
#
_

A confusable character must not silently become a canonical delimiter.

Tooling may issue a security warning where appropriate.

The lexer must preserve the original source spelling.

---

37. Source Spans

Every delimiter token must retain its exact source span.

For a single-character delimiter:

(

the token span must cover exactly that source character.

For UTF-8 source, source spans must follow the repository's canonical source-span representation and must not assume that:

byte offset == Unicode scalar index

unless the source-span contract explicitly defines that representation.

The delimiter layer must not invent a second source-span type.

Source spans are integrated with:

grammar/spec/source-spans.md

and the existing frontend source-span model.

---

38. Error Recovery

A malformed delimiter sequence must produce deterministic lexical or syntactic diagnostics.

Examples:

(
{
[

at end of input may be syntactically incomplete.

The lexer should emit the delimiter token normally.

The parser should diagnose unmatched structure.

The lexer must not invent closing delimiters.

For example, it must never silently convert:

fn f() {

into:

fn f() {}

unless an explicitly specified recovery mode is being used.

Recovery must never alter the canonical token stream used for successful compilation.

---

39. Invalid Delimiter Characters

Characters that resemble delimiters but are not part of the canonical lexical set must not be silently ignored.

Examples include unsupported punctuation or malformed Unicode punctuation.

The lexer must produce a deterministic lexical diagnostic containing:

- diagnostic identifier;
- source span;
- offending source representation;
- language version;
- recovery behavior if recovery is possible.

Diagnostic identifiers are owned by:

grammar/lexer/diagnostics.md

This file defines the required diagnostic conditions but does not create a competing diagnostic registry.

---

40. No Delimiter-Based Hardware Limits

Delimiter syntax must never contain machine limits such as:

q[0..31]

as a universal grammar restriction.

This is valid source-level data if the programmer explicitly writes it.

What is prohibited is a language rule such as:

array indexes may only range from 0 to 31

or:

quantum register syntax supports only 32 elements

unless such a limitation is explicitly part of a separate target dialect.

The core language remains unbounded subject to available resources.

---

41. Dialect Extensions

A dialect may introduce additional punctuation only through the formal dialect system.

A dialect must declare:

- dialect name;
- version;
- owner;
- additional delimiters;
- additional operators;
- lexical mode if required;
- parser integration;
- AST mapping;
- semantic mapping;
- compatibility behavior;
- diagnostics;
- tests.

A dialect must not redefine a core delimiter with a different meaning in the same lexical context.

A dialect may introduce an alternate spelling only through an explicit compatibility/extension contract.

---

42. Macro Interaction

Macros consume lexical structures but must not silently redefine core delimiter identities.

A macro may receive:

(
)
{
}
[
]
,
.
:
;
@
#
_

as ordinary tokens.

Macro expansion must preserve delimiter balance unless the macro system explicitly defines token-tree construction.

Malformed macro output must be diagnosed before semantic analysis.

Macros must not bypass:

- parser validation;
- AST construction;
- semantic validation;
- resource validation;
- safety validation.

---

43. Metaprogramming Interaction

Quotation/unquotation may represent delimiter tokens as syntax objects.

For example, a syntax tree may contain:

LPAREN
IDENTIFIER
COMMA
IDENTIFIER
RPAREN

The delimiter identity remains lexical/syntactic.

Metaprogramming must not reinterpret:

LPAREN

as a machine operation or runtime object.

---

44. Formatting and Pretty Printing

The formatter must use canonical delimiter spellings.

Canonical output:

(
)
{
}
[
]
,
.
:
;
@
#
_

The formatter may choose whitespace around delimiters according to formatting rules.

It must not substitute Unicode confusable punctuation.

Formatting must preserve semantic structure.

---

45. Syntax Highlighting

Syntax highlighters must classify delimiters as punctuation/delimiter tokens.

They must not classify:

(
)
{
}
[
]
,
.
:
;
@
#
_

as keywords.

The highlighter should be generated or validated against the canonical token registry.

---

46. LSP Integration

The language server must use the same delimiter token identities as the compiler.

Delimiter-aware tooling includes:

- matching brackets;
- structural selection;
- folding;
- syntax highlighting;
- diagnostics;
- formatting;
- code completion;
- refactoring.

Examples:

(
)

must match.

{
}

must match.

[
]

must match.

Unmatched delimiters must be highlighted without changing lexical semantics.

---

47. Parser Integration

"grammar/antlr/Core.g4" must consume the canonical delimiter tokens.

The canonical names are:

LPAREN
RPAREN

LBRACE
RBRACE

LBRACKET
RBRACKET

COMMA
DOT
COLON
SEMICOLON

AT
HASH

UNDERSCORE

The following integration correction is mandatory:

SEMI

must not exist as a second lexer token for ";".

The parser should consume:

SEMICOLON

or use an explicitly documented parser-level alias.

Likewise, "HASH" must be available for the normative:

#[...]

attribute syntax.

---

48. Parser Productions Supported by Delimiters

The delimiter contract must support at least:

argumentList
parameterList
genericParameterList
typeArgumentList
arrayLiteral
tupleLiteral
indexExpression
memberExpression
block
moduleBody
structBody
recordBody
enumBody
traitBody
implBody
classBody
interfaceBody
effectBody
contractBody
attribute
attributeArguments
matchBody
matchArm
quantumTargetList
quantumOperationArguments
resourceList
capabilityList
HDL port lists
HDL signal lists
distributed resource lists
AI/data collection expressions

The number of elements in these lists is not language-limited.

---

49. AST Contract

Delimiters normally do not require dedicated AST nodes.

For example:

(
)

around an expression may disappear from the semantic AST after establishing grouping.

Likewise:

,

usually establishes list structure rather than becoming a semantic node.

However, source spans and source-preservation information must remain available where required by:

- diagnostics;
- formatting;
- IDE tooling;
- macro expansion;
- provenance;
- source-to-source transformation.

The AST must not depend on target hardware.

---

50. Semantic Contract

The delimiter layer contributes structural information only.

Examples:

[]

may become:

- array;
- index;
- slice;
- tensor dimension;
- quantum register access;
- HDL vector access.

The delimiter itself does not determine which.

Semantic analysis determines the meaning from:

- surrounding syntax;
- types;
- declarations;
- namespaces;
- domain;
- capabilities;
- context.

---

51. IR Integration

Delimiters normally disappear after parsing.

They contribute structure to the domain-neutral AST.

Example:

q[index]

becomes an AST indexing expression.

Later semantic analysis may lower this to:

Index(base=q, index=index)

and eventually to an appropriate canonical IR representation.

The delimiter layer must never directly generate:

- physical qubit IDs;
- CPU registers;
- GPU memory addresses;
- FPGA ports;
- QPU topology edges;
- hardware routing decisions.

---

52. Quantum IR Boundary

The canonical quantum semantic boundary remains:

quantum::ir

Delimiter syntax must therefore follow:

delimiter
    ↓
parser structure
    ↓
domain-neutral AST
    ↓
semantic quantum representation
    ↓
quantum::ir

There must not be:

delimiter
    ↓
lexer-specific quantum IR

or:

delimiter
    ↓
frontend-specific QuantumGate IR

---

53. Classical / Quantum / HDL Unification

The same delimiter vocabulary must serve all computational domains.

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
embedded
HPC
future domains

This is essential to maintaining one language instead of creating multiple incompatible lexical dialects.

---

54. POCO-REAF Contract

Delimiter syntax must remain target-independent.

A program may contain:

for item in data {
    ...
}

without specifying:

CPU count
GPU count
thread count
node count
accelerator count

Likewise:

q[i]

does not imply:

physical_qubit_i

and:

buffer[i]

does not imply:

GPU_memory_bank_i

Target realization occurs after semantic analysis.

---

55. Scalability Contract

There is no language-level upper bound on:

- nesting depth;
- list length;
- number of parameters;
- number of arguments;
- number of fields;
- number of array dimensions;
- number of tensor dimensions;
- number of quantum targets;
- number of HDL ports;
- number of distributed nodes;
- number of resources;
- number of modules;
- number of declarations;
- number of syntax objects.

Implementations may have configurable resource budgets.

Those budgets must be reported as resource failures, not syntax restrictions.

For example:

parser-memory-budget-exceeded

is valid implementation behavior.

This is not equivalent to:

maximum 1024 nested braces

being a language rule.

---

56. Tiny-to-Large Examples

Tiny

()

{}

[]

Ordinary

fn f(a, b) {
    return a + b;
}

Quantum

apply H to q[0];

Hybrid

let result = measure(q[0]);
if result {
    apply X to q[1];
}

HDL

module Device {
    input clock: Clock;
    output result: Signal;
}

Large symbolic structure

pipeline[
    stage_a,
    stage_b,
    stage_c,
    ...
]

The grammar does not impose a fixed maximum number of elements.

---

57. Negative Lexical Tests

The following must be covered by conformance tests.

Invalid/unrecognized delimiter

unsupported_punctuation

when it is not a valid Zamani token.

Unterminated grouping

(

The lexer emits "LPAREN"; the parser reports structural incompleteness.

Unterminated attribute

#[inline

The lexer must not fabricate "RBRACKET".

Wrong delimiter

foo]

The lexer emits "RBRACKET"; the parser diagnoses the unmatched structure.

Confusable punctuation

Unicode lookalikes must not silently become ASCII delimiters.

---

58. Boundary Tests

At minimum:

()
{}
[]
,
.
:
;
@
#
_

must each produce exactly one canonical delimiter token.

The following must remain distinct:

.
..
..=

:
::

-
->

=
=>

_
_value

The following must remain distinct:

@
@name
#
#[name]

---

59. Context Tests

The following must be tested:

fn f() {}

fn f(a, b) {}

let x: T = value;

x[i];

x[i, j];

module::member

#[inline]

#[requires(capability("quantum.measurement"))]

match x {
    _ => fallback
}

---

60. Identifier Collision Tests

These are mandatory:

_
_value
__value
value_
value__x

Expected:

_          -> UNDERSCORE
_value     -> IDENTIFIER
__value    -> IDENTIFIER
value_     -> IDENTIFIER
value__x   -> IDENTIFIER

The standalone wildcard must not destroy ordinary underscore-prefixed identifiers.

---

61. Comment Tests

These must not emit delimiter tokens from their contents:

// ( ) { } [ ] , . : ; @ # _

and:

/*
(
)
{
}
[
]
,
.
:
;
@
#
_
*/

Documentation comments must likewise preserve their contents through the documented hidden/trivia mechanism without exposing internal delimiters as parser tokens.

---

62. String Tests

"(){}[],.:;@#_"

must produce one string token.

Likewise:

"#[inline]"

must remain a string.

---

63. Character Tests

Where character literals are supported:

'('
')'
'{'
'}'
'['
']'
','
'.'
':'
';'
'@'
'#'
'_'

must remain character literals.

---

64. Numeric Interaction Tests

These must be tested together with the numeric-literal specification:

1.0
.5
1..
1..=10
0..10

The lexer must apply the canonical numeric/range precedence rather than arbitrary delimiter splitting.

---

65. Quantum Interaction Tests

At minimum:

q[0]
q[0,1]
q[index]
apply H to q[0];
|0⟩
|1⟩

must tokenize deterministically.

The delimiter layer must remain independent of quantum hardware.

---

66. HDL Interaction Tests

At minimum:

module Device {
    input clock: Clock;
    output signal: Signal;
}

must produce the expected:

LBRACE
COLON
SEMICOLON
RBRACE

tokens where appropriate.

---

67. Resource/Capability Interaction Tests

Examples:

requires(
    capability("quantum.measurement"),
    capability("tensor.compute")
);

must be represented using ordinary delimiters.

No capability-specific delimiter tokens should be required.

---

68. Determinism

For identical:

- source bytes;
- language version;
- lexical configuration;
- compatibility mode;

the delimiter token stream must be identical.

Lexical behavior must not depend on:

- CPU;
- GPU;
- QPU;
- FPGA;
- node count;
- operating system locale;
- clock;
- random state;
- hash iteration order;
- network;
- target device;
- resource availability.

---

69. Safe Rust Requirement

The Rust implementation integrating this delimiter contract must use:

Rust 1.97 / Rust 1.97.1

and:

no unsafe Rust

This includes:

- delimiter scanning;
- source slicing;
- token creation;
- source spans;
- diagnostics;
- incremental lexing;
- streaming lexing;
- tooling integration.

No delimiter feature may require Rust "unsafe".

---

70. Generated ANTLR Integration

The canonical ANTLR lexer is:

grammar/antlr/ZamaniLexer.g4

The delimiter implementation must define exactly one lexical identity per delimiter spelling.

Required canonical rules are conceptually:

LPAREN     : '(' ;
RPAREN     : ')' ;

LBRACE     : '{' ;
RBRACE     : '}' ;

LBRACKET   : '[' ;
RBRACKET   : ']' ;

COMMA      : ',' ;
DOT        : '.' ;
COLON      : ':' ;
SEMICOLON  : ';' ;

AT         : '@' ;
HASH       : '#' ;

UNDERSCORE : '_' ;

The actual canonical lexer may compose these rules differently, but the resulting token vocabulary must be equivalent.

---

71. Required Canonical Lexer Corrections

The current repository requires the following integration corrections.

71.1 Add "HASH"

The normative lexical specification includes:

#

but the current canonical lexer does not expose a "HASH" delimiter token.

Production state requires:

HASH

to be added.

---

71.2 Standardize "SEMICOLON"

The canonical lexer defines:

SEMICOLON

while parts of "Core.g4" use:

SEMI

Production state requires one canonical identity:

SEMICOLON

No duplicate lexer token for ";".

---

71.3 Resolve "AT" versus "NANO_ANNOTATION"

The current lexer has:

NANO_ANNOTATION

and:

AT

The production architecture should use:

AT + IDENTIFIER

for generic annotations/attributes.

"NANO_ANNOTATION" should therefore either:

1. be removed from the canonical lexer;
2. become a compatibility-only construct;
3. or be moved into a formally activated dialect.

It must not shadow generic "AT" syntax.

---

71.4 Resolve standalone underscore

The parser already requires:

UNDERSCORE

for wildcard patterns.

The lexer must therefore emit:

UNDERSCORE

for standalone "_".

At the same time:

_value

must remain one identifier.

Longest-match behavior provides the required distinction.

---

72. No Duplicate Lexical Spellings

There must not be two default-mode lexer rules that independently claim the same delimiter spelling.

Examples prohibited:

COMMA : ',' ;
ARG_SEPARATOR : ',' ;

or:

COLON : ':' ;
TYPE_COLON : ':' ;

or:

SEMICOLON : ';' ;
SEMI : ';' ;

A single spelling must have one canonical lexical identity.

ANTLR itself warns against redundant string literals across lexer rules because identical spellings create ambiguous token ownership.

---

73. No Catch-All Override Before Delimiter Validation

A catch-all lexer rule such as:

UNKNOWN : . ;

must not interfere with valid delimiter rules.

Unknown-character handling must occur only after all valid lexical forms have had the opportunity to match.

Delimiter conformance tests must explicitly verify that every canonical delimiter is recognized before unknown-character recovery.

---

74. Versioning

Delimiter spellings are part of the language's source compatibility surface.

Changing:

( ) { } [ ] , . : ; @ # _

is a compatibility-sensitive language change.

Adding a new delimiter requires:

- specification update;
- lexer update;
- parser integration;
- AST impact analysis;
- tooling impact analysis;
- formatter update;
- compatibility entry;
- positive tests;
- negative tests;
- boundary tests.

Removing or changing an existing delimiter requires migration documentation.

---

75. Compatibility Policy

Adding a new delimiter can be source-breaking if existing identifiers or literals previously used that character.

Therefore every new delimiter must undergo collision analysis against:

- identifiers;
- keywords;
- literals;
- operators;
- comments;
- attributes;
- macros;
- dialects;
- interoperability formats.

No new delimiter may be introduced merely because a backend uses that punctuation.

---

76. Dialect Compatibility

A dialect may add punctuation only when explicitly enabled.

Core Zamani delimiter meanings remain stable.

A dialect must not silently change:

:
;
.
@
#
[
]
(
)
{
}

into unrelated meanings.

If a dialect requires different syntax, the dialect contract must declare the extension and compatibility behavior.

---

77. Interoperability

Imported external languages may use different delimiter systems.

Examples include:

- C/C++;
- Rust;
- Python;
- OpenQASM;
- HDL dialects;
- WASM textual forms;
- foreign DSLs.

Those syntaxes must be parsed by their interoperability frontend.

They must not modify Zamani's canonical delimiter vocabulary.

The translation boundary is:

foreign syntax
    |
foreign parser
    |
foreign representation
    |
Zamani semantic model

rather than:

foreign delimiter
    |
global Zamani lexer

---

78. Security

Delimiter processing must be resistant to lexical ambiguity attacks.

The implementation must correctly handle:

- Unicode confusables;
- malformed UTF-8;
- extremely large delimiter-separated lists;
- deeply nested delimiters;
- malformed nesting;
- unterminated constructs;
- delimiter-like content inside strings;
- delimiter-like content inside comments;
- macro-generated delimiter sequences.

Resource exhaustion must be reported as resource exhaustion, not as invalid delimiter syntax.

---

79. Performance

Delimiter recognition is a constant-size lexical decision.

The implementation must avoid unnecessary:

- heap allocations;
- string copying;
- Unicode normalization;
- semantic lookup;
- hardware queries;
- network access.

Delimiter recognition must remain independent of:

- machine size;
- target device;
- runtime state.

Large nested programs may consume more compiler resources, but this must not change the language definition.

---

80. Incremental Lexing

Incremental tooling must invalidate only the affected lexical region where practical.

Changing:

(

to:

[

must change the token from:

LPAREN

to:

LBRACKET

without requiring semantic re-analysis of unrelated source.

Incremental implementations must preserve exact source spans.

---

81. Streaming Lexing

Delimiter recognition must work in streaming/incremental source environments.

A delimiter is complete when its source character has been received.

For example:

(

can immediately produce:

LPAREN

while multi-character operators remain governed by longest-match rules.

The implementation must not require the entire source program to determine a single-character delimiter.

---

82. Error Recovery and IDEs

For incomplete source:

fn f(

the lexer should still emit:

FN
IDENTIFIER
LPAREN

The parser/LSP can then report the missing structure.

The lexer must not manufacture:

RPAREN

unless the parser operates in a clearly defined recovery representation.

Recovery tokens must never be confused with canonical source tokens used for compilation.

---

83. Documentation and Comments

Delimiter documentation belongs here.

Semantic explanations belong in:

grammar/spec/syntax.md

Lexical details belong in:

grammar/spec/lexical.md

Implementation details belong in:

grammar/antlr/ZamaniLexer.g4
src/lexer.rs

Conformance status belongs in:

grammar/grammar.md

Historical/proposed material belongs in:

grammar/Zamani-Grammar.md

No one of these may silently become a competing delimiter authority.

---

84. Required Integration with Other Lexer Files

"grammar/lexer/tokens.md"

Must reference this file as the canonical delimiter-token contract.

"grammar/lexer/operators.md"

Must own:

..
..=
::
->
=>
<
>
<=
>=
...

and must explicitly distinguish operators from delimiters.

"grammar/lexer/identifiers.md"

Must define:

IDENTIFIER

and coordinate standalone "_" with "UNDERSCORE".

"grammar/lexer/literals.md"

Must define delimiter interactions with literals.

"grammar/lexer/numeric-literals.md"

Must define:

.
..
..=

interaction with numbers/ranges.

"grammar/lexer/comments.md"

Must ensure delimiter characters inside comments are not emitted as parser tokens.

"grammar/lexer/unicode.md"

Must define Unicode/confusable policy.

"grammar/lexer/keywords.md"

Must ensure delimiter spellings are not simultaneously reserved-word spellings.

"grammar/lexer/diagnostics.md"

Must own diagnostic identifiers for delimiter errors.

"grammar/lexer/conformance.md"

Must execute the delimiter test matrix.

---

85. Required Integration with "grammar/spec/lexical.md"

"grammar/spec/lexical.md" remains the complete normative lexical authority.

This file supplies its delimiter subsection.

The canonical punctuation set must therefore be:

(
)
{
}
[
]
,
.
:
;
@
#
_

with operators separately defined.

No implementation may claim a delimiter that is absent from this contract without updating the specification.

---

86. Required Integration with "grammar/spec/syntax.md"

"grammar/spec/syntax.md" may consume:

LPAREN
RPAREN
LBRACE
RBRACE
LBRACKET
RBRACKET
COMMA
DOT
COLON
SEMICOLON
AT
HASH
UNDERSCORE

It must not redefine their lexical spelling.

This maintains the separation:

lexical specification
        |
        v
tokens
        |
        v
syntax specification

---

87. Required Integration with "grammar/antlr/Core.g4"

"Core.g4" must use the canonical delimiter token names.

In particular:

SEMICOLON

must replace or formally alias any stale:

SEMI

reference.

"HASH" must be available for:

#[...]

attribute syntax.

"UNDERSCORE" must be available for wildcard patterns.

---

88. Required Integration with "grammar/Zamani.g4"

"Zamani.g4" must not redefine delimiter tokens.

It is the composition/root grammar, not a second lexical authority.

The delimiter vocabulary must flow from:

ZamaniLexer

into the parser composition.

---

89. Required Integration with Frontend AST

The domain-neutral AST must represent semantic structure rather than raw delimiter tokens wherever possible.

Examples:

(a)

may become an expression node without a semantic parenthesis node.

But source spans must remain available for:

- diagnostics;
- formatting;
- tooling;
- macro/source transformation;
- provenance.

This is consistent with the requirement that the AST remain domain-neutral and not become a quantum/backend IR.

---

90. Required Integration with Semantic Analysis

Semantic analysis must determine the meaning established by delimiter structure.

Examples:

x[i]

could represent:

- array indexing;
- tensor indexing;
- quantum register access;
- HDL vector selection;
- distributed partition access.

The delimiter itself carries none of those meanings.

---

91. Required Integration with Compiler

The compiler consumes AST/semantic structures.

It must not inspect raw delimiter spellings to determine:

- CPU count;
- GPU count;
- QPU count;
- FPGA capacity;
- physical qubit mapping;
- network topology;
- memory capacity.

Delimiter syntax remains target-independent.

---

92. Required Integration with Runtime

Runtime behavior must never depend on the presence of a raw delimiter token.

By runtime time, delimiter syntax should already have been converted into semantic structures/IR.

---

93. Required Integration with Tooling

The same canonical token model must be consumed by:

- formatter;
- syntax highlighter;
- LSP;
- parser diagnostics;
- documentation tooling;
- macro tooling;
- source maps;
- code navigation;
- refactoring tools.

No tool may maintain an incompatible delimiter table.

---

94. Completion Checklist

This file is complete only when all of the following are true:

Authority

- [ ] one delimiter authority exists;
- [ ] "grammar/spec/lexical.md" references this contract;
- [ ] "ZamaniLexer.g4" conforms;
- [ ] "Core.g4" conforms.

Token identity

- [ ] "LPAREN";
- [ ] "RPAREN";
- [ ] "LBRACE";
- [ ] "RBRACE";
- [ ] "LBRACKET";
- [ ] "RBRACKET";
- [ ] "COMMA";
- [ ] "DOT";
- [ ] "COLON";
- [ ] "SEMICOLON";
- [ ] "AT";
- [ ] "HASH";
- [ ] "UNDERSCORE".

Conflicts resolved

- [ ] "SEMI" versus "SEMICOLON" resolved;
- [ ] "HASH" added;
- [ ] "AT" versus "NANO_ANNOTATION" resolved;
- [ ] standalone "_" versus identifier "_" resolved;
- [ ] delimiter/operator boundaries resolved;
- [ ] delimiter/literal boundaries resolved.

Syntax

- [ ] function syntax;
- [ ] block syntax;
- [ ] array/index syntax;
- [ ] attributes;
- [ ] paths;
- [ ] type annotations;
- [ ] lists;
- [ ] patterns;
- [ ] quantum syntax;
- [ ] HDL syntax;
- [ ] resource syntax;
- [ ] distributed syntax;
- [ ] AI/data syntax.

Scalability

- [ ] no fixed nesting limit;
- [ ] no fixed list length;
- [ ] no fixed quantum target count;
- [ ] no fixed tensor rank;
- [ ] no fixed hardware count;
- [ ] no fixed node count;
- [ ] no fixed resource count;
- [ ] no machine-specific delimiter semantics.

Safety

- [ ] Rust 1.97/1.97.1 compatible;
- [ ] no Rust "unsafe";
- [ ] no unsafe lexical architecture;
- [ ] deterministic diagnostics;
- [ ] malformed input handled deterministically.

Unicode

- [ ] Unicode delimiter confusables do not silently become ASCII delimiters;
- [ ] source spelling preserved;
- [ ] normalization not performed implicitly.

Tests

- [ ] positive;
- [ ] negative;
- [ ] boundary;
- [ ] Unicode;
- [ ] comment;
- [ ] string;
- [ ] character;
- [ ] numeric;
- [ ] quantum;
- [ ] HDL;
- [ ] resource;
- [ ] macro;
- [ ] incremental;
- [ ] determinism;
- [ ] scalability;
- [ ] compatibility.

---

95. Final Canonical Delimiter Contract

The production Zamani lexer has exactly this core delimiter vocabulary:

LPAREN      = "("
RPAREN      = ")"

LBRACE      = "{"
RBRACE      = "}"

LBRACKET    = "["
RBRACKET    = "]"

COMMA       = ","
DOT         = "."
COLON       = ":"
SEMICOLON   = ";"

AT          = "@"
HASH        = "#"

UNDERSCORE  = "_"

The following remain outside this delimiter file:

RANGE
RANGE_INCLUSIVE
DOUBLE_COLON
ARROW
FAT_ARROW

PLUS
MINUS
STAR
SLASH
PERCENT
ASSIGN
NOT_OPERATOR
TILDE
AMPERSAND
PIPE
CARET
QUESTION

LT
GT
LE
GE
SHIFT_LEFT
SHIFT_RIGHT
EQ_EQ
NOT_EQ
AND_AND
OR_OR
PLUS_ASSIGN
MINUS_ASSIGN
STAR_ASSIGN
SLASH_ASSIGN

Those belong to the operator contract.

---

96. Architectural Result

The delimiter architecture therefore becomes:

Source
  |
  v
UTF-8 / Unicode validation
  |
  v
Canonical ZamaniLexer
  |
  +--> keywords
  +--> identifiers
  +--> literals
  +--> operators
  +--> delimiters
  +--> comments/trivia
  |
  v
Canonical Token Stream
  |
  v
Core.g4 / Zamani.g4
  |
  v
Domain-Neutral AST
  |
  v
Structural Validation
  |
  v
Semantic Analysis
  |
  +--> classical
  +--> quantum
  +--> hybrid
  +--> HDL
  +--> hardware
  +--> AI
  +--> data
  +--> distributed
  +--> networking
  +--> security
  +--> future domains
  |
  v
Canonical Semantic Model
  |
  +--> quantum::ir
  +--> classical/control/data representations
  +--> HDL/hardware representations
  +--> resource/capability metadata
  |
  v
Optimization
  |
  v
Routing / Scheduling / Resilience / QEC / ZQN
  |
  v
HAL / Target Lowering
  |
  v
CPU / GPU / FPGA / QPU / ASIC / Future Hardware

The delimiter layer therefore remains deliberately small, deterministic, extensible, hardware-independent, Unicode-safe, and scalable.

It provides structure without becoming a hidden second language.

Most importantly, delimiters never encode today's machine limits. A tiny program and a program describing computation over enormous collections, quantum systems, distributed systems, tensors, HDL structures, or future hardware use the same delimiter model; only the available resources and later semantic/compilation layers determine what can actually be realized.