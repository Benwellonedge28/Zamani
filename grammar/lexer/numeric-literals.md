Zamani Numeric Literal Specification

Path: "grammar/lexer/numeric-literals.md"
Language: Zamani
Specification role: Normative numeric lexical contract
Status: Production-ready target contract
Specification version: 1.0
Implementation baseline: Rust 1.97 / Rust 1.97.1
Rust edition: Rust 2021
Safety: Safe Rust only; Rust "unsafe" is prohibited
Portability: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)
Scalability: No artificial language-level numeric or hardware limits
Canonical quantum semantic boundary: "quantum::ir"

---

0. Purpose

This document defines the normative source-level lexical contract for numeric literals in the Zamani programming language.

It defines:

- integer literal syntax;
- decimal, binary, octal, and hexadecimal integer syntax;
- floating-point literal syntax;
- exponent syntax;
- digit separators;
- numeric token boundaries;
- range/operator ambiguity;
- sign ownership;
- radix identification;
- source-spelling preservation;
- arbitrary numeric magnitude;
- arbitrary source precision;
- malformed numeric input;
- numeric diagnostics;
- numeric lexical compatibility;
- parser integration;
- AST integration;
- semantic-analysis integration;
- canonical IR integration;
- compiler/runtime integration;
- POCO-REAF requirements;
- scalability requirements;
- resource-budget separation;
- conformance and testing requirements.

This file is specifically concerned with lexical numeric syntax.

It does not decide whether a numeric value is:

- "i8";
- "i16";
- "i32";
- "i64";
- "i128";
- "u8";
- "u16";
- "u32";
- "u64";
- "u128";
- "f16";
- "f32";
- "f64";
- "f128";
- arbitrary precision;
- fixed point;
- decimal;
- rational;
- symbolic;
- tensor-valued;
- complex;
- a quantum parameter;
- a resource quantity;
- a hardware-native value.

Those decisions belong downstream.

---

1. Architectural Position

Numeric literals participate in the following pipeline:

source bytes
    │
    ▼
UTF-8 source decoding
    │
    ▼
lexical scanning
    │
    ├── numeric token kind
    ├── exact source spelling
    ├── numeric lexical metadata
    └── exact source span
    │
    ▼
parser
    │
    ▼
domain-neutral AST literal
    │
    ▼
structural validation
    │
    ▼
semantic analysis
    │
    ├── type interpretation
    ├── range checking
    ├── precision checking
    ├── constant evaluation
    └── contextual interpretation
    │
    ▼
canonical semantic model
    │
    ▼
canonical IR
    │
    ├── classical representation
    ├── quantum::ir
    ├── HDL/hardware representation
    ├── data/tensor representation
    └── other domain representations
    │
    ▼
optimization / lowering / routing / scheduling / resilience
    │
    ▼
target realization

The lexer MUST NOT skip directly from source text to a machine numeric value.

In particular, lexical recognition MUST NOT require conversion to:

i64
u64
usize
f32
f64

merely because those are convenient Rust host representations.

---

2. Authority and Integration

The numeric-literal authority chain is:

grammar/spec/lexical.md
        │
        ▼
grammar/lexer/numeric-literals.md
        │
        ▼
grammar/lexer/numeric-literals.g4
        │
        ▼
canonical lexer / token registry
        │
        ├───────────────┐
        ▼               ▼
src/lexer.rs      grammar/antlr/ZamaniLexer.g4
        │               │
        └───────┬───────┘
                ▼
           src/parser.rs
                │
                ▼
       src/frontend/ast/
                │
                ▼
        semantic analysis
                │
                ▼
          canonical IR

Related repository contracts:

File/component| Responsibility
"grammar/spec/lexical.md"| Overall lexical authority
"grammar/lexer/tokens.md"| Canonical token taxonomy
"grammar/lexer/numeric-literals.md"| Numeric lexical contract
"grammar/lexer/numeric-literals.g4"| Numeric ANTLR lexical rules
"grammar/lexer/literals.md"| Overall literal contract
"grammar/lexer/operators.md"| Operators and operator precedence
"grammar/lexer/identifiers.md"| Identifier syntax
"grammar/lexer/unicode.md"| Unicode policy
"grammar/Zamani.g4"| Canonical grammar composition root
"grammar/antlr/ZamaniLexer.g4"| Canonical ANTLR lexer assembly
"src/lexer.rs"| Executable Rust lexer
"src/parser.rs"| Parser
"src/frontend/ast/node/expressions/literal.rs"| Domain-neutral literal AST
"grammar/grammar.md"| Implementation-conformance reference
"grammar/Zamani-Grammar.md"| Broader design/history/proposals
"grammar/tests/"| Conformance testing
semantic layer| Numeric interpretation
canonical IR| Computational meaning
"quantum::ir"| Canonical quantum semantic boundary

Neither "grammar/grammar.md" nor "grammar/Zamani-Grammar.md" may silently introduce numeric syntax.

A numeric feature becomes stable only after it is incorporated into the normative specification, canonical grammar, lexer, parser, AST/semantic contract, and conformance tests.

---

3. Ownership

3.1 This file owns

This file owns:

- numeric lexical categories;
- numeric lexical spelling;
- numeric radix syntax;
- decimal integer syntax;
- binary integer syntax;
- octal integer syntax;
- hexadecimal integer syntax;
- decimal floating syntax;
- exponent syntax;
- numeric digit separators;
- numeric lexical boundaries;
- numeric/range ambiguity rules;
- numeric/operator boundary rules;
- numeric malformed-input requirements;
- numeric source-preservation requirements;
- numeric lexical diagnostics;
- numeric lexical compatibility requirements;
- numeric scalability requirements.

3.2 This file does not own

This file does not own:

- numeric type definitions;
- type inference;
- integer signedness semantics;
- integer width;
- arbitrary-precision arithmetic implementation;
- floating-point implementation;
- rounding modes;
- overflow semantics;
- underflow semantics;
- constant-folding algorithms;
- numerical libraries;
- tensor semantics;
- complex-number semantics;
- quantum amplitudes;
- QEC;
- ZQN;
- HAL;
- routing;
- scheduling;
- calibration;
- backend selection;
- hardware topology;
- device identity;
- resource allocation.

---

4. Production Invariants

Every conforming implementation MUST satisfy these invariants.

4.1 Determinism

