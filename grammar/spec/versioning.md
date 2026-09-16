Zamani Versioning Specification

Path: "grammar/spec/versioning.md"
Status: Normative cross-layer specification contract
Language: Zamani
Scope: Language-version resolution, grammar-version conformance, feature evolution, compatibility classification, dialect/version integration, AST/semantic/IR compatibility, generated artifacts, migration, reproducibility, and long-term POCO-REAF
Implementation baseline: Rust 1.97 / Rust 1.97.1
Edition: Rust 2021
Safety: Safe Rust only; "unsafe" Rust is prohibited
Primary objective: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)

---

1. Purpose

This document defines the cross-layer versioning contract for Zamani.

It does not replace the language-level version specification in:

grammar/specification/language-version.md

and it does not replace release/deprecation policy in:

grammar/compatibility/versions.md
grammar/compatibility/migrations.md
grammar/compatibility/deprecated.md

Instead, this document defines how those contracts connect to the actual compiler and grammar implementation.

The objective is to ensure that versioning remains coherent across:

Zamani source
    ↓
language version
    ↓
lexical contract
    ↓
canonical grammar
    ↓
Rust lexer
    ↓
ANTLR parser
    ↓
frontend AST
    ↓
semantic analysis
    ↓
canonical semantic model
    ↓
canonical IR
    ↓
compiler
    ↓
runtime
    ↓
target realization

Versioning MUST therefore be treated as a language-wide contract rather than merely a version-number parser.

---

2. Authority and ownership

Zamani currently contains several version-related documents and grammar components.

They MUST NOT become competing authorities.

The ownership model is:

grammar/specification/language-version.md
        │
        │ owns language-level version semantics
        ▼
grammar/spec/versioning.md
        │
        │ owns cross-layer implementation/conformance rules
        ▼
grammar/spec/compatibility.md
        │
        │ owns compatibility relationships
        ▼
grammar/compatibility/versions.md
        │
        │ owns release/version compatibility policy
        ├───────────────┐
        ▼               ▼
migrations.md      deprecated.md
        │
        ▼
grammar/Zamani.g4
        │
        ▼
lexer / parser / AST / semantic analysis / IR

2.1 "grammar/specification/language-version.md"

Owns:

- the meaning of a Zamani language version;
- source-level version semantics;
- version classification;
- language-version evolution;
- language-version selection;
- language-level compatibility principles.

It is the language specification authority for version semantics.

2.2 "grammar/spec/versioning.md"

Owns:

- cross-layer versioning contracts;
- implementation conformance;
- version propagation;
- version metadata;
- compatibility checks between compiler layers;
- grammar/lexer/parser/AST/semantic/IR version relationships;
- versioned artifacts;
- feature traceability;
- deterministic version resolution;
- version validation requirements;
- repository-wide version consistency.

This document MUST NOT redefine the meaning of the language version.

2.3 "grammar/spec/compatibility.md"

Owns:

- compatibility categories;
- compatibility between implementation layers;
- conformance relationships;
- compatibility testing;
- source/AST/IR/target compatibility.

2.4 "grammar/compatibility/versions.md"

Owns:

- release policy;
- supported version ranges;
- compatibility windows;
- release-level version classification.

2.5 "grammar/compatibility/migrations.md"

Owns:

- migration procedures;
- automated migration;
- source transformations;
- version transition guidance.

2.6 "grammar/compatibility/deprecated.md"

Owns:

- deprecation lifecycle;
- deprecated syntax;
- removal schedules;
- replacements.

2.7 "grammar/Zamani.g4"

Owns:

- canonical ANTLR composition;
- source syntax composition;
- language-version syntax only to the extent specified by the normative specification.

It MUST NOT independently define version semantics.

2.8 "grammar/grammar.md"

Owns:

- implementation-conformance documentation.

It MUST identify which language-version features are:

- specified;
- implemented;
- partially implemented;
- experimental;
- deprecated;
- unsupported.

It MUST NOT become a second normative version specification.

2.9 "grammar/Zamani-Grammar.md"

Owns:

- historical;
- explanatory;
- aspirational;
- proposed;
- extended language-design material.

Its version-related material MUST NOT silently introduce syntax into a stable language version.

---

3. Core principle

The fundamental invariant is:

«A version identifies a semantic contract, not a machine.»

Versioning MUST describe language and software contracts.

Versioning MUST NOT permanently encode current limitations of:

- CPUs;
- cores;
- threads;
- GPUs;
- FPGAs;
- ASICs;
- QPUs;
- physical qubits;
- logical qubits;
- memory;
- storage;
- vector lanes;
- registers;
- network nodes;
- distributed services;
- accelerators;
- topology;
- timing hardware;
- calibration;
- device counts.

For example, a language version MUST NOT mean:

Zamani 1.0 = maximum 32 qubits

or:

Zamani 1.0 = maximum 64 CPU cores

or:

Zamani 1.0 = 24 GB GPU memory

Such relationships are prohibited.

Actual limits belong to:

resource availability
capability negotiation
target constraints
deployment policy
runtime policy
implementation limits

---

4. Versioning layers

Zamani MUST distinguish at least these version domains:

Language Version
Grammar Contract Version
Lexer Contract Version
Parser Contract Version
AST Contract Version
Semantic Contract Version
IR Contract Version
Dialect Version
Package/Module Version
API Version
ABI Version
Compiler Version
Runtime Version
Target Contract Version
Artifact Format Version
Tooling Contract Version

These versions are related but MUST NOT be conflated.

For example:

Zamani language 1.2.0
compiler 0.9.4
quantum::ir 3.1
runtime 2.7
artifact format 1.4

is valid as a conceptual model.

A compiler version MUST NOT automatically imply a language-version change.

A runtime version MUST NOT automatically imply a source-language change.

A hardware generation MUST NOT automatically imply a source-language change.

---

5. Language version versus implementation version

The following are independent:

Language Version
Compiler Version
Runtime Version
IR Version
Target Version

A compiler implementation may support several language versions:

Compiler
 ├── Zamani 1.0
 ├── Zamani 1.1
 └── Zamani 1.2

A compiler upgrade MUST NOT silently migrate source code to a different language version.

The effective language version MUST be resolved explicitly and deterministically.

---

6. Canonical language-version form

The canonical language version is:

MAJOR.MINOR.PATCH

Examples:

1.0.0
1.1.0
1.1.1
2.0.0

Pre-release identifiers MAY be used:

1.0.0-alpha
1.0.0-beta
1.0.0-rc.1

Build metadata MAY be used where the language-level version model permits it:

1.0.0+build.42

Build metadata MUST NOT change semantic version precedence.

---

7. Version-component requirements

Language versions MUST NOT impose artificial numeric limits on version components.

The implementation MUST NOT define rules such as:

MAX_MAJOR
MAX_MINOR
MAX_PATCH
MAX_VERSION

The representation MAY have practical resource limits imposed by the host implementation, but such limits are implementation constraints rather than language semantics.

The compiler MUST NOT reject a language version merely because a language-version component exceeds an arbitrary language-level constant.

---

8. Four-component versions

The existing "grammar/core/versioning.g4" contains a "versionRevision" production in addition to major/minor/patch.

That does not automatically make:

1.2.3.4

a fourth component of the canonical Zamani language version.

The canonical language version remains:

MAJOR.MINOR.PATCH

If a fourth or additional component is required for:

- implementation versions;
- artifact versions;
- vendor versions;
- target versions;
- build identifiers;

it MUST be represented by the corresponding version domain rather than silently changing the language-version contract.

This prevents:

language version

from becoming an unrestricted implementation-version grammar.

---

9. Version precedence

When versions are compared, the comparison algorithm MUST be defined by the applicable version domain.

For canonical language versions:

MAJOR
MINOR
PATCH

are compared in that order.

Pre-release versions MUST follow the language's declared pre-release ordering rules.

