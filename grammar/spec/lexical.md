Zamani Lexical Specification

Path: "grammar/spec/lexical.md"
Language: Zamani
Specification role: Normative lexical contract
Status: Production architecture
Specification version: 1.0
Implementation baseline: Rust 1.97 / Rust 1.97.1
Compiler safety: Safe Rust only; Rust "unsafe" is prohibited
Portability principle: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)
Scalability principle: No artificial language-level resource or hardware limits
Canonical semantic quantum boundary: "quantum::ir"

---

0. Purpose

This document defines the normative lexical contract of the Zamani programming language.

Lexical analysis converts source text into a deterministic sequence of lexical tokens and structured lexical diagnostics.

The lexical layer establishes:

- source encoding;
- Unicode handling;
- identifiers;
- keywords;
- contextual keywords;
- literals;
- operators;
- punctuation;
- comments;
- whitespace;
- source positions;
- source spans;
- lexical diagnostics;
- token precedence;
- longest-match behavior;
- lexical configuration;
- language-version behavior;
- token preservation;
- deterministic recovery;
- implementation conformance.

The lexical layer does not establish:

- type semantics;
- name resolution;
- ownership;
- borrowing;
- effects;
- quantum semantics;
- QEC;
- ZQN;
- routing;
- scheduling;
- calibration;
- hardware topology;
- resource allocation;
- target selection;
- backend selection;
- optimization strategy;
- runtime behavior;
- deployment;
- physical device identity.

The fundamental boundary is:

source bytes
    ↓
source decoding
    ↓
Unicode scalar values
    ↓
lexical scanning
    ↓
tokens + lexical diagnostics
    ↓
parser
    ↓
frontend AST
    ↓
structural validation
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

---

1. Architectural Authority

Zamani must have one normative lexical contract.

The authority relationship is:

grammar/spec/lexical.md
        │
        ├───────────────┐
        │               │
        ▼               ▼
src/lexer.rs       grammar/antlr/ZamaniLexer.g4
        │               │
        └───────┬───────┘
                ▼
       canonical token model
                │
                ▼
          src/parser.rs
                │
                ▼
        frontend AST

The authority order for lexical behavior is:

1. "grammar/spec/lexical.md"
2. the canonical language/version policy
3. canonical token definitions
4. "src/lexer.rs"
5. "grammar/antlr/ZamaniLexer.g4"
6. parser grammar
7. frontend AST
8. semantic implementation
9. "grammar/grammar.md"
10. "grammar/Zamani-Grammar.md"

"grammar/grammar.md" describes current implementation conformance.

"grammar/Zamani-Grammar.md" describes broader language design, historical material, proposals, and future features.

Neither may silently override this document.

---

2. File Ownership

2.1 This file owns

This file owns:

- lexical categories;
- token identity;
- token spelling;
- token precedence;
- lexical ambiguity resolution;
- identifier syntax;
- keyword classification;
- literal lexical forms;
- comments;
- whitespace;
- source encoding;
- source spans;
- lexical errors;
- lexical recovery;
- lexical versioning;
- lexical configuration;
- lexical scalability requirements.

2.2 This file does not own

This file does not own:

- AST node definitions;
- type checking;
- semantic interpretation;
- resource allocation;
- target-specific implementation;
- quantum operation semantics;
- QEC;
- ZQN;
- scheduling;
- routing;
- HAL behavior;
- compiler optimization;
- runtime behavior.

---

3. Integration Contract

The lexical contract integrates with the repository as follows.

Component| Responsibility
"grammar/spec/lexical.md"| normative lexical behavior
"grammar/spec/syntax.md"| parser-visible syntax
"grammar/spec/type-system.md"| type interpretation
"grammar/antlr/ZamaniLexer.g4"| ANTLR lexical implementation
"grammar/Zamani.g4"| root grammar composition
"src/lexer.rs"| executable Rust lexer
"src/parser.rs"| parser and syntax interpretation
"src/ast/"| target-independent source AST
"grammar/grammar.md"| implementation-conformance reference
"grammar/Zamani-Grammar.md"| broader design/history/proposals
"grammar/tests/"| lexical conformance
semantic layer| semantic meaning
canonical IR| language-independent computational representation
"quantum::ir"| canonical quantum semantic boundary

No downstream component may require the lexer to know machine topology, physical resources, or backend details.

---

4. Production Invariants

Every conforming Zamani lexer MUST satisfy all of the following.

4.1 Determinism

For identical:

- source bytes;
- language version;
- lexical configuration;
- compatibility mode;

the lexer MUST produce identical:

- token kinds;
- token order;
- token source spans;
- literal source representations;
- diagnostics;
- diagnostic ordering.

Lexing MUST NOT depend on:

- CPU architecture;
- CPU count;
- GPU;
- QPU;
- FPGA;
- operating system locale;
- wall-clock time;
- random state;
- hash iteration order;
- network state;
- hardware topology;
- available device count.

---

5. Safety

The reference compiler implementation MUST use safe Rust.

The lexical implementation MUST NOT use:

unsafe

or:

unsafe { ... }

or unsafe traits, unsafe functions, unsafe blocks, or unsafe mutable aliasing.

The lexical architecture MUST NOT require unsafe Rust.

This applies to:

- UTF-8 processing;
- source slicing;
- token construction;
- diagnostics;
- buffering;
- incremental lexing;
- streaming lexing;
- parallel lexing;
- Unicode processing.

Safe Rust abstractions must be used instead.

---

6. Scalability and POCO-REAF

Lexical syntax MUST NOT encode hardware limits.

The lexer MUST NOT contain language-level limits such as:

MAX_TOKENS
MAX_SOURCE_SIZE
MAX_IDENTIFIER_LENGTH
MAX_INTEGER_WIDTH
MAX_STRING_LENGTH
MAX_QUANTUM_STATE_SIZE
MAX_QUBITS
MAX_CPUS
MAX_GPUS
MAX_FPGAS
MAX_NODES
MAX_THREADS
MAX_MEMORY

A compiler may have explicit resource policies.

Such policies are not lexical-language limits.

For example:

compiler resource budget:
    maximum source bytes
    maximum diagnostics
    maximum memory
    maximum compilation time

is an implementation/resource policy.

It is not:

Zamani identifiers may contain at most N characters

as a language rule unless that restriction is genuinely required for language semantics.

---

7. Resource-Bounded Lexing

Production implementations may operate under resource budgets.

A resource exhaustion condition MUST be distinguishable from invalid source.

Examples:

source-too-large
lexical-memory-budget-exceeded
diagnostic-budget-exceeded
compilation-time-budget-exceeded

must not be reported as:

invalid-token
invalid-identifier
invalid-number

Resource limits MUST be:

- explicit;
- observable;
- configurable where appropriate;
- deterministic;
- documented;
- separate from language validity.

A larger machine may process a larger source program without changing the language semantics.

---

8. Source Encoding

The canonical source encoding is UTF-8.

The pipeline is:

source bytes
    ↓
UTF-8 validation
    ↓
Unicode scalar values
    ↓
lexical scanning

Invalid UTF-8 MUST produce a source-encoding diagnostic.

The compiler MUST NOT reinterpret invalid bytes using:

- operating-system locale;
- host code page;
- platform-specific encoding;
- implicit replacement characters.

Invalid UTF-8 MUST NOT silently become U+FFFD.

---

9. Unicode Scalar Model

The lexical layer operates on Unicode scalar values.

A Unicode scalar value is a Unicode code point excluding surrogate code points.

The lexer MUST distinguish:

raw bytes
Unicode scalar values
source characters
tokens

The lexer MUST NOT treat UTF-16 surrogate halves as valid standalone source characters.

---

10. Unicode Normalization

The lexer MUST NOT silently normalize source identifiers.

In particular, it MUST NOT silently apply:

- NFC;
- NFD;
- NFKC;
- NFKD;

to identifiers unless a future version explicitly adopts such behavior.

Normalization policy belongs to the language name-resolution specification.

If Zamani later adopts normalized identifiers, the language version MUST explicitly define:

- normalization form;
- normalization timing;
- keyword comparison;
- identifier comparison;
- combining-character rules;
- confusable policy;
- source preservation;
- diagnostic behavior.

The lexer must preserve the original source spelling.

---

11. Identifier Policy

Zamani supports Unicode-aware identifiers.

The canonical identifier policy is:

IDENTIFIER
    = IDENTIFIER_START IDENTIFIER_CONTINUE*

"IDENTIFIER_START" may contain:

- ASCII letters;
- underscore;
- Unicode identifier-start characters defined by the canonical Unicode identifier profile.

"IDENTIFIER_CONTINUE" may additionally contain:

- decimal digits;
- Unicode identifier-continue characters;
- explicitly permitted combining marks.

Digits MUST NOT begin an identifier.

Examples:

x
value
_result
qubit_count
QuantumState
π
résultat
状態
данные

Invalid:

1value
42abc

Unicode support MUST NOT introduce a machine-size limitation.

---

12. Unicode Identifier Security

Unicode identifiers MUST preserve source identity.

The compiler SHOULD provide diagnostics or tooling for:

- mixed scripts;
- confusable identifiers;
- invisible characters;
- bidi-control characters;
- suspicious identifier sequences.

Such tooling MUST NOT silently rename source identifiers.

Security diagnostics belong to validation/tooling, not lexical reinterpretation.

---

13. Keyword Model

Zamani has two keyword categories.

13.1 Reserved keywords

A reserved keyword is never available as an ordinary identifier in the corresponding language version.

13.2 Contextual keywords

A contextual keyword is interpreted specially only in a grammar context that requires it.

The preferred strategy is:

identifier
    ↓
parser context
    ↓
contextual meaning

rather than continuously expanding the reserved keyword set.

This preserves source compatibility and allows Zamani to scale across future computing domains.

---

14. Keyword Registry

There MUST be one canonical keyword registry.

The registry must define:

lexeme
token identity
keyword class
language version
feature status
context
compatibility status

The registry is shared conceptually by:

- Rust lexer;
- ANTLR lexer;
- parser;
- formatter;
- syntax highlighter;
- LSP;
- documentation generator;
- compatibility tooling;
- conformance tests.

No subsystem may invent an independent keyword list.

---

15. Keyword Explosion Prevention

A domain concept MUST NOT become a keyword merely because the concept exists.

The preferred form is:

identifier
+
compositional syntax
+
semantic resolution

rather than:

one keyword per operation

This is especially important for:

- quantum gates;
- mathematical functions;
- AI operators;
- accelerator operations;
- hardware devices;
- vendor APIs;
- optimization algorithms;
- cryptographic algorithms;
- networking protocols.

For example, these SHOULD normally remain identifiers:

H
X
CNOT
custom_gate
vendor_operation
fft
svd
adam
cuda_kernel
tensor_contract

unless a specific spelling has genuine language-level lexical significance.

---

16. Core Reserved Vocabulary

The exact stable keyword inventory is maintained by the canonical keyword registry.

The following categories are language-level concepts and may contain reserved or contextual keywords:

fn
let
var
mut
const

return
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
yield

module
import
export
use
from
as
package

type
struct
enum
trait
impl
class
interface
record

public
pub
private
protected
internal

static
override
virtual
abstract
final
extends
implements

this
self
super
new

where

async
await
spawn
parallel

try
catch
finally
throw
handle

effect
effects
with

requires
ensures
invariant

quantum
circuit
Qubit

apply
measure
reset
barrier
control
adjoint
inverse
observe

This list is a compatibility baseline, not a license to make every domain vocabulary word reserved.

---

17. "unsafe"

The Zamani language is designed around safe computation.

The production language MUST NOT provide an "unsafe" escape hatch that permits the source program to bypass the language safety model.

Therefore:

unsafe

is reserved in the lexical compatibility layer but MUST NOT introduce a valid executable language construct.

This allows the compiler to produce a deterministic, explicit diagnostic rather than treating "unsafe" as an ordinary identifier.

A stable parser MUST reject an "unsafe" construct with a dedicated diagnostic such as:

ZMN-SAFETY-UNSAFE-DISALLOWED

The lexical layer itself does not enforce semantic safety; it only guarantees that the spelling is recognized consistently.

No Rust "unsafe" implementation is permitted.

---

18. Boolean Literals

The canonical boolean literals are:

true
false

They are literal tokens.

They MUST NOT also be emitted as ordinary identifiers.

Their semantic type is established by the type system.

---

19. Null-Like Literals

The canonical stable null literal is:

nil

"null" may remain reserved for compatibility if already present in the implementation.

If both spellings are retained, their semantic relationship MUST be explicitly defined.

The lexer MUST NOT give them accidentally different meanings.

If "null" is deprecated, the lexer should continue recognizing it according to the active compatibility version and the parser/diagnostic layer should issue the appropriate deprecation diagnostic.

---

20. Integer Literals

Zamani supports arbitrary lexical integer magnitude.

Canonical forms include:

0
42
1_000_000
0xFF
0b1010
0o755

The lexical representation is:

INTEGER
    = DECIMAL_INTEGER
    | HEX_INTEGER
    | BINARY_INTEGER
    | OCTAL_INTEGER

Digit separators may be permitted between digits.

Leading/trailing separators MUST be rejected.

Examples:

1_000       valid
0xFF_A0     valid
0b1010_0101 valid

_100        invalid
100_        invalid
0x_FF       invalid

The lexer MUST preserve the literal spelling.

---

21. Integer Scalability

The lexer MUST NOT parse an integer directly into:

u32
u64
usize
i32
i64

merely because those are convenient host representations.

For example:

999999999999999999999999999999999999999999999999999999

is lexically valid.

The lexical representation must remain available to later literal semantics.

Semantic analysis determines:

- signedness;
- width;
- arbitrary precision;
- range;
- target representation;
- constant-evaluation rules.

No host integer overflow may corrupt the source literal.

---

22. Floating-Point Literals

Zamani supports decimal floating-point syntax.

Examples:

1.0
0.5
3.14159
1e9
1.5e-9
1_000.25
.5

The canonical forms are:

DIGIT_SEQUENCE "." DIGIT_SEQUENCE EXPONENT?
"." DIGIT_SEQUENCE EXPONENT?
DIGIT_SEQUENCE EXPONENT

where:

EXPONENT = ("e" | "E") ("+" | "-")? DIGIT_SEQUENCE

The lexer identifies syntax only.

It MUST NOT silently convert the value to:

f32
f64

during lexical scanning.

Precision and representation belong to the type and semantic layers.

---

23. Numeric Literal Preservation

For every numeric literal the implementation must preserve enough source information to determine:

- original base;
- digit sequence;
- separator positions if required for diagnostics;
- sign where syntactically represented;
- exponent;
- suffix if a future version introduces suffixes;
- source span.

Numeric parsing MUST NOT depend on target hardware width.

---

24. Future Numeric Forms

Zamani may support:

- hexadecimal floating point;
- decimal floating point;
- rational literals;
- complex literals;
- fixed-point literals;
- arbitrary-precision literals;
- symbolic numeric literals;
- physical-unit literals;
- probability literals;
- quantum-specific numeric notation.

Each new form requires:

1. lexical specification;
2. ambiguity analysis;
3. token identity;
4. parser integration;
5. AST contract;
6. semantic contract;
7. tests;
8. compatibility classification.

No extension may silently change an existing literal's meaning.

---

25. String Literals

A normal string literal begins and ends with:

"

Example:

"Hello, Zamani"

A string may contain Unicode scalar values and supported escape sequences.

An unterminated string is a lexical error.

A raw string facility may be introduced as a separate lexical form rather than overloading ordinary strings.

---

26. Escape Sequences

The canonical escape table includes:

\"    quotation mark
\\    backslash
\b    backspace
\f    form feed
\n    line feed
\r    carriage return
\t    horizontal tab

Unicode escapes must use a separately specified canonical syntax.

At minimum, if "\u" is supported, the exact grammar must define:

- number of hexadecimal digits;
- scalar-value validation;
- surrogate rejection;
- escape normalization;
- diagnostic behavior.

Unknown escapes MUST be rejected.

For example:

"\q"

must not silently become:

q

---

27. Character Literals

Character literals use:

'

Examples:

'a'
'\n'
'λ'

A character literal represents exactly one Unicode scalar value after escape interpretation.

Invalid:

''
'ab'
'abc'

Surrogate values are invalid.

Unterminated character literals are lexical errors.

---

28. Raw Strings

If raw strings are enabled, they must have a delimiter system that allows embedded quotation marks without ambiguous termination.

The raw-string grammar must be independently specified.

Raw strings MUST preserve their payload without interpreting ordinary escape sequences.

The source representation must remain available for diagnostics and tooling.

---

29. Byte and Binary Literals

Byte-oriented literals must remain distinct from Unicode character literals.

If introduced, the language must distinguish:

character
string
byte
byte-string
binary data

without relying on host encoding.