For identical:

- source bytes;
- language version;
- compatibility mode;
- lexical configuration;

the lexer MUST produce identical:

- token kinds;
- token order;
- token source spans;
- numeric source spelling;
- numeric lexical metadata;
- diagnostics;
- diagnostic ordering.

Lexing MUST NOT depend on:

- CPU architecture;
- native integer width;
- operating-system locale;
- available memory size, except explicit resource exhaustion;
- CPU count;
- GPU availability;
- QPU availability;
- FPGA availability;
- network state;
- device topology;
- wall-clock time;
- randomness;
- hash-map iteration order.

---

5. Safe Rust Requirement

The reference Rust implementation MUST use safe Rust.

No numeric lexical operation may require:

unsafe

or:

unsafe { ... }

or:

- unsafe functions;
- unsafe traits;
- unsafe implementations;
- raw-pointer arithmetic;
- unchecked numeric conversion.

Numeric source text MUST be handled using safe Rust string/character/slice abstractions.

The language specification itself does not require any unsafe implementation technique.

---

6. Scalability and POCO-REAF

Numeric syntax MUST be independent of target-machine capacity.

There MUST NOT be language-level lexical constants such as:

MAX_INTEGER_BITS
MAX_INTEGER_DIGITS
MAX_FLOAT_DIGITS
MAX_EXPONENT
MAX_NUMERIC_LITERAL_LENGTH
MAX_NATIVE_INTEGER_WIDTH
MAX_REGISTER_WIDTH
MAX_VECTOR_WIDTH

merely to model a machine.

The following is therefore conceptually valid lexical input:

0
42
1_000
999999999999999999999999999999999999999999999999999999999999999999

The lexical layer recognizes the syntax.

Whether a semantic type or target can represent the resulting value is determined later.

---

7. Language Limits vs Resource Limits

A production implementation MAY impose explicit compilation resource budgets.

For example:

maximum source bytes
maximum compilation memory
maximum diagnostic count
maximum compilation time
maximum AST size
maximum constant-evaluation budget

Such limits are implementation resource policies, not numeric language rules.

The implementation MUST distinguish:

invalid numeric literal

from:

numeric processing resource budget exceeded

For example, a compiler may report:

ZMN-RESOURCE-NUMERIC-BUDGET

when constant evaluation exceeds a configured resource budget.

It MUST NOT report:

ZMN-LEX-NUMBER-INVALID

merely because the compiler's chosen resource budget was exhausted.

---

8. Canonical Numeric Token Categories

The canonical stable numeric token categories are:

INTEGER
FLOAT

The "INTEGER" token represents all supported integer radices.

The "FLOAT" token represents the stable floating-point lexical forms.

The radix MUST NOT require separate semantic token identities such as:

Int32
Int64
UInt64
Float32
Float64

The parser/semantic layer determines the eventual meaning.

---

9. Integer Literal Forms

Zamani supports:

decimal
binary
octal
hexadecimal

The conceptual lexical production is:

INTEGER
    : DECIMAL_INTEGER
    | BINARY_INTEGER
    | OCTAL_INTEGER
    | HEX_INTEGER
    ;

---

10. Decimal Integer Literals

Canonical syntax:

DECIMAL_INTEGER
    := DECIMAL_DIGIT_SEQUENCE

Examples:

0
1
7
42
100
1000000
123456789

Digit separators are permitted:

1_000
1_000_000
123_456_789

---

11. Binary Integer Literals

Canonical syntax:

0b BINARY_DIGIT_SEQUENCE
0B BINARY_DIGIT_SEQUENCE

Examples:

0b0
0b1
0b10
0b101010
0b1010_0101
0B1111_0000

Only:

0
1
_

may occur after the prefix, subject to separator rules.

Invalid:

0b
0b2
0b102
0b_1010
0b1010_
0b10__10

---

12. Octal Integer Literals

Canonical syntax:

0o OCTAL_DIGIT_SEQUENCE
0O OCTAL_DIGIT_SEQUENCE

Examples:

0o0
0o7
0o10
0o755
0o755_123

Valid digits are:

0 1 2 3 4 5 6 7

Invalid:

0o
0o8
0o9
0o128
0o_755
0o755_

---

13. Hexadecimal Integer Literals

Canonical syntax:

0x HEX_DIGIT_SEQUENCE
0X HEX_DIGIT_SEQUENCE

Examples:

0x0
0x1
0xFF
0xff
0xDEAD_BEEF
0XCAFE_BABE

Valid hexadecimal digits are:

0-9
a-f
A-F

Invalid:

0x
0xG
0x_FF
0xFF_
0xFF__00

---

14. Explicit Octal Policy

A leading zero MUST NOT implicitly convert a decimal integer to octal.

Therefore:

0123

is a decimal integer containing the digits "0", "1", "2", "3".

Explicit octal notation is:

0o123

This eliminates historical ambiguity between decimal and implicit-octal conventions.

---

15. Digit Separator Rules

The underscore "_" is a numeric digit separator.

A separator MUST occur between two valid digits belonging to the same numeric digit sequence.

Valid:

1_000
1_000_000

0b1010_0101

0o755_123

0xDEAD_BEEF

1_000.25
1_000.25_50

1e1_000

Invalid:

_100
100_

1__000

0x_FF
0xFF_

0b_1010
0b1010_

0o_755
0o755_

1._5
1_.5

1e_10
1e10_

1e_+10
1e+_10

The grammar MUST NOT use a permissive pattern equivalent to:

DIGIT (DIGIT | '_')*

when that pattern permits trailing or repeated separators.

---

16. Separator Scope

Separators MUST NOT cross lexical components.

For example:

0x_FF

is invalid.

The prefix:

0x

and digit sequence:

FF

are separate lexical components.

Likewise:

1._5

is invalid because "_" does not separate two digits.

And:

1e_10

is invalid because the exponent marker is not a digit.

---

17. Leading Zeroes

Multiple leading decimal zeroes are valid:

0
00
000
00042

They remain decimal integers.

The source spelling MUST be preserved.

Thus:

00042

and:

42

may have the same semantic value while remaining distinct source spellings.

This distinction matters for:

- formatting;
- source reconstruction;
- diagnostics;
- provenance;
- refactoring;
- reproducible compilation;
- source-to-source transformation.

---

18. Sign Ownership

A leading "+" or "-" is NOT part of the numeric token.

Therefore:

42

is:

INTEGER("42")

while:

-42

is conceptually:

MINUS
INTEGER("42")

and:

+42

is:

PLUS
INTEGER("42")

This applies equally to floating-point literals:

-1.5

is:

MINUS
FLOAT("1.5")

This preserves the distinction between:

- literal syntax;
- unary operators;
- constant evaluation;
- type-directed semantics.

---

19. Floating-Point Literals

The stable baseline floating-point syntax is decimal.

Supported forms include:

1.0
0.5
42.75
3.141592653589793

.5
.25

1.
1.e2

1e10
1E10

1.5e10
1.5E10

1.5e-10
1.5e+10

.5e2
.5e-2

1_000.25
1.234_567
1.0e1_000

The exact accepted forms are governed by the canonical grammar and parser contract, but the production design MUST preserve these lexical distinctions consistently.

---

20. Canonical Floating Forms

Conceptually:

FLOAT
    := DIGITS "." DIGITS? EXPONENT?
     | "." DIGITS EXPONENT?
     | DIGITS EXPONENT

where:

EXPONENT
    := ("e" | "E") SIGN? DIGITS

and:

SIGN
    := "+"
     | "-"

The exponent sign is part of the floating literal because it belongs to the exponent syntax.

The outer numeric sign remains an operator.

Thus:

-1.5e-10

is:

MINUS
FLOAT("1.5e-10")

---

21. Exponent Rules

An exponent MUST contain at least one decimal digit.

Valid:

1e0
1e1
1e10
1e+10
1e-10
1e1_000
1.5e10
.5e-2

Invalid:

1e
1e+
1e-
1e_
1e_10
1e10_
1e1__0

The exponent marker is case-insensitive:

e
E

---

22. Decimal Point Ambiguity

The lexer MUST distinguish numeric decimal points from punctuation and range operators.

For example:

1.5

is:

FLOAT

while:

1..10

must be:

INTEGER
DOTDOT
INTEGER

and:

1..=10

must be:

INTEGER
DOT_DOT_EQUALS
INTEGER

The lexer MUST NOT interpret:

1..10

as:

FLOAT("1.")
DOT
INTEGER("10")

when the language grammar recognizes ".." as a range operator.

---

23. Range Operator Precedence

Where floating-point syntax and range syntax compete for the same source prefix, the canonical lexical rule MUST preserve the longest valid range operator.

Therefore:

1..10

must recognize:

1
..
10

and:

1..=10

must recognize:

1
..=
10

This requirement integrates directly with "grammar/lexer/operators.md".

The numeric grammar MUST NOT independently redefine ".." or "..=".

---

24. Member Access Ambiguity

Numeric literals MUST also coexist deterministically with member access.

For example:

42.foo

must follow the canonical parser/operator policy.

The lexer MUST NOT silently reinterpret source according to target-language floating-point behavior.

The language specification must choose one deterministic interpretation and test it.

The numeric-literal specification therefore establishes this invariant:

«Numeric lexical recognition must never make parser-level member-access syntax ambiguous.»

---

25. Floating-Point Precision

The lexical layer imposes no universal floating-point precision.

A source literal such as:

0.123456789012345678901234567890123456789

must not be silently converted to "f64".

The semantic layer may subsequently interpret it as:

f16
f32
f64
f128
decimal
arbitrary precision
rational
symbolic

or reject it if the selected semantic type cannot represent it under the applicable language rules.

---

26. Integer Magnitude

There is no universal language-level integer-width limit.

These are lexically valid forms:

0
42
18446744073709551616
340282366920938463463374607431768211456

and arbitrarily larger values, subject only to explicit implementation resource exhaustion.

The lexer MUST NOT convert them into a fixed-width Rust integer merely to recognize them.

In particular, lexical recognition MUST NOT overflow:

i64
u64
usize
i128
u128

---

27. Source Representation of Large Numbers

The numeric token MUST preserve the exact source spelling.

For example:

1_000_000

must not become only:

1000000

before source-level tooling has had an opportunity to inspect the original representation.

At minimum, downstream lexical consumers must be able to recover:

- original spelling;
- source span;
- radix;
- digit sequence;
- exponent;
- separator positions where required.

The existing token contract explicitly requires literal metadata to support values larger than host integer types and requires source spelling preservation.

---

28. Numeric AST Integration

The domain-neutral AST is the next stage after lexical recognition.

The current literal AST already follows the required architectural principle:

«the AST represents what was written, rather than prematurely selecting a machine representation.»

It explicitly avoids forcing integer literals into "i64"/"u64" or floating literals into "f32"/"f64".

The numeric lexical contract therefore maps conceptually to:

INTEGER token
    ↓
LiteralKind::Integer {
    raw,
    radix
}

and:

FLOAT token
    ↓
LiteralKind::Float {
    raw
}

The lexer/parser MUST NOT introduce target-specific numeric AST variants such as:

Integer64
Integer128
Float64
GPUFloat
QuantumFloat
FPGAFloat

---

29. Radix Preservation

For integer literals, the AST/semantic boundary MUST preserve the source radix.

For example:

42

has radix:

Decimal

while:

0x2A

has radix:

Hexadecimal

and:

0b101010

has radix:

Binary

and:

0o52

has radix:

Octal

These values may be semantically equal, but their source representations are not identical.

---

30. Canonical Integer Radix Model

The semantic-neutral radix vocabulary is:

Decimal
Binary
Octal
Hexadecimal

The implementation MUST NOT encode radix as a machine-specific numeric representation.

A Rust implementation may safely use an enum equivalent to:

enum IntegerRadix {
    Decimal,
    Binary,
    Octal,
    Hexadecimal,
}

provided that the surrounding AST/semantic contracts remain source-oriented.

---

31. No Fixed-Width Numeric Token Types

The lexer MUST NOT create lexical categories based on width.

Prohibited universal token categories include:

INTEGER8
INTEGER16
INTEGER32
INTEGER64
INTEGER128

FLOAT16
FLOAT32
FLOAT64
FLOAT128

A type annotation may later specify a semantic width.

The literal itself remains source syntax.

---

32. Numeric Suffixes

Numeric suffixes are not part of the baseline numeric lexical contract unless separately standardized.

Examples:

42u32
42i64
1.0f32
1.0f64

must not be silently accepted merely because a downstream compiler could interpret them.

If numeric suffixes are introduced later, they require coordinated changes to:

grammar/spec/type-system.md
grammar/spec/lexical.md
grammar/lexer/numeric-literals.md
grammar/lexer/numeric-literals.g4
grammar/lexer/tokens.md
grammar/types/
grammar/Zamani.g4
src/lexer.rs
src/parser.rs
src/frontend/ast/
semantic literal handling
grammar/tests/
grammar/compatibility/

The feature is not complete until all those contracts agree.

---

33. Hardware Independence

Numeric syntax MUST NOT encode:

- CPU register width;
- GPU lane width;
- SIMD width;
- FPGA datapath width;
- QPU width;
- accelerator width;
- memory width;
- physical device format.

For example:

1024

must not inherently mean:

1024-bit hardware register

Likewise:

32

must not inherently mean:

32 CPU cores

or:

32 qubits

Its meaning comes from its semantic context.

---

34. Resource Quantities

Numeric literals may participate in resource expressions:

requires memory >= 64GiB
requires qubits >= n
requires nodes >= required_nodes
requires bandwidth >= 10Gbps

The numeric literal remains ordinary numeric syntax.

The resource system determines the meaning of:

64
10
n

and any quantity/unit syntax.

The numeric lexer must not become responsible for resource semantics.

This preserves the distinction between:

numeric value

and:

resource requirement

---

35. Units and Quantities

Units are not inherently numeric suffixes.

A future quantity such as:

64GiB
10GHz
5ns
20GB/s

must be specified as a coordinated quantity/unit feature.

The numeric lexer MUST NOT silently absorb arbitrary alphabetic suffixes.

For example:

123abc

must not automatically become one numeric token.

This is important because identifiers and numeric literals have separate lexical ownership.

---

36. Complex Numbers

Complex-number syntax is a semantic/type-system concern.

The baseline numeric lexer should not require special numeric tokens merely for complex values.

A future syntax may be expressed compositionally, for example:

3 + 4i

or by an explicitly specified literal form.

If a dedicated complex literal syntax is introduced, it must define:

- lexical grammar;
- tokenization;
- parser behavior;
- AST representation;
- semantic type;
- constant evaluation;
- formatting;
- compatibility;
- tests.

It must not force hardware-specific complex representations into the lexer.

---

37. Rational and Decimal Numbers

Exact rational or decimal semantics may be supported without changing ordinary integer/float lexical categories.

For example:

1 / 3

may be a semantic rational expression.

A future exact-decimal literal syntax must be explicitly specified.

The lexer MUST NOT assume that every decimal source spelling is an IEEE floating-point value.

---

38. Symbolic Numbers

Symbolic mathematics may use numeric literals as operands:

x^2 + 2*x + 1

The lexer only recognizes:

x
2
2
1

The symbolic mathematics subsystem determines the semantic interpretation.

This keeps "classical/", "data/", AI, scientific computing, and other domains from requiring separate numeric lexical languages.

---

39. Quantum Integration

Quantum syntax may contain numeric parameters:

rotation(theta)
rotation(3.141592653589793)

or:

phase(pi / 2)

The numeric lexer remains domain-neutral.

Numeric literals do not become:

QuantumFloat
QubitNumber
GateParameter
PhysicalPulseNumber

at lexical time.

The pipeline remains:

numeric source
    ↓
generic numeric token
    ↓
domain-neutral AST
    ↓
semantic interpretation
    ↓
quantum semantic model
    ↓
quantum::ir

"quantum::ir" remains the canonical quantum semantic boundary.

---

40. HDL Integration

HDL constructs may contain numeric values for:

- widths;
- parameters;
- timing;
- counts;
- addresses;
- dimensions;
- state encodings;
- memory descriptions.

The numeric lexer remains generic.

For example:

width = 1024

does not mean that the lexer has selected a 1024-bit hardware datapath.

That interpretation belongs to HDL/hardware semantics.

---

41. AI/Data/Tensor Integration

Numeric literals may appear in:

- tensor dimensions;
- shapes;
- training parameters;
- learning rates;
- thresholds;
- probabilities;
- dataset metadata;
- optimization parameters.

The lexer remains unaware of the consuming domain.

For example:

shape = [1024, 4096]

contains ordinary numeric literals.

The tensor/data semantic layer determines their meaning.

No fixed maximum tensor dimension may be inferred from the lexer.

---

42. Numeric Literal and Identifier Boundary

A numeric literal MUST terminate before an identifier begins unless the complete spelling is explicitly defined as part of a future numeric form.

For example:

123abc

must not silently become:

INTEGER("123abc")

It must be tokenized according to the canonical lexer boundary policy and, where appropriate, produce a diagnostic.

Likewise:

0xFFvalue

must not silently become an arbitrary hexadecimal number.

Malformed numeric continuations must be diagnosed deterministically.

---

43. Malformed Numeric Input

Malformed numeric source MUST NOT be silently reinterpreted as another valid number.

Examples include:

0x
0b
0o

0xG
0b2
0o8

1e
1e+
1e-

1__0
0x__FF
0b_101
0o755_

1._5
1_.5

The implementation may tokenize malformed input into smaller lexical units where required for recovery, but the complete compiler pipeline MUST produce a deterministic diagnostic identifying the malformed numeric construct.

---

44. Invalid Digit Diagnostics

The compiler should distinguish errors such as:

0b102

from:

0xGG

and:

0o89

where practical.

Diagnostics should identify:

- numeric base;
- offending character;
- source span;
- expected digit class;
- language version;
- relevant recovery suggestion where available.

Example:

ZMN-LEX-NUMBER-INVALID-DIGIT
invalid digit '2' in binary integer literal
expected '0' or '1'

The exact diagnostic identifier must be centralized with the repository diagnostic system rather than duplicated ad hoc.

---

45. Invalid Separator Diagnostics

Examples:

1__000
0x_FF
0b101_
1e_10

should produce a separator-specific diagnostic where practical.

Example:

ZMN-LEX-NUMBER-INVALID-SEPARATOR
numeric separator must occur between digits

The lexer MUST preserve the exact span of the invalid separator.

---

46. Unterminated Numeric Constructs

Numeric syntax has no quote-like terminator, but incomplete prefixes must still be diagnosed.

Examples:

0x
0b
0o

must not silently become:

0
x

or:

0
b

without a diagnostic if the source context establishes an intended numeric prefix.

The canonical lexer/error policy determines the exact recovery representation.

---

47. Error Recovery

Error recovery MUST be deterministic.

For malformed numeric input, the lexer should:

1. identify the earliest unambiguous numeric error;
2. preserve the source span;
3. emit a structured diagnostic;
4. consume enough input to make forward progress;
5. avoid infinite loops;
6. avoid silently changing the programmer's value;
7. continue lexing where safe.

Recovery MUST NOT depend on target hardware.

---

48. Overflow and Underflow

Overflow and underflow are NOT lexical validity conditions.

For example:

999999999999999999999999999999999999999999999999

is lexically valid.

Whether it fits a selected semantic type is determined later.

Likewise:

1e1000000

may be lexically valid even if a selected target representation cannot represent it.

The semantic layer determines whether the value is:

- representable;
- rounded;
- infinite;
- rejected;
- represented symbolically;
- represented with arbitrary precision.

The lexer must not silently replace it with another value.

---

49. Constant Evaluation

Constant evaluation occurs after parsing.

The lexical layer MUST NOT:

- evaluate arithmetic;
- simplify expressions;
- calculate powers;
- evaluate exponents;
- fold unary signs;
- infer types;
- select numeric widths.

For example:

2 ^ 1000

contains numeric literals and an operator.

Constant folding is a semantic/compiler responsibility.

---

50. Arbitrary Precision

Zamani's architecture must permit arbitrary-precision semantic representations.

This does NOT mean every compiler must allocate unbounded memory.

It means:

«The language syntax itself must not impose a finite machine-derived maximum.»

A compiler may impose an explicit resource budget.

The distinction is:

language validity

versus:

implementation resource availability

This distinction is fundamental to POCO-REAF.

---

51. Source-Length Scalability

The numeric grammar MUST NOT encode an arbitrary source-length limit.

A sufficiently large source environment may therefore contain:

very_small_number

or:

very_large_number

according to available compilation resources.

If an implementation cannot process a particular source due to resource exhaustion, it must report resource exhaustion rather than claim that the numeric syntax itself is invalid.

---

52. Streaming and Incremental Lexing

The numeric lexical design must be compatible with:

- complete-file lexing;
- streaming lexing;
- incremental lexing;
- IDE lexing;
- formatter lexing;
- syntax highlighting;
- language-server tooling.

A numeric token MUST have a stable source span regardless of whether the surrounding source is processed incrementally.

The implementation must not depend on loading the entire program into a fixed-size numeric buffer merely to recognize a numeric literal.

---

53. Parallel Compilation

Numeric lexical results must be deterministic under parallel compilation.

The result MUST NOT change because files are lexed:

sequentially

versus:

in parallel

Diagnostics must have deterministic ordering according to the compiler's global diagnostic ordering policy.

Numeric token identity itself must never depend on thread scheduling.

---

54. Unicode and Numeric Digits

The baseline numeric grammar uses ASCII digits:

0-9

for numeric literals.

Unicode decimal characters MUST NOT automatically become numeric digits merely because Unicode classifies them as decimal characters.

For example, the language must not accidentally treat visually similar Unicode digits as equivalent numeric syntax.

If Unicode numeric digit syntax is ever introduced, it requires an explicit lexical specification and compatibility analysis.

This preserves deterministic source interpretation.

---

55. Locale Independence

Numeric literals are locale-independent.

The lexer MUST NOT interpret:

1,5

as:

1.5

because the host operating system uses a comma decimal separator.

The decimal point is defined by the language specification.

Likewise:

1,000

does not acquire locale-dependent numeric meaning.

Thousands separators are represented by "_" where permitted.

---

56. No Locale-Dependent Formatting

Numeric parsing and semantic interpretation MUST NOT depend on:

- locale;
- regional decimal conventions;
- operating-system number formatting;
- user language settings.

This is necessary for reproducible compilation.

---

57. Canonical Examples

Valid integers

0
1
42
00042
1_000
1_000_000
0b0
0b1010
0b1010_0101
0o0
0o755
0o755_123
0x0
0xFF
0xDEAD_BEEF

Valid floats

0.0
1.0
42.75
.5
.25
1.
1.e2
1e10
1E10
1.5e10
1.5e-10
1.5e+10
.5e2
1_000.25
1.234_567
1.0e1_000

Invalid integers

0x
0b
0o
0xG
0b2
0o8
0x_FF
0b_1010
0o_755
1__000
1_000_

Invalid floats

1e
1e+
1e-
1e_10
1e10_
1e1__0
1._5
1_.5

---

58. Range Examples

The lexer MUST preserve range syntax:

0..10
1..=10
100..200

as range expressions.

The numeric portion is:

INTEGER

not:

FLOAT

for the first boundary.

Tests MUST explicitly cover:

0..1
0..=1
1..10
1..=10
1_000..2_000
0x10..0x20

---

59. Member and Method Examples

Numeric literal boundaries must be tested around:

1.foo
1.0.foo
42.to_string()
1.5.to_string()

The parser specification must establish the precise interpretation.

The lexer must not create target-language-specific behavior.

---

60. Future Hexadecimal Floating-Point Support

The architecture reserves an extension point for hexadecimal floating-point syntax such as:

0x1.fp4
0x1.8p+2
0x1.8p-2

This syntax is NOT considered stable merely because the architecture reserves it.

If adopted, it must be added to:

grammar/spec/lexical.md
grammar/lexer/numeric-literals.md
grammar/lexer/numeric-literals.g4
grammar/lexer/tokens.md
grammar/spec/type-system.md
grammar/Zamani.g4
src/lexer.rs
src/parser.rs
AST/semantic literal handling
tests
compatibility
grammar.md

The hexadecimal floating exponent uses "p"/"P", not "e"/"E".

---

61. Future Numeric Suffix Extension

The architecture also reserves a coordinated extension point for suffixes.

Potential examples:

42u32
42i64
1.0f32
1.0f64

No suffix may be accepted accidentally.

Suffixes must not be interpreted by the numeric lexer as arbitrary identifier text.

When standardized, the implementation must distinguish:

numeric lexical syntax

from:

semantic type selection

and must preserve POCO-REAF.

---

62. No Vendor Numeric Syntax in Core

The core numeric grammar MUST NOT become a collection of vendor-specific formats.

The following must remain downstream or extension-level concepts unless explicitly standardized:

CUDA-specific numeric forms
ROCm-specific numeric forms
vendor QPU numeric formats
FPGA vendor numeric formats
CPU vendor numeric formats
accelerator-specific numeric encodings

A backend may lower a generic semantic numeric value to an appropriate target representation.

---

63. No Quantum-Specific Numeric Token Types

The lexer MUST NOT create token kinds such as:

QubitIndex
QuantumAngle
QuantumAmplitude
PulseAmplitude
GateParameter
PhysicalQubitNumber
LogicalQubitNumber

unless a future language specification explicitly establishes them as distinct lexical constructs.

Ordinary numeric syntax is sufficient for ordinary numeric parameters.

---

64. No HDL-Specific Numeric Token Types

Likewise, the lexer MUST NOT create:

RegisterWidth
SignalWidth
ClockFrequency
MemoryDepth
FPGAAddress

as ordinary numeric token categories.

These are semantic interpretations of numeric values in HDL/hardware contexts.

---

65. Feature Closure Contract

This file is considered complete only when the following contracts already exist or are explicitly integrated:

numeric lexical specification
        ↓
numeric-literals.g4
        ↓
canonical lexer token
        ↓
src/lexer.rs
        ↓
src/parser.rs
        ↓
LiteralKind::Integer / Float
        ↓
semantic numeric representation
        ↓
canonical IR

No later file should need to redefine the basic numeric lexical rules.

A downstream file may extend semantics, but it must not silently modify this file's lexical contract.

---

66. AST Contract

The numeric lexer provides:

token kind
source spelling
radix metadata where applicable
source span

The AST provides:

LiteralKind::Integer {
    raw,
    radix
}

or:

LiteralKind::Float {
    raw
}

The AST does not choose machine width.

This is consistent with the current domain-neutral literal AST, whose stated contract is to preserve source representation and defer numeric interpretation to semantic analysis.

---

67. Semantic Contract

Semantic analysis is responsible for determining:

- numeric type;
- signedness;
- precision;
- scale;
- range;
- overflow;
- underflow;
- rounding;
- exactness;
- constant-evaluation behavior;
- contextual meaning;
- domain-specific meaning.

For example:

42

may eventually become:

i32
i64
u64
u128
BigInt
resource quantity
tensor dimension
quantum parameter

depending on context.

The lexer remains unchanged.

---

68. Canonical IR Contract

Numeric lexical information must not become a second computational IR.

The flow is:

source spelling
    ↓
AST literal
    ↓
semantic numeric value/type
    ↓
canonical IR

For quantum computation:

numeric source
    ↓
domain-neutral AST
    ↓
semantic quantum parameter
    ↓
quantum::ir

The numeric lexer must never depend directly on:

- "quantum::ir";
- QEC;
- ZQN;
- HAL;
- routing;
- scheduling.

---

69. Compiler Integration

The compiler may choose target-specific representations after semantic analysis.

For example:

source:
    42

could be lowered to different target representations depending on context.

The lexical result must remain identical.

Likewise:

1.5

must be capable of being lowered differently for:

- CPU;
- GPU;
- FPGA;
- QPU control parameters;
- accelerator;
- distributed execution;
- software simulation.

This is one of the mechanisms enabling POCO-REAF.

---

70. Runtime Integration

The runtime MUST NOT reinterpret lexical numeric spelling.

Runtime receives values or IR produced by earlier compiler stages.

The runtime may discover available:

- memory;
- compute;
- accelerators;
- quantum resources;
- network resources;

but these do not alter the lexical interpretation of the source.

---

71. Tooling Integration

Numeric source spelling must remain available to:

- formatter;
- syntax highlighter;
- LSP;
- IDE;
- diagnostics;
- refactoring;
- source maps;
- documentation tools;
- code generation;
- reproducibility tooling.

For example:

1_000_000

must remain distinguishable from:

1000000

at the source representation layer.

---

72. Serialization and Reproducibility

Numeric semantic representations may be serialized after semantic analysis.

However, source-level tooling must be able to preserve:

raw spelling
radix
source span
language version
compatibility mode

where required.

Compilation must not produce different semantic results merely because the source was formatted differently, provided the two spellings have the same defined language semantics.

---

73. Security

Numeric parsing is an untrusted-input boundary.

The implementation MUST protect against:

- integer overflow;
- allocation amplification;
- pathological exponent processing;
- pathological constant folding;
- denial-of-service through enormous numeric literals;
- malformed UTF-8;
- malformed separators;
- parser/lexer infinite loops.

Security/resource defenses must be explicit and must not change language validity.

---

74. Resource-Budget Separation

A compiler may expose policies such as:

numeric lexical budget
constant evaluation budget
maximum semantic numeric allocation
maximum compile-time computation budget

These belong to compiler policy.

They MUST NOT be embedded into the language grammar as universal limits.

This permits the same program to compile on increasingly capable systems without rewriting the source.

---

75. Deterministic Resource Failure

If a numeric resource budget is exceeded, the compiler must produce a deterministic diagnostic.

For example:

ZMN-RESOURCE-CONSTANT-EVALUATION
constant evaluation exceeded the configured compilation budget

rather than silently:

- truncating;
- rounding;
- wrapping;
- substituting zero;
- substituting infinity;
- changing radix;
- changing type.

---

76. Compatibility

Numeric syntax is versioned.

A language version may:

- add a new numeric form;
- deprecate a form;
- clarify an ambiguity;
- introduce a suffix;
- introduce hexadecimal floating-point syntax.

Such changes require explicit compatibility handling.

Existing valid source MUST NOT silently acquire a different meaning without a documented language-version rule.

---

77. Deprecated Numeric Forms

If a legacy numeric spelling exists, compatibility handling must distinguish:

accepted and stable
accepted but deprecated
accepted only in compatibility mode
rejected
reserved for future use

The lexer/parser must not silently reinterpret deprecated syntax as a different numeric value.

---

78. Conformance Matrix

Every numeric form must be tested across:

