Zamani Lexical Specification

File: "grammar/specification/lexical.md"
Status: Normative
Specification Layer: Language Foundation
Scope: Lexical structure of the Zamani programming language
Implementation Baseline: Rust 1.97 / Rust 1.97.1
Rust Safety Requirement: Zamani-owned Rust implementation MUST NOT use "unsafe"
Primary Grammar Composition Root: "grammar/Zamani.g4"
Implementation Conformance Reference: "grammar/grammar.md"
Lexical Contracts: "grammar/lexer/"
Frontend Lexer: "src/lexer.rs"

---

1. Purpose

This document defines the normative lexical contract for the Zamani programming language.

It specifies how a Zamani source character stream becomes a deterministic sequence of lexical tokens consumed by the parser.

This document defines:

- source encoding;
- Unicode handling;
- characters and code points;
- whitespace;
- line terminators;
- comments;
- identifiers;
- keywords;
- contextual/reserved words;
- literals;
- numeric literals;
- character literals;
- string literals;
- interpolation;
- operators;
- delimiters;
- punctuation;
- annotations and attributes;
- quantum notation;
- hardware/resource notation;
- lexical errors;
- source locations;
- token identity;
- token text preservation;
- escape handling;
- lexical normalization;
- ambiguity rules;
- compatibility;
- scalability;
- determinism;
- security requirements;
- lexer-to-parser integration.

This document does not define the complete Zamani parser grammar.

The lexical layer answers:

«What token is this source text?»

The syntactic layer answers:

«How may those tokens be composed?»

The semantic layer answers:

«What does that composition mean?»

---

2. Normative Language

The following terms are normative:

- MUST — mandatory.
- MUST NOT — prohibited.
- SHOULD — recommended unless a documented reason exists otherwise.
- SHOULD NOT — discouraged unless a documented reason exists otherwise.
- MAY — permitted but optional.
- IMPLEMENTATION LIMIT — a practical limit imposed by an implementation, not by the Zamani language.
- LANGUAGE LIMIT — a limit imposed by the language specification.

Unless explicitly stated otherwise, Zamani MUST NOT impose artificial implementation limits as language rules.

---

3. File Contract

3.1 Purpose

Provide one authoritative lexical specification that all lexer, parser, grammar, AST, semantic-analysis, tooling, testing, and compatibility work can consume.

3.2 Owns

This file owns:

- lexical categories;
- lexical precedence;
- tokenization rules;
- Unicode lexical rules;
- identifier rules;
- keyword classification;
- literal syntax;
- escape syntax;
- comment syntax;
- operator spelling;
- delimiter spelling;
- token boundaries;
- lexical error categories;
- source-span requirements;
- lexical compatibility rules.

3.3 Does Not Own

This file does not own:

- AST node definitions;
- semantic types;
- evaluation rules;
- quantum IR;
- classical IR;
- HDL IR;
- hardware topology;
- physical device selection;
- routing;
- scheduling;
- QEC;
- ZQN;
- HAL;
- compiler optimization;
- runtime implementation;
- vendor APIs;
- CPU/GPU/QPU/FPGA limits;
- application-specific libraries.

3.4 Inputs

The lexer receives:

source bytes
        ↓
Unicode decoding
        ↓
source characters/code points

3.5 Outputs

The lexer produces a deterministic token stream:

Token {
    kind,
    lexeme/span,
    source_location,
    normalized_form_if_applicable
}

The exact Rust representation is owned by "src/lexer.rs".

3.6 Upstream Contracts

This specification depends on:

- source encoding rules;
- Unicode rules;
- repository version policy.

3.7 Downstream Consumers

The lexical contract is consumed by:

- "grammar/Zamani.g4";
- "grammar/lexer/*";
- "grammar/specification/syntax.md";
- "grammar/spec/syntax.md";
- "src/lexer.rs";
- "src/parser.rs";
- frontend AST construction;
- diagnostics;
- syntax highlighting;
- formatter;
- language server tooling;
- macro/token processing;
- compatibility validation;
- lexical tests.

3.8 AST Contract

Lexical tokens MUST preserve enough information for the parser and AST builder to preserve:

- source locations;
- identifiers;
- literal values;
- literal source representation when required for diagnostics/provenance;
- attributes;
- annotations;
- operator identity.

Lexical normalization MUST NOT erase semantic information needed downstream.

3.9 Semantic Contract

The lexer MUST NOT assign semantic meaning that belongs to semantic analysis.

For example, the lexer recognizes:

qubit

as a lexical token where appropriate.

It does not determine:

- how many qubits exist;
- whether a qubit is logical or physical;
- which device owns a qubit;
- how a quantum operation is implemented.

3.10 IR Integration

No lexical token directly defines a canonical IR operation.

Lexical information flows through:

source
 ↓
lexer
 ↓
parser
 ↓
AST
 ↓
semantic model
 ↓
canonical IR

Quantum constructs eventually lower toward "quantum::ir", not to a lexer-specific quantum representation.

3.11 Compiler Integration

The compiler MUST consume semantic information rather than interpreting raw token spelling as hardware policy.

3.12 Runtime Integration

The runtime MUST NOT depend on source-token spelling.

Runtime behavior comes from compiled semantic artifacts.

3.13 Tooling Integration

The lexical contract MUST support:

- syntax highlighting;
- formatting;
- diagnostics;
- IDE/LSP tooling;
- refactoring;
- source mapping;
- macro tooling;
- documentation tooling.

3.14 Cross-Domain Integration

The lexical layer is shared by:

- classical;
- quantum;
- hybrid;
- HDL;
- hardware;
- AI/ML;
- data;
- distributed;
- networking;
- security;
- scientific;
- embedded;
- accelerator;
- future domains.

No domain may silently redefine the global meaning of a lexical token.

3.15 Tests

Every lexical category MUST have:

- positive tests;
- negative tests;
- boundary tests;
- Unicode tests where applicable;
- scalability tests;
- determinism tests;
- compatibility tests.

3.16 Hard-Coding Audit

The lexical implementation MUST NOT contain artificial language limits such as:

MAX_IDENTIFIER_LENGTH
MAX_STRING_LENGTH
MAX_NUMBER_OF_TOKENS
MAX_QUANTUM_REGISTER_SIZE
MAX_QUBITS
MAX_CPUS
MAX_GPUS
MAX_FPGAS
MAX_NODES
MAX_TENSOR_SIZE

Practical implementation limits MAY exist internally for memory exhaustion protection, but they MUST NOT become semantic language restrictions and MUST be documented separately.

---

4. Lexical Architecture

The lexical pipeline is:

source bytes
    │
    ▼
Unicode decoding
    │
    ▼
code-point stream
    │
    ▼
lexical scanning
    │
    ├── whitespace
    ├── comments
    ├── identifiers
    ├── keywords
    ├── literals
    ├── operators
    ├── delimiters
    ├── attributes
    └── domain notation
    │
    ▼
token stream
    │
    ▼
parser

The lexer MUST be deterministic.

Given the same source bytes and lexical specification version, it MUST produce the same token sequence.

---

5. Source Encoding

Zamani source files MUST be interpreted as Unicode text.

UTF-8 SHOULD be the canonical source encoding.

A conforming implementation:

1. MUST decode UTF-8 deterministically;
2. MUST reject malformed UTF-8;
3. MUST preserve source offsets sufficiently for diagnostics;
4. MUST NOT silently reinterpret malformed byte sequences.

The language MUST NOT require ASCII-only source files.

ASCII remains supported for compatibility and ease of tooling.

---

6. Unicode

Zamani is Unicode-aware.

Unicode support applies to:

- identifiers;
- strings;
- characters;
- comments;
- documentation;
- source metadata;
- user-visible diagnostics.

Unicode does not automatically make every Unicode character syntactically meaningful.

The lexer MUST distinguish:

Unicode character allowed in identifier

from:

Unicode character that is merely valid source text

and:

Unicode character with lexical meaning

---

7. Unicode Normalization

Identifiers SHOULD have a defined normalization policy.

The implementation MUST NOT accidentally create two identifiers that compare differently merely because of equivalent Unicode representations unless the language explicitly defines that behavior.

The specification SHOULD use Unicode normalization consistently for identifier comparison.

Source spelling SHOULD remain available for diagnostics.

For example, the lexer may retain:

original spelling
normalized identity

as separate information.

The original source MUST NOT be rewritten merely because normalization occurs.

---

8. Unicode Security

The lexer and tooling SHOULD detect potentially confusing Unicode constructs, including:

- visually confusable identifiers;
- invisible formatting characters;
- bidirectional-control characters;
- unexpected zero-width characters.

Security diagnostics MUST NOT silently alter source meaning.

Where a character is prohibited, the lexer MUST produce a deterministic diagnostic.

---

9. Whitespace

Whitespace separates lexical elements where necessary.

Whitespace MAY include Unicode whitespace recognized by the lexical specification.

Whitespace generally has no semantic meaning unless a specific future or existing construct explicitly defines it as meaningful.

The parser MUST NOT depend on arbitrary formatting whitespace.

These should generally be equivalent:

let x = 1;

and:

let
x
=
1
;

unless a specific lexical or syntactic construct explicitly states otherwise.

---

10. Line Terminators

The lexer MUST recognize the supported line-ending forms consistently.

At minimum, implementations SHOULD handle:

LF
CRLF
CR

without changing program meaning.

Line terminators MUST be preserved through source locations even when they are not emitted as ordinary tokens.

---

11. Significant Newlines

Newlines are not automatically significant.

A construct MAY explicitly require newline sensitivity, but such behavior MUST be specified in the syntax contract.

The lexer MUST NOT infer statement boundaries merely because a newline occurs unless the language specification says so.

This avoids accidental incompatibility between:

a + b

and:

a
+ b

where the syntax permits continuation.

---

12. Comments

Comments are lexically discarded from ordinary parsing unless explicitly requested by tooling.

Zamani SHOULD support:

// line comment

and:

/* block comment */

where those forms are part of the canonical grammar.

Block comments MUST have deterministic termination behavior.

An unterminated block comment MUST produce a lexical error.

---

13. Nested Block Comments

If nested block comments are supported, nesting MUST be explicitly defined.

If nesting is not supported, the first matching terminator closes the comment.

The implementation MUST NOT accidentally provide different behavior from "Zamani.g4".

If nested comments are adopted, the behavior MUST be shared by:

- "src/lexer.rs";
- "Zamani.g4";
- lexical tests;
- syntax tooling.

---

14. Documentation Comments

Documentation comments MAY be recognized separately from ordinary comments.

If supported, they MUST produce distinct token metadata or be made available to documentation tooling.

Documentation comments MUST NOT change program semantics unless a future explicit language feature defines such behavior.

---

15. Identifiers

An identifier represents a programmer-defined or implementation-defined name.

Conceptually:

identifier
    = identifier-start identifier-continue*

The exact character classes MUST be defined in:

grammar/lexer/identifiers.md

and implemented consistently by:

src/lexer.rs
grammar/Zamani.g4

Identifiers SHOULD support Unicode where safe and practical.

---

16. Identifier Start

An identifier-start character MAY include:

- Unicode letters;
- "_";
- other explicitly approved Unicode identifier categories.

Digits MUST NOT be the first character of an ordinary identifier unless the lexical specification explicitly introduces a separate identifier class.

Thus:

value1

is valid in principle.

But:

1value

is lexically interpreted as a number followed by another token or rejected according to the exact lexical rules.

---

17. Identifier Continuation

After the first character, identifiers MAY contain:

- letters;
- Unicode identifier characters;
- digits;
- underscore;
- explicitly approved continuation characters.

Identifier rules MUST be independent of machine architecture.

There is no language-level maximum number of identifier characters.

---

18. Reserved Keywords

Keywords are lexical words with language-defined syntactic roles.

The canonical keyword registry belongs under:

grammar/lexer/keywords.md

and MUST correspond with:

src/lexer.rs
grammar/Zamani.g4
grammar/specification/syntax.md

A keyword MUST NOT be added merely because a library, vendor, accelerator, framework, or hardware platform exists.

---

19. Keyword Stability

Keywords are part of the language compatibility surface.

Adding a new hard keyword can break existing programs.

Therefore new keywords SHOULD undergo compatibility review.

Where practical, Zamani SHOULD prefer contextual recognition for extensible concepts that do not require globally reserved syntax.

---

20. Contextual Keywords

A contextual keyword is treated as a keyword only in a syntactic context where that meaning is required.

This mechanism is useful for extensibility without consuming the global identifier namespace unnecessarily.

Contextual keyword behavior MUST be deterministic and specified by the parser contract.

The lexer MUST NOT depend on semantic type information to decide whether a character sequence is lexically valid.

---

21. Domain Names

Quantum, hardware, AI, HDL, networking, security, and other domain names SHOULD normally be represented as identifiers or qualified names rather than global reserved keywords.

For example, a semantic operation such as:

quantum.measure

does not require every possible future quantum operation to become a keyword.

This is essential for extensibility.

---

22. Qualified Names

Zamani supports qualified naming concepts.

Conceptually:

namespace.name

or the canonical qualified-name syntax defined by "syntax.md".

The lexer MUST recognize the individual lexical components consistently.

It MUST NOT resolve whether a name refers to:

- a module;
- type;
- function;
- operation;
- quantum operation;
- hardware capability;
- library;
- vendor extension.

Name resolution belongs to semantic analysis.

---

23. Namespaces and Vendor Extensions

Vendor or external names MUST NOT require new lexer keywords.

For example:

vendor.operation

may be lexically represented using ordinary identifier/path tokens.

The compiler or interoperability layer may later determine the meaning.

This preserves future extensibility.

---

24. Numeric Literals

Numeric literals represent programmer-specified numeric values.

The lexical specification MUST NOT impose artificial numeric magnitude limits.

The following concepts SHOULD be supported where defined by the language:

- decimal integers;
- binary integers;
- octal integers;
- hexadecimal integers;
- decimal floating-point;
- hexadecimal floating-point where supported;
- exponent notation;
- digit separators;
- signed/unsigned forms through syntax rather than ambiguous lexical behavior;
- arbitrary-precision source literals where supported by the semantic type system.

The lexer recognizes representation.

The type system determines numeric meaning.

---

25. Numeric Magnitude

The language MUST NOT define numeric magnitude using the implementation's native machine integer size.

For example, the lexer MUST NOT reject a literal solely because it exceeds:

u64
i64
u128

unless the literal is being parsed specifically into a semantic type that imposes such a constraint.

This distinction is essential:

source literal capacity

is not the same as:

target machine integer width

---

26. Integer Prefixes

Where supported, integer prefixes MUST be unambiguous.

Typical forms include:

0b...
0o...
0x...

The exact canonical spellings belong in:

grammar/lexer/numeric-literals.md

Unknown or malformed prefixes MUST produce deterministic lexical errors.

---

27. Digit Separators

Digit separators MAY be supported to improve readability.

For example:

1_000_000

If supported, separator placement MUST be specified.

Invalid forms such as leading, trailing, or repeated separators MUST either be explicitly supported or rejected.

There MUST be no implementation-dependent interpretation.

---

28. Floating-Point Literals

Floating-point literals MUST have an unambiguous lexical form.

The lexer MUST distinguish:

integer

from:

floating-point literal

without relying on target floating-point width.

The semantic type system determines whether a literal becomes:

- a machine floating-point value;
- arbitrary precision;
- decimal arithmetic;
- symbolic numeric representation;
- another supported numeric representation.

---

29. Exponents

Exponent notation MUST be deterministic.

Conceptually:

1e10
1.5e-9

may represent numeric literals where the syntax supports them.

The lexer MUST reject malformed exponents rather than silently splitting them into unrelated tokens where doing so would hide a source error.

---

30. Boolean Literals

Boolean literals are language-defined lexical forms.

If:

true
false

are the canonical forms, they MUST be recognized consistently across:

- "src/lexer.rs";
- "Zamani.g4";
- syntax specification;
- AST;
- tests.

They represent language-level values and do not encode machine-specific representations.

---

31. Character Literals

Character literals represent a character or explicitly defined character value.

They MUST support the language's Unicode model.

A character literal MUST have deterministic escape handling.

Invalid character literal forms MUST produce lexical diagnostics.

The semantic representation MUST NOT assume that every Unicode character occupies one byte.

---

32. String Literals

Zamani string literals represent sequences of characters or bytes according to their declared literal form.

The lexer MUST distinguish any supported forms such as:

"text"

from:

b"bytes"

or other explicitly specified variants.

The exact forms belong to the lexical and type specifications.

---

33. String Size

The language MUST NOT impose a fixed maximum string length.

A string may be:

- small;
- large;
- streamed;
- generated;
- distributed;
- lazily represented;

subject to available resources and semantic constraints.

An implementation may encounter resource exhaustion, but such exhaustion MUST NOT become a language-level maximum.

---

34. Escape Sequences

Escape sequences MUST have a centralized definition.

The lexer SHOULD support explicitly specified forms for:

- quote characters;
- backslashes;
- newline;
- carriage return;
- tab;
- Unicode code points;
- byte escapes where applicable.

Unknown escapes MUST produce deterministic diagnostics unless explicitly allowed.

---

35. Unicode Escapes

Unicode escape syntax MUST define:

- valid hexadecimal digits;
- required digit counts;
- scalar-value validation;
- surrogate handling;
- invalid code-point handling.

The lexer MUST reject invalid Unicode scalar values.

---

36. Raw Strings

Raw strings MAY be supported for source text that contains many escape characters.

If supported, delimiter rules MUST guarantee deterministic termination.

Raw-string delimiters MUST NOT impose a fixed maximum content size.

---

37. String Interpolation

If Zamani supports interpolation, the lexical contract MUST define how literal text and embedded expressions interact.

Conceptually:

"hello ${name}"

may contain:

string text
expression start
identifier
expression end

The lexer/parser boundary MUST be explicitly defined.

Interpolation MUST NOT cause the lexer to interpret arbitrary runtime values.

---

38. Operators

Operators are lexical tokens whose syntactic combination determines operations.

The canonical operator registry belongs in:

grammar/lexer/operators.md

Operators MUST define:

- spelling;
- token identity;
- lexical precedence when necessary;
- ambiguity resolution;
- parser usage;
- compatibility status.

Operator meaning is ultimately determined by semantic analysis.

---

39. Longest-Match Rule

Where multiple tokenizations are possible, the lexer SHOULD apply the deterministic longest-match rule unless a specific lexical rule overrides it.

For example, if both:

>

and:

>=

exist, the input:

>=

MUST produce the longer operator token.

The same principle applies to other multi-character operators.

---

40. Operator Extensibility

The lexer MUST NOT reserve every possible punctuation combination merely for future use.

Reserved operators SHOULD be documented.

New operators require compatibility analysis.

Semantic libraries MUST NOT require new lexer operators merely to expose ordinary mathematical or domain functionality.

---

41. Delimiters

Delimiters include constructs such as:

(
)
[
]
{
}

and any additional canonical Zamani delimiters.

Delimiter spelling MUST be centralized.

Delimiters MUST NOT encode machine dimensions.

For example:

array[1024]

contains a program value.

The delimiter "[" does not imply a 1024-element limit.

---

42. Punctuation

Punctuation tokens include separators and structural symbols such as:

,
:
;
.

and other explicitly defined punctuation.

Every punctuation character with syntactic meaning MUST have one canonical lexical interpretation unless a context-sensitive rule is explicitly specified.

---

43. Attributes and Annotations

Zamani attributes/annotations MUST use a centralized lexical contract.

They may carry metadata such as:

- compilation hints;
- semantic attributes;
- resource requirements;
- capabilities;
- optimization preferences;
- interoperability metadata;
- diagnostics metadata.

Attributes MUST NOT silently become hardware-specific compiler directives.

---

44. Resource and Capability Syntax

Lexically, resource/capability constructs are composed from ordinary identifiers, literals, operators, and delimiters wherever practical.

The lexer MUST NOT contain special tokens for:

MAX_CPU
MAX_GPU
MAX_QUBIT
MAX_NODE

or equivalent implementation limits.

Resource semantics belong to:

grammar/resources/
grammar/hardware/
grammar/compile/
grammar/execution/

---

45. Quantum Lexical Forms

Quantum notation is a first-class lexical concern but not a separate lexical language.

The lexer MAY recognize explicitly defined quantum notation such as:

|0⟩
|1⟩
|ψ⟩

and other canonical quantum literal forms.

Quantum lexical support MUST remain extensible.

The lexer MUST NOT contain a finite list such as:

H
X
Y
Z
CNOT
SWAP
...

as the only possible quantum operations.

---

46. Quantum Operation Names

Quantum operations SHOULD use ordinary identifier or qualified-name mechanisms.

For example:

H
custom_gate
vendor.phase
domain.operation

may all be represented using the same general lexical infrastructure.

This allows future operations without changing the lexer for every new operation.

---

47. Quantum Resource Identifiers

Identifiers such as:

q
q0
register
ancilla
logical

are names.

They MUST NOT be confused with physical-device identifiers unless the program explicitly enters a target-realization context.

The lexer has no responsibility for determining whether:

q17

is a logical qubit, physical qubit, register element, or ordinary identifier.

---

48. Quantum Number Scaling

There is no lexical limit on:

- number of qubit identifiers;
- register names;
- operation names;
- quantum parameters;
- quantum literals.

The lexer MUST NOT contain:

MAX_QUBITS

or equivalent.

A resource limit belongs to target/resource analysis.

---

49. HDL Lexical Forms

HDL constructs MUST use the common lexical system.

HDL-specific keywords MAY exist where language-level semantics justify them.

However, the lexer MUST NOT encode a fixed:

- bit width;
- register width;
- bus width;
- number of ports;
- number of modules;
- number of signals;
- number of pipeline stages;
- number of hardware units.

For example:

data[width]

may express parameterized width.

The lexer does not decide the value of "width".

---

50. Hardware Names

Hardware names SHOULD use identifiers and qualified names.

The lexer MUST NOT reserve:

gpu0
gpu1
qpu0
fpga0
cpu0
core0

as special language tokens.

Such names may be ordinary identifiers or semantic target descriptions.

---

51. AI and Data Lexical Forms

AI/ML/data concepts MUST use the same lexical foundation.

The lexer MUST NOT contain a keyword for every:

- model architecture;
- optimizer;
- neural network;
- tensor operation;
- dataset format;
- framework;
- vendor accelerator.

Generic lexical mechanisms allow these concepts to evolve independently.

---

52. Networking Lexical Forms

Networking constructs use:

- identifiers;
- qualified names;
- literals;
- strings;
- addresses where explicitly supported;
- operators;
- delimiters.

The lexer MUST NOT assume a fixed:

- address width;
- number of endpoints;
- number of nodes;
- number of network interfaces;
- topology size.

---

53. Security Lexical Forms

Security constructs use the common lexical system.

Cryptographic algorithms SHOULD NOT automatically become globally reserved keywords.

For example, a cryptographic operation may be represented through a qualified operation name and semantic capability.

The lexer must recognize syntax, not implement cryptography.

---

54. Time and Duration Literals

If Zamani supports temporal literals, their lexical forms MUST be defined independently from target clock representation.

The language MUST NOT assume:

32-bit timestamp
64-bit timestamp
fixed clock frequency
fixed timer width

Temporal semantics belong to the semantic/execution specifications.

---

55. Units and Quantities

If units are supported, the lexical model SHOULD allow a numeric value and unit representation without making the lexer responsible for unit conversion.

Examples may conceptually include:

10 ms
5 GB
3 GHz
20 ns

The semantic/type layer determines:

- dimensional compatibility;
- conversion;
- precision;
- resource meaning.

---

56. Paths and URLs

If paths, module paths, or URI-like values exist, their lexical treatment MUST be explicit.

The lexer MUST NOT automatically perform filesystem or network access.

A lexical string that looks like a path is still source text.

Security-sensitive resolution occurs outside lexical analysis.

---

57. No Lexical I/O

The lexer MUST NOT:

- read arbitrary files;
- access networks;
- query hardware;
- inspect QPU topology;
- inspect CPU count;
- inspect GPU count;
- inspect environment variables for semantic decisions;
- execute user code.

The lexer is a pure source-to-token transformation.

---

58. Token Identity

Every emitted token MUST have a stable token kind.

Token identity MUST NOT depend on:

- target machine;
- runtime state;
- hardware capabilities;
- available resources;
- compiler optimization level.

The same source MUST tokenize identically regardless of whether the eventual target is:

- a microcontroller;
- CPU;
- GPU;
- FPGA;
- QPU;
- cluster;
- cloud;
- future computational architecture.

---

59. Token Text Preservation

The lexer SHOULD preserve source spans.

For diagnostics and tooling, implementations SHOULD retain sufficient source information to recover the original lexeme.

This is especially important for:

- Unicode;
- numeric literals;
- strings;
- attributes;
- macros;
- diagnostics;
- formatting;
- provenance.

---

60. Source Spans

Every token consumed by the parser MUST have source-location information sufficient for diagnostics.

A source span SHOULD identify:

source file/unit
start position
end position

The implementation MUST define whether positions are measured in:

- bytes;
- Unicode scalar values;
- grapheme clusters;
- line/column coordinates.

The compiler/tooling representation SHOULD preserve byte offsets for reliable source slicing while providing human-readable line/column information.

---

61. Source Files and Multiple Units

The lexical model MUST support arbitrarily many source units subject to available resources.

A lexer invocation MUST NOT assume:

one source file

is the maximum program size.

Modules and packages are handled by the module/build system.

The lexical specification only defines tokenization of each source unit.

---

62. Token Stream Scalability

The lexical specification MUST support programs ranging from:

tiny embedded program

to:

large scientific application

to:

distributed application

to:

large hardware/software co-design system

without changing language semantics.

The lexer SHOULD use streaming or incremental techniques where beneficial.

No fixed token-count language limit is permitted.

---

63. Memory Exhaustion

An implementation MAY fail because the host system cannot allocate enough memory.

That is an implementation resource failure.

It MUST NOT be represented as a language rule such as:

programs may contain at most N tokens

unless N is explicitly documented as an implementation safety bound rather than a language restriction.

---

64. Infinite Source

A source file is a finite source artifact.

Zamani does not require a lexer to tokenize an actually infinite byte stream.

However, the language MUST NOT impose an arbitrary small maximum source size.

Programs that produce unbounded computation use runtime semantics rather than infinite source files.

---

65. Determinism

Lexical analysis MUST be deterministic.

For:

same source bytes
+
same language version
+
same lexical configuration

the lexer MUST produce:

same token kinds
same token boundaries
same lexical values
same source spans

Any explicitly configurable lexical dialect MUST be represented in the compilation configuration and versioned.

---

66. Error Recovery

The lexer SHOULD recover sufficiently to report multiple independent lexical errors where practical.

However, recovery MUST NOT change the meaning of successfully recognized source.

Every recovery strategy MUST be deterministic.

---

67. Lexical Errors

Lexical errors MUST include enough information to identify:

- error category;
- source span;
- offending lexeme where safe;
- expected lexical form where determinable;
- language version;
- relevant recovery information where applicable.

Typical categories include:

invalid UTF-8
invalid Unicode scalar
invalid identifier
invalid numeric literal
invalid escape
unterminated string
unterminated character literal
unterminated comment
invalid operator
invalid delimiter
invalid literal

---

68. Diagnostics

Diagnostics SHOULD use stable error identifiers.

For example:

LEX001
LEX002
...

The exact registry belongs under:

grammar/lexer/diagnostics.md
grammar/spec/diagnostics.md

Error codes MUST remain compatible across releases unless intentionally deprecated.

---

69. Invalid Source

The lexer MUST reject malformed lexical constructs rather than silently reinterpret them when doing so could produce a different valid program.

For example, a malformed numeric literal SHOULD NOT silently become two unrelated literals if the source clearly intended one numeric literal.

---

70. Lexical Ambiguity

Every lexical ambiguity MUST have a deterministic resolution.

Resolution mechanisms include:

1. longest match;
2. lexical precedence;
3. explicit delimiters;
4. contextual tokenization where explicitly defined;
5. parser-level disambiguation where lexical ambiguity is intentionally preserved.

The implementation MUST NOT resolve ambiguity using machine-specific behavior.

---

71. Lexer and ANTLR Integration

"grammar/Zamani.g4" is the canonical ANTLR composition root.

The lexical contract MUST be reflected by the grammar's lexer rules.

ANTLR-specific implementation details MUST NOT silently redefine the normative lexical model.

If ANTLR limitations require an implementation technique different from the conceptual specification, the mapping MUST be documented.

---

72. Lexer and "src/lexer.rs"

"src/lexer.rs" is the Rust implementation of the lexical contract.

It MUST conform to:

grammar/specification/lexical.md
grammar/lexer/*
grammar/spec/lexical.md

The implementation MUST NOT silently introduce lexical constructs absent from the specification.

Likewise, the specification MUST NOT claim lexical constructs that the implementation intentionally cannot support without recording their conformance status.

---

73. Lexer–Parser Boundary

The lexer owns:

characters → tokens

The parser owns:

tokens → syntax tree

The lexer MUST NOT perform parsing.

The parser MUST NOT need to reconstruct characters that the lexer discarded unless the lexical contract explicitly requires source preservation.

---

74. Lexer–AST Boundary

The lexer does not construct semantic AST nodes.

The AST layer consumes parser output.

The desired architecture is:

source
 ↓
lexer
 ↓
parser
 ↓
domain-neutral AST
 ↓
structural validation
 ↓
semantic model

This preserves the established requirement that the AST remain domain-neutral and not become a second quantum IR.

---

75. Quantum AST Boundary

Quantum lexical tokens MUST NOT force a fixed AST such as:

enum QuantumGate {
    X,
    H,
    CNOT,
    ...
}

Instead, lexical representation should permit generic operations.

The semantic representation can carry operation information such as:

name
namespace
operands
parameters
results
attributes
modifiers
effects
capabilities
source

The eventual quantum semantic representation is lowered through the canonical "quantum::ir".

---

76. Classical AST Boundary

Mathematical identifiers and operations MUST NOT require a keyword for every mathematical function.

For example, mathematical functionality may be represented through:

identifier
qualified identifier
call
operator
literal
generic type

Semantic libraries and intrinsic capabilities determine meaning.

---

77. HDL AST Boundary

HDL lexical forms MUST remain sufficiently generic for:

- parameterized widths;
- generated hardware;
- arbitrary signal counts;
- arbitrary module counts;
- configurable timing;
- hardware/software co-design.

The lexer does not decide whether a construct targets:

- ASIC;
- FPGA;
- CPU;
- GPU;
- accelerator;
- simulator;
- future hardware.

---

78. Resource AST Boundary

Resource expressions remain source-level constructs until semantic analysis.

For example:

requires capability("quantum.measurement")

is lexically just a sequence of:

identifier
identifier
punctuation
string
...

The lexer does not determine whether a target actually has that capability.

---

79. Macro and Metaprogramming Boundary

Macros may operate over tokens or syntax trees, but macro expansion MUST ultimately produce valid Zamani syntax subject to normal validation.

Macro systems MUST NOT bypass lexical correctness.

Generated tokens MUST use the same canonical token definitions.

---

80. Dialects

Dialect extensions MUST NOT silently redefine core token meanings.

Every dialect MUST declare:

- name;
- version;
- lexical extensions;
- keyword additions;
- operator additions;
- compatibility policy;
- parser integration.

Dialect-specific lexical rules MUST be isolated from the core language wherever possible.

---

81. Reserved Extension Space

The language MAY reserve carefully defined lexical extension points.

However, reservation MUST be minimal.

The goal is:

stable core
+
extensible namespaces
+
explicit dialects

rather than:

global keyword list containing every future feature

---

82. Feature Flags

Experimental lexical constructs MUST be gated explicitly.

A feature gate MUST identify:

- feature name;
- version;
- status;
- syntax;
- compatibility;
- implementation state.

Feature flags MUST NOT silently change the meaning of stable source.

---

83. Versioning

Lexical behavior is versioned with the language.

A compiler MUST know which language version governs lexical interpretation.

A source file MAY explicitly select a language version if the language syntax supports version declarations.

Otherwise, project/build metadata determines the version according to the compatibility specification.

---

84. Backward Compatibility

Changes to:

- keyword sets;
- identifier rules;
- operator spellings;
- literal syntax;
- escape syntax;
- comment syntax;

are compatibility-sensitive.

A new version MUST document incompatible lexical changes.

Where practical, migrations SHOULD be mechanically detectable.

---

85. Forward Compatibility

Unknown future constructs SHOULD fail predictably rather than being silently interpreted as something else.

This is especially important for:

- new operators;
- new keywords;
- new literals;
- new dialects;
- future computational paradigms.

---

86. Deprecation

Deprecated lexical constructs MUST be documented.

A deprecated construct SHOULD:

1. remain recognized for the supported compatibility period;
2. produce a warning where appropriate;
3. have a migration path;
4. eventually be removed only through the language compatibility process.

---

87. Security and Injection Resistance

The lexer MUST treat source as data.

It MUST NOT:

- execute commands;
- evaluate expressions;
- resolve network resources;
- load arbitrary files;
- instantiate hardware;
- invoke vendor APIs.

This is particularly important for:

- macros;
- interpolation;
- attributes;
- external tooling;
- generated source.

---

88. No Hardware Discovery During Lexing

Lexical analysis MUST be independent of the machine executing the compiler.

The lexer MUST NOT ask:

How many CPUs exist?
How much RAM exists?
How many GPUs exist?
How many QPUs exist?
How many qubits exist?
What FPGA is installed?
What topology exists?

The lexical result MUST remain unchanged regardless of the compilation host.

---

89. POCO-REAF

The lexical layer is foundational to:

«Program Once, Compile Once, Run Everywhere, Anywhere, Forever»

POCO-REAF requires lexical source meaning to remain independent of the physical target.

A source program must not require lexical rewriting merely because the target changes from:

microcontroller

to:

CPU

to:

GPU

to:

FPGA

to:

QPU

to:

distributed system

or a future architecture.

---

90. Scaling Model

Zamani's lexical model scales along all relevant dimensions.

Program size

No fixed number of tokens.

Identifier count

No fixed number of identifiers.

Identifier length

No artificial language maximum.

String size

No artificial language maximum.

Numeric magnitude

No fixed machine-width lexical ceiling.

Quantum resources

No fixed qubit lexical ceiling.

Hardware resources

No fixed CPU/GPU/FPGA/QPU lexical ceiling.

Distributed resources

No fixed node count.

Tensor dimensions

No fixed dimension encoded into lexical rules.

Modules

No fixed module count.

Functions

No fixed function count.

Domains

No fixed number of supported computational domains.

The only practical limits are resources available to the implementation.

---

91. Important Distinction: Language Constant vs Implementation Limit

This specification does not prohibit programmers from writing explicit values.

This is valid:

let n = 1024;

This is valid:

allocate qubits[n];

This is valid:

tensor<float, 1024, 1024>

if those constructs are valid under the type/semantic specification.

What is prohibited is turning those values into universal compiler restrictions:

MAX_QUBITS = 1024

or:

lexer rejects values above 1024

The lexical layer must never confuse programmer intent with implementation limits.

---

92. Arbitrary Resource Expressions

Resource quantities SHOULD be lexically expressible using normal numeric, identifier, unit, and expression mechanisms.

This permits:

n

required_memory

problem_size * memory_per_element

rather than requiring fixed constants.

Resource evaluation belongs to semantic analysis and resource management.

---

93. Mathematical Extensibility

The lexical system MUST support mathematical extensibility without continually expanding the keyword set.

A new mathematical operation SHOULD normally be expressible using:

identifier
qualified identifier
call
operator
generic syntax

rather than requiring a lexer update.

This prevents mathematical vocabulary from becoming a parser bottleneck.

---

94. Scientific Computing

Scientific notation, units, symbolic names, complex values, tensors, matrices, vectors, and related constructs MUST be expressible through the common lexical system.

The lexer MUST NOT assume:

- fixed precision;
- fixed tensor rank;
- fixed vector width;
- fixed matrix dimension.

Those properties belong to semantic types and target realization.

---

95. Distributed and Parallel Computing

The lexer MUST remain independent of execution topology.

A token such as:

parallel

does not imply:

8 threads

Likewise:

distributed

does not imply:

16 nodes

Resource and placement semantics are resolved downstream.

---

96. Embedded Computing

Zamani source may target extremely constrained devices.

The lexical language remains the same.

A compiler targeting a small device may reject or transform a program because of available resources, but the lexical specification itself does not become smaller or impose arbitrary language restrictions.

This supports source portability across scales.

---

97. Cloud and Edge Computing

The same lexical source model applies to:

- edge;
- cloud;
- cluster;
- supercomputer;
- distributed system;
- heterogeneous accelerator systems.

No lexical token may encode a mandatory deployment size.

---

98. Future Computational Paradigms

The lexical architecture MUST permit future computational models without requiring redesign of the core lexer whenever possible.

Future domains should be introduced through:

existing lexical primitives
+
explicit syntax extensions
+
dialects
+
semantic capabilities

rather than arbitrary global keyword growth.

---

99. Lexical Registry

The repository SHOULD maintain a canonical registry under:

grammar/lexer/

including:

tokens.md
keywords.md
operators.md
identifiers.md
literals.md
numeric-literals.md
comments.md
unicode.md
quantum-literals.md
diagnostics.md
conformance.md

These files refine this normative document.

They MUST NOT contradict it.

If a conflict exists, the specification authority process MUST resolve it before implementation is considered conformant.

---

100. Token Registry Requirements

Every token definition MUST identify:

Token Name
Lexical Spelling
Category
Recognition Rule
Precedence
Context Sensitivity
Source Span
Parser Consumers
AST Consumers
Semantic Meaning
Compatibility Status
Negative Cases
Boundary Cases
Scalability Requirements

A token is not considered complete merely because it has been added to an ANTLR lexer rule.

---

101. Keyword Registry Requirements

Every keyword MUST identify:

Keyword
Status
Language Version
Lexical Form
Syntactic Consumers
AST Mapping
Semantic Meaning
Reserved/Contextual Classification
Compatibility Impact
Dialect Restrictions
Tests

---

102. Operator Registry Requirements

Every operator MUST identify:

Operator
Spelling
Token
Arity
Precedence
Associativity
Parser Consumer
AST Representation
Semantic Contract
Compatibility Status

The lexer does not decide semantic overload resolution.

---

103. Literal Registry Requirements

Every literal class MUST identify:

Literal Kind
Lexical Form
Escape Rules
Normalization
Semantic Value
Type-System Integration
AST Representation
Error Conditions
Boundary Tests
Scalability Tests

---

104. Feature Manifest Integration

Features under:

grammar/specification/features/

MUST reference their lexical dependencies.

For example:

lexer_tokens:
  - identifier
  - integer_literal
  - quantum_operation_name

The feature manifest MUST NOT redefine the token spelling independently.

It references the canonical lexical registry.

---

105. Grammar Integration

"grammar/Zamani.g4" MUST consume the canonical lexical model.

The root grammar MUST remain the composition point.

Domain grammars under:

grammar/quantum/
grammar/hdl/
grammar/classical/
grammar/ai/
...

MUST NOT introduce incompatible lexical definitions.

Shared lexical constructs MUST be reused.

---

106. Existing "grammar.md"

"grammar/grammar.md" serves as an implementation-conformance reference.

It SHOULD report lexical status such as:

SPECIFIED
IMPLEMENTED
PARTIALLY IMPLEMENTED
EXPERIMENTAL
DEPRECATED
NOT IMPLEMENTED

It MUST NOT silently become a second normative lexical authority.

---

107. Existing "Zamani-Grammar.md"

"grammar/Zamani-Grammar.md" remains a broader design/reference document.

Its lexical concepts MAY be harvested into the canonical lexical specification.

However:

«Presence in "Zamani-Grammar.md" does not automatically make a lexical construct valid Zamani syntax.»

Promotion requires:

proposal
 ↓
lexical contract
 ↓
syntax contract
 ↓
AST contract
 ↓
semantic contract
 ↓
implementation
 ↓
tests
 ↓
compatibility approval

---

108. Existing "grammar/DESIGN.md"

"grammar/DESIGN.md" remains the architectural authority for grammar organization.

This lexical specification implements its principles:

- one language;
- deterministic parsing;
- no artificial hardware limits;
- modular grammar;
- explicit ownership;
- canonical semantic boundaries;
- portability;
- compatibility;
- validation.

---

109. Existing "grammar/spec/lexical.md"

If "grammar/spec/lexical.md" already exists, it should serve as the machine-checkable/formal contract corresponding to this specification.

It MUST NOT introduce lexical rules that contradict this document.

The relationship is:

specification/lexical.md
        │
        ▼
formal lexical contracts
        │
        ▼
grammar/lexer/*
        │
        ▼
Zamani.g4
        │
        ▼
src/lexer.rs

---

110. Relationship to "src/parser.rs"

The parser MUST consume the token contract defined here.

If the parser requires a token that is not defined here, the lexical contract is incomplete.

If the lexer emits tokens that the parser cannot consume and that are not explicitly transitional, the implementation is non-conformant.

---

111. Relationship to the Frontend AST

The frontend AST remains domain-neutral.

Lexical constructs MUST map through syntax and parsing into generic structural nodes where possible.

The lexical layer must not force:

QuantumGate
HardwareDevice
GpuInstruction
QecOperation
VendorInstruction

into the AST merely because those concepts exist lexically.

---

112. Relationship to Quantum IR

The lexical layer has no direct dependency on the internal structure of "quantum::ir".

The architecture remains:

quantum source
      ↓
lexical tokens
      ↓
parser
      ↓
domain-neutral AST
      ↓
semantic quantum model
      ↓
quantum::ir

This prevents the lexer from becoming coupled to backend quantum implementation details.

---

113. Relationship to QEC

Lexical constructs MAY express QEC-related terminology where the language syntax requires it.

The lexer does not perform error correction.

QEC remains downstream.

---

114. Relationship to ZQN

Lexical constructs MAY represent source-level noise/fault concepts.

The lexer does not simulate or evaluate noise.

ZQN remains responsible for fault/noise semantics downstream.

---

115. Relationship to HAL

The lexer MUST NOT query HAL.

Hardware capability information is runtime/compiler semantic data.

---

116. Relationship to Routing

The lexer does not perform physical mapping.

A source identifier is not automatically a physical resource identifier.

Routing remains responsible for realization.

---

117. Relationship to Scheduling

The lexer does not schedule operations.

Timing notation, if present, is lexical source data whose meaning is established later.

Scheduling remains downstream.

---

118. Relationship to Resilience

The lexer does not perform retries, recovery, health management, or orchestration.

Those belong to the resilience architecture.

---

119. Safe Rust Requirement

The reference implementation is Rust 1.97 / Rust 1.97.1.

Zamani-owned Rust code MUST NOT use:

unsafe

or unsafe blocks.

The lexer implementation MUST be designed using safe Rust abstractions.

Generated Rust owned by the Zamani project SHOULD also comply with the repository's no-"unsafe" policy.

---

120. Dependency Safety

The lexical implementation MUST distinguish:

Zamani source safety

from:

compiler implementation safety

The compiler implementation MUST remain safe Rust according to repository policy.

Dependencies MUST be reviewed according to the repository's dependency/security policy.

---

121. Performance

Lexical analysis SHOULD be:

- linear in source size for ordinary input;
- deterministic;
- allocation-conscious;
- suitable for incremental tooling;
- capable of handling large source units.

The lexical specification MUST NOT require algorithms whose complexity grows unnecessarily with unrelated hardware-resource quantities.

---

122. Incremental Lexing

Tooling MAY support incremental lexing.

Incremental lexing MUST preserve the same tokenization that a complete lexical pass would produce for unchanged source.

Incremental implementation is an optimization, not a separate lexical language.

---

123. Parallel Lexing

Implementations MAY parallelize lexical processing where source boundaries make this safe.

Parallel lexing MUST produce exactly the same observable token stream as deterministic sequential lexing.

---

124. Reproducibility

Lexical output MUST be reproducible.

Compiler host:

CPU
GPU
QPU
OS
memory size
thread count

MUST NOT alter lexical semantics.

---

125. Environment Independence

The lexer MUST NOT use environmental state to decide whether source text is valid.

For example, this is prohibited:

if GPU_exists:
    recognize_gpu_keyword()

Lexical validity is a language property, not a hardware-discovery property.

---

126. No Vendor Leakage

Vendor-specific lexical behavior MUST be isolated through:

- interoperability;
- dialects;
- explicit extensions;
- qualified names.

A vendor cannot silently redefine a core token.

---

127. Canonical Lexical Precedence

The lexical precedence hierarchy SHOULD be:

source decoding
        ↓
comments/whitespace
        ↓
multi-character tokens
        ↓
single-character tokens
        ↓
identifiers/keywords
        ↓
numeric literals
        ↓
string/character literals
        ↓
domain literals
        ↓
error

The exact ordering MUST be implemented consistently.

---

128. Tokenization Example

Illustrative example:

let n = 1024;

may tokenize conceptually as:

LET
IDENTIFIER("n")
EQUAL
INTEGER_LITERAL("1024")
SEMICOLON

The lexer does not decide:

- whether "n" is a resource count;
- whether 1024 qubits are available;
- whether the target is CPU or QPU.

---

129. Quantum Example

Illustratively:

apply custom.phase(theta) to q;

may tokenize as:

APPLY
IDENTIFIER("custom")
DOT
IDENTIFIER("phase")
LPAREN
IDENTIFIER("theta")
RPAREN
TO
IDENTIFIER("q")
SEMICOLON

No new lexer token is required for every future quantum operation.

---

130. Hardware Example

Illustratively:

requires capability("tensor.compute");

may tokenize using ordinary:

IDENTIFIER
IDENTIFIER
LPAREN
STRING_LITERAL
RPAREN
SEMICOLON

The lexer does not determine whether a target provides the capability.

---

131. HDL Example

Illustratively:

module Accelerator(width) { ... }

uses generic:

MODULE
IDENTIFIER
LPAREN
IDENTIFIER
RPAREN
LBRACE
...
RBRACE

"width" remains a program parameter rather than a lexer-defined hardware width.

---

132. Invalid Example

A lexer MUST NOT reinterpret:

0x

as a valid hexadecimal literal followed by an arbitrary token if the canonical syntax requires hexadecimal digits.

It should produce a deterministic lexical diagnostic.

---

133. No Implicit Semantic Evaluation

The lexer MUST NOT evaluate:

1024 * 1024

into a resource size.

It recognizes tokens.

Constant evaluation belongs to later semantic/compiler phases.

---

134. No Implicit Hardware Mapping

The lexer MUST NOT interpret:

q0

as:

physical qubit 0

unless a later explicit target realization construct establishes that meaning.

---

135. Lexical Test Matrix

The lexical test suite MUST cover:

Category| Positive| Negative| Boundary| Scale| Deterministic
Unicode| ✓| ✓| ✓| ✓| ✓
identifiers| ✓| ✓| ✓| ✓| ✓
keywords| ✓| ✓| ✓| ✓| ✓
numbers| ✓| ✓| ✓| ✓| ✓
strings| ✓| ✓| ✓| ✓| ✓
characters| ✓| ✓| ✓| ✓| ✓
comments| ✓| ✓| ✓| ✓| ✓
operators| ✓| ✓| ✓| ✓| ✓
delimiters| ✓| ✓| ✓| ✓| ✓
quantum literals| ✓| ✓| ✓| ✓| ✓
attributes| ✓| ✓| ✓| ✓| ✓
diagnostics| ✓| ✓| ✓| ✓| ✓

---

136. Scalability Test Requirements

The lexical test suite MUST include tests demonstrating that no artificial language limit is introduced for:

- very large identifiers;
- many declarations;
- many modules;
- many tokens;
- large literals;
- large strings;
- many quantum operations;
- many resource expressions;
- large generated programs;
- large HDL descriptions;
- large tensor declarations;
- large distributed configurations.

Tests MAY use progressively larger values rather than a fixed "maximum."

---

137. Boundary Tests

Boundary tests MUST include:

- empty source;
- whitespace-only source;
- comment-only source;
- one-character source;
- minimum valid literals;
- maximum representable semantic values where relevant;
- Unicode boundaries;
- delimiter boundaries;
- adjacent operators;
- adjacent literals;
- identifier/keyword boundaries;
- comment/string termination boundaries.

---

138. Negative Tests

Negative tests MUST include:

- malformed UTF-8;
- invalid Unicode;
- invalid identifiers;
- malformed numbers;
- malformed escapes;
- unterminated strings;
- unterminated comments;
- invalid operators;
- invalid delimiters;
- invalid Unicode escapes;
- ambiguous lexical sequences.

---

139. Compatibility Tests

Compatibility tests MUST verify that:

1. existing stable source continues to tokenize identically;
2. deprecated syntax produces the documented behavior;
3. new keywords do not unexpectedly break identifiers unless explicitly approved;
4. dialect extensions do not alter core lexical meaning;
5. language-version changes are deterministic.

---

140. Determinism Tests

Given the same source, repeated lexing MUST produce identical:

token kinds
token order
token spans
literal values
diagnostics

Tests SHOULD repeat lexical processing across:

- different thread counts;
- different compiler optimization levels;
- different hosts;

where practical.

---

141. Hard-Coding Tests

Automated repository validation SHOULD scan lexical implementation and grammar sources for prohibited universal resource limits such as:

MAX_QUBITS
MAX_CPUS
MAX_GPUS
MAX_FPGAS
MAX_NODES
MAX_THREADS
MAX_MEMORY
MAX_TENSOR
MAX_REGISTER

False positives MUST be reviewable.

Legitimate implementation safety bounds MUST be explicitly documented as implementation limits.

---

142. Grammar/Implementation Conformance

The following must eventually be mechanically comparable:

specification/lexical.md
        ↕
spec/lexical.md
        ↕
lexer/*
        ↕
Zamani.g4
        ↕
src/lexer.rs
        ↕
lexical tests

A feature is not lexically complete if these layers disagree.

---

143. Completion Contract

This file is complete only when all of the following are true:

Specification

- [ ] lexical categories defined;
- [ ] token boundaries defined;
- [ ] Unicode rules defined;
- [ ] identifiers defined;
- [ ] keywords defined;
- [ ] contextual keywords defined;
- [ ] literals defined;
- [ ] operators defined;
- [ ] delimiters defined;
- [ ] comments defined;
- [ ] escapes defined;
- [ ] quantum lexical forms defined;
- [ ] domain extension model defined.

Integration

- [ ] "grammar/lexer/*" contracts mapped;
- [ ] "grammar/Zamani.g4" integration defined;
- [ ] "grammar/spec/lexical.md" relationship defined;
- [ ] "src/lexer.rs" contract defined;
- [ ] "src/parser.rs" boundary defined;
- [ ] frontend AST boundary defined;
- [ ] semantic boundary defined;
- [ ] "quantum::ir" boundary protected;
- [ ] tooling integration defined.

Scalability

- [ ] no artificial hardware limits;
- [ ] no fixed token limit;
- [ ] no fixed identifier limit;
- [ ] no fixed string limit;
- [ ] no fixed numeric magnitude;
- [ ] no fixed qubit limit;
- [ ] no fixed node limit;
- [ ] no fixed tensor dimension;
- [ ] resource limits remain downstream.

Safety

- [ ] no arbitrary I/O;
- [ ] no hardware discovery;
- [ ] no runtime execution;
- [ ] no vendor coupling;
- [ ] safe Rust implementation;
- [ ] no "unsafe" in Zamani-owned Rust implementation.

Testing

- [ ] positive tests;
- [ ] negative tests;
- [ ] boundary tests;
- [ ] Unicode tests;
- [ ] scalability tests;
- [ ] determinism tests;
- [ ] compatibility tests;
- [ ] hard-coding audit.

Only after these conditions are satisfied may the lexical layer be considered production-ready.

---

144. Implementation Independence Rule

After this file is completed, downstream files MUST NOT need to reinterpret its fundamental lexical rules.

A downstream file MAY add:

- semantic meaning;
- syntax composition;
- AST representation;
- domain behavior;
- IR mapping.

A downstream file MUST NOT redefine:

- identifier spelling;
- token identity;
- numeric literal syntax;
- comment syntax;
- keyword meaning;
- operator spelling;

without a formal lexical-specification change.

This is the primary mechanism that makes the file independently completable.

---

145. Required Integration Matrix

Layer| Consumes This File| May Extend| May Override
"lexer/*"| Yes| Detailed contracts| No
"Zamani.g4"| Yes| Composition| No
"grammar.md"| Yes| Implementation status| No
"Zamani-Grammar.md"| Reference| Proposed ideas| No
"spec/syntax.md"| Yes| Syntax composition| No
"src/lexer.rs"| Yes| Implementation| No
"src/parser.rs"| Yes| Parsing| No
AST| Indirectly| Structure| No
semantic analysis| Indirectly| Meaning| No
"quantum::ir"| Indirectly| Quantum semantics| No
compiler| Indirectly| Lowering| No
runtime| Indirectly| Execution| No
HAL| Indirectly| Hardware realization| No

---

146. Authority Rule

If a conflict exists:

language specification
        ↓
formal specification contracts
        ↓
lexical contracts
        ↓
canonical grammar
        ↓
implementation
        ↓
generated/reference documentation

The lower layer MUST NOT silently override the higher layer.

An intentional change must propagate through the documented change process.

---

147. Final Lexical Principle

The Zamani lexer is not a database of today's computers.

It is not a database of today's quantum gates.

It is not a database of today's CPUs.

It is not a database of today's GPUs.

It is not a database of today's FPGAs.

It is not a database of today's AI frameworks.

It is not a database of today's network topologies.

It is not a database of today's hardware limits.

It is the deterministic lexical foundation of one universal programming language.

The fundamental contract is:

SOURCE
  │
  ▼
LEXICAL MEANING
  │
  ▼
SYNTAX
  │
  ▼
AST
  │
  ▼
SEMANTICS
  │
  ▼
CANONICAL IR
  │
  ├──────────────┬───────────────┐
  ▼              ▼               ▼
Classical      quantum::ir      HDL
  │              │               │
  └──────────────┴───────────────┘
                 │
                 ▼
          Optimization
                 │
       ┌─────────┼─────────┐
       ▼         ▼         ▼
    Routing  Scheduling  Resilience
       │         │         │
       └─────────┼─────────┘
                 ▼
                ZQN
                 │
                 ▼
                HAL
                 │
                 ▼
        Target Realization
                 │
       ┌─────────┼────────────┐
       ▼         ▼            ▼
      CPU       GPU          QPU
       │         │            │
      FPGA    Cluster       Future

The lexical layer therefore has one overriding requirement:

«The same valid Zamani source must retain the same lexical meaning regardless of the size, architecture, topology, vendor, or computational substrate on which it will eventually execute.»

That is the lexical foundation required for Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF).

---

148. File Closure Record

When implementation work on this file is declared complete, the repository SHOULD record:

File:
grammar/specification/lexical.md

Specification:
Normative

Implementation:
src/lexer.rs

Grammar:
grammar/Zamani.g4

Detailed Contracts:
grammar/lexer/

Formal Contract:
grammar/spec/lexical.md

Parser Consumer:
src/parser.rs

AST Consumer:
src/frontend/ast/

Semantic Consumer:
semantic analysis

Quantum Boundary:
quantum::ir

Tests:
grammar/tests/lexical/

Validation:
grammar/validation/

Compatibility:
grammar/compatibility/

Status:
Production Ready

No downstream feature may require changing the fundamental lexical model merely because that feature is implemented.

If a genuinely new lexical requirement is discovered, it must enter through the formal language-change process rather than through an undocumented downstream workaround.