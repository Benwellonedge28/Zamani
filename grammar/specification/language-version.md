Zamani Language Versioning Specification

Path: "grammar/specification/language-version.md"
Status: Normative
Scope: Zamani language, grammar, semantic specification, compiler frontend, canonical IR boundaries, tooling, and compatibility
Implementation baseline: Rust 1.97 / Rust 1.97.1
Rust safety requirement: "unsafe" Rust is prohibited
Primary goals: semantic stability, deterministic evolution, backward compatibility, forward extensibility, POCO-REAF, and scale from atom to everywhere

---

1. Purpose

This document defines the versioning model for the Zamani programming language.

It establishes how Zamani versions:

- source syntax;
- lexical rules;
- grammar;
- language semantics;
- types;
- effects;
- capabilities;
- resource requirements;
- classical computation;
- quantum computation;
- hybrid computation;
- HDL and hardware description;
- distributed computation;
- AI/ML and data computation;
- networking and security;
- compilation;
- execution;
- interoperability;
- dialects;
- tooling;
- canonical intermediate representations;
- generated artifacts;
- compatibility guarantees;
- deprecations;
- migrations;
- experimental features; and
- future language extensions.

The versioning system MUST allow Zamani to evolve indefinitely without turning temporary implementation or hardware limitations into permanent language limitations.

The governing principle is:

«Zamani versions semantic contracts, not machine limitations.»

A language version MUST NOT introduce an artificial ceiling on the number or size of:

- qubits;
- logical qubits;
- physical qubits;
- CPUs;
- cores;
- threads;
- GPUs;
- accelerators;
- FPGAs;
- ASIC resources;
- nodes;
- devices;
- memories;
- registers;
- vector lanes;
- tensor dimensions;
- network participants;
- distributed services;
- program elements; or
- other scalable resources.

Actual execution limits belong to resource availability, target capabilities, implementation constraints, or deployment policy.

---

2. Normative terminology

The keywords MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT, SHOULD, SHOULD NOT, MAY, and OPTIONAL are normative.

2.1 Language version

A language version identifies a stable set of Zamani source-level lexical, syntactic, semantic, typing, effect, capability, resource, and compatibility rules.

2.2 Grammar version

A grammar version identifies a version of the concrete syntax representation.

A grammar version MUST NOT be treated as equivalent to a language semantic version.

2.3 Compiler version

A compiler version identifies a particular compiler implementation.

Compiler versions MAY evolve independently of language versions when compatibility is preserved.

2.4 IR version

An IR version identifies a representation contract for an intermediate representation.

IR versions MUST be separately versioned from source syntax.

In particular, "quantum::ir" MUST retain its own explicit compatibility/versioning contract.

2.5 Dialect version

A dialect version identifies an extension to Zamani syntax and/or semantics within an explicitly declared dialect namespace.

2.6 Target version

A target version identifies a target description, ABI, backend, hardware generation, runtime contract, or deployment environment.

A target version MUST NOT silently redefine the meaning of portable Zamani source.

---

3. Versioning hierarchy

Zamani MUST distinguish at least the following layers:

Language Version
        │
        ├── Lexical Contract
        ├── Grammar Contract
        ├── Semantic Contract
        ├── Type Contract
        ├── Effect Contract
        ├── Capability Contract
        ├── Resource Contract
        │
        └── Source Compatibility Contract
                 │
                 ▼
             AST Contract
                 │
                 ▼
          Semantic Resolution
                 │
                 ▼
          Canonical IR Contracts
                 │
        ┌────────┼────────┐
        ▼        ▼        ▼
 classical   quantum   hardware
    IR          IR        IR
                 │
                 ▼
       Optimization / Scheduling
                 │
                 ▼
          Target Lowering
                 │
                 ▼
          Runtime / Hardware

No lower layer MAY redefine a higher-level source semantic contract.

A backend MAY reject a program because a target lacks a required capability.

A backend MUST NOT change the meaning of the source program merely to fit its limitations unless an explicitly defined approximation or alternative semantic mode is requested.

---

4. Language version is not compiler version

A Zamani program MUST identify the language contract it expects rather than requiring an exact compiler implementation version.

For example, conceptually:

language 1.x;

means:

«Compile this source according to the compatible Zamani language contract.»

It MUST NOT mean:

«Require one particular compiler binary.»

Compiler selection remains a tooling concern.

This distinction is essential for POCO-REAF.

---

5. Version authority

The authoritative version relationship MUST be:

language-version.md
        │
        ▼
grammar-authority.md
        │
        ▼
syntax-model.md
        │
        ▼
semantic-model.md
        │
        ▼
compilation-model.md
        │
        ▼
execution-model.md

The individual grammar files MUST implement the contracts established by these specifications.

The following files MUST NOT independently invent incompatible version semantics:

- "grammar/Zamani.g4"
- "grammar/Zamani-Grammar.md"
- "grammar/grammar.md"
- files under "grammar/lexer/"
- files under "grammar/core/"
- files under "grammar/types/"
- files under "grammar/quantum/"
- files under "grammar/hdl/"
- files under "grammar/hardware/"
- or any other grammar subdirectory.

---

6. Existing repository authority

Versioning MUST integrate with the existing repository rather than establish a disconnected language implementation.

The version contract MUST account for existing:

- lexer implementation;
- parser implementation;
- AST structures;
- semantic analysis;
- type checking;
- compiler infrastructure;
- classical representations;
- "quantum::ir";
- quantum optimization;
- scheduling;
- QEC;
- ZQN;
- hardware abstraction;
- calibration;
- benchmarking;
- resource management;
- runtime infrastructure;
- tests;
- examples;
- interoperability;
- documentation; and
- generated artifacts.

The versioning system MUST NOT create a second semantic representation of concepts already owned by these subsystems.

---

7. Current grammar artifacts

The repository currently contains multiple grammar-related representations, including:

grammar/Zamani.g4
grammar/Zamani-Grammar.md
grammar/grammar.md

Their version relationship MUST be explicitly established.

7.1 "grammar/Zamani.g4"

This is the ANTLR grammar representation.

It MUST:

- declare or inherit the language version it implements;
- remain synchronized with the authoritative syntax contract;
- be validated automatically;
- avoid introducing undocumented semantic changes;
- avoid fixed hardware/resource limits; and
- remain compatible with the repository parser architecture.

It MUST NOT independently become the semantic authority.

7.2 "grammar/Zamani-Grammar.md"

This document is explanatory/specification material.

It MUST:

