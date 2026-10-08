Zamani Dialect Version Contract

Path: "grammar/compatibility/dialect-version.md"
Status: Normative
Scope: Dialect identity, dialect version identity, dialect-version semantics, version ordering, compatibility classification, version constraints, resolution, evolution, migration, deprecation, feature gating, provenance, reproducibility, AST/semantic/IR compatibility, cross-dialect composition, POCO-REAF, and production integration
Grammar technology: ANTLR4-compatible
Rust implementation baseline: Rust 1.97 or later
Rust edition: 2021
Rust safety requirement: Production Zamani Rust implementation MUST use safe Rust. "unsafe" Rust MUST NOT be required or used by the Zamani implementation.
Primary portability objective: Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)

---

1. Purpose

This document defines the normative contract for dialect version identity and evolution in Zamani.

It answers:

- What is a dialect version?
- What does a dialect version identify?
- How is a dialect version related to the Zamani language version?
- How is a dialect version related to AST, semantic, IR, compiler, runtime, ABI, and target versions?
- How are dialect versions compared?
- How are compatible versions selected?
- How are incompatible versions rejected?
- How are dialect versions evolved?
- How are old dialect versions migrated?
- How are experimental and deprecated dialect versions handled?
- How are dialect versions recorded in provenance?
- How are dialect versions preserved through compilation?
- How does dialect versioning preserve POCO-REAF?
- How can dialects scale from tiny programs to arbitrarily large programs and systems without language-level capacity ceilings?

This file is a compatibility/versioning contract.

It is not the dialect grammar.

It is not the complete dialect compatibility policy.

It is not the language-version specification.

It is not an implementation algorithm.

It does not define concrete dialect syntax.

---

2. Architectural Position

The authoritative relationship is:

grammar/DESIGN.md
        |
        v
grammar/specification/
        |
        +--> language meaning
        |
        v
grammar/spec/
        |
        +--> cross-layer contracts
        |
        v
grammar/compatibility/
        |
        +--> versioning
        +--> compatibility
        +--> migration
        +--> deprecation
        |
        +--> THIS FILE
        |    dialect-version semantics
        |
        v
grammar/dialects/
        |
        +--> dialect syntax
        +--> dialect registration
        +--> dialect version syntax
        +--> dialect compatibility syntax
        |
        v
grammar/Zamani.g4
        |
        v
Lexer
        |
        v
Parser
        |
        v
Domain-neutral AST
        |
        v
Structural validation
        |
        +--> types
        +--> effects
        +--> capabilities
        +--> resources
        +--> contracts
        +--> policies
        +--> provenance
        |
        v
Semantic analysis
        |
        v
Canonical semantic representation
        |
        +--------------------------+
        |                          |
        v                          v
   Classical IR              quantum::ir
        |                          |
        +------------+-------------+
                     |
                     v
                Optimization
                     |
                     v
             Lowering / Routing
                     |
                     v
                 Scheduling
                     |
                     v
        Resilience / Recovery / QEC
                     |
                     v
                    ZQN
                     |
                     v
                    HAL
                     |
                     v
                  Target

The dialect-version contract applies primarily at the dialect identity and semantic-contract boundary.

It MUST NOT move target-specific version decisions into source syntax.

---

3. Ownership

3.1 This file owns

This file owns:

1. dialect-version identity;
2. dialect-version structure;
3. dialect-version comparison semantics;
4. dialect-version ordering;
5. dialect-version compatibility classification;
6. dialect-version requirement semantics;
7. dialect-version resolution requirements;
8. dialect-version evolution rules;
9. dialect-version compatibility guarantees;
10. dialect-version migration requirements;
11. dialect-version deprecation interaction;
12. dialect-version feature-state interaction;
13. dialect-version provenance requirements;
14. dialect-version reproducibility requirements;
15. dialect-version interaction with language versions;
16. dialect-version interaction with semantic contracts;
17. dialect-version interaction with canonical IR contracts;
18. dialect-version interaction with POCO-REAF.

3.2 This file does not own

This file does NOT own:

- dialect concrete syntax;
- lexical tokens;
- parser productions;
- AST implementation;
- semantic implementation algorithms;
- IR implementation;
- target discovery;
- hardware selection;
- resource allocation;
- routing;
- scheduling;
- QEC;
- ZQN;
- HAL;
- runtime implementation;
- compiler implementation details;
- Rust implementation details beyond the safety/baseline contract.

---

4. Companion-File Authority

The following ownership is mandatory.

File| Owns
"grammar/DESIGN.md"| Overall architecture and architectural invariants
"grammar/specification/"| Normative language meaning
"grammar/specification/language-version.md"| Zamani language-version semantics
"grammar/spec/versioning.md"| Cross-layer versioning model
"grammar/spec/compatibility.md"| General compatibility semantics
"grammar/compatibility/language-version.md"| Language-version compatibility
"grammar/compatibility/semantic-version.md"| Semantic-contract versioning
"grammar/compatibility/grammar-version.md"| Grammar-contract versioning
"grammar/compatibility/dialect-version.md"| Dialect-version semantics — this file
"grammar/compatibility/dialect-compatibility.md"| Broader dialect compatibility
"grammar/compatibility/dialects.md"| Dialect compatibility/conformance inventory
"grammar/compatibility/versions.md"| General version/release policy
"grammar/compatibility/migrations.md"| Migration procedures
"grammar/compatibility/deprecated.md"| Deprecation lifecycle
"grammar/compatibility/feature-gates.md"| Feature-state/lifecycle gating
"grammar/compatibility/compatibility-matrix.md"| Cross-layer compatibility relationships
"grammar/dialects/README.md"| Dialect architecture/navigation
"grammar/dialects/dialects.g4"| Dialect composition façade
"grammar/dialects/dialect.g4"| Public dialect grammar boundary
"grammar/dialects/registration.g4"| Dialect registration syntax
"grammar/dialects/versioning.g4"| Dialect-version syntax
"grammar/dialects/compatibility.g4"| Dialect compatibility syntax
"grammar/dialects/namespaces.g4"| Dialect namespace syntax
"grammar/Zamani.g4"| Canonical grammar composition root
"src/" frontend| Executable implementation
canonical semantic model| Semantic interpretation
canonical IR| Lower-level representation
"quantum::ir"| Canonical quantum semantic IR
backend/HAL/runtime| Target realization

No lower-level file may redefine the semantics established here.

---

5. Relationship to "dialect-compatibility.md"

"grammar/compatibility/dialect-version.md" and "grammar/compatibility/dialect-compatibility.md" are complementary.

They MUST NOT become duplicate specifications.

This file answers:

«What does a dialect version mean and how does a dialect version evolve?»

"dialect-compatibility.md" answers:

«Given dialects and versions, are their syntax, semantics, capabilities, resources, effects, IR mappings, and dependencies compatible?»

The relationship is:

dialect-version.md
        |
        +--> defines version identity
        +--> defines ordering
        +--> defines evolution
        +--> defines version compatibility semantics
        |
        v
dialect-compatibility.md
        |
        +--> applies compatibility to dialect composition
        +--> checks dependencies
        +--> checks semantic compatibility
        +--> checks IR compatibility
        +--> checks capability/resource compatibility

Neither file may silently replace the other.

---

6. Dialect Definition

A dialect is a named, versioned, explicitly scoped extension of the Zamani language contract.

A dialect MAY provide:

- syntax;
- declarations;
- expressions;
- statements;
- types;
- effects;
- capabilities;
- resource requirements;
- domain semantics;
- interoperability;
- semantic operations;
- lowering contracts.

A dialect MUST eventually map into Zamani's common compilation architecture.

A dialect MUST NOT become an unrelated programming language hidden behind the word "dialect".

---

7. Dialect Identity

A dialect identity is distinct from its version.

Conceptually:

DialectIdentity
    =
    Namespace
    +
    Name

and:

DialectVersionedIdentity
    =
    DialectIdentity
    +
    DialectVersion

For example:

quantum::standard

and:

quantum::standard @ 1.2.0

are different levels of identity.

The namespace/name identifies what dialect is being referenced.

The version identifies which contract of that dialect is being referenced.

---

8. Dialect Identity MUST Be Open-World

The grammar MUST NOT contain a finite list of all dialects.

It MUST NOT require rules such as:

quantum
openqasm
cuda
verilog
...

as a universal enumeration.

Dialect identities MUST remain symbolic.

Examples:

quantum::standard
quantum::openqasm
classical::numeric
hdl::rtl
hardware::fpga
ai::tensor
distributed::messaging
future::computing
vendor::domain::extension

These are examples only.

They do not constitute a closed registry.

A new dialect MUST be registrable without changing the universal grammar merely because its identity is new.

---

9. Dialect Version Identity

A dialect version identifies a particular public dialect contract.

It MAY cover:

- syntax;
- AST mapping;
- type semantics;
- effect semantics;
- capability semantics;
- resource semantics;
- evaluation semantics;
- interoperability semantics;
- canonical semantic mappings;
- canonical IR mappings;
- diagnostics;
- compatibility behavior.

It does NOT identify:

- a particular CPU;
- a particular GPU;
- a particular FPGA;
- a particular ASIC;
- a particular QPU;
- a physical qubit;
- a memory bank;
- a cluster;
- a cloud provider;
- a runtime process;
- a device firmware revision;
- a calibration state;
- a scheduler instance.

Those are target/runtime concerns.

---

10. Dialect Version MUST Be Separate From Other Versions

Zamani MUST distinguish at least:

LanguageVersion
GrammarVersion
LexerVersion
ASTVersion
SemanticModelVersion
TypeSystemVersion
EffectSystemVersion
ResourceCapabilityVersion
ClassicalIRVersion
QuantumIRVersion
HDLIRVersion
DialectVersion
PackageVersion
APIVersion
ABIVersion
ArtifactVersion
SerializationVersion
CompilerVersion
RuntimeVersion
TargetDescriptorVersion
BackendVersion
ToolchainVersion

A change to one does not automatically change all others.

For example:

new compiler
    !=
new dialect version

new GPU backend
    !=
new dialect version

new QPU
    !=
new dialect version

new quantum::ir encoding
    !=
new dialect source contract

unless the relevant public dialect semantics actually changed.

---

11. Dialect Version Format

Stable dialects SHOULD use:

MAJOR.MINOR.PATCH

Optional pre-release and build metadata MAY be used according to the repository's normative versioning model.

Examples:

1.0.0
1.1.0
1.1.1
2.0.0
2.0.0-alpha
2.0.0-beta
2.0.0-rc.1
2.0.0+build.7
2.0.0-rc.1+build.7

The concrete syntax is owned by:

grammar/dialects/versioning.g4

This file defines the semantics.

---

12. No Artificial Version Limits

The versioning architecture MUST NOT impose universal limits such as:

MAX_DIALECT_VERSION
MAX_MAJOR
MAX_MINOR
MAX_PATCH
MAX_DIALECTS
MAX_VERSION_REQUIREMENTS
MAX_DEPENDENCIES
MAX_COMPOSITION_DEPTH

The language MUST NOT encode finite hardware-style capacity ceilings into version semantics.

Version components may grow according to the actual representation and validation implementation.

If a particular implementation cannot represent an input version, that is an implementation limitation and MUST NOT silently become a language-level restriction.

---

13. Version Components

A stable dialect version consists conceptually of:

major
minor
patch
pre_release?
build_metadata?

The semantic interpretation is:

major
    breaking evolution

minor
    compatible additive evolution

patch
    compatible corrective evolution

Pre-release and build metadata do not automatically imply semantic incompatibility.

---

14. Major Dialect Version

A MAJOR version increment is required when stable semantics become intentionally incompatible.

Examples include:

- changing the meaning of an existing stable operation;
- removing stable syntax;
- changing stable type meaning;
- changing stable ownership behavior;
- changing stable effect behavior;
- changing stable resource semantics;
- changing stable capability semantics;
- changing stable evaluation behavior;
- changing stable interoperability meaning;
- changing a stable mapping to canonical semantic representation incompatibly;
- changing a stable mapping to canonical IR incompatibly.

Example:

1.4.3
    ->
2.0.0

may be breaking.

A major version MUST NOT be required merely because an implementation changed.

---

15. Minor Dialect Version

A MINOR version increment is used for compatible additions.

Examples:

- new optional constructs;
- new capabilities;
- new optional attributes;
- new additive operations;
- new non-conflicting syntax;
- new interoperability mappings;
- new domain functionality;
- new optional resource declarations;
- new metadata;
- new diagnostics that do not invalidate valid programs.

Example:

1.4.3
    ->
1.5.0

Existing valid programs under "1.4.x" MUST retain their specified meaning when interpreted under the compatible "1.5.x" contract.

---

16. Patch Dialect Version

A PATCH increment is used for compatible corrections.

Examples:

- implementation corrections;
- parser corrections;
- diagnostic corrections;
- documentation corrections;
- compatibility metadata corrections;
- conformance corrections;
- deterministic-behavior corrections where determinism was already specified;
- migration documentation corrections.

Example:

1.4.3
    ->
1.4.4

A patch release MUST NOT intentionally redefine stable dialect semantics.

---

17. Semantic Compatibility Is the Primary Test

Version-number similarity is not enough.

The fundamental invariant is:

«A dialect version advertised as compatible MUST NOT silently change the specified meaning of valid source.»

Therefore:

same syntax
    !=
same semantics

and:

different syntax
    !=
different semantics

A migration or normalization MAY change source representation while preserving semantics.

---

18. Compatibility Classes

Dialect-version compatibility MUST distinguish at least:

EXACT
BACKWARD_COMPATIBLE
FORWARD_COMPATIBLE
SOURCE_COMPATIBLE
AST_COMPATIBLE
SEMANTICALLY_COMPATIBLE
IR_COMPATIBLE
ARTIFACT_COMPATIBLE
RUNTIME_COMPATIBLE
TARGET_COMPATIBLE
MIGRATABLE
CONDITIONALLY_COMPATIBLE
INCOMPATIBLE
UNKNOWN

These classifications MUST NOT be collapsed into one Boolean value internally.

For example:

semantic compatibility = true
target feasibility = false

is a valid result.

It means the dialect contract is compatible, but the selected target cannot realize the program under the current resource/capability conditions.

---

19. Exact Compatibility

"EXACT" means the requested dialect version and resolved dialect version identify the same version contract.

Example:

required: 1.2.3
resolved: 1.2.3

This is stronger than ordinary compatibility.

---

20. Backward Compatibility

A newer dialect version is backward-compatible with an older version when programs valid under the older stable contract retain their specified meaning under the newer contract.

Example:

dialect 1.2.0
    ->
dialect 1.3.0

may be backward-compatible.

Backward compatibility MUST be established semantically, not inferred solely from numeric ordering.

---

21. Forward Compatibility

Forward compatibility means an older implementation can safely process a newer dialect contract within an explicitly defined compatibility subset.

It MUST NOT be assumed merely because:

major == major

or:

minor <= minor

Unknown constructs MUST NOT be silently interpreted as known constructs.

If a future construct is outside the supported compatibility subset, the implementation MUST produce a deterministic diagnostic or use an explicitly specified preservation mechanism.

---

22. Unknown Versions MUST Fail Closed

If a compiler cannot determine the semantics of a dialect version, it MUST NOT guess.

For example:

dialect X @ 7.4.2

when the implementation only understands versions through "6.x" MUST NOT silently treat "7.4.2" as "6.x".

The compiler MUST produce a deterministic diagnostic such as:

UNKNOWN_DIALECT_VERSION

unless an explicit compatibility contract establishes safe interpretation.

---

23. Dialect Version Constraints

A dialect dependency MAY specify a version constraint.

Conceptually:

dialect::example >= 1.2.0

or:

dialect::example >= 1.2.0, < 2.0.0

The exact source syntax is owned by:

grammar/dialects/versioning.g4
grammar/dialects/compatibility.g4

This file defines only the semantic meaning.

---

24. Version Constraint Domain

A version constraint MUST identify what it applies to.

The implementation MUST distinguish:

language
dialect
grammar
compiler
runtime
package
API
ABI
IR
artifact
target descriptor

A bare numeric version MUST NOT be interpreted as a dialect version merely because it appears near a dialect declaration.

The subject must remain known throughout semantic analysis.

---

25. Version Resolution

Dialect-version resolution MUST be:

- deterministic;
- explicit;
- reproducible;
- version-aware;
- conflict-aware;
- provenance-preserving;
- independent of hardware accident.

Resolution may consider:

dialect identity
required version/range
language version
dialect dependencies
available dialect contracts
feature gates
compatibility declarations
migration rules
policy

Resolution MUST NOT silently depend on:

- CPU model;
- GPU model;
- QPU model;
- filesystem ordering;
- hash-map ordering;
- current wall-clock time;
- locale;
- random state;
- network response ordering.

If external discovery is part of the implementation, its resulting input set MUST be normalized before deterministic resolution.

---

26. Version Resolution Is Not Target Resolution

These are different operations.

Dialect version resolution
        |
        v
Which dialect contract applies?

versus:

Target resolution
        |
        v
Where/how can that contract be realized?

The second may consider:

- CPU;
- GPU;
- FPGA;
- ASIC;
- accelerator;
- QPU;
- simulator;
- cluster;
- distributed topology;
- memory;
- network;
- power;
- thermal conditions;
- reliability;
- available resources.

The dialect-version system MUST NOT perform target selection.

---

27. Dialect Version and Language Version

A dialect version MUST remain subordinate to the Zamani language contract.

Conceptually:

Zamani language 1.x
        |
        +--> dialect A 2.x
        +--> dialect B 1.x
        +--> dialect C 4.x

A dialect may support multiple Zamani language versions.

For example:

dialect X 2.0
    supports Zamani >= 1.0, < 2.0

The dialect version and language version remain independent dimensions.

---

28. Core-Language Compatibility

A dialect version MUST declare or otherwise establish the Zamani language contract(s) under which it is valid.

A dialect MUST NOT silently assume that all future language versions preserve its assumptions.

If a future language version is unknown:

UNKNOWN_CORE_LANGUAGE_VERSION

MUST be diagnosed unless an explicit compatibility contract covers it.

---

29. Dialect Version and Grammar Version

These are distinct.

DialectVersion
    =
dialect semantic/public contract

GrammarVersion
    =
grammar representation contract

A grammar implementation may evolve without changing dialect semantics.

Conversely, dialect semantics may change even if the grammar happens to remain textually unchanged.

Therefore:

grammar version == dialect version

MUST NOT be assumed.

---

30. Dialect Version and AST Version

A dialect version MAY depend on a particular AST contract.

However:

AST version
    !=
dialect version

A compatible AST normalization may permit multiple dialect versions to map to one AST version.

Likewise, one dialect version may be represented by different internal AST implementations provided the semantic contract is preserved.

---

31. Dialect Version and Semantic Model Version

The semantic model is the authoritative interpretation after parsing.

A dialect version MUST identify any semantic-model requirements necessary to interpret the dialect correctly.

For example:

dialect X @ 2.0
requires semantic model >= 1.4

The exact representation belongs to the semantic/versioning contracts.

---

32. Dialect Version and Type-System Version

If a dialect relies on a particular type-system feature, the dependency MUST be represented explicitly.

Examples:

generic types
associated types
linear types
affine types
dependent constraints
tensor types
quantum state types
hardware intent types

The dialect MUST NOT infer type compatibility merely from parser acceptance.

---

33. Dialect Version and Effect-System Version

If dialect constructs depend on effects, the version contract MUST preserve their effect semantics.

Examples:

io
network
mutation
randomness
native
foreign
distributed
measurement
quantum
learning
adaptation
reflection
code_generation
simulation

Changing the meaning of a stable effect can be a dialect breaking change.

---

34. Dialect Version and Capability Version

Capabilities are separate from dialect versions.

For example:

dialect quantum::standard @ 2.0.0

does not imply:

capability("quantum.measurement")

is physically available.

The dialect version describes the contract.

Capability negotiation determines whether the target can realize the required operation.

---

35. Dialect Version and Resource Requirements

Dialect versioning MUST NOT encode fixed machine capacities.

The following are prohibited as dialect-version semantics:

MAX_QUBITS
MAX_CPUS
MAX_GPUS
MAX_FPGAS
MAX_NODES
MAX_MEMORY
MAX_THREADS
MAX_REGISTER_WIDTH
MAX_TENSOR_RANK
MAX_NETWORK_SIZE
MAX_DEVICE_COUNT

A dialect MAY express resource requirements.

For example:

requires qubits >= n;
requires memory >= required_memory;
requires capability("quantum.measurement");
requires capability("tensor.compute");
requires topology(required_topology);

These are requirements.

They are not universal limits.

---

36. Dialect Version and POCO-REAF

Dialect versioning MUST preserve the distinction between:

portable program meaning

and:

target realization

A dialect version MUST NOT encode assumptions that force source programs to be rewritten merely because the target grows from:

tiny

to:

larger

or from:

single device

to:

heterogeneous/distributed execution

The same semantic program may be realized differently according to:

- capabilities;
- resources;
- topology;
- policies;
- optimization;
- routing;
- scheduling;
- resilience;
- target availability.

---

37. Dialect Version Does Not Select Hardware

This is invalid:

dialect quantum::standard @ 1.0.0

being interpreted as:

use QPU model X

Likewise:

dialect hardware::gpu @ 2.0.0

MUST NOT mean:

use GPU number 3

Dialect versions describe language/domain contracts.

Hardware selection remains downstream.

---

38. Dialect Composition

Multiple dialects may be composed.

For example:

classical::numeric
quantum::standard
ai::tensor
distributed::execution

may participate in one source program.

There MUST be no universal maximum number of dialects.

Actual implementation resource exhaustion is an implementation constraint, not a language-level dialect limit.

---

39. Composition Version Resolution

Every composed dialect MUST have a resolved version.

Conceptually:

Dialect A @ 1.x
Dialect B @ 3.x
Dialect C @ 2.x

The resolver MUST validate:

A ↔ B
B ↔ C
A ↔ C

and all declared dependencies.

Pairwise checks alone are insufficient when a combined semantic constraint exists.

The final composition MUST therefore be validated as a whole.

---

40. Composition Conflicts

If two dialect versions provide incompatible semantics, resolution MUST fail unless an explicit deterministic composition rule exists.

Possible diagnostics include:

DIALECT_VERSION_CONFLICT
DIALECT_SEMANTIC_CONFLICT
DIALECT_TYPE_CONFLICT
DIALECT_EFFECT_CONFLICT
DIALECT_CAPABILITY_CONFLICT
DIALECT_RESOURCE_CONFLICT
DIALECT_IR_CONFLICT
DIALECT_LOWERING_CONFLICT

The exact diagnostic owner remains the diagnostics subsystem.

---

41. No Implicit Conflict Resolution

Dialect conflicts MUST NOT be resolved by:

- declaration order;
- import order;
- filesystem order;
- hash-map iteration;
- compiler implementation accident;
- target availability;
- optimization level;
- current hardware;
- runtime state.

If precedence is required, it MUST be explicitly specified, deterministic, versioned, and tested.

---

42. Dependency Versions

A dialect dependency MUST identify:

dependency dialect identity
+
accepted version range
+
compatibility requirements

For example:

A @ 2.0
    requires
B >= 1.4, < 2.0

A resolver MUST NOT silently substitute:

B 3.0

unless the compatibility contract explicitly proves that substitution valid.

---

43. Dependency Conflicts

Suppose:

A requires B >= 1.0, < 2.0
C requires B >= 3.0, < 4.0

If there is no compatible intersection, resolution MUST fail.

The implementation MUST NOT arbitrarily select one version.

The failure MUST be deterministic.

---

44. Dependency Cycles

Dialect dependencies MAY form graphs.

The implementation MUST support arbitrary graph depth subject only to available implementation resources.

It MUST detect cycles.

Example:

A -> B
B -> C
C -> A

must produce a deterministic cycle diagnostic.

No fixed maximum dependency depth may be encoded as language semantics.

---

45. Version Selection

When multiple compatible versions exist, selection MUST be deterministic.

The resolver SHOULD prefer:

1. exact requirements;
2. explicitly pinned requirements;
3. the highest compatible stable version when the contract permits such selection;
4. explicitly permitted pre-release versions only when requested/allowed.

The exact selection policy MUST be centralized rather than independently implemented by every dialect.

---

46. Reproducibility

A reproducible build MUST preserve the resolved dialect version set.

A reproducibility record SHOULD contain:

language version
dialect identity
dialect version
dialect dependencies
resolved dependency versions
feature states
grammar contract version
semantic contract version
IR contract versions
compiler contract
relevant policy identifiers
source provenance

A build MUST NOT depend silently on an ambient dialect version.

---

47. Locking

A toolchain MAY provide a lock mechanism for reproducible dependency resolution.

A lock file MUST record resolved identities and versions.

It MUST NOT turn implementation-specific lock data into language syntax.

Locking is a tooling/build concern.

---

48. Provenance

Every resolved dialect version SHOULD be available to provenance tooling.

Where provenance is enabled, the compiler SHOULD be able to record:

dialect identity
dialect version
source declaration
resolved dependency
compatibility decision
migration applied
feature gate
semantic transformation
IR mapping

This allows a compiled artifact to answer:

«Which dialect contract was used to interpret this source?»

---

49. Semantic Fingerprint

A dialect implementation MAY provide a semantic fingerprint for a dialect contract.

The fingerprint MUST represent the public semantic contract, not:

- compiler memory addresses;
- filesystem paths;
- process IDs;
- device IDs;
- random values;
- build timestamps unless explicitly part of the artifact identity.

If a fingerprint is used for compatibility verification, its algorithm and input domain MUST be specified.

---

50. Build Metadata

Build metadata MUST NOT alter semantic compatibility unless the relevant contract explicitly states otherwise.

For example:

1.2.3+build.100

and:

1.2.3+build.101

may represent the same dialect semantics.

Build metadata may identify an implementation artifact.

It MUST NOT silently redefine dialect meaning.

---

51. Pre-release Versions

Pre-release versions are not automatically stable compatibility contracts.

Examples:

2.0.0-alpha
2.0.0-beta
2.0.0-rc.1

A compiler MUST NOT assume:

2.0.0-alpha

is compatible with:

2.0.0

unless explicitly specified.

Pre-release compatibility MUST be declared or governed by the repository's version policy.

---

52. Experimental Dialects

Experimental dialects MAY evolve without stable compatibility guarantees.

However, every experimental dialect SHOULD still identify:

identity
version
status
owner
language dependency
semantic scope
compatibility expectations

Experimental status MUST NOT be used to bypass:

- parser validation;
- semantic validation;
- type validation;
- effect validation;
- capability validation;
- resource validation;
- policy validation;
- provenance where required.

---

53. Feature Gates

Dialect versions may expose features through the repository's feature-gating mechanism.

The distinction is:

dialect version
    =
contract identity

versus:

feature gate
    =
availability/status of a construct

A feature may be:

stable
experimental
deprecated
removed
implementation-defined
unsupported

The canonical feature-state policy belongs to:

grammar/compatibility/feature-gates.md

This file only defines the dialect-version interaction.

---

54. Deprecated Dialect Versions

A dialect version MAY become deprecated.

Deprecation MUST identify:

dialect identity
dialect version
deprecation status
reason
replacement
migration path
removal policy

Deprecation MUST NOT silently change semantics.

A deprecated dialect version remains identifiable until its contract is explicitly removed from the supported compatibility set.

---

55. Removal

Removal of a dialect version MUST be explicit.

The compiler SHOULD distinguish:

DEPRECATED_DIALECT_VERSION

from:

UNSUPPORTED_DIALECT_VERSION

and:

UNKNOWN_DIALECT_VERSION

These are different states.

---

56. Migration

Migration from one dialect version to another MUST preserve semantics where compatibility permits.

The migration pipeline is:

source
   |
   v
dialect version identification
   |
   v
compatibility analysis
   |
   v
migration plan
   |
   v
source/AST normalization
   |
   v
semantic validation
   |
   v
new dialect contract

Migration MUST NOT silently change program meaning.

---

57. Automatic Migration

Automatic migration MAY be provided when the transformation is:

- deterministic;
- semantics-preserving;
- fully specified;
- diagnostically transparent;
- reversible where required by the migration contract.

A migration tool MUST report transformations that could affect semantics.

---

58. Migration MUST NOT Hide Breaking Changes

A migration from:

dialect A @ 1.x

to:

dialect A @ 2.x

does not make the versions themselves compatible.

It means:

old contract
    ->
explicit migration
    ->
new contract

The migration artifact and resulting source/AST MUST identify the resulting dialect version.

---

59. Source Compatibility

Source compatibility means source can be accepted without migration.

It is distinct from:

semantic compatibility

A dialect MAY require source rewriting while preserving semantics.

In that case:

source compatibility = false
semantic compatibility = true
migratable = true

---

60. AST Compatibility

AST compatibility means the dialect's frontend representation remains compatible with the relevant AST contract.

A dialect-version change MAY require AST normalization without being semantically breaking.

The AST layer MUST preserve enough source provenance to explain such normalization where required.

---

61. Semantic Compatibility

Semantic compatibility is the highest-priority compatibility classification before target realization.

A dialect version is semantically compatible only when stable program meaning remains preserved.

This includes, where applicable:

- type semantics;
- evaluation;
- effects;
- concurrency;
- resource intent;
- capability intent;
- numerical semantics;
- quantum semantics;
- measurement;
- classical/quantum interaction;
- HDL intent;
- hardware intent;
- distributed semantics;
- networking semantics;
- interoperability semantics.

---

62. Canonical IR Compatibility

A dialect version MUST declare the canonical IR boundary it requires where relevant.

For quantum dialects, the canonical boundary is:

quantum::ir

A dialect MUST NOT introduce an unrelated second universal quantum IR merely to support dialect versioning.

A dialect-version change that requires a new "quantum::ir" contract MUST identify that dependency explicitly.

---

63. IR Evolution

An IR implementation may evolve independently from source dialect syntax.

For example:

Dialect X @ 1.4

may continue to be valid while:

quantum::ir

changes internally, provided the semantic contract and lowering compatibility remain preserved.

Conversely, a change to canonical IR may require a dialect-version update if the dialect's public semantic contract depends on the changed behavior.

The decision MUST be based on semantic impact, not implementation convenience.

---

64. Artifact Compatibility

A compiled artifact MAY contain:

language version
dialect versions
IR versions
ABI version
artifact format version
compiler/toolchain provenance

Artifact compatibility MUST NOT be inferred solely from source dialect compatibility.

For example:

source compatible

does not automatically mean:

binary artifact compatible

---

65. ABI Compatibility

Dialect versions are independent from ABI versions.

A dialect may remain semantically stable while an ABI changes.

Conversely, an ABI may remain stable while dialect semantics change.

The compiler MUST keep these dimensions separate.

---

66. Runtime Compatibility

Runtime compatibility is separate from dialect compatibility.

A dialect version may be valid while the available runtime cannot execute a required operation.

The correct result is a runtime/capability failure, not an incorrect dialect-version reinterpretation.

---

67. Target Compatibility

Target compatibility is downstream.

A dialect version does not guarantee that every target can execute every program written using that dialect.

The distinction is:

dialect-valid
        !=
target-feasible

For example:

dialect quantum::standard @ 2.0

may define a valid quantum program requiring a capability unavailable on a particular target.

The compiler MUST report the target limitation rather than alter the dialect semantics.

---

68. Resource Failure

Resource insufficiency MUST remain distinct from dialect incompatibility.

For example:

required memory > available memory

is not:

DIALECT_VERSION_CONFLICT

Likewise:

required capability unavailable

is not:

DIALECT_VERSION_CONFLICT

The appropriate resource/capability diagnostics MUST be used.

---

69. Quantum Dialect Versioning

Quantum dialects MUST follow the same versioning model.

A quantum dialect version may govern:

- operation semantics;
- state semantics;
- measurement semantics;
- dynamic-circuit semantics;
- parameter semantics;
- classical/quantum interaction;
- error/noise intent;
- QEC intent;
- resource requirements;
- capability requirements.

It MUST NOT encode a universal maximum number of qubits.

It MUST NOT hard-code a finite set of all future quantum operations.

The operation model remains data-driven.

---

70. Quantum IR Boundary

Quantum dialect versioning MUST converge on:

quantum::ir

The path is:

quantum dialect syntax
        |
        v
domain-neutral AST
        |
        v
quantum semantic model
        |
        v
quantum::ir

Dialect-version compatibility MUST be validated before irreversible target-specific quantum lowering.

---

71. HDL Dialect Versioning

HDL dialect versions may define:

- hardware intent;
- signals;
- interfaces;
- timing;
- state;
- pipelines;
- memories;
- verification;
- synthesis intent;
- parameterization.

They MUST NOT encode universal maximum widths, devices, modules, clocks, memories, or hardware resources.

Physical limits remain target/resource constraints.

---

72. Classical Dialect Versioning

Classical dialect versions may govern:

- numerical semantics;
- memory abstractions;
- parallel operations;
- vector/tensor semantics;
- data processing;
- optimization constructs.

They MUST remain independent of a particular CPU instruction set.

---

73. AI/Data Dialect Versioning

AI/data dialects may version:

- model abstractions;
- tensor semantics;
- inference constructs;
- learning constructs;
- reasoning constructs;
- uncertainty;
- provenance;
- data-query semantics.

A dialect version MUST NOT become a catalogue of application-specific keywords.

Domain applications belong in libraries, policies, capabilities, interoperability layers, or higher-level dialects.

---

74. Hybrid Dialect Versioning

Hybrid dialects may compose:

classical
quantum
AI
data
HDL
distributed
accelerator

Version compatibility MUST be evaluated across the complete semantic composition.

The hybrid dialect MUST NOT create a second semantic universe.

---

75. Distributed Dialect Versioning

Distributed dialects may version:

- message semantics;
- actor semantics;
- topology intent;
- consistency requirements;
- service interfaces;
- fault-tolerance contracts.

They MUST NOT encode:

MAX_NODES
MAX_MESSAGES
MAX_ACTORS
MAX_NETWORK_SIZE

as universal language limits.

---

76. Interoperability Dialect Versioning

External formats such as:

SQL
JSON
XML
OpenQASM
vendor formats

may have independent versions.

The Zamani dialect version MUST identify the semantic adapter contract, not merely repeat the external format's version.

For example:

external format version
        +
Zamani adapter dialect version

are distinct.

---

77. Vendor Dialect Versions

Vendor-specific dialects MAY exist.

A vendor dialect MUST:

- have a unique identity;
- declare ownership;
- declare its version;
- declare its language requirements;
- declare compatibility;
- declare capabilities;
- declare resource requirements where relevant;
- map into Zamani semantics;
- avoid silently changing core language meaning.

Vendor dialects MUST NOT force vendor-specific hardware assumptions into universal Zamani semantics.

---

78. Namespaces and Version Identity

The dialect namespace is part of dialect identity.

Changing:

vendor::x

to:

other_vendor::x

is not a patch-level version change.

It is an identity change.

The migration relationship must therefore be explicit.

---

79. Renaming a Dialect

A dialect rename MUST NOT silently reuse the old identity.

A proper migration is:

old.identity @ old.version
        |
        v
migration mapping
        |
        v
new.identity @ new.version

This preserves provenance and prevents ambiguity.

---

80. Dialect Ownership

A stable dialect SHOULD identify an owner or governing authority.

Ownership metadata MUST NOT alter semantic compatibility.

Ownership MAY be represented by:

- namespace;
- registry identity;
- package metadata;
- provenance metadata.

The identity remains the dialect's stable namespace/name.

---

81. Registry Integration

The dialect registry MAY contain:

DialectId
DialectVersion
Owner
Status
LanguageCompatibility
Dependencies
Capabilities
Compatibility
IRContracts
Migration
Provenance

The registry is an implementation/tooling mechanism.

It MUST NOT replace the normative dialect specification.

---

82. Registry Absence

A dialect MAY be locally available without being present in a global registry, provided the implementation has sufficient trusted metadata to establish its contract.

The language MUST NOT require a single global network registry for semantic validity.

This is important for:

- offline compilation;
- embedded systems;
- air-gapped systems;
- reproducible builds;
- long-term archival;
- future environments.

---

83. Registry Conflicts

If two metadata sources provide incompatible definitions for the same dialect identity/version, the compiler MUST fail closed.

Possible diagnostics:

DIALECT_REGISTRY_CONFLICT
DIALECT_METADATA_CONFLICT
DIALECT_PROVENANCE_CONFLICT

The compiler MUST NOT silently select whichever source happens to be encountered first.

---

84. Trust and Security

Dialect metadata MUST NOT automatically grant execution authority.

A dialect version identifies semantics.

It does not automatically grant:

- native execution;
- filesystem access;
- network access;
- reflection;
- code generation;
- adaptation;
- hardware access;
- privileged operations.

Those are governed by:

effects
capabilities
policies
security
sandbox
authorization

---

85. Dialect Version and Policies

A policy MAY constrain which dialect versions are permitted.

For example:

forbid dialect::legacy @ < 2.0.0

or conceptually:

allow dialect::trusted @ >= 3.0.0

Policy resolution MUST remain separate from version identity.

A policy MUST NOT rewrite a dialect version silently.

---

86. Dialect Version and Sandboxing

Sandboxing MAY restrict a dialect version's effects or capabilities.

For example:

dialect A @ 2.0

may be syntactically valid while:

native execution

is forbidden by the active sandbox.

This is a security decision, not a dialect-version incompatibility.

---

87. Dialect Version and Adaptation

A dialect version MAY define adaptation semantics.

Adaptation MUST remain controlled by:

- policy;
- authorization;
- capabilities;
- effects;
- resources;
- provenance.

A dialect version MUST NOT create unrestricted self-modifying semantics merely because the dialect supports adaptive execution.

---

88. Deterministic Resolution

For identical:

source
language version
dialect declarations
dialect metadata
version constraints
compatibility policy
feature state
dependency metadata

the dialect resolution result MUST be deterministic.

Equivalent input sets MUST NOT produce different results merely because of:

- filesystem traversal order;
- hash-map iteration;
- thread scheduling;
- host platform;
- machine identity.

---

89. Deterministic Diagnostics

Version resolution failures MUST be deterministic.

Equivalent invalid inputs SHOULD produce equivalent diagnostic classes and stable source locations.

Diagnostics MUST identify, where applicable:

dialect
requested version
available version(s)
constraint
conflicting requirement
compatibility class
migration possibility
source span

---

90. Version Comparison

Version comparison MUST be semantic.

It MUST NOT compare versions as raw strings.

For example:

1.10.0

must not be ordered before:

1.9.0

merely because lexical comparison says otherwise.

The canonical comparison algorithm belongs to the repository's versioning implementation/specification.

This file requires its use.

---

91. Numeric Representation

Version components MUST be represented using a safe representation sufficient for the implementation's supported version domain.

Rust implementation code MUST NOT use "unsafe" tricks to obtain artificial capacity.

If a representation has an implementation-defined maximum, that limit MUST be documented as an implementation constraint rather than presented as a universal dialect-version rule.

---

92. Integer Overflow

Version parsing and comparison MUST NOT silently wrap numeric components.

For example:

999999999999999999999999999

MUST either:

- be represented safely;
- be rejected with a precise representation diagnostic;
- or be handled according to the supported version representation contract.

