Zamani Language — Canonical Syntax Specification

Path: "grammar/spec/syntax.md"
Language: Zamani
Status: Normative / Production Architecture
Specification version: 1.0
Minimum Rust implementation: Rust 1.97 / Rust 1.97.1
Implementation safety requirement: No "unsafe" Rust.
Language safety requirement: The stable Zamani language provides no unrestricted "unsafe" escape hatch.
Primary goal: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF).

---

0. Document Contract

0.1 Purpose

This document defines the canonical source-language syntax contract for Zamani.

It establishes the boundary between:

source text
    ↓
lexical analysis
    ↓
tokens + spans
    ↓
parsing
    ↓
domain-neutral AST
    ↓
structural validation
    ↓
semantic/type/effect/resource analysis
    ↓
canonical semantic model
    ↓
canonical IR
    ↓
domain IR
    ↓
optimization
    ↓
routing / scheduling / resilience / QEC / ZQN
    ↓
HAL / target realization
    ↓
execution

The syntax specification defines what source forms are legal.

It does not define:

- machine limits;
- physical topology;
- compiler implementation algorithms;
- runtime implementation;
- quantum routing;
- scheduling algorithms;
- QEC algorithms;
- ZQN implementation;
- calibration;
- physical device identifiers;
- vendor-specific hardware behavior;
- backend-specific IR;
- resource availability;
- execution-provider internals.

---

0.2 Ownership

This file owns:

- source grammar;
- syntactic composition;
- syntax categories;
- grammar-level ambiguity rules;
- syntactic precedence;
- syntactic associativity;
- syntactic extension points;
- syntax-to-AST requirements;
- syntax-to-semantic-model requirements;
- syntax-to-IR integration requirements;
- syntax-level portability requirements.

This file does not own:

- individual Unicode code-point tables;
- tokenization;
- comment recognition;
- whitespace recognition;
- keyword token implementation;
- type checking;
- ownership checking;
- effect checking;
- resource feasibility;
- quantum physical realization;
- hardware capability discovery;
- scheduling;
- routing;
- QEC;
- ZQN;
- optimization algorithms.

Those belong to their respective contracts.

---

0.3 Related authoritative files

The production architecture is:

grammar/DESIGN.md
        │
        ├── architecture
        │
        ▼
grammar/spec/syntax.md
        │
        ├── source syntax
        │
        ├── grammar/Zamani.g4
        ├── src/parser.rs
        └── src/ast/
        
grammar/spec/lexical.md
        │
        └── lexer contract

grammar/spec/type-system.md
        │
        └── type contract

grammar/spec/semantics.md
        │
        └── semantic contract

grammar/spec/resources.md
        │
        └── resource/capability contract

grammar/spec/quantum.md
        │
        └── quantum semantic contract

"grammar/Zamani.g4" is an implementation/tooling representation of this specification.

It is not a competing authority.

---

0.4 Authority rule

The authority hierarchy is:

1. canonical language specification;
2. canonical lexical specification;
3. canonical semantic/type specifications;
4. canonical AST contracts;
5. canonical IR contracts;
6. implementation;
7. generated/documentation representations;
8. historical or aspirational material.

"grammar/Zamani-Grammar.md" may contain proposals and historical designs.

"grammar/grammar.md" describes implementation conformance.

Neither may silently introduce source syntax.

---

0.5 Feature-completion rule

A syntax production does not make a feature implemented.

A feature is production-ready only when:

LEXED
  ↓
PARSED
  ↓
AST REPRESENTED
  ↓
STRUCTURALLY VALIDATED
  ↓
SEMANTICALLY VALIDATED
  ↓
DIAGNOSTICS
  ↓
CANONICAL IR LOWERING
  ↓
IR VERIFICATION
  ↓
COMPILER INTEGRATION
  ↓
RUNTIME/TARGET INTEGRATION
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

A keyword alone is not a feature.

A grammar rule alone is not a feature.

An AST variant alone is not a feature.

---

1. Fundamental Language Model

Zamani is one language with multiple computational domains.

The domains include:

- classical computing;
- systems programming;
- functional programming;
- object-oriented programming;
- generic programming;
- numerical computing;
- symbolic computing;
- scientific computing;
- quantum computing;
- hybrid quantum-classical computing;
- hardware description;
- hardware/software co-design;
- distributed computing;
- parallel computing;
- HPC;
- AI/ML;
- data processing;
- networking;
- cryptography;
- security;
- embedded computing;
- accelerator computing;
- edge computing;
- cloud computing;
- temporal computing;
- Sankofa constructs;
- nano computation;
- future computational models.

These are not separate languages.

They share:

- lexical rules;
- expressions;
- types;
- declarations;
- control flow;
- modules;
- diagnostics;
- source spans;
- semantic analysis;
- resource/capability concepts;
- canonical AST;
- canonical semantic representation;
- canonical IR boundaries.

---

2. POCO-REAF

Zamani source expresses computational intent.

The intended model is:

Program Once
      ↓
Compile Once
      ↓
Run Everywhere
      ↓
Run Anywhere
      ↓
Run Forever

Subject to:

- semantic validity;
- target capability;
- available resources;
- explicit portability policy;
- compiler policy;
- runtime feasibility;
- future compatibility rules.

The source language must not require a particular:

- CPU;
- GPU;
- FPGA;
- QPU;
- accelerator;
- simulator;
- operating system;
- memory capacity;
- network size;
- node count;
- register width;
- vector width;
- quantum topology;
- physical qubit mapping.

---

3. Scalability Contract

The language has no artificial universal upper bound on:

- source-file size;
- number of declarations;
- number of statements;
- number of functions;
- number of modules;
- number of parameters;
- number of generic parameters;
- array dimensions;
- tensor dimensions;
- tensor rank;
- number of classical values;
- number of quantum values;
- number of logical qubits;
- number of quantum operations;
- number of hardware resources;
- number of nodes;
- number of processes;
- number of tasks;
- number of timelines;
- number of agents;
- number of channels;
- number of ports;
- number of HDL instances.

An implementation may impose external resource or safety budgets.

Such a budget is not part of the language semantics.

For example:

language validity:
    valid

compiler policy:
    maximum parser memory = configured policy

execution:
    insufficient resources

must not become:

syntax error:
    program is too large

unless the source violates an actual syntactic rule.

---

4. Formal Grammar Notation

This specification uses EBNF-style notation.

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

Terminal strings are written in double quotes.

Lexical classes are uppercase.

Examples:

IDENT
INTEGER
FLOAT
STRING
CHAR

Semantic constraints are explicitly marked as semantic constraints and are not silently encoded as grammar restrictions.

---

5. Lexical Boundary

Lexical syntax is defined by:

grammar/spec/lexical.md

This syntax specification consumes canonical tokens.

It must not redefine:

- identifier character classes;
- Unicode classification;
- keyword tables;
- escape sequences;
- numeric-token internals;
- comment syntax.

The syntax specification may refer to:

IDENT
INTEGER
FLOAT
STRING
CHAR
BOOL

and canonical operator/delimiter tokens.

---

6. Source Unit

A source unit is:

Program ::= { Attribute | Item } EOF ;

An empty source file is valid.

An implementation may represent a program internally as a list/vector or another scalable collection.

No source-level maximum is permitted.

---

7. Items

Item ::=
      Declaration
    | Statement
    | ModuleDeclaration
    | ImportDeclaration
    | ExportDeclaration
    | Attribute ;

The exact AST representation is implementation-defined only below the canonical semantic boundary.

Every semantically meaningful item must retain its source span.

---

8. Attributes

Attributes are syntactic metadata.

Attribute ::= "#[" AttributeBody "]" ;

AttributeBody ::=
      AttributePath
    | AttributePath "(" [ ArgumentList ] ")"
    | AttributePath "=" Expression ;

AttributePath ::= Path ;

Examples:

#[inline]
#[deprecated]
#[experimental]
#[target(...)]
#[requires(...)]
#[capability(...)]

Attributes may express:

- compilation policy;
- optimization intent;
- diagnostics;
- interoperability metadata;
- resource requirements;
- capabilities;
- portability metadata;
- experimental status;
- domain metadata.

Attributes must not directly mutate compiler global state.

Unknown attributes are handled by semantic/compatibility policy.

---

9. Names and Paths

Name ::= IDENT ;

Path ::= PathSegment { "::" PathSegment } ;

PathSegment ::= IDENT ;

A path is semantic naming syntax.

It may identify:

- modules;
- types;
- functions;
- operations;
- capabilities;
- resources;
- namespaces;
- dialect members;
- interoperability symbols.

The parser must not resolve a path.

Name resolution is a later phase.

---

10. Visibility

Visibility ::=
      "pub"
    | "public"
    | "private"
    | "protected" ;

Visibility semantics belong to the module/type system.

No backend may reinterpret visibility.

---

11. Modifiers

Modifiers are syntactic metadata attached to constructs that explicitly permit them.

Modifier ::=
      "mut"
    | "static"
    | "virtual"
    | "override"
    | "abstract"
    | "async"
    | "linear"
    | "affine" ;

A modifier is valid only where the corresponding declaration grammar permits it.

The existence of a token does not make the modifier universally legal.

---

12. Declarations

Declaration ::=
      VariableDeclaration
    | ConstantDeclaration
    | FunctionDeclaration
    | StructDeclaration
    | EnumDeclaration
    | TraitDeclaration
    | ImplDeclaration
    | ClassDeclaration
    | InterfaceDeclaration
    | TypeAliasDeclaration
    | ModuleDeclaration
    | ResourceDeclaration
    | CapabilityDeclaration
    | DomainDeclaration
    | EffectDeclaration
    | QuantumDeclaration
    | HDLDeclaration
    | ModelDeclaration
    | AgentDeclaration
    | DialectDeclaration
    | MacroDeclaration
    | InteropDeclaration ;

A declaration introduces syntax-level structure.

Name binding occurs later.

---

13. Variable Declarations

VariableDeclaration ::=
    "let"
    ["mut"]
    IDENT
    [":" TypeExpression]
    "="
    Expression
    [";"] ;

Optional compatibility form:

MutableVariableDeclaration ::=
    "var"
    IDENT
    [":" TypeExpression]
    "="
    Expression
    [";"] ;

Whether "var" remains stable or compatibility-only is controlled by the version/compatibility contract.

---

14. Constants

ConstantDeclaration ::=
    "const"
    IDENT
    [":" TypeExpression]
    "="
    Expression
    [";"] ;

Constancy is semantic.

The parser must not attempt compile-time evaluation.

---

15. Functions

FunctionDeclaration ::=
    [Visibility]
    ["async"]
    "fn"
    IDENT
    [GenericParameters]
    "(" [ParameterList] ")"
    [ReturnType]
    [WhereClause]
    FunctionBody ;

FunctionBody ::=
      BlockExpression
    | ";" ;

ReturnType ::= "->" TypeExpression ;

Functions are target-independent.

Calling conventions, ABI details, and backend lowering are not determined by the core syntax.

---

16. Parameters

ParameterList ::= Parameter { "," Parameter } ;

Parameter ::=
    ["mut"]
    Pattern
    [":" TypeExpression]
    ["=" Expression] ;

The number of parameters is not globally bounded.

Semantic analysis determines:

- binding validity;
- type validity;
- default-value validity;
- ownership;
- effects;
- calling semantics.

---

17. Generic Parameters

GenericParameters ::=
    "<"
    GenericParameter { "," GenericParameter }
    ">" ;

GenericParameter ::=
      TypeParameter
    | ValueParameter
    | ResourceParameter
    | CapabilityParameter ;

TypeParameter ::=
    IDENT
    { TypeBound } ;

ValueParameter ::=
    IDENT
    ":"
    TypeExpression ;

ResourceParameter ::=
    IDENT
    ":"
    ResourceType ;

CapabilityParameter ::=
    IDENT
    ":"
    CapabilityType ;

TypeBound ::= ":" TypeExpression ;

Generic parameters may be:

- type-level;
- value-level;
- resource-level;
- capability-level.

Their semantic interpretation belongs to the type/resource systems.

---

18. Where Clauses

WhereClause ::=
    "where"
    WherePredicate { "," WherePredicate } ;

WherePredicate ::=
      IDENT ":" TypeExpression
    | Expression ;

Constraints must be preserved in the AST.

The parser must never discard constraints to simplify parsing.

---

19. Return

ReturnStatement ::= "return" [Expression] [";"] ;

A bare return represents the function's unit/implicit-return semantic according to the type-system contract.

---

20. Blocks

BlockExpression ::= "{" { Statement } [TrailingExpression] "}" ;

TrailingExpression ::= Expression [";"] ;

A block may be an expression.

Block result semantics belong to semantic analysis.

Block depth is not a language-level machine limit.

---

21. Statements

Statement ::=
      Declaration
    | ExpressionStatement
    | ReturnStatement
    | BreakStatement
    | ContinueStatement
    | WhileStatement
    | ForStatement
    | LoopStatement
    | IfStatement
    | MatchStatement
    | TryStatement
    | ThrowStatement
    | EffectStatement
    | ResourceStatement
    | ConcurrencyStatement
    | QuantumStatement
    | HDLStatement
    | ExecutionStatement
    | EmptyStatement ;

---

22. Expression Statements

ExpressionStatement ::= Expression [";"] ;

---

23. Empty Statements

EmptyStatement ::= ";" ;

---

24. If

IfStatement ::=
    "if"
    Expression
    BlockExpression
    [ "else" (IfStatement | BlockExpression) ] ;

The condition must be semantically valid for conditional control flow.

---

25. While

WhileStatement ::=
    "while"
    Expression
    BlockExpression ;

No iteration count is encoded in the grammar.

---

26. For

ForStatement ::=
    "for"
    Pattern
    "in"
    Expression
    BlockExpression ;

The iterable may be:

- finite;
- dynamically sized;
- lazily produced;
- distributed;
- streamed;
- target-generated.

The grammar imposes no fixed iteration count.

---

27. Infinite/Unbounded Loop

LoopStatement ::= "loop" BlockExpression ;

Termination is semantic/runtime behavior.

---

28. Break and Continue

BreakStatement ::= "break" [";"] ;

ContinueStatement ::= "continue" [";"] ;

---

29. Match

MatchStatement ::=
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

---

30. Patterns

Pattern ::=
      WildcardPattern
    | IdentifierPattern
    | LiteralPattern
    | TuplePattern
    | StructPattern
    | EnumPattern
    | OrPattern
    | RangePattern
    | ReferencePattern ;

WildcardPattern ::= "_" ;

IdentifierPattern ::= IDENT ;

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
    IDENT [":" Pattern] ;

EnumPattern ::=
    Path
    [ "(" [ Pattern { "," Pattern } ] ")" ]
    [ "{" [ StructPatternField { "," StructPatternField } ] "}" ] ;

OrPattern ::=
    Pattern "|" Pattern { "|" Pattern } ;

RangePattern ::=
    Literal (".." | "..=") Literal ;

ReferencePattern ::= "&" Pattern ;

Pattern semantics are defined by the type system.

---

31. Expressions

The canonical expression model is precedence-based.

Expression ::= AssignmentExpression ;

AssignmentExpression ::=
    ConditionalExpression
    [ AssignmentOperator AssignmentExpression ] ;

AssignmentOperator ::=
      "="
    | "+="
    | "-="
    | "*="
    | "/=" ;

The parser must implement right-associative assignment.

---

32. Conditional Expressions

ConditionalExpression ::=
    LogicalOrExpression
    [ "?" Expression ":" Expression ] ;

If the implementation reserves "?" for error propagation, conditional expressions must use a distinct versioned grammar form rather than creating lexical ambiguity.

The canonical implementation must choose one meaning per syntactic context.

---

33. Logical Expressions

LogicalOrExpression ::=
    LogicalAndExpression
    { LogicalOrOperator LogicalAndExpression } ;

LogicalOrOperator ::= "||" | "or" ;

LogicalAndExpression ::=
    BitwiseOrExpression
    { LogicalAndOperator BitwiseOrExpression } ;

LogicalAndOperator ::= "&&" | "and" ;

The parser must normalize equivalent spellings into one semantic operator where compatibility permits.

---

34. Bitwise Expressions

BitwiseOrExpression ::=
    BitwiseXorExpression
    { "|" BitwiseXorExpression } ;

BitwiseXorExpression ::=
    BitwiseAndExpression
    { "^" BitwiseAndExpression } ;

BitwiseAndExpression ::=
    EqualityExpression
    { "&" EqualityExpression } ;

"|", "&", and related spellings have one canonical lexical token each.

Their semantic interpretation is determined by operand types.

---

35. Equality

EqualityExpression ::=
    ComparisonExpression
    { EqualityOperator ComparisonExpression } ;

EqualityOperator ::= "==" | "!=" ;

---

36. Comparison

ComparisonExpression ::=
    ShiftExpression
    { ComparisonOperator ShiftExpression } ;

ComparisonOperator ::=
      "<"
    | ">"
    | "<="
    | ">=" ;

---

37. Shift

ShiftExpression ::=
    AdditiveExpression
    { ShiftOperator AdditiveExpression } ;

ShiftOperator ::= "<<" | ">>" ;

---

38. Additive

AdditiveExpression ::=
    MultiplicativeExpression
    { ("+" | "-") MultiplicativeExpression } ;

---

39. Multiplicative

MultiplicativeExpression ::=
    PrefixExpression
    { ("*" | "/" | "%") PrefixExpression } ;

---

40. Prefix

PrefixExpression ::=
      PostfixExpression
    | PrefixOperator PrefixExpression ;

PrefixOperator ::=
      "+"
    | "-"
    | "!"
    | "~"
    | "&" ;

---

41. Postfix Expressions

PostfixExpression ::=
    PrimaryExpression
    { PostfixOperation } ;

PostfixOperation ::=
      CallSuffix
    | IndexSuffix
    | MemberSuffix
    | TrySuffix ;

CallSuffix ::= "(" [ArgumentList] ")" ;

IndexSuffix ::= "[" Expression "]" ;

MemberSuffix ::= "." IDENT ;

TrySuffix ::= "?" ;

"?" is postfix error propagation.

A conditional operator, if retained, must be disambiguated through the canonical parser contract.

---

42. Calls

ArgumentList ::= Argument { "," Argument } ;

Argument ::=
      Expression
    | NamedArgument ;

NamedArgument ::= IDENT ":" Expression ;

There is no universal maximum number of arguments.

---

43. Primary Expressions

PrimaryExpression ::=
      IDENT
    | Literal
    | "(" Expression ")"
    | TupleExpression
    | ArrayExpression
    | MapExpression
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
    | StructLiteral
    | QuantumExpression
    | HDLExpression
    | ResourceExpression
    | EffectExpression
    | TemporalExpression
    | MetaExpression ;

---

44. Tuple Expressions

TupleExpression ::=
    "("
    Expression
    ","
    [ Expression { "," Expression } ]
    ")" ;

A parenthesized expression without a comma is not a tuple.

---

45. Arrays

ArrayExpression ::=
    "["
    [ Expression { "," Expression } ]
    "]" ;

Array size is semantic data.

No universal maximum is permitted.

---

46. Maps

MapExpression ::=
    "{"
    [ MapEntry { "," MapEntry } ]
    "}" ;

MapEntry ::= Expression ":" Expression ;

The parser must distinguish a map literal from a block according to contextual grammar.

---

47. Lambdas

LambdaExpression ::=
    "|"
    [ParameterList]
    "|"
    (BlockExpression | Expression) ;

Example:

let square = |x: Int| x * x;

---

48. Anonymous Functions

AnonymousFunctionExpression ::=
    "fn"
    "(" [ParameterList] ")"
    [ReturnType]
    BlockExpression ;

---

49. If Expressions

IfExpression ::=
    "if"
    Expression
    BlockExpression
    [ "else" (IfExpression | BlockExpression) ] ;

---

50. Match Expressions

MatchExpression ::=
    "match"
    Expression
    "{"
    { MatchArm }
    "}" ;

---

51. Async

AsyncExpression ::= "async" Expression ;

AwaitExpression ::= "await" Expression ;

SpawnExpression ::= "spawn" Expression ;

Scheduling is not syntax.

Thread count is not syntax.

Core count is not syntax.

---

52. Object Construction

NewExpression ::=
    "new"
    Path
    ["(" [ArgumentList] ")"] ;

---

53. Struct Literals

StructLiteral ::=
    Path
    "{"
    [ StructFieldInitializer { "," StructFieldInitializer } ]
    "}" ;

StructFieldInitializer ::= IDENT ":" Expression ;

---

54. Type Expressions

The full type contract belongs to:

grammar/spec/type-system.md

The syntax entry point is:

TypeExpression ::= TypePrimary { TypeSuffix } ;

TypePrimary ::=
      Path
    | PrimitiveType
    | GenericType
    | TupleType
    | FunctionType
    | ArrayType
    | SliceType
    | ReferenceType
    | OptionalType
    | ResultType
    | NeverType
    | QuantumType
    | ResourceType
    | CapabilityType
    | TemporalType
    | DependentType
    | ParenthesizedType ;

---

55. Generic Types

GenericType ::=
    Path
    "<"
    TypeArgument { "," TypeArgument }
    ">" ;

TypeArgument ::=
      TypeExpression
    | Expression ;

The number of generic arguments is not machine-limited.

---

56. Function Types

FunctionType ::=
    "(" [TypeExpression { "," TypeExpression }] ")"
    "->"
    TypeExpression ;

---

57. Tuple Types

TupleType ::=
    "("
    TypeExpression
    ","
    [ TypeExpression { "," TypeExpression } ]
    ")" ;

---

58. Array and Slice Types

ArrayType ::=
    TypeExpression
    "["
    Expression
    "]" ;

SliceType ::=
    TypeExpression
    "[]" ;

The expression in an array type may be:

- constant;
- symbolic;
- generic;
- dependent;
- resource-derived,

subject to the type-system rules.

---

59. References

ReferenceType ::=
    "&"
    ["mut"]
    TypeExpression ;

Ownership and lifetime semantics belong to semantic analysis.

---

60. Optional and Result

OptionalType ::= TypeExpression "?" ;

ResultType ::=
    "Result"
    "<"
    TypeExpression
    ["," TypeExpression]
    ">" ;

NeverType ::= "Never" ;

---

61. Quantum Types

Quantum syntax must remain target-independent.

QuantumType ::=
      "Qubit"
    | "Qubit" "[" Expression "]"
    | "Quantum" "<" TypeExpression ">"
    | Path ;

The actual quantum type semantics belong to:

grammar/spec/quantum.md

and the canonical quantum semantic boundary.

The grammar must not impose a maximum qubit count.

---

62. Resource Types

ResourceType ::=
    "Resource"
    "<"
    TypeExpression
    ">" ;

Resource quantities may be:

- concrete;
- symbolic;
- inferred;
- runtime-dependent;
- target-dependent.

---

63. Capability Types

CapabilityType ::=
    "Capability"
    "<"
    Path
    ">" ;

Capabilities are semantic declarations.

They are not vendor-specific machine identifiers.

---

64. Temporal Types

TemporalType ::=
    "Temporal"
    "<"
    TypeExpression
    ">" ;

Temporal semantics belong outside the parser.

---

65. Dependent Types

DependentType ::=
      "Pi"
      IDENT
      ":"
      TypeExpression
      "."
      TypeExpression

    | "Sigma"
      IDENT
      ":"
      TypeExpression
      "."
      TypeExpression ;

Unicode aliases may be provided by the lexical/version contract if explicitly standardized.

---

66. Identity Types

IdentityType ::=
    "Identity"
    "<"
    Expression
    ","
    Expression
    ">" ;

---

67. Struct Declarations

StructDeclaration ::=
    [Visibility]
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
    ["=" Expression]
    ["," | ";"] ;

---

68. Enum Declarations

EnumDeclaration ::=
    [Visibility]
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
      | "{"
        { StructField }
        "}"
    ]
    ["," | ";"] ;

---

69. Traits

TraitDeclaration ::=
    [Visibility]
    "trait"
    IDENT
    [GenericParameters]
    [":" TypeExpression { "," TypeExpression }]
    "{"
    { TraitItem }
    "}" ;

TraitItem ::=
      FunctionDeclaration
    | TypeDeclaration
    | ConstantDeclaration ;

---

70. Interfaces

InterfaceDeclaration ::=
    [Visibility]
    "interface"
    IDENT
    [GenericParameters]
    "{"
    { InterfaceItem }
    "}" ;

InterfaceItem ::=
      FunctionDeclaration
    | TypeDeclaration
    | ConstantDeclaration ;

---

71. Implementations

ImplDeclaration ::=
    "impl"
    [GenericParameters]
    [Path "for"]
    TypeExpression
    [WhereClause]
    "{"
    { ImplItem }
    "}" ;

ImplItem ::=
      FunctionDeclaration
    | ConstantDeclaration
    | TypeDeclaration ;

---

72. Type Aliases

TypeAliasDeclaration ::=
    [Visibility]
    "type"
    IDENT
    [GenericParameters]
    "="
    TypeExpression
    [";"] ;

---

73. Modules

ModuleDeclaration ::=
    [Visibility]
    "module"
    Path
    (
        BlockExpression
        | ";"
    ) ;

Modules may be arbitrarily nested subject to resources.

---

74. Imports

ImportDeclaration ::=
    "import"
    ImportPath
    ["as" IDENT]
    [";"] ;

ImportPath ::= Path | STRING ;

Package resolution is not parser responsibility.

---

75. Exports

ExportDeclaration ::=
    "export"
    ExportTarget
    [";"] ;

ExportTarget ::=
      Path
    | "{"
      [Path { "," Path }]
      "}" ;

---

76. Effects

EffectDeclaration ::=
    "effect"
    IDENT
    [GenericParameters]
    [WhereClause]
    (
        BlockExpression
        | ";"
    ) ;

EffectExpression ::= "perform" Expression ;

HandleExpression ::=
    "handle"
    Expression
    "with"
    BlockExpression ;

Effect semantics belong to "grammar/spec/effects.md".

---

77. Error Handling

TryExpression ::=
    "try"
    Expression
    { CatchClause }
    [FinallyClause] ;

CatchClause ::=
    "catch"
    ["(" [IDENT ":"] TypeExpression ")"]
    BlockExpression ;

FinallyClause ::= "finally" BlockExpression ;

ThrowStatement ::= "throw" Expression [";"] ;

TryPropagationExpression ::= Expression "?" ;

---

78. Concurrency

Concurrency syntax describes intent.

ConcurrencyStatement ::=
      SpawnStatement
    | ParallelStatement
    | ChannelDeclaration
    | SendStatement
    | ReceiveStatement
    | SynchronizeStatement ;

SpawnStatement ::= "spawn" Expression [";"] ;

ParallelStatement ::=
    "parallel"
    BlockExpression ;

ChannelDeclaration ::=
    "channel"
    IDENT
    [":" TypeExpression]
    [";"] ;

SendStatement ::=
    Expression
    "!"
    Expression
    [";"] ;

ReceiveStatement ::=
    Expression
    "?"
    [";"] ;

SynchronizeStatement ::=
    "synchronize"
    Expression
    [";"] ;

No fixed number of:

- threads;
- cores;
- workers;
- tasks;
- actors.

---

79. Parallelism

Parallelism may be expressed as:

parallel { ... }

or through semantic constructs such as:

map(...)
reduce(...)
pipeline(...)
spawn(...)

The language must not require:

parallel_on_8_cores
parallel_on_128_threads

unless such syntax exists in an explicitly target-specific dialect.

---

80. Resource Intent

Resource syntax is target-independent.

ResourceStatement ::=
      RequirementStatement
    | CapabilityRequirement
    | PreferenceStatement
    | ConstraintStatement
    | ResourceBinding ;

RequirementStatement ::=
    "requires"
    Requirement
    [";"] ;

Requirement ::=
      ResourceRequirement
    | CapabilityRequirement
    | PropertyRequirement ;

---

81. Resource Requirements

ResourceRequirement ::=
    ResourceKind
    ComparisonOperator
    Expression ;

ResourceKind ::= Path ;

Examples:

requires qubits >= n;
requires memory >= required_memory;
requires nodes >= workers;
requires tensor.rank >= rank;

These are semantic requirements.

They are not compiler hard limits.

---

82. Capability Requirements

CapabilityRequirement ::=
    "capability"
    "("
    Path
    [ArgumentList]
    ")" ;

Examples:

requires capability("quantum.measurement");
requires capability("quantum.mid_circuit_measurement");
requires capability("tensor.compute");
requires capability("distributed.communication");

Capability resolution belongs to target/resource analysis.

---

83. Preferences

PreferenceStatement ::=
    "prefer"
    Preference
    [";"] ;

Preference ::=
    Path
    [ArgumentList] ;

A preference is not a requirement.

The compiler may ignore it when doing so preserves program semantics.

---

84. Constraints

ConstraintStatement ::=
    "constrain"
    Expression
    [";"] ;

Constraints may be:

- timing;
- memory;
- reliability;
- energy;
- communication;
- precision;
- fidelity;
- power;
- thermal;
- placement;
- portability.

---

85. Resource Bindings

A portable program may describe abstract resource relationships:

ResourceBinding ::=
    "bind"
    IDENT
    "="
    ResourceExpression
    [";"] ;

Physical binding is not part of the portable core language.

---

86. Hardware/Software Co-Design

Hardware intent is represented separately from physical realization.

HDLDeclaration ::=
      HardwareModuleDeclaration
    | SignalDeclaration
    | PortDeclaration
    | ClockDeclaration
    | InterfaceDeclaration
    | HardwareProcessDeclaration
    | HardwareAssertion
    | HardwareGenerate ;

---

87. Hardware Modules

HardwareModuleDeclaration ::=
    [Visibility]
    "hardware"
    "module"
    IDENT
    [GenericParameters]
    "{"
    { HDLDeclaration | Statement }
    "}" ;

---

88. Ports

PortDeclaration ::=
    "port"
    IDENT
    ":"
    PortType
    ["=" Expression]
    [";"] ;

PortType ::=
      "input"
    | "output"
    | "inout" ;

Where richer direction/type syntax is required, the type-system/HDL contracts extend "PortType".

---

89. Signals

SignalDeclaration ::=
    "signal"
    IDENT
    [":" TypeExpression]
    [";"] ;

---

90. Registers

RegisterDeclaration ::=
    "register"
    IDENT
    ":"
    TypeExpression
    [";"] ;

Register width is determined by the declared type.

The language must not impose a universal register width.

---

91. Clocking

ClockDeclaration ::=
    "clock"
    IDENT
    [":" TypeExpression]
    [";"] ;

Timing semantics belong to HDL analysis.

---

92. Hardware Processes

HardwareProcessDeclaration ::=
    "process"
    [ProcessSensitivity]
    BlockExpression ;

ProcessSensitivity ::=
    "("
    [Expression { "," Expression }]
    ")" ;

---

93. Hardware Assertions

HardwareAssertion ::=
    "assert"
    Expression
    [";"] ;

The semantic domain determines whether the assertion is:

- simulation-time;
- synthesis-time;
- formal verification;
- runtime verification.

---

94. Hardware Generation

HardwareGenerate ::=
    "generate"
    IDENT
    "in"
    Expression
    BlockExpression ;

Generation counts are semantic.

No fixed instance count exists.

---

95. Quantum Computing

Quantum syntax is part of Zamani's common language.

It does not create a separate quantum language.

The canonical quantum semantic boundary is:

Zamani AST
   ↓
quantum semantic model
   ↓
quantum::ir
   ↓
optimization
   ↓
routing
   ↓
scheduling
   ↓
QEC / resilience / ZQN
   ↓
HAL
   ↓
target

---

96. Quantum Declarations

QuantumDeclaration ::=
      QuantumCircuitDeclaration
    | QuantumRegisterDeclaration
    | QuantumOperationDeclaration
    | QuantumObservableDeclaration
    | QuantumNoiseDeclaration
    | QuantumErrorCorrectionDeclaration ;

---

97. Quantum Circuits

QuantumCircuitDeclaration ::=
    "quantum"
    "circuit"
    IDENT
    [GenericParameters]
    [ParameterList]
    BlockExpression ;

Compatibility syntax may allow:

circuit Name { ... }

if explicitly enabled by the compatibility policy.

All forms normalize to one semantic representation.

---

98. Quantum Registers

QuantumRegisterDeclaration ::=
    "qubit"
    IDENT
    [":" TypeExpression]
    ["=" Expression]
    [";"] ;

Parameterized allocation may use:

qubit q[n];

provided the type/resource contract defines the form.

There is no maximum "n" in the language.

---

99. Quantum States

Quantum state values may use the canonical literal forms defined by the lexical contract.

They are values, not physical identifiers.

Example:

let zero = |0⟩;
let plus = |+⟩;

---

100. Quantum Operations

The core quantum grammar must not enumerate gates.

The canonical form is generic:

QuantumOperationExpression ::=
    "apply"
    QuantumOperationSpecifier
    QuantumTargetClause
    [";"] ;

QuantumOperationSpecifier ::=
      Path
    | Path "(" [ArgumentList] ")"
    | ControlledOperation
    | AdjointOperation
    | PoweredOperation ;

---

101. Quantum Targets

QuantumTargetClause ::=
      "to" QuantumTargetList
    | "from" QuantumTargetList "to" QuantumTargetList
    | "on" QuantumTargetList ;

QuantumTargetList ::=
    QuantumTarget { "," QuantumTarget } ;

QuantumTarget ::=
      Expression
    | RangeExpression ;

---

102. Controlled Operations

ControlledOperation ::=
    "controlled"
    "("
    QuantumOperationSpecifier
    ")"
    [ "by" QuantumTargetList ] ;

---

103. Adjoint Operations

AdjointOperation ::=
    "adjoint"
    "("
    QuantumOperationSpecifier
    ")" ;

---

104. Powered Operations

PoweredOperation ::=
    "power"
    "("
    QuantumOperationSpecifier
    ","
    Expression
    ")" ;

---

105. Custom Quantum Operations

Any syntactically valid operation identifier may represent a quantum operation:

apply H to q;
apply vendor.operation(theta) to q;
apply custom_namespace.CustomGate(a, b) to q0, q1;

The parser must not require the operation to be in a fixed built-in gate list.

Semantic analysis determines whether it exists and whether the target can realize it.

This is essential for future hardware and POCO-REAF.

---

106. Measurement

QuantumMeasurementStatement ::=
    "measure"
    QuantumTargetList
    [ "into" Expression ]
    [";"] ;

Measurement semantics belong to quantum semantic analysis.

---

107. Reset

QuantumResetStatement ::=
    "reset"
    QuantumTargetList
    [";"] ;

---

108. Barrier

QuantumBarrierStatement ::=
    "barrier"
    QuantumTargetList
    [";"] ;

A barrier is semantic intent.

It does not directly assign hardware cycles.

---

109. Classical Feed-Forward

Quantum operations may depend on classical expressions:

QuantumConditionalStatement ::=
    "if"
    Expression
    QuantumStatement ;

This permits:

if result == 1 {
    apply correction to q;
}

No fixed number of classical measurement bits is assumed.

---

110. Quantum Noise

QuantumNoiseDeclaration ::=
    "noise"
    IDENT
    [GenericParameters]
    BlockExpression ;

Noise semantics belong to ZQN/quantum semantic infrastructure.

The grammar does not implement a noise model.

---

111. Quantum Error Correction

QuantumErrorCorrectionDeclaration ::=
    "error_correction"
    IDENT
    [GenericParameters]
    BlockExpression ;

Source syntax may describe intent such as:

requires capability("quantum.error_correction");

QEC implementation remains outside syntax.

---

112. Logical Quantum Computation

The language may express:

logical
surface
parity

through semantic constructs.

They must not become hard-coded physical layouts.

No:

MAX_QUBITS
MAX_CODE_DISTANCE
MAX_STABILIZERS

may be language-level limits.

---

113. Quantum Resource Requirements

QuantumResourceRequirement ::=
    "requires"
    QuantumResourceProperty
    ComparisonOperator
    Expression
    [";"] ;

QuantumResourceProperty ::=
      "qubits"
    | "logical_qubits"
    | "fidelity"
    | "coherence"
    | "measurement"
    | "connectivity"
    | "error_rate"
    | Path ;

These are requirements, not physical mappings.

---

114. Quantum Routing Boundary

Routing is not grammar responsibility.

The source may state:

apply controlled(U) from control to target;

The routing layer may decide to use:

- swaps;
- teleportation;
- remapping;
- movement;
- decomposition;
- ancillas;
- target-specific operations.

The source does not change merely because the target topology changes.

---

115. Quantum Scheduling Boundary

The grammar must not encode:

cycle 0
cycle 1
physical_slot 7
qubit 13 at time 4

in the portable core language.

Explicit low-level scheduling syntax may exist only inside a target-specific dialect.

---

116. Hybrid Quantum-Classical Computation

Hybrid computation is represented using the same language.

Example structure:

classical expression
    ↓
quantum operation
    ↓
measurement
    ↓
classical condition
    ↓
quantum operation

The syntax therefore permits quantum statements inside ordinary control flow.

No separate parser is required for hybrid programs.

---

117. Classical Computing

Classical computation uses the universal syntax:

- variables;
- functions;
- loops;
- conditionals;
- patterns;
- types;
- generics;
- collections;
- mathematical expressions;
- concurrency;
- resources.

Specialized numerical operations should normally be expressed as:

generic operation
+
typed arguments
+
capability/intrinsic/library semantics

rather than as an ever-growing keyword list.

---

118. Mathematical Operations

Operations such as:

- FFT;
- SVD;
- gradient;
- optimization;
- matrix multiplication;
- tensor contraction;
- statistics;
- symbolic differentiation;

should not automatically become reserved keywords.

For example:

fft(signal)
gradient(loss)
svd(matrix)

can be ordinary calls.

The semantic/type system and libraries determine their meaning.

This keeps the grammar extensible.

---

119. Tensor Syntax

Tensor types may be represented through generic types:

Tensor<T>
Tensor<T, Shape>
Tensor<T, shape>

or another canonical type-system representation.

Shape expressions may be:

- static;
- symbolic;
- generic;
- dependent;
- runtime-derived.

The grammar must not impose a maximum tensor rank.

---

120. Data Processing

Data constructs use generic declarations, types, expressions, functions, and resource capabilities.

Optional dedicated syntax may include:

DatasetDeclaration ::=
    "dataset"
    IDENT
    [GenericParameters]
    [":" TypeExpression]
    [ "=" Expression ]
    [";"] ;

StreamDeclaration ::=
    "stream"
    IDENT
    [":" TypeExpression]
    [";"] ;

---

121. AI/ML

AI/ML syntax is part of the universal language.

Optional first-class declarations:

ModelDeclaration ::=
    "model"
    IDENT
    [GenericParameters]
    BlockExpression ;

AgentDeclaration ::=
    "agent"
    IDENT
    [GenericParameters]
    BlockExpression ;

The grammar does not encode:

- PyTorch;
- TensorFlow;
- JAX;
- CUDA;
- ROCm;
- vendor-specific model formats.

Those belong to libraries, interoperability, or dialects.

---

122. Training and Inference

Training/inference should normally use generic expressions:

train(model, data)
infer(model, input)

If dedicated syntax is introduced, it must lower to the same canonical semantic model.

---

123. Distributed Computing

Distributed syntax must remain topology-independent.

DistributedDeclaration ::=
      ServiceDeclaration
    | ProcessDeclaration
    | ActorDeclaration
    | ChannelDeclaration
    | PlacementDeclaration
    | ReplicationDeclaration ;

Example:

distributed {
    spawn worker;
}

The program does not specify a fixed number of nodes.

---

124. Placement

Portable placement describes requirements or preferences:

PlacementDeclaration ::=
    "placement"
    IDENT
    "="
    PlacementExpression
    [";"] ;

Physical node IDs are not part of the portable semantic contract.

---

125. Replication

ReplicationDeclaration ::=
    "replicate"
    Expression
    ["across" Expression]
    [";"] ;

Replication count may be:

- static;
- symbolic;
- resource-dependent;
- dynamically selected.

---

126. Networking

Networking uses abstract endpoints and capabilities.

EndpointExpression ::=
    "endpoint"
    "("
    Expression
    ")" ;

The language may describe:

- connections;
- protocols;
- channels;
- requests;
- responses;
- streams;
- services.

Actual addresses and routing are deployment concerns unless explicitly required by a target dialect.

---

127. Security

Security constructs may express:

- identity;
- authorization;
- policies;
- capabilities;
- trust;
- secrets;
- provenance;
- secure computation.

PolicyDeclaration ::=
    "policy"
    IDENT
    BlockExpression ;

CapabilityDeclaration ::=
    "capability"
    IDENT
    [GenericParameters]
    BlockExpression ;

Cryptographic algorithms should not become mandatory parser keywords merely because they exist today.

---

128. Memory

Memory syntax describes semantic memory intent.

Possible forms:

MemoryDeclaration ::=
    "memory"
    IDENT
    [":" TypeExpression]
    [ "=" Expression ]
    [";"] ;

Memory domains may include:

- local;
- shared;
- distributed;
- persistent;
- accelerator;
- quantum;
- device;
- managed.

No fixed capacity belongs to syntax.

---

129. Memory Address Spaces

If address-space syntax is required:

AddressSpaceType ::=
    "AddressSpace"
    "<"
    Path
    ">" ;

The semantic model determines what the address space means.

Physical addresses are target-specific.

---

130. Execution Intent

Execution syntax expresses policies, not hardware internals.

ExecutionStatement ::=
      RunStatement
    | ScheduleStatement
    | CheckpointStatement
    | RecoverStatement
    | ObserveStatement ;

---

131. Run

RunStatement ::=
    "run"
    Expression
    [";"] ;

---

132. Scheduling Intent

ScheduleStatement ::=
    "schedule"
    Expression
    [";"] ;

This does not define the scheduler.

---

133. Checkpointing

CheckpointStatement ::=
    "checkpoint"
    [Expression]
    [";"] ;

Checkpoint semantics belong to execution/runtime specifications.

---

134. Recovery

RecoverStatement ::=
    "recover"
    [Expression]
    [";"] ;

Recovery orchestration belongs to resilience/runtime layers.

---

135. Observability

ObserveStatement ::=
    "observe"
    Expression
    [";"] ;

Tracing/profiling implementation is outside syntax.

---

136. Resilience

Portable resilience intent may use:

ResilienceDeclaration ::=
    "resilience"
    IDENT
    BlockExpression ;

The implementation integrates with:

QEC
ZQN
HAL
routing
scheduling
optimization
runtime

The grammar does not duplicate those systems.

---

137. Sankofa

Sankofa constructs remain part of Zamani where accepted by the language lifecycle.

RememberStatement ::=
    "remember"
    IDENT
    [":" TypeExpression]
    "="
    Expression
    [";"] ;

RecallExpression ::=
    "recall"
    (
        "(" Expression ")"
        | Expression
    ) ;

LearnExpression ::=
    ("learn" | "infer")
    ["from"]
    Expression ;

WisdomStatement ::=
    "wisdom"
    IDENT
    ["=" Expression]
    [";"] ;

These are semantic constructs.

The parser does not maintain memory.

---

138. Temporal Constructs

TemporalExpression ::=
      "zamani" (BlockExpression | Expression)
    | "sasa" (BlockExpression | Expression) ;

"zamani" and "sasa" are semantic temporal scopes.

They are not textual macros.

---

139. Multi-Timeline Syntax

Where the MTS feature is promoted to stable syntax:

TimelineDeclaration ::=
    "timeline"
    IDENT
    [GenericParameters]
    BlockExpression ;

ForkExpression ::=
    "fork"
    Expression ;

MergeExpression ::=
    "merge"
    Expression
    "with"
    Expression ;

No fixed number of timelines or branches exists.

---

140. MTS Compatibility

The old monolithic "MTS_LITERAL" token must not become a second syntax authority.

Preferred production architecture:

mts [ expression ]

is parsed using normal:

IDENT + "[" + Expression + "]"

unless a future lexical specification explicitly proves that a dedicated literal token is required.

This avoids domain-specific lexical coupling.

---

141. Nano Computing

Nano computation remains a domain, not a second language.

NanoDeclaration ::=
      NanoAgentDeclaration
    | NanoEntityDeclaration
    | NanoInteractionDeclaration ;

NanoAgentDeclaration ::=
    ("nano" "agent" | "agent" | "nano")
    IDENT
    (BlockExpression | Expression) ;

The grammar must not encode a fixed physical periodic table or device inventory.

---

142. Generic Domain Extensions

Future domains must use extension points rather than modifying unrelated grammar rules.

DomainDeclaration ::=
    "domain"
    IDENT
    [GenericParameters]
    BlockExpression ;

A domain declares:

- name;
- version;
- syntax extensions;
- semantic model;
- AST mapping;
- IR mapping;
- capability requirements;
- compatibility;
- diagnostics.

---

143. Macros

Macros must not bypass semantic analysis.

MacroDeclaration ::=
    "macro"
    IDENT
    [GenericParameters]
    "(" [ParameterList] ")"
    BlockExpression ;

MacroInvocation ::=
    Path
    "!"
    "(" [ArgumentList] ")" ;

Macro expansion must produce source/AST information sufficient for diagnostics.

---

144. Metaprogramming

MetaExpression ::=
      QuoteExpression
    | UnquoteExpression
    | ReflectionExpression
    | CompileTimeExpression ;

Metaprogramming must remain capability-controlled.

It must not create an unrestricted compiler escape hatch.

---

145. Quote

QuoteExpression ::= "quote" BlockExpression ;

---

146. Unquote

UnquoteExpression ::= "unquote" Expression ;

Exact macro/metaprogramming semantics belong to the metaprogramming contract.

---

147. Compile-Time Expressions

CompileTimeExpression ::=
    "comptime"
    Expression ;

Compile-time evaluation must remain deterministic under a deterministic compiler configuration.

---

148. Interoperability

Interoperability is not the canonical semantic model.

InteropDeclaration ::=
      ForeignDeclaration
    | ABIDeclaration
    | FormatDeclaration ;

---

149. Foreign Functions

ForeignDeclaration ::=
    "extern"
    Path
    BlockExpression ;

The exact FFI contract belongs to interoperability specifications.

---

150. ABIs

ABI syntax describes interoperability intent.

It must not force the entire language to one ABI.

ABI selection may depend on:

- target;
- platform;
- calling convention;
- foreign interface.

---

151. OpenQASM / QIR / HDL Interoperability

OpenQASM, QIR, HDL formats, LLVM-related representations, MLIR-related representations, and vendor formats are interoperability boundaries.

They are not the canonical Zamani semantic model.

The pipeline is:

external representation
        ↓
format frontend
        ↓
canonical semantic representation
        ↓
canonical IR

and the reverse for exporters.

---

152. Dialects

Dialects extend Zamani without silently changing the core language.

DialectDeclaration ::=
    "dialect"
    IDENT
    [Version]
    BlockExpression ;

A dialect must identify:

- name;
- version;
- compatibility;
- syntax extensions;
- semantic extensions;
- AST mapping;
- IR mapping;
- capability requirements.

---

153. Dialect Isolation

A dialect must not:

- redefine core keywords silently;
- change the meaning of existing core syntax;
- create a competing AST;
- create an incompatible semantic model;
- bypass canonical IR;
- bypass security checks;
- introduce unsafe compiler implementation.

Dialect syntax must be explicitly activated.

---

154. Literals

The syntax consumes the lexical literal classes:

Literal ::=
      BooleanLiteral
    | NullLiteral
    | IntegerLiteral
    | FloatLiteral
    | StringLiteral
    | CharLiteral
    | QuantumLiteral ;

The detailed lexical grammar belongs to "grammar/spec/lexical.md".

---

155. Boolean Literals

BooleanLiteral ::= "true" | "false" ;

---

156. Null Literals

Compatibility spellings may include:

NullLiteral ::= "nil" | "null" ;

They normalize to one semantic representation if compatibility policy permits both.

---

157. Integer Literals

Canonical integer tokenization belongs to the lexical specification.

The syntax accepts:

INTEGER

without imposing a machine width.

Literal magnitude checking is semantic/type-system responsibility.

---

158. Floating-Point Literals

The syntax accepts:

FLOAT

as defined by the lexical specification.

The type system determines the destination type.

The parser must not assume:

f64

for every floating literal.

---

159. Strings

StringLiteral ::= STRING ;

The lexical specification owns:

- delimiters;
- escapes;
- Unicode handling;
- termination.

---

160. Characters

CharLiteral ::= CHAR ;

The lexical layer ensures the literal is structurally valid.

Semantic analysis determines whether it is compatible with the destination type.

---

161. Quantum Literals

The lexical contract may define primitive state literals:

|0⟩
|1⟩
|+⟩
|-⟩

These represent source-level quantum values.

They do not identify physical qubits.

General symbolic quantum states must use semantic quantum expressions rather than an ever-growing lexer enumeration.

---

162. Ranges

RangeExpression ::=
      Expression ".." Expression
    | Expression "..=" Expression ;

Range size is semantic.

A range must not have a machine-imposed maximum length.

---

163. Precedence

The canonical precedence hierarchy is:

lowest
│
├── assignment
├── conditional / propagation
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
├── postfix call
├── indexing
└── member access
highest

The parser and ANTLR representation must agree exactly.

Backends must not reinterpret precedence.

---

164. Associativity

Canonical associativity:

Construct| Associativity
assignment| right
conditional| right
range| non-chainable unless explicitly defined
logical| left
bitwise| left
equality| left
comparison| left
shift| left
additive| left
multiplicative| left
prefix| right
call| left/postfix
index| left/postfix
member| left/postfix

Any exception must be explicitly specified.

---

165. Ambiguity Resolution

The grammar must prefer structural/contextual resolution over lexical duplication.

Examples:

a & b

has one "&" token.

Its meaning depends on syntax/type context.

Likewise:

a | b

must not require multiple lexical tokens for the same spelling.

---

166. "_" Handling

The underscore spelling has contextual roles.

It may represent:

- wildcard pattern;
- identifier component where permitted;
- ignored binding;
- placeholder.

The lexer must not produce competing token identities for "_".

The parser determines the syntactic role.

---

167. "@" Handling

Annotations use the generic form:

Annotation ::= "@" Path [ "(" [ArgumentList] ")" ] ;

Domain-specific annotation meanings belong to semantic analysis.

A separate lexical token such as "NANO_ANNOTATION" must not be required merely because the annotation happens to be used by nano syntax.

---

168. Unsafe Policy

There is no stable core production:

UnsafeDeclaration

There is no stable core construct granting arbitrary unsafe access.

The compiler implementation itself must contain no Rust "unsafe".

If the historical token "unsafe" remains reserved for compatibility, it must be rejected as a forbidden construct or handled by an explicit migration diagnostic.

It must never enable:

- arbitrary memory access;
- unchecked compiler internals;
- unrestricted FFI;
- backend bypass;
- semantic-analysis bypass.

---

169. Hardware Independence

The portable grammar must not contain mandatory source constructs for:

CPU0
GPU0
QPU0
core0
thread0
physical_qubit0
memory_bank0
register31
device7
node8

unless they occur inside an explicitly target-specific dialect.

---

170. Resource Requirement vs Implementation Decision

The language distinguishes:

Requirement

requires qubits >= n;

Capability

requires capability("quantum.measurement");

Preference

prefer accelerator("quantum");

Constraint

constrain fidelity >= required_fidelity;

Implementation decision

map q -> physical_device_specific_resource;

The first four can be portable.

The last belongs downstream or inside an explicit target dialect.

---

171. No Hard-Coded Hardware Limits

The grammar must never define:

MAX_QUBITS
MAX_CPUS
MAX_CORES
MAX_THREADS
MAX_GPUS
MAX_FPGAS
MAX_NODES
MAX_MEMORY
MAX_REGISTER_WIDTH
MAX_VECTOR_WIDTH
MAX_TENSOR_RANK
MAX_ARRAY_SIZE
MAX_TIMELINES
MAX_PROCESSES
MAX_CHANNELS
MAX_ACCELERATORS

as universal language constants.

External security/resource limits are allowed.

They must be represented as compiler/runtime policy rather than language semantics.

---

172. Source Span Contract

Every parsed semantic construct must preserve source location.

The canonical span must support:

start byte offset
end byte offset
source identifier
line/column mapping

Ranges are end-exclusive.

The AST must preserve sufficient information for:

- diagnostics;
- source maps;
- formatting;
- refactoring;
- IDE/LSP tooling;
- macro expansion diagnostics;
- provenance.

---

173. Error Recovery

Parser errors must be structured.

A parser must:

- never panic on ordinary invalid user input;
- make deterministic progress;
- report source spans;
- distinguish lexical errors from syntax errors;
- recover where safe;
- avoid silently discarding semantic constructs.

Recovery must never manufacture valid semantic AST nodes from invalid syntax.

---

174. Diagnostics

Diagnostics must contain enough information to identify:

- source;
- span;
- diagnostic category;
- severity;
- stable diagnostic identifier;
- message;
- optional explanation;
- optional suggestion;
- optional related spans.

Examples of categories:

LEXICAL_ERROR
SYNTAX_ERROR
AMBIGUITY_ERROR
UNEXPECTED_TOKEN
UNTERMINATED_CONSTRUCT
INVALID_ATTRIBUTE
INVALID_DIALECT
INVALID_RESOURCE_CONSTRAINT
UNSUPPORTED_FEATURE
DEPRECATED_FEATURE
FORBIDDEN_UNSAFE_CONSTRUCT

---

175. Determinism

Given identical:

source
language version
dialect configuration
feature configuration
compiler configuration

parsing must produce the same:

token sequence
source spans
AST structure
syntax diagnostics

independent of:

- host CPU;
- host GPU;
- operating system;
- thread scheduling;
- locale;
- hardware topology.

---

176. Security

The syntax layer must not:

- execute user code;
- access files;
- access networks;
- invoke devices;
- mutate compiler global state;
- bypass semantic validation;
- bypass resource policies.

Macros and metaprogramming must operate through explicit capability-controlled interfaces.

---

177. Parser Implementation Contract

"src/parser.rs" must implement this syntax contract.

It may use:

- recursive descent;
- Pratt parsing;
- precedence climbing;
- generated parser machinery;

provided behavior is conformant.

The current parser already uses a precedence model covering assignment through member access. The canonical implementation must align its token mapping with this specification and eliminate duplicate lexical concepts rather than preserve accidental implementation distinctions.

---

178. AST Contract

"src/ast/mod.rs" is the canonical frontend AST representation.

Every semantic syntax production must map to an AST representation.

The AST must preserve:

- source spans;
- names;
- structure;
- expressions;
- declarations;
- patterns;
- attributes;
- generic arguments;
- resource intent;
- capability intent;
- domain constructs;
- quantum operation identity;
- HDL intent;
- interoperability information.

The AST must remain domain-neutral at its architectural boundary.

---

179. Generic Operation Contract

Operations must not be represented as a closed enumeration of every possible future operation.

The semantic model should support:

namespace
name
operands
parameters
results
attributes
modifiers
effects
capabilities
source span

This applies particularly to:

- quantum operations;
- tensor operations;
- accelerators;
- hardware operations;
- future computational substrates.

---

180. Quantum AST Contract

Quantum syntax lowers into a generic semantic operation model.

The required conceptual mapping is:

quantum syntax
      ↓
generic AST operation
      ↓
semantic quantum operation
      ↓
quantum::ir

It must not become:

quantum syntax
      ↓
fixed QuantumGate enum
      ↓
second quantum IR

---

181. HDL AST Contract

HDL syntax maps into hardware/co-design semantic structures.

The AST must retain:

- module identity;
- ports;
- signals;
- processes;
- timing intent;
- interfaces;
- assertions;
- generation;
- resource intent.

Physical synthesis details remain downstream.

---

182. Resource AST Contract

Resource syntax maps into semantic requirements.

A resource expression must preserve whether it is:

- requirement;
- capability;
- preference;
- constraint;
- hint;
- binding.

These categories must not be collapsed.

---

183. Semantic Boundary

After AST construction:

AST
 ↓
name resolution
 ↓
type analysis
 ↓
effect analysis
 ↓
ownership/linearity
 ↓
resource analysis
 ↓
domain semantic validation
 ↓
canonical semantic model

Only then does lowering occur.

---

184. Canonical IR Boundary

The source grammar does not define one IR for every domain.

The canonical architecture is:

canonical semantic model
          │
          ├── classical semantic IR
          ├── quantum::ir
          ├── HDL/hardware semantic IR
          ├── distributed semantic IR
          └── other domain IR

These are semantic domain representations, not competing source ASTs.

---

185. Quantum IR

The canonical quantum boundary is:

AST
 ↓
semantic quantum model
 ↓
quantum::ir
 ↓
optimization
 ↓
routing
 ↓
scheduling
 ↓
QEC / resilience / ZQN
 ↓
HAL
 ↓
target

All quantum backends must consume or explicitly lower from this canonical representation.

---

186. Scheduling Boundary

Scheduling consumes semantic operations and constraints.

The grammar must not define:

- physical slots;
- universal cycle counts;
- fixed timing widths;
- hardware-specific schedule algorithms.

A target dialect may expose such concepts explicitly.

---

187. Routing Boundary

Routing consumes:

- operations;
- dependencies;
- topology/capability information;
- resource requirements.

The source grammar does not implement routing.

---

188. QEC Boundary

QEC consumes quantum semantic information.

The source language may express:

requires capability("quantum.error_correction");
requires fidelity >= required_fidelity;

but QEC implementation belongs downstream.

---

189. ZQN Boundary

ZQN represents fault/noise semantics.

The syntax may declare noise or reliability intent.

The grammar must not duplicate ZQN's fault model.

---

190. HAL Boundary

HAL determines actual target capabilities and state.

Source syntax may request capabilities.

It must not require a specific physical implementation in portable code.

---

191. Optimization Boundary

Optimization operates after semantic lowering.

The grammar must not prescribe:

- optimization passes;
- pass order;
- target-specific decomposition;
- register allocation;
- gate cancellation;
- scheduling algorithms.

---

192. Compiler Integration

The compiler must consume syntax through:

lexer
parser
AST
semantic analysis
canonical semantic model
IR

A backend must not parse Zamani source independently.

---

193. Runtime Integration

Runtime consumes compiled semantic/IR representations.

Runtime behavior must not alter source syntax.

Runtime resource failure must be represented as runtime/resource diagnostics rather than retroactively changing syntax validity.

---

194. Tooling Integration

Syntax must support:

- formatter;
- syntax highlighter;
- parser diagnostics;
- LSP;
- refactoring;
- symbol navigation;
- documentation extraction;
- source maps;
- semantic highlighting.

Source spans must be stable.

---

195. Inter-Domain Integration

A Zamani program may contain multiple domains in one compilation unit.

Example conceptual structure:

module application

classical computation

quantum computation

hardware intent

distributed execution

AI model

resource requirements

The parser must not require separate languages.

---

196. Domain Isolation

A domain grammar must own only its domain syntax.

For example:

quantum/

owns quantum syntax.

It must not redefine:

- expressions;
- identifiers;
- modules;
- types globally;
- source spans;
- generic parsing;
- diagnostics.

Likewise:

hdl/

must not create another expression language.

---

197. Extension Rule

A new feature must first define:

feature ID
status
version
syntax
tokens
AST mapping
semantic mapping
IR mapping
compiler consumers
runtime consumers
diagnostics
tests
compatibility
scalability policy
hard-coding audit

Only after those contracts exist may it become stable syntax.

---

198. Compatibility

Existing syntax may be retained through compatibility rules.

Compatibility must distinguish:

STABLE
EXPERIMENTAL
DEPRECATED
COMPATIBILITY
PLANNED
REMOVED

A deprecated syntax must not silently acquire a different meaning.

---

199. Historical Syntax

"grammar/Zamani-Grammar.md" may contain syntax that is:

- historical;
- proposed;
- experimental;
- aspirational.

Such syntax is not legal merely because it appears there.

Promotion requires the full feature-completion process.

---

200. "grammar/grammar.md"

"grammar/grammar.md" is an implementation-conformance reference.

It must distinguish:

SPECIFIED
IMPLEMENTED
PARTIAL
EXPERIMENTAL
DEPRECATED
PLANNED

It must not silently become another source of truth.

---

201. "grammar/Zamani.g4"

"grammar/Zamani.g4" is the canonical ANTLR composition root.

It must eventually contain:

program
item
declaration
statement
expression
type
pattern
domain dispatch

and compose domain-specific grammar components.

It must not become an ever-growing monolithic list of every quantum gate, AI operation, hardware instruction, or vendor API.

---

202. "grammar/antlr/"

"grammar/antlr/" must not become a competing grammar authority.

If legacy ANTLR files remain, they must either:

1. become generated/intermediate tooling artifacts; or
2. be explicitly marked compatibility/historical.

There must ultimately be one canonical ANTLR composition path.

---

203. Lexical Integration

The syntax parser consumes the tokens defined by:

grammar/spec/lexical.md

The lexical specification owns:

- identifiers;
- keywords;
- literals;
- operators;
- delimiters;
- comments;
- Unicode;
- whitespace;
- source token spans.

The syntax specification must not redefine those rules.

---

204. Token Meaning

A lexical token identifies spelling.

The parser determines grammatical role.

For example:

IDENT

may represent:

- variable;
- type;
- function;
- module;
- operation;
- quantum operation;
- resource;
- capability;
- domain symbol.

Semantic resolution determines which.

---

205. No Keyword Explosion

A concept should become a keyword only when it has language-level syntactic meaning that cannot reasonably be represented through:

identifier
+
generic syntax
+
attributes
+
types
+
capabilities
+
library/intrinsic semantics

This is especially important for:

- mathematics;
- AI;
- quantum operations;
- vendor features;
- accelerators;
- networking protocols.

---

206. Fixed Gate Prohibition

The grammar must not contain:

quantumGate
    : H
    | X
    | Y
    | Z
    | CNOT
    | ...

as the universal quantum operation grammar.

"H", "X", "CNOT", and future operations must be representable as operation identifiers.

This permits:

apply H to q;
apply X to q;
apply CNOT to q0, q1;
apply vendor.CustomOperation(a) to q;

without changing the grammar.

---

207. Fixed Hardware Prohibition

The core grammar must not enumerate:

x86
ARM
CUDA
ROCm
specific QPU
specific FPGA
specific GPU
specific accelerator

as mandatory language constructs.

Interoperability and target dialects may name them explicitly.

---

208. Fixed Topology Prohibition

Portable syntax must not require:

linear topology
grid topology
ring topology
specific coupling map
specific physical qubit IDs

A program can express connectivity requirements.

The target realization supplies actual topology.

---

209. Fixed Register Prohibition

The grammar must not assume:

8-bit
16-bit
32-bit
64-bit
128-bit

as universal register widths.

Widths belong to types and target capabilities.

---

210. Fixed Tensor Prohibition

The grammar must not impose:

rank <= N
dimension <= N

as language rules.

---

211. Fixed Distributed-System Prohibition

The grammar must not impose:

N nodes
N workers
N processes
N services
N channels

as universal limits.

---

212. Fixed Timeline Prohibition

Temporal/MTS syntax must not impose:

maximum timelines
maximum branches
maximum history entries

as language semantics.

---

213. Resource Exhaustion

Resource exhaustion is not syntax invalidity.

Examples:

parser memory exhausted
compiler memory exhausted
target lacks requested capability
runtime lacks required memory
QPU lacks required capability

must have distinct diagnostics.

---

214. Small-to-Large Scaling

The same syntax must work conceptually for:

one scalar
one instruction
one qubit
one signal
one tensor element
one node

and:

very large programs
very large tensors
very large quantum systems
large distributed systems
large hardware designs
large AI workloads

subject to actual available resources.

No separate “small” and “large” language is permitted.

---

215. Infinity Clarification

“Scale to infinity” means:

«the language specification does not impose an artificial finite upper bound where the mathematical/semantic model can remain parameterized.»

It does not mean that a finite machine has infinite memory or compute.

Therefore:

unbounded by language semantics

and:

bounded by actual available resources

are intentionally different concepts.

---

216. Safe Rust Requirement

The Rust implementation must:

- compile on Rust 1.97/1.97.1;
- use safe Rust;
- contain no "unsafe";
- avoid unsafe FFI assumptions;
- validate external data;
- avoid unchecked indexing where possible;
- handle malformed input without panics;
- use explicit resource/error handling.

This requirement applies to:

- lexer;
- parser;
- AST;
- grammar tooling;
- conformance tooling;
- compiler integration.

---

217. Parser Resource Policy

The parser may have externally configured safety budgets such as:

maximum diagnostic count
maximum parser recovery work
maximum compiler memory
maximum compilation time

These are implementation policies.

They are not language grammar limits.

---

218. Grammar Conformance Tests

Every syntax production requires:

Positive

At least one valid example.

Negative

At least one invalid example.

Boundary

Smallest valid and structurally difficult cases.

Scalability

Large/generated cases with no artificial language maximum.

Determinism

Repeated parsing produces equivalent AST and diagnostics.

Compatibility

Version-specific behavior is verified.

---

219. Lexical Tests

Tests must cover:

- identifiers;
- keywords;
- Unicode source;
- whitespace;
- comments;
- literals;
- operators;
- delimiters;
- malformed literals;
- malformed escapes;
- invalid characters.

These belong primarily under:

grammar/tests/lexical/

---

220. Expression Tests

Tests must cover:

- precedence;
- associativity;
- assignment;
- calls;
- indexing;
- member access;
- unary operations;
- ranges;
- lambdas;
- blocks;
- conditional forms;
- propagation.

---

221. Type Tests

Tests must cover:

- generic types;
- dependent types;
- quantum types;
- resource types;
- capability types;
- arrays;
- slices;
- function types;
- references;
- optional;
- result;
- linear;
- affine;
- temporal.

---

222. Quantum Tests

Quantum syntax tests must include:

one qubit
many qubits
parameterized qubit count
dynamic allocation
custom operations
parameterized operations
controls
adjoints
measurement
reset
barrier
classical feed-forward
logical operations
resource requirements
unknown operation names
vendor namespace operations

The test suite must not use a maximum qubit count as a language-validity criterion.

---

223. HDL Tests

HDL tests must include:

- modules;
- ports;
- signals;
- registers;
- processes;
- clocking;
- reset;
- timing;
- interfaces;
- assertions;
- generate constructs;
- parameterization;
- co-design.

---

224. Distributed Tests

Tests must cover:

- processes;
- services;
- actors;
- channels;
- messages;
- replication;
- partitioning;
- placement;
- consistency;
- fault tolerance;
- scaling.

No fixed node count is permitted.

---

225. AI Tests

Tests must cover:

- model declarations;
- agent declarations;
- tensor types;
- datasets;
- training;
- inference;
- symbolic operations;
- probabilistic constructs;
- distributed execution;
- accelerator requirements.

Framework-specific syntax must be isolated to interoperability/dialects.

---

226. Resource Tests

Tests must distinguish:

requires
capability
prefer
constrain
bind

They must verify that requirements do not accidentally become implementation decisions.

---

227. Negative Tests

Negative syntax tests must include:

- malformed declarations;
- missing delimiters;
- invalid patterns;
- malformed generic lists;
- malformed quantum operations;
- malformed HDL;
- malformed resource declarations;
- forbidden unsafe constructs;
- invalid dialect activation;
- ambiguous syntax;
- unterminated constructs.

---

228. Hard-Coding Audit

The syntax specification passes its hard-coding audit only if it contains no universal language limits based on:

qubits
CPUs
cores
threads
GPUs
FPGAs
nodes
memory
registers
vector widths
tensor dimensions
timelines
channels
processes
accelerators

Examples such as:

qubit[1024]
matrix<1024, 1024>

remain valid because those are program values.

A universal grammar restriction such as:

qubit_count <= 1024

is prohibited.

---

229. Portability Audit

A portable source program must not require modification merely because it is moved between:

CPU
GPU
FPGA
QPU
simulator
emulator
distributed cluster
edge system
cloud
future computational substrate

provided the destination exposes the capabilities required by the program.

---

230. Capability Failure

If a target cannot satisfy a program's requirements:

source remains syntactically valid

The compiler/target analysis reports:

capability mismatch
resource insufficiency
unsupported semantic requirement

rather than changing the syntax definition.

---

231. Deterministic Syntax

Parsing must not depend on:

- target hardware;
- runtime state;
- network state;
- random choices;
- locale;
- device discovery;
- available QPU;
- available GPU.

Hardware discovery occurs after parsing.

---

232. No Backend Leakage

The core syntax must not require:

- CUDA source syntax;
- LLVM source syntax;
- MLIR source syntax;
- x86 registers;
- ARM instructions;
- vendor QPU identifiers;
- simulator-specific commands.

Those belong to interoperability or target-specific dialects.

---

233. No Semantic Leakage

The parser must not:

- perform type checking;
- resolve symbols;
- select a physical qubit;
- choose a scheduler;
- perform routing;
- choose QEC;
- determine calibration;
- determine physical placement;
- select a backend.

---

234. No Runtime Leakage

The syntax layer must not:

- allocate physical resources;
- open devices;
- communicate with hardware;
- execute quantum operations;
- access networks;
- start processes.

---

235. Source Preservation

Parsing must preserve sufficient information for:

- diagnostics;
- formatting;
- source-to-AST mapping;
- source-to-IR provenance;
- refactoring;
- debugging;
- reproducibility.

---

236. Provenance

Every semantic construct that originates in source must be traceable to source spans.

The downstream pipeline should support:

source
 ↓
AST
 ↓
semantic model
 ↓
IR
 ↓
lowered target

with provenance where the downstream representation supports it.

---

237. Generated Code

Generated source or AST must remain distinguishable from user-written source.

Diagnostics must identify generated locations when possible.

Generated constructs must not silently circumvent semantic validation.

---

238. Formatting

The syntax must be structurally parseable independent of formatting.

Whitespace must not change semantics except where lexical separation is required.

The formatter must operate from the AST or syntax tree rather than guessing source structure.

---

239. Comments

Comments are owned by the lexical specification.

The parser does not assign semantic meaning to ordinary comments.

Documentation comments may be retained as tooling metadata.

---

240. Versioning

A syntax version must be associated with a language/compatibility configuration.

Syntax changes must be classified:

additive
clarifying
deprecated
breaking
experimental
dialect-only

A breaking change requires an explicit compatibility rule.

---

241. Feature Gates

Experimental syntax must require an explicit feature/compatibility mechanism.

Experimental syntax must not silently become stable syntax.

---

242. Reserved Vocabulary

Reserved words are controlled by the lexical keyword registry.

A token being present in the lexer does not make it legal everywhere.

The parser decides whether the token participates in a production.

---

243. Unsupported Reserved Words

A reserved token whose semantic feature is not implemented must produce a deterministic diagnostic such as:

feature not implemented

rather than being parsed into a meaningless AST node.

---

244. Legacy "unsafe"

Historical source may contain:

unsafe

The stable syntax does not define an "UnsafeDeclaration".

If compatibility requires recognizing it, the parser must emit a deterministic migration/forbidden-feature diagnostic.

It must never construct an executable unsafe semantic node.

---

245. Canonical AST Normalization

Equivalent compatibility spellings should normalize to one canonical AST representation.

Examples:

nil
null

may normalize to the same null node.

Likewise:

and
&&

may normalize to one logical-and operation if both are enabled.

Normalization must not lose source provenance.

---

246. Generic Operation Normalization

These:

apply H to q;
apply X to q;
apply vendor.Custom to q;

should use one generic operation representation.

The operation identity is data.

The grammar does not require a new production for every operation.

---

247. Future Computational Substrates

Future domains must be expressible through:

generic operations
generic types
resources
capabilities
effects
constraints
dialects
interoperability

The language must not require redesign merely because a new computational substrate appears.

---

248. Domain Addition Contract

To add a new domain, create:

grammar/<domain>/

only when maintainability requires it.

The domain must define:

README
syntax
AST contract
semantic contract
IR mapping
compiler integration
runtime integration
capabilities
resource model
diagnostics
tests
compatibility
hard-coding audit

It must reuse universal grammar components.

---

249. File Completion Contract

Every grammar/specification file must declare:

File
Purpose
Status
Owns
Does Not Own
Inputs
Outputs
Dependencies
Upstream Contracts
Downstream Consumers
Public Grammar Contract
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
Determinism Tests
Compatibility Tests
Diagnostics
Security
Performance
Hard-Coding Audit
Completion Criteria

This is mandatory for production feature work.

---

250. Completion Criteria for This File

"grammar/spec/syntax.md" is complete only when:

- lexical ownership is delegated to "grammar/spec/lexical.md";
- type ownership is delegated to "grammar/spec/type-system.md";
- semantic ownership is delegated to semantic specifications;
- "Zamani.g4" can represent this syntax;
- "src/parser.rs" can conform to it;
- "src/ast/mod.rs" has an explicit mapping for every semantic production;
- every production has defined precedence where applicable;
- every ambiguous construct has a resolution rule;
- quantum syntax is generic rather than a fixed gate list;
- HDL syntax is target-independent;
- resource syntax distinguishes requirements from implementation;
- distributed syntax has no fixed node limits;
- AI syntax does not hard-code frameworks;
- dialect syntax is isolated;
- interoperability does not become canonical semantics;
- "unsafe" is not a stable language escape hatch;
- no machine-specific universal limits exist;
- source spans are preserved;
- diagnostics are deterministic;
- positive tests exist;
- negative tests exist;
- boundary tests exist;
- scalability tests exist;
- determinism tests exist;
- compatibility tests exist;
- the complete syntax-to-AST-to-semantic-to-IR chain is specified.

---

251. Final Canonical Pipeline

The complete Zamani architecture is:

                         ZAMANI SOURCE
                              │
                              ▼
                  grammar/spec/lexical.md
                              │
                              ▼
                            LEXER
                              │
                        tokens + spans
                              │
                              ▼
                   grammar/spec/syntax.md
                              │
                              ▼
                           PARSER
                              │
                              ▼
                      DOMAIN-NEUTRAL AST
                              │
             ┌────────────────┼────────────────┐
             │                │                │
             ▼                ▼                ▼
           Types           Effects         Resources
             │                │                │
             └────────────────┼────────────────┘
                              │
                              ▼
                     Semantic Analysis
                              │
             ┌────────────────┼────────────────┐
             │                │                │
             ▼                ▼                ▼
         Classical         Quantum            HDL
         Semantic Model    Semantic Model     Semantic Model
             │                │                │
             ▼                ▼                ▼
       Classical IR      quantum::ir       Hardware IR
             │                │                │
             └────────────────┼────────────────┘
                              │
                              ▼
                         Optimization
                              │
                ┌─────────────┼─────────────┐
                │             │             │
                ▼             ▼             ▼
             Routing      Scheduling     Resilience
                │             │             │
                └─────────────┼─────────────┘
                              │
                    QEC / ZQN / Resources
                              │
                              ▼
                             HAL
                              │
                              ▼
                     Target Realization
                              │
          ┌──────────┬────────┼────────┬──────────┐
          ▼          ▼        ▼        ▼          ▼
         CPU        GPU      FPGA      QPU       Future
          │          │        │        │        substrate
          └──────────┴────────┴────────┴──────────┘
                              │
                              ▼
                          EXECUTION

The defining architectural rule is:

«Zamani syntax describes portable computational intent. It does not describe today's hardware limits.»

The compiler, semantic layer, resource system, optimization system, quantum IR, routing, scheduler, QEC, ZQN, HAL, and backend collectively determine how that intent is realized.

---

252. Canonical Examples

252.1 Classical

fn sum<T>(values: T[]) -> T {
    reduce(values)
}

252.2 Parameterized computation

fn process<T, N>(data: Tensor<T, N>) -> Tensor<T, N> {
    transform(data)
}

252.3 Quantum

quantum circuit algorithm(n) {
    qubit q[n];

    apply H to q[0];
    apply vendor_namespace.CustomOperation(theta) to q[0];

    measure q[0] into result;
}

252.4 Hybrid

fn hybrid(input: Data) -> Result {
    let state = prepare(input);

    quantum circuit compute {
        apply H to q;
        measure q into result;
    }

    if result == 1 {
        return classical_correction(input);
    }

    input
}

252.5 Resource intent

requires qubits >= n;
requires capability("quantum.mid_circuit_measurement");
requires capability("distributed.communication");

prefer accelerator("quantum");

252.6 Hardware/software co-design

hardware module Accelerator<T> {
    port input: input;
    port output: output;

    process {
        output = compute(input);
    }
}

252.7 Distributed

distributed {
    spawn worker;
    replicate worker across available_resources;
}

No number of workers or nodes is hard-coded.

252.8 AI

model Model<T> {
    input: Tensor<T>;
    output: Tensor<T>;
}

fn train<T>(model: Model<T>, data: Dataset<T>) {
    train(model, data);
}

---

253. Final Production Rule

A future contributor must never solve a new language feature by merely adding:

keyword
+
grammar rule

Instead:

feature proposal
      ↓
lexical contract
      ↓
syntax contract
      ↓
AST contract
      ↓
semantic contract
      ↓
type/effect/resource contract
      ↓
canonical IR mapping
      ↓
compiler integration
      ↓
runtime/target integration
      ↓
diagnostics
      ↓
positive tests
      ↓
negative tests
      ↓
boundary tests
      ↓
scalability tests
      ↓
determinism tests
      ↓
compatibility tests
      ↓
hard-coding audit
      ↓
STABLE

This is the required definition of production-ready syntax for Zamani.

The language therefore remains extensible from the smallest computation to arbitrarily large computation, while the actual limit at any particular execution is determined by semantic validity, compiler policy, target capabilities, and available resources—not by artificial constants embedded in the grammar.