- identify the language version it describes;
- identify whether a construct is normative, explanatory, experimental, or historical;
- avoid documenting syntax that is not part of the declared version;
- avoid silently changing syntax semantics.

7.3 "grammar/grammar.md"

This is an implementation-oriented grammar reference until the repository formally consolidates its grammar authority.

It MUST:

- identify its relationship to the authoritative specification;
- identify implementation-only details;
- avoid becoming an independent competing language specification;
- be checked for divergence.

---

8. Version identifiers

Zamani SHOULD use a semantic versioning-inspired structure:

MAJOR.MINOR.PATCH

with language-specific compatibility rules.

8.1 MAJOR

A major version MAY introduce intentional source-incompatible changes.

A major version MUST NOT be used merely because:

- a compiler was rewritten;
- a backend was added;
- a new CPU appeared;
- a new GPU appeared;
- a new quantum processor appeared;
- an FPGA technology changed;
- a runtime implementation changed;
- an optimization improved;
- a scheduler changed;
- an IR implementation was refactored internally.

Major versions represent changes to the public language contract.

8.2 MINOR

A minor version SHOULD add backward-compatible language functionality.

Examples include:

- new syntax that cannot ambiguously reinterpret existing valid programs;
- new standard constructs;
- new capabilities;
- new resource expressions;
- new interoperable domains;
- new portable abstractions;
- new dialect facilities.

Existing valid source SHOULD retain its meaning.

8.3 PATCH

A patch version MUST be reserved for compatible corrections.

Examples:

- documentation corrections;
- diagnostics improvements;
- parser bug fixes that do not change valid program meaning;
- conformance corrections;
- compatibility metadata fixes;
- implementation corrections;
- specification clarifications.

A patch release MUST NOT silently change the meaning of a valid program.

---

9. Syntax compatibility

A language version MUST define whether syntax is:

1. unchanged;
2. extended;
3. deprecated;
4. removed;
5. reinterpreted; or
6. experimental.

9.1 Existing valid syntax

Existing valid syntax MUST retain its meaning within a compatible version range.

9.2 New syntax

New syntax MUST NOT introduce ambiguity with existing syntax.

Before accepting new syntax, validation MUST check:

- lexical conflicts;
- parser ambiguity;
- precedence conflicts;
- keyword conflicts;
- identifier conflicts;
- AST compatibility;
- semantic ambiguity;
- dialect conflicts;
- macro conflicts;
- tooling impact.

---

10. Keyword evolution

Introducing a new reserved keyword can break existing programs.

Therefore a new keyword MUST undergo compatibility analysis.

A keyword MAY initially be introduced as:

- contextual;
- reserved only in a specific grammar context;
- dialect-scoped;
- explicitly escaped;
- or otherwise compatibility-preserving.

A globally reserved keyword SHOULD NOT be introduced without a migration strategy.

The version specification MUST distinguish:

keyword
contextual keyword
reserved identifier
future-reserved identifier
dialect keyword

---

11. Token evolution

Token definitions are part of the lexical contract.

Changes to tokens MUST be audited for:

- parser effects;
- source compatibility;
- lexical ambiguity;
- operator precedence;
- formatting;
- serialization;
- IDE tooling;
- syntax highlighting;
- macro expansion;
- dialect parsing.

Known token-overlap risks in the repository, such as concepts represented by overlapping operator/token names, MUST be resolved centrally rather than independently in unrelated grammar files.

Examples include potential distinctions such as:

Arrow / ThinArrow
Question / QuestionMark
BitAnd / Ampersand
BitOr / Pipe

The exact canonical token names belong to the lexical authority, not this version document.

---

12. Semantic compatibility

Syntax compatibility alone is insufficient.

A version change is breaking if it changes the semantic meaning of an existing valid program.

Examples of breaking semantic changes include:

- changing evaluation order;
- changing ownership rules;
- changing integer behavior;
- changing overflow semantics;
- changing floating-point guarantees;
- changing concurrency guarantees;
- changing quantum measurement semantics;
- changing effect semantics;
- changing resource requirement meaning;
- changing hardware semantics;
- changing memory visibility;
- changing distributed consistency guarantees.

Such changes MUST require an explicit compatibility classification.

---

13. Semantic identity

For POCO-REAF, the following invariant MUST hold:

same source
+
same language semantic contract
=
same intended program semantics

Target selection MUST NOT alter the semantic identity of the program unless the source explicitly requests target-dependent behavior.

This allows the same semantic program to be lowered to:

CPU
GPU
FPGA
ASIC
QPU
simulator
accelerator
cluster
cloud
embedded target
future target

without requiring a source rewrite merely because the target changed.

---

14. POCO-REAF versioning

POCO-REAF means:

Program Once
Compile Once
Run Everywhere
Anywhere
Forever

The version system MUST distinguish this principle from the impossible promise that one historical machine-code binary can execute natively on every future architecture.

14.1 Program Once

The source program expresses stable computation and intent.

14.2 Compile Once

A compatible compiler MUST be able to produce a stable portable semantic/compiled artifact where the selected compilation model supports it.

That artifact MAY contain:

- target-independent semantics;
- canonical IR;
- capability requirements;
- resource requirements;
- optimization metadata;
- portable executable representation;
- target-lowering information;
- version metadata.

14.3 Run Everywhere

The execution system MUST be able to select an appropriate realization for the available target.

14.4 Run Anywhere

Execution MAY occur locally, remotely, distributed, embedded, cloud-hosted, quantum, classical, or hybrid.

14.5 Run Forever

Long-lived artifacts MUST carry enough version information to permit:

- validation;
- migration;
- compatibility checking;
- semantic preservation;
- target re-lowering;
- future representation conversion.

---

15. Forever does not mean immutable

"Forever" MUST NOT mean that Zamani source syntax or implementation details can never evolve.

It means that language evolution MUST preserve semantic continuity wherever the compatibility policy permits.

A future compiler SHOULD be able to determine:

artifact language version
artifact IR version
required capabilities
required semantics
required dialects
required resource properties
target-independent intent

before execution.

---

16. Versioned artifacts

Compiled and serialized Zamani artifacts SHOULD carry:

language version
grammar compatibility version
semantic version
IR version
dialect versions
capability requirements
resource requirements
compiler provenance
artifact format version
compatibility metadata

They MUST NOT rely solely on a compiler executable's version.

---

17. Source version declarations

A Zamani source file MAY explicitly declare its intended language version.

The declaration MUST identify the language contract, not a hardware configuration.

A conceptual form is:

language 1.0;

A future implementation MAY choose a different concrete syntax, but the semantic requirement remains:

«source version selection MUST be explicit, deterministic, and independently verifiable.»

A source program MUST NOT require:

CPU count
GPU count
QPU name
physical device ID
memory capacity
cluster size

merely to identify its language version.

---

18. Version ranges

Libraries, packages, dialects, and tools SHOULD support version ranges.

Examples conceptually include:

requires Zamani >= 1.2
requires Zamani >= 1.2 < 2.0

The exact dependency syntax belongs to the modules/package grammar.

Version ranges MUST NOT permit a dependency to silently select a semantically incompatible language version.

---

19. Compatibility classes

Every language feature MUST have one of the following compatibility classes:

Stable
Compatible Extension
Experimental
Deprecated
Removed
Reserved
Implementation-Defined
Target-Defined
Dialect-Defined

19.1 Stable

A stable feature is part of the supported public language contract.

19.2 Compatible Extension

A compatible extension adds functionality without changing the meaning of existing valid programs.

19.3 Experimental

Experimental features MAY change or disappear.

They MUST be explicitly marked.

They MUST NOT silently become stable.

19.4 Deprecated

Deprecated features remain temporarily supported while migration is available.

19.5 Removed

Removed features are no longer accepted by the applicable language version.

Removal MUST follow the compatibility policy.

19.6 Reserved

Reserved syntax is intentionally unavailable for current general use but retained for future evolution.

19.7 Implementation-Defined

The language defines a range of permitted implementation behaviors and requires the implementation to document its choice.

19.8 Target-Defined

The behavior depends on an explicitly selected target capability or implementation.

Target-defined behavior MUST NOT masquerade as universal semantics.

19.9 Dialect-Defined

The behavior belongs to an explicitly selected dialect.

19.10 Experimental compatibility

Experimental syntax MUST NOT silently become part of the stable core.

---

20. Deprecation policy

A feature MUST NOT be removed merely because a newer design is preferred.

Before deprecation:

1. identify the feature;
2. identify all repository consumers;
3. identify source compatibility impact;
4. define its replacement;
5. define migration rules;
6. update examples;
7. update tests;
8. update diagnostics;
9. document the deprecation version;
10. document the earliest removal version.

Deprecation MUST NOT be used as a substitute for resolving architectural ownership.

---

21. Migration policy

Every breaking language change MUST provide a migration path where technically possible.

Migration information SHOULD specify:

old syntax
new syntax
semantic differences
automatic migration possibility
manual migration requirements
affected tools
affected AST nodes
affected IR
affected backends
affected dialects

Migration tooling SHOULD operate on structured syntax/AST information rather than fragile text replacement whenever possible.

---

22. Version locking and reproducibility

Builds MUST be reproducible with respect to language semantics.

A reproducible build SHOULD record:

- language version;
- dependency versions;
- dialect versions;
- relevant compiler version;
- IR versions;
- target information;
- compilation configuration;
- feature configuration.

Reproducibility MUST NOT require embedding fixed hardware capacities into source semantics.

---

23. Hardware independence

Language versioning MUST remain independent of hardware generations.

For example:

Zamani 1.x

MUST NOT mean:

Zamani for CPU generation X

or:

Zamani for QPU generation Y

Hardware versions belong to target descriptions.

---

24. Resource independence

Resource availability MUST NOT define language versions.

The language MUST NOT create versions such as:

Zamani-64Q
Zamani-128Q
Zamani-1024Core

because such naming embeds hardware limitations into the language.

Instead:

language version
+
resource requirements
+
target capabilities

MUST determine whether a program can execute.

---

25. Quantum versioning

Quantum language evolution MUST preserve the distinction between:

quantum semantics
logical resources
physical resources
backend capabilities
hardware topology
calibration
noise
error correction
execution scheduling

Quantum syntax belongs to the language/grammar layer.

Canonical quantum semantics MUST ultimately lower into the repository's "quantum::ir".

The grammar MUST NOT become a second quantum IR.

---

26. Quantum compatibility

A new quantum feature MUST identify whether it affects:

- logical circuit semantics;
- measurement semantics;
- state semantics;
- operation semantics;
- parameter semantics;
- dynamic control;
- classical interaction;
- error correction;
- resource requirements;
- physical placement;
- backend capability.

For example, adding a new gate spelling MAY be a compatible syntax extension.

Changing the mathematical semantics of an existing gate is a semantic breaking change.

---

27. Quantum resource versioning

Quantum programs MUST NOT be versioned according to fixed qubit counts.

The following are invalid language-versioning concepts:

language version 1 = max 32 qubits
language version 2 = max 64 qubits

Instead:

program semantics
+
logical resource requirements
+
available capabilities

determine execution feasibility.

A program requiring an arbitrary number of qubits MUST be representable without a grammar-level maximum.

---

28. Classical versioning

Classical language evolution MUST cover:

- scalar types;
- structured types;
- generics;
- control flow;
- memory;
- ownership;
- borrowing;
- concurrency;
- parallelism;
- numerical computation;
- symbolic computation;
- vectorization;
- matrix computation;
- tensor computation;
- accelerator execution.

A classical feature MUST NOT prevent future quantum, hardware, distributed, or other extensions.

---

29. HDL and hardware versioning

HDL and hardware constructs MUST distinguish:

hardware semantics

from:

target technology

A new hardware target MUST NOT require a new core language version solely because the physical technology changed.

For example, adding support for a new FPGA family SHOULD be a target/backend evolution rather than a language breaking change.

A new hardware semantic construct MAY require a language version change.

---

30. Distributed versioning

Distributed constructs MUST version semantic guarantees such as:

- communication;
- ordering;
- consistency;
- replication;
- fault tolerance;
- placement;
- failure handling;
- remote execution.

Changes to distributed semantics MUST be explicit.

Network scale MUST NOT be encoded into language versions.

---

31. AI and data versioning

AI/data language features MUST distinguish:

language semantics
model representation
dataset representation
accelerator capabilities
runtime behavior
training implementation

New accelerator support SHOULD NOT require a new source language version unless source semantics change.

Tensor dimensions MUST remain dynamically or parametrically expressible where the semantics permit.

---

32. Networking and security versioning

Security-sensitive syntax MUST be versioned conservatively.

Changes to:

- authentication semantics;
- authorization;
- capability checking;
- cryptographic operations;
- privacy guarantees;
- trust semantics;
- secure execution;

MUST be explicitly documented.

Security weakening MUST NOT be introduced as an accidental compatibility change.

---

33. Effect and capability versioning

Effects and capabilities are semantic contracts.

Adding a new effect MAY be backward-compatible if it cannot reinterpret existing programs.

Changing an existing effect's meaning MUST be treated as a semantic compatibility event.

Capability identifiers SHOULD be extensible.

A capability MUST NOT imply a particular physical device unless the capability itself explicitly represents device identity.

---

34. Resource-model versioning

The universal resource model MUST distinguish:

resource
requirement
constraint
capability
preference
hint
target
placement

Changes to one concept MUST NOT silently redefine another.

For example:

requires quantum

MUST NOT implicitly mean:

use QPU-XYZ

or:

allocate exactly N physical qubits

unless explicitly expressed by a separate source-level requirement.

---

35. Dialect versioning

Dialects MUST have:

- unique namespaces;
- explicit versions;
- compatibility rules;
- ownership;
- capability declarations;
- reserved syntax boundaries.

A dialect MUST NOT silently redefine core Zamani syntax.

Vendor-specific syntax SHOULD be isolated within a dialect.

A vendor dialect MUST NOT force vendor hardware identifiers into otherwise portable programs.

---

36. Experimental features

Experimental features MUST be explicitly identifiable.

An experimental feature SHOULD specify:

feature name
feature version
status
owner
syntax
semantics
compatibility expectations
known limitations
migration risk
promotion criteria
removal policy

Experimental syntax MUST NOT be required by stable core programs.

---

37. Feature activation

Feature activation MAY be used for experimental or compatibility-sensitive functionality.

Feature activation MUST NOT be required to overcome artificial hardware limits.

For example, this is inappropriate:

feature max_1024_qubits

when the language can naturally represent arbitrary logical qubit collections.

The correct model is capability/resource discovery.

---

38. Reserved space

The language MUST maintain reserved namespace and syntax space for future evolution.

Reserved space SHOULD cover:

- keywords;
- operators;
- annotations;
- dialect identifiers;
- effect identifiers;
- capability identifiers;
- resource kinds;
- target kinds.

Reserved space MUST be documented in:

specification/reserved-space.md

It MUST NOT be used to create arbitrary hidden limits.

---

39. AST compatibility

Language-version changes MUST be mapped to AST compatibility.

The AST MUST represent semantic distinctions required by the active language version.

The grammar MUST NOT depend directly on backend-specific IR structures.

AST evolution MUST be explicit.

A syntax change SHOULD NOT force an unrelated backend redesign.

---

40. Canonical IR compatibility

The AST and semantic layer MUST lower into the appropriate canonical IR.

For quantum programs:

Zamani source
    ↓
lexer
    ↓
parser
    ↓
AST
    ↓
semantic analysis
    ↓
quantum::ir

The grammar MUST NOT import or depend on optimizer, scheduler, QEC, ZQN, or hardware implementation details merely to parse source.

"quantum::ir" remains the canonical quantum semantic boundary.

---

41. IR migration

When a canonical IR changes:

1. determine whether source semantics changed;
2. determine whether only representation changed;
3. provide an IR migration if required;
4. preserve source compatibility where possible;
5. maintain version metadata;
6. update consumers;
7. test serialization/deserialization;
8. test semantic equivalence.

An internal IR refactor MUST NOT automatically become a language major version.

---

42. Compiler compatibility

A compiler MUST reject unsupported language versions explicitly.

It MUST NOT silently compile a newer source program under an older semantic interpretation.

Diagnostics SHOULD identify:

requested language version
supported language versions
required migration

---

43. Forward compatibility

Future compilers SHOULD be able to recognize older stable language versions.

A future compiler MAY support multiple language versions simultaneously.

It MUST NOT reinterpret historical source silently.

---

44. Backward compatibility

A newer compatible language version SHOULD accept older valid programs.

Compatibility MUST be tested at:

- lexical level;
- parsing level;
- AST level;
- semantic level;
- type level;
- effect level;
- capability level;
- resource level;
- IR lowering level.

---

45. Unknown syntax

Unknown future syntax MUST produce deterministic diagnostics unless an explicit extension mechanism is being used.

The parser MUST NOT silently discard unknown constructs.

This is particularly important for:

- future quantum operations;
- future hardware constructs;
- future effects;
- future resource kinds;
- future dialects.

---

46. Unknown capabilities

Unknown capabilities MUST be distinguishable from unsupported syntax.

For example:

syntax recognized
capability unavailable

is different from:

syntax not recognized

This distinction is necessary for portable compilation and runtime negotiation.

---

47. Target negotiation

A portable artifact SHOULD permit the execution environment to determine:

required capabilities
required semantics
resource requirements
optional preferences
target-independent behavior

The target MAY then select an implementation.

This permits:

same program
→ small CPU
→ large CPU
→ GPU
→ FPGA
→ QPU
→ simulator
→ cluster

without changing source semantics.

---

48. Target-specific behavior

Target-specific behavior MUST be explicitly represented.

It MUST NOT leak into the core language through undocumented special cases.

A target-specific construct SHOULD identify:

target
capability
dialect
requirement
constraint

as appropriate.

---

49. No machine-version language forks

Zamani MUST NOT create permanent language forks for individual:

- CPU vendors;
- GPU vendors;
- QPU vendors;
- FPGA vendors;
- operating systems;
- cloud providers;
- cluster sizes;
- deployment providers.

Target-specific implementations belong below the language semantic boundary.

---

50. Compiler implementation safety

All Rust implementation associated with grammar versioning MUST use safe Rust.

The repository MUST NOT use:

unsafe

for lexer, parser, AST, version validation, grammar tooling, compatibility tooling, or related infrastructure.

The implementation baseline is:

Rust 1.97 / Rust 1.97.1

The version specification itself MUST NOT require language features unavailable under the supported Rust baseline.

---

51. Determinism

Version resolution MUST be deterministic.

Given identical:

source
language version
dependency/version constraints
dialect versions
feature configuration
compiler compatibility set

version selection MUST produce the same result.

Version resolution MUST NOT depend on hidden machine properties.

---

52. Version metadata

Every normative language feature SHOULD be traceable to:

feature identifier
introduced version
compatibility class
owning specification
grammar representation
AST representation
semantic representation
IR mapping
tests
documentation

This enables repository-wide auditing.

---

53. Feature lifecycle

Every public feature SHOULD follow:

Reserved
   ↓
Experimental
   ↓
Candidate
   ↓
Stable
   ↓
Deprecated
   ↓
Removed

Not every feature must pass through every state.

A feature MAY remain experimental indefinitely if it is not ready for stabilization.

---

54. Versioning of examples

Examples under:

grammar/examples/

MUST identify the language version they target when ambiguity exists.

Examples MUST NOT silently depend on experimental syntax while being presented as stable examples.

The POCO-REAF example MUST demonstrate semantic portability rather than a fixed machine configuration.

---

55. Versioning of tests

Tests MUST be version-aware.

At minimum, test categories SHOULD include:

lexer compatibility
parser compatibility
semantic compatibility
AST compatibility
IR compatibility
positive compatibility
negative compatibility
boundary compatibility
cross-domain compatibility
dialect compatibility
migration compatibility
round-trip compatibility
determinism
scalability

---

56. Compatibility test matrix

The repository SHOULD maintain a matrix similar to:

Source Version| Compiler| Expected Result
older stable| newer compatible compiler| accept
current stable| current compiler| accept
current stable| older incompatible compiler| explicit rejection
experimental| unsupported compiler| explicit rejection
removed feature| compatible removal version| explicit diagnostic
dialect A| dialect A-compatible compiler| accept
dialect A| no dialect support| explicit diagnostic

The exact supported range belongs to the release policy.

---

57. Scalability compatibility

Version evolution MUST be tested against arbitrarily scalable source constructs.

Tests MUST verify that introducing a language version does not accidentally create fixed limits on:

- program length;
- declaration count;
- nesting;
- qubit references;
- register cardinality;
- collection size;
- tensor dimensions;
- nodes;
- devices;
- resource requirements.

Implementation limits MAY exist because of available memory or execution resources.

Such limits MUST NOT be presented as language semantic limits unless they are genuinely required by the language.

---

58. "Infinity" semantics

Zamani's scalability goal is conceptually:

«no artificial finite language-imposed ceiling.»

This does not mean that a physical machine has infinite resources.

Therefore:

language scalability
≠
physical resource infinity

Instead:

language has no arbitrary fixed ceiling
+
implementation uses representable/dynamic structures
+
execution is bounded by actual resources

This distinction MUST be preserved across every language version.

---

59. Versioning must not hard-code resources

The following are prohibited as language-version boundaries:

MAX_QUBITS
MAX_CORES
MAX_THREADS
MAX_GPUS
MAX_FPGAS
MAX_NODES
MAX_MEMORY
MAX_TENSOR_RANK
MAX_VECTOR_WIDTH

unless a specific value is an implementation safety limit external to the language contract.

If such a value exists internally, it MUST be classified as:

implementation limitation

and MUST NOT become part of source semantics.

---

60. Compiler limits

A compiler MAY have practical limits.

For example:

- available memory;
- compilation time;
- operating-system limitations;
- parser stack/resource limits;
- backend capacity.

Such limits MUST be:

1. documented;
2. distinguishable from language limits;
3. reported diagnostically;
4. removable or scalable where practical.

A compiler limitation MUST NOT be encoded into the language specification as a semantic maximum.

---

61. Compatibility with runtime evolution

Runtime evolution MUST preserve language semantics.

A runtime MAY gain:

- new hardware support;
- new scheduling;
- new optimization;
- new resource discovery;
- new deployment mechanisms.

These changes SHOULD NOT require source changes.

---

62. Compatibility with optimization

Optimization changes MUST preserve observable language semantics.

An optimization release MUST NOT become a language version solely because:

optimization algorithm changed

This applies to:

- classical optimization;
- quantum optimization;
- gate reduction;
- scheduling;
- routing;
- hardware mapping;
- accelerator lowering.

---

63. Compatibility with ZQN

ZQN evolution MUST remain below the language semantic boundary unless it changes exposed source semantics.

Changes to:

- noise models;
- calibration;
- fault models;
- noise-aware execution;
- backend analysis;

MUST NOT automatically change Zamani source semantics.

If a source-visible noise semantic is introduced, it MUST receive explicit language/version treatment.

---

64. Compatibility with QEC

Quantum error-correction features MUST distinguish:

source-level semantic intent

from:

QEC implementation strategy

Changing the internal QEC implementation SHOULD NOT require a language version change if the externally visible semantics remain compatible.

---

65. Compatibility with scheduling

Scheduling algorithms MAY evolve independently.

A scheduler MAY select different valid execution schedules while preserving required program semantics.

Scheduling policy MUST NOT become hidden source syntax.

---

66. Compatibility with hardware abstraction

Hardware abstraction changes MUST preserve portable source semantics.

Adding support for a new:

- CPU;
- GPU;
- FPGA;
- ASIC;
- QPU;
- accelerator;
- interconnect;

SHOULD normally be a target/backend evolution.

---

67. Compatibility with interoperability

Foreign interfaces MUST identify their own version domains.

For example:

Zamani language version
+
foreign ABI version
+
foreign library version

must remain distinguishable.

An external API changing MUST NOT silently redefine Zamani core semantics.

---

68. Package and module compatibility

Modules and packages MUST declare compatible language requirements.

A dependency SHOULD identify:

minimum language version
maximum incompatible version
dialect requirements
feature requirements
ABI requirements

Package management belongs to the appropriate tooling layer; this document defines only the language-version contract consumed by that tooling.

---

69. Source-to-source compatibility

A migration tool SHOULD be able to transform source from an older version to a newer compatible version.

The migration MUST preserve semantics where the compatibility policy promises preservation.

Automated migration MUST NOT silently:

- change hardware requirements;
- change quantum semantics;
- alter security behavior;
- alter resource constraints;
- change numerical meaning.

---

70. Diagnostics

Version diagnostics MUST be:

- deterministic;
- precise;
- actionable;
- source-located;
- version-aware.

A diagnostic SHOULD identify:

current source version
required version
feature involved
compatibility status
reason
migration path

---

71. Documentation synchronization

Whenever a feature changes version status, the following SHOULD be checked:

grammar documentation
language specification
ANTLR grammar
lexer
parser
AST
semantic analysis
IR mapping
tests
examples
tooling
compatibility documentation

The change MUST be recorded in the appropriate compatibility/migration documentation.

---

72. Generated artifacts

Generated parsers, lexers, documentation, and other artifacts MUST identify the source grammar/specification version from which they were generated where practical.

Generated artifacts MUST NOT be edited manually as a substitute for updating their source.

The generation pipeline MUST be reproducible.

---

73. Grammar generation contract

Where "Zamani.g4" or other grammar files are generated or composed:

normative specification
        ↓
grammar source
        ↓
generated parser/lexer

must remain a one-way derivation.

The generated parser MUST NOT become a source of truth.

---

74. No circular version authority

The following architecture is prohibited:

language version
    ↓
grammar
    ↓
compiler
    ↓
IR
    ↓
language version

Instead:

language specification
    ↓
grammar
    ↓
AST
    ↓
semantic analysis
    ↓
canonical IR
    ↓
compiler/runtime

Version metadata MAY accompany every stage, but ownership MUST remain acyclic.

---

75. Cross-domain versioning

A source program may combine domains.

For example:

classical
+
quantum
+
HDL
+
hardware
+
distributed
+
AI

The language version MUST remain one coherent source-level contract.

Domain extensions MUST NOT create incompatible semantic islands.

Cross-domain constructs MUST have explicitly defined compatibility behavior.

---

76. Version composition

A program may depend on multiple dialects.

Conceptually:

Zamani core
+
quantum dialect
+
hardware dialect
+
vendor dialect

Each component MUST retain its own version identity.

The compiler MUST validate the complete compatibility set before semantic lowering.

---

77. Version conflict resolution

If two dependencies require incompatible language versions, tooling MUST report the conflict explicitly.

It MUST NOT silently select an incompatible version.

Where multiple versions can coexist safely, the module/package system MAY isolate them.

---

78. Version conflict and hardware

Hardware capability conflicts MUST NOT be confused with language-version conflicts.

For example:

language compatible
hardware capability unavailable

is not a language-version failure.

The correct diagnostic should identify the missing target capability.

---

79. ABI versioning

ABI compatibility MUST be separately versioned.

Language version:

semantic contract

ABI version:

binary calling/data-layout contract

must not be conflated.

Changing an ABI MUST NOT automatically require a language major version.

---

80. Data representation compatibility

Serialization formats, binary formats, and persistent data formats MUST have independent version identifiers.

A language version MUST NOT implicitly guarantee binary compatibility unless explicitly specified.

---

81. Source encoding compatibility

Source encoding rules MUST remain stable and explicit.

Unicode evolution MUST be handled without introducing ambiguous identifiers or tokenization.

Changes to Unicode identifier rules require lexical compatibility analysis.

---

82. Version-aware security

Version metadata MUST NOT be trusted blindly when it comes from untrusted artifacts.

Tooling SHOULD validate:

- version syntax;
- supported ranges;
- signatures where applicable;
- dependency constraints;
- dialect identity;
- artifact integrity.

Security validation belongs to the security/tooling layers, but language version metadata MUST be designed to support it.

---

83. Reproducible semantic compilation

A semantic compilation record SHOULD make it possible to establish:

source version
+
source
+
dependencies
+
dialects
+
semantic configuration
=
canonical semantic result

Target realization MAY then vary according to available resources and capabilities.

---

84. Future hardware

A future hardware platform MUST NOT require changing historical source merely because the machine is new.

The intended evolution is:

existing Zamani program
        ↓
existing semantic contract
        ↓
future compiler/runtime
        ↓
future target lowering

This is a fundamental POCO-REAF requirement.

---

85. Future computational paradigms

The language MUST reserve extension mechanisms for future paradigms not currently represented by:

- classical computing;
- quantum computing;
- HDL;
- distributed computing;
- AI;
- networking;
- accelerators.

Future paradigms SHOULD enter through:

stable semantic abstractions
+
capabilities
+
dialects
+
resource contracts
+
canonical IR extensions

rather than forcing unrelated core-language rewrites.

---

86. Versioning of future paradigms

A future paradigm SHOULD receive its own feature/dialect/version contract before becoming part of the stable core.

Its integration MUST specify:

syntax
semantics
types
effects
resources
capabilities
AST
IR
compiler
runtime
testing
compatibility

---

87. Repository file ownership

This file owns:

- language version identity;
- language version lifecycle;
- version compatibility principles;
- version classification;
- language-version evolution rules;
- relationship between language and implementation versions.

This file does not own:

- concrete token definitions;
- parser productions;
- AST structs;
- quantum IR schema;
- QEC algorithms;
- ZQN implementation;
- scheduler implementation;
- hardware backend implementation;
- runtime implementation;
- package-manager implementation;
- individual target specifications.

---

88. Integration with "grammar/specification/README.md"

"specification/README.md" MUST identify this document as the authoritative versioning contract.

It MUST link the language-version contract with:

language-principles.md
language-scope.md
compatibility.md
grammar-authority.md

No specification README MAY redefine version semantics.

---

89. Integration with "language-principles.md"

"language-principles.md" defines the overarching principles.

This file operationalizes those principles for version evolution.

In particular, versioning MUST preserve:

- semantic stability;
- portability;
- extensibility;
- safety;
- deterministic behavior;
- scalability;
- POCO-REAF.

---

90. Integration with "language-scope.md"

"language-scope.md" defines what Zamani is intended to express.

This file defines how that scope evolves over time.

A newly supported computational domain MUST have an explicit compatibility classification.

---

91. Integration with "compatibility.md"

"compatibility.md" MUST define the general compatibility policy.

This file defines the language-version identity and lifecycle used by that policy.

If a conflict exists:

language-version.md

owns version identity, while:

compatibility.md

owns compatibility procedure.

---

92. Integration with "grammar-authority.md"

"grammar-authority.md" MUST define which artifact is authoritative for syntax.

This file defines how that authoritative syntax contract is versioned.

No grammar representation may independently declare incompatible language semantics.

---

93. Integration with "syntax-model.md"

"syntax-model.md" defines concrete syntax architecture.

Version changes to syntax MUST be classified according to this document.

---

94. Integration with "semantic-model.md"

"semantic-model.md" defines semantic meaning.

Semantic changes MUST be evaluated as version compatibility events.

---

95. Integration with compilation and execution

"compilation-model.md" and "execution-model.md" MUST consume language-version metadata.

They MUST distinguish:

language compatibility

from:

target capability

and:

implementation availability

---

96. Integration with lexical grammar

Files under:

grammar/lexer/

MUST:

- implement the active lexical contract;
- identify incompatible lexical changes;
- avoid undocumented keyword changes;
- preserve deterministic tokenization.

They MUST NOT independently decide language version numbers.

---

97. Integration with core grammar

Files under:

grammar/core/

MUST implement version declarations, metadata, capabilities, requirements, constraints, and related source-level contracts.

Version metadata MUST remain distinct from target resource values.

---

98. Integration with types

Files under:

grammar/types/

MUST identify type-system changes by language version.

Type syntax and type semantics MUST be separately evaluated.

---

99. Integration with quantum grammar

Files under:

grammar/quantum/

MUST:

- identify quantum feature versions;
- preserve backend-independent semantics;
- support extensible operations;
- avoid fixed qubit limits;
- lower semantically toward "quantum::ir".

Quantum grammar files MUST NOT embed target-generation versions into core quantum syntax.

---

100. Integration with HDL and hardware grammar

Files under:

grammar/hdl/
grammar/hardware/

MUST distinguish language semantic versions from:

- FPGA versions;
- ASIC processes;
- CPU generations;
- GPU generations;
- QPU generations;
- hardware device revisions.

---

101. Integration with resources

Files under:

grammar/resources/

MUST represent resource contracts without turning resource capacities into language-version limits.

---

102. Integration with dialects

Files under:

grammar/dialects/

MUST own dialect registration/version syntax.

Core language versioning remains owned here.

Dialect compatibility MUST be validated before semantic compilation.

---

103. Integration with validation

Files under:

grammar/validation/

MUST validate:

- version declarations;
- compatibility;
- version ranges;
- reserved features;
- deprecated features;
- hard-coding rules;
- dialect compatibility;
- deterministic version resolution.

---

104. Integration with compatibility directory

Files under:

grammar/compatibility/

MUST provide operational compatibility information:

versions.md
migrations.md
deprecated.md
reserved.md
compatibility-matrix.md

This file supplies their normative version model.

---

105. Version manifest

The repository SHOULD maintain machine-readable version metadata alongside human-readable documentation.

The machine-readable representation SHOULD include:

language_version
grammar_version
specification_version
supported_compiler_range
supported_rust_range
stable_features
experimental_features
deprecated_features
reserved_features
dialect_versions
IR_compatibility

The exact file location MUST be established by the repository's tooling architecture rather than duplicated across multiple manifests.

---

106. Rust compatibility

The grammar/compiler infrastructure MUST remain compatible with:

Rust 1.97

or:

Rust 1.97.1

as the supported baseline.

The implementation MUST use safe Rust only.

No versioning feature may require "unsafe" code.

---

107. Version validation algorithm requirements

Version validation tooling MUST:

1. parse the version declaration;
2. validate its syntax;
3. identify the language version;
4. determine compiler support;
5. resolve dialect requirements;
6. resolve feature requirements;
7. validate compatibility;
8. produce deterministic diagnostics;
9. refuse unsupported semantic interpretation;
10. pass validated metadata to later compilation stages.

The validator MUST NOT inspect hardware capacity to determine whether a language version is valid.

Hardware capability checks belong to target/resource validation.

---

108. Version and resource separation

The following conceptual flow is REQUIRED:

Language Version
      │
      ▼
Language Semantics
      │
      ▼
Program Requirements
      │
      ▼
Target Capabilities
      │
      ▼
Resource Availability
      │
      ▼
Execution Plan

The reverse flow is prohibited:

hardware size
      ↓
language version
      ↓
language semantics

---

109. Version and optimization separation

Optimization MAY depend on compiler version and target capabilities.

It MUST preserve the selected language semantic contract.

A compiler may optimize:

small machine
large machine
CPU
GPU
QPU
FPGA
cluster

without changing the language version.

---

110. Version and scheduling separation

Scheduling decisions MUST remain implementation/target concerns unless scheduling behavior is explicitly observable language semantics.

Changing scheduling algorithms SHOULD NOT require source-language migration.

---

111. Version and physical placement

Physical placement is not a language version.

A physical mapping may change between executions:

logical qubit
→ physical qubit A

or:

logical qubit
→ physical qubit B

without changing the language semantics.

---

112. Version and simulation

A quantum program MUST be able to target a simulator without requiring a different language version solely because execution changes from:

QPU

to:

simulation

provided the simulator implements the required semantics.

---

113. Version and heterogeneous execution

A single source program MAY contain multiple computational domains.

Its language version remains one coherent contract.

Target realization MAY distribute work across:

CPU
GPU
QPU
FPGA
network
storage

subject to capabilities and resource constraints.

---

114. Version and deployment

Deployment environments MAY have their own versions.

For example:

Zamani language version
+
runtime version
+
deployment environment version

must remain distinct.

Deployment upgrades MUST NOT silently redefine source semantics.

---

115. Version and persistence

Persistent artifacts SHOULD include semantic version information sufficient to determine whether they remain interpretable.

If an artifact cannot be safely interpreted, tooling MUST fail explicitly rather than guessing.

---

116. Version and hashing

If canonical source or IR hashes are used elsewhere in the repository, version identity MUST be included where necessary to prevent semantically different representations from colliding.

The exact hashing scheme belongs to the appropriate canonicalization/hashing subsystem.

---

117. Version and signatures

If language artifacts are signed, the signed metadata SHOULD include the applicable:

language version
dialect versions
artifact format version
IR version

so that signatures do not accidentally authorize an artifact under a different semantic interpretation.

---

118. No hidden version changes

A compiler MUST NOT change the effective language version because:

- a new target was selected;
- optimization was enabled;
- a hardware backend was selected;
- a runtime changed;
- a scheduler changed;
- a simulator was selected.

The source language version remains explicit.

---

119. No version-dependent hardware limits

The following pattern is prohibited:

version 1 → 32 qubits
version 2 → 64 qubits
version 3 → 128 qubits

The correct model is:

version N
+
program qubit requirements
+
target capabilities

---

120. Compatibility with arbitrary scale

A language-version implementation MUST remain capable of representing source programs at arbitrary scale subject only to actual implementation resources.

Examples include:

1 qubit
10 qubits
10^3 qubits
10^6 logical references

provided the implementation and execution environment can represent and process them.

The grammar MUST NOT define a fixed maximum merely for convenience.

---

121. Completion contract for this file

This file is complete only when all of the following are true:

Version identity

- [ ] Language version semantics are defined.
- [ ] Grammar version is distinguished from language version.
- [ ] Compiler version is distinguished from language version.
- [ ] IR versions are distinguished from language versions.
- [ ] Dialect versions are distinguished from core language versions.
- [ ] Target versions are distinguished from language versions.

Compatibility

- [ ] Major/minor/patch behavior is defined.
- [ ] Stable features are defined.
- [ ] Experimental features are defined.
- [ ] Deprecation is defined.
- [ ] Removal is defined.
- [ ] Migration is defined.
- [ ] Forward compatibility is defined.
- [ ] Backward compatibility is defined.

POCO-REAF

- [ ] Program Once is defined.
- [ ] Compile Once is defined without promising one universal machine binary.
- [ ] Run Everywhere is defined.
- [ ] Run Anywhere is defined.
- [ ] Run Forever is defined through semantic/artifact evolution.

Scalability

- [ ] No language-version hardware limits exist.
- [ ] No fixed qubit limit exists.
- [ ] No fixed CPU/core/thread limit exists.
- [ ] No fixed accelerator limit exists.
- [ ] No fixed node/cluster limit exists.
- [ ] No fixed tensor/data scale exists where abstraction permits dynamic scale.
- [ ] Physical limits are separated from language semantics.

Repository integration

- [ ] "Zamani.g4" relationship is defined.
- [ ] "Zamani-Grammar.md" relationship is defined.
- [ ] "grammar.md" relationship is defined.
- [ ] lexer integration is defined.
- [ ] parser integration is defined.
- [ ] AST integration is defined.
- [ ] semantic integration is defined.
- [ ] "quantum::ir" integration is defined.
- [ ] QEC integration boundary is defined.
- [ ] ZQN integration boundary is defined.
- [ ] optimization integration is defined.
- [ ] scheduling integration is defined.
- [ ] hardware integration is defined.
- [ ] resource integration is defined.
- [ ] runtime integration is defined.
- [ ] dialect integration is defined.
- [ ] compatibility tooling integration is defined.

Safety

- [ ] Rust 1.97/1.97.1 compatibility is explicit.
- [ ] "unsafe" is prohibited.
- [ ] deterministic version resolution is required.

Future-proofing

- [ ] future hardware can be added without source-language forks;
- [ ] future computational paradigms can be added through extensible mechanisms;
- [ ] future dialects have version isolation;
- [ ] version metadata is extensible;
- [ ] reserved space is defined elsewhere and linked to this contract.

---

122. File ownership contract

File

grammar/specification/language-version.md

Purpose

Define the normative versioning model for Zamani.

Owns

- language version identity;
- language-version lifecycle;
- language compatibility classes;
- version evolution rules;
- source-version semantics;
- language-version metadata requirements;
- relationship between language versions and other version domains.

Does Not Own

- concrete grammar productions;
- lexer token definitions;
- AST implementation;
- canonical quantum IR;
- QEC implementation;
- ZQN implementation;
- scheduler implementation;
- optimization implementation;
- hardware backend implementation;
- runtime implementation;
- package-manager implementation.

Inputs

- language principles;
- language scope;
- grammar authority;
- repository architecture;
- existing frontend;
- existing IR boundaries;
- compatibility requirements.

Outputs

- normative version contract;
- version lifecycle;
- compatibility classification;
- version metadata requirements.

Dependencies

Conceptually:

language-principles.md
language-scope.md

This document MUST remain understandable without requiring implementation details from later grammar files.

Upstream contracts

- "language-principles.md"
- repository grammar authority
- repository semantic architecture

Downstream consumers

- "grammar-authority.md"
- "syntax-model.md"
- "semantic-model.md"
- "compilation-model.md"
- "execution-model.md"
- "compatibility.md"
- "compatibility/*"
- "dialects/*"
- lexer/parser validation
- compiler version validation
- artifact tooling

Public grammar contract

Version declarations and version-related source constructs defined elsewhere MUST conform to this document.

AST contract

Version information MUST be represented explicitly where semantic processing requires it.

Semantic contract

The language version determines the source-level semantic contract.

IR integration

IR versions remain independently owned by their respective IR systems.

Language-version metadata MAY accompany IR artifacts but MUST NOT redefine IR semantics.

Compiler integration

The compiler MUST validate language versions before semantic compilation.

Runtime integration

Runtime systems MAY use language/IR metadata to determine compatibility.

They MUST NOT redefine the source language.

Tooling integration

Tooling MUST use deterministic version parsing and compatibility resolution.

Cross-domain integration

All computational domains use the same core language-version contract while retaining their own dialect/IR/target versions where required.

Tests

Required tests include:

- version parsing;
- version comparison;
- valid version declarations;
- invalid declarations;
- compatibility;
- incompatibility;
- migration;
- experimental features;
- deprecated features;
- dialect compatibility;
- cross-domain compatibility;
- artifact metadata;
- deterministic resolution.

Negative tests

Must reject:

- malformed versions;
- unsupported versions;
- unknown incompatible versions;
- conflicting requirements;
- silent fallback to incompatible semantics;
- undeclared experimental features where declaration is required.

Boundary tests

Must test:

- earliest supported version;
- current version;
- next-version compatibility;
- major-version incompatibility;
- very large version component values where representable;
- large dependency graphs;
- large dialect sets.

No test may encode a false machine-size ceiling.

Compatibility requirements

Existing stable programs MUST remain compatible according to the declared compatibility policy.

Scalability requirements

Version metadata MUST scale with program/dependency complexity without fixed resource assumptions.

Hard-Coding Audit

Search for:

MAX_VERSION
MAX_FEATURES
MAX_DIALECTS
MAX_DEPENDENCIES
MAX_QUBITS
MAX_CORES
MAX_DEVICES

Any such limit MUST be justified as an implementation/resource limit rather than language semantics.

Completion Criteria

This file is complete when:

1. language version identity is unambiguous;
2. version classes are defined;
3. compatibility policy is defined;
4. POCO-REAF implications are defined;
5. hardware/resource independence is guaranteed;
6. quantum/classical/HDL/hardware integration is defined;
7. IR boundaries are preserved;
8. Rust 1.97/1.97.1 and safe-Rust requirements are explicit;
9. migration/deprecation rules are explicit;
10. repository integration contracts are explicit;
11. no downstream file can require reopening this file merely to establish basic version semantics.

---

123. Final invariant

The entire Zamani versioning architecture MUST preserve:

Language Version
      ↓
Stable Program Semantics
      ↓
Portable Semantic Representation
      ↓
Capability / Resource Negotiation
      ↓
Target-Specific Realization
      ↓
Execution

and MUST NOT become:

Machine
  ↓
Machine limitation
  ↓
Language version
  ↓
Restricted source language

The ultimate invariant is:

«A Zamani language version describes what a program means, not how large the machine happens to be.»

Therefore:

One Program
      ↓
One Stable Semantic Meaning
      ↓
Many Compilers
      ↓
Many IR Realizations
      ↓
Many Targets
      ↓
Many Architectures
      ↓
Many Hardware Configurations
      ↓
Many Scales
      ↓
Many Execution Environments
      ↓
Future Platforms

subject only to actual semantic compatibility, available capabilities, implementation support, and physical resources.

This is the versioning foundation required for:

Zamani — From Atom to Everywhere

and:

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF).