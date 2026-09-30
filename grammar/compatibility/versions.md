Zamani Language Version and Compatibility Policy

Path: "grammar/compatibility/versions.md"
Status: Normative
Language: Zamani
Scope: Language versions, compatibility, source evolution, grammar evolution, lexer/parser compatibility, AST compatibility, semantic compatibility, IR compatibility, dialect compatibility, compiler compatibility, artifact compatibility, target compatibility, and POCO-REAF portability
Rust baseline: Rust 1.97 / Rust 1.97.1
Rust edition: 2021
Rust safety requirement: Production Rust implementation MUST use safe Rust; "unsafe" MUST NOT be required or used by the Zamani compiler implementation
Primary objective: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)

---

1. Purpose

This document defines the normative versioning and compatibility contract for Zamani.

It establishes how Zamani evolves while preserving, where promised:

- source compatibility;
- lexical compatibility;
- syntactic compatibility;
- AST compatibility;
- semantic compatibility;
- type compatibility;
- effect compatibility;
- capability and resource compatibility;
- module compatibility;
- dialect compatibility;
- canonical IR compatibility;
- quantum semantic compatibility;
- classical semantic compatibility;
- HDL and hardware semantic compatibility;
- artifact compatibility;
- ABI compatibility where explicitly defined;
- runtime compatibility where explicitly defined;
- target portability;
- deterministic compilation;
- reproducibility;
- long-term POCO-REAF portability.

This document applies to the complete language pipeline:

Zamani source
    ↓
language-version resolution
    ↓
lexical contract
    ↓
canonical grammar
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
optimization / lowering
    ↓
routing / scheduling / resilience / QEC / ZQN
    ↓
HAL / target realization
    ↓
runtime / execution

This document defines compatibility policy.

It does not define:

- backend algorithms;
- hardware topology;
- QEC algorithms;
- calibration;
- scheduling algorithms;
- routing algorithms;
- runtime implementation;
- physical-device limits;
- compiler optimization strategies.

Those remain owned by their respective repository subsystems.

---

2. Normative Language

The following terms are normative:

- MUST — mandatory.
- MUST NOT — prohibited.
- SHOULD — recommended unless a documented reason exists not to follow it.
- SHOULD NOT — discouraged unless a documented reason exists.
- MAY — permitted.
- REQUIRED — equivalent to MUST.
- OPTIONAL — permitted but not required.

A compatibility claim MUST be supported by the appropriate specification and conformance tests.

---

3. Authority and Ownership

Versioning must not create a second language specification.

The repository's authority relationship is:

grammar/DESIGN.md
        │
        ▼
canonical language specification
        │
        ├── grammar/specification/
        ├── grammar/spec/
        │
        ▼
canonical grammar
grammar/Zamani.g4
        │
        ▼
lexer / parser
        │
        ▼
frontend AST
        │
        ▼
semantic analysis
        │
        ▼
canonical semantic representation
        │
        ├── classical semantics
        ├── quantum semantics
        └── HDL/hardware semantics
        │
        ▼
canonical IR
        │
        ├── quantum::ir
        └── other canonical IR boundaries
        │
        ▼
optimization / lowering
        │
        ├── routing
        ├── scheduling
        ├── resilience
        ├── QEC
        └── ZQN
        │
        ▼
HAL / runtime / target realization

3.1 "grammar/DESIGN.md"

Owns:

- grammar architecture;
- ownership boundaries;
- grammar composition;
- architectural invariants;
- POCO-REAF architecture;
- scalability architecture.

It does not define individual release numbers.

3.2 "grammar/specification/"

Owns normative language specification.

In particular:

"grammar/specification/language-version.md"

owns the language-level meaning of language versions.

3.3 "grammar/spec/versioning.md"

Owns cross-layer versioning and implementation-conformance rules.

3.4 "grammar/spec/compatibility.md"

Owns compatibility dimensions and semantic compatibility interpretation.

3.5 "grammar/compatibility/versions.md"

This file owns:

- version-policy classification;
- release compatibility rules;
- language/compiler separation;
- compatibility guarantees;
- compatibility obligations;
- version-transition rules;
- version-related production acceptance criteria.

It does not replace the detailed language-version specification.

3.6 "grammar/compatibility/migrations.md"

Owns migration procedures.

3.7 "grammar/compatibility/deprecated.md"

Owns deprecation lifecycle and active deprecated features.

3.8 "grammar/compatibility/reserved.md"

Owns reserved syntax and identifiers.

3.9 "grammar/compatibility/compatibility-matrix.md"

Owns explicit compatibility relationships among supported versions and layers.

3.10 "grammar/grammar.md"

Owns implementation-conformance reporting.

It MUST NOT silently become the language authority.

3.11 "grammar/Zamani-Grammar.md"

Remains the broad/historical/extended design source.

It MAY contain proposed, experimental, historical, or future features.

Its contents do not become stable language merely because they appear there.

---

4. Compatibility Is Layered

Compatibility is not a single property.

The following layers MUST be evaluated independently:

Source
  ↓
Lexical
  ↓
Syntactic
  ↓
AST
  ↓
Name / Module
  ↓
Type
  ↓
Effect
  ↓
Capability / Resource
  ↓
Semantic
  ↓
IR
  ↓
Artifact
  ↓
ABI
  ↓
Runtime
  ↓
Target
  ↓
Execution

A program MAY therefore be:

- source-compatible but target-incompatible;
- syntax-compatible but semantically incompatible;
- semantic-compatible but resource-infeasible;
- AST-incompatible while source-compatible;
- IR-incompatible while source and semantic meaning remain compatible;
- runtime-incompatible while the source language remains unchanged.

These conditions MUST NOT be conflated.

---

5. Fundamental Compatibility Invariant

The fundamental rule is:

«A compatible Zamani implementation MUST NOT silently change the specified meaning of valid existing source code.»

If an intentionally breaking change is introduced, it MUST be:

1. explicitly classified;
2. assigned to an appropriate language version;
3. documented;
4. diagnosable;
5. testable;
6. migratable where practical;
7. reflected in compatibility metadata.

A compiler MUST NOT silently reinterpret an older program using incompatible newer semantics.

---

6. Language Versions

Zamani language versions use:

MAJOR.MINOR.PATCH

Examples:

1.0.0
1.1.0
1.1.1
2.0.0

Language version numbers describe the language contract.

They do not describe:

- Rust versions;
- compiler implementation versions;
- operating-system versions;
- CPU generations;
- GPU generations;
- FPGA generations;
- ASIC generations;
- QPU generations;
- hardware topology;
- runtime versions;
- device firmware versions.

---

7. Major Versions

A MAJOR version change is required when stable language meaning becomes intentionally incompatible.

Examples include:

- changing the meaning of existing valid syntax;
- removing stable syntax;
- changing stable type semantics incompatibly;
- changing stable ownership semantics incompatibly;
- changing stable effect semantics incompatibly;
- changing module resolution incompatibly;
- changing stable resource semantics incompatibly;
- changing stable quantum semantics incompatibly;
- changing stable evaluation semantics incompatibly.

A major version MUST NOT be incremented merely because:

- a new target exists;
- a new GPU exists;
- a new QPU exists;
- a backend is rewritten;
- compiler optimization improves;
- a new accelerator is supported;
- a new hardware topology exists.

Those are implementation/target concerns unless they change the language contract.

---

8. Minor Versions

A MINOR version MAY add backward-compatible language capability.

Examples:

- new non-conflicting syntax;
- new optional attributes;
- new extensible operation forms;
- new resource expressions;
- new capabilities;
- new domain constructs;
- new interoperable forms;
- new standard library-facing language constructs.

A minor release MUST NOT silently alter the meaning of existing stable source.

---

9. Patch Versions

A PATCH version is for compatible corrections.

Examples include:

- diagnostic corrections;
- documentation corrections;
- parser bug fixes;
- grammar clarifications;
- conformance corrections;
- implementation corrections;
- compatibility metadata corrections;
- deterministic behavior fixes that preserve specified semantics.

A patch release MUST NOT introduce incompatible language meaning.

---

10. Language Version Is Independent of Rust Version

Zamani language version and Rust compiler version are independent.

For example:

Zamani language: 1.0.0
Zamani compiler: 0.x.y
Rust implementation: 1.97.1

is valid.

Rust 1.97 and Rust 1.97.1 do not constitute separate Zamani language versions.

The production implementation baseline is:

Rust 1.97
Rust 1.97.1
Rust edition 2021

The exact "Cargo.toml" representation is owned by the repository's Rust build configuration.

This document MUST NOT prescribe invalid Cargo syntax.

CI and release validation SHOULD validate the supported Rust baseline explicitly.

---

11. Safe Rust Requirement

The Zamani compiler implementation MUST use safe Rust.

Production implementation MUST NOT introduce Rust "unsafe" merely to implement:

- language versioning;
- grammar handling;
- compatibility checking;
- AST compatibility;
- semantic compatibility;
- IR compatibility;
- migration support;
- diagnostics;
- validation.

If a future external integration requires unsafe internals, that requirement MUST remain outside the language/grammar compatibility contract and MUST NOT become a requirement of Zamani source semantics.

---

12. Compiler Versions

Compiler version and language version are independent.

A compiler MAY support multiple language versions.

For example:

Compiler C
 ├── Zamani 1.0
 ├── Zamani 1.1
 └── Zamani 1.2

The compiler MUST expose its supported language-version set through machine-readable and human-readable tooling where applicable.

An unsupported language version MUST result in a structured diagnostic.

The compiler MUST NOT silently treat an unsupported language version as another version.

---

13. Version Resolution

Every compilable Zamani source unit MUST have a determinable effective language version.

Version resolution MUST be deterministic.

The implementation MAY obtain the version from:

1. explicit source declaration;
2. package/project manifest;
3. workspace declaration;
4. documented legacy default.

The exact precedence is owned by the language-version specification.

Once resolved, the effective language version MUST be available to:

- lexical policy;
- parser policy;
- semantic analysis;
- compatibility checking;
- diagnostics;
- artifact metadata;
- tooling where applicable.

---

14. Version Declaration

The canonical syntax for language-version declarations is owned by:

grammar/specification/language-version.md
grammar/core/versioning.g4

This file MUST NOT invent a competing syntax.

The repository currently contains versioning machinery capable of representing declarations such as:

language Zamani "1.0";

The final accepted syntax MUST have one canonical definition.

If historical forms exist, they MUST be explicitly classified and normalized.

The following must not independently become competing language authorities:

language Zamani "1.0";
version "1.0";
@version("1.0");
pragma version 1.0;

unless the canonical specification deliberately defines them as compatibility forms.

---

15. Version Ranges

Version ranges describe compatibility with language or semantic contracts.

For example:

>=1.0,<2.0

MUST NOT be interpreted as:

- CPU generation;
- GPU generation;
- QPU generation;
- node count;
- memory capacity;
- topology;
- compiler optimization level.

Version ranges and resource requirements are different semantic categories.

---

16. Forward Compatibility

A compiler MUST NOT claim support for a future language version merely because the source happens to parse.

For example, a compiler supporting Zamani 1.x MUST NOT silently interpret Zamani 2.x as Zamani 1.x.

Unknown future semantics MUST result in an explicit diagnostic unless an extension mechanism explicitly defines how they may be handled.

This prevents accidental semantic corruption.

---

17. Backward Compatibility

Within a compatible language-version family, stable source SHOULD remain valid and preserve meaning.

Compatibility exceptions MUST be explicit for features classified as:

- experimental;
- implementation-defined;
- target-defined;
- deprecated;
- unstable;
- opt-in;
- dialect-specific.

The exception MUST be documented in compatibility metadata.

---

18. Experimental Features

A feature MUST NOT become stable merely because grammar rules exist.

Experimental features MUST have:

- unique feature identity;
- owning specification;
- status;
- syntax contract;
- AST contract;
- semantic contract;
- IR contract;
- implementation status;
- compatibility classification;
- migration policy;
- tests.

Feature lifecycle:

PROPOSED
   ↓
DESIGNED
   ↓
EXPERIMENTAL
   ↓
IMPLEMENTED
   ↓
STABLE
   ↓
DEPRECATED
   ↓
REMOVED

A feature MAY remain experimental indefinitely until the required contracts are complete.

---

19. Grammar Presence Does Not Equal Compatibility

The existence of a rule in:

grammar/Zamani.g4

does not establish:

- stable language support;
- AST support;
- semantic support;
- IR support;
- backend support;
- runtime support.

The complete production path is:

Specified
   ↓
Lexically represented
   ↓
Parsed
   ↓
AST represented
   ↓
Semantically validated
   ↓
Canonical semantic representation
   ↓
IR represented
   ↓
Compiler supported
   ↓
Runtime/backend supported where applicable
   ↓
Tested
   ↓
Stable

"grammar/grammar.md" MUST accurately expose implementation status.

---

20. Compatibility of the Canonical Grammar

"grammar/Zamani.g4" is the canonical ANTLR composition root.

Compatibility changes to the grammar MUST preserve the authority relationship:

specification
    ↓
Zamani.g4
    ↓
lexer/parser
    ↓
AST

Modular grammar files MAY provide domain-specific rules.

They MUST NOT establish an alternative language authority.

The root grammar MUST remain the composition point.

---

21. Lexer Compatibility

Version changes MAY affect lexical behavior.

Potential compatibility-sensitive lexical changes include:

- keywords;
- contextual keywords;
- identifiers;
- literals;
- Unicode behavior;
- escape sequences;
- operators;
- punctuation;
- comments;
- interpolation;
- quantum literals.

A lexical change MUST be evaluated for identifier collisions.

A new keyword SHOULD preferably be introduced through:

- contextual keyword behavior;
- explicit namespace qualification;
- feature gating;
- versioned syntax;

when doing so preserves source compatibility.

---

22. Token Identity

Token naming is an implementation contract and must remain coherent across:

grammar/lexer/
grammar/Zamani.g4
grammar/antlr/
src/lexer.rs
src/parser.rs

Duplicate concepts MUST NOT be created accidentally.

For example, the repository's lexical audit has identified possible duplicate concepts such as:

Question / QuestionMark
Ampersand / BitAnd

Any such duplication MUST be resolved by explicit lexical ownership rather than allowing version-specific accidental interpretations.

---

23. Operator Compatibility

Operator compatibility includes:

- token spelling;
- precedence;
- associativity;
- arity;
- parse structure;
- semantic meaning.

Changing precedence or associativity can break existing programs even if all tokens remain valid.

Therefore such changes MUST be classified as compatibility-sensitive.

---

24. AST Compatibility

The AST is the source-semantic structural contract.

A grammar feature is incomplete unless its AST representation is defined.

For every stable construct:

grammar
   ↓
parse tree
   ↓
frontend AST

must preserve all source semantics required by downstream stages.

AST changes MUST identify:

- added nodes;
- removed nodes;
- changed fields;
- changed optionality;
- changed discriminants;
- source-span behavior;
- semantic interpretation.

The AST MUST remain domain-neutral where the architecture requires it.

---

25. AST and Hardware Independence

Portable AST structures MUST NOT require:

- physical CPU IDs;
- physical GPU IDs;
- physical FPGA IDs;
- physical QPU IDs;
- physical qubit assignments;
- calibration records;
- target topology.

Those belong downstream.

For example:

Qubit[n]

may express source-level quantum intent.

A physical mapping such as:

logical q0 -> physical qubit 17

is a later realization decision.

---

26. Semantic Compatibility

Semantic compatibility is stronger than syntactic compatibility.

The following changes are semantic compatibility changes:

- type inference;
- ownership;
- borrowing;
- lifetime semantics;
- effect semantics;
- capability semantics;
- resource semantics;
- evaluation order;
- numerical semantics;
- concurrency semantics;
- quantum measurement semantics;
- module resolution;
- name resolution.

A syntax-preserving semantic change may still require a major language version.

---

27. Resource Compatibility

Resource availability is not language compatibility.

A program may require:

requires qubits >= n

or:

requires memory >= required_memory

or:

requires capability("gpu.compute")

or:

requires capability("quantum.measurement")

Failure to satisfy such a requirement on a target MUST be diagnosed as a resource/capability/target issue rather than silently changing the language semantics.

---

28. No Artificial Compatibility Ceilings

The compatibility system MUST NOT establish artificial language limits such as:

MAX_QUBITS
MAX_CPUS
MAX_CORES
MAX_THREADS
MAX_GPUS
MAX_FPGAS
MAX_QPUS
MAX_NODES
MAX_MEMORY
MAX_REGISTER_WIDTH
MAX_VECTOR_WIDTH
MAX_TENSOR_RANK
MAX_NETWORK_SIZE
MAX_DEVICE_COUNT
MAX_TIMELINES
MAX_PROGRAM_SIZE

These names are examples of prohibited universal capacity assumptions.

A compatibility document MUST NOT hide such limits in tables or prose.

---

29. Scalability Is Independent of Test Capacity

A test environment's capacity is not a language limit.

For example:

tested with N resources

does not imply:

language supports only N resources

The compatibility contract must distinguish:

language expressiveness
        ≠
implementation capacity
        ≠
target capacity
        ≠
currently tested capacity

---

30. Representation Limits

Zamani cannot promise mathematical infinity where finite representations are inherently required.

Therefore scalability means:

«The language and compatibility architecture MUST NOT impose arbitrary domain ceilings; actual execution is bounded only by the representational requirements of the program, explicit semantic constraints, implementation capabilities, and resources available to the selected realization.»

A representation limit must be:

- technically necessary;
- documented;
- implementation-specific where possible;
- independent from arbitrary source-language ceilings.

---

31. POCO-REAF

POCO-REAF means:

Program Once
      ↓
Defined Portable Semantics
      ↓
Canonical Representation
      ↓
Compile Once
      ↓
Target Adaptation
      ↓
Run Everywhere
      ↓
Anywhere
      ↓
Forever

POCO-REAF does not mean that one machine-code binary can execute unchanged on every architecture.

It means that the program's portable semantic contract survives changes in realization.

The implementation MAY use:

- target-independent IR;
- deferred lowering;
- cached artifacts;
- target-specific lowering;
- runtime dispatch;
- AOT compilation;
- JIT compilation;
- heterogeneous compilation;
- distributed compilation;
- remote compilation;
- simulation;
- virtualization.

The source program MUST NOT require semantic rewriting merely because hardware changes.

---

32. Compile Once

"Compile Once" is a semantic portability objective, not a requirement that one physical binary contain every possible target instruction set.

The architecture SHOULD allow:

source
  ↓
canonical semantic representation
  ↓
portable artifact
  ↓
target-specific realization

without changing program meaning.

Where target-specific code generation is necessary, it belongs downstream of the portable semantic boundary.

---

33. Quantum Compatibility

Quantum language compatibility MUST remain independent of particular quantum hardware.

Adding a new:

- gate;
- operation;
- topology;
- native instruction;
- qubit technology;
- QPU;
- simulator;
- calibration model;
- connectivity model;

MUST NOT automatically require a new Zamani language version.

These are generally capability or target concerns.

The language MUST NOT define permanent limits such as:

MAX_QUBITS
MAX_GATES
MAX_REGISTER_SIZE
MAX_CONTROLS
MAX_DEPTH

as universal source-language limits.

---

34. Quantum Operation Model

Quantum syntax MUST support extensible operation semantics.

The architecture must prefer a generic operation model such as:

operation name
namespace
parameters
controls
targets
results
attributes
modifiers
effects
capabilities

rather than making a finite list of today's gates the permanent definition of Zamani quantum computation.

This permits source-level constructs such as:

apply H to q
apply custom_gate to q
apply vendor.operation to q0, q1
apply operation(parameter) to q0, q1

subject to the canonical quantum specification.

A new operation should not require a language-version change merely because a target introduces it.

---

35. Canonical "quantum::ir" Boundary

Quantum compatibility MUST preserve:

Zamani source
    ↓
frontend AST
    ↓
semantic quantum model
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
resilience / QEC / ZQN
    ↓
HAL
    ↓
target

A compatibility migration MUST NOT create a competing frontend quantum IR merely to preserve historical syntax.

The canonical quantum semantic boundary remains "quantum::ir".

---

36. Classical Compatibility

Classical language evolution must preserve the same principles.

New:

- numeric types;
- vector operations;
- matrix operations;
- tensor abstractions;
- parallelism;
- accelerator operations;
- scientific constructs;

MUST NOT introduce artificial hardware limits into source compatibility.

A language-level vector or tensor width is semantic program information.

A target's physical vector register width is a lowering concern.

---

37. HDL and Hardware Compatibility

HDL and hardware syntax must describe intent rather than permanently encode one generation of hardware.

A version change MUST NOT be required merely because:

- an FPGA has more resources;
- an ASIC has a new primitive;
- a register width changes;
- memory capacity increases;
- interconnect topology changes;
- an accelerator gains capabilities.

Target-specific features MAY be represented through explicit dialects or capabilities when required.

The portable core remains target-independent.

---

38. Hardware Capability vs Language Version

These are distinct:

language compatibility
target capability
resource availability
hardware realization

For example:

requires capability("gpu.compute")

does not define a GPU language version.

Likewise:

requires capability("quantum.measurement")

does not select a particular QPU.

---

39. Dialect Compatibility

Dialects MUST have explicit identity and versioning.

A dialect SHOULD identify:

- name;
- version;
- owner;
- specification;
- syntax extensions;
- semantic extensions;
- AST mapping;
- IR mapping;
- capability requirements;
- compatibility requirements.

Dialect version MUST NOT be confused with:

- Zamani language version;
- compiler version;
- Rust version;
- target version.

A dialect may evolve independently subject to its compatibility contract.

---

40. Extension Compatibility

Extensions MUST NOT silently redefine stable core semantics.

The extension hierarchy is:

Zamani core
    ↓
defined extension point
    ↓
dialect / domain extension
    ↓
semantic validation
    ↓
canonical semantic model

An extension MUST declare its compatibility requirements where those requirements affect interpretation.

---

41. Module Compatibility

Module syntax is structural language organization.

It must remain distinct from:

- package version;
- dependency version;
- language version;
- compiler version;
- runtime version;
- hardware version.

Changing package resolution must not silently redefine module syntax.

Module compatibility is governed jointly by:

grammar/modules/
grammar/compatibility/
grammar/specification/

with ownership remaining explicit.

---

42. Dependency Compatibility

Dependencies MUST distinguish:

dependency identity
dependency version
language version
dialect version
compiler version
ABI version
runtime protocol version

A dependency version MUST NOT be interpreted as a hardware version.

Dependency resolution belongs to downstream tooling.

The grammar describes dependency intent.

---

43. Artifact Compatibility

Compiled artifacts MAY contain:

- language version;
- compiler version;
- grammar/schema version;
- AST/schema metadata where needed;
- semantic contract identifiers;
- IR version;
- dialect versions;
- dependency metadata;
- target requirements;
- capability requirements;
- resource requirements;
- reproducibility metadata.

Artifact metadata MUST be sufficient to determine whether the artifact can be safely consumed.

Unknown incompatible artifact versions MUST produce explicit diagnostics.

---

44. IR Compatibility

IR compatibility is distinct from source compatibility.

An IR representation MAY change internally while source compatibility remains stable if:

1. semantic meaning is preserved;
2. migration or translation exists where needed;
3. compatibility metadata is explicit;
4. downstream consumers are updated;
5. no semantic information is lost.

Canonical IR owners remain responsible for their own IR version contracts.

This document does not create a second IR versioning authority.

---

45. Canonical IR Principle

The compatibility system MUST preserve the repository's canonical semantic boundaries.

The following must not be introduced merely for compatibility convenience:

legacy quantum IR
new quantum IR
compatibility quantum IR
frontend quantum IR
backend quantum IR

when a canonical "quantum::ir" already owns the semantic boundary.

Compatibility layers should translate into canonical representations whenever practical.

---

46. Migration Principle

Migration follows:

old representation
      ↓
migration / compatibility layer
      ↓
canonical semantic meaning
      ↓
current representation
      ↓
canonical IR

Migration should preserve semantics rather than preserve obsolete internal structures indefinitely.

Detailed migration procedures are owned by:

grammar/compatibility/migrations.md

---

47. Deprecation

Deprecation is distinct from incompatibility.

A deprecated feature may remain valid.

A deprecated feature MUST have:

- feature identity;
- deprecation version;
- replacement or explanation where applicable;
- warning policy;
- removal policy;
- migration guidance;
- compatibility classification.

Detailed deprecation policy is owned by:

grammar/compatibility/deprecated.md

---

48. Reserved Syntax

Reserved syntax must be governed separately from active language features.

A newly reserved keyword can break source compatibility.

Therefore reservation decisions MUST consider:

- existing identifier usage;
- lexical ambiguity;
- contextual-keyword alternatives;
- migration requirements;
- language-version impact.

Detailed reserved syntax belongs to:

grammar/compatibility/reserved.md

---

49. Compatibility Matrix

The repository SHOULD maintain a machine-readable and human-readable compatibility matrix.

The matrix SHOULD distinguish at least:

Layer| Compatibility Question
Language| Is this language version supported?
Lexer| Is lexical interpretation compatible?
Parser| Is syntax accepted equivalently?
AST| Can source meaning be represented?
Semantics| Is meaning preserved?
Types| Are type rules compatible?
Effects| Are effect guarantees compatible?
Resources| Are requirements representable?
Capabilities| Are required capabilities expressible?
IR| Can semantic meaning be lowered?
Quantum| Is "quantum::ir" compatible?
HDL| Is hardware intent preserved?
Dialect| Is the extension compatible?
Artifact| Can the artifact be consumed?
ABI| Is binary interface compatibility promised?
Runtime| Is runtime protocol compatibility promised?
Target| Can the target realize the program?

A target failure MUST NOT be recorded as a language incompatibility unless the language contract itself is affected.

---

50. Compatibility Categories

Every compatibility decision SHOULD use one of these classifications:

COMPATIBLE
CONDITIONALLY_COMPATIBLE
SOURCE_COMPATIBLE
LEXICALLY_INCOMPATIBLE
SYNTACTICALLY_INCOMPATIBLE
AST_INCOMPATIBLE
SEMANTICALLY_INCOMPATIBLE
IR_INCOMPATIBLE
ARTIFACT_INCOMPATIBLE
ABI_INCOMPATIBLE
RUNTIME_INCOMPATIBLE
TARGET_INCOMPATIBLE
RESOURCE_INSUFFICIENT
CAPABILITY_UNAVAILABLE
DIALECT_INCOMPATIBLE
UNSUPPORTED
UNKNOWN

These are classifications, not rankings.

---

51. Resource Failure Classification

If a valid program cannot execute because a target lacks resources, the compiler SHOULD distinguish:

SOURCE_VALID
SEMANTICALLY_VALID
TARGET_SELECTED
RESOURCE_INSUFFICIENT

from:

INVALID_SOURCE

This is essential for POCO-REAF.

For example, a program requiring more quantum resources than a particular QPU provides remains a valid program.

---

52. No Silent Resource Substitution

A compiler MUST NOT silently change program semantics to fit a smaller target unless the source program explicitly permits such behavior.

Examples of potentially semantic-changing substitutions include:

- reducing requested precision;
- reducing qubit count;
- dropping operations;
- changing error guarantees;
- changing consistency guarantees;
- changing numerical semantics.

If approximation or fallback is permitted, that permission must be part of the program's explicit semantic contract.

---

53. Determinism

Version resolution MUST be deterministic.

Given identical:

source
language version
dialect declarations
dependency contracts
compiler configuration
relevant compatibility metadata

the compatibility decision MUST be reproducible.

A compatibility result MUST NOT depend on:

- arbitrary network order;
- hardware discovery order;
- hash iteration order;
- unspecified filesystem traversal;
- nondeterministic runtime state.

---

54. Diagnostics

Compatibility failures MUST be diagnosable.

Diagnostics SHOULD identify:

- effective language version;
- requested version;
- supported versions;
- feature involved;
- compatibility layer;
- migration path where available;
- source location;
- applicable dialect;
- target/resource distinction where relevant.

Examples of diagnostic categories include:

unsupported-language-version
future-language-version
removed-feature
deprecated-feature
incompatible-syntax
incompatible-semantics
incompatible-dialect
incompatible-ir
incompatible-artifact
resource-insufficient
capability-unavailable
target-incompatible

---

55. Source Location Preservation

Compatibility transformations MUST preserve source locations whenever practical.

When a migration rewrites:

old syntax
    ↓
new syntax

diagnostics should remain traceable to the original source location.

AST and migration metadata SHOULD preserve enough information for tooling to explain the transformation.

---

56. Compatibility and Generated Code

Generated Zamani source MUST declare or inherit a deterministic language-version contract.

Generators MUST NOT emit syntax whose compatibility status is unknown.

Generated code MUST remain subject to the same:

- lexical rules;
- grammar rules;
- semantic rules;
- compatibility rules;
- dialect rules;
- version rules.

---

57. Compatibility and Macros

Macros MUST NOT bypass language-version validation.

Macro definitions and expansions MUST have deterministic compatibility behavior.

A macro MUST NOT silently expand into syntax unavailable under the effective language version.

Macro compatibility belongs jointly to:

grammar/macros/
grammar/compatibility/
grammar/spec/

---

58. Compatibility and Metaprogramming

Compile-time reflection and code generation MUST respect the effective language contract.

A metaprogram MUST NOT create an accidental second language version.

Generated AST structures must ultimately conform to the canonical language/semantic model.

---

59. Compatibility and Interoperability

Interoperability formats such as:

- OpenQASM;
- QIR;
- LLVM-family representations;
- MLIR-family representations;
- HDL formats;
- foreign-function interfaces;

must be treated as interoperability contracts.

They do not become alternative Zamani language authorities.

Compatibility conversion MUST follow:

external representation
      ↓
validated importer
      ↓
canonical Zamani semantic model
      ↓
canonical IR

or the reverse for exports.

---

60. OpenQASM and Quantum Interoperability

OpenQASM compatibility MUST NOT redefine Zamani quantum semantics.

The intended relationship is:

Zamani quantum source
        ↓
frontend AST
        ↓
semantic quantum model
        ↓
quantum::ir
        ↓
OpenQASM export/import where supported

OpenQASM version differences are interoperability concerns unless the Zamani source contract itself changes.

---

61. Hardware and Backend Versioning

Hardware/backend versions MUST remain separate from language versions.

A target description MAY contain:

hardware identity
hardware capabilities
resource availability
topology
calibration metadata
firmware version
backend version

but these do not become source-language version semantics.

A new hardware generation SHOULD be usable without requiring a new language version whenever its capabilities fit existing semantic contracts.

---

62. Compiler Backend Evolution

A compiler backend MAY change:

- optimization;
- instruction selection;
- routing;
- scheduling;
- decomposition;
- memory placement;
- accelerator mapping;
- quantum gate decomposition;
- QEC strategy.

Such changes do not necessarily constitute language-version changes.

They become language-version changes only when they alter specified source semantics.

---

63. Runtime Evolution

Runtime protocols may evolve independently.

Runtime compatibility MUST be explicitly declared where artifacts depend on runtime contracts.

A runtime implementation MUST NOT reinterpret stable source semantics merely because its internal representation changed.

---

64. Reproducibility

A release claiming reproducible compilation SHOULD record:

- language version;
- compiler version;
- relevant grammar/specification revision;
- dialect versions;
- dependency versions;
- canonical IR version where applicable;
- target profile;
- relevant configuration;
- compatibility metadata.

Reproducibility metadata must not contain artificial hardware limits as language semantics.

---

65. Versioned Specifications

Specification documents may evolve independently as documents, but normative semantic changes MUST be associated with the appropriate language version.

Documentation-only changes MUST NOT be presented as language changes.

A specification clarification that changes interpretation of existing valid source is not merely documentation.

It must be classified according to its semantic effect.

---

66. "grammar/grammar.md" Integration

"grammar/grammar.md" MUST report implementation status against the canonical specification.

It SHOULD identify features using statuses such as:

SPECIFIED
IMPLEMENTED
PARTIALLY IMPLEMENTED
PLANNED
DEPRECATED
REMOVED

A feature MUST NOT be marked:

IMPLEMENTED

solely because it appears in "Zamani.g4".

---

67. "grammar/Zamani-Grammar.md" Integration

"Zamani-Grammar.md" remains a broad design and historical source.

Features must be classified as:

stable
proposed
experimental
deprecated
historical
not implemented

A feature becomes stable only through:

proposal
   ↓
semantic design
   ↓
AST contract
   ↓
canonical grammar
   ↓
implementation
   ↓
canonical IR integration
   ↓
tests
   ↓
compatibility acceptance
   ↓
stable

This prevents historical or aspirational material from silently becoming language law.

---

68. Feature-Level Compatibility Contract

Every production language feature SHOULD have a traceable identity.

A feature contract SHOULD identify:

feature ID
name
status
introduced version
deprecated version
removed version, if applicable
owning specification
grammar owner
lexer dependencies
AST representation
semantic representation
IR representation
compiler consumers
runtime consumers
dialect requirements
capability requirements
resource requirements
migration
compatibility class
positive tests
negative tests
boundary tests
scalability tests
determinism tests
hard-coding audit

This allows a feature to be completed independently without requiring undocumented rework elsewhere.

---

69. Per-File Integration Requirement

Any compatibility-related grammar/specification file must document:

Purpose

What the file exists to define.

Owns

The exact contract owned by the file.

Does Not Own

Adjacent responsibilities deliberately excluded.

Inputs

Upstream syntax/contracts.

Outputs

Contracts consumed downstream.

Dependencies

Other files that must exist.

Lexer Integration

Tokens consumed or defined elsewhere.

Parser Integration

Grammar composition point.

AST Integration

AST representation.

Semantic Integration

Semantic consumer.

IR Integration

Canonical IR boundary.

Compiler Integration

Compiler components consuming the result.

Runtime Integration

Runtime consumers where applicable.

Cross-Domain Integration

Quantum/classical/HDL/AI/distributed/etc. interactions.

Compatibility Integration

Version/migration/deprecation relationships.

Tests

Positive, negative, boundary, scalability, compatibility, and determinism tests.

Completion Criteria

Conditions required before the file is considered complete.

---

70. Compatibility Testing

Every compatibility-sensitive change MUST have appropriate tests.

Tests SHOULD include:

positive/
negative/
boundary/
scalability/
compatibility/
determinism/
migration/
diagnostics/

where applicable.

---

71. Lexical Compatibility Tests

Must test:

- existing identifiers;
- new keywords;
- contextual keywords;
- operators;
- literals;
- Unicode;
- comments;
- quantum literals;
- version declarations.

A new lexical rule must be checked for collisions with existing source.

---

72. Parser Compatibility Tests

Must test:

- previously valid syntax;
- newly added syntax;
- ambiguous syntax;
- precedence;
- associativity;
- version-gated syntax;
- deprecated syntax;
- rejected future syntax.

---

73. AST Compatibility Tests

Must verify:

- node identity;
- field preservation;
- optionality;
- source spans;
- attributes;
- names;
- modifiers;
- semantic information.

A grammar change that loses semantic information at AST construction is incompatible even if parsing succeeds.

---

74. Semantic Compatibility Tests

Must verify:

- type meaning;
- ownership meaning;
- effect meaning;
- capability meaning;
- resource meaning;
- evaluation meaning;
- concurrency meaning;
- quantum meaning;
- module resolution;
- error behavior where specified.

---

75. IR Compatibility Tests

Must verify that stable source semantics lower into the intended canonical IR.

For quantum features, tests MUST verify the canonical:

quantum::ir

boundary.

Compatibility tests MUST NOT introduce an unnecessary alternate quantum IR.

---

76. POCO-REAF Scalability Tests

Scalability tests MUST vary resource scale without converting tested sizes into language limits.

Examples include progressively larger:

- arrays;
- tensors;
- modules;
- dependency graphs;
- classical workloads;
- quantum registers;
- quantum operation sequences;
- HDL structures;
- distributed graphs;
- data streams;
- timelines.

Tests may use finite values because tests require finite execution.

Those values MUST NOT become normative language ceilings.

---

77. Tiny-to-Large Compatibility

The same semantic feature must remain conceptually valid across scales.

For example:

one qubit

and:

many qubits

must use the same semantic model.

Likewise:

one node

and:

many nodes

must not require unrelated language versions merely because deployment scale changed.

---

78. Infinity and Resource Availability

Zamani MUST NOT claim literal execution of infinite physical resources.

Instead, "scale to infinity" means:

«No arbitrary language-defined ceiling prevents the expression of computations whose size grows beyond currently available implementation capacity.»

Actual realization remains bounded by:

- physical resources;
- finite storage;
- execution time;
- representational requirements;
- target capabilities;
- explicit semantic constraints.

The language itself MUST NOT turn current resource availability into permanent syntax limits.

---

79. Compatibility and Resource Negotiation

Resource negotiation belongs downstream.

The source may express:

requires ...
prefers ...
allows ...
constrains ...

The compiler/runtime may determine:

available resources
available capabilities
placement
mapping
partitioning
routing
scheduling

Compatibility policy must preserve that separation.

---

80. Compatibility and Preferences

Preferences are not requirements.

For example:

prefer capability("gpu.compute")

does not mean:

program invalid without GPU

Likewise:

requires capability("quantum.measurement")

is a semantic requirement.

Compatibility analysis must preserve this distinction.

---

81. Compatibility and Deterministic Parsing

Version-sensitive parsing must remain deterministic.

The compiler MUST NOT select a language interpretation based on:

- current hardware;
- available GPU;
- available QPU;
- CPU model;
- runtime load;
- network availability.

Hardware selection occurs after language interpretation.

---

82. Compatibility and Security

Compatibility processing MUST NOT execute arbitrary source code.

Version resolution MUST NOT require:

- arbitrary filesystem execution;
- arbitrary network execution;
- arbitrary runtime execution;
- arbitrary hardware access.

Compatibility checks should operate on declared contracts and trusted metadata.

---

83. Compatibility and Supply Chain

Package/dependency compatibility SHOULD include integrity information where supported.

The compatibility layer SHOULD be capable of distinguishing:

requested dependency
resolved dependency
verified dependency
compatible dependency

from:

untrusted dependency

This remains a toolchain/security concern rather than grammar semantics.

---

84. Compatibility and Dialect Isolation

A dialect MUST NOT silently alter core Zamani semantics.

Dialect-specific behavior must be explicit.

If a dialect changes interpretation of existing core syntax, it MUST be treated as a compatibility boundary rather than a transparent extension.

---

85. Compatibility and Future Computing

Future computing models may introduce:

- new physical substrates;
- new accelerators;
- new quantum architectures;
- new memory models;
- new interconnects;
- new execution models;
- new mathematical systems;
- new AI systems.

The language version system SHOULD permit these to be added through:

capabilities
dialects
semantic extensions
resource requirements
interoperability
canonical IR evolution

without forcing unnecessary core-language version changes.

---

86. Compatibility and Nano / Microscopic Computing

Nano or microscopic computation MUST follow the same versioning principles.

New nano capabilities must not impose universal source limits.

The source-level abstraction remains independent of physical substrate.

---

87. Compatibility and Temporal / Multi-Timeline Features

Temporal, speculative, rewind, observation, or multi-timeline semantics must have explicit semantic version contracts.

The language MUST NOT hard-code a universal maximum number of:

- timelines;
- branches;
- observations;
- events;
- temporal states.

Versioning concerns the semantics of the feature, not its physical capacity.

---

88. Compatibility and Sankofa Features

Sankofa-related language concepts must remain versioned as semantic language features or dialect capabilities.

Versioning must not imply a fixed capacity for:

- memories;
- knowledge entries;
- histories;
- learning events;
- recalls;
- agents;
- temporal records.

Actual capacity is a resource/runtime concern.

---

89. Compatibility and AI

AI language features must distinguish:

language construct
model version
model artifact version
framework version
runtime version
accelerator capability

These are not interchangeable.

Changing an external AI framework does not automatically constitute a Zamani language-version change.

---

90. Compatibility and Data/Tensor Semantics

Tensor dimensions and ranks are program semantics.

They must not be confused with hardware vector widths or backend limits.

A compiler MAY reject a target because resources are insufficient.

That does not invalidate the source language.

---

91. Compatibility and Distributed Systems

Distributed syntax must not encode permanent:

- node counts;
- process counts;
- cluster sizes;
- network sizes;
- topology sizes.

Version changes concern distributed semantics, not current infrastructure capacity.

---

92. Compatibility and HDL

HDL semantics may evolve.

Compatibility must distinguish:

portable hardware intent

from:

target synthesis realization

Changing a target's synthesis technology should not require source migration if the same hardware intent remains representable.

---

93. Compatibility and ABI

ABI compatibility is independent of source compatibility.

An ABI change MAY occur while source compatibility remains intact if the compiler can regenerate compatible target artifacts.

ABI guarantees MUST be explicitly documented.

The language MUST NOT imply ABI stability merely because source syntax is stable.

---

94. Compatibility and Binary Artifacts

Binary compatibility depends on:

- target architecture;
- ABI;
- runtime contract;
- artifact format;
- compiler/backend;
- dependency interfaces.

Therefore:

source compatibility

does not automatically imply:

binary compatibility

and vice versa.

---

95. Compatibility and Recompilation

Recompilation MAY be required when:

- target architecture changes;
- ABI changes;
- runtime protocol changes;
- compiler artifact format changes;
- target capabilities change.

Such recompilation does not necessarily violate POCO-REAF.

The invariant is preservation of portable program semantics.

---

96. Compatibility and Optimization

Compiler optimization changes are normally implementation changes.

An optimization MUST NOT change specified program semantics merely to improve target performance.

If a source-level optimization hint has semantic meaning, it must be explicitly specified.

---

97. Compatibility and Approximation

Approximation MUST NOT be introduced silently.

If a target cannot provide exact semantics and approximation is permitted, that permission must be explicitly represented in the source or execution contract.

Otherwise the compiler MUST reject the incompatible realization rather than silently change meaning.

---

98. Compatibility and Error Semantics

Error behavior is compatibility-sensitive when specified by the language.

Changes to:

- error propagation;
- exceptions;
- result semantics;
- failure recovery;
- quantum measurement failure;
- resource exhaustion semantics;

must be evaluated for semantic compatibility.

---

99. Compatibility and Concurrency

Changes to:

- ordering;
- synchronization;
- memory visibility;
- actor semantics;
- task semantics;
- channel semantics;
- determinism guarantees;

may be semantic breaking changes.

A concurrency feature must never be versioned merely according to the number of available threads or cores.

---

100. Compatibility and Memory

Memory semantics must be independent of physical memory size.

Version compatibility must not encode:

RAM capacity
VRAM capacity
register width
cache size
memory-bank count

unless the property is explicitly part of a target capability contract.

---

101. Compatibility and Hardware Topology

Topology changes are target changes.

For example:

linear topology
grid topology
mesh topology
star topology
arbitrary topology

must not require a new source-language version merely because the compiler learns a new routing strategy.

---

102. Compatibility and Routing

Routing is downstream of semantic meaning.

Quantum routing, network routing, memory placement, and accelerator placement MUST NOT redefine source semantics.

Routing algorithm versions are implementation/backend versions.

---

103. Compatibility and Scheduling

Scheduling changes MUST preserve source semantics unless scheduling is itself explicitly observable under the language contract.

A new scheduler does not constitute a language-version change merely because it produces a different execution schedule.

---

104. Compatibility and QEC/ZQN

QEC and ZQN are downstream compatibility layers.

Changes to:

- QEC strategies;
- fault models;
- noise models;
- resilience strategies;
- logical-to-physical mapping;

must not automatically become language-version changes.

The source language specifies quantum semantics.

QEC/ZQN determine how those semantics are realized under physical constraints.

---

105. Compatibility and HAL

HAL contracts describe target capabilities and state.

HAL versions must remain independent of Zamani language versions.

A HAL may expose:

capabilities
resources
topology
availability
calibration
reliability

without redefining the language.

---

106. Version Compatibility and Target Selection

Target selection MUST occur after language semantics are established.

Conceptually:

source
  ↓
language version
  ↓
semantic meaning
  ↓
requirements/capabilities
  ↓
target selection
  ↓
lowering

The target MUST NOT influence the meaning of source syntax.

---

107. Compatibility and Compiler Configuration

Compiler configuration MAY select:

- optimization level;
- target;
- backend;
- diagnostic verbosity;
- cache;
- deployment mode.

Configuration MUST NOT silently change the meaning of stable source unless the configuration is explicitly part of the source semantic contract.

---

108. Compatibility and Build Reproducibility

A reproducible build should record sufficient compatibility metadata to reconstruct:

language version
compiler version
dialect versions
dependency versions
IR versions
relevant target profile

The build must not depend on hidden current hardware characteristics unless the target profile explicitly includes them.

---

109. Compatibility and Documentation

Every compatibility-sensitive feature must be documented consistently across:

grammar/DESIGN.md
grammar/specification/
grammar/spec/
grammar/compatibility/
grammar/Zamani.g4
grammar/grammar.md
grammar/Zamani-Grammar.md

No document may silently contradict a higher-authority normative contract.

---

110. Compatibility and Existing Files

Existing filenames MUST be retained unless an explicit migration requires otherwise.

In particular, this policy intentionally preserves:

grammar/compatibility/versions.md
grammar/compatibility/migrations.md
grammar/compatibility/deprecated.md
grammar/spec/compatibility.md
grammar/spec/versioning.md
grammar/core/versioning.g4
grammar/Zamani.g4
grammar/grammar.md
grammar/Zamani-Grammar.md
grammar/DESIGN.md
grammar/README.md

The objective is integration, not unnecessary renaming.

---

111. Independent Completion Contract

This file is complete only when its downstream relationships are already known.

Owns

- compatibility version policy;
- language-version compatibility classification;
- release compatibility rules;
- cross-layer compatibility principles;
- POCO-REAF compatibility guarantees.

Does not own

- detailed language-version syntax;
- migration algorithms;
- active deprecation list;
- reserved identifiers;
- backend algorithms;
- runtime implementation;
- QEC implementation;
- hardware implementation.

Upstream

grammar/DESIGN.md
grammar/specification/language-version.md
grammar/spec/compatibility.md
grammar/spec/versioning.md

Downstream

grammar/compatibility/migrations.md
grammar/compatibility/deprecated.md
grammar/compatibility/reserved.md
grammar/compatibility/compatibility-matrix.md
grammar/validation/
grammar/Zamani.g4
grammar/grammar.md
src/lexer.rs
src/parser.rs
src/frontend/ast/
semantic analysis
canonical IR
quantum::ir
compiler
runtime
HAL

Cross-domain consumers

classical
quantum
hybrid
hdl
hardware
distributed
ai
data
networking
security
interoperability
dialects
macros
metaprogramming
memory
concurrency
execution
resources

---

112. Production Acceptance Criteria

"grammar/compatibility/versions.md" is production-ready only when:

Authority

- [ ] No competing version authority exists.
- [ ] Language-version semantics remain owned by the language specification.
- [ ] Compatibility semantics remain owned by the compatibility specification.
- [ ] Migration remains owned by "migrations.md".
- [ ] Deprecation remains owned by "deprecated.md".

Language

- [ ] MAJOR/MINOR/PATCH rules are defined.
- [ ] Stable semantic changes are classified.
- [ ] Experimental features cannot silently become stable.
- [ ] Future versions cannot silently be interpreted as older versions.

Lexer/parser

- [ ] Version-sensitive lexical behavior is defined.
- [ ] Version-sensitive parsing is deterministic.
- [ ] Keyword collisions are considered.
- [ ] Operator changes are compatibility-audited.

AST

- [ ] Every stable syntax change has an AST contract.
- [ ] Source spans remain traceable.
- [ ] AST changes are compatibility-classified.

Semantics

- [ ] Semantic changes are classified independently from syntax.
- [ ] Type/effect/resource/capability changes are audited.
- [ ] Quantum semantics are separately audited.
- [ ] Module semantics are separately audited.

IR

- [ ] Canonical IR compatibility is explicit.
- [ ] "quantum::ir" remains the canonical quantum semantic boundary.
- [ ] No duplicate quantum IR is introduced merely for compatibility.

POCO-REAF

- [ ] Language versions remain independent of hardware versions.
- [ ] Resource insufficiency is not treated as source incompatibility.
- [ ] Target capability failure is not silently converted into semantic changes.
- [ ] No artificial hardware ceilings are introduced.

Scalability

- [ ] No universal "MAX_*" capacity is encoded.
- [ ] Versioning does not impose resource ceilings.
- [ ] Test capacity is not presented as language capacity.
- [ ] Tiny and large computations share the same semantic contracts.

Rust

- [ ] Rust 1.97 / 1.97.1 compatibility is documented.
- [ ] Rust version does not define Zamani language version.
- [ ] Production implementation uses Rust 2021.
- [ ] Production implementation uses no "unsafe".

Testing

- [ ] Positive compatibility tests exist.
- [ ] Negative compatibility tests exist.
- [ ] Boundary tests exist.
- [ ] Scalability tests exist.
- [ ] Determinism tests exist.
- [ ] Migration tests exist.
- [ ] Diagnostic tests exist.
- [ ] Cross-domain tests exist.

---

113. Final Compatibility Model

The production Zamani compatibility architecture is:

                    ZAMANI VERSION CONTRACT
                              │
          ┌───────────────────┼───────────────────┐
          │                   │                   │
     Language Version     Dialect Version    Tool/Artifact
          │                   │               Versions
          └───────────────────┼───────────────────┘
                              │
                              ▼
                    Compatibility Analysis
                              │
       ┌──────────────────────┼──────────────────────┐
       │                      │                      │
     Source                 Semantic              Artifact
       │                      │                      │
       ▼                      ▼                      ▼
     Lexer                  AST/Model              IR
       │                      │                      │
       ▼                      ▼                      ▼
     Parser              Semantic Rules        Canonical IR
       │                      │                      │
       └──────────────────────┼──────────────────────┘
                              │
                              ▼
                   Resource / Capability
                         Analysis
                              │
                              ▼
                    Target Compatibility
                              │
          ┌───────────────────┼────────────────────┐
          │                   │                    │
        CPU                  GPU                 FPGA
          │                   │                    │
          ├───────────────────┼────────────────────┤
          │                   │                    │
        ASIC                  QPU              Simulator
          │                   │                    │
          └───────────────────┼────────────────────┘
                              │
                              ▼
                       Future Targets

The invariant is:

Language meaning
        ≠
Compiler implementation
        ≠
IR representation
        ≠
Hardware capability
        ≠
Resource availability
        ≠
Target realization

And therefore:

Program Once
      ↓
Stable Zamani Semantics
      ↓
Canonical AST / Semantic Model
      ↓
Canonical IR
      ↓
Target-independent intent
      ↓
Target-specific realization
      ↓
CPU / GPU / FPGA / ASIC / QPU /
distributed / cloud / embedded /
future computational substrates

The compatibility system exists to preserve this separation across time.

---

114. Final Normative Invariant

The following invariant governs all future changes:

«A new machine, larger machine, smaller machine, new accelerator, new quantum processor, new FPGA, new ASIC, new topology, new runtime, new compiler optimization, new backend, or new computational substrate MUST NOT by itself require a new Zamani language version.»

A language-version change is required only when the language contract itself changes.

Likewise:

«A target that lacks the resources or capabilities required by a valid Zamani program does not make that program invalid.»

And:

«A larger future machine does not require a larger language.»

The language remains capable of expressing larger computations because the grammar and compatibility system do not encode artificial physical ceilings.

The implementation resolves actual realization according to:

program semantics
+
declared requirements
+
capabilities
+
resources
+
constraints
+
target characteristics

while preserving the source program's defined meaning.

This is the compatibility foundation required for Zamani's:

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF) architecture.