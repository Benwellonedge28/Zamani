Zamani Token Specification

Path: "grammar/lexer/tokens.md"
Language: Zamani
Specification role: Normative lexical token contract
Status: Production-ready target contract
Specification version: 1.0
Implementation baseline: Rust 1.97 / Rust 1.97.1
Edition: Rust 2021
Safety: Safe Rust only; Rust "unsafe" is prohibited
Portability: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)
Scalability: No artificial language-level resource or hardware limits
Canonical quantum semantic boundary: "quantum::ir"

---

0. Purpose

This document defines the canonical token model of the Zamani programming language.

It establishes the contract between:

source bytes
    ↓
UTF-8 decoding
    ↓
Unicode scalar values
    ↓
lexical scanning
    ↓
canonical tokens
    ↓
lexical diagnostics
    ↓
parser
    ↓
domain-neutral AST
    ↓
semantic analysis
    ↓
canonical semantic model
    ↓
canonical IR
    ↓
optimization / lowering / routing / scheduling / resilience
    ↓
target realization

This file owns the lexical identity and behavior of tokens.

It does not own:

- AST node definitions;
- type semantics;
- name resolution;
- ownership;
- borrowing;
- effect semantics;
- quantum semantics;
- QEC;
- ZQN;
- routing;
- scheduling;
- calibration;
- HAL;
- optimization algorithms;
- backend selection;
- runtime behavior;
- deployment;
- physical hardware topology.

The lexer must recognize syntax.

It must not decide what a token ultimately means to a particular hardware target.

---

1. Authority and Integration

The token architecture must have exactly one normative lexical contract.

The authority relationship is:

grammar/spec/lexical.md
        │
        ▼
grammar/lexer/tokens.md
        │
        ├───────────────┐
        │               │
        ▼               ▼
canonical token      canonical lexical
registry             spelling rules
        │               │
        └───────┬───────┘
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
       semantic analysis
                │
                ▼
         canonical IR

The existing repository documents the lexical specification as the normative lexical contract and identifies "src/lexer.rs" as the executable Rust implementation. This file refines that contract at the token level.

The following files integrate with this document:

File| Responsibility
"grammar/spec/lexical.md"| Normative lexical architecture
"grammar/lexer/tokens.md"| Canonical token taxonomy and token contracts
"grammar/lexer/keywords.md"| Keyword registry and keyword policy
"grammar/lexer/operators.md"| Operator spelling, precedence-facing tokenization
"grammar/lexer/identifiers.md"| Identifier syntax
"grammar/lexer/literals.md"| Literal forms
"grammar/lexer/comments.md"| Comments
"grammar/lexer/unicode.md"| Unicode policy
"grammar/lexer/quantum-literals.md"| Quantum literal forms
"grammar/Zamani.g4"| Canonical ANTLR parser composition
"src/lexer.rs"| Safe Rust lexical implementation
"src/parser.rs"| Parser consumption of tokens
"src/ast/"| Domain-neutral AST
"grammar/grammar.md"| Implementation-conformance reference
"grammar/Zamani-Grammar.md"| Broader design/history/proposals
"grammar/tests/"| Conformance tests
semantic layer| Semantic interpretation
canonical IR| Computational representation
"quantum::ir"| Canonical quantum semantic boundary

Neither "grammar/grammar.md" nor "grammar/Zamani-Grammar.md" may silently introduce token kinds.

---

2. Token Design Principles

Every token must satisfy the following principles.

2.1 Stable identity

Each language-level token category has one canonical identity.

Different Rust enum variants must not represent the same language token merely because they were introduced at different times.

For example, the following must not remain independent canonical concepts:

Arrow
ThinArrow

if both represent:

->

Likewise:

Question
QuestionMark

must not both represent:

?

unless they are deliberately distinguished by lexical context and that distinction is documented.

The preferred production architecture is:

one source spelling
        ↓
one canonical token kind
        ↓
parser context determines semantic role

---

3. Canonical Token Model

A token conceptually contains:

Token {
    kind
    source_span
    source_text
    value_metadata?
}

The executable representation may differ internally, but the observable contract must preserve these properties.

3.1 Token kind

"kind" identifies the lexical category.

Examples:

Identifier
IntegerLiteral
FloatLiteral
StringLiteral
CharLiteral
Plus
Minus
Arrow
KeywordFn
KeywordIf
...

Token kind must not contain target-specific information.

---

3.2 Source span

Every token must carry an exact source span.

The span must identify the original source region that produced the token.

The span must be expressed using the repository's existing source-map abstraction rather than inventing a second position model.

The existing Rust lexer already attaches "Span" to tokens.

The token contract therefore requires:

Token
 ├── kind
 ├── source spelling
 └── Span

The span must not be reconstructed later by the parser.

---

3.3 Source spelling preservation

The lexical layer must preserve the original source representation needed by downstream diagnostics, formatting, tooling, and semantic literal processing.

For example:

1_000
0xFF
0b1010
3.14e-10
"hello"

must not be reduced prematurely to a host-language value.

The token layer may additionally provide normalized or decoded metadata, but it must not discard the source representation required for diagnostics.

---

4. Token Categories

The canonical token taxonomy consists of:

1. End-of-input
2. Identifiers
3. Keywords
4. Contextual keywords
5. Boolean/null literals
6. Numeric literals
7. Character literals
8. String literals
9. Quantum literals
10. Attribute/annotation syntax
11. Punctuation
12. Operators
13. Delimiters
14. Documentation/comments
15. Error/recovery tokens

The categories are deliberately semantic rather than hardware-specific.

---

5. End-of-Input Token

Canonical token

EOF

Spelling

No source spelling.

Semantics

Represents the end of the source input.

Requirements

"EOF":

- occurs exactly once in a complete token stream;
- has a deterministic source position;
- must not consume source bytes;
- must not be confused with an invalid character;
- must be emitted even for an empty source;
- must be independent of source size.

The parser uses "EOF" to terminate compilation-unit parsing.

---

6. Identifier Token

Canonical token

Identifier

Spelling

An identifier consists of:

IDENTIFIER_START IDENTIFIER_CONTINUE*

The exact Unicode character profile is defined by:

grammar/lexer/identifiers.md
grammar/lexer/unicode.md

Examples:

x
value
_qubit
qubit_count
QuantumState
π
résultat
状態
данные

Invalid examples:

1value
42abc

---

7. Identifier and Keyword Separation

Lexical recognition must follow this conceptual sequence:

source spelling
      ↓
identifier-shaped lexical unit
      ↓
keyword registry lookup
      ↓
reserved keyword OR contextual keyword OR Identifier

The Rust implementation may use a keyword map for efficiency, but the map must be derived from the canonical registry rather than becoming the language authority.

The current implementation uses a "HashMap<String, TokenType>" for keyword lookup. That is an implementation mechanism, not the specification.

---

8. Keywords

Keywords are divided into:

ReservedKeyword
ContextualKeyword
CompatibilityKeyword

8.1 Reserved keyword

A reserved keyword cannot be used as an ordinary identifier in the applicable language version.

Examples include core constructs such as:

fn
let
var
const
if
else
for
while
return
match
module
import
export
type
struct
enum
trait
impl

---

8.2 Contextual keyword

A contextual keyword is recognized as a special word only where the parser grammar requires it.

This is preferred for vocabulary that may grow with future computing domains.

Examples may include:

parallel
resource
capability
requires
ensures
constraint
preference
hint
quantum
hardware

provided their final language specification defines them contextually.

This prevents every domain concept from becoming permanently reserved.

---

8.3 Compatibility keyword

A compatibility keyword may remain lexically recognized because historical Zamani programs use it, while the current language may discourage or deprecate it.

The parser/semantic/diagnostic layer determines whether it is:

accepted
accepted-with-warning
deprecated
rejected

The lexer must remain deterministic.

---

9. Canonical Keyword Families

The following families are supported by the language architecture.

They are not permission to add every word as a reserved keyword.

9.1 Core declarations

fn
let
var
const
type
struct
record
enum
class
interface
trait
impl

9.2 Modules

package
module
import
export
use
from
as

9.3 Visibility

pub
public
private
protected
internal

9.4 Type/generic system

where
self
Self

"Self" should be handled according to the final identifier/keyword policy; the language must not accidentally create two incompatible representations.

9.5 Control flow

if
else
for
in
while
loop
break
continue
match
case
when
return
yield

9.6 Concurrency

async
await
spawn
parallel

9.7 Error/effect handling

try
catch
finally
throw
effect
effects
handle
perform
with

9.8 Contracts

requires
ensures
invariant

9.9 Classical types

bool
char
int
uint
float
f16
f32
f64
f128
str
String
byte
unit
void
never

These are lexical spellings only.

Their type semantics belong to:

grammar/types/
grammar/spec/type-system.md
semantic analysis

---

10. Boolean Literals

Canonical tokens:

TrueLiteral
FalseLiteral

Canonical spellings:

true
false

They must never simultaneously become ordinary identifiers.

---

11. Null-like Literal

Canonical stable spelling:

nil

If:

null

remains for compatibility, it must have a documented compatibility status.

It must not accidentally have a different semantic meaning merely because it has a different token enum variant.

Preferred model:

nil
 └── NullLiteral

null
 └── Compatibility spelling of NullLiteral

---

12. Integer Literal

Canonical token:

IntegerLiteral

The token represents the lexical form, not the final machine integer representation.

Supported forms include:

0
42
1_000_000
0xFF
0b1010
0o755

Potential future forms must be added through the numeric-literal specification without changing the meaning of existing literals.

---

13. Integer Scalability

The lexer must not require every integer to fit into:

i32
i64
u32
u64
usize
isize

A source literal may be much larger.

For example:

999999999999999999999999999999999999999999999999999999999999

must remain lexically representable.

The lexer preserves the digit sequence.

Semantic analysis determines:

- signedness;
- mathematical value;
- requested type;
- arbitrary precision;
- compile-time evaluability;
- target representation.

This is mandatory for POCO-REAF.

---

14. Numeric Separators

Underscore separators may occur between digits.

Valid:

1_000
0xFF_A0
0b1010_0101
1_000.25
1e1_000

provided the corresponding numeric grammar permits the form.

Invalid:

_100
100_
0x_FF
0b_101

The exact placement rules belong to:

grammar/lexer/literals.md

The token contract only guarantees that invalid placement produces a deterministic lexical diagnostic.

---

15. Floating-Point Literal

Canonical token:

FloatLiteral

Examples:

1.0
0.5
3.14159
1e9
1.5e-9
1_000.25
.5

The lexer must not silently convert the literal to "f32" or "f64".

The token represents syntax.

The semantic/type layers determine representation.

---

16. Character Literal

Canonical token:

CharLiteral

The lexical layer owns:

- delimiters;
- escape syntax;
- Unicode scalar validity;
- unterminated-character diagnostics.

The lexical layer does not decide the target representation.

A character literal must represent a Unicode scalar value unless a future language version explicitly defines another form.

---

17. String Literal

Canonical token:

StringLiteral

The lexical layer owns:

- delimiters;
- escape sequences;
- interpolation boundaries where applicable;
- termination;
- Unicode handling;
- source preservation.

It does not decide:

- allocation strategy;
- encoding at runtime;
- storage location;
- memory placement;
- target ABI.

---

18. Raw Strings

If raw strings are supported, they must have a dedicated documented lexical form.

A raw string token must not be confused with an ordinary string token.

Example conceptual forms:

StringLiteral
RawStringLiteral

The exact spelling must be defined by:

grammar/lexer/literals.md

before implementation.

---

19. Quantum Literal

The current Rust lexer has a "QuantumLiteral" token.

The production contract is:

QuantumLiteral

for syntactically atomic quantum-state literal forms.

Examples:

|0⟩
|1⟩
|+⟩
|-⟩

Potential generalized forms may include:

|ψ⟩
|state⟩

subject to the quantum literal grammar.

The lexer does not determine:

- state-vector dimension;
- number of qubits;
- physical qubit identity;
- quantum device;
- gate decomposition;
- error correction;
- noise;
- routing;
- scheduling.

Those belong downstream.

---

20. Quantum Literal Scalability

Quantum literal syntax must not impose:

MAX_QUBITS
MAX_AMPLITUDES
MAX_REGISTER_SIZE
MAX_STATE_VECTOR_SIZE

A literal's mathematical/semantic size may be large.

Available resources determine whether compilation or execution is feasible.

That distinction is mandatory:

language validity
        ≠
available execution resources

---

21. Nano Annotation

The current lexer contains:

NanoAnnotation

with examples such as:

@atom
@molecule

This category must be treated carefully.

The canonical attribute syntax in "Zamani.g4" is already:

'@' qualifiedName

Therefore the production architecture must not require a separate lexical token for every nano annotation.

Preferred model:

@
identifier

or:

@
qualifiedName

with semantic interpretation later.

Consequently:

@atom
@molecule
@bionano.material

are preferably lexed compositionally as:

At
Identifier
...

rather than becoming an ever-growing list of lexical tokens.

"NanoAnnotation" may remain in "src/lexer.rs" temporarily for compatibility, but it must not be part of the canonical long-term token taxonomy unless a lexical distinction is proven necessary.

---

22. MTS Literals

The current Rust lexer contains:

MTSLiteral

for forms such as:

mts[timestamp]

This must not become a universal opaque lexical token.

The production architecture is:

mts
[
expression
]

or the equivalent canonical grammar defined by the MTS specification.

This is preferable because:

- timestamps may be expressions;
- MTS values may evolve;
- the parser should retain structure;
- semantic validation belongs outside the lexer;
- arbitrary precision and future temporal types remain possible.

Therefore "MTSLiteral" is a compatibility implementation token, not a required canonical token, unless a future lexical specification explicitly proves that the entire construct is indivisible.

---

23. Attribute Marker

Canonical token:

At

Spelling:

@

Used by:

@attribute
@namespace.attribute
@attribute(...)
@attribute {...}

The token does not determine the attribute's meaning.

The parser handles structure.

Semantic analysis handles interpretation.

---

24. Hash Marker

Canonical token:

Hash

Spelling:

#

If the repository retains hash-prefixed syntax for compatibility, it must be documented separately from the canonical attribute syntax.

The token must not automatically imply a cryptographic hash.

Lexical spelling and semantic meaning are separate.

---

25. Delimiters

Canonical delimiter tokens are:

LParen       (
RParen       )

LBracket     [
RBracket     ]

LBrace       {
RBrace       }

Comma        ,
Dot          .
Semicolon    ;
Colon        :

Additional delimiters may be introduced only through the canonical lexical specification.

Delimiters must have one canonical token identity per spelling.

---

26. Canonical Operators

The production token set includes:

Plus
Minus
Star
Slash
Modulo

Assign

Equals
NotEquals

LessThan
LessThanEqual
GreaterThan
GreaterThanEqual

LogicalAnd
LogicalOr

BitAnd
BitOr
Caret

LeftShift
RightShift

Not
Tilde

Question

Arrow
FatArrow

DoubleColon

DotDot
DotDotEq

PlusAssign
MinusAssign
StarAssign
SlashAssign

The exact precedence and parser role are defined by:

grammar/lexer/operators.md
grammar/expressions/precedence.md
grammar/Zamani.g4
src/parser.rs

---

27. Canonical Operator Deduplication

The current Rust lexer contains overlapping token names including:

Arrow
ThinArrow
BitAnd
Ampersand
BitOr
Pipe
Question
QuestionMark

These must not remain duplicate canonical language concepts without a documented lexical distinction.

The production canonicalization is:

Source spelling| Canonical token
"->"| "Arrow"
"=>"| "FatArrow"
"&"| "BitAnd"
"|"| "BitOr"
"?"| "Question"
"::"| "DoubleColon"
".."| "DotDot"
"..="| "DotDotEq"
"<<"| "LeftShift"
">>"| "RightShift"
"&&"| "LogicalAnd"
"||"| "LogicalOr"
"=="| "Equals"
"!="| "NotEquals"
"<="| "LessThanEqual"
">="| "GreaterThanEqual"
"+="| "PlusAssign"
"-="| "MinusAssign"
"*="| "StarAssign"
"/="| "SlashAssign"

Compatibility aliases may temporarily exist inside implementation code, but they must not appear as separate language-level tokens.

---

28. "&" Must Not Be Split Arbitrarily

The spelling:

&

is one canonical token:

BitAnd

The parser/contextual grammar determines whether it is used for:

- bitwise conjunction;
- reference syntax;
- another explicitly specified language construct.

If a future version requires a lexical distinction, it must introduce an explicit specification and tests.

The lexer must not create multiple token meanings solely because different parser productions currently consume the same spelling.

---

29. "|" Must Not Be Split Arbitrarily

The spelling:

|

is:

BitOr

while:

||

is:

LogicalOr

A lambda delimiter or pattern operator must be resolved through parser context or explicitly specified tokenization.

The lexer must not create duplicate token kinds for the same spelling without a real lexical distinction.

---

30. Question Mark

The canonical token for:

?

is:

Question

The parser may use it for:

- optional types;
- conditional expressions;
- postfix optional/error propagation;
- future contextual constructs.

Its meaning belongs to syntax/semantics.

"QuestionMark" must not be a second canonical token for the same spelling.

---

31. Arrow

The canonical token for:

->

is:

Arrow

This may be used by:

fn f() -> T

and function types:

(T) -> U

The existing "ThinArrow" implementation name should be treated as a compatibility alias if retained.

There must be one canonical source spelling and one canonical language token.

---

32. Fat Arrow

The canonical token:

FatArrow

represents:

=>

It is used by constructs such as match arms where defined by the grammar.

---

33. Range Operators

Canonical tokens:

DotDot
DotDotEq

represent:

..
..=

Longest-match ordering is mandatory:

..=   → DotDotEq
..    → DotDot
.     → Dot

The lexer must never tokenize:

..=

as:

Dot
Dot
Assign

when the longest valid token is available.

---

34. Assignment Operators

Canonical tokens:

Assign
PlusAssign
MinusAssign
StarAssign
SlashAssign

represent:

=
+=
-=
*=
/=

Additional compound assignment operators must be introduced through the operator specification rather than through ad-hoc lexer changes.

---

35. Longest-Match Rule

The lexer must use deterministic longest-match behavior.

Examples:

>=  → GreaterThanEqual
>   → GreaterThan

<=  → LessThanEqual
<   → LessThan

==  → Equals
=   → Assign

!=  → NotEquals
!   → Not

&&  → LogicalAnd
&   → BitAnd

||  → LogicalOr
|   → BitOr

<<  → LeftShift
<   → LessThan

>>  → RightShift
>   → GreaterThan

->  → Arrow
-   → Minus

=>  → FatArrow
=   → Assign

..= → DotDotEq
..  → DotDot
.   → Dot

This rule is mandatory for deterministic lexing.

---

36. No Hardware-Specific Tokens

The lexer must never introduce tokens such as:

CPU0
GPU0
QPU0
FPGA0
CORE0
QUBIT0
NODE0
MEMORY_BANK0

as universal language constructs.

Hardware identity is not lexical language semantics.

Hardware-specific identifiers, when explicitly required, remain identifiers or structured resource specifications.

---

37. No Artificial Resource Tokens

The language must not create tokens such as:

MAX_QUBITS
MAX_CPUS
MAX_GPUS
MAX_THREADS
MAX_NODES
MAX_MEMORY
MAX_TENSOR_DIM
MAX_REGISTER_WIDTH

A program may contain literal values such as:

1000
1024
1_000_000

but those are program data.

The compiler/runtime may impose resource budgets.

Those budgets are not lexical tokens.

---

38. Resource and Capability Vocabulary

Resource-related concepts should normally be represented compositionally:

requires capability("quantum.measurement")
requires resource(...)
requires memory(...)
requires communication(...)

rather than introducing one keyword per device.

This permits:

CPU
GPU
FPGA
QPU
TPU
NPU
DSP
future accelerator

without changing the lexical architecture for every new device.

---

39. Quantum Gate Names Are Not Keywords

The lexer must not require a permanent token for every quantum gate.

These should normally remain identifiers:

H
X
Y
Z
S
T
CNOT
CZ
SWAP
Toffoli
custom_gate
vendor.operation

The parser recognizes the surrounding quantum operation syntax.

Semantic resolution determines whether the identifier denotes:

- a standard operation;
- a user operation;
- a library operation;
- a dialect operation;
- a vendor operation;
- a future operation.

This preserves extensibility and avoids a fixed gate universe.

---

40. Mathematical Functions Are Not Keywords

The lexer must not contain one token per mathematical operation.

Examples that should normally remain identifiers:

sin
cos
tan
exp
log
sqrt
fft
ifft
svd
qr
eigen
gradient
integrate
differentiate
optimize

Their meaning belongs to libraries, intrinsics, semantic capabilities, or domain specifications.

---

41. AI/ML Vocabulary Is Not Automatically Reserved

These should normally remain identifiers unless a specific construct gives them language-level syntax:

model
dataset
optimizer
adam
transformer
attention
embedding
inference
training
loss
gradient
agent

