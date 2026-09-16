Zamani Source Map and Source Location Specification

Path: "grammar/spec/source-map.md"
Language: Zamani
Specification role: Normative source-location and source-mapping contract
Status: Production architecture
Specification version: 1.0
Implementation baseline: Rust 1.97 / Rust 1.97.1
Rust edition: 2021
Rust safety policy: Safe Rust only; production compiler implementation MUST NOT use Rust "unsafe"
Portability objective: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)
Scalability objective: Source mapping MUST scale from the smallest supported source unit to arbitrarily large compilations subject only to actual implementation/resource availability and explicitly declared resource policies.
Canonical quantum semantic boundary: "quantum::ir"

---

0. Purpose

This specification defines the normative source-location contract for the Zamani compiler.

It defines how the compiler represents, preserves, resolves, combines, and reports locations originating from Zamani source text.

The source-map system exists so that every diagnostic, syntax construct, semantic construct, transformation, generated artifact, and relevant compiler event can be traced back to its originating source location whenever such an origin exists.

The source-map system MUST support:

- source files;
- source identities;
- source names;
- source contents;
- byte offsets;
- source spans;
- line/column information;
- Unicode-aware source handling;
- multi-file compilation;
- imported modules;
- generated source;
- macro expansion;
- included/generated fragments where supported;
- diagnostics;
- AST source locations;
- semantic source locations;
- IR provenance;
- compiler diagnostics;
- IDE/LSP integration;
- deterministic behavior;
- incremental compilation;
- reproducible builds;
- serialization where required;
- very large source files;
- very large compilation graphs;
- safe Rust implementation.

The source-map layer MUST NOT own:

- parsing;
- lexical semantics;
- type checking;
- name resolution;
- ownership;
- effects;
- quantum semantics;
- QEC;
- ZQN;
- routing;
- scheduling;
- hardware selection;
- resource allocation;
- optimization;
- runtime execution.

The fundamental boundary is:

source bytes
    ↓
source identity
    ↓
source map
    ↓
source locations / spans
    ↓
lexer
    ↓
parser
    ↓
AST
    ↓
semantic analysis
    ↓
canonical semantic model
    ↓
IR
    ↓
optimization / lowering / routing / scheduling / resilience
    ↓
target realization

Source mapping is therefore a cross-cutting provenance service, not another compiler semantic layer.

---

1. Architectural Authority

The source-location authority relationship is:

grammar/spec/source-map.md
            │
            ├───────────────┐
            │               │
            ▼               ▼
      language rules    source-location
                        implementation
                            │
                            ▼
                    src/source_map.rs
                            │
              ┌─────────────┼─────────────┐
              ▼             ▼             ▼
             Lexer        Parser          AST
              │             │             │
              └─────────────┼─────────────┘
                            ▼
                   semantic diagnostics
                            │
                            ▼
                    canonical IR
                            │
                            ▼
                 compiler / tooling / IDE

The normative authority for source-location semantics is this specification.

The Rust implementation is:

src/source_map.rs

The lexical integration contract is:

grammar/spec/lexical.md

The syntax contract is:

grammar/spec/syntax.md

The AST integration contract is:

src/ast/

The parser integration is:

src/parser.rs

The lexer integration is:

src/lexer.rs

The source-map specification MUST NOT become a second implementation.

---

2. File Ownership

2.1 This file owns

This specification owns:

- source identity semantics;
- source-file identity;
- byte-position semantics;
- span semantics;
- location validity;
- source-file relationships;
- line mapping requirements;
- column mapping requirements;
- Unicode location rules;
- source provenance;
- source-origin relationships;
- generated-source relationships;
- macro-source relationships;
- multi-file source mapping;
- span composition;
- span preservation;
- source-map determinism;
- source-map scalability;
- source-map diagnostics;
- source-map serialization requirements;
- source-map compatibility requirements.

2.2 This file does not own

This file does not own:

- token definitions;
- keyword definitions;
- parser grammar;
- AST node structure;
- type semantics;
- quantum semantics;
- hardware semantics;
- IR semantics;
- runtime behavior;
- target selection;
- routing;
- scheduling;
- QEC;
- ZQN;
- HAL;
- optimization algorithms.

---

3. Existing Repository Integration

The current repository already provides:

src/source_map.rs
src/lexer.rs
src/parser.rs
src/ast/

The existing source-map implementation provides:

FileId
BytePos
Span
SourceFile
SourceMap

and source files retain shared source contents through "Arc<String>".

The lexer imports:

use crate::source_map::{BytePos, FileId, SourceFile, Span};

and therefore already establishes the intended lexer/source-map integration.

The parser imports "Span" and attaches spans to parser errors.

The AST documentation states that every node carries a "Span" for precise diagnostics.

These existing interfaces MUST be preserved unless a repository-wide compatibility decision explicitly changes them.

The source-map specification therefore extends the existing architecture rather than requiring an unnecessary rename or replacement.

---

4. Integration Contract

Component| Responsibility
"grammar/spec/source-map.md"| Normative source-location contract
"src/source_map.rs"| Source-map implementation
"src/lexer.rs"| Token source locations
"src/parser.rs"| Parser source locations and parser diagnostics
"src/ast/"| AST source locations
semantic analysis| Semantic diagnostic provenance
IR generation| Source provenance preservation
IR verification| Diagnostic provenance
compiler| Transformation provenance
runtime/debugging| Runtime-to-source correlation where supported
IDE/LSP tooling| Source-location presentation
"grammar/tests/"| Conformance tests
"grammar/spec/diagnostics.md"| Diagnostic integration
"grammar/spec/lexical.md"| Lexical location requirements
"grammar/spec/syntax.md"| Syntax location requirements
"grammar/spec/semantics.md"| Semantic provenance requirements

No downstream component may assume that source locations correspond to physical hardware, physical memory, physical qubits, CPU cores, GPU IDs, FPGA resources, or other target-specific resources.

---

5. Fundamental Source Identity Model

Every source-bearing artifact MUST have a stable identity within a compilation session.

A source identity MUST distinguish files even when:

- their names are equal;
- their contents are equal;
- they originate from different modules;
- they occur in different compilation roots;
- they are generated independently.

The identity is conceptually:

SourceId
    ↓
SourceFile
    ↓
SourceContent

The current implementation uses:

FileId

for this purpose.

The implementation MAY evolve from "FileId" to another internal representation in a future compatible release, but the semantic requirement remains:

«A source span MUST identify exactly which source unit it belongs to.»

---

6. File Identity MUST NOT Be the File Path

A path is metadata.

A path is not sufficient as a source identity because:

- aliases may exist;
- relative paths may differ;
- virtual files may have no path;
- generated sources may not have a filesystem path;
- two compilation units may temporarily use the same display name;
- remote/imported sources may use logical identifiers;
- editor buffers may not correspond to filesystem files.

Therefore:

FileId ≠ filesystem path

A source file MAY have:

display name
logical URI
filesystem path
module path
package identity
content identity

without any of those replacing the internal source identity.

---

7. Source Identity Lifetime

A "FileId" MUST be unique within its source-map domain.

The minimum required domain is one compilation session.

Implementations SHOULD ensure deterministic assignment when deterministic compilation requires it.

The source-map implementation MUST NOT rely on:

- pointer addresses;
- memory addresses;
- object allocation order outside controlled compilation state;
- hash iteration order;
- thread scheduling;
- operating-system process identifiers;
- wall-clock timestamps;
- random identifiers.

The identity MUST remain stable while the source-map instance is alive.

---

8. Dummy / Synthetic Source Locations

The existing implementation reserves:

FileId(0)

for a dummy span.

This is an acceptable implementation mechanism provided the semantic contract remains:

«A dummy span represents the absence of a real source location.»

A dummy location MUST NOT be presented to users as though it originated from source text.

A diagnostic renderer MUST distinguish:

real source location

from:

no source location

A synthetic compiler object MAY have:

no source location

without being erroneous.

However, the compiler SHOULD preserve provenance whenever a real origin exists.

---

9. Byte Position Model

Source positions are measured in UTF-8 byte offsets.

Conceptually:

BytePos = number of bytes from the beginning of the source

The first byte has offset:

0

The position immediately after the final byte has offset:

source_length

This permits half-open intervals.

The canonical span representation is:

[start, end)

where:

start <= end

and:

length = end - start

The current implementation uses "BytePos(u32)".

That implementation detail is not sufficient as a universal scalability guarantee.

If a future compiler must support source files whose byte offsets exceed the representable range of the current implementation, the representation MUST be widened before such sources are accepted.

The language MUST NOT impose a semantic source-size ceiling merely because the current implementation uses a particular integer width.

---

10. Source-Map Scalability Rule

The distinction is:

language limit
        ≠
implementation representation limit
        ≠
resource budget

For example:

BytePos representation capacity

is an implementation property.

It MUST NOT become:

Zamani source files may never exceed N bytes

unless such a limit is explicitly adopted as a language specification requirement.

Production implementations MAY impose resource budgets.

Such a budget MUST produce an explicit resource diagnostic such as:

ZMN-RESOURCE-SOURCE-SIZE

rather than pretending that the source is syntactically invalid.

---

11. Span Model

A span represents a contiguous source region.

Conceptually:

Span {
    source_id,
    start,
    end,
}

The existing implementation additionally stores:

start_line
start_column

inside "Span".

This is permitted.

However, the normative identity of a span is its source identity and byte interval.

Line/column values are derived location information and MUST remain consistent with the source map.

---

12. Span Validity

A valid non-dummy span MUST satisfy:

start <= end

and both positions MUST refer to the same source file.

A span MUST NOT contain positions outside its source file.

An implementation MUST NOT silently construct a span whose end exceeds the source length.

Invalid internal spans MUST be treated as compiler implementation errors rather than user syntax errors.

---

13. Empty Spans

An empty span satisfies:

start == end

Empty spans are valid.

They are useful for:

- insertion points;
- missing-token diagnostics;
- zero-width diagnostics;
- generated constructs;
- end-of-file locations;
- recovery points.

An empty span MUST still have a valid source identity unless it is explicitly synthetic.

---

14. Span Containment

For spans belonging to the same source:

A.contains(B)

is true when:

A.start <= B.start
B.end <= A.end

Spans from different source files MUST NOT be treated as mutually containing.

The existing implementation already checks source identity when evaluating containment.

---

15. Span Ordering

Span ordering MUST be deterministic.

Within the same source file, implementations SHOULD order spans by:

1. start offset;
2. end offset.

Source IDs MUST be considered before byte offsets when comparing spans from different sources.

The ordering MUST NOT depend on:

- hash-map order;
- thread execution order;
- memory address;
- filesystem enumeration order.

---

16. Span Combination

Combining two spans is valid only when they belong to the same source unless an explicit multi-source provenance structure is used.

The current implementation's "Span::merge" asserts that both spans belong to the same file.

For production diagnostics, callers SHOULD validate source compatibility before requesting a merge.

A cross-file construct MUST NOT be represented by pretending that two unrelated files occupy one contiguous source range.

Instead use a provenance structure such as:

PrimarySpan
RelatedSpan*

or an equivalent diagnostic representation.

---

17. Diagnostic Primary and Related Locations

A diagnostic MUST distinguish:

primary location

from:

related locations

Example:

error:
    use of undeclared value `x`

primary:
    source.zm:10:5

related:
    declaration expected here
    source.zm:3:5

A diagnostic MUST NOT merge unrelated locations into a false contiguous span.

---

18. Line Mapping

The source map MUST provide deterministic mapping from byte offset to line information.

The canonical line model is:

line 1 begins at byte 0

Line numbers are one-based for user-facing diagnostics.

The source-map implementation currently precomputes line starts and performs binary search to resolve line information.

That is an appropriate implementation strategy.

The implementation MUST preserve correct behavior for:

empty files
single-line files
multi-line files
final newline
no final newline
multiple consecutive newlines
Unicode text
large files
empty lines

---

19. Newline Semantics

The source-map system MUST define newline handling independently from operating-system conventions.

At minimum, the implementation MUST consistently handle:

LF
CRLF
CR

according to the canonical lexical specification.

The compiler MUST NOT produce different source locations merely because the same source is processed on different operating systems.

For normalized source handling, the source bytes MUST still remain available for exact diagnostics where required.

---

20. Columns

Columns MUST be defined explicitly.

The canonical internal source position is the byte offset.

User-facing columns MAY be represented as:

Unicode scalar columns

or:

display columns

but the selected model MUST be documented and deterministic.

The implementation MUST NOT confuse:

byte offset

with:

Unicode scalar index

or:

terminal display width

For example, a multi-byte UTF-8 character MUST NOT cause the compiler to treat its second byte as the beginning of another user-visible character.

---

21. Unicode Source Mapping

Zamani source is UTF-8.

Source mapping MUST preserve exact byte offsets in the original UTF-8 source.

For Unicode text:

byte position

remains authoritative for source identity and slicing.

User-facing line/column information is derived from valid UTF-8 boundaries.

A diagnostic MUST NOT point into the middle of a UTF-8 code point unless it is explicitly reporting malformed source bytes.

---

22. Invalid UTF-8

Invalid UTF-8 MUST be rejected before normal Unicode lexical processing.

Invalid bytes MUST NOT silently become:

U+FFFD

or another replacement character.

If invalid UTF-8 is encountered, the compiler MUST emit a source-encoding diagnostic.

This follows the lexical contract established by "grammar/spec/lexical.md".

The source-map layer MUST preserve enough byte information to report the invalid region accurately.

---

23. Source Slicing

Source slicing MUST operate on byte ranges known to be valid UTF-8 boundaries whenever the requested result is interpreted as text.

The implementation MUST NOT use unchecked byte indexing requiring Rust "unsafe".

Safe Rust APIs MUST be used.

If an internal operation requests an invalid UTF-8 boundary, the compiler MUST handle the condition deterministically rather than invoking undefined behavior.

---

24. Source Content Ownership

The source map MUST provide stable access to source content for as long as diagnostics or compiler structures may require it.

The current implementation uses "Arc<String>" to share source contents without unnecessary copies.

This is compatible with the architectural goal.

The implementation MAY later use another safe representation such as:

Arc<str>

or an equivalent safe storage abstraction.

Such an implementation change MUST NOT change source-location semantics.

---

25. Source Content Immutability

Once a source file has been registered in a compilation source map, its source contents SHOULD be immutable.

This guarantees:

Span
    ↓
same bytes

for the lifetime of the compilation.

Mutation of source content after tokenization would invalidate spans and is therefore prohibited unless the entire source map and all dependent compiler artifacts are deliberately rebuilt or use an explicitly versioned incremental model.

---

26. Incremental Compilation

Incremental compilation MUST treat source versions as distinct source states.

A source edit MUST NOT silently mutate the meaning of existing spans.

Conceptually:

file.zm@version A

and:

file.zm@version B

are different source states.

Caches MUST NOT reuse source-dependent artifacts merely because their display path is equal.

At minimum, cache identity SHOULD account for:

source identity
source version/content identity
language version
relevant configuration

---

27. Multi-File Compilation

A compilation MAY contain any number of source files subject only to actual resource availability.

The source map MUST NOT encode a fixed maximum number of files.

It MUST support:

one file
many files
modules
packages
generated files
foreign sources
virtual sources

without changing the source-location model.

---

28. Module Integration

Module resolution MUST retain source provenance.

For:

import foo

the compiler MUST be able to identify:

importing source
import location
resolved module
module source

when producing relevant diagnostics.

A module path MUST NOT replace source identity.

---

29. Virtual Sources

The compiler MAY support source units that do not originate from ordinary filesystem files.

Examples include:

editor buffers
stdin
generated source
macro expansion
REPL input
embedded source
remote source
test fixtures

Every virtual source MUST still have a source identity.

The display name MAY be:

<stdin>
<generated>
<macro>
<repl>

or another stable logical name.

---

30. Generated Sources

Generated source MUST preserve provenance whenever possible.

For example:

generated_file
    ↓
generated span
    ↓
originating source span(s)

A generated construct MUST NOT falsely claim that generated text was literally written at a source location where it was not.

Generated-source provenance MAY be represented separately from ordinary "Span".

---

31. Macro Expansion Provenance

Macros require special treatment.

A macro-expanded AST node MAY have:

expansion span

and:

origin span

These are not necessarily identical.

The compiler MUST be able to distinguish:

where the macro was invoked

from:

where the generated syntax originated

and, when applicable:

where the macro definition originated

A production diagnostic SHOULD show the most useful user-facing location while retaining related provenance.

---

32. Provenance Chain

For generated or transformed constructs, provenance MAY form a chain:

source
  ↓
macro
  ↓
generated AST
  ↓
semantic construct
  ↓
IR
  ↓
lowered operation

The compiler SHOULD preserve enough information to reconstruct relevant origin relationships.

The chain MUST remain finite for any individual compiler artifact.

Recursive transformations MUST NOT create unbounded diagnostic recursion.

---

33. AST Integration

The AST MUST preserve source locations.

The existing AST explicitly documents that every node carries a "Span".

This contract is retained.

Every AST node representing source syntax SHOULD have a meaningful span.

For example:

Program
Function
Statement
Expression
Type
Pattern
Attribute

must retain source provenance where they correspond to source text.

Synthetic AST nodes MAY have no real source span.

---

34. AST Span Requirements

An AST span SHOULD cover the smallest useful syntactic construct that provides meaningful diagnostics.

For example, for:

let x = value;

the declaration span should cover the declaration.

The identifier "x" SHOULD have its own identifier span where the AST representation permits it.

The initializer SHOULD have its own span.

This allows diagnostics to point precisely at:

x
value
whole declaration

rather than only the complete source file.

---

35. Parser Integration

The parser MUST propagate source spans from tokens into syntax structures.

The existing parser already stores "Span" in "ParserError".

Parser diagnostics MUST identify:

- unexpected token;
- expected token;
- malformed expression;
- malformed declaration;
- delimiter mismatch;
- invalid construct;
- recovery location.

Parser recovery MUST preserve valid source locations for subsequently parsed constructs.

---

36. Lexer Integration

Every token SHOULD carry:

token kind
source span

The current lexer architecture already imports "Span", "FileId", "BytePos", and "SourceFile".

The lexer MUST NOT attach target-dependent metadata to source spans.

A token's source location describes source text only.

It does not describe:

CPU
GPU
QPU
FPGA
physical qubit
memory bank
network node

---

37. Source Map and Quantum Compilation

Quantum source constructs MUST retain source provenance through:

quantum syntax
    ↓
frontend AST
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
QEC / resilience
    ↓
ZQN
    ↓
HAL
    ↓
target

"quantum::ir" remains the canonical quantum semantic boundary.

Source mapping MUST NOT create a second quantum IR merely to preserve locations.

Instead, quantum IR operations SHOULD carry or reference source provenance through the repository's established IR metadata/provenance mechanism.

---

38. Source Map and Classical Compilation

Classical operations MUST preserve source provenance through:

AST
    ↓
semantic analysis
    ↓
classical IR
    ↓
optimization
    ↓
lowering
    ↓
backend

Optimization MUST NOT erase all source provenance merely because an operation is transformed.

When several source operations become one optimized operation, the resulting operation MAY have:

primary source span
+
related source spans

or an equivalent provenance representation.

---

39. Source Map and HDL

HDL constructs MUST retain source provenance through:

HDL syntax
    ↓
AST
    ↓
hardware semantic model
    ↓
hardware/HDL IR
    ↓
synthesis/lowering
    ↓
target

A synthesized hardware element MUST NOT be reported as though the exact generated implementation had been explicitly written by the programmer.

The compiler SHOULD identify the originating HDL construct when diagnostics can be meaningfully traced back.

---

40. Source Map and Hardware Portability

Source locations MUST remain independent of target hardware.

A span MUST NOT encode:

physical CPU
GPU index
QPU index
physical qubit
FPGA tile
memory bank
network node
accelerator ID

Those belong to later target realization.

This preserves POCO-REAF.

---

41. Source Map and Resource Scaling

The source map MUST scale with:

number of source files
source size
number of tokens
number of AST nodes
number of diagnostics
number of generated artifacts

without language-level fixed ceilings.

An implementation MAY apply resource budgets.

Resource budgets MUST be separate from source-language validity.

For example:

SOURCE_MAP_MEMORY_BUDGET

may be a compiler configuration.

It MUST NOT become:

Zamani supports only N source files

as a language rule.

---

42. Large-Source Requirements

Production implementations SHOULD avoid unnecessary whole-source duplication.

They SHOULD prefer:

shared immutable source storage

over repeated copies.

The current "Arc<String>" implementation is aligned with this requirement.

Large source handling MUST NOT require Rust "unsafe".

Implementations SHOULD consider:

- shared source buffers;
- compact source IDs;
- compact span representations;
- lazy line indexing;
- incremental line indexing;
- memory-efficient diagnostics;
- bounded diagnostic rendering;
- streaming input where appropriate.

---

43. Line Index Scalability

The current implementation stores a "Vec<BytePos>" containing line starts.

This provides efficient binary-search lookup.

For very large files, implementations MAY replace this with:

paged line indexes
compressed indexes
lazy indexes
incremental indexes

provided observable source-location behavior remains identical.

The representation MUST NOT introduce an artificial language-level maximum number of lines.

---

44. Diagnostic Scalability

A pathological source file MUST NOT cause unbounded diagnostic memory growth.

Compiler resource budgets MAY cap:

number of emitted diagnostics
diagnostic rendering size
stored related locations

but truncation MUST be explicit.

The compiler MUST distinguish:

source has errors

from:

diagnostic collection budget exhausted

---

45. Determinism

For identical:

source bytes
source identities
language version
compiler configuration
compatibility mode

the source-map system MUST produce deterministic:

FileId assignment where deterministic assignment is part of the compilation mode
line mapping
span boundaries
source lookup
diagnostic ordering

Source mapping MUST NOT depend on:

- CPU count;
- GPU availability;
- QPU availability;
- operating-system locale;
- filesystem enumeration order;
- network state;
- random state;
- hash iteration order;
- thread scheduling.

---

46. Parallel Compilation

The source map MAY be accessed by parallel compiler stages.

The implementation MUST use safe Rust synchronization/ownership mechanisms.

It MUST NOT require Rust "unsafe".

Parallel compilation MUST NOT produce different source spans for the same deterministic compilation inputs.

---

47. Thread Safety

If "SourceMap" is shared across compilation tasks, the implementation MUST establish explicit ownership and synchronization semantics.

Possible safe strategies include:

immutable shared source map
Arc
read/write synchronization
message passing
thread-local handles referencing stable source IDs

The specification does not require a specific strategy.

The observable source-location contract remains fixed.

---

48. No Hidden Global Source State

Source maps MUST NOT rely on hidden process-global mutable state.

This includes:

global FileId counters
global source buffers
global current file
global current line
global current compiler

A compilation MUST explicitly own or receive its source-map context.

This is necessary for:

- deterministic builds;
- parallel compilation;
- embedded compiler use;
- testing;
- IDE integration;
- multiple simultaneous compilations.

---

49. Reproducibility

Source locations MUST be reproducible.

A build performed twice with equivalent inputs MUST NOT produce different source identities merely because:

temporary directory names differ
filesystem traversal order differs
thread scheduling differs

Logical source identity and display path MUST be separated.

---

50. Serialization

If source maps are serialized into:

incremental caches
compiler artifacts
debug information
diagnostic artifacts
IDE state
build metadata

the serialized representation MUST be versioned.

Serialization MUST NOT assume:

usize
pointer width
host endianness
host architecture

as semantic language properties.

Portable serialized source-map data SHOULD use explicitly sized integer representations.

---

51. Cross-Platform Behavior

The source-map contract MUST behave consistently across:

Linux
Windows
macOS
embedded targets
cloud environments
containers
WASM
future supported targets

where the compiler itself is supported.

Filesystem paths are platform-specific metadata.

Source byte offsets and language-defined line semantics are not platform-specific.

---

52. Path Normalization

Path normalization MUST NOT alter source identity semantics.

If paths are normalized, the compiler MUST define:

- separator handling;
- relative path resolution;
- case sensitivity;
- symbolic links;
- URI conversion;
- workspace roots;
- package roots.

Two different logical source identities MUST NOT accidentally collapse merely because their display paths normalize to the same string.

---

53. Source Names and User Diagnostics

A source file SHOULD have a human-readable display name.

Examples:

main.zm
math/vector.zm
<stdin>
<generated>
<macro expansion>

The display name is diagnostic metadata.

It MUST NOT be used as the sole source identity.

---

54. Source Map Lookup

The source map MUST provide a deterministic lookup operation conceptually equivalent to:

SourceId → SourceFile

A missing source ID MUST be represented explicitly.

It MUST NOT silently return an unrelated file.

The existing implementation provides:

get_file(file_id)

returning an optional source file.

This is consistent with the contract.

---

55. Source Line Lookup

The source map SHOULD provide a source-line lookup capability.

The current implementation provides:

get_source_line(file_id, line_num)

which returns an optional line string.

The contract requires that:

- invalid file IDs return no source;
- invalid line numbers return no source;
- valid lines are deterministic;
- returned source text corresponds to the original source;
- Unicode is preserved;
- source mutation is not silently reflected into old spans.

---

56. End-of-File Locations

The compiler MUST support an EOF source position.

For a source of byte length "N":

EOF = BytePos(N)

An EOF diagnostic MAY use:

[N, N)

as its zero-width span.

This is especially important for:

unexpected EOF
missing delimiter
unfinished string
unfinished declaration
unfinished module

diagnostics.

---

57. Missing-Token Diagnostics

When the parser reports a missing token, the source span SHOULD normally be the insertion point where the token was expected.

Example:

fn main( {

The diagnostic can point to:

the location immediately before `{`

rather than manufacturing a span over unrelated text.

---

58. Delimiter Diagnostics

For:

(
)
[
]
{
}

the parser SHOULD preserve both:

opening delimiter span
closing delimiter span

when available.

A mismatched delimiter diagnostic SHOULD use:

primary span = offending delimiter
related span = opening delimiter

instead of merging them.

---

59. Diagnostics and Source Ownership

Every user-facing diagnostic SHOULD have:

diagnostic code
severity
message
primary source location
optional related locations
optional notes

Source mapping supplies the locations.

Diagnostic semantics belong to:

grammar/spec/diagnostics.md

The source-map specification does not define the wording of every diagnostic.

---

60. Error Recovery

Source-map correctness MUST survive lexer and parser recovery.

Recovery MUST NOT:

- create invalid source offsets;
- assign spans from unrelated files;
- mutate existing source content;
- silently shift source locations;
- create non-deterministic locations.

A recovered AST node MAY have a synthetic or partial span if the original source construct was incomplete.

---

61. Synthetic Nodes

Compiler-generated AST or semantic nodes MAY have:

synthetic = true

or an equivalent representation.

Synthetic nodes MUST NOT be indistinguishable from real source nodes when diagnostics depend on provenance.

If a synthetic node is derived from source syntax, the implementation SHOULD preserve the originating source span separately.

---

62. IR Provenance

IR lowering MUST preserve source provenance whenever practical.

The minimum desired relationship is:

IR operation
    ↓
source provenance
    ↓
source span(s)

The provenance representation MUST NOT require creation of a parallel IR merely for source locations.

This is particularly important for:

quantum::ir
classical IR
HDL/hardware IR

---

63. Optimization Provenance

Optimization may transform:

A
B
C

into:

D

The resulting operation "D" SHOULD retain provenance to the relevant source operations.

It MUST NOT falsely claim that "D" was directly written by the programmer if it was synthesized by optimization.

Possible provenance:

D
├── source A
├── source B
└── source C

---

64. Dead-Code Elimination

When source code is eliminated, its source locations remain available in the original AST/semantic model as long as those objects remain alive.

The compiler SHOULD retain enough provenance to report diagnostics involving eliminated constructs when required.

An optimized executable does not need to preserve the entire source map at runtime.

---

65. Constant Folding

For:

let x = 1 + 2;

if the compiler folds:

1 + 2 → 3

the resulting constant SHOULD preserve provenance to:

1
2
+

or to the smallest meaningful enclosing source span.

---

66. Macro and Metaprogramming Safety

Macros and metaprogramming MUST NOT be allowed to forge arbitrary source locations that point to unrelated user files.

A generated diagnostic location MUST correspond to an actual provenance relationship.

This prevents diagnostics from becoming misleading or security-sensitive.

---

67. Interoperability

Foreign-language integration MAY require foreign source locations.

For:

Zamani → C
Zamani → C++
Zamani → Rust
Zamani → QIR
Zamani → OpenQASM
Zamani → HDL

the compiler SHOULD retain Zamani-origin provenance.

Foreign source locations MAY be stored as related provenance records.

The foreign representation MUST NOT replace the original Zamani source location.

---

68. Quantum Interoperability

When Zamani lowers to OpenQASM, QIR, or another quantum representation, the compiler SHOULD preserve:

Zamani operation
    ↓
foreign operation

source relationships.

For example:

apply H to q

must remain traceable to the original Zamani source even if lowered through several representations.

---

69. Resource and Hardware Diagnostics

Resource diagnostics may ultimately refer to target-specific conditions.

For example:

requested capability unavailable

The diagnostic MUST still retain the source location of the program construct that requested the capability.

The source map MUST NOT itself know:

how many QPUs exist
how many physical qubits exist
how much RAM exists
how many GPUs exist

Those facts belong to resource analysis/HAL/backend systems.

---

70. POCO-REAF Requirement

Source locations MUST be independent of target realization.

A source span describes:

WHERE IN THE SOURCE

not:

WHERE ON THE MACHINE

Therefore a program compiled for:

CPU
GPU
FPGA
QPU
distributed cluster
future accelerator

retains the same source provenance.

Target-specific lowering may change implementation, but it MUST NOT change the meaning of the original source spans.

---

71. No Hard-Coded Resource Limits

The source-map implementation MUST NOT introduce:

MAX_FILES
MAX_SOURCE_BYTES
MAX_LINES
MAX_COLUMNS
MAX_SPANS
MAX_DIAGNOSTICS
MAX_MODULES
MAX_GENERATED_FILES

as universal language limitations.

Implementation resource budgets MAY exist.

They MUST be explicit and separate.

The distinction is:

language capability
        ≠
compiler resource policy

---

72. Rust Safety

The implementation MUST compile under:

Rust 1.97
Rust 1.97.1
Edition 2021

and MUST use safe Rust.

Production implementation MUST NOT contain:

unsafe
unsafe fn
unsafe impl
unsafe trait
unsafe { ... }

for source-map functionality.

Safe alternatives MUST be used for:

- source slicing;
- Unicode handling;
- shared source ownership;
- synchronization;
- indexing;
- serialization;
- caching.

---

73. Integer Conversion Safety

The existing implementation converts some source lengths and indexes into "u32".

Production code MUST ensure that conversions are checked.

It MUST NOT silently truncate:

usize → u32
u64 → u32

when the value cannot be represented.

If the current representation cannot represent a source position, the compiler MUST produce an explicit resource/representation diagnostic or widen the implementation representation.

It MUST NOT silently wrap.

---

74. Span Arithmetic Safety

Operations such as:

end - start
start + offset
line_start + length

MUST be checked.

Arithmetic overflow MUST NOT silently produce an invalid source span.

The implementation MUST use safe checked/saturating logic as appropriate to the operation's semantics.

---

75. Source Map Construction

Conceptually:

SourceMap::new()
    ↓
empty map
    ↓
add source
    ↓
assign source identity
    ↓
build source metadata
    ↓
register source

The operation MUST be deterministic.

The source-map implementation SHOULD reject or explicitly handle duplicate source registration according to the compilation model rather than silently replacing an existing source.

---

76. Duplicate Source Registration

Two source registrations with the same display path MUST NOT automatically be considered the same source.

For example:

module.zm

may represent two different source versions.

The source identity remains distinct unless the compilation system explicitly establishes identity equivalence.

---

77. Content Identity

Implementations MAY calculate content identities/hashes for:

- incremental compilation;
- caching;
- reproducibility;
- provenance;
- dependency tracking.

Such hashes are metadata.

They do not replace "FileId" or source identity within the source map.

---

78. Security

Source maps MUST treat source names and paths as untrusted data.

They MUST NOT assume:

- trusted path strings;
- trusted Unicode;
- trusted generated source names;
- trusted module names.

Diagnostic rendering SHOULD protect against terminal/control-sequence injection.

Source content MUST NOT be interpreted as executable markup merely because it appears in a diagnostic.

---

79. Privacy

Source-map infrastructure SHOULD avoid unnecessarily copying or persisting source contents.

When source retention is configurable, implementations SHOULD allow source contents to be released after the compilation phase in which they are required, provided no remaining diagnostic/debugging contract requires them.

Source locations may remain without retaining the entire source file if the consumer no longer requires source text.

---

80. Diagnostics Must Never Panic on User Input

Malformed source MUST NOT cause the source-map implementation to panic merely because a user supplied:

- invalid offsets;
- invalid Unicode;
- huge literals;
- enormous files;
- unusual line endings;
- malformed delimiters;
- empty files.

Internal invariant violations MAY be treated as compiler bugs, but user-controlled malformed source MUST be handled deterministically.

---

81. Existing "Span::merge" Contract

The existing implementation currently uses an assertion when attempting to merge spans from different files.

The architectural contract is:

«Cross-file spans MUST NOT be merged into one ordinary contiguous span.»

Production callers SHOULD therefore validate source identity before invoking a same-source merge.

Where a user-controlled path could trigger a cross-file merge, the compiler SHOULD prefer a recoverable diagnostic/provenance representation rather than allowing an avoidable process panic.

A future implementation MAY change "merge" to return:

Option<Span>

or:

Result<Span, SpanMergeError>

if repository-wide API compatibility permits.

Such a change MUST be coordinated with all callers before implementation.

---

82. Existing Line Information Contract

The existing "SourceFile::get_line_info" performs binary search over precomputed line starts.

This is compatible with the production architecture.

However, production conformance additionally requires:

- checked offset handling;
- Unicode correctness;
- deterministic newline semantics;
- representation-scalability review;
- correct EOF behavior;
- no silent overflow.

---

83. Existing "get_line" Contract

The current implementation uses:

self.content.lines().nth(...)

for line retrieval.

The production specification requires that line retrieval semantics match the canonical source model.

If "str::lines()" semantics differ from the language's canonical newline definition, the implementation MUST be changed.

The specification, not a convenience API, is authoritative.

---

84. Source-Map API Stability

The following existing concepts SHOULD remain stable:

FileId
BytePos
Span
SourceFile
SourceMap

No unnecessary renaming is required.

The implementation MAY add capabilities such as:

SourceOrigin
RelatedSpan
SourceVersion
SourceKind
SourceName
SourceHash

if required by later integration.

New structures MUST have clearly separated ownership.

---

85. Proposed Source Metadata Model

If expanded, a source file MAY conceptually contain:

SourceFile
├── identity
├── display_name
├── logical_name
├── optional_path
├── optional_uri
├── source_kind
├── immutable_content
├── content_identity
└── line_index

Possible "source_kind" values include:

File
Virtual
Generated
MacroExpansion
Embedded
Stdin
Repl
Foreign

These are implementation concepts and MUST NOT become source-language syntax.

---

86. Source-Origin Model

Generated constructs MAY use:

SourceOrigin
├── Direct(Span)
├── Generated(OriginSet)
├── MacroExpansion(...)
├── Foreign(...)
└── Synthetic

The exact Rust representation is implementation-defined.

The semantics are:

Direct
    = directly corresponds to source text

Generated
    = produced from source-derived information

MacroExpansion
    = produced through macro processing

Foreign
    = originated in an external representation

Synthetic
    = compiler-created without direct source origin

---

87. Source Map Does Not Become a Semantic IR

The source-map model MUST remain orthogonal to:

AST
semantic model
quantum::ir
classical IR
HDL IR
ZQN
HAL
runtime

Source provenance is metadata about computational artifacts.

It MUST NOT become another semantic representation of the program.

---

88. Tooling Integration

The source-map contract MUST support tooling such as:

formatter
language server
IDE
syntax highlighter
debugger
profiler
compiler diagnostics
static analyzer
documentation generator
test framework

Tooling MUST use source identities and spans rather than reimplementing independent location calculations.

---

89. LSP Integration

For LSP-compatible tooling, conversion MAY be required between:

Zamani byte positions

and:

LSP line/character positions

The conversion MUST explicitly account for:

- UTF-8;
- UTF-16;
- line endings;
- Unicode scalar boundaries.

The internal compiler source map MUST NOT be redesigned merely to match an external protocol's coordinate model.

---

90. Formatter Integration

A formatter MUST preserve source semantics.

If the formatter produces a transformed source file, it SHOULD create a new source version rather than mutating the old source map's source contents.

Formatter-generated diagnostics SHOULD refer to the appropriate source version.

---

91. Testing Contract

Source-map conformance MUST include:

grammar/tests/source-map/

or the repository's equivalent source-map test location.

Tests MUST cover:

- empty source;
- one-line source;
- multi-line source;
- final newline;
- no final newline;
- LF;
- CRLF;
- CR;
- Unicode;
- multi-byte UTF-8;
- combining marks;
- invalid UTF-8;
- EOF;
- empty spans;
- nested spans;
- span containment;
- same-file merge;
- cross-file merge;
- duplicate file names;
- virtual sources;
- generated sources;
- macro provenance;
- large sources;
- many source files;
- deterministic mapping;
- parallel access;
- checked integer conversion;
- resource exhaustion.

---

92. Negative Tests

Negative tests MUST include:

invalid FileId
offset beyond source
end before start
cross-file merge
invalid UTF-8
invalid UTF-8 boundary
overflowing position conversion
invalid line number
invalid column
source mutation after registration

The implementation MUST fail deterministically.

---

93. Boundary Tests

Boundary tests MUST include:

0-byte source
1-byte source
source ending exactly at EOF
empty final line
very long line
many lines
Unicode immediately before EOF
Unicode immediately after newline
largest supported position representation

Where the implementation has a representation boundary, that boundary MUST be tested explicitly.

---

94. Scalability Tests

Scalability tests MUST vary:

source size
line count
file count
token count
AST node count
diagnostic count
module count
generated-source count

Tests MUST verify that no artificial language limit has been introduced.

Resource exhaustion MUST be distinguished from invalid syntax.

---

95. Determinism Tests

Given identical:

source
source identities
language version
compiler configuration

repeated runs MUST produce equivalent:

source IDs where deterministic assignment is promised
spans
line mappings
diagnostic locations
provenance

Tests SHOULD repeat compilation under different safe parallel execution schedules where practical.

---

96. Compatibility Tests

Source-map compatibility tests MUST ensure that changes to:

lexer
parser
AST
semantic analyzer
IR
diagnostics

do not silently invalidate source-location contracts.

Compatibility tests MUST include older supported language versions where source-location behavior is version-sensitive.

---

97. Hard-Coding Audit

The source-map subsystem MUST be audited for artificial limits.

Forbidden as universal language limits:

MAX_FILES
MAX_SOURCE_SIZE
MAX_LINES
MAX_COLUMNS
MAX_SPANS
MAX_DIAGNOSTICS
MAX_MODULES
MAX_GENERATED_SOURCES

Implementation-specific budgets MAY exist only when:

- explicitly named;
- configurable where appropriate;
- documented;
- observable;
- tested;
- separate from language validity.

---

98. Performance Requirements

The source map SHOULD provide:

O(1)

or effectively constant-time source-ID lookup where practical.

Line lookup SHOULD be:

O(log n)

with "n" equal to the number of indexed lines, or use an equivalent scalable strategy.

Source registration SHOULD avoid unnecessary source duplication.

Span operations SHOULD remain inexpensive because spans occur throughout the compiler.

Performance optimizations MUST NOT change observable source-location semantics.

---

99. Memory Requirements

The implementation SHOULD avoid storing redundant copies of:

source text
line information
file names
spans
diagnostic strings

where safe sharing is possible.

The implementation MUST NOT sacrifice source correctness merely to minimize memory.

Memory optimization is subordinate to:

correctness
determinism
safety
provenance

---

100. Safe Rust Requirement

The complete source-map implementation MUST be implementable entirely in safe Rust.

Required baseline:

Rust 1.97
Rust 1.97.1
Edition 2021

No unsafe implementation is necessary or permitted.

This includes future support for:

large sources
parallel compilation
incremental compilation
Unicode
memory sharing
source serialization
IDE integration

---

101. Cross-Domain Integration Matrix

Domain| Source-map responsibility| Downstream owner
Classical| source provenance| classical semantic/IR pipeline
Quantum| source provenance| "quantum::ir"
Hybrid| source provenance across boundaries| hybrid semantic pipeline
HDL| source provenance| HDL/hardware IR
Hardware| diagnostics for intent| hardware/resource analysis
Distributed| source provenance for distributed constructs| distributed compiler/runtime
AI| model/program provenance| AI semantic/IR pipeline
Data| schema/query provenance| data subsystem
Networking| source provenance| networking subsystem
Security| source provenance for security constructs| security semantic layer
Memory| source provenance| memory/type/semantic layers
Concurrency| source provenance| concurrency scheduler/compiler
Macros| origin + expansion provenance| macro subsystem
Metaprogramming| generated provenance| metaprogramming subsystem
Interoperability| Zamani origin + foreign location| interop subsystem

---

102. Integration With "grammar/spec/lexical.md"

"grammar/spec/lexical.md" owns lexical behavior.

This source-map specification owns the location model consumed by lexical analysis.

The relationship is:

lexical.md
    ↓
tokenization rules
    ↓
source-map.md
    ↓
token locations

The lexical specification MUST NOT independently redefine:

FileId
BytePos
Span

It references this specification instead.

---

103. Integration With "grammar/spec/syntax.md"

The syntax specification defines grammar structure.

This source-map specification defines where syntax structures originate.

Therefore:

syntax rule
    ↓
parser node
    ↓
span

Every normative syntax production SHOULD identify the source-location behavior expected for the resulting syntax construct.

---

104. Integration With "grammar/spec/diagnostics.md"

Diagnostics consume source-map information.

The diagnostic specification owns:

severity
codes
message structure
notes
help
rendering

This specification owns:

source identity
span
related source location
provenance

Neither specification should duplicate the other's responsibilities.

---

105. Integration With AST

Every source-bearing AST node MUST have a predetermined source-location contract before the grammar feature is considered production complete.

Feature completion therefore requires:

grammar
↓
token spans
↓
AST node span
↓
semantic provenance
↓
IR provenance
↓
diagnostic behavior

This satisfies the project's independently-completable-file requirement.

---

106. Integration With Canonical IR

Every new source-level feature that reaches IR MUST specify:

source span
source provenance
generated provenance
transformation provenance

before implementation is considered complete.

The source map MUST NOT require later re-editing merely because an IR backend was added.

---

107. Integration With Compiler Optimization

Every optimization pass that creates or combines IR operations MUST define its provenance policy.

At minimum:

unchanged operation
    → preserve span

one-to-one transformation
    → preserve origin

many-to-one transformation
    → aggregate origins

one-to-many transformation
    → propagate origin

synthetic operation
    → mark synthetic + retain cause when available

---

108. Integration With Runtime

Runtime execution MAY retain source locations for:

debugging
profiling
tracing
error reporting
observability

Runtime source mapping is optional unless required by a runtime feature.

Runtime MUST NOT require the entire compiler source map merely to execute ordinary programs.

---

109. Integration With POCO-REAF

The same source program MUST retain source provenance when compiled for different targets.

For example:

Zamani source
    ↓
CPU compilation

and:

Zamani source
    ↓
GPU compilation

must preserve the same source-level meaning and source locations.

Likewise:

Zamani quantum program
    ↓
QPU A

and:

Zamani quantum program
    ↓
QPU B

must not require different source-location semantics.

---

110. Integration With Resource Negotiation

A resource negotiation failure SHOULD point to the source construct responsible for the requirement.

Example:

requires capability("quantum.mid_circuit_measurement")

If the selected target lacks that capability, the compiler SHOULD report:

primary:
    the `requires` construct

related:
    target capability information

The source map itself does not determine whether the target possesses the capability.

---

111. Integration With Scheduling and Routing

Scheduling/routing MAY transform source operations.

They SHOULD preserve provenance through those transformations.

For example:

logical operation
    ↓
routed operation sequence

must remain traceable to the original source operation.

The source-map layer does not perform routing or scheduling.

---

112. Integration With QEC and ZQN

QEC and ZQN may generate additional operations or diagnostics.

Generated operations SHOULD retain provenance to the source quantum construct that caused them when meaningful.

The source map MUST NOT own:

QEC policy
noise semantics
fault semantics
physical qubit mapping

Those remain downstream responsibilities.

---

113. Integration With HAL

HAL may report:

unsupported capability
device unavailable
resource unavailable
calibration unavailable

When such conditions correspond to source-level requirements, the diagnostic system SHOULD retain the source location.

HAL MUST NOT modify source-map semantics.

---

114. Integration With Tests and Fixtures

Every source fixture SHOULD have deterministic source naming.

Tests MUST NOT rely on machine-specific absolute paths.

Preferred:

tests/quantum/basic.zm

rather than:

/home/user/project/tests/quantum/basic.zm

Diagnostics SHOULD normalize or abstract environment-specific paths when snapshot testing.

---

115. Integration With Generated Documentation

Generated documentation MAY link language constructs back to specification sections.

It MUST NOT invent source spans.

Documentation examples SHOULD be treated as separate source units when parsed.

---

116. Feature Completion Contract

A source-map-dependent language feature is complete only when all of the following are predetermined:

✓ source syntax
✓ lexical source spans
✓ parser spans
✓ AST span
✓ semantic provenance
✓ IR provenance
✓ diagnostic primary span
✓ related spans where necessary
✓ generated-source behavior
✓ macro behavior where relevant
✓ Unicode behavior
✓ EOF behavior
✓ malformed-input behavior
✓ scalability behavior
✓ deterministic behavior
✓ compatibility behavior
✓ safe Rust implementation
✓ tests

No downstream file should need to redefine the source-location semantics.

---

117. Required Repository Changes

This specification itself should be independent.

The following existing files integrate with it:

grammar/spec/lexical.md
grammar/spec/syntax.md
grammar/spec/diagnostics.md
grammar/spec/semantics.md
grammar/DESIGN.md
grammar/README.md
grammar/grammar.md
grammar/Zamani-Grammar.md
src/source_map.rs
src/lexer.rs
src/parser.rs
src/ast/

No existing filename needs to be renamed.

Potential implementation work in "src/source_map.rs" MUST be treated as a separate implementation task after this contract is accepted.

The specification MUST be completed before that implementation work begins.

---

118. Independent Completion Boundary

This file is independently complete when:

source identity defined
        +
byte position defined
        +
span defined
        +
line mapping defined
        +
column semantics defined
        +
Unicode behavior defined
        +
multi-file behavior defined
        +
generated provenance defined
        +
macro provenance defined
        +
AST integration defined
        +
lexer integration defined
        +
parser integration defined
        +
IR integration defined
        +
diagnostic integration defined
        +
scalability defined
        +
determinism defined
        +
Rust safety defined
        +
tests defined
        +
compatibility defined

No later grammar feature should need to redefine these concepts.

---

119. Completion Criteria

"grammar/spec/source-map.md" is production-ready when:

- source identity is unambiguous;
- byte offsets are authoritative;
- spans use a defined half-open interval;
- source boundaries are validated;
- EOF is defined;
- Unicode behavior is defined;
- line endings are deterministic;
- columns are explicitly defined;
- multi-file compilation is supported;
- virtual sources are supported;
- generated sources have provenance;
- macro expansion has provenance;
- AST integration is defined;
- parser integration is defined;
- lexer integration is defined;
- semantic provenance is defined;
- IR provenance is defined;
- quantum provenance reaches "quantum::ir";
- hardware realization remains downstream;
- QEC remains downstream;
- ZQN remains downstream;
- routing remains downstream;
- scheduling remains downstream;
- HAL remains downstream;
- POCO-REAF remains intact;
- no artificial hardware limits exist;
- no artificial source-language scalability limits exist;
- resource budgets are distinguished from language validity;
- safe Rust is mandatory;
- Rust 1.97 / 1.97.1 is supported;
- deterministic behavior is specified;
- cross-platform behavior is specified;
- diagnostics are source-aware;
- incremental compilation semantics are defined;
- security considerations are defined;
- test requirements are defined;
- compatibility requirements are defined.

---

120. Final Source-Location Architecture

The production Zamani source-location architecture is:

                         Zamani Source
                              │
                              ▼
                         Source Bytes
                              │
                              ▼
                         Source Map
                              │
                ┌─────────────┼─────────────┐
                ▼             ▼             ▼
             FileId        BytePos       SourceFile
                │             │             │
                └─────────────┼─────────────┘
                              ▼
                            Span
                              │
             ┌────────────────┼────────────────┐
             ▼                ▼                ▼
           Lexer            Parser             AST
             │                │                │
             └────────────────┼────────────────┘
                              ▼
                     Semantic Analysis
                              │
                              ▼
                    Canonical Semantic Model
                              │
             ┌────────────────┼─────────────────┐
             ▼                ▼                 ▼
        Classical         quantum::ir       HDL/Hardware
             │                │                 │
             └────────────────┼─────────────────┘
                              ▼
                         Canonical IR
                              │
                              ▼
                         Optimization
                              │
             ┌────────────────┼────────────────┐
             ▼                ▼                ▼
          Routing         Scheduling       Resilience
             │                │                │
             └────────────────┼────────────────┘
                              ▼
                             ZQN
                              │
                              ▼
                             HAL
                              │
                              ▼
                       Target Realization
                              │
             ┌────────────────┼─────────────────┐
             ▼                ▼                 ▼
            CPU              GPU               QPU
             │                │                 │
             └────────────────┼─────────────────┘
                              │
                              ▼
                    Diagnostics / Debugging /
                    Profiling / Tooling
                              │
                              ▼
                       Original Source Span

The invariant is:

«Every compiler artifact should retain source provenance whenever a meaningful source origin exists, but source provenance must never become a substitute for semantic ownership.»

This keeps "grammar/spec/source-map.md" independent, allows "src/source_map.rs" to evolve safely, preserves the existing filenames, integrates with the current lexer/parser/AST, and prevents source-location machinery from becoming another hidden scalability or target-specific constraint.

Hard-coding audit: no universal machine, quantum, hardware, file-count, source-size, node-count, CPU, GPU, FPGA, QPU, memory, topology, tensor, timeline, or accelerator limit is defined by this specification.

Safety audit: the implementation contract requires safe Rust only; no Rust "unsafe" is required.

POCO-REAF audit: source locations identify source provenance, never physical realization, so compilation for a different machine or hardware configuration does not require changing the source-location model.