Silent wrapping is prohibited.

---

93. Rust Safety

The Rust implementation of dialect-version handling MUST use safe Rust.

It MUST NOT require:

unsafe

for:

- parsing;
- comparison;
- range resolution;
- dependency resolution;
- compatibility checking;
- migration planning;
- provenance;
- diagnostics.

Safe standard-library and crate abstractions MUST be preferred.

---

94. Rust Version Baseline

The implementation target is:

Rust 1.97 or later
Rust edition 2021

The repository's "Cargo.toml" remains authoritative for the actual compiler/toolchain declaration.

This document does not redefine Cargo configuration.

A Rust toolchain update MUST NOT automatically imply a dialect-version update.

---

95. Source Version Declaration

Where source syntax declares a dialect version, the syntax is owned by:

grammar/dialects/versioning.g4

This document does not duplicate that grammar.

Conceptually, a declaration may identify:

dialect identity
+
version expression

The parser accepts structure.

Semantic analysis decides whether the version is valid and compatible.

---

96. Parser Versus Semantic Validation

The parser MUST NOT be expected to decide:

is this dialect version supported?

The correct separation is:

lexer
    |
    v
parser
    |
    v
structural AST
    |
    v
version validation
    |
    v
compatibility resolution
    |
    v
semantic analysis

A syntactically valid version expression may still be semantically invalid.

---

97. Version Syntax Must Remain Open

The dialect version grammar MUST NOT enumerate all future dialect names.

It MUST NOT enumerate all future vendors.

It MUST NOT enumerate all future dialect capabilities.

It MUST NOT enumerate all future computing domains.

Versioning is metadata over an extensible dialect architecture.

---

98. Feature Evolution

A new dialect feature MUST follow:

proposal
    |
    v
semantic design
    |
    v
version impact analysis
    |
    v
grammar
    |
    v
AST
    |
    v
semantic implementation
    |
    v
IR mapping
    |
    v
compiler/backend
    |
    v
tests
    |
    v
compatibility classification
    |
    v
stable release

The version increment MUST be selected from the actual compatibility impact.

---

99. Stable Feature Promotion

An experimental dialect feature MUST NOT automatically become stable merely because it has parser support.

Before stable promotion it requires:

- specified semantics;
- version classification;
- AST mapping;
- semantic mapping;
- IR mapping where applicable;
- diagnostics;
- positive tests;
- negative tests;
- boundary tests;
- determinism tests;
- compatibility tests;
- scalability tests;
- migration/deprecation treatment where applicable.

---

100. Version Change Audit

Every dialect-version change MUST answer:

1. Did syntax change?
2. Did tokenization change?
3. Did AST shape change?
4. Did type meaning change?
5. Did effect meaning change?
6. Did capability meaning change?
7. Did resource meaning change?
8. Did evaluation semantics change?
9. Did concurrency semantics change?
10. Did quantum semantics change?
11. Did HDL/hardware semantics change?
12. Did canonical IR mapping change?
13. Did ABI behavior change?
14. Did runtime assumptions change?
15. Did diagnostics change incompatibly?
16. Does migration exist?
17. Does deprecation apply?
18. Does provenance need updating?

The highest applicable compatibility impact determines the version classification.

---

101. Version Change Classification

A version change MUST be classified as one of:

PATCH
MINOR
MAJOR
PRE_RELEASE
DEPRECATION
REMOVAL
MIGRATION_ONLY
IMPLEMENTATION_ONLY
DOCUMENTATION_ONLY

The classification MUST be recorded in release/compatibility metadata.

---

102. Implementation-Only Changes

A dialect implementation may change without changing the dialect version when the public contract remains identical.

Examples:

- parser optimization;
- faster resolver;
- memory optimization;
- improved diagnostics;
- backend optimization;
- cache implementation;
- parallel compilation;
- data-structure replacement.

The implementation change MUST NOT alter observable dialect semantics.

---

103. Backend Changes

Adding a new backend MUST NOT require a dialect version change unless the dialect's public semantics change.

For example:

new CPU backend
new GPU backend
new FPGA backend
new ASIC backend
new QPU backend
new simulator
new distributed backend

may all be added independently.

This is essential to POCO-REAF.

---

104. Hardware Evolution

A dialect version MUST remain independent from physical hardware evolution.

The same dialect contract may be realized on:

tiny embedded target
single CPU
multicore CPU
GPU
FPGA
ASIC
accelerator
QPU
simulator
HPC system
cluster
distributed environment
future architecture

subject to capabilities and resources.

Hardware growth MUST NOT require artificial dialect-version proliferation.

---

105. Resource Scaling

There MUST be no dialect-version semantics such as:

dialect version 1 supports 32 qubits
dialect version 2 supports 64 qubits

This would incorrectly bind semantic versioning to physical capacity.

Correct architecture:

dialect version
        |
        v
portable semantic contract
        |
        v
resource requirements
        |
        v
capability negotiation
        |
        v
target realization

---

106. Tiny-to-Arbitrarily-Large Scaling

The dialect-version system MUST be independent of program scale.

It MUST NOT define fixed maximums for:

- dialect declarations;
- dependencies;
- composition depth;
- modules;
- operations;
- types;
- source size;
- program size;
- quantum registers;
- hardware resources;
- distributed nodes.

Implementation limits MAY exist due to available memory, storage, processing time, or other resources.

Those are not language-level semantic ceilings.

---

107. "Infinity" Interpretation

"Infinity" in the scalability objective means:

«No artificial finite ceiling is imposed by the dialect-version language contract.»

It does not mean an implementation can physically process an infinite artifact.

Actual execution remains constrained by:

available resources
representation
time
storage
target capabilities
physical feasibility
implementation capacity

The architecture MUST allow the implementation and target ecosystem to scale without requiring a redesign of dialect-version semantics.

---

108. Dialect Version and Contracts

Dialect versions MUST participate in the broader contract system.

A dialect may declare:

requires
ensures
invariant
assume
guarantee
property

The version contract identifies which semantics govern those constructs.

Changing contract interpretation can therefore require a dialect MAJOR increment.

---

109. Dialect Version and Provenance

A provenance record SHOULD retain:

dialect identity
dialect version
language version
source location
migration
dependency resolution
compatibility decision
semantic transformation
IR mapping
compiler/toolchain identity

This allows long-lived artifacts to remain explainable.

---

110. Long-Term Reproducibility

POCO-REAF requires that a future toolchain can determine what dialect contract was intended.

Therefore an artifact SHOULD NOT rely exclusively on:

"latest dialect"

or:

ambient installed dialect

for reproducible builds.

The effective dialect versions MUST be recoverable from explicit source/build/provenance information.

---

111. Archived Dialect Versions

A historical dialect version MAY remain available for:

- reproducibility;
- source migration;
- archival compilation;
- conformance testing;
- scientific reproducibility;
- legacy interoperability.

Archival availability does not mean the version is currently recommended.

---

112. Frozen Dialect Versions

A dialect version may be marked frozen.

A frozen version:

- remains semantically immutable;
- may receive implementation fixes;
- may receive tooling improvements;
- MUST NOT silently acquire new semantics.

A new semantic contract requires a new dialect version.

---

113. Dialect Version Immutability

Once a dialect version is released as stable:

Dialect X @ 1.2.3

MUST refer to the same public semantic contract permanently.

If the contract changes, release:

1.2.4

for a compatible correction, or:

1.3.0

for compatible additions, or:

2.0.0

for breaking changes.

A released version MUST NOT be silently republished with different semantics.

---

114. Compatibility Matrix Integration

"grammar/compatibility/compatibility-matrix.md" SHOULD be able to represent:

dialect identity
dialect version
language version
grammar version
AST version
semantic version
IR version
compiler version
runtime version
target contract
compatibility result

This file supplies the dialect-version semantics used by that matrix.

The matrix MUST NOT invent a second dialect-version model.

---

115. Frontend Conformance Integration

"grammar/compatibility/frontend-conformance.md" MUST verify that:

source dialect version
        |
        v
lexer
        |
        v
parser
        |
        v
AST

preserves the dialect-version identity correctly.

The frontend MUST NOT silently discard the selected dialect version when that version affects semantics.

---

116. AST Conformance Integration

"grammar/compatibility/ast-conformance.md" MUST verify:

- dialect identity preservation;
- dialect version preservation;
- source spans;
- version constraints;
- migration metadata where required;
- dependency identity where required.

The AST representation MUST be sufficient for semantic validation.

---

117. IR Conformance Integration

"grammar/compatibility/ir-conformance.md" MUST verify that the resolved dialect contract is reflected correctly in semantic/IR lowering where relevant.

The IR MUST NOT invent a dialect version different from the one actually resolved.

---

118. Feature-Gate Integration

"grammar/compatibility/feature-gates.md" MUST consume the dialect version when feature availability depends on it.

The relationship is:

dialect version
        |
        v
feature availability
        |
        v
semantic validation

A feature gate MUST NOT silently reinterpret a dialect version.

---

119. Migration Integration

"grammar/compatibility/migrations.md" owns migration procedures.

This file requires migrations to identify:

source dialect identity
source dialect version
target dialect identity
target dialect version
compatibility classification
semantic-preservation guarantee
required transformations
diagnostics
provenance

---

120. Deprecation Integration

"grammar/compatibility/deprecated.md" owns lifecycle policy.

This file requires dialect-version deprecations to retain unambiguous version identity.

The implementation MUST distinguish:

stable
deprecated
removed
unknown
unsupported
experimental

---

121. Dialect Grammar Integration

The concrete syntax remains in:

grammar/dialects/versioning.g4

That grammar already provides the structural dialect-version machinery.

This document therefore MUST NOT duplicate grammar productions.

The integration is:

grammar/dialects/versioning.g4
        |
        v
parse dialect-version declaration
        |
        v
AST version representation
        |
        v
THIS CONTRACT
        |
        v
semantic validation/resolution

---

122. Dialect Grammar Composition

"grammar/dialects/dialects.g4" remains a composition façade.

It MUST NOT implement version compatibility algorithms.

"grammar/dialects/registration.g4" owns registration syntax.

"grammar/dialects/compatibility.g4" owns compatibility declaration syntax.

"grammar/dialects/versioning.g4" owns version syntax.

This file owns the semantic interpretation of those declarations.

---

123. No Duplicate Version Grammars

The repository MUST NOT create another grammar such as:

grammar/compatibility/dialect-version.g4

merely to implement this document.

The current architecture already gives version syntax to:

grammar/dialects/versioning.g4

A compatibility document is not a parser grammar.

---

124. Lexer Integration

Dialect versioning MUST reuse the canonical lexer.

It MUST NOT create a dialect-specific lexer merely for version parsing.

Numeric, identifier, string, punctuation, and version-related tokens remain owned by the canonical lexical architecture.

---

125. Token Compatibility

If dialect version syntax introduces aliases or legacy spellings, the repository MUST identify:

canonical token
legacy token/spelling
compatibility status
migration
deprecation
tests

Token aliases MUST NOT create redundant token pairs without semantic need.

---

126. AST Integration Contract

The AST representation for dialect versioning MUST preserve at least:

dialect identity
version expression
source span
version constraint information

Where required:

pre-release
build metadata
dependency
feature state
migration origin

must also be preserved.

The exact Rust type belongs to the AST implementation.

---

127. Semantic Integration Contract

Semantic analysis MUST:

1. resolve dialect identity;
2. parse/validate dialect version;
3. resolve version constraints;
4. load/obtain trusted dialect metadata;
5. validate language compatibility;
6. validate dependencies;
7. validate feature state;
8. validate semantic compatibility;
9. validate IR requirements;
10. produce deterministic diagnostics;
11. record provenance where enabled.

---

128. Capability Integration

After dialect-version resolution:

dialect contract
        |
        v
required capabilities
        |
        v
capability discovery
        |
        v
capability negotiation

The dialect version itself does not guarantee physical capability.

---

129. Resource Integration

Likewise:

dialect contract
        |
        v
resource requirements
        |
        v
resource analysis
        |
        v
target feasibility

Resource failure MUST NOT become dialect incompatibility.

---

130. Effect Integration

If a dialect version changes effect semantics, that is a compatibility event.

Examples:

dialect 1
    network operation has effect(network)

dialect 2
    same operation additionally mutates global state

If this changes observable semantics, it MUST receive appropriate compatibility classification.

---

131. Policy Integration

Policies may accept or reject dialect versions.

For example:

allow stable dialects
forbid deprecated dialects
require dialect >= version

Policy decisions MUST be recorded separately from version identity.

---

132. Provenance Integration

The compiler SHOULD be capable of producing a provenance chain:

source
  |
  +--> language version
  |
  +--> dialect version
  |
  +--> dependencies
  |
  +--> compatibility decisions
  |
  +--> migration
  |
  +--> semantic analysis
  |
  +--> IR
  |
  +--> lowering
  |
  +--> target

This supports auditing and reproducibility.

---

133. Diagnostics

The dialect-version subsystem SHOULD define stable diagnostic categories for at least:

UNKNOWN_DIALECT
UNKNOWN_DIALECT_VERSION
INVALID_DIALECT_VERSION
DIALECT_VERSION_CONFLICT
DIALECT_VERSION_UNSUPPORTED
DIALECT_VERSION_DEPRECATED
DIALECT_VERSION_REMOVED
DIALECT_VERSION_MIGRATION_REQUIRED
DIALECT_LANGUAGE_VERSION_CONFLICT
DIALECT_DEPENDENCY_CONFLICT
DIALECT_DEPENDENCY_CYCLE
DIALECT_METADATA_CONFLICT
DIALECT_REGISTRY_CONFLICT
DIALECT_SEMANTIC_CONFLICT
DIALECT_IR_CONFLICT

Exact diagnostic codes/messages belong to the diagnostics subsystem.

---

134. Error Reporting Requirements

A version diagnostic SHOULD include:

dialect identity
requested version
resolved/available version
source span
constraint
conflicting dependency
compatibility classification
migration suggestion

where available.

Diagnostics MUST NOT depend on machine-specific hardware enumeration order.

---

135. Negative Cases

The implementation MUST reject at least:

1. malformed dialect versions;
2. unknown dialect versions without compatibility coverage;
3. contradictory version constraints;
4. incompatible dialect dependencies;
5. ambiguous dialect identity;
6. conflicting dialect ownership metadata;
7. unsupported stable versions;
8. removed versions without migration;
9. semantic conflicts between composed versions;
10. incompatible language-version requirements.

---

136. Positive Cases

Tests MUST cover:

dialect::x @ 1.0.0
dialect::x @ 1.10.0
dialect::x @ 1.0.0-alpha
dialect::x @ 1.0.0-beta
dialect::x @ 1.0.0-rc.1
dialect::x @ 1.0.0+build

and:

- exact requirements;
- ranges;
- multiple constraints;
- compatible dependencies;
- multiple dialect composition;
- migrations;
- deprecated versions;
- experimental versions;
- unknown versions;
- large version components;
- long dependency graphs.

---

137. Boundary Tests

Boundary tests MUST include:

0.0.0
0.1.0
1.0.0
1.0.1
1.1.0
1.10.0
2.0.0

and large representable components.

The purpose is to test semantic comparison, not to establish a maximum.

---

138. Scalability Tests

The test suite MUST verify that the versioning implementation does not contain artificial limits on:

- number of dialects;
- number of dependencies;
- dependency depth;
- version constraints;
- composition relationships;
- metadata entries;
- program size.

Tests may use progressively larger generated inputs subject to available test-machine resources.

---

139. Determinism Tests

Given identical inputs:

source
language version
dialect metadata
version constraints
dependency graph
feature state
policy

the resolver MUST produce the same:

resolved dialect versions
compatibility classifications
diagnostic categories
provenance decisions

Equivalent ordering of semantically unordered metadata MUST NOT alter the result.

---

140. Reproducibility Tests

A reproducibility test MUST demonstrate that the effective dialect version set can be reconstructed from the recorded build/source metadata.

The test SHOULD verify:

compile
   ->
record dialect versions
   ->
reconstruct
   ->
resolve
   ->
same dialect contract

---

141. Cross-Domain Tests

The dialect-version tests MUST include at least:

classical dialect
quantum dialect
HDL dialect
hardware dialect
AI/data dialect
distributed dialect
networking dialect
interoperability dialect
hybrid dialect

The purpose is to prove that versioning remains domain-neutral.

---

142. Quantum Cross-Domain Test

At minimum, verify:

quantum dialect version
        |
        v
quantum semantic model
        |
        v
quantum::ir
        |
        v
resource/capability analysis
        |
        v
routing
        |
        v
QEC/ZQN/HAL

A new physical quantum target MUST NOT require a dialect-version change unless the dialect's semantics actually change.

---

143. Hybrid Cross-Domain Test

Verify a composition such as:

classical::numeric
quantum::standard
ai::tensor
distributed::execution

with independent versions.

The resolver MUST:

- preserve each identity;
- resolve each version;
- validate dependencies;
- validate semantic composition;
- avoid creating a combined artificial version unless a separate hybrid dialect explicitly defines one.

---

144. Hardware Scaling Test

The same dialect contract MUST be testable against target descriptions representing:

small target
large target
heterogeneous target
accelerator target
distributed target
future target

The dialect version MUST remain unchanged when only target capabilities differ.

---

145. No Hardware Version Leakage

A compiler MUST NOT perform:

target hardware version
        ->
automatic dialect version change

unless an explicit target/dialect compatibility contract requires a different dialect contract.

Hardware discovery must never silently rewrite source semantics.

---

146. Compiler Integration

The compiler MUST carry the resolved dialect-version information through the frontend.

Conceptually:

source
  |
  v
lexer
  |
  v
parser
  |
  v
AST
  |
  v
DialectVersionResolver
  |
  v
SemanticContext

The resolved version MUST be available to version-sensitive semantic passes.

---

147. Runtime Integration

Runtime components MAY consume dialect-version metadata when required to execute a contract.

However, runtime MUST NOT redefine the dialect semantics.

If runtime support is missing:

RUNTIME_DIALECT_SUPPORT_UNAVAILABLE

or an equivalent diagnostic/result belongs to the runtime layer.

---

148. Tooling Integration

Tooling such as:

- formatter;
- linter;
- language server;
- documentation generator;
- migration tool;
- package manager;
- compiler driver;

MUST understand the dialect-version model consistently.

Tools MUST NOT invent independent dialect-version semantics.

---

149. Package Integration

Packages that provide dialects SHOULD declare:

dialect identity
dialect version
supported language versions
dependencies
feature states
compatibility metadata

Package version and dialect version remain separate.

A package MAY contain several dialect versions.

---

150. Dialect Version Versus Package Version

For example:

package: quantum-tools 5.2.0
dialect: quantum::standard 2.1.0

The package may update without changing the dialect.

Likewise, a dialect can change while the packaging structure remains stable.

---

151. Version Aliases

Aliases such as:

latest
stable
current

MAY be supported by tooling.

They MUST NOT replace explicit version identity for reproducible language semantics.

For reproducible compilation:

dialect X @ 1.4.2

is preferable to:

dialect X @ latest

---

152. Floating Versions

A floating compatibility range MAY be used where explicitly permitted.

For example:

>= 1.2.0, < 2.0.0

The resolver MUST record the actual selected version.

The source semantics MUST therefore remain reproducible through the resolved version record.

---

153. Lock-Free Source Portability

A source program MUST remain portable even when it does not embed a package-lock artifact.

The language contract can remain stable while tooling supplies a reproducible dependency environment externally.

This preserves the separation between:

language source

and:

build environment

---

154. Long-Lived Source

A source program should remain interpretable years later provided its declared dialect contract remains available or a documented migration path exists.

The architecture MUST therefore preserve:

dialect identity
dialect version
language version
semantic assumptions

rather than relying solely on "current" behavior.

---

155. Archival Compatibility

Archival tooling SHOULD be capable of recording enough metadata to reconstruct:

source
language version
dialect versions
dependencies
compatibility policies
migration history
IR contracts

This is especially important for:

- scientific computation;
- quantum experiments;
- hardware designs;
- simulations;
- safety-critical systems;
- reproducible research;
- long-lived embedded systems.

---

156. Version Negotiation

Negotiation may occur between:

source requirement
dialect provider
compiler
runtime
target

However, negotiation must occur in layers.

The compiler MUST NOT solve:

dialect version
+
hardware capability
+
runtime version

as one ambiguous version number.

The correct architecture is:

dialect version resolution
        |
        v
semantic compatibility
        |
        v
capability negotiation
        |
        v
resource feasibility
        |
        v
target realization

---

157. Graceful Target Failure

If the dialect contract is valid but the target cannot realize it:

dialect valid
semantic valid
target infeasible

the compiler MAY:

- choose another target;
- choose another compatible implementation strategy;
- simulate;
- defer;
- retry;
- recover;
- report infeasibility;

according to the execution/resource/policy architecture.

It MUST NOT silently downgrade the dialect semantics.

---

158. Semantic Preservation During Optimization

Optimizations may transform representations.

They MUST preserve the dialect semantics selected during version resolution.

Therefore:

dialect version
        |
        v
semantic meaning
        |
        v
optimization

and not:

optimization
        |
        v
new dialect meaning

---

159. Lowering

Lowering may specialize the dialect to:

- CPU;
- GPU;
- FPGA;
- ASIC;
- accelerator;
- QPU;
- simulator;
- distributed system.

Lowering MUST preserve the selected dialect contract.

A target-specific lowering failure is not automatically a dialect-version incompatibility.

---

160. Resilience and Recovery

Runtime resilience MAY retry or recover target execution.

It MUST NOT change the dialect version contract during recovery.

For example:

Healthy
Degraded
Unstable
Unavailable
Recovering
Quarantined
Retired

are runtime/target states.

They are not dialect versions.

---

161. Versioning and Simulation

Simulation may execute a dialect contract without physical hardware.

The simulator MUST use the same resolved dialect semantics as physical execution unless a simulation dialect explicitly defines different semantics.

Simulation is therefore an execution strategy, not a dialect-version substitute.

---

162. Versioning and Deterministic Execution

If a dialect promises deterministic semantics, the dialect version MUST preserve that contract.

If nondeterminism is intentional, it must be represented through the appropriate effect/semantic system.

A version change that changes deterministic guarantees may require a compatibility impact classification.

---

163. Versioning and Provenance of Decisions

If dialect semantics influence:

- optimization;
- resource selection;
- hardware placement;
- quantum routing;
- scheduling;
- policy decisions;

the provenance system SHOULD retain the dialect version responsible for the semantic decision.

---

164. Versioning and Explainability

When an explanation system explains why a compiler/runtime decision was made, it SHOULD be able to identify the dialect contract that contributed to the decision.

Example:

Decision:
    selected implementation X

Reason:
    dialect quantum::standard @ 2.1.0 requires semantic operation Y

This is diagnostic/provenance information, not a new language construct.

---

165. Versioning and Security

Security-sensitive dialects MUST NOT use version numbers as authorization credentials.

A dialect version says:

which semantics

not:

who is trusted

Trust belongs to the security/provenance/policy architecture.

---

166. Version Authenticity

Where trusted external dialect metadata is required, authenticity SHOULD be established by the package/registry/security layer.

This document does not define cryptographic algorithms.

The semantic version remains the version identifier regardless of how its metadata was obtained.

---

167. No Implicit Network Dependency

Dialect-version resolution MUST NOT inherently require live network access.

A compiler MUST be able to work from local trusted metadata when the required dialect contract is locally available.

This supports:

- offline systems;
- embedded systems;
- secure environments;
- reproducible builds;
- archival compilation.

---

168. No Time-Dependent Semantics

The meaning of a dialect version MUST NOT change merely because the current date/time changed.

For example:

dialect X @ 1.0.0

must not silently mean something different next year.

Deprecation/removal is metadata/lifecycle state and must be explicitly resolved.

---

169. Compatibility With Future Computing Models

A dialect-version contract MUST remain extensible to future domains.

Future dialects may describe:

- new classical architectures;
- new quantum architectures;
- new photonic systems;
- new neuromorphic systems;
- new accelerators;
- new distributed substrates;
- new programmable materials;
- new computational models not yet known.

No change to the universal versioning architecture should be required merely because a new dialect identity appears.

---

170. Reserved Versioning Space

The repository MUST NOT reserve arbitrary finite numeric ranges as a substitute for extensibility.

For example, this is prohibited as universal policy:

dialect versions 0..255

or:

major <= 65535

The representation and parser must be extensible.

---

171. Version Metadata Extensibility

Additional version metadata MAY be introduced through the established metadata architecture.

New metadata MUST NOT silently redefine:

major
minor
patch

or their compatibility meaning.

New fields require specification and compatibility analysis.

---

172. Version Contract Header

Every stable dialect specification SHOULD identify:

Dialect Identity
Dialect Version
Status
Owner
Language Compatibility
Grammar Contract
AST Contract
Semantic Contract
Effect Contract
Capability Contract
Resource Contract
IR Contract
ABI Contract
Migration Policy
Deprecation State
Provenance Contract

This is documentation/contract metadata, not necessarily source syntax.

---

173. Independent-File Completion Contract

This file is complete independently when the following are fixed:

Purpose

Dialect version semantic contract.

Owns

Dialect version identity, ordering, compatibility, evolution, resolution, migration semantics.

Does not own

Dialect concrete grammar or general dialect compatibility algorithms.

Inputs

- dialect identity;
- version expression;
- language version;
- dependency metadata;
- compatibility metadata;
- feature state;
- policy.

Outputs

- resolved dialect version;
- compatibility classification;
- migration requirement;
- deterministic diagnostics;
- provenance metadata.

AST dependency

Consumes dialect-version AST representation from the dialect grammar.

Semantic dependency

Feeds dialect semantic compatibility analysis.

IR dependency

Provides resolved dialect contract metadata to relevant IR/conformance stages.

Runtime dependency

Provides contract identity only; runtime remains responsible for execution.

Test dependency

Dialect-version conformance tests.

Scalability

No artificial limits.

Safety

Safe Rust implementation.

Completion

All semantics in this document are represented by specification, implementation contract, and tests.

---

174. Exact Integration Contract

The implementation integration must be:

grammar/dialects/versioning.g4
        |
        | syntax
        v
frontend AST
        |
        | representation
        v
dialect version resolver
        |
        | this document
        v
compatibility classification
        |
        +--> language-version compatibility
        +--> grammar-version compatibility
        +--> semantic-version compatibility
        +--> feature gates
        +--> dialect dependencies
        +--> migration
        +--> deprecation
        |
        v
dialect-compatibility.md
        |
        v
semantic analysis
        |
        +--> types
        +--> effects
        +--> capabilities
        +--> resources
        +--> contracts
        +--> policies
        +--> provenance
        |
        v
canonical semantic model
        |
        +--> Classical IR
        +--> quantum::ir
        +--> HDL/Hardware IR
        |
        v
lowering / optimization / routing / scheduling
        |
        v
ZQN / HAL
        |
        v
target

No step may skip semantic validation.

---

175. Required Rust Implementation Components

The implementation SHOULD have clearly separated responsibilities corresponding to:

DialectId
DialectVersion
DialectVersionRequirement
DialectVersionRange
DialectVersionConstraint
DialectCompatibility
DialectVersionResolution
DialectVersionDiagnostic
DialectVersionProvenance

These names are conceptual contracts.

Existing repository naming should be reused where equivalent types already exist rather than creating duplicates.

---

176. Single Ownership of "DialectVersion"

There MUST be one canonical semantic representation of a dialect version in the relevant implementation layer.

Specialized systems MAY wrap it, but MUST NOT create incompatible independent definitions.

For example:

frontend dialect version
quantum dialect version
HDL dialect version
AI dialect version

may have domain-specific metadata, but their common version identity must remain interoperable.

---

177. Quantum Specialized Versions

Existing quantum-specific Rust structures such as:

StandardDialectVersion
PulseDialectVersion

may remain specialized implementation types where they genuinely represent distinct public contracts.

However, they MUST map consistently to the repository-wide dialect-version model.

A quantum-specific version MUST NOT silently redefine general version ordering or compatibility.

---

178. Version Conversion

Conversions between:

generic DialectVersion

and specialized dialect-version types MUST be:

- explicit;
- checked;
- lossless where required;
- deterministic.

A failed conversion MUST produce a diagnostic/result rather than silently truncating information.

---

179. No Numeric Truncation

The implementation MUST NOT convert arbitrary version components through fixed-width narrowing conversions merely for convenience.

For example, it MUST NOT silently convert:

large version component

to:

u8

and wrap/truncate it.

Any representation boundary must validate the value.

---

180. Error Handling

Dialect-version APIs SHOULD use explicit error/result semantics.

Errors SHOULD distinguish:

parse error
invalid version
unsupported version
unknown version
conflict
dependency failure
migration required
policy rejection
metadata failure

The implementation MUST NOT silently recover by selecting an arbitrary version.

---

181. Concurrency

Dialect-version resolution MAY be performed concurrently.

Concurrent implementation MUST remain deterministic.

Parallel processing MUST NOT cause version selection to depend on completion order.

---

182. Caching

Version metadata and compatibility results MAY be cached.

Caches MUST be keyed by sufficient semantic identity.

At minimum, cache keys SHOULD account for:

dialect identity
dialect version
language contract
relevant dependency versions
compatibility policy
feature state

A stale cache MUST NOT silently change semantics.

---

183. Incremental Compilation

Incremental compilation MAY cache resolved dialect versions.

The cache must be invalidated when relevant compatibility inputs change.

A compiler MUST NOT reuse a dialect-resolution result after the dialect contract has changed.

---

184. Distributed Compilation

Distributed compilation workers MUST receive the resolved dialect contract or enough immutable metadata to reproduce it.

Workers MUST NOT independently select arbitrary dialect versions.

This is important for deterministic distributed compilation.

---

185. Remote Compilation

Remote compilation MUST preserve:

language version
dialect versions
dependency versions
compatibility policy

A remote compiler MUST NOT silently substitute a different dialect contract.

---

186. Cross-Compiler Compatibility

Two Zamani compilers may implement the same dialect version.

They are compatible when they preserve the same specified semantics.

Different compiler implementations do not require different dialect versions.

---

187. Cross-Platform Compatibility

A dialect version is platform-independent unless the dialect specification explicitly defines platform-dependent semantics.

Platform-specific realization belongs to:

capabilities
resources
target contracts
backend
runtime

---

188. Source Portability

A dialect version should describe portable semantics.

A program using a dialect may still have target requirements.

For example:

dialect valid
requires capability("quantum.measurement")

The program remains portable even if a particular target lacks that capability.

---

189. Target Infeasibility

The compiler MUST distinguish:

dialect unsupported

from:

dialect supported
target infeasible

This distinction is essential for POCO-REAF.

---

190. Compatibility Matrix Example

A conceptual result may be:

Layer| Result
Language| compatible
Dialect| compatible
Grammar| compatible
AST| compatible
Semantics| compatible
Effects| compatible
Capabilities| available
Resources| insufficient
IR| compatible
Runtime| compatible
Target| infeasible

The final result is not a dialect-version failure.

It is a resource/target feasibility failure.

---

191. Production Readiness Checklist

"grammar/compatibility/dialect-version.md" is production-ready only when:

- dialect identity is defined;
- dialect version identity is defined;
- dialect version is separate from language version;
- dialect version is separate from grammar version;
- dialect version is separate from AST version;
- dialect version is separate from semantic version;
- dialect version is separate from IR version;
- dialect version is separate from compiler version;
- dialect version is separate from runtime version;
- dialect version is separate from ABI version;
- dialect version is separate from target version;
- semantic version ordering is defined;
- compatibility classes are defined;
- unknown versions fail closed;
- dependency conflicts are deterministic;
- dependency cycles are detected;
- version resolution is deterministic;
- migration is defined;
- deprecation is defined;
- feature-gate interaction is defined;
- provenance is defined;
- reproducibility is defined;
- quantum integration is defined;
- "quantum::ir" remains canonical;
- HDL integration is defined;
- classical integration is defined;
- hybrid integration is defined;
- distributed integration is defined;
- interoperability integration is defined;
- resource compatibility is separated from dialect compatibility;
- capability compatibility is separated from dialect compatibility;
- target compatibility is separated from dialect compatibility;
- no hardware limits are encoded;
- no fixed dialect-count limit is encoded;
- no fixed dependency-depth limit is encoded;
- no fixed version-component ceiling is encoded;
- no fixed program-size limit is encoded;
- Rust 1.97+ is supported;
- Rust "unsafe" is prohibited;
- parser syntax remains owned by "grammar/dialects/versioning.g4";
- no duplicate version grammar is created;
- AST integration is defined;
- semantic integration is defined;
- IR integration is defined;
- compiler integration is defined;
- runtime integration is defined;
- tests are defined;
- scalability tests are defined;
- deterministic tests are defined;
- cross-domain tests are defined.

---

192. Required Test Matrix

The corresponding tests SHOULD be organized approximately as:

grammar/tests/
└── compatibility/
    └── dialect-version/
        ├── syntax/
        ├── parsing/
        ├── ast/
        ├── comparison/
        ├── constraints/
        ├── ranges/
        ├── resolution/
        ├── dependencies/
        ├── conflicts/
        ├── cycles/
        ├── migration/
        ├── deprecation/
        ├── feature-gates/
        ├── provenance/
        ├── reproducibility/
        ├── determinism/
        ├── scalability/
        ├── quantum/
        ├── classical/
        ├── hdl/
        ├── hybrid/
        ├── distributed/
        ├── interoperability/
        ├── negative/
        └── boundary/

The exact test-tree location may follow the repository's existing test organization, but ownership must remain unambiguous.

---

193. Minimum Acceptance Tests

The production implementation MUST demonstrate:

Exact

1.2.3 == 1.2.3

Numeric ordering

1.10.0 > 1.9.0

Patch compatibility

1.2.3 -> 1.2.4

Minor compatibility

1.2.x -> 1.3.x

when the public contract remains backward-compatible.

Major incompatibility

1.x -> 2.x

when semantics intentionally break.

Unknown

unknown version

fails closed.

Dependency conflict

A -> B >= 1,<2
C -> B >= 3,<4

fails deterministically.

Dependency cycle

A -> B -> C -> A

is detected.

Target independence

A target change does not silently change the dialect version.

Quantum independence

A QPU generation change does not silently change the dialect version.

Resource independence

A memory shortage does not become a dialect-version error.

---

194. POCO-REAF Acceptance Test

The decisive test is:

ONE SOURCE PROGRAM
        |
        v
ONE LANGUAGE VERSION
        |
        v
ONE DIALECT VERSION SET
        |
        v
COMMON SEMANTICS
        |
        +-------------------+
        |                   |
        v                   v
   SMALL TARGET        LARGE TARGET
        |                   |
        +---------+---------+
                  |
                  v
          SAME PROGRAM MEANING

The target may choose different:

- representations;
- instruction sets;
- kernels;
- routing;
- scheduling;
- memory placement;
- QPU mappings;
- FPGA structures;
- distributed partitions;
- runtime strategies.

The dialect version remains the same unless the dialect semantics themselves change.

---

195. Architectural Non-Negotiables

The following rules are permanent.

1. "grammar/dialects/versioning.g4" owns dialect-version syntax.

2. This document owns dialect-version semantics.

3. "grammar/compatibility/dialect-compatibility.md" owns broader dialect compatibility.

4. "grammar/spec/versioning.md" owns the cross-layer versioning model.

5. "grammar/specification/language-version.md" owns language-version meaning.

6. No second canonical dialect-version grammar may be introduced.

7. Dialect identity is open-world.

8. Dialect versions are independently identifiable.

9. Language versions and dialect versions are separate.

10. Grammar versions and dialect versions are separate.

11. AST versions and dialect versions are separate.

12. Semantic-model versions and dialect versions are separate.

13. IR versions and dialect versions are separate.

14. Compiler versions and dialect versions are separate.

15. Runtime versions and dialect versions are separate.

16. ABI versions and dialect versions are separate.

17. Target versions and dialect versions are separate.

18. Unknown versions fail closed.

19. Version resolution is deterministic.

20. Dependency conflicts cannot be resolved accidentally.

21. Dependency cycles must be detected.

22. Released stable dialect versions are immutable semantic contracts.

23. Breaking semantic changes require appropriate major-version evolution.

24. Compatible additions use appropriate minor-version evolution.

25. Compatible corrections use appropriate patch-version evolution.

26. Migration does not retroactively make incompatible versions identical.

27. Deprecated versions remain identifiable.

28. Provenance should preserve resolved dialect versions.

29. Resource insufficiency is not dialect incompatibility.

30. Capability absence is not dialect incompatibility.

31. Target infeasibility is not dialect incompatibility.

32. Hardware generation is not dialect versioning.

33. QPU generation is not dialect versioning.

34. No fixed machine capacity belongs in dialect versioning.

35. No artificial dialect-count limit exists.

36. No artificial dependency-depth limit exists.

37. No artificial version-component limit exists.

38. No artificial program-size limit exists.

39. Quantum dialects converge through "quantum::ir".

40. Dialects cannot create a parallel semantic universe.

41. Dialects cannot bypass type/effect/capability/resource/policy validation.

42. Rust 1.97 or later is supported.

43. Rust edition 2021 remains the implementation baseline unless the repository explicitly changes it.

44. "unsafe" Rust is prohibited.

45. Version resolution must be implementable using safe Rust.

46. Future dialects must be addable without changing the universal versioning architecture.

47. Application-specific functionality belongs in appropriate libraries, dialects, capabilities, policies, or applications rather than becoming an uncontrolled universal keyword set.

---

196. Final Production Contract

A Zamani dialect version means:

«A specific, immutable, identifiable public contract describing the syntax/semantic extension and associated compatibility requirements of a dialect at a particular point in its evolution.»

It does not mean:

«a compiler version, hardware generation, target machine, runtime instance, device identifier, resource capacity, or physical execution configuration.»

The complete architecture is therefore:

                    ZAMANI SOURCE
                          |
                          v
                    Language Version
                          |
                          v
                    Dialect Identity
                          |
                          v
                    Dialect Version
                          |
                          v
              Dialect Dependency Resolution
                          |
                          v
             Compatibility / Feature Gates
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
               Structural Validation
                          |
       +------------------+------------------+
       |                  |                  |
      Types            Effects          Contracts
       |                  |                  |
       +------------------+------------------+
                          |
              Capabilities / Resources
                          |
                          v
                Semantic Validation
                          |
                          v
              Canonical Semantic Model
                          |
             +------------+------------+
             |                         |
             v                         v
       Classical IR               quantum::ir
             |                         |
             +------------+------------+
                          |
                          v
                     Optimization
                          |
                          v
                 Lowering / Routing
                          |
                          v
                     Scheduling
                          |
                          v
               Resilience / Recovery
                          |
                          v
                         ZQN
                          |
                          v
                         HAL
                          |
                          v
                       TARGET

The governing rule is:

«Dialect versioning describes portable semantic contracts; it does not describe the size, identity, generation, or availability of the machine on which those contracts are eventually realized.»

That separation is what allows a Zamani dialect to remain stable while its implementation scales from tiny systems through large heterogeneous systems, quantum systems, hardware designs, accelerators, HPC, clusters, distributed systems, and future computational substrates.

The result is a dialect-version architecture that supports Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF) without turning today's implementation limits into tomorrow's language limits.