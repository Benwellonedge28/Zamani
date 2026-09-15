Zamani Migration Specification

Path: "grammar/compatibility/migrations.md"
Status: Normative
Scope: Source-language migrations, grammar migrations, AST migrations, semantic migrations, dialect migrations, IR migrations, artifact migrations, tooling migrations, and repository-wide compatibility transitions
Implementation baseline: Rust 1.97 / Rust 1.97.1
Safety requirement: Zamani Rust implementation MUST NOT use "unsafe" Rust
Primary objective: Enable controlled language evolution without sacrificing semantic stability, portability, scalability, or POCO-REAF
POCO-REAF: Program Once, Compile Once, Run Everywhere, Anywhere, Forever

---

1. Purpose

This document defines the normative migration policy for Zamani.

It specifies how Zamani evolves when an existing:

- source program;
- grammar construct;
- token;
- AST representation;
- type;
- effect;
- capability;
- resource expression;
- module;
- dialect;
- semantic contract;
- canonical IR;
- quantum representation;
- hardware representation;
- compiled artifact;
- ABI;
- runtime contract;
- tool;
- diagnostic;
- serialization format;
- or repository interface

must change.

Migration MUST preserve program meaning whenever the compatibility contract permits it.

Migration MUST make incompatibilities explicit when semantic preservation is impossible.

Migration MUST NOT conceal breaking changes as implementation details.

Migration MUST NOT turn temporary hardware limitations into permanent source-language limitations.

---

2. Fundamental migration principle

The governing rule is:

«Migrate representations; preserve semantics.»

The preferred transformation is:

Old Source
    |
    v
Old Syntax
    |
    v
Migration / Compatibility Layer
    |
    v
Canonical Semantic Model
    |
    v
Current AST / Semantic Model
    |
    v
Canonical IR
    |
    +--------------------+
    |                    |
    v                    v
Classical IR         quantum::ir
    |                    |
    +----------+---------+
               |
               v
       Optimization / Scheduling
               |
               v
        Target / Runtime

A migration MUST NOT create a permanent duplicate semantic architecture merely to support historical syntax.

Where possible:

old syntax
    ↓
canonical meaning
    ↓
current representation

is preferred over:

old syntax
    ↓
old permanent IR
    ↓
parallel compiler

---

3. Relationship to other compatibility authorities

This document does not replace other normative compatibility documents.

The ownership relationship is:

grammar/specification/language-version.md
        |
        | owns language version model
        v
grammar/specification/compatibility.md
        |
        | owns compatibility interpretation
        v
grammar/compatibility/migrations.md
        |
        | owns migration procedure
        +-------------------------------+
        |               |               |
        v               v               v
deprecated.md      reserved.md    compatibility-matrix.md
        |
        v
grammar/validation/compatibility-rules.md
        |
        v
grammar-authority / syntax / semantic contracts
        |
        v
grammar
        |
        v
lexer / parser / AST
        |
        v
semantic analysis
        |
        v
canonical IR
        |
        +-------------+-------------+
        |             |             |
        v             v             v
    classical      quantum       hardware
        |             |             |
        +-------------+-------------+
                      |
                      v
             compiler / runtime

Ownership

"language-version.md" owns:

- version numbering;
- language-version semantics;
- major/minor/patch meaning;
- language version declarations.

"compatibility.md" owns:

- compatibility dimensions;
- compatibility levels;
- compatibility classification.

"migrations.md" owns:

- migration procedures;
- migration ordering;
- migration obligations;
- migration validation;
- source transformation policy;
- compatibility-layer policy;
- migration testing;
- migration completion criteria.

"deprecated.md" owns:

- active deprecated features;
- deprecation metadata;
- removal eligibility.

"reserved.md" owns:

- reserved syntax;
- future-reserved identifiers;
- reserved namespace policy.

"compatibility-matrix.md" owns:

- machine-readable or tabular compatibility relationships between versions and components.

"grammar/validation/compatibility-rules.md" owns:

- validation rules used to detect compatibility violations.

No document MAY silently take ownership of another document's contract.

---

4. Migration goals

Every migration MUST be evaluated against these goals:

1. semantic preservation;
2. deterministic behavior;
3. source compatibility where promised;
4. explicit breaking-change classification;
5. reproducibility;
6. diagnosability;
7. rollback capability where practical;
8. forward extensibility;
9. POCO-REAF preservation;
10. hardware independence;
11. quantum/classical semantic preservation;
12. canonical IR integrity;
13. dialect isolation;
14. artifact integrity;
15. security preservation;
16. scalability preservation.

---

5. Migration terminology

5.1 Source migration

Transformation of source code from one supported language representation to another.

5.2 Grammar migration

Migration necessitated by changes to lexical or syntactic representation.

5.3 Semantic migration

Migration required because the meaning or interpretation of a construct changed.

5.4 AST migration

Transformation between AST schemas while preserving source semantics.

5.5 IR migration

Transformation between IR schema versions.

5.6 Artifact migration

Transformation of compiled or serialized artifacts.

5.7 Dialect migration

Migration between versions of an explicitly declared dialect.

5.8 Target migration

Migration caused by target/backend changes.

Target migration MUST NOT automatically become a language migration.

5.9 Mechanical migration

A transformation that can be performed deterministically without requiring semantic judgment.

5.10 Assisted migration

A transformation where tooling can identify and transform most cases but human review is required for some cases.

5.11 Manual migration

A transformation requiring explicit developer decisions because semantics cannot be inferred safely.

---

6. Migration classes

Every migration MUST be classified.

The supported classes are:

M0  No migration
M1  Transparent compatibility
M2  Automatic mechanical migration
M3  Tool-assisted migration
M4  Explicit source migration
M5  Semantic migration
M6  Artifact migration
M7  Dialect migration
M8  Target migration
M9  Breaking migration

A change MAY belong to more than one class at different layers.

Example:

source syntax: M2
AST:            M2
semantic model: M0
IR:             M0
runtime:        M8

This is preferable to classifying the entire change as one undifferentiated migration.

---

7. Migration decision rule

For every proposed change, determine:

Does old valid source still parse?
        |
        +-- yes --> Does it mean the same thing?
        |             |
        |             +-- yes --> compatible
        |             |
        |             +-- no --> semantic migration
        |
        +-- no --> Can deterministic transformation preserve meaning?
                      |
                      +-- yes --> automatic/assisted migration
                      |
                      +-- no --> breaking migration

A parser failure alone MUST NOT determine the migration class.

Semantic compatibility is the primary decision criterion.

---

8. Migration invariants

A valid migration MUST preserve, where applicable:

- identifiers;
- binding relationships;
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
- concurrency;
- synchronization;
- error behavior;
- deterministic behavior;
- numerical semantics;
- quantum state semantics;
- quantum operation ordering;
- measurement semantics;
- classical feed-forward;
- hardware intent;
- HDL behavior;
- timing semantics;
- distributed semantics;
- security properties;
- provenance;
- source locations required for diagnostics.

---

9. POCO-REAF migration invariant

Migration MUST preserve the distinction between:

program intent

and:

target realization

A source migration MUST NOT introduce unnecessary target assumptions.

For example, migrating:

requires quantum

MUST NOT silently transform it into:

requires device "specific-qpu"

Likewise:

requires parallelism

MUST NOT silently become:

requires exactly N cores

Likewise:

requires memory capacity >= expression

MUST NOT become a permanent language-level maximum.

Resource realization belongs to:

- resource management;
- capability resolution;
- target selection;
- scheduling;
- deployment;
- runtime;
- hardware abstraction.

---

10. No hard-coded scalability limits

Migration tooling MUST NOT introduce fixed limits for:

- qubits;
- logical qubits;
- physical qubits;
- CPUs;
- cores;
- threads;
- GPUs;
- FPGAs;
- accelerators;
- ASIC resources;
- memory;
- registers;
- vector width;
- tensor dimensions;
- nodes;
- devices;
- network participants;
- services;
- quantum registers;
- hardware modules;
- program size;
- module count;
- package count;
- data size;
- concurrency;
- parallelism.

If migration requires resource information, it MUST use symbolic or dynamically resolved information.

Examples:

resource
capability
requirement
constraint
target
placement
runtime capability
deployment context

rather than fixed implementation constants.

---

11. Source migration requirements

Every source migration MUST define:

Source Version:
Target Version:
Affected Syntax:
Affected Semantics:
Migration Class:
Automatic:
Tool-Assisted:
Manual:
Old Form:
New Form:
Semantic Preservation:
Potential Semantic Changes:
Diagnostics:
Tests:
Rollback:
Compatibility Window:
Removal Version:

No migration may be considered complete without these fields being resolved.

---

12. Automatic migration requirements

A migration MAY be automatic only when the transformation is deterministic.

An automatic migration MUST NOT guess developer intent.

For example, changing an unambiguous spelling is suitable:

old_keyword -> new_keyword

Changing a semantic construct may not be suitable:

old_memory_model -> unknown_new_memory_model

unless the transformation has a formally specified equivalent meaning.

Automatic migration MUST preserve:

- comments where tooling promises preservation;
- source locations where practical;
- formatting where tooling promises formatting preservation;
- annotations;
- attributes;
- semantic metadata.

---

13. Assisted migration requirements

Tool-assisted migrations MUST:

1. identify all affected constructs;
2. produce deterministic diagnostics;
3. explain why migration is required;
4. show the proposed replacement;
5. identify unresolved semantic decisions;
6. refuse unsafe guessing;
7. permit validation before rewriting;
8. emit migration provenance.

A tool MUST distinguish:

automatically transformed

from:

requires human decision

---

14. Manual migration requirements

Manual migration is REQUIRED when:

- semantics changed;
- information was removed;
- two old constructs map to different new meanings;
- hardware intent must be re-expressed;
- resource semantics changed;
- quantum semantics cannot be inferred;
- concurrency semantics changed;
- security guarantees changed;
- dialect semantics changed.

Manual migration documentation MUST contain concrete before/after examples.

---

15. Grammar migration

A grammar reorganization is NOT automatically a language migration.

For example, splitting:

grammar/Zamani.g4

into:

grammar/lexer/
grammar/core/
grammar/types/
grammar/expressions/
...

MAY be implementation-neutral if:

- accepted syntax remains equivalent;
- AST meaning remains equivalent;
- diagnostics remain within the declared contract;
- no new semantic behavior is introduced.

Likewise, ANTLR rule factoring, naming changes, imports, or grammar modularization MAY be version-neutral.

The actual language contract, not file structure, determines compatibility.

---

16. Lexer migration

Lexer changes MUST be audited for:

- token identity;
- keyword recognition;
- contextual keywords;
- identifier compatibility;
- numeric literal interpretation;
- string literal interpretation;
- character literal interpretation;
- Unicode behavior;
- comments;
- whitespace;
- operator precedence;
- token ambiguity.

A new keyword MUST NOT silently invalidate a previously valid identifier unless the compatibility policy explicitly permits it.

Preferred progression:

identifier
    ↓
contextual keyword
    ↓
reserved keyword

when this reduces migration risk.

---

17. Keyword migration

A new keyword MUST undergo:

1. identifier collision analysis;
2. parser conflict analysis;
3. dialect conflict analysis;
4. macro conflict analysis;
5. tooling analysis;
6. migration analysis.

If existing programs may use the proposed keyword as an identifier, the language SHOULD prefer contextual recognition where semantically possible.

If a global reservation is necessary, migration documentation MUST identify:

- affected source;
- first version where reservation applies;
- compatibility period;
- replacement spelling;
- escaping mechanism if available.

---

18. Operator migration

Operator changes MUST evaluate:

- tokenization;
- precedence;
- associativity;
- unary/binary interpretation;
- parser ambiguity;
- AST shape;
- semantic meaning;
- macro interaction;
- formatting;
- tooling.

Changing precedence is potentially semantic-breaking even if every affected program still parses.

Therefore:

same syntax
+
different precedence
=
potentially breaking semantic migration

---

19. Type migration

Type migrations MUST preserve type meaning where compatibility is promised.

When a type is renamed:

OldType -> NewType

the migration SHOULD be mechanical.

When a type's semantics change:

OldType -> NewType

the migration MUST be classified as semantic.

The migration MUST identify:

- representation differences;
- value-domain differences;
- ownership differences;
- conversion requirements;
- overflow behavior;
- nullability;
- generic constraints;
- ABI effects.

---

20. Generic and parameterized migration

Generic parameters MUST remain semantic abstractions.

Migration MUST NOT replace generic resource parameters with fixed hardware quantities.

For example:

tensor<T, Shape>

MUST NOT be migrated into a fixed tensor width merely because a particular backend currently prefers one.

Likewise:

qubit register

MUST remain scalable.

---

21. Effect migration

Changes to effects MUST be treated as semantic changes when they alter observable behavior.

Migration MUST account for:

- IO;
- hardware interaction;
- quantum effects;
- network effects;
- distributed effects;
- security effects;
- resource effects;
- custom effects.

Adding a new effect annotation may be compatible if it merely makes existing semantics explicit.

Removing an effect that was required for correctness may be breaking.

---

22. Capability migration

Capabilities describe what an implementation or target can provide.

A migration MUST NOT confuse:

required capability

with:

selected implementation

Example:

requires capability quantum.measurement

MAY remain portable across multiple QPUs.

The migration MUST NOT bind it permanently to one device.

---

23. Resource migration

Resource declarations MUST remain symbolic and scalable.

A migration MUST preserve the distinction between:

requirement
constraint
preference
hint
capability
availability
placement
allocation

For example:

requires memory >= expression

is not equivalent to:

allocate exactly fixed_memory

unless the language semantics explicitly define them as equivalent.

---

24. Classical migration

Classical migrations MUST preserve:

- arithmetic meaning;
- numeric domains;
- control flow;
- memory semantics;
- ownership;
- concurrency;
- synchronization;
- error behavior;
- deterministic guarantees.

Changes to numerical semantics MUST receive special scrutiny because apparently small changes can alter scientific and financial programs.

---

25. Quantum migration

Quantum migrations MUST preserve, where applicable:

- qubit identity;
- logical-qubit identity;
- physical-qubit identity when explicitly represented;
- register identity;
- operation order;
- gate parameters;
- controlled-operation semantics;
- measurement semantics;
- reset semantics;
- observables;
- classical feed-forward;
- dynamic-circuit behavior;
- probability semantics;
- entanglement;
- state evolution;
- resource requirements;
- error-correction intent.

A syntax migration MUST lower to the existing canonical quantum semantic boundary.

It MUST NOT create a second permanent quantum IR.

The architecture remains:

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
quantum semantic lowering
    ↓
quantum::ir
    ↓
QEC / optimization / routing / scheduling
    ↓
HAL / target
    ↓
runtime

Migration code MUST NOT bypass "quantum::ir".

---

26. Physical versus logical quantum migration

A migration MUST distinguish:

logical qubit

from:

physical qubit

A logical-qubit program MUST remain portable.

Mapping logical qubits to physical qubits belongs to later stages such as:

- routing;
- scheduling;
- hardware abstraction;
- target lowering;
- runtime realization.

A migration MUST NOT convert logical identity into a fixed physical identifier unless the source explicitly declares physical hardware intent.

---

27. Quantum error-correction migration

QEC syntax migrations MUST preserve semantic intent.

Where existing QEC infrastructure owns:

- limits;
- syndrome representation;
- decoding;
- scheduling;
- resources;
- checkpoints;
- distributed execution;

grammar migration MUST only transform source-level declarations into the corresponding semantic structures.

It MUST NOT duplicate QEC algorithms or resource accounting.

---

28. HDL migration

HDL migrations MUST preserve:

- module semantics;
- port semantics;
- signal semantics;
- wire semantics;
- register semantics;
- clock semantics;
- timing;
- combinational behavior;
- sequential behavior;
- process semantics;
- state-machine behavior;
- memory behavior;
- pipeline intent;
- hardware interface semantics.

Hardware implementation choices remain downstream.

A migration MUST NOT turn a generic hardware parameter into a fixed physical implementation unless the source explicitly requests that target-specific behavior.

---

29. Hardware migration

Hardware migrations MUST preserve the distinction:

hardware intent

versus:

hardware realization

The following remain target-level concerns:

- physical device identifiers;
- placement;
- topology;
- routing;
- exact accelerator count;
- exact memory capacity;
- exact device addresses;
- physical resource allocation.

Source migration MUST NOT hard-code them.

---

30. Distributed migration

Distributed migrations MUST preserve:

- communication semantics;
- ordering guarantees;
- consistency requirements;
- fault-tolerance semantics;
- replication intent;
- service identity;
- placement intent.

A migration MUST NOT replace an abstract node/service requirement with a fixed cluster topology unless explicitly requested.

---

31. AI and data migration

AI/data migrations MUST preserve:

- model semantics;
- tensor meaning;
- shape semantics;
- dataset meaning;
- training semantics;
- inference semantics;
- differentiation semantics;
- stream semantics;
- serialization meaning.

Tensor dimensions SHOULD remain symbolic or semantically defined where possible.

A backend-specific tensor limitation MUST NOT become a permanent source-language maximum.

---

32. Networking migration

Networking migrations MUST distinguish:

protocol semantics
endpoint semantics
service semantics
deployment topology

A source migration MUST NOT assume a fixed number of nodes, addresses, endpoints, or services.

---

33. Security migration

Security migrations are semantic migrations whenever they alter:

- authorization;
- authentication;
- confidentiality;
- integrity;
- privacy;
- trust;
- capability boundaries;
- cryptographic meaning.

Security weakening MUST NOT be hidden as a compatibility conversion.

Migration tools MUST NOT silently remove security requirements to make old code compile.

---

34. Module and package migration

Module migrations MUST preserve:

- exported names;
- visibility;
- dependency meaning;
- namespace identity;
- version requirements;
- capability requirements.

When an API moves:

old.module.item

to:

new.module.item

the migration SHOULD provide an explicit compatibility alias where feasible.

Dependency migration MUST NOT silently select a semantically incompatible language version.

---

35. Dialect migration

Every dialect migration MUST identify:

Dialect Namespace:
Old Dialect Version:
New Dialect Version:
Language Version:
Compatibility Class:
Semantic Changes:
Required Capabilities:
Migration Procedure:

Dialect syntax MUST remain namespace-controlled.

A dialect migration MUST NOT silently modify core Zamani semantics.

Dialect conflicts MUST be diagnosed explicitly.

---

36. Experimental feature migration

Experimental features MUST NOT be treated as permanently stable.

When an experimental feature becomes stable:

experimental
    ↓
stabilization
    ↓
stable

the migration MUST verify:

- syntax;
- semantics;
- diagnostics;
- AST representation;
- tooling;
- compatibility;
- documentation;
- tests.

If the final stable syntax differs, a migration path MUST be documented.

---

37. Deprecation-to-removal lifecycle

The standard lifecycle is:

Stable
  ↓
Deprecated
  ↓
Migration Available
  ↓
Removal Eligible
  ↓
Removed
  ↓
Reserved

Removal MUST NOT occur merely because a replacement exists.

Removal requires:

1. documented deprecation;
2. migration path;
3. compatibility analysis;
4. affected-consumer analysis;
5. updated tests;
6. updated examples;
7. updated documentation;
8. removal-version declaration.

---

38. Breaking migration policy

A breaking migration MUST explicitly identify:

Breaking Change:
Affected Version:
Affected Constructs:
Affected Programs:
Reason:
Semantic Impact:
Migration Required:
Automatic Migration:
Manual Decisions:
Compatibility Window:
Removal Version:

Breaking changes MUST NOT be hidden in patch releases.

---

39. Version transition rules

For a transition:

V_old -> V_new

the repository MUST classify every affected public contract.

At minimum:

Lexer
Grammar
Source
AST
Types
Effects
Capabilities
Resources
Modules
Dialects
Semantics
Classical IR
quantum::ir
Hardware IR
Artifacts
ABI
Runtime
Targets
Tooling
Diagnostics
Serialization

Each MUST be classified independently.

---

40. Multi-stage migration

Large migrations SHOULD be staged.

Preferred model:

V1
 |
 | introduce compatibility representation
 v
V1 + migration bridge
 |
 | migrate consumers
 v
V2
 |
 | remove bridge after compatibility window
 v
V3

A compatibility bridge MUST have an explicit owner and removal condition.

Compatibility bridges MUST NOT become permanent undocumented architecture.

---

41. AST migration contract

AST migrations MUST preserve semantic information.

The migration MUST account for:

- source span;
- identifier;
- type;
- generic parameters;
- attributes;
- annotations;
- effects;
- capabilities;
- resource requirements;
- domain information;
- quantum intent;
- hardware intent;
- diagnostics metadata.

If an AST field disappears, the migration MUST prove that its information is either:

1. no longer semantically required; or
2. represented elsewhere without loss.

---

42. Canonical IR migration

IR migrations MUST occur after semantic normalization wherever possible.

Preferred model:

old source
    ↓
old syntax migration
    ↓
current semantic model
    ↓
current canonical IR

rather than:

old source
    ↓
old IR
    ↓
current IR

unless artifact compatibility explicitly requires old IR decoding.

---

43. quantum::ir migration

"quantum::ir" remains the canonical quantum semantic boundary.

If its schema changes:

1. version the IR contract;
2. identify semantic changes;
3. provide conversion where possible;
4. preserve source-level quantum meaning;
5. update quantum consumers;
6. update serialization;
7. update QEC integration;
8. update optimization;
9. update routing;
10. update scheduling;
11. update HAL integration;
12. update runtime integration;
13. update compatibility tests.

The grammar MUST NOT become responsible for the internal quantum IR migration.

---

44. Artifact migration

Artifacts SHOULD carry enough metadata to identify:

- language version;
- grammar compatibility;
- semantic contract;
- IR version;
- dialect versions;
- capability requirements;
- resource requirements;
- artifact format;
- compiler provenance;
- migration provenance.

An artifact MUST NOT depend solely on:

compiler executable version

for compatibility determination.

---

45. Artifact migration safety

An artifact migration MUST validate:

1. artifact integrity;
2. declared version;
3. supported version range;
4. schema;
5. semantic compatibility;
6. capability requirements;
7. resource requirements;
8. target requirements;
9. migration path;
10. provenance.

Malformed or unsupported artifacts MUST fail deterministically.

---

46. Serialization migration

Serialization changes MUST distinguish:

serialization representation

from:

semantic meaning

A serialization format MAY change while semantic compatibility remains intact.

Where backward decoding is supported, the decoder MUST reject:

- malformed data;
- unsupported versions;
- incompatible schemas;
- ambiguous representations.

Migration MUST NOT silently reinterpret corrupted or incompatible data.

---

47. ABI migration

ABI migration is distinct from language migration.

A compiler MAY change ABI while retaining source compatibility if an explicit ABI boundary exists.

ABI migration MUST identify:

- calling convention;
- data layout;
- alignment;
- symbol conventions;
- ownership conventions;
- exception/error boundaries;
- FFI contracts.

The source language MUST NOT be unnecessarily tied to one ABI.

---

48. Runtime migration

Runtime changes MUST preserve source semantics.

A runtime MAY change:

- scheduling;
- allocation;
- optimization;
- device selection;
- execution strategy;

provided the specified semantics remain intact.

Runtime migration MUST NOT silently alter:

- synchronization guarantees;
- quantum measurement meaning;
- memory visibility;
- security guarantees;
- error semantics;
- resource requirements.

---

49. Target migration

Target migration is normally independent of language migration.

Example:

CPU target A
    ↓
CPU target B

MUST NOT require source migration merely because the target changed.

Likewise:

QPU A
    ↓
QPU B

MUST be handled through capability resolution, routing, scheduling, calibration, HAL, and target lowering where possible.

---

50. Compiler migration

A compiler implementation MAY be rewritten without requiring a language migration.

Compiler replacement MUST be validated against:

- grammar conformance;
- AST conformance;
- semantic conformance;
- IR conformance;
- diagnostics;
- determinism;
- compatibility suites.

The compiler implementation is not the language specification.

---

51. Rust implementation requirements

All migration-supporting Rust code MUST target:

Rust 1.97
Rust 1.97.1

The implementation MUST NOT use "unsafe".

Recommended repository-level enforcement:

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

Migration infrastructure MUST use safe Rust abstractions.

Migration code MUST NOT depend on undefined behavior, unsafe FFI assumptions, or target-specific unsafe shortcuts.

---

52. Migration provenance

Every automated migration SHOULD produce provenance containing:

source version
target version
migration identifier
migration tool version
migration ruleset version
input identity
output identity
timestamp where required
warnings
unresolved decisions

Provenance MUST NOT contain secrets unless explicitly required and securely handled.

---

53. Deterministic migrations

Given identical:

input
+
language version
+
migration rules
+
migration tool version

the migration MUST produce the same result.

Migration ordering MUST be deterministic.

Ambiguous migrations MUST fail or request explicit user input.

---

54. Migration idempotence

Where practical, automatic migrations SHOULD be idempotent.

Applying the same migration twice SHOULD produce:

same semantic result

and MUST NOT repeatedly transform the program.

Example:

M(V1 -> V2)(M(V1 -> V2)(P))

MUST NOT continue changing "P" after the first successful migration.

---

55. Migration ordering

When multiple migrations apply:

V1 -> V2
V2 -> V3
V3 -> V4

the preferred migration path is:

V1 -> V2 -> V3 -> V4

unless a formally validated direct migration exists.

Migration tooling MUST identify the selected path.

A migration MUST NOT apply incompatible transformations out of order.

---

56. Migration graph

The compatibility system SHOULD represent migration relationships as a directed graph:

                 +------> V2 ------> V3
                 |         |
                 |         +------> V2.1
                 |
V1 --------------+
 |
 +--------------> V1.1

Each edge MUST identify:

- source;
- destination;
- migration class;
- compatibility;
- automation level;
- semantic risk;
- tests.

---

57. Migration graph constraints

The graph MUST NOT contain undocumented semantic cycles.

If:

V1 -> V2
V2 -> V1

both directions are supported, both transformations MUST specify whether they are:

- lossless;
- lossy;
- semantically equivalent;
- canonicalizing.

A lossy reverse migration MUST never be presented as lossless compatibility.

---

58. Migration validation pipeline

Every production migration SHOULD pass:

Input
  ↓
Version validation
  ↓
Syntax validation
  ↓
Migration analysis
  ↓
Transformation
  ↓
Formatting / preservation
  ↓
Parse validation
  ↓
AST validation
  ↓
Semantic validation
  ↓
Capability validation
  ↓
Resource validation
  ↓
IR validation
  ↓
Compatibility validation
  ↓
Regression tests
  ↓
Output

---

59. Migration testing

Every migration MUST have:

Positive tests

Valid old programs migrate successfully.

Negative tests

Invalid source is rejected.

Boundary tests

Very small and very large valid programs are tested.

Semantic tests

Old and migrated programs are compared at the semantic level.

Compatibility tests

Declared compatibility promises are tested.

Regression tests

Previously migrated programs remain valid.

Determinism tests

Repeated migration produces equivalent output.

Idempotence tests

Repeated migration does not continually transform the program.

---

60. Cross-domain migration tests

The migration suite MUST include combinations of:

classical
quantum
hybrid
HDL
hardware
distributed
AI
data
networking
security

At minimum test:

classical + quantum
classical + HDL
quantum + HDL
quantum + hardware
quantum + distributed
AI + quantum
AI + hardware
classical + quantum + distributed
classical + quantum + HDL + hardware

---

61. Scalability migration tests

Migration tests MUST verify that migrations do not introduce artificial ceilings.

Test with semantically valid representations whose sizes are limited only by available resources.

The migration framework MUST NOT contain assumptions such as:

MAX_QUBITS = fixed value
MAX_CORES = fixed value
MAX_DEVICES = fixed value
MAX_NODES = fixed value

unless such a value is explicitly a representation-format limit and is clearly documented as such.

A representation limit MUST NOT be confused with a language semantic limit.

---

62. Resource exhaustion

A migration implementation MAY encounter resource exhaustion.

This MUST be treated as an implementation/resource failure, not as a language semantic restriction.

Diagnostics SHOULD distinguish:

migration unsupported

from:

migration exceeded available resources

and:

program semantically incompatible

---

63. Migration diagnostics

Migration diagnostics MUST identify:

- migration ID;
- source version;
- target version;
- affected construct;
- source location;
- compatibility classification;
- reason;
- suggested action;
- whether transformation is automatic;
- whether manual intervention is required.

Diagnostics MUST NOT falsely claim that a target limitation is a language limitation.

---

64. Migration warnings

Warnings MUST be deterministic.

A migration warning SHOULD state:

what changed
why it changed
whether semantics changed
what action is required
when the old construct will be removed

Warnings MUST NOT be emitted for constructs that are not actually affected.

---

65. Error classification

Migration failures MUST distinguish at least:

unsupported source version
unsupported target version
unsupported migration
ambiguous migration
semantic incompatibility
syntax incompatibility
AST incompatibility
IR incompatibility
dialect incompatibility
artifact incompatibility
ABI incompatibility
capability unavailable
resource unavailable
malformed input
migration tool failure

A generic "migration failed" diagnostic is insufficient when a more precise classification is available.

---

66. Migration and diagnostics compatibility

Diagnostic changes SHOULD NOT themselves become unnecessary source-breaking changes.

However, diagnostics MUST remain truthful.

A migration MUST NOT preserve an old diagnostic merely for compatibility if doing so causes the diagnostic to become misleading.

---

67. Migration and documentation

Every migration MUST update, where affected:

- "grammar/README.md";
- normative specifications;
- grammar authority documentation;
- language version documentation;
- compatibility documentation;
- deprecation documentation;
- examples;
- migration documentation;
- tests;
- tooling documentation.

Documentation MUST identify whether content is:

Normative
Stable
Experimental
Proposed
Historical
Deprecated
Removed
Implementation-defined
Dialect-defined

---

68. Migration and grammar authority

The migration system MUST preserve a single source-of-truth architecture.

The authority relationship remains:

normative language contract
        ↓
syntax contract
        ↓
grammar representation
        ↓
parser
        ↓
AST
        ↓
semantic model

A migration document MUST NOT become a second grammar specification.

It describes how to move between versions.

---

69. Migration and "Zamani.g4"

Changes to "grammar/Zamani.g4" MUST be accompanied by:

- compatibility classification;
- migration impact analysis;
- grammar tests;
- negative tests;
- compatibility tests;
- AST impact analysis;
- semantic impact analysis;
- documentation impact analysis.

If the change is grammar-only and semantics remain unchanged, it MAY remain language-version-neutral.

---

70. Migration and "Zamani-Grammar.md"

"Zamani-Grammar.md" MUST be updated when migration changes the documented source language.

Historical descriptions MUST be clearly marked.

Migration examples MUST NOT be presented as current syntax unless explicitly identified as such.

---

71. Migration and "grammar.md"

"grammar.md" MUST remain aligned with implementation reality.

If it describes implementation-specific behavior, that behavior MUST be labeled as implementation-specific.

A migration MUST NOT treat an accidental implementation limitation as a language rule.

---

72. Migration and AST/frontend

Frontend migration MUST follow:

source
 ↓
lexer
 ↓
parser
 ↓
AST
 ↓
structural validation
 ↓
semantic model
 ↓
ZUIR / canonical semantic representation
 ↓
domain IR

The AST MUST remain domain-neutral where that is its architectural contract.

Quantum-specific semantics MUST continue toward the canonical quantum IR rather than turning the AST into a quantum backend representation.

---

73. Migration and semantic ownership

A migration MUST be implemented at the layer that owns the changed concept.

Examples:

syntax spelling
    -> grammar/frontend

type meaning
    -> type system

quantum semantic representation
    -> quantum semantic/IR layer

QEC behavior
    -> QEC subsystem

routing
    -> routing subsystem

timing
    -> scheduling subsystem

physical device mapping
    -> hardware/HAL

execution policy
    -> runtime

The grammar migration layer MUST NOT absorb responsibilities owned by downstream subsystems.

---

74. Migration and QEC

QEC migration MUST integrate with the existing QEC contracts.

Migration MUST NOT duplicate:

- QEC limits;
- syndrome representation;
- decoder logic;
- partitioning;
- resource accounting;
- cancellation;
- checkpointing.

Where QEC artifacts carry versions, migration MUST use the QEC artifact/version contract rather than inventing a grammar-specific copy.

---

75. Migration and scheduling

Scheduler migrations MUST preserve scheduling semantics.

A grammar migration MUST NOT hard-code:

- qubit count;
- resource count;
- topology;
- fixed timing;
- fixed device identity.

Scheduling remains responsible for adapting semantic requirements to available resources.

---

76. Migration and hardware abstraction

Hardware migrations MUST use capability-based adaptation.

Preferred:

source intent
    ↓
capability requirements
    ↓
target discovery
    ↓
mapping

Not:

source syntax
    ↓
fixed device identifier

---

77. Migration and runtime

Runtime migration MUST remain compatible with source semantics.

The runtime MAY choose different:

- devices;
- scheduling strategies;
- execution plans;
- allocation strategies;
- communication paths;

provided the language semantics remain satisfied.

---

78. Migration and interoperability

Interoperability migrations MUST identify:

- external language/version;
- ABI;
- serialization;
- calling convention;
- ownership;
- error model;
- data representation;
- security boundary.

Supported external formats such as OpenQASM, Verilog, C, C++, or Python interfaces MUST be treated as interoperability boundaries rather than allowing their semantics to silently redefine Zamani.

---

79. Migration and macros

Macros complicate migration because syntax may be generated rather than written directly.

Migration tooling MUST distinguish:

source-written syntax

from:

expanded/generated syntax

Migration SHOULD occur at the appropriate pre-expansion or post-expansion boundary defined by the macro system.

Macro-generated code MUST NOT be silently rewritten in ways that alter macro semantics.

---

80. Migration and metaprogramming

Compile-time execution and metaprogramming MUST be version-aware.

A migration MUST account for:

- generated source;
- generated AST;
- compile-time APIs;
- reflection;
- specialization;
- compile-time evaluation.

If a metaprogram generates syntax affected by a migration, the migration contract MUST specify whether:

1. the generator migrates;
2. generated output migrates;
3. both migrate;
4. migration is prohibited until the generator is updated.

---

81. Migration and dialect registration

Dialect migrations MUST use explicit:

namespace
version
compatibility

metadata.

A dialect MUST NOT silently override core language migration rules.

Dialect registration MUST identify whether the dialect:

- extends;
- constrains;
- replaces;
- conflicts with;
- or merely annotates

core Zamani constructs.

---

82. Reserved syntax migration

Reserved syntax MUST NOT be assigned incompatible meanings without a versioned decision.

Reserved space exists to permit future evolution without unexpectedly breaking programs.

When reserved syntax becomes active:

1. compatibility analysis is mandatory;
2. identifier collision analysis is mandatory;
3. migration documentation is mandatory;
4. tests are mandatory.

---

83. Migration aliases

Aliases SHOULD be used where they reduce migration risk.

An alias MUST have:

- owner;
- introduction version;
- compatibility window;
- deprecation status if applicable;
- removal policy.

Aliases MUST NOT become an uncontrolled permanent namespace.

---

84. Compatibility shims

A compatibility shim MAY translate:

old representation
        ↓
current representation

A shim MUST:

- have a defined owner;
- have explicit version boundaries;
- preserve semantics;
- be tested;
- be observable in diagnostics/provenance where relevant;
- have a removal criterion.

A shim MUST NOT silently accumulate indefinitely.

---

85. Migration of generated files

Generated files MUST NOT be manually migrated when regeneration from authoritative sources is available.

Preferred:

authoritative source
    ↓
generator
    ↓
generated artifact

rather than:

old generated artifact
    ↓
manual migration

The generator version and source version MUST be sufficient to reproduce the generated result.

---

86. Migration of grammar fragments

When the grammar is modularized:

grammar/
├── lexer/
├── core/
├── types/
├── expressions/
├── quantum/
├── hdl/
...

fragment migration MUST preserve the public grammar contract.

Internal rule names MAY change without source migration when externally observable syntax and semantics remain unchanged.

---

87. Migration compatibility matrix

"grammar/compatibility/compatibility-matrix.md" SHOULD record at least:

Source| Target| Source| AST| Semantic| IR| Artifact| Migration
old| new| supported| supported| supported| supported| supported| automatic/assisted/manual
old| future| conditional| conditional| conditional| conditional| conditional| defined separately

The matrix MUST NOT claim compatibility merely because parsing succeeds.

---

88. Migration identifiers

Every non-trivial migration SHOULD receive a stable identifier.

Recommended conceptual format:

ZM-MIG-<domain>-<number>

Examples:

ZM-MIG-GRAMMAR-001
ZM-MIG-QUANTUM-001
ZM-MIG-HDL-001
ZM-MIG-IR-001

Identifiers MUST remain stable after publication.

---

89. Migration records

A migration record SHOULD contain:

Migration ID
Title
Status
Introduced In
Applies From
Applies To
Compatibility Class
Affected Domains
Affected Files
Old Contract
New Contract
Semantic Difference
Automatic Procedure
Manual Procedure
Diagnostics
Tests
Deprecation
Removal
Rollback

---

90. Migration completion contract

A migration is complete only when:

- source transformation is defined;
- semantic behavior is defined;
- compatibility classification is defined;
- AST impact is resolved;
- IR impact is resolved;
- downstream consumers are identified;
- tests exist;
- diagnostics exist;
- documentation exists;
- deprecation status exists where applicable;
- removal status exists where applicable;
- scalability audit passes;
- hard-coding audit passes;
- unsafe-code requirement remains satisfied;
- repository integration passes.

---

91. Hard-coding audit

Every migration MUST be audited for accidental hard-coding.

Search for:

MAX_QUBITS
MAX_CORES
MAX_THREADS
MAX_DEVICES
MAX_NODES
MAX_MEMORY
MAX_TENSOR_DIM
MAX_REGISTER
MAX_ACCELERATORS

and equivalent hidden assumptions.

Every discovered limit MUST be classified as:

1. language semantic requirement;
2. target requirement;
3. resource constraint;
4. implementation limit;
5. accidental hard-coding;
6. test-only limit;
7. representation-format limit;
8. documentation-only limit.

Only genuine semantic or representation limits may remain.

---

92. Representation limits

A representation may have technical limits.

For example, a serialized field may have a finite integer representation.

Such a limit MUST be documented as:

representation limit

and MUST NOT be described as:

Zamani language limit

Where practical, representations SHOULD use scalable encodings.

---

93. Infinite-scale interpretation

"Infinity" in POCO-REAF means that the language architecture MUST NOT impose an arbitrary finite machine-scale ceiling.

It does not mean:

- infinite physical memory;
- infinite execution time;
- infinite hardware;
- mathematically infinite allocation;
- guaranteed execution regardless of resources.

Actual execution remains constrained by:

- available resources;
- physical laws;
- target capabilities;
- implementation capacity;
- explicit resource policy.

The language architecture MUST remain scalable as those resources grow.

---

94. Migration and physical reality

A migration MUST distinguish:

semantic impossibility

from:

target cannot currently realize requirement

For example:

program requires a capability

followed by:

target lacks capability

is a target compatibility failure, not necessarily a source migration requirement.

---

95. Migration rollback

Where migration is destructive or lossy, tooling MUST preserve the original source or require an explicit backup policy.

A migration SHOULD be reversible when the transformation is formally reversible.

If not reversible, tooling MUST report that fact.

---

96. Lossy migration

Lossy migration MUST NEVER be presented as lossless.

The migration report MUST identify:

information lost
reason
semantic consequence
manual reconstruction requirements

Lossy migration SHOULD require explicit confirmation where tooling performs it automatically.

---

97. Semantic equivalence testing

Where practical, migration tests MUST compare:

old program semantics

with:

migrated program semantics

rather than merely comparing source text or AST structure.

Equivalent ASTs are not required when equivalent semantics can be established through different representations.

---

98. Differential testing

For supported migrations, implementations SHOULD perform differential testing:

old source
   |
   +--> old implementation
   |
   +--> migrate
          |
          v
       new source
          |
          v
     new implementation

The observable semantic result MUST agree within the declared semantic contract.

---

99. Quantum differential testing

Quantum migrations SHOULD compare:

- circuit semantics;
- operation ordering;
- measurement semantics;
- classical conditions;
- observable behavior;
- state-transition semantics where representable;
- resource requirements.

The comparison MUST account for legitimate representation changes.

---

100. HDL differential testing

HDL migrations SHOULD compare:

- combinational behavior;
- sequential behavior;
- state transitions;
- timing semantics;
- interface behavior.

A target-specific synthesis result MUST NOT be mistaken for the language semantic representation.

---

101. Distributed differential testing

Distributed migrations MUST test:

- ordering;
- consistency;
- fault semantics;
- message semantics;
- replication;
- service behavior.

Tests MUST not assume a fixed number of nodes unless the tested semantic construct itself explicitly requires one.

---

102. Migration security

Migration tooling MUST treat input source and artifacts as potentially untrusted.

It MUST NOT:

- execute arbitrary source as part of syntax migration without an explicit safe execution model;
- trust artifact metadata blindly;
- silently elevate privileges;
- disable security checks;
- bypass capability validation.

Compile-time execution and metaprogramming migrations require explicit security boundaries.

---

103. Migration determinism and provenance

For reproducible builds:

migration input
+
migration rules
+
tool version
+
language contract

MUST be sufficient to explain the transformation.

Environment-dependent behavior MUST be explicitly declared.

---

104. Migration and compiler caching

Caches MUST include all migration-relevant identity information.

A cache key MUST NOT rely only on:

source filename

It SHOULD account for:

- source content;
- language version;
- migration rules;
- dialect versions;
- compiler/tool version where relevant;
- relevant semantic contracts.

Otherwise an old migrated result could be incorrectly reused.

---

105. Migration and reproducible builds

A migration MUST be reproducible under the same declared inputs.

If timestamps or environmental values affect output, they MUST be excluded or explicitly modeled.

Migration tooling SHOULD support deterministic output suitable for source control and reproducible builds.

---

106. Migration and source control

Migration changes SHOULD be reviewable as source transformations.

Tooling SHOULD provide:

- before;
- after;
- migration ID;
- semantic warning;
- unresolved decisions.

Generated-only changes SHOULD be clearly identifiable.

---

107. Migration and CI

CI MUST validate migrations that affect production grammar.

At minimum:

grammar validation
lexer tests
parser tests
AST tests
semantic tests
compatibility tests
migration tests
negative tests
boundary tests
scalability tests
cross-domain tests
determinism tests

Where applicable:

quantum tests
HDL tests
hardware tests
distributed tests
IR tests
artifact tests

must also run.

---

108. Migration gate

A migration MUST NOT be released as production-ready if:

- semantic behavior is undefined;
- compatibility classification is missing;
- migration tests are missing;
- affected downstream consumers are unknown;
- hard-coded scalability restrictions were introduced;
- unsafe Rust is required;
- canonical IR ownership is violated;
- target-specific behavior has been promoted into universal source semantics.

---

109. Migration review checklist

Before accepting a migration, reviewers MUST answer:

Language

- Is the affected language version identified?
- Is the change classified correctly?
- Is source compatibility understood?

Grammar

- Is the grammar change deterministic?
- Are lexer conflicts resolved?
- Are parser ambiguities resolved?

AST

- Is all semantic information preserved?
- Are source locations preserved where required?

Semantics

- Is meaning preserved?
- If not, is the change explicitly breaking?

Quantum

- Is quantum meaning preserved?
- Does "quantum::ir" remain canonical?

Hardware

- Are physical target details kept out of portable semantics?

Resources

- Are requirements separate from capabilities and allocation?

Scalability

- Were arbitrary finite limits avoided?

Rust

- Does implementation remain compatible with Rust 1.97/1.97.1?
- Is "unsafe" absent?

Tooling

- Are diagnostics deterministic?
- Is provenance available?

Tests

- Are positive, negative, boundary, cross-domain, determinism, and compatibility tests present?

---

110. Repository integration contract

This file integrates with the repository as follows:

grammar/specification/language-version.md
    -> defines versions

grammar/specification/compatibility.md
    -> defines compatibility dimensions

grammar/compatibility/migrations.md
    -> defines migration procedures

grammar/compatibility/deprecated.md
    -> records deprecated features

grammar/compatibility/reserved.md
    -> records reserved space

grammar/compatibility/compatibility-matrix.md
    -> records compatibility relationships

grammar/validation/compatibility-rules.md
    -> validates compatibility

grammar/specification/grammar-authority.md
    -> determines authority

grammar/Zamani.g4
    -> implements syntax

lexer/parser
    -> recognizes source

AST
    -> represents structure

semantic analysis
    -> establishes meaning

canonical IR
    -> represents semantics

quantum::ir
    -> canonical quantum semantic boundary

QEC / ZQN / optimization / routing / scheduling
    -> consume canonical semantic/IR contracts

HAL / hardware
    -> realizes target capabilities

runtime
    -> executes according to semantic contract

No circular dependency is permitted.

---

111. File ownership

This file owns:

- migration policy;
- migration classification;
- migration procedures;
- migration invariants;
- migration validation;
- migration testing;
- migration provenance requirements.

This file does NOT own:

- grammar productions;
- lexer token definitions;
- AST implementation;
- semantic type-checking implementation;
- quantum IR implementation;
- QEC algorithms;
- scheduling algorithms;
- routing;
- calibration;
- hardware drivers;
- runtime execution;
- package dependency resolution.

Those remain owned by their respective components.

---

112. Downstream consumers

This document is consumed by:

- grammar maintainers;
- lexer maintainers;
- parser maintainers;
- AST maintainers;
- semantic-analysis maintainers;
- type-system maintainers;
- dialect maintainers;
- compiler maintainers;
- IR maintainers;
- quantum subsystem maintainers;
- QEC maintainers;
- hardware/HAL maintainers;
- runtime maintainers;
- tooling maintainers;
- documentation maintainers;
- compatibility-test maintainers.

---

113. Upstream contracts

This document depends on:

grammar/specification/language-version.md
grammar/specification/compatibility.md
grammar/specification/grammar-authority.md
grammar/specification/syntax-model.md
grammar/specification/semantic-model.md
grammar/specification/compilation-model.md
grammar/specification/execution-model.md
grammar/specification/scalability-model.md

It MUST NOT redefine those documents.

---

114. Public contract

The public migration contract is:

A Zamani migration MUST explicitly identify
what changed,
why it changed,
whether semantics changed,
how existing programs migrate,
which layers are affected,
and how compatibility is verified.

No migration may rely on undocumented behavior.

---

115. Completion criteria

"grammar/compatibility/migrations.md" is complete when:

- migration terminology is defined;
- migration classes are defined;
- source migration is defined;
- grammar migration is defined;
- AST migration is defined;
- semantic migration is defined;
- IR migration is defined;
- quantum migration is defined;
- HDL migration is defined;
- hardware migration is defined;
- distributed migration is defined;
- AI/data migration is defined;
- networking migration is defined;
- security migration is defined;
- dialect migration is defined;
- artifact migration is defined;
- ABI migration is defined;
- runtime migration is defined;
- target migration is defined;
- deprecation/removal lifecycle is defined;
- migration provenance is defined;
- deterministic migration is defined;
- scalability requirements are defined;
- hard-coding policy is defined;
- Rust 1.97/1.97.1 compatibility is defined;
- unsafe Rust prohibition is explicit;
- testing requirements are defined;
- repository integration is defined;
- ownership boundaries are defined;
- no duplicate semantic architecture is introduced.

---

116. Final migration principle

Zamani migration exists to preserve the language while allowing the language to evolve.

The governing architecture is:

OLD PROGRAM
     |
     v
VERSION IDENTIFICATION
     |
     v
COMPATIBILITY ANALYSIS
     |
     v
MIGRATION PLAN
     |
     +------------------+
     |                  |
     v                  v
MECHANICAL          SEMANTIC
TRANSFORMATION      REVIEW
     |                  |
     +---------+--------+
               |
               v
        CURRENT SOURCE
               |
               v
             AST
               |
               v
       SEMANTIC MODEL
               |
               v
        CANONICAL IR
               |
       +-------+--------+
       |       |        |
       v       v        v
   classical quantum hardware
       |       |        |
       +-------+--------+
               |
               v
       target realization
               |
               v
            runtime

The permanent invariant is:

«Zamani source expresses computation, intent, semantics, capabilities, requirements, and constraints—not arbitrary limitations of the machine available at the time the program was written.»

Therefore:

Program Once
      ↓
Stable Semantics
      ↓
Versioned Evolution
      ↓
Deterministic Migration
      ↓
Canonical IR
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

This is the migration foundation required for:

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever

and for Zamani to evolve from:

Atom → Everywhere

without sacrificing semantic stability, portability, quantum/classical interoperability, HDL capability, hardware independence, scalability, or long-term compatibility.