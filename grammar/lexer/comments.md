Zamani Comment Lexical Specification

File: "grammar/lexer/comments.md"
Status: Normative — Production Specification
Language: Zamani
Layer: Lexer / Lexical Syntax
Rust baseline: Rust 1.97 / Rust 1.97.1
Safety: "unsafe" forbidden
Scalability model: No language-imposed finite resource limits
Authority: This document defines the normative lexical behavior of comments.

---

1. Purpose

This document specifies how Zamani source comments are recognized, delimited, classified, preserved or discarded, and integrated with:

- "grammar/Zamani.g4"
- "grammar/lexer/tokens.md"
- "grammar/lexer/keywords.md"
- "grammar/lexer/operators.md"
- "grammar/lexer/delimiters.md"
- "grammar/lexer/identifiers.md"
- "grammar/lexer/literals.md"
- "grammar/lexer/unicode.md"
- "grammar/lexer/interpolation.md"
- "grammar/lexer/diagnostics.md"
- "grammar/lexer/conformance.md"
- "grammar/specification/lexical.md"
- "grammar/spec/lexical.md"
- "grammar/spec/diagnostics.md"
- "grammar/spec/source-spans.md"
- "grammar/spec/determinism.md"
- "grammar/validation/"
- "grammar/tests/"
- "src/lexer.rs"
- "src/parser.rs"
- "src/frontend/ast/"
- downstream semantic analysis, tooling, compiler, and runtime components.

The objective is to make comments a deterministic, Unicode-safe, scalable part of the lexical specification without allowing comments to interfere with program semantics.

---

2. Scope

This file owns the lexical definition and behavior of:

1. ordinary comments;
2. line comments;
3. block comments;
4. nested block comments;
5. documentation comments;
6. comment termination;
7. comment/source-span behavior;
8. comments adjacent to tokens;
9. comments adjacent to literals;
10. comments inside otherwise whitespace-separated syntax;
11. comment preservation for tooling;
12. comment removal for parsing;
13. comment diagnostics;
14. comment scalability;
15. comment determinism;
16. comment interaction with Unicode;
17. comment interaction with strings and other literals;
18. comment interaction with interpolation;
19. comment interaction with macros and generated source;
20. comment compatibility and versioning.

It does not own:

- identifiers;
- keywords;
- operators;
- delimiters;
- literals;
- expression grammar;
- statement grammar;
- AST architecture;
- semantic meaning of program constructs;
- compiler optimization;
- runtime behavior;
- hardware;
- quantum semantics;
- QEC;
- ZQN;
- HAL;
- scheduling;
- routing;
- resource management.

---

3. Normative Authority

The authority relationship is:

grammar/DESIGN.md
        │
        ▼
grammar/specification/lexical.md
        │
        ▼
grammar/lexer/comments.md
        │
        ├── lexer/tokens.md
        ├── lexer/unicode.md
        ├── lexer/literals.md
        ├── lexer/interpolation.md
        └── lexer/diagnostics.md
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
frontend AST

This document is a normative lexical contract.

"Zamani-Grammar.md" must not silently override this document.

"grammar/grammar.md" may describe the implemented behavior, but does not independently redefine the comment language.

---

4. Design Principles

Zamani comments MUST satisfy all of the following.

4.1 Deterministic

The same source text, language version, dialect configuration, and lexical configuration MUST produce the same comment classification.

4.2 Non-semantic by default

Ordinary comments MUST NOT alter the semantic meaning of a program.

4.3 Source-preserving

The lexer MUST be capable of preserving the original source span of comments for tooling, diagnostics, formatting, documentation extraction, IDE features, and source mapping.

4.4 Unicode-safe

Comments operate over valid Zamani source text according to the Unicode/source-encoding rules defined by "lexer/unicode.md".

4.5 Scalable

There MUST be no language-level maximum for:

- comment length;
- number of comments;
- number of lines inside a comment;
- nesting depth of block comments;
- documentation-comment size;
- number of documentation comments;
- source-file size.

Actual implementations MAY impose resource budgets supplied by the caller or execution environment.

Those are resource policies, not grammar limits.

4.6 Safe Rust

The reference Rust implementation MUST use safe Rust.

No:

unsafe

or unsafe implementation mechanism is permitted.

4.7 No semantic leakage

Comments MUST NOT directly control:

- hardware selection;
- quantum execution;
- QEC;
- ZQN;
- routing;
- scheduling;
- optimization;
- memory allocation;
- distributed placement;
- runtime execution.

Any comment-driven tooling convention must be explicitly defined as tooling metadata and MUST NOT silently become program semantics.

---

5. Comment Classes

Zamani defines the following conceptual comment classes:

Comment
├── LineComment
├── BlockComment
└── DocumentationComment
    ├── LineDocumentationComment
    └── BlockDocumentationComment

The exact token names belong to "lexer/tokens.md".

This document defines behavior, not competing token-enum names.

---

6. Ordinary Line Comments

A line comment begins with the canonical line-comment introducer defined by the Zamani lexical grammar.

The recommended production form is:

//

The line comment extends from the introducer through all subsequent source characters up to, but not including, the next recognized line terminator or end of source.

Conceptually:

// comment text

The newline terminator belongs to the surrounding whitespace/newline lexical model unless the implementation explicitly represents newline as a separate token.

The comment itself MUST NOT consume the newline as comment content.

---

7. Line Comment Termination

A line comment terminates at:

1. LF ("U+000A");
2. CR ("U+000D") when treated as a line terminator;
3. CRLF ("U+000D U+000A");
4. another line-ending representation explicitly accepted by "lexer/unicode.md";
5. end-of-source.

For CRLF, the lexer MUST treat the pair as one logical line ending.

The implementation MUST NOT accidentally produce different lexical behavior for:

\r\n

and the equivalent logical newline.

---

8. End-of-File Line Comments

A line comment MAY terminate at end-of-file without a trailing newline.

Valid:

value = 42 // final comment

The lexer MUST emit the comment representation, if comments are retained, and then EOF.

This MUST NOT produce an unterminated-comment diagnostic.

---

9. Block Comments

A block comment begins with the canonical block-comment opener:

/*

and ends with the corresponding block-comment closer:

*/

Conceptually:

/*
   comment
*/

Block comments may span multiple lines.

---

10. Nested Block Comments

Zamani SHOULD support nested block comments as a production lexical feature.

Example:

/*
    outer comment

    /*
        nested comment
    */

    outer comment continues
*/

The lexical nesting level is:

/*
    depth = 1

    /*
        depth = 2
    */

    depth = 1
*/
depth = 0

A block comment terminates only when its corresponding nesting level returns to zero.

This is preferable to non-nesting block comments because it allows:

- temporary source exclusion;
- generated source;
- documentation examples;
- large-scale refactoring;
- commented-out regions containing existing block comments;
- macro-generated source;
- nested documentation examples.

---

11. Unbounded Comment Nesting

The language MUST NOT specify:

MAX_COMMENT_NESTING = 32

or any equivalent fixed language limit.

The lexical language permits arbitrary finite nesting depth.

For a finite source file:

depth = any finite non-negative value

subject only to available implementation resources.

An implementation MAY receive a configurable lexical resource budget.

For example, an embedding application could specify a maximum nesting depth for denial-of-service protection.

Such a limit MUST be:

- externally configurable;
- clearly reported as a resource/policy limitation;
- independent of language semantics;
- absent from the canonical grammar as a language restriction.

---

12. Unterminated Block Comments

The following is invalid:

/*
    comment

because the block comment does not return to nesting depth zero before EOF.

The lexer MUST report a structured lexical diagnostic.

It MUST NOT:

- silently terminate the comment;
- invent a closing delimiter;
- reinterpret the remaining source as program text;
- panic;
- call "unwrap()" on missing input;
- call "expect()" for malformed source;
- continue with an ambiguous token stream.

---

13. Stray Block Comment Terminators

A closing sequence appearing outside a block comment:

*/

MUST NOT silently become a comment.

It must be tokenized according to the operator/delimiter rules if the individual characters have valid meanings.

If the combination is not valid in that context, the parser or lexer MUST report the appropriate diagnostic.

The lexer MUST NOT invent an implicit opening comment.

---

14. Comments Inside Comments

Within a block comment, comment delimiters participate in nesting.

For example:

/*
    /*
        nested
    */
*/

is valid.

The following:

/*
    /*
        nested
*/

is invalid because the outer block remains open.

---

15. Comment Delimiters Inside Line Comments

Everything after the line-comment introducer and before the line terminator is comment text.

For example:

// /*

does not open a block comment.

Likewise:

// */

does not close anything.

The line-comment lexical state takes precedence until its termination.

---

16. Comment Delimiters Inside Block Comments

Inside a block comment, ordinary program tokens have no independent lexical meaning.

For example:

/*
    "string"
    12345
    fn foo()
    quantum_operation
    /*
       nested
    */
*/

must be treated as comment content except for the block-comment nesting delimiters.

The lexer MUST NOT tokenize the contents as:

- identifiers;
- keywords;
- literals;
- operators;
- quantum operations;
- HDL constructs;
- hardware declarations.

---

17. Comments Inside String Literals

Comment delimiters inside ordinary string literals are string content.

For example:

"this is not // a comment"

and:

"this is not /* a comment */"

must remain string literals.

The string-literal rules are owned by:

grammar/lexer/literals.md

The comment scanner MUST NOT inspect string contents as if they were source-level comments.

---

18. Comments Inside Character Literals

Likewise:

'/'
'*'

are character-literal content.

A character literal containing characters that resemble comment delimiters MUST NOT initiate or terminate a comment.

Character literal validation belongs to "lexer/literals.md".

---

19. Comments Inside Raw Literals

If Zamani supports raw strings or other raw literal forms, comment delimiters inside those literals are data.

For example, a raw literal containing:

/*

MUST NOT initiate a comment.

The raw-literal delimiter and lexical-state rules belong to "lexer/literals.md".

---

20. Comments and Interpolation

If Zamani supports interpolated strings, comment recognition MUST respect the interpolation lexical state.

For example:

"result = ${value // operation}"

must follow the interpolation specification rather than blindly treating every "//" sequence as a comment.

The exact rules for comments inside interpolation expressions are owned jointly by:

lexer/comments.md
lexer/interpolation.md
expressions/

The integration contract is:

literal state
    │
    ├── literal text → comments disabled
    │
    └── interpolation expression → normal lexical rules apply

There MUST be no ambiguity about which lexical state is active.

---

21. Documentation Comments

Documentation comments are comments intentionally exposed to documentation and source-analysis tooling.

They MUST remain lexically distinguishable from ordinary comments.

The canonical documentation syntax MUST be defined by the authoritative Zamani grammar and token specification.

A conventional design is:

///
/// Documentation text
///

for line documentation, and:

/**
 * Documentation text
 */

for block documentation.

If the existing canonical grammar uses a different spelling, that spelling remains authoritative until explicitly versioned.

This document does not authorize a second spelling merely by mentioning the conventional forms.

---

22. Documentation Comments Are Not Program Semantics

Documentation comments MUST NOT automatically change:

- types;
- control flow;
- effects;
- capabilities;
- resource requirements;
- hardware requirements;
- quantum semantics;
- HDL semantics;
- security policies;
- compiler optimization.

For example:

/// requires gpu
fn compute() { ... }

MUST NOT make GPU use semantically mandatory unless a separate, explicitly standardized attribute or declaration mechanism defines that meaning.

Documentation is not a hidden semantic channel.

---

23. Documentation Attachment

Documentation extraction tooling MAY associate a documentation comment with the next eligible declaration.

For example:

/// Computes a result.
fn compute() { ... }

The association algorithm belongs to the documentation/tooling layer.

The lexer provides:

- comment kind;
- source span;
- source text;
- ordering.

The lexer MUST NOT determine declaration ownership.

---

24. Multiple Documentation Comments

Multiple documentation comments MAY occur before a declaration.

Example:

/// First paragraph.
/// Second paragraph.
fn compute() {}

Tooling MAY combine them according to the documentation specification.

The lexer MUST preserve their individual source spans and ordering.

---

25. Documentation and Ordinary Comments

An ordinary comment between documentation comments MAY terminate an attachment sequence according to the documentation-attachment rules.

Example:

/// Documentation

// ordinary implementation note

/// More documentation
fn compute() {}

Whether the second documentation comment remains attached is a tooling/specification question, not a lexical one.

The lexer MUST only classify each comment correctly.

---

26. Whitespace Around Comments

Comments behave lexically as whitespace separators for purposes of token separation unless the specification explicitly says otherwise.

For example:

foo/* comment */bar

must not accidentally become one identifier:

foobar

if the lexical tokenization rules would otherwise require a boundary.

The comment therefore represents a source-region separator.

The parser MUST receive token boundaries consistent with the source.

---

27. Comment Removal Must Preserve Token Boundaries

A lexer or preprocessing implementation MUST NOT simply delete comment bytes and concatenate the surrounding source.

Incorrect conceptual transformation:

foo/* comment */bar

→

foobar

Correct lexical interpretation preserves the fact that a comment occurred between "foo" and "bar".

The implementation may represent the comment as a token or trivia item, but token boundaries must remain correct.

---

28. Comments Between Numeric Components

Comments MUST NOT silently join numeric components.

For example:

1/* comment */23

MUST NOT become:

123

Likewise:

0x/* comment */FF

MUST NOT silently become:

0xFF