Some existing words such as "model", "agent", "learn", "infer", "remember", and similar concepts exist in the current lexer. Their long-term reserved/contextual status must be controlled through the canonical keyword registry rather than by continuously expanding the Rust enum.

---

42. HDL Vocabulary Is Not Automatically Reserved

HDL syntax may require contextual words such as:

module
signal
port
wire
register
clock
reset
generate

But arbitrary hardware vocabulary must remain compositional.

The lexer must not become a catalog of every HDL primitive, FPGA primitive, ASIC cell, or vendor component.

---

43. Networking Vocabulary Is Not Automatically Reserved

Protocol names such as:

TCP
UDP
HTTP
QUIC
RDMA
MPI
InfiniBand
Ethernet

should normally remain identifiers or structured names.

The lexer must not need modification every time a new protocol appears.

---

44. Cryptographic Vocabulary Is Not Automatically Reserved

Algorithm names such as:

AES
SHA256
SHA3
BLAKE3
RSA
ECDSA
Ed25519
Kyber
Dilithium

should normally remain identifiers.

The lexical layer must not become a cryptographic registry.

---

45. Vendor Vocabulary

Vendor-specific identifiers must remain extensible.

Examples:

vendor.operation
vendor.device
vendor.capability
vendor.intrinsic

must be representable without changing the core lexer.

This is essential to POCO-REAF.

---

46. Unicode Symbols

Zamani may support Unicode symbols where they provide genuine language-level value.

Examples already associated with the broader language design include:

π
Σ
Π
⟩

However, Unicode symbols must not be introduced merely because a domain has a convenient notation.

Every Unicode punctuation/operator token must have:

- canonical spelling;
- normalization policy;
- source preservation;
- parser integration;
- diagnostic behavior;
- compatibility policy;
- tests.

---

47. Sigma and Pi

The current Rust lexer contains:

SigmaSymbol
PiSymbol

These should remain canonical only if the language specification defines:

Σ
Π

as lexical symbols.

If they are ordinary mathematical identifiers/operators rather than syntax, they should instead be handled as identifiers or contextual syntax.

The token contract must not preserve implementation-specific special cases without language-level justification.

---

48. Keyword "unsafe"

The Rust implementation currently has:

KeywordUnsafe

The production Zamani language is designed to prohibit unsafe language constructs.

Therefore:

unsafe

may remain lexically reserved so that the compiler can issue a deterministic safety diagnostic.

It must not provide an executable escape hatch that bypasses the language safety model.

This is separate from the implementation requirement:

Rust unsafe

which is prohibited entirely.

---

49. Error Token

Canonical token:

Illegal

or an equivalent canonical lexical-error representation.

The preferred production behavior is:

valid token
+
diagnostic

or:

InvalidToken
+
diagnostic

rather than silently skipping invalid source.

An invalid source character must never be silently transformed into a valid token.

---

50. Error Recovery

Lexical recovery must be deterministic.

After an invalid sequence, the lexer should:

1. identify the offending source span;
2. produce a structured lexical diagnostic;
3. consume a deterministic amount of input;
4. continue when safe;
5. avoid infinite loops;
6. preserve subsequent valid tokens whenever possible.

Recovery must not alter valid source semantics.

---

51. Diagnostics

Tokenization diagnostics must identify:

diagnostic code
severity
message
primary span
optional secondary spans
optional notes
optional suggestions

Examples:

ZMN-LEX-INVALID-CHARACTER
ZMN-LEX-INVALID-UTF8
ZMN-LEX-UNTERMINATED-STRING
ZMN-LEX-UNTERMINATED-CHAR
ZMN-LEX-INVALID-NUMBER
ZMN-LEX-INVALID-ESCAPE
ZMN-LEX-INVALID-IDENTIFIER
ZMN-LEX-RESOURCE-LIMIT

A resource exhaustion diagnostic must never masquerade as invalid syntax.

---

52. Resource-Bounded Lexing

Production implementations may impose operational budgets such as:

maximum source bytes
maximum token count
maximum diagnostic count
maximum lexer memory
maximum compilation time

These are compiler resource policies, not language token limits.

They must be explicit and observable.

For example:

resource budget exceeded

is different from:

invalid token

The same source must remain linguistically valid even if one machine cannot process it under a particular resource budget.

---

53. No Token-Count Ceiling in Language Semantics

There is no language-level:

MAX_TOKENS

A program containing:

10
10^6
10^9

tokens is subject only to practical compiler/resource constraints.

The lexer architecture must therefore support streaming or bounded-memory implementations where practical.

---

54. Streaming Compatibility

The canonical token contract must not require the entire source file to be loaded into memory.

An implementation may use:

whole-source lexing
incremental lexing
streaming lexing
chunked lexing
editor/incremental lexing

provided all implementations produce identical observable tokenization for identical inputs and configurations.

The Rust implementation must remain safe Rust.

---

55. Incremental Lexing

Incremental implementations must preserve:

- token identity;
- source spans;
- diagnostics;
- lexical state;
- deterministic ordering.

An incremental lexer must not create a different language merely because input was supplied in chunks.

---

56. Parallel Lexing

Parallel lexing is an implementation optimization, not a language feature.

If a compiler partitions source input for parallel processing, it must preserve exactly the same token stream and diagnostics as sequential lexing.

Parallelism must never depend on:

- CPU count;
- execution order;
- hash iteration order;
- operating system;
- hardware topology.

---

57. Token Ordering

Tokens must be emitted in monotonically increasing source order.

For tokens:

T1
T2
...
Tn

their spans must satisfy:

start(T1) <= start(T2) <= ... <= start(Tn)

except for "EOF", which has zero-width end-of-source position.

---

58. Zero-Width Tokens

The lexer must distinguish:

source-consuming tokens

from:

zero-width markers

"EOF" is zero-width.

Synthetic parser recovery markers must not be confused with source tokens.

The lexer must never invent source text that does not exist.

---

59. Comments and Documentation

Comments are lexical input but normally do not become parser-visible semantic tokens.

The canonical policy is defined by:

grammar/lexer/comments.md

Documentation comments may be preserved as:

Documentation

tokens or lexical metadata if they are consumed by documentation tooling.

The implementation must not silently lose documentation comments if the language promises documentation extraction from source.

---

60. Whitespace

Whitespace normally separates lexical units but does not become ordinary parser tokens.

Whitespace may be significant only where the language specification explicitly defines it as syntax.

The lexer must handle:

- ASCII whitespace;
- supported Unicode whitespace;
- line endings;
- CRLF;
- LF;
- CR if supported.

Line-ending behavior must be deterministic across platforms.

---

61. Comments and Operators Must Not Conflict

The lexer must define longest-match behavior for combinations involving:

/
/
*

and any future comment syntax.

For example, if:

//
/*
*/

are comments, those forms must be recognized before treating "/" as "Slash" where applicable.

---

62. Identifier Keyword Case Sensitivity

Zamani keywords are case-sensitive unless a future language version explicitly states otherwise.

Therefore:

fn

is not equivalent to:

FN
Fn
fN

Likewise identifiers preserve case.

The lexer must not apply locale-sensitive case folding.

---

63. Locale Independence

Lexing must not depend on host locale.

The lexer must not use locale-specific rules for:

- case conversion;
- decimal separators;
- Unicode classification;
- identifier recognition;
- whitespace.

For example:

1.5

must not become invalid merely because the host locale uses a comma as a decimal separator.

---

64. Source Encoding and BOM

If UTF-8 BOM support is provided, it must be explicitly defined.

A BOM must not become an ordinary identifier or illegal character unexpectedly.

The behavior must be identical on every supported platform.

---

65. Token Values Must Not Encode Targets

A token's value must not contain:

CPU architecture
GPU model
QPU model
device ID
physical qubit
memory bank
network node
thread ID
core ID
FPGA resource location

Such information belongs to later target/resource models.

---

66. Token Values and Arbitrary Precision

Literal token metadata must be capable of preserving values larger than host integer types.

The lexer may store source text as the authoritative representation.

It must never silently overflow.

This is especially important for:

- integer literals;
- tensor dimensions;
- resource quantities;
- quantum counts;
- timestamps;
- large identifiers;
- cryptographic constants.

---

67. No "usize" Semantic Dependence

"usize" may be used internally for indexing buffers in Rust.

It must not define Zamani language value limits.

For example:

array[huge_index]

is a semantic program construct.

The language does not inherit a semantic integer limit merely because the implementation uses "usize" to index a Rust string or slice.

---

68. Source Spans Must Be Byte-Safe

The existing implementation uses source-map types including:

BytePos
FileId
Span

and this architecture should be retained rather than creating a second span system.

Spans must:

- refer to the original source;
- remain valid for UTF-8 boundaries;
- never split a Unicode scalar value;
- remain deterministic;
- be consumable by diagnostics;
- integrate with AST source spans.

---

69. Token Stream Contract With "src/parser.rs"

The parser must consume canonical tokens.

The current parser explicitly matches token variants such as:

Assign
PlusAssign
MinusAssign
StarAssign
SlashAssign
DotDot
DotDotEq
LogicalOr
KeywordOr
LogicalAnd
KeywordAnd
Pipe
Caret
BitAnd
Ampersand
Equals
NotEquals
LessThan
LessThanEqual
GreaterThan
GreaterThanEqual
LeftShift
RightShift
Plus
Minus
Star
Slash
Modulo
LParen
LBracket
Dot

which demonstrates why the token model needs canonicalization before the lexer/parser boundary becomes permanent.

The production contract requires the parser-facing vocabulary to be stable and non-duplicative.

---

70. Parser Integration Rule

The parser must never need to know whether a token came from:

whole-source lexer
incremental lexer
streaming lexer
editor lexer
ANTLR conformance lexer

All conforming lexers produce the same canonical token contract.

---

71. ANTLR Integration

"grammar/Zamani.g4" is the canonical grammar composition root.

ANTLR lexical implementation, if retained, must produce the same conceptual token vocabulary.

If:

grammar/antlr/ZamaniLexer.g4

exists, it must be treated as an implementation/conformance artifact rather than a competing specification.

The ANTLR lexer must not silently define token meanings absent from:

grammar/spec/lexical.md
grammar/lexer/tokens.md

---

72. Rust Integration

"src/lexer.rs" remains the executable Rust lexer.

The current implementation already defines a "TokenType" enum and "Token" containing:

token_type
literal
span

This overall architecture can remain, but the enum must converge on the canonical token taxonomy defined here.

The implementation must use:

Rust 1.97 / Rust 1.97.1
Rust 2021
safe Rust

and must contain no:

unsafe

construct.

---

73. Canonical Rust Naming

The Rust implementation should use stable canonical names corresponding to language tokens.

Recommended examples:

Eof
Identifier
IntegerLiteral
FloatLiteral
CharLiteral
StringLiteral
BooleanLiteral
NullLiteral

LParen
RParen
LBracket
RBracket
LBrace
RBrace

Comma
Dot
Semicolon
Colon

Plus
Minus
Star
Slash
Percent

Assign
PlusAssign
MinusAssign
StarAssign
SlashAssign

Equals
NotEquals
LessThan
LessThanEqual
GreaterThan
GreaterThanEqual

LogicalAnd
LogicalOr
BitAnd
BitOr
Caret

LeftShift
RightShift

Not
Tilde
Question

Arrow
FatArrow
DoubleColon

DotDot
DotDotEq

At
Hash

QuantumLiteral

Illegal

The exact Rust naming style may remain aligned with existing project conventions, but the language-level identity must be one-to-one.

---

74. Compatibility Aliases

During migration, old Rust variants may temporarily exist as aliases/adapters.

Examples:

ThinArrow → Arrow
QuestionMark → Question
Ampersand → BitAnd
Pipe → BitOr

Such aliases must not:

- be emitted independently;
- appear in public language documentation as separate tokens;
- create parser ambiguity;
- produce different semantic meanings.

Migration may happen in a controlled compatibility step.

---

75. Keyword Registry Integration

"grammar/lexer/keywords.md" must own the detailed registry.

Each entry must specify:

Spelling
Canonical token
Category
Reserved/contextual status
Language version
Feature status
Parser contexts
Compatibility status
Deprecated status
AST relevance
Semantic owner
Tests

"tokens.md" defines the token category.

"keywords.md" defines the complete word inventory.

This prevents duplication.

---

76. Operators Integration

"grammar/lexer/operators.md" owns:

spelling
canonical token
precedence
associativity
parser contexts
compound-operator relationships
compatibility aliases

This file only establishes token identity.

---

77. Identifier Integration

"grammar/lexer/identifiers.md" owns:

identifier start
identifier continuation
Unicode profile
keyword interaction
confusables
bidi controls
normalization

"tokens.md" only establishes that the resulting lexical unit is:

Identifier

when it is not classified as a keyword.

---

78. Literal Integration

"grammar/lexer/literals.md" owns:

integer syntax
floating syntax
character syntax
string syntax
escape syntax
raw strings
numeric separators
future numeric forms

"tokens.md" establishes the resulting token kinds.

---

79. Quantum Integration

"grammar/lexer/quantum-literals.md" owns the detailed lexical forms for quantum literals.

"grammar/quantum/" owns quantum syntax.

The lexer must not own:

quantum::ir
QEC
ZQN
routing
scheduling
HAL
calibration
physical qubits

This preserves the established architecture in which "quantum::ir" remains the canonical quantum semantic boundary.

---

80. AST Integration

Every token that can produce syntax must have a predetermined downstream mapping.

The contract is:

Token
  ↓
Parser production
  ↓
AST node
  ↓
Semantic model
  ↓
Canonical IR

No token may be added with:

"AST mapping will be decided later"

The token may map to an existing generic AST construct.

For example:

Identifier

can participate in:

generic operation names
library functions
quantum operations
HDL components
AI models
network protocols
vendor extensions

without creating new lexer tokens.

---

81. Domain-Neutral Token Requirement

Tokens must be reusable across domains.

The same:

Identifier
IntegerLiteral
StringLiteral
At
Dot
Colon
Comma
Arrow

can participate in:

classical
quantum
hybrid
HDL
AI
data
distributed
networking
security
embedded
scientific
accelerator
future domains

This is essential for one unified language.

---

82. Dialect Integration

Dialect-specific syntax must not silently modify core token meanings.

A dialect may introduce:

new contextual keywords
new operators
new literal forms

only through an explicit dialect contract.

The dialect must declare:

name
version
token extensions
grammar extensions
semantic extensions
AST mapping
IR mapping
compatibility
feature gate

Core token meanings remain stable.

---

83. Macros Integration

Macros must receive canonical token streams.

Macro expansion must not bypass lexical validation.

Macro-generated source/token streams must be subject to the same syntactic and semantic contracts.

The macro system may manipulate tokens, but it must not redefine core token identity.

---

84. Metaprogramming Integration

Compile-time reflection and code generation may inspect:

token kind
source spelling
source span
AST structure
semantic metadata

but must not rely on undocumented Rust enum implementation details.

The public compiler-facing contract is the canonical token model.

---

85. Security Requirements

The lexer must defend against:

- malformed UTF-8;
- invalid Unicode scalars;
- bidi controls;
- confusable identifiers;
- unterminated strings;
- pathological numeric literals;
- pathological escape sequences;
- lexer recovery loops;
- tokenization ambiguity;
- accidental denial-of-service behavior.

Security validation must remain separate from semantic interpretation.

---

86. No Silent Source Mutation

The lexer must never silently:

- normalize identifiers;
- change numeric values;
- change case;
- remove meaningful characters;
- reinterpret invalid UTF-8;
- replace invalid bytes with U+FFFD;
- rewrite quantum literals;
- change source spans.

Source transformation belongs to explicit tooling stages.

---

87. Determinism

For identical:

source bytes
language version
lexical configuration
compatibility mode

the lexer must produce identical:

token kinds
token order
token spans
source spellings
diagnostic codes
diagnostic ordering

It must not depend on:

CPU
GPU
QPU
FPGA
CPU count
thread count
OS locale
wall clock
randomness
network
hardware topology
hash iteration order

---

88. Reproducibility

The same source must tokenize identically on:

small machine
large machine
CPU
GPU-assisted compiler
cloud
edge
embedded host
distributed build
future target

provided the language version and lexical configuration are the same.

This is one of the lexical foundations of POCO-REAF.

---

89. Scalability Principle

There is no language-level maximum for:

source length
token count
identifier length
integer magnitude
string size
quantum register size
tensor dimensions
resource quantities
timeline count
node count
device count

Practical compiler limits may exist as resource policies.

Those limits must not change whether source syntax is valid.

---

90. Tiny-to-Large Principle

The exact same lexical rules must work for:

x

and:

a computation spanning billions of source tokens

subject only to implementation resources.

No second lexical language may be introduced for large programs.

---

91. Hardware Independence

Lexical behavior must not depend on target hardware.

The lexer must not query:

CPU count
GPU count
QPU count
FPGA count
RAM size
VRAM size
network topology
device IDs

to decide how source is tokenized.

---

92. Compilation Independence

The token stream represents source language syntax.

Compilation decisions occur later:

tokens
 ↓
AST
 ↓
semantic model
 ↓
IR
 ↓
optimization
 ↓
resource/capability analysis
 ↓
target selection
 ↓
routing/scheduling
 ↓
backend

The lexer must not perform target selection.

---

93. Quantum Independence

Quantum syntax must remain portable.

The lexer must never encode:

physical qubit 0
physical qubit 1
device 17
QPU model
coupling map
gate duration
calibration
error rate

Those belong downstream.

---

94. HDL Independence

HDL tokens express source syntax.

They do not encode:

FPGA LUT count
ASIC cell count
BRAM count
DSP count
routing resource count
clock frequency limit
pin count

Those belong to hardware/resource analysis.

---

95. AI Independence

The lexer must not depend on:

GPU architecture
tensor-core count
VRAM capacity
specific ML framework
specific model architecture

AI source vocabulary remains portable.

---

96. Distributed Independence

The lexer must not encode:

MAX_NODES
NODE_0
NODE_1
MAX_PROCESSES
MAX_THREADS

Node counts and placement belong to resource/deployment semantics.

---

97. Networking Independence

The lexer must not encode fixed topology.

The following remain data or identifiers:

endpoint
address
port
protocol
service
route

The compiler/runtime resolves actual infrastructure.

---

98. Error Recovery and Forward Compatibility

When an unknown future feature is encountered, the lexer should produce a deterministic result.

If the spelling is syntactically identifier-like:

future_keyword

should normally remain:

Identifier

until the language version makes it reserved.

This provides source compatibility and reduces keyword collisions.

---

99. Versioning

Token changes must be versioned.

A change is classified as:

non-breaking
contextual expansion
deprecated
compatibility-only
breaking

Changing:

Identifier → Keyword

is potentially breaking and must require an explicit language-version policy.

Changing:

duplicate implementation token → canonical token

is an implementation migration unless source-visible behavior changes.

---

100. Deprecation

Deprecated token spellings must have:

deprecated_since
replacement
diagnostic
removal_policy
compatibility_mode

The lexer must not silently change the meaning of a deprecated spelling.

---

101. Token Registry Requirements

The production repository should maintain a canonical registry containing, for every token:

ID
Name
Spelling
Category
Lexical precedence
Longest-match relationships
Keyword status
Contextual status
Version introduced
Version deprecated
Compatibility aliases
Parser consumers
AST mapping
Semantic owner
Test coverage

The registry should be machine-checkable.

---

102. Suggested Stable Token IDs

If stable numeric IDs are required for serialization or tooling, they must be explicitly assigned.

Do not derive public token identity from:

Rust enum ordinal
HashMap order
source-file ordering
compiler build ordering

A future enum reordering must not silently change serialized token identity.

Prefer symbolic stable names unless numeric persistence is genuinely required.

---

103. Token Serialization

If tokens are serialized for tooling, caches, incremental compilation, or diagnostics, serialization must include:

language version
token kind
source span
source representation/version metadata

Serialized token streams must not depend on Rust internal enum layout.

---

104. Token Cache Compatibility

Cached token streams must be invalidated when any relevant input changes:

language version
lexical configuration
keyword registry
token definitions
Unicode policy
literal grammar

The cache key must not be based solely on source bytes if lexical configuration can affect tokenization.

---

105. Formatter Integration

The formatter must consume canonical token kinds.

It must not inspect obsolete implementation aliases.

Formatting must preserve:

- semantic tokens;
- comments/documentation where promised;
- literal meaning;
- source spans where applicable.

---

106. Syntax Highlighting Integration

Syntax highlighting should derive classifications from:

canonical token kind

rather than maintaining an independent keyword list.

This avoids divergence between:

compiler
editor
LSP
documentation

---

107. LSP Integration

The language server should use canonical token kinds for:

- semantic highlighting;
- completion;
- hover;
- diagnostics;
- code actions;
- navigation.

A new keyword should therefore propagate through the registry rather than requiring unrelated manual lists.

---

108. Documentation Generation

Documentation generators may consume documentation tokens and canonical identifiers.

They must not infer language syntax from Rust enum names.

---

109. Test Integration

Every canonical token must have tests for:

positive recognition
negative recognition
longest match
source span
Unicode
compatibility
versioning
diagnostics
boundary cases
scalability
determinism

---

110. Minimum Token Test Matrix

At minimum:

empty source
single identifier
Unicode identifier
keyword
keyword prefix
identifier resembling keyword
integer
large integer
hex integer
binary integer
octal integer
float
scientific notation
string
escaped string
unterminated string
character
escaped character
quantum literal
attribute
operator
compound operator
range
arrow
fat arrow
comment
documentation comment
invalid Unicode
invalid character
EOF

---

111. Operator Collision Tests

The test suite must explicitly verify:

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

-
->

=
=>

.
..
..=

?

No pair may tokenize ambiguously.

---

112. Keyword Collision Tests

Every keyword must be tested against:

keyword
keyword_suffix
keyword_prefix
keyword_with_digit
Unicode_similar_identifier

For example:

fn
fnx
function
fn2

must be classified deterministically.

---

113. Unicode Tests

Test:

π
Σ
Π
状态
данные
résultat
emoji where permitted
combining marks
confusable identifiers
bidi controls
invalid scalars

The lexical contract must define which are accepted and which are diagnosed.

---

114. Quantum Tests

Test:

|0⟩
|1⟩
|+⟩
|-⟩
|ψ⟩

plus malformed variants.

The lexer must not interpret physical quantum resources.

---

115. Scalability Tests

Tests must include progressively larger:

identifier
integer
string
source file
token stream
quantum expression
tensor expression
resource expression

until practical CI resource boundaries are reached.

The tests must distinguish:

language rejection

from:

test/compiler resource exhaustion

---

116. Hard-Coding Audit

The lexical implementation must be audited for accidental constants such as:

MAX_QUBITS
MAX_CPUS
MAX_GPUS
MAX_FPGAS
MAX_NODES
MAX_THREADS
MAX_MEMORY
MAX_TENSOR_DIM
MAX_IDENTIFIER_LENGTH
MAX_INTEGER_WIDTH
MAX_SOURCE_SIZE

A compiler operational budget is allowed.

A language-level semantic limit is not.

---

117. Allowed Implementation Bounds

Safe implementation bounds are allowed when they protect the host process, for example:

diagnostic budget
memory budget
compilation time budget
input transport limit
editor rendering limit

They must be represented as implementation configuration and diagnostics.

They must not change token semantics.

---

118. Rust Safety

The lexer implementation must compile with:

Rust 1.97
Rust 1.97.1

and must contain no:

unsafe

code.

No dependency may require the lexical subsystem to use unsafe language constructs.

Where standard-library APIs internally use optimized implementation details, that does not violate the project's source-level safe-Rust requirement.

---

119. Rust API Stability

The executable lexer should expose a stable conceptual API:

Lexer
Token
TokenKind / TokenType
LexerError

The exact names may remain compatible with the existing repository.

The implementation should avoid requiring downstream consumers to understand lexical internals.

---

120. Existing "TokenType" Migration

The current "src/lexer.rs" contains a broad "TokenType" enumeration including:

Identifier
String
Integer
Float
Char
Boolean
QuantumLiteral
NanoAnnotation
MTSLiteral

and many operators and keywords.

The production migration should be:

Identifier → Identifier
String → StringLiteral
Integer → IntegerLiteral
Float → FloatLiteral
Char → CharLiteral
Boolean → BooleanLiteral

with compatibility aliases where necessary.

The special-purpose tokens:

NanoAnnotation
MTSLiteral

should be evaluated against the compositional architecture described above rather than retained automatically.

---

121. Keyword Migration

The current Rust lexer contains many specialized keywords, including:

omniversal
asi
aesi
asesi
admin
payment
gateway
graphics
video
adjust
versioning
copyright
notice
legal
action
tailor
business

These should not automatically remain permanent core reserved words.

Each must be classified through:

stable core keyword
contextual keyword
dialect keyword
compatibility keyword
ordinary identifier
deprecated keyword

The classification must be documented in "grammar/lexer/keywords.md".

This prevents lexical bloat.

---

122. "Zamani.g4" Integration

The current root grammar intentionally uses literal spellings directly, for example:

'fn'
'module'
'import'
'export'
'@'
'->'
'='

and has a broad language surface.

That is acceptable at the parser grammar level provided the lexical contract remains consistent.

The grammar must not introduce a parser spelling that has no corresponding lexical contract.

---

123. Grammar/Parser Consistency Rule

For every literal spelling in "Zamani.g4":

parser literal

must correspond to exactly one of:

canonical punctuation
canonical operator
canonical keyword
identifier/contextual construct

Any mismatch is a conformance error.

---

124. Token-to-Grammar Traceability

Each token must be traceable to:

tokens.md
    ↓
keywords.md/operators.md/literals.md/etc.
    ↓
Zamani.g4
    ↓
src/parser.rs
    ↓
AST

A token with no consumer must be marked:

reserved
future
compatibility
deprecated

rather than silently remaining unexplained.

---

125. Grammar-to-Token Traceability

Conversely, every lexical spelling consumed by:

Zamani.g4

must be traceable back to this token contract.

This prevents grammar-only tokens from silently becoming part of the language.

---

126. AST Contract

Tokenization must remain below the AST abstraction.

For example:

H

is an:

Identifier

not:

QuantumGateH

The parser/semantic layer may interpret:

H

as a quantum operation in the appropriate context.

This is critical for extensibility.

---

127. Canonical Quantum Pipeline

The lexical boundary is:

quantum source spelling
        ↓
canonical tokens
        ↓
domain-neutral AST
        ↓
semantic quantum operation
        ↓
quantum::ir

Not:

lexer
 ↓
QuantumGate enum
 ↓
second quantum IR

The repository's canonical architecture explicitly keeps "quantum::ir" as the quantum semantic boundary.

---

128. Hardware/Resource Pipeline

The lexical boundary must likewise remain:

resource vocabulary
        ↓
Identifier / keyword / literal
        ↓
AST
        ↓
resource/capability semantics
        ↓
compiler/resource manager
        ↓
target realization

The lexer does not resolve resources.

---

129. POCO-REAF Guarantee

The token layer contributes to POCO-REAF by guaranteeing:

same source
    +
same language version
    +
same lexical configuration
    ↓
same token stream

regardless of the eventual target:

tiny CPU
large CPU
GPU
FPGA
QPU
cluster
cloud
edge
embedded
future machine

The lexer is therefore target-independent.

---

130. "Infinity" Interpretation

"Scale to infinity" means:

«No artificial language-level upper bound is imposed by the grammar or token model.»

It does not mean that a physical machine has infinite memory or execution resources.

The implementation must therefore distinguish:

unbounded language model

from:

finite implementation resources

This distinction is mandatory throughout Zamani.

---

131. Production Completion Criteria

"grammar/lexer/tokens.md" is complete only when all of the following are true:

- [ ] canonical token categories defined;
- [ ] every canonical spelling defined;
- [ ] duplicate token concepts resolved;
- [ ] keyword boundary defined;
- [ ] contextual keyword policy defined;
- [ ] identifier integration defined;
- [ ] literal integration defined;
- [ ] operator integration defined;
- [ ] quantum token integration defined;
- [ ] comment integration defined;
- [ ] source-span integration defined;
- [ ] error token defined;
- [ ] recovery policy defined;
- [ ] deterministic behavior defined;
- [ ] Unicode policy referenced;
- [ ] versioning defined;
- [ ] compatibility defined;
- [ ] serialization policy defined;
- [ ] ANTLR integration defined;
- [ ] Rust lexer integration defined;
- [ ] parser integration defined;
- [ ] AST integration defined;
- [ ] semantic boundary defined;
- [ ] IR boundary defined;
- [ ] quantum::ir boundary preserved;
- [ ] resource/hardware boundary preserved;
- [ ] no hardware limits encoded;
- [ ] no artificial token limits encoded;
- [ ] safe-Rust requirement defined;
- [ ] positive tests defined;
- [ ] negative tests defined;
- [ ] boundary tests defined;
- [ ] scalability tests defined;
- [ ] determinism tests defined;
- [ ] compatibility tests defined;
- [ ] hard-coding audit defined.

---

132. Definition of Done for "src/lexer.rs"

After this document is accepted, "src/lexer.rs" is complete only when:

TokenType
    ↓
matches canonical token taxonomy

Lexer
    ↓
implements canonical longest-match behavior

keywords
    ↓
come from canonical keyword policy

literals
    ↓
preserve source representation

spans
    ↓
are exact and UTF-8-safe

errors
    ↓
are deterministic and structured

special domains
    ↓
remain extensible and target-independent

resource budgets
    ↓
are implementation policy, not language limits

Rust
    ↓
1.97 / 1.97.1
safe only

No later file should require redesigning the token model.

---

133. Definition of Done for "src/parser.rs"

The parser is complete against this token contract when:

- every canonical token consumed by syntax has a defined production;
- duplicate token aliases have been removed or isolated behind compatibility adapters;
- parser precedence agrees with "grammar/expressions/precedence.md";
- token spans are propagated into AST nodes;
- contextual keywords are handled contextually;
- identifiers remain extensible;
- domain operations are not hard-coded into lexical token categories;
- quantum operations lower toward the existing semantic architecture;
- no target-specific token is required.

The current parser uses a Pratt-style precedence model and already consumes many of the overlapping token variants; this is precisely where canonicalization must eventually be applied.

---

134. Definition of Done for "grammar/Zamani.g4"

The root grammar is complete against this token contract when:

every parser spelling
        ↕
canonical lexical contract

and:

no grammar rule

requires an undocumented token.

"Zamani.g4" remains the canonical ANTLR composition root.

It must not become another token authority.

---

135. Definition of Done for Tooling

The token registry is complete when the following can derive their lexical classifications from one source of truth:

compiler lexer
ANTLR lexer
parser
formatter
syntax highlighter
LSP
documentation tooling
conformance tests
compatibility tooling

No independent keyword/operator lists should drift from the canonical token contract.

---

136. Final Canonical Token Inventory

The production conceptual inventory is:

End of input

EOF

Identifiers

Identifier

Literals

BooleanLiteral
NullLiteral
IntegerLiteral
FloatLiteral
CharLiteral
StringLiteral
RawStringLiteral
QuantumLiteral

Punctuation

LParen
RParen
LBracket
RBracket
LBrace
RBrace
Comma
Dot
Semicolon
Colon
At
Hash

Operators

Plus
Minus
Star
Slash
Modulo

Assign
PlusAssign
MinusAssign
StarAssign
SlashAssign

Equals
NotEquals

LessThan
LessThanEqual
GreaterThan
GreaterThanEqual

LogicalAnd
LogicalOr

BitAnd
BitOr
Caret

LeftShift
RightShift

Not
Tilde

Question

Arrow
FatArrow
DoubleColon

DotDot
DotDotEq

Domain-independent special lexical forms

QuantumLiteral

Additional domain constructs should normally be represented compositionally using existing token kinds.

Keywords

Defined by the canonical registry in:

grammar/lexer/keywords.md

rather than by an unbounded enumeration in this document.

Error

Illegal

or its final canonical equivalent.

---

137. Explicit Non-Goals

This file must never become the authority for:

maximum qubits
maximum CPUs
maximum GPUs
maximum FPGAs
maximum nodes
maximum threads
maximum memory
maximum tensor dimensions
maximum register width
maximum accelerator count
maximum timeline count
maximum source size
maximum mathematical value
maximum quantum state size
physical topology
device IDs
physical qubit IDs
gate decomposition
routing
scheduling
QEC
ZQN
HAL
calibration
backend selection
runtime scheduling

Those belong to other layers.

---

138. Architectural Invariant

The most important invariant is:

LEXICAL TOKEN
    ≠
SEMANTIC OPERATION
    ≠
RESOURCE
    ≠
HARDWARE
    ≠
IR
    ≠
RUNTIME IMPLEMENTATION

For example:

CNOT

may be an identifier.

It is not inherently:

physical two-qubit gate

The semantic layer decides what the identifier means in context.

Likewise:

GPU

may be an identifier or capability name.

It does not force the compiler to use a particular GPU.

---

139. Final Integration Contract

The complete production relationship is:

grammar/spec/lexical.md
        │
        ▼
grammar/lexer/tokens.md
        │
        ├── grammar/lexer/keywords.md
        ├── grammar/lexer/operators.md
        ├── grammar/lexer/identifiers.md
        ├── grammar/lexer/literals.md
        ├── grammar/lexer/comments.md
        ├── grammar/lexer/unicode.md
        └── grammar/lexer/quantum-literals.md
        │
        ▼
canonical lexical behavior
        │
        ├───────────────┐
        ▼               ▼
   src/lexer.rs    ANTLR lexer
        │               │
        └───────┬───────┘
                ▼
        canonical token stream
                │
                ▼
        grammar/Zamani.g4
                │
                ▼
          src/parser.rs
                │
                ▼
        domain-neutral AST
                │
                ▼
        structural validation
                │
                ▼
        semantic analysis
                │
                ▼
       canonical semantic model
                │
        ┌───────┼────────┐
        ▼       ▼        ▼
   Classical quantum::ir HDL
       IR          IR
        │       │        │
        └───────┼────────┘
                ▼
          optimization
                ▼
      routing / scheduling
                ▼
       resilience / QEC / ZQN
                ▼
               HAL
                ▼
       target realization

No stage is permitted to move hardware-specific semantics backward into the lexical layer.

---

140. Final Production Principle

The Zamani token system must be:

small
stable
composable
Unicode-aware
deterministic
versioned
source-preserving
extensible
domain-neutral
hardware-independent
resource-independent
safe-Rust compatible
ANTLR-compatible
parser-compatible
AST-compatible
IR-compatible
tooling-compatible

The lexer should recognize the language, not the current inventory of machines, algorithms, quantum gates, AI frameworks, FPGA primitives, networking protocols, or vendor products.

That is the lexical foundation required for:

Program Once
Compile Once
Run Everywhere
Anywhere
Forever

while preserving the architecture:

source
 → lexer
 → parser
 → AST
 → semantic model
 → canonical IR
 → optimization
 → routing/scheduling/resilience
 → ZQN
 → HAL
 → target

and preserving "quantum::ir" as the canonical quantum semantic boundary.