Zamani Deprecation Policy

Path: "grammar/compatibility/deprecated.md"
Status: Normative
Language: Zamani
Scope: Source-language features, lexical constructs, grammar constructs, AST contracts, semantic constructs, dialects, interoperability constructs, tooling-facing syntax, and other user-visible language features subject to deprecation
Implementation baseline: Rust 1.97 / Rust 1.97.1
Safety requirement: Zamani's Rust implementation MUST use safe Rust; "unsafe" Rust MUST NOT be required by this policy or by its implementation
Primary objective: Preserve semantic stability, portability, scalability, extensibility, and POCO-REAF while allowing controlled language evolution

POCO-REAF:

«Program Once, Compile Once, Run Everywhere, Anywhere, Forever»

---

1. Purpose

This document defines the normative deprecation policy for Zamani.

It establishes:

- what may be deprecated;
- why a feature may be deprecated;
- how deprecation is declared;
- how deprecation is communicated;
- how long deprecated functionality remains supported;
- how migration paths are defined;
- how deprecated syntax interacts with the grammar;
- how deprecated constructs interact with the AST and semantic model;
- how deprecated constructs interact with canonical IR;
- how quantum, classical, HDL, hardware, distributed, AI, networking, security, and future-domain features are deprecated;
- how removal is approved;
- how compatibility is preserved;
- how tooling reports deprecated constructs;
- how deprecation is tested;
- how deprecation avoids introducing artificial scalability limits.

Deprecation MUST be deliberate, explicit, deterministic, documented, testable, and reversible until the removal stage.

A feature MUST NOT become deprecated merely because its current implementation is incomplete.

A feature MUST NOT be removed merely because a particular backend or machine cannot support it.

---

2. Fundamental Principle

The governing principle is:

«Deprecate representations when necessary; preserve stable semantics whenever possible.»

Deprecation is a language-evolution mechanism.

It is NOT:

- a substitute for fixing implementation bugs;
- a substitute for backend capability management;
- a substitute for resource management;
- a substitute for migration tooling;
- a way to encode hardware limitations;
- a way to remove difficult compiler functionality without a compatibility decision.

The preferred lifecycle is:

PROPOSED
   ↓
EXPERIMENTAL
   ↓
STABLE
   ↓
DEPRECATED
   ↓
MIGRATION AVAILABLE
   ↓
REMOVAL ELIGIBLE
   ↓
REMOVED

A feature MAY move directly from experimental status to removal when it was never promised as stable.

A stable feature MUST NOT silently jump from:

STABLE

to:

REMOVED

without an explicit compatibility decision.

---

3. Ownership Boundaries

This file owns deprecation policy.

It does NOT own:

- language version numbering;
- migration mechanics;
- reserved identifiers;
- compatibility matrices;
- canonical grammar authority;
- semantic definitions;
- compiler implementation;
- runtime policy;
- hardware capability definitions.

The ownership relationship is:

specification/language-version.md
        │
        │ owns language-version semantics
        ▼
specification/compatibility.md
        │
        │ owns compatibility dimensions
        ▼
compatibility/versions.md
        │
        │ owns version policy
        ▼
compatibility/deprecated.md
        │
        ├──────────────► compatibility/migrations.md
        │
        ├──────────────► compatibility/reserved.md
        │
        └──────────────► compatibility/compatibility-matrix.md

3.1 "versions.md"

Owns:

- language version numbering;
- major/minor/patch rules;
- version compatibility classes;
- version declaration;
- language/compiler-version separation.

3.2 "migrations.md"

Owns:

- migration procedures;
- source transformation;
- AST migration;
- semantic migration;
- artifact migration;
- migration validation;
- migration tooling.

3.3 "deprecated.md"

Owns:

- deprecation lifecycle;
- deprecation eligibility;
- deprecation metadata;
- warnings and diagnostics;
- removal eligibility;
- deprecation compatibility guarantees.

3.4 "reserved.md"

Owns:

- reserved identifiers;
- reserved keywords;
- future-reserved syntax;
- reserved namespaces.

3.5 "compatibility-matrix.md"

Owns:

- compatibility relationships among language versions;
- compiler versions;
- grammar versions;
- AST schemas;
- IR schemas;
- dialect versions;
- runtime protocols.

No file may silently redefine another file's ownership.

---

4. Definition of Deprecation

A feature is deprecated when Zamani continues to recognize and support the feature under an explicitly defined compatibility policy, while communicating that new source should normally use a replacement or newer representation.

Deprecation does NOT automatically mean:

- unsupported;
- invalid;
- removed;
- unsafe;
- inefficient;
- hardware-specific;
- semantically obsolete.

A deprecated construct remains usable until the applicable removal policy says otherwise.

---

5. What Can Be Deprecated

The following may be deprecated independently:

5.1 Lexical features

Examples:

- keywords;
- literal spellings;
- escape forms;
- operator spellings;
- annotation spellings;
- comment forms.

5.2 Grammar constructs

Examples:

- declarations;
- statements;
- expressions;
- module syntax;
- legacy syntax aliases.

5.3 Semantic constructs

Examples:

- obsolete ownership semantics;
- obsolete effect forms;
- obsolete resource declarations;
- obsolete execution semantics.

5.4 Types

Examples:

- obsolete built-in types;
- legacy aliases;
- superseded type constructors.

5.5 APIs and language-facing interfaces

Examples:

- legacy standard-library interfaces;
- old module paths;
- legacy intrinsics.

5.6 Dialects

A dialect may be deprecated independently from the core language.

5.7 Interoperability constructs

Examples:

- obsolete FFI declarations;
- obsolete ABI declarations;
- legacy interoperability syntax.

5.8 Tooling syntax

Examples:

- obsolete annotations;
- legacy compilation directives;
- deprecated source-level configuration constructs.

5.9 Serialization or schema forms

A serialized language representation may be deprecated without deprecating the corresponding source construct.

5.10 Hardware-facing syntax

Hardware syntax may be deprecated when a more portable semantic abstraction replaces it.

However, target-specific implementation constraints MUST NOT be treated as language deprecations.

---

6. What Must Not Be Deprecated for the Wrong Reason

The following are NOT valid reasons to deprecate a language construct by themselves:

- a particular CPU cannot execute it;
- a particular GPU lacks support;
- a QPU has too few physical qubits;
- a device has insufficient memory;
- a topology cannot realize an operation;
- a backend has not implemented it;
- a scheduler has not implemented it;
- a runtime currently lacks a feature;
- a vendor prefers another representation;
- a particular machine is too small;
- a test environment is resource constrained.

These are implementation, target, capability, or resource issues.

They must be represented through:

- capabilities;
- requirements;
- constraints;
- targets;
- resource contexts;
- scheduling;
- routing;
- runtime availability;
- deployment configuration.

They MUST NOT be disguised as source-language deprecation.

---

7. POCO-REAF Requirement

Deprecation MUST preserve the distinction between:

Program Semantics

and:

Target Realization

A deprecated construct MUST NOT be replaced by a target-specific construct merely because the target implementation is easier.

For example, this is acceptable:

legacy_quantum_operation
        ↓
portable quantum operation
        ↓
quantum::ir
        ↓
target-specific realization

This is NOT acceptable as a general migration:

legacy_quantum_operation
        ↓
specific-qpu-operation

unless the source program explicitly requested that target-specific behavior.

Likewise:

requires parallel execution

MUST NOT be deprecated into:

requires exactly N cores

merely because one compiler backend currently uses a fixed configuration.

---

8. No Hard-Coded Scalability Limits

Deprecation policy MUST remain independent of physical scale.

No deprecated feature may be replaced with a construct that permanently hard-codes:

- maximum qubits;
- maximum logical qubits;
- maximum physical qubits;
- maximum CPUs;
- maximum cores;
- maximum threads;
- maximum GPUs;
- maximum FPGAs;
- maximum accelerators;
- fixed memory capacity;
- fixed register count;
- fixed vector width;
- fixed tensor dimensions where abstraction is appropriate;
- fixed device counts;
- fixed cluster size;
- fixed network size;
- fixed topology;
- fixed program size;
- fixed module count;
- fixed data size.

A replacement MUST use scalable abstractions where the concept is inherently scalable.

---

9. Required Deprecation Metadata

Every deprecated feature MUST have a canonical deprecation record.

The conceptual record is:

Feature:
Stable Name:
Feature Kind:
Status:
Introduced:
Stable Since:
Deprecated Since:
Deprecation Reason:
Replacement:
Migration Path:
Migration Class:
Automatic Migration:
Manual Action:
Semantic Preservation:
Compatibility Guarantee:
Warning Policy:
Removal Eligibility:
Earliest Removal Version:
Removal Conditions:
Affected Files:
Affected Tests:
Documentation:
Owner:

These fields MUST be resolved before the deprecation is considered complete.

---

10. Feature Identity

Every deprecated feature MUST have a stable identity independent of its spelling.

The identity SHOULD be based on:

domain
namespace
feature-name
semantic-role

rather than solely on the textual spelling.

This is important because:

old_name

may later become:

new_name

while representing the same semantic feature.

A spelling change is therefore not necessarily a semantic feature replacement.

---

11. Deprecation Status Model

Zamani defines the following normative states.

11.1 "PROPOSED"

The feature is under design.

It is not a compatibility promise.

11.2 "EXPERIMENTAL"

The feature exists for experimentation.

Experimental features:

- MAY change;
- MAY be removed;
- SHOULD carry explicit experimental status;
- MUST NOT be presented as stable language guarantees.

11.3 "STABLE"

The feature is part of the supported language contract.

Stable features receive normal compatibility guarantees.

11.4 "DEPRECATED"

The feature remains supported but new source should normally avoid it.

11.5 "MIGRATION_AVAILABLE"

A supported replacement and migration path exist.

This status SHOULD normally precede removal.

11.6 "REMOVAL_ELIGIBLE"

The feature has satisfied all removal prerequisites.

It is still not necessarily removed.

11.7 "REMOVED"

The feature is no longer part of the current language version.

Historical migration documentation MUST remain available.

---

12. Experimental Features and Deprecation

Experimental features are different from deprecated stable features.

An experimental feature:

EXPERIMENTAL → REMOVED

may be valid without a long deprecation period.

A stable feature:

STABLE → DEPRECATED → MIGRATION_AVAILABLE → REMOVAL_ELIGIBLE → REMOVED

requires stronger compatibility guarantees.

The compiler and documentation MUST distinguish these cases.

---

13. Deprecation Reasons

A deprecation MUST have a documented reason.

Allowed reasons include:

13.1 Redundant syntax

Two constructs express the same stable semantics and one is unnecessarily duplicative.

13.2 Ambiguous syntax

A construct creates persistent lexical or grammatical ambiguity.

13.3 Unsound semantic model

The construct cannot express its intended semantics correctly.

13.4 Security weakness

The construct creates a security problem that cannot reasonably be corrected without replacing its interface.

13.5 Semantic inconsistency

The construct conflicts with the canonical language semantic model.

13.6 Maintainability

A construct creates unnecessary duplicate compiler architecture.

This reason alone SHOULD NOT be sufficient to remove stable user-facing semantics.

13.7 Superseded representation

A more general representation provides the same semantics with better extensibility.

13.8 Interoperability evolution

An external standard or interface has changed.

13.9 Namespace or terminology correction

A name is misleading or conflicts with the language's canonical terminology.

13.10 Specification correction

A previous feature definition was incorrect and must be replaced.

---

14. Invalid Deprecation Reasons

The following MUST NOT be used as sole justification:

- "the implementation is hard";
- "the backend does not support it";
- "the hardware is too small";
- "we currently only have one device";
- "the compiler currently has a fixed limit";
- "the parser implementation is inconvenient";
- "a vendor does not use it";
- "another language does not have it".

The language must not shrink merely to match a temporary implementation limitation.

---

15. Deprecation and Grammar Authority

The grammar architecture is:

Canonical Language Specification
            ↓
Canonical Grammar
            ↓
Lexer / Parser
            ↓
AST
            ↓
Semantic Analysis
            ↓
Canonical IR

A deprecated construct may remain in the canonical grammar while it remains source-compatible.

Its status MUST be represented through the compatibility/specification layer.

The presence of a grammar rule alone MUST NOT imply that the feature is:

- stable;
- recommended;
- semantically implemented;
- backend-supported.

---

16. Deprecated Syntax Must Remain Deterministic

A deprecated construct MUST parse deterministically.

Deprecation MUST NOT introduce:

- ambiguous parsing;
- silent fallback;
- heuristic interpretation;
- target-dependent parsing;
- random interpretation;
- compiler-version-dependent reinterpretation.

A deprecated construct must either:

1. parse according to its specified historical semantics; or
2. produce a deterministic diagnostic.

---

17. Deprecated Syntax Must Not Be Silently Reinterpreted

The compiler MUST NOT silently reinterpret:

deprecated old construct

as a different semantic construct merely because the new construct has a similar spelling.

If the meanings differ, the compiler MUST either:

- preserve old semantics;
- require an explicit migration;
- or reject the construct according to the version policy.

---

18. Warning Requirements

Use of a deprecated feature SHOULD produce a structured deprecation diagnostic when compiling a version in which the feature remains supported.

The diagnostic SHOULD contain:

feature
deprecated-since
reason
replacement
migration guidance
removal information

Example conceptual diagnostic:

warning: deprecated feature `legacy.form`

deprecated since: 1.4.0
replacement: `modern.form`
migration: automatic
earliest removal: 2.0.0

The diagnostic MUST NOT falsely claim that the feature is already removed.

---

19. Diagnostic Stability

Deprecation diagnostics MUST be machine-readable where compiler tooling supports structured diagnostics.

A diagnostic SHOULD expose stable fields such as:

code
severity
feature-id
deprecated-since
replacement
migration-kind
source-span

Human-readable text MAY evolve without changing the semantic diagnostic identity.

---

20. Deprecation Severity

The preferred severity levels are:

INFO
WARNING
ERROR

INFO

Used when the tool is explicitly inspecting compatibility status without requiring an ordinary compilation warning.

WARNING

Used when deprecated but still-supported source is compiled.

ERROR

Used after the feature has been removed or when use violates an explicitly declared compatibility mode.

A compiler MUST NOT report a removed feature merely as a warning if the current language version no longer accepts it.

---

21. Warning Suppression

If Zamani supports warning suppression, suppression MUST NOT:

- restore removed syntax;
- alter semantic meaning;
- bypass mandatory compatibility checks;
- bypass security checks;
- silently downgrade errors to successful compilation.

Suppression affects diagnostics, not language semantics.

Any suppression mechanism MUST itself have a stable specification and compatibility policy.

---

22. Replacement Requirements

A stable feature SHOULD NOT be deprecated without identifying a replacement unless:

- the feature is inherently obsolete;
- there is no meaningful replacement;
- retaining it creates a documented correctness/security problem;
- the feature was never stable.

A replacement MUST be semantically described.

A replacement is not sufficient merely because:

old syntax → new syntax

is mechanically possible.

The language must establish:

old meaning
    ↓
new meaning

with sufficient precision.

---

23. Replacement Must Be More General or Correct

Where a feature is deprecated because it is too narrow, its replacement SHOULD be more general.

For example:

fixed hardware declaration

should preferably migrate toward:

capability
requirement
constraint
target description

rather than another fixed hardware declaration.

This preserves POCO-REAF.

---

24. Automatic Migration

A deprecated feature SHOULD have an automatic migration when the transformation is deterministic.

Example:

old_keyword X

to:

new_keyword X

is suitable when semantics are identical.

Automatic migration MUST NOT guess.

If multiple replacements are possible:

old_feature
    ↓
new_feature_a
or
new_feature_b

the migration MUST become assisted or manual.

---

25. Migration Integration

All deprecation migrations MUST integrate with:

"grammar/compatibility/migrations.md"

The migration file owns:

- transformation mechanics;
- migration ordering;
- source rewriting;
- AST migration;
- semantic validation;
- migration testing.

This file owns:

- why the feature is deprecated;
- its lifecycle;
- its replacement;
- removal eligibility.

No duplicate migration algorithm should be defined here.

---

26. Migration Record

Each deprecated feature with a migration path SHOULD provide a migration record conceptually equivalent to:

source-version
target-version
feature-id
old-form
new-form
migration-class
semantic-equivalence
manual-review-required
diagnostic-code
test-suite

The concrete representation belongs to the migration infrastructure.

---

27. Migration Must Be Idempotent

Where a migration is automatic:

migrate(migrate(source))

MUST produce the same semantic result as:

migrate(source)

The migration tool MUST NOT repeatedly transform already-migrated source.

---

28. Migration Must Be Deterministic

Given:

same source
same source version
same target version
same migration configuration

the migration result MUST be deterministic.

It MUST NOT depend on:

- machine size;
- CPU count;
- available GPUs;
- available QPUs;
- network topology;
- scheduler state;
- runtime timing;
- random choices.

---

29. Lossless Migration

A migration SHOULD be lossless.

Where possible, migration MUST preserve:

- comments;
- source spans;
- annotations;
- metadata;
- attributes;
- documentation;
- identifiers;
- semantic information.

If information cannot be preserved, the migration MUST explicitly report the loss.

Silent information loss is prohibited.

---

30. Semantic Validation

After migration:

old source
    ↓
old semantics

and:

migrated source
    ↓
new semantics

MUST be compared wherever semantic equivalence is promised.

Validation MUST occur before the migration is declared successful.

---

31. AST Integration

A deprecated grammar construct MUST map to an AST representation capable of preserving its historical semantics.

Preferred architecture:

deprecated source
       ↓
lexer
       ↓
parser
       ↓
normal AST
       ↓
semantic normalization
       ↓
current semantic model

The AST SHOULD NOT accumulate permanent duplicate node families solely for historical syntax.

If two syntactic forms have identical semantics, they SHOULD normalize to the same semantic representation.

---

32. AST Compatibility

An AST migration MUST preserve:

- source meaning;
- names;
- scopes;
- types;
- effects;
- capabilities;
- resource intent;
- control flow;
- concurrency semantics;
- quantum semantics;
- hardware intent.

AST compatibility MUST NOT require target-specific data that did not exist in the source semantics.

---

33. Canonical IR Integration

Deprecated syntax MUST ultimately lower into the canonical semantic representation.

The grammar compatibility layer MUST NOT introduce a permanent parallel IR merely to retain historical syntax.

The preferred flow is:

Deprecated Source
       ↓
Parser
       ↓
AST
       ↓
Semantic Normalization
       ↓
Canonical IR

For quantum:

Deprecated Quantum Syntax
       ↓
AST
       ↓
Quantum Semantic Normalization
       ↓
quantum::ir
       ↓
QEC / Optimization / Routing / Scheduling
       ↓
HAL / Target

"quantum::ir" remains the canonical quantum semantic boundary.

---

34. Quantum Deprecation

Quantum constructs MUST be deprecated according to semantic meaning, not hardware age.

A quantum construct MUST NOT be deprecated merely because:

- a QPU does not support it;
- a gate is not native;
- a topology is inconvenient;
- a device has insufficient qubits;
- a backend has not implemented a lowering;
- a simulator performs it differently.

Those are backend or capability issues.

---

35. Quantum Operation Evolution

Where possible, a deprecated quantum operation spelling SHOULD migrate to a generic operation model.

Conceptually:

legacy_operation
       ↓
canonical operation
       ↓
quantum::ir

The grammar MUST NOT require a new keyword or grammar production for every future physical gate.

This preserves extensibility.

---

36. Qubit Scalability

Deprecation policy MUST NOT introduce:

MAX_QUBITS
MAX_LOGICAL_QUBITS
MAX_PHYSICAL_QUBITS
MAX_REGISTER_SIZE

or equivalent restrictions.

A deprecated quantum syntax may be replaced with a scalable declaration.

For example:

legacy fixed-register form

may migrate toward a semantic resource expression without imposing a permanent maximum.

---

37. Logical and Physical Quantum Separation

If a deprecated construct conflates logical and physical qubits, migration MUST preserve the distinction.

The source-level semantic model SHOULD distinguish:

logical quantum intent

from:

physical realization

Physical mapping belongs to:

- routing;
- scheduling;
- HAL;
- target description;
- resource management.

It does not belong in a historical syntax compatibility shim.

---

38. QEC Integration

Deprecating quantum error-correction syntax MUST preserve QEC intent.

A migration MUST distinguish:

error-correction requirement

from:

specific implementation

A source program specifying a resilience requirement MUST NOT be silently rewritten to a particular device topology or fixed number of physical qubits.

The QEC subsystem remains responsible for realization.

---

39. ZQN Integration

ZQN-related syntax MUST be deprecated independently from:

- quantum semantic representation;
- QEC;
- routing;
- scheduling;
- hardware abstraction.

A deprecated ZQN syntax MUST lower through the existing canonical semantic boundary rather than introducing a second quantum fault representation.

---

40. Classical Deprecation

Classical constructs MUST preserve:

- evaluation semantics;
- numeric semantics;
- control flow;
- memory semantics;
- ownership;
- concurrency;
- synchronization;
- error behavior.

Numerical changes require particular care.

A deprecation MUST NOT silently alter:

- overflow behavior;
- rounding;
- precision;
- signedness;
- evaluation order;
- exceptional values.

If those semantics change, the migration MUST be classified as semantic.

---

41. HDL Deprecation

HDL constructs MUST preserve hardware intent where the language promises semantic equivalence.

A deprecated HDL representation MAY be replaced by a more expressive representation.

However, migration MUST preserve, where applicable:

- signal behavior;
- combinational semantics;
- sequential semantics;
- state transitions;
- timing semantics;
- reset behavior;
- clock relationships;
- interface semantics;
- parameterization.

A target-specific synthesis limitation MUST NOT become a language-level deprecation.

---

42. Hardware Deprecation

Hardware syntax must distinguish:

hardware intent

from:

hardware instance

Deprecated constructs should migrate toward:

- capabilities;
- requirements;
- constraints;
- targets;
- placement;
- resources.

A language feature MUST NOT become deprecated simply because a newer generation of hardware exists.

---

43. Resource Deprecation

Resource-related constructs MUST preserve the distinction among:

resource
requirement
constraint
preference
hint
capability
availability
allocation
placement

A deprecated resource construct MUST NOT collapse these concepts.

For example:

requires quantum

must not become:

use device "X"

unless device selection was explicitly part of the original semantics.

---

44. Distributed Computing Deprecation

Deprecated distributed constructs MUST preserve, where applicable:

- communication semantics;
- ordering;
- consistency;
- replication intent;
- fault behavior;
- placement intent;
- service identity;
- message semantics.

A migration MUST NOT replace an abstract distributed requirement with a fixed node count.

---

45. AI/ML Deprecation

AI/ML syntax MUST remain independent of:

- fixed tensor dimensions where abstraction is appropriate;
- fixed accelerator counts;
- fixed device types;
- fixed memory capacities.

A deprecated AI construct SHOULD migrate toward semantic model/data/operation representations rather than backend-specific syntax.

---

46. Networking Deprecation

Deprecated networking constructs MUST preserve semantic intent.

A network construct MUST NOT be deprecated solely because:

- a protocol implementation changes;
- a particular network topology changes;
- a device lacks support.

Protocol-specific compatibility belongs to the networking/interoperability layer.

---

47. Security Deprecation

Security-sensitive constructs require special treatment.

A deprecated security feature MUST NOT silently weaken security.

If a replacement provides stronger guarantees, migration SHOULD make the security improvement explicit.

If the old feature is unsafe by specification, continued support MAY be shortened, but the compiler MUST provide a clear diagnostic and migration path where practical.

---

48. Dialect Deprecation

Dialect deprecation MUST be isolated from core-language compatibility.

A dialect MUST have:

- stable identity;
- namespace;
- version;
- owner;
- compatibility status.

Deprecating:

dialect X

MUST NOT automatically deprecate the core Zamani construct that the dialect interoperates with.

---

49. Vendor Extensions

Vendor extensions MAY be deprecated independently.

Vendor-specific syntax MUST NOT become part of the portable core merely because it is widely implemented.

Conversely, a vendor extension MUST NOT constrain the core language's scalability model.

---

50. Reserved Syntax Integration

When a deprecated feature is removed, its former syntax MAY become:

- reserved;
- available for a replacement;
- available to extensions;
- released for reuse only under an explicit compatibility policy.

"grammar/compatibility/reserved.md" owns the reserved-space decision.

This file MUST record the relationship when necessary.

---

51. Removed Syntax Must Not Be Reused Immediately

A removed keyword or syntax form SHOULD NOT immediately be reassigned to unrelated semantics.

Reusing historical syntax can make old source dangerous to interpret.

Before reuse, the compatibility analysis MUST establish that:

- historical source cannot be silently reinterpreted;
- migration boundaries are clear;
- version resolution is deterministic.

---

52. Removal Is Different From Deprecation

Deprecation means:

recognized + supported + discouraged

Removal means:

not part of the current language contract

A compiler supporting the version in which a feature is deprecated MUST continue to implement its specified semantics unless the compatibility policy explicitly says otherwise.

---

53. Removal Prerequisites

A stable feature MUST NOT become removable until all applicable conditions are satisfied:

1. feature identity is documented;
2. deprecation version is documented;
3. reason is documented;
4. replacement is documented;
5. migration path exists where practical;
6. migration tests exist;
7. diagnostics exist;
8. documentation identifies the change;
9. compatibility matrix is updated;
10. affected examples are updated;
11. AST implications are resolved;
12. semantic implications are resolved;
13. IR implications are resolved;
14. tooling implications are resolved;
15. cross-domain implications are resolved;
16. removal version is documented;
17. no hidden stable consumer remains;
18. repository-wide tests pass.

---

54. Removal Version

The earliest removal version MUST be explicit.

Example:

Deprecated since: 1.4.0
Earliest removal: 2.0.0

The actual removal MUST still satisfy the compatibility policy.

A feature MUST NOT be removed earlier than its declared earliest removal version unless an emergency security/correctness policy explicitly permits accelerated removal.

---

55. Security and Correctness Exception

An immediate or accelerated removal MAY occur when retaining a feature would create a severe:

- security vulnerability;
- semantic unsoundness;
- data-integrity problem;
- compiler correctness problem.

In such a case the release documentation MUST state:

- the affected feature;
- the reason;
- affected versions;
- mitigation;
- replacement;
- migration instructions;
- compatibility consequences.

This exception MUST NOT become a general mechanism for avoiding normal compatibility work.

---

56. Deprecation of Broken Features

A feature that was never correctly implemented MUST NOT be presented as a normal stable feature merely because a grammar rule exists.

The feature status should instead reflect its actual maturity:

Specified
Parsed
AST-supported
Semantically-supported
IR-supported
Backend-supported
Runtime-supported
Stable

If a grammar-only feature is removed because it was never part of the implemented language contract, documentation MUST make that distinction explicit.

---

57. Grammar-Only Deprecation

If a construct exists only in documentation or an obsolete grammar but has no valid implementation contract, it MUST NOT automatically receive a stable deprecation promise.

The authority audit MUST determine:

- whether it was normative;
- whether users could compile it;
- whether compatibility was promised;
- whether migration is required.

This prevents obsolete documentation from creating accidental language guarantees.

---

58. Existing Feature Preservation

Before deprecating any existing feature:

1. identify every occurrence;
2. identify all grammar rules;
3. identify lexer tokens;
4. identify parser rules;
5. identify AST nodes;
6. identify semantic consumers;
7. identify IR consumers;
8. identify compiler consumers;
9. identify runtime consumers;
10. identify tooling consumers;
11. identify tests;
12. identify examples;
13. identify documentation;
14. identify cross-domain consumers.

Only then may deprecation be approved.

---

59. Repository-Wide Consumer Audit

A deprecation audit MUST search at minimum:

grammar/
src/lexer.rs
src/parser.rs
src/frontend/
src/ast/
src/semantic/
src/ir/
src/quantum/
src/classical/
src/compiler/
src/runtime/
src/hardware/
src/hdl/
tests/
examples/
docs/

The exact directories MUST follow the actual repository structure.

A feature MUST NOT be considered removable merely because it disappeared from the grammar.

---

60. Integration With "grammar/Zamani.g4"

"grammar/Zamani.g4" is the canonical ANTLR grammar representation.

For every deprecated syntax still supported:

- its lexical representation MUST remain defined;
- its parser representation MUST remain deterministic;
- its semantic status MUST be documented;
- its deprecation diagnostic MUST be available through the compiler/tooling layer where supported.

The grammar itself MUST NOT become the repository's only source of deprecation metadata.

Deprecation metadata belongs in the compatibility specification and implementation metadata.

---

61. Integration With Modular Grammar Files

If the grammar is split into:

lexer/
core/
types/
expressions/
statements/
...

deprecation status MUST remain globally consistent.

A construct MUST NOT be:

deprecated

in one grammar component and:

stable

in another.

The feature identity, not the grammar-file location, determines status.

---

62. Integration With "grammar/grammar.md"

"grammar/grammar.md" MUST accurately describe the currently accepted and supported language.

Deprecated constructs MAY be documented there when they remain accepted, but their status MUST be explicit.

The document MUST NOT imply that deprecated constructs are recommended new syntax.

---

63. Integration With "grammar/Zamani-Grammar.md"

"Zamani-Grammar.md" may contain broader language design information.

It MUST distinguish:

- stable;
- experimental;
- deprecated;
- proposed;
- future.

A proposed or future construct MUST NOT be presented as a stable compatibility guarantee.

---

64. Integration With "specification/language-version.md"

The language-version specification determines the version in which deprecation becomes effective.

This file determines:

- what deprecation means;
- how it progresses;
- what metadata is required;
- what removal requires.

---

65. Integration With "specification/compatibility.md"

Compatibility policy determines whether a deprecation is:

- source-compatible;
- syntax-compatible;
- AST-compatible;
- semantic-compatible;
- IR-compatible;
- target-compatible;
- runtime-compatible.

Deprecation MUST NOT collapse these dimensions.

---

66. Integration With "compatibility/versions.md"

"versions.md" determines version semantics.

For every deprecation:

deprecated-since
earliest-removal-version

MUST refer to the language-version model defined there.

Rust version MUST NOT be used as the Zamani feature version.

---

67. Rust Version Independence

The Zamani language version MUST remain independent of:

Rust 1.97
Rust 1.97.1

Rust is an implementation toolchain.

It is not the Zamani language version.

The implementation baseline is:

minimum supported Rust: 1.97
production validation: Rust 1.97.1

The implementation MUST use safe Rust only.

No deprecation mechanism may require Rust "unsafe".

---

68. Integration With AST

The AST implementation under "src/frontend/ast/" MUST remain domain-neutral.

Deprecation handling MUST NOT turn the AST into a target-specific representation.

In particular, deprecated syntax MUST NOT force AST nodes to contain:

- LLVM-specific structures;
- vendor-specific backend structures;
- physical topology;
- calibration state;
- QEC implementation details;
- routing decisions;
- scheduler decisions.

Those belong downstream.

---

69. Integration With Semantic Analysis

Semantic analysis determines whether a deprecated construct remains valid under the effective language version.

The semantic layer SHOULD normalize equivalent historical syntax before domain-specific lowering.

Preferred model:

historical syntax
       ↓
AST
       ↓
normalization
       ↓
current semantic representation

---

70. Integration With Classical IR

Deprecated classical syntax MUST lower into the existing canonical classical semantic/IR pathway.

No permanent "legacy classical IR" should be introduced solely to preserve deprecated syntax.

---

71. Integration With "quantum::ir"

"quantum::ir" remains the canonical quantum semantic boundary.

Deprecated quantum syntax MUST be normalized before or during semantic lowering into that canonical boundary.

The deprecation layer MUST NOT:

- duplicate quantum IR;
- define competing Qubit IDs;
- create alternate physical-qubit identity systems;
- encode target topology;
- encode QEC implementation;
- encode routing decisions.

---

72. Qubit Identity Compatibility

Where quantum source explicitly refers to semantic qubits, compatibility must preserve canonical identity.

The repository's canonical:

quantum::ir::qubit::QubitId
quantum::ir::qubit::PhysicalQubitId

must remain authoritative where those identities are applicable.

A deprecated syntax MUST NOT introduce competing identifiers.

---

73. QEC Integration

Deprecated QEC syntax MUST preserve QEC intent.

The migration MUST distinguish:

desired error-correction behavior

from:

specific decoder
specific code implementation
specific hardware mapping

unless the source explicitly specifies those implementation details.

---

74. Scheduling Integration

Deprecation MUST NOT encode scheduler policy into source compatibility.

Scheduling remains responsible for:

- ordering;
- timing;
- resource availability;
- dependencies;
- hardware realization.

A deprecated scheduling hint may be replaced by a portable semantic hint or requirement.

---

75. Routing Integration

Routing decisions are not language-version semantics unless explicitly defined as source semantics.

A deprecated syntax MUST NOT permanently bind a program to:

- a topology;
- physical qubit locations;
- a fixed path;
- a fixed device.

---

76. Hardware Abstraction Integration

Hardware abstraction remains responsible for target realization.

A deprecation replacement MUST prefer:

capability
requirement
constraint
target
resource

over:

fixed machine

where portability is intended.

---

77. Runtime Integration

Runtime behavior MUST NOT silently change because a source construct became deprecated.

If runtime behavior changes, that change requires its own compatibility classification.

The deprecation system may communicate the lifecycle of source syntax, but it does not own runtime semantics.

---

78. Tooling Integration

Tooling SHOULD provide:

- deprecation diagnostics;
- migration suggestions;
- feature status;
- replacement information;
- machine-readable metadata;
- source transformation where safe;
- compatibility checks.

Tooling MUST use the same feature identity as the language specification.

---

79. Documentation Integration

Every deprecated feature MUST have documentation in:

- compatibility documentation;
- relevant feature documentation;
- migration documentation where migration exists;
- release notes when the deprecation is introduced;
- removal documentation when removed.

Documentation MUST NOT silently delete the historical explanation immediately after removal.

Historical compatibility information is required for long-lived source maintenance.

---

80. Test Integration

Every deprecated feature MUST have tests covering:

Positive

The old syntax remains accepted while supported.

Diagnostic

The deprecation warning is emitted correctly.

Migration

The replacement is correct.

Semantic equivalence

Old and migrated source have equivalent meaning where promised.

Removal

The feature is rejected in the removal version.

Regression

The replacement remains supported.

---

81. Negative Tests

Negative tests MUST cover:

- malformed deprecated syntax;
- invalid version declarations;
- unsupported future versions;
- removed constructs;
- invalid migration targets;
- ambiguous replacements;
- unsupported combinations.

A compiler MUST fail deterministically.

---

82. Boundary Tests

Boundary tests MUST include:

- smallest valid program using the deprecated construct;
- nested use;
- repeated use;
- large source files;
- deeply nested syntax where supported;
- large collections of deprecated constructs;
- many modules;
- many quantum operations;
- large resource expressions.

No test may use an artificial fixed maximum merely because it is convenient.

---

83. Cross-Domain Deprecation Tests

At minimum, compatibility testing SHOULD cover:

classical + quantum
classical + HDL
quantum + HDL
quantum + hardware
quantum + distributed
AI + quantum
AI + hardware
classical + quantum + distributed
classical + quantum + HDL + hardware

Deprecated constructs must preserve the intended cross-domain semantics.

---

84. Scalability Tests

Deprecation tests MUST verify that migration semantics are independent of resource scale.

The same source semantics must remain valid across:

tiny target
small target
medium target
large target
distributed target
heterogeneous target
future target

subject only to actual capability/resource availability.

---

85. Determinism Tests

For identical:

source
language version
compiler compatibility configuration
migration configuration

the deprecation result MUST be deterministic.

No hidden random state may influence:

- warning classification;
- migration choice;
- feature status;
- semantic interpretation.

---

86. Round-Trip Tests

Where a source printer/serializer exists:

old source
   ↓
lexer
   ↓
parser
   ↓
AST
   ↓
migration/normalization
   ↓
printer
   ↓
parser

must preserve intended semantics.

---

87. Deprecation Must Not Break Reproducibility

A program compiled with a fixed:

language version
compiler version
dependency set
dialect versions
migration configuration

must produce deterministic compatibility behavior.

Deprecation decisions MUST NOT depend on:

- current hardware;
- current machine size;
- current QPU availability;
- network availability;
- wall-clock time;
- undocumented environment variables.

---

88. Deprecation and Dependencies

A dependency may deprecate an API without automatically deprecating the Zamani language feature that uses it.

The compiler MUST distinguish:

language deprecation

from:

dependency deprecation

Migration tooling SHOULD identify dependency-level changes separately.

---

89. Deprecation and Dialect Versioning

A dialect may have its own lifecycle.

For example:

core Zamani: stable
dialect X: deprecated
dialect Y: stable

This is valid.

Deprecating a dialect MUST NOT require a core-language major version unless the dialect is part of the core language contract.

---

90. Compatibility Modes

A compiler MAY support compatibility modes such as:

strict
default
legacy
migration

if the implementation needs them.

Such modes MUST NOT change the underlying language semantics silently.

A compatibility mode MUST have a documented purpose and deterministic behavior.

---

91. Legacy Mode

A legacy mode MAY allow additional historical syntax.

If supported:

- it MUST be explicitly selected;
- it MUST report the effective language version;
- it MUST not silently reinterpret current syntax;
- it MUST remain deterministic;
- it MUST have a documented removal policy.

Legacy mode MUST NOT become a permanent excuse for avoiding migration.

---

92. Forward Compatibility

A compiler MUST NOT assume that unknown future constructs are deprecated constructs.

Unknown future syntax MUST either:

- be handled by an explicitly defined extension mechanism; or
- produce an unsupported-version/unknown-feature diagnostic.

It MUST NOT be silently treated as an old construct.

---

93. Extension Namespaces

Future and experimental functionality SHOULD use explicit namespaces or extension identities where appropriate.

This reduces collisions between:

stable core
experimental features
vendor extensions
future language features

Deprecation MUST preserve those namespace boundaries.

---

94. Feature Flags

If feature flags exist, a deprecated feature's status MUST be independent from whether the feature happens to be enabled in one compiler build.

A compiler build configuration MUST NOT redefine the language specification.

Feature flags may control implementation availability, but the compatibility model remains authoritative.

---

95. Deprecating a Keyword

A keyword deprecation requires analysis of:

- lexer;
- parser;
- identifier collisions;
- contextual keyword behavior;
- dialects;
- macros;
- tooling;
- migration.

If possible:

keyword

should transition through contextual handling before becoming unavailable.

---

96. Deprecating an Operator

Operator deprecation requires:

- lexical analysis;
- precedence analysis;
- associativity analysis;
- parser analysis;
- semantic analysis;
- migration examples.

If the replacement changes precedence, the migration MUST insert explicit grouping where required.

Example conceptually:

old: a op b op c

must not become an ambiguous:

new: a new_op b new_op c

without preserving the original evaluation structure.

---

97. Deprecating a Type

Type deprecation requires:

- type identity;
- conversion semantics;
- generic behavior;
- ownership;
- ABI implications;
- serialization implications;
- numerical semantics.

A type alias may be deprecated mechanically when the semantic type is unchanged.

A representation change requires semantic analysis.

---

98. Deprecating a Module or Namespace

A deprecated module path MUST identify:

old path
new path
visibility changes
name conflicts
dependency effects
migration

Import migration MUST preserve symbol resolution.

---

99. Deprecating a Declaration

Declaration migration MUST preserve:

- scope;
- visibility;
- binding;
- initialization;
- lifetime;
- type;
- attributes;
- effects.

A declaration syntax change MUST NOT accidentally change initialization or evaluation order.

---

100. Deprecating Concurrency Syntax

Concurrency deprecation requires special care.

The migration MUST preserve:

- task semantics;
- synchronization;
- ordering;
- cancellation;
- error propagation;
- ownership;
- shared-state semantics.

A deprecated concurrency construct MUST NOT be replaced with a construct that assumes a fixed thread count.

---

101. Deprecating Memory Syntax

Memory migration MUST preserve:

- ownership;
- borrowing;
- lifetime;
- allocation semantics;
- deallocation semantics;
- aliasing guarantees;
- shared-memory semantics;
- distributed-memory semantics.

A target's memory capacity is not a language compatibility property.

---

102. Deprecating Compile-Time Features

Compile-time constructs require distinction between:

compile-time semantics

and:

compiler implementation

A feature MUST NOT be deprecated merely because one compiler implementation cannot evaluate it efficiently.

---

103. Deprecating Runtime Features

Runtime behavior belongs to runtime compatibility.

A source construct may remain stable even if a runtime API changes.

Where a runtime change requires source changes, the source-facing migration MUST be explicitly documented.

---

104. Deprecation and ABI

ABI compatibility is distinct from source compatibility.

A source feature may be deprecated while its ABI remains supported.

Conversely, an ABI may change while source syntax remains stable.

These cases MUST be tracked independently.

---

105. Deprecation and Serialization

Serialization formats MUST have independent lifecycle metadata.

A deprecated source feature MUST NOT automatically make all serialized artifacts invalid.

If serialization changes:

source compatibility

and:

artifact compatibility

must be tracked separately.

---

106. Deprecation and Determinism

A deprecation MUST NOT alter deterministic semantics unless the specification explicitly defines the change.

If a deprecated construct had deterministic behavior, migration must preserve that behavior where compatibility is promised.

---

107. Deprecation and Numerical Semantics

Numerical features require special protection.

Deprecation MUST NOT silently change:

- precision;
- rounding;
- overflow;
- underflow;
- signedness;
- NaN behavior;
- infinity behavior;
- evaluation order.

If such behavior changes, the migration must be classified accordingly.

---

108. Deprecation and Security

Security-sensitive deprecated constructs require explicit security classification.

The compiler SHOULD identify:

security impact
replacement security properties
migration risk

A migration MUST NOT weaken a security guarantee without explicit source-level acknowledgement where required by the language specification.

---

109. Deprecation and Provenance

Migration tooling SHOULD record provenance:

original language version
migration performed
migration tool version
migration rule
target language version

This enables reproducibility and debugging.

Provenance MUST NOT contain secrets.

---

110. Deprecation and Source Spans

Where possible, deprecation diagnostics SHOULD preserve accurate source spans.

A migration should preserve enough source-location information to allow developers to locate the historical construct.

---

111. Deprecation and Comments

Comments SHOULD be preserved during automatic migration when the tooling promises source-preserving migration.

A migration MUST NOT silently delete documentation attached to a deprecated declaration.

---

112. Deprecation and Formatting

Formatting may change during migration if the migration tool explicitly operates as a formatter.

However:

semantic migration

and:

formatting

must remain conceptually separate.

---

113. Deprecation Registry

The implementation SHOULD maintain a machine-readable internal registry derived from this normative policy.

Each record SHOULD contain:

feature_id
status
introduced_version
deprecated_version
removal_version
replacement
migration_kind
diagnostic_code
semantic_contract

The registry MUST have one authoritative representation.

Generated documentation MUST NOT become an independent authority.

---

114. Generated Deprecation Documentation

If deprecation metadata is generated:

canonical metadata
       ↓
generated documentation

must be the direction of derivation.

The reverse:

documentation
       ↓
implementation semantics

must not occur.

---

115. Grammar Generation

If modular grammar files generate "Zamani.g4", the dependency direction must remain:

canonical grammar components
        ↓
generated/composed grammar
        ↓
ANTLR
        ↓
parser

Deprecation metadata must not introduce:

parser
  ↓
grammar
  ↓
deprecated registry

cycles.

---

116. No Circular Dependency

The architecture MUST NOT contain:

deprecated.md
    ↓
parser
    ↓
deprecated.md

or:

grammar
    ↓
runtime
    ↓
grammar

or:

quantum grammar
    ↓
hardware grammar
    ↓
quantum grammar

Deprecation is a compatibility specification layer consumed by validation/tooling.

---

117. File Contract

This file is independently complete under the repository's file-completion standard.

File

"grammar/compatibility/deprecated.md"

Purpose

Define the normative lifecycle and compatibility policy for deprecated Zamani language features.

Owns

- deprecation lifecycle;
- deprecation metadata;
- deprecation eligibility;
- deprecation warnings;
- removal eligibility;
- replacement requirements;
- deprecation-specific compatibility guarantees.

Does Not Own

- version numbering;
- migration algorithms;
- canonical grammar semantics;
- AST implementation;
- IR definitions;
- runtime behavior;
- target selection;
- hardware capabilities.

Inputs

- language-version policy;
- compatibility policy;
- canonical grammar;
- semantic specifications;
- existing feature inventory;
- migration specifications;
- repository implementation state.

Outputs

- deprecation status;
- feature lifecycle;
- replacement requirements;
- removal requirements;
- diagnostic requirements;
- compatibility obligations.

Dependencies

Normative dependencies:

specification/language-version.md
specification/compatibility.md
compatibility/versions.md
compatibility/migrations.md
compatibility/reserved.md
compatibility/compatibility-matrix.md

Implementation dependencies:

lexer
parser
AST
semantic analysis
IR
compiler
tooling
tests

Upstream Contracts

Must consume:

- canonical language version model;
- canonical compatibility model;
- canonical grammar authority;
- semantic contracts.

Downstream Consumers

Expected consumers:

- lexer/parser compatibility handling;
- semantic validation;
- compiler diagnostics;
- migration tooling;
- IDE/tooling;
- compatibility tests;
- documentation generation.

Public Grammar Contract

This file does not directly define ordinary source syntax.

It defines the lifecycle of syntax and language features that the canonical grammar recognizes.

AST Contract

Deprecated syntax must remain representable until removal where required for source compatibility.

Equivalent historical syntax SHOULD normalize into current semantic representations.

Semantic Contract

Deprecation MUST NOT silently change semantics.

Semantic changes require explicit migration classification.

IR Integration

Deprecated syntax must lower into existing canonical IR.

No legacy permanent IR may be introduced solely for deprecation support.

Quantum constructs continue to lower through "quantum::ir".

Compiler Integration

The compiler must:

- resolve language version;
- recognize deprecated constructs;
- emit structured diagnostics;
- enforce removal versions;
- invoke migration tooling where supported.

Runtime Integration

Runtime behavior is not owned here.

Source deprecation MUST NOT silently alter runtime semantics.

Tooling Integration

Tooling SHOULD expose:

- status;
- replacement;
- migration;
- removal version;
- machine-readable diagnostic identity.

Cross-Domain Integration

The policy applies consistently to:

- classical;
- quantum;
- hybrid;
- HDL;
- hardware;
- distributed;
- AI/ML;
- data;
- networking;
- security;
- accelerators;
- embedded;
- future domains.

Tests

Required:

- status tests;
- warning tests;
- migration tests;
- semantic-equivalence tests;
- removal tests;
- version tests;
- cross-domain tests;
- determinism tests;
- scalability tests;
- round-trip tests.

Negative Tests

Required:

- removed feature;
- unknown feature;
- invalid version;
- invalid replacement;
- invalid migration;
- unsupported future feature;
- ambiguous historical syntax.

Boundary Tests

Required:

- smallest source;
- large source;
- repeated deprecated constructs;
- deeply nested constructs where supported;
- large quantum programs;
- large resource expressions;
- many modules;
- many operations.

No artificial machine-scale maximum may be introduced.

Compatibility Requirements

Must preserve the compatibility guarantees defined by "versions.md" and "specification/compatibility.md".

Scalability Requirements

Deprecation behavior MUST NOT depend on:

- machine size;
- number of CPUs;
- number of cores;
- number of threads;
- number of GPUs;
- number of FPGAs;
- number of QPUs;
- qubit count;
- memory size;
- network size;
- cluster size;
- topology.

Hard-Coding Audit

The implementation MUST reject or remove accidental constants representing:

- hardware limits;
- resource limits;
- topology;
- device counts;
- qubit counts;
- fixed accelerator counts;
- fixed memory;
- fixed deployment size.

Feature lifecycle versions are NOT considered hardware hard-coding.

Completion Criteria

This file is complete when:

1. every deprecation has a stable identity;
2. lifecycle states are defined;
3. ownership boundaries are defined;
4. migration integration is defined;
5. version integration is defined;
6. reserved-space integration is defined;
7. grammar integration is defined;
8. AST integration is defined;
9. semantic integration is defined;
10. IR integration is defined;
11. quantum integration is defined;
12. hardware integration is defined;
13. diagnostics are defined;
14. removal policy is defined;
15. test obligations are defined;
16. scalability obligations are defined;
17. no circular dependency exists;
18. no accidental hardware limitation is introduced.

---

118. Required Implementation Order

The deprecation policy should be implemented in dependency order.

Recommended sequence:

1. specification/language-version.md
2. specification/compatibility.md
3. compatibility/versions.md
4. compatibility/deprecated.md
5. compatibility/reserved.md
6. compatibility/migrations.md
7. compatibility/compatibility-matrix.md
8. validation/compatibility-rules.md
9. lexer compatibility implementation
10. parser compatibility implementation
11. AST compatibility
12. semantic compatibility
13. IR compatibility
14. compiler diagnostics
15. migration tooling
16. compatibility tests
17. cross-domain tests
18. scalability tests

"deprecated.md" must be completed before implementation of deprecation diagnostics because those diagnostics depend on its lifecycle contract.

---

119. Required Repository-Wide Integration Checklist

Before declaring deprecation infrastructure production-ready, verify:

Grammar

- [ ] "Zamani.g4" agrees with the specification.
- [ ] Modular grammar components agree with "Zamani.g4".
- [ ] Deprecated syntax is deterministic.
- [ ] Removed syntax is rejected at the correct version.

Lexer

- [ ] Keyword transitions are tested.
- [ ] Identifier collisions are tested.
- [ ] Literal changes are tested.
- [ ] Operator changes are tested.

Parser

- [ ] Deprecated constructs parse correctly.
- [ ] Removed constructs fail correctly.
- [ ] No ambiguity is introduced.

AST

- [ ] Historical semantics are representable.
- [ ] Equivalent syntax normalizes correctly.
- [ ] No target-specific information leaks into AST.

Semantic Analysis

- [ ] Deprecated constructs retain historical semantics.
- [ ] Semantic changes are explicit.
- [ ] Resource semantics remain scalable.

Quantum

- [ ] Quantum deprecations preserve semantic intent.
- [ ] "quantum::ir" remains canonical.
- [ ] No competing Qubit IDs are introduced.
- [ ] No fixed qubit limits are introduced.
- [ ] QEC intent remains intact.
- [ ] ZQN integration remains intact.

Hardware

- [ ] No device is silently selected.
- [ ] No topology is hard-coded.
- [ ] Capability and requirement remain separate.
- [ ] Target realization remains downstream.

Compiler

- [ ] Diagnostics are deterministic.
- [ ] Version resolution is deterministic.
- [ ] Removed features are rejected correctly.

Tooling

- [ ] Migration suggestions are available where possible.
- [ ] Machine-readable deprecation metadata exists.
- [ ] Provenance is available where required.

Testing

- [ ] Positive tests exist.
- [ ] Negative tests exist.
- [ ] Boundary tests exist.
- [ ] Cross-domain tests exist.
- [ ] Determinism tests exist.
- [ ] Round-trip tests exist.
- [ ] Scalability tests exist.

Rust

- [ ] Rust 1.97 is supported.
- [ ] Rust 1.97.1 is production-validated.
- [ ] No "unsafe" Rust is required.
- [ ] Zamani language version remains independent of Rust version.

---

120. Final Architectural Rule

The definitive rule for Zamani deprecation is:

«A feature may be deprecated because its language representation or semantics genuinely need to evolve; it must never be deprecated merely because the currently available machine, backend, scheduler, QPU, compiler implementation, or resource pool is limited.»

The intended evolution is:

Historical Source
       │
       ▼
Deprecated Syntax
       │
       ▼
Deterministic Migration
       │
       ▼
Current Semantic Model
       │
       ▼
Canonical IR
       │
       ├───────────────┐
       ▼               ▼
Classical IR       quantum::ir
       │               │
       └───────┬───────┘
               ▼
      Optimization / QEC
               ▼
       Routing / Scheduling
               ▼
      Hardware Abstraction
               ▼
       Resource Resolution
               ▼
          Runtime

The source program remains a description of computation and intent.

The machine remains a description of available realization.

Therefore:

one program
     ↓
one semantic meaning
     ↓
many compiler targets
     ↓
many architectures
     ↓
many hardware configurations
     ↓
many scales
     ↓
many execution environments
     ↓
future platforms

while preserving:

Semantic Stability
Hardware Independence
Quantum/Classical Interoperability
HDL Capability
Extensibility
Deterministic Parsing
Strong Diagnostics
Version Compatibility
Canonical IR Boundaries
Repository-Wide Integration
No Accidental Hard-Coded Limits
Safe Rust
POCO-REAF

Zamani deprecation is therefore a controlled language-evolution mechanism, not a mechanism for shrinking the language to fit today's hardware.