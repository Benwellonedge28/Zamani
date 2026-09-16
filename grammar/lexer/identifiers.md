Zamani Identifier Specification

Path: "grammar/lexer/identifiers.md"
Status: Normative lexical specification
Authority: Canonical identifier contract for Zamani lexical analysis
Language: Zamani
Implementation baseline: Rust 1.97 / Rust 1.97.1, Rust 2021
Safety: Safe Rust only; "unsafe" is prohibited
Primary grammar authority: "grammar/Zamani.g4"
Implementation reference: "src/lexer.rs"
Parser consumer: "src/parser.rs"
AST consumer: "src/frontend/ast/" and the repository's canonical AST implementation
Semantic consumers: name resolution, semantic analysis, type/effect/resource analysis
IR consumers: canonical semantic IRs, including "quantum::ir" for quantum semantics

---

1. Purpose

This file specifies the complete lexical and semantic contract for Zamani identifiers.

An identifier is the language mechanism used to give stable names to program entities without imposing implementation-specific limits on:

- identifier length;
- namespace depth;
- module depth;
- number of declarations;
- number of symbols;
- number of generic parameters;
- number of fields;
- number of resources;
- number of devices;
- number of qubits;
- number of CPUs;
- number of GPUs;
- number of FPGAs;
- number of nodes;
- number of timelines;
- number of processes;
- number of network endpoints;
- tensor dimensions;
- register widths;
- or any other machine-dependent quantity.

Identifier syntax is therefore part of the portable source-language layer.

It must remain independent of:

- CPU architecture;
- operating system;
- ABI;
- compiler backend;
- GPU vendor;
- QPU vendor;
- FPGA family;
- HDL implementation;
- physical qubit numbering;
- routing;
- scheduling;
- QEC implementation;
- ZQN implementation;
- HAL implementation;
- deployment topology;
- runtime resource limits.

The identifier system must support Zamani's:

«Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)»

architecture.

---

2. Scope

This specification owns:

1. Identifier lexical recognition.
2. Unicode identifier rules.
3. Identifier start characters.
4. Identifier continuation characters.
5. ASCII compatibility.
6. Unicode normalization policy.
7. Identifier comparison policy.
8. Reserved keywords.
9. Contextual keywords.
10. Raw identifiers, if enabled.
11. Escaped identifiers, if enabled.
12. Identifier source spelling preservation.
13. Identifier source spans.
14. Qualified names.
15. Namespace/path composition.
16. Identifier diagnostics.
17. Lexical ambiguity rules.
18. Identifier scalability requirements.
19. Lexer-to-parser integration.
20. Lexer-to-AST integration requirements.
21. Identifier-related conformance requirements.

This specification does not own:

- symbol-table implementation;
- scope resolution algorithms;
- type checking;
- overload resolution;
- module loading;
- package resolution;
- name mangling;
- ABI symbols;
- linker symbols;
- backend-specific symbol restrictions;
- filesystem naming restrictions;
- operating-system environment-variable naming;
- target-specific identifier restrictions.

Those belong to downstream contracts.

---

3. Architectural Position

The identifier pipeline is:

source text
    │
    ▼
Unicode decoding
    │
    ▼
identifier lexical recognition
    │
    ▼
Token::Identifier / keyword token
    │
    ▼
parser
    │
    ▼
AST name representation
    │
    ▼
name resolution
    │
    ▼
semantic model
    │
    ▼
canonical IR
    │
    ├── classical IR
    ├── quantum::ir
    └── HDL/hardware/domain IR
    │
    ▼
compiler/backend/runtime

The lexer must not perform semantic name resolution.

The parser must not reinterpret the spelling of an identifier.

The AST must preserve enough information for diagnostics and source-to-source tooling.

The semantic layer determines what a successfully lexed identifier refers to.

---

4. Normative Terms

The following terms are normative:

- MUST — mandatory.
- MUST NOT — prohibited.
- SHOULD — recommended unless a documented compatibility reason prevents it.
- SHOULD NOT — discouraged unless a documented compatibility reason exists.
- MAY — permitted.
- IMPLEMENTATION LIMIT — a limit imposed by a compiler, runtime, host, filesystem, backend, or tool.
- LANGUAGE LIMIT — a limit imposed by the Zamani specification itself.

An implementation limit MUST NOT silently become a language limit.

---

5. Core Identifier Model

A Zamani identifier consists conceptually of:

IdentifierStart IdentifierContinue*

where:

IdentifierStart

is the set of Unicode characters permitted at the beginning of an identifier, and:

IdentifierContinue

is the set of Unicode characters permitted after the first character.

The preferred Unicode model is based on Unicode identifier properties equivalent to:

XID_Start
XID_Continue

with the language-defined allowance of "_" where specified below.

The implementation MUST use Unicode-aware character classification rather than an ASCII-only test.

The implementation MUST NOT invent a finite list of supported alphabetic scripts.

---

6. ASCII Identifier Form

The following ASCII forms MUST be accepted:

name
value
counter
result
foo_bar
_value
value2
x0
matrix_1024
quantum_state

The following ASCII forms MUST NOT be accepted as identifiers:

123name
1value
0xname
3d

A digit MAY occur after the first identifier character.

A digit MUST NOT be the first identifier character.

---

7. Unicode Identifier Form

Zamani identifiers MUST support Unicode identifiers.

Examples:

π
λ
μ
Δ
量子
状態
данные
даннi
дані
δοκιμή
مرحبا
अनुभव
தமிழ்

Unicode support is not restricted to a predetermined script list.

The implementation MUST classify characters according to the Unicode identifier model rather than a hard-coded set such as:

[A-Za-z]

or:

[A-Za-z_][A-Za-z0-9_]*

as the complete language rule.

ASCII remains fully supported for portability and interoperability.

---

8. Unicode Version

The language specification MUST define which Unicode identifier property version is normative for a given Zamani language version.

The implementation MUST expose its Unicode-data version through compiler/tooling metadata where practical.

A compiler MUST NOT silently change identifier classification merely because the host operating system has a different Unicode database.

The Unicode classification used for a given Zamani language version SHOULD therefore be deterministic.

If Unicode property data is upgraded:

1. the language version or compatibility policy MUST document the change;
2. conformance tests MUST be updated;
3. compatibility implications MUST be recorded;
4. previously valid source MUST NOT be silently invalidated without a documented language-version rule.

---

9. Unicode Normalization

Zamani MUST preserve the original source spelling of an identifier.

The compiler MUST NOT silently rewrite the source text merely to normalize it.

Identifier equality MUST use the language-defined identifier comparison policy.

The preferred production policy is:

source spelling preserved
+
Unicode identifier classification based on XID properties
+
no implicit normalization of source text

The implementation MUST document whether semantic identifier equality uses canonical Unicode normalization.

For the initial production contract:

«Identifiers are compared by their normalized identifier representation only if the language version explicitly enables such normalization; otherwise source code points remain significant.»

This distinction is important because visually identical identifiers can have different Unicode representations.

For example:

é

and:

e + combining acute accent

MUST NOT accidentally become indistinguishable merely because they render similarly.

A future normalization policy MUST be versioned and tested.

---

10. Case Sensitivity

Zamani identifiers are case-sensitive.

Therefore:

value
Value
VALUE
vAlUe

are distinct identifiers.

The lexer MUST preserve case.

The lexer MUST NOT perform case folding.

Keyword recognition MUST use the exact language-defined keyword spelling unless a future language version explicitly introduces a different policy.

---

11. Underscore

The underscore character "_" MAY appear:

- as the first identifier character;
- inside an identifier;
- as the final identifier character.

Examples:

_
_value
value_
_value_2
quantum_register

The language MAY reserve "_" in specific syntactic contexts such as pattern matching.

That contextual meaning belongs to the parser/semantic layer.

The lexer MUST NOT globally reject "_" merely because "_" has contextual meanings elsewhere.

For example:

_

may lex as an identifier token while the parser later interprets it as a wildcard pattern.

---

12. Digits

Digits are permitted after the first identifier character.

Examples:

x0
x1
register2
state_3
tensor16

The language MUST NOT impose a maximum number of digits in an identifier.

Therefore:

x123456789012345678901234567890

is lexically valid provided the implementation has sufficient resources.

The implementation MUST NOT impose a language-level maximum such as:

identifier length <= 32
identifier length <= 64
identifier length <= 255

unless such a limit is explicitly established by a future language specification.

---

13. Identifier Length

There is no universal semantic maximum identifier length.

A conforming implementation MAY impose implementation-resource limits for:

- memory;
- parser stack;
- token-buffer capacity;
- diagnostic storage;
- compiler process resources.

Such limits MUST:

1. be implementation limits;
2. be documented;
3. produce a deterministic diagnostic;
4. not change the language grammar;
5. not be represented as a grammar-level identifier-length restriction.

The grammar MUST NOT contain constructs such as:

Identifier: [a-zA-Z_][a-zA-Z0-9_]{0,63}

or equivalent fixed-width restrictions.

The compiler MUST scale with available resources rather than with arbitrary language constants.

---

14. Identifier Representation

The lexer MUST distinguish between:

identifier spelling

and:

resolved symbol

A token MUST carry the source representation needed by downstream stages.

The existing lexer token contract contains:

TokenType::Identifier
Token.literal
Token.span

The identifier specification is therefore compatible with the existing architecture.

The lexer MUST NOT resolve an identifier to a declaration.

For example:

value

must remain an identifier token even if the lexer has no knowledge of whether "value" refers to:

- a variable;
- function;
- type;
- module;
- quantum register;
- hardware capability;
- resource;
- data object;
- AI model;
- HDL signal;
- namespace;
- macro;
- imported symbol.

---

15. Source Spelling Preservation

The original identifier spelling MUST be available for diagnostics and tooling.

For:

π

the lexer MUST preserve:

π

rather than replacing it with an internal ASCII approximation.

For:

量子

the source spelling MUST remain:

量子

This is required for:

- diagnostics;
- IDE integration;
- source maps;
- formatting;
- refactoring;
- semantic highlighting;
- debugging;
- source-to-source transformation;
- provenance.

---

16. Source Spans

Every identifier token MUST have an accurate source span.

The existing lexer already associates tokens with:

Span

and the repository's source-map architecture is therefore the downstream integration point.

A span MUST identify the exact source range occupied by the identifier.

The span MUST be measured in the repository's canonical source-position representation.

The lexer MUST NOT calculate identifier positions by assuming:

one Unicode character == one byte

because UTF-8 characters may occupy multiple bytes.

The implementation MUST preserve correct byte offsets while also supporting line/column diagnostics.

---

17. UTF-8

Zamani source is UTF-8.

The lexer MUST process valid UTF-8 source deterministically.

Invalid UTF-8 MUST be rejected at the source-decoding boundary rather than interpreted as arbitrary identifier characters.

The lexer MUST NOT perform unsafe byte-to-character conversion.

Rust implementation MUST remain safe Rust.

No:

unsafe

is permitted.

---

18. Unicode Combining Marks

Unicode combining marks may occur in identifier continuation positions when permitted by the Unicode identifier property model.

They MUST NOT independently begin an identifier unless the applicable Unicode identifier property explicitly permits them as an identifier start.

This prevents arbitrary combining marks from being accepted as standalone names while allowing legitimate Unicode identifiers.

---

19. Unicode Joiners and Format Characters

Unicode format characters MUST NOT automatically become identifier characters.

Only characters permitted by the normative identifier property set may participate in identifiers.

If a future language version permits specific joiner/format characters, that decision MUST be explicitly specified and tested.

The lexer MUST NOT accept arbitrary invisible characters merely because Unicode classifies them as format characters.

This is important for:

- source security;
- visual ambiguity;
- identifier spoofing;
- reproducible builds;
- code review.

---

20. Confusable Identifiers

Zamani does not prohibit legitimate Unicode identifiers merely because they are visually confusable.

However, tooling SHOULD provide diagnostics or warnings for suspicious confusable identifiers.

For example:

foo
fοο

where characters from different scripts may appear visually similar.

Such warnings are tooling/security behavior.

They MUST NOT silently change the identifier's semantic identity.

---

21. Bidirectional Text

Identifiers MAY contain Unicode characters from scripts that participate in bidirectional text.

The compiler/toolchain SHOULD detect suspicious bidirectional control characters.

Bidirectional control characters MUST NOT silently alter the logical identifier tokenization.

Security diagnostics SHOULD identify suspicious source constructs.

The source representation MUST remain deterministic.

---

22. Reserved Keywords

The current implementation contains a very large keyword table in "src/lexer.rs".

That table MUST NOT automatically define the permanent language keyword set.

The authoritative keyword policy belongs to the language specification and canonical grammar.

A word is a reserved keyword only when the language specification explicitly reserves it.

The lexer MUST recognize only the authoritative keyword set.

---

23. Keyword and Identifier Separation

The lexical pipeline is:

identifier-shaped source
        │
        ▼
identifier scanner
        │
        ▼
exact keyword lookup
        │
        ├── reserved keyword → keyword token
        │
        └── otherwise → Identifier

Keyword lookup MUST happen after complete identifier scanning.

The lexer MUST NOT split a longer identifier because it contains a keyword.

For example, if:

fn

is reserved, then:

fn

is a keyword, but:

fn_value
function
fn2

remain identifiers unless separately reserved.

---

24. Keyword Matching

Keyword matching MUST be:

- exact;
- case-sensitive;
- performed on the complete identifier lexeme;
- deterministic.

The lexer MUST NOT use substring matching.

Incorrect:

if "fn" appears anywhere in identifier

Correct:

complete lexeme == "fn"

---

25. Existing Keyword Problem

The current "src/lexer.rs" includes words such as:

quantum
qubit
circuit
agent
remember
recall
learn
wisdom
noise
fidelity
surface
parity
omniversal
payment
gateway
graphics
video
business

among its keyword mappings.

These MUST be reviewed against the authoritative language specification.

A domain concept does not automatically require a reserved keyword.

For example:

qubit

may be a built-in type name in one language design, while:

quantum

may be an ordinary namespace or declaration name.

The grammar and specification must decide.

The identifier contract MUST NOT independently declare domain-specific words reserved.

---

26. Built-in Names Are Not Automatically Keywords

Zamani SHOULD distinguish:

keyword

from:

built-in name

For example, a type such as:

Qubit

may be represented as a built-in semantic type name rather than requiring the lexer to produce a special keyword token.

This reduces unnecessary lexical coupling.

The distinction is:

keyword
    → syntax-control word

built-in name
    → semantic declaration supplied by the language environment

user identifier
    → source-defined name

This separation is particularly important for POCO-REAF and dialect extensibility.

---

27. No Domain-Specific Identifier Restrictions

Identifiers MUST be equally usable across:

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
- embedded;
- accelerator;
- scientific;
- systems;
- parallel/HPC;
- future domains.

The lexer MUST NOT have separate identifier rules such as:

quantumIdentifier
gpuIdentifier
hdlIdentifier
aiIdentifier
networkIdentifier

unless a specific syntactic namespace requires an independently specified lexical form.

The default identifier mechanism is universal.

---

28. Qualified Names

A qualified name is composed from identifiers and namespace separators.

The canonical semantic concept is:

identifier
    .
identifier
    .
identifier

or the language-defined path separator.

Examples:

math.linear
quantum.state
hardware.capability
ai.model
network.protocol
module.submodule.symbol

Qualified-name syntax MUST be defined by the core/name/path grammar rather than duplicated by every domain.

The identifier component itself remains governed by this document.

---

29. Qualified Name Depth

There is no language-defined maximum namespace depth.

Therefore:

a
a.b
a.b.c
a.b.c.d
...

remain conceptually valid.

An implementation MAY encounter resource limits, but those limits MUST NOT become grammar restrictions.

This supports arbitrarily large module/package hierarchies subject to available resources.

---

30. Empty Qualified-Name Components

The following MUST NOT be valid ordinary qualified names:

.a
a.
a..b
..

unless a separate, explicitly specified syntax assigns those forms a different meaning.

A qualified name consists of complete identifier components.

---

31. Numeric Components

A qualified-name component MUST obey identifier rules.

Therefore:

module.123

is not an ordinary identifier-qualified name.

If numeric indexing is required, it must use the appropriate expression syntax:

module[123]

rather than pretending that "123" is an identifier.

---

32. Namespace Separators

The namespace separator is a syntax concern shared with:

grammar/core/
grammar/modules/
grammar/modules/namespaces
grammar/core/paths

These files MUST consume the identifier contract rather than redefine identifier characters.

A future separator such as:

::

must not create a second identifier lexical grammar.

---

33. Raw Identifiers

If Zamani supports identifiers that coincide with reserved keywords, the syntax MUST be explicitly defined as a raw/escaped identifier mechanism.

A preferred future form is:

r#keyword

or another Zamani-defined equivalent.

The exact syntax MUST be standardized before implementation.

The important semantic rule is:

raw identifier

represents a user identifier whose semantic name is:

keyword

while allowing it to occur in identifier positions.

Raw identifiers MUST NOT bypass Unicode identifier validation.

They change keyword interpretation, not identifier validity.

---

34. Escaped Identifiers

Zamani SHOULD NOT initially introduce arbitrary escaped identifiers unless a concrete interoperability requirement exists.

If escaped identifiers are introduced later:

1. the escape syntax MUST be unambiguous;
2. source spelling MUST be preserved;
3. decoded identifier semantics MUST be deterministic;
4. Unicode validation MUST still apply;
5. keyword handling MUST be explicit;
6. diagnostics MUST identify both source spelling and semantic spelling.

An arbitrary escape mechanism must not become a second language inside identifiers.

---

35. Keywords Inside Qualified Names

Keyword handling MUST be context-aware at the parser level where necessary.

For example, if:

type

is reserved, then:

module.type

must either:

1. be rejected as a reserved keyword component; or
2. be accepted only through an explicitly defined raw-identifier mechanism.

The lexer MUST NOT silently reinterpret reserved keywords as ordinary identifiers merely because they occur after ".".

---

36. Identifier Contexts

The same identifier lexical token may occur in:

- variable declarations;
- function declarations;
- type declarations;
- module declarations;
- package declarations;
- imports;
- exports;
- generic parameters;
- fields;
- methods;
- traits;
- interfaces;
- resources;
- capabilities;
- quantum objects;
- logical qubits;
- hardware resources;
- HDL signals;
- AI models;
- datasets;
- network endpoints;
- security policies;
- macros;
- dialects;
- interoperability declarations.

The parser/semantic layer determines the context.

The lexer does not.

---

37. "self", "this", and Similar Names

Special receiver names such as:

self
this
super

must be explicitly categorized by the language specification.

They may be:

- reserved keywords;
- contextual keywords;
- built-in names;
- ordinary identifiers with special semantic interpretation.

The lexer must not invent special treatment independently of the specification.

The current implementation already has dedicated tokens for "self", "this", and "super"; those classifications therefore require reconciliation with the canonical language contract.

---

38. Contextual Keywords

Zamani SHOULD use contextual keywords where doing so materially improves extensibility.

A contextual keyword is treated as a keyword only in a specified syntactic context.

This is useful for domain expansion because it prevents every future computing concept from consuming a permanent global identifier.

Example:

model

could be contextual to a model declaration rather than globally preventing all user declarations named "model".

The exact contextual-keyword list MUST be maintained by the language specification.

---

39. Identifier Namespaces

Semantic namespaces are not lexical namespaces.

The same lexical identifier may be used in different semantic namespaces where the language permits:

type Foo
fn Foo(...)
module Foo

Whether such declarations conflict is determined by semantic name-resolution rules.

The lexer must not reject them.

---

40. Shadowing

Identifier shadowing is a semantic issue.

The lexer MUST accept:

let value = 1;

{
    let value = 2;
}

even if the semantic rules later prohibit or constrain shadowing.

The lexer MUST NOT maintain scope state merely to determine whether a name has already appeared.

---

41. Forward References

The lexer MUST support identifiers regardless of declaration order.

For example:

fn first() {
    second()
}

fn second() {}

The lexer does not need to know that "second" is declared later.

---

42. Unicode Security Policy

The toolchain SHOULD provide diagnostics for:

- mixed-script identifiers;
- invisible characters;
- bidi controls;
- suspicious confusables;
- noncanonical source representations;
- unusual Unicode categories.

These SHOULD be diagnostics rather than automatic source rewriting.

Security tooling must not alter program semantics without explicit user action.

---

43. Identifier Token Payload

The lexical token should conceptually contain:

Token {
    kind,
    source span,
    source spelling,
}

The token SHOULD NOT require eagerly allocating a second owned copy of every identifier if the implementation can safely retain a source-backed representation.

However, any optimization MUST preserve:

- source lifetime correctness;
- thread safety where required;
- deterministic diagnostics;
- AST ownership requirements.

The implementation must remain safe Rust.

---

44. Avoiding Unnecessary Allocation

The current lexer stores:

literal: String

for tokens.

That design may be retained for compatibility.

However, the production lexer SHOULD prefer source-backed or span-backed identifier representations where repository-wide ownership contracts permit them.

If the existing public token API requires "String", conversion should occur at the established ownership boundary rather than creating unnecessary intermediate allocations.

This optimization MUST NOT change token semantics.

---

45. Keyword Lookup

The current lexer uses:

HashMap<String, TokenType>

for keywords.

This is functionally possible but should not be treated as the final identifier architecture.

Production requirements are:

1. deterministic keyword lookup;
2. exact matching;
3. no accidental keyword aliases;
4. no per-character semantic processing beyond lexical scanning;
5. no dependence on machine size;
6. no unsafe code.

A static table or equivalent deterministic lookup mechanism MAY be used if it remains compatible with Rust 1.97.1 and repository dependencies.

The identifier specification does not mandate a particular data structure.

---

46. Keyword Aliases

Aliases such as:

null
nil

must not be added merely because the lexer currently maps them to the same token.

If both spellings are language keywords, they must be specified as such.

If only one is normative, the other MUST remain an identifier or be rejected according to the compatibility policy.

The lexer must not silently create language synonyms.

---

47. Unicode Case Folding Is Forbidden

The following MUST NOT be treated as equivalent:

Foo
foo
FOO

No locale-sensitive case folding is permitted.

This guarantees deterministic behavior across:

- operating systems;
- locales;
- machines;
- compiler hosts.

---

48. Locale Independence

Identifier recognition MUST NOT depend on the host locale.

A source file must lex identically regardless of:

- machine locale;
- operating-system language;
- timezone;
- terminal encoding configuration;
- filesystem locale.

UTF-8 and the specified Unicode identifier properties define the lexical behavior.

---

49. Filesystem Independence

Identifiers MUST NOT be restricted to the naming rules of:

- Windows filesystems;
- POSIX filenames;
- macOS filesystems;
- object-store keys;
- environment variables.

A module/path resolver may later impose external naming constraints when mapping language modules to external resources.

Those constraints are not identifier grammar rules.

---

50. Backend Symbol Independence

A compiler backend may have symbol restrictions.

Examples include:

- linker symbol character restrictions;
- object-file limits;
- ABI naming rules;
- target assembler restrictions.

The compiler MUST perform target-specific name mangling or encoding downstream.

Portable Zamani source MUST NOT be restricted to the least capable backend.

This is a direct POCO-REAF requirement.

---

51. Name Mangling

Name mangling is downstream of lexical analysis.

The identifier:

quantum_state

must remain that source-level identifier.

A backend may internally produce something like:

_Zamani_...

but that transformation belongs to compiler/backend infrastructure.

The source identifier specification must not describe backend mangling as part of the lexical grammar.

---

52. Reserved Prefixes

Zamani SHOULD minimize globally reserved identifier prefixes.

A prefix may be reserved only when required for:

- compiler-generated symbols;
- language syntax;
- macro hygiene;
- ABI/tooling integration;
- future compatibility.

Any reserved prefix MUST be explicitly documented.

The compiler MUST NOT silently reject arbitrary names because they resemble internal symbols.

---

53. Compiler-Generated Names

Compiler-generated identifiers must not collide semantically with user identifiers.

The implementation should use an internal symbol representation or hygienic/generated namespace.

Generated names MUST NOT require source-level reserved limits.

This is particularly important for:

- macro expansion;
- generic specialization;
- quantum decomposition;
- HDL elaboration;
- loop lowering;
- distributed lowering;
- accelerator generation;
- compiler-generated temporaries.

---

54. Macro Hygiene

Macro systems must distinguish:

user-written identifier

from:

generated identifier

without changing the lexical identifier rules.

Macro hygiene belongs to:

grammar/macros/

and downstream syntax/semantic infrastructure.

The identifier contract supplies the lexical foundation only.

---

55. Metaprogramming

Reflection and code-generation systems must create identifiers through the same canonical identifier validation rules.

Generated source must not bypass lexical validation.

A metaprogram may construct:

identifier

but the resulting identifier must satisfy the same language contract as handwritten source.

---

56. Dialects

Dialect-specific syntax MUST NOT redefine the universal identifier character set.

A dialect may introduce:

- contextual keywords;
- attributes;
- namespaces;
- semantic names.

It MUST NOT create a competing universal identifier grammar.

This preserves interoperability among:

classical
quantum
HDL
AI
data
networking
security
distributed

and future dialects.

---

57. Quantum Integration

Quantum identifiers use the same universal identifier rules.

Examples:

q
qreg
logical_qubit
ancilla
state
observable
measurement

The identifier grammar MUST NOT impose:

q0
q1
q2
...

as a fixed universe.

Physical qubit identifiers belong to hardware realization and routing.

Logical quantum names remain source-level names.

The resulting semantic representation flows toward:

quantum::ir

rather than a lexer-specific quantum identifier system.

The canonical grammar itself explicitly preserves "quantum::ir" as the downstream quantum semantic boundary.

---

58. HDL Integration

HDL names use the same identifier rules.

Examples:

clock
reset
data
valid
ready
pipeline
memory
accelerator
interface

The identifier system MUST NOT impose fixed signal widths or hardware dimensions.

For example:

data_width

is an identifier.

A width value is semantic data.

Identifier syntax must not encode an artificial maximum width.

---

59. Hardware Integration

Hardware resource names use the same identifier system.

Examples:

compute
memory
accelerator
quantum_device
interconnect
capability
topology

Identifiers must remain abstract enough to support resource negotiation and target realization.

The identifier:

gpu0

is lexically valid if the source author wants that name.

The language must not infer that the platform contains a device numbered zero.

---

60. Distributed Computing Integration

Distributed identifiers may name:

node
service
worker
actor
channel
partition
replica
cluster
endpoint

No fixed number of distributed entities is implied by their identifiers.

The identifier system MUST remain independent of actual deployment topology.

---

61. AI/Data Integration

Identifiers may name:

model
dataset
tensor
feature
embedding
agent
policy
checkpoint
optimizer
inference
training

These remain ordinary lexical identifiers unless explicitly reserved by the language specification.

AI framework names MUST NOT automatically become Zamani keywords.

---

62. Networking Integration

Network-related names use the same identifier system.

For example:

server
client
endpoint
protocol
channel
route
service

Network addresses themselves are data or typed values, not identifiers.

Therefore:

server

and:

192.0.2.1

are fundamentally different lexical categories.

---

63. Security Integration

Security names use the same identifier rules:

principal
policy
permission
capability
credential
signature
key
identity

Secrets MUST NOT be encoded into identifier semantics.

Identifier spelling must not leak secret material through diagnostics or compiler-generated logs.

---

64. Resource and Capability Integration

The resource system may use identifiers such as:

memory
compute
quantum
tensor
communication
fault_tolerance
mid_circuit_measurement

These identifiers express semantic names.

The lexer does not determine whether the requested capability exists.

That is a semantic/resource-analysis concern.

---

65. No Hardware-Derived Identifier Limits

The following are explicitly prohibited as language rules:

maximum 8-character hardware identifiers
maximum 32-character device identifiers
maximum 64-character symbols
maximum 1024 namespaces
maximum 4096 identifiers

Such constraints would violate the language's scalability objective.

---

66. Identifier Scalability

The identifier implementation MUST be capable of processing identifiers from tiny examples:

x

through very large identifiers limited only by available implementation resources.

The language must conceptually support:

x
xxxxxxxx...

without a grammar-level finite maximum.

A compiler MAY reject an excessively large input because the host cannot allocate enough memory.

That is an implementation/resource failure, not a language semantic limit.

---

67. Total Number of Identifiers

There is no language-level limit on the number of identifiers in a program.

Programs may contain:

1 identifier

or:

millions/billions of identifiers

subject to available resources.

The compiler must not encode a fixed symbol-count constant into lexical validation.

Scaling mechanisms belong to:

- parser architecture;
- symbol table;
- memory management;
- incremental compilation;
- module loading;
- compiler resource management.

---

68. Identifier Depth

There is no language-level maximum nesting depth for names.

However, recursive parser implementations must avoid accidental stack exhaustion for malicious or extremely deep source.

Where practical, implementation should use iterative processing or bounded internal mechanisms with clear diagnostics.

A parser implementation limit MUST NOT be presented as a grammar restriction.

---

69. Error Classification

Identifier errors MUST be distinguishable from:

- invalid UTF-8;
- invalid character;
- malformed literal;
- malformed escape;
- reserved keyword misuse;
- malformed qualified name;
- parser-context error;
- semantic name-resolution error.

The lexer should report the narrowest accurate error category available.

---

70. Invalid Identifier Start

An invalid identifier start must produce a deterministic lexical result.

Example:

123abc

The lexer should tokenize the numeric prefix according to numeric-literal rules and then process the remaining source according to the lexical grammar.

It must not silently manufacture an identifier token covering invalid characters.

Exact diagnostic behavior must be consistent with numeric literal and parser contracts.

---

71. Invalid Identifier Continuation

If a valid identifier is followed by a character that cannot continue it, the lexer must terminate the identifier and reprocess the following character under the appropriate lexical rule.

Example:

value+

must produce:

Identifier("value")
Plus

rather than:

Identifier("value+")

---

72. Keyword Boundary

Keyword recognition must obey identifier boundaries.

For example, if:

if

is reserved:

if

must be a keyword, while:

ifdef
if2
if_value

must not become the "if" keyword merely because they begin with "if".

---

73. Identifier Followed by Unicode

The same boundary rule applies to Unicode.

The lexer must determine whether a Unicode character is a valid continuation according to the identifier property.

It must not use ASCII-only boundary logic.

---

74. Zero-Width and Invisible Characters

Invisible Unicode characters that are not valid identifier characters must not silently disappear from source.

They must be:

- rejected;
- diagnosed;
- or handled according to a specific lexical rule.

The lexer must never silently normalize arbitrary invisible characters into nothing.

---

75. Comments

Comment delimiters must terminate or precede identifiers according to the canonical lexer grammar.

For example:

value//comment

must not become one identifier.

The comment subsystem owns comment recognition.

This document owns only the boundary between identifiers and comments.

---

76. String Literals

Identifier characters inside strings are string contents.

For example:

"quantum_state"

does not contain an identifier token.

The identifier scanner must never enter a string literal.

---

77. Character Literals

Likewise:

'q'

is a character literal, not an identifier.

The lexer must prioritize literal recognition according to the canonical lexical grammar.

---

78. Numeric Literals

The lexer must not confuse:

123

with an identifier.

A valid identifier cannot begin with a decimal digit.

Numeric literal parsing belongs to the numeric literal specification.

---

79. Quantum Dirac Notation

Quantum notation such as:

|0⟩
|1⟩
|+⟩
|ψ⟩

must not cause identifier recognition to absorb:

|

or:

⟩

unless the surrounding lexical specification explicitly defines them as identifier characters.

The current lexer already has a separate "QuantumLiteral" token category.

That specialized token must remain separate from ordinary identifiers.

---

80. Nano Annotations

Forms such as:

@atom
@molecule

must be handled according to the annotation/attribute grammar.

The identifier component:

atom
molecule

must obey this identifier contract.

The "@" character is not itself part of the identifier.

The current lexer has a separate "NanoAnnotation" category; this must be reconciled with the canonical attribute/annotation grammar rather than creating a second identifier mechanism.

---

81. MTS / Timeline Names

Multi-timeline constructs may contain identifiers such as:

timeline
branch
history
checkpoint
world

These use normal identifiers.

A timeline identifier must not encode a fixed maximum number of timelines.

---

82. Sankofa Integration

Sankofa concepts such as:

remember
recall
learn
wisdom
ancestor

must not automatically become reserved identifiers simply because they occur in the extended design material.

Their lexical status must be established by the canonical language specification.

If implemented as keywords, they must be documented in the keyword registry.

If they are ordinary domain names, users must be able to use them as identifiers subject to normal rules.

---

83. Identifier Registry

The repository SHOULD maintain one authoritative identifier/keyword registry.

Recommended location:

grammar/lexer/keywords.md

This file should define:

- reserved keywords;
- contextual keywords;
- built-in names;
- deprecated keywords;
- aliases;
- raw identifier policy;
- version introduced;
- version deprecated;
- compatibility behavior.

"identifiers.md" defines how identifiers work.

"keywords.md" defines which names are reserved.

Neither file should duplicate the other.

---

84. Relationship to "grammar/Zamani.g4"

"grammar/Zamani.g4" remains the canonical ANTLR composition/root grammar.

Its identifier rules must consume this specification.

The root grammar currently uses:

identifier
qualifiedName

throughout declarations, modules, functions, types, expressions, and other constructs.

The identifier contract therefore becomes a cross-domain lexical dependency.

No domain grammar may redefine the universal identifier rules.

---

85. Relationship to "grammar/core/"

"grammar/core/" owns reusable language-wide name/path constructs.

It should consume:

identifier
qualifiedName
path

rather than defining another identifier character set.

Recommended ownership:

lexer/identifiers.md
    → lexical identifier semantics

core/identifiers.g4
    → canonical parser/grammar production

core/names.g4
    → names and qualified names

core/paths.g4
    → paths/module references

---

86. Relationship to "grammar/modules/"

Modules consume qualified names.

The module grammar MUST NOT redefine:

IdentifierStart
IdentifierContinue

It only determines the syntactic structure in which identifiers appear.

---

87. Relationship to "grammar/types/"

Type names use ordinary identifiers unless they are explicitly reserved built-in names.

Examples:

MyType
Matrix
Tensor
QuantumState
Device

The type grammar determines whether a name is a type.

The lexer determines only that it is lexically an identifier.

---

88. Relationship to "grammar/functions/"

Function names use ordinary identifiers.

The lexer must not distinguish:

functionName

from:

variableName

unless the language explicitly defines a different lexical category.

The parser and semantic model provide the distinction.

---

89. Relationship to "grammar/quantum/"

Quantum identifiers use the universal identifier grammar.

Quantum-specific grammar defines how those names participate in:

- register declarations;
- operations;
- states;
- observables;
- measurement;
- logical qubits;
- resource requirements.

It does not redefine identifier characters.

---

90. Relationship to "grammar/hdl/"

HDL names use the universal identifier grammar.

HDL-specific semantics determine whether a name represents:

- signal;
- port;
- register;
- net;
- module;
- interface;
- clock;
- reset;
- parameter.

---

91. Relationship to "grammar/hardware/"

Hardware names use ordinary identifiers.

Physical device IDs and topology identifiers must be modeled as semantic values or target metadata rather than being embedded into the lexical identifier mechanism.

---

92. Relationship to "grammar/resources/"

Resource and capability names use ordinary identifiers.

For example:

requires capability("quantum.mid_circuit_measurement")

may contain a string or qualified semantic capability name.

The lexer must not create special identifier categories for every capability.

---

93. Relationship to "grammar/dialects/"

Dialects may define contextual names.

They MUST NOT silently expand the global reserved-keyword set without:

- specification update;
- versioning;
- compatibility documentation;
- lexer/parser conformance tests.

---

94. Relationship to "grammar/macros/"

Macros may manipulate identifiers.

Macro expansion MUST preserve the canonical identifier validity rules.

Generated identifiers must be validated before entering the semantic pipeline.

---

95. Relationship to "grammar/metaprogramming/"

Compile-time generated identifiers must obey the same rules as source identifiers.

Reflection APIs must not expose backend-specific names as if they were source-level identifiers.

---

96. Relationship to "grammar/interoperability/"

Foreign-language interoperability may require identifiers that are not legal in a foreign language or vice versa.

The interoperability layer MUST provide explicit mapping.

Zamani identifiers remain governed by this document.

For example:

Zamani identifier
        ↓
foreign ABI name

is a lowering/mapping operation, not lexical identifier behavior.

---

97. Relationship to AST

The AST must represent identifiers as semantic names without losing source information required by diagnostics.

At minimum, the AST identifier contract must provide:

source spelling or recoverable source span
semantic identifier representation
source location

The AST MUST NOT introduce a domain-specific identifier hierarchy merely because a name occurs in quantum, HDL, AI, or another domain.

---

98. Relationship to Semantic Analysis

Semantic analysis owns:

- declaration lookup;
- scope;
- shadowing;
- visibility;
- imports;
- exports;
- overload resolution;
- type namespace;
- value namespace;
- module namespace;
- capability namespace;
- resource namespace.

The lexer MUST NOT perform these operations.

---

99. Relationship to IR

Identifiers that survive into IR must be represented according to the canonical IR contract.

The frontend must not create a second quantum-specific identifier mechanism.

For quantum constructs:

source identifier
    ↓
AST identifier
    ↓
semantic quantum entity
    ↓
quantum::ir

The lexer remains completely unaware of physical realization.

---

100. Relationship to Compiler Backends

Backend lowering may rename identifiers.

Examples:

source symbol
→ LLVM symbol
→ machine symbol
→ HDL signal
→ QPU instruction symbol

These mappings are backend concerns.

The source-language identifier specification remains target-independent.

---

101. Relationship to Runtime

Runtime identifiers may refer to:

- functions;
- resources;
- capabilities;
- services;
- handles;
- devices.

Runtime names do not retroactively change lexical rules.

---

102. Diagnostics

Identifier diagnostics SHOULD include:

- diagnostic code;
- severity;
- source span;
- source spelling;
- reason;
- suggested correction where unambiguous;
- contextual information where useful.

Example conceptual diagnostic:

Z0010: invalid identifier start

or:

Z0011: reserved keyword cannot be used as an identifier here

Diagnostic numbering must ultimately be centralized in the repository's diagnostics specification.

This document must not create a second global diagnostic registry.

---

103. Diagnostic Privacy

Identifier diagnostics MUST NOT accidentally expose:

- secrets;
- credentials;
- private keys;
- authentication material;
- protected source data.

Where source text is sensitive, tooling should use spans and bounded excerpts according to repository diagnostic policy.

---

104. Error Recovery

The lexer SHOULD continue after an invalid identifier-related character where safe to do so.

It should avoid cascading failures where one malformed character causes the entire remainder of the source file to become invalid.

Recovery behavior must remain deterministic.

---

105. Incremental Lexing

The identifier contract MUST support incremental tooling.

An IDE or incremental compiler should be able to re-lex a changed region without requiring the entire repository to be reinterpreted solely because an identifier changed.

The source span must make token invalidation deterministic.

---

106. IDE Integration

Identifiers must support:

- syntax highlighting;
- completion;
- rename;
- go-to-definition;
- find references;
- hover;
- diagnostics;
- semantic classification.

These features consume AST/semantic information.

The lexer provides the lexical foundation but must not implement semantic completion.

---

107. Formatting

A formatter must preserve identifier spelling unless an explicit rename/refactoring operation changes it.

Formatting must not:

- case-fold identifiers;
- normalize them silently;
- replace Unicode characters;
- rename identifiers to backend-compatible names.

---

108. Refactoring

A rename operation must operate on semantic symbol identity, not textual substring replacement.

For example:

foo
foobar
myfoo

must not all be renamed when only "foo" is selected.

The identifier span and semantic symbol table provide the necessary boundaries.

---

109. Serialization

AST/IR serialization may store identifier names.

Serialization formats must preserve Unicode correctly.

Serialized identifiers must not be silently converted to ASCII.

If serialization imposes escaping, that escaping belongs to the serialization format and must be reversible.

---

110. Determinism

Given identical:

source bytes
language version
Unicode identifier version
dialect configuration

the lexer MUST produce identical identifier tokenization.

Results must not depend on:

- locale;
- OS;
- CPU;
- GPU;
- QPU;
- filesystem;
- network;
- time;
- random number generator.

---

111. Reproducibility

A build using the same source and language/toolchain contract must produce the same identifier interpretation.

This is required for:

- reproducible builds;
- provenance;
- caching;
- distributed compilation;
- verification;
- deterministic replay.

---

112. Security and Homoglyph Policy

The lexical layer should recognize valid Unicode identifiers without banning legitimate international source code.

Security tooling SHOULD separately identify:

- confusable characters;
- mixed scripts;
- invisible characters;
- bidi controls.

The warning system must not alter lexical meaning.

---

113. Resource Safety

Identifier processing MUST be bounded by available implementation resources rather than arbitrary language constants.

The implementation must avoid:

- unbounded recursive processing;
- unchecked allocation;
- integer overflow in byte offsets;
- unsafe pointer operations;
- unchecked assumptions about character width.

Rust 1.97.1 safe abstractions must be used.

---

114. Integer Safety for Source Positions

Identifier length and source offsets may become very large.

The implementation must use the repository's canonical source-position types.

The lexer must not calculate byte lengths through arithmetic that can silently overflow.

If the source model cannot represent an input size, the compiler must produce a deterministic resource/position diagnostic rather than wrap around.

---

115. No "unsafe"

The implementation of identifier recognition MUST NOT require:

unsafe

No unsafe Rust may be introduced to accelerate Unicode identifier recognition.

Performance optimizations must use safe Rust.

---

116. Rust Compatibility

The implementation MUST compile under the repository's supported Rust baseline:

Rust 1.97 / Rust 1.97.1
Rust 2021

Identifier implementation must not depend on language/library features newer than the supported baseline unless the repository's Rust policy is updated first.

---

117. ANTLR Compatibility

The ANTLR representation of identifiers must remain compatible with the canonical "Zamani.g4".

The grammar must not contain a Java/Rust implementation-specific identifier rule that diverges from the Rust lexer.

ANTLR and Rust lexer behavior must be tested against the same conformance corpus.

---

118. Single Source of Truth

Identifier semantics have one normative contract:

grammar/lexer/identifiers.md

The following are derived/implementation representations:

grammar/Zamani.g4
src/lexer.rs
src/parser.rs
AST
generated parser artifacts
tests
grammar/grammar.md

No implementation file may silently introduce a different identifier definition.

---

119. Authority Hierarchy

The identifier authority hierarchy is:

Language specification
        ↓
grammar/lexer/identifiers.md
        ↓
grammar/lexer/*.md
        ↓
grammar/core/*.g4
        ↓
grammar/Zamani.g4
        ↓
lexer implementation
        ↓
parser
        ↓
AST
        ↓
semantic analysis
        ↓
IR
        ↓
compiler/runtime

"Zamani-Grammar.md" does not override this contract.

"grammar.md" does not independently define new identifier semantics.

---

120. "grammar/grammar.md" Integration

"grammar/grammar.md" is the implementation-conformance reference.

It should document the actual accepted identifier syntax generated/derived from:

identifiers.md
Zamani.g4
src/lexer.rs

If it disagrees with this normative document, the discrepancy must be reported as a conformance failure.

---

121. "grammar/Zamani-Grammar.md" Integration

"Zamani-Grammar.md" contains broad and aspirational language-design material.

Identifier features described there must not automatically become normative.

A new identifier feature must pass:

proposal
→ specification
→ grammar
→ implementation
→ AST
→ semantic analysis
→ tests
→ compatibility

before becoming stable Zamani syntax.

---

122. Existing "src/lexer.rs" Integration

The current lexer already provides:

TokenType::Identifier
Token.literal
Token.span
Lexer::keywords_map

and source-file/byte-position infrastructure.

The production implementation must preserve the public architectural intent while correcting the following:

1. identifier rules must be Unicode-aware;
2. keyword status must come from the authoritative keyword contract;
3. keyword aliases must be explicitly specified;
4. domain-specific words must not automatically become reserved;
5. identifier recognition must not use arbitrary hardware limits;
6. source spans must remain exact;
7. identifier spelling must remain recoverable;
8. case sensitivity must remain deterministic;
9. locale must not affect behavior;
10. no unsafe Rust may be introduced.

---

123. Existing Keyword Table Integration

The existing "HashMap<String, TokenType>" must be audited against "grammar/lexer/keywords.md".

For every current keyword:

lexeme
TokenType
specification status
grammar status
parser usage
AST usage
semantic usage
tests
compatibility status

must be known.

Unused or aspirational keyword tokens must not automatically remain part of the stable language.

---

124. TokenType Integration

"TokenType::Identifier" remains the canonical ordinary identifier token.

The implementation SHOULD avoid creating:

QuantumIdentifier
HDLIdentifier
AIIdentifier
HardwareIdentifier
GPUIdentifier
QubitIdentifier

unless a future lexical specification establishes a genuinely different lexical form.

Domain identity is semantic, not lexical.

---

125. Keyword Token Integration

Dedicated keyword token types are permitted when they simplify parser behavior.

However, they must be generated from the authoritative keyword registry.

There must not be:

keyword in Zamani.g4

but:

ordinary identifier in src/lexer.rs

or the reverse, without an explicitly documented compatibility state.

---

126. Boolean Literals

Words such as:

true
false

are literals if the language specifies them as boolean literals.

They are not ordinary identifiers in expression contexts.

The lexer must follow the literal/keyword authority defined elsewhere.

The identifier document only establishes that a reserved literal word cannot simultaneously become an ordinary identifier in the same lexical context unless raw/contextual identifier syntax permits it.

---

127. Nil/Null

If:

nil

and:

null

are both accepted, that must be an explicit language decision.

The current lexer maps both to the same token.

This behavior must be documented and tested rather than inferred from the implementation.

---

128. Built-in Type Names

Names such as:

int
uint
float
f16
f32
f64
f128
bool
char
str
String

must have a documented status.

They may be:

- reserved type keywords;
- built-in names;
- contextual names.

The identifier contract does not decide this.

The type-system specification and keyword registry must.

---

129. Case-Sensitive Built-ins

If:

String

is distinct from:

string

that difference must be intentional and documented.

The lexer must not case-fold them.

---

130. Generic Parameters

Generic parameters use ordinary identifiers.

Examples:

T
U
Element
Scalar
Shape
Backend
Capability

The lexer must not assign a special token type solely because an identifier is a generic parameter.

---

131. Lifetime Names

Lifetime identifiers are a related but distinct syntax.

Existing repository material includes dedicated lifetime grammar.

The lifetime grammar may consume an identifier-like component, but it must define its own delimiter/context.

For example:

'a

may be a lifetime construct if the canonical grammar specifies it.

The underlying name:

a

must remain governed by the identifier character rules.

---

132. Pattern Bindings

Pattern-bound names use ordinary identifiers.

Examples:

match value {
    Some(x) => ...
}

The lexer recognizes:

x

as an identifier.

Pattern semantics belong to the parser/semantic layer.

---

133. Destructuring

Destructured identifiers use ordinary identifier syntax.

Example:

let (left, right) = pair;

"left" and "right" are ordinary identifiers.

---

134. Attributes

In:

@attribute

the identifier after "@" must obey this contract.

The attribute grammar determines whether the attribute is:

- built-in;
- user-defined;
- dialect-defined;
- compiler-defined.

The lexer must not maintain separate attribute identifier rules.

---

135. Annotation Names

Annotation names follow the same rules.

This prevents every domain from creating incompatible annotation naming conventions.

---

136. Names in String-Based APIs

If a language feature accepts:

"identifier-name"

as a string, it is not lexically an identifier.

Semantic APIs that convert strings into identifiers must explicitly validate them.

They must not assume that arbitrary strings are valid identifiers.

---

137. Generated Source

Generated source must pass through the same identifier validator as handwritten source.

A code generator MUST NOT create invalid identifiers and rely on a downstream backend to repair them silently.

---

138. Serialization Round Trip

A valid identifier serialized and then deserialized must retain semantic identity.

The required invariant is:

decode(encode(identifier)) == identifier

subject to the serialization format's documented representation.

---

139. Versioning

Identifier rules are language-versioned.

Changes requiring compatibility review include:

- changing Unicode identifier properties;
- adding/removing reserved keywords;
- changing case rules;
- adding raw identifiers;
- changing normalization;
- changing namespace separators;
- changing keyword classification.

Adding a new reserved keyword is potentially source-breaking because previously valid user identifiers may become reserved.

---

140. Keyword Addition Policy

A new keyword MUST NOT be added merely because a new domain needs a convenient word.

Before reserving a word, the design must determine whether:

ordinary identifier

or:

contextual keyword

is sufficient.

Global keywords should be used sparingly.

This preserves long-term extensibility.

---

141. Compatibility Strategy

When a new keyword is required:

1. document it;
2. assign language version;
3. update keyword registry;
4. update "Zamani.g4";
5. update lexer keyword table;
6. update parser;
7. update AST/semantic consumers if required;
8. add positive tests;
9. add negative tests;
10. add compatibility tests;
11. update "grammar.md".

This prevents partial implementation.

---

142. Deprecating a Keyword

A deprecated keyword must not disappear abruptly.

The compatibility policy must define:

deprecated
→ warning
→ migration guidance
→ removal version

If the word becomes available as an ordinary identifier again, that must occur only in a versioned language transition.

---

143. Feature Gates

Experimental identifier features must be gated through the repository's feature/dialect/version mechanisms.

They must not silently become stable syntax.

---

144. Conformance Corpus

The identifier test corpus must include:

ASCII

x
value
_value
value_
value2
_value2

Unicode

π
λ
Δ
量子
状態
данные
δοκιμή
مرحبا

Long identifiers

very_long_identifier_...

with no artificial language maximum.

Invalid starts

1value
2foo

Case sensitivity

foo
Foo
FOO

Boundaries

fn
fn_value
fn2

Qualified names

a.b
a.b.c
module.submodule.value

Unicode qualified names

量子.状態

Security cases

- mixed scripts;
- confusables;
- bidi controls;
- invisible characters;
- combining marks.

---

145. Negative Test Requirements

Negative tests MUST include:

1. invalid identifier start;
2. invalid continuation;
3. malformed Unicode;
4. invalid UTF-8 boundary;
5. malformed raw identifier;
6. reserved keyword in identifier position;
7. malformed qualified name;
8. empty qualified-name component;
9. forbidden control character;
10. unsupported identifier escape.

---

146. Boundary Tests

Boundary tests MUST include:

- one-character identifier;
- "_";
- maximum implementation-supported identifier;
- multi-byte Unicode;
- combining sequences;
- identifier adjacent to punctuation;
- identifier adjacent to numbers;
- identifier adjacent to comments;
- identifier adjacent to operators;
- deeply qualified names;
- large numbers of identifiers.

---

147. Scalability Tests

Scalability tests must verify that the language does not introduce artificial limits.

Test dimensions include:

identifier length
number of identifiers
namespace depth
module depth
generic parameter count
field count
declaration count
source size
Unicode code-point count

Tests must distinguish:

language validity

from:

implementation resource exhaustion

---

148. Property Tests

Identifier recognition SHOULD have property-based tests for:

valid_start + valid_continue*

The following invariant should hold:

if a string satisfies the identifier grammar,
the lexer recognizes it as one identifier token

subject to keyword classification.

---

149. Round-Trip Tests

For valid identifiers:

source
→ lexer
→ token
→ formatter/source representation

must preserve identifier spelling.

---

150. Lexer/ANTLR Differential Tests

The repository should run equivalent identifier examples through:

ANTLR Zamani.g4

and:

Rust src/lexer.rs

and verify equivalent lexical classification.

This is essential because the repository currently has both a canonical ANTLR grammar and an independent Rust lexer implementation.

---

151. Parser Integration Tests

For every valid identifier test:

lexer
→ parser

must successfully produce the expected syntactic construct where the context is valid.

---

152. AST Integration Tests

Identifier-bearing syntax must map to the expected AST representation.

Examples:

variable declaration
function declaration
type declaration
module
import
export
field
generic
quantum register
HDL signal
resource
capability

must all preserve identifier identity.

---

153. Semantic Integration Tests

Semantic tests must verify:

- declaration lookup;
- scope;
- shadowing;
- imports;
- qualified names;
- Unicode names;
- keyword restrictions;
- namespace separation.

These are downstream tests, not lexer tests, but identifier conformance depends on their integration.

---

154. IR Integration Tests

Where names reach IR:

source identifier
→ AST
→ semantic entity
→ IR symbol

must remain deterministic.

Backend name mangling must not corrupt source-level identity.

---

155. Quantum Integration Tests

Quantum tests must include:

q
q0
logical_qubit
ancilla
量子
状態

and verify that identifiers remain ordinary source names while quantum semantics are resolved downstream.

No test may imply a fixed maximum number of qubits.

---

156. HDL Integration Tests

HDL tests must include:

clk
reset
data
valid
ready
pipeline_stage

and Unicode names where supported.

No identifier test may establish fixed signal counts or widths.

---

157. Distributed Integration Tests

Distributed tests must include:

node
worker
service
replica
partition
channel

without imposing a fixed node count.

---

158. AI/Data Integration Tests

Test:

model
dataset
tensor
embedding
feature
agent

as ordinary names unless explicitly reserved.

Framework names must not become lexical requirements.

---

159. Security Tests

Security conformance should verify that:

- bidi controls are detected appropriately;
- confusable warnings do not alter semantics;
- secrets are not unnecessarily echoed;
- identifier comparison is deterministic;
- Unicode handling is locale-independent.

---

160. Performance Requirements

Identifier lexing SHOULD be approximately linear in the number of source code points/bytes processed.

The implementation must not repeatedly rescan the same identifier.

Keyword classification should occur after a single identifier scan.

Unicode validation should not require quadratic processing.

---

161. Memory Requirements

Memory usage should scale approximately with:

input size
+
token retention requirements
+
diagnostic requirements

and not with a fixed maximum identifier universe.

The lexer should avoid unnecessary duplication of long identifier strings.

---

162. Parallel Compilation

Identifier semantics must be deterministic under parallel compilation.

Two compiler workers processing identical source must obtain identical identifier classifications.

No global mutable keyword state should affect lexical behavior.

---

163. Distributed Compilation

Distributed compilation must use the same:

language version
Unicode identifier policy
keyword registry
dialect configuration

to produce equivalent tokenization.

---

164. Cache Keys

Identifier-containing compilation artifacts must use semantic identity plus relevant language-version information in cache keys.

A cache must not accidentally treat:

foo
Foo

as identical.

Unicode representation/version changes must invalidate incompatible cached results.

---

165. Provenance

Compiler provenance should record:

Zamani language version
identifier specification version
Unicode identifier version
keyword registry version
dialect configuration

when reproducibility requires it.

---

166. Compatibility With Existing Source

Existing valid identifiers should remain valid unless a documented language-version change explicitly changes the rule.

Keyword additions require special compatibility review because they can turn existing identifiers into reserved words.

---

167. No Automatic Renaming

The compiler MUST NOT silently rename an invalid or newly reserved identifier.

It should instead provide a diagnostic and, where appropriate, a migration suggestion.

Automatic source rewriting belongs to explicit tooling.

---

168. Tooling API

The repository should expose enough lexical information for tooling to answer:

Is this token an identifier?
What is its exact spelling?
Where is it in the source?
Is it a keyword?
Which language version defines it?

Semantic tooling may then determine:

What does it refer to?

---

169. Completion Criteria

"grammar/lexer/identifiers.md" is complete when:

- [x] purpose defined;
- [x] ownership defined;
- [x] non-ownership defined;
- [x] ASCII identifiers specified;
- [x] Unicode identifiers specified;
- [x] XID model specified;
- [x] underscore behavior specified;
- [x] digit behavior specified;
- [x] case sensitivity specified;
- [x] normalization policy specified;
- [x] keyword boundary specified;
- [x] qualified names specified;
- [x] source spelling preservation specified;
- [x] source span contract specified;
- [x] security considerations specified;
- [x] scalability requirements specified;
- [x] POCO-REAF integration specified;
- [x] Rust 1.97.1 compatibility specified;
- [x] safe-Rust/no-unsafe requirement specified;
- [x] ANTLR integration specified;
- [x] lexer integration specified;
- [x] parser integration specified;
- [x] AST integration specified;
- [x] semantic integration specified;
- [x] IR integration specified;
- [x] quantum integration specified;
- [x] HDL integration specified;
- [x] hardware integration specified;
- [x] distributed integration specified;
- [x] AI/data integration specified;
- [x] networking/security integration specified;
- [x] macros/metaprogramming integration specified;
- [x] dialect integration specified;
- [x] testing requirements specified;
- [x] positive cases specified;
- [x] negative cases specified;
- [x] boundary cases specified;
- [x] scalability cases specified;
- [x] compatibility rules specified;
- [x] hard-coding audit specified.

---

170. Hard-Coding Audit

The identifier implementation MUST NOT contain universal constants for:

MAX_IDENTIFIER_LENGTH
MAX_IDENTIFIER_COUNT
MAX_NAMESPACE_DEPTH
MAX_MODULE_DEPTH
MAX_SYMBOL_COUNT
MAX_UNICODE_CHARS
MAX_SCRIPT_COUNT
MAX_QUBITS
MAX_CPUS
MAX_CORES
MAX_THREADS
MAX_GPUS
MAX_FPGAS
MAX_NODES
MAX_MEMORY
MAX_DEVICES
MAX_TIMELINES

A resource-related constant used internally for implementation safety may exist only when it is:

1. clearly an implementation/resource safeguard;
2. not represented as a language rule;
3. documented;
4. configurable where appropriate;
5. accompanied by deterministic diagnostics.

---

171. Final Identifier Contract

The complete Zamani identifier model is:

                         Zamani Source
                              │
                              ▼
                    UTF-8 source decoding
                              │
                              ▼
                    Unicode identifier scan
                              │
                ┌─────────────┴─────────────┐
                │                           │
        reserved keyword              ordinary identifier
                │                           │
                ▼                           ▼
         Keyword token                Identifier token
                                            │
                                            ▼
                                      source span
                                            │
                                            ▼
                                          Parser
                                            │
                                            ▼
                                            AST
                                            │
                                            ▼
                                     Name resolution
                                            │
                                            ▼
                                    Semantic model
                                            │
                         ┌──────────────────┼─────────────────┐
                         │                  │                 │
                         ▼                  ▼                 ▼
                    Classical          quantum::ir       HDL/Hardware
                         │                  │                 │
                         └──────────────────┼─────────────────┘
                                            │
                                            ▼
                                     Compiler lowering
                                            │
                                            ▼
                                  target realization

The fundamental invariant is:

«An identifier is a portable source-level name, not a machine resource identifier.»

Therefore Zamani identifiers must remain independent of the number, size, topology, architecture, vendor, or physical arrangement of the resources on which a program eventually executes.

A program containing:

q

does not imply one physical qubit.

A program containing:

gpu

does not imply one GPU.

A program containing:

node

does not imply one distributed node.

A program containing:

memory

does not imply a fixed memory size.

A program containing:

tensor

does not imply a fixed tensor rank or dimension.

Identifiers name semantic entities.

Resource realization belongs downstream.

---

172. Integration Checklist

Before declaring this contract implemented, the repository integration must verify:

grammar/lexer/identifiers.md
        │
        ├── grammar/lexer/keywords.md
        ├── grammar/core/identifiers.g4
        ├── grammar/core/names.g4
        ├── grammar/core/paths.g4
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
semantic/name resolution
        │
        ▼
canonical semantic model
        │
        ├── classical IR
        ├── quantum::ir
        └── HDL/hardware IR
        │
        ▼
compiler
        │
        ▼
runtime/backend

Every edge must have a documented contract before the identifier implementation is considered production-ready.

---

173. Required Repository Changes After This File

This file itself should be independently completable.

The following files must not be silently edited as part of this document's completion.

They are downstream integration tasks:

grammar/lexer/keywords.md
grammar/core/identifiers.g4
grammar/core/names.g4
grammar/core/paths.g4
grammar/Zamani.g4
src/lexer.rs
src/parser.rs
src/frontend/ast/*
grammar/grammar.md
grammar/tests/*
grammar/validation/*

Their integration contracts have already been specified here so that those files can subsequently be completed without changing the fundamental identifier design.

---

174. Definition of Done

This file is considered complete when the following statement is true:

«Any conforming Zamani implementation can determine whether source text is an identifier using one deterministic, Unicode-aware, target-independent contract; preserve its source identity and span; distinguish it from reserved/contextual keywords according to the authoritative registry; pass it unchanged through parsing and AST construction; resolve its meaning only in semantic analysis; and carry its semantic identity into the appropriate IR without introducing machine-specific identifier limits or unsafe Rust.»

That is the identifier contract required for Zamani's:

Program Once → Compile Once → Run Everywhere, Anywhere, Forever

architecture.