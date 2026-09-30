Zamani Duplicate-Token Validation and Canonicalization Specification

Path: "grammar/validation/duplicate-tokens.md"
Language: Zamani
Specification role: Normative duplicate-token detection, canonicalization, compatibility, and conformance contract
Status: Production-ready target contract
Specification version: 1.0
Implementation baseline: Rust 1.97 / Rust 1.97.1
Rust edition: Rust 2021
Safety: Zamani-owned Rust implementation MUST use safe Rust only; "unsafe" MUST NOT be used
Canonical grammar composition root: "grammar/Zamani.g4"
Canonical lexical composition: "grammar/lexer/tokens.g4"
Canonical operator lexical component: "grammar/lexer/operators.g4"
Canonical punctuation lexical component: "grammar/lexer/punctuation.g4"
Canonical Rust lexer: "src/lexer.rs"
Canonical parser: "src/parser.rs"
Canonical frontend AST: "src/frontend/ast/" where implemented; existing "src/ast/" compatibility structures MUST NOT silently become a second language authority
Canonical quantum semantic boundary: "quantum::ir"
Portability objective: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)
Scalability objective: No artificial language-level hardware, resource, topology, width, count, or capacity limits

---

1. Purpose

This document defines the production contract for detecting, preventing, resolving, and validating duplicate or conflicting lexical token identities throughout the Zamani repository.

The fundamental rule is:

«One source spelling has one canonical lexical token identity in one language version.»

A source spelling MUST NOT acquire multiple canonical token identities merely because:

- different compiler subsystems use different names;
- an older implementation used another enum variant;
- ANTLR and Rust were developed independently;
- parser productions require different contextual interpretations;
- quantum, classical, HDL, AI, distributed, or other domains use the same spelling;
- a token was historically introduced under another name.

The architecture is:

source characters
       │
       ▼
canonical lexical ownership
       │
       ▼
one canonical token identity
       │
       ├───────────────┐
       ▼               ▼
ANTLR lexical model   Rust lexer
       │               │
       └───────┬───────┘
               ▼
          parser token stream
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
        ┌──────┼───────────────┐
        ▼      ▼               ▼
    classical quantum::ir   HDL/hardware
        │      │               │
        └──────┼───────────────┘
               ▼
      optimization/lowering
               │
       routing/scheduling/
       resilience/QEC/ZQN
               │
               ▼
          target realization

Duplicate-token validation therefore protects the entire compiler pipeline, not merely the lexer.

---

2. Scope

This specification covers:

- duplicate token enum variants;
- duplicate ANTLR lexer rules;
- duplicate source spellings;
- aliases accidentally treated as independent tokens;
- token-name divergence between ANTLR and Rust;
- parser references to obsolete token identities;
- keyword/operator collisions;
- punctuation/operator collisions;
- literal/operator collisions;
- contextual-token collisions;
- compatibility aliases;
- generated lexer vocabulary;
- token-to-AST integration;
- token-to-semantic integration;
- token-to-IR integration;
- source-span preservation;
- diagnostics;
- deterministic tokenization;
- incremental lexing;
- scalability;
- compatibility;
- validation tests.

It does not define:

- operator precedence;
- operator algebra;
- type checking;
- overload resolution;
- semantic interpretation;
- quantum gate semantics;
- QEC;
- ZQN;
- routing;
- scheduling;
- calibration;
- HAL;
- hardware discovery;
- backend implementation.

Those remain owned by their respective contracts.

---

3. Authority Hierarchy

The repository MUST have one lexical authority.

The intended authority chain is:

grammar/specification/lexical.md
          │
          ▼
grammar/lexer/
          │
          ├── tokens.md
          ├── keywords.md
          ├── operators.md
          ├── identifiers.md
          ├── literals.md
          ├── punctuation.g4
          ├── operators.g4
          └── other lexical components
          │
          ▼
grammar/lexer/tokens.g4
          │
          ▼
canonical ANTLR lexer
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

The following files MUST NOT become independent token authorities:

- "grammar/Zamani.g4"
- "grammar/grammar.md"
- "grammar/Zamani-Grammar.md"
- "src/parser.rs"
- "src/ast/mod.rs"
- domain-specific grammar directories
- tests
- dialects

They may consume or document the canonical token model, but they MUST NOT independently redefine it.

---

4. Definition of a Duplicate Token

A duplicate token exists when two or more lexical identities represent the same source spelling or indistinguishable lexical construct within the same language version.

Examples:

Question
QuestionMark

for:

?

or:

BitAnd
Ampersand

for:

&

or:

Arrow
ThinArrow

for:

->

A duplicate also exists when names differ while the source spelling is identical:

EQ : '==';
EQUAL_EQUAL : '==';

This is prohibited.

---

5. Types of Duplication

Duplicate-token validation MUST distinguish at least the following classes.

5.1 Exact lexical duplicate

Two rules recognize the same source spelling.

A -> '&'
B -> '&'

Severity: ERROR

---

5.2 Duplicate implementation identity

Two Rust token variants represent the same canonical lexical token.

BitAnd
Ampersand

Severity: ERROR

---

5.3 Duplicate parser identity

The parser treats two token kinds as equivalent because the lexer can emit either for the same spelling.

Example:

Ampersand | BitAnd

Severity: ERROR after canonicalization

The parser MUST consume one canonical token.

---

5.4 Specification duplicate

Two specification files define different canonical identities for the same spelling.

Example:

tokens.md:
& -> BitAnd

operators.g4:
& -> AMPERSAND

Severity: ERROR

---

5.5 Historical alias

An old token name remains in implementation code but is explicitly mapped to the canonical token and cannot be emitted.

Example:

legacy: QuestionMark
canonical: Question

Severity: WARNING during migration

It becomes an ERROR if the old identity remains observable in the production token stream.

---

5.6 Semantic alias

Two semantic concepts intentionally share a lexical token.

Example:

&

may participate in:

- bitwise conjunction;
- reference syntax.

This is not a duplicate lexical token.

The lexer emits one token.

The parser/semantic layer determines context.

---

6. Fundamental Invariant

For every language version "V":

source spelling
        →
exactly one canonical TokenKind

Formally:

∀ spelling ∈ LexicalSpellings(V):
    cardinality(CanonicalTokenKinds(spelling, V)) = 1

Compatibility aliases MAY exist in implementation source code, but:

cardinality(EmittableTokenKinds(spelling, V)) = 1

MUST remain true.

---

7. Existing Repository Audit

The current repository contains the following known overlapping token identities in "src/lexer.rs":

Arrow
ThinArrow

BitAnd
Ampersand

BitOr
Pipe

Question
QuestionMark

The current parser also consumes overlapping identities, including:

Pipe
BitAnd
Ampersand
ThinArrow
QuestionMark

The current parser therefore demonstrates that the duplication is not merely cosmetic: the parser's precedence and prefix/postfix logic currently depend on duplicate identities.

The current ANTLR operator specification already provides a stronger canonical lexical structure:

THIN_ARROW
AMPERSAND
PIPE

while "?" is currently identified by the punctuation/operator ownership model rather than as a separate alternative operator family.

Therefore the production correction MUST converge all layers onto one canonical identity per spelling.

---

8. Canonical Tokenization Decision

The canonical identities SHALL follow the established canonical ANTLR lexical vocabulary where one already exists.

Rust "TokenType" names use Rust/PascalCase representation of the same canonical identity.

The canonical mappings are:

Source spelling| Canonical ANTLR token| Canonical Rust token| Previous duplicate/overlap
"->"| "THIN_ARROW"| "ThinArrow"| "Arrow"
"=>"| "FAT_ARROW"| "FatArrow"| none
"&"| "AMPERSAND"| "Ampersand"| "BitAnd"
"|"| "PIPE"| "Pipe"| "BitOr"
"?"| "QUESTION_MARK"| "QuestionMark"| "Question"
"::"| "DOUBLE_COLON"| "DoubleColon"| none
"&&"| "LOGICAL_AND"| "LogicalAnd"| none
"||"| "LOGICAL_OR"| "LogicalOr"| none
"=="| "EQUAL_EQUAL"| "Equals"| naming divergence only
"!="| "NOT_EQUAL"| "NotEquals"| naming divergence only
"<"| "LESS"| "LessThan"| naming divergence only
"<="| "LESS_EQUAL"| "LessThanEqual"| naming divergence only
">"| "GREATER"| "GreaterThan"| naming divergence only
">="| "GREATER_EQUAL"| "GreaterThanEqual"| naming divergence only

The important distinction is:

«A different Rust naming convention is not itself a duplicate token.»

Therefore:

EQUAL_EQUAL ↔ Equals
LESS ↔ LessThan

is a cross-representation naming mapping, not two emitted lexical tokens.

However:

BitAnd
Ampersand

is a real duplicate because both exist as Rust "TokenType" variants and the parser currently accepts both.

---

9. Canonical Arrow Token

9.1 "->"

Canonical identity:

ANTLR: THIN_ARROW
Rust:  ThinArrow

"Arrow" MUST NOT remain an independently emittable token.

The parser MUST use:

TokenType::ThinArrow

for all "->" syntax.

All current parser locations using "ThinArrow" remain canonical.

Any parser use of "Arrow" MUST be removed or converted.

---

9.2 "=>"

Canonical identity:

ANTLR: FAT_ARROW
Rust:  FatArrow

There is no second canonical identity.

---

10. Canonical Ampersand Token

10.1 "&"

Canonical identity:

ANTLR: AMPERSAND
Rust:  Ampersand

"BitAnd" MUST NOT be emitted by the lexer.

The parser MUST NOT contain:

Ampersand | BitAnd

It MUST consume:

Ampersand

where the source spelling is "&".

---

10.2 Semantic overload is not lexical duplication

"&" may later be interpreted as:

bitwise conjunction
reference
borrow
other explicitly specified semantic role

The parser and semantic analyzer may distinguish these contexts.

They MUST NOT require different lexical token identities for the same source spelling.

---

11. Canonical Pipe Token

11.1 "|"

Canonical identity:

ANTLR: PIPE
Rust:  Pipe

"BitOr" MUST NOT be emitted.

The parser MUST NOT contain:

Pipe | BitOr

for ordinary "|" operator handling.

---

11.2 Quantum interaction

The source character "|" may also participate in quantum notation.

This does not authorize a second ordinary "|" token.

Quantum literals such as:

|0⟩
|1⟩
|ψ⟩

must be recognized according to the quantum-literal lexical contract before ordinary "PIPE" tokenization is applied where the complete quantum literal form is lexically indivisible.

The validator MUST therefore distinguish:

PIPE

from:

QuantumLiteral

by complete lexical rule and longest-match behavior.

There must not be two independently competing rules for the same complete quantum literal.

---

12. Canonical Question-Mark Token

12.1 "?"

Canonical identity:

ANTLR: QUESTION_MARK
Rust:  QuestionMark

"Question" MUST NOT remain an independently emitted token.

The current parser already expects "QuestionMark" for optional types and postfix try-like syntax, so this canonicalization minimizes parser churn.

---

12.2 "?" semantic contexts

The same lexical token may participate in:

T?
expr?

or another language construct defined by the parser.

The lexer MUST NOT create:

Question
QuestionMark
OptionalTypeQuestion
TryQuestion

for the same source spelling.

---

13. "?." and "??"

These are distinct source spellings and therefore distinct canonical tokens:

?. -> QUESTION_DOT
?? -> NULL_COALESCE

They MUST NOT be tokenized as:

QUESTION_MARK DOT

or:

QUESTION_MARK QUESTION_MARK

when the canonical language specifies the compound operators.

Longest-match behavior is mandatory.

---

14. Compound Operator Rule

For every compound spelling "S" with prefix "P":

S MUST be recognized before P.

Examples:

->  before -
=>  before =
::  before :
..= before ..
..  before .
?.  before ?
??  before ?
!=  before !
<=  before <
>=  before >
==  before =
&&  before &
||  before |
<<  before <
>>  before >
+=  before +
-=  before -
*=  before *
/=  before /
%=  before %
&=  before &
|=  before |
^=  before ^
++  before +
--  before -

The validator MUST test all prefix relationships.

---

15. Token Identity Versus Semantic Operation

The following distinction is mandatory:

lexical token
    ≠
semantic operation

For example:

&

has one token:

Ampersand

but may have multiple semantic interpretations.

Likewise:

|

has one token:

Pipe

but may participate in:

- bitwise operations;
- pattern alternatives;
- quantum syntax where the complete construct is not a "PIPE" token;
- future language contexts.

Semantic context MUST NOT create lexical duplicates.

---

16. Token Identity Versus AST Identity

The AST MUST NOT contain one node merely because two obsolete lexer token names existed.

For example, this is acceptable:

BinaryOperation {
    operator: Ampersand
}

It is not acceptable merely because of historical names to create:

BitAndOperation
AmpersandOperation

unless the semantic language genuinely defines two different operations.

The same principle applies to:

Arrow
ThinArrow

and:

Question
QuestionMark

---

17. Token Identity Versus Semantic Domain

The same token can occur across:

- classical computation;
- quantum computation;
- HDL;
- hardware descriptions;
- distributed computing;
- AI/data computation;
- networking;
- metaprogramming.

A domain MUST NOT create a domain-specific duplicate token merely because it interprets the token differently.

For example:

AMPERSAND

does not become:

QUANTUM_AMPERSAND
HDL_AMPERSAND
GPU_AMPERSAND
CPU_AMPERSAND

The semantic layer owns domain interpretation.

---

18. "NanoAnnotation" Is Not a General Token Family

The current Rust lexer contains:

NanoAnnotation

for examples such as:

@atom
@molecule

The canonical lexical architecture already has:

AT
Identifier

and attribute/name grammar.

Therefore individual semantic annotations MUST NOT become an expanding token catalogue.

These:

@atom
@molecule
@bionano.material

should normally be represented structurally as:

AT + name/path

unless a future lexical specification demonstrates that a complete annotation is required to be indivisible.

"NanoAnnotation" MUST therefore be classified as:

legacy/transitional implementation token

until removed from the production lexer.

It MUST NOT be treated as a second canonical token family.

---

19. "MTSLiteral" Is Not a Duplicate Operator

The current lexer has:

MTSLiteral

for MTS-style syntax.

This is not inherently a duplicate-token problem, but it must not be allowed to compete with ordinary lexical constructs.

The preferred production architecture is structural parsing of the MTS construct.

For example, conceptually:

mts [ expression ]

rather than a growing opaque lexer rule.

If "MTSLiteral" remains temporarily for compatibility, it MUST:

1. have an explicit compatibility status;
2. never overlap ordinary valid source in an ambiguous way;
3. have exactly one lexical owner;
4. preserve its complete source span;
5. map deterministically to the canonical AST;
6. have a migration path to the structural representation.

---

20. "QuantumLiteral" and Ordinary "PIPE"

The validator MUST specifically test the boundary between:

|

and:

|0⟩
|1⟩
|+⟩
|-⟩
|ψ⟩

The lexer must never produce multiple possible token streams for the same source.

For every valid quantum literal:

source
→ exactly one lexical interpretation

must hold.

For an incomplete construct such as:

|

the behavior must be deterministic and documented:

- either "PIPE";
- or a deterministic lexical error;

but never nondeterministically one or the other.

---

21. Keyword/Identifier Collisions

Duplicate-token validation also covers cases where a keyword and identifier rule can recognize the same spelling.

The canonical policy is:

identifier-shaped lexeme
        ↓
keyword registry
        ↓
keyword OR Identifier

There MUST NOT be separate independent lexer rules that both emit different canonical token kinds for the same reserved spelling.

For example:

fn

must not simultaneously be:

Identifier
KeywordFn

in the same language version.

Contextual keywords are handled through parser context or a specified contextual-keyword mechanism, not duplicate lexical rules.

---

22. "true", "false", "nil", and Compatibility Spellings

The current lexer maps:

true  -> KeywordTrue
false -> KeywordFalse
nil   -> KeywordNil
null  -> KeywordNil

This is a valid example of a compatibility spelling sharing one canonical token identity.

The rule is:

true -> one canonical boolean token
false -> one canonical boolean token
nil -> one canonical null token
null -> compatibility spelling of that same token

"nil" and "null" MUST NOT become two canonical semantic null tokens merely because both spellings are accepted.

---

23. Token Names Are Not Source Spellings

The following are different layers:

source spelling
ANTLR token name
Rust TokenType variant
AST operator representation
semantic operation
IR operation

Example:

Source:
    ->

ANTLR:
    THIN_ARROW

Rust:
    ThinArrow

AST:
    function/type/control-flow arrow representation

Semantic:
    context-specific arrow meaning

IR:
    only if the semantic construct requires an IR representation

The validator MUST compare these layers through an explicit mapping.

It MUST NOT flag a naming-convention difference as a duplicate merely because:

THIN_ARROW

and:

ThinArrow

have different textual names.

---

24. Required Canonical Mapping Table

The canonical token registry MUST contain a machine-readable equivalent of the following:

Spelling| Owner| ANTLR| Rust| Status
"("| punctuation| "LPAREN"| "LParen"| stable
")"| punctuation| "RPAREN"| "RParen"| stable
"{"| punctuation| "LBRACE"| "LBrace"| stable
"}"| punctuation| "RBRACE"| "RBrace"| stable
"["| punctuation| "LBRACKET"| "LBracket"| stable
"]"| punctuation| "RBRACKET"| "RBracket"| stable
","| punctuation| "COMMA"| "Comma"| stable
"."| punctuation| "DOT"| "Dot"| stable
";"| punctuation| "SEMICOLON"| "Semicolon"| stable
":"| punctuation| "COLON"| "Colon"| stable
"@"| punctuation| "AT"| "At"| stable
"#"| punctuation| "HASH"| "Hash"| stable
"?"| punctuation/operator contract| "QUESTION_MARK"| "QuestionMark"| stable
"+"| operator| "PLUS"| "Plus"| stable
"-"| operator| "MINUS"| "Minus"| stable
"*"| operator| "STAR"| "Star"| stable
"/"| operator| "SLASH"| "Slash"| stable
"%"| operator| "MODULO"| "Modulo"| stable
"="| operator| "ASSIGN"| "Assign"| stable
"!"| operator| "NOT"| "Not"| stable
"&"| operator| "AMPERSAND"| "Ampersand"| stable
"|"| operator| "PIPE"| "Pipe"| stable
"^"| operator| "CARET"| "Caret"| stable
"~"| operator| "TILDE"| "Tilde"| stable
"<"| operator| "LESS"| "LessThan"| stable
">"| operator| "GREATER"| "GreaterThan"| stable
"=="| operator| "EQUAL_EQUAL"| "Equals"| stable
"!="| operator| "NOT_EQUAL"| "NotEquals"| stable
"<="| operator| "LESS_EQUAL"| "LessThanEqual"| stable
">="| operator| "GREATER_EQUAL"| "GreaterThanEqual"| stable
"&&"| operator| "LOGICAL_AND"| "LogicalAnd"| stable
"||"| operator| "LOGICAL_OR"| "LogicalOr"| stable
"<<"| operator| "LEFT_SHIFT"| "LeftShift"| stable
">>"| operator| "RIGHT_SHIFT"| "RightShift"| stable
"::"| operator| "DOUBLE_COLON"| "DoubleColon"| stable
"->"| operator| "THIN_ARROW"| "ThinArrow"| stable
"=>"| operator| "FAT_ARROW"| "FatArrow"| stable
".."| operator| "DOT_DOT"| "DotDot"| stable
"..="| operator| "DOT_DOT_EQ"| "DotDotEq"| stable
"..."| operator| "ELLIPSIS"| "Ellipsis"| stable
"?."| operator| "QUESTION_DOT"| "QuestionDot"| stable
"??"| operator| "NULL_COALESCE"| "NullCoalesce"| stable

Where a Rust variant does not currently exist, the implementation MUST add the canonical variant rather than inventing another synonym.

---

25. Required Removal of Duplicate Rust Variants

The following variants MUST cease to be independently emitted:

Arrow
BitAnd
BitOr
Question

The canonical variants are:

ThinArrow
Ampersand
Pipe
QuestionMark

They may temporarily remain in source code only when required by a controlled migration, but such variants MUST NOT be constructible from normal lexical scanning after the canonicalization change.

The final production "TokenType" MUST contain no duplicate canonical identities.

---

26. Parser Integration Requirements

"src/parser.rs" currently contains duplicate handling such as:

Pipe
BitAnd | Ampersand

These branches MUST be collapsed.

The parser must instead use the canonical token:

Ampersand

or:

Pipe

depending on source spelling.

For example:

Pipe => Precedence::BitOr,
Ampersand => Precedence::BitAnd,

not:

Pipe => ...
BitAnd | Ampersand => ...

Likewise:

Ampersand

must be used for prefix/reference/borrow syntax wherever the grammar identifies "&".

"ThinArrow" remains the sole token for "->".

"QuestionMark" remains the sole token for "?".

---

27. Parser Precedence Contract

Duplicate-token removal MUST NOT change operator precedence.

The current parser precedence model must be rewritten only in terms of canonical token identities.

For example:

Pipe       -> bitwise OR precedence
Caret      -> bitwise XOR precedence
Ampersand  -> bitwise AND precedence

The precedence table belongs to:

grammar/expressions/precedence.md
grammar/specification/syntax.md
src/parser.rs

but the token identity comes from the lexical contract.

No duplicate token may exist solely to express precedence.

---

28. Prefix Parsing Contract

The current parser has logic equivalent to:

Ampersand | BitAnd

This MUST become:

Ampersand

The parser may still interpret "&" as:

reference
borrow
address/reference expression
bitwise operation

depending on syntactic position.

The token remains singular.

---

29. Postfix "?" Contract

The current parser uses:

QuestionMark

for postfix "?".

That identity is canonical.

The parser MUST NOT additionally accept:

Question

for the same source spelling after migration.

Optional types:

T?

and postfix/error propagation:

expr?

share the same lexical token.

The parser/AST/semantic layer distinguishes their contexts.

---

30. Generic "<" Versus Comparison "<"

"<" MUST NOT have separate lexical identities for:

generic delimiter

and:

less-than operator

The canonical token is:

LESS / LessThan

Parser context determines whether the token participates in generic syntax or comparison syntax.

The same applies to:

>

No:

GenericLess
LessThan

duplicate lexical identities are permitted.

---

31. Colon Versus Double Colon

These are distinct source spellings:

:
::

and therefore legitimately have different tokens:

Colon
DoubleColon

This is not duplication.

The validator MUST understand prefix relationships and distinguish:

same spelling

from:

one spelling being a prefix of another spelling

The latter is resolved by maximal munch.

---

32. Dot Versus Range Operators

The following are distinct:

.
..
..=
...

They require distinct canonical tokens because they are distinct lexical strings.

The validator MUST verify:

.
..
..=
...

cannot produce multiple tokenizations for the same valid source.

---

33. Operator/Punctuation Ownership

Each concrete spelling MUST have exactly one owner.

Current ownership is:

punctuation.g4
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

operators.g4
    +
    -
    *
    /
    %
    =
    !
    &
    |
    ^
    ~
    <
    >
    ==
    !=
    <=
    >=
    &&
    ||
    <<
    >>
    ::
    ->
    =>
    ..
    ..=
    ...
    ?.
    ??

The question mark MUST be explicitly added to the canonical punctuation/operator registry as:

? -> QUESTION_MARK

so that the current Rust implementation and parser are represented by the specification.

No second "?" rule may exist.

---

34. Generated ANTLR Vocabulary

"grammar/lexer/tokens.g4" is the lexical composition root.

It MUST produce exactly one vocabulary identity for every canonical token.

The generated lexer MUST NOT contain:

AMPERSAND
BIT_AND

for the same source spelling.

Likewise:

PIPE
BIT_OR

is forbidden.

The composition validator MUST inspect the generated vocabulary rather than assuming the component grammars are correct.

---

35. "grammar/Zamani.g4" Integration

"grammar/Zamani.g4" is the canonical parser composition root.

It MUST consume the canonical lexical vocabulary.

It MUST NOT:

- define duplicate lexical rules;
- redefine token spellings;
- introduce parser-only aliases that masquerade as lexical tokens;
- directly recreate operator tokens;
- introduce domain-specific duplicates.

All parser rules must reference canonical token identities.

---

36. "grammar/lexer/tokens.md" Integration

"grammar/lexer/tokens.md" currently contains canonicalization guidance, but its naming must be reconciled with the actual ANTLR operator vocabulary.

The duplicate-token validator therefore requires that:

tokens.md
operators.md
operators.g4
punctuation.g4
tokens.g4

all resolve to the same canonical mapping.

No file may claim:

& -> BitAnd

while another canonical file claims:

& -> AMPERSAND

without explicitly declaring the Rust-to-ANTLR representation mapping.

The final documentation should state:

ANTLR AMPERSAND
        ↕
Rust Ampersand

rather than presenting them as competing tokens.

---

37. "grammar/lexer/operators.md" Integration

"operators.md" already identifies:

THIN_ARROW
AMPERSAND
PIPE

as canonical operator names.

This document therefore treats those identities as authoritative for the ANTLR lexical layer.

"operators.md" MUST NOT introduce aliases such as:

ARROW
BIT_AND
BIT_OR

as separately emitted lexer tokens.

If descriptive names are needed, they MUST be documentation aliases only.

---

38. "grammar/lexer/punctuation.g4" Integration

"punctuation.g4" currently identifies "?" as punctuation ownership territory.

The final lexical composition MUST give "?" one canonical token:

QUESTION_MARK

It MUST NOT also define:

QUESTION

for the same spelling.

The parser decides whether the token is used for:

optional type
postfix propagation
other specified syntax

---

39. Rust Lexer Integration

"src/lexer.rs" MUST be the executable implementation of the canonical lexical contract.

The final lexer MUST guarantee:

one spelling
    →
one TokenType

for all stable language spellings.

The keyword map:

HashMap<String, TokenType>

is an implementation detail.

It MUST NOT become an independent lexical specification.

The lexer MUST remain deterministic.

---

40. Rust Safety Contract

The duplicate-token validator MUST reject any proposed implementation that introduces Rust:

unsafe
unsafe fn
unsafe {

into the Zamani lexer or token-validation implementation.

All required token validation can be implemented with safe Rust 1.97/1.97.1.

No unsafe memory access, unsafe FFI, raw pointer manipulation, or unchecked memory assumptions are necessary.

---

41. AST Integration

The AST must preserve the canonical token identity where operator identity is structurally relevant.

The current AST contains:

Expression::Prefix(..., TokenType, ...)
Expression::Infix(..., TokenType, ...)
Expression::CompoundAssign(..., TokenType, ...)

This means duplicate-token elimination directly affects AST construction.

The AST MUST receive only canonical tokens.

For example:

&

must never result in:

TokenType::BitAnd

in one parse and:

TokenType::Ampersand

in another equivalent parse.

---

42. AST Domain Neutrality

Token canonicalization MUST NOT introduce domain-specific AST nodes.

Do not solve lexical duplication by creating:

QuantumAmpersand
HardwareAmpersand
GpuAmpersand
HdlAmpersand

The AST remains domain-neutral.

Domain semantics are resolved after parsing.

---

43. Semantic Integration

Semantic analysis may map one canonical token into different semantic operations depending on:

- operand types;
- syntactic context;
- effects;
- capabilities;
- domain;
- resource constraints.

That does not create duplicate lexical tokens.

For example:

Ampersand

may map to a reference/borrow operation or bitwise operation.

The semantic model owns that distinction.

---

44. Quantum Integration

Quantum syntax MUST consume canonical lexical tokens.

No quantum domain may create alternate copies of:

Pipe
Ampersand
QuestionMark
ThinArrow

Quantum operations themselves remain data-driven.

The lexer MUST NOT turn every quantum gate into a separate token.

Therefore:

H
X
Y
Z
CNOT
vendor.operation
custom_gate

must not require a growing lexer token catalogue.

Generic operation syntax is preferred.

The pipeline remains:

source
→ lexer
→ parser
→ domain-neutral AST
→ semantic quantum model
→ quantum::ir
→ optimization
→ routing
→ scheduling
→ QEC/resilience
→ ZQN
→ HAL
→ target

---

45. HDL Integration

HDL syntax may use the same canonical operators:

=
&
|
^
~
<
>
<=
>=
==
!=
<<
>>

No HDL-specific duplicate tokens are permitted merely because the semantics eventually represent gates, signals, registers, state machines, timing, or hardware structures.

For example, the following are forbidden as lexical duplicates:

HDL_AND
AMPERSAND

for the same "&" spelling.

---

46. Classical Integration

Classical vector, matrix, tensor, symbolic, numeric, and scalar computation all consume the same canonical token stream.

The lexer MUST NOT introduce fixed-width variants such as:

BitAnd32
BitAnd64
BitAnd128

or hardware-specific operator tokens.

Type width and representation belong downstream.

---

47. Resource and Capability Integration

Resource expressions may use canonical operators.

For example:

available >= required

uses:

GreaterThanEqual

There is no:

MemoryGreaterEqual
QubitGreaterEqual
GpuGreaterEqual

lexical token.

The semantic/resource layer interprets operands.

This is essential for POCO-REAF.

---

48. Scalability Contract

Duplicate-token validation MUST remain independent of finite hardware capacity.

The validator MUST NOT contain:

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

as language limits.

The validator may use finite implementation data structures to execute the validation itself, but those are implementation-resource considerations and MUST NOT become Zamani language semantics.

---

49. Arbitrary Source Size

The duplicate-token specification must work for:

- tiny source files;
- large source files;
- generated source;
- deeply nested expressions;
- large token streams;
- large Unicode inputs;
- large quantum programs;
- large HDL descriptions;
- distributed programs;
- future computational domains.

Validation MUST NOT assume a fixed maximum number of:

- tokens;
- declarations;
- operators;
- source bytes;
- expressions;
- modules;
- domains.

Practical implementation exhaustion must be reported as an implementation/resource failure, not encoded as a language maximum.

---

50. Determinism

For a fixed:

source
language version
dialect set
lexical configuration

the token stream MUST be deterministic.

Repeated lexical analysis must produce:

same token kinds
same token order
same source spans
same source literals
same diagnostics

subject only to explicitly documented nondeterministic metadata that is not part of the lexical result.

---

51. Incremental Lexing

If the implementation supports incremental lexing:

unchanged source region

MUST produce the same canonical tokenization as a complete lexical pass.

A local edit MUST NOT cause unrelated duplicate token identities to appear.

The validation suite must include edits around every ambiguous boundary:

&
&&
&=
|
||
|=
?
?.
??
-
->
=
=>
<
<=
<<
>
>=
>>
.
..
..=
...
:
::
!
!=

---

52. Unicode

Unicode must not create duplicate lexical identities for ASCII tokens.

For example, the Unicode mathematical symbol:

∧

must not silently become another lexical spelling for:

&

unless the language specification explicitly declares it an alias.

If aliases are added later, they must map to the same canonical token and must not produce multiple canonical token kinds.

---

53. Dialect Integration

A dialect may add lexical vocabulary only under explicit dialect ownership.

A dialect MUST NOT silently redefine a core Zamani token.

Forbidden:

core:
& -> AMPERSAND

dialect:
& -> DialectAnd

while both are active.

A dialect may:

1. introduce a new spelling;
2. explicitly replace a token under a versioned compatibility rule;
3. use an explicit lexical namespace/escape mechanism.

Such changes require compatibility review.

---

54. Macro Integration

Macro token streams must preserve canonical token identity.

Macro expansion MUST NOT create an obsolete token identity.

If macro APIs expose tokens, they must expose canonical token kinds.

A macro must not distinguish:

Ampersand
BitAnd

because "BitAnd" is no longer a canonical lexical identity.

---

55. Metaprogramming Integration

Reflection/introspection over tokens must report canonical identities.

For example:

token.kind("&")

must return the canonical "Ampersand" identity.

It must not depend on which parser path happened to consume the token.

---

56. Interoperability

External formats may have their own token or opcode names.

For example:

OpenQASM
QIR
HDL formats
LLVM/MLIR-related representations
foreign language frontends

must be translated into the canonical Zamani token/AST/semantic model.

External token names MUST NOT be imported as additional canonical Zamani tokens.

---

57. Compatibility Policy

Removing duplicate token identities is a compatibility-sensitive implementation change.

The source language behavior SHOULD remain unchanged for valid programs.

The compatibility contract is:

old source spelling
        ↓
same canonical source meaning

even if the internal token enum changes.

For example, if historical code internally used:

BitAnd

for "&", the migration must preserve the source program:

a & b

without requiring source changes.

---

58. Compatibility Alias Policy

Compatibility aliases may exist only at migration boundaries.

A compatibility alias:

legacy TokenType
        ↓
canonical TokenType

MUST satisfy all of:

- cannot be emitted by the canonical lexer;
- cannot be serialized as a new canonical token;
- cannot be used as a second parser identity;
- has an explicit removal/version policy;
- is covered by compatibility tests;
- does not create ambiguous tokenization.

---

59. Forbidden Compatibility Pattern

The following is prohibited:

match token.token_type {
    TokenType::Ampersand | TokenType::BitAnd => ...
}

after the migration is complete.

That pattern proves that two identities remain active.

The final code must instead be:

match token.token_type {
    TokenType::Ampersand => ...
}

The same rule applies to all duplicate pairs.

---

60. Duplicate Detection Algorithm

The validator should conceptually perform the following checks.

Step 1 — Collect lexical definitions

Collect from:

grammar/lexer/*.g4

all lexer token definitions.

---

Step 2 — Normalize source spellings

Normalize only for validation purposes:

- grammar literal escaping;
- Unicode representation where applicable;
- equivalent ANTLR literal notation.

Do not perform semantic normalization that could hide distinct source spellings.

---

Step 3 — Group by spelling

Build:

spelling → token definitions

---

Step 4 — Detect exact duplicates

If:

spelling → >1 independently emitted token

report:

DUPLICATE_LEXICAL_SPELLING

---

Step 5 — Build canonical registry

Compare against:

grammar/lexer/tokens.md
grammar/lexer/operators.md
grammar/lexer/punctuation.g4
grammar/lexer/operators.g4

---

Step 6 — Inspect Rust "TokenType"

Find every token variant corresponding to the same canonical spelling.

---

Step 7 — Inspect parser references

Find all parser references to obsolete token variants.

---

Step 8 — Inspect AST construction

Find AST construction sites that consume obsolete variants.

---

Step 9 — Inspect tests

Find tests expecting obsolete token identities.

---

Step 10 — Verify generated vocabulary

The final ANTLR vocabulary must contain one canonical emitted identity per spelling.

---

61. Required Diagnostic Classes

The validator MUST expose stable diagnostic identifiers.

At minimum:

ZMN-DUP-TOK-001  Duplicate lexical spelling
ZMN-DUP-TOK-002  Duplicate Rust token identity
ZMN-DUP-TOK-003  ANTLR/Rust token mismatch
ZMN-DUP-TOK-004  Parser references obsolete token
ZMN-DUP-TOK-005  AST references obsolete token
ZMN-DUP-TOK-006  Duplicate keyword spelling
ZMN-DUP-TOK-007  Operator/punctuation ownership collision
ZMN-DUP-TOK-008  Literal/operator lexical collision
ZMN-DUP-TOK-009  Dialect token collision
ZMN-DUP-TOK-010  Compatibility alias emitted
ZMN-DUP-TOK-011  Non-deterministic lexical overlap
ZMN-DUP-TOK-012  Missing canonical token mapping
ZMN-DUP-TOK-013  Generated vocabulary collision
ZMN-DUP-TOK-014  Obsolete token remains in parser
ZMN-DUP-TOK-015  Canonical token has multiple owners

Diagnostic identifiers themselves are not lexical tokens.

---

62. Diagnostic Requirements

Each duplicate-token diagnostic MUST identify, where available:

language version
source spelling
canonical token
conflicting token
owner file
conflicting owner file
ANTLR name
Rust name
parser references
compatibility status
recommended canonical identity

Example conceptual diagnostic:

ZMN-DUP-TOK-002

Duplicate Rust token identity for source spelling '&'.

Canonical:
    Ampersand

Conflicting variant:
    BitAnd

Owner:
    grammar/lexer/operators.g4

Rust implementation:
    src/lexer.rs

Required action:
    Remove BitAnd as an emitted lexical identity.
    Update parser references to consume Ampersand.

---

63. Negative Validation Cases

The validation suite MUST reject:

A : '&';
B : '&';

and:

BitAnd,
Ampersand,

when both are mapped to "&".

It must reject:

Pipe | BitOr

for "|".

It must reject:

Question | QuestionMark

for "?".

It must reject:

Arrow | ThinArrow

for "->".

It must reject:

LESS : '<';
LT   : '<';

when both are emitted tokens.

It must reject:

EQUAL_EQUAL : '==';
EQ           : '==';

as independent emitted identities.

---

64. Positive Validation Cases

The validator MUST accept:

&  -> Ampersand
|  -> Pipe
?  -> QuestionMark
-> -> ThinArrow
=> -> FatArrow

and:

<= -> LessThanEqual
<  -> LessThan

because these are distinct spellings.

It must also accept:

:  -> Colon
:: -> DoubleColon

and:

.   -> Dot
..  -> DotDot
..= -> DotDotEq
... -> Ellipsis

provided the lexer produces exactly one tokenization for each valid source.

---

65. Boundary Tests

Required boundary cases include:

&
&&
&=
|
||
|=
?
?.
??
-
->
-=
--
=
==
=>
<
<=
<<
>
>=
>>
.
..
..=
...
:
::
!
!=
^
^=
+
++
+=
-
--
-=

Every adjacent pair must be validated.

---

66. Quantum Boundary Tests

Required:

|
|0⟩
|1⟩
|+⟩
|-⟩
|ψ⟩
q | r
a | b

The expected tokenization must be specified for each valid language context.

No source may produce two valid token streams.

---

67. Optional-Type Boundary Tests

Required:

T?
expr?
T ? expr
?.
??

The lexical layer must distinguish complete compound operators from the standalone question-mark token according to maximal munch.

The parser then determines whether "QuestionMark" represents:

- optional type syntax;
- postfix propagation;
- another grammar-defined role.

---

68. Arrow Boundary Tests

Required:

-
->
=>
=
==

The lexer must produce:

-  -> Minus
-> -> ThinArrow
=  -> Assign
== -> Equals
=> -> FatArrow

without alternative token streams.

---

69. Ampersand Boundary Tests

Required:

&
&&
&=

Expected:

&  -> Ampersand
&& -> LogicalAnd
&= -> AmpAssign

No "BitAnd" token may be emitted.

---

70. Pipe Boundary Tests

Required:

|
||
|=

Expected:

|  -> Pipe
|| -> LogicalOr
|= -> PipeAssign

No "BitOr" token may be emitted.

---

71. Parser Conformance Tests

For every canonical token, parser tests MUST verify that equivalent source always produces the same AST.

Examples:

a & b

must produce the same AST regardless of whether the program originated from:

- direct source;
- macro expansion;
- incremental parsing;
- module parsing;
- dialect-enabled parsing where no lexical override exists.

The token identity must remain canonical.

---

72. AST Conformance Tests

AST tests MUST assert that:

a & b

does not produce one AST containing "BitAnd" and another containing "Ampersand".

Likewise:

fn f() -> T

must always use the canonical "ThinArrow" representation.

---

73. Semantic Conformance Tests

Semantic tests MUST verify that token canonicalization does not change:

- overload resolution;
- type checking;
- ownership;
- borrowing;
- effect analysis;
- resource analysis;
- capability checking;
- quantum semantics;
- HDL semantics.

Token identity is an implementation-level normalization.

It must not change source meaning.

---

74. IR Conformance

Duplicate-token correction MUST NOT create duplicate IR operations.

For example:

Ampersand

must not force:

BitAndIR
AmpersandIR

unless those are genuinely different semantic operations.

The semantic layer determines the operation.

The canonical IR remains downstream of semantic analysis.

Quantum operations continue through:

quantum::ir

and not through a token-specific quantum IR.

---

75. No Hardware Coupling

Duplicate-token validation MUST NOT inspect or encode:

CPU count
GPU count
FPGA count
QPU count
node count
memory size
register width
network size
device count
qubit count

as lexical constraints.

The same canonical token system must support:

atom-scale
embedded
single CPU
multicore
GPU
FPGA
ASIC
accelerator
QPU
simulator
HPC
cluster
distributed
cloud
future hardware

subject to semantic validity and available resources.

---

76. POCO-REAF Requirement

The duplicate-token architecture contributes to POCO-REAF by ensuring that:

source syntax

does not encode a target-specific token vocabulary.

A program must not need different source operators merely because it targets:

CPU
GPU
FPGA
ASIC
QPU

The lexical layer is therefore target-independent.

---

77. Resource Availability Is Not Lexical Validity

The following must remain separate:

lexically valid
syntactically valid
semantically valid
resource-feasible
target-compatible
executable

A program containing:

a & b

does not become lexically invalid because a target lacks a particular hardware operation.

Lowering may choose another implementation.

---

78. Infinite/Unbounded Conceptual Scalability

"Scalable to infinity" means the language does not establish artificial finite semantic maxima.

It does not require physically infinite memory or computation.

Therefore this validator must distinguish:

unbounded language model

from:

finite implementation resources

A parser or lexer may legitimately fail because the host process exhausts resources.

That failure MUST NOT be represented as:

Zamani supports only N tokens

or any language-level maximum.

---

79. File-by-File Integration Contract

This document is complete only when the following integration responsibilities are respected.

"grammar/validation/duplicate-tokens.md"

Owns:

- duplicate detection policy;
- canonicalization policy;
- validation rules;
- diagnostics;
- migration rules;
- cross-file consistency rules.

Does not own:

- actual token definitions.

---

"grammar/lexer/tokens.md"

Owns:

- canonical token taxonomy;
- token contracts;
- token metadata;
- source-span requirements.

Must consume this document's canonical duplicate rules.

---

"grammar/lexer/operators.md"

Owns:

- operator spellings;
- operator lexical categories.

Must use one canonical token per spelling.

---

"grammar/lexer/operators.g4"

Owns:

- actual operator lexer rules.

Must not contain aliases that emit duplicate tokens.

---

"grammar/lexer/punctuation.g4"

Owns:

- structural punctuation.

Must not duplicate operator spellings.

"?" must have one explicit canonical owner.

---

"grammar/lexer/tokens.g4"

Owns:

- lexical composition.

Must expose one canonical token identity per spelling.

---

"grammar/lexer/keywords.g4"

Owns:

- keyword spellings.

Must not duplicate identifiers as independently emitted tokens.

---

"grammar/Zamani.g4"

Owns:

- parser composition.

Must consume canonical tokens only.

---

"src/lexer.rs"

Owns:

- executable lexical scanning.

Must emit canonical "TokenType" variants only.

---

"src/parser.rs"

Owns:

- syntactic interpretation.

Must not compensate for duplicate lexical identities.

After migration it MUST NOT contain:

Ampersand | BitAnd

or equivalent duplicate handling.

---

"src/ast/"

Owns:

- structural AST representation.

Must receive canonical token identities.

---

"src/frontend/ast/"

Where this architecture is implemented, this remains the preferred domain-neutral frontend AST boundary.

It must not reintroduce lexical duplicates.

---

"grammar/specification/lexical.md"

Owns:

- normative lexical architecture.

Must agree with this document.

---

"grammar/specification/syntax.md"

Owns:

- parser syntax.

Must consume canonical token identities.

---

"grammar/spec/compatibility.md"

Owns:

- compatibility policy.

Must classify duplicate-token canonicalization as an internal implementation change unless observable source behavior changes.

---

"grammar/compatibility/"

Owns:

- migrations;
- versioning;
- deprecated features;
- compatibility matrices.

Legacy token names belong here as migration information, not as canonical lexer definitions.

---

"grammar/grammar.md"

Owns:

- implementation conformance status.

It must report the canonicalized state rather than documenting duplicate identities as current language behavior.

---

"grammar/Zamani-Grammar.md"

Owns:

- historical/broad design material.

It must not silently create token identities.

---

"grammar/tests/"

Owns:

- executable conformance evidence.

Tests must use canonical identities.

---

80. Required Migration Matrix

The implementation migration MUST follow:

Existing identity| Canonical identity| Action
"Arrow"| "ThinArrow"| remove emitted identity
"ThinArrow"| "ThinArrow"| retain
"BitAnd"| "Ampersand"| remove emitted identity
"Ampersand"| "Ampersand"| retain
"BitOr"| "Pipe"| remove emitted identity
"Pipe"| "Pipe"| retain
"Question"| "QuestionMark"| remove emitted identity
"QuestionMark"| "QuestionMark"| retain
"NanoAnnotation"| structural "At + name"| migrate
"MTSLiteral"| structural MTS representation where specified| migrate under compatibility contract

The migration MUST preserve source-level behavior.

---

81. Required Search Audit

Before the duplicate-token change is considered complete, the repository must be searched for every obsolete identity:

Arrow
BitAnd
BitOr
Question

and every canonical identity:

ThinArrow
Ampersand
Pipe
QuestionMark

The search MUST cover:

grammar/
src/
tests/
examples/
benchmarks/
build scripts
code generation
documentation

The audit must distinguish:

canonical reference
historical documentation
compatibility documentation
obsolete implementation reference

A token name appearing in historical documentation is not itself an implementation defect.

---

82. Forbidden Final-State Patterns

The production repository MUST NOT contain executable lexical logic equivalent to:

Arrow | ThinArrow
BitAnd | Ampersand
BitOr | Pipe
Question | QuestionMark

for equivalent spellings.

It MUST NOT contain multiple lexer rules for:

&
|
?
->

with different emitted identities.

It MUST NOT use a dialect to silently redefine these core spellings.

---

83. Required Final-State Properties

A production-ready implementation MUST satisfy all of:

P1  Every source spelling has one canonical lexical identity.

P2  Every canonical lexical identity has one owner.

P3  Every parser operator reference uses a canonical token.

P4  Every AST operator reference uses canonical identity.

P5  ANTLR and Rust tokenization agree.

P6  Compatibility aliases cannot be emitted.

P7  Generated lexer vocabulary contains no duplicate identities.

P8  Compound operators use deterministic maximal munch.

P9  Quantum literals do not collide with ordinary pipe syntax.

P10 Keyword recognition does not create duplicate identifier tokens.

P11 Dialects cannot silently redefine core tokens.

P12 Macros preserve canonical token identity.

P13 Incremental lexing preserves canonical tokenization.

P14 Token spans remain exact.

P15 Diagnostics identify lexical ownership conflicts.

P16 No Rust unsafe is required or introduced.

P17 No hardware capacity is encoded.

P18 No artificial language-level resource maximum is encoded.

P19 Canonicalization does not change valid source semantics.

P20 Compatibility behavior is explicitly tested.

---

84. Production Acceptance Gate

"grammar/validation/duplicate-tokens.md" is considered implemented only when:

Lexical

- [ ] duplicate source spellings are detected;
- [ ] duplicate lexer rules are detected;
- [ ] duplicate Rust token variants are detected;
- [ ] operator/punctuation collisions are detected;
- [ ] keyword/identifier collisions are detected;
- [ ] literal/operator collisions are detected;
- [ ] compound-token prefix collisions are tested.

Canonicalization

- [ ] "->" has one identity;
- [ ] "&" has one identity;
- [ ] "|" has one identity;
- [ ] "?" has one identity;
- [ ] all other operators have one identity;
- [ ] ANTLR and Rust mappings agree.

Parser

- [ ] parser contains no obsolete duplicate-token branches;
- [ ] precedence uses canonical identities;
- [ ] prefix parsing uses canonical identities;
- [ ] postfix parsing uses canonical identities;
- [ ] generic/comparison ambiguity remains contextual rather than lexical.

AST

- [ ] canonical tokens reach the AST;
- [ ] no duplicate operator AST forms were created;
- [ ] source spans remain intact.

Compatibility

- [ ] old source continues to parse;
- [ ] legacy internal token aliases are non-emitting;
- [ ] migration tests exist;
- [ ] diagnostics remain deterministic.

Scalability

- [ ] no machine-size constants exist;
- [ ] no hardware-specific token identities exist;
- [ ] no finite token-count limit is encoded;
- [ ] large source inputs remain conceptually supported;
- [ ] quantum syntax remains unbounded by lexical hardware assumptions.

Safety

- [ ] Rust 1.97/1.97.1 compatibility is maintained;
- [ ] Rust 2021 is maintained;
- [ ] no "unsafe" is introduced.

---

85. Required Test Families

The repository MUST eventually contain tests under:

grammar/tests/lexical/
grammar/tests/syntax/
grammar/tests/negative/
grammar/tests/boundary/
grammar/tests/scalability/
grammar/tests/determinism/
grammar/tests/compatibility/

At minimum:

duplicate_operator_spellings.zm
duplicate_punctuation_spellings.zm
duplicate_keyword_spellings.zm
operator_prefixes.zm
question_mark_contexts.zm
ampersand_contexts.zm
pipe_contexts.zm
arrow_contexts.zm
quantum_pipe_boundaries.zm
generic_comparison_boundaries.zm
dialect_collision.zm
macro_token_identity.zm

Tests must assert token identity as well as successful parsing.

---

86. Validator Implementation Requirements

If a Rust validator is added for this specification, it should use only safe Rust 1.97/1.97.1.

The validator should operate on a canonical registry conceptually equivalent to:

TokenRegistry
 ├── source spelling
 ├── lexical owner
 ├── canonical ANTLR token
 ├── canonical Rust token
 ├── category
 ├── compatibility aliases
 ├── dialect status
 └── source span/documentation location

The registry must not contain hardware-specific limits.

The validator should produce structured diagnostics rather than relying only on text matching.

---

87. No Self-Referential Authority Loop

This document MUST NOT become a fourth token-definition authority.

Its role is validation.

The division is:

lexical specification
    defines what the canonical token is

lexer grammar
    defines how it is recognized

Rust lexer
    implements recognition

duplicate validator
    proves there is only one identity

The validator must never invent new canonical tokens.

---

88. Completion Independence

This file is considered independently complete when it specifies:

1. what constitutes a duplicate;
2. canonical identities;
3. existing repository conflicts;
4. migration rules;
5. ANTLR integration;
6. Rust integration;
7. parser integration;
8. AST integration;
9. semantic integration;
10. IR integration;
11. compatibility;
12. diagnostics;
13. tests;
14. scalability;
15. safety;
16. completion criteria.

Another file being edited later MUST NOT require this document to invent additional duplicate-token policy.

New lexical tokens introduced later MUST instead pass through this existing contract.

---

89. Future Token Addition Protocol

A future feature MUST follow:

feature proposal
      ↓
lexical spelling decision
      ↓
owner selection
      ↓
duplicate-token validation
      ↓
canonical token registry
      ↓
lexer implementation
      ↓
parser integration
      ↓
AST mapping
      ↓
semantic mapping
      ↓
IR mapping where required
      ↓
tests
      ↓
compatibility review
      ↓
stable

No feature may skip duplicate-token validation merely because it belongs to:

- quantum computing;
- HDL;
- AI;
- networking;
- distributed computing;
- metaprogramming;
- future hardware;
- a dialect.

---

90. Final Canonical Rule

The production Zamani language MUST enforce:

ONE SOURCE SPELLING
        ↓
ONE LEXICAL OWNER
        ↓
ONE CANONICAL TOKEN
        ↓
ONE DETERMINISTIC TOKEN STREAM
        ↓
CONTEXTUAL PARSING
        ↓
DOMAIN-NEUTRAL AST
        ↓
SEMANTIC INTERPRETATION
        ↓
CANONICAL IR

Not:

ONE SPELLING
    ↓
MANY TOKEN ENUMS
    ↓
PARSER ACCEPTS ALL OF THEM
    ↓
SEMANTIC LAYER GUESSES

The second architecture is precisely what this specification prevents.

---

91. Final Canonicalization Summary

The current duplicate identities are resolved as follows:

->

    canonical: ThinArrow
    remove:     Arrow


&

    canonical: Ampersand
    remove:     BitAnd


|

    canonical: Pipe
    remove:     BitOr


?

    canonical: QuestionMark
    remove:     Question

The existing ANTLR vocabulary already points toward:

THIN_ARROW
AMPERSAND
PIPE

and the existing parser already uses:

ThinArrow
Ampersand
Pipe
QuestionMark

in substantial portions of the implementation.

Therefore this resolution minimizes unnecessary renaming while eliminating the actual duplicate identities.

---

92. Production End State

The completed lexical architecture must have:

grammar/lexer/
│
├── tokens.md
├── tokens.g4
├── keywords.md
├── keywords.g4
├── operators.md
├── operators.g4
├── punctuation.g4
├── identifiers.md
├── identifiers.g4
├── literals.md
├── quantum-literals.md
├── comments.md
└── ...
        │
        ▼
canonical lexical vocabulary
        │
        ▼
canonical Zamani lexer
        │
        ▼
canonical TokenType identities
        │
        ▼
parser
        │
        ▼
domain-neutral AST
        │
        ▼
semantic model
        │
        ├── classical
        ├── quantum::ir
        ├── HDL/hardware
        ├── AI/data
        ├── distributed
        └── other domains
        │
        ▼
optimization / lowering
        │
        ▼
routing / scheduling / resilience / QEC / ZQN
        │
        ▼
HAL / target realization

No layer may reintroduce:

Arrow
BitAnd
BitOr
Question

as independently emitted lexical identities.

The language remains target-independent, resource-scalable, deterministic, compatible, and safe-Rust implementable.

---

93. Definition of Done

This file and its associated implementation are DONE only when the repository can demonstrate all of the following:

✓ No duplicate source spelling has multiple canonical token identities.
✓ No canonical token has multiple lexical owners.
✓ ANTLR and Rust agree on tokenization.
✓ src/parser.rs consumes canonical identities only.
✓ AST construction consumes canonical identities only.
✓ Legacy aliases cannot be emitted.
✓ Keyword/identifier collisions are deterministic.
✓ Operator/punctuation collisions are deterministic.
✓ Quantum literal/pipe boundaries are deterministic.
✓ Compound operators use maximal munch.
✓ Token source spans remain exact.
✓ Compatibility tests preserve valid source behavior.
✓ Dialects cannot silently redefine core tokens.
✓ Macro/metaprogramming token streams remain canonical.
✓ No artificial hardware limits exist.
✓ No artificial resource limits exist.
✓ No unsafe Rust is required.
✓ Rust 1.97 and Rust 1.97.1 remain supported.
✓ The validator itself is deterministic.
✓ The same source produces the same canonical token stream.
✓ The canonical token model can support future Zamani domains without
  adding duplicate lexical identities.

The final architectural invariant is:

«A Zamani program is written against one stable language vocabulary. Hardware scale, device topology, resource availability, quantum realization, HDL realization, scheduling, routing, resilience, and future computational substrates are resolved downstream of lexical and syntactic analysis rather than being encoded as competing lexical identities.»

That invariant is required for the grammar layer to remain compatible with the broader POCO-REAF architecture.