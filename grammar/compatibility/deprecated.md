Zamani Deprecation and Removal Policy

Path: "grammar/compatibility/deprecated.md"
Status: Normative
Scope: Language-feature deprecation, compatibility preservation, migration coordination, removal eligibility, diagnostics, and historical retention
Rust baseline: Rust 1.97 or later
Rust edition: 2021
Implementation safety: Zamani's Rust implementation MUST use safe Rust only
Primary portability objective: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)

---

1. Purpose

This document defines the normative deprecation and removal policy for Zamani.

It establishes:

- what constitutes a deprecated feature;
- what does not constitute deprecation;
- who owns deprecation decisions;
- how deprecation is represented;
- how deprecated source remains compatible;
- how deprecation interacts with language versions;
- how deprecation interacts with grammar versions;
- how deprecation interacts with lexer and parser compatibility;
- how deprecation interacts with the domain-neutral AST;
- how deprecation interacts with semantic analysis;
- how deprecation interacts with canonical IR;
- how deprecation interacts with "quantum::ir";
- how deprecation interacts with classical, quantum, HDL, hardware, distributed, AI, data, networking, security, and future domains;
- how dialect deprecation works;
- how migrations are coordinated;
- how removal becomes eligible;
- how removal is validated;
- how diagnostics are represented;
- how compatibility is tested;
- how historical information is retained;
- how scalability and POCO-REAF are preserved.

The governing principle is:

«A representation may be deprecated when language evolution requires it, but specified semantic meaning MUST NOT be discarded merely because an implementation, backend, device, vendor, or machine cannot currently realize it.»

This document does not define ordinary feature semantics. It defines the lifecycle and compatibility consequences of changing or removing already-defined features.

---

2. Normative Terms

The following terms are normative:

Term| Meaning
MUST| Mandatory requirement
MUST NOT| Prohibited behavior
REQUIRED| Mandatory requirement
SHOULD| Recommended unless a documented technical reason exists
SHOULD NOT| Discouraged unless a documented technical reason exists
MAY| Permitted behavior
OPTIONAL| Permitted but not required

A feature is not considered officially deprecated merely because:

- a comment says it is deprecated;
- a historical document labels it obsolete;
- an implementation branch no longer uses it;
- a backend does not support it;
- a test has been disabled;
- a developer intends to remove it.

A normative deprecation requires an authoritative compatibility record and corresponding conformance evidence.

---

3. Scope

This policy applies to compatibility-sensitive changes involving:

source syntax
keywords
operators
punctuation
literal forms
grammar constructs
AST constructs
semantic constructs
types
effects
capabilities
resource declarations
contracts
policies
dialects
module interfaces
canonical IR
quantum::ir
serialized artifacts
ABI-facing declarations
compiler-facing interfaces
runtime-facing artifacts

It applies across:

classical
quantum
HDL
hardware
hybrid
AI
data
distributed
networking
security
embedded
accelerator
simulation
interoperability
future computational domains

It does not give this file ownership of those domains' semantics.

---

4. Authority Model

The repository uses separate authorities for language definition, implementation conformance, versioning, migration, and deprecation.

The relationship is:

grammar/DESIGN.md
        │
        ▼
grammar/specification/
        │
        │ human/normative language authority
        ▼
grammar/spec/
        │
        │ machine-oriented semantic and implementation contracts
        ▼
grammar/compatibility/
        │
        ├── language-version.md
        ├── grammar-version.md
        ├── AST-version.md
        ├── semantic-version.md
        ├── IR-version.md
        ├── dialect-version.md
        ├── target-compatibility-version.md
        ├── versions.md
        ├── migrations.md
        ├── compatibility-matrix.md
        └── deprecated.md

The responsibilities are intentionally separated.

"grammar/specification/"

Defines what the language means.

"grammar/spec/"

Defines machine-oriented contracts and cross-layer conformance.

"grammar/compatibility/"

Defines compatibility management, version relationships, migrations, and deprecation lifecycle.

"grammar/Zamani.g4"

Defines canonical grammar composition.

"grammar/grammar.md"

Reports implementation/conformance status.

"grammar/Zamani-Grammar.md"

Provides historical and extended grammar/design material and MUST NOT silently become normative authority.

---

5. Ownership of This File

5.1 This file owns

This file owns:

- deprecation definition;
- deprecation status;
- deprecation lifecycle;
- deprecation eligibility;
- removal eligibility;
- removal prerequisites;
- deprecation metadata requirements;
- removal metadata requirements;
- deprecation diagnostic requirements;
- compatibility requirements specific to deprecated features;
- relationship between deprecation and migration;
- relationship between deprecation and language versions;
- relationship between deprecation and grammar versions;
- rules for retaining deprecated syntax;
- rules for removing deprecated syntax;
- historical retention requirements.

5.2 This file does not own

This file does NOT own:

- language-version numbering;
- grammar syntax;
- lexer token definitions;
- AST design;
- ordinary semantic definitions;
- type-system semantics;
- effect semantics;
- resource semantics;
- capability semantics;
- canonical IR design;
- "quantum::ir" design;
- routing;
- scheduling;
- optimization;
- QEC;
- ZQN;
- HAL;
- runtime implementation;
- backend implementation;
- migration algorithms;
- dialect implementation;
- hardware capabilities.

Those remain owned by their respective specifications and implementation contracts.

---

6. Relationship to "grammar/compatibility/versions.md"

"grammar/compatibility/versions.md" owns the general version and compatibility policy.

It answers:

«What compatibility relationship exists between versions?»

This file answers:

«What happens when an existing feature is intentionally retired from future language evolution?»

"versions.md" MUST NOT redefine individual deprecation records.

This file MUST NOT redefine the language version numbering scheme.

A deprecation record MUST reference the applicable language version using the version model established by "versions.md".

---

7. Relationship to "grammar/compatibility/migrations.md"

"migrations.md" owns migration mechanics.

It defines:

- transformation algorithms;
- migration tooling;
- source transformation;
- AST transformation;
- semantic normalization;
- migration validation;
- migration reporting;
- automated and manual migration procedures.

This file defines:

- why a feature is deprecated;
- when deprecation begins;
- whether migration is required;
- whether removal is eligible;
- what compatibility guarantees apply.

This file MUST NOT duplicate migration algorithms.

---

8. Relationship to "grammar/compatibility/compatibility-matrix.md"

The compatibility matrix records compatibility relationships between:

- language versions;
- grammar versions;
- AST contracts;
- semantic contracts;
- IR versions;
- dialect versions;
- compiler versions;
- runtime artifacts;
- target contracts.

The matrix MUST represent deprecation status where applicable.

The matrix MUST NOT become the authority for deciding whether a feature is deprecated.

The deprecation record remains authoritative for the feature's lifecycle.

---

9. Relationship to Version-Specific Compatibility Files

This policy integrates with, but does not replace:

grammar/compatibility/language-version.md
grammar/compatibility/grammar-version.md
grammar/compatibility/AST-version.md
grammar/compatibility/semantic-version.md
grammar/compatibility/IR-version.md
grammar/compatibility/dialect-version.md
grammar/compatibility/target-compatibility-version.md

These files describe the versioning contract for their respective layers.

A deprecated feature MAY affect several of these layers simultaneously.

The layers MUST NOT be assumed to share the same version number.

For example:

language version
    ≠
grammar version
    ≠
AST version
    ≠
semantic version
    ≠
IR version
    ≠
dialect version
    ≠
target contract version

---

10. Definition of Deprecation

A feature is deprecated only when all of the following are true:

1. The feature has a stable or otherwise applicable compatibility identity.
2. A normative authority has approved its deprecation.
3. The feature's deprecated status is recorded.
4. The applicable language/version scope is identified.
5. The reason for deprecation is documented.
6. The preferred replacement, if one exists, is documented.
7. Compatibility behavior is defined.
8. Migration requirements are defined.
9. Diagnostic behavior is defined.
10. Removal eligibility requirements are defined.
11. Tests cover the lifecycle.

Deprecation means:

«The feature remains part of an applicable compatibility contract, but new source SHOULD prefer the replacement or successor representation.»

Deprecation does not by itself mean:

invalid
removed
unsupported
semantically wrong
unsafe
unavailable
target-specific
obsolete at runtime

---

11. Deprecation Is Not Implementation Status

The following are independent dimensions:

specified
implemented
partially implemented
experimentally implemented
AST-supported
semantically supported
IR-supported
compiler-supported
runtime-supported
backend-supported
target-supported
deprecated
removed

A feature can therefore legitimately be:

STABLE + BACKEND_UNAVAILABLE

or:

DEPRECATED + FULLY_IMPLEMENTED

or:

STABLE + PARTIALLY_IMPLEMENTED

These states MUST NOT be conflated.

Implementation status belongs to conformance documentation.

