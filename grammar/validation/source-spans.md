Zamani Source-Span Validation Specification

Path: "grammar/validation/source-spans.md"
Language: Zamani
Status: Normative validation contract
Specification authority: "grammar/spec/source-spans.md"
Architecture authority: "grammar/DESIGN.md"
Implementation baseline: Rust 1.97 / Rust 1.97.1
Rust edition: 2021
Safety: Safe Rust only; no Rust "unsafe"
Portability objective: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)
Scalability objective: Scale from the smallest supported source input to arbitrarily large source inputs, bounded only by representational capabilities and explicitly declared implementation resource budgets.

---

1. Purpose

This document defines the production validation requirements for source spans throughout the Zamani repository.

It establishes how source positions, spans, source identities, line indexes, diagnostics, AST provenance, generated constructs, and downstream compiler provenance must be validated.

It is a validation contract, not a replacement for the normative source-span specification.

Its purpose is to guarantee that every compiler-visible source location:

1. Refers to the correct source file.
2. Uses UTF-8 byte offsets consistently.
3. Represents a valid half-open interval.
4. Respects UTF-8 character boundaries.
5. Has deterministic line and column information.
6. Remains correct across Unicode and line-ending variations.
7. Preserves source identity across lexer, parser, AST, semantic analysis, and IR.
8. Supports generated and transformed source without fabricating original locations.
9. Does not impose artificial language-level size limits.
10. Produces precise and reproducible diagnostics.
11. Scales with available memory and processing resources.
12. Is implemented entirely using safe Rust.

A source-span feature is not production-ready merely because "Span" exists or because a lexer attaches spans to tokens.

Production readiness requires validation of the complete source-provenance pipeline.

---

2. Authority and Integration

2.1 Authority hierarchy

The following hierarchy is mandatory:

grammar/DESIGN.md
        |
        v
grammar/spec/source-spans.md
        |
        v
grammar/validation/source-spans.md
        |
        +-------------------------+
        |                         |
        v                         v
src/source_map.rs            src/lexer.rs
        |                         |
        +------------+------------+
                     |
                     v
                src/parser.rs
                     |
                     v
                 src/ast/
                     |
                     v
              Semantic Analysis
                     |
                     v
             Canonical Semantic Model
                     |
                     v
                Canonical IR
                     |
          +----------+----------+
          |          |          |
          v          v          v
      Classical   quantum::ir   HDL
          |          |          |
          +----------+----------+
                     |
                     v
            Optimization/Lowering
                     |
                     v
             Routing/Scheduling
                     |
                     v
                 QEC/ZQN
                     |
                     v
                 HAL/Backend

"grammar/spec/source-spans.md" owns the normative source-span model.

This file owns the validation rules, conformance conditions, negative cases, boundary cases, integration requirements, and acceptance criteria.

No downstream component may silently redefine source-span semantics.

2.2 Existing files that must remain

Do not rename:

- "grammar/DESIGN.md"
- "grammar/README.md"
- "grammar/spec/source-spans.md"
- "grammar/validation/source-spans.md"
- "src/source_map.rs"
- "src/lexer.rs"
- "src/parser.rs"
- "src/ast/mod.rs"
- "grammar/grammar.md"

Do not create another competing source-map implementation.

Do not create another independent span authority.

2.3 Ownership table

File/component| Owns| Does not own
"grammar/DESIGN.md"| Overall architecture and invariants| Detailed span implementation
"grammar/spec/source-spans.md"| Normative source-span semantics| Executable implementation
"grammar/validation/source-spans.md"| Validation and conformance| Redefining normative semantics
"src/source_map.rs"| Source files, identities, offsets, spans, line indexes| Lexical or semantic meaning
"src/lexer.rs"| Tokenization and token spans| Source-file identity policy
"src/parser.rs"| Syntax recognition and construct spans| Hardware realization
"src/ast/mod.rs"| AST provenance representation| Physical resource mapping
Semantic analysis| Semantic provenance| Source decoding
Canonical IR| Transformation provenance| Original-source reconstruction
Diagnostics| Location rendering| Changing canonical positions
IDE/LSP| Protocol coordinate conversion| Redefining compiler byte offsets
Macro subsystem| Expansion provenance| Fabricating original locations
Compiler backends| Preserving available provenance| Reinterpreting source offsets

---

3. Normative Terminology

The following terms are used consistently.

MUST: Mandatory requirement.

MUST NOT: Prohibited behavior.

SHOULD: Recommended unless a documented technical reason justifies otherwise.

SHOULD NOT: Discouraged unless a documented technical reason justifies otherwise.

MAY: Optional behavior.

Concrete span: A span referring to an actual source-file revision and byte interval.

Synthetic span: A span associated with generated syntax that does not have a direct original-source interval.

Unknown span: A location that cannot be established.

Source revision: A particular immutable snapshot of a source file.

Canonical position: Source identity plus UTF-8 byte offset.

Presentation position: A derived line/column representation.

Resource budget: An implementation-defined operational limit, not a language-level validity restriction.

---

4. Canonical Source Position

4.1 Required representation

The canonical position MUST conceptually contain:

SourcePosition {
    file_id,
    revision_id,
    byte_offset
}

Revision identity MAY be maintained by the source-map context rather than stored directly in every position, provided stale positions cannot be silently reused.

The canonical position MUST NOT be based on:

- CPU word size;
- Unicode character count;
- grapheme count;
- terminal display width;
- UTF-16 code units;
- line number;
- column number;
- physical file descriptor;
- memory address;
- hardware identifier.

4.2 Byte-offset invariant

For a concrete source file:

0 <= start <= end <= source_byte_length

