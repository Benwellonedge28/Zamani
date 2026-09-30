Zamani Keyword-Collision Validation and Canonicalization Specification

Path: "grammar/validation/keyword-collisions.md"
Language: Zamani
Status: Normative / Production-Ready Validation Contract
Specification role: Keyword, identifier, token, operator, punctuation, contextual-keyword, dialect, Unicode, compatibility, and lexical-collision validation
Rust baseline: Rust 1.97 / Rust 1.97.1
Rust edition: Rust 2021
Rust safety: Safe Rust only; Zamani-owned production Rust MUST NOT use "unsafe"
Canonical root grammar: "grammar/Zamani.g4"
Canonical lexical specification: "grammar/specification/lexical.md"
Canonical keyword specification: "grammar/lexer/keywords.md"
Canonical token specification: "grammar/lexer/tokens.md"
Canonical operator specification: "grammar/lexer/operators.md"
Canonical duplicate-token specification: "grammar/validation/duplicate-tokens.md"
Canonical ambiguity specification: "grammar/validation/ambiguity.md"
Canonical Rust lexer: "src/lexer.rs"
Canonical parser: "src/parser.rs"
Canonical frontend AST: "src/frontend/ast/" where implemented
Canonical quantum semantic boundary: "quantum::ir"
Portability objective: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)

---

1. Purpose

This document defines the complete production contract for detecting, preventing, resolving, and continuously validating collisions involving Zamani keywords.

It covers collisions between:

- keywords and identifiers;
- keywords and contextual identifiers;
- keywords and operators;
- keywords and punctuation;
- keywords and literals;
- keywords and Unicode identifiers;
- keywords and Unicode confusables;
- keywords and dialect extensions;
- keywords and macros;
- keywords and interoperability vocabularies;
- keywords and historical aliases;
- keyword names and token names;
- ANTLR vocabulary and Rust token vocabulary;
- generated lexer vocabulary and hand-written lexer vocabulary;
- parser expectations and lexer output;
- source-level vocabulary and AST representation.

The central invariant is:

«A source spelling has exactly one canonical lexical classification for a given language version and explicitly selected lexical configuration.»

A spelling MUST NOT have multiple independently emittable canonical token identities merely because different parts of the repository historically assigned different names to it.

Likewise, a language keyword MUST NOT accidentally become unavailable as an identifier without an explicit reservation decision.

The keyword-collision validator therefore protects the complete frontend:

source
  │
  ▼
Unicode decoding
  │
  ▼
keyword / identifier / literal / operator classification
  │
  ▼
canonical token stream
  │
  ▼
parser
  │
  ▼
domain-neutral AST
  │
  ▼
semantic analysis
  │
  ▼
canonical semantic model
  │
  ├───────────────┬──────────────────┐
  ▼               ▼                  ▼
classical      quantum::ir      HDL/hardware
  │               │                  │
  └───────────────┴──────────────────┘
                  │
                  ▼
       optimization / lowering
                  │
          routing / scheduling
                  │
          resilience / QEC / ZQN
                  │
                  ▼
                 HAL
                  │
                  ▼
          target realization

Keyword validation is therefore not merely a lexer quality check. It is a language-consistency invariant.

---

2. Normative Language Authority

The repository contains multiple documents and representations of Zamani syntax. They MUST NOT become competing authorities.

The authority relationship is:

grammar/specification/
        │
        ▼
grammar/spec/
        │
        ▼
grammar/lexer/
        │
        ├── keywords.md
        ├── tokens.md
        ├── operators.md
        ├── identifiers.md
        └── lexical component grammars
        │
        ▼
grammar/antlr/
        │
        ├── ZamaniLexer.g4
        └── ZamaniParser.g4
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
AST
        │
        ▼
semantic analysis
        │
        ▼
canonical IR

The responsibilities are:

"grammar/DESIGN.md"

Owns architecture and invariants.

It does not own the detailed keyword inventory.

"grammar/Zamani.g4"

Owns canonical root grammar composition.

It MUST NOT independently create keyword identities that disagree with the lexical authority.

"grammar/lexer/keywords.md"

Owns the normative keyword inventory and keyword policy.

"grammar/lexer/keywords.g4"

Owns executable ANTLR keyword rules.

"grammar/lexer/tokens.g4"

Owns canonical token declarations where applicable.

"grammar/lexer/operators.g4"

Owns operator lexical identity.

"grammar/antlr/ZamaniLexer.g4"

Owns assembled ANTLR lexer composition.

"src/lexer.rs"

Owns the executable Rust lexer implementation.

It MUST conform to the canonical lexical contract.

"src/parser.rs"

Consumes canonical token identities.

It MUST NOT create alternative lexical identities for the same source spelling.

"grammar/grammar.md"

Documents implementation conformance.

It is not a second keyword authority.

"grammar/Zamani-Grammar.md"

May contain historical, proposed, experimental, or future vocabulary.

Its contents do not automatically reserve source spellings.

---

3. Core Collision Invariant

For every language version "V" and active lexical configuration "C":

canonical_token(spelling, V, C)

MUST return at most one canonical token identity.

For a reserved keyword:

keyword(spelling, V, C) = exactly one canonical keyword token

For an ordinary identifier:

keyword(spelling, V, C) = none

For an intentional contextual keyword:

keyword(spelling, V, C) = contextual classification

but the spelling MUST NOT acquire multiple simultaneously emitted token kinds for the same lexical position.

The following is prohibited:

"?" → Question
"?" → QuestionMark

when both are independently emitted.

Likewise:

"&" → BitAnd
"&" → Ampersand

is prohibited.

The following is valid:

"&" → Ampersand

with semantic interpretation determined later by context.

---

4. Existing Collision Corrections

The repository already identifies several duplicate lexical identities.

These MUST be canonicalized as follows.

Source spelling| Canonical identity| Legacy/duplicate identity| Required state
"?"| "QuestionMark"| "Question"| canonical only
"&"| "Ampersand"| "BitAnd"| canonical only
"|"| "Pipe"| "BitOr"| canonical only
"->"| "ThinArrow"| "Arrow"| canonical only
"=>"| "FatArrow"| none| canonical
"&&"| "LogicalAnd"| none| canonical
"||"| "LogicalOr"| none| canonical

The canonicalization is intentionally aligned with the existing lexical/operator contracts.

4.1 "?"

Canonical:

QUESTION_MARK
QuestionMark

"Question" MUST NOT remain independently emittable.

The parser MUST consume "QuestionMark".

The semantic layer may distinguish:

T?
expression?

or other contexts without introducing another lexical token.

---

4.2 "&"

Canonical:

AMPERSAND
Ampersand

"BitAnd" MUST NOT be independently emitted.

The source spelling:

&

may later represent:

- bitwise conjunction;
- reference syntax;
- borrowing;
- another explicitly specified semantic construct.

Those are parser/semantic distinctions.

They are not separate lexical spellings.

---

4.3 "|"

Canonical:

PIPE
Pipe

"BitOr" MUST NOT be independently emitted.

The source spelling may participate in:

a | b

or other syntactic forms.

Quantum state notation such as:

|0⟩
|1⟩
|+⟩
|-⟩
|ψ⟩

MUST be handled by the quantum-literal lexical contract when the complete form is defined as a quantum literal.

That does not justify two ordinary tokens for "|".

---

4.4 "->"

Canonical:

THIN_ARROW
ThinArrow

"Arrow" MUST NOT be independently emitted.

Every parser rule representing "->" MUST consume "ThinArrow".

---

4.5 "=>"

Canonical:

FAT_ARROW
FatArrow

No second canonical identity may be introduced.

---

5. Keyword Versus Identifier

The most important keyword-collision rule is:

«Reserve a spelling only when lexical reservation is required by language syntax.»

A name does not become a keyword merely because:

- it is commonly used;
- it is a standard-library function;
- it is a quantum gate;
- it is a hardware vendor;
- it is an AI framework;
- it is a backend;
- it is a mathematical function;
- it is an API;
- it appears in "Zamani-Grammar.md";
- it appears in historical grammar material.

For example, the following should remain identifiers unless a future normative specification explicitly reserves them:

H
X
Y
Z
S
T
CNOT
CZ
SWAP
RX
RY
RZ
U
Toffoli
custom_gate
vendor_gate
CUDA
ROCm
Qiskit
TensorFlow
NVIDIA
AMD
Intel
IBM
fft
svd
matmul
gradient
serialize
deserialize

