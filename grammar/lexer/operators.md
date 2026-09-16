Zamani Operator Specification

File: "grammar/lexer/operators.md"
Language: Zamani
Status: Normative lexical/operator contract
Version: 1.0
Target baseline: Rust 1.97 / Rust 1.97.1
Safety: Zamani-owned Rust integration MUST use safe Rust; "unsafe" is prohibited.
Canonical implementation: "grammar/lexer/operators.g4"

---

1. Purpose

This document defines the complete operator contract for the Zamani programming language.

It is the normative specification for:

- operator spellings;
- operator token identities;
- lexical ownership;
- compound-operator recognition;
- maximal-munch requirements;
- operator categories;
- assignment operators;
- arithmetic operators;
- comparison operators;
- logical operators;
- bitwise operators;
- shift operators;
- range operators;
- arrows;
- path/namespace operators;
- optional/null-propagation operators;
- operator extensibility;
- parser integration;
- AST integration;
- semantic integration;
- IR integration;
- diagnostics;
- compatibility;
- scalability;
- determinism;
- testing.

This file does not define operator precedence or operator semantics.

Those are downstream contracts.

---

2. Architectural Authority

The Zamani operator pipeline is:

source characters
        |
        v
grammar/lexer/operators.g4
        |
        v
canonical operator token
        |
        v
parser grammar
        |
        v
frontend AST
        |
        v
semantic analysis
        |
        +--> type checking
        +--> overload resolution
        +--> effect checking
        +--> capability checking
        +--> resource checking
        |
        v
canonical semantic model
        |
        +--> classical IR
        +--> quantum::ir
        +--> HDL/hardware representation
        +--> distributed/data/AI representations
        |
        v
optimization
        |
        v
routing / scheduling / resilience
        |
        v
target realization
        |
        v
runtime / hardware

The lexical operator layer MUST remain independent of all target hardware.

It MUST NOT know whether a program eventually executes on:

- a CPU;
- multicore CPU;
- GPU;
- FPGA;
- ASIC;
- DSP;
- accelerator;
- QPU;
- simulator;
- embedded device;
- cluster;
- distributed system;
- cloud system;
- edge system;
- future computational substrate.

---

3. File Contract

3.1 Owns

"operators.md" owns the specification of:

- operator spellings;
- operator categories;
- operator token names;
- lexical ownership;
- compound-operator relationships;
- lexical collision rules;
- operator compatibility policy;
- operator extension policy.

"operators.g4" owns the actual ANTLR lexical recognition.

---

3.2 Does not own

This file does NOT own:

- keywords;
- identifiers;
- literals;
- comments;
- whitespace;
- punctuation;
- expression precedence;
- associativity;
- type checking;
- overload resolution;
- implicit conversions;
- constant folding;
- algebraic identities;
- quantum gate semantics;
- QEC;
- ZQN;
- routing;
- scheduling;
- calibration;
- hardware discovery;
- target selection;
- deployment;
- runtime behavior.

---

4. Existing Repository Integration

The current repository already has:

grammar/lexer/operators.g4
grammar/lexer/punctuation.g4
grammar/lexer/tokens.g4
grammar/expressions/
grammar/core/
grammar/hdl/
grammar/hardware/
grammar/quantum/
grammar/resources/
grammar/security/
...

"operators.g4" already establishes itself as the lexical owner of operator spellings and deliberately separates operators from punctuation and semantic concerns.

"punctuation.g4" separately owns structural punctuation such as:

( ) { } [ ] , . ; : @ #

and explicitly separates compound forms such as:

::
?.
..
..=
->
=>

from punctuation.

This separation is retained.

---

5. Critical Existing Inconsistency

The repository currently contains an operator-token naming inconsistency.

"operators.g4" defines names including:

EQUAL_EQUAL
NOT_EQUAL
LESS_EQUAL
GREATER_EQUAL
LESS
GREATER

while "grammar/expressions/comparison.g4" documents/uses:

EQ
NE
LT
LE
GT
GE

The repository also contains grammar modules using the "EQUAL_EQUAL" family directly.

This MUST NOT remain ambiguous.

Canonical decision

The existing "operators.g4" token names are the canonical public lexer token names:

EQUAL_EQUAL
NOT_EQUAL
LESS
LESS_EQUAL
GREATER
GREATER_EQUAL

The short aliases:

EQ
NE
LT
LE
GT
GE

MUST NOT become a second lexical vocabulary.

Parser grammars MUST consume the canonical token names.

If an internal semantic layer wants shorter enum variants such as:

Eq
Ne
Lt
Le
Gt
Ge

that is acceptable, but those are AST/semantic names, not lexer token names.

---

6. Single Lexical Owner Rule

Every operator spelling MUST have exactly one lexical owner.

For example:

+       -> operators.g4
+=      -> operators.g4

.       -> punctuation.g4
..      -> operators.g4
..=     -> operators.g4
?.      -> operators.g4

:       -> punctuation.g4
::      -> operators.g4

There MUST NOT be duplicate rules such as:

LESS : '<';
LT   : '<';

for the same spelling in different lexer components.

Likewise, there must not be:

EQUAL_EQUAL : '==';
EQ           : '==';

as independent lexer tokens.

---

7. Canonical Operator Inventory

The following operator spellings constitute the current universal operator vocabulary.

7.1 Range and variadic operators

Spelling| Token| Category
"..."| "ELLIPSIS"| variadic/range syntax
"..="| "DOT_DOT_EQ"| inclusive range
".."| "DOT_DOT"| range

These are already present in "operators.g4".

---

7.2 Arrows

Spelling| Token| Category
"->"| "THIN_ARROW"| function/type/control-flow arrow
"=>"| "FAT_ARROW"| mapping/match/control-flow arrow

These remain lexical operators rather than punctuation.

---

7.3 Namespace/path operator

Spelling| Token| Category
"::"| "DOUBLE_COLON"| qualified/path/name operator

The single ":" remains owned by "punctuation.g4".

---

7.4 Equality

Spelling| Token
"=="| "EQUAL_EQUAL"
"!="| "NOT_EQUAL"

---

7.5 Relational comparison

Spelling| Token
"<"| "LESS"
"<="| "LESS_EQUAL"
">"| "GREATER"
">="| "GREATER_EQUAL"

The token names above are authoritative.

---

7.6 Logical operators

Spelling| Token
"&&"| "LOGICAL_AND"
`| 
"!"| "NOT"

Keyword forms such as:

and
or
not

are keyword-layer concerns and MUST NOT create duplicate symbolic operator tokens.

---

7.7 Arithmetic

Spelling| Token
"+"| "PLUS"
"-"| "MINUS"
"*"| "STAR"
"/"| "SLASH"
"%"| "MODULO"

---

7.8 Assignment

Spelling| Token
"="| "ASSIGN"
"+="| "PLUS_ASSIGN"
"-="| "MINUS_ASSIGN"
"*="| "STAR_ASSIGN"
"/="| "SLASH_ASSIGN"
"%="| "PERCENT_ASSIGN"
"&="| "AMP_ASSIGN"
`| =`
"^="| "CARET_ASSIGN"

---

7.9 Bitwise operators

Spelling| Token
"&"| "AMPERSAND"
`| `
"^"| "CARET"
"~"| "TILDE"

---

7.10 Shift operators

Spelling| Token
"<<"| "LEFT_SHIFT"
">>"| "RIGHT_SHIFT"

---

7.11 Increment/decrement

Spelling| Token
"++"| "INCREMENT"
"--"| "DECREMENT"

Their legality in a particular expression context is determined by the parser and semantic layers.

---

7.12 Optional/null-propagation operators

Spelling| Token
"?."| "QUESTION_DOT"
"??"| "NULL_COALESCE"

The single "?" is not currently an operator token and remains available to the punctuation contract.

---

8. Canonical Lexical Ownership Table

Source spelling| Owner| Token
"("| punctuation| "LPAREN"
")"| punctuation| "RPAREN"
"{"| punctuation| "LBRACE"
"}"| punctuation| "RBRACE"
"["| punctuation| "LBRACKET"
"]"| punctuation| "RBRACKET"
","| punctuation| "COMMA"
"."| punctuation| "DOT"
";"| punctuation| "SEMICOLON"
":"| punctuation| "COLON"
"@"| punctuation| "AT"
"#"| punctuation| "HASH"
"::"| operators| "DOUBLE_COLON"
".."| operators| "DOT_DOT"
"..="| operators| "DOT_DOT_EQ"
"..."| operators| "ELLIPSIS"
"->"| operators| "THIN_ARROW"
"=>"| operators| "FAT_ARROW"
"?."| operators| "QUESTION_DOT"
"??"| operators| "NULL_COALESCE"
"="| operators| "ASSIGN"
"=="| operators| "EQUAL_EQUAL"
"!="| operators| "NOT_EQUAL"
"<"| operators| "LESS"
"<="| operators| "LESS_EQUAL"
">"| operators| "GREATER"
">="| operators| "GREATER_EQUAL"
"+"| operators| "PLUS"
"-"| operators| "MINUS"
"*"| operators| "STAR"
"/"| operators| "SLASH"
"%"| operators| "MODULO"
"&"| operators| "AMPERSAND"
`| `| operators
"^"| operators| "CARET"
"~"| operators| "TILDE"
"!"| operators| "NOT"
"&&"| operators| "LOGICAL_AND"
`| | `
"<<"| operators| "LEFT_SHIFT"
">>"| operators| "RIGHT_SHIFT"
"+="| operators| "PLUS_ASSIGN"
"-="| operators| "MINUS_ASSIGN"
"*="| operators| "STAR_ASSIGN"
"/="| operators| "SLASH_ASSIGN"
"%="| operators| "PERCENT_ASSIGN"
"&="| operators| "AMP_ASSIGN"
`| =`| operators
"^="| operators| "CARET_ASSIGN"
"++"| operators| "INCREMENT"
"--"| operators| "DECREMENT"

---

9. Maximal-Munch Requirement

The canonical lexer MUST recognize the longest valid operator spelling.

Examples:

...   -> ELLIPSIS
..=   -> DOT_DOT_EQ
..    -> DOT_DOT

and:

->    -> THIN_ARROW
=>    -> FAT_ARROW
::    -> DOUBLE_COLON

and:

==    -> EQUAL_EQUAL
!=    -> NOT_EQUAL
<=    -> LESS_EQUAL
>=    -> GREATER_EQUAL

and:

&&    -> LOGICAL_AND
||    -> LOGICAL_OR
<<    -> LEFT_SHIFT
>>    -> RIGHT_SHIFT

and:

+=    -> PLUS_ASSIGN
-=    -> MINUS_ASSIGN
*=    -> STAR_ASSIGN
/=    -> SLASH_ASSIGN
%=    -> PERCENT_ASSIGN

and:

?.    -> QUESTION_DOT
??    -> NULL_COALESCE

The lexer MUST NOT decompose these into shorter tokens.

For example:

a >= b

MUST produce:

IDENTIFIER
GREATER_EQUAL
IDENTIFIER

not:

IDENTIFIER
GREATER
ASSIGN
IDENTIFIER

---

10. Prefix Collision Rules

The following prefix relationships require explicit tests:

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

The lexer must always select the valid complete operator.

---

11. Operator Boundaries

An operator MUST NOT be recognized inside an identifier.

For example:

value1
foo_bar
quantum_operation
matrix2

remain identifiers according to the identifier contract.

Likewise:

a>=b

and:

a >= b

must produce the same operator token sequence.

Whitespace MUST NOT be required around operators.

---

12. Operators and Unicode

The canonical operator vocabulary is ASCII unless a future language version explicitly introduces a Unicode operator.

Unicode lookalikes MUST NOT silently become equivalent operators.

For example:

＝
＜
＞ 
∧
∨

must not silently tokenize as:

=
<
>
&&
||

unless a future normative Unicode operator extension explicitly says so.

This prevents:

- visual spoofing;
- confusable source;
- ambiguous parsing;
- portability problems;
- security-sensitive source substitution.

Unicode mathematical symbols may be supported by dedicated lexical/semantic domains without redefining the universal operator vocabulary.

---

13. Operators Are Syntax, Not Semantics

The token:

PLUS

means only:

+

It does not mean:

integer addition
floating-point addition
vector addition
matrix addition
tensor addition
symbolic addition
quantum operation
hardware adder
distributed reduction

Those interpretations belong downstream.

The same principle applies to:

MINUS
STAR
SLASH
LESS
GREATER
EQUAL_EQUAL
AMPERSAND
PIPE
CARET
LEFT_SHIFT
RIGHT_SHIFT

---

14. Operator Precedence

Precedence MUST NOT be encoded in "operators.g4".

This document defines lexical identity only.

Precedence belongs to the expression parser.

The expected architectural relationship is:

lexer
  |
  v
operator token
  |
  v
expression grammar
  |
  +--> postfix
  +--> unary
  +--> multiplicative
  +--> additive
  +--> shift
  +--> relational
  +--> equality
  +--> bitwise
  +--> logical
  +--> range
  +--> conditional
  +--> assignment

The exact precedence hierarchy is owned by the expression specification and expression grammar.

Changing precedence MUST NOT require changing the operator's lexical spelling or token identity.

---

15. Operator Associativity

Associativity is not a lexical property.

The lexer must not distinguish:

a + b + c

from:

a + (b + c)

The parser and semantic system determine structure.

---

16. Operator Overloading

Operator tokens are reusable across domains.

For example:

+

may eventually be meaningful for:

integer
float
vector
matrix
tensor
symbolic expression
user-defined type
data structure
hardware value
domain-specific value

The semantic system determines legality.

The lexer MUST NOT create:

VECTOR_PLUS
MATRIX_PLUS
TENSOR_PLUS
GPU_PLUS
QPU_PLUS

merely because different semantic types use the same operator.

---

17. No Domain-Specific Operator Explosion

Do not turn every domain function into a lexical operator.

For example, do not create permanent core operators for:

FFT
SVD
QFT
gradient
backpropagation
matrix_inverse
tensor_contract
quantum_measure
quantum_reset
route
schedule
allocate_gpu
allocate_qpu

Such capabilities belong to:

- ordinary expressions;
- typed operations;
- intrinsic functions;
- libraries;
- domain semantics;
- capability declarations;
- canonical IR operations.

This keeps the lexical language extensible without making the lexer a catalog of every computing operation.

---

18. Quantum Integration

Quantum syntax may reuse ordinary operators.

For example:

a + b
a == b
a != b
a -> b

may appear in quantum-aware source contexts.

But "operators.g4" MUST NOT define physical quantum operations.

It MUST NOT contain:

H
X
Y
Z
CX
CNOT
CZ
SWAP
T
S

as universal operator tokens.

Gate names are semantic operation names and belong to the quantum operation model.

The grammar must therefore support generic quantum operations without turning the operator lexer into a fixed gate dictionary.

---

19. Canonical Quantum IR Boundary

Operators remain above the canonical quantum IR.

The correct flow is:

operator token
      |
      v
generic AST expression/operator
      |
      v
semantic quantum analysis
      |
      v
quantum::ir

The lexer MUST NOT create a quantum IR.

The lexer MUST NOT know:

- physical qubit IDs;
- coupling maps;
- gate durations;
- calibration;
- QEC code;
- error rates;
- device topology;
- QPU model.

Those remain responsibilities of the established downstream quantum architecture.

---

20. HDL Integration

The same operator token can participate in HDL semantics.

For example:

counter < limit

may become an HDL comparison.

Likewise:

signal_a & signal_b

may represent a hardware bitwise operation.

The lexer must not decide whether:

&
|
^
~
<<
>>

represent software operations, hardware logic, or another semantic domain.

That decision occurs after parsing.

---

21. Hardware and Resource Integration

Operators can appear inside resource and capability expressions:

required >= available

or:

capacity > threshold

but the operator layer MUST NOT encode actual machine limits.

There must be no:

MAX_QUBITS
MAX_CORES
MAX_THREADS
MAX_GPUS
MAX_FPGAS
MAX_NODES
MAX_MEMORY
MAX_PORTS
MAX_TENSOR_RANK
MAX_REGISTER_WIDTH

inside the operator specification.

---

22. POCO-REAF Requirement

Operators MUST be target independent.

A program using:

+
==
<
&&

does not become a CPU program merely because those operators are present.

Likewise:

q == r

does not encode a physical QPU.

The lexical layer therefore remains valid from tiny computational systems through arbitrarily larger systems, constrained only by available compilation/runtime resources.

---

23. No Artificial Language Limits

The operator specification MUST NOT impose maximum:

- expression length;
- number of operands;
- number of operators;
- number of declarations;
- number of resources;
- number of qubits;
- number of processors;
- number of accelerators;
- number of nodes;
- number of timelines;
- number of tensor dimensions.

Implementation resource budgets may exist, but they are:

implementation policy

not:

language semantics

Such budgets MUST be configurable where appropriate and MUST NOT silently change the language definition.

---

24. Assignment Operators

Compound assignment is lexical sugar for a semantic assignment form.

For example:

x += y

is lexically:

IDENTIFIER
PLUS_ASSIGN
IDENTIFIER

The parser/semantic layer determines whether it represents:

x = x + y

or a type-specific overloaded operation.

The lexer MUST NOT perform that expansion.

---

25. Increment / Decrement

The lexer recognizes:

++
--

if these remain part of the accepted Zamani language surface.

The parser and semantic layer determine:

- prefix usage;
- postfix usage;
- operand legality;
- mutability;
- effect behavior;
- evaluation ordering.

No hardware-specific interpretation is allowed.

If the language specification ultimately removes "++"/"--", that is a versioned language change rather than an ad-hoc lexer change.

---

26. Arrow Operators

"->" and "=>" are distinct.

->  = THIN_ARROW
=>  = FAT_ARROW

The parser determines whether they represent:

- function return type syntax;
- mapping;
- match arms;
- transformation;
- control-flow relation;
- domain-specific syntax.

The lexer does not assign any of these meanings.

---

27. Namespace and Path Operators

"::" is:

DOUBLE_COLON

It is distinct from:

:

which is:

COLON

owned by punctuation.

Examples:

module::name
Type::member
namespace::symbol

are parser-level constructs.

---

28. Range Operators

The range vocabulary is:

..
..=

with:

DOT_DOT
DOT_DOT_EQ

and:

...

as:

ELLIPSIS

The lexer does not decide whether a range is:

- an iterator;
- an array range;
- a compile-time range;
- a quantum register range;
- an HDL range;
- a resource range.

Semantic interpretation is downstream.

---

29. Optional and Null-Coalescing Operators

The lexical layer recognizes:

?.
??

as:

QUESTION_DOT
NULL_COALESCE

The lexer does not decide:

- nullability;
- optional types;
- lazy evaluation;
- ownership;
- error propagation;
- resource behavior.

Those belong to the type/semantic system.

---

30. Interaction With Keywords

Operators and keywords are separate lexical categories.

For example:

and
or
not

may be keyword-level logical forms if retained by the language.

They MUST NOT cause symbolic tokens:

LOGICAL_AND
LOGICAL_OR
NOT

to be duplicated.

The semantic system may normalize:

and

and:

&&

to related semantic operations if the language specification defines them as equivalent.

That equivalence is not established by the lexer.

---

31. Interaction With Identifiers

Operator characters terminate ordinary identifiers.

For example:

foo+bar

must tokenize conceptually as:

IDENTIFIER
PLUS
IDENTIFIER

and:

foo>=bar

as:

IDENTIFIER
GREATER_EQUAL
IDENTIFIER

Operator spellings MUST NOT become part of an ordinary identifier.

---

32. Interaction With Numeric Literals

The "." operator/punctuation boundary requires particular care.

The lexer must distinguish:

.
..
..=
...

from decimal literals such as:

1.0
0.5

Numeric literal ownership belongs to the numeric-literal grammar.

The operator lexer MUST NOT consume part of a numeric literal.

Likewise, a numeric literal parser MUST NOT accidentally redefine:

.
..
..=

as numeric operators.

---

33. Comments and Operators

Operator-looking text inside comments MUST NOT generate operator tokens.

For example:

// a >= b

must produce a comment token/channel according to the comment contract.

Likewise:

/*
a += b
*/

must remain comment content.

---

34. Strings and Operators

Operator characters inside strings MUST remain string contents.

For example:

" a >= b "

must not produce:

GREATER_EQUAL

tokens.

Literal grammars own this boundary.

---

35. Source Spans

Every emitted operator token must preserve its exact source span through the lexer/parser pipeline.

The operator layer MUST NOT invent a second source-location system.

The existing canonical source-span infrastructure owns:

- byte offsets;
- line;
- column;
- source-file identity;
- source mapping;
- diagnostics.

The operator token must carry or remain traceable to its original source span.

This is necessary for:

- diagnostics;
- IDE tooling;
- formatting;
- refactoring;
- macro expansion;
- semantic errors;
- provenance;
- reproducible compilation.

---

36. AST Contract

The operator lexer creates no AST.

The parser converts:

PLUS
MINUS
STAR
...

into the existing generic AST/operator representation.

The repository already contains an AST operator abstraction whose purpose is to represent source-level operator intent, with semantic analysis determining whether an operator is legal and what it means.

Therefore:

lexer operator
    |
    v
AST operator
    |
    v
semantic operator

must remain the contract.

Do not introduce a second operator AST solely for quantum, HDL, hardware, AI, or another domain.

---

37. Semantic Contract

Semantic analysis owns:

- type compatibility;
- overload resolution;
- operator traits/interfaces;
- conversions;
- evaluation rules;
- effect checking;
- ownership rules;
- mutability;
- domain semantics;
- resource semantics;
- capability requirements.

The lexer must never perform semantic validation.

For example:

a < b

is lexically valid regardless of whether "a" and "b" are actually comparable.

---

38. IR Contract

No operator lexer rule directly maps to an IR instruction.

Instead:

operator
    |
    v
AST
    |
    v
semantic operation
    |
    v
canonical IR

Possible downstream destinations include:

classical IR
quantum::ir
HDL/hardware representation
distributed IR
data/AI representation
resource/constraint representation

The operator layer remains independent of all of them.

---

39. Compiler Integration

The compiler must be able to lower operators according to semantic type and target capability.

For example:

+

could lower differently for:

integer
float
vector
matrix
tensor
symbolic value
hardware signal

without changing the lexer.

This is essential for POCO-REAF.

---

40. Runtime Integration

Runtime behavior is outside the operator lexer.

The runtime may eventually execute an operation represented by:

PLUS

but the lexer has no runtime dependency.

Parsing must work without:

- hardware discovery;
- network access;
- QPU access;
- GPU access;
- calibration;
- scheduling;
- deployment.

---

41. Optimization Integration

Optimization may transform operator expressions.

For example:

x + 0

may be simplified where semantically valid.

But the lexer must not perform that transformation.

Optimization MUST preserve:

- semantics;
- effects;
- observable behavior;
- overflow behavior;
- floating-point rules;
- quantum semantics;
- hardware semantics.

---

42. Quantum Optimization Integration

Quantum-specific optimization remains downstream.

The operator lexer does not perform:

- gate fusion;
- decomposition;
- cancellation;
- routing;
- scheduling;
- QEC;
- noise analysis.

Those remain separate responsibilities.

---

43. HDL Integration

HDL parser/semantic modules consume the same canonical operator tokens.

Existing HDL grammar modules already use common comparison/operator tokens rather than defining independent lexical concepts.

They must continue to use the universal operator vocabulary.

Do not create:

HDL_EQUAL
HDL_LESS
HDL_AND
HDL_OR

when the same language operator already exists.

---

44. Hardware Integration

Hardware grammar modules must also consume the universal operator vocabulary.

For example:

ASSIGN
EQUAL_EQUAL
NOT_EQUAL
LESS
LESS_EQUAL
GREATER
GREATER_EQUAL

remain universal tokens.

Hardware semantics determine what they mean.

They do not become hardware-specific lexical tokens.

---

45. Resource Integration

Resource constraints use the same operators.

For example:

resource.capacity >= requirement

uses:

DOT
GREATER_EQUAL

not a special:

RESOURCE_GREATER_EQUAL

This avoids lexical fragmentation.

---

46. Distributed Computing

Distributed programs use the same operator vocabulary.

No special operator token should be created merely because an operation executes across:

- processes;
- nodes;
- machines;
- clusters;
- regions;
- services.

Distribution is semantic/runtime context.

---

47. AI / Data Integration

AI and data domains reuse universal operators.

Examples:

loss < threshold
tensor_a == tensor_b
gradient += update

do not require:

AI_LESS
TENSOR_EQUAL
GRADIENT_ASSIGN

unless a future language extension establishes genuinely distinct syntax.

---

48. Security Integration

Security expressions reuse universal operators.

Examples:

policy == expected
risk <= threshold
signature != invalid

remain ordinary operators.

Cryptographic semantics belong to the security semantic layer.

---

49. No Vendor-Specific Operators in Core

The universal lexer must not permanently encode vendor-specific operators such as:

CUDA-specific
ROCm-specific
vendor-QPU-specific
FPGA-vendor-specific
ASIC-specific
cloud-provider-specific
framework-specific

Vendor-specific syntax, if genuinely necessary, must be introduced through an explicit dialect/extension mechanism.

---

50. Dialect Extension Contract

A dialect may introduce an operator only if it declares:

dialect name
dialect version
operator spelling
operator token identity
lexical ownership
parser usage
precedence
associativity
AST mapping
semantic meaning
IR mapping
compatibility
diagnostics
tests

A dialect MUST NOT silently redefine a universal operator.

A dialect also MUST NOT create lexical ambiguity with an existing core operator.

---

51. Operator Identity Stability

The following are stable lexical identities:

ASSIGN
EQUAL_EQUAL
NOT_EQUAL

LESS
LESS_EQUAL
GREATER
GREATER_EQUAL

PLUS
MINUS
STAR
SLASH
MODULO

AMPERSAND
PIPE
CARET
TILDE

LOGICAL_AND
LOGICAL_OR
NOT

LEFT_SHIFT
RIGHT_SHIFT

PLUS_ASSIGN
MINUS_ASSIGN
STAR_ASSIGN
SLASH_ASSIGN
PERCENT_ASSIGN
AMP_ASSIGN
PIPE_ASSIGN
CARET_ASSIGN

INCREMENT
DECREMENT

DOT_DOT
DOT_DOT_EQ
ELLIPSIS

THIN_ARROW
FAT_ARROW
DOUBLE_COLON

QUESTION_DOT
NULL_COALESCE

ANTLR-generated numeric token IDs MUST NOT be treated as public stable identifiers.

Only symbolic token identities are part of this contract.

---

52. "tokens.g4" Integration

The repository currently has "grammar/lexer/tokens.g4", which describes itself as the canonical lexical vocabulary and currently also contains actual keyword/type/operator-related lexer definitions.

This creates a potential second lexical authority.

Production architecture MUST converge on:

operators.md
        |
        v
operators.g4
        |
        v
canonical operator token vocabulary

while:

tokens.g4

must not independently redefine the same operator spellings.

There must be exactly one implementation rule for:

'=='
'!='
'<'
'<='
'>'
'>='
...

The existing filename "tokens.g4" should not be unnecessarily renamed.

Instead, its role should be clarified/refactored so that token vocabulary and operator spelling ownership cannot conflict.

---

53. "operators.g4" Integration

"operators.g4" is the implementation companion to this document.

It must remain responsible for actual lexical rules.

The file already explicitly separates lexical operator recognition from parser semantics, AST construction, IR, QEC, ZQN, routing, scheduling, and hardware behavior.

This document is the normative explanation/contract.

"operators.g4" is the executable lexical grammar.

Neither should silently contradict the other.

---

54. "punctuation.g4" Integration

"punctuation.g4" remains the owner of:

LPAREN
RPAREN
LBRACE
RBRACE
LBRACKET
RBRACKET
COMMA
DOT
SEMICOLON
COLON
AT
HASH

and MUST NOT define:

DOUBLE_COLON
DOT_DOT
DOT_DOT_EQ
QUESTION_DOT
NULL_COALESCE
THIN_ARROW
FAT_ARROW

This ownership boundary is already explicitly documented by the repository.

---

55. "expressions/" Integration

Expression grammars consume operator tokens.

They must not redeclare operator spellings.

For example:

comparison.g4

must consume:

EQUAL_EQUAL
NOT_EQUAL
LESS
LESS_EQUAL
GREATER
GREATER_EQUAL

rather than introducing an independent:

EQ
NE
LT
LE
GT
GE

lexer vocabulary.

This is the required correction to the current token-contract mismatch. The current comparison grammar explicitly expects the shorter names.

---

56. Parser Precedence Integration

The expression layer should contain the precedence hierarchy.

For example:

assignment
    |
conditional
    |
logical-or
    |
logical-and
    |
bitwise
    |
equality
    |
relational
    |
shift
    |
additive
    |
multiplicative
    |
unary
    |
postfix
    |
primary

The precise ordering belongs to the expression specification.

Changing precedence must not change lexical ownership.

---

57. AST Operator Mapping

The parser should map:

ASSIGN
PLUS_ASSIGN
MINUS_ASSIGN
...

to the repository's existing generic operator representation.

Do not create separate AST operator enums for:

classical
quantum
HDL
GPU
QPU
AI
distributed

unless a semantic distinction is actually required.

---

58. Semantic Operator Mapping

A canonical semantic mapping should conceptually resemble:

ASSIGN          -> Assignment
EQUAL_EQUAL     -> Equality
NOT_EQUAL       -> Inequality

LESS            -> LessThan
LESS_EQUAL      -> LessEqual
GREATER         -> GreaterThan
GREATER_EQUAL   -> GreaterEqual

PLUS            -> Add
MINUS           -> Subtract
STAR            -> Multiply
SLASH           -> Divide
MODULO          -> Remainder

AMPERSAND       -> BitAnd
PIPE            -> BitOr
CARET           -> BitXor
TILDE           -> BitNot

LOGICAL_AND     -> LogicalAnd
LOGICAL_OR      -> LogicalOr
NOT             -> LogicalNot

LEFT_SHIFT      -> ShiftLeft
RIGHT_SHIFT     -> ShiftRight

These semantic names are illustrative of the contract; the repository's existing semantic/AST types remain authoritative.

---

59. Operator Metadata

Every canonical operator should be traceable to:

token
spelling
category
parser consumers
AST representation
semantic representation
diagnostics
compatibility status

No operator should exist solely as an isolated lexer rule.

---

60. Diagnostics

Operator diagnostics belong to the existing lexical/parser diagnostic infrastructure.

Required diagnostic categories include:

Unknown operator

Source contains an unsupported operator sequence.

Malformed compound operator

A sequence resembles a compound operator but is not valid.

Invalid operator context

The token exists but cannot occur in the current syntactic position.

Deprecated operator

A legacy operator remains accepted under compatibility rules.

Conflicting dialect operator

A dialect attempts to introduce a conflicting spelling.

Ambiguous operator

Two active lexical definitions claim the same source spelling.

The diagnostic catalog itself belongs to:

grammar/lexer/diagnostics.md

This file defines required behavior, not a second diagnostic authority.

---

61. Error Recovery

The lexer must report unknown operator characters through the canonical lexer error mechanism.

It must not silently:

- discard unknown characters;
- reinterpret them as another operator;
- normalize them to a different operator;
- invoke semantic analysis;
- query hardware;
- execute code.

---

62. Security Requirements

Operator lexing must be deterministic and side-effect free.

The operator lexer must perform no:

- filesystem I/O;
- network I/O;
- process execution;
- hardware discovery;
- runtime execution;
- external command execution;
- dynamic code loading.

Unicode confusables must not silently become ASCII operators.

---

63. Determinism

For the same:

source
language version
active dialect set
lexical configuration

the operator token stream must be identical.

It must not depend on:

CPU count
GPU count
QPU count
memory size
network state
filesystem state
runtime state
scheduler state
hardware topology
device calibration

---

64. Performance

Operator recognition should be performed by the generated lexer/DFA rather than a runtime semantic lookup.

Compound operators must be recognized without repeatedly rescanning arbitrary source.

The implementation must not introduce a fixed maximum operator count as a language limitation.

A finite operator vocabulary exists for each language version, but adding a future operator must be governed by the compatibility/dialect process rather than a hard-coded capacity.

---

65. Scalability

The operator layer imposes no semantic limits on:

program size
expression size
number of expressions
number of operands
number of resources
number of qubits
number of processors
number of nodes
number of accelerators
number of devices
tensor dimensions
HDL modules
distributed processes
timelines

A sufficiently capable implementation may therefore process programs ranging from tiny embedded programs to extremely large heterogeneous/distributed computations.

The language definition must remain independent of those limits.

---

66. Rust 1.97 / 1.97.1 Contract

The grammar contains no Rust implementation.

The Rust lexer/parser/compiler integration MUST target:

Rust 1.97
Rust 1.97.1

as the supported baseline specified by the repository architecture.

Zamani-owned Rust code MUST NOT use:

unsafe

or require unsafe blocks/functions for operator processing.

Operator recognition must be implementable entirely through safe Rust.

No operator syntax should require FFI merely to be recognized.

---

67. No Hardware-Specific Operator Semantics

The following are prohibited from the lexical operator layer:

GPU_ADD
GPU_MUL
QPU_GATE
FPGA_ASSIGN
CPU_VECTOR_ADD
CUDA_OPERATOR
ROCM_OPERATOR
PHYSICAL_QUBIT_OPERATOR

Target-specific implementation belongs after semantic analysis and lowering.

---

68. Mathematical Extensibility

The same operators can serve:

scalar arithmetic
vector arithmetic
matrix arithmetic
tensor arithmetic
symbolic mathematics
statistics
optimization
scientific computing
signal processing

The grammar therefore does not need one lexical operator for every mathematical operation.

For example:

+

does not need separate:

VECTOR_PLUS
MATRIX_PLUS
TENSOR_PLUS

unless syntax genuinely differs.

---

69. AI/Data Extensibility

AI and data operations reuse universal operators.

Examples:

loss <= threshold
gradient += delta
tensor_a == tensor_b
probability >= threshold

The semantic layer determines the domain.

The lexer remains unchanged.

---

70. Distributed Extensibility

Operators do not encode distributed topology.

The following remain semantic concepts:

node
process
service
actor
channel
replica
partition
placement

Operators simply operate on values representing those concepts.

---

71. Future Operator Extensions

A new universal operator may be added only when:

1. its spelling is specified;
2. lexical ownership is assigned;
3. token name is assigned;
4. collision analysis passes;
5. parser usage is defined;
6. precedence is defined;
7. associativity is defined;
8. AST mapping is defined;
9. semantic mapping is defined;
10. diagnostics are defined;
11. compatibility behavior is defined;
12. positive tests exist;
13. negative tests exist;
14. boundary tests exist;
15. scalability tests exist;
16. dialect conflicts are checked.

No new operator is considered complete merely because "operators.g4" accepts it.

---

72. Backward Compatibility

Existing valid operator spellings must not silently change meaning.

Changing the meaning of:

+
-
*
/
==
!=
<
<=
>
>=
&&
||

is a language compatibility event.

Removing an operator requires:

deprecated
    |
    v
compatibility warning
    |
    v
migration guidance
    |
    v
versioned removal

not silent deletion.

---

73. Adding a New Operator

Adding:

<new operator>

must not accidentally change the tokenization of an existing program.

Before acceptance, test:

existing operator
existing operator + suffix
existing identifier
existing literal
existing comment
existing string
existing punctuation
existing dialect

for lexical compatibility.

---

74. Removing an Operator

An operator may be removed only through the language compatibility process.

The removal must document:

- affected syntax;
- affected parser rules;
- affected AST;
- affected semantic operations;
- migration path;
- compatibility mode;
- diagnostics;
- version boundary;
- tests.

---

75. Dialect Operator Rules

A dialect operator MUST NOT redefine:

+
-
*
/
==
!=
<
<=
>
>=
...

with a different lexical meaning.

A dialect may introduce a new spelling if it does not collide with core syntax.

Dialect activation must be explicit.

A program must not change tokenization merely because an unrelated dialect happens to be installed.

---

76. Macro Integration

Macros may manipulate operator tokens, but macro expansion must eventually pass through normal:

AST validation
semantic validation
type checking
effect checking
resource checking
IR lowering

Macros must not use operator tokens to bypass semantic safety.

---

77. Metaprogramming Integration

Metaprogramming may inspect or generate operators.

It must use the canonical operator vocabulary.

It must not create a second hidden operator language.

Generated source must be lexically equivalent to ordinary source.

---

78. Formatting / Tooling Integration

Formatter, syntax highlighter, IDE, parser diagnostics, refactoring tools, and language servers should consume the same operator token vocabulary.

They must not maintain independent operator lists.

This avoids situations where:

compiler accepts operator

but:

formatter does not recognize it

or:

IDE highlights it incorrectly

---

79. Documentation Integration

The following files must reference this contract rather than redefine the operator vocabulary:

grammar/lexer/README.md
grammar/lexer/tokens.md
grammar/lexer/operators.g4
grammar/lexer/punctuation.g4
grammar/specification/lexical.md
grammar/spec/lexical.md
grammar/spec/syntax.md
grammar/expressions/*.g4
grammar/reference/*
grammar/compatibility/*
grammar/validation/*
grammar/tests/*

No downstream file may silently introduce another operator token for an existing spelling.

---

80. "grammar.md" Integration

"grammar/grammar.md" is the implementation-conformance reference.

It should report operator support from the canonical operator contract.

It must distinguish:

specified
implemented
partially implemented
planned
deprecated

It must not independently invent an operator vocabulary.

---

81. "Zamani-Grammar.md" Integration

"Zamani-Grammar.md" may document broader or historical operator concepts.

However:

«Documentation in "Zamani-Grammar.md" does not automatically make an operator legal Zamani syntax.»

An operator becomes normative only after promotion through:

proposal
    |
lexical contract
    |
parser contract
    |
AST contract
    |
semantic contract
    |
IR contract
    |
tests
    |
compatibility approval
    |
stable language

---

82. "Zamani.g4" Integration

"Zamani.g4" is the composition root.

It must not redefine operator lexer rules already owned by "operators.g4".

Its parser rules consume canonical operator tokens.

There must be no second root operator vocabulary inside "Zamani.g4".

---

83. ANTLR Integration

The repository uses modular ANTLR grammar components.

The final assembled lexer must have one effective operator vocabulary.

Care must be taken with imported lexer grammars and catch-all rules.

In ANTLR, an imported lexer rule can be shadowed by rules in the importing grammar; therefore a root catch-all rule such as:

UNKNOWN: .;

must not accidentally take precedence over imported operator rules. This is a known ANTLR grammar-composition issue.

Therefore:

- operator rules must be present in the effective lexer vocabulary;
- catch-all handling must occur only after valid operator recognition;
- token-collision tests must exercise the assembled lexer, not only isolated grammar files.

---

84. Canonical Token Naming Rule

Use:

EQUAL_EQUAL
NOT_EQUAL
LESS
LESS_EQUAL
GREATER
GREATER_EQUAL

rather than maintaining multiple aliases such as:

EQ
NE
LT
LE
GT
GE

at lexer level.

Semantic enums may use concise forms:

Eq
Ne
Lt
Le
Gt
Ge

if appropriate.

This produces:

source spelling
    |
    v
canonical lexer token
    |
    v
semantic operator enum

rather than:

source spelling
    |
    +--> EQ
    +--> EQUAL_EQUAL
    +--> another token

---

85. Test Matrix

Every operator requires at least:

- isolated lexical test;
- prefix collision test;
- suffix collision test;
- whitespace test;
- no-whitespace test;
- identifier boundary test;
- literal boundary test;
- comment boundary test;
- parser test;
- AST test;
- semantic test;
- negative test;
- compatibility test.

---

86. Required Positive Lexical Tests

The canonical lexer must recognize:

+
-
*
/
%
=
==
!=
<
<=
>
>=
&
|
^
~
!
&&
||
<<
>>
+=
-=
*=
/=
%=
&=
|=
^=
++
--
..
..=
...
->
=>
::
?.
??

as their exact canonical tokens.

---

87. Required Prefix Tests

Test:

.
..
..=
...

Test:

:
::

Test:

?
?.
??

Test:

!
!=

Test:

<
<=
<<

Test:

>
>=
>>

Test:

+
++
+=

Test:

-
--
-=
->

and equivalent combinations for:

*
/
%
&
|
^

---

88. Required Expression Tests

Examples:

a + b
a - b
a * b
a / b
a % b

a == b
a != b
a < b
a <= b
a > b
a >= b

a && b
a || b
!a

a & b
a | b
a ^ b
~a

a << n
a >> n

x += y
x -= y
x *= y
x /= y

---

89. Required Quantum Tests

Examples should include:

q0
q1
q0 == q1
q0 != q1
state_a == state_b
observable_a >= threshold

The tests must verify that ordinary operators remain generic.

No test may establish a universal physical qubit maximum.

---

90. Required HDL Tests

Examples:

signal_a == signal_b
counter < limit
counter += step
value << shift
value & mask

The same operator tokens must be used by HDL grammar.

---

91. Required Resource Tests

Examples:

available >= required
memory >= minimum
latency <= maximum
throughput > threshold

These remain generic expressions.

---

92. Required Negative Tests

Reject or diagnose:

===
!==

unless a future language version explicitly introduces them.

Also reject malformed forms such as:

a <
a >
a ==
a !=
a +=

when the parser requires a right operand.

The lexer must not silently invent an interpretation.

---

93. Identifier Collision Tests

These must remain identifiers where appropriate:

foo
foo1
foo_bar
greater_value
plus_count
equal_value

An operator token must not be embedded into an identifier.

---

94. String Collision Tests

These must remain strings:

"+"
"=="
">="
"->"
"q[0]"

No operator token may escape from a string literal.

---

95. Comment Collision Tests

These must remain comments:

// a >= b
// x += y
// q -> r

and:

/*
a == b
x += y
*/

---

96. Scalability Tests

The test suite must include generated expressions with:

- many operators;
- long operator chains;
- deeply nested expressions;
- large aggregate operands;
- many declarations;
- large source files.

The language specification must not establish an arbitrary semantic maximum.

Implementation tests may use configurable resource budgets to prevent denial-of-service conditions.

Those budgets are not language limits.

---

97. Determinism Tests

Repeated lexing of identical source must produce identical:

token kinds
token text
source spans
token ordering

regardless of:

- host CPU;
- memory capacity;
- available GPU;
- QPU availability;
- network;
- runtime state.

---

98. Compatibility Tests

For each language version, test:

accepted operators
deprecated operators
removed operators
dialect operators
operator precedence
operator token identities

No version transition may silently reinterpret an existing operator without an explicit compatibility rule.

---

99. Hard-Coding Audit

This file passes the hard-coding audit only if it contains no language-level limit involving:

MAX_QUBITS
MAX_CPUS
MAX_CORES
MAX_THREADS
MAX_GPUS
MAX_FPGAS
MAX_NODES
MAX_DEVICES
MAX_MEMORY
MAX_REGISTERS
MAX_VECTOR_WIDTH
MAX_TENSOR_RANK
MAX_CIRCUIT_DEPTH
MAX_EXPRESSION_SIZE

The operator grammar must never depend on target hardware.

---

100. Completion Criteria

"grammar/lexer/operators.md" is complete when all of the following are true:

- [x] operator lexical ownership is defined;
- [x] punctuation ownership is separated;
- [x] canonical operator spellings are documented;
- [x] canonical token names are documented;
- [x] compound operators are documented;
- [x] maximal munch is specified;
- [x] precedence ownership is separated;
- [x] semantic ownership is separated;
- [x] AST integration is defined;
- [x] IR integration is defined;
- [x] quantum integration is defined;
- [x] HDL integration is defined;
- [x] hardware integration is defined;
- [x] resource integration is defined;
- [x] AI/data integration is defined;
- [x] distributed integration is defined;
- [x] dialect integration is defined;
- [x] macro integration is defined;
- [x] tooling integration is defined;
- [x] compatibility rules are defined;
- [x] source-span ownership is defined;
- [x] diagnostics ownership is defined;
- [x] determinism requirements are defined;
- [x] security requirements are defined;
- [x] scalability requirements are defined;
- [x] Rust 1.97/1.97.1 requirement is defined;
- [x] unsafe Rust is prohibited;
- [x] hard-coding policy is defined;
- [x] tests are defined;
- [x] existing token-name inconsistency is identified;
- [x] canonical token naming is resolved;
- [x] no unnecessary existing filename rename is required.

---

101. Required Repository Integration Changes

This document itself does not require renaming existing files.

The following integration work is required elsewhere:

"grammar/lexer/operators.g4"

Make it conform exactly to this document.

It remains the implementation owner of operator spellings.

"grammar/lexer/punctuation.g4"

Keep structural punctuation separate.

No duplicate operator rules.

"grammar/lexer/tokens.g4"

Resolve its current overlap with "operators.g4".

It must not independently define a competing operator vocabulary.

"grammar/expressions/comparison.g4"

Replace lexer-level expectations:

EQ
NE
LT
LE
GT
GE

with the canonical vocabulary:

EQUAL_EQUAL
NOT_EQUAL
LESS
LESS_EQUAL
GREATER
GREATER_EQUAL

unless the repository deliberately introduces a separate semantic alias layer.

"grammar/expressions/*.g4"

All expression grammars must consume the canonical operator tokens.

"grammar/hdl/*.g4"

Reuse universal operator tokens.

"grammar/hardware/*.g4"

Reuse universal operator tokens.

"grammar/resources/*.g4"

Reuse universal operator tokens.

"grammar/quantum/*.g4"

Reuse universal operator tokens and do not create fixed quantum-gate operator tokens.

"grammar/specification/lexical.md"

Reference this document as the operator lexical contract.

"grammar/spec/lexical.md"

Formalize the operator ownership/token invariants.

"grammar/validation/duplicate-tokens.md"

Validate that no operator spelling has multiple lexical owners.

"grammar/validation/keyword-collisions.md"

Ensure operator spellings cannot be consumed as keyword/identifier text incorrectly.

"grammar/tests/lexical/"

Add the complete operator matrix.

"grammar/tests/negative/"

Add malformed/unsupported operator cases.

"grammar/tests/scalability/"

Add large operator-chain and large-source tests.

"grammar/compatibility/"

Record operator additions/removals/changes by language version.

---

102. Final Architectural Rule

The definitive Zamani operator model is:

                    SOURCE
                      |
                      v
             operators.g4
                      |
                      v
             canonical token
                      |
                      v
              parser grammar
                      |
                      v
               generic AST
                      |
                      v
            semantic operator
                      |
          +-----------+-----------+
          |           |           |
          v           v           v
      classical    quantum::ir    HDL
          |           |           |
          +-----------+-----------+
                      |
                      v
                optimization
                      |
          +-----------+-----------+
          |           |           |
          v           v           v
       routing    scheduling   resilience
                      |
                      v
                     ZQN
                      |
                      v
                     HAL
                      |
                      v
              target realization
                      |
          +-----------+-----------+
          |           |           |
          v           v           v
         CPU         GPU         QPU
          |           |           |
          +-----------+-----------+
                      |
                      v
                 future targets

The operator layer therefore expresses syntax, not hardware.

It must remain capable of supporting programs that scale from:

one operation

to:

arbitrarily large classical computation
arbitrarily large quantum computation
arbitrarily large HDL designs
arbitrarily large heterogeneous systems
arbitrarily large distributed systems

subject only to actual available compilation/runtime resources.

That separation is a necessary part of Zamani's:

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever

architecture.