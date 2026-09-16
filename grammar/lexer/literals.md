Zamani Literal Specification

Path: "grammar/lexer/literals.md"
Status: Normative
Version: Zamani language specification v1 literal contract
Implementation baseline: Rust 1.97 / Rust 1.97.1
Safety: Safe Rust only; "unsafe" is prohibited
Scope: All source-level literals recognized by the Zamani lexer and consumed by the parser
Canonical implementation: "src/lexer.rs"
Canonical grammar composition: "grammar/Zamani.g4"

---

1. Purpose

This document defines the complete lexical and semantic contract for Zamani literals.

It establishes:

- what constitutes a literal;
- how literals are delimited;
- how literal text is preserved;
- how numeric bases are represented;
- how arbitrary-size integer literals are supported;
- how floating-point literals are represented;
- how decimal, hexadecimal, binary, octal and future numeric forms remain extensible;
- how strings are represented;
- how character literals are represented;
- how escape sequences work;
- how Unicode is handled;
- how Boolean and null/nil literals work;
- how complex numbers work;
- how quantum-state literals work;
- how resource/shape/quantity literals may be represented without imposing machine limits;
- how temporal/MTS literals work;
- how raw and byte literals may be represented;
- how literal source spans are preserved;
- how malformed literals are diagnosed;
- how literal syntax integrates with AST and semantic analysis;
- how literal syntax remains independent of hardware capacity;
- how literal syntax scales from tiny programs to programs whose values exceed any particular target machine's native width.

This document is normative for literal behavior.

It is not an implementation description of how Rust stores values internally.

---

2. Architectural Contract

The literal pipeline is:

source text
    │
    ▼
lexer
    │
    ├── literal spelling
    ├── literal token kind
    └── exact source span
    │
    ▼
parser
    │
    ▼
domain-neutral AST literal node
    │
    ▼
semantic analysis
    │
    ├── type validation
    ├── value validation
    ├── constant evaluation
    └── contextual interpretation
    │
    ▼
canonical semantic model / IR
    │
    ▼
compiler / optimizer / runtime / backend

The lexer does not:

- evaluate arbitrary expressions;
- perform constant folding;
- determine machine representation;
- select a CPU instruction;
- determine GPU representation;
- determine QPU representation;
- determine FPGA width;
- allocate memory;
- perform quantum compilation;
- perform QEC;
- perform routing;
- perform scheduling;
- resolve physical qubits;
- select a hardware device.

The lexer only determines:

1. that the source text is a valid literal;
2. which literal category it belongs to;
3. the exact source spelling;
4. the exact source span;
5. sufficient normalized lexical metadata for downstream semantic analysis.

---

3. Ownership

3.1 This file owns

This file owns:

- literal lexical categories;
- literal delimiters;
- literal lexical structure;
- digit/separator rules;
- escape syntax;
- literal token boundaries;
- malformed-literal rules;
- literal token payload contracts;
- source-span requirements;
- literal portability requirements;
- literal lexical compatibility rules.

3.2 This file does not own

This file does not own:

- AST node implementation;
- type inference;
- arbitrary-precision arithmetic implementation;
- floating-point implementation;
- constant-folding algorithms;
- runtime value representation;
- quantum IR;
- QEC;
- ZQN;
- HAL;
- scheduling;
- routing;
- hardware resource discovery;
- compiler target selection.

Those belong to their respective repository components.

---

4. Upstream Contracts

This specification depends on:

- "grammar/DESIGN.md"
- "grammar/README.md"
- "grammar/specification/lexical.md"
- "grammar/specification/syntax.md"
- "grammar/specification/semantics.md"
- "grammar/spec/lexical.md"
- "grammar/lexer/tokens.md"
- "grammar/lexer/identifiers.md"
- "grammar/lexer/operators.md"
- "grammar/lexer/unicode.md"
- "src/lexer.rs"
- "src/source_map.rs"

The current "src/lexer.rs" already defines "TokenType" categories for strings, integers, floats, chars, booleans, quantum literals and MTS literals, and attaches "Span" information to tokens. This document makes those concepts precise rather than creating a competing token system.

---

5. Downstream Contracts

Literal tokens are consumed by:

grammar/Zamani.g4
        │
        ▼
src/lexer.rs
        │
        ▼
src/parser.rs
        │
        ▼
src/ast/
        │
        ▼
semantic/type analysis
        │
        ▼
canonical semantic model
        │
        ├── classical IR
        ├── quantum::ir
        └── HDL/hardware semantic representation

No downstream consumer may infer a universal hardware limit from the lexical representation of a literal.

---

6. Fundamental Literal Rule

A literal is source syntax denoting a value, symbolic value, or value-like compile-time object.

Examples:

42
-42
3.14159
0xff
0b101010
0o755
1_000_000
true
false
nil
"hello"
'Z'
|0⟩
|+⟩
1.5e1000
0x1.fp4

The exact semantic type is determined later.

---

7. Literal Token Categories

The canonical literal categories are:

INTEGER_LITERAL
FLOAT_LITERAL
STRING_LITERAL
CHAR_LITERAL
BOOLEAN_LITERAL
NIL_LITERAL
COMPLEX_LITERAL
QUANTUM_LITERAL
RAW_STRING_LITERAL
BYTE_STRING_LITERAL
BYTE_LITERAL
MTS_LITERAL

Additional domain-specific literal categories may be introduced only through an explicit language specification change.

They must not be silently added to the lexer.

---

8. General Lexical Principle

The lexer must preserve the programmer's literal spelling.

For example:

1_000_000

must preserve enough information to distinguish it from:

1000000

even though both denote the same mathematical integer.

This is required for:

- source-to-source tools;
- formatting;
- diagnostics;
- source maps;
- IDEs;
- refactoring;
- provenance;
- reproducible compilation;
- exact source reconstruction.

The semantic value may subsequently normalize separators.

---

9. Integer Literals

9.1 Decimal integers

Canonical syntax:

DIGIT+

Examples:

0
1
42
1000000
123456789

Digit separators are permitted:

1_000
1_000_000
123_456_789

Rules:

- "_" may occur only between digits;
- "_" may not occur at the beginning;
- "_" may not occur at the end;
- consecutive "_" characters are invalid;
- a separator must not cross a base prefix.

Invalid:

_42
42_
4__2

---

10. Binary Integer Literals

Syntax:

0b BINARY_DIGIT+

or:

0B BINARY_DIGIT+

Examples:

0b0
0b1
0b1010
0b1111_0000

Only:

0
1
_

are valid after the prefix.

Invalid:

0b102
0b_101
0b101_

---

11. Octal Integer Literals

Syntax:

0o OCTAL_DIGIT+

or:

0O OCTAL_DIGIT+

Examples:

0o0
0o755
0o7_777

Valid digits:

0-7

Invalid:

0o8
0o128

---

12. Hexadecimal Integer Literals

Syntax:

0x HEX_DIGIT+

or:

0X HEX_DIGIT+

Examples:

0x0
0xff
0xFF
0xDEAD_BEEF

Valid digits:

0-9
a-f
A-F
_

---

13. Integer Magnitude

Zamani literal syntax must not impose a maximum integer width.

Therefore the language must not define:

MAX_INTEGER_BITS = 64
MAX_INTEGER_BITS = 128
MAX_INTEGER_BITS = 256

as universal language limits.

The following is legal source syntax if representable by the implementation's source/value infrastructure:

12345678901234567890123456789012345678901234567890

The lexer treats this as an integer literal.

Whether a particular target can execute the value directly is a later semantic/code-generation question.

This is essential for POCO-REAF.

---

14. Integer Representation Contract

The lexer should preserve the literal as text.

The lexer must not require conversion to:

i64
u64
usize
u128

merely to recognize the token.

A target-independent implementation may represent the token as:

Token {
    token_type: TokenType::Integer,
    literal: String,
    span: Span,
}

or an equivalent owned representation.

The semantic layer may subsequently parse the literal into:

- arbitrary-precision integer;
- target-sized integer;
- compile-time symbolic integer;
- dependent-size parameter;
- resource quantity;
- constant-expression representation.

The lexical layer must remain target independent.

---

15. Sign Handling

"+" and "-" are operators, not part of the integer literal token.

Therefore:

-42

lexes conceptually as:

Minus
Integer("42")

and:

+42

as:

Plus
Integer("42")

This allows:

- unary operator semantics;
- constant folding;
- type-directed interpretation;
- arbitrary precision;
- consistent parsing.

The exception is where a future explicitly specified literal format requires a signed lexical representation. Such a format must be separately specified.

---

16. Floating-Point Literals

Zamani supports decimal floating-point source syntax.

Minimum forms:

1.0
0.5
42.75
3.141592653589793

Exponent forms:

1e10
1E10
1.5e10
1.5E10
1.5e-10
1.5e+10

Separator forms:

1_000.25
1.234_567
1.0e1_000

Separators must obey the same adjacency rules as integer digits.

---

17. Floating-Point Grammar

Conceptually:

decimal_integer '.' decimal_digits exponent?
decimal_integer exponent
'.' decimal_digits exponent?

where:

exponent
    := ('e' | 'E') ('+' | '-')? decimal_digits

Examples:

1.0
0.5
.5
1.
1e10
1.0e-10
.5e+2

Whether "1." and ".5" are accepted must remain consistent with "grammar/Zamani.g4", "grammar/specification/syntax.md", and the parser.

If the language chooses not to support one of these forms, it must reject it deterministically rather than tokenizing it inconsistently.

---

18. Range Operator Ambiguity

Floating literals must not consume the range operator.

For example:

1..10

must lex as:

INTEGER_LITERAL("1")
DOTDOT
INTEGER_LITERAL("10")

not:

FLOAT_LITERAL("1.")
DOT
INTEGER_LITERAL("10")

Similarly:

1..=10

must remain:

INTEGER_LITERAL
DOT_DOT_EQUALS
INTEGER_LITERAL

The lexer must use maximal-munch with explicit range-operator precedence over a trailing decimal point where necessary.

This is especially important because the current grammar contains range operators and floating literals.

---

19. Floating-Point Semantic Width

The lexical grammar must not restrict floating literals to:

f32
f64

The source may denote values later assigned to:

f16
f32
f64
f128
arbitrary precision
decimal
symbolic
interval
complex
quantum parameter

The current grammar already exposes multiple floating-point type names, so literal syntax must remain independent from the selected semantic type.

---

20. Non-Finite Floating Values

The lexical grammar must not make:

NaN
Infinity
-Infinity

ordinary decimal literals.

If supported, they should be named constants or explicitly specified special literals.

This avoids confusing:

1e999999

with:

Infinity

The semantic layer determines whether an exponent overflows a selected numeric type.

---

21. Hexadecimal Floating-Point Literals

Zamani should reserve an extensibility point for exact hexadecimal floating-point notation:

0x1.fp4
0x1.8p+2

If not yet accepted by "grammar/Zamani.g4", it must be treated as reserved/proposed, not silently accepted by the Rust lexer.

Once stabilized, its syntax should be:

0x hexadecimal_digits ('.' hexadecimal_digits)? p exponent

with:

p | P

and a signed decimal exponent.

This is useful for exact low-level numeric work, HDL, scientific computing, embedded programming and reproducible numerical constants.

---

22. String Literals

Canonical string syntax:

"hello"

Strings support Unicode.

Examples:

"hello"
"Zamani"
"量子"
"🙂"
"scale from atom to everywhere"

The lexer must preserve UTF-8 source correctly.

String contents must not be limited to ASCII.

---

23. String Escape Sequences

The canonical escape forms are:

\\
\"
\'
\n
\r
\t
\b
\f
\0
\v
\a

Unicode escapes should support an explicit syntax such as:

\u{1F600}

Examples:

"line one\nline two"
"quote: \""
"unicode: \u{1F600}"

Invalid escapes must produce a lexical diagnostic.

---

24. Unicode Escape Validation

Unicode escapes must:

- contain valid hexadecimal digits;
- contain a valid Unicode scalar value;
- reject surrogate code points;
- reject values above "0x10FFFF".

For example:

"\u{1F600}"

is valid.

A surrogate such as:

"\u{D800}"

is invalid.

The lexer must report the exact source span of the invalid escape.

---

25. String Newlines

Ordinary quoted strings should not silently consume an unescaped physical newline unless multiline strings are explicitly enabled.

For:

"hello
world"

the lexer must either:

1. reject it; or
2. recognize it as a dedicated multiline-string form.

It must never accidentally continue scanning indefinitely.

---

26. Multiline Strings

A dedicated multiline/raw form may be supported:

"""
hello
world
"""

or another explicitly standardized delimiter.

If supported, the delimiter must be specified in:

grammar/lexer/literals.md
grammar/Zamani.g4
grammar/lexer/tokens.md
grammar/specification/lexical.md

The lexer must correctly handle:

- embedded quotes;
- newlines;
- Unicode;
- escapes if enabled;
- unterminated delimiters.

---

27. Raw Strings

Raw strings are required for:

- regular expressions;
- source generation;
- embedded HDL;
- embedded quantum formats;
- JSON;
- SQL;
- configuration;
- documentation;
- code generation.

A Rust-like raw syntax is suitable:

r"hello\nworld"

where "\n" remains two source characters rather than becoming a newline.

A delimiter-counted form may be used for arbitrary embedded quote sequences:

r#"..."#

or an equivalent Zamani-specific design.

The exact delimiter must be standardized before implementation.

---

28. Byte Strings

Byte strings should be distinct from Unicode strings.

Example:

b"hello"

The semantic type is byte-oriented.

Unicode source may be rejected or explicitly encoded according to the byte-string specification.

The lexer must not silently truncate Unicode characters into bytes.

---

29. Character Literals

Canonical syntax:

'a'
'Z'
'🙂'
'\n'
'\u{1F600}'

A character literal denotes exactly one Unicode scalar value.

It must not denote an arbitrary UTF-8 byte sequence.

---

30. Character Literal Cardinality

Exactly one Unicode scalar value is required after escape processing.

Invalid:

''
'ab'
'hello'

unless a separate literal category explicitly defines them.

---

31. Escaped Character Literals

Supported forms include:

'\n'
'\r'
'\t'
'\\'
'\''
'\"'
'\0'
'\u{1F600}'

The semantic value is one Unicode scalar value.

---

32. Boolean Literals

Canonical Boolean literals:

true
false

The lexer may represent them as:

BOOLEAN_LITERAL

or map them to dedicated keyword token types if that is the established token architecture.

The distinction must be documented in "grammar/lexer/tokens.md".

The parser and AST must not require duplicate Boolean literal concepts.

---

33. Boolean Lexical Case

The canonical forms are lowercase:

true
false

Alternative spellings such as:

TRUE
FALSE
True
False

must not be silently accepted unless case-insensitive keywords are explicitly adopted by the language specification.

---

34. Nil / Null Literal

Zamani may support a null-like literal:

nil

If "null" is retained as an alias, the compatibility specification must explicitly define:

nil
null

as equivalent source spellings.

The current lexer maps both "nil" and "null" to "KeywordNil"; this behavior must be reflected consistently across the grammar and language specification rather than remaining an implementation-only behavior.

The semantic layer determines whether the value means:

- absence;
- optional-none;
- nullable reference;
- empty handle;
- domain-specific null state.

---

35. Complex Literals

Complex values should not require a dedicated keyword for every possible imaginary value.

A generic representation may use expressions:

3 + 4i

or an explicitly standardized complex literal form.

If a dedicated suffix is adopted:

i
j

must be specified as a lexical suffix and must not conflict with identifiers.

The preferred architecture is to keep complex numbers compositional:

integer/float literal
+
identifier/operator semantics

unless dedicated lexical notation provides a demonstrated language-level benefit.

The grammar currently exposes "COMPLEX_LITERAL" in its literal vocabulary; that concept must therefore either be fully specified and implemented or explicitly marked proposed rather than remaining a phantom grammar feature.

---

36. Quantum Literals

Quantum literals are source-level representations of quantum states or basis-state notation.

Examples:

|0⟩
|1⟩
|+⟩
|-⟩
|ψ⟩

The lexer must not assume that quantum literals are limited to:

0
1
+
-

only.

The syntax must be extensible without hard-coding a finite list of quantum states.

---

37. Quantum Literal Delimiters

The canonical bra-ket state-literal form is:

| payload ⟩

where:

| U+007C
⟩ U+27E9

are the delimiters.

The payload may be:

- a basis-state label;
- an identifier;
- a symbolic state name;
- a normalized state expression;
- a domain-approved symbolic representation.

The lexical and semantic specifications must determine exactly which forms are stable.

---

38. Quantum Literal Scalability

The lexer must not impose:

MAX_QUBITS
MAX_STATE_BITS
MAX_STATE_VECTOR_LENGTH
MAX_AMPLITUDES
MAX_REGISTER_SIZE

A quantum literal's source length is limited only by implementation resources.

The semantic layer may reject an invalid state expression, but that is not a lexical hardware limit.

---

39. Quantum Literal and "quantum::ir"

Quantum literals must lower through the existing domain-neutral frontend AST and then into the canonical "quantum::ir" semantic boundary.

They must not create:

QuantumLiteralIR
ZamaniQuantumIR
GrammarQuantumIR

as a competing quantum representation.

The flow is:

QUANTUM_LITERAL
      │
      ▼
AST literal
      │
      ▼
semantic quantum state
      │
      ▼
quantum::ir

This preserves the established repository architecture.

---

40. Quantum Lexing Must Be General

The current lexer contains special handling that recognizes a "|...⟩" form by consuming the delimiter and one inner character. That is too narrow for a production universal language.

Production behavior must instead:

1. detect the opening delimiter;
2. scan according to the quantum-literal grammar;
3. correctly recognize the closing delimiter;
4. preserve the entire source spelling;
5. reject malformed/unclosed forms;
6. support Unicode safely;
7. produce an exact span;
8. avoid imposing a fixed state vocabulary.

---

41. MTS / Temporal Literals

The lexer currently declares an "MTSLiteral" token type, while the implementation path does not provide a corresponding complete literal scanner.

This must be corrected.

MTS literals must not remain a declared-but-unimplemented token.

A temporal literal must have a fully specified syntax before becoming stable.

Possible conceptual forms include:

mts[...]

but the contents must be specified independently.

The grammar must distinguish:

- timestamp;
- duration;
- interval;
- temporal coordinate;
- timeline identifier;
- branch identifier;
- observation point.

---

42. MTS Scalability

MTS syntax must not hard-code:

MAX_TIMELINES
MAX_BRANCHES
MAX_TIMESTAMP
MAX_HISTORY

A literal may contain arbitrarily large representable values subject to source/compiler resources.

---

43. Quantity Literals

Zamani may need portable quantities such as:

10ms
1GB
5GHz
100nm
3K

However, units must not be silently lexed as part of an arbitrary number unless the grammar explicitly defines them.

Preferred architecture:

numeric literal
+
unit suffix

with units represented as semantic identifiers or a controlled unit-literal token.

This allows:

10ms
10 us
10 nanoseconds

to be standardized without coupling the lexer to hardware implementations.

---

44. Physical Quantities

Quantity literals may represent:

- time;
- frequency;
- distance;
- mass;
- energy;
- power;
- voltage;
- current;
- temperature;
- bandwidth;
- latency;
- storage;
- memory;
- quantum fidelity;
- probability;
- error rate.

The lexer must not convert them to machine-specific units.

For example:

1GB

is a source-level quantity.

It does not mean:

allocate exactly 1 GB of physical RAM

until semantic/resource analysis establishes that meaning.

---

45. Resource Quantities and POCO-REAF

Resource literals must describe portable requirements.

Example:

requires memory >= 8GB;

does not mean:

machine must contain exactly 8GB

and:

requires qubits >= n;

does not imply:

physical qubits are numbered 0..n-1

The compiler resolves actual resources later.

---

46. Duration Literals

Duration literals should be compositional.

Examples:

10ns
5us
2ms
1s
3min
1h

If compound forms are supported:

1h30min

must be explicitly standardized.

Duration values must not be stored by the lexer in a fixed-width machine integer.

---

47. Timestamp Literals

Timestamp literals should use an unambiguous standardized representation.

Examples may include:

2026-09-16T10:30:00Z

or a dedicated:

time"2026-09-16T10:30:00Z"

form.

The preferred design is an explicit prefix if ambiguity with subtraction or arithmetic would otherwise arise.

Timestamp parsing belongs to the lexical/semantic literal subsystem, not the generic identifier scanner.

---

48. Regular Expressions and Other Structured Literals

Regular expressions, embedded query languages, HDL snippets and other structured source objects should use explicit delimiters.

They must not be implemented by adding arbitrary special cases to ordinary string scanning.

Each structured literal must define:

- delimiter;
- escape rules;
- nesting rules;
- Unicode behavior;
- termination;
- source span;
- AST representation;
- semantic validation.

---

49. Literal Prefixes

Literal prefixes are reserved for semantic categories.

Examples:

r"..."     raw string
b"..."     byte string
q"..."     future quantum textual literal if adopted
time"..."  timestamp
regex"..." regular expression

A prefix must be recognized only when followed by its required delimiter.

Otherwise it remains an identifier.

For example:

radius

must remain an identifier.

The lexer must not accidentally classify:

rvalue

as a raw string prefix.

---

50. Lexical Maximal-Munch

The lexer must use deterministic longest-valid-token behavior.

Examples:

123.45

must be one floating literal.

1..10

must remain an integer + range operator + integer.