Binary data literals may represent arbitrary octets and therefore MUST NOT be interpreted as Unicode text.

---

30. Punctuation

Canonical punctuation includes:

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
@
#

and lexical operator symbols.

Each spelling has exactly one canonical lexical identity.

Parser context determines meaning.

---

31. Operators

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

and:

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

Future operators must be introduced through the operator registry and precedence specification.

---

32. Longest-Match Rule

The lexer MUST use longest-valid-token matching.

Examples:

>=  → GE
>   → GT

>>  → SHIFT_RIGHT
>   → GT

::  → DOUBLE_COLON
:   → COLON

..= → RANGE_INCLUSIVE
..  → RANGE

->  → ARROW
-   → MINUS

=>  → FAT_ARROW
=   → ASSIGN

A valid longer token MUST always win over its valid shorter prefix.

---

33. Canonical Operator Identity

A source spelling MUST have one canonical token identity.

For example:

&

must not simultaneously be:

BitAnd
Ampersand
ReferenceOperator

at the lexical layer.

Instead:

&
    ↓
AMPERSAND
    ↓
parser context
    ↓
AST meaning
    ↓
semantic meaning

The same principle applies to:

|
?
*
-
+

and other overloaded operators.

---

34. Arrow Operators

The following are distinct:

->
=>

"->" may be used for type/function relationships.

"=>" may be used for match arms or other grammar-defined constructs.

Their semantic meaning belongs to parser/AST layers.

---

35. Range Operators

The canonical range operators are:

..
..=

They MUST be lexed as complete tokens.

They MUST NOT become:

.
.

when the two-dot sequence forms a valid range token.

---

36. Question Mark

The canonical token for:

?

is "QUESTION".

The lexer does not decide whether it means:

- propagation;
- optionality;
- a future type construct;
- another grammar-defined feature.

That decision belongs to parser and semantic layers.

---

37. Attributes

The hash symbol:

#

is punctuation.

An attribute such as:

#[inline]

is parsed by the attribute grammar.

The lexer MUST NOT resolve:

inline

to its semantic meaning.

Attribute validation belongs downstream.

---

38. Annotations

The at sign:

@

is punctuation unless the canonical grammar explicitly defines a complete lexical token for an annotation.

The preferred architecture is:

@
+
identifier
+
optional arguments

rather than a separate lexical token for every annotation.

For example:

@atom
@molecule(...)
@hardware(...)
@capability(...)

remain compositional syntax.

The lexer MUST NOT execute or interpret annotations.

---

39. Quantum Lexical Boundary

Quantum source is ordinary Zamani source.

The lexical layer MUST NOT contain a fixed hardware gate vocabulary.

These are normally identifiers:

H
X
Y
Z
CNOT
CZ
SWAP
U
custom_gate
vendor_operation
logical_operation

This allows:

apply H to q;
apply CNOT to q0, q1;
apply custom_gate to q;
apply vendor.operation to q;

without requiring the lexer to know the available target hardware.

---

40. Quantum Literals

The baseline quantum state literal forms are:

|0⟩
|1⟩
|+⟩
|-⟩

These are lexical source values.

They do not imply:

- physical qubit allocation;
- a particular QPU;
- a simulator;
- a physical qubit index;
- a hardware topology;
- fixed numerical precision.

The lexer MUST NOT allocate quantum resources.

---

41. Quantum Literal Extension

Future versions may permit richer quantum state notation.

Examples could include:

|ψ⟩
|φ⟩
|n⟩
|α⟩

or structured state expressions.

Such forms must be introduced through a formal grammar extension.

The lexer must not guess whether an arbitrary sequence between "|" and "⟩" is a quantum state.

Ambiguous notation must be handled by the canonical syntax specification.

---

42. Quantum Pipeline

Quantum lexical constructs flow through:

source
  ↓
lexer
  ↓
parser
  ↓
generic frontend AST
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
target

The lexical layer must never create a competing quantum IR.

---

43. Domain Operations

The same principle applies outside quantum computing.

Operations belonging to:

- classical mathematics;
- AI;
- machine learning;
- networking;
- cryptography;
- HDL;
- accelerator programming;
- distributed computing;
- scientific computing;

should normally remain identifiers.

For example:

fft
matmul
attention
encrypt
hash
reduce
broadcast
synthesize
kernel
pipeline

must not automatically become keywords.

A word becomes a keyword only when the language itself needs lexical distinction.

---

44. Hardware Vocabulary

Hardware-specific names MUST NOT become universal keywords.

The lexer must not require permanent lexical identities for:

gpu0
qpu0
fpga0
cpu0
device17
core7
physical_qubit42
memory_bank3
node8

These are target/resource information.

They belong to:

resources/
hardware/
compile/
execution/
semantic analysis
target lowering

as appropriate.

---

45. POCO-REAF Lexical Requirement

A valid Zamani source program MUST NOT need to change merely because the target changes from:

small machine
→ workstation
→ server
→ cluster
→ supercomputer
→ GPU system
→ FPGA system
→ QPU
→ heterogeneous system
→ future computational substrate

Lexical behavior remains unchanged.

Target realization happens after semantic analysis and canonical IR formation.

---

46. HDL Lexical Boundary

HDL constructs must describe hardware intent.

The lexer must not hard-code:

- bus widths;
- register counts;
- device counts;
- FPGA families;
- ASIC families;
- clock frequencies;
- memory capacities.

For example:

signal data: Bits<N>;

contains a program parameter.

It does not establish a universal hardware maximum.

---

47. AI Lexical Boundary

AI-related concepts such as:

model
dataset
train
infer
agent
tensor
gradient
parameter

may be language keywords only where language-level syntax requires them.

Framework-specific identifiers remain identifiers.

The lexer must not contain permanent vocabularies for:

PyTorch
TensorFlow
JAX
CUDA
ROCm
vendor-specific accelerator APIs

unless a future compatibility dialect explicitly requires lexical treatment.

---

48. Networking Lexical Boundary

Networking syntax must remain independent of:

- fixed node counts;
- fixed interface counts;
- fixed packet sizes;
- fixed topology;
- fixed IP formats where abstract addressing is intended.

Network protocol names should normally be identifiers or semantic capability names.

---

49. Security Lexical Boundary

Cryptographic algorithms should normally remain semantic identifiers.

The lexer must not become a cryptographic algorithm registry.

For example:

hash
sign
verify
encrypt
decrypt
prove
verify_proof

may be semantic operations.

Specific algorithms may be library/domain identifiers.

Secret material must never be interpreted by the lexer as executable behavior.

---

50. Comments

Zamani supports:

// line comments

and:

/*
   block comments
*/

A line comment terminates at a logical line boundary.

A block comment terminates at:

*/

An unterminated block comment is a lexical error.

---

51. Nested Block Comments

The stable lexical language does not require nested block comments.

Therefore:

/*
    /*
        nested
    */
*/

must not be silently treated as a valid nested-comment construct.

If nested comments are introduced later, they require a language-versioned lexical rule and conformance tests.

---

52. Documentation Comments

Documentation comments are lexically distinguishable from ordinary comments:

///
/// /** ... */

They MUST NOT become ordinary semantic parser tokens.

They may be retained on a hidden/documentation channel for:

- documentation generation;
- IDEs;
- LSP;
- symbol indexing;
- formatting;
- source transformation.

Their existence must not alter program semantics.

---

53. Comment Preservation

The lexer implementation may discard ordinary comments from the parser-facing token stream.

However, source spans must allow tooling to recover comments where required.

The implementation should therefore conceptually maintain:

semantic token stream
+
trivia/source metadata

rather than forcing comments into the AST.

---

54. Whitespace

Whitespace is normally insignificant.

The baseline whitespace includes:

space
tab
carriage return
line feed
vertical tab
form feed

Unicode whitespace is not automatically syntax-significant.

If additional Unicode whitespace is accepted, the rule must be explicitly specified.

---

55. Newlines

Newlines are not semantic in the baseline language.

Therefore:

let x = 1;
let y = 2;

and equivalent whitespace arrangements must tokenize consistently unless the parser grammar explicitly requires a delimiter.

Newlines must remain available through source spans for:

- diagnostics;
- formatting;
- IDE tooling;
- documentation;
- debugging;
- source mapping.

---

56. Significant Whitespace

If a future dialect requires significant indentation or whitespace, it must be an explicit dialect/version feature.

The base language lexer must not accidentally become indentation-sensitive.

---

57. EOF

The lexer MUST emit a deterministic EOF marker.

EOF has a valid source position.

EOF must not consume additional source.

Repeated requests after EOF must have deterministic behavior.

---

58. Invalid Characters

An unrecognized character MUST produce a structured lexical diagnostic.

The lexer MUST NOT:

- silently discard it;
- replace it;
- skip arbitrarily far;
- reinterpret it as another token.

Recovery may consume the offending scalar value and continue.

---

59. Lexical Recovery

Recovery must guarantee progress.

For every invalid input position:

detect error
    ↓
emit diagnostic
    ↓
consume a well-defined unit
    ↓
continue or terminate according to policy

The lexer must never repeatedly inspect the same invalid position indefinitely.

---

60. Diagnostic Requirements

A lexical diagnostic should contain:

diagnostic code
severity
source span
message
optional explanation
optional suggested correction
language version

Diagnostics must be deterministic.

Example categories:

invalid-character
invalid-utf8
unterminated-string
unterminated-character
unterminated-comment
invalid-escape
invalid-number
invalid-identifier
invalid-unicode-scalar
reserved-keyword
lexical-resource-exhausted

---

61. Diagnostic Stability

Diagnostic identifiers should remain stable across compatible releases.

Human-readable wording may improve without changing the diagnostic identity.

Tools must use diagnostic codes rather than matching complete diagnostic strings.

---

62. Source Spans

Every source-derived token MUST have a source span.

A span must be capable of representing:

source identity
start position
end position

The implementation may additionally expose:

line
column
byte offset
Unicode scalar offset
UTF-16 position

for tooling.

The canonical internal representation must avoid ambiguity between byte offsets and character positions.

---

63. Byte and Character Positions

Because source is UTF-8, byte offsets and Unicode scalar offsets are different concepts.

The lexer MUST define which coordinate system is canonical internally.

Tooling adapters may provide:

- UTF-8 byte offsets;
- Unicode scalar offsets;
- UTF-16 offsets;
- line/column.

Conversion must be deterministic.

---

64. Token Value Preservation

A token may carry:

token kind
source span
raw source representation
decoded value

The implementation must not require semantic interpretation during lexing.

For literals, preserving raw spelling is particularly important.

Example:

1_000

must remain distinguishable from:

1000

at least to tooling/diagnostic layers if source preservation requires it.

---

65. Token Precedence

The conceptual lexical precedence is:

1. source decoding;
2. whitespace/comments;
3. documentation comments;
4. longest multi-character operators;
5. complete domain literals;
6. ordinary literals;
7. identifiers and keyword classification;
8. single-character operators;
9. punctuation;
10. invalid characters;
11. EOF.

The concrete scanner implementation may use a different internal algorithm provided the observable behavior is identical.

---

66. Domain Literal Isolation

Domain-specific literals must never consume content that belongs to ordinary strings.

For example:

"mts[...]"

is always a string.

Likewise:

"|0⟩"

is a string.

The lexer must identify the opening quotation mark before considering domain literal syntax inside the string.

---

67. MTS Lexical Contract

The existing repository contains MTS terminology and an "MTSLiteral" concept.

The production lexical rule is:

mts[ ... ]

is a complete MTS lexical literal only if the active language version enables MTS literal syntax.

The payload remains source text at the lexical layer.

The lexer MUST NOT:

- parse timestamps;
- create timelines;
- allocate timelines;
- evaluate temporal semantics;
- create branches;
- manipulate history.

Those belong downstream.

---

68. MTS Ambiguity

The word:

mts

must have exactly one status for each language version:

1. reserved keyword;
2. contextual keyword;
3. ordinary identifier;
4. literal prefix.

It must not simultaneously have multiple incompatible lexical interpretations.

If "mts[...]" is the literal syntax, the parser and lexical specification must agree on whether:

mts

alone is an identifier or reserved word.

---

69. MTS Scalability

MTS syntax must not impose:

MAX_TIMELINES
MAX_BRANCHES
MAX_HISTORY
MAX_TIMESTAMP
MAX_EVENTS

A program may request or construct as many temporal structures as available semantic/runtime resources permit.

Resource exhaustion remains an implementation/runtime condition, not lexical invalidity.

---

70. Nano Syntax

Nano-domain annotations are lexical syntax only.

Examples:

@atom
@molecule(...)

The lexer MUST NOT determine:

- physical dimensions;
- chemistry;
- molecular behavior;
- biological behavior;
- material properties;
- hardware implementation.

Those belong to domain semantics.

---

71. Identifier-Like Domain Names

The following remain identifiers unless specifically reserved:

H
CNOT
FFT
CUDA
QPU
GPU
FPGA
Tensor
Agent
Molecule
Material
Device

This prevents the lexer from becoming an ever-growing dictionary of computational concepts.

---

72. Qualified Names

Qualified names are parsed from ordinary identifiers and punctuation.

For example:

vendor.operation
module::symbol
namespace::type

The lexer recognizes:

IDENTIFIER
DOT
DOUBLE_COLON

and does not resolve ownership or namespace semantics.

---

73. Numeric Suffixes

If numeric suffixes are introduced, they must be explicitly specified.

Examples:

42u64
3.14f32
10ms
5GHz

must not be partially consumed into unrelated tokens.

The specification must define:

- suffix registry;
- ambiguity rules;
- case sensitivity;
- semantic ownership.

Units belong to the type/unit system, not the hardware lexer.

---

74. Physical Units

Physical-unit literals may eventually support:

time
frequency
distance
mass
energy
power
temperature
memory quantities
quantum durations

The lexical layer only recognizes their syntax.

The semantic system determines dimensional validity.

No physical unit may imply a specific machine.

---

75. Case Sensitivity

Zamani identifiers are case-sensitive.

Therefore:

value
Value
VALUE

are distinct identifiers.

Keywords are recognized according to their exact canonical spelling unless the language specification explicitly defines another form.

The lexer MUST NOT silently case-fold identifiers.

---

76. Keyword Case

Canonical keywords use their specified spelling.

For example:

fn
let
quantum
measure

do not automatically imply:

FN
LET
QUANTUM
MEASURE

as equivalent keywords.

Case-insensitive keyword behavior would require an explicit versioned lexical change.

---

77. Reserved Built-In Type Names

Existing implementation vocabulary may reserve type names such as:

void
int
float
bool
str
string
char
Result
Never

However, the lexical reservation does not determine the complete type system.

Type meaning belongs to:

grammar/spec/type-system.md

and downstream semantic analysis.

---

78. Generic Built-In Operations

Names such as:

print
println
assert
panic
len
sizeof

may be reserved if the stable language requires them as built-in syntax.

Library functionality should normally remain ordinary identifiers.

The lexer must not encode the entire standard library.

---

79. Resource and Capability Vocabulary

Words describing resource intent may be reserved only where syntax requires it.

Examples:

requires
capability
resource
prefer
constraint
hint
target

The important distinction is:

lexical recognition

versus:

resource semantics

The lexer only performs the former.

---

80. Requirements vs Hardware Decisions

Lexical syntax must permit semantic distinctions such as:

requires capability("quantum.measurement")

versus:

map ...

The lexer must not decide whether a requirement can be satisfied.

It merely recognizes:

requires
capability
string
parentheses

and other tokens.

---

81. No Physical Resource Tokenization

The lexer must not create universal tokens such as:

QUBIT_0
CPU_0
GPU_0
FPGA_0
NODE_0
MEMORY_BANK_0

unless they occur as ordinary identifiers.

Physical identity belongs downstream.

---

82. No Fixed Widths

Lexical rules must not assume:

8-bit
16-bit
32-bit
64-bit
128-bit
256-bit

as universal machine widths.

A numeric literal may be larger than any host-native integer.

A type may later constrain its representation.

---

83. Source Size Scalability

The language places no artificial lexical source-size limit.

Implementations may stream or chunk source input.

The lexer must be capable of operating over:

tiny source
large source
generated source
very large source

subject only to available resources and explicitly configured implementation budgets.

---

84. Token Count Scalability

No universal token-count limit is part of the language.

A compiler may impose a resource budget.

Such a condition must be reported as resource exhaustion, not malformed source.

---

85. Identifier Length Scalability

No artificial language-level identifier-length limit is specified.

An implementation may enforce resource budgets.

Such enforcement must be documented and must not alter the language's semantic model.

---

86. Literal Length Scalability

The same principle applies to:

- strings;
- byte strings;
- identifiers;
- comments;
- numeric literals;
- domain literals.

No artificial universal language limits are introduced merely for implementation convenience.

---

87. Streaming Lexing

The lexer architecture SHOULD support incremental or streaming input.

Streaming must preserve the same observable lexical result as lexing the complete source.

Chunk boundaries must not affect tokenization.

For example:

"qua" + "ntum"

received as separate input chunks must produce the same result as:

"quantum"

when the stream represents the same source.

---

88. Incremental Lexing

Incremental lexing must preserve deterministic token identity after edits.

An incremental implementation may reuse unaffected lexical regions.

It must not alter lexical semantics because a source file is processed incrementally.

---

89. Parallel Lexing

Independent source units may be lexed concurrently.

For:

Source A
Source B
Source C

parallel execution must produce the same individual lexical results as sequential execution.

The lexer must not rely on shared mutable global lexical state.

---

90. Thread Safety

The lexical implementation should prefer immutable configuration.

Keyword registries, operator registries, and language-version tables should be immutable after construction.

Global mutable lexer state is prohibited where it can affect deterministic results.

---

91. Configuration

Lexical configuration may include:

language version
compatibility mode
enabled lexical dialects
diagnostic mode
resource budget
trivia retention
documentation retention

Configuration must be explicit.

The lexer must not silently inspect:

- environment variables;
- hardware;
- current time;
- current directory;
- network state;

to determine lexical meaning.

---

92. Language Versions

Lexical behavior is versioned.

A language version may change:

- keyword sets;
- literal forms;
- operator sets;
- Unicode policies;
- comment forms;
- escape sequences.

Such changes require explicit compatibility rules.

Old source must not silently acquire a different lexical meaning under an incompatible version.

---

93. Dialects

A dialect may extend lexical behavior only through an explicit dialect declaration.

A dialect must specify:

dialect name
dialect version
base language version
new tokens
changed lexical rules
compatibility behavior
AST mapping
semantic mapping

Dialects must not silently redefine core Zamani tokens.

---

94. Interoperability Formats

Interoperability formats such as:

OpenQASM
QIR
HDL
C
C++
Rust
WASM

are not separate lexical definitions of the Zamani core language.

Their parsers belong under:

grammar/interoperability/

or their corresponding frontend modules.

The Zamani lexer must not absorb foreign-language lexical syntax merely because Zamani can interoperate with that language.

---

95. Macro Lexical Boundary

Macros may operate on:

- tokens;
- token trees;
- syntax structures.

Macro expansion must not bypass lexical validity.

Generated tokens must conform to the same canonical token model before parser consumption.

Macro systems must not create hidden token kinds unavailable to the canonical grammar.

---

96. Metaprogramming Boundary

Compile-time reflection and code generation may generate source or AST structures.

The generated representation must pass through the canonical validation pipeline.

Metaprogramming must not create a second lexical language.

---

97. Lexer-to-Parser Contract

The lexer guarantees:

token kind
source span
source representation
deterministic ordering
lexical validity

The parser guarantees:

grammar structure
precedence
associativity
construct formation
syntax diagnostics

The lexer MUST NOT perform parser work.

The parser MUST NOT depend on undocumented lexer side effects.

---

98. Lexer-to-AST Contract

The lexer must expose enough information for the AST layer to construct target-independent syntax.

For literals:

raw spelling
source span
token kind

must be sufficient to preserve source meaning.

The AST must not depend on:

- hardware;
- quantum device identity;
- compiler backend;
- vendor SDK;
- physical topology.

---

99. Quantum AST Contract

Quantum lexical tokens may lower into generic frontend structures.

For example:

apply H to q

should conceptually become:

Operation {
    name: H,
    operands: [q],
    parameters: ...,
    attributes: ...,
    modifiers: ...,
    source: ...
}

rather than requiring:

enum QuantumGate {
    H,
    X,
    Y,
    ...
}

in the lexical layer.

The semantic quantum layer subsequently maps this into the canonical "quantum::ir".

---

100. Classical AST Contract

Classical operators and literals similarly lower into generic AST structures.

The lexer must not know whether:

+

will ultimately become:

- integer addition;
- floating-point addition;
- vector addition;
- matrix addition;
- tensor addition;
- symbolic addition;
- another overloaded semantic operation.

That is a type/semantic question.

---

101. HDL AST Contract

HDL lexical tokens describe source syntax.

The lexer does not determine:

- synthesis strategy;
- FPGA family;
- ASIC process;
- clock implementation;
- placement;
- routing;
- physical timing closure.

Those belong to HDL semantics and downstream compilation.

---

102. AI AST Contract

AI-related lexical constructs must lower to target-independent AST representations.

The lexer does not determine:

- training backend;
- GPU;
- accelerator;
- numerical kernel;
- framework;
- distributed strategy.

Those are semantic/compiler/runtime concerns.

---

103. Resource AST Contract

Resource-related words describe source-level intent.

The semantic model distinguishes:

requirement
constraint
capability
preference
hint
implementation decision

The lexer only recognizes their syntax.

---

104. Error Recovery and Parser Synchronization

The lexer should produce enough recovery information for the parser to continue where safe.

Recovery must not manufacture arbitrary valid tokens.

For example, an invalid character:

§

must not silently become:

;

or:

identifier

---

105. Invalid UTF-8 Recovery

Invalid UTF-8 must be reported before semantic tokenization.

The implementation may recover by consuming an invalid byte sequence according to its configured source-recovery policy, but the recovery must never silently convert it into valid source text.

---

106. Invalid Unicode Scalar Values

Surrogate code points and invalid Unicode scalar values are rejected.

A Unicode escape that represents a surrogate must produce a lexical/literal diagnostic.

---

107. Comments and Strings

Comment markers inside strings are ordinary string content.

Examples:

"// not a comment"
"/* not a comment */"

Similarly, quote characters inside comments are comment content.

The lexical state must be unambiguous.

---

108. Operators Inside Strings

Operators inside strings are not operators.

Example:

">="

is a string.

The lexer must not emit "GE" for the contents.

---

109. Operators Inside Comments

Operators inside comments are comment text.

Example:

// >= && || quantum

produces no semantic operator tokens.

---

110. Comment Termination

Line comments terminate before or at the logical line ending.

Block comments terminate at the first valid:

*/

unless a future version explicitly defines nesting.

---

111. Empty Input

Empty source is lexically valid.

The lexer emits:

EOF

with a valid source position.

---

112. Whitespace-Only Input

Whitespace-only source is lexically valid.

The parser may subsequently determine whether an empty program is syntactically valid.

The lexer does not reject whitespace-only source.

---

113. Comment-Only Input

Comment-only source is lexically valid.

The parser receives no semantic tokens other than EOF, while documentation/trivia tooling may retain the comments.

---

114. Identifier Followed by Number

The lexer must distinguish:

value42

as one identifier from:

value 42

as two tokens.

A digit may continue an identifier but may not begin one.

---

115. Number Followed by Identifier

Malformed adjacency must not be silently merged.

For example:

42abc

must not become:

INTEGER(42)
IDENTIFIER(abc)

if the language's lexical rules define the sequence as an invalid numeric/identifier adjacency.

The diagnostic policy must be explicit and tested.

---

116. Numeric Separator Validation

Separators are permitted only where the numeric grammar allows them.

Invalid examples include:

1_
_1
1__0
0x_FF
0b_101

unless a future version explicitly changes the grammar.

---

117. Leading Zeroes

Decimal leading-zero policy must be explicit.

Unless a base prefix is present:

0123

is a decimal integer according to the canonical lexical grammar.

It must not silently become octal.

Octal requires:

0o123

---

118. Hexadecimal Case

Hexadecimal digits are case-insensitive:

0xFF
0xff
0xFf

are lexically equivalent numeric forms.

The original spelling remains available to tooling.

---

119. Binary and Octal Case

The prefixes:

0b
0B
0o
0O

are accepted if the corresponding literal forms are enabled.

Their semantic value is determined downstream.

---

120. Floating Ambiguity

The lexer must distinguish:

1.

from:

1.0

according to the canonical floating/range/member-access rules.

Likewise:

1..10

must not accidentally become a floating-point literal followed by a dot if the grammar intends a range.

The lexical priority must preserve range tokens.

---

121. Dot and Range

The priority is:

..=
.. 
.

Therefore:

a..=b

must tokenize deterministically.

---

122. Negative Numbers

The minus sign is an operator.

Therefore:

-42

is lexically:

MINUS
INTEGER

not necessarily a single signed-integer token.

Semantic constant folding determines whether the expression is a negative literal value.

This avoids duplicating unary semantics inside the lexer.

---

123. Positive Numbers

Likewise:

+42

is:

PLUS
INTEGER

unless a future numeric suffix grammar explicitly defines otherwise.

---

124. String Escapes and Unicode

Unicode source characters may occur directly inside strings.

For example:

"Zamani 世界 λ"

is valid.

Unicode escapes, where supported, must represent valid scalar values.

---

125. Character Escapes

Character escapes must resolve to exactly one scalar value.

Examples:

'\n'
'\t'
'\\'
'\''

where supported by the canonical escape table.

---

126. Token Registry

The production token registry must map each lexical spelling to one canonical token identity.

Conceptually:

TokenKind
├── keyword
├── identifier
├── literal
├── operator
├── punctuation
├── trivia
├── diagnostic/recovery
└── EOF

The internal Rust representation may differ.

The observable token contract must not.

---

127. Duplicate Token Elimination

The following kinds of duplication are prohibited:

BitAnd / Ampersand
BitOr / Pipe
Question / QuestionMark
Arrow / ThinArrow

when they represent the same source spelling.

Each source spelling gets one canonical token.

The semantic interpretation happens after tokenization.

---

128. ANTLR Integration

"grammar/antlr/ZamaniLexer.g4" must implement this specification.

Its responsibility is lexical implementation.

It must not become an independent language specification.

The ANTLR grammar must not introduce tokens absent from this specification unless they are explicitly classified as:

experimental
proposed
dialect
compatibility

and are not silently treated as stable.

---

129. Rust Lexer Integration

"src/lexer.rs" is the executable Rust implementation.

It must conform to:

- this lexical specification;
- canonical token identities;
- source-span requirements;
- deterministic behavior;
- safe Rust;
- language-version rules.

It must not silently introduce lexical behavior not specified here.

---

130. Parser Integration

"src/parser.rs" consumes canonical lexical tokens.

The parser owns:

- precedence;
- associativity;
- grammar context;
- contextual keyword interpretation;
- syntax structure.

The parser must not depend on duplicate lexical identities for overloaded operators.

---

131. AST Integration

"src/ast/mod.rs" and related frontend AST files own syntactic structure.

The AST must remain domain-neutral where possible.

The lexical layer must not require AST changes merely because a new backend or hardware target is added.

---

132. Canonical Quantum IR Integration

Quantum syntax must eventually map into:

quantum::ir

The lexer does not own that representation.

The lexical layer must not introduce:

QuantumGateIR
HardwareQuantumIR
BackendQuantumIR

as competing semantic models.

---

133. Compiler Integration

Compiler stages consume semantic representations after parsing.

The lexer must not:

- select a target;
- choose a backend;
- select a gate decomposition;
- perform routing;
- schedule operations;
- perform QEC;
- perform calibration;
- query HAL.

---

134. Runtime Integration

The lexer has no runtime dependency.

Runtime state must never influence tokenization.

The lexer must not query:

- devices;
- network;
- memory availability;
- hardware;
- runtime configuration.

---

135. Hardware Integration

Hardware information belongs downstream.

The lexical language remains invariant across:

CPU
GPU
FPGA
ASIC
QPU
photonic hardware
neuromorphic systems
future substrates

---

136. Resource Integration

Resources are semantic constraints.

The lexer recognizes the syntax but does not evaluate:

requires memory >= ...
requires qubits >= ...
requires capability(...)

Resource analysis determines feasibility.

---

137. Diagnostics Integration

Diagnostics must carry source spans from the lexical layer.

The diagnostic subsystem may enrich errors with:

- source excerpts;
- suggestions;
- feature status;
- compatibility guidance.

The lexer itself remains deterministic.

---

138. Formatting Integration

A formatter may use token/trivia information to reconstruct source layout.

The formatter must not change semantic token identity.

Formatting is not lexical interpretation.

---

139. LSP Integration

The lexer must support tooling that needs:

- token ranges;
- diagnostics;
- identifier boundaries;
- comments;
- documentation comments;
- incremental updates.

LSP coordinate conversions must be deterministic.

---

140. Syntax Highlighting

Syntax highlighters may derive token classes from the canonical token model.

They must not maintain a competing keyword registry.

---

141. Documentation Generation

Documentation comments may be preserved independently of parser semantics.

Documentation generators may inspect:

/// ...
/** ... */

without modifying the language's semantic model.

---

142. Security Requirements

The lexer must defend against:

- malformed UTF-8;
- invalid Unicode;
- pathological literal sizes;
- unterminated constructs;
- tokenization ambiguity;
- non-progress loops;
- resource exhaustion;
- denial-of-service inputs.

Resource-bounded operation must remain distinguishable from invalid source.

---

143. Deterministic Ordering

When multiple diagnostics are generated, their order must be deterministic.

The preferred ordering is source order, followed by stable diagnostic priority when multiple diagnostics have identical spans.

---

144. No Environment Dependence

Lexical behavior must not depend on:

LANG
LC_ALL
TZ
OS locale
current directory
environment variables
host architecture
host endianess
device availability

unless explicitly supplied as part of a documented lexical configuration.

---

145. No Network Dependence

The lexer must operate without network access.

Keyword definitions, lexical rules, Unicode policy, and tokenization cannot be fetched dynamically from a server.

---

146. No Runtime Hardware Dependence

Lexical behavior must be identical whether the compiler runs on:

tiny machine
laptop
server
cluster
supercomputer
GPU system
QPU system
embedded system
future machine

subject only to explicit resource exhaustion.

---

147. No Fixed Quantum Limits

The lexer must never contain:

MAX_QUBITS
MAX_QREGS
MAX_QUBIT_INDEX

or equivalent universal lexical limits.

Quantum resource constraints belong to semantic/resource analysis.

---

148. No Fixed Classical Limits

The lexer must never contain:

MAX_REGISTERS
MAX_VECTOR_WIDTH
MAX_TENSOR_DIM
MAX_THREADS
MAX_CORES

as language rules.

---

149. No Fixed HDL Limits

The lexer must never impose:

MAX_SIGNAL_WIDTH
MAX_PORTS
MAX_MODULES
MAX_REGISTERS
MAX_MEMORY_BANKS

as universal language limits.

---

150. No Fixed Distributed Limits

The lexer must never impose:

MAX_NODES
MAX_PROCESSES
MAX_SERVICES
MAX_CHANNELS
MAX_PARTITIONS

as language limits.

---

151. No Fixed AI Limits

The lexer must never impose:

MAX_TENSOR_RANK
MAX_MODEL_SIZE
MAX_LAYERS
MAX_PARAMETERS
MAX_BATCH_SIZE

as language limits.

---

152. No Fixed Networking Limits

The lexer must not impose:

MAX_ENDPOINTS
MAX_CONNECTIONS
MAX_NODES
MAX_ROUTES
MAX_MESSAGES

as language rules.

---

153. Lexical Feature Lifecycle

Every lexical feature follows:

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

A token enum variant does not prove implementation.

A grammar rule does not prove implementation.

A documentation example does not prove implementation.

A feature is stable only when the complete integration contract exists.

---

154. Feature Completion Contract

For every lexical feature, completion requires:

File
Purpose
Owns
Does Not Own
Inputs
Outputs
Dependencies
Upstream Contracts
Downstream Consumers
Token Contract
Source-Span Contract
Parser Contract
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
Diagnostics
Security Review
Hard-Coding Audit
Completion Criteria

This ensures that a lexical feature can be completed independently without later redesign merely because another downstream file was completed.

---

155. Lexical Conformance Tests

The repository must maintain lexical tests under:

grammar/tests/lexical/

Tests must cover:

- UTF-8;
- Unicode identifiers;
- keywords;
- contextual keywords;
- numbers;
- strings;
- characters;
- escapes;
- comments;
- documentation comments;
- operators;
- punctuation;
- quantum literals;
- MTS literals;
- annotations;
- invalid source;
- EOF;
- source spans.

---

156. Positive Tests

Positive tests must include:

empty source
whitespace
comments
identifiers
Unicode identifiers
keywords
integer literals
floating literals
strings
characters
operators
ranges
qualified names
quantum literals
MTS literals
annotations
generic syntax
domain-independent operations

---

157. Negative Tests

Negative tests must include:

invalid UTF-8
invalid identifier starts
invalid escapes
unterminated strings
unterminated characters
unterminated block comments
invalid numbers
invalid numeric separators
invalid Unicode scalar escapes
invalid operators
invalid domain literals
forbidden unsafe constructs

---

158. Boundary Tests

Boundary tests must cover:

- empty input;
- one-character source;
- one-token source;
- very long identifier;
- very long numeric literal;
- very long string;
- very long comment;
- Unicode at source boundaries;
- token at EOF;
- multi-byte UTF-8 adjacent to ASCII;
- operator prefixes;
- nested lexical-looking constructs inside strings/comments.

---

159. Scalability Tests

Scalability tests must prove that the language does not impose artificial limits.

They should cover progressively larger:

source files
identifier lengths
numeric literals
strings
comments
token streams
module inputs
generated programs

The tests must distinguish:

language rejection

from:

implementation resource exhaustion

---

160. Determinism Tests

The same source must be lexically identical when processed:

- sequentially;
- repeatedly;
- incrementally;
- from different input chunk boundaries;
- on different supported host platforms.

---

161. Parallelism Tests

Independent source units may be lexed in parallel.

The result must equal sequential lexing.

Tests must ensure there is no shared mutable state that changes lexical output.

---

162. Compatibility Tests

Compatibility tests must compare:

language version
token stream
diagnostics
reserved words
literal behavior
operator behavior
Unicode behavior

A compatible release must not silently change lexical identity.

---

163. Cross-Repository Conformance

Lexical conformance must eventually be checked across:

grammar/spec/lexical.md
grammar/antlr/ZamaniLexer.g4
src/lexer.rs
src/parser.rs
src/ast/
grammar/Zamani.g4
grammar/grammar.md
grammar/tests/

A feature is conformant only when all relevant representations agree.

---

164. "grammar/grammar.md" Integration

"grammar/grammar.md" must report implementation status against this document.

It must distinguish:

SPECIFIED
IMPLEMENTED
TESTED
CONFORMANT
EXPERIMENTAL
DEPRECATED
UNIMPLEMENTED

It must never present an enum variant as proof that a lexical feature is implemented.

---

165. "grammar/Zamani-Grammar.md" Integration

"grammar/Zamani-Grammar.md" remains a broad design/history/proposal document.

Features described there are not automatically stable language features.

A feature becomes stable only after:

proposal
→ specification
→ lexical contract
→ syntax contract
→ AST contract
→ semantic contract
→ IR contract
→ implementation
→ tests
→ compatibility validation

---

166. "grammar/Zamani.g4" Integration

"grammar/Zamani.g4" is the composition root.

It must not redefine lexical behavior independently.

Its parser rules consume the canonical token model.

---

167. "grammar/antlr/ZamaniLexer.g4" Integration

The existing "grammar/antlr/ZamaniLexer.g4" must be brought into exact conformance with this document.

In particular, its implementation must be audited for:

- "UNSAFE";
- Unicode identifiers;
- keyword inventory;
- duplicate tokens;
- numeric forms;
- escape rules;
- comments;
- documentation comments;
- MTS literals;
- quantum literals;
- annotation tokenization;
- operator precedence at lexical level;
- token ordering.

No second lexical authority may remain.

---

168. "src/lexer.rs" Integration

"src/lexer.rs" must be audited against every token category here.

The implementation must:

- use safe Rust;
- preserve source spans;
- preserve literal source representation;
- avoid native-width overflow;
- guarantee progress;
- emit deterministic diagnostics;
- avoid hardware/environment dependencies;
- avoid duplicate lexical token identities.

---

169. "src/parser.rs" Integration

"src/parser.rs" must consume canonical tokens.

It must interpret overloaded tokens according to grammar context.

It must not require lexical variants such as:

BitAnd
Ampersand

for the same spelling.

---

170. "src/ast/mod.rs" Integration

The AST remains target-independent.

It should preserve:

- source spans;
- generic operations;
- names;
- literals;
- attributes;
- modifiers;
- effects;
- capabilities;
- resource intent.

The lexical layer must not encode backend-specific AST variants.

---

171. Type-System Integration

"grammar/spec/type-system.md" owns semantic interpretation of:

- integer widths;
- floating precision;
- quantum types;
- resource types;
- capability types;
- references;
- ownership;
- generic types.

Lexical syntax must remain representation-neutral.

---

172. Semantic Integration

The semantic layer determines whether a syntactically valid literal or identifier is semantically valid.

For example:

999999999999999999999999

is lexically valid.

Whether it fits a selected type is a semantic question.

---

173. Canonical IR Integration

The lexical layer terminates before canonical semantic IR.

No lexical token is itself an IR instruction.

The chain is:

token
→ AST
→ semantic model
→ canonical IR

---

174. Quantum IR Integration

Quantum syntax eventually reaches:

quantum::ir

without the lexer creating a parallel semantic representation.

This preserves the repository's canonical quantum boundary.

---

175. QEC Integration

QEC is not lexical.

The lexer does not:

- encode correction strategies;
- allocate logical qubits;
- select codes;
- determine distances;
- schedule correction.

Those belong downstream.

---

176. ZQN Integration

ZQN is not lexical.

Lexical constructs may describe source-level fault/noise intent, but the lexer does not determine:

- noise models;
- fault semantics;
- probabilities;
- calibration;
- physical error behavior.

---

177. Routing Integration

Routing is downstream.

The lexer must not encode:

physical_qubit_0
physical_qubit_1

as special universal tokens.

They remain ordinary identifiers unless a target-specific dialect explicitly defines otherwise.

---

178. Scheduling Integration

Scheduling is downstream.

Lexical syntax does not determine:

- execution time;
- instruction duration;
- resource conflicts;
- hardware timing;
- pulse scheduling.

---

179. HAL Integration

HAL is downstream.

The lexer must never query or encode actual device capabilities.

---

180. Calibration Integration

Calibration is downstream.

Lexical source may contain declarative calibration intent, but calibration data and physical device state do not belong to lexical analysis.

---

181. Benchmarking Integration

Benchmark syntax may exist as domain syntax, but the lexer does not execute benchmarks or inspect hardware.

---

182. Resource Manager Integration

Resource managers operate after semantic analysis.

The lexer does not reserve:

- memory;
- CPUs;
- GPUs;
- QPUs;
- FPGA resources;
- network resources.

---

183. Provenance

Lexical diagnostics and token streams may contribute source provenance.

The lexer must preserve enough information to trace syntax back to source.

Generated source must be distinguishable from original source through the source-management layer, not by changing token semantics.

---

184. Reproducibility

Given identical:

source
language version
lexical configuration

lexical output must be reproducible.

No external state may alter the result.

---

185. Build Reproducibility

The lexer implementation must not embed:

- timestamps;
- host paths;
- machine IDs;
- device IDs;
- random values;

into lexical output.

---

186. Internationalization

User-facing diagnostics may eventually support multiple languages.

Diagnostic identity must remain language-independent.

The token model must not change based on diagnostic locale.

---

187. Source Preservation

Where practical, the frontend should preserve:

raw source
tokens
trivia
spans

to enable:

- formatter stability;
- refactoring;
- IDE support;
- diagnostics;
- source-to-source transformations.

---

188. No Silent Semantic Normalization

The lexer must never silently transform:

identifier
number
string
character
quantum literal
MTS literal

into a different semantic value.

Lexical decoding must be explicit and deterministic.

---

189. Lexical Purity

The lexer is conceptually a pure transformation:

(source, version, lexical configuration)
    →
(tokens, diagnostics)

No hidden mutable global state may alter the result.

---

190. Forbidden Lexical Dependencies

The lexer MUST NOT depend on:

quantum hardware
classical hardware
GPU drivers
FPGA tools
network services
runtime state
device discovery
calibration state
compiler cache contents
current time
randomness

---

191. Feature-Gated Lexical Extensions

Experimental lexical features must be explicitly gated.

For example:

experimental
unstable
dialect(...)
compatibility(...)

must not silently become stable syntax.

---

192. Backward Compatibility

When a previously ordinary identifier becomes a reserved keyword, the language version must classify that change as source-incompatible.

The compatibility system must provide migration information.

---

193. Deprecation

Deprecated keywords/literals may continue to lex normally while generating downstream deprecation diagnostics.

Deprecation must not create duplicate token kinds.

---

194. Removed Features

Removed lexical features must not be silently reinterpreted as another valid token.

The compiler should provide an explicit compatibility diagnostic where appropriate.

---

195. Token Naming

Token names in implementations should describe lexical identity, not semantic meaning.

Prefer:

AMPERSAND
PIPE
QUESTION
ARROW
IDENTIFIER
INTEGER
FLOAT

over semantic names such as:

REFERENCE_OPERATOR
BITWISE_AND
QUANTUM_GATE

when one spelling can have multiple semantic meanings.

---

196. Token Metadata

The implementation may associate metadata with tokens.

Useful metadata includes:

kind
span
raw text
decoded value
trivia relationship
language version

