Zamani Grammar — Left-Recursion Validation and Prevention

File: "grammar/validation/left-recursion.md"
Status: Normative production specification
Language: Zamani
Grammar technology: ANTLR4-compatible grammar architecture + Rust parser implementation
Rust baseline: Rust 1.97 / Rust 1.97.1
Rust edition: 2021
Safety policy: Production compiler implementation MUST use safe Rust; Rust "unsafe" MUST NOT be used
Scope: Entire "grammar/" tree, canonical ANTLR grammar, imported grammar components, generated grammar artifacts, Rust parser conformance, AST integration, and validation tooling
Primary objectives: Deterministic parsing, termination, scalability, maintainability, compatibility, and POCO-REAF

---

1. Purpose

This document defines the normative production policy for detecting, preventing, classifying, documenting, testing, and resolving left recursion in the Zamani grammar.

The objective is not merely to make ANTLR stop reporting a grammar warning.

The objective is to guarantee that every canonical Zamani grammar has:

- deterministic parser behavior;
- guaranteed parser progress;
- no unintended infinite-recursion path;
- no hidden recursion introduced through imports;
- no nullable-prefix recursion cycle;
- no precedence cycle;
- no accidental recursion introduced by domain extensions;
- no duplicated expression hierarchy;
- no parser behavior dependent on target hardware;
- no artificial language-level scalability limits.

The policy applies equally to:

- classical syntax;
- quantum syntax;
- hybrid syntax;
- HDL;
- hardware intent;
- distributed computing;
- AI;
- data;
- networking;
- security;
- resources;
- effects;
- memory;
- concurrency;
- modules;
- declarations;
- functions;
- macros;
- metaprogramming;
- dialects;
- interoperability grammars;
- future computational domains.

The canonical rule is:

«No grammar production may participate in an unintended left-recursive derivation.»

This includes both obvious direct recursion and recursion hidden behind other rules.

---

2. Normative Language

The following terms are normative:

- MUST
- MUST NOT
- REQUIRED
- SHALL
- SHALL NOT
- SHOULD
- SHOULD NOT
- MAY

A MUST requirement is required for production conformance.

A SHOULD requirement may be departed from only when the deviation is documented and justified.

---

3. Authority

Left-recursion validation is governed by the following architecture:

grammar/DESIGN.md
        │
        ▼
grammar/specification/
        │
        ▼
grammar/spec/
        │
        ▼
canonical grammar architecture
        │
        ├── grammar/Zamani.g4
        ├── grammar/antlr/ZamaniParser.g4
        └── grammar/*/*.g4
        │
        ▼
grammar/validation/left-recursion.md
        │
        ▼
automated grammar validation
        │
        ▼
ANTLR generation
        │
        ▼
Rust lexer/parser conformance
        │
        ▼
AST
        │
        ▼
semantic analysis
        │
        ▼
canonical IR

This document governs left-recursion correctness.

It does not replace:

- "grammar/DESIGN.md";
- "grammar/validation/ambiguity-rules.md";
- "grammar/expressions/precedence.md";
- "grammar/spec/syntax.md";
- "grammar/spec/type-system.md";
- AST contracts;
- semantic contracts;
- canonical IR contracts.

Each retains its existing responsibility.

---

4. Existing Repository Architecture

The repository already contains the intended separation between grammar components.

The important existing expression composition is approximately:

expression
    ↓
assignmentExpression
    ↓
conditionalExpression
    ↓
rangeExpression
    ↓
logicalOrExpression
    ↓
logicalAndExpression
    ↓
bitwiseOrExpression
    ↓
bitwiseXorExpression
    ↓
bitwiseAndExpression
    ↓
equalityExpression
    ↓
relationalExpression
    ↓
shiftExpression
    ↓
additiveExpression
    ↓
multiplicativeExpression
    ↓
prefixExpression
    ↓
postfixExpression
    ↓
primaryExpression

The current canonical expression grammar already uses iterative forms such as:

additiveExpression
    : multiplicativeExpression
      (
          additiveOperator
          multiplicativeExpression
      )*
    ;

and:

multiplicativeExpression
    : prefixExpression
      (
          multiplicativeOperator
          prefixExpression
      )*
    ;

This is the preferred architecture.

The Rust parser independently implements precedence through its Pratt-style expression parser.

Therefore:

«The grammar and Rust parser MUST represent the same precedence model without requiring either implementation to introduce left recursion.»

---

5. Definition of Left Recursion

A grammar rule "A" is directly left recursive if:

A → A α

for some sequence "α".

Example:

expression
    : expression PLUS expression
    | primaryExpression
    ;

This is direct left recursion.

A rule is indirectly left recursive if:

A → B α
B → C β
C → A γ

and the derivation returns to "A" before consuming a token.

A rule is nullable-prefix left recursive if recursion occurs after one or more nullable productions.

For example:

A → B C
B → ε
C → A

Although "A" does not visibly begin with "A", it can derive:

A → B C
  → ε C
  → A

Therefore it is left recursive.

---

6. Production Invariant

For every parser rule "R":

R

MUST NOT have a derivation:

R ⇒* R α

where the recursive return to "R" occurs before consuming a terminal token.

More generally, for a strongly connected component of grammar rules:

R1 → R2
R2 → R3
...
Rn → R1

the cycle MUST contain a guaranteed terminal-consuming transition before recursion can return to its starting rule.

If no such guaranteed consumption exists, the cycle is invalid.

---

7. Why Left Recursion Is Being Prohibited

ANTLR4 can transform certain direct left-recursive expression rules.

That capability does not make arbitrary left recursion a production-safe architecture for Zamani.

The repository has additional requirements:

- modular grammar composition;
- Rust parser conformance;
- deterministic AST construction;
- grammar portability;
- grammar validation;
- dialect composition;
- future grammar generation;
- interoperability grammars;
- maintainability;
- independent file completion;
- scalability.

Allowing ANTLR-specific left-recursion transformations in one part of the language while the Rust parser uses another mechanism would create two parsing models.

Therefore:

«ANTLR's ability to accept a left-recursive rule is not sufficient evidence that the rule is production-valid Zamani grammar.»

The canonical grammar architecture MUST prefer explicit non-left-recursive precedence layers.

---

8. Absolute Rule

The production Zamani grammar MUST NOT contain:

expression
    : expression ...
    ;

or any equivalent direct left-recursive rule.

It MUST NOT contain:

A
    : B ...
    ;

B
    : A ...
    ;

when the recursive cycle can be entered without consuming a token.

It MUST NOT contain nullable-prefix recursion such as:

A
    : optionalB C
    ;

optionalB
    :
    | B
    ;

C
    : A
    ;

It MUST NOT hide recursion through imported grammar fragments.

---

9. Direct Left Recursion

The validator MUST detect patterns equivalent to:

expression
    : expression PLUS term
    ;

typeExpression
    : typeExpression QUESTION
    ;

statement
    : statement statementSuffix
    ;

declaration
    : declaration modifier
    ;

Such constructs MUST be rejected unless the rule is explicitly classified as a permitted non-parser-recursive semantic construct.

For the canonical Zamani grammar, no parser rule receives such an exemption.

---

10. Indirect Left Recursion

The validator MUST construct a grammar dependency graph.

For every parser rule:

Rule A

record the rules that can occur before the first guaranteed terminal.

For example:

A → B x
B → C
C → A

produces:

A → B
B → C
C → A

The validator MUST detect the cycle.

A simple textual search for:

A : A

is therefore insufficient.

---

11. Nullable Rules

The validator MUST calculate nullability.

A rule is nullable if it can derive the empty sequence:

ε

Examples:

optionalModifier
    :
    | modifier
    ;

and:

attributes
    : attribute*
    ;

are nullable.

Nullability MUST be calculated to a fixed point.

For the set of grammar rules:

N = ∅

the validator repeatedly adds:

A ∈ N

when an alternative for "A" consists entirely of:

- nullable rules;
- empty alternatives;
- zero-or-more constructs;
- optional constructs;
- semantic constructs known to consume no token.

The calculation terminates when no new nullable rules can be added.

No finite artificial bound may be used to decide language nullability.

---

12. FIRST-Terminal Analysis

The validator MUST calculate the set of possible first terminals for every rule.

Conceptually:

FIRST(A)

contains terminals that may appear first when parsing "A".

It must also track:

nullable(A)

because a nullable rule can expose the next symbol.

This permits detection of hidden recursion.

Example:

A
    : B C
    ;

B
    :
    | X
    ;

C
    : A
    ;

Because:

nullable(B) = true

the first meaningful symbol of "A" may come from "C".

Since:

C → A

the cycle is detected.

---

13. Left-Corner Graph

The production validator MUST construct a left-corner graph.

For every production:

A → X α

if "X" is a parser rule, add:

A → X

If "X" is nullable, continue scanning the alternative:

A → X Y Z

and add:

A → X
A → Y
A → Z

while preceding symbols remain nullable.

For terminals:

A → TOKEN ...

the left-corner chain terminates.

This graph is the central structure used for left-recursion detection.

---

14. Strongly Connected Components

The validator MUST compute strongly connected components (SCCs) over the left-corner graph.

A component containing:

A
B
C

with:

A → B
B → C
C → A

is a recursive component.

The validator MUST then determine whether the cycle has a guaranteed consuming terminal.

A recursive SCC without guaranteed consumption is a production error.

---

15. Consumption Analysis

A recursion cycle is safe only when every cycle traversal necessarily consumes input before returning to its starting rule.

The validator MUST distinguish:

recursive dependency

from:

left recursion

For example:

prefixExpression
    : prefixOperator prefixExpression
    | postfixExpression
    ;

is recursively nested but not left recursive.

The recursive call occurs after:

prefixOperator

which consumes a terminal.

Therefore:

prefixExpression
→ prefixOperator
→ TOKEN
→ prefixExpression

consumes input before recursion.

This is valid.

---

16. Right Recursion

Right recursion is not automatically prohibited.

For example:

prefixExpression
    : prefixOperator prefixExpression
    | postfixExpression
    ;

is right recursive.

Likewise:

powerExpression
    : unaryExpression
    | unaryExpression POWER powerExpression
    ;

can be right recursive.

The validator MUST classify it as:

RIGHT_RECURSION

rather than:

LEFT_RECURSION

provided the recursive call occurs after a guaranteed consuming prefix.

---

17. Right Recursion and Stack Scalability

Although right recursion is grammatically valid, the implementation MUST consider parser stack growth.

A grammar can be:

non-left-recursive

while still producing extremely deep parser recursion.

Therefore left-recursion validation MUST report, but not incorrectly reject, deep right-recursive constructs.

For example:

!!!!!!!!!!!!...value

may produce a very deep prefix-expression structure.

The language MUST NOT define a semantic maximum prefix depth merely to protect the implementation.

Implementation strategies MAY include:

- iterative parser loops;
- explicit stacks;
- parser resource budgets;
- configurable compilation limits;
- incremental parsing;
- bounded diagnostic recovery.

Such implementation limits MUST be reported as implementation/resource constraints rather than language semantics.

---

18. Expression Grammar Policy

The canonical Zamani expression grammar MUST use the following structural pattern:

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

Binary precedence levels SHOULD use:

lowerExpression
    : higherExpression
      (operator higherExpression)*
    ;

rather than:

lowerExpression
    : lowerExpression operator higherExpression
    | higherExpression
    ;

The former is the canonical Zamani style.

---

19. Associativity

Associativity MUST be expressed explicitly.

For left-associative binary operators:

additiveExpression
    : multiplicativeExpression
      (additiveOperator multiplicativeExpression)*
    ;

The AST builder interprets the sequence left-to-right.

For right-associative assignment:

assignmentExpression
    : conditionalExpression
    | assignmentTarget assignmentOperator assignmentExpression
    ;

the recursive occurrence appears on the right.

This is valid because the recursive call is not left-corner recursion.

---

20. Assignment Recursion

The current grammar's assignment structure is intentionally right recursive:

assignmentExpression
    : conditionalExpression
    | assignmentTarget assignmentOperator assignmentExpression
    ;

This MUST remain right recursive unless the language specification changes assignment semantics.

The validator MUST classify:

assignmentExpression

as:

RIGHT_RECURSIVE

not:

LEFT_RECURSIVE

The AST must preserve:

a = (b = c)

rather than:

(a = b) = c

---

21. Prefix Recursion

The current canonical expression grammar contains:

prefixExpression
    : prefixOperator prefixExpression
    | postfixExpression
    ;

This is valid.

The validator MUST verify:

prefixOperator

cannot derive ε.

If a future modification makes "prefixOperator" nullable, this rule becomes potentially left recursive and MUST fail validation.

Therefore nullability analysis is mandatory.

---

22. Postfix Expressions

Postfix expressions SHOULD use iterative suffix composition.

Conceptually:

postfixExpression
    : primaryExpression postfixSuffix*
    ;

where:

postfixSuffix
    =
      call
    | index
    | memberAccess
    | optionalMemberAccess
    | postfixOperator
    | other explicitly defined suffix

This supports arbitrary chaining:

a
a()
a()[i]
a()[i].field
a()[i].field(x)
a()[i].field(x)[j]

without left recursion.

The number of suffixes is semantic input, not a language-level fixed maximum.

---

23. Member Access

Member access MUST NOT be implemented as:

memberExpression
    : memberExpression DOT identifier
    | primaryExpression
    ;

The preferred structure is:

postfixExpression
    : primaryExpression postfixSuffix*
    ;

with:

memberSuffix
    : DOT identifier
    ;

This preserves arbitrary member chains without left recursion.

---

24. Calls

Calls MUST be postfix suffixes rather than left-recursive expression alternatives.

Preferred:

callSuffix
    : LPAREN argumentList? RPAREN
    ;

combined with:

postfixExpression
    : primaryExpression postfixSuffix*
    ;

This permits:

f()
f()()
f(a)(b)
object.method()(x)
pipeline.stage()(x)

without introducing left recursion.

Semantic validity remains downstream.

---

25. Indexing

Indexing MUST likewise be postfix composition.

Preferred:

indexSuffix
    : LBRACKET expression RBRACKET
    ;

rather than:

indexExpression
    : indexExpression LBRACKET expression RBRACKET
    | primaryExpression
    ;

The latter is unnecessarily left recursive.

---

26. Generic Type Recursion

Generic types are recursively nested but must not be left recursive.

Preferred structural form:

typeExpression
    : typeQualifier* typeCore typePostfix*
    ;

with generic arguments owned by the appropriate type-core production.

Nested types such as:

Map<String, Vector<Matrix<T>>>

must be representable without a fixed nesting limit.

The grammar MUST NOT contain:

typeExpression
    : typeExpression LESS ...

or equivalent left recursion.

---

27. Generic Type Versus Shift Operators

The validator MUST treat:

>>
>>>

and nested generic closing delimiters as a lexical/parser integration concern.

The validator MUST verify that the chosen lexer/parser strategy is consistent across:

grammar/lexer/
grammar/types/
grammar/expressions/
grammar/antlr/
src/lexer.rs
src/parser.rs

A solution MUST NOT depend on arbitrary rule ordering.

If the lexer produces a single:

SHIFT_RIGHT

token while the type parser requires two generic closers, the grammar implementation MUST explicitly define how that distinction is handled.

It MUST NOT rely on undocumented ANTLR behavior.

---

28. Type Recursion and References

Recursive type definitions are semantically legitimate.

For example:

List<List<T>>

or:

&T
&&T

does not imply left recursion.

The distinction is:

recursive language structure

versus:

recursive invocation before token consumption

The validator MUST not reject arbitrary recursion simply because a type can contain itself.

---

29. Tuple Types

Tuple nesting MUST be expressed structurally:

tupleType
    : LPAREN typeExpression (COMMA typeExpression)+ RPAREN
    ;

This supports:

(A, B)
(A, (B, C))
((A, B), (C, D))

without a separate recursive left edge.

---

30. Array and Slice Types

Array/slice recursion MUST enter through already-consumed delimiters.

Conceptually:

arrayType
    : LBRACKET typeExpression RBRACKET
    ;

and:

sliceType
    : LBRACKET typeExpression RBRACKET
    ;

or the repository's established exact syntax.

The important invariant is:

opening delimiter
    ↓
recursive type
    ↓
closing delimiter

rather than:

typeExpression
    : typeExpression suffix

---

31. Function Types

Function types MUST not introduce a left-recursive cycle through parameter or return type parsing.

A valid structure is conceptually:

functionType
    : FN LPAREN typeList? RPAREN THIN_ARROW typeExpression
    ;

The exact Zamani spelling remains governed by the canonical type grammar.

Nested function types MUST remain possible without a finite depth restriction.

---

32. Pattern Recursion

Pattern recursion is allowed when the recursive occurrence follows a consuming delimiter.

Valid conceptual form:

tuplePattern
    : LPAREN patternList? RPAREN
    ;

and:

enumPattern
    : identifier LPAREN patternList? RPAREN
    ;

The recursion is safe because:

LPAREN

is consumed before nested pattern parsing.

---

33. Block Recursion

Blocks may contain statements that contain blocks.

For example:

{
    if condition {
        while condition {
            ...
        }
    }
}

This is nested recursion, not left recursion.

The validator MUST therefore distinguish:

block → statement → block

from:

block → block ...

The first is safe because structural delimiters establish consumption.

---

34. Conditional Expressions

Conditional expressions MUST have a consuming discriminator.

For example:

ifExpression
    : IF expression blockExpression
      (ELSE (ifExpression | blockExpression))?
    ;

The recursive "ifExpression" occurs after:

ELSE

which is a consumed terminal.

This is not left recursion.

The dangling-"else" policy remains governed by:

grammar/validation/ambiguity-rules.md

---

35. Match Expressions

Match expressions MUST consume their opening delimiter before recursively parsing arms/patterns:

MATCH
LPAREN / expression delimiter
LBRACE
...

Recursive patterns and nested expressions are therefore structurally bounded by consumed delimiters.

The validator MUST inspect nullable prefixes nevertheless.

---

36. Macro Grammar

Macro syntax MUST be included in left-recursion validation.

Macro grammar MUST NOT introduce:

macro → expression → macro

without a consuming discriminator.

Macro expansion MUST NOT be used to hide grammar recursion from the static validator.

The validator operates on the declared grammar architecture before expansion.

Expanded syntax MUST subsequently be parsed using the same left-recursion-safe grammar.

---

37. Dialects

Every enabled dialect MUST be incorporated into left-recursion validation.

A dialect MUST NOT introduce:

core rule
    → dialect rule
    → core rule

without guaranteed token consumption.

Dialect activation MUST be deterministic.

A dialect cannot evade validation by being loaded dynamically after grammar validation.

---

38. Interoperability Grammars

Files under:

grammar/interoperability/

may intentionally represent external grammars such as:

- OpenQASM;
- Verilog;
- SystemVerilog;
- assembly;
- Zig;
- other foreign syntaxes.

They MUST be treated as distinct grammar namespaces.

An interoperability grammar MUST NOT accidentally participate in the canonical Zamani parser graph.

If it is imported into the canonical Zamani grammar, it becomes subject to this document.

Otherwise it is validated as an independent grammar artifact.

This prevents a foreign grammar's recursion strategy from contaminating Zamani's canonical parser architecture.

---

39. Foreign Grammar Compatibility

A foreign grammar may contain left recursion if its own parser technology requires it.

That does not automatically make it valid as a Zamani canonical grammar component.

The validation result MUST therefore distinguish:

CANONICAL_ZAMANI

from:

FOREIGN_INTEROPERABILITY

and:

HISTORICAL/REFERENCE

The foreign grammar MUST NOT be silently promoted into canonical Zamani syntax.

---

40. Semantic Recursion Is Not Grammar Recursion

The following are not grammar-left-recursion problems:

recursive function calls
recursive data structures
recursive types
recursive algorithms
recursive quantum circuits
recursive HDL modules
distributed recursive workflows
AI recursive models

For example:

fn factorial(n) {
    factorial(n - 1)
}

is a semantic/runtime recursion issue.

It does not constitute grammar left recursion.

The validator MUST NOT reject source programs merely because their semantics are recursive.

---

41. AST Integration

The left-recursion policy MUST preserve the existing domain-neutral AST architecture.

The AST already distinguishes constructs including:

UnaryExpression
BinaryExpression
Assignment
CallExpression
MemberAccess
IndexExpression
CastExpression
TypeAscription
ConditionalExpression
BlockExpression
MatchExpression
LoopExpression
LambdaExpression
AwaitExpression
AsyncExpression
SpawnExpression
RangeExpression
ArrayExpression
TupleExpression
StructExpression

The grammar transformation from left-recursive expression forms to iterative precedence forms MUST NOT change the semantic AST model.

For example:

a + b + c

may be parsed through:

additiveExpression
    : multiplicativeExpression
      (additiveOperator multiplicativeExpression)*

while still producing the canonical binary-expression structure expected by the frontend.

The grammar transformation is an implementation technique, not a semantic change.

---

42. AST Associativity Preservation

For:

a - b - c

the grammar must preserve:

(a - b) - c

For:

a = b = c

the grammar must preserve:

a = (b = c)

The left-recursion validator therefore MUST work together with:

grammar/expressions/precedence.md

and AST conformance tests.

Removing left recursion MUST NOT accidentally change associativity.

---

43. Rust Parser Integration

The repository's Rust parser uses explicit precedence handling.

Therefore the canonical integration is:

tokens
   ↓
src/parser.rs
   ↓
parse_expression(precedence)
   ↓
prefix / postfix / infix processing
   ↓
frontend AST

The Rust parser MUST NOT introduce a separate precedence model from the grammar.

The validator SHOULD maintain a machine-readable or testable correspondence between:

grammar precedence level

and:

Rust parser precedence level

The two models must agree on:

- precedence;
- associativity;
- assignment;
- range;
- unary operators;
- postfix operations;
- calls;
- indexing;
- member access;
- casts;
- optional/try operators;
- future operators.

---

44. Pratt Parser and Left Recursion

A Pratt parser naturally avoids grammar left recursion by representing precedence procedurally.

This does not remove the need to validate "grammar/*.g4".

The two representations have different responsibilities:

ANTLR grammar
    ↓
declarative syntax contract

Rust Pratt parser
    ↓
executable parser implementation

Both MUST describe the same language.

Neither is allowed to silently expand or restrict the other.

---

45. Parser Termination

Every parser rule MUST have a termination argument.

For each recursive rule, documentation or validation metadata MUST establish:

What token(s) are consumed before recursion?

For example:

prefixExpression
    : prefixOperator prefixExpression

termination condition:

prefixOperator consumes one token.

For:

assignmentExpression
    : assignmentTarget assignmentOperator assignmentExpression

termination condition:

assignmentOperator consumes one token before recursion.

A recursive rule without such a proof MUST fail production validation.

---

46. Progress Invariant

Every parser loop MUST satisfy:

iteration
    → either consumes input
    → or terminates

This is particularly important for:

X*
X+

where "X" may itself be nullable.

The validator MUST reject:

items
    : optionalItem*
    ;

optionalItem
    :
    | ITEM
    ;

because the loop can repeat without consuming input.

This is a zero-consumption repetition error even when it is not technically left recursion.

It belongs in the same production validation pass because it can produce nontermination.

---

47. Nullable Repetition

The validator MUST detect:

A*

where:

nullable(A) = true

and:

A+

where:

nullable(A) = true

These constructs MUST be rejected.

Examples of dangerous patterns include:

list
    : element*
    ;

element
    :
    | item
    ;

and:

suffixes
    : suffix*
    ;

suffix
    : optionalPart
    ;

optionalPart
    :
    | TOKEN
    ;

---

48. Optional Recursion

The validator MUST detect recursive paths hidden behind "?".

Example:

A
    : B? C
    ;

B
    : A
    ;

If "B" is optional and "C" can be reached without consumption, the cycle is invalid.

---

49. EBNF Operators

The validator MUST expand or model:

?
*
+

semantically when constructing its dependency graph.

It MUST NOT treat them merely as textual punctuation.

For example:

A
    : B* C
    ;

means:

A may begin with B

because the repetition may be zero occurrences.

Therefore "B" participates in left-corner analysis.

---

50. Grouped Alternatives

Grouped alternatives MUST be analyzed individually.

For:

A
    : (B | C)? D
    ;

the validator must consider:

A → B
A → C
A → D

because the group is optional.

A textual first-token check is insufficient.

---

51. Semantic Predicates

Semantic predicates MUST NOT be used to hide left recursion.

This is prohibited as an architectural escape hatch:

A
    : {condition}? A X
    ;

The validator MUST still classify the recursive structure.

Semantic predicates MAY constrain a valid syntactic decision, but they MUST NOT be used to make an otherwise invalid recursive grammar appear safe.

---

52. Actions

Canonical Zamani grammar MUST remain free of parser actions that alter recursion behavior.

Grammar actions MUST NOT:

- mutate global parser state;
- select grammar rules based on hardware;
- dynamically disable recursion;
- inspect filesystem state;
- inspect environment variables;
- access network state;
- discover hardware;
- invoke compiler backends.

The canonical grammar remains declarative.

---

53. Error Recovery

Error recovery MUST NOT introduce recursive grammar paths.

Recovery productions such as:

error

or:

synchronization

MUST consume or skip input deterministically.

Recovery MUST NOT create:

A → error
error → A

without a guaranteed consuming boundary.

---

54. Generated Grammar Validation

The validator MUST operate on the grammar actually presented to the parser generator.

It is insufficient to validate only hand-authored fragments.

The production pipeline MUST be:

source grammar files
        ↓
import resolution
        ↓
grammar composition
        ↓
normalized grammar representation
        ↓
left-corner analysis
        ↓
SCC analysis
        ↓
nullability analysis
        ↓
consumption analysis
        ↓
ANTLR generation
        ↓
generated grammar validation

If the generated grammar differs structurally from the source model, validation MUST fail or report the discrepancy.

---

55. Import Graph

The validator MUST construct a grammar import graph.

For example:

Zamani.g4
    ↓
ZamaniParser.g4
    ↓
Expressions
    ↓
Postfix
    ↓
Calls

Every import MUST be resolved.

The validator MUST reject:

A → B
B → A

when those imports create a recursive grammar dependency that is not explicitly permitted by the grammar architecture.

Import cycles and parser-rule cycles are separate concepts, but both MUST be visible to validation.

---

56. Canonical Grammar Ownership

A parser rule MUST have exactly one canonical owner.

For example:

expression

belongs to:

grammar/expressions/expressions.g4

Specialized files MUST NOT redefine it.

Likewise:

typeExpression

belongs to the canonical type grammar.

Specialized type files MUST extend the canonical architecture rather than creating competing public roots.

This reduces accidental recursive cycles caused by duplicated entry points.

---

57. Domain Grammar Rule

Domain grammars MUST NOT redefine the universal expression hierarchy.

The following MUST remain centralized:

expression
assignmentExpression
conditionalExpression
rangeExpression
logicalOrExpression
logicalAndExpression
bitwiseOrExpression
bitwiseXorExpression
bitwiseAndExpression
equalityExpression
relationalExpression
shiftExpression
additiveExpression
multiplicativeExpression
prefixExpression
postfixExpression
primaryExpression

Quantum, HDL, AI, hardware, networking, and other domains MAY contribute:

- primary-expression forms;
- postfix forms;
- explicit domain constructs;
- declarations;
- statements;
- semantic capabilities;

but they MUST integrate through the canonical expression architecture.

---

58. Quantum Integration

Quantum syntax MUST NOT introduce a second expression recursion hierarchy.

For example, this is prohibited:

quantumExpression
    : quantumExpression operation quantumExpression
    ;

unless it is explicitly part of a separately validated foreign grammar.

The preferred model is:

generic expression
        ↓
domain-aware semantic analysis
        ↓
quantum operation
        ↓
quantum::ir

This is consistent with the repository's canonical quantum architecture.

The grammar MUST NOT use recursion to enumerate gate sets.

It MUST support open-ended operation names.

---

59. HDL Integration

HDL syntax MUST not create a competing expression grammar.

For example:

hdlExpression

MUST integrate with the canonical expression system wherever the HDL construct is an ordinary expression.

HDL-specific recursion may exist for:

- module nesting;
- generate constructs;
- state machines;
- interface structures;

but each recursive cycle must have a termination/consumption proof.

---

60. Hardware and Resource Integration

Resource syntax such as:

requires qubits >= n
requires memory >= required_memory
requires capability("gpu.compute")
requires topology(...)

must not require recursive hardware enumeration.

The grammar MUST NOT encode:

resource0
resource1
...
resourceN

as a universal grammar architecture.

Resource cardinality is semantic data.

---

61. POCO-REAF

Left-recursion architecture MUST preserve:

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever

Parsing MUST NOT depend on:

- CPU count;
- GPU availability;
- FPGA capacity;
- QPU size;
- physical qubit count;
- memory size;
- node count;
- network topology;
- runtime scheduling;
- calibration;
- backend selection.

A parser grammar must remain identical whether the eventual target is:

atom-scale / embedded
CPU
multicore CPU
GPU
FPGA
ASIC
QPU
quantum simulator
accelerator
HPC
cluster
distributed system
cloud
future architecture

---

62. Scalability

No grammar-level recursion rule may impose a fixed semantic limit such as:

MAX_RECURSION
MAX_EXPRESSION_DEPTH
MAX_TYPE_DEPTH
MAX_GENERIC_DEPTH
MAX_NESTING
MAX_CALL_CHAIN

These identifiers MUST NOT be used as language-level limits.

A compiler implementation MAY maintain a configurable resource budget to protect against:

- stack exhaustion;
- memory exhaustion;
- denial-of-service inputs;
- compilation resource exhaustion.

Such a budget is an implementation policy.

It is not a Zamani language limit.

---

63. "Infinity" Interpretation

The requirement that Zamani scale from tiny computations toward arbitrarily large computations means:

«The language grammar MUST NOT impose artificial finite semantic ceilings.»

It does not mean:

infinite memory
infinite CPU
infinite compilation time

is assumed.

Therefore:

language expressibility

and:

implementation feasibility

MUST remain distinct.

A source program can be syntactically valid but infeasible for a particular target.

That is a resource/capability decision, not a left-recursion decision.

---

64. Validation Algorithm

The production validator SHOULD implement the following algorithm.

Step 1 — Load grammar roots

Load:

grammar/Zamani.g4
grammar/antlr/ZamaniParser.g4

according to the repository's canonical composition architecture.

Step 2 — Resolve imports

Resolve all imported grammar files.

Step 3 — Parse grammar definitions

Construct an internal grammar representation containing:

RuleId
Alternative
Element
Terminal
RuleReference
Sequence
Choice
Optional
ZeroOrMore
OneOrMore
Predicate
Action

Step 4 — Calculate nullability

Compute:

nullable(rule)

to a fixed point.

Step 5 — Build left-corner graph

For each rule, traverse its prefix until a guaranteed terminal is encountered.

Step 6 — Compute SCCs

Run a deterministic SCC algorithm such as:

Tarjan

or:

Kosaraju

Step 7 — Inspect recursive SCCs

For each SCC containing more than one rule, or a self-edge:

determine whether the recursive path necessarily consumes a token.

Step 8 — Detect zero-consumption cycles

Reject cycles where no guaranteed terminal is consumed before recursion.

Step 9 — Detect nullable repetition

Reject:

nullable(X)*
nullable(X)+

Step 10 — Detect recursive predicates/actions

Report grammar actions/predicates participating in recursion.

Step 11 — Emit deterministic diagnostics

Diagnostics MUST include:

- rule;
- recursive path;
- source file;
- source location;
- recursion category;
- reason;
- suggested structural correction;
- affected canonical owner.

---

65. Diagnostic Categories

The validator MUST provide stable diagnostic categories.

Recommended identifiers:

GRAMMAR-LR001
GRAMMAR-LR002
GRAMMAR-LR003
GRAMMAR-LR004
GRAMMAR-LR005
GRAMMAR-LR006
GRAMMAR-LR007
GRAMMAR-LR008
GRAMMAR-LR009
GRAMMAR-LR010

Recommended meanings:

Code| Meaning
"GRAMMAR-LR001"| Direct left recursion
"GRAMMAR-LR002"| Indirect left recursion
"GRAMMAR-LR003"| Nullable-prefix recursion
"GRAMMAR-LR004"| Zero-consumption recursive cycle
"GRAMMAR-LR005"| Nullable repetition
"GRAMMAR-LR006"| Nullable "+" repetition
"GRAMMAR-LR007"| Recursive predicate/action cycle
"GRAMMAR-LR008"| Duplicate public precedence hierarchy
"GRAMMAR-LR009"| Unvalidated imported grammar recursion
"GRAMMAR-LR010"| Rust/ANTLR precedence divergence

These identifiers MUST remain stable once exposed to tooling.

---

66. Diagnostic Example

For:

expression
    : expression PLUS term
    | term
    ;

the validator SHOULD report conceptually:

GRAMMAR-LR001: direct left recursion

rule: expression
file: grammar/expressions/expressions.g4

recursive path:
    expression
    -> expression

reason:
    recursive reference occurs before any guaranteed terminal is consumed

required correction:
    use the canonical precedence-chain form

example:
    expression
        : term (PLUS term)*
        ;

---

67. Indirect Diagnostic Example

For:

A
    : B X
    ;

B
    : C
    ;

C
    : A
    ;

the diagnostic SHOULD report:

GRAMMAR-LR002: indirect left recursion

recursive cycle:
    A -> B -> C -> A

consuming prefix:
    none

result:
    production grammar rejected

---

68. Nullable Diagnostic Example

For:

A
    : B C
    ;

B
    :
    | X
    ;

C
    : A
    ;

the validator MUST report:

GRAMMAR-LR003: nullable-prefix left recursion

cycle:
    A -> C -> A

nullable prefix:
    B

no terminal is guaranteed before recursive return

---

69. Zero-Consumption Diagnostic

For:

items
    : item*
    ;

item
    :
    | ITEM
    ;

the validator MUST report:

GRAMMAR-LR005: nullable repetition

rule:
    items

repeated rule:
    item

reason:
    item can derive ε

result:
    repetition can make zero parser progress

---

70. Correcting Left-Recursive Binary Expressions

Incorrect:

additiveExpression
    : additiveExpression PLUS multiplicativeExpression
    | additiveExpression MINUS multiplicativeExpression
    | multiplicativeExpression
    ;

Correct:

additiveExpression
    : multiplicativeExpression
      (
          additiveOperator
          multiplicativeExpression
      )*
    ;

additiveOperator
    : PLUS
    | MINUS
    ;

This is the canonical Zamani pattern.

---

71. Correcting Left-Recursive Calls

Incorrect:

callExpression
    : callExpression LPAREN argumentList? RPAREN
    | primaryExpression
    ;

Correct:

postfixExpression
    : primaryExpression postfixSuffix*
    ;

postfixSuffix
    : callSuffix
    | indexSuffix
    | memberSuffix
    ;

---

72. Correcting Left-Recursive Member Access

Incorrect:

memberExpression
    : memberExpression DOT IDENTIFIER
    | primaryExpression
    ;

Correct:

postfixExpression
    : primaryExpression postfixSuffix*
    ;

memberSuffix
    : DOT IDENTIFIER
    ;

---

73. Correcting Left-Recursive Indexing

Incorrect:

indexExpression
    : indexExpression LBRACKET expression RBRACKET
    | primaryExpression
    ;

Correct:

postfixExpression
    : primaryExpression postfixSuffix*
    ;

indexSuffix
    : LBRACKET expression RBRACKET
    ;

---

74. Correcting Left-Recursive Type Qualifiers

Incorrect:

typeExpression
    : typeExpression QUESTION
    | typeCore
    ;

Correct:

typeExpression
    : typeQualifier* typeCore typePostfix*
    ;

or the repository's established equivalent.

Optionality is a postfix property, not a reason to create left recursion.

---

75. Correcting Recursive Qualified Names

Incorrect:

qualifiedName
    : qualifiedName DOUBLE_COLON IDENTIFIER
    | IDENTIFIER
    ;

Correct:

qualifiedName
    : IDENTIFIER
      (DOUBLE_COLON IDENTIFIER)*
    ;

This supports arbitrary namespace depth without left recursion.

---

76. Correcting Recursive Paths

The same principle applies to module paths:

module::submodule::type

must use:

first component
+
zero or more separator/component suffixes

rather than:

path → path separator component

This is both easier to validate and easier to map into the AST.

---

77. Precedence-Level Completeness

Every expression precedence level MUST have:

1. exactly one canonical owner;
2. one documented precedence position;
3. explicit associativity;
4. a lower-precedence parent;
5. a higher-precedence child;
6. no cycle to itself through nullable prefixes;
7. AST mapping;
8. Rust parser mapping;
9. conformance tests.

The validator MUST reject duplicate precedence levels that introduce competing paths.

---

78. Precedence Graph

The validator SHOULD construct a precedence graph:

assignment
    ↓
conditional
    ↓
range
    ↓
logical-or
    ↓
logical-and
    ↓
bitwise-or
    ↓
bitwise-xor
    ↓
bitwise-and
    ↓
equality
    ↓
relational
    ↓
shift
    ↓
additive
    ↓
multiplicative
    ↓
prefix
    ↓
postfix
    ↓
primary

This graph MUST be acyclic.

Any cycle is a production error.

For example:

additive
    → multiplicative
    → prefix
    → postfix
    → primary
    → additive

would be invalid unless the return path is through a consumed syntactic delimiter and does not represent precedence recursion.

---

79. Rust/ANTLR Precedence Conformance

The validator MUST compare the declarative grammar hierarchy with the Rust parser.

The Rust parser currently exposes precedence behavior through its precedence enum and expression parsing methods.

The production conformance test MUST verify representative expressions at every precedence boundary.

Examples:

a + b * c
a * b + c
a << b + c
a < b == c
a & b == c
a && b || c
a .. b
a = b = c

The expected AST must be identical in meaning between:

ANTLR parser

and:

Rust parser

where both are used as conformance implementations.

---

80. AST Conformance Tests

For each corrected left-recursion structure, tests MUST verify:

source
→ parse tree
→ AST

Examples:

a + b + c
a * b * c
a + b * c
f()(x)[i].field
a = b = c
Map<String, Vector<T>>

The tests MUST verify nesting and associativity, not merely successful parsing.

---

81. Negative Tests

The validation suite MUST contain grammar-level negative cases for:

direct left recursion
indirect left recursion
nullable-prefix recursion
zero-consumption repetition
recursive optional cycles
duplicate precedence roots
duplicate expression hierarchies
unresolved imported recursion

A negative grammar fixture MUST fail the validator for the expected reason.

---

82. Boundary Tests

Boundary tests MUST cover:

one operator
two operators
many operators
deep nesting
deep generic types
deep namespace paths
deep postfix chains
deep prefix chains
nested blocks
nested patterns
nested function types
nested tuples
nested arrays

No test may establish a false language maximum.

---

83. Scalability Tests

The validator and parser conformance suite SHOULD generate structurally larger programs rather than defining hard-coded maximums.

Examples:

expression with N additive operators
type nested N levels
namespace path with N components
postfix chain with N suffixes
nested tuple with N levels

where "N" is a test parameter.

The test harness MAY choose practical values according to CI resources.

Those values MUST NOT become language constants.

---

84. Resource-Bounded Testing

Testing an extremely deep recursive grammar may exhaust stack or memory.

The test harness MAY therefore define:

test budget

including:

- maximum generated input size;
- wall-clock budget;
- memory budget;
- process budget.

These are test infrastructure controls.

They are not language limits.

---

85. Fuzz Testing

Production validation SHOULD fuzz grammar recursion boundaries.

Fuzz inputs SHOULD target:

- nested parentheses;
- nested generics;
- chained calls;
- chained indexes;
- chained member access;
- unary chains;
- assignment chains;
- nested blocks;
- nested patterns;
- optional constructs;
- empty constructs;
- delimiter-heavy constructs.

The fuzz harness MUST detect:

stack overflow
nontermination
parser hang
excessive allocation
infinite error recovery
inconsistent AST
inconsistent diagnostics

---

86. Determinism Tests

The same input MUST produce the same result across repeated runs.

The validator MUST NOT use:

- random rule iteration;
- hash-map iteration order;
- filesystem ordering;
- network information;
- hardware discovery;
- wall-clock time.

Graph traversal MUST use stable ordering where diagnostics or serialized validation output are exposed.

For example, rules SHOULD be ordered by canonical source location or fully qualified rule name.

---

87. Diagnostic Determinism

For the same:

grammar version
source files
imports
dialects
feature configuration

the validator MUST produce the same diagnostics in the same deterministic order.

Diagnostics SHOULD be sorted by:

1. canonical file path;
2. source location;
3. diagnostic code;
4. rule name;
5. recursive path.

---

88. No Hardware Dependency

The validator MUST NOT inspect:

CPU
GPU
FPGA
ASIC
QPU
memory capacity
device count
cluster size
network topology

to determine whether grammar recursion is valid.

Grammar correctness is target-independent.

---

89. No Fixed Recursion Constants

The following MUST NOT become language-level configuration:

MAX_RECURSION_DEPTH
MAX_PARSE_DEPTH
MAX_GRAMMAR_DEPTH
MAX_EXPRESSION_DEPTH
MAX_TYPE_DEPTH
MAX_GENERIC_DEPTH

A safe implementation may use an execution budget, but the distinction MUST be explicit:

language semantics:
    unbounded by artificial grammar depth

implementation:
    finite resources

---

90. Safe Rust Requirement

Any validator implementation associated with this specification MUST use safe Rust.

The implementation MUST NOT contain:

unsafe
unsafe fn
unsafe impl
unsafe trait
unsafe { ... }

The validator SHOULD use standard safe data structures such as:

Vec
HashMap
HashSet
BTreeMap
BTreeSet
VecDeque

as appropriate.

Graph algorithms MUST be implemented without unsafe pointer manipulation.

---

91. Rust 1.97 / 1.97.1 Compatibility

The validator implementation MUST compile against the repository's stated Rust baseline:

Rust 1.97
Rust 1.97.1
Edition 2021

It MUST NOT depend on a newer Rust language feature unless the repository's supported compiler baseline is deliberately changed through the normal compatibility process.

---

92. Parser Stack Safety

The validator MUST distinguish:

grammar left recursion

from:

runtime stack consumption

Eliminating left recursion does not automatically guarantee arbitrary parser depth.

The production parser SHOULD use iterative approaches where deep input is expected.

Particularly important candidates are:

postfix chains
binary operator chains
qualified names
generic argument lists
collection literals
statement lists
source elements

---

93. Iterative Preferred Forms

Where a grammar construct is naturally a sequence, prefer:

X
    : Y suffix*
    ;

over recursive linked syntax.

This is especially appropriate for:

operators
namespaces
member access
calls
indexes
arguments
parameters
imports
exports
attributes
modifiers
resource requirements
capabilities
collection elements

This reduces parser stack growth and simplifies AST construction.

---

94. Recursive Preferred Forms

Recursion remains appropriate when the language structure is intrinsically hierarchical.

Examples:

parenthesized expressions
nested blocks
nested types
nested tuples
nested patterns
nested function types
nested generic types
nested declarations

The recursion MUST cross a consuming structural boundary.

---

95. Grammar Review Checklist

Every new grammar file containing recursion MUST answer:

What rule recurses?
Why is recursion necessary?
Is it direct?
Is it indirect?
Which rules are nullable?
What is the left-corner graph?
Does the recursive path consume a token?
Can the recursion terminate?
Can repetition contain nullable content?
Does the rule participate in a precedence cycle?
Does it duplicate an existing public rule?
What AST node receives the result?
What Rust parser function handles it?
What tests prove termination?
What tests prove associativity?
What tests prove scalability?

A file is not complete until these questions have deterministic answers.

---

96. File Completion Contract

This document itself is complete when:

- the definition of left recursion is normative;
- direct recursion is covered;
- indirect recursion is covered;
- nullable-prefix recursion is covered;
- nullable repetition is covered;
- SCC analysis is defined;
- left-corner analysis is defined;
- consumption analysis is defined;
- precedence cycles are covered;
- imported grammars are covered;
- domain grammars are covered;
- dialect grammars are covered;
- macro grammars are covered;
- interoperability grammars are covered;
- ANTLR integration is defined;
- Rust parser integration is defined;
- AST integration is defined;
- scalability is defined;
- POCO-REAF is defined;
- safe Rust is required;
- Rust 1.97/1.97.1 compatibility is defined;
- diagnostic identifiers are defined;
- positive tests are defined;
- negative tests are defined;
- boundary tests are defined;
- scalability tests are defined;
- determinism tests are defined;
- no existing filename needs to be renamed.

---

97. Required Integration With Existing Files

This file integrates with the existing repository as follows.

"grammar/DESIGN.md"

"DESIGN.md" remains the architectural authority.

This document implements its requirement that grammar validation detect:

- left recursion;
- precedence conflicts;
- excessive backtracking;
- parser conflicts;
- unreachable rules;
- deterministic grammar structure.

No architectural rule in this document may contradict "DESIGN.md".

---

"grammar/validation/ambiguity-rules.md"

The existing ambiguity policy contains a section permitting ANTLR-supported left recursion where it is deterministic.

For the canonical Zamani grammar, that permission MUST be interpreted narrowly and superseded by this document:

«ANTLR-supported left recursion MUST NOT be used as the canonical Zamani grammar architecture.»

ANTLR compatibility does not override the repository's cross-parser, modularity, and scalability requirements.

"ambiguity-rules.md" SHOULD therefore be updated so its left-recursion section explicitly delegates canonical Zamani left-recursion policy to this document.

The older statement:

ANTLR-supported left recursion MAY be used

must not be treated as permission to introduce new canonical left-recursive grammar rules.

---

"grammar/expressions/expressions.g4"

This remains the canonical expression-composition owner.

Its existing precedence hierarchy is already aligned with this policy.

The file MUST retain the non-left-recursive structure:

assignment
→ conditional
→ range
→ logical
→ bitwise
→ equality
→ relational
→ shift
→ additive
→ multiplicative
→ prefix
→ postfix
→ primary

No specialized expression grammar may create a competing hierarchy.

---

"grammar/expressions/precedence.md"

This file owns detailed precedence and associativity semantics.

"left-recursion.md" validates its structural implementation.

The two files MUST agree on:

- precedence order;
- associativity;
- recursive direction;
- AST structure.

---

"grammar/types/types.g4"

The canonical type-expression hierarchy MUST remain non-left-recursive.

Nested types remain supported through structural recursion and postfix composition.

---

"grammar/antlr/ZamaniParser.g4"

This remains the ANTLR parser composition boundary.

It MUST consume the canonical modular grammar without introducing another expression or type precedence hierarchy.

Generated parser validation MUST use the same left-recursion checks.

---

"grammar/Zamani.g4"

"Zamani.g4" remains the canonical visible ANTLR composition root.

It MUST NOT reintroduce left-recursive rules that have already been eliminated in modular grammar files.

The root grammar must compose canonical rules rather than redefine them.

---

"src/parser.rs"

The Rust parser remains the executable frontend implementation.

Its Pratt-style precedence parser MUST remain semantically aligned with the grammar hierarchy.

Grammar changes MUST NOT silently introduce a different precedence model.

---

"src/ast/"

The domain-neutral AST remains the structural target.

Removing left recursion MUST NOT require target-specific AST nodes.

Expression parsing must continue to produce canonical:

UnaryExpression
BinaryExpression
Assignment
CallExpression
MemberAccess
IndexExpression
CastExpression
TypeAscription
ConditionalExpression
BlockExpression
MatchExpression
LoopExpression
LambdaExpression
AwaitExpression
AsyncExpression
SpawnExpression
RangeExpression
ArrayExpression
TupleExpression
StructExpression

where applicable.

---

Semantic Analysis

Semantic analysis remains responsible for:

- name resolution;
- type checking;
- overload resolution;
- generic inference;
- effects;
- resources;
- capabilities;
- quantum legality;
- hardware requirements.

Left-recursion validation MUST NOT move semantic decisions into parsing.

---

Canonical "quantum::ir"

Left-recursion validation has no direct dependency on "quantum::ir".

The integration remains:

quantum source
    ↓
lexer
    ↓
parser
    ↓
domain-neutral AST
    ↓
semantic analysis
    ↓
quantum::ir

No second quantum frontend IR is introduced.

---

98. Required Production Validation Pipeline

The repository's validation pipeline SHOULD become:

1. Discover grammar files
        ↓
2. Resolve imports
        ↓
3. Validate grammar ownership
        ↓
4. Parse grammar definitions
        ↓
5. Calculate nullability
        ↓
6. Build left-corner graph
        ↓
7. Find strongly connected components
        ↓
8. Analyze guaranteed token consumption
        ↓
9. Detect direct left recursion
        ↓
10. Detect indirect left recursion
        ↓
11. Detect nullable-prefix recursion
        ↓
12. Detect nullable repetition
        ↓
13. Detect precedence cycles
        ↓
14. Validate canonical ownership
        ↓
15. Generate ANTLR parser
        ↓
16. Validate generated grammar
        ↓
17. Run Rust parser conformance
        ↓
18. Compare AST results
        ↓
19. Run semantic/IR conformance
        ↓
20. Run positive/negative/boundary/scalability tests

A production build MUST fail when a mandatory validation stage fails.

---

99. Production Acceptance Criteria

The grammar/ subsystem satisfies this document only when:

Structural

- [ ] No direct left recursion.
- [ ] No indirect left recursion.
- [ ] No nullable-prefix left recursion.
- [ ] No zero-consumption recursive cycle.
- [ ] No nullable repetition.
- [ ] No precedence cycle.
- [ ] No duplicate public expression hierarchy.
- [ ] No duplicate public type hierarchy.
- [ ] Imported grammar dependencies are resolvable.
- [ ] Recursive SCCs have explicit consumption proofs.

Parser

- [ ] ANTLR generation succeeds.
- [ ] Rust parser succeeds.
- [ ] Rust parser and ANTLR agree on precedence.
- [ ] Parser recovery makes progress.
- [ ] Deep input does not trigger infinite recursion.
- [ ] No unsafe Rust is required.

AST

- [ ] Expression associativity is preserved.
- [ ] Operator precedence is preserved.
- [ ] Source spans are preserved.
- [ ] Nested structures map to the existing AST.
- [ ] No backend-specific AST nodes are required merely to remove left recursion.

Domain integration

- [ ] Classical grammar is covered.
- [ ] Quantum grammar is covered.
- [ ] Hybrid grammar is covered.
- [ ] HDL grammar is covered.
- [ ] Hardware grammar is covered.
- [ ] Distributed grammar is covered.
- [ ] AI grammar is covered.
- [ ] Data grammar is covered.
- [ ] Networking grammar is covered.
- [ ] Security grammar is covered.
- [ ] Resource grammar is covered.
- [ ] Macro grammar is covered.
- [ ] Metaprogramming grammar is covered.
- [ ] Dialect grammar is covered.
- [ ] Interoperability grammar is isolated or validated.

Scalability

- [ ] No artificial grammar recursion maximum exists.
- [ ] No hardware-specific parser limit exists.
- [ ] No fixed quantum limit exists.
- [ ] No fixed tensor-depth limit exists.
- [ ] No fixed namespace-depth limit exists.
- [ ] No fixed expression-chain limit exists.
- [ ] Resource budgets remain implementation concerns.

POCO-REAF

- [ ] Parsing is hardware independent.
- [ ] Parsing is deterministic.
- [ ] Parsing does not inspect target capabilities.
- [ ] Parsing does not inspect runtime state.
- [ ] Source semantics do not change with target size.

---

100. Final Invariant

The production Zamani grammar MUST satisfy:

No unintended left recursion
        +
No zero-consumption cycle
        +
No nullable repetition
        +
One canonical precedence hierarchy
        +
One canonical type hierarchy
        +
One canonical parser composition
        +
One domain-neutral AST
        +
Deterministic Rust/ANTLR conformance
        +
No artificial scalability limits
        =
Production grammar

The fundamental parser invariant is:

Every recursive grammar path either:

    1. terminates without recursion,

or:

    2. consumes a syntactically guaranteed token before recursive re-entry.

Therefore:

A → A α

is forbidden.

But:

A → TOKEN A

may be valid.

Likewise:

A → B*

is valid only when:

B

cannot derive:

ε

The validator MUST prove these properties structurally rather than relying on parser-generator behavior.

---

101. Final Zamani Parsing Architecture

The final architecture is:

                         Zamani Source
                              │
                              ▼
                         Zamani Lexer
                              │
                              ▼
                         Token Stream
                              │
                              ▼
                     Canonical Parser
                              │
                  ┌───────────┴───────────┐
                  │                       │
             ANTLR grammar           Rust parser
                  │                       │
                  └───────────┬───────────┘
                              │
                              ▼
                    Domain-Neutral AST
                              │
                              ▼
                    Structural Validation
                              │
             ┌────────────────┼────────────────┐
             ▼                ▼                ▼
       Name Resolution    Type Analysis    Effect Analysis
             │                │                │
             └────────────────┼────────────────┘
                              ▼
                Resource / Capability Analysis
                              │
                              ▼
                       Semantic Model
                              │
             ┌────────────────┼────────────────┐
             ▼                ▼                ▼
        Classical         quantum::ir       HDL/Hardware
                              │
                              ▼
                         Optimization
                              │
                ┌─────────────┼─────────────┐
                ▼             ▼             ▼
             Routing      Scheduling     Resilience
                │             │             │
                └─────────────┼─────────────┘
                              ▼
                             ZQN
                              │
                              ▼
                             HAL
                              │
                              ▼
                       Target Realization

Left-recursion validation belongs entirely in the parser/grammar correctness portion of this architecture.

It MUST NOT leak target information upward.

---

102. Final POCO-REAF Rule

The grammar must describe:

what the program means

not:

how many machines exist
how many cores exist
how many qubits exist
how many GPUs exist
how many FPGA resources exist
how much memory exists
how many nodes exist

The parser therefore remains independent of:

CPU
GPU
FPGA
ASIC
QPU
simulator
accelerator
cluster
cloud
future hardware

The same grammar can therefore accept one program whose eventual realization may range from:

tiny embedded execution
        ↓
CPU
        ↓
multicore
        ↓
GPU
        ↓
FPGA
        ↓
ASIC
        ↓
QPU
        ↓
quantum simulator
        ↓
accelerator
        ↓
HPC
        ↓
distributed cluster
        ↓
cloud
        ↓
future architecture

subject to actual program semantics and available resources.

That is the required grammar-level foundation for:

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever

---

103. Completion Statement

"grammar/validation/left-recursion.md" is the normative authority for left-recursion validation.

It does not require renaming:

Zamani.g4
grammar.md
Zamani-Grammar.md
grammar/antlr/ZamaniParser.g4
grammar/antlr/ZamaniLexer.g4
grammar/validation/ambiguity-rules.md

It instead integrates with the existing architecture.

The required architectural correction is:

ANTLR-supported left recursion
        ↓
NOT a canonical Zamani design permission
        ↓
canonical grammar uses explicit non-left-recursive structure
        ↓
Rust parser uses the same precedence model
        ↓
AST remains unchanged semantically
        ↓
semantic/IR layers remain downstream

The production rule is therefore:

«Canonical Zamani grammar MUST contain no unintended left recursion, whether direct, indirect, nullable-prefix, imported, generated, dialect-induced, macro-induced, or precedence-induced. Recursive syntax remains fully permitted where recursion crosses a guaranteed consuming boundary.»