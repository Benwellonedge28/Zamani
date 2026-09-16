Zamani Lexer Conformance

Path: "grammar/lexer/conformance.md"
Status: Normative
Authority: Lexer conformance contract
Language: Zamani
Rust implementation baseline: Rust 1.97 / Rust 1.97.1
Rust safety requirement: No "unsafe"
Scope: Lexical specification, lexer implementation, ANTLR lexical grammar, diagnostics, source spans, parser/token consumers, tests, compatibility, and tooling

---

1. Purpose

This document defines the production conformance contract for the Zamani lexer.

The lexer is the boundary between raw source text and the token stream consumed by the parser.

It must guarantee that:

source bytes / source text
        │
        ▼
input validation
        │
        ▼
Unicode / source decoding
        │
        ▼
lexical analysis
        │
        ▼
tokens + source spans + lexical diagnostics
        │
        ▼
parser
        │
        ▼
domain-neutral AST

The lexer must be:

- deterministic;
- reproducible;
- Unicode-aware;
- source-span accurate;
- incrementally recoverable;
- scalable;
- resource-aware;
- independent of target hardware;
- independent of runtime topology;
- independent of quantum-device size;
- independent of compiler backend;
- independent of vendor APIs;
- compatible with the canonical Zamani grammar;
- compatible with the Rust implementation;
- compatible with the frontend AST;
- compatible with diagnostics and tooling.

The lexer must never introduce artificial language limits that prevent POCO-REAF:

«Program Once → Compile Once → Run Everywhere, Anywhere, Forever»

subject only to actual implementation resources, explicitly supplied resource budgets, language-version rules, and target capabilities.

---

2. Normative status

The requirements in this document are normative unless explicitly marked:

- Informative
- Recommended
- Implementation Note
- Future Extension

Normative terms:

- MUST — mandatory.
- MUST NOT — prohibited.
- SHOULD — required unless a documented reason exists not to do so.
- SHOULD NOT — normally prohibited unless justified.
- MAY — permitted but optional.

A lexer implementation is not production-conformant merely because it can tokenize valid examples.

It is conformant only when it satisfies the complete contract in this document.

---

3. File completion contract

File

"grammar/lexer/conformance.md"

Purpose

Define the complete observable contract by which all Zamani lexical implementations and specifications are judged equivalent.

Owns

This file owns:

- lexical conformance criteria;
- lexer/specification agreement;
- lexer/ANTLR agreement;
- lexer/parser token agreement;
- source-span conformance;
- diagnostic conformance;
- lexical recovery conformance;
- determinism requirements;
- scalability requirements;
- compatibility requirements;
- lexer test requirements;
- implementation-safety requirements;
- conformance reporting.

Does not own

This file does not own:

- AST semantics;
- type checking;
- name resolution;
- resource allocation;
- hardware discovery;
- quantum routing;
- quantum scheduling;
- QEC;
- ZQN fault/noise semantics;
- HAL implementation;
- compiler optimization;
- runtime execution;
- target-specific lowering.

Those remain owned by their respective repository components.

---

4. Authority hierarchy

The lexer must not have multiple independent sources of truth.

The intended authority relationship is:

language specification
        │
        ▼
grammar/spec lexical contracts
        │
        ▼
grammar/lexer lexical contracts
        │
        ├───────────────┐
        ▼               ▼
Zamani.g4          Rust lexer
        │               │
        └───────┬───────┘
                ▼
        conformance tests
                │
                ▼
        parser / AST contracts

The following roles are distinct.

"grammar/specification/lexical.md"

Human-readable normative language specification.

"grammar/spec/lexical.md"

Formal lexical contract.

"grammar/lexer/*.md"

Detailed lexical contracts.

"grammar/Zamani.g4"

Canonical ANTLR grammar composition root.

"src/lexer.rs"

Executable Rust lexer implementation.

"grammar/grammar.md"

Implementation-conformance reference describing what the current implementation accepts.

"grammar/Zamani-Grammar.md"

Historical/design/aspirational reference.

It MUST NOT silently introduce syntax that is absent from the normative lexical specification.

---

5. Single lexical authority

There MUST be one lexical model.

The following must agree:

keywords
operators
delimiters
identifiers
numeric literals
string literals
character literals
quantum literals
comments
interpolation
directives
whitespace
Unicode handling
source spans
diagnostics

No domain directory may independently redefine these.

For example:

grammar/quantum/
grammar/ai/
grammar/hdl/
grammar/networking/
grammar/security/

MUST NOT create competing definitions of:

- identifier;
- integer literal;
- string;
- comment;
- whitespace;
- Unicode identifier;
- operator;
- delimiter.

Domain grammars consume the common lexical vocabulary.

---

6. Canonical lexical pipeline

The production lexer follows:

Input
  │
  ├── source bytes
  │
  ▼
Input decoding
  │
  ├── valid UTF-8
  │
  └── malformed UTF-8 diagnostic
  │
  ▼
Unicode-aware scanning
  │
  ▼
Trivia recognition
  │
  ├── whitespace
  ├── comments
  └── directives where applicable
  │
  ▼
Token recognition
  │
  ├── identifiers
  ├── keywords
  ├── literals
  ├── operators
  └── delimiters
  │
  ▼
Source-span assignment
  │
  ▼
Lexical diagnostics
  │
  ▼
Token stream
  │
  ▼
Parser

The lexer MUST NOT perform semantic analysis during lexical scanning.

---

7. Lexical versus syntactic responsibility

The boundary must remain explicit.

Lexer owns

Examples:

- invalid character;
- malformed UTF-8;
- malformed numeric literal;
- invalid escape;
- unterminated string;
- unterminated character literal;
- unterminated comment;
- invalid identifier spelling;
- invalid lexical operator;
- malformed lexical interpolation delimiter;
- malformed lexical quantum literal.

Parser owns

Examples:

- missing expression;
- invalid declaration structure;
- invalid statement sequence;
- unexpected token in a grammar position;
- missing closing delimiter when the lexer successfully tokenized the delimiter;
- invalid operator placement.

Semantic analysis owns

Examples:

- type mismatch;
- undefined name;
- invalid resource requirement;
- unavailable capability;
- invalid quantum operation semantics;
- invalid hardware requirement;
- illegal effect combination.

Compiler/runtime owns

Examples:

- unavailable target;
- insufficient physical resources;
- failed device capability negotiation;
- scheduling failure;
- backend limitation;
- runtime resource exhaustion.

A lexer MUST NOT emit a semantic diagnostic merely because a construct is large, unusual, quantum-specific, hardware-specific, or computationally expensive.

---

8. Token identity

Every token must have a stable identity.

A token consists conceptually of:

Token {
    kind
    source_span
    lexical_value
    optional raw representation
}

The exact Rust representation may follow the existing implementation architecture, but its observable behavior MUST satisfy this contract.

Token identity MUST NOT depend on:

- memory address;
- hash-map iteration order;
- machine architecture;
- CPU count;
- GPU count;
- QPU count;
- physical qubit numbering;
- thread count;
- compiler optimization level.

---

9. Token taxonomy

The lexical system must provide categories for:

EOF
identifier
keyword
integer literal
floating literal
character literal
string literal
raw string literal
interpolated string component
boolean literal where applicable
null/unit literal where applicable
quantum literal
operator
delimiter
punctuation
directive/pragma token where applicable

Additional token categories MAY be added only through the lexical specification.

Domain-specific tokens must not duplicate universal tokens unnecessarily.

---

10. Keywords

Keyword recognition is governed by:

"grammar/lexer/keywords.md"

The keyword registry MUST be centralized.

A keyword MUST NOT be independently declared by:

quantum/
hdl/
ai/
networking/
security/

without being registered through the common lexical authority.

Contextual keywords SHOULD remain lexically representable as identifiers when their interpretation depends on parser context.

This preserves extensibility and forward compatibility.

---

11. Identifier conformance

Identifier behavior MUST conform to:

- "grammar/lexer/identifiers.md"
- "grammar/lexer/unicode.md"
- "grammar/spec/lexical.md"
- "grammar/specification/lexical.md"

Identifiers must:

- have deterministic classification;
- preserve their source representation;
- have accurate spans;
- support the specified Unicode identifier policy;
- reject invalid identifier characters;
- avoid silent lossy conversion.

The lexer MUST NOT silently rewrite an identifier unless normalization is explicitly part of the language specification.

---

12. Unicode conformance

The lexer MUST treat Unicode as a first-class language concern.

It must correctly handle:

- Unicode scalar values;
- Unicode whitespace where specified;
- Unicode identifiers where specified;
- Unicode operators where specified;
- Unicode punctuation where specified;
- Unicode source spans.

The lexer MUST NOT assume ASCII-only source.

The lexer MUST NOT assume that one Unicode scalar value occupies one byte.

Rust implementation must use safe UTF-8-aware APIs.

---

13. UTF-8 conformance

If the lexer API accepts Rust "&str", the input is already valid UTF-8.

If a byte-oriented API exists, malformed UTF-8 MUST be detected before interpreting malformed byte sequences as language characters.

Malformed input MUST NOT be silently replaced with U+FFFD unless such replacement is explicitly specified as part of a separate preprocessing contract.

The preferred behavior is:

invalid bytes
     │
     ▼
LEX002 InvalidUtf8
     │
     ▼
safe recovery

No "unsafe" UTF-8 conversion is permitted.

---

14. Source spans

All emitted tokens MUST have source locations.

The canonical span model must agree with:

- "grammar/spec/source-spans.md";
- "grammar/validation/source-spans.md";
- frontend AST source-span contracts;
- parser diagnostics.

The preferred canonical representation is a half-open range:

[start, end)

where:

start <= end

and offsets are measured against the canonical source representation.

Byte offsets are preferred as the canonical internal coordinate because Rust strings are UTF-8.

Human-facing:

- line;
- column;
- display width;

must be derived from the source map rather than stored as the authoritative lexical identity.

---

15. Span invariants

Every token span MUST satisfy:

start <= end

and:

end <= source_length

for valid source coordinates.

Adjacent tokens MUST have deterministic ordering.

The lexer MUST NOT:

- produce negative positions;
- produce reversed spans;
- produce spans outside the source;
- assign arbitrary spans;
- lose the location of an invalid token.

EOF MUST have a deterministic source location/span according to the source-span specification.

---

16. Trivia

The lexer must define whether whitespace and comments are:

- discarded;
- preserved;
- attached as trivia;
- exposed through a separate token stream.

The decision MUST be consistent across:

- lexer;
- parser;
- formatter;
- IDE/LSP tooling;
- source-preserving transformations;
- macro/token-tree systems.

If trivia is discarded from the parser token stream, source positions MUST remain available independently.

---

17. Whitespace

Whitespace is lexical input.

The lexer must explicitly define:

- horizontal whitespace;
- line breaks;
- Unicode whitespace;
- carriage-return/line-feed behavior;
- form-feed behavior if supported;
- source-file boundary behavior.

Whitespace MUST NOT accidentally become a semantic hardware or execution constraint.

---

18. Comments

Comment behavior is defined by:

"grammar/lexer/comments.md"

The lexer must support all comment forms declared there.

Every comment form must define:

opening syntax
closing syntax
nesting behavior
newline behavior
EOF behavior
span behavior
recovery behavior

An unterminated comment MUST produce a deterministic lexical diagnostic.

The lexer MUST avoid diagnostic amplification such as producing one error for every character after the missing terminator.

---

19. Numeric literals

Numeric literal conformance is defined by:

- "grammar/lexer/numeric-literals.md"
- "grammar/lexer/literals.md"

The lexer must support the complete specified numeric syntax, including applicable:

- decimal;
- binary;
- octal;
- hexadecimal;
- separators;
- decimal fractions;
- exponents;
- signed/exponent forms;
- suffixes;
- arbitrary literal widths;
- future extensible literal forms.

The lexer MUST distinguish:

lexical validity

from:

numeric representability

For example, the lexer MUST NOT reject a syntactically valid integer solely because it does not fit in:

i32
i64
u64

or another implementation-selected machine type.

Representability belongs to semantic/type analysis.

This is essential for scalable computing.

---

20. Numeric overflow

A source literal may be lexically valid while being too large for a selected semantic type.

Therefore:

123456789012345678901234567890...

MUST NOT be rejected merely because the lexer implementation happens to use a smaller native integer type internally.

The lexer should preserve the literal representation or use an arbitrary-precision-safe representation appropriate to the existing frontend architecture.

No fixed maximum digit count may be encoded as a language rule.

An implementation may have an externally supplied resource budget.

That budget is not a language limitation.

---

21. Strings

String syntax is governed by "grammar/lexer/literals.md".

Each string form must specify:

- opening delimiter;
- closing delimiter;
- escape rules;
- newline rules;
- Unicode behavior;
- interpolation behavior;
- raw-string behavior;
- EOF behavior;
- source spans;
- diagnostics.

Unterminated strings must generate one primary lexical diagnostic rather than a cascade of unrelated errors.

---

22. Escape sequences

Invalid escapes MUST be diagnosed lexically.

For example:

"\q"

must either:

- be valid according to the lexical specification; or
- produce a stable invalid-escape diagnostic.

The lexer MUST NOT silently transform an invalid escape into a different valid program.

---

23. Character literals

Character literal rules must define:

- number of logical characters;
- escapes;
- Unicode scalar handling;
- delimiters;
- EOF behavior.

If the language allows multi-character character-like literals, that must be explicit.

The lexer must not assume a character equals one byte.

---

24. Interpolation

Interpolation is governed by:

"grammar/lexer/interpolation.md"

The lexer must define the boundary between:

string text

and:

embedded Zamani expression

Nested delimiters must be handled deterministically.

Malformed interpolation must not cause unbounded recovery scanning.

Interpolation expressions ultimately belong to parser/AST semantics.

---

25. Quantum literal conformance

Quantum lexical forms are governed by:

"grammar/lexer/quantum-literals.md"

The lexer may recognize lexical forms such as:

|0⟩
|1⟩
|+⟩
|-⟩
|ψ⟩

and future generalized forms specified by the language.

The lexer MUST NOT determine:

- number of physical qubits;
- state-vector size;
- Hilbert-space dimension;
- physical device;
- QPU topology;
- gate decomposition;
- QEC code;
- noise model;
- calibration;
- routing;
- scheduling.

Those belong downstream.

A syntactically valid quantum literal MUST NOT be rejected because it implies a large computational state.

---

26. Generic quantum operations

The lexical layer must support generic operation identifiers.

It MUST NOT require the lexer to enumerate every quantum operation known to humanity.

For example, lexical recognition must permit operation names corresponding to:

H
X
CNOT
custom_operation
vendor.operation
domain.operation
future.operation

when those spellings are valid identifiers/names according to the language.

Semantic validation determines whether an operation exists and is applicable.

This preserves extensibility and the canonical:

generic AST Operation
        ↓
quantum::ir

architecture.

---

27. Operators

Operator conformance is governed by:

"grammar/lexer/operators.md"

Every operator must define:

spelling
token identity
precedence relationship
associativity
lexical ambiguity rules
source span
parser consumer

The lexer must use deterministic longest-valid-token behavior where required.

Operator recognition must not depend on target hardware.

---

28. Delimiters

Delimiters include language-defined constructs such as:

(
)
[
]
{
}
,
:
;
.

and any additional delimiters specified by Zamani.

The lexer must recognize delimiters independently of whether their later semantic use is classical, quantum, HDL, AI, or another domain.

---

29. Longest-match rule

Where lexical alternatives overlap, the lexer must define a deterministic resolution rule.

The preferred rule is:

«Consume the longest valid lexical token according to the active lexical profile.»

Exceptions must be explicitly documented.

There must be no dependence on:

- hash-map order;
- thread scheduling;
- compiler optimization;
- machine architecture.

---

30. Lexical ambiguity

A lexical ambiguity must be resolved at the lexical level whenever the specification requires it.

If resolution requires grammatical context, the tokenization strategy must preserve enough information for the parser.

The lexer must not invent semantic interpretation merely to eliminate ambiguity.

---

31. Diagnostics

Lexical diagnostics are governed by:

"grammar/lexer/diagnostics.md"

Every lexical error MUST provide:

stable diagnostic code
severity
primary source span
human-readable message
optional labels
optional notes
optional help
optional related spans
optional fix-it

Fix-its MUST only be generated when the correction is unambiguous.

The core lexer MUST NOT print diagnostics directly to:

stdout
stderr

Library code should return or collect structured diagnostics.

CLI and IDE layers render them.

---

32. Stable diagnostic codes

Lexical diagnostics must use stable identifiers.

The implementation should reserve the "LEX" namespace.

Example registry:

Code| Meaning
"LEX001"| InvalidCharacter
"LEX002"| InvalidUtf8
"LEX003"| UnexpectedEndOfInput
"LEX004"| UnterminatedString
"LEX005"| InvalidEscape
"LEX006"| UnterminatedComment
"LEX007"| UnterminatedInterpolation
"LEX008"| InvalidIdentifier
"LEX009"| InvalidUnicodeIdentifier
"LEX010"| InvalidNumber
"LEX011"| InvalidNumericSeparator
"LEX012"| InvalidRadixPrefix
"LEX013"| InvalidStringPrefix
"LEX014"| InvalidCharacterLiteral
"LEX015"| InvalidOperator
"LEX016"| InvalidDelimiter
"LEX017"| InvalidQuantumLiteral
"LEX018"| LexicalAmbiguity
"LEX019"| UnsupportedLexicalFeature
"LEX020"| ReservedLexicalConstruct
"LEX021"| ConflictingLexicalDirective
"LEX022"| ResourceExhausted
"LEX023"| InvalidLineBreak

The exact existing repository diagnostic registry must be preserved where one already exists.

If a code already exists, it MUST NOT be silently reassigned.

Diagnostic codes are never reused for a different meaning.

---

33. Diagnostic stability

Diagnostic code stability is more important than message-text stability.

The following may change between compatible releases:

wording
punctuation
formatting
help text

The following must remain stable unless a documented breaking language-version change occurs:

diagnostic identity
diagnostic meaning
primary span semantics
severity category

---

34. Diagnostic ordering

Given identical:

source
language version
lexical profile
dialect configuration
resource budget

the lexer must emit diagnostics in deterministic order.

Preferred ordering:

source position
then span length
then diagnostic code
then deterministic emission order

Hash-map iteration order MUST NOT determine diagnostic ordering.

---

35. Error recovery

The lexer should recover after recoverable lexical errors.

Recovery MUST preserve:

- forward progress;
- deterministic output;
- useful subsequent tokens;
- accurate source spans.

Every error path MUST either:

1. consume input; or
2. transition to EOF/terminal state.

No error path may repeatedly inspect the same input without making progress.

This prevents infinite loops.

---

36. Recovery strategies

Recommended recovery behavior:

Invalid character

Consume the invalid lexical unit and continue.

Invalid UTF-8

Consume the malformed byte sequence safely and continue where possible.

Invalid escape

Consume the malformed escape unit and continue the surrounding literal where possible.

Unterminated string

Emit one diagnostic and terminate the string construct at EOF.

Unterminated comment

Emit one diagnostic and terminate the comment at EOF.

Malformed number

Consume the maximal candidate numeric sequence and report one primary diagnostic.

Invalid operator

Consume the longest identifiable candidate and recover deterministically.

---

37. Error amplification control

The lexer must avoid producing thousands or millions of diagnostics from one malformed construct.

For example, an unterminated string:

"very long source...

should not generate a diagnostic for every character.

Diagnostics must be proportional to meaningful lexical failures rather than input length.

---

38. Resource exhaustion

The lexer must be scalable from tiny input to extremely large input.

It MUST NOT define language-level limits such as:

MAX_TOKEN_LENGTH
MAX_IDENTIFIER_LENGTH
MAX_STRING_LENGTH
MAX_COMMENT_LENGTH
MAX_NUMBER_DIGITS
MAX_SOURCE_SIZE
MAX_QUANTUM_LITERAL_SIZE

as fixed universal constants.

Instead, implementation resource controls MAY be supplied by the caller:

LexingContext
LexicalBudget
ResourceManager
CancellationToken

or the equivalent existing repository abstractions.

A resource budget is an implementation policy, not a language semantic restriction.

---

39. "LEX022 ResourceExhausted"

If the implementation cannot continue because an explicitly supplied resource budget is exhausted, it may emit:

LEX022 ResourceExhausted

The diagnostic must identify:

- resource category;
- relevant source span;
- whether the limit came from the caller/tooling environment;
- whether recovery is possible.

The lexer must not report:

«"Zamani supports only N characters."»

unless N is genuinely part of the language specification, which conflicts with the universal scalability objective and therefore should normally not exist.

---

40. Tiny-to-infinity scalability

"Infinity" means:

«no artificial language-level upper bound.»

Physical execution remains bounded by:

- available memory;
- available storage;
- available compute;
- address-space limitations;
- configured resource budgets;
- operating-system constraints;
- target capabilities.

The lexer must therefore scale according to available resources rather than a fixed language maximum.

---

41. Streaming and incremental lexing

The lexical design SHOULD support:

- whole-source lexing;
- streaming input where the API permits;
- incremental lexing;
- editor/LSP use;
- partial source;
- large generated files.

The lexical specification must not assume that the entire program fits in one fixed-size buffer.

State required across chunks must be explicit and deterministic.

---

42. No hard-coded hardware assumptions

The lexer MUST NOT contain language restrictions based on:

CPU count
core count
thread count
GPU count
FPGA count
QPU count
qubit count
memory size
storage size
network node count
tensor dimensions
vector width
register width
accelerator count
timeline count

The lexer does not know the execution target.

---

43. Lexer independence from domains

The lexer must remain domain-neutral.

Quantum, classical, HDL, AI, networking, distributed computing, cryptography, scientific computing, and future domains consume the same lexical foundation.

The lexical layer MUST NOT contain logic such as:

if quantum_target { ... }
if gpu_count > ... { ... }
if cpu_count == ... { ... }

Lexical recognition is independent of target realization.

---

44. Capability independence

The lexer may recognize capability-related syntax.

It must not determine whether the capability exists.

For example:

requires capability("quantum.mid_circuit_measurement")

may be lexically valid regardless of the actual target.

Capability availability belongs to semantic analysis, compilation, scheduling, HAL, or runtime.

---

45. Version and dialect profiles

Lexical behavior must be determined by an explicit language configuration when versioned or dialect-dependent behavior exists.

Conceptually:

LexicalProfile {
    language_version
    enabled_features
    dialects
    compatibility_mode
}

The actual type must integrate with the existing repository architecture rather than creating a duplicate profile system.

The lexer MUST NOT infer language versions from:

- target hardware;
- operating system;
- CPU;
- compiler host;
- environment variables unless explicitly specified.

---

46. Forward compatibility

Unknown future syntax should be handled according to the language's compatibility policy.

A future keyword SHOULD NOT automatically become an incompatible lexical reservation unless the specification deliberately reserves it.

This is particularly important for:

- quantum operations;
- accelerator features;
- AI constructs;
- HDL constructs;
- future computational paradigms.

Extensibility should favor generic lexical forms where semantics can evolve independently.

---

47. Reserved words

Reserved lexical forms must be centrally registered.

Each reserved form must specify:

spelling
version introduced
status
reason for reservation
context
parser impact
compatibility behavior
deprecation behavior

A word must not become reserved merely because one domain happens to use it internally.

---

48. Feature gates

Feature-gated lexical syntax must be explicit.

The lexer must distinguish:

lexically invalid

from:

lexically valid but feature disabled

Where possible, the latter should produce a dedicated compatibility/feature diagnostic rather than pretending that the spelling itself is malformed.

---

49. Compatibility

Conformance must be checked across:

language version
lexer
parser
AST
semantic analyzer
IR
compiler
runtime
tooling

A lexical change is not complete until its compatibility effect is documented in:

"grammar/compatibility/"

particularly:

- "versions.md"
- "migrations.md"
- "deprecated.md"
- "feature-gates.md"

where those files exist.

---

50. Lexer-to-parser contract

The parser must consume the canonical token kinds emitted by the lexer.

The parser MUST NOT duplicate lexical recognition.

For example, the parser should not independently decide whether a character sequence is:

identifier
integer
operator
string
comment

when that decision belongs to the lexer.

If the existing "src/parser.rs" currently duplicates lexical assumptions, conformance work must identify and eliminate the duplication without creating another token authority.

---

51. Lexer-to-AST contract

Tokens carry source spans into the AST.

The frontend AST must preserve enough lexical/source information to support:

- diagnostics;
- source mapping;
- IDE tooling;
- refactoring;
- macro diagnostics;
- provenance;
- deterministic error reporting.

The lexer must not construct domain IR.

---

52. Lexer-to-semantic-model contract

The lexer supplies lexical facts.

Semantic analysis determines meaning.

For example:

tensor<1024, 1024>

is lexically valid if the syntax is valid.

The lexer does not decide whether:

- the tensor fits memory;
- the target supports it;
- a GPU exists;
- an accelerator exists.

Likewise:

qubit[n]

is lexically independent of physical qubit capacity.

---

53. Lexer-to-quantum::ir contract

The lexer must never lower directly into "quantum::ir".

The intended path is:

lexical tokens
      ↓
parser
      ↓
domain-neutral AST
      ↓
semantic analysis
      ↓
quantum semantic model
      ↓
quantum::ir

This preserves "quantum::ir" as the canonical quantum semantic boundary.

---

54. Lexer-to-HDL contract

The lexer recognizes HDL lexical forms.

It must not determine:

- FPGA size;
- ASIC cell library;
- physical placement;
- timing closure;
- synthesis result;
- fabrication technology.

Those are downstream concerns.

---

55. Lexer-to-hardware contract

Hardware identifiers and capability names are lexical data.

The lexer does not validate whether:

gpu.compute
quantum.measurement
fpga.logic
network.low_latency
memory.persistent

actually exists.

That is a capability/semantic concern.

---

56. Lexer-to-resource contract

Resource expressions are lexically validated only.

For example:

requires memory >= 1TiB
requires qubits >= n
requires nodes >= count

must not be rejected because the host currently has less memory or fewer nodes.

The lexer has no knowledge of actual resource availability.

---

57. Rust implementation contract

The Rust implementation must compile with:

Rust 1.97

or:

Rust 1.97.1

and remain compatible with the repository's declared Rust 2021 environment.

The lexer implementation MUST:

- use safe Rust;
- contain no "unsafe";
- avoid undefined behavior;
- avoid unchecked indexing;
- avoid unchecked UTF-8 conversion;
- avoid unsafe FFI assumptions;
- avoid panics on malformed user source;
- avoid "unwrap()"/"expect()" on untrusted lexical input;
- use explicit error handling;
- preserve source spans;
- preserve deterministic behavior.

---

58. No "unsafe"

No Rust lexer or diagnostic implementation may contain:

unsafe

blocks, functions, traits, implementations, or unchecked operations.

The conformance test suite SHOULD include a repository-level safety audit to ensure the lexer path remains free of unsafe Rust.

---

59. Panic safety

Malformed source is normal input to a compiler frontend.

Therefore malformed source MUST NOT cause:

panic
abort
undefined behavior
memory corruption

unless the process itself is externally terminated due to operating-system resource exhaustion.

Expected lexical failures must be represented through structured diagnostics/results.

---

60. Safe indexing

The implementation must not assume source boundaries without validation.

Preferred safe Rust patterns include:

- "char_indices()";
- checked ranges;
- "get()";
- iterators;
- explicit state machines.

Unchecked indexing is prohibited where malformed input could make a boundary invalid.

---

61. Complexity

For ordinary source, lexical processing SHOULD be:

O(n)

where "n" is the source length.

The lexer must avoid pathological quadratic behavior caused by:

- repeatedly rescanning long literals;
- repeated string concatenation;
- repeated source slicing;
- repeated full-source searches;
- recursive recovery over large malformed constructs.

Long tokens must not automatically imply quadratic processing.

---

62. Memory behavior

The lexer should avoid unnecessary copies.

Where compatible with the existing API, token values SHOULD reference or efficiently represent source data rather than copying large literals repeatedly.

However, optimization must never compromise:

- safety;
- source lifetime correctness;
- deterministic behavior;
- diagnostics;
- incremental lexing.

---

63. Very large literals

Very large literals must be handled according to available resources.

The lexer must not impose artificial semantic limits on:

identifier length
integer digits
string length
array literal textual size
quantum-state textual size
tensor dimensions

If a caller supplies a resource budget, exhaustion may be reported as "LEX022".

---

64. Determinism

For identical:

source
language version
dialect configuration
feature configuration
lexical profile

the lexer must produce identical:

token kinds
token boundaries
token values
source spans
diagnostic codes
diagnostic spans
diagnostic ordering

The result MUST NOT depend on:

- CPU;
- GPU;
- number of threads;
- hash randomization;
- map iteration;
- operating system scheduling.

---

65. Parallel lexing

Parallel lexing MAY be implemented as an optimization.

If implemented, it must preserve exactly the same externally observable result as deterministic single-pass lexing.

Parallel execution MUST NOT alter:

- token order;
- spans;
- diagnostics;
- recovery;
- token identity.

Parallelism must therefore be an implementation optimization rather than a language semantic.

---

66. Incremental lexing determinism

When an editor modifies a source region, incremental lexing may reuse unaffected lexical regions.

The resulting complete token stream MUST be equivalent to lexing the complete resulting source from scratch.

Formally:

Lex(full_source_after_edit)
==
IncrementalLex(previous_source, edit)

in all observable lexical outputs.

---

67. Source identity

Diagnostics MAY carry source identity such as:

file
module
virtual document
generated source
macro expansion

but source identity must not alter lexical semantics unless the language specification explicitly defines source-dependent behavior.

---

68. Generated source

Generated source must be lexed using the same lexical contract.

Generators MUST NOT bypass lexical validation.

If generated code requires a dialect or feature profile, that profile must be explicit.

---

69. Macro token streams

Macros and metaprogramming may consume token streams.

Macro systems must receive canonical lexer tokens rather than a separate unofficial token representation.

Macro-generated tokens must still be traceable to:

- source origin;
- expansion origin;
- source span;
- diagnostic context.

---

70. Lexer and interoperability

Interoperability formats such as:

- OpenQASM;
- QIR;
- HDL formats;
- C;
- C++;
- Python;
- Rust;
- WebAssembly;

must not alter Zamani's canonical lexical rules.

Format-specific frontends belong under:

"src/quantum/frontend/formats/"

or the appropriate interoperability subsystem.

The Zamani lexer remains the lexer for Zamani source.

---

71. OpenQASM relationship

OpenQASM syntax is not automatically Zamani syntax.

An OpenQASM frontend may have its own parser/lexer.

Its translated semantic representation must integrate downstream with Zamani's canonical semantic architecture.

The Zamani lexer must not become an accidental OpenQASM compatibility parser.

---

72. HDL interoperability

Likewise, Verilog/SystemVerilog/VHDL or other HDL syntax must not be injected wholesale into Zamani lexical rules.

HDL interoperability belongs to:

"grammar/interoperability/"

and relevant frontend adapters.

Zamani HDL syntax remains governed by:

"grammar/hdl/"

---

73. Diagnostics and tooling

Core lexer diagnostics must be renderer-neutral.

The lexer should produce structured data.

CLI tooling may render:

file.zm:12:7: error[LEX005]: invalid escape sequence

LSP tooling may render:

diagnostic range
severity
code
message

Web/IDE tooling may render richer forms.

The underlying diagnostic identity remains identical.

---

74. Security

The lexer is exposed to potentially hostile input.

It must defend against:

- pathological token lengths;
- malformed UTF-8;
- deeply nested lexical constructs where nesting exists;
- diagnostic amplification;
- quadratic scanning;
- uncontrolled memory growth;
- repeated recovery loops;
- source-snippet duplication;
- malicious Unicode sequences;
- denial-of-service input.

Diagnostics MUST NOT automatically log entire source files.

Sensitive source content must not be written to logs merely because lexing failed.

---

75. Unicode security

The lexical implementation should cooperate with language/tooling policy concerning:

- confusable characters;
- mixed scripts;
- invisible characters;
- normalization;
- bidirectional control characters.

Core lexical validity and optional security linting must remain distinct.

A warning about a confusable identifier should not incorrectly turn a valid program into a lexical error unless the language specification explicitly requires that behavior.

---

76. Logging

Library lexer code MUST NOT write normal diagnostics to:

stdout
stderr

and MUST NOT log complete source text by default.

Structured diagnostics are returned to callers.

Logging, if available, must be:

- opt-in or policy-controlled;
- deterministic where required;
- free of secret source leakage.

---

77. Conformance with "grammar/Zamani.g4"

"Zamani.g4" is the canonical ANTLR composition root.

Its lexical behavior must agree with this contract.

Where ANTLR-specific implementation constraints differ from the Rust lexer, the difference must be:

1. documented;
2. intentional;
3. tested;
4. semantically equivalent.

There must not be two accepted lexical languages.

---

78. ANTLR versus Rust conformance

The following relationship is required:

ANTLR lexer specification
          │
          ▼
canonical token vocabulary
          │
     ┌────┴────┐
     ▼         ▼
ANTLR path   Rust lexer
     │         │
     └────┬────┘
          ▼
   same lexical contract

If one accepts a source program and the other rejects it, the difference must be classified as:

- bug;
- intentional version difference;
- intentional dialect difference;
- unsupported implementation feature.

Unclassified divergence is a conformance failure.

---

79. "grammar/grammar.md"

"grammar/grammar.md" must describe implementation reality.

It should be generated or validated against the actual lexer/parser implementation where possible.

It MUST NOT claim lexical support that "src/lexer.rs" does not implement.

It MUST NOT silently document aspirational lexical features as implemented.

---

80. "grammar/Zamani-Grammar.md"

"Zamani-Grammar.md" may retain broader design material.

However, lexical constructs there must be classified as:

stable
experimental
proposed
deprecated
historical
not implemented

A lexical feature does not become part of the language merely because it appears in this document.

---

81. "grammar/specification/lexical.md"

This is the human-readable normative lexical specification.

It must agree with:

lexer/tokens.md
lexer/keywords.md
lexer/operators.md
lexer/identifiers.md
lexer/literals.md
lexer/comments.md
lexer/unicode.md
lexer/interpolation.md
lexer/quantum-literals.md
lexer/diagnostics.md

Any contradiction must be resolved through the specification authority process rather than silently choosing whichever file was edited last.

---

82. "grammar/spec/lexical.md"

This file should provide the formal lexical contract.

It should reference this conformance document rather than duplicate all implementation details.

It should define:

- tokenization model;
- lexical precedence;
- token boundaries;
- lexical invariants;
- error categories;
- profile/version behavior.

---

83. "grammar/lexer/README.md"

The lexer README should provide navigation to:

tokens.md
keywords.md
operators.md
delimiters.md
identifiers.md
literals.md
comments.md
unicode.md
interpolation.md
quantum-literals.md
numeric-literals.md
diagnostics.md
conformance.md

It should state that "conformance.md" is the final cross-file integration contract.

---

84. Existing "src/lexer.rs"

"src/lexer.rs" is the executable implementation consumer of this contract.

Before marking lexer conformance complete, audit it for:

- token vocabulary;
- keywords;
- operators;
- delimiters;
- literal forms;
- Unicode handling;
- comments;
- interpolation;
- source spans;
- diagnostics;
- recovery;
- EOF behavior;
- determinism;
- hard-coded limits;
- panic paths;
- unsafe code;
- parser compatibility.

Existing behavior must be documented rather than silently changed where compatibility matters.

---

85. Existing "src/parser.rs"

"src/parser.rs" must consume the canonical lexer tokens.

The parser must not reimplement lexical rules.

Parser diagnostics must remain distinguishable from lexer diagnostics.

Example:

LEX010 InvalidNumber

is lexical.

Whereas:

unexpected token

is parser-level.

---

86. Existing "src/ast/mod.rs"

The AST must consume token/source-span information without creating a competing lexical representation.

Every AST node originating from source should be traceable to source coordinates.

The AST should remain domain-neutral.

---

87. Frontend integration

The complete frontend path must be:

source
  ↓
lexer
  ↓
tokens + diagnostics + spans
  ↓
parser
  ↓
domain-neutral AST
  ↓
structural validation
  ↓
semantic model
  ↓
canonical IR

The lexer must remain below semantic/domain analysis.

---

88. Feature completion contract

No lexical feature is complete until all of the following are known:

Feature name
Feature status
Specification owner
Lexical spelling
Token kind
Token boundaries
Keyword/operator status
Unicode behavior
Source span
Diagnostics
Recovery
Version behavior
Dialect behavior
AST consumer
Parser consumer
Semantic consumer
IR implications if any
Tooling behavior
Compatibility behavior
Security implications
Performance characteristics
Positive tests
Negative tests
Boundary tests
Scalability tests
Determinism tests
Hard-coding audit

This is what makes a lexical file independently completable.

---

89. Hard-coding audit

Every lexer change must be checked for accidental fixed limits.

Forbidden universal lexical limits include:

MAX_SOURCE_SIZE
MAX_TOKEN_SIZE
MAX_IDENTIFIER_SIZE
MAX_NUMBER_DIGITS
MAX_STRING_SIZE
MAX_COMMENT_SIZE
MAX_QUANTUM_SIZE
MAX_NESTING
MAX_SYMBOLS
MAX_KEYWORDS

A constant is permitted when it describes implementation configuration rather than language semantics.

For example:

caller-provided diagnostic budget

is acceptable.

A language rule such as:

identifiers may contain at most 256 characters

would require explicit specification justification and conflicts with the desired unbounded scalability model.

---

90. Semantic constants versus implementation limits

This distinction is mandatory.

Valid:

let n = 1024;

because "1024" is program data.

Potentially invalid architecture:

const MAX_QUANTUM_REGISTER = 1024;

if it is used as a universal language restriction.

Likewise:

let width = 4096;

does not violate scalability.

But:

lexer rejects widths > 4096

does.

---

91. Testing architecture

Lexer conformance tests must exist independently from parser tests.

Required categories:

positive
negative
boundary
scalability
determinism
Unicode
diagnostics
recovery
compatibility
incremental
security

Recommended locations:

grammar/tests/lexical/
grammar/tests/diagnostics/
grammar/tests/negative/
grammar/tests/boundary/
grammar/tests/scalability/
grammar/tests/determinism/
grammar/tests/compatibility/

The exact existing test structure should be retained where possible.

---

92. Positive tests

Positive tests must cover:

- every keyword;
- every operator;
- every delimiter;
- every literal;
- every comment form;
- every identifier category;
- Unicode;
- quantum literals;
- interpolation;
- directives;
- numeric forms;
- domain-independent names;
- future-compatible generic operation names.

---

93. Negative tests

Negative tests must cover:

- invalid characters;
- malformed UTF-8 where byte APIs exist;
- invalid identifiers;
- invalid escapes;
- malformed numbers;
- invalid numeric separators;
- malformed prefixes;
- unterminated strings;
- unterminated comments;
- malformed interpolation;
- invalid operators;
- malformed quantum literals;
- unsupported feature forms.

Each negative test must assert the diagnostic code.

---

94. Boundary tests

Boundary tests must include:

- empty source;
- whitespace-only source;
- comment-only source;
- EOF immediately after an opening delimiter;
- EOF immediately after escape prefix;
- EOF inside interpolation;
- EOF inside numeric literal;
- Unicode at token boundaries;
- multi-line strings where permitted;
- very long identifiers;
- very long literals;
- very large source.

Boundary tests must not encode artificial universal maxima.

---

95. Scalability tests

Scalability tests must verify that the lexer can process increasingly large source inputs subject to available test resources.

Tests should scale dimensions such as:

source length
identifier length
literal length
number of declarations
number of tokens
number of comments
number of Unicode characters
number of nested lexical constructs

The tests should verify:

no artificial language limit
no quadratic behavior
no memory leak
no infinite recovery loop
deterministic results

---

96. Determinism tests

The same input must be lexed repeatedly and produce byte-for-byte or structurally equivalent outputs.

Where token values borrow source memory, comparison should use logical token values rather than pointer identity.

Tests must also verify that enabling internal parallelism, where supported, does not change observable results.

---

97. Compatibility tests

For every supported language version:

source
+
version/profile
→
expected tokens
+
expected diagnostics

must be tested.

Breaking lexical changes must have explicit migration guidance.

---

98. Differential testing

Where both ANTLR and Rust lexical paths are available, differential tests SHOULD compare:

token kind
token spelling/value
start span
end span
diagnostics
diagnostic spans

for the same corpus.

Any divergence must be classified.

---

99. Fuzz testing

The lexer SHOULD be fuzz-tested with arbitrary:

- Unicode;
- malformed UTF-8;
- random delimiters;
- random operators;
- long literals;
- deeply structured input;
- invalid escape sequences;
- partial source.

Fuzzing must assert:

no panic
no unsafe behavior
no infinite loop
no undefined behavior
forward progress
valid spans
deterministic result

---

100. Property tests

Useful properties include:

Span validity

0 <= start <= end <= source_length

Progress

Every non-EOF iteration advances or terminates.

Determinism

lex(source) == lex(source)

Incremental equivalence

incremental_lex(edit(source))
==
lex(edit(source))

No silent corruption

Invalid input is either:

- diagnosed; or
- valid according to the specification.

---

101. Repository-wide conformance matrix

The production implementation should maintain a feature matrix covering:

Layer| Required relationship
"specification/lexical.md"| normative lexical semantics
"spec/lexical.md"| formal lexical contract
"lexer/*.md"| detailed lexical contracts
"Zamani.g4"| canonical grammar composition
"src/lexer.rs"| executable implementation
"src/parser.rs"| token consumer
"src/ast/mod.rs"| source/AST consumer
"grammar/grammar.md"| implementation conformance
"grammar/tests/"| executable evidence
"compatibility/"| version evolution
tooling| diagnostic rendering

No feature is complete while one of these layers silently disagrees.

---

102. Grammar-to-token traceability

Every lexical token must be traceable:

token
  ↓
lexical specification
  ↓
lexer contract
  ↓
ANTLR/Rust implementation
  ↓
parser consumer
  ↓
AST consumer
  ↓
tests

Conversely, every lexer token must have a declared consumer or explicit reason for being retained.

Orphaned tokens are conformance failures unless documented as reserved/future tokens.

---

103. Token-to-AST traceability

For tokens that carry semantic information:

token
 ↓
parser rule
 ↓
AST node/property

must be defined in advance.

This avoids the workflow:

write lexer
↓
write parser
↓
discover AST needs
↓
rewrite lexer

The integration contract must already exist.

---

104. Diagnostic-to-test traceability

Every diagnostic code must have:

definition
example
negative fixture
boundary fixture
expected span
expected severity

Unused diagnostic codes should be treated as incomplete unless explicitly reserved for future use.

---

105. Versioning diagnostic codes

Diagnostic codes are API surface for tooling.

Therefore:

- do not reuse codes;
- do not casually change their meaning;
- document additions;
- document removals;
- preserve compatibility where required.

LSP and IDE clients may depend on them.

---

106. Source-preserving transformations

Formatter, refactoring, macro, and source-to-source tools must be able to rely on canonical token spans.

A formatter must not need to reconstruct lexical boundaries independently.

This is especially important for:

- Unicode;
- interpolation;
- comments;
- macros;
- generated source.

---

107. Lexer ownership boundary

The lexer owns:

"What characters/tokens are present?"

It does not own:

"What does this computation mean?"

It does not decide:

Which CPU?
Which GPU?
Which QPU?
Which FPGA?
How many cores?
How much memory?
Which physical qubit?
Which network node?
Which accelerator?
Which topology?
Which schedule?
Which QEC code?

Those are downstream concerns.

---

108. POCO-REAF conformance

The lexer contributes to POCO-REAF by ensuring that source syntax is not coupled to machine realization.

A source program should remain lexically valid when moved between:

tiny device
laptop
server
HPC cluster
GPU system
FPGA system
quantum system
hybrid system
distributed system
future architecture

provided the source itself satisfies the language specification.

Target incompatibility must be discovered downstream through capability and semantic analysis.

---

109. Domain expansion rule

When a new Zamani domain is added:

new domain
    ↓
domain syntax
    ↓
existing lexical vocabulary where possible
    ↓
new lexical forms only if necessary
    ↓
lexer contract
    ↓
parser contract
    ↓
AST contract
    ↓
semantic contract
    ↓
IR contract
    ↓
tests

The domain must not fork the lexer.

---

110. Example: new accelerator

A new accelerator domain should not require:

MAX_ACCELERATORS
ACCELERATOR_0
ACCELERATOR_1
ACCELERATOR_2

as lexical concepts.

Instead:

accelerator
capability
resource
operation

are lexically generic.

Actual accelerator realization belongs downstream.

---

111. Example: new quantum gate

A new quantum operation must not require editing a giant lexer enumeration.

If:

apply future_gate to q

is lexically expressible through generic identifiers/operation syntax, the lexer remains unchanged.

Semantic quantum infrastructure can then recognize the operation.

This is a key scalability property.

---

112. Example: new HDL primitive

A new hardware primitive should preferably be represented using:

identifier
attributes
parameters
ports
operations

rather than requiring a new keyword for every primitive.

Only language-level constructs that genuinely require reserved syntax should add lexical entries.

---

113. Example: new AI operator

AI frameworks must not require every framework operation to become a keyword.

Generic names and operation identifiers should carry framework-independent syntax.

Framework semantics belong to libraries, dialects, semantic registries, or interoperability layers.

---

114. Dialect integration

A dialect may introduce lexical extensions only through:

"grammar/dialects/"

and the explicit dialect registry.

Every lexical extension must define:

dialect name
version
token additions
keyword additions
operator additions
conflict behavior
AST mapping
semantic mapping
compatibility

Dialect syntax must not silently alter the base language.

---

115. Future paradigms

The lexer must be designed so that future computational paradigms can be introduced without redesigning the lexical core.

This favors:

- generic identifiers;
- extensible attributes;
- namespaced operations;
- generic literals where justified;
- explicit dialects;
- capability declarations;
- metadata;
- generic resource expressions.

The lexical layer should remain small and stable even while the semantic universe grows.

---

116. Completion criteria

"grammar/lexer/conformance.md" and the associated lexer implementation are considered production-ready only when all of the following are true:

Authority

- [ ] One lexical authority is established.
- [ ] "Zamani.g4" does not compete with Rust lexical authority.
- [ ] "grammar.md" reflects implementation reality.
- [ ] "Zamani-Grammar.md" cannot silently introduce syntax.

Tokens

- [ ] Every token has a defined identity.
- [ ] Keywords are centralized.
- [ ] Operators are centralized.
- [ ] Delimiters are centralized.
- [ ] Literals are specified.
- [ ] Unicode behavior is specified.

Diagnostics

- [ ] Every lexical error has a stable diagnostic code.
- [ ] Every diagnostic has an accurate span.
- [ ] Diagnostic ordering is deterministic.
- [ ] Recovery is defined.
- [ ] Diagnostic amplification is controlled.

Source spans

- [ ] Token spans are valid.
- [ ] EOF behavior is defined.
- [ ] Unicode coordinates are handled correctly.
- [ ] Parser and AST spans remain compatible.

Safety

- [ ] Rust 1.97/1.97.1 builds successfully.
- [ ] No "unsafe".
- [ ] No unsafe UTF-8 conversion.
- [ ] Malformed source cannot cause lexer panics.
- [ ] Recovery guarantees forward progress.

Scalability

- [ ] No artificial source-size limit.
- [ ] No artificial token-size limit.
- [ ] No artificial identifier-size limit.
- [ ] No artificial literal-size limit.
- [ ] No artificial quantum-size limit.
- [ ] Resource budgets are externally configurable.
- [ ] Large inputs avoid pathological complexity.

Determinism

- [ ] Same source gives same tokens.
- [ ] Same source gives same diagnostics.
- [ ] Same source gives same spans.
- [ ] Parallel/incremental implementations remain equivalent.

Integration

- [ ] "src/lexer.rs" conforms.
- [ ] "src/parser.rs" consumes canonical tokens.
- [ ] "src/ast/mod.rs" preserves source mapping.
- [ ] semantic analysis receives no lexer-specific semantics.
- [ ] "quantum::ir" remains the canonical quantum semantic boundary.
- [ ] HDL/hardware semantics remain downstream.
- [ ] resource/capability semantics remain downstream.
- [ ] tooling consumes structured diagnostics.

Testing

- [ ] Positive tests.
- [ ] Negative tests.
- [ ] Boundary tests.
- [ ] Unicode tests.
- [ ] Diagnostic tests.
- [ ] Recovery tests.
- [ ] Scalability tests.
- [ ] Determinism tests.
- [ ] Compatibility tests.
- [ ] Differential ANTLR/Rust tests where applicable.
- [ ] Fuzz tests.
- [ ] Incremental tests where supported.

---

117. Required integration updates

This file is the conformance contract; it should not need to be repeatedly rewritten whenever an implementation file changes.

The following existing files must instead conform to this contract:

grammar/lexer/README.md
grammar/lexer/tokens.md
grammar/lexer/keywords.md
grammar/lexer/operators.md
grammar/lexer/delimiters.md
grammar/lexer/identifiers.md
grammar/lexer/literals.md
grammar/lexer/comments.md
grammar/lexer/unicode.md
grammar/lexer/interpolation.md
grammar/lexer/quantum-literals.md
grammar/lexer/numeric-literals.md
grammar/lexer/diagnostics.md

grammar/spec/lexical.md
grammar/spec/source-spans.md

grammar/specification/lexical.md

grammar/Zamani.g4
grammar/grammar.md
grammar/Zamani-Grammar.md

src/lexer.rs
src/parser.rs
src/ast/mod.rs

Tests should be added under the repository's existing test hierarchy rather than creating duplicate test systems.

---

118. Required repository invariants

The repository must maintain these invariants:

ONE language
ONE lexical contract
ONE canonical token vocabulary
ONE canonical source-span model
ONE diagnostic identity system
ONE parser token contract
ONE domain-neutral AST boundary
ONE canonical quantum semantic boundary
MANY domains
MANY targets
MANY implementations

Domain growth must not cause lexical fragmentation.

---

119. Final architecture

The production lexical architecture is:

                 Zamani source
                       │
                       ▼
             ┌───────────────────┐
             │ Source decoding   │
             │ Unicode handling  │
             └─────────┬─────────┘
                       │
                       ▼
             ┌───────────────────┐
             │ Canonical Lexer   │
             │                   │
             │ tokens            │
             │ spans             │
             │ diagnostics       │
             └─────────┬─────────┘
                       │
                       ▼
                 Token stream
                       │
                       ▼
                    Parser
                       │
                       ▼
              Domain-neutral AST
                       │
                       ▼
              Structural validation
                       │
                       ▼
              Semantic analysis
                       │
             ┌─────────┼──────────┐
             ▼         ▼          ▼
         Classical  quantum::ir   HDL
             │         │          │
             └─────────┼──────────┘
                       │
                       ▼
             Optimization / lowering
                       │
          ┌────────────┼─────────────┐
          ▼            ▼             ▼
       Routing     Scheduling    Resilience
                       │
                       ▼
                      ZQN
                       │
                       ▼
                      HAL
                       │
                       ▼
               Target realization
                       │
       ┌───────────────┼────────────────┐
       ▼               ▼                ▼
      CPU             GPU              QPU
       │               │                │
       └───────────────┼────────────────┘
                       ▼
                 Future targets

The lexer therefore remains deliberately small in responsibility while supporting an extremely large language.

---

120. Final conformance principle

The fundamental rule is:

«The Zamani lexer defines how source is represented as language tokens; it does not define the limits of computation.»

Therefore:

lexical syntax
      ≠
hardware capability
      ≠
resource availability
      ≠
semantic validity
      ≠
physical realization

A Zamani source program may describe computation at any scale.

The lexer must not prevent that scalability by embedding assumptions about today's machines.

The correct architecture is:

Program intent
      ↓
lexical representation
      ↓
AST
      ↓
semantic requirements
      ↓
canonical IR
      ↓
capability discovery
      ↓
resource negotiation
      ↓
optimization
      ↓
routing / scheduling / resilience
      ↓
target realization

This preserves:

«Program Once → Compile Once → Run Everywhere, Anywhere, Forever»

while allowing the implementation to evolve from today's machines to future computational systems without repeatedly redesigning the lexical foundation.