0xFF

must be one hexadecimal integer.

0x1.fp4

must either be one standardized hexadecimal float or a deterministic lexical error/reserved form.

There must never be implementation-dependent tokenization.

---

51. Literal Separators

Digit separators:

_

are allowed only where specified.

Valid:

1_000
0xff_ff
0b1010_0101
1.234_567
1e1_000

Invalid:

_100
100_
1__000
0x_FF
0b_101
1_.0
1._0

The lexer must produce a targeted diagnostic rather than silently splitting malformed literals into unrelated tokens.

---

52. Leading Zeroes

Decimal literals may contain leading zeroes:

00042

unless the language specification explicitly gives another meaning.

Leading zeroes must not automatically imply octal.

Octal requires the explicit:

0o

prefix.

---

53. Numeric Overflow

Lexical recognition must not fail merely because a literal is too large for the target machine.

For example:

340282366920938463463374607431768211456

must still be recognized as an integer literal if its syntax is valid.

Overflow relative to a selected semantic type is a semantic/type-checking concern.

This distinction is essential for POCO-REAF.

---

54. Numeric Underflow

Likewise:

1e-1000000

is lexically a valid floating literal if its syntax is valid.

Whether it is representable as:

f16
f32
f64
f128

is determined later.

---

55. Exact Decimal Semantics

The source spelling of decimal floating literals must be preserved.

The compiler must not assume that every decimal literal should immediately be converted to binary IEEE floating-point.

This permits future semantic types such as:

Decimal
Rational
ExactReal
Interval
SymbolicReal

without changing lexical syntax.

---

56. Rational Literals

Rational values should preferably remain compositional:

1 / 3

rather than introducing:

RATIONAL_LITERAL

unless a dedicated exact-rational syntax is justified.

This prevents the lexer from becoming a mathematical library.

---

57. Symbolic Numeric Values

Symbolic values such as:

pi
e
tau

should normally be identifiers or semantic constants, not primitive lexer literals.

The current lexer has dedicated "PiSymbol"/"SigmaSymbol" concepts, so any dedicated mathematical symbol must be explicitly justified in the lexical specification and kept separate from arbitrary mathematical constants.

---

58. Unicode Numeric Symbols

Unicode mathematical symbols may be accepted only when explicitly specified.

For example:

π
Σ
Π

must not become arbitrary numeric literals merely because they have mathematical meaning.

They may instead be:

- identifiers;
- operators;
- dedicated symbolic tokens.

The lexer and parser must remain deterministic.

---

59. String Source Preservation

The token literal should preserve source spelling where required by tooling.

For:

"hello\n"

the token must retain sufficient information to distinguish:

backslash + n

from an actual newline.

Semantic decoding happens later or through a dedicated lexer-decoding layer.

This prevents source reconstruction loss.

---

60. Escape Decoding Contract

There are two distinct representations:

source spelling
semantic value

Example:

"\n"

Source spelling:

\ + n

Semantic value:

U+000A

The lexer must not confuse the two.

---

61. Invalid Escape Diagnostics

Examples:

"\q"
"\u"
"\u{}"
"\u{ZZZZ}"
"\u{D800}"

must produce diagnostics identifying:

- literal kind;
- invalid escape;
- source span;
- expected syntax.

Diagnostics should be deterministic and machine-readable.

---

62. Unterminated Literal Diagnostics

The lexer must detect:

"hello

'c

|0

r"hello

and similar forms.

The diagnostic span should begin at the literal delimiter and extend to the best available termination point.

The lexer must recover sufficiently to continue lexing later source when practical.

---

63. Error Recovery

Malformed literals must not cause:

- infinite loops;
- repeated diagnostics for the same byte;
- accidental swallowing of the rest of the source;
- corrupted source positions.

Recovery should normally advance to a safe synchronization point:

- matching delimiter;
- newline for ordinary strings;
- token boundary;
- EOF.

---

64. UTF-8 Correctness

All literal processing must operate on valid UTF-8 source.

The lexer must distinguish:

byte offset
Unicode scalar value
source line
source column

A byte offset is the canonical source position.

Character columns may be derived.

The implementation must never use a byte offset as though it were a character index.

This is particularly important because the current lexer uses UTF-8-aware "char" scanning but also contains indexing logic that mixes byte offsets and character iteration. That must be corrected in "src/lexer.rs" as an implementation consequence of this specification.

---

65. Source Span Contract

Every literal token must have:

file_id
start byte offset
end byte offset
start line
start column

The span must cover the complete literal.

For:

"hello"

the span includes the opening and closing quotes.

For:

0xDEAD

the span includes:

0
x
D
E
A
D

---

66. Empty Literals

Empty string:

""

is valid.

Empty character:

''

is invalid.

Empty quantum state:

|⟩

is valid only if the quantum grammar explicitly defines it.

Otherwise it is invalid.

---

67. Literal Context Independence

A literal must be lexically valid independently of whether it is used in:

- classical code;
- quantum code;
- HDL;
- AI;
- data;
- networking;
- distributed computing;
- embedded code;
- accelerator code;
- compile-time code.

Context may determine its semantic type, but not arbitrarily change its lexical spelling.

---

68. Literal Type Inference

Examples:

42

may infer an integer type.

3.14

may infer a floating type.

"hello"

may infer a string type.

|0⟩

may infer a quantum-state type.

The lexer must not perform this inference.

---

69. Literal Genericity

Literal syntax must support generic semantic types.

For example:

let x: f128 = 1.0;
let y: Decimal = 1.0;
let z: Rational = 1;

The same lexical literal can participate in different semantic representations.

---

70. Compile-Time Evaluation

Literal values may participate in constant expressions:

const n = 1_000 * 1_000;

But the lexer must not evaluate:

1_000 * 1_000

It only emits:

INTEGER_LITERAL
STAR
INTEGER_LITERAL

Constant evaluation belongs to semantic analysis.

---

71. Array and Shape Literals

Literal syntax must support arbitrarily large semantic shape expressions.

Example:

Tensor<f64, [n, m, k]>

The lexer must not impose:

MAX_RANK
MAX_DIMENSION
MAX_SHAPE_VALUE

on the language.

The parser and semantic layer determine whether the expression is valid.

---

72. Quantum Resource Literals

A source program may contain:

Qubit<1024>

or:

Qubit<n>

The literal:

1024

is an ordinary integer literal.

The quantum semantic layer determines what it means.

The lexer must not know that:

1024

is a maximum, minimum, physical device size, or topology.

---

73. HDL Width Literals

A hardware description may contain:

width = 32;

or:

width = parameter;

The number "32" is a program parameter.

The language must not impose a universal:

MAX_WIDTH = 32

or:

MAX_WIDTH = 4096

limit.

---

74. Hardware Resource Literals

Resource expressions may contain:

memory >= 8GB
cores >= n
qubits >= q
bandwidth >= 10Gbps

Literal parsing remains independent of the eventual hardware.

---

75. Network Address Literals

IP addresses, MAC addresses and other network forms should not automatically become primitive numeric literals.

They should have explicit syntax or typed constructors.

For example:

ipv4"192.0.2.1"

is less ambiguous than interpreting:

192.0.2.1

as a floating number.

This principle applies to:

- IPv4;
- IPv6;
- MAC;
- CIDR;
- URLs;
- URIs;
- socket addresses.