Build metadata MUST NOT change semantic precedence.

Version comparison MUST be:

- deterministic;
- independent of host architecture;
- independent of target hardware;
- independent of hash iteration order;
- independent of network state.

---

10. Version syntax versus version semantics

The grammar recognizes structure.

Semantic analysis determines meaning.

Therefore:

grammar
    ↓
version syntax
    ↓
AST
    ↓
version semantic validation
    ↓
compatibility evaluation

The parser MUST NOT decide:

"1.2.0 is compatible with dialect X"

The parser only establishes:

version expression = 1.2.0

Compatibility resolution belongs to semantic analysis/tooling.

---

11. Version ranges

Version ranges MUST be semantically open-ended.

Examples may include:

>= 1.0.0
< 2.0.0
>= 1.0.0 and < 2.0.0
1.0.0 .. 2.0.0
1.0.0 ..= 2.0.0

The exact concrete syntax is owned by:

grammar/core/versioning.g4

The semantic interpretation is owned by version analysis.

No fixed number of constraints may be imposed.

For example, this MUST be possible conceptually:

>= 1.0.0
and < 5.0.0
and != 2.1.0
and != 3.4.0
and ...

subject only to available compiler resources.

---

12. Version constraint satisfiability

A parser MUST NOT determine whether constraints are satisfiable.

For example:

>= 2.0.0
and < 1.0.0

is syntactically representable but semantically unsatisfiable.

The semantic/version-resolution layer MUST report the conflict deterministically.

The diagnostic SHOULD identify:

- the conflicting constraints;
- their source spans;
- the affected package/dialect/language contract;
- the effective version;
- the reason for failure.

---

13. Version resolution

The effective language version MUST be resolved before version-dependent semantic interpretation.

The conceptual pipeline is:

Source
  ↓
Source-unit discovery
  ↓
Version discovery
  ↓
Version syntax validation
  ↓
Version normalization
  ↓
Version compatibility validation
  ↓
Version-specific lexical policy
  ↓
Version-specific parsing
  ↓
AST
  ↓
Semantic analysis

A compiler MUST NOT:

1. parse source under an unknown version;
2. perform semantic interpretation;
3. discover a version later;
4. reinterpret the already-created AST.

Version selection must happen early enough to make version-sensitive parsing deterministic.

---

14. Version-source precedence

The implementation MUST define one deterministic precedence order for obtaining the effective language version.

The recommended order is:

1. explicit source declaration
2. explicit project/workspace declaration
3. package/module declaration
4. compiler-selected compatibility default
5. no implicit version

The exact repository policy MUST be recorded in:

grammar/specification/language-version.md
grammar/compatibility/versions.md

An explicit source declaration MUST NOT be silently overridden by a lower-precedence configuration.

If multiple declarations exist at the same precedence level and conflict, compilation MUST fail deterministically.

---

15. Missing language version

The compiler MUST define a policy for source without an explicit version.

Possible policies include:

legacy default
project-defined default
manifest-defined default
mandatory explicit version

The implementation MUST NOT choose different behavior based on:

- compiler host;
- operating system;
- CPU;
- target hardware;
- environment variable;
- network response;
- installation location.

The selected policy MUST be deterministic.

---

16. Version normalization

The compiler MAY normalize version representations internally.

For example:

"1.0.0"
1.0.0

may normalize to the same semantic version where the language contract permits both forms.

Normalization MUST preserve:

- semantic identity;
- source span;
- diagnostics information;
- original spelling where required for tooling.

Normalization MUST NOT silently transform a semantically distinct version into another version.

---

17. Version declaration uniqueness

A source unit MUST NOT contain conflicting language-version declarations.

For example:

language 1.0.0;
language 2.0.0;

MUST produce a deterministic diagnostic.

If multiple declarations are allowed for compatibility reasons, their combination rule MUST be explicitly specified.

The compiler MUST NOT silently choose one.

---

18. Language-version declarations and hardware

Language-version declarations MUST remain target-independent.

Invalid examples as universal language semantics include:

language 1.0 requires 32 qubits;
language 1.0 requires 8 GPUs;
language 1.0 requires 64 cores;

Resource requirements belong to resource/capability contracts.

A language version may define the meaning of a resource expression, but it MUST NOT turn a temporary resource capacity into a permanent language limit.

---

19. Feature versioning

Every language feature MUST have an explicit lifecycle.

Recommended statuses:

PROPOSED
DESIGNED
SPECIFIED
IMPLEMENTED
EXPERIMENTAL
STABLE
DEPRECATED
REMOVED
RESERVED

Status MUST NOT be inferred merely from the existence of:

- a grammar rule;
- a parser branch;
- a documentation section;
- an example;
- a token.

A feature is stable only when its complete implementation contract is satisfied.

---

20. Feature completion contract

A feature MUST NOT be marked "STABLE" until all applicable layers are accounted for:

Feature identity
    ↓
Language specification
    ↓
Lexical contract
    ↓
Grammar
    ↓
Parser
    ↓
AST
    ↓
Semantic model
    ↓
Canonical IR
    ↓
Compiler
    ↓
Runtime
    ↓
Tooling
    ↓
Positive tests
    ↓
Negative tests
    ↓
Boundary tests
    ↓
Scalability tests
    ↓
Compatibility tests
    ↓
Determinism tests

For target-dependent domains, also:

Capability validation
Resource validation
Target lowering
Target diagnostics

---

21. Feature manifest integration

Where feature manifests are used under:

grammar/specification/features/

each feature SHOULD identify:

id:
name:
status:
introduced:
stabilized:
deprecated:
removed:
language_version:
syntax:
grammar:
lexer_tokens:
ast_nodes:
semantic_rules:
ir_mapping:
compiler_consumers:
runtime_consumers:
capabilities:
resource_requirements:
positive_tests:
negative_tests:
boundary_tests:
scalability_tests:
determinism_tests:
compatibility_tests:
migration:
hard_coding_policy:

The manifest MUST NOT become another semantic authority.

It is a machine-readable index of the authoritative contracts.

---

22. Compatibility classification

Every language change MUST be classified.

22.1 Fully compatible

Examples:

- compiler optimization;
- diagnostic improvements;
- new backend;
- new simulator;
- new hardware target;
- new runtime implementation;
- internal refactoring.

These MUST preserve language semantics.

22.2 Compatible extension

Examples:

- new contextual syntax;
- new portable abstraction;
- new standard attribute;
- new capability expression;
- new domain construct that cannot reinterpret existing valid programs.

22.3 Version-gated change

Examples:

- new globally reserved keyword;
- changed operator precedence;
- changed literal interpretation;
- changed ownership semantics;
- changed quantum semantics.

These require explicit version handling.

22.4 Breaking change

Examples:

- removing stable syntax;
- changing the meaning of stable syntax;
- incompatible type-system semantics;
- incompatible effect semantics;
- incompatible resource semantics.

Breaking changes require the applicable major-version policy.

---

23. Patch releases

A patch release MUST NOT change the meaning of valid stable source.

Patch-level changes MAY include:

- bug fixes;
- specification clarifications;
- diagnostics;
- documentation corrections;
- implementation fixes;
- conformance corrections;
- generated-reference corrections.

If a correction changes the meaning of a previously valid program, it is not merely a patch-level correction and MUST undergo compatibility review.

---

24. Minor releases

A minor release SHOULD add functionality without changing the meaning of existing stable programs.

Before adding syntax, validation MUST check:

- lexical conflicts;
- keyword collisions;
- parser ambiguity;
- precedence;
- associativity;
- AST representation;
- semantic interpretation;
- dialect collisions;
- macro interactions;
- tooling impact.

---

25. Major releases

A major language version MAY introduce intentionally incompatible changes.

A major release MUST document:

- every breaking change;
- affected constructs;
- semantic difference;
- migration path;
- compatibility status;
- deprecation/removal relationship;
- compiler support;
- test coverage.

A major version MUST NOT be created merely because:

- a compiler was rewritten;
- Rust changed;
- a new backend was added;
- a new GPU appeared;
- a new QPU appeared;
- an FPGA generation changed;
- scheduling improved;
- routing improved;
- QEC improved;
- ZQN changed internally;
- HAL was refactored.

Those are implementation/target concerns unless they alter the language contract.

---

26. Rust version independence

Zamani language versioning MUST remain independent of Rust versioning.

The repository implementation baseline is:

Rust 1.97
Rust 1.97.1 production validation
Rust 2021
safe Rust only

Rust MUST NOT appear as a Zamani source-language version.

For example:

Zamani 1.2.0

and:

Rust compiler 1.97.1

are independent values.

The Rust compiler version MUST NOT be encoded into the language grammar.

The implementation MUST NOT use "unsafe" Rust.

---

27. Grammar-version independence

A grammar representation may change without changing the language.

For example:

Zamani language 1.2.0

may continue to be represented by a revised:

grammar/Zamani.g4

provided the language semantics remain compatible.

Therefore:

language version != grammar file revision

A grammar refactor MUST NOT automatically constitute a language-version change.

---

28. Lexer compatibility

Version-sensitive lexical behavior MUST be governed by:

grammar/spec/lexical.md
grammar/lexer/
src/lexer.rs

The Rust lexer and ANTLR lexical representation MUST agree for the applicable language version.

Version-sensitive lexical features may include:

- keywords;
- contextual keywords;
- operators;
- delimiters;
- literal forms;
- escapes;
- Unicode rules;
- comments;
- interpolation;
- numeric literal forms.

A new keyword MUST undergo identifier-collision analysis.

---

29. Keyword evolution

The preferred order for new language vocabulary is:

existing syntax
    ↓
compositional syntax
    ↓
contextual keyword
    ↓
scoped keyword
    ↓
global reserved keyword

A new globally reserved keyword SHOULD be avoided when a compositional representation can express the same semantics without breaking identifiers.

This is particularly important for:

- quantum operations;
- mathematical operations;
- AI operations;
- hardware capabilities;
- networking protocols;
- vendor operations;
- future domains.

---

30. Versioning and identifiers

A versioning mechanism MUST NOT unnecessarily reserve identifiers.

The following must remain distinguishable:

language keyword
contextual keyword
identifier
reserved identifier
dialect keyword
future-reserved identifier

The canonical classification belongs to the lexical specification.

---

31. AST compatibility

Every accepted version-sensitive construct MUST have a corresponding AST representation.

The invariant is:

accepted syntax
    ↓
AST representation

No information required for semantic correctness may disappear.

The AST MUST preserve, where applicable:

- source spans;
- version declaration;
- version expression;
- constraints;
- dialect association;
- package/module association;
- attributes;
- semantic modifiers.

The AST MUST NOT resolve target hardware while representing a language version.

---

32. Semantic compatibility

Semantic analysis owns version meaning after parsing.

It MUST determine:

- effective language version;
- compatibility;
- feature availability;
- deprecation;
- removal;
- feature gates;
- dialect compatibility;
- package compatibility;
- compiler support;
- runtime support where applicable.

A semantic version conflict MUST produce a structured diagnostic.

---

33. Canonical IR compatibility

Source-language versioning and IR versioning are separate contracts.

The pipeline is:

Zamani source
    ↓
Frontend AST
    ↓
Semantic analysis
    ↓
Canonical semantic model
    ↓
Canonical IR

An IR revision MUST NOT silently alter source-language semantics.

An IR may evolve while a source language remains stable if:

- the semantic contract is preserved;
- migration/lowering is defined;
- compatibility is tested.

---

34. Quantum IR boundary

For quantum computation:

Zamani source
    ↓
frontend AST
    ↓
semantic quantum model
    ↓
quantum::ir

"quantum::ir" remains the canonical quantum semantic boundary.

Versioning MUST NOT create a second frontend quantum IR.

Quantum-language versioning MUST NOT become coupled to:

- physical qubit count;
- native gate set;
- QPU vendor;
- topology;
- calibration;
- physical qubit IDs;
- scheduling implementation;
- routing implementation;
- QEC implementation;
- ZQN implementation;
- HAL implementation.

A quantum language version describes quantum semantics.

Hardware capability versions describe hardware.

---

35. Quantum feature evolution

A new hardware-native operation does not automatically require a new Zamani language version.

For example, a new QPU may introduce:

new_native_operation

That should ordinarily be represented through:

operation identity
capability
dialect
target capability
lowering rule

rather than permanently adding a new core-language keyword.

The source language MUST remain open-world where semantics permit.

---

36. Classical feature evolution

New classical capabilities MUST NOT require language-version changes merely because hardware gains:

- wider vectors;
- more cores;
- more cache;
- more memory;
- new accelerators;
- new instruction sets.

A stable Zamani program should express computation independently of the exact hardware realization.

Target-specific instructions MAY exist through explicitly defined interoperability/dialect mechanisms.

---

37. HDL and hardware versioning

HDL and hardware descriptions require separate version domains.

The language version describes:

what the Zamani source means

The hardware/target contract describes:

what a particular target can realize

Changing:

- FPGA family;
- ASIC process;
- timing characteristics;
- available resources;
- device topology;
- memory architecture;

MUST NOT silently redefine portable Zamani semantics.

Parameterized hardware descriptions MUST remain parameterized.

No version contract may introduce universal limits such as:

MAX_WIDTH
MAX_PORTS
MAX_MODULES
MAX_MEMORIES
MAX_PIPELINE_STAGES

unless such a bound is genuinely part of the semantic specification rather than an implementation restriction.

---

38. Resource and capability compatibility

The following concepts MUST remain separate:

language requirement
capability requirement
resource requirement
preference
hint
target decision

Example:

requires capability("quantum.mid_circuit_measurement")

is not equivalent to:

use qpu-17

Similarly:

requires qubits >= n

is not equivalent to:

map q[0] -> physical_qubit(0)

Versioning MUST preserve this distinction.

---

39. Target compatibility

A target MAY be unable to execute a program.

That is not necessarily language incompatibility.

The compiler/runtime MUST distinguish:

language incompatibility
dialect incompatibility
semantic incompatibility
IR incompatibility
capability insufficiency
resource insufficiency
target incompatibility
runtime incompatibility

These conditions require different diagnostics.

---

40. POCO-REAF

Versioning MUST support:

Program Once
Compile Once
Run Everywhere
Anywhere
Forever

without claiming that one immutable native machine-code binary can execute natively on every future architecture.

The durable artifact model should instead preserve:

source semantics
language version
dialect versions
capability requirements
resource requirements
canonical semantic information
IR contract
provenance
compatibility metadata

This allows future systems to validate and re-lower the artifact without requiring a source rewrite.

---

41. Program Once

The source program SHOULD express:

- computation;
- semantics;
- types;
- effects;
- resource intent;
- capability requirements;
- correctness requirements;
- portability constraints.

It SHOULD NOT unnecessarily encode:

- physical device IDs;
- fixed topology;
- temporary resource limits;
- vendor-specific implementation choices.

---

42. Compile Once

A compilation artifact MAY contain:

- canonical IR;
- language version;
- semantic metadata;
- capability requirements;
- resource requirements;
- dialect versions;
- target-independent optimization information;
- provenance.

A compiled artifact MUST identify enough information for a future compatible system to determine whether it remains valid.

---

43. Run Everywhere

A runtime MAY select a target realization according to:

capabilities
resources
constraints
availability
policy
security
correctness

Version resolution MUST happen before target realization.

The runtime MUST NOT reinterpret language semantics merely because the target differs.

---

44. Run Anywhere

Version metadata MUST be independent of deployment location.

The same semantic artifact may be deployed to:

embedded
desktop
server
cluster
cloud
edge
accelerator
FPGA
GPU
QPU
simulator
emulator
future computational substrate

provided the target satisfies the required contracts.

---

45. Run Forever

"Forever" means semantic continuity and recoverable compatibility, not that implementation details never change.

Long-lived artifacts SHOULD preserve:

language version
artifact format version
IR version
dialect versions
compiler provenance
semantic requirements
capability requirements
resource requirements
migration metadata

Future tooling can then:

validate
migrate
re-lower
recompile
reinterpret

according to explicit compatibility rules.

---

46. Dialect versioning

Dialect versioning MUST be subordinate to the core language version model.

The repository already contains:

grammar/dialects/versioning.g4

Its role MUST remain an adapter around the canonical versioning grammar rather than a second version language.

A dialect may have:

dialect identity
dialect version
compatibility requirements
capabilities
semantic extensions
grammar extensions

but it MUST NOT redefine:

core version syntax
core version comparison
core version precedence

---

47. Dialect identity versus dialect version

These are distinct:

dialect identity = what extension is this?
dialect version  = which contract revision is this?

For example:

quantum::dynamic

identifies a dialect.

Its version identifies the contract of that dialect.

Adding another dialect MUST NOT require editing the central versioning grammar.

---

48. Dialect isolation

A dialect MUST declare:

- identity;
- version;
- owner/namespace;
- language-version compatibility;
- syntax extensions;
- semantic extensions;
- AST mapping;
- IR mapping;
- capabilities;
- compatibility;
- migration policy.

A dialect MUST NOT silently change the semantics of core Zamani syntax.

---

49. Package and module versions

Package/module versions are independent of language versions.

A package may declare:

package version
language compatibility
dependency versions
dialect versions
API versions
ABI versions

A package version MUST NOT be interpreted as a language version.

Dependency resolution MUST NOT silently select a semantically incompatible language contract.

---

50. API and ABI versions

API and ABI versions MUST be independent.

Changing a public API does not necessarily change the language.

Changing an ABI does not necessarily change source semantics.

ABI compatibility may depend on:

- target;
- calling convention;
- data representation;
- platform;
- architecture.

Those are separate contracts.

---

51. Compiler compatibility

Every production compiler MUST expose its supported Zamani language-version range.

Conceptually:

compiler
    supports:
        Zamani 1.0.x
        Zamani 1.1.x
        Zamani 1.2.x

Unsupported versions MUST produce structured diagnostics.

The compiler MUST NOT silently treat:

Zamani 2.x

as:

Zamani 1.x

unless an explicit compatibility/migration mode exists.

---

52. Forward compatibility

A compiler MUST NOT claim future-version support merely because future syntax happens to parse.

For an unsupported future language version:

unsupported language version

MUST be reported.

Unknown syntax MUST NOT be silently interpreted using older semantics.

Explicit extension mechanisms MAY permit forward-compatible constructs, but their semantics must be defined.

---

53. Backward compatibility

Within a compatible language-version range:

valid stable source

MUST retain its specified meaning.

Exceptions must be explicitly documented for:

- experimental features;
- implementation-defined behavior;
- target-defined behavior;
- dialect-defined behavior;
- deprecated constructs;
- unspecified behavior.

---

54. Experimental features

Experimental features MUST have:

- unique identity;
- owner;
- specification;
- status;
- version introduction;
- feature gate where necessary;
- AST contract;
- semantic contract;
- IR contract;
- tests;
- migration policy.

An experimental feature MUST NOT silently become stable.

A stable compiler MUST NOT accidentally expose experimental syntax merely because the parser contains the rule.

---

55. Deprecation

A deprecated feature MUST identify:

introduced version
deprecated version
replacement
migration path
earliest removal version

Deprecation MUST be coordinated with:

grammar/compatibility/deprecated.md
grammar/compatibility/migrations.md
grammar/compatibility/versions.md

Removal MUST NOT silently reinterpret the old syntax as another construct.

---

56. Reserved syntax

Reserved syntax is not implemented syntax.

A reserved spelling MUST be distinguishable from:

stable syntax
experimental syntax
ordinary identifier
dialect syntax

Encountering reserved syntax where a construct is expected SHOULD produce a deterministic diagnostic.

---

57. Version migration

Every breaking change MUST have a migration strategy where technically possible.

Migration documentation MUST identify:

old version
new version
affected feature
old syntax
new syntax
semantic difference
automatable transformation
manual transformation
diagnostic
test migration

Migration tooling MUST NOT silently change program semantics.

---

58. Source-preserving migration

Where possible, migrations SHOULD preserve:

- source semantics;
- comments;
- source spans where practical;
- formatting;
- attributes;
- metadata;
- explicit resource/capability requirements.

If exact source preservation is impossible, the migration tool MUST clearly report the transformation.

---

59. Versioned generated artifacts

Generated artifacts MUST identify their originating contracts where relevant.

Examples:

generated parser
generated lexer
generated AST bindings
generated documentation
generated grammar reference
generated IR
generated compatibility metadata

Generated artifacts MUST NOT become independent authorities.

The generation direction MUST remain explicit:

normative specification
        ↓
canonical grammar
        ↓
generated representation

not:

generated file
        ↓
normative language

unless explicitly promoted through the repository's specification process.

---

60. "grammar/grammar.md"

"grammar/grammar.md" MUST identify:

language version
grammar representation version
implementation status
generated/manual status
source authority

It MUST distinguish:

specified
implemented
experimental
deprecated
removed
reserved

A grammar construct appearing in "grammar.md" MUST NOT automatically mean that it is stable language syntax.

---

61. "grammar/Zamani-Grammar.md"

"Zamani-Grammar.md" may contain broader future language concepts.

Its version-sensitive material MUST be classified.

Recommended status markers:

STABLE
SPECIFIED
IMPLEMENTED
EXPERIMENTAL
PROPOSED
HISTORICAL
DEPRECATED
REMOVED
RESERVED

A feature MUST NOT enter a stable language version merely because it appears in this document.

---

62. "grammar/Zamani.g4"

"Zamani.g4" MUST remain the canonical ANTLR composition root.

Its versioning rules MUST:

- conform to this specification;
- conform to "grammar/specification/language-version.md";
- use the canonical versioning grammar;
- avoid duplicate version semantics;
- avoid hardware limits;
- avoid target-specific language versions;
- remain deterministic.

---

63. "grammar/core/versioning.g4"

"grammar/core/versioning.g4" owns syntax for:

- version declarations;
- version expressions;
- exact versions;
- ranges;
- constraints;
- references;
- channels;
- compatibility expressions.

It MUST NOT determine semantic compatibility.

It MUST NOT select hardware.

It MUST NOT encode resource limits.

It MUST NOT perform version resolution through parser actions.

The existing broad structure should therefore be retained only where it conforms to this contract.

---

64. Correction to "grammar/core/versioning.g4"

The following distinction MUST be enforced:

canonical language version
    = MAJOR.MINOR.PATCH

versus:

implementation/vendor/artifact version
    = implementation-defined contract

A fourth "versionRevision" component MUST NOT silently become part of the language-version SemVer contract.

If retained for compatibility, its semantic domain MUST be explicit.

---

65. Correction to "grammar/core/versioning.g4": open-world channels

The grammar currently permits identifier-based version channels.

That mechanism MUST NOT allow arbitrary identifiers to be silently interpreted as globally recognized language channels.

Semantic analysis MUST distinguish:

known channel
unknown channel
ordinary identifier/reference
dialect-defined channel

Unknown channels MUST produce a deterministic diagnostic unless an explicit extension mechanism permits them.

---

66. Correction to "grammar/core/versioning.g4": version strings

String versions MAY remain supported for compatibility with existing source.

However:

"1.0.0"

and:

1.0.0

MUST normalize to the same semantic version only when the applicable language-version rules say so.

Malformed version strings MUST be diagnosed semantically.

The parser MUST NOT silently accept arbitrary strings as valid language versions merely because they are strings.

---

67. "grammar/dialects/versioning.g4"

"grammar/dialects/versioning.g4" MUST remain an adapter around:

grammar/core/versioning.g4

It MUST NOT duplicate:

- version comparison;
- version ranges;
- version components;
- compatibility algorithms.

The current architectural intention of making dialect versioning an adapter is correct and must be preserved.

---

68. Versioning and source maps

Version-sensitive source maps MUST preserve:

source file
source span
language version
dialect version
generated source relationship

A diagnostic referring to a generated or migrated construct MUST be traceable back to the original source.

This is especially important for:

- macros;
- code generation;
- dialect lowering;
- generated HDL;
- generated quantum operations;
- interoperability formats.

---

69. Versioning and diagnostics

Version diagnostics MUST be structured.

At minimum, a diagnostic SHOULD identify:

diagnostic code
severity
source span
effective language version
requested version
supported version range
affected feature
reason
migration guidance

Example categories:

VERSION_UNKNOWN
VERSION_MALFORMED
VERSION_UNSUPPORTED
VERSION_CONFLICT
VERSION_RANGE_UNSATISFIABLE
VERSION_FEATURE_UNAVAILABLE
VERSION_DIALECT_INCOMPATIBLE
VERSION_IR_INCOMPATIBLE
VERSION_TARGET_INCOMPATIBLE
VERSION_MIGRATION_REQUIRED
VERSION_DEPRECATED
VERSION_REMOVED

Exact diagnostic identifiers belong to the diagnostic specification.

---

70. Determinism

Version resolution MUST be deterministic.

The same:

source
project configuration
compiler version
declared dependencies
declared dialects

MUST yield the same effective version result.

Version resolution MUST NOT depend on:

- hash-map iteration order;
- filesystem ordering;
- network timing;
- machine topology;
- thread scheduling;
- random values;
- locale;
- host architecture.

---

71. Reproducibility

A reproducible build SHOULD record:

language version
compiler version
grammar contract
dialect versions
dependency versions
IR version
artifact format
feature gates
target-independent semantic metadata

Reproducibility metadata MUST distinguish semantic inputs from incidental host information.

---

72. Version locks

A package/workspace MAY lock dependencies to compatible version ranges.

A lockfile MUST NOT silently change the language version.

Language-version changes require explicit compatibility evaluation.

---

73. Dependency resolution

Dependency resolution MUST distinguish:

package identity
package version
language compatibility
API compatibility
ABI compatibility
dialect compatibility
runtime compatibility
target compatibility

These are not interchangeable.

A package may be:

package-compatible

but:

language-incompatible

or:

target-incompatible

The resolver MUST preserve these distinctions.

---

74. Version conflicts

If two dependencies require incompatible language/dialect contracts, the compiler MUST produce a structured conflict.

For example:

dependency A requires language >= 1.2 < 2.0
dependency B requires language >= 2.0

MUST NOT be resolved by silently selecting one requirement.

The compiler MUST either:

- prove a valid compatibility relationship;
- apply an explicit compatibility mechanism;
- or report the conflict.

---

75. Cross-domain versioning

All Zamani domains use the same core versioning model.

This includes:

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
memory
concurrency
interoperability
macros
metaprogramming

A domain MUST NOT create an incompatible local version language.

---

76. Domain feature versions

A domain feature MAY have its own feature/dialect version.

For example:

quantum dialect version
HDL dialect version
AI dialect version

but the domain version MUST remain associated with the core Zamani language version.

The domain MUST declare compatibility rather than silently replacing the core language.

---

77. Quantum versioning

Quantum versioning MUST distinguish:

Zamani language version
quantum semantic version
quantum dialect version
quantum::ir version
QPU target version
QPU capability set

A new QPU generation MUST NOT automatically require a new Zamani language version.

A new quantum semantic rule may require a language/dialect version.

---

78. QEC, ZQN, HAL, routing, and scheduling

These systems have independent implementation contracts.

Versioning MUST preserve their boundaries.

QEC
    = error detection/correction

ZQN
    = fault/noise semantics

HAL
    = hardware capability/state abstraction

routing
    = physical realization

scheduling
    = timing/order/resource scheduling

optimization
    = implementation improvement

A change to one subsystem MUST NOT silently redefine language semantics.

Where a subsystem version affects source-visible behavior, the relevant semantic contract MUST explicitly record that dependency.

---

79. Resource scalability

Versioning MUST remain unbounded with respect to program/resource scale.

No language-version contract may impose:

MAX_QUBITS
MAX_CPUS
MAX_CORES
MAX_THREADS
MAX_GPUS
MAX_FPGAS
MAX_NODES
MAX_MEMORY
MAX_REGISTERS
MAX_VECTOR_WIDTH
MAX_TENSOR_DIMENSION
MAX_NETWORK_NODES
MAX_TIMELINES

A compiler may have implementation resource limits.

Such limits MUST be reported as implementation/resource errors, not encoded as language semantics.

---

80. Tiny-to-infinity requirement

The language-version system MUST work for programs ranging from:

one instruction
one value
one qubit
one processing element
one hardware module

through arbitrarily large programs and systems, limited only by:

available resources
implementation capacity
target capability
declared constraints

The versioning model itself MUST NOT introduce scaling ceilings.

---

81. No source rewrite for scaling

Changing from:

small machine

to:

large machine

MUST NOT require a language-version change merely because the machine is larger.

For example:

1 qubit

and:

n qubits

must remain governed by the same semantic language contract where their constructs are otherwise equivalent.

Likewise:

1 CPU

versus:

many CPUs

must not require a different language version merely because the target scale changes.

---

82. Versioning and optimization

Compiler optimization changes MUST NOT require language-version changes when semantics are preserved.

Examples:

constant folding
vectorization
parallelization
circuit optimization
gate decomposition
scheduling optimization
memory optimization
distributed optimization

may evolve independently.

An optimization MUST preserve the semantic contract unless an explicitly requested approximate/relaxed mode is part of the language.

---

83. Versioning and target lowering

Target lowering may change across compiler versions.

For example:

Zamani source
    ↓
quantum::ir
    ↓
QPU-A lowering

may later become:

Zamani source
    ↓
quantum::ir
    ↓
QPU-B lowering

without changing the source language.

Target lowering changes MUST NOT silently alter source semantics.

---

84. Versioning and interoperability

External formats MUST have independent versions.

Examples include:

OpenQASM
QIR
LLVM
MLIR
HDL formats
WASM
C ABI
C++
Python
Rust

Interoperability version != Zamani language version.

Adapters MUST declare compatibility.

The canonical Zamani semantic model remains the integration boundary.

---

85. Versioning and macros

Macro systems MUST record enough version context to ensure deterministic expansion.

A macro invocation MUST NOT silently expand according to a different language version than the source unit unless the macro contract explicitly defines that behavior.

Macro-generated syntax MUST be interpreted under an explicitly defined version context.

---

86. Versioning and metaprogramming

Compile-time code generation MUST preserve version context.

Generated code MUST NOT silently acquire a newer language version merely because it was generated by a newer compiler.

The generated artifact MUST identify its applicable language contract.

---

87. Versioning and examples

Examples MUST identify the applicable language version when their syntax is version-sensitive.

Examples MUST NOT be treated as normative unless explicitly promoted.

A stable example SHOULD be continuously compiled by conformance tests.

---

88. Versioning and tests

Every language-version change MUST update applicable tests.

Required categories:

lexical
syntax
AST
semantic
IR
compiler
runtime
positive
negative
boundary
scalability
determinism
compatibility
migration
diagnostics

Domain-specific additions:

quantum
HDL
hardware
AI
distributed
networking
security

where applicable.

---

89. Version compatibility tests

Compatibility tests MUST verify at least:

old stable source + compatible compiler
old stable source + newer compatible compiler
new source + old compiler
unsupported future source
deprecated source
removed source
conflicting versions
invalid versions
unsatisfiable ranges
dialect conflicts
package conflicts
IR-version conflicts

---

90. Negative version tests

Negative tests MUST include:

malformed version
missing version component
invalid separator
invalid pre-release identifier
invalid build metadata
conflicting declarations
unsupported version
future version
unsatisfiable range
unknown dialect version
incompatible dialect
incompatible package
incompatible IR

No negative test should rely on a machine-specific resource limit.

---

91. Boundary tests

Boundary tests MUST cover:

0
1
large representable version components
many constraints
many dialects
many dependencies
deep module graphs
multiple compatible versions
multiple incompatible versions
pre-release boundaries
release boundaries
major-version transitions

The tests MUST NOT encode arbitrary implementation maxima as language rules.

---

92. Scalability tests

Versioning scalability tests MUST demonstrate that the model can represent:

many version requirements
many dialects
many packages
large dependency graphs
large compatibility expressions
large version components
large generated artifacts

subject only to available resources.

Tests MUST distinguish:

implementation exhaustion

from:

language-level prohibition

---

93. Determinism tests

Given identical inputs:

source
configuration
dependencies
compiler
language-version policy

version resolution MUST produce identical results.

The tests MUST be independent of:

- OS;
- CPU;
- thread count;
- filesystem order;
- network order;
- hash seed;
- machine topology.

---

94. Hard-coding audit

Every versioning implementation MUST pass a hard-coding audit.

Forbidden universal constants include:

MAX_LANGUAGE_VERSION
MAX_VERSION_COMPONENT
MAX_DIALECTS
MAX_VERSION_REQUIREMENTS
MAX_DEPENDENCIES
MAX_COMPATIBILITY_RULES
MAX_IR_VERSIONS

unless they are strictly implementation safeguards and are never presented as language semantics.

If an implementation limit is necessary for safety/resource protection, it MUST:

1. be outside the language grammar;
2. be documented;
3. produce a structured resource/implementation diagnostic;
4. not change the language's theoretical semantics;
5. not be used as a portability rule.

---

95. Security

Version resolution MUST be safe and deterministic.

Version strings MUST NOT cause:

- arbitrary code execution;
- filesystem access;
- network access;
- shell execution;
- dynamic library loading;
- hardware access.

Version parsing MUST be data processing only.

Compiler implementation MUST use safe Rust.

No versioning feature may require "unsafe".

---

96. Denial-of-service considerations

Although the language is conceptually unbounded, implementations MAY protect themselves against pathological inputs.

Such protections MUST be:

implementation limits

rather than:

language semantics

If a safety limit is reached, the diagnostic MUST clearly state:

implementation/resource limit exceeded

rather than:

invalid Zamani syntax

when the syntax itself is valid.

---

97. Version metadata provenance

Version metadata SHOULD record provenance where required:

language version
compiler version
grammar revision
IR version
dialect versions
package versions
artifact version
build provenance

Provenance MUST NOT alter program semantics.

---

98. Reproducible version resolution

A reproducible build SHOULD be able to reconstruct:

effective language version
effective dialect versions
dependency versions
feature gates
IR compatibility
compiler compatibility

from recorded metadata.

Version resolution MUST NOT depend on an unavailable network service unless network resolution is explicitly part of the package/tooling contract.

---

99. Offline behavior

A compiler operating offline MUST be able to resolve all version information that has already been declared or vendored locally.

An unavailable network registry MUST NOT silently change the selected language version.

Failure to retrieve external metadata MUST be reported as an external dependency/registry error.

---

100. Time independence

Version semantics MUST NOT depend on the current date/time.

A source program compiled today and later with the same version inputs MUST resolve the same language semantics.

Date-based behavior belongs to release/tooling policy, not language-version interpretation.

---

101. Version aliases

Aliases MAY exist for user convenience, such as:

stable
beta
alpha

but aliases MUST resolve deterministically.

A moving alias MUST NOT be embedded into a supposedly immutable artifact without recording the resolved version.

For example:

stable

may resolve today to:

1.4.0

and later to:

1.5.0

An artifact MUST record the resolved version if long-term reproducibility is required.

---

102. Floating versus pinned versions

The implementation MUST distinguish:

floating compatibility requirement

from:

pinned version

For example:

>= 1.2.0 < 2.0.0

is a compatibility range.

1.2.7

is a specific version.

A lockfile may resolve a range to a concrete version without changing the source declaration.

---

103. Version selection must be explicit

When multiple versions satisfy a range, the resolver MUST use a documented deterministic policy.

Possible policies include:

highest compatible
lowest compatible
lockfile-selected
explicit selection

The repository MUST choose one policy per dependency context.

It MUST NOT depend on incidental registry ordering.

---

104. Compatibility is not preference

The following are distinct:

requires version >= 1.2
prefers version 1.4
supports version 1.5
targets version 2.0

A preference MUST NOT be treated as a requirement.

An unsupported required version MUST fail.

An unsupported preference MAY cause a different compatible selection.

---

105. Versioning and capabilities

Version support and capability support are separate.

For example:

language version = 1.5
capability = quantum.mid_circuit_measurement

A target may support the language version but lack the capability.

Conversely, a target may possess a capability that the selected language version cannot express.

These conditions MUST remain distinct.

---

106. Versioning and resources

Version compatibility does not guarantee resource sufficiency.

A program may be valid under:

Zamani 1.5

but require:

N qubits

while the target provides fewer.

The correct result is:

resource insufficiency

not:

language incompatibility

---

107. Versioning and correctness

A compiler MUST NOT downgrade or reinterpret a program merely because a target lacks resources.

For example, it MUST NOT silently transform:

requires exact quantum semantics

into:

approximate classical simulation

unless the source explicitly permits such an alternative.

---

108. Versioning and determinism of semantics

A language-version change MUST be evaluated for:

- evaluation order;
- concurrency;
- memory visibility;
- numerical semantics;
- floating-point behavior;
- distributed behavior;
- quantum measurement;
- probabilistic behavior;
- effects;
- resource semantics.

These are semantic changes even if syntax remains unchanged.

---

109. Versioning and probabilistic computation

A language version MUST define whether behavior is:

deterministic
probabilistic
nondeterministic
implementation-defined

where relevant.

Changing this classification for existing syntax is a semantic compatibility change.

---

110. Versioning and quantum measurement

Changing the semantics of:

measurement
reset
mid-circuit measurement
classical feed-forward

is a language semantic change.

Such changes MUST NOT be hidden behind a parser/grammar revision.

The quantum specification and compatibility tests MUST be updated together.

---

111. Versioning and HDL semantics

Changing semantics for:

clocking
reset
timing
signal assignment
sequential behavior
combinational behavior

is a semantic language change.

A grammar-only change is insufficient.

The semantic contract, AST contract, IR contract, and compatibility tests must be updated.

---

112. Versioning and distributed semantics

Changing semantics for:

consistency
ordering
replication
partitioning
failure
message delivery
transactions

is a compatibility-sensitive change.

A new distributed backend MUST NOT automatically change the language semantics.

---

113. Versioning and AI/data semantics

Changes affecting:

tensor shape semantics
differentiation
probability
training semantics
inference semantics
data ownership
stream semantics

must undergo semantic compatibility review.

A new AI framework or accelerator MUST NOT automatically create a new core-language version.

---

114. Versioning and security

Security semantics are version-sensitive.

Changes affecting:

- identity;
- authorization;
- capability boundaries;
- secret handling;
- cryptographic semantics;
- provenance;
- isolation;

must undergo compatibility review.

A compiler MUST NOT silently weaken a stable security guarantee for target compatibility.

---

115. Versioning and memory/ownership

Changes to:

- ownership;
- borrowing;
- aliasing;
- lifetime;
- memory visibility;
- resource ownership;

are semantic changes.

They require explicit compatibility classification.

---

116. Versioning and concurrency

Changes to:

- scheduling guarantees;
- synchronization semantics;
- ordering;
- memory visibility;
- actor semantics;
- channel semantics;

must undergo compatibility review.

Performance improvements that preserve semantics do not require a language-version change.

---

117. Versioning and source spans

Every version-sensitive AST construct MUST preserve source location information.

At minimum:

source identifier
start position
end position

where supported by the frontend.

Version diagnostics MUST be able to point to the exact source construct responsible for a compatibility problem.

---

118. Versioning and generated code

Generated code MUST carry version context.

A generator MUST NOT produce source that is ambiguous about its language version when the generated syntax is version-sensitive.

The generator's own version MUST remain separate from the generated program's language version.

---

119. Versioning and compiler plugins/tools

Compiler plugins and tools MUST declare compatibility with:

language version
compiler API
AST contract
IR contract
dialect contract

A tool compatible with one compiler implementation is not automatically compatible with all compiler versions.

---

120. Versioning and IDE/LSP tooling

IDE tooling SHOULD expose:

effective language version
supported language versions
deprecated constructs
experimental constructs
migration suggestions
version conflicts

Syntax highlighting MUST use the effective language version.

Completion MUST NOT suggest syntax unavailable under the selected version unless clearly marked as a future/experimental feature.

---

121. Versioning and formatting

Formatting is not language semantics.

A formatter may change representation while preserving:

language version
semantic meaning
source structure

Formatter versions MUST NOT silently migrate language versions.

---

122. Versioning and documentation

Every normative language document MUST identify:

document status
language version scope
ownership
dependencies

Documentation MUST distinguish:

normative
informative
experimental
historical
generated

---

123. Repository-wide version traceability

Every stable feature SHOULD be traceable through:

feature ID
    ↓
language specification
    ↓
version introduced
    ↓
grammar rule
    ↓
lexer tokens
    ↓
AST node
    ↓
semantic rule
    ↓
IR mapping
    ↓
compiler consumer
    ↓
runtime consumer
    ↓
tests

This ensures that a feature can be completed independently without requiring later architectural rework.

---

124. Independent-file completion contract

For any versioning-related file, completion MUST establish:

File
Purpose
Status
Owns
Does Not Own
Inputs
Outputs
Dependencies
Upstream Contracts
Downstream Consumers
Public Grammar Contract
AST Contract
Semantic Contract
IR Integration
Compiler Integration
Runtime Integration
Tooling Integration
Cross-Domain Integration
Positive Tests
Negative Tests
Boundary Tests
Scalability Tests
Compatibility Tests
Determinism Tests
Migration Tests
Hard-Coding Audit
Diagnostics
Security
Performance
Completion Criteria

No versioning file should be considered complete if these integration relationships are undefined.

---

125. This file's own contract

File

grammar/spec/versioning.md

Purpose

Define the cross-layer implementation and conformance contract for Zamani versioning.

Owns

- version-layer relationships;
- version propagation;
- implementation conformance;
- deterministic version resolution;
- AST/semantic/IR integration;
- artifact metadata requirements;
- repository-wide version traceability;
- versioning hard-coding rules.

Does not own

- concrete source grammar as the primary authority;
- release notes;
- migration procedures;
- deprecation inventory;
- runtime behavior;
- hardware behavior;
- target selection;
- QEC;
- ZQN;
- routing;
- scheduling;
- compiler implementation details.

Inputs

- "grammar/specification/language-version.md"
- "grammar/spec/compatibility.md"
- "grammar/compatibility/versions.md"
- "grammar/core/versioning.g4"
- lexical contracts
- canonical grammar
- AST contracts
- semantic contracts
- IR contracts

Outputs

- versioning conformance requirements;
- integration requirements;
- compatibility validation requirements.

Upstream contracts

grammar/specification/language-version.md
grammar/spec/lexical.md
grammar/spec/syntax.md
grammar/spec/semantics.md

Downstream consumers

grammar/Zamani.g4
grammar/core/versioning.g4
grammar/dialects/versioning.g4
src/lexer.rs
src/parser.rs
src/frontend/ast/
semantic analysis
IR generation
compiler
runtime
tooling
tests

---

126. Integration contract: specification layer

The specification layer MUST establish:

what a version means

The implementation layer MUST establish:

how that version contract is enforced

This file belongs to the second category.

---

127. Integration contract: grammar layer

The grammar layer MUST establish:

what version syntax can be parsed

It MUST NOT establish:

whether the requested version is supported

or:

whether a target can execute it

---

128. Integration contract: lexer layer

The lexer MUST provide the tokens required by canonical version syntax.

Token names MUST be centrally owned.

The versioning specification MUST NOT create duplicate token concepts.

---

129. Integration contract: parser layer

The parser MUST construct version AST nodes without performing semantic resolution.

Parser actions MUST NOT:

- access files;
- access networks;
- access hardware;
- select versions dynamically;
- select compilers;
- select runtimes.

---

130. Integration contract: AST layer

The AST MUST preserve enough information to perform:

version comparison
compatibility
diagnostics
migration
tooling

without reparsing source text.

---

131. Integration contract: semantic layer

Semantic analysis MUST resolve:

effective language version
feature availability
compatibility
deprecation
migration requirements
dialect compatibility
package compatibility
IR compatibility

---

132. Integration contract: IR layer

IR generation MUST receive an already-resolved semantic contract.

IR MUST NOT independently reinterpret language-version syntax.

IR compatibility must remain separately versioned.

---

133. Integration contract: compiler layer

The compiler MUST:

1. resolve language version;
2. validate support;
3. validate dependencies;
4. validate dialects;
5. validate feature availability;
6. perform semantic analysis;
7. lower to canonical IR;
8. preserve semantic identity.

---

134. Integration contract: runtime layer

The runtime MUST NOT reinterpret source-language versions.

Runtime compatibility checks may validate:

artifact format
IR version
runtime contract
capabilities
resources

but source semantics remain owned by the language/compiler contracts.

---

135. Integration contract: hardware layer

Hardware versions MUST remain separate from language versions.

A target may report:

hardware generation
capabilities
resources
topology
calibration

without changing the source language.

---

136. Integration contract: quantum layer

Quantum language versioning MUST integrate with:

src/quantum/
quantum::ir
QEC
ZQN
HAL
routing
scheduling
optimization

without taking ownership of their implementations.

---

137. Integration contract: HDL layer

HDL versioning MUST integrate with:

grammar/hdl/
hardware intent
synthesis
simulation
verification
hardware IR

without turning target-specific hardware limitations into language-version limits.

---

138. Integration contract: interoperability

Every external language/format adapter MUST declare:

source format
source format version
Zamani language compatibility
adapter version
semantic mapping
lossiness

Lossy translation MUST be explicit.

---

139. Lossless compatibility

A compatibility adapter is lossless only if all source semantics required by the target contract survive translation.

If information cannot be represented, the adapter MUST:

- report the limitation;
- identify the lost construct;
- avoid silent semantic loss.

---

140. Version compatibility and semantic loss

The following is prohibited:

old source
 ↓
new parser
 ↓
AST loses feature
 ↓
IR ignores feature
 ↓
program still compiles

If a version change makes a construct unsupported, compilation MUST fail or perform an explicitly requested migration.

---

141. Compatibility matrix

The repository SHOULD maintain a compatibility matrix covering:

Layer| Version| Supported range| Authority
Language| language version| language policy| "specification/language-version.md"
Lexer| lexer contract| implementation policy| "spec/lexical.md"
Grammar| grammar contract| grammar policy| "Zamani.g4"
AST| AST contract| frontend policy| "src/frontend/ast/"
Semantics| semantic contract| semantic policy| semantic specification
Quantum IR| "quantum::ir"| IR policy| quantum subsystem
Compiler| compiler version| implementation| compiler
Runtime| runtime version| runtime policy| runtime
Dialect| dialect version| dialect policy| "dialects/"
Artifact| artifact version| serialization policy| artifact contract
Target| target contract| target policy| hardware/HAL

The matrix MUST NOT imply that these versions are numerically interchangeable.

---

142. Release gate

A language release MUST NOT be considered production-ready until:

specification
    ↓
grammar
    ↓
lexer
    ↓
parser
    ↓
AST
    ↓
semantic analysis
    ↓
IR
    ↓
compiler
    ↓
runtime
    ↓
tests

agree for the declared version.

Required gates:

- specification conformance;
- grammar conformance;
- lexer conformance;
- parser conformance;
- AST coverage;
- semantic coverage;
- IR coverage;
- compatibility coverage;
- migration coverage;
- deterministic diagnostics;
- scalability validation;
- hard-coding audit.

---

143. Version release checklist

Before release, verify:

Specification

- [ ] version defined;
- [ ] compatibility class defined;
- [ ] semantic changes documented;
- [ ] feature statuses updated.

Grammar

- [ ] canonical grammar updated;
- [ ] no competing grammar introduced;
- [ ] no ambiguity introduced;
- [ ] no fixed resource limit introduced.

Lexer

- [ ] token changes reviewed;
- [ ] keyword collisions checked;
- [ ] literal compatibility checked.

Parser

- [ ] AST mapping complete;
- [ ] no semantic actions introduced.

AST

- [ ] all syntax represented;
- [ ] source spans preserved;
- [ ] no target-specific leakage.

Semantics

- [ ] version resolution deterministic;
- [ ] compatibility checked;
- [ ] conflicts diagnosed.

IR

- [ ] semantic information preserved;
- [ ] canonical IR boundary preserved;
- [ ] quantum::ir preserved where applicable.

Compiler

- [ ] supported versions declared;
- [ ] unsupported versions diagnosed;
- [ ] migrations tested.

Runtime

- [ ] artifact compatibility validated;
- [ ] runtime version separated from language version.

Testing

- [ ] positive;
- [ ] negative;
- [ ] boundary;
- [ ] scalability;
- [ ] determinism;
- [ ] migration;
- [ ] compatibility.

Safety

- [ ] no "unsafe";
- [ ] no arbitrary filesystem access;
- [ ] no arbitrary network access;
- [ ] no target-dependent parser behavior.

---

144. Required coordinated repository updates

This file is independently complete, but the following existing files MUST remain consistent with it.

"grammar/specification/language-version.md"

Clarify that it owns:

language-level version semantics

and does not own implementation conformance details.

It should reference this file for cross-layer implementation requirements.

"grammar/spec/compatibility.md"

Reference this file for:

version propagation
version resolution
implementation conformance

It should remain the broader compatibility contract.

"grammar/compatibility/versions.md"

Keep release/version policy here.

It MUST reference:

grammar/specification/language-version.md
grammar/spec/versioning.md

rather than redefining version semantics.

"grammar/compatibility/migrations.md"

Own migration procedures.

It MUST consume the version classifications defined by the specification.

"grammar/compatibility/deprecated.md"

Own deprecation inventory and lifecycle.

"grammar/core/versioning.g4"

Keep syntax ownership here.

Correct/validate:

- canonical "MAJOR.MINOR.PATCH";
- four-component compatibility handling;
- channel semantics;
- string-version validation;
- token consistency;
- ambiguity;
- parser/AST mapping.

"grammar/dialects/versioning.g4"

Keep this as an adapter.

It MUST NOT create an independent version grammar.

"grammar/Zamani.g4"

Use the canonical versioning composition.

"grammar/grammar.md"

Report actual implementation status.

"grammar/Zamani-Grammar.md"

Classify broader versioning ideas as proposed/experimental/historical unless implemented.

---

145. No unnecessary file renaming

The following existing files SHOULD NOT be renamed merely to implement this specification:

grammar/Zamani.g4
grammar/Zamani-Grammar.md
grammar/grammar.md
grammar/specification/language-version.md
grammar/spec/compatibility.md
grammar/compatibility/versions.md
grammar/compatibility/migrations.md
grammar/compatibility/deprecated.md
grammar/core/versioning.g4
grammar/dialects/versioning.g4

"grammar/spec/versioning.md" is a new complementary specification file, not a replacement for those files.

---

146. No duplicate authority

The following architecture is prohibited:

specification/language-version.md
        +
spec/versioning.md
        +
compatibility/versions.md
        +
Zamani-Grammar.md
        +
Zamani.g4

all independently defining different meanings for versioning.

Instead:

language-version.md
        │
        ├── semantic authority
        │
        ▼
spec/versioning.md
        │
        ├── implementation/conformance
        │
        ▼
spec/compatibility.md
        │
        ├── compatibility model
        │
        ▼
compatibility/
        │
        ├── release policy
        ├── migration
        └── deprecation

---

147. Completion criteria

"grammar/spec/versioning.md" is complete when:

- [ ] language-version ownership is defined;
- [ ] implementation-version ownership is defined;
- [ ] grammar-version ownership is defined;
- [ ] dialect-version ownership is defined;
- [ ] package/API/ABI versions are separated;
- [ ] IR versions are separated;
- [ ] target versions are separated;
- [ ] Rust version is separated;
- [ ] version syntax is separated from version semantics;
- [ ] canonical language version is defined;
- [ ] pre-release/build semantics are defined;
- [ ] version ranges are defined;
- [ ] compatibility classes are defined;
- [ ] experimental lifecycle is defined;
- [ ] deprecation lifecycle is integrated;
- [ ] migration lifecycle is integrated;
- [ ] compiler support policy is defined;
- [ ] forward compatibility is defined;
- [ ] backward compatibility is defined;
- [ ] AST integration is defined;
- [ ] semantic integration is defined;
- [ ] IR integration is defined;
- [ ] quantum::ir integration is defined;
- [ ] HDL integration is defined;
- [ ] hardware independence is defined;
- [ ] capability/resource separation is defined;
- [ ] POCO-REAF is addressed;
- [ ] scalability is addressed;
- [ ] determinism is addressed;
- [ ] reproducibility is addressed;
- [ ] diagnostics are addressed;
- [ ] security is addressed;
- [ ] safe-Rust requirement is addressed;
- [ ] hard-coding audit is defined;
- [ ] repository integration is defined;
- [ ] independent-file completion contract is defined.

---

148. Final invariant

The complete Zamani versioning system MUST preserve this invariant:

                    VERSION
                       │
          ┌────────────┼────────────┐
          │            │            │
       LANGUAGE      DIALECT       IR
          │            │            │
          ▼            ▼            ▼
       SOURCE        EXTENSION    REPRESENTATION
          │            │            │
          └────────────┼────────────┘
                       │
                       ▼
                 SEMANTIC MODEL
                       │
                       ▼
              TARGET-INDEPENDENT IR
                       │
        ┌──────────────┼──────────────┐
        │              │              │
       CPU            GPU            QPU
        │              │              │
       FPGA          ASIC        future target

The versioning system describes contracts, not machines.

Therefore:

language evolution
        ≠
compiler evolution
        ≠
runtime evolution
        ≠
IR evolution
        ≠
hardware evolution

and:

program semantics
        ≠
target resources
        ≠
target capabilities
        ≠
implementation choices

The ultimate POCO-REAF invariant is:

«A Zamani program's semantic meaning is determined by its declared language contract, not by the size, topology, vendor, generation, or temporary availability of the machine on which it is eventually realized.»

That is the versioning foundation required for Zamani to scale from a tiny computation to arbitrarily large classical, quantum, hybrid, HDL, distributed, AI, data, networking, accelerator, embedded, and future computational systems, subject to available resources and capabilities, without turning today's implementation constraints into tomorrow's language limits.