Deprecation status belongs to compatibility policy.

---

12. Implementation Failure Must Not Trigger Deprecation

The following MUST NOT independently justify deprecation:

- incomplete parser implementation;
- incomplete AST implementation;
- incomplete semantic implementation;
- incomplete IR lowering;
- incomplete backend;
- incomplete runtime;
- unavailable compiler optimization;
- unavailable routing;
- unavailable scheduling;
- unavailable QEC;
- unavailable ZQN functionality;
- unavailable HAL support;
- vendor limitations;
- accelerator limitations;
- simulator limitations;
- insufficient test hardware;
- compiler memory limitations;
- temporary implementation limitations.

Such conditions MUST be represented through their proper implementation or capability contracts.

---

13. Target Limitations Are Not Deprecation

A feature MUST NOT be deprecated because:

one CPU cannot execute it
one GPU cannot execute it
one FPGA cannot synthesize it
one ASIC cannot realize it
one QPU cannot execute it
one simulator cannot simulate it
one machine lacks memory
one machine lacks compute capacity
one topology cannot realize it
one cluster cannot provide required resources

Instead:

program requirement
        ↓
capability analysis
        ↓
resource analysis
        ↓
target feasibility
        ↓
realization or diagnostic

must determine whether the program can execute on that target.

---

14. POCO-REAF Invariant

Deprecation MUST preserve the separation between:

portable program meaning

and:

target realization

The canonical flow is:

source
  ↓
lexer
  ↓
parser
  ↓
domain-neutral AST
  ↓
semantic analysis
  ↓
canonical semantic model
  ↓
canonical IR
  ↓
optimization
  ↓
lowering
  ↓
routing
  ↓
scheduling
  ↓
resilience / QEC / ZQN
  ↓
HAL
  ↓
target realization

A deprecated representation MUST NOT be replaced with a target-specific semantic representation merely because that target is easier to support.

---

15. Scalability Invariant

Deprecation MUST NOT create artificial limits on the computational universe described by Zamani.

The language MUST NOT introduce fixed universal ceilings such as:

MAX_QUBITS
MAX_CPUS
MAX_CORES
MAX_THREADS
MAX_GPUS
MAX_FPGAS
MAX_ASICS
MAX_ACCELERATORS
MAX_NODES
MAX_MEMORY
MAX_REGISTER_WIDTH
MAX_TENSOR_RANK
MAX_TENSOR_DIMENSION
MAX_NETWORK_SIZE
MAX_DEVICE_COUNT
MAX_MODULE_COUNT
MAX_PROGRAM_SIZE

or equivalent mechanisms.

A compiler MAY have implementation limits caused by available resources.

Those limits are implementation/environment constraints and MUST NOT become language-level deprecation rules.

---

16. Resource Availability

Resource requirements remain valid independently of deprecation.

For example:

requires qubits >= n;
requires memory >= required_memory;
requires capability("quantum.measurement");
requires capability("gpu.compute");
requires capability("tensor.compute");
requires topology(required_topology);

If a target cannot satisfy the requirement:

requirement unsatisfied

is the correct result.

It is not:

feature deprecated

and it is not:

feature removed

---

17. Feature Identity

Every feature subject to deprecation MUST have a stable identity independent of its spelling or file location.

A feature identity SHOULD represent:

domain
namespace
feature
semantic role

For example:

quantum.operation

is a semantic identity.

An old spelling and a new spelling may therefore represent the same feature:

old spelling
    ↓
same semantic identity
    ↓
new spelling

Changing spelling does not automatically mean that the semantic feature changed.

---

18. Representation Deprecation Versus Semantic Deprecation

Two fundamentally different cases MUST be distinguished.

18.1 Representation deprecation

Example:

old syntax
    ↓
new syntax

while preserving:

same semantics

This is the preferred form of deprecation.

18.2 Semantic deprecation

A semantic operation itself is being removed or replaced.

This is a substantially stronger compatibility event.

It requires:

- explicit semantic rationale;
- language-version classification;
- migration analysis;
- AST impact analysis;
- semantic impact analysis;
- IR impact analysis;
- compatibility impact analysis;
- test migration;
- removal approval.

Semantic deprecation MUST NOT be disguised as a grammar change.

---

19. Deprecation Lifecycle

The normative lifecycle is:

PROPOSED
    │
    ▼
EXPERIMENTAL
    │
    ▼
STABLE
    │
    ▼
DEPRECATED
    │
    ├───────────────┐
    ▼               │
MIGRATION_AVAILABLE │
    │               │
    ▼               │
REMOVAL_ELIGIBLE    │
    │               │
    ▼               │
REMOVED ◄───────────┘

These statuses have different meanings.

Migration availability is a property of a deprecated feature, not necessarily a separate feature lifecycle state.

Therefore tooling SHOULD represent:

status = DEPRECATED
migration = AVAILABLE

rather than treating migration availability as an independent semantic state.

---

20. Proposed

"PROPOSED" means:

- design work is ongoing;
- no stable compatibility promise exists;
- the feature MAY change;
- the feature MAY be abandoned.

A proposed feature cannot normally be "deprecated" because it has not yet established a stable contract.

It MAY instead be withdrawn or superseded.

---

21. Experimental

"EXPERIMENTAL" means:

- implementation or specification is available for experimentation;
- semantics MAY change;
- syntax MAY change;
- compatibility guarantees are limited;
- removal MAY occur without the stable-feature removal process.

Experimental features MUST still have explicit status metadata.

Experimental status MUST NOT be used to hide an otherwise stable feature's breaking change.

---

22. Stable

"STABLE" means:

- the feature belongs to the supported language contract;
- compatibility rules apply;
- source meaning is defined;
- relevant AST/semantic/IR behavior is defined;
- tests exist at the applicable conformance levels.

A stable feature MUST NOT be silently removed.

---

23. Deprecated

"DEPRECATED" means:

- the feature remains recognized where the applicable version permits it;
- new source SHOULD avoid the feature;
- diagnostics SHOULD identify its deprecated status;
- migration guidance MUST be available;
- removal eligibility MUST be tracked.

Deprecation MUST NOT silently change the feature's semantics.

---

24. Removed

"REMOVED" means:

- the feature is not part of the applicable current language contract;
- new source using it MUST produce a compatibility error;
- a historical record MUST remain available;
- migration documentation MUST remain available where migration is meaningful.

A removed feature MUST NOT be reported merely as a warning.

---

25. Removal Eligibility

A deprecated feature becomes "REMOVAL_ELIGIBLE" only when all applicable requirements have been satisfied.

At minimum:

- deprecation has been publicly recorded;
- the applicable removal version is defined;
- migration documentation exists;
- a replacement exists, or permanent removal is justified;
- compatibility impact has been assessed;
- source diagnostics exist;
- affected AST contracts have been assessed;
- affected semantic contracts have been assessed;
- affected IR contracts have been assessed;
- affected dialects have been assessed;
- compatibility tests exist;
- migration tests exist where applicable;
- historical documentation exists;
- release documentation is prepared.

"REMOVAL_ELIGIBLE" does not itself remove the feature.

---

26. Normal Stable-Feature Removal

The normal lifecycle is:

STABLE
  ↓
DEPRECATED
  ↓
migration documented
  ↓
removal eligibility established
  ↓
future language version
  ↓
REMOVED

A stable feature MUST NOT silently jump directly to removal during ordinary language evolution.

---

27. Exceptional Security or Correctness Removal

An exceptional accelerated removal MAY occur when retaining the feature would create a demonstrated unacceptable:

- security defect;
- semantic corruption;
- integrity failure;
- correctness violation;
- unsoundness in the language contract.

Such removal requires:

1. documented technical rationale;
2. affected versions identified;
3. explicit compatibility classification;
4. migration or remediation guidance where feasible;
5. deterministic diagnostics;
6. tests;
7. release documentation;
8. historical record.

The exception MUST NOT be used merely to simplify implementation.

---

28. Deprecation Record

Every deprecated feature MUST have a complete record.

The minimum record is:

Feature ID:
Canonical Name:
Domain:
Namespace:
Feature Kind:
Semantic Role:

Status:
Introduced:
Stable Since:
Deprecated Since:
Removal Eligible Since:
Earliest Removal Version:

Reason:
Replacement:
Migration ID:
Migration Availability:
Migration Class:

Source Compatibility:
Lexical Compatibility:
Grammar Compatibility:
AST Compatibility:
Type Compatibility:
Semantic Compatibility:
Effect Compatibility:
Capability Compatibility:
Resource Compatibility:
Contract Compatibility:
Policy Compatibility:
IR Compatibility:
Runtime Compatibility:
Artifact Compatibility:
Dialect Compatibility:
Target Compatibility:

Semantic Preservation:
Lossless Migration:
Automatic Migration:
Manual Migration:
Migration Validation:

Diagnostic Code:
Diagnostic Severity:
Warning Conditions:
Error Conditions:

Affected Lexer Files:
Affected Grammar Files:
Affected AST Nodes:
Affected Semantic Components:
Affected IR Components:
Affected Compiler Components:
Affected Runtime Components:
Affected Dialects:

Positive Tests:
Negative Tests:
Boundary Tests:
Scalability Tests:
Determinism Tests:
Compatibility Tests:
Migration Tests:

Specification Reference:
Machine Contract Reference:
Compatibility Matrix Reference:
Migration Reference:
Provenance:

Owner:
Review Status:
Approval:

No field may be silently invented by an implementation.

---

29. Machine-Readable Deprecation Metadata

Where feature manifests exist, the corresponding record SHOULD contain fields equivalent to:

id:
name:
domain:
namespace:
kind:
semantic_role:

status:

introduced:
stable_since:
deprecated_since:
removal_eligible_since:
earliest_removal_version:

reason:
replacement:
migration:
migration_available:
migration_class:

compatibility:
  source:
  lexical:
  grammar:
  ast:
  type:
  semantic:
  effects:
  capabilities:
  resources:
  contracts:
  policies:
  ir:
  runtime:
  artifact:
  dialect:
  target:

semantic_preservation:
lossless_migration:
automatic_migration:
manual_migration:

diagnostic:
  code:
  severity:

integration:
  lexer:
  grammar:
  ast:
  semantic:
  ir:
  compiler:
  runtime:
  dialects:

tests:
  positive:
  negative:
  boundary:
  scalability:
  determinism:
  compatibility:
  migration:

specification:
machine_contract:
provenance:
owner:

The exact serialization format is owned by the repository's feature metadata contract.

This document defines the required semantics, not a mandatory serialization syntax.

---

30. One Feature, One Deprecation Identity

The same feature MUST NOT have contradictory lifecycle states in different files.

Invalid:

grammar/quantum/operation.g4
    → STABLE

compatibility/deprecated.md
    → DEPRECATED

feature manifest
    → EXPERIMENTAL

unless the discrepancy is explicitly documented as an implementation/specification conformance issue.

The canonical feature identity MUST be used to reconcile all records.

---

31. Lexer Deprecation

Lexical forms may be deprecated independently from their semantic meaning.

Possible lexical deprecations include:

keyword spelling
operator spelling
punctuation spelling
literal spelling
escape spelling
annotation spelling
identifier alias

A lexical deprecation MUST define:

- old spelling;
- token identity;
- replacement spelling;
- applicable versions;
- tokenization precedence;
- compatibility behavior;
- diagnostics;
- migration behavior.

A deprecated spelling MUST NOT introduce lexical ambiguity.

---

32. Central Lexer Authority

Canonical token ownership remains in:

grammar/lexer/
grammar/antlr/ZamaniLexer.g4

A deprecation record MUST NOT create a duplicate token definition.

If two spellings represent the same semantic token, the lexical architecture SHOULD preserve a common canonical token identity where appropriate.

---

33. Grammar Deprecation

A deprecated grammar rule MAY remain in the canonical grammar while the feature is supported.

The rule MUST remain deterministic.

Deprecation metadata MUST NOT be encoded solely by deleting or commenting out grammar rules.

The canonical source of grammar composition remains:

grammar/Zamani.g4

The deprecation policy does not authorize creation of a second root grammar.

---

34. Deprecated Syntax Must Remain Deterministic

For the same:

source
language version
dialect configuration
compatibility configuration

deprecated syntax MUST have deterministic interpretation.

Interpretation MUST NOT depend on:

- hardware;
- machine size;
- backend selection;
- runtime scheduling;
- random state;
- compiler process ordering;
- filesystem ordering;
- network availability.

---

35. Deprecated Syntax Must Not Be Silently Reinterpreted

If an old form and a new form have different semantics, the compiler MUST NOT silently treat the old form as the new form.

It MUST either:

1. preserve the old meaning;
2. require explicit migration;
3. reject the construct under the selected language version.

Silent semantic substitution is prohibited.

---

36. AST Compatibility

When deprecated syntax remains accepted, the frontend MUST preserve enough information to distinguish:

original construct
canonical semantic construct
deprecation metadata
source location

The AST MUST NOT silently discard information required for:

- diagnostics;
- migration;
- semantic validation;
- provenance;
- compatibility analysis;
- tooling.

If multiple source representations map to one semantic operation, they MAY share a canonical AST representation provided that required source provenance remains available.

---

37. Semantic Compatibility

Deprecation MUST preserve specified semantics while the feature remains supported.

The compiler MUST distinguish:

old representation

from:

semantic meaning

A parser change alone cannot redefine semantic compatibility.

A semantic change requires coordination with:

grammar/specification/
grammar/spec/
semantic contracts
compatibility/versioning
tests

---

38. Type Compatibility

If a deprecated construct affects types, compatibility analysis MUST determine whether:

- the type remains identical;
- the type is normalized;
- implicit conversion exists;
- explicit conversion is required;
- type inference changes;
- generic constraints change;
- ownership changes;
- linear/affine requirements change;
- dependent information changes.

A type-compatible migration MUST preserve the specified type semantics.

---

39. Effect Compatibility

Deprecated constructs MUST preserve effect semantics unless the effect contract explicitly changes.

Examples include:

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

A migration MUST NOT silently remove an effect merely because a backend does not implement it.

---

40. Capability Compatibility

Capabilities remain separate from deprecation.

For example:

requires capability("quantum.measurement");

does not become deprecated because one target lacks that capability.

The compatibility system MUST distinguish:

feature lifecycle

from:

target capability

---

41. Resource Compatibility

Resource requirements MUST remain target-independent.

For example:

requires qubits >= n;
requires memory >= required_memory;

remain valid regardless of the size of a particular target.

A resource failure is not evidence of deprecation.

---

42. Contract Compatibility

Deprecated features interacting with:

requires
ensures
invariant
assume
guarantee
property
assert

MUST preserve the contract semantics while the feature remains supported.

A migration MUST NOT silently weaken a contract.

---

43. Policy Compatibility

Deprecated constructs interacting with policies MUST preserve:

- authorization;
- restrictions;
- permissions;
- prohibitions;
- resource policies;
- execution policies;
- adaptation policies;
- security policies.

A deprecated feature MUST NOT become a way around a policy boundary.

---

44. Provenance

Deprecation-aware compilation SHOULD preserve:

source representation
feature ID
language version
deprecated version
migration applied
replacement
semantic normalization
compiler version
IR version
diagnostic identity

This is particularly important for:

- reproducible builds;
- audits;
- scientific computing;
- security analysis;
- AI decisions;
- quantum workflows;
- hardware compilation;
- long-lived artifacts.

---

45. Quantum Compatibility

Quantum deprecation MUST preserve the distinction between:

source quantum semantics

and:

physical realization

Applicable compatibility must be assessed across:

quantum syntax
      ↓
AST
      ↓
quantum semantics
      ↓
quantum::ir
      ↓
optimization
      ↓
routing
      ↓
scheduling
      ↓
QEC
      ↓
ZQN
      ↓
HAL

Deprecating a quantum operation spelling does not automatically deprecate the underlying quantum semantic operation.

A target lacking a native operation does not constitute language deprecation.

---

46. Generic Quantum Operations

Quantum operation compatibility MUST remain compatible with the data-driven operation architecture.

The language MUST NOT require the deprecation system to maintain an artificial hard-coded list of every possible operation.

An operation may be identified through:

operation identity
specifier
parameters
targets
results
attributes
modifiers
effects
capabilities
resources
provenance

If an old operation representation is replaced, migration MUST preserve its semantic identity where possible.

---

47. Classical Compatibility

Classical feature deprecation MUST preserve:

- type semantics;
- arithmetic semantics;
- control flow;
- memory semantics;
- ownership;
- effects;
- concurrency;
- resource requirements.

A compiler optimization is not a deprecation unless the language contract changes.

---

48. HDL and Hardware Compatibility

Deprecation of HDL constructs MUST distinguish:

language-level hardware intent

from:

specific implementation technology

A construct MUST NOT be deprecated solely because:

- a particular FPGA family lacks a primitive;
- a particular ASIC process lacks a feature;
- a particular simulator lacks support;
- a particular synthesis tool lacks lowering.

Such limitations belong to implementation, target, or capability contracts.

---

49. Distributed Compatibility

Deprecated distributed constructs MUST preserve their specified:

- ordering;
- consistency;
- messaging;
- actor behavior;
- channel behavior;
- failure semantics;
- replication semantics;
- transaction semantics.

A larger or smaller cluster MUST NOT change the language meaning merely because resources differ.

---

50. AI and Data Compatibility

Compatibility-sensitive AI/data constructs include:

inference
reasoning
knowledge
learning
adaptation
probability
uncertainty
confidence
evidence
provenance
explanation
decision records
agents
tensor semantics
data queries

Deprecation MUST apply to language representations rather than to particular external frameworks.

An implementation framework, model architecture, accelerator, or service is not automatically a language feature.

---

51. Interoperability Compatibility

External formats and interfaces MUST remain separately versioned.

Examples include:

SQL
JSON
XML
foreign functions
ABI contracts
external data schemas
external language adapters

A compatibility adapter MUST identify:

external format
external version
Zamani language version
adapter version
semantic mapping
lossiness

A lossy migration MUST NOT be described as lossless.

---

52. Dialect Deprecation

Dialect features have two compatibility dimensions:

dialect compatibility
language compatibility

A dialect feature MUST NOT silently become a core-language feature.

Conversely, deprecating a dialect construct MUST NOT automatically deprecate an equivalent universal Zamani semantic feature.

Dialect compatibility remains governed by:

grammar/compatibility/dialect-version.md
grammar/compatibility/dialect-compatibility.md

---

53. Reserved Identifiers

Deprecation of identifiers that affect reservation MUST coordinate with:

grammar/compatibility/reserved.md
grammar/lexer/tokens.g4
grammar/antlr/ZamaniLexer.g4

A newly reserved identifier can be a source-compatibility change even when it has no semantic change.

Therefore the compatibility record MUST distinguish:

semantic deprecation

from:

lexical reservation change

---

54. Feature Gates

Feature gates MAY control availability by language version or experimental status.

They MUST NOT be used to encode:

- machine size;
- hardware capacity;
- vendor-specific source semantics;
- arbitrary resource ceilings.

A feature gate answers:

«Is this language feature enabled under this language contract?»

It does not answer:

«Can this machine execute the program?»

---

55. Diagnostics

A supported deprecated feature SHOULD produce a structured diagnostic.

The diagnostic SHOULD include:

diagnostic code
feature ID
feature name
deprecated since
reason
replacement
migration ID
earliest removal version
source span

Example:

warning:
  code: <stable-diagnostic-code>
  feature: <feature-id>
  deprecated_since: <version>
  replacement: <replacement>
  migration: <migration-id>
  earliest_removal: <version>

Human-readable wording MAY evolve.

Machine-readable diagnostic identity MUST remain stable for the compatibility contract.

---

56. Diagnostic Severity

The preferred diagnostic levels are:

INFO
WARNING
ERROR

Use:

INFO

for inspection or tooling information.

Use:

WARNING

for a deprecated feature that remains valid.

Use:

ERROR

when the feature is not valid under the effective language contract.

A removed feature MUST NOT be reported merely as deprecated.

---

57. Diagnostic Determinism

The same:

source
language version
dialect configuration
compatibility configuration

MUST produce the same compatibility classification.

Diagnostic ordering MUST be deterministic.

Diagnostic identity MUST NOT depend on:

- hardware;
- target selection;
- resource availability;
- network state;
- backend ordering.

---

58. Warning Suppression

If warning suppression exists, it MAY suppress presentation of a deprecation warning.

It MUST NOT:

- restore removed syntax;
- alter semantics;
- bypass compatibility checking;
- bypass type checking;
- bypass effect checking;
- bypass capability checking;
- bypass resource validation;
- bypass security policy;
- bypass provenance requirements.

Suppression changes diagnostic presentation, not language validity.

---

59. Migration Classes

Every deprecated feature MUST classify migration as one of:

NONE
AUTOMATIC
SEMI_AUTOMATIC
MANUAL
SEMANTIC_REDESIGN

"NONE"

No migration is required because the feature remains permanently supported or no successor exists.

"AUTOMATIC"

A mechanically verified transformation preserves semantics.

"SEMI_AUTOMATIC"

Tooling can transform the representation but requires developer decisions.

"MANUAL"

No reliable automatic transformation exists.

"SEMANTIC_REDESIGN"

The replacement changes semantic intent and requires explicit developer action.

---

60. Lossless Migration

A migration is lossless only when all required semantics survive.

This includes, where applicable:

values
types
effects
capabilities
resources
contracts
policies
ordering
concurrency
quantum behavior
HDL behavior
provenance
source intent

If information is lost, the migration MUST report the loss.

Silent semantic loss is prohibited.

---

61. Migration and Canonical IR

Where a deprecated source representation has a compatible canonical semantic meaning, migration SHOULD normalize through the semantic layer:

deprecated source
      ↓
AST
      ↓
semantic normalization
      ↓
canonical semantic model
      ↓
canonical IR

Migration MUST NOT require a target-specific representation merely to remove deprecated syntax.

---

62. Migration and "quantum::ir"

Quantum migrations SHOULD preserve:

quantum semantics
      ↓
quantum::ir

If a deprecated source form maps to an existing quantum semantic operation, migration SHOULD preserve that operation rather than introduce a target-specific substitute.

---

63. Migration Must Be Explicit About Meaning

A migration tool MUST distinguish:

syntax rewrite

from:

semantic change

A tool MUST NOT claim semantic preservation when the migration changes program behavior.

---

64. Removal Procedure

Before removal:

DEPRECATED
    ↓
impact analysis
    ↓
migration available or justified
    ↓
tests updated
    ↓
compatibility matrix updated
    ↓
removal eligible
    ↓
release decision
    ↓
REMOVED

The removal change MUST update every affected compatibility artifact.

---

65. Required Removal Review

Removal review MUST consider:

source compatibility
lexical compatibility
grammar compatibility
AST compatibility
type compatibility
semantic compatibility
effect compatibility
capability compatibility
resource compatibility
contract compatibility
policy compatibility
IR compatibility
artifact compatibility
dialect compatibility
runtime compatibility
tooling compatibility

---

66. Removal Does Not Erase History

When a feature is removed:

- its feature ID MUST remain known;
- historical syntax MAY remain documented;
- migration guidance MUST remain available where useful;
- previous compatibility behavior MUST remain auditable;
- release history MUST identify the removal;
- provenance of migrated artifacts SHOULD remain recoverable.

Historical documentation does not make removed syntax valid.

---

67. Source Compatibility

Source compatibility asks:

«Can existing source still be parsed and interpreted under the applicable language contract?»

Deprecation SHOULD preserve source compatibility until removal.

If source compatibility is intentionally broken, the compatibility record MUST state:

affected versions
affected syntax
replacement
migration
diagnostic

---

68. Grammar Compatibility

Grammar compatibility asks:

«Can the canonical grammar continue to represent the supported language contract?»

A deprecated grammar rule MAY remain present.

Removal requires coordinated updates to:

grammar/Zamani.g4
modular grammar files
lexer contracts
parser contracts
tests

---

69. AST Compatibility

AST compatibility asks:

«Can the frontend represent the semantic information required by the effective language contract?»

Removing syntax MUST NOT accidentally remove semantic information required by another supported representation.

AST changes require coordination with:

AST versioning
frontend conformance
semantic analysis
migration
tests
tooling

---

70. Semantic Compatibility

Semantic compatibility asks:

«Does the program mean the same thing?»

This is more important than textual compatibility.

Two source forms may differ completely in syntax while remaining semantically compatible.

Conversely, identical syntax with changed semantics is a breaking semantic change.

---

71. IR Compatibility

IR compatibility asks:

«Can the semantic meaning be represented and consumed by the applicable canonical IR contract?»

A deprecated source feature MUST NOT be removed merely because a target-specific backend lacks a lowering.

If the semantic feature remains supported, its canonical IR representation remains authoritative.

For quantum semantics, the relevant boundary remains:

quantum semantics
      ↓
quantum::ir

---

72. Artifact Compatibility

Compiled or serialized artifacts MUST identify enough compatibility metadata to determine:

language contract
IR contract
dialect contracts
artifact format
compiler compatibility requirements
runtime compatibility requirements

A runtime MUST NOT guess source-language semantics from an artifact.

---

73. Runtime Compatibility

The runtime may validate:

artifact format
IR version
runtime contract
required capabilities
required resources
deployment policy

It MUST NOT redefine source-language deprecation semantics.

---

74. Target Compatibility

Target compatibility concerns:

capabilities
resources
topology
performance
reliability
availability
deployment constraints

Target incompatibility MUST NOT be represented as source-language deprecation.

---

75. Target-Independent Deprecation

A feature's deprecation state MUST be independent of:

CPU generation
GPU generation
FPGA family
ASIC process
QPU generation
simulator implementation
cluster size
cloud provider
network topology
machine capacity

A target can reject a requirement without changing the language contract.

---

76. Reproducibility

Deprecation diagnostics and migration decisions MUST be reproducible.

Given equivalent:

source
language version
dialect versions
compatibility configuration
compiler compatibility rules

the result MUST be deterministic.

Migration tools SHOULD emit sufficient provenance to reproduce the transformation.

---

77. Compiler Version Independence

The compiler version is separate from language version.

For example:

language version = X
compiler version = Y

A newer compiler MAY implement the same language version.

Compiler implementation improvements MUST NOT automatically deprecate language features.

---

78. Rust Implementation Requirement

The Zamani implementation covered by this grammar architecture MUST target:

Rust 1.97 or later
edition 2021

The implementation MUST use safe Rust only.

The compatibility specification MUST NOT require compiler implementation techniques that depend on memory-unsafe Rust behavior.

This requirement applies to:

lexer
parser
AST
semantic analysis
compatibility tooling
migration tooling
IR tooling
diagnostics
test infrastructure

---

79. No Implementation-Capacity Constants

Deprecation tooling MUST NOT encode universal limits such as:

maximum feature count
maximum module count
maximum AST depth
maximum quantum count
maximum tensor rank
maximum target count
maximum device count
maximum resource count

Any practical limit belongs to the implementation environment and MUST be represented separately.

---

80. Version-Range Semantics

A deprecation record MUST identify the version scope in which it applies.

For example:

deprecated_since
earliest_removal_version

must use the canonical language-version model.

The record MUST NOT invent a second version syntax.

If a feature is deprecated only for a dialect or compatibility profile, that scope MUST be explicit.

---

81. Backward Compatibility

A newer compiler SHOULD continue accepting deprecated source when the selected language version permits it.

If the selected language version makes the feature invalid, the compiler MUST report an error.

The compiler MUST NOT silently reinterpret it as a different construct.

---

82. Forward Compatibility

A compiler MUST NOT assume that every future feature is deprecated, removed, or equivalent to an existing construct.

Unknown future syntax SHOULD be diagnosed according to the applicable language-version contract.

Future syntax MUST NOT be silently assigned an unrelated existing semantic meaning.

---

83. Feature Status and "grammar/grammar.md"

"grammar/grammar.md" remains the implementation/conformance status reference.

It MAY report:

SPECIFIED
IMPLEMENTED
PARTIALLY IMPLEMENTED
PLANNED
DEPRECATED

and associated implementation dimensions such as:

AST_IMPLEMENTED
SEMANTIC_IMPLEMENTED
IR_IMPLEMENTED
TESTED
STABLE
EXPERIMENTAL

However:

«"grammar/grammar.md" reports status; this file defines deprecation policy.»

The two documents MUST agree.

---

84. Extended/Historical Grammar Documentation

"grammar/Zamani-Grammar.md" may contain:

stable
proposed
experimental
deprecated
historical
not implemented

Those labels MUST correspond to authoritative compatibility metadata.

A feature appearing in the document does not by itself establish:

stable
implemented
deprecated
removed

---

85. Compatibility Matrix Integration

"grammar/compatibility/compatibility-matrix.md" MUST be able to answer:

Feature
Language Version
Grammar Version
AST Version
Semantic Contract
IR Version
Dialect Version
Compiler Compatibility
Runtime Compatibility
Target Compatibility
Deprecation State
Migration State

This file supplies the deprecation lifecycle.

The matrix records its relationships.

---

86. Migration Integration

"grammar/compatibility/migrations.md" MUST consume:

Feature ID
deprecated_since
earliest_removal_version
replacement
migration class
semantic preservation
compatibility impact

This file does not duplicate migration algorithms.

---

87. Dialect Integration

A dialect MUST be able to identify deprecated constructs independently.

Dialect deprecation records MUST reference:

dialect ID
dialect version
feature ID
replacement
migration
language compatibility

A dialect MUST NOT use deprecation to bypass core language validation.

---

88. Specification Integration

When a stable feature is deprecated, the relevant normative specification MUST be reviewed.

The specification MUST identify whether the change affects:

syntax
semantics
types
effects
resources
capabilities
contracts
policies
provenance
IR

This file records the lifecycle.

The domain specification remains the authority for the feature's meaning.

---

89. Root Grammar Integration

"grammar/Zamani.g4" remains the composition root.

Deprecation MUST NOT create:

deprecated root grammar
legacy root grammar
compatibility root grammar

Instead, the canonical root grammar and its modular components MUST represent the applicable language contract.

---

90. Lexer Integration

The lexer layer MUST:

- retain deprecated tokens while supported;
- reject removed lexical forms under applicable versions;
- preserve deterministic tokenization;
- maintain canonical token identity;
- avoid duplicate token concepts.

Deprecation records MUST reference the actual lexer token where one exists.

---

91. Parser Integration

The parser MUST:

- accept supported deprecated constructs;
- reject removed constructs under the applicable language version;
- preserve source locations;
- produce the correct AST representation;
- remain deterministic.

Parser implementation MUST NOT decide deprecation policy independently.

---

92. Semantic Integration

Semantic analysis MUST resolve:

effective language version
feature status
compatibility
deprecation
migration requirements
dialect compatibility
type compatibility
effect compatibility
capability compatibility
resource compatibility
contract compatibility
policy compatibility

Semantic analysis MUST happen before canonical IR generation.

---

93. Canonical IR Integration

A deprecated source representation SHOULD normalize to the same canonical semantic representation when semantics are unchanged.

The preferred path is:

old source
   ↓
AST
   ↓
semantic normalization
   ↓
canonical semantic operation
   ↓
canonical IR

rather than:

old source
   ↓
special legacy backend

---

94. Quantum IR Integration

Where the feature is quantum-related:

source
   ↓
AST
   ↓
quantum semantics
   ↓
quantum::ir

must remain the canonical semantic path.

Deprecation MUST NOT create a second quantum IR merely to preserve historical syntax.

---

95. Classical IR Integration

Classical constructs MUST normalize through the canonical classical semantic representation and its designated IR.

A deprecated classical syntax should not require a separate legacy IR unless a separately specified compatibility artifact requires one.

---

96. HDL/Hardware IR Integration

Deprecated HDL syntax SHOULD normalize to the current hardware semantic model when semantics are preserved.

Physical realization remains downstream.

A synthesis tool limitation MUST NOT determine language deprecation.

---

97. Cross-Domain Integration

A feature may participate in several domains.

For example:

learning
+
quantum
+
classical
+
distributed
+
resource requirements

Deprecation MUST be tracked by feature identity rather than by whichever grammar directory happens to contain the syntax.

Cross-domain semantics MUST be assessed before removal.

---

98. Controlled Reflection and Metaprogramming

If a deprecated construct participates in:

reflection
introspection
compile-time execution
code generation
macros
syntax-tree manipulation

the migration MUST preserve semantic validation boundaries.

Deprecation MUST NOT become a mechanism for bypassing:

type checking
effect checking
capability checking
resource checking
policy checking
provenance

---

99. Security Boundary

Deprecation and migration MUST preserve security properties.

A migration MUST NOT silently:

- broaden permissions;
- remove authorization requirements;
- remove capability requirements;
- remove sandbox restrictions;
- remove provenance;
- expose protected resources;
- weaken isolation.

Security-sensitive removal MAY use the accelerated removal process when justified.

---

100. Deterministic Compatibility Resolution

Compatibility resolution MUST be deterministic.

The effective status of a feature MUST be determined from explicit inputs such as:

language version
dialect version
compatibility profile
feature metadata

It MUST NOT depend on:

hardware
target size
randomness
runtime load
network availability
filesystem enumeration order

---

101. Compilation Order

Compatibility processing SHOULD occur before irreversible lowering.

Preferred flow:

source
  ↓
version resolution
  ↓
lexing
  ↓
parsing
  ↓
AST
  ↓
compatibility validation
  ↓
semantic analysis
  ↓
canonical semantic model
  ↓
IR
  ↓
optimization/lowering

A backend MUST NOT be allowed to decide whether source syntax is deprecated.

---

102. Error Ordering

When multiple compatibility problems exist, diagnostics SHOULD be emitted in deterministic source order, followed by deterministic feature/diagnostic ordering.

The implementation MUST NOT depend on hash-map iteration order or backend discovery order for compatibility diagnostics.

---

103. Deprecated Syntax and Formatting

Formatting MUST preserve semantic meaning.

A formatter MAY normalize deprecated syntax when explicitly requested, but it MUST NOT silently change the effective language version.

If a formatter performs migration, it SHOULD identify:

original feature
replacement
migration
semantic-preservation status

---

104. IDE and Tooling Integration

Tooling SHOULD expose:

- deprecated status;
- deprecation version;
- replacement;
- migration;
- earliest removal version;
- applicable language version;
- dialect compatibility.

Autocomplete SHOULD prefer non-deprecated forms.

Tooling MUST NOT suggest removed constructs as valid source.

---

105. Generated Source

Generators producing Zamani source MUST know the target language version.