Every concrete span MUST satisfy this invariant.

The source length and offsets MUST use a representation capable of expressing every source position supported by the implementation's declared compilation envelope.

The existing "BytePos(u32)" representation in "src/source_map.rs" MUST be audited and upgraded if it prevents the implementation from representing source positions required by its supported resource envelope.

A conversion from a larger offset into a narrower representation MUST never silently truncate.

4.3 No language-level source ceiling

The following MUST NOT be universal language limits:

MAX_SOURCE_BYTES
MAX_FILE_SIZE
MAX_LINE_COUNT
MAX_LINE_LENGTH
MAX_COLUMN
MAX_TOKEN_LENGTH
MAX_IDENTIFIER_LENGTH
MAX_SPAN_LENGTH

A compiler MAY enforce an operational budget.

Such a failure MUST be reported as a resource or compilation-budget failure, not as a fabricated language syntax error.

---

5. Half-Open Interval Validation

All concrete spans MUST use:

[start, end)

The beginning is included and the end is excluded.

Required properties:

span.start <= span.end
span.length = span.end - span.start

Adjacent spans MUST compose without overlapping:

[a, b)
[b, c)

The following operations MUST follow the same convention:

- containment;
- intersection;
- joining;
- slicing;
- token coverage;
- diagnostic highlighting;
- AST source coverage;
- source-map translation;
- generated-source mapping.

A zero-length span is valid.

A zero-length span MUST NOT be rejected merely because its length is zero.

---

6. Source Identity Validation

6.1 File identity

Every concrete span MUST refer to an existing source-file identity in the relevant source-map context.

An unknown or reserved identity MUST NOT be interpreted as a real source file.

If backward compatibility retains:

Span::dummy()

the implementation MUST guarantee that the dummy identity cannot collide with a real registered file.

6.2 Stable identity

Registering another source file MUST NOT silently change existing file identities.

Source identity MUST remain independent of:

- operating system;
- target architecture;
- physical memory location;
- execution device;
- backend;
- distributed node.

6.3 Revision validation

A span associated with revision A MUST NOT silently refer to different text in revision B.

The implementation MUST:

1. Preserve the original revision;
2. Explicitly translate through a verified edit map; or
3. Reject stale span use.

The following is prohibited:

old byte offset
    |
    v
new source revision
    |
    v
silently assumed equivalent location

6.4 File identity failure cases

The validator MUST reject:

- unknown file IDs;
- invalid revision IDs;
- spans referring to removed revisions without an explicit mapping;
- accidental dummy-to-real conversion;
- cross-file span operations that silently discard identity.

A join operation across different source files MUST return a structured failure or an explicitly defined multi-source provenance result.

It MUST NOT produce an arbitrary single-file span.

---

7. UTF-8 Validation

7.1 Canonical encoding

Source text is UTF-8.

Canonical byte positions refer to the original UTF-8 source bytes.

The source map and lexer MUST agree on that representation.

7.2 Boundary requirements

For a concrete text span, both endpoints MUST be valid UTF-8 boundaries.

The following is prohibited:

valid UTF-8 character
    |
    +-- span starts inside encoded character

The implementation MUST validate source slicing boundaries before producing string slices.

7.3 Safe Rust

Source-span processing MUST use safe Rust.

Permitted approaches include:

- "str::get";
- "str::get_mut" where appropriate;
- checked range conversion;
- "char_indices";
- "is_char_boundary";
- checked integer conversion;
- explicit error propagation.

The implementation MUST NOT rely on unchecked UTF-8 slicing.

The production implementation MUST NOT introduce:

unsafe
unsafe fn
unsafe impl
unsafe trait
unsafe { ... }

7.4 Invalid UTF-8 input

When source bytes are supplied directly, invalid UTF-8 MUST produce a structured source-encoding diagnostic.

The compiler MUST NOT silently replace invalid bytes with replacement characters and then pretend that the resulting offsets refer to the original source.

If decoding requires a recovery representation, the implementation MUST preserve a mapping to the original byte positions.

---

8. Line and Column Validation

8.1 Canonical versus derived coordinates

The canonical position is:

FileId + RevisionId + ByteOffset

Line and column are derived.

Cached coordinates MAY be retained for compatibility or performance, but MUST always be verifiable against the canonical byte position.

8.2 Line numbering

Human-facing line numbers MUST be one-based.

first line = 1

8.3 Column numbering

The compiler MUST distinguish:

- byte column;
- Unicode scalar column;
- grapheme column;
- display column;
- UTF-16 column.

The canonical diagnostic coordinate SHOULD use one-based Unicode scalar columns.

Protocol-specific coordinate conversion MUST happen at the tooling boundary.

8.4 Coordinate consistency

For every concrete span:

cached_start_line
cached_start_column

MUST correspond to:

source_map.lookup(span.file_id, span.start)

Cached values MUST NOT be trusted when inconsistent with the source map.

The same requirement applies to end coordinates if they are cached.

8.5 Unicode

The following must produce correct byte offsets and presentation coordinates:

a
π
λ
漢
🙂
é

The final example may consist of more than one Unicode scalar value.

The implementation MUST distinguish Unicode scalar positions from grapheme positions.

8.6 Tabs

Tabs MUST count as one source scalar and their actual UTF-8 byte length MUST determine byte offsets.

Terminal display expansion MUST NOT change canonical positions.

8.7 Line-ending policy

The source map and lexer MUST share one documented line-ending policy.

At minimum, test:

LF
CRLF
CR

If standalone CR or Unicode separators are not accepted as line terminators, their behavior MUST be explicitly defined and consistent across the lexer, source map, diagnostics, formatter, and tooling.

