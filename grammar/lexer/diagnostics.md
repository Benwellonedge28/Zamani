Zamani Lexer Diagnostics Specification

Path: "grammar/lexer/diagnostics.md"
Language: Zamani
Repository: "Benwellonedge28/Zamani"
Status: Production specification
Implementation baseline: Rust 1.97 / Rust 1.97.1
Safety: Safe Rust only; Rust "unsafe" is prohibited
Scope: Lexical diagnostics and their integration with the Zamani language toolchain

---

1. Purpose

This document defines the complete diagnostic contract for the Zamani lexical layer.

It specifies how lexical errors, warnings, notes, source locations, recovery information, machine-readable diagnostic identities, compatibility behavior, and tooling-facing diagnostic data are represented and propagated.

The goal is deterministic, precise, scalable diagnostics for Zamani programs ranging from the smallest source unit to programs whose size is limited only by available implementation resources.

The lexical diagnostic system MUST support:

- classical computing;
- quantum computing;
- hybrid quantum-classical computing;
- HDL and hardware/software co-design;
- distributed computing;
- parallel and HPC computing;
- AI/ML;
- data processing;
- networking;
- cryptography and security;
- embedded and systems programming;
- accelerator programming;
- future Zamani domains;
- dialects and extensions;
- macros and metaprogramming;
- Unicode source;
- arbitrary program scale subject to available resources.

Diagnostics MUST NOT introduce artificial language limits.

---

2. Architectural Position

The diagnostic pipeline is:

source bytes
    │
    ▼
source decoding
    │
    ▼
Unicode source
    │
    ▼
ZamaniLexer
    │
    ├──────────────► lexical diagnostics
    │
    ▼
tokens + source spans
    │
    ▼
parser
    │
    ├──────────────► syntax diagnostics
    │
    ▼
AST
    │
    ├──────────────► structural diagnostics
    │
    ▼
semantic analysis
    │
    ├──────────────► semantic/type/effect/resource diagnostics
    │
    ▼
canonical semantic model / IR
    │
    ├──────────────► IR/verification diagnostics
    │
    ▼
optimization / lowering
    │
    ▼
routing / scheduling / resilience / QEC / ZQN
    │
    ▼
target realization

This document owns only the lexical portion of that pipeline.

The lexical diagnostic system MUST NOT perform:

- type checking;
- name resolution;
- overload resolution;
- quantum operation validation;
- QEC validation;
- routing;
- scheduling;
- hardware discovery;
- target selection;
- resource allocation;
- calibration;
- runtime execution;
- semantic interpretation.

Those responsibilities belong to later layers.

---

3. Relationship to Existing Repository Files

This file is integrated with the following existing architecture.

3.1 Canonical lexical grammar

The existing canonical ANTLR lexer is:

grammar/antlr/ZamaniLexer.g4

It remains the lexical grammar authority unless the repository's explicit grammar-authority contract is changed.

This document does not create another lexer.

---

3.2 Lexer implementation

The Rust implementation is:

src/lexer.rs

The current implementation already contains:

- "TokenType";
- "Token";
- "LexerError";
- "Span";
- "FileId";
- source-file integration;
- keyword lookup;
- lexical scanning.

The diagnostic contract defined here MUST be implemented consistently by "src/lexer.rs".

---

3.3 Lexer architecture documentation

The parent lexical architecture is:

grammar/lexer/README.md

That document defines lexical ownership and modularization.

This document defines the diagnostic contract used by that architecture.

---

3.4 Lexical specification

This document integrates with:

grammar/spec/lexical.md
grammar/spec/syntax.md
grammar/spec/source-spans.md
grammar/spec/diagnostics.md

If any of those files do not yet exist, they must be created as separate authoritative contracts rather than silently embedding their entire responsibilities here.

---

3.5 Parser

The parser consumes lexical diagnostics indirectly through the token stream and directly through lexical error state.

The parser MUST NOT reinterpret a lexical error as valid syntax.

---

3.6 AST

The AST MUST preserve source provenance for successfully lexed constructs.

A lexical error MUST NOT create a fabricated semantic AST node merely to hide the error.

Error-recovery nodes may exist only if the AST contract explicitly defines them.

---

4. Ownership

This file owns:

- lexical diagnostic categories;
- lexical diagnostic identifiers;
- severity;
- diagnostic structure;
- lexical error messages;
- lexical error spans;
- diagnostic labels;
- recovery metadata;
- related lexical notes;
- diagnostic ordering;
- diagnostic determinism;
- machine-readable diagnostic output contract;
- lexical diagnostic compatibility.

This file does NOT own:

- parser diagnostics;
- type errors;
- semantic errors;
- runtime errors;
- hardware errors;
- QEC errors;
- scheduling errors;
- backend errors.

---

5. Fundamental Diagnostic Principles

5.1 Diagnostics are data

A diagnostic MUST be represented as structured data.

Human-readable text is only one rendering of the diagnostic.

The internal diagnostic model MUST contain enough information for:

- CLI rendering;
- IDE rendering;
- LSP-style tooling;
- JSON output;
- machine processing;
- test assertions;
- documentation generation;
- compiler provenance.

---

5.2 Stable diagnostic identity

Every production diagnostic MUST have a stable identifier.

Recommended namespace:

ZL0001
ZL0002
ZL0003
...

Where:

ZL = Zamani Lexer

Diagnostic numbers MUST NOT encode:

- source line;
- token index;
- machine architecture;
- hardware ID;
- compiler memory address;
- process ID.

The identifier identifies the diagnostic class, not a particular occurrence.

---

6. Severity Model

The lexical diagnostic system supports:

Error
Warning
Info
Hint

However, lexical correctness MUST distinguish between actual invalid source and advisory information.

6.1 Error

The source cannot be correctly tokenized according to the current lexical specification.

Examples:

- illegal character;
- unterminated string;
- malformed escape;
- malformed numeric literal;
- unterminated block comment;
- malformed character literal.

---

6.2 Warning

A lexically valid construct may nevertheless be discouraged or compatibility-sensitive.

Examples:

- deprecated lexical spelling;
- compatibility-only spelling;
- future-reserved identifier warning.

Warnings MUST NOT silently convert valid source into errors.

---

6.3 Info

Informational lexical metadata.

Example:

A deprecated spelling was accepted under compatibility mode.

---

6.4 Hint

Tooling-oriented suggestions.

Example:

Use the canonical spelling `...`.

Hints MUST never alter program semantics.

---

7. Diagnostic Structure

The canonical lexical diagnostic model MUST conceptually contain:

Diagnostic {
    code
    severity
    message
    primary_span
    labels
    notes
    help
    related
    recovery
    phase
}

The exact Rust representation may differ, but the semantic fields are mandatory.

---

8. Required Diagnostic Fields

8.1 Code

Example:

ZL0001

Stable across compiler versions unless the diagnostic contract explicitly marks the code deprecated.

---

8.2 Severity

One of:

error
warning
info
hint

---

8.3 Primary message

The message describes what happened.

It MUST be:

- concise;
- deterministic;
- actionable;
- independent of machine architecture;
- independent of memory addresses;
- independent of nondeterministic ordering.

---

8.4 Primary span

Every source-related lexical diagnostic MUST have a primary source span whenever a source location exists.

---

8.5 Labels

Labels identify relevant source regions.

Example:

expected closing quote

---

8.6 Notes

Notes explain additional context without replacing the primary diagnostic.

---

8.7 Help

Optional corrective guidance.

Example:

help: terminate the string with `"`

---

8.8 Recovery

If lexical recovery occurred, the diagnostic MAY record:

recovery:
    resumed_at
    skipped_range
    synthetic_token
    recovery_strategy

Recovery metadata is tooling information.

It is not semantic source information.

---

9. Diagnostic Ordering

Diagnostics MUST be deterministic.

For diagnostics originating from a single source file, default ordering is:

1. primary source position;
2. end position;
3. diagnostic code;
4. stable discovery order.

Parallel implementation MUST NOT make diagnostic ordering nondeterministic.

The same source, compiler version, language version, and diagnostic configuration MUST produce equivalent diagnostic ordering.

---

10. Source Span Contract

Diagnostics depend on the repository's source-map infrastructure.

The current lexer already associates tokens and lexer errors with "Span".

The lexical diagnostic contract therefore requires:

FileId
+
byte start
+
byte end

as the canonical source identity.

Line and column are derived presentation information.

They MUST NOT replace byte offsets as the fundamental source identity.

---

11. UTF-8 and Unicode

Zamani is Unicode-aware.

Diagnostics MUST distinguish:

byte offset
character position
line
column

when tooling needs all of them.

The compiler MUST NOT assume:

1 character = 1 byte

and MUST NOT calculate Unicode source positions using ASCII-only assumptions.

---

12. Invalid UTF-8

If source acquisition supplies invalid UTF-8 to a UTF-8 Zamani source unit, the source decoding layer SHOULD reject it before normal lexical analysis.

If the lexer itself receives invalid source representation through an internal API, it MUST produce a deterministic diagnostic rather than:

- panicking;
- invoking undefined behavior;
- using "unsafe";
- silently replacing bytes;
- silently dropping bytes.

Recommended diagnostic:

ZL0001 invalid source encoding

Example:

error[ZL0001]: source is not valid UTF-8

The exact message may identify the offending byte span.

---

13. Illegal Character

Diagnostic:

ZL0002

Condition:

A source character cannot participate in any valid Zamani lexical construct in the current lexical context.

Example:

let x = §;

Diagnostic:

error[ZL0002]: illegal character `§`

The diagnostic MUST identify the exact source span.

The lexer SHOULD continue when safe to do so, allowing additional independent diagnostics.

---

14. Unexpected End of Input

Diagnostic:

ZL0003

Use when a lexical construct requires additional source characters but the source ends.

Examples:

"hello

/*
comment

0x

The more specific diagnostics below SHOULD be preferred when the construct can be identified.

---

15. Unterminated String

Diagnostic:

ZL0004

Example:

let message = "hello

Required diagnostic:

error[ZL0004]: unterminated string literal

Recommended label:

string literal starts here

Help:

help: terminate the string literal with `"`

The lexer MUST NOT silently consume the remainder of the source as a string.

---

16. Unterminated Character Literal

Diagnostic:

ZL0005

Example:

let c = 'a

Required behavior:

- identify the opening delimiter;
- report the missing closing delimiter;
- avoid silently converting the rest of the file into the character literal.

---

17. Invalid Escape

Diagnostic:

ZL0006

Example:

"\q"

if "\q" is not a valid Zamani escape.

Diagnostic:

error[ZL0006]: invalid escape sequence `\q`

The diagnostic SHOULD identify the valid escape family when practical.

---

18. Invalid Unicode Escape

Diagnostic:

ZL0007

Examples:

"\u12"

"\uGGGG"

The lexer MUST reject malformed Unicode escape syntax deterministically.

Semantic Unicode validity belongs to the appropriate semantic layer when lexical syntax alone cannot determine it.

---

19. Unterminated Block Comment

Diagnostic:

ZL0008

Example:

/*
 documentation

Required:

error[ZL0008]: unterminated block comment

The diagnostic SHOULD point to the opening delimiter and identify the end of file as the recovery boundary.

---

20. Invalid Numeric Literal

Diagnostic:

ZL0009

Examples include malformed forms such as:

0x
0b
0o
1e
1e+

when these do not form valid lexical tokens.

The diagnostic MUST identify the malformed literal rather than reporting only a generic illegal character.

---

21. Invalid Numeric Separator

Diagnostic:

ZL0010

Examples:

1_
0x_FF
1__000

if these forms are prohibited by the numeric literal specification.

The exact accepted separator policy MUST be centralized in:

grammar/lexer/numeric-literals.g4
grammar/spec/lexical.md

The diagnostic implementation MUST follow that contract rather than inventing additional numeric rules.

---

22. Invalid Character Literal

Diagnostic:

ZL0011

Examples:

''

or:

'ab'

when the Zamani character literal grammar requires exactly one character or one valid escape.

The lexer MUST distinguish this from an unterminated character literal where possible.

---

23. Invalid Quantum Literal

Diagnostic:

ZL0012

This applies only when source begins a recognized quantum literal form but violates its lexical structure.

Example:

|0

or:

|unknown⟩

if the lexical grammar defines only a finite set of literal state symbols.

Important:

This diagnostic MUST NOT be used for an arbitrary quantum operation name.

For example:

apply my_custom_gate to q;

must not produce a lexical error merely because "my_custom_gate" is not a built-in gate.

Quantum operation identity belongs downstream to semantic resolution and the canonical "quantum::ir" boundary.

---

24. Invalid Annotation

Diagnostic:

ZL0013

Example:

@

when an annotation requires a name.

The lexical layer identifies annotation syntax.

It does not determine whether an annotation is semantically supported.

---

25. Invalid MTS Literal

The current Rust lexer has an explicit:

MTSLiteral

token category.

If MTS lexical syntax remains part of the authoritative language specification, malformed MTS literals require a dedicated diagnostic.

Recommended code:

ZL0014

The exact MTS syntax MUST be specified before the implementation is considered complete.

The diagnostic MUST NOT invent a syntax that differs from:

grammar/spec/
grammar/antlr/
grammar/Zamani.g4

---

26. Unterminated Interpolation

If string interpolation becomes part of the authoritative lexical syntax, use:

ZL0015

Example:

"hello ${name

The diagnostic MUST identify:

- interpolation start;
- missing terminator;
- recovery boundary.

---

27. Invalid Operator

Diagnostic:

ZL0016

Use when a character sequence resembles an operator but cannot form a valid Zamani operator.

Example:

<invalid sequence>

The exact operator inventory is owned by:

grammar/lexer/operators.g4

The diagnostic file must not duplicate that inventory.

---

28. Invalid Delimiter

Diagnostic:

ZL0017

Use for malformed delimiter sequences.

Examples may include an invalid Unicode or ASCII delimiter combination that cannot form a valid lexical construct.

---

29. Conflicting Lexical Construct

Diagnostic:

ZL0018

Use only when two lexical interpretations genuinely conflict and the language specification requires deterministic rejection.

The lexer MUST NOT report a conflict merely because:

- a word is both meaningful in quantum computing and classical computing;
- a name resembles a hardware identifier;
- a library function resembles a keyword.

Semantic overloading is not a lexical conflict.

---

30. Deprecated Lexical Spelling

Diagnostic:

ZL0019

Severity:

warning

Example:

warning[ZL0019]: deprecated lexical spelling

The diagnostic SHOULD provide:

help: use `<replacement>`

Deprecated spellings MUST remain documented in:

grammar/compatibility/deprecated.md

---

31. Future-Reserved Identifier

Diagnostic:

ZL0020

This is applicable only if the language specification explicitly reserves the identifier for future language evolution.

It SHOULD normally be a warning rather than an error unless the compatibility specification says otherwise.

The mechanism prevents future language additions from unexpectedly breaking source code.

---

32. Keyword Used as Identifier

This is primarily a parser/name-resolution issue.

The lexer MUST NOT emit a generic lexical error merely because a keyword occurs where an identifier might later be required.

The parser or semantic layer determines whether that use is valid.

This separation prevents the lexer from becoming coupled to every syntactic context.

---

33. Case Sensitivity

If Zamani is case-sensitive, the lexer MUST preserve source spelling exactly.

For example:

quantum
Quantum
QUANTUM

are distinct lexical spellings unless the specification explicitly says otherwise.

Diagnostics MUST NOT silently normalize case.

---

34. Unicode Normalization

The lexer MUST NOT silently normalize identifiers.

For example, canonically equivalent Unicode sequences must not be silently rewritten unless the language specification explicitly adopts normalization.

If normalization diagnostics are ever required, they must have a separate diagnostic identity and compatibility specification.

---

35. Unicode Confusables

Confusable-character detection is a tooling/security concern rather than ordinary lexical validity.

If enabled, it MUST be an optional warning layer.

It MUST NOT reject valid Unicode identifiers solely because they resemble other identifiers unless a future normative security specification explicitly requires this.

Possible future diagnostic:

ZL0100

Such security diagnostics must not change ordinary lexical grammar.

---

36. Reserved Keyword Compatibility

The current Rust lexer contains a substantial keyword table, including core, quantum, Sankofa, AI/system, OOP, and advanced vocabulary.

The canonical keyword set MUST be reconciled against:

grammar/antlr/ZamaniLexer.g4
src/lexer.rs
grammar/spec/lexical.md
grammar/grammar.md
grammar/Zamani-Grammar.md

A keyword present in one authority but absent from another is a compatibility defect.

The correct response is not to create another keyword table in this file.

Instead, validation MUST report the mismatch.

Recommended diagnostic:

ZL0021 lexical authority mismatch

This is primarily a development/conformance diagnostic.

---

37. Tokenization Mismatch

Diagnostic:

ZL0022

This detects cases where:

ANTLR lexer

and:

Rust lexer

produce incompatible token classifications for the same source.

Example:

source: quantum

ANTLR:

QUANTUM

Rust:

Identifier

This is not necessarily a user-source diagnostic.

It is a compiler conformance diagnostic.

It belongs in validation and CI.

---

38. Token Span Mismatch

Diagnostic:

ZL0023

Used when the token content and source span disagree.

Examples:

- token claims bytes "[10,20)" but contains text from "[11,21)";
- span splits an invalid UTF-8 boundary;
- token end precedes token start.

This MUST be treated as an internal compiler correctness failure, not a user-language error.

---

39. Diagnostic Categories

Diagnostics SHOULD be grouped into:

lexical
encoding
literal
delimiter
escape
comment
identifier
keyword
operator
annotation
quantum-lexical
compatibility
conformance
internal

Category information supports tooling and filtering.

---

40. User Diagnostic vs Compiler Diagnostic

This distinction is mandatory.

User-source diagnostic

Example:

ZL0004 unterminated string literal

The user can correct the source.

Compiler-conformance diagnostic

Example:

ZL0022 lexical authority mismatch

The compiler/repository implementation is inconsistent.

The compiler MUST NOT present internal implementation defects as though the user's source were necessarily invalid.

---

41. Diagnostic Message Stability

Diagnostic codes are more stable than human-readable messages.

The compiler MUST:

- keep codes stable;
- allow message wording improvements;
- avoid tests that require exact full message strings unless necessary;
- prefer testing diagnostic code + span + essential labels.

Golden tests MAY validate complete rendered output where rendering itself is under test.

---

42. Machine-Readable Diagnostic Format

Tooling SHOULD be able to request structured diagnostics.

Conceptually:

{
  "code": "ZL0004",
  "severity": "error",
  "message": "unterminated string literal",
  "file": "main.zm",
  "start": 42,
  "end": 43
}

The actual serialized schema belongs to the compiler tooling contract.

The lexical diagnostic model MUST contain sufficient information to generate such output.

---

43. Human-Readable Rendering

CLI diagnostics SHOULD follow a structure such as:

error[ZL0004]: unterminated string literal
 --> main.zm:12:19
  |
12 | let message = "hello
  |               ^ string literal starts here
  |
  = help: terminate the string with `"`

The renderer, not the lexer, owns terminal formatting.

---

44. No ANSI Codes in Core Diagnostics

The diagnostic data model MUST NOT contain terminal-specific ANSI escape sequences.

Color belongs to the presentation layer.

This ensures compatibility with:

- terminals;
- IDEs;
- CI;
- JSON output;
- language servers;
- log files;
- automated testing.

---

45. Recovery Strategy

Lexical recovery MUST be conservative.

The lexer SHOULD recover only when doing so cannot produce misleading tokenization.

Preferred recovery boundaries include:

- newline;
- delimiter;
- whitespace;
- known token boundary;
- end of file.

The lexer MUST NOT fabricate arbitrary semantic tokens to make parsing continue.

---

46. Error Token Policy

The existing Rust lexer exposes:

TokenType::Illegal

This can remain for compatibility.

However:

Illegal

MUST NOT be the only diagnostic representation.

Every illegal token occurrence SHOULD have an associated structured "LexerError".

Conceptually:

Illegal token
+
LexerError

rather than:

Illegal token

alone.

---

47. Error Collection

The lexer MAY collect multiple independent lexical diagnostics.

The collection MUST be deterministic.

It MUST NOT stop after the first error unless:

- recovery is impossible;
- the caller explicitly requests fail-fast behavior;
- resource exhaustion prevents safe continuation.

---

48. Resource Exhaustion

A critical distinction:

language limit

is not the same as:

implementation resource exhaustion

The lexer MUST NOT introduce artificial lexical limits such as:

MAX_IDENTIFIER_LENGTH = 256
MAX_LITERAL_LENGTH = 1024
MAX_SOURCE_SIZE = 1 MB
MAX_COMMENT_SIZE = 64 KB
MAX_STRING_SIZE = 64 KB

unless such limits are explicitly documented as implementation safeguards rather than language semantics.

Where an implementation cannot continue because available resources are exhausted, it MUST produce a resource diagnostic rather than pretending the source is syntactically invalid.

---

49. Resource Diagnostic

Recommended implementation diagnostic:

ZL0024

Example:

error[ZL0024]: insufficient resources to continue lexical analysis

The diagnostic SHOULD avoid exposing:

- memory addresses;
- secrets;
- internal pointers;
- sensitive filesystem details.

---

50. Cancellation

Lexical analysis MAY support cancellation through the compiler's existing cancellation infrastructure.

Cancellation MUST NOT be reported as a malformed source error.

It should be represented as a compilation/control-flow condition.

If exposed diagnostically:

ZL0025 lexical analysis cancelled

This should normally be non-user-source-facing.

---

51. Determinism

Given identical:

- source bytes;
- language version;
- dialect configuration;
- compatibility configuration;
- compiler configuration;

the lexer MUST produce equivalent:

- tokens;
- spans;
- diagnostic codes;
- diagnostic severity;
- diagnostic ordering.

Hardware availability MUST NOT alter lexical diagnostics.

CPU count MUST NOT alter lexical diagnostics.

GPU availability MUST NOT alter lexical diagnostics.

QPU availability MUST NOT alter lexical diagnostics.

Network topology MUST NOT alter lexical diagnostics.

---

52. POCO-REAF Requirement

The lexical diagnostic layer is part of Zamani's:

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever

architecture.

Therefore lexical diagnostics MUST remain target-independent.

The same source:

apply H to q;

must not lex differently because the eventual target is:

CPU simulator
GPU simulator
FPGA
ASIC
QPU
distributed machine
future architecture

Target-specific diagnostics belong downstream.

---

53. Quantum Diagnostic Boundary

The lexer may diagnose malformed quantum syntax.

It must not diagnose semantic hardware conditions.

For example:

apply H to q;

is lexically valid.

The lexer must not report:

device does not support H

That belongs to:

- semantic analysis;
- capability analysis;
- quantum lowering;
- HAL;
- target validation.

Likewise, the lexer must not report:

too many qubits

because no universal machine limit exists at lexical level.

---

54. Hardware Diagnostic Boundary

The lexer may tokenize:

requires capability("quantum.measurement");

It must not determine whether that capability exists.

The later resource/capability system owns that decision.

---

55. HDL Diagnostic Boundary

The lexer may recognize HDL syntax.

It must not report:

FPGA has insufficient LUTs

or:

ASIC cannot implement this timing requirement

Those are hardware/resource/implementation diagnostics.

---

56. Distributed Diagnostic Boundary

The lexer may tokenize:

distributed
node
service
channel
replicate

It must not validate:

- number of nodes;
- physical network topology;
- available machines;
- cluster capacity.

Those belong to later layers.

---

57. AI Diagnostic Boundary

The lexer may tokenize AI-related language constructs.

It must not validate:

- tensor dimensions;
- accelerator availability;
- training memory;
- model convergence;
- numerical stability.

Those belong downstream.

---

58. Diagnostic Provenance

Every diagnostic SHOULD carry phase provenance:

lexical

This allows the complete compiler to distinguish:

lexical
parser
AST
semantic
type
effect
resource
IR
optimization
routing
scheduling
runtime
target

The lexer MUST identify itself as the lexical phase.

---

59. No Backend Leakage

Diagnostic messages emitted by the lexer MUST NOT mention backend details unless the lexical source itself explicitly contains a backend-specific literal whose syntax is invalid.

Bad:

error: NVIDIA GPU does not support token

Correct:

error[ZL0002]: illegal character `...`

Backend diagnostics belong downstream.

---

60. No Physical Qubit Diagnostics

The lexical layer MUST NOT report:

physical qubit 17 unavailable

maximum qubits exceeded

QPU topology invalid

gate cannot be routed

These belong to quantum semantic lowering, routing, scheduling, HAL, or runtime.

---

61. Keyword Diagnostics and Extensibility

Zamani is intended to support future computing domains.

Therefore the lexer should minimize unnecessary reserved words.

A domain operation should remain an identifier when the parser can represent it generically.

For example:

H
X
CNOT
custom_gate
future_gate
vendor_operation

should not require the lexer to be updated merely because a new quantum operation is introduced.

This is essential for long-term language extensibility.

---

62. Keyword Table Conformance

The current Rust implementation has a large "keywords_map".

That map MUST be treated as an implementation artifact derived from the canonical lexical specification.

It MUST NOT silently become an independent source of truth.

CI should verify:

canonical keyword specification
        ==
ANTLR keyword vocabulary
        ==
Rust keyword mapping

apart from explicitly documented compatibility aliases.

---

63. Compatibility Aliases

When two lexical spellings are intentionally supported:

old_name
new_name

the compatibility system MUST document:

- canonical spelling;
- legacy spelling;
- version introduced;
- version deprecated;
- diagnostic behavior;
- removal policy.

The lexer should classify both correctly while the diagnostic layer may emit a deprecation warning.

---

64. Generated Lexer Compatibility

If ANTLR generates the canonical lexer, generated files MUST NOT be manually modified to implement diagnostic policy.

Diagnostic behavior belongs in:

- grammar;
- lexer specification;
- generated-lexer integration;
- Rust wrapper/adapter;
- diagnostic subsystem.

Generated output is derived.

---

65. Rust Safety Contract

The Zamani compiler implementation MUST use safe Rust.

The lexical diagnostic implementation MUST NOT use:

unsafe

or:

unsafe {}

or unsafe FFI merely for diagnostic generation.

Safe standard-library mechanisms, owned values, references, "Arc", "Result", "Option", and normal collection types are sufficient for this subsystem.

---

66. Panic Policy

Malformed user source MUST NOT cause a Rust panic.

The lexer MUST convert expected malformed input into diagnostics.

Examples:

- invalid character;
- malformed number;
- unterminated string;
- malformed escape;
- malformed comment.

"panic!" is not an acceptable user-source error mechanism.

Internal invariants may be asserted during development, but production user-input paths must remain robust.

---

67. Integer Overflow in Diagnostic Positions

Source positions MUST use the repository's source-map position types.

Diagnostic calculations MUST avoid unchecked arithmetic.

If a source position cannot be represented by the implementation's position type, the implementation must fail deterministically rather than wrap silently.

No fixed source-size semantic limit should be introduced merely to simplify arithmetic.

---

68. Memory Safety

The lexer MUST NOT retain unnecessary copies of entire source fragments merely to create diagnostics.

Where practical:

source span
+
source map

should identify the relevant text.

Diagnostic rendering may materialize snippets when required.

This supports large programs without unnecessary memory multiplication.

---

69. Large Source Files

The diagnostic design MUST work for source files ranging from:

tiny

to:

very large

subject only to available implementation resources.

The diagnostic model must not assume:

line count fits u16
column fits u16
token count fits u32
literal length fits u16

unless the repository's canonical source-map contract explicitly establishes a representation.

---

70. Large Literals

A malformed large literal MUST produce a normal diagnostic.

The lexer must not need to copy the entire literal into the error message.

Preferred:

error[ZL0009]: invalid numeric literal

with a source span covering the literal.

Avoid embedding potentially enormous source content into diagnostics.

---

71. Large Identifiers

Identifier diagnostics MUST use spans rather than copying arbitrarily large identifier text into messages.

For example:

error[ZL0020]: identifier uses a reserved spelling

rather than printing a potentially enormous identifier.

---

72. Secrets

Diagnostics MUST NOT accidentally expose secrets.

The lexer must not echo complete source literals when they may contain:

- credentials;
- cryptographic material;
- access tokens;
- private keys;
- sensitive strings.

Diagnostics should prefer source spans and concise excerpts.

---

73. Source Snippet Policy

Diagnostic renderers may display source snippets.

The diagnostic core should not require storing the snippet itself.

This preserves:

- memory efficiency;
- privacy;
- deterministic storage;
- separation between diagnostics and presentation.

---

74. Multi-File Diagnostics

A lexical diagnostic is associated with one source unit.

If an included/imported/generated source unit causes a lexical error, the diagnostic must identify the actual source "FileId".

Cross-file relationships belong in related diagnostics.

---

75. Generated Source

If macros or metaprogramming generate source, diagnostics must distinguish:

original source

from:

generated source

Source mapping belongs to the compiler source-map layer.

The lexical diagnostic itself should retain its actual source span.

---

76. Macro Interaction

Macro-generated lexical errors must not be confused with errors in the macro invocation source.

The eventual diagnostic system should be able to represent:

generated lexical error

with related provenance:

generated from macro invocation at ...

The lexer itself remains unaware of macro semantics.

---

77. Dialect Interaction

Dialect-specific lexical diagnostics must use explicit dialect configuration.

A dialect MUST NOT silently modify the core lexer.

Diagnostics should identify the active dialect when relevant.

Example:

error[ZL0002]: character is not valid in the active lexical dialect

---

78. Feature Gates

If a lexical feature is experimental:

experimental_feature

the diagnostic system may report:

warning

or:

error

depending on the feature-gate contract.

Feature-gate diagnostics belong to compatibility/specification policy rather than hard-coded lexer behavior.

---

79. Diagnostic Configuration

The lexer SHOULD support configuration for:

- language version;
- dialect;
- compatibility mode;
- warning policy;
- diagnostic verbosity;
- recovery mode.

It MUST NOT receive hardware topology merely to tokenize source.

---

80. Warning Policy

Warning promotion:

warning -> error

belongs to the compiler diagnostic configuration.

The lexical grammar itself should not change because warnings are denied.

---

81. Suppression

Diagnostic suppression, if supported, belongs to the compiler/tooling diagnostic framework.

The lexical layer MUST NOT silently suppress errors.

Source-level suppression directives must themselves have formally defined lexical and semantic behavior.

---

82. Testing Contract

Every diagnostic MUST have tests.

Each diagnostic class should include:

positive
negative
boundary
Unicode
large-input
recovery
determinism
compatibility

where applicable.

---

83. Required Lexical Diagnostic Test Categories

The test suite must cover:

tests/diagnostics/lexer/

or the repository's established equivalent.

Required groups:

illegal-character/
unterminated-string/
unterminated-character/
invalid-escape/
invalid-unicode/
unterminated-comment/
numeric/
quantum/
annotation/
mts/
operators/
unicode/
keywords/
compatibility/
spans/
ordering/
recovery/
resource/
determinism/

Do not create duplicate test hierarchies if an existing repository test structure already provides these categories; integrate with it.

---

84. Positive Tests

Positive tests verify that valid source produces:

- no lexical errors;
- correct tokens;
- correct spans.

Examples:

let x = 42;

apply H to q;

let π = 3.14159;

let name = "Zamani";

---

85. Negative Tests

Negative tests verify:

"unterminated

0x

"\q"

/*

and equivalent malformed constructs.

Each test should assert:

- diagnostic code;
- severity;
- relevant span;
- recovery behavior where specified.

---

86. Boundary Tests

Boundary tests include:

- empty source;
- one-character source;
- source ending immediately after an opening delimiter;
- source ending after escape prefix;
- source ending after numeric exponent;
- source ending after Unicode escape;
- maximum representable implementation position;
- very long valid identifier;
- very long valid literal.

The tests must not establish artificial language limits.

---

87. Unicode Tests

Required tests include:

- Unicode identifiers;
- combining characters;
- non-ASCII source text;
- Unicode quantum notation;
- invalid Unicode escape;
- Unicode delimiter handling;
- source-span correctness for multi-byte characters.

---

88. Quantum Tests

Required examples include:

|0⟩
|1⟩
|+⟩
|-⟩

and malformed equivalents.

Also verify that:

H
X
CNOT
U
custom_gate
vendor.operation

are not rejected merely because they are not enumerated in the lexer.

---

89. No Hardware-Scaling Test

The lexer test suite MUST verify that lexical validity does not depend on:

qubit count
CPU count
GPU count
FPGA count
node count
memory capacity
tensor dimensions
device count
topology size

Example:

allocate qubits[n];

must not be rejected lexically because "n" has no fixed machine bound.

---

90. Determinism Tests

Run the same source repeatedly.

Assert:

same tokens
same spans
same diagnostics
same ordering

No random values should appear in diagnostics.

No memory addresses should appear.

No thread IDs should appear.

No hardware identifiers should appear.

---

91. Conformance Tests

The repository must eventually compare:

grammar/antlr/ZamaniLexer.g4

against:

src/lexer.rs

for representative lexical fixtures.

A source fixture should produce equivalent lexical classifications.

The test must distinguish intentional compatibility differences from defects.

---

92. Authority Mismatch Tests

CI should detect:

keyword in ANTLR but absent in Rust
keyword in Rust but absent in ANTLR
operator mismatch
literal mismatch
comment mismatch
identifier mismatch
quantum literal mismatch

These are implementation conformance failures.

---

93. Grammar-to-Rust Integration

The integration contract is:

grammar/lexer/
        │
        ▼
grammar/antlr/ZamaniLexer.g4
        │
        ▼
canonical lexical behavior
        │
        ├────────► src/lexer.rs
        │
        ▼
token contract
        │
        ▼
src/parser.rs

The modular files under "grammar/lexer/" MUST NOT be treated as an independent runtime lexer.

---

94. "src/lexer.rs" Integration Requirements

"src/lexer.rs" MUST:

1. preserve existing public compatibility where practical;
2. use the canonical diagnostic codes;
3. associate errors with "Span";
4. avoid unsafe Rust;
5. avoid panics for malformed source;
6. keep deterministic ordering;
7. preserve Unicode correctness;
8. distinguish lexical errors from implementation failures;
9. avoid hard-coded hardware limits;
10. remain target-independent.

---

95. "TokenType" Integration

The existing "TokenType" enumeration contains a broad lexical vocabulary.

The diagnostic system does not require every token to receive a dedicated diagnostic.

Diagnostics describe failures to produce valid tokens.

Token taxonomy belongs to:

grammar/lexer/tokens.g4

and:

grammar/antlr/ZamaniLexer.g4

The Rust "TokenType" must remain synchronized through conformance tests.

---

96. "keywords_map" Integration

The current implementation uses a "HashMap<String, TokenType>" for keyword lookup.

The diagnostic system must not create another keyword map.

The canonical lexical specification determines which words are reserved.

The Rust map is an implementation of that contract.

Any mismatch is a conformance failure.

---

97. Error API Integration

The current implementation exposes:

LexerError {
    message,
    span
}

This should be evolved, without unnecessary public breakage, toward a diagnostic structure that can carry at least:

code
severity
message
span

Additional fields may be introduced compatibly.

If changing the public structure is necessary, the migration must be documented in:

grammar/compatibility/

and the corresponding Rust API documentation.

---

98. No Duplicate Diagnostic Systems

Do not create independent:

ANTLR diagnostic model
Rust lexer diagnostic model
parser diagnostic model
AST diagnostic model

that cannot interoperate.

There should be one compiler-wide diagnostic model with phase-specific codes.

The lexer owns the lexical subset.

---

99. Diagnostic Namespace

Recommended namespaces:

ZLxxxx   Lexer
ZPxxxx   Parser
ZAxxxx   AST/structural
ZSxxxx   Semantic
ZTxxxx   Type
ZE####   Effect
ZR####   Resource
ZI####   IR
ZC####   Compiler
ZX####   Execution/target

Exact namespaces should be centralized in the repository's diagnostics specification.

This file owns only "ZL".

---

100. Internal Diagnostic Codes

Compiler-internal failures should be distinguishable from source errors.

For example:

ZLIxxx

may be reserved for lexer implementation invariants.

These must never be presented as ordinary user-source errors.

---

101. Diagnostic Versioning

Diagnostic codes are API-like contracts.

Changing:

ZL0004

to another code without compatibility documentation is a breaking tooling change.

Diagnostic messages may evolve more freely.

---

102. Localization

The canonical diagnostic representation should use stable English diagnostic identifiers and semantic messages.

Localization, if introduced, belongs to the renderer.

Diagnostic codes MUST remain language-independent.

---

103. IDE Integration

IDE tooling should be able to obtain:

- code;
- severity;
- source range;
- message;
- related locations;
- help;
- phase.

The lexer should not contain IDE-specific rendering logic.

---

104. Formatter Integration

The formatter may use lexical diagnostics to determine whether source is safely formattable.

Malformed input must not cause the lexer to modify the source.

The formatter decides whether partial formatting is safe.

---

105. Syntax Highlighting Integration

Syntax highlighting may consume tokens even when diagnostics exist.

Recovery tokens may be useful for highlighting, but they must not be interpreted as semantic source.

---

106. Language Server Integration

A language server can expose lexer diagnostics directly.

The diagnostic contract therefore must remain:

- deterministic;
- span-accurate;
- structured;
- machine-readable.

---

107. Documentation Integration

The diagnostic codes should eventually be documented in:

grammar/reference/diagnostics.md

and indexed from:

grammar/README.md
grammar/spec/diagnostics.md

This document remains the lexical diagnostic authority.

---

108. Compatibility Integration

Lexical diagnostic changes must be recorded in:

grammar/compatibility/versions.md
grammar/compatibility/migrations.md
grammar/compatibility/deprecated.md

Examples requiring compatibility consideration:

- changing a warning to an error;
- changing a token classification;
- removing a keyword;
- changing numeric literal syntax;
- changing escape rules;
- changing Unicode identifier rules.

---

109. Grammar Integration

The following relationship is mandatory:

grammar/spec/lexical.md
        │
        ▼
grammar/lexer/*.g4
        │
        ▼
grammar/antlr/ZamaniLexer.g4
        │
        ▼
generated lexer
        │
        ▼
src/lexer.rs compatibility/conformance

This file defines diagnostics for the resulting lexical contract.

---

110. "grammar.md" Integration

"grammar/grammar.md" must reflect the lexical behavior actually implemented.

It must not independently invent diagnostic behavior.

If it lists lexical forms, those forms must trace back to the canonical lexical authority.

---

111. "Zamani-Grammar.md" Integration

"grammar/Zamani-Grammar.md" may contain proposed or historical lexical concepts.

Such concepts are not automatically diagnostic requirements.

Only promoted/stable lexical features become mandatory here.

---

112. Specification Status

Every lexical feature should have one of:

stable
experimental
proposed
deprecated
historical

Diagnostics must follow the status.

Historical syntax should not silently become valid production syntax.

---

113. Error Recovery and Parser Synchronization

When lexical recovery is possible, recovery must preserve parser synchronization.

For example, after an invalid character:

let x = §;
let y = 2;

the lexer should, when safe, continue far enough for the parser to see the second statement.

The exact recovery algorithm belongs to the implementation.

This document defines the observable diagnostic contract.

---

114. Recovery Must Not Hide Errors

Recovery does not mean accepting malformed source silently.

For every recovered lexical error:

diagnostic emitted

must remain true.

---

115. Multiple Errors in One Token

When multiple lexical defects overlap, the lexer should report the most specific primary diagnostic.

Example:

"\uGGGG

should preferably report:

invalid Unicode escape

rather than only:

unterminated string

Additional related diagnostics may be emitted when useful, but diagnostic floods should be avoided.

---

116. Diagnostic Deduplication

The same lexical defect should not be reported repeatedly by independent recovery passes.

Diagnostic deduplication must be deterministic.

---

117. Diagnostic Limits

The compiler may have a configurable diagnostic reporting limit for resource protection.

Such a limit is an implementation policy.

It MUST NOT become a language restriction.

For example:

maximum 1000 displayed diagnostics

does not mean:

Zamani source may contain only 1000 errors.

---

118. Streaming and Incremental Lexing

The diagnostic contract must permit future incremental/streaming lexing.

Diagnostics must therefore not depend on:

- whole-program global state;
- fixed source size;
- fixed token count;
- fixed line count.

Incremental lexing may invalidate diagnostics in affected ranges and regenerate them.

---

119. Thread Safety

The lexical diagnostic representation should not require global mutable state.

Parallel lexical analysis of independent source files should be possible.

Diagnostic merging must be deterministic.

---

120. No Hardware-Dependent Diagnostics

This requirement is absolute.

Lexical diagnostics must not vary based on:

CPU
GPU
FPGA
ASIC
QPU
RAM
VRAM
network
cluster
device
topology

The lexer sees source.

Later layers see target resources.

---

121. No Hard-Coded Language Limits

This document must never establish limits such as:

maximum identifiers
maximum source length
maximum token count
maximum qubits
maximum devices
maximum nodes
maximum tensor dimensions
maximum timelines
maximum quantum operations

Any implementation resource limit must remain explicitly categorized as an implementation/resource concern.

---

122. Security

Diagnostics must avoid:

- secret disclosure;
- arbitrary source dumping;
- filesystem path leakage beyond configured policy;
- memory addresses;
- internal object identifiers;
- credentials;
- private cryptographic material.

---

123. Deterministic Security Behavior

Security-sensitive diagnostics must remain deterministic.

The compiler must not include random identifiers in diagnostic output unless they are explicitly required for external correlation and are kept outside semantic diagnostic identity.

---

124. Performance

Diagnostic generation should avoid unnecessary allocations.

Preferred architecture:

source span
    +
diagnostic code
    +
small structured metadata

rather than eagerly copying source substrings.

Large source snippets should be generated only by presentation layers.

---

125. Rust 1.97 / 1.97.1 Requirements

The implementation must compile under the repository's declared Rust baseline:

Rust 1.97
Rust 1.97.1

No feature requiring a newer Rust version may be introduced without an explicit repository-wide version change.

The lexical diagnostic implementation must remain entirely safe Rust.

---

126. Completion Contract

"grammar/lexer/diagnostics.md" is complete only when all of the following are true:

- lexical diagnostic ownership is explicit;
- diagnostic namespace is defined;
- severity is defined;
- source spans are defined;
- Unicode behavior is defined;
- illegal characters are defined;
- string errors are defined;
- character errors are defined;
- escape errors are defined;
- Unicode escape errors are defined;
- comment errors are defined;
- numeric errors are defined;
- annotation errors are defined;
- quantum lexical errors are defined;
- MTS behavior is defined;
- compatibility diagnostics are defined;
- conformance diagnostics are defined;
- recovery is defined;
- deterministic ordering is defined;
- machine-readable structure is defined;
- human rendering is separated;
- Rust integration is defined;
- ANTLR integration is defined;
- parser integration is defined;
- AST integration is defined;
- tooling integration is defined;
- compatibility integration is defined;
- tests are defined;
- scalability requirements are defined;
- hard-coding audit is passed;
- unsafe Rust is prohibited;
- no backend assumptions exist.

---

127. File Independence Contract

This file is independently completable.

It does not require another future file to decide:

- what constitutes a lexical diagnostic;
- what information a lexical diagnostic carries;
- how lexical diagnostics are identified;
- how spans are represented conceptually;
- how lexical diagnostics differ from parser/semantic diagnostics;
- how diagnostics integrate with the Rust lexer;
- how diagnostics integrate with the canonical ANTLR lexer;
- how diagnostic determinism works;
- how scalability is handled.

Other files may refine their own responsibilities, but they must not invalidate this lexical diagnostic contract without an explicit specification/compatibility change.

---

128. Upstream Contracts

This file depends on:

grammar/spec/lexical.md
grammar/spec/source-spans.md
grammar/spec/diagnostics.md
grammar/lexer/README.md
grammar/antlr/ZamaniLexer.g4

where available.

It also integrates with:

src/source_map.rs
src/lexer.rs

The existing repository architecture must remain the implementation source for concrete Rust types.

---

129. Downstream Consumers

This contract is consumed by:

src/parser.rs
src/frontend/ast/
semantic analysis
compiler diagnostics
CLI
IDE tooling
formatter
syntax highlighter
language-server tooling
tests
compatibility tooling

It does not directly integrate with:

quantum::ir
QEC
ZQN
HAL
routing
scheduling
calibration
runtime

Those systems receive later semantic/IR diagnostics.

---

130. Integration Matrix

Concern| Lexer diagnostics| Owner
Illegal character| Yes| Lexer
Invalid string| Yes| Lexer
Invalid escape| Yes| Lexer
Invalid number| Yes| Lexer
Unicode source| Yes| Lexer/source map
Keyword classification| Conformance| Lexer/spec
Parser expectation| No| Parser
Type mismatch| No| Type system
Missing symbol| No| Name resolution
Quantum gate availability| No| Capability/HAL
Qubit topology| No| Routing/HAL
QEC feasibility| No| QEC
Scheduling feasibility| No| Scheduler
Hardware capacity| No| Resource analysis
Runtime failure| No| Runtime
Backend failure| No| Backend

---

131. Hard-Coding Audit

This file contains no language-level hardware limits.

The following are explicitly prohibited as lexical diagnostic conditions:

MAX_QUBITS
MAX_CPUS
MAX_GPUS
MAX_FPGAS
MAX_ASICS
MAX_NODES
MAX_MEMORY
MAX_VRAM
MAX_REGISTER_WIDTH
MAX_TENSOR_DIM
MAX_TIMELINES
MAX_THREADS
MAX_DEVICES
MAX_NETWORK_SIZE

A diagnostic must never state that a lexical construct is invalid merely because a particular machine cannot execute it.

---

132. Final Architectural Contract

The Zamani lexer must answer:

«Is this sequence of source characters lexically valid Zamani under the active language/version/dialect contract?»

It must not answer:

«Can this program run on today's machine?»

The complete architecture remains:

Zamani source
      │
      ▼
Unicode/source decoding
      │
      ▼
canonical lexical grammar
      │
      ▼
lexer
      │
      ├── lexical diagnostics
      │
      ▼
parser
      │
      ├── syntax diagnostics
      │
      ▼
AST
      │
      ├── structural diagnostics
      │
      ▼
semantic analysis
      │
      ├── type/effect/resource/capability diagnostics
      │
      ▼
canonical semantic representation
      │
      ├── quantum::ir
      ├── classical/data/control representations
      └── HDL/hardware representations
      │
      ▼
optimization
      │
      ▼
routing / scheduling / resilience / QEC / ZQN
      │
      ▼
HAL / target realization
      │
      ▼
actual hardware/runtime

The lexical diagnostic system is therefore deliberately small in responsibility but foundational in correctness.

Its purpose is not to know every possible future computer.

Its purpose is to ensure that the same Zamani source receives precise, deterministic, target-independent lexical treatment regardless of whether that source eventually targets an atom-scale device, a classical machine, a quantum processor, an FPGA/ASIC, a distributed system, an accelerator, a hybrid system, or a future computational architecture.

Production readiness criterion:

One canonical lexical contract
        +
one deterministic diagnostic model
        +
precise source spans
        +
ANTLR/Rust conformance
        +
safe Rust 1.97/1.97.1
        +
no artificial resource limits
        +
complete negative/boundary/scalability tests
        =
production-ready lexical diagnostics