Zamani Migration Specification

Path: "grammar/compatibility/migrations.md"
Status: Normative
Language: Zamani
Repository: "Benwellonedge28/Zamani"
Rust baseline: Rust 1.97 / Rust 1.97.1
Rust edition: 2021
Rust safety requirement: Production Zamani Rust implementation MUST use safe Rust; Rust "unsafe" MUST NOT be required or used.
Primary objective: Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)
Scope: Source, lexical, grammar, parser, AST, semantic, type, effect, resource, capability, dialect, IR, quantum, HDL, artifact, ABI, runtime, tooling, and repository integration migrations.

---

1. Purpose

This document defines the normative migration system for Zamani.

It specifies how Zamani changes are introduced, transformed, validated, deprecated, removed, and carried across versions without unnecessarily breaking existing programs or creating parallel semantic architectures.

It covers migrations involving:

- source programs;
- identifiers;
- keywords;
- operators;
- literals;
- lexical rules;
- grammar rules;
- declarations;
- expressions;
- statements;
- types;
- generics;
- ownership;
- effects;
- modules;
- concurrency;
- classical computation;
- quantum computation;
- hybrid computation;
- HDL;
- hardware intent;
- resources;
- capabilities;
- distributed computation;
- AI/data computation;
- networking;
- security;
- interoperability;
- dialects;
- AST schemas;
- semantic representations;
- canonical IR;
- "quantum::ir";
- compiled artifacts;
- ABI contracts;
- runtime contracts;
- diagnostics;
- tooling;
- generated documentation;
- compatibility metadata.

The fundamental rule is:

«Migrate representations; preserve specified semantics.»

A migration MUST NOT use temporary implementation limitations, current hardware availability, backend limitations, or compiler convenience as justification for permanently reducing the language's scalable semantic model.

---

2. Normative Status

The following terms are normative:

- MUST — mandatory.
- MUST NOT — prohibited.
- REQUIRED — mandatory.
- SHOULD — recommended unless a documented technical reason exists otherwise.
- SHOULD NOT — discouraged unless justified.
- MAY — permitted.
- OPTIONAL — permitted but not required.

A migration is not complete merely because source text can be mechanically rewritten.

A migration is complete only when the resulting program retains the required semantic meaning and passes the applicable compatibility, semantic, scalability, and integration checks.

---

3. Ownership

This file owns migration procedure.

It does not become the owner of language version numbering, deprecation policy, grammar syntax, semantic definitions, or backend implementation.

The repository authority relationship is:

grammar/DESIGN.md
        │
        ▼
language specification
        │
        ├── grammar/specification/
        └── grammar/spec/
        │
        ▼
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
canonical semantic model
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
- authority hierarchy;
- ownership boundaries;
- composition rules;
- production-readiness principles;
- POCO-REAF architecture;
- scalability architecture.

It does not own individual migration records.

3.2 "grammar/compatibility/versions.md"

Owns:

- language-version policy;
- version numbering;
- compatibility classes;
- language/compiler version separation;
- release-level compatibility guarantees.

It answers:

«What does a version mean?»

This file answers:

«How is a change migrated?»

3.3 "grammar/compatibility/deprecated.md"

Owns:

- deprecation lifecycle;
- deprecation metadata;
- deprecation diagnostics;
- removal eligibility.

It answers:

«When and why is an existing feature deprecated?»

This file answers:

«How is the feature transformed or preserved during migration?»

3.4 "grammar/compatibility/compatibility-matrix.md"

Owns:

- compatibility relationships between versions and layers.

It answers:

«Which versions/layers are compatible?»

This file answers:

«What procedure must be performed to move between them?»

3.5 "grammar/spec/compatibility.md"

Owns compatibility concepts and dimensions.

3.6 "grammar/grammar.md"

Owns implementation-conformance reporting.

It MUST NOT silently become another normative language specification.

3.7 "grammar/Zamani.g4"

Is the canonical ANTLR grammar composition root.

It is a syntax representation.

It MUST NOT become a migration engine or semantic authority.

3.8 "grammar/Zamani-Grammar.md"

Remains the broad historical/extended/design reference.

A feature appearing there does not automatically become stable Zamani syntax.

3.9 "src/lexer.rs"

Owns executable lexical tokenization.

3.10 "src/parser.rs"

Owns executable source parsing for the reference frontend.

3.11 "src/ast/"

Owns structural source representation.

3.12 Semantic analysis

Owns interpretation of source meaning.

3.13 "quantum::ir"

Owns the canonical quantum semantic boundary.

Migration MUST NOT introduce a second permanent frontend quantum IR merely to preserve legacy quantum syntax.

---

4. Fundamental Migration Architecture

The preferred migration path is:

Old Source
    │
    ▼
Old Syntax
    │
    ▼
Compatibility / Migration Layer
    │
    ▼
Canonical Meaning
    │
    ▼
Current AST / Semantic Model
    │
    ▼
Canonical IR
    │
    ├───────────────┐
    ▼               ▼
Classical IR    quantum::ir
    │               │
    └───────┬───────┘
            ▼
      Optimization
            ▼
 Routing / Scheduling / Resilience / QEC / ZQN
            ▼
           HAL
            ▼
       Target Runtime

A migration MUST NOT create this architecture:

Old Syntax
    ▼
Old Permanent IR
    ▼
Old Compiler
    ▼
Old Runtime

unless an external artifact compatibility requirement explicitly requires it.

Historical syntax should normally terminate at the current semantic model.

---

5. Migration Invariants

Every migration MUST preserve all applicable invariants.

These include:

- identifier identity;
- binding relationships;
- scope;
- name resolution;
- type meaning;
- generic meaning;
- ownership;
- borrowing;
- lifetime semantics;
- evaluation order;
- control flow;
- effects;
- capabilities;
- resource requirements;
- resource constraints;
- concurrency semantics;
- synchronization;
- error behavior;
- numerical meaning;
- deterministic semantics;
- quantum state meaning;
- quantum operation ordering;
- parameter meaning;
- measurement semantics;
- reset semantics;
- classical feed-forward;
- entanglement semantics;
- logical/physical resource distinction;
- HDL behavioral meaning;
- hardware intent;
- timing constraints;
- distributed semantics;
- networking semantics;
- security properties;
- provenance;
- source spans where promised;
- diagnostics where compatibility requires them.

A migration MAY change representation.

It MUST NOT silently change meaning.

---

6. Compatibility Is Layered

Migration decisions MUST be made independently at each layer.

Source
  ↓
Lexical
  ↓
Syntax
  ↓
AST
  ↓
Name / Module
  ↓
Type
  ↓
Effect
  ↓
Capability
  ↓
Resource
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

A migration MAY therefore have different classifications at different layers.

For example:

Source:          automatic
Lexer:           automatic
Parser:          automatic
AST:             unchanged
Semantics:       unchanged
quantum::ir:     unchanged
Artifact:        incompatible
Runtime:         target-dependent

The entire change MUST NOT be incorrectly classified as universally breaking merely because one downstream representation changed.

Likewise, source compatibility MUST NOT be claimed merely because parsing still succeeds.

---

7. Migration Classes

Every migration MUST be classified.

Class| Name| Meaning
M0| No Migration| Representation remains compatible
M1| Transparent Compatibility| Old and new representations coexist without source transformation
M2| Automatic Mechanical| Deterministic source transformation
M3| Tool-Assisted| Tool transforms known cases and requests decisions for ambiguous cases
M4| Explicit Source Migration| Developer must change source
M5| Semantic Migration| Meaning/model itself changes and requires explicit semantic handling
M6| Artifact Migration| Compiled/serialized representation changes
M7| Dialect Migration| Dialect version or extension changes
M8| Target Migration| Target/backend/runtime representation changes
M9| Breaking Migration| Existing valid source or semantics can no longer be preserved under the promised compatibility contract

A single change MAY have multiple classifications.

Example:

Source syntax:      M2
AST:                M2
Semantic meaning:   M0
quantum::ir:        M0
Artifact:           M6
Target:             M8

---

8. Migration Decision Procedure

For every proposed change:

Does existing valid source still parse?
        │
        ├── YES
        │    │
        │    └── Does it retain the same specified meaning?
        │             │
        │             ├── YES → M0/M1
        │             │
        │             └── NO → M5
        │
        └── NO
             │
             └── Is there a deterministic semantics-preserving transformation?
                       │
                       ├── YES → M2/M3
                       │
                       └── NO
                            │
                            ├── explicit source decision → M4
                            │
                            └── incompatible semantics → M9

Parser failure alone MUST NOT determine whether a change is breaking.

Semantic preservation is the primary criterion.

---

9. Required Migration Record

Every non-trivial migration MUST have a migration record containing:

Migration ID:
Feature ID:
Migration Name:

Source Language Version:
Target Language Version:

Source Representation:
Target Representation:

Migration Class:

Affected Layer:
Affected Domain:

Old Syntax:
New Syntax:

Old AST:
New AST:

Old Semantic Model:
New Semantic Model:

Old IR:
New IR:

Quantum IR Impact:
HDL/HW Impact:

Capability Impact:
Resource Impact:

Semantic Preservation:
Semantic Changes:

Automatic Transformation:
Tool-Assisted Transformation:
Required Manual Action:

Diagnostics:
Source-Span Policy:

Compatibility Window:
Deprecation Status:
Removal Version:

Rollback / Recovery:
Artifact Impact:

Positive Tests:
Negative Tests:
Boundary Tests:
Scalability Tests:
Determinism Tests:
Compatibility Tests:

Affected Repository Files:
Downstream Consumers:

Feature Manifest:
Compatibility Matrix Entry:

Hard-Coding Audit:
Unsafe-Rust Audit:

Completion Status:

A migration lacking required fields MUST NOT be marked production-complete.

---

10. Migration Identity

Migration identity MUST be stable.

A migration ID MUST NOT depend solely on the textual spelling of a feature.

Preferred identity:

domain + semantic feature + migration sequence

For example:

quantum.operation.generic.v2

rather than:

apply_gate_to_q

This allows:

old spelling
    ↓
new spelling

without treating a spelling change as an entirely new semantic feature.

---

11. Source Migration

A source migration transforms one valid Zamani source representation into another.

The migration MUST preserve:

- program structure;
- semantic intent;
- types;
- effects;
- resources;
- capabilities;
- quantum meaning;
- HDL meaning;
- diagnostics where promised.

Where tooling promises formatting preservation, formatting MUST be preserved.

Where tooling promises comment preservation, comments MUST be preserved.

Where exact source-span preservation is impossible, migration tooling MUST explicitly document the new span mapping.

---

12. Automatic Migration

Automatic migration is allowed only where the transformation is deterministic.

Examples:

legacy_keyword → canonical_keyword

or:

legacy.module.path → canonical.module.path

provided the semantics are formally equivalent.

Automatic migration MUST NOT guess developer intent.

For example:

legacy_resource_model
        ↓
unknown_new_resource_model

MUST NOT be automatically transformed if multiple semantic interpretations are possible.

---

13. Tool-Assisted Migration

Tool-assisted migration MUST:

1. discover all affected constructs;
2. report their locations;
3. identify the migration rule;
4. show the proposed transformation;
5. distinguish deterministic transformations from decisions;
6. refuse unsafe semantic guessing;
7. produce machine-readable diagnostics;
8. provide validation before rewriting;
9. record migration provenance;
10. allow the resulting source to be re-parsed and re-validated.

A migration tool MUST distinguish:

AUTOMATICALLY TRANSFORMED

from:

REQUIRES DEVELOPER DECISION

---

14. Manual Migration

Manual migration is required when semantics cannot be inferred safely.

Examples include:

- changed ownership semantics;
- changed concurrency semantics;
- changed effect semantics;
- changed resource semantics;
- changed security guarantees;
- changed quantum measurement semantics;
- changed hardware intent;
- incompatible dialect semantics;
- removed information;
- multiple possible replacements.

Manual migration documentation MUST contain concrete before/after examples and explain the semantic difference.

---

15. Grammar Migration

A grammar reorganization is not automatically a language migration.

For example, moving rules from the historical monolithic "grammar/Zamani.g4" architecture into modular grammar domains is implementation-neutral when the accepted language and semantic meaning remain unchanged.

A grammar reorganization is compatible only if:

- accepted source meaning remains equivalent;
- tokenization remains equivalent or has a documented migration;
- precedence remains equivalent;
- associativity remains equivalent;
- AST meaning remains equivalent;
- diagnostics remain within documented compatibility guarantees.

The modular directory architecture MUST NOT cause duplicate language authorities.

---

16. "Zamani.g4" Migration

"grammar/Zamani.g4" remains the canonical ANTLR composition root.

Migration work MUST converge toward:

Zamani.g4
    │
    ├── lexical vocabulary
    ├── core syntax
    ├── declarations
    ├── statements
    ├── expressions
    ├── types
    ├── modules
    ├── functions
    ├── effects
    ├── memory
    ├── concurrency
    ├── classical
    ├── quantum
    ├── hybrid
    ├── HDL
    ├── hardware
    ├── resources
    ├── distributed
    ├── AI
    ├── data
    ├── networking
    ├── security
    ├── interoperability
    ├── dialects
    ├── macros
    └── metaprogramming

Historical syntax may remain during a compatibility window.

It MUST eventually converge onto the canonical semantic model.

No second permanent root grammar may be introduced.

---

17. Lexer Migration

The executable lexer is "src/lexer.rs".

The lexical specification is represented under:

grammar/spec/
grammar/lexer/

and the ANTLR representation under:

grammar/Zamani.g4

All three must converge.

The current lexer contains a broad vocabulary covering core language, OOP, quantum, nano, Sankofa, mathematical, effect, concurrency, and advanced concepts.

Therefore, adding additional keywords MUST NOT be the default migration strategy.

Before adding a keyword, migration analysis MUST determine whether the concept can be represented by:

- an identifier;
- a qualified name;
- an operation name;
- an attribute;
- a capability;
- a type;
- a generic;
- an effect;
- a dialect extension.

This protects the language from uncontrolled keyword growth.

---

18. Token Identity Migration

The existing lexer/parser integration contains overlapping concepts such as:

BitAnd / Ampersand
Question / QuestionMark

and related operator-context distinctions.

Migration MUST establish one canonical lexical identity per source spelling unless there is a demonstrable lexical reason for distinct token identities.

Context-sensitive semantic meaning belongs downstream.

The migration rule is:

source spelling
      ↓
canonical token
      ↓
parser context
      ↓
AST meaning
      ↓
semantic meaning

not:

one spelling
      ↓
multiple competing lexical authorities

Any token consolidation MUST be tested against all existing parser contexts before the legacy token is removed.

---

19. Keyword Migration

A keyword migration MUST account for:

- lexer keyword registry;
- token enum;
- parser dispatch;
- grammar;
- AST;
- diagnostics;
- source compatibility;
- reserved identifiers;
- dialect interaction;
- tooling;
- tests.

A keyword MUST NOT be added solely because an extended design document uses that word.

The feature must first have an accepted specification and semantic owner.

---

20. Literal Migration

Literal migrations MUST preserve value semantics.

Numeric migrations MUST NOT introduce artificial machine limits.

A literal representation MAY be restricted by the actual implementation representation, but such a representation limit must not be incorrectly presented as a universal language maximum.

Quantum literal migrations must preserve the semantic distinction between:

|0⟩
|1⟩
|+⟩
|-⟩
|ψ⟩

and arbitrary quantum-state expressions.

---

21. Type Migration

Type migrations MUST preserve:

- type identity;
- generic parameters;
- constraints;
- ownership;
- variance where applicable;
- resource semantics;
- capability requirements;
- quantum semantics;
- hardware intent.

Types such as:

Qubit[n]
Tensor<T, shape>
Memory<T, size>

must be treated as semantic/program requirements rather than compiler-wide maximums.

A migration MUST NOT transform a symbolic resource requirement into a fixed compiler limit.

---

22. Resource Migration

The following concepts MUST remain distinct:

Requirement
Capability
Constraint
Preference
Hint
Realization

For example:

requires qubits >= n

does not mean:

physical qubits 0..n-1

Likewise:

requires capability("gpu.compute")

does not mean:

GPU 0

A migration MUST preserve this distinction.

---

23. Scalability Migration Invariant

Migration tooling MUST preserve open-ended scalability.

The following MUST NOT be introduced as universal language ceilings:

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
MAX_VECTOR_WIDTH
MAX_TENSOR_RANK
MAX_TENSOR_DIMENSION
MAX_DEVICE_COUNT
MAX_NETWORK_SIZE
MAX_TIMELINE_COUNT
MAX_AGENT_COUNT
MAX_MODULE_COUNT
MAX_PROGRAM_SIZE

Likewise, migrations MUST NOT replace scalable abstractions with fixed machine assumptions.

A program may contain an ordinary constant:

let n = 1024;

because that is program data.

The prohibited construct is a compiler/language architecture that says:

Zamani supports no more than 1024 elements.

---

24. POCO-REAF Migration Rule

A migration MUST preserve:

Program Intent
        ≠
Target Realization

A valid migration:

requires quantum
        ↓
requires capability("quantum.compute")
        ↓
quantum semantic model
        ↓
quantum::ir
        ↓
target realization

is preferable to:

requires quantum
        ↓
specific vendor QPU

unless the original source explicitly requested a vendor-specific target.

Likewise:

parallel

must not silently migrate into:

exactly N threads

merely because the current backend happens to use N threads.

---

25. Quantum Migration

Quantum migration is governed by the canonical "quantum::ir" boundary.

Legacy quantum syntax MUST migrate toward:

source
  ↓
AST
  ↓
semantic quantum operation
  ↓
quantum::ir

not:

source
  ↓
legacy QuantumGate IR
  ↓
new quantum IR

The quantum grammar MUST remain open-world where the semantics permit it.

A fixed enumeration such as:

H
X
Y
Z
CNOT
...

MUST NOT become the universal semantic gate model.

The preferred model is:

operationSpecifier
quantumTargetList
parameters
results
attributes
modifiers
effects
capabilities

This permits:

apply H
apply custom_gate
apply vendor.operation
apply operation(parameter)

without making every future operation a grammar change.

---

26. Quantum Operation Migration

For every migrated quantum operation, preserve:

- operation identity;
- namespace;
- operands;
- parameters;
- result semantics;
- modifiers;
- control semantics;
- adjoint semantics;
- measurement semantics;
- ordering;
- classical feed-forward;
- resource requirements;
- capability requirements;
- source spans.

A gate spelling change is not necessarily a semantic migration.

A change in measurement, state preparation, control, or ordering semantics is a semantic migration and MUST be classified accordingly.

---

27. Quantum Resource Migration

Quantum migrations MUST preserve the distinction between:

logical qubits
physical qubits
available qubits
required qubits
allocated qubits
mapped qubits

A migration MUST NOT convert:

Qubit[n]

into a fixed maximum.

Likewise, the migration system MUST NOT introduce:

MAX_QUBITS

as a language restriction.

Resource feasibility belongs to resource analysis, routing, scheduling, HAL, and runtime layers.

---

28. QEC and ZQN Migration

Migration MUST NOT move QEC implementation into the grammar.

The architecture remains:

quantum source
      ↓
quantum semantic model
      ↓
quantum::ir
      ↓
QEC analysis
      ↓
ZQN
      ↓
routing
      ↓
scheduling
      ↓
HAL

A source migration involving:

error correction
fault tolerance
noise
fidelity
logical operations
resilience

must preserve semantic intent while leaving actual correction, decoding, calibration, and physical realization to their respective subsystems.

---

29. HDL Migration

HDL migrations MUST preserve hardware intent.

They MUST distinguish:

logical width
physical implementation width
parameterized width
target-supported width

A legacy construct such as:

wire [31:0]

must not be generalized by hard-coding another universal width.

Where the width is a program property, it remains program data or a type parameter.

Where the width is a target restriction, it belongs to capability/target analysis.

---

30. Hardware Migration

Hardware-facing syntax MUST migrate toward target-independent intent.

For example:

requires capability("tensor.compute")
requires capability("gpu.compute")
requires capability("quantum.measurement")
requires memory >= required_memory
requires topology(...)

are portable requirements.

Physical mappings such as:

logical resource → physical resource

belong downstream.

A migration MUST NOT turn portable source semantics into a physical device selection unless the source explicitly requested target-specific behavior.

---

31. Classical Migration

Classical computation MUST retain the existing general mathematical and computational capabilities without converting every operation into a reserved keyword.

Existing vector, matrix, tensor, numerical, symbolic, statistical, optimization, signal-processing, and scientific concepts should migrate toward:

generic operation
+
type information
+
library/intrinsic semantics
+
capability/resource requirements

rather than a continuously expanding keyword list.

---

32. Hybrid Migration

Hybrid migrations MUST preserve the boundary between classical and quantum semantics.

For example:

classical computation
        ↓
quantum operation
        ↓
measurement
        ↓
classical decision
        ↓
quantum operation

must remain one semantic program.

Migration MUST NOT duplicate classical and quantum AST/IR pipelines unnecessarily.

---

33. Distributed Migration

Distributed migrations MUST preserve logical topology and communication semantics.

A migration MUST NOT replace:

parallel distributed computation

with:

exactly N nodes

unless the number is part of explicit program semantics.

Node count, placement, routing, replication, and physical network topology belong to resource/deployment/runtime layers.

---

34. AI and Data Migration

AI/data migrations MUST preserve semantic constructs such as:

- model;
- tensor;
- dataset;
- training;
- inference;
- differentiation;
- optimization;
- agent;
- pipeline;
- provenance.

Framework-specific constructs MUST remain interoperability or dialect concerns unless explicitly accepted into core Zamani.

Migration must not make a specific AI framework a permanent language dependency.

---

35. Networking Migration

Networking migrations MUST distinguish:

logical endpoint
logical channel
protocol requirement
service capability
physical address
physical route

A migration from one address representation to another must preserve logical endpoint identity where promised.

Physical addresses must not be embedded into portable semantic requirements unless explicitly target-specific.

---

36. Security Migration

Security migrations MUST preserve:

- identity;
- authorization;
- capabilities;
- policies;
- provenance;
- cryptographic intent;
- trust relationships;
- secure-computation requirements.

A migration MUST NOT silently weaken a security guarantee.

If a security property cannot be preserved, the migration MUST be classified as semantic and require explicit handling.

---

37. Dialect Migration

Dialects are explicitly versioned extensions.

Every dialect migration MUST define:

Dialect ID
Old Version
New Version
Owner
Syntax Changes
Semantic Changes
AST Mapping
IR Mapping
Dependencies
Capabilities
Requirements
Compatibility
Migration Procedure
Removal Policy
Tests

A dialect MUST NOT silently change the meaning of core Zamani syntax.

Dialect migration MUST terminate in the canonical semantic model.

Quantum dialects MUST terminate in "quantum::ir" where they express quantum computation.

---

38. AST Migration

AST migrations MUST be semantic transformations, not merely struct renames.

For each AST change, define:

Old AST node
New AST node
Field mapping
Default handling
Deleted information
New information
Source-span mapping
Semantic equivalence
Diagnostics
Tests

An AST node may be renamed without semantic migration.

An AST change that changes semantic interpretation is an M5 migration.

---

39. Semantic Model Migration

Semantic migration is required whenever the language's meaning changes.

The migration MUST explicitly document:

old meaning
new meaning
reason
programs affected
preserved properties
changed properties
required developer action

The compiler MUST NOT silently apply a semantic change to older source without a version/compatibility rule authorizing it.

---

40. IR Migration

IR migrations MUST preserve semantic contracts.

The canonical architecture remains:

AST
 ↓
semantic analysis
 ↓
canonical semantic model
 ↓
canonical IR

A temporary compatibility adapter MAY translate:

old IR
 ↓
current IR

but such an adapter MUST have a documented retirement condition.

It MUST NOT become a second permanent compiler architecture.

---

41. "quantum::ir" Migration

"quantum::ir" is the canonical quantum semantic boundary.

Any quantum migration MUST answer:

Does the old representation map to quantum::ir?
Are all parameters preserved?
Are controls preserved?
Are measurements preserved?
Are results preserved?
Are ordering constraints preserved?
Are resource requirements preserved?
Are capabilities preserved?
Are source spans preserved?

If yes, the old quantum representation should terminate at "quantum::ir".

If no, the migration is semantic and MUST be explicitly classified.

---

42. Artifact Migration

Compiled artifacts MAY require migration independently of source.

Artifact migration MUST define:

- artifact identity;
- artifact version;
- schema version;
- producer version;
- consumer compatibility;
- integrity verification;
- migration direction;
- rollback/recovery;
- provenance.

An artifact migration MUST NOT be confused with source-language compatibility.

A source program can remain compatible while an old binary artifact becomes unsupported.

---

43. ABI Migration

ABI migrations MUST be handled independently from language syntax.

A source-level migration MUST NOT introduce ABI assumptions into the grammar merely because one backend ABI changed.

ABI compatibility belongs to interoperability/compiler/backend contracts.

If an ABI change affects externally visible semantics, it MUST be reflected in the appropriate compatibility layer.

---

44. Runtime Migration

Runtime migration MUST preserve source semantics.

Runtime changes may include:

- scheduling;
- resource discovery;
- placement;
- resilience;
- checkpointing;
- execution lifecycle;
- observability;
- deployment.

These changes do not automatically constitute language migrations.

A runtime incompatibility MUST be reported as runtime/target incompatibility rather than incorrectly deprecating valid source syntax.

---

45. Target Migration

Target migration is explicitly separate from language migration.

The same Zamani program may be realized on:

embedded
CPU
multicore CPU
GPU
FPGA
ASIC
QPU
quantum simulator
accelerator
HPC
cluster
distributed system
cloud
future target

A target migration MUST preserve source semantics where the target satisfies the required capabilities and resources.

If it does not, the compiler/runtime MUST report resource or capability incompatibility.

It MUST NOT silently change the program.

---

46. Capability Migration

Capability identifiers are semantic requirements.

A capability migration MUST preserve capability meaning.

For example:

quantum.measurement

may evolve in representation while retaining the same semantic capability identity.

If the capability meaning changes, the migration must be classified semantically.

Capabilities MUST remain open-ended.

No universal finite capability list may be treated as the complete future language universe.

---

47. Resource Migration

Resource expressions MUST remain symbolic where appropriate.

Examples:

requires qubits >= n
requires memory >= required_memory
requires capability("tensor.compute")
requires topology(...)

A migration MUST preserve whether a resource expression is:

- mandatory;
- optional;
- preferred;
- a constraint;
- a hint.

A migration MUST NOT turn a preference into a requirement or a requirement into a hard-coded implementation choice without an explicit semantic rule.

---

48. Migration and Deprecation

Deprecation and migration are related but distinct.

The preferred stable-feature lifecycle is:

STABLE
   ↓
DEPRECATED
   ↓
MIGRATION AVAILABLE
   ↓
REMOVAL ELIGIBLE
   ↓
REMOVED

Experimental features MAY follow:

EXPERIMENTAL
   ↓
REMOVED

when they were never promised as stable.

A deprecated feature must remain supported according to "deprecated.md" until its removal conditions are satisfied.

---

49. Migration and Reserved Syntax

If an old identifier conflicts with newly reserved syntax, migration MUST account for:

- lexical ambiguity;
- source compatibility;
- escape mechanisms;
- rename tooling;
- diagnostics;
- dialect interaction.

A newly reserved keyword MUST NOT silently capture an existing identifier and change its meaning.

Where possible, the migration should provide a deterministic rename.

---

50. Migration Ordering

When several layers change, migrations MUST be ordered:

1. Specification
2. Feature identity
3. Lexical contract
4. Grammar contract
5. AST contract
6. Semantic contract
7. IR contract
8. Compiler integration
9. Runtime/backend integration
10. Compatibility metadata
11. Tests
12. Generated documentation
13. Deprecation/removal metadata

However, implementation work may be developed independently provided that the final integration satisfies this dependency order.

No downstream file may silently invent an upstream contract.

---

51. Independent-File Completion Contract

Every migration-related file MUST be independently complete.

Before a file is considered finished, it MUST identify:

Purpose
Owns
Does Not Own
Inputs
Outputs
Dependencies
Upstream Contracts
Downstream Consumers
Syntax
AST Contract
Semantic Contract
IR Contract
Compiler Integration
Runtime Integration
Diagnostics
Positive Tests
Negative Tests
Boundary Tests
Scalability Tests
Compatibility Tests
Determinism
Security
Hard-Coding Audit
Unsafe-Rust Audit
Completion Criteria

This prevents the situation where one file appears finished but must later be rewritten because another file introduced an undocumented dependency.

---

52. Feature Manifest Integration

Where the repository uses feature manifests, each migrated feature MUST reference its feature manifest.

The manifest SHOULD contain:

id
name
status
version
syntax
lexer_tokens
grammar_rules
ast_nodes
semantic_rules
ir_mapping
compiler_consumers
runtime_consumers
domain
capabilities
resource_requirements
negative_tests
boundary_tests
scalability_tests
compatibility
hard_coding_policy
migration
deprecation

The migration record and feature manifest MUST agree.

Neither may silently contradict the other.

---

53. Compatibility Matrix Integration

Every migration that changes compatibility MUST update the compatibility matrix.

At minimum, the matrix must be able to represent:

Language Version
Grammar Version
Lexer Contract
Parser Contract
AST Schema
Semantic Schema
IR Schema
Quantum IR Schema
Dialect Version
Artifact Schema
ABI
Runtime Contract

A migration is incomplete if its compatibility relationship is not represented where the matrix requires it.

---

54. "grammar.md" Integration

After migration:

grammar.md

must accurately report implementation status.

The migration system MUST NOT mark a feature "IMPLEMENTED" merely because the migration tool can rewrite source.

The implementation chain remains:

Specification
 ↓
Grammar
 ↓
Lexer
 ↓
Parser
 ↓
AST
 ↓
Semantic
 ↓
IR
 ↓
Compiler
 ↓
Runtime
 ↓
Tests

---

55. "Zamani-Grammar.md" Integration

Historical and proposed syntax in "Zamani-Grammar.md" MUST be classified.

Recommended statuses:

stable
proposed
experimental
deprecated
historical
not implemented

A migration MUST NOT promote a feature directly from historical design material into stable language without passing the normal specification → implementation → test path.

---

56. Existing Lexer/Parser Compatibility

The current reference implementation in "src/lexer.rs" and "src/parser.rs" contains a substantially broader vocabulary than a minimal core language, including quantum, nano, Sankofa, effects, advanced declarations, and other constructs.

Therefore, migration work MUST begin with an inventory of actual executable behavior.

For each existing token/parser construct:

token exists?
grammar exists?
parser exists?
AST exists?
semantic implementation exists?
IR mapping exists?
tests exist?

The result must be recorded as:

SPECIFIED
LEXER_IMPLEMENTED
PARSER_IMPLEMENTED
AST_IMPLEMENTED
SEMANTIC_IMPLEMENTED
IR_IMPLEMENTED
TESTED
STABLE
EXPERIMENTAL
DEPRECATED

No migration document may claim full implementation merely from grammar presence.

---

57. Source-Level "unsafe" vs Rust "unsafe"

The repository requirement is:

«Production Zamani Rust implementation MUST use safe Rust.»

This means the Rust implementation MUST NOT contain or require:

unsafe
unsafe fn
unsafe impl
unsafe trait
unsafe { ... }

However, this requirement must not be confused with the Zamani source-language word:

unsafe

The current reference frontend contains an "unsafe" keyword/parser path.

Therefore migration must treat these as separate concerns.

If Zamani source-level "unsafe" remains, it requires:

- syntax definition;
- semantic definition;
- capability model;
- security model;
- compiler behavior;
- diagnostics;
- tests;
- compatibility status.

Parsing the word "unsafe" does not itself provide safety.

If the intended future language contract eliminates unrestricted source-level unsafe execution, that source construct must follow the normal:

STABLE/EXPERIMENTAL
 ↓
DEPRECATED
 ↓
MIGRATION
 ↓
REMOVED

process.

The Rust implementation's safe-Rust requirement remains independent.

---

58. Rust 1.97 / 1.97.1 Compatibility

The production baseline is:

Rust 2021
Rust 1.97 / Rust 1.97.1

Migration tooling and implementation MUST NOT depend on APIs unavailable to the declared minimum Rust version.

Rust compiler-version changes MUST NOT be confused with Zamani language-version changes.

For example:

Zamani 1.x
Rust 1.97.1

is a valid combination.

A future Rust update may change the implementation environment without changing the Zamani source language.

The language compatibility policy and Rust toolchain policy therefore remain separate.

---

59. Determinism

Migration MUST be deterministic.

Given:

same source
same source version
same target version
same migration rules
same declared configuration

the migration tool MUST produce the same semantic result.

Migration MUST NOT depend on:

- machine topology;
- current GPU count;
- current QPU count;
- random backend selection;
- uncontrolled environment state;
- unspecified filesystem ordering;
- network responses;
- mutable external services.

If target discovery is necessary, it belongs to target realization, not source transformation.

---

60. Reproducibility

Migration outputs MUST be reproducible when the migration is declared deterministic.

Migration metadata SHOULD record:

source language version
target language version
migration tool version
migration rule versions
feature manifest versions
dialect versions
configuration identity

Where source rewriting is deterministic, the same migration metadata must produce equivalent output.

---

61. Provenance

Every non-trivial migration SHOULD emit provenance containing:

migration-id
source-version
target-version
source-file
source-span
old-feature-id
new-feature-id
migration-class
tool-version
rule-version
timestamp where operationally necessary

Provenance MUST NOT alter program semantics.

---

62. Diagnostics

Migration diagnostics MUST be structured.

A diagnostic SHOULD expose:

code
severity
feature-id
migration-id
source-version
target-version
source-span
old-form
new-form
semantic-impact
automatic-or-manual
replacement

Human-readable wording may evolve without changing the stable diagnostic identity.

---

63. Migration Failure

A migration MUST fail explicitly when semantic preservation cannot be established.

It MUST NOT:

- silently drop operations;
- silently discard resource requirements;
- silently remove quantum controls;
- silently remove measurements;
- silently change HDL widths;
- silently remove security constraints;
- silently convert requirements to hints;
- silently convert capabilities to preferences;
- silently select a physical target;
- silently reduce resource requirements.

Failure is preferable to silent semantic corruption.

---

64. Partial Migration

If a source contains several independent migrations and one cannot be transformed automatically, the tool MAY migrate the deterministic portions provided it clearly records the unresolved portions.

Example:

automatic:
  keyword migration
  module-path migration
  attribute migration

manual:
  changed ownership semantics

The result MUST NOT be presented as fully migrated.

---

65. Rollback

A source migration SHOULD support rollback when technically practical.

Rollback is especially important for:

- automated repository-wide rewrites;
- large syntax migrations;
- generated artifacts;
- schema changes;
- dialect migrations.

Rollback MUST NOT imply that semantic migrations can always be reversed automatically.

Where semantic information has been intentionally removed, the migration must document that reversal requires the original source or external information.

---

66. Migration Transaction Boundary

A repository-wide migration should be treated as a logical transaction:

discover
  ↓
classify
  ↓
validate
  ↓
transform
  ↓
parse
  ↓
semantic-check
  ↓
IR-check
  ↓
test
  ↓
commit migration

A failed validation MUST prevent the migration from being declared complete.

---

67. Migration Testing

Every migration MUST have applicable:

Positive tests

Valid old source transforms into valid new source or valid current semantic representation.

Negative tests

Invalid legacy forms remain invalid or produce the specified migration diagnostic.

Boundary tests

Test:

- empty constructs;
- smallest valid values;
- large symbolic values;
- nested constructs;
- generic constructs;
- Unicode;
- qualified names;
- malformed input;
- ambiguous syntax.

Scalability tests

Verify that migration does not impose artificial ceilings.

Tests must include progressively larger semantic/resource expressions without defining a language maximum merely for the test suite.

Determinism tests

The same input and migration configuration produce equivalent output.

Compatibility tests

Verify the declared compatibility relationship between versions.

---

68. Quantum Migration Tests

Quantum migrations MUST include applicable tests for:

- one qubit;
- multiple qubits;
- symbolic qubit counts;
- parameterized registers;
- custom operations;
- qualified operations;
- parameterized operations;
- controls;
- adjoints;
- measurement;
- reset;
- mid-circuit measurement;
- classical feed-forward;
- logical operations;
- physical mapping metadata;
- resource requirements;
- capability requirements;
- unknown operations;
- unsupported target capabilities.

No test may establish an artificial universal maximum.

---

69. HDL Migration Tests

HDL migrations MUST test:

- parameterized widths;
- signals;
- registers;
- combinational logic;
- sequential logic;
- clocking;
- reset;
- timing;
- interfaces;
- state machines;
- memories;
- generate constructs;
- synthesis intent;
- simulation intent;
- verification properties.

Physical resource limitations belong to target tests, not universal syntax tests.

---

70. POCO-REAF Scalability Tests

The migration test suite MUST distinguish:

language expressiveness

from:

test-machine capacity

A test machine may be small.

That does not authorize the migration system to define the language as small.

For example:

small machine
   ↓
cannot execute 10^12-resource program

is a resource-feasibility result.

It is not:

Zamani cannot express 10^12-resource programs.

---

71. No Hardware-Driven Language Migration

A backend limitation MUST NOT trigger source-language deprecation by itself.

Invalid reasoning:

backend supports only N
        ↓
deprecate programs requiring > N

Correct reasoning:

program requires R
        ↓
target capability/resource analysis
        ↓
target supports R?
     /       \
   yes        no
   ↓           ↓
compile      diagnostic / alternative realization

The source language remains scalable.

---

72. Migration of Resource Requirements

Suppose an old source expresses:

requires qubits >= n

and the new resource model introduces a capability form.

The migration may produce:

requires capability("quantum.compute")
requires qubits >= n

only if the specification explicitly establishes that both are required and semantically equivalent.

It MUST NOT silently discard "n".

Similarly, a migration MUST NOT turn:

requires memory >= required_memory

into:

MAX_MEMORY = fixed_value

---

73. Migration of Physical Mappings

Physical mappings are downstream realization data.

For example:

logical q
    ↓
physical q17

must not be injected into portable source migration unless the original source explicitly contained physical placement.

A migration from a physical-specific legacy construct to portable semantics must preserve any explicitly requested physical behavior.

It must not invent physical behavior where none existed.

---

74. Migration of Operation Enumerations

If an old grammar contains a fixed operation list:

H
X
Y
Z
CNOT
...

and the new semantic model uses:

operationSpecifier

the migration should transform the operation spelling into semantic operation identity.

The migration must preserve:

- operation name;
- namespace;
- parameters;
- targets;
- controls;
- modifiers;
- effects.

New operations should not require historical migration of every existing operation if they can enter through the generic operation model.

---

75. Migration of Vendor-Specific Features

Vendor-specific features MUST be isolated through:

- dialects;
- interoperability;
- capabilities;
- target profiles;
- backend contracts.

A vendor feature MUST NOT silently become a core Zamani language requirement.

Migration from a vendor-specific representation to portable Zamani should preserve semantics where a portable representation exists.

Where no portable representation exists, the source remains explicitly target-specific.

---

76. Module and Package Migration

Module migrations MUST preserve:

- module identity;
- import identity;
- export visibility;
- aliases;
- dependencies;
- version constraints;
- namespace resolution.

Renaming a module is an M2/M3 migration only if the identity mapping is deterministic.

A semantic module split/merge may require M4 or M5 handling.

---

77. Function Migration

Function migrations MUST preserve:

- function identity;
- parameter order;
- parameter types;
- generic parameters;
- effects;
- return type;
- ownership;
- calling semantics;
- async behavior;
- capability requirements.

Changing only syntax is not automatically a semantic migration.

Changing evaluation or ownership semantics is.

---

78. Effect Migration

Effect migrations MUST preserve the declared effect semantics.

A migration MUST NOT silently:

pure → effectful

or:

effectful → pure

without a semantic rule.

Effect handlers must remain associated with their semantic effect identity rather than textual spelling alone.

---

79. Concurrency Migration

Concurrency migrations MUST preserve:

- happens-before relationships;
- synchronization;
- ownership;
- task semantics;
- channel semantics;
- cancellation;
- structured concurrency;
- determinism guarantees.

A migration MUST NOT silently convert a deterministic program into an explicitly nondeterministic program.

Nor may it silently introduce a fixed thread count.

---

80. Memory Migration

Memory migrations MUST preserve:

- ownership;
- borrowing;
- allocation semantics;
- regions;
- address-space meaning;
- persistence;
- sharing;
- distributed memory semantics;
- accelerator-memory intent;
- quantum-memory semantics.

Physical memory capacity remains a resource property.

It is not a language migration limit.

---

81. Interoperability Migration

Interoperability migrations MAY involve:

- C;
- C++;
- Rust;
- Python;
- WebAssembly;
- OpenQASM;
- QIR;
- HDL;
- serialization formats;
- external ABIs.

These are interoperability boundaries.

They MUST NOT replace the canonical Zamani semantic model.

Quantum interoperability formats must map into the canonical quantum semantic model and ultimately "quantum::ir".

---

82. Generated Files

Generated compatibility artifacts MUST identify:

generated
source authority
generator version
generation inputs

A generated file MUST NOT become an independent authority.

For example:

grammar.md

may be generated or synchronized from the authoritative specification and implementation.

Manual edits to generated output MUST NOT be used to establish new language semantics.

---

83. Migration and Documentation

After a migration:

- normative specifications must be updated;
- compatibility metadata must be updated;
- deprecation metadata must be updated where applicable;
- generated references must be regenerated;
- migration examples must be added;
- tests must be updated.

Documentation must not claim that an old construct is removed while the compiler still accepts it under the compatibility contract.

Likewise, documentation must not claim stability for an implementation that has only partial integration.

---

84. Migration Completion Criteria

A migration is COMPLETE only when all applicable conditions are true:

[ ] Migration ID exists
[ ] Feature identity is stable
[ ] Source version identified
[ ] Target version identified
[ ] Migration class identified
[ ] Old syntax documented
[ ] New syntax documented
[ ] AST mapping documented
[ ] Semantic mapping documented
[ ] IR mapping documented
[ ] quantum::ir mapping verified when applicable
[ ] Resource semantics verified
[ ] Capability semantics verified
[ ] Target independence verified
[ ] Diagnostics implemented
[ ] Compatibility metadata updated
[ ] Deprecation metadata updated when applicable
[ ] Feature manifest updated
[ ] Positive tests pass
[ ] Negative tests pass
[ ] Boundary tests pass
[ ] Scalability tests pass
[ ] Determinism tests pass
[ ] Compatibility tests pass
[ ] Hard-coding audit passes
[ ] Rust unsafe audit passes
[ ] Repository integration passes
[ ] Documentation is synchronized

Only then may the migration be marked production-complete.

---

85. Repository-Wide Acceptance Gate

A migration is not production-ready merely because "grammar/Zamani.g4" parses.

The complete acceptance chain is:

Specification
      ↓
Lexical Contract
      ↓
Zamani.g4
      ↓
Lexer
      ↓
Parser
      ↓
AST
      ↓
Structural Validation
      ↓
Name Resolution
      ↓
Type Analysis
      ↓
Effect Analysis
      ↓
Resource / Capability Analysis
      ↓
Semantic Validation
      ↓
Canonical Semantic Model
      ↓
Canonical IR
      ↓
Quantum::IR where applicable
      ↓
Optimization
      ↓
Routing
      ↓
Scheduling
      ↓
Resilience
      ↓
QEC
      ↓
ZQN
      ↓
HAL
      ↓
Target
      ↓
Runtime
      ↓
Tests

A migration must be rejected as incomplete if it leaves a semantic gap in the applicable chain.

---

86. Migration Status Values

Migration records SHOULD use:

PLANNED
DESIGNED
IMPLEMENTING
IMPLEMENTED
VALIDATING
COMPATIBLE
DEPRECATED
MIGRATION_AVAILABLE
REMOVAL_ELIGIBLE
REMOVED
BLOCKED
REQUIRES_MANUAL_ACTION

"IMPLEMENTED" does not mean "COMPATIBLE".

"COMPATIBLE" does not mean "STABLE".

"REMOVED" does not erase historical migration documentation.

---

87. Breaking Migration Requirements

A migration may be classified as breaking only when the compatibility contract cannot preserve existing source semantics.

A breaking migration MUST include:

1. reason;
2. affected versions;
3. affected features;
4. affected source patterns;
5. semantic explanation;
6. migration procedure;
7. diagnostics;
8. compatibility-matrix update;
9. deprecation/removal relationship;
10. tests.

A breaking migration MUST NOT be declared merely because implementation restructuring is inconvenient.

---

88. Removal Requirements

Before a stable feature is removed:

[ ] Deprecated status exists
[ ] Replacement exists where applicable
[ ] Migration exists where practical
[ ] Documentation identifies replacement
[ ] Compatibility window has elapsed
[ ] Removal version is defined
[ ] Compatibility matrix is updated
[ ] Negative tests verify removal
[ ] Historical migration documentation remains

A removed feature MUST produce a deterministic diagnostic rather than silently changing meaning.

---

89. Historical Compatibility

Historical migration support MUST remain documented even after the old syntax is removed.

The repository should retain:

old syntax
 ↓
migration rule
 ↓
new syntax

so that old programs can continue to be upgraded even when the current compiler no longer accepts the old syntax directly.

Where a standalone migration tool supports historical versions, its accepted source-version range MUST be explicit.

---

90. Security of Migration Tools

Migration tools process source code and therefore MUST be treated as compiler infrastructure.

They MUST:

- avoid executing migrated source;
- avoid executing arbitrary generated code;
- avoid hidden network access;
- avoid uncontrolled filesystem modification;
- produce deterministic transformations;
- validate transformed syntax;
- report failures explicitly;
- preserve provenance.

Migration tooling must not use Rust "unsafe".

---

91. No Unsafe Rust

The production migration implementation MUST use safe Rust.

The following are prohibited:

unsafe
unsafe fn
unsafe impl
unsafe trait
unsafe { ... }

If a dependency internally uses unsafe implementation details, that does not automatically mean Zamani itself uses unsafe Rust; however, the project's direct production Rust code MUST NOT introduce unsafe blocks or unsafe APIs as a migration shortcut.

The migration architecture should prefer:

- safe ownership;
- standard collections;
- safe parsing;
- explicit error handling;
- checked arithmetic where needed;
- bounded external input processing;
- deterministic data structures where ordering matters.

---

92. Arithmetic and Representation Limits

Migration code MUST distinguish:

program-defined numeric values

from:

implementation resource limits

It MUST avoid converting an implementation representation detail into a language-level restriction.

Where a representation genuinely has a finite domain, the migration must document that representation boundary explicitly rather than pretending that the language has an arbitrary universal ceiling.

---

93. Migration of Future Domains

The migration system is intentionally open to future domains.

A new domain should integrate through:

common lexical model
        ↓
common syntax foundations
        ↓
common type/effect/resource/capability model
        ↓
domain semantic model
        ↓
canonical IR boundary
        ↓
compiler/backend

A new domain MUST NOT require creating a second language.

The same migration rules apply to future domains even when the actual hardware does not yet exist.

---

94. Migration of Nano / Atom-Scale Computation

Nano/atom-scale constructs MUST preserve semantic intent without encoding a fixed physical universe into the grammar.

For example:

atom
molecule
material
nano-agent
interaction

may be semantic constructs.

The migration system MUST NOT encode a finite set of atoms, molecules, agents, or interaction counts as universal language limits.

Physical feasibility belongs downstream.

---

95. Migration of Temporal / Multi-Timeline Features

Timeline migrations MUST preserve:

- timeline identity;
- temporal ordering;
- branch/merge semantics;
- observation semantics;
- rewind semantics;
- speculative execution semantics.

The number of timelines MUST remain scalable.

A migration MUST NOT impose a universal fixed timeline count.

---

96. Sankofa-Related Migration

Where Sankofa concepts are represented in Zamani syntax, migration must preserve their semantic distinction.

Examples may include:

remember
recall
learn
wisdom
zamani
sasa
ancestor
consensus
temporal learning

These constructs must not be treated as parser-only keywords.

Where their semantics are stable, migration preserves their semantic identity independent of spelling.

Where they are experimental or historical, their status must be represented in the feature/deprecation system.

---

97. Compiler Self-Description and Governance Features

Extended grammar material may contain compiler/self-hosting concepts such as:

bootstrap
compiler
grammar_snapshot
constitutional compiler
self_compile
grammar_diff
compiler_snapshot

These MUST NOT automatically become stable Zamani syntax merely because they occur in historical/extended grammar material.

If accepted, each feature must pass:

specification
→ syntax
→ lexer
→ parser
→ AST
→ semantics
→ IR/implementation contract
→ tests
→ compatibility

Migration of these features must not create a second compiler governance system outside the repository's canonical architecture.

---

98. Migration of Existing Monolithic Grammar Features

The existing broad "Zamani.g4" contains many domains in one grammar surface.

Migration toward modular directories is allowed and encouraged for maintainability, but the migration MUST preserve one canonical language.

The preferred transformation is:

historical monolithic rule
        ↓
canonical modular rule
        ↓
same AST semantics
        ↓
same semantic model
        ↓
same IR

not:

old monolithic language
+
new modular language
=
two languages

---

99. No Unnecessary File Renames

Migration work MUST NOT rename existing authoritative files unnecessarily.

In particular, retain:

grammar/Zamani.g4
grammar/grammar.md
grammar/Zamani-Grammar.md
grammar/DESIGN.md
grammar/README.md
grammar/compatibility/versions.md
grammar/compatibility/migrations.md
grammar/compatibility/deprecated.md
grammar/compatibility/compatibility-matrix.md

If a new file is required, it should be introduced only when it has a distinct ownership responsibility.

Existing directories should be populated before a parallel hierarchy is created.

---

100. Integration Map

The migration file integrates with the repository as follows:

DESIGN.md
    ↓
architecture

specification/
    ↓
normative language meaning

spec/
    ↓
formal contracts

compatibility/versions.md
    ↓
version policy

compatibility/migrations.md
    ↓
migration procedure

compatibility/deprecated.md
    ↓
deprecation lifecycle

compatibility/compatibility-matrix.md
    ↓
compatibility relationships

compatibility/reserved.md
    ↓
reserved syntax

validation/
    ↓
automated compatibility checks

Zamani.g4
    ↓
canonical ANTLR syntax

lexer/
    ↓
lexical contracts

src/lexer.rs
    ↓
executable lexical implementation

src/parser.rs
    ↓
executable parser

src/ast/
    ↓
source structure

semantic analysis
    ↓
meaning

quantum::ir
    ↓
canonical quantum semantic boundary

IR
    ↓
canonical compiler representation

compiler
    ↓
lowering/optimization

routing / scheduling / resilience / QEC / ZQN
    ↓
realization planning

HAL
    ↓
target capability/state

runtime
    ↓
execution

---

101. Migration Dependency Rule

A migration MUST identify every downstream consumer that depends on its changed contract.

For example, a quantum syntax migration may affect:

grammar/quantum/
lexer
parser
AST
semantic quantum model
quantum::ir
optimizer
routing
QEC
ZQN
HAL
OpenQASM interoperability
tests
documentation

The migration record must identify these consumers before the migration is declared complete.

---

102. No Hidden Cross-File Dependencies

A migration file MUST NOT rely on an undocumented future change in another file.

All assumptions must be expressed through explicit references to:

- specification;
- feature manifest;
- compatibility matrix;
- AST contract;
- semantic contract;
- IR contract;
- implementation contract.

This ensures a file can be completed independently and integrated without re-editing it merely because another file later became more specific.

---

103. Migration Review Checklist

Before accepting a migration:

Authority

- [ ] Correct owner identified.
- [ ] No competing authority created.
- [ ] Existing filenames retained where appropriate.

Syntax

- [ ] Lexer impact known.
- [ ] Grammar impact known.
- [ ] Parser impact known.
- [ ] Ambiguity checked.
- [ ] Precedence checked.

AST

- [ ] AST mapping defined.
- [ ] Source spans preserved.
- [ ] No unnecessary duplicate AST representation.

Semantics

- [ ] Meaning preserved or explicitly changed.
- [ ] Type semantics checked.
- [ ] Effects checked.
- [ ] Ownership checked.
- [ ] Resource semantics checked.
- [ ] Capability semantics checked.

IR

- [ ] Canonical IR mapping defined.
- [ ] "quantum::ir" checked when applicable.
- [ ] No duplicate permanent IR introduced.

Hardware

- [ ] Target independence checked.
- [ ] No physical resource limit introduced.
- [ ] Capability/requirement distinction preserved.
- [ ] Routing/scheduling remain downstream.

Compatibility

- [ ] Version policy updated.
- [ ] Compatibility matrix updated.
- [ ] Deprecation metadata updated where necessary.
- [ ] Removal conditions defined where applicable.

Safety

- [ ] No Rust "unsafe".
- [ ] Migration tool does not execute source.
- [ ] External input handling is validated.

Tests

- [ ] Positive.
- [ ] Negative.
- [ ] Boundary.
- [ ] Scalability.
- [ ] Determinism.
- [ ] Compatibility.
- [ ] Diagnostics.

---

104. Production-Readiness Definition

"grammar/compatibility/migrations.md" and its migration system are production-ready only when:

Every supported migration has a stable identity
        AND
Every migration has an explicit compatibility class
        AND
Every migration has a semantic preservation statement
        AND
Every affected layer is identified
        AND
Every migration has tests
        AND
Every breaking change is explicit
        AND
Every deprecation has a migration path where practical
        AND
No migration introduces artificial hardware limits
        AND
No migration duplicates canonical quantum semantics
        AND
No migration silently changes target realization
        AND
No production Rust migration implementation uses unsafe
        AND
Repository-wide compatibility metadata is synchronized

---

105. Final Migration Principle

The complete Zamani migration architecture is:

                  OLD PROGRAM
                       │
                       ▼
              VERSION IDENTIFICATION
                       │
                       ▼
              MIGRATION CLASSIFICATION
                       │
                       ▼
              SYNTAX / AST ANALYSIS
                       │
                       ▼
             SEMANTIC EQUIVALENCE
                 /             \
               YES              NO
                │                │
                ▼                ▼
       AUTOMATIC / ASSISTED   EXPLICIT
             MIGRATION        SEMANTIC MIGRATION
                │                │
                └───────┬────────┘
                        ▼
                CURRENT AST
                        │
                        ▼
               SEMANTIC ANALYSIS
                        │
                        ▼
             CANONICAL SEMANTIC MODEL
                        │
            ┌───────────┴───────────┐
            ▼                       ▼
       Classical                Quantum
                                 │
                                 ▼
                            quantum::ir
            │                       │
            └───────────┬───────────┘
                        ▼
                 CANONICAL IR
                        │
                        ▼
                  OPTIMIZATION
                        │
             ┌──────────┼──────────┐
             ▼          ▼          ▼
          ROUTING   SCHEDULING   RESILIENCE
             │          │          │
             └──────────┼──────────┘
                        ▼
                       QEC
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
             ┌──────────┼──────────┐
             ▼          ▼          ▼
            CPU        GPU        FPGA
             │          │          │
             ├──────────┼──────────┤
             ▼          ▼          ▼
            ASIC       QPU      DISTRIBUTED
                        │
                        ▼
                  FUTURE TARGETS

The migration system therefore protects the central Zamani invariant:

«A language evolution may change how a program is represented, but it must not silently change what the program means.»

And it protects the POCO-REAF invariant:

«Program source describes computation and requirements; migration must not turn those semantics into temporary hardware limitations.»

Consequently, migration must preserve the ability for one Zamani program to scale from the smallest useful computation to arbitrarily large realizations permitted by the program's semantics, implementation representation, declared constraints, target capabilities, and resources actually available.

The language does not promise that infinite physical resources exist.

It promises that the language architecture does not manufacture artificial finite ceilings where the semantics do not require them.

The permanent architectural boundary is therefore:

SOURCE
  ↓
MEANING
  ↓
CANONICAL IR
  ↓
REALIZATION

not:

SOURCE
  ↓
TODAY'S HARDWARE
  ↓
PERMANENT LANGUAGE LIMIT

That is the compatibility and migration foundation required for Zamani's production path toward:

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF).