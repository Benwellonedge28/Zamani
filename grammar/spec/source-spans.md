Zamani Source-Span Specification

Path: "grammar/spec/source-spans.md"
Language: Zamani
Status: Normative / Production Architecture
Specification version: 1.0
Implementation baseline: Rust 1.97 / Rust 1.97.1
Rust edition: 2021
Rust safety requirement: Safe Rust only; production compiler code MUST NOT use Rust "unsafe"
Primary portability objective: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)
Scalability objective: From the smallest supported source unit to arbitrarily large source programs, subject only to explicit implementation/resource budgets and the actual resources available
Canonical quantum semantic boundary: "quantum::ir"

---

0. Purpose

This document defines the normative source-location and source-span contract for the Zamani programming language and its compiler/tooling ecosystem.

Source spans provide stable, precise provenance from source text to:

- lexical tokens;
- parser constructs;
- frontend AST nodes;
- diagnostics;
- semantic entities;
- canonical semantic representations;
- canonical IR;
- domain IR;
- compiler transformations;
- generated artifacts;
- runtime/debugging metadata where supported;
- IDE/LSP features;
- formatter and refactoring tools;
- conformance tests;
- provenance and reproducibility systems.

The source-span system MUST make it possible to answer:

«Where in the original source did this compiler-visible entity originate?»

It MUST do so without coupling source locations to:

- CPU architecture;
- operating-system word size;
- target machine;
- GPU;
- FPGA;
- QPU;
- physical qubit;
- hardware topology;
- backend;
- scheduling;
- routing;
- QEC;
- ZQN;
- runtime resource availability.

Source spans are provenance metadata, not computational semantics.

---

1. Scope

This specification covers:

1. source-file identity;
2. source byte positions;
3. source ranges;
4. source spans;
5. line/column mapping;
6. Unicode handling;
7. UTF-8 boundaries;
8. multi-byte characters;
9. tabs;
10. CRLF and LF handling;
11. Unicode line separators;
12. EOF positions;
13. zero-width spans;
14. dummy/missing spans;
15. span containment;
16. span intersection;
17. span joining;
18. span ordering;
19. span validation;
20. source-file revisions;
21. generated source;
22. macro expansion;
23. included/imported source;
24. diagnostics;
25. AST provenance;
26. semantic provenance;
27. IR provenance;
28. source maps;
29. incremental compilation;
30. deterministic behavior;
31. large-source scalability;
32. resource-bounded operation;
33. serialization;
34. compatibility;
35. testing;
36. implementation integration.

This specification does not define:

- program semantics;
- type semantics;
- ownership;
- borrowing;
- effects;
- resource allocation;
- quantum semantics;
- QEC;
- ZQN;
- routing;
- scheduling;
- calibration;
- hardware topology;
- backend behavior;
- runtime execution semantics.

Those remain owned by their respective specifications.

---

2. Architectural Position

The source-span system sits at the bottom of the entire frontend provenance chain:

source bytes
    │
    ▼
SourceFile
    │
    ▼
lexical scanner
    │
    ▼
Token + Span
    │
    ▼
parser
    │
    ▼
AST node + Span
    │
    ▼
structural validation
    │
    ▼
semantic analysis
    │
    ▼
semantic entity provenance
    │
    ▼
canonical semantic model
    │
    ▼
canonical IR + provenance
    │
    ├── classical IR
    ├── quantum::ir
    └── HDL/hardware representation
    │
    ▼
optimization/lowering
    │
    ▼
routing/scheduling/resilience/QEC/ZQN
    │
    ▼
HAL/backend/runtime

The span system MUST NOT move semantic responsibility upward or target-specific responsibility downward.

---

3. Authority

The source-span authority relationship is:

grammar/DESIGN.md
        │
        ▼
grammar/spec/source-spans.md
        │
        ├───────────────┬──────────────────┐
        ▼               ▼                  ▼
src/source_map.rs   src/lexer.rs      src/parser.rs
        │               │                  │
        └───────────────┴──────────────────┘
                        │
                        ▼
                    src/ast/
                        │
                        ▼
                 semantic analysis
                        │
                        ▼
                   canonical IR

The following files have the following responsibilities:

File/component| Responsibility
"grammar/DESIGN.md"| Overall grammar architecture
"grammar/spec/source-spans.md"| Normative source-span contract
"grammar/spec/lexical.md"| Lexical source/token behavior
"grammar/spec/syntax.md"| Parser-visible syntax
"src/source_map.rs"| Executable source-file/span infrastructure
"src/lexer.rs"| Token spans
"src/parser.rs"| Syntactic construct spans
"src/ast/"| AST provenance
semantic layer| Semantic provenance
IR layer| IR provenance
diagnostics| Human-facing location rendering
IDE/LSP tooling| Editor-facing source locations
"grammar/grammar.md"| Current implementation conformance
"grammar/Zamani-Grammar.md"| Historical/proposed language material

No generated documentation, historical grammar, backend, or runtime may silently redefine this contract.

---

4. Fundamental Definitions

4.1 Source file

A source file is a logical compilation input identified by a "FileId".

A source file consists of:

FileId
logical name
source bytes
decoded UTF-8 text
revision identity
line-index metadata

A source file MUST have stable identity within a compilation/session.

---

4.2 Byte position

A byte position identifies an offset into the UTF-8 source representation.

A byte position is measured in bytes, not Unicode scalar values, grapheme clusters, display cells, or UTF-16 code units.

For a source:

abc

the positions are:

0 1 2 3
^     ^
start EOF

For UTF-8:

π

the byte range is larger than one Unicode scalar value.

Therefore:

byte offset != character count

This distinction is mandatory.

---

5. Canonical Position Model

The canonical source position is:

FileId + byte offset

A line/column value is a derived presentation coordinate.

The compiler MUST NOT use line/column as the canonical identity of source positions.

Therefore:

canonical:
    FileId + ByteOffset

derived:
    line
    column

This is essential for:

- Unicode;
- incremental compilation;
- source transformations;
- large files;
- deterministic diagnostics;
- efficient indexing;
- source maps.

---

6. Byte Offset Requirements

The current repository uses:

pub struct BytePos(pub u32);

The production architecture MUST treat the choice of integer representation as an implementation detail rather than a language-level source-size limit.

The source-span abstraction MUST NOT establish an artificial maximum source size.

If the executable implementation requires a wider offset representation to satisfy the supported compilation/resource envelope, the implementation MUST use a representation capable of representing every supported source offset.

The language specification MUST NOT define a maximum source size based on:

u32
usize
u64

or any particular host representation.

The important invariant is:

0 <= start <= end <= source_byte_length

for every valid concrete span.

---

7. No Artificial Source-Size Limit

The grammar MUST NOT impose:

MAX_SOURCE_BYTES
MAX_FILE_SIZE
MAX_LINE_COUNT
MAX_LINE_LENGTH
MAX_COLUMN
MAX_IDENTIFIER_LENGTH
MAX_TOKEN_LENGTH

as language restrictions.

A compiler MAY impose operational resource budgets.

Those are implementation policies and MUST remain separate from language validity.

For example:

compilation memory budget exceeded

MUST NOT be transformed into:

invalid source span

or:

source program too large for Zamani

unless the actual language specification intentionally defines such a semantic restriction.

---

8. Span Structure

A concrete source span represents a half-open interval:

[start, end)

where:

start <= end

The span includes "start" and excludes "end".

Therefore:

let x = 42;

A token covering "42" has:

start = byte offset of '4'
end   = byte offset immediately after '2'

The span length is:

end - start

in bytes.

---

9. Half-Open Interval Requirement

All compiler source ranges MUST use half-open intervals.

This provides unambiguous composition:

[A, B)
[B, C)

are adjacent without overlap.

For example:

foo

can be represented as:

[0, 3)

and the EOF position as:

[3, 3)

This convention MUST be used consistently across:

- lexer;
- parser;
- AST;
- diagnostics;
- source maps;
- macro expansion;
- semantic analysis;
- IR provenance;
- tooling.

---

10. Valid Span Invariants

Every concrete span MUST satisfy:

start <= end

and:

end <= source length in bytes

when the span directly references a concrete source file.

The implementation MUST NOT permit a concrete span to silently represent:

start > end

or:

end > source length

unless it is explicitly represented as an unresolved/generated/provenance span according to this specification.

---

11. Zero-Length Spans

Zero-length spans are valid.

A zero-length span has:

start == end

They are required for:

- EOF;
- insertion diagnostics;
- missing tokens;
- expected-token diagnostics;
- parser recovery;
- empty constructs;
- cursor locations;
- generated constructs with no direct source extent.

Example:

foo|

The cursor at EOF may be represented as:

[3, 3)

Zero-length spans MUST NOT be treated as invalid merely because their length is zero.

---

12. EOF Span

EOF MUST have a deterministic source position.

For a source containing "N" bytes:

EOF byte offset = N

EOF is therefore:

[N, N)

Its file identity MUST remain the actual source file.

EOF MUST NOT use a dummy span.

---

13. Dummy / Unknown Spans

The current implementation uses:

FileId(0)
BytePos(0)
BytePos(0)

as a dummy span.

The production architecture MUST preserve the distinction between:

real source position

and:

unknown/no-source position

A dummy/unknown span MUST NOT be accidentally interpreted as:

the first byte of the first source file

The preferred semantic model is an explicit distinction between:

Known(SourceSpan)

and:

Unknown

or an equivalent representation that cannot be confused with a real file position.

If backward compatibility requires "Span::dummy()", the implementation MUST document "FileId(0)" as permanently reserved for "no source file" and MUST guarantee that no real source file receives it.

---

14. Source File Identity

"FileId" identifies a logical source file within a source-map context.

"FileId" MUST NOT encode:

- operating-system file descriptors;
- memory addresses;
- physical storage locations;
- device identifiers;
- network addresses;
- process identifiers;
- backend identifiers.

A file may originate from:

- local disk;
- generated source;
- in-memory editor buffer;
- package dependency;
- embedded source;
- macro expansion;
- remote source cache.

The source-span model remains identical.

---

15. File Identity Stability

Within one compilation context:

FileId -> logical source file

MUST remain stable.

Adding another source file MUST NOT silently change the identity of an already registered file.

This is important for:

- deterministic diagnostics;
- incremental compilation;
- caching;
- IDE integration;
- serialized diagnostics;
- provenance;
- reproducibility.

---

16. Source Revision Identity

For editors and incremental compilation, source content may change while the logical file remains the same.

The implementation SHOULD therefore maintain a source revision identity separately from "FileId".

Conceptually:

FileId
    +
RevisionId

identifies a particular source snapshot.

A span referring to an old revision MUST NOT silently be applied to a new revision when offsets may no longer correspond.

The compiler/tooling layer MUST either:

1. reject stale spans;
2. translate them through an explicit edit map;
3. associate them with the original revision.

Silent reinterpretation is prohibited.

---

17. UTF-8 Canonical Representation

Zamani source text is UTF-8.

Source byte offsets MUST refer to the original UTF-8 byte sequence.

The lexical specification already establishes UTF-8 as canonical and requires invalid UTF-8 to produce a source-encoding diagnostic rather than silently replacing bytes.

Source spans MUST therefore never be based on an implicit platform encoding.

---

18. Unicode Scalar Values

The lexer operates over Unicode scalar values after successful UTF-8 decoding.

A source span still refers to UTF-8 byte offsets.

Therefore:

UTF-8 bytes
    ↓
Unicode scalar values
    ↓
tokens
    ↓
byte spans

The span representation does not change because a character occupies multiple bytes.

---

19. UTF-8 Boundary Invariant

A concrete span boundary MUST occur at a UTF-8 code-point boundary whenever the span is intended to identify source text.

The compiler MUST NOT create an ordinary source-text span that begins or ends in the middle of a UTF-8 encoded scalar value.

For example, if:

π

occupies two UTF-8 bytes, the implementation MUST NOT produce a user-facing identifier span covering only one byte of that encoding.

Internal byte scanning MAY inspect individual bytes.

User-visible source spans MUST remain valid UTF-8 boundaries.

---

20. Source Slicing

Any API equivalent to:

source[start..end]

MUST validate that:

start
end

are valid UTF-8 boundaries before constructing a "&str".

The implementation MUST use safe Rust facilities.

No unchecked UTF-8 slicing is permitted.

No "unsafe" implementation is necessary or permitted.

---

21. Line Number Model

Line numbers are one-based for user-facing diagnostics.

Therefore:

first line = 1

not:

first line = 0

The internal line-index representation MAY be zero-based, but conversion to diagnostics MUST be deterministic.

---

22. Column Model

The specification MUST explicitly distinguish:

byte column
Unicode scalar column
grapheme column
display column
UTF-16 column

These are not equivalent.

The canonical compiler column for source diagnostics SHOULD be defined as a Unicode scalar-value-based one-based column unless a tooling protocol explicitly requires another coordinate system.

Thus:

line = 1
column = 1

identifies the first Unicode scalar value on the line.

For LSP or another protocol that requires UTF-16 positions, conversion MUST occur at the tooling boundary.

The compiler's canonical source identity remains byte-based.

---

23. Tabs

Tabs MUST NOT alter byte offsets.

A tab occupies one byte in UTF-8 source when represented by U+0009.

For diagnostic rendering, a configurable display width MAY be used.

For example:

\t

MUST remain one source scalar value regardless of whether a terminal displays it as:

2 columns
4 columns
8 columns

Therefore display width MUST NOT be used for span identity.

---

24. Carriage Return and Line Feed

The source-span implementation MUST explicitly support:

LF   = U+000A
CRLF = U+000D U+000A
CR   = U+000D

The canonical language policy MUST define which sequences constitute line boundaries.

The implementation MUST NOT depend on the host operating system.

At minimum, LF and CRLF MUST be handled deterministically.

If standalone CR is accepted as a line terminator, it MUST be handled consistently.

---

25. CRLF Span Semantics

For:

a\r\nb

the line break consists of two bytes.

The next line begins after both bytes.

Therefore:

line 1:
a

line 2:
b

The CRLF sequence MUST NOT accidentally produce two logical lines.

A source span covering the entire line break MAY cover both bytes:

[\r, after \n)

while the next line starts at the byte after LF.

---

26. Unicode Line Separators

The implementation MUST have an explicit policy for Unicode line separators, including at least:

U+2028 LINE SEPARATOR
U+2029 PARAGRAPH SEPARATOR

The lexical specification and implementation MUST agree on whether these are:

- line terminators;
- whitespace;
- ordinary characters;
- invalid source characters.

They MUST NOT be inconsistently interpreted between:

lexer
source map
diagnostics
formatter
IDE

---

27. Line Index

A source file SHOULD maintain an efficient line-start index.

Conceptually:

line 1 -> byte 0
line 2 -> byte ...
line 3 -> byte ...

The existing "SourceFile" already precomputes line starts and uses binary search to locate a line.

The production implementation MUST preserve the performance property:

position -> line

should be efficiently resolvable without rescanning the entire source from byte zero.

---

28. Large Source Files

The source-map implementation MUST scale with source size.

It MUST NOT:

- scan the entire source for every diagnostic;
- scan from byte zero for every token;
- create one heap allocation per character;
- duplicate the entire source for every span;
- store copied source text inside every token;
- use quadratic algorithms for line lookup.

A practical implementation SHOULD use:

shared source storage
+
compact source positions
+
indexed line starts
+
binary search or equivalent efficient lookup

The existing use of "Arc" for source content is compatible with this architecture.

---

29. Span Memory Model

A span SHOULD remain small and copyable.

A span SHOULD contain identity and offsets rather than owning source text.

Preferred conceptual representation:

SourceSpan {
    file_id,
    start,
    end,
}

Line and column information SHOULD be derived from the source map rather than redundantly copied into every span.

If cached line/column information is retained for compatibility or performance, it MUST remain derived metadata and MUST NOT become the canonical source identity.

---

30. Existing "Span" Integration

The repository currently defines:

Span {
    file_id,
    start,
    end,
    start_line,
    start_column,
}

in "src/source_map.rs".

For production conformance:

1. "file_id", "start", and "end" are the canonical identity.
2. "start_line" and "start_column" are derived/cached presentation information.
3. Cached coordinates MUST correspond exactly to "start".
4. They MUST NOT be used to reconstruct the byte range.
5. Their meaning MUST be documented.
6. Future APIs SHOULD provide canonical conversion from "BytePos" through "SourceMap".

No existing filename needs to be renamed merely to satisfy this specification.

---

31. Span Length

The length of a valid span is:

end - start

in bytes.

The implementation MUST validate ordering before subtraction where the underlying representation could otherwise underflow.

This is especially important for public APIs.

A malformed span MUST result in:

- an error;
- rejection;
- an explicitly invalid state;

rather than integer underflow or panic caused by unchecked subtraction.

---

32. Span Containment

A span "A" contains span "B" iff:

A.file_id == B.file_id
A.start <= B.start
A.end >= B.end

This relation MUST be deterministic.

A zero-length span at the boundary is considered contained according to the half-open interval rules.

---

33. Span Intersection

Two spans intersect when their half-open ranges overlap.

For spans in the same file:

A.start < B.end
&&
B.start < A.end

A zero-length span does not create a positive-width overlap merely by sharing a boundary.

This distinction MUST be preserved for diagnostics and syntax highlighting.

---

34. Span Adjacency

Two spans are adjacent when:

A.end == B.start

and they belong to the same file/revision.

Adjacent spans MAY be joined when the resulting provenance semantics permit it.

---

35. Span Joining

A "join" operation MUST be explicit about whether the source regions need to be:

1. overlapping;
2. adjacent;
3. merely in the same file.

The preferred operation for compiler provenance is:

join(A, B)

which produces the smallest source range containing both spans when they belong to the same source revision.

The result MUST preserve the minimum start and maximum end.

Line/column metadata MUST be recalculated from the resulting start position rather than approximated using:

min(start_line)
min(start_column)

The current implementation's "Span::merge()" uses this approximation and therefore MUST NOT be treated as the normative production algorithm.

---

36. Cross-File Span Joining

Two spans belonging to different files MUST NOT be represented as one ordinary "Span".

An operation equivalent to the current:

assert_eq!(self.file_id, other.file_id)