Generated source SHOULD avoid deprecated features unless compatibility generation explicitly requires them.

Generated source MUST NOT silently claim a language version incompatible with its syntax.

---

106. Generated Artifacts

Generated artifacts SHOULD preserve compatibility metadata sufficient to determine:

language contract
IR contract
dialect contract
compiler requirements
runtime requirements
migration state
provenance

Artifact generation MUST NOT erase compatibility provenance.

---

107. Reproducible Migration

Migration tools SHOULD be deterministic.

Equivalent input under equivalent compatibility configuration MUST produce equivalent migrated output.

Where ordering is semantically irrelevant, canonical ordering SHOULD be used to improve reproducibility.

---

108. Migration Validation

Every automated migration MUST be validated at the strongest applicable level:

parse equivalence
AST equivalence
type equivalence
effect equivalence
resource equivalence
contract equivalence
policy equivalence
semantic equivalence
IR equivalence

The validation depth depends on the migration class.

---

109. Semantic Equivalence

Where semantic equivalence is claimed, validation MUST establish that the migrated representation preserves the language-defined meaning.

Compilation success alone does not prove semantic equivalence.

---

110. Resource and Capability Preservation

A migration MUST preserve explicit resource and capability requirements unless the migration specification explicitly changes them.

For example:

requires capability("quantum.measurement");

must not disappear simply because a replacement syntax happens to omit the textual requirement.

---

111. Effect Preservation

A migration MUST preserve effects.

For example, a migrated operation that originally required:

effect(network)

MUST NOT become effect-free merely because its new syntax is shorter.

---

112. Contract Preservation

A migration MUST preserve applicable:

requires
ensures
invariant
assume
guarantee
property
assertions

unless the migration explicitly identifies a semantic redesign.

---

113. Provenance Preservation

Migration tooling SHOULD retain:

original source location
original feature ID
original spelling
migration ID
replacement feature ID
migration tool version
language version
resulting representation

This supports long-lived reproducibility and auditing.

---

114. Deprecation of Keywords

When a keyword is deprecated, the repository MUST determine whether:

1. the keyword is removed;
2. the spelling becomes an identifier;
3. an alias replaces it;
4. the semantic construct remains but syntax changes.

These cases are not interchangeable.

A reserved identifier change MUST coordinate with the reservation policy.

---

115. Deprecation of Operators

Operator deprecation MUST evaluate:

precedence
associativity
tokenization
overload resolution
type semantics
effect semantics

An operator replacement MUST NOT silently alter precedence or evaluation semantics.

---

116. Deprecation of Literal Forms

Literal migration MUST preserve:

value
type
precision
units
encoding
escape semantics
source provenance

where these are part of the language contract.

---

117. Deprecation of Types

Type deprecation MUST distinguish:

type alias removal
type representation change
type semantic removal
type conversion requirement
generic constraint change
ownership change

A type alias may be deprecated without deprecating the underlying semantic type.

---

118. Deprecation of Effects

An effect may be deprecated independently of an operation using it.

The compatibility record MUST identify whether the change affects:

effect identity
effect inference
effect declaration
effect checking
effect ordering
effect propagation

---

119. Deprecation of Capabilities

Capability names and capability semantics are separate.

Changing a capability identifier MAY be a representation migration.

Changing what a capability guarantees is a semantic compatibility event.

A capability MUST NOT be removed solely because one target does not provide it.

---

120. Deprecation of Resource Contracts

Resource declarations MUST distinguish:

resource expression
resource requirement
resource constraint
resource preference
resource capability

Removing one syntax MUST NOT accidentally remove the underlying resource semantics.

---

121. Deprecation of Contracts

Contract constructs are compatibility-sensitive because they affect correctness.

Deprecating:

requires
ensures
invariant
assume
guarantee
property

requires analysis of:

verification
optimization assumptions
runtime checking
proof obligations
diagnostics

---

122. Deprecation of Policies

Policy constructs MUST be migrated without weakening security or execution constraints.

A policy migration MUST identify whether the result is:

equivalent
stricter
weaker
incomparable

A migration that weakens a policy MUST NOT claim lossless semantic preservation.

---

123. Deprecation of Provenance

Provenance fields MUST NOT be silently discarded during migration if they are required by the applicable contract.

Where provenance is optional, the migration SHOULD preserve it when available.

---

124. Compatibility With Future Computational Domains

Deprecation policy MUST remain domain-neutral.

A future domain can introduce:

syntax
AST mapping
semantic constructs
capabilities
resources
IR mapping
runtime contract
tests

without requiring a new deprecation architecture.

The feature lifecycle remains:

feature identity
    ↓
version scope
    ↓
status
    ↓
compatibility
    ↓
migration
    ↓
removal

---

125. Application-Level Features

Application-specific libraries, services, and dialects MUST NOT be treated as core-language features merely because they have language-facing syntax.

For example, application domains should normally remain outside the universal core when they can be expressed through:

libraries
dialects
capabilities
policies
APIs
services
data schemas

Deprecating an application library feature MUST NOT automatically deprecate the universal language primitives it uses.

---

126. Feature Promotion and Deprecation

A feature may evolve through:

PROPOSED
   ↓
EXPERIMENTAL
   ↓
STABLE

and later:

STABLE
   ↓
DEPRECATED

Promotion and deprecation are separate decisions.

An experimental feature that is abandoned MAY be removed without first becoming stable.

---

127. Experimental Removal

Experimental removal requires:

- status record;
- affected version scope;
- diagnostic behavior where applicable;
- historical documentation;
- tests updated;
- compatibility matrix updated.

It does not require the full stable-feature deprecation period unless the specification explicitly promises one.

---

128. Permanent Features

Not every deprecated feature needs a replacement.

A feature MAY be deprecated permanently when:

- it has no viable successor;
- retaining it conflicts with a stronger invariant;
- its semantics are demonstrably defective;
- security or correctness requires removal.

The record MUST explicitly state:

replacement = none

and explain why.

---

129. No Silent Removal

Deleting:

grammar rule
lexer token
AST node
semantic rule
IR mapping

without updating compatibility metadata is prohibited.

Repository deletion is not equivalent to language removal.

---

130. No Silent Resurrection

A removed feature MUST NOT become valid again accidentally because:

- a legacy parser branch remains;
- an old token is still emitted;
- a dialect reintroduces the spelling;
- an implementation fallback exists.

If a removed semantic feature is intentionally reintroduced, it receives a new compatibility decision and MUST be explicitly documented.

---

131. Feature Replacement

A replacement MUST identify:

old feature ID
new feature ID
semantic relationship
migration
compatibility

The replacement MAY be:

same semantic feature, new representation
new semantic feature
combination of features
library/dialect facility

The distinction MUST be explicit.

---

132. Combination Replacements

A deprecated feature MAY be replaced by several primitives.

For example:

old feature
   ↓
type + effect + capability + policy

In this case migration MUST verify that all required semantics are represented.

---

133. Library Replacements

A core-language construct MAY be replaced by a library facility only when the specification establishes that the semantics do not require core-language status.

The migration MUST distinguish:

language semantic guarantee

from:

library behavior

---

134. Dialect Replacements

A core feature MUST NOT be moved into a dialect merely to avoid compatibility obligations.

Such a change requires explicit language architecture review.

A dialect MUST NOT silently redefine the universal meaning of a core construct.

---

135. Compatibility Profiles

Where compatibility profiles exist, deprecation MUST identify the profile scope.

A feature MAY be:

supported in profile A
deprecated in profile B
removed in profile C

provided that the distinction is explicit and deterministic.

---

136. Cross-Version Compilation

When compiling source written for an older language version, the compiler SHOULD:

1. determine the declared/effective source version;
2. resolve compatibility rules;
3. identify deprecated constructs;
4. emit deterministic diagnostics;
5. migrate only when explicitly requested or contractually permitted;
6. validate the resulting semantics;
7. continue through the canonical semantic and IR pipeline.

---

137. Migration Must Precede Semantic Destruction

If an old AST or semantic construct is required to perform a lossless migration, it MUST remain available until migration can be performed or historical source can be diagnosed correctly.

The implementation MUST NOT delete the information first and attempt to reconstruct it later from textual guesses.

---

138. Source Spans

Deprecated constructs MUST preserve source locations where the frontend supports source locations.

At minimum:

source identifier
start position
end position

SHOULD be available for diagnostics.

Migration tools SHOULD retain source mapping from old constructs to new constructs.

---

139. Compatibility and Concurrency

Deprecation of concurrent constructs MUST consider:

ordering
synchronization
message delivery
memory visibility
actor semantics
channel semantics
task semantics
scheduler guarantees

A syntax-only migration MUST NOT silently change concurrency semantics.

---

140. Compatibility and Simulation

A simulation-only representation MUST NOT become the canonical meaning of a deprecated computational feature.

Simulation remains an execution strategy.

A migration MUST preserve the computation being simulated.

---

141. Compatibility and Adaptive Execution

Adaptive execution MAY respond to:

resource availability
device health
network state
load
faults
calibration
power
thermal conditions
reliability

These runtime conditions MUST NOT determine source-feature deprecation.

---

142. Compatibility and Deterministic Execution

If a language feature promises deterministic semantics, deprecation MUST preserve that promise.

If a feature explicitly permits nondeterminism or probabilistic behavior, that behavior remains part of its semantic contract.

A migration MUST NOT silently change:

deterministic
probabilistic
nondeterministic
implementation-defined

behavior classifications.

---

143. Compatibility and Randomness

If a deprecated feature involves randomness, migration MUST preserve the specified random semantics where those semantics are part of the language contract.

Compiler implementation randomness MUST NOT affect compatibility classification.

---

144. Compatibility and Learning

Learning-related constructs MUST distinguish:

language semantics
model/data semantics
implementation framework
hardware accelerator
runtime strategy

Deprecating a syntax for learning MUST NOT imply that learning as a computational capability is deprecated.

---

145. Compatibility and Adaptation

Controlled adaptation is a semantic operation when specified by the language.

Deprecation MUST preserve applicable:

policy
authorization
capability
effect
resource
provenance
validation

requirements.

A migration MUST NOT turn controlled adaptation into unrestricted program mutation.

---

146. Compatibility and Reflection

Reflection and metaprogramming changes MUST consider:

compile-time execution
syntax-tree access
type introspection
code generation
semantic validation
provenance

A migration MUST NOT create a hidden semantic escape around the compiler's validation architecture.

---

147. Compatibility and FFI/ABI

Foreign interfaces MUST distinguish:

language compatibility
FFI compatibility
ABI compatibility
external API compatibility

A deprecated FFI declaration does not automatically mean that the external ABI is deprecated.

Conversely, an external ABI change does not automatically change the Zamani language version.

---

148. Compatibility and Security

Security-sensitive deprecation requires additional review when it affects:

authorization
capabilities
sandboxing
isolation
cryptographic contracts
provenance
auditability
foreign/native interfaces
reflection
code generation

A migration MUST preserve security invariants.

---

149. Compatibility and Memory Safety

The language/compiler compatibility process MUST preserve the memory-safety architecture of the implementation.

The Rust implementation remains safe Rust.

Deprecation MUST NOT introduce a compatibility mechanism requiring unsafe implementation techniques.

---

150. Performance

Deprecation processing SHOULD scale with source size and feature metadata size.

Implementations SHOULD avoid:

- repeated whole-program reparsing;
- unnecessary compatibility passes;
- pathological dependency traversal;
- unbounded recursive migration loops;
- duplicate diagnostics.

Performance optimizations MUST preserve deterministic compatibility results.

---

151. Compatibility Dependency Graph

Feature deprecation SHOULD be represented as a dependency graph:

deprecated feature
       │
       ├── lexer
       ├── grammar
       ├── AST
       ├── semantics
       ├── types
       ├── effects
       ├── capabilities
       ├── resources
       ├── contracts
       ├── policies
       ├── IR
       ├── dialects
       ├── compiler
       ├── runtime
       └── tests

Removal is permitted only after all required dependencies have been assessed.

---

152. No Circular Compatibility Authority

Compatibility documents MUST NOT form a circular authority chain.

The intended direction is:

language specification
        ↓
machine contracts
        ↓
version policy
        ↓
deprecation
        ↓
migration
        ↓
compatibility matrix
        ↓
implementation conformance

References MAY be bidirectional for navigation, but semantic authority MUST remain unambiguous.

---

153. Repository File Contract

This file's complete contract is:

Purpose

Define deprecation and removal lifecycle.

Owns

deprecation
removal eligibility
removal prerequisites
deprecation diagnostics
deprecation compatibility
historical retention

Does Not Own

language semantics
version numbering
grammar
lexer
AST
IR design
migration algorithms
runtime
hardware
QEC
ZQN
HAL

Inputs

language specification
version policy
feature identity
compatibility contracts
feature metadata
implementation conformance
migration metadata

Outputs

deprecation classification
removal eligibility
diagnostic requirements
migration requirements
compatibility requirements
historical retention requirements

Upstream Contracts

grammar/DESIGN.md
grammar/specification/
grammar/spec/
grammar/compatibility/versions.md
grammar/compatibility/language-version.md
grammar/compatibility/grammar-version.md
grammar/compatibility/AST-version.md
grammar/compatibility/semantic-version.md
grammar/compatibility/IR-version.md
grammar/compatibility/dialect-version.md
grammar/compatibility/target-compatibility-version.md

Downstream Consumers

grammar/compatibility/migrations.md
grammar/compatibility/compatibility-matrix.md
grammar/compatibility/feature-gates.md
grammar/compatibility/frontend-conformance.md
grammar/compatibility/ast-conformance.md
grammar/compatibility/ir-conformance.md
grammar/compatibility/dialect-compatibility.md
grammar/grammar.md
grammar/Zamani.g4
lexer
parser
AST
semantic analysis
IR generation
compiler
migration tooling
diagnostics
IDE/tooling
tests

---

154. Required Feature Traceability

Every deprecated feature SHOULD be traceable:

Feature ID
    ↓
specification
    ↓
version introduced
    ↓
grammar rule
    ↓
lexer token
    ↓
AST node
    ↓
semantic construct
    ↓
effect/capability/resource contract
    ↓
canonical IR
    ↓
compiler consumer
    ↓
runtime/artifact consumer
    ↓
migration
    ↓
tests

A feature with an incomplete trace MUST NOT be considered ready for removal.

---

155. Reverse Traceability

The reverse path MUST also be possible:

IR/compiler construct
    ↓
semantic construct
    ↓
AST
    ↓
grammar
    ↓
source feature
    ↓
compatibility identity
    ↓
deprecation state

This prevents orphaned legacy implementation paths.

---

156. Required Test Categories

Every deprecated feature MUST have applicable tests for:

lexical compatibility
parser compatibility
AST compatibility
semantic compatibility
type compatibility
effect compatibility
capability compatibility
resource compatibility
contract compatibility
policy compatibility
IR compatibility
dialect compatibility
diagnostics
migration

Where applicable, also:

quantum
classical
HDL
hardware
distributed
networking
AI
data
security
interoperability
metaprogramming
simulation

---

157. Positive Tests

Positive tests verify that supported deprecated syntax remains valid under the versions where it is promised.

Tests MUST verify:

parse succeeds
AST is correct
semantic interpretation is correct
diagnostic is correct
canonical representation is correct

where applicable.

---

158. Negative Tests

Negative tests verify that:

- removed features fail;
- unsupported versions fail;
- incompatible dialects fail;
- invalid migrations fail;
- semantic-loss claims are rejected.

---

159. Boundary Tests

Boundary tests MUST cover transitions such as:

last supported version
first deprecated version
migration boundary
earliest removal version
first removed version

Where version ranges apply, every boundary MUST be deterministic.

---

160. Scalability Tests

Deprecation and migration tests MUST not establish artificial universal limits.

Test dimensions SHOULD include:

small source
large source
many modules
deep dependency graphs
many features
large ASTs
large IRs
many dialects
large resource expressions
large quantum programs
large HDL designs
large distributed programs

Where an environment cannot execute a large test, the limitation belongs to the test environment, not the language contract.

---

161. Determinism Tests

Compatibility tests MUST verify:

same source
same version
same dialect configuration
same compatibility configuration

produce the same:

feature status
diagnostics
migration result
semantic classification

---

162. Migration Tests

Every automatic migration MUST include:

before
migration
after
semantic comparison
IR comparison where applicable
diagnostic comparison
provenance comparison

Manual migrations MUST contain explicit human-action requirements.

---

163. POCO-REAF Compatibility Tests

At least one compatibility test suite MUST demonstrate that a deprecated representation does not force target-specific source changes.

Conceptually:

same source
   ↓
same language semantics
   ↓
canonical representation
   ↓
different target realizations

Targets may vary in:

size
resources
capabilities
topology
architecture
device type

without changing the deprecation state of the source feature.

---

164. Hard-Coding Audit

Every deprecation implementation MUST be audited for forbidden fixed ceilings.

The audit MUST search for constants or equivalent logic representing:

maximum qubits
maximum processors
maximum threads
maximum GPUs
maximum FPGAs
maximum devices
maximum nodes
maximum memory
maximum tensor dimensions
maximum network size

Such implementation limits MUST be classified as environment/implementation constraints rather than language semantics.

---

165. Completion Criteria

This file is complete when:

- [ ] deprecation is formally defined;
- [ ] removal is formally defined;
- [ ] feature identity is defined;
- [ ] lifecycle is defined;
- [ ] removal eligibility is defined;
- [ ] implementation failure is separated from deprecation;
- [ ] target capability is separated from deprecation;
- [ ] resource availability is separated from deprecation;
- [ ] language versions are separated from compiler versions;
- [ ] grammar versions are separated from language versions;
- [ ] AST versions are separated from language versions;
- [ ] semantic versions are separated from language versions;
- [ ] IR versions are separated from language versions;
- [ ] dialect versions are separated from language versions;
- [ ] target versions are separated from language versions;
- [ ] migration ownership is defined;
- [ ] compatibility-matrix ownership is defined;
- [ ] lexer integration is defined;
- [ ] parser integration is defined;
- [ ] AST integration is defined;
- [ ] semantic integration is defined;
- [ ] type integration is defined;
- [ ] effect integration is defined;
- [ ] capability integration is defined;
- [ ] resource integration is defined;
- [ ] contract integration is defined;
- [ ] policy integration is defined;
- [ ] provenance integration is defined;
- [ ] classical integration is defined;
- [ ] quantum integration is defined;
- [ ] "quantum::ir" integration is defined;
- [ ] HDL integration is defined;
- [ ] hardware independence is defined;
- [ ] dialect integration is defined;
- [ ] interoperability integration is defined;
- [ ] security integration is defined;
- [ ] safe Rust requirement is defined;
- [ ] deterministic diagnostics are defined;
- [ ] reproducibility is defined;
- [ ] scalability is defined;
- [ ] hard-coding audit is defined;
- [ ] positive tests are defined;
- [ ] negative tests are defined;
- [ ] boundary tests are defined;
- [ ] scalability tests are defined;
- [ ] determinism tests are defined;
- [ ] migration tests are defined;
- [ ] historical retention is defined.

---

166. Production-Readiness Gate

A deprecated feature is ready for production deprecation only when:

SPECIFICATION
     ↓
FEATURE IDENTITY
     ↓
VERSION SCOPE
     ↓
COMPATIBILITY CLASSIFICATION
     ↓
DEPRECATION RECORD
     ↓
DIAGNOSTICS
     ↓
MIGRATION
     ↓
AST IMPACT
     ↓
SEMANTIC IMPACT
     ↓
IR IMPACT
     ↓
DIALECT IMPACT
     ↓
TESTS
     ↓
COMPATIBILITY MATRIX
     ↓
RELEASE DOCUMENTATION

is complete.

A feature is ready for removal only when:

DEPRECATED
     ↓
MIGRATION READY
     ↓
COMPATIBILITY IMPACT COMPLETE
     ↓
TESTS COMPLETE
     ↓
REMOVAL ELIGIBLE
     ↓
APPROVED LANGUAGE VERSION
     ↓
REMOVED

---

167. Final Repository Integration Contract

The following files MUST remain coordinated.

"grammar/specification/language-version.md"

Owns the meaning of language versions.

Must reference this policy for feature lifecycle consequences.

"grammar/spec/compatibility.md"

Owns the broader compatibility model.

Must reference this policy for deprecation semantics.

"grammar/spec/versioning.md"

Owns implementation-level version propagation and conformance.

Must consume deprecation metadata.

"grammar/compatibility/versions.md"

Owns general versioning and compatibility policy.

Must not redefine individual feature deprecation semantics.

"grammar/compatibility/migrations.md"

Owns migration procedures.

Must consume feature deprecation records.

"grammar/compatibility/compatibility-matrix.md"

Records compatibility relationships and current status.

Must represent deprecated/removed states consistently.

"grammar/compatibility/feature-gates.md"

Controls feature availability.

Must consume, not redefine, deprecation status.

"grammar/compatibility/reserved.md"

Owns reservation behavior.

Must be updated when deprecated keywords become identifiers or reservation rules change.

"grammar/compatibility/dialect-compatibility.md"

Owns dialect compatibility.

Must coordinate dialect-specific deprecation.

"grammar/grammar.md"

Reports implementation status.

Must not independently declare a feature deprecated.

"grammar/Zamani-Grammar.md"

Retains extended/historical information.

Must not independently establish normative deprecation.

"grammar/Zamani.g4"

Remains the canonical composition root.

Must contain only the applicable canonical grammar composition.

"grammar/lexer/tokens.g4"

Remains the token registry.

Deprecated token records must remain consistent with it.

"grammar/antlr/ZamaniLexer.g4"

Remains the canonical lexer implementation contract.

"src/lexer.rs"

Must conform to the lexical contract.

"src/parser.rs"

Must conform to parser and compatibility contracts.

"src/frontend/ast/"

Must preserve information required by compatibility and migration.

Canonical semantic layer

Must resolve compatibility before irreversible lowering.

Classical IR

Must preserve semantics for supported deprecated representations.

"quantum::ir"

Remains the canonical quantum semantic IR boundary.

QEC / ZQN / routing / scheduling / HAL

Remain downstream realization systems and MUST NOT determine language deprecation.

---

168. Forbidden Architecture

The following architecture is prohibited:

backend limitation
      ↓
language feature deprecated

small machine
      ↓
language feature removed

vendor limitation
      ↓
language semantics changed

new parser
      ↓
old construct silently reinterpreted

migration
      ↓
semantic information discarded

deprecated feature
      ↓
special legacy IR
      ↓
target-specific meaning

compatibility file
      ↓
new independent language grammar

---

169. Required Architecture

The correct architecture is:

LANGUAGE SPECIFICATION
        ↓
LANGUAGE VERSION
        ↓
FEATURE IDENTITY
        ↓
DEPRECATION POLICY
        ↓
COMPATIBILITY CONTRACT
        ↓
MIGRATION CONTRACT
        ↓
LEXER
        ↓
CANONICAL GRAMMAR
        ↓
DOMAIN-NEUTRAL AST
        ↓
SEMANTIC ANALYSIS
        ↓
EFFECT / CAPABILITY / RESOURCE / CONTRACT / POLICY VALIDATION
        ↓
CANONICAL SEMANTIC MODEL
        ↓
CLASSICAL IR / quantum::ir / HDL-HARDWARE IR
        ↓
OPTIMIZATION
        ↓
LOWERING
        ↓
ROUTING
        ↓
SCHEDULING
        ↓
RESILIENCE / QEC / ZQN
        ↓
HAL
        ↓
TARGET REALIZATION

Deprecation operates across this pipeline as metadata and compatibility policy.

It does not replace the pipeline.

---

170. Final POCO-REAF Invariant

The ultimate invariant is:

«A Zamani program's semantic meaning is determined by its language contract and declared intent, not by the size, vendor, topology, generation, temporary availability, or implementation limitations of the machine on which it is realized.»

Therefore:

program
   ↓
portable semantics
   ↓
canonical representation
   ↓
target-independent optimization
   ↓
target realization

A deprecated source representation MAY change.

The program's specified semantics MUST remain stable for as long as the applicable compatibility contract promises them.

---

171. Final Conformance Statement

"grammar/compatibility/deprecated.md" is the normative owner of deprecation and removal lifecycle policy.

It does not become a second language specification.

It does not become a second versioning system.

It does not become a migration implementation.

It does not become a target-selection mechanism.

It does not encode hardware capacity.

It does not determine quantum physical feasibility.

It does not own canonical IR design.

It does not redefine domain semantics.

Its responsibility is precise:

WHAT IS DEPRECATED
        ↓
WHY
        ↓
SINCE WHEN
        ↓
WHAT REMAINS COMPATIBLE
        ↓
HOW IT MIGRATES
        ↓
WHEN REMOVAL MAY OCCUR
        ↓
HOW REMOVAL IS VERIFIED
        ↓
HOW HISTORY IS PRESERVED

The production invariant is:

DEPRECATION
     ≠
IMPLEMENTATION FAILURE

DEPRECATION
     ≠
RESOURCE FAILURE

DEPRECATION
     ≠
CAPABILITY FAILURE

DEPRECATION
     ≠
TARGET FAILURE

DEPRECATION
     ≠
COMPILER VERSION

DEPRECATION
     ≠
IR VERSION

DEPRECATION
     ≠
HARDWARE VERSION

and:

LANGUAGE EVOLUTION
        ↓
EXPLICIT COMPATIBILITY CONTRACT
        ↓
DETERMINISTIC DIAGNOSTICS
        ↓
EXPLICIT MIGRATION
        ↓
SEMANTIC PRESERVATION
        ↓
CONTROLLED REMOVAL

This is the required compatibility foundation for Zamani to evolve indefinitely while preserving a single target-independent language architecture capable of expressing classical, quantum, HDL, hybrid, AI, data, distributed, networking, embedded, accelerator, and future computational systems from very small realizations to arbitrarily large realizations constrained only by their actual semantic requirements and available resources.