---

76. Cryptographic Literals

Hashes, keys and signatures should use explicit typed literals if literal syntax is provided.

Examples:

hex"0123456789abcdef"

or:

sha256"..."

must not be confused with ordinary strings.

Cryptographic validation belongs downstream.

---

77. Data/Serialization Literals

Structured formats such as:

JSON
YAML
TOML
CSV

should not become primitive literals unless there is a language-level semantic reason.

Prefer:

json"{...}"

or library/parser APIs.

This prevents the lexical grammar from absorbing external languages.

---

78. Embedded Language Literals

HDL, OpenQASM, SQL, C, C++, Python and other embedded source forms must use explicit embedding constructs.

They must not silently become ordinary strings if the language needs syntax-aware processing.

The interoperability layer owns their semantic interpretation.

---

79. Literal Prefix Namespace

Future literal prefixes should be namespaced semantically rather than becoming an uncontrolled keyword list.

For example:

prefix"payload"

may identify a literal family.

The prefix must be registered by the language/dialect specification.

Unknown prefixes should either:

1. remain identifiers in an ordinary expression context; or
2. produce a precise diagnostic when syntactically required to be a literal prefix.

They must never cause lexer ambiguity.

---

80. Dialect Interaction

A dialect may introduce additional literals.

A dialect literal must declare:

literal name
prefix/delimiter
lexical rules
escape rules
AST mapping
semantic type
compatibility
source span behavior
diagnostics

Dialect literals must not silently change the meaning of stable core literals.

---

81. Macro Interaction

Macros receive literal syntax as tokens/AST nodes.

Macros must not bypass literal validation.

A macro generating:

0xZZ

must cause the same lexical/parser/semantic validation as source-written syntax at the relevant compilation phase.

---

82. Metaprogramming Interaction

Compile-time code may inspect literal syntax.

Therefore literal AST nodes should preserve:

- literal category;
- original spelling;
- decoded value when available;
- source span;
- base;
- suffix/prefix;
- semantic metadata.

This is important for source transformation and code generation.

---

83. Literal Suffixes

Numeric suffixes may select semantic types.

Examples could include:

42u
42u64
42i32
1.0f32
1.0f64

However, suffixes must be explicitly standardized.

They must not be introduced ad hoc by the lexer.

If suffixes are supported, the token contract should distinguish:

literal body
suffix

so semantic analysis can decide the type.

---

84. Target Independence of Suffixes

A source suffix such as:

42u64

expresses a language-level semantic type.

It does not guarantee that every target uses a physical 64-bit register.

The compiler may lower it appropriately.

---

85. Arbitrary Precision Requirement

For language-level integers and compile-time arithmetic, the compiler architecture should permit arbitrary precision.

This does not require the lexer itself to implement big integers.

The key requirement is:

source literal length
    ≠
target machine integer width

---

86. Resource-Bounded Compilation

“Unbounded” in POCO-REAF means the language has no artificial fixed resource ceiling.

It does not mean infinite memory is physically available.

Therefore:

program size <= available compiler resources

is an implementation constraint, not a language grammar limit.

A compiler may report:

resource exhausted

without redefining the language to have a fixed maximum literal size.

---

87. Literal Security

The lexer must defend against pathological literals without embedding arbitrary language limits.

Security mechanisms may include:

- streaming scanning;
- bounded diagnostic construction;
- avoiding quadratic string concatenation;
- avoiding repeated rescanning;
- cancellation;
- resource accounting;
- configurable compiler resource budgets.

These are implementation controls, not language-level literal limits.

---

88. No "unsafe"

The literal subsystem must be implemented entirely in safe Rust.

Prohibited:

unsafe { ... }

and unsafe APIs.

This applies to:

- lexer scanning;
- literal decoding;
- UTF-8 processing;
- escape processing;
- token construction;
- diagnostics;
- tests.

The current project metadata establishes Rust 2021 and a Rust 1.97/1.97.1 baseline; this specification therefore targets that baseline without requiring newer language features.

---

89. Rust Implementation Contract

"src/lexer.rs" should provide literal scanning routines conceptually equivalent to:

read_integer_literal
read_float_literal
read_string_literal
read_char_literal
read_raw_string_literal
read_byte_literal
read_byte_string_literal
read_quantum_literal
read_mts_literal

The actual names may differ.

The important requirement is that each routine has one responsibility and one deterministic termination rule.

---

90. Do Not Use "chars().nth()" for Cursor Movement

The lexer must not repeatedly locate characters using:

input.chars().nth(byte_offset)

because a byte offset is not a character index.

The current lexer contains this class of UTF-8 position problem and must be corrected when "src/lexer.rs" is brought into conformance.

Preferred safe-Rust approaches include:

- maintaining a UTF-8-aware cursor;
- using "char_indices";
- maintaining byte offsets directly;
- using a dedicated cursor abstraction.

---

91. Cursor Invariant

The lexer cursor must maintain:

current byte position
next byte position
current character

with the invariant:

position <= read_position <= input_length

and:

position

always denotes the start byte of the current Unicode scalar value.

---

92. Literal Scanner Invariant

Every literal scanner must guarantee:

on success:
    cursor = first character after literal

on failure:
    cursor advances sufficiently for deterministic recovery

never:
    cursor remains unchanged indefinitely

This prevents infinite loops.

---

93. Literal Span Invariant

For every successful literal:

span.start = first byte of opening/prefix
span.end   = first byte after closing literal

The span is half-open:

[start, end)

This makes adjacent literals and operators unambiguous.

---

94. Token Literal Payload

For compatibility with the current "Token" structure, the lexical payload may remain:

pub literal: String

as the raw source spelling.

Do not force the lexer to eagerly construct large semantic values.

This is especially important for arbitrary-precision integers and large strings.

---

95. Lazy Semantic Decoding

Where practical:

lexer:
    preserve spelling

parser:
    preserve syntax

semantic analysis:
    decode/validate value

constant evaluator:
    compute value

This separation avoids duplicating semantic logic in the lexer.

---

96. Diagnostics

Literal diagnostics should identify:

error code
literal kind
message
primary span
optional secondary span
expected form

Suggested categories:

E1001 InvalidIntegerLiteral
E1002 InvalidBinaryLiteral
E1003 InvalidOctalLiteral
E1004 InvalidHexLiteral
E1005 InvalidFloatLiteral
E1006 InvalidDigitSeparator
E1007 UnterminatedString
E1008 InvalidEscape
E1009 InvalidUnicodeEscape
E1010 InvalidCharacterLiteral
E1011 UnterminatedCharacterLiteral
E1012 InvalidQuantumLiteral
E1013 UnterminatedQuantumLiteral
E1014 InvalidTemporalLiteral
E1015 InvalidLiteralSuffix

Exact numeric codes may be assigned by the central diagnostics specification.

The important requirement is that literal errors are stable and machine-readable.

---

97. Error Message Quality

Bad:

invalid token

Better:

invalid hexadecimal integer literal: expected at least one hexadecimal digit after `0x`

Better still, with a structured code:

E1004 invalid hexadecimal integer literal
expected one or more hexadecimal digits after `0x`

---

98. Compatibility With Existing Token Types

Existing token concepts should be retained where they are already established:

Integer
Float
String
Char
Boolean
QuantumLiteral
MTSLiteral

Do not unnecessarily rename them.

Where the existing token type is semantically overloaded, resolve that through documentation and AST/semantic contracts before renaming.

The goal is compatibility and consolidation, not gratuitous churn.

---

99. "MTSLiteral" Compatibility

Because "TokenType::MTSLiteral" already exists, the implementation must choose one of two explicit states:

Stable

Fully implement and test MTS literal scanning.

Proposed

Remove it from the active lexical contract only after the compatibility/deprecation process explicitly marks it as proposed.

It must not remain silently declared but unused.

---

100. Boolean Token Compatibility

The existing lexer maps:

true
false

to dedicated keyword token variants rather than a generic "Boolean" token.

This is acceptable if:

grammar/lexer/tokens.md
grammar/Zamani.g4
src/parser.rs
AST

all agree.

Do not introduce a second Boolean-token mechanism solely for this document.

---

101. Literal-to-AST Contract

Every literal category must have a deterministic AST representation.

Conceptually:

IntegerLiteral
FloatLiteral
StringLiteral
CharLiteral
BooleanLiteral
NilLiteral
ComplexLiteral
QuantumLiteral
TemporalLiteral
RawStringLiteral
ByteStringLiteral
ByteLiteral

The actual AST type names are owned by "src/frontend/ast/" and may differ.

The critical requirement is one canonical AST representation per semantic literal category.

---

102. Generic AST Requirement

The AST must not encode hardware-specific numeric representations.

Bad:

GpuFloatLiteral
QpuIntegerLiteral
FpgaIntegerLiteral

Good:

Literal
  ├── Integer
  ├── Float
  ├── String
  ├── Char
  ├── Boolean
  ├── Quantum
  └── ...

Target realization happens later.

---

103. Literal-to-IR Contract

Literals are lowered only after semantic analysis.

Examples:

Integer
    ↓
classical semantic constant
    ↓
classical IR constant

Quantum state
    ↓
quantum semantic state
    ↓
quantum::ir

No literal-specific hardware IR should be created by the lexer.

---

104. Quantum Literal IR Contract

Quantum literals must use:

AST
→ semantic quantum representation
→ quantum::ir

They must not bypass the canonical quantum semantic boundary.

This is mandatory.

---

105. HDL Literal Contract

HDL-specific numeric values remain ordinary numeric literals unless a dedicated hardware literal is explicitly standardized.

For example:

width = 64;

uses an integer literal.

The HDL semantic layer determines that "width" represents a signal width.

---

106. AI/Data Literal Contract

Tensor dimensions, dataset sizes, model parameters and similar values use normal literals unless a dedicated literal form is semantically necessary.

For example:

shape = [128, 256, 512];

contains ordinary integer literals.

The AI/data subsystem determines their meaning.

---

107. Networking Literal Contract

Networking-specific addresses should use explicit typed syntax where necessary.

The lexer must not guess whether:

1.2.3.4

means:

- floating-point arithmetic;
- IPv4;
- four decimal values.

Explicit syntax is required to eliminate ambiguity.

---

108. Security Literal Contract

Security-related literals must preserve exact source representation.

This is particularly important for:

- keys;
- hashes;
- signatures;
- byte sequences;
- encoded identities.

The lexer must not normalize cryptographic bytes in a lossy manner.

---

109. Serialization Contract

Literal source must round-trip through tooling whenever possible.

A formatter/parser pair should be able to distinguish:

0xff
255
0b1111_1111

even if their semantic values are equal.

This is why source spelling preservation is mandatory.

---

110. Determinism

For the same source bytes and language version:

lexer(source)

must produce the same token sequence and spans.

No literal classification may depend on:

- CPU;
- GPU;
- operating system;
- locale;
- available RAM;
- thread count;
- target architecture;
- QPU;
- runtime state.

---

111. Locale Independence

Numeric literals must always use language-defined separators and decimal syntax.

The lexer must not depend on host locale.

For example:

1.25

must never become dependent on whether the host locale uses:

.

or:

,

as its decimal separator.

---

112. Normalization

The lexer must not Unicode-normalize arbitrary string contents.

Source spelling must remain preserved.

Identifiers and literals have different normalization contracts.

Literal contents are data.

---

113. Security Against Escape Confusion

The lexer must distinguish:

source backslash
decoded character
Unicode escape
literal delimiter

to prevent malformed or ambiguous source.

Escapes must be validated before semantic decoding.

---

114. Security Against Pathological Input

Literal scanners must avoid:

- unbounded recursive calls;
- quadratic concatenation;
- repeated whole-input scans;
- allocation proportional to repeated intermediate states.

For large literals, scanning should be linear in source length.

Target complexity:

O(n)

where "n" is literal source length.

---

115. Memory Scalability

A lexer may store the source in an owned/shared representation, as the current implementation does.

However, literal scanning must not create unnecessary copies.

For large strings and numeric literals, source slices or equivalent efficient representations should be preferred where architecture permits.

The public token API may continue using owned strings for compatibility if required.

---

116. Streaming Compatibility

The literal specification must not prevent future streaming lexers.

Therefore semantic behavior must not depend on:

whole-file availability

beyond the normal requirement to identify literal termination.

A future streaming lexer may implement the same token contract.

---

117. Incremental Compilation

Literal spans and token boundaries must remain stable under incremental compilation.

A change to one literal should not require reparsing unrelated files merely because a literal value changed.

---

118. IDE/LSP Requirements

Literal tokens must provide enough information for:

- syntax highlighting;
- hover;
- diagnostics;
- semantic tokenization;
- formatting;
- completion;
- refactoring.

For example:

0xDEAD_BEEF

should remain identifiable as one integer literal.

---

119. Formatter Requirements

The formatter may normalize:

1_000_000

to:

1_000_000

or another canonical spelling.

But it must preserve semantic value.

Formatting rules are separate from lexical recognition.

---

120. Serialization of AST Literals

If AST serialization is supported, literal nodes should preserve:

kind
raw spelling
decoded/normalized value when available
base
prefix
suffix
span

This supports reproducibility and tooling.

---

121. Testing Strategy

Every literal category must have:

positive tests
negative tests
boundary tests
Unicode tests
source-span tests
round-trip tests
scalability tests
compatibility tests
determinism tests

---

122. Integer Positive Tests

Minimum examples:

0
1
42
1_000
0b0
0b1010
0o755
0xFF
0xDEAD_BEEF

Large-value examples:

123456789012345678901234567890
0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF

---

123. Integer Negative Tests

_
1_
_1
1__2
0b
0b2
0o8
0x
0xG

---

124. Floating Positive Tests

0.0
1.0
.5
1.
1e10
1.5e10
1.5e-10
1_000.25
1.234_567

Only include forms actually standardized by "Zamani.g4".

---

125. Floating Negative Tests

1._0
1_.0
1e
1e+
1e-
1e_
1__0.0

---

126. Range Tests

1..10
1..=10
0..n
1.0..2.0

The lexer must prove that range operators are not accidentally swallowed by floating literals.

---

127. String Positive Tests

""
"hello"
"hello world"
"🙂"
"\n"
"\u{1F600}"

---

128. String Negative Tests

"unterminated
"\q"
"\u"
"\u{}"
"\u{D800}"
"\u{110000}"

---

129. Character Positive Tests

'a'
'Z'
'🙂'
'\n'
'\u{1F600}'

---

130. Character Negative Tests

''
'ab'
'unterminated
'\q'

---

131. Quantum Positive Tests

|0⟩
|1⟩
|+⟩
|-⟩
|ψ⟩

Plus all future standardized symbolic forms.

---

132. Quantum Negative Tests

|
|0
0⟩
|⟩

The exact status of "|⟩" depends on the quantum literal grammar.

---

133. MTS Tests

Once MTS syntax is stabilized, tests must include:

minimum valid timestamp
maximum specified timestamp
Unicode
timezone
duration
interval
malformed delimiters
invalid numeric components
unterminated literal

No fixed timestamp width may be treated as a universal resource limit.

---

134. Unicode Tests

Test:

"é"
"中"
"🙂"
"𐀀"
'🙂'
|ψ⟩

and verify exact byte spans.

---

135. Source Span Tests

For every literal:

assert token.span.start
assert token.span.end
assert token.literal

must correspond exactly to source bytes.

Test both ASCII and multi-byte Unicode literals.

---

136. Determinism Tests

Lex the same source repeatedly:

source
→ tokens
→ tokens
→ tokens

and assert byte-for-byte equivalent token streams.

---

137. Scalability Tests

Test progressively larger literals:

1 digit
10 digits
100 digits
1,000 digits
10,000 digits
...

subject to test-runner resource budgets.

The test suite must not define a language maximum.

The purpose is to detect:

- quadratic behavior;
- stack overflow;
- accidental fixed-width conversion;
- excessive copying.

---

138. Cross-Domain Tests

The same literal syntax must work in:

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

where semantically applicable.

---

139. Compatibility Matrix

Literal compatibility must be tracked across:

Layer| Requirement
"lexer/literals.md"| normative literal contract
"lexer/tokens.md"| token identity
"Zamani.g4"| grammar recognition
"src/lexer.rs"| lexical implementation
"src/parser.rs"| parsing
"src/frontend/ast/"| AST representation
semantic analysis| type/value validation
classical IR| numeric constants
"quantum::ir"| quantum literals
HDL/hardware IR| hardware constants
compiler| target lowering
runtime| execution representation
tests| conformance

A literal feature is not production complete until every required row has a defined contract.

---

140. Existing "src/lexer.rs" Corrections Required

The implementation currently has literal-scanning behavior that must be brought into conformance.

The following are required:

140.1 Fix UTF-8 cursor handling

Do not use byte offsets as character indexes.

140.2 Fix token-end/span calculation

The current implementation records "end_pos" before all token advancement has necessarily completed. This must be made explicit and half-open.

140.3 Replace narrow quantum scanning

Do not consume exactly one payload character.

140.4 Implement MTS scanning

Do not leave "MTSLiteral" as a declared-but-unimplemented category.

140.5 Strengthen numeric scanning

Support:

- bases;
- separators;
- exponents;
- range ambiguity;
- future suffixes.

140.6 Strengthen string scanning

Handle:

- Unicode;
- escapes;
- invalid escapes;
- unterminated strings;
- multiline/raw forms if standardized.

140.7 Strengthen character scanning

Require exactly one Unicode scalar value.

140.8 Do not convert huge numbers to machine integers in the lexer.

140.9 Ensure all diagnostics carry accurate spans.

---

141. "grammar/Zamani.g4" Integration

The root grammar currently references literal categories such as integer and decimal literals and also includes quantum/complex literal concepts.

The canonical grammar must therefore:

1. expose exactly the literal categories defined here;
2. not introduce additional undocumented literal forms;
3. not duplicate literal rules in multiple domains;
4. keep domain-specific interpretation downstream;
5. preserve precedence around ranges/operators;
6. preserve literal source spans through parser contexts.

---

142. "grammar/lexer/tokens.md" Integration

"tokens.md" must define:

INTEGER
FLOAT
STRING
CHAR
BOOLEAN
NIL
COMPLEX
QUANTUM_LITERAL
RAW_STRING
BYTE_STRING
BYTE
MTS_LITERAL

or the exact established token names.

It must document:

- spelling;
- token kind;
- payload;
- span;
- parser consumers.

No token may exist only because it happens to be present in "src/lexer.rs".

---

143. "grammar/lexer/operators.md" Integration

Operators must explicitly document interactions with literals:

.
..
..=
+
-
/
*
%

Especially:

1.0
1..
1..=

This is necessary to prevent lexical ambiguity.

---

144. "grammar/lexer/unicode.md" Integration

Unicode specification must define:

- UTF-8 source;
- Unicode scalar values;
- escape sequences;
- Unicode delimiters;
- column semantics;
- normalization policy.

This document owns literal-specific Unicode rules.

"unicode.md" owns the general Unicode model.

---

145. "grammar/spec/lexical.md" Integration

"spec/lexical.md" must reference this document instead of duplicating every literal rule.

The hierarchy is:

spec/lexical.md
       │
       └── lexer/literals.md

This avoids contradictory copies.

---

146. "grammar/specification/lexical.md" Integration

This document should provide the human-readable normative overview.

"lexer/literals.md" contains the detailed literal contract.

The two files must not disagree.

---

147. "grammar/grammar.md" Integration

"grammar.md" should report implementation status for every literal:

stable
implemented
partial
proposed
deprecated

Example:

Integer literals       STABLE / IMPLEMENTED
Binary literals        STABLE / IMPLEMENTED
Raw strings            PROPOSED / NOT IMPLEMENTED
MTS literals           IMPLEMENTATION REQUIRED
Quantum literals       PARTIAL / IMPLEMENTATION REQUIRED

This prevents documentation from claiming functionality that "src/lexer.rs" does not actually implement.

---

148. AST Integration

The AST team must receive the complete literal contract before implementing or modifying literal AST nodes.

Required fields where appropriate:

kind
raw source
decoded value
base
prefix
suffix
span

No later grammar change should require redesigning the literal AST merely because literal metadata was omitted.

---

149. Semantic Integration

Semantic analysis owns:

- integer type inference;
- float type inference;
- conversion;
- overflow;
- underflow;
- exactness;
- quantum state validity;
- resource quantity interpretation;
- timestamp validity;
- unit compatibility;
- literal suffix semantics.

The lexer must not duplicate these rules.

---

150. Compiler Integration

The compiler must decide:

source literal
→ semantic value
→ target representation

based on:

- type;
- target;
- optimization;
- resource availability;
- backend capabilities.

The literal grammar remains unchanged.

---

151. Runtime Integration

Runtime representation may vary by target.

For example, an integer may become:

native integer
multiword integer
heap integer
symbolic value
constant pool entry

The runtime choice must never change the source literal grammar.

---

152. Hardware Integration

A literal such as:

32

must never inherently mean:

32-bit hardware

unless its semantic context explicitly gives it that meaning.

Similarly:

1024

does not inherently mean:

1024 physical qubits

or:

1024 hardware threads

---

153. Quantum Integration

A literal:

10

may become:

number of logical qubits

only through semantic context.

Quantum hardware discovery occurs downstream.

---

154. HDL Integration

A literal:

32

may become:

signal width

only through HDL semantics.

The literal itself remains an integer.

---

155. AI/Data Integration

A literal:

128

may represent:

batch size
tensor dimension
embedding dimension
layer count

depending on semantic context.

The lexer does not know which one.

---

156. POCO-REAF Guarantee

Literal syntax is portable because it describes values, not machines.

A source program may therefore contain:

let n = 1_000_000;

without requiring a machine with a fixed universal implementation width.

The compiler may choose the appropriate representation.

---

157. What “Forever” Means

POCO-REAF does not mean an old binary will literally execute forever on every future physical system.

It means the source-level semantic contract remains portable and target-independent, while compilers and runtimes provide appropriate realization for future targets.

Literal syntax therefore must not encode assumptions about today's hardware.

---

158. No Artificial Literal Limits

The grammar must not define:

MAX_LITERAL_LENGTH = ...
MAX_INTEGER_DIGITS = ...
MAX_STRING_LENGTH = ...
MAX_QUANTUM_STATE_SIZE = ...
MAX_TIMESTAMP_DIGITS = ...

as language semantics.

Implementations may have configurable resource budgets.

Those budgets belong to compiler/tooling configuration.

---

159. Hard-Coding Audit

This file passes the hard-coding rule only if it contains no universal machine limits.

Forbidden:

MAX_QUBITS
MAX_CPUS
MAX_GPUS
MAX_THREADS
MAX_FPGAS
MAX_MEMORY
MAX_NODES
MAX_LITERAL_VALUE

Allowed:

program literal = 1024

because that is program data.

---

160. Feature Manifest Integration

A future feature manifest may record:

id: literal.integer
status: stable
grammar: grammar/Zamani.g4
lexical_contract: grammar/lexer/literals.md
token_contract: grammar/lexer/tokens.md
implementation: src/lexer.rs
ast_contract: src/frontend/ast/
semantic_contract: grammar/specification/semantics.md
tests:
  - grammar/tests/lexical/
  - grammar/tests/boundary/
  - grammar/tests/scalability/

This creates the closed feature contract required for independently completable work.

---

161. Completion Criteria

"grammar/lexer/literals.md" is complete when:

- [ ] every stable literal category is defined;
- [ ] every token category has a defined owner;
- [ ] integer syntax is defined;
- [ ] binary syntax is defined;
- [ ] octal syntax is defined;
- [ ] hexadecimal syntax is defined;
- [ ] floating syntax is defined;
- [ ] range ambiguity is defined;
- [ ] digit separators are defined;
- [ ] string syntax is defined;
- [ ] character syntax is defined;
- [ ] escape syntax is defined;
- [ ] Unicode behavior is defined;
- [ ] Boolean behavior is defined;
- [ ] nil behavior is defined;
- [ ] complex literal status is defined;
- [ ] quantum literal behavior is defined;
- [ ] MTS literal status is defined;
- [ ] raw-string status is defined;
- [ ] byte-literal status is defined;
- [ ] quantity-literal status is defined;
- [ ] literal suffix status is defined;
- [ ] source-span behavior is defined;
- [ ] diagnostics are defined;
- [ ] AST contract is defined;
- [ ] semantic contract is defined;
- [ ] IR contract is defined;
- [ ] quantum::ir integration is defined;
- [ ] HDL integration is defined;
- [ ] resource semantics are defined;
- [ ] POCO-REAF constraints are defined;
- [ ] hard-coding audit passes;
- [ ] Rust 1.97/1.97.1 compatibility is defined;
- [ ] unsafe Rust is prohibited;
- [ ] positive tests exist;
- [ ] negative tests exist;
- [ ] boundary tests exist;
- [ ] Unicode tests exist;
- [ ] source-span tests exist;
- [ ] scalability tests exist;
- [ ] determinism tests exist;
- [ ] compatibility tests exist.

---

162. Required Repository Changes After This File

This document itself should be treated as the independent contract.

Only after it is accepted should the implementation work proceed in this order:

1. grammar/lexer/literals.md
        │
        ▼
2. grammar/lexer/tokens.md
        │
        ▼
3. grammar/lexer/operators.md
        │
        ▼
4. grammar/spec/lexical.md
        │
        ▼
5. grammar/Zamani.g4
        │
        ▼
6. src/lexer.rs
        │
        ▼
7. src/parser.rs
        │
        ▼
8. src/frontend/ast/
        │
        ▼
9. semantic analysis
        │
        ▼
10. canonical IR
        │
        ├── classical IR
        ├── quantum::ir
        └── HDL/hardware semantic IR
        │
        ▼
11. compiler/runtime
        │
        ▼
12. conformance tests

The order ensures that later implementation files consume an already-defined contract rather than requiring this file to be rewritten.

---

163. Final Literal Architecture

The production architecture is:

                    ZAMANI SOURCE
                          │
                          ▼
                 ┌─────────────────┐
                 │      LEXER      │
                 │                 │
                 │ integers        │
                 │ floats          │
                 │ strings         │
                 │ chars           │
                 │ booleans        │
                 │ nil             │
                 │ quantum         │
                 │ temporal        │
                 │ raw/bytes       │
                 │ future literals │
                 └────────┬────────┘
                          │
                          ▼
                       TOKENS
                          │
                          ▼
                       PARSER
                          │
                          ▼
                    DOMAIN-NEUTRAL AST
                          │
                          ▼
                  SEMANTIC ANALYSIS
                          │
             ┌────────────┼────────────┐
             ▼            ▼            ▼
        CLASSICAL      QUANTUM         HDL
           IR         quantum::ir   HW semantic IR
             │            │            │
             └────────────┼────────────┘
                          ▼
                  OPTIMIZATION / LOWERING
                          │
             ┌────────────┼────────────┐
             ▼            ▼            ▼
         ROUTING      SCHEDULING    RESILIENCE
                                       │
                                    QEC/ZQN
                                       │
                                       ▼
                                      HAL
                                       │
                                       ▼
                              TARGET REALIZATION

The fundamental invariant is:

«A literal describes source-level value semantics; it never describes an implicit hardware limit.»

Therefore the same Zamani source can express:

- tiny integer computations;
- arbitrarily large compile-time integers;
- scientific values;
- tensor dimensions;
- HDL widths;
- quantum state notation;
- resource quantities;
- temporal values;
- AI/data parameters;
- distributed-computing parameters;

without changing the language merely because the target changes from:

tiny embedded device
        ↓
CPU
        ↓
GPU
        ↓
FPGA
        ↓
ASIC
        ↓
QPU
        ↓
distributed cluster
        ↓
future computational substrate

The literal layer is therefore a portable lexical foundation, not a machine-description layer.

---

164. Final Rule

The authoritative rule for all future literal work is:

SOURCE SPELLING
      ↓
LEXICAL CATEGORY
      ↓
TOKEN + EXACT SPAN
      ↓
DOMAIN-NEUTRAL AST
      ↓
SEMANTIC VALUE
      ↓
CANONICAL IR
      ↓
TARGET-SPECIFIC REALIZATION

Never:

SOURCE SPELLING
      ↓
MACHINE-SPECIFIC VALUE

and never:

SOURCE LITERAL
      ↓
HARD-CODED HARDWARE LIMIT

That separation is mandatory for Zamani's:

Program Once → Compile Once → Run Everywhere, Anywhere, Forever (POCO-REAF)

architecture.