Test Unicode line separators:

U+2028
U+2029

No component may independently invent line boundaries.

8.8 CRLF

For:

a\r\nb

the next logical line MUST start after both CR and LF.

The CRLF sequence MUST NOT create two logical line breaks.

The span covering the line terminator MAY cover both bytes.

8.9 EOF

For a source containing N UTF-8 bytes:

EOF = [N, N)

EOF MUST:

- use the actual source-file identity;
- refer to the correct source revision;
- be a valid UTF-8 boundary;
- resolve to a deterministic line and column;
- not use the dummy span.

An empty source MUST have a valid EOF span at byte offset zero.

---

9. Source Map Requirements

"src/source_map.rs" owns executable source mapping.

It MUST provide a coherent implementation for:

1. Source registration.
2. File identity.
3. Revision identity.
4. Source content ownership.
5. Byte-position validation.
6. Span validation.
7. Span construction.
8. EOF construction.
9. Unknown/dummy location handling.
10. Line indexing.
11. Byte-to-line lookup.
12. Byte-to-column lookup.
13. Source slicing.
14. Span joining.
15. Span containment.
16. Span intersection.
17. Source revision validation.
18. Structured failures.

The API MUST make invalid concrete spans difficult to construct.

Where public fields must remain for compatibility, validation MUST occur at all trust boundaries.

The source map MUST remain the authority for resolving concrete source positions.

---

10. Existing Source-Map Remediation

The existing "src/source_map.rs" uses:

pub struct FileId(pub usize);
pub struct BytePos(pub u32);

and stores line/column information in "Span".

These choices require the following production checks.

10.1 BytePos width

Audit all conversions involving:

u32
usize
String::len()

No source length or byte offset may be silently truncated.

Replace the narrow representation where necessary, maintaining API compatibility through explicit conversion methods where practical.

10.2 Checked arithmetic

All offset arithmetic MUST be checked.

The implementation MUST detect:

- overflow;
- underflow;
- invalid conversions;
- invalid range endpoints;
- offsets beyond source length.

It MUST NOT wrap an invalid offset into a different valid-looking position.

10.3 Line index

Line-start indexing MUST be deterministic and efficient.

Recommended complexity:

source registration: O(n)
line lookup:         O(log L)

where:

- n is source byte length;
- L is number of indexed logical lines.

An equivalent implementation is acceptable if it provides comparable or better complexity.

10.4 Source storage

Source text SHOULD be shared rather than copied into every span.

The existing "Arc"-based storage approach may be retained.

The source map MUST NOT allocate one independent source string per token.

10.5 API invariants

Every public operation MUST document:

- accepted inputs;
- returned values;
- failure behavior;
- revision requirements;
- UTF-8 boundary requirements;
- complexity;
- ownership;
- compatibility behavior.

---

11. Lexer Integration

"src/lexer.rs" owns tokenization and token spans.

Every emitted token MUST have a span satisfying the source-span contract.

For ordinary source tokens:

token.span.start
    =
first byte belonging to token

token.span.end
    =
first byte after token

11.1 Existing lexer concerns

The existing lexer uses byte positions but also searches through character indices.

Its "read_char()" and "peek_char()" implementation MUST be audited.

In particular:

- repeated "char_indices()" searches MUST NOT make scanning quadratic;
- character indices MUST NOT be confused with byte offsets;
- EOF detection MUST not truncate large source lengths;
- UTF-8 traversal MUST remain valid;
- every token endpoint MUST be a valid UTF-8 boundary.

11.2 Required scanner properties

The lexer SHOULD maintain a monotonic byte cursor.

Character traversal SHOULD use an efficient forward iterator or equivalent cursor abstraction.

For a token stream:

token[0]
token[1]
...
token[n]

the lexer MUST guarantee deterministic positions independent of target hardware.

11.3 Whitespace and comments

Whitespace and comment consumption MUST preserve source positions.

Unterminated comments and strings MUST produce accurate error spans.

An unterminated construct MUST NOT receive an unrelated previous-token span.

11.4 Quantum literals

Quantum literals, including:

|0⟩
|1⟩
|+⟩
|-⟩
|ψ⟩

MUST receive spans measured in UTF-8 bytes.

The span MUST include the exact original source spelling.

No quantum-specific offset unit is permitted.

11.5 MTS and future Unicode syntax

MTS literals and other Unicode-oriented syntax MUST use the same position model.

Adding a new language domain MUST NOT introduce a separate coordinate convention.

---

12. Parser Integration

"src/parser.rs" MUST preserve source provenance while constructing syntax.

For every parsed construct, the parser MUST define:

1. Which token determines its beginning.
2. Which token determines its end.
3. Whether delimiters are included.
4. How empty constructs are represented.
5. How missing tokens are represented.
6. How recovery affects provenance.
7. How nested constructs preserve their individual spans.

12.1 Construct spans

For a complete construct:

start = first source byte belonging to the construct
end   = first byte after the construct

The parser MUST NOT use a child span as the entire parent span when the parent contains additional source syntax.

12.2 Parent and child spans

Parent spans MUST contain the source extents of their concrete children where the grammar defines containment.

Child spans MUST NOT be expanded merely to match their parents.

12.3 Missing syntax

A missing token MUST use a deterministic zero-width insertion span at the recovery position.

The parser MUST NOT fabricate source bytes for missing syntax.

12.4 Recovery

Parser recovery MUST preserve the original spans of valid tokens.

Recovery-generated nodes MUST be explicitly identifiable.

They MUST NOT silently appear to have been written by the user.

12.5 Progress

Span validation MUST accompany parser recovery progress checks.

A parser MUST NOT repeatedly produce errors at the same invalid location without consuming input or terminating the recovery attempt.

---

13. AST Integration

"src/ast/" owns the representation of source syntax.

Every source-originating AST node MUST have a documented provenance policy.

For each AST node, specify:

- source file;
- source revision;
- start offset;
- end offset;
- parent-child relationship;
- synthetic status;
- recovery status;
- provenance behavior during transformations.

13.1 Existing AST structures

Do not introduce a parallel AST solely to support source spans.

Reuse existing AST structures and add only the provenance capabilities necessary to satisfy this specification.

13.2 Identifier spans

Identifiers MUST retain their exact source spans.

The AST MUST NOT reconstruct identifier locations from their text.

13.3 Literal spans

Literal values MUST retain the source extent of their original spelling.

This is important when the parsed semantic value differs from the spelling.

For example:

1_000
1000

may represent the same numeric value but have different source spans and source spellings.

13.4 Composite expressions

Binary expressions, calls, indexing, member access, ranges, and assignments MUST preserve their own full construct spans.

Their child spans MUST remain independently accurate.

13.5 AST transformations

An AST transformation MUST preserve provenance where possible.

If a transformation creates new syntax without a direct source extent, it MUST mark that syntax synthetic or unknown.

It MUST NOT assign an unrelated source span merely to satisfy a non-optional field.

---

14. Semantic Analysis Integration

Semantic analysis MUST preserve the distinction between:

source origin
semantic identity
semantic transformation

A semantic entity may originate from:

- one source construct;
- multiple source constructs;
- generated syntax;
- imported declarations;
- macro expansion;
- compiler synthesis.

A semantic entity MUST NOT be forced into one misleading contiguous source span when its origin is genuinely non-contiguous.

Where necessary, use a provenance collection.

Semantic errors MUST identify the relevant source construct and may include related locations.

Examples include:

- duplicate declarations;
- unresolved names;
- invalid type arguments;
- ownership violations;
- effect violations;
- capability violations;
- invalid quantum operations;
- incompatible HDL declarations;
- invalid resource requirements.

Semantic validation MUST NOT reinterpret source offsets.

---

15. Canonical IR Integration

All source-originating IR instructions MUST preserve available provenance.

The canonical IR remains the authority for cross-domain semantic representation.

Quantum provenance MUST flow into "quantum::ir".

The frontend MUST NOT introduce another quantum IR merely to store source spans.

15.1 IR provenance

IR provenance MAY include:

Origin {
    source_origin,
    transformation_origin,
    generated_origin
}

The exact representation belongs to the IR implementation.

It MUST support:

- original source locations;
- multiple source origins;
- generated instructions;
- optimization transformations;
- lowering transformations;
- source-aware diagnostics.

15.2 One-to-many transformations

One source construct may produce multiple IR instructions.

All resulting instructions MUST preserve the source origin or an explicit transformation chain.

15.3 Many-to-one transformations

Multiple source constructs may produce one IR instruction.

The implementation MUST retain enough provenance to report meaningful diagnostics.

It MUST NOT arbitrarily select one source location and discard all others when doing so would make diagnostics misleading.

15.4 Optimization

Optimization MUST preserve or explicitly transform provenance.

This applies to:

- constant folding;
- dead-code elimination;
- inlining;
- vectorization;
- tensor transformations;
- quantum operation decomposition;
- quantum routing;
- HDL lowering;
- scheduling;
- distributed transformations.

15.5 Target independence

Source spans MUST remain independent of:

- CPU;
- GPU;
- FPGA;
- ASIC;
- QPU;
- accelerator;
- physical qubit;
- network node;
- memory bank;
- runtime location.

---

16. Generated Source and Macro Provenance

Generated syntax requires explicit provenance.

A generated construct MUST be represented as one of:

1. Direct source origin.
2. Expansion origin.
3. Compiler-generated origin.
4. Unknown origin.

16.1 Macro expansion

For macro expansion, the provenance model MUST distinguish:

macro invocation
macro definition
expanded syntax

Diagnostics SHOULD identify both the invocation and definition where relevant.

16.2 Included source

Imported or included source MUST retain its own source-file identity.

The parent file's span MUST NOT replace the included file's span.

16.3 Synthetic nodes

Synthetic nodes MUST NOT masquerade as concrete source nodes.

A synthetic node MAY point to an invocation or originating construct, but the relationship MUST be explicit.

16.4 Provenance cycles

Expansion and transformation provenance MUST avoid unbounded recursive ownership cycles.

A provenance graph MUST be traversable with resource-aware algorithms.

16.5 Source reconstruction

A synthetic span MUST NOT be used as proof that source text existed at that location.

Source reconstruction MUST use the original source snapshot and explicit transformation information.

---

17. Diagnostics Integration

Diagnostics MUST use canonical byte positions internally.

Human-facing rendering MUST derive coordinates from the source map.

Every diagnostic MUST have:

- a stable diagnostic identity or category;
- a primary location when available;
- a clear message;
- an appropriate severity;
- related locations where useful;
- deterministic ordering;
- explicit unknown-location behavior.

17.1 Invalid span diagnostics

The implementation MUST distinguish:

- invalid source identity;
- invalid revision;
- reversed span;
- out-of-bounds endpoint;
- invalid UTF-8 boundary;
- inconsistent cached coordinates;
- stale span;
- synthetic location;
- unknown location.

17.2 Diagnostic rendering

Rendering MUST NOT panic on malformed or stale spans.

If a source location cannot be resolved, the renderer MUST produce a safe fallback diagnostic without inventing line or column information.

17.3 Related locations

Multiple relevant source locations SHOULD be supported.

This is necessary for duplicate declarations, macro expansion, and multi-origin semantic errors.

17.4 Unicode rendering

The renderer MUST correctly highlight Unicode source.

Display width MUST NOT change the underlying byte interval.

---

18. IDE and LSP Integration

IDE/LSP coordinate conversion belongs to the tooling boundary.

The compiler's canonical position remains UTF-8 byte-based.

Tooling MUST explicitly handle the negotiated position encoding.

Conversion MUST be checked.

The following must never be assumed equivalent:

UTF-8 byte offset
Unicode scalar column
UTF-16 column
grapheme column
display column

Incremental editor updates MUST invalidate or translate affected spans explicitly.

Stale editor positions MUST NOT be silently reused.

---

19. Scalability Requirements

Source-span processing MUST scale with available resources.

The architecture MUST NOT define artificial universal ceilings for:

- source bytes;
- number of source files;
- number of lines;
- number of tokens;
- number of AST nodes;
- number of diagnostics;
- number of revisions;
- number of generated nodes;
- number of source origins.

19.1 Resource-aware behavior

Implementations MAY enforce configurable operational budgets.

Examples:

source registration budget
memory budget
diagnostic budget
incremental update budget
provenance traversal budget
compilation time budget

Budget exhaustion MUST produce an explicit diagnostic or operational result.

It MUST NOT silently corrupt spans.

19.2 Complexity

The implementation SHOULD provide:

Operation| Desired complexity
Source registration| O(n)
Span construction| O(1), excluding validation
Byte-position validation| O(1) or efficient equivalent
Line lookup| O(log L) or better
Span length| O(1)
Span containment| O(1)
Span intersection| O(1)
Source slicing| O(k), where k is requested slice size
Sequential lexer traversal| O(n)
Diagnostic coordinate resolution| O(log L) or better

The implementation MUST avoid quadratic source scanning.

19.3 Memory behavior

The implementation SHOULD:

- share source storage;
- store compact source identities;
- avoid copying source text into every span;
- avoid one allocation per character;
- avoid rescanning the entire source for every diagnostic;
- avoid unnecessary duplicated line indexes;
- release obsolete revisions when no longer referenced.

19.4 Deep provenance

Provenance traversal MUST be safe for deeply nested generated structures.

Use iterative traversal where recursion could cause stack exhaustion.

19.5 Tiny-input correctness

The implementation MUST correctly support:

- empty files;
- one-byte files;
- one-character files;
- a single newline;
- a single Unicode scalar;
- one token;
- one zero-width span.

Scalability does not justify breaking small-input correctness.

---

20. Determinism

Identical source snapshots and identical compiler configurations MUST produce identical:

- file identity assignment within the defined compilation context;
- byte offsets;
- span boundaries;
- line indexes;
- coordinate conversions;
- diagnostic locations;
- source provenance;
- serialized span data.

Results MUST NOT depend on:

- CPU architecture;
- operating system;
- hash-map iteration order;
- scheduling order;
- target hardware;
- available GPU/QPU devices;
- runtime timing.

Where IDs are assigned through unordered collections, deterministic ordering MUST be established before observable output is produced.

---

21. Serialization and Compatibility

Serialized source locations MUST identify their coordinate convention.

The serialized representation MUST distinguish:

- source file identity;
- revision identity;
- start byte;
- end byte;
- synthetic/unknown status;
- optional presentation coordinates.

A serialized span MUST NOT be applied to an unrelated source snapshot.

Deserialization MUST validate the representation.

Malformed data MUST return an error, not panic.

Changes to the source-position representation MUST be accompanied by:

- compatibility documentation;
- migration policy;
- versioning;
- tests;
- downstream consumer audit.

Do not silently reinterpret existing serialized "u32" offsets as a different coordinate system.

---

22. Validation Architecture

The validator MUST validate both structural invariants and integration behavior.

Validation is divided into the following layers.

22.1 Layer A: Representation

Validate:

- valid file identity;
- valid revision;
- ordered endpoints;
- in-bounds endpoints;
- valid UTF-8 boundaries;
- correct half-open intervals;
- correct EOF;
- explicit synthetic/unknown state.

22.2 Layer B: Source map

Validate:

- source registration;
- stable identities;
- revision handling;
- line indexes;
- byte-to-line lookup;
- coordinate conversion;
- checked arithmetic;
- safe slicing.

22.3 Layer C: Lexer

Validate:

- token start;
- token end;
- Unicode;
- comments;
- strings;
- malformed input;
- EOF;
- whitespace;
- CRLF;
- quantum literals;
- MTS literals.

22.4 Layer D: Parser

Validate:

- expression spans;
- statement spans;
- declaration spans;
- nested spans;
- missing-token spans;
- recovery spans;
- EOF handling.

22.5 Layer E: AST

Validate:

- every source-originating node;
- child containment;
- identifier spans;
- literal spans;
- transformed nodes;
- synthetic nodes.

22.6 Layer F: Semantic analysis

Validate:

- semantic-origin mapping;
- multi-origin diagnostics;
- imported declarations;
- macro-related locations;
- generated semantic entities.

22.7 Layer G: IR

Validate:

- source-to-IR provenance;
- one-to-many lowering;
- many-to-one optimization;
- transformation provenance;
- quantum IR provenance;
- HDL provenance.

22.8 Layer H: Tooling

Validate:

- diagnostics;
- source rendering;
- UTF-16 conversion where required;
- incremental edits;
- stale revisions;
- serialization.

---

23. Required Test Structure

Tests MUST be organized under the existing test hierarchy.

Recommended location:

grammar/tests/
├── diagnostics/
│   └── source-spans/
│       ├── basic/
│       ├── unicode/
│       ├── line-endings/
│       ├── eof/
│       ├── invalid/
│       ├── revisions/
│       ├── generated/
│       ├── macros/
│       ├── ast/
│       ├── semantic/
│       ├── ir/
│       ├── quantum/
│       ├── hdl/
│       ├── scalability/
│       ├── determinism/
│       └── compatibility/

Use existing Rust test modules and repository test conventions where those are already authoritative.

Do not duplicate tests unnecessarily.

Every test MUST have a clearly stated expected outcome.

---

24. Positive Tests

The following cases MUST pass.

24.1 Basic spans

empty source
single ASCII character
multiple ASCII characters
single token
multiple tokens
adjacent tokens
nested expressions
nested blocks
EOF

24.2 Unicode

π
λ
漢
🙂
é

Validate both byte positions and displayed coordinates.

24.3 Line endings

LF
CRLF
CR
empty line
multiple empty lines
final newline
no final newline

24.4 Language constructs

Validate spans for:

let x = 1;
fn f() {}
module example {}

Also cover:

- types;
- generic types;
- arrays;
- tensors;
- classical expressions;
- quantum operations;
- quantum measurements;
- hybrid control flow;
- HDL modules;
- hardware requirements;
- AI constructs;
- distributed constructs;
- effects;
- macros;
- Sankofa constructs;
- MTS constructs.

The source-span mechanism MUST be domain-neutral.

24.5 Large source inputs

Generate source inputs of increasing size.

Validate:

- monotonic offsets;
- exact EOF;
- line lookup;
- valid token boundaries;
- no integer truncation;
- no quadratic scanner behavior.

Tests MUST be parameterized rather than establishing an artificial universal maximum.

---

25. Negative Tests

The following MUST be rejected or explicitly represented as invalid.

Case| Expected behavior
Start greater than end| Reject
End beyond source length| Reject
Unknown file identity| Reject
Invalid revision| Reject
Invalid UTF-8 boundary| Reject
Integer conversion overflow| Return structured error
Span from wrong source| Reject
Stale revision| Reject or explicitly translate
Dummy identity used as concrete source| Reject
Cached coordinates inconsistent with source map| Detect
Cross-file join| Explicit failure or multi-origin result
Invalid serialized span| Reject
Invalid source encoding| Encoding diagnostic
Span slicing outside source| Safe failure
Provenance cycle| Detect or prevent
Unresolved synthetic span| Preserve explicit synthetic/unknown status

No malformed span may cause undefined behavior or a panic in normal diagnostic handling.

---

26. Boundary Tests

Boundary testing MUST cover:

1. Empty source "[0,0)".
2. First byte "[0,1)".
3. Final byte.
4. EOF "[N,N)".
5. Entire source "[0,N)".
6. Adjacent spans.
7. Zero-width insertion.
8. Multibyte character boundaries.
9. Attempted mid-character boundaries.
10. Empty line.
11. CRLF boundary.
12. Final newline.
13. Maximum representable implementation offset.
14. Checked offset addition.
15. Checked conversion.
16. Empty token stream.
17. Deeply nested syntax.
18. Large line count.
19. Large single-line source.
20. Large generated-source provenance.

Where a test would require impractical memory, use unit tests around checked arithmetic and representation boundaries.

Do not allocate enormous source strings merely to test integer overflow behavior.

---

27. Property-Based Tests

Property-based tests SHOULD be used for source-span invariants.

For every valid generated source and span:

start <= end
end <= source.len()

For concrete text spans:

source.is_char_boundary(start)
source.is_char_boundary(end)

For valid spans:

slice(start, end)

must return the exact original substring.

For adjacent spans:

[a,b) + [b,c)

must preserve ordering without overlap.

For line lookup:

byte position
    ->
line/column
    ->
validated byte position

must round-trip where the coordinate model supports an unambiguous conversion.

Randomized tests MUST use deterministic seeds in reproducibility-sensitive test environments.

Property tests MUST not assume ASCII-only source.

---

28. Determinism Tests

Run identical source snapshots repeatedly.

Verify identical:

- FileId assignments;
- offsets;
- token spans;
- AST spans;
- diagnostics;
- line indexes;
- serialized results.

Repeat under supported host configurations where CI permits.

No test may rely on unspecified hash-map iteration order.

---

29. Incremental Compilation Tests

The implementation MUST test edits that:

- insert bytes at the beginning;
- delete bytes at the beginning;
- modify a token;
- insert a Unicode scalar;
- delete a Unicode scalar;
- change LF to CRLF;
- insert a line;
- remove a line;
- modify a macro definition;
- modify an imported file;
- invalidate a source revision.

Old spans MUST remain associated with their original revision.

New spans MUST refer to the updated revision.

Any translation MUST use an explicit, validated edit map.

---

30. Cross-Domain Tests

The same source-span contract MUST apply to all domains.

30.1 Classical

Test variables, functions, arrays, matrices, tensors, and mathematical expressions.

30.2 Quantum

Test:

- quantum declarations;
- qubit registers;
- quantum literals;
- generic operations;
- custom operations;
- measurements;
- controls;
- adjoints;
- feed-forward;
- QEC annotations;
- logical operations.

The grammar MUST NOT introduce fixed qubit limits to support source spans.

30.3 HDL

Test:

- module declarations;
- signals;
- ports;
- registers;
- timing constructs;
- parameterized widths;
- generated hardware structures.

A source span MUST NOT depend on the physical width of a signal.

30.4 Hybrid

Test source locations crossing classical and quantum control-flow boundaries.

30.5 Distributed

Test imported declarations, remote computation descriptions, and generated distributed operations.

30.6 AI and data

Test tensors, model declarations, datasets, and generated transformations.

Every domain MUST use the same canonical byte-offset model.

---

31. Integration Requirements by File

This section defines the completion contract for each integration point.

31.1 "grammar/spec/source-spans.md"

Owns: Normative span semantics.

Must define:

- byte-offset model;
- half-open intervals;
- source identity;
- revisions;
- UTF-8;
- line and column policy;
- line endings;
- EOF;
- unknown and synthetic locations;
- provenance;
- scalability;
- serialization.

This validation file MUST reference that specification rather than silently duplicating or contradicting it.

31.2 "src/source_map.rs"

Owns: Executable source-map implementation.

Must implement and test:

- representable byte positions;
- checked conversions;
- source registration;
- stable file identity;
- source revisions;
- span validation;
- line indexes;
- Unicode coordinate conversion;
- EOF;
- safe slicing;
- structured failures;
- compatibility behavior.

Completion condition: All source-map unit, property, boundary, and determinism tests pass.

31.3 "src/lexer.rs"

Owns: Token source locations.

Must implement and test:

- forward UTF-8-safe scanning;
- monotonic byte cursor;
- exact token boundaries;
- correct EOF;
- comments;
- whitespace;
- Unicode;
- malformed tokens;
- quantum literals;
- MTS literals;
- safe large-source traversal.

Completion condition: Every emitted token and lexer diagnostic satisfies the source-span contract.

31.4 "src/parser.rs"

Owns: Syntax construct locations.

Must implement and test:

- full construct spans;
- nested construct spans;
- missing-token spans;
- recovery spans;
- EOF spans;
- progress guarantees;
- deep-structure behavior.

Completion condition: Every successfully constructed source-originating AST node has correct provenance.

31.5 "src/ast/mod.rs"

Owns: AST provenance.

Must define:

- concrete node locations;
- identifier locations;
- literal locations;
- synthetic-node behavior;
- recovery-node behavior;
- transformation preservation.

Completion condition: AST traversal can validate all source-originating nodes without inventing locations.

31.6 Semantic analysis

Owns: Semantic entity provenance.

Must preserve source origins for:

- declarations;
- references;
- types;
- effects;
- resource requirements;
- capabilities;
- quantum operations;
- HDL constructs.

Completion condition: Semantic diagnostics identify the correct source construct.

31.7 "src/ir_gen.rs"

Owns: AST-to-IR lowering.

Must:

- preserve source provenance;
- define one-to-many mappings;
- define many-to-one mappings;
- preserve synthetic origins;
- avoid fabricating spans.

Completion condition: Every generated IR construct has a valid provenance policy.

31.8 "src/ir_verify.rs"

Owns: IR validation.

Must verify:

- provenance references;
- source identity;
- transformation metadata;
- generated-origin validity.

Completion condition: Malformed provenance is rejected deterministically.

31.9 "src/quantum/ir/"

Owns: Canonical quantum IR provenance.

Must retain source-origin information through:

- quantum operation lowering;
- measurement;
- decomposition;
- logical operation construction;
- optimization.

Do not introduce a separate frontend quantum IR.

31.10 "grammar/grammar.md"

Must report the actual source-span implementation status.

Use distinct states:

SPECIFIED
IMPLEMENTED
PARTIALLY IMPLEMENTED
PLANNED
DEPRECATED

Do not claim complete source-span support merely because "Span" and "BytePos" exist.

31.11 "grammar/validation/README.md"

Must link to this validation specification and identify it as the source-span conformance contract.

It MUST NOT duplicate the normative source-span specification.

31.12 "grammar/tests/"

Must provide executable test coverage for all applicable categories in this document.

Tests MUST be connected to the actual Rust frontend, not merely illustrative examples.

---

32. Rust 1.97 / 1.97.1 Requirements

The implementation MUST compile using:

Rust 1.97
Rust 1.97.1
Edition 2021

The exact minimum supported toolchain MUST be documented in the repository's authoritative toolchain configuration.

Production source-span code MUST use safe Rust.

The implementation MUST NOT rely on:

- unchecked indexing;
- unchecked UTF-8 conversion;
- unchecked integer narrowing;
- wrapping offset arithmetic;
- platform-specific coordinate assumptions;
- undefined behavior;
- Rust "unsafe".

The absence of "unsafe" MUST be checked in the actual implementation and not inferred from this document.

---

33. CI and Validation Gates

Source-span validation MUST be included in the repository's normal test and CI workflow.

The required gates are:

1. Formatting
2. Compilation on the supported Rust baseline
3. Source-map unit tests
4. Lexer span tests
5. Parser span tests
6. AST provenance tests
7. Diagnostic tests
8. Unicode tests
9. Line-ending tests
10. Boundary tests
11. Property-based tests
12. Determinism tests
13. Incremental revision tests
14. Cross-domain tests
15. IR provenance tests
16. Quantum IR provenance tests
17. Safe-Rust audit
18. Scalability tests
19. Compatibility tests

A failure in any required gate MUST prevent the source-span implementation from being marked production-ready.

CI MUST distinguish infrastructure failures from test failures.

CI MUST NOT silently skip mandatory source-span validation.

---

34. Hard-Coding Audit

The source-span implementation MUST be audited for artificial limits.

The audit MUST search for suspicious constants and narrowing operations, including:

MAX_SOURCE_BYTES
MAX_FILE_SIZE
MAX_LINE_COUNT
MAX_LINE_LENGTH
MAX_TOKEN_LENGTH
MAX_SPAN_LENGTH
u32
as u32
as usize
saturating_add
wrapping_add

These patterns are audit triggers, not automatic defects.

Each occurrence MUST be classified.

For example:

- a documented protocol field width may be legitimate;
- an explicitly configured operational budget may be legitimate;
- an unchecked narrowing conversion is not legitimate;
- a universal source-size ceiling is prohibited.

The audit MUST also inspect all paths by which source offsets enter, leave, or are serialized from the compiler.

---

35. Diagnostics for Resource Exhaustion

The implementation MAY reject a compilation when actual resource availability is insufficient.

However, it MUST distinguish:

invalid source span

from:

source-map resource budget exhausted

and:

source revision unavailable

and:

source offset not representable

The implementation MUST NOT convert resource exhaustion into a misleading syntax diagnostic.

When an operational budget is exhausted, the compiler MUST preserve already validated source provenance wherever possible.

---

36. Production Acceptance Criteria

Source-span validation is production-ready only when every mandatory condition below is satisfied.

Representation

- [ ] Canonical UTF-8 byte positions.
- [ ] Half-open intervals.
- [ ] Checked arithmetic.
- [ ] No silent integer truncation.
- [ ] Valid file identities.
- [ ] Explicit revision behavior.
- [ ] Explicit unknown/synthetic behavior.
- [ ] Correct EOF.
- [ ] Correct zero-width spans.

Source map

- [ ] Stable file identity.
- [ ] Efficient line index.
- [ ] Correct Unicode handling.
- [ ] Correct line-ending behavior.
- [ ] Safe slicing.
- [ ] No quadratic position lookup.
- [ ] No artificial language-level source limit.

Lexer

- [ ] Every token has a valid span.
- [ ] UTF-8 scanning is correct.
- [ ] Token boundaries are exact.
- [ ] EOF is correct.
- [ ] Error spans are correct.
- [ ] Quantum literals are covered.
- [ ] MTS literals are covered.
- [ ] Large-source scanning is efficient.

Parser and AST

- [ ] Complete construct spans.
- [ ] Correct nested spans.
- [ ] Correct recovery spans.
- [ ] Correct missing-token spans.
- [ ] Synthetic nodes are distinguishable.
- [ ] Deep nesting is handled safely.

Semantic and IR

- [ ] Semantic provenance is preserved.
- [ ] Multi-origin entities are supported.
- [ ] AST-to-IR provenance is defined.
- [ ] One-to-many lowering is supported.
- [ ] Many-to-one transformation is supported.
- [ ] Quantum provenance reaches "quantum::ir".
- [ ] HDL and classical provenance are preserved.
- [ ] Optimization does not silently discard provenance.

Tooling

- [ ] Diagnostics are deterministic.
- [ ] Unicode rendering is correct.
- [ ] LSP conversion is explicit.
- [ ] Stale revisions are rejected or translated.
- [ ] Serialization is validated.

Safety and scalability

- [ ] Rust 1.97 compatibility.
- [ ] Rust 1.97.1 compatibility.
- [ ] Edition 2021.
- [ ] No Rust "unsafe".
- [ ] No unchecked UTF-8 operations.
- [ ] No unchecked offset narrowing.
- [ ] Resource exhaustion is explicit.
- [ ] Scalability tests pass.
- [ ] Determinism tests pass.

Repository integration

- [ ] "grammar/spec/source-spans.md" agrees with implementation.
- [ ] "grammar/DESIGN.md" remains authoritative architecturally.
- [ ] "src/source_map.rs" implements the contract.
- [ ] "src/lexer.rs" conforms.
- [ ] "src/parser.rs" conforms.
- [ ] "src/ast/" preserves provenance.
- [ ] Semantic analysis preserves provenance.
- [ ] "src/ir_gen.rs" preserves provenance.
- [ ] "src/ir_verify.rs" validates provenance.
- [ ] "quantum::ir" retains source origins.
- [ ] "grammar/grammar.md" reports actual implementation status.
- [ ] CI executes the required tests.

---

37. Definition of Done

This file is complete when its validation requirements, ownership boundaries, integration contracts, failure behavior, tests, and acceptance criteria are fully defined.

Its completion does not automatically establish that the Rust implementation is production-ready.

The implementation is complete only when the repository passes all applicable acceptance criteria.

Once this specification is accepted, another file MUST NOT require changing its normative span semantics.

If a downstream implementation discovers a genuine contradiction, the change MUST follow the repository's specification-versioning and compatibility process.

The validation file MUST remain stable while independently implemented components conform to it.

---

38. Final Architectural Guarantee

Zamani source spans describe where source constructs originate.

They do not describe where computations execute.

The same source-span model MUST work for:

Classical
Quantum
Hybrid
HDL
AI
Tensor
Distributed
Networking
Security
Sankofa
MTS
Nano
Future computational domains

The source-span architecture MUST remain independent of physical hardware and available resources.

The implementation may use wider representations, additional indexing, streaming, and resource-aware algorithms as required.

It MUST NOT turn current implementation capacities into permanent language restrictions.

The production guarantee is:

One canonical source-position model
            |
            v
One consistent source-map authority
            |
            v
Correct lexer and parser provenance
            |
            v
Correct AST and semantic provenance
            |
            v
Correct canonical IR provenance
            |
            v
Correct diagnostics and tooling
            |
            v
Portable, deterministic, scalable source identity

POCO-REAF invariant: A Zamani program's source locations remain stable and meaningful regardless of whether its computation is compiled for an embedded processor, CPU, GPU, FPGA, ASIC, QPU, simulator, distributed system, HPC cluster, cloud environment, or future computational target.

No artificial source-size ceiling. No target-dependent offsets. No fabricated provenance. No Rust "unsafe".