This is necessary for extensibility.

---

6. Stable Keyword Principle

A keyword SHOULD exist only when at least one of these conditions is true:

1. It introduces a language-level syntactic construct.
2. It changes parsing structure.
3. It is necessary to disambiguate the language grammar.
4. It represents a stable language-level declaration category.
5. It is required for compatibility with an already-stable Zamani syntax.
6. It is required by a formally specified contextual grammar.

A keyword SHOULD NOT exist merely because a feature exists.

For example:

apply

may be a language keyword because it introduces quantum operation syntax.

But:

H
CNOT
RX
custom_gate

should normally remain operation identifiers.

This permits:

apply H to q
apply CNOT to q0, q1
apply custom_gate to q
apply vendor.operation(parameter) to q

without expanding the reserved-word set.

---

7. Keyword Classes

Keyword validation MUST classify keywords into explicit categories.

At minimum:

CORE
DECLARATION
TYPE
CONTROL_FLOW
MODULE
VISIBILITY
FUNCTION
CONCURRENCY
EFFECT
CONTRACT
QUANTUM
CLASSICAL
HDL
HARDWARE
RESOURCE
DISTRIBUTED
AI
DATA
NETWORKING
SECURITY
COMPILATION
EXECUTION
INTEROPERABILITY
MACRO
METAPROGRAMMING
DIALECT
SANKOFA
TEMPORAL
NANO
LITERAL
DIAGNOSTIC
CONTEXTUAL
COMPATIBILITY
DEPRECATED

A keyword MUST belong to one authoritative category.

The category MUST NOT affect token identity.

For example:

measure → MEASURE

remains one token regardless of whether the parser later encounters it inside quantum syntax.

---

8. Keyword Registry Requirements

The keyword registry MUST provide, for every reserved spelling:

keyword_id
source_spelling
canonical_token
category
status
introduced_version
deprecated_version
replacement
contextual_or_reserved
case_policy
unicode_policy
dialect_scope
parser_consumers
AST_mapping
semantic_mapping
compatibility_policy
tests

The registry MUST NOT rely on implicit alphabetical ordering or source-file ordering for correctness.

Ordering may be used for deterministic serialization only.

---

9. One Spelling, One Token Identity

The validator MUST reject:

IF      : "if";
IF_WORD : "if";

Likewise:

QUESTION      : "?";
QUESTION_MARK : "?";

Likewise:

AMPERSAND : "&";
BIT_AND   : "&";

Likewise:

PIPE   : "|";
BIT_OR : "|";

Likewise:

ARROW      : "->";
THIN_ARROW : "->";

The existence of aliases is permitted only if they are non-emitting compatibility names.

---

10. Compatibility Aliases

A historical token name MAY remain temporarily in source code when migration requires it.

Example:

legacy: Question
canonical: QuestionMark

However:

lexer output:
QuestionMark

MUST be the only production behavior.

A compatibility alias MUST satisfy:

alias → canonical token

and never:

source spelling → alias token

The validator MUST distinguish:

legacy source-code name

from:

emittable lexical token

The former may be temporarily tolerated.

The latter is prohibited.

---

11. Keyword and Operator Collision

Keywords and operators occupy separate lexical classes.

A keyword MUST NOT accidentally absorb an operator prefix.

For example:

if

must not interfere with:

ifx

Therefore:

ifx

must be an identifier if "if" is reserved and identifier rules permit it.

The lexer MUST NOT tokenize:

ifx

as:

IF
IDENTIFIER("x")

unless the language explicitly specifies such lexical behavior.

---

12. Keyword Boundary Rule

A keyword match MUST obey identifier boundaries.

For a keyword:

if

the following MUST remain identifiers when identifiers permit them:

ifx
ifdef
if_condition

The exact identifier grammar determines the legal forms.

The keyword validator MUST therefore test every reserved keyword against:

keyword
keyword_suffix
keyword_prefix
keyword_identifier
keyword_with_digits
keyword_with_unicode_identifier_suffix

where each form is legal under the identifier specification.

---

13. Keyword Prefix Collisions

Every keyword MUST be checked against:

1. shorter keywords;
2. longer keywords beginning with the same spelling;
3. identifiers beginning with the keyword;
4. contextual keywords;
5. dialect keywords.

Example:

in
infer

If both exist, the lexer MUST deterministically recognize:

infer

as "INFER".

It MUST NOT tokenize:

infer

as:

IN
IDENTIFIER("fer")

when "infer" is a reserved keyword.

This rule applies to all keyword prefix relationships.

---

14. Keyword and Numeric Boundary

Keywords MUST NOT match the beginning of a valid identifier containing digits.

For example:

match2
async2
quantum2
measure2

must be classified according to the identifier rules, not split because the prefix happens to be a keyword.

---

15. Case Sensitivity

Zamani keyword matching MUST be explicitly defined.

The canonical policy is:

«Keyword spellings are case-sensitive unless a future normative language version explicitly establishes case-insensitive behavior.»

Therefore:

if

does not automatically mean:

IF
If
iF

The validator MUST reject an implementation that silently treats keyword case differently from the specification.

This applies equally to:

- Rust lexer;
- ANTLR lexer;
- parser;
- syntax highlighter;
- formatter;
- IDE tooling.

---

16. Unicode Normalization

Unicode source handling MUST NOT silently normalize different source spellings into different keyword identities unless normalization is explicitly specified.

The lexer MUST distinguish:

ASCII keyword

from visually similar Unicode text.

For example, Unicode confusables MUST NOT silently become:

if

or another ASCII keyword.

Normalization policy belongs to the lexical specification.

The collision validator MUST verify that the Rust lexer and ANTLR lexer apply the same policy.

---

17. Unicode Confusable Protection

The validator SHOULD maintain a confusable analysis set for:

- ASCII keywords;
- punctuation;
- operators;
- security-sensitive identifiers.

Examples of visually similar characters include:

＝
＜
＞
∧
∨

These MUST NOT silently become:

=
<
>
&&
||

unless a normative language extension explicitly establishes those equivalences.

This protects:

- deterministic parsing;
- source review;
- portability;
- security;
- reproducible builds.

---

18. Unicode Mathematical Vocabulary

Mathematical symbols may be part of Zamani without becoming ordinary reserved keywords.

For example:

Π
Σ
∀
∃
λ

may have dedicated lexical roles.

The validator MUST distinguish:

symbol

from:

keyword

and:

identifier

The existence of:

Π
Σ

does not automatically reserve:

Pi
Sigma

as identifiers.

---

19. Keyword and Literal Collision

Keyword spellings MUST NOT collide with literal syntax.

For example:

true
false
nil
null

are literal vocabulary if specified as such.

The validator MUST ensure that:

true

cannot simultaneously be:

TRUE
IDENTIFIER("true")

in the same lexical configuration.

Likewise, string and character delimiters MUST remain unambiguous.

---

20. Boolean and Null Vocabulary

The currently established literal vocabulary includes:

true
false
nil
null

Canonical token identities MUST remain unique.

The validator MUST verify:

true  → TRUE
false → FALSE
nil   → NIL
null  → NULL

and never:

true → TRUE | Identifier

within the same language configuration.

---

21. Operator Maximal-Munch Interaction

Keyword validation cannot be separated from operator validation.

The lexical validator MUST verify longest-match behavior for overlapping operators.

Examples:

.
..
..=

:
::

?
?.
??

!
!=

<
<=
<<

>
>=
>>

+
++
+=

-
--
-=
->

*
*=

/
 /=

%
%=

&
&&
&=

|
||
|=

^
^=

The lexer MUST select the complete valid operator where applicable.

---

22. Keyword/Operator Mixed Prefix Cases

The validator MUST include mixed cases such as:

if->x
return->x
measure->q
apply->q
async->x

and verify that keyword classification ends exactly at the identifier boundary and operator classification begins at the correct character.

Whitespace MUST NOT be required unless the lexical specification explicitly requires it.

---

23. Keyword and Punctuation Collision

Punctuation has lexical ownership independent of keywords.

Examples:

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

A keyword MUST NOT consume punctuation as part of its lexical identity unless that complete spelling is explicitly defined as one token.

For example:

async

and:

async(...)

must remain a keyword followed by punctuation/call syntax.

---

24. Keyword and Attribute Collision

Zamani uses attribute-like syntax such as:

@name

and related constructs.

The lexer MUST classify:

@

according to punctuation ownership.

The following:

@observe
@living_doc
@temporal_learn

MUST NOT require the lexer to invent a unique keyword token for every attribute.

Unless an attribute is formally defined as a keyword, the preferred representation is:

AT
IDENTIFIER

followed by parser/semantic interpretation.

This prevents uncontrolled growth of the keyword vocabulary.

---

25. Contextual Keywords

A contextual keyword is a spelling that acts as a keyword only in a defined syntactic position.

Contextual treatment SHOULD be preferred when global reservation is unnecessary.

Potential examples include vocabulary such as:

model
move
switch
then
ancestor

if the final grammar can distinguish their contexts structurally.

A contextual keyword MUST have:

context
activation rule
parser owner
AST mapping
compatibility behavior
diagnostic behavior

It MUST NOT be context-sensitive because of:

- hardware;
- resource availability;
- filesystem state;
- installed packages;
- runtime state;
- network state;
- randomness.

---

26. Contextual Keyword Determinism

A contextual keyword MUST be determined solely from:

source tokens
language version
explicit feature configuration
explicit dialect configuration

It MUST NOT depend on:

current hardware
available GPU
available QPU
available memory
network state
environment variables
runtime state
time
randomness
filesystem discovery

This is mandatory for POCO-REAF.

---

27. Dialect Keyword Collisions

Dialects MUST NOT silently steal core keywords.

Every dialect keyword must declare:

dialect
version
spelling
token identity
scope
activation condition
core compatibility
conflict behavior

If a dialect attempts to redefine an existing core keyword, the validator MUST reject the configuration unless the language specification explicitly defines a compatible contextual extension.

---

28. Dialect Collision Example

A dialect MUST NOT silently redefine:

measure

from:

MEASURE

to:

DIALECT_MEASURE

for the same source and configuration.

Instead, the dialect must either:

1. use the existing canonical token;
2. introduce a different spelling;
3. define explicit contextual syntax;
4. establish a versioned language extension.

---

29. Macro Keyword Collisions

Macro expansion MUST NOT silently create a different lexical language.

Macro-produced tokens MUST pass through the same canonical token identity model.

A macro MUST NOT manufacture two identities for:

?
&
|
->

or any other canonical spelling.

Macro expansion therefore follows:

macro source
   ↓
token tree / syntax representation
   ↓
canonical lexical identity
   ↓
parser

rather than introducing a parallel keyword vocabulary.

---

30. Macro Hygiene

Macro hygiene MUST preserve the distinction between:

keyword
identifier
generated identifier
literal
operator
punctuation

A macro-generated identifier MUST NOT accidentally become a keyword merely because of textual concatenation unless the macro system explicitly specifies token construction semantics that produce a keyword token.

Likewise, a keyword MUST NOT accidentally become an identifier because it originated from a macro.

---

31. Interoperability Vocabulary

Foreign-language keywords MUST NOT automatically become Zamani core keywords.

For example, Rust vocabulary such as:

unsafe
trait
impl
extern

may occur in interoperability contexts.

That does not automatically make their Rust semantics Zamani semantics.

Interoperability-specific interpretation belongs under:

grammar/interoperability/

The keyword validator must distinguish:

Zamani core reservation

from:

foreign-language vocabulary

---

32. "unsafe" Special Case

The source spelling:

unsafe

requires explicit treatment.

The compiler implementation requirement is:

«Zamani-owned production Rust uses no "unsafe".»

That does not automatically determine whether the source language may contain an "unsafe" keyword.

If "unsafe" remains for compatibility or interoperability, the language specification MUST define its source semantics explicitly.

If the normative Zamani language is safe-only, lexical recognition MAY remain for compatibility, but semantic validation MUST reject unsupported use.

The lexer MUST NOT treat:

unsafe

as proof that Rust "unsafe" implementation is permitted.

---

33. Quantum Operation Collision Policy

Quantum operations MUST remain open-ended.

The following should normally remain identifiers:

H
X
Y
Z
CNOT
CZ
SWAP
RX
RY
RZ
Toffoli
custom_gate
vendor.operation

The preferred syntax is:

apply operationSpecifier targetList

rather than:

quantumGate
    : H
    | X
    | Y
    | Z
    | CNOT
    | ...

This is essential because adding a new quantum operation MUST NOT require modifying the core keyword registry.

---

34. Quantum Vendor Names

Vendor and device names MUST NOT become core keywords merely because a backend exists.

Examples:

ibm
rigetti
ionq
quantinuum
nvidia
amd
intel

remain ordinary names unless explicitly reserved by a normative language extension.

This preserves:

Program Once
Compile Once
Run Everywhere
Anywhere
Forever

and avoids vendor coupling.

---

35. HDL Keyword Collision Policy

HDL vocabulary must not be allowed to create fixed hardware assumptions.

For example:

wire
register
signal
clock
reset
module
port

may be syntactic concepts where specified.

But they must not imply:

32-bit
64-bit
fixed register count
fixed memory size
fixed FPGA capacity
fixed ASIC geometry

The keyword layer identifies syntax only.

---

36. Hardware Resource Names Are Not Keywords

The following MUST remain semantic/resource data rather than universal keywords:

gpu.compute
quantum.measurement
tensor.compute
memory
interconnect
topology
latency
bandwidth
power
thermal
reliability

A source construct such as:

requires capability("gpu.compute")

does not require "gpu" to be a keyword.

The capability name is semantic data.

---

37. Resource Requirements and Keywords

The validator MUST preserve the distinction:

requires

is language syntax.

But:

qubits
memory
gpu.compute
quantum.measurement
tensor.compute

are semantic/resource vocabulary.

The language must not create one keyword for every possible future resource.

This is critical for unbounded extensibility.

---

38. No Hardware-Derived Keywords

The parser MUST NOT dynamically create keywords based on discovered hardware.

Forbidden behavior:

GPU discovered
    ↓
GPU becomes keyword

Forbidden:

QPU operation discovered
    ↓
new lexer keyword

Forbidden:

FPGA resource discovered
    ↓
new parser token

Hardware discovery belongs downstream.

---

39. No Resource-Derived Keywords

Likewise:

available memory
available cores
available qubits
available devices
available nodes

MUST NOT alter keyword classification.

A program must parse identically on:

tiny device
desktop
HPC cluster
GPU system
FPGA
QPU
cloud
future machine

assuming the same language version and lexical configuration.

---

40. No Capacity-Based Keyword Tables

The keyword validator MUST reject implementations that contain language semantics such as:

MAX_KEYWORDS
MAX_CONTEXTUAL_KEYWORDS
MAX_DIALECT_KEYWORDS
MAX_IDENTIFIERS
MAX_OPERATORS
MAX_TOKENS

when such values impose artificial language limits.

Implementation data structures MAY have representation/resource limits imposed by the implementation environment, but those are not language semantics and MUST NOT change source meaning.

---

41. Identifier Namespace Separation

The validator MUST distinguish at least:

keyword namespace
identifier namespace
operator namespace
literal namespace
punctuation namespace
dialect namespace
macro namespace
attribute namespace

These namespaces must not accidentally produce multiple token identities for the same spelling.

---

42. Keyword and Qualified Names

Qualified names must be parsed structurally.

For:

vendor.operation
module.function
namespace.Type

the lexer should normally produce:

IDENTIFIER
DOT
IDENTIFIER

rather than requiring a keyword for every namespace component.

A component that happens to be a reserved keyword must follow the language's explicit qualified-name policy.

The validator MUST test:

keyword.name
name.keyword
keyword::name
name::keyword

where such forms are syntactically legal.

---

43. Keywords in Member Access

The language must explicitly define whether a reserved keyword can appear after member-access punctuation.

For example:

object.type
object.match
object.measure

If keywords are prohibited in member names, the parser must reject them deterministically.

If contextual member names are allowed, the rule must be explicit.

The implementation MUST NOT accidentally differ between:

object.type

and:

object . type

unless the grammar explicitly specifies such a difference.

---

44. Keyword and Generic Syntax

Generic syntax such as:

Tensor<T, Shape>
Qubit<N>
Memory<T, Size>
History<T, Years>

must not require every generic parameter to become a keyword.

"T", "N", "Shape", "Size", "Years" remain identifiers unless explicitly reserved.

This preserves scalable generic programming.

---

45. Keyword and Dependent Values

Values used in dependent type/resource expressions are semantic expressions, not keywords.

Examples:

Qubit[n]
Tensor<T, shape>
Memory<T, size>

The validator MUST NOT reserve:

n
shape
size

as language keywords.

---

46. Keyword and Pattern Matching

Pattern variables remain identifiers.

For example:

match value {
    Some(x) => ...
}

"x" is an identifier.

The validator must ensure that introducing pattern syntax does not accidentally classify every pattern binding as a keyword.

---

47. Keyword and Function Names

Functions should not require a keyword for every function.

For example:

fn fft(...)
fn matmul(...)
fn custom_operation(...)

only "fn" needs to be reserved.

The function name remains an identifier.

This applies to:

- mathematical operations;
- AI operations;
- quantum operations;
- HDL operations;
- distributed operations;
- networking operations;
- cryptographic functions.

---

48. Keyword and Type Names

The language may reserve primitive type spellings such as:

int
float
bool
str
string
char
void

when required by the current type grammar.

However, the validator MUST ensure that these names do not encode machine representation limits.

For example:

int

must not imply:

32-bit register

unless explicitly defined by the type specification.

---

49. Keyword and Mathematical Vocabulary

Mathematical functions should generally remain identifiers:

sin
cos
tan
exp
log
sqrt
fft
svd
eig
gradient
integral
derivative

Likewise:

matrix
tensor
vector
distribution
polynomial

should become keywords only when they introduce genuine language-level syntax.

This prevents an unbounded mathematical vocabulary from becoming an unbounded reserved-word list.

---

50. Keyword and AI Vocabulary

AI concepts should not automatically become keywords.

Framework names and model names remain identifiers.

For example:

model
training
inference
dataset
agent

may be language constructs if the specification requires them.

But:

transformer
bert
gpt
pytorch
tensorflow
jax

must remain identifiers unless explicitly reserved.

---

51. Keyword and Networking Vocabulary

Networking syntax may reserve stable constructs such as:

endpoint
channel
protocol
stream

if they are actual language-level constructs.

But:

http
https
tcp
udp
quic
mqtt

should generally remain protocol identifiers/data.

The parser must not depend on a finite protocol keyword list.

---

52. Keyword and Security Vocabulary

Security concepts may have language-level constructs such as:

requires
capability
authorize
policy

where specified.

Algorithm names should normally remain identifiers:

sha256
sha3
aes
chacha20
ed25519
rsa

unless the specification explicitly requires dedicated syntax.

---

53. Keyword and Distributed Computing Vocabulary

The language may reserve:

spawn
parallel
actor
channel
replicate
partition

where they represent language constructs.

It must not reserve:

node001
node002
worker001
gpu0
qpu0

as universal language vocabulary.

Physical placement remains downstream.

---

54. Keyword and Temporal/Sankofa Vocabulary

Existing vocabulary such as:

mts
zamani
sasa
remember
recall
wisdom

may remain reserved where their syntax is stable.

They MUST NOT encode:

MAX_TIMELINES
MAX_HISTORY
MAX_BRANCHES
MAX_TIMESTAMP
MAX_MEMORY

Keyword classification remains independent of temporal-resource capacity.

---

55. Historical and Extended Vocabulary

"Zamani-Grammar.md" contains broad historical and extended vocabulary.

Presence in that document does not reserve a spelling.

A historical spelling becomes a production keyword only after:

proposal
  ↓
semantic design
  ↓
specification
  ↓
AST contract
  ↓
grammar
  ↓
lexer
  ↓
parser
  ↓
semantic implementation
  ↓
IR integration
  ↓
tests
  ↓
stable status

This prevents accidental promotion of historical concepts into the core language.

---

56. Keyword Status Lifecycle

Every keyword MUST have one status:

PROPOSED
EXPERIMENTAL
STABLE
DEPRECATED
REMOVED
HISTORICAL
CONTEXTUAL
DIALECT
COMPATIBILITY

The validator MUST reject contradictory statuses.

For example:

STABLE
REMOVED

cannot simultaneously describe the same spelling in the same language version.

---

57. Deprecated Keywords

A deprecated keyword may remain lexically recognized for compatibility.

It MUST:

1. retain deterministic tokenization;
2. produce the specified diagnostic;
3. identify its replacement where one exists;
4. preserve source spans;
5. not create a second token identity;
6. not silently change meaning.

Example:

legacy_keyword
    ↓
DEPRECATED_LEGACY_TOKEN
    ↓
diagnostic

or, preferably:

legacy_keyword
    ↓
canonical token
    ↓
deprecation metadata

depending on the canonical lexical architecture.

---

58. Removed Keywords

A removed keyword MUST NOT silently become an identifier if that would reinterpret old source differently.

The compatibility policy must define whether the source:

old_keyword

produces:

removed-keyword diagnostic

rather than silently becoming:

IDENTIFIER("old_keyword")

This prevents accidental semantic reinterpretation.

---

59. Future-Reserved Words

Zamani MAY maintain a future-reserved set.

However, future-reserved words must be explicitly distinguished from currently reserved keywords.

A future-reserved spelling:

future_keyword

must either:

- remain a valid identifier with documented reservation warning behavior; or
- be lexically reserved by specification.

The compiler MUST NOT reserve arbitrary words merely as a precaution.

---

60. Feature-Gated Keywords

Feature-gated vocabulary must be deterministic.

A keyword activated by a feature must have:

language version
feature identifier
dialect identifier if applicable
canonical token
activation rule
compatibility behavior

The feature gate MUST be explicit.

It MUST NOT depend on:

hardware
runtime
installed backend
filesystem
network
resource availability

---

61. Keyword Collision Across Feature Gates

Two feature configurations MUST NOT cause the same source spelling to map simultaneously to incompatible canonical tokens without explicit configuration.

Bad:

feature A:
foo → TOKEN_A

feature B:
foo → TOKEN_B

with both active.

Valid:

foo → TOKEN_A

under feature A, with feature B inactive.

Or:

foo → same canonical TOKEN

with semantic extension controlled downstream.

---

62. Dialect Merge Algorithm

When multiple dialects are active, keyword validation MUST construct a combined lexical registry.

Conceptually:

core keywords
+
dialect A keywords
+
dialect B keywords
+
explicit compatibility aliases
=
active lexical registry

Then validate:

duplicate spelling
duplicate token identity
conflicting category
conflicting case policy
conflicting contextual policy
conflicting status
conflicting semantics

The merge MUST be deterministic.

It MUST NOT depend on iteration order.

---

63. Deterministic Conflict Resolution

A collision MUST NOT be resolved by:

first declaration wins
last declaration wins
hash-map iteration
filesystem order
package installation order
backend preference
hardware capability
runtime state
random choice

A collision must instead produce a deterministic diagnostic.

---

64. Diagnostic Requirements

Keyword-collision diagnostics MUST include:

diagnostic code
severity
source spelling
canonical owner
conflicting owner
language version
feature configuration
dialect
source span when source is available
recommended resolution
compatibility information

Recommended diagnostic codes include:

ZKW001 KEYWORD_DUPLICATE_SPELLING
ZKW002 KEYWORD_DUPLICATE_TOKEN
ZKW003 KEYWORD_IDENTIFIER_COLLISION
ZKW004 KEYWORD_OPERATOR_COLLISION
ZKW005 KEYWORD_PUNCTUATION_COLLISION
ZKW006 KEYWORD_LITERAL_COLLISION
ZKW007 KEYWORD_DIALECT_COLLISION
ZKW008 KEYWORD_CONTEXT_COLLISION
ZKW009 KEYWORD_CASE_COLLISION
ZKW010 KEYWORD_UNICODE_CONFUSABLE
ZKW011 KEYWORD_LEGACY_ALIAS_EMITTED
ZKW012 KEYWORD_TOKEN_DIVERGENCE
ZKW013 KEYWORD_PARSER_DIVERGENCE
ZKW014 KEYWORD_UNDECLARED_RESERVATION
ZKW015 KEYWORD_STATUS_CONFLICT
ZKW016 KEYWORD_VERSION_CONFLICT
ZKW017 KEYWORD_FEATURE_CONFLICT
ZKW018 KEYWORD_MACRO_COLLISION
ZKW019 KEYWORD_INTEROP_COLLISION
ZKW020 KEYWORD_NONDETERMINISTIC_RESOLUTION

Diagnostic numbering is part of the validation contract and MUST remain stable once published.

---

65. Source Diagnostics

For source code:

foo(...)

where "foo" collides with a keyword, diagnostics must identify the exact source span.

The diagnostic MUST NOT report an unrelated downstream span when the lexical collision can be identified directly.

---

66. ANTLR/Rust Conformance

The ANTLR and Rust lexical implementations MUST agree.

For every canonical keyword:

source

must produce equivalent:

ANTLR token
Rust token

The comparison must be performed by canonical token identity, not merely token spelling.

Example:

if

ANTLR → IF
Rust  → If

is valid if:

IF ↔ If

is the declared canonical mapping.

But:

ANTLR → IF
Rust  → Identifier

is a conformance failure.

---

67. Token Name Versus Token Identity

Different naming conventions do not automatically constitute collisions.

For example:

ANTLR: EQUAL_EQUAL
Rust:  Equals

may be valid.

The registry must explicitly map:

EQUAL_EQUAL ↔ Equals

to one canonical lexical identity.

By contrast:

BitAnd
Ampersand

cannot both remain independently emittable identities for:

&

---

68. Parser Conformance

Every canonical keyword token must have an explicit parser consumer or a documented reason why it is lexical-only.

The validator must detect:

keyword exists
but parser has no valid consumption path

unless the token is intentionally reserved for a future or compatibility state.

Likewise:

parser expects obsolete token

is a validation error.

---

69. AST Conformance

A keyword's presence does not automatically require a unique AST node.

For example:

apply

may map to a generic operation/declaration node.

The AST contract must be determined by semantic structure.

The validator must verify:

keyword
→ parser construct
→ AST contract

where applicable.

It must not require unnecessary AST proliferation.

---

70. Semantic Conformance

Keyword validation stops at lexical identity, but the validator must verify that every production keyword has a documented semantic owner.

For example:

measure

may map through:

parser
→ AST operation
→ semantic quantum operation
→ quantum::ir

The keyword validator does not implement measurement.

It only ensures the keyword is not lexically duplicated or ambiguously classified.

---

71. Quantum IR Boundary

No keyword collision rule may create a second quantum frontend IR.

The canonical path remains:

source
→ lexer
→ parser
→ domain-neutral AST
→ semantic analysis
→ quantum::ir

The keyword validator must explicitly reject architecture that makes:

keyword
→ frontend QuantumGate enum
→ separate quantum IR

the canonical semantic path.

---

72. No Fixed Quantum Keyword Enumeration

The validator MUST NOT require one keyword per gate.

This is prohibited as a scalability architecture:

H
X
Y
Z
CNOT
CZ
SWAP
...

as the complete quantum operation vocabulary.

Operations must remain data-driven wherever possible.

This allows:

apply H ...
apply custom_gate ...
apply vendor.operation ...
apply future.operation ...

without modifying the keyword registry.

---

73. Hardware Independence

Keyword classification MUST be identical regardless of target:

CPU
GPU
FPGA
ASIC
QPU
accelerator
embedded device
HPC
cluster
distributed system
cloud
future target

A hardware target MUST NOT add, remove, or redefine core keywords during parsing.

---

74. Resource Independence

The same source must receive the same lexical interpretation regardless of available:

memory
CPUs
cores
threads
GPUs
FPGAs
QPUs
nodes
devices
accelerators
network links
storage

A resource shortage is not a keyword collision.

It is a later resource/target diagnostic.

---

75. POCO-REAF Requirement

Keyword classification is part of the POCO-REAF guarantee.

Therefore:

same source
+
same language version
+
same explicit dialect/feature configuration

must produce:

same canonical token stream

regardless of execution target.

---

76. No Environment-Dependent Keywords

The lexical implementation MUST NOT inspect:

environment variables
current working directory
installed packages
network services
hardware
runtime state
wall-clock time
randomness

to decide whether a word is a keyword.

---

77. No Filesystem-Dependent Keywords

The compiler MUST NOT search the filesystem and dynamically add keywords.

For example:

package installed
→ package name becomes keyword

is prohibited.

Packages provide semantic symbols.

They do not redefine the core lexical language.

---

78. No Network-Dependent Keywords

The compiler MUST NOT query a registry or network service to determine lexical classification.

This is essential for:

- reproducible builds;
- offline compilation;
- deterministic diagnostics;
- security;
- POCO-REAF.

---

79. No Runtime-Dependent Keywords

The runtime cannot alter the source language.

The parser MUST complete before runtime execution.

Runtime capabilities therefore cannot affect keyword recognition.

---

80. Scalability

The keyword system must scale with:

small programs
large programs
large modules
large dependency graphs
large ASTs
large distributed systems
large quantum programs
large HDL programs
large AI programs
large datasets
future domains

without language-level artificial keyword-count limits.

The validator MUST NOT introduce constants such as:

MAX_KEYWORDS
MAX_CONTEXTUAL_KEYWORDS
MAX_DIALECT_KEYWORDS
MAX_IDENTIFIER_LENGTH
MAX_NAMESPACE_DEPTH

as language semantics.

Implementation resource exhaustion must be reported separately from language invalidity.

---

81. Complexity

Keyword lookup SHOULD be approximately:

O(1)

average-case or otherwise bounded by the chosen deterministic lexical data structure.

The validator MUST prioritize:

- deterministic behavior;
- memory safety;
- reproducibility;
- clear diagnostics;
- scalability.

A trie, generated table, deterministic map, or equivalent structure may be used.

The choice is an implementation concern and MUST NOT change language semantics.

---

82. Safe Rust Requirement

All Zamani-owned Rust keyword validation and lexical infrastructure MUST compile under:

Rust 1.97
Rust 1.97.1
Rust 2021

and MUST use safe Rust.

The implementation MUST NOT require:

unsafe

or:

unsafe fn

or unsafe operations.

Production validation must use safe abstractions for:

- token registries;
- collision maps;
- deterministic ordering;
- diagnostics;
- source spans;
- generated vocabulary;
- parser integration.

---

83. Deterministic Data Structures

Where output or diagnostics depend on iteration order, implementations SHOULD use deterministic structures such as:

BTreeMap
BTreeSet

or explicitly sorted collections.

Hash-map iteration order MUST NOT determine:

which keyword wins
which diagnostic appears first
which token identity is selected

---

84. Collision Detection Algorithm

The production validator SHALL conceptually perform:

1. Load normative keyword registry.
2. Load canonical token registry.
3. Load operator registry.
4. Load punctuation registry.
5. Load literal registry.
6. Load identifier rules.
7. Load active dialects.
8. Load active compatibility aliases.
9. Load active feature configuration.
10. Normalize registry representation according to the lexical specification.
11. Group entries by source spelling.
12. Detect duplicate canonical identities.
13. Detect duplicate emittable identities.
14. Detect keyword/identifier collisions.
15. Detect keyword/operator collisions.
16. Detect keyword/punctuation collisions.
17. Detect keyword/literal collisions.
18. Detect contextual conflicts.
19. Detect dialect conflicts.
20. Detect version conflicts.
21. Detect Unicode/confusable conflicts.
22. Detect ANTLR/Rust divergence.
23. Detect parser references to obsolete tokens.
24. Detect aliases that remain emittable.
25. Verify deterministic resolution.
26. Emit stable diagnostics.
27. Verify conformance fixtures.
28. Verify scalability invariants.
29. Verify no hard-coded language capacities.
30. return success only if all mandatory checks pass.

---

85. Collision Severity

The validator MUST classify findings.

Error

Examples:

two active keywords share a spelling
keyword and identifier are indistinguishable when the grammar requires distinction
ANTLR and Rust emit different canonical tokens
legacy duplicate token remains emittable
dialects define incompatible active meanings
same spelling has two canonical identities

Warning

Examples:

deprecated keyword used
future-reserved identifier used
legacy alias referenced only in implementation source

Informational

Examples:

contextual keyword recognized
compatibility alias encountered

A production build MUST fail on mandatory errors.

---

86. Positive Tests

For every stable keyword:

keyword

must be accepted where syntactically valid.

Tests must verify:

canonical token
source span
parser path
AST mapping where applicable

---

87. Identifier Collision Tests

For every keyword:

keywordx
xkeyword
keyword_foo
foo_keyword
keyword2

must be tested according to identifier rules.

The purpose is to ensure keyword matching does not split valid identifiers.

---

88. Case Tests

For each keyword:

keyword
KEYWORD
Keyword
kEyWoRd

must be tested where useful.

The expected result must follow the canonical case policy.

---

89. Unicode Tests

Tests must include:

- canonical ASCII spelling;
- Unicode confusable spelling;
- normalized/non-normalized forms;
- Unicode identifiers adjacent to keywords;
- Unicode identifiers beginning/ending near keyword boundaries.

The expected result must be deterministic.

---

90. Operator Collision Tests

Every keyword-adjacent operator must be tested without whitespace:

if->x
if=>x
measure->q
apply->q
async->x
return->x

and with whitespace:

if -> x
measure -> q
apply -> q

The token stream must remain consistent with the lexical specification.

---

91. Punctuation Tests

Test:

keyword(...)
keyword[...]
keyword{...}
keyword<...>
keyword::name
keyword.name
@keyword
#keyword

where each form is syntactically meaningful.

The validator must distinguish genuine collisions from intentional syntax.

---

92. Literal Tests

Test keywords adjacent to:

integer literals
floating-point literals
string literals
character literals
boolean literals
null literals
quantum literals

Examples:

true_value
null_value
measure1
apply2

must remain deterministic.

---

93. Quantum Collision Tests

At minimum:

apply H to q
apply X to q
apply CNOT to q0, q1
apply custom_gate to q
apply vendor.operation to q

must demonstrate that gate/operation names are not dependent on keyword enumeration.

Also test:

|0⟩
|1⟩
|+⟩
|-⟩
|ψ⟩

against ordinary:

a | b

to ensure ordinary "PIPE" and quantum literal handling do not accidentally create duplicate lexical ownership.

---

94. HDL Collision Tests

Test ordinary identifiers adjacent to HDL vocabulary:

module_name
register2
signal_data
clock_source
reset_value
wire_data

The validator must not split identifiers merely because they begin with an HDL term.

---

95. AI Collision Tests

Test:

model
model_name
training
training_data
agent
agent2
inference
inference_result

according to the selected status of each spelling.

---

96. Distributed Collision Tests

Test:

node
node0
node_name
channel
channel_data
parallel
parallelism
spawn
spawned

to ensure reserved words do not accidentally consume identifier prefixes.

---

97. Dialect Tests

For every dialect:

core only
core + dialect A
core + dialect B
core + A + B

must be validated.

A collision in:

core + A + B

must be reported deterministically.

---

98. Macro Tests

Macro tests must cover:

keyword generated by macro
identifier generated by macro
keyword-looking identifier
operator generated by macro
nested macro expansion
hygienic identifier generation

The final token identity must be deterministic.

---

99. Compatibility Tests

Every changed keyword must have:

previous version fixture
current version fixture
migration expectation
diagnostic expectation

A keyword promotion must not silently break existing source without a documented compatibility policy.

---

100. Versioned Keyword Sets

The validator must conceptually support:

KeywordSet(language_version, feature_configuration, dialect_configuration)

This prevents historical versions from being evaluated against the wrong vocabulary.

The active set is determined explicitly.

---

101. Grammar Version Integration

Keyword additions, removals, or semantic reservation changes MUST integrate with:

grammar/compatibility/versions.md
grammar/compatibility/migrations.md
grammar/compatibility/deprecated.md

and the relevant language-version specification.

---

102. "grammar.md" Integration

"grammar/grammar.md" must be able to report keyword status as:

SPECIFIED
LEXER_IMPLEMENTED
PARSER_IMPLEMENTED
AST_IMPLEMENTED
SEMANTIC_IMPLEMENTED
IR_IMPLEMENTED
TESTED
STABLE
EXPERIMENTAL
DEPRECATED

The keyword-collision validator should consume the canonical registry rather than treating "grammar.md" as the source of truth.

---

103. "Zamani-Grammar.md" Integration

"Zamani-Grammar.md" remains historical/extended design material.

The validator MUST NOT automatically ingest every word in that document as a keyword.

Only vocabulary promoted through the normative feature lifecycle becomes lexical authority.

---

104. "Zamani.g4" Integration

"grammar/Zamani.g4" remains the root grammar.

It MUST reference canonical keyword/token definitions.

It MUST NOT introduce a competing keyword vocabulary.

A root-level literal such as:

'if'

must be reconciled with the canonical keyword registry rather than independently defining another lexical identity.

---

105. "grammar/lexer/keywords.md" Integration

This document is subordinate to the normative keyword inventory defined there.

"keyword-collisions.md" validates that inventory.

It MUST NOT silently redefine the list.

Where a conflict is discovered:

keywords.md
vs
keywords.g4

the validator reports the conflict; the authoritative keyword specification remains the source for the intended language decision.

---

106. "grammar/lexer/tokens.md" Integration

Every keyword token must exist in the canonical token vocabulary.

A keyword that has no canonical token identity is an incomplete lexical feature.

A token that exists without a declared lexical owner must be audited.

---

107. "grammar/lexer/operators.md" Integration

Operator spellings are separate from keywords.

The validator cross-checks both registries to ensure:

keyword spelling != operator spelling

unless an explicitly defined lexical mechanism exists.

---

108. "grammar/validation/duplicate-tokens.md" Integration

"duplicate-tokens.md" remains the canonical contract for duplicate token identity.

This document extends that contract specifically to keyword collisions.

The two documents must remain complementary:

duplicate-tokens.md
    ↓
token identity

keyword-collisions.md
    ↓
keyword ownership and collision classes

Neither should duplicate the other's entire responsibility.

---

109. "grammar/validation/ambiguity.md" Integration

"ambiguity.md" owns general syntactic ambiguity.

This document owns lexical keyword collision.

Relationship:

keyword collision
      ↓
lexical validation
      ↓
canonical token stream
      ↓
ambiguity validation
      ↓
canonical parse

A keyword collision MUST be fixed before parser ambiguity is evaluated.

---

110. "grammar/validation/hard-coding.md" Integration

Keyword validation must cooperate with hard-coding validation.

The lexical system MUST NOT introduce:

MAX_KEYWORDS
MAX_DIALECTS
MAX_TOKENS
MAX_IDENTIFIER_DEPTH
MAX_NAMESPACE_DEPTH

as language restrictions.

---

111. "grammar/validation/scalability.md" Integration

Scalability validation must verify that adding:

- more keywords;
- more dialects;
- more domains;
- more identifiers;
- more operations;
- more quantum operations;
- more hardware capabilities;

does not require artificial language limits.

---

112. "grammar/validation/grammar-validator.md" Integration

The grammar validator orchestrates this specification.

It should invoke keyword validation before parser-level ambiguity validation.

Recommended sequence:

lexical inventory validation
        ↓
keyword collision validation
        ↓
duplicate-token validation
        ↓
operator validation
        ↓
grammar validation
        ↓
ambiguity validation
        ↓
AST coverage
        ↓
semantic coverage
        ↓
IR coverage

---

113. Lexer/Parser Integration Contract

The canonical contract is:

source spelling
       ↓
one lexical owner
       ↓
one canonical token
       ↓
parser consumes canonical token
       ↓
AST structure

The parser MUST NOT contain compatibility alternatives such as:

Ampersand | BitAnd
QuestionMark | Question
Pipe | BitOr
ThinArrow | Arrow

for the same source spelling.

---

114. Required Parser Cleanup

The following patterns MUST be removed from production parser logic:

Question | QuestionMark
BitAnd | Ampersand
BitOr | Pipe
Arrow | ThinArrow

when they represent the same source spelling.

The parser should consume only:

QuestionMark
Ampersand
Pipe
ThinArrow

respectively.

---

115. Required Lexer Cleanup

The Rust lexer MUST NOT emit:

Question
BitAnd
BitOr
Arrow

for the canonical source spellings where the repository has already established:

QuestionMark
Ampersand
Pipe
ThinArrow

Legacy names may remain temporarily in migration documentation but not in production emission.

---

116. Generated Lexer Cleanup

Generated ANTLR lexer output must not contain two active token rules for the same source spelling.

If an ANTLR alias is required internally, it must resolve to the same canonical lexical identity without creating a second semantic token.

---

117. Source Span Preservation

Canonicalization MUST NOT lose source spans.

For:

&

whether historically represented as:

BitAnd

or:

Ampersand

the canonical token must preserve the same:

start offset
end offset
line
column
source file

as required by the source-map contract.

---

118. Diagnostics Must Be Stable

The same invalid source under the same configuration must produce deterministic diagnostics.

The diagnostic must not change because:

hash-map ordering changed
dialect files were discovered in another order
hardware changed
CPU count changed
GPU count changed
QPU changed
machine size changed

---

119. Incremental Lexing

Incremental lexing must preserve the same canonical classification as full lexing.

Changing an unrelated source region must not cause a keyword in another region to receive a different token identity.

Cache invalidation must account for:

language version
feature configuration
dialect configuration
keyword registry version

where those affect lexical behavior.

---

120. Formatter Integration

The formatter must preserve keyword identity.

Formatting:

if(x){...}

into:

if (x) {
    ...
}

must not change:

IF

into an identifier.

Formatting must preserve AST/token semantics.

---

121. IDE/LSP Integration

IDE syntax highlighting may recover incomplete source, but it must consume the same canonical keyword registry.

Autocomplete may suggest identifiers that happen to resemble keywords only according to the language's reservation rules.

IDE-specific recovery must not change compiler semantics.

---

122. Serialization

If tokens are serialized for:

- caching;
- diagnostics;
- IDE communication;
- incremental compilation;
- debugging;

the serialized token identity must use the canonical token ID.

Legacy token names must not become a second wire-level identity.

---

123. Reproducible Builds

Keyword classification must be reproducible.

A build performed twice with identical:

source
language version
feature configuration
dialect configuration
compiler version
grammar version

must produce identical lexical results.

---

124. Security

Keyword collisions are security-sensitive when they can cause source text to be interpreted differently from what a reviewer expects.

The validator must therefore protect against:

- Unicode confusables;
- invisible characters where prohibited;
- case confusion;
- keyword-prefix confusion;
- dialect shadowing;
- generated-token confusion;
- macro token confusion;
- incompatible lexer implementations.

---

125. Invisible Character Policy

Characters that may alter visual interpretation without changing apparent source must be handled according to the Unicode lexical specification.

The validator should test source containing:

zero-width characters
directional controls
combining characters
confusable characters

where applicable.

They must never silently turn an identifier into a keyword or vice versa.

---

126. No Semantic Keyword Matching

Semantic names must not dynamically become keywords.

For example:

capability("quantum.measurement")

must not cause:

quantum
measurement

to become dynamically reserved words.

The string is semantic data.

---

127. No Library-Driven Keyword Matching

Importing a library must not add keywords.

Bad:

import quantum_library

causes:

new quantum_library keywords

Good:

import quantum_library

adds semantic symbols while preserving the language's lexical rules.

---

128. No Vendor-Driven Keyword Matching

Installing a backend must not add:

vendor_keyword

to the core lexer.

Vendor APIs should be represented by:

identifiers
qualified names
capabilities
dialects
interoperability contracts

where appropriate.

---

129. Keyword Registry Extensibility

New domains must be able to add semantics without requiring a core keyword whenever possible.

The preferred pattern is:

stable syntactic keyword
+
identifier-based operation/name
+
typed semantic data
+
capability/resource metadata

rather than:

new domain
→ dozens of new reserved words

This is especially important for:

- quantum;
- AI;
- HDL;
- hardware;
- networking;
- distributed systems;
- future computation models.

---

130. Production Test Matrix

The validator MUST support at least:

Area| Positive| Negative| Boundary| Scalability| Determinism| Compatibility
Core keywords| ✓| ✓| ✓| ✓| ✓| ✓
Identifiers| ✓| ✓| ✓| ✓| ✓| ✓
Operators| ✓| ✓| ✓| ✓| ✓| ✓
Unicode| ✓| ✓| ✓| ✓| ✓| ✓
Literals| ✓| ✓| ✓| ✓| ✓| ✓
Quantum| ✓| ✓| ✓| ✓| ✓| ✓
HDL| ✓| ✓| ✓| ✓| ✓| ✓
Hardware| ✓| ✓| ✓| ✓| ✓| ✓
Resources| ✓| ✓| ✓| ✓| ✓| ✓
AI| ✓| ✓| ✓| ✓| ✓| ✓
Distributed| ✓| ✓| ✓| ✓| ✓| ✓
Macros| ✓| ✓| ✓| ✓| ✓| ✓
Dialects| ✓| ✓| ✓| ✓| ✓| ✓
Interoperability| ✓| ✓| ✓| ✓| ✓| ✓

---

131. Required Collision Fixtures

The production suite MUST contain fixtures for the known historical collisions:

?
&
|
->

and verify:

?       → QuestionMark
&       → Ampersand
|       → Pipe
->      → ThinArrow

It MUST additionally verify that the legacy identities are never emitted.

---

132. Required Prefix Fixtures

The production suite must include:

?
?.
??

!
!=

&
&&
&=

|
||
|=

-
--
-=
->

.
..
..=

:
::

<
<=
<<

>
>=
>>

---

133. Required Keyword Fixtures

For each stable keyword:

keyword
keyword_suffix
prefix_keyword
keyword_identifier
keyword2

must be checked according to identifier rules.

---

134. Required Contextual Fixtures

For each contextual keyword:

keyword in keyword position
keyword in identifier position
keyword after member access
keyword inside qualified name
keyword inside macro
keyword inside dialect

must be tested.

---

135. Differential ANTLR/Rust Testing

For every fixture:

ANTLR lexer result

must be compared with:

Rust lexer result

using canonical token identity.

A difference is a production failure unless explicitly documented as a temporary implementation status.

---

136. Property-Based Testing

The validator SHOULD generate:

- random valid identifiers;
- identifiers derived from keywords;
- keyword prefixes;
- keyword suffixes;
- operator-adjacent identifiers;
- Unicode identifiers;
- qualified names;
- generic names;
- macro-generated names.

The core property is:

lex(source) is deterministic

and:

same source/configuration
→ same token identities

---

137. Fuzzing

Fuzzing SHOULD cover:

keyword boundaries
operator boundaries
Unicode
dialects
macro expansion
qualified names
generic syntax
quantum literals
HDL syntax
attributes
nested constructs

Failures must distinguish:

wrong token
wrong keyword
wrong identifier classification
panic
timeout
resource exhaustion
diagnostic instability
ANTLR/Rust divergence

---

138. Resource Exhaustion Is Not a Keyword Limit

A validator may protect itself from actual implementation resource exhaustion.

For example, an input can be rejected because it exceeds the implementation's safe processing capacity.

That MUST NOT be reported as:

language keyword limit exceeded

unless the language specification itself defines such a limit.

The implementation limit and language semantics must remain separate.

---

139. No Artificial Nesting Limits

Keyword validation MUST NOT introduce language limits on:

nested namespaces
nested modules
nested generics
nested expressions
nested macros
nested dialects
qualified-name depth

Any implementation safety limit must be explicitly classified as an implementation/resource safeguard, not language syntax.

---

140. Domain Integration Matrix

Every keyword must identify its domain owner where applicable:

core
expressions
types
statements
declarations
functions
modules
effects
memory
concurrency
classical
quantum
hybrid
hdl
hardware
resources
distributed
ai
data
networking
security
compile
execution
interoperability
dialects
macros
metaprogramming

A domain MUST consume the canonical token.

It MUST NOT create a duplicate token for an existing spelling.

---

141. Domain-Neutral AST Requirement

Keywords must map into the domain-neutral frontend AST where the construct is universal.

For example:

apply

should not require a parser-level vendor-specific AST representation.

Semantic interpretation occurs later.

---

142. Semantic Ownership

The keyword layer owns:

classification

The parser owns:

structure

The AST owns:

language-independent structure

Semantic analysis owns:

meaning

IR owns:

canonical executable representation

Backends own:

target realization

No keyword implementation may bypass these boundaries.

---

143. Quantum Integration

The keyword system must support:

quantum
circuit
qubit
apply
measure
reset
barrier
control
adjoint
inverse
observe

where these are stable language constructs.

Operation names remain open-ended.

The semantic pipeline remains:

keyword/token
   ↓
parser
   ↓
domain-neutral AST
   ↓
quantum semantic analysis
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

---

144. HDL Integration

HDL keywords must map into HDL syntax without encoding target-specific hardware limits.

For example:

module
port
signal
register
clock
reset

may be syntax.

But:

32-bit
64-bit
8-register
24GB
64GB

must never become implicit lexical constraints.

---

145. Hardware Integration

Hardware keywords describe intent.

They must not select a physical target.

For example:

requires
capability
resource
topology
latency
power
thermal
reliability

remain abstract language concepts.

---

146. Resource Integration

Resource names remain data wherever possible.

The validator must preserve:

requirement
capability
constraint
preference
hint
realization

as distinct semantic categories.

Keyword classification must not collapse these distinctions.

---

147. Distributed Integration

Distributed vocabulary must remain target-independent.

No keyword may imply:

8 nodes
16 workers
32 threads
64 GPUs

or any other fixed capacity.

---

148. AI Integration

AI syntax must remain extensible.

The keyword system must not enumerate:

models
frameworks
architectures
optimizers
hardware vendors

as core keywords unless the language specification genuinely requires syntactic reservation.

---

149. Future-Domain Integration

A future computing paradigm must be able to enter Zamani without modifying unrelated lexical categories.

The preferred extension path is:

new domain
    ↓
semantic capability
    ↓
typed data
    ↓
existing generic syntax
    ↓
dialect/extension only where necessary

rather than:

new domain
    ↓
large new keyword list
    ↓
core parser changes

---

150. Production Acceptance Criteria

"grammar/validation/keyword-collisions.md" is considered satisfied when all of the following are true:

- [ ] one canonical keyword authority exists;
- [ ] one canonical token identity exists per spelling;
- [ ] "Question"/"QuestionMark" is resolved;
- [ ] "BitAnd"/"Ampersand" is resolved;
- [ ] "BitOr"/"Pipe" is resolved;
- [ ] "Arrow"/"ThinArrow" is resolved;
- [ ] obsolete duplicate identities are not emitted;
- [ ] keyword/identifier boundaries are deterministic;
- [ ] keyword/operator collisions are deterministic;
- [ ] keyword/punctuation collisions are deterministic;
- [ ] keyword/literal collisions are deterministic;
- [ ] Unicode behavior is deterministic;
- [ ] confusable handling is specified;
- [ ] contextual keywords are explicit;
- [ ] dialect collisions are explicit;
- [ ] feature-gated keywords are explicit;
- [ ] compatibility aliases are non-emitting;
- [ ] ANTLR and Rust token identities conform;
- [ ] parser consumes canonical tokens;
- [ ] AST ownership is documented;
- [ ] semantic ownership is documented;
- [ ] "quantum::ir" remains canonical;
- [ ] no quantum gate keyword enumeration is required;
- [ ] hardware does not affect lexical classification;
- [ ] resource availability does not affect lexical classification;
- [ ] filesystem state does not affect lexical classification;
- [ ] network state does not affect lexical classification;
- [ ] runtime state does not affect lexical classification;
- [ ] randomness does not affect lexical classification;
- [ ] wall-clock time does not affect lexical classification;
- [ ] no universal "MAX_*" language limits are introduced;
- [ ] Rust 1.97/1.97.1 is supported;
- [ ] Rust 2021 is supported;
- [ ] production Rust uses no "unsafe";
- [ ] diagnostics are deterministic;
- [ ] positive tests exist;
- [ ] negative tests exist;
- [ ] boundary tests exist;
- [ ] scalability tests exist;
- [ ] compatibility tests exist;
- [ ] ANTLR/Rust differential tests exist;
- [ ] fuzz/property tests cover collision boundaries;
- [ ] formatter/LSP behavior preserves canonical lexical identity.

---

151. Required Repository Integration Changes

This file is complete as a specification when written.

The following implementation integrations are required elsewhere, but they do not require this document to be rewritten when those files change.

"grammar/lexer/keywords.md"

Must remain the keyword inventory authority.

"grammar/lexer/keywords.g4"

Must implement the inventory without duplicate emitted spellings.

"grammar/lexer/tokens.g4"

Must provide one canonical token identity.

"grammar/lexer/operators.g4"

Must own operators rather than keyword rules.

"grammar/antlr/ZamaniLexer.g4"

Must compose the canonical lexical components.

"grammar/antlr/ZamaniParser.g4"

Must consume canonical token identities.

"grammar/Zamani.g4"

Must remain the root composition grammar and must not create a second lexical authority.

"src/lexer.rs"

Must emit only canonical token identities.

In particular:

?  → QuestionMark
&  → Ampersand
|  → Pipe
-> → ThinArrow

and not the obsolete duplicate variants.

"src/parser.rs"

Must consume the canonical identities and remove duplicate alternatives for the same source spelling.

"src/frontend/ast/"

Must remain domain-neutral.

No keyword collision should force vendor-specific or hardware-specific AST nodes.

"grammar/validation/duplicate-tokens.md"

Continues to own generic duplicate-token identity.

"grammar/validation/ambiguity.md"

Continues to own parser ambiguity.

"grammar/validation/hard-coding.md"

Continues to own artificial-capacity detection.

"grammar/validation/scalability.md"

Continues to own scalability validation.

"grammar/compatibility/"

Owns version and migration policy.

"grammar/tests/"

Owns executable lexical/conformance fixtures.

---

152. File Completion Contract

This file is complete when it specifies all of the following independently:

Purpose
Scope
Authority
Ownership
Non-ownership
Keyword identity
Token identity
Identifier boundaries
Operator boundaries
Literal boundaries
Punctuation boundaries
Unicode policy
Contextual keyword policy
Dialect policy
Feature-gate policy
Compatibility policy
Macro policy
Interoperability policy
Quantum policy
HDL policy
Hardware policy
Resource policy
AST integration
Semantic integration
IR integration
ANTLR integration
Rust integration
Diagnostics
Testing
Scalability
Security
Determinism
Versioning
Production acceptance

No later modification of:

lexer.rs
parser.rs
Zamani.g4
AST
semantic analyzer
quantum::ir
backend
runtime

should require changing the fundamental rules in this document merely because those implementations evolve.

If an implementation exposes a conflict with this document, the implementation must be corrected or the language specification must deliberately change through the normal versioned specification process.

---

153. Definition of Done

The keyword-collision system is production-ready only when the following statement is true:

«For every supported Zamani language version, explicit feature configuration, and explicit dialect configuration, every source spelling has one deterministic lexical interpretation.»

For every active keyword:

keyword spelling
    ↓
one canonical token
    ↓
one deterministic parser interpretation

For every ordinary identifier:

identifier spelling
    ↓
IDENTIFIER

unless the language specification explicitly reserves that spelling.

For every compatibility alias:

legacy identity
    ↓
canonical identity

and the legacy identity is never independently emitted.

For every domain:

classical
quantum
HDL
hybrid
AI
distributed
networking
security
data
hardware
future domains

the lexical foundation remains one Zamani language.

For every target:

atom/small device
embedded
CPU
multicore
GPU
FPGA
ASIC
QPU
accelerator
HPC
cluster
distributed
cloud
future hardware

keyword classification remains independent of target resources.

No artificial language-level limits are introduced.

No:

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
MAX_KEYWORDS

may define the language's semantic capacity.

Program values remain program values.

Resource availability remains a resource concern.

Capability availability remains a capability concern.

Hardware realization remains a backend concern.

Quantum realization remains downstream of:

quantum::ir

and:

routing
scheduling
resilience/QEC
ZQN
HAL

remain outside lexical ownership.

The production Rust implementation remains compatible with:

Rust 1.97
Rust 1.97.1
Rust 2021

and uses:

safe Rust only

with no requirement for "unsafe".

---

154. Final Canonical Rule

The complete rule for Zamani keyword collisions is:

SOURCE SPELLING
      │
      ▼
LEXICAL OWNERSHIP
      │
      ├── keyword
      ├── identifier
      ├── operator
      ├── punctuation
      ├── literal
      └── explicit contextual construct
      │
      ▼
ONE CANONICAL TOKEN IDENTITY
      │
      ▼
DETERMINISTIC PARSER
      │
      ▼
DOMAIN-NEUTRAL AST
      │
      ▼
SEMANTIC ANALYSIS
      │
      ├── type
      ├── effect
      ├── resource
      ├── capability
      ├── quantum
      ├── classical
      ├── HDL
      ├── hardware
      └── other domains
      │
      ▼
CANONICAL IR
      │
      ├── classical
      ├── quantum::ir
      └── HDL/hardware intent
      │
      ▼
OPTIMIZATION / LOWERING
      │
      ├── routing
      ├── scheduling
      ├── resilience/QEC
      └── ZQN
      │
      ▼
HAL
      │
      ▼
TARGET REALIZATION

The governing invariant is:

«Syntax determines structure. Semantics determines meaning. Resources determine feasibility. Compilation determines realization. Runtime determines execution. Hardware never determines what a keyword means.»

This keyword-collision contract therefore directly protects:

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF).