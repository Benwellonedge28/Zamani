Zamani Frontend Conformance

Path: "grammar/compatibility/frontend-conformance.md"
Status: Normative
Scope: Lexing, parsing, parse-tree construction, frontend AST construction, source spans, diagnostics, frontend semantic hand-off, frontend compatibility, determinism, scalability, and integration with the rest of the Zamani compiler pipeline
Language: Zamani
Grammar technology: ANTLR4
Rust edition: 2021
Supported Rust implementation baseline: Rust 1.97 / Rust 1.97.1
Rust safety requirement: Production Rust implementation MUST NOT use "unsafe"
Primary portability objective: "Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)"

---

1. Purpose

This document defines the production conformance contract for the Zamani frontend.

The frontend is the boundary that transforms:

Zamani source
    ↓
lexical tokens
    ↓
parse tree
    ↓
frontend AST
    ↓
semantic analysis

The frontend MUST preserve the meaning expressed by the source program while remaining independent of:

- CPU model;
- GPU model;
- FPGA model;
- ASIC implementation;
- QPU model;
- accelerator model;
- machine size;
- cluster size;
- memory capacity;
- physical qubit count;
- register width;
- network size;
- number of nodes;
- number of timelines;
- vendor-specific hardware;
- runtime scheduling decisions;
- physical quantum mapping;
- QEC implementation;
- ZQN implementation;
- HAL implementation.

The frontend is therefore a portable language boundary, not a hardware realization layer.

This document exists to ensure that a feature cannot be declared frontend-complete merely because a parser rule has been added.

A production frontend feature is complete only when:

source
  ↓
lexer
  ↓
parser
  ↓
AST
  ↓
source spans
  ↓
diagnostics
  ↓
semantic hand-off
  ↓
compatibility contract
  ↓
tests

is completely defined.

---

2. Authority

This document is the authoritative contract for frontend conformance.

It does not replace the other compatibility documents.

Concern| Authority
Overall grammar architecture| "grammar/DESIGN.md"
Grammar navigation and authority map| "grammar/README.md"
Normative language specification| "grammar/specification/"
Formal language contracts| "grammar/spec/"
Canonical ANTLR composition root| "grammar/Zamani.g4"
Lexical contracts| "grammar/lexer/"
Implementation/conformance reference| "grammar/grammar.md"
Historical/proposed grammar material| "grammar/Zamani-Grammar.md"
Cross-layer compatibility relationships| "grammar/compatibility/compatibility-matrix.md"
Language/release version policy| "grammar/compatibility/versions.md"
Migration transformations| "grammar/compatibility/migrations.md"
Deprecation lifecycle| "grammar/compatibility/deprecated.md"
Feature-gate policy| "grammar/compatibility/feature-gates.md"
Dialect compatibility| "grammar/compatibility/dialects.md"
Reserved namespace/syntax| "grammar/compatibility/reserved-space.md"
Frontend conformance| This document
AST conformance after frontend construction| "grammar/compatibility/ast-conformance.md"
IR conformance| "grammar/compatibility/ir-conformance.md"
Compiler conformance| "grammar/compatibility/compiler-conformance.md"
Runtime conformance| "grammar/compatibility/runtime-conformance.md"
Target conformance| "grammar/compatibility/target-conformance.md"
Tooling conformance| "grammar/compatibility/tooling-conformance.md"

No document listed above may silently assume ownership belonging to another document.

---

3. Frontend Definition

For this document:

«Frontend means the implementation boundary responsible for converting valid Zamani source text into a stable, source-located, domain-neutral frontend representation suitable for semantic analysis.»

The frontend includes:

1. source acquisition;
2. Unicode handling;
3. lexical analysis;
4. token classification;
5. token source locations;
6. parsing;
7. parse-tree construction;
8. syntax diagnostics;
9. parse-tree validation where required;
10. frontend AST construction;
11. AST source spans;
12. frontend attributes/modifiers;
13. syntactic feature/version validation;
14. frontend feature-gate recognition;
15. semantic-analysis hand-off.

The frontend does not own:

- type checking;
- ownership checking;
- resource feasibility;
- capability discovery;
- physical hardware selection;
- quantum routing;
- scheduling;
- QEC;
- ZQN;
- calibration;
- target deployment;
- runtime execution.

---

4. Frontend Pipeline

The production frontend MUST implement the following conceptual pipeline:

source text
    │
    ▼
source normalization
    │
    ▼
lexer
    │
    ▼
canonical token stream
    │
    ▼
parser
    │
    ▼
parse tree
    │
    ▼
parse validation
    │
    ▼
frontend AST construction
    │
    ▼
source-span attachment
    │
    ▼
frontend diagnostics
    │
    ▼
semantic-analysis input

The frontend MUST NOT skip the AST boundary by directly converting arbitrary parser structures into backend-specific IR.

---

5. Canonical Compiler Relationship

The repository-wide relationship is:

Zamani source
      ↓
lexer
      ↓
parser
      ↓
frontend AST
      ↓
semantic analysis
      ↓
canonical semantic representation
      ↓
canonical IR
      ↓
optimization
      ↓
routing / scheduling / resilience
      ↓
QEC / ZQN where applicable
      ↓
HAL
      ↓
target realization
      ↓
runtime

The frontend therefore has no authority to decide how a computation is physically realized.

---

6. Root Grammar Relationship

"grammar/Zamani.g4" is the canonical ANTLR composition root.

The frontend MUST treat it as the canonical parser entry point.

"Zamani.g4" MUST NOT become a second semantic specification.

The root grammar should compose the language from reusable grammar contracts rather than becoming an uncontrolled monolithic list of domain-specific constructs.

Conceptually:

Zamani.g4
    │
    ├── core
    ├── declarations
    ├── statements
    ├── expressions
    ├── types
    ├── functions
    ├── modules
    ├── effects
    ├── memory
    ├── concurrency
    ├── classical
    ├── quantum
    ├── hybrid
    ├── hdl
    ├── hardware
    ├── resources
    ├── distributed
    ├── ai
    ├── data
    ├── networking
    ├── security
    ├── interoperability
    ├── dialects
    ├── macros
    └── metaprogramming

The composition mechanism MUST preserve one language rather than creating separate languages for each domain.

---

7. Specification-to-Frontend Traceability

Every stable language feature MUST be traceable through:

specification
    ↓
grammar contract
    ↓
lexer contract
    ↓
parser rule
    ↓
parse-tree rule
    ↓
AST node
    ↓
semantic contract
    ↓
IR contract
    ↓
implementation
    ↓
tests

A feature that cannot be traced through this chain MUST NOT be marked fully implemented.

---

8. Frontend Feature Completion Contract

Every frontend feature MUST define, before implementation:

Feature
Purpose
Status
Specification owner
Grammar owner
Lexer tokens
Parser rules
Parse-tree shape
AST node
AST fields
Source-span rules
Attributes
Modifiers
Feature gates
Version requirements
Diagnostics
Semantic hand-off
IR hand-off
Downstream consumers
Positive tests
Negative tests
Boundary tests
Scalability tests
Determinism tests
Compatibility tests
Migration requirements
Deprecation requirements
Hard-coding audit
Security considerations
Performance considerations
Completion criteria

This is mandatory for production features.

---

9. Frontend Ownership

The frontend owns:

- syntactic validity;
- lexical validity;
- structural syntax;
- source locations;
- parse-tree structure;
- frontend AST construction;
- syntax-level feature/version recognition;
- syntax diagnostics;
- deterministic source-to-AST transformation;
- preservation of source information needed downstream.

The frontend does not own:

- physical resource availability;
- hardware selection;
- quantum error-correction strategy;
- device calibration;
- physical qubit allocation;
- GPU selection;
- FPGA routing;
- CPU scheduling;
- distributed placement;
- runtime recovery;
- target-specific optimization.

---

10. Lexer Contract

The lexer MUST conform to the contracts under:

grammar/lexer/

The lexer MUST provide a canonical token stream.

Each token MUST have, where applicable:

- token kind;
- source spelling;
- normalized representation if required;
- source span;
- line;
- column;
- source-file identity;
- channel/category;
- lexical diagnostics.

Token identity MUST be canonical.

The implementation MUST NOT silently create duplicate semantic token concepts merely because different grammar files use different names.

---

11. Token Identity

Known token areas requiring an explicit repository-wide audit include:

Question
QuestionMark

and:

Ampersand
BitAnd

If two names represent the same lexical concept, one canonical identity MUST be selected.

If they represent genuinely different concepts, their distinction MUST be documented in:

grammar/lexer/tokens.md
grammar/lexer/operators.md

and covered by tests.

Aliases are permitted only when their ownership and compatibility behavior are explicit.

---

12. Keywords

The frontend MUST use a centralized keyword policy.

Adding a keyword MUST evaluate:

1. existing identifiers;
2. contextual-keyword alternatives;
3. dialect conflicts;
4. feature-gate conflicts;
5. version compatibility;
6. migration impact;
7. diagnostics;
8. tooling impact.

A previously valid identifier MUST NOT become unusable without an explicit compatibility decision.

---

13. Unicode

Zamani frontend processing MUST define Unicode behavior explicitly.

Unicode rules MUST cover:

- identifier normalization;
- allowed identifier characters;
- source encoding;
- invalid sequences;
- normalization policy;
- diagnostics;
- source-span calculation;
- Unicode punctuation;
- mathematical symbols;
- quantum notation.

The implementation MUST avoid treating byte offsets as character offsets when the public source model requires character-aware locations.

---

14. Numeric Literals

Numeric literals MUST not impose artificial machine-size limits.

The frontend may parse:

- integers;
- arbitrary-precision literals where supported;
- floating-point literals;
- scientific notation;
- hexadecimal;
- binary;
- octal;
- typed literals;
- dimension/shape literals;
- resource quantities where specified.

A numeric literal's size is a property of the source program.

It MUST NOT become a compiler-wide maximum merely because a particular target cannot represent it.

Target feasibility belongs downstream.

---

15. Quantum Literal Compatibility

The frontend MUST support the quantum literal forms defined by the authoritative language specification.

Where supported, examples include:

|0⟩
|1⟩
|+⟩
|-⟩
|ψ⟩

The frontend MUST preserve these as syntactic representations.

It MUST NOT determine:

- physical qubit allocation;
- physical topology;
- device calibration;
- noise model;
- QEC strategy;
- pulse implementation.

Those decisions occur downstream.

---

16. Parsing Determinism

For a fixed:

source
+
language version
+
dialect set
+
feature-gate configuration

the frontend MUST produce a deterministic parse result.

Equivalent implementations MUST NOT randomly choose between incompatible parses.

Ambiguous grammar constructs MUST be:

- eliminated;
- explicitly disambiguated;
- resolved by a normative precedence rule;
- or rejected.

---

17. Parser Recovery

Error recovery MUST NOT silently transform invalid source into a valid program with different semantics.

The parser may recover sufficiently to report multiple diagnostics, but recovery MUST preserve the distinction between:

valid source

and:

source containing syntax errors

Recovered parse trees MUST NOT be passed to semantic analysis as if they were validated programs unless the semantic layer explicitly supports error-tolerant analysis.

---

18. Parse Tree Versus AST

The parse tree is an implementation artifact.

The frontend AST is the stable structural boundary.

The relationship is:

ANTLR parse tree
        ↓
AST builder
        ↓
frontend AST

The parse tree MUST NOT become the long-term semantic representation.

ANTLR-specific implementation details MUST NOT leak into the stable semantic model unless explicitly required.

---

19. Frontend AST Principles

The frontend AST MUST be:

- domain-aware where source syntax genuinely requires domain distinctions;
- otherwise domain-neutral;
- source-located;
- structurally complete;
- deterministic;
- semantically unambiguous;
- independent of physical hardware;
- independent of backend selection.

The AST MUST represent what the programmer wrote.

It MUST NOT prematurely represent what a particular backend decided to do.

---

20. AST Source Spans

Every user-authored AST construct that can generate a diagnostic MUST preserve a source span.

A span should provide enough information to identify:

- source file;
- start position;
- end position;
- line/column information where required;
- originating syntax where needed.

Source spans MUST survive:

lexer
→ parser
→ AST
→ semantic diagnostics
→ compatibility diagnostics

as required by downstream contracts.

---

21. AST Evolution

Adding an optional AST field SHOULD be non-breaking where possible.

Changing the meaning of an existing AST field is breaking.

Removing an AST field used by downstream semantic analysis is breaking.

Renaming an AST node or field requires an explicit compatibility decision.

AST changes MUST be coordinated with:

grammar/compatibility/ast-conformance.md

This document owns the frontend consequences; "ast-conformance.md" owns the detailed AST contract.

---

22. Domain-Neutral Frontend Boundary

The frontend MUST NOT prematurely bind source constructs to physical implementation.

For example:

Qubit[n]

may represent a quantum resource declaration.

It MUST NOT become:

PhysicalQubitArray<Device42>

in the frontend.

Likewise:

Tensor<T, shape>

must not become:

GPUBuffer<SpecificGpu>

during parsing.

And:

Memory<T, size>

must not become:

RamBank<SpecificMachine>

during parsing.

---

23. Quantum Frontend Contract

Quantum syntax MUST use a data-driven operation model.

The frontend MUST NOT require a permanent grammar enumeration such as:

H
X
Y
Z
CNOT
...

as the complete universe of quantum operations.

The preferred structural model is equivalent to:

quantumOperation
    : operationSpecifier quantumTargetList
    ;

The frontend must therefore be capable of structurally representing forms such as:

apply H to q
apply custom_gate to q
apply vendor.operation to q
apply operation(parameter) to q0, q1

without requiring a new parser rule for every future operation.

---

24. Quantum Operation AST

A quantum operation AST representation SHOULD preserve, as applicable:

name
namespace
operands
parameters
results
attributes
modifiers
effects
capabilities
source span

The frontend does not decide whether the operation is:

- native;
- decomposed;
- synthesized;
- routed;
- error-corrected;
- pulse-level;
- simulator-only;
- vendor-specific.

Those decisions belong downstream.

---

25. Canonical Quantum Boundary

The frontend MUST integrate with the existing canonical:

quantum::ir

boundary.

The repository MUST NOT create a second competing frontend quantum IR merely to represent parser output.

The required path is:

quantum source
    ↓
frontend AST
    ↓
semantic analysis
    ↓
quantum::ir
    ↓
optimization
    ↓
decomposition
    ↓
routing
    ↓
scheduling
    ↓
resilience/QEC/ZQN
    ↓
HAL
    ↓
target

---

26. QEC Boundary

QEC MUST remain downstream of frontend parsing.

The frontend may represent:

error_correction(...)
fault_tolerance(...)
noise_budget(...)
reliability(...)

where those constructs are part of the language.

The frontend MUST NOT implement:

- code-distance calculation;
- syndrome decoding;
- physical error correction;
- logical-to-physical mapping;
- device-specific QEC;
- QEC scheduling.

Those belong to the appropriate quantum/QEC subsystem.

Existing QEC resource policy contracts, including the canonical treatment of "QecLimits" where applicable in the implementation, MUST not be duplicated in frontend syntax.

---

27. ZQN Boundary

ZQN semantics MUST remain downstream.

The frontend may preserve source intent concerning:

- resilience;
- noise;
- fault tolerance;
- reliability;
- recovery.

The frontend MUST NOT calculate physical fault/noise behavior.

---

28. HDL Frontend Contract

HDL syntax MUST represent hardware intent.

The frontend may parse:

module
port
signal
net
register
memory
clock
reset
pipeline
interface
protocol
timing
assertion
generate
parameter

where specified.

It MUST NOT assume:

32-bit
64-bit
64 GB
24 GB
fixed number of registers
fixed number of ports
fixed number of devices
fixed number of pipeline stages

unless the value is explicitly supplied by the programmer as program data or a target-specific constraint.

---

29. Hardware Intent

Hardware constructs MUST remain distinct from physical target realization.

The frontend may parse:

requires capability("gpu.compute")
requires capability("quantum.measurement")
requires capability("tensor.compute")
requires topology(...)
requires memory >= required_memory

but MUST NOT resolve those requirements to a particular physical device during parsing.

---

30. Resource and Capability Syntax

The frontend MUST preserve the distinction between:

Requirement

Something the program needs.

Capability

Something the target must provide.

Constraint

Something the realization must obey.

Preference

Something the compiler should prefer if feasible.

Hint

An optimization suggestion without semantic necessity.

Realization

A downstream physical implementation decision.

The frontend MUST NOT collapse these categories.

---

31. Classical Computing

Classical constructs MUST share the same frontend foundations as all other Zamani domains.

The frontend MUST support the language's defined:

- scalar values;
- integers;
- floating-point values;
- vectors;
- matrices;
- tensors;
- symbolic expressions;
- collections;
- functions;
- control flow;
- concurrency;
- memory;
- effects.

Mathematical library functions MUST NOT automatically become grammar keywords merely because they exist in a mathematical library.

---

32. Hybrid Computing

The frontend MUST support composition of:

classical computation
        ↓
quantum operation
        ↓
measurement
        ↓
classical computation
        ↓
quantum operation

without creating a separate hybrid language.

The same AST and semantic infrastructure must support the boundary.

---

33. Distributed Computing

Distributed constructs MUST remain scalable.

The frontend MUST NOT encode universal limits on:

- nodes;
- processes;
- actors;
- channels;
- messages;
- services;
- replicas;
- partitions;
- topology size.

A source program may express a resource requirement or deployment constraint.

The actual number of available nodes is a downstream resource concern.

---

34. AI and Data Computing

AI/data syntax MUST integrate with shared frontend infrastructure.

The frontend may represent:

- models;
- tensors;
- datasets;
- training;
- inference;
- differentiable computation;
- agents;
- symbolic computation;
- probabilistic computation;
- pipelines.

It MUST NOT make a particular framework, accelerator API, or vendor runtime part of the core syntax unless explicitly defined as an interoperability/dialect feature.

---

35. Networking

Networking constructs MUST remain abstract where the language specification allows.

The frontend may represent:

- endpoints;
- protocols;
- services;
- channels;
- requests;
- responses;
- streams;
- routing intent.

It MUST NOT silently convert portable source constructs into physical network interfaces during parsing.

---

36. Security

The frontend MUST preserve security-related source intent without executing it.

Parsing MUST NOT:

- execute external commands;
- read arbitrary files;
- access secrets;
- perform network requests;
- invoke hardware;
- mutate external state.

The parser parses.

Semantic analysis validates.

The compiler/runtime execute only after the appropriate security and capability checks.

---

37. FFI and Interoperability

Interoperability syntax MUST be explicitly represented.

Examples include:

- foreign functions;
- foreign types;
- ABI declarations;
- calling conventions;
- C/C++;
- Python;
- Rust;
- WebAssembly;
- OpenQASM;
- QIR;
- HDL formats.

The frontend MUST preserve enough structure for the interoperability layer to validate the boundary.

It MUST NOT make an external format the canonical Zamani semantic model.

---

38. OpenQASM and External Quantum Formats

OpenQASM is an interoperability format.

It MUST NOT replace:

quantum::ir

as the canonical Zamani quantum semantic boundary.

The frontend/import layer may translate:

OpenQASM
    ↓
Zamani representation
    ↓
semantic analysis
    ↓
quantum::ir

and export may proceed in the opposite direction where supported.

---

39. Dialects

Dialect syntax MUST be explicitly declared.

A dialect MUST identify, at minimum:

name
version
syntax additions
syntax restrictions
semantic additions
AST mapping
feature gates
compatibility policy
migration policy

A dialect MUST NOT silently change the meaning of the core language.

Dialect handling integrates with:

grammar/compatibility/dialects.md
grammar/compatibility/feature-gates.md
grammar/compatibility/versions.md

---

40. Feature Gates

Frontend feature gates control syntax availability.

They MUST NOT be confused with implementation status.

A feature can be:

implemented but gated

or:

implemented and stable

or:

specified but not implemented

The frontend MUST reject gated syntax deterministically when the required gate is unavailable.

It MUST produce a diagnostic that identifies:

- feature;
- required gate;
- language version;
- source span;
- supported alternatives where appropriate.

---

41. Language Versions

The frontend MUST identify the language version against which source is being parsed.

It MUST NOT infer language semantics from the Rust compiler version.

The Zamani language version is independent of:

Rust 1.97
Rust 1.97.1
compiler release
runtime release
target firmware
hardware generation

Unsupported language versions MUST be rejected explicitly rather than silently interpreted as another version.

---

42. Deprecation

Deprecated syntax MUST remain governed by:

grammar/compatibility/deprecated.md

The frontend's responsibility is to:

- recognize the deprecated construct when the compatibility policy requires recognition;
- emit the required diagnostic;
- preserve semantics during the supported deprecation period;
- provide migration information where defined.

The frontend MUST NOT invent its own deprecation lifecycle.

---

43. Migration

Migration transformations belong to:

grammar/compatibility/migrations.md

Frontend conformance requires that deprecated syntax can be mapped to its migration contract where applicable.

A migration MUST NOT silently change program semantics.

---

44. Reserved Space

Reserved syntax and names belong to:

grammar/compatibility/reserved-space.md

The frontend MUST enforce reserved-space rules.

Future-reserved syntax MUST NOT accidentally become valid stable syntax without a compatibility decision.

---

45. Diagnostics

Frontend diagnostics MUST distinguish at least:

lexical error
syntax error
unsupported language version
disabled feature
unsupported dialect
reserved construct
invalid literal
invalid identifier
invalid operator
invalid declaration
invalid expression
invalid type syntax
invalid module syntax

Frontend diagnostics MUST NOT report downstream failures as syntax failures.

For example:

target lacks required capability

is not a parser error.

Likewise:

insufficient physical qubits

is not necessarily a source syntax error.

---

46. Diagnostic Source Locations

Diagnostics MUST identify the smallest useful source span available.

Diagnostics SHOULD preserve:

- source file;
- line;
- column;
- range;
- offending token;
- expected syntax where applicable;
- feature/version context;
- actionable correction where appropriate.

Diagnostic wording MAY evolve without changing semantics, but diagnostic compatibility must remain governed by the tooling and compatibility contracts.

---

47. Error Categories

The frontend MUST preserve the following conceptual boundary:

LEXICAL
    ↓
SYNTACTIC
    ↓
STRUCTURAL/AST
    ↓
SEMANTIC
    ↓
RESOURCE/CAPABILITY
    ↓
IR
    ↓
COMPILER
    ↓
TARGET
    ↓
RUNTIME

A failure MUST be reported at the earliest correct layer without misclassifying a downstream condition as a frontend error.

---

48. Source Preservation

Where required by tooling and diagnostics, the frontend MUST preserve:

- comments;
- attributes;
- source spans;
- documentation comments;
- formatting-sensitive metadata;
- macro-origin information;
- expansion-origin information.

Whether comments remain in the stable AST is governed by the AST contract.

The frontend MUST NOT discard information that a promised downstream feature requires.

---

49. Macros

Macro syntax MUST be parsed before macro expansion.

Macro expansion MUST NOT bypass:

- syntax validation;
- source tracking;
- semantic validation;
- capability checks;
- security policy.

Expanded constructs MUST remain traceable to their source origin where diagnostics require it.

Macro expansion MUST be deterministic where the language contract requires deterministic expansion.

---

50. Metaprogramming

Metaprogramming syntax MUST have explicit phase boundaries.

The frontend MUST distinguish:

source-time syntax
compile-time computation
generated syntax
runtime computation

Metaprogramming MUST NOT become an unrestricted mechanism for bypassing semantic or security validation.

---

51. Memory and Ownership

The frontend may parse syntax representing:

- ownership;
- borrowing;
- references;
- allocation;
- regions;
- shared memory;
- distributed memory;
- accelerator memory;
- quantum memory.

The frontend MUST preserve structure for semantic analysis.

It MUST NOT decide actual memory placement.

---

52. Concurrency

The frontend may parse:

- async;
- await;
- spawn;
- tasks;
- actors;
- channels;
- synchronization;
- parallelism;
- pipelines;
- data parallelism;
- task parallelism.

It MUST NOT impose universal limits on:

threads
tasks
actors
channels
workers
cores
devices

Program-level constants are permitted.

Compiler-wide artificial ceilings are not.

---

53. Scalability Contract

The frontend MUST scale according to available implementation resources.

"Unbounded" means:

«The language model does not encode an artificial fixed maximum; practical limits arise only from representational constraints, implementation resources, operating environment, target capabilities, and explicitly declared program requirements.»

The frontend MUST therefore avoid universal constants such as:

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
MAX_TIMELINES

These are prohibited as universal language limits.

---

54. Large Source Structures

The parser MUST support arbitrarily large valid structures subject to available resources.

Examples include:

- large modules;
- large declaration sets;
- large expressions;
- large arrays;
- large tensor shapes;
- large quantum operation lists;
- large HDL descriptions;
- large distributed deployment descriptions.

The frontend MUST not reject a program merely because it exceeds an arbitrary language-defined count.

---

55. Tiny-to-Large Compatibility

The same syntax and AST contracts MUST work for:

single value
single operation
single qubit
single processing element
single device

through increasingly large programs and systems.

The frontend MUST NOT have a separate language for "small" and "large" systems.

Scaling is a property of program data, resource requirements, and backend realization.

---

56. Deterministic AST Construction

For identical:

source
language version
dialect configuration
feature gates

the frontend MUST construct an equivalent AST.

The AST MUST NOT depend on:

- machine size;
- CPU count;
- GPU count;
- thread scheduling;
- available QPU count;
- hardware topology;
- network state.

---

57. Frontend Resource Independence

The frontend MUST parse resource requirements without checking actual target resources.

For example:

requires qubits >= n

may be syntactically and semantically valid even when the machine currently has fewer qubits.

The frontend's responsibility is to preserve the requirement.

Resource feasibility is determined later.

---

58. Target Independence

The same source MUST be frontend-compatible regardless of whether it is eventually intended for:

CPU
GPU
FPGA
ASIC
QPU
quantum simulator
accelerator
HPC system
distributed cluster
cloud system
edge device
future target

provided the source syntax itself does not explicitly require a target-specific dialect.

---

59. Target-Specific Syntax

Target-specific syntax MUST be visibly and explicitly target-specific.

It should integrate through:

dialects
interoperability
hardware intent
resource/capability declarations

The core frontend MUST NOT silently reinterpret portable source based on the current machine.

---

60. POCO-REAF Frontend Contract

The frontend contribution to:

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever

is semantic preservation.

The frontend MUST guarantee that changing the eventual target does not change how valid portable source is lexed, parsed, or structurally represented.

For example:

same source
    ↓
same language version
    ↓
same frontend contract

must remain stable when the backend target changes from:

CPU
→ GPU
→ FPGA
→ QPU
→ distributed
→ future target

Target-specific realization happens after frontend processing.

---

61. "Compile Once" Clarification

The frontend MUST NOT claim that one binary artifact can execute on every conceivable future machine.

POCO-REAF at the language architecture level means:

one source program
        ↓
one defined semantic meaning
        ↓
portable intermediate representation/artifact policy
        ↓
target-specific realization where required

Future targets must satisfy the relevant language version, IR, runtime, capability, and interoperability contracts.

This distinction prevents the language from making an impossible universal binary-compatibility promise.

---

62. "Forever" Compatibility

"Forever" is an architectural objective, not an unconditional promise that every future implementation will preserve every historical construct without migration.

Long-term compatibility is achieved through:

- explicit versions;
- stable contracts;
- deprecation;
- migrations;
- compatibility modes where supported;
- reserved space;
- dialect versioning;
- IR versioning;
- deterministic semantic definitions.

These mechanisms remain owned by the existing compatibility files.

---

63. Frontend Compatibility Categories

A frontend change MUST be classified as one or more of:

Category| Meaning
Lexically compatible| Existing tokenization remains equivalent
Syntactically compatible| Existing valid source still parses equivalently
AST compatible| Existing source maps to equivalent AST
Source-span compatible| Diagnostic/source locations remain valid
Semantically compatible| Meaning is preserved
Feature compatible| Feature-gate behavior remains compatible
Version compatible| Declared language-version behavior remains compatible
Dialect compatible| Dialect contract remains compatible
Tooling compatible| Supported tooling remains compatible
Intentionally incompatible| Break is explicit and versioned

A feature MUST NOT be labeled simply "compatible" when only one dimension was tested.

---

64. Compatibility Direction

Frontend compatibility has multiple directions.

Backward source compatibility

New implementation accepts previously valid stable source.

Forward compatibility

Implementation recognizes newer syntax only when explicitly supported by versioning or extension mechanisms.

AST compatibility

Frontend output remains consumable by the declared downstream semantic contract.

Tooling compatibility

Language servers, formatters, diagnostics, analyzers, and generators understand the same frontend contract.

The frontend MUST NOT pretend to support future syntax merely because a parser recovery mechanism accepts it.

---

65. Unknown Syntax

Unknown syntax MUST NOT be silently accepted as valid core-language syntax.

Depending on the language contract, unknown syntax may be:

- rejected;
- recognized as an explicitly declared dialect;
- recognized as a future-reserved construct;
- recognized through an extension mechanism.

It MUST NOT silently acquire arbitrary semantics.

---

66. Grammar and Implementation Drift

The following must remain synchronized:

grammar/specification/
grammar/spec/
grammar/Zamani.g4
grammar/lexer/
grammar/grammar.md
src/lexer.rs
src/parser.rs
src/frontend/ast/

If these disagree, the repository MUST classify the discrepancy.

Examples:

SPECIFICATION_ONLY
GRAMMAR_ONLY
IMPLEMENTATION_ONLY
PARTIALLY_IMPLEMENTED
DEPRECATED
INTENTIONALLY_UNSUPPORTED

No implementation-only feature becomes stable merely because code exists.

---

67. "grammar.md"

"grammar/grammar.md" is an implementation/conformance reference.

The frontend MUST contribute enough information for it to distinguish:

SPECIFIED
IMPLEMENTED
PARTIALLY IMPLEMENTED
PLANNED
DEPRECATED

The frontend conformance document does not replace "grammar.md".

Instead:

grammar.md
    ↓
implementation status

frontend-conformance.md
    ↓
frontend contract

---

68. "Zamani-Grammar.md"

"grammar/Zamani-Grammar.md" contains broader historical/design material.

Its presence MUST NOT cause the frontend to accept every construct described there.

A construct becomes canonical only after:

proposal
 ↓
semantic design
 ↓
AST contract
 ↓
grammar contract
 ↓
implementation
 ↓
IR contract
 ↓
tests
 ↓
compatibility review
 ↓
stable status

---

69. AST Integration

After frontend AST construction, the next authority is:

grammar/compatibility/ast-conformance.md

The frontend MUST hand off:

- structurally valid AST;
- complete required fields;
- source spans;
- attributes;
- modifiers;
- syntactic version information where required;
- macro origin where required.

Semantic analysis then owns meaning.

---

70. Semantic Integration

The frontend MUST provide sufficient information for semantic analysis to perform:

- name resolution;
- type checking;
- effect checking;
- ownership checking;
- resource validation;
- capability validation;
- domain-specific semantic validation.

The frontend MUST NOT perform semantic checks merely because they are convenient to implement in parsing.

---

71. Resource Integration

Frontend syntax may describe:

requires qubits >= n
requires memory >= required_memory
requires capability("tensor.compute")
requires capability("gpu.compute")
requires topology(...)

The frontend preserves these expressions.

Resource analysis determines:

Can the selected realization satisfy them?

The parser does not.

---

72. Capability Integration

Capability identifiers MUST be data-driven where practical.

The grammar MUST NOT require a new parser production for every future hardware capability.

For example:

capability("quantum.measurement")
capability("gpu.compute")
capability("tensor.compute")

should be structurally representable without changing the universal grammar for every new capability.

---

73. Hardware Integration

Hardware descriptions MUST pass through:

frontend AST
    ↓
semantic hardware/resource model
    ↓
hardware/IR contracts
    ↓
compiler realization

The frontend MUST NOT instantiate actual hardware objects.

---

74. Runtime Integration

The frontend does not execute programs.

Its output must be consumable by semantic analysis and ultimately by:

compiler
runtime
target

The frontend MUST NOT assume runtime scheduling decisions.

---

75. Safety and Rust "unsafe"

The production Rust implementation MUST use safe Rust.

The repository SHOULD enforce this with:

#![forbid(unsafe_code)]

at applicable crate boundaries, together with CI auditing.

The frontend MUST NOT require:

- unsafe lexer implementations;
- unsafe parser implementations;
- unsafe AST construction;
- unsafe memory manipulation;
- unsafe FFI merely for normal language processing.

The existence of a Zamani-language source file named:

statements/unsafe.g4

does not authorize Rust "unsafe".

If the language itself intends to prohibit an "unsafe" construct, that policy must be established in the language specification and handled through the normal deprecation/removal process rather than being silently changed by this document.

---

76. No-Unsafe Validation

CI MUST distinguish Rust implementation code from textual examples.

The no-unsafe audit SHOULD identify:

unsafe
unsafe fn
unsafe impl
unsafe trait
unsafe {

and classify each occurrence.

Allowed occurrences may include:

- documentation describing the prohibited Rust construct;
- negative tests;
- language syntax examples;
- compatibility documentation.

Production Rust implementation occurrences MUST be rejected unless an explicitly documented repository exception exists.

The production profile requested by this document has no Rust-implementation unsafe exception.

---

77. Performance Conformance

Frontend implementation MUST scale with source size subject to available resources.

Validation SHOULD include:

- small files;
- deeply nested constructs;
- large modules;
- large token streams;
- large declaration sets;
- large expressions;
- large quantum programs;
- large HDL descriptions;
- large distributed descriptions;
- large generated programs.

Performance optimizations MUST preserve:

- token identity;
- parse result;
- AST meaning;
- source spans;
- diagnostics.

---

78. Memory Conformance

The frontend MUST NOT impose artificial universal memory ceilings.

If the implementation cannot process a source file because the host lacks sufficient memory, that is an implementation/resource limitation, not a language-level maximum.

The implementation SHOULD fail deterministically and diagnostically rather than corrupting or silently truncating source.

---

79. Deep Nesting

Deeply nested valid syntax MUST be tested.

Where parser technology imposes implementation-specific recursion constraints, the limitation MUST be:

- documented;
- tested;
- treated as an implementation limitation;
- not promoted into a universal language rule unless formally specified.

Where practical, iterative or bounded-resource-safe implementation techniques SHOULD be preferred.

---

80. Frontend Test Taxonomy

Every frontend feature MUST have:

positive tests
negative tests
boundary tests
scalability tests
determinism tests
compatibility tests
diagnostic tests
source-span tests

Where applicable it should also have:

dialect tests
feature-gate tests
migration tests
deprecation tests
macro tests
interoperability tests

---

81. Positive Tests

Positive tests MUST demonstrate that valid source is accepted.

They should cover:

- minimal form;
- normal form;
- generic form;
- parameterized form;
- nested form;
- cross-domain form;
- representative large form.

---

82. Negative Tests

Negative tests MUST demonstrate rejection of invalid source.

They should cover:

- malformed tokens;
- malformed syntax;
- missing delimiters;
- invalid operators;
- invalid literals;
- invalid declarations;
- invalid expressions;
- invalid feature-gate usage;
- invalid dialect usage;
- unsupported language versions.

---

83. Boundary Tests

Boundary tests MUST target semantic edges of syntax rather than artificial machine limits.

Examples:

empty program
single item
single element
nested constructs
zero-length lists where permitted
large representable literal
deep nesting
maximum representable implementation index
Unicode boundary
source-file boundary

Tests MUST NOT establish artificial universal hardware maxima.

---

84. Scalability Tests

Scalability tests MUST verify that the same language constructs remain valid as program data grows.

Examples:

1 operation
many operations

1 qubit
many qubits

1 tensor dimension
many dimensions

1 module
many modules

1 node
many nodes

1 timeline
many timelines

The test suite MUST not define a final universal maximum merely because a particular test fixture stops at a certain size.

---

85. Determinism Tests

For identical input:

source
version
dialect configuration
feature gates

the frontend MUST produce equivalent:

tokens
parse structure
AST
diagnostic classification

where deterministic behavior is promised.

Tests SHOULD compare stable structural representations rather than memory addresses or incidental implementation details.

---

86. Source Round-Trip Tests

Where formatter/parser round-tripping is supported, tests SHOULD verify:

source
 ↓
parse
 ↓
AST
 ↓
format
 ↓
parse
 ↓
equivalent AST

Formatting changes MUST NOT alter semantic structure.

---

87. Grammar Round-Trip Tests

For supported grammar serialization or generated syntax representations:

grammar
 ↓
generated parser
 ↓
source
 ↓
parser

must remain consistent with the canonical specification.

Generated artifacts MUST NOT become independent authorities.

---

88. Feature-Level Completion

A frontend feature is complete only when:

- specification exists;
- lexical contract exists;
- grammar exists;
- parser integration exists;
- AST mapping exists;
- source spans exist;
- diagnostics exist;
- semantic hand-off exists;
- compatibility classification exists;
- positive tests exist;
- negative tests exist;
- boundary tests exist;
- scalability tests exist;
- determinism tests exist;
- hard-coding audit passes;
- Rust safety audit passes;
- downstream consumers are documented.

---

89. Repository-Wide Frontend Matrix

The following minimum matrix MUST be maintained conceptually:

Layer| Required evidence
Specification| Feature is defined
Lexer| Required tokens exist
Grammar| Syntax is defined
Parser| Syntax is accepted/rejected correctly
Parse tree| Structural representation is stable
AST| AST mapping exists
Source spans| Locations are preserved
Diagnostics| Failures are correctly classified
Semantic hand-off| Required information is preserved
Compatibility| Version/gate/dialect impact defined
Tests| Required test classes exist
Safety| No Rust unsafe implementation
Scalability| No artificial universal limit
Integration| Downstream consumer is defined

---

90. Cross-Domain Frontend Matrix

Domain| Frontend responsibility| Must not own
Classical| Parse classical constructs| Target CPU selection
Quantum| Parse quantum intent| Physical qubit mapping
HDL| Parse hardware intent| FPGA/ASIC realization
Hybrid| Parse cross-domain composition| Backend scheduling
AI| Parse model/data/training syntax| Framework execution
Data| Parse data structures/pipelines| Storage engine realization
Distributed| Parse distributed intent| Actual node allocation
Networking| Parse communication intent| Physical routing
Security| Parse security policy syntax| Secret execution
Memory| Parse memory semantics| Physical memory placement
Concurrency| Parse concurrency constructs| Actual thread placement
Hardware| Parse capability/resource intent| Device selection
Resources| Parse requirements/constraints| Feasibility decision
Interoperability| Parse foreign boundaries| Foreign runtime execution
Dialects| Parse declared extensions| Silent language forks
Macros| Parse macro syntax| Semantic bypass
Metaprogramming| Parse staged constructs| Unrestricted execution

---

91. Frontend Compatibility With AST Conformance

"frontend-conformance.md" answers:

«Did the frontend correctly produce the structural representation?»

"ast-conformance.md" answers:

«Does that AST conform to the repository's AST contract?»

Therefore:

frontend-conformance
        ↓
ast-conformance
        ↓
semantic analysis

must remain a strict boundary.

A frontend change that alters an AST node MUST trigger AST-conformance review.

---

92. Frontend Compatibility With IR Conformance

The frontend does not directly own IR.

The required chain is:

frontend AST
    ↓
semantic analysis
    ↓
canonical semantic representation
    ↓
IR

A frontend feature MUST identify its eventual IR mapping before being considered complete, but the frontend MUST NOT define IR ownership.

---

93. Frontend Compatibility With Compiler Conformance

Compiler conformance begins after semantic validation and IR construction.

A frontend change is compiler-relevant when it changes:

- source meaning;
- AST structure;
- semantic inputs;
- resource requirements;
- capability requirements;
- effect behavior.

The compiler conformance document owns backend implementation compatibility.

---

94. Frontend Compatibility With Runtime Conformance

Runtime behavior MUST be derived from the semantic/IR contract.

The frontend MUST NOT assume a runtime implementation.

A source construct requiring runtime support MUST declare the semantic/runtime dependency before being marked complete.

---

95. Frontend Compatibility With Target Conformance

Target feasibility belongs downstream.

A valid source program may be:

frontend-valid
semantic-valid
IR-valid

while being:

target-infeasible

because the target lacks a required capability.

That does not make the source grammar invalid.

---

96. Frontend Compatibility With Tooling

Tooling that consumes frontend structures must respect:

- token contracts;
- AST contracts;
- source spans;
- diagnostics;
- versions;
- dialects;
- feature gates.

Examples include:

- language servers;
- formatters;
- syntax highlighters;
- linters;
- documentation generators;
- code analyzers;
- IDE integrations.

Tooling MUST NOT silently invent syntax.

---

97. Compatibility Failure Classification

Frontend failures MUST be classified as one or more of:

LEXICAL_DRIFT
GRAMMAR_DRIFT
PARSER_DRIFT
AST_DRIFT
SOURCE_SPAN_DRIFT
DIAGNOSTIC_DRIFT
VERSION_DRIFT
FEATURE_GATE_DRIFT
DIALECT_DRIFT
SEMANTIC_HANDOFF_DRIFT
SCALABILITY_VIOLATION
HARD_CODING_VIOLATION
SAFETY_VIOLATION
DOCUMENTATION_DRIFT

Each classification MUST identify the owning layer.

---

98. Hard-Coding Audit

Every frontend implementation change MUST be reviewed for accidental fixed limits.

Audit for patterns representing universal limits, including:

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
MAX_TIMELINES

Also audit for hard-coded physical identities such as:

QUBIT_0
QUBIT_1
QUBIT_2

when used as universal language assumptions.

A fixed value is allowed when it is genuinely:

- a program literal;
- a protocol-defined constant;
- a representation invariant;
- a test fixture;
- a target profile;
- a documented implementation/resource policy.

It is prohibited when presented as a universal language capacity.

---

99. No Artificial Scaling Boundary

The frontend MUST NOT contain logic equivalent to:

if qubit_count > fixed_limit:
    reject

unless that limit is a formally specified representation or implementation constraint and is not presented as the language's universal capacity.

The preferred model is:

program requirement
        ↓
semantic representation
        ↓
resource analysis
        ↓
target capability matching

---

100. Hardware Availability

The frontend MUST not inspect actual hardware to decide whether source is syntactically valid.

This would violate separation of concerns.

Correct:

parse source
    ↓
validate syntax
    ↓
build AST
    ↓
semantic analysis
    ↓
discover target
    ↓
check requirements

Incorrect:

parse source
    ↓
inspect current machine
    ↓
change grammar based on hardware

---

101. Vendor Independence

Vendor-specific constructs MUST be isolated behind:

- dialects;
- interoperability;
- target capabilities;
- explicit target-specific declarations.

The core frontend MUST remain vendor-neutral.

A vendor adding a new accelerator MUST NOT require changing the semantics of unrelated Zamani programs.

---

102. Future Computing Models

The frontend architecture MUST support future computing domains without requiring a redesign of the universal language foundation.

Future domains should integrate through:

shared lexer
 ↓
shared syntax
 ↓
shared AST
 ↓
semantic extension
 ↓
domain IR
 ↓
compiler/backend

A future target or domain MUST NOT require rewriting:

- identifiers;
- basic expressions;
- declarations;
- modules;
- source spans;
- diagnostics;
- core types;

unless the language specification itself changes.

---

103. Example Portable Intent

The frontend should be capable of structurally representing intent such as:

requires capability("tensor.compute")
requires capability("quantum.measurement")
requires memory >= required_memory
requires qubits >= n
requires topology(...)

The frontend does not determine whether the request is satisfiable.

It preserves the program's intent.

---

104. Example Quantum Portability

The following conceptual source forms must remain structurally portable:

apply H to q
apply custom_gate to q
apply vendor.operation to q
apply operation(parameter) to q0, q1

The frontend should not need a new universal grammar production every time a new quantum operation appears.

---

105. Example Hardware Portability

The language may describe:

requires capability("gpu.compute")

without binding the source program to:

GPU 0

The latter is a realization concern unless explicitly part of a target-specific dialect.

---

106. Example Resource Portability

The language may describe:

requires qubits >= n

without declaring:

MAX_QUBITS = 128

The first scales with "n".

The second creates an artificial language limit.

---

107. Frontend Security Boundary

The frontend MUST be side-effect free with respect to external systems, except for explicitly controlled compiler infrastructure needed to obtain source input.

Parsing MUST NOT:

- execute source;
- invoke arbitrary commands;
- access secrets;
- mutate hardware;
- contact remote systems;
- allocate external resources based on untrusted syntax without appropriate controls.

---

108. FFI Safety

FFI declarations may be parsed safely without invoking foreign code.

The frontend MUST treat FFI as declarative metadata.

Actual ABI validation and invocation remain downstream responsibilities.

The Rust implementation itself MUST not require "unsafe" for ordinary frontend operation.

---

109. Reproducibility

Given identical:

source
language version
feature configuration
dialect configuration
frontend implementation version

the frontend MUST produce reproducible structural results.

Where reproducible builds are promised, the frontend MUST avoid:

- timestamps;
- random identifiers;
- host-dependent ordering;
- hardware-dependent parsing;
- nondeterministic traversal.

---

110. Incremental Compilation

If incremental parsing or compilation is implemented, incremental processing MUST preserve the same language semantics as full parsing.

For a source program "S":

full_parse(S)

and:

incremental_parse(S)

must produce equivalent frontend structures where the feature is supported.

---

111. Generated Parser Integration

If ANTLR generates parser code:

grammar/Zamani.g4

and its composed grammar sources remain authoritative.

Generated parser output is not an independent specification.

The generated output MUST be reproducible from the declared grammar inputs and tool version.

---

112. Generated File Policy

Generated files MUST identify:

generator
source grammar
generation version
generation procedure

Generated artifacts MUST NOT be manually edited to introduce language behavior.

Changes belong in the authoritative source.

---

113. Rust Toolchain

The implementation baseline is:

Rust 1.97
or
Rust 1.97.1
edition 2021

The repository MUST use a valid Rust toolchain declaration.

Language version and Rust version MUST remain separate concepts.

The compiler MUST NOT expose Rust implementation details as Zamani language semantics.

---

114. Required CI Validation

The repository's frontend production gate SHOULD include:

cargo fmt --check
cargo check
cargo test
cargo clippy

plus the repository's grammar/ANTLR validation commands.

The exact commands MUST follow the repository's actual build tooling rather than being invented by this document.

CI MUST also validate:

grammar → lexer → parser → AST

and, where available:

AST → semantic → IR → compiler → runtime

---

115. No-Unsafe CI Gate

CI MUST reject new production Rust unsafe code.

The gate should distinguish:

Rust implementation

from:

documentation
tests
Zamani-language source
compatibility examples

The final production Rust implementation MUST contain no unsafe implementation requirement.

---

116. Compatibility Test Matrix

At minimum, frontend CI should test:

Test class| Required
Lexical acceptance| Yes
Lexical rejection| Yes
Syntax acceptance| Yes
Syntax rejection| Yes
AST construction| Yes
Source spans| Yes
Diagnostics| Yes
Determinism| Yes
Feature gates| Yes
Language versions| Yes
Dialects| Yes
Deprecations| Yes
Migrations| Where applicable
Quantum syntax| Yes
Classical syntax| Yes
HDL syntax| Yes
Hybrid syntax| Yes
Resource/capability syntax| Yes
Scalability| Yes
Hard-coding audit| Yes
Safe-Rust audit| Yes

---

117. Cross-Version Frontend Tests

For every stable language version, CI SHOULD maintain representative programs proving:

old valid source
    ↓
new frontend
    ↓
same intended structure

when backward compatibility is promised.

Breaking changes MUST have explicit version tests.

---

118. Negative Compatibility Tests

CI MUST verify that unsupported constructs are rejected rather than silently reinterpreted.

Examples:

unsupported language version
unsupported dialect
disabled feature
removed syntax
reserved syntax
invalid operator
invalid literal
unknown target-specific syntax in core mode

---

119. Frontend Conformance Checklist

A frontend implementation is production-conformant only when all applicable items below pass.

Specification

- [ ] Feature has a normative specification.
- [ ] Feature status is known.
- [ ] Version requirements are known.
- [ ] Feature-gate requirements are known.
- [ ] Dialect requirements are known.

Lexer

- [ ] Tokens are defined.
- [ ] Token identity is canonical.
- [ ] Keyword conflicts are resolved.
- [ ] Literals are defined.
- [ ] Unicode behavior is defined.
- [ ] Source spans are preserved.

Parser

- [ ] Grammar rule exists.
- [ ] Rule is reachable.
- [ ] Ambiguity is resolved.
- [ ] Precedence is defined.
- [ ] Associativity is defined.
- [ ] Recovery behavior is defined.
- [ ] Determinism is tested.

AST

- [ ] AST node exists.
- [ ] Fields are defined.
- [ ] Optionality is defined.
- [ ] Source span is preserved.
- [ ] Attributes are preserved.
- [ ] Modifiers are preserved.
- [ ] AST compatibility is documented.

Semantic hand-off

- [ ] Required semantic information is preserved.
- [ ] Resource requirements are preserved.
- [ ] Capability requirements are preserved.
- [ ] Effects are preserved.
- [ ] Domain information is preserved.
- [ ] No target-specific realization leaks into the frontend.

Compatibility

- [ ] Compatibility dimension is identified.
- [ ] Version impact is identified.
- [ ] Migration impact is identified.
- [ ] Deprecation impact is identified.
- [ ] Dialect impact is identified.
- [ ] Tooling impact is identified.

Scalability

- [ ] No artificial universal hardware limit.
- [ ] No fixed qubit limit.
- [ ] No fixed CPU limit.
- [ ] No fixed GPU limit.
- [ ] No fixed FPGA limit.
- [ ] No fixed node limit.
- [ ] No fixed memory limit.
- [ ] No fixed thread limit.
- [ ] No fixed tensor-rank limit.
- [ ] No fixed register-width limit.
- [ ] No fixed network-size limit.

Testing

- [ ] Positive tests.
- [ ] Negative tests.
- [ ] Boundary tests.
- [ ] Scalability tests.
- [ ] Determinism tests.
- [ ] Diagnostic tests.
- [ ] Source-span tests.
- [ ] Compatibility tests.

Safety

- [ ] Production Rust contains no unsafe implementation.
- [ ] Toolchain baseline is valid.
- [ ] CI checks the safety requirement.

---

120. Definition of Frontend Production Readiness

The Zamani frontend is production-ready only when:

1. the canonical specification is identified;
2. "Zamani.g4" has one clear composition role;
3. lexer ownership is unambiguous;
4. parser ownership is unambiguous;
5. AST ownership is unambiguous;
6. source spans are preserved;
7. diagnostics are deterministic and correctly classified;
8. every stable grammar feature maps to an AST;
9. every AST feature has a semantic hand-off;
10. quantum syntax reaches the canonical "quantum::ir" path;
11. classical, quantum, HDL, AI, distributed and other domains share common frontend foundations;
12. hardware realization does not leak into parsing;
13. resource requirements are not confused with physical resources;
14. capability requirements are not confused with implementation decisions;
15. dialects are explicitly versioned;
16. feature gates are explicitly defined;
17. migrations are governed by "migrations.md";
18. deprecations are governed by "deprecated.md";
19. version policy remains governed by "versions.md";
20. no artificial universal hardware limits exist;
21. frontend behavior is deterministic;
22. scalability is limited only by actual implementation/resource constraints rather than arbitrary language constants;
23. positive, negative, boundary, scalability and compatibility tests exist;
24. source locations remain correct;
25. generated parser artifacts are reproducible;
26. Rust 1.97/1.97.1 compatibility is maintained;
27. production Rust uses no "unsafe";
28. downstream semantic, IR, compiler, runtime and target contracts are defined;
29. existing filenames are retained unless a rename is genuinely required;
30. documentation agrees with implementation.

---

121. Completion Rule for Individual Files

No frontend file is considered complete merely because its immediate syntax works.

For every file, completion means:

Purpose
  ↓
Owns
  ↓
Does Not Own
  ↓
Inputs
  ↓
Outputs
  ↓
Dependencies
  ↓
Upstream contracts
  ↓
Downstream consumers
  ↓
Syntax
  ↓
AST
  ↓
Semantics
  ↓
IR hand-off
  ↓
Diagnostics
  ↓
Compatibility
  ↓
Scalability
  ↓
Security
  ↓
Tests
  ↓
Hard-coding audit
  ↓
Safe-Rust audit
  ↓
Completion criteria

The file MUST contain enough information to be integrated without discovering fundamental missing contracts only after another file has been completed.

---

122. Change Procedure

A frontend change MUST follow:

1. Identify specification owner.
2. Identify compatibility impact.
3. Define tokens.
4. Define grammar.
5. Define parse-tree structure.
6. Define AST mapping.
7. Define source spans.
8. Define diagnostics.
9. Define feature/version/dialect gates.
10. Define semantic hand-off.
11. Define IR destination.
12. Identify compiler/runtime consumers.
13. Add positive tests.
14. Add negative tests.
15. Add boundary tests.
16. Add scalability tests.
17. Add determinism tests.
18. Add compatibility tests.
19. Run hard-coding audit.
20. Run no-unsafe audit.
21. Run repository integration tests.
22. Update derived documentation.

The order is intentional.

A developer should not discover the AST or IR contract after writing the parser.

---

123. Prohibited Change Pattern

The following is prohibited:

add parser rule
    ↓
make up AST later
    ↓
make up semantic meaning later
    ↓
discover IR later
    ↓
discover hardware requirements later
    ↓
rewrite frontend

The required pattern is:

feature contract
    ↓
syntax contract
    ↓
AST contract
    ↓
semantic contract
    ↓
IR contract
    ↓
implementation
    ↓
tests

---

124. Integration With Existing Compatibility Matrix

"compatibility/compatibility-matrix.md" remains the repository-wide cross-layer relationship authority.

This document provides the frontend-specific detail required by that matrix.

The relationship is:

compatibility-matrix.md
        │
        ├── frontend-conformance.md
        ├── ast-conformance.md
        ├── ir-conformance.md
        ├── compiler-conformance.md
        ├── runtime-conformance.md
        ├── target-conformance.md
        └── tooling-conformance.md

The matrix answers:

«Are the layers compatible?»

This document answers:

«Does the frontend itself conform to its contract?»

---

125. Integration With Existing AST Conformance

"ast-conformance.md" owns the complete AST contract.

This document owns the frontend obligation to produce that AST correctly.

Therefore:

frontend parser
      ↓
frontend AST
      ↓
AST conformance

Any AST change MUST be reflected in the appropriate compatibility review.

---

126. Integration With Existing IR Conformance

"ir-conformance.md" owns IR compatibility.

This document only identifies the required semantic hand-off.

No frontend grammar file may invent an alternative IR merely because it is convenient.

The canonical quantum path remains:

frontend AST
    ↓
semantic analysis
    ↓
quantum::ir

---

127. Integration With Compiler, Runtime and Target Conformance

The frontend provides source-level meaning.

The later documents determine whether:

AST
→ semantic representation
→ IR
→ compiled artifact
→ runtime
→ target

remain compatible.

A target limitation MUST NOT be back-propagated into the core grammar as an artificial syntax limitation.

---

128. Long-Term Architectural Invariant

The following invariant MUST remain true:

«Zamani source describes computation, semantics, requirements, constraints, capabilities, effects, and portable intent. The frontend preserves that information without encoding accidental limits of a particular implementation or machine.»

This invariant is the frontend's contribution to POCO-REAF.

---

129. Final Frontend Architecture

                         ZAMANI SOURCE
                              │
                              ▼
                    specification contract
                              │
                              ▼
                     grammar/Zamani.g4
                              │
                              ▼
                           LEXER
                              │
                              ▼
                         TOKEN STREAM
                              │
                              ▼
                           PARSER
                              │
                              ▼
                         PARSE TREE
                              │
                              ▼
                      FRONTEND AST
                              │
             ┌────────────────┼────────────────┐
             │                │                │
             ▼                ▼                ▼
        source spans     attributes       modifiers
             │                │                │
             └────────────────┼────────────────┘
                              ▼
                    SEMANTIC HAND-OFF
                              │
             ┌────────────────┼────────────────┐
             │                │                │
             ▼                ▼                ▼
        classical        quantum intent      HDL intent
             │                │                │
             └────────────────┼────────────────┘
                              ▼
                    RESOURCE/CAPABILITY
                         VALIDATION
                              │
                              ▼
                    CANONICAL SEMANTICS
                              │
                              ▼
                         CANONICAL IR
                              │
             ┌────────────────┼────────────────┐
             │                │                │
             ▼                ▼                ▼
        optimization       routing        scheduling
                              │
                              ▼
                       resilience/QEC
                              │
                              ▼
                             ZQN
                              │
                              ▼
                             HAL
                              │
                              ▼
                       TARGET REALIZATION
                              │
             ┌────────────────┼────────────────┐
             │                │                │
             ▼                ▼                ▼
            CPU              GPU              FPGA
             │                │                │
             └────────────────┼────────────────┘
                              │
             ┌────────────────┼────────────────┐
             ▼                ▼                ▼
            ASIC             QPU          future target
                              │
                              ▼
                           RUNTIME

---

130. Governing Principles

The production Zamani frontend MUST follow these rules:

1. One canonical language.
2. One canonical frontend architecture.
3. One canonical AST boundary.
4. One canonical quantum semantic boundary: "quantum::ir".
5. No competing root grammar.
6. No silent syntax authority in "Zamani-Grammar.md".
7. No competing specification in "grammar.md".
8. No hardware limits encoded as language limits.
9. No fixed universal qubit/core/GPU/FPGA/node/thread/tensor/timeline limits.
10. No vendor implementation embedded in the portable core.
11. No QEC implementation in the parser.
12. No routing implementation in the parser.
13. No scheduling implementation in the parser.
14. No calibration implementation in the parser.
15. No HAL implementation in the parser.
16. No semantic bypass through macros.
17. No semantic bypass through metaprogramming.
18. No silent dialect forks.
19. No silent version reinterpretation.
20. No Rust "unsafe" in production implementation.
21. Every stable syntax construct has a predetermined AST contract.
22. Every AST construct has a predetermined semantic hand-off.
23. Every feature identifies its downstream consumers before implementation is considered complete.
24. Every feature has positive, negative, boundary and scalability tests.
25. Compatibility is multidimensional and must be explicitly classified.
26. Target feasibility is downstream from language validity.
27. Resource requirements are not physical assignments.
28. Capability requirements are not vendor bindings.
29. Program constants are not compiler capacity limits.
30. Practical limits come from available resources and declared requirements, not artificial language ceilings.
31. Existing filenames are preserved unless a genuine architectural reason requires a migration.
32. The frontend remains stable while target realization evolves.
33. The language remains extensible without redesigning its universal foundations.
34. POCO-REAF is achieved through semantic portability and explicit compatibility contracts rather than an impossible promise of one immutable binary for every future machine.

---

131. Final Production Criterion

"grammar/compatibility/frontend-conformance.md" is satisfied when every frontend feature can be answered completely before implementation:

What syntax does it own?
What tokens does it consume?
What parse-tree structure represents it?
What AST node represents it?
What fields does that AST node contain?
What source span does it preserve?
What diagnostics can it produce?
What language versions support it?
What feature gates affect it?
What dialects affect it?
What semantic information does it provide?
What resource requirements does it preserve?
What capabilities does it preserve?
What IR eventually represents it?
Which compiler components consume that representation?
Which runtime components consume it?
Which targets may realize it?
What compatibility guarantees exist?
What migrations are required?
What deprecations are required?
What positive tests prove it?
What negative tests prove it?
What boundary tests prove it?
What scalability tests prove it?
What determinism tests prove it?
What hard-coding audit proves scalability?
What safety audit proves no Rust unsafe?

If any required answer is missing, the feature is not frontend-production-ready.

The governing rule is:

«The Zamani frontend preserves portable program meaning; it does not encode the accidental limits of today's machines.»

Therefore the frontend must allow the same language foundation to scale from tiny programs and devices through classical machines, GPUs, FPGAs, ASICs, QPUs, simulators, accelerators, distributed systems, HPC, cloud systems, and future computing targets, subject to the resources, capabilities, semantic requirements, compatibility contracts, and realization mechanisms actually available at the time of execution.