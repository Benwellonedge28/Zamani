

grammar/spec/lexical.md

Zamani Lexical Specification

Path: "grammar/spec/lexical.md"
Language: Zamani
Specification status: Production lexical architecture
Compiler baseline: Rust 1.97 / Rust 1.97.1
Rust safety: "unsafe" Rust prohibited
Scalability: No artificial language-level machine-size limits
Primary execution principle: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)

---

1. Purpose

This document defines the normative lexical architecture of the Zamani programming language.

Lexical analysis converts Zamani source text into a deterministic stream of source tokens.

Source Text
    │
    ▼
Source Map
    │
    ▼
Lexical Scanner
    │
    ├── Tokens
    └── Lexical Diagnostics
         │
         ▼
     Parser
         │
         ▼
       AST

The lexical layer is responsible for:

- Unicode-aware source processing where specified;
- whitespace handling;
- comment handling;
- identifiers;
- keywords;
- literals;
- operators;
- punctuation;
- source spans;
- lexical diagnostics;
- deterministic tokenization.

The lexical layer is not responsible for:

- type checking;
- name resolution;
- ownership;
- effects;
- quantum semantics;
- hardware capabilities;
- optimization;
- scheduling;
- package resolution;
- runtime behavior;
- target selection.

---

2. Lexical Authority

Zamani must have one canonical lexical specification.

The following implementations/documentation must conform to it:

grammar/spec/lexical.md
        │
        ├───────────────┐
        ▼               ▼
src/lexer.rs       ZamaniLexer.g4
        │               │
        └───────┬───────┘
                ▼
        Canonical Token Model
                │
                ▼
           src/parser.rs

"src/lexer.rs" is the reference executable lexer.

"ZamaniLexer.g4" is the ANTLR representation.

Neither implementation is permitted to silently define lexical behavior that is absent from the canonical specification.

The implementation-conformance document "grammar/grammar.md" must describe what the reference implementation currently accepts, while this document defines the intended lexical contract.

---

3. Lexical Invariants

Every production lexer MUST satisfy these invariants.

3.1 Determinism

For the same:

source bytes
language version
lexical configuration

the lexer must produce the same token sequence and diagnostics.

Lexing must not depend on:

- CPU architecture;
- operating system;
- available RAM;
- number of CPU cores;
- QPU;
- GPU;
- network;
- wall-clock time;
- hash-map iteration order;
- random state.

---

3.2 Source preservation

Every token that originates from source must have sufficient source-location information to support diagnostics.

The repository's lexer already associates tokens with "Span" information.

The lexical architecture therefore requires:

Token
├── token kind
├── source span
└── source representation/value

The exact internal representation may evolve, but source locations must remain available.

---

3.3 No silent corruption

The lexer MUST NOT:

- silently discard an invalid character;
- silently reinterpret malformed literals;
- silently merge unrelated tokens;
- silently truncate source;
- silently normalize meaningful characters;
- silently change numeric values.

Invalid source must produce a structured lexical diagnostic.

---

3.4 Progress guarantee

For every non-EOF input position:

lexer iteration
    ↓
consume input
or
emit diagnostic and consume/recover

The lexer must never repeatedly inspect the same invalid input position forever.

---

4. Character Model

Zamani source is fundamentally a sequence of Unicode scalar values represented in source encoding.

The implementation must distinguish:

source bytes
Unicode decoding
characters
lexical tokens

A malformed source encoding must produce a lexical error rather than being interpreted as arbitrary text.

Identifiers and keywords have their own normalization rules defined by this specification.

---

5. Unicode Normalization

Zamani MUST NOT silently normalize identifiers in a way that changes program identity.

For the initial stable language:

- ASCII letters remain the normative identifier alphabet unless a future language version explicitly expands it;
- Unicode may be permitted inside strings, characters, comments, and other explicitly Unicode-capable literals;
- visually similar identifiers must not automatically be considered identical;
- Unicode normalization must not silently alter an identifier's semantic identity.

If full Unicode identifiers are introduced later, the language specification must define:

- allowed Unicode categories;
- normalization form;
- combining-character rules;
- confusable handling;
- identifier comparison;
- keyword recognition.

That change must be versioned.

---

6. Source Encoding

The reference compiler should consume UTF-8 source.

The lexical pipeline is:

UTF-8 bytes
   ↓
Unicode validation
   ↓
character stream
   ↓
lexical scanning

Invalid UTF-8 is a lexical/source-encoding error.

The compiler must not reinterpret invalid bytes according to host-specific encodings.

---

7. Whitespace

Whitespace separates lexical units but is otherwise insignificant unless a future language feature explicitly assigns semantic meaning to it.

The baseline whitespace set includes:

space
tab
carriage return
line feed

Additional Unicode whitespace characters should not automatically become syntax-significant.

If Unicode whitespace is accepted in a future version, it must be explicitly specified and tested.

---

8. Newlines

Zamani does not make newline placement intrinsically semantic in the baseline grammar.

Therefore:

let x = 1;
let y = 2;

and equivalent whitespace-separated forms should tokenize consistently.

Newlines may nevertheless be preserved in source spans for:

- diagnostics;
- formatting;
- IDE tooling;
- source mapping;
- documentation;
- debugging.

The lexer must not discard information needed by those systems merely because the parser does not use newlines as syntax.

---

9. Comments

Zamani supports line and block comments.

9.1 Line comments

A line comment begins with:

//

and continues until the end of the logical source line.

Example:

let x = 42; // explanation

---

9.2 Block comments

A block comment begins with:

/*

and terminates with:

*/

The baseline form is non-nesting.

Example:

/*
   multi-line comment
*/

Nested block comments must not be silently accepted unless explicitly added to the language specification.

An unterminated block comment is a lexical error.

---

10. Comment Ownership

Comments are lexically recognized but normally do not become ordinary parser tokens.

The implementation may preserve comment information for:

- documentation generation;
- formatting;
- IDE tooling;
- source transformation;
- macro tooling.

If comments are discarded from the parser token stream, their source spans must still remain recoverable by tooling that requires them.

---

11. Identifiers

The baseline identifier grammar is:

IDENTIFIER =
    IDENTIFIER_START
    { IDENTIFIER_CONTINUE } ;

IDENTIFIER_START =
    ASCII_LETTER
    | "_" ;

IDENTIFIER_CONTINUE =
    ASCII_LETTER
    | DIGIT
    | "_" ;

ASCII_LETTER =
    "a".."z"
    | "A".."Z" ;

DIGIT =
    "0".."9" ;

Examples:

x
value
_result
qubit_count
Patient
QuantumState

Invalid examples:

1value
42abc

---

12. Identifier and Keyword Resolution

The lexer must recognize keywords before producing an ordinary identifier token.

For example:

fn
let
quantum
struct

must be classified according to the active Zamani language version.

A keyword must not simultaneously become an identifier token in the same lexical context unless the language explicitly defines contextual keywords.

---

13. Reserved vs Contextual Keywords

Zamani should distinguish:

Reserved keyword

A word that cannot be used as an ordinary identifier.

Contextual keyword

A word that receives special meaning only in specific grammar positions.

This distinction is important because Zamani has a very broad keyword inventory.

New functionality should prefer contextual keywords when doing so avoids unnecessary source incompatibility.

---

14. Canonical Keyword Registry

The keyword mapping must have one authoritative implementation.

Conceptually:

lexeme
   ↓
Keyword Registry
   ↓
TokenType

The Rust lexer, ANTLR lexer, documentation, formatter, LSP, and syntax highlighter should derive or validate against the same canonical keyword set.

The repository currently contains a large "TokenType" keyword inventory, including core, OOP, quantum, temporal, mathematical, and advanced-system keywords.

That inventory must be audited before additional keywords are added.

---

15. Keyword Explosion Prevention

The lexer must not become a dumping ground for every future Zamani capability.

A new concept should become a keyword only when it genuinely requires lexical recognition.

Prefer:

generic identifier
+
compositional syntax
+
semantic resolution

over:

one keyword for every operation

This is particularly important for:

- quantum gates;
- mathematical operations;
- hardware capabilities;
- AI operations;
- optimization algorithms;
- device names.

---

16. Boolean Literals

The baseline boolean literals are:

true
false

They are lexed as boolean/keyword tokens according to the canonical token model.

They must not also be emitted as ordinary identifiers.

---

17. Nil Literals

The baseline null-like literals are:

nil
null

Whether both remain stable language forms must be determined by the canonical semantic specification.

If both are retained, they must have one explicitly defined semantic relationship.

The lexer must not assign them different meanings accidentally.

---

18. Integer Literals

The baseline integer form is:

INTEGER =
    DIGIT { DIGIT } ;

Example:

0
1
42
1000000

The lexer must preserve the source representation sufficiently for later semantic processing.

Numeric interpretation belongs to semantic/literal processing rather than being coupled to machine word size.

---

19. Integer Scalability

The lexer must not assume that an integer fits in:

u32
u64
usize
i64

merely because the host compiler uses those types internally.

For example:

999999999999999999999999999999999999

must not be lexically rejected solely because it exceeds the host machine's native integer width.

The lexical layer should identify it as an integer literal.

Semantic analysis determines whether its value is valid for the selected type/context.

Where arbitrary-precision semantics are required, the implementation should use a safe representation capable of preserving the literal without overflow.

---

20. Floating-Point Literals

The baseline floating form is:

FLOAT =
    DIGIT { DIGIT }
    "."
    DIGIT { DIGIT } ;

Example:

0.0
1.5
3.14159

The lexer must not silently round a floating literal to the host machine's floating-point representation during lexical scanning.

Lexing identifies the literal.

Semantic analysis determines:

- precision;
- range;
- representation;
- constant evaluation;
- target compatibility.

---

21. Future Numeric Literal Extensions

Future versions may introduce:

scientific notation
hexadecimal
binary
octal
digit separators
arbitrary precision
decimal
rational
complex
fixed point
quantum numeric literals
symbolic numeric literals

Each extension must be explicitly specified and must not make existing literals ambiguous.

---

22. String Literals

The baseline string literal begins and ends with:

"

Example:

"Hello, Zamani"

The lexer must recognize escape sequences according to a single canonical escape table.

An unterminated string is a lexical error.

---

23. Escape Sequences

The canonical escape specification must define each supported escape explicitly.

At minimum, the architecture must distinguish:

escaped quote
escaped backslash
newline
carriage return
tab
Unicode/code-point escape forms, if supported

Unknown escape sequences must not silently become arbitrary characters.

---

24. Character Literals

Character literals use:

'

Example:

'a'
'\n'

A character literal must represent exactly one valid character/code point according to the language's character model.

An unterminated or multi-character literal must produce a lexical diagnostic unless a future version explicitly defines a different character-literal model.

---

25. Punctuation

The canonical punctuation inventory includes:

(
)
{
}
[
]
,
.
;
:

and other syntactically significant punctuation described below.

Each punctuation sequence must have one lexical interpretation at a given source position.

---

26. Operators

The baseline operator families include:

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
<
>
?

and compound/multi-character operators:

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

The lexer must use longest-valid-token matching where operators overlap.

For example:

>=

must not become:

>
=

when ">=" is a valid token.

---

27. Operator Token Canonicalization

The current Rust lexer contains overlapping token concepts such as:

BitAnd / Ampersand
BitOr / Pipe
Question / QuestionMark

These must not remain semantically ambiguous.

The production token model should establish a single canonical token identity for each lexical spelling.

For example:

&

must have one canonical lexical token.

Whether that token is later interpreted as:

- bitwise AND;
- reference operator;
- another semantic operation;

is determined by parser context and semantic analysis.

The same principle applies to:

|
?

and other overloaded syntax.

---

28. Overloaded Lexemes

One lexical spelling may have multiple grammatical or semantic meanings.

This does not require multiple lexer token types.

Example:

&

can participate in different constructs.

The preferred architecture is:

source lexeme
     ↓
canonical token
     ↓
parser context
     ↓
AST meaning
     ↓
semantic meaning

not:

same source spelling
     ↓
many competing lexer token kinds

unless there is a genuine lexical distinction.

---

29. Arrow Tokens

Zamani currently uses arrow-like forms including:

->
=>

These must remain distinct canonical tokens.

Typical uses include:

function return types
match arms

but semantic ownership belongs to the parser/AST rather than the lexer.

---

30. Range Tokens

The range operators are:

..
..=

The lexer must recognize them as complete operators.

They must not be confused with two consecutive "." tokens.

---

31. Question Mark

The question-mark spelling:

?

must have one canonical token.

Its meaning may include expression-level propagation or future type syntax depending on grammar context.

The lexer must not decide whether "?" means:

try propagation
optional type
other future syntax

That belongs to the parser and semantic system.

---

32. Hash and Attributes

The hash character:

#

is lexically significant because Zamani supports attribute-style syntax.

For example:

#[attribute]

The lexer recognizes the punctuation.

The parser and AST determine the structure.

The semantic system determines whether the attribute exists and what it means.

---

33. At Sign and Annotations

The at sign:

@

supports annotation-oriented syntax.

Examples include concepts such as:

@atom
@molecule(...)

The lexer may produce a dedicated "NanoAnnotation" token when the lexical form is unambiguously defined.

However, annotation semantics belong downstream.

An annotation must never directly alter hardware behavior during lexical scanning.

---

34. Quantum Literals

Zamani supports quantum-oriented literal concepts such as:

|0⟩
|1⟩
|+⟩
|-⟩

The current lexical model includes a "QuantumLiteral" token.

The lexer should recognize only the formally specified literal language.

It must not attempt to:

- simulate a quantum state;
- allocate a qubit;
- determine physical qubit count;
- select a backend;
- determine hardware topology.

A quantum literal is source data.

Its computational meaning is established later.

---

35. Quantum Literal Scalability

Quantum literals must not encode an implicit finite machine.

For example:

|0⟩

is a semantic value.

It is not:

physical_qubit_0

Likewise, lexing a quantum construct must never allocate:

QPU resources
physical qubits
memory
hardware registers

---

36. Quantum Operations Are Not Lexical Hardware Definitions

The lexer must not define a permanent vocabulary such as:

IBM_H
IBM_CX
DEVICE_17_GATE
PHYSICAL_QUBIT_42

Quantum source should use compositional language constructs.

The mapping to native gates belongs downstream:

Quantum Source
      ↓
Quantum AST
      ↓
Quantum Semantic IR
      ↓
Decomposition
      ↓
Synthesis
      ↓
Routing
      ↓
Scheduling
      ↓
Target

This is required for POCO-REAF.

---

37. Nano Annotations

The lexer currently supports a dedicated nano annotation concept.

The lexical layer should recognize the syntactic boundary only.

For example:

@atom
@molecule(x)

must not cause the lexer to determine:

- physical dimensions;
- chemistry;
- biological behavior;
- hardware;
- simulation method.

Those are semantic/domain concerns.

---

38. MTS Literals

The current token model contains:

MTSLiteral

but the implementation-conformance grammar documents that the lexer does not currently emit the literal form and that "mts[...]" can currently be lexed as ordinary identifier/punctuation sequences.

This is a production specification issue.

The repository must not simultaneously claim:

MTS literal is implemented

and:

MTS literal is not emitted

The correct status model is:

MTSLiteral
    ↓
Specified: YES
    ↓
Token enum: YES
    ↓
Reference lexer emission: VERIFY/IMPLEMENT
    ↓
Parser: VERIFY/IMPLEMENT
    ↓
AST: VERIFY/IMPLEMENT
    ↓
Semantic model: VERIFY/IMPLEMENT
    ↓
Tests: REQUIRED

Until that pipeline is complete, "MTSLiteral" must be marked experimental/unimplemented, not stable.

---

39. Keyword "mts"

The lexical status of:

mts

must also be clarified.

If it is:

reserved keyword

then it must be lexed consistently as such.

If it is:

contextual keyword

then its special meaning belongs to specific grammar contexts.

If it is merely the prefix of a literal syntax:

mts[...]

the lexer may recognize the complete literal when the grammar formally defines it.

There must not be three simultaneous interpretations.

---

40. Quantum, Nano and MTS Lexical Boundaries

Domain-specific lexical constructs must follow this architecture:

Lexical identity
        ↓
Source representation
        ↓
AST
        ↓
Domain semantics

The lexer must not become a domain interpreter.

This ensures that adding a future computational domain does not require rewriting the fundamental lexer architecture.

---

41. Strings vs Domain Literals

Domain syntax must not accidentally consume string contents.

For example:

"mts[123]"

is a string.

It must never become an "MTSLiteral".

Likewise:

"|0⟩"

is a string rather than a quantum literal.

Lexical context must be unambiguous.

---

42. Token Precedence

When lexical forms overlap, the lexer must follow a deterministic priority.

Recommended order:

1. whitespace/comments
2. multi-character operators
3. domain-specific complete literals
4. literals
5. identifiers/keywords
6. single-character punctuation/operators
7. invalid characters
8. EOF

The exact implementation may differ, but the result must be deterministic.

---

43. Longest-Match Rule

Where multiple tokens share a prefix, the longest valid token must be selected.

Examples:

>=  → GreaterThanEqual
>   → GreaterThan

>>  → RightShift
>   → GreaterThan

::  → DoubleColon
:   → Colon

->  → Arrow/ThinArrow canonical token
-   → Minus

=>  → FatArrow
=   → Assign

The repository currently contains both "Arrow" and "ThinArrow"; this is another token-model duplication that must be resolved into one canonical lexical identity unless there is a documented reason for both.

---

44. Token Model

The production token representation should conceptually contain:

Token {
    kind
    span
    source/value
}

The current implementation already follows this general model.

The token kind must describe lexical identity.

The token payload must preserve the source value or safely represent its decoded value as appropriate.

---

45. Token Values

Token values must not be prematurely converted to machine-specific representations.

For example, an integer token should not necessarily become:

u64

during lexing.

Likewise, a floating literal should not automatically become:

f64

unless the semantic contract explicitly requires that representation.

The lexer identifies syntax.

Literal interpretation belongs to a later stage.

---

46. Source Spans

Every emitted source token must have a valid span.

A span must identify:

file
start
end

according to the repository's source-map model.

The lexer already uses source-map types including:

BytePos
FileId
SourceFile
Span

and attaches spans to tokens.

Span handling must remain independent of target architecture.

---

47. Span Correctness

For every token:

start <= end

and the span must correspond to the source region that produced the token.

The lexer must never report:

- negative positions;
- reversed spans;
- spans belonging to another file;
- positions beyond the source;
- inconsistent byte/character offsets.

---

48. Multi-Byte Unicode

When Unicode characters occur in source, the implementation must distinguish:

byte offsets
character positions
display columns

A byte-based internal source map is acceptable and generally preferable for compiler infrastructure, provided diagnostics correctly translate positions for user-facing display.

---

49. Lexical Errors

A lexical error must contain at least:

message
span

The current "LexerError" already follows this basic model.

Production diagnostics should additionally support structured error codes where the compiler diagnostic framework provides them.

Examples:

ZLEX001 invalid character
ZLEX002 unterminated string
ZLEX003 invalid escape
ZLEX004 unterminated block comment
ZLEX005 malformed character literal
ZLEX006 malformed domain literal

The final error-code registry should be centralized rather than duplicated in every lexer branch.

---

50. Illegal Token

An "Illegal" token may be useful for parser recovery and tooling, but its meaning must be explicit.

An illegal token represents:

«Input that the lexer could not classify as valid Zamani lexical syntax.»

It must not be treated as valid program semantics.

A production compiler should normally report the lexical diagnostic immediately while retaining enough token structure for controlled recovery.

---

51. EOF

The lexer must emit exactly one logical EOF token for a successfully scanned source unit.

EOF must have a valid zero-width span at the end of the source.

No source text follows EOF.

---

52. Lexer State

The lexer should not rely on hidden global mutable state.

State should be associated with the lexer/source instance.

The current implementation stores source-related information in the lexer structure and uses an explicit keyword mapping.

Any shared immutable tables may be safely shared.

Compilation-specific mutable state must remain isolated.

---

53. Thread Safety

The lexical architecture should support independent compilation units being lexed concurrently.

This means:

Source A → Lexer A
Source B → Lexer B
Source C → Lexer C

must not require shared mutable lexical state.

Parallel compilation must not alter lexical results.

---

54. Memory Scalability

The lexer must be designed for source files ranging from:

tiny source files

to:

very large generated programs

subject to available resources.

It must not define artificial limits such as:

MAX_TOKENS = 1_000_000
MAX_SOURCE = 10MB
MAX_IDENTIFIERS = 100_000

unless such values are explicit configurable resource policies rather than language rules.

---

55. Resource-Bounded Compilation

Production compilers may impose resource budgets.

Examples:

memory budget
time budget
diagnostic budget
token budget
source-size policy

Such limits must be:

- explicit;
- configurable where appropriate;
- observable;
- documented;
- distinguishable from lexical invalidity.

For example:

source is lexically valid
but compilation budget was exceeded

must not be reported as:

invalid Zamani syntax

---

56. No Host-Machine Assumptions

Lexical behavior must not depend on:

usize width
pointer width
CPU endianess
CPU instruction set
host operating system
native integer size
native floating-point size

This is essential for POCO-REAF.

A Zamani source file must have the same lexical meaning regardless of whether it is compiled on:

x86-64
ARM
RISC-V
GPU-hosted environment
distributed compiler
quantum development system
future architecture

---

57. Rust Implementation Requirements

The repository declares Rust 2021 and a Rust version requirement of 1.97/1.97.1.

The reference lexer must therefore remain compatible with the selected Rust baseline.

The lexical implementation must use safe Rust.

The following are prohibited in Zamani compiler source:

unsafe { ... }
unsafe fn ...
unsafe impl ...
unsafe trait ...

The lexer must not require unsafe memory access for performance.

Performance optimization must use safe mechanisms such as:

- slices;
- iterators where appropriate;
- indexing with validated bounds;
- owned/borrowed strings;
- "Arc" where sharing is appropriate;
- explicit buffers;
- deterministic data structures.

---

58. Source-Level "unsafe" Keyword

The existence of a Zamani source-level keyword named:

unsafe

must not be confused with Rust implementation safety.

The current language grammar includes an "unsafe" source construct.

If Zamani retains this source-level concept, its semantics must be defined independently.

The Rust compiler implementation itself must remain safe.

If the final Zamani language policy prohibits unsafe source semantics as well, the keyword must eventually be removed/deprecated through a versioned grammar change.

The lexical specification must not silently conflate these two meanings.

---

59. Keyword Inventory Audit

The current token inventory contains a very large number of keywords, including:

core/control
OOP
modules
effects
concurrency
quantum
nano
Sankofa
temporal
dependent types
omniversal/system concepts
business/payment concepts
graphics/video concepts

as visible in "src/lexer.rs".

This must be audited before declaring the complete keyword set stable.

Every keyword must have:

lexical purpose
parser usage
AST representation
semantic meaning
tests
compatibility status

An unused keyword should not automatically become part of the stable language.

---

60. Keyword Status Table

The production documentation should maintain a generated or validated table:

Lexeme| Token| Status| Parser| Semantic owner
"fn"| "KeywordFn"| Stable| Function declaration| Semantic/IR
"let"| "KeywordLet"| Stable| Binding| Semantic/IR
"quantum"| "KeywordQuantum"| Domain| Quantum parser| Quantum semantic layer
"nano"| "KeywordNano"| Domain| Nano parser| Nano semantic layer
"mts"| "KeywordMts"| Verify| Verify| Temporal/MTS
"unsafe"| "KeywordUnsafe"| Policy-dependent| Safety parser| Safety semantics
...| ...| ...| ...| ...

The table should eventually be generated from the canonical keyword registry to prevent documentation drift.

---

61. Contextual Interpretation

The lexer should remain as simple as possible.

For example:

quantum

should be recognized as the canonical quantum keyword if it is reserved.

But the lexer should not determine whether the following construct is:

quantum circuit
quantum operation
quantum type
quantum expression

That belongs to the parser and semantic layers.

---

62. Lexical vs Semantic Names

An identifier such as:

H
CNOT
U
Tensor
Patient
Machine

must not receive special lexical meaning merely because a library, backend, or domain currently recognizes it.

This is especially important for quantum computing.

User-defined operations and mathematical abstractions must remain possible.

---

63. Hardware Independence

The lexer must never encode:

physical qubit identifiers
QPU topology
gate availability
coupling maps
pulse names
device-specific registers
device-specific memory
CPU registers
GPU blocks

Hardware information belongs downstream.

This guarantees that lexical syntax does not become the limiting factor for future computational substrates.

---

64. Domain Extensibility

Adding a new Zamani computational domain should normally require:

new semantic constructs
possibly new parser productions
possibly new contextual syntax

but should not require redesigning the fundamental lexical engine.

The lexer must provide generic building blocks:

identifier
literal
operator
punctuation
annotation
delimiter

from which new domains can be composed.

---

65. ANTLR Synchronization

"Zamani.g4" and any future:

grammar/ANTLR/ZamaniLexer.g4

must implement the same lexical contract.

ANTLR is not an alternative language.

It is an alternative representation/tooling implementation.

The conformance suite must compare:

Rust lexer
vs
ANTLR lexer

using the same corpus.

---

66. Lexer Conformance Tests

The lexical test suite must include:

grammar/tests/lexical/
├── whitespace/
├── comments/
├── identifiers/
├── keywords/
├── integers/
├── floats/
├── strings/
├── chars/
├── operators/
├── punctuation/
├── quantum/
├── nano/
├── mts/
├── unicode/
├── invalid/
├── spans/
└── compatibility/

---

67. Positive Lexical Tests

Every stable token must have positive tests.

Examples:

let
fn
quantum
struct
123
3.14
"hello"
'a'
|0⟩
@atom

Each test should verify:

token kind
token value
source span
token ordering
EOF behavior

---

68. Negative Lexical Tests

Negative tests must include:

unterminated string
unterminated character
unterminated block comment
invalid escape
invalid numeric literal
invalid UTF-8
illegal character
malformed quantum literal
malformed annotation
malformed future-domain literal

Negative tests must verify that the lexer fails deterministically and makes progress.

---

69. Ambiguity Tests

The test suite must explicitly test overlapping tokens.

Examples:

>
>=

<
<=

=
==

!
!=

&
&&

|
||

.
..

:
::

-
->

=
=>
?

This prevents accidental changes to longest-match behavior.

---

70. Keyword/Identifier Tests

Every reserved keyword must be tested against an identifier that resembles it.

Examples:

fn
fn_value

quantum
quantum_state

let
letter

The test suite must verify that only the exact reserved spelling receives keyword treatment.

---

71. Literal Boundary Tests

Test boundaries such as:

123abc
1.2.3
"abc"
"abc
'a'
'ab'
|0⟩
|0
@atom
@atom(...)

The purpose is to ensure the lexer does not accidentally consume too much or too little input.

---

72. Property-Based Testing

Where practical, the lexical layer should use property-based testing.

Useful properties include:

lexer always terminates
lexer always makes progress
all token spans are valid
tokens preserve source ordering
EOF occurs exactly once
valid generated identifiers round-trip
valid operators tokenize deterministically

Property-based tests must remain bounded by explicit test resources rather than hard-coding language limits.

---

73. Fuzzing

The lexer should be fuzz-tested with arbitrary byte and Unicode inputs.

Required properties:

never undefined-behave
never hang
never infinite-loop
never panic on ordinary malformed input
never produce invalid source spans

The implementation should convert malformed input into diagnostics rather than crashing.

---

74. Parser Integration

The lexical contract ends at tokens.

The parser owns grammar.

Therefore the lexer must not attempt to parse:

functions
types
quantum circuits
match arms
modules
effects

beyond whatever lexical grouping is required to identify a token.

---

75. AST Integration

The lexer has no direct knowledge of AST structure.

The flow remains:

Token
 ↓
Parser
 ↓
AST

This separation prevents lexical implementation changes from unnecessarily coupling to semantic representation.

---

76. IR Integration

The lexer must never emit target-specific IR.

The correct architecture is:

Lexical Token
    ↓
Parser
    ↓
AST
    ↓
Semantic Analysis
    ↓
Canonical IR

This is particularly important for quantum computing.

---

77. Quantum IR Integration

Quantum lexical forms eventually reach:

quantum::ir

through the frontend and semantic/lowering layers.

The lexer must not know:

gate decomposition
routing
scheduling
noise
calibration
physical qubits
backend instructions

This separation enables target-independent quantum programs.

---

78. Diagnostics and Recovery

Lexical recovery must be conservative.

If the lexer encounters:

@

without enough information to determine a valid annotation, it should report a diagnostic according to the lexical contract.

It must not invent:

Identifier("...")

merely to keep parsing alive.

Recovery should preserve as much source structure as possible without falsely claiming semantic validity.

---

79. Documentation Synchronization

The following documents must remain synchronized:

grammar/README.md
grammar/DESIGN.md
grammar/spec/lexical.md
grammar/grammar.md
grammar/Zamani.g4
grammar/Zamani-Grammar.md

But their roles differ:

README.md
    architecture

DESIGN.md
    detailed design/invariants

spec/lexical.md
    normative lexical specification

grammar.md
    current implementation behavior

Zamani.g4
    ANTLR grammar representation

Zamani-Grammar.md
    broader language design/history/proposals

No document may silently override the normative lexical specification.

---

80. Feature Lifecycle

A lexical feature should progress through:

PROPOSED
   ↓
SPECIFIED
   ↓
IMPLEMENTED
   ↓
TESTED
   ↓
CONFORMANT
   ↓
STABLE

It must not be described as stable merely because its enum variant exists.

For example:

TokenType::MTSLiteral

does not by itself prove that:

mts[...]

is implemented end-to-end.

---

81. No Duplicate Token Authorities

There must eventually be one canonical mapping for:

lexeme → token kind

Duplicated mappings in:

Rust lexer
ANTLR lexer
documentation
formatter
LSP
syntax highlighter

should either be generated or continuously validated.

This prevents drift.

---

82. Generated Artifacts

Where generated lexer/parser artifacts are used, the source grammar remains authoritative.

Generated files must not be manually modified as the primary language-definition mechanism.

The build process should make it possible to determine:

which grammar generated this artifact
which language version it implements
which generator version was used

---

83. Reproducibility

Lexical generation must be reproducible.

Given the same:

canonical grammar
generator version
language version
tool configuration

the resulting generated artifacts should be reproducible wherever the generator guarantees deterministic output.

---

84. Compatibility

A lexical change can be breaking even when the parser would otherwise understand the resulting token.

Examples:

identifier → keyword
one token → two tokens
two tokens → one token
operator reassignment
literal syntax change
escape syntax change
comment syntax change

Therefore every lexical change requires compatibility analysis.

---

85. Adding a Keyword

Before adding a keyword:

1. Search existing source usage.
2. Determine whether contextual syntax is sufficient.
3. Determine whether an identifier must become reserved.
4. Update the canonical keyword registry.
5. Update Rust lexer.
6. Update ANTLR lexer.
7. Update parser.
8. Update documentation.
9. Add compatibility tests.
10. Add positive/negative lexical tests.

---

86. Removing a Keyword

Removing a keyword requires determining whether existing source can now use it as an identifier.

The change must be versioned where necessary.

A deprecated keyword should not silently change meaning without a migration path.

---

87. Adding an Operator

Before adding an operator:

1. Check lexical prefix collisions.
2. Check longest-match behavior.
3. Check parser precedence.
4. Check AST representation.
5. Check semantic meaning.
6. Check ANTLR conformance.
7. Add ambiguity tests.
8. Check source compatibility.

---

88. Adding a Domain Literal

Before adding a domain-specific literal:

new_literal

must have:

lexical grammar
token representation
parser production
AST representation
semantic representation
error rules
span rules
conformance tests
compatibility policy

A token enum addition alone is insufficient.

---

89. Resource Scaling

Lexical processing must scale with source size.

The implementation should avoid unnecessary:

O(source_size²)

behavior.

Preferred behavior is approximately:

O(source_size)

for ordinary lexing, subject to Unicode decoding and diagnostics.

Memory use should scale with the information actually required by the compiler pipeline.

Streaming or incremental lexing may be introduced for very large sources.

---

90. Incremental Lexing

The lexical architecture should permit incremental compilation.

A source edit should not require re-lexing unrelated files unnecessarily.

The design should therefore preserve:

file identity
source ranges
token spans
stable source-map relationships

where practical.

Incremental lexing must produce results equivalent to complete lexing of the resulting source.

---

91. Parallel Lexing

Independent source files may be lexed concurrently.

The lexical result for a file must not depend on whether another file is being compiled simultaneously.

This enables scalable compilation for large Zamani projects.

---

92. Modules

Lexing a module must not require reading imported modules.

For example:

import quantum::foo;

is lexed locally.

Dependency resolution occurs later.

The lexer must not perform filesystem or network operations.

---

93. Security

The lexer must treat source as untrusted input.

It must not:

- execute source;
- evaluate arbitrary expressions;
- access secrets;
- access the network;
- invoke subprocesses;
- open arbitrary files;
- perform package resolution.

Parsing source must remain an isolated compiler operation.

---

94. Source Size and Denial-of-Service Protection

Very large or adversarial source must not cause uncontrolled resource consumption.

The compiler may provide explicit resource budgets.

However, such limits must be distinguished from language rules.

For example:

Zamani language:
    valid

Compiler invocation:
    rejected because memory budget = 512 MB

is preferable to:

invalid Zamani

when the program is lexically valid.

---

95. Diagnostics Must Be Resource-Aware

If a diagnostic budget is exhausted, the compiler should report a resource diagnostic rather than silently dropping errors.

The lexer should avoid generating unbounded duplicate diagnostics from one malformed construct.

---

96. Stable Lexical Contract

The stable lexical contract consists of:

source encoding
whitespace
comments
identifier rules
keyword rules
literal rules
operator rules
punctuation
domain literal rules
token spans
error behavior
EOF behavior

Everything else belongs downstream.

---

97. What the Lexer Must Never Know

The lexer must never need to know:

number of qubits
number of CPU cores
RAM size
GPU model
QPU model
QPU topology
native gate set
pulse duration
calibration
noise model
optimization level
scheduler policy
package registry
runtime environment
operating system
ABI

Those are compiler, semantic, runtime, or backend concerns.

---

98. POCO-REAF Lexical Requirement

For POCO-REAF:

Program Once
    ↓
same lexical meaning
    ↓
Compile Once
    ↓
target-independent semantics
    ↓
Run Everywhere
    ↓
target-specific realization

Lexical syntax must therefore remain stable across machines.

A program must not need different lexical syntax merely because it is targeting:

small machine
large machine
distributed machine
quantum computer
simulator
future computational substrate

---

99. Definition of Done

"grammar/spec/lexical.md" and its implementation are production-ready when:

- one canonical lexical specification exists;
- the Rust lexer conforms to it;
- ANTLR conforms to it;
- token duplication has been eliminated or explicitly justified;
- keyword ownership is explicit;
- contextual keywords are distinguished from reserved keywords;
- literals have deterministic boundaries;
- numeric lexing does not impose host-width limits;
- Unicode behavior is defined;
- comments are defined;
- source spans are correct;
- lexical diagnostics are structured;
- malformed input cannot hang the lexer;
- EOF behavior is deterministic;
- MTS lexical status is resolved;
- quantum literals are target-independent;
- nano annotations are target-independent;
- hardware vocabulary is not embedded in lexing;
- lexer behavior is deterministic;
- incremental lexing remains possible;
- parallel lexing remains possible;
- large sources are supported subject to resources;
- explicit resource limits are distinguishable from language validity;
- fuzzing exists;
- positive tests exist;
- negative tests exist;
- ambiguity tests exist;
- ANTLR conformance tests exist;
- compatibility tests exist;
- Rust 1.97/1.97.1 compatibility is maintained;
- compiler implementation contains no Rust "unsafe".

---

100. Required Repository Integration

The production implementation should converge toward:

grammar/
├── README.md
├── DESIGN.md
├── spec/
│   ├── lexical.md          ← this specification
│   ├── syntax.md
│   ├── semantics.md
│   ├── type-system.md
│   ├── effects.md
│   ├── quantum.md
│   ├── modules.md
│   ├── compatibility.md
│   └── conformance.md
│
├── antlr/
│   ├── ZamaniLexer.g4
│   └── ZamaniParser.g4
│
├── Zamani.g4              ← compatibility/legacy entry point as appropriate
├── grammar.md              ← implementation-conformance snapshot
├── Zamani-Grammar.md       ← broader design/specification history
│
└── tests/
    ├── lexical/
    ├── valid/
    ├── invalid/
    ├── quantum/
    ├── mathematics/
    ├── types/
    ├── effects/
    ├── scaling/
    └── compatibility/

The exact migration should be performed incrementally so that existing compiler functionality is not broken unnecessarily.

---

101. Final Lexical Architecture

The production lexical architecture is:

                       ZAMANI SOURCE
                            │
                            ▼
                    UTF-8 Source Input
                            │
                            ▼
                     Source Validation
                            │
                            ▼
                    Comment / Whitespace
                       Recognition
                            │
                            ▼
                    Longest-Match Lexer
                            │
            ┌───────────────┼────────────────┐
            ▼               ▼                ▼
       Identifiers       Literals        Operators
            │               │                │
            └───────────────┼────────────────┘
                            ▼
                    Canonical Tokens
                            │
                            ▼
                     Source Spans
                            │
                            ▼
                 Lexical Diagnostics
                            │
                            ▼
                         Parser
                            │
                            ▼
                           AST
                            │
                            ▼
                   Semantic Analysis
                            │
                            ▼
                     Canonical IR
                            │
                ┌───────────┴───────────┐
                ▼                       ▼
          Classical IR             Quantum IR
                │                       │
                └───────────┬───────────┘
                            ▼
                    Target-independent
                       compilation
                            │
                            ▼
                 Target-specific lowering
                            │
                            ▼
             Any supported computational target

---

102. Governing Principle

The Zamani lexer must remain small, deterministic, compositional, target-independent, resource-aware, and mechanically verifiable.

It should recognize the language without attempting to become the language.

The fundamental rule is:

«Lex source syntax once. Preserve its meaning. Defer semantics, resources, optimization, quantum realization, and hardware decisions to the appropriate downstream layers.»

That is the lexical foundation required for Zamani to scale:

from atom → everywhere,

while preserving:

Program Once → Compile Once → Run Everywhere → Anywhere → Forever.