Feature| Lexer| Parser| AST| Semantic| Tests
Decimal integer| required| required| required| required| required
Binary integer| required| required| required| required| required
Octal integer| required| required| required| required| required
Hex integer| required| required| required| required| required
Decimal float| required| required| required| required| required
Exponent| required| required| required| required| required
Separators| required| required| required| required| required
Range boundary| required| required| required| required| required
Huge integer| required| required| required| required| required
Huge exponent| required| required| required| required| required
Invalid digit| required| required| diagnostic| diagnostic| required
Invalid separator| required| required| diagnostic| diagnostic| required

---

79. Positive Tests

At minimum, lexical tests MUST include:

0
1
42
00042
1_000
1_000_000

0b0
0b1
0b1010
0b1010_0101

0o0
0o7
0o755
0o755_123

0x0
0xFF
0xDEAD_BEEF

0.0
1.0
.5
1.
1.e2

1e0
1e10
1e+10
1e-10

1.5e10
1.5e-10
.5e2

1_000.25
1.234_567
1.0e1_000

---

80. Negative Tests

At minimum:

0x
0b
0o

0xG
0b2
0o8

0x_FF
0b_1010
0o_755

0xFF_
0b1010_
0o755_

1__000
1._5
1_.5

1e
1e+
1e-
1e_10
1e10_
1e1__0

must be tested.

---

81. Boundary Tests

Boundary tests MUST include:

0
largest practical compiler-tested integer
very large integer
many separators
zero exponent
positive exponent
negative exponent
very large exponent
very small decimal magnitude
leading decimal point
trailing decimal point
range boundary
identifier boundary
member-access boundary

The test suite must not establish a maximum numeric size as a language rule.

---

82. Scalability Tests

Scalability tests MUST demonstrate that numeric syntax does not impose fixed machine widths.

Test values should include progressively larger representations.

For example:

42
18446744073709551616
340282366920938463463374607431768211456

and substantially larger test fixtures where practical.

The tests should verify:

- no host integer overflow during lexing;
- source spelling preserved;
- radix preserved;
- deterministic tokenization;
- deterministic diagnostics;
- no hardware-derived limit.

---

83. Cross-Domain Tests

Numeric literals must be tested in:

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
resource
compile
execution
interoperability

contexts.

The same numeric lexical form must retain the same lexical identity across domains.

Only semantic context changes.

---

84. Quantum Tests

At minimum:

rotation(0.0)
rotation(1.5707963267948966)
phase(-3.141592653589793)
apply gate(theta)

must demonstrate that numeric literals remain generic source literals.

Tests must not introduce a fixed number of qubits or machine-specific numeric width.

---

85. HDL Tests

At minimum:

width = 1
width = 8
width = 32
width = 1024
width = 1_000_000

must be treated as ordinary numeric source syntax.

The lexer must not infer that:

1024

means a fixed universal hardware width.

---

86. Resource Tests

Test:

requires qubits >= 1
requires qubits >= 1024
requires memory >= 1
requires memory >= 1000000

The numeric lexer must recognize ordinary integers.

Resource interpretation remains downstream.

---

87. POCO-REAF Acceptance Test

A conformance implementation passes the numeric POCO-REAF requirement when:

1. numeric source syntax is target-independent;
2. no machine width is encoded into the lexical grammar;
3. source spelling is preserved;
4. arbitrary numeric magnitude is lexically representable;
5. semantic representation is selected downstream;
6. target-specific representation occurs downstream;
7. compilation resource budgets are separate from language validity;
8. the same source can flow toward different targets without lexical rewriting.

---

88. Hard-Coding Audit

This file passes the hard-coding audit only if it contains no universal numeric machine assumptions such as:

MAX_INTEGER_BITS
MAX_INTEGER_DIGITS
MAX_FLOAT_BITS
MAX_FLOAT_DIGITS
MAX_REGISTER_WIDTH
MAX_VECTOR_WIDTH
MAX_GPU_WIDTH
MAX_QPU_WIDTH
MAX_TENSOR_DIMENSION

as language-level restrictions.

The following are permitted:

explicit lexical syntax
explicit language-version rules
explicit compiler resource budgets
explicit semantic type widths
explicit program constants

provided they are not confused with universal hardware limits.

---

89. Required Repository Integration

The implementation work associated with this document must integrate with, but not unnecessarily rename, the existing files:

grammar/lexer/numeric-literals.g4
grammar/lexer/literals.md
grammar/lexer/tokens.md
grammar/spec/lexical.md
grammar/specification/lexical.md
grammar/Zamani.g4
grammar/antlr/ZamaniLexer.g4
src/lexer.rs
src/parser.rs
src/frontend/ast/node/expressions/literal.rs
grammar/grammar.md
grammar/Zamani-Grammar.md
grammar/tests/

No second numeric authority may be introduced.

---

90. Required Changes to "numeric-literals.g4"

"grammar/lexer/numeric-literals.g4" must implement this contract.

It must:

- own numeric lexical fragments;
- use one canonical integer token;
- use one canonical floating token;
- support decimal/binary/octal/hexadecimal integers;
- enforce valid separators;
- enforce required digits after prefixes;
- enforce valid exponent structure;
- preserve range-operator behavior;
- avoid signs as part of numeric tokens;
- avoid machine-width assumptions;
- avoid numeric suffixes until separately standardized;
- avoid domain-specific numeric token kinds.

The current repository's numeric grammar already follows much of this architecture, including separate decimal/binary/octal/hexadecimal integer forms and explicit rejection of fixed-width lexical assumptions.

---

91. Required Changes to "src/lexer.rs"

"src/lexer.rs" is the executable lexical implementation.

It must:

- recognize the canonical numeric forms;
- preserve exact source spelling;
- preserve source spans;
- avoid fixed-width numeric conversion during lexical scanning;
- recognize radix;
- reject malformed separators;
- reject invalid radix digits;
- handle range/floating ambiguity deterministically;
- emit deterministic diagnostics;
- use safe Rust only.

The current lexer already uses a "Token" containing "token_type", "literal", and "span", and has distinct "Integer" and "Float" token categories.

Those existing concepts should be completed rather than replaced by another token architecture.

---

92. Required AST Integration

"src/frontend/ast/node/expressions/literal.rs" already provides the correct architectural direction:

LiteralKind::Integer {
    raw,
    radix
}

and:

LiteralKind::Float {
    raw
}

The numeric implementation must continue to preserve source spelling rather than introducing machine-width values into the AST.

No unnecessary AST redesign is required solely because this numeric specification is being added.

---

93. Required "grammar/lexer/literals.md" Integration

"grammar/lexer/literals.md" remains the umbrella literal specification.

It should reference this file for numeric details rather than duplicating the complete numeric grammar.

Its numeric section should establish:

numeric literals
    ↓
grammar/lexer/numeric-literals.md

This avoids two numeric authorities.

The existing literals document already defines arbitrary-size integer intent, numeric source preservation, and the separation between lexical recognition and semantic interpretation.

---

94. Required "grammar/spec/lexical.md" Integration

"grammar/spec/lexical.md" remains the higher-level lexical authority.

It should state that detailed numeric rules are owned by:

grammar/lexer/numeric-literals.md

It must not redefine individual numeric productions in a conflicting form.

Its role is:

overall lexical policy

while this file provides:

numeric lexical detail

---

95. Required Token Integration

"grammar/lexer/tokens.md" owns token identity.

Therefore this file MUST NOT redefine token enumeration.

It only requires the canonical tokens:

INTEGER
FLOAT

or their established repository equivalents.

The token contract already requires source spelling preservation and metadata capable of handling values larger than host integer types.

---

96. Required Parser Integration

The parser must receive:

INTEGER
FLOAT

tokens and construct the domain-neutral literal AST.

It must not parse a numeric token directly into a machine numeric type.

Parser responsibilities include:

- syntactic placement;
- expression precedence;
- unary sign;
- ranges;
- contextual interpretation.

Semantic numeric conversion remains downstream.

---

97. Required Semantic Integration

The semantic layer must consume the raw spelling and radix metadata.

It is responsible for:

source numeric
    ↓
semantic numeric representation

This may involve arbitrary precision or target-independent mathematical representations.

Any conversion that can overflow must be checked.

No unchecked conversion is permitted.

---

98. Required Compiler Integration

Compiler lowering may choose:

native integer
arbitrary precision
floating-point format
fixed point
decimal
symbolic
accelerator-specific
quantum-specific
HDL-specific

according to semantic context and target capabilities.

That decision must happen after lexical analysis.

---

99. Required Runtime Integration

The runtime consumes compiler-generated representations.

It must not reinterpret the original numeric token according to hardware.

Runtime resource availability can affect execution strategy but cannot alter source lexical meaning.

---

100. Completion Criteria

"grammar/lexer/numeric-literals.md" is complete when all of the following are true:

- [ ] Decimal integers specified.
- [ ] Binary integers specified.
- [ ] Octal integers specified.
- [ ] Hexadecimal integers specified.
- [ ] Decimal floats specified.
- [ ] Exponents specified.
- [ ] Digit separators specified.
- [ ] Leading/trailing separators rejected.
- [ ] Repeated separators rejected.
- [ ] Invalid radix digits diagnosed.
- [ ] Numeric sign ownership specified.
- [ ] Range ambiguity specified.
- [ ] Member-access boundary specified.
- [ ] Source spelling preservation specified.
- [ ] Radix preservation specified.
- [ ] Arbitrary lexical magnitude specified.
- [ ] No fixed integer width imposed.
- [ ] No fixed floating width imposed.
- [ ] No hardware assumptions.
- [ ] No locale dependence.
- [ ] No Unicode-digit ambiguity.
- [ ] Resource limits separated from language validity.
- [ ] Deterministic diagnostics specified.
- [ ] Safe-Rust requirement specified.
- [ ] AST contract specified.
- [ ] Semantic contract specified.
- [ ] IR contract specified.
- [ ] Compiler contract specified.
- [ ] Runtime contract specified.
- [ ] Tooling contract specified.
- [ ] Quantum integration specified.
- [ ] HDL integration specified.
- [ ] AI/data integration specified.
- [ ] Resource integration specified.
- [ ] Positive tests specified.
- [ ] Negative tests specified.
- [ ] Boundary tests specified.
- [ ] Scalability tests specified.
- [ ] Cross-domain tests specified.
- [ ] Compatibility requirements specified.
- [ ] Hard-coding audit specified.

---

101. Final Numeric Architecture

The production numeric architecture is:

                    Zamani source
                         │
                         ▼
             grammar/spec/lexical.md
                         │
                         ▼
          lexer/numeric-literals.md
                         │
                         ▼
         lexer/numeric-literals.g4
                         │
                         ▼
               canonical lexer
                         │
                  ┌──────┴──────┐
                  ▼             ▼
              INTEGER         FLOAT
                  │             │
                  └──────┬──────┘
                         ▼
                       parser
                         │
                         ▼
                 domain-neutral AST
                         │
                  ┌──────┴──────┐
                  ▼             ▼
             Integer(raw,       Float(raw)
             radix)
                  │             │
                  └──────┬──────┘
                         ▼
                 semantic analysis
                         │
                         ▼
              target-independent
               numeric meaning
                         │
                         ▼
                  canonical IR
                         │
        ┌────────────────┼─────────────────┐
        ▼                ▼                 ▼
    classical       quantum::ir       HDL/hardware
        │                │                 │
        └────────────────┼─────────────────┘
                         ▼
              optimization / lowering
                         │
             ┌───────────┼────────────┐
             ▼           ▼            ▼
          routing    scheduling    resilience
                         │
                         ▼
                    target HAL
                         │
          ┌──────────────┼──────────────┐
          ▼              ▼              ▼
         CPU            GPU            QPU
          │              │              │
          └──────────────┼──────────────┘
                         ▼
                 actual realization

The governing principle is:

«A numeric literal describes source-level numeric intent, not the width, representation, or capacity of the machine that eventually executes it.»

Therefore:

42

is not inherently "i32".

18446744073709551616

is not rejected merely because a host implementation has a 64-bit integer.

1.2345678901234567890123456789

is not silently reduced to "f64".

0xDEAD_BEEF

does not select a CPU register width.

1024

does not mean 1024 qubits, cores, bits, nodes, or tensor elements until its semantic context says so.

This separation is what allows the numeric lexical layer to remain stable while Zamani expands across classical computing, quantum computing, HDL, AI, data, distributed systems, accelerators, networking, scientific computing, and future computational paradigms.

Completion boundary: once this document, "numeric-literals.g4", the canonical lexer, parser, literal AST, semantic numeric representation, and their conformance tests satisfy the contracts above, downstream files may consume the numeric contract without redefining or retroactively editing its lexical meaning.