

Zamani Language — Canonical Syntax Specification

Path: "grammar/spec/syntax.md"
Language: Zamani
Specification role: Canonical source-language syntax contract
Status: Production architecture
Specification version: 1.0
Minimum Rust implementation baseline: Rust 1.97 / 1.97.1
Safety requirement: No "unsafe" Rust is permitted in the Zamani compiler implementation.

---

0. Purpose

This document defines the canonical syntax of the Zamani programming language.

It exists to establish a single, implementation-verifiable syntax contract between:

source
  │
  ▼
lexer
  │
  ▼
parser
  │
  ▼
frontend AST
  │
  ▼
semantic analysis
  │
  ▼
canonical IR
  │
  ▼
optimization
  │
  ▼
target-independent lowering
  │
  ▼
target-specific realization
  │
  ├── CPU
  ├── GPU
  ├── FPGA
  ├── accelerator
  ├── simulator
  ├── emulator
  ├── QPU
  ├── photonic quantum hardware
  ├── fault-tolerant quantum hardware
  └── future computational substrates

The specification deliberately separates:

1. lexical syntax;
2. concrete syntax;
3. AST structure;
4. semantic rules;
5. type rules;
6. resource requirements;
7. quantum semantics;
8. optimization;
9. scheduling;
10. target realization.

The grammar must not encode hardware assumptions.

In particular, the syntax must not require:

- a fixed number of qubits;
- a fixed number of classical registers;
- a fixed gate set;
- a fixed topology;
- a fixed CPU architecture;
- a fixed word size;
- a fixed memory capacity;
- a fixed accelerator;
- a fixed quantum architecture;
- a fixed simulator;
- a fixed execution provider.

Zamani source describes computational intent.

The compiler determines how that intent can be realized.

---

1. Canonical-specification rule

Zamani previously contained several overlapping language descriptions.

These are not equivalent:

grammar/grammar.md
grammar/Zamani.g4
grammar/Zamani-Grammar.md
src/lexer.rs
src/parser.rs
src/ast/

The production architecture therefore defines the following authority order.

1.1 Authority hierarchy

1. This canonical language specification
2. Canonical lexical specification
3. Canonical parser implementation
4. Canonical AST representation
5. Canonical semantic/type rules
6. Canonical IR
7. Target-specific implementations
8. Historical/aspirational documents

No historical grammar may silently introduce syntax.

No backend may introduce source-language syntax.

No hardware backend may modify the meaning of source syntax.

If an implementation disagrees with this document, one of the following must happen:

SPEC_CHANGE
IMPLEMENTATION_FIX
EXPLICIT_COMPATIBILITY_RULE

A divergence must never be silently accepted.

---

2. Scope

Zamani is a general computational language.

Its syntax is capable of representing:

- ordinary computation;
- systems programming;
- functional programming;
- object-oriented programming;
- generic programming;
- algebraic data types;
- pattern matching;
- concurrency;
- asynchronous computation;
- algebraic effects;
- temporal computation;
- quantum computation;
- numerical computation;
- symbolic computation;
- nano computation;
- distributed computation;
- hardware-oriented descriptions;
- metaprogramming;
- AI/agent-oriented computation;
- Sankofa temporal-memory constructs;
- resource-aware computation.

These are language domains.

They are not separate languages.

The same lexical, syntactic, semantic, type, diagnostic, and IR infrastructure must be reused.

---

3. Core design principle: Program Once

Zamani is designed around:

POCO-REAF
Program Once
Compile Once
Run Everywhere
Anywhere
Forever

This means that portable source code describes what computation means, rather than assuming where it will execute.

For example, source code may describe:

quantum {
    apply H to q[0];
    apply controlled(X) from q[0] to q[1];
}

without declaring:

IBM
IonQ
Rigetti
CUDA
x86_64
ARM64
AVX
GPU count
QPU topology
physical qubit identifiers

The semantic and lowering layers determine an appropriate realization.

---

4. Formal notation

This document uses EBNF-style notation.

A ::= B

defines a production.

A | B

means alternative.

[A]

means optional.

{A}

means zero or more occurrences.

(A)

groups productions.

Terminal literals appear in double quotes.

Examples:

"fn"
"("
")"

Lexical classes appear in uppercase:

IDENT
INTEGER
STRING

Semantic constraints are written separately from syntax.

---

5. Source file

A Zamani source file is a sequence of declarations/statements followed by EOF.

Program ::= { Attribute | Item | Statement } EOF ;

A source file may be empty.



is valid.

A source file may contain arbitrary supported Unicode in comments and string literals.

Identifiers follow the canonical identifier rules defined below.

---

6. Lexical grammar

6.1 Unicode model

Zamani source is UTF-8.

The compiler must treat source positions consistently as byte offsets for internal source mapping while preserving Unicode character boundaries for lexical interpretation.

The language specification must never assume that:

1 character == 1 byte

Source spans therefore identify source ranges through the compiler's canonical "Span" representation.

---

7. Whitespace

Whitespace is insignificant except where it separates lexical tokens.

SPACE
TAB
CR
LF

may appear between tokens.

Whitespace must not alter program semantics.

---

8. Comments

8.1 Line comments

// comment

A line comment extends to the end of the line.

8.2 Block comments

/* comment */

Block comments are non-nesting unless a future language revision explicitly introduces nesting.

Unterminated block comments are lexical errors.

Comments do not produce AST nodes.

---

9. Identifiers

The canonical identifier grammar is:

IDENT ::= IDENT_START { IDENT_CONTINUE } ;

IDENT_START ::= LETTER | "_" ;

IDENT_CONTINUE ::= LETTER | DIGIT | "_" ;

LETTER ::= ASCII_LETTER | UNICODE_LETTER ;

DIGIT ::= "0".."9" ;

The implementation may internally support Unicode identifier classification, but the accepted character policy must be centralized in the lexer.

Identifiers must not be silently truncated.

Identifier comparison is case-sensitive.

Therefore:

foo
Foo
FOO

are distinct identifiers.

---

10. Keywords

Keywords are reserved.

They cannot be used as ordinary identifiers unless a future explicit escape mechanism is standardized.

The current canonical keyword families are:

10.1 Core

let
var
mut
const
fn
return
if
else
while
for
in
loop
break
continue
match
case
when

10.2 Types and declarations

struct
enum
trait
impl
class
interface
record
type
module
import
export
use
from
as
where

10.3 Visibility and object model

public
pub
private
protected
static
virtual
override
abstract
extends
implements
new
this
self
super

10.4 Functions and concurrency

async
await
spawn
yield

10.5 Error/effect handling

try
catch
finally
throw
effect
perform
handle

10.6 Quantum

quantum
qubit
circuit
entangle
noise
fidelity
surface
logical
parity

10.7 Nano/agent

nano
agent

10.8 Sankofa/temporal

remember
recall
learn
infer
wisdom
zamani
sasa
ancestor

10.9 Type-system qualifiers

linear
affine

10.10 Language/meta facilities

language
model
macro

10.11 Built-in primitive vocabulary

void
int
float
bool
str
String
char
nil
null

10.12 Built-in operations

print
println
assert
panic
len
sizeof

Additional keywords already represented by the implementation lexer may only become canonical language constructs after their parser, AST, semantic, diagnostic, and test contracts are defined.

They must not be treated as semantically meaningful merely because they exist as lexer tokens.

---

11. Literals

11.1 Boolean

BooleanLiteral ::= "true" | "false" ;

11.2 Null

NullLiteral ::= "nil" | "null" ;

Both spellings represent the same null semantic value.

The AST must normalize them to one canonical representation.

---

12. Integer literals

The minimum decimal form is:

IntegerLiteral ::= DIGIT { DIGIT } ;

The language may later add explicit radix forms:

0b...
0o...
0x...

but such forms must not be claimed as implemented until lexer, parser, AST, semantic checking, diagnostics, tests, and code generation support them.

Integer literal magnitude must not be restricted by a machine-specific integer width during lexing.

Literal range checking belongs to semantic/type analysis.

---

13. Floating-point literals

The canonical decimal floating form is:

FloatLiteral ::= DIGIT { DIGIT } "." DIGIT { DIGIT } ;

Floating-point representation is determined by the type system.

The parser must not assume that every floating literal is necessarily "f64".

---

14. String literals

StringLiteral ::= '"' { StringChar | EscapeSequence } '"' ;

The lexer must reject unterminated strings.

Escape processing is performed by the lexical layer.

The AST receives the decoded semantic string value together with its source span.

---

15. Character literals

CharLiteral ::= "'" CharChar "'" ;

Exactly one semantic character is required after escape processing.

Invalid or unterminated character literals are lexical errors.

---

16. Quantum literals

The existing quantum literal form is:

QuantumLiteral ::= "|" QuantumBasis "⟩" ;

QuantumBasis ::= "0"
                | "1"
                | "+"
                | "-" ;

Examples:

|0⟩
|1⟩
|+⟩
|-⟩

Quantum literals are source-level values.

They do not identify physical qubits.

---

17. Nano annotations

Nano annotations use:

NanoAnnotation ::= "@" IDENT [ "(" ArgumentList? ")" ] ;

Examples:

@atom
@molecule(x)

Annotations are syntactic metadata.

Their meaning is assigned by semantic analysis.

---

18. MTS literals

The reserved temporal literal form is:

MTSLiteral ::= "mts" "[" Expression "]" ;

The lexer currently contains an MTS token category, but implementation support must not be considered complete until the lexer emits it and the parser/AST/semantic layers consume it consistently.

Until then, an implementation must not claim full MTS literal conformance.

---

19. Punctuation

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
::
@
#
~
?
!

---

20. Operators

20.1 Assignment

=
+=
-=
*=
/=

20.2 Arithmetic

+
-
*
/
%

20.3 Comparison

==
!=
<
>
<=
>=

20.4 Logical

&&
||

and the keyword forms:

and
or

may be supported where defined by the parser and semantic layer.

20.5 Bitwise

&
|
^
~
<<
>>

20.6 Range

..
..=

20.7 Other

?
!
->
=>
::

---

21. Operator precedence

The canonical precedence ordering is:

Lowest
│
├── assignment
├── range
├── logical OR
├── logical AND
├── bitwise OR
├── bitwise XOR
├── bitwise AND
├── equality
├── comparison
├── shift
├── additive
├── multiplicative
├── prefix
├── call
├── index
└── member
Highest

The parser uses precedence-driven parsing.

The exact precedence table is:

Level| Category
1| Assignment
2| Range
3| Logical OR
4| Logical AND
5| Bitwise OR
6| Bitwise XOR
7| Bitwise AND
8| Equality
9| Comparison
10| Shift
11| Sum
12| Product
13| Prefix
14| Call
15| Index
16| Member

Associativity must be explicitly defined by the parser rather than inferred by a backend.

---

22. Attributes

Attributes precede a declaration or statement:

Attribute ::= "#[" AttributeBody "]" ;

The body is intentionally extensible.

Attributes may be used for:

- compilation directives;
- optimization hints;
- diagnostics;
- ABI annotations;
- target constraints;
- resource declarations;
- generated code;
- experimental features.

Attributes must not directly mutate compiler global state.

Unknown attributes must produce a deterministic diagnostic according to the active language/compatibility policy.

---

23. Statements

Statement ::=
      LetStatement
    | VarStatement
    | ConstStatement
    | FunctionDeclaration
    | ReturnStatement
    | BreakStatement
    | ContinueStatement
    | WhileStatement
    | ForStatement
    | MatchStatement
    | StructDeclaration
    | EnumDeclaration
    | TraitDeclaration
    | ImplDeclaration
    | ClassDeclaration
    | InterfaceDeclaration
    | ModuleDeclaration
    | ImportStatement
    | UseStatement
    | TypeAliasDeclaration
    | QuantumDeclaration
    | NanoAgentDeclaration
    | RememberStatement
    | EffectDeclaration
    | HandleStatement
    | UnsafeDeclaration
    | WisdomStatement
    | LanguageDeclaration
    | ExpressionStatement
    ;

A future implementation may introduce additional declarations without changing the fundamental compilation architecture.

---

24. Variable bindings

24.1 "let"

LetStatement ::=
    "let"
    ["mut"]
    IDENT
    [":" TypeExpression]
    "="
    Expression
    [";"] ;

Example:

let x = 42;
let y: Int = 100;
let mut state = initial_state;

24.2 "var"

VarStatement ::=
    "var"
    IDENT
    [":" TypeExpression]
    "="
    Expression
    [";"] ;

"var" is syntactically distinct but may share semantic implementation with "let" according to the language's mutability rules.

24.3 Constants

ConstStatement ::=
    "const"
    IDENT
    [":" TypeExpression]
    "="
    Expression
    [";"] ;

Constant validity is a semantic property.

---

25. Functions

FunctionDeclaration ::=
    ["async"]
    "fn"
    IDENT
    [GenericParameters]
    "(" [ParameterList] ")"
    ["->" TypeExpression]
    [WhereClause]
    BlockExpression ;

Examples:

fn add(a: Int, b: Int) -> Int {
    a + b
}

async fn compute(input: Data) -> Result {
    ...
}

---

26. Parameters

ParameterList ::= Parameter { "," Parameter } ;

Parameter ::=
    ["mut"]
    IDENT
    [":" TypeExpression]
    ["=" Expression] ;

A parameter without a type annotation is syntactically valid.

Its semantic interpretation must be determined by type inference.

The compiler must not silently treat missing type annotations as an unlimited dynamic type unless the active type-system policy explicitly enables dynamic typing.

---

27. Generic parameters

GenericParameters ::=
    "<"
    TypeParameter { "," TypeParameter }
    ">" ;

TypeParameter ::=
    IDENT
    { TypeBound } ;

TypeBound ::=
    ":" TypeExpression ;

Generic constraints belong to semantic/type analysis.

---

28. Where clauses

WhereClause ::=
    "where"
    WherePredicate
    { "," WherePredicate } ;

WherePredicate ::=
    IDENT ":" TypeExpression ;

A parser must preserve where-clause information.

It must never discard constraints merely to reach a block.

---

29. Return

ReturnStatement ::= "return" [Expression] [";"] ;

A bare return represents the unit value.

---

30. Control flow

30.1 If

IfExpression ::=
    "if"
    Expression
    BlockExpression
    ["else" (IfExpression | BlockExpression)] ;

30.2 While

WhileStatement ::=
    "while"
    Expression
    BlockExpression ;

30.3 For

ForStatement ::=
    "for"
    Pattern
    "in"
    Expression
    BlockExpression ;

The parser must not constrain iteration to a fixed collection size.

30.4 Loop

LoopExpression ::= "loop" BlockExpression ;

---

31. Break and continue

BreakStatement ::= "break" [";"] ;

ContinueStatement ::= "continue" [";"] ;

---

32. Match

MatchExpression ::=
    "match"
    Expression
    "{"
    { MatchArm }
    "}" ;

MatchArm ::=
    Pattern
    [ "if" Expression ]
    "=>"
    (Expression | BlockExpression)
    ["," ] ;

The AST already supports structured patterns including wildcard, identifier, literal, tuple, struct, enum, OR, range, and reference patterns.

Therefore the production grammar must define patterns structurally rather than pretending that match arms are arbitrary expressions.

---

33. Patterns

Pattern ::=
      "_"
    | IDENT
    | LiteralPattern
    | TuplePattern
    | StructPattern
    | EnumPattern
    | OrPattern
    | RangePattern
    | ReferencePattern
    ;

LiteralPattern ::= Literal ;

TuplePattern ::=
    "("
    [ Pattern { "," Pattern } ]
    ")" ;

StructPattern ::=
    Path
    "{"
    [ StructPatternField { "," StructPatternField } ]
    "}" ;

StructPatternField ::=
    IDENT
    [":" Pattern] ;

EnumPattern ::=
    Path
    "("
    [ Pattern { "," Pattern } ]
    ")" ;

OrPattern ::=
    Pattern
    "|"
    Pattern
    { "|" Pattern } ;

RangePattern ::=
    Pattern (".." | "..=") Pattern ;

ReferencePattern ::=
    "&" Pattern ;

Semantic validation determines whether a pattern is legal for the matched value.

---

34. Blocks

BlockExpression ::= "{" { Statement } "}" ;

A block is an expression.

Its resulting value is determined by its final expression according to semantic rules.

A block must never depend on a machine-specific stack size.

Resource exhaustion is an execution/resource-management concern, not a grammar rule.

---

35. Expressions

Expression ::=
    PrimaryExpression
    { PostfixOrInfixExpression } ;

The parser's Pratt/precedence implementation determines the concrete association.

---

36. Primary expressions

PrimaryExpression ::=
      IDENT
    | Literal
    | "(" Expression ")"
    | TupleExpression
    | ArrayExpression
    | BlockExpression
    | LambdaExpression
    | AnonymousFunctionExpression
    | IfExpression
    | MatchExpression
    | LoopExpression
    | AsyncExpression
    | AwaitExpression
    | SpawnExpression
    | NewExpression
    | TryExpression
    | RecallExpression
    | LearnExpression
    | PerformExpression
    | ZamaniExpression
    | SasaExpression
    | QuantumExpression
    | NanoExpression
    ;

---

37. Tuple expressions

TupleExpression ::=
    "("
    Expression
    ","
    [ Expression { "," Expression } ]
    ")" ;

A single parenthesized expression without a comma is not a tuple.

---

38. Arrays

ArrayExpression ::=
    "["
    [ Expression { "," Expression } ]
    "]" ;

Array length is semantic/runtime information.

The syntax must not impose a maximum array size.

---

39. Lambdas

LambdaExpression ::=
    "|"
    [ParameterList]
    "|"
    (BlockExpression | Expression) ;

Example:

let square = |x: Int| x * x;

---

40. Anonymous functions

AnonymousFunctionExpression ::=
    "fn"
    "("
    [ParameterList]
    ")"
    ["->" TypeExpression]
    BlockExpression ;

---

41. Calls

CallExpression ::=
    Expression
    "("
    [ArgumentList]
    ")" ;

ArgumentList ::= Expression { "," Expression } ;

The number of arguments is not constrained by a global machine constant.

Semantic checking determines whether a call matches its declaration.

---

42. Indexing

IndexExpression ::=
    Expression
    "["
    Expression
    "]" ;

Index width and container capacity are semantic/runtime concerns.

---

43. Member access

MemberExpression ::=
    Expression
    "."
    IDENT ;

Method calls are represented as member access followed by invocation.

---

44. Assignment

AssignmentExpression ::=
    Expression
    "="
    Expression ;

CompoundAssignmentExpression ::=
    Expression
    ("+=" | "-=" | "*=" | "/=")
    Expression ;

Assignment validity is determined semantically.

The left-hand side must be assignable.

---

45. Type casts and ascription

CastExpression ::=
    Expression
    "as"
    TypeExpression ;

TypeAscriptionExpression ::=
    Expression
    ":"
    TypeExpression ;

---

46. Try propagation

TryPropagationExpression ::= Expression "?" ;

Its semantics are type-dependent.

The grammar does not assume a particular error type.

---

47. Try/catch

TryExpression ::=
    "try"
    Expression
    {
        "catch"
        ["(" [IDENT ":"] TypeExpression ")"]
        BlockExpression
    }
    ["finally" BlockExpression] ;

---

48. Async computation

AsyncExpression ::= "async" Expression ;

AwaitExpression ::= "await" Expression ;

SpawnExpression ::= "spawn" Expression ;

The runtime decides how asynchronous work is scheduled.

The language does not assume a particular thread count.

---

49. Object construction

NewExpression ::=
    "new"
    Path
    ["(" [ArgumentList] ")"] ;

---

50. Struct literals

StructLiteral ::=
    Path
    "{"
    [ StructFieldInitializer { "," StructFieldInitializer } ]
    "}" ;

StructFieldInitializer ::=
    IDENT ":" Expression ;

---

51. Paths

Path ::= PathSegment { ("::" | ".") PathSegment } ;

PathSegment ::= IDENT ;

Special path keywords such as "self" and "super" are semantically interpreted.

---

52. Type expressions

The canonical type grammar is:

TypeExpression ::=
      TypePath
    | GenericType
    | TupleType
    | ArrayType
    | SliceType
    | FunctionType
    | ReferenceType
    | PointerType
    | OptionalType
    | ResultType
    | QuantumType
    | LinearType
    | AffineType
    | TemporalType
    | DependentType
    | IdentityType
    | SelfType
    | UnitType
    | NeverType
    ;

---

53. Named types

TypePath ::= Path ;

---

54. Generic types

GenericType ::=
    TypePath
    "<"
    TypeExpression
    { "," TypeExpression }
    ">" ;

Example:

Vector<Float>
Result<Value, Error>

---

55. Tuple types

TupleType ::=
    "("
    TypeExpression
    ","
    [ TypeExpression { "," TypeExpression } ]
    ")" ;

---

56. Unit type

UnitType ::= "(" ")" ;

---

57. Array types

The language must distinguish semantic array size from syntax.

ArrayType ::=
    "["
    TypeExpression
    [ ";" Expression ]
    "]" ;

A dynamic/unspecified size must not be encoded as a hard-coded sentinel.

The semantic representation must use an explicit optional/unknown size.

The implementation must not use:

0 == unknown

because zero is a legitimate semantic size.

---

58. Slice types

SliceType ::= "&" ["mut"] "[" TypeExpression "]" ;

---

59. Reference types

ReferenceType ::= "&" ["mut"] TypeExpression ;

---

60. Pointer types

PointerType ::= "*" ["mut"] TypeExpression ;

Pointer semantics are target-dependent.

The syntax does not guarantee that raw pointers exist on every target.

---

61. Function types

FunctionType ::=
    "fn"
    "("
    [ TypeExpression { "," TypeExpression } ]
    ")"
    ["->" TypeExpression] ;

---

62. Optional types

OptionalType ::= TypeExpression "?" ;

The parser must distinguish expression-level "?" from type-level "?".

The semantic representation is:

Optional<T>

---

63. Result types

ResultType ::= "Result" "<" TypeExpression "," TypeExpression ">" ;

---

64. Never type

NeverType ::= "!" ;

The semantic type represents computations that do not return normally.

---

65. Self type

SelfType ::= "Self" | "self" ;

---

66. Quantum types

Quantum types describe computational quantum resources, not physical hardware identifiers.

A canonical semantic quantum type is:

Quantum<T>

The syntax may use:

QuantumType ::= "Quantum" "<" TypeExpression ">" ;

or the implementation's established "quantum" type form where supported.

The semantic representation must preserve:

- logical dimensionality;
- ownership;
- linearity;
- entanglement relationships;
- measurement state;
- resource requirements.

It must not embed:

physical qubit number
device-specific index
vendor-specific gate identifier

---

67. Linear types

LinearType ::= "linear" TypeExpression ;

A linear value must obey the language's linear ownership rules.

This is especially important for quantum resources.

---

68. Affine types

AffineType ::= "affine" TypeExpression ;

An affine value may be consumed at most once.

---

69. Temporal types

TemporalType ::= "Temporal" "<" TypeExpression ">" ;

Temporal semantics are defined outside the grammar.

---

70. Dependent types

Zamani may represent dependent type constructs using:

Π
Σ

and their keyword forms where implemented.

Conceptually:

DependentType ::=
      "Π" IDENT ":" TypeExpression "." TypeExpression
    | "Pi" IDENT ":" TypeExpression "." TypeExpression
    | "Σ" IDENT ":" TypeExpression "." TypeExpression
    | "Sigma" IDENT ":" TypeExpression "." TypeExpression ;

The exact dependent-type semantics belong to the type checker.

---

71. Identity types

IdentityType ::=
    "Identity"
    "<"
    Expression
    ","
    Expression
    ">" ;

---

72. Struct declarations

StructDeclaration ::=
    ["public"]
    "struct"
    IDENT
    [GenericParameters]
    "{"
    { StructField }
    "}" ;

StructField ::=
    [Visibility]
    IDENT
    [":" TypeExpression]
    ["," | ";"] ;

---

73. Enumerations

EnumDeclaration ::=
    "enum"
    IDENT
    [GenericParameters]
    "{"
    { EnumVariant }
    "}" ;

EnumVariant ::=
    IDENT
    [
        "(" [TypeExpression { "," TypeExpression }] ")"
      | "{" { StructField } "}"
    ]
    ["," ] ;

---

74. Traits

TraitDeclaration ::=
    "trait"
    IDENT
    [GenericParameters]
    [":" TypeExpression { "," TypeExpression }]
    "{"
    { TraitItem }
    "}" ;

TraitItem ::=
      TraitMethod
    | TraitAssociatedType
    | TraitConstant ;

TraitMethod ::=
    "fn"
    IDENT
    [GenericParameters]
    "(" [ParameterList] ")"
    ["->" TypeExpression]
    (BlockExpression | ";") ;

TraitAssociatedType ::=
    "type"
    IDENT
    [":" TypeExpression]
    ";" ;

TraitConstant ::=
    "const"
    IDENT
    ":"
    TypeExpression
    ["=" Expression]
    ";" ;

---

75. Implementations

ImplDeclaration ::=
    "impl"
    [GenericParameters]
    TypeExpression
    ["for" TypeExpression]
    "{"
    { ImplItem }
    "}" ;

---

76. Classes

ClassDeclaration ::=
    [Visibility]
    "class"
    IDENT
    [GenericParameters]
    [InheritanceClause]
    "{"
    { ClassMember }
    "}" ;

InheritanceClause ::=
    ("extends" TypeExpression)
    |
    ("implements" TypeExpression { "," TypeExpression })
    |
    ("extends" TypeExpression
     "implements" TypeExpression { "," TypeExpression }) ;

---

77. Interfaces

InterfaceDeclaration ::=
    "interface"
    IDENT
    [GenericParameters]
    [":" TypeExpression { "," TypeExpression }]
    "{"
    { InterfaceMember }
    "}" ;

---

78. Modules

ModuleDeclaration ::=
    "module"
    IDENT
    (";" | BlockExpression) ;

Nested module paths are semantic entities.

The language must not impose a fixed maximum module depth.

---

79. Imports

ImportStatement ::=
    "import"
    Path
    ["as" IDENT]
    [";"] ;

---

80. Use declarations

UseStatement ::=
    "use"
    UsePath
    [";"] ;

UsePath ::=
      Path
    | Path "::" "*"
    | Path "::" "{"
        IDENT { "," IDENT }
      "}" ;

Filesystem/network resolution is not grammar behavior.

Import resolution must be policy-controlled.

---

81. Type aliases

TypeAliasDeclaration ::=
    "type"
    IDENT
    [GenericParameters]
    "="
    TypeExpression
    [";"] ;

---

82. Quantum computing

Quantum syntax is intentionally semantic.

The language must not define the universe of quantum operations as a closed list such as:

H
X
Y
Z
T
S
CNOT
SWAP

Those are operations that may be available in particular target dialects.

The source language instead describes operations by semantic identity.

---

83. Quantum declarations

A canonical quantum region is:

QuantumDeclaration ::=
    "quantum"
    (
        "circuit" IDENT [QuantumParameterList] BlockExpression
      | BlockExpression
      | IDENT BlockExpression
    ) ;

Compatibility syntax may include:

circuit Name { ... }
quantum circuit Name { ... }

but all forms normalize into the same semantic AST representation.

---

84. Quantum resources

Quantum resources may be declared abstractly.

Conceptually:

let q = qubit();
let register = qubit[1024];

The source must not require the physical system to contain exactly 1024 qubits.

If a target has fewer resources, the compiler may:

- reject the target;
- partition the computation;
- transform the computation;
- time-multiplex resources;
- invoke a simulator;
- invoke a distributed execution strategy;

according to semantic and compilation policy.

The source program itself remains unchanged.

---

85. Quantum operation syntax

The canonical conceptual form is:

QuantumOperation ::=
    "apply"
    QuantumOperator
    QuantumOperands
    [QuantumModifiers] ;

The operator is an extensible semantic operation identifier.

QuantumOperator ::=
    Path
    | IdentifierExpression ;

Operands may identify logical quantum values:

QuantumOperands ::=
    "to" QuantumValueList
    |
    "from" QuantumValueList "to" QuantumValueList
    |
    QuantumValueList ;

This allows semantic operations such as:

apply H to q[0];
apply controlled(X) from q[0] to q[1];
apply U(theta, phi, lambda) to q[0];

without making the grammar dependent on any vendor gate library.

---

86. Quantum measurement

Measurement is a semantic operation.

Conceptually:

QuantumMeasureExpression ::=
    "measure"
    QuantumValue ;

The compiler must preserve measurement semantics and dependencies.

It must not automatically insert measurements merely because a target backend needs them.

---

87. Entanglement

The lexer and AST already recognize entanglement-related constructs.

The canonical semantic form is:

EntangleExpression ::=
    "entangle"
    Expression
    "with"
    Expression ;

Entanglement is a semantic relationship.

It is not synonymous with a particular two-qubit gate.

---

88. Quantum noise

Noise is represented as an explicit semantic concern.

Conceptually:

NoiseDeclaration ::=
    "noise"
    IDENT
    BlockExpression ;

or another explicitly standardized noise declaration form.

Noise models must not be hard-coded into source syntax.

The noise subsystem determines:

- channel model;
- stochastic parameters;
- calibration data;
- fault model;
- execution context.

---

89. Fidelity

Fidelity analysis is a semantic/verification operation.

A source construct such as:

fidelity

must not imply a particular algorithm.

The compiler and quantum subsystem determine the appropriate fidelity calculation.

---

90. Surface-code and logical quantum computation

The source language may describe logical quantum computation without requiring a specific physical error-correction implementation.

Conceptually:

logical
surface
parity

are semantic domain constructs.

The compiler may lower them through:

logical IR
    ↓
QEC planning
    ↓
physical realization

The source syntax must never hard-code:

MAX_QUBITS
MAX_CODE_DISTANCE
MAX_STABILIZERS

or equivalent constants.

---

91. Quantum scalability contract

Zamani quantum programs must scale from:

one logical qubit

through:

small registers

through:

large registers

through:

distributed quantum systems

subject only to:

available resources
semantic validity
target capability
compiler policy
execution feasibility

No source-level constant may artificially establish an upper limit.

---

92. Nano agents

The canonical nano-agent declaration is:

NanoAgentDeclaration ::=
    ("nano" "agent" | "agent" | "nano")
    IDENT
    (BlockExpression | Expression) ;

The semantics of nano computation are not encoded into lexical rules.

---

93. Sankofa constructs

93.1 Remember

RememberStatement ::=
    "remember"
    IDENT
    [":" TypeExpression]
    "="
    Expression
    [";"] ;

93.2 Recall

RecallExpression ::=
    "recall"
    (
        "(" Expression ")"
        |
        Expression
    ) ;

93.3 Learn

LearnExpression ::=
    ("learn" | "infer")
    ["from"]
    Expression ;

93.4 Wisdom

WisdomStatement ::=
    "wisdom"
    IDENT
    ["=" Expression]
    [";"] ;

---

94. Temporal constructs

ZamaniExpression ::= "zamani" (BlockExpression | Expression) ;

SasaExpression ::= "sasa" (BlockExpression | Expression) ;

"zamani" and "sasa" are semantic temporal scopes.

They must not be implemented as textual macros.

---

95. Effects

EffectDeclaration ::=
    "effect"
    IDENT
    (BlockExpression | ";") ;

---

96. Performing effects

PerformExpression ::= "perform" Expression ;

The effect system determines:

- effect identity;
- effect arguments;
- capabilities;
- handlers;
- resource requirements;
- whether an effect is permitted.

---

97. Effect handlers

HandleStatement ::=
    "handle"
    IDENT
    BlockExpression
    ["with" BlockExpression] ;

---

98. Language declaration

LanguageDeclaration ::=
    "language"
    IDENT
    [StringLiteral]
    [";"] ;

Language declarations identify source compatibility/version intent.

They must not silently alter the compiler's interpretation.

Version selection must be deterministic.

---

99. Unsafe

The source language may contain an "unsafe" construct only if its semantic purpose is explicitly defined.

However:

«The Zamani compiler implementation itself must contain no Rust "unsafe" code.»

This distinction is mandatory.

A source-level:

unsafe {
    ...
}

does not authorize the compiler implementation to use Rust "unsafe".

If source-level unsafe semantics are retained, the compiler must:

1. parse them;
2. represent them explicitly;
3. apply capability/type/effect checks;
4. produce diagnostics;
5. lower them through safe Rust abstractions.

---

100. Expression statements

ExpressionStatement ::= Expression [";"] ;

The parser must preserve expression statements in the AST where their value or side effects matter.

---

101. Semantic boundary

The grammar answers:

«Is the source syntactically valid?»

The AST answers:

«What source construct was written?»

The semantic layer answers:

«Is the construct valid?»

The type system answers:

«What are its types and resource obligations?»

The IR answers:

«What computation does it mean?»

The optimizer answers:

«What equivalent computation is cheaper/better?»

The scheduler answers:

«When can it execute?»

The target backend answers:

«How can this target realize it?»

This boundary is mandatory.

---

102. Canonical compilation pipeline

The intended production pipeline is:

UTF-8 source
      │
      ▼
Lexer
      │
      ▼
Tokens + Spans
      │
      ▼
Parser
      │
      ▼
Canonical AST
      │
      ▼
Name resolution
      │
      ▼
Type checking
      │
      ▼
Effect checking
      │
      ▼
Ownership / linearity / affinity checking
      │
      ▼
Resource analysis
      │
      ▼
Quantum semantic analysis
      │
      ▼
Canonical IR
      │
      ▼
IR verification
      │
      ▼
Optimization
      │
      ▼
Scheduling / routing
      │
      ▼
Target lowering
      │
      ▼
Execution

No backend may bypass the semantic/IR boundary.

---

103. AST contract

Every syntax production that has semantic meaning must map to an explicit AST representation.

The AST must preserve enough information to support:

- diagnostics;
- formatting;
- source mapping;
- semantic analysis;
- refactoring;
- optimization;
- lowering;
- tooling;
- LSP features.

Every AST node must retain a source span.

The existing AST already follows this direction, with "Span" carried throughout declarations and expressions.

---

104. No syntax-only phantom features

A feature must not be considered implemented merely because:

a keyword exists

or:

an AST enum variant exists

or:

a grammar rule exists

Production status requires:

LEXED
PARSED
REPRESENTED
SEMANTICALLY VALIDATED
DIAGNOSTICS
IR LOWERING
IR VERIFICATION
TESTED
DOCUMENTED

A feature may explicitly be marked:

PLANNED
EXPERIMENTAL
PARTIAL
COMPATIBILITY
STABLE

but status must be truthful.

---

105. Canonical grammar vs ANTLR

"grammar/Zamani.g4" must not become a second independent language.

The ANTLR grammar is an implementation/tooling representation of the canonical syntax.

Therefore:

canonical syntax specification
        │
        ├── hand-written parser contract
        │
        └── ANTLR representation

not:

Zamani syntax
├── grammar.md
├── Zamani.g4
├── Zamani-Grammar.md
└── parser.rs

with each independently defining syntax.

Any generated parser must be checked against canonical parser conformance tests.

---

106. Grammar modularity

The grammar implementation should eventually be organized into semantic modules:

grammar/
├── README.md
├── spec/
│   ├── lexical.md
│   ├── syntax.md
│   ├── semantics.md
│   ├── types.md
│   ├── effects.md
│   ├── quantum.md
│   ├── compatibility.md
│   └── conformance.md
│
├── antlr/
│   ├── ZamaniLexer.g4
│   ├── ZamaniParser.g4
│   ├── Core.g4
│   ├── Types.g4
│   ├── Modules.g4
│   ├── Quantum.g4
│   ├── Effects.g4
│   ├── Concurrency.g4
│   └── Meta.g4
│
├── reference/
│   └── ...
│
└── tests/
    ├── valid/
    ├── invalid/
    ├── lexical/
    ├── types/
    ├── patterns/
    ├── quantum/
    ├── effects/
    ├── concurrency/
    ├── temporal/
    └── compatibility/

These files must compose into one language.

They must never define competing syntax.

---

107. Infinite scalability principle

"Infinite" means:

«No artificial language-defined upper bound.»

It does not mean physically unlimited execution.

The actual execution boundary is:

requested resources
        ∩
available resources
        ∩
target capabilities
        ∩
compiler/runtime policy

Therefore the language must not introduce constants such as:

MAX_QUBITS
MAX_THREADS
MAX_ARRAY_SIZE
MAX_AST_DEPTH
MAX_MODULES
MAX_FUNCTIONS
MAX_TENSOR_RANK
MAX_REGISTER_COUNT

unless the constant is explicitly a safety/resource policy outside the language semantics.

---

108. Resource-aware scaling

The compiler must represent resource quantities using scalable semantic representations.

Resource values may be:

known constant
symbolic
inferred
runtime-dependent
target-dependent
unbounded by source syntax

A compiler implementation must not use a machine-sized integer merely because it is convenient if that would create an artificial language limit.

Where resource quantities are represented numerically, the representation must be chosen according to the semantic domain and checked for overflow.

---

109. Quantum scalability

Quantum programs must express:

logical qubits
logical operations
logical measurements
logical dependencies
logical entanglement
logical resources

rather than:

physical qubit 0
physical qubit 1
physical qubit 2
...

Physical allocation belongs to target lowering.

---

110. Hardware independence

The syntax must not contain vendor-specific source requirements.

Vendor functionality must enter through:

dialects
capabilities
target profiles
lowering rules
plugins/backends

rather than source-level grammar forks.

---

111. Dialects

A dialect is an extension of the semantic operation/type namespace, not a replacement for Zamani syntax.

A dialect may define:

operations
types
attributes
constraints
lowering rules
capabilities

but must integrate with canonical AST/IR interfaces.

---

112. Unknown quantum operations

The parser must not reject a quantum operation merely because it is not in a fixed built-in gate list if the operation is syntactically expressible as a semantic operation identifier.

For example:

apply vendor_namespace.CustomOperation(...) to q;

may be syntactically valid.

Semantic analysis determines whether the operation exists.

This prevents the grammar from becoming obsolete whenever quantum hardware evolves.

---

113. Quantum decomposition

A source operation may lower through:

semantic operation
        ↓
abstract operation
        ↓
decomposition
        ↓
target operation set
        ↓
routing
        ↓
scheduling
        ↓
physical execution

The source program must not have to change merely because the target gate set changes.

---

114. Quantum routing

Routing is never a parser responsibility.

The source may express:

apply controlled(U) from control to target;

The routing layer decides whether the target requires:

- swaps;
- teleportation;
- movement;
- remapping;
- ancilla allocation;
- decomposition.

---

115. Scheduling

Scheduling is never grammar-defined.

The scheduler consumes semantic operations and resource/timing constraints.

The source grammar must not contain:

cycle 0
cycle 1
physical_slot 7

unless explicitly describing a low-level target dialect.

---

116. Target capabilities

Targets expose capabilities such as:

supports_operation
supports_type
supports_precision
supports_parallelism
supports_entanglement
supports_measurement
supports_error_correction
supports_memory
supports_transport

The compiler selects or rejects a realization based on these capabilities.

---

117. Simulation

Simulation is a target.

The source language must not become a simulation language merely because a simulator is used.

The same semantic program may target:

simulator
emulator
QPU
hybrid system

without source changes.

---

118. Determinism

Parsing must be deterministic.

For identical:

source
language version
configuration
dialect set

the parser must produce equivalent AST structure.

Diagnostics must be deterministic.

No random number generator may influence parsing.

---

119. Error recovery

The parser must never silently discard malformed source.

Recoverable syntax errors should:

1. emit a diagnostic;
2. retain source span;
3. synchronize at a safe grammar boundary;
4. continue where possible;
5. avoid infinite parser loops.

A production parser must guarantee forward progress during error recovery.

---

120. Diagnostics

Diagnostics must identify:

file
source span
severity
error code
message
optional explanation
optional related spans

Examples of conceptual error classes:

Z001 lexical error
Z002 unexpected token
Z003 missing delimiter
Z004 invalid declaration
Z005 invalid type
Z006 invalid pattern
Z007 invalid quantum operation
Z008 unavailable target capability
Z009 resource requirement failure

Diagnostic numbering is an API and should remain stable once released.

---

121. Source spans

Every meaningful AST node must have a source span.

Spans must be based on source positions, not token-count indexes.

Unicode source must remain correctly diagnosable.

---

122. Semicolon policy

Semicolons may terminate statements.

Where the parser supports optional semicolons, omission must be unambiguous.

The language must not introduce implicit semicolon rules that create incompatible parsing interpretations.

---

123. Reserved syntax

Syntax may be reserved for future features only when explicitly documented.

Reserved tokens must not accidentally become valid identifiers.

A reserved feature must be classified:

reserved
experimental
implemented
stable
deprecated

---

124. Compatibility

Language evolution must use explicit versions.

A source file may declare:

language Zamani "1.0";

Compatibility behavior must be defined by the language version.

A compiler must not silently reinterpret an old program using new incompatible syntax.

---

125. Backward compatibility

New syntax should preferably be:

additive
unambiguous
context-sensitive only where necessary

Breaking changes require:

language version
migration documentation
diagnostic
conformance tests

---

126. Implementation synchronization

The following must remain synchronized:

grammar/spec/syntax.md
src/lexer.rs
src/parser.rs
src/ast/
grammar/Zamani.g4
conformance tests

Synchronization must be tested automatically.

---

127. Conformance tests

Every grammar production must have at least:

one positive test
one boundary test
one negative test

for production-critical syntax.

Quantum syntax must additionally have:

generic operation tests
multi-resource tests
zero/single/many-resource tests
target-independent tests
invalid semantic tests

---

128. Parser conformance

A syntax test should verify at minimum:

source
  ↓
lexer
  ↓
parser
  ↓
AST

The AST must be compared structurally rather than only checking that parsing succeeded.

---

129. AST/IR conformance

For semantic constructs:

source
→ AST
→ semantic analysis
→ IR

must be tested.

The IR must not contain hidden source-specific semantics.

---

130. No backend leakage

The following are prohibited in canonical grammar:

CUDA-only syntax
LLVM-only source syntax
x86-only registers
ARM-only instructions
vendor QPU identifiers
fixed physical qubit numbers
simulator-specific commands

Such concepts belong in target-specific dialects or backend representations.

---

131. Existing implementation alignment

The repository's current lexer already contains a significantly larger token inventory than the historical implementation grammar documents, including:

quantum
qubit
circuit
entangle
noise
fidelity
surface
logical
parity
mts

as well as advanced system vocabulary.

The production specification therefore distinguishes token existence from language feature completion.

The parser currently dispatches quantum, noise, surface-code, nano, Sankofa, effect, language, and advanced declarations.

The AST contains corresponding semantic structures, including:

QuantumCircuit
NoiseModel
SurfaceCode
NanoAgent
QuantumOp
Entangle
NanoOp
Recall
Remember
Learn
Perform
Zamani
Sasa

and rich type representations including:

Optional
Result
Never
Quantum
Linear
Affine
Temporal
Pi
Sigma
Identity
HKT

which establishes the intended semantic direction.

The IR layer currently contains quantum and nano instructions, but the production architecture requires those instructions to represent semantic operations rather than force the source language to expose target-specific implementation details.

---

132. Critical correction: no fake implementation guarantees

The following must never be written in the specification:

"supported"

when only a lexer token exists.

Likewise:

"implemented"

when only an AST variant exists.

Feature maturity must be measured by the complete pipeline.

---

133. Rust implementation requirement

The Zamani compiler implementation is Rust 2021 with Rust 1.97/1.97.1 as the baseline.

The implementation must:

- compile on the declared minimum Rust version;
- avoid Rust "unsafe";
- avoid unsafe FFI assumptions;
- avoid architecture-sized semantic limits;
- use checked arithmetic where appropriate;
- avoid recursion where unbounded source nesting could cause stack exhaustion;
- provide deterministic diagnostics;
- avoid panics for ordinary malformed user programs.

A malformed source file must produce a compiler diagnostic, not an uncontrolled process crash.

---

134. Parser scalability

Because arbitrary source nesting can be extremely deep, production parser implementation should avoid relying on unbounded host-language recursion wherever practical.

The specification places no artificial nesting limit.

Implementation limits, if required for denial-of-service protection, must be explicit resource policies rather than language semantics.

---

135. AST scalability

AST containers must grow according to available resources.

The compiler must not encode fixed-size arrays such as:

MAX_NODES
MAX_STATEMENTS
MAX_FIELDS
MAX_PARAMETERS
MAX_CHILDREN

as language limits.

Security/resource limits may exist externally and must be represented as compiler policy.

---

136. IR scalability

The IR must represent arbitrary semantic resource counts subject to implementation/resource limits.

The IR must not use:

0 = unknown

or:

u32 because machines usually support it

where this would artificially constrain semantics.

The canonical IR should use resource abstractions capable of representing:

known
unknown
symbolic
runtime-derived
target-derived

quantities.

---

137. Quantum IR boundary

The canonical quantum semantic boundary is:

AST
 ↓
semantic quantum model
 ↓
quantum IR
 ↓
optimization
 ↓
routing
 ↓
scheduling
 ↓
QEC/noise/resource planning
 ↓
target lowering

Optimization, routing, scheduling, ZQN, hardware adapters, and benchmarking must consume canonical quantum IR rather than defining independent quantum operation types.

---

138. No duplicated quantum models

There must be one canonical semantic representation of:

quantum operation
quantum value
quantum resource
quantum measurement
quantum dependency

A backend may define an internal target representation, but it must provide explicit lowering from the canonical representation.

---

139. Resource availability

A program may request more resources than a particular target has.

That does not make the source syntactically invalid.

The compilation system may report:

target cannot realize requested program

at target analysis time.

This preserves POCO-REAF.

---

140. Program portability

The following must be valid as a design principle:

same source
    ↓
different compiler target
    ↓
different realization

For example:

same quantum program
 ├── simulator
 ├── QPU A
 ├── QPU B
 ├── fault-tolerant system
 └── future quantum architecture

without source-language rewrites.

---

141. Semantic equivalence

Optimization and lowering are permitted only when semantic equivalence is preserved.

For quantum programs this includes preservation of:

- observable measurement behavior;
- permitted global phase equivalence where formally applicable;
- entanglement relationships;
- resource ownership;
- effect behavior;
- declared temporal semantics.

---

142. Source-level intent

Zamani syntax should prefer:

what

over:

how

For example:

apply FourierTransform to register;

is preferable as a semantic source construct to requiring the programmer to manually specify every low-level gate.

The compiler may lower it into a target-appropriate implementation.

---

143. Explicit low-level programming

Zamani must still permit lower-level programming when required.

Therefore semantic operations may be progressively lowered:

high-level operation
↓
intermediate operation
↓
primitive operation
↓
target operation

The existence of low-level facilities must not contaminate the high-level canonical grammar.

---

144. Extensibility

New computational domains must be introduced through:

syntax extension
AST extension
semantic model
IR representation
verification
lowering
tests

rather than arbitrary parser branches.

A new feature must answer:

What syntax does it introduce?
What AST node represents it?
What semantic invariants exist?
What types does it use?
What effects does it have?
What resources does it require?
What IR represents it?
How is it verified?
How is it lowered?
How is it tested?

---

145. Production definition

A Zamani syntax feature is production-ready only when all of the following are true:

[ ] Canonical syntax specified
[ ] Lexical behavior specified
[ ] Parser implemented
[ ] AST representation implemented
[ ] Semantic rules implemented
[ ] Type rules implemented
[ ] Diagnostics implemented
[ ] IR lowering implemented
[ ] IR verification implemented
[ ] Target-independent behavior tested
[ ] Negative tests implemented
[ ] Compatibility behavior specified
[ ] Documentation complete

---

146. Definition of done for this specification

This file is complete when:

1. there is one canonical syntax authority;
2. the grammar is compositional;
3. syntax does not contain machine assumptions;
4. quantum syntax is target-independent;
5. quantum gate vocabularies are extensible;
6. resource counts are not hard-coded;
7. AST and semantic boundaries are explicit;
8. parser and lexer conformance can be tested;
9. ANTLR can represent the same language without becoming a competing authority;
10. future language domains can be added without rewriting the language architecture;
11. Rust 1.97/1.97.1 remains the implementation baseline;
12. compiler implementation remains entirely safe Rust.

---

147. Canonical architecture

The final architectural contract is:

                  ZAMANI SOURCE
                        │
                        ▼
                ┌───────────────┐
                │ Canonical     │
                │ Lexer         │
                └───────┬───────┘
                        │
                        ▼
                ┌───────────────┐
                │ Canonical     │
                │ Parser        │
                └───────┬───────┘
                        │
                        ▼
                ┌───────────────┐
                │ Canonical AST │
                └───────┬───────┘
                        │
                        ▼
             ┌──────────────────────┐
             │ Semantic Analysis    │
             │ Types / Effects      │
             │ Ownership / Resource │
             └──────────┬───────────┘
                        │
                        ▼
              ┌────────────────────┐
              │ Canonical Zamani   │
              │ Semantic IR        │
              └─────────┬──────────┘
                        │
          ┌─────────────┼──────────────┐
          ▼             ▼              ▼
      Classical      Quantum         Other
      Optimization   Optimization    Domains
          │             │              │
          └─────────────┼──────────────┘
                        ▼
                ┌───────────────┐
                │ Target-       │
                │ Independent   │
                │ Lowering      │
                └───────┬───────┘
                        │
          ┌─────────────┼───────────────┐
          ▼             ▼               ▼
        CPU            GPU             QPU
          │             │               │
          ▼             ▼               ▼
      Hardware       Hardware        Hardware

The source language remains stable while the realization space evolves.

---

148. Final invariant

The fundamental Zamani language invariant is:

«Syntax expresses computation. Semantic analysis establishes meaning. IR expresses canonical computation. Backends determine realization. Hardware never defines the language.»

Therefore:

Program Once
    ↓
Compile Once
    ↓
Semantic Program
    ↓
Target-independent IR
    ↓
Many realizations

is the architectural basis for:

POCO-REAF

and for scaling:

atom
→ one value
→ one qubit
→ one processor
→ one machine
→ many processors
→ many machines
→ distributed systems
→ heterogeneous systems
→ quantum systems
→ future computational substrates
→ as far as available resources permit

without embedding an artificial upper bound into the Zamani language grammar.

---

149. Normative statement

This document is normative for Zamani syntax.

Where another grammar document conflicts with this specification:

this specification wins

until an explicit language-version change supersedes it.

Historical grammar documents may remain for:

- migration;
- archaeology;
- design history;
- compatibility analysis.

They must not silently define new Zamani syntax.

End of canonical syntax specification.