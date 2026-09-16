Zamani Interpolation Lexical Specification

File: "grammar/lexer/interpolation.md"
Status: Normative
Authority: Detailed lexical interpolation contract
Language: Zamani
Target implementation: Rust 1.97 / Rust 1.97.1
Safety: "unsafe" forbidden
Scalability: No language-imposed finite limit on interpolation count, nesting depth, expression size, string size, or source size
Primary integration: "lexer/literals.md", "lexer/tokens.md", "lexer/unicode.md", "lexer/identifiers.md", "lexer/comments.md", "lexer/diagnostics.md", "lexer/conformance.md"

---

1. Purpose

This document defines the production lexical contract for string interpolation in Zamani.

Interpolation allows literal text and Zamani expressions to coexist in one source construct without turning interpolation into a separate language.

Conceptually:

literal text
      +
Zamani expression
      +
literal text
      +
Zamani expression
      +
...

Interpolation is therefore a lexical/syntactic bridge between:

- string literals;
- expressions;
- identifiers;
- formatting/specification metadata where supported;
- compile-time and runtime expression evaluation;
- AST construction;
- semantic analysis;
- canonical IR;
- diagnostics;
- tooling.

This file defines how interpolation boundaries are recognized and represented lexically.

It does not define the semantic meaning of the embedded expression, formatting behavior, evaluation order, or runtime string construction.

---

2. Architectural position

The interpolation pipeline is:

source bytes
    │
    ▼
UTF-8 validation
    │
    ▼
Unicode-aware lexical scanning
    │
    ▼
string/interpolation tokenization
    │
    ▼
parser
    │
    ▼
AST
    │
    ▼
structural validation
    │
    ▼
semantic analysis
    │
    ▼
canonical semantic model / IR
    │
    ▼
compiler
    │
    ▼
runtime/backend

Interpolation must not create a second expression language.

The expression inside an interpolation must eventually use the same expression grammar and semantic system as an ordinary Zamani expression.

Therefore:

"${x + y}"

and:

x + y

must ultimately refer to the same expression semantics.

The difference is only their syntactic context.

---

3. Authority hierarchy

Interpolation follows the repository-wide authority model.

The authority order is:

grammar/DESIGN.md
        │
        ▼
specification/lexical.md
        │
        ▼
lexer/interpolation.md
        │
        ├── lexer/tokens.md
        ├── lexer/literals.md
        ├── lexer/unicode.md
        └── lexer/identifiers.md
        │
        ▼
grammar/Zamani.g4
        │
        ▼
src/lexer.rs
        │
        ▼
src/parser.rs
        │
        ▼
src/frontend/ast/
        │
        ▼
semantic model
        │
        ▼
canonical IR

"Zamani-Grammar.md" may describe proposed or historical interpolation syntax, but cannot silently override this contract.

"grammar/grammar.md" reports implementation conformance and is not an independent syntax authority.

---

4. Scope

This file owns:

- interpolation boundaries;
- interpolation opening/closing delimiters;
- lexical states required to scan interpolated strings;
- transitions between literal text and expression mode;
- escaped interpolation delimiters;
- nested delimiter handling where applicable;
- interaction with strings;
- interaction with Unicode;
- tokenization requirements;
- malformed interpolation diagnostics;
- interpolation source spans;
- lexer state-machine requirements;
- scalability requirements;
- deterministic behavior;
- ANTLR integration requirements;
- Rust implementation requirements;
- conformance requirements.

---

5. This file does not own

This file does not own:

- general string-literal syntax;
- character-literal syntax;
- numeric-literal syntax;
- identifier grammar;
- expression precedence;
- expression semantics;
- type checking;
- formatting semantics;
- runtime string allocation;
- localization;
- Unicode normalization;
- grapheme segmentation;
- display width;
- compiler optimization;
- memory-management policy;
- target hardware;
- quantum semantics;
- QEC;
- ZQN;
- HAL;
- routing;
- scheduling;
- physical resource limits.

Those responsibilities remain with their respective contracts.

---

6. Core design principle

Interpolation is a context-sensitive lexical mode, not a collection of special expression keywords.

The lexer must conceptually support:

NORMAL
STRING
INTERPOLATION_EXPRESSION

with transitions:

NORMAL
  │
  │ opening string delimiter
  ▼
STRING
  │
  │ interpolation opener
  ▼
INTERPOLATION_EXPRESSION
  │
  │ matching interpolation closer
  ▼
STRING
  │
  │ closing string delimiter
  ▼
NORMAL

Nested expressions may themselves contain:

- parentheses;
- brackets;
- braces;
- blocks;
- function calls;
- indexing;
- arrays;
- maps;
- lambdas;
- match expressions;
- nested strings;
- nested interpolated strings;
- macros;
- domain-specific expressions.

The lexer must therefore not terminate interpolation merely because it encounters the first closing brace-like character.

---

7. Interpolation must reuse the Zamani expression grammar

The embedded portion is not a reduced expression language.

For example, if the normal expression grammar permits:

a + b * c

the interpolation expression must use the same grammar.

Likewise, if Zamani permits:

foo.bar(x)[i]

inside ordinary expressions, interpolation must not invent a separate restricted syntax.

Conceptually:

interpolated-expression
    → expression

not:

interpolated-expression
    → special-small-expression

This prevents two different meanings for the same expression syntax.

---

8. Canonical conceptual form

The lexical structure is conceptually:

interpolated-string
    ::= string-start interpolation-part* string-end

interpolation-part
    ::= literal-text
      | interpolation

interpolation
    ::= interpolation-start expression interpolation-end

The exact delimiter spelling is owned by the canonical literal/token specification and must not be duplicated independently in multiple files.

This document therefore specifies the behavior of interpolation delimiters without creating a second delimiter registry.

---

9. Recommended canonical delimiter model

Where the language specification adopts brace-based interpolation, the conceptual form is:

"${ expression }"

For example:

"hello ${name}"

and:

"result = ${x + y}"

The exact accepted spelling must be synchronized with:

lexer/literals.md
lexer/tokens.md
Zamani.g4
src/lexer.rs
src/parser.rs

There must never be a situation where:

literals.md

claims one delimiter while:

src/lexer.rs

recognizes another.

If another interpolation spelling is retained for compatibility, it must be explicitly versioned and classified as stable, experimental, deprecated, or historical.

---

10. Literal text

Literal text between interpolation boundaries remains part of the enclosing string.

For:

"Hello ${name}!"

the conceptual structure is:

STRING_START
TEXT("Hello ")
INTERPOLATION_START
EXPRESSION(name)
INTERPOLATION_END
TEXT("!")
STRING_END

The lexer/parser must preserve the source text required for:

- diagnostics;
- source mapping;
- tooling;
- formatting;
- round-tripping where required.

---

11. Empty literal segments

An interpolation may occur at the beginning or end of a string.

For example:

"${value}"

must not require artificial text before or after the interpolation.

Conceptually:

STRING_START
INTERPOLATION_START
EXPRESSION(value)
INTERPOLATION_END
STRING_END

Empty text segments must not create meaningless semantic string values unless the AST contract explicitly requires them.

---

12. Multiple interpolations

An interpolated string may contain any number of interpolation segments subject only to available resources.

Example:

"${a} ${b} ${c}"

There must be no language-level rule such as:

MAX_INTERPOLATIONS = 32

or:

MAX_INTERPOLATION_SEGMENTS = 1024

Any implementation resource budget must be external to the language grammar.

---

13. No fixed string-size limit

The grammar must not impose:

MAX_STRING_LENGTH
MAX_INTERPOLATED_STRING_LENGTH
MAX_INTERPOLATION_BYTES
MAX_INTERPOLATION_COUNT

A program may contain arbitrarily large strings and arbitrarily many interpolation segments, limited only by:

- available memory;
- storage;
- compiler resource policy;
- runtime resource policy;
- operating-system limits;
- target capabilities.

Such implementation limits must never become language semantics.

---

14. Escaping interpolation delimiters

Literal text must provide a deterministic way to represent an interpolation opener literally.

For example, if:

${...}

starts interpolation, a literal:

${name}

must be expressible without accidentally entering interpolation mode.

The exact escape spelling belongs to "lexer/literals.md".

This file requires that:

1. the escape be unambiguous;
2. the lexer recognize it before interpolation-start recognition;
3. escaped interpolation markers remain literal data;
4. escaping not depend on locale;
5. escaping not depend on target hardware;
6. escaping be deterministic.

The lexer must not guess whether the programmer intended interpolation.

---

15. Escape processing order

The scanner must distinguish:

escaped delimiter

from:

actual interpolation delimiter

before entering interpolation mode.

For example, conceptually:

literal escape
      │
      ├── literal `${`
      │
      ▼
continue STRING mode

whereas:

actual interpolation opener
      │
      ▼
enter INTERPOLATION_EXPRESSION mode

The two cases must never be ambiguous.

---

16. Nested delimiters

The most important lexer requirement is correct delimiter balancing.

Consider:

"${foo({a: 1})}"

The interpolation must not terminate at the first "}".

The scanner must understand the delimiter nesting introduced by the expression.

Conceptually:

INTERPOLATION_START
    expression
        (
            {
                ...
            }
        )
INTERPOLATION_END

Only the closing interpolation delimiter belonging to the interpolation's own nesting level may terminate interpolation mode.

---

17. Nested strings inside interpolation

Interpolation expressions may contain ordinary strings.

For example:

"${format("value")}"

The lexer must not interpret the """ inside the expression as the end of the outer interpolated string.

It must track lexical modes independently.

Conceptually:

OUTER_STRING
    INTERPOLATION
        EXPRESSION
            INNER_STRING
        EXPRESSION
    END_INTERPOLATION
OUTER_STRING

---

18. Nested interpolated strings

If Zamani's canonical expression/literal grammar permits nested interpolated strings, they must be handled recursively.

Example conceptually:

"${"${value}"}"

Whether a particular spelling is accepted is controlled by the canonical string grammar.

If nested interpolation is accepted, the implementation must use balanced lexical state rather than ad hoc delimiter searching.

There must be no arbitrary nesting limit in the grammar.

---

19. Brace-like syntax inside interpolation

The interpolation expression may contain constructs using braces, including:

- blocks;
- maps;
- struct literals;
- patterns;
- attributes;
- macros;
- domain constructs.

Therefore the implementation must distinguish:

expression brace

from:

interpolation terminator

using lexical/parser context.

A raw search for the next "}" is not production-safe.

---

20. Parentheses and brackets

The interpolation scanner must correctly account for:

(...)
[...]
{...}

when those delimiters are part of the embedded expression.

The implementation must not assume that the interpolation expression is a single identifier or simple arithmetic expression.

---

21. Comments inside interpolation

If comments are legal inside expressions, their handling must use the normal comment rules.

For example:

"${foo /* comment */ + bar}"

or line comments where permitted.

The interpolation scanner must not interpret delimiter characters contained inside comments as interpolation terminators.

For example, a "}" inside a comment must not accidentally close the interpolation.

"lexer/comments.md" remains authoritative for comment syntax.

---

22. Strings inside interpolation

Likewise, delimiter characters inside nested string literals are literal string contents.

For example:

"${map["key}"]}"

must not close the interpolation at the "}" inside ""key}"".

The lexer must therefore correctly transition between:

INTERPOLATION_EXPRESSION

and:

STRING

states.

---

23. Character literals inside interpolation

If character literals are valid expressions, they must be scanned according to "lexer/literals.md".

A delimiter inside a character literal is data, not an interpolation terminator.

For example, conceptually:

"${lookup('}')}"

must not terminate early.

---

24. Unicode

Interpolation is Unicode-aware but does not redefine Unicode.

"lexer/unicode.md" is authoritative for:

- UTF-8;
- Unicode scalar values;
- source encoding;
- Unicode escapes;
- Unicode source spans;
- BOM handling;
- normalization policy;
- control characters;
- bidirectional controls;
- zero-width characters;
- noncharacters;
- Unicode whitespace.

Interpolation must preserve the exact Unicode semantics defined there.

---

25. Unicode normalization

The lexer must not globally normalize interpolated strings.

For example, literal text containing canonically equivalent sequences must not silently be rewritten.

Likewise:

"é"

and:

"e\u{301}"

must preserve their source representation unless a separate semantic operation explicitly requests normalization.

Interpolation does not change this rule.

---

26. Unicode inside expressions

The expression side may contain Unicode identifiers where permitted by "lexer/identifiers.md".

For example, a valid identifier policy may permit:

"${π * r²}"

or other Unicode identifiers.

The interpolation lexer must not maintain a second identifier policy.

---

27. Unicode interpolation delimiters

If Zamani supports Unicode punctuation as interpolation syntax, it must be explicitly registered in:

lexer/tokens.md
lexer/operators.md
lexer/unicode.md

and composed through the canonical grammar.

Visually similar Unicode characters must never become implicit aliases.

For example, the lexer must not treat arbitrary confusable brace-like characters as "{" or "}".

---

28. Confusables

Unicode confusables must not alter interpolation semantics.

A visually similar character is not automatically equivalent to an interpolation delimiter.

For example:

${x}

must not be equivalent to a visually confusable sequence merely because it looks similar.

Security-oriented confusable detection belongs to tooling/validation.

The lexer may emit a diagnostic where repository security policy requires it, but must not silently rewrite source.

---

29. Bidirectional controls

Bidirectional formatting controls require special handling because they can make source visually misleading.

Rules:

- interpolation delimiters remain determined by actual code points;
- bidi controls must never change lexical nesting;
- identifiers follow "identifiers.md";
- string contents may contain valid Unicode data;
- comments may contain such data subject to comment diagnostics;
- security tooling should surface suspicious bidi usage;
- source must never be reordered by the lexer.

The lexer must process logical source order, not visual rendering order.

---

30. Zero-width characters

Zero-width characters must not silently create or remove interpolation boundaries.

For example, a zero-width character inserted near a delimiter must remain a distinct code point unless explicitly prohibited by the Unicode contract.

The lexer must not perform visual matching.

---

31. Grapheme clusters

Interpolation operates on source code points/bytes, not user-perceived grapheme clusters.

For example:

👨‍👩‍👧‍👦

may contain multiple Unicode scalar values.

The lexer must not assume that one visible glyph corresponds to one Unicode scalar value.

Grapheme segmentation is a tooling/display concern.

---

32. Byte-oriented input

The canonical source representation is UTF-8.

A byte-oriented lexer entry point must:

1. validate UTF-8;
2. reject malformed UTF-8;
3. report the byte position of the error;
4. never reinterpret invalid bytes as interpolation syntax.

No lossy conversion such as:

invalid UTF-8 → replacement character

may occur silently.

---

33. Source spans

Interpolation tokens and AST nodes must use the repository-wide source-span contract.

The canonical span model is:

start: byte offset
end: byte offset

with:

start <= end

and:

end

exclusive.

A span must never split a UTF-8 code point.

For interpolation:

STRING_START
TEXT
INTERPOLATION_START
EXPRESSION
INTERPOLATION_END
STRING_END

must each be capable of being mapped back to source ranges as required by the AST/diagnostic contract.

---

34. Line and column tracking

Interpolation must preserve normal source line tracking.

Embedded expressions may contain newlines when the expression grammar permits them.

Line/column information must therefore continue across:

STRING
→ INTERPOLATION
→ EXPRESSION
→ STRING

without resetting the source position.

CRLF, CR, and LF behavior follows "lexer/unicode.md" and "spec/source-spans.md".

---

35. Interpolation and comments

Comment recognition must remain independent.

The lexer must never interpret interpolation syntax appearing inside a comment as executable interpolation.

For example:

// "${not_an_expression}"

contains no interpolation expression.

Likewise block comments containing interpolation-looking sequences remain comments.

---

36. Interpolation and raw strings

If Zamani provides raw-string literals, interpolation must be explicitly defined for them.

The default production rule should be:

«A raw string does not perform interpolation unless the raw-string specification explicitly declares an interpolation form.»

This prevents accidental interpretation of arbitrary source text.

For example, a raw string containing:

${x}

must remain literal if the raw-string contract says interpolation is disabled.

---

37. Interpolation and escaped strings

If ordinary string escaping is active, escape processing must be deterministic.

The lexer must distinguish:

escaped character

from:

interpolation opener

according to the literal grammar.

There must be no target-dependent or implementation-dependent interpretation.

---

38. Interpolation and byte strings

Byte strings are byte-oriented.

Interpolation must not silently convert Unicode text into bytes.

If interpolation into byte strings is supported, it requires an explicit semantic conversion contract.

Otherwise:

byte-string interpolation

must be rejected structurally.

The lexer must not guess an encoding.

---

39. Interpolation and character literals

A character literal is a scalar-value construct, not an arbitrary string.

Interpolation does not change character-literal semantics.

Unicode validity and escape validation remain owned by "lexer/unicode.md" and "lexer/literals.md".

---

40. Interpolation and numeric literals

Expressions embedded in interpolation use the ordinary numeric literal grammar.

For example:

"${1_000 + 2_000}"

must use the same numeric lexical rules as:

1_000 + 2_000

There must be no separate interpolation-specific numeric grammar.

---

41. Interpolation and identifiers

Embedded identifiers use the normal identifier contract.

For example:

"${user.name}"

must produce the same identifier/member-access semantics as the corresponding ordinary expression.

No special interpolation identifier table is permitted.

---

42. Interpolation and operators

Operators inside interpolation use "lexer/operators.md".

The lexer must not introduce special interpolation versions of ordinary operators.

For example:

"${a + b}"

uses the normal "+" token.

This guarantees:

ordinary expression

and:

interpolated expression

remain semantically aligned.

---

43. Expression termination

The interpolation terminator is recognized only when the embedded expression has returned to the interpolation's delimiter nesting level.

Conceptually:

depth = 0

while scanning interpolation:
    nested opening delimiter:
        depth += 1

    matching nested closing delimiter:
        depth -= 1

    interpolation closing delimiter:
        terminate only when depth == 0

The actual implementation must account for strings, comments, escaped characters, and other lexical states.

A naive character search is prohibited.

---

44. Lexer state requirements

A production implementation should maintain explicit lexical state.

At minimum:

enum LexerMode {
    Normal,
    String,
    InterpolationExpression,
}

The actual implementation may use a richer internal state machine.

The state must contain enough information to distinguish:

- current mode;
- interpolation nesting;
- delimiter nesting;
- source position;
- string/raw-string mode;
- escape state;
- comment state where required.

No state may rely on global mutable process state.

---

45. Reentrancy

The lexer must not use global mutable interpolation state.

Two independent lexer instances must be able to process separate source units simultaneously without affecting one another.

This is required for:

- parallel compilation;
- IDE tooling;
- language servers;
- incremental compilation;
- test isolation;
- deterministic builds.

---

46. Determinism

Given identical:

source bytes
+
language version
+
lexical profile

the lexer must produce the same token sequence and source spans.

Interpolation behavior must not depend on:

- locale;
- operating system;
- host filesystem;
- environment variables;
- CPU architecture;
- GPU;
- QPU;
- compiler scheduling;
- machine topology.

---

47. No target dependence

Interpolation is entirely target-independent.

The lexer must not inspect:

CPU count
GPU count
QPU count
FPGA count
RAM
VRAM
qubit count
network nodes
storage size

to determine whether interpolation is legal.

---

48. No fixed nesting limit in the language

The language must not define:

MAX_INTERPOLATION_DEPTH
MAX_EXPRESSION_DEPTH
MAX_STRING_SEGMENTS
MAX_NESTED_STRINGS

as semantic language limits.

An implementation may expose configurable resource budgets to prevent denial-of-service conditions.

Such a budget must produce a resource-limit diagnostic rather than pretending that the source is grammatically invalid.

---

49. Resource exhaustion

Extremely large interpolation constructs may exhaust compiler resources.

The implementation must handle this safely.

It must:

- return a structured diagnostic;
- avoid stack overflow where practical;
- avoid unbounded accidental recursion;
- avoid quadratic rescanning;
- avoid uncontrolled allocation;
- avoid process aborts;
- avoid undefined behavior;
- never use "unsafe".

Resource limits must be configurable outside the language grammar.

---

50. Rust safety requirements

The implementation target is:

Rust 1.97
Rust 1.97.1

The lexer must use safe Rust only.

A lexer crate/module should enforce:

#![forbid(unsafe_code)]

where compatible with the repository's crate structure.

No:

unsafe

is permitted.

---

51. Safe UTF-8 handling

The preferred implementation model is to operate on Rust "&str".

Safe APIs include:

str::chars()
str::char_indices()
str::is_char_boundary()

and equivalent safe standard-library operations.

If raw bytes are accepted:

std::str::from_utf8(...)

must be used to validate them.

Invalid UTF-8 must produce a structured lexer error.

---

52. UTF-8 slicing

Because Rust string indices are byte offsets, the implementation must never slice a "&str" at an arbitrary byte offset.

Invalid:

source[start..end]

unless both offsets are known UTF-8 boundaries.

The lexer may use byte offsets for source spans, but every string slice must respect UTF-8 boundaries.

---

53. Error handling

Malformed interpolation is user input, not an internal invariant violation.

Therefore the lexer must not use:

unwrap()
expect()
panic!()
unreachable!()

as the normal response to malformed interpolation input.

Instead it must return structured diagnostics.

Examples:

unterminated string
unterminated interpolation
unexpected interpolation terminator
invalid escape
invalid Unicode escape
invalid UTF-8
invalid nested lexical construct
resource limit exceeded

Exact diagnostic identifiers are owned by "lexer/diagnostics.md".

---

54. Diagnostics

Diagnostics must provide enough information to identify:

- source location;
- interpolation boundary;
- expected construct;
- relevant delimiter;
- recovery point where possible.

Example conceptual diagnostic:

error: unterminated interpolation
  --> example.zm:12:18
   |
12 | "result = ${compute(x)"
   |            ^ expected interpolation terminator

The exact diagnostic wording belongs to "lexer/diagnostics.md".

---

55. Recovery

The lexer should recover from malformed interpolation where practical so tooling can continue parsing later source.

Recovery must not silently turn malformed source into valid source.

For example, an IDE may receive:

"hello ${name
next_statement();

The lexer should produce a diagnostic and a deterministic recovery state.

Recovery behavior must be documented and tested.

---

56. Error recovery must not change valid programs

Error recovery exists only for invalid/incomplete input.

For valid source, recovery logic must never alter tokenization.

This is especially important for:

- IDEs;
- incremental parsing;
- code completion;
- syntax highlighting;
- language servers.

---

57. AST contract

The lexer must expose sufficient token information for the parser to construct an interpolation AST.

A preferred conceptual AST representation is:

InterpolatedString
    parts:
        Text(...)
        Expression(...)
        Text(...)
        Expression(...)

The exact AST type is owned by:

src/frontend/ast/

The lexer must not invent a second AST.

---

58. Generic expression AST

The expression part must map to the normal generic expression AST.

Do not create:

InterpolatedAddExpression
InterpolatedCallExpression
InterpolatedIdentifierExpression

merely because those expressions occur inside strings.

Instead:

InterpolatedString
    └── Expression
          └── ordinary expression AST

This preserves AST uniformity.

---

59. Semantic contract

The semantic layer determines:

- expression typing;
- expression evaluation;
- conversion to textual representation;
- formatting;
- effect behavior;
- ownership;
- resource requirements;
- compile-time/runtime evaluation;
- side-effect rules.

The lexer must not evaluate interpolated expressions.

For example:

"${compute()}"

must never call "compute()" during lexical analysis.

---

60. Evaluation order

Evaluation order is a semantic concern.

This file therefore does not establish whether:

"${a}${b}${c}"

evaluates:

a → b → c

or follows another explicitly specified evaluation model.

That rule belongs to:

specification/semantics.md

and the relevant expression/evaluation contract.

The lexer merely preserves source order.

---

61. Side effects

Interpolation must not implicitly grant additional effects.

If the embedded expression is effectful, the normal Zamani effect system applies.

For example:

"${read_file(path)}"

does not become effect-free merely because it occurs inside a string.

---

62. Compile-time interpolation

If Zamani supports compile-time evaluation/interpolation, it must be explicitly represented through the compile-time semantic system.

The lexer must not decide whether interpolation is:

compile-time
runtime
constant
symbolic
deferred

Those are semantic/compiler decisions.

---

63. Symbolic interpolation

Interpolation may eventually support symbolic values.

For example:

"${tensor_dimension}"

may remain symbolic until specialization.

The lexer must preserve the expression without requiring the value to be known.

This supports POCO-REAF.

---

64. Quantum integration

Interpolation itself has no quantum semantics.

An embedded expression may nevertheless refer to quantum constructs where the semantic language permits it.

For example, a program might construct diagnostic text from a measurement result.

The lexical system must not:

- enumerate quantum gates;
- impose qubit limits;
- map physical qubits;
- perform QEC;
- perform routing;
- perform scheduling;
- access HAL state.

Any quantum expression ultimately follows the canonical:

frontend
    ↓
semantic quantum model
    ↓
quantum::ir

boundary.

---

65. HDL integration

Interpolation may be useful in generated names, diagnostics, metadata, or code-generation contexts.

However, interpolation must not itself become an HDL construct.

HDL semantic interpretation remains owned by:

grammar/hdl/

and downstream HDL semantic/IR systems.

---

66. AI/data integration

Interpolation may embed expressions referring to:

- tensors;
- models;
- datasets;
- agents;
- symbolic values;
- distributed objects.

The lexer treats them uniformly as expressions.

Framework-specific behavior belongs downstream.

No AI framework-specific interpolation keywords should be added to the lexer.

---

67. Hardware/resource integration

Interpolation does not select hardware.

For example:

"using ${available_resource}"

is source-level expression evaluation.

It must not cause the lexer to inspect machine resources.

Hardware capability selection remains owned by:

hardware/
resources/
compile/
execution/

---

68. POCO-REAF requirements

Interpolation must preserve:

«Program Once → Compile Once → Run Everywhere, Anywhere, Forever.»

An interpolated program must not require rewriting merely because:

- the CPU changes;
- GPU count changes;
- QPU size changes;
- FPGA resources change;
- memory changes;
- node count changes;
- topology changes;
- accelerator architecture changes.

The expression describes portable program semantics.

Target realization happens later.

---

69. Interpolation must not encode hardware limits

Prohibited lexer/grammar constructs include:

MAX_INTERPOLATIONS
MAX_STRING_SIZE
MAX_INTERPOLATION_EXPRESSION_SIZE
MAX_FORMAT_FIELDS
MAX_NESTING
MAX_TARGETS

when used as universal language limits.

A configurable compiler resource budget is acceptable.

---

70. Difference between program constants and language limits

This is valid:

let n = 1024;

This is not a language rule:

MAX_INTERPOLATIONS = 1024

Likewise, an implementation may impose a configurable memory budget for compiling a source file, but that budget must not redefine the Zamani language.

---

71. Formatting specifications

If interpolation supports formatting metadata such as:

"${value:format}"

the exact formatting grammar must be specified separately.

Recommended ownership:

lexer/interpolation.md
    boundary detection

lexer/literals.md
    literal integration

expressions/
    expression

types/
    type information

specification/formatting.md
    formatting semantics

If no dedicated formatting specification exists yet, formatting should not be silently invented inside this file.

---

72. Dynamic format expressions

If the language permits an expression as formatting metadata, that expression must use the ordinary expression system.

There must not be a second mini-language unless explicitly justified and specified.

---

73. Conversion semantics

The lexer must not decide how values become text.

Potential semantic conversions include:

Display
Debug
String
Format
Serialize
Encode

The selected mechanism is a semantic/type-system/runtime concern.

---

74. Security

Interpolation can create security-sensitive behavior if interpolated strings later become:

- SQL;
- shell commands;
- HTML;
- URLs;
- network protocols;
- source code;
- HDL;
- configuration;
- cryptographic messages.

The lexer must not claim that interpolation itself makes any destination safe.

Security-sensitive escaping/encoding belongs to the relevant type/effect/security APIs.

---

75. Injection resistance

The language should provide semantic mechanisms for safe construction of structured data.

For example, a future typed API may distinguish:

SqlQuery
ShellCommand
Html
Json

from:

String

Interpolation must not erase those distinctions.

The lexer only recognizes syntax.

---

76. Macro interaction

Macros must not bypass interpolation lexical correctness.

If macros manipulate token streams, their output must still satisfy the canonical lexical contract before entering later compilation stages.

Macro expansion must not create an undocumented second interpolation syntax.

---

77. Metaprogramming interaction

Quoted syntax containing interpolation must have an explicitly defined phase model.

The lexer must not guess whether:

"${x}"

is intended for:

- immediate interpolation;
- delayed interpolation;
- generated source;
- syntax quotation.

Those distinctions belong to the metaprogramming contract.

---

78. Dialect interaction

Dialects may extend interpolation only through declared extension points.

A dialect must specify:

dialect name
version
interpolation extension
new tokens, if any
AST mapping
semantic mapping
compatibility
feature gate

A dialect must not silently redefine core interpolation syntax.

---

79. Versioning

Interpolation syntax is versioned with the language.

Changes that reinterpret existing valid interpolation source are breaking changes unless explicitly covered by a compatibility mechanism.

Examples of potentially breaking changes:

- changing the interpolation delimiter;
- changing escape meaning;
- changing delimiter nesting;
- changing whether raw strings interpolate;
- changing whether a character terminates interpolation;
- changing expression grammar in a way that changes interpolation parsing.

---

80. Backward compatibility

New interpolation features should be introduced without changing the meaning of existing valid programs.

Adding support for additional Unicode identifiers inside interpolation is generally safe when it does not reinterpret existing token sequences.

Introducing a new delimiter that was previously ordinary text may require compatibility analysis.

All such changes must be recorded in:

compatibility/versions.md
compatibility/migrations.md
compatibility/feature-gates.md

---

81. ANTLR integration

"grammar/Zamani.g4" is the composition root.

Interpolation rules should be modularized under the lexer/literal grammar architecture and composed by the root grammar.

The root grammar must not duplicate an independent interpolation grammar.

Conceptually:

Zamani.g4
    │
    ├── core
    ├── expressions
    ├── types
    ├── statements
    └── lexical/literal/interpolation rules

The exact ANTLR import/delegation mechanism must follow the repository's existing grammar composition strategy.

---

82. Lexer/parser boundary

The lexer should emit enough structure for the parser to identify interpolation boundaries.

However, if delimiter matching requires parser-level expression awareness, the architecture must ensure the lexer/parser split remains deterministic.

The implementation must not create inconsistent behavior between:

ANTLR lexer/parser

and:

Rust lexer/parser

---

83. ANTLR and Rust parity

For every accepted interpolation construct:

ANTLR

and:

Rust frontend

must agree on:

- acceptance;
- token boundaries;
- interpolation boundaries;
- source spans;
- malformed input;
- escape handling.

Any intentional difference must be documented and tested.

---

84. "src/lexer.rs" integration

"src/lexer.rs" owns executable lexical behavior.

It must implement this contract without duplicating incompatible definitions elsewhere.

The lexer must:

- validate UTF-8;
- recognize string mode;
- recognize interpolation boundaries;
- handle nested lexical constructs;
- preserve source spans;
- produce structured errors;
- remain deterministic;
- use safe Rust;
- avoid fixed language limits.

---

85. "src/parser.rs" integration

"src/parser.rs" consumes interpolation tokens and constructs the appropriate AST.

It must reuse the normal expression parser for interpolation expressions.

It must not introduce:

parse_interpolation_expression()

that implements a second restricted expression grammar unless required solely as a parser-context wrapper around the ordinary expression grammar.

---

86. AST integration

"src/frontend/ast/" should represent interpolation generically.

Preferred conceptual model:

InterpolatedString {
    parts: Vec<InterpolatedPart>
}

InterpolatedPart {
    Text(...)
    Expression(Expression)
}

The exact Rust types are owned by the AST implementation.

Source spans must be retained according to the repository source-span contract.

---

87. AST must not depend on runtime formatting

The AST must not contain:

RuntimeStringBuilder
GPUFormatter
QuantumFormatter
VendorFormatter

or equivalent backend concepts.

It should represent source meaning.

---

88. Semantic model integration

Semantic analysis resolves:

- names;
- types;
- effects;
- conversions;
- formatting;
- compile-time evaluation;
- runtime evaluation;
- ownership/resource implications.

The interpolation syntax must lower into the canonical semantic model without creating a parallel string IR.

---

89. IR integration

Interpolation must lower into the repository's canonical IR representation.

Conceptually:

InterpolatedString
        │
        ▼
semantic string construction
        │
        ▼
canonical IR

The IR must not contain lexer-specific interpolation delimiters.

The semantic result should represent the actual operation required by the program.

---

90. Quantum IR integration

If an interpolation expression contains a quantum expression whose semantics eventually enter "quantum::ir", interpolation must not create a separate quantum representation.

The architecture remains:

interpolated expression
        ↓
ordinary expression AST
        ↓
semantic analysis
        ↓
canonical semantic representation
        ↓
quantum::ir where applicable

---

91. Compiler integration

The compiler may optimize interpolation where semantics permit.

Examples include:

- constant folding;
- compile-time construction;
- concatenation elimination;
- allocation optimization;
- formatting specialization.

These optimizations must preserve semantics.

They must not alter lexical behavior.

---

92. Runtime integration

Runtime behavior is downstream.

The runtime may choose:

- stack/heap representation;
- rope/string representation;
- streaming construction;
- lazy construction;
- constant storage;
- target-specific optimization.

None of these choices may affect source syntax.

---

93. Tooling integration

Tooling should understand interpolation boundaries for:

- syntax highlighting;
- formatting;
- code completion;
- refactoring;
- rename;
- navigation;
- diagnostics;
- semantic selection;
- source maps.

The tool must distinguish literal text from embedded expressions.

---

94. Formatter requirements

A formatter must preserve the semantics of interpolation.

It may format the embedded expression according to normal expression formatting rules.

It must not:

- normalize literal content;
- change escape semantics;
- change delimiter meaning;
- introduce ambiguous delimiters;
- remove required escapes.

---

95. Syntax highlighting

Syntax highlighters should distinguish:

string text

from:

interpolation expression

and should reuse normal syntax highlighting for the embedded expression.

This avoids maintaining a second expression highlighter.

---

96. Language-server requirements

Incomplete interpolation should be recoverable.

For example:

"hello ${na

should allow code completion for "na" rather than causing the entire remainder of the document to become unusable.

The exact recovery algorithm is a tooling concern, but the lexer must provide stable source positions and deterministic recovery.

---

97. Incremental parsing

Interpolation boundaries must be stable under incremental edits.

Editing inside an interpolation expression should not unnecessarily invalidate unrelated source.

For example:

"prefix ${expression} suffix"

editing "expression" should not require reparsing unrelated files.

---

98. Performance

Lexical processing should be approximately:

O(n)

in source size for normal scanning.

The implementation must avoid repeatedly rescanning the same literal text.

Nested interpolation and delimiter tracking must not introduce accidental quadratic behavior.

---

99. Memory behavior

The lexer should avoid allocating a new heap object for every Unicode scalar or literal byte.

Where practical:

- operate over source slices;
- use indices;
- emit spans;
- defer semantic materialization;
- avoid unnecessary copying.

Large interpolated strings must be processed according to resource availability.

---

100. Streaming considerations

The lexical contract should permit future streaming/incremental implementations.

A streaming lexer must preserve exactly the same lexical semantics as a complete-source lexer.

Delimiter sequences split across input chunks must be handled correctly.

For example, if an interpolation opener spans a chunk boundary, the scanner must not misclassify it.

---

101. Chunk boundaries

No semantic rule may depend on how source bytes are chunked.

These must produce identical lexical results:

entire source in memory

and:

source delivered in arbitrary chunks

where streaming input is supported.

---

102. Infinite scalability principle

Strictly speaking, no finite computer can hold an actually infinite string.

POCO-REAF therefore means:

«Zamani imposes no artificial finite interpolation limit beyond the representational and resource limits of the executing environment.»

There must be no grammar rule restricting interpolation to a fixed maximum.

A machine with more resources may process larger valid programs without requiring a language rewrite.

---

103. Resource policy separation

If the compiler exposes:

max_source_bytes
max_token_count
max_parser_memory
max_nesting_depth

these belong to an implementation/resource policy.

They must not be embedded in:

interpolation.md
Zamani.g4

as universal language limits.

---

104. No hardware assumptions

Interpolation processing must work independently of:

CPU architecture
GPU availability
FPGA availability
QPU availability
RAM size
cache size
thread count
node count
network topology
accelerator count

The compiler may use available hardware to accelerate compilation, but that does not alter the language.

---

105. Error categories

The interpolation implementation should be able to distinguish at least:

InvalidUtf8
InvalidEscape
UnterminatedString
UnterminatedInterpolation
UnexpectedInterpolationEnd
InvalidNestedDelimiter
InvalidLiteralContext
ResourceLimitExceeded

Exact public diagnostic identifiers are owned by:

lexer/diagnostics.md

---

106. Diagnostics must be deterministic

The same malformed source must produce the same primary diagnostic independent of:

- operating system;
- locale;
- target;
- compiler parallelism;
- machine size.

Secondary diagnostics may be implementation-configured, but core lexical diagnostics must remain stable.

---

107. Security diagnostics

Tooling may additionally report:

- bidi controls;
- suspicious zero-width characters;
- confusable identifiers;
- suspicious delimiter-like Unicode characters;
- unusual escape sequences.

These warnings must not silently alter source.

---

108. Testing strategy

Interpolation requires much more than ordinary positive tests.

Every implementation must include:

positive
negative
boundary
scalability
determinism
compatibility
security
source-span
ANTLR/Rust parity

tests.

---

109. Positive tests

At minimum:

"hello"
"hello ${name}"
"${name}"
"${a}${b}"
"hello ${a} world ${b}"
"${a + b}"
"${foo.bar}"
"${foo(x)}"
"${array[i]}"

Also test all ordinary expression categories supported by Zamani.

---

110. Nested-expression tests

Test:

"${foo({a: 1})}"
"${foo([a, b, c])}"
"${foo(bar(baz(x)))}"
"${{ block }}"

where each form is legal in the ordinary expression grammar.

The important property is that the interpolation terminator is not confused with nested expression delimiters.

---

111. Nested-string tests

Test expressions containing:

"nested strings"
'characters'
raw strings
nested interpolated strings

where supported.

Delimiter characters inside those constructs must not terminate the outer interpolation.

---

112. Comment tests

Where comments are legal inside expressions, test:

"${foo(/* } */ x)}"

and equivalent block/line comment cases.

A delimiter appearing in a comment must not close interpolation.

---

113. Escape tests

Test:

- escaped interpolation opener;
- escaped quote;
- escaped backslash;
- Unicode escape;
- malformed Unicode escape;
- delimiter-looking escaped text;
- consecutive escapes.

Every accepted escape must have deterministic behavior.

---

114. Unicode tests

Test:

ASCII
Latin
Greek
Cyrillic
Arabic
Hebrew
Devanagari
CJK
Hangul
combining marks
astral Unicode
emoji
variation selectors
ZWJ sequences
private-use characters
noncharacters according to unicode.md policy

The lexer must preserve valid literal data.

---

115. Unicode normalization tests

Test that source spelling is preserved.

For example:

"é"

and:

"e\u{301}"

must not be silently normalized by the lexer.

If identifier normalization is ever adopted, it must be tested separately through "identifiers.md".

---

116. Invalid UTF-8 tests

Provide malformed byte sequences to the byte-oriented lexer entry point.

Verify:

- no panic;
- no "unsafe";
- deterministic diagnostic;
- correct byte location;
- no replacement-character substitution;
- no accidental interpolation recognition.

---

117. Unterminated interpolation tests

Examples:

"${x"
"hello ${x"
"${foo("
"${foo({"

where malformed.

The lexer must return a structured error.

---

118. Unexpected terminator tests

Test interpolation closing syntax appearing where interpolation is not active.

For example, according to the actual delimiter contract:

unexpected interpolation terminator

must not silently become ordinary syntax.

---

119. Boundary tests

Test interpolation:

- at beginning;
- at end;
- adjacent interpolations;
- empty text between interpolations;
- one-character expression;
- deeply nested expression;
- very large literal segment;
- very large expression;
- multiline expression;
- Unicode immediately before delimiter;
- Unicode immediately after delimiter.

---

120. Scalability tests

The test suite must generate interpolation sources parametrically.

Examples:

1 interpolation
10 interpolations
100 interpolations
N interpolations

where "N" is controlled by the test resource budget rather than encoded as a language limit.

Likewise test:

small expression
large expression
deeply nested expression
large Unicode literal
large source unit

The purpose is to prove that no artificial grammar ceiling exists.

---

121. Property-based tests

Property-based testing should verify:

lex(source) is deterministic

and that generated valid interpolation structures satisfy:

parse(lex(source))

without violating the lexical contract.

Malformed generated inputs must never crash the lexer.

---

122. Fuzzing

Fuzz:

- UTF-8 bytes;
- quote sequences;
- escape sequences;
- braces;
- parentheses;
- brackets;
- Unicode;
- comments;
- nested strings;
- nested interpolation;
- malformed source.

Required property:

«Arbitrary malformed input must not cause undefined behavior, memory unsafety, or uncontrolled process termination.»

---

123. ANTLR conformance tests

Every interpolation test should have expected behavior for:

Zamani.g4 / ANTLR

and:

Rust lexer/parser

The two implementations must agree.

---

124. Round-trip tests

Where source-preserving tooling is supported:

source
    ↓
lex
    ↓
parse
    ↓
AST
    ↓
format/serialize

must preserve interpolation semantics.

Literal content must not be silently normalized.

---

125. Source-span tests

Verify that spans:

- start at UTF-8 boundaries;
- end at UTF-8 boundaries;
- cover the correct interpolation opener;
- cover the correct expression;
- cover the correct interpolation closer;
- remain correct across Unicode;
- remain correct across CRLF/CR/LF;
- remain correct after nested expressions.

---

126. Compatibility tests

Every historical interpolation form retained by the repository must have an explicit test.

Each form must be labeled:

stable
experimental
deprecated
historical
unsupported

There must be no undocumented legacy behavior.

---

127. Negative tests

At minimum test:

unterminated string
unterminated interpolation
unexpected closing delimiter
invalid UTF-8
invalid escape
invalid Unicode escape
invalid nesting
invalid raw-string interpolation
invalid byte-string interpolation

according to the final literal contract.

---

128. Security tests

Test that:

- bidi controls cannot alter delimiter interpretation;
- confusables do not become aliases;
- zero-width characters cannot disappear;
- escaped delimiters remain literal;
- comments cannot accidentally activate interpolation;
- nested strings cannot accidentally terminate interpolation.

---

129. Hard-coding audit

This file passes its hard-coding audit only if it contains no implementation-imposed universal limits such as:

MAX_INTERPOLATIONS
MAX_STRING_LENGTH
MAX_EXPRESSION_LENGTH
MAX_NESTING
MAX_UNICODE
MAX_CODEPOINTS

as language restrictions.

Test fixtures may use finite values.

Those finite values are test parameters, not language limits.

---

130. Dependency policy

The interpolation lexer should prefer the Rust standard library for:

- UTF-8 handling;
- character iteration;
- source indexing;
- basic scanning.

Do not add a dependency merely to implement ordinary interpolation scanning.

If Unicode normalization, grapheme segmentation, or Unicode security data is eventually required, it must be introduced through the appropriate Unicode/identifier/security contract rather than hidden inside interpolation.

---

131. No locale dependence

Interpolation must never depend on:

LC_ALL
LANG
system locale
OS language
keyboard layout

String and expression tokenization are language-defined.

---

132. No filesystem dependence

The lexer must not read external files to decide whether an interpolation is legal.

Imports, modules, symbols, and semantic resolution happen later.

---

133. No network dependence

Interpolation tokenization must not depend on network services.

This guarantees deterministic offline compilation.

---

134. No runtime dependency

The lexer must not execute runtime functions while scanning interpolation.

This is a strict phase boundary:

lexing ≠ evaluation

---

135. No semantic lookup during lexing

The lexer must not ask:

Does this identifier exist?
Is this variable a string?
Is this value quantum?
Is this resource available?
Which GPU exists?
Which QPU exists?

Those are semantic/compiler/runtime questions.

---

136. Interpolation as syntax, not string concatenation syntax

The parser should not necessarily lower interpolation directly into a chain of ordinary concatenation operators.

The semantic model should preserve the fact that the source requested interpolation where that distinction matters.

For example:

InterpolatedString

may later lower to an efficient formatting/string-construction operation.

---

137. Constant folding

A compiler may fold:

"${1 + 2}"

to its constant result when language semantics permit.

This is an optimization, not a lexical transformation.

The source AST remains faithful to the program.

---

138. No accidental evaluation during diagnostics

Diagnostic tooling may display interpolated source, but must never evaluate embedded expressions simply to display them.

For example:

"${dangerous_operation()}"

must never execute because an IDE displays the source.

---

139. Reproducible compilation

Interpolation compilation must be reproducible.

If compile-time interpolation exists, its inputs must be explicitly controlled by the semantic/build system.

The lexer itself must remain pure with respect to source input.

---

140. Provenance

If the semantic/compiler pipeline records provenance, interpolation parts should retain source provenance through AST and lowering.

This allows:

runtime/generated value
        ↓
source interpolation
        ↓
source span

to remain traceable.

---

141. Diagnostics and source ownership

An error inside:

"${foo.bar}"

should point into the expression's source span rather than merely reporting an error against the enclosing string.

This requires correct lexical boundaries.

---

142. Cross-domain integration

Interpolation must remain domain-neutral.

It may appear in programs involving:

classical
quantum
hybrid
HDL
hardware
AI
data
distributed
networking
security
embedded
scientific
HPC
accelerators

without requiring domain-specific interpolation syntax.

---

143. Quantum example

A semantic system may eventually allow something conceptually like:

"measurement = ${measurement_result}"

The interpolation lexer only sees:

string
+
expression

It does not need to know whether "measurement_result" came from a QPU, simulator, or classical computation.

---

144. Hardware example

Likewise:

"selected capability = ${capability}"

does not cause lexical hardware discovery.

The value is resolved downstream.

---

145. Distributed example

A distributed program may construct:

"node ${node_id}"

without the grammar imposing any maximum number of nodes.

The node count remains a resource/deployment concern.

---

146. AI example

An AI program may construct:

"loss = ${loss}"

without making "loss" a special lexer token.

It is an ordinary expression.

---

147. HDL example

Generated HDL or hardware diagnostics may use interpolation where the language permits it.

The lexer must remain oblivious to the target HDL backend.

---

148. Embedded/resource-constrained targets

Small targets must not require a different interpolation language.

The same source language should remain valid.

A compiler may select a target-specific implementation strategy based on available resources while preserving semantics.

---

149. Large-scale targets

Large machines must be able to compile programs containing:

- huge strings;
- many interpolation segments;
- large expressions;
- generated source;
- distributed metadata.

No grammar rule should artificially prevent scaling.

---

150. Future Unicode versions

Interpolation delimiters and source characters must remain compatibility-managed as Unicode evolves.

New Unicode characters must not automatically become Zamani syntax merely because Unicode adds them.

Conversely, adding Unicode support must not require a fixed list of scripts.

The Unicode policy remains versioned and deterministic.

---

151. Future interpolation extensions

Potential future extensions may include:

- formatting;
- typed interpolation;
- compile-time interpolation;
- raw interpolation;
- structured interpolation;
- localization-aware formatting;
- serialization interpolation;
- domain-specific interpolation profiles.

Each extension must define:

syntax
tokens
AST
semantics
IR
compiler behavior
runtime behavior
diagnostics
security
compatibility
tests
hard-coding policy

before becoming stable.

---

152. Feature manifest integration

Interpolation should have a feature manifest under:

grammar/specification/features/

for example:

features/interpolation.yaml

The manifest should record:

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

The exact filename must follow the repository's feature-manifest naming convention if one already exists.

---

153. Required adjacent-file contracts

This file is independently complete, but integration must be explicit.

"lexer/tokens.md"

Must define the canonical token identities used for:

- string start;
- string end;
- interpolation start;
- interpolation end;
- text segments if represented lexically.

It must not create duplicate token names for equivalent concepts.

---

"lexer/literals.md"

Owns:

- string-literal forms;
- exact delimiter spellings;
- escape syntax;
- raw strings;
- byte strings;
- literal decoding rules.

It must reference this document for interpolation behavior.

---

"lexer/unicode.md"

Owns:

- UTF-8;
- Unicode scalar values;
- Unicode escapes;
- source spans;
- Unicode security rules.

Interpolation must obey it.

---

"lexer/identifiers.md"

Owns identifier character classes and identifier equality.

Embedded expressions use it unchanged.

---

"lexer/comments.md"

Owns comment recognition.

Interpolation must not activate inside comments.

---

"lexer/diagnostics.md"

Owns stable public diagnostic identifiers and formatting conventions.

This file defines interpolation error conditions that diagnostics.md must encode.

---

"lexer/conformance.md"

Must contain the interpolation conformance matrix.

---

"specification/lexical.md"

Must reference this file as the detailed interpolation contract.

---

"specification/syntax.md"

Must specify how interpolated-string syntax enters the complete language grammar.

---

"specification/semantics.md"

Must define expression evaluation and interpolation semantics.

---

"spec/source-spans.md"

Must remain compatible with the byte-offset span contract.

---

"spec/determinism.md"

Must guarantee deterministic interpolation tokenization.

---

"spec/compatibility.md"

Must govern changes to interpolation syntax.

---

"validation/hard-coding.md"

Must verify that no artificial interpolation limits have been introduced.

---

"validation/scalability.md"

Must verify scaling with source size, interpolation count, nesting, and Unicode content.

---

"grammar/Zamani.g4"

Must compose the canonical interpolation rules and must not duplicate conflicting interpolation syntax.

---

"src/lexer.rs"

Must implement this lexical contract in safe Rust 1.97/1.97.1.

---

"src/parser.rs"

Must reuse the normal expression grammar for interpolation expressions.

---

"src/frontend/ast/"

Must represent interpolation without creating a second expression AST.

---

Semantic model / IR

Must lower interpolation into canonical semantic/IR constructs rather than preserving lexer-specific delimiter concepts.

---

154. Integration invariants

The following invariants are mandatory:

One interpolation syntax
        ↓
One lexical contract
        ↓
One expression grammar
        ↓
One expression AST
        ↓
One semantic model
        ↓
One canonical IR

There must not be:

ANTLR interpolation syntax
        ≠
Rust interpolation syntax

or:

interpolation expression AST
        ≠
ordinary expression AST

or:

Zamani interpolation
        ≠
domain-specific interpolation languages

unless an explicitly versioned dialect says so.

---

155. Required invariants for production readiness

The implementation is not complete unless:

- interpolation boundaries are unambiguous;
- nested expressions are handled;
- nested strings are handled;
- comments are handled;
- escapes are handled;
- Unicode is handled;
- source spans are correct;
- malformed input produces diagnostics;
- valid input is deterministic;
- ANTLR and Rust implementations agree;
- AST mapping is defined;
- semantic mapping is defined;
- IR mapping is defined;
- compiler integration is defined;
- runtime integration is defined;
- tooling integration is defined;
- security behavior is defined;
- compatibility behavior is defined;
- scalability is tested;
- hard-coding audit passes;
- no "unsafe" is used.

---

156. Completion checklist

File

"grammar/lexer/interpolation.md"

Purpose

Normative lexical interpolation contract.

Status

Production specification.

Owns

Interpolation lexical boundaries and scanning behavior.

Does not own

General expressions, semantic evaluation, formatting semantics, Unicode policy, runtime behavior.

Inputs

UTF-8 Zamani source.

Outputs

Deterministic interpolation-aware lexical structure.

Dependencies

DESIGN.md
specification/lexical.md
lexer/tokens.md
lexer/literals.md
lexer/unicode.md
lexer/identifiers.md
lexer/comments.md
lexer/diagnostics.md

Upstream contracts

source encoding
Unicode
literal delimiters
token taxonomy

Downstream consumers

parser
AST
semantic analyzer
IR
compiler
runtime
tooling

Public grammar contract

Interpolation embeds the normal Zamani expression grammar inside a string literal.

AST contract

Interpolated strings contain text parts and ordinary expression nodes.

Semantic contract

Expression semantics remain identical to ordinary Zamani expressions.

IR integration

Interpolation lowers into canonical string/formatting semantics.

Compiler integration

Optimization may occur only while preserving semantics.

Runtime integration

Runtime determines actual representation and evaluation strategy.

Tooling integration

Editors must understand literal/expression boundaries.

Cross-domain integration

Interpolation remains domain-neutral.

Positive tests

Required.

Negative tests

Required.

Boundary tests

Required.

Scalability tests

Required.

Compatibility tests

Required.

Determinism tests

Required.

Hard-coding audit

No artificial language limits.

Diagnostics

Structured, deterministic, source-spanned.

Security

No visual Unicode interpretation, no delimiter confusable equivalence, no implicit escaping guarantees.

Performance

Linear or near-linear scanning; no accidental quadratic rescanning.

Completion criteria

The file is complete only when all preceding contracts are implemented and conformance-tested.

---

157. Final normative rules

The following rules are mandatory and override implementation convenience:

1. Interpolation is part of the ordinary Zamani language, not a separate mini-language.

2. Embedded expressions use the ordinary expression grammar.

3. Interpolation must preserve source spans.

4. Interpolation must be deterministic.

5. Interpolation must be Unicode-correct according to "lexer/unicode.md".

6. Escaped interpolation markers must remain literal.

7. Nested expressions must not prematurely terminate interpolation.

8. Strings and comments inside expressions must be respected.

9. The lexer must never evaluate expressions.

10. The lexer must never perform semantic name lookup.

11. The lexer must never inspect hardware resources.

12. The lexer must never impose a maximum number of interpolation segments.

13. The lexer must never impose a maximum string size as a language rule.

14. The lexer must never impose a maximum Unicode size as a language rule.

15. Resource limits belong to configurable implementation policy.

16. ANTLR and Rust lexical behavior must remain conformant.

17. The AST must reuse the ordinary expression AST.

18. Interpolation must not create a second IR.

19. Quantum expressions remain subject to the canonical "quantum::ir" architecture.

20. Interpolation must not contain QEC, ZQN, HAL, routing, scheduling, or hardware realization logic.

21. No locale-dependent behavior is permitted.

22. No hidden normalization is permitted.

23. Malformed UTF-8 must be rejected rather than replaced silently.

24. Malformed interpolation must produce structured diagnostics rather than panics.

25. Rust implementation must use Rust 1.97 or Rust 1.97.1 and no "unsafe".

26. No existing repository filename should be renamed merely to implement interpolation.

27. No second competing interpolation grammar may be introduced.

28. Every stable interpolation feature must have specification, grammar, AST, semantic, IR, compiler, runtime, tooling, compatibility, and conformance contracts.

29. The implementation must scale from tiny programs to the largest programs permitted by available resources without requiring a language rewrite.

30. POCO-REAF remains the governing portability principle: interpolation describes program intent and data, not the machine on which the program happens to execute.

---

158. Production-ready definition

"grammar/lexer/interpolation.md" is considered production-ready when the repository can demonstrate the complete trace:

interpolation specification
        ↓
lexer/interpolation.md
        ↓
lexer/tokens.md
        ↓
lexer/literals.md
        ↓
Zamani.g4
        ↓
Rust lexer
        ↓
Rust parser
        ↓
frontend AST
        ↓
semantic model
        ↓
canonical IR
        ↓
compiler
        ↓
runtime
        ↓
tooling

with:

no conflicting authority
no undocumented syntax
no duplicated expression language
no fixed scalability ceiling
no target-specific lexical behavior
no unsafe code
no malformed-input panic
no Unicode ambiguity
no source-span ambiguity
no ANTLR/Rust divergence
no hidden normalization
no undocumented compatibility behavior

The result is a single, deterministic, Unicode-correct interpolation mechanism that remains independent of whether the same Zamani program ultimately executes on a tiny embedded system, classical processor, GPU, FPGA, distributed system, quantum computer, heterogeneous accelerator, or future computational substrate.