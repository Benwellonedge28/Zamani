

grammar/dialects/registry.md

# Zamani Dialect Registry

Path: `grammar/dialects/registry.md`

Project: Zamani Universal Programming Language

Status: **NORMATIVE DIALECT REGISTRY CONTRACT**

Grammar baseline: ANTLR4

Compiler baseline: Rust 1.97 / Rust 1.97.1

Rust edition: Rust 2021

Safety: **Safe Rust only; `unsafe` is prohibited**

Architecture:

> Program Once → Compile Once → Run Everywhere → Anywhere → Forever

Abbreviation:

> **POCO-REAF**

---

# 1. Purpose

This document defines the normative architecture and contract for the **Zamani Dialect Registry**.

The dialect registry is the semantic registry through which Zamani identifies, validates, resolves, versions, composes, and governs language dialects and their extensions.

The registry exists so that Zamani can support:

- classical computing;
- quantum computing;
- hybrid computing;
- HDL;
- hardware/software co-design;
- embedded computing;
- accelerators;
- AI/ML;
- tensor computing;
- scientific computing;
- distributed computing;
- networking;
- security;
- cryptography;
- data processing;
- edge computing;
- cloud computing;
- HPC;
- future computational models;
- organization-specific extensions;
- vendor extensions;
- experimental extensions;

without turning the core Zamani grammar into a closed catalogue of technologies.

The registry is therefore **open-world**.

A new dialect must be able to exist without requiring a modification to the universal grammar merely because the dialect was invented after the compiler was released.

---

# 2. Registry Authority

The registry is part of the following architecture:

```text
Zamani source
     |
     v
canonical lexer
     |
     v
canonical parser
     |
     v
domain-neutral AST
     |
     v
semantic analysis
     |
     +----------------------+
     |                      |
     v                      v
Dialect Registry       Core Language
     |                      |
     +----------+-----------+
                |
                v
       Capability Resolution
                |
                v
       Compatibility Analysis
                |
                v
       Resource / Constraint Analysis
                |
                v
       Canonical Semantic Model
                |
       +--------+---------+-------------+
       |                  |             |
       v                  v             v
 Classical IR        quantum::ir   HDL/Hardware IR
       |                  |             |
       +------------------+-------------+
                          |
                          v
                  Optimization
                          |
              +-----------+-----------+
              |           |           |
              v           v           v
           Routing   Scheduling   Resilience
              |           |           |
              +-----------+-----------+
                          |
                          v
                         ZQN
                          |
                          v
                         HAL
                          |
                          v
                  Target realization

The registry is therefore between parsing and target realization.

It is not part of the lexical layer.

It is not part of the parser.

It is not an IR.

It is not a runtime hardware catalogue.


---

3. Single Authority Rule

There must be exactly one semantic authority for dialect registration.

The responsibilities are divided as follows:

grammar/dialects/dialect.g4
    Public dialect composition boundary

grammar/dialects/dialects.g4
    Existing public dialect facade / compatibility boundary

grammar/dialects/registration.g4
    Dialect declaration syntax

grammar/dialects/namespaces.g4
    Namespace syntax

grammar/dialects/versioning.g4
    Dialect-context version adapter

grammar/dialects/capabilities.g4
    Dialect-context capability adapter

grammar/dialects/compatibility.g4
    Compatibility declaration syntax

grammar/dialects/extension-points.g4
    Extension-point syntax

grammar/dialects/vendor.g4
    Explicit vendor-extension syntax

grammar/dialects/experimental.g4
    Experimental-extension syntax

grammar/dialects/registry.md
    Registry semantic contract

The registry document does not replace any of those grammar files.

Conversely, none of those grammar files may silently become a competing registry specification.


---

4. Registry Does Not Mean Runtime Hardware Registry

The word "registry" must not be interpreted as a machine inventory.

The Zamani dialect registry does not primarily register:

CPUs;

GPUs;

FPGAs;

ASICs;

QPUs;

physical qubits;

memory banks;

network nodes;

storage devices;

accelerator instances;

machine addresses;

deployment locations;

hardware serial numbers.


Those belong to target, hardware, resource, deployment, and runtime systems.

The dialect registry registers language contracts.

Therefore:

Dialect Registry
    =
language-contract registry

not:

Dialect Registry
    =
hardware inventory


---

5. Fundamental Registry Invariant

A registry entry describes:

> what a dialect is and what language contract it provides.



It does not describe:

> which machine must execute it.



For example:

quantum::standard

may identify a quantum language contract.

It does not mean:

use QPU 0

or:

use 32 physical qubits

or:

use topology X

or:

use vendor Y

Those decisions belong downstream.


---

6. Open-World Registry

The registry MUST NOT use a closed enumeration such as:

dialect =
    quantum
  | classical
  | cuda
  | openqasm
  | verilog
  | vendor_a
  | vendor_b

That design would make the language dependent on today's technologies.

Instead:

dialect identity
    =
symbolic qualified name

Examples:

quantum::standard
quantum::openqasm
classical::numeric
classical::parallel
hdl::rtl
hardware::programmable_logic
ai::tensor
distributed::messaging
networking::streaming
security::zero_knowledge
future::computing::extension
organization::research::dialect
vendor::domain::extension

These are examples only.

They are not automatically reserved names.

The registry determines whether a name is registered.


---

7. Registry Identity

Every dialect registry entry MUST have a stable identity.

The identity consists conceptually of:

namespace
+
dialect name
+
dialect version

The logical identity can therefore be represented as:

DialectId {
    namespace
    name
    version
}

The exact Rust representation belongs to the implementation layer.

The grammar must not dictate a Rust struct representation.


---

8. Identity Stability

A dialect identity MUST remain stable across compatible releases.

Changing:

quantum::standard

into:

quantum::standard_v2

must not be used merely because the implementation has changed internally.

Semantic breaking changes belong to the versioning and compatibility model.

Identity and implementation version are distinct concepts.


---

9. Namespace Ownership

Namespaces are owned by:

grammar/dialects/namespaces.g4

and the canonical core name system.

The registry consumes namespace identities.

The registry does not redefine qualified-name syntax.

A registry entry therefore uses symbolic identity such as:

organization::domain::dialect

without taking ownership of the syntax used to parse that name.


---

10. Registry Entry Model

Every registry entry SHOULD conceptually contain the following information:

Dialect Registry Entry
├── identity
│   ├── namespace
│   ├── name
│   └── version
│
├── lifecycle
│   ├── status
│   ├── introduced
│   ├── deprecated
│   └── removed
│
├── ownership
│   ├── owner
│   └── maintainers
│
├── contract
│   ├── syntax
│   ├── semantics
│   ├── capabilities
│   ├── requirements
│   └── constraints
│
├── composition
│   ├── imports
│   ├── dependencies
│   ├── extensions
│   └── inherited dialects
│
├── compatibility
│   ├── compatible versions
│   ├── incompatible versions
│   ├── replacements
│   └── migrations
│
├── implementation
│   ├── grammar references
│   ├── AST mappings
│   ├── semantic mappings
│   └── IR mappings
│
├── provenance
│   ├── specification
│   ├── source
│   ├── version
│   └── integrity metadata
│
└── policy
    ├── stability
    ├── trust
    ├── experimental status
    └── compatibility policy

Not every field must be represented directly in source syntax.

The registry model may be constructed from several authoritative sources.


---

11. Registry Entry Does Not Mean All Fields Are Required in Source

The distinction between:

source syntax

and:

registry metadata

is mandatory.

A dialect declaration may provide only a subset of metadata.

The semantic registry may derive or validate additional metadata from:

the dialect specification;

grammar manifests;

package metadata;

compiler metadata;

feature manifests;

compatibility manifests;

signed registry metadata;

implementation descriptors.


The parser must not be burdened with every registry implementation concern.


---

12. Dialect Lifecycle

Every registered dialect MUST have an explicit lifecycle state.

The lifecycle vocabulary is:

proposed
experimental
stable
deprecated
retired

Optional repository-level states may include:

historical
draft
disabled
quarantined

The exact lifecycle state vocabulary must be centralized rather than independently invented by every dialect.


---

13. Lifecycle Meaning

proposed

The dialect has been designed but is not yet a supported stable contract.

It must not silently become production syntax.


---

experimental

The dialect may be implemented and tested, but semantic compatibility is not guaranteed across releases.

Experimental status must remain visible to tooling.


---

stable

The dialect has a defined:

syntax contract;

AST contract;

semantic contract;

compatibility contract;

implementation contract;

test contract.


Stable does not mean immutable.

It means changes must follow the language compatibility process.


---

deprecated

The dialect remains recognized but users should migrate away from it.

The registry must preserve:

deprecation reason;

deprecation version;

replacement;

migration guidance.



---

retired

The dialect is no longer part of the active language surface.

Retired entries may remain in historical registries for provenance and migration tooling.


---

14. Dialect Versioning

Dialect versioning is separate from Zamani language versioning.

For example:

Zamani language version
        !=
quantum dialect version

A registry entry therefore needs to preserve both concepts where applicable.

Version syntax is owned by:

grammar/core/versioning.g4

Dialect-specific adaptation is owned by:

grammar/dialects/versioning.g4

The registry owns neither version parsing nor version comparison algorithms.


---

15. Version Semantics

The registry semantic layer must be able to answer:

Does requested dialect version V satisfy the required contract?

It may therefore perform:

exact version matching;

range matching;

constraint intersection;

compatibility evaluation;

migration selection;

deprecation evaluation.


The grammar does not perform these operations.


---

16. Version Independence From Hardware

A dialect version MUST NOT encode:

CPU generation;

GPU generation;

FPGA family;

QPU generation;

memory size;

number of cores;

number of qubits;

number of devices;

topology;

calibration version;

scheduler version.


Those are target properties.

A dialect version identifies a language contract.


---

17. Dialect Dependencies

A dialect may depend on other dialects.

Conceptually:

dialect A
    |
    +--> dialect B
    |
    +--> dialect C

Dependencies may be:

required;

optional;

capability-based;

version-constrained;

compatibility-constrained.


The registry semantic layer resolves them.


---

18. Dependency Graph

The registry must model dialect dependencies as a graph.

It must not assume:

maximum dependency depth = N

or:

maximum number of dependencies = N

The implementation may impose resource budgets for protection against pathological input, but those budgets are implementation policy, not language semantics.


---

19. Dependency Cycles

Cycles are not a parser concern.

The registry resolver must detect:

A -> B -> C -> A

and produce a structured diagnostic.

The diagnostic should identify:

the cycle;

each participating dialect;

relevant versions;

source locations where declarations caused the dependency;

possible remediation.



---

20. Dialect Composition

Dialects may compose other dialects.

Composition may include:

imports
extends
requires
provides
uses
extensions
compatibility contracts

The registry must preserve the distinction between these relationships.

It must not collapse all of them into a generic dependency.


---

21. Import vs Dependency

An import is source-level syntax.

A dependency is a semantic relationship.

Therefore:

import A

does not necessarily mean:

A is semantically required by the entire program

The semantic layer determines the actual dependency graph.


---

22. Extension Ownership

A dialect may expose extension points.

Extension-point syntax is owned by:

grammar/dialects/extension-points.g4

An extension must identify:

owning dialect;

extension point;

extension identity;

version;

compatibility;

semantic contract;

capabilities;

requirements.


An extension must not silently modify core Zamani semantics.


---

23. Extension Isolation

An extension belongs to the namespace and lifecycle of its owning dialect.

For example:

vendor::example::quantum

must not automatically create a new universal Zamani keyword.

Vendor-specific functionality remains explicitly scoped.


---

24. Vendor Dialects

Vendor dialects are allowed.

The registry MUST NOT enumerate vendors.

This permits future additions without changing the universal grammar.

For example:

vendor::organization::accelerator

is structurally valid as a symbolic identity.

The registry determines whether that dialect exists and whether it is trusted/usable.


---

25. Vendor Neutrality

The core Zamani language must not become dependent on vendor dialects.

A vendor dialect may provide:

optimizations;

optional operations;

target-specific facilities;

interoperability;

deployment hints;

specialized capabilities.


It must not redefine the meaning of stable core Zamani constructs.


---

26. Experimental Dialects

Experimental dialects may exist without being stable.

They must be explicitly marked.

The registry must preserve experimental status so that:

tooling can warn;

compatibility analysis can identify instability;

compilers can enforce policy;

migration tooling can distinguish experimental contracts;

stable programs do not accidentally acquire experimental dependencies.



---

27. Capability Contract

Dialect capabilities are symbolic.

Examples:

quantum::measurement
quantum::dynamic_control
classical::vector_execution
classical::parallel
hdl::sequential_logic
hardware::programmable_logic
distributed::messaging
ai::tensor_compute
networking::streaming
security::zero_knowledge

These are examples only.

The registry must not enumerate the complete universe of capabilities.


---

28. Capability Provider Model

A dialect may provide capabilities:

dialect
    |
    +--> provides capability A
    +--> provides capability B

A program may require capabilities:

program
    |
    +--> requires capability A

The registry may establish that:

dialect -> provides A

but it does not establish that:

hardware -> currently provides A

Hardware capability resolution is downstream.


---

29. Capability vs Hardware

This distinction is mandatory.

language capability
        !=
hardware capability

For example:

requires quantum::dynamic_control

means that the program requires a semantic facility.

It does not mean:

select QPU X

or:

select physical qubits 0..N


---

30. Resource Independence

The registry MUST NOT define universal hardware capacities.

Forbidden registry semantics include:

MAX_QUBITS
MAX_CPUS
MAX_CORES
MAX_THREADS
MAX_GPUS
MAX_FPGAS
MAX_QPUS
MAX_NODES
MAX_MEMORY
MAX_STORAGE
MAX_REGISTER_WIDTH
MAX_TENSOR_RANK
MAX_DEVICE_COUNT
MAX_NETWORK_SIZE

The registry must remain valid whether the eventual target has:

one resource

or:

an arbitrarily large resource population supported by the environment.


---

31. Requirements

A dialect may declare semantic requirements.

Examples:

requires quantum::measurement
requires quantum::dynamic_control
requires hardware::programmable_logic
requires distributed::messaging
requires classical::parallel

Requirements are semantic prerequisites.

They are not machine selectors.


---

32. Constraints

Constraints describe conditions under which a dialect realization is valid.

They may concern:

semantic compatibility;

supported capabilities;

execution model;

timing;

communication;

numerical behavior;

memory model;

security;

reliability;

portability.


Physical resource constraints belong to the resource and target layers.


---

33. Preferences

Preferences are advisory.

Examples include conceptual preferences for:

prefer classical acceleration
prefer quantum execution
prefer distributed execution
prefer low-latency realization

A preference must never silently become a mandatory target dependency.


---

34. Hints

Hints are weaker than requirements.

A hint may influence:

optimization;

target selection;

scheduling;

placement;

lowering.


A hint must not change the program's semantic meaning.


---

35. Requirement / Constraint / Preference / Hint Separation

The semantic model must preserve these four categories:

Category	Meaning

Requirement	Necessary semantic condition
Constraint	Forbidden realization condition
Preference	Desired realization if practical
Hint	Advisory implementation information


The registry must not collapse them into a generic property.


---

36. Compatibility

Compatibility is a relationship between language contracts.

The registry may represent:

compatible
incompatible
supports
requires
replaces
supersedes
migrates-to
deprecated-by
equivalent-to

The exact syntax is owned by:

grammar/dialects/compatibility.g4

The registry owns the semantic interpretation.


---

37. Compatibility Must Be Directional Where Necessary

Not every compatibility relationship is symmetric.

For example:

A supports B

does not necessarily imply:

B supports A

Likewise:

A replaces B

does not imply:

B replaces A

The registry semantic model must preserve relationship direction.


---

38. Compatibility Resolution

The registry resolver must be capable of determining:

requested dialect
        +
requested version
        +
current language version
        +
dependencies
        +
compatibility declarations
        |
        v
resolved dialect contract

Failure must produce structured diagnostics.

Silent fallback is prohibited unless explicitly allowed by the language compatibility policy.


---

39. Migration

A dialect may provide migration metadata.

Migration metadata can identify:

source dialect/version;

destination dialect/version;

deprecated feature;

replacement feature;

migration rules;

compatibility conditions.


The registry records the contract.

The migration engine performs the migration.

The grammar does not execute migration.


---

40. Deprecation

A registry entry may become deprecated.

Deprecation metadata should preserve:

deprecated_since
replacement
migration
reason
removal_policy

The registry must retain enough information for tooling to produce useful diagnostics.


---

41. Retired Dialects

Retirement does not require historical data to disappear.

A retired dialect may remain in a historical registry so that:

old source can be diagnosed;

migration can be performed;

provenance can be preserved;

reproducible builds can identify historical dependencies.


Retired does not mean "pretend it never existed."


---

42. Registry Provenance

Every registry entry should have provenance sufficient to determine:

where the contract came from
which version produced it
which specification defines it
which implementation claims it

Conceptually:

provenance
├── specification
├── grammar
├── implementation
├── version
└── integrity


---

43. Specification Reference

A stable dialect must reference its normative specification.

Conceptually:

specification:
    grammar/specification/...

or another repository-approved specification identifier.

The exact path is metadata.

The semantic requirement is that the registry entry be traceable to a normative contract.


---

44. Grammar Reference

A registry entry may identify the grammar components implementing its syntax.

For example:

grammar/dialects/registration.g4
grammar/dialects/extension-points.g4

The registry must not assume that a dialect has exactly one grammar file.

A dialect may be composed from multiple grammar fragments.


---

45. AST Contract

Every stable dialect must define how its syntax maps to the domain-neutral frontend AST.

The registry must be able to identify:

grammar rule
        |
        v
AST construct

The AST must remain domain-neutral at the common frontend boundary.

Vendor-specific or hardware-specific details must not contaminate universal AST structures merely because a dialect exists.


---

46. Semantic Contract

Every stable dialect must identify its semantic interpretation.

The semantic contract determines:

names;

types;

effects;

capabilities;

resource requirements;

ownership;

validation;

correctness;

portability;

domain meaning.


The parser must not implement these semantics.


---

47. IR Contract

A dialect must identify its canonical IR destination where applicable.

The registry must not create a new IR simply because a dialect exists.

For quantum:

quantum dialect
      |
      v
domain-neutral AST
      |
      v
semantic quantum model
      |
      v
quantum::ir

quantum::ir remains the canonical quantum boundary.


---

48. Classical IR

Classical dialects must lower through the canonical classical semantic/IR architecture.

The registry must not invent:

dialect-specific classical IR

unless a genuinely independent semantic boundary has been formally approved.


---

49. HDL / Hardware IR

HDL and hardware dialects must identify their appropriate semantic/IR destination.

The registry must preserve the distinction between:

software semantics
hardware intent
physical realization

It must not collapse them into one representation.


---

50. Quantum Integration

Quantum dialect registration must support:

quantum operations;

quantum types;

measurement;

dynamic control;

quantum/classical interaction;

logical operations;

resilience requirements;

capability requirements.


It must not define:

physical qubit IDs;

coupling maps;

calibration data;

physical gate timing;

QEC algorithms;

routing algorithms;

schedules.


Those belong downstream.


---

51. Quantum Canonical Boundary

The invariant is:

Quantum dialect
      |
      v
Zamani AST
      |
      v
semantic quantum model
      |
      v
quantum::ir
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
QEC / resilience / ZQN
      |
      v
HAL
      |
      v
target

No second quantum IR may be introduced by the registry.


---

52. HDL Integration

HDL dialects may describe language facilities for:

modules;

ports;

signals;

nets;

registers;

clocks;

timing;

pipelines;

memories;

state machines;

verification;

synthesis intent.


The registry only identifies the dialect contract.

It does not impose:

32-bit
64-bit
128-bit
N ports
N registers
N pipeline stages

unless such values are explicit program semantics.


---

53. Distributed Computing Integration

Distributed dialects may provide contracts for:

messaging;

services;

replication;

consistency;

partitioning;

fault tolerance;

collective operations.


The registry must not encode:

MAX_NODES
MAX_SERVICES
MAX_CHANNELS
MAX_NETWORK_SIZE

or fixed network topology.


---

54. AI and Data Integration

AI/data dialects may provide:

tensor semantics;

model semantics;

datasets;

training;

inference;

differentiation;

pipelines;

agents;

data streams.


The registry must not impose:

MAX_TENSOR_RANK
MAX_MODEL_SIZE
MAX_DATASET_SIZE
MAX_ACCELERATORS

Those are implementation/resource concerns.


---

55. Embedded and Nano Integration

The registry must permit future dialects for:

embedded systems;

nano computing;

atomic-scale computing;

molecular systems;

future computational substrates.


The grammar must not contain a finite catalogue of such technologies.


---

56. Sankofa Integration

Sankofa-related facilities may be represented through dialects where appropriate.

The registry must treat such functionality as language/semantic contracts.

It must not make the parser itself maintain:

memory;

history;

learning state;

consensus state;

temporal state.


Those belong to semantic/runtime subsystems.


---

57. Multi-Timeline Integration

If multi-timeline functionality is represented by a dialect, the registry must treat:

timeline
branch
fork
merge
observation
rewind
speculation

as semantic facilities.

It must not impose:

MAX_TIMELINES
MAX_BRANCHES
MAX_HISTORY


---

58. Dialect Feature Model

A dialect may expose features.

Each feature should have a stable identity.

Conceptually:

Dialect
    |
    +-- Feature A
    +-- Feature B
    +-- Feature C

A feature should have:

id
version
status
syntax
semantics
capabilities
requirements
compatibility
tests

This aligns the dialect registry with the repository-wide feature-manifest strategy.


---

59. Feature Manifests

Where machine-readable feature manifests are introduced, the registry should consume them rather than duplicating their complete content.

Conceptually:

feature manifest
       |
       v
registry validation
       |
       v
dialect registry

The registry should verify that the feature manifest agrees with:

grammar;

specification;

AST;

semantic model;

IR;

tests.



---

60. Registry and grammar.md

grammar/grammar.md is the implementation-conformance reference.

It must report whether dialect functionality is:

SPECIFIED
IMPLEMENTED
PARTIALLY IMPLEMENTED
PLANNED
DEPRECATED

The registry must not become a second implementation-status authority.

The registry describes the dialect contract.

grammar.md reports implementation conformance.


---

61. Registry and Zamani-Grammar.md

grammar/Zamani-Grammar.md remains historical/extended design material.

A dialect appearing there does not automatically become registered.

The promotion path remains:

Zamani-Grammar.md
        |
        v
feature proposal
        |
        v
semantic design
        |
        v
AST contract
        |
        v
canonical grammar
        |
        v
implementation
        |
        v
IR contract
        |
        v
tests
        |
        v
stable registry entry

This prevents aspirational syntax from silently becoming language authority.


---

62. Registry and DESIGN.md

grammar/DESIGN.md remains the higher-level architectural authority.

This file defines the specific registry contract under that architecture.

If a future registry requirement conflicts with DESIGN.md, the architecture must be reconciled before implementation.

No registry feature may silently override DESIGN.md.


---

63. Registry and Specification

The normative language specification remains under:

grammar/specification/

The registry references that specification.

The registry does not replace it.

The separation is:

specification
    = what the language means

grammar
    = how source is parsed

registry
    = which dialect contracts exist and how they relate

implementation
    = how those contracts are realized


---

64. Registry and Compatibility

The compatibility grammar:

grammar/dialects/compatibility.g4

owns source syntax.

This registry owns the semantic registry relationship.

Therefore:

compatibility.g4
        |
        v
AST compatibility declarations
        |
        v
registry compatibility model
        |
        v
compatibility resolver

There must not be a second compatibility model hidden inside the registry document.


---

65. Registry and Versioning

Version syntax comes from:

grammar/core/versioning.g4

Dialect adaptation comes from:

grammar/dialects/versioning.g4

Registry semantics consume normalized version structures.

The registry must not parse version strings independently in Rust.

This avoids two different version languages.


---

66. Registry and Capabilities

Capability syntax is governed by:

grammar/core/capabilities.g4
grammar/dialects/capabilities.g4

The registry stores normalized capability identities and relationships.

Capability resolution is a semantic/compiler responsibility.

Hardware capability discovery remains outside the registry.


---

67. Registry and Resources

Resources belong to:

grammar/resources/

and the hardware/resource semantic model.

The registry may declare that a dialect has a requirement such as:

requires capability("quantum.measurement")

but it must not decide whether:

required_memory <= available_memory

or whether a particular QPU can satisfy a requirement.

That is target/resource analysis.


---

68. Registry and Compile

Compilation infrastructure under:

grammar/compile/

consumes resolved dialect contracts.

The compiler may use the registry to determine:

supported dialects;

versions;

feature availability;

lowering contracts;

compatibility;

capabilities.


The registry does not perform compilation.


---

69. Registry and Execution

Execution infrastructure under:

grammar/execution/

may consume:

dialect capabilities;

execution requirements;

resilience contracts;

runtime compatibility;

deployment metadata.


The registry does not schedule or execute programs.


---

70. Registry and Interoperability

Interoperability formats such as:

OpenQASM
QIR
HDL
WASM
foreign language interfaces

must be represented as interoperability contracts rather than silently becoming alternate Zamani languages.

The registry may identify a dialect/adapter contract for such interoperability.

The canonical Zamani semantic model remains authoritative.


---

71. Registry and Vendor Extensions

Vendor-specific interoperability must remain explicit.

The registry must be able to identify:

vendor identity
extension identity
version
compatibility
capabilities
lowering contract

without making the vendor a dependency of the universal grammar.


---

72. Registry Trust

Where registry entries are externally supplied, the semantic infrastructure should be able to distinguish:

trusted
untrusted
unknown
revoked
quarantined

Trust status is registry/security metadata.

It must not be confused with dialect lifecycle.

For example:

stable + untrusted

and:

experimental + trusted

are conceptually different states.


---

73. Registry Integrity

Registry entries should support integrity verification where the repository/package ecosystem requires it.

Integrity metadata may include:

content digest;

provenance;

signing metadata;

source identity;

registry version;

specification version.


The grammar itself must not implement cryptographic verification.


---

74. Registry Security

The registry must never execute registry content merely to resolve a dialect.

Registry resolution must not implicitly:

execute arbitrary code;

execute macros;

invoke plugins;

inspect hardware;

access secrets;

access the network.


Resolution should operate on declarative metadata.


---

75. Deterministic Resolution

Given identical:

source
language version
dialect declarations
registry snapshot
compatibility policy
resolution policy

resolution must be deterministic.

The registry must not select different dialect contracts because:

machine ordering changed;

network response ordering changed;

hardware changed;

process scheduling changed;

wall-clock time changed.


If multiple valid resolutions exist, the language policy must define deterministic selection or require explicit disambiguation.


---

76. Registry Snapshot

For reproducible builds, compilation should be able to identify the registry state used for resolution.

Conceptually:

Registry Snapshot
    |
    +-- registry version
    +-- dialect entries
    +-- compatibility data
    +-- provenance
    +-- integrity metadata

A build should be able to record which registry snapshot participated in compilation.

This supports reproducibility and long-term provenance.


---

77. Registry Evolution

The registry itself is versioned.

Registry evolution must not silently change the meaning of an existing stable dialect.

Changes must follow compatibility policy.

Registry version:

!=
Zamani language version

!=
dialect version

All three are distinct concepts.


---

78. Registry Lookup

A semantic registry lookup should conceptually support:

lookup(namespace, name)
lookup(namespace, name, version)
lookup(feature)
lookup(capability)
lookup_compatible(...)
lookup_dependency(...)
lookup_replacement(...)
lookup_migration(...)

These are implementation responsibilities.

They are documented here so that the semantic implementation has a closed contract.


---

79. Registry Resolution Order

Resolution should conceptually proceed as:

1. Parse identity
       |
2. Normalize identity
       |
3. Resolve namespace
       |
4. Locate candidate entries
       |
5. Apply requested version
       |
6. Apply lifecycle policy
       |
7. Resolve dependencies
       |
8. Resolve compatibility
       |
9. Resolve capabilities
       |
10. Validate extensions
       |
11. Validate semantic contract
       |
12. Produce resolved dialect model

Hardware realization occurs later.


---

80. No Target Selection During Registry Resolution

The registry resolver must not select:

CPU
GPU
FPGA
ASIC
QPU
simulator
cluster
device

It resolves language contracts.

Target selection belongs to compilation/target/resource infrastructure.

This is essential for POCO-REAF.


---

81. Resource Scaling

The registry must remain valid for:

tiny computation

through:

large computation

and through the largest computation supported by available resources.

The registry must not contain finite machine-size assumptions.

For example, these are semantic requirements:

requires qubits >= n
requires memory >= required_memory
requires capability("tensor.compute")
requires topology(...)
requires capability("gpu.compute")
requires capability("quantum.measurement")

The registry may preserve such requirements.

It must not replace them with constants such as:

MAX_QUBITS = 1024


---

82. Infinite / Unbounded Semantics

"Infinity" in the POCO-REAF requirement means:

> No arbitrary language-level finite ceiling is imposed merely because an implementation currently has a finite machine.



It does not mean that a physical computer has infinite resources.

Therefore:

language domain
    = open-ended

while:

execution environment
    = finite resources at any particular realization

The compiler/runtime determines whether the requested computation can be realized.


---

83. Program Meaning Must Not Change With Scale

A program must retain the same semantic meaning when realized on:

small machine
large machine
CPU
GPU
FPGA
ASIC
QPU
distributed system
future accelerator

provided the target satisfies the required semantic contract.

Scaling may change:

performance;

placement;

parallelism;

scheduling;

resource allocation;

optimization.


It must not arbitrarily change program semantics.


---

84. Registry Does Not Guarantee Universal Executability

POCO-REAF does not mean:

> every dialect runs on every physical machine.



Instead:

portable program
        |
        v
required capabilities
        |
        v
environment capability analysis
        |
        +--> satisfiable
        |
        +--> migration/lowering possible
        |
        +--> unsupported

If a target lacks a mandatory semantic capability, the compiler must report that fact rather than pretending the target supports it.


---

85. Capability Negotiation

Where the language supports negotiation, the registry can participate in:

required capability
        |
        v
candidate dialect facilities
        |
        v
compatible implementation

The registry does not negotiate hardware resources directly.

That is the responsibility of the resource/target layer.


---

86. Dialect Aliases

Aliases are source-level conveniences.

For example:

import organization::domain::dialect as d

The alias does not create a new dialect identity.

The registry continues to identify the original dialect by its canonical identity.


---

87. Registry Canonicalization

Registry identities should be canonicalized before semantic comparison.

Canonicalization must preserve meaning.

It may normalize:

namespace representation;

qualified-name structure;

version representation;

compatibility constraints.


It must not erase meaningful distinctions.


---

88. Name Collision Rules

Two dialects must not occupy the same canonical identity within the same registry namespace/version context.

If two declarations resolve to the same identity but have incompatible contracts, resolution must fail.

Silent replacement is prohibited unless explicitly defined by registry policy.


---

89. Shadowing

Local aliases may shadow names.

Canonical registry identities must not be shadowed.

For example:

alias local = organization::dialect

may affect source lookup.

It must not mutate the global identity:

organization::dialect


---

90. Registry Conflict Detection

The semantic registry layer must detect conflicts involving:

duplicate identities;

incompatible versions;

incompatible extensions;

conflicting capabilities;

contradictory requirements;

incompatible inherited dialects;

incompatible semantic contracts;

incompatible lowering contracts.


Conflict diagnostics must identify the participating entries.


---

91. Registry Error Model

Registry failures should be structured.

At minimum, diagnostics should distinguish:

UnknownDialect
UnknownVersion
DuplicateDialect
IncompatibleVersion
MissingDependency
DependencyCycle
CapabilityConflict
RequirementUnsatisfied
ExtensionConflict
DeprecatedDialect
RetiredDialect
UntrustedDialect
InvalidRegistryEntry
InvalidProvenance
InvalidCompatibility
MissingLowering
AmbiguousResolution

Exact Rust error names belong to the implementation layer.


---

92. Source Spans

Registry-related syntax diagnostics must preserve source spans.

The AST/semantic layer must be able to identify:

dialect name;

version;

dependency;

capability;

compatibility declaration;

extension declaration;

deprecation declaration.


The registry itself does not own source-span implementation.


---

93. Diagnostics Must Be Actionable

A registry error should identify:

what failed
where it failed
why it failed
which contract caused it
what alternatives exist

For compatibility failures, the diagnostic should ideally include:

requested version
available version(s)
compatibility rule
replacement/migration if available


---

94. Registry and Safe Rust

The registry implementation must compile under:

Rust 1.97
Rust 1.97.1
Rust 2021

and must use:

safe Rust only

No unsafe code is required by this contract.

The registry document must not require unsafe ABI assumptions.


---

95. Registry Data Structures

The implementation may use structures conceptually equivalent to:

Registry
RegistrySnapshot
DialectId
DialectVersion
DialectEntry
DialectDependency
DialectCapability
DialectRequirement
DialectExtension
DialectCompatibility
DialectMigration
DialectProvenance
DialectTrust
DialectLifecycle
ResolvedDialect
ResolutionError

These are semantic implementation concepts.

They are not required to have these exact Rust names.


---

96. Registry Immutability During Resolution

A registry snapshot used for compilation should be treated as immutable during a resolution operation.

This provides deterministic resolution.

Conceptually:

registry snapshot
       |
       +--> compilation
       |
       +--> resolution
       |
       +--> diagnostics

No concurrent mutation should change the result halfway through resolution.


---

97. Registry Concurrency

The registry implementation may be shared between compilation operations.

The implementation must use safe Rust concurrency primitives.

No unsafe synchronization mechanism is required.

The semantic contract must remain deterministic even when multiple compilation tasks resolve dialects concurrently.


---

98. Registry Caching

Caching is permitted.

Cached results must be keyed by all semantic inputs that affect resolution.

A cache must never cause stale dialect semantics to be returned for a different registry snapshot.

Conceptually:

cache key =
registry snapshot
+
dialect identity
+
requested version
+
compatibility policy
+
resolution policy

The exact implementation is outside this document.


---

99. Registry Persistence

A registry may be persisted to:

files;

package metadata;

build artifacts;

registry services;

signed metadata stores.


The source language must not depend on one particular persistence mechanism.


---

100. Registry and Network Access

The parser must never require network access.

A registry implementation may use a remote registry during dependency acquisition or package resolution, but:

network resolution
    !=
parsing

and:

network registry
    !=
language semantics

A reproducible build must be able to operate against a fixed registry snapshot.


---

101. Registry and Package Management

The dialect registry may integrate with Zamani's package/dependency infrastructure.

However:

package identity
    !=
dialect identity

A package may provide one or more dialects.

A dialect may be distributed by one or more packages subject to package policy.


---

102. Registry and Danga / Package Infrastructure

Where Danga/package infrastructure supplies dialect packages, it must provide the registry with normalized metadata.

The grammar remains independent of the package manager.

The dialect registry must not embed package-manager implementation details into source syntax.


---

103. Registry and Compilation Provenance

A successful compilation should be able to record:

Zamani language version
registry snapshot
resolved dialects
resolved dialect versions
compatibility decisions
lowering contracts

This allows a build to be reproduced or audited later.


---

104. Registry and Reproducibility

For deterministic builds:

same source
+
same compiler
+
same language specification
+
same registry snapshot
+
same dependencies
+
same compilation policy

must result in equivalent semantic resolution.

Target-specific optimization may still vary where the compilation policy explicitly permits target adaptation.


---

105. Registry and Target Adaptation

After registry resolution:

resolved dialect contract
        |
        v
semantic analysis
        |
        v
capability analysis
        |
        v
resource analysis
        |
        v
target selection
        |
        v
lowering

The registry is not responsible for target adaptation.


---

106. Registry and Optimization

Optimization must preserve the semantic contracts represented by resolved dialects.

A dialect may declare semantic requirements that optimization must respect.

The registry itself does not optimize.


---

107. Registry and Routing

Routing is downstream.

This is especially important for quantum and hardware dialects.

The registry may establish:

requires quantum::connectivity-aware-execution

but it does not perform:

logical qubit -> physical qubit

routing.


---

108. Registry and Scheduling

Scheduling is downstream.

A dialect may require timing semantics or capabilities.

The registry does not produce a schedule.


---

109. Registry and QEC

QEC is downstream.

A quantum dialect may declare a requirement for an error-correction capability.

The registry does not implement QEC.


---

110. Registry and ZQN

ZQN remains the canonical fault/noise semantic subsystem.

The registry may reference a capability or requirement associated with ZQN.

It must not duplicate ZQN semantics.


---

111. Registry and HAL

The HAL determines target-specific realization.

The registry must not contain HAL implementation details.

The boundary is:

dialect registry
      |
      v
semantic contract
      |
      v
capability/resource analysis
      |
      v
HAL


---

112. Registry and Future Computing

The registry must support future computational domains without modification to its core identity model.

A future domain may introduce:

future::domain::dialect

with its own:

syntax;

semantics;

capabilities;

resources;

compatibility;

lowering.


The registry architecture remains unchanged.


---

113. Registry Does Not Enumerate Technology

The registry must not become a list of:

NVIDIA
AMD
Intel
IBM
Google
Rigetti
IonQ
Xilinx
AMD FPGA
specific ASIC
specific QPU
specific accelerator

unless those are explicit external registry entries.

They must not be hard-coded into the universal registry contract.


---

114. Registry Does Not Enumerate Algorithms

The registry must not become a universal list of:

FFT
SVD
GEMM
H
X
CNOT
gradient_descent
...

A dialect can provide semantic capabilities or extension contracts for such facilities.

The core registry remains generic.


---

115. Registry Does Not Enumerate AI Frameworks

The registry must not require:

TensorFlow
PyTorch
JAX
...

as universal language constructs.

Frameworks are interoperability/backend concerns.


---

116. Registry Does Not Enumerate HDL Vendors

Similarly, the registry must not require vendor-specific HDL toolchains as universal dialect identities.

Vendor integration remains explicit.


---

117. Registry Governance

A stable dialect must have an ownership model.

At minimum:

owner
maintainer
specification authority
implementation authority

The exact governance mechanism may evolve.

The registry must preserve enough information to determine who owns the semantic contract.


---

118. Registry Promotion Process

A new dialect should follow:

proposal
    |
    v
semantic design
    |
    v
specification
    |
    v
grammar
    |
    v
AST contract
    |
    v
semantic implementation
    |
    v
IR contract
    |
    v
compiler integration
    |
    v
tests
    |
    v
registry entry
    |
    v
experimental
    |
    v
stable

No dialect should become stable merely because its grammar parses.


---

119. Dialect Completion Contract

A dialect is complete only when all of the following exist:

[ ] identity
[ ] namespace
[ ] version
[ ] lifecycle
[ ] specification
[ ] grammar
[ ] lexer dependencies
[ ] AST contract
[ ] semantic contract
[ ] capability contract
[ ] requirement contract
[ ] compatibility contract
[ ] migration policy
[ ] deprecation policy
[ ] IR mapping
[ ] compiler integration
[ ] runtime integration where applicable
[ ] interoperability contract where applicable
[ ] positive tests
[ ] negative tests
[ ] boundary tests
[ ] scalability tests
[ ] compatibility tests
[ ] determinism tests
[ ] provenance
[ ] hard-coding audit
[ ] security review


---

120. Registry Entry Completion Contract

An individual registry entry is complete only when:

Identity
    +
Specification
    +
Syntax
    +
AST
    +
Semantics
    +
Capabilities
    +
Requirements
    +
Compatibility
    +
IR
    +
Tests
    +
Provenance

are all defined.

This satisfies the independent-file requirement:

> When a registry contract is marked complete, its integration points are already known and should not require redesign merely because another repository file is subsequently completed.




---

121. Hard-Coding Audit

Every registry-related change must be checked for prohibited universal constants.

The audit must search for:

MAX_QUBITS
MAX_CPUS
MAX_CORES
MAX_THREADS
MAX_GPUS
MAX_FPGAS
MAX_QPUS
MAX_NODES
MAX_MEMORY
MAX_STORAGE
MAX_REGISTER_WIDTH
MAX_TENSOR_RANK
MAX_DEVICE_COUNT
MAX_NETWORK_SIZE
MAX_DIALECTS
MAX_EXTENSIONS
MAX_CAPABILITIES
MAX_REQUIREMENTS
MAX_DEPENDENCIES
MAX_NAMESPACE_DEPTH

The absence of these identifiers alone is insufficient.

Equivalent hidden restrictions must also be rejected.


---

122. Hidden Hard-Coding

The following patterns are also prohibited as universal registry semantics:

dialectCount <= N
capabilityCount <= N
extensionCount <= N
dependencyCount <= N
namespaceDepth <= N
targetCount <= N
deviceCount <= N

Likewise, a finite enumeration disguised as:

switch dialectName {
    ...
}

must not become the universal semantic registry mechanism.

New dialects should be data-driven.


---

123. Data-Driven Registry

The preferred architecture is:

dialect metadata
        |
        v
registry
        |
        v
generic resolver

not:

dialect metadata
        |
        v
hard-coded compiler switch
        |
        +--> quantum
        +--> gpu
        +--> fpga
        +--> vendor_x

The latter prevents long-term extensibility.


---

124. Registry Extensibility

Adding a new dialect should normally require:

new dialect specification
new dialect grammar fragments
new semantic implementation
new tests
new registry entry

It should NOT require:

editing universal registry algorithms
adding another hard-coded match branch
adding a new hardware limit
adding another universal keyword
creating another root grammar

unless the new dialect genuinely requires a new language-wide semantic primitive.


---

125. Registry Validation

Registry validation should verify:

identity validity
version validity
namespace validity
dependency validity
dependency cycles
compatibility validity
capability references
requirement references
extension references
AST mappings
semantic mappings
IR mappings
provenance
lifecycle consistency
deprecation consistency
retirement consistency
security policy


---

126. Registry Validation Must Be Separate From Parsing

The parser answers:

> Is this source structurally valid?



The registry validator answers:

> Does this dialect contract make semantic sense?



The compiler answers:

> Can this resolved contract be realized for this program and target?



These questions must remain separate.


---

127. Negative Registry Tests

Tests must include:

unknown dialect
unknown version
duplicate dialect
invalid version
invalid dependency
dependency cycle
incompatible dialect
missing capability
conflicting capability
invalid extension
invalid migration
invalid replacement
retired dialect
invalid provenance
ambiguous resolution


---

128. Boundary Tests

Boundary tests must cover:

single dialect
many dialects
deep dependency graphs
many capabilities
many requirements
many extensions
many compatibility declarations
large qualified names
large version components
large metadata sets

No test may define an arbitrary maximum as a language rule.


---

129. Scalability Tests

Scalability tests should increase input size according to available test resources.

The test should verify that:

larger valid registry

remains semantically equivalent to a smaller registry when the additional entries are irrelevant.

The test must not assert:

registry must stop at N entries


---

130. Determinism Tests

Given:

same registry snapshot
same source
same language version
same resolution policy

the resolver must produce equivalent results.

Repeated runs must not produce different dialect resolutions merely because of:

hash-map iteration order;

thread scheduling;

network ordering;

machine ordering.



---

131. Compatibility Tests

Compatibility tests must cover:

exact match
compatible range
incompatible range
migration available
migration unavailable
deprecated source
retired source
extension compatibility
dependency compatibility
transitive compatibility


---

132. Quantum Registry Tests

Quantum-specific registry tests should cover:

quantum dialect registration
quantum dialect versioning
quantum capability declaration
quantum capability requirement
dynamic-control capability
measurement capability
custom operation dialect
quantum/classical hybrid dialect
quantum interoperability
OpenQASM adapter
QIR adapter

The tests must not assume a fixed qubit count.


---

133. Classical Registry Tests

Classical tests should cover:

numeric dialect
parallel dialect
vector capability
tensor capability
scientific computing
symbolic computing
distributed classical dialect

Again, no fixed core/thread/device count may be assumed.


---

134. HDL Registry Tests

HDL tests should cover:

RTL dialect
hardware module dialect
timing capability
sequential logic capability
memory capability
verification capability
synthesis capability
co-design dialect

No universal register width or hardware capacity may be imposed.


---

135. Distributed Registry Tests

Tests should cover:

messaging dialect
service dialect
replication dialect
consistency dialect
fault-tolerance dialect
distributed execution dialect

No fixed node count is permitted.


---

136. AI/Data Registry Tests

Tests should cover:

tensor dialect
model dialect
training dialect
inference dialect
dataset dialect
differentiation dialect
pipeline dialect
agent dialect

No fixed tensor rank, model size, accelerator count, or dataset size is permitted.


---

137. Security Tests

Registry tests should verify that registry resolution:

does not execute arbitrary registry content;

does not access secrets;

does not silently load executable plugins;

preserves provenance;

respects trust state;

rejects revoked/quarantined entries according to policy.



---

138. No Unsafe Requirement

Nothing in this registry contract requires:

unsafe

The implementation must remain compatible with:

Rust 1.97
Rust 1.97.1
Rust 2021

and safe Rust.


---

139. Performance

Registry resolution should avoid unnecessary repeated work.

Appropriate implementation techniques may include:

immutable snapshots;

canonicalized identities;

memoization;

dependency graph caching;

compatibility-result caching.


Performance optimizations must not alter semantic results.


---

140. Memory Scaling

Registry implementations must avoid fixed-size arrays as semantic limits.

Dynamic collections should be used where appropriate.

Practical memory exhaustion is an implementation/resource failure, not a language-level maximum.


---

141. Error Recovery

Registry failures should not corrupt unrelated compilation state.

The compiler should be able to report multiple independent registry diagnostics where practical.

For example:

unknown dialect A
missing capability B
deprecated dialect C

may be reported together if semantic analysis can continue safely.


---

142. Registry and Tooling

Language servers and tooling may consume registry metadata for:

completion;

documentation;

diagnostics;

navigation;

migration;

deprecation warnings;

capability information;

compatibility information.


Tooling must not invent dialect semantics independently of the registry.


---

143. Registry and Documentation

Documentation generation may derive:

dialect reference
feature reference
capability reference
version reference
compatibility reference

from registry/specification metadata.

Generated documentation is not a new semantic authority.


---

144. Registry and Source Formatting

A formatter may preserve dialect declarations and metadata.

Registry semantics must not depend on formatting.

Equivalent source representations must resolve to equivalent dialect contracts.


---

145. Registry and Macros

Macros may refer to dialect facilities subject to normal semantic validation.

Macros must not bypass registry compatibility or lifecycle checks.

A macro-generated dialect reference is still a dialect reference.


---

146. Registry and Metaprogramming

Compile-time reflection may inspect registered dialect metadata where explicitly permitted.

It must not mutate the semantic registry arbitrarily during compilation.

Registry mutation during a compilation unit would compromise deterministic resolution.


---

147. Registry and Modules

Dialect declarations may be associated with modules.

The module system remains responsible for:

scope;

visibility;

import/export;

module identity.


The registry remains responsible for dialect identity and contract resolution.


---

148. Registry and Security Policies

Security policy may restrict which dialects may be used.

For example:

trusted-only policy
stable-only policy
no-vendor policy
no-experimental policy

These are policies applied to the registry.

They do not change the underlying dialect identity.


---

149. Registry and Build Profiles

Compilation profiles may choose policies such as:

development
experimental
production
reproducible
restricted
embedded
quantum
HPC

The registry may be queried under those policies.

The registry itself must remain policy-neutral unless a policy is explicitly part of its resolution input.


---

150. Registry and POCO-REAF

The registry is one of the mechanisms enabling:

Program Once
      |
      v
Portable semantic contract
      |
      v
Compile Once
      |
      v
Resolved dialect semantics
      |
      v
Capability/resource adaptation
      |
      v
Target realization
      |
      v
Run Everywhere
      |
      v
Run Anywhere
      |
      v
Run Forever

The registry must therefore preserve semantic portability.


---

151. Meaning of "Forever"

"Forever" requires preservation of:

stable identities;

semantic versioning;

compatibility metadata;

migration paths;

provenance;

historical entries;

explicit retirement.


It does not require every old implementation to execute every future program.


---

152. Registry and Long-Term Compatibility

A stable dialect should be designed so that future compiler versions can determine:

what the dialect meant
which version was used
what replaced it
whether migration exists
what compatibility guarantees applied

This is a key part of POCO-REAF.


---

153. Registry and Canonical Semantic Model

The registry must never become the canonical semantic model itself.

It provides metadata about semantic contracts.

The actual semantics remain owned by:

language semantic model
domain semantic models
canonical IRs


---

154. Registry and Quantum IR Invariant

The following invariant is absolute:

Dialect Registry
      |
      v
Dialect Contract
      |
      v
AST
      |
      v
Semantic Analysis
      |
      v
quantum::ir

Never:

Dialect Registry
      |
      v
Dialect-specific Quantum IR
      |
      v
quantum::ir

unless a formally approved intermediate representation is required for a separate compiler stage.

No second canonical quantum IR may be created.


---

155. Registry and Hardware Independence Invariant

The registry must remain valid regardless of whether the eventual target is:

atom-scale
nano
embedded
CPU
multicore
GPU
FPGA
ASIC
QPU
HPC
cluster
cloud
edge
future hardware

The dialect contract describes semantics.

The environment determines realization.


---

156. Repository Integration Matrix

The registry contract integrates with the repository as follows:

File / Directory	Registry relationship

grammar/DESIGN.md	Higher-level architecture
grammar/README.md	Navigation and authority
grammar/Zamani.g4	Root composition
grammar/grammar.md	Implementation conformance
grammar/Zamani-Grammar.md	Historical/extended design
grammar/dialects/dialect.g4	Public dialect boundary
grammar/dialects/dialects.g4	Existing dialect facade
grammar/dialects/registration.g4	Registration syntax
grammar/dialects/namespaces.g4	Namespace syntax
grammar/dialects/versioning.g4	Version adapter
grammar/dialects/capabilities.g4	Capability adapter
grammar/dialects/compatibility.g4	Compatibility syntax
grammar/dialects/extension-points.g4	Extension-point syntax
grammar/dialects/vendor.g4	Vendor extensions
grammar/dialects/experimental.g4	Experimental extensions
grammar/core/names.g4	Canonical names
grammar/core/versioning.g4	Canonical version syntax
grammar/core/capabilities.g4	Canonical capability syntax
grammar/resources/	Resource semantics
grammar/hardware/	Hardware capabilities
grammar/compile/	Target compilation
grammar/execution/	Runtime/execution
grammar/compatibility/	Compatibility documentation
grammar/validation/	Validation contracts
grammar/tests/	Conformance tests
src/frontend/ast/	Domain-neutral AST
semantic analysis	Dialect resolution
quantum::ir	Canonical quantum semantic boundary
compiler	Lowering/target realization
runtime/HAL	Actual environment realization



---

157. Required Repository Corrections

The registry contract exposes several integration requirements that must be completed elsewhere.

157.1 One Dialect Public Boundary

The repository currently contains both:

dialects.g4
dialect.g4

These must not become competing public dialect grammars.

The final architecture should establish one canonical public boundary.

The other file may remain as a compatibility facade if existing tooling requires it, but it must delegate rather than duplicate semantics.


---

157.2 Registration Must Be Canonical

registration.g4 remains the source-level registration grammar.

The registry document must not be copied into another grammar.


---

157.3 Compatibility Must Have One Owner

compatibility.g4 must remain the source-level compatibility syntax owner.

Compatibility constructs currently duplicated elsewhere must be routed through that canonical contract.


---

157.4 Versioning Must Have One Core Owner

Dialect versioning must adapt:

grammar/core/versioning.g4

rather than creating a second independent version language.


---

157.5 Capability Syntax Must Have One Core Owner

Dialect capability grammar must adapt the canonical capability model.

No dialect-specific capability syntax should redefine the core capability language.


---

158. Required AST Mapping

The dialect frontend should map into domain-neutral constructs conceptually including:

DialectDeclaration
DialectReference
DialectImport
DialectUse
DialectDependency
DialectExtension
DialectCapability
DialectRequirement
DialectConstraint
DialectPreference
DialectHint
DialectVersion
DialectCompatibility
DialectMigration
DialectDeprecation
DialectProvenance
DialectLifecycle

These are conceptual contracts.

Exact Rust AST names must be reconciled with the existing src/frontend/ast/ model rather than blindly creating duplicate nodes.


---

159. AST Domain-Neutrality

The AST must not contain:

NvidiaDialectNode
QpuVendorNode
PhysicalQubitDialectNode
FpgaDeviceDialectNode

as universal nodes merely because a dialect exists.

Vendor and target-specific information should remain attached to generic extension/metadata structures unless a domain semantic model explicitly owns it.


---

160. Semantic Mapping

The semantic layer consumes the registry to produce:

resolved dialect set
resolved versions
resolved dependencies
resolved capabilities
resolved requirements
resolved compatibility
resolved extensions
resolved lifecycle state

The result feeds normal semantic analysis.


---

161. IR Mapping

The registry does not directly generate IR.

Instead:

registry
    |
    v
resolved semantic dialect contract
    |
    v
semantic analysis
    |
    v
canonical IR

This keeps the registry independent from backend representation.


---

162. Compiler Integration

The compiler must be able to query:

Is dialect X available?
Which version is resolved?
What capabilities does it provide?
What does it require?
Which features are stable?
Which features are deprecated?
Which lowering contract applies?

The compiler then performs target-specific analysis.


---

163. Runtime Integration

The runtime may receive resolved dialect metadata as part of deployment metadata.

It must not reinterpret source syntax.

Runtime validation may verify:

required capability
required semantic contract
required runtime facility

against the actual environment.


---

164. Target Adaptation

The final chain is:

Dialect
   |
   v
Registry
   |
   v
Semantic contract
   |
   v
Capability analysis
   |
   v
Resource analysis
   |
   v
Target selection
   |
   v
Lowering
   |
   v
Optimization
   |
   v
Routing / Scheduling / Resilience
   |
   v
HAL
   |
   v
Target

The registry must not skip directly from:

dialect

to:

physical device


---

165. Production Readiness Checklist

grammar/dialects/registry.md is considered complete when:

[x] registry purpose defined;

[x] registry authority defined;

[x] grammar boundary defined;

[x] semantic boundary defined;

[x] registry is open-world;

[x] identity model defined;

[x] namespace ownership defined;

[x] version ownership defined;

[x] capability ownership defined;

[x] compatibility ownership defined;

[x] lifecycle defined;

[x] provenance defined;

[x] trust model defined;

[x] dependency model defined;

[x] extension model defined;

[x] vendor model defined;

[x] experimental model defined;

[x] migration model defined;

[x] deprecation model defined;

[x] retirement model defined;

[x] deterministic resolution defined;

[x] registry snapshot defined;

[x] reproducibility defined;

[x] security boundaries defined;

[x] AST integration defined;

[x] semantic integration defined;

[x] IR integration defined;

[x] quantum::ir boundary protected;

[x] compiler integration defined;

[x] runtime integration defined;

[x] hardware boundary defined;

[x] resource boundary defined;

[x] POCO-REAF preserved;

[x] scalability requirements defined;

[x] hard-coding prohibition defined;

[x] diagnostics defined;

[x] testing requirements defined;

[x] safe Rust requirement defined.



---

166. Definition of Registry Completion

The registry subsystem is complete only when the following chain is executable without architectural ambiguity:

Dialect Source
      |
      v
Lexer
      |
      v
Parser
      |
      v
Domain-Neutral AST
      |
      v
Registry Resolution
      |
      +------------------+
      |                  |
      v                  v
Version Resolution   Dependency Resolution
      |                  |
      +--------+---------+
               |
               v
       Compatibility
               |
               v
       Capability Model
               |
               v
       Requirement Model
               |
               v
       Semantic Dialect Set
               |
               v
       Canonical Semantic Model
               |
       +-------+--------+
       |       |        |
       v       v        v
  Classical quantum   HDL/
     IR      ::ir    Hardware IR
       |       |        |
       +-------+--------+
               |
               v
        Compiler Pipeline
               |
       +-------+--------+
       |       |        |
       v       v        v
    Routing Scheduling Resilience
       |       |        |
       +-------+--------+
               |
               v
              ZQN
               |
               v
              HAL
               |
               v
       Target Realization


---

167. Final Registry Invariants

The following invariants are mandatory.

Invariant 1 — One language

Dialects extend Zamani.

They do not create unrelated root languages.

Invariant 2 — Open world

New dialects do not require modification of universal registry algorithms.

Invariant 3 — No artificial limits

No arbitrary machine/resource/dialect limits are encoded as language semantics.

Invariant 4 — Target independence

Dialect identity does not select hardware.

Invariant 5 — Semantic separation

Registry metadata does not replace semantic analysis.

Invariant 6 — Canonical IR

Dialect registration does not create competing IRs.

Invariant 7 — Quantum boundary

quantum::ir remains the canonical quantum semantic boundary.

Invariant 8 — Determinism

Equivalent inputs and registry snapshots produce equivalent resolutions.

Invariant 9 — Provenance

Stable dialect contracts remain traceable.

Invariant 10 — Compatibility

Version and compatibility relationships are explicit.

Invariant 11 — Safe Rust

Registry implementation requires no unsafe Rust.

Invariant 12 — POCO-REAF

Portable source semantics are separated from target realization.


---

168. Final Architectural Statement

The Zamani dialect registry is therefore:

NOT:

a list of hardware
a list of vendors
a list of GPUs
a list of QPUs
a list of CPUs
a list of HDL tools
a list of AI frameworks
a list of mathematical functions
a second grammar
a second AST
a second quantum IR
a compiler backend
a runtime
a scheduler
a router
a QEC engine
a ZQN engine

It is:

a registry of language contracts

whose purpose is to establish:

identity
version
lifecycle
ownership
dependencies
capabilities
requirements
extensions
compatibility
migration
provenance
semantic integration
IR integration
implementation status

while remaining independent of physical machine scale.

The fundamental relationship is:

ZAMANI
                       |
                 Portable Source
                       |
                       v
                 Dialect Registry
                       |
              Resolved Language Contract
                       |
                       v
               Semantic Analysis
                       |
          +------------+------------+
          |            |            |
      Classical     Quantum        HDL
          |            |            |
          |       quantum::ir      |
          +------------+------------+
                       |
                       v
                 Canonical IR
                       |
                       v
               Optimization
                       |
          +------------+------------+
          |            |            |
       Routing     Scheduling   Resilience
          |            |            |
          +------------+------------+
                       |
                      ZQN
                       |
                      HAL
                       |
                       v
              Target Realization
                       |
       +---------------+----------------+
       |       |       |       |        |
      CPU     GPU     FPGA    QPU     Future

Therefore the registry must preserve the central Zamani principle:

> The source program describes what computation means and what semantic capabilities and guarantees it requires. The registry describes which language contracts provide those semantics. The compiler and runtime determine how those contracts are realized on the resources actually available.



That separation is what allows Zamani to scale from the smallest meaningful computation to arbitrarily large computations supported by available resources while preserving:

Program Once → Compile Once → Run Everywhere → Run Anywhere → Forever.


---

169. File Ownership Summary

This file:

grammar/dialects/registry.md

owns:

registry architecture;

registry semantic contract;

dialect identity model;

lifecycle model;

dependency model;

compatibility relationship model;

registry provenance;

registry trust model;

registry resolution contract;

registry integration contract;

registry scalability policy;

registry hard-coding policy;

registry production-readiness criteria.


It does not own:

lexical tokens;

identifiers;

qualified-name syntax;

version syntax;

capability syntax;

dialect declaration syntax;

compatibility grammar syntax;

AST implementation;

semantic implementation;

IR implementation;

quantum IR;

hardware discovery;

resource allocation;

routing;

scheduling;

QEC;

ZQN;

HAL;

runtime execution.


Those remain owned by their existing repository files and subsystems.


---

170. Completion Rule

No future dialect should be considered production-ready merely because:

the parser accepts its syntax.

It is production-ready only when:

Specification
    +
Grammar
    +
AST
    +
Semantics
    +
Registry
    +
Capabilities
    +
Requirements
    +
Compatibility
    +
IR
    +
Compiler
    +
Runtime
    +
Tests
    +
Provenance
    +
Scalability audit
    +
Hard-coding audit

are all satisfied.

That is the complete dialect-registry contract for production Zamani.

