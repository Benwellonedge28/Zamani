# Zamani Deprecation Policy
 
**Path:** `grammar/compatibility/deprecated.md`
**Status:** Normative
**Language:** Zamani
**Repository:** `Benwellonedge28/Zamani`
**Rust baseline:** Rust 1.97 / Rust 1.97.1
**Rust edition:** 2021
**Safety requirement:** Production Zamani Rust implementation MUST use safe Rust. Rust `unsafe` MUST NOT be required or used.
**Primary objective:** Preserve semantic stability, portability, scalability, extensibility, and POCO-REAF while allowing controlled language evolution.
 
**POCO-REAF:**
 
> **Program Once, Compile Once, Run Everywhere, Anywhere, Forever**

 
***
 
## 1. Purpose
 
This document defines the normative deprecation policy for Zamani.
 
It establishes:
 
- what may be deprecated;
- what must not be deprecated merely because of implementation limitations;
- how a feature becomes deprecated;
- how deprecation is represented;
- how deprecated syntax remains compatible;
- how deprecation interacts with language versions;
- how deprecation interacts with `Zamani.g4`;
- how deprecation interacts with the lexer and parser;
- how deprecation interacts with the domain-neutral AST;
- how deprecation interacts with semantic analysis;
- how deprecation interacts with canonical IR;
- how quantum syntax reaches `quantum::ir`;
- how classical, HDL, hardware, distributed, AI, networking, security, and future domains evolve;
- how migrations are connected to `compatibility/migrations.md`;
- how removal is approved;
- how diagnostics are emitted;
- how deprecation is tested;
- how deprecation remains deterministic;
- how deprecation preserves scalability;
- how deprecation preserves POCO-REAF.

 
The governing rule is:
 
> **Deprecate representations when necessary; preserve specified semantics whenever possible.**

 
Deprecation is a controlled language-evolution mechanism. It is not a mechanism for shrinking the language to match temporary compiler, runtime, vendor, or hardware limitations.
 
A feature MUST NOT become deprecated merely because its current implementation is incomplete.
 
A feature MUST NOT be removed merely because a particular backend, device, runtime, or machine cannot support it.
 
***
 
# 2. Normative Language
 
The following terms are normative:
 
- **MUST** — mandatory.
- **MUST NOT** — prohibited.
- **REQUIRED** — mandatory.
- **SHOULD** — recommended unless a documented technical reason exists otherwise.
- **SHOULD NOT** — discouraged unless justified.
- **MAY** — permitted.
- **OPTIONAL** — permitted but not required.

 
A deprecation claim is normative only when it is represented by the feature's authoritative compatibility metadata and supported by conformance tests.
 
A statement in an example, historical document, comment, or proposal MUST NOT by itself create a deprecation guarantee.
 
***
 
# 3. File Completion Contract
 
This file is complete only when its responsibilities can be implemented without requiring another file to redefine what deprecation means.
 
## 3.1 Purpose
 
This file defines the lifecycle and compatibility policy for deprecated Zamani features.
 
## 3.2 Owns
 
This file owns:
 
- deprecation semantics;
- deprecation lifecycle;
- deprecation eligibility;
- deprecation status;
- deprecation metadata;
- deprecation diagnostics policy;
- removal eligibility;
- removal prerequisites;
- deprecation compatibility guarantees;
- relationship between deprecation and language versions;
- relationship between deprecation and migration;
- rules governing deprecated source constructs.

 
## 3.3 Does not own
 
This file does NOT own:
 
- language version numbering;
- grammar syntax;
- lexer token definitions;
- AST node definitions;
- semantic meaning of ordinary features;
- canonical IR definitions;
- quantum IR definitions;
- resource semantics;
- hardware capabilities;
- routing;
- scheduling;
- QEC;
- ZQN;
- HAL;
- runtime implementation;
- compiler optimization;
- migration transformation algorithms;
- reserved identifiers.

 
Those remain owned by their existing repository contracts.
 
***
 
# 4. Repository Authority Model
 
Deprecation is part of a larger authority system.
 
The intended relationship is:
 
```text
grammar/DESIGN.md
        │
        ▼
grammar/specification/
        │
        ├── language specification
        ├── language version
        └── domain semantics
        │
        ▼
grammar/spec/
        │
        ├── compatibility
        ├── versioning
        ├── portability
        └── semantic contracts
        │
        ▼
grammar/compatibility/
        │
        ├── versions.md
        ├── deprecated.md
        ├── migrations.md
        ├── compatibility-matrix.md
        └── dialect compatibility
        │
        ▼
grammar/Zamani.g4
        │
        ▼
lexer
        │
        ▼
parser
        │
        ▼
domain-neutral AST
        │
        ▼
semantic analysis
        │
        ▼
canonical semantic representation
        │
        ├── classical
        ├── quantum
        ├── HDL/hardware
        └── other domains
        │
        ▼
canonical IR
        │
        ├── quantum::ir
        └── other domain IR
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
```
 
No lower layer may redefine a higher-level language contract merely because it has a different implementation representation.
 
***
 
# 5. Relationship to Existing Compatibility Files
 
## 5.1 `compatibility/versions.md`
 
`versions.md` owns:
 
- language version numbering;
- major/minor/patch rules;
- version compatibility;
- supported-version policy;
- language/compiler version separation.

 
It answers:
 
> **What does a version mean?**

 
This file answers:
 
> **What does it mean for a feature to be deprecated within that version system?**

 
***
 
## 5.2 `compatibility/migrations.md`
 
`migrations.md` owns:
 
- migration mechanics;
- transformation algorithms;
- source migration;
- AST migration;
- semantic normalization;
- artifact migration;
- migration validation;
- migration tooling.

 
It answers:
 
> **How does an old representation move to a new representation?**

 
This file answers:
 
> **Why is the old representation deprecated, what is its lifecycle, and when can it be removed?**

 
No migration algorithm should be duplicated here.
 
***
 
## 5.3 `compatibility/compatibility-matrix.md`
 
The compatibility matrix owns explicit relationships among:
 
- language versions;
- grammar versions;
- lexer/parser implementations;
- AST contracts;
- semantic contracts;
- IR versions;
- dialect versions;
- compiler versions;
- runtime contracts;
- artifacts.

 
This file supplies the deprecation status that the matrix must represent.
 
***
 
## 5.4 `specification/language-version.md`
 
Owns the semantic meaning of language versions.
 
A deprecation version MUST use that version model.
 
***
 
## 5.5 `spec/versioning.md`
 
Owns cross-layer version propagation and implementation conformance.
 
Deprecation metadata MUST be propagated consistently across the layers described there.
 
***
 
## 5.6 `spec/compatibility.md`
 
Owns compatibility dimensions.
 
Deprecation MUST distinguish:
 
- source compatibility;
- lexical compatibility;
- grammar compatibility;
- AST compatibility;
- semantic compatibility;
- IR compatibility;
- artifact compatibility;
- runtime compatibility;
- target compatibility.

 
***
 
## 5.7 `grammar/grammar.md`
 
`grammar.md` remains an implementation-conformance reference.
 
It MUST identify deprecated syntax when it remains accepted.
 
It MUST NOT become the authority for deciding whether a feature is deprecated.
 
***
 
## 5.8 `grammar/Zamani-Grammar.md`
 
`Zamani-Grammar.md` remains a broad historical/extended design source.
 
Features appearing there MUST be classified appropriately.
 
Presence in that document alone MUST NOT make a feature:
 
- stable;
- implemented;
- deprecated;
- removable.

 
***
 
# 6. Definition of Deprecation
 
A feature is **deprecated** when:
 
1. it remains recognized or supported under an applicable language/version contract;
2. its use is discouraged for new source;
3. a replacement, rationale, or explicit reason for eventual removal is documented;
4. its compatibility behavior is defined;
5. its lifecycle is tracked.

 
Deprecation does NOT automatically mean:
 
- unsupported;
- invalid;
- removed;
- unsafe;
- inefficient;
- hardware-specific;
- semantically incorrect;
- unavailable on every target.

 
A deprecated feature remains valid until the applicable removal policy says otherwise.
 
***
 
# 7. Deprecation Is Not Implementation Failure
 
The following are implementation states and MUST NOT automatically become deprecation states:
 
```text
specified
parsed
AST-supported
semantically-supported
IR-supported
compiler-supported
runtime-supported
backend-supported
target-supported
```
 
For example:
 
```text
grammar supports feature
backend does not support feature
```
 
does NOT imply:
 
```text
feature is deprecated
```
 
Instead:
 
```text
feature remains part of the language
        │
        ▼
capability analysis
        │
        ▼
target/backend feasibility
```
 
must determine whether a particular realization is possible.
 
***
 
# 8. Invalid Reasons for Deprecation
 
A feature MUST NOT be deprecated solely because:
 
- one CPU cannot execute it;
- one GPU lacks support;
- one FPGA lacks a required resource;
- one ASIC lacks an implementation;
- one QPU lacks a native operation;
- one QPU has too few physical qubits;
- a device has insufficient memory;
- a topology cannot realize an operation;
- routing is currently unavailable;
- scheduling is currently unavailable;
- QEC is currently unavailable;
- ZQN is currently incomplete;
- a runtime has not implemented it;
- a vendor does not support it;
- the compiler implementation is currently difficult;
- a test machine is small;
- a test environment lacks an accelerator;
- a particular compiler build has a fixed internal capacity.

 
Those are capability, resource, implementation, or target issues.
 
They MUST be represented using the appropriate subsystem:
 
- capabilities;
- requirements;
- constraints;
- resource analysis;
- routing;
- scheduling;
- resilience;
- QEC;
- ZQN;
- HAL;
- runtime;
- deployment.

 
***
 
# 9. POCO-REAF Requirement
 
Deprecation MUST preserve the distinction between:
 
```text
program semantics
```
 
and:
 
```text
target realization
```
 
A deprecated portable feature MUST NOT be replaced by a target-specific representation merely because the latter is easier to implement.
 
Correct:
 
```text
legacy source
    ↓
portable semantic meaning
    ↓
canonical semantic model
    ↓
canonical IR
    ↓
target realization
```
 
Incorrect:
 
```text
legacy portable source
    ↓
specific CPU
specific GPU
specific QPU
specific FPGA
specific physical qubit
```
 
unless the original program explicitly required that target-specific behavior.
 
***
 
# 10. Scalability Invariant
 
Deprecation MUST preserve Zamani's scalability objective:
 
> From the smallest supported computation to arbitrarily large realizations, subject only to actual available resources and explicit semantic requirements.

 
Deprecation MUST NOT introduce universal language limits such as:
 
```text
MAX_QUBITS
MAX_LOGICAL_QUBITS
MAX_PHYSICAL_QUBITS
MAX_CPUS
MAX_CORES
MAX_THREADS
MAX_GPUS
MAX_FPGAS
MAX_ASICS
MAX_NODES
MAX_MEMORY
MAX_REGISTER_WIDTH
MAX_TENSOR_RANK
MAX_NETWORK_SIZE
MAX_DEVICE_COUNT
MAX_TIMELINES
```
 
or equivalent constants.
 
A migration from old to new syntax MUST NOT introduce such limits either.
 
***
 
# 11. Resource Availability Is Not Deprecation
 
These are semantically different:
 
```text
feature is deprecated
```
 
and:
 
```text
target lacks capability
```
 
and:
 
```text
target lacks resources
```
 
and:
 
```text
backend does not implement lowering
```
 
For example:
 
```text
requires qubits >= n
```
 
may be valid indefinitely.
 
If a target has insufficient qubits, the result is a resource/capability failure, not language deprecation.
 
Likewise:
 
```text
requires capability("quantum.measurement")
```
 
must not become deprecated because one QPU lacks measurement support.
 
***
 
# 12. Deprecation Lifecycle
 
The normative lifecycle is:
 
```text
PROPOSED
    ↓
EXPERIMENTAL
    ↓
STABLE
    ↓
DEPRECATED
    ↓
MIGRATION_AVAILABLE
    ↓
REMOVAL_ELIGIBLE
    ↓
REMOVED
```
 
Not every feature must traverse every state.
 
An experimental feature MAY transition directly:
 
```text
EXPERIMENTAL → REMOVED
```
 
A stable feature SHOULD follow:
 
```text
STABLE
  ↓
DEPRECATED
  ↓
MIGRATION_AVAILABLE
  ↓
REMOVAL_ELIGIBLE
  ↓
REMOVED
```
 
A stable feature MUST NOT silently transition directly from:
 
```text
STABLE → REMOVED
```
 
except under the explicit security/correctness emergency process defined below.
 
***
 
# 13. Status Definitions
 
## 13.1 `PROPOSED`
 
The feature is under design.
 
No compatibility promise exists.
 
***
 
## 13.2 `EXPERIMENTAL`
 
The feature exists for experimentation.
 
Experimental functionality:
 
- MAY change;
- MAY be removed;
- SHOULD be explicitly marked;
- MUST NOT be represented as a stable guarantee.

 
***
 
## 13.3 `STABLE`
 
The feature is part of the supported language contract.
 
Normal compatibility guarantees apply.
 
***
 
## 13.4 `DEPRECATED`
 
The feature remains supported but new source SHOULD use the replacement or preferred representation.
 
***
 
## 13.5 `MIGRATION_AVAILABLE`
 
The feature remains supported and a documented migration path exists.
 
Automatic migration SHOULD be provided when safe.
 
***
 
## 13.6 `REMOVAL_ELIGIBLE`
 
All required removal prerequisites have been satisfied.
 
The feature is still not necessarily removed.
 
***
 
## 13.7 `REMOVED`
 
The feature is no longer part of the current language contract.
 
Historical migration documentation MUST remain available.
 
***
 
# 14. Stable Feature Identity
 
Every deprecated feature MUST have an identity independent of its spelling.
 
The identity SHOULD be based on:
 
```text
domain
namespace
feature
semantic role
```
 
rather than only its textual representation.
 
For example:
 
```text
quantum.operation
```
 
may remain the same semantic feature even if an old spelling changes.
 
Therefore:
 
```text
old spelling → new spelling
```
 
does not automatically mean:
 
```text
old semantic feature → new semantic feature
```
 
***
 
# 15. Required Deprecation Record
 
Every stable feature entering deprecation MUST have a record containing, at minimum:
 
```text
Feature ID:
Canonical Name:
Domain:
Namespace:
Feature Kind:
Status:

Introduced:
Stable Since:
Deprecated Since:
Earliest Removal Version:

Reason:
Replacement:
Migration ID:
Migration Class:

Source Compatibility:
Lexical Compatibility:
Grammar Compatibility:
AST Compatibility:
Semantic Compatibility:
IR Compatibility:
Runtime Compatibility:
Target Compatibility:

Automatic Migration:
Manual Action Required:
Semantic Preservation:

Diagnostic Code:
Warning Policy:

Affected Grammar Files:
Affected Lexer Files:
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

Documentation:
Provenance:
Owner:
```
 
A deprecation record is incomplete until all applicable fields are resolved.
 
***
 
# 16. Machine-Readable Feature Metadata
 
Where the repository uses feature manifests under:
 
```text
grammar/specification/features/
```
 
the deprecation state MUST be represented there as well.
 
The manifest SHOULD contain fields equivalent to:
 
```yaml
id:
name:
domain:
status:
introduced:
stable_since:
deprecated_since:
earliest_removal:
replacement:
migration:
semantic_preservation:
source_compatibility:
grammar_compatibility:
ast_compatibility:
semantic_compatibility:
ir_compatibility:
runtime_compatibility:
target_compatibility:
diagnostic:
tests:
provenance:
```
 
The manifest MUST NOT contradict this document.
 
Generated documentation MAY be derived from the manifests, but generated output MUST NOT become a competing authority.
 
***
 
# 17. Feature Identity Versus File Location
 
A feature's deprecation state is global.
 
A feature MUST NOT be:
 
```text
deprecated in grammar/quantum/
stable in grammar/Zamani.g4
```
 
or:
 
```text
deprecated in documentation
stable in lexer
```
 
without an explicitly documented implementation-conformance discrepancy.
 
The feature identity determines status, not the file in which it happens to appear.
 
***
 
# 18. Lexical Deprecation
 
The following may be deprecated independently:
 
- keywords;
- operators;
- punctuation spellings;
- literal spellings;
- escape forms;
- annotations;
- comment forms;
- identifier aliases.

 
Lexical deprecation MUST account for tokenization precedence.
 
In particular, known token-collision areas such as:
 
```text
Question / QuestionMark
Ampersand / BitAnd
```
 
MUST have one canonical lexical interpretation.
 
A deprecation MUST NOT be used to conceal unresolved lexical ambiguity.
 
***
 
# 19. Grammar Deprecation
 
A grammar construct may remain in `Zamani.g4` while deprecated.
 
The grammar rule MUST remain deterministic for as long as the feature is supported.
 
Deprecation metadata belongs to compatibility/specification metadata, not to grammar syntax alone.
 
A grammar rule existing in:
 
```text
Zamani-Grammar.md
```
 
does not establish stable syntax.
 
A grammar rule existing in an experimental modular grammar does not automatically establish stable syntax.
 
***
 
# 20. Deprecated Syntax Must Be Deterministic
 
Deprecated syntax MUST NOT be parsed using:
 
- heuristic guessing;
- random choice;
- target-dependent interpretation;
- runtime-dependent interpretation;
- compiler-build-dependent interpretation;
- machine-size-dependent interpretation.

 
For the same:
 
```text
source
language version
dialect set
compiler compatibility configuration
```
 
the interpretation MUST be deterministic.
 
***
 
# 21. Deprecated Syntax Must Not Be Silently Reinterpreted
 
If old and new forms have different semantics, the compiler MUST NOT silently reinterpret the old form as the new form.
 
It must instead:
 
1. preserve the historical meaning;
2. require explicit migration; or
3. reject the construct under the applicable language version.

 
***
 
# 22. Deprecation Diagnostics
 
When a deprecated feature remains accepted, compilation SHOULD produce a structured diagnostic.
 
A diagnostic SHOULD identify:
 
```text
diagnostic code
feature ID
feature name
deprecated since
reason
replacement
migration ID
earliest removal version
source span
```
 
Example:
 
```text
warning: deprecated Zamani feature

feature: <feature-id>
deprecated-since: <version>
replacement: <replacement>
migration: <migration-id>
earliest-removal: <version>
```
 
The exact human-readable wording MAY evolve.
 
The machine-readable diagnostic identity MUST remain stable.
 
***
 
# 23. Diagnostic Severity
 
The preferred levels are:
 
```text
INFO
WARNING
ERROR
```
 
`INFO` may be used by compatibility inspection tools.
 
`WARNING` is appropriate for deprecated but supported source.
 
`ERROR` is required when a feature has been removed from the effective language version.
 
A removed feature MUST NOT be reported as merely deprecated.
 
***
 
# 24. Warning Suppression
 
If Zamani supports warning suppression, suppression:
 
MUST NOT:
 
- restore removed syntax;
- change semantic meaning;
- bypass compatibility checks;
- bypass security checks;
- turn a removed feature into valid syntax.

 
Suppression affects diagnostics only.
 
***
 
# 25. Replacement Requirements
 
A stable feature SHOULD NOT be deprecated without a documented replacement unless:
 
- no meaningful replacement exists;
- the feature is inherently obsolete;
- retaining it causes a documented correctness problem;
- retaining it creates a documented security problem;
- it was never actually part of the stable implementation contract.

 
A replacement MUST describe semantic meaning, not merely spelling.
 
***
 
# 26. Replacement Generality
 
Where a feature is deprecated because it is too target-specific or narrow, the replacement SHOULD be more general.
 
For example:
 
```text
fixed device selection
```
 
should normally migrate toward:
 
```text
capability
requirement
constraint
preference
placement intent
```
 
rather than another fixed device identifier.
 
This preserves POCO-REAF.
 
***
 
# 27. Automatic Migration
 
Automatic migration SHOULD be provided when:
 
- the transformation is deterministic;
- the transformation is semantics-preserving;
- no information is lost;
- there is one unambiguous replacement;
- source provenance can be retained.

 
Examples of suitable migrations include simple spelling changes where semantics are identical.
 
Automatic migration MUST NOT guess when multiple semantic replacements are possible.
 
***
 
# 28. Migration Ownership
 
All transformation mechanics belong to:
 
```text
grammar/compatibility/migrations.md
```
 
This file MUST NOT duplicate the migration engine.
 
For every deprecated feature with a migration path, this file references a stable migration identity.
 
Example:
 
```text
Migration ID: ZM-MIG-<stable-id>
```
 
The migration document owns the transformation.
 
This document owns the deprecation lifecycle.
 
***
 
# 29. Migration Invariants
 
A migration SHOULD be:
 
- deterministic;
- idempotent;
- lossless;
- source-span preserving;
- provenance preserving;
- semantics-preserving where promised;
- scalable;
- independent of target hardware.

 
For automatic migration:
 
```text
migrate(migrate(source))
```
 
MUST produce the same semantic result as:
 
```text
migrate(source)
```
 
***
 
# 30. Lossless Deprecation
 
Migration SHOULD preserve:
 
- comments where practical;
- source spans;
- identifiers;
- attributes;
- annotations;
- documentation;
- metadata;
- semantic information.

 
If information cannot be preserved, the migration MUST explicitly identify the loss.
 
Silent information loss is prohibited.
 
***
 
# 31. Semantic Equivalence
 
Where a deprecation promises semantic preservation:
 
```text
old source
    ↓
old semantics
```
 
and:
 
```text
migrated source
    ↓
new semantics
```
 
MUST be equivalent according to the applicable semantic contract.
 
Textual similarity is not sufficient.
 
AST similarity is not sufficient.
 
IR similarity is not necessarily required if different IR representations have equivalent semantics.
 
***
 
# 32. AST Integration
 
Deprecated syntax SHOULD normally follow:
 
```text
deprecated source
       ↓
lexer
       ↓
parser
       ↓
normal/domain-neutral AST
       ↓
semantic normalization
       ↓
current semantic model
```
 
The AST SHOULD NOT accumulate permanent duplicate node families merely because multiple historical spellings exist.
 
If two syntactic forms have identical semantics, they SHOULD normalize to the same semantic representation.
 
***
 
# 33. AST Compatibility
 
A deprecation migration MUST preserve applicable:
 
- names;
- bindings;
- scopes;
- types;
- generics;
- effects;
- capabilities;
- resource requirements;
- resource constraints;
- control flow;
- ownership;
- concurrency semantics;
- quantum semantics;
- HDL semantics;
- provenance.

 
Deprecated syntax MUST NOT force target-specific structures into the domain-neutral AST.
 
***
 
# 34. Semantic Analysis Integration
 
Semantic analysis determines whether a deprecated construct is valid under the effective language version.
 
Preferred flow:
 
```text
historical syntax
       ↓
AST
       ↓
semantic normalization
       ↓
current semantic model
```
 
Semantic normalization MUST happen before domain-specific target realization.
 
***
 
# 35. Canonical IR Integration
 
Deprecated syntax MUST ultimately reach the current canonical semantic/IR architecture.
 
The compatibility layer MUST NOT create a permanent legacy IR solely for historical syntax.
 
Preferred flow:
 
```text
deprecated source
       ↓
AST
       ↓
semantic normalization
       ↓
canonical IR
```
 
***
 
# 36. Quantum Deprecation
 
Quantum features MUST be deprecated according to semantic meaning, not hardware age.
 
A quantum feature MUST NOT be deprecated merely because:
 
- a QPU lacks a native operation;
- a simulator handles it differently;
- a topology cannot directly implement it;
- a device has insufficient physical qubits;
- routing is unavailable;
- scheduling is unavailable;
- a backend has not implemented a decomposition.

 
Those are downstream concerns.
 
***
 
# 37. Quantum Canonical Boundary
 
All quantum deprecation paths MUST preserve:
 
```text
quantum source
    ↓
domain-neutral AST
    ↓
quantum semantic normalization
    ↓
quantum::ir
    ↓
optimization
    ↓
routing
    ↓
scheduling
    ↓
QEC / resilience / ZQN
    ↓
HAL
    ↓
target
```
 
`quantum::ir` remains the canonical quantum semantic boundary.
 
No new permanent frontend quantum IR may be introduced merely to accommodate deprecated syntax.
 
***
 
# 38. Quantum Operation Evolution
 
The grammar MUST NOT evolve toward a new fixed enumeration of physical gates merely because old gate syntax is deprecated.
 
The preferred semantic model remains data-driven:
 
```text
operation name
namespace
operands
parameters
results
attributes
modifiers
effects
capabilities
source provenance
```
 
Therefore historical syntax such as a legacy fixed operation form SHOULD normalize into the generic quantum operation model where semantics permit.
 
The migration MUST preserve:
 
- operation identity;
- namespace;
- operands;
- parameters;
- controls;
- modifiers;
- measurement semantics;
- ordering;
- source provenance.

 
***
 
# 39. Quantum Scalability
 
Deprecation MUST NOT introduce:
 
```text
MAX_QUBITS
MAX_LOGICAL_QUBITS
MAX_PHYSICAL_QUBITS
MAX_REGISTER_SIZE
```
 
or equivalent universal restrictions.
 
A source declaration such as:
 
```text
Qubit[n]
```
 
expresses program/resource semantics.
 
It does not establish a compiler maximum.
 
***
 
# 40. Logical Versus Physical Quantum State
 
If a deprecated construct conflates logical and physical qubits, migration MUST preserve the distinction.
 
Portable source describes logical quantum intent.
 
Physical realization belongs to:
 
- resource analysis;
- routing;
- scheduling;
- QEC;
- HAL;
- target description.

 
A migration MUST NOT silently replace logical qubits with physical device identities.
 
***
 
# 41. QEC Integration
 
Deprecating QEC-related syntax MUST preserve QEC intent.
 
A migration MUST distinguish:
 
```text
required resilience/error correction
```
 
from:
 
```text
specific code
specific decoder
specific physical layout
specific device
```
 
unless the original source explicitly specified those details.
 
The QEC subsystem remains responsible for realization.
 
***
 
# 42. ZQN Integration
 
ZQN-related syntax MUST remain separate from:
 
- quantum semantic representation;
- QEC;
- routing;
- scheduling;
- hardware abstraction.

 
A deprecated ZQN construct MUST normalize into the existing canonical architecture.
 
No duplicate fault/noise IR may be introduced solely for migration.
 
***
 
# 43. Classical Deprecation
 
Classical migrations MUST preserve applicable:
 
- arithmetic semantics;
- numeric representation;
- overflow behavior;
- rounding;
- precision;
- signedness;
- exceptional values;
- evaluation order;
- control flow;
- ownership;
- memory behavior;
- concurrency;
- synchronization;
- error behavior.

 
A change to any of these is semantic and MUST NOT be classified as a mere spelling migration.
 
***
 
# 44. HDL Deprecation
 
HDL deprecation MUST preserve applicable hardware intent:
 
- signal behavior;
- combinational semantics;
- sequential semantics;
- state transitions;
- clock relationships;
- reset behavior;
- timing constraints;
- interfaces;
- parameterization;
- verification intent.

 
A synthesis limitation is not automatically a language deprecation.
 
***
 
# 45. Parameterized HDL
 
A deprecated fixed-width representation SHOULD migrate toward parameterized semantics where the width itself is program meaning.
 
The migration MUST NOT turn an implementation-specific width into a universal language maximum.
 
For example, a fixed-width construct must not cause the grammar to establish:
 
```text
MAX_REGISTER_WIDTH
```
 
as a language rule.
 
***
 
# 46. Hardware Deprecation
 
Hardware-facing syntax MUST distinguish:
 
```text
hardware intent
```
 
from:
 
```text
hardware instance
```
 
Portable replacements SHOULD prefer:
 
```text
capability
requirement
constraint
resource
preference
topology intent
placement intent
```
 
over fixed device identities where portability is intended.
 
***
 
# 47. Resource Deprecation
 
Resource features MUST preserve the distinction among:
 
```text
resource
requirement
constraint
capability
preference
hint
availability
allocation
placement
```
 
A migration MUST NOT collapse these concepts.
 
For example:
 
```text
requires qubits >= n
```
 
is semantically different from:
 
```text
map q0 -> physical_qubit(17)
```
 
The former is portable resource intent.
 
The latter is target realization.
 
***
 
# 48. Distributed Computing Deprecation
 
Distributed migrations MUST preserve applicable:
 
- communication semantics;
- ordering;
- consistency;
- replication;
- partitioning;
- fault behavior;
- service identity;
- placement intent;
- message semantics.

 
A migration MUST NOT introduce a fixed maximum node count.
 
***
 
# 49. AI and Data Deprecation
 
AI/data migrations MUST preserve semantic meaning independently of:
 
- accelerator vendor;
- accelerator count;
- fixed memory capacity;
- fixed tensor rank where abstraction is appropriate;
- fixed tensor dimensions where not semantically required;
- framework implementation.

 
Framework-specific syntax MUST remain outside the portable language core unless explicitly standardized as part of the language.
 
***
 
# 50. Networking Deprecation
 
Networking syntax may be deprecated because of:
 
- obsolete protocol semantics;
- obsolete API representation;
- security requirements;
- incompatible external standards.

 
It MUST NOT be deprecated merely because a particular network topology or device does not support it.
 
Portable network requirements SHOULD remain abstract.
 
***
 
# 51. Security Deprecation
 
Security-sensitive features require additional safeguards.
 
A deprecated security feature MUST NOT silently weaken security.
 
If a replacement has stronger guarantees, the migration SHOULD make the change explicit.
 
If the old construct has a severe security defect, accelerated removal MAY be used under the emergency process.
 
***
 
# 52. Dialect Deprecation
 
Dialect deprecation is independent of core-language deprecation.
 
A dialect MUST have:
 
```text
identity
namespace
version
owner
status
compatibility contract
migration policy
```
 
Deprecating:
 
```text
dialect X
```
 
does not automatically deprecate a core Zamani semantic feature used by that dialect.
 
***
 
# 53. Vendor Extensions
 
Vendor-specific syntax MAY be deprecated independently.
 
Vendor syntax MUST NOT become core syntax merely because it is popular.
 
Likewise, a vendor extension MUST NOT force the core language to adopt:
 
- fixed device counts;
- fixed topologies;
- fixed register widths;
- fixed memory capacities;
- fixed quantum hardware limits.

 
***
 
# 54. Interoperability Deprecation
 
Interoperability formats such as:
 
- OpenQASM;
- QIR;
- HDL formats;
- LLVM-related formats;
- MLIR-related formats;
- foreign-language interfaces;

 
are compatibility boundaries, not replacements for the canonical Zamani semantic model.
 
Deprecating an interoperability format MUST NOT deprecate the underlying Zamani semantics automatically.
 
The interoperability layer owns external-format conversion.
 
***
 
# 55. Deprecated OpenQASM/External Quantum Syntax
 
A deprecated external quantum representation MUST normalize through:
 
```text
external format
    ↓
Zamani semantic model
    ↓
quantum::ir
```
 
It MUST NOT establish an external format as a second canonical quantum IR.
 
Any existing OpenQASM frontend structure MUST remain integrated with the current repository organization rather than creating a competing quantum frontend architecture.
 
***
 
# 56. Reserved Syntax
 
When a deprecated feature is removed, its former syntax may become:
 
- reserved;
- reusable under an explicit version rule;
- available to a dialect;
- available for a replacement;
- permanently unavailable.

 
Reserved identifiers and namespaces remain owned by the repository's reserved-syntax contract if present.
 
Removal MUST record any change in reserved status.
 
***
 
# 57. Removed Syntax Must Not Be Silently Reused
 
A removed syntax form SHOULD NOT immediately acquire unrelated semantics.
 
Before reuse, the repository MUST establish that old source cannot be silently reinterpreted.
 
Version resolution MUST make the distinction deterministic.
 
***
 
# 58. Experimental Features
 
Experimental features have different guarantees from deprecated stable features.
 
An experimental feature may change or disappear without the same compatibility window as a stable feature.
 
However, experimental status MUST be explicit.
 
The compiler and documentation MUST distinguish:
 
```text
EXPERIMENTAL
```
 
from:
 
```text
DEPRECATED
```
 
They are not interchangeable.
 
***
 
# 59. Stable Features
 
A stable feature receives normal compatibility guarantees.
 
Before stable functionality is deprecated:
 
1. its identity must be known;
2. its consumers must be audited;
3. its semantic contract must be known;
4. its replacement must be defined where practical;
5. its migration path must be defined where practical;
6. its diagnostics must be defined;
7. its tests must exist;
8. the compatibility matrix must be updated.

 
***
 
# 60. Deprecation of Never-Implemented Features
 
A feature appearing only in:
 
- historical documentation;
- proposals;
- incomplete grammar;
- experimental files;
- unimplemented examples;

 
MUST NOT automatically receive a stable deprecation promise.
 
The repository MUST first determine whether the feature was ever:
 
- normative;
- accepted by the implementation;
- documented as stable;
- part of a compatibility guarantee.

 
If not, it may instead be classified as:
 
```text
PROPOSED
EXPERIMENTAL
PLANNED
NOT IMPLEMENTED
HISTORICAL
```
 
as appropriate.
 
This is particularly important for the broad material retained in `Zamani-Grammar.md`.
 
***
 
# 61. Repository-Wide Deprecation Audit
 
Before approving a stable-feature deprecation, the repository MUST be searched for:
 
```text
grammar/
src/
tests/
examples/
docs/
```
 
and, where present:
 
```text
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
```
 
The exact paths MUST follow the actual repository.
 
The audit MUST identify:
 
- grammar definitions;
- lexer tokens;
- parser rules;
- AST nodes;
- semantic consumers;
- IR consumers;
- compiler consumers;
- runtime consumers;
- backend consumers;
- interoperability consumers;
- tests;
- examples;
- documentation;
- feature manifests;
- dialects.

 
A feature MUST NOT be considered safely removable merely because its grammar rule disappeared.
 
***
 
# 62. Integration With `Zamani.g4`
 
`grammar/Zamani.g4` remains the canonical ANTLR composition root.
 
Deprecated syntax that remains supported MUST have:
 
- deterministic lexical representation;
- deterministic parser representation;
- semantic compatibility;
- diagnostic support;
- migration metadata.

 
`Zamani.g4` MUST NOT become a deprecation registry.
 
The compatibility layer owns lifecycle metadata.
 
***
 
# 63. Integration With Modular Grammar Files
 
If grammar functionality is split across:
 
```text
lexer/
core/
types/
expressions/
statements/
quantum/
hdl/
hardware/
...
```
 
the deprecation status remains global.
 
A modular file MAY define syntax.
 
It MUST NOT independently redefine lifecycle status.
 
***
 
# 64. Integration With `grammar.md`
 
`grammar.md` MUST report the implementation status of deprecated features accurately.
 
A deprecated-but-supported feature MAY appear as accepted syntax.
 
Its status MUST clearly indicate:
 
```text
DEPRECATED
```
 
rather than presenting it as recommended syntax.
 
***
 
# 65. Integration With `Zamani-Grammar.md`
 
The broad grammar/design document may retain historical syntax.
 
It MUST classify it appropriately.
 
A historical feature MUST NOT be revived accidentally merely because a developer copies an old rule into the canonical grammar.
 
Promotion requires the established lifecycle:
 
```text
proposal
→ semantic design
→ AST contract
→ grammar
→ implementation
→ IR
→ tests
→ compatibility review
→ stable
```
 
***
 
# 66. Integration With the Lexer
 
The lexer MUST continue to recognize deprecated lexical forms while they remain supported.
 
If a deprecated spelling is removed:
 
1. the lexical contract changes;
2. parser behavior is updated;
3. diagnostics are updated;
4. migration metadata is updated;
5. tests are updated.

 
A lexer implementation change MUST NOT silently alter the semantic meaning of unrelated syntax.
 
***
 
# 67. Integration With the AST
 
The AST MUST remain domain-neutral at the frontend boundary.
 
Deprecation MUST NOT introduce:
 
- physical topology;
- vendor backend structures;
- calibration state;
- routing decisions;
- scheduling decisions;
- QEC implementation details;

 
into the frontend AST merely to support historical syntax.
 
***
 
# 68. Integration With Semantic Analysis
 
Semantic analysis MUST determine:
 
- whether the deprecated feature is legal for the effective version;
- its historical meaning;
- its normalized meaning;
- whether migration is required;
- whether semantic equivalence is preserved.

 
***
 
# 69. Integration With Canonical IR
 
Deprecated source MUST ultimately lower through current canonical IR contracts.
 
No permanent legacy IR should be added solely to preserve deprecated source.
 
For quantum:
 
```text
deprecated quantum syntax
        ↓
AST
        ↓
semantic normalization
        ↓
quantum::ir
```
 
For classical and HDL/hardware constructs, the corresponding canonical domain IR remains authoritative.
 
***
 
# 70. Integration With Routing, Scheduling, QEC, and ZQN
 
Deprecation MUST NOT transfer ownership of downstream responsibilities into the grammar.
 
Routing owns physical realization.
 
Scheduling owns ordering and resource scheduling.
 
QEC owns error-correction realization.
 
ZQN owns its defined resilience/noise/fault semantics.
 
HAL owns hardware realization.
 
Deprecation merely determines how historical source reaches those current systems.
 
***
 
# 71. Runtime Integration
 
Runtime behavior MUST NOT silently change merely because source syntax became deprecated.
 
If runtime semantics change, that is a separate compatibility event and MUST receive its own compatibility classification.
 
***
 
# 72. Tooling Integration
 
Tooling SHOULD provide:
 
- deprecation diagnostics;
- feature status;
- replacement suggestions;
- migration links/identifiers;
- machine-readable metadata;
- compatibility inspection;
- safe automatic migration where available.

 
Tooling MUST use the same feature identity as the specification.
 
***
 
# 73. Documentation Integration
 
Every deprecated feature MUST be represented in:
 
- compatibility documentation;
- feature documentation;
- migration documentation when applicable;
- release documentation when introduced;
- removal documentation when removed.

 
Historical information MUST remain available after removal.
 
***
 
# 74. Test Requirements
 
Every stable deprecated feature MUST have, where applicable:
 
### Positive tests
 
Old syntax remains accepted while supported.
 
### Diagnostic tests
 
Deprecation warning is emitted with the expected identity.
 
### Migration tests
 
Migration produces the intended replacement.
 
### Semantic tests
 
Old and migrated source have equivalent semantics where promised.
 
### Removal tests
 
Removed syntax is rejected under the removal version.
 
### Regression tests
 
Replacement syntax remains valid.
 
***
 
# 75. Negative Tests
 
Negative tests MUST cover:
 
- malformed deprecated syntax;
- invalid version declarations;
- unsupported versions;
- removed constructs;
- ambiguous replacements;
- invalid migration targets;
- incompatible dialect versions;
- conflicting compatibility requirements.

 
Failures MUST be deterministic.
 
***
 
# 76. Boundary Tests
 
Boundary tests SHOULD cover:
 
- smallest valid deprecated program;
- nested deprecated constructs;
- repeated deprecated constructs;
- interactions with modules;
- interactions with generics;
- interactions with effects;
- interactions with resources;
- interactions with quantum constructs;
- interactions with HDL;
- interactions with distributed constructs;
- large source files;
- large numbers of declarations;
- large operation sequences.

 
No artificial maximum should be encoded merely for testing convenience.
 
***
 
# 77. Scalability Tests
 
Deprecation tests MUST demonstrate that semantics do not depend on arbitrary machine limits.
 
The same semantic construct SHOULD be testable against:
 
```text
tiny realization
small realization
medium realization
large realization
distributed realization
heterogeneous realization
future realization
```
 
subject to actual resource and capability availability.
 
***
 
# 78. Determinism Tests
 
Given identical:
 
```text
source
language version
dialect set
compiler compatibility configuration
migration configuration
```
 
deprecation behavior MUST be identical.
 
It MUST NOT depend on:
 
- CPU count;
- GPU availability;
- QPU availability;
- memory size;
- network state;
- wall-clock time;
- random numbers;
- hash iteration order;
- undocumented environment variables.

 
***
 
# 79. Reproducibility
 
Deprecation decisions MUST be reproducible.
 
A fixed:
 
```text
language version
compiler version
dependency set
dialect set
migration configuration
```
 
must produce deterministic deprecation classification and diagnostics.
 
***
 
# 80. Cross-Domain Tests
 
The repository SHOULD test deprecated features in combinations including:
 
```text
classical + quantum
classical + HDL
quantum + HDL
quantum + hardware
quantum + distributed
AI + quantum
AI + hardware
classical + quantum + distributed
classical + quantum + HDL + hardware
```
 
This prevents a deprecation from appearing correct in isolation while breaking cross-domain semantics.
 
***
 
# 81. Round-Trip Tests
 
Where source serialization exists:
 
```text
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
```
 
must preserve intended semantics.
 
***
 
# 82. No Regex-Only Semantic Migration
 
Textual replacement MUST NOT be used as the sole migration mechanism for semantic changes.
 
Safe lexical spelling changes may use token-aware transformation.
 
Semantic migrations SHOULD operate on:
 
```text
tokens
AST
semantic model
canonical representation
```
 
as appropriate.
 
For example, changing a quantum operation's semantic representation MUST NOT rely solely on replacing text strings.
 
***
 
# 83. Source Provenance
 
Migration and deprecation processing SHOULD preserve:
 
- original source span;
- original spelling;
- normalized spelling;
- feature ID;
- migration ID;
- language version;
- dialect version;
- transformation status.

 
This allows diagnostics and debugging to explain how historical source reached the current representation.
 
***
 
# 84. Compatibility Classification
 
Every deprecation MUST be classified across applicable dimensions:
 

|Dimension|Possible status|
|---|---|
|Source|compatible / incompatible|
|Lexer|compatible / incompatible|
|Grammar|compatible / incompatible|
|AST|compatible / normalized / incompatible|
|Semantics|equivalent / changed / incompatible|
|IR|equivalent / translated / incompatible|
|Artifact|compatible / translated / incompatible|
|Runtime|compatible / conditional / incompatible|
|Target|compatible / capability-dependent / incompatible|
|Dialect|compatible / translated / incompatible|
 
A syntax-only deprecation SHOULD normally preserve semantic compatibility.
 
A semantic change MUST NOT be mislabeled as a spelling-only deprecation.
 
***
 
# 85. Deprecation and Versioning
 
A deprecation record MUST identify:
 
```text
deprecated since
earliest removal version
```
 
using the language version system from:
 
```text
grammar/compatibility/versions.md
grammar/specification/language-version.md
```
 
Rust versions MUST NOT be used as deprecation version identifiers.
 
For example:
 
```text
Deprecated since: Zamani 1.4.0
```
 
is meaningful.
 
```text
Deprecated since: Rust 1.97.1
```
 
is not a Zamani language deprecation.
 
***
 
# 86. Rust Implementation Requirements
 
The reference implementation baseline is:
 
```text
Rust 1.97 / Rust 1.97.1
Rust 2021
```
 
The implementation MUST use safe Rust.
 
Deprecation handling MUST NOT require:
 
```rust
unsafe
```
 
or equivalent unsafe behavior.
 
Implementation SHOULD favor deterministic data structures and processing.
 
Where output order is observable, ordering MUST be explicitly defined rather than depending on hash iteration order.
 
***
 
# 87. No Network-Dependent Deprecation
 
Compiler deprecation classification MUST NOT depend on live network access.
 
The result MUST be derivable from:
 
- source;
- language specification;
- local compatibility metadata;
- compiler configuration;
- declared dependencies/dialects.

 
Network-based package metadata may be a tooling concern, but it MUST NOT silently redefine language deprecation semantics during compilation.
 
***
 
# 88. No Hardware-Dependent Deprecation
 
The compiler MUST NOT decide that a feature is deprecated because the current machine is:
 
- too small;
- too large;
- missing a GPU;
- missing a QPU;
- missing an FPGA;
- missing memory;
- missing a particular topology.

 
Hardware feasibility belongs downstream.
 
***
 
# 89. Compatibility Modes
 
A compiler MAY provide modes such as:
 
```text
strict
default
legacy
migration
```
 
if required.
 
Any such mode MUST:
 
- be explicitly defined;
- be deterministic;
- report the effective language version;
- not silently alter semantics;
- not restore removed language features.

 
Legacy mode MUST NOT become a permanent replacement for normal migration.
 
***
 
# 90. Legacy Mode
 
If supported, legacy mode MAY accept historical constructs that are otherwise difficult to infer from the current source version.
 
Legacy mode MUST:
 
- be explicit;
- identify the source/language version;
- produce appropriate diagnostics;
- remain deterministic;
- use the same semantic model;
- ultimately reach the current canonical IR.

 
Legacy mode MUST NOT create a permanent parallel compiler architecture.
 
***
 
# 91. Forward Compatibility
 
Unknown future syntax MUST NOT be treated as deprecated syntax.
 
A compiler encountering unknown syntax MUST either:
 
- use an explicitly defined extension mechanism; or
- report an unknown-feature/unsupported-version diagnostic.

 
It MUST NOT guess that future syntax means an old deprecated construct.
 
***
 
# 92. Feature Flags
 
Feature flags MAY control implementation availability.
 
They MUST NOT redefine the normative language contract.
 
A compiler build lacking an experimental feature does not thereby change whether that feature is deprecated.
 
***
 
# 93. Security and Correctness Emergency
 
Accelerated removal MAY occur when retaining a stable feature creates a severe:
 
- security vulnerability;
- semantic unsoundness;
- data-integrity risk;
- compiler-correctness defect.

 
The release MUST document:
 
- feature identity;
- affected versions;
- reason;
- severity;
- mitigation;
- replacement;
- migration;
- compatibility consequences.

 
This mechanism MUST NOT become a general shortcut around normal deprecation.
 
***
 
# 94. Removal Prerequisites
 
A stable feature is `REMOVAL_ELIGIBLE` only after all applicable conditions are satisfied:
 
1. stable feature identity exists;
2. deprecation version is documented;
3. reason is documented;
4. replacement is documented where practical;
5. migration ID exists where applicable;
6. migration is tested where applicable;
7. diagnostics exist;
8. compatibility matrix is updated;
9. grammar status is updated;
10. lexer status is updated;
11. parser status is updated;
12. AST implications are resolved;
13. semantic implications are resolved;
14. IR implications are resolved;
15. compiler consumers are audited;
16. runtime consumers are audited;
17. dialect consumers are audited;
18. documentation is updated;
19. positive tests are updated;
20. negative tests exist;
21. boundary tests exist;
22. scalability tests exist;
23. determinism tests exist;
24. no hidden stable consumer remains;
25. repository-wide conformance passes.

 
***
 
# 95. Removal Version
 
The earliest removal version MUST be explicit.
 
Example:
 
```text
Deprecated since: 1.4.0
Earliest removal: 2.0.0
```
 
The feature MUST NOT be removed before the declared earliest removal version except through the documented security/correctness emergency process.
 
***
 
# 96. Removal Behavior
 
When removed:
 
```text
removed source
    ↓
deterministic diagnostic
```
 
The compiler MUST NOT:
 
- silently reinterpret it;
- silently migrate it;
- silently ignore it;
- execute it under another semantic meaning.

 
Automatic migration MAY be offered as a separate tooling action.
 
***
 
# 97. Historical Records Are Immutable
 
Once a deprecation record has been published for a released language version, its identity and historical meaning MUST remain stable.
 
Corrections MAY clarify:
 
- documentation;
- diagnostics;
- migration instructions;
- implementation details.

 
They MUST NOT rewrite history in a way that changes what an already-released language version meant.
 
***
 
# 98. No Unnecessary File Renaming
 
This policy MUST integrate with the repository's existing filenames.
 
In particular, it MUST retain:
 
```text
grammar/compatibility/deprecated.md
grammar/compatibility/migrations.md
grammar/compatibility/versions.md
grammar/compatibility/compatibility-matrix.md
grammar/Zamani.g4
grammar/grammar.md
grammar/Zamani-Grammar.md
grammar/DESIGN.md
```
 
unless a separately approved repository-wide architectural change establishes a compelling reason otherwise.
 
No duplicate replacement file should be created merely to rename an existing authority.
 
***
 
# 99. Feature Completion Contract
 
A deprecation is complete only when the feature has a traceable path through:
 
```text
feature identity
      ↓
deprecation record
      ↓
language version
      ↓
canonical syntax
      ↓
lexer
      ↓
parser
      ↓
AST
      ↓
semantic normalization
      ↓
canonical IR
      ↓
compiler
      ↓
runtime
      ↓
target
```
 
and corresponding tests.
 
For quantum features:
 
```text
feature
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
QEC / resilience / ZQN
      ↓
HAL
      ↓
target
```
 
No compatibility feature is complete merely because its source spelling has been removed.
 
***
 
# 100. Repository Integration Checklist
 
Before marking a feature deprecated, verify:
 
## Specification
 
- &#91; &#93; feature has a stable identity;
- &#91; &#93; semantic meaning is documented;
- &#91; &#93; deprecation reason is documented;
- &#91; &#93; replacement is documented where applicable;
- &#91; &#93; language version is defined.

 
## Grammar
 
- &#91; &#93; `Zamani.g4` status is known;
- &#91; &#93; modular grammar status is known;
- &#91; &#93; no duplicate grammar authority exists;
- &#91; &#93; deprecated syntax remains deterministic while supported.

 
## Lexer
 
- &#91; &#93; token behavior is known;
- &#91; &#93; keyword/operator collisions are resolved;
- &#91; &#93; lexical migration is defined if necessary.

 
## Parser
 
- &#91; &#93; parser behavior is deterministic;
- &#91; &#93; negative cases are covered.

 
## AST
 
- &#91; &#93; AST mapping is known;
- &#91; &#93; no unnecessary legacy AST hierarchy exists;
- &#91; &#93; source provenance is preserved where required.

 
## Semantics
 
- &#91; &#93; historical meaning is defined;
- &#91; &#93; semantic normalization is defined;
- &#91; &#93; semantic compatibility is classified.

 
## IR
 
- &#91; &#93; current canonical IR mapping exists;
- &#91; &#93; no unnecessary legacy IR exists;
- &#91; &#93; quantum features reach `quantum::ir`.

 
## Compiler/runtime
 
- &#91; &#93; consumers are audited;
- &#91; &#93; backend implications are known;
- &#91; &#93; runtime implications are known.

 
## Compatibility
 
- &#91; &#93; `versions.md` is consistent;
- &#91; &#93; `migrations.md` contains the migration;
- &#91; &#93; compatibility matrix is updated;
- &#91; &#93; dialect compatibility is updated if applicable.

 
## Testing
 
- &#91; &#93; positive tests;
- &#91; &#93; diagnostic tests;
- &#91; &#93; migration tests;
- &#91; &#93; negative tests;
- &#91; &#93; boundary tests;
- &#91; &#93; scalability tests;
- &#91; &#93; determinism tests;
- &#91; &#93; cross-domain tests.

 
## Scalability
 
- &#91; &#93; no universal capacity limit was introduced;
- &#91; &#93; no `MAX_*` language limit was introduced;
- &#91; &#93; no fixed hardware replacement was introduced;
- &#91; &#93; resource requirements remain semantic;
- &#91; &#93; capability requirements remain semantic.

 
## Safety
 
- &#91; &#93; Rust 1.97/1.97.1 compatibility considered;
- &#91; &#93; Rust 2021 considered;
- &#91; &#93; no `unsafe` requirement;
- &#91; &#93; no network-dependent compiler behavior;
- &#91; &#93; deterministic implementation behavior.

 
***
 
# 101. Definition of Done
 
A deprecation is **DONE** only when:
 
```text
Identity
   +
Reason
   +
Version
   +
Replacement
   +
Migration
   +
Diagnostics
   +
Grammar
   +
Lexer
   +
Parser
   +
AST
   +
Semantics
   +
IR
   +
Compiler
   +
Runtime
   +
Dialect
   +
Tests
   +
Compatibility Matrix
   +
Scalability Audit
   +
Determinism Audit
```
 
are all resolved as applicable.
 
A feature MUST NOT be declared deprecated merely because a developer intends to replace it later.
 
***
 
# 102. Production Invariant
 
The following invariant is mandatory:
 
> **Deprecation changes the lifecycle of a representation; it does not silently reduce the semantic capabilities of the Zamani language.**

 
Therefore:
 
```text
deprecated syntax
        ↓
current semantics
        ↓
current canonical IR
        ↓
current compiler architecture
```
 
is preferred over:
 
```text
deprecated syntax
        ↓
legacy compiler
        ↓
legacy IR
        ↓
legacy runtime
```
 
***
 
# 103. POCO-REAF Final Invariant
 
The deprecation system MUST preserve:
 
```text
Program
   ↓
portable semantics
   ↓
canonical representation
   ↓
target-independent compilation
   ↓
target adaptation
   ↓
actual resources
```
 
It MUST NOT turn:
 
```text
today's hardware
```
 
into:
 
```text
tomorrow's language limitation
```
 
A language construct may be deprecated because its representation is obsolete, ambiguous, insecure, redundant, or semantically superseded.
 
It MUST NOT be deprecated merely because hardware, backend implementations, or current resource availability changed.
 
***
 
# 104. Final Architecture
 
The complete deprecation architecture is:
 
```text
                    Zamani Source
                         │
                         ▼
              Effective Language Version
                         │
                         ▼
                Deprecation Resolution
                         │
             ┌───────────┴───────────┐
             │                       │
        supported               removed
             │                       │
             ▼                       ▼
       normal parsing           diagnostic
             │
             ▼
            AST
             │
             ▼
     Semantic Normalization
             │
             ▼
      Canonical Semantic Model
             │
       ┌─────┴──────────┐
       │                │
       ▼                ▼
 Classical          Quantum
   IR              quantum::ir
       │                │
       └───────┬────────┘
               ▼
          Optimization
               │
       ┌───────┼────────┐
       ▼       ▼        ▼
    Routing Scheduling QEC/ZQN
       │       │        │
       └───────┼────────┘
               ▼
              HAL
               │
               ▼
       Target Realization
               │
       ┌───────┼──────────┐
       ▼       ▼          ▼
      CPU     GPU       FPGA/QPU
       │       │          │
       └───────┼──────────┘
               ▼
        Available Resources
```
 
The central invariant is:
 
> **Zamani deprecates syntax and representations when necessary, while preserving the strongest possible stable semantic contract.**

 
That contract remains independent of the number of CPUs, GPUs, FPGAs, QPUs, nodes, threads, qubits, tensor dimensions, memory capacity, network participants, or other physical resources.
 
The language therefore remains capable of scaling from the smallest supported realization to arbitrarily large realizations subject only to the actual resources and capabilities required by the program.
 
**POCO-REAF remains the governing objective:**
 
> **Program Once → Compile Once → Run Everywhere, Anywhere, Forever.**