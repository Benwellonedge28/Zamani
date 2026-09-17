Zamani Expression Precedence and Associativity Specification

File: "grammar/expressions/precedence.md"
Status: Normative / Production
Specification role: Canonical expression precedence and associativity contract
Language: Zamani
Specification version: 1.0
Minimum implementation baseline: Rust 1.97 / Rust 1.97.1
Rust edition: 2021
Safety: Safe Rust only; "unsafe" Rust is prohibited
Primary architectural goal: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)

---

0. Purpose

This document is the single normative authority for Zamani expression precedence, associativity, and expression-composition boundaries.

It defines:

- the complete precedence hierarchy;
- operator grouping;
- associativity;
- prefix/postfix binding;
- assignment associativity;
- range composition;
- conditional-expression placement;
- call/index/member postfix composition;
- generic expression composition;
- delimiter boundaries;
- expression nesting;
- AST grouping requirements;
- semantic interpretation boundaries;
- canonical IR integration;
- quantum/classical/HDL/domain integration;
- scalability requirements;
- parser determinism;
- diagnostics requirements;
- compatibility requirements;
- conformance requirements.

This document does not implement parsing.

The executable grammar remains in:

grammar/expressions/expressions.g4

The modular expression grammars provide individual syntax components.

This document determines how those components compose.

There must be exactly one authoritative precedence hierarchy.

---

1. Architectural Position

The canonical pipeline is:

Zamani source
    │
    ▼
canonical lexer
    │
    ▼
canonical parser
    │
    ▼
grammar/expressions/expressions.g4
    │
    ▼
domain-neutral frontend AST
    │
    ▼
structural validation
    │
    ▼
semantic analysis
    │
    ├── type analysis
    ├── name resolution
    ├── overload resolution
    ├── effect analysis
    ├── capability analysis
    ├── resource analysis
    └── domain analysis
    │
    ▼
canonical semantic model / ZUIR
    │
    ├──────────────┬────────────────┐
    ▼              ▼                ▼
classical       quantum::ir      HDL/hardware
    │              │                │
    └──────────────┴────────────────┘
                   │
                   ▼
              optimization
                   │
          ┌────────┴────────┐
          ▼                 ▼
      routing           scheduling
          │                 │
          └────────┬────────┘
                   ▼
            resilience / QEC
                   │
                   ▼
                  ZQN
                   │
                   ▼
                  HAL
                   │
                   ▼
           target realization

The frontend AST is domain-neutral. The existing AST explicitly separates source expressions from semantic/IR/backend concepts and uses child "NodeId" references rather than embedding recursive backend structures.

"quantum::ir" remains the canonical quantum semantic boundary.

This precedence document must never introduce:

- a quantum expression IR;
- a quantum-specific precedence system;
- a hardware expression IR;
- a compiler-specific expression hierarchy;
- a backend-specific operator hierarchy.

---

2. Authority

The authority chain is:

grammar/spec/syntax.md
        │
        ▼
grammar/expressions/precedence.md
        │
        ▼
grammar/expressions/expressions.g4
        │
        ├── assignment.g4
        ├── conditionals.g4
        ├── ranges.g4 / range.g4
        ├── logical.g4
        ├── bitwise.g4
        ├── comparison.g4
        ├── arithmetic.g4
        ├── unary.g4
        ├── postfix.g4
        ├── calls.g4
        ├── indexing.g4
        ├── member-access.g4
        └── other expression components
        │
        ▼
frontend AST
        │
        ▼
semantic analysis
        │
        ▼
canonical IR

"Zamani.g4" is the parser composition root.

"grammar.md" is an implementation-conformance reference.

"Zamani-Grammar.md" is not allowed to silently change precedence.

No modular ".g4" file may define an alternative precedence hierarchy.

---

3. Single-Authority Rule

There shall be exactly one public expression entry point:

expression

There shall be exactly one canonical precedence hierarchy.

There shall be exactly one owner for every precedence level.

A modular file may define the syntax belonging to its level, but it must not redefine the entire expression hierarchy.

For example:

arithmetic.g4

may own arithmetic operators.

It must not define:

expression
assignmentExpression
conditionalExpression
logicalOrExpression
...

as a competing hierarchy.

Likewise:

conditionals.g4

owns conditional-expression syntax, but does not own the complete expression hierarchy.

The current "expressions.g4" already establishes itself as the composition layer and places "assignmentExpression" above "conditionalExpression".

That architecture is retained.

---

4. Canonical Precedence Hierarchy

From lowest binding strength to highest binding strength:

1.  Assignment
2.  Conditional
3.  Range
4.  Logical OR
5.  Logical AND
6.  Bitwise OR
7.  Bitwise XOR
8.  Bitwise AND
9.  Equality
10. Relational
11. Shift
12. Additive
13. Multiplicative
14. Prefix / unary
15. Postfix
16. Primary / atom

Conceptually:

expression
    │
    ▼
assignmentExpression
    │
    ▼
conditionalExpression
    │
    ▼
rangeExpression
    │
    ▼
logicalOrExpression
    │
    ▼
logicalAndExpression
    │
    ▼
bitwiseOrExpression
    │
    ▼
bitwiseXorExpression
    │
    ▼
bitwiseAndExpression
    │
    ▼
equalityExpression
    │
    ▼
relationalExpression
    │
    ▼
shiftExpression
    │
    ▼
additiveExpression
    │
    ▼
multiplicativeExpression
    │
    ▼
prefixExpression
    │
    ▼
postfixExpression
    │
    ▼
primaryExpression

Higher levels bind more tightly.

Lower levels bind less tightly.

Parentheses and other explicit delimiters may override natural precedence.

---

5. Complete Precedence Table

Level| Category| Representative operators/forms| Associativity
1| Assignment| "=", "+=", "-=", "*=", "/=", "%=", "&=", `| =", "^=`
2| Conditional| "if ... else", "? :"| Right for ternary
3| Range| "..", "..="| Non-chainable by default
4| Logical OR| `| 
5| Logical AND| "&&"| Left
6| Bitwise OR| "|"| Left
7| Bitwise XOR| "^"| Left
8| Bitwise AND| "&"| Left
9| Equality| "==", "!="| Non-associative
10| Relational| "<", ">", "<=", ">="| Non-associative
11| Shift| "<<", ">>"| Left
12| Additive| "+", "-"| Left
13| Multiplicative| "*", "/", "%"| Left
14| Prefix| unary "+", unary "-", logical "!", bitwise "~", other declared prefix forms| Right
15| Postfix| calls, indexing, member access, optional member access, postfix operators| Left/chained
16| Primary| literals, identifiers, grouped expressions, collection literals, domain-neutral extension atoms| N/A

This table is normative.

If another document or grammar file disagrees with this table, that artifact is inconsistent and must be corrected rather than creating a second interpretation.

---

6. Assignment

Assignment has the lowest precedence among ordinary Zamani expression operators.

Canonical forms include:

x = y
x += y
x -= y
x *= y
x /= y
x %= y
x &= y
x |= y
x ^= y

Assignment is right associative.

Therefore:

a = b = c

groups as:

a = (b = c)

not:

(a = b) = c

The grammar structure must therefore be equivalent to:

assignmentExpression
    : conditionalExpression
    | assignmentTarget assignmentOperator assignmentExpression
    ;

The existing expression grammar follows this right-recursive architecture.

6.1 Assignment target

The parser may recognize a syntactically broad assignment target.

Semantic analysis determines whether it is actually assignable.

For example:

x = 1
array[i] = value
object.field = value

may be syntactically accepted.

Whether the target is:

- mutable;
- writable;
- borrowed;
- linear;
- affine;
- distributed;
- hardware-backed;
- quantum-valid;

is a semantic question.

The precedence document must not encode those restrictions.

---

7. Conditional Expressions

Conditional expressions occupy the second-lowest expression level.

There must be exactly one conditional-expression boundary.

The canonical relationship is:

assignmentExpression
    │
    ▼
conditionalExpression
    │
    ▼
rangeExpression

"conditionals.g4" owns:

conditionalExpression
ifExpression
else-if branches
else branch
ternary conditional syntax

The existing repository already identifies "conditionals.g4" as the modular owner of value-producing conditional expressions.

7.1 Critical recursion rule

The conditional grammar must not define a ternary expression as:

expression QUESTION expression COLON expression

as its final production.

That form allows the conditional component to consume the complete expression hierarchy recursively and creates a competing precedence boundary.

Instead, the canonical conditional composition must behave as:

conditionalExpression
    : rangeExpression
      (
          QUESTION
          expression
          COLON
          conditionalExpression
      )?
    ;

or an exactly equivalent composition that preserves the following rules:

condition
    = range-level expression

then branch
    = complete expression

else branch
    = conditional expression

This gives ternary conditionals right associativity without introducing another global expression hierarchy.

For:

a ? b : c ? d : e

the required grouping is:

a ? b : (c ? d : e)

not:

(a ? b : c) ? d : e

---

8. Structured "if" Expressions

Structured conditional expressions are value-producing expressions when the language context permits them.

Example:

if condition {
    value_a
} else {
    value_b
}

The condition itself uses the canonical expression system.

The branch body uses the canonical block-expression system.

No separate:

booleanExpression
quantumCondition
hardwareCondition
resourceCondition

precedence hierarchy is introduced.

Semantic analysis determines whether the condition is valid.

---

9. Range Expressions

Range expressions are below logical, bitwise, comparison, and arithmetic operators.

Canonical operators:

..
..=

Examples:

0 .. n
0 ..= n
start .. end
start ..= end

The range grammar must not recursively invoke the complete "expression" rule as both operands.

Its operands must be constrained to the immediately higher precedence boundary.

Conceptually:

rangeExpression
    : logicalOrExpression
      (rangeOperator logicalOrExpression)?
    ;

or an equivalent structure established by the canonical range component.

This prevents uncontrolled recursion.

---

10. Range Non-Associativity

Ranges are not automatically chainable.

The following:

a .. b .. c

must not silently acquire an arbitrary grouping.

If chained ranges are eventually supported, that must be an explicit language feature with:

- a defined semantic meaning;
- a defined AST representation;
- a defined precedence contract;
- positive tests;
- negative tests;
- compatibility rules.

Until then, a second range operator at the same expression level should produce a syntax error.

This prevents accidental interpretation of malformed range expressions.

---

11. Logical OR

Logical OR:

||

is left associative.

a || b || c

groups as:

(a || b) || c

The parser establishes structure.

Whether evaluation is:

- short-circuiting;
- speculative;
- distributed;
- vectorized;
- quantum/classical hybrid;

is semantic/compiler behavior and does not change source precedence.

---

12. Logical AND

Logical AND:

&&

is left associative.

a && b && c

groups as:

(a && b) && c

AND binds more strongly than OR:

a || b && c

means:

a || (b && c)

---

13. Bitwise Precedence

The canonical order is:

bitwise OR
    <
bitwise XOR
    <
bitwise AND

Therefore:

a | b ^ c & d

groups as:

a | (b ^ (c & d))

The same hierarchy applies whether the operands are:

- scalar integers;
- arbitrary-width integers;
- vectors;
- bitsets;
- symbolic values;
- hardware signals;
- HDL values;
- data-parallel values.

The semantic layer determines whether an operation is legal.

The grammar does not impose a width limit.

---

14. Equality

Equality operators:

==
!=

are at the same precedence.

They are non-associative.

Therefore:

a == b

is valid.

But:

a == b == c

must not silently become:

(a == b) == c

unless a future language specification explicitly introduces chained equality semantics.

This is important because boolean equality chaining is often accidental and may hide a semantic error.

---

15. Relational Operators

Relational operators:

<
>
<=
>=

share one precedence level.

They are non-associative.

Therefore:

a < b

is valid.

But:

a < b < c

must not silently be interpreted as:

(a < b) < c

If mathematical chained comparisons are introduced later, they must receive an explicit semantic and AST contract.

---

16. Shift Operators

Canonical shifts:

<<
>>

are left associative.

a << b << c

groups as:

(a << b) << c

The grammar must not encode a machine-specific shift width.

For example, the language must not impose:

shift < 64

as a grammar rule.

The semantic/type system determines whether the operation is valid for the operand type.

---

17. Additive Operators

Canonical additive operators:

+
-

are left associative.

a - b - c

means:

(a - b) - c

Additive operators bind more weakly than multiplicative operators:

a + b * c

means:

a + (b * c)

Unary operators are a separate higher-precedence category.

---

18. Multiplicative Operators

Canonical multiplicative operators:

*
/
%

are left associative.

a / b / c

means:

(a / b) / c

Multiplicative expressions bind more strongly than additive expressions.

No machine word size is implied by the grammar.

"%" does not imply a fixed integer representation.

---

19. Prefix Operators

Prefix operators bind more strongly than multiplicative operators and less strongly than postfix operators.

Representative prefix forms include:

+x
-x
!x
~x

Prefix operators are right associative by nesting:

!!x

means:

!(!x)

and:

--x

must be interpreted according to the declared lexical/parser distinction between prefix decrement and other syntax.

The same token may participate in different syntactic roles where the grammar explicitly defines those roles.

For example:

-

may be:

prefix negation

or:

infix subtraction

depending on its syntactic position.

The AST records operator identity, while semantic analysis determines operator meaning.

---

20. Prefix vs. Infix Ambiguity

The parser must distinguish operator role structurally.

Example:

-a

contains prefix "-".

Example:

a - b

contains infix "-".

The precedence document therefore does not assign one global semantic meaning to the character "-".

It specifies the syntactic positions in which the operator can occur.

The frontend AST already deliberately represents operators as source-level identities rather than binding the AST to lexer implementation types.

---

21. Postfix Expressions

Postfix syntax binds more strongly than prefix operators.

Postfix constructs include:

call
index
member access
optional member access
declared postfix operators

Examples:

f(x)
a[i]
object.field
object?.field
value!

Postfix operations chain left-to-right:

a.b.c

means:

(a.b).c

and:

f(x)(y)

means:

(f(x))(y)

where such a call is semantically valid.

---

22. Call Expressions

Calls are postfix operations.

f(a, b)

groups the function expression first:

f

then applies the argument list.

Arguments themselves are complete expressions according to the function-call grammar's delimiter boundary.

Therefore:

f(a + b, c * d)

groups as:

f(
    (a + b),
    (c * d)
)

The number of arguments is not globally bounded.

The parser must not encode:

MAX_ARGUMENTS

or similar language-level limits.

---

23. Index Expressions

Indexing is postfix:

a[i]

and chains:

a[i][j]

as:

(a[i])[j]

An index expression may contain a complete expression where permitted:

a[i + j]
a[condition ? x : y]
a[f(x)]

The grammar does not impose a fixed number of dimensions.

---

24. Member Access

Member access:

object.member

binds as a postfix operation.

Chained member access is left associative:

a.b.c

means:

(a.b).c

The parser does not resolve:

- whether "a" is a module;
- whether "b" is a field;
- whether "c" is a method;
- whether the path is a hardware capability;
- whether the name is a quantum operation.

Those are semantic questions.

---

25. Optional Member Access

If the canonical lexer/parser supports:

?.

then:

a?.b

belongs to the postfix/member-access level.

It must not receive an independent precedence level.

The operator's semantic behavior belongs to the type/effect system.

---

26. Primary Expressions

Primary expressions are atomic expression forms.

They include, where supported by the language:

identifier
literal
grouped expression
tuple
array
map
function/lambda literal
block expression
domain-neutral extension atom

Primary expressions do not have an associativity.

They establish the starting operand for higher-level postfix and prefix composition.

---

27. Parenthesized Expressions

Parentheses explicitly override precedence.

Example:

(a + b) * c

must group as:

(a + b) * c

rather than:

a + (b * c)

Parentheses restart expression parsing at the complete expression boundary.

They therefore provide a deliberate escape from natural precedence.

---

28. Delimiter Boundaries

Delimiters terminate or constrain expression parsing.

Important delimiters include:

)
]
}
,
:
;

depending on syntactic context.

A precedence level must never consume a delimiter belonging to an enclosing construct.

For example, inside:

f(a + b, c)

the comma terminates the first argument.

Inside:

a ? b : c

the colon terminates the middle expression.

Inside:

[a, b, c]

commas delimit elements.

Delimiter ownership must therefore be explicit in every modular grammar component.

---

29. Conditional Delimiter Boundary

The ternary conditional is a particularly important delimiter case.

For:

condition ? then_value : else_value

the parser must ensure that the ":" belongs to the current conditional expression.

The "then_value" may be a complete expression.

The "else_value" enters the conditional-expression level recursively.

Therefore:

a ? b = c : d

may represent:

a ? (b = c) : d

where assignment in a conditional branch is semantically permitted.

The parser must not accidentally consume the ":" as part of an unrelated expression.

---

30. Precedence Does Not Determine Evaluation Order

Precedence determines syntactic grouping.

It does not automatically determine evaluation order.

For example:

a + b * c

establishes:

a + (b * c)

It does not by itself specify:

- whether "a" is evaluated before "b";
- whether "b" and "c" are evaluated sequentially;
- whether subexpressions are parallelized;
- whether an optimizer fuses operations;
- whether execution occurs on CPU/GPU/QPU/FPGA;
- whether a distributed runtime evaluates operands remotely.

Evaluation semantics belong to the semantic specification.

---

31. Precedence Does Not Determine Optimization

The parser must preserve the source grouping.

Later optimization may transform:

(a + b) + c

into an equivalent representation if semantic rules permit it.

Examples include:

- constant folding;
- algebraic simplification;
- vectorization;
- tensor fusion;
- quantum optimization;
- hardware lowering;
- distributed execution;
- common-subexpression elimination.

Such transformations must happen downstream.

The grammar must not perform optimization.

---

32. Precedence Does Not Determine Hardware

The expression:

a * b + c

does not mean:

CPU multiply
CPU add

It may eventually become:

CPU instructions
GPU operations
FPGA logic
tensor accelerator operations
distributed computation
quantum/classical hybrid computation

depending on semantic type, capabilities, compilation context, and target realization.

No hardware information belongs in precedence.

---

33. Quantum Integration

Quantum syntax uses the same universal expression precedence.

The expression grammar must not introduce a special quantum precedence hierarchy.

For example:

theta + phi

has the same additive precedence regardless of whether "theta" and "phi" are later interpreted as:

- classical numbers;
- symbolic parameters;
- quantum rotation parameters;
- tensor values;
- hardware-calibration parameters.

Likewise:

operation(theta)(q)

is structurally:

postfix(call)
    └── postfix(call)
          └── primary(operation)

Semantic analysis determines whether:

operation

represents a quantum operation.

The grammar must not enumerate:

X
Y
Z
H
CNOT
RX
RY
RZ
...

as a universal precedence construct.

The existing frontend AST explicitly rejects a closed "QuantumGate"-style vocabulary in favor of generic/extensible expressions.

---

34. Quantum Canonical Boundary

Expression parsing proceeds:

quantum source expression
        │
        ▼
domain-neutral AST
        │
        ▼
semantic quantum interpretation
        │
        ▼
quantum::ir

There must not be:

expression
    ↓
grammar quantum IR
    ↓
quantum::ir

or:

expression
    ↓
frontend QuantumGate IR
    ↓
quantum::ir

"quantum::ir" remains the canonical quantum semantic boundary.

---

35. Classical Integration

The same precedence rules apply to:

- scalar computation;
- arbitrary-width integers;
- floating-point computation;
- vectors;
- matrices;
- tensors;
- symbolic computation;
- numerical computation;
- signal processing;
- scientific computation;
- data processing.

The expression grammar does not need a separate precedence hierarchy for each numeric domain.

Type and semantic analysis determine the meaning.

---

36. HDL Integration

HDL expressions use the universal precedence contract unless an HDL-specific construct explicitly requires a different syntactic construct.

For example:

a & b | c

retains:

a & (b |? ...)

according to the canonical bitwise hierarchy defined here, rather than acquiring a hardware-specific parser precedence.

HDL semantics may interpret the resulting AST as:

- boolean logic;
- bit-vector logic;
- signal logic;
- hardware expression;
- synthesis expression.

The grammar does not choose a synthesis implementation.

---

37. AI / Tensor / Data Integration

Tensor and data expressions use the same expression hierarchy.

For example:

a + b * c

has the same source grouping regardless of whether "a", "b", and "c" are:

- scalars;
- vectors;
- matrices;
- tensors;
- distributed arrays;
- model values;
- symbolic expressions.

Shape compatibility, broadcasting, contraction, differentiation, accelerator selection, and memory placement belong downstream.

The precedence grammar must never encode fixed tensor dimensions or ranks.

---

38. Distributed Integration

Distributed expressions retain ordinary precedence.

For example:

a + b * c

does not encode:

node0
node1
node2

or a fixed cluster topology.

Distribution is represented through semantic resource/capability constructs and resolved later.

---

39. Effects

Expression precedence must remain independent of effects.

An expression may eventually have effects such as:

- I/O;
- mutation;
- allocation;
- asynchronous execution;
- communication;
- quantum measurement;
- hardware interaction.

Precedence only determines source grouping.

Effect analysis occurs after parsing.

---

40. Resource and Capability Integration

Expressions must not encode physical resource limits.

Forbidden as language-level precedence rules:

MAX_CPUS
MAX_GPUS
MAX_QUBITS
MAX_THREADS
MAX_NODES
MAX_MEMORY
MAX_TENSOR_RANK
MAX_REGISTER_WIDTH

Likewise, precedence must not depend on:

- discovered hardware;
- device count;
- available memory;
- current QPU topology;
- accelerator availability;
- runtime state.

Parsing must be target-independent.

---

41. POCO-REAF Requirement

The same source expression must retain the same syntactic grouping regardless of target.

For example:

a + b * c

always parses as:

a + (b * c)

whether eventually compiled for:

tiny embedded target
CPU
many-core system
GPU
FPGA
ASIC
QPU
distributed cluster
HPC system
future architecture

The target may alter implementation.

It must not alter source precedence.

---

42. No Artificial Scalability Limits

The grammar must not impose finite limits on:

- expression count;
- expression depth;
- operand count;
- function arguments;
- tuple elements;
- collection elements;
- index dimensions;
- member chains;
- call chains;
- nested parentheses;
- nested conditionals;
- range magnitude;
- integer width;
- tensor size;
- tensor rank;
- quantum register size;
- quantum operation count.

Repetition must be represented structurally.

Examples:

*
+
recursive composition

rather than:

operand1
operand2
...
operand16

"Infinity" means:

«no artificial universal finite language limit is imposed by this precedence specification.»

Actual compiler resource limits may exist as implementation policy.

Such limits must not change language semantics.

---

43. Compiler Resource Limits

An implementation may have configurable safety/resource policies such as:

maximum parser memory
maximum diagnostic count
maximum compilation budget
maximum recursion budget
maximum source size
maximum AST nodes

Those are implementation policies, not language precedence.

They must produce resource-policy diagnostics rather than silently redefining the grammar.

For example:

program exceeds configured compiler memory budget

is fundamentally different from:

program is syntactically invalid because it has too many expressions

---

44. Deep Expression Nesting

The language imposes no artificial expression-depth limit.

However, the compiler implementation must avoid unnecessary stack-overflow hazards.

Where practical, implementation traversal should use:

- explicit work stacks;
- iterative AST traversal;
- bounded diagnostic queues;
- resource-policy controls.

The frontend AST already provides non-recursive child enumeration so whole-AST traversal can be performed by an external graph walker rather than recursively traversing the Rust call stack.

This is important for scalability.

---

45. AST Contract

The precedence parser must preserve grouping in the AST.

For:

a + b * c

the AST must represent:

Binary(+)
├── a
└── Binary(*)
    ├── b
    └── c

not:

Binary(*)
├── Binary(+)
│   ├── a
│   └── b
└── c

The AST must preserve:

- operator identity;
- operand order;
- grouping;
- source span;
- child order;
- nesting;
- delimiter boundaries;
- syntactic form where semantically relevant.

The existing expression AST is explicitly a source-level representation and maps unary, binary, and conditional constructs to canonical node kinds.

---

46. AST Does Not Own Precedence

The AST does not need to store a numeric precedence value merely because the parser used precedence.

Precedence is a property of source syntax.

The resulting AST already contains grouping.

For example:

a + b * c

does not require:

Binary {
    precedence: 12
}

unless a separate AST metadata contract explicitly requires it.

The AST should retain semantic operator identity rather than parser implementation details.

---

47. Operator Identity

Operators must not be permanently tied to lexer implementation types in the domain-neutral AST.

The existing AST architecture intentionally uses source-level operator identities rather than depending directly on lexer token implementation types.

Therefore:

lexer token
    ↓
parser operator identity
    ↓
AST OperatorRef
    ↓
semantic operator resolution

is preferred.

This keeps lexer evolution from forcing AST redesign.

---

48. Semantic Contract

After parsing, semantic analysis determines:

- operator validity;
- operand types;
- result type;
- conversions;
- overload resolution;
- generic inference;
- ownership;
- borrowing;
- effects;
- capabilities;
- resources;
- domain meaning.

For example:

a + b

does not intrinsically mean integer addition.

It may represent a semantic operation over:

- integers;
- floats;
- vectors;
- matrices;
- tensors;
- symbolic expressions;
- user-defined types;
- hardware values;
- future domains.

The grammar establishes only syntactic structure.

---

49. Operator Overloading

Operator overloading does not change precedence.

If multiple semantic operations correspond to:

+

they all share the same syntactic precedence.

Overload resolution happens later.

For example:

a + b * c

always groups:

a + (b * c)

even if:

+

and:

*

resolve to user-defined operations.

---

50. Generic Operators

Generic/type-level constructs must not silently introduce another expression hierarchy.

For example:

T::member

remains path/member syntax according to the canonical postfix/path contracts.

Generic arguments must be parsed using their own explicit delimiter boundary.

No generic syntax may modify ordinary arithmetic precedence.

---

51. Macros

Macro syntax may generate expressions.

However, macro expansion must not create an alternative precedence interpretation of already parsed source.

The architecture is:

source
    ↓
parse macro syntax
    ↓
macro expansion according to macro contract
    ↓
parse/validate resulting syntax where required
    ↓
AST

Macros must not mutate the canonical precedence table.

---

52. Metaprogramming

Compile-time or metaprogramming expressions use the same precedence system unless explicitly declared as syntax-level metaprogramming constructs.

Metaprogramming must not be allowed to redefine:

+
*
==
&&
||
=

precedence globally.

A dialect may introduce syntax only through the approved dialect extension mechanism.

---

53. Dialects

A dialect may add expression syntax only if it declares:

- dialect name;
- version;
- operator/token identity;
- precedence level;
- associativity;
- prefix/infix/postfix role;
- delimiter behavior;
- AST mapping;
- semantic mapping;
- compatibility behavior;
- diagnostics;
- feature gate.

A dialect must not silently alter the precedence of an existing stable operator.

A dialect-specific operator should preferably occupy an explicitly reserved extension range or use a namespaced/non-conflicting syntactic form.

---

54. Future Operators

New operators must not be added merely by editing a random ".g4" file.

The required process is:

operator proposal
    ↓
semantic definition
    ↓
precedence decision
    ↓
associativity decision
    ↓
lexical contract
    ↓
AST contract
    ↓
semantic contract
    ↓
IR contract
    ↓
compiler/runtime consumers
    ↓
positive tests
    ↓
negative tests
    ↓
boundary tests
    ↓
scalability tests
    ↓
compatibility review
    ↓
canonical grammar

Only after that process may the operator become stable language syntax.

---

55. Operator Addition Checklist

Every new operator must answer:

Name:
Spelling:
Token:
Prefix/Infix/Postfix:
Precedence:
Associativity:
Operands:
Result:
Delimiter interaction:
Lexer ownership:
AST representation:
Semantic meaning:
Type-system interaction:
Effect interaction:
Resource interaction:
Quantum interaction:
HDL interaction:
IR mapping:
Compiler consumers:
Runtime consumers:
Diagnostics:
Positive tests:
Negative tests:
Boundary tests:
Scalability tests:
Compatibility:
Hard-coding audit:

If any item is unresolved, the operator is not production-ready.

---

56. Existing Modular Grammar Integration

The following ownership must remain clear.

File| Responsibility
"expressions.g4"| canonical composition and public expression entry
"assignment.g4"| assignment syntax
"conditionals.g4"| conditional-expression syntax
"ranges.g4" / "range.g4"| range syntax
"logical.g4"| logical syntax
"bitwise.g4"| bitwise syntax
"comparison.g4"| equality/relational syntax
"arithmetic.g4"| arithmetic syntax
"unary.g4"| prefix syntax
"postfix.g4"| postfix composition
"calls.g4"| calls
"indexing.g4"| indexing
"member-access.g4"| member access
"literals.g4"| literal expressions
"tuples.g4"| tuple expressions
"arrays.g4"| array expressions
"maps.g4"| map expressions
"blocks.g4"| expression blocks
"async.g4"| async-specific expression forms
"match.g4"| match expression forms where applicable
"metaprogramming.g4"| metaprogramming expressions

The current repository contains both "range.g4" and "ranges.g4". They must not become competing precedence authorities. The canonical range ownership must be selected by the existing repository composition and compatibility contract; whichever file is retained as the executable owner must implement this document's range contract. The other must not independently define the same public hierarchy.

---

57. "conditional-expressions.g4"

"grammar/expressions/conditional-expressions.g4" must not be a second owner of conditional precedence.

The repository has already established "conditionals.g4" as the modular conditional-expression authority.

If "conditional-expressions.g4" is unused, it should remain deleted rather than being reintroduced as a competing grammar.

If retained temporarily for migration history, it must contain no competing parser rule.

---

58. "expressions.g4" Integration

"grammar/expressions/expressions.g4" must remain the composition layer.

Its hierarchy must correspond exactly to this document:

expression
    → assignmentExpression
    → conditionalExpression
    → rangeExpression
    → logicalOrExpression
    → logicalAndExpression
    → bitwiseOrExpression
    → bitwiseXorExpression
    → bitwiseAndExpression
    → equalityExpression
    → relationalExpression
    → shiftExpression
    → additiveExpression
    → multiplicativeExpression
    → prefixExpression
    → postfixExpression
    → primaryExpression

The existing file already documents this hierarchy and identifies itself as the canonical expression-composition layer.

No other file may replace this hierarchy.

---

59. "grammar/spec/syntax.md" Integration

"grammar/spec/syntax.md" remains the broader normative syntax specification.

This document specializes its expression section.

Therefore:

grammar/spec/syntax.md

defines the overall syntax model.

This file defines the detailed expression precedence contract.

If a conflict exists:

1. resolve it at the specification level;
2. update this document;
3. update canonical grammar;
4. update tests;
5. update implementation-conformance documentation.

Do not allow the grammar implementation to silently decide the language.

The current syntax specification already identifies syntactic precedence and associativity as normative concerns and places the syntax between lexical analysis and AST/semantic processing.

---

60. Lexer Integration

This document consumes canonical lexer tokens.

It does not define tokens.

The lexer owns:

- operator spelling;
- token identity;
- comments;
- whitespace;
- identifiers;
- literals;
- Unicode lexical rules.

The parser owns structural composition.

A parser rule must not invent a token alias because a desired spelling is missing from the lexer.

If a token is missing:

lexer contract
    ↓
token addition
    ↓
lexer tests
    ↓
parser integration

must occur explicitly.

---

61. Rust Parser Integration

The Rust frontend implementation must consume the canonical grammar contract.

Rust implementation requirements:

Rust 1.97
Rust 1.97.1
Edition 2021
stable
no unsafe

The parser implementation must not duplicate precedence in multiple independent locations.

If a hand-written parser path exists alongside ANTLR-derived parsing, both must use the same precedence contract.

There must not be:

ANTLR precedence ≠ Rust parser precedence

for the same language version.

---

62. No "unsafe"

Neither the grammar nor the corresponding Rust implementation may require "unsafe".

Expression parsing must be implementable entirely using safe Rust.

No:

unsafe { ... }

is permitted.

The existing frontend AST explicitly declares safe-Rust/no-unsafe requirements.

---

63. Determinism

Given:

source text
+
language version
+
enabled syntax/dialect configuration

expression parsing must be deterministic.

It must not depend on:

- current time;
- randomness;
- filesystem state;
- environment variables;
- network state;
- hardware discovery;
- available GPU;
- available QPU;
- scheduler state;
- runtime state.

The same source must produce the same syntactic grouping.

---

64. Security

Expression parsing is non-executing.

Parsing:

system.run(command)

must not execute "command".

Parsing must not:

- access files;
- access secrets;
- access credentials;
- access hardware;
- contact networks;
- execute subprocesses;
- invoke a QPU;
- invoke an FPGA;
- invoke an HDL simulator;
- invoke a backend;
- perform resource allocation.

All execution belongs downstream.

---

65. Error Handling

The parser must produce deterministic diagnostics for:

- unexpected operator;
- missing operand;
- invalid operator placement;
- malformed assignment;
- malformed conditional;
- malformed range;
- invalid chained equality;
- invalid chained relational operation;
- missing closing delimiter;
- unexpected delimiter;
- missing ternary colon;
- incomplete postfix expression;
- invalid prefix expression.

Diagnostics must identify:

- source span;
- offending token;
- expected syntactic category;
- language version where relevant.

Diagnostics must not execute semantic operations.

---

66. Recovery

Parser recovery must not change the canonical precedence model.

Recovery may:

- synchronize at delimiters;
- recover at statement boundaries;
- recover at commas;
- recover at closing brackets;
- recover at closing braces;
- recover at semicolons.

Recovered ASTs must be explicitly marked as structurally incomplete/invalid where required.

A recovery node must not be mistaken for a valid semantic expression.

---

67. Ambiguity Policy

The canonical precedence table must resolve all ordinary operator-grouping ambiguity.

An expression is invalid if it requires an undefined associativity decision.

Examples:

a == b == c

must not receive an accidental left grouping.

Likewise:

a .. b .. c

must not receive an accidental grouping.

Ambiguity must be resolved by specification, not by whichever parser implementation happens to accept the source.

---

68. Parentheses as Explicit Disambiguation

Whenever natural precedence is insufficient or readability requires it, programmers may use parentheses.

Example:

a + (b * c)

and:

(a + b) * c

must produce different AST groupings where the underlying operations are otherwise valid.

Parentheses therefore provide a stable source-level mechanism independent of target architecture.

---

69. Example Grouping Matrix

Arithmetic

a + b * c

→

a + (b * c)

Shift

a + b << c

→

(a + b) << c

because additive binds more strongly than shift.

Comparison

a + b < c * d

→

(a + b) < (c * d)

Logical

a || b && c

→

a || (b && c)

Bitwise

a | b ^ c & d

→

a | (b ^ (c & d))

Assignment

a = b + c * d

→

a = (b + (c * d))

Ternary

a ? b : c ? d : e

→

a ? b : (c ? d : e)

Postfix

a.b[i](x)

→

(((a.b)[i])(x))

subject to semantic validity.

---

70. Expression Completeness

The precedence system must be closed over all expression forms.

Every expression-producing grammar construct must have an explicitly documented position in the hierarchy.

No new expression form may be introduced as:

someSpecialExpression

without answering:

Where does it bind?
What does it bind to?
What binds to it?
Is it prefix/infix/postfix/mixfix?
What is its associativity?
What delimiters terminate it?
What is its AST mapping?

---

71. Mixfix Expressions

Mixfix constructs such as:

condition ? a : b

must have explicit delimiter and binding contracts.

The same principle applies to future syntax such as:

match ...
await ...
yield ...
co_await ...

if they are expression-producing constructs.

They must not silently consume arbitrary expression levels.

---

72. Async Integration

Async expression constructs must use this precedence contract.

An async construct must explicitly declare whether it behaves as:

- prefix;
- postfix;
- primary;
- structured expression.

It must not create a parallel expression hierarchy.

The existing repository's "async.g4" already expects canonical expression composition to determine where async expression syntax enters the hierarchy.

---

73. Match Integration

A match expression, if enabled, is a structured primary/expression construct.

Its arms may contain complete expressions.

Pattern syntax does not redefine ordinary expression precedence.

The expression:

match value {
    pattern => expression
}

must preserve the "=>" delimiter as belonging to the match arm.

---

74. Blocks as Expressions

A block expression behaves as a primary-level expression.

Therefore:

f({
    a + b
})

must parse the block as one argument expression where the surrounding grammar permits it.

The block grammar owns block structure.

Precedence does not duplicate block grammar.

---

75. Collections

Collection literals such as:

[a + b, c * d]

contain complete element expressions.

The comma is the element delimiter.

Collection size is unbounded by the language.

No precedence rule may impose a fixed collection cardinality.

---

76. Function Arguments

Function argument expressions are complete expressions bounded by commas and the closing delimiter.

For:

f(
    a + b,
    c * d,
    condition ? x : y
)

each argument is independently parsed using the canonical precedence hierarchy.

---

77. Index Expressions and Ranges

An index may contain a range expression where the relevant indexing grammar permits it:

array[start .. end]

or:

array[start ..= end]

The indexing grammar owns the bracket boundary.

The range grammar owns the range operator.

Precedence remains centralized here.

---

78. No Machine-Specific Operators

The core precedence specification must not add operators solely because a particular:

- CPU;
- GPU;
- FPGA;
- QPU;
- accelerator;
- network device;
- vendor SDK

supports them.

Target-specific operations belong to semantic capabilities, libraries, dialects, or interoperability mechanisms.

If a future target introduces a genuinely language-level operator, it must go through the operator-addition process.

---

79. No Quantum Hardware Precedence

There must never be a precedence table such as:

QPU gate precedence
physical qubit precedence
pulse precedence
calibration precedence
routing precedence

at the source expression layer.

Those are downstream concepts.

---

80. No HDL Timing Precedence

Clocking, timing, synthesis, and physical constraints must not silently alter ordinary expression precedence.

An HDL-specific timing construct must define its own syntactic construct while retaining the universal expression hierarchy for embedded expressions.

---

81. No Resource-Availability Precedence

The parser must not inspect resources to determine precedence.

For example, the precedence of:

a + b * c

must not change depending on whether:

- a GPU exists;
- a QPU exists;
- memory is available;
- an FPGA is connected;
- a cluster is reachable.

---

82. Canonical IR Integration

The parser produces source-level AST grouping.

Semantic analysis converts this into the canonical semantic representation.

Expressions then lower into the appropriate canonical/domain IR.

For quantum:

expression
    ↓
AST
    ↓
semantic quantum interpretation
    ↓
quantum::ir

For classical:

expression
    ↓
AST
    ↓
semantic classical interpretation
    ↓
classical IR

For hardware:

expression
    ↓
AST
    ↓
semantic hardware interpretation
    ↓
HDL/hardware IR

The precedence specification does not own those IRs.

---

83. Optimization Boundary

Optimization may transform expression structure only after semantic correctness has been established.

Examples:

(a + b) + c

may be optimized according to the language's algebraic/ordering rules.

But the optimizer must not reinterpret the parser's original grouping arbitrarily when observable semantics would change.

This is especially important for:

- floating-point arithmetic;
- effects;
- overflow semantics;
- concurrency;
- quantum operations;
- measurement;
- hardware timing;
- volatile operations.

---

84. Quantum Optimization Boundary

Quantum optimization may transform:

operation_a
operation_b

into another equivalent quantum representation.

However, the expression parser does not perform:

- gate decomposition;
- routing;
- scheduling;
- QEC;
- pulse generation;
- calibration;
- topology mapping.

Those remain downstream.

---

85. Runtime Boundary

Runtime behavior cannot alter parsed grouping.

The runtime may choose implementation strategies based on:

- target capabilities;
- resource availability;
- runtime state;
- scheduling;
- resilience;
- deployment.

But the source expression's AST grouping remains fixed.

---

86. Compatibility

Changing precedence is a language compatibility change.

It is not an implementation-only change.

For example, changing:

a + b * c

from:

a + (b * c)

to:

(a + b) * c

would be a breaking language change.

Any precedence modification requires:

1. specification update;
2. compatibility assessment;
3. migration guidance;
4. parser update;
5. AST tests;
6. semantic tests;
7. compiler tests;
8. documentation update.

---

87. Language Versioning

A stable language version must have one deterministic precedence table.

If a future language version changes precedence, the parser must know the active language version before parsing.

Precedence must never depend on:

target machine
compiler optimization level
runtime
hardware

---

88. Compatibility Test Requirements

Every stable precedence level must have tests for:

Single operator

a OP b

Repeated operator

a OP b OP c

Neighboring precedence

a lower b higher c

Reverse neighboring precedence

a higher b lower c

Parentheses

(a OP b) OP c
a OP (b OP c)

Nested operators

a OP1 b OP2 c OP3 d

Delimiters

f(...)
a[...]
(...)
{...}

Composition

assignment
conditional
range
call
index
member

---

89. Negative Tests

The test suite must reject malformed expressions such as:

a +
* b
a ==
b
a < b < c
a == b == c
a .. b .. c
a ? b
a ? b c
a ? b : 
= a
a +=

where the relevant forms are not otherwise valid.

Negative tests must confirm that invalid forms do not accidentally become valid through precedence recursion.

---

90. Boundary Tests

Boundary tests must include:

a
-a
+a
!a
~a
a!
a.b
a[i]
a()
a.b[i](x)
a + b
a * b
a << b
a < b
a == b
a && b
a || b
a .. b
a ? b : c
a = b

and combinations of adjacent levels.

---

91. Scalability Tests

The test suite must verify that the grammar has no artificial universal limits.

Tests should generate increasingly large expressions such as:

a + b + c + ...

and deeply nested structures:

((((a))))

and:

f(g(h(i(...))))

subject only to configured compiler resource policies.

Tests must distinguish:

language invalid

from:

implementation resource exhausted

---

92. Quantum Scalability Tests

The expression layer must be tested with symbolic collections of varying size:

q
q[i]
operation(theta)(q)
operation(theta)(q0, q1, ..., qn)

No grammar test may establish a universal qubit count.

The grammar must remain valid as the number of quantum operands grows.

---

93. Tensor/Data Scalability Tests

Expressions must support arbitrarily represented tensor/index structures without grammar-level dimensions:

tensor[i]
tensor[i, j]
tensor[i, j, k]
...

No fixed:

2D
3D
4D
8D
16D

grammar ceiling is permitted.

---

94. Distributed Scalability Tests

Expressions referring to distributed abstractions must not require:

node0
node1
...
nodeN

as parser-level constructs.

The number of nodes is a resource/deployment concern.

---

95. HDL Scalability Tests

Parameterized HDL expressions must not encode universal fixed widths.

For example, the grammar must permit semantic constructs corresponding to:

signal[width]

where "width" is program/type information rather than a grammar maximum.

The grammar must not reject a valid width merely because it exceeds a historical machine width.

---

96. Determinism Tests

The same source and language version must always produce the same grouping.

Test:

a + b * c

repeatedly and verify identical AST structure.

Do not involve:

- current time;
- randomness;
- target discovery;
- filesystem;
- network;
- hardware.

---

97. AST Golden Tests

For every precedence boundary, maintain AST golden tests.

Example:

a + b * c

expected conceptual AST:

Binary(+)
├── Identifier(a)
└── Binary(*)
    ├── Identifier(b)
    └── Identifier(c)

For:

a = b ? c : d

expected conceptual structure:

Assignment
├── a
└── Conditional
    ├── b
    ├── c
    └── d

Exact Rust node types remain owned by the AST implementation.

---

98. Source Span Preservation

Every grouped expression must retain a source span covering the complete expression.

For:

a + b * c

the outer expression span covers the entire source range.

The inner multiplication span covers:

b * c

Source spans must not be recomputed from semantic IR.

They originate at the frontend.

---

99. Diagnostics and Precedence

Diagnostics must report the actual source construct.

For:

a < b < c

the parser/semantic pipeline should identify that chaining is not permitted under the current precedence/associativity contract.

It must not report an unrelated hardware, quantum, or type-system error before syntax is structurally established.

---

100. Performance Contract

Expression parsing should be linear in the number of consumed expression tokens under normal operation.

The grammar must avoid unnecessary ambiguous alternatives that cause excessive parser prediction.

The expression hierarchy must be deterministic.

The implementation must avoid:

- unbounded speculative parsing;
- repeated reparsing of the same source region;
- semantic execution during parsing;
- hardware queries during parsing.

---

101. Memory Contract

The language imposes no universal expression-memory limit.

The compiler may apply explicit resource policy.

The parser/AST implementation should prefer scalable representations.

The existing AST uses "NodeId" references and dynamic collections rather than recursive concrete expression embedding, supporting scalable graph representation.

---

102. No Recursive AST Explosion

The grammar may naturally be recursive.

The Rust AST must not therefore require recursive Rust object ownership for every expression.

The canonical AST graph should continue to use:

NodeId

references.

Traversal can then be performed iteratively.

This supports very deep expressions subject to explicit compiler resource policy rather than accidental Rust call-stack limitations.

---

103. Feature Completion Contract

An expression feature is complete only when:

LEXER
  ↓
GRAMMAR
  ↓
PRECEDENCE
  ↓
AST
  ↓
STRUCTURAL VALIDATION
  ↓
SEMANTICS
  ↓
DIAGNOSTICS
  ↓
CANONICAL IR
  ↓
COMPILER
  ↓
RUNTIME/TARGET
  ↓
POSITIVE TESTS
  ↓
NEGATIVE TESTS
  ↓
BOUNDARY TESTS
  ↓
SCALABILITY TESTS
  ↓
DETERMINISM TESTS
  ↓
COMPATIBILITY TESTS

A ".g4" rule alone is not feature completion.

---

104. File Contract

File

grammar/expressions/precedence.md

Purpose

Canonical precedence and associativity specification.

Owns

- precedence levels;
- associativity;
- grouping;
- expression composition boundaries;
- precedence integration requirements.

Does not own

- lexer rules;
- AST Rust structures;
- semantic meaning;
- type checking;
- quantum IR;
- classical IR;
- HDL IR;
- optimization;
- routing;
- scheduling;
- QEC;
- ZQN;
- HAL;
- runtime.

Inputs

- language syntax specification;
- lexical token contract;
- modular expression grammar contracts;
- compatibility/version rules.

Outputs

- one canonical precedence hierarchy;
- operator grouping contract;
- associativity contract;
- integration constraints.

Upstream Contracts

grammar/DESIGN.md
grammar/spec/syntax.md
grammar/spec/lexical.md
grammar/spec/type-system.md
grammar/spec/semantics.md

Downstream Consumers

grammar/expressions/expressions.g4
grammar/expressions/*.g4
grammar/Zamani.g4
src/lexer.rs
src/parser.rs
src/frontend/ast/
semantic analysis
ZUIR
quantum::ir
classical IR
HDL/hardware IR
compiler
runtime
tests

Public Grammar Contract

Exactly one:

expression

entry point.

Exactly one canonical hierarchy.

AST Contract

Grouping must be represented structurally through the domain-neutral AST.

No precedence numbers need to survive into the AST unless explicitly required by another contract.

Semantic Contract

Precedence establishes syntax.

Semantic analysis establishes meaning.

IR Integration

No IR is defined here.

All expression semantics lower through the canonical semantic model and then appropriate domain IR.

Compiler Integration

Compiler passes consume the semantically validated grouping.

They may optimize only where language semantics permit.

Runtime Integration

Runtime behavior must not alter source grouping.

Tooling Integration

Formatting, syntax highlighting, diagnostics, IDE parsing, and refactoring tools must use this precedence contract.

Cross-Domain Integration

The same precedence model applies to:

- classical;
- quantum;
- hybrid;
- HDL;
- hardware/software co-design;
- AI/ML;
- data;
- distributed;
- networking;
- security;
- embedded;
- accelerator;
- future domains.

---

105. Hard-Coding Audit

This document must contain no:

MAX_QUBITS
MAX_CPUS
MAX_CORES
MAX_THREADS
MAX_GPUS
MAX_FPGAS
MAX_NODES
MAX_MEMORY
MAX_REGISTERS
MAX_TENSOR_RANK
MAX_VECTOR_WIDTH
MAX_ARGUMENTS
MAX_OPERANDS
MAX_EXPRESSION_DEPTH
MAX_CIRCUIT_DEPTH

as language semantics.

No physical device IDs.

No vendor IDs.

No fixed topology.

No fixed hardware width.

No fixed resource count.

No machine-specific precedence.

---

106. Compatibility Audit

The following must agree:

grammar/spec/syntax.md
grammar/expressions/precedence.md
grammar/expressions/expressions.g4
grammar/expressions/assignment.g4
grammar/expressions/conditionals.g4
grammar/expressions/ranges.g4
grammar/expressions/range.g4
grammar/expressions/logical.g4
grammar/expressions/bitwise.g4
grammar/expressions/comparison.g4
grammar/expressions/arithmetic.g4
grammar/expressions/unary.g4
grammar/expressions/postfix.g4
src/lexer.rs
src/parser.rs
src/frontend/ast/
grammar/tests/

Any disagreement must be resolved explicitly.

There must never be two different answers to:

«What does "a + b * c" mean syntactically?»

---

107. Repository Integration Rule

Before marking this file complete, verify:

[ ] expression.g4 follows this hierarchy
[ ] assignment.g4 follows assignment precedence
[ ] conditionals.g4 has one conditional owner
[ ] ternary recursion does not create a competing expression hierarchy
[ ] range.g4/ranges.g4 do not compete
[ ] logical.g4 follows logical precedence
[ ] bitwise.g4 follows bitwise precedence
[ ] comparison.g4 follows equality/relational precedence
[ ] arithmetic.g4 follows additive/multiplicative precedence
[ ] unary.g4 follows prefix precedence
[ ] postfix.g4 follows postfix precedence
[ ] calls.g4 follows postfix precedence
[ ] indexing.g4 follows postfix precedence
[ ] member-access.g4 follows postfix precedence
[ ] lexer tokens match parser tokens
[ ] AST grouping matches parser grouping
[ ] semantic analysis consumes generic operator identity
[ ] quantum semantics lower to quantum::ir
[ ] no quantum-specific precedence exists
[ ] no hardware-specific precedence exists
[ ] no fixed resource limits exist
[ ] Rust implementation is compatible with 1.97/1.97.1
[ ] no unsafe Rust is required
[ ] positive tests exist
[ ] negative tests exist
[ ] boundary tests exist
[ ] scalability tests exist
[ ] determinism tests exist
[ ] compatibility tests exist

---

108. Completion Criteria

This file is production complete only when all of the following are true:

1. There is exactly one canonical precedence hierarchy.
2. "expression" has exactly one public entry point.
3. Assignment is right associative.
4. Conditional expressions have exactly one composition boundary.
5. Ternary conditional parsing is right associative.
6. Range syntax has a defined precedence boundary.
7. Logical precedence is defined.
8. Bitwise precedence is defined.
9. Equality is non-associative.
10. Relational comparison is non-associative.
11. Shift precedence is defined.
12. Additive precedence is defined.
13. Multiplicative precedence is defined.
14. Prefix precedence is defined.
15. Postfix precedence is defined.
16. Primary expressions are defined.
17. Delimiter ownership is explicit.
18. Parentheses override precedence correctly.
19. AST grouping is deterministic.
20. Source spans are preserved.
21. Semantic interpretation remains downstream.
22. No IR is defined here.
23. "quantum::ir" remains the canonical quantum semantic boundary.
24. No quantum gate enumeration is introduced.
25. Classical, quantum, HDL, AI, data, distributed, and future domains use the universal expression system.
26. No machine-size limits are encoded.
27. No hardware identifiers are encoded.
28. No runtime state affects parsing.
29. No filesystem/network/hardware access occurs during parsing.
30. Rust 1.97/1.97.1 compatibility is maintained.
31. "unsafe" Rust is not required or permitted.
32. Positive tests exist.
33. Negative tests exist.
34. Boundary tests exist.
35. Scalability tests exist.
36. Determinism tests exist.
37. Compatibility tests exist.
38. Every modular grammar file has a defined ownership boundary.
39. No competing precedence authority remains.
40. "grammar/grammar.md" can describe the implementation without becoming a second authority.
41. "Zamani-Grammar.md" cannot silently introduce a conflicting precedence rule.
42. The precedence contract can be consumed by future computational domains without modifying the fundamental hierarchy.
43. A valid program's precedence does not change when compiled for a different target.
44. Resource availability cannot change parsing.
45. Target hardware cannot change parsing.
46. The file can therefore be treated as a closed contract by downstream implementers.

---

109. Canonical Summary

The final Zamani expression model is:

LOWEST
│
├── assignment              right
├── conditional             right for ternary
├── range                   non-associative
├── logical OR              left
├── logical AND             left
├── bitwise OR              left
├── bitwise XOR             left
├── bitwise AND             left
├── equality                non-associative
├── relational              non-associative
├── shift                   left
├── additive                left
├── multiplicative          left
├── prefix                  right
├── postfix                 left/chained
└── primary                 atomic
│
HIGHEST

The architectural rule is:

precedence
    ↓
syntax grouping
    ↓
domain-neutral AST
    ↓
semantic interpretation
    ↓
canonical semantic model
    ↓
domain IR
    ↓
optimization
    ↓
routing / scheduling / resilience
    ↓
QEC / ZQN where applicable
    ↓
HAL
    ↓
target realization

Therefore the precedence layer remains:

portable, deterministic, target-independent, resource-independent, domain-neutral, extensible, and free of artificial finite machine limits.

That is the required expression-level foundation for POCO-REAF.