is insufficient as a complete production API because a library-facing compiler should avoid uncontrolled panics for ordinary invalid user/tool input.

Production APIs SHOULD return:

Result

or use a dedicated multi-span/provenance representation.

A cross-file diagnostic MUST use multiple labeled spans rather than pretending the files form one source range.

---

37. Primary and Secondary Spans

Diagnostics SHOULD distinguish:

primary span
secondary span(s)

The primary span identifies the source location where the diagnostic is anchored.

Secondary spans provide related context.

Example:

error: type mismatch
       ^^^^^^^^^^^ primary

expected type declared here
      ^^^^^^^^^^^^^^^^^^^ secondary

The diagnostic system MUST NOT merge unrelated files into one span.

---

38. Multi-Span Diagnostics

A diagnostic MAY contain an arbitrary number of labeled secondary spans, subject to explicit diagnostic/resource budgets.

The language MUST NOT define:

MAX_DIAGNOSTIC_SPANS

as a semantic limit.

A compiler implementation MAY limit diagnostics under a resource budget.

Such truncation MUST be explicit and deterministic.

---

39. Diagnostic Ordering

For identical:

source
language version
compiler configuration
diagnostic policy

diagnostic spans MUST be emitted in deterministic order.

Ordering SHOULD be based on:

1. diagnostic phase;
2. source file identity;
3. source byte position;
4. stable diagnostic identifier;
5. explicit secondary-span ordering.

Hash-map iteration order MUST NOT determine user-visible diagnostic ordering.

---

40. Lexer Integration

Every token produced by "src/lexer.rs" MUST have a source span unless the token is explicitly synthetic.

The current lexer already imports:

BytePos
FileId
SourceFile
Span

and documents source-location attachment.

The production lexer contract is:

source bytes
    ↓
validated UTF-8
    ↓
lexical scanning
    ↓
Token {
    kind,
    source representation,
    span
}

The lexer MUST NOT attach approximate positions.

---

41. Token Span Boundaries

For every non-synthetic token:

token.span.start

MUST identify the first byte belonging to the token.

token.span.end

MUST identify the byte immediately after the token.

Whitespace and comments may be omitted from the ordinary token stream, but their source regions remain recoverable from the source file.

---

42. Parser Integration

The parser MUST construct AST spans from actual source spans.

A parser production's span SHOULD normally cover:

first significant token
    through
last significant token

including syntactically meaningful delimiters where appropriate.

Examples:

fn foo(...) { ... }

should normally have a span covering the declaration as a whole.

---

43. Missing Tokens During Recovery

When parser recovery inserts a conceptual token that is not present in source, it MUST NOT fabricate a misleading non-empty source span.

Instead, the parser SHOULD use:

zero-width span at recovery position

or:

synthetic/unknown provenance

depending on the construct.

Diagnostics MUST identify the location as an insertion point where appropriate.

---

44. AST Integration

The current AST implementation already places "Span" on many constructs, including statements, declarations, patterns, and other nodes.

Production AST requirements:

Every user-written AST construct that can produce:

- an error;
- a warning;
- a semantic diagnostic;
- a refactoring;
- a source-to-source transformation;

MUST retain sufficient source provenance.

An AST node MUST NOT need to reconstruct its source location from child nodes after the fact.

---

45. AST Span Ownership

An AST node owns its source provenance.

Child nodes retain their own spans.

Therefore:

parent span
    contains
child span

for ordinary syntactic containment.

This relationship MUST NOT require all child spans to be contiguous.

For example, an expression may contain comments or whitespace that do not belong to child nodes.

---

46. AST Synthetic Nodes

Some AST nodes may be created by:

- desugaring;
- parser recovery;
- macro expansion;
- implicit language rules;
- compiler transformations.

Such nodes MUST retain provenance indicating that they are synthetic or derived.

They MUST NOT falsely claim that their generated text existed literally at a source location.

---

47. Provenance Categories

The production architecture SHOULD distinguish:

Direct
Derived
Generated
Expanded
Imported
Unknown

Conceptually:

Direct(source span)
Derived(source spans)
Generated(origin)
Expanded(call site, definition site)
Imported(source file/span)
Unknown

The exact Rust representation belongs to "src/source_map.rs" and associated provenance infrastructure.

The specification requires the semantic distinction even if the initial implementation uses a simpler representation.

---

48. Macro Provenance

Macros MUST preserve both:

1. expansion location;
2. originating source location.

A macro-generated diagnostic may need to identify:

macro invocation

and:

macro definition

Therefore a single flat span is insufficient for all macro diagnostics.

The macro system MUST be able to preserve provenance chains.

---

49. Macro Expansion Stack

A generated node MAY have provenance conceptually represented as:

generated node
    ↓
macro expansion
    ↓
macro invocation
    ↓
original source

The provenance chain MUST remain finite and deterministic.

Recursive macro expansion is governed by macro expansion/resource policies, not by the source-span specification.

---

50. Included / Imported Source

Imported source files receive their own "FileId".

A span MUST NOT encode a relative import path instead of file identity.

For:

import foo;

the "import" declaration has a span in the importing file.

The imported module's declarations have spans in the imported file.

Cross-file relationships belong to semantic/module provenance.

---

51. Generated Source

Generated source MUST have explicit source identity.

Examples:

generated://...

or an equivalent internal source identifier MAY be used.

Generated source MUST NOT be silently attributed to the user's source file.

Diagnostics MAY display generated provenance differently while retaining the original source origin where available.

---

52. Embedded Languages

Zamani may contain or interoperate with:

- OpenQASM;
- HDL;
- C/C++;
- Python;
- Rust;
- WASM;
- other dialects.

Each embedded source region MUST have a source provenance model.

An embedded-language parser MUST NOT invent byte offsets unrelated to the original containing source.

Conceptually:

Zamani source span
    ↓
embedded region
    ↓
embedded-language span

A mapping between coordinate spaces MUST be maintained.

---

53. Quantum Integration

Quantum source constructs ultimately map toward the canonical "quantum::ir" boundary.

Source spans MUST remain attached to the originating Zamani syntax.

For example:

quantum operation
    ↓
AST span
    ↓
semantic quantum operation
    ↓
quantum::ir operation + provenance

The source-span system MUST NOT become a quantum IR.

It must not contain:

- physical qubit IDs;
- topology;
- QEC state;
- calibration;
- scheduling slots;
- device identity.

These remain downstream concerns.

---

54. Quantum Multi-Source Provenance

A transformed quantum operation may originate from:

one source operation

or:

multiple source operations

after decomposition or optimization.

Therefore downstream quantum IR provenance SHOULD support multiple source spans where necessary.

Example:

source operation A ─┐
                    ├── optimized IR operation
source operation B ─┘

The resulting operation may retain:

[A span, B span]

rather than inventing a single false source location.

---

55. Classical Integration

Classical operations use the same source-span system.

No separate classical location model is permitted.

This ensures:

classical
quantum
hybrid
HDL
AI
distributed
networking

can all coexist within one source and provenance system.

---

56. HDL Integration

HDL constructs MUST retain Zamani source provenance through:

HDL syntax
    ↓
AST
    ↓
hardware semantic model
    ↓
HDL/hardware IR

Generated RTL or netlists MUST NOT be presented as though they were original Zamani source.

Where generated artifacts map back to source, source maps MUST preserve that relationship.

---

57. Hybrid Integration

Hybrid operations may contain:

classical source
quantum source
hardware source

within one logical computation.

Each nested construct retains its own span.

A parent hybrid construct MAY have one encompassing span.

The span system itself remains domain-neutral.

---

58. Semantic Analysis Integration

Semantic entities SHOULD retain source provenance.

Examples:

symbol declaration
symbol reference
type error
effect violation
resource requirement
capability requirement
quantum operation
HDL port
network endpoint declaration

Semantic analysis MUST NOT replace source provenance with internal IDs alone.

Internal IDs may identify semantic entities, but diagnostics must remain capable of returning to source.

---

59. IR Integration

IR nodes SHOULD retain provenance sufficient for:

- diagnostics;
- optimization explanations;
- verification errors;
- source mapping;
- debugging;
- profiling;
- provenance;
- generated-code mapping.

IR provenance MUST NOT be assumed to be one-to-one with AST spans.

A single IR node may originate from:

zero
one
many

source constructs.

---

60. Optimization Integration

Optimizations MUST preserve provenance.

For transformations:

A -> B

where B is semantically derived from A, B SHOULD retain provenance pointing to A.

For:

A + B -> C

C SHOULD retain both A and B provenance where diagnostics or tooling may need it.

Optimization MUST NOT create fake source locations.

---

61. Removed IR Nodes

If an optimization removes an IR node, its provenance MAY be retained in optimization metadata where useful.

For example:

dead operation
    ↓
removed

does not mean the original source location disappears from diagnostic/debugging infrastructure.

The implementation may garbage-collect provenance according to lifecycle/resource policy after no consumer can require it.

---

62. Routing Integration

Routing may transform logical quantum operations into physical operations.

The resulting physical operations MUST NOT receive the original source span as though the physical operation were literally written by the user.

Instead:

physical operation
    provenance:
        derived from source operation

must remain distinguishable.

---

63. Scheduling Integration

Scheduling adds:

