Worked for 34s

I read the actual repository files relevant to precedence. In particular, the repository already designates grammar/expressions/precedence.md as the authoritative precedence table, while the normative syntax specification says precedence and associativity belong to the expression layer rather than the lexer.

There is also an important existing inconsistency that this file must explicitly resolve: grammar/lexer/operators.md establishes canonical lexer names such as EQUAL_EQUAL, NOT_EQUAL, LESS, LESS_EQUAL, GREATER, GREATER_EQUAL, LEFT_SHIFT, RIGHT_SHIFT, MODULO, LOGICAL_AND, and LOGICAL_OR; expression grammars must not invent alternate names such as EQ, LT, LE, etc.

grammar/expressions/precedence.md does not currently exist, so this should be created, not renamed from another file.

Here is the production-ready file:

# Zamani Expression Precedence and Associativity

**File:** `grammar/expressions/precedence.md`  
**Status:** Normative / Production  
**Specification version:** 1.0  
**Language:** Zamani  
**Implementation baseline:** Rust 1.97 / Rust 1.97.1  
**Edition:** Rust 2021  
**Safety:** Safe Rust only; Zamani compiler/runtime implementation must not require `unsafe` Rust.

---

## 0. Purpose

This document defines the **single authoritative precedence and associativity
contract** for Zamani source expressions.

It determines how tokenized expressions are structurally grouped by the
parser.

It is the contract shared by:

- `grammar/expressions/expression.g4`;
- expression subgrammars under `grammar/expressions/`;
- `grammar/spec/syntax.md`;
- `grammar/specification/syntax.md`;
- `grammar/lexer/operators.md`;
- `grammar/lexer/operators.g4`;
- `grammar/lexer/tokens.g4`;
- `grammar/Zamani.g4`;
- `src/lexer.rs`;
- `src/parser.rs`;
- frontend AST construction;
- structural validation;
- semantic analysis;
- expression lowering;
- canonical IR integration;
- conformance tests.

The repository already identifies this file as the authoritative location for
expression precedence. `grammar/grammar.md` explicitly states that the parser
implementation must conform to `grammar/expressions/precedence.md`.

---

# 1. Scope

This document owns:

- precedence levels;
- relative precedence;
- associativity;
- grouping rules;
- expression-chain behavior;
- operator binding;
- parenthesized-expression behavior;
- parser-level ambiguity resolution;
- precedence compatibility requirements;
- precedence-related diagnostics;
- precedence conformance tests.

This document does **not** own:

- lexical token spelling;
- tokenization;
- keyword recognition;
- operator semantics;
- type checking;
- overload resolution;
- implicit conversion;
- ownership;
- borrowing;
- effect checking;
- capability checking;
- resource feasibility;
- optimization;
- constant folding;
- quantum operation semantics;
- QEC;
- ZQN;
- routing;
- scheduling;
- calibration;
- HAL behavior;
- target selection;
- runtime behavior.

Those concerns remain downstream.

---

# 2. Architectural Authority

The expression pipeline is:

```text
source text
    |
    v
canonical lexer
    |
    v
canonical operator tokens
    |
    v
precedence-defined parser
    |
    v
domain-neutral frontend AST
    |
    v
structural validation
    |
    v
semantic analysis
    |
    v
canonical semantic model / ZUIR
    |
    +--------------------+--------------------+
    |                    |                    |
    v                    v                    v
classical              quantum::ir       HDL/hardware
    |                    |                    |
    +--------------------+--------------------+
                         |
                         v
                 optimization/lowering
                         |
                         v
              routing/scheduling/resilience
                         |
                         v
                        QEC
                         |
                         v
                        ZQN
                         |
                         v
                        HAL
                         |
                         v
                  target realization

Precedence is therefore a source-structure concern.

It must not depend on:

available hardware;

target architecture;

number of CPUs;

number of cores;

number of threads;

number of GPUs;

number of QPUs;

number of qubits;

memory capacity;

accelerator availability;

topology;

runtime state;

scheduling state;

network state;

calibration state.



---

3. Single Precedence Authority

There MUST be exactly one normative precedence table.

That table is this file.

The following files must conform to this document:

grammar/expressions/expression.g4
grammar/expressions/assignment.g4
grammar/expressions/conditionals.g4
grammar/expressions/range.g4
grammar/expressions/ranges.g4
grammar/expressions/logical.g4
grammar/expressions/bitwise.g4
grammar/expressions/comparison.g4
grammar/expressions/shift.g4
grammar/expressions/arithmetic.g4
grammar/expressions/unary.g4
grammar/expressions/postfix.g4
grammar/expressions/binary.g4
grammar/expressions/async.g4
grammar/expressions/quantum-expressions.g4
grammar/expressions/effects.g4
grammar/expressions/metaprogramming.g4

Only files that actually exist or are retained by the repository need to be wired into the final composition.

Specialized grammar files MUST NOT publish a conflicting precedence hierarchy.

A specialized file may document the precedence level it implements, but it must derive that level from this contract.


---

4. Precedence Model

Zamani uses a precedence hierarchy.

A larger binding priority means that an expression binds more tightly.

From lowest precedence to highest precedence:

Level	Expression category	Associativity

1	Assignment	Right
2	Conditional	Right
3	Range	Non-associative
4	Logical OR	Left
5	Logical AND	Left
6	Bitwise OR	Left
7	Bitwise XOR	Left
8	Bitwise AND	Left
9	Equality	Non-associative
10	Relational	Non-associative
11	Shift	Left
12	Additive	Left
13	Multiplicative	Left
14	Prefix / unary	Right
15	Postfix	Left
16	Primary / atomic	N/A


This ordering is normative.


---

5. Level 16 — Primary / Atomic Expressions

Primary expressions bind most tightly because they are atomic expression components.

Examples include:

identifier
literal
parenthesized expression
array literal
tuple literal
map literal
lambda/closure literal
block expression
domain-specific primary expression

A primary expression does not contain an implicit operator relationship with another primary expression.

Examples:

x
42
"hello"
true
(x)
[value]
(a, b)

Primary syntax belongs to the appropriate expression subgrammar.

This file only defines its precedence position.


---

6. Level 15 — Postfix Expressions

Postfix expressions bind more tightly than prefix, multiplicative, additive, shift, comparison, logical, range, conditional, and assignment expressions.

Postfix constructs include the repository's applicable forms such as:

call
index
member access
qualified/member access
optional/null-propagating access
postfix operators

Examples:

f(x)
value[index]
object.field
object?.field
value[index].field(arg)

Postfix chaining is structurally repeatable.

There is no language-level maximum on postfix-chain length.

For example:

a.b.c.d.e.f.g(...)

is not rejected because the chain has reached an arbitrary language-defined depth.

Operational parser limits, if any, are implementation resource policies and must not become language semantics.

Associativity

Postfix chaining is left associative.

Conceptually:

a.b.c

groups as:

(a.b).c

and:

f(a)(b)

groups as:

(f(a))(b)

Semantic validity is determined downstream.


---

7. Level 14 — Prefix / Unary Expressions

Prefix operators bind more tightly than multiplicative operators and less tightly than postfix expressions.

Canonical prefix operators include:

+
-
!
~
&
*

where those operators are available in the canonical lexical vocabulary.

Examples:

-x
!flag
~bits
&value
*reference
---x
!!flag

Prefix operators may nest without an artificial language-defined maximum.

Associativity

Prefix operators are right associative.

Conceptually:

---x

groups as:

-( -( -x ) )

The semantic layer determines whether a particular operator/type combination is legal.


---

8. Level 13 — Multiplicative Expressions

Multiplicative operators:

*
/
%

Canonical lexical token identities:

STAR
SLASH
MODULO

Examples:

a * b
a / b
a % b
a * b / c

Associativity

Multiplicative operators are left associative.

Therefore:

a * b / c

groups as:

(a * b) / c

and:

a / b * c

groups as:

(a / b) * c

The grammar must not silently reinterpret these as right-associated expressions.


---

9. Level 12 — Additive Expressions

Additive operators:

+
-

Canonical tokens:

PLUS
MINUS

Examples:

a + b
a - b
a + b - c

Associativity

Additive operators are left associative.

Therefore:

a + b - c

groups as:

(a + b) - c


---

10. Level 11 — Shift Expressions

Shift operators:

<<
>>

Canonical tokens:

LEFT_SHIFT
RIGHT_SHIFT

Examples:

value << amount
value >> amount
a << b << c

Associativity

Shift operators are left associative.

Therefore:

a << b >> c

groups as:

(a << b) >> c

The grammar must not use the non-canonical token names:

SHIFT_LEFT
SHIFT_RIGHT

when the canonical lexer contract defines:

LEFT_SHIFT
RIGHT_SHIFT


---

11. Level 10 — Relational Expressions

Relational operators:

<
<=
>
>=

Canonical tokens:

LESS
LESS_EQUAL
GREATER
GREATER_EQUAL

Examples:

a < b
a <= b
a > b
a >= b

Associativity

Relational comparison is non-associative.

Therefore:

a < b

is valid.

But:

a < b < c

must NOT silently acquire an arbitrary mathematical interpretation.

The parser/semantic contract must either:

1. reject chained relational comparisons; or


2. introduce an explicitly specified language construct for comparison chaining.



Until such a construct is normatively specified, the production expression language treats chained relational operators as invalid.

Parentheses make intent explicit:

(a < b) < c

or:

a < (b < c)

The semantic layer decides whether either explicitly grouped expression is type-correct.


---

12. Level 9 — Equality Expressions

Equality operators:

==
!=

Canonical tokens:

EQUAL_EQUAL
NOT_EQUAL

Examples:

a == b
a != b

Associativity

Equality is non-associative.

Therefore:

a == b == c

must not silently become:

(a == b) == c

or:

a == (b == c)

without an explicit language rule.

Use explicit grouping where chained comparison semantics are intended.


---

13. Level 8 — Bitwise AND

Canonical operator:

&

Canonical token:

AMPERSAND

Example:

a & b

Associativity

Bitwise AND is left associative.

a & b & c

groups as:

(a & b) & c

The token's semantics remain type-dependent.

The lexer does not distinguish:

integer AND
bit-vector AND
hardware signal AND
domain-specific AND

Those meanings belong downstream.


---

14. Level 7 — Bitwise XOR

Canonical operator:

^

Canonical token:

CARET

Example:

a ^ b

Associativity

Bitwise XOR is left associative.

a ^ b ^ c

groups as:

(a ^ b) ^ c


---

15. Level 6 — Bitwise OR

Canonical operator:

|

Canonical token:

PIPE

Example:

a | b

Associativity

Bitwise OR is left associative.

a | b | c

groups as:

(a | b) | c


---

16. Level 5 — Logical AND

Canonical symbolic operator:

&&

Canonical token:

LOGICAL_AND

Keyword-form logical operators, where supported by the language specification, must map to the canonical semantic logical operation rather than creating a second semantic operator family.

Example:

a && b

Associativity

Logical AND is left associative.

a && b && c

groups as:

(a && b) && c

Short-circuit behavior is semantic/runtime behavior and is not defined by precedence alone.


---

17. Level 4 — Logical OR

Canonical symbolic operator:

||

Canonical token:

LOGICAL_OR

Example:

a || b

Associativity

Logical OR is left associative.

a || b || c

groups as:

(a || b) || c

Short-circuit behavior is defined by the semantic specification, not by the lexer.


---

18. Level 3 — Range Expressions

Canonical range operators:

..
..=

Canonical tokens:

DOT_DOT
DOT_DOT_EQ

Examples:

start .. end
start ..= end

Associativity

Range construction is non-associative.

An expression such as:

a .. b .. c

must not silently acquire an arbitrary grouping.

If nested ranges are required, use explicit parentheses:

(a .. b) .. c

or:

a .. (b .. c)

and allow semantic analysis to determine whether the resulting types are valid.

Scalability

A range has no grammar-level maximum number of representable values.

For example:

0 .. n

remains valid for any representable program-level n.

The grammar must not establish:

MAX_RANGE_LENGTH

or an equivalent artificial limit.

Whether a range is:

eager;

lazy;

distributed;

parallel;

symbolic;

infinite;

materialized;

stream-like;


is a semantic/runtime decision.


---

19. Level 2 — Conditional Expressions

The conditional expression form is:

condition ? when_true : when_false

Canonical structural tokens are determined by the canonical punctuation/operator vocabulary.

Example:

condition ? a : b

Associativity

Conditional expressions are right associative.

Therefore:

a ? b : c ? d : e

groups as:

a ? b : (c ? d : e)

Explicit parentheses may always be used to make the desired grouping clear.

The condition itself is parsed according to the normal expression hierarchy.

The true and false branches are full expression contexts subject to the language's expression grammar and semantic rules.


---

20. Level 1 — Assignment Expressions

Assignment has the lowest precedence among ordinary Zamani expressions.

Canonical assignment operators include:

=
+=
-=
*=
/=
%=
&=
|=
^=

subject to the canonical lexer/operator vocabulary.

Examples:

x = y
x += y
x -= y
x *= y
x /= y
x %= y

Associativity

Assignment is right associative.

Therefore:

a = b = c

groups as:

a = (b = c)

This permits chained assignment where semantic analysis determines that the operands are valid assignment targets and the resulting expression types are compatible.

Assignment target legality

Precedence does NOT determine whether an expression is assignable.

For example:

42 = x

may be structurally parseable as an assignment expression, but semantic validation must reject an invalid assignment target.

The grammar must not encode machine-specific lvalue/rvalue limits.


---

21. Parentheses Override Precedence

Parenthesized expressions explicitly establish grouping.

For example:

a + b * c

groups as:

a + (b * c)

while:

(a + b) * c

groups as:

(a + b) * c

Parentheses therefore provide an explicit structural override.

Parentheses do not change the semantic meaning of an operator by themselves.


---

22. Canonical Precedence Table

The following table is the authoritative compact representation.

Priority	Category	Operators / Forms	Token family	Associativity

1	Assignment	=, +=, -=, *=, /=, %=, &=, |=, ^=	assignment tokens	Right
2	Conditional	? :	conditional punctuation/tokens	Right
3	Range	.., ..=	DOT_DOT, DOT_DOT_EQ	Non-associative
4	Logical OR	||	LOGICAL_OR	Left
5	Logical AND	&&	LOGICAL_AND	Left
6	Bitwise OR	|	PIPE	Left
7	Bitwise XOR	^	CARET	Left
8	Bitwise AND	&	AMPERSAND	Left
9	Equality	==, !=	EQUAL_EQUAL, NOT_EQUAL	Non-associative
10	Relational	<, <=, >, >=	LESS, LESS_EQUAL, GREATER, GREATER_EQUAL	Non-associative
11	Shift	<<, >>	LEFT_SHIFT, RIGHT_SHIFT	Left
12	Additive	+, -	PLUS, MINUS	Left
13	Multiplicative	*, /, %	STAR, SLASH, MODULO	Left
14	Prefix	+, -, !, ~, &, *	operator tokens	Right
15	Postfix	calls, indexing, member access, postfix forms	applicable canonical tokens	Left
16	Primary	identifiers, literals, grouped/atomic forms	applicable tokens	N/A


Higher priority means tighter binding.


---

23. Canonical Examples

23.1 Arithmetic

a + b * c

means structurally:

a + (b * c)

because multiplication has higher precedence than addition.


---

23.2 Shift and arithmetic

a + b << c

means:

(a + b) << c

because additive expressions bind more tightly than shifts.


---

23.3 Comparison

a + b < c * d

means:

(a + b) < (c * d)


---

23.4 Logical operators

a || b && c

means:

a || (b && c)


---

23.5 Bitwise and logical operators

a | b && c

means:

(a | b) && c

because bitwise OR has higher precedence than logical AND.


---

23.6 Conditional

a || b ? c + d : e * f

means:

(a || b) ? (c + d) : (e * f)


---

23.7 Assignment

x = a + b * c

means:

x = (a + (b * c))


---

23.8 Chained assignment

a = b = c

means:

a = (b = c)


---

23.9 Postfix binding

f(x)[i].field

groups structurally as:

((f(x))[i]).field


---

23.10 Prefix and postfix

-f(x)

means:

-(f(x))

not:

(-f)(x)


---

24. Operator Token Authority

Precedence MUST use the canonical lexical token names.

The current canonical operator contract establishes names including:

EQUAL_EQUAL
NOT_EQUAL

LESS
LESS_EQUAL
GREATER
GREATER_EQUAL

LOGICAL_AND
LOGICAL_OR

PLUS
MINUS
STAR
SLASH
MODULO

AMPERSAND
PIPE
CARET
TILDE

LEFT_SHIFT
RIGHT_SHIFT

ASSIGN
PLUS_ASSIGN
MINUS_ASSIGN
STAR_ASSIGN
SLASH_ASSIGN
PERCENT_ASSIGN
AMP_ASSIGN
PIPE_ASSIGN
CARET_ASSIGN

DOT_DOT
DOT_DOT_EQ

Expression grammars MUST NOT introduce alternate lexer vocabularies such as:

EQ
NE
LT
LE
GT
GE

SHIFT_LEFT
SHIFT_RIGHT

MOD

AND_AND
OR_OR

unless the lexical authority explicitly adopts those names.

Short semantic names may exist in AST or semantic representations, but they are not lexer token identities.


---

25. Lexical Precedence vs Syntactic Precedence

Lexical maximal-munch and expression precedence are different mechanisms.

For example:

a >= b

must first lex as:

IDENTIFIER
GREATER_EQUAL
IDENTIFIER

It must not become:

IDENTIFIER
GREATER
ASSIGN
IDENTIFIER

Likewise:

a >> b

must lex as:

IDENTIFIER
RIGHT_SHIFT
IDENTIFIER

before expression precedence is applied.

Therefore:

lexical recognition

comes before:

syntactic precedence

The precedence document MUST NOT attempt to solve lexical maximal-munch problems.

Those belong to:

grammar/lexer/operators.g4
grammar/lexer/operators.md


---

26. Keyword Operators

Keyword forms such as:

and
or
not

must not accidentally create a second incompatible precedence hierarchy.

If a keyword form is specified as semantically equivalent to a symbolic operator, its parser representation must enter the same precedence level.

For example, if:

and

is the keyword form of logical conjunction, it must have the same precedence and associativity as logical AND.

Likewise, if:

or

is the keyword form of logical disjunction, it must have the same precedence and associativity as logical OR.

The exact accepted keyword forms remain governed by the lexical and syntax specifications.


---

27. Operator Overloading

Precedence is independent of operator semantics.

For example:

a + b

may eventually represent addition over:

integers;

floating-point values;

vectors;

matrices;

tensors;

symbolic values;

user-defined values;

data structures;

hardware values;

domain-specific semantic values.


The parser does not select the meaning.

Semantic analysis determines the applicable operation.

The grammar MUST NOT create separate precedence levels merely because the same operator is used by different domains.


---

28. Quantum Integration

Quantum expressions use the same expression precedence system unless a specific quantum syntax contract explicitly defines a separate syntactic construct.

For example:

angle_a + angle_b

uses ordinary additive precedence.

A quantum operation expression may contain ordinary:

calls;

indexing;

member access;

arithmetic;

comparisons;

logical expressions;

ranges;

conditionals;

assignments where permitted.


Quantum semantics remain downstream.

The precedence grammar MUST NOT introduce:

quantum_addition_precedence
quantum_gate_precedence
qpu_precedence
physical_qubit_precedence

or any equivalent target-specific hierarchy.

The canonical quantum semantic boundary remains:

source expression
    |
    v
domain-neutral AST
    |
    v
semantic analysis
    |
    v
quantum::ir

This document does not create another quantum IR.


---

29. Classical Integration

Classical numerical expressions use the same universal precedence hierarchy.

Examples:

a + b * c

matrix_a * matrix_b + vector_c

tensor_a + tensor_b * scalar

The grammar does not decide whether * means:

scalar multiplication;

matrix multiplication;

tensor contraction;

user-defined multiplication;

hardware operation.


Those meanings belong to semantic analysis and canonical IR lowering.


---

30. HDL and Hardware Integration

HDL/hardware expressions inherit the universal precedence rules unless an explicit HDL syntactic construct is specified.

The precedence system MUST NOT depend on:

signal width;

register width;

FPGA family;

ASIC process;

number of gates;

number of ports;

number of pipeline stages;

number of hardware units.


For example:

a + b * c

has the same source grouping regardless of whether it is ultimately lowered to:

CPU instructions;

GPU instructions;

FPGA logic;

ASIC hardware;

a simulator;

another future computational substrate.



---

31. AI / Data / Distributed Integration

AI, tensor, data, networking, and distributed expressions inherit the same universal expression precedence.

The grammar must not create framework-specific precedence for:

CUDA
ROCm
PyTorch
TensorFlow
vendor-specific accelerators

or any other implementation framework.

Likewise, distributed execution must not create precedence based on:

node count;

worker count;

cluster size;

network topology;

accelerator count.


Those are semantic/resource/deployment concerns.


---

32. Metaprogramming Integration

Metaprogramming expressions must enter the expression hierarchy at an explicitly documented precedence level.

A metaprogramming feature MUST NOT silently redefine:

expression
assignmentExpression
conditionalExpression
rangeExpression
logicalOrExpression
...

A file such as:

grammar/expressions/metaprogramming.g4

may provide expression-level integration, but the authoritative precedence position remains here.

Declaration-level metaprogramming belongs under:

grammar/metaprogramming/

and does not automatically become an expression.


---

33. Effects Integration

Effectful expressions participate in the normal expression hierarchy.

Effect syntax MUST NOT create an independent precedence system.

For example, an effect invocation embedded in an expression must have a documented position relative to calls, postfix expressions, unary operators, and the remaining hierarchy.

Effect semantics remain owned by:

grammar/effects/

and semantic analysis.

The expression layer only establishes structural grouping.


---

34. Ranges and Iteration

Range expressions are deliberately positioned above logical expressions and below conditional expressions.

This allows structures such as:

start .. end

and:

condition ? start .. end : fallback

to have deterministic parsing.

Where ambiguity could materially affect readability, parentheses are recommended:

condition ? (start .. end) : fallback

The parser must still follow the normative hierarchy.


---

35. No Artificial Scalability Limits

This document establishes no maximum for:

expression depth;

operator-chain length;

postfix-chain length;

call-chain length;

range magnitude;

number of operands;

number of arguments;

number of indices;

number of tuple elements;

number of array elements;

number of nested expressions;

number of parenthesis levels;

quantum operands;

quantum operation parameters;

tensor dimensions;

distributed participants;

timelines.


The language is intended to scale:

atom
    ->
small embedded system
    ->
single processor
    ->
multicore
    ->
many-core
    ->
GPU
    ->
FPGA
    ->
ASIC
    ->
QPU
    ->
cluster
    ->
distributed system
    ->
cloud
    ->
future computational substrate

subject only to actual representational, implementation, resource, and execution constraints.

"Infinity" therefore means:

> no artificial finite language-level maximum is introduced by this precedence contract.



It does not claim that a physical machine has infinite resources.


---

36. Resource Independence

Precedence must remain identical regardless of resource availability.

The parser must not change the grouping of:

a + b * c

because:

a GPU is available;

a QPU is unavailable;

memory is low;

a cluster is large;

a device has a different topology;

a compiler optimization is enabled;

a runtime scheduler chooses a different target.


Parsing is deterministic with respect to source tokens and language version.


---

37. Determinism

For the same:

source
+
canonical lexical vocabulary
+
language version
+
grammar version

the parser must produce the same expression structure.

Precedence must not depend on:

system time
randomness
environment variables
hardware discovery
device state
network state
runtime scheduling
resource availability
calibration state


---

38. AST Contract

The parser must preserve the grouping established by this precedence table.

For:

a + b * c

the AST must represent:

Binary(
    operator = PLUS,
    left = a,
    right = Binary(
        operator = STAR,
        left = b,
        right = c
    )
)

or the repository's equivalent domain-neutral representation.

The exact AST type belongs to the frontend AST contract.

The precedence document does not define concrete Rust struct names.

The AST must preserve:

operator identity;

operand order;

grouping;

source span;

syntactic nesting;

explicit parentheses where source provenance requires them.


It must not introduce target-specific information merely because the expression may eventually execute on a target machine.


---

39. Semantic Contract

The parser determines:

> how the expression is grouped.



Semantic analysis determines:

> whether that grouped expression is meaningful.



Semantic analysis owns:

type checking;

overload resolution;

conversion;

name resolution;

ownership;

borrowing;

effect checking;

capability checking;

resource requirements;

quantum legality;

classical legality;

HDL legality;

interoperability.


Therefore:

a + b * c

has a deterministic parse independent of whether a, b, and c are:

integers;

matrices;

tensors;

symbolic expressions;

hardware values;

quantum-related semantic values;

distributed data;

user-defined values.



---

40. IR Contract

Precedence must be fully resolved before semantic IR lowering.

The flow is:

tokens
  |
  v
expression parse tree
  |
  v
domain-neutral AST
  |
  v
semantic analysis
  |
  v
canonical semantic model
  |
  +--------------------+-------------------+
  |                    |                   |
  v                    v                   v
classical IR       quantum::ir       HDL/hardware IR

The precedence document must never introduce an alternative IR.

For quantum constructs, the canonical quantum boundary remains:

quantum::ir


---

41. Compiler Integration

The compiler must consume the already-grouped semantic representation.

Compiler optimization MUST NOT reinterpret source precedence.

For example, an optimizer may transform mathematically equivalent operations where the semantic contract permits it, but it cannot pretend that:

a + b * c

was parsed as:

(a + b) * c

merely because an optimization would be convenient.

Any transformation must preserve program semantics.


---

42. Runtime Integration

Runtime behavior must never affect expression parsing.

Runtime scheduling, resource allocation, hardware selection, QEC, routing, ZQN, calibration, and HAL decisions occur after expression structure has been established.


---

43. Compatibility Contract

Changing any precedence or associativity relationship is a language compatibility change.

Examples of compatibility-sensitive changes include:

a + b * c

changing from:

a + (b * c)

to:

(a + b) * c

or:

a = b = c

changing from right associative to left associative.

Such changes require:

1. specification update;


2. grammar update;


3. parser conformance update;


4. AST conformance update;


5. semantic compatibility analysis;


6. migration documentation;


7. compatibility tests;


8. versioning decision.



A precedence change must never be introduced merely by editing one expression grammar.


---

44. Backward Compatibility

Existing valid programs must retain their parse structure across compatible language releases.

If a new operator is introduced:

its lexical token must be canonical;

its precedence must be explicitly assigned;

its associativity must be explicitly assigned;

ambiguous interactions must be tested;

compatibility implications must be documented.


No new operator may implicitly steal an existing operator's precedence level without an explicit specification decision.


---

45. New Operator Contract

Every future operator proposal MUST specify:

operator spelling
lexer token
lexical owner
precedence level
associativity
prefix/infix/postfix role
valid operands
AST representation
semantic meaning
effect behavior
capability requirements
resource implications
IR mapping
compiler consumers
runtime consumers
diagnostics
compatibility impact
positive tests
negative tests
boundary tests
scalability tests
determinism tests

The proposal is incomplete without all of these.


---

46. Ambiguity Policy

When a new syntax construct could be interpreted at multiple precedence levels, the language specification must resolve the ambiguity explicitly.

The implementation must not depend on:

ANTLR adaptive prediction accidentally selecting a preferred meaning;

generated parser behavior;

source formatting;

whitespace;

target hardware;

semantic guessing;

runtime information.


Ambiguity must be resolved by the language contract.


---

47. Whitespace Independence

Precedence does not depend on whitespace.

These forms must have equivalent token structure:

a+b*c

and:

a + b * c

Likewise:

a>=b

and:

a >= b

must have equivalent operator tokenization.

Whitespace remains the responsibility of the lexical contract.


---

48. Unicode Independence

Unicode lookalikes must not silently change precedence.

For example, visually similar characters must not automatically become:

+
-
*
/
<
>
=

or any other canonical operator.

Unicode operator extensions require an explicit lexical and syntax specification.


---

49. Error Diagnostics

Precedence-related diagnostics must identify structural problems without guessing semantic intent.

Examples include:

ambiguous expression
invalid chained comparison
invalid chained range
unexpected operator
missing conditional separator
missing assignment operand
unexpected postfix operator

Diagnostics should include source spans where available.

The parser must not silently rewrite malformed expressions into a different valid expression merely to recover.

Error recovery policy belongs to the parser/diagnostics contract, but it must preserve the precedence contract for successfully parsed expressions.


---

50. Testing Contract

The precedence implementation is incomplete without tests.

50.1 Positive tests

At minimum:

a + b * c
a * b + c
a << b + c
a < b
a == b
a & b
a ^ b
a | b
a && b
a || b
a .. b
a ..= b
a ? b : c
a = b
a = b = c
f(x)[i].field
-f(x)


---

50.2 Parentheses tests

(a + b) * c
a * (b + c)
(a < b) == c
a == (b == c)
(a .. b) .. c

The parser must preserve explicit grouping.


---

50.3 Negative tests

At minimum:

a < b < c
a == b == c
a .. b .. c
a ? b
a = 
= b

where those forms are not made valid by another normative syntax rule.


---

50.4 Associativity tests

Left associative

a + b + c
a * b * c
a << b << c
a & b & c
a ^ b ^ c
a | b | c
a && b && c
a || b || c

Right associative

a = b = c
a ? b : c ? d : e

Non-associative

a < b < c
a == b == c
a .. b .. c


---

51. Maximal-Munch Interaction Tests

The lexer/parser integration must test:

a >= b
a > = b

a <= b
a < = b

a >> b
a > > b

a << b
a < < b

a == b
a = = b

a != b
a ! = b

a += b
a + = b

a .. b
a . . b

a ..= b
a . . = b

a || b
a | | b

a && b
a & & b

The compact operator forms must use the canonical compound token where such a token exists.


---

52. Domain-Crossing Tests

The same precedence must be validated across domains.

Examples:

classical_value + quantum_parameter * scalar

tensor_a + tensor_b * tensor_c

signal_a & signal_b | signal_c

measurement == threshold && ready

start .. end

condition ? quantum_expression : classical_expression

The semantic layer decides whether the resulting expressions are legal.

The parser must not create separate precedence systems merely because domains differ.


---

53. POCO-REAF Contract

Expression precedence is part of the portable source-language definition.

A program's expression grouping must not change because it is compiled for:

tiny embedded hardware
CPU
multicore CPU
GPU
FPGA
ASIC
QPU
simulator
single machine
cluster
distributed system
cloud
edge
future hardware

Therefore:

a + b * c

always means structurally:

a + (b * c)

regardless of target.

This is essential to:

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever


---

54. No Hardware-Dependent Precedence

The grammar MUST NOT contain rules such as:

if gpu_available then ...
if qpu_available then ...
if vector_width == ...
if register_width == ...
if cpu_count == ...

Precedence is a language property, not a hardware property.


---

55. No Artificial Limits

This document MUST NOT introduce:

MAX_EXPRESSION_DEPTH
MAX_OPERATOR_CHAIN
MAX_CALL_CHAIN
MAX_ARGUMENTS
MAX_INDICES
MAX_TUPLE_ELEMENTS
MAX_RANGE_SIZE
MAX_TENSOR_RANK
MAX_QUBITS
MAX_THREADS
MAX_NODES

or equivalent constants.

Any implementation-level parser budget must be:

configurable;

explicitly documented as operational policy;

independent of language semantics;

incapable of changing the meaning of a successfully parsed program.



---

56. Integration Matrix

Component	Responsibility	Integration

grammar/lexer/operators.g4	operator lexing	supplies canonical operator tokens
grammar/lexer/operators.md	lexical operator contract	supplies canonical spellings/token identities
grammar/lexer/tokens.g4	shared vocabulary	supplies token vocabulary
grammar/expressions/expression.g4	expression hierarchy	implements this precedence contract
expression subgrammars	individual expression categories	implement assigned precedence level only
grammar/spec/syntax.md	normative syntax	references this precedence contract
grammar/specification/syntax.md	language specification	must agree with this document
grammar/grammar.md	implementation conformance	records implementation status
grammar/Zamani.g4	composition root	consumes canonical expression hierarchy
src/lexer.rs	lexical implementation	must emit canonical tokens
src/parser.rs	parser implementation	must produce this grouping
frontend AST	source structure	preserves grouping/source spans
semantic analysis	meaning	validates grouped expression
classical IR	classical lowering	consumes semantic expression
quantum::ir	quantum semantic boundary	receives semantically validated quantum operations
HDL/hardware IR	hardware lowering	receives semantic expression intent
optimizer	transformations	must preserve expression semantics
runtime	execution	must not alter source grouping



---

57. File Completion Contract

This file is considered complete only when all of the following are true:

Authority

[x] This file is the sole normative precedence table.

[x] Precedence is separated from lexical spelling.

[x] Precedence is separated from semantic meaning.

[x] Associativity is explicitly defined.

[x] Compatibility policy is defined.


Lexer integration

[x] Canonical token names are used.

[x] Alternate token vocabularies are prohibited.

[x] Maximal-munch responsibility remains with the lexer.

[x] Compound operators are not redefined here.


Parser integration

[x] Every precedence level is defined.

[x] Every associativity rule is defined.

[x] Parentheses override precedence.

[x] Chained non-associative constructs are explicitly handled.

[x] Postfix and prefix binding is explicit.

[x] Assignment is explicitly right associative.

[x] Conditional expressions are explicitly right associative.


AST integration

[x] Grouping must be preserved.

[x] Operand ordering must be preserved.

[x] Source spans must be preserved.

[x] No target-specific information is introduced.


Semantic integration

[x] Operator meaning remains downstream.

[x] Type checking remains downstream.

[x] Overload resolution remains downstream.

[x] Domain legality remains downstream.


IR integration

[x] No expression-specific IR is introduced.

[x] Classical lowering is downstream.

[x] quantum::ir remains the canonical quantum boundary.

[x] HDL/hardware lowering is downstream.


POCO-REAF

[x] No hardware limit is encoded.

[x] No fixed expression-depth limit is encoded.

[x] No fixed operand count is encoded.

[x] No fixed resource count is encoded.

[x] Precedence is target-independent.

[x] Parsing is deterministic.


Testing

[x] Positive tests are specified.

[x] Negative tests are specified.

[x] Boundary tests are specified.

[x] Associativity tests are specified.

[x] Maximal-munch integration tests are specified.

[x] Cross-domain tests are specified.

[x] Compatibility requirements are specified.



---

58. Required Downstream Conformance

This document does not claim that the entire expression subsystem is implemented merely because this file exists.

The repository must eventually prove:

precedence.md
    |
    v
expression grammar
    |
    v
ANTLR parser
    |
    v
Rust lexer/parser
    |
    v
frontend AST
    |
    v
structural validation
    |
    v
semantic validation
    |
    v
canonical IR
    |
    v
compiler
    |
    v
runtime

A precedence rule is production-ready only when that chain agrees.


---

59. Final Normative Rule

For every Zamani expression:

> The parser MUST apply this precedence and associativity contract deterministically and independently of semantic type, execution target, available resources, hardware topology, runtime state, or deployment environment.



The language therefore has one stable expression structure from:

atom

to:

arbitrarily large program

subject only to actual implementation/resource constraints and never to an artificial language-level hardware limit.

This document is the authoritative precedence contract for that behavior.

### One important repository correction

I deliberately **did not copy the precedence hierarchy from the earlier proposed file blindly**. The actual repository's operator specification uses `LESS`, `GREATER`, `EQUAL_EQUAL`, `LEFT_SHIFT`, `RIGHT_SHIFT`, `MODULO`, `LOGICAL_AND`, etc., and explicitly says those are the canonical lexer identities. 

Likewise, the existing `expression.g4` describes the expression hierarchy as:

`assignment → conditional → range → logical → bitwise → equality → relational → shift → additive → multiplicative → prefix → postfix → primary`, which this document formalizes rather than inventing a competing hierarchy. 

**Integration consequence:** after this file is added, the expression grammars should be checked against this contract and the canonical lexer vocabulary. In particular, the existing expression grammar's older names such as `EQ_EQ`, `NOT_EQ`, `SHIFT_LEFT`, etc. should not be allowed to become a second lexer vocabulary; the repository's `grammar/lexer/operators.md` already identifies that inconsistency as something that must be eliminated.