unless the language explicitly defines such preprocessing behavior.

Zamani's lexical model should treat comments as source separators, not textual deletion macros.

---

29. Comments Between Operators

Comments likewise MUST preserve operator boundaries.

For example:

a + /* comment */ + b

must remain two plus operators separated by a comment, not become a new operator spelling.

Operator composition belongs to "lexer/operators.md".

---

30. Comments and Whitespace

The lexical model is conceptually:

source
 ├── significant tokens
 ├── whitespace
 └── comments

Whitespace and comments may often be discarded before parsing, but source information MUST remain available to tooling and diagnostics.

The implementation should therefore conceptually distinguish:

Token
Trivia
    ├── Whitespace
    └── Comment

even if the parser-facing API later filters trivia.

---

31. Trivia Preservation

The lexer implementation SHOULD support a mode in which comments and whitespace are retained as trivia.

This is necessary for:

- formatters;
- documentation generation;
- IDEs;
- refactoring;
- source-to-source transformations;
- code navigation;
- syntax highlighting;
- comment-aware macros;
- source maps;
- diagnostics;
- reproducible formatting;
- code review tools.

The parser-facing token stream MAY exclude trivia when the parser does not need it.

The original source span MUST remain recoverable.

---

32. Source Span Requirements

Every retained comment MUST have a source span.

The span MUST identify:

start
end

using the source-position model established by:

grammar/spec/source-spans.md

The implementation MUST NOT use line/column alone as the sole identity of a comment.

Byte offsets are appropriate for source indexing in UTF-8, provided they always identify valid source boundaries.

---

33. Unicode Source Safety

Comments may contain arbitrary valid Unicode scalar values permitted by the source encoding rules.

For example:

// α β γ
// 你好
// مرحبا
// Привет
// 🚀

must be handled without corruption.

Comment contents MUST NOT be normalized merely because they are comments.

The exact source spelling should remain available to tooling.

Unicode lexical policy belongs to:

grammar/lexer/unicode.md

---

34. Invalid UTF-8

If the lexer receives source bytes rather than an already validated Rust "&str", invalid UTF-8 MUST be diagnosed before Unicode-dependent lexical interpretation.

The implementation MUST NOT:

- silently replace invalid bytes;
- silently drop invalid bytes;
- reinterpret invalid bytes using a locale;
- construct invalid Rust "String" values;
- use unsafe UTF-8 conversion.

Rust's safe UTF-8 APIs MUST be used.

---

35. Unicode Line Terminators

The lexer MUST use the line-ending policy defined by "lexer/unicode.md".

The comment implementation MUST NOT accidentally terminate a line comment at arbitrary Unicode characters merely because they visually resemble line separators.

Only standardized line terminators recognized by the lexical specification may terminate a line comment.

---

36. No Locale Dependence

Comment scanning MUST NOT depend on:

- operating-system locale;
- user locale;
- machine language;
- character encoding defaults;
- regional settings.

A Zamani source file has deterministic lexical behavior independent of host locale.

---

37. Comments and Identifiers

Comment markers adjacent to identifiers must obey ordinary token-boundary rules.

Example:

foo//comment
bar

is equivalent, lexically, to:

foo
bar

with the intervening comment represented as trivia.

However:

foo/*comment*/bar

does not permit the implementation to concatenate identifier characters across the comment.

---

38. Comments and Keywords

Keywords are unaffected by comment contents.

For example:

// fn while quantum hardware

contains no keywords from the parser's perspective.

The lexer MUST NOT emit keyword tokens for comment contents.

Keyword recognition is active only in the normal source lexical state.

---

39. Comments and Literals

Comment contents MUST NOT be interpreted as:

- numbers;
- strings;
- characters;
- booleans;
- quantum literals;
- byte literals;
- resource quantities;
- addresses;
- paths;
- timestamps;
- domain literals.

Literal semantics remain owned by "lexer/literals.md".

---

40. Comments and Quantum Syntax

Comment text MUST NOT create quantum semantics.

For example:

// apply H to q

must not produce a quantum operation.

Likewise:

/*
    measure q
*/

must not result in a measurement.

Quantum syntax is active only outside comment lexical state.

Quantum lexical constructs are specified by:

grammar/lexer/quantum-literals.md
grammar/quantum/

---

41. Comments and HDL

Commented HDL source is not HDL.

For example:

/*
module cpu {
    ...
}
*/

must not produce HDL AST nodes.

This prevents comments from accidentally affecting:

- synthesis;
- simulation;
- timing;
- ports;
- signals;
- hardware resources.

---

42. Comments and Hardware Intent

Comments MUST NOT implicitly create hardware requirements.

This:

// requires 64 GB RAM

does not create a resource requirement.

A real requirement must use the resource/capability language defined under:

grammar/resources/
grammar/hardware/

This preserves POCO-REAF.

---

43. Comments and Resource Scalability

Comments themselves must not impose artificial source-size limits.

The following are all valid in the language model, subject only to available resources:

// a very long comment

/*
    arbitrarily many lines
*/

/*
    arbitrarily deep finite nesting
*/

There is no language-level:

MAX_COMMENT_LENGTH
MAX_COMMENT_LINES
MAX_COMMENT_DEPTH
MAX_COMMENTS

---

44. “Infinity” and Comments

The language may be described as scaling from tiny systems to arbitrarily large systems given available resources.

This does not mean a literal source file can contain physically infinite text.

Every concrete source artifact is finite.

Therefore:

unbounded language domain

means:

«no artificial language-defined finite ceiling is imposed.»

It does not mean:

«a compiler must store an infinite comment.»

Large or unbounded logical program structures should use appropriate program constructs rather than relying on infinitely large comments.

---

45. Resource Limits

Implementations MAY expose resource policies such as:

maximum source bytes
maximum comment bytes
maximum nesting depth
maximum diagnostic storage
maximum retained trivia

These MUST be runtime/compiler configuration rather than grammar constants.

For example:

LexicalResourcePolicy

may be supplied by an embedding application.

A resource-policy failure MUST be distinguishable from:

invalid Zamani syntax

---

46. Resource Exhaustion Diagnostics

If an implementation cannot process a comment because an externally supplied resource budget has been exhausted, the diagnostic MUST communicate that fact.

It MUST NOT claim:

«“Comments may not exceed N characters in Zamani.”»

unless N is genuinely part of a versioned language specification.

The distinction is:

language-invalid

versus:

implementation-resource-exhausted

---

47. Linear-Time Requirement

Comment recognition SHOULD be:

O(n)

in the number of source characters examined.

The implementation MUST avoid algorithms that repeatedly rescan increasingly large comment contents.

For nested block comments, maintain lexical nesting state while scanning forward.

Conceptually:

depth += 1

on an opening delimiter and:

depth -= 1

on a closing delimiter.

The source should be traversed once where practical.

---

48. No Quadratic String Construction

The lexer MUST NOT repeatedly append to an immutable string in a way that produces quadratic behavior for large comments.

Prefer:

- source spans;
- slices;
- indexed source regions;
- amortized buffers;
- streaming interfaces where appropriate.

The lexer should not copy a multi-gigabyte comment repeatedly merely to recognize it.

---

49. Streaming Compatibility

The lexical specification SHOULD permit implementation against a source abstraction capable of processing large inputs incrementally.

However, the existing parser architecture may continue to operate over an in-memory source representation where required.

The grammar itself MUST NOT depend on either:

in-memory source

or:

streaming source

as a semantic requirement.

---

50. No Recursion Required for Nested Comments

Nested comments MUST NOT require recursive function calls proportional to nesting depth.

A production implementation should use iterative state:

depth: integer

rather than:

recursive_scan_comment()

for each nested comment.

This avoids call-stack exhaustion for deeply nested finite input.

---

51. Integer Representation of Comment Depth

Comment nesting depth is an implementation detail.

It MUST NOT be hard-coded to a small integer type merely because typical programs use shallow nesting.

The implementation should select an appropriate safe representation and enforce any implementation resource budget externally.

A depth overflow must become a controlled diagnostic/resource error rather than an integer wraparound.

---

52. No Integer Wraparound

The implementation MUST NOT allow nesting depth arithmetic to wrap.

Conceptually:

depth + 1

must either succeed or produce a controlled resource failure.

It MUST NOT silently wrap:

MAX → 0

which could incorrectly terminate a comment.

---

53. Deterministic Lexical State

At every source position, the lexer must have an unambiguous lexical state.

Relevant states include conceptually:

Normal
LineComment
BlockComment(depth)
String
Character
ByteString
ByteCharacter
RawLiteral
Interpolation

The actual implementation state names are implementation details.

The state machine MUST ensure that comment recognition is active only when the current lexical context permits it.

---

54. Priority of Lexical States

When source text resembles multiple constructs, the active lexical state determines interpretation.

For example:

"//"

is a string.

//"

begins a line comment.

Likewise:

/*
    "*/"
*/

contains a quoted sequence inside a comment; quotation marks do not suspend comment processing.

---

55. No Comment Preprocessing Pass

The compiler MUST NOT implement comments as an uncontrolled textual preprocessing pass such as:

source.replace(comment, "")

because that can change lexical token boundaries.

Comment recognition belongs inside the lexical pipeline.

Correct conceptual pipeline:

source
  ↓
lexical state machine
  ↓
tokens + trivia/source spans
  ↓
parser

not:

source
  ↓
blind text deletion
  ↓
lexer

---

56. Parser Integration

"src/parser.rs" MUST receive a token stream whose comment behavior is consistent with this specification.

The parser normally does not need ordinary comments.

Therefore the lexer MAY provide:

significant tokens

while retaining comments separately as trivia.

The parser MUST NOT be responsible for recognizing comment syntax.

---

57. AST Integration

The core AST should not require ordinary comments to represent program semantics.

However, AST nodes MAY retain documentation or source-trivia metadata where required by tooling.

The AST contract is therefore:

source span
+
syntactic construct
+
optional documentation/trivia metadata

rather than:

AST node whose semantics depend on ordinary comments

This is consistent with the domain-neutral AST architecture.

---

58. Semantic Analysis Integration

Semantic analysis MUST ignore ordinary comments.

Documentation comments may be consumed by a separate documentation-analysis subsystem.

The semantic analyzer MUST NOT infer program requirements from arbitrary prose.

For example:

// use exactly 8 CPUs

must not create:

requires cpus = 8

---

59. IR Integration

Ordinary comments MUST NOT produce canonical semantic IR operations.

The expected flow is:

comment
   ↓
lexer trivia
   ↓
(optional tooling/documentation metadata)
   ↓
no semantic IR

A comment MUST NOT generate:

- classical IR operation;
- "quantum::ir" operation;
- HDL operation;
- hardware operation;
- scheduling operation;
- resource allocation;
- ZQN event.

---

60. Quantum IR Integration

"quantum::ir" remains the canonical quantum semantic boundary.

Comments do not become quantum IR.

For example:

// apply H

does not create a quantum operation.

Actual quantum syntax must pass through the normal:

lexer
→ parser
→ AST
→ semantic quantum model
→ quantum::ir

pipeline.

---

61. Compiler Integration

The compiler MAY retain comments for:

- debug information;
- documentation;
- generated-source mapping;
- diagnostics;
- source maps;
- provenance;
- formatter integration.

The compiler MUST NOT require comments to compile an otherwise valid program.

Removing ordinary comments from a valid program must preserve program semantics.

---

62. Runtime Integration

Runtime behavior MUST NOT depend on ordinary source comments.

The runtime may receive metadata generated from documentation or tooling, but that is separate from ordinary lexical comments.

---

63. Tooling Integration

Comment spans and text SHOULD be available to:

- language servers;
- IDEs;
- formatters;
- documentation generators;
- syntax highlighters;
- refactoring tools;
- static analyzers;
- source indexers;
- code navigation;
- provenance systems;
- compiler diagnostics.

The tooling layer should not need to reimplement comment parsing.

---

64. Formatting Integration

A formatter must be able to distinguish:

// comment

from:

/* comment */

and preserve their source positions sufficiently to make formatting decisions.

A formatter MAY normalize whitespace inside comments only when explicitly configured.

Default formatting should preserve comment text rather than unexpectedly rewriting documentation.

---

65. Macro Integration

Macros MUST receive clearly defined comment/trivia behavior.

A macro system MUST NOT accidentally treat ordinary comments as syntax tokens unless its macro contract explicitly requests trivia.

For hygienic syntax-tree macros:

comment

is normally trivia.

For token-stream macros, trivia MAY be exposed when requested.

Macro expansion MUST NOT introduce lexical ambiguity through comment removal or insertion.

---

66. Generated Source

Generated Zamani source MUST obey the same comment lexical rules as handwritten source.

A generator MUST NOT depend on implementation-specific comment syntax.

Generated code should use the language version and dialect rules active for the generated source.

---

67. Comments and Dialects

A dialect MAY define additional documentation or comment conventions only through the dialect mechanism.

A dialect MUST NOT silently redefine the meaning of core comment delimiters.

Core comments remain stable across dialects unless an explicitly versioned language extension changes them.

---

68. Comments and Interoperability

Imported source formats may have different comment systems.

For example:

- OpenQASM;
- HDL formats;
- C/C++;
- Rust;
- Python;
- other DSLs.

Their source comments MUST be translated according to the interoperability frontend.

They MUST NOT cause Zamani's core lexer to acquire vendor-specific comment syntax accidentally.

---

69. OpenQASM Integration

When OpenQASM source is imported, OpenQASM comments belong to the OpenQASM frontend's lexical rules.

After translation into Zamani syntax/AST, the resulting representation must obey Zamani's own lexical and source-span contracts.

The canonical quantum semantic boundary remains:

quantum::ir

---

70. HDL Integration

HDL-specific imported comments must remain within the HDL frontend boundary.

They must not become universal Zamani comments merely because the imported source contains them.

The HDL frontend may preserve them as source metadata.

---

71. Documentation Markup

Documentation comments may contain Markdown or another documented markup language.

The lexer treats the contents as comment text.

It MUST NOT parse Markdown syntax.

For example:

/// # Heading
/// `code`

The lexer classifies the entire region as documentation comment content.

Markdown parsing belongs to documentation tooling.

---

72. Code Examples Inside Documentation

Documentation comments may contain source examples including comment delimiters.

Documentation extraction must not confuse embedded examples with actual source-level comments because the lexical comment has already been delimited.

For example:

/**
Example:

    // this is example source

*/

The "//" inside the block documentation comment is documentation text.

---

73. Comments and Source Encoding

The canonical source encoding SHOULD be UTF-8.

The lexer MUST treat source encoding deterministically.

Comments MUST preserve the original valid Unicode scalar sequence for tooling.

No platform-specific code-page interpretation is permitted.

---

74. Comment Text Preservation

When comments are retained, implementations SHOULD preserve:

- exact source spelling;
- exact delimiter spelling;
- whitespace;
- line endings;
- Unicode characters;
- source span.

A normalized documentation representation MAY be created downstream.

The original representation must remain available where source fidelity is required.

---

75. Comment Hashing and Provenance

Tooling MAY hash comment source regions for:

- incremental compilation;
- caching;
- provenance;
- source indexing.

Such hashing is outside the grammar.

Any hash must operate over a deterministic byte representation.

Comments must not introduce nondeterministic metadata into compilation.

---

76. Incremental Compilation

An incremental lexer SHOULD be able to identify affected comment regions without rescanning unrelated source where the implementation architecture permits.

Changing:

// documentation

should not require semantic recompilation of an unrelated program region unless tooling policy requires it.

The language specification does not mandate a particular incremental strategy.

---

77. Determinism Requirements

For a fixed:

source bytes
language version
dialect configuration
lexical configuration

comment classification MUST be deterministic.

The result MUST NOT depend on:

- locale;
- operating system;
- CPU architecture;
- machine word size;
- available GPU;
- QPU;
- thread count;
- timing;
- hash-map iteration order;
- nondeterministic parallel execution.

---

78. Error Recovery

For malformed comments, the lexer MAY recover for IDE/editor use.

However, recovery MUST be explicitly marked as recovery.

For compiler-conformance mode:

unterminated block comment

must produce a lexical error.

Recovery must never silently convert malformed source into valid production semantics.

---

79. Diagnostic Requirements

Comment diagnostics MUST provide, where available:

- stable diagnostic identifier;
- severity;
- source span;
- concise message;
- relevant source context;
- language-version information when applicable;
- recovery information when recovery occurred.

The exact diagnostic identifier registry belongs to:

grammar/lexer/diagnostics.md
grammar/spec/diagnostics.md

This document defines conditions, not a competing diagnostic-number registry.

---

80. Required Comment Diagnostics

At minimum, the diagnostic system must be able to represent:

C001 — Unterminated block comment

A block comment reaches EOF before its nesting depth returns to zero.

C002 — Resource limit exceeded

The implementation cannot process the comment within an externally supplied resource budget.

C003 — Invalid comment state

An internal lexer-state invariant was violated.

"C003" is an implementation failure and MUST NOT be produced merely because ordinary user source is malformed.

The actual stable IDs may be assigned centrally by "lexer/diagnostics.md".

---

81. Diagnostics Must Not Panic

Malformed source is expected input to a compiler.

The lexer MUST NOT use:

unwrap()
expect()
unreachable!()
panic!()

as the normal response to malformed comments.

Errors must be represented through the project's structured error mechanism.

---

82. Rust Implementation Contract

The Rust implementation in "src/lexer.rs" MUST be compatible with:

Rust 1.97
Rust 1.97.1

and use safe Rust only.

The implementation MUST:

- operate on valid UTF-8 source safely;
- avoid unchecked indexing;
- avoid unsafe pointer manipulation;
- avoid unsafe UTF-8 construction;
- avoid undefined behavior;
- return structured lexical errors;
- preserve source spans;
- avoid integer overflow;
- avoid recursion proportional to comment nesting;
- avoid quadratic scanning;
- remain deterministic.

---

83. Suggested Internal Model

The implementation may conceptually use:

Trivia
├── Whitespace
├── LineComment
├── BlockComment
├── LineDocumentationComment
└── BlockDocumentationComment

with metadata such as:

kind
span
raw_text

The exact Rust structs remain an implementation decision.

They must conform to the public lexer/token/source-span contracts.

---

84. Borrowing and Ownership

Where practical, comment text SHOULD be represented as a source slice rather than copied.

For example, conceptually:

SourceSpan

can identify the original region.

If ownership is required by the existing lexer API, copying is permitted.

Correctness and source preservation take priority over premature allocation optimization.

---

85. No Unsafe Lifetime Tricks

The implementation MUST NOT use unsafe mechanisms to manufacture long-lived references to source text.

Safe Rust borrowing and owned representations are sufficient.

---

86. Comments Must Not Affect Hardware Scalability

The lexical comment system must not contain assumptions such as:

CPU_COUNT = 8
GPU_COUNT = 1
QUBIT_COUNT = 32
NODE_COUNT = 16

or any other target-specific capacity.

Comments are independent of target hardware.

---

87. POCO-REAF Compatibility

Comments are compatible with:

«Program Once, Compile Once, Run Everywhere, Anywhere, Forever.»

The same source program with ordinary comments removed must have the same program semantics.

Therefore:

source + comments

and:

source without ordinary comments

must compile to semantically equivalent programs, subject to documentation/tooling metadata.

---

88. Comment-Driven Build Directives

Build directives embedded in comments are NOT part of the core comment semantics unless explicitly standardized.

The compiler MUST NOT interpret arbitrary comments as hidden directives.

If Zamani requires build attributes, they should use explicit syntax under:

grammar/compile/
grammar/resources/
grammar/hardware/

rather than comment conventions.

---

89. Security

Comment processing must be resistant to resource-exhaustion attacks.

Threats include:

- extremely large comments;
- deeply nested comments;
- pathological source files;
- huge documentation blocks;
- enormous Unicode sequences;
- repeated delimiter patterns.

Mitigations include:

- linear scanning;
- configurable resource budgets;
- iterative nesting;
- checked arithmetic;
- bounded diagnostic storage;
- no recursive depth proportional to source;
- no catastrophic regular expressions;
- no unsafe code.

---

90. Comment Bomb Resistance

A compiler service MAY set a lexical budget.

For example:

LexicalResourcePolicy {
    ...
}

The policy is environment-specific.

The grammar itself remains unbounded over finite source.

This distinction is required for scalable deployment across:

- embedded devices;
- desktops;
- servers;
- clusters;
- cloud systems;
- HPC systems;
- quantum-control environments.

---

91. Small-System Compatibility

A minimal Zamani implementation must be able to process small source files without requiring large fixed allocations for comments.

The implementation should not preallocate memory proportional to an arbitrary maximum comment size.

---

92. Large-System Compatibility

Large source repositories must be supported without artificial language limits.

A compiler may use:

- streaming;
- chunked storage;
- memory mapping;
- incremental lexing;
- external source storage;
- lazy trivia loading.

These are implementation strategies and not language semantics.

---

93. Comment Count

There is no language-defined maximum number of comments.

A source unit may contain:

0

or any finite number of comments.

Resource exhaustion remains an implementation/environment concern.

---

94. Comment Length

There is no language-defined maximum comment length.

A finite comment is valid if its syntax is valid and the implementation has sufficient resources to process it.

---

95. Empty Comments

Empty comments are valid where the delimiters themselves form a valid comment.

For example:

//
//

and:

/**/

where the latter is a valid empty block comment.

Nested empty comments are also valid where nesting syntax permits them.

---

96. Whitespace-Only Comments

Comments containing only whitespace are valid.

Example:

/*
    
*/

Documentation tooling may choose whether such documentation is meaningful.

The lexer still classifies it correctly.

---

97. Comment Delimiters Adjacent to EOF

A line comment may terminate at EOF.

A block comment requires its closing delimiter.

Therefore:

// comment

is valid at EOF, while:

/* comment

is invalid.

---

98. Comment Delimiters Adjacent to Tokens

These are all lexically meaningful:

foo/*x*/bar
foo//x
bar

The lexer must preserve the appropriate token boundaries.

Whitespace does not need to be physically present around a comment for it to function as a lexical separator.

---

99. Comment Delimiters in Numeric Syntax

A comment cannot be used to splice a numeric literal.

For example:

12/*comment*/34

is not:

1234

It is two numeric token regions separated by trivia.

This protects exact numeric-literal semantics.

---

100. Comment Delimiters in Operators

A comment cannot manufacture a longer operator by textual concatenation.

For example:

<
/* comment */
=

does not automatically become:

<=

unless the actual token stream contains the operator according to "operators.md".

This rule prevents comment stripping from changing semantics.

---

101. Comments and Macro Expansion

Comment syntax applies to source before macro expansion.

Generated source is then lexed under the same rules.

A macro MUST NOT depend on comments being silently concatenated or deleted.

---

102. Comments and Metaprogramming

Quoted source/code representations may contain comment text as data.

Metaprogramming must distinguish:

comment in generated/source syntax

from:

comment represented as data

The lexical rules of the resulting source apply only when that source is actually parsed as Zamani.

---

103. Comments and Source-to-Source Translation

A translator MAY preserve comments.

If it does not preserve them, it MUST preserve program semantics.

Source comments must not be required to reconstruct semantic program behavior.

---

104. Comments and Reproducible Builds

Ordinary comments should not alter executable semantics.

Therefore a build system may choose whether comments participate in semantic compilation hashes.

If comments are included in artifact/provenance hashes, this must be an explicitly documented policy.

---

105. Comments and Incremental Cache Keys

The compiler MAY distinguish:

semantic source changes

from:

comment-only changes

to improve incremental compilation.

The grammar does not require a particular cache strategy.

---

106. Comments and Source Maps

Source spans for comments should be retained sufficiently for tooling.

Compiler-generated code may retain mappings to source comments when documentation/debug tooling requires them.

Ordinary comments need not map to executable instructions.

---

107. Comments and Diagnostics

Compiler diagnostics may point to nearby comments for context, but a comment must not become the primary semantic source of an error unless it is malformed lexical syntax.

---

108. Comments and Versioning

The comment syntax is part of the Zamani lexical compatibility surface.

A future language version MUST NOT silently reinterpret valid existing comments.

If comment syntax changes, the change must be:

- versioned;
- documented;
- tested;
- migration-aware;
- reflected in compatibility metadata.

---

109. Adding New Comment Syntax

A new comment form may be introduced only after completing:

lexer/tokens.md
lexer/comments.md
lexer/diagnostics.md
lexer/conformance.md
specification/lexical.md
spec/lexical.md
Zamani.g4
src/lexer.rs
tests
compatibility

The new syntax must be checked for collisions with:

- operators;
- delimiters;
- literals;
- identifiers;
- macros;
- interpolation;
- dialect syntax.

---

110. Reserved Future Comment Forms

If additional comment forms are desired in the future, they should remain explicitly reserved until standardized.

The lexer MUST NOT accidentally accept them as comments merely because a prefix resembles an existing delimiter.

---

111. Canonical Lexical Pseudogrammar

The conceptual lexical model is:

comment
    ::= line-comment
     | block-comment
     | documentation-comment
     ;

line-comment
    ::= line-comment-introducer line-comment-char*
        (line-terminator | end-of-source)
     ;

block-comment
    ::= block-comment-opener block-comment-content block-comment-closer
     ;

block-comment-content
    ::= comment-text
     | nested-block-comment
     ;

nested-block-comment
    ::= block-comment
     ;

documentation-comment
    ::= line-documentation-comment
     | block-documentation-comment
     ;

This is a semantic specification model.

The exact ANTLR lexer productions must be implemented consistently with the canonical lexical contract.

---

112. ANTLR Integration

"grammar/Zamani.g4" remains the composition/root grammar.

Comment lexer rules must be integrated into the canonical lexical grammar rather than creating a competing ANTLR grammar.

The implementation must ensure that:

ANTLR lexical behavior

and:

src/lexer.rs behavior

recognize the same language.

Where nested comments require state that cannot be represented cleanly as a single regular lexer rule, the implementation strategy may use a deterministic lexer state mechanism.

The resulting language must remain identical.

---

113. Rust Lexer Conformance

"src/lexer.rs" is an implementation of the lexical contract.

It must not silently diverge from:

grammar/lexer/comments.md

If a discrepancy is discovered, it must be resolved through the compatibility/specification process rather than silently changing one side.

---

114. Token Integration

"lexer/tokens.md" owns the exact public token taxonomy.

It should define whether the implementation exposes:

LineComment
BlockComment
DocumentationComment

as explicit tokens or stores them as trivia.

This document requires that their distinction remain recoverable for tooling.

---

115. Keyword Integration

Comments are processed before keyword recognition can apply to their contents.

Therefore:

// fn

must not emit "fn" as a keyword token.

Keyword definitions remain in:

lexer/keywords.md

---

116. Operator Integration

Comment delimiters have lexical precedence over ordinary operators when they form valid comment introducers.

After entering comment state, operator recognition is suspended until comment termination.

Operator definitions remain in:

lexer/operators.md

---

117. Literal Integration

Literal lexical states take precedence over comment recognition where a comment-like sequence occurs inside a literal.

This prevents:

"//"
"/* */"

from being misclassified.

Literal rules remain in:

lexer/literals.md

---

118. Unicode Integration

The comment scanner uses the source encoding and line-ending rules established in:

lexer/unicode.md

It must preserve valid Unicode source content.

---

119. Diagnostic Integration

Comment-specific errors are registered centrally through:

lexer/diagnostics.md
spec/diagnostics.md

No duplicate diagnostic numbering system should be created here.

---

120. Conformance Integration

"lexer/conformance.md" must verify that the implementation satisfies every normative comment rule in this document.

Conformance must cover:

- valid comments;
- invalid comments;
- nested comments;
- Unicode;
- EOF;
- token boundaries;
- literals;
- interpolation;
- documentation;
- source spans;
- resource policies;
- determinism.

---

121. Positive Test Requirements

At minimum, tests must include:

// comment

/* comment */

/** documentation */

// comment at EOF

/* nested /* comment */ comment */

foo/*comment*/bar

"// not a comment"

"/* not a comment */"

/*
    Unicode: α β γ
*/

/*
    中文
    日本語
    العربية
    🚀
*/

---

122. Negative Test Requirements

Tests must reject:

/* unterminated

and:

/*
    /*
        nested
*/

when the outer comment remains unclosed.

Tests must also verify that malformed lexical states cannot silently recover into valid semantic programs.

---

123. Boundary Tests

Boundary tests must include:

1. empty line comment;
2. empty block comment;
3. comment at byte zero;
4. comment at EOF;
5. comment immediately before EOF;
6. comment between two identifiers;
7. comment between two numbers;
8. comment between operators;
9. comment adjacent to delimiters;
10. deeply nested finite comments;
11. Unicode at comment boundaries;
12. CRLF;
13. LF;
14. CR;
15. very long finite comment;
16. large documentation comment.

---

124. Scalability Tests

The test suite must verify that the implementation does not contain language-level assumptions about:

comment length
comment count
line count
nesting depth
source size
Unicode content

Tests should progressively exercise larger finite inputs until constrained by the test environment.

A test must never assert:

maximum supported comment size = N

as a language rule.

---

125. Stress Testing

Stress tests should cover:

many small comments

one very large comment

deeply nested comments

alternating delimiters

large Unicode comments

comments between every token

The objective is to expose:

- quadratic behavior;
- stack exhaustion;
- integer overflow;
- memory amplification;
- incorrect lexical-state transitions.

---

126. Determinism Tests

The same input must produce identical:

- comment classification;
- source spans;
- token boundaries;
- diagnostics.

Run determinism tests across supported platforms where practical.

---

127. Property-Based Testing

The lexical implementation should be suitable for property-based testing.

Useful properties include:

Property 1

Adding an ordinary comment at a legal trivia position does not change semantic tokenization except for trivia.

Property 2

Removing an ordinary comment while preserving token boundaries does not change semantic tokenization.

Property 3

A closed nested block comment always returns the lexer to the lexical state active before it.

Property 4

An unterminated block comment never produces tokens from its supposed contents.

Property 5

Comment processing is deterministic.

---

128. Fuzzing

The lexer should be fuzz-tested with:

- random Unicode;
- random delimiters;
- deeply nested delimiters;
- malformed delimiters;
- large comments;
- comment/literal mixtures;
- comment/operator mixtures;
- comment/identifier mixtures.

The lexer must never exhibit undefined behavior or panic on malformed input.

---

129. Security Tests

Security testing must include:

deep nesting
large comment
large documentation comment
delimiter storms
Unicode storms
CR/LF storms
comment/token alternation

The implementation must fail gracefully when externally imposed resource budgets are exhausted.

---

130. Hard-Coding Audit

This file and its implementation MUST contain no universal hardware limits.

Forbidden examples include:

MAX_COMMENT_SIZE = 1024
MAX_COMMENTS = 10000
MAX_COMMENT_DEPTH = 32

when presented as language semantics.

A configurable resource-policy value is acceptable.

Likewise forbidden are unrelated machine assumptions such as:

MAX_CPUS
MAX_GPUS
MAX_QUBITS
MAX_NODES
MAX_MEMORY

---

131. Performance Requirements

Expected comment processing complexity:

Time: O(n)

where "n" is the amount of source examined.

Additional memory should be approximately:

O(depth)

for nested-comment state if comment text is represented by source spans, plus whatever memory the surrounding source/token infrastructure requires.

Implementations should avoid copying comment contents unnecessarily.

---

132. Error Handling Requirements

All malformed source input must result in structured error handling.

The lexer must never rely on:

panic
unwrap
expect
unchecked arithmetic
unsafe memory operations

for normal malformed-comment handling.

---

133. Compatibility Matrix

The completed implementation must maintain this conceptual compatibility:

Layer| Requirement
"comments.md"| Normative comment contract
"specification/lexical.md"| References/contains lexical authority
"spec/lexical.md"| Formal lexical contract
"Zamani.g4"| Canonical grammar composition
"src/lexer.rs"| Safe implementation
"src/parser.rs"| Consumes conforming tokens
AST| Optional source/documentation metadata
Semantic layer| Ignores ordinary comments
IR| No ordinary-comment operations
Compiler| May preserve tooling metadata
Runtime| No ordinary-comment semantics
Tooling| May consume retained comments
Tests| Full lexical conformance

---

134. Relationship to "grammar/grammar.md"

"grammar/grammar.md" should report whether the comment implementation is:

SPECIFIED
IMPLEMENTED
PARTIAL
PLANNED
DEPRECATED

It must not introduce a second comment grammar.

If the implementation differs from this document, the discrepancy must be explicitly reported.

---

135. Relationship to "Zamani-Grammar.md"

"Zamani-Grammar.md" may contain historical or proposed comment syntax.

Such syntax is not automatically legal Zamani.

The promotion path is:

proposal
↓
lexical specification
↓
grammar
↓
lexer
↓
tests
↓
compatibility
↓
stable

---

136. Relationship to "DESIGN.md"

"DESIGN.md" governs:

- authority;
- architecture;
- portability;
- determinism;
- hard-coding policy;
- integration boundaries.

This file specializes those principles for comments.

No rule here may violate "DESIGN.md".

---

137. Relationship to "specification/lexical.md"

"specification/lexical.md" should summarize the comment model and link/reference this document for detailed behavior.

It should not create conflicting comment syntax.

---

138. Relationship to "lexer/tokens.md"

"tokens.md" owns:

- exact token names;
- token representation;
- trivia representation;
- public lexer API conventions.

This file owns:

- what constitutes a comment;
- when comments begin/end;
- nesting;
- lexical states;
- comment semantics.

---

139. Relationship to "lexer/literals.md"

"literals.md" owns literal lexical states.

The key integration invariant is:

comment-like text inside literal ≠ comment

and:

literal-like text inside comment ≠ literal

---

140. Relationship to "lexer/operators.md"

Comment delimiters must take precedence when they form valid comment introducers.

Operator recognition resumes after comment termination.

No comment stripping pass may manufacture operators.

---

141. Relationship to "lexer/identifiers.md"

Comments create lexical separation but do not become identifier characters.

Identifier scanning MUST stop before a comment introducer.

---

142. Relationship to "lexer/interpolation.md"

Interpolation defines transitions between literal text and expression lexical state.

Comments are recognized only in the appropriate state.

The two specifications must be tested together.

---

143. Relationship to "grammar/expressions/"

Expression grammar does not parse comments.

Comments are lexical trivia.

Expressions receive significant tokens after comment handling.

---

144. Relationship to "grammar/statements/"

Statements do not own comment syntax.

A comment may occur wherever lexical trivia is permitted.

Statement boundaries must never depend on ordinary comment contents.

---

145. Relationship to "grammar/macros/"

Macros must explicitly specify whether they see trivia.

They must not accidentally depend on comment deletion.

---

146. Relationship to "grammar/metaprogramming/"

Quoted/generated code is data until parsed.

Once parsed as Zamani source, normal comment rules apply.

---

147. Relationship to "grammar/dialects/"

Dialects cannot silently reinterpret core comments.

Dialect-specific documentation syntax must be explicitly registered and versioned.

---

148. Relationship to Documentation

Documentation tooling consumes documentation comments after lexical classification.

The lexer does not:

- render Markdown;
- resolve links;
- validate documentation semantics;
- associate comments with declarations.

Those responsibilities belong to tooling/documentation layers.

---

149. Relationship to Security

Comments cannot silently weaken or strengthen security semantics.

For example:

// trusted

does not establish trust.

// no authentication required

does not disable authentication.

Security semantics belong to:

grammar/security/

and downstream semantic/security systems.

---

150. Relationship to Provenance

Comments may contain human-readable provenance information.

Formal provenance MUST use the provenance mechanisms defined by the language/toolchain.

Ordinary comments are not authoritative provenance records.

---

151. Relationship to Reproducibility

Comment preservation must not make compilation nondeterministic.

A build system may intentionally include comments in source hashes, but the policy must be deterministic and documented.

---

152. Relationship to POCO-REAF

The comment system supports target independence because comments contain no mandatory target realization.

A source file can move between:

tiny embedded system
CPU
GPU
FPGA
ASIC
QPU
HPC cluster
distributed system
cloud
future target

without requiring a different comment language.

---

153. Production Acceptance Checklist

This file is complete only when all of the following are true.

Specification

- [ ] Comment classes defined.
- [ ] Line comments defined.
- [ ] Block comments defined.
- [ ] Nested comments defined.
- [ ] Documentation comments integrated.
- [ ] EOF behavior defined.
- [ ] Unterminated behavior defined.
- [ ] Unicode behavior defined.
- [ ] Literal interaction defined.
- [ ] Interpolation interaction defined.
- [ ] Token-boundary behavior defined.
- [ ] Source-span behavior defined.
- [ ] Resource policy defined.
- [ ] Determinism defined.
- [ ] Security requirements defined.

Grammar

- [ ] Canonical "Zamani.g4" integration defined.
- [ ] No competing root grammar.
- [ ] No hidden comment preprocessing.
- [ ] Lexer/parser boundary defined.

Rust

- [ ] Rust 1.97/1.97.1 compatible.
- [ ] No "unsafe".
- [ ] No unchecked user-input assumptions.
- [ ] No panic-based malformed-input handling.
- [ ] No integer-wraparound depth logic.
- [ ] No recursive nesting implementation requirement.
- [ ] Linear scanning achieved.

AST / semantics

- [ ] Ordinary comments do not affect semantics.
- [ ] Documentation metadata has an explicit boundary.
- [ ] Source spans are preserved.
- [ ] No comment-specific semantic IR.
- [ ] No quantum IR duplication.

Scalability

- [ ] No comment length limit in grammar.
- [ ] No comment count limit in grammar.
- [ ] No nesting limit in grammar.
- [ ] No source-size limit in grammar.
- [ ] Resource budgets are external/configurable.
- [ ] Large comments are processed without quadratic behavior.

Testing

- [ ] Positive tests.
- [ ] Negative tests.
- [ ] EOF tests.
- [ ] Nested-comment tests.
- [ ] Unicode tests.
- [ ] Literal-interaction tests.
- [ ] Interpolation tests.
- [ ] token-boundary tests.
- [ ] source-span tests.
- [ ] determinism tests.
- [ ] scalability tests.
- [ ] fuzz tests.
- [ ] security/resource tests.
- [ ] compatibility tests.

---

154. Completion Criteria

"grammar/lexer/comments.md" is considered production complete only when:

Normative specification
        +
Canonical grammar integration
        +
Rust lexer implementation
        +
Token/trivia contract
        +
Source-span contract
        +
Diagnostic contract
        +
AST/tooling integration
        +
Compatibility rules
        +
Positive tests
        +
Negative tests
        +
Boundary tests
        +
Scalability tests
        +
Determinism tests
        +
Security tests
        +
Hard-coding audit

all agree.

No later grammar file should need to redefine how comments work.

A later domain file such as:

quantum/
hdl/
ai/
distributed/
hardware/
networking/
security/

must automatically inherit the same comment lexical behavior.

---

155. Final Invariant

The fundamental invariant is:

Comments are lexical source information,
not hidden program semantics.

Therefore:

source
  │
  ├── comments ───────────────► tooling/documentation
  │
  └── significant tokens
                │
                ▼
              parser
                │
                ▼
               AST
                │
                ▼
        semantic analysis
                │
                ▼
        canonical semantic IR
                │
        ┌───────┼────────┐
        ▼       ▼        ▼
    classical quantum   HDL
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

Comments never become an accidental second semantic channel.

They never establish hardware limits.

They never establish quantum limits.

They never establish resource limits.

They never bypass semantic analysis.

They never bypass the canonical "quantum::ir" boundary.

They never create a second language.

They never make POCO-REAF target-dependent.

The production Zamani lexer therefore treats comments as deterministic, source-preserving, Unicode-safe, scalable lexical trivia with explicit documentation/tooling integration and no artificial language limits.