- timestamps;
- durations;
- resource assignments;
- ordering.

These are not source coordinates.

A schedule entry MAY refer back to source provenance but MUST NOT modify the meaning of the original source span.

---

64. QEC Integration

QEC-generated operations are derived operations.

They MUST be distinguishable from source-written operations.

For example:

source logical operation
    ↓
QEC insertion
    ↓
physical correction operation

The correction operation may carry:

derived-from source span

but must not pretend the programmer explicitly wrote it.

---

65. ZQN Integration

ZQN fault/noise semantics may attach provenance to:

- source-level requirements;
- operations;
- diagnostics;
- generated transformations.

ZQN MUST NOT redefine the source-span coordinate system.

---

66. HAL Integration

HAL may report target-specific diagnostics such as:

required capability unavailable

The diagnostic may point to the source construct that requested the capability.

The HAL itself MUST NOT manufacture a physical source location.

---

67. Resource and Capability Diagnostics

For:

requires capability("...")

the diagnostic span should point to the source requirement.

If the target lacks the capability:

source requirement span
+
target capability diagnostic

may be combined.

The source program remains syntactically valid even when a particular target is incapable of satisfying the requirement.

This preserves POCO-REAF.

---

68. Portability

Source spans MUST NOT depend on:

- pointer width;
- target architecture;
- filesystem implementation;
- locale;
- timezone;
- GPU;
- QPU;
- number of CPUs;
- thread count.

The same source must produce equivalent source coordinates under equivalent compiler inputs.

---

69. Determinism

For identical:

source bytes
language version
source-file identity mapping
compiler lexical configuration

source spans MUST be identical.

This includes:

- token spans;
- AST spans;
- diagnostic spans;
- source-map conversions.

Hash-map order, filesystem enumeration order, thread scheduling, or hardware topology MUST NOT affect source locations.

---

70. Parallel Compilation

Compilation may become parallel.

Source-span generation MUST remain deterministic.

Parallel processing MUST NOT produce:

different FileId assignment
different span boundaries
different line/column values

merely because tasks completed in a different order.

File identity allocation SHOULD therefore be based on stable registration/order or explicit IDs rather than completion timing.

---

71. Incremental Compilation

Incremental compilation MUST preserve source-span correctness.

When a source edit occurs:

old source
   ↓
edit
   ↓
new source

the compiler MUST NOT assume that all old spans remain valid.

A source-edit mapping MAY translate unaffected ranges.

Changed ranges MUST be invalidated or remapped explicitly.

---

72. Incremental Span Translation

A source edit can be represented conceptually as:

Edit {
    start,
    old_end,
    new_end
}

For a span completely before the edit:

unchanged

For a span completely after the edit:

shifted by delta

For a span intersecting the edit:

invalidated or explicitly remapped

This logic belongs in source-map/incremental infrastructure, not the grammar.

---

73. Source Transformations

Formatter, refactoring, macro expansion, and code generation MUST NOT silently mutate the meaning of an existing span.

If source is transformed:

original span
    ↓
transformed source

a new revision/source identity MUST be used where necessary.

---

74. Serialization

Serialized spans MUST use a stable representation.

Conceptually:

file identity
start byte
end byte

Additional presentation fields such as:

line
column

MAY be serialized for convenience but MUST remain derived.

Serialized spans MUST NOT depend on:

- Rust memory addresses;
- pointer values;
- process-local references;
- "HashMap" iteration order;
- platform-specific path encoding.

---

75. Path Handling

A source span MUST identify a file through "FileId".

Human-readable rendering may display:

/path/to/file.zm

but the source-span core MUST NOT depend on raw filesystem paths as identity.

This supports:

- virtual files;
- package sources;
- remote compilation;
- reproducible builds;
- IDE buffers;
- generated files.

---

76. Path Normalization

Path normalization belongs to source-file identity management.

The source-span layer MUST NOT silently apply platform-specific path normalization that changes logical identity.

For example:

case-sensitive filesystem

and:

case-insensitive filesystem

must not cause source spans to change merely because compilation moved between hosts.

---

77. Line Rendering

Diagnostics SHOULD render source context using:

file name
line
source text
marker

The rendering layer MUST calculate display columns separately from canonical byte positions.

It MUST account for:

- Unicode;
- tabs;
- combining marks where necessary;
- terminal display behavior.

Display rendering MUST NOT modify canonical span identity.

---

78. Unicode Combining Characters

A grapheme cluster may consist of multiple Unicode scalar values.

For example:

base + combining mark

may display as one visible glyph.

The canonical source position remains based on UTF-8 byte offsets.

Therefore a diagnostic MUST NOT assume:

one glyph = one scalar = one byte

Tooling MAY provide grapheme-aware display alignment.

---

79. Bidirectional Text

Source spans remain byte-based even for bidirectional text.

Diagnostics and tooling SHOULD detect suspicious bidi controls according to the lexical/security specification.

Source-span rendering MUST NOT silently reorder or rewrite the underlying source.

---

80. Invalid UTF-8

Invalid UTF-8 MUST be rejected during source decoding.

A span for an invalid byte sequence may be represented as a byte-oriented diagnostic range because decoding has not successfully produced Unicode scalar values.

The implementation MUST still provide deterministic:

file
start byte
end byte

information.

---

81. Source Decoding Diagnostics

A decoding diagnostic MUST be distinguishable from:

invalid token
invalid identifier
invalid number
syntax error

The lexical specification already establishes that invalid UTF-8 must not silently become U+FFFD.

---

82. Comments and Whitespace

Comments and whitespace do not normally become semantic AST nodes.

Nevertheless their source ranges remain meaningful for:

- formatter;
- documentation tooling;
- source reconstruction;
- diagnostics;
- IDE features.

The source-span system therefore MUST remain capable of representing positions within comments and whitespace.

---

83. Documentation Comments

Documentation comments MAY be attached to declarations.

The documentation comment itself and the declaration MUST retain distinct spans where tooling requires them.

For example:

documentation span
        ↓
declaration span

The AST may retain a relationship without merging the two ranges.

---

84. Error Recovery

Source spans MUST remain valid during lexer/parser error recovery.

Recovery MUST NOT create:

negative-length span
out-of-file span
random span

because malformed source was encountered.

A recovery construct SHOULD use:

zero-width insertion point

or:

explicit unknown/synthetic provenance

as appropriate.

---

85. Panic-Free Span APIs

Ordinary malformed source or tool input MUST NOT cause span APIs to panic.

The current implementation uses an assertion when merging different files.

For production compiler infrastructure, public span operations SHOULD return structured errors for invalid operations.

Panics SHOULD be reserved for internal invariants that indicate compiler bugs, not ordinary source conditions.

---

86. Source Map API Contract

"src/source_map.rs" MUST provide, directly or through equivalent APIs:

register/add source file
lookup file by FileId
lookup source content
lookup source name
convert byte position -> line/column
convert line/column -> byte position where valid
validate span
extract source text for span
find line boundaries
join/compare spans safely
resolve provenance

All operations MUST be deterministic.

---

87. Byte Position API

"BytePos" SHOULD provide safe operations for:

construction
comparison
ordering
offset arithmetic
checked addition
checked subtraction

Operations that can overflow MUST use checked or explicitly bounded behavior.

The implementation MUST NOT silently wrap byte positions.

---

88. Source Span API

"Span" SHOULD provide:

new
start
end
file_id
len
is_empty
contains
intersects
is_adjacent
join
source_text
start_location
end_location

Where an operation may fail, it SHOULD return "Result"/"Option" rather than panic.

---

89. Location API

A derived location SHOULD contain:

FileId
line
column
byte_offset

The exact public Rust representation is an implementation concern, but the semantic distinction between canonical byte offset and presentation coordinates is mandatory.

---

90. Start and End Locations

For a span:

[start, end)

the start location points at:

start

The end location points at:

end

The end location may therefore be:

EOF

or the first position of the following token.

This is intentional.

---

91. Range Extraction

Given a valid concrete span:

[file, start, end)

the source-map API SHOULD provide source extraction without copying the entire source file.

For example:

source_text(span) -> &str

where safe lifetime/API design permits.

The implementation MUST validate UTF-8 boundaries.

---

92. Lifetime and Ownership

Source spans MUST NOT own source strings.

A source map owns or shares source content.

Tokens/AST nodes own spans.

This prevents:

one source copy per token
one source copy per AST node

and supports large programs.

---

93. Thread Safety

The source-span model SHOULD permit concurrent read access where compiler architecture requires it.

Shared immutable source files MAY use:

Arc

or equivalent safe ownership.

Mutable source-map operations MUST have explicit synchronization/ownership rules.

No "unsafe" synchronization is permitted.

---

94. No Unsafe Rust

Production source-span infrastructure MUST NOT use:

unsafe
unsafe fn
unsafe impl
unsafe trait
unsafe {}

The entire source-map implementation MUST be expressible in safe Rust 1.97/1.97.1.

This includes:

- UTF-8 processing;
- source slicing;
- line indexing;
- span arithmetic;
- caching;
- serialization;
- parallel access.

---

95. Rust 1.97 / 1.97.1 Compatibility

Implementation MUST compile under the repository's declared Rust baseline:

Rust 1.97
Rust 1.97.1
Edition 2021

The specification MUST NOT require unstable language features.

Dependencies introduced for source spans MUST justify their necessity.

The core span representation SHOULD preferably remain dependency-light.

---

96. No Hardware Coupling

The source-span implementation MUST NOT reference:

CPU
GPU
FPGA
QPU
qubit
core
thread
node
accelerator
device
topology

as part of source-coordinate semantics.

A source span identifies source, not hardware.

---

97. No Artificial Span Count Limits

The language MUST NOT define:

MAX_SPANS
MAX_AST_SPANS
MAX_DIAGNOSTICS
MAX_FILES
MAX_LINES

as semantic language limits.

Implementation budgets MAY exist.

They must be:

explicit
configurable where appropriate
observable
deterministic
separate from language validity

---

98. Resource-Bounded Operation

A compiler may operate under:

memory budget
diagnostic budget
compilation-time budget
source cache budget
incremental cache budget

If a budget is exhausted, the compiler MUST distinguish:

resource exhaustion

from:

invalid source span

This preserves the POCO-REAF distinction between language validity and target/compiler resource feasibility.

---

99. Extremely Large Source Programs

The implementation SHOULD use algorithms whose cost scales approximately with:

source size
+
number of requested lookups

rather than:

source size × number of diagnostics

Line lookup SHOULD be indexed.

Repeated source extraction SHOULD avoid unnecessary whole-file copying.

Source text SHOULD be shared.

---

100. Small Source Programs

The same architecture MUST work efficiently for tiny sources:

x

or:

fn main() {}

No heavyweight target-specific initialization should be required merely to calculate source spans.

---

101. Infinite / Unbounded Conceptual Scalability

"Infinity" in POCO-REAF means that the language does not impose an artificial finite machine-derived ceiling.

Physical implementations remain finite.

Therefore:

conceptual language capacity

is separated from:

available compiler memory
available storage
available target resources

Source spans follow the same principle.

The language does not define a machine-derived maximum number of source bytes, lines, spans, files, or diagnostics.

---

102. Source Span and POCO-REAF

Source provenance MUST remain invariant across target compilation.

The same source:

program.zm

may compile for:

CPU
GPU
FPGA
QPU
distributed system
heterogeneous system
future target

The source span of:

requires capability(...)

does not change merely because the selected target changes.

Only target-specific diagnostics may differ.

---

103. Target-Specific Diagnostics

A target may produce:

capability unavailable
resource insufficient
unsupported lowering

These diagnostics MAY point back to the original source span.

The source span MUST remain target-independent.

---

104. Source Span Versus Resource Realization

For:

requires qubits >= n

the source span points to the requirement.

It does not point to:

physical qubit 0
physical qubit 1
...

because those are downstream realization details.

---

105. Source Span Versus Physical Mapping

A mapping such as:

logical q -> physical q17

is downstream metadata.

It may retain:

source provenance = source span of q

but the physical ID MUST NOT become part of the source span.

---

106. Source Span Versus Scheduling

A schedule may contain:

start_time
duration
resource
dependency

These are not source coordinates.

A schedule entry MAY reference source provenance, but its scheduling metadata MUST remain separate.

---

107. Source Span Versus Optimization

Optimization may change:

one source operation

into:

many target operations

or:

many source operations

into:

one optimized operation

The source-span model MUST support both directions through provenance.

---

108. Source Span Versus Generated Code

Generated code should carry source maps where feasible.

For example:

Zamani source
    ↓
generated LLVM/HDL/QIR/etc.

Generated instructions may point back to the source construct from which they originated.

The generated representation MUST NOT replace the canonical Zamani source location.

---

109. Interoperability

For OpenQASM, QIR, HDL, C/C++, Python, Rust, WASM, or other formats:

Zamani span
    ↓
foreign representation

must remain traceable where source mapping is supported.

The interoperability layer owns the coordinate transformation.

The source-span specification owns the canonical Zamani coordinate system.

---

110. IDE / LSP Integration

LSP commonly uses line/character coordinates rather than raw byte positions.

The adapter MUST explicitly convert:

Zamani byte position
        ↓
Unicode/LSP coordinate

rather than changing the canonical span model.

If UTF-16 coordinates are required, conversion MUST happen at the LSP boundary.

The core compiler MUST NOT adopt UTF-16 as its source identity merely for editor compatibility.

---

111. Formatter Integration

The formatter MUST preserve source provenance where it produces diagnostics or mappings.

Formatting may produce a new source revision.

A formatter MUST NOT assume that old spans remain valid after changing source layout.

---

112. Refactoring Integration

Rename/move/extract/refactor tools MUST operate on source spans validated against the correct source revision.

A stale span MUST NOT silently modify a different source region.

---

113. Syntax Highlighting

Syntax highlighting MAY use token spans.

It MUST NOT infer token ranges from:

display column

or:

character count

when UTF-8 byte offsets are available.

---

114. Diagnostics and Source Snippets

A diagnostic renderer SHOULD obtain source snippets through "SourceMap".

It MUST NOT store copied source text in every diagnostic by default.

For large diagnostics, snippets MAY be lazily extracted.

---

115. Diagnostic Labels

A labeled span SHOULD contain:

span
label
severity/context

The label MUST not alter the span.

Labels may identify:

expected here
defined here
used here
declared here
required here
conflicts here

---

116. Diagnostic Suggestions

Suggestions MAY include replacement ranges.

Every replacement range MUST be:

valid
same source revision
UTF-8 boundary aligned

unless the suggestion explicitly targets a generated/foreign coordinate space.

---

117. Multiple Replacement Edits

A code action may contain multiple edits.

Edits MUST have deterministic ordering and MUST either:

- be non-overlapping;
- have explicitly defined composition rules.

Overlapping edits MUST NOT be applied in an unspecified order.

---

118. Source Provenance Through Diagnostics

When an error originates downstream:

backend
HAL
QEC
routing
scheduling
semantic analysis

the diagnostic SHOULD retain the nearest meaningful user source provenance.

Example:

source operation
    ↓
semantic requirement
    ↓
routing failure

The user-facing error may point to the original operation while separately describing the target failure.

---

119. Diagnostics Must Not Lie

A diagnostic MUST NOT claim:

line 10

if the actual source location is line 11.

It MUST NOT highlight:

physical operation

as though that operation was literally written by the programmer.

It MUST distinguish:

written source

from:

compiler-generated source

where the distinction affects interpretation.

---

120. Span Ordering

Within the same source revision, spans MUST have a total ordering based on:

start byte
then end byte

File ordering MUST be separately defined.

The ordering MUST be deterministic.

---

121. Cross-File Ordering

A compilation unit containing multiple files requires stable file ordering.

The ordering SHOULD be based on:

1. explicit source registration identity;
2. canonical logical source identity where required;
3. stable compilation-unit ordering.

It MUST NOT depend on:

filesystem enumeration order
HashMap iteration order
thread completion order

---

122. Equality

Two source spans are equal iff their canonical identities are equal:

same source identity/revision
same start
same end

Cached:

line
column

must not cause unequal spans when their canonical positions are identical.

---

123. Hashing

Hashing MUST be based on canonical identity.

The hash MUST NOT include:

- cached display values that can be recomputed;
- memory addresses;
- source contents duplicated into the span;
- target information.

---

124. Span Copying

Span values SHOULD be cheap to copy.

They SHOULD be immutable after construction.

Mutation of source-span identity after attachment to an AST node is strongly discouraged and SHOULD NOT occur.

---

125. Span Immutability

Once a span is attached to:

token
AST node
semantic entity
IR node
diagnostic

the span SHOULD be immutable.

If a transformation requires a different range, it creates a new provenance record.

---

126. Source Map Mutability

The source map itself may grow as files are discovered.

Existing file identities MUST remain stable.

Once a source revision is registered, its contents MUST be immutable for the lifetime of references to that revision.

---

127. Source Content Ownership

"SourceFile" may share source content using:

Arc<String>

or an equivalent safe immutable representation.

The current repository already uses "Arc<String>" for source content.

The implementation SHOULD avoid unnecessary cloning.

---

128. Line Index Correctness

The line index MUST be derived from exactly the same source representation used by the lexer.

It MUST NOT:

- normalize newlines differently;
- normalize Unicode;
- use platform-specific decoding;
- interpret tabs as multiple bytes;
- omit the final empty line position.

---

129. Final Empty Line

For a source ending in a newline:

a\n

the source contains an empty line after the newline.

The source-map policy MUST define whether:

EOF

is reported as:

line 1, column 2

or:

line 2, column 1

The production implementation SHOULD use the latter because the EOF byte offset is at the beginning of the new empty line.

This policy MUST be shared by diagnostics, parser recovery, and IDE tooling.

---

130. Empty Source

An empty source file is valid as a source-map object.

Its source length is:

0

EOF is:

[0, 0)

at:

line 1, column 1

No special dummy span is required for EOF.

---

131. Single-Byte Source

For:

x

the source positions are:

0 = before x
1 = EOF

The identifier span is:

[0, 1)

---

132. Unicode Source Example

For:

π = 3.14

the span of "π" covers the UTF-8 bytes of the scalar.

The column is based on the defined Unicode scalar coordinate model, not on byte count.

Thus byte position and user-facing column are intentionally different coordinate systems.

