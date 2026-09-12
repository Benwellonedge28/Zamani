Worked for 14s

Below is the complete production-ready grammar/lexer/README.md specification. I inspected the current canonical lexer and top-level grammar before defining it. The current repository already identifies grammar/antlr/ZamaniLexer.g4 as the canonical lexer, with grammar/antlr/Core.g4 as its parser-side counterpart, and explicitly requires safe Rust 1.97/1.97.1.  The existing lexer also already has important compatibility vocabulary, Unicode identifiers, generic quantum operation handling, numeric/string/character literals, and a stated requirement for deterministic rejection of unterminated lexical constructs.

Zamani Lexer Architecture

Status

Production architecture specification

Path

"grammar/lexer/README.md"

Language

Zamani

Repository

"Benwellonedge28/Zamani"

Grammar technology

ANTLR4

Compiler implementation

Rust 1.97 / Rust 1.97.1

Safety requirement

Safe Rust only. "unsafe" is prohibited.

Primary goal

Provide the lexical foundation for:

«Zamani — From Atom to Everywhere»

and:

«Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)»

---

1. Purpose

This directory defines the lexical architecture of Zamani.

The lexer is the first language-processing layer after source acquisition and before parsing.

Its responsibility is to transform source characters into a deterministic stream of lexical tokens while preserving sufficient source information for:

- parsing;
- diagnostics;
- source locations;
- documentation tooling;
- IDE tooling;
- syntax highlighting;
- formatting;
- source mapping;
- semantic analysis;
- macro processing;
- compiler provenance;
- compatibility tooling;
- error recovery.

The lexer MUST NOT determine the physical machine on which a Zamani program executes.

The lexer MUST NOT encode:

- machine capacity;
- CPU count;
- GPU count;
- FPGA count;
- ASIC count;
- quantum processor size;
- qubit capacity;
- memory capacity;
- register capacity;
- topology;
- device identifiers;
- network size;
- deployment topology;
- hardware timing limits;
- backend-specific gate sets;
- vendor-specific execution requirements.

Those concerns belong to later semantic, resource, compilation, scheduling, routing, hardware, runtime, and deployment layers.

---

2. Architectural Position

The lexical pipeline is:

source bytes
    |
    v
source decoding
    |
    v
Unicode character stream
    |
    v
ZamaniLexer
    |
    +---- lexical diagnostics
    |
    +---- documentation/comment metadata
    |
    v
token stream
    |
    v
Zamani parser
    |
    v
frontend AST
    |
    v
semantic analysis
    |
    +---- type analysis
    +---- effect analysis
    +---- capability analysis
    +---- resource analysis
    +---- quantum semantic lowering
    +---- classical semantic lowering
    +---- HDL semantic lowering
    |
    v
canonical semantic representations / IR
    |
    +---- quantum::ir
    +---- classical/control/data representations
    +---- temporal/resource metadata
    |
    v
optimization
    |
    v
routing / scheduling
    |
    v
target lowering
    |
    v
hardware / runtime

The lexer therefore has no dependency on:

quantum::ir
QEC
ZQN
scheduling
routing
optimization
hardware discovery
calibration
runtime

Those systems consume information originating from the parser/semantic layers rather than controlling lexical recognition.

---

3. Lexer Directory Ownership

The directory:

grammar/lexer/

owns lexical syntax.

It does not own the complete Zamani grammar.

It does not own AST construction.

It does not own semantic interpretation.

It does not own machine realization.

The lexical directory is responsible for defining:

- token categories;
- lexical spellings;
- reserved words;
- identifiers;
- literals;
- operators;
- punctuation;
- comments;
- documentation comments;
- Unicode lexical classes;
- annotation markers;
- lexical error sentinels where appropriate;
- lexical compatibility policy.

---

4. File Inventory

The intended lexer directory is:

grammar/lexer/
├── README.md
├── tokens.g4
├── keywords.g4
├── identifiers.g4
├── literals.g4
├── numeric-literals.g4
├── string-literals.g4
├── character-literals.g4
├── boolean-literals.g4
├── quantum-literals.g4
├── hardware-literals.g4
├── duration-literals.g4
├── size-literals.g4
├── annotations.g4
├── operators.g4
├── punctuation.g4
├── comments.g4
├── unicode.g4
└── lexer-errors.g4

These files are modular lexical specifications.

They MUST NOT become independent competing lexers.

There MUST remain exactly one canonical composed Zamani lexer.

The canonical currently identified repository lexer is:

grammar/antlr/ZamaniLexer.g4

The modular files under:

grammar/lexer/

must therefore have an explicitly defined relationship to that canonical lexer.

---

5. Canonical Lexer Authority

The repository currently identifies:

grammar/antlr/ZamaniLexer.g4

as the canonical lexical grammar.

It also identifies:

grammar/antlr/Core.g4

as the parser-side grammar.

This existing relationship MUST be preserved during migration.

The modular lexer files MUST NOT silently become a second lexical authority.

The eventual architecture must be:

grammar/lexer/*.g4
          |
          v
canonical lexical composition
          |
          v
grammar/antlr/ZamaniLexer.g4
          |
          v
Core parser

or, if the repository later adopts a generated composition architecture:

grammar/lexer/*.g4
          |
          v
canonical generated/composed lexer
          |
          v
Core parser

The authority decision MUST be documented in:

grammar/specification/grammar-authority.md

and mirrored in:

grammar/README.md
grammar/grammar.md
grammar/Zamani-Grammar.md

No documentation file may claim that a different lexer is authoritative without updating the authority specification.

---

6. Lexer File Contract

Every lexer file MUST be independently completable.

Before a file is considered complete, its author must know:

- what tokens it owns;
- what tokens it does not own;
- what lexical forms it accepts;
- what malformed forms it rejects;
- which fragments it depends on;
- which parser rules consume its tokens;
- which semantic layer interprets them;
- what compatibility guarantees apply;
- what tests prove correctness.

No lexer file may rely on an undocumented future change to another lexer file.

---

7. Universal Lexical Principles

All lexer files MUST follow these principles.

7.1 Lexical syntax is not semantic meaning

For example:

H
X
CNOT
U
custom_gate
vendor_operation

may be lexically identifiers.

The lexer must not require a fixed quantum gate vocabulary merely because a quantum backend currently recognizes certain gates.

Quantum operation resolution belongs to semantic analysis and the canonical quantum IR.

---

7.2 No hardware limits

The lexer must not encode:

MAX_QUBITS
MAX_CORES
MAX_THREADS
MAX_MEMORY
MAX_DEVICES
MAX_NODES
MAX_TENSOR_RANK
MAX_REGISTER_SIZE

or equivalent lexical limits.

A lexical sequence may be arbitrarily large subject only to:

- available source representation;
- compiler implementation limits;
- host/runtime resource availability.

Any implementation limit must be treated as an implementation/resource limitation, not as language semantics.

---

7.3 No fixed machine topology

The lexer must not contain assumptions such as:

q[0]
q[1]
gpu0
cpu0
node0
device0

as universal language constructs.

Those may occur as ordinary identifiers or values when a program explicitly refers to a target-specific resource.

The lexer must not assign universal semantic meaning to them.

---

7.4 Unicode

Zamani source is Unicode-aware.

Unicode character classes must be defined centrally.

Identifier semantics belong to "identifiers.g4".

Unicode support itself belongs to "unicode.g4".

Unicode normalization MUST NOT silently occur during lexical analysis.

If normalization is required by the language, it must be specified as an explicit language-level identifier policy.

---

8. "tokens.g4"

Purpose

Define the canonical token vocabulary and reusable lexical token contracts.

Owns

- token declarations required by the lexer composition;
- token names;
- token-category contracts where ANTLR requires explicit declarations.

Does Not Own

- keyword spellings;
- identifier syntax;
- parser grammar;
- semantic types;
- AST nodes.

Inputs

Unicode character stream.

Outputs

Token declarations used by the composed lexer.

Dependencies

- ANTLR lexer model;
- lexical specification.

Downstream Consumers

- canonical lexer;
- parser;
- diagnostic tooling;
- token inspection tools.

Integration

Every token defined here must correspond to a documented lexical concept.

No semantic subsystem should depend directly on a token name without a documented parser/semantic contract.

Tests

Test:

- token existence;
- token uniqueness;
- token stability;
- token category consistency;
- generated lexer compilation.

Completion

Complete when all public token contracts are identified and no duplicate token authority exists.

---

9. "keywords.g4"

Purpose

Define words reserved by the Zamani language.

The existing canonical lexer already contains substantial reserved vocabulary including core control-flow, module, type, concurrency, effect, quantum, and system terms.

These existing features must be audited rather than silently discarded.

Owns

Reserved lexical words.

Does Not Own

- ordinary library names;
- arbitrary quantum gates;
- arbitrary hardware names;
- arbitrary vendor operations;
- semantic interpretation.

Critical Rule

A word should become a keyword only when the parser requires it to have syntactically reserved meaning.

Domain vocabulary that can safely remain an identifier should remain an identifier.

This is especially important for extensibility.

Integration

Keyword recognition must precede identifier recognition according to ANTLR lexical priority.

For example:

fn
module
import
quantum
measure

may be reserved.

But:

H
X
CNOT
my_gate
vendor_gate

should not automatically become keywords merely because they are quantum-related.

Tests

Positive:

fn
module
quantum
measure

Negative:

- keyword embedded in a longer identifier;
- case variants unless explicitly reserved.

Boundary:

fnx
quantum2
measure_result

must be handled according to identifier rules rather than accidentally split.

---

10. "identifiers.g4"

Purpose

Define identifiers independently of their semantic meaning.

Owns

- identifier start characters;
- identifier continuation characters;
- Unicode identifier support;
- escaping rules if Zamani adopts identifier escapes.

Does Not Own

- symbol resolution;
- namespaces;
- types;
- variable existence;
- hardware identity;
- quantum identity.

Requirements

Identifiers must support scalable naming.

No artificial finite identifier length should be imposed by grammar semantics.

The implementation may have resource limits, but such limits must be classified as implementation limits.

Compatibility

The existing canonical lexer explicitly requires Unicode identifier support.

That capability must be preserved.

Integration

Keywords are resolved before or against identifier recognition according to the canonical lexer design.

The semantic layer later determines whether:

q
qubit
processor
GPU
H
CNOT

represent a variable, operation, resource, type, namespace member, or other entity.

---

11. "literals.g4"

Purpose

Provide the umbrella lexical contract for literal categories.

Owns

The conceptual literal family.

Delegates

Specific forms to:

numeric-literals.g4
string-literals.g4
character-literals.g4
boolean-literals.g4
quantum-literals.g4
hardware-literals.g4
duration-literals.g4
size-literals.g4

Does Not Own

Semantic conversion into runtime values.

For example, the lexer identifies:

42
3.14
"hello"
true

but does not determine:

machine integer width
floating-point implementation
heap representation
target ABI

---

12. "numeric-literals.g4"

Purpose

Define integer, floating-point, exponent, radix, separator, and related numeric syntax.

Required Forms

At minimum investigate and standardize:

0
42
1_000
0xFF
0b1010
0o755
3.14
1e9
1.5e-9

Scalability

Numeric lexical syntax must not imply a fixed machine width.

Do not encode:

u8
u16
u32
u64

as universal lexical storage semantics unless they are explicitly part of Zamani's type syntax.

Literal magnitude checking belongs to semantic/type analysis.

Malformed forms

The implementation must deterministically diagnose forms such as:

0x
0b
1_
1e
1e+

according to the finalized numeric specification.

Malformed numeric recognition belongs here rather than in generic lexer errors.

---

13. "string-literals.g4"

Purpose

Define string syntax.

Owns

- delimiters;
- escapes;
- multiline rules if supported;
- raw-string forms if supported;
- interpolation delimiters if formally part of lexical syntax.

Does Not Own

- allocation;
- encoding conversion at runtime;
- string type semantics;
- formatting semantics.

Requirements

Unterminated strings must produce deterministic diagnostics.

String length must not be artificially capped by grammar.

Resource exhaustion belongs to compiler/runtime resource handling.

---

14. "character-literals.g4"

Purpose

Define character literal syntax.

Owns

- character delimiters;
- escape syntax;
- Unicode character representation.

Does Not Own

- runtime character width;
- storage representation;
- encoding policy outside lexical source representation.

Malformed character literals must be rejected deterministically.

---

15. "boolean-literals.g4"

Purpose

Define:

true
false

and any future explicitly standardized boolean literal syntax.

Does Not Own

- Boolean type semantics;
- conditional semantics;
- machine representation.

Boolean literals must remain independent of target representation.

---

16. "quantum-literals.g4"

Purpose

Define lexical forms specifically required for quantum-state notation.

Possible forms include notation such as:

|0⟩
|1⟩
|+⟩
|-⟩

where formally adopted.

Critical Boundary

This file does NOT define quantum semantics.

It must not decide:

- number of physical qubits;
- simulator representation;
- Hilbert-space dimension;
- physical state storage;
- gate implementation;
- measurement behavior;
- backend topology.

Those belong downstream.

Integration

Quantum literal syntax is parsed into AST syntax and semantically lowered toward the canonical quantum representation.

The grammar must never become a second quantum IR.

---

17. "hardware-literals.g4"

Purpose

Define genuinely lexical hardware/resource literal forms where the language specification requires them.

Examples may include formally specified:

- resource quantities;
- addresses;
- symbolic hardware identifiers;
- target references.

Critical Rule

Hardware literals must describe syntax, not availability.

A literal must not imply that a device exists.

For example:

device("some-target")

is source-level data/reference syntax.

Whether that target exists is a semantic/resource/runtime question.

---

18. "duration-literals.g4"

Purpose

Define duration syntax.

Examples may include:

1ns
10us
2ms
1s

if adopted by the language specification.

Does Not Own

- physical clock availability;
- hardware timing;
- scheduler decisions;
- pulse duration feasibility.

Those belong to timing, scheduling, hardware capabilities, and execution systems.

A duration is a program-level value or constraint.

---

19. "size-literals.g4"

Purpose

Define generic size/quantity syntax where Zamani requires it.

Examples may include:

1KiB
1MiB
1GiB

or language-standardized resource quantities.

Scalability

The grammar must not impose an upper machine size.

A literal's numerical range is a semantic/type/resource concern.

---

20. "annotations.g4"

Purpose

Define lexical annotation syntax.

The lexical layer should recognize the annotation marker and annotation name structure without deciding annotation semantics.

The existing repository has historically used forms such as:

@atom
@molecule

and the broader language already uses annotation syntax.

Ownership

This file owns:

@

and/or annotation lexical components according to the canonical token ownership decision.

Critical Integration Rule

There must be exactly one lexical owner for the "@" token.

The repository already contains an "AT : '@';" token in the existing grammar architecture.

Therefore the modular lexer must not create a second incompatible "AT" token.

The migration must select one authority and remove duplication.

Does Not Own

- annotation semantics;
- resource discovery;
- hardware configuration;
- compiler directives unless separately specified.

---

21. "operators.g4"

Purpose

Define operators.

This includes arithmetic, comparison, logical, bitwise, assignment, range, access, and other formally standardized operators.

Requirement

Longest operators must be recognized before their prefixes where lexical ambiguity exists.

For example, if both exist:

>
>=

then ">=" must be recognized as one token.

Likewise:

=
==

and:

-
->

must be handled consistently.

Does Not Own

Operator precedence.

Precedence belongs to parser grammar.

Operator meaning belongs to semantic analysis.

---

22. "punctuation.g4"

Purpose

Define structural punctuation.

Examples include:

(
)
[
]
{
}
,
;
:
.
::

where supported.

Ownership Rule

Every punctuation character must have exactly one lexical owner.

For example, "@" must not simultaneously be independently defined by:

punctuation.g4
annotations.g4
Core.g4

without an explicit generated-token architecture.

The existing repository already contains punctuation definitions, so migration must reconcile duplicates rather than add competing definitions.

---

23. "comments.g4"

Purpose

Define:

- line comments;
- block comments;
- documentation comments.

Requirements

Ordinary comments must not become semantic parser tokens.

Documentation comments must remain available to tooling when the language requires documentation preservation.

The existing canonical lexer explicitly requires documentation comments to be preserved on a dedicated hidden channel.

That behavior must remain compatible.

Unterminated Comments

Unterminated block comments must be diagnosed deterministically.

The error representation must not accidentally consume a valid block comment.

---

24. "unicode.g4"

Purpose

Provide reusable Unicode lexical fragments.

Owns

- Unicode-aware character classes;
- identifier-compatible Unicode fragments;
- Unicode source character categories where needed by lexical rules.

Does Not Own

- normalization;
- locale-specific case conversion;
- semantic identifiers;
- Unicode source decoding.

Malformed UTF-8 must be handled before ANTLR receives a valid character stream.

The lexer cannot reliably repair arbitrary malformed byte encoding.

---

25. "lexer-errors.g4"

Purpose

Provide deterministic lexical error recognition where an invalid lexical construct can be identified structurally.

This file is deliberately narrow.

It must NOT become a generic dumping ground for every parser or semantic error.

Owns

Examples include:

- structurally unterminated block comments;
- dedicated lexical error sentinels where deterministic tokenization is preferable;
- reusable lexical-error fragments.

Does Not Own

- parser errors;
- type errors;
- semantic errors;
- resource errors;
- hardware errors;
- runtime errors;
- QEC errors;
- ZQN errors;
- scheduling errors;
- optimization errors.

---

26. Generic Unknown Characters

A generic final catch-all such as:

LEXER_ERROR_CHAR : . ;

must be treated carefully.

It is potentially useful for deterministic diagnostics, but its placement is critical.

If implemented as an imported lexer rule, ANTLR rule priority may cause valid future or downstream tokens to be shadowed.

Therefore the canonical lexer composition must either:

1. place the generic error rule explicitly as the final canonical lexer rule; or
2. use a controlled error-token architecture whose precedence is formally specified.

The modular "lexer-errors.g4" file must not independently introduce a catch-all that can silently steal valid tokens.

This is particularly important for future extensibility.

---

27. Unterminated Block Comment

A dedicated malformed block-comment rule may be provided if the canonical lexer composition supports it.

The rule must distinguish:

/* valid */

from:

/* unterminated

It must never use a pattern that matches a valid block comment merely because the error rule can consume more source.

The error rule therefore requires lexical structure that excludes the valid terminator.

Conceptually:

/*
   zero or more characters that do not form */
EOF

rather than:

/*
   arbitrary characters
EOF

because the latter can incorrectly outrank a valid block comment.

---

28. Lexical Error Policy

Every lexical error must have:

- deterministic location;
- token/span information;
- stable diagnostic category;
- useful human-readable message;
- machine-readable diagnostic identity where required;
- recovery behavior;
- compatibility expectations.

The lexer must not print diagnostics directly to stdout.

Diagnostics must flow through the compiler diagnostic infrastructure.

---

29. Error Recovery

Lexer recovery must never silently convert malformed source into valid semantics.

Examples:

"unterminated

must not become a valid string.

Likewise:

0x

must not silently become:

0

unless the language specification explicitly defines that behavior.

Error recovery should favor:

deterministic failure
+
precise source span
+
continued analysis where safely possible

rather than silent repair.

---

30. Source Locations

Every emitted token must retain enough information for:

- source file identity;
- start position;
- end position;
- line;
- column;
- offset information where required.

The exact source-location representation belongs to the compiler/frontend implementation, not the ".g4" files.

The lexer must not destroy location information.

---

31. Hidden Channels

Comments and documentation comments must be assigned to explicitly documented channels.

Recommended conceptual distinction:

DEFAULT_TOKEN_CHANNEL

for parser-visible tokens.

HIDDEN

for ordinary comments and whitespace.

A dedicated documentation channel may be used if the compiler/tooling architecture benefits from it.

The choice must be consistent with parser, formatter, documentation generator, and IDE tooling.

---

32. Whitespace

Whitespace is lexical trivia unless the language explicitly makes whitespace significant.

Whitespace must not be used to encode:

- machine size;
- hardware layout;
- quantum topology;
- scheduling;
- resource limits.

Whitespace handling must be deterministic and Unicode-aware where applicable.

---

33. Reserved Words and Future Evolution

The keyword set must be versioned.

Adding a keyword can break programs that previously used that spelling as an identifier.

Therefore keyword additions require:

- language-version policy;
- compatibility classification;
- migration documentation;
- reserved-word testing.

The language must maintain a reserved-space policy so future domains can be added without unnecessarily consuming common identifiers.

---

34. Domain Extensibility

The lexer must remain deliberately domain-light.

Quantum, classical, HDL, AI, networking, hardware, and distributed features should generally reuse:

identifiers
literals
operators
punctuation
annotations

rather than continuously adding domain-specific lexical keywords.

For example, a future operation:

topological_fault_tolerant_transform

should not require a lexer modification merely because a new quantum technology was invented.

Semantic extensibility is essential to POCO-REAF.

---

35. Quantum Integration

The lexical layer supports quantum syntax but does not implement quantum computation.

The boundary is:

quantum source syntax
        |
        v
lexer
        |
        v
parser
        |
        v
frontend AST
        |
        v
semantic lowering
        |
        v
quantum::ir

The lexer MUST NOT:

- construct quantum IR;
- assign physical qubit IDs;
- assign physical topology;
- select gates;
- select QEC codes;
- choose hardware;
- schedule gates;
- infer calibration;
- execute circuits.

---

36. Classical Integration

Classical syntax follows the same principle.

The lexer identifies:

fn
let
42
+
foo

but does not determine:

- register allocation;
- CPU architecture;
- SIMD width;
- memory layout;
- vector width;
- ABI;
- instruction selection.

Those belong to later compilation stages.

---

37. HDL Integration

HDL lexical forms must remain independent of a particular FPGA, ASIC, Verilog implementation, or vendor.

The lexer may recognize HDL-specific reserved syntax where required.

It must not encode:

number_of_luts
number_of_flip_flops
clock_frequency
device_part_number

as universal lexical semantics.

Those belong to hardware/resource/target descriptions.

---

38. Hardware Integration

Hardware references are source-level descriptions or requirements.

The lexer must not verify hardware existence.

For example:

target("device")

may be syntactically valid.

Whether the device exists is determined later.

The lexical architecture therefore supports:

program semantics
        +
requirements
        +
capabilities
        +
constraints
        +
hints

without making any of them equivalent to hardware realization.

---

39. Resource Integration

Resource quantities must be lexically representable without imposing finite machine limits.

For example, source may express a requirement conceptually equivalent to:

requires quantum
requires memory >= expression
requires capability(...)

The lexer recognizes the syntax.

Resource analysis determines whether the requirement can be satisfied.

Runtime/resource managers determine actual availability.

---

40. Compilation Integration

The lexer feeds the parser.

The parser produces syntax.

Semantic analysis converts syntax into meaning.

Compilation then performs:

semantic analysis
    |
    v
IR
    |
    v
optimization
    |
    v
routing
    |
    v
scheduling
    |
    v
target lowering

The lexer must remain outside these decisions.

---

41. Runtime Integration

The runtime must never depend directly on lexical implementation details.

Runtime consumers receive:

- compiled representations;
- semantic metadata;
- resource requirements;
- executable plans;
- target information.

They should not need to know whether the source used:

0xFF
255
two hundred fifty-five

if all represent the same semantic value.

---

42. AST Contract

The lexer does not own the AST.

The contract is:

Lexer
  -> tokens + source locations
Parser
  -> syntax tree
AST
  -> semantic interpretation

The lexer must therefore avoid encoding semantic objects inside token text.

Token text may be used by later layers to construct AST values.

---

43. Semantic Contract

The lexical layer answers:

«"What lexical token is this source sequence?"»

It does not answer:

«"What does this token mean in this program?"»

Examples:

H

may lex as an identifier.

Semantic resolution may later determine that it names:

- a quantum operation;
- a user function;
- a dialect operation;
- a library symbol;
- another entity.

Likewise:

gpu

may be a reserved word only if the grammar requires it.

Its actual hardware meaning is not a lexer concern.

---

44. Determinism

Given identical source text and identical language-version configuration, the lexer must produce the same token stream.

Determinism must cover:

- token kind;
- token text;
- token ordering;
- source spans;
- channel;
- lexical diagnostics.

No lexical rule may depend on:

- current time;
- random state;
- hardware;
- network;
- filesystem state;
- runtime device discovery.

---

45. Scalability

The lexical architecture must scale from:

tiny source

to:

very large source

subject to available resources.

There must be no language-semantic maximum such as:

MAX_TOKENS
MAX_IDENTIFIER_LENGTH
MAX_LITERAL_VALUE
MAX_SOURCE_LINES
MAX_PROGRAM_SIZE

inside the grammar.

If implementation limits are unavoidable, they must be:

1. explicit;
2. externally configurable where appropriate;
3. documented;
4. tested;
5. reported as resource/implementation limitations;
6. never confused with language semantics.

---

46. Hard-Coding Audit

Every lexer change must be checked for:

fixed qubit counts
fixed CPU counts
fixed GPU counts
fixed node counts
fixed device IDs
fixed topology
fixed memory
fixed register widths
fixed accelerator counts
fixed tensor dimensions
fixed deployment sizes
fixed hardware timing

Any occurrence must be classified as:

language semantic requirement
target-specific requirement
resource constraint
implementation limitation
accidental hard-coding
test-only limitation
documentation-only limitation

Accidental hard-coding must be removed.

---

47. Compatibility With Existing Lexer

The current canonical lexer already contains a broad lexical vocabulary and existing literal rules.

Migration must preserve valid existing functionality unless explicitly deprecated.

Important existing areas include:

- core keywords;
- module/package vocabulary;
- type vocabulary;
- asynchronous/concurrent constructs;
- effect vocabulary;
- quantum vocabulary;
- Unicode identifiers;
- numeric literals;
- floating-point literals;
- strings;
- characters;
- quantum-state literals;
- annotations;
- compatibility-oriented system vocabulary.

The current lexer explicitly states that quantum gate vocabulary should not be made an exhaustive keyword list.

That principle must be preserved.

---

48. Duplicate Definition Audit

Before finalizing modular lexer composition, search for duplicate definitions across:

grammar/Zamani.g4
grammar/antlr/ZamaniLexer.g4
grammar/antlr/Core.g4
grammar/lexer/*.g4
grammar/core/*.g4
grammar/expressions/*.g4

Particular attention must be paid to:

AT
IDENTIFIER
INTEGER
FLOAT
STRING
CHAR
TRUE
FALSE
operators
punctuation
comments

There must be one authoritative lexical definition for each token.

---

49. Existing "@" Annotation Conflict

The repository already contains an "AT : '@';" lexical definition in the grammar architecture.

The canonical lexer has also historically represented annotation-related syntax.

Therefore:

annotations.g4
punctuation.g4
Core.g4
ZamaniLexer.g4

must not independently define incompatible versions of "@".

The migration must establish:

one token
one lexical owner
many parser consumers

The parser may consume that token in:

annotation
attribute
pragma
metadata

rules.

---

50. Generated Rust Integration

ANTLR-generated Rust code is generated output.

It must not be manually edited as the primary implementation.

The repository must establish:

.g4 source
   |
   v
ANTLR generation
   |
   v
generated Rust
   |
   v
cargo check/test

The generated code must compile under:

Rust 1.97
Rust 1.97.1

as specified by the repository toolchain policy.

No generated or handwritten compiler Rust may use:

unsafe

---

51. Lexer Runtime Boundary

ANTLR is responsible for lexical state-machine execution.

Rust compiler infrastructure is responsible for:

- source acquisition;
- diagnostics;
- integration;
- testing;
- API boundaries;
- error reporting;
- safe resource management.

No lexer grammar should require unsafe Rust actions.

Avoid embedding substantial Rust actions inside ".g4" files.

Prefer pure ANTLR lexical rules and external Rust diagnostic handling.

---

52. Security

The lexer must be robust against hostile source.

Tests must include:

- enormous identifiers;
- enormous literals;
- deeply repetitive lexical sequences;
- malformed Unicode;
- malformed escapes;
- unterminated constructs;
- pathological comments;
- pathological operator sequences;
- invalid numeric prefixes;
- invalid delimiter sequences.

The lexer must not:

- access the network;
- read arbitrary files;
- execute user code;
- invoke hardware;
- inspect runtime devices;
- print directly to stdout;
- leak secrets.

---

53. Resource Exhaustion

The language must distinguish:

invalid syntax

from:

valid syntax that exceeds available resources

For example, a very large valid identifier is not necessarily invalid language syntax merely because a particular compiler configuration cannot allocate enough memory.

The diagnostic system must preserve this distinction.

---

54. Testing Architecture

Every lexer component requires tests.

Required categories:

positive
negative
boundary
compatibility
determinism
scalability
cross-domain
diagnostic
round-trip
hard-coding audit

---

55. Positive Lexer Tests

Test every valid token family.

Examples:

fn
let
quantum
measure
module
import
@
foo
αβ
42
0xff
3.14
1e9
"hello"
'a'
true
false
|0⟩

as applicable to the finalized language specification.

---

56. Negative Lexer Tests

Examples:

0x
0b
1_
1e+
"unterminated
'unterminated
/* unterminated
invalid escape

where those forms are invalid under the final lexical specification.

Every negative test must verify:

- failure category;
- source location;
- deterministic behavior;
- absence of silent semantic corruption.

---

57. Boundary Tests

Boundary tests must include:

- empty source;
- one-character source;
- one-token source;
- adjacent tokens;
- maximum supported implementation input;
- extremely long identifiers;
- extremely long comments;
- extremely long literals;
- large Unicode identifiers;
- large valid programs.

Tests must not establish artificial language limits merely by using fixed-size fixtures.

---

58. Cross-Domain Tests

The lexer must tokenize source combining:

classical + quantum
classical + HDL
quantum + hardware
quantum + distributed
AI + quantum
AI + hardware
classical + quantum + distributed
classical + quantum + HDL + hardware

The lexer must not need to understand the semantics of those combinations.

Its role is to tokenize them consistently.

---

59. Determinism Tests

Run identical input repeatedly and compare:

token type
token text
token order
source span
channel
diagnostics

All must remain stable.

---

60. Round-Trip Tests

Where a formatter/printer exists:

source
  |
lexer
  |
parser
  |
AST
  |
printer
  |
source
  |
lexer
  |
parser

must preserve intended semantics.

Whitespace/comment preservation may have separate guarantees.

---

61. Compatibility Tests

Existing valid Zamani programs must be lexed consistently before and after modularization.

The compatibility suite must cover all existing repository examples and fixtures.

Before removing a lexical spelling:

1. locate consumers;
2. classify the spelling;
3. determine compatibility impact;
4. document migration;
5. add regression tests.

---

62. Lexer-to-Parser Contract

The lexer must provide stable token names and spellings to:

grammar/antlr/Core.g4

Parser rules must not depend on undocumented token behavior.

If a token changes:

token name
token spelling
token channel
token boundary

the parser compatibility contract must be reviewed.

---

63. Lexer-to-AST Contract

The lexer exposes lexical facts.

The AST represents syntax.

Therefore:

lexer

must never create semantic AST concepts such as:

PhysicalQubit
GPUDevice
QuantumBackend
Schedule
QECCode
NoiseModel

Those are downstream concepts.

---

64. Lexer-to-IR Contract

There is no direct lexer → IR dependency.

The correct path is:

lexer
  ↓
parser
  ↓
AST
  ↓
semantic analysis
  ↓
canonical IR

This is especially important for "quantum::ir".

The lexer must never contain rules designed to construct or imitate "quantum::ir".

---

65. Lexer-to-QEC Contract

No direct dependency.

QEC consumes semantic quantum representations and execution/resource information.

Lexical syntax such as:

logical
surface
code
parity

must not cause the lexer to instantiate QEC semantics.

---

66. Lexer-to-ZQN Contract

No direct dependency.

ZQN describes noise/fault semantics downstream.

A token such as:

noise

is merely lexical.

Its interpretation belongs to semantic analysis and ZQN integration.

---

67. Lexer-to-Scheduling Contract

No direct dependency.

Timing/duration literals may be parsed lexically, but scheduling decides:

- ordering;
- timing;
- resource usage;
- alignment;
- dependencies.

The lexer must not schedule anything.

---

68. Lexer-to-Hardware Contract

No direct dependency.

Hardware syntax describes source-level intent.

Hardware discovery/capabilities are resolved later.

The lexer must not query hardware.

---

69. Lexer-to-Optimization Contract

No direct dependency.

Operators and expressions are tokenized.

Optimization later transforms semantic representations.

The lexer must not perform optimization.

---

70. Lexer-to-Runtime Contract

No direct dependency.

Runtime receives compiled/executable representations.

The runtime must not need to understand lexical tokenization.

---

71. Lexer Documentation Contract

Every lexical file must document:

Purpose
Owns
Does Not Own
Syntax
Dependencies
Token contract
Parser consumers
Semantic consumers
Error behavior
Compatibility
Scalability
Tests
Completion criteria

This README establishes the directory-wide contract.

Individual files should provide their local details.

---

72. Dependency Order

The recommended lexical implementation order is:

1. unicode.g4
2. tokens.g4
3. identifiers.g4
4. punctuation.g4
5. operators.g4
6. keywords.g4
7. boolean-literals.g4
8. numeric-literals.g4
9. character-literals.g4
10. string-literals.g4
11. quantum-literals.g4
12. duration-literals.g4
13. size-literals.g4
14. hardware-literals.g4
15. literals.g4
16. annotations.g4
17. comments.g4
18. lexer-errors.g4
19. canonical lexer composition
20. parser integration
21. lexer tests
22. compatibility tests

This order minimizes backtracking and prevents higher-level lexical constructs from defining foundations prematurely.

---

73. Integration Before Completion

A lexer file is NOT complete merely because ANTLR accepts it.

Completion requires:

grammar valid
+
token ownership resolved
+
priority resolved
+
parser contract defined
+
diagnostics defined
+
compatibility checked
+
tests present
+
scalability audit passed
+
hard-coding audit passed

---

74. File Completion Template

Every lexer file must satisfy this template.

File:
Purpose:
Owns:
Does Not Own:
Inputs:
Outputs:
Dependencies:
Upstream Contracts:
Downstream Consumers:
Public Grammar Contract:
AST Contract:
Semantic Contract:
IR Integration:
Compiler Integration:
Runtime Integration:
Tooling Integration:
Cross-Domain Integration:
Tests:
Negative Tests:
Boundary Tests:
Compatibility Requirements:
Scalability Requirements:
Hard-Coding Audit:
Completion Criteria:

No file is complete until every field has a known answer.

---

75. Production Readiness Gate

The lexer subsystem is production-ready only when:

- one canonical lexer exists;
- all modular lexer files have explicit ownership;
- duplicate tokens are removed;
- keyword policy is documented;
- Unicode policy is documented;
- literal policy is documented;
- comment policy is documented;
- documentation-comment behavior is stable;
- lexical error behavior is deterministic;
- malformed lexical constructs cannot silently become valid constructs;
- parser token contracts are stable;
- existing features are preserved or explicitly migrated;
- no machine-size limits exist in grammar semantics;
- no hardware topology is encoded;
- no quantum backend is encoded;
- no QEC implementation is encoded;
- no ZQN implementation is encoded;
- no scheduling implementation is encoded;
- no runtime implementation is encoded;
- generated Rust compiles with Rust 1.97/1.97.1;
- no unsafe Rust is required;
- positive tests pass;
- negative tests pass;
- boundary tests pass;
- determinism tests pass;
- compatibility tests pass;
- cross-domain tests pass;
- scalability tests pass;
- hard-coding audit passes.

---

76. POCO-REAF Verification

The lexical architecture supports POCO-REAF when the same source can remain lexically valid regardless of target scale.

The following should not require lexical redesign merely because the target changes:

tiny embedded system
CPU
multicore CPU
GPU
FPGA
ASIC
quantum processor
quantum simulator
heterogeneous accelerator
cluster
supercomputer
cloud
distributed system
future architecture

The source remains a description of computation.

Target realization occurs later.

---

77. Final Ownership Boundary

The fundamental boundary is:

LEXER
  |
  | recognizes source syntax
  v
PARSER
  |
  | constructs syntax
  v
AST
  |
  | determines meaning
  v
SEMANTIC ANALYSIS
  |
  | establishes requirements/capabilities/types/effects
  v
CANONICAL IR
  |
  | optimization/routing/scheduling
  v
TARGET REALIZATION
  |
  v
RUNTIME / HARDWARE

The lexer must never cross this boundary merely for convenience.

---

78. Final Principle

The Zamani lexer must embody:

«Zamani describes computation, intent, capabilities, constraints, and semantics—not arbitrary limitations of the machine currently available.»

Therefore:

one source program
        |
        v
one lexical meaning
        |
        v
one syntactic meaning
        |
        v
one semantic program
        |
        +--> many scales
        +--> many architectures
        +--> many hardware configurations
        +--> many execution environments
        +--> future platforms

This is the lexical foundation required for:

«Zamani: From Atom to Everywhere»

and:

«Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF).»

The important repository-specific correction is that this README does not declare grammar/lexer/*.g4 to be a second authoritative lexer. The existing grammar/antlr/ZamaniLexer.g4 already declares itself the canonical lexer, so the modular directory must ultimately compose into or deterministically derive that authority rather than creating competing token definitions.

Also, I deliberately kept lexer-errors.g4 narrow: malformed numeric/string/character forms belong with their respective literal grammars, while generic parser/semantic/runtime errors do not belong in the lexer. This avoids the common production failure where a catch-all lexer error rule masks future valid syntax.