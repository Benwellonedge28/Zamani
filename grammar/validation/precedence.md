Zamani Grammar Precedence Validation

Path: "grammar/validation/precedence.md"
Status: Normative validation contract
Authority: "grammar/DESIGN.md" → "grammar/specification/" → "grammar/spec/" → canonical precedence contract → executable grammar/lexer/parser → AST → semantic analysis → canonical IR
Scope: Lexical-token interaction, expression precedence, associativity, ambiguity detection, parser conformance, AST/semantic/IR traceability, scalability, determinism, and compatibility
Rust baseline: Rust 1.97 / Rust 1.97.1
Safety: Safe Rust only; "unsafe" is prohibited
Primary principle: Precedence is a language-semantic rule, never a hardware/resource limit.

---

1. Purpose

This document defines the production validation contract for operator precedence and associativity throughout the Zamani language.

It exists to ensure that:

1. every expression has deterministic interpretation;
2. precedence is defined exactly once;
3. associativity is defined exactly once;
4. lexical token identity is separated from semantic meaning;
5. parser behavior agrees with the normative precedence model;
6. AST construction preserves the intended grouping;
7. semantic analysis never has to guess parser intent;
8. lowering to canonical IR preserves expression meaning;
9. quantum, classical, HDL, AI, distributed, networking, memory, and other domains share the same core expression rules unless an explicitly specified domain extension applies;
10. dialects cannot silently redefine core precedence;
11. macros cannot bypass precedence validation;
12. generated parsers cannot drift from the canonical specification;
13. precedence validation remains deterministic;
14. no machine, hardware, register, thread, qubit, tensor, memory, node, or device capacity becomes a language-level limit;
15. the implementation remains scalable from tiny programs to programs limited only by available resources.

This document is a validation contract.

It does not replace the normative expression grammar.

The canonical precedence definition belongs to:

"grammar/expressions/precedence.md"

This document validates that all executable grammar and implementation layers conform to that authority.

---

2. Authority and Ownership

Precedence must have one normative owner.

The authority chain is:

grammar/DESIGN.md
        │
        ▼
grammar/specification/lexical.md
grammar/specification/syntax.md
grammar/specification/semantics.md
        │
        ▼
grammar/spec/lexical.md
grammar/spec/syntax.md
        │
        ▼
grammar/expressions/precedence.md
        │
        ├── lexer/tokens.md
        ├── lexer/operators.md
        ├── expressions/*.g4
        ├── statements/*.g4
        ├── types/*.g4
        └── domain grammars
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
semantic analysis
        │
        ▼
canonical semantic model
        │
        ├── classical IR
        ├── quantum::ir
        └── HDL/hardware-oriented IR
        │
        ▼
optimization / lowering / routing / scheduling / resilience / ZQN
        │
        ▼
target realization

No downstream file may redefine precedence independently.

---

3. Relationship to "grammar/expressions/precedence.md"

"grammar/expressions/precedence.md" defines:

- the canonical precedence hierarchy;
- the canonical associativity of each operator class;
- grouping constructs;
- operator families;
- domain-independent expression composition;
- explicitly documented domain-specific exceptions.

This file defines how those rules are validated.

Therefore:

File| Responsibility
"expressions/precedence.md"| Defines precedence
"validation/precedence.md"| Validates precedence
"lexer/operators.md"| Defines lexical operator identity
"lexer/tokens.md"| Defines canonical token identity
"expressions/*.g4"| Implements expression syntax
"Zamani.g4"| Composes grammar
"src/lexer.rs"| Implements lexical recognition
"src/parser.rs"| Implements parsing
"src/frontend/ast/"| Represents structure
semantic analysis| Determines semantic validity
canonical IR| Represents executable meaning

No file outside the canonical precedence authority may silently change the ordering.

---

4. Core Invariants

A production-ready Zamani parser must satisfy all of the following invariants.

4.1 One operator, one canonical lexical identity

A single exact lexeme must not produce multiple competing token identities.

Existing canonical mappings include:

Lexeme| Canonical token
"?"| "QuestionMark"
"&"| "Ampersand"
`| `
"->"| "ThinArrow"

The following duplicate concepts must not coexist as separate emitted tokens for the same exact spelling:

Question
QuestionMark

Ampersand
BitAnd

Arrow
ThinArrow

BitOr
Pipe

unless their lexical spellings are genuinely different.

Semantic meaning is determined downstream.

For example, "&" may participate in different syntactic or semantic constructs, but the lexer must not create separate token identities merely because the eventual meaning differs.

---

5. Precedence Is Not Lexical Meaning

The lexer identifies tokens.

The parser determines structural grouping.

Semantic analysis determines meaning.

Therefore:

lexeme
  ↓
token
  ↓
grammar position
  ↓
AST structure
  ↓
semantic interpretation

must not be collapsed into:

lexeme
  ↓
pre-selected semantic operation

For example:

a & b

must first produce:

Identifier
Ampersand
Identifier

The parser and semantic layer determine which valid language construct that represents.

The lexer must not emit:

BitAnd
BorrowOperator
ReferenceOperator

as separate tokens for the same "&".

---

6. Canonical Precedence Model

The canonical expression hierarchy must be represented by "grammar/expressions/precedence.md".

The validator must verify the implementation against that hierarchy.

The implementation should conceptually follow a structure equivalent to:

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
prefix/unary
    ↓
postfix
    ↓
primary

This ordering is a validation model.

The authoritative operator list and exact ordering remain in:

grammar/expressions/precedence.md

If the canonical file differs, the canonical file wins and this validator must report the implementation drift.

---

7. No Precedence by Hardware or Execution Target

Precedence must never depend on:

- CPU;
- GPU;
- FPGA;
- ASIC;
- QPU;
- accelerator;
- simulator;
- operating system;
- runtime;
- compiler optimization level;
- number of cores;
- number of threads;
- memory capacity;
- cluster size;
- distributed node count;
- device count;
- target topology.

The expression:

a + b * c

must mean the same structural expression regardless of where it eventually executes.

Optimization may transform the representation only after semantic meaning has been established.

---

8. Associativity

Every operator class must have an explicitly documented associativity.

Allowed classifications include:

left
right
non-associative
chainable
context-defined

No operator may be left with implicit associativity.

For a left-associative operator:

a op b op c

must be represented as:

(a op b) op c

For a right-associative operator:

a op b op c

must be represented as:

a op (b op c)

For a non-associative operator:

a op b op c

must be rejected unless the specification explicitly defines chaining.

For a chainable operator:

a < b < c

must use the exact semantic model defined by the language specification.

The validator must not infer chain semantics merely because the grammar can parse them.

---

9. Parentheses and Explicit Grouping

Parentheses must override ordinary operator precedence.

For example:

a + b * c

and:

(a + b) * c

must produce structurally different ASTs.

The validator must verify:

1. grouping is preserved;
2. redundant grouping does not change semantics unless explicitly specified;
3. grouping cannot bypass type/effect/resource validation;
4. grouping does not create target-specific semantics;
5. source spans include the correct grouping boundaries.

---

10. Primary Expressions

Primary expressions form the base of the precedence hierarchy.

They include only constructs explicitly accepted by the canonical expression specification, such as applicable:

- literals;
- identifiers;
- names;
- grouped expressions;
- array literals;
- tuple literals;
- block expressions;
- lambda expressions;
- closures;
- domain-defined primary constructs.

The validator must verify that primary expressions do not accidentally participate in lower-level operator recursion.

---

11. Postfix Expressions

Postfix constructs normally bind more tightly than prefix and binary operators.

Where applicable:

call
index
member access
postfix operator
generic application
other canonical postfix constructs

must be validated against the authoritative hierarchy.

For example, where the language permits:

f(x).field[i]

the AST must preserve:

index(
    member(
        call(f, x),
        field
    ),
    i
)

or the equivalent canonical AST structure.

The exact AST node names are owned by the AST contract, not this document.

---

12. Prefix / Unary Expressions

Prefix operators must have an explicitly defined binding strength.

For example, if both exist:

- x ^ y

the language specification must decide whether the structure is equivalent to:

(-x) ^ y

or:

-(x ^ y)

The parser must not decide this accidentally based on implementation convenience.

The validator must include every prefix operator declared by the canonical operator registry.

---

13. Binary Operators

Every binary operator must have:

- canonical token;
- precedence level;
- associativity;
- grammar owner;
- AST mapping;
- semantic operation;
- diagnostic behavior;
- test coverage.

No binary operator may exist only in:

Zamani.g4

without corresponding lexical, AST, semantic, and test contracts.

---

14. Ternary / Conditional Expressions

Conditional expression precedence must be validated independently from statement-level control flow.

Expression-level conditional constructs must not be confused with statement-level:

if
else

structures.

Where the canonical expression grammar uses:

condition ? then_expression : else_expression

the validator must verify:

1. "?" is the canonical "QuestionMark" token;
2. ":" is the canonical colon token;
3. condition precedence is correct;
4. the then-expression boundary is deterministic;
5. the else-expression is parsed according to its documented associativity;
6. nested conditionals produce the intended AST;
7. statement-level "if" does not redefine expression precedence.

The existing conditional-expression work establishes the canonical form around:

rangeExpression QUESTION expression COLON conditionalExpression

with right-associative nesting.

That contract must remain aligned with this validator.

---

15. Range Expressions

Range operators must have one canonical precedence location.

The validator must test interactions with:

- arithmetic;
- indexing;
- slicing;
- comparisons;
- conditionals;
- assignment;
- function arguments;
- collection construction.

Examples such as:

a + b .. c

must never have implementation-defined grouping.

The exact intended grouping comes from the canonical precedence authority.

---

16. Assignment

Assignment must have explicitly defined precedence and associativity.

If assignment is right-associative, for example:

a = b = c

the AST must reflect the canonical right-associated structure.

If chained assignment is not legal, the validator must require rejection.

Assignments must not accidentally absorb:

- declarations;
- type annotations;
- effect handlers;
- resource requirements;
- hardware mappings;
- quantum operation modifiers.

---

17. Logical Operators

Logical operators must have stable ordering.

For example, if the language defines logical conjunction more tightly than logical disjunction:

a || b && c

must have the canonical structure:

a || (b && c)

The validator must verify both parsing and AST construction.

Short-circuit behavior is a semantic/runtime contract and must not be encoded as a parser-side optimization.

---

18. Bitwise Operators

Bitwise precedence must be distinct from logical precedence if the language specification distinguishes them.

The validator must ensure that:

&
|

do not receive multiple lexical identities merely because their semantic contexts differ.

For the canonical lexemes:

& → Ampersand
| → Pipe

the parser and semantic analyzer determine the valid operation.

A duplicate token such as:

BitAnd
BitOr

must not be introduced merely to make precedence implementation easier.

If a genuinely different bitwise spelling is added in the future, it receives a distinct canonical token only because the lexeme is different.

---

19. Equality and Relational Operators

Equality and relational operators must have independently validated precedence.

The validator must test:

a == b < c

and equivalent mixed forms against the canonical specification.

The parser must never rely on downstream type checking to repair an incorrectly grouped expression.

Semantic invalidity and syntactic grouping are separate concerns.

---

20. Shift Operators

Shift operators must occupy an explicitly documented precedence level.

They must be tested against:

- additive operators;
- multiplicative operators;
- comparisons;
- bitwise operators;
- assignment.

For example:

a + b << c

must have exactly one canonical parse.

---

21. Arithmetic Operators

Additive and multiplicative operators must have deterministic ordering.

For example:

a + b * c

must not produce multiple possible ASTs.

Similarly:

a * b + c

must not require semantic analysis to choose the grouping.

The parser must already produce the canonical structure.

---

22. Domain-Specific Operators

Quantum, HDL, AI, distributed, networking, memory, and other domains may introduce operators.

They must not automatically create a new global precedence level.

Every proposed domain operator must declare:

operator identifier
lexeme
token
domain
precedence class
associativity
AST representation
semantic operation
IR representation
feature status
version
dialect
compatibility

The precedence validator then verifies whether the operator belongs to:

1. an existing universal precedence level; or
2. an explicitly approved domain-specific precedence level.

A domain must not silently redefine an existing operator's precedence.

---

23. Quantum Precedence

Quantum operations must use the generic operation model already established for Zamani.

The grammar must not use a fixed gate enumeration to solve precedence.

For example:

apply H to q
apply custom_gate to q
apply vendor.operation to q
apply operation(parameter) to q

must be represented through the generic quantum operation syntax.

If a quantum expression contains ordinary operators, the ordinary expression precedence rules remain authoritative.

Quantum-specific grouping must be defined explicitly where quantum syntax requires it.

The validator must verify that:

quantum expression
    ↓
AST
    ↓
semantic quantum operation
    ↓
quantum::ir

preserves the parser's grouping.

No quantum hardware topology can alter precedence.

---

24. HDL Precedence

HDL operators must not inherit accidental precedence from textual similarity to software operators.

Every HDL operator must be assigned an explicit precedence contract.

Hardware timing, clocking, reset, signal width, synthesis, and physical implementation are semantic concerns.

They must not change ordinary expression grouping.

For example, if an HDL construct embeds an expression:

signal = a + b * c;

the arithmetic grouping must remain deterministic.

---

25. Resource and Capability Expressions

Resource expressions such as:

requires qubits >= n
requires memory >= required_memory
requires capability("tensor.compute")
requires topology(...)

must obey the ordinary expression precedence model.

The validator must distinguish:

syntax

from:

resource semantics

The presence of a resource expression must never introduce an implicit machine-size limit.

---

26. Precedence and Type Expressions

Type syntax must not accidentally enter expression precedence.

Examples that must be separately defined include:

T<U>
T[a]
T?
&T
fn(T) -> U

The parser must determine whether a token sequence is:

- a type;
- an expression;
- a generic argument;
- an index;
- a function type;
- another grammar construct.

This distinction belongs to grammar structure and contextual parsing, not arbitrary precedence hacks.

---

27. Generic Arguments vs Comparison Operators

Sequences such as:

Type<A, B>

must not be accidentally interpreted as ordinary comparison expressions.

If generic syntax and comparison syntax can share token sequences, the canonical grammar must resolve the ambiguity deterministically.

The validator must test:

Type<A>
a < b
a < b > c
Type<A<B>>

and all equivalent nested forms supported by the specification.

The solution must not depend on target hardware or compiler resource availability.

---

28. Member Access and Qualified Names

Qualified names such as:

vendor.operation
module.item
namespace::item

must be validated against member-access, path, and operator precedence.

A qualified name must not accidentally become:

identifier
operator
identifier

unless that is the intended grammar.

This is especially important for:

- quantum operations;
- dialect names;
- modules;
- interoperability formats;
- hardware capabilities;
- vendor namespaces.

---

29. Macro Expansion

Macro expansion must not invalidate precedence.

There are two validation stages:

source syntax
    ↓
pre-expansion validation

and:

expanded token/tree
    ↓
post-expansion validation

A macro must not rely on token adjacency to change operator grouping unexpectedly.

Macros must either:

1. emit syntax whose grouping is explicit; or
2. emit a syntax tree whose structure is already known.

Hygiene and precedence are related but separate:

- "macros/hygiene.g4" owns hygiene;
- "validation/precedence.md" validates resulting expression structure.

Macro-generated identifiers and operators remain subject to the same canonical lexical and precedence contracts.

---

30. Metaprogramming

Generated syntax must be validated exactly like handwritten syntax.

Reflection, quotation, code generation, and compile-time construction cannot create an alternate precedence system.

Generated expression trees must satisfy:

canonical precedence
canonical associativity
canonical AST structure
canonical semantic rules

before they enter later compiler phases.

---

31. Dialects

A dialect may add syntax only through an explicit dialect contract.

A dialect must not silently:

- change precedence of a core operator;
- change associativity of a core operator;
- reinterpret an existing operator;
- introduce a conflicting token;
- make a previously unambiguous core expression ambiguous.

A dialect extension must declare:

dialect
version
operator
token
precedence
associativity
scope
feature gate
AST mapping
semantic mapping
IR mapping
compatibility policy

Core Zamani precedence remains stable unless a language-version change explicitly changes the normative specification.

---

32. Versioning

Precedence is part of language compatibility.

Changing:

operator precedence

or:

operator associativity

is a potentially source-breaking language change.

Therefore a precedence change requires:

1. specification change;
2. version classification;
3. compatibility analysis;
4. migration guidance;
5. parser tests;
6. AST tests;
7. semantic tests;
8. regression tests;
9. diagnostic tests;
10. compatibility tests.

No implementation may silently change precedence because a parser generator makes another grammar shape convenient.

---

33. Parser Implementation Requirements

The parser must implement the canonical precedence model.

Acceptable implementation strategies include:

- explicitly layered grammar rules;
- precedence-aware recursive descent;
- Pratt parsing;
- another deterministic precedence mechanism.

The implementation technique is not normative.

The resulting parse is normative.

The validator must compare observable AST structure against the canonical precedence contract.

---

34. ANTLR Integration

"grammar/Zamani.g4" remains the composition root.

ANTLR grammar fragments may implement precedence rules, but they must not create independent precedence authorities.

If ANTLR uses precedence mechanisms internally, the generated behavior must conform to:

grammar/expressions/precedence.md

The validator must detect:

- conflicting precedence declarations;
- duplicated precedence rules;
- unreachable precedence alternatives;
- ambiguous alternatives;
- accidental left recursion;
- accidental right recursion;
- precedence inversion;
- operator omission;
- operator duplication.

Generated ANTLR files must not be manually edited.

The source grammar and canonical specification are the editable authorities.

---

35. Rust Lexer Integration

"src/lexer.rs" must agree with:

lexer/tokens.md
lexer/operators.md
lexer/keywords.md
specification/lexical.md

The lexer owns tokenization, not precedence.

Therefore the lexer must not encode semantic precedence decisions.

It must correctly recognize multi-character operators before their shorter prefixes.

For example:

-> 

must not be incorrectly emitted as:

-
>

when "->" is a canonical operator.

Likewise, exact lexical identity must remain canonical:

?  → QuestionMark
&  → Ampersand
|  → Pipe
-> → ThinArrow

---

36. Longest-Match Rule

Where multiple lexical tokens share a prefix, lexical recognition must use deterministic maximal matching according to the canonical lexical specification.

For example:

longer valid operator

must be recognized before:

shorter prefix operator

This is a lexical rule.

It must not be confused with expression precedence.

The validator must test all operator-prefix relationships declared by the canonical token registry.

---

37. Identifier Boundaries

Keyword matching must respect identifier boundaries.

For example, if "if" is a keyword:

if

may be a keyword while:

iffy

remains an identifier.

The parser must not produce:

IF + Identifier("fy")

merely because a keyword is a prefix of an identifier.

This validation belongs jointly to:

keyword-collisions.md

and lexical validation.

---

38. AST Contract

Every precedence-sensitive expression must have an unambiguous AST representation.

The AST must preserve:

- operator;
- operand order;
- grouping;
- source spans;
- nesting;
- associativity;
- relevant modifiers;
- domain metadata where specified.

The AST must not contain multiple node variants merely because two lexical token names were accidentally created for the same operator.

For example, the AST should not need:

BitAndExpression
AmpersandExpression

solely because the lexer previously had both:

BitAnd
Ampersand

for "&".

---

39. AST Structural Invariant

For every expression "E":

parse(E)

must produce exactly one canonical structural interpretation.

Equivalent source spellings may be accepted where the language specification explicitly defines aliases.

But each accepted spelling must map to one canonical semantic structure.

---

40. Semantic Integration

Semantic analysis must never repair a parser ambiguity.

Invalid:

parser creates multiple interpretations
↓
semantic analyzer chooses one

Required:

source
↓
one deterministic parse
↓
one AST
↓
semantic validation

Semantic analysis may reject an AST because of:

- type mismatch;
- invalid operand;
- invalid effect;
- invalid resource requirement;
- unavailable capability;
- invalid quantum operation;
- invalid hardware intent;
- invalid ownership;
- invalid lifetime;
- invalid domain constraint.

It must not decide which precedence interpretation the source intended.

---

41. Canonical IR Integration

The canonical IR must preserve the meaning represented by the AST.

For every precedence-sensitive construct:

source
→ tokens
→ AST
→ semantic model
→ IR

must preserve operand ordering and grouping.

Optimizations may transform:

(a + b) + c

into an equivalent representation only after semantic correctness has been established.

Optimization must never use an incorrect parse as input.

---

42. Quantum IR Integration

Quantum constructs must lower through the canonical:

quantum::ir

boundary.

There must not be:

frontend quantum precedence
↓
temporary private quantum IR
↓
second quantum IR

The parser's grouping must survive the quantum semantic mapping.

Quantum-specific optimization may later transform the canonical representation.

---

43. Optimization Invariant

Optimization is allowed to change representation.

It is not allowed to change language meaning.

Therefore every optimization pass must satisfy:

semantic(parse(source))
≈
semantic(optimize(parse(source)))

where equivalence is defined by the language/compiler semantic model.

For quantum programs, equivalence must use the canonical quantum semantic/IR contract.

For HDL, equivalence must respect the HDL semantic contract.

For distributed programs, equivalence must respect concurrency/distribution semantics.

---

44. Floating-Point and Algebraic Reassociation

The parser must never assume mathematical associativity merely because an operator appears associative mathematically.

For example:

(a + b) + c

and:

a + (b + c)

may differ under finite-precision arithmetic.

Therefore:

- parser associativity is a language rule;
- semantic associativity is a type/operation property;
- optimizer reassociation is a legality question.

The optimizer must not infer reassociation merely from the grammar.

---

45. Side Effects

Expressions containing side effects must preserve evaluation ordering specified by the language.

For example, if function calls are permitted as operands:

f() + g() * h()

the parser establishes grouping, but evaluation order must be defined by the semantic/execution specification.

Precedence must never be used as an implicit evaluation-order specification.

---

46. Effects

Effectful expressions must preserve effect structure.

An expression's precedence does not determine:

- effect sequencing;
- concurrency scheduling;
- resource acquisition;
- device placement;
- quantum measurement ordering.

Those are owned by the semantic/effect/execution layers.

---

47. Concurrency

Precedence must remain independent from execution parallelism.

An expression such as:

a + b * c

has one structural meaning regardless of whether its operands are evaluated:

- sequentially;
- concurrently;
- on a CPU;
- on a GPU;
- on an FPGA;
- on a QPU;
- across a cluster.

The concurrency system may determine execution strategy only after semantic meaning is established.

---

48. Hardware and Resource Independence

The validator must reject designs that introduce constructs such as:

precedence depends on core count
precedence depends on SIMD width
precedence depends on register width
precedence depends on GPU count
precedence depends on QPU topology

Precedence is language semantics.

It is not target configuration.

---

49. No Artificial Capacity Limits

Precedence validation must not introduce constants such as:

MAX_OPERATORS
MAX_PRECEDENCE_LEVELS
MAX_EXPRESSION_DEPTH
MAX_OPERANDS
MAX_NESTING
MAX_AST_DEPTH
MAX_TOKENS
MAX_RULES

as language-level ceilings.

Implementation resources may naturally constrain an individual compilation process.

Those are environmental/resource constraints, not language semantics.

A validator may reject an input only when actual available resources are exhausted or when a configured operational policy explicitly applies.

It must not pretend that a finite implementation constant is a language maximum.

---

50. Scalability Model

The precedence validator must scale with:

N = number of declared operators/rules
M = total operator/grammar metadata
L = total source/token length
D = actual expression nesting depth

The validator must not depend on a fixed maximum value for any of these.

The design should use data structures that scale with actual input.

Preferred Rust structures include:

BTreeMap
BTreeSet
Vec
VecDeque

where deterministic ordering is required.

"HashMap"/"HashSet" may be used only where nondeterministic iteration cannot leak into observable diagnostics, or results must be explicitly sorted before reporting.

---

51. Deterministic Validation

Given identical:

specification
operator registry
grammar
lexer configuration
dialect set
language version
source

the validator must produce identical:

- acceptance/rejection;
- precedence result;
- associativity result;
- diagnostic codes;
- source spans;
- diagnostic ordering;
- AST structure.

Results must not depend on:

- hash seed;
- thread scheduling;
- CPU architecture;
- GPU availability;
- number of cores;
- machine memory size;
- target device;
- distributed execution order.

---

52. Parallel Validation

Validation may be parallelized internally.

However, observable results must remain deterministic.

If independent validation tasks run concurrently:

operator validation
grammar validation
AST validation
test validation

their diagnostics must be merged using a stable ordering.

A suitable ordering is:

source location
→ diagnostic code
→ operator/token identifier
→ stable message key

or another documented total ordering.

---

53. Safe Rust Requirements

Any executable precedence validator must be implemented in safe Rust.

Required:

Rust 1.97 or Rust 1.97.1
Rust 2021
no unsafe

Prohibited:

unsafe
static mut
raw-pointer based core validation
undefined behavior assumptions

The validator should prefer ownership-safe structures from the standard library.

The grammar specification itself must not require unsafe implementation techniques.

---

54. Recursion and Deep Expressions

The language must not impose an artificial semantic maximum expression depth.

However, an implementation must protect itself from actual resource exhaustion.

The implementation should therefore distinguish:

language expressiveness

from:

implementation resource exhaustion

A parser may use an iterative strategy where practical.

If recursion is used, stack exhaustion must be treated as an implementation/resource failure, not a language-level precedence rule.

Tests must include increasingly deep expressions without declaring an arbitrary language maximum.

---

55. Grammar Ambiguity Detection

The validator must detect:

- multiple parses for the same token sequence;
- hidden ambiguity caused by optional rules;
- ambiguous prefix operators;
- ambiguous postfix operators;
- overlapping binary operators;
- conflicting conditional syntax;
- generic-vs-comparison ambiguity;
- path-vs-member ambiguity;
- macro-generated ambiguity;
- dialect-generated ambiguity.

An expression is production-ready only when its valid parse is deterministic.

---

56. Unreachable Precedence Rules

The validator must identify precedence rules that cannot be reached because an earlier rule always consumes the same token sequence.

This includes:

shadowed operator
shadowed grammar alternative
unreachable postfix form
unreachable binary alternative
unreachable dialect extension

A rule may intentionally be unreachable only if the specification explicitly marks it as generated/documentational and it is excluded from executable grammar.

---

57. Duplicate Precedence Declarations

A canonical operator must not be assigned two precedence levels.

For every operator:

operator → exactly one canonical precedence class

unless the specification explicitly defines contextual precedence.

Contextual precedence must still have deterministic grammar rules.

---

58. Contextual Operators

If the same token can participate in different syntactic constructs, the validator must distinguish:

lexical identity

from:

grammar role

For example:

&
|
?

must not become multiple lexical tokens simply because they have multiple grammatical roles.

Contextual interpretation belongs to parser/semantic grammar.

---

59. Keyword Interaction

Keywords must not accidentally become operators.

For example, if a word is a keyword:

keyword

its precedence behavior must be explicitly defined by the grammar.

Keyword-like constructs must not be implemented through arbitrary precedence tricks.

Keyword collisions are validated primarily by:

validation/keyword-collisions.md

while this file validates their expression-grouping consequences.

---

60. Literal Interaction

Operators adjacent to literals must remain deterministic.

Examples include:

-1
+1
1..n
1..=n
0xFF << n

The validator must distinguish:

negative literal

from:

prefix negation applied to literal

according to the canonical lexical and semantic specification.

This distinction must not be left implicit.

---

61. Numeric Literals

Numeric magnitude is not a precedence limitation.

The parser must not impose language-level maximums such as:

32-bit literal
64-bit literal
128-bit literal

unless a particular type explicitly defines that representation.

Arbitrary-magnitude literals must remain compatible with the canonical literal/type system.

---

62. Quantum Literals

Quantum literals such as:

|0⟩
|1⟩
|+⟩
|-⟩
|ψ⟩

must be lexically unambiguous with:

Pipe

and other expressions.

The lexer must recognize the canonical quantum-literal forms according to "lexer/quantum-literals.md".

The parser must not accidentally reinterpret a complete quantum literal as:

Pipe
expression
Pipe

unless the grammar explicitly intends that representation.

---

63. Pipeline-Like Syntax

If Zamani defines pipeline constructs involving "|", their precedence must be specified independently of the token identity:

| → Pipe

The parser must determine whether "Pipe" represents:

- pipeline syntax;
- bitwise operation;
- pattern syntax;
- another valid construct.

The lexical layer must remain unchanged.

---

64. Arrow Syntax

For:

-> → ThinArrow

the precedence/grammar contract must distinguish:

- function types;
- return types;
- mappings;
- lambda syntax;
- other arrow-based constructs.

"Arrow" must not become a second token for the same "->" lexeme.

If another arrow spelling is introduced, it must have a distinct lexical spelling and independently specified precedence/grammar role.

---

65. Question Mark Syntax

For:

? → QuestionMark

the validator must distinguish all valid roles through grammar context.

For example, depending on the canonical specification, "?" may participate in:

- conditional expressions;
- optional types;
- propagation constructs;
- other syntax.

The exact role must be documented in the relevant grammar contracts.

One lexeme must not produce multiple token identities merely because its grammatical roles differ.

---

66. Ampersand Syntax

For:

& → Ampersand

the validator must ensure that:

- reference syntax;
- borrowing;
- bitwise syntax;
- other permitted roles

do not require duplicate lexical tokens.

The parser/semantic system must distinguish the roles structurally.

---

67. Parser Error Recovery

Error recovery must not silently choose a precedence interpretation that the valid grammar would reject.

For invalid source:

a + * b

the parser may recover for diagnostic purposes, but the recovered tree must not be presented as a valid canonical AST.

Diagnostics must identify:

- location;
- unexpected token;
- expected construct where available;
- relevant operator/precedence context;
- recovery status.

---

68. Diagnostic Requirements

Production precedence diagnostics should be stable and machine-readable.

Recommended diagnostic categories:

PREC001  precedence conflict
PREC002  associativity conflict
PREC003  ambiguous expression
PREC004  unreachable precedence rule
PREC005  duplicate precedence declaration
PREC006  missing precedence declaration
PREC007  operator registry drift
PREC008  lexer/parser precedence drift
PREC009  AST grouping mismatch
PREC010  semantic grouping mismatch
PREC011  dialect precedence conflict
PREC012  version precedence conflict
PREC013  macro expansion precedence violation
PREC014  generated grammar drift
PREC015  unsupported precedence extension
PREC016  invalid operator chaining
PREC017  invalid grouping
PREC018  literal/operator ambiguity
PREC019  generic/comparison ambiguity
PREC020  quantum expression grouping mismatch
PREC021  HDL expression grouping mismatch
PREC022  nondeterministic diagnostic ordering

Diagnostic identifiers are stable API.

Changing their meaning requires compatibility review.

---

69. Source Spans

Every precedence-related diagnostic must carry accurate source location information.

At minimum:

source file
start offset
end offset
line
column

where supported by the existing source-span contract.

For an operator conflict, the diagnostic should identify the operator token itself.

For grouping errors, the diagnostic should identify the smallest relevant expression span.

---

70. Error Recovery and AST Validity

Recovered parser nodes must be distinguishable from valid nodes.

A downstream semantic pass must not mistake:

error recovery

for:

successful precedence resolution

This prevents invalid source from reaching the IR as if it were valid.

---

71. Feature-Gate Integration

Precedence-bearing operators may be feature-gated.

The validator must verify:

operator
→ feature
→ version
→ dialect
→ availability

An inactive feature must not accidentally participate in the active precedence table.

A proposed or experimental operator must not silently become a stable core operator.

---

72. Status Integration

Operators and precedence rules must use the repository feature lifecycle:

stable
proposed
experimental
deprecated
historical
not implemented

Only executable statuses may participate in the active grammar.

Historical content in:

Zamani-Grammar.md

must never automatically alter the precedence table.

---

73. "grammar/grammar.md" Integration

"grammar/grammar.md" must report precedence implementation status.

For every precedence class, it should be possible to determine:

SPECIFIED
IMPLEMENTED
PARTIALLY IMPLEMENTED
PLANNED
DEPRECATED

This file must not become another authority.

If "grammar/grammar.md" disagrees with the executable parser, the conformance validator reports drift.

---

74. "Zamani-Grammar.md" Integration

"Zamani-Grammar.md" may contain:

- historical precedence designs;
- proposed operators;
- experimental constructs;
- alternative syntax.

Those entries must carry status markers.

Only promoted features enter the canonical precedence contract.

Promotion requires:

proposal
→ semantic design
→ AST contract
→ grammar
→ implementation
→ IR
→ tests
→ stable

---

75. "Zamani.g4" Integration

"Zamani.g4" must remain the composition root.

It must not independently redefine precedence.

The root grammar may invoke expression rules, but expression precedence belongs to the expression grammar/precedence authority.

A feature must not be implemented once in:

expressions/*.g4

and again independently in:

Zamani.g4

because that creates two parser authorities.

---

76. Expression Grammar Integration

"grammar/expressions/" is responsible for implementing the canonical precedence model.

The validator must verify:

expressions/precedence.md
        ↕
expressions/*.g4

for every operator.

No operator may be missing from the canonical table.

No executable operator may exist outside the canonical table.

---

77. Statement Integration

Statement-level constructs may contain expressions.

Examples:

let
return
if
while
for
match
assert
resource statements
quantum statements
HDL statements

Whenever a statement embeds an expression, it must use the canonical expression entry point.

Statements must not create alternate expression precedence.

---

78. Declaration Integration

Declarations may contain:

- initializer expressions;
- default arguments;
- array sizes;
- generic constraints;
- resource requirements;
- capability expressions;
- type expressions.

Each embedded expression must use the appropriate canonical expression/type grammar.

---

79. Type-System Integration

Where type-level operators exist, their precedence must be defined separately from value-expression operators where necessary.

The validator must prevent accidental reuse of value-expression precedence rules for type syntax.

Type operators must have:

type grammar owner
AST owner
semantic owner
compatibility owner
tests

---

80. Effects Integration

Effect expressions and handlers must use the canonical expression model unless explicitly specified otherwise.

Effect syntax must not change ordinary operator precedence.

---

81. Memory Integration

Memory expressions such as:

size
alignment
layout
address-space
allocation constraints

must use canonical expression precedence.

A memory implementation must not define its own arithmetic precedence.

---

82. Concurrency Integration

Concurrency constructs such as:

spawn
await
parallel
channels
synchronization

must use the canonical expression grammar for embedded expressions.

Concurrency scheduling must never alter parsing.

---

83. Classical Computing Integration

Classical mathematical expressions must use the universal precedence model.

Domain libraries may provide functions such as:

fft(...)
svd(...)
gradient(...)

without creating arbitrary parser-level precedence rules.

Mathematical associativity must remain distinct from parser associativity.

---

84. AI Integration

AI syntax may contain:

- tensor expressions;
- model expressions;
- training expressions;
- inference expressions;
- dataset expressions;
- optimization expressions.

Embedded expressions use the canonical precedence model.

Framework-specific semantics do not belong in precedence validation.

---

85. Data Integration

Data transformations and query expressions must either:

1. use the universal expression grammar; or
2. explicitly define a separate DSL/dialect grammar.

A data subsystem must not silently introduce a competing global precedence table.

---

86. Networking Integration

Networking expressions such as:

endpoint
route
protocol
filter
query
stream

must not modify global expression precedence.

---

87. Security Integration

Security policies may contain boolean, comparison, capability, and resource expressions.

They must use the canonical precedence hierarchy.

Security semantics may impose additional validity requirements but cannot change parsing meaning.

---

88. Distributed Integration

Distributed placement and topology expressions must use canonical precedence.

For example:

requires nodes >= n

is an expression whose resource meaning belongs downstream.

The parser must not interpret "n" according to a fixed machine capacity.

---

89. Hardware Integration

Hardware intent expressions must remain target-independent.

The validator must reject architecture designs in which precedence depends on:

CPU family
GPU vendor
FPGA family
QPU topology
register width
memory size
device count

Hardware realization occurs downstream.

---

90. Interoperability Integration

External formats such as:

- OpenQASM;
- QIR;
- HDL formats;
- LLVM/MLIR-related representations;
- foreign languages;

may have different precedence systems.

Their parsers must retain their own format semantics at the interoperability boundary.

They must be translated into Zamani's canonical semantic representation.

External precedence must never silently redefine Zamani precedence.

For quantum interoperability:

external format
→ format-specific frontend
→ canonical quantum::ir

not:

external precedence
→ Zamani parser precedence

---

91. Compatibility Integration

Backward-compatible syntax may preserve old spellings.

However, compatibility aliases must map to the canonical precedence model.

An old spelling must not create a second semantic precedence rule.

If historical syntax had different precedence, compatibility must be explicitly versioned.

Silent reinterpretation is prohibited.

---

92. Generated Code Integration

Generated parser/token files must be treated as derived artifacts.

Do not manually modify generated files to fix precedence.

The correction flow is:

authority
→ source grammar/specification
→ generator
→ generated parser
→ conformance tests

Generated output must be reproducible.

---

93. Registry Requirements

The canonical operator registry should contain enough information to validate precedence without requiring source-code heuristics.

Conceptually:

OperatorSpec {
    canonical_name
    token
    lexeme
    category
    precedence
    associativity
    arity
    domain
    status
    version
    dialect
    ast_mapping
    semantic_mapping
    ir_mapping
}

The actual executable representation may differ.

The information must exist somewhere authoritative.

---

94. Registry Invariants

For active operators:

one canonical token
one canonical precedence class
one canonical associativity
one canonical arity model
one AST mapping
one semantic mapping

Contextual operators may have multiple grammar roles, but they must still have a deterministic specification.

---

95. No Pairwise Explosion

The validator must not require manually maintaining every possible pair of operators.

Instead of storing:

A > B
A > C
A > D
...

it should derive relationships from canonical precedence levels.

For example:

precedence level 10
precedence level 20
precedence level 30

determines ordering automatically.

Explicit pairwise exceptions should exist only where the language semantics require them.

This makes the system scalable as operators are added.

---

96. Validation Algorithm

The production validator should conceptually perform:

1. Load canonical operator registry.
2. Load canonical precedence specification.
3. Load active language version.
4. Load active dialects/features.
5. Verify every active operator has exactly one precedence declaration.
6. Verify every precedence declaration references a valid canonical token.
7. Verify token identity against lexer registry.
8. Detect duplicate lexemes.
9. Detect duplicate canonical identities.
10. Detect conflicting precedence assignments.
11. Detect conflicting associativity.
12. Detect operator-prefix hazards.
13. Detect grammar-rule ambiguity.
14. Detect unreachable precedence rules.
15. Detect missing operators.
16. Detect extra executable operators.
17. Verify AST mapping.
18. Verify semantic mapping.
19. Verify IR mapping.
20. Verify domain-specific mappings.
21. Verify version compatibility.
22. Verify dialect compatibility.
23. Verify macro/metaprogramming output contracts.
24. Execute positive precedence tests.
25. Execute negative ambiguity tests.
26. Execute boundary tests.
27. Execute scalability tests.
28. Execute determinism tests.
29. Emit stable diagnostics.
30. Produce conformance result.

---

97. Complexity

The validator should be approximately:

O(N + M + L + T)

for ordinary registry/specification/test traversal, where:

- "N" = number of operator definitions;
- "M" = grammar/metadata size;
- "L" = relevant source/input size;
- "T" = test input size.

More complex grammar-analysis algorithms may naturally have higher complexity.

That complexity must derive from actual grammar structure, not artificial fixed limits.

Avoid unnecessary O(N²) pairwise operator comparison when indexing can solve the same problem.

---

98. Memory Scalability

The validator must allocate according to actual input.

It must not contain language-level constants such as:

MAX_OPERATORS
MAX_LEVELS
MAX_EXPRESSION_DEPTH

A deployment may impose operational resource limits outside the language specification.

Those must be configuration/runtime policy, not grammar semantics.

---

99. Deterministic Data Structures

Where validation output depends on collection iteration, prefer:

BTreeMap
BTreeSet

or explicitly sort results.

Do not expose hash iteration order in diagnostics.

For example:

operator names

must be reported in stable order.

---

100. Safe Rust Reference Model

A production implementation may use structures conceptually similar to:

use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct OperatorKey {
    pub name: String,
    pub lexeme: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Associativity {
    Left,
    Right,
    NonAssociative,
    Chainable,
    ContextDefined,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrecedenceSpec {
    pub level: usize,
    pub associativity: Associativity,
}

pub type PrecedenceTable = BTreeMap<OperatorKey, PrecedenceSpec>;
pub type OperatorSet = BTreeSet<OperatorKey>;

This is illustrative of the required safety and determinism properties.

The repository's actual token/operator types remain authoritative.

No "unsafe" implementation is required.

---

101. Avoid Hard-Coded Precedence in Multiple Places

Do not implement:

precedence in lexer.rs
precedence in parser.rs
precedence in Zamani.g4
precedence in AST
precedence in validation

as independent manually maintained tables.

The preferred model is:

canonical precedence authority
        ↓
derived/executable representation
        ↓
parser
        ↓
validator
        ↓
tests

The parser may encode precedence structurally because parser generators require it, but that structure must be generated or checked against the canonical authority.

---

102. Test Requirements

Every operator must have tests for:

1. isolated use;
2. higher-precedence neighbor;
3. lower-precedence neighbor;
4. equal-precedence neighbor;
5. left association;
6. right association;
7. invalid chaining where applicable;
8. parentheses;
9. nested expressions;
10. function arguments;
11. indexing;
12. member access;
13. assignment;
14. conditional expressions;
15. type-context interactions where applicable;
16. domain-specific contexts;
17. macro-generated use;
18. dialect use;
19. version compatibility.

---

103. Canonical Test Families

The test suite must contain tests equivalent to:

a + b * c
(a + b) * c
a * b + c
a - b - c
a ^ b ^ c
a < b == c
a && b || c
a & b | c
a << b + c
a + b << c
a ? b : c
a ? b : c ? d : e
f(a + b * c)
a[b + c * d]
a.b + c

The exact operator spellings must come from the current canonical operator registry.

Tests must not invent operators that do not exist in Zamani.

---

104. Negative Tests

Negative tests must include:

ambiguous operator sequence
invalid operator chaining
missing operand
missing delimiter
conflicting generic/comparison form
invalid conditional expression
invalid associativity
unknown operator
disabled feature operator
dialect-only operator outside dialect
version-incompatible operator

---

105. Boundary Tests

Boundary testing must include:

single operator
two operators
deeply nested expression
long operator chain
large operand count
nested function calls
nested indexing
nested member access
mixed domain expressions
macro-expanded expressions
generated expressions

No artificial upper boundary should be embedded in the test suite.

The tests may use progressively larger generated inputs until the available test environment's operational resources are reached.

---

106. Scalability Tests

Scalability tests must verify that:

1 expression

and:

very large expression

follow the same precedence rules.

Tests should scale generated expressions rather than asserting a maximum size.

The purpose is to verify:

correctness independent of scale

not:

support exactly N operators

---

107. Determinism Tests

The same source must produce identical:

tokens
AST
diagnostics
semantic representation
IR

across repeated executions.

If validation is parallelized, repeated executions must still have identical diagnostics and ordering.

---

108. Property-Based Testing

Where the repository's test infrastructure permits it, precedence should be tested through generated expression trees.

Generate:

operand
operator
operand

recursively according to the canonical precedence model.

Then verify:

source generation
→ parse
→ AST

matches the original generated tree.

The generated tests must not introduce fixed semantic limits.

Any operational generator bound must be a test-run configuration rather than a language restriction.

---

109. Round-Trip Testing

Where canonical formatting/serialization exists:

source
→ parse
→ AST
→ format
→ parse

must preserve precedence.

The second AST must be semantically equivalent to the first.

For canonical formatting:

(a + b) * c

must not be emitted as:

a + b * c

unless the latter has identical canonical semantics.

---

110. Parenthesis Elision Testing

If a formatter removes redundant parentheses, it must prove that removal preserves AST structure.

For every candidate transformation:

before AST
=
after AST

or semantic equivalence must be explicitly established.

Formatting is not allowed to alter meaning.

---

111. Pretty-Printer Integration

If a pretty-printer exists, precedence metadata must be available to it.

The printer must insert parentheses whenever required to preserve AST meaning.

This is a downstream consumer of the canonical precedence model.

The pretty-printer must not invent its own precedence table.

---

112. Formatter Safety

Formatting must preserve:

- operator precedence;
- associativity;
- source semantics;
- effect structure;
- quantum operation grouping;
- HDL expression grouping.

The formatter must not optimize expressions merely for visual appearance.

---

113. IDE / Language Server Integration

If IDE tooling exists, syntax highlighting and completion should derive operator identity from the canonical lexer registry.

IDE features must not maintain a separate precedence authority.

Hover/documentation may expose:

operator
precedence
associativity
domain
status
version

using canonical metadata.

---

114. Diagnostics Integration

Diagnostics should identify the actual canonical operator/token.

They should not expose obsolete internal aliases as if they were active language constructs.

For example, diagnostics should report:

Ampersand

rather than presenting both:

Ampersand / BitAnd

as competing canonical tokens.

---

115. Compatibility Aliases

Legacy names may remain in compatibility documentation.

For example:

BitAnd → Ampersand
BitOr → Pipe
Question → QuestionMark
Arrow → ThinArrow

where those aliases correspond to the existing migration policy.

These aliases must not produce duplicate parser token variants.

---

116. No Duplicate AST Semantics

The following architecture is prohibited:

Ampersand
    ↓
BitAnd AST

Ampersand
    ↓
Borrow AST

based solely on token identity.

Instead:

Ampersand
    ↓
grammar context
    ↓
canonical AST construct
    ↓
semantic interpretation

---

117. Expression Semantics and POCO-REAF

POCO-REAF requires the source program to describe computation independently of target realization.

Therefore precedence must be invariant across:

atom-scale computation
embedded systems
single CPU
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
future architectures

The same source expression must have the same language meaning.

Only implementation strategy may change.

---

118. Resource Availability

If a program requires resources:

requires qubits >= n
requires memory >= required_memory
requires capability("gpu.compute")

the requirement is evaluated downstream.

It does not change precedence.

A resource shortage may produce a resource diagnostic.

It must never produce a different parse.

---

119. Target Independence

The validator must be target-independent.

The same validation result must occur without requiring:

GPU
QPU
FPGA
ASIC
cluster
network
special accelerator

to be installed.

Grammar validation is a language property.

---

120. Compiler Pipeline Integration

The complete precedence conformance path is:

source
  ↓
lexer
  ↓
canonical tokens
  ↓
expression parser
  ↓
AST
  ↓
semantic analysis
  ↓
canonical semantic model
  ↓
canonical IR
  ↓
optimization
  ↓
lowering
  ↓
routing
  ↓
scheduling
  ↓
resilience/QEC where applicable
  ↓
ZQN where applicable
  ↓
HAL
  ↓
target

Precedence validation must stop structural ambiguity before semantic/IR lowering.

---

121. Feature Completion Contract

A precedence-bearing feature is not complete merely because its parser rule exists.

It is complete only when all of the following are true:

[ ] canonical operator identity
[ ] lexical spelling defined
[ ] precedence defined
[ ] associativity defined
[ ] arity defined
[ ] grammar owner defined
[ ] AST mapping defined
[ ] semantic mapping defined
[ ] IR mapping defined
[ ] source-span behavior defined
[ ] diagnostics defined
[ ] version defined
[ ] dialect behavior defined
[ ] compatibility defined
[ ] positive tests exist
[ ] negative tests exist
[ ] boundary tests exist
[ ] scalability tests exist
[ ] determinism tests exist
[ ] macro behavior defined
[ ] formatter behavior defined
[ ] interoperability behavior defined where applicable
[ ] hard-coding audit passed
[ ] safe-Rust implementation requirement satisfied
[ ] no duplicate authority exists

---

122. Completion Contract for This File

"grammar/validation/precedence.md" itself is complete when it:

1. defines validation ownership;
2. references the canonical precedence authority;
3. covers lexical/operator interaction;
4. covers associativity;
5. covers parser ambiguity;
6. covers AST grouping;
7. covers semantic integration;
8. covers canonical IR;
9. covers quantum::ir;
10. covers HDL;
11. covers classical expressions;
12. covers resource/capability expressions;
13. covers macros;
14. covers dialects;
15. covers versioning;
16. covers compatibility;
17. covers generated grammar;
18. covers diagnostics;
19. covers source spans;
20. covers scalability;
21. covers determinism;
22. prohibits artificial language limits;
23. requires safe Rust;
24. requires no "unsafe";
25. defines positive/negative/boundary/scalability/determinism tests;
26. defines feature completion criteria;
27. does not create a second precedence authority.

---

123. Repository Integration Matrix

Existing area| Integration
"DESIGN.md"| Global authority and architectural invariants
"README.md"| Navigation and authority explanation
"Zamani.g4"| Composition root
"grammar.md"| Implementation-conformance status
"Zamani-Grammar.md"| Historical/proposed feature source
"specification/lexical.md"| Lexical rules
"specification/syntax.md"| Syntax authority
"specification/semantics.md"| Meaning/evaluation
"spec/lexical.md"| Formal lexical contract
"spec/syntax.md"| Formal syntax contract
"lexer/tokens.md"| Canonical tokens
"lexer/operators.md"| Operator lexemes
"lexer/keywords.md"| Keyword registry
"expressions/precedence.md"| Canonical precedence
"expressions/*.g4"| Executable expression grammar
"statements/*.g4"| Expression consumers
"types/*.g4"| Type-expression boundaries
"declarations/*.g4"| Declaration-expression boundaries
"quantum/*.g4"| Quantum expression integration
"hdl/*.g4"| HDL expression integration
"hardware/*.g4"| Hardware intent expressions
"resources/*.g4"| Resource/capability expressions
"macros/*"| Generated expression validation
"dialects/*"| Extension validation
"compatibility/*"| Version/migration behavior
"tests/*"| Executable conformance
"src/lexer.rs"| Tokenization
"src/parser.rs"| Parsing
"src/frontend/ast/"| Structural representation
semantic analysis| Meaning/type/effect/resource validation
"quantum::ir"| Canonical quantum representation
compiler/lowering| Target realization

---

124. Cross-File Change Rule

When a precedence rule changes, the change must begin at the authority that owns the rule.

Required flow:

canonical specification
        ↓
expressions/precedence.md
        ↓
operator registry
        ↓
grammar implementation
        ↓
lexer/parser conformance
        ↓
AST tests
        ↓
semantic tests
        ↓
IR tests
        ↓
compatibility tests

A developer must not directly patch a generated parser merely to make a test pass.

---

125. No Unnecessary File Renames

Existing repository filenames remain authoritative unless there is a documented architectural reason to change them.

In particular, this validation contract does not require renaming:

Zamani.g4
grammar.md
Zamani-Grammar.md
DESIGN.md
README.md

Existing domain files should be integrated before new parallel replacements are introduced.

---

126. No Parallel Authority

The following architecture is prohibited:

expressions/precedence.md
        +
Zamani.g4 precedence table
        +
src/parser.rs precedence table
        +
another precedence.md

unless each is explicitly derived from the canonical authority.

The repository must have:

one semantic precedence authority
many validated consumers

not:

many competing precedence authorities

---

127. Validation Gate

The precedence gate passes only when:

canonical precedence
=
implemented precedence
=
parser behavior
=
AST grouping
=
semantic grouping
=
IR grouping

for every stable feature.

For experimental/proposed features, their status must remain explicit.

---

128. Production-Ready Acceptance Criteria

The Zamani precedence system is production-ready only when all of these are true:

[ ] One canonical precedence authority exists.
[ ] Every active operator has exactly one precedence definition.
[ ] Every active operator has explicit associativity.
[ ] Lexer token identities are canonical.
[ ] Duplicate token identities are eliminated.
[ ] Maximal lexical matching is deterministic.
[ ] Keyword boundaries are deterministic.
[ ] Parser grouping is deterministic.
[ ] No unresolved ambiguity remains.
[ ] No unreachable active precedence rule remains.
[ ] AST grouping is canonical.
[ ] Semantic analysis never repairs parser ambiguity.
[ ] Canonical IR preserves expression meaning.
[ ] quantum::ir preserves quantum grouping.
[ ] HDL grouping is deterministic.
[ ] Resource expressions are target-independent.
[ ] Hardware capabilities do not alter precedence.
[ ] Dialects cannot silently redefine core precedence.
[ ] Version changes are compatibility-aware.
[ ] Macros cannot bypass precedence validation.
[ ] Generated code is reproducible.
[ ] Diagnostics are deterministic.
[ ] Source spans are correct.
[ ] Positive tests exist.
[ ] Negative tests exist.
[ ] Boundary tests exist.
[ ] Scalability tests exist.
[ ] Determinism tests exist.
[ ] Compatibility tests exist.
[ ] Formatter round-tripping preserves grouping.
[ ] No artificial language capacity limits exist.
[ ] No `unsafe` Rust is required.
[ ] Rust 1.97/1.97.1 compatibility is maintained.
[ ] No unnecessary existing file has been renamed.

---

129. Final Precedence Contract

Zamani precedence is a property of the language, not of the machine.

The fundamental rule is:

lexical identity
    ↓
syntactic precedence
    ↓
associativity
    ↓
AST structure
    ↓
semantic meaning
    ↓
canonical IR
    ↓
target realization

Never:

target hardware
    ↓
parser behavior

Never:

runtime capability
    ↓
operator precedence

Never:

optimization
    ↓
reinterpret source

Never:

semantic analysis
    ↓
choose between ambiguous parses

The source program must have one deterministic interpretation before target-specific compilation begins.

---

130. POCO-REAF Guarantee

The precedence architecture contributes to:

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever

by guaranteeing that the meaning of an expression is independent of its eventual realization.

A Zamani program may ultimately execute on:

atom-scale systems
embedded systems
CPU
multicore CPU
GPU
FPGA
ASIC
QPU
quantum simulator
specialized accelerator
HPC system
cluster
distributed system
cloud
future computing architecture

without changing the language's precedence semantics.

The compiler may choose different:

lowering
optimization
vectorization
parallelization
routing
scheduling
resource placement
quantum decomposition
QEC strategy
hardware mapping

but those decisions occur after parsing and semantic interpretation.

---

131. Final Principle

The production-ready Zamani precedence system is therefore:

ONE lexical identity
        ↓
ONE canonical precedence model
        ↓
ONE deterministic parse
        ↓
ONE canonical AST structure
        ↓
ONE semantic interpretation
        ↓
ONE canonical IR representation
        ↓
MANY valid target realizations

The number of operators, expression nodes, operands, nesting depth, qubits, CPUs, GPUs, FPGAs, ASIC resources, QPUs, nodes, threads, tensor dimensions, memory capacity, or future devices is never converted into an artificial grammar maximum.

The implementation scales according to actual available resources.

The language semantics remain invariant.

That invariant is the foundation required for Zamani's POCO-REAF model.