---

133. Multi-Byte Identifier Example

For:

状態 = 1

the entire identifier span covers all UTF-8 bytes belonging to the identifier.

The source-map API MUST not split the identifier at arbitrary byte positions.

---

134. Emoji and Non-Identifier Characters

An emoji or other non-identifier Unicode scalar may appear in source only where the lexical grammar permits it.

If it is invalid:

invalid character

the diagnostic still receives an exact UTF-8 byte span.

---

135. Source Span and Lexical Errors

Examples:

invalid UTF-8
invalid character
unterminated string
invalid numeric literal
invalid escape

must all produce deterministic source ranges.

The range should identify the smallest useful source region consistent with the lexical diagnostic.

---

136. Source Span and Parser Errors

Examples:

expected ')'
expected expression
unexpected token
unterminated block

should use:

- the unexpected token span; or
- a zero-width insertion span where a token is missing.

---

137. Source Span and Semantic Errors

Examples:

unknown identifier
type mismatch
invalid effect
resource requirement conflict
capability unavailable
invalid quantum operation

must point to the relevant source construct.

If multiple constructs are involved, use multiple labeled spans.

---

138. Source Span and Type Errors

For:

x + y

where the operands have incompatible types, a diagnostic may use:

operator span
x span
y span

rather than incorrectly highlighting the entire program.

---

139. Source Span and Resource Errors

For:

requires memory >= requirement

the requirement span identifies the user's intent.

If the target cannot satisfy it, target/resource diagnostics may attach additional context.

The compiler MUST distinguish:

program requirement

from:

target resource availability

---

140. Source Span and Capability Errors

For:

requires capability("quantum.mid_circuit_measurement")

the span identifies the capability declaration.

A missing target capability is not a lexical or syntax error.

---

141. Source Span and Hardware Errors

Hardware-specific failures belong downstream.

A physical mapping failure may point to:

source operation

through derived provenance.

The source span MUST remain unchanged.

---

142. Generated Operations

Generated operations MUST be marked as derived/generated provenance.

This applies to:

- QEC operations;
- routing swaps;
- scheduling barriers;
- compiler-generated copies;
- lowering instructions;
- HDL synthesis constructs;
- accelerator lowering;
- distributed communication inserted by the compiler.

---

143. Provenance Compression

For extremely large transformations, provenance MAY be compressed.

However, compression MUST preserve the ability to recover the required source relationships.

A compiler MAY use:

span interning
range sets
provenance IDs
source-map tables
interval structures

as implementation strategies.

These are not part of source-language semantics.

---

144. Provenance Granularity

The compiler SHOULD preserve source provenance at a granularity sufficient for diagnostics.

It does not necessarily need to preserve every whitespace byte in every downstream IR node.

However, the compiler MUST NOT discard provenance before all required diagnostic/tooling consumers have completed.

---

145. Provenance Lifetime

Provenance lifetime is tied to the consumers that require it.

For:

frontend

token spans are required during parsing.

For:

semantic analysis

AST spans are required.

For:

IR

semantic/AST provenance may be compressed or translated.

For:

debugging

additional retention may be required.

The lifecycle MUST be explicit.

---

146. No Semantic Meaning in Span Values

The numeric value:

start = 42

must never be interpreted as:

qubit 42
CPU core 42
device 42
node 42
register 42

Source positions and computational resource IDs are different namespaces.

---

147. Namespace Separation

The following MUST remain separate:

FileId
BytePos
ResourceId
QubitId
PhysicalQubitId
NodeId
DeviceId
RegisterId
SymbolId
TypeId
IRId

A source span may reference semantic entities, but the span itself MUST NOT substitute one identifier type for another.

This preserves the repository's broader typed-ID architecture.

---

148. Canonical Quantum Boundary

The source-span layer MUST NOT introduce:

QuantumSpan
QuantumSourceMap
QuantumGateSpan
PhysicalQubitSpan

unless such types are purely wrappers around the universal provenance model and are demonstrably necessary.

Quantum source provenance should use the universal source-span contract.

The canonical quantum semantic boundary remains:

quantum::ir

---

149. HDL Boundary

Likewise, HDL does not receive an independent coordinate system.

HDL source constructs use the same:

FileId
BytePos
Span
Provenance

model.

---

150. AI/Data/Networking Boundary

AI, data, networking, security, distributed, and other domains all use the same source provenance system.

There must not be one source-span model per domain.

---

151. Grammar Integration

The grammar itself does not encode numeric source coordinates.

ANTLR source positions and Rust source positions MUST be reconciled through explicit adapters where both implementations exist.

"grammar/Zamani.g4" remains the syntax composition root.

The source-span contract is shared by:

ANTLR tooling
Rust lexer
Rust parser
AST
diagnostics

---

152. ANTLR Integration

If ANTLR produces:

line
character position
token interval

the adapter MUST map it to the canonical Zamani:

FileId + byte range

model.

ANTLR-specific coordinates MUST NOT become the canonical Zamani source identity.

---

153. Rust Lexer Integration

"src/lexer.rs" MUST:

1. maintain a current byte offset;
2. maintain correct line state;
3. produce half-open spans;
4. preserve UTF-8 boundaries;
5. avoid unchecked slicing;
6. distinguish EOF;
7. produce deterministic diagnostics;
8. never encode target hardware limits.

The current lexer architecture already attaches "Span" information to tokens and uses "SourceFile"/"BytePos".

---

154. Parser Integration

"src/parser.rs" MUST consume token spans rather than reconstructing source locations from token text.

The parser SHOULD provide helpers for:

span_of(token)
span_between(first, last)
zero_width_at(token.start)

or equivalent.

---

155. AST Integration

"src/ast/mod.rs" currently stores spans directly on many AST nodes.

That pattern is retained.

Every new AST node introduced in the future MUST define its provenance contract before implementation.

A new AST node is incomplete if its source span behavior is unspecified.

---

156. Feature Completion Requirement

Every new grammar feature MUST specify:

source span of the whole construct
source span of each significant child
generated/synthetic span behavior
error span behavior
semantic provenance
IR provenance
macro provenance if applicable
test expectations

This is mandatory for independent file completion.

---

157. Feature Manifest Integration

If the repository adopts feature manifests, each feature SHOULD contain:

source_spans:
  primary:
  children:
  generated:
  diagnostics:
  provenance:

This allows a feature to be completed without later discovering that source mapping was unspecified.

---

158. Specification Integration

The following specifications depend on this contract:

grammar/spec/lexical.md
grammar/spec/syntax.md
grammar/spec/type-system.md
grammar/spec/semantics.md
grammar/spec/resources.md
grammar/spec/quantum.md
grammar/spec/diagnostics.md
grammar/spec/compatibility.md

Where those files refer to source positions, they MUST conform to this document.

---

159. Diagnostics Specification Integration

A future/associated:

grammar/spec/diagnostics.md

MUST use this document for:

primary spans
secondary spans
replacement ranges
line/column conversion
source snippets
generated provenance

It MUST NOT redefine span identity.

---

160. Validation Integration

"grammar/validation/" MUST include source-span validation.

Validation SHOULD check:

span start <= end
span within source
UTF-8 boundary validity
EOF correctness
line mapping correctness
deterministic ordering
cross-file safety
synthetic provenance
AST span coverage
IR provenance coverage

---

161. Tests Integration

"grammar/tests/" MUST include:

source-spans/

or an equivalent existing test organization.

Tests MUST cover:

- empty source;
- one-byte source;
- multi-line source;
- CRLF;
- LF;
- Unicode;
- combining marks;
- multi-byte identifiers;
- tabs;
- EOF;
- zero-width spans;
- invalid UTF-8;
- malformed input;
- parser recovery;
- cross-file diagnostics;
- macro provenance;
- generated provenance;
- incremental edits;
- very large source;
- deterministic output.

---

162. Negative Tests

Negative source-span tests MUST include:

start > end
end > source length
invalid UTF-8 boundary
cross-file merge
stale revision
invalid line/column conversion
overflowing position arithmetic

The expected result MUST be structured failure rather than silent corruption.

---

163. Boundary Tests

Boundary tests MUST include:

offset 0
offset source_length
last byte
first byte after newline
before/after CRLF
Unicode scalar boundary
EOF
empty line
empty file

---

164. Scalability Tests

Scalability tests MUST demonstrate that the implementation handles progressively larger inputs without an artificial language-level ceiling.

Tests SHOULD include:

tiny source
small source
large source
very large source
many lines
long lines
many Unicode characters
many AST nodes
many diagnostics
many files

The exact test sizes are implementation/resource-policy concerns and MUST NOT become language limits.

---

165. Determinism Tests

Given identical inputs and configuration:

run 1
run 2
run 3

must produce identical:

FileId assignments
token spans
AST spans
diagnostic spans
line/column conversions
serialized source maps

---

166. Parallel Determinism Tests

Run compilation with different safe parallel configurations where supported.

The resulting source spans MUST remain identical.

---

167. Unicode Tests

Tests MUST include at least:

ASCII
Latin-1 supplement
CJK
Cyrillic
Greek
combining marks
multi-byte identifiers
emoji in legal contexts
invalid UTF-8

The exact legal identifier set remains governed by "grammar/spec/lexical.md".

---

168. Newline Tests

Tests MUST cover:

LF
CRLF
CR
mixed newline styles
newline at EOF
no newline at EOF
multiple consecutive newlines
empty lines

The accepted behavior for CR MUST match the canonical lexical policy.

---

169. Tab Tests

Tests MUST demonstrate that:

tab source position

is independent of:

display width

---

170. Source Extraction Tests

For every valid span:

extract(span)

MUST return exactly the source substring represented by:

[start, end)

where the span corresponds to valid UTF-8 text.

---

171. Round-Trip Position Tests

Where line/column conversion is supported:

byte position
    ↓
line/column
    ↓
byte position

MUST round-trip at valid scalar boundaries.

If the supplied line/column identifies a display-only position that cannot map uniquely to a byte offset, the API MUST return an explicit error/none rather than guessing.

---

172. Span Serialization Tests

Serialized/deserialized spans MUST preserve canonical identity.

Round trip:

Span
 ↓
serialize
 ↓
deserialize
 ↓
Span

must produce equivalent spans.

---

173. Source Revision Tests

Tests MUST demonstrate that a span from revision "R1" cannot silently be interpreted against unrelated revision "R2".

---

174. Macro Tests

Macro tests MUST verify:

invocation span
definition span
generated span
nested expansion provenance

where supported.

---

175. IR Provenance Tests

Tests MUST verify that:

source
 ↓
AST
 ↓
semantic model
 ↓
IR

does not lose the required source provenance.

---

176. Quantum Provenance Tests

Tests MUST verify:

quantum source
 ↓
AST
 ↓
quantum semantic model
 ↓
quantum::ir

retains source provenance.

After transformations:

optimization
routing
QEC
scheduling

derived operations must remain distinguishable from source-written operations.

---

177. HDL Provenance Tests

Tests MUST verify:

Zamani HDL source
 ↓
AST
 ↓
hardware semantic model
 ↓
HDL/hardware IR

retains provenance.

---

178. Error Message Tests

Diagnostics should be tested not only for text but for:

file
start byte
end byte
line
column
primary/secondary status
provenance

This prevents accidental source-location regressions.

---

179. Compatibility

Changes to source-span semantics are compatibility-sensitive.

A change to:

line numbering
column numbering
newline policy
UTF-8 boundary behavior
EOF position
span interval semantics

MUST be versioned if it changes observable language/tooling behavior.

---

180. Backward Compatibility

Existing source programs MUST NOT receive different source locations merely because the compiler was upgraded unless:

1. the language version changes;
2. the source-span specification intentionally changes;
3. the compiler fixes a documented correctness bug.

Correctness fixes SHOULD be documented in compatibility/release notes.

---

181. Source Span Versioning

The source-span contract SHOULD have an explicit version independent of compiler implementation version.

Conceptually:

language version
source-span policy version
compiler version

remain separately identifiable.

---

182. Generated Documentation

"grammar/grammar.md" may document source-span behavior as implementation conformance.

It MUST NOT redefine this specification.

"grammar/Zamani-Grammar.md" may discuss future provenance ideas but MUST NOT silently change the canonical source-span model.

---

183. No Duplicate Source-Span Authority

The repository MUST NOT contain:

one source-span specification for ANTLR
another for Rust
another for AST
another for quantum
another for HDL

There is one universal source-span contract.

Adapters may exist, but they conform to this document.

---

184. Error Handling Policy

The source-map implementation MUST use structured errors where operations can fail because of caller input.

Possible categories include:

invalid_file_id
invalid_position
invalid_range
cross_file_operation
invalid_utf8_boundary
stale_revision
line_out_of_range
column_out_of_range
resource_exhausted

Exact Rust error types belong to implementation.

---

185. No Silent Saturation

Source positions MUST NOT silently saturate:

overflow -> maximum value

or:

underflow -> zero

unless an API explicitly defines saturating arithmetic for a non-semantic convenience operation.

Canonical source positions require exactness.

---

186. No Silent Wrapping

Likewise:

maximum + 1 -> 0

is prohibited for canonical source positions.

Checked arithmetic MUST be used where overflow is possible.

---

187. Public API Safety

Public source-span APIs MUST be safe to call with malformed external values.

They should return:

Result
Option

where appropriate.

They MUST NOT require callers to use Rust "unsafe".

---

188. Internal Assertions

Internal assertions MAY validate impossible compiler invariants.

However, ordinary source errors and user/tool input MUST be represented as structured errors rather than panics.

---

189. Source Map and Compiler Cache

Compilation caches MAY cache:

line indexes
source hashes
serialized spans
provenance

Cache keys MUST include enough source/revision identity to prevent stale span reuse.

---

190. Reproducibility

For reproducible builds, source-span output MUST NOT depend on:

absolute machine path
filesystem enumeration
wall-clock time
random numbers
thread order
hardware
locale
timezone

Logical source identities SHOULD be used.

---

191. Content Hashing

A source revision MAY have a content hash.

A hash is metadata for identity/cache validation.

It is not a substitute for:

FileId
BytePos
Span

---

192. Source Span and Security

Source spans are security-relevant because incorrect spans can cause:

- misleading diagnostics;
- incorrect automated edits;
- source corruption;
- refactoring the wrong code;
- IDE manipulation of unintended text.

Therefore source-span APIs MUST favor correctness over convenience.

---

193. Automated Fix Safety

Before applying an automated fix:

replacement span

MUST be validated against the exact source revision.

The tool MUST NOT blindly apply a stale range.

---

194. Source Span and Secrets

Source spans themselves MUST NOT cause source text to be unnecessarily copied.

Diagnostics SHOULD avoid copying sensitive source content unless required.

Source-span metadata can normally identify a region without retaining its contents.

---

195. Source Span and Logging

Compiler logs SHOULD log:

FileId
byte range
diagnostic identifier

rather than copying entire source files.

Debug logging MUST NOT accidentally dump entire source files merely because a span is inspected.

---

196. Source Span and Telemetry

If compiler telemetry exists, source spans SHOULD be represented without transmitting source contents unless explicitly permitted.

The span model itself does not require source disclosure.

---

197. Source Span and Networking

Source spans MUST NOT require network access.

Remote source retrieval, package resolution, or distributed compilation may assign source identities before compilation.

Once registered, span calculation is local and deterministic.

---

198. Distributed Compilation

Distributed compilation MUST preserve logical source identity.

Worker:

worker A

and:

worker B

MUST NOT produce conflicting source locations for the same logical source revision merely because files were processed on different machines.

---

199. Remote Diagnostics

A remote compiler MAY return:

file identity
byte range
line
column
diagnostic

to a client.

The client MUST be able to associate the logical source identity with its local buffer.

---

200. Container/Virtual Files

Source files may originate from:

virtual editor buffers
archives
package stores
memory
remote sources
generated files

The source-span contract remains unchanged.

---

201. Source Maps for Generated Artifacts

Generated artifacts such as:

LLVM
QIR
HDL
assembly
machine code

MAY carry source maps.

The mapping direction is:

generated location
    ↓
Zamani provenance
    ↓
original source span(s)

It MUST NOT be treated as a replacement for the canonical Zamani source-span model.

---

202. Many-to-One Mapping

Multiple source spans MAY map to one generated location.

Example:

a + b

may become one optimized instruction.

The generated instruction may retain both source spans.

---

203. One-to-Many Mapping

One source span MAY map to many generated locations.

Example:

quantum operation

may become:

decomposition operation 1
decomposition operation 2
routing operation
correction operation

All may retain derived provenance.

---

204. Provenance DAG

For complex compiler pipelines, provenance is conceptually a graph rather than a single range:

source A ─┐
          ├── semantic node ──┐
source B ─┘                   │
                              ├── IR node
source C ─────────────────────┘

The implementation MAY represent this through compact provenance IDs.

The graph MUST remain deterministic.

---

205. Provenance Cycles

Provenance MUST NOT create accidental cycles.

Generated artifacts may refer back to source, but source spans themselves must remain the root provenance layer.

---

206. Provenance and Optimization Pipelines

Each transformation stage:

semantic
optimization
routing
scheduling
resilience
QEC
ZQN
HAL lowering

MUST either:

1. preserve provenance;
2. explicitly transform provenance;
3. explicitly discard provenance when no consumer requires it.

Silent loss is prohibited for production diagnostic paths.

---

207. Provenance and Verification

IR verification errors SHOULD point back to source provenance whenever possible.

For example:

invalid IR invariant

should identify the source construct that caused the invalid IR when the compiler can establish that relationship.

---

208. Provenance and Runtime Errors

Runtime systems MAY receive source maps for debugging.

Runtime source locations MUST be derived from compiler-produced metadata.

Runtime MUST NOT reinterpret raw source spans using a different coordinate convention.

---

209. Provenance and Profiling

Profilers MAY aggregate execution metrics by source span.

Aggregation MUST not modify source identity.

A hot source operation remains the source operation regardless of how many target operations implement it.

---

210. Provenance and Scheduling Visualization

Scheduling tools MAY display:

source operation
physical realization
scheduled operation

as separate layers.

The source span identifies the source operation.

---

211. Provenance and QEC Visualization

QEC tools MAY display:

logical source operation
generated correction operations

with derived provenance.

The user must be able to distinguish source-written operations from generated operations.

---

212. Provenance and Hardware Visualization

Hardware tools MAY map source operations to:

device
core
accelerator
physical qubit
memory
network node

but these remain separate metadata dimensions.

The source span remains unchanged.

---

213. Completion Contract for "grammar/spec/source-spans.md"

This file is complete when:

- normative position model is defined;
- half-open range semantics are defined;
- EOF semantics are defined;
- zero-width semantics are defined;
- UTF-8 semantics are defined;
- Unicode coordinate policy is defined;
- line/column semantics are defined;
- newline policy is defined;
- cross-file semantics are defined;
- dummy/unknown semantics are defined;
- revision semantics are defined;
- diagnostics integration is defined;
- lexer integration is defined;
- parser integration is defined;
- AST integration is defined;
- semantic integration is defined;
- IR integration is defined;
- quantum integration is defined;
- HDL integration is defined;
- generated-code provenance is defined;
- macro provenance is defined;
- incremental compilation is defined;
- deterministic behavior is defined;
- scalability requirements are defined;
- safe-Rust requirement is defined;
- compatibility behavior is defined;
- tests are defined.

No subsequent grammar feature should need to reopen this document merely to invent its own span semantics.

---

214. Required Repository Integration

The following existing files must conform to this contract.

"src/source_map.rs"

Owns executable:

- "FileId";
- "BytePos";
- "Span";
- "SourceFile";
- "SourceMap";
- line indexing;
- source extraction;
- location conversion;
- span validation;
- source revision/provenance support where implemented.

It MUST NOT own grammar semantics.

The current implementation's "Span::merge()" approximation and unchecked "len()" arithmetic should be corrected during implementation of this contract.

---

"src/lexer.rs"

Owns:

- tokenization;
- lexical state;
- token spans.

It MUST consume the source-map contract.

The current lexer already carries "Span" information and therefore should be evolved rather than replaced with another location system.

---

"src/parser.rs"

Owns:

- syntactic parsing;
- construction of AST spans;
- recovery positions.

It MUST NOT independently calculate an incompatible coordinate system.

---

"src/ast/"

Owns:

- AST node provenance.

The current AST's direct use of "Span" is compatible with this design.

New AST nodes MUST carry appropriate provenance.

---

"grammar/spec/lexical.md"

Owns lexical rules.

It MUST refer to this document for source-span semantics rather than redefining them.

Its existing UTF-8 and source-span architecture remains compatible with this contract.

---

"grammar/spec/syntax.md"

Owns syntax.

It MUST specify construct boundaries but MUST use this document for the meaning of source spans.

Its existing source → tokens + spans → AST pipeline remains the correct integration model.

---

"grammar/DESIGN.md"

Owns architecture and authority.

It remains the top-level architecture document and points to this file as the normative source-location contract.

---

215. Required Future/Associated Files

If created, the following files integrate as follows:

grammar/spec/diagnostics.md
    -> diagnostic rendering and labels

grammar/spec/compatibility.md
    -> source-location compatibility/versioning

grammar/validation/source-spans.md
    -> automated source-span validation

grammar/tests/source-spans/
    -> executable conformance tests

grammar/reference/source-spans.md
    -> generated/public reference

None may redefine this contract.

---

216. Implementation Sequence

To make this file independently completable, implementation should proceed in this order:

1. grammar/spec/source-spans.md
       ↓
2. src/source_map.rs
       ↓
3. lexer span conformance
       ↓
4. parser span conformance
       ↓
5. AST span conformance
       ↓
6. diagnostics
       ↓
7. semantic provenance
       ↓
8. canonical IR provenance
       ↓
9. quantum::ir provenance
       ↓
10. HDL/hardware provenance
       ↓
11. optimization provenance
       ↓
12. routing/scheduling/QEC/ZQN provenance
       ↓
13. generated-artifact source maps
       ↓
14. IDE/LSP adapters
       ↓
15. conformance tests

The specification is completed first so implementation files do not need to be repeatedly redesigned when later domains are integrated.

---

217. Hard-Coding Audit

This specification explicitly prohibits source-span semantics based on:

MAX_SOURCE_SIZE
MAX_LINE_COUNT
MAX_LINE_LENGTH
MAX_COLUMN
MAX_SPAN_COUNT
MAX_FILE_COUNT
MAX_DIAGNOSTIC_COUNT
MAX_AST_NODES
MAX_IR_NODES

as language-level limits.

Implementation resource budgets may exist but must remain external to language validity.

No physical hardware limit belongs in the source-span system.

---

218. Production Invariants

A production implementation MUST guarantee:

I1: every concrete span has a valid source identity

I2: every concrete span is half-open

I3: start <= end

I4: concrete end <= source length

I5: source boundaries are UTF-8-safe

I6: EOF is a valid zero-width position

I7: line/column are derived coordinates

I8: byte position is canonical

I9: cross-file ranges are never silently merged

I10: stale revisions are not silently reused

I11: diagnostics are deterministic

I12: source identity is target-independent

I13: provenance is preserved through required compiler stages

I14: generated constructs are distinguishable from source constructs

I15: no unsafe Rust is required

I16: no hardware-derived language limits exist

I17: malformed source cannot cause silent span corruption

I18: public invalid-input operations use structured failure

I19: Unicode behavior is deterministic

I20: newline behavior is deterministic

I21: source extraction matches the canonical range exactly

I22: incremental compilation cannot silently reinterpret stale ranges

I23: parallel compilation cannot change source locations

I24: serialization round-trips canonical span identity

---

219. Production Readiness Checklist

Specification

- [x] Source identity defined
- [x] Byte positions defined
- [x] Half-open ranges defined
- [x] Zero-width ranges defined
- [x] EOF defined
- [x] Unicode defined
- [x] UTF-8 boundaries defined
- [x] Line/column semantics defined
- [x] Newline policy defined
- [x] Diagnostics defined
- [x] AST integration defined
- [x] IR integration defined
- [x] Quantum integration defined
- [x] HDL integration defined
- [x] Generated provenance defined
- [x] Macro provenance defined
- [x] Incremental compilation defined
- [x] Determinism defined
- [x] Scalability defined
- [x] Compatibility defined

Implementation

- [ ] Correct canonical position representation
- [ ] Checked span arithmetic
- [ ] Safe UTF-8 slicing
- [ ] Exact line index
- [ ] Correct CRLF behavior
- [ ] Correct EOF behavior
- [ ] Cross-file-safe joining
- [ ] Explicit dummy/unknown provenance
- [ ] Revision validation
- [ ] Structured span errors
- [ ] Deterministic FileId assignment
- [ ] Lexer conformance
- [ ] Parser conformance
- [ ] AST conformance
- [ ] Diagnostic conformance
- [ ] IR provenance conformance

Testing

- [ ] Empty source
- [ ] EOF
- [ ] Unicode
- [ ] Combining characters
- [ ] Multi-byte identifiers
- [ ] CRLF
- [ ] LF
- [ ] CR policy
- [ ] Tabs
- [ ] Invalid UTF-8
- [ ] Malformed source
- [ ] Recovery
- [ ] Cross-file diagnostics
- [ ] Macro provenance
- [ ] Generated provenance
- [ ] Incremental edits
- [ ] Large sources
- [ ] Determinism
- [ ] Parallel determinism
- [ ] Serialization round-trip
- [ ] Quantum provenance
- [ ] HDL provenance

---

220. Final Architectural Rule

The complete Zamani source-location architecture is:

                    SOURCE BYTES
                         │
                         ▼
                  SOURCE REVISION
                         │
                         ▼
                      FileId
                         │
                         ▼
                     BytePos
                         │
                         ▼
                 ┌───────────────┐
                 │    Span       │
                 │ [start,end)   │
                 └───────────────┘
                         │
          ┌──────────────┼──────────────┐
          ▼              ▼              ▼
        Token           AST        Diagnostics
          │              │              │
          └──────────────┼──────────────┘
                         ▼
                 Semantic Provenance
                         │
                         ▼
                  Canonical IR
                         │
          ┌──────────────┼───────────────┐
          ▼              ▼               ▼
     Classical       quantum::ir       HDL
          │              │               │
          └──────────────┼───────────────┘
                         ▼
                  Transformations
                         │
          ┌──────────────┼──────────────┐
          ▼              ▼              ▼
       Routing       Scheduling        QEC
          │              │              │
          └──────────────┼──────────────┘
                         ▼
                        ZQN
                         │
                         ▼
                        HAL
                         │
                         ▼
                  Target Realization

The invariant is:

«Source location identifies where the user's program came from; it never identifies where the program will run.»

Therefore:

source span
    ≠
hardware resource

and:

source provenance
    ≠
physical realization

The same source-span contract applies from:

one line

to:

arbitrarily large source programs

and from:

one classical operation

to:

massively distributed heterogeneous quantum/classical/HDL systems

without introducing artificial machine-derived limits.

Completion criterion: Once "grammar/spec/source-spans.md" is adopted as the normative contract, "src/source_map.rs" is brought into conformance, and lexer/parser/AST/diagnostic/IR tests pass, future grammar domains can consume source spans without inventing another location model or requiring this specification to be redesigned.