The metadata must remain deterministic.

---

197. Token Equality

Token equality for parser purposes should be based on canonical token identity and relevant value data.

Source positions should not accidentally change semantic token equality.

---

198. Source Identity

If multiple source files are lexed, source identity must remain distinguishable.

A span from:

module A

must never be confused with an identical offset from:

module B

---

199. Include / Import Boundaries

Lexical analysis of imported modules remains independent.

The lexer does not resolve imports.

Module resolution belongs to the module/package subsystem.

---

200. Generated Source

Generated source must pass through the same lexical contract unless it is represented directly as canonical AST/IR by an explicitly specified compiler interface.

Generated code cannot silently introduce private token kinds.

---

201. Macro-Generated Tokens

Macro expansion must use canonical tokens.

A macro cannot invent a new semantic token by changing the token's internal enum without updating the language specification and compatibility model.

---

202. Domain Extensibility

A new computing domain must not require rewriting the lexical architecture.

A domain may add:

syntax
keywords
contextual keywords
literals
dialect extensions

only when genuinely required.

The preferred extension model remains:

existing lexical primitives
+
existing compositional grammar
+
semantic vocabulary

---

203. Future Computational Paradigms

The lexical architecture must remain usable for future:

- quantum paradigms;
- photonic computing;
- neuromorphic computing;
- molecular computing;
- biological computing;
- optical computing;
- reversible computing;
- analog computing;
- distributed substrates;
- unknown future computational models.

The lexer must not encode today's hardware as the permanent shape of Zamani.

---

204. POCO-REAF Definition at Lexical Layer

POCO-REAF means that the source lexical form describes the program independently of the machine that ultimately realizes it.

Lexical compatibility therefore requires:

same source
+
same language version
+
same lexical configuration
=
same lexical meaning

regardless of target hardware.

---

205. Tiny-to-Large Principle

The same lexical language must support:

single expression

through:

large software system
large scientific program
large quantum program
large HDL/software co-design
large distributed program
large AI workload
large generated program

without introducing separate lexical languages merely because the program is larger.

---

206. Infinity Principle

“Infinity” means:

«the language does not impose artificial finite limits where the underlying semantic model can remain abstract and the actual limit is determined by available resources.»

It does not mean that a finite machine can physically store an infinite source.

Therefore the specification distinguishes:

unbounded language model

from:

finite implementation resources

---

207. Hard-Coding Audit

Every lexical implementation change must be checked for hard-coded:

hardware counts
resource counts
machine widths
device IDs
topology
vendor names
backend names
fixed tensor dimensions
fixed qubit counts
fixed node counts
fixed thread counts
fixed memory capacities

If such a value is necessary for lexical syntax, it must be justified explicitly as language syntax rather than an implementation limit.

---

208. Production Acceptance Criteria

"grammar/spec/lexical.md" is considered implemented only when:

- [ ] "src/lexer.rs" conforms;
- [ ] "grammar/antlr/ZamaniLexer.g4" conforms;
- [ ] token duplicates are removed;
- [ ] keyword authority is centralized;
- [ ] Unicode policy is consistent;
- [ ] numeric literals are representation-safe;
- [ ] strings and characters have canonical escapes;
- [ ] comments are deterministic;
- [ ] documentation comments are recoverable;
- [ ] longest-match behavior is tested;
- [ ] source spans are preserved;
- [ ] invalid UTF-8 is diagnosed;
- [ ] lexical recovery guarantees progress;
- [ ] "unsafe" language constructs are rejected;
- [ ] Rust implementation uses no "unsafe";
- [ ] quantum literals are target-independent;
- [ ] MTS status is unambiguous;
- [ ] domain operations are not hard-coded as exhaustive keywords;
- [ ] hardware limits are absent from lexical rules;
- [ ] resource exhaustion is distinct from invalid source;
- [ ] incremental lexing is deterministic;
- [ ] parallel lexing is deterministic;
- [ ] compatibility tests exist;
- [ ] negative tests exist;
- [ ] boundary tests exist;
- [ ] scalability tests exist;
- [ ] parser conformance exists;
- [ ] AST mapping exists;
- [ ] canonical IR integration is documented;
- [ ] "quantum::ir" remains the canonical quantum semantic boundary.

---

209. Final Lexical Architecture

The production architecture is:

                         SOURCE
                           │
                           ▼
                    UTF-8 validation
                           │
                           ▼
                  Unicode scalar stream
                           │
                           ▼
                    lexical scanner
                           │
          ┌────────────────┼────────────────┐
          │                │                │
       tokens           trivia          diagnostics
          │                │                │
          └────────────────┼────────────────┘
                           ▼
                       parser
                           │
                           ▼
                    frontend AST
                           │
                           ▼
              structural validation
                           │
                           ▼
                  semantic analysis
                           │
          ┌────────────────┼─────────────────┐
          │                │                 │
        types           effects          resources
          │                │                 │
          └────────────────┼─────────────────┘
                           ▼
                 canonical semantic model
                           │
                           ▼
                     canonical IR
                           │
          ┌────────────────┼──────────────────┐
          │                │                  │
     classical IR      quantum::ir       HDL/other IR
          │                │                  │
          └────────────────┼──────────────────┘
                           ▼
                     optimization
                           │
              ┌────────────┼────────────┐
              │            │            │
           routing     scheduling    resilience
              │            │            │
              └────────────┼────────────┘
                           ▼
                          ZQN
                           │
                          HAL
                           │
                           ▼
                 target realization
                           │
          ┌────────────────┼───────────────────┐
          │                │                   │
         CPU              GPU                 FPGA
          │                │                   │
         QPU        accelerator          future target
          └────────────────┴───────────────────┘

The lexical layer therefore has one job:

«Convert portable Zamani source text into a deterministic, source-preserving, versioned token stream without embedding machine limits, backend assumptions, quantum hardware assumptions, or runtime behavior.»

Everything beyond that boundary belongs to the parser, AST, semantic system, canonical IR, compiler, runtime, and target realization layers.

---

210. Normative Summary

The following rules are mandatory.

1. UTF-8 is the canonical source encoding.
2. Invalid UTF-8 is diagnosed.
3. Unicode identifiers are supported according to the canonical identifier profile.
4. Identifiers are case-sensitive.
5. Identifier normalization is not silently performed.
6. Keywords have one canonical registry.
7. Contextual keywords are preferred where possible.
8. Domain operations are not exhaustively encoded as keywords.
9. Operators have one canonical lexical identity per spelling.
10. Longest valid token wins.
11. Integer literals are not constrained by host integer width.
12. Floating literals are not silently rounded during lexing.
13. Numeric source representation is preserved.
14. Strings have a canonical escape system.
15. Character literals represent exactly one Unicode scalar value.
16. Comments never become semantic tokens.
17. Documentation comments remain recoverable for tooling.
18. Invalid source never becomes silently valid source.
19. Lexical recovery must guarantee progress.
20. Every source token has a source span.
21. Lexical behavior is deterministic.
22. Lexical behavior is independent of hardware.
23. Lexical behavior is independent of network state.
24. Lexical behavior is independent of wall-clock time.
25. Lexical behavior is independent of host locale.
26. Lexical behavior is independent of machine size.
27. No artificial universal source/token/identifier limits are defined.
28. Implementation resource limits are separate from language validity.
29. "unsafe" Rust is prohibited.
30. The Zamani language does not provide an unsafe execution escape hatch.
31. Quantum gates are not a fixed lexical vocabulary.
32. Quantum literals do not allocate hardware.
33. MTS literals do not allocate timelines.
34. Nano annotations do not execute domain behavior.
35. Hardware identities remain ordinary data/identifiers.
36. Resource requirements remain semantic intent.
37. The lexer does not perform QEC.
38. The lexer does not perform ZQN processing.
39. The lexer does not route.
40. The lexer does not schedule.
41. The lexer does not calibrate.
42. The lexer does not select a backend.
43. The lexer does not query HAL.
44. "src/lexer.rs" and "grammar/antlr/ZamaniLexer.g4" implement this contract.
45. "src/parser.rs" interprets canonical tokens.
46. The frontend AST remains target-independent.
47. "quantum::ir" remains the canonical quantum semantic boundary.
48. "grammar/grammar.md" reports implementation conformance.
49. "grammar/Zamani-Grammar.md" remains broader design/history/proposal material.
50. Every lexical feature requires specification, implementation, tests, conformance, compatibility, scalability, and hard-coding validation before becoming stable.

This is the complete normative lexical boundary for production Zamani.